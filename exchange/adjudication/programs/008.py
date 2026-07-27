#!/usr/bin/env python3
"""Exact no-void support census for Straight Texas 42.

Claim recomputed by this receipt:
    NO_VOID_SLICE = 624892870

Standard library only. Deterministic. No network and no file I/O.
"""
from __future__ import annotations

from collections import Counter
from itertools import combinations, permutations, product
from math import comb
import gc
import sys
import time

# ---------------------------------------------------------------------------
# Straight 42 mechanics, from first principles
# ---------------------------------------------------------------------------

PIPS = tuple(range(7))
DOMINOES = tuple((h, l) for h in PIPS for l in range(h + 1))
INDEX = {d: i for i, d in enumerate(DOMINOES)}
ALL_MASK = (1 << 28) - 1

PIP_DECLARATIONS = tuple(range(7))
DT = "DT"
NT = "NT"
DECLARATIONS = PIP_DECLARATIONS + (DT, NT)
MODULE_DECLARATIONS = PIP_DECLARATIONS + (DT,)
CALLED = 7
VIEWER = 0
HIDDEN = (1, 2, 3)


def bits(mask: int):
    while mask:
        bit = mask & -mask
        yield bit.bit_length() - 1
        mask ^= bit


def tile_mask(indices) -> int:
    out = 0
    for i in indices:
        out |= 1 << i
    return out


def contains(d: tuple[int, int], pip: int) -> bool:
    return d[0] == pip or d[1] == pip


def is_double(d: tuple[int, int]) -> bool:
    return d[0] == d[1]


def called_mask(declaration) -> int:
    if declaration in PIP_DECLARATIONS:
        return sum(
            1 << i
            for i, d in enumerate(DOMINOES)
            if contains(d, declaration)
        )
    if declaration == DT:
        return sum(
            1 << i
            for i, d in enumerate(DOMINOES)
            if is_double(d)
        )
    if declaration == NT:
        return 0
    raise ValueError(declaration)


CALLED_MASK = {d: called_mask(d) for d in DECLARATIONS}


def effective_mask(declaration, context: int) -> int:
    if context == CALLED:
        return CALLED_MASK[declaration]
    natural = sum(
        1 << i
        for i, d in enumerate(DOMINOES)
        if contains(d, context)
    )
    return natural & ~CALLED_MASK[declaration]


EFFECTIVE = {
    (d, q): effective_mask(d, q)
    for d in DECLARATIONS
    for q in range(8)
}


def led_context(declaration, domino_index: int) -> int:
    if CALLED_MASK[declaration] & (1 << domino_index):
        return CALLED
    return DOMINOES[domino_index][0]


LED_CONTEXT = {
    (d, i): led_context(d, i)
    for d in DECLARATIONS
    for i in range(28)
}

LEAD_FIBER = {
    (d, q): sum(
        1 << i
        for i in range(28)
        if LED_CONTEXT[(d, i)] == q
    )
    for d in DECLARATIONS
    for q in range(8)
}


def rank_value(declaration, domino_index: int) -> int:
    h, l = DOMINOES[domino_index]
    if declaration == DT and h == l:
        return h
    if h == l and declaration != DT:
        return 100  # TOP, above every mixed rank.
    return h + l


def trick_key(declaration, context: int, domino_index: int) -> int:
    bit = 1 << domino_index
    if CALLED_MASK[declaration] & bit:
        tier = 2
    elif EFFECTIVE[(declaration, context)] & bit:
        tier = 1
    else:
        tier = 0
    return tier * 128 + (rank_value(declaration, domino_index) if tier else 0)


KEY = {
    (d, q, i): trick_key(d, q, i)
    for d in DECLARATIONS
    for q in range(8)
    for i in range(28)
}


def trick_winner(declaration, plays: tuple[tuple[int, int], ...]) -> int:
    context = LED_CONTEXT[(declaration, plays[0][1])]
    values = [KEY[(declaration, context, i)] for _, i in plays]
    maximum = max(values)
    if values.count(maximum) != 1:
        raise AssertionError((declaration, plays, values))
    return plays[values.index(maximum)][0]


def legal_mask(declaration, hand_mask: int, current_trick) -> int:
    if not current_trick:
        return hand_mask
    context = LED_CONTEXT[(declaration, current_trick[0][1])]
    followers = hand_mask & EFFECTIVE[(declaration, context)]
    return followers if followers else hand_mask


def check_rules() -> int:
    assert len(DOMINOES) == 28
    assert len(set(DOMINOES)) == 28
    assert sum(1 for d in DOMINOES if is_double(d)) == 7
    for p in PIPS:
        assert sum(1 for d in DOMINOES if contains(d, p)) == 7

    expected_contexts = {
        **{p: (set(PIPS) - {p}) | {CALLED} for p in PIPS},
        DT: {1, 2, 3, 4, 5, 6, CALLED},
        NT: set(PIPS),
    }
    for declaration in DECLARATIONS:
        contexts = {
            q for q in range(8) if LEAD_FIBER[(declaration, q)]
        }
        assert contexts == expected_contexts[declaration]
        sizes = sorted(
            LEAD_FIBER[(declaration, q)].bit_count()
            for q in contexts
        )
        assert sizes == list(range(1, 8))
        union = 0
        for q in contexts:
            assert not (union & LEAD_FIBER[(declaration, q)])
            union |= LEAD_FIBER[(declaration, q)]
        assert union == ALL_MASK

    cases = 0
    for declaration in DECLARATIONS:
        for lead in range(28):
            context = LED_CONTEXT[(declaration, lead)]
            rest = [i for i in range(28) if i != lead]
            for others in combinations(rest, 3):
                values = [
                    KEY[(declaration, context, i)]
                    for i in (lead,) + others
                ]
                assert values.count(max(values)) == 1
                cases += 1
    assert cases == 737_100
    return cases


# ---------------------------------------------------------------------------
# Admissible complete-trick modules and strong hidden triples
# ---------------------------------------------------------------------------


def make_groups(declaration):
    regular = {size: set() for size in range(1, 5)}
    strong = set()

    for context in range(8):
        lead = LEAD_FIBER[(declaration, context)]
        if not lead:
            continue
        follow = EFFECTIVE[(declaration, context)]
        members = tuple(bits(follow))
        global_top = max(
            range(28),
            key=lambda i: KEY[(declaration, context, i)],
        )

        for size in range(1, 5):
            for choice in combinations(members, size):
                mask = tile_mask(choice)
                if not (mask & lead):
                    continue
                regular[size].add(mask)
                if size == 3 and (mask & (1 << global_top)):
                    strong.add(mask)

    return (
        {size: tuple(sorted(values)) for size, values in regular.items()},
        tuple(sorted(strong)),
    )


GROUP = {}
STRONG = {}
for _declaration in DECLARATIONS:
    GROUP[_declaration], STRONG[_declaration] = make_groups(_declaration)


def module_assignment_exists(declaration, group: int, desired_winner: int) -> bool:
    tiles = tuple(bits(group))
    for leader in range(4):
        order = tuple((leader + offset) % 4 for offset in range(4))
        for assignment in permutations(tiles):
            plays = tuple(zip(order, assignment))
            context = LED_CONTEXT[(declaration, plays[0][1])]
            if not all(
                EFFECTIVE[(declaration, context)] & (1 << i)
                for _, i in plays
            ):
                continue
            if trick_winner(declaration, plays) == desired_winner:
                return True
    return False


def check_group_lemmas() -> dict[str, int]:
    for declaration in MODULE_DECLARATIONS:
        assert [len(GROUP[declaration][s]) for s in range(1, 5)] == [
            28,
            91,
            140,
            119,
        ]
        assert len(STRONG[declaration]) == 15

    assert [len(GROUP[NT][s]) for s in range(1, 5)] == [
        28,
        112,
        210,
        224,
    ]
    assert len(STRONG[NT]) == 105

    module_checks = 0
    for declaration in MODULE_DECLARATIONS:
        for group in GROUP[declaration][4]:
            for desired in range(4):
                assert module_assignment_exists(declaration, group, desired)
                module_checks += 1
    assert module_checks == 8 * 119 * 4

    strong_checks = 0
    for declaration in DECLARATIONS:
        for group in STRONG[declaration]:
            contexts = []
            for q in range(8):
                if not LEAD_FIBER[(declaration, q)]:
                    continue
                if group & ~EFFECTIVE[(declaration, q)]:
                    continue
                top = max(
                    range(28),
                    key=lambda i: KEY[(declaration, q, i)],
                )
                if group & (1 << top) and group & LEAD_FIBER[(declaration, q)]:
                    contexts.append((q, top))
            assert contexts
            for q, top in contexts:
                assert LEAD_FIBER[(declaration, q)] & (1 << top)
                assert all(
                    KEY[(declaration, q, top)] >= KEY[(declaration, q, i)]
                    for i in range(28)
                )
            strong_checks += 1

    return {
        "module_assignments": module_checks,
        "strong_groups": strong_checks,
    }


# Disjoint unions of r ordinary four-tile modules for one declaration.
FOUR_UNIONS = {}


def build_four_unions() -> dict[object, tuple[frozenset[int], ...]]:
    result = {}
    expected_module_levels = (1, 119, 4610, 65430, 246379, 72771, 210)

    for declaration in DECLARATIONS:
        maximum = 6 if declaration in MODULE_DECLARATIONS else 2
        levels = [frozenset({0})]
        current = {0}
        groups = GROUP[declaration][4]
        for _ in range(maximum):
            nxt = {
                used | group
                for used in current
                for group in groups
                if not (used & group)
            }
            levels.append(frozenset(nxt))
            current = nxt
        result[declaration] = tuple(levels)

        if declaration in MODULE_DECLARATIONS:
            assert tuple(len(level) for level in levels) == expected_module_levels

    return result


# ---------------------------------------------------------------------------
# Exact 28-bit upward-closure counter, split into 14+14 bits
# ---------------------------------------------------------------------------

HALF = 14
N_HALF = 1 << HALF
LOW_MASK = N_HALF - 1


def make_target_masks():
    masks = []
    for index in range(HALF):
        block = 1 << index
        chunk = (1 << block) - 1
        mask = 0
        for start in range(0, N_HALF, 2 * block):
            mask |= chunk << (start + block)
        masks.append(mask)
    return tuple(masks)


TARGET_MASKS = make_target_masks()
POPCOUNT_MASKS = [0] * (HALF + 1)
for _high in range(N_HALF):
    POPCOUNT_MASKS[_high.bit_count()] |= 1 << _high


def spec_unions(spec):
    """Yield witness unions for (regular4 count, strong3 count, end size, declarations)."""
    regular_count, strong_count, end_size, declarations = spec
    for declaration in declarations:
        current = set(FOUR_UNIONS[declaration][regular_count])
        for _ in range(strong_count):
            current = {
                used | group
                for used in current
                for group in STRONG[declaration]
                if not (used & group)
            }
        for used in current:
            for group in GROUP[declaration][end_size]:
                if not (used & group):
                    yield used | group


def upward_rows(specs, count_witnesses: bool = False):
    rows = [0] * N_HALF
    witness_set = set() if count_witnesses else None

    for spec in specs:
        for witness in spec_unions(spec):
            if witness_set is not None:
                witness_set.add(witness)
            rows[witness & LOW_MASK] |= 1 << (witness >> HALF)

    # Low-half subset zeta transform.
    for index in range(HALF):
        bit = 1 << index
        for low in range(N_HALF):
            if low & bit:
                rows[low] |= rows[low ^ bit]

    # High-half superset zeta transform, bit-parallel.
    for low, value in enumerate(rows):
        for index in range(HALF):
            value |= (value << (1 << index)) & TARGET_MASKS[index]
        rows[low] = value

    return rows, (len(witness_set) if witness_set is not None else None)


def exact_size_coverage(specs, target_size: int, extract_missing: bool = False,
                        count_witnesses: bool = False):
    rows, witness_count = upward_rows(specs, count_witnesses)
    total = 0
    missing = []

    for low, row in enumerate(rows):
        high_count = target_size - low.bit_count()
        if not 0 <= high_count <= HALF:
            continue
        allowed = POPCOUNT_MASKS[high_count]
        total += (row & allowed).bit_count()
        if extract_missing:
            absent = allowed & ~row
            while absent:
                bit = absent & -absent
                absent ^= bit
                high = bit.bit_length() - 1
                missing.append(low | (high << HALF))

    del rows
    gc.collect()
    return total, missing, witness_count


def R(regular_count: int, end_size: int):
    return (regular_count, 0, end_size, MODULE_DECLARATIONS)


def S(regular_count: int, strong_count: int, end_size: int):
    return (regular_count, strong_count, end_size, DECLARATIONS)


# ---------------------------------------------------------------------------
# Exact target recognizer / realization algorithm
# ---------------------------------------------------------------------------

PARTIAL_SCHEDULES = {}
for _low_mask in range(8):
    schedules = []
    for _leader in range(4):
        for _length in range(4):
            observed = 0
            for _offset in range(_length):
                seat = (_leader + _offset) % 4
                if seat:
                    observed |= 1 << (seat - 1)
            if observed == _low_mask:
                schedules.append((_leader, _length))
    PARTIAL_SCHEDULES[_low_mask] = tuple(schedules)


def shape_parameters(capacities: tuple[int, int, int]):
    high = max(capacities)
    completed = 7 - high
    if capacities[0] == capacities[1] == capacities[2]:
        low_mask = 0
    else:
        low_mask = sum(
            1 << seat
            for seat, value in enumerate(capacities)
            if value == high - 1
        )
    return completed, low_mask


def find_void_free_trace(T: int, capacities: tuple[int, int, int],
                         need_certificate: bool = False):
    """Exhaustively find a declaration/leader/trace realizing (U,k) void-free.

    T is the complement of U. Hidden followers are restricted to following tiles. Viewer
    legality is checked retrospectively after T determines the viewer hand.
    """
    completed, low_mask = shape_parameters(capacities)
    target_partials = PARTIAL_SCHEDULES[low_mask]

    for declaration in DECLARATIONS:
        viewer_plays = []  # (tile, context, was_lead), chronological
        trace = []

        def viewer_legal_at_end(remaining: int) -> bool:
            hand = remaining
            for tile, _, _ in viewer_plays:
                hand |= 1 << tile
            for tile, context, was_lead in viewer_plays:
                if not was_lead:
                    followers = hand & EFFECTIVE[(declaration, context)]
                    if followers and not (followers & (1 << tile)):
                        return False
                hand ^= 1 << tile
            return True

        def play_partial(remaining: int, leader: int, length: int,
                         position: int, trick: list[tuple[int, int]]) -> bool:
            if position == length:
                return viewer_legal_at_end(remaining)

            actor = (leader + position) % 4
            candidates = remaining
            context = -1
            if position:
                context = LED_CONTEXT[(declaration, trick[0][1])]
                if actor != VIEWER:
                    candidates &= EFFECTIVE[(declaration, context)]

            while candidates:
                bit = candidates & -candidates
                candidates ^= bit
                tile = bit.bit_length() - 1
                trick.append((actor, tile))
                trace.append((actor, tile))
                if actor == VIEWER:
                    viewer_plays.append((tile, context, position == 0))
                if play_partial(
                    remaining ^ bit, leader, length, position + 1, trick
                ):
                    return True
                if actor == VIEWER:
                    viewer_plays.pop()
                trace.pop()
                trick.pop()
            return False

        def play_rounds(remaining: int, leader: int, rounds_left: int) -> bool:
            if rounds_left == 0:
                for partial_leader, length in target_partials:
                    if length and partial_leader != leader:
                        continue
                    if play_partial(
                        remaining, partial_leader, length, 0, []
                    ):
                        return True
                return False

            trick = []

            def play_position(position: int, rem: int) -> bool:
                actor = (leader + position) % 4
                candidates = rem
                context = -1
                if position:
                    context = LED_CONTEXT[(declaration, trick[0][1])]
                    if actor != VIEWER:
                        candidates &= EFFECTIVE[(declaration, context)]

                while candidates:
                    bit = candidates & -candidates
                    candidates ^= bit
                    tile = bit.bit_length() - 1
                    trick.append((actor, tile))
                    trace.append((actor, tile))
                    if actor == VIEWER:
                        viewer_plays.append((tile, context, position == 0))

                    if position == 3:
                        winner = trick_winner(declaration, tuple(trick))
                        success = play_rounds(
                            rem ^ bit, winner, rounds_left - 1
                        )
                    else:
                        success = play_position(position + 1, rem ^ bit)

                    if success:
                        return True
                    if actor == VIEWER:
                        viewer_plays.pop()
                    trace.pop()
                    trick.pop()
                return False

            return play_position(0, remaining)

        for initial_leader in range(4):
            if play_rounds(T, initial_leader, completed):
                certificate = {
                    "declaration": declaration,
                    "initial_leader": initial_leader,
                    "trace": tuple(trace),
                }
                return certificate if need_certificate else True

    return None if need_certificate else False


def replay_certificate(T: int, capacities: tuple[int, int, int], certificate) -> dict:
    declaration = certificate["declaration"]
    initial_leader = certificate["initial_leader"]
    trace = certificate["trace"]
    U = ALL_MASK ^ T

    played_by = {seat: [] for seat in range(4)}
    for seat, tile in trace:
        played_by[seat].append(tile)

    hidden_public = 0
    for seat in HIDDEN:
        hidden_public |= tile_mask(played_by[seat])
    viewer_initial = T & ~hidden_public
    assert viewer_initial.bit_count() == 7

    remaining_u = list(bits(U))
    final_hidden = {}
    cursor = 0
    for seat, capacity in zip(HIDDEN, capacities):
        hand = remaining_u[cursor:cursor + capacity]
        cursor += capacity
        final_hidden[seat] = tile_mask(hand)
    assert cursor == len(remaining_u)

    hands = {VIEWER: viewer_initial}
    for seat in HIDDEN:
        hands[seat] = final_hidden[seat] | tile_mask(played_by[seat])

    assert all(hands[seat].bit_count() == 7 for seat in range(4))
    union = 0
    for seat in range(4):
        assert not (union & hands[seat])
        union |= hands[seat]
    assert union == ALL_MASK

    leader = initial_leader
    current = []
    hidden_voids = {seat: set() for seat in HIDDEN}
    public = 0

    for actor, tile in trace:
        assert actor == (leader + len(current)) % 4
        bit = 1 << tile
        assert hands[actor] & bit
        legal = legal_mask(declaration, hands[actor], current)
        assert legal & bit

        if current and actor in HIDDEN:
            context = LED_CONTEXT[(declaration, current[0][1])]
            if not (EFFECTIVE[(declaration, context)] & bit):
                hidden_voids[actor].add(context)

        hands[actor] ^= bit
        public |= bit
        current.append((actor, tile))
        if len(current) == 4:
            leader = trick_winner(declaration, tuple(current))
            current = []

    assert hidden_voids == {1: set(), 2: set(), 3: set()}
    derived_capacities = tuple(hands[s].bit_count() for s in HIDDEN)
    assert derived_capacities == capacities
    derived_u = hands[1] | hands[2] | hands[3]
    assert derived_u == U
    assert (hands[VIEWER] | public | U) == ALL_MASK

    return {
        "declaration": declaration,
        "initial_leader": initial_leader,
        "viewer_initial": viewer_initial,
        "trace_length": len(trace),
        "pool": U,
        "capacities": capacities,
    }


# ---------------------------------------------------------------------------
# Full no-void coverage of all 50 ordered capacity profiles
# ---------------------------------------------------------------------------

GENERAL_COVERAGE_JOBS = (
    # equal profiles: j complete tricks; final hidden triple completes trick j
    ("eq1", (R(0, 3),), 10, 13_123_110, 0),
    ("eq2", (R(1, 3), S(0, 1, 3)), 13, 37_442_160, 0),
    ("eq3", (R(2, 3), S(1, 1, 3)), 16, 30_421_755, 0),
    ("eq4", (R(3, 3),), 19, 6_906_900, 0),
    ("eq5", (R(4, 3),), 22, 376_740, 0),
    ("eq6", (R(5, 3),), 25, 3_276, 0),
    ("eq7", (R(6, 3),), 28, 1, 0),

    # one low hidden seat
    ("b1j0", (R(0, 1),), 8, 3_108_105, 0),
    ("b1j2", (R(2, 1), S(1, 1, 1), S(0, 2, 1)),
     14, 40_116_575, 25),
    ("b1j3", (R(3, 1), S(2, 1, 1), S(1, 2, 1), S(0, 3, 1)),
     17, 21_474_180, 0),
    ("b1j4", (R(4, 1),), 20, 3_106_530, 1_575),
    ("b1j5", (R(5, 1),), 23, 98_175, 105),
    ("b1j6", (R(6, 1),), 26, 273, 105),

    # two low hidden seats
    ("b2j0", (R(0, 2),), 9, 6_906_900, 0),
    ("b2j1", (R(1, 2), S(0, 1, 2)), 12, 30_421_755, 0),
    ("b2j2", (R(2, 2), S(1, 1, 2), S(0, 2, 2)),
     15, 37_442_160, 0),
    ("b2j3", (R(3, 2), S(2, 1, 2)), 18, 13_123_110, 0),
    ("b2j4", (R(4, 2),), 21, 1_184_040, 0),
    ("b2j5", (R(5, 2),), 24, 20_475, 0),
    ("b2j6", (R(6, 2),), 27, 28, 0),
)


def singleton_profiles(completed: int):
    high = 7 - completed
    low = high - 1
    return tuple(
        tuple(low if seat == low_seat else high for seat in range(3))
        for low_seat in range(3)
    )


def check_star_pigeonhole() -> dict[str, int]:
    # Seven pip stars plus the doubles star form the K8 edge-star model.
    stars = [CALLED_MASK[p] for p in PIPS] + [CALLED_MASK[DT]]
    assert all(star.bit_count() == 7 for star in stars)
    for tile in range(28):
        assert sum(bool(star & (1 << tile)) for star in stars) == 2
    # For every 11-set T the incidence total is 22. If all eight degrees
    # were at most two, the total would be at most 16, contradiction.
    assert 2 * 11 > 8 * 2
    return {"stars": 8, "incidences_in_11_set": 22}


def check_general_coverage():
    missing_by_job = {}
    coverage_table = {}

    for name, specs, target_size, expected_count, expected_missing in GENERAL_COVERAGE_JOBS:
        count, missing, _ = exact_size_coverage(
            specs,
            target_size,
            extract_missing=bool(expected_missing),
        )
        assert count == expected_count, (name, count, expected_count)
        assert comb(28, target_size) - count == expected_missing
        assert len(missing) == expected_missing
        missing_by_job[name] = tuple(missing)
        coverage_table[name] = {
            "T_size": target_size,
            "covered": count,
            "missing": expected_missing,
        }
        print(f"PASS coverage_{name} {count} missing={expected_missing}")

    # The only template exceptions are singleton-low profiles at j=2,4,5,6.
    exception_shapes = {
        "b1j2": singleton_profiles(2),
        "b1j4": singleton_profiles(4),
        "b1j5": singleton_profiles(5),
        "b1j6": singleton_profiles(6),
    }

    realized_exceptions = 0
    for name, profiles in exception_shapes.items():
        for T in missing_by_job[name]:
            for capacities in profiles:
                certificate = find_void_free_trace(T, capacities, True)
                assert certificate is not None, (name, T, capacities)
                replay_certificate(T, capacities, certificate)
                realized_exceptions += 1

    assert realized_exceptions == 3 * (25 + 1575 + 105 + 105)
    print(f"PASS coverage_exceptions_replayed {realized_exceptions}")
    del missing_by_job
    gc.collect()
    return coverage_table, realized_exceptions


# ---------------------------------------------------------------------------
# Recompute the adjudicated 001 shallow no-void family as a special case
# ---------------------------------------------------------------------------

def regular_witness_masks(pattern: tuple[int, ...]):
    witnesses = set()
    for declaration in MODULE_DECLARATIONS:
        current = {0}
        for size in pattern:
            groups = GROUP[declaration][size]
            current = {
                used | group
                for used in current
                for group in groups
                if not (used & group)
            }
        witnesses.update(current)
    return witnesses


def coverage_from_witnesses(witnesses, target_size: int) -> int:
    rows = [0] * N_HALF
    for witness in witnesses:
        rows[witness & LOW_MASK] |= 1 << (witness >> HALF)

    for index in range(HALF):
        bit = 1 << index
        for low in range(N_HALF):
            if low & bit:
                rows[low] |= rows[low ^ bit]
    for low, value in enumerate(rows):
        for index in range(HALF):
            value |= (value << (1 << index)) & TARGET_MASKS[index]
        rows[low] = value

    total = 0
    for low, row in enumerate(rows):
        high_count = target_size - low.bit_count()
        if 0 <= high_count <= HALF:
            total += (row & POPCOUNT_MASKS[high_count]).bit_count()
    del rows
    gc.collect()
    return total


def check_001_no_void_anchor() -> tuple[int, dict[str, object]]:
    expected = {
        (4, 2): (25_584, {12: 30_402_400}),
        (4, 3): (36_128, {12: 30_294_577}),
        (4, 4): (19_394, {13: 34_115_923, 14: 39_546_166}),
        (4, 4, 2): (944_482, {15: 37_400_509}),
        (4, 4, 3): (985_332, {15: 37_241_110, 16: 30_419_732}),
        (4, 4, 4): (381_140, {17: 21_408_593}),
    }
    actual = {}
    for pattern, (expected_witnesses, expected_counts) in expected.items():
        witnesses = regular_witness_masks(pattern)
        assert len(witnesses) == expected_witnesses
        counts = {
            target: coverage_from_witnesses(witnesses, target)
            for target in expected_counts
        }
        assert counts == expected_counts
        actual[pattern] = counts
        del witnesses
        gc.collect()

    early = (
        comb(28, 7)
        + 3 * comb(28, 8)
        + 3 * comb(28, 9)
        + comb(28, 10)
        + 3 * comb(28, 11)
    )
    assert early == 108_774_705

    total = early
    total += 2 * actual[(4, 2)][12] + actual[(4, 3)][12]
    total += actual[(4, 4)][13] + 3 * actual[(4, 4)][14]
    total += 2 * actual[(4, 4, 2)][15] + actual[(4, 4, 3)][15]
    total += actual[(4, 4, 3)][16] + 3 * actual[(4, 4, 4)][17]
    assert total == 559_316_142
    return total, {str(k): v for k, v in actual.items()}


# ---------------------------------------------------------------------------
# Fixed-viewer-hand exhaustive j<=2 cross-check, two independent state forms
# ---------------------------------------------------------------------------

FIXED_VIEWER_HAND = sum(
    1 << i for i, d in enumerate(DOMINOES) if is_double(d)
)
FIXED_INITIAL_POOL = ALL_MASK ^ FIXED_VIEWER_HAND


def exhaustive_full_trick_prefixes(declaration):
    # State retains the full ordered current trick.
    states = {
        (FIXED_VIEWER_HAND, FIXED_INITIAL_POOL, (7, 7, 7), leader, tuple(), 0)
        for leader in range(4)
    }
    outputs = set()
    state_count = 0

    while states:
        successors = set()
        for viewer_hand, U, capacities, leader, current, completed in states:
            outputs.add((U, capacities))
            state_count += 1
            if completed == 2 and len(current) == 3:
                continue

            actor = (leader + len(current)) % 4
            if actor == VIEWER:
                candidates = legal_mask(declaration, viewer_hand, current)
            else:
                candidates = U
                if current:
                    context = LED_CONTEXT[(declaration, current[0][1])]
                    candidates &= EFFECTIVE[(declaration, context)]

            for tile in bits(candidates):
                bit = 1 << tile
                if actor == VIEWER:
                    next_viewer = viewer_hand ^ bit
                    next_u = U
                    next_capacities = capacities
                else:
                    next_viewer = viewer_hand
                    next_u = U ^ bit
                    values = list(capacities)
                    values[actor - 1] -= 1
                    next_capacities = tuple(values)

                next_current = current + ((actor, tile),)
                if len(next_current) == 4:
                    next_leader = trick_winner(declaration, next_current)
                    next_current = tuple()
                    next_completed = completed + 1
                else:
                    next_leader = leader
                    next_completed = completed

                successors.add((
                    next_viewer,
                    next_u,
                    next_capacities,
                    next_leader,
                    next_current,
                    next_completed,
                ))
        states = successors

    return outputs, state_count


def exhaustive_folded_prefix_dp(declaration):
    # State folds the current trick to (length, context, best key, winner).
    states = {
        (FIXED_VIEWER_HAND, FIXED_INITIAL_POOL, (7, 7, 7),
         leader, 0, -1, -1, -1, 0)
        for leader in range(4)
    }
    outputs = set()
    state_count = 0

    while states:
        successors = set()
        for (viewer_hand, U, capacities, leader,
             length, context, best_key, winner, completed) in states:
            outputs.add((U, capacities))
            state_count += 1
            if completed == 2 and length == 3:
                continue

            actor = (leader + length) % 4
            if actor == VIEWER:
                candidates = viewer_hand
                if length:
                    followers = viewer_hand & EFFECTIVE[(declaration, context)]
                    if followers:
                        candidates = followers
            else:
                candidates = U
                if length:
                    candidates &= EFFECTIVE[(declaration, context)]

            for tile in bits(candidates):
                bit = 1 << tile
                if actor == VIEWER:
                    next_viewer = viewer_hand ^ bit
                    next_u = U
                    next_capacities = capacities
                else:
                    next_viewer = viewer_hand
                    next_u = U ^ bit
                    values = list(capacities)
                    values[actor - 1] -= 1
                    next_capacities = tuple(values)

                if length == 0:
                    next_context = LED_CONTEXT[(declaration, tile)]
                    next_best = KEY[(declaration, next_context, tile)]
                    next_winner = actor
                    next_length = 1
                else:
                    value = KEY[(declaration, context, tile)]
                    next_context = context
                    if value > best_key:
                        next_best = value
                        next_winner = actor
                    else:
                        next_best = best_key
                        next_winner = winner
                    next_length = length + 1

                next_leader = leader
                next_completed = completed
                if next_length == 4:
                    next_leader = next_winner
                    next_length = 0
                    next_context = -1
                    next_best = -1
                    next_winner = -1
                    next_completed += 1

                successors.add((
                    next_viewer,
                    next_u,
                    next_capacities,
                    next_leader,
                    next_length,
                    next_context,
                    next_best,
                    next_winner,
                    next_completed,
                ))
        states = successors

    return outputs, state_count


def check_fixed_hand_crosscheck():
    expected = {0: 81_974, DT: 379, NT: 208_874}
    details = {}
    for declaration in (0, DT, NT):
        brute, brute_states = exhaustive_full_trick_prefixes(declaration)
        folded, folded_states = exhaustive_folded_prefix_dp(declaration)
        assert brute == folded
        assert len(brute) == expected[declaration]
        details[str(declaration)] = {
            "brute_outputs": len(brute),
            "folded_outputs": len(folded),
            "brute_states": brute_states,
            "folded_states": folded_states,
        }
        print(
            f"PASS crosscheck_{declaration} "
            f"brute={len(brute)} folded={len(folded)}"
        )
        del brute, folded
        gc.collect()
    return details


# ---------------------------------------------------------------------------
# Counting and deterministic realization spot-checks
# ---------------------------------------------------------------------------


def capacity_profiles():
    return tuple(
        capacities
        for capacities in product(range(8), repeat=3)
        if max(capacities) - min(capacities) <= 1
    )


def unrank_combination_mask(n: int, size: int, rank: int) -> int:
    assert 0 <= rank < comb(n, size)
    mask = 0
    candidate = 0
    remaining = size
    while remaining:
        block = comb(n - candidate - 1, remaining - 1)
        if rank < block:
            mask |= 1 << candidate
            candidate += 1
            remaining -= 1
        else:
            rank -= block
            candidate += 1
    return mask


def count_no_void_slice():
    profiles = capacity_profiles()
    assert len(profiles) == 50
    by_profile = {
        capacities: comb(28, sum(capacities))
        for capacities in profiles
    }
    total = sum(by_profile.values())
    assert total == 624_892_870
    return total, by_profile


def check_realization_stride():
    profiles = capacity_profiles()
    checked = 0
    profile_hits = set()
    stride_table = {}

    # For each of all 50 ordered profiles, sample 21 equally strided ranks
    # in lexicographic combination order (deduplicated for tiny blocks).
    for capacities in profiles:
        T_size = 28 - sum(capacities)
        block = comb(28, T_size)
        stride = max(1, block // 21)
        ranks = sorted({min(i * stride, block - 1) for i in range(21)})
        stride_table[capacities] = (stride, len(ranks))
        for rank in ranks:
            T = unrank_combination_mask(28, T_size, rank)
            certificate = find_void_free_trace(T, capacities, True)
            assert certificate is not None, (capacities, rank, T)
            replay_certificate(T, capacities, certificate)
            checked += 1
            profile_hits.add(capacities)

    assert profile_hits == set(profiles)
    assert checked == 1_030
    print(f"PASS realization_replays {checked} profiles={len(profile_hits)}")
    return checked, len(profile_hits)


def corpus_floor_anchor() -> int:
    value = (
        comb(28, 21)
        + 3 * comb(28, 20)
        + 3 * comb(28, 19)
        + comb(28, 18)
    )
    assert value == 44_352_165
    return value


# ---------------------------------------------------------------------------
# Driver
# ---------------------------------------------------------------------------


def run_check(name, function):
    try:
        value = function()
    except Exception as exc:
        print(f"FAIL {name} {type(exc).__name__}: {exc}")
        return False, None
    if isinstance(value, (int, str)):
        print(f"PASS {name} {value}")
    else:
        print(f"PASS {name}")
    return True, value


def main() -> int:
    global FOUR_UNIONS
    started = time.time()
    ok = True
    values = {}

    checks = (
        ("rules", check_rules),
        ("group_lemmas", check_group_lemmas),
        ("star_pigeonhole", check_star_pigeonhole),
    )
    for name, function in checks:
        passed, value = run_check(name, function)
        ok &= passed
        values[name] = value
    if not ok:
        return 1

    try:
        FOUR_UNIONS = build_four_unions()
        print("PASS four_union_levels")
    except Exception as exc:
        print(f"FAIL four_union_levels {type(exc).__name__}: {exc}")
        return 1

    remaining_checks = (
        ("general_coverage", check_general_coverage),
        ("anchor_001_no_void", check_001_no_void_anchor),
        ("corpus_floor", corpus_floor_anchor),
        ("fixed_hand_j_le_2_crosscheck", check_fixed_hand_crosscheck),
        ("headline_count", count_no_void_slice),
        ("realization_stride", check_realization_stride),
    )
    for name, function in remaining_checks:
        passed, value = run_check(name, function)
        ok &= passed
        values[name] = value
    if not ok:
        return 1

    try:
        headline, by_profile = values["headline_count"]
        anchor_001, _ = values["anchor_001_no_void"]
        floor = values["corpus_floor"]
        stride_checked, profile_count = values["realization_stride"]
        _, exceptions = values["general_coverage"]

        assert headline == 624_892_870
        assert anchor_001 == 559_316_142
        assert floor == 44_352_165
        assert stride_checked >= 1_000
        assert profile_count == 50
        assert exceptions == 5_430
        assert sum(by_profile.values()) == headline
    except Exception as exc:
        print(f"FAIL headline_consistency {type(exc).__name__}: {exc}")
        return 1

    print("PASS headline_consistency")
    print("PASS FINAL NO_VOID_SLICE 624892870")
    print(f"PASS runtime_seconds {time.time() - started:.3f}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
