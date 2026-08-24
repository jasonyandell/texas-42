#!/usr/bin/env python3
"""Stdlib-only aggregator for the field-swap smoke JSONL.

Recomputes every per-policy tally from the world rows, verifies the
per-row L2-T1 pointwise bound (a nonzero correction only on a split
world), verifies the summary rows against the recomputation, and checks
the fixed-policy correction bound |c| <= E[|C|] <= d exactly (integer
arithmetic on tallies -- no floats anywhere).

Usage: python3 summarize.py fieldswap.jsonl
"""

import json
import sys
from collections import Counter, defaultdict
from fractions import Fraction


def main(paths):
    roots = {}
    worlds = defaultdict(list)
    policies = {}
    for path in paths:
        with open(path, encoding="utf-8") as f:
            for line in f:
                rec = json.loads(line)
                kind = rec["kind"]
                if kind == "root":
                    roots[rec["root"]] = rec
                elif kind == "world":
                    worlds[(rec["root"], rec["policy"])].append(rec)
                elif kind == "policy":
                    policies[(rec["root"], rec["policy"])] = rec
                else:
                    raise SystemExit(f"unknown record kind: {kind}")

    failures = 0
    for key in sorted(policies):
        summary = policies[key]
        rows = worlds[key]
        assert summary["tier"] == "FrozenPolicyExposure", "tier is preserved"
        n = len(rows)
        exposed = sum(1 for r in rows if r["d"] == 1)
        c_plus = sum(1 for r in rows if r["u1"] and not r["u0"])
        c_minus = sum(1 for r in rows if r["u0"] and not r["u1"])
        # L2-T1 pointwise: a correction without a split is impossible.
        for r in rows:
            if r["u0"] != r["u1"] and r["d"] != 1:
                print(f"L2-T1 VIOLATION at {key} index {r['index']}")
                failures += 1
            if (r["split"] is not None) != (r["d"] == 1):
                print(f"split/d mismatch at {key} index {r['index']}")
                failures += 1
        for name, got, want in [
            ("worlds", summary["worlds"], n),
            ("exposed", summary["exposed"], exposed),
            ("c_plus", summary["c_plus"], c_plus),
            ("c_minus", summary["c_minus"], c_minus),
        ]:
            if got != want:
                print(f"summary mismatch at {key}: {name} {got} != {want}")
                failures += 1
        d_hat = Fraction(exposed, n)
        c_hat = Fraction(c_plus - c_minus, n)
        c_abs = Fraction(c_plus + c_minus, n)
        if not (abs(c_hat) <= c_abs <= d_hat):
            print(f"correction bound VIOLATION at {key}")
            failures += 1
        if Fraction(summary["d_hat"]) != d_hat or Fraction(summary["c_hat"]) != c_hat:
            print(f"rational mismatch at {key}")
            failures += 1
        by_trick = Counter(r["split"]["trick"] for r in rows if r["split"])
        by_seat = Counter(r["split"]["seat"] for r in rows if r["split"])
        root = roots[key[0]]
        print(
            f"{key[0]:>14} {key[1]:>10}  domain={root['domain']:<28} "
            f"worlds={n:>5} d={exposed}/{n} c=+{c_plus}/-{c_minus} "
            f"d_hat={d_hat} c_hat={c_hat}"
        )
        if by_trick:
            tricks = " ".join(f"t{t}:{c}" for t, c in sorted(by_trick.items()))
            seats = " ".join(f"s{s}:{c}" for s, c in sorted(by_seat.items()))
            print(f"{'':>26}  first-split by trick: {tricks}; by seat: {seats}")
        else:
            print(f"{'':>26}  fields never split on these worlds")
    if failures:
        raise SystemExit(f"{failures} check(s) FAILED")
    print("all checks pass: tallies, L2-T1 pointwise, |c| <= E[|C|] <= d")


if __name__ == "__main__":
    main(sys.argv[1:] or ["fieldswap.jsonl"])
