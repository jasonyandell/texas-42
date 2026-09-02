//! controller_bridge — the §16.4 evidence/decision-controller walt as a
//! subprocess player. An exploratory, non-normative surface (CE thread:
//! CE = sampling depth) that lets an external harness (plunge, or the mk5
//! arena) seat the controller player for one decision per line of stdin
//! with ZERO external-side changes: it speaks the SAME line protocol as
//! walt_bridge.rs / rob_bridge (`rob:<path-to-controller_bridge>`).
//! Instrument tooling; creates no receipts and no claims.
//!
//! request:  `seat decl bidder h0 h1 h2 h3 h4 h5 h6 n (actor domino)*n`
//! reply:    `domino leader points0 points1` (identical grammar and seat
//!           rotation to walt_bridge.rs — the bidding team is internal T1;
//!           even arena bidders rotate every label by +1, rotated back in
//!           replies)
//! request:  `declare bidder h0 h1 h2 h3 h4 h5 h6`
//! reply:    a single declaration id (level-1 argmax P(make 30), the
//!           walt_bridge policy through the library solver; trump naming
//!           is not a controller decision)
//!
//! Decisions run `solver::act`: the decision controller on one frozen
//! level-1 continuation per legal root action (preroute-exact at small
//! fibers, adaptive above them), then THE ACTION POLICY — settled winner
//! played; honest exact tie → level-1 rank among the TIED set; Unresolved
//! at the cap → level-1 rank among the δ-SURVIVORS. The eliminations are
//! δ-safe (inside the correctness boundary); the level-1 ordering among
//! survivors/ties is a scheduling choice outside it, and every logged
//! record carries which route chose the tile (WALT_CTRL_LOG, JSONL,
//! per-PID suffix).
//!
//! Knobs (argv positional, then WALT_* env, then default):
//!   1 WALT_CTRL_WORLD_CAP   128   controller cap in raw worlds — a
//!                                 think-time budget; a low cap only
//!                                 produces more honest Unresolved→
//!                                 fallback decisions, never wrong
//!                                 settlements. Trick-1/2 decisions at
//!                                 cap 512 cost minutes.
//!   2 WALT_CTRL_EXACT_CAP  2000   preroute-exact fiber ceiling
//!   3 WALT_N_OUTER          200   fallback level-1 outer count
//!   4 WALT_N0                 8   fallback level-1 inner count
//!   5 WALT_N_DECLARE        100   declare-path belief sample
//!   6 WALT_PER_MOVE         120   declare-path seconds budget
//!   WALT_CTRL_N_OUTER_FROZEN 8 / WALT_CTRL_N0_FROZEN 2 (env only): the
//!   frozen candidates' declared schedule (identity fields, CE-A5).
//!   WALT_DECLARE_FULL=1 adds doubles/no-trump to the declare candidates.
//!
//! δ_run = 1/100 per hand; decision ordinal d = plies played + 1 (any
//! injective ordinal assignment telescopes within δ_run — stateless).
//! ESTIMATES, never receipts; not a P-A21 statement. No floats.

use std::io::{BufRead, Write};
use std::sync::Arc;
use std::time::Duration;

use num_rational::BigRational;
use num_traits::Zero;

use walt::rules::rules::Trick;
use walt::rules::{ContextSet, Decl, Domino, DominoSet, Seat, Team};
use walt::solver::act::{act, delta_run_default, ActConfig};
use walt::solver::adaptive::DrivenState;
use walt::solver::{
    arena_decl_id, bp, decl_of, mask_bits, mix, sample_belief, Deadline, Field, Key, Shared,
    Solver, SplitMix64,
};

/// Frozen seed for the declare path's belief sampling (a distinct stream
/// from every other surface constant; the play path's fallback stream is
/// `solver::act::ACT_FALLBACK_SEED`).
const CTRL_DECLARE_SEED: u64 = 0x2FFD_72DB_D01A_DFB7;

struct Config {
    act: ActConfig,
    n_declare: usize,
    per_move_secs: u64,
}

/// The replayed public record in INTERNAL seat labels (arena + r mod 4,
/// r chosen so the bidding team is internal T1 — the audited walt_bridge
/// rotation), with voids kept as CONTEXT sets (the driven-state alphabet).
struct Folded {
    r: usize,
    leader: Seat,
    trick_plays: Vec<Domino>,
    /// Banked points, indexed by `Team::index()`.
    banked: [u32; 2],
    prior_played: DominoSet,
    voids: [ContextSet; 4],
    completed: usize,
}

fn fold(dcl: Decl, bidder_arena: usize, pairs: &[(usize, usize)]) -> Folded {
    let r = if bidder_arena.is_multiple_of(2) { 1 } else { 0 };
    let mut st = Folded {
        r,
        leader: Seat::from_index((bidder_arena + r) % 4).expect("seat 0..4"),
        trick_plays: Vec::new(),
        banked: [0; 2],
        prior_played: DominoSet::EMPTY,
        voids: [ContextSet::EMPTY; 4],
        completed: 0,
    };
    for &(actor_arena, tile_id) in pairs {
        let actor = (actor_arena + r) % 4;
        let expect = st.leader.plus(st.trick_plays.len()).index();
        assert_eq!(actor, expect, "history follows turn order");
        let tile = Domino::from_index(tile_id).expect("tile id 0..28");
        assert!(
            !st.prior_played.contains(tile) && !st.trick_plays.contains(&tile),
            "tile played once"
        );
        if let Some(first) = st.trick_plays.first() {
            let led = dcl.led_context(*first);
            if !dcl.follows(tile, led) {
                st.voids[actor].insert(led);
            }
        }
        st.trick_plays.push(tile);
        if st.trick_plays.len() == 4 {
            let doms: [Domino; 4] = core::array::from_fn(|i| st.trick_plays[i]);
            let trick = Trick::new(st.leader, doms).expect("distinct tiles");
            let winner = trick.winner(dcl);
            st.banked[winner.team().index()] += trick.points();
            for d in doms {
                st.prior_played.insert(d);
            }
            st.leader = winner;
            st.trick_plays.clear();
            st.completed += 1;
        }
    }
    st
}

/// Instrumentation: when WALT_CTRL_LOG is set, append one JSON line per
/// decision (per-PID file suffix) carrying WHICH ROUTE chose the tile —
/// the record-grade honesty requirement of the action policy. Basis
/// points only; display-side division, no floats here.
fn log_decision(
    nums: &[usize],
    st: &Folded,
    viewer_i: usize,
    d: u64,
    decision: &walt::solver::act::ActDecision,
) {
    let Ok(base) = std::env::var("WALT_CTRL_LOG") else {
        return;
    };
    let path = format!("{base}.{}", std::process::id());
    let among: Vec<String> = decision
        .among
        .iter()
        .map(|t| t.index().to_string())
        .collect();
    let (tag, consumed) = decision
        .evaluation
        .as_ref()
        .map_or(("forced", 0), |e| (e.result.tag(), e.consumed));
    let opts: Vec<String> = decision
        .fallback_opts
        .as_ref()
        .map(|o| o.iter().map(|(t, v)| format!("[{t},{}]", bp(v))).collect())
        .unwrap_or_default();
    let line = format!(
        "{{\"seat\":{},\"decl\":{},\"bidder\":{},\"viewer_internal\":{viewer_i},\
         \"d\":{d},\"completed\":{},\"route\":\"{}\",\"settled\":{},\
         \"controller_route\":\"{}\",\"tag\":\"{tag}\",\"consumed\":{consumed},\
         \"choice\":{},\"among\":[{}],\"fallback_opts\":[{}]}}",
        nums[0],
        nums[1],
        nums[2],
        st.completed,
        decision.route.label(),
        decision.route.settled(),
        decision.controller_route,
        decision.tile.index(),
        among.join(","),
        opts.join(","),
    );
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
    {
        let _ = writeln!(f, "{line}");
    }
}

fn decide(nums: &[usize], cfg: &Config) -> (usize, usize, u32, u32) {
    assert!(nums.len() >= 11, "request needs seat decl bidder h0..h6 n");
    let dcl = decl_of(nums[1]);
    let bidder_arena = nums[2];
    let n_history = nums[10];
    assert_eq!(
        nums.len(),
        11 + 2 * n_history,
        "history carries exactly n (actor, domino) pairs"
    );
    let pairs: Vec<(usize, usize)> = nums[11..].chunks_exact(2).map(|c| (c[0], c[1])).collect();
    let st = fold(dcl, bidder_arena, &pairs);

    let viewer_i = (nums[0] + st.r) % 4;
    let mut hand0 = DominoSet::EMPTY;
    for &raw in &nums[3..10] {
        assert!(
            hand0.insert(Domino::from_index(raw).expect("domino id 0..28")),
            "seven distinct dealt tiles"
        );
    }
    assert_eq!(hand0.len(), 7, "seven distinct dealt tiles");
    assert_eq!(
        viewer_i,
        st.leader.plus(st.trick_plays.len()).index(),
        "viewer is the seat to act"
    );
    let in_trick: DominoSet = st.trick_plays.iter().copied().collect();
    let hand = hand0.difference(st.prior_played).difference(in_trick);

    let state = DrivenState {
        decl: dcl,
        bid: 30,
        declaring_team: Team::T1,
        viewer_hand: hand,
        leader: st.leader,
        trick_plays: &st.trick_plays,
        banked: st.banked,
        prior_played: st.prior_played,
        voids: st.voids,
    };
    let d = (st.completed * 4 + st.trick_plays.len() + 1) as u64;
    let run_scope = format!("run:ctrl-bridge-{:07x}-b{bidder_arena}", hand0.bits());
    let decision = act(&state, &cfg.act, &run_scope, d, &delta_run_default());
    log_decision(nums, &st, viewer_i, d, &decision);

    // Reply in arena labels. Arena team0 = arena seats {0,2}; under
    // rotation r those are internal T1 exactly when r == 1.
    let leader_arena = (st.leader.index() + 4 - st.r) % 4;
    let (points0, points1) = if st.r == 1 {
        (st.banked[Team::T1.index()], st.banked[Team::T0.index()])
    } else {
        (st.banked[Team::T0.index()], st.banked[Team::T1.index()])
    };
    (decision.tile.index(), leader_arena, points0, points1)
}

/// The walt_bridge declare policy through the library solver: argmax
/// P(make 30) over the bidder's belief sample, saturation ties refined on
/// 4x fresh samples capped at 16x, never index-broken.
fn declare(nums: &[usize], cfg: &Config, full: bool) -> usize {
    assert_eq!(nums.len(), 8, "declare needs bidder h0..h6");
    let bidder_arena = nums[0];
    let r = if bidder_arena.is_multiple_of(2) { 1 } else { 0 };
    let bidder_i = (bidder_arena + r) % 4;
    let seat = Seat::from_index(bidder_i).expect("bid seat");
    let mut hand0: u32 = 0;
    for &raw in &nums[1..8] {
        hand0 |= 1u32 << Domino::from_index(raw).expect("domino id 0..28").index();
    }
    assert_eq!(hand0.count_ones(), 7, "seven distinct dealt tiles");

    let candidates: Vec<Decl> = if full {
        Decl::ALL.to_vec()
    } else {
        Decl::ALL[..7].to_vec()
    };
    let mut rng = SplitMix64(CTRL_DECLARE_SEED ^ mix(u64::from(hand0)) ^ mix(0xDEC1));
    let deadline = Deadline::after(Duration::from_secs(cfg.per_move_secs));
    let root = Key {
        played: 0,
        leader: bidder_i as u8,
        plays: Vec::new(),
        banked_t1: 0,
        banked_t0: 0,
        alive: 0,
    };
    let eval = |dcl: Decl, worlds: Vec<[u32; 4]>| -> BigRational {
        let sh = Arc::new(Shared::new(
            dcl,
            30,
            vec![cfg.act.fallback_n0],
            0,
            7,
            deadline,
        ));
        let solver =
            Solver::new(sh, seat, hand0, true, worlds, Vec::new(), Field::Level(0)).parallel();
        let mut best = BigRational::zero();
        for t in mask_bits(hand0) {
            let tile = Domino::from_index(usize::from(t)).expect("tile");
            let child = solver.child_after_play(&root, tile, 0);
            match solver.solve(&child) {
                Some(v) => {
                    if v > best {
                        best = v;
                    }
                }
                None => break,
            }
        }
        solver.flush_nodes();
        best
    };

    let worlds = sample_belief(bidder_i, hand0, 0, [7; 4], [0; 4], cfg.n_declare, &mut rng)
        .expect("a void-free frame is feasible: every deal of the unseen pool is lawful");
    let mut vals: Vec<(Decl, BigRational)> = candidates
        .iter()
        .map(|&dcl| (dcl, eval(dcl, worlds.clone())))
        .collect();
    let mut n_cur = cfg.n_declare;
    loop {
        let best = vals
            .iter()
            .map(|(_, v)| v.clone())
            .max()
            .expect("candidates");
        let tied: Vec<Decl> = vals
            .iter()
            .filter(|(_, v)| *v == best)
            .map(|(dcl, _)| *dcl)
            .collect();
        if tied.len() == 1 || n_cur >= cfg.n_declare * 16 {
            break;
        }
        n_cur *= 4;
        let worlds = sample_belief(bidder_i, hand0, 0, [7; 4], [0; 4], n_cur, &mut rng)
            .expect("a void-free frame is feasible: every deal of the unseen pool is lawful");
        for dcl in tied {
            let v = eval(dcl, worlds.clone());
            let slot = vals.iter_mut().find(|(x, _)| *x == dcl).expect("tied decl");
            slot.1 = v;
        }
    }
    let chosen = vals
        .into_iter()
        .reduce(|best, cand| if cand.1 > best.1 { cand } else { best })
        .expect("candidates")
        .0;
    arena_decl_id(chosen)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // argv first, then WALT_* env (a subprocess adapter launches bare).
    let pick = |i: usize, env: &str, default: usize| -> usize {
        args.get(i)
            .map(|s| s.parse().expect("integer argument"))
            .or_else(|| std::env::var(env).ok().map(|s| s.parse().expect(env)))
            .unwrap_or(default)
    };
    let env_only = |env: &str, default: usize| -> usize {
        std::env::var(env)
            .ok()
            .map(|s| s.parse().expect(env))
            .unwrap_or(default)
    };
    let cfg = Config {
        act: ActConfig {
            n_outer_frozen: env_only("WALT_CTRL_N_OUTER_FROZEN", 8) as u64,
            n0_frozen: env_only("WALT_CTRL_N0_FROZEN", 2) as u64,
            world_cap: pick(1, "WALT_CTRL_WORLD_CAP", 128) as u64,
            exact_cap: pick(2, "WALT_CTRL_EXACT_CAP", 2000) as u128,
            fallback_n_outer: pick(3, "WALT_N_OUTER", 200),
            fallback_n0: pick(4, "WALT_N0", 8),
        },
        n_declare: pick(5, "WALT_N_DECLARE", 100),
        per_move_secs: pick(6, "WALT_PER_MOVE", 120) as u64,
    };
    let declare_full = std::env::var("WALT_DECLARE_FULL").is_ok_and(|v| v == "1");

    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    for line in stdin.lock().lines() {
        let line = line.expect("stdin line");
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("declare ") {
            let nums: Vec<usize> = rest
                .split_whitespace()
                .map(|t| t.parse().expect("integer token"))
                .collect();
            let decl_id = declare(&nums, &cfg, declare_full);
            writeln!(out, "{decl_id}").expect("stdout write");
        } else {
            let nums: Vec<usize> = trimmed
                .split_whitespace()
                .map(|t| t.parse().expect("integer token"))
                .collect();
            let (chosen, leader, p0, p1) = decide(&nums, &cfg);
            writeln!(out, "{chosen} {leader} {p0} {p1}").expect("stdout write");
        }
        out.flush().expect("stdout flush");
    }
}
