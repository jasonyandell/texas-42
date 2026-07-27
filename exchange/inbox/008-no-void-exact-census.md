---
number: 8
slug: no-void-exact-census
conversation: https://chatgpt.com/c/6a66e786-2ac0-83ea-ade0-dff707fae5e6
submitted-at: 2026-07-27T18:15:40.691Z
harvested-at: 2026-07-27T20:01:49.039Z
extraction: copy-button
caveats: none
---
The no-void slice is **saturated**: for every one of the 50 ordered capacity profiles with (\max k_s-\min k_s\le 1), **every** pool (U\subseteq\mathcal D) of size (k_1+k_2+k_3) has a legal Straight realization with no hidden-seat void.

Therefore

[
\begin{aligned}
\mathrm{NO_VOID_SLICE}
&=
\sum_{\substack{k\in{0,\ldots,7}^3\
\max k-\min k\le1}}
\binom{28}{k_1+k_2+k_3}\
&=
\sum_{h=0}^{7}\binom{28}{3h}
+
3\sum_{h=0}^{6}
\left(
\binom{28}{3h+1}
+
\binom{28}{3h+2}
\right)\
&=\boxed{624,892,870}.
\end{aligned}
]

The decisive constructive fact is that an unrestricted support remains unrestricted after a hidden lead or successful follow: forcing the played tile into its actor’s hand and removing it is bijective onto the unrestricted successor fiber. The remaining problem is therefore an exact public-tile scheduling problem. The construction below closes that problem for all 50 profiles using complete-trick modules, strong hidden triples, a (K_8)-star argument for the sole exceptional shallow phase, and exhaustive exact replay for the finite complements left by the uniform templates. The underlying lead, follow, and winner rules are those of the attached profile; the foundation supplies the 50-profile capacity theorem, complete support invariant, and symbolic trace equivalence.    

## MACHINE-CHECKABLE ARTIFACTS

FINAL ANSWER: NO_VOID_SLICE = 624892870

A completed reference run reported **116.295 seconds**. A conservative allowance is under 30 minutes on one CPU core, with peak memory below 1 GB. The program is deterministic, uses only the Python standard library, performs no network or file I/O, and exits zero exactly when every check passes.

SHA-256: `38fd84ea59519d985c244b83d4fa9cf6802df608c562e0dca746dcd4e1e89eec`

[Download the identical Python receipt](sandbox:/mnt/data/no_void_slice_receipt.py)

```python
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
```

### Proof

1. **No-void supports are exactly unrestricted capacity fibers.**
   [USES: definitions of (U,V_s,P_s,k_s,\Phi); CELL-05; CELL-14.]
   If (V_1=V_2=V_3=\varnothing), then
   [
   P_1=P_2=P_3=U.
   ]
   The fiber is
   [
   \Phi(U,k)=
   \left{
   (H_1,H_2,H_3):
   |H_s|=k_s,\
   H_1\sqcup H_2\sqcup H_3=U
   \right}.
   ]
   It is nonempty exactly when (|U|=k_1+k_2+k_3).

   The pair ((U,k)) is recoverable from this nonempty fiber: every world has hand-size vector (k), and the union of its three hands is (U). Hence two unrestricted fibers are equal iff their pairs ((U,k)) are equal. By CELL-14, their normal forms are equal iff the pairs are equal. This remains true at boundary profiles with zero capacities, where the normal form may be binary or determinate rather than ternary. The declaration is not part of this object. 

2. **Every reachable no-void pair has one of 50 capacity profiles.**
   [USES: exact hidden-capacity reachability theorem, Math §7.13.1.]
   After (j) completed tricks and a partial next trick, each hidden seat has played either (j) or (j+1) tiles. Thus each capacity is (7-j) or (6-j), and
   [
   \max_s k_s-\min_s k_s\le1.
   ]
   Conversely, the attachment proves that exactly the 50 ordered triples satisfying this inequality occur as public-turn profiles. 

3. **Complement coordinates.**
   [USES: step 2; conservation of the 28 physical tiles.]
   For a target pair ((U,k)), write
   [
   T=\mathcal D\setminus U.
   ]
   Let (h=\max k_s), (j=7-h), and let
   [
   B={s:k_s=h-1}
   ]
   be the hidden seats that have already acted in the current trick. Then hidden seats have made (3j+|B|) public plays. The other seven members of (T) are the viewer’s initial hand, including any of its already public tiles. Therefore
   [
   |T|=7+3j+|B|=28-|U|.
   ]

4. **Successful hidden plays preserve unrestricted support.**
   [USES: definition of (\Phi(U,k)); typed transition theorem, TRANS-08/09.]
   Suppose the current support is unrestricted and hidden seat (s) leads or successfully follows with tile (d). Conditioning on (d\in H_s) and removing (d) gives
   [
   \vartheta_{s,d}
   \bigl(
   {\omega\in\Phi(U,k):d\in H_s(\omega)}
   \bigr)
   ======

   \Phi(U\setminus{d},k-e_s).
   ]
   Surjectivity follows by adding (d) back to seat (s); injectivity follows by removing it. On a successful follow, (d) itself supplies a following tile, so no further complete-void predicate is imposed. Viewer actions act identically on the hidden fiber.

   Consequently, any legal public trace in which every hidden follower plays a following tile ends with unrestricted support on its final pool and capacities. A hidden off-suit follower play would record a void and is precisely what the no-void construction excludes. This is the unrestricted specialization of the attachment’s typed support update. 

5. **Admissible groups.**
   [USES: effective-suit, led-context, and trick-key definitions; unique-winner theorem.]
   For declaration (\delta) and leadable context (q), call
   [
   G\subseteq\widehat\sigma_q^\delta
   ]
   an admissible (r)-group when (|G|=r) and
   [
   G\cap L_{\delta,q}\ne\varnothing.
   ]

   An admissible four-group is a complete all-follow trick. Let (x) be its unique maximum-key tile. For any desired winner (w), assign (x) to (w). If (x) is leadable, let (w) lead it. Otherwise assign a leadable member to another seat as leader. Every play follows and (w) wins. Thus a four-group can pass the lead to any desired seat.

   An admissible three-group can be assigned to the three hidden seats, with a legal viewer filler completing the trick. A **strong** three-group additionally contains a globally maximum-key tile for its context that is itself leadable. Giving that tile to a chosen hidden leader makes that seat win regardless of the viewer filler. The program derives every such group from the rules, verifies the group counts, exhausts 3,808 ordinary module/desired-winner assignments, and checks every strong group’s leadable global maximum. The winner rules used are exactly those in the package.  

6. **Stitching lemma.**
   [USES: steps 3–5; auction hand-independence.]
   Consider disjoint selected groups consisting of:

   * (r) ordinary four-groups;
   * (s) strong hidden three-groups;
   * one final admissible group of size (e\in{1,2,3}).

   Select the hidden leader required by the final partial-trick profile. Place the final group last. Immediately before it, place the (s) strong triples, all led and won by that same hidden seat. Prepend the (r) ordinary modules in reverse order, choosing each module’s winner to be the leader required by the following trick. The first resulting leader wins a legal (P(30)) auction—make its clockwise predecessor the shaker and have every later actor pass—and chooses the declaration used by the groups.

   Every hidden follower tile in this construction belongs to the led effective suit. Viewer module tiles also follow. For strong or terminal hidden triples, choose the viewer’s filler from its then-current legal set; such a set is always nonempty. A strong leader’s global maximum cannot be displaced by that filler.

   For a final group of size one, the designated low hidden seat leads one tile. For a size-two group, adjacent low hidden seats play lead/follow. For the separated pair ({h_1,h_3}), let (h_3) lead, choose a legal viewer play, and let (h_1) follow. For a size-three group, all hidden seats play and the viewer supplies the fourth tile.

   The group union contains all hidden public tiles and the viewer tiles used in ordinary four-groups. Exactly seven tiles of (T) are ultimately assigned to the viewer; all remaining public tiles are added to their hidden actors. Partition (U) arbitrarily into final hidden hands of sizes (k_s). Because hidden followers already play following tiles, adding arbitrary unplayed (U)-tiles cannot invalidate any hidden play. The result is a complete legal deal and a void-free prefix realizing ((U,k)).

7. **Finite construction grammar.**
   [USES: step 6.]
   Write `4` for an ordinary complete module, `S` for a strong hidden triple completing a trick with a viewer filler, and `1`, `2`, or `3` for the final admissible hidden group. The explicit JSON coverage table below lists the grammar used for every ((j,|B|)) phase.

   Equal profiles use a final `3`; singleton-low profiles use a final `1`; double-low profiles use a final `2`. The initial profile ((7,7,7)) needs no play. The singleton-low (j=1) profile is handled separately by step 10.

8. **Exact meet-in-the-middle coverage computation.**
   [USES: step 7; program functions `spec_unions`, `upward_rows`, and `exact_size_coverage`.]
   Every generated witness is represented by a 28-bit union mask (W). Split it into 14-bit halves (W=(W_L,W_H)). Initially, row (W_L) has bit (W_H) set.

   The low-half subset-zeta transform makes row (L) contain all witness high halves whose low halves satisfy (W_L\subseteq L). The high-half bit-parallel superset transform then sets bit (H) exactly when some such (W_H\subseteq H). Therefore, after the two transforms,
   [
   \text{row }L\text{ has bit }H
   \iff
   \exists W\text{ generated by the grammar with }W\subseteq(L,H).
   ]
   Intersecting with an exact-popcount mask counts each target (T) once, regardless of how many witnesses it contains. Thus the reported coverage and missing-mask counts are exact finite values, not samples or witness multiplicities.

9. **Uniform grammar coverage and finite exceptions.**
   [USES: steps 7–8; JSON coverage table.]
   Every equal-profile and double-low block is completely covered by the grammar. Singleton-low blocks are also complete except for these exact complement sets:

   [
   \begin{array}{c|c|c}
   j&|T|&\text{missing masks}\\hline
   2&14&25\
   4&20&1,575\
   5&23&105\
   6&26&105
   \end{array}
   ]

   These are the complete bitwise complements of the upward-closed languages, not sampled failures. For each missing mask, the exact trace recognizer of step 11 was run for each of the three possible singleton-low seat labels. All
   [
   3(25+1,575+105+105)=5,430
   ]
   labeled exceptions produced certificates, and every certificate was replayed through the from-scratch legality checker.

10. **The singleton-low (j=1) block.**
    [USES: looped-(K_7) domino model; step 6.]
    View the 28 dominoes as the edges of (K_8), where the eighth vertex represents doubles. Its eight vertex stars are the seven pip-incidence sets and the doubles set. Every tile belongs to exactly two stars.

    Here (|T|=11), so the total number of star incidences contributed by (T) is (22). If every star met (T) in at most two tiles, the total would be at most (8\cdot2=16), contradiction. Hence some star contains at least three tiles of (T).

    Declare that pip or doubles trump. Give the three highest selected star tiles to the hidden seats in the first trick, assigning the highest to the desired singleton-low seat and making that seat the leader. If the viewer holds another tile of the star, it must follow with a lower one; otherwise it legally sloughs. The designated hidden seat wins and then leads any fourth hidden public tile. The remaining seven complement tiles form the viewer’s initial hand. This realizes every (11)-tile complement for each singleton-low seat label.

11. **Exact realization algorithm.**
    [USES: steps 4 and 6–10; symbolic trace equivalence.]
    Given any counted pair ((U,k)), the program’s `find_void_free_trace` performs this finite deterministic search:

    1. set (T=\mathcal D\setminus U);
    2. enumerate all nine declarations;
    3. enumerate all four initial leaders;
    4. enumerate exactly (j) complete tricks and every current-trick actor schedule matching the low-seat set (B);
    5. allow every remaining (T)-tile on a hidden lead;
    6. allow only tiles of the led effective suit on hidden follower turns;
    7. retain viewer choices whose reconstructed seven-tile viewer hand makes every viewer play legal;
    8. return the lexicographically first accepted trace.

    This search is sound because every accepted branch is replayed under the full lead/follow/winner rules. It is complete because any void-free realization has one of the enumerated declarations, initial leaders, schedules, and tile choices, and every hidden follower in such a realization necessarily chooses one of the enumerated following tiles.

    `replay_certificate` partitions (U) into final hidden hands of sizes (k_s), adds each hidden public tile back to its actor, defines the viewer’s initial hand as the seven remaining complement tiles, and replays the entire trace. Step 4 then proves that the final support is exactly the unrestricted fiber (\Phi(U,k)), not merely that the displayed deal is one compatible world. This is also an explicit specialization of the attachment’s symbolic trace equivalence. 

12. **Constructive lower/completeness direction.**
    [USES: steps 6–11.]
    Let (k) satisfy (\max k-\min k\le1), and let (U) have size (\sum k_s).

    * The initial equal profile is realized with no play.
    * Singleton-low (j=1) is realized by step 10.
    * Every other complement (T) either contains a uniform grammar witness by steps 7–9 or belongs to one of the finite exception sets exhaustively realized in step 9.
    * Step 6 constructs a complete deal and legal prefix from a grammar witness.
    * Step 11 constructs and replays a certificate for every exception.

    Therefore every capacity-consistent pair ((U,k)) is no-void reachable.

13. **Upper/no-overcount direction.**
    [USES: steps 1–2.]
    Every reachable no-void normal form yields a unique pair ((U,k)). Its capacities satisfy the 50-profile condition, and conservation gives (|U|=\sum k_s). Thus every reachable no-void normal form is included in the counted set.

    Conversely, step 12 reaches every counted pair. Step 1 proves that different pairs give different normal forms. Hence the enumeration is exact in both directions.

14. **Evaluation of the census.**
    [USES: step 13; binomial subset count.]
    Equal profiles are
    [
    (h,h,h),\qquad 0\le h\le7,
    ]
    and contribute (\binom{28}{3h}) each.

    For each (0\le h\le6), the nonconstant profiles consist of the three ordered permutations of
    [
    (h,h,h+1)
    ]
    and the three ordered permutations of
    [
    (h,h+1,h+1).
    ]
    Therefore
    [
    \mathrm{NO_VOID_SLICE}
    ======================

    \sum_{h=0}^{7}\binom{28}{3h}
    +
    3\sum_{h=0}^{6}
    \left[
    \binom{28}{3h+1}+\binom{28}{3h+2}
    \right]
    =======

    624,892,870.
    ]

15. **Established-anchor recomputation.**
    [USES: adjudicated 001 facts; program’s independent group generation and upward-closure count.]
    The receipt does not merely assert the earlier no-void number. It regenerates the 001 admissible-module languages and their upward closures, reproducing
    [
    559,316,142.
    ]
    It separately evaluates
    [
    \binom{28}{21}
    +3\binom{28}{20}
    +3\binom{28}{19}
    +\binom{28}{18}
    ===============

    44,352,165,
    ]
    reproducing the corpus floor construction described in the foundation. 

16. **Restricted exhaustive cross-check.**
    [USES: Straight mechanics; step 4.]
    Fix the viewer’s hand to the seven doubles. For each representative declaration (0), `DT`, and `NT`, the program exhausts every void-free prefix through two completed tricks and up to three plays of the next trick.

    The first enumerator retains the full ordered current trick. The second independently folds it to
    [
    (\text{length},\text{context},\text{best key},\text{current winner}).
    ]
    Their exact sets of ((U,k)) outputs agree:

    [
    \begin{array}{c|r|r}
    \delta&\text{full-trick outputs}&\text{folded outputs}\\hline
    0&81,974&81,974\
    \mathrm{DT}&379&379\
    \mathrm{NT}&208,874&208,874
    \end{array}
    ]

17. **Realization spot-check.**
    [USES: step 11.]
    All 50 ordered profiles are visited. Within each profile’s lexicographically ranked complement block, the deterministic stride is
    [
    \max\left(1,\left\lfloor\binom{28}{|T|}/21\right\rfloor\right).
    ]
    The ranks
    [
    \min(i\cdot\text{stride},,\binom{28}{|T|}-1),
    \qquad i=0,\ldots,20,
    ]
    are deduplicated. This produces 1,030 tested pairs across all profiles, including ((0,0,0)), ((0,0,1)), and the other deepest endgame shapes. Every generated certificate replayed legally, produced no hidden void, and recovered the exact target (U) and (k).

18. **Conclusion.**
    [USES: steps 12–17.]
    Both completeness directions hold, the construction is explicit and executable, all 50 ordered capacity profiles are saturated, and
    [
    \boxed{\mathrm{NO_VOID_SLICE}=624,892,870}.
    ]

```json
{
  "no_void_shape_orbits": [
    {
      "shape": [0, 0, 0],
      "labelings": 1,
      "pool_size": 0,
      "per_labeling": 1,
      "contribution": 1
    },
    {
      "shape": [0, 0, 1],
      "labelings": 3,
      "pool_size": 1,
      "per_labeling": 28,
      "contribution": 84
    },
    {
      "shape": [0, 1, 1],
      "labelings": 3,
      "pool_size": 2,
      "per_labeling": 378,
      "contribution": 1134
    },
    {
      "shape": [1, 1, 1],
      "labelings": 1,
      "pool_size": 3,
      "per_labeling": 3276,
      "contribution": 3276
    },
    {
      "shape": [1, 1, 2],
      "labelings": 3,
      "pool_size": 4,
      "per_labeling": 20475,
      "contribution": 61425
    },
    {
      "shape": [1, 2, 2],
      "labelings": 3,
      "pool_size": 5,
      "per_labeling": 98280,
      "contribution": 294840
    },
    {
      "shape": [2, 2, 2],
      "labelings": 1,
      "pool_size": 6,
      "per_labeling": 376740,
      "contribution": 376740
    },
    {
      "shape": [2, 2, 3],
      "labelings": 3,
      "pool_size": 7,
      "per_labeling": 1184040,
      "contribution": 3552120
    },
    {
      "shape": [2, 3, 3],
      "labelings": 3,
      "pool_size": 8,
      "per_labeling": 3108105,
      "contribution": 9324315
    },
    {
      "shape": [3, 3, 3],
      "labelings": 1,
      "pool_size": 9,
      "per_labeling": 6906900,
      "contribution": 6906900
    },
    {
      "shape": [3, 3, 4],
      "labelings": 3,
      "pool_size": 10,
      "per_labeling": 13123110,
      "contribution": 39369330
    },
    {
      "shape": [3, 4, 4],
      "labelings": 3,
      "pool_size": 11,
      "per_labeling": 21474180,
      "contribution": 64422540
    },
    {
      "shape": [4, 4, 4],
      "labelings": 1,
      "pool_size": 12,
      "per_labeling": 30421755,
      "contribution": 30421755
    },
    {
      "shape": [4, 4, 5],
      "labelings": 3,
      "pool_size": 13,
      "per_labeling": 37442160,
      "contribution": 112326480
    },
    {
      "shape": [4, 5, 5],
      "labelings": 3,
      "pool_size": 14,
      "per_labeling": 40116600,
      "contribution": 120349800
    },
    {
      "shape": [5, 5, 5],
      "labelings": 1,
      "pool_size": 15,
      "per_labeling": 37442160,
      "contribution": 37442160
    },
    {
      "shape": [5, 5, 6],
      "labelings": 3,
      "pool_size": 16,
      "per_labeling": 30421755,
      "contribution": 91265265
    },
    {
      "shape": [5, 6, 6],
      "labelings": 3,
      "pool_size": 17,
      "per_labeling": 21474180,
      "contribution": 64422540
    },
    {
      "shape": [6, 6, 6],
      "labelings": 1,
      "pool_size": 18,
      "per_labeling": 13123110,
      "contribution": 13123110
    },
    {
      "shape": [6, 6, 7],
      "labelings": 3,
      "pool_size": 19,
      "per_labeling": 6906900,
      "contribution": 20720700
    },
    {
      "shape": [6, 7, 7],
      "labelings": 3,
      "pool_size": 20,
      "per_labeling": 3108105,
      "contribution": 9324315
    },
    {
      "shape": [7, 7, 7],
      "labelings": 1,
      "pool_size": 21,
      "per_labeling": 1184040,
      "contribution": 1184040
    }
  ],
  "total": 624892870,
  "increment_beyond_adjudicated_001_no_void": 65576728
}
```

```json
{
  "construction_notation": {
    "4": "ordinary four-tile all-follow module with selectable winner",
    "S": "strong hidden three-tile module; a hidden leader plays the leadable global maximum and remains leader",
    "1": "one hidden current-trick lead",
    "2": "two hidden current-trick plays, with a legal viewer filler when the low seats are separated",
    "3": "three hidden plays completed by one legal viewer filler"
  },
  "coverage_by_phase": [
    {
      "class": "equal",
      "j": 0,
      "T_size": 7,
      "patterns": ["initial-no-play"],
      "covered": 1184040,
      "total_masks": 1184040,
      "exceptions": 0
    },
    {
      "class": "equal",
      "j": 1,
      "T_size": 10,
      "patterns": ["3"],
      "covered": 13123110,
      "total_masks": 13123110,
      "exceptions": 0
    },
    {
      "class": "equal",
      "j": 2,
      "T_size": 13,
      "patterns": ["43", "S3"],
      "covered": 37442160,
      "total_masks": 37442160,
      "exceptions": 0
    },
    {
      "class": "equal",
      "j": 3,
      "T_size": 16,
      "patterns": ["443", "4S3"],
      "covered": 30421755,
      "total_masks": 30421755,
      "exceptions": 0
    },
    {
      "class": "equal",
      "j": 4,
      "T_size": 19,
      "patterns": ["4443"],
      "covered": 6906900,
      "total_masks": 6906900,
      "exceptions": 0
    },
    {
      "class": "equal",
      "j": 5,
      "T_size": 22,
      "patterns": ["44443"],
      "covered": 376740,
      "total_masks": 376740,
      "exceptions": 0
    },
    {
      "class": "equal",
      "j": 6,
      "T_size": 25,
      "patterns": ["444443"],
      "covered": 3276,
      "total_masks": 3276,
      "exceptions": 0
    },
    {
      "class": "equal",
      "j": 7,
      "T_size": 28,
      "patterns": ["4444443"],
      "covered": 1,
      "total_masks": 1,
      "exceptions": 0
    },
    {
      "class": "singleton_low",
      "j": 0,
      "T_size": 8,
      "patterns": ["1"],
      "covered": 3108105,
      "total_masks": 3108105,
      "exceptions": 0
    },
    {
      "class": "singleton_low",
      "j": 1,
      "T_size": 11,
      "patterns": ["K8-star-degree-at-least-3"],
      "covered": 21474180,
      "total_masks": 21474180,
      "exceptions": 0
    },
    {
      "class": "singleton_low",
      "j": 2,
      "T_size": 14,
      "patterns": ["441", "4S1", "SS1"],
      "covered": 40116575,
      "total_masks": 40116600,
      "exceptions": 25
    },
    {
      "class": "singleton_low",
      "j": 3,
      "T_size": 17,
      "patterns": ["4441", "44S1", "4SS1", "SSS1"],
      "covered": 21474180,
      "total_masks": 21474180,
      "exceptions": 0
    },
    {
      "class": "singleton_low",
      "j": 4,
      "T_size": 20,
      "patterns": ["44441"],
      "covered": 3106530,
      "total_masks": 3108105,
      "exceptions": 1575
    },
    {
      "class": "singleton_low",
      "j": 5,
      "T_size": 23,
      "patterns": ["444441"],
      "covered": 98175,
      "total_masks": 98280,
      "exceptions": 105
    },
    {
      "class": "singleton_low",
      "j": 6,
      "T_size": 26,
      "patterns": ["4444441"],
      "covered": 273,
      "total_masks": 378,
      "exceptions": 105
    },
    {
      "class": "double_low",
      "j": 0,
      "T_size": 9,
      "patterns": ["2"],
      "covered": 6906900,
      "total_masks": 6906900,
      "exceptions": 0
    },
    {
      "class": "double_low",
      "j": 1,
      "T_size": 12,
      "patterns": ["42", "S2"],
      "covered": 30421755,
      "total_masks": 30421755,
      "exceptions": 0
    },
    {
      "class": "double_low",
      "j": 2,
      "T_size": 15,
      "patterns": ["442", "4S2", "SS2"],
      "covered": 37442160,
      "total_masks": 37442160,
      "exceptions": 0
    },
    {
      "class": "double_low",
      "j": 3,
      "T_size": 18,
      "patterns": ["4442", "44S2"],
      "covered": 13123110,
      "total_masks": 13123110,
      "exceptions": 0
    },
    {
      "class": "double_low",
      "j": 4,
      "T_size": 21,
      "patterns": ["44442"],
      "covered": 1184040,
      "total_masks": 1184040,
      "exceptions": 0
    },
    {
      "class": "double_low",
      "j": 5,
      "T_size": 24,
      "patterns": ["444442"],
      "covered": 20475,
      "total_masks": 20475,
      "exceptions": 0
    },
    {
      "class": "double_low",
      "j": 6,
      "T_size": 27,
      "patterns": ["4444442"],
      "covered": 28,
      "total_masks": 28,
      "exceptions": 0
    }
  ],
  "exception_replays": {
    "unlabeled_masks": 1810,
    "singleton_low_labelings_per_mask": 3,
    "total_labeled_certificates_replayed": 5430
  }
}
```

```json
{
  "fixed_viewer_hand_crosscheck": {
    "viewer_hand": [
      [0, 0],
      [1, 1],
      [2, 2],
      [3, 3],
      [4, 4],
      [5, 5],
      [6, 6]
    ],
    "scope": "all void-free prefixes with at most two completed tricks and at most three plays of the next trick",
    "representatives": [
      {
        "declaration": 0,
        "full_trick_outputs": 81974,
        "folded_outputs": 81974,
        "full_trick_states_visited": 231541,
        "folded_states_visited": 144861
      },
      {
        "declaration": "DT",
        "full_trick_outputs": 379,
        "folded_outputs": 379,
        "full_trick_states_visited": 7466,
        "folded_states_visited": 6801
      },
      {
        "declaration": "NT",
        "full_trick_outputs": 208874,
        "folded_outputs": 208874,
        "full_trick_states_visited": 847627,
        "folded_states_visited": 504557
      }
    ]
  },
  "realization_stride": {
    "ordered_profiles": 50,
    "rule": "For a profile with complement block N=C(28,|T|), use stride=max(1,floor(N/21)); test the deduplicated ranks min(i*stride,N-1), i=0..20.",
    "certificates_replayed": 1030,
    "includes_deepest_shapes": true
  }
}
```

```json
{
  "explicit_template_exception_witness": {
    "T_mask": 90555626,
    "declaration": 0,
    "initial_leader": 0,
    "shaker": 3,
    "auction": [
      {
        "seat": 0,
        "action": "P(30)"
      },
      {
        "seat": 1,
        "action": "pass"
      },
      {
        "seat": 2,
        "action": "pass"
      },
      {
        "seat": 3,
        "action": "pass"
      }
    ],
    "capacities": [4, 5, 5],
    "pool_U": [
      [0, 0],
      [1, 1],
      [2, 1],
      [3, 2],
      [3, 3],
      [4, 1],
      [4, 2],
      [4, 3],
      [5, 2],
      [5, 4],
      [5, 5],
      [6, 2],
      [6, 4],
      [6, 6]
    ],
    "initial_hands": {
      "0": [
        [1, 0],
        [2, 2],
        [5, 0],
        [5, 1],
        [6, 0],
        [6, 1],
        [6, 5]
      ],
      "1": [
        [0, 0],
        [1, 1],
        [2, 0],
        [2, 1],
        [3, 2],
        [4, 4],
        [6, 3]
      ],
      "2": [
        [3, 0],
        [3, 3],
        [4, 1],
        [4, 2],
        [4, 3],
        [5, 2],
        [5, 3]
      ],
      "3": [
        [3, 1],
        [4, 0],
        [5, 4],
        [5, 5],
        [6, 2],
        [6, 4],
        [6, 6]
      ]
    },
    "trace": [
      {
        "seat": 0,
        "domino": [1, 0]
      },
      {
        "seat": 1,
        "domino": [2, 0]
      },
      {
        "seat": 2,
        "domino": [3, 0]
      },
      {
        "seat": 3,
        "domino": [4, 0]
      },
      {
        "seat": 3,
        "domino": [3, 1]
      },
      {
        "seat": 0,
        "domino": [2, 2]
      },
      {
        "seat": 1,
        "domino": [6, 3]
      },
      {
        "seat": 2,
        "domino": [5, 3]
      },
      {
        "seat": 1,
        "domino": [4, 4]
      }
    ],
    "final_hidden_hands": {
      "1": [
        [0, 0],
        [1, 1],
        [2, 1],
        [3, 2]
      ],
      "2": [
        [3, 3],
        [4, 1],
        [4, 2],
        [4, 3],
        [5, 2]
      ],
      "3": [
        [5, 4],
        [5, 5],
        [6, 2],
        [6, 4],
        [6, 6]
      ]
    },
    "hidden_void_sets": {
      "1": [],
      "2": [],
      "3": []
    }
  }
}
```

