//! Persistent full-game transport; policy implementation remains walt_player::decide.
use std::io::{self, BufRead, Write};
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.expect("read game job");
        let result = if line.len() > 4096 {
            Err("game request too large".to_owned())
        } else {
            walt_player::played::run(&line)
        };
        let value = result.unwrap_or_else(|error| serde_json::json!({"error":error}));
        println!("{value}");
        io::stdout().flush().expect("flush completed game");
    }
}
