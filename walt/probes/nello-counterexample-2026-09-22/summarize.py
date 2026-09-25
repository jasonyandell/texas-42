#!/usr/bin/env python3
"""Rebuild descriptive totals from retained, completed cases; no new search."""
from collections import Counter
from fractions import Fraction
import json
from pathlib import Path
import statistics

HERE = Path(__file__).resolve().parent


def load(path):
    return json.loads(path.read_text())


def substantive(value):
    """v2 adds free-choice coverage counters; everything else must match v1."""
    if isinstance(value, dict):
        ignored = {"total_ms", "counterexample_phase_ms", "viewer_free_decisions",
                   "fallback_free_decisions"}
        return {k: substantive(v) for k, v in value.items() if k not in ignored}
    if isinstance(value, list):
        return [substantive(v) for v in value]
    return value


def evaluation(case, label):
    return next(e for e in case["evaluations"] if e["label"] == label)


def chosen_row(case, label):
    e = evaluation(case, label)
    return next(r for r in e["examination"] if r["action"] == e["choice"])


def tile(index):
    high = next(h for h in range(7) if index < (h + 1) * (h + 2) // 2)
    return f"{high}-{index - high * (high + 1) // 2}"


def summarize(cases):
    out = {"cases": len(cases), "examination_draws": sum(c["examination_worlds"] for c in cases)}
    for label in ["baseline", "round-3", "random-control"]:
        rows = [chosen_row(c, label) for c in cases]
        out[label] = {k: sum(r[k] for r in rows) for k in ["sets", "worlds", "viewer_free_decisions", "fallback_free_decisions"]}
        out[label]["choices"] = dict(Counter(tile(evaluation(c, label)["choice"]) for c in cases))
    for kind in ["augmented", "random_control", "tie_only"]:
        out["paired_" + kind] = [sum(c["paired_" + kind][i] for c in cases) for i in range(4)]
    out["baseline_ties"] = sum(len(c["baseline_ties"]) > 1 for c in cases)
    out["saturated_baseline_leaders"] = sum(
        min(Fraction(*map(int, p["declarer_make"])) for p in evaluation(c, "baseline")["training"]) == 0
        for c in cases)
    out["changed_root_choices"] = sum(evaluation(c, "baseline")["choice"] != evaluation(c, "round-3")["choice"] for c in cases)
    out["changed_tie_only_choices"] = sum(evaluation(c, "baseline")["choice"] != evaluation(c, "round-3")["tie_only_choice"] for c in cases)
    for key in ["counterexample_phase_ms", "total_ms"]:
        values = [c[key] for c in cases]
        out[key] = {"min": min(values), "median": statistics.median(values), "max": max(values)}
    witnesses = [w for c in cases for a in c["attacks"] for w in a["new_witnesses"]]
    out["retained_witnesses"] = len(witnesses)
    out["target_action_doomed"] = sum(next(d["doomed"] for d in w["doom"] if d["action"] == w["target_action"]) for w in witnesses)
    return out


if __name__ == "__main__":
    cases = []
    for path in sorted((HERE / "panel-v2/cases").glob("*.json")):
        case = load(path)
        assert case["status"] == "completed" and case["witness_count"] == 12
        assert substantive(case) == substantive(load(HERE / "cases" / path.name)), path
        for attack in case["attacks"]:
            assert all(r["fallback_decisions"] == 0 for r in attack["replay_retained_witnesses"])
        cases.append((path.name, case))
    assert len(cases) == 32
    groups = []
    for ply in [6, 9]:
        for ordinary in [40, 160]:
            group = [c for name, c in cases if name.startswith(f"ply{ply}-n{ordinary}-")]
            assert len(group) == 8
            groups.append({"ply": ply, "ordinary": ordinary, **summarize(group)})
    summary = {"schema": "nello-counterexample-summary-v1", "groups": groups,
               "overall": summarize([c for _, c in cases]), "v1_v2_outcome_parity": True,
               "panel_wall_seconds": load(HERE / "panel-v2/panel-run.json")["pool_elapsed_seconds"]}
    (HERE / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
    print(json.dumps(summary, indent=2))
