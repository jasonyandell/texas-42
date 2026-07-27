#!/usr/bin/env python3
"""Exact finite checks for support minimality and Straight reachability.

This proof receipt supplements ``verify_foundation.py``.  It checks the finite
censuses and constructed witnesses used by sections 7.10--7.13 of the
mathematical foundation.  It uses only the Python standard library, but it
imports shared abstract-world enumeration helpers from ``verify_foundation``;
the two entry points are therefore not independent implementations.  Neither
serves as implementation source code.
"""

from __future__ import annotations

from collections import Counter, defaultdict
from functools import lru_cache
from itertools import combinations, permutations, product
from math import comb, factorial

import verify_foundation as base

PIPS = tuple(range(7))
PIP_DECLARATIONS = tuple(range(7))
DOUBLES_TRUMP = 7
NO_TRUMP = 8
DECLARATIONS = PIP_DECLARATIONS + (DOUBLES_TRUMP, NO_TRUMP)
CALLED = 7
SEATS = (0, 1, 2)  # hidden seats in clockwise viewer-relative order
PERMUTATIONS = tuple(permutations(SEATS))

# Canonical physical identity order: 0-0, 1-0, 1-1, ..., 6-6.
DOMINOES = tuple((high, low) for high in PIPS for low in range(high + 1))
N_DOMINOES = len(DOMINOES)


def contains(domino: tuple[int, int], pip: int) -> bool:
    high, low = domino
    return high == pip or low == pip


def is_double(domino: tuple[int, int]) -> bool:
    high, low = domino
    return high == low


def called_set(declaration: int) -> frozenset[tuple[int, int]]:
    if declaration in PIP_DECLARATIONS:
        return frozenset(d for d in DOMINOES if contains(d, declaration))
    if declaration == DOUBLES_TRUMP:
        return frozenset(d for d in DOMINOES if is_double(d))
    if declaration == NO_TRUMP:
        return frozenset()
    raise ValueError(declaration)


def effective_follow_set(
    declaration: int, context: int
) -> frozenset[tuple[int, int]]:
    called = called_set(declaration)
    if context == CALLED:
        return called
    return frozenset(
        d for d in DOMINOES if contains(d, context) and d not in called
    )


def led_context(domino: tuple[int, int], declaration: int) -> int:
    return CALLED if domino in called_set(declaration) else domino[0]


def lead_fibers(
    declaration: int,
) -> dict[int, frozenset[tuple[int, int]]]:
    result: dict[int, set[tuple[int, int]]] = defaultdict(set)
    for domino in DOMINOES:
        result[led_context(domino, declaration)].add(domino)
    return {key: frozenset(value) for key, value in result.items()}


def hall_feasible(
    universe: frozenset[tuple[int, int]],
    possible: tuple[frozenset[tuple[int, int]], ...],
    capacities: tuple[int, ...],
) -> bool:
    if len(universe) != sum(capacities):
        return False
    for mask in range(1, 1 << len(capacities)):
        neighbors: set[tuple[int, int]] = set()
        quota = 0
        for seat in range(len(capacities)):
            if mask & (1 << seat):
                neighbors.update(possible[seat] & universe)
                quota += capacities[seat]
        if len(neighbors) < quota:
            return False
    return True


def marginal_holder_sets(
    universe: frozenset[tuple[int, int]],
    possible: tuple[frozenset[tuple[int, int]], ...],
    capacities: tuple[int, ...],
) -> tuple[frozenset[tuple[int, int]], ...] | None:
    if not hall_feasible(universe, possible, capacities):
        return None
    projected: list[set[tuple[int, int]]] = [set() for _ in capacities]
    for domino in universe:
        for seat, capacity in enumerate(capacities):
            if capacity == 0 or domino not in possible[seat]:
                continue
            successor_universe = universe - {domino}
            successor_possible = tuple(p - {domino} for p in possible)
            successor_capacities = list(capacities)
            successor_capacities[seat] -= 1
            if hall_feasible(
                successor_universe,
                successor_possible,
                tuple(successor_capacities),
            ):
                projected[seat].add(domino)
    return tuple(frozenset(values) for values in projected)


def multinomial_assignments_with_bounds(
    remainder: int, bounds: tuple[int, int, int]
) -> int:
    """The F(R;b) function from the support census."""

    total = 0
    fact_r = factorial(remainder)
    for c0 in range(min(bounds[0], remainder) + 1):
        for c1 in range(min(bounds[1], remainder - c0) + 1):
            max_c2 = min(bounds[2], remainder - c0 - c1)
            for c2 in range(max_c2 + 1):
                outside = remainder - c0 - c1 - c2
                total += fact_r // (
                    factorial(c0)
                    * factorial(c1)
                    * factorial(c2)
                    * factorial(outside)
                )
    return total


def valid_ternary_signatures() -> tuple[tuple[int, int, int, int, int, int], ...]:
    signatures: list[tuple[int, int, int, int, int, int]] = []
    for r0, r1, r2 in product(range(1, 8), repeat=3):
        n = r0 + r1 + r2
        for n0 in range(n + 1):
            if n - n0 < r0 + 1:
                continue
            for n1 in range(n - n0 + 1):
                if n - n1 < r1 + 1:
                    continue
                max_n2 = n - n0 - n1
                for n2 in range(max_n2 + 1):
                    if n - n2 < r2 + 1:
                        continue
                    signatures.append((r0, n0, r1, n1, r2, n2))
    return tuple(signatures)


def allocation_matrices(
    signature: tuple[int, int, int, int, int, int]
) -> tuple[tuple[tuple[int, ...], ...], ...]:
    """Enumerate category-allocation matrices for one ternary signature.

    Matrix columns are ``star, excludes-0, excludes-1, excludes-2`` and rows
    are holder seats 0, 1, 2.  A matrix is represented row-major.
    """

    r0, n0, r1, n1, r2, n2 = signature
    n = r0 + r1 + r2
    nstar = n - n0 - n1 - n2
    matrices: list[tuple[tuple[int, ...], ...]] = []

    # x0: excludes-0 category assigned to seat 1; remainder to seat 2.
    # x1: excludes-1 category assigned to seat 0; remainder to seat 2.
    # x2: excludes-2 category assigned to seat 0; remainder to seat 1.
    for x0 in range(n0 + 1):
        for x1 in range(n1 + 1):
            for x2 in range(n2 + 1):
                star0 = r0 - x1 - x2
                star1 = r1 - x0 - (n2 - x2)
                star2 = r2 - (n0 - x0) - (n1 - x1)
                if min(star0, star1, star2) < 0:
                    continue
                if star0 + star1 + star2 != nstar:
                    continue
                matrix = (
                    (star0, 0, x1, x2),
                    (star1, x0, 0, n2 - x2),
                    (star2, n0 - x0, n1 - x1, 0),
                )
                matrices.append(matrix)
    return tuple(matrices)


def canonical_signature(
    signature: tuple[int, int, int, int, int, int]
) -> tuple[tuple[int, int], ...]:
    r0, n0, r1, n1, r2, n2 = signature
    return tuple(sorted(((r0, n0), (r1, n1), (r2, n2))))


def representative_signature(
    key: tuple[tuple[int, int], ...]
) -> tuple[int, int, int, int, int, int]:
    return (
        key[0][0],
        key[0][1],
        key[1][0],
        key[1][1],
        key[2][0],
        key[2][1],
    )


def stabilizer(
    key: tuple[tuple[int, int], ...]
) -> tuple[tuple[int, int, int], ...]:
    return tuple(
        permutation
        for permutation in PERMUTATIONS
        if tuple(key[permutation.index(new)] for new in SEATS) == key
    )


def permute_matrix(
    matrix: tuple[tuple[int, ...], ...],
    permutation: tuple[int, int, int],
) -> tuple[tuple[int, ...], ...]:
    """Simultaneously relabel holder seats and excluded-seat categories.

    ``permutation[old]`` is the new label of an old seat.
    """

    result = [[0] * 4 for _ in SEATS]
    for old_row in SEATS:
        new_row = permutation[old_row]
        result[new_row][0] = matrix[old_row][0]
        for old_excluded in SEATS:
            new_excluded = permutation[old_excluded]
            result[new_row][1 + new_excluded] = matrix[old_row][1 + old_excluded]
    return tuple(tuple(row) for row in result)



def generic_scc_components(
    universe: tuple[int, ...],
    possible: tuple[frozenset[int], ...],
    witness: tuple[tuple[int, ...], ...],
) -> dict[tuple[str, int], int]:
    vertices = tuple(("d", d) for d in universe) + tuple(
        ("s", seat) for seat in range(3)
    )
    adjacency: dict[tuple[str, int], list[tuple[str, int]]] = {
        vertex: [] for vertex in vertices
    }
    holder = {
        domino: seat
        for seat, hand in enumerate(witness)
        for domino in hand
    }
    for seat in range(3):
        for domino in possible[seat]:
            if holder[domino] == seat:
                adjacency[("s", seat)].append(("d", domino))
            else:
                adjacency[("d", domino)].append(("s", seat))

    index = 0
    stack: list[tuple[str, int]] = []
    on_stack: set[tuple[str, int]] = set()
    indices: dict[tuple[str, int], int] = {}
    lowlink: dict[tuple[str, int], int] = {}
    component: dict[tuple[str, int], int] = {}
    component_id = 0

    def visit(vertex: tuple[str, int]) -> None:
        nonlocal index, component_id
        indices[vertex] = index
        lowlink[vertex] = index
        index += 1
        stack.append(vertex)
        on_stack.add(vertex)

        for successor in adjacency[vertex]:
            if successor not in indices:
                visit(successor)
                lowlink[vertex] = min(lowlink[vertex], lowlink[successor])
            elif successor in on_stack:
                lowlink[vertex] = min(lowlink[vertex], indices[successor])

        if lowlink[vertex] == indices[vertex]:
            while True:
                member = stack.pop()
                on_stack.remove(member)
                component[member] = component_id
                if member == vertex:
                    break
            component_id += 1

    for vertex in vertices:
        if vertex not in indices:
            visit(vertex)
    return component


def generic_support_normal_form(
    universe: tuple[int, ...],
    capacities: tuple[int, int, int],
    worlds: frozenset[tuple[tuple[int, ...], ...]],
) -> tuple:
    if not worlds:
        return ("EMPTY",)
    marginal = tuple(
        frozenset(
            domino
            for world in worlds
            for domino in world[seat]
        )
        for seat in range(3)
    )
    holders = {
        domino: frozenset(
            seat for seat in range(3) if domino in marginal[seat]
        )
        for domino in universe
    }
    certain = tuple(
        tuple(sorted(d for d in universe if holders[d] == {seat}))
        for seat in range(3)
    )
    certain_union = {d for hand in certain for d in hand}
    ambiguous = tuple(d for d in universe if d not in certain_union)
    residual = tuple(capacities[s] - len(certain[s]) for s in range(3))
    active = tuple(s for s in range(3) if residual[s] > 0)
    assert len(active) in (0, 2, 3)
    if not active:
        assert not ambiguous
        ambiguity = ("D",)
    elif len(active) == 2:
        inactive = next(s for s in range(3) if s not in active)
        assert all(holders[d] == frozenset(active) for d in ambiguous)
        ambiguity = ("B", inactive, ambiguous, residual[active[0]])
    else:
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


def decode_generic_normal_form(normal: tuple) -> frozenset[tuple[tuple[int, ...], ...]]:
    if normal == ("EMPTY",):
        return frozenset()
    _, certain, ambiguity = normal
    certain_sets = tuple(frozenset(hand) for hand in certain)
    if ambiguity[0] == "D":
        return frozenset({tuple(tuple(sorted(hand)) for hand in certain_sets)})
    if ambiguity[0] == "B":
        _, inactive, ambiguous, first = ambiguity
        active = tuple(s for s in range(3) if s != inactive)
        residual = [0, 0, 0]
        residual[active[0]] = first
        residual[active[1]] = len(ambiguous) - first
        possible = tuple(
            frozenset(ambiguous) if s in active else frozenset()
            for s in range(3)
        )
    else:
        _, ambiguous, r0, r1, exclusions = ambiguity
        excluded = dict(exclusions)
        residual = [r0, r1, len(ambiguous) - r0 - r1]
        possible = tuple(
            frozenset(d for d in ambiguous if excluded.get(d) != s)
            for s in range(3)
        )
    ambiguous_worlds = base.enumerate_abstract_worlds(
        tuple(ambiguous), tuple(possible), tuple(residual)
    )
    return frozenset(
        tuple(
            tuple(sorted(set(certain_sets[s]) | set(world[s])))
            for s in range(3)
        )
        for world in ambiguous_worlds
    )


def generic_ordered_count_rank_checks(
    normal: tuple,
    worlds: frozenset[tuple[tuple[int, ...], ...]],
) -> None:
    if normal == ("EMPTY",):
        return
    _, certain, ambiguity = normal
    certain_sets = tuple(set(hand) for hand in certain)
    if ambiguity[0] == "D":
        assert len(worlds) == 1
        return
    if ambiguity[0] == "B":
        _, inactive, ambiguous, first = ambiguity
        active = tuple(s for s in range(3) if s != inactive)
        start = [0, 0, 0]
        start[active[0]] = first
        start[active[1]] = len(ambiguous) - first
        allowed = {d: active for d in ambiguous}
    else:
        _, ambiguous, r0, r1, exclusions = ambiguity
        start = [r0, r1, len(ambiguous) - r0 - r1]
        excluded = dict(exclusions)
        allowed = {
            d: tuple(s for s in range(3) if excluded.get(d) != s)
            for d in ambiguous
        }
    order = tuple(ambiguous)
    start_tuple = tuple(start)

    @lru_cache(maxsize=None)
    def completions(index: int, remaining: tuple[int, int, int]) -> int:
        if index == len(order):
            return int(remaining == (0, 0, 0))
        domino = order[index]
        total = 0
        for seat in allowed[domino]:
            if remaining[seat] == 0:
                continue
            successor = list(remaining)
            successor[seat] -= 1
            total += completions(index + 1, tuple(successor))
        return total

    assert completions(0, start_tuple) == len(worlds)

    strings = []
    for world in worlds:
        holder = {
            domino: seat
            for seat, hand in enumerate(world)
            for domino in hand
            if domino not in certain_sets[seat]
        }
        strings.append(tuple(holder[d] for d in order))
    strings.sort()

    def rank_string(string: tuple[int, ...]) -> int:
        rank = 0
        remaining = list(start_tuple)
        for index, chosen in enumerate(string):
            domino = order[index]
            for seat in sorted(allowed[domino]):
                if seat >= chosen:
                    break
                if remaining[seat] == 0:
                    continue
                successor = list(remaining)
                successor[seat] -= 1
                rank += completions(index + 1, tuple(successor))
            assert chosen in allowed[domino] and remaining[chosen] > 0
            remaining[chosen] -= 1
        return rank

    def unrank_string(rank: int) -> tuple[int, ...]:
        remaining = list(start_tuple)
        result = []
        for index, domino in enumerate(order):
            for seat in sorted(allowed[domino]):
                if remaining[seat] == 0:
                    continue
                successor = list(remaining)
                successor[seat] -= 1
                block = completions(index + 1, tuple(successor))
                if rank < block:
                    result.append(seat)
                    remaining = successor
                    break
                rank -= block
            else:
                raise AssertionError("rank outside language")
        assert rank == 0
        return tuple(result)

    for expected_rank, string in enumerate(strings):
        assert rank_string(string) == expected_rank
        assert unrank_string(expected_rank) == string


def check_tiny_support_normal_forms() -> dict[str, int]:
    checked = 0
    feasible = 0
    witness_scc_checks = 0
    exclusions_removed = 0
    rank_worlds = 0
    fiber_to_normal: dict[frozenset, tuple] = {}
    normal_to_fiber: dict[tuple, frozenset] = {}

    for universe_size in range(1, 5):
        universe = tuple(range(universe_size))
        subsets = tuple(
            frozenset(x for x in universe if mask & (1 << x))
            for mask in range(1 << universe_size)
        )
        for possible in product(subsets, repeat=3):
            for k0 in range(universe_size + 1):
                for k1 in range(universe_size - k0 + 1):
                    capacities = (k0, k1, universe_size - k0 - k1)
                    worlds = base.enumerate_abstract_worlds(
                        universe, possible, capacities
                    )
                    normal = generic_support_normal_form(
                        universe, capacities, worlds
                    )
                    assert decode_generic_normal_form(normal) == worlds
                    assert fiber_to_normal.setdefault(worlds, normal) == normal
                    assert normal_to_fiber.setdefault(normal, worlds) == worlds
                    checked += 1
                    if not worlds:
                        continue
                    feasible += 1
                    rank_worlds += len(worlds)

                    marginal = tuple(
                        frozenset(
                            domino
                            for world in worlds
                            for domino in world[seat]
                        )
                        for seat in range(3)
                    )
                    for witness in worlds:
                        components = generic_scc_components(
                            universe, possible, witness
                        )
                        holder = {
                            domino: seat
                            for seat, hand in enumerate(witness)
                            for domino in hand
                        }
                        compiled = []
                        for seat in range(3):
                            supported = set(witness[seat])
                            for domino in possible[seat]:
                                if holder[domino] == seat:
                                    continue
                                if components[("d", domino)] == components[("s", seat)]:
                                    supported.add(domino)
                            compiled.append(frozenset(supported))
                        assert tuple(compiled) == marginal
                        witness_scc_checks += 1

                    if normal[0] == "F":
                        _, certain, ambiguity = normal
                        if ambiguity[0] == "B":
                            _, inactive, ambiguous, first = ambiguity
                            assert len(worlds) == comb(len(ambiguous), first)
                        elif ambiguity[0] == "T":
                            _, ambiguous, r0, r1, exclusions = ambiguity
                            r2 = len(ambiguous) - r0 - r1
                            excluded = dict(exclusions)
                            possible_amb = tuple(
                                frozenset(d for d in ambiguous if excluded.get(d) != s)
                                for s in range(3)
                            )
                            residual = (r0, r1, r2)
                            # Strict Hall on every nonempty proper active subset.
                            for mask in range(1, 7):
                                neighbors = set()
                                quota = 0
                                for seat in range(3):
                                    if mask & (1 << seat):
                                        neighbors.update(possible_amb[seat])
                                        quota += residual[seat]
                                assert len(neighbors) >= quota + 1
                            # Every stored exclusion is essential.
                            for domino, seat in exclusions:
                                relaxed = list(possible_amb)
                                relaxed[seat] = relaxed[seat] | {domino}
                                before = base.enumerate_abstract_worlds(
                                    tuple(ambiguous), possible_amb, residual
                                )
                                after = base.enumerate_abstract_worlds(
                                    tuple(ambiguous), tuple(relaxed), residual
                                )
                                assert before < after
                                exclusions_removed += 1

                            # Grouped category coefficient agrees with worlds.
                            ncounts = tuple(
                                sum(1 for d in ambiguous if excluded.get(d) == s)
                                for s in range(3)
                            )
                            signature = (r0, ncounts[0], r1, ncounts[1], r2, ncounts[2])
                            matrix_weight = 0
                            for matrix in allocation_matrices(signature):
                                weight = 1
                                for column in range(4):
                                    total = sum(matrix[row][column] for row in range(3))
                                    weight *= factorial(total)
                                    for row in range(3):
                                        weight //= factorial(matrix[row][column])
                                matrix_weight += weight
                            assert matrix_weight == len(worlds)

                    generic_ordered_count_rank_checks(normal, worlds)

    assert checked == 66_968
    assert witness_scc_checks == 22_620

    # Universal unrestricted ternary automaton contains all 8^3 residual vectors.
    coaccessible = {
        (v0, v1, v2)
        for v0, v1, v2 in product(range(8), repeat=3)
    }
    assert len(coaccessible) == 512
    assert {
        v0 + (v1 << 3) + (v2 << 6) for v0, v1, v2 in coaccessible
    } == set(range(512))

    return {
        "systems": checked,
        "feasible": feasible,
        "scc_witnesses": witness_scc_checks,
        "exclusions": exclusions_removed,
        "rank_worlds": rank_worlds,
    }


def check_native_support_census() -> dict[str, int]:
    signatures = valid_ternary_signatures()
    assert len(signatures) == 136_514

    matrix_total = 0
    max_matrices = 0
    canonical_keys: set[tuple[tuple[int, int], ...]] = set()
    for signature in signatures:
        matrices = allocation_matrices(signature)
        assert matrices
        matrix_total += len(matrices)
        max_matrices = max(max_matrices, len(matrices))
        canonical_keys.add(canonical_signature(signature))
    assert matrix_total == 1_667_666
    assert max_matrices == 114
    assert len(canonical_keys) == 23_842

    canonical_matrix_total = 0
    orbit_total = 0
    max_orbits = 0
    stabilizer_histogram: Counter[int] = Counter()
    orbit_size_set: set[int] = set()

    for key in sorted(canonical_keys):
        signature = representative_signature(key)
        matrices = allocation_matrices(signature)
        canonical_matrix_total += len(matrices)
        group = stabilizer(key)
        stabilizer_histogram[len(group)] += 1

        unseen = set(matrices)
        orbit_count = 0
        while unseen:
            matrix = min(unseen)
            orbit = {permute_matrix(matrix, p) for p in group}
            assert orbit.issubset(set(matrices))
            orbit_size_set.add(len(orbit))
            unseen.difference_update(orbit)
            orbit_count += 1
        orbit_total += orbit_count
        max_orbits = max(max_orbits, orbit_count)

    assert canonical_matrix_total == 296_721
    assert stabilizer_histogram == Counter({1: 21_686, 2: 2_121, 6: 35})
    assert orbit_total == 279_048
    assert max_orbits == 103
    assert orbit_size_set == {1, 2, 3, 6}

    n_empty = 1
    n_det = multinomial_assignments_with_bounds(28, (7, 7, 7))

    n_bin = 0
    for inactive in SEATS:
        active = tuple(seat for seat in SEATS if seat != inactive)
        for r_a, r_b in product(range(1, 8), repeat=2):
            residuals = [0, 0, 0]
            residuals[active[0]] = r_a
            residuals[active[1]] = r_b
            n = r_a + r_b
            bounds = tuple(7 - residuals[s] for s in SEATS)
            n_bin += comb(28, n) * multinomial_assignments_with_bounds(
                28 - n, bounds
            )

    n_ter = 0
    fact28 = factorial(28)
    for signature in signatures:
        r0, n0, r1, n1, r2, n2 = signature
        n = r0 + r1 + r2
        nstar = n - n0 - n1 - n2
        ambiguity_assignments = fact28 // (
            factorial(28 - n)
            * factorial(n0)
            * factorial(n1)
            * factorial(n2)
            * factorial(nstar)
        )
        n_ter += ambiguity_assignments * multinomial_assignments_with_bounds(
            28 - n, (7 - r0, 7 - r1, 7 - r2)
        )

    assert n_det == 8_102_258_940_222_814
    assert n_bin == 11_495_078_055_913_018_482
    assert n_ter == 1_830_955_704_129_296_418_354_864
    total = n_empty + n_det + n_bin + n_ter
    assert total == 1_830_967_207_309_611_271_596_161
    assert 2**80 < total < 2**81

    return {
        "signatures": len(signatures),
        "matrices": matrix_total,
        "max_matrices": max_matrices,
        "signature_orbits": len(canonical_keys),
        "canonical_matrices": canonical_matrix_total,
        "matrix_orbits": orbit_total,
        "max_matrix_orbits": max_orbits,
        "n_empty": n_empty,
        "n_det": n_det,
        "n_bin": n_bin,
        "n_ter": n_ter,
        "support_total": total,
    }


def hidden_capacity_profiles() -> tuple[tuple[int, int, int], ...]:
    return tuple(
        capacities
        for capacities in product(range(8), repeat=3)
        if max(capacities) - min(capacities) <= 1
    )


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


def profile_parameters(
    capacities: tuple[int, int, int]
) -> tuple[int, int, int, frozenset[int]]:
    """Return pool size n, completed tricks j, |F(B)|, and low-seat set B."""

    n = sum(capacities)
    if capacities[0] == capacities[1] == capacities[2]:
        h = capacities[0]
        return n, 7 - h, 0, frozenset()
    h = max(capacities)
    assert min(capacities) == h - 1
    low = frozenset(i for i, value in enumerate(capacities) if value == h - 1)
    followers = FOLLOWER_MAXIMUM[low]
    return n, 7 - h, len(followers), low


def enumerate_turn_prefix_profiles() -> dict[
    frozenset[int], set[frozenset[int]]
]:
    """Enumerate low hidden-seat sets and hidden followers in a partial trick."""

    result: dict[frozenset[int], set[frozenset[int]]] = defaultdict(set)
    viewer = 0
    hidden_absolute = (1, 2, 3)
    absolute_to_relative = {1: 0, 2: 1, 3: 2}
    for leader in range(4):
        order = tuple((leader + offset) % 4 for offset in range(4))
        for played in range(4):
            prefix = order[:played]
            low = frozenset(
                absolute_to_relative[seat]
                for seat in prefix
                if seat in hidden_absolute
            )
            followers = frozenset(
                absolute_to_relative[seat]
                for seat in prefix[1:]
                if seat in hidden_absolute
            )
            result[low].add(followers)
    return result


def check_capacity_and_context_reachability() -> dict[str, object]:
    profiles = hidden_capacity_profiles()
    assert len(profiles) == 50
    assert all(max(k) - min(k) <= 1 for k in profiles)

    prefix_profiles = enumerate_turn_prefix_profiles()
    assert set(prefix_profiles) == set(FOLLOWER_MAXIMUM)
    for low, followers in prefix_profiles.items():
        maximum = FOLLOWER_MAXIMUM[low]
        assert maximum in followers
        assert all(candidate.issubset(maximum) for candidate in followers)

    expected_contexts = {
        **{
            trump: (set(PIPS) - {trump}) | {CALLED}
            for trump in PIP_DECLARATIONS
        },
        DOUBLES_TRUMP: {1, 2, 3, 4, 5, 6, CALLED},
        NO_TRUMP: set(PIPS),
    }
    lead_sizes_by_declaration: dict[int, tuple[int, ...]] = {}
    for declaration in DECLARATIONS:
        fibers = lead_fibers(declaration)
        assert set(fibers) == expected_contexts[declaration]
        assert set().union(*fibers.values()) == set(DOMINOES)
        assert sum(len(value) for value in fibers.values()) == 28
        assert all(
            fibers[a].isdisjoint(fibers[b])
            for a, b in combinations(fibers, 2)
        )
        sizes = tuple(sorted(len(value) for value in fibers.values()))
        assert sizes == (1, 2, 3, 4, 5, 6, 7)
        lead_sizes_by_declaration[declaration] = sizes

    # Doubles-trump natural effective suit 0 is nonempty but unleadable.
    zero_follow = effective_follow_set(DOUBLES_TRUMP, 0)
    assert zero_follow
    assert 0 not in lead_fibers(DOUBLES_TRUMP)

    return {
        "profiles": len(profiles),
        "prefix_profile_count": len(prefix_profiles),
        "lead_sizes": lead_sizes_by_declaration,
    }


def exact_schedule_counts() -> tuple[tuple[int, ...], tuple[int, ...], tuple[int, ...]]:
    """Exhaust the 8^7 possible triples of per-context void membership."""

    by_used = Counter()
    by_used_has_f1 = Counter()
    by_used_has_f2 = Counter()

    for code in range(8**7):
        value = code
        used = 0
        has_f1 = False  # nonempty subset of canonical one-seat F={0}
        has_f2 = False  # nonempty subset of canonical two-seat F={0,1}
        for _ in range(7):
            pattern = value & 7
            value >>= 3
            if pattern:
                used += 1
                if pattern == 0b001:
                    has_f1 = True
                if pattern in (0b001, 0b010, 0b011):
                    has_f2 = True
        by_used[used] += 1
        if has_f1:
            by_used_has_f1[used] += 1
        if has_f2:
            by_used_has_f2[used] += 1

    a = []
    t1 = []
    t2 = []
    for j in range(8):
        base = sum(by_used[u] for u in range(j + 1))
        a.append(base)
        if j < 7:
            t1.append(base + by_used_has_f1[j + 1])
            t2.append(base + by_used_has_f2[j + 1])
        else:
            t1.append(base)
            t2.append(base)

    expected_a = (
        1,
        50,
        1_079,
        13_084,
        97_119,
        450_066,
        1_273_609,
        2_097_152,
    )
    expected_t1 = (
        8,
        323,
        5_524,
        51_759,
        286_770,
        947_017,
        1_817_216,
        2_097_152,
    )
    expected_t2 = (
        22,
        743,
        10_844,
        88_159,
        428_562,
        1_244_937,
        2_080_768,
        2_097_152,
    )
    assert tuple(a) == expected_a
    assert tuple(t1) == expected_t1
    assert tuple(t2) == expected_t2

    for j in range(7):
        formula_a = sum(comb(7, u) * 7**u for u in range(j + 1))
        formula_t1 = formula_a + comb(7, j + 1) * (
            7 ** (j + 1) - 6 ** (j + 1)
        )
        formula_t2 = formula_a + comb(7, j + 1) * (
            7 ** (j + 1) - 4 ** (j + 1)
        )
        assert a[j] == formula_a
        assert t1[j] == formula_t1
        assert t2[j] == formula_t2

    return tuple(a), tuple(t1), tuple(t2)


def polynomial_multiply(a: list[int], b: list[int]) -> list[int]:
    result = [0] * (len(a) + len(b) - 1)
    for i, left in enumerate(a):
        for j, right in enumerate(b):
            result[i + j] += left * right
    return result


def lead_witness_subset_counts() -> tuple[tuple[int, ...], ...]:
    """Return B[n][u] for lead-fiber sizes 1..7."""

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


def outer_profile_count(
    capacities: tuple[int, int, int],
    witness_counts: tuple[tuple[int, ...], ...],
) -> int:
    if capacities == (0, 0, 0):
        return 1
    n, completed, follower_count, _ = profile_parameters(capacities)
    total = sum(
        7**used * witness_counts[n][used]
        for used in range(completed + 1)
    )
    if follower_count:
        used = completed + 1
        current_patterns = 7**used - (8 - 2**follower_count) ** used
        total += current_patterns * witness_counts[n][used]
    return total


def check_reachable_support_bounds() -> dict[str, int]:
    witness_counts = lead_witness_subset_counts()
    profiles = hidden_capacity_profiles()
    by_profile = {
        capacities: outer_profile_count(capacities, witness_counts)
        for capacities in profiles
    }
    per_declaration = sum(by_profile.values())
    declaration_tagged = len(DECLARATIONS) * per_declaration
    maximum_fixed_profile = max(by_profile.values())

    assert per_declaration == 7_124_838_074_989
    assert declaration_tagged == 64_123_542_674_901
    assert declaration_tagged < 2**46
    assert per_declaration < 2**43
    assert maximum_fixed_profile == 839_220_930_919
    assert maximum_fixed_profile < 2**40
    assert len(DECLARATIONS) * maximum_fixed_profile < 2**43

    lower_family = (
        comb(28, 21)
        + 3 * comb(28, 20)
        + 3 * comb(28, 19)
        + comb(28, 18)
    )
    assert lower_family == 44_352_165
    assert 2**25 < lower_family < 2**26

    return {
        "per_declaration": per_declaration,
        "declaration_tagged": declaration_tagged,
        "maximum_fixed_profile": maximum_fixed_profile,
        "lower_family": lower_family,
    }


def check_feasible_unreachable_witness() -> dict[str, object]:
    sigma_zero = frozenset(d for d in DOMINOES if contains(d, 0))
    doubles = frozenset(d for d in DOMINOES if is_double(d))
    extras = frozenset(((2, 1), (3, 1), (3, 2), (4, 1), (4, 2)))
    universe = sigma_zero | doubles | extras
    assert len(universe) == 18
    capacities = (6, 6, 6)
    target_possible = (universe - sigma_zero, universe, universe)
    target_marginal = marginal_holder_sets(
        universe, target_possible, capacities
    )
    assert target_marginal == target_possible

    matches: list[tuple[int, int, int]] = []
    matched_lead_fibers: list[frozenset[tuple[int, int]]] = []
    no_void_matches = 0
    raw_generators_checked = 0

    for declaration in DECLARATIONS:
        no_void = (universe, universe, universe)
        raw_generators_checked += 1
        if marginal_holder_sets(universe, no_void, capacities) == target_marginal:
            no_void_matches += 1

        fibers = lead_fibers(declaration)
        for context in sorted(fibers):
            follow_set = effective_follow_set(declaration, context)
            for membership in range(1, 8):
                possible = tuple(
                    universe - follow_set
                    if membership & (1 << seat)
                    else universe
                    for seat in SEATS
                )
                raw_generators_checked += 1
                marginal = marginal_holder_sets(universe, possible, capacities)
                if marginal == target_marginal:
                    matches.append((declaration, context, membership))
                    matched_lead_fibers.append(fibers[context])

    assert no_void_matches == 0
    assert matches == [
        (0, CALLED, 0b001),
        (NO_TRUMP, 0, 0b001),
    ]
    assert matched_lead_fibers[0] == sigma_zero
    assert matched_lead_fibers[1] == frozenset({(0, 0)})
    assert all(fiber.issubset(universe) for fiber in matched_lead_fibers)
    assert raw_generators_checked == 9 * (1 + 7 * 7)

    return {
        "universe_size": len(universe),
        "raw_generators_checked": raw_generators_checked,
        "matches": tuple(matches),
        "lead_fiber_sizes": tuple(len(fiber) for fiber in matched_lead_fibers),
    }


def main() -> None:
    tiny = check_tiny_support_normal_forms()
    census = check_native_support_census()
    reachability = check_capacity_and_context_reachability()
    schedule_a, schedule_t1, schedule_t2 = exact_schedule_counts()
    bounds = check_reachable_support_bounds()
    witness = check_feasible_unreachable_witness()

    print("Texas 42 support minimality and reachability verification: PASS")
    print(
        "tiny global-normal-form systems: "
        f"{tiny['systems']:,} total; {tiny['feasible']:,} feasible; "
        f"{tiny['scc_witnesses']:,} SCC witness compilations; "
        f"{tiny['exclusions']:,} essential exclusions; "
        f"{tiny['rank_worlds']:,} rank/unrank world checks"
    )
    print(
        "native ternary signatures/matrices: "
        f"{census['signatures']:,} signatures; "
        f"{census['matrices']:,} matrices; "
        f"max {census['max_matrices']:,}/signature"
    )
    print(
        "S3 quotient: "
        f"{census['signature_orbits']:,} signature orbits; "
        f"{census['canonical_matrices']:,} representative matrices; "
        f"{census['matrix_orbits']:,} stabilizer orbits; "
        f"max {census['max_matrix_orbits']:,}/signature"
    )
    print(
        "full native exact-support census: "
        f"empty={census['n_empty']:,}; "
        f"determinate={census['n_det']:,}; "
        f"binary={census['n_bin']:,}; "
        f"ternary={census['n_ter']:,}; "
        f"total={census['support_total']:,}; fixed-width minimum=81 bits"
    )
    print(f"reachable hidden-capacity profiles: {reachability['profiles']}")
    print("observable lead contexts per declaration: 7; lead-fiber sizes: 1..7")
    print(f"projected schedule counts A_j: {schedule_a}")
    print(f"projected schedule counts T_j,1: {schedule_t1}")
    print(f"projected schedule counts T_j,2: {schedule_t2}")
    print(
        "necessary outer reachability profiles: "
        f"{bounds['per_declaration']:,}/declaration; "
        f"{bounds['declaration_tagged']:,} total; "
        "standalone ceiling=46 bits"
    )
    print(
        "context-supplied outer ceilings: "
        "declaration supplied <=43 bits; capacity profile supplied <=43 bits; "
        "both supplied <=40 bits"
    )
    print(
        "universally reachable no-void supports: "
        f"{bounds['lower_family']:,}; standalone floor=26 bits"
    )
    print(
        "feasible-but-unreachable witness: "
        f"{witness['universe_size']} hidden tiles; "
        f"{witness['raw_generators_checked']} static generators checked; "
        f"matching generators={witness['matches']}; "
        f"lead-fiber sizes={witness['lead_fiber_sizes']}; all lead tiles hidden"
    )
    print("proved standalone reachable-support interval: 26..46 bits")
    print("supplemental support/reachability bits relative to certified mechanical state: 0")


if __name__ == "__main__":
    main()
