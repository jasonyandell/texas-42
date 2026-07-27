---
number: 1
slug: reachable-support-cardinality
conversation: https://chatgpt.com/c/6a66e786-2ac0-83ea-ade0-dff707fae5e6
submitted-at: 2026-07-27T05:07:18.983Z
harvested-at: 2026-07-27T13:02:58.354Z
extraction: copy-button
caveats: none
---
I did not obtain the exact value of (|R|), B1, or B2. I obtained a rigorously disjoint family of **17,668,066,045 reachable supports**, which is larger than (2^{34}=17,179,869,184). Combined with the attachment’s declaration-tagged outer bound, this improves the standalone bit interval from ([26,46]) to **([35,46])**.

The construction uses only the normative rules, the exact support normal form, and the proved outer-certificate theorem in the supplied corpus.   The program also independently reproduces the relevant anchors from the supplied verification package. 

## MACHINE-CHECKABLE ARTIFACTS

FINAL ANSWER: INTERVAL [35,46] bits

Expected runtime is approximately 40 CPU-seconds on a current CPython implementation with efficient big integers; a conservative allowance is 15 minutes. Expected peak memory is below 1 GB. The three large counting phases are sequential, so only one CPU core is used.

```python
#!/usr/bin/env python3
"""Exact verification receipt for INTERVAL [35,46] bits for OPEN-11.

Standard library only. Deterministic. No network and no file I/O.
"""

from __future__ import annotations

from collections import defaultdict
from functools import lru_cache
from itertools import combinations, permutations, product
from math import comb
import gc
import sys
import time


# ---------------------------------------------------------------------------
# Straight 42 rule algebra
# ---------------------------------------------------------------------------

PIPS = tuple(range(7))
DOMINOES = tuple((h, l) for h in PIPS for l in range(h + 1))
INDEX = {d: i for i, d in enumerate(DOMINOES)}
ALL_MASK = (1 << 28) - 1

DECLARATIONS = PIPS + ("DT", "NT")
MODULE_DECLARATIONS = PIPS + ("DT",)
CALLED = 7

VIEWER = 0
HIDDEN = (1, 2, 3)
K8_DOUBLE_VERTEX = 7


def tile_mask(tiles):
    result = 0
    for d in tiles:
        result |= 1 << INDEX[tuple(d)]
    return result


def bit_indices(mask):
    while mask:
        bit = mask & -mask
        yield bit.bit_length() - 1
        mask ^= bit


def contains(d, pip):
    return d[0] == pip or d[1] == pip


def is_double(d):
    return d[0] == d[1]


def count_label(d):
    if d in ((5, 0), (4, 1), (3, 2)):
        return 5
    if d in ((6, 4), (5, 5)):
        return 10
    return 0


def called_mask(decl):
    if decl in PIPS:
        return sum(
            1 << i for i, d in enumerate(DOMINOES) if contains(d, decl)
        )
    if decl == "DT":
        return sum(
            1 << i for i, d in enumerate(DOMINOES) if is_double(d)
        )
    if decl == "NT":
        return 0
    raise ValueError(decl)


CALLED_MASK = {decl: called_mask(decl) for decl in DECLARATIONS}


def effective_mask(decl, context):
    if context == CALLED:
        return CALLED_MASK[decl]
    natural = sum(
        1 << i
        for i, d in enumerate(DOMINOES)
        if contains(d, context)
    )
    return natural & ~CALLED_MASK[decl]


EFFECTIVE = {
    (decl, q): effective_mask(decl, q)
    for decl in DECLARATIONS
    for q in range(8)
}


def led_context_index(decl, index):
    if CALLED_MASK[decl] & (1 << index):
        return CALLED
    return DOMINOES[index][0]


LED_CONTEXT = {
    (decl, i): led_context_index(decl, i)
    for decl in DECLARATIONS
    for i in range(28)
}

LEAD_FIBER = {
    (decl, q): sum(
        1 << i
        for i in range(28)
        if LED_CONTEXT[(decl, i)] == q
    )
    for decl in DECLARATIONS
    for q in range(8)
}


def rank_value(decl, index):
    h, l = DOMINOES[index]
    if decl == "DT" and h == l:
        return h
    if h == l and decl != "DT":
        return 100  # TOP
    return h + l


def trick_key(decl, context, index):
    bit = 1 << index
    if CALLED_MASK[decl] & bit:
        tier = 2
    elif EFFECTIVE[(decl, context)] & bit:
        tier = 1
    else:
        tier = 0
    return (tier, rank_value(decl, index) if tier else 0)


def trick_winner(decl, plays):
    """plays is a sequence of (seat, domino_index), with the lead first."""
    context = LED_CONTEXT[(decl, plays[0][1])]
    keys = [trick_key(decl, context, i) for _, i in plays]
    maximum = max(keys)
    if keys.count(maximum) != 1:
        raise AssertionError((decl, context, plays, keys))
    return plays[keys.index(maximum)][0]


def legal_indices(decl, hand_mask, current_plays):
    if not current_plays:
        return hand_mask
    context = LED_CONTEXT[(decl, current_plays[0][1])]
    followers = hand_mask & EFFECTIVE[(decl, context)]
    return followers if followers else hand_mask


def check_rules():
    assert len(DOMINOES) == 28
    assert len(set(DOMINOES)) == 28
    assert sum(count_label(d) for d in DOMINOES) == 35

    for pip in PIPS:
        assert sum(1 for d in DOMINOES if contains(d, pip)) == 7

    expected_contexts = {
        **{pip: (set(PIPS) - {pip}) | {CALLED} for pip in PIPS},
        "DT": {1, 2, 3, 4, 5, 6, CALLED},
        "NT": set(PIPS),
    }

    for decl in DECLARATIONS:
        contexts = {q for q in range(8) if LEAD_FIBER[(decl, q)]}
        assert contexts == expected_contexts[decl]

        sizes = sorted(
            LEAD_FIBER[(decl, q)].bit_count() for q in contexts
        )
        assert sizes == list(range(1, 8))

        union = 0
        for q in contexts:
            assert not (union & LEAD_FIBER[(decl, q)])
            union |= LEAD_FIBER[(decl, q)]
        assert union == ALL_MASK

    cases = 0
    for decl in DECLARATIONS:
        for lead in range(28):
            rest = [i for i in range(28) if i != lead]
            context = LED_CONTEXT[(decl, lead)]
            for others in combinations(rest, 3):
                keys = [
                    trick_key(decl, context, i)
                    for i in (lead,) + others
                ]
                assert keys.count(max(keys)) == 1
                cases += 1

    assert cases == 737_100
    return cases


def check_auction_rules():
    expected = (2380, 3060, 3196, 3213, 3214, 3214, 3214)

    @lru_cache(maxsize=None)
    def rec(turn, cap, high_kind, high_value):
        # high_kind: 0 none, 1 point bid, 2 mark bid.
        if turn == 4:
            return 1, high_value if high_kind == 2 else 0

        # Pass.
        actions = [(high_kind, high_value)]

        if high_kind < 2:
            first_point = 30 if high_kind == 0 else high_value + 1
            actions.extend((1, p) for p in range(first_point, 42))
            actions.extend(
                (2, m) for m in range(1, min(2, cap) + 1)
            )
        elif high_value + 1 <= cap:
            actions.append((2, high_value + 1))

        total = 0
        largest_mark = 0
        for kind, value in actions:
            count, reached = rec(turn + 1, cap, kind, value)
            total += count
            largest_mark = max(largest_mark, reached)
        return total, largest_mark

    observed = []
    for cap in range(1, 8):
        count, largest = rec(0, cap, 0, 0)
        observed.append(count)
        assert largest == min(cap, 5)

    assert tuple(observed) == expected
    return tuple(observed)


# ---------------------------------------------------------------------------
# Exact three-seat support algebra
# ---------------------------------------------------------------------------

def hall_feasible(universe, possible, capacities):
    universe = frozenset(universe)
    if len(universe) != sum(capacities):
        return False

    for seat_mask in range(1, 1 << len(capacities)):
        neighbors = set()
        quota = 0
        for seat in range(len(capacities)):
            if seat_mask & (1 << seat):
                neighbors.update(possible[seat] & universe)
                quota += capacities[seat]
        if len(neighbors) < quota:
            return False
    return True


def fiber_count_dp(universe, possible, capacities):
    order = tuple(sorted(universe))
    dp = {(0,) * len(capacities): 1}

    for d in order:
        nxt = defaultdict(int)
        for occupancy, value in dp.items():
            for seat in range(len(capacities)):
                if (
                    d in possible[seat]
                    and occupancy[seat] < capacities[seat]
                ):
                    new_occupancy = list(occupancy)
                    new_occupancy[seat] += 1
                    nxt[tuple(new_occupancy)] += value
        dp = nxt

    return dp.get(tuple(capacities), 0)


def enumerate_worlds(universe, possible, capacities):
    universe = tuple(sorted(universe))
    out = set()

    def rec(seat, remaining, hands):
        if seat == len(capacities) - 1:
            hand = frozenset(remaining)
            if (
                len(hand) == capacities[seat]
                and hand <= possible[seat]
            ):
                out.add(tuple(hands + [hand]))
            return

        allowed = sorted(set(remaining) & set(possible[seat]))
        for choice in combinations(allowed, capacities[seat]):
            chosen = frozenset(choice)
            rec(
                seat + 1,
                tuple(d for d in remaining if d not in chosen),
                hands + [chosen],
            )

    rec(0, universe, [])
    return frozenset(out)


def marginal_sets(universe, possible, capacities):
    if not hall_feasible(universe, possible, capacities):
        return None

    out = [set() for _ in capacities]

    for d in universe:
        for seat, capacity in enumerate(capacities):
            if capacity == 0 or d not in possible[seat]:
                continue

            successor_universe = frozenset(universe) - {d}
            successor_possible = tuple(
                frozenset(p) - {d} for p in possible
            )
            successor_capacities = list(capacities)
            successor_capacities[seat] -= 1

            if hall_feasible(
                successor_universe,
                successor_possible,
                tuple(successor_capacities),
            ):
                out[seat].add(d)

    return tuple(frozenset(values) for values in out)


def check_one_context_cell_lemma():
    """Exhaust the capacity/exclusion profiles used in the new families."""
    checked = 0

    profiles = (
        (5, 5, 6),
        (5, 5, 5),
        (4, 5, 5),
        (4, 4, 5),
        (4, 4, 4),
        (3, 4, 4),
        (3, 3, 4),
        (3, 3, 3),
    )

    for capacities in profiles:
        n = sum(capacities)
        universe = frozenset(range(n))

        for excluded_count in range(2, min(7, n) + 1):
            excluded = frozenset(range(excluded_count))
            nonexcluded = universe - excluded

            for membership_size in (1, 2):
                for membership in combinations(
                    range(3), membership_size
                ):
                    possible = tuple(
                        nonexcluded if seat in membership else universe
                        for seat in range(3)
                    )

                    if not hall_feasible(
                        universe, possible, capacities
                    ):
                        continue

                    marginal = marginal_sets(
                        universe, possible, capacities
                    )
                    assert marginal is not None

                    complement = frozenset(
                        seat
                        for seat in range(3)
                        if seat not in membership
                    )

                    for d in excluded:
                        holders = frozenset(
                            seat
                            for seat in range(3)
                            if d in marginal[seat]
                        )
                        assert holders == complement

                    checked += 1

    assert checked == 216
    return checked


def check_90_world_support_crosscheck():
    """Brute-force the corpus's 90-world no-trump endpoint and 12 updates."""
    decl = "NT"

    viewer_initial = frozenset(
        {
            (6, 3),
            (5, 0),
            (4, 0),
            (2, 1),
            (5, 1),
            (3, 1),
            (4, 1),
        }
    )

    history = [
        [(0, (6, 3)), (1, (6, 1)), (2, (6, 4)), (3, (6, 0))],
        [(2, (0, 0)), (3, (2, 2)), (0, (5, 0)), (1, (2, 0))],
        [(2, (4, 3)), (3, (4, 2)), (0, (4, 0)), (1, (5, 4))],
        [(1, (1, 1)), (2, (3, 0)), (3, (3, 3)), (0, (2, 1))],
        [(1, (1, 0)), (2, (6, 6)), (3, (5, 2)), (0, (5, 1))],
    ]

    unseen = frozenset(
        {(5, 5), (4, 4), (3, 2), (6, 5), (5, 3), (6, 2)}
    )

    played_by = {seat: set() for seat in range(4)}
    for trick in history:
        for seat, d in trick:
            played_by[seat].add(d)

    assert played_by[0] | {(3, 1), (4, 1)} == set(viewer_initial)

    worlds = set()
    ordered = sorted(unseen)

    for h1 in combinations(ordered, 2):
        rem1 = [d for d in ordered if d not in h1]
        for h2 in combinations(rem1, 2):
            h3 = tuple(d for d in rem1 if d not in h2)
            worlds.add(
                (frozenset(h1), frozenset(h2), frozenset(h3))
            )

    assert len(worlds) == 90

    endpoint_remainders = set()
    global_voids = {1: set(), 2: set(), 3: set()}

    for world in worlds:
        hands = {
            0: set(viewer_initial),
            1: set(world[0]) | played_by[1],
            2: set(world[1]) | played_by[2],
            3: set(world[2]) | played_by[3],
        }

        leader = 0
        local_voids = {1: set(), 2: set(), 3: set()}

        for trick in history:
            assert trick[0][0] == leader
            current = []

            for offset, (actor, d) in enumerate(trick):
                assert actor == (leader + offset) % 4
                index = INDEX[d]

                legal = legal_indices(
                    decl, tile_mask(hands[actor]), current
                )
                assert legal & (1 << index)

                if current and actor in HIDDEN:
                    context = LED_CONTEXT[
                        (decl, current[0][1])
                    ]
                    if not (
                        EFFECTIVE[(decl, context)] & (1 << index)
                    ):
                        local_voids[actor].add(context)

                hands[actor].remove(d)
                current.append((actor, index))

            leader = trick_winner(decl, current)

        assert hands[0] == {(3, 1), (4, 1)}

        endpoint_remainders.add(
            (
                frozenset(hands[1]),
                frozenset(hands[2]),
                frozenset(hands[3]),
            )
        )

        for seat in HIDDEN:
            global_voids[seat].update(local_voids[seat])

    assert endpoint_remainders == worlds
    assert global_voids == {1: set(), 2: {1}, 3: {0, 1}}

    possible = []
    for seat in HIDDEN:
        forbidden = 0
        for context in global_voids[seat]:
            forbidden |= EFFECTIVE[(decl, context)]

        possible.append(
            frozenset(
                d
                for d in unseen
                if not (forbidden & (1 << INDEX[d]))
            )
        )

    possible = tuple(possible)
    capacities = (2, 2, 2)

    assert all(p == unseen for p in possible)
    assert enumerate_worlds(
        unseen, possible, capacities
    ) == frozenset(worlds)
    assert fiber_count_dp(unseen, possible, capacities) == 90

    transition_checks = 0

    for viewer_lead in ((3, 1), (4, 1)):
        context = viewer_lead[0]
        follow = frozenset(
            d
            for d in unseen
            if EFFECTIVE[(decl, context)] & (1 << INDEX[d])
        )

        for d in unseen:
            brute_successors = set()

            for world in worlds:
                hand = world[0]
                legal = (
                    d in hand
                    and ((d in follow) if (hand & follow) else True)
                )
                if legal:
                    brute_successors.add(
                        (
                            frozenset(set(hand) - {d}),
                            world[1],
                            world[2],
                        )
                    )

            if not brute_successors:
                continue

            successor_universe = unseen - {d}
            first_possible = set(unseen)

            if d not in follow:
                first_possible.difference_update(follow)

            successor_possible = (
                frozenset(first_possible - {d}),
                frozenset(unseen - {d}),
                frozenset(unseen - {d}),
            )
            successor_capacities = (1, 2, 2)

            symbolic = enumerate_worlds(
                successor_universe,
                successor_possible,
                successor_capacities,
            )

            assert symbolic == frozenset(brute_successors)
            assert fiber_count_dp(
                successor_universe,
                successor_possible,
                successor_capacities,
            ) == len(brute_successors)

            reduced = marginal_sets(
                successor_universe,
                successor_possible,
                successor_capacities,
            )
            assert reduced is not None
            assert enumerate_worlds(
                successor_universe,
                reduced,
                successor_capacities,
            ) == symbolic

            transition_checks += 1

    assert transition_checks == 12
    return transition_checks


# ---------------------------------------------------------------------------
# Admissible complete-trick modules and transports
# ---------------------------------------------------------------------------

def admissible_groups(decl, size):
    groups = set()

    for context in range(8):
        lead = LEAD_FIBER[(decl, context)]
        if not lead:
            continue

        suit = EFFECTIVE[(decl, context)]
        members = tuple(bit_indices(suit))

        for choice in combinations(members, size):
            mask = sum(1 << i for i in choice)
            if mask & lead:
                groups.add(mask)

    return tuple(sorted(groups))


GROUPS = {
    decl: {
        size: admissible_groups(decl, size)
        for size in (2, 3, 4)
    }
    for decl in MODULE_DECLARATIONS
}


def check_complete_module_lemma():
    checked = 0

    for decl in MODULE_DECLARATIONS:
        for group in GROUPS[decl][4]:
            tiles = tuple(bit_indices(group))

            for desired in range(4):
                found = False

                for leader in range(4):
                    order = tuple(
                        (leader + offset) % 4
                        for offset in range(4)
                    )

                    for assignment in permutations(tiles):
                        plays = tuple(zip(order, assignment))
                        context = LED_CONTEXT[
                            (decl, plays[0][1])
                        ]

                        if all(
                            EFFECTIVE[(decl, context)]
                            & (1 << index)
                            for _, index in plays
                        ) and trick_winner(decl, plays) == desired:
                            found = True
                            break

                    if found:
                        break

                assert found
                checked += 1

    assert checked == 8 * 119 * 4
    return checked


def domino_edge(d):
    h, l = d
    if h == l:
        return h, K8_DOUBLE_VERTEX
    return h, l


def edge_domino(a, b):
    if a == K8_DOUBLE_VERTEX or b == K8_DOUBLE_VERTEX:
        pip = b if a == K8_DOUBLE_VERTEX else a
        return pip, pip
    return max(a, b), min(a, b)


def vertex_transport_mask(mask, permutation):
    result = 0
    for index in bit_indices(mask):
        a, b = domino_edge(DOMINOES[index])
        d = edge_domino(permutation[a], permutation[b])
        result |= 1 << INDEX[d]
    return result


def pip_transport_mask(mask, mapping):
    result = 0
    for index in bit_indices(mask):
        h, l = DOMINOES[index]
        d = (
            max(mapping[h], mapping[l]),
            min(mapping[h], mapping[l]),
        )
        result |= 1 << INDEX[d]
    return result


def check_module_transports():
    # Order-preserving pip-trump transports from declaration 0.
    for target in PIPS:
        mapping = {0: target}
        source_active = list(range(1, 7))
        target_active = sorted(set(PIPS) - {target})
        mapping.update(zip(source_active, target_active))

        assert pip_transport_mask(
            CALLED_MASK[0], mapping
        ) == CALLED_MASK[target]

        for size in (2, 3, 4):
            transported = {
                pip_transport_mask(mask, mapping)
                for mask in GROUPS[0][size]
            }
            assert transported == set(GROUPS[target][size])

        for position, target_context in enumerate(
            target_active, start=1
        ):
            source_context = position

            assert pip_transport_mask(
                EFFECTIVE[(0, source_context)], mapping
            ) == EFFECTIVE[(target, target_context)]

            assert pip_transport_mask(
                LEAD_FIBER[(0, source_context)], mapping
            ) == LEAD_FIBER[(target, target_context)]

            source_omitted = 1 << INDEX[(source_context, 0)]
            target_omitted = 1 << INDEX[
                (
                    max(target_context, target),
                    min(target_context, target),
                )
            ]
            assert pip_transport_mask(
                source_omitted, mapping
            ) == target_omitted

    # K8 swap 0 <-> double vertex maps declaration 0 to doubles trump
    # for the admissible-module language.
    permutation = list(range(8))
    permutation[0], permutation[K8_DOUBLE_VERTEX] = (
        K8_DOUBLE_VERTEX,
        0,
    )

    assert vertex_transport_mask(
        CALLED_MASK[0], permutation
    ) == CALLED_MASK["DT"]

    for size in (2, 3, 4):
        transported = {
            vertex_transport_mask(mask, permutation)
            for mask in GROUPS[0][size]
        }
        assert transported == set(GROUPS["DT"][size])

    return True


@lru_cache(maxsize=None)
def module_unions(decl, count):
    unions = {0}
    groups = GROUPS[decl][4]

    for _ in range(count):
        nxt = set()
        for used in unions:
            for group in groups:
                if not (used & group):
                    nxt.add(used | group)
        unions = nxt

    return frozenset(unions)


# ---------------------------------------------------------------------------
# Exact meet-in-the-middle upward-closure counter
# ---------------------------------------------------------------------------

HALF = 14
NHALF = 1 << HALF
LOW_MASK = NHALF - 1


def make_target_masks():
    masks = []

    for index in range(HALF):
        block = 1 << index
        chunk = (1 << block) - 1
        mask = 0

        for start in range(0, NHALF, 2 * block):
            mask |= chunk << (start + block)

        masks.append(mask)

    return tuple(masks)


TARGET_MASKS = make_target_masks()

POPCOUNT_MASKS = [0] * (HALF + 1)
for high in range(NHALF):
    POPCOUNT_MASKS[high.bit_count()] |= 1 << high


def witness_superset_rows(witnesses):
    """Boolean upward closure of 28-bit witness masks, split 14+14."""
    rows = [0] * NHALF

    for witness in witnesses:
        rows[witness & LOW_MASK] |= 1 << (witness >> HALF)

    # Low-half subset zeta.
    for index in range(HALF):
        bit = 1 << index
        for low in range(NHALF):
            if low & bit:
                rows[low] |= rows[low ^ bit]

    # High-half superset zeta, bit-parallel.
    for low, value in enumerate(rows):
        for index in range(HALF):
            value |= (
                value << (1 << index)
            ) & TARGET_MASKS[index]
        rows[low] = value

    return rows


@lru_cache(maxsize=None)
def high_category_masks(suit_high, forbidden_high):
    categories = [[0] * 8 for _ in range(15)]

    for high in range(NHALF):
        if high & forbidden_high:
            continue

        categories[
            high.bit_count()
        ][
            (high & suit_high).bit_count()
        ] |= 1 << high

    return tuple(tuple(row) for row in categories)


def count_from_rows(
    rows,
    total_size,
    suit_mask,
    allowed_suit_counts,
    forbidden_mask=0,
):
    suit_low = suit_mask & LOW_MASK
    suit_high = suit_mask >> HALF
    forbidden_low = forbidden_mask & LOW_MASK
    forbidden_high = forbidden_mask >> HALF

    categories = high_category_masks(
        suit_high, forbidden_high
    )

    total = 0

    for low, row in enumerate(rows):
        if low & forbidden_low:
            continue

        high_popcount = total_size - low.bit_count()
        if not 0 <= high_popcount <= HALF:
            continue

        low_suit_count = (low & suit_low).bit_count()
        allowed = 0

        for suit_count in allowed_suit_counts:
            high_suit_count = suit_count - low_suit_count

            if 0 <= high_suit_count < 8:
                allowed |= categories[
                    high_popcount
                ][
                    high_suit_count
                ]

        total += (row & allowed).bit_count()

    return total


# ---------------------------------------------------------------------------
# No-void lower family
# ---------------------------------------------------------------------------

def build_no_void_witnesses(pattern):
    witnesses = set()

    for decl in MODULE_DECLARATIONS:
        lists = [GROUPS[decl][size] for size in pattern]

        def rec(position, used):
            if position == len(lists):
                witnesses.add(used)
                return

            for group in lists[position]:
                if not (group & used):
                    rec(position + 1, used | group)

        rec(0, 0)

    return witnesses


def coverage_counts(pattern, target_sizes):
    witnesses = build_no_void_witnesses(pattern)
    rows = witness_superset_rows(witnesses)
    counts = {}

    for total_size in target_sizes:
        value = 0

        for low, row in enumerate(rows):
            high_popcount = total_size - low.bit_count()

            if 0 <= high_popcount <= HALF:
                value += (
                    row & POPCOUNT_MASKS[high_popcount]
                ).bit_count()

        counts[total_size] = value

    return len(witnesses), counts


def check_no_void_lower():
    expected = {
        (4, 2): (
            25_584,
            {12: 30_402_400},
        ),
        (4, 3): (
            36_128,
            {12: 30_294_577},
        ),
        (4, 4): (
            19_394,
            {
                13: 34_115_923,
                14: 39_546_166,
            },
        ),
        (4, 4, 2): (
            944_482,
            {15: 37_400_509},
        ),
        (4, 4, 3): (
            985_332,
            {
                15: 37_241_110,
                16: 30_419_732,
            },
        ),
        (4, 4, 4): (
            381_140,
            {17: 21_408_593},
        ),
    }

    actual = {}

    for pattern, (
        expected_witnesses,
        expected_counts,
    ) in expected.items():
        witness_count, counts = coverage_counts(
            pattern, tuple(expected_counts)
        )

        assert witness_count == expected_witnesses
        assert counts == expected_counts

        actual[pattern] = (witness_count, counts)

    early = (
        comb(28, 7)
        + 3 * comb(28, 8)
        + 3 * comb(28, 9)
        + comb(28, 10)
        + 3 * comb(28, 11)
    )

    assert early == 108_774_705

    total = early
    total += (
        2 * actual[(4, 2)][1][12]
        + actual[(4, 3)][1][12]
    )
    total += actual[(4, 4)][1][13]
    total += 3 * actual[(4, 4)][1][14]
    total += (
        2 * actual[(4, 4, 2)][1][15]
        + actual[(4, 4, 3)][1][15]
    )
    total += actual[(4, 4, 3)][1][16]
    total += 3 * actual[(4, 4, 4)][1][17]

    assert total == 559_316_142
    return total


# ---------------------------------------------------------------------------
# One-context void languages
# ---------------------------------------------------------------------------

def extra_masks(follow_mask, lead_mask, kind):
    followers = tuple(bit_indices(follow_mask))
    leaders = set(bit_indices(lead_mask))
    outsiders = tuple(bit_indices(ALL_MASK ^ follow_mask))
    masks = []

    if kind == "pair":
        for leader in leaders:
            for outsider in outsiders:
                masks.append(
                    (1 << leader) | (1 << outsider)
                )

    elif kind == "m1":
        for a, b in combinations(followers, 2):
            if a not in leaders and b not in leaders:
                continue

            pair = (1 << a) | (1 << b)

            for outsider in outsiders:
                masks.append(pair | (1 << outsider))

    elif kind == "m2":
        for leader in leaders:
            lead_bit = 1 << leader

            for a, b in combinations(outsiders, 2):
                masks.append(
                    lead_bit | (1 << a) | (1 << b)
                )

    else:
        raise ValueError(kind)

    return tuple(masks)


def contextual_witnesses(
    decl,
    module_count,
    follow_mask,
    lead_mask,
    kind,
):
    witnesses = set()
    extras = extra_masks(
        follow_mask, lead_mask, kind
    )

    for modules in module_unions(decl, module_count):
        for extra in extras:
            if not (modules & extra):
                witnesses.add(modules | extra)

    return witnesses


CALLED_VOID_GROUPS = (
    (
        1,
        "pair",
        (
            (
                "C12_hidden_pair",
                12,
                (1, 2, 3, 4, 5),
                23_966_810,
                3,
            ),
        ),
    ),
    (
        1,
        "m1",
        (
            (
                "C12_viewer_m1",
                12,
                (2, 3, 4, 5),
                19_940_291,
                1,
            ),
            (
                "C13_m1",
                13,
                (2, 3, 4, 5),
                29_359_456,
                3,
            ),
            (
                "C14_m1",
                14,
                (2, 3, 4, 5),
                34_711_089,
                6,
            ),
        ),
    ),
    (
        1,
        "m2",
        (
            (
                "C12_viewer_m2",
                12,
                (1, 2, 3, 4, 5),
                23_966_810,
                1,
            ),
            (
                "C13_m2",
                13,
                (2, 3, 4, 5),
                31_177_027,
                3,
            ),
            (
                "C14_m2",
                14,
                (3, 4, 5),
                29_851_710,
                3,
            ),
        ),
    ),
    (
        2,
        "pair",
        (
            (
                "C15_hidden_pair",
                15,
                (1, 2, 3, 4, 5),
                26_574_471,
                3,
            ),
        ),
    ),
    (
        2,
        "m1",
        (
            (
                "C15_viewer_m1",
                15,
                (2, 3, 4, 5),
                20_479_830,
                1,
            ),
            (
                "C16_m1",
                16,
                (2, 3, 4, 5),
                21_582_309,
                3,
            ),
            (
                "C17_m1",
                17,
                (2, 3, 4, 5),
                17_212_461,
                6,
            ),
        ),
    ),
    (
        2,
        "m2",
        (
            (
                "C15_viewer_m2",
                15,
                (2, 3, 4, 5),
                25_760_616,
                1,
            ),
            (
                "C16_m2",
                16,
                (3, 4, 5),
                22_693_062,
                3,
            ),
            (
                "C17_m2",
                17,
                (4, 5),
                13_118_238,
                3,
            ),
        ),
    ),
    (
        3,
        "pair",
        (
            (
                "C18_hidden_pair",
                18,
                (1, 2, 3, 4, 5),
                8_905_344,
                3,
            ),
        ),
    ),
    (
        3,
        "m1",
        (
            (
                "C19_m1",
                19,
                (2, 3, 4, 5),
                4_114_740,
                3,
            ),
        ),
    ),
    (
        3,
        "m2",
        (
            (
                "C19_m2",
                19,
                (4, 5),
                4_233_495,
                3,
            ),
        ),
    ),
)


def check_called_void_lower():
    follow = CALLED_MASK[0]
    lead = follow
    subtotal = 0

    for module_count, kind, queries in CALLED_VOID_GROUPS:
        witnesses = contextual_witnesses(
            0,
            module_count,
            follow,
            lead,
            kind,
        )
        rows = witness_superset_rows(witnesses)

        for (
            name,
            total_size,
            suit_counts,
            expected_count,
            categories_per_declaration,
        ) in queries:
            count = count_from_rows(
                rows,
                total_size,
                follow,
                suit_counts,
            )

            assert count == expected_count, (
                name,
                count,
                expected_count,
            )

            subtotal += (
                8
                * categories_per_declaration
                * count
            )

        del rows, witnesses
        gc.collect()

    assert subtotal == 8_387_350_664
    return subtotal


NATURAL_GROUPS = (
    (
        1,
        "pair",
        (
            (
                "N12_hidden_pair",
                12,
                (1, 2, 3, 4),
                (
                    5_760_594,
                    9_210_738,
                    11_360_129,
                    12_660_550,
                    13_404_689,
                    13_812_747,
                ),
                3,
            ),
        ),
    ),
    (
        1,
        "m1",
        (
            (
                "N12_viewer_m1",
                12,
                (2, 3, 4),
                (
                    5_205_424,
                    8_170_230,
                    9_851_930,
                    10_700_937,
                    11_015_340,
                    11_015_340,
                ),
                1,
            ),
            (
                "N13_m1",
                13,
                (2, 3, 4),
                (
                    7_491_473,
                    11_679_933,
                    13_902_336,
                    14_925_464,
                    15_273_873,
                    15_273_873,
                ),
                3,
            ),
        ),
    ),
    (
        1,
        "m2",
        (
            (
                "N12_viewer_m2",
                12,
                (1, 2, 3, 4),
                (
                    5_760_594,
                    9_210_738,
                    11_360_129,
                    12_660_550,
                    13_404_689,
                    13_812_747,
                ),
                1,
            ),
            (
                "N13_m2",
                13,
                (1, 2, 3, 4),
                (
                    7_908_238,
                    12_476_855,
                    15_067_893,
                    16_446_391,
                    17_134_605,
                    17_458_845,
                ),
                3,
            ),
        ),
    ),
    (
        2,
        "m1",
        (
            (
                "N15_viewer_m1",
                15,
                (2, 3, 4),
                (
                    5_711_043,
                    8_584_043,
                    10_057_926,
                    10_705_653,
                    10_917_124,
                    10_917_124,
                ),
                1,
            ),
            (
                "N16_m1",
                16,
                (2, 3, 4),
                (
                    5_238_902,
                    7_878_214,
                    9_083_898,
                    9_529_061,
                    9_652_578,
                    9_652_578,
                ),
                3,
            ),
        ),
    ),
    (
        2,
        "m2",
        (
            (
                "N15_viewer_m2",
                15,
                (1, 2, 3, 4),
                (
                    6_175_761,
                    9_384_409,
                    11_150_343,
                    12_046_854,
                    12_471_751,
                    12_662_139,
                ),
                1,
            ),
            (
                "N16_m2",
                16,
                (2, 3, 4),
                (
                    5_318_483,
                    8_010_800,
                    9_254_244,
                    9_723_064,
                    9_860_356,
                    9_869_049,
                ),
                3,
            ),
        ),
    ),
)


def check_natural_void_lower():
    # There are seven ordered (trump, context) pairs for each
    # natural lead-fiber size 1,...,6.
    histogram = defaultdict(int)

    for trump in PIPS:
        for context in PIPS:
            if context != trump:
                size = LEAD_FIBER[
                    (trump, context)
                ].bit_count()
                histogram[size] += 1

    assert dict(histogram) == {
        size: 7 for size in range(1, 7)
    }

    subtotal = 0

    # Declaration 0, natural contexts 1,...,6, are representatives
    # for the six lead-fiber sizes.
    for module_count, kind, queries in NATURAL_GROUPS:
        sums = [0] * len(queries)

        for context in range(1, 7):
            follow = EFFECTIVE[(0, context)]
            lead = LEAD_FIBER[(0, context)]

            # Under zeroes trump, context-q omits the called edge q:0.
            omitted = 1 << INDEX[(context, 0)]

            witnesses = contextual_witnesses(
                0,
                module_count,
                follow,
                lead,
                kind,
            )
            rows = witness_superset_rows(witnesses)

            for query_index, query in enumerate(queries):
                (
                    name,
                    total_size,
                    suit_counts,
                    expected_by_size,
                    categories_per_context,
                ) = query

                count = count_from_rows(
                    rows,
                    total_size,
                    follow,
                    suit_counts,
                    omitted,
                )

                expected = expected_by_size[context - 1]

                assert count == expected, (
                    name,
                    context,
                    count,
                    expected,
                )

                sums[query_index] += 7 * count

            del rows, witnesses
            gc.collect()

        for query, sum_over_42_contexts in zip(
            queries, sums
        ):
            categories_per_context = query[4]
            subtotal += (
                categories_per_context
                * sum_over_42_contexts
            )

    assert subtotal == 8_721_399_239
    return subtotal


# ---------------------------------------------------------------------------
# Existing outer-language anchors
# ---------------------------------------------------------------------------

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


def hidden_capacity_profiles():
    return tuple(
        capacities
        for capacities in product(range(8), repeat=3)
        if max(capacities) - min(capacities) <= 1
    )


def profile_parameters(capacities):
    n = sum(capacities)

    if (
        capacities[0]
        == capacities[1]
        == capacities[2]
    ):
        return n, 7 - capacities[0], 0

    high = max(capacities)

    low_seats = frozenset(
        index
        for index, value in enumerate(capacities)
        if value == high - 1
    )

    return (
        n,
        7 - high,
        len(FOLLOWER_MAXIMUM[low_seats]),
    )


def polynomial_multiply(a, b):
    result = [0] * (len(a) + len(b) - 1)

    for i, left in enumerate(a):
        for j, right in enumerate(b):
            result[i + j] += left * right

    return result


def lead_witness_counts():
    counts = [[0] * 8 for _ in range(29)]

    for used_mask in range(1 << 7):
        used = used_mask.bit_count()
        polynomial = [1]

        for index, size in enumerate(range(1, 8)):
            factor = [
                comb(size, selected)
                for selected in range(size + 1)
            ]

            if used_mask & (1 << index):
                factor[-1] -= 1

            polynomial = polynomial_multiply(
                polynomial, factor
            )

        for selected, coefficient in enumerate(polynomial):
            counts[selected][used] += coefficient

    for selected in range(29):
        assert counts[selected][0] == comb(28, selected)

    return tuple(tuple(row) for row in counts)


def outer_count_for_profile(capacities, witness_counts):
    if capacities == (0, 0, 0):
        return 1

    n, completed, follower_count = profile_parameters(
        capacities
    )

    total = sum(
        7**used * witness_counts[n][used]
        for used in range(completed + 1)
    )

    if follower_count:
        used = completed + 1
        total += (
            7**used
            - (8 - 2**follower_count) ** used
        ) * witness_counts[n][used]

    return total


def check_corpus_anchors():
    floor = (
        comb(28, 21)
        + 3 * comb(28, 20)
        + 3 * comb(28, 19)
        + comb(28, 18)
    )

    assert floor == 44_352_165

    profiles = hidden_capacity_profiles()
    assert len(profiles) == 50

    witness_counts = lead_witness_counts()

    per_declaration = sum(
        outer_count_for_profile(
            capacities, witness_counts
        )
        for capacities in profiles
    )

    tagged = 9 * per_declaration

    assert per_declaration == 7_124_838_074_989
    assert tagged == 64_123_542_674_901
    assert tagged < 2**46

    return floor, per_declaration, tagged


# ---------------------------------------------------------------------------
# Driver
# ---------------------------------------------------------------------------

def run_check(name, function):
    try:
        value = function()
    except Exception as exc:
        print(
            f"FAIL {name} {type(exc).__name__}: {exc}"
        )
        return False, None

    print(
        f"PASS {name}"
        + (f" {value}" if value is not None else "")
    )
    return True, value


def main():
    started = time.time()
    ok = True
    values = {}

    checks = (
        ("natural_void_lower", check_natural_void_lower),
        ("called_void_lower", check_called_void_lower),
        ("no_void_lower", check_no_void_lower),
        ("rules", check_rules),
        ("auction", check_auction_rules),
        (
            "support_crosscheck_90world",
            check_90_world_support_crosscheck,
        ),
        ("module_transports", check_module_transports),
        (
            "complete_module_lemma",
            check_complete_module_lemma,
        ),
        (
            "one_context_cell_lemma",
            check_one_context_cell_lemma,
        ),
        ("corpus_anchors", check_corpus_anchors),
    )

    for name, function in checks:
        passed, value = run_check(name, function)
        ok &= passed
        values[name] = value

    if not ok:
        return 1

    lower = (
        values["no_void_lower"]
        + values["called_void_lower"]
        + values["natural_void_lower"]
    )

    _, per_declaration, tagged = values["corpus_anchors"]

    assert lower == 17_668_066_045
    assert 2**34 < lower < 2**35
    assert per_declaration == 7_124_838_074_989
    assert tagged == 64_123_542_674_901
    assert tagged < 2**46

    print(f"PASS certified_disjoint_lower {lower}")
    print("PASS headline INTERVAL [35,46] bits")
    print(
        f"PASS runtime_seconds "
        f"{time.time() - started:.3f}"
    )

    return 0


if __name__ == "__main__":
    sys.exit(main())
```

### Proof

1. **Source objects and notation.**
   [USES: normative rules profile; definitions of (U,k,P_s,\Phi); CELL-05; CELL-14.]
   Fix the viewer-relative hidden seats (J={1,2,3}). Write (T=\mathcal D\setminus U). Because every reachable support is nonempty, CELL-14 implies that its normal form recovers its exact fiber; the nonempty-support recovery theorem then recovers (U), the labeled capacity triple (k), and every marginal holder set. Consequently, supports with different (U), different labeled (k), or different marginal holder relations are distinct members of (R).

2. **The eight full stars.**
   [USES: domino-incidence definition; doubles definition.]
   Represent a mixed domino (p:q) as the edge (pq) of a graph on vertices ({0,\ldots,6,\mathrm D}), and represent the double (p:p) by the edge (p\mathrm D). The seven pip-incidence sets and the doubles set are then the eight vertex stars:
   [
   \mathcal S={\sigma_0,\ldots,\sigma_6,\mathcal D^\circ}.
   ]
   Each has seven tiles, every domino belongs to exactly two such stars, and two distinct stars meet in exactly one domino. Each (S\in\mathcal S) is the called suit of one declaration in ({0,\ldots,6,\mathrm{DT}}).

3. **Admissible modules.**
   [USES: led-context definition; follow relation; unique-winner theorem.]
   For a fixed declaration, an admissible (r)-group is an (r)-tile set contained in one effective follow set and containing at least one tile from that context’s lead fiber. An admissible four-group can be played as a complete trick: assign a lead-fiber tile to the leader and the remaining three tiles to the followers. Every follower follows.

   Let (x) be the unique maximum-key tile of the group in that context. For any desired next leader (w), assign (x) to (w). If (x) is itself leadable, make (w) the current leader. Otherwise assign a leadable group tile to any different current leader. The four plays are legal and (w) wins. The program independently exhausts all (8\cdot119\cdot4=3,808) declaration/group/desired-winner cases.

4. **Backward stitching of modules.**
   [USES: step 3; auction hand-independence; bidder leads trick 1.]
   Suppose a later fragment must be led by seat (w_j). Apply step 3 to the preceding module, choosing an assignment whose winner is (w_j); this determines some leader (w_{j-1}) for that module. Repeat backward. The first resulting leader can be made the bidder through a legal auction because bid legality is hand-independent. Thus any disjoint sequence of admissible modules can be followed by any fragment whose required leader was specified in advance.

5. **One-context cells.**
   [USES: CELL-05; Hall feasibility; marginal-holder definition.]
   Let (X\subseteq U), (N=U\setminus X), and let (M\subset J) be nonempty with (|M|\le2). Consider
   [
   P_s=
   \begin{cases}
   N,&s\in M,\
   U,&s\notin M.
   \end{cases}
   ]
   This is the cell system obtained when exactly the seats in (M) slough in one context whose surviving intersection with (U) is (X).

   If (M={r}), the tabled ranges satisfy (|N|\ge k_r+1) and (X) fits in the two unrestricted seats. Directly assigning a selected tile and partitioning the remaining homogeneous sets proves
   [
   A(d)=
   \begin{cases}
   J\setminus{r},&d\in X,\
   J,&d\in N.
   \end{cases}
   ]

   If (M=J\setminus{u}), all of (X) is forced to (u), and the tabled ranges satisfy (|X|\le k_u). Thus
   [
   A(d)={u}\quad(d\in X).
   ]
   Every (d\in N) is possible at both seats in (M), and it is also possible at (u) exactly when (k_u>|X|). The equality case produces the expected binary ambiguity on (N). The program exhausts all 216 capacity/exclusion profiles used below and verifies these marginal holder descriptions.

6. **Disjointness of called-suit families.**
   [USES: steps 1, 2, 5.]
   In every called-suit family, (X=U\cap S) and (|X|\ge2). Since distinct full stars meet in only one tile, (X) identifies (S). Step 5 then identifies (M) from the marginal holder relation. Step 1 identifies (U) and (k). Hence two counted called-suit objects coincide only when all their table coordinates coincide. They are also distinct from unrestricted supports because (X\ne\varnothing).

7. **Disjointness of natural-suit families.**
   [USES: effective-suit algebra; steps 1 and 5.]
   For ordered distinct pips (t,q), under pip-trump declaration (t), define
   [
   E_{t,q}=\sigma_q\setminus{q:t},\qquad e_{t,q}=q:t.
   ]
   The natural families require
   [
   e_{t,q}\in U,\qquad X=U\cap E_{t,q},\qquad |X|\ge2.
   ]
   Therefore
   [
   U\cap\sigma_q=X\sqcup{e_{t,q}}.
   ]
   Any two distinct tiles of (X) have the unique common natural pip (q), so the support identifies (q). The unique extra tile in ((U\cap\sigma_q)\setminus X) identifies (e_{t,q}), hence (t). Step 5 identifies (M), and step 1 identifies (U,k). Thus different ordered natural contexts, seat memberships, pools, or capacity labels do not collide.

   These supports also do not collide with the called-star family at (q): the called-star support forbids the additional edge (e_{t,q}) at seats in (M), whereas the natural support permits it. A different called star cannot contain the two-tile set (X).

8. **Trace templates.**
   [USES: steps 3–5; follow-if-possible rule; CELL-05.]
   The generated witnesses use these templates:

   * `pair`: after the modules, a hidden leader plays a lead-fiber tile and the next hidden seat sloughs an outsider. This creates one hidden void.
   * `m1`: a complete or viewer-led fragment contains two following tiles, at least one leadable, and one outsider. Exactly one hidden seat sloughs.
   * `m2`: a complete or viewer-led fragment contains one leadable following tile and two outsiders. Exactly two hidden seats slough.
   * In a complete void trick, the viewer chooses any legal tile from its known remaining hand. Such a tile always exists. Viewer following or sloughing does not alter hidden-seat cells.
   * In a viewer-led fragment, the viewer leads the selected lead-fiber tile.
   * In the called-suit singleton-low rows, the low seat leads the void trick with the strongest available called tile, wins, and then makes the additional current-trick lead. Every other played called tile ranks below it.

   The remaining tiles of (T) form the viewer’s initial seven-tile hand after the hidden public tiles are assigned. Any Hall-feasible world of the step-5 cell supplies the hidden remaining hands. At every hidden void turn, that seat’s current hand consists only of (N)-tiles plus its outsider, so the slough is legal. Every other hidden follower plays a following tile. CELL-05 therefore gives exactly the claimed one-context support.

9. **Exact witness-mask generation.**
   [USES: steps 3, 4, and 8.]
   For each declaration and template, the program recursively selects pairwise-disjoint admissible modules and one final `pair`, `m1`, or `m2` fragment. It stores only the union mask. A stored mask is therefore equivalent to the existence of a decomposition into legal stitched modules and the indicated fragment.

10. **Exact upward-closure count.**
    [USES: step 9.]
    Split each 28-bit mask as (T=(L,H)) with two 14-bit halves. Initially, `rows[L]` has bit (H) set exactly for generated witness masks.

    The low-half subset zeta transform replaces `rows[L]` by the union of rows for all witness low halves contained in (L). The high-half bit-parallel superset transform then sets bit (H) exactly when some corresponding witness high half is contained in (H). Hence after both transforms:
    [
    \texttt{rows[L] has bit H}
    \iff
    T=(L,H)\text{ contains at least one generated witness}.
    ]
    Intersecting with exact total-popcount masks counts each qualifying (T) once, not once per witness. The suit-popcount masks impose the displayed (X)-size and feasibility ranges. Natural rows additionally forbid (e_{t,q}) from (T), which is exactly (e_{t,q}\in U).

11. **No-void count.**
    [USES: REACH-13 for the original (T=7,8,9,10) rows; steps 2–4 and 10.]
    The existing four rows contribute (44,352,165).

    For (|T|=11), the 11 K8 edges have 22 incidences among eight stars, so some star contains at least three of them. Use the strongest such tile at the desired hidden winner, two more at the other hidden seats, and lead the called suit from a hidden seat. If a fourth called tile is present, the viewer follows with one; otherwise all called tiles are hidden and the viewer is void. The winning hidden seat then makes one additional lead. This reaches every pool for each labeling of ((5,6,6)), contributing (3\binom{28}{11}=64,422,540).

    The exact module-coverage rows in the JSON table contribute the remaining (450,541,437). Therefore the unrestricted reachable family contains exactly
    [
    559,316,142
    ]
    distinct supports.

12. **Called-suit void count.**
    [USES: steps 2–6 and 8–10.]
    The K8 transport checked by the program maps the declaration-0 admissible-module language to each of the other six pip-trump languages and, by the vertex swap (0\leftrightarrow\mathrm D), to doubles trump. Thus each tabled per-category count occurs for eight called suits. Multiplying by the explicit seat/capacity category multiplicities gives
    [
    8,387,350,664
    ]
    distinct reachable supports.

13. **Natural-suit transport and count.**
    [USES: pip-trump mechanics transport; steps 5, 7–10.]
    For each trump (t), order the six nontrump pips increasingly. The order-preserving pip transport maps declaration 0, natural context (i), to the (i)-th natural context under declaration (t), preserving modules, effective follow membership, lead-fiber membership, and the omitted edge. Consequently the count depends only on the lead-fiber size (i\in{1,\ldots,6}). There are exactly seven ordered ((t,q)) contexts of each size. The program verifies these transports and the (7,7,7,7,7,7) histogram.

    Summing the exact six-entry rows and their category multiplicities gives
    [
    8,721,399,239
    ]
    additional distinct reachable supports.

14. **Certified disjoint lower bound.**
    [USES: steps 6, 7, 11–13.]
    The three subfamilies are pairwise disjoint and have total
    [
    \begin{aligned}
    L
    &=559,316,142\
    &\quad+8,387,350,664\
    &\quad+8,721,399,239\
    &=17,668,066,045.
    \end{aligned}
    ]
    Since
    [
    2^{34}=17,179,869,184<L,
    ]
    it follows that
    [
    \left\lceil\log_2|R|\right\rceil\ge35.
    ]

15. **Upper bound.**
    [USES: REACH-11/12/13 outer-certificate theorem; program’s independent recomputation.]
    The attachment proves and the program recomputes
    [
    \sum_k C(k)=7,124,838,074,989
    ]
    for one supplied declaration and
    [
    9\sum_k C(k)=64,123,542,674,901<2^{46}.
    ]
    Every reachable support has a declaration-tagged outer certificate, so
    [
    |R|<2^{46}
    \quad\Longrightarrow\quad
    \left\lceil\log_2|R|\right\rceil\le46.
    ]

16. **Conclusion.**
    [USES: steps 14 and 15.]
    Therefore
    [
    \boxed{
    35\le
    \left\lceil\log_2|R|\right\rceil
    \le46
    }.
    ]

### Explicit JSON tables and witness

```json
{
  "headline": {
    "no_void_reachable": 559316142,
    "called_suit_one_void_reachable": 8387350664,
    "natural_suit_one_void_reachable": 8721399239,
    "certified_disjoint_total": 17668066045,
    "two_to_34": 17179869184,
    "outer_certificate_total": 64123542674901,
    "bit_interval": [35, 46]
  }
}
```

```json
{
  "no_void_table": [
    {
      "T_size": 7,
      "capacity_profiles": [[7, 7, 7]],
      "count": 1184040,
      "construction": "initial support"
    },
    {
      "T_size": 8,
      "capacity_orbit": [6, 7, 7],
      "labelings": 3,
      "count": 9324315,
      "construction": "one hidden lead"
    },
    {
      "T_size": 9,
      "capacity_orbit": [6, 6, 7],
      "labelings": 3,
      "count": 20720700,
      "construction": "two-play fragment"
    },
    {
      "T_size": 10,
      "capacity_profiles": [[6, 6, 6]],
      "count": 13123110,
      "construction": "three hidden plays"
    },
    {
      "T_size": 11,
      "capacity_orbit": [5, 6, 6],
      "labelings": 3,
      "count": 64422540,
      "construction": "one called-suit trick and one hidden lead"
    },
    {
      "T_size": 12,
      "capacity_orbit": [5, 5, 6],
      "adjacent_labelings": 2,
      "adjacent_count_each": 30402400,
      "separated_labelings": 1,
      "separated_count_each": 30294577,
      "count": 91099377
    },
    {
      "T_size": 13,
      "capacity_profiles": [[5, 5, 5]],
      "count": 34115923
    },
    {
      "T_size": 14,
      "capacity_orbit": [4, 5, 5],
      "labelings": 3,
      "count_each": 39546166,
      "count": 118638498
    },
    {
      "T_size": 15,
      "capacity_orbit": [4, 4, 5],
      "adjacent_labelings": 2,
      "adjacent_count_each": 37400509,
      "separated_labelings": 1,
      "separated_count_each": 37241110,
      "count": 112042128
    },
    {
      "T_size": 16,
      "capacity_profiles": [[4, 4, 4]],
      "count": 30419732
    },
    {
      "T_size": 17,
      "capacity_orbit": [3, 4, 4],
      "labelings": 3,
      "count_each": 21408593,
      "count": 64225779
    }
  ],
  "no_void_total": 559316142
}
```

```json
{
  "no_void_module_coverage": [
    {
      "pattern": [4, 2],
      "witness_union_masks": 25584,
      "target_T_size": 12,
      "covered_T_masks": 30402400
    },
    {
      "pattern": [4, 3],
      "witness_union_masks": 36128,
      "target_T_size": 12,
      "covered_T_masks": 30294577
    },
    {
      "pattern": [4, 4],
      "witness_union_masks": 19394,
      "coverage": {
        "13": 34115923,
        "14": 39546166
      }
    },
    {
      "pattern": [4, 4, 2],
      "witness_union_masks": 944482,
      "target_T_size": 15,
      "covered_T_masks": 37400509
    },
    {
      "pattern": [4, 4, 3],
      "witness_union_masks": 985332,
      "coverage": {
        "15": 37241110,
        "16": 30419732
      }
    },
    {
      "pattern": [4, 4, 4],
      "witness_union_masks": 381140,
      "target_T_size": 17,
      "covered_T_masks": 21408593
    }
  ]
}
```

```json
{
  "called_suit_void_table": [
    {
      "name": "C12_hidden_pair",
      "T_size": 12,
      "modules": 1,
      "fragment": "pair",
      "per_category": 23966810,
      "categories_per_declaration": 3,
      "declarations": 8,
      "contribution": 575203440
    },
    {
      "name": "C12_viewer_m1",
      "T_size": 12,
      "modules": 1,
      "fragment": "viewer_m1",
      "per_category": 19940291,
      "categories_per_declaration": 1,
      "declarations": 8,
      "contribution": 159522328
    },
    {
      "name": "C12_viewer_m2",
      "T_size": 12,
      "modules": 1,
      "fragment": "viewer_m2",
      "per_category": 23966810,
      "categories_per_declaration": 1,
      "declarations": 8,
      "contribution": 191734480
    },
    {
      "name": "C13_m1",
      "T_size": 13,
      "modules": 1,
      "fragment": "complete_void_trick_m1",
      "per_category": 29359456,
      "categories_per_declaration": 3,
      "declarations": 8,
      "contribution": 704626944
    },
    {
      "name": "C13_m2",
      "T_size": 13,
      "modules": 1,
      "fragment": "complete_void_trick_m2",
      "per_category": 31177027,
      "categories_per_declaration": 3,
      "declarations": 8,
      "contribution": 748248648
    },
    {
      "name": "C14_m1",
      "T_size": 14,
      "modules": 1,
      "fragment": "winning_void_trick_m1_then_lead",
      "per_category": 34711089,
      "categories_per_declaration": 6,
      "declarations": 8,
      "contribution": 1666132272
    },
    {
      "name": "C14_m2",
      "T_size": 14,
      "modules": 1,
      "fragment": "winning_void_trick_m2_then_lead",
      "per_category": 29851710,
      "categories_per_declaration": 3,
      "declarations": 8,
      "contribution": 716441040
    },
    {
      "name": "C15_hidden_pair",
      "T_size": 15,
      "modules": 2,
      "fragment": "pair",
      "per_category": 26574471,
      "categories_per_declaration": 3,
      "declarations": 8,
      "contribution": 637787304
    },
    {
      "name": "C15_viewer_m1",
      "T_size": 15,
      "modules": 2,
      "fragment": "viewer_m1",
      "per_category": 20479830,
      "categories_per_declaration": 1,
      "declarations": 8,
      "contribution": 163838640
    },
    {
      "name": "C15_viewer_m2",
      "T_size": 15,
      "modules": 2,
      "fragment": "viewer_m2",
      "per_category": 25760616,
      "categories_per_declaration": 1,
      "declarations": 8,
      "contribution": 206084928
    },
    {
      "name": "C16_m1",
      "T_size": 16,
      "modules": 2,
      "fragment": "complete_void_trick_m1",
      "per_category": 21582309,
      "categories_per_declaration": 3,
      "declarations": 8,
      "contribution": 517975416
    },
    {
      "name": "C16_m2",
      "T_size": 16,
      "modules": 2,
      "fragment": "complete_void_trick_m2",
      "per_category": 22693062,
      "categories_per_declaration": 3,
      "declarations": 8,
      "contribution": 544633488
    },
    {
      "name": "C17_m1",
      "T_size": 17,
      "modules": 2,
      "fragment": "winning_void_trick_m1_then_lead",
      "per_category": 17212461,
      "categories_per_declaration": 6,
      "declarations": 8,
      "contribution": 826198128
    },
    {
      "name": "C17_m2",
      "T_size": 17,
      "modules": 2,
      "fragment": "winning_void_trick_m2_then_lead",
      "per_category": 13118238,
      "categories_per_declaration": 3,
      "declarations": 8,
      "contribution": 314837712
    },
    {
      "name": "C18_hidden_pair",
      "T_size": 18,
      "modules": 3,
      "fragment": "pair",
      "per_category": 8905344,
      "categories_per_declaration": 3,
      "declarations": 8,
      "contribution": 213728256
    },
    {
      "name": "C19_m1",
      "T_size": 19,
      "modules": 3,
      "fragment": "complete_void_trick_m1",
      "per_category": 4114740,
      "categories_per_declaration": 3,
      "declarations": 8,
      "contribution": 98753760
    },
    {
      "name": "C19_m2",
      "T_size": 19,
      "modules": 3,
      "fragment": "complete_void_trick_m2",
      "per_category": 4233495,
      "categories_per_declaration": 3,
      "declarations": 8,
      "contribution": 101603880
    }
  ],
  "called_suit_void_total": 8387350664
}
```

```json
{
  "natural_suit_void_table": [
    {
      "name": "N12_hidden_pair",
      "lead_fiber_sizes": [1, 2, 3, 4, 5, 6],
      "counts": [5760594, 9210738, 11360129, 12660550, 13404689, 13812747],
      "sum_over_42_contexts": 463466129,
      "categories_per_context": 3,
      "contribution": 1390398387
    },
    {
      "name": "N12_viewer_m1",
      "lead_fiber_sizes": [1, 2, 3, 4, 5, 6],
      "counts": [5205424, 8170230, 9851930, 10700937, 11015340, 11015340],
      "sum_over_42_contexts": 391714407,
      "categories_per_context": 1,
      "contribution": 391714407
    },
    {
      "name": "N12_viewer_m2",
      "lead_fiber_sizes": [1, 2, 3, 4, 5, 6],
      "counts": [5760594, 9210738, 11360129, 12660550, 13404689, 13812747],
      "sum_over_42_contexts": 463466129,
      "categories_per_context": 1,
      "contribution": 463466129
    },
    {
      "name": "N13_m1",
      "lead_fiber_sizes": [1, 2, 3, 4, 5, 6],
      "counts": [7491473, 11679933, 13902336, 14925464, 15273873, 15273873],
      "sum_over_42_contexts": 549828664,
      "categories_per_context": 3,
      "contribution": 1649485992
    },
    {
      "name": "N13_m2",
      "lead_fiber_sizes": [1, 2, 3, 4, 5, 6],
      "counts": [7908238, 12476855, 15067893, 16446391, 17134605, 17458845],
      "sum_over_42_contexts": 605449789,
      "categories_per_context": 3,
      "contribution": 1816349367
    },
    {
      "name": "N15_viewer_m1",
      "lead_fiber_sizes": [1, 2, 3, 4, 5, 6],
      "counts": [5711043, 8584043, 10057926, 10705653, 10917124, 10917124],
      "sum_over_42_contexts": 398250391,
      "categories_per_context": 1,
      "contribution": 398250391
    },
    {
      "name": "N15_viewer_m2",
      "lead_fiber_sizes": [1, 2, 3, 4, 5, 6],
      "counts": [6175761, 9384409, 11150343, 12046854, 12471751, 12662139],
      "sum_over_42_contexts": 447238799,
      "categories_per_context": 1,
      "contribution": 447238799
    },
    {
      "name": "N16_m1",
      "lead_fiber_sizes": [1, 2, 3, 4, 5, 6],
      "counts": [5238902, 7878214, 9083898, 9529061, 9652578, 9652578],
      "sum_over_42_contexts": 357246617,
      "categories_per_context": 3,
      "contribution": 1071739851
    },
    {
      "name": "N16_m2",
      "lead_fiber_sizes": [1, 2, 3, 4, 5, 6],
      "counts": [5318483, 8010800, 9254244, 9723064, 9860356, 9869049],
      "sum_over_42_contexts": 364251972,
      "categories_per_context": 3,
      "contribution": 1092755916
    }
  ],
  "natural_suit_void_total": 8721399239
}
```

```json
{
  "category_schema": {
    "hidden_pair_profiles": [
      {"capacities": [5, 5, 6], "void_seats": [2]},
      {"capacities": [6, 5, 5], "void_seats": [3]},
      {"capacities": [5, 6, 5], "void_seats": [1]}
    ],
    "viewer_led_profile_at_T12": {
      "capacities": [5, 5, 6],
      "m1_void_seats": [1],
      "m2_void_seats": [1, 2]
    },
    "viewer_led_profile_at_T15": {
      "capacities": [4, 4, 5],
      "m1_void_seats": [1],
      "m2_void_seats": [1, 2]
    },
    "equal_profile_m1_memberships": [[1], [2], [3]],
    "equal_profile_m2_memberships": [[1, 2], [1, 3], [2, 3]],
    "singleton_low_m1_examples": [
      {"capacities": [4, 5, 5], "winning_low_seat": 1, "void_seats": [2]},
      {"capacities": [4, 5, 5], "winning_low_seat": 1, "void_seats": [3]}
    ],
    "singleton_low_m2_example": {
      "capacities": [4, 5, 5],
      "winning_low_seat": 1,
      "void_seats": [2, 3]
    }
  }
}
```

```json
{
  "brute_force_crosscheck_witness": {
    "declaration": "NT",
    "viewer": 0,
    "viewer_initial_hand": [
      [6, 3],
      [5, 0],
      [4, 0],
      [2, 1],
      [5, 1],
      [3, 1],
      [4, 1]
    ],
    "public_tricks": [
      [
        {"seat": 0, "domino": [6, 3]},
        {"seat": 1, "domino": [6, 1]},
        {"seat": 2, "domino": [6, 4]},
        {"seat": 3, "domino": [6, 0]}
      ],
      [
        {"seat": 2, "domino": [0, 0]},
        {"seat": 3, "domino": [2, 2]},
        {"seat": 0, "domino": [5, 0]},
        {"seat": 1, "domino": [2, 0]}
      ],
      [
        {"seat": 2, "domino": [4, 3]},
        {"seat": 3, "domino": [4, 2]},
        {"seat": 0, "domino": [4, 0]},
        {"seat": 1, "domino": [5, 4]}
      ],
      [
        {"seat": 1, "domino": [1, 1]},
        {"seat": 2, "domino": [3, 0]},
        {"seat": 3, "domino": [3, 3]},
        {"seat": 0, "domino": [2, 1]}
      ],
      [
        {"seat": 1, "domino": [1, 0]},
        {"seat": 2, "domino": [6, 6]},
        {"seat": 3, "domino": [5, 2]},
        {"seat": 0, "domino": [5, 1]}
      ]
    ],
    "endpoint_hidden_pool": [
      [5, 5],
      [4, 4],
      [3, 2],
      [6, 5],
      [5, 3],
      [6, 2]
    ],
    "endpoint_capacities": [2, 2, 2],
    "derived_hidden_voids": {
      "1": [],
      "2": [1],
      "3": [0, 1]
    },
    "fiber_cardinality": 90,
    "typed_successor_crosschecks": 12
  }
}
```

