//! granrun — the Gran-anchor probe runner. EXPLORATORY tier; instrument
//! tooling that creates no receipts and backs no claim above exploratory.
//!
//! Boundary (CE-A7/§20.16): a VARIANT surface. Nothing here touches
//! `solver::act`'s policy, `controller_bridge`, `walt_bridge`, or any
//! default; `waking_bridge` is not modified either — this binary drives
//! `solver::waking::WakingSeat` directly under the SAME declared epoch
//! the `probes/waking` profile declares, so the two censuses compose.
//!
//! Anchors (`walt/probes/gran/`) are hand transcriptions of Plunge
//! screenshots, so every one of them is validated mechanically before it
//! is used: `rules::replay::replay_hand` re-derives the deal, every
//! follow's legality, every trick winner, every trick's points, the
//! declaring total and the made/set verdict from the tiles alone.
//!
//! Modes:
//!   granrun validate <fixture.txt>
//!       Parse and replay the anchor; print the derived deal and the
//!       per-trick derivation. Non-zero exit on any disagreement.
//!   granrun replay <fixture.txt> <seat> <out.jsonl>
//!       The waking seat sits in `seat` (e.g. `S2`) from the OPENING
//!       LEAD; the other three seats play exactly what the record says.
//!       One census record per waking decision, plus one `kind:compare`
//!       line per decision carrying the recorded play beside the
//!       waking choice.
//!   granrun driven <fixture.txt> <out.jsonl>
//!       The waking seat at all four chairs from the anchor's deal, the
//!       whole hand, in the `waking_bridge driven` census record shape.
//!
//! Epoch (identical to `waking_bridge`'s defaults and to
//! `probes/waking/README.md`): σ0 = `Level0 { n0 = 2 }`, σ1 =
//! `Level1 { n_outer = 4, n0 = 2 }`, frozen candidate schedule [8, 2],
//! `ActConfig::interactive` (world cap 128, exact cap 2000, fallback
//! 200x8), wake budget 24 paired worlds, exact wake cap 1024,
//! escalation caps 4096/128/24. Same WALT_* env knobs as the bridge.
//!
//! ESTIMATES, never receipts; not a P-A21 statement. No floats.

use std::io::Write;
use std::path::Path;

use walt::rules::receipt::{self, ReceiptHand};
use walt::rules::replay::{deal as receipt_deal, replay_hand};
use walt::rules::rules::{legal_plays, Trick};
use walt::rules::{ContextSet, Decl, Domino, DominoSet, Seat};
use walt::solver::adaptive::DrivenState;
use walt::solver::arena_decl_id;
use walt::solver::waking::{WakingConfig, WakingSeat};

fn env_u64(env: &str, default: u64) -> u64 {
    std::env::var(env)
        .ok()
        .map(|s| s.parse().expect(env))
        .unwrap_or(default)
}

fn env_usize(env: &str, default: usize) -> usize {
    std::env::var(env)
        .ok()
        .map(|s| s.parse().expect(env))
        .unwrap_or(default)
}

/// The declared epoch, field for field with `waking_bridge`'s `config()`
/// defaults so the two surfaces' censuses compose.
fn waking_config() -> WakingConfig {
    let mut waking = WakingConfig::live();
    waking.act.n_outer_frozen = env_u64("WALT_WAKING_N_OUTER_FROZEN", 8);
    waking.act.n0_frozen = env_u64("WALT_WAKING_N0_FROZEN", 2);
    waking.act.world_cap = env_u64("WALT_CTRL_WORLD_CAP", 128);
    waking.act.exact_cap = u128::from(env_u64("WALT_CTRL_EXACT_CAP", 2000));
    waking.act.fallback_n_outer = env_usize("WALT_N_OUTER", 200);
    waking.act.fallback_n0 = env_usize("WALT_N0", 8);
    waking.wake_world_budget = env_u64("WALT_WAKE_WORLDS", 24);
    waking.wake_exact_fiber_cap = u128::from(env_u64("WALT_WAKE_EXACT_CAP", 1024));
    waking.escalation_exact_fiber_cap = u128::from(env_u64("WALT_WAKE_ESC_EXACT_CAP", 4096));
    waking.escalation_baseline_prefix = env_u64("WALT_WAKE_ESC_BASELINE", 128);
    waking.escalation_e3_prefix = env_u64("WALT_WAKE_ESC_E3", 24);
    waking
}

fn load(path: &str) -> ReceiptHand {
    let parsed = receipt::parse_file(Path::new(path)).expect("the anchor parses");
    assert_eq!(parsed.hands.len(), 1, "an anchor fixture holds ONE hand");
    parsed.hands.into_iter().next().expect("one hand")
}

fn seat_of(tok: &str) -> Seat {
    let n: usize = tok
        .strip_prefix('S')
        .expect("a seat like S2")
        .parse()
        .expect("a seat index");
    Seat::from_index(n).expect("a seat index 0..4")
}

fn set_json(s: DominoSet) -> String {
    let parts: Vec<String> = s.iter().map(|d| d.index().to_string()).collect();
    format!("[{}]", parts.join(","))
}

// -------------------------------------------------------------------------
// validate — the mechanical check the transcription must survive.
// -------------------------------------------------------------------------

fn validate(path: &str) {
    let hand = load(path);
    let r = match replay_hand(&hand) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("granrun: ANCHOR FAILS THE RULES LAYER: {e}");
            std::process::exit(1);
        }
    };
    println!("anchor {path}");
    println!(
        "  bidder {} bid {} declaration {} declaring {}",
        hand.bidder, hand.bid_points, hand.decl, hand.declaring_team
    );
    for s in Seat::ALL {
        let tiles: Vec<String> = r.deal[s.index()].iter().map(|d| d.to_string()).collect();
        println!(
            "  deal {s}: {} ({} tiles)",
            tiles.join(" "),
            r.deal[s.index()].len()
        );
    }
    for (i, (w, p)) in r.trick_winners.iter().zip(&r.trick_points).enumerate() {
        println!("  trick {} -> {w} +{p}", i + 1);
    }
    println!(
        "  team points T0 {} - {} T1; tricks T0 {} - {} T1",
        r.team_points[0], r.team_points[1], r.team_tricks[0], r.team_tricks[1]
    );
    println!(
        "  declaring {} took {} against bid {} -> {}",
        hand.declaring_team,
        r.team_points[hand.declaring_team.index()],
        hand.bid_points,
        if hand.made { "made" } else { "set" }
    );
    println!("VALIDATED: 28 distinct tiles, 7 per seat, every follow legal, every winner and every trick's points re-derived, totals and verdict agree.");
}

// -------------------------------------------------------------------------
// replay — the waking seat in one chair against the recorded line.
// -------------------------------------------------------------------------

fn replay(path: &str, seat_tok: &str, out_path: &str) {
    let hand = load(path);
    replay_hand(&hand).expect("the anchor validates before it is played");
    let walt_seat = seat_of(seat_tok);
    let cfg = waking_config();
    let decl = hand.decl;
    let declaring_team = hand.declaring_team;
    let bid = hand.bid_points;
    let deal = receipt_deal(&hand).expect("the anchor's deal");
    let ctx = format!("gran-replay-{}", walt_seat);
    let run_scope = format!("run:gran-replay-{}", walt_seat);
    let engine = WakingSeat::new(cfg.clone());

    let mut out = std::fs::File::create(out_path).expect("the output file opens");
    writeln!(
        out,
        "{{\"kind\":\"config\",\"mode\":\"replay\",\"anchor\":\"{path}\",\
         \"walt_seat\":{},\"decl\":{},\"bid\":{bid},\"declaring_team\":{},\
         \"bidder\":{},\"deal\":[{},{},{},{}],\"sigma0\":\"Level0{{n0={}}}\",\
         \"sigma1\":\"Level1{{n_outer={},n0={}}}\",\"world_cap\":{},\
         \"wake_world_budget\":{},\"wake_exact_fiber_cap\":\"{}\"}}",
        walt_seat.index(),
        arena_decl_id(decl),
        declaring_team.index(),
        hand.bidder.index(),
        set_json(deal[0]),
        set_json(deal[1]),
        set_json(deal[2]),
        set_json(deal[3]),
        cfg.act.n0_frozen,
        cfg.sigma1_n_outer,
        cfg.sigma1_n0,
        cfg.act.world_cap,
        cfg.wake_world_budget,
        cfg.wake_exact_fiber_cap,
    )
    .expect("the output file writes");

    let mut hands = deal;
    let mut prior_played = DominoSet::EMPTY;
    let mut trick_plays: Vec<Domino> = Vec::new();
    let mut leader = hand.bidder;
    let mut banked = [0u32; 2];
    let mut voids = [ContextSet::EMPTY; 4];
    let mut agreements = 0u32;
    let mut decisions = 0u32;

    for rec in &hand.tricks {
        assert_eq!(rec.plays[0].0, leader, "the record's leader");
        for (k, (actor, recorded)) in rec.plays.into_iter().enumerate() {
            assert_eq!(actor, leader.plus(k), "turn order");
            let held = hands[actor.index()];
            let led = trick_plays.first().map(|t| decl.led_context(*t));
            let legal = legal_plays(decl, held, led);
            assert!(legal.contains(recorded), "the recorded play is legal");

            if actor == walt_seat {
                let d = (prior_played.len() + trick_plays.len() + 1) as u64;
                let state = DrivenState {
                    decl,
                    bid,
                    declaring_team,
                    viewer_hand: held,
                    leader,
                    trick_plays: &trick_plays,
                    banked,
                    prior_played,
                    voids,
                };
                let wall = std::time::Instant::now();
                let decision = engine.decide(&state, &run_scope, d);
                let wall_us = u64::try_from(wall.elapsed().as_micros()).expect("micros fit");
                assert!(legal.contains(decision.tile), "the waking choice is legal");
                writeln!(out, "{}", decision.census.to_jsonl(&ctx)).expect("the output writes");
                let agreed_with_record = decision.tile == recorded;
                if agreed_with_record {
                    agreements += 1;
                }
                decisions += 1;
                let legal_list: Vec<String> =
                    legal.iter().map(|t| t.index().to_string()).collect();
                writeln!(
                    out,
                    "{{\"kind\":\"compare\",\"ctx\":\"{ctx}\",\"d\":{d},\"trick\":{},\
                     \"record_played\":{},\"waking_played\":{},\"sigma0\":{},\
                     \"agreed_with_record\":{agreed_with_record},\"legal\":[{}],\
                     \"wall_us\":{wall_us}}}",
                    rec.number,
                    recorded.index(),
                    decision.tile.index(),
                    decision.census.sigma0.index(),
                    legal_list.join(","),
                )
                .expect("the output writes");
                out.flush().expect("the output flushes");
                eprintln!(
                    "granrun: trick {} d{d}: record {recorded}, waking {} (sigma0 {}), \
                     path {}, wake {:?}, {wall_us} us",
                    rec.number,
                    decision.tile,
                    decision.census.sigma0,
                    decision.census.path.tag(),
                    decision.census.wake_kind,
                );
            }

            // The line played is ALWAYS the record's: this mode measures
            // the waking seat's choice at the real decision points, it
            // does not re-play the hand.
            if let Some(led) = led {
                if !decl.follows(recorded, led) {
                    voids[actor.index()].insert(led);
                }
            }
            assert!(hands[actor.index()].remove(recorded), "the tile is held");
            trick_plays.push(recorded);
        }
        let doms: [Domino; 4] = core::array::from_fn(|i| trick_plays[i]);
        let trick = Trick::new(leader, doms).expect("four distinct tiles");
        let winner = trick.winner(decl);
        assert_eq!(winner, rec.winner, "derived winner matches the record");
        banked[winner.team().index()] += trick.points();
        for t in doms {
            prior_played.insert(t);
        }
        leader = winner;
        trick_plays.clear();
    }

    writeln!(
        out,
        "{{\"kind\":\"replay-summary\",\"ctx\":\"{ctx}\",\"decisions\":{decisions},\
         \"agreed_with_record\":{agreements},\"sigma1_cache\":{}}}",
        engine.sigma1_cache_len()
    )
    .expect("the output writes");
    out.flush().expect("the output flushes");
    eprintln!("granrun: replay wrote {out_path} ({decisions} decisions, {agreements} matched the record)");
}

// -------------------------------------------------------------------------
// driven — the waking seat at all four chairs from the anchor's deal.
// -------------------------------------------------------------------------

fn driven(path: &str, out_path: &str) {
    let hand = load(path);
    replay_hand(&hand).expect("the anchor validates before it is played");
    let cfg = waking_config();
    let decl: Decl = hand.decl;
    let bidder = hand.bidder;
    let declaring_team = hand.declaring_team;
    let bid = hand.bid_points;
    let deal = receipt_deal(&hand).expect("the anchor's deal");
    let ctx = "gran-driven".to_string();
    let run_scope = "run:gran-driven".to_string();
    let engine = WakingSeat::new(cfg.clone());

    let mut out = std::fs::File::create(out_path).expect("the output file opens");
    writeln!(
        out,
        "{{\"kind\":\"config\",\"mode\":\"driven\",\"anchor\":\"{path}\",\
         \"decl\":{},\"bid\":{bid},\"declaring_team\":{},\"bidder\":{},\
         \"deal\":[{},{},{},{}]}}",
        arena_decl_id(decl),
        declaring_team.index(),
        bidder.index(),
        set_json(deal[0]),
        set_json(deal[1]),
        set_json(deal[2]),
        set_json(deal[3]),
    )
    .expect("the output writes");

    let mut hands = deal;
    let mut prior_played = DominoSet::EMPTY;
    let mut trick_plays: Vec<Domino> = Vec::new();
    let mut leader = bidder;
    let mut banked = [0u32; 2];
    let mut voids = [ContextSet::EMPTY; 4];
    let mut decisions = 0u64;

    for trick_no in 1..=7usize {
        for _ply in 0..4usize {
            let seat = leader.plus(trick_plays.len());
            let held = hands[seat.index()];
            let led = trick_plays.first().map(|t| decl.led_context(*t));
            let legal = legal_plays(decl, held, led);
            assert!(!legal.is_empty(), "a seat to move holds a legal tile");
            let d = (prior_played.len() + trick_plays.len() + 1) as u64;
            let state = DrivenState {
                decl,
                bid,
                declaring_team,
                viewer_hand: held,
                leader,
                trick_plays: &trick_plays,
                banked,
                prior_played,
                voids,
            };
            let decision = engine.decide(&state, &run_scope, d);
            let choice = decision.tile;
            writeln!(out, "{}", decision.census.to_jsonl(&ctx)).expect("the output writes");
            out.flush().expect("the output flushes");
            decisions += 1;
            assert!(legal.contains(choice), "the chosen tile is legal");
            eprintln!(
                "granrun: driven trick {trick_no} d{d} seat {seat}: {choice} \
                 (sigma0 {}), path {}",
                decision.census.sigma0,
                decision.census.path.tag()
            );
            if let Some(led) = led {
                if !decl.follows(choice, led) {
                    voids[seat.index()].insert(led);
                }
            }
            assert!(hands[seat.index()].remove(choice), "the chosen tile is held");
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
        "{{\"kind\":\"hand\",\"ctx\":\"{ctx}\",\"anchor\":\"{path}\",\"decl\":{},\
         \"bid\":{bid},\"declaring_team\":{},\"bidder\":{},\"made\":{made},\
         \"banked\":[{},{}],\"decisions\":{decisions},\"sigma1_cache\":{}}}",
        arena_decl_id(decl),
        declaring_team.index(),
        bidder.index(),
        banked[0],
        banked[1],
        engine.sigma1_cache_len(),
    )
    .expect("the output writes");
    out.flush().expect("the output flushes");
    eprintln!(
        "granrun: driven wrote {out_path} ({decisions} decisions, made={made}, \
         banked=[{},{}])",
        banked[0], banked[1]
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("validate") => validate(args.get(2).expect("granrun validate <fixture>")),
        Some("replay") => replay(
            args.get(2).expect("granrun replay <fixture> <seat> <out>"),
            args.get(3).expect("granrun replay <fixture> <seat> <out>"),
            args.get(4).expect("granrun replay <fixture> <seat> <out>"),
        ),
        Some("driven") => driven(
            args.get(2).expect("granrun driven <fixture> <out>"),
            args.get(3).expect("granrun driven <fixture> <out>"),
        ),
        _ => {
            eprintln!(
                "usage: granrun validate <fixture.txt>\n       \
                 granrun replay <fixture.txt> <seat> <out.jsonl>\n       \
                 granrun driven <fixture.txt> <out.jsonl>"
            );
            std::process::exit(2);
        }
    }
}
