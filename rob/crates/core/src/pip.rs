//! Pip values and pip permutations.
//!
//! Implements the pip set `P = {0..6}` of Math §2.1 and the pip-permutation
//! group `S_7` consumed by the transport theory (Math §3.9–3.10; ALG-17..24).

/// A pip value in `0..=6`. Implements a member of `P` (Math §2.1).
///
/// Pips carry their natural numeric order. That order is game-meaningful:
/// the higher pip of an uncalled domino determines its led suit
/// (Rules R-PLAY-05), unlike [`crate::domino::DominoId`] whose magnitude is
/// identity only (INV-7).
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Pip(u8);

impl Pip {
    /// Validating constructor: `Some` exactly for values `0..=6`.
    pub const fn new(value: u8) -> Option<Pip> {
        if value <= 6 {
            Some(Pip(value))
        } else {
            None
        }
    }

    /// The numeric pip value in `0..=6`.
    pub const fn value(self) -> u8 {
        self.0
    }
}

/// The seven pips in ascending order. Implements `P` of Math §2.1.
pub const PIPS: [Pip; 7] = [Pip(0), Pip(1), Pip(2), Pip(3), Pip(4), Pip(5), Pip(6)];

/// A bijection `P -> P`. Implements a pip permutation of Math §3.9 (ALG-17).
///
/// Applied to dominoes endpoint-wise and to natural suit contexts directly;
/// the called-suit label 7 is always fixed (Math §3.9).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PipPermutation {
    image: [Pip; 7],
}

impl PipPermutation {
    /// Validating constructor: requires `image` to be a bijection on `P`.
    pub fn new(image: [Pip; 7]) -> Option<PipPermutation> {
        let mut seen = [false; 7];
        for pip in image {
            let i = pip.value() as usize;
            if seen[i] {
                return None;
            }
            seen[i] = true;
        }
        Some(PipPermutation { image })
    }

    /// The identity permutation.
    pub fn identity() -> PipPermutation {
        PipPermutation { image: PIPS }
    }

    /// The swap `2 <-> 3`, the sole nonidentity count-preserving pip
    /// permutation (Math §3.9; ALG-17).
    pub fn swap_2_3() -> PipPermutation {
        PipPermutation {
            image: [Pip(0), Pip(1), Pip(3), Pip(2), Pip(4), Pip(5), Pip(6)],
        }
    }

    /// Image of one pip.
    pub fn apply(&self, pip: Pip) -> Pip {
        self.image[pip.value() as usize]
    }

    /// The inverse permutation.
    pub fn inverse(&self) -> PipPermutation {
        let mut image = PIPS;
        for (i, &target) in self.image.iter().enumerate() {
            image[target.value() as usize] = PIPS[i];
        }
        PipPermutation { image }
    }

    /// All `7! = 5,040` pip permutations, in a fixed deterministic order.
    ///
    /// Exhaustion surface for the count-preserving classification
    /// (Math §3.9; ALG-17).
    pub fn all() -> Vec<PipPermutation> {
        let mut out = Vec::with_capacity(5040);
        let mut current = [Pip(0); 7];
        let mut used = [false; 7];
        fill(&mut out, &mut current, &mut used, 0);
        out
    }
}

fn fill(out: &mut Vec<PipPermutation>, current: &mut [Pip; 7], used: &mut [bool; 7], depth: usize) {
    if depth == 7 {
        out.push(PipPermutation { image: *current });
        return;
    }
    for i in 0..7 {
        if !used[i] {
            used[i] = true;
            current[depth] = PIPS[i];
            fill(out, current, used, depth + 1);
            used[i] = false;
        }
    }
}
