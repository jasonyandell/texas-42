//! Native transport for the shared partnership evaluator.
use std::io::{self, BufRead, Read, Write};
use walt::solver::partnership_wire::run;

fn main() {
    if std::env::args().any(|a| a == "--stream") {
        let mut input = String::new();
        for line in io::stdin().lock().lines() {
            let line = line.expect("read request");
            if line.is_empty() {
                if input.is_empty() {
                    continue;
                }
                match run(&input) {
                    Ok(output) => println!("{output}"),
                    Err(e) => {
                        // Protocol errors have controlled, ASCII messages.
                        let escaped = e
                            .replace('\\', "\\\\")
                            .replace('"', "\\\"")
                            .replace('\n', " ");
                        println!("{{\"status\":\"error\",\"error\":\"{escaped}\"}}");
                    }
                }
                io::stdout().flush().expect("flush response");
                input.clear();
            } else {
                input.push_str(&line);
                input.push('\n');
                assert!(input.len() <= 16384, "request too large");
            }
        }
        return;
    }
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("read request");
    match run(&input) {
        Ok(output) => println!("{output}"),
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    }
}
