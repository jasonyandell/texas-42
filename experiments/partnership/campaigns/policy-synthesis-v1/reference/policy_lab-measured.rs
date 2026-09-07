//! One atomic seed's paired policy-construction experiment. External runner
//! owns wall deadlines, process groups, retries and checkpoint publication.
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::time::Instant;
use walt::policy_search::{self, ActionPool, HashField, Search, TablePolicy, Work};
use walt::rules::{Domino, DominoSet};
use walt::solver::adaptive::{root_identity, SlicePolicy};
use walt::solver::policy::Level0Field;

fn quote(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if c <= '\u{1f}' => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
fn main() {
    match run() {
        Ok(s) => println!("{s}"),
        Err(e) => {
            eprintln!("policy_lab: {e}");
            std::process::exit(1);
        }
    }
}
fn run() -> Result<String, String> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args == ["--help"] {
        return Ok("Usage: policy_lab --seed N --tiles 2..7 --samples 1,2,4,8 --test-worlds N --node-budget N --decl 0..7|9 --mode random-own-hand|fixed-root-hand [--hand seven,ids] [--field hash-legal|l0-8] [--artifact-dir PATH]".into());
    }
    let mut options = BTreeMap::new();
    for pair in args.chunks(2) {
        if pair.len() != 2
            || ![
                "--seed",
                "--tiles",
                "--samples",
                "--test-worlds",
                "--node-budget",
                "--decl",
                "--mode",
                "--hand",
                "--artifact-dir",
                "--field",
                "--request",
                "--exact-test",
            ]
            .contains(&pair[0].as_str())
            || options.insert(pair[0].as_str(), pair[1].as_str()).is_some()
        {
            return Err("invalid or duplicate option".into());
        }
    }
    let value = |k: &str, default: &str| options.get(k).copied().unwrap_or(default).to_string();
    let integer = |k: &str, default: u64| -> Result<u64, String> {
        options.get(k).map_or(Ok(default), |v| {
            v.parse().map_err(|_| format!("invalid {k}"))
        })
    };
    let seed = integer("--seed", 420600)?;
    let tiles = integer("--tiles", 7)? as usize;
    let test_n = integer("--test-worlds", 128)? as usize;
    let node_limit = integer("--node-budget", 2_000_000)?;
    let exact_test = match integer("--exact-test", 0)? {
        0 => false,
        1 => true,
        _ => return Err("exact-test must be 0 or 1".into()),
    };
    let decl_id = integer("--decl", 0)? as usize;
    if !(2..=7).contains(&tiles)
        || !(1..=10000).contains(&test_n)
        || !(1..=20_000_000).contains(&node_limit)
        || ![0, 1, 2, 3, 4, 5, 6, 7, 9].contains(&decl_id)
    {
        return Err("option outside supported bounds".into());
    }
    let schedule = value("--samples", "1,2,4,8,16,32")
        .split(',')
        .map(str::parse::<usize>)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    if schedule.is_empty()
        || schedule[0] == 0
        || schedule.last().unwrap() > &512
        || schedule.windows(2).any(|s| s[0] >= s[1])
    {
        return Err("samples must strictly increase in 1..512".into());
    }
    let mode = if options.contains_key("--request") {
        "request".into()
    } else {
        value("--mode", "random-own-hand")
    };
    if !["random-own-hand", "fixed-root-hand", "request"].contains(&mode.as_str()) {
        return Err("invalid mode".into());
    }
    let fixed = if let Some(hand) = options.get("--hand") {
        let ids = hand
            .split(',')
            .map(str::parse::<usize>)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;
        let mut set = DominoSet::EMPTY;
        for id in ids {
            if !set.insert(Domino::from_index(id).ok_or("invalid hand tile")?) {
                return Err("duplicate hand tile".into());
            }
        }
        if set.len() != 7 {
            return Err("hand must have seven tiles".into());
        }
        Some(set)
    } else {
        None
    };
    if (mode == "fixed-root-hand") != fixed.is_some() {
        return Err("fixed-root-hand requires --hand, random mode forbids it".into());
    }
    let fixture = if let Some(path) = options.get("--request") {
        policy_search::request::from_text(
            &std::fs::read_to_string(path).map_err(|e| e.to_string())?,
        )?
    } else {
        policy_search::fixture(seed, walt::solver::decl_of(decl_id), tiles, fixed)?
    };
    let tiles = fixture.exercise.root.kernel().viewer_hand().len();
    let decl_id = walt::solver::arena_decl_id(fixture.exercise.position.decl);
    let field_name = value("--field", "hash-legal");
    let field: Box<dyn SlicePolicy> = match field_name.as_str() {
        "hash-legal" => Box::new(HashField),
        "l0-8" => Box::new(Level0Field::new(8)),
        "gym" => Box::new(walt::gym::GymField::new(
            fixture.exercise.root.kernel().viewer(),
            40,
        )),
        _ => return Err("invalid field".into()),
    };
    let root = &fixture.exercise;
    let identity = root_identity(&root.root, &root.position);
    if exact_test && root.root.count() > 10_000 {
        return Err("exact-test fiber exceeds 10000 worlds".into());
    }
    let training = (0..*schedule.last().unwrap())
        .map(|i| {
            root.root
                .world_at(identity, seed ^ 0x0054_5241_494e, i as u64)
        })
        .collect::<Vec<_>>();
    let testing = (0..test_n)
        .map(|i| root.root.world_at(identity, seed ^ 0x5445_5354, i as u64))
        .collect::<Vec<_>>();
    let artifacts = options.get("--artifact-dir").map(PathBuf::from);
    if let Some(path) = &artifacts {
        std::fs::create_dir_all(path).map_err(|e| e.to_string())?;
    }
    let mut persistent = Search::new(root, &*field);
    let mut donors = Search::new(root, &*field);
    let mut composed = Search::new(root, &*field);
    let mut pool = ActionPool::new();
    let mut donor_count = 0;
    let mut incumbents = [
        TablePolicy::default(),
        TablePolicy::default(),
        TablePolicy::default(),
    ];
    let mut solved_samples = [0usize; 3];
    let mut rows = Vec::new();
    let mut replay_records = Vec::new();
    for &n in &schedule {
        while persistent.samples() < n {
            let w = training[persistent.samples()];
            persistent.append(w)?;
            donors.append(w)?;
            composed.append(w)?;
        }
        let mut fresh = Search::new(root, &*field);
        for &w in &training[..n] {
            fresh.append(w)?;
        }
        // Rotate ordering to avoid consistently favouring one method's wall
        // measurement. No shared mutable search or field cache across arms.
        for offset in 0..3 {
            let arm = (offset + (seed as usize + n) % 3) % 3;
            let mut work = Work::new(node_limit);
            let start = Instant::now();
            let result = match arm {
                0 => fresh.solve(&mut work),
                1 => persistent.solve(&mut work),
                _ => {
                    let mut refusal = None;
                    while donor_count < n {
                        match donors.donor(donor_count, &mut pool, &mut work) {
                            Ok(_) => donor_count += 1,
                            Err(e) => {
                                refusal = Some(e);
                                break;
                            }
                        }
                    }
                    if let Some(e) = refusal {
                        Err(e)
                    } else {
                        composed.compose(&pool, &mut work)
                    }
                }
            };
            let elapsed_us = start.elapsed().as_micros();
            let (status, expected, reason) = match result {
                Ok(node) => {
                    incumbents[arm] = TablePolicy::from_node(&node);
                    solved_samples[arm] = n;
                    ("completed", Some(node.makes), None)
                }
                Err(e) if e == "node budget exhausted" => ("budget", None, Some(e)),
                Err(e) => return Err(e),
            };
            let policy = &incumbents[arm];
            let mut train_makes = 0;
            let mut test_makes = 0;
            let mut train_bits = Vec::new();
            let mut test_bits = Vec::new();
            for world in &training[..n] {
                let (made, _) = policy_search::replay(root, &*field, world, policy)?;
                train_makes += usize::from(made);
                train_bits.push(usize::from(made));
            }
            for world in &testing {
                let (made, _) = policy_search::replay(root, &*field, world, policy)?;
                test_makes += usize::from(made);
                test_bits.push(usize::from(made));
            }
            if expected.is_some_and(|e| e != train_makes) {
                return Err("extracted policy failed independent repricing".into());
            }
            let arm_name = ["fresh", "persistent", "compose"][arm];
            let program = policy_search::program::export(&fixture, policy, "sampled-policy")?;
            let source = program.to_string();
            let restored: walt::scheme::PolicyProgram = source
                .parse()
                .map_err(|e: walt::scheme::Error| e.to_string())?;
            let compiled = restored
                .compile(&walt::scheme::Registry::standard())
                .map_err(|e| e.to_string())?;
            // Reprice the actual serialized executable, not only its source table.
            for world in training[..n].iter().chain(testing.iter()) {
                let a = policy_search::replay(root, &*field, world, policy)?;
                let b =
                    policy_search::program::replay_program(&fixture, &*field, world, &compiled)?;
                if a != b {
                    return Err("serialized policy replay disagrees with table".into());
                }
            }
            let policy_id = policy_search::program_digest(&source);
            let root_action = policy
                .choose(
                    &policy_search::State::from_root(&root.position),
                    walt::rules::legal_plays(
                        root.position.decl,
                        root.root.kernel().viewer_hand(),
                        root.frame.led_context(),
                    ),
                )
                .index();
            let exact_makes = if exact_test {
                let mut makes = 0usize;
                for world in root.root.worlds() {
                    makes += usize::from(
                        policy_search::program::replay_program(
                            &fixture, &*field, &world, &compiled,
                        )?
                        .0,
                    );
                }
                Some(makes)
            } else {
                None
            };
            if let Some(dir) = &artifacts {
                std::fs::write(dir.join(format!("{arm_name}-{n}.policy")), &source)
                    .map_err(|e| e.to_string())?;
            }
            let cache_entries = match arm {
                0 => fresh.cache_len(),
                1 => persistent.cache_len(),
                _ => composed.cache_len(),
            };
            rows.push(format!("{{\"samples\":{n},\"arm\":{},\"status\":{},\"reason\":{},\"solved_samples\":{},\"train_makes\":{train_makes},\"train_worlds\":{n},\"test_makes\":{test_makes},\"test_worlds\":{test_n},\"train_bits\":{train_bits:?},\"test_bits\":{test_bits:?},\"nodes\":{},\"cache_hits\":{},\"field_calls\":{},\"cache_entries\":{cache_entries},\"donors\":{},\"pool_states\":{},\"policy_states\":{},\"policy_bytes\":{},\"policy_id\":{},\"root_action\":{root_action},\"exact_policy_makes\":{},\"exact_policy_worlds\":{},\"elapsed_ms\":{},\"elapsed_us\":{elapsed_us}}}",quote(arm_name),quote(status),reason.as_ref().map_or("null".into(),|r|quote(r)),solved_samples[arm],work.nodes,work.hits,work.field_calls,if arm==2{donor_count}else{0},if arm==2{pool.len()}else{0},policy.choices.len(),source.len(),quote(&policy_id),exact_makes.map_or("null".into(),|n|n.to_string()),if exact_test{root.root.count().to_string()}else{"null".into()},elapsed_us/1000));
            if n == *schedule.last().unwrap() {
                let (made, trace) = policy_search::replay(root, &*field, &testing[0], policy)?;
                let hands = walt::rules::Seat::ALL.map(|s| {
                    testing[0]
                        .hand(s)
                        .iter()
                        .map(Domino::index)
                        .collect::<Vec<_>>()
                });
                let history = fixture
                    .history
                    .iter()
                    .chain(trace.iter())
                    .map(|(s, d)| [s.index(), d.index()])
                    .collect::<Vec<_>>();
                replay_records.push(format!("{{\"arm\":{},\"made\":{made},\"remaining_hands\":{hands:?},\"history\":{history:?}}}",quote(arm_name)));
            }
        }
    }
    let hand = fixture
        .original
        .iter()
        .map(Domino::index)
        .collect::<Vec<_>>();
    let prefix = fixture
        .history
        .iter()
        .map(|(s, d)| [s.index(), d.index()])
        .collect::<Vec<_>>();
    Ok(format!("{{\"schema\":\"policy-lab-v1\",\"seed\":{seed},\"tiles\":{tiles},\"decl\":{decl_id},\"bid\":30,\"mode\":{},\"field\":{},\"hand\":{hand:?},\"root_history\":{prefix:?},\"root_worlds\":{},\"rows\":[{}],\"replays\":[{}]}}",quote(&mode),quote(field.id()),root.root.count(),rows.join(","),replay_records.join(",")))
}
