use std::io::{self, BufRead};
fn main() {
    for line in io::stdin().lock().lines() {
        let result = match line {
            Ok(text) => native_late_player::handle(&text, |_| {}),
            Err(e) => serde_json::json!({"error": e.to_string()}),
        };
        println!("{result}");
    }
}
