//! Strict public/own-hand request bridge for existing partnership-gym cases.
use std::collections::BTreeMap;

use crate::gym;
use crate::rules::{Domino, DominoSet, Seat};
use crate::solver;

use super::Fixture;

/// Parse the seven-line partnership-gym wire format.  No hidden hand is an
/// accepted field; the resulting root is reconstructed by `gym::from_request`.
pub fn from_text(text: &str) -> Result<Fixture, String> {
    if text.len() > 65_536 {
        return Err("request too large".into());
    }
    let allowed = ["decl", "bid", "bidder", "seat", "hand", "plays", "seed"];
    let mut fields = BTreeMap::new();
    for line in text.lines() {
        let mut words = line.split_whitespace();
        let Some(name) = words.next() else { continue };
        if !allowed.contains(&name) {
            return Err("unknown request field".into());
        }
        let values = words
            .map(str::parse::<u64>)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| format!("non-integer {name}"))?;
        if fields.insert(name, values).is_some() {
            return Err(format!("duplicate request field {name}"));
        }
    }
    if fields.len() != allowed.len() {
        return Err("request needs exactly decl, bid, bidder, seat, hand, plays, seed".into());
    }
    let scalar = |name| -> Result<u64, String> {
        match fields.get(name).map(Vec::as_slice) {
            Some([value]) => Ok(*value),
            _ => Err(format!("{name} needs one integer")),
        }
    };
    let decl_id = scalar("decl")?;
    if ![0, 1, 2, 3, 4, 5, 6, 7, 9].contains(&decl_id) || scalar("bid")? != 30 {
        return Err("gym request needs a straight declaration and bid 30".into());
    }
    // Required and range-checked even though synthesis uses its CLI seed for
    // sample streams.  This preserves the exact seven-field request identity.
    let _request_seed = scalar("seed")?;
    let seat = |value: u64| {
        Seat::from_index(usize::try_from(value).unwrap_or(usize::MAX))
            .ok_or_else(|| "seat outside 0..3".to_owned())
    };
    let tile = |value: u64| {
        Domino::from_index(usize::try_from(value).unwrap_or(usize::MAX))
            .ok_or_else(|| "tile outside 0..27".to_owned())
    };
    let bidder = seat(scalar("bidder")?)?;
    let viewer = seat(scalar("seat")?)?;
    let ids = fields.get("hand").expect("field completeness checked");
    if ids.len() != 7 {
        return Err("original hand needs seven tiles".into());
    }
    let mut hand = DominoSet::EMPTY;
    for &id in ids {
        if !hand.insert(tile(id)?) {
            return Err("duplicate original tile".into());
        }
    }
    let plays = fields.get("plays").expect("field completeness checked");
    if plays.len() % 2 != 0 {
        return Err("public history needs actor/tile pairs".into());
    }
    let history = plays
        .chunks(2)
        .map(|pair| Ok((seat(pair[0])?, tile(pair[1])?)))
        .collect::<Result<Vec<_>, String>>()?;
    let exercise = gym::from_request(
        solver::decl_of(decl_id as usize),
        bidder,
        viewer,
        hand,
        &history,
    )?;
    Ok(Fixture {
        exercise,
        original: hand,
        history,
    })
}

#[cfg(test)]
mod tests {
    use super::from_text;

    const VALID: &str = "decl 6\nbid 30\nbidder 0\nseat 0\nhand 0 1 2 3 4 5 6\nplays\nseed 42\n";

    #[test]
    fn public_request_constructs_the_same_root_fields() {
        let fixture = from_text(VALID).unwrap();
        assert_eq!(fixture.original.len(), 7);
        assert!(fixture.history.is_empty());
        assert_eq!(fixture.exercise.position.bid, 30);
    }

    #[test]
    fn malformed_or_private_fields_are_rejected() {
        assert!(from_text(&VALID.replace("seed 42\n", "")).is_err());
        assert!(from_text(&format!("{VALID}hands 7 8 9\n")).is_err());
        assert!(from_text(&VALID.replace("plays\n", "plays 0\n")).is_err());
        assert!(from_text(&VALID.replace("hand 0 1 2 3 4 5 6", "hand 0 0 2 3 4 5 6")).is_err());
    }
}
