//! Local exploratory partnership oracle. The public JSON interface and hard
//! decision budget live in experiments/partnership/player.py. This worker
//! receives only one seat's original hand and the public record.
use std::collections::HashMap;
use std::io::{self, Read};
use std::time::Duration;

use walt::rules::rules::legal_plays;
use walt::rules::{Domino, Seat};
use walt::solver::{self, bit, mask_bits, mask_of, set_of, Key};

fn scalar(fields: &HashMap<String, Vec<u64>>, key: &str) -> Result<u64, String> {
    let values = fields.get(key).ok_or_else(|| format!("missing {key}"))?;
    if values.len() != 1 {
        return Err(format!("{key} needs one value"));
    }
    Ok(values[0])
}

fn run(input: &str) -> Result<String, String> {
    let mut lines = input.lines();
    let mode = lines.next().ok_or("missing mode")?;
    let mut f = HashMap::new();
    for line in lines {
        let mut words = line.split_whitespace();
        let Some(name) = words.next() else { continue };
        let values: Result<Vec<u64>, _> = words.map(str::parse).collect();
        if f.insert(name.to_owned(), values.map_err(|_| "invalid integer")?)
            .is_some()
        {
            return Err(format!("duplicate {name}"));
        }
    }
    let decl_id = scalar(&f, "decl")?;
    let bid_raw = scalar(&f, "bid")?;
    let actor_raw = scalar(&f, "seat")?;
    let bidder_raw = scalar(&f, "bidder")?;
    if ![0, 1, 2, 3, 4, 5, 6, 7, 9].contains(&decl_id)
        || !(30..=42).contains(&bid_raw)
        || actor_raw > 3
        || bidder_raw > 3
    {
        return Err("invalid declaration, bid, or seat".into());
    }
    let (dcl, bid, actor, bidder) = (
        solver::decl_of(decl_id as usize),
        bid_raw as u8,
        actor_raw as usize,
        bidder_raw as usize,
    );
    let ids = f.get("hand").ok_or("missing hand")?;
    if ids.len() != 7 || ids.iter().any(|&t| t >= 28) {
        return Err("hand needs seven tile ids 0..27".into());
    }
    let hand0 = ids.iter().fold(0u32, |a, &t| a | (1u32 << t));
    if hand0.count_ones() != 7 {
        return Err("duplicate hand tile".into());
    }
    let plays = f.get("plays").cloned().unwrap_or_default();
    if plays.len() % 2 != 0 || plays.len() > 56 {
        return Err("invalid record length".into());
    }
    let mut pairs = Vec::new();
    let mut current_hand = hand0;
    for p in plays.chunks_exact(2) {
        if p[0] > 3 || p[1] >= 28 {
            return Err("invalid record actor/tile".into());
        }
        let st = solver::replay(dcl, bidder, &pairs);
        let s = (p[0] as usize + st.r) % 4;
        let tile = Domino::from_index(p[1] as usize).unwrap();
        if s != (usize::from(st.leader) + st.plays.len()) % 4 || st.played & bit(tile) != 0 {
            return Err("record violates turn order or repeats tile".into());
        }
        if st.voids[s] & bit(tile) != 0 {
            return Err("record contradicts a revealed void".into());
        }
        if p[0] as usize == actor {
            let led = st
                .plays
                .first()
                .map(|&t| dcl.led_context(Domino::from_index(t as usize).unwrap()));
            if !legal_plays(dcl, set_of(current_hand), led).contains(tile) {
                return Err("own historical play is illegal".into());
            }
            current_hand &= !bit(tile);
        } else if hand0 & bit(tile) != 0 {
            return Err("another seat played own tile".into());
        }
        pairs.push((p[0] as usize, p[1] as usize));
    }
    let st = solver::replay(dcl, bidder, &pairs);
    let seat = Seat::from_index((actor + st.r) % 4).unwrap();
    if seat.index() != (usize::from(st.leader) + st.plays.len()) % 4 || st.completed == 7 {
        return Err("not this seat's turn, or hand is complete".into());
    }
    let key = Key {
        voids: None,
        played: st.played,
        leader: st.leader,
        plays: st.plays.clone(),
        banked_t1: st.banked_t1,
        banked_t0: st.banked_t0,
        alive: 0,
    };
    let mut sizes = [7 - st.completed; 4];
    for i in 0..st.plays.len() {
        sizes[(st.leader as usize + i) % 4] -= 1;
    }
    solver::belief_frame_feasibility(seat.index(), current_hand, st.played, sizes, st.voids)
        .map_err(|e| e.to_string())?;
    let led = st
        .plays
        .first()
        .map(|&t| dcl.led_context(Domino::from_index(t as usize).unwrap()));
    let legal = mask_of(legal_plays(dcl, set_of(current_hand), led));
    let legal_ids = mask_bits(legal);
    let points = if st.r == 0 {
        [st.banked_t0, st.banked_t1]
    } else {
        [st.banked_t1, st.banked_t0]
    };
    let prefix = format!(
        "\"legal\":{:?},\"leader\":{},\"points\":{:?},\"trick\":{}",
        legal_ids,
        (st.leader as usize + 4 - st.r) % 4,
        points,
        st.completed + 1
    );
    if mode == "status" {
        return Ok(format!("{{{prefix}}}"));
    }
    let n = scalar(&f, "n")? as usize;
    let n0 = scalar(&f, "n0")? as usize;
    let n1 = scalar(&f, "n1")? as usize;
    let ms = scalar(&f, "budget_ms")?;
    if n == 0 || n > 640 || n0 == 0 || n0 > 64 || n1 == 0 || n1 > 64 || ms > 14000 {
        return Err("invalid sampling or time budget".into());
    }
    let profile = match mode {
        "baseline" => solver::partnership::FieldProfile::Baseline,
        "partner" => solver::partnership::FieldProfile::PartnerOnly,
        "all-l1" => solver::partnership::FieldProfile::AllLevel1,
        _ => return Err("unknown mode".into()),
    };
    let inner_belief = match f.get("inner_belief").map(Vec::as_slice) {
        None | Some([0]) => solver::InnerBelief::Voidless,
        Some([1]) => solver::InnerBelief::VoidsCounted,
        _ => return Err("unknown inner belief strategy".into()),
    };
    let seed = scalar(&f, "seed")? ^ solver::mix(u64::from(hand0)) ^ solver::record_hash(&key);
    let cfg = solver::partnership::Config {
        inner_belief,
        profile,
        n_outer: n,
        n0,
        n1,
        seed,
        deadline: solver::Deadline::after(Duration::from_millis(ms)),
    };
    let report = solver::partnership::evaluate(
        dcl,
        bid,
        seat,
        current_hand,
        legal,
        &key,
        sizes,
        st.voids,
        st.trick_start_played,
        7 - st.completed,
        &cfg,
    )
    .map_err(|e| format!("{e:?}"))?;
    let options: Vec<String> = report
        .actions
        .iter()
        .map(|a| {
            format!(
                "[{},\"{}\",\"{}\"]",
                a.tile.index(),
                a.value.numer(),
                a.value.denom()
            )
        })
        .collect();
    Ok(format!("{{{prefix},\"choice\":{},\"options\":[{}],\"inner_belief\":\"{}\",\"outer_worlds\":{},\"outer_draw_attempts\":{},\"pi_calls_by_level\":{:?},\"inner_worlds_by_level\":{:?},\"nodes\":{},\"solver_us\":{}}}",report.best(), options.join(","), inner_belief.name(), report.stats.outer_worlds, report.stats.outer_draw_attempts, report.stats.pi_calls_by_level,report.stats.inner_worlds_by_level,report.stats.nodes,report.stats.elapsed.as_micros()))
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("read request");
    match run(&input) {
        Ok(output) => println!("{output}"),
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    }
}
