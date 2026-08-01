#!/usr/bin/env python3
"""Two further independent routes to b_8.

Route 1 (fibered, but with a different pure-orbit enumeration and a different
local orbit count than the response program): enumerate pure 8-carrier orbit
representatives by direct closure over C(28,8) subsets, take stabilizers, then
count Stab-orbits on the set of restricted transported labelings by explicit
orbit enumeration (NOT a Burnside fixed-point sum).

Route 2 (fully direct): closure of the label-preserving relation over all
C(28,8) subsets.
"""
from itertools import combinations, permutations
import sys

N = 7
V = tuple(range(N))
PERMS = list(permutations(V))
LOOPS = [(i, i) for i in V]
PROPER = [(i, j) for i in V for j in range(i + 1, N)]
EDGES = LOOPS + PROPER
EI = {e: k for k, e in enumerate(EDGES)}


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

FIVE = 0
for e in [(0, 5), (1, 4), (2, 3)]:
    FIVE |= 1 << EI[e]
TEN = 0
for e in [(5, 5), (4, 6)]:
    TEN |= 1 << EI[e]


def permute(mask, m):
    out = 0
    while mask:
        b = mask & -mask
        out |= 1 << m[b.bit_length() - 1]
        mask -= b
    return out


TRANSPORTED = sorted({(permute(FIVE, m), permute(TEN, m)) for m in EMAPS})
print("transported labelings:", len(TRANSPORTED))

J = 8
ALLMASKS = []
for ch in combinations(range(28), J):
    mk = 0
    for e in ch:
        mk |= 1 << e
    ALLMASKS.append(mk)
print("layer size:", len(ALLMASKS))
sys.stdout.flush()

# ---- Route 1 ----
visited = set()
reps = []
for mk in ALLMASKS:
    if mk in visited:
        continue
    stab = []
    for i in range(5040):
        im = permute(mk, EMAPS[i])
        visited.add(im)
        if im == mk:
            stab.append(i)
    reps.append((mk, stab))
print("pure 8-carrier orbits (a_8):", len(reps), "covered:", len(visited))
sys.stdout.flush()

total = 0
for mk, stab in reps:
    codes = {(mk & f, mk & t) for f, t in TRANSPORTED}
    if len(stab) == 1:
        total += len(codes)
        continue
    rem = set(codes)
    while rem:
        f, t = rem.pop()
        total += 1
        for i in stab:
            m = EMAPS[i]
            rem.discard((permute(f, m), permute(t, m)))
print("ROUTE1 b8 =", total)
sys.stdout.flush()

# ---- Route 2: fully direct labeled closure ----
visited2 = set()
count = 0
for mk in ALLMASKS:
    if mk in visited2:
        continue
    count += 1
    for i in range(5040):
        if mk & BAD[i]:
            continue
        visited2.add(permute(mk, EMAPS[i]))
print("ROUTE2 b8 =", count, "covered:", len(visited2))
