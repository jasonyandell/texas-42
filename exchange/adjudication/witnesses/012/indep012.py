#!/usr/bin/env python3
"""Fully independent recomputation of dispatch-012 quantities.

Deliberately different algorithms from the response program:
  * a_j : cycle index summed over ALL 5040 permutations (no conjugacy-class
          shortcut, no representative-permutation construction).
  * a_4, b_4 : canonical-minimum classification (not visited-closure).
  * b_8 : canonical-minimum classification over all C(28,8) subsets.
  * role : per-carrier allowed-permutation set + canonical minimum.
"""
from itertools import combinations, permutations
from math import comb
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

# ---- labels ----
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

# ---- a_j via full-group cycle index ----
A = [0] * 29
for m in EMAPS:
    seen = [False] * 28
    lens = []
    for s in range(28):
        if seen[s]:
            continue
        L = 0
        x = s
        while not seen[x]:
            seen[x] = True
            L += 1
            x = m[x]
        lens.append(L)
    poly = [0] * 29
    poly[0] = 1
    for L in lens:
        for d in range(28 - L, -1, -1):
            if poly[d]:
                poly[d + L] += poly[d]
    for j, v in enumerate(poly):
        A[j] += v
assert all(v % 5040 == 0 for v in A), "cycle index not integral"
A = [v // 5040 for v in A]
print("indep a[0..28] =", A)
print("indep sum a =", sum(A))
print("indep a4 =", A[4])


def permute(mask, m):
    out = 0
    while mask:
        b = mask & -mask
        out |= 1 << m[b.bit_length() - 1]
        mask -= b
    return out


def masks(j):
    for ch in combinations(range(28), j):
        mk = 0
        for e in ch:
            mk |= 1 << e
        yield mk


def pure_classes_canonical(j):
    canon = set()
    for mk in masks(j):
        best = min(permute(mk, m) for m in EMAPS)
        canon.add(best)
    return len(canon)


def labeled_classes_canonical(j):
    canon = set()
    for mk in masks(j):
        best = mk
        for i in range(5040):
            if mk & BAD[i]:
                continue
            im = permute(mk, EMAPS[i])
            if im < best:
                best = im
        canon.add(best)
    return len(canon)


for j in (0, 1, 2, 3, 4, 5):
    pc = pure_classes_canonical(j)
    lc = labeled_classes_canonical(j)
    print(f"indep j={j}: a={pc} (cycidx {A[j]}) b={lc}")
    assert pc == A[j], (j, pc, A[j])

sys.stdout.flush()

# ---- role decorated, independent ----
role_canon = set()
b4_check = set()
for mk in masks(4):
    allowed = [EMAPS[i] for i in range(5040) if not (mk & BAD[i])]
    ids = [i for i in range(28) if (mk >> i) & 1]
    b4_check.add(min(permute(mk, m) for m in allowed))
    for led in ids:
        for partner in ids:
            if partner == led:
                continue
            opp = [i for i in ids if i != led and i != partner]
            best = None
            for m in allowed:
                o = sorted((m[opp[0]], m[opp[1]]))
                t = (m[led], m[partner], o[0], o[1])
                if best is None or t < best:
                    best = t
            role_canon.add(best)
print("indep b4 (2nd route) =", len(b4_check))
print("indep role_decorated =", len(role_canon))
sys.stdout.flush()

# ---- b_8 direct ----
print("starting b8 ...")
sys.stdout.flush()
canon8 = set()
BADL = BAD
EM = EMAPS
for ch in combinations(range(28), 8):
    mk = 0
    for e in ch:
        mk |= 1 << e
    best = mk
    for i in range(5040):
        if mk & BADL[i]:
            continue
        m = EM[i]
        out = 0
        t = mk
        while t:
            b = t & -t
            out |= 1 << m[b.bit_length() - 1]
            t -= b
        if out < best:
            best = out
    canon8.add(best)
print("indep b8 =", len(canon8))
