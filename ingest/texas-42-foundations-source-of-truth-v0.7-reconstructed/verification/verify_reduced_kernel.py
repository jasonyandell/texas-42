#!/usr/bin/env python3
"""Finite checks for the reduced Texas 42 play/support kernel.

This proof receipt verifies the finite statements added in the v0.7 reduction
pass.  It is intentionally separate from any reference implementation.
"""

from __future__ import annotations

from collections import defaultdict, deque
from functools import lru_cache
from itertools import combinations, product
from typing import Iterable, Sequence

import verify_foundation as base
import verify_minimality_and_reachability as support

PIPS = tuple(range(7))
DECLARATIONS = base.DECLARATIONS
CALLED = base.CALLED
SEATS4 = tuple(range(4))
HIDDEN = tuple(range(3))


def sign(value: tuple[int, int], other: tuple[int, int]) -> int:
    return (value > other) - (value < other)


def pip_transport(source: int, target: int) -> tuple[int, ...]:
    """Order-preserving transport off the called pip, with source -> target."""

    source_rest = tuple(p for p in PIPS if p != source)
    target_rest = tuple(p for p in PIPS if p != target)
    mapping = [0] * 7
    mapping[source] = target
    for left, right in zip(source_rest, target_rest, strict=True):
        mapping[left] = right
    assert sorted(mapping) == list(PIPS)
    return tuple(mapping)


def transport_domino(domino: int, mapping: tuple[int, ...]) -> int:
    high, low = base.DOMINOES[domino]
    image = (max(mapping[high], mapping[low]), min(mapping[high], mapping[low]))
    return base.ID_OF[image]


def transport_context(context: int, mapping: tuple[int, ...]) -> int:
    return CALLED if context == CALLED else mapping[context]


def check_looped_graph_count_and_unscored_transports() -> dict[str, int]:
    assert len(base.DOMINOES) == 28

    # D = Sym^2(P): the edge set of complete K7 with one loop per vertex.
    expected_edges = {
        (max(a, b), min(a, b)) for a in PIPS for b in PIPS
    }
    assert set(base.DOMINOES) == expected_edges
    for pip in PIPS:
        closed_star = {
            domino for domino, endpoints in enumerate(base.DOMINOES)
            if pip in endpoints
        }
        assert len(closed_star) == 7
        assert closed_star == {
            domino for domino in range(28) if base.contains(domino, pip)
        }

    # Count is exactly the sum-5/sum-10 antidiagonal decoration.
    for domino, (high, low) in enumerate(base.DOMINOES):
        expected = high + low if high + low in (5, 10) else 0
        assert base.count_points(domino) == expected

    relation_comparisons = 0
    led_checks = 0
    for source in PIPS:
        for target in PIPS:
            mapping = pip_transport(source, target)
            image = tuple(transport_domino(d, mapping) for d in range(28))
            assert len(set(image)) == 28

            for domino in range(28):
                mapped = image[domino]
                assert base.is_called(domino, source) == base.is_called(mapped, target)
                assert base.is_powered(domino, source) == base.is_powered(mapped, target)
                expected_suits = frozenset(
                    transport_context(q, mapping)
                    for q in base.effective_suits(domino, source)
                )
                assert base.effective_suits(mapped, target) == expected_suits
                assert transport_context(
                    base.led_suit(domino, source), mapping
                ) == base.led_suit(mapped, target)
                led_checks += 1

            for context in range(8):
                mapped_context = transport_context(context, mapping)
                for domino in range(28):
                    mapped = image[domino]
                    assert base.follows(domino, context, source) == base.follows(
                        mapped, mapped_context, target
                    )
                for left in range(28):
                    mapped_left = image[left]
                    left_key = base.trick_key(left, context, source)
                    mapped_left_key = base.trick_key(
                        mapped_left, mapped_context, target
                    )
                    for right in range(28):
                        mapped_right = image[right]
                        assert sign(
                            left_key, base.trick_key(right, context, source)
                        ) == sign(
                            mapped_left_key,
                            base.trick_key(mapped_right, mapped_context, target),
                        )
                        relation_comparisons += 1

    # Invariants distinguish exactly three unscored mechanics classes.
    signatures: dict[tuple[int, int], list[int]] = defaultdict(list)
    for declaration in DECLARATIONS:
        powered = sum(base.is_powered(d, declaration) for d in range(28))
        unpowered_degree_one = sum(
            (not base.is_powered(d, declaration))
            and len(base.effective_suits(d, declaration)) == 1
            for d in range(28)
        )
        signatures[(powered, unpowered_degree_one)].append(declaration)
    assert sorted(signatures.values(), key=lambda xs: (len(xs), xs)) == [
        [base.DOUBLES_TRUMP],
        [base.NO_TRUMP],
        list(PIPS),
    ]

    return {
        "ordered_pip_transports": 49,
        "led_checks": led_checks,
        "order_comparisons": relation_comparisons,
        "mechanics_classes": len(signatures),
    }


@lru_cache(maxsize=None)
def competitive_ordinals(declaration: int, context: int) -> dict[int, int]:
    competitive = [
        domino
        for domino in range(28)
        if base.trick_key(domino, context, declaration) != (0, 0)
    ]
    competitive.sort(key=lambda domino: base.trick_key(domino, context, declaration))
    keys = [base.trick_key(domino, context, declaration) for domino in competitive]
    assert len(keys) == len(set(keys))
    return {domino: index + 1 for index, domino in enumerate(competitive)}


def start_fold(
    declaration: int, leader: int, domino: int
) -> tuple[int, int, int, int]:
    context = base.led_suit(domino, declaration)
    ordinal = competitive_ordinals(declaration, context)[domino]
    return context, ordinal, leader, base.count_points(domino)


def advance_fold(
    declaration: int,
    folded: tuple[int, int, int, int],
    actor: int,
    domino: int,
) -> tuple[int, int, int, int]:
    context, current_ordinal, current_winner, pending_count = folded
    ordinal = competitive_ordinals(declaration, context).get(domino, 0)
    if ordinal > current_ordinal:
        current_ordinal = ordinal
        current_winner = actor
    return (
        context,
        current_ordinal,
        current_winner,
        pending_count + base.count_points(domino),
    )


def check_folded_trick() -> dict[str, int]:
    trick_cases = 0
    fold_updates = 0
    max_chain = 0
    pending_values: set[int] = set()

    for declaration in DECLARATIONS:
        for context in range(8):
            max_chain = max(max_chain, len(competitive_ordinals(declaration, context)))
        for lead in range(28):
            remaining = tuple(d for d in range(28) if d != lead)
            for followers in combinations(remaining, 3):
                plays = ((0, lead), (1, followers[0]), (2, followers[1]), (3, followers[2]))
                folded = start_fold(declaration, 0, lead)
                pending_values.add(folded[3])
                for actor, domino in plays[1:]:
                    folded = advance_fold(declaration, folded, actor, domino)
                    fold_updates += 1
                    pending_values.add(folded[3])
                winner, points = base.resolve_trick(plays, declaration)
                assert folded[2] == winner
                assert 1 + folded[3] == points
                trick_cases += 1

    assert trick_cases == 737_100
    assert fold_updates == 2_211_300
    assert max_chain == 13
    # During an unresolved trick there are at most three tiles, hence at most
    # two tens and one five: 25 count points.
    unresolved_values = set()
    for declaration in DECLARATIONS:
        for lead in range(28):
            fold = start_fold(declaration, 0, lead)
            unresolved_values.add(fold[3])
            for second in range(28):
                if second == lead:
                    continue
                fold2 = advance_fold(declaration, fold, 1, second)
                unresolved_values.add(fold2[3])
                for third in range(second + 1, 28):
                    if third == lead:
                        continue
                    fold3 = advance_fold(declaration, fold2, 2, third)
                    unresolved_values.add(fold3[3])
    assert unresolved_values == {0, 5, 10, 15, 20, 25}

    return {
        "trick_cases": trick_cases,
        "fold_updates": fold_updates,
        "max_competitive_chain": max_chain,
        "pending_count_values": len(unresolved_values),
    }


def decode_actor_from_capacities(
    capacities: tuple[int, int, int, int]
) -> tuple[int, int, int]:
    low = min(capacities)
    high = max(capacities)
    assert high == low + 1
    low_seats = {s for s, value in enumerate(capacities) if value == low}
    assert 1 <= len(low_seats) <= 3
    leaders = [s for s in low_seats if (s - 1) % 4 not in low_seats]
    assert len(leaders) == 1
    leader = leaders[0]
    plays_made = len(low_seats)
    assert low_seats == {(leader + i) % 4 for i in range(plays_made)}
    actor = (leader + plays_made) % 4
    return leader, plays_made, actor


def check_actor_capacity_shapes() -> int:
    checked = 0
    for hand_size_at_boundary in range(1, 8):
        for leader in range(4):
            for plays_made in range(1, 4):
                capacities = [hand_size_at_boundary] * 4
                for offset in range(plays_made):
                    capacities[(leader + offset) % 4] -= 1
                assert decode_actor_from_capacities(tuple(capacities)) == (
                    leader,
                    plays_made,
                    (leader + plays_made) % 4,
                )
                checked += 1
    assert checked == 84
    return checked


def replay_prefix_state(
    initial_hands: tuple[tuple[int, ...], ...],
    initial_leader: int,
    plays: tuple[tuple[int, int], ...],
    declaration: int,
) -> tuple[
    tuple[tuple[int, ...], ...],
    int,
    tuple[tuple[int, int], ...],
    tuple[int, int],
]:
    hands = [list(hand) for hand in initial_hands]
    leader = initial_leader
    trick: tuple[tuple[int, int], ...] = ()
    scores = [0, 0]
    for expected_actor, domino in plays:
        actor = (leader + len(trick)) % 4
        assert actor == expected_actor
        assert domino in base.legal_plays(tuple(hands[actor]), trick, declaration)
        hands[actor].remove(domino)
        trick += ((actor, domino),)
        if len(trick) == 4:
            leader, points = base.resolve_trick(trick, declaration)
            scores[leader % 2] += points
            trick = ()
    return base.normalized_hands(hands), leader, trick, (scores[0], scores[1])


def check_score_recovery() -> int:
    checked = 0
    for declaration in DECLARATIONS:
        for case in range(12):
            initial_hands, initial_leader, complete_plays = (
                base.generate_deterministic_contracted_hand(declaration, case)
            )
            viewer = (case + 2 * declaration) % 4
            for prefix_length in range(29):
                remaining, _leader, trick, scores = replay_prefix_state(
                    initial_hands,
                    initial_leader,
                    complete_plays[:prefix_length],
                    declaration,
                )
                own = set(remaining[viewer])
                hidden_pool = set().union(
                    *(set(remaining[s]) for s in range(4) if s != viewer)
                )
                pending_count = sum(base.count_points(d) for _, d in trick)
                completed = (28 - len(own) - len(hidden_pool) - len(trick)) // 4
                recovered_banked_count = (
                    35
                    - sum(base.count_points(d) for d in own)
                    - sum(base.count_points(d) for d in hidden_pool)
                    - pending_count
                )
                recovered_total = completed + recovered_banked_count
                assert recovered_total == sum(scores)
                assert 28 - len(own) - len(hidden_pool) - len(trick) == 4 * completed
                checked += 1
    assert checked == 3_132
    return checked


def normal_to_graph(
    normal: tuple,
) -> tuple[tuple[int, ...], tuple[int, int, int], tuple[frozenset[int], ...]]:
    assert normal != ("EMPTY",)
    _, certain, ambiguity = normal
    certain_sets = tuple(frozenset(hand) for hand in certain)
    if ambiguity[0] == "D":
        ambiguous: tuple[int, ...] = ()
        residual = (0, 0, 0)
        possible_amb = (frozenset(), frozenset(), frozenset())
    elif ambiguity[0] == "B":
        _, inactive, ambiguous, first = ambiguity
        active = tuple(s for s in range(3) if s != inactive)
        residual_list = [0, 0, 0]
        residual_list[active[0]] = first
        residual_list[active[1]] = len(ambiguous) - first
        residual = tuple(residual_list)
        possible_amb = tuple(
            frozenset(ambiguous) if s in active else frozenset()
            for s in range(3)
        )
    else:
        _, ambiguous, r0, r1, exclusions = ambiguity
        excluded = dict(exclusions)
        residual = (r0, r1, len(ambiguous) - r0 - r1)
        possible_amb = tuple(
            frozenset(d for d in ambiguous if excluded.get(d) != s)
            for s in range(3)
        )
    universe = tuple(sorted(set().union(*certain_sets, set(ambiguous))))
    capacities = tuple(len(certain_sets[s]) + residual[s] for s in range(3))
    possible = tuple(certain_sets[s] | possible_amb[s] for s in range(3))
    return universe, capacities, possible


def normal_from_marginal(
    universe: tuple[int, ...],
    capacities: tuple[int, int, int],
    marginal: tuple[frozenset[int], ...],
) -> tuple:
    holders = {
        d: frozenset(s for s in range(3) if d in marginal[s]) for d in universe
    }
    assert all(holders[d] for d in universe)
    certain = tuple(
        tuple(sorted(d for d in universe if holders[d] == {s}))
        for s in range(3)
    )
    certain_union = {d for hand in certain for d in hand}
    ambiguous = tuple(d for d in universe if d not in certain_union)
    residual = tuple(capacities[s] - len(certain[s]) for s in range(3))
    active = tuple(s for s in range(3) if residual[s] > 0)
    if not active:
        assert not ambiguous
        ambiguity = ("D",)
    elif len(active) == 2:
        inactive = next(s for s in range(3) if s not in active)
        assert all(holders[d] == frozenset(active) for d in ambiguous)
        ambiguity = ("B", inactive, ambiguous, residual[active[0]])
    else:
        assert len(active) == 3
        exclusions = tuple(
            sorted(
                (d, next(iter(set(range(3)) - set(holders[d]))))
                for d in ambiguous
                if len(holders[d]) == 2
            )
        )
        assert all(len(holders[d]) in (2, 3) for d in ambiguous)
        ambiguity = ("T", ambiguous, residual[0], residual[1], exclusions)
    return ("F", certain, ambiguity)


def hall_reduce(
    universe: tuple[int, ...],
    capacities: tuple[int, int, int],
    possible: tuple[frozenset[int], ...],
) -> tuple:
    if not base.hall_feasible(universe, possible, capacities):
        return ("EMPTY",)
    marginal: list[set[int]] = [set() for _ in range(3)]
    for seat in range(3):
        for domino in possible[seat]:
            successor = base.forced_successor_system(
                universe, possible, capacities, domino, seat
            )
            if successor is not None and base.hall_feasible(*successor):
                marginal[seat].add(domino)
    return normal_from_marginal(
        universe, capacities, tuple(frozenset(x) for x in marginal)
    )


def direct_support_transition(
    normal: tuple,
    actor: int,
    domino: int,
    kind: str,
    follower_set: frozenset[int] = frozenset(),
) -> tuple:
    universe, capacities, possible = normal_to_graph(normal)
    if domino not in possible[actor] or capacities[actor] == 0:
        return ("EMPTY",)
    if kind == "follow" and domino not in follower_set:
        return ("EMPTY",)
    if kind == "slough" and domino in follower_set:
        return ("EMPTY",)

    conditioned = list(possible)
    if kind == "slough":
        conditioned[actor] = conditioned[actor] - follower_set
        if domino not in conditioned[actor]:
            return ("EMPTY",)
    elif kind not in ("lead", "follow"):
        raise ValueError(kind)

    successor_universe = tuple(d for d in universe if d != domino)
    successor_possible = tuple(p - {domino} for p in conditioned)
    successor_capacities = list(capacities)
    successor_capacities[actor] -= 1
    return hall_reduce(
        successor_universe,
        tuple(successor_capacities),
        successor_possible,
    )


def holder_map(normal: tuple) -> dict[int, frozenset[int]]:
    if normal == ("EMPTY",):
        return {}
    universe, _capacities, possible = normal_to_graph(normal)
    return {
        domino: frozenset(seat for seat in range(3) if domino in possible[seat])
        for domino in universe
    }


def ambiguity_rank(normal: tuple) -> int:
    if normal == ("EMPTY",):
        return -1
    tag = normal[2][0]
    return {"D": 0, "B": 1, "T": 2}[tag]


def unique_small_supports(max_universe: int = 4) -> dict[tuple, frozenset]:
    normals: dict[tuple, frozenset] = {}
    for universe_size in range(max_universe + 1):
        universe = tuple(range(universe_size))
        subsets = tuple(
            frozenset(x for x in universe if (mask >> x) & 1)
            for mask in range(1 << universe_size)
        )
        for possible in product(subsets, repeat=3):
            for k0 in range(universe_size + 1):
                for k1 in range(universe_size - k0 + 1):
                    capacities = (k0, k1, universe_size - k0 - k1)
                    worlds = base.enumerate_abstract_worlds(
                        universe, possible, capacities
                    )
                    if not worlds:
                        continue
                    normal = support.generic_support_normal_form(
                        universe, capacities, worlds
                    )
                    prior = normals.setdefault(normal, worlds)
                    assert prior == worlds
    return normals


def exact_world_transition(
    worlds: frozenset,
    actor: int,
    domino: int,
    kind: str,
    follower_set: frozenset[int],
) -> frozenset:
    successors = set()
    for world in worlds:
        if domino not in world[actor]:
            continue
        if kind == "follow" and domino not in follower_set:
            continue
        if kind == "slough":
            if domino in follower_set or not set(world[actor]).isdisjoint(follower_set):
                continue
        successor = [list(hand) for hand in world]
        successor[actor].remove(domino)
        successors.add(tuple(tuple(sorted(hand)) for hand in successor))
    return frozenset(successors)


def check_support_normal_dynamics() -> dict[str, int]:
    normals = unique_small_supports(4)
    observations = 0
    nonempty_successors = 0
    monotone_edge_checks = 0
    branch_checks = 0

    for normal, worlds in normals.items():
        universe, capacities, possible = normal_to_graph(normal)
        pre_holders = holder_map(normal)
        subsets = tuple(
            frozenset(x for x in universe if (mask >> x) & 1)
            for mask in range(1 << len(universe))
        )
        for actor in range(3):
            for domino in universe:
                if domino not in possible[actor]:
                    continue

                typed: list[tuple[str, frozenset[int]]] = [("lead", frozenset())]
                typed.extend(
                    ("follow" if domino in follower_set else "slough", follower_set)
                    for follower_set in subsets
                )
                for kind, follower_set in typed:
                    observations += 1
                    successor_worlds = exact_world_transition(
                        worlds, actor, domino, kind, follower_set
                    )
                    successor_universe = tuple(d for d in universe if d != domino)
                    successor_capacities = list(capacities)
                    successor_capacities[actor] -= 1
                    expected = support.generic_support_normal_form(
                        successor_universe,
                        tuple(successor_capacities),
                        successor_worlds,
                    )
                    actual = direct_support_transition(
                        normal, actor, domino, kind, follower_set
                    )
                    assert actual == expected
                    if not successor_worlds:
                        continue
                    nonempty_successors += 1
                    post_holders = holder_map(actual)
                    for survivor, post in post_holders.items():
                        assert post <= pre_holders[survivor]
                        for seat in range(3):
                            assert (seat in post) <= (seat in pre_holders[survivor])
                            monotone_edge_checks += 1
                    assert ambiguity_rank(actual) <= ambiguity_rank(normal)
                    branch_checks += 1

    return {
        "support_states": len(normals),
        "typed_observations": observations,
        "nonempty_successors": nonempty_successors,
        "monotone_edge_checks": monotone_edge_checks,
        "branch_checks": branch_checks,
    }


def compile_marginal_from_witness(
    universe: tuple[int, ...],
    possible: tuple[frozenset[int], ...],
    capacities: tuple[int, int, int],
    witness: tuple[tuple[int, ...], ...],
) -> tuple:
    if not universe:
        return normal_from_marginal(
            (), capacities, (frozenset(), frozenset(), frozenset())
        )
    components = support.generic_scc_components(universe, possible, witness)
    holder = {
        domino: seat for seat, hand in enumerate(witness) for domino in hand
    }
    marginal: list[set[int]] = [set(hand) for hand in witness]
    for seat in range(3):
        for domino in possible[seat]:
            if holder[domino] == seat:
                continue
            if components[("d", domino)] == components[("s", seat)]:
                marginal[seat].add(domino)
    return normal_from_marginal(
        universe, capacities, tuple(frozenset(x) for x in marginal)
    )


def expected_normal_from_public_residue(
    remaining: tuple[tuple[int, ...], ...],
    viewer: int,
    hidden_seats: tuple[int, int, int],
    voids: tuple[frozenset[int], ...],
    declaration: int,
) -> tuple:
    witness = tuple(remaining[seat] for seat in hidden_seats)
    universe = tuple(sorted(d for hand in witness for d in hand))
    capacities = tuple(len(hand) for hand in witness)
    possible = []
    for seat in hidden_seats:
        possible.append(
            frozenset(
                d
                for d in universe
                if all(not base.follows(d, q, declaration) for q in voids[seat])
            )
        )
    return compile_marginal_from_witness(
        universe, tuple(possible), capacities, witness
    )


def check_symbolic_complete_hands() -> dict[str, int]:
    hands_checked = 0
    transitions_checked = 0
    edge_deletions = 0

    for declaration in DECLARATIONS:
        for case in range(12):
            initial_hands, initial_leader, complete_plays = (
                base.generate_deterministic_contracted_hand(declaration, case)
            )
            viewer = (case + 2 * declaration) % 4
            hidden_seats = tuple(s for s in range(4) if s != viewer)
            hidden_index = {seat: index for index, seat in enumerate(hidden_seats)}
            hands = [list(hand) for hand in initial_hands]
            viewer_hand = set(hands[viewer])
            voids = [set() for _ in range(4)]
            leader = initial_leader
            trick: tuple[tuple[int, int], ...] = ()

            universe = tuple(sorted(d for seat in hidden_seats for d in hands[seat]))
            initial_possible = tuple(frozenset(universe) for _ in range(3))
            normal = hall_reduce(universe, (7, 7, 7), initial_possible)
            assert sum(len(v) for v in holder_map(normal).values()) == 63

            for expected_actor, domino in complete_plays:
                remaining_before = base.normalized_hands(hands)
                expected_before = expected_normal_from_public_residue(
                    remaining_before,
                    viewer,
                    hidden_seats,
                    tuple(frozenset(v) for v in voids),
                    declaration,
                )
                assert normal == expected_before
                pre_holders = holder_map(normal)

                actor = (leader + len(trick)) % 4
                assert actor == expected_actor
                legal = base.legal_plays(tuple(hands[actor]), trick, declaration)
                assert domino in legal

                if actor == viewer:
                    assert domino in viewer_hand
                    viewer_hand.remove(domino)
                    next_normal = normal
                else:
                    index = hidden_index[actor]
                    if not trick:
                        kind = "lead"
                        follower_set = frozenset()
                    else:
                        context = base.led_suit(trick[0][1], declaration)
                        follower_set = frozenset(
                            d for d in pre_holders if base.follows(d, context, declaration)
                        )
                        if base.follows(domino, context, declaration):
                            kind = "follow"
                        else:
                            kind = "slough"
                            voids[actor].add(context)
                    next_normal = direct_support_transition(
                        normal, index, domino, kind, follower_set
                    )
                    assert next_normal != ("EMPTY",)

                hands[actor].remove(domino)
                trick += ((actor, domino),)
                if len(trick) == 4:
                    leader, _points = base.resolve_trick(trick, declaration)
                    trick = ()

                remaining_after = base.normalized_hands(hands)
                expected_after = expected_normal_from_public_residue(
                    remaining_after,
                    viewer,
                    hidden_seats,
                    tuple(frozenset(v) for v in voids),
                    declaration,
                )
                assert next_normal == expected_after
                post_holders = holder_map(next_normal)
                for survivor, post in post_holders.items():
                    assert post <= pre_holders[survivor]
                deletion = sum(len(v) for v in pre_holders.values()) - sum(
                    len(v) for v in post_holders.values()
                )
                assert deletion >= 0
                edge_deletions += deletion
                normal = next_normal
                transitions_checked += 1

            assert not viewer_hand
            assert not holder_map(normal)
            hands_checked += 1

    assert hands_checked == 108
    assert transitions_checked == 3_024
    assert edge_deletions == 108 * 63 == 6_804
    return {
        "hands": hands_checked,
        "transitions": transitions_checked,
        "holder_edge_deletions": edge_deletions,
    }


def check_oriented_dihedral_frames() -> int:
    checked = 0
    for reflected in (False, True):
        for offset in range(4):
            orientation = -1 if reflected else 1

            def frame(seat: int) -> int:
                return (offset - seat) % 4 if reflected else (seat + offset) % 4

            for seat in range(4):
                assert frame((seat + 1) % 4) == (
                    frame(seat) + orientation
                ) % 4
                assert frame((seat + 2) % 4) == (frame(seat) + 2) % 4
            checked += 1
    assert checked == 8
    return checked


def partition_refinement(
    transitions: tuple[tuple[int, int], ...], outputs: tuple[int, ...]
) -> tuple[int, ...]:
    classes = tuple(outputs)
    while True:
        signatures = [
            (outputs[state], classes[transitions[state][0]], classes[transitions[state][1]])
            for state in range(len(outputs))
        ]
        mapping: dict[tuple[int, int, int], int] = {}
        refined = tuple(mapping.setdefault(sig, len(mapping)) for sig in signatures)
        if refined == classes:
            return refined
        classes = refined


def pair_future_equivalent(
    transitions: tuple[tuple[int, int], ...],
    outputs: tuple[int, ...],
    left: int,
    right: int,
) -> bool:
    queue = deque([(left, right)])
    seen = set()
    while queue:
        pair = queue.popleft()
        if pair in seen:
            continue
        seen.add(pair)
        a, b = pair
        if outputs[a] != outputs[b]:
            return False
        for action in range(2):
            queue.append((transitions[a][action], transitions[b][action]))
    return True


def check_future_equivalence_minimum() -> dict[str, int]:
    machines = 0
    pair_checks = 0
    for states in range(1, 4):
        transition_choices = tuple(product(range(states), repeat=2 * states))
        for flat in transition_choices:
            transitions = tuple(
                (flat[2 * state], flat[2 * state + 1]) for state in range(states)
            )
            for outputs in product((0, 1), repeat=states):
                classes = partition_refinement(transitions, outputs)
                for left in range(states):
                    for right in range(left + 1, states):
                        equivalent = pair_future_equivalent(
                            transitions, outputs, left, right
                        )
                        assert equivalent == (classes[left] == classes[right])
                        pair_checks += 1
                machines += 1
    assert machines == 5_898
    assert pair_checks == 17_560
    return {"machines": machines, "state_pairs": pair_checks}


def main() -> None:
    algebra = check_looped_graph_count_and_unscored_transports()
    fold = check_folded_trick()
    capacity_shapes = check_actor_capacity_shapes()
    score_prefixes = check_score_recovery()
    dynamics = check_support_normal_dynamics()
    corpus = check_symbolic_complete_hands()
    frames = check_oriented_dihedral_frames()
    future = check_future_equivalence_minimum()

    print("Texas 42 reduced-kernel verification: PASS")
    print("domino universe: looped K7 edges=28; count decoration=sum 5 or 10")
    print(
        "unscored pip-trump transports: "
        f"{algebra['ordered_pip_transports']} ordered; "
        f"{algebra['order_comparisons']:,} contextual order comparisons; "
        f"mechanics classes={algebra['mechanics_classes']}"
    )
    print(
        "folded trick: "
        f"{fold['trick_cases']:,} trick cases; "
        f"{fold['fold_updates']:,} sequential updates; "
        f"max competitive chain={fold['max_competitive_chain']}; "
        f"pending count values={fold['pending_count_values']}"
    )
    print(f"legal open-trick actor/capacity shapes: {capacity_shapes}")
    print(f"score-recovery prefixes: {score_prefixes:,}")
    print(
        "exact support-normal dynamics: "
        f"{dynamics['support_states']:,} distinct feasible supports; "
        f"{dynamics['typed_observations']:,} typed observations; "
        f"{dynamics['nonempty_successors']:,} nonempty successors; "
        f"{dynamics['monotone_edge_checks']:,} holder-edge checks"
    )
    print(
        "symbolic legal-hand corpus: "
        f"{corpus['hands']} hands; {corpus['transitions']:,} transitions; "
        f"{corpus['holder_edge_deletions']:,} total holder-edge deletions"
    )
    print(f"oriented dihedral frames checked: {frames}")
    print(
        "future-equivalence machines: "
        f"{future['machines']:,}; state-pair comparisons={future['state_pairs']:,}"
    )


if __name__ == "__main__":
    main()
