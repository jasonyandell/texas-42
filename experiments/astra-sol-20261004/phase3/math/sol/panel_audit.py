#!/usr/bin/env python3
"""Independent receipt accounting and exact vector replay for phase3 panel."""
import hashlib
import json
from pathlib import Path
import sys
ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT))
from compiled_tape import Kernel, make_tape

panel = ROOT / "results/panel-a"
plan = json.loads((panel / "plan.json").read_text())
summary = json.loads((panel / "summary.json").read_text())
source = ROOT.parent / "phase2/results/cost-panel-a"
assert hashlib.sha256((source / "plan.json").read_bytes()).hexdigest() == plan["source_panel_sha256"]
assert plan["world_count"] == 40 and len(plan["tape_seeds"]) == 4
assert plan["bids"] == list(range(30,43))
records = []
for row in plan["fixtures"]:
    receipt = json.loads((panel / ("case-%04d.json" % row["id"])).read_text())
    assert receipt["status"] == "completed"
    assert receipt["request"] == row["request"]
    rotation = 1 if row["request"]["bidder"] % 2 == 0 else 0
    stored = json.loads((source / ("case-%04d.json" % row["id"])).read_text())["result"]["worlds"]
    worlds = [[w[(seat+rotation)%4] for seat in range(4)] for w in stored]
    assert receipt["worlds"] == worlds
    assert len(receipt["tapes"]) == 4
    k = Kernel(row["request"], worlds, row["points"], row["request"]["seat"])
    universal = k.compile(node_cap=100000, seconds=5)
    assert universal.statistics() == receipt["universal_statistics"]
    for seed, r in zip(plan["tape_seeds"], receipt["tapes"]):
        tape = make_tape(seed, 40, k.plies)
        assert r["seed"] == seed and r["tape"] == [list(x) for x in tape]
        reached = universal.route(tape)
        assert len(reached) == r["adaptive_statistics"]["nodes"]
        assert universal.statistics()["nodes"] >= len(reached)
        for bid in plan["bids"]:
            actual, calls = k.reference(tape, bid)
            expected = {int(t): value for t,value in r["values"][str(bid)].items()}
            assert actual == expected == universal.reduce(reached, bid)
            assert calls == r["reference_calls"][str(bid)]
        full, calls = k.reference(tape, 30, False)
        assert calls == r["full_reference_bid30_calls"]
    records.append(receipt)

pairs = [t for r in records for t in r["tapes"]]
unodes = sum(r["universal_statistics"]["nodes"] for r in records)
ainstances = sum(t["adaptive_statistics"]["nodes"] for t in pairs)
usupport = sum(r["universal_statistics"]["support_incidences"] for r in records)
asupport = sum(t["adaptive_statistics"]["support_incidences"] for t in pairs)
assert summary["planned"] == summary["completed"] == len(records) == 15
assert summary["tapes"] == len(pairs) == 60
assert summary["root_action_vectors_compared"] == 780
assert summary["universal_nodes"] == unodes
ucost = summary["universal_compile_us"] + summary["universal_route_us"] + summary["universal_reduce_13_bids_us"]
acost = summary["adaptive_compile_us"] + summary["adaptive_route_us"] + summary["adaptive_reduce_13_bids_us"]
refcost = summary["reference_13_bids_us"]
print(json.dumps(dict(source_cases=len(records), tapes=len(pairs), vectors_replayed=780,
    all_values_calls_worlds_tapes_statistics_equal=True,
    compiler_sha256=hashlib.sha256((ROOT / "compiled_tape.py").read_bytes()).hexdigest(),
    universal_nodes_once_per_case=unodes, universal_nodes_four_tape_instances=4*unodes,
    adaptive_nodes_across_all_tapes=ainstances,
    universal_support_incidences_once_per_case=usupport,
    adaptive_support_incidences_across_all_tapes=asupport,
    universal_physical_to_adaptive_instance_ratio=4*unodes/ainstances,
    universal_total_us=ucost, adaptive_total_us=acost, recursive_13_bids_us=refcost,
    universal_to_recursive_ratio=ucost/refcost, adaptive_to_recursive_ratio=acost/refcost,
    max_universal_python_object_bytes=summary["universal_max_python_object_bytes"],
    scope="13-bid payoff repricing is real repeated graph reuse; fixed-bid production inference is a different workload. Timing is original retained panel, not this audit."), indent=2))
