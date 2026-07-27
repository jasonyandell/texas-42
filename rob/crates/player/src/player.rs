//! The evening player: fixed-field information-set best response at the
//! root, Monte Carlo over the exact fiber (Math §11.4; UTIL-04A/B).
//!
//! The one law (anti-strategy-fusion, Math §11.4): never re-optimize per
//! sampled world. One common continuation policy — fixed before any world is
//! drawn — plays every seat in every simulated world; per-action values are
//! world-averages of rollouts under that same policy (average-then-max,
//! never max-then-average). The evaluator exposes no per-world action hook,
//! so the fused order is unrepresentable through this API.

use rob_core::{algebra_for, derive_rule_cells, DominoId, MechanicalState, PlayPhase};

use crate::policy::RandomLegal;
use crate::rng::SplitMix64;
use crate::rollout::{finish_hand, RolloutPosition};
use crate::worlds::sample_worlds;

/// The utility lens parameter (Math §11.1 named lenses). Values are exact
/// integers from the deciding viewer's team perspective; world-averages are
/// compared as exact integer totals over a common world count (no division,
/// no floats — INV-4).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UtilityLens {
    /// Signed point differential: viewer-team points minus opponent points
    /// (order-equivalent to declaring points for the declaring side,
    /// UTIL-02).
    Points,
    /// Contract success: +1 when the settlement favors the viewer's team,
    /// −1 otherwise.
    ContractSuccess,
}

/// Root-decision configuration for the Monte Carlo best-response player.
#[derive(Clone, Copy, Debug)]
pub struct MonteCarloPlayer {
    /// Worlds sampled per decision (exactly uniform on the fiber).
    pub worlds_per_decision: usize,
    /// The utility lens.
    pub lens: UtilityLens,
    /// Base seed; each decision derives its streams from `(seed, nonce)`.
    pub seed: u64,
}

/// One root decision with its evaluation evidence.
#[derive(Clone, Debug)]
pub struct DecisionReport {
    /// The chosen action (argmax of world-total utility; ties break to the
    /// earliest legal action in canonical identity order).
    pub chosen: DominoId,
    /// The legal set, canonical identity order.
    pub legal: Vec<DominoId>,
    /// Per-action utility totals over the common sampled worlds (aligned
    /// with `legal`). Averages compare identically because the world count
    /// is shared.
    pub totals: Vec<i64>,
    /// Worlds actually sampled (zero when the action was forced).
    pub worlds: usize,
}

/// The viewer's exact legal set from its own information (Exec §11 via the
/// engine's follow relation).
pub fn viewer_legal(state: &MechanicalState) -> Vec<DominoId> {
    let algebra = algebra_for(state.contract().declaration());
    let hand = state.own_remaining_hand();
    if state.current_trick().is_empty() {
        return hand.iter().collect();
    }
    let q = algebra.led_suit(state.current_trick()[0].domino);
    let followers: Vec<DominoId> = hand.iter().filter(|&d| algebra.follows(d, q)).collect();
    if followers.is_empty() {
        hand.iter().collect()
    } else {
        followers
    }
}

impl MonteCarloPlayer {
    /// Decide the viewer's play at `state` (must be the acting seat's own
    /// mechanical state). Deterministic in `(seed, nonce)`.
    pub fn decide(&self, state: &MechanicalState, nonce: u64) -> DecisionReport {
        assert_eq!(
            state.phase(),
            PlayPhase::Play,
            "a decision needs the play phase"
        );
        let viewer = state.viewer();
        assert_eq!(
            state.current_actor(),
            Some(viewer),
            "the deciding seat must be the acting seat"
        );
        let legal = viewer_legal(state);
        assert!(!legal.is_empty());
        if legal.len() == 1 {
            return DecisionReport {
                chosen: legal[0],
                totals: vec![0],
                legal,
                worlds: 0,
            };
        }

        let algebra = algebra_for(state.contract().declaration());
        // The one law, step 1: fix the common continuation policy BEFORE any
        // world is drawn. Its private tape is independent of every sampled
        // world (§11.4).
        let policy = RandomLegal(SplitMix64::new(
            self.seed ^ nonce.wrapping_mul(0x9e37_79b9_7f4a_7c15),
        ));
        // Exact belief: cells derived from the public prefix, worlds sampled
        // exactly uniformly from the fiber.
        let cells = derive_rule_cells(state);
        let mut world_rng = SplitMix64::new(self.seed.rotate_left(17) ^ nonce);
        let worlds = sample_worlds(&cells, self.worlds_per_decision, &mut world_rng);

        let viewer_team = viewer.team();
        let declaring = state.contract().declaring_team();
        let threshold = state.contract().threshold();
        let hidden = state.hidden_seats();

        // The one law, step 2: average-then-max. For each action, sum the
        // rollout utility over the SAME worlds under a clone of the SAME
        // policy tape; then take the argmax of the totals.
        let mut totals = vec![0i64; legal.len()];
        for world in &worlds {
            for (action_index, &action) in legal.iter().enumerate() {
                let mut hands = [rob_core::DominoSet::empty(); 4];
                hands[viewer.index()] = *state.own_remaining_hand();
                for (i, &seat) in hidden.iter().enumerate() {
                    hands[seat.index()] = world.hidden_hands[i];
                }
                let mut position = RolloutPosition {
                    hands,
                    leader: state.leader(),
                    trick: state.current_trick().to_vec(),
                    points: state.hand_points(),
                };
                position.apply(&algebra, action);
                let final_points = finish_hand(&algebra, position, &mut policy.clone());
                totals[action_index] +=
                    self.utility(final_points, viewer_team, declaring, threshold);
            }
        }
        let best = totals
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(&a.0)))
            .map(|(i, _)| i)
            .expect("nonempty legal set");
        DecisionReport {
            chosen: legal[best],
            legal,
            totals,
            worlds: worlds.len(),
        }
    }

    fn utility(
        &self,
        final_points: [u32; 2],
        viewer_team: rob_core::Team,
        declaring: rob_core::Team,
        threshold: u32,
    ) -> i64 {
        match self.lens {
            UtilityLens::Points => {
                let own = final_points[viewer_team.index()] as i64;
                let other = final_points[viewer_team.opponent().index()] as i64;
                own - other
            }
            UtilityLens::ContractSuccess => {
                let made = final_points[declaring.index()] >= threshold;
                let favorable = if viewer_team == declaring {
                    made
                } else {
                    !made
                };
                if favorable {
                    1
                } else {
                    -1
                }
            }
        }
    }
}
