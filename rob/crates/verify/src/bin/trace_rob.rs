//! rob inspector trace generator (P5): four deterministic deals, rob
//! playing team 0 by rolling re-solve, the baseline playing team 1; rob
//! decisions carry capped contingency-book projections.
//!
//! Output (non-normative, regenerable display data):
//! - `rob/inspector/trace.js`   (`window.ROB_TRACE = {...};`)
//! - `rob/inspector/trace.json`
//!
//! Usage: `cargo run --release --bin trace_rob`, then open
//! `rob/inspector/index.html` in a browser. (`trace_player` regenerates
//! the baseline-only view of the same file when wanted.)

use std::path::Path;

fn main() {
    let (document, plans) = rob_verify::p5::rob_trace_document();
    let json = document.to_json();

    let inspector = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../inspector");
    std::fs::create_dir_all(&inspector).expect("create rob/inspector");
    let js_path = inspector.join("trace.js");
    let json_path = inspector.join("trace.json");
    std::fs::write(&js_path, format!("window.ROB_TRACE = {json};\n")).expect("write trace.js");
    std::fs::write(&json_path, &json).expect("write trace.json");

    println!("rob trace written (non-normative, deterministic):");
    println!("  {}", js_path.canonicalize().expect("path").display());
    println!("  {}", json_path.canonicalize().expect("path").display());
    println!(
        "hands: {}; rob plans embedded: {plans}",
        document.hands.len()
    );
    println!(
        "open rob/inspector/index.html in a browser — rob decisions show the contingency book"
    );
}
