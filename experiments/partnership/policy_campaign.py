#!/usr/bin/env python3
"""Run reproducible, resumable policy-synthesis experiments.

The native ``policy_lab`` process owns one seed and prints one JSON object on
stdout.  This wrapper pins every experiment input, commits complete seeds with
an atomic rename, and never treats partial stdout or a failed process as data.
"""

import argparse
import concurrent.futures
import datetime
import fcntl
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

from process_groups import kill_process_group


SCHEMA = "texas42-policy-campaign-v1"
RESULT_SCHEMA = "texas42-policy-campaign-seed-v1"
MAX_WORKERS = 10
MAX_SECONDS = 290.0
FIELD_IDS = {"hash-legal": "hash-legal-v1", "l0-8": "field:level0-n8-v1"}


def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def sha256(path):
    h = hashlib.sha256()
    with Path(path).open("rb") as source:
        for chunk in iter(lambda: source.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def digest(value):
    return hashlib.sha256(
        json.dumps(value, sort_keys=True, separators=(",", ":")).encode()
    ).hexdigest()


def atomic(path, value):
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_name("." + path.name + ".{}.tmp".format(os.getpid()))
    with temporary.open("w", encoding="utf-8") as out:
        json.dump(value, out, sort_keys=True, separators=(",", ":"))
        out.write("\n")
        out.flush()
        os.fsync(out.fileno())
    os.replace(temporary, path)
    directory = os.open(str(path.parent), os.O_RDONLY)
    try:
        os.fsync(directory)
    finally:
        os.close(directory)


def read_json(path):
    with Path(path).open(encoding="utf-8") as source:
        return json.load(source)


def positive_int(text):
    value = int(text)
    if value <= 0:
        raise argparse.ArgumentTypeError("must be positive")
    return value


def csv_ints(text, *, nonnegative=False):
    try:
        values = [int(part) for part in text.split(",") if part != ""]
    except ValueError as error:
        raise argparse.ArgumentTypeError("expected comma-separated integers") from error
    if not values or len(set(values)) != len(values):
        raise argparse.ArgumentTypeError("values must be nonempty and distinct")
    lower = 0 if nonnegative else 1
    if any(value < lower for value in values):
        raise argparse.ArgumentTypeError("values out of range")
    return values


def seed_list(text):
    if ":" not in text:
        return csv_ints(text, nonnegative=True)
    pieces = text.split(":")
    if len(pieces) not in (2, 3):
        raise argparse.ArgumentTypeError("seed range is START:STOP[:STEP]")
    try:
        start, stop = int(pieces[0]), int(pieces[1])
        step = int(pieces[2]) if len(pieces) == 3 else 1
    except ValueError as error:
        raise argparse.ArgumentTypeError("seed range contains a non-integer") from error
    values = list(range(start, stop, step)) if step else []
    if not values or min(values) < 0:
        raise argparse.ArgumentTypeError("seed range must be nonempty and nonnegative")
    return values


def manifest_id(manifest):
    return digest({key: value for key, value in manifest.items() if key != "id"})


def initialize(args):
    output = args.output.resolve()
    binary = args.binary.resolve()
    if output.exists():
        raise ValueError("output already exists; use run to resume")
    if not binary.is_file() or not os.access(binary, os.X_OK):
        raise ValueError("policy_lab binary is missing or not executable")
    if not 1 <= args.workers <= MAX_WORKERS:
        raise ValueError("workers must be between 1 and 10")
    if any(seed >= 2**64 for seed in args.seeds):
        raise ValueError("seeds must fit unsigned 64-bit integers")
    if args.tiles < 2 or args.tiles > 7:
        raise ValueError("tiles must be between 2 and 7")
    if (not args.samples or len(set(args.samples)) != len(args.samples)
            or args.samples != sorted(args.samples) or min(args.samples) < 1
            or max(args.samples) > 512):
        raise ValueError("samples must strictly increase and end at no more than 512")
    if not 1 <= args.test_worlds <= 10000:
        raise ValueError("test worlds must be between 1 and 10000")
    if not 1 <= args.node_budget <= 20_000_000:
        raise ValueError("node budget must be between 1 and 20000000")
    if (not math.isfinite(args.seed_timeout) or args.seed_timeout <= 0
            or args.seed_timeout > MAX_SECONDS):
        raise ValueError("seed timeout must be greater than zero and at most 290")
    if args.mode == "fixed-root-hand" and args.hand is None:
        raise ValueError("fixed-root-hand requires exactly seven --hand tile ids")
    if args.mode == "random-own-hand" and args.hand is not None:
        raise ValueError("--hand is only valid for fixed-root-hand")
    if args.hand is not None and (len(args.hand) != 7 or len(set(args.hand)) != 7
                                  or min(args.hand) < 0 or max(args.hand) > 27):
        raise ValueError("--hand needs seven distinct tile ids from 0 through 27")
    runner = Path(__file__).resolve()
    configuration = {
        "mode": args.mode,
        "field": args.field,
        "seeds": args.seeds,
        "tiles": args.tiles,
        "samples": args.samples,
        "test_worlds": args.test_worlds,
        "node_budget": args.node_budget,
        "decl": args.decl,
        "hand": args.hand,
        "seed_timeout_seconds": args.seed_timeout,
        "workers": args.workers,
    }
    manifest = {
        "schema": SCHEMA,
        "created_utc": now(),
        "configuration": configuration,
        "binary": {"path": str(binary), "sha256": sha256(binary)},
        "source": {"path": str(runner), "sha256": sha256(runner)},
        "process_groups": {
            "path": str(runner.with_name("process_groups.py")),
            "sha256": sha256(runner.with_name("process_groups.py")),
        },
        "command_contract": "policy-lab-cli-v1-json-stdout",
    }
    manifest["id"] = manifest_id(manifest)
    output.mkdir(parents=True)
    atomic(output / "manifest.json", manifest)
    write_status(output, manifest)
    return manifest


def load(output, verify=True):
    output = Path(output).resolve()
    manifest = read_json(output / "manifest.json")
    if manifest.get("schema") != SCHEMA or manifest_id(manifest) != manifest.get("id"):
        raise ValueError("campaign manifest is corrupt")
    if verify:
        for kind in ("binary", "source", "process_groups"):
            identity = manifest[kind]
            path = Path(identity["path"])
            if not path.is_file() or sha256(path) != identity["sha256"]:
                raise ValueError(kind + " fingerprint changed; refusing mixed resume")
    return manifest


def result_path(output, seed):
    return Path(output) / "results" / (str(seed) + ".json")


def validate_result(row, manifest, seed):
    if row.get("schema") != RESULT_SCHEMA:
        raise ValueError("result {} has the wrong schema".format(seed))
    if row.get("campaign") != manifest["id"] or row.get("seed") != seed:
        raise ValueError("result {} has the wrong campaign or seed".format(seed))
    if not isinstance(row.get("result"), dict):
        raise ValueError("result {} has no native JSON object".format(seed))
    validate_native(row["result"], manifest, seed)
    return row


def validate_native(native, manifest, seed):
    config = manifest["configuration"]
    if native.get("schema") != "policy-lab-v1" or native.get("seed") != seed:
        raise ValueError("seed {} native identity is invalid".format(seed))
    if native.get("tiles") != config["tiles"] or native.get("decl") != config["decl"]:
        raise ValueError("seed {} native configuration disagrees with manifest".format(seed))
    if native.get("field") != FIELD_IDS[config["field"]]:
        raise ValueError("seed {} native field disagrees with manifest".format(seed))
    rows = native.get("rows")
    if not isinstance(rows, list):
        raise ValueError("seed {} has no rows".format(seed))
    expected = {(samples, arm) for samples in config["samples"]
                for arm in ("fresh", "persistent", "compose")}
    actual = set()
    for item in rows:
        if not isinstance(item, dict):
            raise ValueError("seed {} contains a non-object row".format(seed))
        key = (item.get("samples"), item.get("arm"))
        if key in actual or key not in expected:
            raise ValueError("seed {} contains an unexpected or duplicate row".format(seed))
        actual.add(key)
        if item.get("status") not in ("completed", "budget"):
            raise ValueError("seed {} row has an invalid status".format(seed))
        solved = item.get("solved_samples")
        if not isinstance(solved, int) or isinstance(solved, bool) \
                or not 0 <= solved <= item["samples"]:
            raise ValueError("seed {} row has invalid solved_samples".format(seed))
        if item["status"] == "completed" and solved != item["samples"]:
            raise ValueError("seed {} completed row is only partially solved".format(seed))
        if not isinstance(item.get("policy_id"), str) or not item["policy_id"]:
            raise ValueError("seed {} row has no policy_id".format(seed))
        for field in ("train_makes", "train_worlds", "test_makes", "test_worlds",
                      "nodes", "cache_hits", "policy_states", "elapsed_ms"):
            value = item.get(field)
            if not isinstance(value, (int, float)) or isinstance(value, bool) or value < 0:
                raise ValueError("seed {} row has invalid {}".format(seed, field))
        if item["train_makes"] > item["train_worlds"] \
                or item["test_makes"] > item["test_worlds"]:
            raise ValueError("seed {} row has impossible make counts".format(seed))
        if item["status"] == "completed" and item["train_worlds"] != item["samples"]:
            raise ValueError("seed {} completed row has wrong training size".format(seed))
        if item["test_worlds"] != config["test_worlds"]:
            raise ValueError("seed {} row uses the wrong heldout panel size".format(seed))
    if actual != expected:
        raise ValueError("seed {} does not contain the complete schedule".format(seed))
    by_key = {(item["samples"], item["arm"]): item for item in rows}
    for samples in config["samples"]:
        stage = [by_key[(samples, arm)] for arm in ("fresh", "persistent", "compose")]
        if len({item["train_worlds"] for item in stage}) != 1:
            raise ValueError("seed {} arms use different training panels".format(seed))
        if len({item["test_worlds"] for item in stage}) != 1:
            raise ValueError("seed {} arms use different heldout panels".format(seed))
        fresh, persistent = stage[:2]
        if fresh["status"] == persistent["status"] == "completed":
            if fresh["train_makes"] != persistent["train_makes"]:
                raise ValueError("seed {} exact persistent value differs from fresh".format(seed))
    return native


def completed(output, manifest):
    rows = {}
    for seed in manifest["configuration"]["seeds"]:
        path = result_path(output, seed)
        if path.exists():
            rows[seed] = validate_result(read_json(path), manifest, seed)
    return rows


def native_command(manifest, seed, artifact_dir):
    config = manifest["configuration"]
    command = [
        manifest["binary"]["path"],
        "--seed", str(seed),
        "--tiles", str(config["tiles"]),
        "--samples", ",".join(map(str, config["samples"])),
        "--test-worlds", str(config["test_worlds"]),
        "--node-budget", str(config["node_budget"]),
        "--decl", str(config["decl"]),
        "--mode", config["mode"],
        "--field", config["field"],
        "--artifact-dir", str(artifact_dir),
    ]
    if config["hand"] is not None:
        command.extend(("--hand", ",".join(map(str, config["hand"]))))
    return command


class Processes:
    def __init__(self):
        self.lock = threading.Lock()
        self.items = set()

    def add(self, proc):
        with self.lock:
            self.items.add(proc)

    def remove(self, proc):
        with self.lock:
            self.items.discard(proc)

    def kill_all(self):
        with self.lock:
            items = list(self.items)
            # Transfer kill ownership to this caller. Concurrent cleanup sees
            # an empty registry rather than signalling the same live group.
            self.items.clear()
        for proc in items:
            kill_process_group(proc)


def run_seed(output, manifest, seed, stop, processes):
    if stop.is_set():
        return None
    seed_dir = Path(output) / "seeds" / str(seed)
    artifacts = seed_dir / "artifacts"
    artifacts.mkdir(parents=True, exist_ok=True)
    command = native_command(manifest, seed, artifacts)
    started_utc, started = now(), time.monotonic()
    proc = subprocess.Popen(
        command, stdin=subprocess.DEVNULL, stdout=subprocess.PIPE,
        stderr=subprocess.PIPE, text=True, start_new_session=True,
    )
    processes.add(proc)
    if stop.is_set():
        processes.kill_all()
    timed_out = False
    try:
        try:
            stdout, stderr = proc.communicate(
                timeout=manifest["configuration"]["seed_timeout_seconds"]
            )
        except subprocess.TimeoutExpired:
            timed_out = True
            kill_process_group(proc)
            stdout, stderr = proc.communicate()
    finally:
        processes.remove(proc)
    atomic(seed_dir / "attempt.json", {
        "command": command, "started_utc": started_utc,
        "elapsed_seconds": round(time.monotonic() - started, 6),
        "returncode": proc.returncode, "timed_out": timed_out,
        "stdout": stdout, "stderr": stderr,
    })
    if timed_out:
        raise TimeoutError("seed {} exceeded its timeout".format(seed))
    if proc.returncode:
        raise RuntimeError("seed {} exited {}: {}".format(seed, proc.returncode,
                                                           stderr.strip()[-500:]))
    try:
        native = json.loads(stdout)
    except json.JSONDecodeError as error:
        raise ValueError("seed {} emitted invalid JSON".format(seed)) from error
    if not isinstance(native, dict):
        raise ValueError("seed {} result must be a JSON object".format(seed))
    validate_native(native, manifest, seed)
    row = {
        "schema": RESULT_SCHEMA, "campaign": manifest["id"], "seed": seed,
        "finished_utc": now(), "elapsed_seconds": round(time.monotonic() - started, 6),
        "result": native,
    }
    atomic(result_path(output, seed), row)
    return row


def aggregate(rows):
    """Pair arms within each seed and training-prefix size."""
    methods, stages = {}, {}
    for row in rows.values():
        native_rows = row["result"]["rows"]
        by_key = {(item["samples"], item["arm"]): item for item in native_rows}
        for item in native_rows:
            name = item["arm"]
            target = methods.setdefault(name, {})
            for metric in ("train_makes", "test_makes", "nodes", "cache_hits",
                           "policy_states", "elapsed_ms"):
                target.setdefault(metric, []).append(item[metric])
        for samples in {item["samples"] for item in native_rows}:
            fresh = by_key[(samples, "fresh")]
            stage = stages.setdefault(str(samples), {
                "seeds": 0,
                "persistent_minus_fresh_test_makes": [],
                "compose_minus_fresh_test_makes": [],
                "persistent_minus_fresh_nodes": [],
                "compose_minus_fresh_nodes": [],
                "persistent_value_mismatches": 0,
                "persistent_policy_mismatches": 0,
                "persistent_policy_comparisons": 0,
                "persistent_budget_advantages": 0,
            })
            stage["seeds"] += 1
            for arm in ("persistent", "compose"):
                other = by_key[(samples, arm)]
                stage[arm + "_minus_fresh_test_makes"].append(
                    other["test_makes"] - fresh["test_makes"])
                stage[arm + "_minus_fresh_nodes"].append(other["nodes"] - fresh["nodes"])
            persistent = by_key[(samples, "persistent")]
            if fresh["status"] == persistent["status"] == "completed":
                stage["persistent_policy_comparisons"] += 1
                stage["persistent_value_mismatches"] += int(
                    (persistent["train_makes"], persistent["train_worlds"],
                     persistent["test_makes"], persistent["test_worlds"])
                    != (fresh["train_makes"], fresh["train_worlds"],
                        fresh["test_makes"], fresh["test_worlds"]))
                stage["persistent_policy_mismatches"] += int(
                    persistent["policy_id"] != fresh["policy_id"])
            stage["persistent_budget_advantages"] += int(
                fresh["status"] == "budget" and persistent["status"] == "completed")
    arms = {
        method: {
            metric: {"n": len(values), "mean": sum(values) / len(values),
                     "min": min(values), "max": max(values)}
            for metric, values in metrics.items()
        }
        for method, metrics in methods.items()
    }
    for stage in stages.values():
        for key, values in list(stage.items()):
            if isinstance(values, list):
                stage[key] = {"n": len(values), "mean": sum(values) / len(values),
                              "min": min(values), "max": max(values)}
    schedule_totals = {
        arm: {
            "nodes": sum(item["nodes"] for row in rows.values()
                         for item in row["result"]["rows"] if item["arm"] == arm),
            "elapsed_ms": sum(item["elapsed_ms"] for row in rows.values()
                              for item in row["result"]["rows"] if item["arm"] == arm),
        }
        for arm in ("fresh", "persistent", "compose")
    }
    schedule_totals["persistent_minus_fresh_nodes"] = (
        schedule_totals["persistent"]["nodes"] - schedule_totals["fresh"]["nodes"])
    schedule_totals["compose_minus_fresh_nodes"] = (
        schedule_totals["compose"]["nodes"] - schedule_totals["fresh"]["nodes"])
    return {"arms": arms, "paired_by_samples": stages,
            "schedule_totals": schedule_totals}


def write_status(output, manifest, active=None, failures=None):
    rows = completed(output, manifest)
    seeds = manifest["configuration"]["seeds"]
    summary = aggregate(rows)
    status = {
        "schema": "texas42-policy-campaign-status-v1",
        "campaign": manifest["id"], "updated_utc": now(),
        "completed_seeds": len(rows), "target_seeds": len(seeds),
        "pending_seeds": [seed for seed in seeds if seed not in rows],
        "active_seeds": sorted(active or []),
        "failures": failures or {}, **summary,
    }
    atomic(Path(output) / "status.json", status)
    atomic(Path(output) / "summary.json", {
        key: status[key] for key in (
            "schema", "campaign", "updated_utc", "completed_seeds",
            "target_seeds", "arms", "paired_by_samples", "schedule_totals")
    })
    return status


def run(output, seconds):
    output = Path(output).resolve()
    manifest = load(output)
    config = manifest["configuration"]
    if not math.isfinite(seconds) or not 0 < seconds <= MAX_SECONDS:
        raise ValueError("seconds must be greater than zero and at most 290")
    lock_file = (output / "runner.lock").open("w")
    fcntl.flock(lock_file, fcntl.LOCK_EX | fcntl.LOCK_NB)
    stop, processes = threading.Event(), Processes()
    deadline = time.monotonic() + seconds
    failures, active = {}, set()
    old_handlers = {}

    def interrupt(_signum, _frame):
        stop.set()
        processes.kill_all()

    for signum in (signal.SIGINT, signal.SIGTERM, signal.SIGHUP):
        old_handlers[signum] = signal.signal(signum, interrupt)
    try:
        done = completed(output, manifest)
        pending = [seed for seed in config["seeds"] if seed not in done]
        with concurrent.futures.ThreadPoolExecutor(max_workers=config["workers"]) as pool:
            futures = {}
            while (pending or futures) and not stop.is_set():
                while (pending and len(futures) < config["workers"]
                       and time.monotonic() < deadline):
                    seed = pending.pop(0)
                    active.add(seed)
                    futures[pool.submit(run_seed, output, manifest, seed,
                                        stop, processes)] = seed
                write_status(output, manifest, active, failures)
                if not futures:
                    break
                remaining = deadline - time.monotonic()
                if remaining <= 0:
                    stop.set()
                    processes.kill_all()
                    break
                finished, _ = concurrent.futures.wait(
                    futures, timeout=min(0.25, remaining),
                    return_when=concurrent.futures.FIRST_COMPLETED)
                for future in finished:
                    seed = futures.pop(future)
                    active.discard(seed)
                    try:
                        future.result()
                    except Exception as error:
                        failures[str(seed)] = "{}: {}".format(type(error).__name__, error)
            if stop.is_set():
                processes.kill_all()
        status = write_status(output, manifest, active, failures)
    finally:
        for signum, handler in old_handlers.items():
            signal.signal(signum, handler)
        lock_file.close()
    print(json.dumps(status, sort_keys=True))
    return 0 if status["completed_seeds"] == status["target_seeds"] else 75


def parser():
    top = argparse.ArgumentParser(
        description=__doc__,
        epilog=("Example: policy_campaign.py init --output runs/policy-pilot "
                "--binary walt/target/release/policy_lab --mode random-own-hand "
                "--field hash-legal --seeds 700000:700020 --tiles 4 "
                "--samples 1,2,4,8,16,32,64 && policy_campaign.py run "
                "--output runs/policy-pilot --seconds 285"))
    commands = top.add_subparsers(dest="command", required=True)
    init = commands.add_parser("init", help="create an immutable campaign")
    init.add_argument("--output", type=Path, required=True)
    init.add_argument("--binary", type=Path, required=True)
    init.add_argument("--mode", choices=("random-own-hand", "fixed-root-hand"),
                      default="random-own-hand")
    init.add_argument("--field", choices=("hash-legal", "l0-8"),
                      default="hash-legal",
                      help="frozen opponent/partner policy used for every arm")
    init.add_argument("--seeds", type=seed_list, required=True,
                      help="comma list or half-open START:STOP[:STEP]")
    init.add_argument("--tiles", type=int, default=7)
    init.add_argument("--samples", type=csv_ints,
                      default=[1, 2, 4, 8, 16, 32, 64])
    init.add_argument("--test-worlds", type=positive_int, default=256)
    init.add_argument("--node-budget", type=positive_int, default=100000)
    init.add_argument("--decl", type=int, choices=(*range(7), 7, 9), default=6)
    init.add_argument("--hand", type=lambda value: csv_ints(value, nonnegative=True))
    init.add_argument("--seed-timeout", type=float, default=120.0)
    init.add_argument("--workers", type=int, default=4)
    go = commands.add_parser("run", help="run or resume pending seeds")
    go.add_argument("--output", type=Path, required=True)
    go.add_argument("--seconds", type=float, default=285.0)
    show = commands.add_parser("status", help="validate and print current status")
    show.add_argument("--output", type=Path, required=True)
    return top


def main(argv=None):
    args = parser().parse_args(argv)
    try:
        if args.command == "init":
            manifest = initialize(args)
            print(json.dumps(manifest, sort_keys=True))
            return 0
        if args.command == "run":
            return run(args.output, args.seconds)
        manifest = load(args.output)
        print(json.dumps(write_status(args.output, manifest), sort_keys=True))
        return 0
    except (OSError, ValueError, BlockingIOError) as error:
        print("policy_campaign: " + str(error), file=sys.stderr)
        return 2


if __name__ == "__main__":
    sys.exit(main())
