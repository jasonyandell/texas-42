//! EXPLORATORY WEB TABLE — walt at a browser table with a REAL AUCTION.
//! Sits below every evidentiary tier; estimates, never receipts.
//!
//! Single-process localhost HTTP server (no deps): deals a hand, runs the
//! auction (each seat in turn bids or passes; walt seats price every
//! declaration at the minimum viable bid over common random worlds, then
//! walk the bid up while P(make b) >= 1/2 — the baseline rule over
//! bidcurve.rs's curves), rotates the deal so the auction winner sits at
//! internal S1 (the bridge's audited rotation; the bidding team is always
//! internal T1), lets the winner name trump (nine declarations, saturation
//! ties refined, never index-broken), then plays the hand with level-1
//! walts at every AI seat. All-pass redeals.
//!
//! Solver: the walt-m3-probe library (banked-correct PiKey, bid-
//! parameterized cutoffs, rayon-parallel). The human plays from a chair
//! with hints evaluated from that chair only — no peeking, ever.
//!
//! Nothing here is quotable above exploratory tier; not a P-A21 statement.

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

use num_rational::BigRational;
use num_traits::Zero;

use walt_core::rules::{legal_plays, Trick};
use walt_core::{Context, Decl, Domino, Pip, Seat, Team};
use walt_m3_probe::{
    best_of, bp, level1_evaluate, mask_bits, mask_of, mix, sample_belief, set_of, Deadline, Field,
    Key, Shared, Solver, SplitMix64,
};

/// Internal bid seat after rotation (the bidding team is internal T1).
const BIDDER: usize = 1;

/// The auction's baseline threshold: bid while P(make b) >= 1/2.
// 11/16: the first zero-overbid rung of the 200-hand bidcurve calibration
// (2026-08-19, probes/bidcurve/ANALYSIS-2026-08-19.txt): at n=40 against
// the n=200 reference, theta=1/2 overbid 37/200; 11/16 overbid 0 with 0
// missed bids. Exploratory estimate.
const THETA_NUM: i64 = 11;
const THETA_DEN: i64 = 16;

fn tile_str(idx: u8) -> String {
    let dm = Domino::from_index(usize::from(idx)).expect("tile < 28");
    format!("{}-{}", dm.hi().value(), dm.lo().value())
}

fn decl_name(i: usize) -> &'static str {
    [
        "blanks", "aces", "deuces", "tres", "fours", "fives", "sixes", "doubles", "no-trump",
    ][i]
}

fn theta() -> BigRational {
    BigRational::new(THETA_NUM.into(), THETA_DEN.into())
}

// ---------------------------------------------------------------------------
// Game state
// ---------------------------------------------------------------------------

struct PlayRec {
    seat: u8,
    tile: u8,
    forced: bool,
    opts: Vec<(u8, i64)>,
}

struct TrickRec {
    plays: Vec<PlayRec>,
    winner: u8,
    pts: u8,
}

#[derive(PartialEq)]
enum Phase {
    Auction,
    Trump,
    Play,
    Done,
}

struct Game {
    /// Human's chair for the current hand (arena frame during the auction,
    /// internal frame after the rotation).
    human: usize,
    /// Human's chair as chosen in the UI (arena frame; survives rotation).
    human_arena: usize,
    hand_no: u64,
    seed: u64,
    n_outer: usize,
    n_auct: usize,
    n0: usize,
    per_move_secs: u64,
    rng: SplitMix64,
    hands: [u32; 4],
    // -- auction --
    auct_start: usize,
    auct_turn: usize,
    auct_acted: usize,
    auct_high: Option<(usize, u8)>,
    auct_worlds: Vec<[u32; 4]>,
    auct_vals: Vec<(usize, BigRational)>,
    auct_walk: Option<(usize, u8, BigRational)>,
    /// Contract bid once the auction closes (thresholds for the whole hand).
    bid: u8,
    // -- trump + play --
    dcl: Option<Decl>,
    decl_idx: Option<usize>,
    trump_worlds: Vec<[u32; 4]>,
    trump_vals: Vec<(usize, u8, BigRational)>,
    phase: Phase,
    played: u32,
    leader: u8,
    cur: Vec<PlayRec>,
    banked_t1: u8,
    banked_t0: u8,
    voids: [u32; 4],
    trick_start_played: u32,
    tricks: Vec<TrickRec>,
    hint: Option<Vec<(u8, i64)>>,
    msgs: Vec<String>,
    makes: u32,
    sets: u32,
}

impl Game {
    fn new(human: usize, seed: u64, n_outer: usize, n0: usize, per_move_secs: u64) -> Game {
        let mut g = Game {
            human,
            human_arena: human,
            hand_no: 0,
            seed,
            n_outer,
            n_auct: (n_outer / 2).clamp(24, 60),
            n0,
            per_move_secs,
            rng: SplitMix64(seed),
            hands: [0; 4],
            auct_start: 0,
            auct_turn: 0,
            auct_acted: 0,
            auct_high: None,
            auct_worlds: Vec::new(),
            auct_vals: Vec::new(),
            auct_walk: None,
            bid: 30,
            dcl: None,
            decl_idx: None,
            trump_worlds: Vec::new(),
            trump_vals: Vec::new(),
            phase: Phase::Auction,
            played: 0,
            leader: BIDDER as u8,
            cur: Vec::new(),
            banked_t1: 0,
            banked_t0: 0,
            voids: [0; 4],
            trick_start_played: 0,
            tricks: Vec::new(),
            hint: None,
            msgs: Vec::new(),
            makes: 0,
            sets: 0,
        };
        g.new_hand(human);
        g
    }

    fn new_hand(&mut self, human_arena: usize) {
        self.human_arena = human_arena;
        self.human = human_arena;
        self.hand_no += 1;
        self.rng = SplitMix64(self.seed ^ mix(self.hand_no));
        let mut tiles: Vec<u8> = (0..28).collect();
        for i in (1..tiles.len()).rev() {
            let j = self.rng.below((i + 1) as u64) as usize;
            tiles.swap(i, j);
        }
        let mask_slice = |sl: &[u8]| sl.iter().fold(0u32, |a, &x| a | (1u32 << x));
        self.hands = [
            mask_slice(&tiles[0..7]),
            mask_slice(&tiles[7..14]),
            mask_slice(&tiles[14..21]),
            mask_slice(&tiles[21..28]),
        ];
        self.auct_start = (self.hand_no as usize) % 4;
        self.auct_turn = self.auct_start;
        self.auct_acted = 0;
        self.auct_high = None;
        self.auct_worlds = Vec::new();
        self.auct_vals = Vec::new();
        self.auct_walk = None;
        self.bid = 30;
        self.dcl = None;
        self.decl_idx = None;
        self.trump_vals = Vec::new();
        self.trump_worlds = Vec::new();
        self.phase = Phase::Auction;
        self.played = 0;
        self.leader = BIDDER as u8;
        self.cur = Vec::new();
        self.banked_t1 = 0;
        self.banked_t0 = 0;
        self.voids = [0; 4];
        self.trick_start_played = 0;
        self.tricks = Vec::new();
        self.hint = None;
        self.msgs.push(format!(
            "hand {}: dealt. auction opens at S{} (30 or pass).",
            self.hand_no, self.auct_start
        ));
    }

    /// Price one declaration for a prospective bidder at bid level `b`:
    /// solve in a per-evaluation internal frame where the bidder sits at
    /// S1 (sound at the auction point — the record is empty and the other
    /// hands are anonymous samples).
    fn eval_bid(
        &mut self,
        decl_idx: usize,
        b: u8,
        worlds: Vec<[u32; 4]>,
        hand: u32,
    ) -> BigRational {
        let dcl = Decl::ALL[decl_idx];
        let deadline = Deadline::after(std::time::Duration::from_secs(self.per_move_secs));
        let sh = Arc::new(Shared::new(dcl, b, vec![self.n0], 0, 7, deadline));
        let solver = Solver::new(
            sh,
            Seat::from_index(BIDDER).expect("bid seat"),
            hand,
            true,
            worlds,
            Vec::new(),
            Field::Level(0),
        )
        .parallel();
        let root = Key {
            played: 0,
            leader: BIDDER as u8,
            plays: Vec::new(),
            banked_t1: 0,
            banked_t0: 0,
            alive: 0,
        };
        let v = solver.solve(&root);
        solver.flush_nodes();
        v.unwrap_or_else(BigRational::zero)
    }

    fn auction_min(&self) -> u8 {
        self.auct_high.map(|(_, b)| b + 1).unwrap_or(30)
    }

    /// One auction step for an AI seat: sample worlds + price the next
    /// declaration at the minimum viable bid, or walk the best declaration
    /// upward, or act.
    fn step_auction(&mut self) {
        if self.phase != Phase::Auction {
            return;
        }
        let s = self.auct_turn;
        if s == self.human_arena {
            return; // human acts via /bid
        }
        let need = self.auction_min();
        if need > 42 {
            self.auction_act(s, None);
            return;
        }
        if self.auct_vals.len() < Decl::COUNT {
            if self.auct_worlds.is_empty() {
                self.auct_worlds = sample_belief(
                    BIDDER,
                    self.hands[s],
                    0,
                    [7; 4],
                    [0; 4],
                    self.n_auct,
                    &mut self.rng,
                );
            }
            let i = self.auct_vals.len();
            let worlds = self.auct_worlds.clone();
            let hand = self.hands[s];
            let v = self.eval_bid(i, need, worlds, hand);
            self.auct_vals.push((i, v));
            return;
        }
        let (d_best, p_best) = self
            .auct_vals
            .iter()
            .cloned()
            .reduce(|best, cand| if cand.1 > best.1 { cand } else { best })
            .expect("nine evals");
        if p_best < theta() {
            self.auction_act(s, None);
            return;
        }
        match self.auct_walk.take() {
            None => {
                self.auct_walk = Some((d_best, need, p_best));
            }
            Some((d, b, p)) => {
                if b >= 42 {
                    self.auction_act(s, Some((d, b, p)));
                    return;
                }
                let worlds = self.auct_worlds.clone();
                let hand = self.hands[s];
                let v = self.eval_bid(d, b + 1, worlds, hand);
                if v >= theta() {
                    self.auct_walk = Some((d, b + 1, v));
                } else {
                    self.auction_act(s, Some((d, b, p)));
                }
            }
        }
    }

    /// Record a seat's auction action (None = pass) and advance; close the
    /// auction after all four have acted.
    fn auction_act(&mut self, s: usize, bid: Option<(usize, u8, BigRational)>) {
        match &bid {
            Some((d, b, p)) => {
                self.auct_high = Some((s, *b));
                self.msgs.push(format!(
                    "S{s} bids {b}  (leaning {}, P(make {b}) ~ {}.{:02}% on its sample)",
                    decl_name(*d),
                    bp(p) / 100,
                    bp(p) % 100
                ));
            }
            None => {
                let why = if self.auction_min() > 42 {
                    " (nothing left above 42)"
                } else {
                    ""
                };
                self.msgs.push(format!("S{s} passes{why}"));
            }
        }
        self.auct_acted += 1;
        self.auct_turn = (self.auct_turn + 1) % 4;
        self.auct_worlds = Vec::new();
        self.auct_vals = Vec::new();
        self.auct_walk = None;
        if self.auct_acted == 4 {
            self.close_auction();
        }
    }

    /// Human auction action from the UI.
    fn human_bid(&mut self, bid: Option<u8>) {
        if self.phase != Phase::Auction || self.auct_turn != self.human_arena {
            return;
        }
        match bid {
            Some(b) if b >= self.auction_min() && b <= 42 => {
                let s = self.human_arena;
                self.auct_high = Some((s, b));
                self.msgs.push(format!("you bid {b}"));
                self.auct_acted += 1;
                self.auct_turn = (self.auct_turn + 1) % 4;
                if self.auct_acted == 4 {
                    self.close_auction();
                }
            }
            Some(_) => {}
            None => {
                self.msgs.push("you pass".to_string());
                self.auct_acted += 1;
                self.auct_turn = (self.auct_turn + 1) % 4;
                if self.auct_acted == 4 {
                    self.close_auction();
                }
            }
        }
    }

    fn close_auction(&mut self) {
        match self.auct_high {
            None => {
                self.msgs
                    .push("all four pass — throw them in, next shake".to_string());
                let h = self.human_arena;
                self.new_hand(h);
            }
            Some((w, b)) => {
                // Rotate the deal so the winner sits at internal S1 (the
                // bidding team is always internal T1).
                let r = (5 - w) % 4;
                let old = self.hands;
                for (s, &h) in old.iter().enumerate() {
                    self.hands[(s + r) % 4] = h;
                }
                self.human = (self.human_arena + r) % 4;
                self.bid = b;
                self.msgs.push(format!(
                    "auction to S{w} at {b}. seats rotate so the contract sits at S1 — you are S{}.",
                    self.human
                ));
                self.trump_worlds = sample_belief(
                    BIDDER,
                    self.hands[BIDDER],
                    0,
                    [7; 4],
                    [0; 4],
                    self.n_outer,
                    &mut self.rng,
                );
                self.phase = Phase::Trump;
            }
        }
    }

    /// Evaluate one declaration for the bid seat over the given worlds:
    /// best opening lead and its P(make bid).
    fn eval_decl(&mut self, decl_idx: usize, worlds: Vec<[u32; 4]>) -> (u8, BigRational) {
        let dcl = Decl::ALL[decl_idx];
        let deadline = Deadline::after(std::time::Duration::from_secs(self.per_move_secs));
        let hand = self.hands[BIDDER];
        let seat = Seat::from_index(BIDDER).expect("bid seat");
        let sh = Arc::new(Shared::new(dcl, self.bid, vec![self.n0], 0, 7, deadline));
        let solver =
            Solver::new(sh, seat, hand, true, worlds, Vec::new(), Field::Level(0)).parallel();
        let root = Key {
            played: 0,
            leader: BIDDER as u8,
            plays: Vec::new(),
            banked_t1: 0,
            banked_t0: 0,
            alive: 0,
        };
        let mut best: Option<(u8, BigRational)> = None;
        for t in mask_bits(hand) {
            let tile = Domino::from_index(usize::from(t)).expect("tile");
            let child = solver.child_after_play(&root, tile, 0);
            match solver.solve(&child) {
                Some(v) => {
                    if best.as_ref().is_none_or(|(_, b)| v > *b) {
                        best = Some((t, v));
                    }
                }
                None => break,
            }
        }
        solver.flush_nodes();
        best.unwrap_or((mask_bits(hand)[0], BigRational::zero()))
    }

    /// One trump-phase step: evaluate the next declaration, or (once all
    /// nine are in and the bidder is an AI) refine ties and announce.
    fn step_trump(&mut self) {
        if self.trump_vals.len() < Decl::COUNT {
            let i = self.trump_vals.len();
            let worlds = self.trump_worlds.clone();
            let (t, v) = self.eval_decl(i, worlds);
            self.trump_vals.push((i, t, v));
            return;
        }
        if self.human == BIDDER {
            return; // human names trump via /pick
        }
        // Saturation ties across declarations: look closer on fresh samples.
        let mut n_cur = self.n_outer;
        loop {
            let best = self
                .trump_vals
                .iter()
                .map(|(_, _, v)| v.clone())
                .max()
                .expect("nine evals");
            let tied: Vec<usize> = self
                .trump_vals
                .iter()
                .filter(|(_, _, v)| *v == best)
                .map(|(i, _, _)| *i)
                .collect();
            if tied.len() == 1 || n_cur >= self.n_outer * 16 {
                break;
            }
            n_cur *= 4;
            self.msgs.push(format!(
                "trump tie at the top ({}) — looking closer at n={n_cur}...",
                tied.iter()
                    .map(|&i| decl_name(i))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            let worlds = sample_belief(
                BIDDER,
                self.hands[BIDDER],
                0,
                [7; 4],
                [0; 4],
                n_cur,
                &mut self.rng,
            );
            for &i in &tied {
                let (t, v) = self.eval_decl(i, worlds.clone());
                let slot = self
                    .trump_vals
                    .iter_mut()
                    .find(|(j, _, _)| *j == i)
                    .expect("tied decl present");
                slot.1 = t;
                slot.2 = v;
            }
        }
        let (i, t, v) = self
            .trump_vals
            .iter()
            .cloned()
            .reduce(|best, cand| if cand.2 > best.2 { cand } else { best })
            .expect("nine evals");
        self.announce(i, Some((t, &v)));
    }

    fn announce(&mut self, decl_idx: usize, lead: Option<(u8, &BigRational)>) {
        self.dcl = Some(Decl::ALL[decl_idx]);
        self.decl_idx = Some(decl_idx);
        self.phase = Phase::Play;
        let who = if self.human == BIDDER {
            "you".to_string()
        } else {
            format!("S{BIDDER} walt")
        };
        match lead {
            Some((t, v)) => self.msgs.push(format!(
                "{who} name{} {} on the {} bid — planned lead {}, P(make) ~ {}.{:02}%",
                if self.human == BIDDER { "" } else { "s" },
                decl_name(decl_idx),
                self.bid,
                tile_str(t),
                bp(v) / 100,
                bp(v) % 100
            )),
            None => self
                .msgs
                .push(format!("{who} named {}", decl_name(decl_idx))),
        }
    }

    fn turn(&self) -> Option<usize> {
        if self.phase != Phase::Play {
            return None;
        }
        Some((usize::from(self.leader) + self.cur.len()) % 4)
    }

    fn sizes_now(&self) -> [usize; 4] {
        let mut sz = [7 - self.tricks.len(); 4];
        for i in 0..self.cur.len() {
            sz[(usize::from(self.leader) + i) % 4] -= 1;
        }
        sz
    }

    fn key_now(&self) -> Key {
        Key {
            played: self.played,
            leader: self.leader,
            plays: self.cur.iter().map(|p| p.tile).collect(),
            banked_t1: self.banked_t1,
            banked_t0: self.banked_t0,
            alive: 0,
        }
    }

    fn led_now(&self, dcl: Decl) -> Option<Context> {
        self.cur
            .first()
            .map(|p| dcl.led_context(Domino::from_index(usize::from(p.tile)).expect("led")))
    }

    fn evaluate_seat(&mut self, seat_i: usize) -> Option<Vec<(u8, BigRational)>> {
        let dcl = self.dcl.expect("declared");
        let seat = Seat::from_index(seat_i).expect("seat");
        let hand = self.hands[seat_i] & !self.played;
        let legal = mask_of(legal_plays(dcl, set_of(hand), self.led_now(dcl)));
        let key = self.key_now();
        let sizes = self.sizes_now();
        let voids = self.voids;
        let tsp = self.trick_start_played;
        let bhs = 7 - self.tricks.len();
        let (bid, n_outer, n0, pm) = (self.bid, self.n_outer, self.n0, self.per_move_secs);
        level1_evaluate(
            dcl,
            bid,
            seat,
            hand,
            legal,
            &key,
            sizes,
            voids,
            tsp,
            bhs,
            n_outer,
            n0,
            pm,
            &mut self.rng,
        )
    }

    /// One play-phase step: the seat to act must be an AI; think and play.
    fn step_play(&mut self) {
        let Some(seat_i) = self.turn() else { return };
        if seat_i == self.human {
            return;
        }
        let dcl = self.dcl.expect("declared");
        let hand = self.hands[seat_i] & !self.played;
        let legal = mask_of(legal_plays(dcl, set_of(hand), self.led_now(dcl)));
        let (tile, forced, opts) = if legal.count_ones() == 1 {
            (legal.trailing_zeros() as u8, true, Vec::new())
        } else {
            let seat = Seat::from_index(seat_i).expect("seat");
            match self.evaluate_seat(seat_i) {
                Some(o) => {
                    let c = best_of(&o, seat.team() == Team::T1);
                    let obp: Vec<(u8, i64)> = o.iter().map(|(t, v)| (*t, bp(v))).collect();
                    (c, false, obp)
                }
                None => {
                    self.msgs
                        .push(format!("S{seat_i} eval timed out; playing lowest legal"));
                    (legal.trailing_zeros() as u8, false, Vec::new())
                }
            }
        };
        self.apply_play(seat_i, tile, forced, opts);
    }

    fn apply_play(&mut self, seat_i: usize, tile_idx: u8, forced: bool, opts: Vec<(u8, i64)>) {
        let dcl = self.dcl.expect("declared");
        let tile = Domino::from_index(usize::from(tile_idx)).expect("tile");
        if let Some(led) = self.led_now(dcl) {
            if !dcl.follows(tile, led) {
                self.voids[seat_i] |= mask_of(dcl.effective_incidence(led));
            }
        }
        self.played |= 1u32 << tile_idx;
        self.cur.push(PlayRec {
            seat: seat_i as u8,
            tile: tile_idx,
            forced,
            opts,
        });
        self.hint = None;
        if self.cur.len() < 4 {
            return;
        }
        let plays: Vec<u8> = self.cur.iter().map(|p| p.tile).collect();
        let doms = [
            Domino::from_index(usize::from(plays[0])).expect("p0"),
            Domino::from_index(usize::from(plays[1])).expect("p1"),
            Domino::from_index(usize::from(plays[2])).expect("p2"),
            Domino::from_index(usize::from(plays[3])).expect("p3"),
        ];
        let trick = Trick::new(
            Seat::from_index(usize::from(self.leader)).expect("leader"),
            doms,
        )
        .expect("distinct");
        let winner = trick.winner(dcl);
        let pts = trick.points() as u8;
        if winner.team() == Team::T1 {
            self.banked_t1 += pts;
        } else {
            self.banked_t0 += pts;
        }
        let rec = TrickRec {
            plays: std::mem::take(&mut self.cur),
            winner: winner.index() as u8,
            pts,
        };
        self.tricks.push(rec);
        self.msgs.push(format!(
            ">> S{} takes trick {} (+{pts}) — count T1 {} / T0 {}",
            winner.index(),
            self.tricks.len(),
            self.banked_t1,
            self.banked_t0
        ));
        self.leader = winner.index() as u8;
        self.trick_start_played = self.played;
        if self.tricks.len() == 7 {
            assert_eq!(self.banked_t1 + self.banked_t0, 42, "all points banked");
            self.phase = Phase::Done;
            if self.banked_t1 >= self.bid {
                self.makes += 1;
                self.msgs.push(format!(
                    "=== T1 MAKES the {} bid: {} to {} ===",
                    self.bid, self.banked_t1, self.banked_t0
                ));
            } else {
                self.sets += 1;
                self.msgs.push(format!(
                    "=== T1 is SET: only {} of {} (T0 took {}) ===",
                    self.banked_t1, self.bid, self.banked_t0
                ));
            }
        }
    }

    fn human_play(&mut self, tile_idx: u8) {
        if self.turn() != Some(self.human) {
            return;
        }
        let dcl = self.dcl.expect("declared");
        let hand = self.hands[self.human] & !self.played;
        let legal = mask_of(legal_plays(dcl, set_of(hand), self.led_now(dcl)));
        if legal & (1u32 << tile_idx) == 0 {
            return;
        }
        let opts = self.hint.take().unwrap_or_default();
        self.apply_play(self.human, tile_idx, legal.count_ones() == 1, opts);
    }

    fn human_hint(&mut self) {
        if self.turn() != Some(self.human) || self.hint.is_some() {
            return;
        }
        let seat = Seat::from_index(self.human).expect("seat");
        let maximize = seat.team() == Team::T1;
        if let Some(o) = self.evaluate_seat(self.human) {
            let mut sorted: Vec<(u8, i64)> = o.iter().map(|(t, v)| (*t, bp(v))).collect();
            sorted.sort_by(|a, b| {
                if maximize {
                    b.1.cmp(&a.1)
                } else {
                    a.1.cmp(&b.1)
                }
            });
            self.hint = Some(sorted);
        } else {
            self.msgs.push("hint timed out; play on".to_string());
        }
    }

    fn human_auto(&mut self) {
        if self.turn() != Some(self.human) {
            return;
        }
        self.human_hint();
        if let Some(h) = &self.hint {
            let best = h[0].0;
            self.human_play(best);
        }
    }

    fn human_pick(&mut self, decl_idx: usize) {
        if self.phase != Phase::Trump || self.human != BIDDER || decl_idx >= Decl::COUNT {
            return;
        }
        self.announce(decl_idx, None);
    }

    fn step(&mut self) {
        match self.phase {
            Phase::Auction => self.step_auction(),
            Phase::Trump => self.step_trump(),
            Phase::Play => self.step_play(),
            Phase::Done => {}
        }
    }

    // -- JSON ---------------------------------------------------------------

    fn json_state(&self) -> String {
        let mut s = String::with_capacity(4096);
        s.push('{');
        s.push_str(&format!(
            "\"phase\":\"{}\",",
            match self.phase {
                Phase::Auction => "auction",
                Phase::Trump => "trump",
                Phase::Play => "play",
                Phase::Done => "done",
            }
        ));
        s.push_str(&format!("\"hand_no\":{},", self.hand_no));
        s.push_str(&format!("\"human\":{},", self.human));
        s.push_str(&format!("\"bidder\":{BIDDER},"));
        s.push_str(&format!("\"bid\":{},", self.bid));
        if self.phase == Phase::Auction {
            let high = match self.auct_high {
                Some((seat, b)) => format!("[{seat},{b}]"),
                None => "null".to_string(),
            };
            s.push_str(&format!(
                "\"auction\":{{\"turn\":{},\"min\":{},\"high\":{high},\"acted\":{},\"thinking\":{}}},",
                self.auct_turn,
                self.auction_min(),
                self.auct_acted,
                self.auct_vals.len(),
            ));
        } else {
            s.push_str("\"auction\":null,");
        }
        match self.decl_idx {
            Some(i) => s.push_str(&format!("\"decl\":{i},")),
            None => s.push_str("\"decl\":null,"),
        }
        s.push_str("\"evals\":[");
        for (k, (i, t, v)) in self.trump_vals.iter().enumerate() {
            if k > 0 {
                s.push(',');
            }
            s.push_str(&format!("{{\"d\":{i},\"bp\":{},\"t\":{t}}}", bp(v)));
        }
        s.push_str("],");
        match self.turn() {
            Some(t) => s.push_str(&format!("\"turn\":{t},")),
            None => s.push_str("\"turn\":null,"),
        }
        let sizes: Vec<String> = (0..4)
            .map(|i| (self.hands[i] & !self.played).count_ones().to_string())
            .collect();
        s.push_str(&format!("\"sizes\":[{}],", sizes.join(",")));
        s.push_str(&format!(
            "\"hand\":{},",
            json_tiles(self.hands[self.human] & !self.played)
        ));
        let legal = if self.turn() == Some(self.human) {
            let dcl = self.dcl.expect("declared in play");
            mask_of(legal_plays(
                dcl,
                set_of(self.hands[self.human] & !self.played),
                self.led_now(dcl),
            ))
        } else {
            0
        };
        s.push_str(&format!("\"legal\":{},", json_tiles(legal)));
        s.push_str(&format!(
            "\"banked\":[{},{}],",
            self.banked_t1, self.banked_t0
        ));
        s.push_str(&format!("\"makes\":{},\"sets\":{},", self.makes, self.sets));
        s.push_str("\"tricks\":[");
        for (k, tr) in self.tricks.iter().enumerate() {
            if k > 0 {
                s.push(',');
            }
            s.push_str(&format!(
                "{{\"plays\":{},\"w\":{},\"p\":{}}}",
                json_plays(&tr.plays),
                tr.winner,
                tr.pts
            ));
        }
        s.push_str("],");
        s.push_str(&format!("\"cur\":{},", json_plays(&self.cur)));
        if self.phase == Phase::Done {
            let deal: Vec<String> = (0..4).map(|i| json_tiles(self.hands[i])).collect();
            s.push_str(&format!("\"deal\":[{}],", deal.join(",")));
            s.push_str(&format!(
                "\"result\":{},",
                i32::from(self.banked_t1 >= self.bid)
            ));
        } else {
            s.push_str("\"deal\":null,\"result\":null,");
        }
        match &self.hint {
            Some(h) => {
                let items: Vec<String> = h.iter().map(|(t, b)| format!("[{t},{b}]")).collect();
                s.push_str(&format!("\"hint\":[{}],", items.join(",")));
            }
            None => s.push_str("\"hint\":null,"),
        }
        s.push_str("\"msg\":[");
        let start = self.msgs.len().saturating_sub(40);
        for (k, m) in self.msgs[start..].iter().enumerate() {
            if k > 0 {
                s.push(',');
            }
            s.push('"');
            for c in m.chars() {
                match c {
                    '"' => s.push_str("\\\""),
                    '\\' => s.push_str("\\\\"),
                    _ => s.push(c),
                }
            }
            s.push('"');
        }
        s.push_str("]}");
        s
    }
}

fn json_tiles(mask: u32) -> String {
    let items: Vec<String> = mask_bits(mask).iter().map(|t| t.to_string()).collect();
    format!("[{}]", items.join(","))
}

fn json_plays(plays: &[PlayRec]) -> String {
    let items: Vec<String> = plays
        .iter()
        .map(|p| {
            let opts: Vec<String> = p.opts.iter().map(|(t, b)| format!("[{t},{b}]")).collect();
            format!(
                "{{\"s\":{},\"t\":{},\"f\":{},\"o\":[{}]}}",
                p.seat,
                p.tile,
                i32::from(p.forced),
                opts.join(",")
            )
        })
        .collect();
    format!("[{}]", items.join(","))
}

// ---------------------------------------------------------------------------
// HTTP
// ---------------------------------------------------------------------------

const PAGE: &str = include_str!("webtable.html");

fn respond(stream: &mut TcpStream, status: &str, ctype: &str, body: &str) {
    let head = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {ctype}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
        body.len()
    );
    let _ = stream.write_all(head.as_bytes());
    let _ = stream.write_all(body.as_bytes());
    let _ = stream.flush();
}

fn query_num(query: &str, name: &str) -> Option<u64> {
    for pair in query.split('&') {
        if let Some(v) = pair.strip_prefix(name) {
            if let Some(v) = v.strip_prefix('=') {
                return v.parse().ok();
            }
        }
    }
    None
}

fn handle(game: &mut Game, stream: &mut TcpStream) {
    let mut reader = BufReader::new(stream.try_clone().expect("clone stream"));
    let mut line = String::new();
    if reader.read_line(&mut line).is_err() {
        return;
    }
    loop {
        let mut h = String::new();
        match reader.read_line(&mut h) {
            Ok(0) | Err(_) => break,
            Ok(_) if h == "\r\n" || h == "\n" => break,
            Ok(_) => {}
        }
    }
    let path_full = line.split_whitespace().nth(1).unwrap_or("/");
    let (path, query) = match path_full.split_once('?') {
        Some((p, q)) => (p, q),
        None => (path_full, ""),
    };
    match path {
        "/" => respond(stream, "200 OK", "text/html; charset=utf-8", PAGE),
        "/state" => respond(stream, "200 OK", "application/json", &game.json_state()),
        "/step" => {
            game.step();
            respond(stream, "200 OK", "application/json", &game.json_state());
        }
        "/play" => {
            if let Some(t) = query_num(query, "t") {
                if t < 28 {
                    game.human_play(t as u8);
                }
            }
            respond(stream, "200 OK", "application/json", &game.json_state());
        }
        "/pick" => {
            if let Some(d) = query_num(query, "d") {
                game.human_pick(d as usize);
            }
            respond(stream, "200 OK", "application/json", &game.json_state());
        }
        "/bid" => {
            if query_num(query, "pass").is_some() {
                game.human_bid(None);
            } else if let Some(b) = query_num(query, "b") {
                if (30..=42).contains(&b) {
                    game.human_bid(Some(b as u8));
                }
            }
            respond(stream, "200 OK", "application/json", &game.json_state());
        }
        "/hint" => {
            game.human_hint();
            respond(stream, "200 OK", "application/json", &game.json_state());
        }
        "/auto" => {
            game.human_auto();
            respond(stream, "200 OK", "application/json", &game.json_state());
        }
        "/new" => {
            let seat = query_num(query, "seat")
                .map(|s| (s as usize).min(3))
                .unwrap_or(game.human_arena);
            game.new_hand(seat);
            respond(stream, "200 OK", "application/json", &game.json_state());
        }
        _ => respond(stream, "404 Not Found", "text/plain", "not found"),
    }
}

fn main() {
    // Pip trump ALL-ordering sanity: Decl::ALL[i] is PipTrump(pip i) for i<7.
    for (i, d) in Decl::ALL.iter().enumerate().take(7) {
        assert_eq!(
            *d,
            Decl::PipTrump(Pip::new(i as u8).expect("pip")),
            "Decl::ALL pip order"
        );
    }
    let args: Vec<String> = std::env::args().collect();
    let port: u16 = args
        .get(1)
        .map(|s| s.parse().expect("port"))
        .unwrap_or(4242);
    let n_outer: usize = args
        .get(2)
        .map(|s| s.parse().expect("belief sample size"))
        .unwrap_or(100);
    let n0: usize = args.get(3).map(|s| s.parse().expect("n0")).unwrap_or(8);
    let seed: u64 = args.get(4).map(|s| s.parse().expect("seed")).unwrap_or(42);
    let human: usize = args
        .get(5)
        .map(|s| s.parse().expect("seat 0..=3"))
        .unwrap_or(0);
    assert!(human < 4, "seat must be 0..=3");
    let per_move_secs: u64 = 120;

    let mut game = Game::new(human, seed, n_outer, n0, per_move_secs);
    let listener = TcpListener::bind(("127.0.0.1", port)).expect("bind localhost port");
    println!("walt web table — EXPLORATORY; estimates, never receipts");
    println!("open http://127.0.0.1:{port}  (you are S{human}; real auction, all-pass redeals)");
    println!("n_outer={n_outer} n0={n0} seed={seed}");
    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => handle(&mut game, &mut s),
            Err(_) => continue,
        }
    }
}
