//! Long-lived native worker. Every line is independent own-hand input.
use std::io::{self, BufRead, Write};
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.expect("read kiln job");
        let value = if line.len() > 16_384 {
            serde_json::json!({"error":"request too large"})
        } else { walt_player::kiln_price(&line) };
        println!("{value}");
        io::stdout().flush().expect("flush durable-job result");
    }
}
