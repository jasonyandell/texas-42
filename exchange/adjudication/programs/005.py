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
