//! Feature-on monomorphized Dice recurrences. The feature-off Search methods
//! retain their runtime MAX/MIN branch for an exact A/B comparison.

use super::{Search, State};

#[cfg(feature = "bounded-choice")]
use super::Bounds;

impl Search<'_> {
    pub(super) fn solve_const<const MAX: bool>(&mut self, state: State) -> Option<u8> {
        self.visit()?;

        self.solve_const_after_visit::<MAX>(state)
    }

    #[inline]
    fn solve_const_after_visit<const MAX: bool>(&mut self, state: State) -> Option<u8> {
        #[cfg(feature = "singleton-dice")]
        if state.alive.is_power_of_two() {
            return self.singleton_solve_const::<MAX>(state);
        }
        if state.t1 >= self.sh.bid {
            return Some(Self::alive_len(state));
        }
        if state.t0 > 42 - self.sh.bid {
            return Some(0);
        }
        if state.is_last_trick() {
            return Some(self.last_trick(state));
        }
        #[cfg(feature = "two-trick-sum")]
        if self.has_two_trick_tail(state) {
            return self
                .two_trick_window(state, -1, i16::from(Self::alive_len(state)) + 1)
                .map(|bounds| bounds.lo);
        }
        let seat = (state.leader + state.len) & 3;
        if seat == self.viewer {
            let legal = self.legal(self.hand & !state.played, state);
            let mass = Self::alive_len(state);
            let mut best = if MAX { 0 } else { mass };
            self.viewer_legal += u64::from(legal.count_ones());
            for tile in self.viewer_moves(state, legal) {
                self.viewer_children += 1;
                let child = self.advance(state, tile, state.alive);
                let value = self.solve_const::<MAX>(child)?;
                best = if MAX {
                    best.max(value)
                } else {
                    best.min(value)
                };
                if (MAX && best == mass) || (!MAX && best == 0) {
                    break;
                }
            }
            Some(best)
        } else {
            let (buckets, mut occupied) = self.field_buckets(state, seat);
            let mut total = 0u8;
            while occupied != 0 {
                let tile = occupied.trailing_zeros();
                occupied &= occupied - 1;
                total +=
                    self.solve_const::<MAX>(self.advance(state, tile, buckets[tile as usize]))?;
            }
            Some(total)
        }
    }

    #[cfg(feature = "bounded-choice")]
    pub(super) fn window_const<const MAX: bool>(
        &mut self,
        state: State,
        alpha: i16,
        beta: i16,
    ) -> Option<Bounds> {
        debug_assert!(alpha < beta);
        self.visit()?;

        self.window_const_after_visit::<MAX>(state, alpha, beta)
    }

    #[cfg(feature = "bounded-choice")]
    #[inline]
    fn window_const_after_visit<const MAX: bool>(
        &mut self,
        state: State,
        alpha: i16,
        beta: i16,
    ) -> Option<Bounds> {
        let mass = Self::alive_len(state);
        let initial = Bounds { lo: 0, hi: mass };
        if initial.separates(alpha, beta) {
            return Some(initial);
        }
        #[cfg(feature = "singleton-dice")]
        if state.alive.is_power_of_two() {
            return self.singleton_solve_const::<MAX>(state).map(Bounds::exact);
        }
        if state.t1 >= self.sh.bid {
            return Some(Bounds::exact(mass));
        }
        if state.t0 > 42 - self.sh.bid {
            return Some(Bounds::exact(0));
        }
        if state.is_last_trick() {
            return Some(Bounds::exact(self.last_trick(state)));
        }
        #[cfg(feature = "two-trick-sum")]
        if self.has_two_trick_tail(state) {
            return self.two_trick_window(state, alpha, beta);
        }
        let seat = (state.leader + state.len) & 3;
        if seat == self.viewer {
            let legal = self.legal(self.hand & !state.played, state);
            self.viewer_legal += u64::from(legal.count_ones());
            if MAX {
                let (mut lower, mut upper) = (0u8, 0u8);
                for tile in self.viewer_moves(state, legal) {
                    if i16::from(lower) >= beta {
                        return Some(Bounds {
                            lo: lower,
                            hi: mass,
                        });
                    }
                    self.viewer_children += 1;
                    let child = self.advance(state, tile, state.alive);
                    let b = self.window_const::<MAX>(child, alpha.max(i16::from(lower)), beta)?;
                    lower = lower.max(b.lo);
                    upper = upper.max(b.hi);
                }
                let result = Bounds {
                    lo: lower,
                    hi: upper,
                };
                debug_assert!(result.separates(alpha, beta));
                Some(result)
            } else {
                let (mut lower, mut upper) = (mass, mass);
                for tile in self.viewer_moves(state, legal) {
                    if i16::from(upper) <= alpha {
                        return Some(Bounds { lo: 0, hi: upper });
                    }
                    self.viewer_children += 1;
                    let child = self.advance(state, tile, state.alive);
                    let b = self.window_const::<MAX>(child, alpha, beta.min(i16::from(upper)))?;
                    lower = lower.min(b.lo);
                    upper = upper.min(b.hi);
                }
                let result = Bounds {
                    lo: lower,
                    hi: upper,
                };
                debug_assert!(result.separates(alpha, beta));
                Some(result)
            }
        } else {
            let (buckets, mut occupied) = self.field_buckets(state, seat);
            let mut sum = 0u8;
            let mut remaining = mass;
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
                remaining -= bucket.count_ones() as u8;
                let child = self.advance(state, tile, bucket);
                let b = self.window_const::<MAX>(
                    child,
                    alpha - i16::from(sum) - i16::from(remaining),
                    beta - i16::from(sum),
                )?;
                if b.lo == b.hi {
                    sum += b.lo;
                } else {
                    let parent = Bounds {
                        lo: sum + b.lo,
                        hi: sum + b.hi + remaining,
                    };
                    debug_assert!(parent.separates(alpha, beta));
                    return Some(parent);
                }
            }
            Some(Bounds::exact(sum))
        }
    }
}
