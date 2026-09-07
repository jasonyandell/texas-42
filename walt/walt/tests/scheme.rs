//! Finite-domain semantic checks for the expressive Scheme/Fix runtime.
//! Expected answers are constructed directly from hands/rules, independently
//! of the query compiler, joins, projection, and probability aggregation.
use std::collections::BTreeSet;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};
use walt::kernel::{Hidden, Kernel};
use walt::rules::{Context, ContextSet, Decl, Domino, DominoSet, Seat};
use walt::scheme::*;

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(BigInt::from(n), BigInt::from(d))
}
fn tile(s: &str) -> Domino {
    s.parse().unwrap()
}
fn tiles(ss: &[&str]) -> DominoSet {
    ss.iter().map(|s| tile(s)).collect()
}
fn frame(decl: Decl) -> Frame {
    let kernel = Kernel::new(
        decl,
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
fn belief() -> Belief {
    Belief::uniform(frame("P6".parse().unwrap()), 90).unwrap()
}
fn compile(s: &str) -> CompiledFix {
    s.parse::<Fix>()
        .unwrap()
        .compile(&Registry::standard())
        .unwrap()
}
fn budget() -> Budget {
    Budget::new(20_000_000)
}
fn one_answer(v: Value) -> Answer {
    Answer(vec![v])
}
const COUNTS: &str = include_str!("../../scheme/examples/partner-count.scheme");
const HAS_COUNT: &str = include_str!("../../scheme/examples/partner-has-count.scheme");

#[test]
fn examples_parse_compile_and_roundtrip() {
    for s in [
        COUNTS,
        HAS_COUNT,
        include_str!("../../scheme/examples/partner-master.scheme"),
        include_str!("../../scheme/examples/current-forced-play.scheme"),
    ] {
        let fix = s.parse::<Fix>().unwrap();
        assert_eq!(fix.to_string().parse::<Fix>().unwrap(), fix);
        assert_eq!(
            fix.compile(&Registry::standard()).unwrap().identity(),
            fix.clone()
                .compile(&Registry::standard())
                .unwrap()
                .identity()
        );
    }
}

#[test]
fn syntax_and_type_errors_are_refused_before_evaluation() {
    for s in [
        "",
        "(fix",
        "(fix (roles) (out)))",
        "(fix (roles) (out)) extra",
        "(fix (roles) (out) (case (not)))",
        "(fix (roles (number a)) (out))",
    ] {
        assert!(s.parse::<Fix>().is_err(), "{s}");
    }
    for s in [
        "(fix (roles (domino a) (chair a)) (out))",
        "(fix (roles (domino S0)) (out))",
        "(fix (roles (domino a)) (out b))",
        "(fix (roles (domino a)) (out a a))",
        "(fix (roles (chair a)) (out) (case (live a)))",
        "(fix (roles (domino a)) (out) (case (holds S0 a q*)))",
        "(fix (roles (domino a)) (out) (case (mystery a)))",
        "(fix (roles (domino a) (chair b)) (out) (case (same a b)))",
        "(fix (roles (domino a)) (out) (case (same a missing)))",
        "(fix (roles (domino a)) (out) (case (count a a)))",
    ] {
        assert!(
            s.parse::<Fix>()
                .unwrap()
                .compile(&Registry::standard())
                .is_err(),
            "{s}"
        );
    }
}

#[test]
fn empty_fix_false_empty_case_true_and_hidden_witnesses_are_existential() {
    let b = belief();
    let world = b.worlds().next().unwrap().0;
    assert!(compile("(fix (roles) (out))")
        .evaluate(b.frame(), world, &mut budget())
        .unwrap()
        .is_empty());
    assert_eq!(
        compile("(fix (roles) (out) (case))")
            .evaluate(b.frame(), world, &mut budget())
            .unwrap(),
        BTreeSet::from([Answer(vec![])])
    );
    // 28 witnesses still produce one Boolean answer.
    let many = compile("(fix (roles (domino hidden)) (out) (case))");
    assert_eq!(
        many.evaluate(b.frame(), world, &mut budget()).unwrap(),
        BTreeSet::from([Answer(vec![])])
    );
}

#[test]
fn equality_patterns_are_complete_and_quotient_before_injective_binding() {
    let b = belief();
    let w = b.worlds().next().unwrap().0;
    let distinct = compile("(fix (roles (domino a) (domino b)) (out a b) (case))");
    let alias = compile("(fix (roles (domino a) (domino b)) (out a b) (case (same a b)))");
    let all = compile("(fix (roles (domino a) (domino b)) (out a b) (case) (case (same a b)))");
    assert_eq!(
        distinct
            .evaluate(b.frame(), w, &mut budget())
            .unwrap()
            .len(),
        28 * 27
    );
    assert_eq!(
        alias.evaluate(b.frame(), w, &mut budget()).unwrap().len(),
        28
    );
    let expected: Answers = Domino::ALL
        .into_iter()
        .flat_map(|a| Domino::ALL.map(|b| Answer(vec![Value::Domino(a), Value::Domino(b)])))
        .collect();
    assert_eq!(all.evaluate(b.frame(), w, &mut budget()).unwrap(), expected);
    let transitive = compile(
        "(fix (roles (chair a) (chair b) (chair c)) (out a b c) (case (same a b) (same c b)))",
    );
    let expected = Seat::ALL
        .map(|s| Answer(vec![Value::Chair(s); 3]))
        .into_iter()
        .collect();
    assert_eq!(
        transitive.evaluate(b.frame(), w, &mut budget()).unwrap(),
        expected
    );
}

#[test]
fn joins_match_direct_relations_on_every_world_under_all_nine_declarations() {
    let query = compile(
        "(fix (roles (chair me) (chair partner) (domino d) (context suit)) (out d suit)
        (case (viewer me) (partner me partner) (holds partner d) (in d suit)))",
    );
    for decl in Decl::ALL {
        let b = Belief::uniform(frame(decl), 90).unwrap();
        for (world, _) in b.worlds() {
            let expected: Answers = world
                .hand(Seat::S2)
                .iter()
                .flat_map(|d| {
                    Context::ALL
                        .into_iter()
                        .filter(move |c| decl.follows(d, *c))
                        .map(move |c| Answer(vec![Value::Domino(d), Value::Context(c)]))
                })
                .collect();
            assert_eq!(
                query.evaluate(b.frame(), world, &mut budget()).unwrap(),
                expected,
                "{decl} {world:?}"
            );
        }
    }
}

#[test]
fn overlapping_branches_and_internal_witnesses_do_not_multiply_mass() {
    let b = belief();
    let base = compile(COUNTS);
    let mut duplicate = base.source().clone();
    duplicate.cases.extend(duplicate.cases.clone());
    let duplicate = duplicate.compile(&Registry::standard()).unwrap();
    assert!(base
        .compare(&duplicate, &b, Comparison::Answers, &mut budget())
        .unwrap()
        .is_none());
    let summary = base.summarize(&b, &mut budget()).unwrap();
    assert_eq!(summary.event_probability(), q(4, 5)); // 1 - C(3,2)/C(6,2)
    for d in ["3-2", "6-4", "5-5"] {
        assert_eq!(
            summary.presence_probability(&one_answer(Value::Domino(tile(d)))),
            q(1, 3)
        );
    }
    let boolean = compile(HAS_COUNT).summarize(&b, &mut budget()).unwrap();
    assert_eq!(boolean.event_probability(), q(4, 5));
    assert_eq!(boolean.answer_presence_mass.len(), 1);
    assert_eq!(
        duplicate
            .summarize(&b, &mut budget())
            .unwrap()
            .answer_set_mass,
        summary.answer_set_mass
    );
}

#[test]
fn answer_multiplicity_is_not_a_probability_distribution() {
    let full = belief();
    let two = full
        .worlds()
        .find(|(w, _)| w.hand(Seat::S2) == tiles(&["3-2", "6-4"]))
        .unwrap()
        .0;
    let one = full
        .worlds()
        .find(|(w, _)| w.hand(Seat::S2) == tiles(&["5-5", "1-0"]))
        .unwrap()
        .0;
    let b = Belief::from_weights(
        full.frame().clone(),
        "two equally weighted worlds",
        [(*two, q(1, 1)), (*one, q(1, 1))],
        2,
    )
    .unwrap();
    let summary = compile(COUNTS).summarize(&b, &mut budget()).unwrap();
    assert_eq!(summary.event_probability(), q(1, 1));
    assert_eq!(
        summary
            .answer_presence_mass
            .values()
            .cloned()
            .sum::<BigRational>()
            / &summary.total_mass,
        q(3, 2)
    );
    assert!(!summary.certainty.world_functional);
    assert_eq!(summary.certainty.constant_multiplicity, None);
    let uniform = summary.select(Selection::UniformWithinWorld);
    let first = summary.select(Selection::LexicographicFirst);
    assert_eq!(
        uniform.probabilities[&one_answer(Value::Domino(tile("3-2")))],
        q(1, 4)
    );
    assert_eq!(
        first.probabilities[&one_answer(Value::Domino(tile("3-2")))],
        q(1, 2)
    );
    assert_eq!(
        uniform.probabilities[&one_answer(Value::Domino(tile("5-5")))],
        q(1, 2)
    );
    assert_eq!(
        uniform.probabilities.values().cloned().sum::<BigRational>(),
        q(1, 1)
    );
}

#[test]
fn certainty_does_not_confuse_existence_with_identity() {
    let b = belief();
    let holder = compile("(fix (roles (chair holder)) (out holder) (case (holds holder 5-5)))");
    let s = holder.summarize(&b, &mut budget()).unwrap();
    assert!(s.certainty.event_certain && s.certainty.world_functional);
    assert_eq!(s.certainty.constant_multiplicity, Some(1));
    assert!(s.certainty.identity.is_none());
    let (posterior, p) = b
        .condition(
            &compile("(fix (roles) (out) (case (holds S2 5-5)))"),
            "observed holder",
            &mut budget(),
        )
        .unwrap();
    assert_eq!(p, q(1, 3));
    assert_eq!(
        holder
            .summarize(&posterior, &mut budget())
            .unwrap()
            .certainty
            .identity,
        Some(one_answer(Value::Chair(Seat::S2)))
    );
    assert_eq!(holder.summarize(&b, &mut budget()).unwrap(), s);
}

#[test]
fn conditioning_uses_events_and_exact_likelihoods_transactionally() {
    let b = belief();
    let (posterior, p) = b
        .condition(&compile(HAS_COUNT), "partner has count", &mut budget())
        .unwrap();
    assert_eq!(posterior.len(), 72);
    assert_eq!(p, q(4, 5));
    let (_, p) = b
        .condition_likelihood("soft evidence", |w| {
            Ok(if w.hand(Seat::S2).contains(tile("5-5")) {
                q(1, 2)
            } else {
                q(1, 4)
            })
        })
        .unwrap();
    assert_eq!(p, q(1, 3));
    assert!(b
        .condition(&compile("(fix (roles) (out))"), "impossible", &mut budget())
        .is_err());
    assert!(b.condition_likelihood("invalid", |_| Ok(q(2, 1))).is_err());
    assert!(b.condition_likelihood("invalid", |_| Ok(q(-1, 1))).is_err());
    assert_eq!(b.len(), 90);
}

#[test]
fn duplicate_worlds_merge_but_bad_measures_and_caps_are_refused() {
    let b = belief();
    let w = *b.worlds().next().unwrap().0;
    let merged = Belief::from_weights(
        b.frame().clone(),
        "duplicate physical worlds",
        [(w, q(1, 3)), (w, q(2, 3))],
        1,
    )
    .unwrap();
    assert_eq!(merged.len(), 1);
    assert_eq!(*merged.total_mass(), q(1, 1));
    assert!(Belief::from_weights(b.frame().clone(), "bad", [(w, q(-1, 1))], 1).is_err());
    assert!(Belief::from_weights(b.frame().clone(), "empty", [(w, q(0, 1))], 1).is_err());
    assert!(Belief::uniform(b.frame().clone(), 89).is_err());
    let invalid = b.frame().kernel().world([DominoSet::EMPTY; 3]);
    assert!(
        Belief::from_weights(b.frame().clone(), "invalid world", [(invalid, q(1, 1))], 1).is_err()
    );
    assert!(compile(COUNTS)
        .evaluate(b.frame(), &invalid, &mut budget())
        .is_err());
}

#[test]
fn answer_equivalence_is_stronger_than_boolean_equivalence() {
    let a = compile("(fix (roles (domino d)) (out d) (case (tile d 0-0)))");
    let b = compile("(fix (roles (domino d)) (out d) (case (tile d 6-6)))");
    let measure = belief();
    assert!(a
        .compare(&b, &measure, Comparison::Existence, &mut budget())
        .unwrap()
        .is_none());
    let witness = a
        .compare(&b, &measure, Comparison::Answers, &mut budget())
        .unwrap()
        .unwrap();
    assert_ne!(witness.left, witness.right);
    assert!(measure.frame().kernel().contains(&witness.world));
    assert_eq!(
        a.evaluate(measure.frame(), &witness.world, &mut budget())
            .unwrap(),
        witness.left
    );
}

#[test]
fn declared_public_residue_drives_partial_trick_queries() {
    let f = frame("P6".parse().unwrap());
    let kernel = Kernel::new(
        f.kernel().decl(),
        Seat::S0,
        tiles(&["0-0"]),
        f.kernel().pool(),
        *f.kernel().hidden(),
    )
    .unwrap();
    let f = Frame::new(kernel, Seat::S0, vec![tile("6-6")], tiles(&["6-6"])).unwrap();
    let b = Belief::uniform(f, 90).unwrap();
    let forced = compile(include_str!(
        "../../scheme/examples/current-forced-play.scheme"
    ));
    for (w, _) in b.worlds() {
        let legal = walt::rules::rules::legal_plays(
            b.frame().kernel().decl(),
            w.hand(Seat::S1),
            Some(Context::Called),
        );
        let expected = if legal.len() == 1 {
            BTreeSet::from([Answer(vec![
                Value::Chair(Seat::S1),
                Value::Domino(legal.iter().next().unwrap()),
            ])])
        } else {
            BTreeSet::new()
        };
        assert_eq!(
            forced.evaluate(b.frame(), w, &mut budget()).unwrap(),
            expected
        );
    }
    let event = compile("(fix (roles) (out) (case (leader S0) (current-winner S0) (next-actor S1) (led-context q*) (played 6-6) (not (live 6-6))))");
    assert_eq!(
        event
            .summarize(&b, &mut budget())
            .unwrap()
            .event_probability(),
        q(1, 1)
    );
}

#[test]
fn undefined_is_not_false_and_never_satisfies_negation() {
    let b = belief();
    for s in [
        "(fix (roles) (out) (case (led-context q*)))",
        "(fix (roles) (out) (case (not (led-context q*))))",
        "(fix (roles) (out) (case (not (legal S1 5-5))))",
    ] {
        assert!(compile(s)
            .summarize(&b, &mut budget())
            .unwrap()
            .event_mass
            .is_zero());
    }
}

#[test]
fn work_exhaustion_refuses_a_partial_query_or_summary() {
    let b = belief();
    let c = compile(COUNTS);
    assert!(c.summarize(&b, &mut Budget::new(1)).is_err());
    assert!(c
        .evaluate(b.frame(), b.worlds().next().unwrap().0, &mut Budget::new(0))
        .is_err());
    assert_eq!(
        c.summarize(&b, &mut budget()).unwrap().event_probability(),
        q(4, 5)
    );
}

struct ViewerProbe(PredicateSpec);
impl Predicate for ViewerProbe {
    fn spec(&self) -> &PredicateSpec {
        &self.0
    }
    fn evaluate(
        &self,
        ctx: PredicateContext<'_>,
        args: &[Value],
        budget: &mut Budget,
    ) -> Result<Option<bool>> {
        assert!(
            ctx.world().is_none(),
            "World must not be available to a viewer-only computation"
        );
        budget.spend(2)?;
        match args {
            [Value::Domino(d)] => Ok(Some(ctx.frame().kernel().viewer_hand().contains(*d))),
            _ => panic!("compiler checked signature"),
        }
    }
}
#[test]
fn extension_registry_freezes_semantics_and_enforces_information_access() {
    let mut r = Registry::standard();
    let spec = PredicateSpec {
        name: "mine".to_owned(),
        version: "test-v1".to_owned(),
        parameters: vec![Sort::Domino],
        horizon_plies: 0,
        access: Access::Viewer,
    };
    r.register(ViewerProbe(spec.clone())).unwrap();
    assert!(r.register(ViewerProbe(spec)).is_err());
    let query = "(fix (roles (domino d)) (out d) (case (mine d)))"
        .parse::<Fix>()
        .unwrap()
        .compile(&r)
        .unwrap();
    let b = belief();
    let s = query.summarize(&b, &mut budget()).unwrap();
    assert_eq!(s.certainty.constant_multiplicity, Some(2));
    assert!(query.identity().contains("test-v1"));
    assert_eq!(
        s.certainty.constant_answers.unwrap(),
        tiles(&["6-6", "0-0"])
            .iter()
            .map(|d| one_answer(Value::Domino(d)))
            .collect()
    );
}

#[test]
fn zero_weight_worlds_do_not_affect_belief_certainty() {
    let full = belief();
    let worlds: Vec<_> = full.worlds().take(2).map(|(w, _)| *w).collect();
    let b = Belief::from_weights(
        full.frame().clone(),
        "point mass",
        [
            (worlds[0], BigRational::one()),
            (worlds[1], BigRational::zero()),
        ],
        2,
    )
    .unwrap();
    assert_eq!(b.len(), 1);
    let query = compile("(fix (roles (chair c)) (out c) (case (holds c 5-5)))");
    assert!(query
        .summarize(&b, &mut budget())
        .unwrap()
        .certainty
        .identity
        .is_some());
}

#[test]
fn receipt_cli_queries_conditions_and_refuses_without_partial_reports() {
    use std::process::Command;
    let query = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../scheme/examples/partner-count.scheme");
    let condition = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../scheme/examples/partner-has-count.scheme");
    let run = |extra: &[&str]| {
        Command::new(env!("CARGO_BIN_EXE_scheme"))
            .arg("--query")
            .arg(&query)
            .args(["--hand", "0", "--trick", "6", "--seat", "0"])
            .args(extra)
            .output()
            .unwrap()
    };
    let result = run(&[]);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let text = String::from_utf8(result.stdout).unwrap();
    // Independently count the three possible holders of the only live count
    // tile (4-1) in this receipt coordinate: 2 of the 6 worlds put it at S2.
    assert!(text.contains("support-worlds: 6"));
    assert!(text.contains("event-probability: 1/3"));
    assert!(text.contains("[count-tile=4-1] probability=1/3"));
    let result = run(&["--condition", condition.to_str().unwrap()]);
    assert!(result.status.success());
    let text = String::from_utf8(result.stdout).unwrap();
    assert!(text.contains("positive-worlds: 2"));
    assert!(text.contains("\nevent-probability: 1\n"));
    for extra in [["--work", "1"], ["--max-worlds", "5"]] {
        let result = run(&extra);
        assert!(!result.status.success());
        assert!(result.stdout.is_empty(), "no partial report on refusal");
    }
}

#[test]
fn mastery_beating_and_negative_joins_match_direct_set_queries() {
    let masters = compile("(fix (roles (domino d)) (out d) (case (master d)))");
    let beaters = compile(
        "(fix (roles (domino a) (domino witness)) (out a)
        (case (holds S2 a) (holds S1 witness) (beats a witness q*) (not (double a))))",
    );
    for decl in Decl::ALL {
        let b = Belief::uniform(frame(decl), 90).unwrap();
        for (world, _) in b.worlds() {
            let expected = b
                .frame()
                .kernel()
                .masters()
                .iter()
                .map(|d| one_answer(Value::Domino(d)))
                .collect();
            assert_eq!(
                masters.evaluate(b.frame(), world, &mut budget()).unwrap(),
                expected
            );
            let expected = world
                .hand(Seat::S2)
                .iter()
                .filter(|a| {
                    !a.is_double()
                        && world.hand(Seat::S1).iter().any(|w| {
                            decl.trick_key(*a, Context::Called) > decl.trick_key(w, Context::Called)
                        })
                })
                .map(|a| one_answer(Value::Domino(a)))
                .collect();
            assert_eq!(
                beaters.evaluate(b.frame(), world, &mut budget()).unwrap(),
                expected
            );
        }
    }
}

#[test]
fn frames_reject_incoherent_history_and_do_not_invent_played_tiles() {
    let f = frame("NT".parse().unwrap());
    assert!(Frame::new(
        f.kernel().clone(),
        Seat::S0,
        vec![tile("6-6")],
        tiles(&["6-6"])
    )
    .is_err());
    assert!(Frame::new(
        f.kernel().clone(),
        Seat::S0,
        vec![tile("4-4")],
        DominoSet::EMPTY
    )
    .is_err());
    assert!(Frame::new(
        f.kernel().clone(),
        Seat::S0,
        vec![tile("4-4"), tile("4-4")],
        tiles(&["4-4"])
    )
    .is_err());
    let b = Belief::uniform(f, 90).unwrap();
    assert_eq!(
        compile("(fix (roles) (out) (case (not (live 4-4)) (not (played 4-4))))")
            .summarize(&b, &mut budget())
            .unwrap()
            .event_probability(),
        q(1, 1)
    );
}
