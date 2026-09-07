//! Strict, inspectable interchange for cost-sensitive learner examples.
//! Teacher costs are data here, never registered as actor predicates.
use std::collections::BTreeMap;

use num_bigint::BigInt;
use num_rational::BigRational;

use crate::rules::{Domino, Seat};

use super::{relational::DecisionExample, Fixture};

pub fn rational(text: &str) -> Result<BigRational, String> {
    let pieces: Vec<_> = text.split('/').collect();
    let numerator = pieces[0]
        .parse::<BigInt>()
        .map_err(|_| "invalid rational")?;
    let denominator = match pieces.as_slice() {
        [_] => BigInt::from(1),
        [_, d] => d.parse::<BigInt>().map_err(|_| "invalid denominator")?,
        _ => return Err("rational has extra separators".into()),
    };
    if denominator <= BigInt::from(0) {
        return Err("rational denominator must be positive".into());
    }
    Ok(BigRational::new(numerator, denominator))
}

pub fn request_text(fixture: &Fixture, history: &[(Seat, Domino)], seed: u64) -> String {
    format!(
        "decl {}\nbid 30\nbidder {}\nseat {}\nhand {}\nplays {}\nseed {}\n",
        crate::solver::arena_decl_id(fixture.exercise.position.decl),
        fixture
            .history
            .first()
            .map_or(fixture.exercise.position.leader, |p| p.0)
            .index(),
        fixture.exercise.root.kernel().viewer().index(),
        fixture
            .original
            .iter()
            .map(|d| d.index().to_string())
            .collect::<Vec<_>>()
            .join(" "),
        history
            .iter()
            .map(|(s, d)| format!("{} {}", s.index(), d.index()))
            .collect::<Vec<_>>()
            .join(" "),
        seed,
    )
}

pub fn lesson_text(
    request: &str,
    weight: &BigRational,
    costs: &BTreeMap<Domino, BigRational>,
) -> String {
    format!(
        "{}weight {}\ncosts {}\n---\n",
        request,
        weight,
        costs
            .iter()
            .map(|(d, c)| format!("{} {}", d.index(), c))
            .collect::<Vec<_>>()
            .join(" ")
    )
}

pub fn read_lessons(text: &str) -> Result<Vec<DecisionExample>, String> {
    if text.len() > 64 * 1024 * 1024 {
        return Err("lesson file exceeds 64 MiB".into());
    }
    let mut result = Vec::new();
    for block in text.split("---") {
        if block.trim().is_empty() {
            continue;
        }
        let mut request = String::new();
        let mut weight = None;
        let mut costs = None;
        for line in block.lines().filter(|l| !l.trim().is_empty()) {
            let words: Vec<_> = line.split_whitespace().collect();
            match words[0] {
                "weight" => {
                    if words.len() != 2 || weight.is_some() {
                        return Err("invalid repeated weight".into());
                    }
                    weight = Some(rational(words[1])?);
                }
                "costs" => {
                    if words.len() % 2 != 1 || costs.is_some() {
                        return Err("invalid repeated action costs".into());
                    }
                    let mut map = BTreeMap::new();
                    for pair in words[1..].chunks(2) {
                        let tile =
                            Domino::from_index(pair[0].parse().map_err(|_| "invalid cost action")?)
                                .ok_or("invalid cost tile")?;
                        if map.insert(tile, rational(pair[1])?).is_some() {
                            return Err("duplicate cost action".into());
                        }
                    }
                    costs = Some(map);
                }
                _ => {
                    request.push_str(line);
                    request.push('\n');
                }
            }
        }
        let f = super::request::from_text(&request)?;
        result.push(DecisionExample::new(
            f.exercise.frame,
            f.history,
            f.exercise.position.banked,
            30,
            f.exercise.position.declaring_team,
            costs.ok_or("missing costs")?,
            weight.ok_or("missing weight")?,
        )?);
    }
    if result.is_empty() {
        return Err("empty lesson set".into());
    }
    Ok(result)
}

/// JSON output only. Inputs use the strict public-request and lesson formats.
pub fn quote(text: &str) -> String {
    let mut out = String::from("\"");
    for c in text.chars() {
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
