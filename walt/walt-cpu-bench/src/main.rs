//! Native, four-seat, full-hand timing of Walt's actual partnership wire path.
//! The referee owns all hands; each solver call receives only its own original
//! hand, contract, public history, and a deal-independent public seed.

use serde_json::{json, Value};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
#[cfg(feature = "compact-dice")]
use std::sync::atomic::Ordering;
use std::time::Instant;
use walt::solver::{partnership_wire, Shared};

#[cfg(feature = "mac-qos")]
mod mac_qos;

const G1: [[u8; 7]; 4] = [
    [0, 4, 9, 14, 22, 24, 27],
    [2, 6, 13, 16, 19, 21, 26],
    [1, 5, 11, 12, 17, 23, 25],
    [3, 7, 8, 10, 15, 18, 20],
];

#[derive(Clone)]
struct Config {
    profile: String,
    fixture: String,
    deal_seed: u64,
    public_seed: u64,
    repeats: usize,
    decl: u8,
    bid: u8,
    bidder: usize,
    n: usize,
    n0: usize,
    n1: usize,
    budget_ms: u64,
    cache: String,
    mac_qos: String,
    output: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            profile: "partner".into(),
            fixture: "g1".into(),
            deal_seed: 420601,
            public_seed: 420600,
            repeats: 1,
            decl: 6,
            bid: 30,
            bidder: 0,
            n: 40,
            n0: 8,
            n1: 2,
            budget_ms: 20_000,
            cache: "cold".into(),
            mac_qos: "default".into(),
            output: None,
        }
    }
}

fn parse() -> Result<Config, String> {
    let mut cfg = Config::default();
    let mut args = env::args().skip(1);
    while let Some(flag) = args.next() {
        if flag == "--help" || flag == "-h" {
            println!("walt-cpu-bench [--profile partner|all-l1] [--fixture g1|shuffle] [--deal-seed N] [--public-seed N] [--repeats N] [--decl 0..7|9] [--bid 30..42] [--bidder 0..3] [--n N --n0 N --n1 N] [--budget-ms 0..20000] [--cache cold|carry] [--mac-qos default|interactive] [--output PATH]\nDefaults: four-seat L2 Partner, fixed/voidless 40/8/2, frozen G1, 28 plays. --cache carry uses the wire API's guarded modeled-policy cache; cold matches current native stream behavior. Interactive QoS requires the runner-only mac-qos feature; setup is included in the first game timer. Progress is JSONL on stderr; complete or refused receipt is JSON on stdout and optional --output.");
            std::process::exit(0);
        }
        let value = args
            .next()
            .ok_or_else(|| format!("missing value for {flag}"))?;
        match flag.as_str() {
            "--profile" => cfg.profile = value,
            "--fixture" => cfg.fixture = value,
            "--deal-seed" => cfg.deal_seed = value.parse().map_err(|_| "invalid deal seed")?,
            "--public-seed" => {
                cfg.public_seed = value.parse().map_err(|_| "invalid public seed")?
            }
            "--repeats" => cfg.repeats = value.parse().map_err(|_| "invalid repeats")?,
            "--decl" => cfg.decl = value.parse().map_err(|_| "invalid declaration")?,
            "--bid" => cfg.bid = value.parse().map_err(|_| "invalid bid")?,
            "--bidder" => cfg.bidder = value.parse().map_err(|_| "invalid bidder")?,
            "--n" => cfg.n = value.parse().map_err(|_| "invalid n")?,
            "--n0" => cfg.n0 = value.parse().map_err(|_| "invalid n0")?,
            "--n1" => cfg.n1 = value.parse().map_err(|_| "invalid n1")?,
            "--budget-ms" => cfg.budget_ms = value.parse().map_err(|_| "invalid budget")?,
            "--cache" => cfg.cache = value,
            "--mac-qos" => cfg.mac_qos = value,
            "--output" => cfg.output = Some(value.into()),
            _ => return Err(format!("unknown option {flag}")),
        }
    }
    if !matches!(cfg.profile.as_str(), "partner" | "all-l1")
        || !matches!(cfg.fixture.as_str(), "g1" | "shuffle")
        || !matches!(cfg.cache.as_str(), "cold" | "carry")
        || !matches!(cfg.mac_qos.as_str(), "default" | "interactive")
        || !matches!(cfg.decl, 0..=7 | 9)
        || !(30..=42).contains(&cfg.bid)
        || cfg.bidder > 3
        || !(1..=640).contains(&cfg.n)
        || !(1..=64).contains(&cfg.n0)
        || !(1..=64).contains(&cfg.n1)
        || cfg.budget_ms > 20_000
        || cfg.repeats == 0
    {
        return Err("invalid configuration; use --help".into());
    }
    if cfg.mac_qos == "interactive" && !cfg!(feature = "mac-qos") {
        return Err("--mac-qos interactive requires a build with the mac-qos feature".into());
    }
    Ok(cfg)
}

fn pip(tile: u8) -> (u8, u8) {
    let hi = (0..7).find(|&h| tile <= h * (h + 1) / 2 + h).unwrap();
    (hi, tile - hi * (hi + 1) / 2)
}

fn trump(tile: u8, decl: u8) -> bool {
    let (hi, lo) = pip(tile);
    if decl < 7 {
        hi == decl || lo == decl
    } else {
        decl == 7 && hi == lo
    }
}

fn context(tile: u8, decl: u8) -> u8 {
    if trump(tile, decl) {
        7
    } else {
        pip(tile).0
    }
}

fn follows(tile: u8, led: u8, decl: u8) -> bool {
    if led == 7 {
        trump(tile, decl)
    } else {
        let (hi, lo) = pip(tile);
        !trump(tile, decl) && (hi == led || lo == led)
    }
}

fn legal(hand: &[u8], trick: &[(usize, u8)], decl: u8) -> Vec<u8> {
    if trick.is_empty() {
        return hand.to_vec();
    }
    let led = context(trick[0].1, decl);
    let following: Vec<u8> = hand
        .iter()
        .copied()
        .filter(|&t| follows(t, led, decl))
        .collect();
    if following.is_empty() {
        hand.to_vec()
    } else {
        following
    }
}

fn winner(trick: &[(usize, u8)], decl: u8) -> usize {
    let led = context(trick[0].1, decl);
    trick
        .iter()
        .max_by_key(|(_, t)| {
            let (hi, lo) = pip(*t);
            let tier = if trump(*t, decl) {
                2
            } else if follows(*t, led, decl) {
                1
            } else {
                0
            };
            let rank = if hi == lo && decl == 7 {
                hi
            } else if hi == lo {
                12
            } else {
                hi + lo
            };
            (tier, rank)
        })
        .unwrap()
        .0
}

fn trick_points(trick: &[(usize, u8)]) -> u8 {
    1 + trick
        .iter()
        .map(|(_, t)| {
            let (hi, lo) = pip(*t);
            if hi + lo == 5 || hi + lo == 10 {
                hi + lo
            } else {
                0
            }
        })
        .sum::<u8>()
}

fn next_u64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e3779b97f4a7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}

fn deal(cfg: &Config, repetition: usize) -> [[u8; 7]; 4] {
    if cfg.fixture == "g1" {
        return G1;
    }
    let mut tiles: Vec<u8> = (0..28).collect();
    let mut state = cfg.deal_seed.wrapping_add(repetition as u64);
    for i in (1..tiles.len()).rev() {
        let j = (next_u64(&mut state) % (i as u64 + 1)) as usize;
        tiles.swap(i, j);
    }
    let mut hands = [[0u8; 7]; 4];
    for seat in 0..4 {
        hands[seat].copy_from_slice(&tiles[seat * 7..seat * 7 + 7]);
        hands[seat].sort();
    }
    hands
}

fn request(cfg: &Config, seat: usize, own: &[u8; 7], plays: &[u8]) -> String {
    let mut out = format!(
        "{}\ndecl {}\nbid {}\nseat {}\nbidder {}\nhand",
        cfg.profile, cfg.decl, cfg.bid, seat, cfg.bidder
    );
    for tile in own {
        out.push_str(&format!(" {tile}"));
    }
    out.push_str("\nplays");
    for v in plays {
        out.push_str(&format!(" {v}"));
    }
    out.push_str(&format!("\nseed {}\nn {}\nn0 {}\nn1 {}\nbudget_ms {}\ninner_belief 0\nselection 0\nmodeled_selection 0\n", cfg.public_seed, cfg.n, cfg.n0, cfg.n1, cfg.budget_ms));
    out
}

fn solver_settings(cfg: &Config) -> Value {
    json!({"mode":cfg.profile,"n":cfg.n,"n0":cfg.n0,"n1":cfg.n1,
        "budget_ms":cfg.budget_ms,"inner_belief":0,"selection":0,
        "modeled_selection":0})
}

fn digest_add(mut hash: u64, bytes: &[u8]) -> u64 {
    // Stable diagnostic FNV-1a over choices and exact rational option vectors.
    // This intentionally omits elapsed time, cache counters, and work counters.
    for &byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

#[cfg(feature = "compact-dice")]
fn compact_diagnostics(shared: Option<&Shared>) -> Value {
    shared.map_or(Value::Null, |sh| {
        json!({
            "calls": sh.compact_dice_calls.load(Ordering::Relaxed),
            "nodes": sh.compact_dice_nodes.load(Ordering::Relaxed),
            "max_tricks": sh.compact_dice_max_tricks.load(Ordering::Relaxed),
        })
    })
}

#[cfg(not(feature = "compact-dice"))]
fn compact_diagnostics(_shared: Option<&Shared>) -> Value {
    Value::Null
}

fn play(cfg: &Config, repetition: usize) -> Value {
    let full_game_start = Instant::now();
    let mut execution = json!({"mac_qos":cfg.mac_qos});
    #[cfg(feature = "mac-qos")]
    if cfg.mac_qos == "interactive" && repetition == 0 {
        match mac_qos::initialize() {
            Ok(qos) => {
                execution["initialization"] = json!({"qos_class":qos.qos_class,
                "relative_priority":qos.relative_priority,"main_thread_ok":qos.main_thread_ok,
                "rayon_workers":qos.rayon_workers,"workers_configured":qos.workers_configured})
            }
            Err(error) => {
                return json!({"status":"qos_error","error":error,
                "repetition":repetition,"full_game_elapsed_us":full_game_start.elapsed().as_micros()})
            }
        }
    }
    #[cfg(not(feature = "mac-qos"))]
    let _ = &mut execution;
    let hands = deal(cfg, repetition);
    let fixture_preparation_us = full_game_start.elapsed().as_micros();
    let mut remain = hands.map(|h| h.to_vec());
    let mut plays: Vec<u8> = Vec::with_capacity(56);
    let mut decisions: Vec<Value> = Vec::with_capacity(28);
    let mut scores = [0u8; 2];
    let mut leader = cfg.bidder;
    let mut previous: Option<Shared> = None;
    let game_start = Instant::now();
    let mut value_digest = 0xcbf29ce484222325u64;
    let mut status = "complete";
    let mut error: Option<String> = None;
    let mut failed_call: Option<Value> = None;
    'game: for trick_index in 0..7 {
        let mut trick: Vec<(usize, u8)> = Vec::with_capacity(4);
        for offset in 0..4 {
            let seat = (leader + offset) % 4;
            let legal = legal(&remain[seat], &trick, cfg.decl);
            let input = request(cfg, seat, &hands[seat], &plays);
            let settings = solver_settings(cfg);
            let before = Instant::now();
            let mut isolated: Option<Shared> = None;
            let result = if cfg.cache == "carry" {
                partnership_wire::run_with_cache(&input, &mut previous)
            } else {
                // run() delegates to this same entry point with None. Keep
                // the completed Shared only long enough to read diagnostics.
                partnership_wire::run_with_cache(&input, &mut isolated)
            };
            let elapsed_us = before.elapsed().as_micros();
            let shared = if cfg.cache == "carry" {
                previous.as_ref()
            } else {
                isolated.as_ref()
            };
            let compact = compact_diagnostics(shared);
            let policy_cache_entries = shared.map(Shared::pi_cache_len);

            let response: Result<Value, String> =
                result.and_then(|s| serde_json::from_str(&s).map_err(|e| e.to_string()));
            let value = match response {
                Ok(v) => v,
                Err(e) => {
                    status = "refused";
                    error = Some(e);
                    failed_call = Some(json!({"play":decisions.len()+1,"trick":trick_index+1,
                        "seat":seat,"request":{"decl":cfg.decl,"bid":cfg.bid,"bidder":cfg.bidder,
                        "seat":seat,"hand":hands[seat],"plays":plays,"seed":cfg.public_seed},
                        "legal":legal,"elapsed_us":elapsed_us,"compact_dice":compact,
                        "wire_request":input,"solver_settings":settings,
                        "policy_cache_entries":policy_cache_entries}));
                    break 'game;
                }
            };
            let reported_legal: Option<Vec<u8>> = value
                .get("legal")
                .and_then(|v| serde_json::from_value(v.clone()).ok());
            let choice = value
                .get("choice")
                .and_then(Value::as_u64)
                .and_then(|n| u8::try_from(n).ok());
            if reported_legal.as_ref() != Some(&legal)
                || value.get("leader").and_then(Value::as_u64) != Some(leader as u64)
                || value
                    .get("points")
                    .and_then(|v| serde_json::from_value::<[u8; 2]>(v.clone()).ok())
                    != Some(scores)
                || !choice.is_some_and(|c| legal.contains(&c))
            {
                status = "invalid_response";
                error = Some(format!(
                    "independent legality/state mismatch at play {}",
                    decisions.len() + 1
                ));
                failed_call = Some(json!({"play":decisions.len()+1,"trick":trick_index+1,
                    "seat":seat,"request":{"decl":cfg.decl,"bid":cfg.bid,"bidder":cfg.bidder,
                    "seat":seat,"hand":hands[seat],"plays":plays,"seed":cfg.public_seed},
                    "expected_legal":legal,"response":value,"elapsed_us":elapsed_us,
                    "compact_dice":compact,"policy_cache_entries":policy_cache_entries,
                    "wire_request":input,"solver_settings":settings}));
                break 'game;
            }
            let tile = choice.unwrap();
            let semantic = json!({"play":decisions.len()+1,"seat":seat,"choice":tile,
                "options":value["options"]});
            value_digest = digest_add(value_digest, &serde_json::to_vec(&semantic).unwrap());
            let receipt = json!({"play": decisions.len() + 1, "trick": trick_index + 1,
                "seat": seat, "request": {"decl": cfg.decl, "bid": cfg.bid, "bidder": cfg.bidder,
                "seat": seat, "hand": hands[seat], "plays": plays, "seed": cfg.public_seed},
                "choice": tile, "legal": legal, "elapsed_us": elapsed_us,
                "forced": legal.len() == 1, "compact_dice": compact,
                "policy_cache_entries":policy_cache_entries,
                "wire_request":input,"solver_settings":settings,"response": value});

            eprintln!(
                "{}",
                json!({"event":"decision","repetition":repetition,"play":decisions.len()+1,
                "trick":trick_index+1,"seat":seat,"choice":tile,"elapsed_us":elapsed_us,
                "game_elapsed_us":game_start.elapsed().as_micros()})
            );
            let _ = io::stderr().flush();
            decisions.push(receipt);
            remain[seat].retain(|&t| t != tile);
            plays.extend([seat as u8, tile]);
            trick.push((seat, tile));
        }
        leader = winner(&trick, cfg.decl);
        scores[leader % 2] += trick_points(&trick);
    }
    let play_elapsed_us = game_start.elapsed().as_micros();
    if status == "complete"
        && (decisions.len() != 28
            || scores.iter().sum::<u8>() != 42
            || remain.iter().any(|h| !h.is_empty()))
    {
        status = "invalid_game";
        error = Some("28-play or 42-point invariant failed".into());
    }
    let mut receipt = json!({"repetition": repetition, "status": status, "error": error,"failed_call":failed_call,
        "execution":execution,
        "fixture":cfg.fixture, "deal_seed":if cfg.fixture == "shuffle" { Some(cfg.deal_seed.wrapping_add(repetition as u64)) } else { None },
        "hands_referee_only": hands, "profile":cfg.profile, "cache":cfg.cache,
        "decl":cfg.decl,"bid":cfg.bid,"bidder":cfg.bidder,"public_seed":cfg.public_seed,
        "samples":{"root":cfg.n,"l0":cfg.n0,"l1":cfg.n1},
        "selection":"fixed","modeled_selection":"fixed","inner_belief":"voidless",
        "budget_ms_per_call":cfg.budget_ms,"decisions":decisions,"plays":plays,
        "points":scores,"made":scores[cfg.bidder % 2] >= cfg.bid,
        "fixture_preparation_us":fixture_preparation_us,
        "play_elapsed_us":play_elapsed_us,
        "choice_value_digest_fnv1a64":format!("{value_digest:016x}")});
    // Include initialization, the referee checks and final receipt allocation;
    // textual serialization and filesystem/console output remain outside.
    receipt["full_game_elapsed_us"] = json!(full_game_start.elapsed().as_micros());
    receipt
}

fn main() {
    let cfg = match parse() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    };
    let batch_start = Instant::now();
    let mut games = Vec::with_capacity(cfg.repeats);
    for r in 0..cfg.repeats {
        let game = play(&cfg, r);
        let failed = game["status"] != "complete";
        if failed {
            eprintln!(
                "{}",
                json!({"event":"failure","repetition":r,"status":game["status"],"error":game["error"]})
            );
        }
        games.push(game);
        if failed {
            break;
        }
    }
    let mut receipt = json!({"schema":"walt-full-game-speed-v1","batch_elapsed_us":batch_start.elapsed().as_micros(),
        "complete":games.len()==cfg.repeats && games.iter().all(|g| g["status"]=="complete"),
        "games":games});
    let _ = &mut receipt;
    let encoded = serde_json::to_string_pretty(&receipt).expect("serialize receipt");
    if let Some(path) = cfg.output {
        if let Err(e) = fs::write(&path, &encoded) {
            eprintln!("cannot write {}: {e}", path.display());
            std::process::exit(2);
        }
    }
    println!("{encoded}");
    if receipt["complete"] != true {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use walt::rules::{Domino, DominoSet, Seat, Trick};

    #[test]
    fn referee_rules_match_walt_for_every_declaration() {
        for id in [0, 1, 2, 3, 4, 5, 6, 7, 9] {
            let decl = walt::solver::decl_of(id);
            for lead in 0u8..28 {
                let led = decl.led_context(Domino::from_index(lead as usize).unwrap());
                for start in 0u8..28 {
                    let mut hand: Vec<u8> = (0..7).map(|k| (start + 3 * k) % 28).collect();
                    hand.sort();
                    let ours = legal(&hand, &[(0, lead)], id as u8);
                    let set: DominoSet = hand
                        .iter()
                        .map(|&t| Domino::from_index(t as usize).unwrap())
                        .collect();
                    let theirs: Vec<u8> = walt::rules::legal_plays(decl, set, Some(led))
                        .iter()
                        .map(|t| t.index() as u8)
                        .collect();
                    assert_eq!(ours, theirs, "decl={id} lead={lead} start={start}");
                }
                for step in 1u8..=6 {
                    let dominoes = [
                        lead,
                        (lead + step) % 28,
                        (lead + step + 7) % 28,
                        (lead + step + 14) % 28,
                    ];
                    if dominoes
                        .iter()
                        .collect::<std::collections::HashSet<_>>()
                        .len()
                        != 4
                    {
                        continue;
                    }
                    let official = Trick::new(
                        Seat::S0,
                        dominoes.map(|t| Domino::from_index(t as usize).unwrap()),
                    )
                    .unwrap();
                    let local = [
                        (0usize, dominoes[0]),
                        (1, dominoes[1]),
                        (2, dominoes[2]),
                        (3, dominoes[3]),
                    ];
                    assert_eq!(
                        winner(&local, id as u8),
                        official.winner(decl).index(),
                        "decl={id} tiles={dominoes:?}"
                    );
                    assert_eq!(
                        u32::from(trick_points(&local)),
                        official.points(),
                        "decl={id} tiles={dominoes:?}"
                    );
                }
            }
        }
    }
}
