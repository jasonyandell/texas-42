//! The current-remainder fiber `Phi(C)` (v0.4 §2.1): exact enumeration and
//! the exact integer counting DP that also drives uniform sampling.
//!
//! The DP groups pool tiles by which hidden slots may hold them. Within a
//! group the tiles are interchangeable as far as the constraints go, so the
//! whole fiber is counted by a multinomial per group -- no enumeration.

use walt_core::{Domino, DominoSet};

use crate::kernel::{Hidden, Kernel, World, HIDDEN_SEATS, MAX_CAPACITY};

const CAP_DIM: usize = MAX_CAPACITY + 1;

type Layer = [[[u128; CAP_DIM]; CAP_DIM]; CAP_DIM];

pub(crate) fn binomial(n: usize, k: usize) -> u128 {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    let mut out: u128 = 1;
    for i in 0..k {
        out = out * (n - i) as u128 / (i as u128 + 1);
    }
    out
}

/// Tiles grouped by allowed-slot pattern, with the layered completion counts.
///
/// `layers[i][r0][r1][r2]` is the number of ways to place `groups[i..]` so
/// that slot `j` receives exactly `r_j` more tiles.
pub struct FiberDp {
    groups: Vec<(u8, Vec<Domino>)>,
    layers: Vec<Box<Layer>>,
    capacities: [usize; HIDDEN_SEATS],
}

impl core::fmt::Debug for FiberDp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("FiberDp")
            .field("groups", &self.groups)
            .field("capacities", &self.capacities)
            .field("count", &self.count())
            .finish()
    }
}

/// All `(c0, c1, c2)` summing to `n` with `c_j = 0` for slots outside the
/// pattern.
pub(crate) fn splits(n: usize, pattern: u8) -> Vec<[usize; HIDDEN_SEATS]> {
    let allows = |j: usize| pattern & (1u8 << j) != 0;
    let mut out = Vec::new();
    for c0 in 0..=(if allows(0) { n } else { 0 }) {
        for c1 in 0..=(if allows(1) { n - c0 } else { 0 }) {
            let c2 = n - c0 - c1;
            if c2 > 0 && !allows(2) {
                continue;
            }
            out.push([c0, c1, c2]);
        }
    }
    out
}

pub(crate) fn multinomial(n: usize, c: [usize; HIDDEN_SEATS]) -> u128 {
    binomial(n, c[0]) * binomial(n - c[0], c[1])
}

impl FiberDp {
    pub fn new(kernel: &Kernel) -> FiberDp {
        let allowed: [DominoSet; HIDDEN_SEATS] = core::array::from_fn(|i| kernel.allowed(i));
        let mut by_pattern: Vec<(u8, Vec<Domino>)> = Vec::new();
        for d in kernel.pool().iter() {
            let mut pattern = 0u8;
            for (i, a) in allowed.iter().enumerate() {
                if a.contains(d) {
                    pattern |= 1u8 << i;
                }
            }
            match by_pattern.iter_mut().find(|(p, _)| *p == pattern) {
                Some((_, tiles)) => tiles.push(d),
                None => by_pattern.push((pattern, vec![d])),
            }
        }
        by_pattern.sort_by_key(|(p, _)| *p);

        let capacities: [usize; HIDDEN_SEATS] =
            core::array::from_fn(|i| kernel.hidden()[i].capacity);

        let zero: Layer = [[[0u128; CAP_DIM]; CAP_DIM]; CAP_DIM];
        let mut layers: Vec<Box<Layer>> = vec![Box::new(zero); by_pattern.len() + 1];
        layers[by_pattern.len()][0][0][0] = 1;
        for i in (0..by_pattern.len()).rev() {
            let (pattern, tiles) = &by_pattern[i];
            let n = tiles.len();
            let mut cur = Box::new(zero);
            for c in splits(n, *pattern) {
                let weight = multinomial(n, c);
                if weight == 0 {
                    continue;
                }
                for r0 in c[0]..=capacities[0] {
                    for r1 in c[1]..=capacities[1] {
                        for r2 in c[2]..=capacities[2] {
                            let sub = layers[i + 1][r0 - c[0]][r1 - c[1]][r2 - c[2]];
                            if sub != 0 {
                                cur[r0][r1][r2] += weight * sub;
                            }
                        }
                    }
                }
            }
            layers[i] = cur;
        }

        FiberDp {
            groups: by_pattern,
            layers,
            capacities,
        }
    }

    /// `|Phi(C)|`, exact.
    pub fn count(&self) -> u128 {
        self.layers[0][self.capacities[0]][self.capacities[1]][self.capacities[2]]
    }

    pub(crate) fn groups(&self) -> &[(u8, Vec<Domino>)] {
        &self.groups
    }

    pub(crate) fn completions(&self, group: usize, remaining: [usize; HIDDEN_SEATS]) -> u128 {
        self.layers[group][remaining[0]][remaining[1]][remaining[2]]
    }

    pub(crate) fn capacities(&self) -> [usize; HIDDEN_SEATS] {
        self.capacities
    }
}

impl Kernel {
    /// Exact `|Phi(C)|` without enumerating.
    pub fn count(&self) -> u128 {
        FiberDp::new(self).count()
    }

    /// Lazy enumeration of `Phi(C)`; nothing beyond the current world is
    /// allocated.
    pub fn worlds(&self) -> FiberIter<'_> {
        FiberIter::new(self)
    }
}

/// Advances `idx` to the next `k`-combination of `0..n` in lexicographic
/// order, returning false when the last one has been passed.
fn next_combination(idx: &mut [usize], n: usize) -> bool {
    let k = idx.len();
    let mut i = k;
    while i > 0 {
        i -= 1;
        if idx[i] + k - i < n {
            idx[i] += 1;
            for j in (i + 1)..k {
                idx[j] = idx[j - 1] + 1;
            }
            return true;
        }
    }
    false
}

/// Depth-first enumeration over the three hidden slots: choose slot 0's cell
/// from its allowed tiles, then slot 1's from what is left, then slot 2 takes
/// the remainder if the remainder is allowed there.
pub struct FiberIter<'k> {
    kernel: &'k Kernel,
    allowed: [DominoSet; HIDDEN_SEATS],
    candidates: [Vec<Domino>; HIDDEN_SEATS],
    combination: [Vec<usize>; HIDDEN_SEATS],
    chosen: [DominoSet; HIDDEN_SEATS],
    level: usize,
    advancing: bool,
    done: bool,
}

impl<'k> FiberIter<'k> {
    fn new(kernel: &'k Kernel) -> FiberIter<'k> {
        FiberIter {
            kernel,
            allowed: core::array::from_fn(|i| kernel.allowed(i)),
            candidates: core::array::from_fn(|_| Vec::new()),
            combination: core::array::from_fn(|_| Vec::new()),
            chosen: [DominoSet::EMPTY; HIDDEN_SEATS],
            level: 0,
            advancing: false,
            done: false,
        }
    }

    fn capacity(&self, level: usize) -> usize {
        let Hidden { capacity, .. } = self.kernel.hidden()[level];
        capacity
    }

    fn remaining_at(&self, level: usize) -> DominoSet {
        let mut out = self.kernel.pool();
        for taken in self.chosen.iter().take(level) {
            out = out.difference(*taken);
        }
        out
    }

    fn sync(&mut self, level: usize) {
        self.chosen[level] = self.combination[level]
            .iter()
            .map(|i| self.candidates[level][*i])
            .collect();
    }

    /// Positions `level` at its first cell, or reports that it has none.
    fn enter(&mut self, level: usize) -> bool {
        let pool = self.remaining_at(level);
        self.candidates[level] = pool.intersection(self.allowed[level]).iter().collect();
        let k = self.capacity(level);
        if self.candidates[level].len() < k {
            return false;
        }
        self.combination[level] = (0..k).collect();
        self.sync(level);
        true
    }

    fn step(&mut self, level: usize) -> bool {
        let n = self.candidates[level].len();
        let mut idx = core::mem::take(&mut self.combination[level]);
        let ok = next_combination(&mut idx, n);
        self.combination[level] = idx;
        if ok {
            self.sync(level);
        }
        ok
    }
}

impl Iterator for FiberIter<'_> {
    type Item = World;

    fn next(&mut self) -> Option<World> {
        loop {
            if self.done {
                return None;
            }
            if self.level == HIDDEN_SEATS {
                let world = self.kernel.world(self.chosen);
                self.level = HIDDEN_SEATS - 1;
                self.advancing = true;
                return Some(world);
            }
            let ok = if self.advancing {
                self.step(self.level)
            } else {
                self.enter(self.level)
            };
            if ok {
                self.advancing = false;
                self.level += 1;
            } else if self.level == 0 {
                self.done = true;
            } else {
                self.level -= 1;
                self.advancing = true;
            }
        }
    }
}
