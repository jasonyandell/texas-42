//! Inspector trace generator: replays the frozen seed-42 self-play match
//! (identical driver and parameters as `verify_player`) and writes the
//! non-normative inspector trace next to the viewer.
//!
//! Output:
//! - `rob/inspector/trace.js`   (`window.ROB_TRACE = {...};` — loadable
//!   from `file://`)
//! - `rob/inspector/trace.json` (the same document, plain JSON)
//!
//! Usage: `cargo run --release --bin trace_player`, then open
//! `rob/inspector/index.html` in a browser.

use std::path::Path;

use rob_core::RulesConfig;
use rob_player::trace::trace_match;
use rob_player::{MonteCarloPlayer, UtilityLens};

fn main() {
    let config = RulesConfig::new(2, 7).expect("valid config");
    let player = MonteCarloPlayer {
        worlds_per_decision: 12,
        lens: UtilityLens::Points,
        seed: 7,
    };
    let document = trace_match(config, &player, 42);
    let json = document.to_json();

    let inspector = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../inspector");
    std::fs::create_dir_all(&inspector).expect("create rob/inspector");
    let js_path = inspector.join("trace.js");
    let json_path = inspector.join("trace.json");
    std::fs::write(&js_path, format!("window.ROB_TRACE = {json};\n")).expect("write trace.js");
    std::fs::write(&json_path, &json).expect("write trace.json");

    println!("rob inspector trace written (non-normative, deterministic):");
    println!("  {}", js_path.canonicalize().expect("path").display());
    println!("  {}", json_path.canonicalize().expect("path").display());
    println!(
        "hands: {}; decision states: {}; final marks T0 {} - {} T1",
        document.hands.len(),
        rob_player::trace::decision_count(&document),
        document.final_marks[0],
        document.final_marks[1]
    );
    println!("open rob/inspector/index.html in a browser to walk the match");
}
