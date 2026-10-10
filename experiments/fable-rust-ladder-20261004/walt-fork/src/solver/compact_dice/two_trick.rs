//! SUM-only penultimate-trick suffix after the viewer has already acted.
//!
//! Every remaining current-trick actor is field. Preserve complete observed
//! buckets, then resolve the forced final trick directly. The incoming state
//! has already been visited and passed the terminal and initial window guards.

use super::{Bounds, Search, SplitMix64, State, TILE_SLOTS};

impl Search<'_> {
    #[inline]
    pub(super) fn has_two_trick_tail(&self, state: State) -> bool {
        state.tricks_left == 2 && (self.hand & !state.played).is_power_of_two()
    }

    pub(super) fn two_trick_window(
        &mut self,
        state: State,
        alpha: i16,
        beta: i16,
    ) -> Option<Bounds> {
        debug_assert!(self.has_two_trick_tail(state));
        debug_assert!(state.t1 < self.sh.bid && state.t0 <= 42 - self.sh.bid);
        #[cfg(feature = "two-trick-single")]
        if state.alive.is_power_of_two() {
            let sid = state.alive.trailing_zeros() as usize;
            return match state.len {
                1 => self.two_trick_single::<1>(state, sid),
                2 => self.two_trick_single::<2>(state, sid),
                3 => self.two_trick_single::<3>(state, sid),
                _ => unreachable!("viewer already acted in penultimate trick"),
            }
            .map(Bounds::exact);
        }
        match state.len {
            1 => self.two_trick_stage::<1>(state, alpha, beta),
            2 => self.two_trick_stage::<2>(state, alpha, beta),
            3 => self.two_trick_stage::<3>(state, alpha, beta),
            _ => unreachable!("viewer already acted in penultimate trick"),
        }
    }

    #[inline]
    fn two_trick_stage<const LEN: u8>(
        &mut self,
        state: State,
        alpha: i16,
        beta: i16,
    ) -> Option<Bounds> {
        debug_assert_eq!(state.len, LEN);
        self.two_trick_stage_after_certificate::<LEN>(state, alpha, beta)
    }

    #[inline]
    fn two_trick_stage_after_certificate<const LEN: u8>(
        &mut self,
        state: State,
        alpha: i16,
        beta: i16,
    ) -> Option<Bounds> {
        let seat = ((state.leader + LEN) & 3) as usize;
        debug_assert_ne!(seat as u8, self.viewer);
        let hash = Self::record_hash(state);
        let mut buckets = [0u8; TILE_SLOTS];
        let mut occupied = 0u32;
        let mut alive = state.alive;
        while alive != 0 {
            let sid = alive.trailing_zeros() as usize;
            alive &= alive - 1;
            let mut legal = self.legal(self.worlds[sid][seat] & !state.played, state);
            debug_assert!((1..=2).contains(&legal.count_ones()));
            if !legal.is_power_of_two() {
                // This is precisely historical below(2), including its
                // unusual two-value rejection zone and every retry draw.
                let mut rng = SplitMix64(self.seeds[sid] ^ hash);
                let selected = loop {
                    let value = rng.next_u64();
                    if value < u64::MAX - 1 {
                        break value & 1;
                    }
                };
                if selected != 0 {
                    legal &= legal - 1;
                }
            }
            let tile = legal.trailing_zeros();

            buckets[tile as usize] |= 1 << sid;
            occupied |= 1 << tile;
        }

        let mut sum = 0u8;
        let mut remaining = state.alive.count_ones() as u8;
        while occupied != 0 {
            if i16::from(sum) >= beta || i16::from(sum + remaining) <= alpha {
                return Some(Bounds {
                    lo: sum,
                    hi: sum + remaining,
                });
            }
            let tile = occupied.trailing_zeros();
            occupied &= occupied - 1;
            let bucket = buckets[tile as usize];
            let mass = bucket.count_ones() as u8;
            remaining -= mass;
            let child = if LEN == 3 {
                self.advance(state, tile, bucket)
            } else {
                State {
                    played: state.played | (1 << tile),
                    plays: state.plays | (tile << (5 * LEN)),
                    len: LEN + 1,
                    alive: bucket,
                    ..state
                }
            };
            self.visit()?;

            let child_alpha = alpha - i16::from(sum) - i16::from(remaining);
            let child_beta = beta - i16::from(sum);
            let initial = Bounds { lo: 0, hi: mass };
            let b = if initial.separates(child_alpha, child_beta) {
                initial
            } else if LEN == 1 {
                self.two_trick_next::<2>(child, child_alpha, child_beta)?
            } else if LEN == 2 {
                self.two_trick_next::<3>(child, child_alpha, child_beta)?
            } else {
                self.two_trick_finish(child, child_alpha, child_beta)
            };
            if b.lo != b.hi {
                let result = Bounds {
                    lo: sum + b.lo,
                    hi: sum + b.hi + remaining,
                };
                debug_assert!(result.separates(alpha, beta));
                return Some(result);
            }
            sum += b.lo;
        }
        Some(Bounds::exact(sum))
    }

    #[inline]
    fn two_trick_next<const LEN: u8>(
        &mut self,
        state: State,
        alpha: i16,
        beta: i16,
    ) -> Option<Bounds> {
        #[cfg(feature = "two-trick-single")]
        if state.alive.is_power_of_two() {
            return self
                .two_trick_single::<LEN>(state, state.alive.trailing_zeros() as usize)
                .map(Bounds::exact);
        }
        self.two_trick_stage::<LEN>(state, alpha, beta)
    }

    /// A completed observation bucket containing one original ID needs no
    /// further partition arrays. Its viewer is already forced; the fixed
    /// remaining field sequence returns an exact Boolean outcome.
    #[cfg(feature = "two-trick-single")]
    #[inline]
    fn two_trick_single<const LEN: u8>(&mut self, state: State, sid: usize) -> Option<u8> {
        let seat = ((state.leader + LEN) & 3) as usize;
        let mut legal = self.legal(self.worlds[sid][seat] & !state.played, state);
        debug_assert!((1..=2).contains(&legal.count_ones()));
        if !legal.is_power_of_two() {
            let mut rng = SplitMix64(self.seeds[sid] ^ Self::record_hash(state));
            let selected = loop {
                let value = rng.next_u64();
                if value < u64::MAX - 1 {
                    break value & 1;
                }
            };
            if selected != 0 {
                legal &= legal - 1;
            }
        }
        let tile = legal.trailing_zeros();
        let child = if LEN == 3 {
            self.advance(state, tile, state.alive)
        } else {
            State {
                played: state.played | (1 << tile),
                plays: state.plays | (tile << (5 * LEN)),
                len: LEN + 1,
                ..state
            }
        };
        self.visit()?;

        if LEN == 1 {
            self.two_trick_single::<2>(child, sid)
        } else if LEN == 2 {
            self.two_trick_single::<3>(child, sid)
        } else {
            Some(self.two_trick_finish(child, -1, 2).lo)
        }
    }

    #[inline]
    fn two_trick_finish(&self, state: State, alpha: i16, beta: i16) -> Bounds {
        debug_assert!(state.is_last_trick() && state.len == 0);
        let mut remaining = state.alive.count_ones() as u8;
        if state.t1 >= self.sh.bid {
            return Bounds::exact(remaining);
        }
        if state.t0 > 42 - self.sh.bid {
            return Bounds::exact(0);
        }
        let mut wins = 0u8;
        let mut alive = state.alive;
        while alive != 0 {
            let bounds = Bounds {
                lo: wins,
                hi: wins + remaining,
            };
            if bounds.separates(alpha, beta) {
                return bounds;
            }
            let sid = alive.trailing_zeros() as usize;
            alive &= alive - 1;
            remaining -= 1;
            let mut plays = 0u32;
            for i in 0..4 {
                let seat = ((state.leader + i) & 3) as usize;
                let hand = if seat as u8 == self.viewer {
                    self.hand & !state.played
                } else {
                    self.worlds[sid][seat] & !state.played
                };
                debug_assert!(hand.is_power_of_two());
                plays |= hand.trailing_zeros() << (5 * i);
            }
            let (winner_at, _) =
                super::trick_table::resolve(self.rules.declaration as usize, plays);
            // With the contract still undecided and 42 points conserved,
            // T1 makes iff T1 wins the forced final trick.
            wins += (state.leader + winner_at) & 1;
        }
        Bounds::exact(wins)
    }
}
