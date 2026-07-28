//! Player-track stage P2 verification harness: the trick-boundary position
//! corpus and exact fiber accounting (BRIEF_PLAYER_01 §8 table P2;
//! CELL-05/10/25/26).

use std::collections::BTreeSet;

use num_bigint::BigUint;
use num_traits::{One, ToPrimitive, Zero};
use rob_core::{
    count_occupancy_dp, derive_rule_cells, rank_world, unrank_world, MechanicalState, Seat,
};
use rob_player::window::{window_depth, WINDOW_BUDGET};

use crate::receipt::{fmt_commas, Receipt};
use crate::s3::{mechanical_trajectory, s3_corpus_hand};

/// Trick boundaries: positions exist after `t = 0..=6` completed tricks.
pub const BOUNDARIES: usize = 7;

/// One position of the P2 corpus: S3 corpus hand `index` truncated after
/// `boundary` completed tricks, viewed by the seat about to lead the next
/// trick (the bidder at boundary 0 — R-LEAD — else the last trick's winner).
pub fn boundary_position(index: u64, boundary: usize) -> MechanicalState {
    assert!(index < 108 && boundary < BOUNDARIES);
    let hand = s3_corpus_hand(index);
    let viewer: Seat = if boundary == 0 {
        hand.bidder
    } else {
        hand.steps[4 * boundary - 1]
            .trick_result
            .expect("play 4t completes trick t")
            .winner
    };
    let state = mechanical_trajectory(&hand, viewer)[4 * boundary].clone();
    assert_eq!(
        state.current_actor(),
        Some(viewer),
        "the viewer is the seat to act at its own boundary position"
    );
    assert!(state.current_trick().is_empty(), "trick-boundary position");
    assert_eq!(state.own_remaining_hand().len(), 7 - boundary);
    state
}

/// The closed-form no-void fiber bound at boundary `t`:
/// `(21 − 3t)! / ((7 − t)!)³`, computed from factorials (never hard-coded).
pub fn boundary_bound(boundary: usize) -> BigUint {
    let factorial = |n: usize| -> BigUint {
        (1..=n)
            .map(BigUint::from)
            .fold(BigUint::one(), |a, b| a * b)
    };
    let denom = factorial(7 - boundary);
    factorial(21 - 3 * boundary) / (&denom * &denom * &denom)
}

/// `r_pos_corpus` (corpus-shape): all 756 positions decode with the viewer
/// to act at a trick boundary, 108 per depth. Returns (positions, depths).
pub fn corpus_check() -> (u64, usize) {
    let mut positions = 0u64;
    for boundary in 0..BOUNDARIES {
        for index in 0..108 {
            let _ = boundary_position(index, boundary);
            positions += 1;
        }
    }
    assert_eq!(positions, 756);
    (positions, BOUNDARIES)
}

/// `r_pos_count` (CELL-10; Math §7): per position the capacity-DP fiber
/// count obeys its closed-form bound, and at boundary 0 it equals the bound
/// (399,072,960 — before any observation, no world is excluded). Returns
/// (bound checks, boundary-0 equalities, grand total of all 756 counts).
pub fn count_check() -> (u64, u64, BigUint) {
    let mut checks = 0u64;
    let mut equalities = 0u64;
    let mut total = BigUint::zero();
    for boundary in 0..BOUNDARIES {
        let bound = boundary_bound(boundary);
        for index in 0..108 {
            let state = boundary_position(index, boundary);
            let (abstract_cells, _) = derive_rule_cells(&state).to_abstract();
            let count = count_occupancy_dp(&abstract_cells).count;
            assert!(count <= bound, "fiber counts obey the closed-form bound");
            if boundary == 0 {
                assert_eq!(count, bound, "boundary 0 excludes no world");
                equalities += 1;
            }
            total += &count;
            checks += 1;
        }
    }
    assert_eq!(
        boundary_bound(0),
        BigUint::from(399_072_960u64),
        "the boundary-0 bound is the Math §7 hidden-assignment count"
    );
    (checks, equalities, total)
}

/// `r_pos_fiber`, enumerated half (CELL-05/10): for the 432 positions with
/// boundary ≥ 3, `fiber_worlds` agrees with the capacity-DP count, every
/// world passes `fiber_contains`, and there are no duplicates.
pub fn fiber_enumerated_check() -> u64 {
    let mut agreements = 0u64;
    for boundary in 3..BOUNDARIES {
        for index in 0..108 {
            let state = boundary_position(index, boundary);
            let cells = derive_rule_cells(&state);
            let (abstract_cells, _) = cells.to_abstract();
            let count = count_occupancy_dp(&abstract_cells)
                .count
                .to_u64()
                .expect("endgame fiber counts fit u64");
            let worlds = cells.fiber_worlds();
            assert_eq!(worlds.len() as u64, count, "enumeration agrees with DP");
            let distinct: BTreeSet<Vec<Vec<u8>>> = worlds
                .iter()
                .map(|w| {
                    w.hidden_hands
                        .iter()
                        .map(|h| h.iter().map(|d| d.index() as u8).collect())
                        .collect()
                })
                .collect();
            assert_eq!(distinct.len() as u64, count, "no duplicate worlds");
            for world in &worlds {
                assert!(cells.fiber_contains(world), "every world is in the fiber");
            }
            agreements += 1;
        }
    }
    assert_eq!(agreements, 432);
    agreements
}

/// `r_pos_fiber`, streamed half (CELL-25/26): for the 324 positions with
/// boundary ≤ 2 the fiber is visited by rank, never materialized — a
/// deterministic three-rank sample `{0, N/2, N−1}` must unrank to a valid
/// world of the cell system and rank back to itself. (The brief's
/// full-visitation clause is replaced by this sample: one `unrank_world`
/// costs dozens of DP counts, so exhaustively unranking 4×10⁸ worlds is not
/// minutes-scale; recorded deviation, BRIEF_PLAYER_01 §8 P2.)
pub fn fiber_streamed_check() -> u64 {
    let mut agreements = 0u64;
    for boundary in 0..3 {
        for index in 0..108 {
            let state = boundary_position(index, boundary);
            let (abstract_cells, _) = derive_rule_cells(&state).to_abstract();
            let count = count_occupancy_dp(&abstract_cells).count;
            assert!(count > BigUint::zero());
            let samples = [
                BigUint::zero(),
                &count / BigUint::from(2u32),
                &count - BigUint::one(),
            ];
            for rank in samples {
                let world = unrank_world(&abstract_cells, &rank);
                for (s, seat_tiles) in world.iter().enumerate() {
                    assert_eq!(seat_tiles.len(), abstract_cells.capacity(s));
                    for &tile in seat_tiles {
                        assert!(abstract_cells.possible(s)[tile], "tile allowed at seat");
                    }
                }
                assert_eq!(rank_world(&abstract_cells, &world), rank, "rank round-trip");
            }
            agreements += 1;
        }
    }
    assert_eq!(agreements, 324);
    agreements
}

/// `r_pos_schedule` (INV-P6; B amended `2³²` → `2²⁸` 2026-07-28, recorded
/// in BRIEF_PLAYER_01 §7 and `rob_player::window`): the §7 window formula's
/// `H` per position is what the bounds force — `H = 1` at boundary 0
/// (budget-floored; the counting engine serves it), `H ∈ {1, 2, 3}` at
/// boundary 1 (fiber-dependent; distribution frozen in the receipt), full
/// depth (`H` = tricks remaining) for boundary ≥ 2. Returns (checks, the
/// boundary-1 H histogram for H = 1..=3).
pub fn schedule_check() -> (u64, [u64; 3]) {
    let mut checks = 0u64;
    let mut t1_histogram = [0u64; 3];
    for boundary in 0..BOUNDARIES {
        for index in 0..108 {
            let state = boundary_position(index, boundary);
            let (abstract_cells, _) = derive_rule_cells(&state).to_abstract();
            let count = count_occupancy_dp(&abstract_cells)
                .count
                .to_u64()
                .expect("fiber counts fit u64");
            let hand = state.own_remaining_hand().len();
            let h = window_depth(count, hand);
            match boundary {
                0 => assert_eq!(h, 1, "boundary 0 affords exactly one trick"),
                1 => {
                    assert!(
                        (1..=3).contains(&h),
                        "boundary 1 affords one to three tricks"
                    );
                    t1_histogram[h - 1] += 1;
                }
                _ => assert_eq!(h, hand, "boundary >= 2 affords full depth"),
            }
            checks += 1;
        }
    }
    assert_eq!(checks, 756);
    (checks, t1_histogram)
}

/// Build the canonical P2 receipt (BRIEF_PLAYER_01 §8–§9). Panics on any
/// check failure.
pub fn receipt() -> String {
    let mut r = Receipt::new("player-p2");
    let (positions, depths) = corpus_check();
    r.line(
        "r_pos_corpus",
        &format!("{positions} positions; {depths} x 108; viewer to act"),
    );
    let bounds: Vec<String> = (0..BOUNDARIES)
        .map(|t| fmt_commas(boundary_bound(t).to_u128().expect("bounds fit u128")))
        .collect();
    r.line("r_pos_bounds", &bounds.join(" / "));
    let (checks, equalities, total) = count_check();
    r.line(
        "r_pos_count",
        &format!(
            "{checks} within bounds; {equalities} boundary-0 equalities at {}",
            fmt_commas(399_072_960u128)
        ),
    );
    r.line(
        "r_pos_census",
        &fmt_commas(total.to_u128().expect("census fits u128")),
    );
    let enumerated = fiber_enumerated_check();
    let streamed = fiber_streamed_check();
    r.line(
        "r_pos_fiber",
        &format!("{enumerated} enumerated agreements; {streamed} streamed sample round-trips"),
    );
    let (schedules, t1) = schedule_check();
    r.line(
        "r_pos_schedule",
        &format!(
            "{schedules} schedules; H=1 at t=0; t=1 H 1/2/3 = {}/{}/{}; full depth for t>=2; B=2^28 ({WINDOW_BUDGET})",
            t1[0], t1[1], t1[2]
        ),
    );
    r.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn r_pos_corpus() {
        assert_eq!(corpus_check(), (756, 7));
    }

    #[test]
    fn r_pos_bounds_septuple() {
        let expected: [u64; 7] = [399_072_960, 17_153_136, 756_756, 34_650, 1_680, 90, 6];
        for (t, &bound) in expected.iter().enumerate() {
            assert_eq!(boundary_bound(t), BigUint::from(bound));
        }
    }

    #[test]
    fn r_pos_fiber_enumerated() {
        assert_eq!(fiber_enumerated_check(), 432);
    }

    #[test]
    fn r_pos_schedule() {
        let (checks, t1) = schedule_check();
        assert_eq!(checks, 756);
        assert_eq!(t1.iter().sum::<u64>(), 108);
    }
}
