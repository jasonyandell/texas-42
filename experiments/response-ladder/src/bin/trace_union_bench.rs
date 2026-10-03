//! Bounded, resumable exact-compute control; model quality is a separate study.
#[cfg(not(feature = "gpu"))]
fn main() {
    eprintln!("trace_union_bench requires --features gpu");
    std::process::exit(2);
}

#[cfg(feature = "gpu")]
mod app {
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};
    use std::{
        fs,
        io::Write,
        path::{Path, PathBuf},
        process::{Command, Stdio},
        time::{Duration, Instant, SystemTime, UNIX_EPOCH},
    };
    use walt::solver::{decl_of, SplitMix64};
    use walt_response_ladder::{
        compiled_family::Family,
        compiled_field::CompiledField,
        core::{self, Config},
        gpu_epochs::EpochEvaluator,
        mechanics::{tiles, Budget, Field, Problem, PublicState, Scenario},
        model,
        policy::{price, Policy},
        rollout,
        trace_union::{self, Trace, TraceBundle},
    };

    const SCHEMA: &str = "trace-union-compute-control-v1";
    #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct Options {
        output: PathBuf,
        c0: PathBuf,
        c1: PathBuf,
        seconds: u64,
        n: usize,
        depths: Vec<usize>,
        seed: u64,
        per_backend_ms: u64,
        scalar: bool,
    }
    #[derive(Clone, Debug, Serialize, Deserialize)]
    struct ProblemWire {
        declaration: usize,
        bid: u8,
        viewer: u8,
        hand: u32,
        root: PublicState,
        scenarios: Vec<Scenario>,
        field_revision: String,
    }
    impl ProblemWire {
        fn from(p: &Problem, declaration: usize) -> Self {
            Self {
                declaration,
                bid: p.bid,
                viewer: p.viewer,
                hand: p.hand,
                root: p.root.clone(),
                scenarios: p.scenarios.clone(),
                field_revision: p.field_revision.clone(),
            }
        }
        fn problem(&self) -> Problem {
            Problem {
                decl: decl_of(self.declaration),
                bid: self.bid,
                viewer: self.viewer,
                hand: self.hand,
                root: self.root.clone(),
                scenarios: self.scenarios.clone(),
                field_revision: self.field_revision.clone(),
            }
        }
    }
    fn io(e: impl std::fmt::Display) -> String {
        e.to_string()
    }
    fn micros(t: Instant) -> u64 {
        t.elapsed().as_micros().min(u128::from(u64::MAX)) as u64
    }
    fn atomic(path: &Path, value: &Value) -> Result<(), String> {
        let tmp = path.with_extension("json.tmp");
        let mut f = fs::File::create(&tmp).map_err(io)?;
        serde_json::to_writer(&mut f, value).map_err(io)?;
        f.write_all(b"\n").map_err(io)?;
        f.sync_all().map_err(io)?;
        fs::rename(tmp, path).map_err(io)
    }
    fn read(path: &Path) -> Result<Value, String> {
        serde_json::from_slice(&fs::read(path).map_err(io)?).map_err(io)
    }
    fn hash(bytes: &[u8]) -> Result<String, String> {
        // Native benchmark provenance only, outside backend solve intervals.
        let mut child = Command::new("shasum")
            .args(["-a", "256"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(io)?;
        child
            .stdin
            .take()
            .ok_or("missing hash stdin")?
            .write_all(bytes)
            .map_err(io)?;
        let output = child.wait_with_output().map_err(io)?;
        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).into_owned());
        }
        let digest = String::from_utf8(output.stdout)
            .map_err(io)?
            .split_whitespace()
            .next()
            .ok_or("missing sha256")?
            .to_owned();
        if digest.len() != 64 || !digest.bytes().all(|c| c.is_ascii_hexdigit()) {
            return Err("invalid sha256 output".into());
        }
        Ok(digest)
    }
    fn source_identity(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
        for entry in fs::read_dir(dir).map_err(io)? {
            let path = entry.map_err(io)?.path();
            if path.is_dir() {
                source_identity(&path, files)?;
            } else {
                files.push(path);
            }
        }
        Ok(())
    }
    fn source_hash(root: &Path) -> Result<String, String> {
        let mut paths = Vec::new();
        source_identity(&root.join("src"), &mut paths)?;
        paths.extend([root.join("Cargo.toml"), root.join("Cargo.lock")]);
        paths.sort();
        // One SHA over length-framed names/content rather than a hash process per file.
        let mut source_bytes = Vec::new();
        for p in paths {
            let name = p.strip_prefix(root).map_err(io)?.to_string_lossy();
            let bytes = fs::read(&p).map_err(io)?;
            source_bytes.extend((name.len() as u64).to_le_bytes());
            source_bytes.extend(name.as_bytes());
            source_bytes.extend((bytes.len() as u64).to_le_bytes());
            source_bytes.extend(bytes);
        }
        hash(&source_bytes)
    }
    fn parse() -> Result<Options, String> {
        let args: Vec<_> = std::env::args().skip(1).collect();
        let mut o = Options {
            output: PathBuf::new(),
            c0: PathBuf::new(),
            c1: PathBuf::new(),
            seconds: 55,
            n: 40,
            depths: vec![4, 5, 6, 7],
            seed: 960901,
            per_backend_ms: 1000,
            scalar: false,
        };
        let mut i = 0;
        while i < args.len() {
            let flag = &args[i];
            i += 1;
            if flag == "--scalar" {
                o.scalar = true;
                continue;
            }
            if flag == "--help" {
                return Err("usage: trace_union_bench --output DIR --c0 PATH --c1 PATH [--seconds 55] [--n 40] [--depths 4,5,6,7] [--seed 960901] [--per-backend-ms 1000] [--scalar]".into());
            }
            let value = args
                .get(i)
                .ok_or_else(|| format!("missing value for {flag}"))?;
            i += 1;
            match flag.as_str() {
                "--output" => o.output = value.into(),
                "--c0" => o.c0 = value.into(),
                "--c1" => o.c1 = value.into(),
                "--seconds" => o.seconds = value.parse().map_err(io)?,
                "--n" => o.n = value.parse().map_err(io)?,
                "--depths" => {
                    o.depths = value
                        .split(',')
                        .map(|v| v.parse::<usize>().map_err(io))
                        .collect::<Result<_, _>>()?
                }
                "--seed" => o.seed = value.parse().map_err(io)?,
                "--per-backend-ms" => o.per_backend_ms = value.parse().map_err(io)?,
                _ => return Err(format!("unknown argument {flag}")),
            }
        }
        if o.output.as_os_str().is_empty()
            || o.c0.as_os_str().is_empty()
            || o.c1.as_os_str().is_empty()
            || !(1..=64).contains(&o.n)
            || o.seconds == 0
            || o.seconds > 55
            || o.per_backend_ms == 0
            || o.per_backend_ms > 1000
            || o.depths.is_empty()
            || o.depths.iter().any(|d| !(4..=7).contains(d))
            || o.depths.windows(2).any(|w| w[0] >= w[1])
        {
            return Err("required paths; n1..64, seconds1..55, backend-ms1..1000, strictly ascending depths within4..7".into());
        }
        Ok(o)
    }
    fn load_family(path: &Path) -> Result<Family, String> {
        let family: Family = serde_json::from_slice(&fs::read(path).map_err(io)?).map_err(io)?;
        family.validate()?;
        Ok(family)
    }
    fn remaining(start: Instant, seconds: u64) -> Duration {
        Duration::from_secs(seconds).saturating_sub(start.elapsed())
    }
    fn allowance(start: Instant, o: &Options) -> Duration {
        remaining(start, o.seconds).min(Duration::from_millis(o.per_backend_ms))
    }
    fn field(p: &Problem, c0: &Family, c1: &Family) -> Result<CompiledField, String> {
        let f = CompiledField::partner_family(p.viewer, c0, c1)?;
        if f.revision() != p.field_revision {
            return Err("frozen field identity mismatch".into());
        }
        Ok(f)
    }

    // A declaring seat opens. One extra legal play, when needed, schedules the
    // other role without changing teams; only even seat rotations follow.
    // Never draw again because of payoff, branching or backend outcome.
    fn prepare(
        declaration: usize,
        depth: usize,
        viewer: u8,
        seed: u64,
        n: usize,
        c0: &Family,
        c1: &Family,
        duration: Duration,
    ) -> Result<Option<(Problem, Value)>, String> {
        let started = Instant::now();
        let mut budget = Budget::new(u64::MAX, duration);
        let decl = decl_of(declaration);
        let mut rng = SplitMix64(seed);
        let mut deck: Vec<u8> = (0..28).collect();
        for i in (1..28).rev() {
            let j = rng.below((i + 1) as u64) as usize;
            deck.swap(i, j);
        }
        let hands: [u32; 4] =
            std::array::from_fn(|s| deck[s * 7..s * 7 + 7].iter().fold(0, |m, &t| m | (1 << t)));
        let leader = 1u8;
        let mut public = PublicState::opening(leader);
        for _ in 0..4 * (7 - depth) {
            if budget.tick().is_none() {
                return Ok(None);
            }
            let legal = tiles(public.legal(decl, hands[public.actor() as usize]));
            public = public.after(decl, legal[rng.below(legal.len() as u64) as usize]);
        }
        let role_adjustment = public.actor() % 2 != viewer % 2;
        if role_adjustment {
            if budget.tick().is_none() {
                return Ok(None);
            }
            let legal = tiles(public.legal(decl, hands[public.actor() as usize]));
            public = public.after(decl, legal[rng.below(legal.len() as u64) as usize]);
        }
        let shift = (viewer + 4 - public.actor()) % 4;
        debug_assert_eq!(shift % 2, 0, "seat rotation must preserve declaring team");
        let mut rotated = PublicState::opening((leader + shift) % 4);
        for &tile in &public.history {
            rotated = rotated.after(decl, tile);
        }
        let hand = hands[((viewer + 4 - shift) % 4) as usize] & !rotated.played;
        let f = CompiledField::partner_family(viewer, c0, c1)?;
        let generation_us = micros(started);
        let sampling = Instant::now();
        let sample_seed = seed ^ 0x5452_4143_455f_554e;
        let Some(p) = model::sample_problem(
            decl,
            30,
            viewer,
            hand,
            &rotated,
            n,
            sample_seed,
            f.revision().to_owned(),
            &mut budget,
        )?
        else {
            return Ok(None);
        };
        let sampling_us = micros(sampling);
        Ok(Some((
            p,
            json!({"status":"complete","generation_us":generation_us,"sampling_us":sampling_us,
            "preparation_us":micros(started),"sampling_ticks":budget.used,"sample_seed":sample_seed,"seat_rotation":shift,
            "initial_leader":(leader+shift)%4,"role_adjustment_plays":u8::from(role_adjustment)}),
        )))
    }
    fn reprice(
        p: &Problem,
        policy: &Policy,
        expected: u64,
        c0: &Family,
        c1: &Family,
        budget: &mut Budget,
    ) -> Value {
        let started = Instant::now();
        let result = field(p, c0, c1).and_then(|mut f| price(p, policy, &mut f, budget));
        match result {
            Ok(Some(value)) => {
                json!({"status":if value==expected{"passed"}else{"mismatch"},"value":value,"expected":expected,"elapsed_us":micros(started)})
            }
            Ok(None) => json!({"status":"timeout","elapsed_us":micros(started)}),
            Err(error) => json!({"status":"error","error":error,"elapsed_us":micros(started)}),
        }
    }
    fn cpu(p: &Problem, c0: &Family, c1: &Family, duration: Duration) -> Value {
        let started = Instant::now();
        let mut budget = Budget::new(u64::MAX, duration);
        let result = field(p, c0, c1).and_then(|mut f| {
            core::solve(
                p,
                &mut f,
                &mut budget,
                &Config {
                    max_horizon: 7,
                    priority_plans: 1,
                    scenario_upper: true,
                    stop_when_certified: false,
                },
            )
        });
        let solve_us = micros(started);
        match result {
            Err(error) => json!({"status":"error","error":error,"solve_us":solve_us}),
            Ok(report) => {
                let complete = report.exact_vector && !budget.exhausted();
                let vector: Vec<_> = report
                    .actions
                    .iter()
                    .map(|a| (a.action, a.lower, a.upper))
                    .collect();
                let check = if complete {
                    reprice(
                        p,
                        &report.incumbent,
                        report.incumbent_value.unwrap(),
                        c0,
                        c1,
                        &mut budget,
                    )
                } else {
                    json!({"status":"not_exact"})
                };
                json!({"status":if complete{"complete"}else{"timeout"},"solve_us":solve_us,"exact_vector":report.exact_vector,
                    "bounds":vector,"action_values":if complete{Some(report.actions.iter().map(|a|(a.action,a.lower)).collect::<Vec<_>>())}else{None},
                    "chosen":report.chosen,"value":report.incumbent_value,"policy":report.incumbent,
                    "controller_ticks":report.work_used,"interrupted":report.interrupted,"reprice":check})
            }
        }
    }
    fn traces(
        p: &Problem,
        c0: &Family,
        c1: &Family,
        duration: Duration,
        gpu: Option<&mut EpochEvaluator>,
    ) -> Value {
        let started = Instant::now();
        let mut budget = Budget::new(u64::MAX, duration);
        let plans = match rollout::priority_plans(p, None) {
            Ok(v) => v,
            Err(e) => return json!({"status":"error","error":e}),
        };
        let policies: Vec<_> = plans
            .iter()
            .map(|order| Policy::priority(p.viewer, order.clone()))
            .collect();
        let plan_us = micros(started);
        let lanes = plans.len() * p.scenarios.len();
        let rollout_started = Instant::now();
        let mut f = match field(p, c0, c1) {
            Ok(f) => f,
            Err(e) => return json!({"status":"error","error":e}),
        };
        let (result, stats): (_, Option<Value>) = if let Some(gpu) = gpu {
            gpu.reset_stats();
            let rows = gpu.traces(p, &policies, &mut f, &mut budget).map(|rows| {
                rows.map(|rows| {
                    rows.into_iter()
                        .map(|r| r.into_iter().map(Trace::from).collect())
                        .collect::<Vec<Vec<Trace>>>()
                })
            });
            (rows, Some(json!(gpu.stats)))
        } else {
            let rows = rollout::evaluate_traces(p, &plans, &mut f, &mut budget).map(|rows| {
                Some(
                    rows.into_iter()
                        .map(|r| r.into_iter().map(Trace::from).collect())
                        .collect::<Vec<Vec<Trace>>>(),
                )
            });
            (rows, None)
        };
        let rollout_readback_us = micros(rollout_started);
        let mut out = json!({"plan_count":plans.len(),"lanes":lanes,"lane_state_bytes":lanes*160,
            "plan_preparation_us":plan_us,"rollout_readback_us":rollout_readback_us,"device":stats});
        let rows = match result {
            Ok(Some(rows)) => rows,
            Ok(None) => {
                out["status"] = json!("timeout_rollout");
                return out;
            }
            Err(error) => {
                out["status"] = json!(if budget.exhausted() {
                    "timeout_rollout"
                } else {
                    "error"
                });
                out["error"] = json!(error);
                return out;
            }
        };
        drop(plans);
        drop(policies);
        out["trace_tile_bytes"] = json!(rows
            .iter()
            .flatten()
            .map(|t| t.continuation.len())
            .sum::<usize>());
        out["completed_lanes"] = json!(rows.iter().map(Vec::len).sum::<usize>());
        let folding = Instant::now();
        let bundle = TraceBundle::new(p, rows);
        let folded = trace_union::fold(p, &bundle, &mut budget);
        drop(bundle); // Include retained trace cleanup in fold/whole-solve cost.
        let fold_extract_us = micros(folding);
        out["fold_extract_us"] = json!(fold_extract_us);
        out["solve_us"] = json!(micros(started));
        match folded {
            Err(error) => {
                out["status"] = json!("error");
                out["error"] = json!(error);
            }
            Ok(None) => {
                out["status"] = json!("timeout_fold");
            }
            Ok(Some(exact)) => {
                out["status"] = json!("complete");
                out["action_values"] = json!(exact.action_values);
                out["chosen"] = json!(exact.chosen);
                out["value"] = json!(exact.value);
                out["unique_nodes"] = json!(exact.unique_nodes);
                out["reprice"] = reprice(p, &exact.policy, exact.value, c0, c1, &mut budget);
                out["policy"] = json!(exact.policy);
            }
        }
        out["controller_ticks"] = json!(budget.used);
        out
    }
    fn parity(case: &Value, other: &str) -> Value {
        let cpu = &case["cpu"];
        let gpu = &case[other];
        if cpu["status"] != "complete" || gpu["status"] != "complete" {
            return json!({"status":"not_comparable","reason":"both exact backend results are required; CPU timeout is not parity"});
        }
        let vectors = cpu["action_values"] == gpu["action_values"];
        let choices = cpu["chosen"] == gpu["chosen"];
        let repriced = cpu["reprice"]["status"] == "passed" && gpu["reprice"]["status"] == "passed";
        let bad_policy = [cpu, gpu]
            .iter()
            .any(|v| v["reprice"]["status"] == "mismatch" || v["reprice"]["status"] == "error");
        json!({"status":if vectors&&choices&&repriced{"passed"}else if !vectors||!choices||bad_policy{"mismatch"}else{"vectors_equal_reprice_incomplete"},
            "vectors_equal":vectors,"choices_equal":choices,"both_policies_repriced":repriced})
    }
    fn checkpoint(out: &Path, total: usize, session: &str, start: Instant) -> Result<(), String> {
        let mut rows = Vec::new();
        let mut complete = 0;
        for index in 0..total {
            let path = out.join("fixtures").join(format!("{index:03}.json"));
            if path.exists() {
                let c = read(&path)?;
                if c["complete"] == true {
                    complete += 1;
                }
                rows.push(json!({"index":index,"complete":c["complete"],"preparation":c["preparation"]["status"],
                    "cpu":c["cpu"]["status"],"gpu":c["gpu"]["status"],"scalar":c["scalar"]["status"],"parity":c["parity"]}));
            }
        }
        atomic(
            &out.join("checkpoint.json"),
            &json!({"schema":SCHEMA,"session":session,"elapsed_us":micros(start),
            "fixture_count":total,"complete_fixtures":complete,"all_attempted":complete==total,"fixtures":rows}),
        )
    }
    pub fn run() -> Result<(), String> {
        let start = Instant::now();
        let mut o = parse()?;
        fs::create_dir_all(&o.output).map_err(io)?;
        o.output = fs::canonicalize(&o.output).map_err(io)?;
        o.c0 = fs::canonicalize(&o.c0).map_err(io)?;
        o.c1 = fs::canonicalize(&o.c1).map_err(io)?;
        fs::create_dir_all(o.output.join("fixtures")).map_err(io)?;
        fs::create_dir_all(o.output.join("sessions")).map_err(io)?;
        let c0 = load_family(&o.c0)?;
        let c1 = load_family(&o.c1)?;
        let executable = std::env::current_exe().map_err(io)?;
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let protocol = json!({"schema":SCHEMA,"options":o,"c0":c0,"c1":c1,
            "c0_sha256":hash(&fs::read(&o.c0).map_err(io)?)?,"c1_sha256":hash(&fs::read(&o.c1).map_err(io)?)?,
            "binary_sha256":hash(&fs::read(&executable).map_err(io)?)?,"source_sha256":source_hash(root)?,
            "schedule":"9 declarations x requested depths; role=(declaration_index+depth_index)%2; declaring seat1 opens; legal random prefix plus one legal play if role differs; even seat rotation only; no retries by outcome",
            "bid":30,"cpu_config":{"max_horizon":7,"priority_plans":1,"scenario_upper":true,"stop_when_certified":false},
            "scope":"Exact fixed-bundle compute control. Settled and refused fixtures retained. No model-quality or phone-performance claim."});
        let protocol_path = o.output.join("protocol.json");
        if protocol_path.exists() {
            if read(&protocol_path)? != protocol {
                return Err("resume protocol/artifact identity mismatch".into());
            }
        } else {
            atomic(&protocol_path, &protocol)?;
        }
        let session = format!(
            "{}-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(io)?
                .as_millis(),
            std::process::id()
        );
        let session_path = o.output.join("sessions").join(format!("{session}.json"));
        let mut session_info = json!({"session":session,"status":"running","schema":SCHEMA,"cycle_seconds":o.seconds,
            "device_initialization":null,"gpu_batches":0});
        atomic(&session_path, &session_info)?;
        let mut gpu: Option<EpochEvaluator> = None;
        let mut gpu_init_error: Option<String> = None;
        let mut gpu_batches = 0;
        let declarations = [0, 1, 2, 3, 4, 5, 6, 7, 9];
        let total = declarations.len() * o.depths.len();
        for index in 0..total {
            if remaining(start, o.seconds).is_zero() {
                break;
            }
            let di = index / o.depths.len();
            let d = index % o.depths.len();
            let declaration = declarations[di];
            let depth = o.depths[d];
            let viewer = ((di + d) % 2) as u8;
            let seed = o
                .seed
                .wrapping_add((index as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15));
            let path = o.output.join("fixtures").join(format!("{index:03}.json"));
            let mut case = if path.exists() {
                read(&path)?
            } else {
                json!({"schema":SCHEMA,"index":index,"declaration":declaration,
                "depth":depth,"viewer":viewer,"role":if viewer==1{"declaring"}else{"defending"},"seed":seed,
                "complete":false,"problem":null,"preparation":null,"cpu":null,"gpu":null,"scalar":null})
            };
            if case["complete"] == true {
                continue;
            }
            // Never silently rerun an attempt killed after its durable start marker.
            for key in ["preparation", "cpu", "gpu", "scalar"] {
                if case[key]["status"] == "running" {
                    case[key] = json!({"status":"interrupted_external","retry":false});
                }
            }
            if case["preparation"].is_null() {
                case["preparation"] = json!({"status":"running","session":session});
                atomic(&path, &case)?;
                let phase = Instant::now();
                match prepare(
                    declaration,
                    depth,
                    viewer,
                    seed,
                    o.n,
                    &c0,
                    &c1,
                    allowance(start, &o),
                ) {
                    Ok(Some((p, prep))) => {
                        let wire = ProblemWire::from(&p, declaration);
                        let bytes = serde_json::to_vec(&wire).map_err(io)?;
                        case["problem_sha256"] = json!(hash(&bytes)?);
                        case["problem"] = json!(wire);
                        case["settled_at_root"] = json!(p.root.payoff(p.bid, p.viewer).is_some());
                        case["preparation"] = prep;
                    }
                    Ok(None) => case["preparation"] = json!({"status":"timeout"}),
                    Err(error) => case["preparation"] = json!({"status":"error","error":error}),
                }
                case["preparation"]["elapsed_including_identity_us"] = json!(micros(phase));
                atomic(&path, &case)?;
            }
            if case["preparation"]["status"] != "complete" {
                case["complete"] = json!(true);
                atomic(&path, &case)?;
                checkpoint(&o.output, total, &session, start)?;
                continue;
            }
            let wire: ProblemWire = serde_json::from_value(case["problem"].clone()).map_err(io)?;
            if json!(hash(&serde_json::to_vec(&wire).map_err(io)?)?) != case["problem_sha256"] {
                return Err("saved immutable problem digest mismatch".into());
            }
            let p = wire.problem();
            p.validate()?;
            let order = if index % 2 == 0 {
                ["cpu", "gpu", "scalar"]
            } else {
                ["gpu", "cpu", "scalar"]
            };
            for backend in order {
                if backend == "scalar" && !o.scalar {
                    continue;
                }
                if !case[backend].is_null() {
                    continue;
                }
                if remaining(start, o.seconds).is_zero() {
                    break;
                }
                if backend == "gpu" && gpu.is_none() && gpu_init_error.is_none() {
                    let cold = Instant::now();
                    match EpochEvaluator::new() {
                        Ok(device) => {
                            session_info["device_initialization"] = json!({"status":"complete","elapsed_us":micros(cold),
                            "reported_initialization_ms":device.initialization_ms,"adapter":device.adapter_name()});
                            gpu = Some(device);
                        }
                        Err(error) => {
                            session_info["device_initialization"] =
                                json!({"status":"error","error":error,"elapsed_us":micros(cold)});
                            gpu_init_error = Some(error);
                        }
                    }
                    atomic(&session_path, &session_info)?;
                }
                if remaining(start, o.seconds).is_zero() {
                    break;
                }
                let cap = allowance(start, &o);
                case[backend] =
                    json!({"status":"running","session":session,"cap_us":cap.as_micros()});
                atomic(&path, &case)?;
                let elapsed = Instant::now();
                let mut result = match backend {
                    "cpu" => cpu(&p, &c0, &c1, cap),
                    "gpu" => {
                        if let Some(device) = gpu.as_mut() {
                            gpu_batches += 1;
                            traces(&p, &c0, &c1, cap, Some(device))
                        } else {
                            json!({"status":"device_refusal","error":gpu_init_error})
                        }
                    }
                    _ => traces(&p, &c0, &c1, cap, None),
                };
                result["backend_elapsed_us"] = json!(micros(elapsed));
                if result["solve_us"].is_null() {
                    result["solve_us"] = result["backend_elapsed_us"].clone();
                }
                result["cap_us"] = json!(cap.as_micros());
                result["session"] = json!(session);
                if backend == "gpu" {
                    result["batch_index_in_session"] = json!(gpu_batches);
                }
                case[backend] = result;
                case["parity"] = parity(&case, "gpu");
                if o.scalar {
                    case["scalar_parity"] = parity(&case, "scalar");
                }
                atomic(&path, &case)?;
                checkpoint(&o.output, total, &session, start)?;
            }
            case["complete"] = json!(
                !case["cpu"].is_null()
                    && !case["gpu"].is_null()
                    && (!o.scalar || !case["scalar"].is_null())
            );
            case["parity"] = parity(&case, "gpu");
            if o.scalar {
                case["scalar_parity"] = parity(&case, "scalar");
            }
            atomic(&path, &case)?;
            checkpoint(&o.output, total, &session, start)?;
        }
        // Recheck after all work, before declaring the cycle valid. Preserve
        // a durable failed verification even if a pinned file disappeared.
        let end_identity = (|| -> Result<Value, String> {
            Ok(json!({"c0_sha256":hash(&fs::read(&o.c0).map_err(io)?)?,
                "c1_sha256":hash(&fs::read(&o.c1).map_err(io)?)?,
                "source_sha256":source_hash(root)?,
                "binary_sha256":hash(&fs::read(&executable).map_err(io)?)?}))
        })();
        let identity_stable = match end_identity {
            Ok(after) => {
                let changed: Vec<_> = ["c0_sha256", "c1_sha256", "source_sha256", "binary_sha256"]
                    .into_iter()
                    .filter(|key| after[*key] != protocol[*key])
                    .collect();
                let stable = changed.is_empty();
                session_info["end_identity_verification"] = json!({"status":if stable{"passed"}else{"changed"},"changed":changed,"actual":after});
                stable
            }
            Err(error) => {
                session_info["end_identity_verification"] = json!({"status":"error","error":error});
                false
            }
        };
        session_info["status"] = json!(if identity_stable {
            "cycle_finished"
        } else {
            "identity_verification_failed"
        });
        session_info["elapsed_us"] = json!(micros(start));
        session_info["gpu_batches"] = json!(gpu_batches);
        atomic(&session_path, &session_info)?;
        checkpoint(&o.output, total, &session, start)?;
        let mut final_checkpoint = read(&o.output.join("checkpoint.json"))?;
        final_checkpoint["last_session_identity_verified"] = json!(identity_stable);
        atomic(&o.output.join("checkpoint.json"), &final_checkpoint)?;
        if !identity_stable {
            return Err("actor/source/binary identity changed or became unreadable during cycle; receipts retained and cycle flagged".into());
        }
        println!(
            "{}",
            json!({"session":session,"output":o.output,"checkpoint":read(&o.output.join("checkpoint.json"))?})
        );
        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use walt_response_ladder::compiled::Actor;

        #[test]
        fn prepare_preserves_declaring_opener_role_depth_and_public_frame() {
            let family = Family::Single(Actor::empty());
            for declaration in [6, 9] {
                for depth in [4, 7] {
                    for viewer in [0, 1] {
                        let seed = 960901
                            + declaration as u64 * 100
                            + depth as u64 * 10
                            + u64::from(viewer);
                        let (p, receipt) = prepare(
                            declaration,
                            depth,
                            viewer,
                            seed,
                            2,
                            &family,
                            &family,
                            Duration::from_secs(3),
                        )
                        .unwrap()
                        .unwrap();
                        let initial = receipt["initial_leader"].as_u64().unwrap() as u8;
                        assert_eq!(initial % 2, 1);
                        assert_eq!(receipt["seat_rotation"].as_u64().unwrap() % 2, 0);
                        assert_eq!(p.viewer, viewer);
                        assert_eq!(p.root.actor(), viewer);
                        assert_eq!(p.hand.count_ones() as usize, depth);
                        assert_eq!(p.root.history.len(), 4 * (7 - depth) + p.root.plays.len());
                        assert!(p.root.plays.len() <= 1);
                        if depth == 7 {
                            assert_eq!(p.root.plays.len(), usize::from(viewer == 0));
                        }
                        let (_, size, sizes) = model::frame(&p.root).unwrap();
                        assert_eq!(size, depth);
                        assert_eq!(sizes[viewer as usize], depth);
                        let mut replay = PublicState::opening(initial);
                        for &tile in &p.root.history {
                            replay = replay.after(p.decl, tile);
                        }
                        assert_eq!(replay, p.root);
                        p.validate().unwrap();
                    }
                }
            }
        }
    }
}
#[cfg(feature = "gpu")]
fn main() {
    if let Err(error) = app::run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
