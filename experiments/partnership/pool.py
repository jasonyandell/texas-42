#!/usr/bin/env python3
"""Shared, resumable game-worker pool. Always use the external watchdog."""

import argparse
import fcntl
import hashlib
import json
import os
import signal
import subprocess
import sys
import time
import uuid
from collections import deque
from pathlib import Path

import campaign as c


def tasks(path, spec):
    return [
        (path, seed, arm)
        for seed in range(spec["start"], spec["start"] + spec["count"])
        if not (path / "results" / f"{seed}.json").exists()
        for arm in c.arms_for(spec)
        if not (path / "seeds" / str(seed) / arm / "result.json").exists()
    ]


def interleave(groups):
    queues = [deque(g) for g in groups]
    out = deque()
    while any(queues):
        for q in queues:
            if q:
                out.append(q.popleft())
    return out


def failed_attempts(folder):
    return sum(
        c.read(p).get("status") == "failed"
        for p in (folder / "pool-attempts").glob("*.json")
    )


def commit_ready(path, spec):
    """Only publish a contiguous seed prefix, regardless of completion order."""
    added = []
    if (path / "STOP.json").exists():
        return added
    for seed in range(spec["start"], spec["start"] + spec["count"]):
        if (path / "results" / f"{seed}.json").exists():
            continue
        result = c.commit_seed(path, spec, seed)
        if result is None:
            break
        added.append(seed)
        summary = c.summarize(path, spec)
        print(
            json.dumps(
                {
                    "path": str(path),
                    "seed": seed,
                    "paired": result["paired"],
                    "completed": summary["completed_seeds"],
                }
            ),
            flush=True,
        )
        reason = c.early_reason(summary, spec)
        if reason:
            c.stop(path, reason)
            break
    return added


def publish(path, spec, workers, active, token):
    s = c.status(path)
    jobs = [
        {
            "seed": seed,
            "arm": arm,
            "plies": len(
                c.read(
                    path / "seeds" / str(seed) / arm / "checkpoint.json",
                    {"decisions": []},
                )["decisions"]
            ),
        }
        for p, seed, arm in active
        if p == path
    ]
    s["execution"] = {
        "mode": "shared-pool",
        "global_worker_limit": workers,
        "native_threads_per_game": spec["threads"],
        "session": token,
        "active_games": jobs,
    }
    c.atomic(path / "status.json", s)
    body = (path / "STATUS.md").read_text()
    old = f"Workers: {spec['workers']} games within one seed, {spec['threads']} native threads per game."
    new = f"Shared pool: up to **{workers} games concurrently**, across seeds/campaigns; {spec['threads']} native threads per game."
    body = body.replace(old, new)
    if jobs:
        body += (
            "\nActive games: "
            + ", ".join(f"{j['seed']} {j['arm']} {j['plies']}/28" for j in jobs)
            + ".\n"
        )
    c.atomic(path / "STATUS.md", body)
    return s


def advance(paths, workers=10, seconds=260, retries=2):
    if not 1 <= workers <= 64 or not 1 <= seconds <= 270 or not 0 <= retries <= 10:
        raise ValueError("invalid pool size, slice allowance, or retry count")
    paths = sorted(set(Path(p).resolve() for p in paths))
    specs = {p: c.load(p) for p in paths}
    for spec in specs.values():
        assert c.digest({k: v for k, v in spec.items() if k != "id"}) == spec["id"], (
            "manifest changed"
        )
    locks = []
    try:
        # All pools in this worktree share one machine budget. Additional
        # campaigns belong in the queue, not in competing pool processes.
        global_lock = (c.HERE / "campaigns" / "pool.lock").open("a+")
        locks.append(global_lock)
        fcntl.flock(global_lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        for p in paths:
            lock = (p / "runner.lock").open("a+")
            locks.append(lock)
            fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
    except BaseException:
        for lock in locks:
            lock.close()
        raise
    token = "pool-" + str(uuid.uuid4())
    yields = {p: p / "control" / (token + ".yield") for p in paths}
    sessions = {p: p / "sessions" / (token + ".json") for p in paths}
    session = {
        "id": token,
        "started_utc": c.now(),
        "status": "running",
        "workers": workers,
        "seconds": seconds,
        "retries": retries,
        "paths": [str(p) for p in paths],
        "scheduler_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        "committed": {str(p): [] for p in paths},
        "attempts": 0,
        "peak_active_games": 0,
    }
    began = time.monotonic()
    active = {}
    queue = interleave(
        [tasks(p, specs[p]) for p in paths if not (p / "STOP.json").exists()]
    )
    old_handlers = {}
    for sig in (signal.SIGINT, signal.SIGTERM):
        old_handlers[sig] = signal.signal(
            sig, lambda *_: [c.stop(p, "user-interrupt") for p in paths]
        )

    def save_session():
        for path in sessions.values():
            c.atomic(path, session)

    def yield_all():
        for path in yields.values():
            if not path.exists():
                c.atomic(path, "pause pool slice\n")

    try:
        save_session()
        for p in paths:
            session["committed"][str(p)] += commit_ready(p, specs[p])
        last_status = 0
        while queue or active:
            paused = time.monotonic() - began >= seconds
            if paused:
                yield_all()
            # Poll first: a slot becomes reusable as soon as one game exits.
            for task, entry in list(active.items()):
                proc, log, record, receipt = entry
                rc = proc.poll()
                if rc is None:
                    continue
                p, seed, arm = task
                folder = p / "seeds" / str(seed) / arm
                record.update(
                    returncode=rc,
                    finished_utc=c.now(),
                    end_ply=len(
                        c.read(folder / "checkpoint.json", {"decisions": []})[
                            "decisions"
                        ]
                    ),
                )
                okay = rc == 0 and (folder / "result.json").exists()
                interrupted = rc == 75 or paused or (p / "STOP.json").exists()
                record["status"] = (
                    "completed" if okay else "interrupted" if interrupted else "failed"
                )
                c.atomic(receipt, record)
                log.close()
                del active[task]
                if not okay and not interrupted:
                    if failed_attempts(folder) <= retries:
                        queue.appendleft(task)
                    else:
                        c.stop(p, f"worker-retries-exhausted:{seed}:{arm}:{rc}")
                session["committed"][str(p)] += commit_ready(p, specs[p])
            while queue and len(active) < workers and not paused:
                task = queue.popleft()
                p, seed, arm = task
                if (p / "STOP.json").exists():
                    continue
                folder = p / "seeds" / str(seed) / arm
                if (folder / "result.json").exists():
                    continue
                if failed_attempts(folder) > retries:
                    c.stop(p, f"worker-retries-exhausted:{seed}:{arm}")
                    continue
                folder.mkdir(parents=True, exist_ok=True)
                attempt = str(uuid.uuid4())
                receipt = folder / "pool-attempts" / (attempt + ".json")
                record = {
                    "session": token,
                    "seed": seed,
                    "arm": arm,
                    "workers": workers,
                    "native_threads": specs[p]["threads"],
                    "status": "running",
                    "started_utc": c.now(),
                    "start_ply": len(
                        c.read(folder / "checkpoint.json", {"decisions": []})[
                            "decisions"
                        ]
                    ),
                }
                c.atomic(receipt, record)
                log = (folder / (attempt + ".log")).open("w")
                cmd = [
                    sys.executable,
                    str(c.HERE / "campaign.py"),
                    "arm",
                    str(p),
                    "--seed",
                    str(seed),
                    "--arm",
                    arm,
                    "--yield-path",
                    str(yields[p]),
                ]
                try:
                    proc = subprocess.Popen(
                        cmd, stdout=log, stderr=subprocess.STDOUT, env=os.environ.copy()
                    )
                except OSError as error:
                    log.close()
                    record.update(
                        status="failed", error=str(error), finished_utc=c.now()
                    )
                    c.atomic(receipt, record)
                    queue.appendleft(task)
                    continue
                active[task] = (proc, log, record, receipt)
                session["attempts"] += 1
                session["peak_active_games"] = max(
                    session["peak_active_games"], len(active)
                )
                save_session()
            if time.monotonic() - last_status >= 2:
                for p in paths:
                    publish(p, specs[p], workers, active, token)
                last_status = time.monotonic()
            if not active and (paused or not queue):
                break
            time.sleep(0.1)
        session["status"] = "slice-complete"
    except BaseException as error:
        session["status"] = "error"
        session["error"] = str(error)
        for p in paths:
            c.stop(p, "pool-error:" + str(error))
        raise
    finally:
        yield_all()
        # Workers inherit the enclosing watchdog group. The external cap also
        # kills/reaps descendants if a worker fails to honor its move deadline.
        cleanup_end = time.monotonic() + 15
        for proc, log, record, receipt in active.values():
            try:
                rc = proc.wait(timeout=max(0.01, cleanup_end - time.monotonic()))
            except subprocess.TimeoutExpired:
                proc.kill()
                rc = proc.wait()
            record.update(status="interrupted", returncode=rc, finished_utc=c.now())
            c.atomic(receipt, record)
            log.close()
        for p in paths:
            if session["status"] != "error":
                session["committed"][str(p)] += commit_ready(p, specs[p])
            publish(p, specs[p], workers, {}, token)
        session.update(elapsed_seconds=time.monotonic() - began, finished_utc=c.now())
        save_session()
        for sig, handler in old_handlers.items():
            signal.signal(sig, handler)
        for lock in locks:
            lock.close()
    print(json.dumps(session), flush=True)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("paths", type=Path, nargs="*")
    parser.add_argument("--queue", type=Path)
    parser.add_argument("--workers", type=int, default=10)
    parser.add_argument("--seconds", type=int, default=260)
    parser.add_argument("--retries", type=int, default=2)
    args = parser.parse_args()
    if args.queue:
        if args.paths:
            parser.error("use paths or --queue, not both")
        config = c.read(args.queue)
        args.paths = [args.queue.resolve().parent / str(p) for p in config["campaigns"]]
        args.workers = config.get("workers", args.workers)
        args.seconds = config.get("seconds", args.seconds)
        args.retries = config.get("retries", args.retries)
    if not args.paths:
        parser.error("provide campaign paths or --queue")
    advance(args.paths, args.workers, args.seconds, args.retries)
