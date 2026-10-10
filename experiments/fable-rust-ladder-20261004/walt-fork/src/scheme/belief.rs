use std::collections::BTreeMap;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};

use crate::kernel::World;
use crate::rules::Seat;

use super::{error, Answer, Answers, Budget, CompiledFix, Frame, Result};

fn key(world: &World) -> [u32; 4] {
    Seat::ALL.map(|seat| world.hand(seat).bits())
}

/// An explicitly supplied finite physical-world marginal. Duplicate worlds
/// merge by ADDING their weights; zero-weight worlds are absent. This is exact
/// on this measure, not a claim that an empirical measure is the true belief.
#[derive(Clone, Debug)]
pub struct Belief {
    frame: Frame,
    id: String,
    worlds: BTreeMap<[u32; 4], (World, BigRational)>,
    total: BigRational,
}
impl Belief {
    pub fn from_weights(
        frame: Frame,
        id: impl Into<String>,
        entries: impl IntoIterator<Item = (World, BigRational)>,
        max_worlds: usize,
    ) -> Result<Self> {
        let id = id.into();
        if id.trim().is_empty() {
            return Err(error("belief requires a declared identity"));
        }
        let mut worlds: BTreeMap<_, (World, BigRational)> = BTreeMap::new();
        for (world, weight) in entries {
            if weight < BigRational::zero() {
                return Err(error("negative belief weight"));
            }
            if !frame.kernel().contains(&world) {
                return Err(error("belief world is outside frame support"));
            }
            if weight.is_zero() {
                continue;
            }
            let k = key(&world);
            if !worlds.contains_key(&k) && worlds.len() >= max_worlds {
                return Err(error(format!("belief exceeds world cap {max_worlds}")));
            }
            worlds
                .entry(k)
                .or_insert_with(|| (world, BigRational::zero()))
                .1 += weight;
        }
        let total: BigRational = worlds.values().map(|(_, w)| w).sum();
        if total.is_zero() {
            return Err(error(
                "zero-mass belief (conditioning event may be impossible)",
            ));
        }
        Ok(Self {
            frame,
            id,
            worlds,
            total,
        })
    }
    /// Counts the full legal fiber before materializing it. Above-cap supports
    /// are refused, never silently sampled.
    pub fn uniform(frame: Frame, max_worlds: usize) -> Result<Self> {
        let count = frame.kernel().count();
        if count > max_worlds as u128 {
            return Err(error(format!(
                "support has {count} worlds, above cap {max_worlds}"
            )));
        }
        let entries: Vec<_> = frame
            .kernel()
            .worlds()
            .map(|w| (w, BigRational::one()))
            .collect();
        Self::from_weights(frame, "uniform-full-support-v1", entries, max_worlds)
    }
    pub fn frame(&self) -> &Frame {
        &self.frame
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn worlds(&self) -> impl Iterator<Item = (&World, &BigRational)> {
        self.worlds.values().map(|(world, weight)| (world, weight))
    }
    pub fn len(&self) -> usize {
        self.worlds.len()
    }
    pub fn is_empty(&self) -> bool {
        self.worlds.is_empty()
    }
    pub fn total_mass(&self) -> &BigRational {
        &self.total
    }

    /// Caller-declared evidence. This does not assert that the observer can
    /// know the event. Likelihoods must lie in [0,1]. Original belief is unchanged
    /// on failure, and no zero-mass posterior is constructed.
    pub fn condition_likelihood(
        &self,
        posterior_id: impl Into<String>,
        mut likelihood: impl FnMut(&World) -> Result<BigRational>,
    ) -> Result<(Self, BigRational)> {
        let mut entries = Vec::with_capacity(self.len());
        for (world, weight) in self.worlds() {
            let l = likelihood(world)?;
            if l < BigRational::zero() || l > BigRational::one() {
                return Err(error("likelihood must be between zero and one"));
            }
            entries.push((*world, weight * l));
        }
        let posterior = Self::from_weights(self.frame.clone(), posterior_id, entries, self.len())?;
        let probability = &posterior.total / &self.total;
        Ok((posterior, probability))
    }
    /// Conditions on EXISTENCE of an answer, not the number of witnesses.
    pub fn condition(
        &self,
        query: &CompiledFix,
        posterior_id: impl Into<String>,
        budget: &mut Budget,
    ) -> Result<(Self, BigRational)> {
        self.condition_likelihood(posterior_id, |world| {
            Ok(if query.evaluate(&self.frame, world, budget)?.is_empty() {
                BigRational::zero()
            } else {
                BigRational::one()
            })
        })
    }
}

/// These assertions concern positive-mass worlds of the NAMED measure only.
/// Uniform full support gives support-wide statements; a posterior gives
/// belief-almost-sure statements. Event certainty does not imply a known tile.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Certainty {
    pub event_possible: bool,
    pub event_certain: bool,
    pub constant_multiplicity: Option<usize>,
    pub constant_answers: Option<Answers>,
    pub world_functional: bool,
    pub identity: Option<Answer>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuerySummary {
    pub frame: Frame,
    pub query_identity: String,
    pub belief_identity: String,
    pub positive_worlds: usize,
    pub total_mass: BigRational,
    pub event_mass: BigRational,
    /// Each answer gets a world's weight ONCE if present. These masses may
    /// sum above total_mass: they are presence events, not a selector law.
    pub answer_presence_mass: BTreeMap<Answer, BigRational>,
    /// Distribution of the entire answer SET; hidden witness choices disappear.
    pub answer_set_mass: BTreeMap<Answers, BigRational>,
    pub certainty: Certainty,
}

impl CompiledFix {
    pub fn summarize(&self, belief: &Belief, budget: &mut Budget) -> Result<QuerySummary> {
        let mut event_mass = BigRational::zero();
        let mut presence: BTreeMap<Answer, BigRational> = BTreeMap::new();
        let mut sets: BTreeMap<Answers, BigRational> = BTreeMap::new();
        for (world, weight) in belief.worlds() {
            let answers = self.evaluate(belief.frame(), world, budget)?;
            if !answers.is_empty() {
                event_mass += weight;
            }
            for answer in &answers {
                *presence.entry(answer.clone()).or_default() += weight;
            }
            *sets.entry(answers).or_default() += weight;
        }
        let first = sets.keys().next().expect("positive belief");
        let constant_answers = (sets.len() == 1).then(|| first.clone());
        let constant_multiplicity = sets
            .keys()
            .all(|s| s.len() == first.len())
            .then_some(first.len());
        let world_functional = constant_multiplicity == Some(1);
        let identity = if world_functional && sets.len() == 1 {
            first.first().cloned()
        } else {
            None
        };
        let certainty = Certainty {
            event_possible: !event_mass.is_zero(),
            event_certain: event_mass == belief.total,
            constant_multiplicity,
            constant_answers,
            world_functional,
            identity,
        };
        Ok(QuerySummary {
            frame: belief.frame.clone(),
            query_identity: self.identity(),
            belief_identity: belief.id.clone(),
            positive_worlds: belief.len(),
            total_mass: belief.total.clone(),
            event_mass,
            answer_presence_mass: presence,
            answer_set_mass: sets,
            certainty,
        })
    }

    /// Revives the archived finite-domain counterexample discipline at the
    /// answer-relation boundary. This checks only the named measure's support.
    pub fn compare(
        &self,
        other: &Self,
        belief: &Belief,
        comparison: Comparison,
        budget: &mut Budget,
    ) -> Result<Option<Difference>> {
        if comparison == Comparison::Answers && self.outputs() != other.outputs() {
            return Err(error(
                "answer comparison requires the same ordered output interface",
            ));
        }
        for (world, _) in belief.worlds() {
            let left = self.evaluate(belief.frame(), world, budget)?;
            let right = other.evaluate(belief.frame(), world, budget)?;
            let differs = match comparison {
                Comparison::Existence => left.is_empty() != right.is_empty(),
                Comparison::Answers => left != right,
            };
            if differs {
                return Ok(Some(Difference {
                    world: *world,
                    left,
                    right,
                }));
            }
        }
        Ok(None)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comparison {
    Existence,
    Answers,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Difference {
    pub world: World,
    pub left: Answers,
    pub right: Answers,
}

/// Explicit conventions. Neither is implicit in Scheme semantics, nor is
/// an offline selector automatically executable by a seat.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Selection {
    LexicographicFirst,
    UniformWithinWorld,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectionLaw {
    pub selection: Selection,
    /// Unconditional probabilities: sum = event_probability, not always one.
    pub probabilities: BTreeMap<Answer, BigRational>,
    pub no_answer_probability: BigRational,
}
impl QuerySummary {
    pub fn event_probability(&self) -> BigRational {
        &self.event_mass / &self.total_mass
    }
    pub fn presence_probability(&self, answer: &Answer) -> BigRational {
        self.answer_presence_mass
            .get(answer)
            .map_or_else(BigRational::zero, |m| m / &self.total_mass)
    }
    pub fn select(&self, selection: Selection) -> SelectionLaw {
        let mut probabilities: BTreeMap<Answer, BigRational> = BTreeMap::new();
        let mut no_answer_probability = BigRational::zero();
        for (answers, weight) in &self.answer_set_mass {
            let p = weight / &self.total_mass;
            if answers.is_empty() {
                no_answer_probability += p;
                continue;
            }
            match selection {
                Selection::LexicographicFirst => {
                    *probabilities
                        .entry(answers.first().expect("nonempty").clone())
                        .or_default() += p;
                }
                Selection::UniformWithinWorld => {
                    let share = p / BigRational::from_integer(BigInt::from(answers.len()));
                    for answer in answers {
                        *probabilities.entry(answer.clone()).or_default() += &share;
                    }
                }
            }
        }
        SelectionLaw {
            selection,
            probabilities,
            no_answer_probability,
        }
    }
}
