#!/usr/bin/env python3
"""Stdlib-only aggregator for the slice-3 cancellation instrument.

Recomputes, with exact fractions, every inequality the records claim:
the cancellation ladder |c| <= r <= d, the pairwise census and the
g = B - H / q = B + H identities (g cross-checked against the frozen
baselines), pair lifts Lambda = c_a - c_b with |Lambda| <= d_a + d_b,
the extended rung ladder R+/- <= R^outcome <= R^exposure = E4, the
frozen-tier and exact-root sandwiches, screen bars/admissible sets from
raw bounds, directional-subset-of-symmetric, winner stability against
exact sigma-1 values, and Stage-4 survivor consistency. Exits nonzero
on any failed check. EXPLORATORY tier: verifies instrument records only.
"""

import json
import sys
from collections import defaultdict
from fractions import Fraction

CHECKS = [0]


def check(cond, msg):
    CHECKS[0] += 1
    if not cond:
        print(f"FAIL: {msg}")
        sys.exit(1)


def frac(s):
    return Fraction(s)


def main(path):
    by_root = defaultdict(lambda: defaultdict(list))
    with open(path, encoding="utf-8") as f:
        for line in f:
            rec = json.loads(line)
            by_root[rec["root"]][rec["kind"]].append(rec)

    for root, kinds in sorted(by_root.items()):
        meta = kinds["root"][0]
        fiber = int(meta["fiber"])
        values = {}
        for b in kinds["baseline"]:
            values[b["field"]] = {
                json.dumps(v["action"]): frac(v["value"]) for v in b["values"]
            }
        v0, v1 = values["sigma0"], values["sigma1"]
        actions = sorted(v0)

        # Ladders: |c| <= r <= d, masses re-derived from counts, zeros nest.
        ladders = {}
        for lad in kinds["ladder"]:
            a = json.dumps(lad["action"])
            d = Fraction(lad["exposed"], lad["worlds"])
            r = Fraction(lad["outcome_changed"], lad["worlds"])
            c = Fraction(lad["c_plus"] - lad["c_minus"], lad["worlds"])
            check(lad["worlds"] == fiber, f"{root} {a}: exact-fiber domain")
            check(d == frac(lad["d"]) and r == frac(lad["r"]) and c == frac(lad["c"]),
                  f"{root} {a}: ladder masses re-derive")
            check(lad["outcome_changed"] == lad["c_plus"] + lad["c_minus"],
                  f"{root} {a}: r counts c+ + c-")
            check(abs(c) <= r <= d, f"{root} {a}: |c| <= r <= d")
            if d == 0:
                check(r == 0, f"{root} {a}: d=0 forces r=0")
            if r == 0:
                check(c == 0, f"{root} {a}: r=0 forces c=0")
            ladders[a] = lad

        # Pairwise: census, identities, g == V(a) - V(b), dominance rule.
        for pw in kinds["pairwise"]:
            a, b = json.dumps(pw["a"]), json.dumps(pw["b"])
            total = pw["benefit"] + pw["hazard"] + pw["both_make"] + pw["both_fail"]
            check(total == fiber, f"{root} {a}|{b}: pairwise census")
            g = Fraction(pw["benefit"] - pw["hazard"], fiber)
            q = Fraction(pw["benefit"] + pw["hazard"], fiber)
            check(g == frac(pw["g"]) and q == frac(pw["q"]),
                  f"{root} {a}|{b}: g/q re-derive")
            vals = values[pw["field"]]
            check(g == vals[a] - vals[b], f"{root} {a}|{b}: g equals value gap")
            want = "Dominated" if pw["hazard"] == 0 and pw["benefit"] > 0 else "Unresolved"
            check(pw["label"] == want, f"{root} {a}|{b}: dominance rule")

        # Pair lifts: Lambda = c_a - c_b, |Lambda| <= d_a + d_b.
        for pl in kinds["pair_lift"]:
            a, b = json.dumps(pl["a"]), json.dumps(pl["b"])
            la, lb = ladders[a], ladders[b]
            lam = frac(la["c"]) - frac(lb["c"])
            check(lam == frac(pl["lambda"]), f"{root} {a}|{b}: Lambda re-derives")
            check(abs(lam) <= frac(la["d"]) + frac(lb["d"]),
                  f"{root} {a}|{b}: |Lambda| <= d_a + d_b")

        # Directional rungs and both sandwiches.
        plus, minus, e4 = {}, {}, {}
        for dr in kinds["directional"]:
            a = json.dumps(dr["action"])
            check(dr["plus_worlds"] <= dr["outcome_worlds"] <= dr["exposure_worlds"],
                  f"{root} {a}: extended rung ladder")
            check(dr["outcome_worlds"] <= dr["plus_worlds"] + dr["minus_worlds"],
                  f"{root} {a}: outcome is the disjoint union")
            plus[a] = Fraction(dr["plus_worlds"], fiber)
            minus[a] = Fraction(dr["minus_worlds"], fiber)
            e4[a] = frac(dr["e4_r"])
            check(Fraction(dr["exposure_worlds"], fiber) == e4[a],
                  f"{root} {a}: exposure equals E4")
        for sw in kinds["sandwich"]:
            a = json.dumps(sw["action"])
            check(v0[a] - minus[a] == frac(sw["low"]), f"{root} {a}: sandwich low")
            check(v0[a] + plus[a] == frac(sw["high"]), f"{root} {a}: sandwich high")
            check(frac(sw["low"]) <= v1[a] <= frac(sw["high"]),
                  f"{root} {a}: frozen-tier sandwich")

        # Screens: bars and admissible sets recomputed from raw bounds.
        exact_root_q0 = {}
        exact_root_q1 = {}
        for er in kinds["exact_root"]:
            a = json.dumps(er["action"])
            exact_root_q0[a] = frac(er["q0"])
            exact_root_q1[a] = frac(er["q1"])
            check(frac(er["sandwich_low"]) <= exact_root_q1[a] <= frac(er["sandwich_high"]),
                  f"{root} {a}: exact-root sandwich")
            check(exact_root_q0[a] >= v0[a] and exact_root_q1[a] >= v1[a],
                  f"{root} {a}: the optimizer dominates the frozen candidate")
        admissible = {}
        for sc in kinds["screen"]:
            key = (sc["bounds"], sc["tier"])
            if sc["tier"] == "exact-frozen-set":
                base = v0
            else:
                base = exact_root_q0
            if sc["bounds"] == "e4":
                lo = {a: base[a] - e4[a] for a in actions}
                hi = {a: base[a] + e4[a] for a in actions}
            else:
                lo = {a: base[a] - minus[a] for a in actions}
                hi = {a: base[a] + plus[a] for a in actions}
            bar = max(lo.values())
            adm = [a for a in actions if hi[a] >= bar]
            check(bar == frac(sc["bar"]), f"{root} {key}: bar re-derives")
            got = sorted(json.dumps(x) for x in sc["admissible"])
            check(sorted(adm) == got, f"{root} {key}: admissible set re-derives")
            admissible[key] = set(adm)
        dir_key = ("directional", "exact-frozen-set")
        e4_key = ("e4", "exact-frozen-set")
        if dir_key in admissible and e4_key in admissible:
            check(admissible[dir_key] <= admissible[e4_key],
                  f"{root}: directional admissible subset of symmetric")

        # Winner stability: slack re-derived; positive slack orders sigma-1.
        for ws in kinds["winner_stability"]:
            for pair in ws["pairs"]:
                a, b = json.dumps(pair["a"]), json.dumps(pair["b"])
                slack = v0[a] - v0[b] - minus[a] - plus[b]
                check(slack == frac(pair["slack"]), f"{root} {a}|{b}: slack re-derives")
                check(pair["winner_stable"] == (slack > 0),
                      f"{root} {a}|{b}: stability flag")
                if slack > 0:
                    check(v1[a] > v1[b], f"{root} {a}|{b}: winner holds under sigma1")

        # Stage 4: survivors are the symmetric admissible set; the selected
        # action is the first sigma-1 argmax (order = legal-set order).
        for s4 in kinds["stage4"]:
            survivors = [json.dumps(x) for x in s4["survivors"]]
            check(set(survivors) == admissible[e4_key], f"{root}: stage-4 survivors")
            best1 = max(v1[a] for a in actions)
            sel = json.dumps(s4["selected1"])
            check(v1[sel] == best1, f"{root}: stage-4 selection is sigma1-optimal")
            check(s4["decision_changed"] == (s4["selected1"] != s4["settled0"]),
                  f"{root}: decision-changed flag")

        # The table.
        print(f"\n== {root} (fiber {fiber})")
        print("action | V0 | V1 | R+ | R- | E4 R_a | Q0 | Q1 | label")
        for a in actions:
            lad = ladders[a]
            print(f"  {a} | {v0[a]} | {v1[a]} | {plus[a]} | {minus[a]} | {e4[a]}"
                  f" | {exact_root_q0.get(a, '-')} | {exact_root_q1.get(a, '-')}"
                  f" | {lad['label']}")
        for sc in kinds["screen"]:
            print(f"  screen[{sc['bounds']},{sc['tier']}]: {sc['result']} "
                  f"{sc['admitted_count']}/{sc['legal_count']}")
        for s4 in kinds["stage4"]:
            print(f"  stage4: survivors={s4['survivors']} settled0={s4['settled0']} "
                  f"selected1={s4['selected1']} result={s4['result']}")
        h84 = [pl for pl in kinds["pair_lift"]
               if pl["a"] == [5, 5] and pl["b"] == [3, 3]]
        for pl in h84:
            print(f"  Lambda(5-5, 3-3) = {pl['lambda']}")

    print(f"\nALL {CHECKS[0]} CHECKS PASS")


if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else "cancel.jsonl")
