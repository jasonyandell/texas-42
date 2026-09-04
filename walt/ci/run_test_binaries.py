#!/usr/bin/env python3
"""Run the workspace's test binaries concurrently (the walt gate's test stage).

Reads cargo's `--message-format=json` stream from `cargo test --no-run` on
stdin, collects every test executable it built, and runs them all with a
bounded worker pool. Cargo itself runs test binaries one after another;
this repository's gate suites are dominated by a handful of long exact
recursions, so serial execution was the whole wall of `check.sh`.

Behaviour is otherwise `cargo test`'s: each binary runs with its package
directory as cwd and `CARGO_MANIFEST_DIR` set; its output is captured and
printed IN FULL whenever it fails; every binary runs even after a failure
so the report is complete; the exit status is 1 if any binary failed.
Nothing about which assertions run changes — this is a scheduler.

stdlib only, run under `python3 -I -B` like the other gate scripts.
Usage: cargo test ... --no-run --message-format=json | run_test_binaries.py TARGET_DIR
"""
import json
import os
import subprocess
import sys
import time
from concurrent.futures import ThreadPoolExecutor

# Suites known to run long; scheduled first so the makespan approaches
# the longest single suite. Purely an ordering hint — every suite runs.
HEAVY_FIRST = (
    "solver_focal_horizon",
    "solver_factor_refine",
    "solver_horizon",
    "solver_unified_carry",
    "solver_focal_budget",
    "solver_focal_anchors",
    "strat_exp5_census",
    "solver_model_belief_recursion",
    "solver_unified",
)


def collect(stream):
    seen = {}
    for line in stream:
        line = line.strip()
        if not line.startswith("{"):
            continue
        msg = json.loads(line)
        if msg.get("reason") != "compiler-artifact":
            continue
        exe = msg.get("executable")
        if not exe or not msg.get("profile", {}).get("test"):
            continue
        target = msg["target"]
        # package_id is "<source>#<name>@<version>" or, when the package is
        # named after its directory, "<source>/<name>#<version>".
        fragment = msg["package_id"].rsplit("#", 1)
        package = fragment[1].split("@")[0] if "@" in fragment[1] else fragment[0].rstrip("/").rsplit("/", 1)[-1]
        label = f"{package}::{target['name']}"
        seen[exe] = (label, os.path.dirname(msg["manifest_path"]))
    return seen


def priority(item):
    exe, (label, _) = item
    name = label.split("::")[-1]
    for i, heavy in enumerate(HEAVY_FIRST):
        if name == heavy:
            return (0, i, label)
    return (1, 0, label)


def run_one(exe, label, cwd, target_dir):
    env = dict(os.environ)
    env["CARGO_MANIFEST_DIR"] = cwd
    env["CARGO_TARGET_TMPDIR"] = os.path.join(target_dir, "tmp")
    started = time.monotonic()
    proc = subprocess.run(
        [exe],
        cwd=cwd,
        env=env,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        errors="replace",
        check=False,
    )
    return label, proc.returncode, time.monotonic() - started, proc.stdout


def main():
    if len(sys.argv) != 2:
        print("usage: run_test_binaries.py TARGET_DIR", file=sys.stderr)
        return 2
    target_dir = sys.argv[1]
    os.makedirs(os.path.join(target_dir, "tmp"), exist_ok=True)
    binaries = sorted(collect(sys.stdin).items(), key=priority)
    if not binaries:
        print("run_test_binaries.py: ERROR: no test executables in the cargo stream", file=sys.stderr)
        return 1
    workers = max(2, (os.cpu_count() or 4) // 2)
    print(f"== running {len(binaries)} test binaries, {workers} at a time", flush=True)
    results = []
    failures = []
    with ThreadPoolExecutor(max_workers=workers) as pool:
        futures = [pool.submit(run_one, exe, label, cwd, target_dir) for exe, (label, cwd) in binaries]
        for fut in futures:
            label, code, secs, out = fut.result()
            results.append((secs, label, code))
            if code == 0:
                print(f"PASS {secs:7.1f}s  {label}", flush=True)
            else:
                failures.append(label)
                print(f"FAIL {secs:7.1f}s  {label} (exit {code}) — full output follows", flush=True)
                print(out, flush=True)
                print(f"---- end of output: {label}", flush=True)
    results.sort(reverse=True)
    print("== slowest suites")
    for secs, label, _ in results[:8]:
        print(f"  {secs:7.1f}s  {label}")
    total = sum(r[0] for r in results)
    print(f"== {len(results)} binaries, {len(failures)} failed, sum of suite walls {total:.0f}s")
    if failures:
        print("FAILED: " + ", ".join(failures), file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
