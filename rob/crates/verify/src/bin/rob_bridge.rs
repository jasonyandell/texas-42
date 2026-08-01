//! rob subprocess bridge — an exploratory, non-normative surface that lets
//! an external harness ask rob for one play decision per line of stdin.
//!
//! This is instrument tooling in the `analysis.md` sense: it exposes the
//! certified `rob_player::solve` (Points lens, normative window) over a
//! dependency-free line protocol so a foreign game harness can seat rob
//! without linking against these crates. It creates no receipts and no
//! claims; play inside rob's own harnesses never routes through it.
//!
//! Protocol (one decision per line, whitespace-separated integers):
//!
//! request:  `seat decl bidder h0 h1 h2 h3 h4 h5 h6 n (actor domino)*n`
//!   - `seat`   viewer seat 0..4 (the seat rob decides for)
//!   - `decl`   declaration id: 0..=6 pip trump, 7 doubles-trump, 9 no-trump
//!     (the foreign harness's canonical declaration ids)
//!   - `bidder` the auction winner's seat (leads trick one, R-LEAD-01)
//!   - `h0..h6` the viewer's seven ORIGINALLY DEALT domino ids in the
//!     canonical triangular order `(0,0)=0, (1,0)=1, …, (6,6)=27`
//!   - `n` then `n` chronological `(actor, domino)` pairs of every play so
//!     far, including the viewer's own
//!
//! reply: `domino leader points0 points1` on one line, flushed — the chosen
//! domino id, then rob's own derived current trick leader and team point
//! totals after the replayed history, so the foreign harness can assert
//! rules conformance (trick resolution and scoring) on every decision.
//!
//! The synthesized contract always carries a P(30) bid: both this protocol
//! and the Points lens are bid-value-blind (the lens is a signed point
//! differential; the contract threshold is read only by ContractSuccess),
//! so only the bidder seat and the declaration are load-bearing.

use std::io::{BufRead, Write};

use rob_core::{
    contract_from_auction, initial_contracted_mechanical, update_support, AuctionAction,
    AuctionResult, AuctionState, BidValue, Contract, Declaration, DominoId, DominoSet, Play,
    PointAmount, RulesConfig, Seat, PIPS,
};
use rob_player::player::UtilityLens;

/// Synthesize the certified contract for `bidder` declaring `declaration`:
/// the shaker is chosen so `bidder` speaks first, bids P(30), and everyone
/// else passes — always a legal completed auction (R-AUC-06/07).
fn synthesized_contract(bidder: Seat, declaration: Declaration, config: RulesConfig) -> Contract {
    let shaker = bidder.offset(3);
    let mut auction = AuctionState::new(shaker);
    debug_assert_eq!(auction.next_actor(), Some(bidder));
    let script = [
        AuctionAction::Bid(BidValue::Point(
            PointAmount::new(30).expect("30 is a point bid"),
        )),
        AuctionAction::Pass,
        AuctionAction::Pass,
        AuctionAction::Pass,
    ];
    for action in script {
        auction = auction
            .apply(action, config)
            .expect("scripted auction is legal");
    }
    let win = match auction.result().expect("complete auction has a result") {
        AuctionResult::Win(win) => win,
        AuctionResult::AllPass => unreachable!("the script always bids"),
    };
    contract_from_auction(&win, declaration, config).expect("synthesized contract certifies")
}

fn declaration_of(decl_id: usize) -> Declaration {
    match decl_id {
        p @ 0..=6 => Declaration::PipTrump(PIPS[p]),
        7 => Declaration::DoublesTrump,
        9 => Declaration::NoTrump,
        other => panic!("declaration id {other} is not a straight-42 game declaration"),
    }
}

fn seat_of(value: usize) -> Seat {
    Seat::new(u8::try_from(value).expect("seat fits u8")).expect("seat 0..4")
}

fn id_of(value: usize) -> DominoId {
    DominoId::from_index(value).expect("domino id 0..28")
}

fn decide(nums: &[usize]) -> (usize, usize, u32, u32) {
    assert!(nums.len() >= 11, "request needs seat decl bidder h0..h6 n");
    let viewer = seat_of(nums[0]);
    let declaration = declaration_of(nums[1]);
    let bidder = seat_of(nums[2]);
    let mut hand = DominoSet::empty();
    for &raw in &nums[3..10] {
        hand.insert(id_of(raw));
    }
    let n_history = nums[10];
    assert_eq!(
        nums.len(),
        11 + 2 * n_history,
        "history carries exactly n (actor, domino) pairs"
    );

    let config = RulesConfig::new(7, 7).expect("valid config");
    let contract = synthesized_contract(bidder, declaration, config);
    let mut state =
        initial_contracted_mechanical(viewer, hand, contract).expect("seven-tile viewer hand");
    for pair in nums[11..].chunks_exact(2) {
        let play = Play {
            actor: seat_of(pair[0]),
            domino: id_of(pair[1]),
        };
        state = update_support(&state, play).expect("foreign history replays legally");
    }
    let plan = rob_player::solve(&state, UtilityLens::Points)
        .expect("the Points lens solves at every window");
    let points = state.hand_points();
    (
        plan.root.action.index(),
        state.leader().index(),
        points[0],
        points[1],
    )
}

fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    for line in stdin.lock().lines() {
        let line = line.expect("stdin line");
        if line.trim().is_empty() {
            continue;
        }
        let nums: Vec<usize> = line
            .split_whitespace()
            .map(|token| token.parse().expect("integer token"))
            .collect();
        let (chosen, leader, points0, points1) = decide(&nums);
        writeln!(out, "{chosen} {leader} {points0} {points1}").expect("stdout write");
        out.flush().expect("stdout flush");
    }
}
