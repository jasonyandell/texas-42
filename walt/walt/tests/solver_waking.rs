//! Gates for the waking seat (`solver::waking`) — act's σ0 baseline, the
//! hard-budgeted wake check, the wake-gated σ1 escalation, and the
//! per-decision census. Parent sections: calculated_evidence §14 (the
//! CE-A6 wake-up split), §16.4/§6 (the decision controller and its risk
//! convention); targeted_level2_field_stability §8 (the escalation);
//! rulings CE-A6, CE-A7/§20.16 (variant surface, defaults untouched),
//! L2-A4 (screen discipline).
//!
//! DECLARED TEST EPOCH PAIR (one (σ0, σ1) pair per experiment epoch;
//! deliberately cheaper than the live pair): σ0 = Level0 { n0 = 2 },
//! σ1 = Level1 { n_outer = 2, n0 = 2 }, frozen focal candidates at
//! declared schedule [2, 2]. Roots from the frozen `verify_player`
//! receipt. The live pair (σ1 = Level1 { 4, 2 }, candidates [8, 2]) and
//! the l2_controller probe's pair (σ0 n0 = 8) are DIFFERENT epochs:
//! numbers do not compose across them.
//!
//! The compile_fail typing locks (a wake is a crossing witness, never a
//! construction; a recorded fallback carries no number) are doctests on
//! the `solver::waking` module.

mod common;

use std::panic::{catch_unwind, AssertUnwindSafe};

use common::receipt;
use num_bigint::BigInt;
use num_rational::BigRational;
use walt::kernel::Kernel;
use walt::rules::receipt::Receipt;
use walt::rules::replay::state_before_trick;
use walt::rules::{ContextSet, Decl, Domino, DominoSet, Seat, Team};
use walt::solver::act::{act, delta_run_default, ActConfig};
use walt::solver::adaptive::{CanonicalRoot, DrivenState, RootPosition};
use walt::solver::policy::{
    ActionRule, DecisionMode, FreezeTuple, FrozenPolicy, InnerSchedule, TieRule,
};
use walt::solver::targeted::{
    delta_frozen_baseline, legal_root_actions, RefusalReason, StageFourOutcome, TypedRefusal,
};
use walt::solver::waking::{
    route_stage_four, waking_decision_scope, EscalationCensus, EscalationOutcome, FallbackReason,
    WakingCensus, WakingConfig, WakingPath, WakingSeat,
};

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(BigInt::from(n), BigInt::from(d))
}

/// The declared cheap test epoch (module doc header).
fn cheap_config() -> WakingConfig {
    WakingConfig {
        act: ActConfig {
            n_outer_frozen: 2,
            n0_frozen: 2,
            world_cap: 16,
            exact_cap: 2000,
            fallback_n_outer: 8,
            fallback_n0: 2,
        },
        sigma1_n_outer: 2,
        sigma1_n0: 2,
        wake_world_budget: 8,
        wake_exact_fiber_cap: 1024,
        escalation_exact_fiber_cap: 4096,
        escalation_baseline_prefix: 48,
        escalation_e3_prefix: 8,
        delta_wake_run: q(1, 20),
        eps_q: q(1, 20),
    }
}

/// An owned driven decision point at the start of a receipt trick: the
/// public frame by replay, the viewer's hand from the receipt deal.
struct Owned {
    position: RootPosition,
    viewer_hand: DominoSet,
    leader: Seat,
    decl: Decl,
    bid: u32,
    declaring_team: Team,
    banked: [u32; 2],
    prior_played: DominoSet,
    voids: [ContextSet; 4],
}

fn owned_at(r: &Receipt, hand_id: usize, trick_no: usize) -> Owned {
    let hand = &r.hands[hand_id];
    assert_eq!(hand.id, hand_id);
    let position = RootPosition::from_receipt_trick(hand, trick_no).expect("a valid position");
    let (hands, leader) = state_before_trick(hand, trick_no).expect("a valid hand");
    Owned {
        viewer_hand: hands[leader.index()],
        leader,
        decl: position.decl,
        bid: position.bid,
        declaring_team: position.declaring_team,
        banked: position.banked,
        prior_played: position.prior_played,
        voids: position.voids,
        position,
    }
}

impl Owned {
    fn state(&self) -> DrivenState<'_> {
        DrivenState {
            decl: self.decl,
            bid: self.bid,
            declaring_team: self.declaring_team,
            viewer_hand: self.viewer_hand,
            leader: self.leader,
            trick_plays: &self.position.trick_plays,
            banked: self.banked,
            prior_played: self.prior_played,
            voids: self.voids,
        }
    }

    fn d(&self) -> u64 {
        (self.prior_played.len() + self.position.trick_plays.len() + 1) as u64
    }
}

// ---------------------------------------------------------------------------
// The forced path: a census record, no detection.
// ---------------------------------------------------------------------------

#[test]
fn a_forced_play_emits_a_census_record_and_never_runs_detection() {
    let r = receipt();
    // Trick 7: every seat holds one tile — the play is forced.
    let owned = owned_at(&r, 0, 7);
    let seat = WakingSeat::new(cheap_config());
    let decision = seat.decide(&owned.state(), "run:gate-forced", owned.d());
    let census = &decision.census;
    assert_eq!(census.legal, 1);
    assert_eq!(census.path, WakingPath::Forced);
    assert_eq!(census.path.tag(), "forced");
    assert!(census.wake_kind.is_none(), "no detection ran");
    assert!(census.rival.is_none(), "no rival was paired");
    assert!(census.escalation.is_none(), "no escalation ran");
    assert_eq!(census.wake_us, 0, "no wake-check spend");
    assert_eq!(census.wake_worlds, 0, "no worlds consumed");
    assert_eq!(census.escalation_us, 0);
    assert_eq!(decision.tile, census.sigma0);
    assert_eq!(census.played, census.sigma0);
    assert!(census.agreed);
    assert_eq!(census.trick, 7);
    // The record round-trips through its JSONL serialization.
    let line = census.to_jsonl("gate-forced");
    assert_eq!(WakingCensus::parse_jsonl(&line).as_ref(), Some(census));
}

// ---------------------------------------------------------------------------
// Honest-outcome discipline: within-budget unsettled is NO wake, and the
// seat plays exactly today's level-0 choice.
// ---------------------------------------------------------------------------

#[test]
fn within_budget_unsettled_plays_exactly_the_sigma0_choice() {
    let r = receipt();
    // receipt-h4-t6 (fiber 90) forced onto the SAMPLED wake route by a
    // tiny exact cap, at a budget too small for any δ crossing (the m=2
    // edge threshold at the declared decision risk is far above what 4
    // paired worlds can accumulate).
    let owned = owned_at(&r, 4, 6);
    let mut cfg = cheap_config();
    cfg.wake_exact_fiber_cap = 8;
    cfg.wake_world_budget = 4;
    let seat = WakingSeat::new(cfg.clone());
    let d = owned.d();
    let decision = seat.decide(&owned.state(), "run:gate-open", d);
    let census = &decision.census;
    assert_eq!(census.path, WakingPath::NoWakeOpen);
    assert_eq!(census.path.tag(), "no-wake-budget-exhausted");
    assert_eq!(census.wake_kind.as_deref(), Some("sampled-open"));
    assert_eq!(census.wake_worlds, 4, "the declared budget was consumed");
    assert!(census.escalation.is_none(), "no wake, no escalation");
    // The played tile IS act's σ0 choice on the same state — asserted
    // against a direct act call, not against a stored copy.
    let direct = act(
        &owned.state(),
        &cfg.act,
        "run:gate-open",
        d,
        &delta_run_default(),
    );
    assert_eq!(decision.tile, direct.tile);
    assert_eq!(census.sigma0, direct.tile);
    assert!(census.agreed);
    // The record round-trips.
    let line = census.to_jsonl("gate-open");
    assert_eq!(WakingCensus::parse_jsonl(&line).as_ref(), Some(census));
}

// ---------------------------------------------------------------------------
// A settled no-wake: the exact route selecting the baseline plays σ0
// without escalation.
// ---------------------------------------------------------------------------

#[test]
fn an_exact_settled_agreement_is_no_wake() {
    let r = receipt();
    // KNOWN CHECK VALUE (deterministic at the declared test epoch):
    // receipt-h4-t6 on the exact wake route settles
    // exact-sigma1-selects-baseline.
    let owned = owned_at(&r, 4, 6);
    let seat = WakingSeat::new(cheap_config());
    let decision = seat.decide(&owned.state(), "run:gate-agree", owned.d());
    let census = &decision.census;
    assert_eq!(census.path, WakingPath::NoWakeSettled);
    assert_eq!(
        census.wake_kind.as_deref(),
        Some("exact-sigma1-selects-baseline")
    );
    assert_eq!(
        census.wake_worlds, 90,
        "the exact route enumerated the fiber"
    );
    assert!(census.escalation.is_none());
    assert_eq!(census.played, census.sigma0);
    assert!(census.agreed);
}

// ---------------------------------------------------------------------------
// The wake path: positive σ1 evidence escalates, and a settled escalation
// selection is the played choice.
// ---------------------------------------------------------------------------

#[test]
fn a_settled_wake_escalates_and_plays_the_settled_selection() {
    let r = receipt();
    // KNOWN CHECK VALUES (deterministic at the declared test epoch):
    // receipt-h5-t5 (fiber 560) wakes on the exact route — the σ1 leg
    // selects the rival [1,0] (index 2) over act's σ0 choice [4,1]
    // (index 9) — and the escalation's exact Stage 4 settles a strict σ1
    // argmax on the same rival, which becomes the played choice.
    let owned = owned_at(&r, 5, 5);
    let seat = WakingSeat::new(cheap_config());
    let decision = seat.decide(&owned.state(), "run:gate-wake", owned.d());
    let census = &decision.census;
    assert_eq!(census.path, WakingPath::Wake);
    assert_eq!(
        census.wake_kind.as_deref(),
        Some("exact-sigma1-selects-rival")
    );
    let rival = census.rival.expect("a wake names its rival");
    assert_eq!(rival.index(), 2, "KNOWN CHECK VALUE: the strongest rival");
    assert_eq!(census.sigma0.index(), 9, "KNOWN CHECK VALUE: act's choice");
    let escalation = census.escalation.as_ref().expect("the escalation ran");
    assert_eq!(escalation.outcome, "exact-survivors");
    assert_eq!(escalation.via, "exact-argmax");
    assert_eq!(escalation.selected, Some(rival));
    assert!(escalation.refusals.is_empty());
    // The controller's per-phase spend vector rides the census verbatim
    // (the targeting data for where escalation microseconds go).
    assert!(
        escalation
            .spend
            .iter()
            .any(|(p, _, _)| p == "baseline-sigma0"),
        "the exact route's Stage-1 phase is on the record"
    );
    assert!(
        escalation
            .spend
            .iter()
            .any(|(p, _, _)| p == "stage4-sigma1"),
        "the survivor σ1 phase is on the record"
    );
    assert_eq!(decision.tile, rival, "the settled selection is played");
    assert_eq!(census.played, rival);
    assert!(!census.agreed, "the escalated choice moved off σ0");
    // The record round-trips with its escalation slice.
    let line = census.to_jsonl("gate-wake");
    assert_eq!(WakingCensus::parse_jsonl(&line).as_ref(), Some(census));
}

#[test]
fn a_refused_escalation_falls_back_to_sigma0_with_the_refusal_recorded() {
    let r = receipt();
    // The same waking root, with the escalation defunded: fiber 560 over
    // an exact cap of 1 and no declared sampled route (baseline prefix
    // 0) is a typed refusal — and the seat plays the σ0 baseline, with
    // the refusal on the record. A refusal is never degraded into a
    // pick.
    let owned = owned_at(&r, 5, 5);
    let mut cfg = cheap_config();
    cfg.escalation_exact_fiber_cap = 1;
    cfg.escalation_baseline_prefix = 0;
    let seat = WakingSeat::new(cfg);
    let decision = seat.decide(&owned.state(), "run:gate-refusal", owned.d());
    let census = &decision.census;
    assert_eq!(census.path, WakingPath::Wake, "the wake still happened");
    let escalation = census.escalation.as_ref().expect("the escalation ran");
    assert_eq!(escalation.outcome, "not-run");
    assert_eq!(escalation.via, "fallback-refused");
    assert_eq!(escalation.selected, None);
    assert!(
        escalation
            .refusals
            .iter()
            .any(|f| f.contains("sampled-route-undeclared")),
        "the typed refusal is on the record"
    );
    assert_eq!(
        census.played, census.sigma0,
        "the recorded fallback plays σ0"
    );
    assert!(census.agreed);
}

// ---------------------------------------------------------------------------
// The escalation routing as a pure function: settlements route to play,
// refusals and open states route to the recorded fallback.
// ---------------------------------------------------------------------------

#[test]
fn stage_four_routing_settlements_and_fallbacks() {
    let tile = Domino::from_index(3).expect("tile");
    // A δ-settled singleton is a settled selection.
    let outcome = StageFourOutcome::DeltaSingleton { selected: tile };
    let EscalationOutcome::Selected { tile: chosen, via } = route_stage_four(&outcome) else {
        panic!("a δ singleton routes to play");
    };
    assert_eq!(chosen, tile);
    assert_eq!(via, "delta-singleton");
    // A typed refusal routes to the recorded fallback, reason typed.
    let refusal = TypedRefusal {
        stage: "baseline",
        reason: RefusalReason::SampledRouteUndeclared,
    };
    let EscalationOutcome::Fallback(fallback) =
        route_stage_four(&StageFourOutcome::NotRun(refusal))
    else {
        panic!("a refusal routes to the recorded fallback");
    };
    assert_eq!(fallback.reason().tag(), "fallback-refused");
    assert!(matches!(
        fallback.reason(),
        FallbackReason::Refused(r) if r.reason == RefusalReason::SampledRouteUndeclared
    ));
}

#[test]
fn open_delta_survivors_route_to_the_recorded_fallback_never_a_pick() {
    // A genuine δ-tier baseline (the public producer on a receipt root)
    // whose selection stayed open: `selected1: None` is the honest open
    // state, and the routing sends it to the recorded fallback.
    let r = receipt();
    let hand = &r.hands[10];
    let kernel = Kernel::from_receipt_trick(hand, 6).expect("a valid kernel");
    let position = RootPosition::from_receipt_trick(hand, 6).expect("a valid position");
    let root = CanonicalRoot::new(kernel);
    let actions: Vec<Domino> = legal_root_actions(&root, &position).iter().collect();
    let policies: Vec<FrozenPolicy> = actions
        .iter()
        .map(|a| {
            FrozenPolicy::new(FreezeTuple {
                solver_source: "walt-level1-continuation-v1".to_string(),
                decl: position.decl,
                bid: position.bid,
                declaring_team: position.declaring_team,
                field_model: "level0".to_string(),
                field_level: 0,
                inner_schedule: InnerSchedule::Declared(vec![2, 2]),
                discovery_stream: "policy-discovery-splitmix64-counter-v1".to_string(),
                discovery_seed_schedule: vec![],
                tie_rule: TieRule::LowestTileIndex,
                practical_equivalence: None,
                policy_library: "level1-continuation-library-v1".to_string(),
                mode: DecisionMode::Heuristic,
                action_rule: ActionRule::PinnedThenLevel1 { pinned: *a },
            })
        })
        .collect();
    let candidates: Vec<(Domino, &FrozenPolicy)> =
        actions.iter().copied().zip(policies.iter()).collect();
    let field0 =
        walt::solver::field::FieldModel::new(walt::solver::waking::sigma0_spec(&cheap_config()));
    let sigma1 = delta_frozen_baseline(
        &root,
        &position,
        &candidates,
        &field0,
        0,
        8,
        &q(1, 400),
        "gate-open-survivors",
    );
    let survivors = sigma1.actions.len();
    let outcome = StageFourOutcome::DeltaSurvivors {
        settled0: sigma1.settled_argmax(),
        selected1: None,
        sigma1,
    };
    let EscalationOutcome::Fallback(fallback) = route_stage_four(&outcome) else {
        panic!("an open δ-survivor set routes to the recorded fallback");
    };
    assert_eq!(fallback.reason().tag(), "fallback-open-survivors");
    assert!(matches!(
        fallback.reason(),
        FallbackReason::OpenSurvivors { survivors: s } if *s == survivors
    ));
}

// ---------------------------------------------------------------------------
// The census format contract.
// ---------------------------------------------------------------------------

#[test]
fn a_census_record_round_trips_through_its_jsonl_serialization() {
    let tile = |i: usize| Domino::from_index(i).expect("tile");
    let census = WakingCensus {
        d: 17,
        trick: 5,
        seat: 3,
        decl: 5,
        fiber: 46_558_512,
        legal: 3,
        path: WakingPath::Wake,
        wake_kind: Some("sampled-sigma1-settles-rival".to_string()),
        rival: Some(tile(2)),
        escalation: Some(EscalationCensus {
            outcome: "not-run".to_string(),
            stop: "refused".to_string(),
            via: "fallback-refused".to_string(),
            selected: None,
            spend: vec![
                ("baseline-sigma0-delta".to_string(), 12_345, 128),
                ("rung-e3".to_string(), 6_789, 3),
            ],
            refusals: vec![
                "TypedRefusal{stage=route;reason=exact-unaffordable{fiber=46558512;cap=4096}}"
                    .to_string(),
                "TypedRefusal{stage=baseline;reason=sampled-route-undeclared}".to_string(),
            ],
        }),
        sigma0: tile(9),
        sigma0_route: "delta-settled".to_string(),
        played: tile(9),
        agreed: true,
        baseline_us: 246_014,
        wake_us: 2_316_112,
        escalation_us: 0,
        wake_worlds: 24,
    };
    let line = census.to_jsonl("run:gate-roundtrip");
    assert_eq!(WakingCensus::parse_jsonl(&line), Some(census));
    // A selected escalation with an empty refusal list round-trips too.
    let census2 = WakingCensus {
        escalation: Some(EscalationCensus {
            outcome: "delta-survivors".to_string(),
            stop: "ladder-complete".to_string(),
            via: "delta-selected1".to_string(),
            selected: Some(tile(11)),
            spend: vec![],
            refusals: vec![],
        }),
        ..WakingCensus::parse_jsonl(&line).expect("parsed")
    };
    let line2 = census2.to_jsonl("run:gate-roundtrip");
    assert_eq!(WakingCensus::parse_jsonl(&line2), Some(census2));
}

// ---------------------------------------------------------------------------
// Risk-scope disjointness: waking scopes are wake:-prefixed, and a caller
// scope inside the waking prefix is rejected mechanically.
// ---------------------------------------------------------------------------

#[test]
fn waking_scopes_are_mechanically_disjoint_from_acts() {
    // Every waking scope extends the wake: stem; act's scopes are
    // `{run_scope}` and `{run_scope}:d{d}` with the caller's run scope,
    // which decide() rejects if it ever carries the waking prefix — so
    // no waking ScopedDelta can collide with an act ScopedDelta.
    let stem = waking_decision_scope("run:gate-scopes", 3);
    assert_eq!(stem, "wake:run:gate-scopes:d3");
    assert!(stem.starts_with("wake:"));
    let r = receipt();
    let owned = owned_at(&r, 0, 7);
    let seat = WakingSeat::new(cheap_config());
    assert!(
        catch_unwind(AssertUnwindSafe(|| {
            seat.decide(&owned.state(), "wake:run:gate-scopes", owned.d())
        }))
        .is_err(),
        "a wake:-prefixed caller scope is rejected, not absorbed"
    );
}
