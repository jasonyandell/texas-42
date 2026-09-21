//! Exact small-support evaluator for a fixed modeled level-k policy.
//!
//! Each bit of `alive` is a distinct sampled world, including duplicate deals.
//! Every evaluated observed-action bucket contains its complete sample set.
//! The optional closed-buckets path certifies completeness by legal exclusion;
//! other paths classify all alive IDs first. Bounds are intervals of integer
//! successful-world counts, never approximate probabilities or partial policies.

use super::{Field, InnerBelief, Key, MoveOrdering, Solver, FULL_MASK};
use crate::rules::{Context, Decl, Domino, Seat};
use std::sync::atomic::Ordering;

#[derive(Clone, Copy)]
struct Rules {
    led: [u8; 28],
    incidence: [u32; 8],
    strength: [[u8; 28]; 8],
    count: [u8; 28],
}

const EMPTY_RULES: Rules = Rules {
    led: [0; 28],
    incidence: [0; 8],
    strength: [[0; 28]; 8],
    count: [0; 28],
};

// Derived at compile time from the authoritative declaration/rank algebra.
const RULES: [Rules; Decl::STRAIGHT_COUNT] = {
    let mut all = [EMPTY_RULES; Decl::STRAIGHT_COUNT];
    let mut di = 0;
    while di < Decl::STRAIGHT_COUNT {
        let d = Decl::STRAIGHT[di];
        let mut tile = 0;
        while tile < Domino::COUNT {
            let domino = Domino::ALL[tile];
            all[di].led[tile] = d.led_context(domino).index() as u8;
            all[di].count[tile] = domino.count() as u8;
            tile += 1;
        }
        let mut q = 0;
        while q < Context::COUNT {
            let context = Context::ALL[q];
            all[di].incidence[q] = d.effective_incidence(context).bits();
            let mut tile = 0;
            while tile < Domino::COUNT {
                let rank = d.trick_key(Domino::ALL[tile], context);
                all[di].strength[q][tile] = (rank.tier as u8) * 16 + rank.rank.value();
                tile += 1;
            }
            q += 1;
        }
        di += 1;
    }
    all
};

#[derive(Clone, Copy)]
struct State {
    played: u32,
    plays: u32,
    len: u8,
    leader: u8,
    t1: u8,
    t0: u8,
    alive: u8,
}

impl State {
    #[inline]
    fn mass(self) -> u8 {
        self.alive.count_ones() as u8
    }

    #[inline]
    fn last_trick(self) -> bool {
        (FULL_MASK & !self.played).count_ones() + u32::from(self.len) == 4
    }

    fn public_key(self) -> Key {
        let mut plays = Vec::with_capacity(self.len as usize);
        for i in 0..self.len {
            plays.push(((self.plays >> (5 * i)) & 31) as u8);
        }
        Key {
            voids: None,
            played: self.played,
            leader: self.leader,
            plays,
            banked_t1: self.t1,
            banked_t0: self.t0,
            alive: 0,
        }
    }
}

#[derive(Clone, Copy)]
struct Bounds {
    lo: u8,
    hi: u8,
}

impl Bounds {
    #[inline]
    fn exact(value: u8) -> Self {
        Self {
            lo: value,
            hi: value,
        }
    }

    #[inline]
    fn separates(self, alpha: i16, beta: i16) -> bool {
        self.lo == self.hi || i16::from(self.hi) <= alpha || i16::from(self.lo) >= beta
    }
}

/// A classified action bucket is certified complete when no unclassified
/// distinct hand can legally produce that action. This is a sufficient
/// certificate; overlapping legality is deliberately treated as unresolved.
#[cfg(feature = "closed-buckets")]
fn closed_tiles(occupied: u32, legal: &[u32], mut unclassified: u8) -> u32 {
    let mut possible = 0u32;
    while unclassified != 0 {
        let i = unclassified.trailing_zeros() as usize;
        unclassified &= unclassified - 1;
        possible |= legal[i];
    }
    occupied & !possible
}

struct Search<'a> {
    solver: &'a Solver,
    rules: &'static Rules,
    field_k: usize,
    nodes: u64,
    viewer_children: u64,
    viewer_legal: u64,
}

impl Search<'_> {
    #[inline]
    fn legal(&self, hand: u32, state: State) -> u32 {
        if state.len == 0 {
            return hand;
        }
        let q = self.rules.led[(state.plays & 31) as usize] as usize;
        let following = hand & self.rules.incidence[q];
        if following == 0 {
            hand
        } else {
            following
        }
    }

    #[inline]
    fn advance(&self, state: State, tile: u32, alive: u8) -> State {
        let mut next = State {
            played: state.played | (1 << tile),
            plays: state.plays | (tile << (5 * state.len)),
            len: state.len + 1,
            alive,
            ..state
        };
        if state.len == 3 {
            #[cfg(feature = "trick-table")]
            let (winner, points) = {
                let (offset, points) =
                    super::compact_dice::trick_outcome(self.solver.sh.dcl, next.plays);
                ((state.leader + offset) & 3, points)
            };
            #[cfg(not(feature = "trick-table"))]
            let (winner, points) = {
                let q = self.rules.led[(state.plays & 31) as usize] as usize;
                let mut highest = 0;
                let mut winner = state.leader;
                let mut points = 1u8;
                for i in 0..4 {
                    let t = ((next.plays >> (5 * i)) & 31) as usize;
                    let rank = self.rules.strength[q][t];
                    if i == 0 || rank > highest {
                        highest = rank;
                        winner = (state.leader + i) & 3;
                    }
                    points += self.rules.count[t];
                }
                (winner, points)
            };
            next.leader = winner;
            next.plays = 0;
            next.len = 0;
            if winner & 1 == 1 {
                next.t1 += points;
            } else {
                next.t0 += points;
            }
        }
        next
    }

    #[inline]
    fn visit(&mut self) -> Option<()> {
        self.nodes += 1;
        if self.solver.sh.dead.load(Ordering::Relaxed) {
            return None;
        }
        if (self.nodes == 1 || self.nodes & 0xff == 0) && self.solver.sh.deadline.passed() {
            self.solver.sh.dead.store(true, Ordering::Relaxed);
            return None;
        }
        Some(())
    }

    fn finished(&self) -> bool {
        if self.solver.sh.dead.load(Ordering::Relaxed) || self.solver.sh.deadline.passed() {
            self.solver.sh.dead.store(true, Ordering::Relaxed);
            false
        } else {
            true
        }
    }

    fn report(&self) {
        self.solver
            .sh
            .nodes
            .fetch_add(self.nodes, Ordering::Relaxed);
        self.solver
            .sh
            .viewer_children
            .fetch_add(self.viewer_children, Ordering::Relaxed);
        self.solver
            .sh
            .viewer_legal
            .fetch_add(self.viewer_legal, Ordering::Relaxed);
    }

    // The same CaptureFirst permutation used by the general viewer node.
    // Reordering only changes the work before an exact integer cutoff.
    fn viewer_moves(&self, state: State, legal: u32) -> ([u8; 7], usize) {
        let mut tiles = [0u8; 7];
        let mut priorities = [0u16; 7];
        let mut len = 0;
        let mut standing = None;
        if self.solver.ordering == MoveOrdering::CaptureFirst && state.len > 0 {
            let q = self.rules.led[(state.plays & 31) as usize] as usize;
            let mut best = 0u8;
            let mut winner_at = 0u8;
            let mut count = 0u16;
            for i in 0..state.len {
                let tile = ((state.plays >> (5 * i)) & 31) as usize;
                let rank = self.rules.strength[q][tile];
                if i == 0 || rank > best {
                    best = rank;
                    winner_at = i;
                }
                count += u16::from(self.rules.count[tile]);
            }
            standing = Some((
                q,
                best,
                count,
                ((state.leader + winner_at) & 1) == (self.solver.viewer.index() as u8 & 1),
            ));
        }
        let mut remaining = legal;
        while remaining != 0 {
            let tile = remaining.trailing_zeros() as usize;
            remaining &= remaining - 1;
            let priority = if self.solver.ordering == MoveOrdering::TileIndex {
                0
            } else if let Some((q, best, count, own_team)) = standing {
                if self.rules.strength[q][tile] > best {
                    1000 + count + u16::from(self.rules.count[tile])
                } else if own_team {
                    20 + u16::from(self.rules.count[tile])
                } else {
                    10 - u16::from(self.rules.count[tile])
                }
            } else {
                let q = self.rules.led[tile] as usize;
                let rank = self.rules.strength[q][tile];
                100 * u16::from(rank >> 4) + u16::from(rank & 15)
            };
            let mut at = len;
            while at > 0
                && (priorities[at - 1] < priority
                    || (priorities[at - 1] == priority && tiles[at - 1] > tile as u8))
            {
                priorities[at] = priorities[at - 1];
                tiles[at] = tiles[at - 1];
                at -= 1;
            }
            priorities[at] = priority;
            tiles[at] = tile as u8;
            len += 1;
        }
        (tiles, len)
    }

    // Complete observed-action partition of the alive multiset. A modeled
    // mind is consulted once per distinct hand, in first-sample-ID order.
    #[cfg(not(feature = "closed-buckets"))]
    fn field_buckets(&self, state: State, seat: usize) -> Option<([u8; 28], u32)> {
        let mut hands = [0u32; 8];
        let mut legal = [0u32; 8];
        let mut choices = [0u8; 8];
        let mut distinct = 0usize;
        let mut alive = state.alive;
        while alive != 0 {
            let sid = alive.trailing_zeros() as usize;
            alive &= alive - 1;
            let hand = self.solver.worlds[sid][seat] & !state.played;
            let lm = self.legal(hand, state);
            debug_assert_ne!(lm, 0);
            if !(0..distinct).any(|i| hands[i] == hand) {
                hands[distinct] = hand;
                legal[distinct] = lm;
                distinct += 1;
            }
        }
        let mut key = None;
        let acting_seat = Seat::from_index(seat).expect("seat in 0..4");
        for i in 0..distinct {
            choices[i] = if legal[i].count_ones() == 1 {
                legal[i].trailing_zeros() as u8
            } else {
                let public = key.get_or_insert_with(|| state.public_key());
                self.solver
                    .pi(self.field_k, public, acting_seat, hands[i], legal[i])?
            };
        }
        let mut buckets = [0u8; 28];
        let mut occupied = 0u32;
        let mut alive = state.alive;
        while alive != 0 {
            let sid = alive.trailing_zeros() as usize;
            alive &= alive - 1;
            let hand = self.solver.worlds[sid][seat] & !state.played;
            let i = (0..distinct)
                .find(|&i| hands[i] == hand)
                .expect("classified hand");
            let tile = usize::from(choices[i]);
            debug_assert_ne!(legal[i] & (1 << tile), 0);
            buckets[tile] |= 1 << sid;
            occupied |= 1 << tile;
        }
        debug_assert_eq!(
            buckets.iter().map(|b| b.count_ones()).sum::<u32>(),
            state.alive.count_ones()
        );
        Some((buckets, occupied))
    }

    /// Evaluate only certified complete observed-action buckets. The other
    /// sample IDs retain the sound aggregate interval [0, remaining]. A
    /// cutoff may therefore avoid consulting their modeled minds entirely.
    #[cfg(feature = "closed-buckets")]
    fn field_window_closed(
        &mut self,
        state: State,
        seat: usize,
        alpha: i16,
        beta: i16,
    ) -> Option<Bounds> {
        let mut hands = [0u32; 8];
        let mut legal = [0u32; 8];
        let mut ids = [0u8; 8];
        let mut distinct = 0usize;
        let mut alive = state.alive;
        while alive != 0 {
            let sid = alive.trailing_zeros() as usize;
            alive &= alive - 1;
            let hand = self.solver.worlds[sid][seat] & !state.played;
            let i = (0..distinct)
                .find(|&i| hands[i] == hand)
                .unwrap_or_else(|| {
                    let i = distinct;
                    hands[i] = hand;
                    legal[i] = self.legal(hand, state);
                    debug_assert_ne!(legal[i], 0);
                    distinct += 1;
                    i
                });
            ids[i] |= 1 << sid;
        }
        let mut unclassified = ((1u16 << distinct) - 1) as u8;
        let mut forced = 0u8;
        for (i, &mask) in legal[..distinct].iter().enumerate() {
            if mask.is_power_of_two() {
                forced |= 1 << i;
            }
        }
        let acting_seat = Seat::from_index(seat).expect("seat in 0..4");
        let mut public = None;
        let mut buckets = [0u8; 28];
        let mut occupied = 0u32;
        let mut sum = 0u8;
        let mut remaining = state.mass();
        loop {
            if i16::from(sum) >= beta || i16::from(sum + remaining) <= alpha {
                return Some(Bounds {
                    lo: sum,
                    hi: sum + remaining,
                });
            }
            // Process each closed bucket with its complete original sample
            // set, never a provisional subset. Already processed actions can
            // receive no later IDs by the legal-exclusion certificate.
            let mut closed = closed_tiles(occupied, &legal[..distinct], unclassified);
            while closed != 0 {
                let tile = closed.trailing_zeros() as usize;
                closed &= closed - 1;
                occupied &= !(1 << tile);
                let bucket = buckets[tile];
                remaining -= bucket.count_ones() as u8;
                let child = self.advance(state, tile as u32, bucket);
                let b = self.window(
                    child,
                    alpha - i16::from(sum) - i16::from(remaining),
                    beta - i16::from(sum),
                )?;
                if b.lo != b.hi {
                    let result = Bounds {
                        lo: sum + b.lo,
                        hi: sum + b.hi + remaining,
                    };
                    debug_assert!(result.separates(alpha, beta));
                    return Some(result);
                }
                sum += b.lo;
                if i16::from(sum) >= beta || i16::from(sum + remaining) <= alpha {
                    return Some(Bounds {
                        lo: sum,
                        hi: sum + remaining,
                    });
                }
            }
            if unclassified == 0 {
                debug_assert_eq!(remaining, 0);
                debug_assert_eq!(occupied, 0);
                return Some(Bounds::exact(sum));
            }
            // Forced hands are free to classify and can close a bucket before
            // an expensive pi call. Otherwise preserve first-hand-ID order.
            let free = unclassified & forced;
            let i = if free != 0 { free } else { unclassified }.trailing_zeros() as usize;
            let tile = if legal[i].is_power_of_two() {
                legal[i].trailing_zeros() as u8
            } else {
                let key = public.get_or_insert_with(|| state.public_key());
                self.solver
                    .pi(self.field_k, key, acting_seat, hands[i], legal[i])?
            };
            debug_assert_ne!(legal[i] & (1 << tile), 0);
            unclassified &= !(1 << i);
            buckets[tile as usize] |= ids[i];
            occupied |= 1 << tile;
        }
    }

    fn finish_last_trick(&self, state: State) -> u8 {
        let mut alive = state.alive;
        let mut wins = 0;
        while alive != 0 {
            let sid = alive.trailing_zeros() as usize;
            alive &= alive - 1;
            let mut next = state;
            while next.len != 0 || next.played != FULL_MASK {
                let seat = ((next.leader + next.len) & 3) as usize;
                let hand = if seat == self.solver.viewer.index() {
                    self.solver.viewer_hand0 & !next.played
                } else {
                    self.solver.worlds[sid][seat] & !next.played
                };
                debug_assert_eq!(hand.count_ones(), 1);
                next = self.advance(next, hand.trailing_zeros(), 1 << sid);
            }
            wins += u8::from(next.t1 >= self.solver.sh.bid);
        }
        wins
    }

    // Return an exact count or a sound interval outside the open window.
    fn window(&mut self, state: State, alpha: i16, beta: i16) -> Option<Bounds> {
        debug_assert!(alpha < beta);
        self.visit()?;
        let mass = state.mass();
        let initial = Bounds { lo: 0, hi: mass };
        if initial.separates(alpha, beta) {
            return Some(initial);
        }
        if state.t1 >= self.solver.sh.bid {
            return Some(Bounds::exact(mass));
        }
        if state.t0 > 42 - self.solver.sh.bid {
            return Some(Bounds::exact(0));
        }
        if state.last_trick() {
            return Some(Bounds::exact(self.finish_last_trick(state)));
        }
        let seat = ((state.leader + state.len) & 3) as usize;
        if seat == self.solver.viewer.index() {
            let legal = self.legal(self.solver.viewer_hand0 & !state.played, state);
            self.viewer_legal += u64::from(legal.count_ones());
            let (moves, len) = self.viewer_moves(state, legal);
            if self.solver.maximize {
                let (mut lo, mut hi) = (0u8, 0u8);
                for &tile in &moves[..len] {
                    if i16::from(lo) >= beta {
                        return Some(Bounds { lo, hi: mass });
                    }
                    self.viewer_children += 1;
                    let child = self.advance(state, u32::from(tile), state.alive);
                    let b = self.window(child, alpha.max(i16::from(lo)), beta)?;
                    lo = lo.max(b.lo);
                    hi = hi.max(b.hi);
                }
                let b = Bounds { lo, hi };
                debug_assert!(b.separates(alpha, beta));
                Some(b)
            } else {
                let (mut lo, mut hi) = (mass, mass);
                for &tile in &moves[..len] {
                    if i16::from(hi) <= alpha {
                        return Some(Bounds { lo: 0, hi });
                    }
                    self.viewer_children += 1;
                    let child = self.advance(state, u32::from(tile), state.alive);
                    let b = self.window(child, alpha, beta.min(i16::from(hi)))?;
                    lo = lo.min(b.lo);
                    hi = hi.min(b.hi);
                }
                let b = Bounds { lo, hi };
                debug_assert!(b.separates(alpha, beta));
                Some(b)
            }
        } else {
            #[cfg(feature = "closed-buckets")]
            {
                self.field_window_closed(state, seat, alpha, beta)
            }
            #[cfg(not(feature = "closed-buckets"))]
            {
                let (buckets, mut occupied) = self.field_buckets(state, seat)?;
                let mut sum = 0u8;
                let mut remaining = mass;
                while occupied != 0 {
                    if i16::from(sum) >= beta || i16::from(sum + remaining) <= alpha {
                        return Some(Bounds {
                            lo: sum,
                            hi: sum + remaining,
                        });
                    }
                    let tile = occupied.trailing_zeros() as usize;
                    occupied &= occupied - 1;
                    let bucket = buckets[tile];
                    remaining -= bucket.count_ones() as u8;
                    let child = self.advance(state, tile as u32, bucket);
                    let b = self.window(
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
}

#[cfg(all(test, feature = "closed-buckets"))]
mod closed_bucket_tests {
    use super::closed_tiles;

    #[test]
    fn legal_exclusion_closes_only_complete_sample_buckets() {
        // Exhaust all three-hand legal masks over three actions and all
        // lawful policy choices, including duplicated sample mass per hand.
        let ids = [0b0000_0011u8, 0b0000_0100, 0b0001_1000];
        for a in 1u32..8 {
            for b in 1u32..8 {
                for c in 1u32..8 {
                    let legal = [a, b, c];
                    for x in 0..3 {
                        for y in 0..3 {
                            for z in 0..3 {
                                let choices = [x, y, z];
                                if (0..3).any(|i| legal[i] & (1 << choices[i]) == 0) {
                                    continue;
                                }
                                let mut complete = [0u8; 3];
                                for i in 0..3 {
                                    complete[choices[i]] |= ids[i];
                                }
                                // Every classification subset, not only an ascending prefix.
                                for unclassified in 0u8..8 {
                                    let mut partial = [0u8; 3];
                                    let mut occupied = 0u32;
                                    for i in 0..3 {
                                        if unclassified & (1 << i) == 0 {
                                            partial[choices[i]] |= ids[i];
                                            occupied |= 1 << choices[i];
                                        }
                                    }
                                    let closed = closed_tiles(occupied, &legal, unclassified);
                                    for t in 0..3 {
                                        if closed & (1 << t) != 0 {
                                            assert_eq!(partial[t], complete[t]);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        // A provisional one-sample bucket must stay open when the other
        // hand could choose the same observed tile.
        assert_eq!(closed_tiles(1, &[1, 3], 0b10), 0);
        assert_eq!(closed_tiles(1, &[1, 2], 0b10), 1);
    }
}

/// None means ineligible (use the general solver); Some(None) means refusal.
pub(super) fn fixed_choice(solver: &Solver, key: &Key, tiles: &[u8]) -> Option<Option<u8>> {
    if !solver.sh.straight_fast_paths() { return None; }
    let field_k = match solver.field {
        Field::Level(k) => k,
        _ => return None,
    };
    if solver.parallel
        || solver.sh.inner_belief != InnerBelief::Voidless
        || !(1..=8).contains(&solver.worlds.len())
        || key.alive != 0
        || key.plays.len() > 3
        || key.voids.is_some()
        || (usize::from(key.leader) + key.plays.len()) % 4 != solver.viewer.index()
    {
        return None;
    }
    assert!(!tiles.is_empty() && tiles.windows(2).all(|w| w[0] < w[1]));
    let mut packed = 0u32;
    for (i, &tile) in key.plays.iter().enumerate() {
        packed |= u32::from(tile) << (5 * i);
    }
    let root = State {
        played: key.played,
        plays: packed,
        len: key.plays.len() as u8,
        leader: key.leader,
        t1: key.banked_t1,
        t0: key.banked_t0,
        alive: ((1u16 << solver.worlds.len()) - 1) as u8,
    };
    let di = Decl::STRAIGHT
        .iter()
        .position(|&d| d == solver.sh.dcl)
        .expect("declaration");
    let mut search = Search {
        solver,
        rules: &RULES[di],
        field_k,
        nodes: 0,
        viewer_children: 0,
        viewer_legal: 0,
    };

    let result = (|| {
        if !search.finished() {
            return None;
        }
        let mut best_tile = tiles[0];
        let mass = root.mass();
        let first = search.advance(root, u32::from(best_tile), root.alive);
        let first_bounds = search.window(first, -1, i16::from(mass) + 1)?;
        debug_assert_eq!(first_bounds.lo, first_bounds.hi);
        let mut best = first_bounds.lo;
        for &tile in &tiles[1..] {
            if !search.finished() {
                return None;
            }
            if (solver.maximize && best == mass) || (!solver.maximize && best == 0) {
                break;
            }
            let child = search.advance(root, u32::from(tile), root.alive);
            let (alpha, beta) = if solver.maximize {
                (i16::from(best), i16::from(mass) + 1)
            } else {
                (-1, i16::from(best))
            };
            let bounds = search.window(child, alpha, beta)?;
            let rejected = if solver.maximize {
                i16::from(bounds.hi) <= alpha
            } else {
                i16::from(bounds.lo) >= beta
            };
            if !rejected {
                // The opposite cutoff lies outside [0,mass], so acceptance
                // is exact and strictly better. Ascending tiles keep first ties.
                debug_assert_eq!(bounds.lo, bounds.hi);
                best = bounds.lo;
                best_tile = tile;
            }
        }
        search.finished().then_some(best_tile)
    })();
    search.report();
    Some(result)
}
