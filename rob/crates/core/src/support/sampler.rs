//! The exact count-ratio uniform fiber sampler.
//!
//! Implements Math §7.8 (CELL-10E/F): once the uniform law is *explicitly
//! selected*, sequentially choosing each tile's holder with exact successor
//! counts as integer weights yields an exactly uniform world by the
//! telescoping product — no fiber materialization, no floats (INV-4).

use num_bigint::BigUint;
use num_rational::BigRational;
use num_traits::{One, Zero};

use crate::support::cells::{AbstractCells, AbstractWorld, HIDDEN_SEATS};
use crate::support::count::assignment_count;

/// An exact rational choice source (Exec §16 `ExactRationalChoiceSource`):
/// returns index `i` with exact probability `weights[i] / Σ weights`;
/// requires a positive total. A floating or modulo-biased mechanism must
/// not implement this trait.
pub trait ExactRationalChoiceSource {
    /// Choose an index with the exact displayed probability law.
    fn choose(&mut self, weights: &[BigUint]) -> usize;
}

/// Sample one exactly uniform world of a nonempty fiber (CELL-10F): the
/// count-ratio recursion over successor systems.
pub fn sample_uniform_world(
    cells: &AbstractCells,
    source: &mut dyn ExactRationalChoiceSource,
) -> AbstractWorld {
    assert!(
        assignment_count(cells) > BigUint::zero(),
        "MissingSamplingLaw/EmptySamplingDomain: the fiber must be nonempty"
    );
    let mut world: AbstractWorld = [Vec::new(), Vec::new(), Vec::new()];
    let mut offsets: Vec<usize> = (0..cells.universe()).collect();
    let mut current = cells.clone();
    while current.universe() > 0 {
        let mut weights = Vec::with_capacity(HIDDEN_SEATS);
        let mut successors = Vec::with_capacity(HIDDEN_SEATS);
        for s in 0..HIDDEN_SEATS {
            if current.capacity(s) > 0 && current.possible(s)[0] {
                let successor = current.removal_update(s, 0).expect("allowed successor");
                weights.push(assignment_count(&successor));
                successors.push(Some(successor));
            } else {
                weights.push(BigUint::zero());
                successors.push(None);
            }
        }
        let chosen = source.choose(&weights);
        assert!(weights[chosen] > BigUint::zero(), "zero-weight choice");
        world[chosen].push(offsets[0]);
        offsets.remove(0);
        current = successors[chosen]
            .take()
            .expect("positive weight has a successor");
    }
    world
}

/// The exact probability the count-ratio sampler assigns to one particular
/// world: the telescoping product of successor-count ratios, as an exact
/// rational (CELL-10E/G). Equals `1 / N(C)` for every fiber world.
pub fn world_probability(cells: &AbstractCells, world: &AbstractWorld) -> BigRational {
    let mut probability = BigRational::one();
    let mut current = cells.clone();
    let mut remaining: AbstractWorld = world.clone();
    while current.universe() > 0 {
        let holder = (0..HIDDEN_SEATS)
            .find(|&s| remaining[s].contains(&0))
            .expect("world assigns every pool tile");
        let total = assignment_count(&current);
        let successor = current
            .removal_update(holder, 0)
            .expect("world edge is allowed");
        let part = assignment_count(&successor);
        probability *= BigRational::new(part.into(), total.into());
        current = successor;
        remaining = core::array::from_fn(|s| {
            remaining[s]
                .iter()
                .filter(|&&t| !(s == holder && t == 0))
                .map(|&t| t - 1)
                .collect()
        });
    }
    probability
}
