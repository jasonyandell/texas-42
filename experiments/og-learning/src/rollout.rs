//! Complete-deal rollout: the learner partnership (one shared coefficient
//! vector, two lawful invocations) against the target's fixed seats, driven
//! on walt's rules engine (`policy_search::State` + `legal_plays`), mirroring
//! `policy_search::program::replay_program_traced`. The physical deal only
//! supplies hands; every learner decision consumes its own seat's `Frame`
//! (own hand + public record) - hidden-world predicates cannot be evaluated
//! on that surface at all. Probe expressions (constructor candidates at
//! coefficient zero) are scored alongside without influencing play.

use num_rational::BigRational;
use num_traits::{One, Zero};
use walt::kernel::{Hidden, Kernel};
use walt::policy_search::{HashField, State};
use walt::rules::{legal_plays, Domino, DominoSet, Seat};
use walt::scheme::{step_frame, Budget, CompiledFix, Frame, ObservedPlay, PlayClass};
use walt::solver::adaptive::SlicePolicy;

use crate::actor::RationalActor;
use crate::features::{set_for, ClauseDictionary};
use crate::rng::SplitMix64;
use crate::target::CampaignTarget;

/// Domain constant for the learner's action stream ("OGPL"). Derived only
/// from (deal seed, ply), so mirrored arms share exogenous randomness while
/// their histories agree.
pub const PLAY_DOMAIN: u64 = 0x4F47_504C;

/// Work budget per deal for scheme inference + frame dynamics (declared cap).
pub const DEAL_WORK_BUDGET: u64 = 200_000_000;

pub struct DealRecord {
    pub y: bool,
    /// Per-clause sum over the learner's decisions of the exact score
    /// x_j(a_t) - E_{b~pi} x_j(b); empty unless `collect_scores`.
    pub score_sums: Vec<BigRational>,
    /// The same exact score sums for the probe expressions (coefficient
    /// zero - they never touch play); empty when no probes are passed.
    pub probe_scores: Vec<BigRational>,
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

/// Test-only accessor for an S0 opening frame (used by constructor gates).
pub fn opening_frame_for_test(decl: walt::rules::Decl, hands: &[DominoSet; 4]) -> Frame {
    opening_frame(decl, Seat::S0, hands, Seat::S0).expect("opening frame")
}

fn centered_score(
    set: DominoSet,
    chosen: Domino,
    actions: &[Domino],
    weights: &[BigRational],
    total: &BigRational,
) -> BigRational {
    let mut mass = BigRational::zero();
    for (a, w) in actions.iter().zip(weights) {
        if set.contains(*a) {
            mass += w;
        }
    }
    let x_chosen = if set.contains(chosen) {
        BigRational::one()
    } else {
        BigRational::zero()
    };
    x_chosen - mass / total
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
    probes: &[CompiledFix],
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
    let mut probe_scores = vec![BigRational::zero(); probes.len()];
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
            let tile = actions[chosen];
            if collect_scores || !probes.is_empty() {
                let total: BigRational = weights.iter().sum();
                if collect_scores {
                    for (j, set) in sets.iter().enumerate() {
                        score_sums[j] +=
                            centered_score(*set, tile, &actions, &weights, &total);
                    }
                }
                for (j, probe) in probes.iter().enumerate() {
                    let set = set_for(probe, frame, legal, &mut budget)
                        .map_err(|e| format!("probe {j}: {e}"))?;
                    probe_scores[j] +=
                        centered_score(set, tile, &actions, &weights, &total);
                }
            }
            learner_decisions += 1;
            tile
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
        probe_scores: if probes.is_empty() { Vec::new() } else { probe_scores },
        learner_decisions,
        plies,
        inference_work: budget.spent(),
    })
}

/// Replay one panel deal under `actor` and return every learner decision's
/// (frame, legal set) pair - the versioned probe panel's raw decisions.
/// With all-ones weights the play law is uniform over legal whatever the
/// dictionary, so the panel is fixed across dictionary growth.
pub fn collect_panel_decisions(
    target: &CampaignTarget,
    dict: &ClauseDictionary,
    actor: &RationalActor,
    deal_seed: u64,
) -> Result<Vec<(Frame, DominoSet)>, String> {
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
            out.push((frame.clone(), legal));
            let sets = dict.candidate_sets(frame, legal, &mut budget)?;
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

    #[test]
    fn rollouts_are_deterministic_and_conserve_the_law() {
        let target = CampaignTarget::og_v1();
        let dict = ClauseDictionary::standard().unwrap();
        let actor = RationalActor::uniform(dict.len(), &dict.version);
        for seed in [11u64, 12, 13] {
            let a = play_deal(&target, &dict, &actor, seed, true, &[]).unwrap();
            let b = play_deal(&target, &dict, &actor, seed, true, &[]).unwrap();
            assert_eq!(a.y, b.y);
            assert_eq!(a.score_sums, b.score_sums);
            assert_eq!(a.plies, b.plies);
            assert!(a.learner_decisions >= 1);
        }
    }

    #[test]
    fn probes_never_change_play_and_score_like_dictionary_members() {
        // Law: a probe identical to a dictionary member accumulates exactly
        // that member's score sum, and probing changes nothing about play.
        let target = CampaignTarget::og_v1();
        let dict = ClauseDictionary::standard().unwrap();
        let actor = RationalActor::uniform(dict.len(), &dict.version);
        let master_probe = walt::policy_search::relational::relational_grammar()
            .into_iter()
            .find(|t| t.id == "master")
            .unwrap()
            .guard
            .compile(dict.registry())
            .unwrap();
        for seed in [21u64, 22] {
            let plain = play_deal(&target, &dict, &actor, seed, true, &[]).unwrap();
            let probed =
                play_deal(&target, &dict, &actor, seed, true, &[master_probe.clone()]).unwrap();
            assert_eq!(plain.y, probed.y);
            assert_eq!(plain.plies, probed.plies);
            assert_eq!(plain.score_sums, probed.score_sums);
            // seed index 4 is "master" in relational_grammar order? assert
            // by lookup instead of position:
            let master_idx = dict.ids.iter().position(|i| i == "master").unwrap();
            assert_eq!(probed.probe_scores[0], plain.score_sums[master_idx]);
        }
    }

    #[test]
    fn per_decision_scores_have_zero_expectation_by_construction() {
        // Law: sum_a pi(a) [x_j(a) - E x_j] = 0 exactly, recomputed at one
        // real opening decision from candidate sets.
        let target = CampaignTarget::og_v1();
        let dict = ClauseDictionary::standard().unwrap();
        let actor = RationalActor::uniform(dict.len(), &dict.version);
        let hands = target.deal(7);
        let decl = target.declaration(hands[0]);
        let frame = opening_frame(decl, Seat::S0, &hands, Seat::S0).unwrap();
        let legal = legal_plays(decl, hands[0], None);
        let mut budget = Budget::new(DEAL_WORK_BUDGET);
        let sets = dict.candidate_sets(&frame, legal, &mut budget).unwrap();
        let (actions, weights) = actor.action_weights(legal, &sets);
        let total: BigRational = weights.iter().sum();
        for set in &sets {
            let mut expectation_of_score = BigRational::zero();
            for (a, w) in actions.iter().zip(&weights) {
                expectation_of_score +=
                    (w / &total) * centered_score(*set, *a, &actions, &weights, &total);
            }
            assert!(expectation_of_score.is_zero());
        }
        // PINNED strictness witness: at this opening root some clause
        // separates the legal actions (its set is neither empty nor all).
        let n = legal.len();
        assert!(sets.iter().any(|s| !s.is_empty() && s.len() < n));
    }
}
