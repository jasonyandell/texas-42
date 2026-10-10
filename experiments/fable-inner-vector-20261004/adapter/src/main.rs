use std::io::{self, BufRead, Write};
fn main() {
    let stdout = io::stdout();
    for line in io::stdin().lock().lines() {
        let result = match line {
            Ok(text) if text.trim().is_empty() => continue,
            Ok(text) => inner_vector_seam::handle(&text),
            Err(e) => serde_json::json!({"error": e.to_string()}),
        };
        let mut out = stdout.lock();
        writeln!(out, "{result}").expect("write");
        out.flush().expect("flush");
    }
}
