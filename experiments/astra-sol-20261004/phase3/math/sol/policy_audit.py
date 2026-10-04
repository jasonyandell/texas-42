#!/usr/bin/env python3
"""Exhaustive lawful focal-policy audit of the reviewed phase3 compiler."""
import hashlib
import itertools
import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT))
from compiled_tape import Kernel, make_tape, pick

source = ROOT.parent / "phase2/results/cost-panel-a"
plan = json.loads((source / "plan.json").read_text())
row = next(r for r in plan["rows"] if r["eligible"] and r["ply"] == 16)
req = row["request"]
rotation = 1 if req["bidder"] % 2 == 0 else 0
stored = json.loads((source / ("case-%04d.json" % row["id"])).read_text())["result"]["worlds"]
worlds = [[w[(seat+rotation) % 4] for seat in range(4)] for w in stored[:2]]
kernel = Kernel(req, worlds, row["points"], req["seat"])
records = []
for seed in (790001, 790002):
    tape = make_tape(seed, kernel.n, kernel.plies)
    adaptive = kernel.compile(tape, node_cap=10000, seconds=3)
    active = adaptive.route(tape)
    vector = adaptive.reduce(active, 30)
    reference, _ = kernel.reference(tape, 30)
    assert vector == reference
    nodes = [n for n in adaptive.nodes if n.focal and n.score is None]
    assert len({n.key for n in nodes}) == len(nodes)
    legal = [tuple(n.children) for n in nodes]
    count = 1
    for actions in legal: count *= len(actions)
    assert count <= 20000, "Do not substitute sampling for exhaustive policy enumeration"
    best = {t: -1 for t in vector}
    for actions in itertools.product(*legal):
        policy = {n.key: t for n, t in zip(nodes, actions)}
        outcomes = []
        for w in range(kernel.n):
            state, key, depth = kernel.root, 0, 0
            while depth < kernel.plies:
                possible = kernel.moves(state, w)
                actor = (state.leader + len(state.trick)) % 4
                tile = policy[key] if actor == kernel.focal else pick(tape, w, depth, possible)
                assert tile in possible
                state = kernel.after(state, tile)
                key = key*29 + tile + 1
                depth += 1
            assert not state.trick and sum(state.points) == 42
            outcomes.append(int(kernel.success(state.points, 30)))
        root_action = policy[0]
        best[root_action] = max(best[root_action], sum(outcomes))
    assert best == vector
    try:
        adaptive.route(make_tape(seed+99, kernel.n, kernel.plies))
    except AssertionError:
        pass
    else:
        raise AssertionError("Tape-specific graph reused on incompatible tape")
    records.append(dict(seed=seed, policies=count, vectors=best, nodes=len(adaptive.nodes),
                        full_policy_enumeration_equal=True, incompatible_tape_rejected=True))

universal = kernel.compile(node_cap=50000, seconds=5)
for record in records:
    tape = make_tape(record["seed"], kernel.n, kernel.plies)
    assert universal.reduce(universal.route(tape), 30) == record["vectors"]
print(json.dumps(dict(source_case=row["id"], original_scenarios=kernel.n,
    continuation_plies=kernel.plies, root_legal_actions=kernel.moves(kernel.root, 0),
    compiler_sha256=hashlib.sha256((ROOT / "compiled_tape.py").read_bytes()).hexdigest(),
    records=records, universal_nodes=len(universal.nodes),
    scope="Exhaustive deterministic public-history policies on one frozen late root; no production field, strength or speed conclusion."), indent=2))
