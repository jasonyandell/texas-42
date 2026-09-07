use std::collections::BTreeSet;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::One;
use walt::kernel::{Hidden, Kernel};
use walt::rules::{Context, ContextSet, Decl, Domino, DominoSet, Seat};
use walt::scheme::*;

fn tile(s: &str) -> Domino {
    s.parse().unwrap()
}
fn tiles(xs: &[&str]) -> DominoSet {
    xs.iter().map(|x| tile(x)).collect()
}
fn initial() -> Frame {
    let kernel = Kernel::new(
        "P6".parse::<Decl>().unwrap(),
        Seat::S0,
        tiles(&["6-6", "0-0"]),
        tiles(&["6-5", "5-5", "3-2", "6-4", "2-1", "1-0"]),
        [Seat::S1, Seat::S2, Seat::S3].map(|seat| Hidden {
            seat,
            capacity: 2,
            voids: ContextSet::EMPTY,
        }),
    )
    .unwrap();
    Frame::new(kernel, Seat::S0, vec![], DominoSet::EMPTY).unwrap()
}
fn budget() -> Budget {
    Budget::new(10_000)
}

#[test]
fn typed_play_rejects_wrong_actor_class_and_illegal_world() {
    let frame = initial();
    let world = frame.kernel().worlds().next().unwrap();
    let wrong_actor = ObservedPlay {
        actor: Seat::S1,
        tile: tile("6-6"),
        class: PlayClass::Lead,
    };
    assert!(step_frame(&frame, wrong_actor, &mut budget()).is_err());
    let wrong_class = ObservedPlay {
        actor: Seat::S0,
        tile: tile("6-6"),
        class: PlayClass::Follow(Context::Called),
    };
    assert!(step_frame(&frame, wrong_class, &mut budget()).is_err());
    let absent = ObservedPlay {
        actor: Seat::S0,
        tile: tile("6-5"),
        class: PlayClass::Lead,
    };
    assert!(step_world(&frame, &world, absent).is_err());
}

#[test]
fn frame_step_rejects_viewer_slough_while_viewer_can_follow() {
    let kernel = Kernel::new(
        "P6".parse::<Decl>().unwrap(),
        Seat::S0,
        tiles(&["5-4", "0-0"]),
        tiles(&["6-6", "4-4", "3-2", "6-4", "2-1"]),
        [
            Hidden {
                seat: Seat::S1,
                capacity: 2,
                voids: ContextSet::EMPTY,
            },
            Hidden {
                seat: Seat::S2,
                capacity: 2,
                voids: ContextSet::EMPTY,
            },
            Hidden {
                seat: Seat::S3,
                capacity: 1,
                voids: ContextSet::EMPTY,
            },
        ],
    )
    .unwrap();
    let frame = Frame::new(kernel, Seat::S3, vec![tile("5-5")], tiles(&["5-5"])).unwrap();
    let q5 = Context::Natural(walt::rules::Pip::new(5).unwrap());
    let illegal = ObservedPlay {
        actor: Seat::S0,
        tile: tile("0-0"),
        class: PlayClass::Slough(q5),
    };
    assert!(step_frame(&frame, illegal, &mut budget()).is_err());
}

#[test]
fn frame_step_rejects_hidden_tile_forbidden_by_an_existing_void() {
    let mut hidden = [Seat::S1, Seat::S2, Seat::S3].map(|seat| Hidden {
        seat,
        capacity: 2,
        voids: ContextSet::EMPTY,
    });
    hidden[0].voids.insert(Context::Called);
    let kernel = Kernel::new(
        "P6".parse::<Decl>().unwrap(),
        Seat::S0,
        tiles(&["0-0"]),
        tiles(&["6-5", "5-5", "3-2", "6-4", "2-1", "1-0"]),
        hidden,
    )
    .unwrap();
    let frame = Frame::new(kernel, Seat::S0, vec![tile("6-6")], tiles(&["6-6"])).unwrap();
    let impossible = ObservedPlay {
        actor: Seat::S1,
        tile: tile("6-5"),
        class: PlayClass::Follow(Context::Called),
    };
    assert!(step_frame(&frame, impossible, &mut budget()).is_err());
}

#[test]
fn hidden_slough_conditions_support_and_pushes_worlds_forward() {
    let lead = ObservedPlay {
        actor: Seat::S0,
        tile: tile("6-6"),
        class: PlayClass::Lead,
    };
    let led = step_frame(&initial(), lead, &mut budget()).unwrap();
    let prior = Belief::uniform(led, 90).unwrap();
    let slough = ObservedPlay {
        actor: Seat::S1,
        tile: tile("3-2"),
        class: PlayClass::Slough(Context::Called),
    };
    let stepped = step_belief(
        &prior,
        slough,
        "slough-posterior",
        90,
        &mut budget(),
        unit_likelihood,
    )
    .unwrap();
    assert!(stepped.evidence_probability > BigRational::from_integer(BigInt::from(0)));
    assert!(stepped.evidence_probability < BigRational::one());
    assert_eq!(stepped.belief.frame().prefix(), &[tile("6-6"), tile("3-2")]);
    assert!(!stepped.belief.frame().kernel().pool().contains(tile("3-2")));
    for (world, _) in stepped.belief.worlds() {
        assert!(!world.hand(Seat::S1).contains(tile("3-2")));
        assert!(world
            .hand(Seat::S1)
            .intersection(
                stepped
                    .belief
                    .frame()
                    .kernel()
                    .decl()
                    .effective_incidence(Context::Called)
            )
            .is_empty());
    }
}

#[test]
fn explicit_likelihood_precedes_pushforward_and_normalization() {
    let lead = ObservedPlay {
        actor: Seat::S0,
        tile: tile("6-6"),
        class: PlayClass::Lead,
    };
    let led = step_frame(&initial(), lead, &mut budget()).unwrap();
    let prior = Belief::uniform(led, 90).unwrap();
    let follow = ObservedPlay {
        actor: Seat::S1,
        tile: tile("6-5"),
        class: PlayClass::Follow(Context::Called),
    };
    let half = BigRational::new(BigInt::from(1), BigInt::from(2));
    let unit = step_belief(&prior, follow, "unit", 90, &mut budget(), unit_likelihood).unwrap();
    let halved = step_belief(&prior, follow, "half", 90, &mut budget(), |_| {
        Ok(half.clone())
    })
    .unwrap();
    assert_eq!(
        halved.evidence_probability,
        unit.evidence_probability * &half
    );
    assert_eq!(halved.belief.len(), unit.belief.len());
    assert_eq!(
        halved.belief.total_mass(),
        &(unit.belief.total_mass() * &half)
    );
}

#[test]
fn viewer_intervention_preserves_hidden_deal_odds() {
    let frame = initial();
    let entries = frame
        .kernel()
        .worlds()
        .take(3)
        .enumerate()
        .map(|(i, world)| {
            (
                world,
                BigRational::from_integer(BigInt::from((i + 1) as u32)),
            )
        })
        .collect::<Vec<_>>();
    let prior = Belief::from_weights(frame, "nonuniform", entries, 3).unwrap();
    let observation = ObservedPlay {
        actor: Seat::S0,
        tile: tile("6-6"),
        class: PlayClass::Lead,
    };
    let stepped = step_belief(
        &prior,
        observation,
        "viewer-intervention",
        3,
        &mut budget(),
        unit_likelihood,
    )
    .unwrap();
    assert_eq!(stepped.evidence_probability, BigRational::one());
    assert_eq!(stepped.belief.len(), prior.len());
    for ((_, before), (_, after)) in prior.worlds().zip(stepped.belief.worlds()) {
        assert_eq!(before, after);
    }
}

#[test]
fn rigid_identity_separates_persistence_extinction_and_birth() {
    let a = Answer(vec![Value::Domino(tile("6-6"))]);
    let b = Answer(vec![Value::Domino(tile("5-5"))]);
    let c = Answer(vec![Value::Domino(tile("3-2"))]);
    let prior = BTreeSet::from([a.clone(), b.clone()]);
    let fresh = BTreeSet::from([b.clone(), c.clone()]);
    let dynamics = compare_answers(&prior, &fresh);
    assert_eq!(dynamics.transported, prior);
    assert_eq!(dynamics.persistent, BTreeSet::from([b]));
    assert_eq!(dynamics.extinct, BTreeSet::from([a]));
    assert_eq!(dynamics.born, BTreeSet::from([c]));
}

#[test]
fn refusal_returns_no_partial_belief() {
    let prior = Belief::uniform(initial(), 90).unwrap();
    let observation = ObservedPlay {
        actor: Seat::S0,
        tile: tile("6-6"),
        class: PlayClass::Lead,
    };
    let mut tiny = Budget::new((prior.len() as u64).saturating_sub(1));
    assert!(step_belief(
        &prior,
        observation,
        "never-built",
        90,
        &mut tiny,
        unit_likelihood
    )
    .is_err());
    assert_eq!(prior.frame().prefix(), &[]);
    assert_eq!(prior.id(), "uniform-full-support-v1");
}

#[test]
fn hindsight_anchor_is_a_preimage_not_revelation() {
    let frame = initial();
    let observation = ObservedPlay {
        actor: Seat::S0,
        tile: tile("6-6"),
        class: PlayClass::Lead,
    };
    let worlds: Vec<_> = frame.kernel().worlds().take(2).collect();
    let expected_world = worlds[0];
    let answer = Answer(vec![Value::Chair(Seat::S1)]);
    let target = step_world(&frame, &worlds[0], observation).unwrap();
    let later = vec![(target, answer.clone())];
    let anchored = anchor_back(
        &frame,
        observation,
        worlds.into_iter().map(|w| (w, answer.clone())),
        &later,
        &mut budget(),
    )
    .unwrap();
    assert_eq!(anchored, vec![(expected_world, answer)]);
}

#[test]
fn hindsight_anchor_validates_empty_inputs_and_deduplicates_relation_rows() {
    let frame = initial();
    let malformed = ObservedPlay {
        actor: Seat::S1,
        tile: tile("6-6"),
        class: PlayClass::Lead,
    };
    assert!(anchor_back(&frame, malformed, Vec::new(), &[], &mut budget()).is_err());

    let observation = ObservedPlay {
        actor: Seat::S0,
        tile: tile("6-6"),
        class: PlayClass::Lead,
    };
    let world = frame.kernel().worlds().next().unwrap();
    let answer = Answer(vec![Value::Chair(Seat::S1)]);
    let successor = step_world(&frame, &world, observation).unwrap();
    let anchored = anchor_back(
        &frame,
        observation,
        vec![(world, answer.clone()), (world, answer.clone())],
        &[(successor, answer.clone()), (successor, answer.clone())],
        &mut budget(),
    )
    .unwrap();
    assert_eq!(anchored, vec![(world, answer)]);
}
