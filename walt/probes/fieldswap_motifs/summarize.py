#!/usr/bin/env python3
"""Recompute every identity in motifs.jsonl from the raw counts, with exact
fractions — the instrument view's independent checker (stdlib only).

EXPLORATORY tier: these checks are session evidence about the committed
records, never receipts. Source: x:024 response Part 3 (§§3.1–3.9),
rulings TRIPLE-A6/A7. Every mass here partitions CORRECTION MASS for its
(root, action, policy, field pair) — never field exposure.
"""

import json
import sys
from fractions import Fraction

MOTIFS = [
    "LeadContextFork",
    "ImmediateControlFork",
    "CountCommitmentFork",
    "TrumpCommitmentFork",
    "SuitShapeFork",
    "StrengthCommitmentFork",
    "Other",
]

checks = 0


def check(condition, message):
    global checks
    if not condition:
        raise SystemExit(f"CHECK FAILED: {message}")
    checks += 1


def frac(s):
    return Fraction(s)


def main(path):
    roots = {}
    histograms = []
    residuals = {}
    traces = {}
    with open(path, encoding="utf-8") as f:
        for line in f:
            r = json.loads(line)
            k = r["kind"]
            if k == "root":
                roots[r["root"]] = r
            elif k == "motif_histogram":
                histograms.append(r)
            elif k == "residual":
                residuals[(r["root"], json.dumps(r["action"]))] = r
            elif k == "trace":
                traces.setdefault((r["root"], json.dumps(r["action"])), []).append(r)
            else:
                raise SystemExit(f"unknown record kind {k!r}")

    check(len(roots) == 3, "three declared roots")
    for h in histograms:
        root = roots[h["root"]]
        key = (h["root"], json.dumps(h["action"]))
        worlds = h["worlds"]
        check(worlds == int(root["fiber"]), "the histogram ranges over the exact fiber")
        check(h["domain"] == "exact-fiber", "exact-fiber domain declared")
        check(h["partition"] == "correction-mass", "the partition is correction mass")
        c_plus, c_minus = h["c_plus"], h["c_minus"]
        corrections = h["correction_worlds"]
        check(corrections == c_plus + c_minus, "corrections = c+ + c-")
        by = {m["motif"]: m for m in h["motifs"]}
        check(list(by) == MOTIFS, "all seven labels present, taxonomy order")
        plus = sum(m["plus"] for m in h["motifs"])
        minus = sum(m["minus"] for m in h["motifs"])
        check(plus == c_plus, "sum m_k+ = c+")
        check(minus == c_minus, "sum m_k- = c-")
        net = Fraction(0)
        for m in h["motifs"]:
            r_k = Fraction(m["plus"] + m["minus"], worlds)
            c_k = Fraction(m["plus"] - m["minus"], worlds)
            check(frac(m["r_k"]) == r_k, "r_k re-derives")
            check(frac(m["c_k"]) == c_k, "c_k re-derives")
            net += c_k
            if r_k == 0:
                check(m["tilt"] is None, "tilt undefined at r_k = 0")
            else:
                check(frac(m["tilt"]) == c_k / r_k, "tilt = c_k / r_k")
        check(net == Fraction(c_plus - c_minus, worlds), "sum c_k = c")
        other = by["Other"]["plus"] + by["Other"]["minus"]
        if corrections == 0:
            check(h["residual_fraction"] is None, "no residual fraction without corrections")
        else:
            check(
                frac(h["residual_fraction"]) == Fraction(other, corrections),
                "residual fraction re-derives",
            )
        sign = h["terminal_sign"]
        check(sign["favors_field1"] == c_plus, "favors_field1 = c+")
        check(sign["favors_field0"] == c_minus, "favors_field0 = c-")
        actor = h["split_actor"]
        check(
            actor["partner"] + actor["opponent"] == corrections,
            "every classified trace has an actor relation",
        )
        for name, n in h["flag_counts"].items():
            check(0 <= n <= corrections, f"flag count {name} within range")
        res = residuals[key]
        check(res["other_worlds"] == other, "the residual record matches the Other bucket")
        check(
            sum(p["count"] for p in res["pairs"]) == other,
            "the residual pairs are a census of Other",
        )
        for t in traces.get(key, []):
            check(t["specimen"] is True, "trace records are declared specimens")
            check("motif" not in t, "no motif tag on a trace record (TRIPLE-A7)")
            check(
                t["root_semantics_hash"] == root["root_semantics_hash"],
                "the trace's semantics hash is the root's",
            )
            check(
                len(t["branch0_suffix"]) == len(t["branch1_suffix"]),
                "both branches play out the same number of tiles",
            )
            check(t["u0"] != t["u1"], "a correction trace has a changed outcome")
        check(len(traces.get(key, [])) <= root["specimen_cap"], "specimen cap respected")

    # The aggregate table, one row per (root, action).
    print(f"{'root':<16} {'action':<8} {'corr':>5} " + " ".join(f"{m[:9]:>9}" for m in MOTIFS))
    for h in histograms:
        by = {m["motif"]: m["plus"] + m["minus"] for m in h["motifs"]}
        print(
            f"{h['root']:<16} {json.dumps(h['action']):<8} {h['correction_worlds']:>5} "
            + " ".join(f"{by[m]:>9}" for m in MOTIFS)
        )
    print(f"ALL {checks} CHECKS PASS")


if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else "motifs.jsonl")
