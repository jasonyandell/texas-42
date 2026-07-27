#!/usr/bin/env python3
"""Independent finite checks for the Texas 42 Foundations mathematics.

This file is a proof receipt for finite claims.  It is intentionally direct,
dependency-free, and separate from the reference implementation requested in
``docs/50_CODEX_IMPLEMENTATION_PROMPT.md``.
"""

from __future__ import annotations

from fractions import Fraction
from functools import lru_cache
from itertools import combinations, permutations, product
from math import factorial
from typing import Iterable, Sequence

PIPS = tuple(range(7))
PIP_DECLARATIONS = tuple(range(7))
DOUBLES_TRUMP = 7
NO_TRUMP = 8
DECLARATIONS = PIP_DECLARATIONS + (DOUBLES_TRUMP, NO_TRUMP)
CALLED = 7

# Canonical physical identity order: 0-0, 1-0, 1-1, ..., 6-6.
DOMINOES = tuple((high, low) for high in PIPS for low in range(high + 1))
ID_OF = {domino: index for index, domino in enumerate(DOMINOES)}
N_DOMINOES = len(DOMINOES)


def domino_id(name: str) -> int:
    a, b = map(int, name.split("-"))
    return ID_OF[(max(a, b), min(a, b))]


def domino_name(domino: int) -> str:
    high, low = DOMINOES[domino]
    return f"{high}-{low}"


def contains(domino: int, pip: int) -> bool:
    high, low = DOMINOES[domino]
    return high == pip or low == pip


def is_double(domino: int) -> bool:
    high, low = DOMINOES[domino]
    return high == low


def count_points(domino: int) -> int:
    high, low = DOMINOES[domino]
    if (high, low) in ((5, 5), (6, 4)):
        return 10
    if (high, low) in ((5, 0), (4, 1), (3, 2)):
        return 5
    return 0


def is_called(domino: int, declaration: int) -> bool:
    if declaration in PIP_DECLARATIONS:
        return contains(domino, declaration)
    if declaration == DOUBLES_TRUMP:
        return is_double(domino)
    if declaration == NO_TRUMP:
        return False
    raise ValueError(f"unknown declaration: {declaration}")


def is_powered(domino: int, declaration: int) -> bool:
    return declaration != NO_TRUMP and is_called(domino, declaration)


def effective_suits(domino: int, declaration: int) -> frozenset[int]:
    if is_called(domino, declaration):
        return frozenset((CALLED,))
    high, low = DOMINOES[domino]
    return frozenset((high,)) if high == low else frozenset((high, low))


def led_suit(domino: int, declaration: int) -> int:
    return CALLED if is_called(domino, declaration) else DOMINOES[domino][0]


def follows(domino: int, suit: int, declaration: int) -> bool:
    return suit in effective_suits(domino, declaration)


def declaration_rank(domino: int, declaration: int) -> int:
    """Return a total integer encoding of the declaration-relative rank.

    14 is used as the formal top element; all pip sums are at most 12.
    Tier-zero ranks are ignored by ``trick_key``.
    """

    high, low = DOMINOES[domino]
    if declaration == DOUBLES_TRUMP and high == low:
        return high
    if high == low:
        return 14
    return high + low


def trick_key(domino: int, suit: int, declaration: int) -> tuple[int, int]:
    if is_powered(domino, declaration):
        return (2, declaration_rank(domino, declaration))
    if follows(domino, suit, declaration):
        return (1, declaration_rank(domino, declaration))
    return (0, 0)


def legal_plays(
    hand: tuple[int, ...],
    trick: tuple[tuple[int, int], ...],
    declaration: int,
) -> tuple[int, ...]:
    if not trick:
        return hand
    suit = led_suit(trick[0][1], declaration)
    followers = tuple(d for d in hand if follows(d, suit, declaration))
    return followers or hand


def resolve_trick(
    trick: tuple[tuple[int, int], ...], declaration: int
) -> tuple[int, int]:
    assert len(trick) == 4
    suit = led_suit(trick[0][1], declaration)
    keys = tuple(trick_key(domino, suit, declaration) for _, domino in trick)
    assert keys.count(max(keys)) == 1
    winner_index = max(range(4), key=keys.__getitem__)
    winner = trick[winner_index][0]
    points = 1 + sum(count_points(domino) for _, domino in trick)
    return winner, points


def prose_resolve_winner(
    dominoes: tuple[int, int, int, int], declaration: int
) -> int:
    """Resolve a trick directly from the prose rules.

    This deliberately does not call ``is_called``, ``is_powered``,
    ``effective_suits``, ``led_suit``, ``follows``, ``declaration_rank``,
    ``trick_key``, ``resolve_trick``, or any comparison table.
    """

    def called(domino: int) -> bool:
        high, low = DOMINOES[domino]
        if declaration in PIP_DECLARATIONS:
            return high == declaration or low == declaration
        if declaration == DOUBLES_TRUMP:
            return high == low
        if declaration == NO_TRUMP:
            return False
        raise ValueError(f"unknown declaration: {declaration}")

    def powered(domino: int) -> bool:
        return declaration != NO_TRUMP and called(domino)

    lead = dominoes[0]
    lead_high, _ = DOMINOES[lead]
    lead_context = CALLED if called(lead) else lead_high

    trumps = tuple(domino for domino in dominoes if powered(domino))
    if trumps:
        if declaration == DOUBLES_TRUMP:
            order_key = lambda domino: DOMINOES[domino][0]
        else:
            trump_pip = declaration

            def order_key(domino: int) -> tuple[int, int]:
                high, low = DOMINOES[domino]
                if high == low:
                    return (1, 0)  # the trump double is highest
                other = low if high == trump_pip else high
                return (0, other)

        winner = max(trumps, key=order_key)
        assert sum(order_key(d) == order_key(winner) for d in trumps) == 1
        return winner

    def follows_prose(domino: int) -> bool:
        high, low = DOMINOES[domino]
        if lead_context == CALLED:
            return called(domino)
        contains_led = high == lead_context or low == lead_context
        return contains_led and not called(domino)

    followers = tuple(domino for domino in dominoes if follows_prose(domino))
    assert lead in followers

    def natural_order_key(domino: int) -> tuple[int, int]:
        high, low = DOMINOES[domino]
        if high == low:
            return (1, 0)  # the natural double is highest
        other = low if high == lead_context else high
        return (0, other)

    winner = max(followers, key=natural_order_key)
    assert sum(
        natural_order_key(d) == natural_order_key(winner) for d in followers
    ) == 1
    return winner


def auction_legal_bids(
    history: tuple[tuple[str, int] | None, ...], cap: int
) -> tuple[tuple[str, int] | None, ...]:
    """Legal bids for the exact one-round Straight auction grammar."""

    assert cap >= 1
    high = next((bid for bid in reversed(history) if bid is not None), None)
    legal: list[tuple[str, int] | None] = [None]

    if high is None:
        legal.extend(("P", value) for value in range(30, 42))
        legal.extend(("M", value) for value in range(1, min(cap, 2) + 1))
    elif high[0] == "P":
        legal.extend(("P", value) for value in range(high[1] + 1, 42))
        legal.extend(("M", value) for value in range(1, min(cap, 2) + 1))
    else:
        next_mark = high[1] + 1
        if next_mark <= cap:
            legal.append(("M", next_mark))

    return tuple(legal)


def enumerate_auction_histories(
    cap: int,
) -> tuple[tuple[tuple[str, int] | None, ...], ...]:
    histories: list[tuple[tuple[str, int] | None, ...]] = []

    def recurse(prefix: tuple[tuple[str, int] | None, ...]) -> None:
        if len(prefix) == 4:
            histories.append(prefix)
            return
        for bid in auction_legal_bids(prefix, cap):
            recurse(prefix + (bid,))

    recurse(())
    return tuple(histories)


def check_auction_exhaustive() -> tuple[tuple[int, ...], tuple[int, ...]]:
    expected_counts = (2380, 3060, 3196, 3213, 3214, 3214, 3214)
    expected_maxima = (1, 2, 3, 4, 5, 5, 5)
    all_histories = tuple(enumerate_auction_histories(cap) for cap in range(1, 8))
    counts = tuple(len(histories) for histories in all_histories)

    maxima: list[int] = []
    for histories in all_histories:
        reached = [
            bid[1]
            for history in histories
            for bid in history
            if bid is not None and bid[0] == "M"
        ]
        maxima.append(max(reached))

    assert counts == expected_counts
    assert tuple(maxima) == expected_maxima
    assert set(all_histories[4]) == set(all_histories[5]) == set(all_histories[6])
    return counts, tuple(maxima)


def check_basic_algebra() -> None:
    assert N_DOMINOES == 28
    assert len(set(DOMINOES)) == 28

    for pip in PIPS:
        natural = {d for d in range(N_DOMINOES) if contains(d, pip)}
        assert len(natural) == 7

    for domino in range(N_DOMINOES):
        natural_memberships = sum(contains(domino, pip) for pip in PIPS)
        assert natural_memberships == (1 if is_double(domino) else 2)

        for declaration in DECLARATIONS:
            suits = effective_suits(domino, declaration)
            assert 1 <= len(suits) <= 2
            assert all(
                follows(domino, suit, declaration) == (suit in suits)
                for suit in range(8)
            )
            assert (led_suit(domino, declaration) == CALLED) == is_called(
                domino, declaration
            )

    assert sum(count_points(d) for d in range(N_DOMINOES)) == 35
    assert 35 + 7 == 42


def check_unique_winner_exhaustive() -> tuple[int, int]:
    checked = 0
    prose_agreements = 0
    universe = set(range(N_DOMINOES))
    for declaration in DECLARATIONS:
        for lead in range(N_DOMINOES):
            suit = led_suit(lead, declaration)
            rest = sorted(universe - {lead})
            for followers in combinations(rest, 3):
                dominoes = (lead,) + followers
                keys = [trick_key(domino, suit, declaration) for domino in dominoes]
                assert keys.count(max(keys)) == 1
                algebraic_winner = max(range(4), key=lambda i: keys[i])
                prose_winner_domino = prose_resolve_winner(dominoes, declaration)
                assert dominoes[algebraic_winner] == prose_winner_domino
                checked += 1
                prose_agreements += 1
    assert checked == prose_agreements == 737_100
    return checked, prose_agreements

def transported_dominoes(permutation: tuple[int, ...]) -> tuple[int, ...]:
    return tuple(
        ID_OF[tuple(sorted((permutation[high], permutation[low]), reverse=True))]
        for high, low in DOMINOES
    )


def declaration_order_isomorphism(
    permutation: tuple[int, ...], declaration: int
) -> bool:
    transported = transported_dominoes(permutation)
    image_declaration = (
        permutation[declaration]
        if declaration in PIP_DECLARATIONS
        else declaration
    )
    suit_transport = permutation + (CALLED,)

    if tuple(count_points(transported[d]) for d in range(N_DOMINOES)) != tuple(
        count_points(d) for d in range(N_DOMINOES)
    ):
        return False

    for domino in range(N_DOMINOES):
        if suit_transport[led_suit(domino, declaration)] != led_suit(
            transported[domino], image_declaration
        ):
            return False

    for suit in range(8):
        image_suit = suit_transport[suit]
        for domino in range(N_DOMINOES):
            if follows(domino, suit, declaration) != follows(
                transported[domino], image_suit, image_declaration
            ):
                return False

        for left in range(N_DOMINOES):
            for right in range(N_DOMINOES):
                left_cmp = (
                    trick_key(left, suit, declaration)
                    > trick_key(right, suit, declaration)
                ) - (
                    trick_key(left, suit, declaration)
                    < trick_key(right, suit, declaration)
                )
                image_cmp = (
                    trick_key(transported[left], image_suit, image_declaration)
                    > trick_key(transported[right], image_suit, image_declaration)
                ) - (
                    trick_key(transported[left], image_suit, image_declaration)
                    < trick_key(transported[right], image_suit, image_declaration)
                )
                if left_cmp != image_cmp:
                    return False
    return True


def check_pip_transport_exhaustive() -> tuple[tuple[int, ...], ...]:
    count_vector = tuple(count_points(d) for d in range(N_DOMINOES))
    survivors: list[tuple[int, ...]] = []

    for permutation in permutations(PIPS):
        transported = transported_dominoes(permutation)
        if tuple(count_points(transported[d]) for d in range(N_DOMINOES)) == count_vector:
            survivors.append(permutation)

    identity = tuple(PIPS)
    swap_2_3 = (0, 1, 3, 2, 4, 5, 6)
    assert set(survivors) == {identity, swap_2_3}
    assert all(declaration_order_isomorphism(identity, d) for d in DECLARATIONS)
    assert {
        d for d in DECLARATIONS if declaration_order_isomorphism(swap_2_3, d)
    } == {2, 3}
    return tuple(survivors)


def direct_assignment_count(
    universe: tuple[int, ...],
    possible: tuple[frozenset[int], ...],
    capacities: tuple[int, ...],
) -> int:
    """Count allowed labeled holder assignments by direct recursion."""

    counts = [0] * len(possible)

    def recurse(index: int) -> int:
        if index == len(universe):
            return int(tuple(counts) == capacities)
        domino = universe[index]
        total = 0
        for seat in range(len(possible)):
            if domino in possible[seat] and counts[seat] < capacities[seat]:
                counts[seat] += 1
                total += recurse(index + 1)
                counts[seat] -= 1
        return total

    return recurse(0)


def direct_assignment_exists(
    universe: tuple[int, ...],
    possible: tuple[frozenset[int], ...],
    capacities: tuple[int, ...],
) -> bool:
    return direct_assignment_count(universe, possible, capacities) > 0


def generating_function_count(
    universe: tuple[int, ...],
    possible: tuple[frozenset[int], ...],
    capacities: tuple[int, ...],
) -> int:
    """Extract [prod x_s^k_s] prod_d sum_{s:d in P_s} x_s by DP."""

    zero = (0,) * len(possible)
    coefficients: dict[tuple[int, ...], int] = {zero: 1}
    for domino in universe:
        successor: dict[tuple[int, ...], int] = {}
        for exponents, coefficient in coefficients.items():
            for seat in range(len(possible)):
                if domino not in possible[seat]:
                    continue
                if exponents[seat] >= capacities[seat]:
                    continue
                updated = list(exponents)
                updated[seat] += 1
                key = tuple(updated)
                successor[key] = successor.get(key, 0) + coefficient
        coefficients = successor
    return coefficients.get(capacities, 0)


def recurrence_assignment_count(
    universe: tuple[int, ...],
    possible: tuple[frozenset[int], ...],
    capacities: tuple[int, ...],
) -> int:
    """Evaluate the exact choose-one-tile fiber recurrence."""

    if any(capacity < 0 for capacity in capacities):
        return 0
    if not universe:
        return int(all(capacity == 0 for capacity in capacities))
    if sum(capacities) != len(universe):
        return 0

    domino = universe[0]
    remaining = universe[1:]
    total = 0
    for seat in range(len(possible)):
        if domino not in possible[seat] or capacities[seat] == 0:
            continue
        updated_capacities = list(capacities)
        updated_capacities[seat] -= 1
        updated_possible = tuple(
            frozenset(value for value in seat_possible if value != domino)
            for seat_possible in possible
        )
        total += recurrence_assignment_count(
            remaining,
            updated_possible,
            tuple(updated_capacities),
        )
    return total


def uniform_sequential_world_probability(
    universe: tuple[int, ...],
    possible: tuple[frozenset[int], ...],
    capacities: tuple[int, ...],
    world: tuple[tuple[int, ...], ...],
) -> Fraction:
    """Probability assigned to one world by the exact count-ratio sampler.

    The sampler takes the first remaining tile, chooses its holder with
    probability N(C^{d->s}) / N(C), and recurses. This evaluator follows the
    unique holder sequence belonging to ``world`` and multiplies those exact
    rational conditionals.
    """

    probability = Fraction(1)
    current_universe = universe
    current_possible = possible
    current_capacities = capacities
    world_sets = tuple(frozenset(hand) for hand in world)

    while current_universe:
        current_count = recurrence_assignment_count(
            current_universe,
            current_possible,
            current_capacities,
        )
        assert current_count > 0

        domino = current_universe[0]
        holder = next(
            seat for seat, hand in enumerate(world_sets) if domino in hand
        )
        assert domino in current_possible[holder]
        assert current_capacities[holder] > 0

        successor_universe = current_universe[1:]
        successor_possible = tuple(
            frozenset(value for value in seat_possible if value != domino)
            for seat_possible in current_possible
        )
        successor_capacities_list = list(current_capacities)
        successor_capacities_list[holder] -= 1
        successor_capacities = tuple(successor_capacities_list)
        successor_count = recurrence_assignment_count(
            successor_universe,
            successor_possible,
            successor_capacities,
        )
        assert successor_count > 0

        probability *= Fraction(successor_count, current_count)
        current_universe = successor_universe
        current_possible = successor_possible
        current_capacities = successor_capacities

    assert all(capacity == 0 for capacity in current_capacities)
    return probability


def hall_feasible(
    universe: tuple[int, ...],
    possible: tuple[frozenset[int], ...],
    capacities: tuple[int, ...],
) -> bool:
    if sum(capacities) != len(universe):
        return False
    if set().union(*possible) != set(universe):
        return False

    seats = range(len(possible))
    for size in range(1, len(possible) + 1):
        for subset in combinations(seats, size):
            union = set().union(*(possible[s] for s in subset))
            if len(union) < sum(capacities[s] for s in subset):
                return False
    return True


def forced_successor_system(
    universe: tuple[int, ...],
    possible: tuple[frozenset[int], ...],
    capacities: tuple[int, ...],
    domino: int,
    seat: int,
) -> tuple[
    tuple[int, ...],
    tuple[frozenset[int], ...],
    tuple[int, ...],
] | None:
    """Return C^{d->s}, or None when the holder edge is locally disallowed."""

    if domino not in possible[seat] or capacities[seat] <= 0:
        return None
    successor_universe = tuple(value for value in universe if value != domino)
    successor_possible = tuple(
        frozenset(value for value in seat_possible if value != domino)
        for seat_possible in possible
    )
    successor_capacities_list = list(capacities)
    successor_capacities_list[seat] -= 1
    return (
        successor_universe,
        successor_possible,
        tuple(successor_capacities_list),
    )


def check_hall_small_exhaustive() -> tuple[int, int, int, int, int]:
    checked = 0
    count_agreements = 0
    uniform_world_checks = 0
    marginal_edge_checks = 0
    reduction_checks = 0

    # These maps exhaustively check the canonical-normal-form biconditional on
    # the stated tiny domain, always within a fixed pool/capacity schema.
    fiber_to_reduction: dict[
        tuple[
            tuple[int, ...],
            tuple[int, ...],
            frozenset[tuple[tuple[int, ...], ...]],
        ],
        tuple[frozenset[int], ...],
    ] = {}
    reduction_to_fiber: dict[
        tuple[
            tuple[int, ...],
            tuple[int, ...],
            tuple[frozenset[int], ...],
        ],
        frozenset[tuple[tuple[int, ...], ...]],
    ] = {}

    for universe_size in range(1, 5):
        universe = tuple(range(universe_size))
        subsets = tuple(
            frozenset(x for x in universe if (mask >> x) & 1)
            for mask in range(1 << universe_size)
        )
        for possible in product(subsets, repeat=3):
            for k0 in range(universe_size + 1):
                for k1 in range(universe_size - k0 + 1):
                    k2 = universe_size - k0 - k1
                    capacities = (k0, k1, k2)
                    direct_count = direct_assignment_count(
                        universe, possible, capacities
                    )
                    polynomial_count = generating_function_count(
                        universe, possible, capacities
                    )
                    recurrence_count = recurrence_assignment_count(
                        universe, possible, capacities
                    )
                    assert (direct_count > 0) == hall_feasible(
                        universe, possible, capacities
                    )
                    assert direct_count == polynomial_count == recurrence_count

                    worlds = enumerate_abstract_worlds(
                        universe, possible, capacities
                    )
                    assert len(worlds) == direct_count

                    projected_possible = tuple(
                        frozenset(
                            domino
                            for world in worlds
                            for domino in world[seat]
                        )
                        for seat in range(3)
                    )
                    assert all(
                        projected_possible[seat].issubset(possible[seat])
                        for seat in range(3)
                    )

                    for seat in range(3):
                        for domino in universe:
                            successor = forced_successor_system(
                                universe,
                                possible,
                                capacities,
                                domino,
                                seat,
                            )
                            edge_is_supported = (
                                successor is not None
                                and hall_feasible(*successor)
                            )
                            assert edge_is_supported == (
                                domino in projected_possible[seat]
                            )
                            marginal_edge_checks += 1

                    reduced_worlds = enumerate_abstract_worlds(
                        universe,
                        projected_possible,
                        capacities,
                    )
                    assert reduced_worlds == worlds

                    # Idempotence: projecting exact holder edges again changes
                    # nothing because the reduced system has the same worlds.
                    reprojected_possible = tuple(
                        frozenset(
                            domino
                            for world in reduced_worlds
                            for domino in world[seat]
                        )
                        for seat in range(3)
                    )
                    assert reprojected_possible == projected_possible

                    fiber_key = (universe, capacities, worlds)
                    prior_reduction = fiber_to_reduction.setdefault(
                        fiber_key, projected_possible
                    )
                    assert prior_reduction == projected_possible

                    reduction_key = (universe, capacities, projected_possible)
                    prior_fiber = reduction_to_fiber.setdefault(
                        reduction_key, worlds
                    )
                    assert prior_fiber == worlds
                    reduction_checks += 1

                    if direct_count > 0:
                        probabilities = tuple(
                            uniform_sequential_world_probability(
                                universe,
                                possible,
                                capacities,
                                world,
                            )
                            for world in worlds
                        )
                        assert all(
                            probability == Fraction(1, direct_count)
                            for probability in probabilities
                        )
                        assert sum(probabilities, Fraction()) == 1
                        uniform_world_checks += len(worlds)

                    checked += 1
                    count_agreements += 1
    assert checked == 66_968
    return (
        checked,
        count_agreements,
        uniform_world_checks,
        marginal_edge_checks,
        reduction_checks,
    )


def enumerate_abstract_worlds(
    universe: tuple[int, ...],
    possible: tuple[frozenset[int], ...],
    capacities: tuple[int, ...],
) -> frozenset[tuple[tuple[int, ...], ...]]:
    """Enumerate abstract labeled cell worlds as sorted per-seat hands."""

    hands: list[list[int]] = [[] for _ in possible]
    worlds: set[tuple[tuple[int, ...], ...]] = set()

    def recurse(index: int) -> None:
        if index == len(universe):
            if tuple(len(hand) for hand in hands) == capacities:
                worlds.add(tuple(tuple(sorted(hand)) for hand in hands))
            return
        domino = universe[index]
        for seat in range(len(possible)):
            if domino in possible[seat] and len(hands[seat]) < capacities[seat]:
                hands[seat].append(domino)
                recurse(index + 1)
                hands[seat].pop()

    recurse(0)
    return frozenset(worlds)


def check_typed_transition_small_exhaustive() -> tuple[int, int, int]:
    """Exhaust typed lead/follow/slough cell updates for universes of size <= 3."""

    lead_cases = 0
    follow_cases = 0
    slough_cases = 0

    for universe_size in range(1, 4):
        universe = tuple(range(universe_size))
        subsets = tuple(
            frozenset(x for x in universe if (mask >> x) & 1)
            for mask in range(1 << universe_size)
        )
        for possible in product(subsets, repeat=3):
            for k0 in range(universe_size + 1):
                for k1 in range(universe_size - k0 + 1):
                    k2 = universe_size - k0 - k1
                    capacities = (k0, k1, k2)
                    predecessor_worlds = enumerate_abstract_worlds(
                        universe, possible, capacities
                    )

                    for actor in range(3):
                        if capacities[actor] == 0:
                            continue
                        for domino in universe:
                            if domino not in possible[actor]:
                                continue

                            updated_universe = tuple(
                                value for value in universe if value != domino
                            )
                            updated_capacities_list = list(capacities)
                            updated_capacities_list[actor] -= 1
                            updated_capacities = tuple(updated_capacities_list)
                            base_possible = tuple(
                                frozenset(
                                    value
                                    for value in seat_possible
                                    if value != domino
                                )
                                for seat_possible in possible
                            )

                            def image_of(
                                legal_predecessors: Iterable[
                                    tuple[tuple[int, ...], ...]
                                ]
                            ) -> frozenset[tuple[tuple[int, ...], ...]]:
                                image = set()
                                for world in legal_predecessors:
                                    successor = [list(hand) for hand in world]
                                    successor[actor].remove(domino)
                                    image.add(
                                        tuple(
                                            tuple(sorted(hand)) for hand in successor
                                        )
                                    )
                                return frozenset(image)

                            actor_holds = frozenset(
                                world
                                for world in predecessor_worlds
                                if domino in world[actor]
                            )
                            lead_successors = enumerate_abstract_worlds(
                                updated_universe,
                                base_possible,
                                updated_capacities,
                            )
                            assert image_of(actor_holds) == lead_successors
                            lead_cases += 1

                            for follower_set in subsets:
                                if domino in follower_set:
                                    follow_successors = enumerate_abstract_worlds(
                                        updated_universe,
                                        base_possible,
                                        updated_capacities,
                                    )
                                    assert image_of(actor_holds) == follow_successors
                                    follow_cases += 1
                                else:
                                    legal_sloughs = frozenset(
                                        world
                                        for world in actor_holds
                                        if set(world[actor]).isdisjoint(follower_set)
                                    )
                                    slough_possible = list(base_possible)
                                    slough_possible[actor] = frozenset(
                                        value
                                        for value in slough_possible[actor]
                                        if value not in follower_set
                                    )
                                    slough_successors = enumerate_abstract_worlds(
                                        updated_universe,
                                        tuple(slough_possible),
                                        updated_capacities,
                                    )
                                    assert image_of(legal_sloughs) == slough_successors
                                    slough_cases += 1

    return lead_cases, follow_cases, slough_cases


def unrestricted_capacity_dp_stats(
    capacities: tuple[int, int, int],
) -> tuple[int, int, int, int, int, tuple[int, ...]]:
    """Instrument the all-edges-allowed occupancy DP for one capacity profile."""

    universe = tuple(range(sum(capacities)))
    coefficients: dict[tuple[int, int, int], int] = {(0, 0, 0): 1}
    layer_sizes: list[int] = []
    total_occupancy_states = 0
    candidate_holder_checks = 0
    capacity_eligible_updates = 0

    for _domino in universe:
        layer_sizes.append(len(coefficients))
        total_occupancy_states += len(coefficients)
        successor: dict[tuple[int, int, int], int] = {}
        for exponents, coefficient in coefficients.items():
            for seat in range(3):
                candidate_holder_checks += 1
                if exponents[seat] >= capacities[seat]:
                    continue
                capacity_eligible_updates += 1
                updated = list(exponents)
                updated[seat] += 1
                key = tuple(updated)
                assert len(key) == 3
                successor[key] = successor.get(key, 0) + coefficient
        coefficients = successor

    layer_sizes.append(len(coefficients))
    total_occupancy_states += len(coefficients)
    expected_count = factorial(sum(capacities))
    for capacity in capacities:
        expected_count //= factorial(capacity)
    assert coefficients == {capacities: expected_count}

    return (
        expected_count,
        total_occupancy_states,
        candidate_holder_checks,
        capacity_eligible_updates,
        max(layer_sizes),
        tuple(layer_sizes),
    )


def check_native_fiber_count_bound() -> tuple[int, int, int, int, int, int]:
    """Check every native unrestricted capacity profile and the sharp maxima."""

    profile_count = 0
    maximum_worlds = 0
    maximizing_capacity = None
    maximum_states = 0
    maximum_candidate_checks = 0
    maximum_eligible_updates = 0
    maximum_live_layer = 0

    for k0 in range(8):
        for k1 in range(8):
            for k2 in range(8):
                capacities = (k0, k1, k2)
                (
                    world_count,
                    total_states,
                    candidate_checks,
                    eligible_updates,
                    max_layer,
                    _layer_sizes,
                ) = unrestricted_capacity_dp_stats(capacities)

                product_bound = (k0 + 1) * (k1 + 1) * (k2 + 1)
                candidate_bound = 3 * (product_bound - 1)
                eligible_bound = (
                    k0 * (k1 + 1) * (k2 + 1)
                    + k1 * (k0 + 1) * (k2 + 1)
                    + k2 * (k0 + 1) * (k1 + 1)
                )

                # With every holder edge allowed, every bounded occupancy
                # vector is live, so the general bounds are attained exactly.
                assert total_states == product_bound
                assert candidate_checks == candidate_bound
                assert eligible_updates == eligible_bound
                assert max_layer <= 48

                profile_count += 1
                if world_count > maximum_worlds:
                    maximum_worlds = world_count
                    maximizing_capacity = capacities
                maximum_states = max(maximum_states, total_states)
                maximum_candidate_checks = max(
                    maximum_candidate_checks, candidate_checks
                )
                maximum_eligible_updates = max(
                    maximum_eligible_updates, eligible_updates
                )
                maximum_live_layer = max(maximum_live_layer, max_layer)

    assert profile_count == 8 ** 3 == 512
    assert maximum_worlds == 399_072_960
    assert maximizing_capacity == (7, 7, 7)
    assert maximum_states == 8 ** 3 == 512
    assert maximum_candidate_checks == 3 * ((8 ** 3) - 1) == 1_533
    assert maximum_eligible_updates == 3 * 7 * (8 ** 2) == 1_344
    assert maximum_live_layer == 48

    (
        initial_count,
        total_occupancy_states,
        candidate_holder_checks,
        capacity_eligible_updates,
        max_layer,
        layer_sizes,
    ) = unrestricted_capacity_dp_stats((7, 7, 7))
    assert initial_count == maximum_worlds
    expected_layer_sizes = (
        1, 3, 6, 10, 15, 21, 28, 36, 42, 46, 48,
        48, 46, 42, 36, 28, 21, 15, 10, 6, 3, 1,
    )
    assert layer_sizes == expected_layer_sizes
    assert total_occupancy_states == maximum_states
    assert candidate_holder_checks == maximum_candidate_checks
    assert capacity_eligible_updates == maximum_eligible_updates
    assert max_layer == maximum_live_layer

    return (
        profile_count,
        maximum_worlds,
        maximum_states,
        maximum_candidate_checks,
        maximum_eligible_updates,
        maximum_live_layer,
    )

def normalized_hands(
    hands: Sequence[Iterable[int]],
) -> tuple[tuple[int, ...], ...]:
    return tuple(tuple(sorted(hand)) for hand in hands)


def generate_deterministic_contracted_hand(
    declaration: int, case: int
) -> tuple[
    tuple[tuple[int, ...], ...],
    int,
    tuple[tuple[int, int], ...],
]:
    """Generate one deterministic legal complete hand for support parity checks."""

    multiplier = case + 1  # nonzero modulo the prime 29 for cases 0..11
    offset = (7 * case + 11 * declaration) % 29
    order = sorted(
        range(N_DOMINOES),
        key=lambda domino: (multiplier * domino + offset) % 29,
    )
    initial_hands = normalized_hands(
        order[seat * 7 : (seat + 1) * 7] for seat in range(4)
    )

    hands = [list(hand) for hand in initial_hands]
    initial_leader = (declaration + case) % 4
    leader = initial_leader
    trick: tuple[tuple[int, int], ...] = ()
    plays: list[tuple[int, int]] = []

    for ply in range(28):
        actor = (leader + len(trick)) % 4
        choices = tuple(sorted(legal_plays(tuple(hands[actor]), trick, declaration)))
        choice_index = (
            declaration * 17 + case * 11 + ply * 5 + actor
        ) % len(choices)
        domino = choices[choice_index]
        hands[actor].remove(domino)
        plays.append((actor, domino))
        trick = trick + ((actor, domino),)

        if len(trick) == 4:
            leader, _ = resolve_trick(trick, declaration)
            trick = ()

    assert all(not hand for hand in hands)
    assert not trick
    return initial_hands, initial_leader, tuple(plays)


def replay_public_prefix(
    initial_hands: tuple[tuple[int, ...], ...],
    initial_leader: int,
    public_plays: tuple[tuple[int, int], ...],
    declaration: int,
) -> tuple[
    tuple[tuple[int, ...], ...],
    tuple[frozenset[int], ...],
] | None:
    """Replay a public prefix, returning current hands and derived voids."""

    flat = tuple(domino for hand in initial_hands for domino in hand)
    if (
        any(len(hand) != 7 for hand in initial_hands)
        or len(flat) != N_DOMINOES
        or set(flat) != set(range(N_DOMINOES))
    ):
        return None

    hands = [list(hand) for hand in initial_hands]
    leader = initial_leader
    trick: tuple[tuple[int, int], ...] = ()
    voids = [set() for _ in range(4)]

    for expected_actor, domino in public_plays:
        actor = (leader + len(trick)) % 4
        if actor != expected_actor or domino not in hands[actor]:
            return None

        choices = legal_plays(tuple(hands[actor]), trick, declaration)
        if domino not in choices:
            return None

        if trick:
            suit = led_suit(trick[0][1], declaration)
            if not follows(domino, suit, declaration):
                voids[actor].add(suit)

        hands[actor].remove(domino)
        trick = trick + ((actor, domino),)
        if len(trick) == 4:
            leader, _ = resolve_trick(trick, declaration)
            trick = ()

    return normalized_hands(hands), tuple(frozenset(void) for void in voids)


def enumerate_capacity_partitions(
    universe: tuple[int, ...], capacities: tuple[int, int, int]
) -> Iterable[tuple[tuple[int, ...], tuple[int, ...], tuple[int, ...]]]:
    """Enumerate all labeled three-hand partitions with exact capacities."""

    k0, k1, k2 = capacities
    for hand0 in combinations(universe, k0):
        after0 = tuple(domino for domino in universe if domino not in hand0)
        for hand1 in combinations(after0, k1):
            hand2 = tuple(domino for domino in after0 if domino not in hand1)
            if len(hand2) == k2:
                yield tuple(sorted(hand0)), tuple(sorted(hand1)), tuple(sorted(hand2))


def check_reachable_cell_support_corpus() -> tuple[int, int, int, int, int]:
    """Check support parity and exact typed transitions on a named corpus.

    This is a finite regression corpus, not an exhaustive proof over all
    reachable histories. The general support and transition theorems are
    proved in the mathematical text.
    """

    prefixes_checked = 0
    prefixes_with_voids = 0
    transitions_checked = 0
    hidden_transitions = 0
    viewer_transitions = 0

    for declaration in DECLARATIONS:
        for case in range(12):
            initial_hands, initial_leader, complete_plays = (
                generate_deterministic_contracted_hand(declaration, case)
            )
            viewer = (case + 2 * declaration) % 4
            hidden_seats = tuple(seat for seat in range(4) if seat != viewer)
            prefix_data: dict[int, dict[str, object]] = {}

            for prefix_length in range(20, 29):
                prefix = complete_plays[:prefix_length]
                replay = replay_public_prefix(
                    initial_hands, initial_leader, prefix, declaration
                )
                assert replay is not None
                actual_remaining, voids = replay

                played_by_seat: list[list[int]] = [[] for _ in range(4)]
                for actor, domino in prefix:
                    played_by_seat[actor].append(domino)

                own_remaining = set(actual_remaining[viewer])
                unseen = tuple(
                    sorted(
                        set(range(N_DOMINOES))
                        - own_remaining
                        - {domino for _, domino in prefix}
                    )
                )
                capacities = tuple(
                    7 - len(played_by_seat[seat]) for seat in hidden_seats
                )
                assert len(capacities) == 3

                possible: list[frozenset[int]] = []
                for seat in hidden_seats:
                    forbidden = {
                        domino
                        for domino in unseen
                        for suit in voids[seat]
                        if follows(domino, suit, declaration)
                    }
                    possible.append(frozenset(set(unseen) - forbidden))

                cell_fiber: set[tuple[tuple[int, ...], ...]] = set()
                replay_support: set[tuple[tuple[int, ...], ...]] = set()

                for remainder in enumerate_capacity_partitions(
                    unseen,
                    (capacities[0], capacities[1], capacities[2]),
                ):
                    if all(
                        set(remainder[index]).issubset(possible[index])
                        for index in range(3)
                    ):
                        cell_fiber.add(remainder)

                    candidate_initial = list(initial_hands)
                    for index, seat in enumerate(hidden_seats):
                        candidate_initial[seat] = tuple(
                            sorted(played_by_seat[seat] + list(remainder[index]))
                        )

                    if replay_public_prefix(
                        tuple(candidate_initial),
                        initial_leader,
                        prefix,
                        declaration,
                    ) is not None:
                        replay_support.add(remainder)

                assert cell_fiber == replay_support
                actual_hidden = tuple(actual_remaining[seat] for seat in hidden_seats)
                assert actual_hidden in cell_fiber

                prefix_data[prefix_length] = {
                    "prefix": prefix,
                    "played_by_seat": tuple(tuple(x) for x in played_by_seat),
                    "fiber": cell_fiber,
                }
                prefixes_checked += 1
                prefixes_with_voids += int(any(voids))

            for prefix_length in range(20, 28):
                pre = prefix_data[prefix_length]
                post = prefix_data[prefix_length + 1]
                pre_fiber = pre["fiber"]
                post_fiber = post["fiber"]
                assert isinstance(pre_fiber, set) and isinstance(post_fiber, set)
                next_actor, next_domino = complete_plays[prefix_length]

                if next_actor == viewer:
                    assert pre_fiber == post_fiber
                    viewer_transitions += 1
                else:
                    actor_index = hidden_seats.index(next_actor)
                    image: set[tuple[tuple[int, ...], ...]] = set()
                    played_pre = pre["played_by_seat"]
                    prefix = pre["prefix"]
                    assert isinstance(played_pre, tuple) and isinstance(prefix, tuple)

                    for remainder in pre_fiber:
                        if next_domino not in remainder[actor_index]:
                            continue

                        candidate_initial = list(initial_hands)
                        for index, seat in enumerate(hidden_seats):
                            candidate_initial[seat] = tuple(
                                sorted(played_pre[seat] + remainder[index])
                            )

                        if replay_public_prefix(
                            tuple(candidate_initial),
                            initial_leader,
                            prefix + ((next_actor, next_domino),),
                            declaration,
                        ) is None:
                            continue

                        successor = list(remainder)
                        successor[actor_index] = tuple(
                            domino
                            for domino in remainder[actor_index]
                            if domino != next_domino
                        )
                        image.add(tuple(successor))

                    assert image == post_fiber
                    assert len(post_fiber) <= len(pre_fiber)
                    hidden_transitions += 1

                transitions_checked += 1

    assert prefixes_checked == 972
    assert prefixes_with_voids == 970
    assert transitions_checked == 864
    assert hidden_transitions == 648
    assert viewer_transitions == 216
    return (
        prefixes_checked,
        prefixes_with_voids,
        transitions_checked,
        hidden_transitions,
        viewer_transitions,
    )

PLAYED_BY_SEAT = (
    tuple(map(domino_id, ("1-0", "1-1", "2-0", "5-4", "6-1"))),
    tuple(map(domino_id, ("0-0", "3-0", "4-3", "6-4", "6-6"))),
    tuple(map(domino_id, ("2-2", "3-3", "4-2", "5-2", "6-0"))),
    tuple(map(domino_id, ("2-1", "4-0", "5-0", "5-1", "6-3"))),
)

PUBLIC_PLAYS = tuple(
    (seat, domino_id(tile))
    for seat, tile in (
        (3, "6-3"),
        (0, "6-1"),
        (1, "6-4"),
        (2, "6-0"),
        (1, "0-0"),
        (2, "2-2"),
        (3, "5-0"),
        (0, "2-0"),
        (1, "4-3"),
        (2, "4-2"),
        (3, "4-0"),
        (0, "5-4"),
        (0, "1-1"),
        (1, "3-0"),
        (2, "3-3"),
        (3, "2-1"),
        (0, "1-0"),
        (1, "6-6"),
        (2, "5-2"),
        (3, "5-1"),
    )
)

VIEWER_HAND = tuple(map(domino_id, ("4-1", "3-1")))
UNSEEN_POOL = tuple(
    map(domino_id, ("5-5", "4-4", "3-2", "6-5", "5-3", "6-2"))
)


def simulate_common_prefix(
    remaining_hands: tuple[tuple[int, ...], ...]
) -> tuple[
    tuple[tuple[int, ...], ...],
    int,
    tuple[int, int],
    tuple[frozenset[int], ...],
] | None:
    full_hands = normalized_hands(
        tuple(PLAYED_BY_SEAT[s] + remaining_hands[s] for s in range(4))
    )
    flat = [domino for hand in full_hands for domino in hand]
    assert len(flat) == 28 and set(flat) == set(range(28))

    hands = [list(hand) for hand in full_hands]
    leader = 3
    trick: tuple[tuple[int, int], ...] = ()
    scores = [0, 0]
    voids = [set() for _ in range(4)]

    for expected_actor, domino in PUBLIC_PLAYS:
        actor = (leader + len(trick)) % 4
        assert actor == expected_actor
        choices = legal_plays(tuple(hands[actor]), trick, NO_TRUMP)
        if domino not in choices:
            return None

        if trick:
            suit = led_suit(trick[0][1], NO_TRUMP)
            if not follows(domino, suit, NO_TRUMP):
                voids[actor].add(suit)

        hands[actor].remove(domino)
        trick = trick + ((actor, domino),)
        if len(trick) == 4:
            winner, points = resolve_trick(trick, NO_TRUMP)
            scores[winner % 2] += points
            leader = winner
            trick = ()

    assert not trick
    return (
        normalized_hands(hands),
        leader,
        (scores[0], scores[1]),
        tuple(frozenset(void) for void in voids),
    )


def single_seat_root_values(
    remaining_hands: tuple[tuple[int, ...], ...],
    leader: int = 3,
    declaration: int = NO_TRUMP,
    optimizing_seat: int = 3,
) -> dict[int, int]:
    """Exact root values against a lowest-legal-ID field.

    The optimizing seat has two tiles at the root and only one afterwards, so
    every later own action is forced.  Averaging these values over hidden
    worlds therefore does not introduce strategy fusion.
    """

    remaining_hands = normalized_hands(remaining_hands)

    @lru_cache(None)
    def recurse(
        hands: tuple[tuple[int, ...], ...],
        current_leader: int,
        trick: tuple[tuple[int, int], ...],
    ) -> int:
        if all(not hand for hand in hands):
            return 0

        actor = (current_leader + len(trick)) % 4
        choices = legal_plays(hands[actor], trick, declaration)
        values: dict[int, int] = {}

        for domino in choices:
            next_hands = [list(hand) for hand in hands]
            next_hands[actor].remove(domino)
            next_hands_tuple = normalized_hands(next_hands)
            next_trick = trick + ((actor, domino),)
            next_leader = current_leader
            reward = 0

            if len(next_trick) == 4:
                winner, points = resolve_trick(next_trick, declaration)
                reward = points if winner % 2 == optimizing_seat % 2 else -points
                next_leader = winner
                next_trick = ()

            values[domino] = reward + recurse(
                next_hands_tuple, next_leader, next_trick
            )

        if actor == optimizing_seat:
            # Only one tile remains after the root action in this witness.
            assert len(choices) == 1
            return values[choices[0]]
        return values[min(choices)]

    assert leader == optimizing_seat
    root_values: dict[int, int] = {}
    for domino in legal_plays(remaining_hands[optimizing_seat], (), declaration):
        next_hands = [list(hand) for hand in remaining_hands]
        next_hands[optimizing_seat].remove(domino)
        root_values[domino] = recurse(
            normalized_hands(next_hands), leader, ((optimizing_seat, domino),)
        )
    return root_values


def enumerate_history_fiber() -> tuple[tuple[tuple[int, ...], ...], ...]:
    worlds: list[tuple[tuple[int, ...], ...]] = []
    for hand0 in combinations(UNSEEN_POOL, 2):
        after0 = tuple(d for d in UNSEEN_POOL if d not in hand0)
        for hand1 in combinations(after0, 2):
            hand2 = tuple(d for d in after0 if d not in hand1)
            world = normalized_hands((hand0, hand1, hand2, VIEWER_HAND))
            endpoint = simulate_common_prefix(world)
            assert endpoint is not None
            worlds.append(world)

    assert len(worlds) == 90
    assert len(set(worlds)) == 90
    endpoints = {
        simulate_common_prefix(world)[1:]  # type: ignore[index]
        for world in worlds
    }
    assert len(endpoints) == 1
    endpoint = next(iter(endpoints))
    assert endpoint == (
        3,
        (2, 18),
        (frozenset(), frozenset({1}), frozenset({0, 1}), frozenset()),
    )
    return tuple(worlds)




def validate_point_auction(
    shaker: int, actions: tuple[tuple[int, int | None], ...]
) -> tuple[int | None, int | None]:
    """Validate the point-bid fragment used by the history witness.

    ``None`` is pass.  This intentionally checks only the finite Straight
    point-bid lane needed by the witness; the general auction is specified and
    proved separately in the foundation.
    """

    expected_seats = tuple((shaker + 1 + offset) % 4 for offset in range(4))
    assert tuple(seat for seat, _ in actions) == expected_seats
    high: int | None = None
    bidder: int | None = None
    for seat, bid in actions:
        if bid is None:
            continue
        assert 30 <= bid <= 41
        assert high is None or bid > high
        high = bid
        bidder = seat
    return bidder, high

def check_history_counterexample() -> dict[str, object]:
    auction_a = ((0, None), (1, 30), (2, None), (3, 31))
    auction_b = ((0, 30), (1, None), (2, None), (3, 31))
    assert validate_point_auction(3, auction_a) == (3, 31)
    assert validate_point_auction(3, auction_b) == (3, 31)

    worlds = enumerate_history_fiber()
    action_31 = domino_id("3-1")
    action_41 = domino_id("4-1")
    tile_44 = domino_id("4-4")

    values_by_world = {
        world: single_seat_root_values(world) for world in worlds
    }

    anchor0 = normalized_hands(
        (
            tuple(map(domino_id, ("5-5", "4-4"))),
            tuple(map(domino_id, ("3-2", "6-5"))),
            tuple(map(domino_id, ("5-3", "6-2"))),
            VIEWER_HAND,
        )
    )
    anchor1 = normalized_hands(
        (
            tuple(map(domino_id, ("5-5", "6-5"))),
            tuple(map(domino_id, ("3-2", "4-4"))),
            tuple(map(domino_id, ("5-3", "6-2"))),
            VIEWER_HAND,
        )
    )
    assert values_by_world[anchor0] == {action_31: 10, action_41: -22}
    assert values_by_world[anchor1] == {action_31: -22, action_41: 22}

    grouped_means: dict[int, dict[int, Fraction]] = {}
    grouped_make: dict[int, dict[int, Fraction]] = {}
    for holder in range(3):
        group = [world for world in worlds if tile_44 in world[holder]]
        assert len(group) == 30
        grouped_means[holder] = {
            action: Fraction(
                sum(values_by_world[world][action] for world in group), len(group)
            )
            for action in (action_31, action_41)
        }
        grouped_make[holder] = {
            action: Fraction(
                sum(values_by_world[world][action] >= 4 for world in group),
                len(group),
            )
            for action in (action_31, action_41)
        }

    assert grouped_means == {
        0: {action_31: Fraction(-104, 15), action_41: Fraction(-98, 5)},
        1: {action_31: Fraction(-122, 15), action_41: Fraction(86, 5)},
        2: {action_31: Fraction(-104, 15), action_41: Fraction(-98, 5)},
    }
    assert grouped_make == {
        0: {action_31: Fraction(1, 3), action_41: Fraction(0)},
        1: {action_31: Fraction(1, 3), action_41: Fraction(4, 5)},
        2: {action_31: Fraction(1, 3), action_41: Fraction(0)},
    }

    def p_bid_30(world: tuple[tuple[int, ...], ...], seat: int) -> Fraction:
        return Fraction(2, 3) if tile_44 in world[seat] else Fraction(1, 3)

    # History A: seat 0 passes, seat 1 bids 30, seat 2 passes, seat 3 bids 31.
    # History B: seat 0 bids 30, seat 1's P(30) is then illegal and it passes,
    #            seat 2 passes, seat 3 bids 31.
    weights_a = tuple((1 - p_bid_30(world, 0)) * p_bid_30(world, 1) for world in worlds)
    weights_b = tuple(p_bid_30(world, 0) for world in worlds)
    assert all(weight > 0 for weight in weights_a + weights_b)

    def normalize(weights: tuple[Fraction, ...]) -> tuple[Fraction, ...]:
        total = sum(weights, Fraction())
        assert total > 0
        return tuple(weight / total for weight in weights)

    posterior_a = normalize(weights_a)
    posterior_b = normalize(weights_b)

    def holder_masses(posterior: tuple[Fraction, ...]) -> tuple[Fraction, ...]:
        return tuple(
            sum(
                probability
                for world, probability in zip(worlds, posterior)
                if tile_44 in world[holder]
            )
            for holder in range(3)
        )

    assert holder_masses(posterior_a) == (
        Fraction(1, 7),
        Fraction(4, 7),
        Fraction(2, 7),
    )
    assert holder_masses(posterior_b) == (
        Fraction(1, 2),
        Fraction(1, 4),
        Fraction(1, 4),
    )

    def expected_values(
        posterior: tuple[Fraction, ...]
    ) -> dict[int, Fraction]:
        return {
            action: sum(
                probability * values_by_world[world][action]
                for world, probability in zip(worlds, posterior)
            )
            for action in (action_31, action_41)
        }

    def make_probabilities(
        posterior: tuple[Fraction, ...]
    ) -> dict[int, Fraction]:
        # 22 points remain.  Signed remaining differential q = 2*x - 22,
        # where x is declaring-team remaining points.  With 18 already banked
        # on a 31 contract, make requires x >= 13, equivalently q >= 4.
        return {
            action: sum(
                probability
                for world, probability in zip(worlds, posterior)
                if values_by_world[world][action] >= 4
            )
            for action in (action_31, action_41)
        }

    expected_a = expected_values(posterior_a)
    expected_b = expected_values(posterior_b)
    make_a = make_probabilities(posterior_a)
    make_b = make_probabilities(posterior_b)

    assert expected_a == {
        action_31: Fraction(-160, 21),
        action_41: Fraction(10, 7),
    }
    assert expected_b == {
        action_31: Fraction(-217, 30),
        action_41: Fraction(-52, 5),
    }
    assert make_a == {
        action_31: Fraction(1, 3),
        action_41: Fraction(16, 35),
    }
    assert make_b == {
        action_31: Fraction(1, 3),
        action_41: Fraction(1, 5),
    }

    assert max(expected_a, key=expected_a.get) == action_41
    assert max(expected_b, key=expected_b.get) == action_31
    assert max(make_a, key=make_a.get) == action_41
    assert max(make_b, key=make_b.get) == action_31

    return {
        "fiber_size": len(worlds),
        "anchor0": {domino_name(k): v for k, v in values_by_world[anchor0].items()},
        "anchor1": {domino_name(k): v for k, v in values_by_world[anchor1].items()},
        "masses_a": holder_masses(posterior_a),
        "masses_b": holder_masses(posterior_b),
        "expected_a": {domino_name(k): v for k, v in expected_a.items()},
        "expected_b": {domino_name(k): v for k, v in expected_b.items()},
        "make_a": {domino_name(k): v for k, v in make_a.items()},
        "make_b": {domino_name(k): v for k, v in make_b.items()},
    }


def check_negative_witnesses() -> None:
    zero_zero = domino_id("0-0")
    one_one = domino_id("1-1")

    def threat(domino: int) -> frozenset[int]:
        suit = led_suit(domino, NO_TRUMP)
        key = trick_key(domino, suit, NO_TRUMP)
        return frozenset(
            other
            for other in range(N_DOMINOES)
            if trick_key(other, suit, NO_TRUMP) > key
        )

    assert threat(zero_zero) == threat(one_one) == frozenset()
    assert follows(zero_zero, 0, NO_TRUMP) and not follows(
        zero_zero, 1, NO_TRUMP
    )
    assert follows(one_one, 1, NO_TRUMP) and not follows(
        one_one, 0, NO_TRUMP
    )

    universe = {"a", "b"}
    cartesian = [(x, y) for x in universe for y in universe]
    feasible = [(x, y) for x, y in cartesian if x != y]
    assert len(cartesian) == 4
    assert set(feasible) == {("a", "b"), ("b", "a")}

    local_universe = (0, 1)
    local_possible = (frozenset({0, 1}), frozenset({0}))
    local_capacities = (1, 1)
    local_worlds = enumerate_abstract_worlds(
        local_universe,
        local_possible,
        local_capacities,
    )
    assert local_worlds == frozenset({((1,), (0,))})
    assert 0 in local_possible[0]
    assert all(0 not in world[0] for world in local_worlds)

    reduced_universe = (0, 1, 2)
    reduced_possible = (
        frozenset(),
        frozenset({0, 1}),
        frozenset({0, 1, 2}),
    )
    reduced_capacities = (0, 1, 2)
    reduced_worlds = enumerate_abstract_worlds(
        reduced_universe,
        reduced_possible,
        reduced_capacities,
    )
    reduced_projection = tuple(
        frozenset(
            domino
            for world in reduced_worlds
            for domino in world[seat]
        )
        for seat in range(3)
    )
    assert reduced_projection == reduced_possible
    raw_successor = forced_successor_system(
        reduced_universe,
        reduced_possible,
        reduced_capacities,
        0,
        2,
    )
    assert raw_successor is not None
    successor_worlds = enumerate_abstract_worlds(*raw_successor)
    successor_projection = tuple(
        frozenset(
            domino
            for world in successor_worlds
            for domino in world[seat]
        )
        for seat in range(3)
    )
    assert successor_projection != raw_successor[1]
    assert 1 in raw_successor[1][2]
    assert 1 not in successor_projection[2]

    reflect = lambda seat: (-seat) % 4
    assert any(
        reflect((seat + 1) % 4) != (reflect(seat) + 1) % 4
        for seat in range(4)
    )


def format_fraction(value: Fraction) -> str:
    return str(value.numerator) if value.denominator == 1 else str(value)


def main() -> None:
    check_basic_algebra()
    unique_cases, prose_agreements = check_unique_winner_exhaustive()
    auction_counts, auction_maxima = check_auction_exhaustive()
    pip_survivors = check_pip_transport_exhaustive()
    (
        hall_cases,
        fiber_count_cases,
        uniform_world_checks,
        marginal_edge_checks,
        reduction_checks,
    ) = check_hall_small_exhaustive()
    (
        native_capacity_profiles,
        native_fiber_max,
        native_total_state_bound,
        native_candidate_check_bound,
        native_eligible_update_bound,
        native_max_layer_bound,
    ) = check_native_fiber_count_bound()
    abstract_leads, abstract_follows, abstract_sloughs = (
        check_typed_transition_small_exhaustive()
    )
    (
        support_prefixes,
        support_prefixes_with_voids,
        support_transitions,
        hidden_support_transitions,
        viewer_support_transitions,
    ) = check_reachable_cell_support_corpus()
    history = check_history_counterexample()
    check_negative_witnesses()

    deal_count = factorial(28) // (factorial(7) ** 4)
    conditional_count = factorial(21) // (factorial(7) ** 3)
    assert deal_count == 472_518_347_558_400
    assert conditional_count == 399_072_960

    print("Texas 42 Foundations finite verification: PASS")
    print(f"dominoes: {N_DOMINOES}")
    print(f"straight declarations: {len(DECLARATIONS)}")
    print(f"ordered deals: {deal_count:,}")
    print(f"conditional hidden assignments: {conditional_count:,}")
    print(f"unique-winner cases: {unique_cases:,}")
    print(f"prose-rule winner agreement cases: {prose_agreements:,}")
    print(f"auction terminal histories for caps 1..7: {auction_counts}")
    print(f"auction reached mark maxima for caps 1..7: {auction_maxima}")
    print("auction histories for caps 5, 6, and 7: identical")
    print(f"Hall systems checked: {hall_cases:,}")
    print(
        "exact fiber-count coefficient/recurrence agreements: "
        f"{fiber_count_cases:,}"
    )
    print(
        "exact uniform count-ratio sampler world probabilities checked: "
        f"{uniform_world_checks:,}"
    )
    print(
        "marginal holder edges checked by world projection vs Hall: "
        f"{marginal_edge_checks:,}"
    )
    print(f"canonical support-reduction systems checked: {reduction_checks:,}")
    print(
        "native unrestricted capacity profiles checked: "
        f"{native_capacity_profiles:,}"
    )
    print(
        "native unrestricted fiber maximum: "
        f"{native_fiber_max:,} worlds; "
        f"count DP <= {native_total_state_bound:,} occupancy states total, "
        f"{native_candidate_check_bound:,} candidate-holder checks, "
        f"{native_eligible_update_bound:,} capacity-eligible updates, "
        f"{native_max_layer_bound:,} live states/layer"
    )
    print(
        "abstract typed cell transitions (universes <= 3): "
        f"{abstract_leads:,} leads; "
        f"{abstract_follows:,} follows; "
        f"{abstract_sloughs:,} sloughs"
    )
    print(
        "reachable cell/deal support parity prefixes: "
        f"{support_prefixes:,} ({support_prefixes_with_voids:,} with public voids)"
    )
    print(
        "typed support transitions: "
        f"{support_transitions:,} total; "
        f"{hidden_support_transitions:,} hidden nonincrease; "
        f"{viewer_support_transitions:,} viewer equality"
    )
    print("count points: 35; seven trick points: 7; total: 42")
    print(f"count-preserving pip permutations: {len(pip_survivors)}")
    print("scoped nontrivial pip transport: 2 <-> 3 only in declarations 2 and 3")
    print("history witness auction paths: legal; bidder seat 3 at 31")
    print(f"history witness fiber worlds: {history['fiber_size']}")
    print(f"history anchor world 0 Q: {history['anchor0']}")
    print(f"history anchor world 1 Q: {history['anchor1']}")
    print(
        "history A posterior 4-4-holder masses: "
        + str(tuple(map(format_fraction, history["masses_a"])))
    )
    print(
        "history B posterior 4-4-holder masses: "
        + str(tuple(map(format_fraction, history["masses_b"])))
    )
    print(f"history A posterior expected Q: {history['expected_a']}")
    print(f"history B posterior expected Q: {history['expected_b']}")
    print(f"history A posterior make probabilities: {history['make_a']}")
    print(f"history B posterior make probabilities: {history['make_b']}")
    print("same 90-world rule support and posterior support; opposite best leads: PASS")
    print("negative witnesses: threat, dependent cells, local-vs-marginal edge, reduction instability, reflection: PASS")

if __name__ == "__main__":
    main()
