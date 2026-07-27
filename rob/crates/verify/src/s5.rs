//! Stage S5 verification harness: the matching-minor calculus
//! (BRIEF_SLICE_02 §9, table S5). All rows are corpus-anchored.

use std::collections::HashSet;

use num_bigint::BigUint;
use num_traits::Zero;
use rob_core::{
    algebra_for, ambiguity_rank, derive_rule_cells, game_observation, matching_minor_update,
    sample_native_world, AbstractCells, AbstractWorld, DominoId, ExactRationalChoiceSource,
    FeasibleSupportNormalForm, ObservationKind, Seat, TotalSupportNormalForm,
    TypedHiddenObservation, HIDDEN_SEATS,
};

use crate::receipt::{fmt_commas, Receipt};
use crate::s3::{mechanical_trajectory, s3_corpus_hand};

/// Enumerate the S5 dynamics corpus (S4's tiny corpus extended by `n = 0`):
/// 66,969 systems.
fn for_each_dynamics_system(mut f: impl FnMut(&AbstractCells)) -> u64 {
    let mut systems = 0u64;
    for n in 0..=4usize {
        let subsets: Vec<Vec<bool>> = (0..1u32 << n)
            .map(|mask| (0..n).map(|t| mask & (1 << t) != 0).collect())
            .collect();
        for p0 in &subsets {
            for p1 in &subsets {
                for p2 in &subsets {
                    for k0 in 0..=n {
                        for k1 in 0..=(n - k0) {
                            let k2 = n - k0 - k1;
                            let cells = AbstractCells::new(
                                n,
                                [p0.clone(), p1.clone(), p2.clone()],
                                [k0, k1, k2],
                            )
                            .expect("well-formed system");
                            f(&cells);
                            systems += 1;
                        }
                    }
                }
            }
        }
    }
    systems
}

/// `r_dyn_corpus` (TRANS-13; Math §7.14.2): corpus sizes and the
/// distinct-NF census. Returns (systems, feasible, distinct NFs in
/// deterministic first-seen order).
pub fn dynamics_corpus() -> (u64, u64, Vec<FeasibleSupportNormalForm>) {
    let mut feasible = 0u64;
    let mut seen: HashSet<FeasibleSupportNormalForm> = HashSet::new();
    let mut distinct: Vec<FeasibleSupportNormalForm> = Vec::new();
    let systems = for_each_dynamics_system(|cells| {
        if let TotalSupportNormalForm::Feasible(nf) = rob_core::compile_total_support(cells, None) {
            feasible += 1;
            if seen.insert(nf.clone()) {
                distinct.push(nf);
            }
        }
    });
    (systems, feasible, distinct)
}

/// Counters for the exhaustive observation sweep.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct ObservationSweep {
    /// Typed observations exercised.
    pub observations: u64,
    /// Nonempty successors.
    pub nonempty: u64,
    /// Per-seat surviving-tile inclusion checks (TRANS-10).
    pub edge_checks: u64,
    /// Ambiguity-rank / inactive-seat checks (TRANS-11).
    pub rank_checks: u64,
}

/// The extensional route (TRANS-08): condition the decoded fiber, push it
/// forward, rebuild cells by world projection (asserting the projected
/// system's fiber is exactly the conditioned set), and compile its NF.
fn extensional_route(
    worlds: &[AbstractWorld],
    universe: usize,
    capacities: [usize; 3],
    seat: usize,
    tile: usize,
    observation: &TypedHiddenObservation,
) -> TotalSupportNormalForm {
    let conditioned: Vec<AbstractWorld> = worlds
        .iter()
        .filter(|w| {
            w[seat].contains(&tile)
                && (observation.kind != ObservationKind::Slough
                    || w[seat].iter().all(|&e| !observation.follow_set[e]))
        })
        .map(|w| {
            core::array::from_fn(|s| {
                w[s].iter()
                    .filter(|&&t| !(s == seat && t == tile))
                    .map(|&t| if t > tile { t - 1 } else { t })
                    .collect::<Vec<usize>>()
            })
        })
        .collect();
    if conditioned.is_empty() {
        return TotalSupportNormalForm::Empty;
    }
    let mut possible: [Vec<bool>; HIDDEN_SEATS] =
        core::array::from_fn(|_| vec![false; universe - 1]);
    for world in &conditioned {
        for (s, hand) in world.iter().enumerate() {
            for &t in hand {
                possible[s][t] = true;
            }
        }
    }
    let mut caps = capacities;
    caps[seat] -= 1;
    let cells = AbstractCells::new(universe - 1, possible, caps)
        .expect("projected successor keeps the schema");
    // Pushforward exactness: the projected system's fiber is exactly the
    // conditioned world set.
    let fiber: HashSet<AbstractWorld> = cells.worlds().into_iter().collect();
    let conditioned_set: HashSet<AbstractWorld> = conditioned.iter().cloned().collect();
    assert_eq!(fiber, conditioned_set, "conditioning is a fiber (TRANS-08)");
    rob_core::compile_total_support(&cells, None)
}

/// `r_dyn_observations` + `r_dyn_monotone` (TRANS-08..13): exhaust the
/// typed observation space over the 1,331 distinct NFs; matching-minor
/// update ≡ extensional conditioning; monotonicity on every nonempty
/// successor.
pub fn observation_sweep() -> ObservationSweep {
    let (_, _, distinct) = dynamics_corpus();
    assert_eq!(distinct.len(), 1_331);
    let mut sweep = ObservationSweep::default();
    for nf in &distinct {
        let total = TotalSupportNormalForm::Feasible(nf.clone());
        let cells = nf.decode();
        let n = cells.universe();
        let worlds = cells.worlds();
        let capacities = [cells.capacity(0), cells.capacity(1), cells.capacity(2)];
        let rank_before = ambiguity_rank(nf);
        let residuals_before = nf.residuals();
        for seat in 0..HIDDEN_SEATS {
            for tile in 0..n {
                if !cells.possible(seat)[tile] {
                    continue;
                }
                let mut observations: Vec<TypedHiddenObservation> = vec![TypedHiddenObservation {
                    kind: ObservationKind::Lead,
                    follow_set: vec![false; n],
                }];
                for mask in 0..(1u32 << n) {
                    let follow_set: Vec<bool> = (0..n).map(|t| mask & (1 << t) != 0).collect();
                    let kind = if follow_set[tile] {
                        ObservationKind::Follow
                    } else {
                        ObservationKind::Slough
                    };
                    observations.push(TypedHiddenObservation { kind, follow_set });
                }
                for observation in &observations {
                    let (updated, _ledger) = matching_minor_update(&total, seat, tile, observation);
                    let extensional =
                        extensional_route(&worlds, n, capacities, seat, tile, observation);
                    assert_eq!(
                        updated, extensional,
                        "matching-minor ≡ extensional conditioning (TRANS-09)"
                    );
                    sweep.observations += 1;
                    if let TotalSupportNormalForm::Feasible(successor) = &updated {
                        sweep.nonempty += 1;
                        // TRANS-10: surviving holder sets only shrink.
                        let successor_cells = successor.decode();
                        for e in 0..n {
                            if e == tile {
                                continue;
                            }
                            let shifted = if e > tile { e - 1 } else { e };
                            for s in 0..HIDDEN_SEATS {
                                assert!(
                                    !successor_cells.possible(s)[shifted] || cells.possible(s)[e],
                                    "holder sets only shrink (TRANS-10)"
                                );
                                sweep.edge_checks += 1;
                            }
                        }
                        // TRANS-11 / INV-12: rank never increases; inactive
                        // seats never reactivate.
                        assert!(
                            ambiguity_rank(successor) <= rank_before,
                            "ambiguity rank never increases (TRANS-11)"
                        );
                        let residuals_after = successor.residuals();
                        for s in 0..HIDDEN_SEATS {
                            assert!(
                                !(residuals_before[s] == 0 && residuals_after[s] > 0),
                                "inactive seats never reactivate (TRANS-11)"
                            );
                        }
                        sweep.rank_checks += 1;
                    }
                }
            }
        }
    }
    sweep
}

/// `r_dyn_typed_wrapper` (TRANS-08; CELL-05): along the S3 parity corpus,
/// the initial 63-edge support evolved purely by game-typed observations
/// (viewer plays as identity) equals the NF recompiled from derived cells
/// after every play. Returns (transitions in the 21..28 window, hidden,
/// viewer) — the corpus-shape-forced counts; equality is asserted at every
/// one of the 28 plays.
pub fn typed_wrapper_check() -> (u64, u64, u64) {
    let viewer = Seat::ALL[0];
    let mut window = 0u64;
    let mut hidden_count = 0u64;
    let mut viewer_count = 0u64;
    for index in 0..108 {
        let hand = s3_corpus_hand(index);
        let declaration = hand.declaration;
        let trajectory = mechanical_trajectory(&hand, viewer);
        // Initial support: the unrestricted 21-tile system.
        let initial_cells = derive_rule_cells(&trajectory[0]);
        let (abstract_initial, mut tile_order) = initial_cells.to_abstract();
        let mut nf = rob_core::compile_total_support(&abstract_initial, None);
        assert!(matches!(nf, TotalSupportNormalForm::Feasible(_)));
        for (t, step) in hand.steps.iter().enumerate() {
            if step.actor == viewer {
                // Viewer plays are the identity on hidden support.
            } else {
                let led = step
                    .trick_before
                    .first()
                    .map(|p| algebra_for(declaration).led_suit(p.domino));
                let observation = game_observation(declaration, led, step.domino, &tile_order);
                let position = tile_order
                    .iter()
                    .position(|&d| d == step.domino)
                    .expect("hidden play comes from the pool");
                let seat = step.actor.index() - 1; // viewer 0: offsets 1..3
                let (next, _ledger) = matching_minor_update(&nf, seat, position, &observation);
                assert!(
                    matches!(next, TotalSupportNormalForm::Feasible(_)),
                    "legal corpus plays have nonempty successors"
                );
                tile_order.remove(position);
                nf = next;
            }
            // Recompile from the mechanical state after this play.
            let mech_cells = derive_rule_cells(&trajectory[t + 1]);
            let (abstract_mech, mech_order) = mech_cells.to_abstract();
            assert_eq!(mech_order, tile_order, "canonical pool order agrees");
            let recompiled = rob_core::compile_total_support(&abstract_mech, None);
            assert_eq!(nf, recompiled, "evolved NF equals recompiled NF (TRANS-08)");
            if t >= 20 {
                window += 1;
                if step.actor == viewer {
                    viewer_count += 1;
                } else {
                    hidden_count += 1;
                }
            }
        }
    }
    (window, hidden_count, viewer_count)
}

/// A deterministic exact-weight source (verify-local, integer arithmetic).
#[derive(Clone)]
struct DeterministicSource(u64);

impl ExactRationalChoiceSource for DeterministicSource {
    fn choose(&mut self, weights: &[BigUint]) -> usize {
        let total: BigUint = weights.iter().sum();
        assert!(total > BigUint::zero());
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1);
        let mut point = BigUint::from(self.0) % total;
        for (i, w) in weights.iter().enumerate() {
            if point < *w {
                return i;
            }
            point -= w;
        }
        unreachable!("point below total")
    }
}

/// `r_dyn_native_sampler` (CELL-10E/F): `sample_native_world` agrees with
/// the abstract sampler through the offset bijection on every S3 parity
/// state, with identical deterministic sources.
pub fn native_sampler_check() -> u64 {
    let viewer = Seat::ALL[0];
    let mut agreements = 0u64;
    for index in 0..108 {
        let hand = s3_corpus_hand(index);
        let trajectory = mechanical_trajectory(&hand, viewer);
        #[allow(clippy::needless_range_loop)] // t seeds the source and indexes states
        for t in 20..=28usize {
            let cells = derive_rule_cells(&trajectory[t]);
            let seed = 0xD1_5EED ^ (index * 64 + t as u64);
            let native = sample_native_world(&cells, &mut DeterministicSource(seed));
            let (abstract_cells, tile_order) = cells.to_abstract();
            let abstract_world =
                rob_core::sample_uniform_world(&abstract_cells, &mut DeterministicSource(seed));
            let lifted = rob_core::RemainderWorld {
                hidden_hands: core::array::from_fn(|s| {
                    rob_core::DominoSet::from_ids(abstract_world[s].iter().map(|&t| tile_order[t]))
                }),
            };
            assert_eq!(native, lifted, "native ≡ abstract through the bijection");
            assert!(
                cells.fiber_contains(&native),
                "sampled world is in the fiber"
            );
            agreements += 1;
        }
    }
    agreements
}

/// Build the canonical S5 receipt. All lines are corpus-anchored.
pub fn receipt() -> String {
    let mut r = Receipt::new("S5");
    let (systems, feasible, distinct) = dynamics_corpus();
    r.line(
        "r_dyn_corpus",
        &format!(
            "{} systems; {} feasible; {} distinct normal forms",
            fmt_commas(systems as u128),
            fmt_commas(feasible as u128),
            fmt_commas(distinct.len() as u128)
        ),
    );
    let sweep = observation_sweep();
    r.line(
        "r_dyn_observations",
        &format!(
            "{} observations; {} nonempty successors",
            fmt_commas(sweep.observations as u128),
            fmt_commas(sweep.nonempty as u128)
        ),
    );
    r.line(
        "r_dyn_monotone",
        &format!(
            "{} edge checks; {} rank checks",
            fmt_commas(sweep.edge_checks as u128),
            fmt_commas(sweep.rank_checks as u128)
        ),
    );
    let (window, hidden, viewer) = typed_wrapper_check();
    r.line(
        "r_dyn_typed_wrapper",
        &format!("{window} transitions; {hidden} hidden; {viewer} viewer"),
    );
    r.line(
        "r_dyn_native_sampler",
        &format!("{} agreements", native_sampler_check()),
    );
    r.finish()
}

/// A `DominoId` visible for doctests in this module's docs.
#[allow(dead_code)]
fn _doc_anchor(_: DominoId) {}
