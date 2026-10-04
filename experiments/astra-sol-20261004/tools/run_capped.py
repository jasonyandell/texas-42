#!/usr/bin/env python3
"""Run one foreground POSIX experiment with a short, enforced wall allowance.

Python 3.9+ standard library; macOS/Linux. Children must stay in the process
group (no daemonization, new sessions, or external job submission). A watchdog
cannot enforce a real-time OS guarantee or cancel detached/external GPU jobs.
295 seconds leaves cleanup room below the task's 300-second absolute ceiling.
"""

import argparse
import datetime
import json
import math
import os
from pathlib import Path
import signal
import subprocess
import sys
import time


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--seconds", type=float, default=295.0)
    parser.add_argument("--output-dir", type=Path, required=True)
    parser.add_argument("command", nargs=argparse.REMAINDER)
    args = parser.parse_args()
    if os.name != "posix":
        parser.error("This runner requires macOS/Linux POSIX process groups.")
    if not math.isfinite(args.seconds) or not 0 < args.seconds <= 295:
        parser.error("--seconds must be greater than 0 and at most 295.")
    command = args.command
    if command[:1] == ["--"]:
        command = command[1:]
    if not command:
        parser.error("Supply an executable and its arguments after --.")
    output = args.output_dir.resolve()
    try:
        output.mkdir(parents=True, exist_ok=False)
    except OSError as error:
        parser.error("Use a new writable output directory: " + str(error))

    proc = None
    interrupted = None
    cleanup_errors = []

    def kill_group():
        if proc is not None:
            try:
                os.killpg(proc.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            except OSError as error:
                cleanup_errors.append(str(error))

    def on_signal(signum, _frame):
        nonlocal interrupted
        interrupted = signum
        kill_group()

    original_handlers = {}
    for signum in (signal.SIGINT, signal.SIGTERM, signal.SIGHUP):
        original_handlers[signum] = signal.signal(signum, on_signal)

    started_utc = datetime.datetime.now(datetime.timezone.utc).isoformat()
    started = time.monotonic()
    deadline = started + args.seconds
    status = "spawn_error"
    error_text = None
    child_returncode = None
    exit_code = 125
    try:
        with (output / "stdout.log").open("wb") as stdout, \
                (output / "stderr.log").open("wb") as stderr:
            proc = subprocess.Popen(
                command,
                stdin=subprocess.DEVNULL,
                stdout=stdout,
                stderr=stderr,
                start_new_session=True,
            )
            if interrupted is not None:
                kill_group()
            try:
                child_returncode = proc.wait(
                    timeout=max(0.0, deadline - time.monotonic())
                )
                if interrupted is not None:
                    status = "interrupted"
                    exit_code = 128 + interrupted
                else:
                    status = "completed" if child_returncode == 0 else "failed"
                    exit_code = (child_returncode if child_returncode >= 0
                                 else 128 - child_returncode)
            except subprocess.TimeoutExpired:
                status = "timed_out"
                exit_code = 124
                # No grace period for the workload: terminate the group now.
                kill_group()
            finally:
                # A parent can exit while its children still run. Clean up the
                # group even for normal exit, so background work cannot persist.
                kill_group()
                if proc.poll() is None:
                    try:
                        proc.wait(timeout=3.0)
                    except subprocess.TimeoutExpired:
                        cleanup_errors.append("Process did not reap after SIGKILL.")
                child_returncode = proc.returncode
    except OSError as error:
        error_text = str(error)
        kill_group()
    finally:
        for signum, handler in original_handlers.items():
            signal.signal(signum, handler)

    if cleanup_errors:
        exit_code = 125
    elapsed = time.monotonic() - started
    receipt = {
        "schema": "texas42-partnership-run-v1",
        "started_utc": started_utc,
        "cwd": os.getcwd(),
        "command": command,
        "allowance_seconds": args.seconds,
        "experiment_ceiling_seconds": 300,
        "elapsed_seconds": round(elapsed, 6),
        "status": status,
        "child_pid": None if proc is None else proc.pid,
        "child_returncode": child_returncode,
        "runner_returncode": exit_code,
        "interruption_signal": interrupted,
        "error": error_text,
        "cleanup_errors": cleanup_errors,
        "stdout": "stdout.log",
        "stderr": "stderr.log",
    }
    (output / "run.json").write_text(
        json.dumps(receipt, indent=2) + "\n", encoding="utf-8"
    )
    print("{} in {:.3f}s; record: {}".format(status, elapsed, output / "run.json"))
    return exit_code


if __name__ == "__main__":
    sys.exit(main())
