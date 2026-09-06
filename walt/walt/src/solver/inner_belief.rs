//! The belief strategy of modeled minds, independent of their level/field.
//!
//! `Voidless` freezes the historical shuffle stream. `VoidsCounted` uses
//! public mechanical deductions and the existing kernel's exact uniform
//! sampler. Neither strategy conditions on a behavioral likelihood.

use super::{bit, mask_bits, mask_of, set_of, Deadline, Key, SplitMix64, FULL_MASK};
use crate::kernel::fiber::FiberDp;
use crate::kernel::kernel::{Hidden, Kernel};
use crate::kernel::sample::SplitMix64 as KernelRng;
use crate::rules::{Context, ContextSet, Decl, Domino, Seat};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InnerBelief {
    #[default]
    Voidless,
    VoidsCounted,
}

impl InnerBelief {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Voidless => "voidless",
            Self::VoidsCounted => "voids-counted",
        }
    }

    /// None is the legacy cache coordinate. Some([0; 4]) is a tracked
    /// opening state: later failures to follow must update it.
    pub const fn root_voids(self, voids: [u32; 4]) -> Option<[u32; 4]> {
        match self {
            Self::Voidless => None,
            Self::VoidsCounted => Some(voids),
        }
    }

    /// Sample a modeled seat's belief without consulting any host world.
    /// None means deadline death. An inconsistent frame is an invariant
    /// failure, never a silently weakened belief or a partial sample.
    #[allow(clippy::too_many_arguments)]
    pub fn sample(
        self,
        dcl: Decl,
        seat: Seat,
        hand: u32,
        key: &Key,
        sizes: [usize; 4],
        n: usize,
        rng: &mut SplitMix64,
        deadline: Deadline,
    ) -> Option<Vec<[u32; 4]>> {
        let unseen = FULL_MASK & !key.played & !hand;
        let others: Vec<usize> = (0..4).filter(|&s| s != seat.index()).collect();
        assert_eq!(
            unseen.count_ones() as usize,
            others.iter().map(|&s| sizes[s]).sum::<usize>()
        );
        if deadline.passed() {
            return None;
        }
        match self {
            Self::Voidless => {
                // Keep shuffle order and RNG consumption byte-for-byte
                // compatible, including the subsequent Dice seeds.
                let mut tiles = mask_bits(unseen);
                let mut worlds = Vec::with_capacity(n);
                for sample_index in 0..n {
                    if (sample_index == 0 || sample_index & 0xFF == 0) && deadline.passed() {
                        return None;
                    }
                    for i in (1..tiles.len()).rev() {
                        let j = rng.below((i + 1) as u64) as usize;
                        tiles.swap(i, j);
                    }
                    let mut w = [0; 4];
                    w[seat.index()] = hand;
                    let mut off = 0;
                    for &s in &others {
                        w[s] = tiles[off..off + sizes[s]]
                            .iter()
                            .fold(0, |m, &t| m | (1u32 << t));
                        off += sizes[s];
                    }
                    worlds.push(w);
                }
                Some(worlds)
            }
            Self::VoidsCounted => {
                let voids = key
                    .voids
                    .expect("tracked beliefs require explicit public voids");
                assert_eq!(
                    hand & voids[seat.index()],
                    0,
                    "modeled hand contradicts public voids"
                );
                let hidden = core::array::from_fn(|i| {
                    let s = others[i];
                    Hidden {
                        seat: Seat::from_index(s).expect("seat"),
                        capacity: sizes[s],
                        voids: contexts(dcl, voids[s]),
                    }
                });
                let kernel = Kernel::new(dcl, seat, set_of(hand), set_of(unseen), hidden)
                    .expect("modeled frame has conserved capacities");
                let dp = FiberDp::new(&kernel);
                assert!(
                    dp.count() > 0,
                    "modeled belief has no void-consistent completion"
                );
                // Separate, deterministic stream for the kernel sampler;
                // its exact integer draws need no new RNG implementation.
                let mut sampler_rng = KernelRng::new(rng.next_u64());
                let mut worlds = Vec::with_capacity(n);
                for _ in 0..n {
                    if deadline.passed() {
                        return None;
                    }
                    let world = kernel
                        .sample_with(&dp, &mut sampler_rng)
                        .expect("positive fiber");
                    worlds.push(core::array::from_fn(|s| mask_of(world.hands()[s])));
                }
                Some(worlds)
            }
        }
    }
}

/// Replay stores forbidden tile masks; the kernel stores effective contexts.
/// Masks must be unions of declaration-relative incidence sets. Reconstruct
/// that union losslessly, including called-suit absorption and doubles.
fn contexts(dcl: Decl, mask: u32) -> ContextSet {
    let mut result = ContextSet::EMPTY;
    let mut reconstructed = 0;
    for q in Context::ALL {
        let incidence = mask_of(dcl.effective_incidence(q));
        if incidence != 0 && incidence & !mask == 0 {
            result.insert(q);
            reconstructed |= incidence;
        }
    }
    assert_eq!(
        reconstructed, mask,
        "void mask must represent public contexts"
    );
    result
}

/// One authority for deductions on both searched and replayed continuations.
/// None stays None, so legacy keys never acquire unused history distinctions.
pub(super) fn after_play(key: &Key, dcl: Decl, tile: Domino) -> Option<[u32; 4]> {
    key.voids.map(|mut voids| {
        let seat = (usize::from(key.leader) + key.plays.len()) % 4;
        assert_eq!(voids[seat] & bit(tile), 0, "play contradicts public voids");
        if let Some(&lead) = key.plays.first() {
            let context = dcl.led_context(Domino::from_index(usize::from(lead)).expect("lead"));
            if !dcl.follows(tile, context) {
                voids[seat] |= mask_of(dcl.effective_incidence(context));
            }
        }
        voids
    })
}
