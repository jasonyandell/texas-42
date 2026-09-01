//! The §62 count-threat cover producer (anytime proof-state Phase 5):
//! the first safe, deliberately incomplete `CountThreatCover` source.
//! Per legal root action holding an incumbent profile fact, it names
//! the §13 resources still in play — contested tricks, five-count and
//! ten-count tiles outside completed tricks — verifies the uniform
//! point-gain inequality with the exact declaring-score range walk
//! ([`factor_belief::declaring_score_range`]: the free-focal range
//! covers EVERY information-consistent deviation sharing the root
//! action, the incumbent's own range is read off its profile fact),
//! and installs the verified cover. Closure then derives the §10/§11
//! rescue-band upper through the incumbent's profile.
//!
//! Mathematical source: `walt/math/anytime_proof_state_score_v0.1.md`
//! §62 (the Phase 5 gate list), §10/§11 (uniform gain and loss
//! bounds, rescue and fragile bands), §13 (count-threat caps and the
//! `CountThreatCover` shape), §5 (the arithmetic envelope the
//! resources decompose), under ruling APS-A9 (phases in-crate).
//!
//! THE VERIFIER. `d⁺ = dev_max − incumbent_floor` (declaring viewer;
//! the §11 mirror `d⁻ = incumbent_ceiling − dev_min` for the setting
//! viewer): every deviation's per-world score sits inside the
//! free-focal range and the incumbent's per-world score inside its
//! profile's support, so the difference uniformly bounds the §10
//! per-world gain — fusion is admitted as a relaxation, which can
//! only WIDEN the bound, never understate it (gated). The verified
//! bound never exceeds the §5 arithmetic resource sum (asserted: the
//! install fence requires it).
//!
//! THE DECLINE PATH (§62: "the producer may decline"). No incumbent
//! profile fact for an action → no cover, no number — a cover is
//! relative to its incumbent, and fabricating one would manufacture
//! an anchor the state has not paid for. Declines are visible: the
//! per-action entry point returns `None` and the producer simply
//! omits the action.
//!
//! WHAT THIS PRODUCER IS NOT. Deliberately incomplete (§62): the
//! resource vocabulary is the whole-cell §5 decomposition — no
//! per-cell partitions, no ruff-transfer or highest-trump protection
//! conditions yet (richer structural producers are the declared §70
//! answer if these covers stay vacuous). One cover per action against
//! the STRONGEST incumbent only. Nothing sampled anywhere.

use crate::rules::{Domino, DominoSet};
use crate::solver::adaptive::{root_identity, CanonicalRoot, RootPosition, SlicePolicy};
use crate::solver::factor_belief::{
    declaring_score_range, ExactCoverOracle, FactorBelief, ResponseStats,
};
use crate::solver::proof_state::{
    CountThreatCoverFact, Fact, ProofProducer, ProofState, ScoreProfileFact,
};

/// The §13 policy region every cover here declares: all
/// information-consistent focal policies sharing the root action (the
/// §36 deviation domain — the region the free-focal range walk
/// covers).
pub const COVER_REGION: &str = "info-consistent-full";

/// The strongest incumbent profile fact for `action` under the
/// state's declared utility: max viewer-objective projection, first
/// fact on ties (the state's insertion order is the declared order).
fn strongest_incumbent(state: &ProofState, action: Domino) -> Option<&ScoreProfileFact> {
    let mut best: Option<(&ScoreProfileFact, u128, u128)> = None;
    for sf in state.facts() {
        let Fact::Profile(p) = &sf.fact else { continue };
        if p.action != action {
            continue;
        }
        let z = p.total();
        let tail = p.tail(state.identity.contract);
        let mass = match state.identity.utility_id.as_str() {
            "pmake-v1" => tail,
            "pmake-setting-v1" => z - tail,
            other => panic!("an unknown utility identity: {other}"),
        };
        // Exact cross-multiplied comparison; strict improvement only.
        let better = match &best {
            None => true,
            Some((_, bm, bz)) => mass * *bz > *bm * z,
        };
        if better {
            best = Some((p, mass, z));
        }
    }
    best.map(|(p, _, _)| p)
}

/// The §62 cover of ONE root action, or the decline. Resources are the
/// exact §5 decomposition at the root: contested tricks (one point
/// each) and count tiles outside completed tricks; the verified
/// `score_gain_upper` comes from the range walk and never exceeds the
/// resource sum (asserted).
pub fn cover_for_action(
    oracle: &dyn ExactCoverOracle,
    root: &CanonicalRoot,
    position: &RootPosition,
    field: &dyn SlicePolicy,
    state: &ProofState,
    action: Domino,
) -> Option<Fact> {
    let incumbent = strongest_incumbent(state, action)?;
    let floor = incumbent
        .bins
        .iter()
        .position(|m| *m > 0)
        .map(|s| u32::try_from(s).expect("s <= 42"))
        .expect("a profile fact holds mass");
    let ceiling = incumbent
        .bins
        .iter()
        .rposition(|m| *m > 0)
        .map(|s| u32::try_from(s).expect("s <= 42"))
        .expect("a profile fact holds mass");
    let child = FactorBelief::uniform_root(root, position, field).focal_play(action);
    let mut stats = ResponseStats::default();
    let (dev_min, dev_max) = declaring_score_range(oracle, &child, None, field, &mut stats);
    // The incumbent is one policy in the region, so its support sits
    // inside the free-focal range.
    assert!(
        dev_min <= floor && ceiling <= dev_max,
        "the incumbent's profile support sits inside the deviation range"
    );
    let gain = match state.identity.utility_id.as_str() {
        // §10: deviations raise the declaring score by at most this.
        "pmake-v1" => dev_max - floor,
        // §11 mirror: deviations lower it by at most this.
        "pmake-setting-v1" => ceiling - dev_min,
        other => panic!("an unknown utility identity: {other}"),
    };
    // The §5 resource decomposition at the root: the position is a
    // trick start (asserted at ProofState::open), so completed-trick
    // tiles are exactly `prior_played` and every remaining trick is
    // one contested trick point.
    assert!(
        position.trick_plays.is_empty(),
        "covers are declared at trick-start roots"
    );
    let in_play = DominoSet::FULL.difference(position.prior_played);
    let fives: Vec<Domino> = in_play.iter().filter(|d| d.count() == 5).collect();
    let tens: Vec<Domino> = in_play.iter().filter(|d| d.count() == 10).collect();
    let tricks = u32::try_from(root.kernel().viewer_hand().len()).expect("at most 7 tricks");
    let unbanked = 42 - position.banked[0] - position.banked[1];
    assert_eq!(
        unbanked,
        tricks + 5 * fives.len() as u32 + 10 * tens.len() as u32,
        "the §5 remainder decomposes exactly into tricks and count tiles"
    );
    assert!(
        gain <= unbanked,
        "the verified movement bound sits inside the arithmetic envelope"
    );
    Some(Fact::Cover(Box::new(CountThreatCoverFact {
        action,
        region_id: COVER_REGION.to_string(),
        incumbent_policy_id: incumbent.policy_id.clone(),
        trick_gain_upper: tricks,
        five_count_tiles: fives,
        ten_count_tiles: tens,
        score_gain_upper: gain,
    })))
}

/// The §62 count-threat producer over one root: one verified cover per
/// legal action holding an incumbent profile, declines omitted.
pub struct CountThreatProducer<'a> {
    pub oracle: &'a dyn ExactCoverOracle,
    pub root: &'a CanonicalRoot,
    pub position: &'a RootPosition,
    pub field: &'a dyn SlicePolicy,
}

impl ProofProducer for CountThreatProducer<'_> {
    fn name(&self) -> &str {
        "count-threat-v1"
    }

    fn produce(&self, state: &ProofState) -> Vec<Fact> {
        assert_eq!(
            state.identity.root_id,
            root_identity(self.root, self.position),
            "the producer's context is the state's root"
        );
        assert_eq!(
            state.identity.contract, self.position.bid,
            "the producer's contract is the state's"
        );
        state
            .legal
            .iter()
            .filter_map(|a| {
                cover_for_action(self.oracle, self.root, self.position, self.field, state, *a)
            })
            .collect()
    }
}
