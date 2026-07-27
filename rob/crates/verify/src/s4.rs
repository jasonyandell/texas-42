//! Stage S4 verification harness: support normal form + capacity DP
//! (BRIEF §8, table S4).

use std::collections::{BTreeMap, BTreeSet};

use num_bigint::BigUint;
use num_rational::BigRational;
use num_traits::{One, Zero};

use rob_core::support::census::{
    factorials, feasible_splits, relabel_matrix, relabel_signature, split_matrix, SEAT_PERMUTATIONS,
};
use rob_core::{
    all_ternary_signatures, compile_total_support, count_deletion_recurrence,
    count_generating_function, count_occupancy_dp, derive_rule_cells, floor_family_count,
    marginal_allowed, marginal_by_projection, marginal_by_scc, rank_world,
    reachable_capacity_profiles, reduce, sample_uniform_world, support_census,
    ternary_signature_valid, unrank_world, world_probability, AbstractCells, AbstractWorld,
    Ambiguity, ExactRationalChoiceSource, FeasibleSupportNormalForm, TernarySignature,
    TotalSupportNormalForm, HIDDEN_SEATS,
};

use crate::receipt::{fmt_commas, Receipt};

/// Enumerate the tiny corpus (BRIEF §8 S4 preamble): for each universe size
/// `n ∈ {1,2,3,4}`, all `(2^n)^3` allowed-set triples × all capacity triples
/// summing to `n` — 66,968 systems in total.
pub fn for_each_tiny_system(mut f: impl FnMut(&AbstractCells)) -> u64 {
    let mut systems = 0u64;
    for n in 1..=4usize {
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
                            .expect("well-formed tiny system");
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

fn world_set(worlds: &[AbstractWorld]) -> BTreeSet<AbstractWorld> {
    worlds.iter().cloned().collect()
}

/// `r_nf_hall` (CELL-09/10): the Hall biconditional against direct
/// enumeration on every tiny system. Returns (systems, feasible, worlds).
pub fn hall_check() -> (u64, u64, u64) {
    let mut feasible = 0u64;
    let mut worlds_total = 0u64;
    let systems = for_each_tiny_system(|cells| {
        let worlds = cells.worlds();
        assert_eq!(
            cells.is_feasible(),
            !worlds.is_empty(),
            "Hall ⇔ nonempty direct enumeration"
        );
        if !worlds.is_empty() {
            feasible += 1;
            worlds_total += worlds.len() as u64;
        }
    });
    (systems, feasible, worlds_total)
}

/// `r_nf_count_routes` (CELL-10A/B/H): generating function, deletion
/// recurrence, and occupancy DP all agree with direct enumeration.
pub fn count_routes_check() -> u64 {
    for_each_tiny_system(|cells| {
        let direct = BigUint::from(cells.worlds().len());
        assert_eq!(count_generating_function(cells), direct, "GF coefficient");
        assert_eq!(
            count_deletion_recurrence(cells),
            direct,
            "deletion recurrence"
        );
        assert_eq!(count_occupancy_dp(cells).count, direct, "occupancy DP");
    })
}

/// Summary of the native capacity-DP exhaustion (CELL-10I/I1).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CapacityDpSummary {
    /// Capacity triples exhausted (8³ = 512).
    pub profiles: u64,
    /// Maximum occupancy states over one whole run.
    pub max_states: u64,
    /// Maximum candidate-holder checks.
    pub max_checks: u64,
    /// Maximum capacity-eligible updates.
    pub max_updates: u64,
    /// Maximum live states in any one layer.
    pub max_layer: u64,
    /// Maximum count.
    pub max_count: BigUint,
}

/// `r_nf_capacity_dp` (CELL-10I/I1): the instrumented native occupancy DP
/// over all 512 unrestricted capacity triples — exact multinomial counts and
/// exact attainment of the operation-bound formulas.
pub fn capacity_dp_check() -> CapacityDpSummary {
    let fact = factorials();
    let mut summary = CapacityDpSummary {
        profiles: 0,
        max_states: 0,
        max_checks: 0,
        max_updates: 0,
        max_layer: 0,
        max_count: BigUint::zero(),
    };
    for k0 in 0..=7usize {
        for k1 in 0..=7usize {
            for k2 in 0..=7usize {
                let n = k0 + k1 + k2;
                let cells =
                    AbstractCells::new(n, core::array::from_fn(|_| vec![true; n]), [k0, k1, k2])
                        .expect("unrestricted native system");
                let stats = count_occupancy_dp(&cells);
                // Exact multinomial count.
                let multinomial = &fact[n] / (&fact[k0] * &fact[k1] * &fact[k2]);
                assert_eq!(stats.count, multinomial, "multinomial count");
                // Exact attainment of the proved operation formulas.
                let product = ((k0 + 1) * (k1 + 1) * (k2 + 1)) as u64;
                assert_eq!(stats.states_visited, product, "total occupancy states");
                assert_eq!(
                    stats.candidate_checks,
                    3 * (product - 1),
                    "candidate checks"
                );
                let eligible = (k0 * (k1 + 1) * (k2 + 1)
                    + k1 * (k0 + 1) * (k2 + 1)
                    + k2 * (k0 + 1) * (k1 + 1)) as u64;
                assert_eq!(
                    stats.capacity_eligible_updates, eligible,
                    "capacity-eligible updates"
                );
                // Layer sizes are the coefficients of ∏ (1+…+x^{k_s}).
                let mut poly = vec![BigUint::one()];
                for k in [k0, k1, k2] {
                    let mut next = vec![BigUint::zero(); poly.len() + k];
                    for (i, c) in poly.iter().enumerate() {
                        for j in 0..=k {
                            next[i + j] += c;
                        }
                    }
                    poly = next;
                }
                let expected_layers: Vec<u64> = poly
                    .iter()
                    .map(|c| u64::try_from(c).expect("small coefficient"))
                    .collect();
                assert_eq!(
                    stats.layer_sizes, expected_layers,
                    "layer coefficient profile"
                );
                assert!(stats.max_live_layer <= 48, "≤ 48 live states per layer");
                if (k0, k1, k2) == (7, 7, 7) {
                    assert_eq!(
                        &stats.layer_sizes[..11],
                        &[1, 3, 6, 10, 15, 21, 28, 36, 42, 46, 48],
                        "native (7,7,7) layer prefix"
                    );
                    assert_eq!(stats.layer_sizes[11], 48);
                }
                summary.profiles += 1;
                summary.max_states = summary.max_states.max(stats.states_visited);
                summary.max_checks = summary.max_checks.max(stats.candidate_checks);
                summary.max_updates = summary.max_updates.max(stats.capacity_eligible_updates);
                summary.max_layer = summary.max_layer.max(stats.max_live_layer);
                summary.max_count = summary.max_count.clone().max(stats.count);
            }
        }
    }
    summary
}

/// `r_nf_marginal` (CELL-10J/K): the marginal-edge criterion (forced
/// successor Hall) against direct world projection for every (seat, tile)
/// pair of every tiny system, plus the local-vs-marginal negative witness.
pub fn marginal_check() -> u64 {
    let mut edges = 0u64;
    for_each_tiny_system(|cells| {
        let projection = marginal_by_projection(cells);
        #[allow(clippy::needless_range_loop)] // (seat, tile) pairs drive both routes
        for s in 0..HIDDEN_SEATS {
            for tile in 0..cells.universe() {
                assert_eq!(
                    marginal_allowed(cells, s, tile),
                    projection[s][tile],
                    "marginal criterion ⇔ world projection"
                );
                edges += 1;
            }
        }
    });
    // CELL-10J: local allowance is not marginal possibility.
    let witness = AbstractCells::new(
        2,
        [vec![true, true], vec![true, false], vec![false, false]],
        [1, 1, 0],
    )
    .expect("witness system");
    assert!(witness.possible(0)[0], "a is locally allowed at seat 0");
    assert!(
        !marginal_allowed(&witness, 0, 0),
        "yet seat 0 holds a in no conserved world"
    );
    edges
}

/// `r_nf_reduction` (CELL-10L/L1/N): canonical reduction — fiber-preserving,
/// contractive, idempotent, and a normal form within each fixed schema —
/// plus the reduction-instability witness.
pub fn reduction_check() -> u64 {
    // Group systems by (universe, capacities); within each schema the map
    // fiber ↔ reduction must be a bijection.
    type Schema = (usize, [usize; 3]);
    type ReducedRepr = Vec<Vec<bool>>;
    let mut groups: BTreeMap<Schema, BTreeMap<Vec<AbstractWorld>, BTreeSet<ReducedRepr>>> =
        BTreeMap::new();
    let systems = for_each_tiny_system(|cells| {
        let reduced = reduce(cells);
        // Fiber preservation.
        assert_eq!(
            world_set(&reduced.worlds()),
            world_set(&cells.worlds()),
            "reduction preserves the fiber"
        );
        // Contractive.
        for s in 0..HIDDEN_SEATS {
            for tile in 0..cells.universe() {
                assert!(
                    !reduced.possible(s)[tile] || cells.possible(s)[tile],
                    "reduction is contractive"
                );
            }
        }
        // Idempotent.
        assert_eq!(reduce(&reduced), reduced, "reduction is idempotent");
        // Normal form within the schema group.
        let key = (
            cells.universe(),
            [cells.capacity(0), cells.capacity(1), cells.capacity(2)],
        );
        let fiber: Vec<AbstractWorld> = world_set(&cells.worlds()).into_iter().collect();
        let reduced_repr: Vec<Vec<bool>> = (0..HIDDEN_SEATS)
            .map(|s| reduced.possible(s).to_vec())
            .collect();
        groups
            .entry(key)
            .or_default()
            .entry(fiber)
            .or_default()
            .insert(reduced_repr);
    });
    let mut seen_reductions: BTreeMap<Schema, BTreeSet<ReducedRepr>> = BTreeMap::new();
    for (key, by_fiber) in &groups {
        for reductions in by_fiber.values() {
            assert_eq!(reductions.len(), 1, "equal fibers ⇒ equal reductions");
            let repr = reductions.iter().next().expect("nonempty").clone();
            assert!(
                seen_reductions.entry(*key).or_default().insert(repr),
                "distinct fibers ⇒ distinct reductions"
            );
        }
    }
    // CELL-10N: a reduced predecessor with an unreduced raw successor.
    let unstable = AbstractCells::new(
        3,
        [
            vec![false, false, false],
            vec![true, true, false],
            vec![true, true, true],
        ],
        [0, 1, 2],
    )
    .expect("witness system");
    assert_eq!(reduce(&unstable), unstable, "the predecessor is reduced");
    let successor = unstable
        .removal_update(2, 0)
        .expect("seat 2 may play tile a");
    assert_ne!(
        reduce(&successor),
        successor,
        "the raw successor has a newly unsupported edge"
    );
    systems
}

/// A deterministic exact-weight choice source for driving the sampler in
/// tests: walks a fixed integer stream, reducing modulo the total weight.
/// (The exactness *claims* are carried by the telescoping-product check,
/// which uses no randomness at all.)
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
        unreachable!("point lies below the total weight");
    }
}

/// `r_nf_sampler` (CELL-10E/F/G): every world probability on the feasible
/// tiny corpus equals the telescoping integer-ratio product, as exact
/// rationals. Returns the number of world probabilities checked.
pub fn sampler_check() -> u64 {
    let mut checked = 0u64;
    let mut source = DeterministicSource(0x5eed);
    for_each_tiny_system(|cells| {
        let worlds = cells.worlds();
        if worlds.is_empty() {
            return;
        }
        let count = BigUint::from(worlds.len());
        let uniform = BigRational::new(BigUint::one().into(), count.clone().into());
        let mut mass = BigRational::zero();
        for world in &worlds {
            let p = world_probability(cells, world);
            assert_eq!(p, uniform, "telescoping product = 1/N exactly");
            mass += p;
            checked += 1;
        }
        assert_eq!(mass, BigRational::one(), "probabilities sum to one");
        // The sampler itself lands inside the fiber.
        let sampled = sample_uniform_world(cells, &mut source);
        assert!(
            world_set(&worlds).contains(&sampled),
            "sampled world lies in the fiber"
        );
    });
    checked
}

/// Summary of the quotient/normal-form exhaustion (CELL-12..20, 25/26).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct QuotientSummary {
    /// One SCC compilation per feasible witness world.
    pub scc_compilations: u64,
    /// Stored ternary exclusions proved essential.
    pub essential_exclusions: u64,
    /// Exact rank/unrank round trips.
    pub rank_unrank: u64,
}

/// `r_nf_quotient` (CELL-12..20, 25/26): trichotomy, NF ↔ fiber bijection,
/// SCC compilation against per-edge Hall for every feasible witness world,
/// essential exclusions, the linear ternary validator, and rank/unrank.
pub fn quotient_check() -> QuotientSummary {
    let mut summary = QuotientSummary {
        scc_compilations: 0,
        essential_exclusions: 0,
        rank_unrank: 0,
    };
    for_each_tiny_system(|cells| {
        let worlds = cells.worlds();
        if worlds.is_empty() {
            assert_eq!(
                compile_total_support(cells, None),
                TotalSupportNormalForm::Empty,
                "infeasible systems compile to the single Empty state"
            );
            return;
        }
        let projection = marginal_by_projection(cells);
        let hall_route: Vec<Vec<bool>> = (0..HIDDEN_SEATS)
            .map(|s| {
                (0..cells.universe())
                    .map(|t| marginal_allowed(cells, s, t))
                    .collect()
            })
            .collect();
        let mut reference: Option<FeasibleSupportNormalForm> = None;
        for world in &worlds {
            // One assignment + one SCC pass ≡ per-edge Hall ≡ projection.
            let scc = marginal_by_scc(cells, world);
            for s in 0..HIDDEN_SEATS {
                assert_eq!(scc[s], projection[s], "SCC ≡ world projection");
                assert_eq!(scc[s], hall_route[s], "SCC ≡ per-edge Hall");
            }
            let nf = match compile_total_support(cells, Some(world)) {
                TotalSupportNormalForm::Feasible(nf) => nf,
                TotalSupportNormalForm::Empty => panic!("feasible system"),
            };
            match &reference {
                Some(r) => assert_eq!(*r, nf, "witness independence"),
                None => reference = Some(nf),
            }
            summary.scc_compilations += 1;
        }
        let nf = reference.expect("at least one world");
        // Native active-seat trichotomy (CELL-12): 0, 2, or 3 — never 1.
        let active = nf.residuals().iter().filter(|&&r| r > 0).count();
        assert!(matches!(active, 0 | 2 | 3), "trichotomy 0/2/3");
        assert!(nf.well_formed(), "compiled form is well-formed");
        // NF ↔ fiber bijection (CELL-13/14): decode reproduces the fiber and
        // recompiling the decode reproduces the normal form.
        let decoded = nf.decode();
        assert_eq!(
            world_set(&decoded.worlds()),
            world_set(&worlds),
            "decode(NF) has the original fiber"
        );
        assert_eq!(
            compile_total_support(&decoded, None),
            TotalSupportNormalForm::Feasible(nf.clone()),
            "compile ∘ decode = identity"
        );
        // Every stored ternary exclusion is essential (CELL-19).
        if let Ambiguity::Ternary { excluded_seat, .. } = &nf.ambiguity {
            for (index, &(tile, seat)) in excluded_seat.iter().enumerate() {
                let mut weakened = nf.clone();
                if let Ambiguity::Ternary {
                    excluded_seat: ref mut exclusions,
                    ..
                } = weakened.ambiguity
                {
                    exclusions.remove(index);
                }
                let enlarged = weakened.decode().worlds();
                assert!(
                    enlarged.len() > worlds.len(),
                    "removing one exclusion strictly enlarges the fiber"
                );
                assert!(
                    enlarged.iter().any(|w| w[seat].contains(&tile)),
                    "a newly admitted world assigns the tile to the excluded seat"
                );
                summary.essential_exclusions += 1;
            }
        }
        // Exact fiber-local rank/unrank round trips (CELL-26).
        let count = BigUint::from(worlds.len());
        let mut seen = BTreeSet::new();
        for world in &worlds {
            let rank = rank_world(cells, world);
            assert!(rank < count, "rank lies below the fiber count");
            assert_eq!(unrank_world(cells, &rank), *world, "rank/unrank round trip");
            assert!(seen.insert(rank), "ranks are injective");
            summary.rank_unrank += 1;
        }
    });
    validator_vs_matching();
    summary
}

/// The linear ternary validator against a brute-force matching search
/// (CELL-20) on a synthetic signature domain: the three comparisons decide
/// exactly nonempty-reduced-component validity.
fn validator_vs_matching() {
    for r0 in 1..=3usize {
        for r1 in 1..=3usize {
            for r2 in 1..=3usize {
                let n = r0 + r1 + r2;
                for n0 in 0..=3usize {
                    for n1 in 0..=3usize {
                        for n2 in 0..=3usize {
                            if n0 + n1 + n2 > n {
                                continue;
                            }
                            // Build the generic payload: n_s tiles excluding
                            // seat s, the rest unrestricted.
                            let mut possible: [Vec<bool>; 3] =
                                core::array::from_fn(|_| vec![true; n]);
                            let mut tile = 0usize;
                            for (s, &count) in [n0, n1, n2].iter().enumerate() {
                                for _ in 0..count {
                                    possible[s][tile] = false;
                                    tile += 1;
                                }
                            }
                            let cells = AbstractCells::new(n, possible, [r0, r1, r2])
                                .expect("structural payload");
                            let matching_route = cells.is_feasible() && reduce(&cells) == cells;
                            assert_eq!(
                                ternary_signature_valid([r0, r1, r2], [n0, n1, n2]),
                                matching_route,
                                "linear validator ≡ matching search at \
                                 r=({r0},{r1},{r2}), n=({n0},{n1},{n2})"
                            );
                        }
                    }
                }
            }
        }
    }
}

/// Summary of the native ternary signature census and S₃ quotient
/// (CELL-21/22/23).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TernaryCensusSummary {
    /// Seat-labeled six-integer signatures.
    pub signatures: u64,
    /// Feasible allocation matrices across all labeled signatures.
    pub matrices: u64,
    /// Maximum matrices for one signature.
    pub max_matrices: u64,
    /// S₃ signature orbits.
    pub orbits: u64,
    /// Feasible matrices across canonical representatives.
    pub representative_matrices: u64,
    /// Stabilizer matrix orbits across canonical representatives.
    pub stabilizer_orbits: u64,
    /// Maximum stabilizer orbits for one signature.
    pub max_stabilizer_orbits: u64,
}

/// `r_nf_ternary_census`: enumerate the native ternary signature census and
/// its S₃ quotient from the implemented validity predicate.
pub fn ternary_census_check() -> TernaryCensusSummary {
    let signatures = all_ternary_signatures();
    let mut matrices = 0u64;
    let mut max_matrices = 0u64;
    for signature in &signatures {
        let splits = feasible_splits(signature);
        matrices += splits.len() as u64;
        max_matrices = max_matrices.max(splits.len() as u64);
    }

    let mut orbit_keys = BTreeSet::new();
    for signature in &signatures {
        orbit_keys.insert(signature.orbit_key());
    }
    let mut representative_matrices = 0u64;
    let mut stabilizer_orbits = 0u64;
    let mut max_stabilizer_orbits = 0u64;
    for key in &orbit_keys {
        let representative = TernarySignature {
            r: [key[0].0, key[1].0, key[2].0],
            n_excluded: [key[0].1, key[1].1, key[2].1],
        };
        assert!(
            ternary_signature_valid(representative.r, representative.n_excluded),
            "validity is S₃-invariant"
        );
        let splits = feasible_splits(&representative);
        representative_matrices += splits.len() as u64;
        let stabilizer: Vec<[usize; 3]> = SEAT_PERMUTATIONS
            .iter()
            .copied()
            .filter(|&p| relabel_signature(&representative, p) == representative)
            .collect();
        let all_matrices: BTreeSet<[[usize; 3]; 4]> = splits
            .iter()
            .map(|&split| split_matrix(&representative, split))
            .collect();
        let mut seen: BTreeSet<[[usize; 3]; 4]> = BTreeSet::new();
        let mut orbits_here = 0u64;
        for matrix in &all_matrices {
            if seen.contains(matrix) {
                continue;
            }
            orbits_here += 1;
            let mut orbit_size = 0u64;
            for &p in &stabilizer {
                let image = relabel_matrix(matrix, p);
                assert!(
                    all_matrices.contains(&image),
                    "the stabilizer preserves the feasible matrix set"
                );
                if seen.insert(image) {
                    orbit_size += 1;
                }
            }
            assert!(
                matches!(orbit_size, 1 | 2 | 3 | 6),
                "orbit sizes lie in {{1,2,3,6}}"
            );
        }
        assert_eq!(
            seen.len(),
            all_matrices.len(),
            "orbits partition the matrices"
        );
        stabilizer_orbits += orbits_here;
        max_stabilizer_orbits = max_stabilizer_orbits.max(orbits_here);
    }
    TernaryCensusSummary {
        signatures: signatures.len() as u64,
        matrices,
        max_matrices,
        orbits: orbit_keys.len() as u64,
        representative_matrices,
        stabilizer_orbits,
        max_stabilizer_orbits,
    }
}

/// `r_nf_census_81` (CELL-27): the full-schema census from the Math §7.12.5
/// formulas — computed, never hard-coded — and the 81-bit fixed width.
pub fn census81_check() -> rob_core::SupportCensus {
    let census = support_census();
    assert_eq!(census.empty.to_string(), "1");
    assert_eq!(census.determinate.to_string(), "8102258940222814");
    assert_eq!(census.binary.to_string(), "11495078055913018482");
    assert_eq!(census.ternary.to_string(), "1830955704129296418354864");
    let total = census.total();
    assert_eq!(total.to_string(), "1830967207309611271596161");
    let two_80 = BigUint::one() << 80;
    let two_81 = BigUint::one() << 81;
    assert!(two_80 < total && total < two_81, "2^80 < total < 2^81");
    assert_eq!(census.fixed_width_bits(), 81, "fixed width 81 bits");
    census
}

use rob_core::{
    all_ids, apply_auction_action, apply_play, begin_contracted_play, begin_deal_attempt,
    close_auction, initial_contracted_mechanical, legal_plays, native_compile_exact_support,
    update_support, AuctionAction, BidValue, CloseAuctionOutcome, DealWorld, Declaration, DominoId,
    DominoSet, MatchState, MechanicalCompiledView, MechanicalState, Play, PlayPhase, PointAmount,
    RulesConfig, Seat, PIPS,
};

use crate::s3::{mechanical_trajectory, s3_corpus_hand};

/// Build a deal that gives `owner` exactly `tiles` (plus canonical filler)
/// and distributes the remaining dominoes canonically to the other seats.
fn deal_where(owner_tiles: &[(Seat, Vec<DominoId>)]) -> DealWorld {
    let mut hands: [DominoSet; 4] = core::array::from_fn(|_| DominoSet::empty());
    let mut used = DominoSet::empty();
    for (seat, tiles) in owner_tiles {
        for &tile in tiles {
            assert!(!used.contains(tile), "constrained tiles are distinct");
            hands[seat.index()].insert(tile);
            used.insert(tile);
        }
    }
    let mut free: Vec<DominoId> = all_ids().filter(|&d| !used.contains(d)).collect();
    for seat in Seat::ALL {
        while hands[seat.index()].len() < 7 {
            hands[seat.index()].insert(free.remove(0));
        }
    }
    DealWorld::new(hands).expect("a valid constrained deal")
}

/// Drive one contracted hand through the certified S2 lifecycle with an
/// explicit play chooser, mirroring every observed play into the viewer-0…
/// no — into a caller-selected viewer's mechanical state. Plays stop after
/// `max_plays`. Returns the mechanical endpoint.
fn drive_prefix(
    deal: DealWorld,
    bidder: Seat,
    declaration: Declaration,
    viewer: Seat,
    max_plays: usize,
    mut choose: impl FnMut(usize, &[DominoId], Seat) -> DominoId,
) -> MechanicalState {
    let config = RulesConfig::new(7, 7).expect("valid config");
    let shaker = bidder.offset(3); // bidder acts first
    let (m0, _) = MatchState::start(config, shaker);
    let (mut attempt, m1, _, _) = begin_deal_attempt(&m0, deal, 0).expect("begin attempt");
    let p30 = AuctionAction::Bid(BidValue::Point(PointAmount::new(30).expect("30")));
    for (k, action) in [
        p30,
        AuctionAction::Pass,
        AuctionAction::Pass,
        AuctionAction::Pass,
    ]
    .into_iter()
    .enumerate()
    {
        let _ = k;
        let (next, _) = apply_auction_action(&attempt, action, config).expect("legal action");
        attempt = next;
    }
    let pending = match close_auction(attempt, &m1, config).expect("closes") {
        CloseAuctionOutcome::Pending(p) => p,
        CloseAuctionOutcome::AllPass(_) => panic!("bidder bid"),
    };
    let (mut objective, _m2, _) =
        begin_contracted_play(pending, declaration, &m1, config).expect("begin play");
    assert_eq!(objective.state().contract().bidder(), bidder);
    let mut mechanical =
        initial_contracted_mechanical(viewer, *deal.hand(viewer), *objective.state().contract())
            .expect("viewer hand");
    for play_index in 0..max_plays {
        let legal: Vec<DominoId> = legal_plays(&objective).iter().collect();
        let actor = objective.state().current_actor().expect("play phase");
        let domino = choose(play_index, &legal, actor);
        assert!(legal.contains(&domino), "chosen play is legal");
        let (next, _, _) = apply_play(&objective, domino).expect("legal play applies");
        objective = next;
        mechanical = update_support(&mechanical, Play { actor, domino }).expect("support update");
    }
    mechanical
}

/// Realize one reachable capacity profile as a legal prefix (Math §7.13.1
/// witness construction): no-trump, the current leader holds all seven
/// doubles, wins the preceding tricks, then begins the row's current-trick
/// prefix.
fn realize_capacity_profile(profile: [usize; 3]) {
    let viewer = Seat::ALL[0];
    let h = *profile.iter().max().expect("three seats");
    let j = 7 - h;
    // Hidden seats with capacity h-1 (the low-capacity set B).
    let low: Vec<usize> = if h == 0 {
        Vec::new()
    } else {
        (0..3).filter(|&i| profile[i] == h - 1).collect()
    };
    // Current-trick prefix actors from the §7.13.1 table (hidden seat i is
    // absolute seat i+1; m is the viewer, seat 0).
    let prefix_actors: Vec<usize> = match low.as_slice() {
        [] => vec![],
        [0] => vec![0, 1],
        [1] => vec![2],
        [2] => vec![3],
        [0, 1] => vec![0, 1, 2],
        [0, 2] => vec![3, 0, 1],
        [1, 2] => vec![2, 3],
        _ => unreachable!("nonconstant profiles have |B| in {{1,2}}"),
    };
    let leader = Seat::ALL[*prefix_actors.first().unwrap_or(&1)];
    let doubles: Vec<DominoId> = all_ids()
        .filter(|&d| rob_core::domino_from_id(d).is_double())
        .collect();
    let deal = deal_where(&[(leader, doubles)]);
    let total_plays = 4 * j + prefix_actors.len();
    let mechanical = drive_prefix(
        deal,
        leader,
        Declaration::NoTrump,
        viewer,
        total_plays,
        |play_index, legal, actor| {
            // Verify the actor schedule inside the current prefix.
            if play_index >= 4 * j {
                assert_eq!(
                    actor,
                    Seat::ALL[prefix_actors[play_index - 4 * j]],
                    "prefix actor schedule"
                );
            }
            legal[0]
        },
    );
    let cells = derive_rule_cells(&mechanical);
    for i in 0..3 {
        assert_eq!(
            cells.capacity(i),
            profile[i],
            "realized hidden-capacity profile {profile:?}"
        );
    }
}

/// `r_nf_capacity_profiles` (REACH-04): reachable hidden-capacity triples
/// are exactly those with `max − min ≤ 1` — 8 + 7·6 = 50 labeled profiles;
/// necessity along the S3 corpus, sufficiency by explicit legal witnesses.
pub fn capacity_profiles_check() -> u64 {
    let profiles = reachable_capacity_profiles();
    assert_eq!(profiles.len(), 50);
    assert_eq!(profiles.len(), 8 + 7 * 6);
    // Necessity: every corpus trajectory state has a profile in the set
    // (capacities derive from trick progress, never three free fields).
    for index in [0u64, 21, 42, 63, 84, 105] {
        let hand = s3_corpus_hand(index);
        for state in mechanical_trajectory(&hand, Seat::ALL[0]) {
            let cells = derive_rule_cells(&state);
            let k = [cells.capacity(0), cells.capacity(1), cells.capacity(2)];
            assert!(profiles.contains(&k), "corpus profile {k:?} is reachable");
        }
    }
    // Sufficiency: realize every one of the 50 profiles legally.
    for &profile in &profiles {
        realize_capacity_profile(profile);
    }
    profiles.len() as u64
}

fn dom(h: u8, l: u8) -> DominoId {
    rob_core::domino_id(rob_core::Domino::new(
        rob_core::Pip::new(h).expect("pip"),
        rob_core::Pip::new(l).expect("pip"),
    ))
}

/// `r_nf_floor` (REACH-12): the universally reachable no-void floor —
/// exact family count, disjointness, and one explicit legal reaching prefix
/// per family through the S2 machine.
pub fn floor_check() -> BigUint {
    let count = floor_family_count();
    assert_eq!(count.to_string(), "44352165");
    assert!(count > (BigUint::one() << 25), "count exceeds 2^25");
    assert_eq!(count.bits(), 26, "at least 26 bits are necessary");

    let viewer = Seat::ALL[0];
    let mut sample_profiles: Vec<[usize; 3]> = Vec::new();

    // Family (7,7,7): any post-declaration initial state.
    {
        let bidder = Seat::ALL[1];
        let deal = deal_where(&[]);
        let mechanical = drive_prefix(deal, bidder, Declaration::NoTrump, viewer, 0, |_, l, _| {
            l[0]
        });
        let cells = derive_rule_cells(&mechanical);
        assert_eq!(cells.unseen_pool().len(), 21);
        let profile = [cells.capacity(0), cells.capacity(1), cells.capacity(2)];
        assert_eq!(profile, [7, 7, 7]);
        for s in 0..3 {
            assert_eq!(cells.possible(s), cells.unseen_pool(), "no voids");
        }
        sample_profiles.push(profile);
    }
    // Family (6,7,7): one hidden lead.
    {
        let bidder = Seat::ALL[1];
        let deal = deal_where(&[]);
        let mechanical = drive_prefix(deal, bidder, Declaration::NoTrump, viewer, 1, |_, l, _| {
            l[0]
        });
        let cells = derive_rule_cells(&mechanical);
        assert_eq!(cells.unseen_pool().len(), 20);
        let profile = [cells.capacity(0), cells.capacity(1), cells.capacity(2)];
        assert_eq!(profile, [6, 7, 7]);
        for s in 0..3 {
            assert_eq!(cells.possible(s), cells.unseen_pool(), "no voids");
        }
        sample_profiles.push(profile);
    }
    // Family (6,6,7): two hidden plays — a doubles-trump lead followed in
    // suit (the complement pair of doubles).
    {
        let bidder = Seat::ALL[1];
        let deal = deal_where(&[
            (Seat::ALL[1], vec![dom(6, 6)]),
            (Seat::ALL[2], vec![dom(5, 5)]),
        ]);
        let mechanical = drive_prefix(
            deal,
            bidder,
            Declaration::DoublesTrump,
            viewer,
            2,
            |i, legal, _| {
                if i == 0 {
                    dom(6, 6)
                } else {
                    assert!(legal.contains(&dom(5, 5)), "the follower holds 5-5");
                    dom(5, 5)
                }
            },
        );
        let cells = derive_rule_cells(&mechanical);
        assert_eq!(cells.unseen_pool().len(), 19);
        let profile = [cells.capacity(0), cells.capacity(1), cells.capacity(2)];
        assert_eq!(profile, [6, 6, 7]);
        for s in 0..3 {
            assert_eq!(cells.possible(s), cells.unseen_pool(), "no voids");
        }
        sample_profiles.push(profile);
    }
    // Family (6,6,6): three hidden plays sharing pip six (the pigeonhole
    // witness: a 10-tile complement with ≤ 2 doubles has a pip on ≥ 3
    // tiles).
    {
        let bidder = Seat::ALL[1];
        let deal = deal_where(&[
            (Seat::ALL[1], vec![dom(6, 6)]),
            (Seat::ALL[2], vec![dom(6, 5)]),
            (Seat::ALL[3], vec![dom(6, 4)]),
        ]);
        let mechanical = drive_prefix(
            deal,
            bidder,
            Declaration::PipTrump(PIPS[6]),
            viewer,
            3,
            |i, legal, _| {
                let choice = [dom(6, 6), dom(6, 5), dom(6, 4)][i];
                assert!(legal.contains(&choice), "trump follow is legal");
                choice
            },
        );
        let cells = derive_rule_cells(&mechanical);
        assert_eq!(cells.unseen_pool().len(), 18);
        let profile = [cells.capacity(0), cells.capacity(1), cells.capacity(2)];
        assert_eq!(profile, [6, 6, 6]);
        for s in 0..3 {
            assert_eq!(cells.possible(s), cells.unseen_pool(), "no voids");
        }
        sample_profiles.push(profile);
    }
    // The four families are pairwise disjoint: distinct capacity shapes.
    for i in 0..sample_profiles.len() {
        for j in (i + 1)..sample_profiles.len() {
            assert_ne!(
                sample_profiles[i], sample_profiles[j],
                "families are disjoint"
            );
        }
    }
    count
}

/// `r_nf_zero_supplemental` (CELL-17): along the S3 parity corpus, cells,
/// reduction, and normal form recomputed from the mechanical state after
/// every transition equal any cached view — support adds zero supplemental
/// bits relative to certified mechanical state.
pub fn zero_supplemental_check() -> u64 {
    for index in 0..108u64 {
        let hand = s3_corpus_hand(index);
        for state in mechanical_trajectory(&hand, Seat::ALL[0]) {
            let cells = derive_rule_cells(&state);
            // Deterministic re-derivation from scratch.
            assert_eq!(cells, derive_rule_cells(&state), "cells re-derive equal");
            let (abstract_cells, _) = cells.to_abstract();
            let reduced = reduce(&abstract_cells);
            assert_eq!(
                reduced,
                reduce(&abstract_cells),
                "reduction re-derives equal"
            );
            if state.phase() == PlayPhase::Play || !cells.unseen_pool().is_empty() {
                let nf = native_compile_exact_support(&cells);
                assert_eq!(
                    nf,
                    native_compile_exact_support(&cells),
                    "normal form re-derives equal"
                );
            }
            // The cached view stays coherent and outside equality (INV-1).
            let view = MechanicalCompiledView {
                state: state.clone(),
                cells_cache: Some(cells),
            };
            assert!(view.coherent(), "cache equals fresh derivation");
        }
    }
    0
}

/// Build the canonical S4 receipt (BRIEF §9). Panics on any check failure.
pub fn receipt() -> String {
    let mut r = Receipt::new("S4");
    let (systems, feasible, worlds) = hall_check();
    r.line(
        "r_nf_hall",
        &format!(
            "{} systems; {} feasible; {} worlds",
            fmt_commas(systems as u128),
            fmt_commas(feasible as u128),
            fmt_commas(worlds as u128)
        ),
    );
    r.line(
        "r_nf_count_routes",
        &fmt_commas(count_routes_check() as u128),
    );
    let dp = capacity_dp_check();
    r.line(
        "r_nf_capacity_dp",
        &format!(
            "{} profiles; {} occupancy states; {} candidate-holder checks; {} capacity-eligible updates; {} live states/layer; max count {}",
            dp.profiles,
            dp.max_states,
            fmt_commas(dp.max_checks as u128),
            fmt_commas(dp.max_updates as u128),
            dp.max_layer,
            fmt_commas(dp.max_count.to_string().parse::<u128>().expect("fits"))
        ),
    );
    r.line("r_nf_marginal", &fmt_commas(marginal_check() as u128));
    r.line("r_nf_reduction", &fmt_commas(reduction_check() as u128));
    r.line("r_nf_sampler", &fmt_commas(sampler_check() as u128));
    let q = quotient_check();
    r.line(
        "r_nf_quotient",
        &format!(
            "{} SCC compilations; {} essential exclusions; {} rank/unrank",
            fmt_commas(q.scc_compilations as u128),
            fmt_commas(q.essential_exclusions as u128),
            fmt_commas(q.rank_unrank as u128)
        ),
    );
    let t = ternary_census_check();
    r.line(
        "r_nf_ternary_census",
        &format!(
            "{} signatures; {} matrices; max {}/signature; {} orbits; {} representative matrices; {} stabilizer orbits; max {}",
            fmt_commas(t.signatures as u128),
            fmt_commas(t.matrices as u128),
            t.max_matrices,
            fmt_commas(t.orbits as u128),
            fmt_commas(t.representative_matrices as u128),
            fmt_commas(t.stabilizer_orbits as u128),
            t.max_stabilizer_orbits
        ),
    );
    let census = census81_check();
    r.line(
        "r_nf_census_81",
        &format!(
            "empty=1; determinate={}; binary={}; ternary={}; total={}; fixed width {} bits",
            fmt_commas(
                census
                    .determinate
                    .to_string()
                    .parse::<u128>()
                    .expect("fits")
            ),
            fmt_commas(census.binary.to_string().parse::<u128>().expect("fits")),
            fmt_commas(census.ternary.to_string().parse::<u128>().expect("fits")),
            fmt_commas(census.total().to_string().parse::<u128>().expect("fits")),
            census.fixed_width_bits()
        ),
    );
    r.line(
        "r_nf_capacity_profiles",
        &capacity_profiles_check().to_string(),
    );
    let floor = floor_check();
    r.line(
        "r_nf_floor",
        &format!(
            "{}; 26 bits necessary; 4 disjoint families; 4 legal reaching prefixes",
            fmt_commas(floor.to_string().parse::<u128>().expect("fits"))
        ),
    );
    r.line(
        "r_nf_zero_supplemental",
        &format!("{} supplemental bits", zero_supplemental_check()),
    );
    r.finish()
}
