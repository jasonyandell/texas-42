---
number: 5
slug: census-integer-audit
conversation: https://chatgpt.com/c/6a66e834-e388-83ea-99e4-3186c6632c50
submitted-at: 2026-07-27T05:10:14.627Z
harvested-at: 2026-07-27T13:03:35.884Z
extraction: copy-button
caveats: none
---
# Audit result

**Every claimed integer reproduced exactly. I found no formula-level error, no evaluation error, and no collision or omission in the displayed counting formulas.**

I treated the attached normative rules and mathematical foundation as the sole source of definitions and targets. The rules establish the 28-tile double-six universe and the nine Straight declarations; the foundation supplies the exact support normal form, ternary validator, certificate construction, and census formulas.    

The strongest independent numeric cross-checks were:

1. **Ternary census regrouping.** Besides summing the 136,514 signatures individually, I fixed only the residual triple (r=(r_0,r_1,r_2)), put (n=\sum r_s), and evaluated
   [
   \sum_{r\in{1,\ldots,7}^3}
   \binom{28}{n}
   F!\left(n;(n-r_0-1,n-r_1-1,n-r_2-1)\right)
   F!\left(28-n;(7-r_0,7-r_1,7-r_2)\right).
   ]
   The first (F) simultaneously counts all valid exclusion histograms and the unrestricted category. This independently gave
   [
   1{,}830{,}955{,}704{,}129{,}296{,}418{,}354{,}864.
   ]

2. **Lead-witness coefficients.** Polynomial convolution was cross-checked against
   [
   B_{n,u}
   =======

   \sum_{\substack{Q\subseteq{1,\ldots,7}\|Q|=u}}
   \sum_{T\subseteq Q}
   (-1)^{|T|}
   \binom{28-\sum_{q\in T}q}
   {n-\sum_{q\in T}q},
   ]
   with out-of-range binomial coefficients zero. The entire (22\times8) table agreed.

3. **Outer certificates.** Every one of the 50 capacity profiles was counted both from the displayed (B_{n,u}) formula and by a direct context-by-context dynamic program tracking pool size, number of used contexts, membership patterns, and whether a qualifying current-context pattern occurred.

4. **Signature and matrix orbits.** Direct canonicalization was checked by Burnside. The fixed-signature counts were (136{,}514) for the identity, (2{,}156) for each transposition, and (35) for each three-cycle, giving
   [
   \frac{136{,}514+3(2{,}156)+2(35)}6=23{,}842.
   ]
   Every allocation-matrix set was independently generated through the three split variables and through a generic row-by-row capacity recursion. Stabilizer orbits were then counted both explicitly and by Burnside.

The final program was executed successfully: **19 PASS lines, exit status 0, 24.53 seconds on one core**, using integer arithmetic only.

## MACHINE-CHECKABLE ARTIFACTS

| Target                                    |                                                                                             Claimed value |                                                                                                  My value | Verdict    | Method(s) used                                                                    |
| ----------------------------------------- | --------------------------------------------------------------------------------------------------------: | --------------------------------------------------------------------------------------------------------: | ---------- | --------------------------------------------------------------------------------- |
| (N_{\mathrm{det}})                        |                                                                                     8,102,258,940,222,814 |                                                                                     8,102,258,940,222,814 | REPRODUCED | Capped multinomial sum; label-by-label four-category DP                           |
| (N_{\mathrm{bin}})                        |                                                                                11,495,078,055,913,018,482 |                                                                                11,495,078,055,913,018,482 | REPRODUCED | Residual-first sum; category-first assignment followed by residual-split counting |
| (N_{\mathrm{ter}})                        |                                                                         1,830,955,704,129,296,418,354,864 |                                                                         1,830,955,704,129,296,418,354,864 | REPRODUCED | Individual signature multinomials; residual-triple generating-function regrouping |
| Grand total                               |                                                                         1,830,967,207,309,611,271,596,161 |                                                                         1,830,967,207,309,611,271,596,161 | REPRODUCED | Exact addition of the four disjoint tags                                          |
| (2^{80}/2^{81}) sandwich                  | 1,208,925,819,614,629,174,706,176 < 1,830,967,207,309,611,271,596,161 < 2,417,851,639,229,258,349,412,352 | 1,208,925,819,614,629,174,706,176 < 1,830,967,207,309,611,271,596,161 < 2,417,851,639,229,258,349,412,352 | REPRODUCED | Exact integer powers and comparisons                                              |
| One-declaration certificate total         |                                                                                         7,124,838,074,989 |                                                                                         7,124,838,074,989 | REPRODUCED | (B_{n,u}) polynomial/IE routes; profile formula/direct context DP                 |
| Nine-declaration certificate total        |                                                                                        64,123,542,674,901 |                                                                                        64,123,542,674,901 | REPRODUCED | Exact multiplication by 9 after declaration-independence check                    |
| Maximum (C(k))                            |                                                                                           839,220,930,919 |                                                                                           839,220,930,919 | REPRODUCED | Exhaustive scan of all 50 profiles by both certificate routes                     |
| Reachable floor                           |                                                                                                44,352,165 |                                                                                                44,352,165 | REPRODUCED | `comb`; independently generated Pascal row; structural disjointness proof         |
| Seat-labeled valid signatures             |                                                                                                   136,514 |                                                                                                   136,514 | REPRODUCED | Direct total enumeration; bounded-composition inclusion–exclusion                 |
| (S_3)-orbits of signatures                |                                                                                                    23,842 |                                                                                                    23,842 | REPRODUCED | Sorted canonical representatives; Burnside fixed-point count                      |
| Labeled allocation matrices               |                                                                                                 1,667,666 |                                                                                                 1,667,666 | REPRODUCED | Split-triple enumeration; generic row/capacity recursion                          |
| Maximum matrices per signature            |                                                                                                       114 |                                                                                                       114 | REPRODUCED | Both complete matrix enumerators                                                  |
| Matrices across canonical representatives |                                                                                                   296,721 |                                                                                                   296,721 | REPRODUCED | Both matrix enumerators over all 23,842 representatives                           |
| Trivial stabilizers                       |                                                                                                    21,686 |                                                                                                    21,686 | REPRODUCED | Pair-multiplicity classification; transposition/three-cycle fixed counts          |
| Order-2 stabilizers                       |                                                                                                     2,121 |                                                                                                     2,121 | REPRODUCED | Pair-multiplicity classification; (2,156-35) fixed-count derivation               |
| Order-6 stabilizers                       |                                                                                                        35 |                                                                                                        35 | REPRODUCED | All-three-pairs-equal enumeration; three-cycle fixed count                        |
| Stabilizer orbits of matrices             |                                                                                                   279,048 |                                                                                                   279,048 | REPRODUCED | Explicit matrix orbits; Burnside per canonical signature                          |
| Maximum matrix orbits per signature       |                                                                                                       103 |                                                                                                       103 | REPRODUCED | Explicit orbit enumeration; Burnside cross-check                                  |

FINAL ANSWER: ALL REPRODUCED

[Download the executed Python audit program](sandbox:/mnt/data/straight42_integer_audit.py)

```python
#!/usr/bin/env python3
"""Independent exact audit of the load-bearing Straight Texas 42 integers.

Standard library only.  No input, files, network, randomness, or floating point.
The computations are derived from the definitions in the audit prompt.
"""

from collections import Counter
from functools import lru_cache
from itertools import combinations, permutations, product
from math import comb
import sys


# ---------------------------------------------------------------------------
# Small exact combinatorial kernels
# ---------------------------------------------------------------------------

def multinomial(total, parts):
    """total! / product(parts!), evaluated as a chain of binomial choices."""
    if sum(parts) != total or any(x < 0 for x in parts):
        return 0
    out = 1
    left = total
    for x in parts:
        out *= comb(left, x)
        left -= x
    return out


@lru_cache(maxsize=None)
def F_sum(R, bounds):
    """Capped four-category assignment count by an explicit multinomial sum."""
    b0, b1, b2 = bounds
    total = 0
    for c0 in range(min(b0, R) + 1):
        for c1 in range(min(b1, R - c0) + 1):
            for c2 in range(min(b2, R - c0 - c1) + 1):
                total += (
                    comb(R, c0)
                    * comb(R - c0, c1)
                    * comb(R - c0 - c1, c2)
                )
    return total


@lru_cache(maxsize=None)
def F_dp(R, bounds):
    """The same count by assigning labeled objects one at a time."""
    b0, b1, b2 = bounds
    dp = {(0, 0, 0): 1}
    for _ in range(R):
        nxt = {}
        for (c0, c1, c2), ways in dp.items():
            # Outside the pool.
            nxt[(c0, c1, c2)] = nxt.get((c0, c1, c2), 0) + ways
            # Certain at one of the three labeled seats.
            if c0 < b0:
                key = (c0 + 1, c1, c2)
                nxt[key] = nxt.get(key, 0) + ways
            if c1 < b1:
                key = (c0, c1 + 1, c2)
                nxt[key] = nxt.get(key, 0) + ways
            if c2 < b2:
                key = (c0, c1, c2 + 1)
                nxt[key] = nxt.get(key, 0) + ways
        dp = nxt
    return sum(dp.values())


def assert_same(label, a, b):
    if a != b:
        raise RuntimeError(f"independent routes disagree for {label}: {a} != {b}")


# ---------------------------------------------------------------------------
# Ternary signatures
# ---------------------------------------------------------------------------

def iter_signatures():
    """Yield every valid seat-labeled native ternary signature exactly once."""
    for r0 in range(1, 8):
        for r1 in range(1, 8):
            for r2 in range(1, 8):
                n = r0 + r1 + r2
                u0 = n - r0 - 1
                u1 = n - r1 - 1
                u2 = n - r2 - 1
                for n0 in range(u0 + 1):
                    for n1 in range(u1 + 1):
                        max_n2 = min(u2, n - n0 - n1)
                        for n2 in range(max_n2 + 1):
                            yield (r0, n0, r1, n1, r2, n2)


def signature_count_inclusion_exclusion():
    """Count signatures as bounded weak compositions, without enumerating n_s."""
    total = 0
    for r0 in range(1, 8):
        for r1 in range(1, 8):
            for r2 in range(1, 8):
                n = r0 + r1 + r2
                uppers = (n - r0 - 1, n - r1 - 1, n - r2 - 1)
                count = 0
                for mask in range(8):
                    shift = 0
                    bits = 0
                    for s in range(3):
                        if mask & (1 << s):
                            shift += uppers[s] + 1
                            bits += 1
                    remain = n - shift
                    term = comb(remain + 3, 3) if remain >= 0 else 0
                    count += -term if bits & 1 else term
                total += count
    return total


def signature_pairs(sig):
    r0, n0, r1, n1, r2, n2 = sig
    return ((r0, n0), (r1, n1), (r2, n2))


# ---------------------------------------------------------------------------
# Full-schema census: two independent routes wherever structurally useful
# ---------------------------------------------------------------------------

def census_values(signatures):
    n_det_a = F_sum(28, (7, 7, 7))
    n_det_b = F_dp(28, (7, 7, 7))
    assert_same("N_det", n_det_a, n_det_b)

    # Binary route A: residuals first, then ambiguity labels and capped certainties.
    n_bin_a = 0
    for inactive in range(3):
        active = [s for s in range(3) if s != inactive]
        for ra in range(1, 8):
            for rb in range(1, 8):
                r = [0, 0, 0]
                r[active[0]] = ra
                r[active[1]] = rb
                n = ra + rb
                n_bin_a += comb(28, n) * F_sum(
                    28 - n, tuple(7 - x for x in r)
                )

    # Binary route B: assign W/K_0/K_1/K_2/outside first, then count residual splits.
    n_bin_b = 0
    for inactive in range(3):
        active = [s for s in range(3) if s != inactive]
        a, b = active
        for n in range(2, 15):
            for c0 in range(8):
                for c1 in range(8):
                    for c2 in range(8):
                        c = (c0, c1, c2)
                        used = n + c0 + c1 + c2
                        if used > 28:
                            continue
                        splits = 0
                        for ra in range(1, 8):
                            rb = n - ra
                            if not 1 <= rb <= 7:
                                continue
                            if c[a] + ra <= 7 and c[b] + rb <= 7:
                                splits += 1
                        if splits:
                            outside = 28 - used
                            n_bin_b += splits * multinomial(
                                28, (n, c0, c1, c2, outside)
                            )
    assert_same("N_bin", n_bin_a, n_bin_b)

    # Ternary route A: sum over each six-integer signature.
    n_ter_a = 0
    for sig in signatures:
        r0, n0, r1, n1, r2, n2 = sig
        n = r0 + r1 + r2
        nstar = n - n0 - n1 - n2
        ambiguity_assignments = comb(28, n) * multinomial(
            n, (n0, n1, n2, nstar)
        )
        n_ter_a += ambiguity_assignments * F_sum(
            28 - n, (7 - r0, 7 - r1, 7 - r2)
        )

    # Ternary route B: for each residual triple, use one capped-category GF for
    # all valid exclusion histograms at once, then another independent label DP
    # for certain/outside labels.
    n_ter_b = 0
    for r0 in range(1, 8):
        for r1 in range(1, 8):
            for r2 in range(1, 8):
                n = r0 + r1 + r2
                exclusion_caps = (n - r0 - 1, n - r1 - 1, n - r2 - 1)
                ambiguity_histograms = F_dp(n, exclusion_caps)
                certain_assignments = F_dp(
                    28 - n, (7 - r0, 7 - r1, 7 - r2)
                )
                n_ter_b += (
                    comb(28, n) * ambiguity_histograms * certain_assignments
                )
    assert_same("N_ter", n_ter_a, n_ter_b)

    grand = 1 + n_det_a + n_bin_a + n_ter_a
    return n_det_a, n_bin_a, n_ter_a, grand


# ---------------------------------------------------------------------------
# Lead-witness polynomial B_{n,u}: convolution and inclusion-exclusion
# ---------------------------------------------------------------------------

def poly_mul(a, b, degree_limit=21):
    out = [0] * (min(degree_limit, len(a) + len(b) - 2) + 1)
    for i, x in enumerate(a):
        if x == 0:
            continue
        for j, y in enumerate(b):
            if i + j > degree_limit:
                break
            if y:
                out[i + j] += x * y
    return out


def B_by_polynomials():
    sizes = tuple(range(1, 8))
    B = [[0] * 8 for _ in range(22)]
    for mask in range(1 << 7):
        u = mask.bit_count()
        poly = [1]
        for i, m in enumerate(sizes):
            top = m - 1 if mask & (1 << i) else m
            factor = [comb(m, k) for k in range(top + 1)]
            poly = poly_mul(poly, factor)
        for n, coeff in enumerate(poly):
            B[n][u] += coeff
    return B


def B_by_inclusion_exclusion():
    sizes = tuple(range(1, 8))
    B = [[0] * 8 for _ in range(22)]
    for mask in range(1 << 7):
        u = mask.bit_count()
        sub = mask
        while True:
            shifted = 0
            for i, m in enumerate(sizes):
                if sub & (1 << i):
                    shifted += m
            sign = -1 if sub.bit_count() & 1 else 1
            for n in range(22):
                k = n - shifted
                if 0 <= k <= 28 - shifted:
                    B[n][u] += sign * comb(28 - shifted, k)
            if sub == 0:
                break
            sub = (sub - 1) & mask
    return B


F_SIZE = {
    frozenset(): 0,
    frozenset({0}): 1,
    frozenset({1}): 0,
    frozenset({2}): 0,
    frozenset({0, 1}): 2,
    frozenset({0, 2}): 1,
    frozenset({1, 2}): 1,
    frozenset({0, 1, 2}): 2,
}


def reachable_profiles():
    return [
        k for k in product(range(8), repeat=3) if max(k) - min(k) <= 1
    ]


def profile_parameters(k):
    h = max(k)
    j = 7 - h
    if k[0] == k[1] == k[2]:
        return sum(k), j, 0
    low = frozenset(s for s, value in enumerate(k) if value == h - 1)
    return sum(k), j, F_SIZE[low]


def C_formula(k, B):
    if k == (0, 0, 0):
        return 1
    n, j, f = profile_parameters(k)
    total = sum((7 ** u) * B[n][u] for u in range(j + 1))
    if f > 0:
        total += (
            7 ** (j + 1) - (8 - 2 ** f) ** (j + 1)
        ) * B[n][j + 1]
    return total


@lru_cache(maxsize=None)
def C_direct_context_dp(n, j, f):
    """Count certificates context-by-context, not through B_{n,u}."""
    # State: (pool size, used-context count, has qualifying current-pattern).
    dp = {(0, 0, False): 1}
    qualifying_patterns = (2 ** f - 1) if f > 0 else 0
    nonqualifying_patterns = 7 - qualifying_patterns
    for m in range(1, 8):
        nxt = {}
        for (pool, used, good), ways in dp.items():
            # Context unused: every subset of its m lead-fiber tiles is allowed.
            for x in range(m + 1):
                if pool + x <= n:
                    key = (pool + x, used, good)
                    nxt[key] = nxt.get(key, 0) + ways * comb(m, x)

            # Context used: all m tiles may not lie in the hidden pool.
            for x in range(m):
                if pool + x > n:
                    continue
                choose_tiles = comb(m, x)
                if nonqualifying_patterns:
                    key = (pool + x, used + 1, good)
                    nxt[key] = (
                        nxt.get(key, 0)
                        + ways * choose_tiles * nonqualifying_patterns
                    )
                if qualifying_patterns:
                    key = (pool + x, used + 1, True)
                    nxt[key] = (
                        nxt.get(key, 0)
                        + ways * choose_tiles * qualifying_patterns
                    )
        dp = nxt

    total = 0
    for (pool, used, good), ways in dp.items():
        if pool != n:
            continue
        if used <= j or (used == j + 1 and good):
            total += ways
    return total


def outer_certificate_values(B):
    profiles = reachable_profiles()
    if len(profiles) != 50:
        raise RuntimeError(f"expected 50 reachable profiles, got {len(profiles)}")

    values = {}
    for k in profiles:
        a = C_formula(k, B)
        if k == (0, 0, 0):
            b = 1
        else:
            n, j, f = profile_parameters(k)
            b = C_direct_context_dp(n, j, f)
        assert_same(f"C{k}", a, b)
        values[k] = a

    one_declaration = sum(values.values())
    nine_declarations = 9 * one_declaration
    maximum = max(values.values())
    return one_declaration, nine_declarations, maximum


# ---------------------------------------------------------------------------
# Reachable no-void floor: binomial and Pascal routes
# ---------------------------------------------------------------------------

def pascal_row(n):
    row = [1]
    for _ in range(n):
        nxt = [1] * (len(row) + 1)
        for i in range(1, len(row)):
            nxt[i] = row[i - 1] + row[i]
        row = nxt
    return row


def reachable_floor_values():
    a = comb(28, 21) + 3 * comb(28, 20) + 3 * comb(28, 19) + comb(28, 18)
    row = pascal_row(28)
    b = row[21] + 3 * row[20] + 3 * row[19] + row[18]
    assert_same("reachable_floor", a, b)
    return a


# ---------------------------------------------------------------------------
# Allocation matrices and S_3 actions
# ---------------------------------------------------------------------------

PERMS = tuple(permutations(range(3)))


def split_matrices(pairs):
    """Enumerate matrices through the three split variables x_0,x_1,x_2."""
    (r0, n0), (r1, n1), (r2, n2) = pairs
    nstar = r0 + r1 + r2 - n0 - n1 - n2
    out = []
    for x0 in range(n0 + 1):
        for x1 in range(n1 + 1):
            for x2 in range(n2 + 1):
                y0 = r0 - x1 - x2
                y1 = r1 - x0 - (n2 - x2)
                y2 = r2 - (n0 - x0) - (n1 - x1)
                if min(y0, y1, y2) < 0:
                    continue
                if y0 + y1 + y2 != nstar:
                    continue
                out.append(
                    (
                        0, x0, n0 - x0,
                        x1, 0, n1 - x1,
                        x2, n2 - x2, 0,
                        y0, y1, y2,
                    )
                )
    return out


def bounded_row_allocations(total, allowed, remaining):
    """Generic bounded weak compositions into an arbitrary allowed-seat set."""
    allowed = tuple(allowed)
    alloc = [0, 0, 0]

    def rec(pos, left):
        if pos == len(allowed):
            if left == 0:
                yield tuple(alloc)
            return
        seat = allowed[pos]
        cap = min(left, remaining[seat])
        for x in range(cap + 1):
            alloc[seat] = x
            yield from rec(pos + 1, left - x)
        alloc[seat] = 0

    yield from rec(0, total)


def generic_matrices(pairs):
    """Enumerate the same matrices by a generic row/capacity recursion."""
    sizes = (pairs[0][1], pairs[1][1], pairs[2][1])
    r = (pairs[0][0], pairs[1][0], pairs[2][0])
    nstar = sum(r) - sum(sizes)
    row_sizes = sizes + (nstar,)
    allowed_rows = ((1, 2), (0, 2), (0, 1), (0, 1, 2))
    out = []

    def rec(row, remaining, flat):
        if row == 4:
            if remaining == (0, 0, 0):
                out.append(tuple(flat))
            return
        for alloc in bounded_row_allocations(
            row_sizes[row], allowed_rows[row], remaining
        ):
            new_remaining = tuple(remaining[s] - alloc[s] for s in range(3))
            rec(row + 1, new_remaining, flat + list(alloc))

    rec(0, r, [])
    return out


def stabilizer(pairs):
    return tuple(
        p for p in PERMS if all(pairs[p[s]] == pairs[s] for s in range(3))
    )


def permute_matrix(matrix, p):
    """Simultaneously relabel seat columns and their excluded-category rows."""
    new = [0] * 12
    for category in range(3):
        for seat in range(3):
            new[3 * p[category] + p[seat]] = matrix[3 * category + seat]
    for seat in range(3):
        new[9 + p[seat]] = matrix[9 + seat]
    return tuple(new)


def orbit_count_explicit(matrices, group):
    unseen = set(matrices)
    count = 0
    sizes = set()
    while unseen:
        matrix = next(iter(unseen))
        orbit = {permute_matrix(matrix, p) for p in group}
        if not orbit <= unseen | (set(matrices) - unseen):
            raise RuntimeError("group image left the matrix set")
        unseen.difference_update(orbit)
        count += 1
        sizes.add(len(orbit))
    return count, sizes


def orbit_count_burnside(matrices, group):
    fixed_sum = 0
    for p in group:
        fixed_sum += sum(
            1 for matrix in matrices if permute_matrix(matrix, p) == matrix
        )
    if fixed_sum % len(group):
        raise RuntimeError("Burnside fixed-point sum is not divisible by group order")
    return fixed_sum // len(group)


def signature_and_matrix_values(signatures):
    labeled_signature_count = len(signatures)

    # Signature orbits by canonicalization.
    labeled_pairs = [signature_pairs(sig) for sig in signatures]
    canonical = sorted({tuple(sorted(pairs)) for pairs in labeled_pairs})
    canonical_count = len(canonical)

    # Independent Burnside calculation for the same S_3 orbit count.
    fixed_by_perm = {}
    for p in PERMS:
        fixed_by_perm[p] = sum(
            1
            for pairs in labeled_pairs
            if all(pairs[p[s]] == pairs[s] for s in range(3))
        )
    burnside_signature_orbits = sum(fixed_by_perm.values()) // 6
    assert_same("signature_orbits", canonical_count, burnside_signature_orbits)

    # Direct and generic matrix enumerators are compared for every labeled signature.
    labeled_matrix_total = 0
    max_labeled_matrices = 0
    for pairs in labeled_pairs:
        a = split_matrices(pairs)
        b = generic_matrices(pairs)
        if set(a) != set(b):
            raise RuntimeError(f"matrix routes disagree for labeled signature {pairs}")
        count = len(a)
        labeled_matrix_total += count
        max_labeled_matrices = max(max_labeled_matrices, count)

    # Canonical representatives, stabilizer classes, and matrix orbits.
    canonical_matrix_total = 0
    stabilizer_counts = Counter()
    matrix_orbit_total = 0
    max_matrix_orbits = 0
    observed_orbit_sizes = set()

    for pairs in canonical:
        matrices_a = split_matrices(pairs)
        matrices_b = generic_matrices(pairs)
        if set(matrices_a) != set(matrices_b):
            raise RuntimeError(f"matrix routes disagree for canonical signature {pairs}")
        canonical_matrix_total += len(matrices_a)

        group = stabilizer(pairs)
        stabilizer_counts[len(group)] += 1

        explicit, sizes = orbit_count_explicit(matrices_a, group)
        burnside = orbit_count_burnside(matrices_a, group)
        assert_same(f"matrix orbit count for {pairs}", explicit, burnside)
        matrix_orbit_total += explicit
        max_matrix_orbits = max(max_matrix_orbits, explicit)
        observed_orbit_sizes.update(sizes)

    if observed_orbit_sizes != {1, 2, 3, 6}:
        raise RuntimeError(f"unexpected matrix orbit sizes: {observed_orbit_sizes}")

    # A second derivation of the stabilizer split from fixed signatures.
    identity = (0, 1, 2)
    transposition = (1, 0, 2)
    three_cycle = (1, 2, 0)
    fixed_transposition = fixed_by_perm[transposition]
    fixed_three_cycle = fixed_by_perm[three_cycle]
    order6_b = fixed_three_cycle
    order2_b = fixed_transposition - fixed_three_cycle
    trivial_b = canonical_count - order2_b - order6_b
    assert_same("trivial stabilizers", stabilizer_counts[1], trivial_b)
    assert_same("order-2 stabilizers", stabilizer_counts[2], order2_b)
    assert_same("order-6 stabilizers", stabilizer_counts[6], order6_b)
    if fixed_by_perm[identity] != labeled_signature_count:
        raise RuntimeError("identity fixed-point count mismatch")

    return (
        labeled_signature_count,
        canonical_count,
        labeled_matrix_total,
        max_labeled_matrices,
        canonical_matrix_total,
        stabilizer_counts[1],
        stabilizer_counts[2],
        stabilizer_counts[6],
        matrix_orbit_total,
        max_matrix_orbits,
    )


# ---------------------------------------------------------------------------
# Main audit and required one-line-per-target report
# ---------------------------------------------------------------------------

EXPECTED = {
    "N_det": 8102258940222814,
    "N_bin": 11495078055913018482,
    "N_ter": 1830955704129296418354864,
    "grand_total": 1830967207309611271596161,
    "sandwich_2p80_2p81": 1830967207309611271596161,
    "outer_one_declaration": 7124838074989,
    "outer_nine_declarations": 64123542674901,
    "outer_max_profile": 839220930919,
    "reachable_floor": 44352165,
    "labeled_signatures": 136514,
    "signature_orbits": 23842,
    "labeled_matrices": 1667666,
    "max_matrices_per_signature": 114,
    "canonical_matrices": 296721,
    "stabilizer_trivial": 21686,
    "stabilizer_order2": 2121,
    "stabilizer_order6": 35,
    "matrix_stabilizer_orbits": 279048,
    "max_matrix_orbits_per_signature": 103,
}


def main():
    signatures = list(iter_signatures())
    sig_ie = signature_count_inclusion_exclusion()
    assert_same("labeled signature count", len(signatures), sig_ie)

    n_det, n_bin, n_ter, grand = census_values(signatures)

    B_poly = B_by_polynomials()
    B_ie = B_by_inclusion_exclusion()
    assert_same("entire B[n,u] table", B_poly, B_ie)
    outer_one, outer_nine, outer_max = outer_certificate_values(B_poly)

    floor = reachable_floor_values()

    (
        labeled_signatures,
        signature_orbits,
        labeled_matrices,
        max_matrices,
        canonical_matrices,
        stabilizer_trivial,
        stabilizer_order2,
        stabilizer_order6,
        matrix_orbits,
        max_matrix_orbits,
    ) = signature_and_matrix_values(signatures)

    computed = {
        "N_det": n_det,
        "N_bin": n_bin,
        "N_ter": n_ter,
        "grand_total": grand,
        "sandwich_2p80_2p81": grand,
        "outer_one_declaration": outer_one,
        "outer_nine_declarations": outer_nine,
        "outer_max_profile": outer_max,
        "reachable_floor": floor,
        "labeled_signatures": labeled_signatures,
        "signature_orbits": signature_orbits,
        "labeled_matrices": labeled_matrices,
        "max_matrices_per_signature": max_matrices,
        "canonical_matrices": canonical_matrices,
        "stabilizer_trivial": stabilizer_trivial,
        "stabilizer_order2": stabilizer_order2,
        "stabilizer_order6": stabilizer_order6,
        "matrix_stabilizer_orbits": matrix_orbits,
        "max_matrix_orbits_per_signature": max_matrix_orbits,
    }

    all_pass = True
    for name, claimed in EXPECTED.items():
        value = computed[name]
        if name == "sandwich_2p80_2p81":
            ok = (1 << 80) < value < (1 << 81) and value == claimed
            if ok:
                print(
                    f"PASS {name} {(1 << 80)}<{value}<{(1 << 81)}"
                )
            else:
                print(
                    f"FAIL {name} claimed={(1 << 80)}<{claimed}<{(1 << 81)} "
                    f"computed={value}"
                )
        else:
            ok = value == claimed
            if ok:
                print(f"PASS {name} {value}")
            else:
                print(f"FAIL {name} claimed={claimed} computed={value}")
        all_pass &= ok

    return 0 if all_pass else 1


if __name__ == "__main__":
    sys.exit(main())
```

### Level 1 formula-vs-structure audit

1. **[FORMULA OK: (F) and (N_{\mathrm{det}})]**

   1. **[USES: determinate normal-form definition]** When (W=\varnothing), every physical domino lies in exactly one of (K_0,K_1,K_2), or outside the hidden pool.
   2. **[USES: native capacity bound]** Since (k_s=|K_s|\le7), the certain-category sizes satisfy (0\le c_s\le7).
   3. **[USES: labeled domino identities]** For fixed sizes ((c_0,c_1,c_2)), the number of assignments to (K_0,K_1,K_2,\mathrm{outside}) is
      [
      \frac{28!}{c_0!c_1!c_2!(28-c_0-c_1-c_2)!}.
      ]
   4. **[USES: disjoint categories]** Every determinate normal form yields one such assignment and vice versa, so (F(28;(7,7,7))) is bijective, not merely an upper bound.
   5. **[USES: independent evaluation]** Both the direct capped multinomial sum and a label-by-label DP give
      [
      N_{\mathrm{det}}=8{,}102{,}258{,}940{,}222{,}814.
      ]

2. **[FORMULA OK: (N_{\mathrm{bin}})]**

   1. **[USES: active-seat trichotomy]** A binary component has one uniquely determined inactive labeled seat (\iota) and two active labeled seats.
   2. **[USES: binary reduced form]** Every (d\in W) has exactly those two active seats as marginal holders. The residuals satisfy
      [
      r_a,r_b\in{1,\ldots,7},\qquad |W|=r_a+r_b.
      ]
      Thus (1\le r_a<|W|) is automatic.
   3. **[USES: labeled ambiguity pool]** (\binom{28}{|W|}) chooses the actual set (W); it does not order its elements.
   4. **[USES: residual capacity accounting]** After (W) is chosen, the remaining labels are assigned to (K_0,K_1,K_2), or outside, with
      [
      |K_s|\le7-r_s.
      ]
      This is exactly the displayed (F(28-|W|;(7-r_0,7-r_1,7-r_2))).
   5. **[USES: marginal support of an unrestricted binary pool]** Because both residuals are positive, every (d\in W) can appear at either active seat: choose a residual-sized subset containing (d), or one omitting it. Hence each counted object is genuinely reduced.
   6. **[USES: labeled capacities and exact-fiber recovery]** The same (W) with a different split (r_a) has different hand cardinalities in its worlds and therefore a different fiber. It is not a duplicate. Likewise, different inactive seats are distinct labeled support states.
   7. **[USES: independent regrouping]** Counting label categories first and then counting admissible residual splits reproduces the residual-first formula exactly:
      [
      N_{\mathrm{bin}}=11{,}495{,}078{,}055{,}913{,}018{,}482.
      ]

3. **[FORMULA OK: ternary signature validity theorem]**

   1. **[USES: category conservation]** The four ambiguity categories partition (W), so
      [
      n_\star=n-n_0-n_1-n_2\ge0
      ]
      is necessary and exactly expresses the existence of the unrestricted category.
   2. **[USES: singleton neighbor sets]** Seat (s) is adjacent to every ambiguity tile except the (n_s) tiles in (W_s). Hence
      [
      |N({s})|=n-n_s.
      ]
   3. **[USES: reduced ambiguity and strict Hall]** If (n-n_s=r_s), every neighbor of seat (s) must be assigned to (s) in every world. Any locally displayed holder edge from one of those tiles to another seat would then be unsupported. A reduced ternary component therefore requires
      [
      n-n_s\ge r_s+1.
      ]
   4. **[USES: at-most-one-exclusion structure]** Any two-seat subset is adjacent to all of (W): no tile excludes two seats. Its quota is (n-r_t\le n-1), since the omitted seat has positive residual capacity.
   5. **[USES: conservation]** The full three-seat set has neighbor size and quota both equal to (n).
   6. **[USES: Hall's theorem]** The three singleton inequalities, the automatic two-seat inequalities, and full-set equality imply a nonempty matching fiber.
   7. **[USES: forced-edge Hall check]** Force any allowed tile-seat edge, remove that tile, and decrement that seat’s quota. Subsets containing the forced seat lose at most one neighbor while their quota also falls by one; subsets not containing it use the one unit of strict-Hall slack. The successor remains feasible.
   8. **[USES: marginal-support definition]** Therefore every allowed edge occurs in some world, so the candidate is reduced. The displayed criterion is both necessary and sufficient.

4. **[FORMULA OK: (N_{\mathrm{ter}}), branch sum, and 81-bit conclusion]**

   1. **[USES: ternary category definition]** For a fixed signature, the first factor chooses four disjoint labeled sets
      [
      W_0,W_1,W_2,W_\star
      ]
      of sizes (n_0,n_1,n_2,n_\star).
   2. **[USES: unordered sets rather than sequences]** The denominator
      [
      n_0!n_1!n_2!n_\star!
      ]
      removes all internal orderings. Thus
      [
      \frac{28!}{(28-n)!n_0!n_1!n_2!n_\star!}
      ]
      counts sets, not ordered labels.
   3. **[USES: certain/outside partition]** The remaining (28-n) labels are assigned by (F) to (K_0,K_1,K_2), or outside, with the exact bounds (7-r_s).
   4. **[USES: complete normal-form invariant]** Different category sets, residuals, or certain sets decode to different marginal-holder relations or capacities and therefore different fibers. There is no cross-signature double count.
   5. **[USES: tagged trichotomy]** Determinate, binary, and ternary branches are disjoint by active-seat count. The separate `Empty` tag is not the feasible determinate state with all hands empty: the latter has a singleton fiber, whereas `Empty` denotes no worlds.
   6. **[USES: exact integer evaluation]** The branch values sum to
      [
      1{,}830{,}967{,}207{,}309{,}611{,}271{,}596{,}161.
      ]
   7. **[USES: exact powers of two]**
      [
      2^{80}=1{,}208{,}925{,}819{,}614{,}629{,}174{,}706{,}176,
      ]
      [
      2^{81}=2{,}417{,}851{,}639{,}229{,}258{,}349{,}412{,}352.
      ]
      The total lies strictly between them, so a universal fixed-width rank needs and admits exactly 81 bits.

5. **[FORMULA OK: (B_{n,u})]**

   1. **[USES: lead-fiber definition]** The seven lead fibers partition the 28 physical dominoes.
   2. **[USES: ordinary subset generating function]** For a fiber of size (m), ((1+x)^m) records all choices of its tiles lying in (U).
   3. **[USES: lead-witness necessity]** A used context requires at least one lead-capable tile outside (U). The unique forbidden pool choice is the complete (m)-tile fiber, represented by (x^m).
   4. **[USES: exact exclusion]** Thus
      [
      (1+x)^m-x^m
      ]
      removes exactly the bad complete-fiber choice and no other choice.
   5. **[USES: certificate stores the used-context set]** Summing over (u)-element sets (Q) correctly counts pairs ((U,Q)); the same pool with a different used-context set is a different certificate.
   6. **[USES: fiber-size multiset]** The expression is invariant under a bijection of contexts preserving fiber size. Because every declaration has fiber-size multiset ({1,2,3,4,5,6,7}), the resulting (B_{n,u}) is declaration-independent.
   7. **[USES: independent coefficient extraction]** Polynomial convolution and inclusion–exclusion agree for every (0\le n\le21), (0\le u\le7).

6. **[FORMULA OK: (C(k)) and the current-context pattern factor]**

   1. **[USES: completed-trick schedule]** With (j) completed tricks, at most (j) distinct void-producing contexts can be placed wholly in completed tricks.
   2. **[USES: hidden-seat membership patterns]** Each used context has exactly seven possible nonempty subsets of the three hidden seats, giving (7^u) pattern maps for (u) used contexts.
   3. **[USES: current follower set of size (f)]** Exactly
      [
      2^f-1
      ]
      of those seven patterns are nonempty subsets of the already-acted follower set (F(B)).
   4. **[USES: complement counting]** For (j+1) used contexts, the number of pattern maps with at least one qualifying context is
      [
      7^{j+1}
      -------

      # \bigl(7-(2^f-1)\bigr)^{j+1}

      7^{j+1}-(8-2^f)^{j+1}.
      ]
   5. **[USES: existential rather than distinguished current context]** This subtraction counts each complete pattern map once, even when several contexts qualify. It does not incorrectly choose and multiply by a distinguished current context.
   6. **[USES: lead-witness coefficient]** Multiplication by (B_{n,j+1}) simultaneously imposes the pool size and a non-hidden lead witness for every used context.
   7. **[USES: terminal certificate definition]** At ((0,0,0)), the unmodified formula would count many historical void-pattern aliases with the same empty remainder. The explicitly defined canonical terminal certificate collapses those aliases to exactly one, so the stated exception is necessary and correct.
   8. **[USES: capacity-profile classification]** There are eight equal profiles and, for each of seven adjacent value pairs, six nonconstant labeled profiles:
      [
      8+7(2^3-2)=50.
      ]
   9. **[USES: direct context DP]** The formula agrees profile-by-profile with direct enumeration of context use, pool subsets, and membership patterns. The totals and maximum therefore reproduce exactly.

7. **[FORMULA OK: reachable no-void floor and disjointness]**

   1. **[USES: all-full cells]** With no hidden void, (P_s=U) for every hidden seat. The support is exactly all labeled partitions of (U) with the displayed capacity vector.
   2. **[USES: shape ((7,7,7))]** Every 21-tile pool is realized before the first play by giving its seven-tile complement to the viewer.
   3. **[USES: permutations of ((6,7,7))]** The eight-tile complement supplies one arbitrary hidden lead and the viewer’s seven-tile current hand. Leading creates no void condition.
   4. **[USES: permutations of ((6,6,7))]** In any nine-tile complement, either two tiles are doubles or two tiles share a pip. Declare doubles or that pip and use the pair as a hidden lead and successful hidden follow. If the viewer lies between the two low hidden seats, choose one of the other seven complement tiles as the viewer’s legal intervening play—use a follower when one exists and otherwise any slough—and retain the other six.
   5. **[USES: shape ((6,6,6))]** In any ten-tile complement, either at least three tiles are doubles, or with (d\le2) doubles the number of distinct tile-pip incidences is
      [
      2(10-d)+d=20-d\ge18.
      ]
      Some pip therefore occurs on at least three distinct tiles. Declare the corresponding suit and use those three tiles as the consecutive hidden lead/follows.
   6. **[USES: successful-follow update]** The public following tiles themselves witness legality and are removed. No positive follower condition survives in the current hidden remainder, so every current partition of (U) remains possible.
   7. **[USES: support recovery]** A nonempty fiber recovers its labeled capacity vector from hand sizes and its actual pool (U) as the union of the hands in any world. Distinct capacity shapes, labeled permutations, or pools cannot collide.
   8. **[USES: exact binomial arithmetic]**
      [
      \binom{28}{21}=1{,}184{,}040,\quad
      \binom{28}{20}=3{,}108{,}105,
      ]
      [
      \binom{28}{19}=6{,}906{,}900,\quad
      \binom{28}{18}=13{,}123{,}110.
      ]
      Hence the eight disjoint families contain
      [
      44{,}352{,}165>33{,}554{,}432=2^{25}.
      ]

8. **[FORMULA OK: signature (S_3)-quotient and stabilizer split]**

   1. **[USES: eligibility incidence]** (W_\star) is the unique category adjacent to all three seats, so every automorphism fixes it.
   2. **[USES: excluded categories]** Each (W_s) is uniquely identified by its one nonneighbor (s). Therefore a structural relabeling must simultaneously permute seat (s) and category (W_s).
   3. **[USES: converse construction]** Every simultaneous seat/category permutation preserves the incidence structure. The complete relabeling group is exactly (S_3), not a larger group.
   4. **[USES: multiset canonicalization]** Sorting the three pairs ((r_s,n_s)) gives one and only one representative of each orbit.
   5. **[USES: Burnside fixed counts]** Identity, transpositions, and three-cycles fix respectively (136{,}514), (2{,}156), and (35) labeled signatures, yielding (23{,}842) orbits.
   6. **[USES: equality multiplicities]** Three distinct pairs give stabilizer order 1; exactly two equal pairs give order 2; all three equal give order 6. Thus
      [
      35,\qquad 2{,}156-35=2{,}121,\qquad
      23{,}842-2{,}121-35=21{,}686.
      ]

9. **[FORMULA OK: allocation matrices and the 114/296,721/1,667,666 counts]**

   1. **[USES: category assignment definition]** A matrix records, for each eligibility category, how many of its labeled tiles are assigned to each seat.
   2. **[USES: exact constraints]** Row sums enforce category sizes, column sums enforce residual capacities, and (a_{W_s,s}=0) enforces exactly the three forbidden holder relations.
   3. **[USES: positive realization]** Every nonnegative integer matrix satisfying those constraints has at least one labeled realization; partition each category’s distinct tiles according to its row entries. Thus “matrix satisfying the constraints” and “feasible allocation matrix” are equivalent.
   4. **[USES: split parameterization]** Choosing (x_0,x_1,x_2) fixes all three excluded-category rows, after which the column sums uniquely force the unrestricted row. Conversely, the matrix recovers those three split values. This is a bijection.
   5. **[USES: independent row recursion]** A generic row-by-row bounded-composition enumerator produced exactly the same matrix set for every one of the 136,514 labeled signatures.
   6. **[USES: exhaustive integer totals]** The resulting totals are 1,667,666 labeled-signature matrices and 296,721 matrices over canonical representatives, with per-signature maximum 114.

10. **[FORMULA OK: stabilizer action on matrices and the 279,048/103 orbit counts]**

    1. **[USES: simultaneous relabeling]** A stabilizer permutation carries row (W_s) to (W_{p(s)}), seat column (s) to (p(s)), and fixes the unrestricted row as a row type. It therefore maps valid matrices of the representative to valid matrices of the same representative.
    2. **[USES: finite group action]** Matrix orbit sizes divide (|G_\lambda|), where (|G_\lambda|\in{1,2,6}). Hence the only possible sizes are among
       [
       {1,2,3,6}.
       ]
    3. **[USES: explicit action]** All four sizes actually occur.
    4. **[USES: independent orbit counts]** For every canonical signature, explicit orbit construction agreed with
       [
       \frac1{|G_\lambda|}
       \sum_{p\in G_\lambda}|\operatorname{Fix}(p)|
       ]
       from Burnside’s lemma.
    5. **[USES: exhaustive sum]** Summing those independently checked orbit counts gives 279,048, and the largest single-signature orbit count is 103.

