//! Bounded native jobs for the shared-policy campaign. The external runner
//! owns process deadlines, immutable job identities and atomic publication.
use std::{collections::BTreeMap, path::PathBuf, time::Instant};
use walt::policy_search::{
    self,
    learning_io::{quote, read_lessons, request_text},
    relational::{RelationalLearner, RelationalLimits},
};
use walt::rules::legal_plays;

fn main() {
    match run() {
        Ok(s) => println!("{s}"),
        Err(e) => {
            eprintln!("relational_lab: {e}");
            std::process::exit(1);
        }
    }
}
fn run() -> Result<String, String> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args == ["--help"] {
        return Ok("relational_lab generate|fit|evaluate [--key value ...]; public request inputs, bid30, bounded finite-root policy learning".into());
    }
    let op = args.first().ok_or("missing operation")?;
    let mut opts = BTreeMap::new();
    for pair in args[1..].chunks(2) {
        if pair.len() != 2 || opts.insert(pair[0].as_str(), pair[1].as_str()).is_some() {
            return Err("invalid duplicate option".into());
        }
    }
    let allowed = match op.as_str() {
        "generate" => vec!["--seed", "--tiles", "--max-worlds", "--attempts", "--decl"],
        "fit" => vec![
            "--lessons",
            "--output",
            "--clauses",
            "--beam",
            "--ast",
            "--work",
        ],
        "evaluate" => vec![
            "--request",
            "--policies",
            "--output",
            "--field",
            "--seed",
            "--samples",
            "--work",
            "--prices",
        ],
        _ => return Err("unknown operation".into()),
    };
    if opts.keys().any(|k| !allowed.contains(k)) {
        return Err("unknown option".into());
    }
    let integer = |key: &str, default: u64| -> Result<u64, String> {
        opts.get(key).map_or(Ok(default), |s| {
            s.parse().map_err(|_| format!("invalid {key}"))
        })
    };
    let required = |key: &str| -> Result<&str, String> {
        opts.get(key)
            .copied()
            .ok_or_else(|| format!("missing {key}"))
    };
    match op.as_str() {
        "generate" => {
            let base = integer("--seed", 910000)?;
            let tiles = integer("--tiles", 3)? as usize;
            let max = integer("--max-worlds", 512)? as u128;
            let attempts = integer("--attempts", 1000)?;
            let decl_id = integer("--decl", 6)? as usize;
            if !(2..=4).contains(&tiles)
                || !(1..=10000).contains(&max)
                || !(1..=1000).contains(&attempts)
                || ![0, 1, 2, 3, 4, 5, 6, 7, 9].contains(&decl_id)
            {
                return Err("generation bounds".into());
            }
            for offset in 0..attempts {
                let seed = base.checked_add(offset).ok_or("seed overflow")?;
                let f = policy_search::fixture(seed, walt::solver::decl_of(decl_id), tiles, None)?;
                let root = &f.exercise;
                if root.root.count() > max
                    || policy_search::State::from_root(&root.position)
                        .success(&root.position)
                        .is_some()
                    || legal_plays(
                        root.position.decl,
                        root.root.kernel().viewer_hand(),
                        root.frame.led_context(),
                    )
                    .len()
                        < 2
                {
                    continue;
                }
                return Ok(format!("{{\"schema\":\"relational-root-v1\",\"seed\":{seed},\"attempts\":{},\"worlds\":{},\"hand\":{},\"request\":{}}}",offset+1,root.root.count(),quote(&format!("{:?}",f.original.bits())),quote(&request_text(&f,&f.history,seed))));
            }
            Err("no qualifying own/public root in allocated seed range".into())
        }
        "fit" => {
            let text =
                std::fs::read_to_string(required("--lessons")?).map_err(|e| e.to_string())?;
            let examples = read_lessons(&text)?;
            let limits = RelationalLimits {
                max_clauses: integer("--clauses", 3)? as usize,
                beam_width: integer("--beam", 6)? as usize,
                max_ast_nodes: integer("--ast", 256)? as usize,
                max_search_work: integer("--work", 2000000)?,
                ..RelationalLimits::default()
            };
            let learner = RelationalLearner::new(limits)?;
            let started = Instant::now();
            let fit = learner.fit(&examples, "shared-relational")?;
            let output = PathBuf::from(required("--output")?);
            std::fs::create_dir_all(&output).map_err(|e| e.to_string())?;
            let mut rows = Vec::new();
            for (i, candidate) in fit.candidates.iter().enumerate() {
                let source = candidate.program.to_string();
                let path = output.join(format!("candidate-{i}.policy"));
                std::fs::write(&path, &source).map_err(|e| e.to_string())?;
                rows.push(format!(
                    "{{\"id\":{i},\"digest\":{},\"bytes\":{},\"clauses\":{},\"ast_nodes\":{},\"weighted_cost\":{},\"path\":{}}}",
                    quote(&policy_search::program_digest(&source)),
                    source.len(),
                    candidate.program.rules.len(),
                    candidate.ast_nodes,
                    quote(&candidate.weighted_cost.to_string()),
                    quote(&path.to_string_lossy())
                ));
            }
            let m = &fit.metrics;
            let metrics=format!("{{\"grammar\":{},\"examples\":{},\"clauses\":{},\"programs_scored\":{},\"neighbor_proposals\":{},\"search_work\":{},\"cache_evaluations\":{},\"cache_hits\":{},\"inference_work\":{},\"budget_refusals\":{},\"search_cap_hit\":{},\"total_group_weight\":{},\"selected_weighted_cost\":{},\"selected_ast_nodes\":{}}}",
                quote(&m.grammar_version),m.examples,m.library_clauses,m.programs_scored,m.neighbor_proposals,m.search_work,m.clause_cache_evaluations,m.clause_cache_hits,m.inference_work,m.budget_refusals,m.search_cap_hit,quote(&m.total_group_weight.to_string()),quote(&m.selected_weighted_cost.to_string()),m.selected_ast_nodes);
            Ok(format!("{{\"schema\":\"relational-fit-v1\",\"examples\":{},\"elapsed_us\":{},\"metrics\":{metrics},\"candidates\":[{}]}}",examples.len(),started.elapsed().as_micros(),rows.join(",")))
        }
        "evaluate" => walt::policy_search::learning_eval::run(
            required("--request")?,
            required("--policies")?,
            required("--output")?,
            opts.get("--field").copied().unwrap_or("gym"),
            integer("--seed", 0)?,
            integer("--samples", 16)? as usize,
            integer("--work", 2000000)?,
            opts.get("--prices").copied().unwrap_or("on"),
        ),
        _ => unreachable!(),
    }
}
