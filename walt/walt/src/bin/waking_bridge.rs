//! waking_bridge — the WAKING SEAT (`solver::waking`) as a subprocess
//! player, plus a driven natural-play census mode. An exploratory,
//! non-normative surface; CE thread for the baseline, L2 thread for the
//! escalation. Instrument tooling; creates no receipts and no claims.
//! Boundary (CE-A7/§20.16): this is a VARIANT surface — nothing here
//! touches `controller_bridge`, `solver::act`'s policy, or any default.
//!
//! Speaks the SAME line protocol as controller_bridge.rs / walt_bridge.rs
//! (`rob:<path-to-waking_bridge>`), so plunge/mk5 can seat the waking
//! player with ZERO external-side changes:
//!
//! request:  `seat decl bidder h0 h1 h2 h3 h4 h5 h6 n (actor domino)*n`
//! reply:    `domino leader points0 points1` (identical grammar and seat
//!           rotation to walt_bridge.rs — the bidding team is internal T1;
//!           even arena bidders rotate every label by +1, rotated back in
//!           replies)
//! request:  `declare bidder h0 h1 h2 h3 h4 h5 h6`
//! reply:    a single declaration id (level-1 argmax P(make 30), the
//!           walt_bridge policy through the library solver; trump naming
//!           is not a waking-seat decision)
//!
//! Decisions run `solver::waking::WakingSeat::decide`: act's σ0 baseline
//! always, the hard-budgeted wake check, and wake-gated σ1 escalation —
//! declared epoch pair and wake rule in the `solver::waking` module docs.
//! One seat (one σ0/σ1 field-model pair) is held per hand, keyed by the
//! dealt hand, so the σ1 action cache amortizes across the hand's
//! decisions. Census JSONL goes to `$WALT_WAKING_LOG` (per-PID file
//! suffix, the WALT_CTRL_LOG pattern), one typed record per decision.
//!
//! Driven mode: `waking_bridge driven <out.jsonl> [n_hands] [knobs]` —
//! whole fresh-deal hands start to finish with the waking seat making
//! EVERY play decision at all four seats. Generic random deals from the
//! declared seed (`WAKING_DRIVEN_SEED`, mixed per hand index — no wall
//! clock, no ambient entropy), bidder rotating by hand index, trump
//! named by the existing level-1 auction policy, bid fixed at 30. One
//! census record per decision to the out path, plus one `kind:hand`
//! summary line per hand.
//!
//! Knobs (argv positional, then WALT_* env, then default — the
//! controller_bridge positions, so a harness swap is a path swap):
//!   1 WALT_CTRL_WORLD_CAP   128   act's controller cap in raw worlds
//!   2 WALT_CTRL_EXACT_CAP  2000   act's preroute-exact fiber ceiling
//!   3 WALT_N_OUTER          200   act's fallback level-1 outer count
//!   4 WALT_N0                 8   act's fallback level-1 inner count
//!   5 WALT_N_DECLARE        100   declare-path belief sample
//!   6 WALT_PER_MOVE         120   declare-path seconds budget
//! Env only: WALT_WAKING_N_OUTER_FROZEN 8 / WALT_WAKING_N0_FROZEN 2 (the
//! frozen candidates' declared schedule), WALT_WAKE_WORLDS 24 (wake
//! world budget), WALT_WAKE_EXACT_CAP 1024 (exact wake fiber cap),
//! WALT_WAKE_ESC_EXACT_CAP 4096 / WALT_WAKE_ESC_BASELINE 128 /
//! WALT_WAKE_ESC_E3 24 (escalation resource limits).
//!
//! δ_run = 1/100 per hand for act (unchanged); the waking layer's own
//! run budget is declared in `WakingConfig::live` (1/20) and split by
//! the same telescoping convention under `wake:`-prefixed scopes.
//! ESTIMATES, never receipts; not a P-A21 statement. No floats.

use std::io::{BufRead, Write};
use std::sync::Arc;
use std::time::Duration;

use num_rational::BigRational;
use num_traits::Zero;

use walt::rules::rules::{legal_plays, Trick};
use walt::rules::{ContextSet, Decl, Domino, DominoSet, Seat, Team};
use walt::solver::adaptive::DrivenState;
use walt::solver::waking::{WakingConfig, WakingSeat};
use walt::solver::{
    arena_decl_id, decl_of, mask_bits, mask_of, mix, sample_belief, Deadline, Field, Key, Shared,
    Solver, SplitMix64,
};

/// Frozen seed for the declare path's belief sampling (a distinct stream
/// from every other surface constant).
const WAKING_DECLARE_SEED: u64 = 0x7A3E_9B21_5C48_D6F1;

/// Frozen seed for driven-mode deals, mixed per hand index (declared
/// constant — no wall clock, no ambient entropy).
const WAKING_DRIVEN_SEED: u64 = 0x51EE_D42A_11FE_600D;

struct Config {
    waking: WakingConfig,
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

/// Census instrumentation: when WALT_WAKING_LOG is set, append the
/// decision's typed census record (per-PID file suffix).
fn log_census(line: &str) {
    let Ok(base) = std::env::var("WALT_WAKING_LOG") else {
        return;
    };
    let path = format!("{base}.{}", std::process::id());
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
    {
        let _ = writeln!(f, "{line}");
    }
}

/// One hand's seat, keyed by the dealt hand identity so the σ0/σ1 field
/// caches persist across the hand's decisions and reset on a new deal.
struct HeldSeat {
    key: u64,
    seat: WakingSeat,
}

fn decide(nums: &[usize], cfg: &Config, held: &mut Option<HeldSeat>) -> (usize, usize, u32, u32) {
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
    let run_scope = format!("run:waking-bridge-{:07x}-b{bidder_arena}", hand0.bits());

    // One seat per hand: the (hand, declaration, bidder) key changes
    // exactly when a new hand starts, and with it the field caches.
    let key =
        mix(u64::from(hand0.bits())) ^ mix(arena_decl_id(dcl) as u64) ^ mix(bidder_arena as u64);
    let refresh = held.as_ref().is_none_or(|h| h.key != key);
    if refresh {
        *held = Some(HeldSeat {
            key,
            seat: WakingSeat::new(cfg.waking.clone()),
        });
    }
    let seat = &held.as_ref().expect("a held seat").seat;
    let decision = seat.decide(&state, &run_scope, d);
    log_census(&decision.census.to_jsonl(&run_scope));

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
/// 4x fresh samples capped at 16x, never index-broken. `bidder_i` is the
/// INTERNAL seat.
fn declare_internal(bidder_i: usize, hand0: u32, cfg: &Config, full: bool) -> Decl {
    let seat = Seat::from_index(bidder_i).expect("bid seat");
    let candidates: Vec<Decl> = if full {
        Decl::ALL.to_vec()
    } else {
        Decl::ALL[..7].to_vec()
    };
    let mut rng = SplitMix64(WAKING_DECLARE_SEED ^ mix(u64::from(hand0)) ^ mix(0xDEC1));
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
            vec![cfg.waking.act.fallback_n0],
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

    let worlds = sample_belief(bidder_i, hand0, 0, [7; 4], [0; 4], cfg.n_declare, &mut rng);
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
        let worlds = sample_belief(bidder_i, hand0, 0, [7; 4], [0; 4], n_cur, &mut rng);
        for dcl in tied {
            let v = eval(dcl, worlds.clone());
            let slot = vals.iter_mut().find(|(x, _)| *x == dcl).expect("tied decl");
            slot.1 = v;
        }
    }
    vals.into_iter()
        .reduce(|best, cand| if cand.1 > best.1 { cand } else { best })
        .expect("candidates")
        .0
}

fn declare(nums: &[usize], cfg: &Config, full: bool) -> usize {
    assert_eq!(nums.len(), 8, "declare needs bidder h0..h6");
    let bidder_arena = nums[0];
    let r = if bidder_arena.is_multiple_of(2) { 1 } else { 0 };
    let bidder_i = (bidder_arena + r) % 4;
    let mut hand0: u32 = 0;
    for &raw in &nums[1..8] {
        hand0 |= 1u32 << Domino::from_index(raw).expect("domino id 0..28").index();
    }
    assert_eq!(hand0.count_ones(), 7, "seven distinct dealt tiles");
    arena_decl_id(declare_internal(bidder_i, hand0, cfg, full))
}

// ---------------------------------------------------------------------------
// Driven mode — whole hands, the waking seat at all four chairs.
// ---------------------------------------------------------------------------

/// A generic random deal from the declared per-hand stream: Fisher-Yates
/// over the 28 tiles, seven to a seat. Deterministic in the hand index.
fn driven_deal(g: usize) -> [DominoSet; 4] {
    let mut rng = SplitMix64(WAKING_DRIVEN_SEED ^ mix(g as u64));
    let mut tiles: Vec<usize> = (0..28).collect();
    for i in (1..tiles.len()).rev() {
        let j = usize::try_from(rng.next_u64() % (i as u64 + 1)).expect("index fits");
        tiles.swap(i, j);
    }
    core::array::from_fn(|s| {
        tiles[s * 7..(s + 1) * 7]
            .iter()
            .map(|&i| Domino::from_index(i).expect("tile id 0..28"))
            .collect()
    })
}

fn set_json(s: DominoSet) -> String {
    let parts: Vec<String> = s.iter().map(|d| d.index().to_string()).collect();
    format!("[{}]", parts.join(","))
}

/// Drive one fresh hand start to finish, every play decision at every
/// seat made by ONE waking seat (its field caches shared hand-wide).
/// Returns the JSONL lines (census records plus one hand summary).
fn drive_hand(g: usize, cfg: &Config, out: &mut impl Write) {
    let deal = driven_deal(g);
    let bidder = Seat::from_index(g % 4).expect("seat 0..4");
    let declaring_team = bidder.team();
    let decl = declare_internal(bidder.index(), mask_of(deal[bidder.index()]), cfg, false);
    let bid = 30u32;
    let seat_engine = WakingSeat::new(cfg.waking.clone());
    let run_scope = format!("run:waking-driven-h{g}");
    let ctx = format!("driven-h{g}");

    let mut hands = deal;
    let mut prior_played = DominoSet::EMPTY;
    let mut trick_plays: Vec<Domino> = Vec::new();
    let mut leader = bidder;
    let mut banked = [0u32; 2];
    let mut voids = [ContextSet::EMPTY; 4];
    let mut decisions = 0u64;
    for _trick_no in 1..=7usize {
        for _ply in 0..4usize {
            let seat = leader.plus(trick_plays.len());
            let hand = hands[seat.index()];
            let led = trick_plays.first().map(|t| decl.led_context(*t));
            let legal = legal_plays(decl, hand, led);
            assert!(!legal.is_empty(), "a seat to move holds a legal tile");
            let d = (prior_played.len() + trick_plays.len() + 1) as u64;
            let state = DrivenState {
                decl,
                bid,
                declaring_team,
                viewer_hand: hand,
                leader,
                trick_plays: &trick_plays,
                banked,
                prior_played,
                voids,
            };
            let decision = seat_engine.decide(&state, &run_scope, d);
            let choice = decision.tile;
            writeln!(out, "{}", decision.census.to_jsonl(&ctx)).expect("the output file writes");
            decisions += 1;
            assert!(legal.contains(choice), "the chosen tile is legal");
            if let Some(led) = led {
                if !decl.follows(choice, led) {
                    voids[seat.index()].insert(led);
                }
            }
            assert!(
                hands[seat.index()].remove(choice),
                "the chosen tile is held"
            );
            trick_plays.push(choice);
        }
        let doms: [Domino; 4] = core::array::from_fn(|i| trick_plays[i]);
        let trick = Trick::new(leader, doms).expect("four distinct tiles");
        let winner = trick.winner(decl);
        banked[winner.team().index()] += trick.points();
        for t in doms {
            prior_played.insert(t);
        }
        leader = winner;
        trick_plays.clear();
    }
    assert_eq!(banked[0] + banked[1], 42, "all points banked");
    let made = banked[declaring_team.index()] >= bid;
    writeln!(
        out,
        "{{\"kind\":\"hand\",\"hand\":{g},\"decl\":{},\"bid\":{bid},\
         \"declaring_team\":{},\"bidder\":{},\"deal\":[{},{},{},{}],\
         \"made\":{made},\"banked\":[{},{}],\"decisions\":{decisions},\
         \"sigma1_cache\":{}}}",
        arena_decl_id(decl),
        declaring_team.index(),
        bidder.index(),
        set_json(deal[0]),
        set_json(deal[1]),
        set_json(deal[2]),
        set_json(deal[3]),
        banked[0],
        banked[1],
        seat_engine.sigma1_cache_len(),
    )
    .expect("the output file writes");
    eprintln!(
        "waking_bridge: driven hand {g} done ({decisions} decisions, made={made}, \
         banked=[{},{}])",
        banked[0], banked[1]
    );
}

fn driven(out_path: &str, n_hands: usize, cfg: &Config) {
    eprintln!(
        "waking_bridge: driven {n_hands} hands; act cap {}, wake budget {}, \
         wake exact cap {}, escalation caps {}/{}/{}",
        cfg.waking.act.world_cap,
        cfg.waking.wake_world_budget,
        cfg.waking.wake_exact_fiber_cap,
        cfg.waking.escalation_exact_fiber_cap,
        cfg.waking.escalation_baseline_prefix,
        cfg.waking.escalation_e3_prefix,
    );
    let mut out = std::fs::File::create(out_path).expect("the output file opens");
    for g in 0..n_hands {
        drive_hand(g, cfg, &mut out);
        out.flush().expect("the output file flushes");
    }
    eprintln!("waking_bridge: wrote {out_path}");
}

// ---------------------------------------------------------------------------
// Entry.
// ---------------------------------------------------------------------------

fn env_u64(env: &str, default: u64) -> u64 {
    std::env::var(env)
        .ok()
        .map(|s| s.parse().expect(env))
        .unwrap_or(default)
}

fn config(args: &[String], skip: usize) -> Config {
    // argv first, then WALT_* env (a subprocess adapter launches bare).
    let pick = |i: usize, env: &str, default: usize| -> usize {
        args.get(skip + i)
            .map(|s| s.parse().expect("integer argument"))
            .or_else(|| std::env::var(env).ok().map(|s| s.parse().expect(env)))
            .unwrap_or(default)
    };
    let mut waking = WakingConfig::live();
    waking.act.n_outer_frozen = env_u64("WALT_WAKING_N_OUTER_FROZEN", 8);
    waking.act.n0_frozen = env_u64("WALT_WAKING_N0_FROZEN", 2);
    waking.act.world_cap = pick(1, "WALT_CTRL_WORLD_CAP", 128) as u64;
    waking.act.exact_cap = pick(2, "WALT_CTRL_EXACT_CAP", 2000) as u128;
    waking.act.fallback_n_outer = pick(3, "WALT_N_OUTER", 200);
    waking.act.fallback_n0 = pick(4, "WALT_N0", 8);
    waking.wake_world_budget = env_u64("WALT_WAKE_WORLDS", 24);
    waking.wake_exact_fiber_cap = u128::from(env_u64("WALT_WAKE_EXACT_CAP", 1024));
    waking.escalation_exact_fiber_cap = u128::from(env_u64("WALT_WAKE_ESC_EXACT_CAP", 4096));
    waking.escalation_baseline_prefix = env_u64("WALT_WAKE_ESC_BASELINE", 128);
    waking.escalation_e3_prefix = env_u64("WALT_WAKE_ESC_E3", 24);
    Config {
        waking,
        n_declare: pick(5, "WALT_N_DECLARE", 100),
        per_move_secs: pick(6, "WALT_PER_MOVE", 120) as u64,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("driven") {
        let out_path = args
            .get(2)
            .cloned()
            .unwrap_or_else(|| "waking-driven.jsonl".to_string());
        let n_hands: usize = args
            .get(3)
            .map(|v| v.parse().expect("hand count"))
            .unwrap_or(2);
        let cfg = config(&args, 3);
        driven(&out_path, n_hands, &cfg);
        return;
    }

    let cfg = config(&args, 0);
    let declare_full = std::env::var("WALT_DECLARE_FULL").is_ok_and(|v| v == "1");
    let mut held: Option<HeldSeat> = None;

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
            let (chosen, leader, p0, p1) = decide(&nums, &cfg, &mut held);
            writeln!(out, "{chosen} {leader} {p0} {p1}").expect("stdout write");
        }
        out.flush().expect("stdout flush");
    }
}
