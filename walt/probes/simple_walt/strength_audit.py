"""EXPLORATORY: independent re-derivation of the strength_*.jsonl summaries.
Exact integers/Fractions only. Usage: python3 strength_audit.py FILE.jsonl"""
import json, sys
from collections import Counter
from fractions import Fraction as F
from math import comb, isqrt


def dec(x, p=3):
    x = F(x); q = round(abs(x) * 10 ** p)
    return "%s%d.%0*d" % ("-" if x < 0 else "", q // 10 ** p, p, q % 10 ** p)


def se2(vals):
    n = len(vals); m = F(sum(vals), n)
    var = sum((v - m) ** 2 for v in vals) / (n - 1)
    return F(isqrt(int(4 * var / n * 10 ** 12)), 10 ** 6)


def sign_p(w, l):
    m = w + l
    return min(F(1), F(2 * sum(comb(m, k) for k in range(min(w, l) + 1)), 2 ** m)) if m else F(1)


rows = [json.loads(l) for l in open(sys.argv[1])]
seeds = [r["seed"] for r in rows]
n = len(rows)
print("rows", n, "unique seeds", len(set(seeds)), "range", min(seeds), max(seeds), "contiguous", sorted(seeds) == list(range(min(seeds), min(seeds) + n)))
bad = [r["seed"] for r in rows if r["pair"] != int(r["x_declares_made"]) - int(r["y_declares_made"])]
print("pair != m1-m2:", len(bad))
print("games end when the contract is decided: plays per game multiple of 4:", all(len(r["game1"]) % 4 == 0 and len(r["game2"]) % 4 == 0 for r in rows), "; mean plays", dec(F(sum(len(r["game1"]) + len(r["game2"]) for r in rows), 2 * n), 1))
print("made vs score>=30 consistent (declarer team = seat1 team = team 1, a=state[3]):",
      all(r["x_declares_made"] == (r["game1_scores"][0] >= 30) and r["y_declares_made"] == (r["game2_scores"][0] >= 30) for r in rows))
print("no tile played twice:", all(len({t for _, t in r["game1"]}) == len(r["game1"]) and len({t for _, t in r["game2"]}) == len(r["game2"]) for r in rows))
pairs = [r["pair"] for r in rows]
xw, yw = pairs.count(1), pairs.count(-1)
print("X wins %d Y wins %d ties %d" % (xw, yw, n - xw - yw))
m = F(sum(pairs), n); s = se2(pairs)
print("mean pair (X-Y) %s  2SE %s  interval %s .. %s" % (dec(m), dec(s), dec(m - s), dec(m + s)))
print("per-game win share of X (= 1/2 + pair mean/2): %s +- %s ; interval %s .. %s" % (dec(F(1, 2) + m / 2), dec(s / 2), dec(F(1, 2) + (m - s) / 2), dec(F(1, 2) + (m + s) / 2)))
xm = sum(r["x_declares_made"] for r in rows); ym = sum(r["y_declares_made"] for r in rows)
print("declaring make X %d/%d Y %d/%d ; both made %d ; neither %d ; only X %d ; only Y %d" % (
    xm, n, ym, n, sum(r["x_declares_made"] and r["y_declares_made"] for r in rows),
    sum(not r["x_declares_made"] and not r["y_declares_made"] for r in rows), xw, yw))
print("sign test p (two-sided exact) %s" % dec(sign_p(xw, yw), 4))
# identical-game stats
print("game1 == game2 move records:", sum(r["game1"] == r["game2"] for r in rows))
print("first differing play index (game1 vs game2):", sorted(Counter(next((i for i, (a, b) in enumerate(zip(r["game1"], r["game2"])) if a != b), 28) for r in rows).items()))
print("mean seconds %s" % dec(F(round(sum(r["seconds"] for r in rows) * 100), 100 * n), 1))
