//! Independent prose-rule trick resolver (BRIEF §1 D4; R-ALG-04).
//!
//! Codes the winner directly from the Rules prose — R-SUIT-01..03,
//! R-PLAY-04/05, R-FOLLOW-01/02, R-WIN-01..03, R-RANK-01..03, R-SCORE-01..03
//! — with **no** use of the algebra layer. It shares nothing with the
//! algebra implementation beyond domino identity and the declaration enum:
//! it never calls `trick_key`, `tier`, `rank`, `beats`, or `resolve_trick`.
//!
//! Count values use the explicit five-tile prose list (R-SCORE-01), not the
//! core antidiagonal formula, keeping the receipt genuinely independent.

use rob_core::{domino_from_id, Declaration, Domino, DominoId, Seat};

/// Whether `d` is a trump domino under the prose suit rules
/// (R-SUIT-01/02/03).
fn is_trump(declaration: Declaration, d: Domino) -> bool {
    match declaration {
        Declaration::PipTrump(p) => d.contains(p),
        Declaration::DoublesTrump => d.is_double(),
        Declaration::NoTrump => false,
    }
}

/// Prose count value from the explicit five-tile list (R-SCORE-01).
fn prose_count(d: Domino) -> u8 {
    let ends = (d.high().value(), d.low().value());
    match ends {
        (5, 5) | (6, 4) => 10,
        (5, 0) | (4, 1) | (3, 2) => 5,
        _ => 0,
    }
}

/// What the first play led (R-PLAY-04/05): the called (trump) suit for a
/// trump lead, else the natural suit of the higher pip.
enum ProseLed {
    Trump,
    Natural(u8),
}

/// Prose strength of a trump domino, higher wins (R-RANK-02/03).
fn trump_strength(declaration: Declaration, d: Domino) -> u8 {
    match declaration {
        // Trump double highest, others by their other end (R-RANK-02).
        Declaration::PipTrump(p) => {
            if d.is_double() {
                7
            } else if d.high() == p {
                d.low().value()
            } else {
                d.high().value()
            }
        }
        // Doubles rank 6-6 high through 0-0 low (R-RANK-03).
        Declaration::DoublesTrump => d.high().value(),
        Declaration::NoTrump => unreachable!("no trump exists in no-trump (R-SUIT-03)"),
    }
}

/// Whether a non-trump domino follows the led natural suit `q`
/// (R-FOLLOW-01 with R-SUIT-01/02: a called domino cannot satisfy a natural
/// lead).
fn follows_natural(declaration: Declaration, d: Domino, q: u8) -> bool {
    !is_trump(declaration, d) && (d.high().value() == q || d.low().value() == q)
}

/// Prose strength of a follower in natural suit `q`: the double is highest,
/// mixed dominoes ordered by their other end (R-RANK-01).
fn follower_strength(d: Domino, q: u8) -> u8 {
    if d.is_double() {
        7
    } else if d.high().value() == q {
        d.low().value()
    } else {
        d.high().value()
    }
}

/// Resolve one completed trick by prose rules alone: returns the winning
/// seat and the trick award `1 +` count (R-WIN-01..03; R-SCORE-02/03).
///
/// Callers supply four distinct dominoes with their actors; the first play
/// is the lead.
pub fn prose_resolve(declaration: Declaration, plays: &[(Seat, DominoId); 4]) -> (Seat, u8) {
    let dominoes: [Domino; 4] = plays.map(|(_, id)| domino_from_id(id));
    let lead = dominoes[0];

    let led = if is_trump(declaration, lead) {
        ProseLed::Trump
    } else {
        ProseLed::Natural(lead.high().value())
    };

    // R-WIN-01: if one or more trump dominoes are played, the highest trump
    // wins.
    let mut best: Option<(usize, u8)> = None;
    for (i, &d) in dominoes.iter().enumerate() {
        if is_trump(declaration, d) {
            let s = trump_strength(declaration, d);
            if best.is_none_or(|(_, bs)| s > bs) {
                best = Some((i, s));
            }
        }
    }

    // R-WIN-02: otherwise the highest domino that follows the led effective
    // suit wins. R-WIN-03: an off-suit discard without trump power cannot
    // win.
    if best.is_none() {
        let q = match led {
            ProseLed::Natural(q) => q,
            ProseLed::Trump => unreachable!("a trump lead is itself a trump play"),
        };
        for (i, &d) in dominoes.iter().enumerate() {
            if follows_natural(declaration, d, q) {
                let s = follower_strength(d, q);
                if best.is_none_or(|(_, bs)| s > bs) {
                    best = Some((i, s));
                }
            }
        }
    }

    let (winner_index, _) = best.expect("the lead itself always follows or is trump");
    let points = 1 + dominoes.iter().map(|&d| prose_count(d)).sum::<u8>();
    (plays[winner_index].0, points)
}
