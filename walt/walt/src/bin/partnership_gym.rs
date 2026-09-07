//! One atomic exact gym assessment per process. The outer runner owns timeout,
//! retry, parallelism, and durable checkpoints. Input is own hand + public play.
use std::collections::BTreeMap;
use std::io::Read;
use walt::gym::{self, GymField};
use walt::rules::{legal_plays, Domino, DominoSet, Seat};
use walt::scheme::{Budget, Fix, Value};
use walt::solver::{self, adaptive::root_identity};

fn quote(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if c <= '\u{1f}' => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
fn run() -> Result<String, String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args == ["--help"] {
        return Ok("Usage: partnership_gym [--inspect] [--max-worlds N] [--partner-worlds N]\nInput lines: decl, bid (30), bidder, seat, hand (7 original ids), plays (actor/tile pairs), seed.\n".into());
    }
    let mut inspect = false;
    let mut cap = 400u128;
    let mut partner_worlds = 40u64;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--inspect" => {
                inspect = true;
                i += 1;
            }
            "--max-worlds" | "--partner-worlds" => {
                let n = args
                    .get(i + 1)
                    .ok_or("missing option value")?
                    .parse::<u64>()
                    .map_err(|_| "invalid option value")?;
                if args[i] == "--max-worlds" {
                    cap = u128::from(n);
                } else {
                    partner_worlds = n;
                }
                i += 2;
            }
            _ => return Err("unknown option".into()),
        }
    }
    if cap == 0 || !(1..=640).contains(&partner_worlds) {
        return Err("invalid cap or partner world count".into());
    }
    let mut input = String::new();
    std::io::stdin()
        .take(65537)
        .read_to_string(&mut input)
        .map_err(|e| e.to_string())?;
    if input.len() > 65536 {
        return Err("request too large".into());
    }
    let mut fields = BTreeMap::new();
    for line in input.lines() {
        let mut words = line.split_whitespace();
        let Some(name) = words.next() else { continue };
        if !["decl", "bid", "bidder", "seat", "hand", "plays", "seed"].contains(&name) {
            return Err("unknown request field".into());
        }
        let values = words
            .map(str::parse::<u64>)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;
        if fields.insert(name, values).is_some() {
            return Err("duplicate request field".into());
        }
    }
    let scalar = |name| -> Result<u64, String> {
        match fields.get(name).map(Vec::as_slice) {
            Some([n]) => Ok(*n),
            _ => Err(format!("{name} needs one integer")),
        }
    };
    let decl_id = scalar("decl")?;
    if ![0, 1, 2, 3, 4, 5, 6, 7, 9].contains(&decl_id) || scalar("bid")? != 30 {
        return Err("gym uses straight declarations and bid 30".into());
    }
    let seat = |n: u64| {
        Seat::from_index(usize::try_from(n).unwrap_or(usize::MAX))
            .ok_or_else(|| "seat outside 0..3".to_owned())
    };
    let tile = |n: u64| {
        Domino::from_index(usize::try_from(n).unwrap_or(usize::MAX))
            .ok_or_else(|| "tile outside 0..27".to_owned())
    };
    let viewer = seat(scalar("seat")?)?;
    let bidder = seat(scalar("bidder")?)?;
    let ids = fields.get("hand").ok_or("missing original hand")?;
    if ids.len() != 7 {
        return Err("original hand needs seven tiles".into());
    }
    let mut hand = DominoSet::EMPTY;
    for id in ids {
        if !hand.insert(tile(*id)?) {
            return Err("duplicate original tile".into());
        }
    }
    let plays = fields.get("plays").ok_or("missing public history")?;
    if plays.len() % 2 != 0 {
        return Err("public history needs actor/tile pairs".into());
    }
    let history = plays
        .chunks(2)
        .map(|p| Ok((seat(p[0])?, tile(p[1])?)))
        .collect::<Result<Vec<_>, String>>()?;
    let ex = gym::from_request(
        solver::decl_of(decl_id as usize),
        bidder,
        viewer,
        hand,
        &history,
    )?;
    let legal: Vec<_> = legal_plays(
        ex.position.decl,
        ex.root.kernel().viewer_hand(),
        ex.frame.led_context(),
    )
    .iter()
    .map(Domino::index)
    .collect();
    let query = gym::OFFER_QUERY
        .parse::<Fix>()
        .map_err(|e| e.to_string())?
        .compile(&gym::registry())
        .map_err(|e| e.to_string())?;
    let answers = query
        .evaluate(
            &ex.frame,
            &ex.root.worlds().next().expect("nonempty support"),
            &mut Budget::new(1_000_000),
        )
        .map_err(|e| e.to_string())?;
    let offers: Vec<_> = answers
        .iter()
        .map(|a| match a.0.as_slice() {
            [Value::Domino(d)] => d.index(),
            _ => unreachable!(),
        })
        .collect();
    let header = format!("\"schema\":\"partnership-gym-v1\",\"root_id\":\"{:016x}\",\"worlds\":{},\"legal\":{:?},\"offers\":{:?},\"leader\":{},\"prefix\":{:?},\"banked\":{:?},\"trick\":{},\"remaining\":{:?},\"scheme_identity\":{}",
        root_identity(&ex.root, &ex.position), ex.root.count(), legal, offers, ex.position.leader.index(), ex.position.trick_plays.iter().map(|d| d.index()).collect::<Vec<_>>(), ex.position.banked,
        ex.position.prior_played.len()/4 + 1, ex.root.kernel().viewer_hand().iter().map(Domino::index).collect::<Vec<_>>(), quote(&query.identity()));
    if inspect {
        return Ok(format!("{{{header}}}"));
    }
    let field = GymField::new(viewer, partner_worlds);
    let assessment = gym::assess(&ex, &field, cap)?;
    let mut actions = Vec::new();
    for action in assessment.actions {
        let traces: Vec<_> = action.traces.iter().map(|t| format!("{{\"hands\":{:?},\"plays\":{:?},\"banked\":{:?},\"partner_count\":{},\"success\":{}}}", t.hands, t.plays, t.banked, t.partner_count, t.success)).collect();
        actions.push(format!("{{\"tile\":{},\"success_mass\":{},\"score_bins\":{:?},\"policy_id\":{},\"policy_states\":{},\"traces\":[{}]}}",
            action.tile, action.success_mass, action.bins, quote(&action.policy_id), action.policy_states, traces.join(",")));
    }
    Ok(format!(
        "{{{header},\"field_id\":{},\"partner_worlds\":{},\"best\":{:?},\"actions\":[{}]}}",
        quote(&assessment.field_id),
        partner_worlds,
        assessment.best,
        actions.join(",")
    ))
}
fn main() {
    match run() {
        Ok(result) => println!("{result}"),
        Err(error) => {
            eprintln!("partnership_gym: {error}");
            std::process::exit(1);
        }
    }
}
