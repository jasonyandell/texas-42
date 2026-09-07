#!/usr/bin/env python3
"""Evaluate synthesized policies on a published exact partnership gym."""

import argparse
import concurrent.futures
from fractions import Fraction
import hashlib
import json
import math
import os
from pathlib import Path
import signal
import subprocess
import sys
import threading
import time

import gym


MAX_WORKERS = 10
MAX_SECONDS = 290.0
SAMPLES = [1, 4, 16, 64]


def file_digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def request_text(request):
    request = gym.pupil_request(request)
    return "".join(
        key + " " + " ".join(map(str, value if isinstance(value, list) else [value])) + "\n"
        for key, value in sorted(request.items())
    )


def atomic_text(path, value):
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_suffix(path.suffix + ".tmp")
    with temporary.open("w") as out:
        out.write(value); out.flush(); os.fsync(out.fileno())
    os.replace(temporary, path)
    descriptor = os.open(path.parent, os.O_RDONLY)
    try: os.fsync(descriptor)
    finally: os.close(descriptor)


def case_identity(name, case):
    text = request_text(case["request"])
    return {
        "name": name,
        "request_sha256": hashlib.sha256(text.encode()).hexdigest(),
        "key_sha256": gym.digest(case["key"]),
    }


def manifest(gallery_path, binary, runner):
    cases = list(gym.gallery(gallery_path))
    if not cases:
        raise ValueError("gallery is empty")
    return {
        "schema": "texas42-policy-gym-campaign-v1",
        "gallery": str(gallery_path.resolve()),
        "binary": {"path": str(binary.resolve()), "sha256": file_digest(binary)},
        "runner_sha256": file_digest(runner),
        "samples": SAMPLES,
        "test_worlds": 128,
        "node_budget": 2_000_000,
        "field": "gym",
        "exact_test": True,
        "cases": [case_identity(name, case) for name, case in cases],
    }


def fraction_text(numerator, denominator):
    return str(Fraction(numerator, denominator))


def assess_case(name, case, native):
    if native.get("schema") != "policy-lab-v1":
        raise ValueError(name + ": wrong native schema")
    request = case["request"]
    if native.get("seed") != request["seed"] or native.get("decl") != request["decl"] \
            or native.get("hand") != sorted(request["hand"]):
        raise ValueError(name + ": native root identity differs from request")
    flat_history = [value for pair in native.get("root_history", []) for value in pair]
    if flat_history != request["plays"]:
        raise ValueError(name + ": native public history differs from request")
    rows = native.get("rows")
    expected = {(n, arm) for n in SAMPLES for arm in ("fresh", "persistent", "compose")}
    if not isinstance(rows, list) or {
        (row.get("samples"), row.get("arm")) for row in rows if isinstance(row, dict)
    } != expected or len(rows) != len(expected):
        raise ValueError(name + ": incomplete or duplicate native rows")
    key = case["key"]
    if native.get("field") != key["field_id"] or native.get("root_worlds") != key["worlds"]:
        raise ValueError(name + ": native exact root or frozen field differs from teacher")
    action_makes = {row["tile"]: row["success_mass"] for row in key["actions"]}
    best = max(action_makes.values())
    worlds = key["worlds"]
    assessed = []
    for row in rows:
        action = row.get("root_action")
        exact_makes, exact_worlds = row.get("exact_policy_makes"), row.get("exact_policy_worlds")
        if row.get("status") not in ("completed", "budget") \
                or not isinstance(row.get("policy_id"), str) \
                or not isinstance(row.get("nodes"), int) or row["nodes"] < 0:
            raise ValueError(name + ": malformed policy row")
        if action not in action_makes:
            raise ValueError(name + ": policy chose an illegal root action")
        if not isinstance(exact_makes, int) or not isinstance(exact_worlds, int) \
                or exact_worlds != worlds or not 0 <= exact_makes <= exact_worlds:
            raise ValueError(name + ": invalid exact full-policy evaluation")
        if exact_makes > best:
            raise ValueError(name + ": policy exceeds independently exact best root value")
        assessed.append({
            "samples": row["samples"],
            "arm": row["arm"],
            "status": row.get("status"),
            "policy_id": row.get("policy_id"),
            "root_action": action,
            "root_action_optimal": action in key["best"],
            "root_action_regret": fraction_text(best - action_makes[action], worlds),
            "root_action_makes": action_makes[action],
            "best_root_q_makes": best,
            "exact_policy_makes": exact_makes,
            "exact_policy_worlds": exact_worlds,
            "full_policy_gap_to_best_root_q": fraction_text(best - exact_makes, worlds),
            "nodes": row.get("nodes"),
        })
    return {
        "schema": "texas42-policy-gym-case-v1",
        "case": name,
        "request_sha256": hashlib.sha256(request_text(case["request"]).encode()).hexdigest(),
        "key_sha256": gym.digest(key),
        "teacher": {"worlds": worlds, "best": key["best"], "best_root_q_makes": best},
        "rows": assessed,
        "native": native,
    }


class Processes:
    def __init__(self):
        self.lock, self.items = threading.Lock(), set()
    def add(self, proc):
        with self.lock: self.items.add(proc)
    def remove(self, proc):
        with self.lock: self.items.discard(proc)
    def kill(self):
        with self.lock: items = list(self.items)
        for proc in items:
            try: os.killpg(proc.pid, signal.SIGKILL)
            except ProcessLookupError: pass


def run_case(output, spec, name, case, timeout, stop, processes):
    if stop.is_set():
        return None
    case_dir = output / "cases" / name
    case_dir.mkdir(parents=True, exist_ok=True)
    request_path = case_dir / "request.txt"
    text = request_text(case["request"])
    if request_path.exists() and request_path.read_text() != text:
        # The manifest and live gallery both pin this derived request. Repair a
        # partial pre-spawn write; completed results are independently checked.
        request_path.unlink()
    if not request_path.exists():
        atomic_text(request_path, text)
    artifacts = case_dir / "artifacts"
    command = [spec["binary"]["path"], "--request", str(request_path),
        "--seed", str(case["request"]["seed"]), "--samples", ",".join(map(str, SAMPLES)),
        "--test-worlds", "128", "--node-budget", "2000000", "--field", "gym",
        "--exact-test", "1", "--artifact-dir", str(artifacts)]
    proc = subprocess.Popen(command, stdin=subprocess.DEVNULL, stdout=subprocess.PIPE,
                            stderr=subprocess.PIPE, text=True, start_new_session=True)
    processes.add(proc)
    timed_out = False
    try:
        try:
            stdout, stderr = proc.communicate(timeout=timeout)
        except subprocess.TimeoutExpired:
            timed_out = True
            os.killpg(proc.pid, signal.SIGKILL)
            stdout, stderr = proc.communicate()
    finally:
        processes.remove(proc)
    gym.atomic(case_dir / "attempt.json", {"command": command, "returncode": proc.returncode,
                                           "timed_out": timed_out,
                                           "stdout": stdout, "stderr": stderr})
    if timed_out:
        raise TimeoutError(name + " timed out")
    if proc.returncode:
        raise ValueError(name + ": native failure: " + stderr.strip()[-500:])
    try:
        native = json.loads(stdout)
    except json.JSONDecodeError as error:
        raise ValueError(name + ": invalid native JSON") from error
    result = assess_case(name, case, native)
    gym.atomic(output / "results" / (name + ".json"), result)
    return result


def validate_saved(output, spec, catalog):
    complete = {}
    identities = {item["name"]: item for item in spec["cases"]}
    for name, case in catalog:
        path = output / "results" / (name + ".json")
        if path.exists():
            saved = json.loads(path.read_text())
            expected = assess_case(name, case, saved.get("native", {}))
            if saved != expected or (saved["request_sha256"], saved["key_sha256"]) != (
                    identities[name]["request_sha256"], identities[name]["key_sha256"]):
                raise ValueError(name + ": corrupt completed result")
            complete[name] = saved
    return complete


def write_status(output, spec, catalog, failures=None, active=None):
    complete = validate_saved(output, spec, catalog)
    rows = [row for result in complete.values() for row in result["rows"]]
    status = {
        "schema": "texas42-policy-gym-status-v1",
        "completed_cases": len(complete), "target_cases": len(catalog),
        "pending_cases": [name for name, _ in catalog if name not in complete],
        "active_cases": sorted(active or []), "failures": failures or {},
        "root_action_optimal": sum(row["root_action_optimal"] for row in rows),
        "evaluated_rows": len(rows),
        "exact_policy_makes": sum(row["exact_policy_makes"] for row in rows),
        "exact_policy_worlds": sum(row["exact_policy_worlds"] for row in rows),
    }
    gym.atomic(output / "status.json", status)
    return status


def execute(args):
    output, binary, gallery_path = args.output.resolve(), args.binary.resolve(), args.gallery.resolve()
    if not binary.is_file() or not os.access(binary, os.X_OK):
        raise ValueError("policy_lab is missing or not executable")
    if not 1 <= args.workers <= MAX_WORKERS:
        raise ValueError("workers must be between 1 and 10")
    if not math.isfinite(args.seconds) or not 0 < args.seconds <= MAX_SECONDS:
        raise ValueError("seconds must be greater than zero and at most 290")
    if not math.isfinite(args.case_timeout) or args.case_timeout <= 0:
        raise ValueError("case timeout must be positive")
    catalog = list(gym.gallery(gallery_path))
    if any(Path(name).name != name or name in ("", ".", "..") for name, _ in catalog):
        raise ValueError("gallery contains an unsafe case name")
    spec = manifest(gallery_path, binary, Path(__file__).resolve())
    output.mkdir(parents=True, exist_ok=True)
    gym.pin(output, spec)
    stop, processes = threading.Event(), Processes()
    failures, active, deadline = {}, set(), time.monotonic() + args.seconds
    old = {}
    def interrupt(_signum, _frame): stop.set(); processes.kill()
    for signum in (signal.SIGINT, signal.SIGTERM, signal.SIGHUP):
        old[signum] = signal.signal(signum, interrupt)
    try:
        with gym.run_lock(output):
            complete = validate_saved(output, spec, catalog)
            pending = [(name, case) for name, case in catalog if name not in complete]
            with concurrent.futures.ThreadPoolExecutor(max_workers=args.workers) as pool:
                futures = {}
                while (pending or futures) and not stop.is_set():
                    while pending and len(futures) < args.workers and time.monotonic() < deadline:
                        name, case = pending.pop(0); active.add(name)
                        future = pool.submit(run_case, output, spec, name, case,
                                             min(args.case_timeout, max(0.01, deadline-time.monotonic())),
                                             stop, processes)
                        futures[future] = name
                    write_status(output, spec, catalog, failures, active)
                    if not futures: break
                    remaining = deadline - time.monotonic()
                    if remaining <= 0: stop.set(); processes.kill(); break
                    done, _ = concurrent.futures.wait(futures, timeout=min(.25, remaining),
                        return_when=concurrent.futures.FIRST_COMPLETED)
                    for future in done:
                        name = futures.pop(future); active.discard(name)
                        try: future.result()
                        except Exception as error: failures[name] = f"{type(error).__name__}: {error}"
                if stop.is_set(): processes.kill()
            status = write_status(output, spec, catalog, failures, active)
    finally:
        for signum, handler in old.items(): signal.signal(signum, handler)
    print(json.dumps(status, sort_keys=True))
    return 0 if status["completed_cases"] == status["target_cases"] else 75


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--gallery", type=Path,
        default=Path("/Users/jason/data/texas-42/partnership-bid-making-v1"))
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--workers", type=int, default=4)
    parser.add_argument("--case-timeout", type=float, default=240)
    parser.add_argument("--seconds", type=float, default=285)
    args = parser.parse_args(argv)
    try: return execute(args)
    except (OSError, ValueError, BlockingIOError) as error:
        print("policy_gym: " + str(error), file=sys.stderr); return 2


if __name__ == "__main__": sys.exit(main())
