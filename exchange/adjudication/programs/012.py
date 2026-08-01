#!/usr/bin/env python3
"""Exact S7 orbit counts for Straight Texas 42 carrier layers.

Standard library only.  No network or file I/O.  The program computes:
  * the pure staircase by the S7 edge cycle index;
  * all pure looped-graph representatives independently;
  * the count-labeled staircase by the correct transported-label
    stabilizer-Burnside decomposition;
  * direct partial-orbit canonicalization for j <= 5 (and j=27,28);
  * the count-labeled role-decorated 4-carrier count directly.
"""

from collections import Counter
from itertools import combinations, permutations
from math import comb, factorial
import sys

N = 7
GROUP_ORDER = factorial(N)
VERTICES = tuple(range(N))
VERTEX_PERMS = list(permutations(VERTICES))

# Full edge order: seven loops first, then the 21 proper edges.
LOOP_EDGES = [(i, i) for i in VERTICES]
PROPER_EDGES = [(i, j) for i in VERTICES for j in range(i + 1, N)]
EDGES = LOOP_EDGES + PROPER_EDGES
EDGE_INDEX = {e: i for i, e in enumerate(EDGES)}
ALL28 = (1 << 28) - 1
LOW28 = ALL28


def edge_image_index(vertex_perm, edge):
    a, b = edge
    x, y = vertex_perm[a], vertex_perm[b]
    if x > y:
        x, y = y, x
    return EDGE_INDEX[(x, y)]


FULL_MAPS = [
    tuple(edge_image_index(p, edge) for edge in EDGES)
    for p in VERTEX_PERMS
]
PROPER_MAPS = [
    tuple(full_map[7 + i] - 7 for i in range(21))
    for full_map in FULL_MAPS
]


def permute_sparse(mask, mapping):
    """Permute a bit mask by an index mapping; fast for sparse masks."""
    out = 0
    while mask:
        bit = mask & -mask
        source = bit.bit_length() - 1
        out |= 1 << mapping[source]
        mask -= bit
    return out


def permute_full_mask(mask, full_map):
    """Permute a 28-edge mask, using its complement when that is sparser."""
    if mask.bit_count() <= 14:
        return permute_sparse(mask, full_map)
    return ALL28 ^ permute_sparse(ALL28 ^ mask, full_map)


# ---------------------------------------------------------------------------
# 1. Pure S7 cycle index on Sym^2({0,...,6}).
# ---------------------------------------------------------------------------

CYCLE_TYPES = [
    (1, 1, 1, 1, 1, 1, 1),
    (2, 1, 1, 1, 1, 1),
    (2, 2, 1, 1, 1),
    (2, 2, 2, 1),
    (3, 1, 1, 1, 1),
    (3, 2, 1, 1),
    (3, 2, 2),
    (3, 3, 1),
    (4, 1, 1, 1),
    (4, 2, 1),
    (4, 3),
    (5, 1, 1),
    (5, 2),
    (6, 1),
    (7,),
]


def representative_permutation(cycle_type):
    p = list(VERTICES)
    start = 0
    for length in cycle_type:
        cycle = list(range(start, start + length))
        for i, v in enumerate(cycle):
            p[v] = cycle[(i + 1) % length]
        start += length
    return tuple(p)


def conjugacy_class_size(cycle_type):
    multiplicities = Counter(cycle_type)
    centralizer = 1
    for length, multiplicity in multiplicities.items():
        centralizer *= (length ** multiplicity) * factorial(multiplicity)
    return GROUP_ORDER // centralizer


def permutation_cycles(mapping):
    seen = [False] * len(mapping)
    cycles = []
    for start in range(len(mapping)):
        if seen[start]:
            continue
        cycle = []
        x = start
        while not seen[x]:
            seen[x] = True
            cycle.append(x)
            x = mapping[x]
        cycles.append(tuple(cycle))
    return cycles


def subset_polynomial(cycle_lengths, degree=28):
    coeff = [0] * (degree + 1)
    coeff[0] = 1
    for length in cycle_lengths:
        for d in range(degree - length, -1, -1):
            if coeff[d]:
                coeff[d + length] += coeff[d]
    return coeff


CYCLE_ROWS = []
PURE_NUMERATOR = [0] * 29
for cycle_type in CYCLE_TYPES:
    p = representative_permutation(cycle_type)
    edge_map = tuple(edge_image_index(p, edge) for edge in EDGES)
    cycles = permutation_cycles(edge_map)
    loop_counter = Counter(
        len(cycle) for cycle in cycles if all(edge_id < 7 for edge_id in cycle)
    )
    proper_counter = Counter(
        len(cycle) for cycle in cycles if all(edge_id >= 7 for edge_id in cycle)
    )
    total_counter = Counter(len(cycle) for cycle in cycles)
    class_size = conjugacy_class_size(cycle_type)
    CYCLE_ROWS.append(
        (cycle_type, class_size, loop_counter, proper_counter, total_counter)
    )
    lengths = []
    for length, multiplicity in total_counter.items():
        lengths.extend([length] * multiplicity)
    fixed_poly = subset_polynomial(lengths)
    for j, value in enumerate(fixed_poly):
        PURE_NUMERATOR[j] += class_size * value

PURE_INTEGRAL = all(value % GROUP_ORDER == 0 for value in PURE_NUMERATOR)
A = [value // GROUP_ORDER for value in PURE_NUMERATOR]


# ---------------------------------------------------------------------------
# 2. Fixed count decoration and its 2520 transported global labelings.
# ---------------------------------------------------------------------------


def edge_mask(edge_list):
    mask = 0
    for a, b in edge_list:
        if a > b:
            a, b = b, a
        mask |= 1 << EDGE_INDEX[(a, b)]
    return mask


FIVE_MASK = edge_mask([(0, 5), (1, 4), (2, 3)])
TEN_MASK = edge_mask([(5, 5), (4, 6)])

TRANSPORTED_LABELINGS = sorted(
    {
        (
            permute_sparse(FIVE_MASK, full_map),
            permute_sparse(TEN_MASK, full_map),
        )
        for full_map in FULL_MAPS
    }
)
TRANSPORTED_LABEL_CODES = [
    five | (ten << 28) for five, ten in TRANSPORTED_LABELINGS
]


def submasks(mask):
    current = mask
    while True:
        yield current
        if current == 0:
            break
        current = (current - 1) & mask


# Every partial label restriction that can occur on some carrier.
ALL_RESTRICTION_CODES = set()
for global_code in TRANSPORTED_LABEL_CODES:
    global_five = global_code & LOW28
    global_ten = global_code >> 28
    for five in submasks(global_five):
        for ten in submasks(global_ten):
            ALL_RESTRICTION_CODES.add(five | (ten << 28))


def invariant_masks_upto(mapping, max_selected):
    """All invariant edge masks of cardinality at most max_selected."""
    cycle_masks = []
    for cycle in permutation_cycles(mapping):
        if len(cycle) > max_selected:
            continue
        mask = 0
        for edge_id in cycle:
            mask |= 1 << edge_id
        cycle_masks.append((mask, len(cycle)))

    states = [(0, 0)]
    for cycle_mask, cycle_size in cycle_masks:
        states += [
            (mask | cycle_mask, size + cycle_size)
            for mask, size in states
            if size + cycle_size <= max_selected
        ]
    return [mask for mask, _ in states]


# For each g in S7, the extendable partial labelings fixed by transported
# action.  A fixed five-mask and ten-mask must each be a union of g-edge
# cycles; they must be disjoint and jointly extendable to a transported c.
FIXED_RESTRICTION_CODES = []
for full_map in FULL_MAPS:
    invariant_five = invariant_masks_upto(full_map, 3)
    invariant_ten = invariant_masks_upto(full_map, 2)
    fixed = set()
    for five in invariant_five:
        for ten in invariant_ten:
            if five & ten:
                continue
            code = five | (ten << 28)
            if code in ALL_RESTRICTION_CODES:
                fixed.add(code)
    FIXED_RESTRICTION_CODES.append(fixed)

GLOBAL_LABEL_STABILIZER_SIZE = sum(
    1
    for full_map in FULL_MAPS
    if permute_sparse(FIVE_MASK, full_map) == FIVE_MASK
    and permute_sparse(TEN_MASK, full_map) == TEN_MASK
)

EDGE_LABEL = [0] * 28
for edge_id in range(28):
    if (FIVE_MASK >> edge_id) & 1:
        EDGE_LABEL[edge_id] = 5
    elif (TEN_MASK >> edge_id) & 1:
        EDGE_LABEL[edge_id] = 10

PRESERVED_SOURCE_MASKS = []
BAD_SOURCE_MASKS = []
for full_map in FULL_MAPS:
    good = 0
    for source, target in enumerate(full_map):
        if EDGE_LABEL[source] == EDGE_LABEL[target]:
            good |= 1 << source
    PRESERVED_SOURCE_MASKS.append(good)
    BAD_SOURCE_MASKS.append(ALL28 ^ good)


def restriction_codes(carrier_mask):
    """Distinct transported count-label restrictions on one pure carrier.

    A code stores the selected five-edges in bits 0..27 and selected ten-edges
    in bits 28..55.  Every other selected carrier edge is label zero.
    """
    selector = carrier_mask | (carrier_mask << 28)
    return {selector & code for code in TRANSPORTED_LABEL_CODES}


def permute_restriction_code(code, full_map):
    five = code & LOW28
    ten = code >> 28
    return permute_sparse(five, full_map) | (
        permute_sparse(ten, full_map) << 28
    )


# ---------------------------------------------------------------------------
# 3. Independent enumeration of every pure looped graph on seven vertices.
#    First quotient 21 proper edges, then quotient 7 loop choices by the
#    proper graph's automorphism group.
# ---------------------------------------------------------------------------

LOOP_TRANSFORM = []
for p in VERTEX_PERMS:
    table = bytearray(128)
    for mask in range(128):
        image = 0
        for v in VERTICES:
            if (mask >> v) & 1:
                image |= 1 << p[v]
        table[mask] = image
    LOOP_TRANSFORM.append(bytes(table))


def enumerate_simple_graph_representatives():
    visited = bytearray(1 << 21)
    representatives = []
    for mask in range(1 << 21):
        if visited[mask]:
            continue
        automorphisms = []
        for perm_index, proper_map in enumerate(PROPER_MAPS):
            image = permute_sparse(mask, proper_map)
            visited[image] = 1
            if image == mask:
                automorphisms.append(perm_index)
        representatives.append((mask, tuple(automorphisms)))
    return representatives


def enumerate_looped_graph_representatives(simple_representatives):
    representatives = []
    for proper_mask, proper_auts in simple_representatives:
        visited_loops = bytearray(128)
        for loop_mask in range(128):
            if visited_loops[loop_mask]:
                continue
            full_auts = []
            for perm_index in proper_auts:
                image = LOOP_TRANSFORM[perm_index][loop_mask]
                visited_loops[image] = 1
                if image == loop_mask:
                    full_auts.append(perm_index)
            carrier_mask = loop_mask | (proper_mask << 7)
            representatives.append((carrier_mask, tuple(full_auts)))
    return representatives


SIMPLE_REPRESENTATIVES = enumerate_simple_graph_representatives()
LOOPED_REPRESENTATIVES = enumerate_looped_graph_representatives(
    SIMPLE_REPRESENTATIVES
)
A_DIRECT_ALL = [0] * 29
for carrier_mask, _ in LOOPED_REPRESENTATIVES:
    A_DIRECT_ALL[carrier_mask.bit_count()] += 1


# ---------------------------------------------------------------------------
# 4. Correct labeled Burnside decomposition.
#
# For one pure representative A, H_A=Stab(A) acts on the finite set L_A of
# distinct restrictions of all transported global labelings.  The number of
# labeled classes above A is |L_A/H_A|, computed explicitly as
#   (1/|H_A|) * sum_{h in H_A} |Fix_{L_A}(h)|.
# FIXED_RESTRICTION_CODES[h] lets us evaluate each fixed-point count exactly.
# ---------------------------------------------------------------------------


def local_orbit_count(codes, automorphisms):
    """Independent quotient count used to cross-check fixed-point Burnside."""
    if len(automorphisms) == 1:
        return len(codes)
    remaining = set(codes)
    orbit_count = 0
    while remaining:
        code = remaining.pop()
        orbit_count += 1
        for perm_index in automorphisms:
            remaining.discard(
                permute_restriction_code(code, FULL_MAPS[perm_index])
            )
    return orbit_count


def explicit_local_burnside(codes, automorphisms):
    # Identity fixes every code.  For nonidentity h, the precomputed fixed set
    # is usually tiny, so membership testing against L_A is fast.
    numerator = len(codes)
    for perm_index in automorphisms:
        if perm_index == 0:
            continue
        numerator += sum(
            1
            for code in FIXED_RESTRICTION_CODES[perm_index]
            if code in codes
        )
    return numerator // len(automorphisms), numerator % len(automorphisms) == 0


B = [0] * 29
LABELED_BURNSIDE_INTEGRAL = True
FOUR_CARRIER_DATA = []
for carrier_mask, automorphisms in LOOPED_REPRESENTATIVES:
    codes = restriction_codes(carrier_mask)
    local_count, integral = explicit_local_burnside(codes, automorphisms)
    B[carrier_mask.bit_count()] += local_count
    LABELED_BURNSIDE_INTEGRAL = LABELED_BURNSIDE_INTEGRAL and integral
    if carrier_mask.bit_count() == 4:
        FOUR_CARRIER_DATA.append((carrier_mask, automorphisms, codes, local_count))


FOUR_CARRIER_BURNSIDE_OK = all(
    local_orbit_count(codes, automorphisms) == burnside_count
    for _, automorphisms, codes, burnside_count in FOUR_CARRIER_DATA
)


# ---------------------------------------------------------------------------
# 5. Direct fixed-slice canonicalization for requested anchor layers.
# ---------------------------------------------------------------------------


def masks_of_size(j):
    for chosen in combinations(range(28), j):
        mask = 0
        for edge_id in chosen:
            mask |= 1 << edge_id
        yield mask


def direct_pure_count(j):
    visited = set()
    classes = 0
    for carrier_mask in masks_of_size(j):
        if carrier_mask in visited:
            continue
        classes += 1
        for full_map in FULL_MAPS:
            visited.add(permute_full_mask(carrier_mask, full_map))
    return classes, len(visited)


def direct_labeled_count(j):
    """Orbit enumeration for the problem's partial label-preserving relation."""
    visited = set()
    classes = 0
    for carrier_mask in masks_of_size(j):
        if carrier_mask in visited:
            continue
        classes += 1
        for perm_index, full_map in enumerate(FULL_MAPS):
            if carrier_mask & BAD_SOURCE_MASKS[perm_index]:
                continue
            visited.add(permute_full_mask(carrier_mask, full_map))
    return classes, len(visited)


DIRECT_JS = tuple(range(6)) + (27, 28)
DIRECT_A = {}
DIRECT_B = {}
DIRECT_EXHAUSTION_OK = True
for j in DIRECT_JS:
    direct_a, visited_a = direct_pure_count(j)
    direct_b, visited_b = direct_labeled_count(j)
    DIRECT_A[j] = direct_a
    DIRECT_B[j] = direct_b
    expected_objects = comb(28, j)
    DIRECT_EXHAUSTION_OK = (
        DIRECT_EXHAUSTION_OK
        and visited_a == expected_objects
        and visited_b == expected_objects
    )


# ---------------------------------------------------------------------------
# 6. Role-decorated count-labeled 4-carriers.
#    led and partner are distinguished; the two opponents are unordered.
# ---------------------------------------------------------------------------


def direct_role_decorated_count():
    visited = set()
    classes = 0
    for led in range(28):
        for partner in range(28):
            if partner == led:
                continue
            remaining_edges = [
                edge_id
                for edge_id in range(28)
                if edge_id != led and edge_id != partner
            ]
            for opponent_1, opponent_2 in combinations(remaining_edges, 2):
                obj = (led, partner, opponent_1, opponent_2)
                if obj in visited:
                    continue
                classes += 1
                carrier_mask = (
                    (1 << led)
                    | (1 << partner)
                    | (1 << opponent_1)
                    | (1 << opponent_2)
                )
                for perm_index, full_map in enumerate(FULL_MAPS):
                    if carrier_mask & BAD_SOURCE_MASKS[perm_index]:
                        continue
                    o1 = full_map[opponent_1]
                    o2 = full_map[opponent_2]
                    if o1 > o2:
                        o1, o2 = o2, o1
                    visited.add((full_map[led], full_map[partner], o1, o2))
    return classes, len(visited)


def local_role_decorated_count():
    total = 0
    for carrier_mask, automorphisms, codes, _ in FOUR_CARRIER_DATA:
        edge_ids = [i for i in range(28) if (carrier_mask >> i) & 1]
        objects = {
            (led, partner, code)
            for led in edge_ids
            for partner in edge_ids
            if partner != led
            for code in codes
        }
        remaining = set(objects)
        orbit_count = 0
        while remaining:
            led, partner, code = remaining.pop()
            orbit_count += 1
            for perm_index in automorphisms:
                full_map = FULL_MAPS[perm_index]
                remaining.discard(
                    (
                        full_map[led],
                        full_map[partner],
                        permute_restriction_code(code, full_map),
                    )
                )
        total += orbit_count
    return total, True


ROLE_DIRECT, ROLE_VISITED = direct_role_decorated_count()
ROLE_LOCAL, ROLE_LOCAL_OK = local_role_decorated_count()


# ---------------------------------------------------------------------------
# 7. Output and checks.
# ---------------------------------------------------------------------------


def cycle_notation(counter):
    pieces = []
    for length in sorted(counter):
        multiplicity = counter[length]
        pieces.append(
            str(length) if multiplicity == 1 else f"{length}^{multiplicity}"
        )
    return " ".join(pieces) if pieces else "-"


def type_notation(cycle_type):
    counts = Counter(cycle_type)
    pieces = []
    for length in sorted(counts, reverse=True):
        multiplicity = counts[length]
        pieces.append(
            str(length) if multiplicity == 1 else f"{length}^{multiplicity}"
        )
    return " ".join(pieces)


checks = []


def check(name, condition):
    condition = bool(condition)
    checks.append(condition)
    print(("PASS " if condition else "FAIL ") + name)


print("INDUCED_EDGE_CYCLE_TABLE")
for cycle_type, class_size, loops, proper, total in CYCLE_ROWS:
    print(
        f"type={type_notation(cycle_type)} class={class_size} "
        f"loops={cycle_notation(loops)} proper={cycle_notation(proper)} "
        f"total={cycle_notation(total)}"
    )

check(
    "15 S7 conjugacy-class sizes sum to 5040",
    sum(r[1] for r in CYCLE_ROWS) == GROUP_ORDER,
)
check("pure cycle-index division is integral", PURE_INTEGRAL)
check(
    "global count-label orbit has 2520 labelings",
    len(TRANSPORTED_LABELINGS) == 2520,
)
check(
    "global count-label stabilizer has size 2",
    GLOBAL_LABEL_STABILIZER_SIZE == 2,
)
check(
    "orbit-stabilizer for global labelings",
    len(TRANSPORTED_LABELINGS) * GLOBAL_LABEL_STABILIZER_SIZE == GROUP_ORDER,
)
check(
    "independent pure representative enumeration matches every a[j]",
    A_DIRECT_ALL == A,
)
check(
    "pure complementation symmetry a[j]=a[28-j]",
    all(A[j] == A[28 - j] for j in range(29)),
)
check(
    "every labeled Burnside quotient is integral",
    LABELED_BURNSIDE_INTEGRAL,
)
check(
    "explicit fixed-point Burnside equals orbit quotient on every pure 4-carrier",
    FOUR_CARRIER_BURNSIDE_OK,
)
check(
    "direct enumerations exhaust every requested layer",
    DIRECT_EXHAUSTION_OK,
)
check(
    "direct a[j] canonicalization agrees for j=0..5,27,28",
    all(DIRECT_A[j] == A[j] for j in DIRECT_JS),
)
check(
    "direct b[j] canonicalization agrees for j=0..5,27,28",
    all(DIRECT_B[j] == B[j] for j in DIRECT_JS),
)
check(
    "direct role enumeration exhausts 28*27*C(26,2) objects",
    ROLE_VISITED == 28 * 27 * comb(26, 2),
)
check(
    "role direct count equals independent stabilizer-Burnside count",
    ROLE_DIRECT == ROLE_LOCAL and ROLE_LOCAL_OK,
)

print("STAIRCASES")
for j in range(29):
    print(f"a[{j}]={A[j]} b[{j}]={B[j]}")
print(f"total_a={sum(A)}")
print(f"total_b={sum(B)}")
print(f"role_decorated_count_labeled_4={ROLE_DIRECT}")
print(f"a4={A[4]} b4={B[4]} b8={B[8]}")

sys.exit(0 if all(checks) else 1)
