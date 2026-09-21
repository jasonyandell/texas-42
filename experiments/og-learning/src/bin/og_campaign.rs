//! Campaign driver. Subcommands:
//!   init  --dir D [--train N] [--dev N] [--stall K]
//!   bench --deals N
//!   train --dir D [--generations N] [--wall-budget-secs S]
//!   panel --dir D [--seeds N]     (versioned dedup probe panel record)
//!   exam  --dir D [--deals N]     (final untouched exam)

use std::path::PathBuf;
use std::time::Instant;

use og_learning::actor::RationalActor;
use og_learning::campaign::{run_exam, run_generation, CampaignState, PANEL_BASE};
use og_learning::features::ClauseDictionary;
use og_learning::rollout::play_deal;
use og_learning::target::CampaignTarget;

fn arg(name: &str, default: u64) -> u64 {
    let args: Vec<String> = std::env::args().collect();
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn dir_arg() -> PathBuf {
    let args: Vec<String> = std::env::args().collect();
    let d = args
        .iter()
        .position(|a| a == "--dir")
        .and_then(|i| args.get(i + 1))
        .expect("--dir required");
    PathBuf::from(d)
}

fn main() -> Result<(), String> {
    let sub = std::env::args().nth(1).unwrap_or_default();
    let target = CampaignTarget::og_v1();
    let dict = ClauseDictionary::standard()?;
    match sub.as_str() {
        "init" => {
            let dir = dir_arg();
            std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
            let state = CampaignState::fresh(
                &target,
                &dict,
                arg("--train", 4096),
                arg("--dev", 2048),
                arg("--stall", 3) as u32,
            );
            state.save(&dir.join("state.txt"))?;
            println!(
                "initialized {} at {} (train={}, dev={}, stall={})",
                target.id,
                dir.display(),
                state.train_deals,
                state.dev_deals,
                state.stall_after
            );
            Ok(())
        }
        "bench" => {
            let n = arg("--deals", 200);
            let actor = RationalActor::uniform(dict.len(), &dict.version);
            let started = Instant::now();
            let mut makes = 0u64;
            let mut decisions = 0u64;
            let mut plies = 0u64;
            for i in 0..n {
                let r = play_deal(&target, &dict, &actor, 77_000_000 + i, true)?;
                makes += u64::from(r.y);
                decisions += u64::from(r.learner_decisions);
                plies += u64::from(r.plies);
            }
            let ms = started.elapsed().as_millis();
            println!(
                "bench: {n} deals in {ms} ms ({} ms/deal x1000), makes {makes}/{n}, \
                 mean learner decisions x100 {}, mean plies x100 {}",
                (ms * 1000) / u128::from(n),
                decisions * 100 / n,
                plies * 100 / n
            );
            Ok(())
        }
        "train" => {
            let dir = dir_arg();
            let generations = arg("--generations", 1);
            let wall_budget = arg("--wall-budget-secs", 480);
            let started = Instant::now();
            let mut state = CampaignState::load(&dir.join("state.txt"))?;
            for _ in 0..generations {
                let report = run_generation(&dir, &mut state, &target, &dict)?;
                println!("{}", report.json);
                if report.stalled {
                    println!("{{\"stopped\":\"registered stall condition\"}}");
                    break;
                }
                if started.elapsed().as_secs() >= wall_budget {
                    println!("{{\"paused\":\"wall budget reached; state saved\"}}");
                    break;
                }
            }
            Ok(())
        }
        "panel" => {
            let dir = dir_arg();
            let n = arg("--seeds", 256);
            let actor = RationalActor::uniform(dict.len(), &dict.version);
            // The versioned dedup probe panel (parent §5 step 4): collect
            // every clause's candidate set over the panel's decisions under
            // the uniform actor; report per-clause constancy and pairwise
            // agreement. Empirical agreement is NOT semantic equivalence.
            let mut per_clause_nontrivial = vec![0u64; dict.len()];
            let mut pair_disagree = vec![vec![0u64; dict.len()]; dict.len()];
            let mut decisions = 0u64;
            for i in 0..n {
                let stats =
                    og_learning::rollout::panel_decision_sets(&target, &dict, &actor, PANEL_BASE + i)?;
                for sets in stats {
                    decisions += 1;
                    for (j, s) in sets.iter().enumerate() {
                        if !s.trivial {
                            per_clause_nontrivial[j] += 1;
                        }
                        for (j2, s2) in sets.iter().enumerate().skip(j + 1) {
                            if s.bits != s2.bits {
                                pair_disagree[j][j2] += 1;
                            }
                        }
                    }
                }
            }
            let mut dupes = Vec::new();
            #[allow(clippy::needless_range_loop)]
            for j in 0..dict.len() {
                for j2 in (j + 1)..dict.len() {
                    if pair_disagree[j][j2] == 0 {
                        dupes.push(format!(
                            "[\"{}\",\"{}\"]",
                            dict.ids[j], dict.ids[j2]
                        ));
                    }
                }
            }
            let rows: Vec<String> = dict
                .ids
                .iter()
                .zip(&per_clause_nontrivial)
                .map(|(id, c)| format!("{{\"clause\":\"{id}\",\"nontrivial\":{c}}}"))
                .collect();
            let json = format!(
                "{{\"panel\":{{\"version\":\"{}\",\"seeds\":{},\"decisions\":{},\
                 \"clauses\":[{}],\"empirically_identical_pairs\":[{}],\
                 \"note\":\"empirical agreement on this panel is not semantic equivalence\"}}}}",
                dict.version,
                n,
                decisions,
                rows.join(","),
                dupes.join(",")
            );
            std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
            std::fs::write(dir.join("panel.json"), format!("{json}\n"))
                .map_err(|e| e.to_string())?;
            println!("{json}");
            Ok(())
        }
        "exam" => {
            let dir = dir_arg();
            let n = arg("--deals", 4096);
            let state = CampaignState::load(&dir.join("state.txt"))?;
            let json = run_exam(&dir, &state, &target, &dict, n)?;
            println!("{json}");
            Ok(())
        }
        other => Err(format!(
            "unknown subcommand {other:?}; use init | bench | train | panel | exam"
        )),
    }
}
