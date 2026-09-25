//! Offline only: bounded counterexample augmentation, matched sample-count
//! control, singleton doom classification, and a fresh examination panel.
use serde::Deserialize;
use serde_json::{json, Value};
use std::{
    collections::BTreeSet,
    fs,
    time::{Duration, Instant},
};
use walt::{
    rules::{Decl, Seat},
    solver::{
        self,
        nello_counterexample::{Lab, Plan, Root, World},
        Contract, Deadline, Key,
    },
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    contract: String,
    decl: usize,
    bid: usize,
    bidder: usize,
    seat: usize,
    hand: Vec<usize>,
    plays: Vec<usize>,
    seed: u64,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    request: Request,
    ordinary: usize,
    rounds: usize,
    witnesses_per_round: usize,
    attack_pool: usize,
    examination: usize,
    n0: usize,
    budget_ms: u64,
}

fn selected(plans: &[Plan], allowed: &[u8]) -> u8 {
    plans
        .iter()
        .filter(|p| allowed.contains(&p.action))
        .min_by(|a, b| {
            a.declarer_value
                .cmp(&b.declarer_value)
                .then(a.action.cmp(&b.action))
        })
        .unwrap()
        .action
}
fn trace_json(t: &solver::nello_counterexample::Trace) -> Value {
    json!({"set":t.set,"plays_internal":t.plays,"viewer_decisions":t.viewer_decisions,"fallback_decisions":t.fallback_decisions,
        "viewer_free_decisions":t.viewer_free_decisions,"fallback_free_decisions":t.fallback_free_decisions})
}
fn inspect(lab: &Lab, plans: &[Plan], worlds: &[World]) -> Option<Value> {
    let mut rows = Vec::new();
    for p in plans {
        let (mut sets, mut decisions, mut fallback) = (0, 0, 0);
        let (mut free, mut fallback_free) = (0, 0);
        for w in worlds {
            let t = lab.replay(p, w)?;
            sets += usize::from(t.set);
            decisions += t.viewer_decisions;
            fallback += t.fallback_decisions;
            free += t.viewer_free_decisions;
            fallback_free += t.fallback_free_decisions;
        }
        rows.push(json!({"action":p.action,"sets":sets,"worlds":worlds.len(),"viewer_decisions":decisions,"fallback_decisions":fallback,
            "viewer_free_decisions":free,"fallback_free_decisions":fallback_free}));
    }
    Some(json!(rows))
}
fn prices(plans: &[Plan]) -> Value {
    json!(plans.iter().map(|p|json!({"action":p.action,"declarer_make":[p.declarer_value.numer().to_string(),p.declarer_value.denom().to_string()],"policy_nodes":p.decision_count()})).collect::<Vec<_>>())
}

fn run(config: Config) -> Result<Value, String> {
    let started = Instant::now();
    let req = &config.request;
    if req.contract != "nello"
        || req.decl != 8
        || config.ordinary == 0
        || config.ordinary > 640
        || config.rounds > 3
        || config.witnesses_per_round > 4
        || config.attack_pool > 1024
        || config.examination == 0
        || config.examination > 10000
        || !(1..=64).contains(&config.n0)
        || !(100..=240000).contains(&config.budget_ms)
    {
        return Err("invalid experiment bounds".into());
    }
    let words = |v: &[usize]| v.iter().map(usize::to_string).collect::<Vec<_>>().join(" ");
    let wire = format!(
        "status\ncontract 1\ndecl 8\nbid {}\nbidder {}\nseat {}\nhand {}\nplays {}\nseed {}\n",
        req.bid,
        req.bidder,
        req.seat,
        words(&req.hand),
        words(&req.plays),
        req.seed
    );
    solver::partnership_wire::run(&wire)?;
    let pairs = req
        .plays
        .chunks_exact(2)
        .map(|p| (p[0], p[1]))
        .collect::<Vec<_>>();
    let st = solver::replay_contract(Decl::DoublesSuit, req.bidder, &pairs, true);
    let original_hand = req.hand.iter().fold(0u32, |m, &t| m | (1u32 << t));
    let root = Root {
        key: Key {
            voids: None,
            played: st.played,
            leader: st.leader,
            plays: st.plays,
            banked_t1: st.banked_t1,
            banked_t0: st.banked_t0,
            alive: 0,
        },
        viewer: Seat::from_index((req.seat + st.r) % 4).unwrap(),
        hand: original_hand & !st.played,
        original_hand,
        voids: st.voids,
        contract: Contract::Nello {
            declarer: Seat::from_index((req.bidder + st.r) % 4).unwrap(),
        },
    };
    let deadline = Deadline::after(Duration::from_millis(config.budget_ms));
    let lab =
        Lab::new(root, config.n0, deadline).ok_or("requires an active Nel-O defender root")?;
    let ordinary = lab
        .root
        .sample(config.ordinary, req.seed, deadline)
        .ok_or("ordinary sampling refused")?;
    let baseline = lab
        .train(&ordinary)
        .ok_or("baseline training/extraction refused")?;
    let legal = lab.root.legal();
    let baseline_choice = selected(&baseline, &legal);
    let best = &baseline
        .iter()
        .find(|p| p.action == baseline_choice)
        .unwrap()
        .declarer_value;
    let baseline_ties = baseline
        .iter()
        .filter(|p| &p.declarer_value == best)
        .map(|p| p.action)
        .collect::<Vec<_>>();
    let mut snapshots = vec![("baseline".to_string(), baseline)];
    let mut witnesses = Vec::<World>::new();
    let mut seen: BTreeSet<World> = ordinary.iter().copied().collect();
    let mut attacks = Vec::new();
    for round in 0..config.rounds {
        let before = &snapshots.last().unwrap().1;
        let pool = lab
            .root
            .sample(
                config.attack_pool,
                req.seed ^ solver::mix(0x41545441434b + round as u64),
                deadline,
            )
            .ok_or("attack sampling refused")?;
        let mut outcomes = Vec::new();
        for w in &pool {
            outcomes.push(
                before
                    .iter()
                    .map(|p| lab.replay(p, w))
                    .collect::<Option<Vec<_>>>()
                    .ok_or("attack replay refused")?,
            );
        }
        let mut found = Vec::new();
        for offset in 0..before.len() {
            if found.len() == config.witnesses_per_round {
                break;
            }
            let index = (offset + round + req.seed as usize % before.len()) % before.len();
            // Prefer a discriminating witness. Still retain a common failure
            // if no discriminating one was found for this action.
            let pick = (0..pool.len())
                .find(|&i| {
                    !seen.contains(&pool[i])
                        && !outcomes[i][index].set
                        && outcomes[i].iter().any(|t| t.set)
                })
                .or_else(|| {
                    (0..pool.len()).find(|&i| !seen.contains(&pool[i]) && !outcomes[i][index].set)
                });
            if let Some(i) = pick {
                seen.insert(pool[i]);
                witnesses.push(pool[i]);
                let doom = before
                    .iter()
                    .map(|p| Some(json!({"action":p.action,"doomed":lab.doom(p.action,&pool[i])?})))
                    .collect::<Option<Vec<_>>>()
                    .ok_or("singleton doom refused")?;
                found.push(json!({"world_internal_masks":pool[i],"target_action":before[index].action,
                    "before":before.iter().zip(&outcomes[i]).map(|(p,t)|json!({"action":p.action,"trace":trace_json(t)})).collect::<Vec<_>>(),"doom":doom}));
            }
        }
        let failures=before.iter().enumerate().map(|(i,p)|json!({"action":p.action,"failures":outcomes.iter().filter(|ts|!ts[i].set).count()})).collect::<Vec<_>>();
        let mut bundle = ordinary.clone();
        bundle.extend_from_slice(&witnesses);
        let after = lab
            .train(&bundle)
            .ok_or("augmented training/extraction refused")?;
        let replay = inspect(&lab, &after, &witnesses).ok_or("witness replay refused")?;
        attacks.push(json!({"round":round+1,"pool_worlds":pool.len(),"failures":failures,"new_witnesses":found,
            "retained_witnesses":witnesses.len(),"training":prices(&after),"replay_retained_witnesses":replay}));
        snapshots.push((format!("round-{}", round + 1), after));
    }
    let training_ms = started.elapsed().as_millis();
    let mut control = ordinary.clone();
    if !witnesses.is_empty() {
        control.extend(
            lab.root
                .sample(
                    witnesses.len(),
                    req.seed ^ solver::mix(0x434f4e54524f4c),
                    deadline,
                )
                .ok_or("control sampling refused")?,
        );
    }
    snapshots.push((
        "random-control".to_string(),
        lab.train(&control)
            .ok_or("control training/extraction refused")?,
    ));
    // Examination is generated only AFTER every candidate policy and selection
    // is frozen. No examination outcome enters witness selection or replanning.
    let examination = lab
        .root
        .sample(
            config.examination,
            req.seed ^ solver::mix(0x4558414d),
            deadline,
        )
        .ok_or("examination sampling refused")?;
    let mut evaluations = Vec::new();
    for (label, plans) in &snapshots {
        evaluations.push(json!({"label":label,"choice":selected(plans,&legal),"tie_only_choice":selected(plans,&baseline_ties),
            "training":prices(plans),"ordinary_replay":inspect(&lab,plans,&ordinary).ok_or("ordinary replay refused")?,
            "examination":inspect(&lab,plans,&examination).ok_or("examination replay refused")?}));
    }
    let final_plans = &snapshots[snapshots.len() - 2].1;
    let bplan = snapshots[0]
        .1
        .iter()
        .find(|p| p.action == baseline_choice)
        .unwrap();
    let fchoice = selected(final_plans, &legal);
    let fplan = final_plans.iter().find(|p| p.action == fchoice).unwrap();
    let control_plans = &snapshots.last().unwrap().1;
    let cchoice = selected(control_plans, &legal);
    let cplan = control_plans.iter().find(|p| p.action == cchoice).unwrap();
    let tchoice = selected(final_plans, &baseline_ties);
    let tplan = final_plans.iter().find(|p| p.action == tchoice).unwrap();
    let mut pairs = [[0usize; 4]; 3];
    for w in &examination {
        let b = lab.replay(bplan, w).ok_or("paired baseline refused")?.set;
        for (row, plan) in pairs.iter_mut().zip([fplan, cplan, tplan]) {
            let a = lab.replay(plan, w).ok_or("paired candidate refused")?.set;
            row[usize::from(b) * 2 + usize::from(a)] += 1;
        }
    }
    Ok(
        json!({"schema":"nello-counterexample-probe-v1","status":"completed","rotation":st.r,
        "seed":req.seed,"ordinary":config.ordinary,"inner_worlds":config.n0,"attack_pool":config.attack_pool,
        "rounds":config.rounds,"witness_limit":config.witnesses_per_round,"witness_count":witnesses.len(),
        "examination_worlds":config.examination,"baseline_ties":baseline_ties,"attacks":attacks,"evaluations":evaluations,
        "paired_order":["both_escape","candidate_only_set","baseline_only_set","both_set"],
        "paired_augmented":pairs[0],"paired_random_control":pairs[1],"paired_tie_only":pairs[2],
        "counterexample_phase_ms":training_ms,"total_ms":started.elapsed().as_millis(),
        "policy_completion":"fixed own/public L0 on histories absent from extracted tree",
        "field":"same deterministic L0/voidless player as discovery; no clairvoyant declarer",
        "claim":"exploratory single-world fixed-field doom and policy failures; no counted-mass or strength claim"}),
    )
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        eprintln!("usage: nello-counterexample INPUT_JSON OUTPUT_JSON");
        std::process::exit(2);
    }
    let input = fs::read_to_string(&args[1]).expect("read config");
    let result = serde_json::from_str(&input)
        .map_err(|e| e.to_string())
        .and_then(run);
    let ok = result.is_ok();
    let mut output = result.unwrap_or_else(|error| json!({"status":"refused","error":error}));
    output["input"] = serde_json::from_str(&input).unwrap_or(Value::Null);
    fs::write(
        &args[2],
        format!("{}\n", serde_json::to_string_pretty(&output).unwrap()),
    )
    .expect("write artifact");
    println!(
        "{}",
        json!({"status":output["status"],"witnesses":output["witness_count"],"paired_augmented":output["paired_augmented"],"total_ms":output["total_ms"],"error":output["error"]})
    );
    if !ok {
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retained_counterexamples_break_a_pinned_saturated_tie() {
        // Deliberately tiny discovery bundle: a mechanism regression, not a
        // strength test. The fixture uses only Ruby's hand and public history.
        let result = run(Config {
            request: Request {
                contract: "nello".into(),
                decl: 8,
                bid: 1,
                bidder: 0,
                seat: 3,
                hand: vec![4, 7, 12, 14, 16, 25, 27],
                plays: vec![0, 3, 1, 23, 3, 12, 1, 11, 3, 25, 0, 13],
                seed: 1,
            },
            ordinary: 1,
            rounds: 1,
            witnesses_per_round: 4,
            attack_pool: 256,
            examination: 1,
            n0: 8,
            budget_ms: 20000,
        })
        .unwrap();
        assert_eq!(result["baseline_ties"], json!([4, 7, 14, 16, 27]));
        let before = result["evaluations"][0]["training"].as_array().unwrap();
        assert!(before
            .iter()
            .all(|p| p["declarer_make"] == json!(["0", "1"])));
        assert_eq!(result["witness_count"], 4);
        assert_eq!(result["attacks"][0]["retained_witnesses"], 4);
        let after = result["evaluations"][1]["training"].as_array().unwrap();
        let values: Vec<_> = after.iter().map(|p| p["declarer_make"].clone()).collect();
        assert_eq!(
            values,
            vec![
                json!(["2", "5"]),
                json!(["2", "5"]),
                json!(["1", "5"]),
                json!(["0", "1"]),
                json!(["1", "5"])
            ]
        );
        assert_eq!(result["evaluations"][1]["tie_only_choice"], 16);
        // Training includes every retained world, so none use off-tree fallback.
        for replay in result["attacks"][0]["replay_retained_witnesses"]
            .as_array()
            .unwrap()
        {
            assert_eq!(replay["worlds"], 4);
            assert_eq!(replay["fallback_decisions"], 0);
        }
    }

    #[test]
    fn discovery_values_match_live_wire_on_both_ruby_roots() {
        for ply in [6, 9] {
            let mut plays = vec![0, 3, 1, 23, 3, 12, 1, 11, 3, 25, 0, 13];
            if ply == 9 {
                plays.extend([3, 27, 0, 9, 1, 2]);
            }
            let text = plays
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(" ");
            let wire=format!("baseline\ncontract 1\ndecl 8\nbid 1\nbidder 0\nseat 3\nhand 4 7 12 14 16 25 27\nplays {text}\nseed 1\nn 160\nn0 8\nn1 2\nbudget_ms 20000\ninner_belief 0\nselection 0\nmodeled_selection 0\n");
            let reference: Value =
                serde_json::from_str(&solver::partnership_wire::run(&wire).unwrap()).unwrap();
            let probe = run(Config {
                request: Request {
                    contract: "nello".into(),
                    decl: 8,
                    bid: 1,
                    bidder: 0,
                    seat: 3,
                    hand: vec![4, 7, 12, 14, 16, 25, 27],
                    plays,
                    seed: 1,
                },
                ordinary: 160,
                rounds: 0,
                witnesses_per_round: 0,
                attack_pool: 0,
                examination: 16,
                n0: 8,
                budget_ms: 20000,
            })
            .unwrap();
            let expected = reference["options"].as_array().unwrap();
            let actual = probe["evaluations"][0]["training"].as_array().unwrap();
            assert_eq!(expected.len(), actual.len());
            for (e, a) in expected.iter().zip(actual) {
                assert_eq!(e[0], a["action"]);
                assert_eq!(e[1], a["declarer_make"][0]);
                assert_eq!(e[2], a["declarer_make"][1]);
            }
            assert_eq!(reference["choice"], probe["evaluations"][0]["choice"]);
        }
    }
}
