//! Regenerate the completed-trick lookup from the canonical rule algebra.
//! `cargo run -p walt --example generate_trick_table -- --check` verifies the
//! checked-in artifact without changing it.

use std::path::PathBuf;
use walt::rules::{Decl, Domino};

const SIDE: usize = 28;
const PER_DECL: usize = SIDE * SIDE * SIDE * SIDE;
const TOTAL: usize = 9 * PER_DECL;

fn table() -> Vec<u8> {
    let mut bytes = vec![0u8; TOTAL];
    for (di, decl) in Decl::ALL.into_iter().enumerate() {
        for a in 0..SIDE {
            let led = decl.led_context(Domino::ALL[a]);
            for b in 0..SIDE {
                for c in 0..SIDE {
                    for d in 0..SIDE {
                        let tiles = [a, b, c, d].map(|i| Domino::ALL[i]);
                        let mut best = decl.trick_key(tiles[0], led);
                        let mut winner = 0u8;
                        for (i, tile) in tiles.iter().enumerate().skip(1) {
                            let key = decl.trick_key(*tile, led);
                            if key > best {
                                best = key;
                                winner = i as u8;
                            }
                        }
                        let points = 1 + tiles.iter().map(|tile| tile.count()).sum::<u32>();
                        assert!(points < 64);
                        let index = di * PER_DECL + (((a * SIDE + b) * SIDE + c) * SIDE + d);
                        bytes[index] = ((points as u8) << 2) | winner;
                    }
                }
            }
        }
    }
    bytes
}

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    assert!(
        args.iter().all(|arg| arg == "--check"),
        "usage: generate_trick_table [--check]"
    );
    let output =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/solver/compact_dice/trick_table.bin");
    let generated = table();
    if args.iter().any(|arg| arg == "--check") {
        let existing = std::fs::read(&output).expect("read checked-in trick table");
        assert_eq!(
            existing, generated,
            "trick table differs from canonical rules"
        );
        println!("verified {} bytes at {}", generated.len(), output.display());
    } else {
        std::fs::write(&output, &generated).expect("write trick table");
        println!("wrote {} bytes to {}", generated.len(), output.display());
    }
}
