//! One optional investigation per process. The player wrapper owns its hard
//! deadline and keeps the completed baseline when this process is interrupted.
use std::io::Read;
use std::time::Instant;
use walt::policy_search::{learning_io::quote, partner_review, request};
use walt::rules::Domino;

fn run() -> Result<String, String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 2 || args[0] != "--baseline" {
        return Err(
            "usage: partner_review --baseline TILE; seven-field public request on stdin".into(),
        );
    }
    let baseline = args[1]
        .parse::<usize>()
        .ok()
        .and_then(Domino::from_index)
        .ok_or("baseline must be a domino id")?;
    let mut text = String::new();
    std::io::stdin()
        .take(65_537)
        .read_to_string(&mut text)
        .map_err(|e| e.to_string())?;
    let start = Instant::now();
    let f = request::from_text(&text)?;
    let r = partner_review::review(&f.exercise, baseline)?;
    let options = r
        .values
        .iter()
        .map(|(tile, value)| format!("[{},{}]", tile.index(), value))
        .collect::<Vec<_>>()
        .join(",");
    Ok(format!("{{\"schema\":{},\"choice\":{},\"baseline\":{},\"status\":{},\"worlds\":{},\"offers\":{:?},\"values\":[{}],\"nodes\":{},\"field_calls\":{},\"field_id\":{},\"prior\":\"uniform-mechanical-root-fiber\",\"elapsed_us\":{}}}",
        quote(partner_review::ID),r.choice.index(),baseline.index(),quote(r.status),r.worlds,
        r.offers.iter().map(Domino::index).collect::<Vec<_>>(),options,r.work.nodes,r.work.field_calls,
        quote(&r.field_id),start.elapsed().as_micros()))
}
fn main() {
    match run() {
        Ok(result) => println!("{result}"),
        Err(error) => {
            eprintln!("partner_review: {error}");
            std::process::exit(1);
        }
    }
}
