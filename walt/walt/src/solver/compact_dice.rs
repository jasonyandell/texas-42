//! Allocation-free CPU recurrence for small fixed Dice bundles.
//!
//! This is an implementation of the existing sampled Dice value, not a new
//! policy or sampler. Sample IDs remain distinct even when deals repeat.

use super::{mix, Key, Shared, Solver, SplitMix64, FULL_MASK};

use crate::rules::{Context, Decl, Domino};

use std::sync::atomic::Ordering;

#[cfg(feature = "const-objective")]
mod const_objective;
#[cfg(feature = "singleton-dice")]
mod singleton;
#[cfg(feature = "trick-table")]
mod trick_table;

#[cfg(feature = "two-trick-sum")]
mod two_trick;

/// Completed-trick winner offset from the leader and point count.
#[cfg(feature = "trick-table")]
#[inline]
pub(super) fn trick_outcome(decl: Decl, packed_plays: u32) -> (u8, u8) {
    let di = match decl {
        Decl::PipTrump(pip) => pip.value() as usize,
        Decl::DoublesTrump => 7,
        Decl::NoTrump => 8,
        Decl::DoublesSuit => unreachable!("straight-only optimized path"),
    };
    trick_table::resolve(di, packed_plays)
}

// The existing API validates public tile IDs before these tables are used.
const TILE_SLOTS: usize = Domino::COUNT;

#[derive(Clone, Copy)]
struct Rules {
    #[cfg(feature = "trick-table")]
    declaration: u8,
    led: [u8; TILE_SLOTS],
    incidence: [u32; 8],
    strength: [[u8; 28]; 8],
    count: [u8; 28],
}

const EMPTY_RULES: Rules = Rules {
    #[cfg(feature = "trick-table")]
    declaration: 0,
    led: [0; TILE_SLOTS],
    incidence: [0; 8],
    strength: [[0; 28]; 8],
    count: [0; 28],
};

// Build from the authoritative rule algebra at compile time. This adds no
// first-decision initialization cost and stores no sampled or policy state.
const RULES: [Rules; Decl::STRAIGHT_COUNT] = {
    let mut all = [EMPTY_RULES; Decl::STRAIGHT_COUNT];
    let mut di = 0;
    while di < Decl::STRAIGHT_COUNT {
        let decl = Decl::STRAIGHT[di];
        #[cfg(feature = "trick-table")]
        {
            all[di].declaration = di as u8;
        }
        let mut t = 0;
        while t < Domino::COUNT {
            let tile = Domino::ALL[t];
            let context = decl.led_context(tile).index();
            assert!(context < Context::COUNT && Context::COUNT == 8);
            all[di].led[t] = context as u8;
            all[di].count[t] = tile.count() as u8;
            t += 1;
        }
        let mut q = 0;
        while q < Context::COUNT {
            let context = Context::ALL[q];
            all[di].incidence[q] = decl.effective_incidence(context).bits();
            let mut tile = 0;
            while tile < Domino::COUNT {
                let k = decl.trick_key(Domino::ALL[tile], context);
                all[di].strength[q][tile] = (k.tier as u8) * 16 + k.rank.value();
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
    #[cfg(feature = "compact-depth")]
    tricks_left: u8,
}

impl State {
    #[inline]
    fn is_last_trick(self) -> bool {
        #[cfg(feature = "compact-depth")]
        {
            self.tricks_left == 1
        }
        #[cfg(not(feature = "compact-depth"))]
        {
            (FULL_MASK & !self.played).count_ones() + u32::from(self.len) == 4
        }
    }
}

#[cfg(feature = "compact-depth")]
fn tricks_left_at_root(key: &Key) -> u8 {
    // The current unfinished trick contributes its already played tiles.
    // Every complete or future trick contributes exactly four physical tiles.
    let remaining = (FULL_MASK & !key.played).count_ones() + key.plays.len() as u32;
    assert_eq!(remaining % 4, 0, "public prefix has whole trick capacity");
    assert!(remaining <= 28, "at most seven tricks remain");
    (remaining / 4) as u8
}

#[cfg(feature = "bounded-choice")]
#[derive(Clone, Copy, Debug)]
struct Bounds {
    lo: u8,
    hi: u8,
}

#[cfg(feature = "bounded-choice")]
impl Bounds {
    fn exact(value: u8) -> Self {
        Self {
            lo: value,
            hi: value,
        }
    }

    fn separates(self, alpha: i16, beta: i16) -> bool {
        self.lo == self.hi || i16::from(self.hi) <= alpha || i16::from(self.lo) >= beta
    }
}

struct Search<'a> {
    rules: &'static Rules,
    sh: &'a Shared,
    worlds: &'a [[u32; 4]],
    seeds: &'a [u64],
    viewer: u8,
    hand: u32,
    maximize: bool,
    nodes: u64,
    viewer_children: u64,
    viewer_legal: u64,
}

struct ViewerMoves {
    mask: u32,
}

impl Iterator for ViewerMoves {
    type Item = u32;

    #[inline]
    fn next(&mut self) -> Option<u32> {
        {
            if self.mask == 0 {
                return None;
            }
            let tile = self.mask.trailing_zeros();
            self.mask &= self.mask - 1;
            Some(tile)
        }
    }
}

impl Search<'_> {
    #[inline]
    fn led_context(&self, tile: usize) -> usize {
        {
            self.rules.led[tile] as usize
        }
    }

    #[inline]
    fn viewer_moves(&self, state: State, legal: u32) -> ViewerMoves {
        {
            let _ = state;
            ViewerMoves { mask: legal }
        }
    }
    #[inline]
    fn alive_len(state: State) -> u8 {
        state.alive.count_ones() as u8
    }

    #[inline]
    fn legal(&self, hand: u32, state: State) -> u32 {
        if state.len == 0 {
            return hand;
        }
        let lead = (state.plays & 31) as usize;
        let q = self.led_context(lead);
        let follows = hand & self.rules.incidence[q];
        if follows == 0 {
            hand
        } else {
            follows
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
            #[cfg(feature = "compact-depth")]
            {
                debug_assert!(state.tricks_left > 0);
                next.tricks_left = state.tricks_left - 1;
            }
            #[cfg(feature = "trick-table")]
            let (winner_at, points) =
                trick_table::resolve(self.rules.declaration as usize, next.plays);
            #[cfg(not(feature = "trick-table"))]
            let (winner_at, points) = {
                let q = self.led_context((state.plays & 31) as usize);
                let mut highest = 0;
                let mut winner_at = 0;
                let mut points = 1u8;
                let mut i = 0;
                while i < 4 {
                    let t = ((next.plays >> (5 * i)) & 31) as usize;
                    let rank = self.rules.strength[q][t];
                    if i == 0 || rank > highest {
                        highest = rank;
                        winner_at = i;
                    }
                    points += self.rules.count[t];
                    i += 1;
                }
                (winner_at, points)
            };
            let winner = (state.leader + winner_at) & 3;
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
    fn record_hash(state: State) -> u64 {
        let mut h = mix(u64::from(state.played));
        h = mix(h ^ (u64::from(state.leader) << 32));
        let mut i = 0;
        while i < state.len {
            h = mix(h ^ (0x100 | u64::from((state.plays >> (5 * i)) & 31)));
            i += 1;
        }
        h
    }

    #[inline]
    fn visit(&mut self) -> Option<()> {
        self.nodes += 1;
        if self.sh.dead.load(Ordering::Relaxed) {
            return None;
        }
        if ((!cfg!(feature = "coalesced-deadlines") && self.nodes == 1) || self.nodes & 0xff == 0)
            && self.sh.deadline.passed()
        {
            self.sh.dead.store(true, Ordering::Relaxed);
            return None;
        }
        Some(())
    }

    // At the last trick every not-yet-acting seat has one tile in each world.
    // Resolve those forced plays directly, including a root inside the trick.
    fn last_trick(&self, state: State) -> u8 {
        let mut alive = state.alive;
        let mut wins = 0;
        while alive != 0 {
            let sid = alive.trailing_zeros() as usize;
            alive &= alive - 1;
            #[cfg(feature = "trick-table")]
            {
                let mut plays = state.plays;
                let mut played = state.played;
                let mut i = state.len;
                while i < 4 {
                    let seat = ((state.leader + i) & 3) as usize;
                    let hand = if seat as u8 == self.viewer {
                        self.hand & !played
                    } else {
                        self.worlds[sid][seat] & !played
                    };
                    debug_assert_eq!(hand.count_ones(), 1);
                    let tile = hand.trailing_zeros();
                    plays |= tile << (5 * i);
                    played |= 1 << tile;
                    i += 1;
                }
                let (winner_at, points) =
                    trick_table::resolve(self.rules.declaration as usize, plays);
                let winner = (state.leader + winner_at) & 3;
                let t1 = state.t1 + if winner & 1 == 1 { points } else { 0 };
                wins += u8::from(t1 >= self.sh.bid);
            }
            #[cfg(not(feature = "trick-table"))]
            {
                let mut next = state;
                while next.len != 0 || next.played != FULL_MASK {
                    let seat = ((next.leader + next.len) & 3) as usize;
                    let hand = if seat as u8 == self.viewer {
                        self.hand & !next.played
                    } else {
                        self.worlds[sid][seat] & !next.played
                    };
                    debug_assert_eq!(hand.count_ones(), 1);
                    next = self.advance(next, hand.trailing_zeros(), 1 << sid);
                }
                wins += u8::from(next.t1 >= self.sh.bid);
            }
        }
        wins
    }

    // Classify the entire surviving multiset before evaluating any observed
    // action child. A provisional bucket would change the viewer information
    // set, so this complete partition is shared by exact and bounded search.
    fn field_buckets(&self, state: State, seat: u8) -> ([u8; TILE_SLOTS], u32) {
        let mut buckets = [0u8; TILE_SLOTS];
        let mut occupied = 0u32;
        let mut alive = state.alive;
        #[cfg(not(feature = "lazy-record-hash"))]
        let hash = Self::record_hash(state);
        #[cfg(feature = "lazy-record-hash")]
        let mut hash = None;
        while alive != 0 {
            let sid = alive.trailing_zeros() as usize;
            alive &= alive - 1;
            let mut legal = self.legal(self.worlds[sid][seat as usize] & !state.played, state);
            let choices = legal.count_ones();
            debug_assert!(choices > 0);
            if choices > 1 {
                #[cfg(feature = "lazy-record-hash")]
                let hash = *hash.get_or_insert_with(|| Self::record_hash(state));
                let mut rng = SplitMix64(self.seeds[sid] ^ hash);
                let skip = rng.below(u64::from(choices));
                let mut i = 0;
                while i < skip {
                    legal &= legal - 1;
                    i += 1;
                }
            }
            let tile = legal.trailing_zeros();

            buckets[tile as usize] |= 1 << sid;
            occupied |= 1 << tile;
        }
        (buckets, occupied)
    }

    #[cfg(not(feature = "const-objective"))]
    fn solve(&mut self, state: State) -> Option<u8> {
        self.visit()?;

        self.solve_after_visit(state)
    }

    // Same physical state, already visited. Certificate projection enters here
    // directly, so it neither visits twice nor recertifies the same support.
    #[cfg(not(feature = "const-objective"))]
    #[inline]
    fn solve_after_visit(&mut self, state: State) -> Option<u8> {
        #[cfg(feature = "singleton-dice")]
        if state.alive.is_power_of_two() {
            return self.singleton_solve(state);
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
            let mut best = if self.maximize { 0 } else { mass };
            self.viewer_legal += u64::from(legal.count_ones());
            for tile in self.viewer_moves(state, legal) {
                self.viewer_children += 1;
                let child = self.advance(state, tile, state.alive);
                let value = self.solve(child)?;
                if self.maximize {
                    best = best.max(value);
                } else {
                    best = best.min(value);
                }
                if (self.maximize && best == mass) || (!self.maximize && best == 0) {
                    break;
                }
            }
            Some(best)
        } else {
            let (buckets, mut occupied) = self.field_buckets(state, seat);
            let mut total = 0;
            while occupied != 0 {
                let tile = occupied.trailing_zeros();
                occupied &= occupied - 1;
                total += self.solve(self.advance(state, tile, buckets[tile as usize]))?;
            }
            Some(total)
        }
    }

    /// Return an exact count or a sound interval wholly outside the open
    /// integer window (alpha, beta). Neither interval endpoint is treated as
    /// an exact value unless the endpoints coincide.
    #[cfg(all(feature = "bounded-choice", not(feature = "const-objective")))]
    fn window(&mut self, state: State, alpha: i16, beta: i16) -> Option<Bounds> {
        debug_assert!(alpha < beta);
        self.visit()?;

        self.window_after_visit(state, alpha, beta)
    }

    #[cfg(all(feature = "bounded-choice", not(feature = "const-objective")))]
    #[inline]
    fn window_after_visit(&mut self, state: State, alpha: i16, beta: i16) -> Option<Bounds> {
        let mass = Self::alive_len(state);
        let initial = Bounds { lo: 0, hi: mass };
        if initial.separates(alpha, beta) {
            return Some(initial);
        }
        #[cfg(feature = "singleton-dice")]
        if state.alive.is_power_of_two() {
            return self.singleton_solve(state).map(Bounds::exact);
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
            if self.maximize {
                let mut lower = 0u8;
                let mut upper = 0u8;
                for tile in self.viewer_moves(state, legal) {
                    if i16::from(lower) >= beta {
                        return Some(Bounds {
                            lo: lower,
                            hi: mass,
                        });
                    }
                    self.viewer_children += 1;
                    let child = self.advance(state, tile, state.alive);
                    let bounds = self.window(child, alpha.max(i16::from(lower)), beta)?;
                    lower = lower.max(bounds.lo);
                    upper = upper.max(bounds.hi);
                }
                let bounds = Bounds {
                    lo: lower,
                    hi: upper,
                };
                debug_assert!(bounds.separates(alpha, beta));
                Some(bounds)
            } else {
                let mut lower = mass;
                let mut upper = mass;
                for tile in self.viewer_moves(state, legal) {
                    if i16::from(upper) <= alpha {
                        return Some(Bounds { lo: 0, hi: upper });
                    }
                    self.viewer_children += 1;
                    let child = self.advance(state, tile, state.alive);
                    let bounds = self.window(child, alpha, beta.min(i16::from(upper)))?;
                    lower = lower.min(bounds.lo);
                    upper = upper.min(bounds.hi);
                }
                let bounds = Bounds {
                    lo: lower,
                    hi: upper,
                };
                debug_assert!(bounds.separates(alpha, beta));
                Some(bounds)
            }
        } else {
            let (buckets, mut occupied) = self.field_buckets(state, seat);
            let mut sum = 0u8;
            let mut remaining = mass;
            while occupied != 0 {
                if i16::from(sum) >= beta {
                    return Some(Bounds {
                        lo: sum,
                        hi: sum + remaining,
                    });
                }
                if i16::from(sum + remaining) <= alpha {
                    return Some(Bounds {
                        lo: sum,
                        hi: sum + remaining,
                    });
                }
                let tile = occupied.trailing_zeros();
                occupied &= occupied - 1;
                let bucket = buckets[tile as usize];
                let child_mass = bucket.count_ones() as u8;
                remaining -= child_mass;
                let child = self.advance(state, tile, bucket);
                let bounds = self.window(
                    child,
                    alpha - i16::from(sum) - i16::from(remaining),
                    beta - i16::from(sum),
                )?;
                if bounds.lo == bounds.hi {
                    sum += bounds.lo;
                } else {
                    let parent = Bounds {
                        lo: sum + bounds.lo,
                        hi: sum + bounds.hi + remaining,
                    };
                    debug_assert!(parent.separates(alpha, beta));
                    return Some(parent);
                }
            }
            Some(Bounds::exact(sum))
        }
    }

    fn finished(&self) -> bool {
        if self.sh.dead.load(Ordering::Relaxed) || self.sh.deadline.passed() {
            self.sh.dead.store(true, Ordering::Relaxed);
            false
        } else {
            true
        }
    }

    // Root entry, periodic visits and final publication retain clock checks.
    // Between root candidates a cancellation load is enough; a bounded
    // candidate may finish after expiry but its answer is then refused.
    fn continue_search(&self) -> bool {
        #[cfg(feature = "coalesced-deadlines")]
        {
            !self.sh.dead.load(Ordering::Relaxed)
        }
        #[cfg(not(feature = "coalesced-deadlines"))]
        {
            self.finished()
        }
    }

    fn report(&self, solver: &Solver, key: &Key) {
        solver.sh.compact_dice_calls.fetch_add(1, Ordering::Relaxed);
        solver
            .sh
            .compact_dice_nodes
            .fetch_add(self.nodes, Ordering::Relaxed);
        let remaining_tricks = (FULL_MASK & !key.played).count_ones() + key.plays.len() as u32;
        solver
            .sh
            .compact_dice_max_tricks
            .fetch_max(u64::from(remaining_tricks / 4), Ordering::Relaxed);
        solver.sh.nodes.fetch_add(self.nodes, Ordering::Relaxed);
        solver
            .local_viewer_children
            .fetch_add(self.viewer_children, Ordering::Relaxed);
        solver
            .local_viewer_legal
            .fetch_add(self.viewer_legal, Ordering::Relaxed);
    }
}

fn start<'a>(solver: &'a Solver, key: &Key) -> Option<(Search<'a>, State)> {
    if !solver.sh.straight_fast_paths() { return None; }
    if solver.field != super::Field::Dice
        || solver.parallel
        || !(1..=8).contains(&solver.worlds.len())
        || solver.seeds.len() != solver.worlds.len()
        || key.plays.len() > 3
        || (usize::from(key.leader) + key.plays.len()) % 4 != solver.viewer.index()
    {
        return None;
    }
    solver.check_belief_key(key);
    let mut packed_plays = 0u32;
    for (i, &tile) in key.plays.iter().enumerate() {
        packed_plays |= u32::from(tile) << (5 * i);
    }
    let full = ((1u16 << solver.worlds.len()) - 1) as u8;
    let root = State {
        played: key.played,
        plays: packed_plays,
        len: key.plays.len() as u8,
        leader: key.leader,
        t1: key.banked_t1,
        t0: key.banked_t0,
        alive: full,
        #[cfg(feature = "compact-depth")]
        tricks_left: tricks_left_at_root(key),
    };
    let di = Decl::STRAIGHT
        .iter()
        .position(|&d| d == solver.sh.dcl)
        .expect("declaration");
    let search = Search {
        rules: &RULES[di],
        sh: &solver.sh,
        worlds: &solver.worlds,
        seeds: &solver.seeds,
        viewer: solver.viewer.index() as u8,
        hand: solver.viewer_hand0,
        maximize: solver.maximize,
        nodes: 0,
        viewer_children: 0,
        viewer_legal: 0,
    };
    Some((search, root))
}

/// `None`: ineligible, use the original solver. `Some(None)`: deadline or
/// cancellation; `Some(Some(counts))`: complete values in caller order.
#[cfg(feature = "const-objective")]
fn collect_values_const<const MAX: bool>(
    search: &mut Search<'_>,
    root: State,
    tiles: &[u8],
) -> Option<Vec<u8>> {
    let mut counts = Vec::with_capacity(tiles.len());
    for &tile in tiles {
        if !search.finished() {
            return None;
        }
        let child = search.advance(root, u32::from(tile), root.alive);
        counts.push(search.solve_const::<MAX>(child)?);
    }
    search.finished().then_some(counts)
}

pub(super) fn action_values(solver: &Solver, key: &Key, tiles: &[u8]) -> Option<Option<Vec<u8>>> {
    let (mut search, root) = start(solver, key)?;
    // A caller may compare only a subset of legal actions, in any order.
    #[cfg(feature = "const-objective")]
    let result = if search.maximize {
        collect_values_const::<true>(&mut search, root, tiles)
    } else {
        collect_values_const::<false>(&mut search, root, tiles)
    };
    #[cfg(not(feature = "const-objective"))]
    let result = {
        let mut counts = Vec::with_capacity(tiles.len());
        for &tile in tiles {
            if !search.finished() {
                break;
            }
            let child = search.advance(root, u32::from(tile), root.alive);
            match search.solve(child) {
                Some(value) => counts.push(value),
                None => break,
            }
        }
        let complete = counts.len() == tiles.len() && search.finished();
        complete.then_some(counts)
    };
    search.report(solver, key);
    Some(result)
}

/// Choose the same tile as a full Fixed comparison, retaining the first
/// ascending legal tile on every tie.
#[cfg(feature = "bounded-choice")]
pub(super) fn fixed_choice(solver: &Solver, key: &Key, tiles: &[u8]) -> Option<u8> {
    let (mut search, root) = start(solver, key).expect("modeled L0 Dice root is eligible");
    let result = choose_fixed(&mut search, root, tiles);
    search.report(solver, key);
    result
}

#[cfg(all(feature = "bounded-choice", not(feature = "const-objective")))]
fn choose_fixed(search: &mut Search<'_>, root: State, tiles: &[u8]) -> Option<u8> {
    assert!(!tiles.is_empty(), "Fixed comparison has a legal candidate");
    assert!(
        tiles.windows(2).all(|w| w[0] < w[1]),
        "Fixed tie rule uses ascending candidates"
    );
    (|| {
        if !search.finished() {
            return None;
        }

        let mut best_tile = tiles[0];
        let first = search.advance(root, u32::from(best_tile), root.alive);
        let mass = Search::alive_len(root);
        let first_bounds = search.window(first, -1, i16::from(mass) + 1)?;
        debug_assert_eq!(first_bounds.lo, first_bounds.hi);
        let mut best = first_bounds.lo;

        let remaining = &tiles[1..];
        for &tile in remaining {
            if !search.continue_search() {
                return None;
            }

            if (search.maximize && best == mass) || (!search.maximize && best == 0) {
                break;
            }
            let child = search.advance(root, u32::from(tile), root.alive);
            let (alpha, beta) = if search.maximize {
                (i16::from(best), i16::from(mass) + 1)
            } else {
                (-1, i16::from(best))
            };
            search
                .sh
                .bounded_choice_probes
                .fetch_add(1, Ordering::Relaxed);
            let bounds = search.window(child, alpha, beta)?;
            let rejected = if search.maximize {
                i16::from(bounds.hi) <= alpha
            } else {
                i16::from(bounds.lo) >= beta
            };
            if rejected {
                search
                    .sh
                    .bounded_choice_rejections
                    .fetch_add(1, Ordering::Relaxed);
                continue;
            }
            // The opposite cutoff is outside [0,mass], so a nonrejected
            // candidate is exact and strictly improves the incumbent.
            debug_assert_eq!(bounds.lo, bounds.hi);
            debug_assert!(if search.maximize {
                bounds.lo > best
            } else {
                bounds.lo < best
            });
            best = bounds.lo;
            best_tile = tile;
        }
        search.finished().then_some(best_tile)
    })()
}

#[cfg(all(feature = "bounded-choice", feature = "const-objective"))]
fn choose_fixed(search: &mut Search<'_>, root: State, tiles: &[u8]) -> Option<u8> {
    if search.maximize {
        choose_fixed_const::<true>(search, root, tiles)
    } else {
        choose_fixed_const::<false>(search, root, tiles)
    }
}

#[cfg(all(feature = "bounded-choice", feature = "const-objective"))]
fn choose_fixed_const<const MAX: bool>(
    search: &mut Search<'_>,
    root: State,
    tiles: &[u8],
) -> Option<u8> {
    assert!(!tiles.is_empty() && tiles.windows(2).all(|w| w[0] < w[1]));
    (|| {
        if !search.finished() {
            return None;
        }

        let mut best_tile = tiles[0];
        let first = search.advance(root, u32::from(best_tile), root.alive);
        let mass = Search::alive_len(root);
        let first_bounds = search.window_const::<MAX>(first, -1, i16::from(mass) + 1)?;
        debug_assert_eq!(first_bounds.lo, first_bounds.hi);
        let mut best = first_bounds.lo;

        let remaining = &tiles[1..];
        for &tile in remaining {
            if !search.continue_search() {
                return None;
            }

            if (MAX && best == mass) || (!MAX && best == 0) {
                break;
            }
            let child = search.advance(root, u32::from(tile), root.alive);
            let (alpha, beta) = if MAX {
                (i16::from(best), i16::from(mass) + 1)
            } else {
                (-1, i16::from(best))
            };
            search
                .sh
                .bounded_choice_probes
                .fetch_add(1, Ordering::Relaxed);
            let bounds = search.window_const::<MAX>(child, alpha, beta)?;
            let rejected = if MAX {
                i16::from(bounds.hi) <= alpha
            } else {
                i16::from(bounds.lo) >= beta
            };
            if rejected {
                search
                    .sh
                    .bounded_choice_rejections
                    .fetch_add(1, Ordering::Relaxed);
                continue;
            }
            debug_assert_eq!(bounds.lo, bounds.hi);
            debug_assert!(if MAX {
                bounds.lo > best
            } else {
                bounds.lo < best
            });
            best = bounds.lo;
            best_tile = tile;
        }
        search.finished().then_some(best_tile)
    })()
}

#[cfg(feature = "stack-dice")]
struct PreparedVoidless {
    worlds: [[u32; 4]; 8],
    seeds: [u64; 8],
}

/// Stack form of `InnerBelief::Voidless::sample` followed by its Dice seeds.
/// The tile pool is shuffled in place across samples, exactly as in the
/// source sampler; resetting it between samples would change every later RNG
/// world even with the same seed.
#[cfg(feature = "stack-dice")]
fn prepare_voidless(
    sh: &Shared,
    key: &Key,
    viewer: usize,
    hand: u32,
    sizes: [usize; 4],
    n: usize,
    rng: &mut SplitMix64,
) -> Option<PreparedVoidless> {
    debug_assert!((1..=8).contains(&n));
    let unseen = FULL_MASK & !key.played & !hand;
    let mut others = [0usize; 3];
    let mut other_len = 0;
    for seat in 0..4 {
        if seat != viewer {
            others[other_len] = seat;
            other_len += 1;
        }
    }
    assert_eq!(
        unseen.count_ones() as usize,
        others.iter().map(|&seat| sizes[seat]).sum::<usize>(),
    );
    if sh.dead.load(Ordering::Relaxed) || sh.deadline.passed() {
        sh.dead.store(true, Ordering::Relaxed);
        return None;
    }
    let mut tiles = [0u8; 28];
    let mut pool = unseen;
    let mut tile_len = 0;
    while pool != 0 {
        tiles[tile_len] = pool.trailing_zeros() as u8;
        tile_len += 1;
        pool &= pool - 1;
    }
    let mut worlds = [[0u32; 4]; 8];
    // The noncoalesced deadline path also needs the sample index.
    #[allow(clippy::needless_range_loop)]
    for sample_index in 0..n {
        #[cfg(not(feature = "coalesced-deadlines"))]
        if (sample_index == 0 || sample_index & 0xff == 0)
            && (sh.dead.load(Ordering::Relaxed) || sh.deadline.passed())
        {
            sh.dead.store(true, Ordering::Relaxed);
            return None;
        }
        #[cfg(feature = "coalesced-deadlines")]
        if sh.dead.load(Ordering::Relaxed) {
            return None;
        }
        for i in (1..tile_len).rev() {
            let j = rng.below((i + 1) as u64) as usize;
            tiles.swap(i, j);
        }
        worlds[sample_index][viewer] = hand;
        let mut off = 0;
        for &seat in &others {
            let end = off + sizes[seat];
            let mut mask = 0u32;
            for &tile in &tiles[off..end] {
                mask |= 1u32 << tile;
            }
            worlds[sample_index][seat] = mask;
            off = end;
        }
    }
    // The reference pi counts only fully sampled worlds, before drawing Dice
    // tapes. All n tapes then continue the very same SplitMix64 stream.
    sh.inner_worlds_by_level[0].fetch_add(n as u64, Ordering::Relaxed);
    let mut seeds = [0u64; 8];
    for seed in &mut seeds[..n] {
        *seed = rng.next_u64();
    }
    Some(PreparedVoidless { worlds, seeds })
}

/// Allocation-free L0 Voidless preparation and Fixed choice. `pi` still owns
/// the policy cache and publishes the choice only after its final deadline
/// check. Counted-void beliefs keep the existing sampler/solver path.
#[cfg(feature = "stack-dice")]
#[allow(clippy::too_many_arguments)]
pub(super) fn prepared_choice(
    sh: &Shared,
    key: &Key,
    viewer: usize,
    hand: u32,
    maximize: bool,
    sizes: [usize; 4],
    n: usize,
    rng: &mut SplitMix64,
    legal_mask: u32,
) -> Option<u8> {
    assert!((1..=8).contains(&n));
    assert_eq!((usize::from(key.leader) + key.plays.len()) % 4, viewer);
    // The seed is read before sampling, without advancing the stream. Timing
    // and atomics compile out completely when this feature is disabled.

    let prepared = prepare_voidless(sh, key, viewer, hand, sizes, n, rng)?;
    let mut root_plays = 0u32;
    for (i, &tile) in key.plays.iter().enumerate() {
        root_plays |= u32::from(tile) << (5 * i);
    }
    let root = State {
        played: key.played,
        plays: root_plays,
        len: key.plays.len() as u8,
        leader: key.leader,
        t1: key.banked_t1,
        t0: key.banked_t0,
        alive: ((1u16 << n) - 1) as u8,
        #[cfg(feature = "compact-depth")]
        tricks_left: tricks_left_at_root(key),
    };
    let di = Decl::STRAIGHT
        .iter()
        .position(|&d| d == sh.dcl)
        .expect("declaration");
    let mut search = Search {
        rules: &RULES[di],
        sh,
        worlds: &prepared.worlds[..n],
        seeds: &prepared.seeds[..n],
        viewer: viewer as u8,
        hand,
        maximize,
        nodes: 0,
        viewer_children: 0,
        viewer_legal: 0,
    };
    let mut candidates = [0u8; 28];
    let mut candidate_len = 0;
    let mut legal = legal_mask;
    while legal != 0 {
        candidates[candidate_len] = legal.trailing_zeros() as u8;
        candidate_len += 1;
        legal &= legal - 1;
    }

    let choice = choose_fixed(&mut search, root, &candidates[..candidate_len]);
    sh.compact_dice_calls.fetch_add(1, Ordering::Relaxed);
    sh.compact_dice_nodes
        .fetch_add(search.nodes, Ordering::Relaxed);
    let remaining_tricks = (FULL_MASK & !key.played).count_ones() + key.plays.len() as u32;
    sh.compact_dice_max_tricks
        .fetch_max(u64::from(remaining_tricks / 4), Ordering::Relaxed);
    sh.nodes.fetch_add(search.nodes, Ordering::Relaxed);
    sh.viewer_children
        .fetch_add(search.viewer_children, Ordering::Relaxed);
    sh.viewer_legal
        .fetch_add(search.viewer_legal, Ordering::Relaxed);
    choice
}

#[cfg(all(test, feature = "stack-dice"))]
mod stack_tests {
    use super::*;
    use crate::rules::Seat;
    use crate::solver::{Deadline, InnerBelief};
    use std::time::Duration;

    #[test]
    fn stack_sampler_matches_worlds_dice_tapes_and_following_rng_state() {
        let opening = Key {
            voids: None,
            played: 0,
            leader: 0,
            plays: vec![],
            banked_t1: 0,
            banked_t0: 0,
            alive: 0,
        };
        let partial = Key {
            voids: None,
            played: (1 << 0) | (1 << 7),
            leader: 0,
            plays: vec![0, 7],
            banked_t1: 0,
            banked_t0: 0,
            alive: 0,
        };
        let frames = [
            (&opening, 0usize, (1 << 7) - 1, [7; 4]),
            (&partial, 2usize, ((1 << 7) - 1) << 14, [6, 6, 7, 7]),
        ];
        for decl in Decl::STRAIGHT {
            for &(key, viewer, hand, sizes) in &frames {
                for n in [1, 2, 8] {
                    let sh = Shared::new(
                        decl,
                        30,
                        vec![n],
                        0,
                        7,
                        Deadline::after(Duration::from_secs(10)),
                    );
                    let seed = 0x2d84_75c1 ^ u64::from(key.played) ^ n as u64;
                    let mut expected_rng = SplitMix64(seed);
                    let expected_worlds = InnerBelief::Voidless
                        .sample(
                            decl,
                            Seat::from_index(viewer).unwrap(),
                            hand,
                            key,
                            sizes,
                            n,
                            &mut expected_rng,
                            sh.deadline,
                        )
                        .unwrap();
                    let expected_seeds: Vec<u64> =
                        (0..n).map(|_| expected_rng.next_u64()).collect();
                    let expected_next = expected_rng.next_u64();
                    let mut actual_rng = SplitMix64(seed);
                    let actual =
                        prepare_voidless(&sh, key, viewer, hand, sizes, n, &mut actual_rng)
                            .unwrap();
                    assert_eq!(&actual.worlds[..n], expected_worlds.as_slice());
                    assert_eq!(&actual.seeds[..n], expected_seeds.as_slice());
                    assert_eq!(actual_rng.next_u64(), expected_next);
                    assert_eq!(sh.inner_worlds_by_level(), vec![n as u64]);
                }
            }
        }
    }
}

#[cfg(all(test, feature = "compact-depth"))]
mod depth_tests {
    use super::*;
    use crate::solver::Deadline;
    use std::time::Duration;

    #[test]
    fn trick_coordinate_matches_public_mask_after_every_legal_play() {
        assert_eq!(std::mem::size_of::<State>(), 16);
        let hands = std::array::from_fn(|seat| ((1u32 << 7) - 1) << (7 * seat));
        let worlds = [hands];
        let seeds = [0u64];
        for (di, decl) in Decl::STRAIGHT.into_iter().enumerate() {
            for leader in 0..4u8 {
                let sh = Shared::new(
                    decl,
                    30,
                    vec![1],
                    0,
                    7,
                    Deadline::after(Duration::from_secs(10)),
                );
                let search = Search {
                    rules: &RULES[di],
                    sh: &sh,
                    worlds: &worlds,
                    seeds: &seeds,
                    viewer: 0,
                    hand: hands[0],
                    maximize: false,
                    nodes: 0,
                    viewer_children: 0,
                    viewer_legal: 0,
                };
                let mut state = State {
                    played: 0,
                    plays: 0,
                    len: 0,
                    leader,
                    t1: 0,
                    t0: 0,
                    alive: 1,
                    tricks_left: 7,
                };
                for _ in 0..28 {
                    let mut key_plays = Vec::new();
                    for i in 0..state.len {
                        key_plays.push(((state.plays >> (5 * i)) & 31) as u8);
                    }
                    let key = Key {
                        voids: None,
                        played: state.played,
                        leader: state.leader,
                        plays: key_plays,
                        banked_t1: state.t1,
                        banked_t0: state.t0,
                        alive: 0,
                    };
                    assert_eq!(state.tricks_left, tricks_left_at_root(&key));
                    assert_eq!(state.is_last_trick(), state.tricks_left == 1);
                    let seat = usize::from((state.leader + state.len) & 3);
                    let hand = hands[seat] & !state.played;
                    let legal = search.legal(hand, state);
                    assert_ne!(legal, 0);
                    state = search.advance(state, legal.trailing_zeros(), 1);
                    let remaining = (FULL_MASK & !state.played).count_ones() + u32::from(state.len);
                    assert_eq!(u32::from(state.tricks_left) * 4, remaining);
                }
                assert_eq!(state.played, FULL_MASK);
                assert_eq!(state.len, 0);
                assert_eq!(state.tricks_left, 0);
                assert_eq!(u16::from(state.t1) + u16::from(state.t0), 42);
            }
        }
    }
}
