//! Finite, extensional Scheme dynamics (v0.4 §6).
//!
//! This module rebuilds the inherited kernel after one typed public play.  It
//! is deliberately an executable finite transform, not a `step` compiler or a
//! compression claim.  [`Belief`] contains only the physical-world marginal;
//! callers whose field policy has latent state must augment that state outside
//! this type and supply the resulting likelihood explicitly.

use num_rational::BigRational;
use num_traits::{One, Zero};

use crate::kernel::{Kernel, World, HIDDEN_SEATS};
use crate::rules::{legal_plays, Context, Domino, DominoSet, Seat, Trick};

use super::{error, Answer, Answers, Belief, Budget, Frame, Result};

/// The public classification carried by a play observation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayClass {
    Lead,
    Follow(Context),
    Slough(Context),
}

/// Actor-attributed, context-typed public evidence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ObservedPlay {
    pub actor: Seat,
    pub tile: Domino,
    pub class: PlayClass,
}

/// Exact result of a physical-marginal belief step.
#[derive(Clone, Debug)]
pub struct BeliefStep {
    pub belief: Belief,
    /// Prior probability of the observation under the supplied likelihood.
    pub evidence_probability: BigRational,
}

fn typed_context(frame: &Frame, observation: ObservedPlay) -> Result<Option<Context>> {
    if frame.next_actor() != Some(observation.actor) {
        return Err(error(format!(
            "observed actor {} is not the next actor {:?}",
            observation.actor,
            frame.next_actor()
        )));
    }
    let led = frame.led_context();
    match (led, observation.class) {
        (None, PlayClass::Lead) => Ok(None),
        (Some(q), PlayClass::Follow(r))
            if q == r && frame.kernel().decl().follows(observation.tile, q) =>
        {
            Ok(Some(q))
        }
        (Some(q), PlayClass::Slough(r))
            if q == r && !frame.kernel().decl().follows(observation.tile, q) =>
        {
            Ok(Some(q))
        }
        _ => Err(error(
            "observed play classification or context disagrees with the public frame",
        )),
    }
}

fn successor_kernel(frame: &Frame, observation: ObservedPlay) -> Result<Kernel> {
    let mut viewer_hand = frame.kernel().viewer_hand();
    let mut pool = frame.kernel().pool();
    let mut hidden = *frame.kernel().hidden();
    if observation.actor == frame.kernel().viewer() {
        if !viewer_hand.remove(observation.tile) {
            return Err(error("viewer does not hold the observed tile"));
        }
    } else {
        if !pool.remove(observation.tile) {
            return Err(error("observed hidden tile is outside the hidden pool"));
        }
        let slot = hidden
            .iter_mut()
            .find(|h| h.seat == observation.actor)
            .expect("kernel represents every hidden chair");
        if slot.capacity == 0 {
            return Err(error("observed actor has no live tiles"));
        }
        slot.capacity -= 1;
        if let PlayClass::Slough(q) = observation.class {
            slot.voids.insert(q);
        }
    }
    Kernel::new(
        frame.kernel().decl(),
        frame.kernel().viewer(),
        viewer_hand,
        pool,
        hidden,
    )
    .map_err(|e| error(format!("invalid successor kernel: {e}")))
}

/// Apply the typed play to the public frame and exact support.
///
/// The play must be legal in at least one predecessor world.  Hidden-holder
/// forcing and slough constraints are represented in the rebuilt kernel; its
/// world iterator performs the matching-supported reduction extensionally.
pub fn step_frame(frame: &Frame, observation: ObservedPlay, budget: &mut Budget) -> Result<Frame> {
    typed_context(frame, observation)?;
    if observation.actor == frame.kernel().viewer() {
        if !legal_plays(
            frame.kernel().decl(),
            frame.kernel().viewer_hand(),
            frame.led_context(),
        )
        .contains(observation.tile)
        {
            return Err(error("viewer play is illegal in the predecessor frame"));
        }
    } else {
        let slot = frame
            .kernel()
            .hidden()
            .iter()
            .position(|h| h.seat == observation.actor)
            .expect("kernel represents every hidden chair");
        if !frame.kernel().allowed(slot).contains(observation.tile) {
            return Err(error(
                "hidden play contradicts the actor's predecessor constraints",
            ));
        }
    }
    budget.spend(1)?;
    let kernel = successor_kernel(frame, observation)?;
    if kernel.count() == 0 {
        return Err(error("observation has empty legal predecessor support"));
    }
    let mut prefix = frame.prefix().to_vec();
    prefix.push(observation.tile);
    let mut leader = frame.leader();
    if prefix.len() == 4 {
        let dominoes: [Domino; 4] = prefix.as_slice().try_into().expect("four-play prefix");
        leader = Trick::new(leader, dominoes)
            .map_err(|_| error("a trick cannot contain a repeated tile"))?
            .winner(kernel.decl());
        prefix.clear();
    }
    Frame::new(
        kernel,
        leader,
        prefix,
        frame.played().union(DominoSet::single(observation.tile)),
    )
}

/// Whether a concrete predecessor is in the typed transition's domain.
pub fn play_is_legal(frame: &Frame, world: &World, observation: ObservedPlay) -> bool {
    typed_context(frame, observation).is_ok()
        && frame.kernel().contains(world)
        && legal_plays(
            frame.kernel().decl(),
            world.hand(observation.actor),
            frame.led_context(),
        )
        .contains(observation.tile)
}

/// Push one legal predecessor world through the observation.
pub fn step_world(frame: &Frame, world: &World, observation: ObservedPlay) -> Result<World> {
    typed_context(frame, observation)?;
    if !play_is_legal(frame, world, observation) {
        return Err(error("world is outside the observation's legal domain"));
    }
    let kernel = successor_kernel(frame, observation)?;
    let mut hidden_hands = [DominoSet::EMPTY; HIDDEN_SEATS];
    for (cell, h) in hidden_hands.iter_mut().zip(kernel.hidden()) {
        *cell = world
            .hand(h.seat)
            .difference(DominoSet::single(observation.tile));
    }
    let next = kernel.world(hidden_hands);
    if !kernel.contains(&next) {
        return Err(error("successor world violates the updated exact support"));
    }
    Ok(next)
}

/// Condition by legality, multiply by an explicit policy likelihood, then
/// push the physical world marginal forward. As elsewhere in [`Belief`], the
/// stored exact weights need not sum to one; probabilities divide by total mass.
///
/// `likelihood` must be in `[0,1]`.  Use one for a forced move, or for a
/// viewer intervention whose randomization is independent of hidden deal.
pub fn step_belief(
    prior: &Belief,
    observation: ObservedPlay,
    posterior_id: impl Into<String>,
    max_worlds: usize,
    budget: &mut Budget,
    mut likelihood: impl FnMut(&World) -> Result<BigRational>,
) -> Result<BeliefStep> {
    typed_context(prior.frame(), observation)?;
    budget.spend(prior.len() as u64)?;
    let next_frame = step_frame(prior.frame(), observation, budget)?;
    let mut entries = Vec::new();
    let mut evidence_mass = BigRational::zero();
    for (world, weight) in prior.worlds() {
        if !play_is_legal(prior.frame(), world, observation) {
            continue;
        }
        let l = likelihood(world)?;
        if l < BigRational::zero() || l > BigRational::one() {
            return Err(error("likelihood must be between zero and one"));
        }
        let weighted = weight * &l;
        if weighted.is_zero() {
            continue;
        }
        evidence_mass += &weighted;
        entries.push((step_world(prior.frame(), world, observation)?, weighted));
    }
    if evidence_mass.is_zero() {
        return Err(error("zero-mass observation under supplied likelihood"));
    }
    let evidence_probability = &evidence_mass / prior.total_mass();
    let belief = Belief::from_weights(next_frame, posterior_id, entries, max_worlds)?;
    Ok(BeliefStep {
        belief,
        evidence_probability,
    })
}

/// Convenience likelihood for forced moves and deal-independent viewer acts.
pub fn unit_likelihood(_: &World) -> Result<BigRational> {
    Ok(BigRational::one())
}

/// Rigid output transport.  Domino, chair, and context values retain identity.
pub fn transport_answers(prior: &Answers) -> Answers {
    prior.clone()
}

/// Identity-aware comparison of transported prior and freshly evaluated answers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnswerDynamics {
    pub transported: Answers,
    pub persistent: Answers,
    pub extinct: Answers,
    pub born: Answers,
}

pub fn compare_answers(prior: &Answers, fresh: &Answers) -> AnswerDynamics {
    let transported = transport_answers(prior);
    let persistent = transported.intersection(fresh).cloned().collect();
    let extinct = transported.difference(fresh).cloned().collect();
    let born = fresh.difference(&transported).cloned().collect();
    AnswerDynamics {
        transported,
        persistent,
        extinct,
        born,
    }
}

/// Backward preimage over an explicitly supplied finite predecessor relation.
/// This is analyst hindsight filtering; it does not alter an earlier player policy.
pub fn anchor_back(
    frame: &Frame,
    observation: ObservedPlay,
    prior: impl IntoIterator<Item = (World, Answer)>,
    later: &[(World, Answer)],
    budget: &mut Budget,
) -> Result<Vec<(World, Answer)>> {
    typed_context(frame, observation)?;
    let prior: Vec<_> = prior.into_iter().collect();
    budget.spend(prior.len() as u64)?;
    let mut out = Vec::new();
    for (world, answer) in prior {
        if play_is_legal(frame, &world, observation) {
            let successor = step_world(frame, &world, observation)?;
            if later.iter().any(|(w, a)| *w == successor && *a == answer)
                && !out.iter().any(|(w, a)| *w == world && *a == answer) {
                out.push((world, answer));
            }
        }
    }
    Ok(out)
}
