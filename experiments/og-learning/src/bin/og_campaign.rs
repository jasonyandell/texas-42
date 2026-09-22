//! Campaign driver. Subcommands:
//!   init  --dir D [--train N] [--dev N] [--stall K] [--offset S] [--constructor 0|1]
//!   bench --deals N
//!   train --dir D [--generations N] [--wall-budget-secs S]
//!   panel --dir D [--seeds N]     (versioned dedup probe panel record)
//!   exam  --dir D [--deals N]     (final untouched exam)

use std::path::PathBuf;
use std::time::Instant;

use og_learning::actor::RationalActor;
use og_learning::campaign::{run_exam, run_generation, CampaignState, PANEL_BASE};
use og_learning::features::{set_for, ClauseDictionary};
use og_learning::rollout::{collect_panel_decisions, play_deal, DEAL_WORK_BUDGET};
use og_learning::target::CampaignTarget;

fn arg(name: &str, default: u64) -> u64 {
    let args: Vec<String> = std::env::args().collect();
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn str_arg(name: &str, default: &str) -> String {
    let args: Vec<String> = std::env::args().collect();
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
        .unwrap_or_else(|| default.to_string())
}

fn field_target() -> Result<CampaignTarget, String> {
    match str_arg("--field", "hash").as_str() {
        "hash" => Ok(CampaignTarget::og_v1()),
        "l0-8" => Ok(CampaignTarget::og_v3()),
        "gym" => Ok(CampaignTarget::og_v4()),
        "gym-v5" => Ok(CampaignTarget::og_v5()),
        "gym4" => Ok(CampaignTarget::og_v5_proxy()),
        other => Err(format!(
            "unknown --field {other:?}; use hash | l0-8 | gym | gym-v5 | gym4"
        )),
    }
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
    match sub.as_str() {
        "init" => {
            let target = field_target()?;
            let dir = dir_arg();
            std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
            let dict = ClauseDictionary::standard()?;
            let mut state = CampaignState::fresh(
                &target,
                &dict,
                arg("--train", 4096),
                arg("--dev", 2048),
                arg("--stall", 3) as u32,
                arg("--offset", 0),
                arg("--constructor", 0) == 1,
            );
            state.promotion_mode = str_arg("--promotion", "mf");
            assert!(
                ["mf", "direct-eb", "anytime-direct"].contains(&state.promotion_mode.as_str())
            );
            state.save(&dir.join("state.txt"))?;
            println!(
                "initialized {} at {} (train={}, dev={}, stall={}, offset={}, constructor={})",
                target.id,
                dir.display(),
                state.train_deals,
                state.dev_deals,
                state.stall_after,
                state.seed_offset,
                state.constructor
            );
            Ok(())
        }
        "bench" => {
            let target = field_target()?;
            let n = arg("--deals", 200);
            let dict = ClauseDictionary::standard()?;
            let actor = RationalActor::uniform(dict.len(), &dict.version);
            let started = Instant::now();
            let mut makes = 0u64;
            let mut decisions = 0u64;
            let mut plies = 0u64;
            for i in 0..n {
                let r = play_deal(&target, &dict, &actor, 77_000_000 + i, true, &[])?;
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
            let target = CampaignTarget::from_id(&state.target_id)?;
            let mut dict = ClauseDictionary::with_learned(&state.learned)?;
            for _ in 0..generations {
                let report = run_generation(&dir, &mut state, &target, &mut dict)?;
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
            let state = CampaignState::load(&dir.join("state.txt"))?;
            let target = CampaignTarget::from_id(&state.target_id)?;
            let dict = ClauseDictionary::with_learned(&state.learned)?;
            let actor = RationalActor::uniform(dict.len(), &dict.version);
            // The versioned dedup probe panel (parent §5 step 4): every
            // expression's candidate set over the panel's decisions under
            // uniform play. Empirical agreement is NOT semantic equivalence.
            let mut per_clause_nontrivial = vec![0u64; dict.len()];
            let mut pair_disagree = vec![vec![0u64; dict.len()]; dict.len()];
            let mut decisions = 0u64;
            let mut budget = walt::scheme::Budget::new(DEAL_WORK_BUDGET * 64);
            for i in 0..n {
                let panel = collect_panel_decisions(
                    &target,
                    &dict,
                    &actor,
                    PANEL_BASE + state.seed_offset + i,
                )?;
                for (frame, legal) in panel {
                    decisions += 1;
                    let sets: Vec<(u32, bool)> = dict
                        .compiled()
                        .iter()
                        .map(|fix| {
                            let s = set_for(fix, &frame, legal, &mut budget)?;
                            Ok::<(u32, bool), String>((
                                s.bits(),
                                s.is_empty() || s == legal,
                            ))
                        })
                        .collect::<Result<_, _>>()?;
                    for (j, (bits, trivial)) in sets.iter().enumerate() {
                        if !trivial {
                            per_clause_nontrivial[j] += 1;
                        }
                        for (j2, (bits2, _)) in sets.iter().enumerate().skip(j + 1) {
                            if bits != bits2 {
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
                        dupes.push(format!("[\"{}\",\"{}\"]", dict.ids[j], dict.ids[j2]));
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
        "pilot" => {
            // The §6 pilot, run properly: a REAL policy pair (the given
            // campaign's incumbent vs uniform) evaluated under both the
            // expensive target and its declared proxy on paired seeds.
            let dir = dir_arg();
            let m = arg("--deals", 1024);
            let state = CampaignState::load(&dir.join("state.txt"))?;
            let base_target = CampaignTarget::from_id(&state.target_id)?;
            let proxy = match str_arg("--proxy", "declared").as_str() {
                "declared" => base_target
                    .proxy()
                    .ok_or_else(|| "target declares no proxy".to_string())?,
                "gym4" => CampaignTarget::og_v5_proxy(),
                "l0-8" => CampaignTarget::og_v4_proxy(),
                other => return Err(format!("unknown --proxy {other:?}")),
            };
            let dict = ClauseDictionary::with_learned(&state.learned)?;
            let actor = og_learning::actor::RationalActor {
                weights: state.weights.clone(),
                dictionary_version: dict.version.clone(),
            };
            let uniform = RationalActor::uniform(dict.len(), &dict.version);
            let base = 800_000_000u64 + state.seed_offset;
            let mut dl = og_learning::promotion::BatchStats::default();
            let mut dh = og_learning::promotion::BatchStats::default();
            let mut dc = og_learning::promotion::BatchStats::default();
            let mut sum_dhdl = 0i64;
            let mut h_ms = 0u128;
            let mut l_ms = 0u128;
            for i in 0..m {
                let t0 = Instant::now();
                let ha = play_deal(&base_target, &dict, &actor, base + i, false, &[])?;
                let hb = play_deal(&base_target, &dict, &uniform, base + i, false, &[])?;
                h_ms += t0.elapsed().as_millis();
                let t1 = Instant::now();
                let la = play_deal(&proxy, &dict, &actor, base + i, false, &[])?;
                let lb = play_deal(&proxy, &dict, &uniform, base + i, false, &[])?;
                l_ms += t1.elapsed().as_millis();
                let vh = i64::from(ha.y) - i64::from(hb.y);
                let vl = i64::from(la.y) - i64::from(lb.y);
                dh.push(vh);
                dl.push(vl);
                dc.push(vh - vl);
                sum_dhdl += vh * vl;
            }
            let fr = |n: i64, d: u64| format!("{n}/{d}");
            let json = format!(
                "{{\"pilot\":{{\"target\":\"{}\",\"proxy\":\"{}\",\"actor_digest\":\"{:016x}\",\
                 \"deals\":{m},\"mean_dh\":\"{}\",\"mean_dl\":\"{}\",\
                 \"var_dh\":\"{}/{}\",\"var_dl\":\"{}/{}\",\"var_dc\":\"{}/{}\",\
                 \"sum_dhdl\":{sum_dhdl},\"h_ms_per_pair\":{},\"l_ms_per_pair\":{}}}}}",
                base_target.id,
                proxy.id,
                actor.digest(),
                fr(dh.sum, m),
                fr(dl.sum, m),
                dh.sample_variance().numer(), dh.sample_variance().denom(),
                dl.sample_variance().numer(), dl.sample_variance().denom(),
                dc.sample_variance().numer(), dc.sample_variance().denom(),
                h_ms / u128::from(m),
                l_ms / u128::from(m)
            );
            std::fs::write(dir.join(format!("pilot-{}.json", proxy.id.split('/').next().unwrap_or("proxy"))), format!("{json}\n")).map_err(|e| e.to_string())?;
            println!("{json}");
            Ok(())
        }
        "exam" => {
            let dir = dir_arg();
            let n = arg("--deals", 4096);
            let state = CampaignState::load(&dir.join("state.txt"))?;
            let target = CampaignTarget::from_id(&state.target_id)?;
            let dict = ClauseDictionary::with_learned(&state.learned)?;
            let json = run_exam(&dir, &state, &target, &dict, n)?;
            println!("{json}");
            Ok(())
        }
        other => Err(format!(
            "unknown subcommand {other:?}; use init | bench | train | panel | pilot | exam"
        )),
    }
}
