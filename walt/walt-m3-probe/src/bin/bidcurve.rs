//! EXPLORATORY BID-CURVE PROBE — sits below every evidentiary tier and is
//! cited by nothing above it (wiki/ideas tier; outputs are ESTIMATES).
//!
//! The substrate of baseline bidding (SCENARIO-PLAYER.md §6.2): the pmake
//! objective parameterizes by bid level b — make ⇔ banked(T1) ≥ b ⇔
//! banked(T0) ≤ 42−b — so the same level-1 solver prices every (declaration,
//! bid) cell at the auction point. This probe deals random 7-tile hands to
//! the internal bidder S1 and computes P(make b) for all nine declarations
//! and b in 30..=42, over COMMON RANDOM WORLDS per hand (one belief sample
//! shared by every cell, so curves are comparable within a hand).
//!
//! The seat PLAYS TO THE BID at every cell: viewer decisions, decided
//! cutoffs, and every modeled mind's objective all use b's thresholds.
//! Consequence (spec note): P(make b) is NOT guaranteed monotone in b —
//! raising b changes the modeled field's behavior too, so adjacent cells
//! are values in slightly different games. Monotonicity is checked softly
//! and violations are reported, not asserted.
//!
//! A baseline bidder is a rule over these curves (e.g. "bid the highest b
//! with P(make b) ≥ θ, else pass"); rule selection is analysis downstream
//! of this output, not baked in here.
//!
//! Solver: the walt-m3-probe library (banked-correct PiKey, rayon-parallel,
//! level-1 configuration: field = level-0 minds, n_inner = [8]).
//! Arithmetic: integer counts and BigRational values, no floats. Nothing
//! here is quotable above exploratory tier; not a P-A21 statement.

use std::sync::Arc;
use std::time::Instant;

use num_rational::BigRational;

use walt_core::{Decl, Domino, Pip, Seat};
use walt_m3_probe::{bp, mask_bits, mix, Field, Key, Shared, Solver, SplitMix64};

/// Frozen bid-curve stream seed (fresh constant).
const BID_SEED: u64 = 0x4528_21E6_38D0_1377;

fn sample_auction_belief(viewer_hand: u32, n: usize, rng: &mut SplitMix64) -> Vec<[u32; 4]> {
    // Auction point: nothing played, no voids — the belief is the full
    // sizes-fiber over the other 21 tiles.
    let unseen = walt_m3_probe::FULL_MASK & !viewer_hand;
    let mut tiles = mask_bits(unseen);
    let mask_slice = |sl: &[u8]| sl.iter().fold(0u32, |a, &x| a | (1u32 << x));
    let mut out: Vec<[u32; 4]> = Vec::with_capacity(n);
    for _ in 0..n {
        for i in (1..tiles.len()).rev() {
            let j = rng.below((i + 1) as u64) as usize;
            tiles.swap(i, j);
        }
        let mut w = [0u32; 4];
        w[1] = viewer_hand;
        w[2] = mask_slice(&tiles[0..7]);
        w[3] = mask_slice(&tiles[7..14]);
        w[0] = mask_slice(&tiles[14..21]);
        out.push(w);
    }
    out
}

fn all_decls() -> Vec<(String, Decl)> {
    let mut v: Vec<(String, Decl)> = (0u8..=6)
        .map(|p| (format!("P{p}"), Decl::PipTrump(Pip::new(p).expect("pip"))))
        .collect();
    v.push(("DT".to_string(), Decl::DoublesTrump));
    v.push(("NT".to_string(), Decl::NoTrump));
    v
}

fn tile_name(t: u8) -> String {
    let dm = Domino::from_index(usize::from(t)).expect("tile");
    format!("{}{}", dm.hi().value(), dm.lo().value())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n_hands: u64 = args
        .get(1)
        .map(|s| s.parse().expect("hand count"))
        .unwrap_or(10);
    let start: u64 = args
        .get(2)
        .map(|s| s.parse().expect("start hand"))
        .unwrap_or(0);
    let n_outer: usize = args
        .get(3)
        .map(|s| s.parse().expect("outer sample size"))
        .unwrap_or(50);
    let budget: u64 = args
        .get(4)
        .map(|s| s.parse().expect("per-cell budget secs"))
        .unwrap_or(120);
    println!("bid-curve probe: EXPLORATORY P(make b) per (declaration, bid) at the auction point");
    println!(
        "level-1 seat (field = level-0 minds, n0=8), n_outer={n_outer} CRN per hand, bids 30..=42, {} rayon threads",
        rayon::current_num_threads()
    );
    println!("SAMPLED — values are ESTIMATES; monotonicity in b is checked softly (see header)");
    println!();
    for h in start..start + n_hands {
        let mut rng = SplitMix64(BID_SEED ^ mix(h));
        let mut tiles: Vec<u8> = (0..28).collect();
        for i in (1..tiles.len()).rev() {
            let j = rng.below((i + 1) as u64) as usize;
            tiles.swap(i, j);
        }
        let hand: u32 = tiles[0..7].iter().fold(0, |a, &t| a | (1u32 << t));
        let names: Vec<String> = mask_bits(hand).iter().map(|&t| tile_name(t)).collect();
        println!("hand {h}: [{}]", names.join(" "));
        let worlds = sample_auction_belief(hand, n_outer, &mut rng);
        let t0 = Instant::now();
        for (dname, dcl) in all_decls() {
            let mut row: Vec<String> = Vec::new();
            let mut prev: Option<BigRational> = None;
            let mut mono_violations = 0usize;
            for b in 30u8..=42 {
                let sh = Arc::new(Shared::new(
                    dcl,
                    b,
                    vec![8],
                    0,
                    7,
                    Instant::now() + std::time::Duration::from_secs(budget),
                ));
                let solver = Solver::new(
                    Arc::clone(&sh),
                    Seat::from_index(1).expect("seat 1"),
                    hand,
                    true,
                    worlds.clone(),
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
                match v {
                    Some(v) => {
                        if let Some(p) = &prev {
                            if v > *p {
                                mono_violations += 1;
                            }
                        }
                        row.push(format!("{b}:{:>5}", bp(&v)));
                        prev = Some(v);
                    }
                    None => {
                        row.push(format!("{b}:  DIED"));
                        prev = None;
                    }
                }
            }
            println!(
                "  {dname:>2}  {}{}",
                row.join(" "),
                if mono_violations > 0 {
                    format!("   [mono-viol x{mono_violations}]")
                } else {
                    String::new()
                }
            );
        }
        let ms = t0.elapsed().as_millis();
        println!("  ({}.{:03}s)", ms / 1000, ms % 1000);
        println!();
    }
    println!(
        "nothing above exploratory tier; bid-curve values are never receipts; not a P-A21 statement"
    );
}
