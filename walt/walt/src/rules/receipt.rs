//! Parser for rob's `verify_player.txt` self-play receipt.
//!
//! The receipt is ground truth for the rules layer; this module only decodes
//! it. Nothing here derives a rule consequence -- that is `replay`.

use core::fmt;
use std::path::{Path, PathBuf};

use crate::rules::decl::Decl;
use crate::rules::domino::Domino;
use crate::rules::seat::{Seat, Team};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseError {
    pub line_no: usize,
    pub message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "receipt line {}: {}", self.line_no, self.message)
    }
}

impl std::error::Error for ParseError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReceiptTrick {
    pub number: usize,
    pub plays: [(Seat, Domino); 4],
    pub winner: Seat,
    pub points: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReceiptHand {
    pub id: usize,
    pub shaker: Seat,
    pub bidder: Seat,
    /// The bid threshold in points; the declaring side is made at or above it.
    pub bid_points: u32,
    pub decl: Decl,
    pub tricks: Vec<ReceiptTrick>,
    pub declaring_team: Team,
    pub declaring_points: u32,
    pub made: bool,
    /// Marks awarded for this hand, and to whom.
    pub marks_awarded: (Team, u32),
    /// Cumulative match marks after this hand, indexed by team.
    pub marks_after: [u32; 2],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Receipt {
    pub hands: Vec<ReceiptHand>,
    /// Final match marks, indexed by team.
    pub final_marks: [u32; 2],
    pub hands_played: usize,
}

fn seat_of(tok: &str) -> Option<Seat> {
    let n = tok.strip_prefix('S')?;
    Seat::from_index(n.parse::<usize>().ok()?)
}

fn team_of(tok: &str) -> Option<Team> {
    match tok.strip_prefix('T')?.parse::<usize>().ok()? {
        0 => Some(Team::T0),
        1 => Some(Team::T1),
        _ => None,
    }
}

struct Cursor<'a> {
    line_no: usize,
    text: &'a str,
}

impl Cursor<'_> {
    fn err<T>(&self, message: impl Into<String>) -> Result<T, ParseError> {
        Err(ParseError {
            line_no: self.line_no,
            message: message.into(),
        })
    }

    fn need<T>(&self, what: &str, v: Option<T>) -> Result<T, ParseError> {
        match v {
            Some(v) => Ok(v),
            None => self.err(format!("expected {what} in {:?}", self.text)),
        }
    }
}

/// `hand 0: shaker S0, bidder S1, bid P(30), declaration P3`
fn parse_hand_header(c: &Cursor<'_>, rest: &str) -> Result<ReceiptHand, ParseError> {
    let (id, fields) = c.need("`N: ...`", rest.split_once(": "))?;
    let id: usize = c.need("hand number", id.trim().parse().ok())?;
    let mut shaker = None;
    let mut bidder = None;
    let mut bid_points = None;
    let mut decl = None;
    for field in fields.split(", ") {
        let (key, value) = c.need("`key value`", field.split_once(' '))?;
        match key {
            "shaker" => shaker = seat_of(value),
            "bidder" => bidder = seat_of(value),
            "bid" => {
                let inner = c.need(
                    "a points bid `P(n)`",
                    value.strip_prefix("P(").and_then(|v| v.strip_suffix(')')),
                )?;
                bid_points = inner.parse().ok();
            }
            "declaration" => decl = value.parse().ok(),
            _ => return c.err(format!("unknown hand field {key:?}")),
        }
    }
    Ok(ReceiptHand {
        id,
        shaker: c.need("shaker seat", shaker)?,
        bidder: c.need("bidder seat", bidder)?,
        bid_points: c.need("bid points", bid_points)?,
        decl: c.need("declaration", decl)?,
        tricks: Vec::new(),
        declaring_team: Team::T0,
        declaring_points: 0,
        made: false,
        marks_awarded: (Team::T0, 0),
        marks_after: [0, 0],
    })
}

/// `trick 1: S1:6-0 S2:6-6 S3:6-4 S0:6-1 -> S2 +11`
fn parse_trick(c: &Cursor<'_>, rest: &str) -> Result<ReceiptTrick, ParseError> {
    let (number, rest) = c.need("`N: ...`", rest.split_once(": "))?;
    let number: usize = c.need("trick number", number.trim().parse().ok())?;
    let (plays_text, outcome) = c.need("` -> `", rest.split_once(" -> "))?;
    let tokens: Vec<&str> = plays_text.split_whitespace().collect();
    if tokens.len() != 4 {
        return c.err(format!("expected 4 plays, found {}", tokens.len()));
    }
    let mut plays = [(Seat::S0, Domino::ALL[0]); 4];
    for (slot, token) in plays.iter_mut().zip(tokens) {
        let (seat, domino) = c.need("`Sn:h-l`", token.split_once(':'))?;
        *slot = (
            c.need("seat", seat_of(seat))?,
            c.need("domino", domino.parse().ok())?,
        );
    }
    let (winner, points) = c.need("`Sn +p`", outcome.split_once(" +"))?;
    Ok(ReceiptTrick {
        number,
        plays,
        winner: c.need("winning seat", seat_of(winner))?,
        points: c.need("trick points", points.trim().parse().ok())?,
    })
}

/// `result: declaring T1 15 points -> set (1 mark to T0); marks T0 1 - 0 T1`
fn parse_result(c: &Cursor<'_>, hand: &mut ReceiptHand, rest: &str) -> Result<(), ParseError> {
    let rest = c.need("`declaring `", rest.strip_prefix("declaring "))?;
    let (declaring, rest) = c.need("` points -> `", rest.split_once(" points -> "))?;
    let (team, points) = c.need("`Tn p`", declaring.split_once(' '))?;
    hand.declaring_team = c.need("declaring team", team_of(team))?;
    hand.declaring_points = c.need("declaring points", points.parse().ok())?;

    let (verdict, marks) = c.need("`; marks `", rest.split_once("; marks "))?;
    let (verdict, award) = c.need("`verdict (award)`", verdict.split_once(" ("))?;
    hand.made = match verdict {
        "made" => true,
        "set" => false,
        other => return c.err(format!("unknown verdict {other:?}")),
    };
    let award = c.need("`(...)`", award.strip_suffix(')'))?;
    let award: Vec<&str> = award.split_whitespace().collect();
    match award.as_slice() {
        [n, "mark" | "marks", "to", team] => {
            hand.marks_awarded = (
                c.need("awarded team", team_of(team))?,
                c.need("awarded marks", n.parse().ok())?,
            );
        }
        _ => return c.err(format!("unknown mark award {award:?}")),
    }
    hand.marks_after = parse_marks(c, marks)?;
    Ok(())
}

/// `T0 1 - 0 T1`
fn parse_marks(c: &Cursor<'_>, text: &str) -> Result<[u32; 2], ParseError> {
    let f: Vec<&str> = text.split_whitespace().collect();
    match f.as_slice() {
        ["T0", a, "-", b, "T1"] => Ok([
            c.need("T0 marks", a.parse().ok())?,
            c.need("T1 marks", b.parse().ok())?,
        ]),
        _ => c.err(format!("unknown marks line {text:?}")),
    }
}

pub fn parse(text: &str) -> Result<Receipt, ParseError> {
    let mut hands: Vec<ReceiptHand> = Vec::new();
    let mut final_marks = None;
    let mut hands_played = 0;
    for (i, raw) in text.lines().enumerate() {
        let c = Cursor {
            line_no: i + 1,
            text: raw,
        };
        let line = raw.trim();
        if let Some(rest) = line.strip_prefix("hand ") {
            hands.push(parse_hand_header(&c, rest)?);
        } else if let Some(rest) = line.strip_prefix("trick ") {
            let hand = match hands.last_mut() {
                Some(h) => h,
                None => return c.err("trick before any hand header"),
            };
            hand.tricks.push(parse_trick(&c, rest)?);
        } else if let Some(rest) = line.strip_prefix("result: ") {
            let hand = match hands.last_mut() {
                Some(h) => h,
                None => return c.err("result before any hand header"),
            };
            parse_result(&c, hand, rest)?;
        } else if let Some(rest) = line.strip_prefix("match result: ") {
            let (marks, tail) = c.need("` after `", rest.split_once(" after "))?;
            final_marks = Some(parse_marks(&c, marks)?);
            let n = c.need("`n hands`", tail.strip_suffix(" hands"))?;
            hands_played = c.need("hand count", n.parse().ok())?;
        }
    }
    let c = Cursor {
        line_no: 0,
        text: "<receipt>",
    };
    Ok(Receipt {
        hands,
        final_marks: c.need("a `match result:` line", final_marks)?,
        hands_played,
    })
}

pub fn parse_file(path: &Path) -> Result<Receipt, Box<dyn std::error::Error>> {
    Ok(parse(&std::fs::read_to_string(path)?)?)
}

/// Walks up from `start` until a directory containing `rob/receipts` is found.
/// The workspace can sit at any depth inside the research repo.
pub fn locate_repo_root(start: &Path) -> Option<PathBuf> {
    let mut dir = Some(start);
    while let Some(d) = dir {
        if d.join("rob/receipts/verify_player.txt").is_file() {
            return Some(d.to_path_buf());
        }
        dir = d.parent();
    }
    None
}

/// The rob self-play receipt this crate is validated against (READ-ONLY).
pub fn locate_verify_player() -> Option<PathBuf> {
    let root = locate_repo_root(Path::new(env!("CARGO_MANIFEST_DIR")))?;
    Some(root.join("rob/receipts/verify_player.txt"))
}
