#!/usr/bin/env python3
"""Replay 12 completed fixed-search pairs from caller-supplied matched builds."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import resource
import subprocess
import time

from compare_panel import run as compare

ROOT = Path(__file__).resolve().parents[2]


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def sources():
    paths = list((ROOT / "walt/walt/src/solver").rglob("*"))
    paths += list((ROOT / "walt/walt-cpu-bench").rglob("*.rs"))
    paths += [ROOT / path for path in (
        "walt/Cargo.lock", "walt/walt/Cargo.toml", "walt/walt-cpu-bench/Cargo.toml"
    )]
    return {str(path.relative_to(ROOT)): sha(path)
            for path in sorted(paths) if path.is_file()}


def write_json(path, value):
    with path.open("x") as output:
        json.dump(value, output, indent=2)
        output.write("\n")


def fixtures():
    yield "g1", ["--fixture", "g1"]
    for seed in range(420601, 420609):
        yield f"shuffle-{seed}", ["--fixture", "shuffle", "--deal-seed", str(seed)]
    for declaration in (0, 7, 9):
        yield f"decl-{declaration}", [
            "--fixture", "shuffle", "--deal-seed", "420609", "--decl", str(declaration)
        ]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--baseline", type=Path, required=True)
    parser.add_argument("--optimized", type=Path, required=True)
    parser.add_argument("--baseline-revision", required=True)
    parser.add_argument("--output-dir", type=Path, required=True)
    parser.add_argument("--threads", type=int, default=18)
    args = parser.parse_args()
    baseline, optimized = args.baseline.resolve(), args.optimized.resolve()
    if not baseline.is_file() or not optimized.is_file() or args.threads <= 0:
        parser.error("both executables and a positive thread count are required")
    output = args.output_dir.resolve()
    output.mkdir(parents=True, exist_ok=False)
    before_sources = sources()
    binary_hashes = {"baseline": sha(baseline), "optimized": sha(optimized)}
    pairs = []
    for index, (name, fixture_args) in enumerate(fixtures()):
        paths = {label: output / f"{name}-{label}.json"
                 for label in ("baseline", "optimized")}
        order = [("baseline", baseline), ("optimized", optimized)]
        if index % 2:
            order.reverse()
        for label, binary in order:
            path = paths[label]
            command = [str(binary), *fixture_args, "--repeats", "1", "--output", str(path)]
            before = resource.getrusage(resource.RUSAGE_CHILDREN)
            started = time.perf_counter()
            with path.with_suffix(".progress.jsonl").open("x") as progress:
                subprocess.run(
                    command, stdout=subprocess.DEVNULL, stderr=progress,
                    env={**os.environ, "RAYON_NUM_THREADS": str(args.threads)},
                    check=True, timeout=285,
                )
            after = resource.getrusage(resource.RUSAGE_CHILDREN)
            write_json(path.with_name(path.stem + "-usage.json"), {
                "command": command,
                "rayon_num_threads": args.threads,
                "process_wall_s": time.perf_counter() - started,
                "user_cpu_s": after.ru_utime - before.ru_utime,
                "system_cpu_s": after.ru_stime - before.ru_stime,
            })
        pairs.append((paths["baseline"], paths["optimized"]))
        row = compare([pairs[-1]])["games"][0]
        print(json.dumps({
            "fixture": name, "exact_equal": row["exact_values_equal"],
            "baseline_s": row["reference_full_game_us"] / 1e6,
            "optimized_s": row["candidate_full_game_us"] / 1e6,
            "speedup": row["speedup"],
        }), flush=True)
    if sources() != before_sources:
        raise RuntimeError("sources changed during comparison")
    if binary_hashes != {"baseline": sha(baseline), "optimized": sha(optimized)}:
        raise RuntimeError("executables changed during comparison")
    result = compare(pairs)
    write_json(output / "comparison.json", result)
    write_json(output / "builds.json", {
        "baseline_head": args.baseline_revision,
        "baseline_binary_sha256": binary_hashes["baseline"],
        "optimized_binary_sha256": binary_hashes["optimized"],
        "compiler": subprocess.check_output(["rustc", "--version"], text=True).strip(),
        "build_settings_basis": "Caller must supply identically built binaries; flags are not inferred from executables.",
        "required_build_settings": {
            "rustflags": "-C target-cpu=native", "lto": "thin",
            "codegen_units": 1, "overflow_checks": True,
        },
        "rayon_num_threads": args.threads, "qos": "default",
        "comparison_scope": "completed fixed L2 Partner search; wall-limited wrapper stages may differ",
        "optimized_sources_sha256": before_sources,
        "source_identity_basis": "Current checkout snapshot; establish its executable binding when building.",
    })
    print(json.dumps(result["summary"]), flush=True)


if __name__ == "__main__":
    main()
