//! EXPLORATORY DOOM-CENSUS INSTRUMENT (`solver::doom`; the §70
//! structural producer, ∀-fail dual of the §16 hierarchy) — sits below
//! every evidentiary tier and is cited by nothing above it. Instrument
//! output only: per opening-root action, the census's certified
//! counterexample mass, its deterministic upper, the class tallies,
//! and the walk bill; then the composed §65 panel — the same proof
//! state carrying the cheap sampled stops PLUS the doom uppers — so
//! the movement of `U*` and Γ under deterministic ceilings is read off
//! one closure. Never a play-strength claim.
//!
//! DECLARED EPOCH: the σ0 Level0 { n0 = 2 } modeled mind under
//! `SupportOracle`; the opening root h0-t1 of the frozen
//! `verify_player` receipt (fiber 399,072,960); census spec in the
//! REPORT constants below (node budgets, walk cap, max level 3,
//! top-8 priority, critical set empty); sampled tier at the Slice A
//! declaration,
//! δ = 1/100 per endpoint, prefixes 16 and 64 only (the cheap stops —
//! the doom uppers are the instrument here, not the sampled tier).
//!
//! Modes:
//!   `doomreport scout <idx> <nodes> <cap> <level> [top-k]`
//!       — one action, caller budgets, timing scout (exploratory)
//!   `doomreport enumscout <idx> <outer-limit>`
//!       — enumeration cost scout over an outer-support prefix
//!   `doomreport report <out.txt>` — the declared full run:
//!       receipt-root census-vs-truth, the h0-t1 priority census,
//!       the God grid, and the composed §65 panel
//!
//! No floats anywhere; wall time is integer microseconds; values are
//! exact rationals, printed alongside integer permille.

use std::io::Write as _;
use std::time::Instant;

use num_bigint::BigInt;
use num_rational::BigRational;
use walt::kernel::Kernel;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::rules::{Domino, DominoSet, Seat};
use walt::solver::adaptive::{CanonicalRoot, RootPosition, SlicePolicy};
use walt::solver::doom::{
    census_authority, census_fact, doom_census, doom_enumeration, DoomCensus, DoomSpec,
};
use walt::solver::factor_belief::{ExactCoverOracle, FactorBelief, SupportOracle};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::opening::{OpeningLadder, OpeningStopSpec};
use walt::solver::policy::{DecisionMode, TieRule};
use walt::solver::proof_state::{ProofState, SemanticsIdentity};

/// The declared report budgets: per-action census node budget, the
/// per-walk cap, and the descent depth.
const REPORT_NODE_BUDGET: u64 = 500_000;
const REPORT_WALK_CAP: u64 = 100_000;
const REPORT_MAX_LEVEL: usize = 3;
const REPORT_DESCEND_TOP: usize = 8;

fn field_spec() -> FieldSpec {
    FieldSpec {
        kind: FieldKind::Level0 { n0: 2 },
        construction: "level0-modeled-mind-v1".to_string(),
        practical_equivalence: None,
        fallback: "none".to_string(),
        seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        policy_library: "field-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
    }
}

fn root_at(r: &Receipt, hand_id: usize, trick_no: usize) -> (CanonicalRoot, RootPosition) {
    let hand = &r.hands[hand_id];
    assert_eq!(hand.id, hand_id);
    let kernel = Kernel::from_receipt_trick(hand, trick_no).expect("a valid kernel");
    let position = RootPosition::from_receipt_trick(hand, trick_no).expect("a valid position");
    (CanonicalRoot::new(kernel), position)
}

fn identity_of(root: &CanonicalRoot, position: &RootPosition) -> SemanticsIdentity {
    let declaring = root.kernel().viewer().team() == position.declaring_team;
    SemanticsIdentity {
        root_id: walt::solver::adaptive::root_identity(root, position),
        rules_id: "texas42-v1".to_string(),
        field_id: "level0-modeled-mind-v1".to_string(),
        utility_id: if declaring {
            "pmake-v1".to_string()
        } else {
            "pmake-setting-v1".to_string()
        },
        contract: position.bid,
        belief_id: "uniform-root".to_string(),
        policy_class_id: "information-consistent-full".to_string(),
        score_semantics_id: "declaring-banked-43bin-v1".to_string(),
    }
}

fn tiles(pairs: &[(usize, usize)]) -> DominoSet {
    let mut s = DominoSet::EMPTY;
    for (a, b) in pairs {
        assert!(
            s.insert(Domino::new(
                walt::rules::Pip::ALL[*a],
                walt::rules::Pip::ALL[*b]
            )),
            "distinct tiles"
        );
    }
    s
}

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(BigInt::from(n), BigInt::from(d))
}

fn permille(v: &BigRational) -> u128 {
    let scaled = (v * BigRational::from_integer(BigInt::from(1000))).floor();
    u128::try_from(scaled.to_integer()).expect("a permille of a probability fits u128")
}

fn legal_actions(root: &CanonicalRoot, position: &RootPosition) -> Vec<Domino> {
    let legal = walt::rules::legal_plays(position.decl, root.kernel().viewer_hand(), None);
    let mut out: Vec<Domino> = (0..DominoSet::FULL.len())
        .filter_map(Domino::from_index)
        .filter(|t| legal.contains(*t))
        .collect();
    out.sort_by_key(|t| t.index());
    out
}

fn print_census(out: &mut String, census: &DoomCensus, wall_us: u128) {
    out.push_str(&format!(
        "\n== ACTION {}: doomed {} of {} ({}‰) upper={}‰ wall={}us ==\n",
        census.action,
        census.doomed_mass,
        census.fiber,
        permille(&BigRational::new(
            BigInt::from(census.doomed_mass),
            BigInt::from(census.fiber)
        )),
        permille(&census.upper),
        wall_us
    ));
    out.push_str(&format!(
        "  classes: walked {} doomed {} survived {} refused {} empty {} | nodes {}{}\n",
        census.classes_walked,
        census.classes_doomed,
        census.classes_survived,
        census.classes_refused,
        census.classes_empty,
        census.nodes,
        if census.whole_fiber {
            " | WHOLE FIBER (level-0)"
        } else {
            ""
        }
    ));
    out.push_str(&format!(
        "  mass ledger: doomed {} survived {} refused {} (fiber {})\n",
        census.doomed_mass, census.survived_mass, census.refused_mass, census.fiber
    ));
    let mut leaves = census.doomed_leaves.clone();
    leaves.sort_by_key(|l| core::cmp::Reverse(l.mass));
    for leaf in leaves.iter().take(8) {
        let path: Vec<String> = leaf
            .path
            .iter()
            .map(|(seat, sig)| format!("{seat:?}:{sig:?}"))
            .collect();
        out.push_str(&format!(
            "  leaf mass {:>12} ({}‰) nodes {:>9} level {}: {}\n",
            leaf.mass,
            permille(&BigRational::new(
                BigInt::from(leaf.mass),
                BigInt::from(census.fiber)
            )),
            leaf.nodes,
            leaf.path.len(),
            path.join(" | ")
        ));
    }
    if leaves.len() > 8 {
        out.push_str(&format!(
            "  ... and {} more doomed leaves\n",
            leaves.len() - 8
        ));
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map(String::as_str).unwrap_or("");
    let r = parse_file(&locate_verify_player().expect("the receipt above the workspace"))
        .expect("the receipt parses");
    let (root, position) = root_at(&r, 0, 1);
    let field = FieldModel::new(field_spec());
    let oracle = SupportOracle;
    let actions = legal_actions(&root, &position);
    match mode {
        "scout" => {
            let idx: usize = args[2].parse().expect("an action index");
            let node_budget: u64 = args[3].parse().expect("a node budget");
            let walk_cap: u64 = args[4].parse().expect("a walk cap");
            let max_level: usize = args[5].parse().expect("a level");
            let descend_top: Option<usize> = args.get(6).map(|a| a.parse().expect("a top-k"));
            let spec = DoomSpec {
                node_budget,
                walk_cap,
                max_level,
                critical: DominoSet::EMPTY,
                descend_top,
            };
            let action = actions[idx];
            eprintln!("scout: action {action} spec {spec:?}");
            let t0 = Instant::now();
            let census = doom_census(&oracle, &root, &position, &field, action, &spec);
            let wall = t0.elapsed().as_micros();
            let mut out = String::new();
            print_census(&mut out, &census, wall);
            print!("{out}");
        }
        "enumscout" => {
            // Exploratory cost scout: the per-world enumeration over a
            // PREFIX of the outer support (biased sample — cost data
            // only, never a quotable doom rate).
            let idx: usize = args[2].parse().expect("an action index");
            let outer_limit: u64 = args[3].parse().expect("an outer-hand limit");
            let action = actions[idx];
            let level0 = walt::solver::policy::Level0Field::new(2);
            let spec = DoomSpec {
                node_budget: 0,
                walk_cap: 0,
                max_level: 1,
                critical: DominoSet::EMPTY,
                descend_top: None,
            };
            eprintln!("enumscout: action {action}, outer limit {outer_limit}");
            let t0 = Instant::now();
            let mut stopped = None;
            let mut progress = |done: u64, total: u64, doomed: u128, nodes: u64| {
                if done.is_multiple_of(8192) || done == outer_limit {
                    eprintln!(
                        "  {done}/{total} outer hands: doomed {doomed}, nodes {nodes}, wall {}us",
                        t0.elapsed().as_micros()
                    );
                }
                if done >= outer_limit && stopped.is_none() {
                    stopped = Some((done, doomed, nodes, t0.elapsed().as_micros()));
                    panic!("SCOUT-STOP");
                }
            };
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                doom_enumeration(
                    &oracle,
                    &root,
                    &position,
                    &level0,
                    action,
                    &spec,
                    &mut progress,
                )
            }));
            match result {
                Ok(e) => println!(
                    "full enumeration: doomed {} of {} nodes {} wall {}us",
                    e.doomed,
                    e.fiber,
                    e.nodes,
                    t0.elapsed().as_micros()
                ),
                Err(_) => {
                    let (done, doomed, nodes, wall) = stopped.expect("the scout stop fired");
                    println!(
                        "scout stop at {done} outer hands: doomed {doomed}, nodes {nodes}, wall {wall}us"
                    );
                }
            }
        }
        "report" => {
            let path = args.get(2).expect("an output path").clone();
            let spec = DoomSpec {
                node_budget: REPORT_NODE_BUDGET,
                walk_cap: REPORT_WALK_CAP,
                max_level: REPORT_MAX_LEVEL,
                critical: DominoSet::EMPTY,
                descend_top: Some(REPORT_DESCEND_TOP),
            };
            let full = DoomSpec {
                node_budget: 10_000_000,
                walk_cap: 1_000_000,
                max_level: 3,
                critical: DominoSet::EMPTY,
                descend_top: None,
            };
            let mut out = String::new();
            out.push_str(&format!(
                "DOOM CENSUS REPORT — field {}\n\
                 opening spec: node_budget={} walk_cap={} max_level={} top-k={} critical=empty\n\
                 authority: {}\n",
                SlicePolicy::id(&field),
                spec.node_budget,
                spec.walk_cap,
                spec.max_level,
                REPORT_DESCEND_TOP,
                census_authority(SlicePolicy::id(&field), &spec),
            ));
            let flush = |s: &str| {
                let mut f = std::fs::File::create(&path).expect("the output file opens");
                f.write_all(s.as_bytes()).expect("the output file writes");
            };
            flush(&out);

            // -------- Part 1: the enumerable receipt roots — the
            // instrument's positive domain. Full census vs the
            // per-world enumeration truth per action.
            out.push_str(
                "\n#### PART 1 — enumerable receipt roots: census vs per-world truth ####\n",
            );
            for (hand_id, trick_no) in [(12usize, 6usize), (10, 6), (5, 6), (4, 6), (8, 5), (3, 5)]
            {
                let (er, ep) = root_at(&r, hand_id, trick_no);
                let eb = FactorBelief::uniform_root(&er, &ep, &field);
                let z = oracle.mass(&eb);
                out.push_str(&format!("\nh{hand_id}-t{trick_no} (fiber {z}):\n"));
                for action in legal_actions(&er, &ep) {
                    let census = doom_census(&oracle, &er, &ep, &field, action, &full);
                    let mut progress = |_: u64, _: u64, _: u128, _: u64| {};
                    let e =
                        doom_enumeration(&oracle, &er, &ep, &field, action, &full, &mut progress);
                    out.push_str(&format!(
                        "  {action}: census doomed {} of {} ({} walk nodes) | \
                         per-world truth {} ({} search nodes) | census recovers {}\n",
                        census.doomed_mass,
                        z,
                        census.nodes,
                        e.doomed,
                        e.nodes,
                        match (census.doomed_mass * 1000).checked_div(e.doomed) {
                            None => "n/a (nothing to find)".to_string(),
                            Some(p) => format!("{p}‰ of the doom"),
                        }
                    ));
                }
                flush(&out);
            }

            // -------- Part 2: the h0-t1 opening root — the priority
            // census at the declared budgets.
            out.push_str("\n#### PART 2 — h0-t1 opening root: the priority census ####\n");
            flush(&out);
            let mut censuses: Vec<DoomCensus> = Vec::new();
            for action in &actions {
                let t0 = Instant::now();
                let census = doom_census(&oracle, &root, &position, &field, *action, &spec);
                let wall = t0.elapsed().as_micros();
                print_census(&mut out, &census, wall);
                flush(&out);
                censuses.push(census);
            }

            // -------- Part 3: the God grid — declared per-world
            // checks (a singleton class is a belief, so the exact
            // recursion IS a world-aware make check). Two hand-built
            // crusher worlds plus a declared stride grid over the
            // first-responder support. A structured grid, never a
            // probability estimate.
            out.push_str(
                "\n#### PART 3 — the God grid: world-aware make checks after the 0-0 lead ####\n",
            );
            flush(&out);
            {
                let belief = FactorBelief::uniform_root(&root, &position, &field);
                let lead = actions[0];
                assert_eq!(format!("{lead}"), "0-0", "the grid is the 0-0 story");
                let crushers: Vec<(&str, [DominoSet; 3])> = vec![
                    (
                        "crusher (S2 blank-count winner, opponents hold top trump + both tens + both loose fives, junk partner)",
                        [
                            tiles(&[(5, 0), (6, 3), (5, 5), (4, 0), (2, 2), (6, 1), (1, 1)]),
                            tiles(&[(1, 0), (3, 0), (4, 2), (4, 4), (5, 1), (5, 4), (6, 2)]),
                            tiles(&[(6, 4), (4, 3), (3, 1), (2, 0), (5, 2), (6, 6), (4, 1)]),
                        ],
                    ),
                    (
                        "trump-wall (S2 holds 6-3/4-3/3-1 over the viewer's 3-5, count behind it, junk partner)",
                        [
                            tiles(&[(6, 3), (4, 3), (3, 1), (5, 5), (6, 4), (5, 0), (1, 1)]),
                            tiles(&[(1, 0), (3, 0), (4, 2), (4, 4), (5, 1), (5, 4), (2, 2)]),
                            tiles(&[(2, 0), (5, 2), (6, 2), (6, 6), (4, 1), (6, 1), (4, 0)]),
                        ],
                    ),
                ];
                let seats = [Seat::ALL[2], Seat::ALL[3], Seat::ALL[0]];
                for (name, hands) in &crushers {
                    let mut b = belief.clone();
                    for (seat, h) in seats.iter().zip(hands.iter()) {
                        b = b.with_factor_table(*seat, vec![(*h, 1u128)]);
                    }
                    let z1 = oracle.mass(&b);
                    assert_eq!(z1, 1, "a declared crusher is one world");
                    let mut stats = walt::solver::factor_belief::ResponseStats::default();
                    let m = walt::solver::factor_belief::response_success_mass(
                        &oracle,
                        &b.focal_play(lead),
                        &field,
                        &mut stats,
                    );
                    out.push_str(&format!(
                        "  {name}:\n    => {}\n",
                        if m == 0 {
                            "DOOMED — even the world-aware viewer fails"
                        } else {
                            "the world-aware viewer MAKES vs the declared field"
                        }
                    ));
                }
                flush(&out);
                // The stride grid: every STRIDE-th first-responder
                // hand, its lexicographically first completion.
                const STRIDE: usize = 512;
                let s2 = Seat::ALL[2];
                let s2_hands: Vec<DominoSet> = belief.factors()[0]
                    .support()
                    .into_iter()
                    .map(|(h, _)| h)
                    .collect();
                let pool = root.kernel().pool();
                let mut grid_worlds = 0u64;
                let mut grid_doomed = 0u64;
                let t0 = Instant::now();
                for h2 in s2_hands.iter().step_by(STRIDE) {
                    let rest = pool.difference(*h2);
                    let h3: DominoSet = rest.iter().take(7).collect();
                    let h0 = rest.difference(h3);
                    let mut b = belief.clone();
                    b = b.with_factor_table(s2, vec![(*h2, 1)]);
                    b = b.with_factor_table(Seat::ALL[3], vec![(h3, 1)]);
                    b = b.with_factor_table(Seat::ALL[0], vec![(h0, 1)]);
                    assert_eq!(oracle.mass(&b), 1, "a grid point is one world");
                    let mut stats = walt::solver::factor_belief::ResponseStats::default();
                    let m = walt::solver::factor_belief::response_success_mass(
                        &oracle,
                        &b.focal_play(lead),
                        &field,
                        &mut stats,
                    );
                    grid_worlds += 1;
                    if m == 0 {
                        grid_doomed += 1;
                    }
                }
                out.push_str(&format!(
                    "  stride-{STRIDE} grid over the S2 support (lex-first completions): \
                     {grid_doomed} of {grid_worlds} grid worlds doomed for the \
                     world-aware viewer | wall {}us\n\
                     (a declared structured grid — NOT a probability estimate)\n",
                    t0.elapsed().as_micros()
                ));
                flush(&out);
            }
            // The composed §65 panel: cheap sampled stops + doom facts.
            let identity = identity_of(&root, &position);
            let mut state = ProofState::open(&root, &position, identity.clone());
            let ladder = OpeningLadder {
                oracle: &oracle,
                root: &root,
                position: &position,
                field: &field,
                scope_budget: q(3, 5),
                epsilon: q(1, 4),
            };
            for (label, prefix) in [("p16", 16u64), ("p64", 64u64)] {
                let stop = OpeningStopSpec {
                    label: label.to_string(),
                    sampled_prefix: prefix,
                    endpoint_delta: q(1, 100),
                    census: false,
                    frontier_budget: 0,
                };
                let t0 = Instant::now();
                let report = ladder.run_stop(&mut state, &stop);
                out.push_str(&format!(
                    "\n== SAMPLED STOP {label}: facts={} bar={}‰ U*={}‰ regret={}‰ wall={}us ==\n",
                    report.facts,
                    permille(&report.proof_bar),
                    permille(&report.global_upper),
                    permille(&report.certified_regret),
                    t0.elapsed().as_micros()
                ));
                flush(&out);
            }
            let before = state.closure();
            out.push_str(&format!(
                "\n== PANEL BEFORE DOOM: bar={}‰ U*={}‰ regret={}‰ ==\n",
                permille(&before.bar),
                permille(&before.u_star),
                permille(&before.certified_regret)
            ));
            for v in &before.views {
                out.push_str(&format!(
                    "  {}: [{:>4},{:>4}]‰{}\n",
                    v.action,
                    permille(&v.lower),
                    permille(&v.upper),
                    if v.excluded { " EXCLUDED" } else { "" }
                ));
            }
            let mut installed = 0usize;
            for census in &censuses {
                if let Some(fact) = census_fact(census, SlicePolicy::id(&field), &spec) {
                    state.install(&identity, fact).expect("a doom upper lands");
                    installed += 1;
                }
            }
            let after = state.closure();
            out.push_str(&format!(
                "\n== PANEL AFTER DOOM ({installed} uppers): bar={}‰ U*={}‰ regret={}‰ ==\n",
                permille(&after.bar),
                permille(&after.u_star),
                permille(&after.certified_regret)
            ));
            for v in &after.views {
                out.push_str(&format!(
                    "  {}: [{:>4},{:>4}]‰{}{}\n",
                    v.action,
                    permille(&v.lower),
                    permille(&v.upper),
                    if v.upper_sampled { "" } else { " det-upper" },
                    if v.excluded { " EXCLUDED" } else { "" }
                ));
            }
            if let Some(rec) = after.exec.as_ref() {
                out.push_str(&format!(
                    "  exec bar: {} via {} = {}‰\n",
                    rec.action,
                    rec.authority,
                    permille(&rec.value)
                ));
            }
            let z = oracle.mass(&FactorBelief::uniform_root(&root, &position, &field));
            let total_doom: u128 = censuses.iter().map(|c| c.doomed_mass).sum();
            out.push_str(&format!(
                "\ntotal certified doom across actions: {total_doom} worlds \
                 (fiber {z} per action)\n"
            ));
            out.push_str(
                "\nEXPLORATORY — below every evidentiary tier; quotable only via gate receipts.\n",
            );
            flush(&out);
            println!("{out}");
        }
        _ => {
            eprintln!(
                "usage: doomreport scout <idx> <nodes> <cap> <level> | doomreport report <out.txt>"
            );
            std::process::exit(2);
        }
    }
}
