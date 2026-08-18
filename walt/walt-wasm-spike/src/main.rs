//! EXPLORATORY WASM FEASIBILITY SPIKE — does level-1 walt fit on a phone?
//!
//! Builds the serial (no-rayon) walt-m3-probe solver for wasm32-wasip1 and
//! times realistic level-1 decisions: the opening lead (worst case: full
//! 7-candidate evaluation over n sampled worlds) and a trick-3 boundary
//! decision. Run natively for the baseline, under a WASM runtime for the
//! answer; the ratio is the phone-feasibility number (a phone's wasm is
//! then a further constant off whatever host runs it).
//!
//! Nothing here is quotable above exploratory tier; timings are
//! machine-relative ESTIMATES; not a P-A21 statement.

use std::sync::Arc;
use std::time::Instant;

use walt_core::rules::legal_plays;
use walt_core::{Decl, Domino, Pip, Seat};
use walt_m3_probe::{
    bp, mask_bits, mix, sample_belief, set_of, Field, Key, Shared, Solver, SplitMix64, FULL_MASK,
};

/// Same stream as bidcurve.rs so the spike's hand 0 is bidcurve's hand 0.
const BID_SEED: u64 = 0x4528_21E6_38D0_1377;

fn tile_name(t: u8) -> String {
    let dm = Domino::from_index(usize::from(t)).expect("tile");
    format!("{}{}", dm.hi().value(), dm.lo().value())
}

fn decide(
    dcl: Decl,
    hand: u32,
    worlds: Vec<[u32; 4]>,
    boundary_played: u32,
    boundary_hand_size: usize,
    root: &Key,
    label: &str,
) {
    let t0 = Instant::now();
    let sh = Arc::new(Shared::new(
        dcl,
        30,
        vec![8],
        boundary_played,
        boundary_hand_size,
        Instant::now() + std::time::Duration::from_secs(600),
    ));
    let solver = Solver::new(
        Arc::clone(&sh),
        Seat::from_index(1).expect("seat 1"),
        hand,
        true,
        worlds,
        Vec::new(),
        Field::Level(0),
    );
    let legal = legal_plays(dcl, set_of(hand & !root.played), None);
    let mut lines: Vec<String> = Vec::new();
    for tile in legal.iter() {
        let child = solver.child_after_play(root, tile, 0);
        let v = solver.solve(&child).expect("budget is generous");
        lines.push(format!("{}:{}bp", tile_name(tile.index() as u8), bp(&v)));
    }
    solver.flush_nodes();
    let ms = t0.elapsed().as_millis();
    println!("{label}: {} ms  [{}]", ms, lines.join(" "));
}

fn main() {
    println!("walt wasm spike: serial level-1 decisions (n_outer=40, n0=8, bid 30)");
    let mut rng = SplitMix64(BID_SEED ^ mix(0));
    let mut tiles: Vec<u8> = (0..28).collect();
    for i in (1..tiles.len()).rev() {
        let j = rng.below((i + 1) as u64) as usize;
        tiles.swap(i, j);
    }
    let hand: u32 = tiles[0..7].iter().fold(0, |a, &t| a | (1u32 << t));
    let names: Vec<String> = mask_bits(hand).iter().map(|&t| tile_name(t)).collect();
    println!("hand: [{}]  trump: sixes", names.join(" "));
    let dcl = Decl::PipTrump(Pip::new(6).expect("pip 6"));

    // Opening decision: S1 leads the hand, nothing played.
    let worlds = sample_belief(1, hand, 0, [7; 4], [0; 4], 40, &mut rng);
    let root = Key {
        played: 0,
        leader: 1,
        plays: Vec::new(),
        banked_t1: 0,
        banked_t0: 0,
        alive: 0,
    };
    decide(
        dcl,
        hand,
        worlds,
        0,
        7,
        &root,
        "opening lead (7 candidates)",
    );

    // Mid-hand decision: pretend two tricks are done (8 arbitrary lawful
    // tiles played, S1 to lead trick 3 with 5 tiles, 12:8 banked). The
    // point is timing shape, not game meaning.
    let hand_bits = mask_bits(hand);
    let mut played: u32 = (1u32 << hand_bits[0]) | (1u32 << hand_bits[1]);
    let mut pool: Vec<u8> = mask_bits(FULL_MASK & !hand);
    // deterministic: take the first six pool tiles
    for &t in pool.iter().take(6) {
        played |= 1u32 << t;
    }
    pool.truncate(6);
    let hand3 = hand & !played;
    let mut sizes = [5usize; 4];
    sizes[1] = 5;
    let worlds3 = sample_belief(1, hand3, played, sizes, [0; 4], 40, &mut rng);
    let root3 = Key {
        played,
        leader: 1,
        plays: Vec::new(),
        banked_t1: 12,
        banked_t0: 8,
        alive: 0,
    };
    decide(
        dcl,
        hand,
        worlds3,
        played,
        5,
        &root3,
        "trick-3 lead (5 candidates)",
    );
    println!("exploratory tier; timings are machine-relative estimates; not a P-A21 statement");
}
