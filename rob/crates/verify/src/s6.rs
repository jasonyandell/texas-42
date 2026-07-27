//! Stage S6 verification harness: the symbolic trace validator + full-hand
//! dynamics (BRIEF_SLICE_02 §9, table S6). All rows are corpus-anchored.

use std::collections::HashSet;

use rob_core::{
    algebra_for, derive_rule_cells, game_observation, matching_minor_update, AbstractCells,
    DominoId, DominoSet, Play, Seat, SymbolicRejection, SymbolicTraceCertificate,
    TotalSupportNormalForm, HIDDEN_SEATS,
};

use crate::corpus::PlayedHand;
use crate::receipt::{fmt_commas, Receipt};
use crate::s3::{mechanical_trajectory, s3_corpus_hand};

/// The final support of a completed hand: empty pool, the (feasible) empty
/// determinate normal form.
fn final_support() -> (DominoSet, TotalSupportNormalForm) {
    let empty = AbstractCells::new(0, core::array::from_fn(|_| Vec::new()), [0, 0, 0])
        .expect("the empty system");
    (
        DominoSet::empty(),
        rob_core::compile_total_support(&empty, None),
    )
}

/// Build the honest certificate of one corpus hand (viewer seat 0).
pub fn certificate_of(hand: &PlayedHand) -> SymbolicTraceCertificate {
    let viewer = Seat::ALL[0];
    let (claimed_pool, claimed_final) = final_support();
    SymbolicTraceCertificate {
        viewer,
        viewer_initial_hand: *hand.deal.hand(viewer),
        contract: *hand.states[0].state().contract(),
        trace: hand
            .steps
            .iter()
            .map(|s| Play {
                actor: s.actor,
                domino: s.domino,
            })
            .collect(),
        claimed_pool,
        claimed_final,
    }
}

/// `r_sym_corpus` (REACH-14/15; TRANS-08): every corpus hand accepted; at
/// every transition the symbolic support, an independent S5 dynamics
/// evolution, and the NF recompiled from the mechanical state agree three
/// ways. Returns (hands, transitions, agreements).
pub fn corpus_check() -> (u64, u64, u64) {
    let viewer = Seat::ALL[0];
    let mut hands = 0u64;
    let mut transitions = 0u64;
    let mut agreements = 0u64;
    for index in 0..108 {
        let hand = s3_corpus_hand(index);
        let declaration = hand.declaration;
        let trajectory = mechanical_trajectory(&hand, viewer);
        // Independent S5 evolution (route 2).
        let mut dyn_nfs: Vec<TotalSupportNormalForm> = Vec::with_capacity(28);
        {
            let initial_cells = derive_rule_cells(&trajectory[0]);
            let (abstract_initial, mut order) = initial_cells.to_abstract();
            let mut nf = rob_core::compile_total_support(&abstract_initial, None);
            for step in &hand.steps {
                if step.actor != viewer {
                    let led = step
                        .trick_before
                        .first()
                        .map(|p| algebra_for(declaration).led_suit(p.domino));
                    let observation = game_observation(declaration, led, step.domino, &order);
                    let position = order
                        .iter()
                        .position(|&d| d == step.domino)
                        .expect("hidden play from pool");
                    let seat = step.actor.index() - 1;
                    let (next, _) = matching_minor_update(&nf, seat, position, &observation);
                    order.remove(position);
                    nf = next;
                }
                dyn_nfs.push(nf.clone());
            }
        }
        // Route 3: NF recompiled from the mechanical state after each play.
        let mech_nfs: Vec<TotalSupportNormalForm> = (1..=28)
            .map(|t| {
                let cells = derive_rule_cells(&trajectory[t]);
                let (abstract_cells, _) = cells.to_abstract();
                rob_core::compile_total_support(&abstract_cells, None)
            })
            .collect();
        // Route 1: the symbolic validator itself.
        let mut symbolic_ok = 0u64;
        let accepted = rob_core::validate_symbolic_trace_with(
            &certificate_of(&hand),
            |play_index, support, _order| {
                assert_eq!(support, &dyn_nfs[play_index], "symbolic ≡ S5 evolution");
                assert_eq!(
                    support, &mech_nfs[play_index],
                    "symbolic ≡ mechanical recompile"
                );
                symbolic_ok += 1;
            },
        );
        assert!(accepted.is_ok(), "every corpus hand is accepted");
        assert_eq!(symbolic_ok, 28);
        hands += 1;
        transitions += 28;
        agreements += 28;
    }
    (hands, transitions, agreements)
}

/// `r_sym_budget` + `inv_edge_budget` (TRANS-12/14; INV-11): per-hand
/// deletion ledgers total exactly 63 with zero reappearances and at most 2
/// informational deletions per live tile. Returns the grand total.
pub fn budget_check() -> u64 {
    let mut grand_total = 0u64;
    for index in 0..108 {
        let hand = s3_corpus_hand(index);
        let certificate = certificate_of(&hand);
        let accepted =
            rob_core::validate_symbolic_trace(&certificate).expect("corpus hand accepted");
        let ledger = &accepted.deletion_ledger;
        assert_eq!(ledger.len(), 63, "each of the 63 edges dies exactly once");
        let mut seen: HashSet<(DominoId, usize)> = HashSet::new();
        let mut informational: std::collections::HashMap<DominoId, u32> =
            std::collections::HashMap::new();
        for &(trace_index, tile, seat) in ledger {
            assert!(seat < HIDDEN_SEATS);
            assert!(
                seen.insert((tile, seat)),
                "no holder edge ever reappears (INV-11)"
            );
            if certificate.trace[trace_index].domino != tile {
                *informational.entry(tile).or_insert(0) += 1;
            }
        }
        for (&tile, &count) in &informational {
            assert!(
                count <= 2,
                "at most 2 informational deletions per live tile ({tile:?}: {count})"
            );
        }
        grand_total += ledger.len() as u64;
    }
    grand_total
}

/// `r_sym_reject` (REACH-14; Exec §18): the constructive mutation battery —
/// every mutated certificate is rejected with the expected typed reason.
pub fn reject_check() -> u64 {
    let viewer = Seat::ALL[0];
    let mut rejections = 0u64;
    for index in 0..108 {
        let hand = s3_corpus_hand(index);
        let honest = certificate_of(&hand);

        // m1: replace the final play's tile with a tile the same actor
        // already played — an empty conditioned successor (hidden) or a
        // duplicate viewer play.
        {
            let mut mutated = honest.clone();
            let last_actor = mutated.trace[27].actor;
            let earlier = mutated.trace[..27]
                .iter()
                .find(|p| p.actor == last_actor)
                .expect("every seat played earlier")
                .domino;
            mutated.trace[27].domino = earlier;
            let expected = if last_actor == viewer {
                SymbolicRejection::ViewerNotHolding
            } else {
                SymbolicRejection::EmptyConditionedSupport
            };
            assert_eq!(
                rob_core::validate_symbolic_trace(&mutated).unwrap_err(),
                expected,
                "m1 rejection reason"
            );
            rejections += 1;
        }

        // m2: replace the viewer's last play with the viewer's first played
        // tile — viewer illegality.
        {
            let mut mutated = honest.clone();
            let viewer_plays: Vec<usize> = mutated
                .trace
                .iter()
                .enumerate()
                .filter(|(_, p)| p.actor == viewer)
                .map(|(i, _)| i)
                .collect();
            let first_tile = mutated.trace[viewer_plays[0]].domino;
            let last_index = *viewer_plays.last().expect("seven viewer plays");
            mutated.trace[last_index].domino = first_tile;
            assert_eq!(
                rob_core::validate_symbolic_trace(&mutated).unwrap_err(),
                SymbolicRejection::ViewerNotHolding,
                "m2 rejection reason"
            );
            rejections += 1;
        }

        // m3: claim the initial unrestricted support as final — mismatch.
        {
            let mut mutated = honest.clone();
            let pool = DominoSet::full().difference(&mutated.viewer_initial_hand);
            let (abstract_initial, _) = {
                let tiles: Vec<DominoId> = pool.iter().collect();
                (
                    AbstractCells::new(
                        tiles.len(),
                        core::array::from_fn(|_| vec![true; tiles.len()]),
                        [7; HIDDEN_SEATS],
                    )
                    .expect("unrestricted"),
                    tiles,
                )
            };
            mutated.claimed_pool = pool;
            mutated.claimed_final = rob_core::compile_total_support(&abstract_initial, None);
            assert_eq!(
                rob_core::validate_symbolic_trace(&mutated).unwrap_err(),
                SymbolicRejection::FinalSupportMismatch,
                "m3 rejection reason"
            );
            rejections += 1;
        }
    }
    rejections
}

/// Build the canonical S6 receipt. All lines are corpus-anchored.
pub fn receipt() -> String {
    let mut r = Receipt::new("S6");
    let (hands, transitions, agreements) = corpus_check();
    r.line(
        "r_sym_corpus",
        &format!(
            "{hands} hands; {} transitions; {} three-way agreements",
            fmt_commas(transitions as u128),
            fmt_commas(agreements as u128)
        ),
    );
    r.line(
        "r_sym_budget",
        &format!(
            "{} deletions; 63 per hand; no reappearance",
            fmt_commas(budget_check() as u128)
        ),
    );
    r.line("r_sym_reject", &reject_check().to_string());
    r.finish()
}
