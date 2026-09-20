//! Recompute cheap, pure L0 policies instead of looking up and retaining them.
//! The empirical gate is the cache-census hit/miss rate. Samples, selection,
//! original tapes, complete information sets and final deadline checks match
//! the cached path; no previously computed answer is substituted.

use super::{compact_dice, mix, record_hash, InnerBelief, Key, Solver, SplitMix64, INNER_SEED};
use crate::rules::{Seat, Team};
use std::sync::atomic::Ordering;

impl Solver {
    /// Outer None means ineligible; inner None is a real refusal/cancellation.
    /// The modeled actor is on turn, so its remaining hand size is exactly
    /// the number of tricks left, including the unfinished current trick.
    pub(super) fn uncached_l0(
        &self,
        key: &Key,
        seat: Seat,
        hand: u32,
        legal_mask: u32,
    ) -> Option<Option<u8>> {
        let n = self.sh.n_inner[0];
        let limit = if cfg!(feature = "bypass-l0-cache-all") {
            7
        } else {
            2
        };
        if self.sh.inner_belief != InnerBelief::Voidless
            || !(1..=8).contains(&n)
            || hand.count_ones() > limit
        {
            return None;
        }
        if self.sh.dead.load(Ordering::Relaxed) || self.sh.deadline.passed() {
            self.sh.dead.store(true, Ordering::Relaxed);
            return Some(None);
        }
        self.sh.pi_calls.fetch_add(1, Ordering::Relaxed);
        self.sh.pi_calls_by_level[0].fetch_add(1, Ordering::Relaxed);
        let sizes = self.hand_sizes_at(key);
        let mut rng = SplitMix64(
            INNER_SEED ^ mix(seat.index() as u64) ^ mix(u64::from(hand)) ^ record_hash(key),
        );
        let choice = compact_dice::prepared_choice(
            &self.sh,
            key,
            seat.index(),
            hand,
            seat.team() == Team::T1,
            sizes,
            n,
            &mut rng,
            legal_mask,
        );
        let result = match choice {
            Some(tile) if !self.sh.dead.load(Ordering::Relaxed) && !self.sh.deadline.passed() => {
                Some(tile)
            }
            _ => {
                self.sh.dead.store(true, Ordering::Relaxed);
                None
            }
        };
        Some(result)
    }
}
