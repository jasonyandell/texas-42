//! Complete-deal rollout: the learner partnership (one shared coefficient
//! vector, two lawful invocations) against the target's fixed seats, driven
//! on walt's rules engine (`policy_search::State` + `legal_plays`), mirroring
//! `policy_search::program::replay_program_traced`. The physical deal only
//! supplies hands; every learner decision consumes its own seat's `Frame`
//! (own hand + public record) - hidden-world predicates cannot be evaluated
//! on that surface at all.

use num_rational::BigRational;
use num_traits::Zero;
use walt::kernel::{Hidden, Kernel};
use walt::policy_search::{HashField, State};
use walt::rules::{legal_plays, Domino, DominoSet, Seat};
use walt::scheme::{step_frame, Budget, Frame, ObservedPlay, PlayClass};
use walt::solver::adaptive::SlicePolicy;

use crate::actor::RationalActor;
use crate::features::ClauseDictionary;
use crate::rng::SplitMix64;
use crate::target::CampaignTarget;

/// Domain constant for the learner's action stream ("OGPL"). Derived only
/// from (deal seed, ply), so mirrored arms share exogenous randomness while
/// their histories agree.
pub const PLAY_DOMAIN: u64 = 0x4F47_504C;

/// Work budget per deal for scheme inference + frame dynamics (declared cap).
pub const DEAL_WORK_BUDGET: u64 = 50_000_000;

pub struct DealRecord {
    pub y: bool,
    /// Per-clause sum over the learner's decisions of the exact score
    /// x_j(a_t) - E_{b~pi} x_j(b); empty unless `collect_scores`.
    pub score_sums: Vec<BigRational>,
    pub learner_decisions: u32,
    pub plies: u32,
    pub inference_work: u64,
}

fn observation(frame: &Frame, actor: Seat, tile: Domino) -> ObservedPlay {
    let class = match frame.led_context() {
        None => PlayClass::Lead,
        Some(q) if frame.kernel().decl().follows(tile, q) => PlayClass::Follow(q),
        Some(q) => PlayClass::Slough(q),
    };
    ObservedPlay { actor, tile, class }
}

fn opening_frame(
    decl: walt::rules::Decl,
    viewer: Seat,
    hands: &[DominoSet; 4],
    leader: Seat,
) -> Result<Frame, String> {
    let hidden: Vec<Hidden> = Seat::ALL
        .iter()
        .filter(|s| **s != viewer)
        .map(|s| Hidden {
            seat: *s,
            capacity: 7,
            voids: walt::rules::ContextSet::EMPTY,
        })
        .collect();
    let kernel = Kernel::new(
        decl,
        viewer,
        hands[viewer.index()],
        DominoSet::FULL.difference(hands[viewer.index()]),
        [hidden[0], hidden[1], hidden[2]],
    )
    .map_err(|e| e.to_string())?;
    Frame::new(kernel, leader, Vec::new(), DominoSet::EMPTY).map_err(|e| e.to_string())
}

/// Play one complete deal of the target with `actor` on the learner seats.
/// Stops as soon as make/set is decided (the utility is already determined
/// and later score terms only add variance).
pub fn play_deal(
    target: &CampaignTarget,
    dict: &ClauseDictionary,
    actor: &RationalActor,
    deal_seed: u64,
    collect_scores: bool,
) -> Result<DealRecord, String> {
    let hands = target.deal(deal_seed);
    let decl = target.declaration(hands[target.bidder.index()]);
    let position = target.opening_root(decl);
    let field = HashField;

    let mut frames: Vec<(Seat, Frame)> = Vec::new();
    for seat in target.learner_seats {
        frames.push((seat, opening_frame(decl, seat, &hands, position.leader)?));
    }

    let mut budget = Budget::new(DEAL_WORK_BUDGET);
    let mut state = State::from_root(&position);
    let mut score_sums = vec![BigRational::zero(); dict.len()];
    let mut learner_decisions = 0u32;
    let mut plies = 0u32;

    let outcome = loop {
        if let Some(decided) = state.success(&position) {
            break decided;
        }
        if state.played.len() >= 28 {
            if state.banked.iter().sum::<u32>() != 42 {
                return Err("score conservation failed at 28 plies".into());
            }
            break state.banked[position.declaring_team.index()] >= position.bid;
        }
        let seat = state.actor();
        let hand = hands[seat.index()].difference(state.played);
        let led = state.prefix.first().map(|d| decl.led_context(*d));
        let legal = legal_plays(decl, hand, led);
        if legal.is_empty() {
            return Err("empty legal set".into());
        }

        let tile = if let Some((_, frame)) = frames.iter().find(|(s, _)| *s == seat) {
            if frame.next_actor() != Some(seat) {
                return Err("frame/state actor disagreement".into());
            }
            let sets = dict.candidate_sets(frame, legal, &mut budget)?;
            let (actions, weights) = actor.action_weights(legal, &sets);
            let chosen = if actions.len() == 1 {
                0
            } else {
                let mut stream =
                    SplitMix64::new(deal_seed ^ PLAY_DOMAIN).derive(u64::from(plies));
                stream.sample_rational(&weights)
            };
            if collect_scores {
                let total: BigRational = weights.iter().sum();
                for (j, set) in sets.iter().enumerate() {
                    let mut mass = BigRational::zero();
                    for (a, w) in actions.iter().zip(&weights) {
                        if set.contains(*a) {
                            mass += w;
                        }
                    }
                    let expected = mass / &total;
                    let x_chosen = if set.contains(actions[chosen]) {
                        BigRational::from_integer(1.into())
                    } else {
                        BigRational::zero()
                    };
                    score_sums[j] += x_chosen - expected;
                }
            }
            learner_decisions += 1;
            actions[chosen]
        } else {
            field.choose(decl, hand, legal, &state.record(&position))
        };

        if !legal.contains(tile) {
            return Err("illegal action selected".into());
        }
        for (_, frame) in frames.iter_mut() {
            *frame = step_frame(frame, observation(frame, seat, tile), &mut budget)
                .map_err(|e| e.to_string())?;
        }
        state = state.step(decl, tile);
        plies += 1;
    };

    Ok(DealRecord {
        y: outcome,
        score_sums: if collect_scores { score_sums } else { Vec::new() },
        learner_decisions,
        plies,
        inference_work: budget.spent(),
    })
}

/// One clause's candidate set at one panel decision, as raw bits plus a
/// triviality flag (empty, or the whole legal set - action-independent).
pub struct PanelSet {
    pub bits: u32,
    pub trivial: bool,
}

/// Replay one panel deal under `actor` and record every learner decision's
/// per-clause candidate sets (for the versioned dedup probe panel).
pub fn panel_decision_sets(
    target: &CampaignTarget,
    dict: &ClauseDictionary,
    actor: &RationalActor,
    deal_seed: u64,
) -> Result<Vec<Vec<PanelSet>>, String> {
    let hands = target.deal(deal_seed);
    let decl = target.declaration(hands[target.bidder.index()]);
    let position = target.opening_root(decl);
    let field = HashField;
    let mut frames: Vec<(Seat, Frame)> = Vec::new();
    for seat in target.learner_seats {
        frames.push((seat, opening_frame(decl, seat, &hands, position.leader)?));
    }
    let mut budget = Budget::new(DEAL_WORK_BUDGET);
    let mut state = State::from_root(&position);
    let mut out = Vec::new();
    let mut plies = 0u32;
    while state.played.len() < 28 && state.success(&position).is_none() {
        let seat = state.actor();
        let hand = hands[seat.index()].difference(state.played);
        let led = state.prefix.first().map(|d| decl.led_context(*d));
        let legal = legal_plays(decl, hand, led);
        let tile = if let Some((_, frame)) = frames.iter().find(|(s, _)| *s == seat) {
            let sets = dict.candidate_sets(frame, legal, &mut budget)?;
            out.push(
                sets.iter()
                    .map(|s| PanelSet {
                        bits: s.bits(),
                        trivial: s.is_empty() || *s == legal,
                    })
                    .collect(),
            );
            let (actions, weights) = actor.action_weights(legal, &sets);
            let chosen = if actions.len() == 1 {
                0
            } else {
                let mut stream =
                    SplitMix64::new(deal_seed ^ PLAY_DOMAIN).derive(u64::from(plies));
                stream.sample_rational(&weights)
            };
            actions[chosen]
        } else {
            field.choose(decl, hand, legal, &state.record(&position))
        };
        for (_, frame) in frames.iter_mut() {
            *frame = step_frame(frame, observation(frame, seat, tile), &mut budget)
                .map_err(|e| e.to_string())?;
        }
        state = state.step(decl, tile);
        plies += 1;
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::One;

    #[test]
    fn rollouts_are_deterministic_and_conserve_the_law() {
        let target = CampaignTarget::og_v1();
        let dict = ClauseDictionary::standard().unwrap();
        let actor = RationalActor::uniform(dict.len(), &dict.version);
        for seed in [11u64, 12, 13] {
            let a = play_deal(&target, &dict, &actor, seed, true).unwrap();
            let b = play_deal(&target, &dict, &actor, seed, true).unwrap();
            assert_eq!(a.y, b.y);
            assert_eq!(a.score_sums, b.score_sums);
            assert_eq!(a.plies, b.plies);
            assert!(a.learner_decisions >= 1);
        }
    }

    #[test]
    fn per_decision_scores_have_zero_expectation_by_construction() {
        // Law: sum_a pi(a) [x_j(a) - E x_j] = 0 exactly. We check the
        // aggregate consequence: with ALL-ONES weights, every clause's score
        // is x_j(a) - |F_j ∩ legal|/|legal|; summed against the uniform law
        // it vanishes. Here we assert the mechanical version at one real
        // decision by recomputing from candidate sets.
        let target = CampaignTarget::og_v1();
        let dict = ClauseDictionary::standard().unwrap();
        let actor = RationalActor::uniform(dict.len(), &dict.version);
        let hands = target.deal(7);
        let decl = target.declaration(hands[0]);
        let position = target.opening_root(decl);
        let frame = opening_frame(decl, Seat::S0, &hands, position.leader).unwrap();
        let legal = legal_plays(decl, hands[0], None);
        let mut budget = Budget::new(DEAL_WORK_BUDGET);
        let sets = dict.candidate_sets(&frame, legal, &mut budget).unwrap();
        let (actions, weights) = actor.action_weights(legal, &sets);
        let total: BigRational = weights.iter().sum();
        for set in &sets {
            let mut expectation_of_score = BigRational::zero();
            for (a, w) in actions.iter().zip(&weights) {
                let x = if set.contains(*a) {
                    BigRational::one()
                } else {
                    BigRational::zero()
                };
                let mut mass = BigRational::zero();
                for (b, wb) in actions.iter().zip(&weights) {
                    if set.contains(*b) {
                        mass += wb;
                    }
                }
                expectation_of_score += (w / &total) * (x - mass / &total);
            }
            assert!(expectation_of_score.is_zero());
        }
        // PINNED strictness witness: at this opening root some clause
        // separates the legal actions (its set is neither empty nor all).
        let n = legal.len();
        assert!(sets
            .iter()
            .any(|s| !s.is_empty() && s.len() < n));
    }
}
