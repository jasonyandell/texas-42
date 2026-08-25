//! `solver::act` — the §16.4 decision controller as an ACTING player.
//!
//! EXPLORATORY tier; CE thread (CE = sampling depth — label every result
//! read off this surface as CE-thread). Sits below every evidentiary
//! tier and is cited by nothing above it. Estimates, never receipts; not
//! a P-A21 statement. No floats anywhere.
//!
//! This module turns the shadow instrument's controller run
//! (`bin/shadow.rs`, which RECORDS and never acts) into one library
//! action policy that play surfaces (`bin/controller_bridge.rs`,
//! `bin/webtable.rs`, `bin/playtable.rs`) consume as thin callers. One
//! decision = one [`act`] call on the public [`DrivenState`].
//!
//! ## The action policy, and where the correctness boundary sits
//!
//! 1. The controller settles a winner — `ExactFrozenSet` with a unique
//!    maximum, or `DeltaSettled` — and the winner is played. These are
//!    INSIDE the correctness boundary: exact results spend no sampling
//!    risk (§6.1), and δ-settled winners are best-of-the-fixed-set except
//!    on an event of probability at most the declared scope budget.
//! 2. An honest exact tie (`ExactFrozenSet` with `winner: null`): the
//!    tile is chosen among the TIED maxima by the live `level1_evaluate`
//!    ordering. Never index-broken.
//! 3. `Unresolved` at the world cap: the tile is chosen among the
//!    δ-SURVIVORS by the same live `level1_evaluate` ordering.
//!
//! The δ-safe ELIMINATIONS behind routes 2–3 remain inside the
//! correctness boundary (a candidate is removed only by a settled edge at
//! the declared risk). The LEVEL-1 RANKING among survivors or exact ties
//! is a scheduling/ordering choice OUTSIDE the correctness boundary — the
//! W7/filtration license: a predictable ordering heuristic affects cost,
//! never truth. A fallback choice is therefore never presented as a
//! settled winner: every [`ActDecision`] carries the route that chose the
//! tile, and [`ActRoute::settled`] is `false` on every fallback route.
//!
//! ## Caps
//!
//! `world_cap` is a resource limit producing honest `Unresolved`, never a
//! settlement rule (§1.5, CE-A3/A5). A LOW cap therefore only produces
//! more `Unresolved` → fallback decisions; it can never produce a wrong
//! settlement. Interactive surfaces default to
//! [`ActConfig::interactive`] (cap 128 — a think-time budget; trick-1/2
//! decisions at cap 512 cost minutes) and batch surfaces to
//! [`ActConfig::full`] (cap 512, the shadow bin's default epoch).

use num_bigint::BigInt;
use num_rational::BigRational;

use crate::rules::rules::legal_plays;
use crate::rules::{Decl, Domino, Team};
use crate::solver::adaptive::{driven_root, DrivenState};
use crate::solver::controller::{
    evaluate_set, exact_frozen_set, CandidateSet, EscalationConfig, RiskPlan, SetEvaluation,
    SetResult, SetSpec,
};
use crate::solver::evidence::{decision_delta, ScopedDelta};
use crate::solver::policy::{
    continuation_frame, t1_frame_bid, ActionRule, DecisionMode, FreezeTuple, FrozenPolicy,
    InnerSchedule, Level0Field, TieRule, NO_DEADLINE_SECS,
};
use crate::solver::{best_of, level1_evaluate, mask_of, mix, record_hash, SplitMix64};

/// Frozen seed for the fallback's level-1 discovery stream — a distinct
/// domain constant from every other surface seed. The per-decision
/// derivation follows the walt_bridge information-consistent pattern
/// (audited CLEAN): seed ^ mix(own remaining hand) ^ record_hash(record).
pub const ACT_FALLBACK_SEED: u64 = 0x4528_21E6_38D0_1377;

/// The default run-scoped risk budget: δ_run = 1/100 per hand (the §6 run
/// scope, matching the shadow instrument).
#[must_use]
pub fn delta_run_default() -> BigRational {
    BigRational::new(BigInt::from(1), BigInt::from(100))
}

/// The knobs of one acting-controller surface. Every count is a declared
/// approximation parameter (CE-A5), never a stopping rule.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActConfig {
    /// Declared outer count of every frozen level-1 continuation
    /// candidate (identity field of its FreezeTuple).
    pub n_outer_frozen: u64,
    /// Declared inner count of the candidates AND of the declared
    /// level-0 evaluation field.
    pub n0_frozen: u64,
    /// Controller resource cap in raw worlds (`Unresolved` producer,
    /// never a settlement rule).
    pub world_cap: u64,
    /// Fibers at or below this run the exact frozen-set endpoint
    /// directly (always sound; spends no risk, §6.1).
    pub exact_cap: u128,
    /// The fallback ranking's live `level1_evaluate` outer count.
    pub fallback_n_outer: usize,
    /// The fallback ranking's live `level1_evaluate` inner count.
    pub fallback_n0: usize,
}

impl ActConfig {
    /// Interactive default: world cap 128 — a think-time budget for live
    /// tables. A low cap only produces more honest Unresolved→fallback
    /// decisions, never wrong settlements.
    #[must_use]
    pub fn interactive() -> ActConfig {
        ActConfig {
            n_outer_frozen: 8,
            n0_frozen: 2,
            world_cap: 128,
            exact_cap: 2000,
            fallback_n_outer: 200,
            fallback_n0: 8,
        }
    }

    /// Batch default: world cap 512 (the shadow bin's default epoch).
    /// Trick-1/2 decisions at this cap cost minutes of wall time.
    #[must_use]
    pub fn full() -> ActConfig {
        ActConfig {
            world_cap: 512,
            ..ActConfig::interactive()
        }
    }
}

/// Which route chose the tile. [`ActRoute::Forced`], [`ActRoute::ExactWinner`],
/// and [`ActRoute::DeltaSettled`] are inside the correctness boundary; the
/// three `..Level1` routes are level-1 ORDERING choices among honest
/// survivors/ties — outside it, and labeled so.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActRoute {
    /// One legal tile; the controller never ran.
    Forced,
    /// `ExactFrozenSet` with a unique maximum: exact best of the frozen
    /// set under the declared field model.
    ExactWinner,
    /// `DeltaSettled`: best of the frozen set except on an event of
    /// probability at most the declared scope budget.
    DeltaSettled,
    /// Honest exact tie (`winner:null`): level-1 rank among the TIED
    /// maxima. An ordering choice, not a settlement.
    ExactTieLevel1,
    /// `Unresolved` at the cap: level-1 rank among the δ-survivors. An
    /// ordering choice, not a settlement.
    UnresolvedLevel1,
    /// `EpsilonEquivalent` survivors (only when an equivalence plan is
    /// configured; the strict plan used here never produces it): level-1
    /// rank among the pairwise-ε-equivalent survivors.
    EpsilonLevel1,
}

impl ActRoute {
    /// The mechanical route tag carried by every record.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            ActRoute::Forced => "forced",
            ActRoute::ExactWinner => "exact-winner",
            ActRoute::DeltaSettled => "delta-settled",
            ActRoute::ExactTieLevel1 => "exact-tie-level1",
            ActRoute::UnresolvedLevel1 => "unresolved-level1",
            ActRoute::EpsilonLevel1 => "epsilon-level1",
        }
    }

    /// `true` exactly when the tile was settled INSIDE the correctness
    /// boundary (forced/exact/δ-settled). Every level-1 fallback route is
    /// `false` — a fallback is never presented as a settled winner.
    #[must_use]
    pub fn settled(self) -> bool {
        match self {
            ActRoute::Forced | ActRoute::ExactWinner | ActRoute::DeltaSettled => true,
            ActRoute::ExactTieLevel1 | ActRoute::UnresolvedLevel1 | ActRoute::EpsilonLevel1 => {
                false
            }
        }
    }
}

/// The pure routing verdict on one controller result: either the
/// controller settled a winner, or an honest survivor/tie set goes to the
/// fallback ordering. Indices are candidate indices (= legal-tile order).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RouteChoice {
    Settled { winner: usize, route: ActRoute },
    Fallback { among: Vec<usize>, route: ActRoute },
}

/// THE ACTION POLICY as a pure function of the controller result — every
/// `SetResult` variant maps to exactly one route (no wildcard arm, so a
/// new result kind is a compile error here, never a silent fallthrough).
#[must_use]
pub fn route_result(result: &SetResult) -> RouteChoice {
    match result {
        SetResult::ExactFrozenSet {
            winner: Some(k), ..
        } => RouteChoice::Settled {
            winner: *k,
            route: ActRoute::ExactWinner,
        },
        SetResult::ExactFrozenSet {
            winner: None, wins, ..
        } => {
            let best = wins.iter().max().expect("m >= 2 candidates");
            let among: Vec<usize> = wins
                .iter()
                .enumerate()
                .filter(|(_, w)| *w == best)
                .map(|(k, _)| k)
                .collect();
            assert!(among.len() >= 2, "an exact tie names at least two maxima");
            RouteChoice::Fallback {
                among,
                route: ActRoute::ExactTieLevel1,
            }
        }
        SetResult::DeltaSettled { winner, .. } => RouteChoice::Settled {
            winner: *winner,
            route: ActRoute::DeltaSettled,
        },
        SetResult::EpsilonEquivalent { survivors, .. } => {
            assert!(!survivors.is_empty(), "survivors are nonempty");
            RouteChoice::Fallback {
                among: survivors.clone(),
                route: ActRoute::EpsilonLevel1,
            }
        }
        SetResult::Unresolved { survivors, .. } => {
            assert!(!survivors.is_empty(), "survivors are nonempty");
            RouteChoice::Fallback {
                among: survivors.clone(),
                route: ActRoute::UnresolvedLevel1,
            }
        }
    }
}

/// One acted decision: the tile, the route that chose it, and the full
/// controller record (a derived view of the evidence stream — kept so
/// surfaces can log which route chose the tile; never a second authority).
pub struct ActDecision {
    /// The chosen tile (always legal).
    pub tile: Domino,
    /// Which action-policy route chose it.
    pub route: ActRoute,
    /// The tiles the route chose among: the winner alone when settled,
    /// the tied set / δ-survivors on a fallback route.
    pub among: Vec<Domino>,
    /// The legal tiles in candidate-index order (index `k` of the
    /// controller result names `legal[k]`).
    pub legal: Vec<Domino>,
    /// The controller's complete record; `None` exactly on a forced play.
    pub evaluation: Option<SetEvaluation>,
    /// "forced" | "preroute" | "sampled" | "escalated" — which controller
    /// endpoint ran.
    pub controller_route: &'static str,
    /// The fallback's level-1 estimates over `among` (fallback routes
    /// with at least two members only). Ordering evidence, not values of
    /// record.
    pub fallback_opts: Option<Vec<(u8, BigRational)>>,
}

/// The frozen level-1 continuation tuple for one pinned root action —
/// field-for-field the shadow instrument's candidate identity (same
/// strings, so equal schedules yield equal `PolicyId`s across shadow
/// records and acted decisions).
#[must_use]
pub fn continuation_tuple(
    decl: Decl,
    bid: u32,
    declaring_team: Team,
    n_outer_frozen: u64,
    n0_frozen: u64,
    pinned: Domino,
) -> FreezeTuple {
    FreezeTuple {
        solver_source: "walt-level1-continuation-v1 (solver::level1_evaluate; \
                        saturation-tie refinement 4x per round capped at 16x)"
            .to_string(),
        decl,
        bid,
        declaring_team,
        field_model: "level0".to_string(),
        field_level: 0,
        inner_schedule: InnerSchedule::Declared(vec![n_outer_frozen, n0_frozen]),
        discovery_stream: "policy-discovery-splitmix64-counter-v1".to_string(),
        discovery_seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        practical_equivalence: None,
        policy_library: "level1-continuation-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
        action_rule: ActionRule::PinnedThenLevel1 { pinned },
    }
}

/// Act at one driven decision: run the §16.4 controller on one frozen
/// level-1 continuation per legal root action under a run-scoped strict
/// risk plan (`δ_d = δ_run/(d(d+1))` for decision event `d`), then apply
/// the action policy. `d` may be any injective assignment of decision
/// events within the run — the §6 allocation telescopes, so any subset
/// of ordinals stays within `δ_run`; play surfaces use
/// `d = plies played + 1`, derivable statelessly from the public record.
#[must_use]
pub fn act(
    state: &DrivenState<'_>,
    cfg: &ActConfig,
    run_scope: &str,
    d: u64,
    delta_run: &BigRational,
) -> ActDecision {
    let decl = state.decl;
    let viewer = state.leader.plus(state.trick_plays.len());
    let led = state.trick_plays.first().map(|t| decl.led_context(*t));
    let legal_set = legal_plays(decl, state.viewer_hand, led);
    assert!(!legal_set.is_empty(), "a seat to move holds a legal tile");
    let legal: Vec<Domino> = legal_set.iter().collect();
    if legal.len() == 1 {
        return ActDecision {
            tile: legal[0],
            route: ActRoute::Forced,
            among: legal.clone(),
            legal,
            evaluation: None,
            controller_route: "forced",
            fallback_opts: None,
        };
    }
    let (root, position) = driven_root(state).expect("a driven decision has a lawful kernel");
    let owned: Vec<FrozenPolicy> = legal
        .iter()
        .map(|t| {
            FrozenPolicy::new(continuation_tuple(
                decl,
                state.bid,
                state.declaring_team,
                cfg.n_outer_frozen,
                cfg.n0_frozen,
                *t,
            ))
        })
        .collect();
    let candidates = CandidateSet::new(owned.iter().collect());
    let dec_scope = format!("{run_scope}:d{d}");
    let plan = RiskPlan::strict(ScopedDelta::new(dec_scope, decision_delta(d, delta_run)))
        .under_run(
            ScopedDelta::new(run_scope.to_string(), delta_run.clone()),
            d,
        );
    let field = Level0Field::new(cfg.n0_frozen as usize);
    let spec = SetSpec {
        root: &root,
        position: &position,
        candidates: &candidates,
        field: &field,
        plan,
        world_cap: cfg.world_cap,
        batch: 8,
        escalation: Some(EscalationConfig {
            cost_sample: 1,
            cost_enumerate: 1,
            check_every: 8,
        }),
    };
    let (evaluation, controller_route) = if root.count() <= cfg.exact_cap {
        (exact_frozen_set(&spec), "preroute")
    } else {
        let evaluation = evaluate_set(&spec);
        let route = if evaluation.escalation.is_some() {
            "escalated"
        } else {
            "sampled"
        };
        (evaluation, route)
    };
    match route_result(&evaluation.result) {
        RouteChoice::Settled { winner, route } => {
            let tile = legal[winner];
            ActDecision {
                tile,
                route,
                among: vec![tile],
                legal,
                evaluation: Some(evaluation),
                controller_route,
                fallback_opts: None,
            }
        }
        RouteChoice::Fallback { among, route } => {
            let tiles: Vec<Domino> = among.iter().map(|&k| legal[k]).collect();
            let (tile, opts) = if tiles.len() == 1 {
                (tiles[0], None)
            } else {
                let frame = continuation_frame(decl, &position, &[]);
                assert_eq!(frame.seat, viewer, "the frame's seat to move is the viewer");
                let among_mask = tiles.iter().fold(0u32, |m, t| m | (1u32 << t.index()));
                let mut rng = SplitMix64(
                    ACT_FALLBACK_SEED
                        ^ mix(u64::from(state.viewer_hand.bits()))
                        ^ record_hash(&frame.key),
                );
                let opts = level1_evaluate(
                    decl,
                    t1_frame_bid(state.bid, state.declaring_team),
                    viewer,
                    mask_of(state.viewer_hand),
                    among_mask,
                    &frame.key,
                    frame.sizes(),
                    frame.voids,
                    frame.trick_start_played,
                    frame.boundary_hand_size,
                    cfg.fallback_n_outer,
                    cfg.fallback_n0,
                    NO_DEADLINE_SECS,
                    &mut rng,
                )
                .expect("the fallback ranking runs without a wall-clock cutoff");
                let choice = best_of(&opts, viewer.team() == Team::T1);
                (
                    Domino::from_index(usize::from(choice)).expect("tile < 28"),
                    Some(opts),
                )
            };
            assert!(tiles.contains(&tile), "the fallback chooses among the set");
            ActDecision {
                tile,
                route,
                among: tiles,
                legal,
                evaluation: Some(evaluation),
                controller_route,
                fallback_opts: opts,
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Pure-routing gates (the action policy given a controller result).
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::Pip;
    use crate::solver::adaptive::{StreamIdentity, SAMPLER_ID};
    use crate::solver::controller::{epoch_identity, DecisionLedger};
    use crate::solver::evidence::edge_threshold;
    use crate::solver::policy::PolicyId;

    fn preference_policy(lowest_first: bool) -> FrozenPolicy {
        let mut order: Vec<Domino> = (0..28)
            .map(|i| Domino::from_index(i).expect("tile"))
            .collect();
        if !lowest_first {
            order.reverse();
        }
        FrozenPolicy::new(FreezeTuple {
            solver_source: "act-route-fixture".to_string(),
            decl: Decl::PipTrump(Pip::new(5).expect("pip")),
            bid: 30,
            declaring_team: Team::T1,
            field_model: "level0".to_string(),
            field_level: 0,
            inner_schedule: InnerSchedule::None,
            discovery_stream: "none".to_string(),
            discovery_seed_schedule: vec![],
            tie_rule: TieRule::FirstInPreference,
            practical_equivalence: None,
            policy_library: "fixture".to_string(),
            mode: DecisionMode::Exact,
            action_rule: ActionRule::Preference(order),
        })
    }

    /// A lawful ledger fixture built through the real constructors (the
    /// epoch is a genuine content address, not a dummy).
    fn fixture(m: u64) -> (DecisionLedger, Vec<PolicyId>) {
        let a = preference_policy(true);
        let b = preference_policy(false);
        let candidates = CandidateSet::new(vec![&a, &b]);
        let delta = ScopedDelta::new(
            "decision:act-fixture",
            BigRational::new(BigInt::from(1), BigInt::from(200)),
        );
        let epoch = epoch_identity(0, &candidates, &delta);
        let stream = StreamIdentity {
            sampler: SAMPLER_ID,
            root_id: 0,
            epoch: epoch.stream_epoch(),
            with_replacement: true,
            fiber: 1,
        };
        let edges = delta.delta().clone();
        let ledger = DecisionLedger {
            plan: RiskPlan::strict(delta),
            m,
            edge_alpha: &edges / BigRational::from_integer(BigInt::from(m * (m - 1))),
            edge_threshold: edge_threshold(m, &edges),
            eq_alpha: None,
            eq_tests: 0,
            epoch,
            stream,
        };
        let ids = candidates.ids();
        (ledger, ids)
    }

    #[test]
    fn an_exact_winner_routes_settled_inside_the_boundary() {
        let (ledger, ids) = fixture(3);
        let result = SetResult::ExactFrozenSet {
            wins: vec![3, 7, 5],
            fiber: 9,
            winner: Some(1),
            policy_ids: vec![ids[0], ids[1], ids[0]],
            ledger,
        };
        let routed = route_result(&result);
        assert_eq!(
            routed,
            RouteChoice::Settled {
                winner: 1,
                route: ActRoute::ExactWinner
            }
        );
        assert!(ActRoute::ExactWinner.settled());
    }

    #[test]
    fn an_honest_exact_tie_routes_fallback_among_the_tied_maxima() {
        let (ledger, ids) = fixture(4);
        let result = SetResult::ExactFrozenSet {
            wins: vec![3, 5, 5, 4],
            fiber: 9,
            winner: None,
            policy_ids: vec![ids[0], ids[1], ids[0], ids[1]],
            ledger,
        };
        let routed = route_result(&result);
        assert_eq!(
            routed,
            RouteChoice::Fallback {
                among: vec![1, 2],
                route: ActRoute::ExactTieLevel1
            }
        );
        assert!(!ActRoute::ExactTieLevel1.settled());
    }

    #[test]
    fn a_delta_settled_winner_routes_settled_inside_the_boundary() {
        let (ledger, ids) = fixture(2);
        let result = SetResult::DeltaSettled {
            winner: 0,
            winner_id: ids[0],
            settled_at: 41,
            ledger,
        };
        assert_eq!(
            route_result(&result),
            RouteChoice::Settled {
                winner: 0,
                route: ActRoute::DeltaSettled
            }
        );
        assert!(ActRoute::DeltaSettled.settled());
    }

    #[test]
    fn unresolved_routes_fallback_among_the_delta_survivors() {
        let (ledger, _) = fixture(3);
        let result = SetResult::Unresolved {
            survivors: vec![0, 2],
            consumed: 128,
            refinements: vec![],
            ledger,
        };
        assert_eq!(
            route_result(&result),
            RouteChoice::Fallback {
                among: vec![0, 2],
                route: ActRoute::UnresolvedLevel1
            }
        );
        assert!(!ActRoute::UnresolvedLevel1.settled());
    }

    #[test]
    fn epsilon_equivalent_routes_fallback_among_the_survivors() {
        let (ledger, _) = fixture(3);
        let result = SetResult::EpsilonEquivalent {
            survivors: vec![1, 2],
            epsilon: BigRational::new(BigInt::from(1), BigInt::from(50)),
            settled_at: 77,
            ledger,
        };
        assert_eq!(
            route_result(&result),
            RouteChoice::Fallback {
                among: vec![1, 2],
                route: ActRoute::EpsilonLevel1
            }
        );
        assert!(!ActRoute::EpsilonLevel1.settled());
    }

    /// The route alphabet is exactly six labels, pairwise distinct, and
    /// exactly three sit inside the correctness boundary — an exhaustive
    /// count in CI style.
    #[test]
    fn the_route_alphabet_is_six_distinct_labels_three_settled() {
        let routes = [
            ActRoute::Forced,
            ActRoute::ExactWinner,
            ActRoute::DeltaSettled,
            ActRoute::ExactTieLevel1,
            ActRoute::UnresolvedLevel1,
            ActRoute::EpsilonLevel1,
        ];
        assert_eq!(routes.len(), 6);
        for (i, a) in routes.iter().enumerate() {
            for b in &routes[i + 1..] {
                assert_ne!(a.label(), b.label(), "route labels are distinct");
            }
        }
        assert_eq!(routes.iter().filter(|r| r.settled()).count(), 3);
        assert!(
            routes
                .iter()
                .filter(|r| !r.settled())
                .all(|r| r.label().ends_with("level1")),
            "every fallback route is labeled as the level-1 ordering choice"
        );
    }
}
