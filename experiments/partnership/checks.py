#!/usr/bin/env python3
"""Focused checks using the already-built library, without building all probe bins."""
import json
import os
from pathlib import Path
import subprocess
import sys
import time
import random
from unittest.mock import patch

from experiment import replay_record
from player import BINARY, HERE, ROOT, INPUT_KEYS, decide, native_text, child
from rules import information_state, legal_tiles, winner, trick_points


def main():
    deps = ROOT / "walt/target/release/deps"
    command = ["rustc", "--edition=2021", "--test",
               str(ROOT/"walt/walt/tests/solver_partnership.rs"),
               "-L", "dependency="+str(deps), "-C", "opt-level=3",
               "-C", "overflow-checks=yes"]
    for name in ("walt", "num_rational"):
        libs = list(deps.glob("lib"+name+"-*.rlib"))
        assert len(libs) == 1, ("ambiguous library; clean isolated build required", name, libs)
        command.extend(["--extern", name+"="+str(libs[0])])
    test_binary = ROOT/"walt/target/release/partnership_checks"
    command.extend(["-o", str(test_binary)])
    subprocess.run(command, check=True)
    subprocess.run([str(test_binary), "--test-threads=1"], check=True,
                   env={**os.environ, "RAYON_NUM_THREADS": "6"})
    fixtures = json.loads((HERE/"fixtures.json").read_text())
    deal = fixtures["full_deals"][0]
    points, _, remaining, _ = replay_record(deal["hands"], deal["record"], deal["decl"], deal["bidder"])
    assert points == [25, 17] and not any(remaining)
    print("PASS independent G1 full-deal legality, winner, count, score replay", flush=True)
    for root in fixtures["roots"]:
        req = {k: v for k, v in root.items() if k in INPUT_KEYS}
        value, status = child([str(BINARY)], native_text(req, "status"), 1)
        assert value is not None, (root["id"], status)
    print("PASS all named bidder/partner/defender information roots", flush=True)
    root = fixtures["roots"][0]
    req = {k: v for k, v in root.items() if k in INPUT_KEYS}
    small = decide(req, "all-l1", 640, 64, 64, 100)
    assert small["choice"] in small["legal"] and "fallback" in small["route"]
    assert small["elapsed_us"] < 200_000
    print("PASS externally bounded 100 ms decision returns labeled legal fallback: " + str(small["elapsed_us"]) + " us", flush=True)
    for mutation in ({"hands": deal["hands"]}, {"seed": -1}, {"plays": [0, 0, 0, 0]}, {"hand": [0]*7}):
        try:
            decide({**req, **mutation}, budget_ms=100)
        except ValueError:
            continue
        raise AssertionError("invalid/private input accepted: "+str(mutation))
    for kwargs in ({"n": 0}, {"n0": 0}, {"n1": 0}, {"budget_ms": 60000}):
        try:
            decide(req, **kwargs)
        except ValueError:
            continue
        raise AssertionError("invalid knob accepted: "+str(kwargs))
    print("PASS private input, replay, seed, hand, sample, and time-budget validation", flush=True)
    state = information_state(req)
    with patch("player.child", return_value=(None, "simulated-worker-failure")):
        resp = decide(req, budget_ms=100)
        assert resp["route"] == "legal-fallback" and resp["choice"] in state["legal"]
    bad = {"choice": 99, "leader": state["leader"], "points": state["points"]}
    with patch("player.child", side_effect=[(state, "completed"), (bad, "completed"), (None, "timeout")]):
        resp = decide(req, budget_ms=100)
        assert resp["route"] == "legal-fallback"
        assert resp["phases"][1]["status"].startswith("rejected:")
    _, result = child([sys.executable, "-c", "import time; time.sleep(5)"], "", 0.02)
    assert result == "timeout"
    print("PASS worker startup failure, malformed fallback rejection, and actual child timeout", flush=True)

    # Different independent reference paths: Python mechanics against native
    # Rust replay, all nine declarations and all four seats, on legal hands.
    tiles = list(range(28))
    random.Random(9017).shuffle(tiles)
    hands = [sorted(tiles[7*s:7*s+7]) for s in range(4)]
    for decl in (0,1,2,3,4,5,6,7,9):
        remaining = [set(h) for h in hands]
        leader, record, points = 0, [], [0, 0]
        for ti in range(7):
            trick = []
            for pos in range(4):
                seat = (leader+pos)%4
                request = {"decl": decl, "bid": 30, "bidder": 0, "seat": seat,
                           "hand": hands[seat], "plays": record[:], "seed": 420600}
                python_state = information_state(request)
                native, result = child([str(BINARY)], native_text(request, "status"), 1)
                assert native == python_state, (decl, ti, pos, native, python_state, result)
                tile = legal_tiles(remaining[seat], trick, decl)[-1]
                remaining[seat].remove(tile)
                record.extend([seat, tile])
                trick.append((seat, tile))
            leader = winner(trick, decl)
            points[leader%2] += trick_points(trick)
        assert sum(points) == 42
    print("PASS 252 native/Python position comparisons across all declarations and seats", flush=True)


if __name__ == "__main__":
    main()
