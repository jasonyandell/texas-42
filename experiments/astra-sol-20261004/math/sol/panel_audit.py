#!/usr/bin/env python3
"""Independently replay final panel and exercise repaired protocol/censor paths."""
import contextlib
from collections import Counter
import io
import json
from pathlib import Path
import random
import shutil
import subprocess
import sys
import tempfile
import time
from types import SimpleNamespace

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tools"))
import arena

panel = ROOT / "results/production-panel"
summary = json.loads((ROOT / "results/production-panel-summary/stdout.log").read_text())
manifest = json.loads((ROOT / "reference/production-phone/manifest.json").read_text())
plan = json.loads((panel / "plan.json").read_text())
attempts = json.loads((panel / "attempts.json").read_text())
identity = lambda g: (g["seed"], g["rotation"], g["role"])
expected = {identity(g): g for g in plan["games"]}
assert len(expected) == len(plan["games"]) == 32
assert {identity(a) for a in attempts} == set(expected)
assert all(a["status"] == "completed" and a["exit_code"] == 0 for a in attempts)
assert arena.sha(arena.PHONE) == manifest["wasm_sha256"]
assert manifest["source_commit"] == "cb1ef3b23072e4c268f31f625f2b61d5facc1929"
rows = [json.loads(p.read_text()) for p in sorted(panel.glob("game-*.json"))]
assert {identity(r) for r in rows} == set(expected)
routes = {False: Counter(), True: Counter()}
budgets = Counter()
checked = 0
for r in rows:
    g = expected[identity(r)]
    assert all(r[k] == g[k] for k in ("seed", "rotation", "role", "decl"))
    assert r["complete"] and len(r["moves"]) == 28
    assert r["candidate_label"] == plan["candidate_label"] == "phone-l1-ablation"
    assert r["candidate_partner"] == plan["candidate_partner"] == False
    assert r["phone_wasm_sha256"] == r["candidate_wasm_sha256"] == manifest["wasm_sha256"]
    assert r["arena_sha256"] == arena.sha(ROOT / "tools/arena.py")
    assert r["adapter_sha256"] == arena.sha(ROOT / "tools/phone_worker.mjs")
    assert r["referee_sha256"] == arena.sha(ROOT.parent / "partnership/rules.py")
    tiles = list(range(28))
    random.Random(r["seed"]).shuffle(tiles)
    base = [sorted(tiles[7*s:7*s+7]) for s in range(4)]
    hands = [base[(s-r["rotation"]) % 4] for s in range(4)]
    assert hands == r["hands"]
    bidder = r["rotation"]
    team = (bidder + (r["role"] == "defending")) % 2
    plays, trick, leader, points = [], [], bidder, [0, 0]
    remaining = [set(h) for h in hands]
    for m in r["moves"]:
        seat = (leader + len(trick)) % 4
        candidate = seat % 2 == team
        assert m["candidate"] == candidate
        # Construct independently instead of using arena.make_request/profile.
        req = dict(decl=r["decl"], bid=30, bidder=bidder, seat=seat,
                   hand=list(hands[seat]), plays=list(plays), seed=7042104)
        opening = not plays and seat == bidder
        call = dict(request=req, worlds=160 if opening else 40,
                    partner=False if opening else not candidate,
                    budget_ms=20000 if opening else 14000)
        assert m["call"] == call
        response = m["response"]
        state = arena.information_state(req)
        legal = arena.legal_tiles(remaining[seat], trick, r["decl"])
        assert type(response["choice"]) is int and response["choice"] in legal
        assert sorted(response["legal"]) == state["legal"] == legal
        assert response["points"] == state["points"] == points
        assert response["leader"] == state["leader"] == leader
        assert not m["timing"]["interrupted"]
        assert response["route"] not in {"legal-fallback", "l1-fallback"}
        routes[candidate][response["route"]] += 1
        budgets[candidate] += call["budget_ms"]
        tile = response["choice"]
        remaining[seat].remove(tile)
        plays.extend([seat, tile]); trick.append((seat, tile))
        if len(trick) == 4:
            leader = arena.winner(trick, r["decl"])
            points[leader % 2] += arena.trick_points(trick)
            trick = []
        checked += 1
    replay_points, _, replay_remaining, tail = arena.replay_record(hands, plays, r["decl"], bidder)
    assert r["points"] == points == replay_points and sum(points) == 42
    assert r["made"] == (points[bidder % 2] >= 30)
    assert not tail and all(not h for h in replay_remaining)

by_key = {identity(r): r for r in rows}
differences = []
for seed in sorted({r["seed"] for r in rows}):
    for rotation in range(4):
        a, b = (by_key[(seed, rotation, role)] for role in ("declaring", "defending"))
        assert a["hands"] == b["hands"] and a["decl"] == b["decl"]
        differences.append(int(a["made"]) - int(b["made"]))
assert differences == [0] * 16
assert summary["games"] == 32 and summary["complete_source_deals"] == 4
assert summary["pair_ties"] == 16 and not summary["censored"]
assert summary["hoeffding_95"] == [-1, 1] and not summary["strength_claim"]
for candidate, label in ((False, "phone"), (True, "candidate")):
    assert summary[label]["routes"] == dict(routes[candidate])
    assert summary[label]["budget_ms"] == budgets[candidate] == 6368000

def protocol_case(kind):
    worker = arena.Worker(arena.PHONE)
    checkpoint = {"choice": 7, "route": "legal-fallback"}
    messages = [{"checkpoint": checkpoint}]
    if kind == "error": messages.append({"error": "synthetic worker error"})
    if kind == "result-error": messages.append({"result": {"error": "fatal result"}})
    source = "import sys\nsys.stdin.readline()\n" + "\n".join(
        "print(" + repr(json.dumps(m)) + ", flush=True)" for m in messages)
    def start():
        worker.proc = subprocess.Popen([sys.executable, "-u", "-c", source],
            stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, bufsize=0)
    worker.start = start
    try:
        try:
            response, timing = worker.call({"budget_ms": 100}, time.monotonic() + 3)
            assert kind != "result-error"
            assert response == checkpoint and timing["interrupted"]
            assert timing["checkpoints"] == 1 and timing["interruption_reason"]
        except RuntimeError as e:
            assert kind == "result-error" and str(e) == "fatal result"
    finally:
        worker.close()

for kind in ("error", "eof", "result-error"):
    protocol_case(kind)

with tempfile.TemporaryDirectory(prefix="sol-panel-censor-") as temp:
    directory = Path(temp)
    for p in panel.glob("*.json"):
        shutil.copyfile(p, directory / p.name)
    (directory / "game-104200-r0-declaring.json").unlink()
    output = io.StringIO()
    with contextlib.redirect_stdout(output):
        arena.summarize(SimpleNamespace(directory=directory))
    censored = json.loads(output.getvalue())
    assert censored["censored"] and censored["complete_source_deals"] == 3
    assert censored["pair_ties"] == 12 and "hoeffding_95" not in censored
    assert censored["uncertainty_refusal"]
    # Entirely absent source deals must remain visible through the prespecified plan.
    local_plan = json.loads((directory / "plan.json").read_text())
    local_plan["games"].extend(dict(seed=999999, rotation=r, role=role, decl=6)
        for r in range(4) for role in ("declaring", "defending"))
    (directory / "plan.json").write_text(json.dumps(local_plan))
    output = io.StringIO()
    with contextlib.redirect_stdout(output):
        arena.summarize(SimpleNamespace(directory=directory))
    absent = json.loads(output.getvalue())
    assert all([999999, r] in absent["missing_pairs"] for r in range(4))
    assert "hoeffding_95" not in absent

print(json.dumps(dict(games=32, independently_replayed_moves=checked,
    complete_source_deals=4, paired_ties=16, source_commit=manifest["source_commit"],
    production_wasm_sha256=manifest["wasm_sha256"], all_hashes_matched=True,
    own_public_request_boundary=True, matched_budget_ms_each=budgets[True],
    routes={"phone": dict(routes[False]), "candidate": dict(routes[True])},
    protocol_error_retains_checkpoint=True, protocol_eof_retains_checkpoint=True,
    result_error_remains_fatal=True, partial_and_wholly_absent_blocks_refuse_interval=True,
    scope="descriptive play-only pilot; no phone latency or strength claim"), indent=2))
