//! Player-track receipt binary (BRIEF_PLAYER_01 §8–§9).
//!
//! Prints the deterministic player-track receipt, one stage section per
//! green stage; CI diffs the output byte-for-byte against
//! `rob/receipts/verify_rob.txt` (INV-5). Lives in `rob-verify` beside the
//! other stage binaries so it can consume the frozen S3 corpus generator —
//! a recorded deviation from BRIEF_PLAYER_01 §13's module map (the player
//! modules themselves live in `rob-player` exactly as mapped).

fn main() {
    print!("{}", rob_verify::p1::receipt());
}
