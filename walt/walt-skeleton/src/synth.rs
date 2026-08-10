//! The static-purity synthesis search (v0.4 §12.9, steps 1-4, at S4
//! scope).
//!
//! §12.9's loop: compute exact response labels, evaluate registered atoms,
//! find a minimum-cost descriptor satisfying the target condition, and emit
//! counterexample pairs on failure. At S4 the candidate language is the
//! `atoms` registry and cost is atom count, so the minimum-cost search is
//! an exhaustive size-ordered subset enumeration -- exact, and honest about
//! its ceiling (`max_size`). Counterexample-guided refinement at full
//! generality (steps 5-7) is S5 factory work.
//!
//! Everything returned is exploratory-tier computed evidence about one
//! kernel; pins made from it are regression pins, never axioms.

use std::collections::BTreeMap;

use walt_core::Domino;
use walt_kernel::Kernel;

use crate::atoms::{Atom, AtomDescriptor, AtomState};
use crate::skeleton::ControlSkeleton;

/// The S4 registry over one kernel: a team fact and a holder fact for every
/// hidden-pool tile, plus the beater-control relation of the valued tile.
/// `2 * |pool| + 1` atoms.
pub fn registry(kernel: &Kernel, valued: Domino) -> Vec<Atom> {
    let mut out: Vec<Atom> = kernel.pool().iter().map(Atom::TeamOf).collect();
    out.extend(kernel.pool().iter().map(Atom::HolderOf));
    out.push(Atom::BeaterCounts(valued));
    out
}

/// Densifies any exact labeling into class ids (first-seen order) and the
/// class count. The ids are what `sound_search` consumes; the original
/// labels stay with the caller.
pub fn class_ids<Y: Ord>(labels: &[Y]) -> (Vec<usize>, usize) {
    let mut seen: BTreeMap<&Y, usize> = BTreeMap::new();
    let ids = labels
        .iter()
        .map(|l| {
            let next = seen.len();
            *seen.entry(l).or_insert(next)
        })
        .collect();
    (ids, seen.len())
}

/// All minimal (by atom count) purpose-sound subsets found at or below the
/// search ceiling.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MinimalSound {
    pub size: usize,
    /// Every sound subset of that size, in registry-index order.
    pub solutions: Vec<Vec<Atom>>,
    /// Descriptor cell counts, aligned with `solutions`.
    pub cells: Vec<usize>,
}

/// The outcome of one size-ordered exhaustive search.
#[derive(Clone, Debug)]
pub struct SoundSearch {
    pub worlds: usize,
    pub registry_size: usize,
    /// The search ceiling: every subset of size <= max_size was tested.
    pub max_size: usize,
    /// `None` means NO subset within the ceiling is purpose-sound for the
    /// target -- a §12.3-style vocabulary ceiling, reportable as such.
    pub minimal: Option<MinimalSound>,
}

/// Exhaustive minimum-size purity search of registry subsets against one
/// exact target labeling (`target[i]` labels the i-th world in
/// `Kernel::worlds` order). Static axis only: the subset is evaluated at
/// the root (§12.1); its update behavior is graded separately by the
/// lumpability checker.
pub fn sound_search(
    kernel: &Kernel,
    registry: &[Atom],
    max_size: usize,
    target: &[usize],
) -> SoundSearch {
    // One latent read per world: the full registry vector, projected per
    // subset thereafter.
    let full = AtomDescriptor::new(kernel, false, registry.to_vec());
    let vectors: Vec<Vec<AtomState>> = kernel
        .worlds()
        .map(|w| full.init(kernel, &w).atoms)
        .collect();
    assert_eq!(
        vectors.len(),
        target.len(),
        "one target label per fiber world"
    );

    let sound_cells = |subset: &[usize]| -> Option<usize> {
        let mut cells: BTreeMap<Vec<AtomState>, usize> = BTreeMap::new();
        for (vector, &label) in vectors.iter().zip(target) {
            let key: Vec<AtomState> = subset.iter().map(|&i| vector[i]).collect();
            match cells.get(&key) {
                None => {
                    cells.insert(key, label);
                }
                Some(&first) => {
                    if first != label {
                        return None;
                    }
                }
            }
        }
        Some(cells.len())
    };

    for size in 0..=max_size.min(registry.len()) {
        let mut solutions = Vec::new();
        let mut cells = Vec::new();
        let mut subset = vec![0usize; size];
        combinations(registry.len(), size, 0, 0, &mut subset, &mut |s| {
            if let Some(n) = sound_cells(s) {
                solutions.push(s.iter().map(|&i| registry[i]).collect());
                cells.push(n);
            }
        });
        if !solutions.is_empty() {
            return SoundSearch {
                worlds: vectors.len(),
                registry_size: registry.len(),
                max_size,
                minimal: Some(MinimalSound {
                    size,
                    solutions,
                    cells,
                }),
            };
        }
    }
    SoundSearch {
        worlds: vectors.len(),
        registry_size: registry.len(),
        max_size,
        minimal: None,
    }
}

fn combinations(
    n: usize,
    k: usize,
    depth: usize,
    start: usize,
    subset: &mut Vec<usize>,
    f: &mut impl FnMut(&[usize]),
) {
    if depth == k {
        f(subset);
        return;
    }
    for i in start..=(n - (k - depth)) {
        subset[depth] = i;
        combinations(n, k, depth + 1, i + 1, subset, f);
    }
}
