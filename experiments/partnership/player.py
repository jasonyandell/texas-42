#!/usr/bin/env python3
"""Budgeted straight-42 play oracle. One JSON request per line, one response.

Input: decl, bid, bidder, seat, hand (seven ORIGINAL tile ids), plays (flat
actor/tile pairs), seed (integer or decimal string). No hidden hands are inputs.
Modes partner/baseline/all-l1 use the native fixed-sample evaluator; phone uses
the preserved Plunge WASM. All have the same legal fallback and wall allowance.
"""
import argparse
import json
import os
from pathlib import Path
import subprocess
import sys
import time

from rules import information_state

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
BINARY = ROOT / "walt/target/release/partnership"
INPUT_KEYS = {"decl", "bid", "bidder", "seat", "hand", "plays", "seed"}


def native_text(req, mode, n=8, n0=2, n1=2, budget_ms=1000, inner_belief="voidless"):
    fields = {**req, "n": n, "n0": n0, "n1": n1, "budget_ms": budget_ms}
    fields["inner_belief"] = {"voidless": 0, "voids-counted": 1}[inner_belief]
    lines = [mode]
    for name, value in fields.items():
        values = value if isinstance(value, list) else [value]
        lines.append(name + " " + " ".join(str(v) for v in values))
    return "\n".join(lines) + "\n"


def child(command, text, allowance):
    if allowance <= 0:
        return None, "no-time"
    try:
        # Deliberately inherit the outer watchdog's process group. These two
        # workers launch no children. subprocess.run kills/reaps on timeout.
        result = subprocess.run(command, input=text, text=True,
                                capture_output=True, timeout=allowance,
                                env={**os.environ, "RAYON_NUM_THREADS": os.environ.get("WALT_RAYON_THREADS", "6")})
    except subprocess.TimeoutExpired:
        return None, "timeout"
    except (OSError, subprocess.SubprocessError) as error:
        return None, "worker-error: " + str(error)[-500:]
    if result.returncode:
        return None, "worker-error: " + result.stderr.strip()[-500:]
    try:
        value = json.loads(result.stdout)
    except json.JSONDecodeError:
        return None, "invalid-worker-output"
    if not isinstance(value, dict):
        return None, "invalid-worker-output"
    if "error" in value:
        return None, "worker-error: " + str(value["error"])
    return value, "completed"


def checked_response(value, state):
    """Accept only a complete, independently conforming worker decision."""
    if not isinstance(value, dict):
        return None, "rejected: malformed-response"
    choice = value.get("choice")
    if type(choice) is not int or choice not in state["legal"]:
        return None, "rejected: illegal-or-missing-choice"
    if value.get("leader") != state["leader"] or value.get("points") != state["points"]:
        return None, "rejected: leader-or-points-disagreement"
    return choice, "completed"


def checked_status(value, state):
    if not isinstance(value, dict):
        return "rejected: malformed-response"
    for key in ("legal", "leader", "points", "trick"):
        if value.get(key) != state[key]:
            return "rejected: state-disagreement"
    return "completed"


def normalize(raw):
    if not isinstance(raw, dict) or set(raw)-INPUT_KEYS:
        raise ValueError("request must contain only decl, bid, bidder, seat, hand, plays, seed")
    req = {**raw}
    for key in ("decl", "bid", "bidder", "seat"):
        if type(req.get(key)) is not int:
            raise ValueError(key + " must be an integer")
    for key in ("hand", "plays"):
        values = req.setdefault(key, [])
        if not isinstance(values, list) or any(type(x) is not int or x < 0 for x in values):
            raise ValueError(key + " must be an unsigned integer list")
    seed = req.get("seed", "13249961062380153451")
    if isinstance(seed, bool) or not isinstance(seed, (int, str)):
        raise ValueError("seed must be an integer or decimal string")
    req["seed"] = int(seed)
    if not 0 <= req["seed"] < 2**64:
        raise ValueError("seed must fit u64")
    return req


def decide(raw, mode="partner", n=40, n0=8, n1=2, budget_ms=14000, inner_belief="voidless"):
    start = time.monotonic()
    if mode not in ("partner", "baseline", "all-l1", "phone"):
        raise ValueError("unknown player mode")
    if not 100 <= budget_ms <= 14000:
        raise ValueError("budget_ms must be 100..14000 (four seats fit one minute)")
    if not 1 <= n <= 640 or not 1 <= n0 <= 64 or not 1 <= n1 <= 64:
        raise ValueError("invalid sample counts")
    if inner_belief not in ("voidless", "voids-counted"):
        raise ValueError("unknown inner belief strategy")
    if mode == "phone" and inner_belief != "voidless":
        raise ValueError("the preserved phone does not support inner belief selection")
    deadline = start + budget_ms / 1000
    reserve = min(0.10, budget_ms / 10000)
    req = normalize(raw)
    state = information_state(req)
    legal = state["legal"]
    # This permanent fallback uses only own legal tiles. Its weakness is
    # visible in the record and is included in executed-policy comparisons.
    choice = legal[0]
    route = "forced" if len(legal) == 1 else "legal-fallback"
    phases = []
    evaluation = None
    fallback_evaluation = None
    # Native replay is a conformance cross-check only. Local validation above
    # already established the move that survives any worker failure.
    t = time.monotonic()
    allowance = min(1.0, max(0, deadline - t - reserve))
    native_state, status = child(
        [str(BINARY)], native_text(req, "status"), allowance)
    if native_state is not None:
        status = checked_status(native_state, state)
    phases.append({"name": "status-check", "status": status,
                   "elapsed_us": round((time.monotonic() - t) * 1_000_000)})
    if len(legal) > 1:
        # A cheap, complete L1 comparison is retained before deeper work.
        # Incomplete comparisons are never used to select a move.
        t = time.monotonic()
        allowance = min(1.5, max(0, (deadline - t - reserve) / 4))
        fallback_evaluation, status = child(
            [str(BINARY)], native_text(req, "baseline", 8, 2, 2,
                                       max(0, int(allowance*1000)-40), inner_belief), allowance)
        fallback_choice = None
        if fallback_evaluation is not None:
            fallback_choice, status = checked_response(fallback_evaluation, state)
        phases.append({"name": "fallback-l1", "status": status,
                       "elapsed_us": round((time.monotonic()-t)*1_000_000)})
        if fallback_choice is not None:
            choice = fallback_choice
            route = "l1-fallback"
        t = time.monotonic()
        allowance = max(0, deadline - t - reserve)
        if mode == "phone":
            text = json.dumps({**req, "seed": str(req["seed"]),
                               "n": n, "n0": n0, "race": True})
            command = ["node", str(HERE / "phone.mjs")]
        else:
            text = native_text(req, mode, n, n0, n1,
                               max(0, int(allowance*1000)-40), inner_belief)
            command = [str(BINARY)]
        evaluation, status = child(command, text, allowance)
        primary_choice = None
        if evaluation is not None:
            primary_choice, status = checked_response(evaluation, state)
        phases.append({"name": mode, "status": status,
                       "elapsed_us": round((time.monotonic()-t)*1_000_000)})
        if primary_choice is not None:
            choice = primary_choice
            route = mode
    elapsed_us = round((time.monotonic()-start)*1_000_000)
    return {"schema": "partnership-decision-v1", "choice": choice, "route": route,
            **state, "mode": mode, "inner_belief": inner_belief, "n": n, "n0": n0, "n1": n1,
            "budget_ms": budget_ms, "elapsed_us": elapsed_us,
            "over_budget": elapsed_us > budget_ms*1000,
            "phases": phases, "evaluation": evaluation,
            "fallback_evaluation": fallback_evaluation}


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--mode", choices=["partner", "baseline", "all-l1", "phone"], default="partner")
    p.add_argument("--inner-belief", choices=["voidless", "voids-counted"], default="voidless")
    p.add_argument("--n", type=int, default=40)
    p.add_argument("--n0", type=int, default=8)
    p.add_argument("--n1", type=int, default=2)
    p.add_argument("--budget-ms", type=int, default=14000)
    args = p.parse_args()
    for line in sys.stdin:
        if not line.strip():
            continue
        try:
            result = decide(json.loads(line), **vars(args))
        except (ValueError, RuntimeError, OSError) as error:
            result = {"error": str(error)}
        print(json.dumps(result, separators=(",", ":")), flush=True)


if __name__ == "__main__":
    main()
