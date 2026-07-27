#!/usr/bin/env python3
"""Self-contained verification of a future-equivalence collapse in the
reduced Straight Texas 42 viewer kernel.

No file or network I/O.  Standard library only.
"""
from __future__ import annotations

from dataclasses import dataclass
from functools import lru_cache
from itertools import combinations
from collections import deque
import json
import sys
from typing import Iterable, Optional

# ---------------------------------------------------------------------------
# Domino universe and Straight 42 rules
# ---------------------------------------------------------------------------

Domino = tuple[int, int]
DOMINOES: tuple[Domino, ...] = tuple(
    (h, l) for h in range(7) for l in range(h + 1)
)
ID_OF = {d: i for i, d in enumerate(DOMINOES)}
ALL_IDS = tuple(range(28))
NT = "NT"
DT = "DT"
Declaration = int | str
DECLARATIONS: tuple[Declaration, ...] = tuple(range(7)) + (DT, NT)
CALLED_CONTEXT = 7
VIEWER = 0
HIDDEN_ABS = (1, 2, 3)
ACTION_ALPHABET = tuple((seat, d) for seat in range(4) for d in ALL_IDS)


def did(h: int, l: int) -> int:
    return ID_OF[(max(h, l), min(h, l))]


def count_points(d: int) -> int:
    h, l = DOMINOES[d]
    total = h + l
    return total if total in (5, 10) else 0


def is_double(d: int) -> bool:
    h, l = DOMINOES[d]
    return h == l


def contains(d: int, pip: int) -> bool:
    h, l = DOMINOES[d]
    return h == pip or l == pip


def is_called(d: int, declaration: Declaration) -> bool:
    if isinstance(declaration, int):
        return contains(d, declaration)
    if declaration == DT:
        return is_double(d)
    if declaration == NT:
        return False
    raise ValueError(declaration)


def follows(d: int, q: int, declaration: Declaration) -> bool:
    if q == CALLED_CONTEXT:
        return is_called(d, declaration)
    return contains(d, q) and not is_called(d, declaration)


def led_context(d: int, declaration: Declaration) -> int:
    if is_called(d, declaration):
        return CALLED_CONTEXT
    return DOMINOES[d][0]


def rank_value(d: int, declaration: Declaration) -> int:
    h, l = DOMINOES[d]
    if declaration == DT and h == l:
        return h
    if h == l:
        return 100  # TOP, above every mixed sum
    return h + l


def trick_key(d: int, q: int, declaration: Declaration) -> tuple[int, int]:
    if declaration != NT and is_called(d, declaration):
        tier = 2
    elif not is_called(d, declaration) and follows(d, q, declaration):
        tier = 1
    else:
        tier = 0
    return (0, 0) if tier == 0 else (tier, rank_value(d, declaration))


@lru_cache(maxsize=None)
def competitive_ordinals(declaration: Declaration, q: int) -> tuple[int, ...]:
    competitive = [d for d in ALL_IDS if trick_key(d, q, declaration) != (0, 0)]
    competitive.sort(key=lambda d: trick_key(d, q, declaration))
    keys = [trick_key(d, q, declaration) for d in competitive]
    assert len(keys) == len(set(keys))
    result = [0] * 28
    for ordinal, d in enumerate(competitive, start=1):
        result[d] = ordinal
    return tuple(result)


def legal_complete_hand_plays(
    hand: set[int], trick: tuple[tuple[int, int], ...], declaration: Declaration
) -> set[int]:
    if not trick:
        return set(hand)
    q = led_context(trick[0][1], declaration)
    followers = {d for d in hand if follows(d, q, declaration)}
    return followers if followers else set(hand)


def resolve_trick(
    trick: tuple[tuple[int, int], ...], declaration: Declaration
) -> tuple[int, int]:
    assert len(trick) == 4
    q = led_context(trick[0][1], declaration)
    winner, _ = max(trick, key=lambda sd: trick_key(sd[1], q, declaration))
    points = 1 + sum(count_points(d) for _, d in trick)
    return winner, points

# ---------------------------------------------------------------------------
# Exact support normal form and matching-minor transition
# ---------------------------------------------------------------------------

# Immutable normal form representation:
#   ("F", certain[3], ambiguity)
# ambiguity:
#   ("D",)
#   ("B", inactive_hidden_index, W, residual_of_first_active)
#   ("T", W, r0, r1, epsilon_pairs)
Normal = tuple


def hall_feasible(
    universe: tuple[int, ...],
    capacities: tuple[int, int, int],
    possible: tuple[frozenset[int], frozenset[int], frozenset[int]],
) -> bool:
    U = frozenset(universe)
    if any(k < 0 for k in capacities) or sum(capacities) != len(universe):
        return False
    if any(not p <= U for p in possible):
        return False
    for mask in range(1, 8):
        seats = [s for s in range(3) if mask & (1 << s)]
        neighbors: set[int] = set()
        demand = 0
        for s in seats:
            neighbors.update(possible[s])
            demand += capacities[s]
        if len(neighbors) < demand:
            return False
    return True


def forced_successor_feasible(
    universe: tuple[int, ...],
    capacities: tuple[int, int, int],
    possible: tuple[frozenset[int], frozenset[int], frozenset[int]],
    d: int,
    seat: int,
) -> bool:
    if capacities[seat] <= 0 or d not in possible[seat]:
        return False
    U2 = tuple(x for x in universe if x != d)
    P2 = tuple(frozenset(x for x in p if x != d) for p in possible)
    k2 = list(capacities)
    k2[seat] -= 1
    return hall_feasible(U2, tuple(k2), P2)  # type: ignore[arg-type]


def normal_from_marginal(
    universe: tuple[int, ...],
    capacities: tuple[int, int, int],
    marginal: tuple[frozenset[int], frozenset[int], frozenset[int]],
) -> Normal:
    holders = {
        d: frozenset(s for s in range(3) if d in marginal[s]) for d in universe
    }
    assert all(holders[d] for d in universe)
    certain = tuple(
        tuple(d for d in universe if holders[d] == frozenset((s,)))
        for s in range(3)
    )
    certain_union = frozenset(d for hand in certain for d in hand)
    W = tuple(d for d in universe if d not in certain_union)
    residual = tuple(capacities[s] - len(certain[s]) for s in range(3))
    active = tuple(s for s in range(3) if residual[s] > 0)
    if not W:
        assert active == ()
        ambiguity = ("D",)
    elif len(active) == 2:
        inactive = next(s for s in range(3) if s not in active)
        active_set = frozenset(active)
        assert all(holders[d] == active_set for d in W)
        ambiguity = ("B", inactive, W, residual[active[0]])
    else:
        assert len(active) == 3
        epsilon = []
        for d in W:
            hs = holders[d]
            assert len(hs) in (2, 3)
            if len(hs) == 2:
                excluded = next(iter(frozenset(range(3)) - hs))
                epsilon.append((d, excluded))
        ambiguity = ("T", W, residual[0], residual[1], tuple(epsilon))
    return ("F", certain, ambiguity)


def reduce_to_normal(
    universe: tuple[int, ...],
    capacities: tuple[int, int, int],
    possible: tuple[frozenset[int], frozenset[int], frozenset[int]],
) -> Optional[Normal]:
    universe = tuple(sorted(universe))
    if not hall_feasible(universe, capacities, possible):
        return None
    marginal = [set() for _ in range(3)]
    for s in range(3):
        for d in sorted(possible[s]):
            if forced_successor_feasible(universe, capacities, possible, d, s):
                marginal[s].add(d)
    return normal_from_marginal(
        universe,
        capacities,
        tuple(frozenset(x) for x in marginal),  # type: ignore[arg-type]
    )


@lru_cache(maxsize=None)
def decode_normal(
    normal: Normal,
) -> tuple[
    tuple[int, ...],
    tuple[int, int, int],
    tuple[frozenset[int], frozenset[int], frozenset[int]],
]:
    assert normal[0] == "F"
    certain = tuple(frozenset(hand) for hand in normal[1])
    ambiguity = normal[2]
    if ambiguity[0] == "D":
        W: tuple[int, ...] = ()
        residual = (0, 0, 0)
        amb_possible = (frozenset(), frozenset(), frozenset())
    elif ambiguity[0] == "B":
        _, inactive, W, first = ambiguity
        active = tuple(s for s in range(3) if s != inactive)
        rr = [0, 0, 0]
        rr[active[0]] = first
        rr[active[1]] = len(W) - first
        residual = tuple(rr)
        amb_possible = tuple(
            frozenset(W) if s in active else frozenset() for s in range(3)
        )
    elif ambiguity[0] == "T":
        _, W, r0, r1, epsilon_pairs = ambiguity
        epsilon = dict(epsilon_pairs)
        residual = (r0, r1, len(W) - r0 - r1)
        amb_possible = tuple(
            frozenset(d for d in W if epsilon.get(d) != s) for s in range(3)
        )
    else:
        raise ValueError(ambiguity)
    universe = tuple(sorted(set(W).union(*certain)))
    capacities = tuple(len(certain[s]) + residual[s] for s in range(3))
    possible = tuple(certain[s] | amb_possible[s] for s in range(3))
    assert hall_feasible(universe, capacities, possible)
    return universe, capacities, possible  # type: ignore[return-value]


@lru_cache(maxsize=None)
def support_transition(
    normal: Normal,
    declaration: Declaration,
    hidden_index: int,
    d: int,
    q_or_boundary: int,
) -> Optional[Normal]:
    """Typed force/delete/contract/reduce.

    q_or_boundary == -1 means a lead. Otherwise it is the open trick context;
    follows(d,q) chooses successful-follow versus slough conditioning.
    """
    universe, capacities, possible = decode_normal(normal)
    if d not in universe or capacities[hidden_index] <= 0:
        return None

    conditioned = list(possible)
    if q_or_boundary != -1:
        q = q_or_boundary
        if not follows(d, q, declaration):
            follower_tiles = frozenset(
                e for e in universe if follows(e, q, declaration)
            )
            conditioned[hidden_index] = conditioned[hidden_index] - follower_tiles

    if d not in conditioned[hidden_index]:
        return None

    U2 = tuple(e for e in universe if e != d)
    P2 = tuple(frozenset(e for e in p if e != d) for p in conditioned)
    k2 = list(capacities)
    k2[hidden_index] -= 1
    return reduce_to_normal(U2, tuple(k2), P2)  # type: ignore[arg-type]


def world_in_support(
    normal: Normal,
    hidden_hands: tuple[set[int], set[int], set[int]],
) -> bool:
    universe, capacities, possible = decode_normal(normal)
    if tuple(len(h) for h in hidden_hands) != capacities:
        return False
    if set().union(*hidden_hands) != set(universe):
        return False
    if any(not hidden_hands[s] <= set(possible[s]) for s in range(3)):
        return False
    return all(
        hidden_hands[a].isdisjoint(hidden_hands[b])
        for a, b in combinations(range(3), 2)
    )

# ---------------------------------------------------------------------------
# Folded kernel and named accumulator interface
# ---------------------------------------------------------------------------


@dataclass(frozen=True)
class Kernel:
    declaration: Declaration
    viewer_hand: tuple[int, ...]
    normal: Normal
    tau: tuple
    alpha: int


BIDDER = 3
POINT_THRESHOLD = 30
ACCUMULATOR_INTERFACE = "P30_DECLARING_POINTS"


def hand_sizes(k: Kernel) -> tuple[int, int, int, int]:
    _, caps, _ = decode_normal(k.normal)
    return (len(k.viewer_hand), caps[0], caps[1], caps[2])


def terminal_label(k: Kernel) -> Optional[str]:
    if any(hand_sizes(k)):
        return None
    return "MADE" if k.alpha >= POINT_THRESHOLD else "SET"


def actor_and_open_length(k: Kernel) -> tuple[int, int]:
    sizes = hand_sizes(k)
    if k.tau[0] == "B":
        assert len(set(sizes)) == 1
        return k.tau[1], 0

    low, high = min(sizes), max(sizes)
    assert high == low + 1
    lows = frozenset(s for s, n in enumerate(sizes) if n == low)
    assert 1 <= len(lows) <= 3
    leaders = [s for s in lows if (s - 1) % 4 not in lows]
    assert len(leaders) == 1
    leader = leaders[0]
    made = len(lows)
    assert lows == frozenset((leader + i) % 4 for i in range(made))
    return (leader + made) % 4, made


@lru_cache(maxsize=None)
def accepted_actions(k: Kernel) -> tuple[tuple[int, int], ...]:
    if terminal_label(k) is not None:
        return ()

    actor, _ = actor_and_open_length(k)
    if actor == VIEWER:
        hand = set(k.viewer_hand)
        if k.tau[0] == "B":
            legal = hand
        else:
            q = k.tau[1]
            follower_tiles = {d for d in hand if follows(d, q, k.declaration)}
            legal = follower_tiles if follower_tiles else hand
        return tuple((actor, d) for d in sorted(legal))

    hidden_index = actor - 1
    universe, _, _ = decode_normal(k.normal)
    q_or_boundary = -1 if k.tau[0] == "B" else k.tau[1]
    accepted = []
    for d in universe:
        successor = support_transition(
            k.normal,
            k.declaration,
            hidden_index,
            d,
            q_or_boundary,
        )
        if successor is not None:
            accepted.append((actor, d))
    return tuple(accepted)


def state_output(k: Kernel) -> tuple:
    return accepted_actions(k), k.normal, terminal_label(k)


def winner_side_name(winner: int) -> str:
    return "DECLARING" if winner % 2 == BIDDER % 2 else "DEFENDING"


def step(k: Kernel, action: tuple[int, int]) -> tuple[Kernel, tuple]:
    if action not in set(accepted_actions(k)):
        raise ValueError("ILLEGAL")

    actor, d = action
    _, open_length = actor_and_open_length(k)

    if actor == VIEWER:
        viewer_hand2 = tuple(x for x in k.viewer_hand if x != d)
        assert len(viewer_hand2) + 1 == len(k.viewer_hand)
        normal2 = k.normal
    else:
        q_or_boundary = -1 if k.tau[0] == "B" else k.tau[1]
        normal2 = support_transition(
            k.normal,
            k.declaration,
            actor - 1,
            d,
            q_or_boundary,
        )
        assert normal2 is not None
        viewer_hand2 = k.viewer_hand

    reward_output: Optional[tuple[str, int]] = None
    alpha2 = k.alpha

    if k.tau[0] == "B":
        q = led_context(d, k.declaration)
        r = competitive_ordinals(k.declaration, q)[d]
        assert r >= 1
        tau2 = ("O", q, r, actor, count_points(d))
    else:
        _, q, r, w, z = k.tau
        rd = competitive_ordinals(k.declaration, q)[d]
        if rd > r:
            r, w = rd, actor
        z += count_points(d)

        if open_length == 3:
            points = 1 + z
            reward_output = (winner_side_name(w), points)
            if w % 2 == BIDDER % 2:
                alpha2 += points
            tau2 = ("B", w)
        else:
            tau2 = ("O", q, r, w, z)

    k2 = Kernel(k.declaration, viewer_hand2, normal2, tau2, alpha2)
    emission = k2.normal, reward_output, terminal_label(k2)
    return k2, emission

# ---------------------------------------------------------------------------
# Witnesses
# ---------------------------------------------------------------------------

DEAL_DOMINOES = (
    ((6, 6), (6, 5), (0, 0), (1, 0), (1, 1), (2, 0), (2, 1)),
    ((6, 4), (2, 2), (3, 0), (3, 1), (3, 2), (3, 3), (4, 0)),
    ((6, 3), (4, 1), (4, 2), (4, 3), (4, 4), (5, 0), (5, 1)),
    ((6, 0), (5, 2), (5, 3), (5, 4), (5, 5), (6, 1), (6, 2)),
)
DEAL_IDS = tuple(tuple(did(*d) for d in hand) for hand in DEAL_DOMINOES)

PREFIX_1 = (
    (3, did(6, 0)),
    (0, did(6, 5)),
    (1, did(6, 4)),
    (2, did(6, 3)),
    (0, did(6, 6)),
)
PREFIX_2 = (
    (3, did(6, 0)),
    (0, did(6, 6)),
    (1, did(6, 4)),
    (2, did(6, 3)),
    (0, did(6, 5)),
)

EXPECTED_W = tuple(
    did(*d)
    for d in (
        (2, 2),
        (3, 0),
        (3, 1),
        (3, 2),
        (3, 3),
        (4, 0),
        (4, 1),
        (4, 2),
        (4, 3),
        (4, 4),
        (5, 0),
        (5, 1),
        (5, 2),
        (5, 3),
        (5, 4),
        (5, 5),
        (6, 1),
        (6, 2),
    )
)

EXPECTED_NORMAL: Normal = (
    "F",
    ((), (), ()),
    ("T", EXPECTED_W, 6, 6, ()),
)
EXPECTED_VIEWER_HAND = tuple(
    did(*d)
    for d in ((0, 0), (1, 0), (1, 1), (2, 0), (2, 1))
)
EXPECTED_K1 = Kernel(
    NT,
    EXPECTED_VIEWER_HAND,
    EXPECTED_NORMAL,
    ("O", 6, 7, 0, 0),
    0,
)
EXPECTED_K2 = Kernel(
    NT,
    EXPECTED_VIEWER_HAND,
    EXPECTED_NORMAL,
    ("O", 6, 6, 0, 0),
    0,
)


def verify_auction() -> None:
    shaker = 2
    order = tuple((shaker + 1 + i) % 4 for i in range(4))
    assert order == (3, 0, 1, 2)

    actions = (
        (3, ("P", 30)),
        (0, "pass"),
        (1, "pass"),
        (2, "pass"),
    )
    assert tuple(actor for actor, _ in actions) == order
    assert actions[0][1] == ("P", 30)
    assert all(bid == "pass" for _, bid in actions[1:])
    assert BIDDER == 3


def initial_kernel() -> Kernel:
    viewer_hand = tuple(sorted(DEAL_IDS[VIEWER]))
    viewer_set = set(viewer_hand)
    universe = tuple(d for d in ALL_IDS if d not in viewer_set)
    possible = (
        frozenset(universe),
        frozenset(universe),
        frozenset(universe),
    )
    normal = reduce_to_normal(universe, (7, 7, 7), possible)
    assert normal is not None
    return Kernel(NT, viewer_hand, normal, ("B", BIDDER), 0)


def replay_witness(prefix: tuple[tuple[int, int], ...]) -> Kernel:
    hands = [set(hand) for hand in DEAL_IDS]
    leader = BIDDER
    trick: tuple[tuple[int, int], ...] = ()
    objective_declaring_points = 0

    kernel = initial_kernel()
    assert world_in_support(
        kernel.normal,
        (hands[1], hands[2], hands[3]),
    )

    for expected_actor, d in prefix:
        actual_actor = (leader + len(trick)) % 4
        assert actual_actor == expected_actor
        legal = legal_complete_hand_plays(
            hands[actual_actor],
            trick,
            NT,
        )
        assert d in legal

        kernel, _ = step(kernel, (expected_actor, d))

        hands[actual_actor].remove(d)
        trick += ((actual_actor, d),)

        if len(trick) == 4:
            leader, points = resolve_trick(trick, NT)
            if leader % 2 == BIDDER % 2:
                objective_declaring_points += points
            trick = ()

        assert tuple(sorted(hands[VIEWER])) == kernel.viewer_hand
        assert world_in_support(
            kernel.normal,
            (hands[1], hands[2], hands[3]),
        )
        assert objective_declaring_points == kernel.alpha

    return kernel

# ---------------------------------------------------------------------------
# Exact synchronized-product exploration
# ---------------------------------------------------------------------------


def check_product_equivalence(
    left: Kernel,
    right: Kernel,
) -> dict[str, int]:
    queue = deque([(left, right)])
    seen: set[tuple[Kernel, Kernel]] = set()

    pairs = 0
    alphabet_checks = 0
    legal_edges = 0
    diagonal_closures = 0
    max_grade = 0

    while queue:
        a, b = queue.popleft()
        if (a, b) in seen:
            continue
        seen.add((a, b))

        pairs += 1
        max_grade = max(max_grade, sum(hand_sizes(a)))

        if state_output(a) != state_output(b):
            raise AssertionError(
                (
                    "state-output mismatch",
                    a,
                    b,
                    state_output(a),
                    state_output(b),
                )
            )

        legal_a = set(accepted_actions(a))
        legal_b = set(accepted_actions(b))

        for action in ACTION_ALPHABET:
            alphabet_checks += 1
            in_a = action in legal_a
            in_b = action in legal_b

            if in_a != in_b:
                raise AssertionError(
                    ("legality mismatch", action, a, b)
                )

            if not in_a:
                continue

            legal_edges += 1
            a2, out_a = step(a, action)
            b2, out_b = step(b, action)

            if out_a != out_b:
                raise AssertionError(
                    (
                        "transition-output mismatch",
                        action,
                        out_a,
                        out_b,
                    )
                )

            if a2 == b2:
                diagonal_closures += 1
            else:
                queue.append((a2, b2))

    return {
        "product_pairs": pairs,
        "alphabet_checks": alphabet_checks,
        "legal_edges": legal_edges,
        "diagonal_closures": diagonal_closures,
        "initial_grade": sum(hand_sizes(left)),
        "max_grade_seen": max_grade,
    }

# ---------------------------------------------------------------------------
# JSON rendering and checks
# ---------------------------------------------------------------------------


def domino_json(d: int) -> list[int]:
    h, l = DOMINOES[d]
    return [h, l]


def normal_json(normal: Normal) -> dict:
    _, certain, ambiguity = normal
    result = {
        "certain": [
            [domino_json(d) for d in hand]
            for hand in certain
        ]
    }

    if ambiguity[0] == "D":
        payload = {"tag": "Determinate"}
    elif ambiguity[0] == "B":
        _, inactive, W, split = ambiguity
        payload = {
            "tag": "Binary",
            "inactive": inactive + 1,
            "W": [domino_json(d) for d in W],
            "split": split,
        }
    else:
        _, W, r0, r1, epsilon = ambiguity
        payload = {
            "tag": "Ternary",
            "W": [domino_json(d) for d in W],
            "r_0": r0,
            "r_1": r1,
            "epsilon": [
                {
                    "tile": domino_json(d),
                    "excluded_hidden_index": seat,
                }
                for d, seat in epsilon
            ],
        }

    result["ambiguity"] = payload
    return result


def kernel_json(
    k: Kernel,
    prefix: tuple[tuple[int, int], ...],
) -> dict:
    if k.tau[0] == "B":
        trick = {
            "boundary_leader": k.tau[1],
            "hand_sizes": list(hand_sizes(k)),
        }
    else:
        _, q, r, w, z = k.tau
        trick = {
            "q": q,
            "r": r,
            "w": w,
            "z": z,
            "hand_sizes": list(hand_sizes(k)),
        }

    return {
        "declaration": k.declaration,
        "viewer": VIEWER,
        "viewer_hand": [
            domino_json(d) for d in k.viewer_hand
        ],
        "normal_form": normal_json(k.normal),
        "trick": trick,
        "accumulator": {
            "interface": ACCUMULATOR_INTERFACE,
            "value": k.alpha,
            "bidder": BIDDER,
            "threshold": POINT_THRESHOLD,
        },
        "witness_prefix": {
            "shaker": 2,
            "auction": [
                [3, "P(30)"],
                [0, "pass"],
                [1, "pass"],
                [2, "pass"],
            ],
            "deal": [
                [list(d) for d in hand]
                for hand in DEAL_DOMINOES
            ],
            "bid_winner": BIDDER,
            "bid": "P(30)",
            "declaration": NT,
            "plays": [
                [seat, domino_json(d)]
                for seat, d in prefix
            ],
        },
    }


def fail(name: str, exc: BaseException) -> None:
    print(f"FAIL {name} {type(exc).__name__}: {exc}")
    raise SystemExit(1)


def main() -> None:
    try:
        assert len(DOMINOES) == 28
        assert len(set(DOMINOES)) == 28
        assert sum(count_points(d) for d in ALL_IDS) == 35

        for declaration in DECLARATIONS:
            for lead in ALL_IDS:
                q = led_context(lead, declaration)
                assert trick_key(lead, q, declaration) != (0, 0)

        print("PASS rules")
    except BaseException as exc:
        fail("rules", exc)

    try:
        verify_auction()
        flat = [d for hand in DEAL_IDS for d in hand]
        assert len(flat) == 28
        assert set(flat) == set(ALL_IDS)
        print("PASS deal_and_auction")
    except BaseException as exc:
        fail("deal_and_auction", exc)

    try:
        k1 = replay_witness(PREFIX_1)
        assert k1 == EXPECTED_K1
        print("PASS witness_1")
    except BaseException as exc:
        fail("witness_1", exc)

    try:
        k2 = replay_witness(PREFIX_2)
        assert k2 == EXPECTED_K2
        print("PASS witness_2")
    except BaseException as exc:
        fail("witness_2", exc)

    try:
        assert k1 != k2
        assert k1.declaration == k2.declaration == NT
        assert k1.viewer_hand == k2.viewer_hand
        assert k1.normal == k2.normal
        assert k1.alpha == k2.alpha == 0
        assert k1.tau == ("O", 6, 7, 0, 0)
        assert k2.tau == ("O", 6, 6, 0, 0)

        ordinals = competitive_ordinals(NT, 6)
        assert ordinals[did(6, 5)] == 6
        assert ordinals[did(6, 6)] == 7

        live, _, _ = decode_normal(k1.normal)
        assert did(6, 6) not in live
        assert max(
            (ordinals[d] for d in live),
            default=0,
        ) <= 3

        print("PASS distinct_kernels")
    except BaseException as exc:
        fail("distinct_kernels", exc)

    try:
        stats = check_product_equivalence(k1, k2)
        assert stats["product_pairs"] > 0
        assert stats["diagonal_closures"] > 0
        print(
            "PASS synchronized_product "
            + " ".join(
                f"{key}={value}"
                for key, value in stats.items()
            )
        )
    except BaseException as exc:
        fail("synchronized_product", exc)

    try:
        j1 = kernel_json(k1, PREFIX_1)
        j2 = kernel_json(k2, PREFIX_2)

        assert j1["trick"]["r"] == 7
        assert j2["trick"]["r"] == 6
        assert j1["viewer_hand"] == j2["viewer_hand"]
        assert j1["normal_form"] == j2["normal_form"]

        json.loads(json.dumps(j1, sort_keys=True))
        json.loads(json.dumps(j2, sort_keys=True))
        print("PASS json_artifacts")
    except BaseException as exc:
        fail("json_artifacts", exc)

    print("PASS OPEN-01_COLLAPSE")


if __name__ == "__main__":
    main()
