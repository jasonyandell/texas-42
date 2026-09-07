//! Offline Scheme/Fix query runner. No player code or private-policy inputs.
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use walt::kernel::ReceiptDecision;
use walt::rules::{receipt, DominoSet, Seat};
use walt::scheme::{Belief, Budget, Comparison, Fix, Frame, Registry, Selection};

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let argv: Vec<_> = std::env::args().skip(1).collect();
    if argv.is_empty() || argv == ["--help"] {
        println!(
            "Scheme/Fix v1 — exact offline relational queries\n\
Usage: scheme --query FILE [--receipt FILE] [--hand N] [--trick 1..7] [--seat 0..3]\n\
              [--max-worlds N] [--work N] [--condition FILE]\n\
              [--compare FILE] [--comparison answers|existence]\n\
              [--selector first|uniform]\n\
       scheme --registry\n\
Defaults: hand 0, trick 6, seat 0, world cap 40000, work 10000000.\n\
Results are exact on uniform legal support, or the explicitly conditioned measure.\n\
No sampled fallback. Budget refusal prints no partial result."
        );
        return Ok(());
    }
    let registry = Registry::standard();
    if argv == ["--registry"] {
        for spec in registry.specs() {
            println!(
                "{} {:?} access={:?} horizon={} version={}",
                spec.name, spec.parameters, spec.access, spec.horizon_plies, spec.version
            );
        }
        return Ok(());
    }
    let allowed = [
        "--query",
        "--receipt",
        "--hand",
        "--trick",
        "--seat",
        "--max-worlds",
        "--work",
        "--condition",
        "--compare",
        "--comparison",
        "--selector",
    ];
    if argv.len() % 2 != 0 {
        return Err("options require values; see --help".into());
    }
    let mut args = BTreeMap::new();
    for pair in argv.chunks(2) {
        if !allowed.contains(&pair[0].as_str())
            || args.insert(pair[0].as_str(), pair[1].as_str()).is_some()
        {
            return Err(format!("unknown or repeated option {}", pair[0]).into());
        }
    }
    let arg = |name, default| args.get(name).copied().unwrap_or(default);
    let query_path = args.get("--query").ok_or("--query is required")?;
    let query_source = std::fs::read_to_string(query_path)?;
    let query = query_source.parse::<Fix>()?.compile(&registry)?;
    let load_query = |path: &str| -> Result<_, Box<dyn std::error::Error>> {
        Ok(std::fs::read_to_string(path)?
            .parse::<Fix>()?
            .compile(&registry)?)
    };
    let default_receipt =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../rob/receipts/verify_player.txt");
    let receipt_path = args
        .get("--receipt")
        .map_or(default_receipt.as_path(), |s| Path::new(s));
    let receipt_text = std::fs::read_to_string(receipt_path)?;
    let receipt = receipt::parse(&receipt_text)?;
    let hand_id = arg("--hand", "0").parse::<usize>()?;
    let hand = receipt
        .hands
        .iter()
        .find(|h| h.id == hand_id)
        .ok_or("hand not found")?;
    let trick = arg("--trick", "6").parse::<usize>()?;
    if !(1..=7).contains(&trick) {
        return Err("trick must be 1..7".into());
    }
    let seat = Seat::from_index(arg("--seat", "0").parse()?).ok_or("seat must be 0..3")?;
    let decision = ReceiptDecision::at(hand, trick, seat)?;
    let mut played = DominoSet::EMPTY;
    for t in hand.tricks.iter().take(trick - 1) {
        for (_, tile) in t.plays {
            played.insert(tile);
        }
    }
    for tile in &decision.prefix {
        played.insert(*tile);
    }
    let frame = Frame::new(decision.kernel, decision.leader, decision.prefix, played)?;
    let support_count = frame.kernel().count();
    let mut belief = Belief::uniform(frame, arg("--max-worlds", "40000").parse()?)?;
    let mut budget = Budget::new(arg("--work", "10000000").parse()?);
    let mut conditioning = None;
    if let Some(path) = args.get("--condition") {
        let condition = load_query(path)?;
        let (posterior, probability) = belief.condition(
            &condition,
            format!("uniform conditioned on {}", condition.identity()),
            &mut budget,
        )?;
        belief = posterior;
        conditioning = Some(probability);
    }
    let summary = query.summarize(&belief, &mut budget)?;
    let comparison = match arg("--comparison", "answers") {
        "answers" => Comparison::Answers,
        "existence" => Comparison::Existence,
        _ => return Err("comparison must be answers or existence".into()),
    };
    let difference = if let Some(path) = args.get("--compare") {
        Some(query.compare(&load_query(path)?, &belief, comparison, &mut budget)?)
    } else {
        None
    };
    let selection = match args.get("--selector").copied() {
        None => None,
        Some("first") => Some(Selection::LexicographicFirst),
        Some("uniform") => Some(Selection::UniformWithinWorld),
        _ => return Err("selector must be first or uniform".into()),
    };
    // Buffer the entire report: no completed-looking partial result on refusal.
    let mut out = String::new();
    use std::fmt::Write;
    writeln!(
        out,
        "scheme-report-v1; exploratory; exact on declared finite measure"
    )?;
    writeln!(out, "query: {}", query.identity())?;
    writeln!(out, "coordinate: decl={} viewer={} hand={:?} pool={:?} hidden={:?} leader={} prefix={:?} played={:?}",
        belief.frame().kernel().decl(), seat, belief.frame().kernel().viewer_hand(), belief.frame().kernel().pool(),
        belief.frame().kernel().hidden(), belief.frame().leader(), belief.frame().prefix(), belief.frame().played())?;
    writeln!(
        out,
        "provenance: receipt={} hand={} trick={}; make-bid={}",
        receipt_path.display(),
        hand_id,
        trick,
        hand.bid_points
    )?;
    writeln!(
        out,
        "support-worlds: {support_count}\nbelief: {}\npositive-worlds: {}",
        belief.id(),
        belief.len()
    )?;
    if let Some(probability) = conditioning {
        writeln!(out, "conditioning-event-probability: {probability}")?;
    }
    writeln!(
        out,
        "event-probability: {}\ncertainty: {:?}",
        summary.event_probability(),
        summary.certainty
    )?;
    for (answer, mass) in &summary.answer_presence_mass {
        let named: Vec<_> = query
            .outputs()
            .iter()
            .zip(&answer.0)
            .map(|(r, v)| format!("{}={v}", r.name))
            .collect();
        writeln!(
            out,
            "answer-presence: [{}] probability={}",
            named.join(", "),
            mass / &summary.total_mass
        )?;
    }
    if let Some(selection) = selection {
        writeln!(out, "explicit-selector: {:?}", summary.select(selection))?;
    }
    if let Some(difference) = difference {
        writeln!(
            out,
            "comparison: {comparison:?}; counterexample={difference:?}"
        )?;
    }
    writeln!(out, "work: {}", budget.spent())?;
    print!("{out}");
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        eprintln!("scheme: {error}");
        std::process::exit(1);
    }
}
