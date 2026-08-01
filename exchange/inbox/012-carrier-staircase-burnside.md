---
number: 12
slug: carrier-staircase-burnside
conversation: https://chatgpt.com/c/6a6da294-5c10-83e8-b8b0-3cb643e722da
submitted-at: 2026-08-01T07:39:02.034Z
harvested-at: 2026-08-01T08:34:07.892Z
extraction: copy-button
caveats: none
---
# MACHINE-CHECKABLE ARTIFACTS

**FINAL ANSWER:** (a_4=\mathbf{37}); (b_4=\mathbf{486}); (b_8=\mathbf{126657}); role-decorated count-labeled 4-carrier count (=\mathbf{4767}).

1. **[USES: attachment definitions; the (S_7)-action on (\operatorname{Sym}^2(\mathbb P))]**

   Write

   [
   E=\operatorname{Sym}^2({0,\ldots,6}),\qquad |E|=28.
   ]

   Thus (E) has seven loops and twenty-one proper edges. The fixed count decoration has three 5-edges, two 10-edges, and twenty-three 0-edges, exactly as specified in the governing foundation. 

   For a vertex permutation (g\in S_7), a pure (j)-carrier (A\subseteq E) is fixed exactly when it is a union of cycles of the induced edge permutation. If those induced edge-cycle lengths are (\ell_1,\ldots,\ell_r), its fixed-subset polynomial is

   [
   P_g(x)=\prod_{i=1}^r(1+x^{\ell_i}).
   ]

   Therefore the number of fixed (j)-subsets is ([x^j]P_g(x)).

2. **[USES: cycle arithmetic for unordered pairs; all 15 conjugacy classes of (S_7)]**

   The induced cycles can be derived without enumerating edges individually:

   * The loops over a vertex cycle of length (r) form one edge cycle of length (r).
   * Between two distinct vertex cycles of lengths (r,s), the (rs) connecting edges split into
     [
     \gcd(r,s)
     ]
     cycles, each of length
     [
     \operatorname{lcm}(r,s).
     ]
   * Among proper edges internal to one vertex (r)-cycle:

     * for odd (r), there are ((r-1)/2) edge cycles of length (r);
     * for even (r), there are ((r-2)/2) edge cycles of length (r), plus one cycle of length (r/2) formed by opposite pairs.

   Four requested explicit derivations:

   * **(1^7):** all seven loops and all twenty-one proper edges are fixed, giving (1^{28}).
   * **(2,1^5):** the loops give (1^5,2). Among proper edges, the edge joining the transposed pair and the (\binom52=10) edges among fixed vertices are fixed, while the ten edges from the transposed pair to the five fixed vertices form five 2-cycles. Hence the proper part is (1^{11}2^5), and the total is (1^{16}2^6).
   * **(3,2,1^2):** loops give (1^2,2,3). Proper edges give:

     * one 3-cycle internal to the 3-cycle;
     * one fixed edge internal to the 2-cycle;
     * one 6-cycle between the 3- and 2-cycles;
     * two 3-cycles from the 3-cycle to the fixed vertices;
     * two 2-cycles from the 2-cycle to the fixed vertices;
     * one fixed edge between the two fixed vertices.

     Thus the proper part is (1^2,2^2,3^3,6), and the total is
     [
     1^4,2^3,3^4,6.
     ]
   * **(7):** the loops form one 7-cycle. Proper edges are classified by cyclic distance (1,2,3); each distance class contains seven edges and forms one 7-cycle. Hence the total is (7^4).

   In the table, exponents are cycle multiplicities.

   | Vertex cycle type | Class size | Loop-edge cycles | Proper-edge cycles | All 28 edge cycles |
   | ----------------- | ---------: | ---------------- | ------------------ | ------------------ |
   | (1^7)             |          1 | (1^7)            | (1^{21})           | (1^{28})           |
   | (2,1^5)           |         21 | (1^5,2)          | (1^{11},2^5)       | (1^{16},2^6)       |
   | (2^2,1^3)         |        105 | (1^3,2^2)        | (1^5,2^8)          | (1^8,2^{10})       |
   | (2^3,1)           |        105 | (1,2^3)          | (1^3,2^9)          | (1^4,2^{12})       |
   | (3,1^4)           |         70 | (1^4,3)          | (1^6,3^5)          | (1^{10},3^6)       |
   | (3,2,1^2)         |        420 | (1^2,2,3)        | (1^2,2^2,3^3,6)    | (1^4,2^3,3^4,6)    |
   | (3,2^2)           |        210 | (2^2,3)          | (1^2,2^2,3,6^2)    | (1^2,2^4,3^2,6^2)  |
   | (3^2,1)           |        280 | (1,3^2)          | (3^7)              | (1,3^9)            |
   | (4,1^3)           |        210 | (1^3,4)          | (1^3,2,4^4)        | (1^6,2,4^5)        |
   | (4,2,1)           |        630 | (1,2,4)          | (1,2^2,4^4)        | (1^2,2^3,4^5)      |
   | (4,3)             |        420 | (3,4)            | (2,3,4,12)         | (2,3^2,4^2,12)     |
   | (5,1^2)           |        504 | (1^2,5)          | (1,5^4)            | (1^3,5^5)          |
   | (5,2)             |        504 | (2,5)            | (1,5^2,10)         | (1,2,5^3,10)       |
   | (6,1)             |        840 | (1,6)            | (3,6^3)            | (1,3,6^4)          |
   | (7)               |        720 | (7)              | (7^3)              | (7^4)              |

   The class sizes are

   [
   |C_\tau|=\frac{7!}{\prod_r r^{m_r}m_r!},
   ]

   and sum to (5040).

3. **[USES: Burnside’s lemma; induced edge-cycle table]**

   The pure staircase generating polynomial is

   [
   \begin{aligned}
   A(x)=\frac1{5040}\big[&
   (1+x)^{28}\
   &+21(1+x)^{16}(1+x^2)^6\
   &+105(1+x)^8(1+x^2)^{10}\
   &+105(1+x)^4(1+x^2)^{12}\
   &+70(1+x)^{10}(1+x^3)^6\
   &+420(1+x)^4(1+x^2)^3(1+x^3)^4(1+x^6)\
   &+210(1+x)^2(1+x^2)^4(1+x^3)^2(1+x^6)^2\
   &+280(1+x)(1+x^3)^9\
   &+210(1+x)^6(1+x^2)(1+x^4)^5\
   &+630(1+x)^2(1+x^2)^3(1+x^4)^5\
   &+420(1+x^2)(1+x^3)^2(1+x^4)^2(1+x^{12})\
   &+504(1+x)^3(1+x^5)^5\
   &+504(1+x)(1+x^2)(1+x^5)^3(1+x^{10})\
   &+840(1+x)(1+x^3)(1+x^6)^4\
   &+720(1+x^7)^4
   \big].
   \end{aligned}
   ]

   Expanding,

   [
   \begin{aligned}
   A(x)={}&1+2x+5x^2+14x^3+37x^4+98x^5+252x^6+585x^7\
   &+1239x^8+2396x^9+4135x^{10}+6340x^{11}+8630x^{12}\
   &+10381x^{13}+11034x^{14}+10381x^{15}+8630x^{16}\
   &+6340x^{17}+4135x^{18}+2396x^{19}+1239x^{20}\
   &+585x^{21}+252x^{22}+98x^{23}+37x^{24}+14x^{25}\
   &+5x^{26}+2x^{27}+x^{28}.
   \end{aligned}
   ]

   Hence

   [
   \sum_{j=0}^{28}a_j=A(1)=79{,}264.
   ]

   Complementation is (S_7)-equivariant:

   [
   g(E\setminus A)=E\setminus g(A).
   ]

   It therefore induces an orbit bijection between (j)- and ((28-j))-carriers, proving

   [
   a_j=a_{28-j}.
   ]

4. **[USES: full transported (S_7)-action on labelings; Burnside’s lemma]**

   **Theorem — correct Burnside formulation for count-labeled carriers.**

   Let

   [
   \mathcal C=G\cdot c
   ]

   be the orbit of the complete global count labeling under (G=S_7), where

   [
   (g_*c)(e)=c(g^{-1}e).
   ]

   Define the invariant set

   [
   Y_j=
   \left{
   (A,\lambda):
   |A|=j,\
   \lambda=d|_A\text{ for some }d\in\mathcal C
   \right}.
   ]

   The action is

   [
   g\cdot(A,\lambda)=(gA,g_*\lambda),
   \qquad
   (g_*\lambda)(ge)=\lambda(e).
   ]

   Then the count-labeled classes in the question are in bijection with (Y_j/G). Consequently,

   [
   b_j=\frac1{5040}\sum_{g\in G}|\operatorname{Fix}_{Y_j}(g)|.
   ]

   Equivalently, by conjugacy classes,

   [
   \sum_jb_jx^j
   ============

   \frac1{5040}
   \sum_{\tau\vdash7}|C_\tau|F_\tau(x),
   ]

   where, for any representative (g_\tau),

   [
   F_\tau(x)=
   \sum_{\substack{(A,\lambda)\in Y\
   g_\tau A=A\
   g_{\tau*}\lambda=\lambda}}
   x^{|A|}.
   ]

   A pair ((A,\lambda)) is fixed by (g) exactly when:

   [
   gA=A,
   \qquad
   \lambda(ge)=\lambda(e)\quad(e\in A),
   ]

   and (\lambda) extends to at least one transported global labeling in (\mathcal C). Thus (A) is a union of induced edge cycles, and every selected edge cycle is monochromatic under (\lambda). There is **no** requirement that (g) preserve the original fixed labeling (c) on all 28 edges.

   **Proof.**

   The set (Y_j) is closed under the full (G)-action. Every orbit in (Y_j) meets the fixed-label slice

   [
   X_j={(A,c|_A):|A|=j}.
   ]

   Indeed, if (\lambda=(h_*c)|_A), applying (h^{-1}) produces

   [
   (h^{-1}A,c|_{h^{-1}A}).
   ]

   Two elements ((A,c|_A)) and ((B,c|_B)) of this slice lie in the same (Y_j)-orbit exactly when some (g\in G) satisfies

   [
   gA=B,\qquad c(ge)=c(e)\quad(e\in A),
   ]

   which is precisely the equivalence relation in the question. Burnside on the invariant set (Y_j) therefore counts the desired classes. The displayed fixed-point condition follows directly from the transported action. ∎

   For computation, choose one representative (A) from every pure carrier orbit and put

   [
   H_A=\operatorname{Stab}_G(A),
   \qquad
   L_A={d|_A:d\in\mathcal C}.
   ]

   The labeled classes lying over the pure class of (A) are exactly (L_A/H_A). Therefore the implementable stabilizer-Burnside formula is

   [
   \boxed{
   b_j=
   \sum_{\substack{A\in\mathcal R\|A|=j}}
   \frac1{|H_A|}
   \sum_{h\in H_A}
   \left|
   \left{
   \lambda\in L_A:
   h_*\lambda=\lambda
   \right}
   \right|
   }.
   ]

   Distinct global transported labelings that have the same restriction to (A) are deduplicated: labels outside (A) are not part of the carrier object.

   The global labeling has stabilizer of order two, but that subgroup is used only to deduplicate the (5040/2=2520) transported global labelings. It is **not** the group acting on carriers. The naive subgroup shortcut is visibly wrong already at (j=1): the order-two global stabilizer has (22) edge orbits, whereas the correct singleton count is

   [
   b_1=5
   ]

   — zero loop, zero proper edge, five-point proper edge, ten-point loop, and ten-point proper edge.

5. **[USES: pure cycle index; transported-label Burnside theorem]**

   The complete staircases are:

   |   (j) |     (a_j) |       (b_j) |
   | ----: | --------: | ----------: |
   |     0 |         1 |           1 |
   |     1 |         2 |           5 |
   |     2 |         5 |          22 |
   |     3 |        14 |         104 |
   | **4** |    **37** |     **486** |
   |     5 |        98 |       2,233 |
   |     6 |       252 |       9,657 |
   |     7 |       585 |      37,473 |
   | **8** | **1,239** | **126,657** |
   |     9 |     2,396 |     367,187 |
   |    10 |     4,135 |     906,917 |
   |    11 |     6,340 |   1,905,647 |
   |    12 |     8,630 |   3,410,313 |
   |    13 |    10,381 |   5,207,473 |
   |    14 |    11,034 |   6,795,644 |
   |    15 |    10,381 |   7,585,129 |
   |    16 |     8,630 |   7,239,878 |
   |    17 |     6,340 |   5,901,438 |
   |    18 |     4,135 |   4,097,566 |
   |    19 |     2,396 |   2,413,856 |
   |    20 |     1,239 |   1,199,770 |
   |    21 |       585 |     499,342 |
   |    22 |       252 |     172,266 |
   |    23 |        98 |      48,575 |
   |    24 |        37 |      10,980 |
   |    25 |        14 |       1,932 |
   |    26 |         5 |         252 |
   |    27 |         2 |          22 |
   |    28 |         1 |           1 |

   Totals:

   [
   \boxed{\sum_ja_j=79{,}264},
   \qquad
   \boxed{\sum_jb_j=47{,}940{,}826}.
   ]

   The (b_j) staircase is not complement-symmetric. A label-preserving isomorphism on (A) need not preserve labels on (E\setminus A), so the complement operation does not descend to the count-labeled equivalence classes. For example,

   [
   b_1=5\ne22=b_{27}.
   ]

6. **[USES: direct orbit enumeration; diagonal role action]**

   Direct canonicalization over the vertex permutations independently gives

   [
   (a_0,\ldots,a_5)=(1,2,5,14,37,98)
   ]

   and

   [
   (b_0,\ldots,b_5)=(1,5,22,104,486,2233).
   ]

   The program also directly checks (j=27,28), and independently reconstructs every pure (a_j) by first enumerating the 1,044 proper-edge graph classes and then quotienting the 128 loop masks by each proper graph’s automorphism group.

   For the role decoration, let

   [
   R_A={(\ell,p)\in A^2:\ell\ne p}.
   ]

   Here (\ell) is the led edge, (p) is the partner edge, and the other two edges form the unordered opponent pair. For a pure 4-carrier representative (A), (H_A) acts diagonally on (R_A\times L_A), so an equivalent Burnside expression is

   [
   \sum_{A\in\mathcal R_4}
   \frac1{|H_A|}
   \sum_{h\in H_A}
   |\operatorname{Fix}*{R_A}(h)|
   |\operatorname{Fix}*{L_A}(h)|.
   ]

   Direct enumeration exhausts

   [
   28\cdot27\cdot\binom{26}{2}=245{,}700
   ]

   fixed-slice role-decorated objects and gives

   [
   \boxed{4767}.
   ]

   The program checks this against the independent stabilizer quotient above.

7. **[USES: exact pure staircase; OEIS comparison]**

   The pure coefficient row is the (n=7) graph-with-loops row of OEIS A070166, and its row sum (79{,}264) is the (n=7) term of OEIS A000666. ([OEIS][1])

8. **[USES: Steps 1–7; Python 3 standard library]**

```python
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
```

[1]: https://oeis.org/A070166?utm_source=chatgpt.com "A070166 - OEIS"

