//! Post-hoc attribution instrument. Same finite prior/search, only the
//! partner policy changes between L0-8 and GymField's L1-40/8 partner.
//! This does not change the deployed candidate or select a new player.
use std::io::Read;
use std::time::Instant;
use walt::gym::GymField;
use walt::policy_search::{learning_io::quote, request, Search, Work};
use walt::rules::legal_plays;
use walt::solver::{adaptive::SlicePolicy, policy::Level0Field};

fn run() -> Result<String, String> {
    let mut input = String::new();
    std::io::stdin()
        .take(65_537)
        .read_to_string(&mut input)
        .map_err(|e| e.to_string())?;
    let f = request::from_text(&input)?;
    let ex = &f.exercise;
    if ex.root.count() > 400 || ex.root.kernel().viewer_hand().len() > 3 {
        return Err("attribution uses small endgames only".into());
    }
    let low = Level0Field::new(8);
    let partner = GymField::new(ex.root.kernel().viewer(), 40);
    let fields: [&dyn SlicePolicy; 2] = [&low, &partner];
    let legal = legal_plays(
        ex.position.decl,
        ex.root.kernel().viewer_hand(),
        ex.frame.led_context(),
    );
    let mut arms = Vec::new();
    for field in fields {
        let start = Instant::now();
        let mut search = Search::new(ex, field);
        for world in ex.root.worlds() {
            search.append(world)?;
        }
        let mut work = Work::new(1_000_000);
        let values = search.compare_root(legal, &mut work)?;
        let options = values
            .iter()
            .map(|(tile, mass)| format!("[{},{}]", tile.index(), mass))
            .collect::<Vec<_>>()
            .join(",");
        arms.push(format!(
            "{{\"field_id\":{},\"values\":[{}],\"nodes\":{},\"field_calls\":{},\"elapsed_us\":{}}}",
            quote(field.id()),
            options,
            work.nodes,
            work.field_calls,
            start.elapsed().as_micros()
        ));
    }
    Ok(format!(
        "{{\"schema\":\"partner-review-attribution-v1\",\"worlds\":{},\"arms\":[{}]}}",
        ex.root.count(),
        arms.join(",")
    ))
}
fn main() {
    match run() {
        Ok(value) => println!("{value}"),
        Err(error) => {
            eprintln!("partner_review_ablation: {error}");
            std::process::exit(1);
        }
    }
}
