//! Exact Boolean continuation after observations leave one sampled ID.
//!
//! Support never grows. There is one deterministic field action at each
//! subsequent state, while the viewer still searches its legal choices.
//! This is not a clairvoyant split of a multi-world information set: callers
//! dispatch after complete observed-action partitioning, or after certified
//! action-constant payoff IDs have been removed, leaves one unresolved ID.

use super::{Search, SplitMix64, State};

impl Search<'_> {
    /// The caller has visited `state` already. Count/check each physical child
    /// before processing it, matching the ordinary compact visit discipline.
    #[cfg(not(feature = "const-objective"))]
    pub(super) fn singleton_solve(&mut self, mut state: State) -> Option<u8> {
        debug_assert!(state.alive.is_power_of_two());
        let sid = state.alive.trailing_zeros() as usize;
        loop {
            if state.t1 >= self.sh.bid {
                return Some(1);
            }
            if state.t0 > 42 - self.sh.bid {
                return Some(0);
            }
            if state.is_last_trick() {
                return Some(self.last_trick(state));
            }
            #[cfg(feature = "two-trick-sum")]
            if self.has_two_trick_tail(state) {
                return self.two_trick_window(state, -1, 2).map(|bounds| bounds.lo);
            }
            let seat = (state.leader + state.len) & 3;
            if seat == self.viewer {
                let legal = self.legal(self.hand & !state.played, state);
                debug_assert!(legal != 0);
                self.viewer_legal += u64::from(legal.count_ones());
                if legal.is_power_of_two() {
                    self.viewer_children += 1;
                    state = self.advance(state, legal.trailing_zeros(), state.alive);
                    self.visit()?;

                    continue;
                }
                // MAX is Boolean OR, MIN Boolean AND. Interior visit order
                // changes only work; the root Fixed policy owns exposed ties.
                for tile in self.viewer_moves(state, legal) {
                    self.viewer_children += 1;
                    let child = self.advance(state, tile, state.alive);
                    self.visit()?;

                    let value = self.singleton_solve(child)?;
                    if (self.maximize && value == 1) || (!self.maximize && value == 0) {
                        return Some(value);
                    }
                }
                return Some(u8::from(!self.maximize));
            }

            let mut legal = self.legal(self.worlds[sid][seat as usize] & !state.played, state);
            let choices = legal.count_ones();
            debug_assert!(choices > 0);
            if choices > 1 {
                let hash = Self::record_hash(state);
                let mut rng = SplitMix64(self.seeds[sid] ^ hash);
                let skip = rng.below(u64::from(choices));
                for _ in 0..skip {
                    legal &= legal - 1;
                }
            }
            // No bucket allocation or SUM bounds: this observed action has
            // exactly the one surviving sample, with its original Dice tape.
            state = self.advance(state, legal.trailing_zeros(), state.alive);
            self.visit()?;
        }
    }

    /// Monomorphized Boolean OR/AND continuation for const-objective Dice.
    /// The runtime method above remains available to its existing callers.
    #[cfg(feature = "const-objective")]
    pub(super) fn singleton_solve_const<const MAX: bool>(
        &mut self,
        mut state: State,
    ) -> Option<u8> {
        debug_assert!(state.alive.is_power_of_two());
        let sid = state.alive.trailing_zeros() as usize;
        loop {
            if state.t1 >= self.sh.bid {
                return Some(1);
            }
            if state.t0 > 42 - self.sh.bid {
                return Some(0);
            }
            if state.is_last_trick() {
                return Some(self.last_trick(state));
            }
            #[cfg(feature = "two-trick-sum")]
            if self.has_two_trick_tail(state) {
                return self.two_trick_window(state, -1, 2).map(|bounds| bounds.lo);
            }
            let seat = (state.leader + state.len) & 3;
            if seat == self.viewer {
                let legal = self.legal(self.hand & !state.played, state);
                debug_assert_ne!(legal, 0);
                self.viewer_legal += u64::from(legal.count_ones());
                if legal.is_power_of_two() {
                    self.viewer_children += 1;
                    state = self.advance(state, legal.trailing_zeros(), state.alive);
                    self.visit()?;

                    continue;
                }
                for tile in self.viewer_moves(state, legal) {
                    self.viewer_children += 1;
                    let child = self.advance(state, tile, state.alive);
                    self.visit()?;

                    let value = self.singleton_solve_const::<MAX>(child)?;
                    if (MAX && value == 1) || (!MAX && value == 0) {
                        return Some(value);
                    }
                }
                return Some(u8::from(!MAX));
            }

            let mut legal = self.legal(self.worlds[sid][seat as usize] & !state.played, state);
            let choices = legal.count_ones();
            debug_assert!(choices > 0);
            if choices > 1 {
                let hash = Self::record_hash(state);
                let mut rng = SplitMix64(self.seeds[sid] ^ hash);
                let skip = rng.below(u64::from(choices));
                for _ in 0..skip {
                    legal &= legal - 1;
                }
            }
            state = self.advance(state, legal.trailing_zeros(), state.alive);
            self.visit()?;
        }
    }
}

#[cfg(all(test, feature = "parallel"))]
mod tests {
    use super::*;
    use crate::rules::rules::{legal_plays, Trick};
    use crate::rules::{Decl, Domino, Seat, Team};
    use crate::solver::{self, Deadline, Field, Key, Shared, Solver};
    use std::sync::Arc;
    use std::time::Duration;

    struct Fixture {
        hands: [u32; 4],
        key: Key,
        boundary: u32,
        bid: u8,
    }

    fn fixture(decl: Decl, depth: usize, partial: usize, seed: u64) -> Fixture {
        let mut rng = SplitMix64(seed);
        for _ in 0..10_000 {
            let mut deck: Vec<u8> = (0..28).collect();
            for i in (1..28).rev() {
                deck.swap(i, rng.below((i + 1) as u64) as usize);
            }
            let mut hands = [0u32; 4];
            for (seat, tiles) in deck.chunks_exact(7).enumerate() {
                hands[seat] = tiles.iter().fold(0, |mask, &t| mask | (1 << t));
            }
            let mut key = Key {
                voids: None,
                played: 0,
                leader: 0,
                plays: Vec::new(),
                banked_t1: 0,
                banked_t0: 0,
                alive: 0,
            };
            let mut boundary = 0;
            for _ in 0..((7 - depth) * 4 + partial) {
                let seat = (usize::from(key.leader) + key.plays.len()) % 4;
                let led = key
                    .plays
                    .first()
                    .map(|&t| decl.led_context(Domino::ALL[t as usize]));
                let legal =
                    solver::mask_bits(legal_plays(decl, solver::set_of(hands[seat]), led).bits());
                let tile = legal[rng.below(legal.len() as u64) as usize];
                hands[seat] &= !(1 << tile);
                key.played |= 1 << tile;
                key.plays.push(tile);
                if key.plays.len() == 4 {
                    let trick = Trick::new(
                        Seat::from_index(key.leader as usize).unwrap(),
                        std::array::from_fn(|i| Domino::ALL[key.plays[i] as usize]),
                    )
                    .unwrap();
                    let winner = trick.winner(decl);
                    if winner.team() == Team::T1 {
                        key.banked_t1 += trick.points() as u8;
                    } else {
                        key.banked_t0 += trick.points() as u8;
                    }
                    key.leader = winner.index() as u8;
                    key.plays.clear();
                    boundary = key.played;
                }
            }
            if let Some(bid) = (30..=42).find(|&b| key.banked_t1 < b && key.banked_t0 <= 42 - b) {
                return Fixture {
                    hands,
                    key,
                    boundary,
                    bid,
                };
            }
        }
        panic!("no undecided legal fixture");
    }

    #[test]
    fn singleton_matches_original_all_declarations_partials_and_long_depth() {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(2)
            .build()
            .unwrap();
        let mut objectives = [0usize; 2];
        let mut cases = 0;
        for (di, decl) in Decl::ALL.into_iter().enumerate() {
            for partial in 0..4 {
                let f = fixture(decl, 4, partial, 0x5151_7000 + di as u64);
                compare(&pool, decl, 4, &f, &mut objectives);
                cases += 1;
            }
        }
        for (di, decl) in [Decl::ALL[0], Decl::ALL[7], Decl::ALL[8]]
            .into_iter()
            .enumerate()
        {
            for partial in [0, 3] {
                let f = fixture(decl, 7, partial, 0x7272_8000 + di as u64);
                compare(&pool, decl, 7, &f, &mut objectives);
                cases += 1;
            }
        }
        assert_eq!(cases, 42);
        assert!(objectives.iter().all(|&n| n > 0));
    }

    fn compare(
        pool: &rayon::ThreadPool,
        decl: Decl,
        depth: usize,
        f: &Fixture,
        objectives: &mut [usize; 2],
    ) {
        let viewer = Seat::from_index((usize::from(f.key.leader) + f.key.plays.len()) % 4).unwrap();
        let maximize = viewer.team() == Team::T1;
        objectives[usize::from(maximize)] += 1;
        let hand = f.hands[viewer.index()];
        let led = f
            .key
            .plays
            .first()
            .map(|&t| decl.led_context(Domino::ALL[t as usize]));
        let tiles = solver::mask_bits(legal_plays(decl, solver::set_of(hand), led).bits());
        let shared = Arc::new(Shared::new(
            decl,
            f.bid,
            vec![1],
            f.boundary,
            depth,
            Deadline::after(Duration::from_secs(20)),
        ));
        let seed = 0xfeed_0042_9999_5555;
        let make = || {
            Solver::new(
                Arc::clone(&shared),
                viewer,
                hand,
                maximize,
                vec![f.hands],
                vec![seed],
                Field::Dice,
            )
        };
        let compact = make().action_values(&f.key, &tiles).unwrap();
        let reference = pool.install(|| make().parallel().action_values(&f.key, &tiles).unwrap());
        assert_eq!(
            compact,
            reference,
            "decl={decl:?}, depth={depth}, partial={}",
            f.key.plays.len()
        );

        // Exercise an actual nonzero surviving ID in the child method. Only
        // its physical world and its own Dice tape may affect this value.
        let worlds = [f.hands; 8];
        let mut seeds = [0u64; 8];
        seeds[7] = seed;
        let root = State {
            played: f.key.played,
            plays: f
                .key
                .plays
                .iter()
                .enumerate()
                .fold(0, |v, (i, &t)| v | (u32::from(t) << (5 * i))),
            len: f.key.plays.len() as u8,
            leader: f.key.leader,
            t1: f.key.banked_t1,
            t0: f.key.banked_t0,
            alive: 1 << 7,
            #[cfg(feature = "compact-depth")]
            tricks_left: super::super::tricks_left_at_root(&f.key),
        };
        let di = Decl::ALL.iter().position(|&d| d == decl).unwrap();
        let mut search = Search {
            rules: &super::super::RULES[di],
            sh: &shared,
            worlds: &worlds,
            seeds: &seeds,
            viewer: viewer.index() as u8,
            hand,
            maximize,
            nodes: 0,
            viewer_children: 0,
            viewer_legal: 0,
        };
        for (&tile, (_, expected)) in tiles.iter().zip(reference) {
            let child = search.advance(root, u32::from(tile), root.alive);
            search.visit().unwrap();
            #[cfg(feature = "const-objective")]
            let value = if maximize {
                search.singleton_solve_const::<true>(child)
            } else {
                search.singleton_solve_const::<false>(child)
            }
            .unwrap();
            #[cfg(not(feature = "const-objective"))]
            let value = search.singleton_solve(child).unwrap();
            assert_eq!(
                expected,
                num_rational::BigRational::from_integer(value.into())
            );
        }
        shared
            .dead
            .store(true, std::sync::atomic::Ordering::Relaxed);
        assert!(make().action_values(&f.key, &tiles).is_none());
    }
}
