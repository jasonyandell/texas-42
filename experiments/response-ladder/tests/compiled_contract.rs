use walt::kernel::{Hidden, Kernel};
use walt::rules::rules::legal_plays;
use walt::rules::{Context, ContextSet, Decl, Domino, DominoSet, Seat, Team};
use walt::scheme::{Budget, DecisionProvenance, Frame, PolicyInput, Registry};

use walt_response_ladder::compiled::{
    clause_actions, Actor, Choice, CLAUSE_COUNT, CLAUSE_NAMES, SCHEMA,
};
use walt_response_ladder::mechanics::{domino, PublicState};

fn mask(tiles: impl IntoIterator<Item = u8>) -> u32 {
    tiles.into_iter().map(|tile| 1u32 << tile).sum()
}

fn with_reference_input<R>(
    decl: Decl,
    viewer: u8,
    original: &[u32; 4],
    public: &PublicState,
    history: &[(Seat, Domino)],
    f: impl for<'a> FnOnce(&'a Frame, &PolicyInput<'a>) -> R,
) -> R {
    let viewer_seat = Seat::from_index(viewer as usize).unwrap();
    let viewer_hand = DominoSet::from_bits(original[viewer as usize] & !public.played).unwrap();
    let mut hidden = Vec::new();
    let mut pool = DominoSet::EMPTY;
    for seat in Seat::ALL {
        if seat == viewer_seat {
            continue;
        }
        let remaining = DominoSet::from_bits(original[seat.index()] & !public.played).unwrap();
        let voids: ContextSet = Context::ALL
            .into_iter()
            .filter(|q| {
                let incidence = decl.effective_incidence(*q).bits();
                incidence != 0 && public.voids[seat.index()] & incidence == incidence
            })
            .collect();
        hidden.push(Hidden {
            seat,
            capacity: remaining.len(),
            voids,
        });
        pool = pool.union(remaining);
    }
    let kernel = Kernel::new(
        decl,
        viewer_seat,
        viewer_hand,
        pool,
        hidden.try_into().unwrap(),
    )
    .unwrap();
    let frame = Frame::new(
        kernel,
        Seat::from_index(public.leader as usize).unwrap(),
        public.plays.iter().copied().map(domino).collect(),
        DominoSet::from_bits(public.played).unwrap(),
    )
    .unwrap();
    let input = PolicyInput::new(
        &frame,
        history,
        [public.banked_t0 as u32, public.banked_t1 as u32],
        30,
        Team::T1,
    )
    .unwrap();
    f(&frame, &input)
}

fn choice_for_scheme(
    decl: Decl,
    viewer: u8,
    original: &[u32; 4],
    public: &PublicState,
    history: &[(Seat, Domino)],
    clause: usize,
) -> Choice {
    let actor = Actor::new(SCHEMA, vec![clause]).unwrap();
    let program = actor.policy_program().unwrap();
    let compiled = program.compile(&Registry::standard()).unwrap();
    with_reference_input(decl, viewer, original, public, history, |_, input| {
        let mut budget = Budget::new(100_000);
        let mut controller = compiled.initialize(input, &mut budget).unwrap();
        let trace = compiled
            .choose_traced(&mut controller, input, &mut budget)
            .unwrap();
        Choice {
            action: trace.action.index() as u8,
            provenance: trace.provenance,
        }
    })
}

#[test]
fn scalar_fourteen_clause_outputs_and_provenance_match_scheme() {
    let original = [mask(0..7), mask(7..14), mask(14..21), mask(21..28)];

    for decl in Decl::ALL {
        for leader in 0..4u8 {
            let mut remaining = original;
            let mut public = PublicState::opening(leader);
            let mut history = Vec::new();
            while history.len() < Domino::COUNT {
                let viewer = public.actor();
                let hand = remaining[viewer as usize] & !public.played;
                let actions = clause_actions(decl, viewer, hand, &public).unwrap();
                for clause in 0..CLAUSE_COUNT {
                    let scalar = Actor::new(SCHEMA, vec![clause])
                        .unwrap()
                        .choose(decl, viewer, hand, &public)
                        .unwrap();
                    let scheme =
                        choice_for_scheme(decl, viewer, &original, &public, &history, clause);
                    assert_eq!(scalar.action, scheme.action, "{decl:?} clause {clause}");
                    assert_eq!(
                        scalar.provenance, scheme.provenance,
                        "{decl:?} clause {clause}"
                    );
                    assert_eq!(
                        actions[clause],
                        Some(scheme.action).filter(|_| {
                            !matches!(scheme.provenance, DecisionProvenance::Fallback)
                        })
                    );
                }

                let legal = legal_plays(
                    decl,
                    DominoSet::from_bits(hand).unwrap(),
                    public
                        .plays
                        .first()
                        .map(|tile| decl.led_context(domino(*tile))),
                );
                let tile = legal.iter().next().unwrap().index() as u8;
                remaining[viewer as usize] &= !(1u32 << tile);
                history.push((Seat::from_index(viewer as usize).unwrap(), domino(tile)));
                public = public.after(decl, tile);
            }
        }
    }
}

#[test]
fn actor_rejects_invalid_serialized_programs() {
    assert!(Actor::new("wrong-schema", vec![]).is_err());
    assert!(Actor::new(SCHEMA, vec![CLAUSE_COUNT]).is_err());
    assert!(Actor::new(SCHEMA, vec![2, 2]).is_err());
    assert!(Actor::new(SCHEMA, vec![0, 1, 2, 3]).is_err());
}

#[test]
fn exported_program_has_the_frozen_rule_names_and_order() {
    let actor = Actor::new(SCHEMA, vec![12, 1, 8]).unwrap();
    let program = actor.policy_program().unwrap();
    let names: Vec<_> = program.rules.iter().map(|rule| rule.name.clone()).collect();
    assert_eq!(
        names,
        vec![
            format!("r0-{}", CLAUSE_NAMES[12]),
            format!("r1-{}", CLAUSE_NAMES[1]),
            format!("r2-{}", CLAUSE_NAMES[8]),
        ]
    );
    assert_eq!(program.fallback, walt::scheme::Fallback::LowestLegal);
}
