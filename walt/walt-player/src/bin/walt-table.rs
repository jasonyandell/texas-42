//! Line-delimited native transport. All policy decisions live in the library.
use std::io::{self, BufRead, Write};
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.expect("read request");
        let result = walt_player::handle(&line, |value| {
            println!("{}", serde_json::json!({"checkpoint":value}));
            io::stdout().flush().expect("flush checkpoint");
        });
        println!("{}", serde_json::json!({"result":result}));
        io::stdout().flush().expect("flush result");
    }
}
