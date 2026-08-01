#!/usr/bin/env python3
"""Independent canonical-min recomputation of b_j at layers the response
program never checked directly: j = 6, 7, 23, 24, 25, 26."""
from itertools import combinations, permutations
import sys

N = 7
V = tuple(range(N))
PERMS = list(permutations(V))
LOOPS = [(i, i) for i in V]
PROPER = [(i, j) for i in V for j in range(i + 1, N)]
EDGES = LOOPS + PROPER
EI = {e: k for k, e in enumerate(EDGES)}
ALL28 = (1 << 28) - 1


def emap(p):
    out = []
    for (a, b) in EDGES:
        x, y = p[a], p[b]
        if x > y:
            x, y = y, x
        out.append(EI[(x, y)])
    return tuple(out)


EMAPS = [emap(p) for p in PERMS]
LAB = [0] * 28
for e in [(0, 5), (1, 4), (2, 3)]:
    LAB[EI[e]] = 5
for e in [(5, 5), (4, 6)]:
    LAB[EI[e]] = 10
BAD = []
for m in EMAPS:
    bad = 0
    for s in range(28):
        if LAB[s] != LAB[m[s]]:
            bad |= 1 << s
    BAD.append(bad)


def permute(mask, m):
    out = 0
    while mask:
        b = mask & -mask
        out |= 1 << m[b.bit_length() - 1]
        mask -= b
    return out


def pmask(mask, m):
    if mask.bit_count() <= 14:
        return permute(mask, m)
    return ALL28 ^ permute(ALL28 ^ mask, m)


def labeled_canon(j):
    canon = set()
    for ch in combinations(range(28), j):
        mk = 0
        for e in ch:
            mk |= 1 << e
        best = mk
        for i in range(5040):
            if mk & BAD[i]:
                continue
            im = pmask(mk, EMAPS[i])
            if im < best:
                best = im
        canon.add(best)
    return len(canon)


for j in (26, 25, 24, 6, 23, 7):
    print(f"indep b[{j}] = {labeled_canon(j)}")
    sys.stdout.flush()
