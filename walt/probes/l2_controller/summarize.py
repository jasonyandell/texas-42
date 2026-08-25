#!/usr/bin/env python3
"""Aggregate the targeted field-1 controller probe (stdlib only).

Recomputes every screen from the row records with exact fractions,
re-verifies bar/admissible/slack against the screen records, checks the
stage-4 records' internal consistency, and prints the per-root table and
the aggregates. Exploratory instrument output; cited by nothing above it.
"""

import json
import sys
from collections import Counter, defaultdict
from fractions import Fraction


def load(path):
    by_root = defaultdict(lambda: defaultdict(list))
    order = []
    with open(path, encoding="utf-8") as f:
        for line in f:
            rec = json.loads(line)
            root = rec["root"]
            if root not in order:
                order.append(root)
            by_root[root][rec["kind"]].append(rec)
    return order, by_root


def frac(s):
    return Fraction(s)


def verify_root(root, recs):
    rows = recs["row"]
    screens = recs.get("screen", [])
    if not screens:
        assert recs["refusal"], f"{root}: no screen requires a typed refusal"
        return
    screen = screens[0]
    lower1 = {}
    upper1 = {}
    exposure = {}
    for row in rows:
        a = tuple(row["action"])
        r = frac(row["exposure"]["upper"])
        exposure[a] = r
        lower1[a] = frac(row["lower0"]) - r
        upper1[a] = frac(row["upper0"]) + r
    bar = max(lower1.values())
    assert bar == frac(screen["bar"]), f"{root}: bar mismatch"
    admissible = sorted(a for a in lower1 if upper1[a] >= bar)
    recorded = sorted(tuple(a) for a in screen["admissible"])
    assert admissible == recorded, f"{root}: admissible mismatch"
    assert screen["admitted_count"] == len(recorded)
    for entry in screen["slack"]:
        a, b = tuple(entry["a"]), tuple(entry["b"])
        la = frac(next(r["lower0"] for r in rows if tuple(r["action"]) == a))
        ub = frac(next(r["upper0"] for r in rows if tuple(r["action"]) == b))
        expected = la - ub - exposure[a] - exposure[b]
        assert frac(entry["slack"]) == expected, f"{root}: slack mismatch"
    stage4 = recs["stage4"][0]
    if stage4["route"] == "exact":
        survivors = sorted(tuple(a) for a in stage4["survivors"])
        assert survivors == recorded, f"{root}: stage-4 ran outside the survivors"
        changed = stage4["settled0"] != stage4["selected1"]
        assert changed == stage4["decision_changed"], f"{root}: decision flag"
    if stage4["route"] == "delta-survivors":
        assert stage4["open"] == (stage4["selected1"] is None)
    risk = recs["risk"][0]
    if risk["spent"] is not None:
        assert frac(risk["spent"]) <= Fraction(1, 50), f"{root}: risk overrun"


def tile(t):
    return f"{t[0]}-{t[1]}"


def main(path):
    order, by_root = load(path)
    kinds = Counter()
    stops = Counter()
    phase_micros = Counter()
    print(f"{'root':<18} {'tier':<17} {'result':<26} {'stop':<17} "
          f"{'adm':<5} {'rungs':<10} {'stage4'}")
    for root in order:
        recs = by_root[root]
        verify_root(root, recs)
        meta = recs["root"][0]
        kinds[recs["stage4"][0]["result"]] += 1
        screen = recs.get("screen", [{}])
        stop = screen[0].get("stop", "refused") if screen[0] else "refused"
        stops[stop] += 1
        for p in recs["spend"][0]["phases"]:
            phase_micros[p["phase"]] += p["micros"]
        rungs = sorted({r["exposure"]["rung"] for r in recs.get("row", [])})
        stage4 = recs["stage4"][0]
        if stage4["route"] == "exact":
            s4 = (f"{tile(stage4['settled0'])}->{tile(stage4['selected1'])}"
                  + (" CHANGED" if stage4["decision_changed"] else ""))
        elif stage4["route"] == "delta-survivors":
            s4 = "open" if stage4["open"] else f"settled {tile(stage4['selected1'])}"
        elif stage4["route"] == "delta-singleton":
            s4 = f"singleton {tile(stage4['selected'])}"
        else:
            s4 = "refused"
        adm = (f"{screen[0]['admitted_count']}/{screen[0]['legal_count']}"
               if screen[0] else "-")
        print(f"{root:<18} {meta['tier']:<17} {stage4['result']:<26} "
              f"{stop:<17} {adm:<5} {'/'.join(rungs):<10} {s4}")
    print()
    print("result kinds: " + ", ".join(f"{k}={v}" for k, v in sorted(kinds.items())))
    print("ladder stops: " + ", ".join(f"{k}={v}" for k, v in sorted(stops.items())))
    print("phase micros (wall, contended parallel run):")
    for phase, micros in sorted(phase_micros.items()):
        print(f"  {phase:<24} {micros:>12}")
    print("all screen/stage-4/risk cross-checks passed")


if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else "records.jsonl")
