//! The static soundness checker (v0.4 §12.1).
//!
//! On a finite domain `X` with exact target response `R*: X -> Y` and
//! descriptor `D: X -> Z`, `D` is purpose-sound exactly when `D(x) = D(y)`
//! implies `R*(x) = R*(y)`, i.e. exactly when `R*` factors as
//! `R* = R-bar o D` for a (then unique) `R-bar: im D -> im R*`.
//!
//! The domain instantiated here is the kernel's root latent states -- the
//! whole void-constrained fiber, enumerated exhaustively -- and `D` is the
//! skeleton's root evaluation (`init`). When purity fails the checker emits
//! a §12.9 step-4 witness: a pair of worlds merged by the descriptor but
//! separated by the target.

use std::collections::BTreeMap;

use walt_kernel::{Kernel, World};

use crate::skeleton::ControlSkeleton;

/// A §12.9 counterexample pair: two worlds in one descriptor cell with
/// different target responses.
#[derive(Clone, Debug)]
pub struct PurityCounterexample<Z, Y> {
    pub cell: Z,
    pub world_a: World,
    pub label_a: Y,
    pub world_b: World,
    pub label_b: Y,
}

/// The exhaustive factorization verdict `|X| -> |im D| -> |im R*|`.
#[derive(Clone, Debug)]
pub struct SoundnessReport<Z, Y> {
    /// `|X|`: the fiber size, asserted against the counting DP.
    pub worlds: usize,
    /// `|im D|`: distinct descriptor cells on the domain.
    pub cells: usize,
    /// `|im R*|`: distinct target responses on the domain.
    pub responses: usize,
    /// The first §12.9 witness found, or `None` when `R* = R-bar o D`.
    pub counterexample: Option<PurityCounterexample<Z, Y>>,
}

impl<Z, Y> SoundnessReport<Z, Y> {
    /// Whether the target factors through the descriptor (§12.1).
    pub fn is_sound(&self) -> bool {
        self.counterexample.is_none()
    }
}

/// Checks §12.1 factorization exhaustively over the kernel's fiber. The
/// `target` closure is invoked once per world, in `Kernel::worlds` order.
/// The full census (cells, responses) is completed even after a
/// counterexample is found -- the counts are part of the verdict.
pub fn check_soundness<S: ControlSkeleton, Y: Ord + Clone>(
    kernel: &Kernel,
    skeleton: &S,
    mut target: impl FnMut(&World) -> Y,
) -> SoundnessReport<S::State, Y> {
    let mut cells: BTreeMap<S::State, (World, Y)> = BTreeMap::new();
    let mut responses: BTreeMap<Y, ()> = BTreeMap::new();
    let mut worlds = 0usize;
    let mut counterexample = None;
    for world in kernel.worlds() {
        worlds += 1;
        let label = target(&world);
        responses.entry(label.clone()).or_insert(());
        let cell = skeleton.init(kernel, &world);
        match cells.get(&cell) {
            None => {
                cells.insert(cell, (world, label));
            }
            Some((first, first_label)) => {
                if counterexample.is_none() && *first_label != label {
                    counterexample = Some(PurityCounterexample {
                        cell: cell.clone(),
                        world_a: *first,
                        label_a: first_label.clone(),
                        world_b: world,
                        label_b: label,
                    });
                }
            }
        }
    }
    assert_eq!(
        worlds as u128,
        kernel.count(),
        "the soundness domain is the whole fiber"
    );
    SoundnessReport {
        worlds,
        cells: cells.len(),
        responses: responses.len(),
        counterexample,
    }
}
