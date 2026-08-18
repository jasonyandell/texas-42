//! The request/response surface. Pure functions — the same code path is
//! unit-tested natively and shipped to wasm, so browser behavior equals
//! tested behavior bit for bit (exact rationals, frozen seeds, no clocks
//! in the value path).
//!
//! Request: UTF-8 lines, each `key value...`, all values unsigned
//! integers. First line is `walt1 <kind>` (version magic). Keys:
//!
//! ```text
//! walt1 play
//! decl 2            # 0..6 pip trump, 7 doubles, 9 no-trump (arena ids)
//! bid 34            # contract bid 30..=42 (the pmake thresholds)
//! seat 0            # viewer's arena seat (the seat to decide for)
//! bidder 1          # auction winner's arena seat (leads trick one)
//! hand 3 9 14 20 24 26 27    # viewer's 7 ORIGINALLY DEALT tile ids
//! plays 1 5 2 12   # chronological (actor, tile) pairs, may be empty
//! n 40              # outer belief sample size
//! n0 8              # modeled level-0 mind sample size
//! seed 4660         # decision-stream seed (u64)
//! budget_ms 120000  # native-only wall budget; inert on wasm
//! ```
//!
//! `bid` kind: `hand`, `need` (minimum viable bid), optional
//! `theta <num> <den>` (default 1/2), `n`, `n0`, `seed`, `budget_ms`.
//! `declare` kind: `hand`, `bid`, `n`, `n0`, `seed`, `budget_ms`.
//!
//! Tile ids are the canonical triangular order (0,0)=0, (1,0)=1, ...,
//! (6,6)=27 — walt-core's `Domino::index`. Responses are JSON with
//! integer basis points only.

use std::collections::HashMap;
use std::sync::Arc;

use num_rational::BigRational;
use num_traits::Zero;

use walt_core::rules::legal_plays;
use walt_core::{Context, Decl, Domino, Seat, Team};
use walt_m3_probe::{
    best_of, bit, bp, decl_of, level1_evaluate, mask_of, mix, record_hash, replay, sample_belief,
    set_of, Deadline, Field, Key, Shared, Solver, SplitMix64,
};

/// Default decision-stream seed (the bridge's frozen e-digits constant).
const DEFAULT_SEED: u64 = 0xB7E1_5162_8AED_2A6B;

/// The nine straight-42 declarations in arena ids, ascending.
const DECL_IDS: [usize; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 9];

/// Handle one request; errors come back as `{"v":1,"error":"..."}` rather
/// than trapping, so a malformed call is diagnosable from the browser.
pub fn handle(req: &str) -> String {
    match handle_inner(req) {
        Ok(s) => s,
        Err(e) => format!("{{\"v\":1,\"error\":\"{}\"}}", esc(&e)),
    }
}

fn esc(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

struct Req {
    kind: String,
    fields: HashMap<String, Vec<u64>>,
}

impl Req {
    fn scalar(&self, key: &str) -> Result<u64, String> {
        match self.fields.get(key).map(Vec::as_slice) {
            Some([v]) => Ok(*v),
            Some(_) => Err(format!("field '{key}' wants exactly one value")),
            None => Err(format!("missing field '{key}'")),
        }
    }

    fn scalar_or(&self, key: &str, default: u64) -> Result<u64, String> {
        match self.fields.get(key).map(Vec::as_slice) {
            Some([v]) => Ok(*v),
            Some(_) => Err(format!("field '{key}' wants exactly one value")),
            None => Ok(default),
        }
    }

    fn list(&self, key: &str) -> Result<&[u64], String> {
        self.fields
            .get(key)
            .map(Vec::as_slice)
            .ok_or_else(|| format!("missing field '{key}'"))
    }
}

fn parse(req: &str) -> Result<Req, String> {
    let mut lines = req.lines().filter(|l| !l.trim().is_empty());
    let head = lines.next().ok_or("empty request")?;
    let mut ht = head.split_whitespace();
    if ht.next() != Some("walt1") {
        return Err("first line must be 'walt1 <kind>'".to_string());
    }
    let kind = ht.next().ok_or("missing request kind")?.to_string();
    let mut fields: HashMap<String, Vec<u64>> = HashMap::new();
    for line in lines {
        let mut t = line.split_whitespace();
        let key = t.next().expect("non-empty line");
        let vals: Result<Vec<u64>, _> = t.map(str::parse::<u64>).collect();
        let vals = vals.map_err(|_| format!("field '{key}': values must be unsigned integers"))?;
        if fields.insert(key.to_string(), vals).is_some() {
            return Err(format!("duplicate field '{key}'"));
        }
    }
    Ok(Req { kind, fields })
}

fn hand_mask(ids: &[u64]) -> Result<u32, String> {
    if ids.len() != 7 {
        return Err("hand wants exactly 7 tile ids".to_string());
    }
    let mut m = 0u32;
    for &raw in ids {
        let dm = Domino::from_index(raw as usize).ok_or("tile id must be 0..28")?;
        m |= bit(dm);
    }
    if m.count_ones() != 7 {
        return Err("hand tiles must be distinct".to_string());
    }
    Ok(m)
}

fn opts_json(opts: &[(u8, BigRational)]) -> String {
    let items: Vec<String> = opts
        .iter()
        .map(|(t, v)| format!("[{t},{}]", bp(v)))
        .collect();
    format!("[{}]", items.join(","))
}

fn handle_inner(req: &str) -> Result<String, String> {
    let r = parse(req)?;
    match r.kind.as_str() {
        "play" => handle_play(&r),
        "bid" => handle_bid(&r),
        "declare" => handle_declare(&r),
        other => Err(format!("unknown request kind '{other}'")),
    }
}

// ---------------------------------------------------------------------------
// play
// ---------------------------------------------------------------------------

fn handle_play(r: &Req) -> Result<String, String> {
    let decl_id = r.scalar("decl")? as usize;
    if !DECL_IDS.contains(&decl_id) {
        return Err(format!("decl {decl_id} is not a straight-42 declaration"));
    }
    let dcl = decl_of(decl_id);
    let bid = r.scalar("bid")? as u8;
    if !(30..=42).contains(&bid) {
        return Err("bid must be 30..=42".to_string());
    }
    let seat_arena = r.scalar("seat")? as usize;
    let bidder_arena = r.scalar("bidder")? as usize;
    if seat_arena > 3 || bidder_arena > 3 {
        return Err("seat and bidder must be 0..4".to_string());
    }
    let hand0 = hand_mask(r.list("hand")?)?;
    let plays = r.fields.get("plays").map(Vec::as_slice).unwrap_or(&[]);
    if !plays.len().is_multiple_of(2) {
        return Err("plays wants (actor, tile) pairs".to_string());
    }
    let pairs: Vec<(usize, usize)> = plays
        .chunks_exact(2)
        .map(|c| (c[0] as usize, c[1] as usize))
        .collect();
    let n_outer = r.scalar_or("n", 40)? as usize;
    let n0 = r.scalar_or("n0", 8)? as usize;
    let seed = r.scalar_or("seed", DEFAULT_SEED)?;
    let budget_ms = r.scalar_or("budget_ms", 120_000)?;

    let st = replay(dcl, bidder_arena, &pairs);
    let viewer_i = (seat_arena + st.r) % 4;
    let seat = Seat::from_index(viewer_i).expect("seat 0..4");
    let expect = (usize::from(st.leader) + st.plays.len()) % 4;
    if viewer_i != expect {
        return Err("viewer is not the seat to act after the replayed record".to_string());
    }

    let hand = hand0 & !st.played;
    let led: Option<Context> = st
        .plays
        .first()
        .map(|&i| dcl.led_context(Domino::from_index(usize::from(i)).expect("led tile")));
    let legal = mask_of(legal_plays(dcl, set_of(hand), led));
    if legal == 0 {
        return Err("viewer has no legal play (empty hand?)".to_string());
    }

    let (chosen, forced, opts) = if legal.count_ones() == 1 {
        (legal.trailing_zeros() as u8, true, Vec::new())
    } else {
        let key = Key {
            played: st.played,
            leader: st.leader,
            plays: st.plays.clone(),
            banked_t1: st.banked_t1,
            banked_t0: st.banked_t0,
            alive: 0,
        };
        let mut sizes = [7 - st.completed; 4];
        for i in 0..st.plays.len() {
            sizes[(usize::from(st.leader) + i) % 4] -= 1;
        }
        let mut rng = SplitMix64(seed ^ mix(u64::from(hand0)) ^ record_hash(&key));
        let opts = level1_evaluate(
            dcl,
            bid,
            seat,
            hand,
            legal,
            &key,
            sizes,
            st.voids,
            st.trick_start_played,
            7 - st.completed,
            n_outer,
            n0,
            budget_ms.div_ceil(1000).max(1),
            &mut rng,
        )
        .ok_or("evaluation deadline hit")?;
        let choice = best_of(&opts, seat.team() == Team::T1);
        (choice, false, opts)
    };

    // Reply in arena labels (the bridge's conformance convention): arena
    // team0 = arena seats {0,2}, internal T1 exactly when r == 1.
    let leader_arena = (usize::from(st.leader) + 4 - st.r) % 4;
    let (points0, points1) = if st.r == 1 {
        (st.banked_t1, st.banked_t0)
    } else {
        (st.banked_t0, st.banked_t1)
    };
    Ok(format!(
        "{{\"v\":1,\"kind\":\"play\",\"choice\":{chosen},\"forced\":{forced},\"opts\":{},\"leader\":{leader_arena},\"points\":[{points0},{points1}]}}",
        opts_json(&opts)
    ))
}

// ---------------------------------------------------------------------------
// bid / declare
// ---------------------------------------------------------------------------

/// Price one declaration at bid level `b` for a prospective bidder: solve
/// in a per-evaluation internal frame with the bidder at S1 leading (sound
/// at the auction point — the record is empty, other hands anonymous).
fn eval_bid(
    dcl: Decl,
    b: u8,
    n0: usize,
    hand: u32,
    worlds: Vec<[u32; 4]>,
    budget_ms: u64,
) -> BigRational {
    let deadline = Deadline::after(std::time::Duration::from_millis(budget_ms));
    let sh = Arc::new(Shared::new(dcl, b, vec![n0], 0, 7, deadline));
    let solver = Solver::new(
        sh,
        Seat::from_index(1).expect("seat 1"),
        hand,
        true,
        worlds,
        Vec::new(),
        Field::Level(0),
    )
    .parallel();
    let root = Key {
        played: 0,
        leader: 1,
        plays: Vec::new(),
        banked_t1: 0,
        banked_t0: 0,
        alive: 0,
    };
    let v = solver.solve(&root);
    solver.flush_nodes();
    v.unwrap_or_else(BigRational::zero)
}

fn prices_json(prices: &[(usize, BigRational)]) -> String {
    let items: Vec<String> = prices
        .iter()
        .map(|(d, v)| format!("[{d},{}]", bp(v)))
        .collect();
    format!("[{}]", items.join(","))
}

fn handle_bid(r: &Req) -> Result<String, String> {
    let hand0 = hand_mask(r.list("hand")?)?;
    let need = r.scalar_or("need", 30)? as u8;
    if !(30..=42).contains(&need) {
        return Err("need must be 30..=42".to_string());
    }
    let (tn, td) = match r.fields.get("theta").map(Vec::as_slice) {
        Some([n, d]) if *d > 0 && *n <= *d => (*n, *d),
        Some(_) => return Err("theta wants 'num den' with num <= den, den > 0".to_string()),
        None => (1, 2),
    };
    let theta = BigRational::new(tn.into(), td.into());
    let n = r.scalar_or("n", 40)? as usize;
    let n0 = r.scalar_or("n0", 8)? as usize;
    let seed = r.scalar_or("seed", DEFAULT_SEED)?;
    let budget_ms = r.scalar_or("budget_ms", 120_000)?;

    let mut rng = SplitMix64(seed ^ mix(u64::from(hand0)) ^ mix(u64::from(need)));
    let worlds = sample_belief(1, hand0, 0, [7; 4], [0; 4], n, &mut rng);
    let prices: Vec<(usize, BigRational)> = DECL_IDS
        .iter()
        .map(|&d| {
            (
                d,
                eval_bid(decl_of(d), need, n0, hand0, worlds.clone(), budget_ms),
            )
        })
        .collect();
    let (d_best, p_best) = prices
        .iter()
        .cloned()
        .reduce(|best, cand| if cand.1 > best.1 { cand } else { best })
        .expect("nine prices");
    if p_best < theta {
        return Ok(format!(
            "{{\"v\":1,\"kind\":\"bid\",\"action\":\"pass\",\"prices\":{}}}",
            prices_json(&prices)
        ));
    }
    // Walk the best declaration up while P(make b) >= theta (the webtable
    // baseline rule over bidcurve.rs's curves).
    let dcl = decl_of(d_best);
    let (mut b, mut p) = (need, p_best);
    while b < 42 {
        let v = eval_bid(dcl, b + 1, n0, hand0, worlds.clone(), budget_ms);
        if v >= theta {
            b += 1;
            p = v;
        } else {
            break;
        }
    }
    Ok(format!(
        "{{\"v\":1,\"kind\":\"bid\",\"action\":\"bid\",\"bid\":{b},\"decl\":{d_best},\"bp\":{},\"prices\":{}}}",
        bp(&p),
        prices_json(&prices)
    ))
}

fn handle_declare(r: &Req) -> Result<String, String> {
    let hand0 = hand_mask(r.list("hand")?)?;
    let bid = r.scalar("bid")? as u8;
    if !(30..=42).contains(&bid) {
        return Err("bid must be 30..=42".to_string());
    }
    let n = r.scalar_or("n", 40)? as usize;
    let n0 = r.scalar_or("n0", 8)? as usize;
    let seed = r.scalar_or("seed", DEFAULT_SEED)?;
    let budget_ms = r.scalar_or("budget_ms", 120_000)?;

    let mut rng = SplitMix64(seed ^ mix(u64::from(hand0)) ^ mix(0xDEC1));
    let worlds = sample_belief(1, hand0, 0, [7; 4], [0; 4], n, &mut rng);
    let mut prices: Vec<(usize, BigRational)> = DECL_IDS
        .iter()
        .map(|&d| {
            (
                d,
                eval_bid(decl_of(d), bid, n0, hand0, worlds.clone(), budget_ms),
            )
        })
        .collect();
    // Saturation ties across declarations: look closer on fresh larger
    // samples (never index-broken).
    let mut n_cur = n;
    loop {
        let best = prices
            .iter()
            .map(|(_, v)| v.clone())
            .max()
            .expect("nine prices");
        let tied: Vec<usize> = prices
            .iter()
            .filter(|(_, v)| *v == best)
            .map(|(d, _)| *d)
            .collect();
        if tied.len() == 1 || n_cur >= n * 16 {
            break;
        }
        n_cur *= 4;
        let worlds = sample_belief(1, hand0, 0, [7; 4], [0; 4], n_cur, &mut rng);
        for d in tied {
            let v = eval_bid(decl_of(d), bid, n0, hand0, worlds.clone(), budget_ms);
            let slot = prices.iter_mut().find(|(x, _)| *x == d).expect("tied decl");
            slot.1 = v;
        }
    }
    let chosen = prices
        .iter()
        .cloned()
        .reduce(|best, cand| if cand.1 > best.1 { cand } else { best })
        .expect("nine prices")
        .0;
    Ok(format!(
        "{{\"v\":1,\"kind\":\"declare\",\"decl\":{chosen},\"prices\":{}}}",
        prices_json(&prices)
    ))
}
