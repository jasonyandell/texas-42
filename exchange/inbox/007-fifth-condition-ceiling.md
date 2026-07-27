---
number: 7
slug: fifth-condition-ceiling
conversation: https://chatgpt.com/c/6a66e7f0-57cc-83ea-b6c8-eab6080b8b76
submitted-at: 2026-07-27T18:15:07.950Z
harvested-at: 2026-07-27T19:01:13.230Z
extraction: copy-button
caveats: none
---
The ceiling moves.

The strongest exactly counted family I can prove from the available outer data has two parts:

1. **Context-capacity supply.** If (M(q)) is the set of hidden seats void in context (q), then all remaining tiles of those seats lie outside (\widehat\sigma_q). Therefore
   [
   \left|\widehat\sigma_q\setminus U\right|
   \ge
   \max!\left(
   1,;
   |\widehat\sigma_q|
   ------------------

   \sum_{s\notin M(q)}k_s
   \right).
   ]
   The first term is the existing lead-witness requirement; the second is the one-context projection of hidden-hand capacity/Hall feasibility. The corpus proves exact follow legality, cell losslessness, Hall feasibility, and the original schedule and lead-witness conditions.  

2. **Exact singleton timing rule.** A singleton hidden void normally forces two distinct public suit tiles: the lead and a successful hidden co-follower. The sole exception is one context acquired in the current partial trick before any nonvoid hidden co-follower has acted. The exact exception-seat table is
   [
   \begin{array}{c|c}
   B&E(B)\\hline
   \varnothing&\varnothing\
   {h_1}&{h_1}\
   {h_2}&\varnothing\
   {h_3}&\varnothing\
   {h_1,h_2}&{h_2}\
   {h_1,h_3}&{h_1}\
   {h_2,h_3}&{h_3}\
   {h_1,h_2,h_3}&\varnothing
   \end{array}
   ]
   and equal capacity profiles have (E=\varnothing). Consequently there may be **at most one** singleton context with only one suit tile outside (U), and its sole voiding seat must lie in (E(B)). This refines the corpus’s exact current-trick schedule language. 

The exact census is:

| Filter                                       |        Tagged profiles |
| -------------------------------------------- | ---------------------: |
| Original checks (1)–(3)                      |     64,123,542,674,901 |
| Licensed ((6,6,6))-only fifth condition      |     64,123,217,904,861 |
| Generalized temporal singleton condition     |     63,529,591,020,871 |
| Temporal condition + context-capacity supply | **33,297,009,347,414** |

Thus
[
2^{44}=17,592,186,044,416
<
33,297,009,347,414
<
35,184,372,088,832=2^{45}.
]

The new certified interval is therefore

[
\boxed{[36,45]\text{ bits}}.
]

This does **not** count full seven-subset Hall feasibility. It counts the stated one-context capacity consequence of Hall, so the resulting language remains an outer language and may still contain infeasible or unreachable profiles. The original (7,124,838,074,989)-per-declaration and (64,123,542,674,901)-tagged anchors are those of REACH-11. 

## MACHINE-CHECKABLE ARTIFACTS

FINAL ANSWER: FILTERED_TAGGED_OUTER = 33297009347414
FINAL ANSWER: CEILING = 45 bits
FINAL ANSWER: INTERVAL [36,45] bits

```json
{
  "baseline_tagged_outer": 64123542674901,
  "licensed_fifth_only": 64123217904861,
  "generalized_temporal_follower_supply": 63529591020871,
  "filtered_tagged_outer": 33297009347414,
  "declaration_counts": [
    {
      "declaration": 0,
      "transport_multiplicity": 7,
      "original_outer": 7124838074989,
      "generalized_temporal": 7053861464788,
      "filtered": 3739199825401,
      "filtered_by_completed_tricks": {
        "0": 216122750,
        "1": 28318361427,
        "2": 533890980042,
        "3": 1827643248097,
        "4": 1203144163244,
        "5": 143682064920,
        "6": 2304884920,
        "7": 1
      }
    },
    {
      "declaration": "DT",
      "transport_multiplicity": 1,
      "original_outer": 7124838074989,
      "generalized_temporal": 7053861464788,
      "filtered": 3739199825401,
      "filtered_by_completed_tricks": {
        "0": 216122750,
        "1": 28318361427,
        "2": 533890980042,
        "3": 1827643248097,
        "4": 1203144163244,
        "5": 143682064920,
        "6": 2304884920,
        "7": 1
      }
    },
    {
      "declaration": "NT",
      "transport_multiplicity": 1,
      "original_outer": 7124838074989,
      "generalized_temporal": 7098699302567,
      "filtered": 3383410744206,
      "filtered_by_completed_tricks": {
        "0": 218564630,
        "1": 28476533605,
        "2": 514158308965,
        "3": 1669668137750,
        "4": 1048210259452,
        "5": 120758375971,
        "6": 1920563832,
        "7": 1
      }
    }
  ],
  "powers_of_two": {
    "2^44": 17592186044416,
    "2^45": 35184372088832
  },
  "necessity_smoke_test": {
    "seed": 70189328,
    "complete_hands": 4000,
    "prefixes": 116000,
    "prefixes_per_depth_0_through_28": 4000
  }
}
```

[Download the executed verifier](sandbox:/mnt/data/verify_filtered_outer_45bit.py)

```python
#!/usr/bin/env python3
"""Exact Straight Texas 42 outer-profile census with new necessary filters.

Self-contained, deterministic, stdlib only, no network and no runtime file I/O.
Exit status is zero iff every asserted anchor, census, witness, and legal-prefix
smoke test passes.
"""

from __future__ import annotations

from collections import defaultdict
from itertools import product
from math import comb
import gc
import random
import sys
import time


PIPS = tuple(range(7))
CALLED = 7
DECLARATIONS = PIPS + ("DT", "NT")
HIDDEN = (0, 1, 2)  # viewer-relative h1,h2,h3
DOMINOES = tuple((h, l) for h in PIPS for l in range(h + 1))
INDEX = {d: i for i, d in enumerate(DOMINOES)}
ALL_MASK = (1 << 28) - 1


def mask_of(dominoes):
    mask = 0
    for domino in dominoes:
        mask |= 1 << INDEX[domino]
    return mask


def dominoes_of(mask):
    return tuple(d for d in DOMINOES if mask & (1 << INDEX[d]))


def contains(domino, pip):
    return domino[0] == pip or domino[1] == pip


SIGMA = tuple(mask_of(d for d in DOMINOES if contains(d, p)) for p in PIPS)
DOUBLES = mask_of(d for d in DOMINOES if d[0] == d[1])


def called_mask(declaration):
    if declaration in PIPS:
        return SIGMA[declaration]
    if declaration == "DT":
        return DOUBLES
    if declaration == "NT":
        return 0
    raise ValueError(declaration)


def effective_mask(declaration, context):
    called = called_mask(declaration)
    if context == CALLED:
        return called
    return SIGMA[context] & ~called


def led_context(declaration, domino):
    bit = 1 << INDEX[domino]
    return CALLED if called_mask(declaration) & bit else domino[0]


def lead_fibers(declaration):
    result = {}
    for domino in DOMINOES:
        q = led_context(declaration, domino)
        result[q] = result.get(q, 0) | (1 << INDEX[domino])
    return result


LEAD_FIBERS = {d: lead_fibers(d) for d in DECLARATIONS}


def trick_rank(declaration, domino):
    h, l = domino
    if declaration == "DT" and h == l:
        return h
    if h == l:
        return 100  # TOP
    return h + l


def trick_key(declaration, domino, context):
    bit = 1 << INDEX[domino]
    called = called_mask(declaration)
    if declaration != "NT" and called & bit:
        return (2, trick_rank(declaration, domino))
    if effective_mask(declaration, context) & bit:
        return (1, trick_rank(declaration, domino))
    return (0, 0)


def trick_winner(declaration, trick):
    context = led_context(declaration, trick[0][1])
    winner_index = max(
        range(4),
        key=lambda i: trick_key(declaration, trick[i][1], context),
    )
    return trick[winner_index][0]


def hall_feasible(universe, possible, capacities):
    if universe.bit_count() != sum(capacities):
        return False
    for seat_mask in range(1, 8):
        neighbors = 0
        quota = 0
        for seat in HIDDEN:
            if seat_mask & (1 << seat):
                neighbors |= possible[seat] & universe
                quota += capacities[seat]
        if neighbors.bit_count() < quota:
            return False
    return True


def marginal_holder_masks(universe, possible, capacities):
    """Canonical edge reduction by forced-edge Hall feasibility."""
    if not hall_feasible(universe, possible, capacities):
        return None
    result = [0, 0, 0]
    remaining = universe
    while remaining:
        tile = remaining & -remaining
        remaining ^= tile
        for seat in HIDDEN:
            if capacities[seat] == 0 or not (possible[seat] & tile):
                continue
            successor_capacities = list(capacities)
            successor_capacities[seat] -= 1
            successor_possible = tuple(p & ~tile for p in possible)
            if hall_feasible(
                universe & ~tile,
                successor_possible,
                tuple(successor_capacities),
            ):
                result[seat] |= tile
    return tuple(result)


def cells_from_voids(declaration, universe, voids):
    possible = []
    for seat_voids in voids:
        forbidden = 0
        for context in seat_voids:
            forbidden |= effective_mask(declaration, context)
        possible.append(universe & ~forbidden)
    return tuple(possible)


FOLLOWER_MAXIMUM = {
    frozenset(): frozenset(),
    frozenset({0}): frozenset({0}),
    frozenset({1}): frozenset(),
    frozenset({2}): frozenset(),
    frozenset({0, 1}): frozenset({0, 1}),
    frozenset({0, 2}): frozenset({0}),
    frozenset({1, 2}): frozenset({2}),
    frozenset({0, 1, 2}): frozenset({1, 2}),
}

# Seats for which the current partial trick can have exactly that one hidden
# follower already act, with no other hidden follower yet public.
CURRENT_SINGLETON_ESCAPE = {
    frozenset(): frozenset(),
    frozenset({0}): frozenset({0}),
    frozenset({1}): frozenset(),
    frozenset({2}): frozenset(),
    frozenset({0, 1}): frozenset({1}),
    frozenset({0, 2}): frozenset({0}),
    frozenset({1, 2}): frozenset({2}),
    frozenset({0, 1, 2}): frozenset(),
}


def hidden_capacity_profiles():
    return tuple(
        k
        for k in product(range(8), repeat=3)
        if max(k) - min(k) <= 1
    )


CAPACITY_PROFILES = hidden_capacity_profiles()
PROFILES_BY_POOL_SIZE = defaultdict(list)
for _k in CAPACITY_PROFILES:
    PROFILES_BY_POOL_SIZE[sum(_k)].append(_k)


def profile_metadata(capacities):
    """Return n,j,B,F,E for the outer schedule convention."""
    n = sum(capacities)
    if capacities[0] == capacities[1] == capacities[2]:
        h = capacities[0]
        return n, 7 - h, frozenset(), frozenset(), frozenset()
    h = max(capacities)
    low = frozenset(i for i, value in enumerate(capacities) if value == h - 1)
    return (
        n,
        7 - h,
        low,
        FOLLOWER_MAXIMUM[low],
        CURRENT_SINGLETON_ESCAPE[low],
    )


def capacity_shape_ok(capacities):
    return max(capacities) - min(capacities) <= 1


def schedule_admissible(capacities, voids):
    _, completed, _, followers, _ = profile_metadata(capacities)
    used = set().union(*voids)
    if len(used) <= completed:
        return True
    if len(used) != completed + 1:
        return False
    for context in used:
        members = frozenset(i for i in HIDDEN if context in voids[i])
        if members and members.issubset(followers):
            return True
    return False


def contexts_leadable(declaration, voids):
    leadable = set(LEAD_FIBERS[declaration])
    return set().union(*voids).issubset(leadable)


def lead_witness_ok(declaration, universe, voids):
    for context in set().union(*voids):
        if not (LEAD_FIBERS[declaration][context] & ~universe):
            return False
    return True


def new_necessary_condition(declaration, capacities, voids, universe):
    """Per-context Hall supply plus exact current-singleton exception.

    Returns (ok, detail).  A used context q with member set M must satisfy

      |S_q \\ U| >= max(1, |S_q| - sum_{s notin M} k_s).

    A singleton M additionally needs two public suit tiles, except that one
    context may have only one when its sole voiding seat is in E(B), meaning
    it can be the current trick with exactly that hidden follower already
    public and no nonvoid hidden co-follower yet public.
    """
    _, _, _, _, escape = profile_metadata(capacities)
    used = set().union(*voids)
    weak = []

    for context in used:
        members = frozenset(i for i in HIDDEN if context in voids[i])
        suit = effective_mask(declaration, context)
        outside_supply = (suit & ~universe).bit_count()
        complement_capacity = sum(
            capacities[i] for i in HIDDEN if i not in members
        )
        required = max(1, suit.bit_count() - complement_capacity)
        if outside_supply < required:
            return False, (
                "context-capacity",
                context,
                tuple(sorted(members)),
                outside_supply,
                required,
            )
        if len(members) == 1 and outside_supply < 2:
            weak.append((context, next(iter(members))))

    if len(weak) > 1:
        return False, ("multiple-current-singletons", tuple(weak))
    if weak and weak[0][1] not in escape:
        return False, (
            "current-singleton-seat",
            weak[0],
            tuple(sorted(escape)),
        )
    return True, None


def outer_profile_checks(declaration, capacities, voids, universe, hall=False):
    checks = {
        "capacity": capacity_shape_ok(capacities)
        and universe.bit_count() == sum(capacities),
        "schedule": schedule_admissible(capacities, voids)
        and contexts_leadable(declaration, voids),
        "lead": lead_witness_ok(declaration, universe, voids),
    }
    if hall:
        possible = cells_from_voids(declaration, universe, voids)
        checks["hall"] = hall_feasible(universe, possible, capacities)
    return checks


def polynomial_multiply(left, right):
    result = [0] * (len(left) + len(right) - 1)
    for i, a in enumerate(left):
        for j, b in enumerate(right):
            result[i + j] += a * b
    return result


def lead_witness_subset_counts():
    """B[n][u] from lead-fiber sizes 1..7."""
    counts = [[0] * 8 for _ in range(29)]
    for used_mask in range(1 << 7):
        used = used_mask.bit_count()
        polynomial = [1]
        for index, fiber_size in enumerate(range(1, 8)):
            factor = [comb(fiber_size, k) for k in range(fiber_size + 1)]
            if used_mask & (1 << index):
                factor[-1] -= 1
            polynomial = polynomial_multiply(polynomial, factor)
        for selected, coefficient in enumerate(polynomial):
            counts[selected][used] += coefficient
    for selected in range(29):
        assert counts[selected][0] == comb(28, selected)
    return tuple(tuple(row) for row in counts)


def original_outer_count_for_profile(capacities, witness_counts):
    if capacities == (0, 0, 0):
        return 1
    n, completed, _, followers, _ = profile_metadata(capacities)
    total = sum(
        7**used * witness_counts[n][used]
        for used in range(completed + 1)
    )
    if followers:
        used = completed + 1
        f = len(followers)
        total += (
            7**used - (8 - 2**f) ** used
        ) * witness_counts[n][used]
    return total


def licensed_fifth_elimination(declaration):
    """Exact rejected count at k=(6,6,6) for the licensed 002 condition."""
    eliminated = 0
    for context, lead in LEAD_FIBERS[declaration].items():
        suit_size = effective_mask(declaration, context).bit_count()
        lead_size = lead.bit_count()
        # U has 18 tiles, contains all but one suit tile, and the missing suit
        # tile must be a lead tile to retain the old lead-witness check.
        from_complement = 18 - (suit_size - 1)
        if 0 <= from_complement <= 28 - suit_size:
            pools = lead_size * comb(28 - suit_size, from_complement)
            eliminated += 3 * pools  # three singleton hidden-seat patterns
    return eliminated


def declaration_context_data(declaration):
    fibers = LEAD_FIBERS[declaration]
    contexts = tuple(sorted(fibers))
    lead_masks = tuple(fibers[q] for q in contexts)
    suit_masks = tuple(effective_mask(declaration, q) for q in contexts)
    assert len(contexts) == 7
    assert sorted(mask.bit_count() for mask in lead_masks) == list(range(1, 8))
    assert sum(mask.bit_count() for mask in lead_masks) == 28
    union = 0
    for lead in lead_masks:
        assert not (union & lead)
        union |= lead
    assert union == ALL_MASK
    for lead, suit in zip(lead_masks, suit_masks):
        assert lead & ~suit == 0
    return contexts, lead_masks, suit_masks


def grouped_lead_fiber_options(declaration):
    """Enumerate local outside-set choices inside each disjoint lead fiber."""
    contexts, lead_masks, suit_masks = declaration_context_data(declaration)
    groups = []
    for context_index, lead_mask in enumerate(lead_masks):
        tiles = dominoes_of(lead_mask)
        aggregated = defaultdict(int)
        for subset in range(1 << len(tiles)):
            suit_counts = [0] * 7
            chosen = subset.bit_count()
            for local_index, domino in enumerate(tiles):
                if not (subset & (1 << local_index)):
                    continue
                bit = 1 << INDEX[domino]
                for suit_index, suit in enumerate(suit_masks):
                    if suit & bit:
                        suit_counts[suit_index] += 1
            code = sum(value << (3 * i) for i, value in enumerate(suit_counts))
            hit = (1 << context_index) if subset else 0
            aggregated[(chosen, code, hit)] += 1
        groups.append(
            tuple((size, code, hit, multiplicity)
                  for (size, code, hit), multiplicity in aggregated.items())
        )
    return contexts, suit_masks, tuple(groups)


def build_outside_signature_distribution(declaration):
    """Exact distribution of W=D\\U by size, suit counts, and lead hits."""
    contexts, suit_masks, groups = grouped_lead_fiber_options(declaration)
    distribution = {(0, 0, 0): 1}
    for options in groups:
        successor = defaultdict(int)
        for (size, code, hit), multiplicity in distribution.items():
            for add_size, add_code, add_hit, local_multiplicity in options:
                successor[(
                    size + add_size,
                    code + add_code,
                    hit | add_hit,
                )] += multiplicity * local_multiplicity
        distribution = dict(successor)
    assert sum(distribution.values()) == 1 << 28
    return contexts, suit_masks, distribution


def membership_count(types, capacities, suit_sizes, followers, escape, mode):
    """Count void-membership maps for one W signature and capacity profile.

    mode='outer'    : checks (1)-(3) only.
    mode='temporal' : adds the generalized singleton follower-supply rule.
    mode='local'    : additionally adds the per-context Hall-capacity bound.
    """
    follower_mask = sum(1 << s for s in followers)
    escape_mask = sum(1 << s for s in escape)

    # P: no weak singleton, all schedule patterns.
    # N: no weak singleton and no current-compatible pattern.
    # Q: exactly one weak singleton, which itself is the current context.
    p = [0] * 8
    no_current = [0] * 8
    one_weak = [0] * 8
    p[0] = 1
    no_current[0] = 1

    for encoded, suit_size in zip(types, suit_sizes):
        outside_supply = encoded >> 1
        lead_hit = encoded & 1
        strong = 0
        strong_not_current = 0
        weak = 0

        if lead_hit:
            for member_mask in range(1, 8):
                if mode == "outer":
                    locally_valid = True
                else:
                    if mode == "local":
                        complement_capacity = sum(
                            capacities[s]
                            for s in HIDDEN
                            if not (member_mask & (1 << s))
                        )
                        required = max(1, suit_size - complement_capacity)
                    else:
                        required = 1
                    locally_valid = outside_supply >= required

                if not locally_valid:
                    continue

                if (
                    mode != "outer"
                    and member_mask.bit_count() == 1
                    and outside_supply == 1
                ):
                    if member_mask & escape_mask:
                        weak += 1
                    continue

                strong += 1
                if member_mask & ~follower_mask:
                    strong_not_current += 1

        old_p = p[:]
        old_no_current = no_current[:]
        old_one_weak = one_weak[:]
        for used in range(1, 8):
            p[used] = old_p[used] + strong * old_p[used - 1]
            no_current[used] = (
                old_no_current[used]
                + strong_not_current * old_no_current[used - 1]
            )
            one_weak[used] = (
                old_one_weak[used]
                + strong * old_one_weak[used - 1]
                + weak * old_p[used - 1]
            )

    _, completed, _, _, _ = profile_metadata(capacities)
    total = sum(p[: completed + 1])
    if completed < 7:
        # At u=j+1, at least one strong pattern must be a nonempty subset of F.
        total += p[completed + 1] - no_current[completed + 1]
        if mode != "outer":
            # A weak singleton is itself the unique current-context exception.
            total += sum(one_weak[1 : completed + 2])
    return total


def census_declaration_class(declaration):
    contexts, suit_masks, distribution = build_outside_signature_distribution(
        declaration
    )
    suit_sizes_ordered = tuple(mask.bit_count() for mask in suit_masks)

    if len(set(suit_sizes_ordered)) == 1:
        # NT: all seven contexts have equal suit size, so the polynomial count
        # depends only on the multiset of (outside_supply, lead_hit) pairs.
        memo_suit_sizes = (suit_sizes_ordered[0],) * 7

        def canonical_types(code, hit):
            values = [
                ((((code >> (3 * i)) & 7) << 1) | ((hit >> i) & 1))
                for i in range(7)
            ]
            values.sort()
            return tuple(values)

    else:
        # Pip trump and DT: six size-6 natural contexts plus one size-7 called
        # context. Natural factors commute; the called factor remains separate.
        called_index = suit_sizes_ordered.index(7)
        natural_indices = tuple(
            i for i, size in enumerate(suit_sizes_ordered) if size == 6
        )
        assert len(natural_indices) == 6
        memo_suit_sizes = (6,) * 6 + (7,)

        def canonical_types(code, hit):
            values = sorted(
                ((((code >> (3 * i)) & 7) << 1) | ((hit >> i) & 1))
                for i in natural_indices
            )
            values.append(
                (((code >> (3 * called_index)) & 7) << 1)
                | ((hit >> called_index) & 1)
            )
            return tuple(values)

    modes = ("outer", "temporal", "local")
    totals_by_profile = {
        mode: {k: 0 for k in CAPACITY_PROFILES}
        for mode in modes
    }
    caches = {
        mode: {k: {} for k in CAPACITY_PROFILES if k != (0, 0, 0)}
        for mode in modes
    }

    for (outside_size, code, lead_hit), multiplicity in distribution.items():
        pool_size = 28 - outside_size
        relevant_profiles = PROFILES_BY_POOL_SIZE.get(pool_size)
        if not relevant_profiles:
            continue
        if pool_size == 0:
            for mode in modes:
                totals_by_profile[mode][(0, 0, 0)] += multiplicity
            continue

        types = canonical_types(code, lead_hit)
        for capacities in relevant_profiles:
            _, _, _, followers, escape = profile_metadata(capacities)
            for mode in modes:
                cache = caches[mode][capacities]
                count = cache.get(types)
                if count is None:
                    count = membership_count(
                        types,
                        capacities,
                        memo_suit_sizes,
                        followers,
                        escape,
                        mode,
                    )
                    cache[types] = count
                totals_by_profile[mode][capacities] += multiplicity * count

    totals = {
        mode: sum(totals_by_profile[mode].values())
        for mode in modes
    }
    phase_local = defaultdict(int)
    for capacities, count in totals_by_profile["local"].items():
        _, completed, _, _, _ = profile_metadata(capacities)
        phase_local[completed] += count

    del distribution
    del caches
    gc.collect()
    return totals, dict(sorted(phase_local.items()))


def verify_002_witness():
    pool = mask_of((
        (0, 0),
        (1, 0), (1, 1),
        (2, 0), (2, 1), (2, 2),
        (3, 0), (3, 1), (3, 2), (3, 3),
        (4, 0), (4, 1),
        (6, 0), (6, 1), (6, 2), (6, 3), (6, 4), (6, 5),
    ))
    declaration = "NT"
    capacities = (6, 6, 6)
    voids = (frozenset({6}), frozenset(), frozenset())
    classic = outer_profile_checks(
        declaration, capacities, voids, pool, hall=True
    )
    possible = cells_from_voids(declaration, pool, voids)
    reduced = marginal_holder_masks(pool, possible, capacities)
    new_ok, detail = new_necessary_condition(
        declaration, capacities, voids, pool
    )
    supply = (effective_mask(declaration, 6) & ~pool).bit_count()
    return (
        all(classic.values())
        and reduced == possible
        and not new_ok
        and supply == 1,
        classic,
        detail,
    )


def verify_reach10_lead_rejection():
    sigma_zero = SIGMA[0]
    extras = mask_of(((2, 1), (3, 1), (3, 2), (4, 1), (4, 2)))
    pool = sigma_zero | DOUBLES | extras
    capacities = (6, 6, 6)
    tags = (
        (0, CALLED),
        ("NT", 0),
    )
    details = []
    for declaration, context in tags:
        voids = (frozenset({context}), frozenset(), frozenset())
        checks = outer_profile_checks(
            declaration, capacities, voids, pool, hall=True
        )
        details.append((declaration, context, checks))
        if not (
            checks["capacity"]
            and checks["schedule"]
            and checks["hall"]
            and not checks["lead"]
        ):
            return False, details
    return True, details


def random_legal_prefix_smoke(hand_count=4000, seed=0x42F0110):
    rng = random.Random(seed)
    prefix_count = 0
    depth_counts = [0] * 29

    for _ in range(hand_count):
        shuffled = list(DOMINOES)
        rng.shuffle(shuffled)
        hands = [
            mask_of(shuffled[7 * seat : 7 * (seat + 1)])
            for seat in range(4)
        ]
        viewer = rng.randrange(4)
        bidder = rng.randrange(4)
        declaration = rng.choice(DECLARATIONS)
        leader = bidder  # legal auction: bidder bids 30, the other seats pass
        trick = []
        played = 0
        voids_absolute = [set() for _ in range(4)]

        def check_prefix(depth):
            nonlocal prefix_count
            hidden_absolute = tuple((viewer + offset) % 4 for offset in (1, 2, 3))
            capacities = tuple(
                hands[seat].bit_count() for seat in hidden_absolute
            )
            universe = ALL_MASK & ~(hands[viewer] | played)
            voids = tuple(
                frozenset(voids_absolute[seat])
                for seat in hidden_absolute
            )
            checks = outer_profile_checks(
                declaration, capacities, voids, universe, hall=True
            )
            if not all(checks.values()):
                return False, (
                    "classic",
                    depth,
                    declaration,
                    capacities,
                    voids,
                    checks,
                )
            possible = cells_from_voids(declaration, universe, voids)
            for relative, absolute in enumerate(hidden_absolute):
                if hands[absolute] & ~possible[relative]:
                    return False, ("actual-world-not-in-cells", depth)
            new_ok, detail = new_necessary_condition(
                declaration, capacities, voids, universe
            )
            if not new_ok:
                return False, (
                    "new-condition",
                    depth,
                    declaration,
                    capacities,
                    voids,
                    detail,
                )
            prefix_count += 1
            depth_counts[depth] += 1
            return True, None

        ok, detail = check_prefix(0)
        if not ok:
            return False, prefix_count, depth_counts, detail

        for depth in range(1, 29):
            actor = leader if not trick else (trick[0][0] + len(trick)) % 4
            if not trick:
                legal = hands[actor]
            else:
                context = led_context(declaration, trick[0][1])
                followers = hands[actor] & effective_mask(declaration, context)
                legal = followers if followers else hands[actor]
            options = dominoes_of(legal)
            tile = rng.choice(options)
            tile_bit = 1 << INDEX[tile]

            if trick:
                context = led_context(declaration, trick[0][1])
                if not (effective_mask(declaration, context) & tile_bit):
                    voids_absolute[actor].add(context)

            hands[actor] &= ~tile_bit
            played |= tile_bit
            trick.append((actor, tile))

            if len(trick) == 4:
                leader = trick_winner(declaration, trick)
                trick = []

            ok, detail = check_prefix(depth)
            if not ok:
                return False, prefix_count, depth_counts, detail

    return (
        prefix_count >= 100_000
        and all(count == hand_count for count in depth_counts),
        prefix_count,
        depth_counts,
        None,
    )


class Results:
    def __init__(self):
        self.failures = 0

    def check(self, name, condition, detail=""):
        suffix = f" {detail}" if detail else ""
        if condition:
            print(f"PASS {name}{suffix}")
        else:
            self.failures += 1
            print(f"FAIL {name}{suffix}")


def main():
    started = time.perf_counter()
    results = Results()

    results.check(
        "rule-universe",
        len(DOMINOES) == 28
        and all(SIGMA[p].bit_count() == 7 for p in PIPS)
        and len(DECLARATIONS) == 9,
    )
    results.check(
        "capacity-profiles",
        len(CAPACITY_PROFILES) == 50,
        f"count={len(CAPACITY_PROFILES)}",
    )

    witness_counts = lead_witness_subset_counts()
    per_declaration_anchor = sum(
        original_outer_count_for_profile(k, witness_counts)
        for k in CAPACITY_PROFILES
    )
    tagged_anchor = 9 * per_declaration_anchor
    results.check(
        "anchor-per-declaration",
        per_declaration_anchor == 7_124_838_074_989,
        f"count={per_declaration_anchor}",
    )
    results.check(
        "anchor-tagged-outer",
        tagged_anchor == 64_123_542_674_901,
        f"count={tagged_anchor}",
    )

    licensed_pip = licensed_fifth_elimination(0)
    licensed_dt = licensed_fifth_elimination("DT")
    licensed_nt = licensed_fifth_elimination("NT")
    licensed_eliminated = 7 * licensed_pip + licensed_dt + licensed_nt
    licensed_filtered = tagged_anchor - licensed_eliminated
    results.check(
        "licensed-fifth-only",
        (
            licensed_pip == 37_509_990
            and licensed_dt == 37_509_990
            and licensed_nt == 24_690_120
            and licensed_filtered == 64_123_217_904_861
        ),
        (
            f"filtered={licensed_filtered} "
            f"eliminated={licensed_eliminated}"
        ),
    )

    witness_ok, witness_classic, witness_detail = verify_002_witness()
    results.check(
        "002-witness-new-filter-rejects",
        witness_ok,
        f"classic={witness_classic} rejection={witness_detail}",
    )

    reach10_ok, reach10_detail = verify_reach10_lead_rejection()
    results.check(
        "REACH-10-lead-witness-rejects",
        reach10_ok,
        f"tags={reach10_detail}",
    )

    expected = {
        0: {
            "outer": 7_124_838_074_989,
            "temporal": 7_053_861_464_788,
            "local": 3_739_199_825_401,
        },
        "DT": {
            "outer": 7_124_838_074_989,
            "temporal": 7_053_861_464_788,
            "local": 3_739_199_825_401,
        },
        "NT": {
            "outer": 7_124_838_074_989,
            "temporal": 7_098_699_302_567,
            "local": 3_383_410_744_206,
        },
    }
    censuses = {}
    phase_tables = {}
    for declaration in (0, "DT", "NT"):
        class_started = time.perf_counter()
        totals, phase_local = census_declaration_class(declaration)
        censuses[declaration] = totals
        phase_tables[declaration] = phase_local
        results.check(
            f"signature-census-{declaration}",
            totals == expected[declaration],
            (
                f"outer={totals['outer']} "
                f"temporal={totals['temporal']} "
                f"local={totals['local']} "
                f"seconds={time.perf_counter() - class_started:.3f}"
            ),
        )

    temporal_tagged = (
        7 * censuses[0]["temporal"]
        + censuses["DT"]["temporal"]
        + censuses["NT"]["temporal"]
    )
    filtered_tagged = (
        7 * censuses[0]["local"]
        + censuses["DT"]["local"]
        + censuses["NT"]["local"]
    )
    results.check(
        "generalized-temporal-follower-supply",
        temporal_tagged == 63_529_591_020_871,
        f"count={temporal_tagged}",
    )
    results.check(
        "headline-filtered-tagged-outer",
        filtered_tagged == 33_297_009_347_414,
        f"count={filtered_tagged}",
    )
    results.check(
        "45-bit-ceiling",
        2**44 < filtered_tagged < 2**45,
        (
            f"2^44={2**44} count={filtered_tagged} "
            f"2^45={2**45}"
        ),
    )

    expected_pip_phase = {
        0: 216_122_750,
        1: 28_318_361_427,
        2: 533_890_980_042,
        3: 1_827_643_248_097,
        4: 1_203_144_163_244,
        5: 143_682_064_920,
        6: 2_304_884_920,
        7: 1,
    }
    expected_nt_phase = {
        0: 218_564_630,
        1: 28_476_533_605,
        2: 514_158_308_965,
        3: 1_669_668_137_750,
        4: 1_048_210_259_452,
        5: 120_758_375_971,
        6: 1_920_563_832,
        7: 1,
    }
    results.check(
        "phase-decomposition",
        (
            phase_tables[0] == expected_pip_phase
            and phase_tables["DT"] == expected_pip_phase
            and phase_tables["NT"] == expected_nt_phase
        ),
        f"pip={phase_tables[0]} DT={phase_tables['DT']} NT={phase_tables['NT']}",
    )

    smoke_ok, prefix_count, depth_counts, smoke_detail = random_legal_prefix_smoke()
    results.check(
        "necessity-smoke",
        smoke_ok,
        (
            f"prefixes={prefix_count} hands=4000 seed={0x42F0110} "
            f"depth_min={min(depth_counts)} depth_max={max(depth_counts)} "
            f"detail={smoke_detail}"
        ),
    )

    elapsed = time.perf_counter() - started
    results.check(
        "runtime",
        elapsed < 21_600,
        f"seconds={elapsed:.3f}",
    )

    if results.failures:
        print(f"FAIL overall failures={results.failures}")
        return 1

    print(
        "PASS overall "
        f"FILTERED_TAGGED_OUTER={filtered_tagged} "
        "CEILING=45 INTERVAL=[36,45] "
        f"seconds={elapsed:.3f}"
    )
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except SystemExit:
        raise
    except Exception as exc:
        print(f"FAIL exception {type(exc).__name__}: {exc}")
        raise SystemExit(1)
```

### Proof

1. **[USES: declaration algebra; definitions of (U,k_s,V_s,M(q)); location conservation]**
   Write
   [
   S_q=\widehat\sigma_q^\delta,\qquad
   W=\mathcal D\setminus U,\qquad
   a_q=|S_q\cap W|=|S_q\setminus U|.
   ]
   Every tile in (W) is either public or in the viewer’s current hand. Every tile in (U) belongs to one of the three current hidden remainders. The lead fiber satisfies
   [
   L_{\delta,q}\subseteq S_q.
   ]

2. **[USES: capacity-cell definition; exact cell losslessness; Hall feasibility]**
   Fix a used context (q), and put (M=M(q)). For every (s\in M), the public void fact gives
   [
   H_s\cap S_q=\varnothing.
   ]
   The disjoint hidden hands of seats in (M) therefore contribute
   [
   \sum_{s\in M}k_s
   ]
   distinct tiles to (U\setminus S_q). Hence
   [
   |U\setminus S_q|
   \ge
   \sum_{s\in M}k_s.
   ]
   Because (|U|=\sum_sk_s),
   [
   |U\cap S_q|
   \le
   \sum_{s\notin M}k_s.
   ]
   Subtracting from (|S_q|) gives the **context-capacity supply bound**
   [
   a_q
   \ge
   |S_q|-\sum_{s\notin M}k_s.
   ]
   The existing lead-witness theorem gives (a_q\ge1). Therefore every reachable profile satisfies
   [
   \boxed{
   a_q\ge
   \beta_q(k,M):=
   \max!\left(
   1,;
   |S_q|-\sum_{s\notin M}k_s
   \right)}.
   ]
   This is precisely the Hall inequality for the seat subset (M), projected onto the single common forbidden set (S_q). It is weaker than full Hall and is therefore safe to count without claiming a full Hall census.

3. **[USES: cyclic four-seat trick order; lead-anything; follow-if-possible]**
   In a completed trick led in context (q), let (\lambda) be the leader. The hidden follower set is
   [
   G(\lambda)=
   \begin{cases}
   {h_1,h_2,h_3},&\lambda=m,\
   {h_1,h_2,h_3}\setminus{\lambda},&\lambda\text{ hidden}.
   \end{cases}
   ]
   The lead contributes one public tile in (S_q). Every hidden follower in (G(\lambda)\setminus M(q)) never publicly failed to follow (q), and therefore played a tile in (S_q) in this trick. All those plays are distinct. Thus any completed acquisition trick supplies at least
   [
   1+|G(\lambda)\setminus M(q)|
   ]
   distinct public (S_q)-tiles.

4. **[USES: Step 3; four possible leader identities]**
   Minimizing the completed-trick bound over leaders compatible with the void membership gives
   [
   g_{\mathrm{completed}}(M)=
   \begin{cases}
   2,&|M|=1,\
   1,&|M|=2,\
   1,&|M|=3.
   \end{cases}
   ]
   For a singleton, the voiding seat must be a follower. A hidden leader can remove at most one of the two other hidden seats from the follower set, leaving one nonvoid hidden co-follower, so two suit tiles are forced. For a pair, the third hidden seat can lead, leaving exactly the two voiding seats as hidden followers. For all three hidden seats, the viewer can lead. Therefore no universal completed-trick improvement beyond one tile exists for pair or triple memberships.

5. **[USES: exact capacity-shape theorem; cyclic current-trick prefixes]**
   Let (B) be the hidden low-capacity set. Enumerating the current clockwise prefixes gives the following cases in which the already-acted hidden follower set is exactly one seat:
   [
   \begin{array}{c|c|c}
   B&\text{current prefix}&\text{sole hidden follower}\\hline
   {h_1}&(m,h_1)&h_1\
   {h_1,h_2}&(h_1,h_2)&h_2\
   {h_1,h_3}&(h_3,m,h_1)&h_1\
   {h_2,h_3}&(h_2,h_3)&h_3.
   \end{array}
   ]
   No other low-set admits exactly one already-acted hidden follower. This yields
   [
   E({h_1})={h_1},\quad
   E({h_1,h_2})={h_2},\quad
   E({h_1,h_3})={h_1},\quad
   E({h_2,h_3})={h_3},
   ]
   with all other (E(B)) empty.

   If the three hidden capacities are equal, the hidden seats have either all or none of the current-trick plays. With none, there is no hidden follower observation. With all three, the only open prefix having equal hidden capacities is ((h_1,h_2,h_3)), whose hidden follower set is ({h_2,h_3}). Therefore equal profiles have no singleton-current exception.

6. **[USES: Steps 3–5; one current trick]**
   Suppose (M(q)={s}) and (a_q=1). The singleton void cannot have been acquired in a completed trick by Step 4. It also cannot have been acquired in a current prefix after another nonvoid hidden follower acted, because that follower’s successful play would be a second public tile in (S_q). Hence the void was acquired in the current trick with exactly one already-acted hidden follower, namely (s). By Step 5,
   [
   s\in E(B).
   ]
   There is only one current trick, so at most one context can use this exception.

   Define
   [
   \mathcal W=
   {q:|M(q)|=1,\ a_q=1}.
   ]
   Every reachable profile therefore satisfies
   [
   |\mathcal W|\le1,
   ]
   and when (\mathcal W={q}), the sole member of (M(q)) belongs to (E(B)).

7. **[USES: Steps 2 and 6]**
   The complete new counted condition is:

   * for every used (q),
     [
     a_q\ge\beta_q(k,M(q));
     ]
   * for singleton (M(q)), normally
     [
     a_q\ge2;
     ]
   * the only permitted violation of the second inequality is one context with (a_q=1), whose singleton seat belongs to (E(B)).

   The licensed fifth condition is the special case (k=(6,6,6)). Equal profiles have (E=\varnothing), so every singleton context requires (a_q\ge2).

8. **[USES: Step 7; established 002 witness]**
   In the 002 witness,
   [
   \delta=\mathrm{NT},\quad
   k=(6,6,6),\quad
   M(6)={h_1},
   ]
   and
   [
   |\sigma_6\setminus U|=1.
   ]
   The four classic checks pass, but (E=\varnothing), so the generalized condition rejects it. The program also recomputes Hall feasibility and confirms that its raw cells are already support-reduced.

9. **[USES: REACH-10; lead-witness theorem]**
   For REACH-10, the only two static tags decoding to the support are zeroes-trump/called-7 and NT/natural-0, each with only (h_1) void. Their lead fibers are respectively (\sigma_0) and ({0:0}), both contained in (U). The program checks both tags and confirms that capacity, schedule, and Hall pass while lead witness fails. This preserves the distinction between REACH-10 and the 002 obstruction. 

10. **[USES: REACH-11 outer formula]**
    Independently of the new census, the program reconstructs
    [
    B_{n,u}
    ]
    from the seven lead-fiber sizes (1,\ldots,7), evaluates all 50 labeled capacity profiles, and obtains
    [
    7,124,838,074,989
    ]
    per declaration and
    [
    64,123,542,674,901
    ]
    over nine declaration tags. This reproduces both required anchors from the supplied corpus verifier without importing it. 

11. **[USES: disjoint lead-fiber partition; (W=\mathcal D\setminus U)]**
    For one declaration, define the outside-set signature
    [
    \Sigma(W)=
    \left(
    |W|,
    (|W\cap S_q|)*{q\in\Lambda*\delta},
    (\mathbf1[W\cap L_{\delta,q}\ne\varnothing])*{q\in\Lambda*\delta}
    \right).
    ]
    The seven lead fibers partition (\mathcal D). The program processes them one at a time. For every local subset (X\subseteq L_{\delta,q}), it adds:

    * (|X|) to the outside-set size;
    * (|X\cap S_r|) to every effective-suit count (r);
    * the lead-hit bit for (q) exactly when (X\ne\varnothing).

    Every (W\subseteq\mathcal D) has one and only one tuple of local choices
    [
    (W\cap L_{\delta,q})_q.
    ]
    Thus the dynamic program counts every outside set exactly once. For each declaration class its final multiplicities sum to
    [
    2^{28}.
    ]

12. **[USES: Steps 2, 6, and 11; exact schedule-language theorem]**
    Fix a signature and capacity profile. For each context (q), classify its seven possible nonempty membership patterns:

    * **strong:** it satisfies the context-capacity bound and, if singleton, has (a_q\ge2);
    * **weak:** it is a singleton with (a_q=1), its seat is in (E(B)), and it satisfies the context-capacity bound;
    * **invalid:** all other cases.

    Let (t_q) be the number of strong patterns, (n_q) the number of strong patterns that are not nonempty subsets of (F(B)), and (w_q) the number of weak patterns.

    Define
    [
    P(x)=\prod_q(1+t_qx),
    \qquad
    N(x)=\prod_q(1+n_qx),
    ]
    and
    [
    Q(x)=
    x\sum_qw_q\prod_{r\ne q}(1+t_rx).
    ]

    The exact number of accepted void-mask assignments for this signature is
    [
    \sum_{u=0}^{j}[x^u]P(x)
    +
    [x^{j+1}]\bigl(P(x)-N(x)\bigr)
    +
    \sum_{u=1}^{j+1}[x^u]Q(x).
    ]
    The three terms respectively count:

    1. at most (j) used contexts;
    2. (j+1) contexts with at least one ordinary current-compatible membership;
    3. exactly one weak singleton, which itself occupies the unique current context.

    Assignments with two weak contexts never enter (Q). This is exactly the combined schedule and new-condition language.

13. **[USES: Steps 11–12; all-pip-trump mechanics transport theorem]**
    Summing the assignment count against every outside-signature multiplicity and all 50 capacity profiles gives:
    [
    \begin{array}{c|r|r|r}
    \text{declaration class}
    &\text{old outer}
    &\text{temporal only}
    &\text{temporal + context capacity}\\hline
    \text{one pip trump}
    &7,124,838,074,989
    &7,053,861,464,788
    &3,739,199,825,401\
    \mathrm{DT}
    &7,124,838,074,989
    &7,053,861,464,788
    &3,739,199,825,401\
    \mathrm{NT}
    &7,124,838,074,989
    &7,098,699,302,567
    &3,383,410,744,206.
    \end{array}
    ]
    The unscored mechanics transport gives the same census for each of the seven pip declarations. Therefore
    [
    \begin{aligned}
    N_{\mathrm{filtered}}
    &=
    7(3,739,199,825,401)
    +3,739,199,825,401
    +3,383,410,744,206\
    &=33,297,009,347,414.
    \end{aligned}
    ]

14. **[USES: Steps 2–7; exact game transition rules]**
    The necessity smoke test generates 4,000 complete random deals from fixed seed `70189328`, chooses random viewers, bidders, and declarations, and plays every hand to completion using only the exact legal action set and declaration-relative winner rule. It checks every depth (0,\ldots,28), giving
    [
    4,000\cdot29=116,000
    ]
    prefixes. At every prefix it reconstructs (U,k,V), verifies capacity shape, schedule, leadability, lead witness, full Hall feasibility, containment of the actual hidden world in the cells, and both new conditions. All 116,000 pass.

15. **[USES: Step 13; established reachable-family floor]**
    Numerically,
    [
    2^{44}
    <
    33,297,009,347,414
    <
    2^{45}.
    ]
    Hence the filtered tagged language proves
    [
    |\mathscr R_{\mathrm{Str}}^m|<2^{45}
    ]
    and therefore
    [
    \left\lceil\log_2|\mathscr R_{\mathrm{Str}}^m|\right\rceil\le45.
    ]
    The established disjoint reachable floor exceeds (2^{35}), so the lower endpoint remains 36. The resulting certified interval is
    [
    \boxed{[36,45]\text{ bits}}.
    ]

