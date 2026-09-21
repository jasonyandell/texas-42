#!/usr/bin/env python3
"""Run frozen confirmation panels in bounded cycles from one immutable manifest.

Only coverage and technical status are inspected. A child failure is retained
and prevents continuation; outcomes never control the schedule.
"""
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import shlex
import subprocess
import sys

import h2h_compiled as h


def commands(path: Path):
    path, spec = h.load_confirmation_protocol(str(path))
    outputs = spec.get("output_directories")
    if not isinstance(outputs, dict) or set(outputs) != set(spec["panels"]):
        raise ValueError("manifest must specify exactly one output directory per panel")
    if len(set(outputs.values())) != len(outputs):
        raise ValueError("panel output directories must be distinct")
    config_path = path.parent / "config.json"
    if json.loads(config_path.read_text()) != spec["candidate_config"]:
        raise ValueError("manifest/config mismatch")
    candidate = spec["identities"]["candidate_command"]["argv"]
    result = []
    for panel in spec["panels"]:
        output = Path(outputs[panel]).resolve()
        command = [sys.executable, str(h.HERE / "tools/h2h_compiled.py"),
                   "--output", str(output), "--candidate-command", shlex.join(candidate),
                   "--cpu-root", spec["identities"]["cpu_root"], "--panel", panel,
                   "--deals-per-cell", str(spec["deals_per_cell"]), "--seconds", "55",
                   "--candidate-ms", str(spec["candidate_ms"]), "--threads", str(spec["threads"]),
                   "--config-json", str(config_path), "--confirmation-protocol", str(path)]
        result.append((panel, output, command))
    return path, spec, result


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("protocol", type=Path)
    parser.add_argument("--validate-only", action="store_true")
    args = parser.parse_args(argv)
    path, spec, schedule = commands(args.protocol)
    frozen_digest = h.digest(path)
    watchdog = h.HERE.parent / "partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py"
    expected = 2 * len(h.DECLARATIONS) * 4 * spec["deals_per_cell"]
    for panel, output, command in schedule:
        if args.validate_only:
            print(h.canonical({"panel": panel, "command": command}), flush=True)
            continue
        prior = sorted(path.parent.glob(f"{panel}-cycle-*/run.json"))
        for record in prior:
            value = json.loads(record.read_text())
            if value.get("status") != "completed" or value.get("child_returncode") != 0:
                raise RuntimeError(f"refusing continuation after failed invocation: {record}")
        cycle = len(prior)
        while True:
            if h.digest(path) != frozen_digest:
                raise RuntimeError("confirmation manifest changed during run")
            progress_path = output / "progress.json"
            progress = json.loads(progress_path.read_text()) if progress_path.exists() else {}
            before = progress.get("completed_arms", 0)
            if progress.get("failed_arms", 0) or progress.get("stop_reason", "budget") not in ("budget", "exhausted"):
                raise RuntimeError(f"technical stop in {panel}: {progress}")
            if before == expected:
                print(h.canonical({"panel": panel, "complete": True, "games": before}), flush=True)
                break
            if before > expected:
                raise RuntimeError("completed count exceeds frozen plan")
            cycle += 1
            receipt_dir = path.parent / f"{panel}-cycle-{cycle:03d}"
            if receipt_dir.exists():
                raise RuntimeError(f"refusing to overwrite cycle: {receipt_dir}")
            env = dict(os.environ)
            env["RUSTFLAGS"] = "-C target-cpu=native"
            result = subprocess.run([sys.executable, str(watchdog), "--seconds", "60",
                                     "--output-dir", str(receipt_dir), "--", *command],
                                    cwd=h.HERE, env=env)
            receipt = json.loads((receipt_dir / "run.json").read_text())
            if result.returncode or receipt.get("status") != "completed" or receipt.get("child_returncode") != 0:
                raise RuntimeError(f"preserved failed invocation: {receipt_dir}")
            progress = json.loads(progress_path.read_text())
            cycle_receipt = json.loads((output / "cycle-receipt.json").read_text())
            if (progress["failed_arms"] or progress["stop_reason"] not in ("budget", "exhausted")
                    or cycle_receipt.get("identity_drift") is not False):
                raise RuntimeError(f"preserved technical failure: {receipt_dir}")
            # A healthy cycle can pause within one long game. Preserve its
            # checkpoint and continue; no outcome is read or retried.
            print(h.canonical({"panel": panel, "cycle": cycle, **progress}), flush=True)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
