#!/usr/bin/env python3
"""Stdlib-only aggregator for the field-swap screen probe (screen.jsonl).

Recomputes every screen from its baseline and exposure records with exact
fractions, re-verifies the screen records' bar/admissible set, replays the
L2-T2 and exclusion audits from the parity records, and prints the
per-root table. Exploratory instrument output; cited by nothing above it.
"""

import json
import sys
from fractions import Fraction


def frac(s):
    return Fraction(s)


def main(path):
    roots = {}
    for line in open(path, encoding="utf-8"):
        rec = json.loads(line)
        roots.setdefault(rec["root"], []).append(rec)
    for name in sorted(roots):
        recs = roots[name]
        by_kind = {}
        for rec in recs:
            by_kind.setdefault(rec["kind"], []).append(rec)
        root = by_kind["root"][0]
        legal = [tuple(t) for t in root["legal"]]
        baselines = {b["field"]: b for b in by_kind["baseline"]}
        v0 = {tuple(v["action"]): frac(v["value"])
              for v in baselines["sigma0"]["values"]}
        v1 = {tuple(v["action"]): frac(v["value"])
              for v in baselines["sigma1"]["values"]}
        # Per-action rung bounds.
        e2 = {}
        e4 = {}
        e0_fires = {}
        for rec in by_kind["exposure"]:
            if rec.get("action") is None:
                continue
            action = tuple(rec["action"])
            bound = frac(rec["bound"]["upper"])
            if rec["producer"] == "clairvoyant-reach-walk":
                e2[action] = bound
                e0_fires[action] = rec["e0_fires"]
            elif rec["producer"] == "exact-split-reach":
                e4[action] = bound
        assert set(v0) == set(legal) and set(v1) == set(legal)
        assert set(e2) == set(legal) and set(e4) == set(legal)
        # Recompute both screens and check the recorded ones.
        for screen in by_kind["screen"]:
            bounds = {
                a: (Fraction(0) if e0_fires[a] else e2[a]) for a in legal
            } if screen["bounds"] == "cheapest" else e4
            lower1 = {a: v0[a] - bounds[a] for a in legal}
            upper1 = {a: v0[a] + bounds[a] for a in legal}
            bar = max(lower1.values())
            admissible = [a for a in legal if upper1[a] >= bar]
            assert frac(screen["bar"]) == bar, (name, screen["bounds"])
            assert [tuple(t) for t in screen["admissible"]] == admissible
            assert screen["admitted_count"] == len(admissible)
            # Slack recheck.
            for s in screen["slack"]:
                a, b = tuple(s["a"]), tuple(s["b"])
                expected = v0[a] - v0[b] - bounds[a] - bounds[b]
                assert frac(s["slack"]) == expected
        # Parity audit replay: L2-T2 with E4 bounds; exclusion soundness.
        best1 = max(v1.values())
        for a in legal:
            assert abs(v1[a] - v0[a]) <= e4[a], (name, a, "L2-T2")
        parity = by_kind["parity"][0]
        for t in parity["excluded"]:
            assert v1[tuple(t)] < best1, (name, t, "exclusion")
        assert parity["every_excluded_sigma1_nonoptimal"]
        assert parity["l2t2_ok"]
        cost = by_kind["cost"][0]
        print(f"== {name} (fiber {root['fiber']}, legal {len(legal)})")
        for a in sorted(legal):
            fired = "E0" if e0_fires[a] else "  "
            print(
                f"  {a[0]}-{a[1]}: Q0={v0[a]} Q1={v1[a]} "
                f"R_exact(E4)={e4[a]} E2={e2[a]} {fired}"
            )
        for screen in by_kind["screen"]:
            print(
                f"  screen[{screen['bounds']}]: {screen['result']} "
                f"admissible {screen['admitted_count']}/{len(legal)} "
                f"bar={screen['bar']}"
            )
        print(
            f"  cost us: baseline0={cost['micros_baseline_sigma0']} "
            f"rungs={cost['micros_rungs_total']} "
            f"sigma1_all={cost['micros_sigma1_all_actions']}"
        )
    print("summarize: all recomputations agree")


if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else "screen.jsonl")
