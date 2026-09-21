//! Long-lived native worker. Every line is independent own-hand input; only
//! completed pure inner-policy answers may carry across matching contexts.
use std::io::{self, BufRead, Write};
fn main() {
    let mut pricer = walt_player::KilnPricer::default();
    for line in io::stdin().lock().lines() {
        let line = line.expect("read kiln job");
        let value = if line.len() > 16_384 {
            serde_json::json!({"error":"request too large"})
        } else { pricer.price(&line) };
        println!("{value}");
        io::stdout().flush().expect("flush durable-job result");
    }
}
