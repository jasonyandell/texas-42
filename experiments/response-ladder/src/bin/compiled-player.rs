//! Persistent compiled lower-rung player, for pinned exploratory comparisons.
use serde_json::json;
use std::io::{self, BufRead, Write};
use walt_response_ladder::{compiled_family::{Family, FAMILY_SCHEMA}, compiled_player};

fn family(path: &str) -> Result<Family, String> {
    let value: Family = serde_json::from_slice(&std::fs::read(path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    value.validate()?;
    Ok(value)
}

fn family_identity(value: &Family) -> Result<serde_json::Value, String> {
    let (declaring, defending) = value.roles()?;
    Ok(json!({"schema": FAMILY_SCHEMA, "declaring": declaring, "defending": defending}))
}

fn run() -> Result<(), String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.is_empty() || args[0] == "--help" {
        println!("usage: compiled-player worker --c0 PATH --c1 PATH [--gpu]");
        return Ok(());
    }
    if args[0] != "worker" { return Err("expected worker command".into()); }
    let mut c0 = None;
    let mut c1 = None;
    let mut use_gpu = false;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--c0" | "--c1" if i + 1 < args.len() => {
                let value = family(&args[i + 1])?;
                if args[i] == "--c0" { c0 = Some(value); } else { c1 = Some(value); }
                i += 2;
            }
            "--gpu" => { use_gpu = true; i += 1; }
            other => return Err(format!("unknown or incomplete argument {other}")),
        }
    }
    let c0 = c0.ok_or("missing --c0")?;
    let c1 = c1.ok_or("missing --c1")?;
    #[cfg(feature = "gpu")]
    let mut gpu = if use_gpu { Some(walt_response_ladder::gpu_epochs::EpochEvaluator::new()?) } else { None };
    #[cfg(not(feature = "gpu"))]
    if use_gpu { return Err("binary needs gpu feature for --gpu".into()); }
    let stdout = io::stdout();
    let mut out = stdout.lock();
    writeln!(out, "{}", json!({"ready":true, "schema":"compiled-player-v1",
        "backend":if use_gpu {"compiled-gpu"} else {"compiled-cpu"},
        "c0":family_identity(&c0)?, "c1":family_identity(&c1)?}))
        .map_err(|e| e.to_string())?;
    out.flush().map_err(|e| e.to_string())?;
    for line in io::stdin().lock().lines() {
        let line = line.map_err(|e| e.to_string())?;
        let response = match serde_json::from_str::<compiled_player::Request>(&line) {
            Err(error) => json!({"error":error.to_string()}),
            Ok(req) => {
                #[cfg(feature = "gpu")]
                let result = {
                    if let Some(ref mut backend) = gpu { backend.reset_stats(); }
                    compiled_player::decide_family(&req, &c0, &c1,
                        gpu.as_mut().map(|g| g as &mut dyn walt_response_ladder::core::PolicyEvaluator))
                };
                #[cfg(not(feature = "gpu"))]
                let result = compiled_player::decide_family(&req, &c0, &c1, None);
                match result {
                    Err(error) => json!({"error":error}),
                    Ok(result) => {
                        #[allow(unused_mut)]
                        let mut value = serde_json::to_value(result).map_err(|e| e.to_string())?;
                        #[cfg(feature = "gpu")]
                        if let Some(ref backend) = gpu {
                            value["compute"] = json!({"backend":"compiled-gpu", "stats":backend.stats});
                        }
                        value
                    }
                }
            }
        };
        writeln!(out, "{response}").map_err(|e| e.to_string())?;
        out.flush().map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() { eprintln!("{error}"); std::process::exit(1); }
}
