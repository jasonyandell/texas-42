//! One bounded continuation check. Its caller retains the completed baseline
//! and enforces the outer wall deadline, including process startup and output.
use std::io::Read;
use std::time::Duration;
use walt::policy_search::{learning_io::quote, partner_rollout as r, request};
use walt::solver::Deadline;

fn run() -> Result<String, String> {
    let mut baseline = None;
    let mut milliseconds = 450;
    let mut samples = r::MAX_SAMPLES;
    let mut audit = false;
    let mut args = std::env::args().skip(1);
    while let Some(flag) = args.next() {
        if flag == "--audit" {
            audit = true;
            continue;
        }
        let value = args
            .next()
            .ok_or("missing option value")?
            .parse::<usize>()
            .map_err(|_| "invalid integer option")?;
        match flag.as_str() {
            "--baseline" => baseline = Some(value),
            "--milliseconds" => milliseconds = value,
            "--samples" => samples = value,
            _ => return Err("unknown option".into()),
        }
    }
    if !(1..=14_000).contains(&milliseconds) {
        return Err("milliseconds outside 1..14000".into());
    }
    let deadline = Deadline::after(Duration::from_millis(milliseconds as u64));
    let mut text = String::new();
    std::io::stdin()
        .take(65_537)
        .read_to_string(&mut text)
        .map_err(|e| e.to_string())?;
    let (fixture, seed) = request::from_text_with_seed(&text)?;
    let baseline = baseline.ok_or("baseline required")?;
    let result = r::review(&fixture, seed, baseline, deadline, samples, audit)?;
    let values = result
        .legal
        .iter()
        .zip(&result.values)
        .map(|(a, m)| format!("[{a},{m}]"))
        .collect::<Vec<_>>()
        .join(",");
    let decisions = if audit {
        result
            .decisions
            .iter()
            .map(|d| {
                let hand = walt::solver::mask_bits(d.original);
                let plays = d
                    .history
                    .iter()
                    .flat_map(|&(s, t)| [s, t])
                    .collect::<Vec<_>>();
                format!(
                    "{{\"seat\":{},\"hand\":{hand:?},\"plays\":{plays:?},\"choice\":{}}}",
                    d.seat, d.choice
                )
            })
            .collect::<Vec<_>>()
            .join(",")
    } else {
        String::new()
    };
    let traces = result.traces.iter().map(|t| {
        let hands = t.hands.iter().map(|h|h.iter().map(|d|d.index()).collect::<Vec<_>>()).collect::<Vec<_>>();
        let record = t.history.iter().flat_map(|&(s,t)|[s,t]).collect::<Vec<_>>();
        format!("{{\"hands\":{hands:?},\"action\":{},\"record\":{record:?},\"made\":{},\"banked\":{:?}}}",t.action,t.made,t.banked)
    }).collect::<Vec<_>>().join(",");
    Ok(format!("{{\"schema\":{},\"status\":{},\"stop\":{},\"choice\":{},\"baseline\":{baseline},\"offers\":{:?},\"legal\":{:?},\"values\":[{values}],\"paired\":{:?},\"support\":{},\"samples\":{},\"requested\":{},\"coverage\":{},\"field\":{},\"calls\":{},\"cache_hits\":{},\"elapsed_us\":{},\"decisions\":[{decisions}],\"traces\":[{traces}]}}",
        quote(r::ID),quote(result.status),quote(result.stop),result.choice,result.offers,result.legal,result.paired,
        result.support,result.samples,result.requested,quote(result.coverage),quote(r::FIELD),
        result.decisions.len(),result.cache_hits,result.elapsed_us))
}

fn main() {
    match run() {
        Ok(value) => println!("{value}"),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
