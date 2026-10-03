use std::time::Duration;
use walt::rules::Decl;
use walt::solver::{SplitMix64, FULL_MASK};
use walt_response_ladder::mechanics::{Budget, DiceField, Field, FieldQuery, Problem, PublicState, Scenario, tiles};
use walt_response_ladder::rollout::{evaluate, evaluate_dice, evaluate_dice_fast, evaluate_traces, priority_plans};

fn fixture(decl: Decl, depth: usize, partial: usize, rotation: usize, seed: u64) -> Problem {
    let mut rng = SplitMix64(seed);
    let mut deck: Vec<_> = (0u8..28).collect();
    for i in (1..28).rev() { let j = rng.below((i+1) as u64) as usize; deck.swap(i, j); }
    let mut hands = [0u32; 4];
    for (i, &tile) in deck.iter().enumerate() { hands[(i/7+rotation)%4] |= 1 << tile; }
    let mut public = PublicState::opening(rotation as u8);
    for _ in 0..(7-depth)*4+partial {
        let seat = public.actor();
        let legal = tiles(public.legal(decl, hands[seat as usize]));
        let tile = legal[rng.below(legal.len() as u64) as usize];
        public = public.after(decl, tile);
    }
    for hand in &mut hands { *hand &= !public.played; }
    let viewer = public.actor();
    let remaining = 42-public.banked_t1-public.banked_t0;
    let bid = public.banked_t1+remaining.div_ceil(2);
    assert!(public.payoff(bid, viewer).is_none());
    let scenarios = [0, 1, u64::MAX, 0x243f6a8885a308d3].into_iter().enumerate()
        .map(|(i,tape)| Scenario { hands, tape, weight: [1,3,7,2][i] }).collect();
    Problem { decl, bid, viewer, hand: hands[viewer as usize], root: public,
        scenarios, field_revision: "historical-dice-v1".into() }
}

fn candidates(problem: &Problem) -> Vec<Vec<u8>> {
    let all = priority_plans(problem, None).unwrap();
    if problem.hand.count_ones() <= 4 { return all; }
    let mut out = Vec::new();
    for root in tiles(problem.root.legal(problem.decl, problem.hand)) {
        let group: Vec<_> = all.iter().filter(|p| p[0] == root).collect();
        for i in [0,group.len()/2,group.len()-1] { out.push(group[i].clone()); }
    }
    out
}

#[test]
fn canonical_and_compact_scalar_agree_over_all_declarations_and_root_shapes() {
    let mut seen = [[[false; 2]; 4]; 6];
    let mut distinct_tape_outcome = false;
    for decl in Decl::ALL {
        for depth in 2..=7 {
            for partial in 0..4 {
                for rotation in 0..2 {
                    let problem = fixture(decl, depth, partial, rotation, 420601);
                    seen[depth-2][partial][problem.viewer as usize % 2] = true;
                    let plans = candidates(&problem);
                    let canonical = evaluate_dice(&problem, &plans).unwrap();
                    assert_eq!(canonical, evaluate_dice_fast(&problem, &plans).unwrap(),
                        "{decl:?}, depth={depth}, partial={partial}, rotation={rotation}");
                    assert!(canonical.iter().all(|row| row.len() == 4));
                    distinct_tape_outcome |= canonical.iter().any(|row| row.iter().any(|v| *v != row[0]));
                }
            }
        }
    }
    assert!(seen.iter().flatten().flatten().all(|x| *x));
    assert!(distinct_tape_outcome, "duplicate deals retain distinct tape-dependent outcomes");
}

#[test]
fn full_opening_family_has_5040_lawful_rows_and_preserves_scenario_order() {
    let problem = fixture(Decl::NoTrump, 7, 0, 1, 81119);
    let plans = priority_plans(&problem, None).unwrap();
    assert_eq!(plans.len(), 5040);
    let canonical = evaluate_dice(&problem, &plans).unwrap();
    assert_eq!(canonical, evaluate_dice_fast(&problem, &plans).unwrap());
    let mut reordered = problem.clone();
    reordered.scenarios.swap(0,3);
    reordered.scenarios[1].weight = 999;
    let swapped = evaluate_dice_fast(&reordered, &plans).unwrap();
    for (a,b) in canonical.iter().zip(swapped) { assert_eq!([a[3],a[1],a[2],a[0]], b.as_slice()); }
    let root = plans[0][0];
    assert_eq!(priority_plans(&problem, Some(&[root])).unwrap().len(), 720);
}

#[test]
fn malformed_plans_wrong_fields_and_interruption_never_return_partial_payoffs() {
    let mut problem = fixture(Decl::DoublesTrump, 4, 2, 0, 9001);
    let plans = candidates(&problem);
    assert_eq!(evaluate_dice(&problem, &[]).unwrap(), Vec::<Vec<u32>>::new());
    assert!(evaluate_dice(&problem, &[vec![plans[0][0]]]).is_err());
    let mut bad = plans[0].clone(); bad[1] = bad[0];
    assert!(evaluate_dice(&problem, &[bad]).is_err());
    assert!(priority_plans(&problem, Some(&[28])).is_err());
    assert!(evaluate(&problem, &plans, &mut DiceField,
        &mut Budget::new(0, Duration::from_secs(10))).is_err());
    struct Changing(bool);
    impl Field for Changing {
        fn revision(&self) -> &str { if self.0 { "changed" } else { "historical-dice-v1" } }
        fn choose(&mut self, q: FieldQuery<'_>, _: &mut Budget) -> Option<u8> {
            self.0 = true;
            Some(q.public.legal(q.decl, q.hand).trailing_zeros() as u8)
        }
    }
    assert!(evaluate(&problem, &plans, &mut Changing(false),
        &mut Budget::new(u64::MAX, Duration::from_secs(10))).unwrap_err().contains("revision changed"));
    problem.field_revision = "different-field".into();
    assert!(evaluate_dice(&problem, &plans).is_err());
    assert!(evaluate_dice_fast(&problem, &plans).is_err());
}

#[test]
fn canonical_traces_are_legal_and_conserve_tiles_through_settlement() {
    let problem = fixture(Decl::DoublesTrump, 7, 3, 1, 18762);
    let plans = candidates(&problem);
    let rows = evaluate_traces(&problem, &plans, &mut DiceField,
        &mut Budget::new(u64::MAX, Duration::from_secs(60))).unwrap();
    for row in rows {
        for (lane, scenario) in row.into_iter().zip(&problem.scenarios) {
            assert!(lane.plays.len() <= 28);
            let mut public = problem.root.clone();
            for tile in lane.plays {
                let hand = scenario.hands[public.actor() as usize];
                assert_ne!(public.legal(problem.decl, hand) & (1 << tile), 0);
                public = public.after(problem.decl, tile);
                let remaining = scenario.hands.iter().fold(0, |a,h| a | (h & !public.played));
                assert_eq!(remaining | public.played, FULL_MASK);
                assert_eq!(remaining & public.played, 0);
            }
            assert_eq!(public.payoff(problem.bid, problem.viewer), Some(u64::from(lane.payoff)));
        }
    }
}

#[cfg(feature="gpu")]
#[test]
fn gpu_full_traces_and_directed_arithmetic_match_frozen_canonical_reference() {
    use walt_response_ladder::gpu::DiceRollout;
    use walt::solver::{mix, record_hash};
    let gpu = DiceRollout::new().expect("gpu feature requires the native Metal harness");
    eprintln!("GPU validation: {}", gpu.adapter_name());
    let mut lanes = 0;
    for decl in Decl::ALL {
        for depth in 2..=7 {
            for partial in 0..4 {
                for rotation in 0..2 {
                    let problem = fixture(decl, depth, partial, rotation, 420601);
                    let plans = candidates(&problem);
                    let canonical = evaluate_traces(&problem, &plans, &mut DiceField,
                        &mut Budget::new(u64::MAX, Duration::from_secs(60))).unwrap();
                    assert_eq!(canonical, gpu.evaluate_dice_traces(&problem, &plans).unwrap(),
                        "trace: {decl:?}, depth={depth}, partial={partial}, rotation={rotation}");
                    let expected: Vec<Vec<_>> = canonical.iter().map(|r| r.iter().map(|l| l.payoff).collect()).collect();
                    assert_eq!(expected, gpu.evaluate_dice(&problem, &plans).unwrap());
                    lanes += plans.len()*problem.scenarios.len();
                }
            }
        }
    }
    // Exercise the complete 7! family, larger scenario counts, and both row
    // and column ordering rather than only small candidate pools.
    let mut full = fixture(Decl::NoTrump, 7, 0, 0, 93991);
    let mut rng = SplitMix64(91827);
    for _ in 4..17 { let mut s = full.scenarios[0].clone(); s.tape = rng.next_u64(); full.scenarios.push(s); }
    let plans = priority_plans(&full, None).unwrap();
    let expected = evaluate_dice_fast(&full, &plans).unwrap();
    assert_eq!(expected, gpu.evaluate_dice(&full, &plans).unwrap());
    let mut subset = plans.iter().step_by(113).cloned().collect::<Vec<_>>();
    subset.reverse();
    full.scenarios.reverse();
    assert_eq!(evaluate_dice_fast(&full, &subset).unwrap(), gpu.evaluate_dice(&full, &subset).unwrap());
    assert!(gpu.evaluate_dice(&full, &[]).unwrap().is_empty());
    full.field_revision = "modeled-field".into();
    assert!(gpu.evaluate_dice(&full, &plans).is_err());

    fn unxor(v: u64, shift: u32) -> u64 {
        let mut x = v; let mut p = shift;
        while p < 64 { x ^= v >> p; p += shift; } x
    }
    fn inverse_odd(a: u64) -> u64 {
        let mut x = 1u64;
        for _ in 0..6 { x = x.wrapping_mul(2u64.wrapping_sub(a.wrapping_mul(x))); } x
    }
    fn inverse_mix(v: u64) -> u64 {
        let x = unxor(v,31).wrapping_mul(inverse_odd(0x94d049bb133111eb));
        let x = unxor(x,27).wrapping_mul(inverse_odd(0xbf58476d1ce4e5b9));
        unxor(x,30).wrapping_sub(0x9e3779b97f4a7c15)
    }
    let mut vectors = Vec::new();
    for n in 1u32..=28 {
        let zone = u64::MAX-u64::MAX%u64::from(n);
        for target in [0,u64::from(n)-1,u64::from(n),zone-1,zone,u64::MAX] {
            let seed = inverse_mix(target);
            assert_eq!(mix(seed), target);
            for partial in 0..4 {
                let public = fixture(Decl::NoTrump, 5, partial, (n%4) as usize, 12345).root;
                vectors.push((seed,n,public));
            }
        }
    }
    let actual = gpu.arithmetic_vectors(&vectors).unwrap();
    for ((seed,n,public), got) in vectors.iter().zip(actual) {
        assert_eq!((mix(*seed),SplitMix64(*seed).below(u64::from(*n)) as u32,record_hash(&public.key())), got,
            "seed={seed}, modulus={n}, partial={:?}", public.plays);
    }
    eprintln!("Validated {lanes} complete traces, {} full-family lanes, {} directed arithmetic vectors", expected.len()*17,vectors.len());
}
