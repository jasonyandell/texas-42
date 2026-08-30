#!/usr/bin/env python3
"""
Exact-rational companion for DESIGN-walt-counted-belief-sandwich-v0.1.md.

The program checks four independent pieces of the proposed architecture:

1. The max-preserving optimization-lock upper confidence construction on
   every two-policy Boolean table over four worlds and every length-four iid
   world stream, using the exact finite-grid e-process inversion already used
   by Walt's E3 producer.
2. Policy-cylinder, grammar/residual, and strategy-fusion identities on a
   small finite policy class.
3. Seat-factor posterior closure, exact action-bucket contraction, and the
   factorized Bellman equation on a three-hidden-seat six-card game.
4. Counted cell bounds, policy-family threat/safety bounds, and a tiny
   counterexample-guided critical-feature refinement.

All correctness arithmetic is integers or fractions.Fraction. There is no
network, file input, randomness, simulation, or floating point.
"""

from __future__ import annotations

from collections import defaultdict
from fractions import Fraction
from itertools import combinations, product
from math import comb, factorial
import sys


def check(name: str, condition: bool, detail: str = "") -> None:
    if condition:
        print(f"PASS {name}" + (f" {detail}" if detail else ""))
    else:
        print(f"FAIL {name}" + (f" {detail}" if detail else ""))
        raise AssertionError(name)


# ---------------------------------------------------------------------------
# Part 1 — exact e-process inversion and max-preserving root upper
# ---------------------------------------------------------------------------


def e_upper(s: int, f: int, c: Fraction) -> Fraction:
    """E^>_{s,f}(c), exact finite sum, 0 < c < 1."""
    assert 0 < c < 1
    ratio = (1 - c) / c
    total = Fraction(0)
    for i in range(s + 1):
        total += (
            Fraction(comb(s, i))
            * ratio**i
            * Fraction(factorial(i) * factorial(f), factorial(i + f + 1))
        )
    return total


def e_lower(s: int, f: int, c: Fraction) -> Fraction:
    """E^<_{s,f}(c)=E^>_{f,s}(1-c), exact."""
    assert 0 < c < 1
    return e_upper(f, s, 1 - c)


def upper_grid(s: int, n: int, population: int, delta: Fraction) -> Fraction:
    """Anytime upper endpoint on G_N={0,1/N,...,1}."""
    assert 0 <= s <= n
    assert population >= 1
    assert 0 < delta < 1
    threshold = 1 / delta
    candidates = [Fraction(0)]
    for k in range(1, population):
        c = Fraction(k, population)
        if e_lower(s, n - s, c) < threshold:
            candidates.append(c)
    if s == n:
        candidates.append(Fraction(1))
    return max(candidates)


def lower_grid(s: int, n: int, population: int, delta: Fraction) -> Fraction:
    """Anytime lower endpoint by complementing the Bernoulli variable."""
    return 1 - upper_grid(n - s, n, population, delta)


def verify_optimization_upper() -> None:
    population = 4
    horizon = 4
    delta = Fraction(1, 4)
    all_policies = list(product((0, 1), repeat=population))

    tables_checked = 0
    stream_evaluations = 0
    worst_upper_undercoverage = Fraction(0)
    worst_lower_overcoverage = Fraction(0)

    # Upper: every two-policy table, every stream.
    for p0 in all_policies:
        for p1 in all_policies:
            table = (p0, p1)
            q = max(Fraction(sum(p), population) for p in table)
            bad_upper = 0
            for stream in product(range(population), repeat=horizon):
                counts = [0, 0]
                running_upper = Fraction(1)
                violated = False
                for n, w in enumerate(stream, start=1):
                    counts[0] += p0[w]
                    counts[1] += p1[w]
                    s_star = max(counts)
                    running_upper = min(
                        running_upper,
                        upper_grid(s_star, n, population, delta),
                    )
                    if q > running_upper:
                        violated = True
                if violated:
                    bad_upper += 1
                stream_evaluations += 1
            prob = Fraction(bad_upper, population**horizon)
            worst_upper_undercoverage = max(worst_upper_undercoverage, prob)
            if prob > delta:
                raise AssertionError(("upper undercoverage", table, q, prob))
            tables_checked += 1

    # Lower: every fixed policy, every stream. This is deliberately fixed,
    # not selected on the evaluation stream.
    for p in all_policies:
        v = Fraction(sum(p), population)
        bad_lower = 0
        for stream in product(range(population), repeat=horizon):
            s = 0
            running_lower = Fraction(0)
            violated = False
            for n, w in enumerate(stream, start=1):
                s += p[w]
                running_lower = max(
                    running_lower,
                    lower_grid(s, n, population, delta),
                )
                if running_lower > v:
                    violated = True
            if violated:
                bad_lower += 1
        prob = Fraction(bad_lower, population**horizon)
        worst_lower_overcoverage = max(worst_lower_overcoverage, prob)
        if prob > delta:
            raise AssertionError(("lower overcoverage", p, v, prob))

    check(
        "optimization-lock max-preserving upper",
        worst_upper_undercoverage <= delta,
        f"tables={tables_checked} streams={stream_evaluations} "
        f"worst={worst_upper_undercoverage} delta={delta}",
    )
    check(
        "fixed-policy lower endpoint",
        worst_lower_overcoverage <= delta,
        f"policies={len(all_policies)} worst={worst_lower_overcoverage} delta={delta}",
    )

    # Strict pathwise relation to a per-world fused upper relaxation.
    table = ((1, 1, 0, 0), (0, 0, 1, 1))
    stream = (0, 1, 2, 3)
    counts = [0, 0]
    fused = 0
    e3 = Fraction(1)
    e2 = Fraction(1)
    for n, w in enumerate(stream, start=1):
        vals = [p[w] for p in table]
        for i, val in enumerate(vals):
            counts[i] += val
        fused += max(vals)
        e3 = min(e3, upper_grid(max(counts), n, population, delta))
        e2 = min(e2, upper_grid(fused, n, population, delta))
        assert e3 <= e2
    check(
        "optimization upper strict specimen",
        e3 == Fraction(3, 4) and e2 == 1,
        f"Q=1/2 E3={e3} fused={e2}",
    )


# ---------------------------------------------------------------------------
# Part 2 — policy cylinders and a two-policy grammar
# ---------------------------------------------------------------------------


def policy_utility(policy: tuple[int, int], world: int) -> int:
    """
    Two focal information states I0/I1, each with actions 0,1,2.

    Worlds 0/1 score the I0 choice, worlds 2/3 score the I1 choice.
    Action 2 is a genuine residual action and scores neither member of its
    coordinate pair.
    """
    a0, a1 = policy
    if world == 0:
        return int(a0 == 0)
    if world == 1:
        return int(a0 == 1)
    if world == 2:
        return int(a1 == 0)
    if world == 3:
        return int(a1 == 1)
    raise AssertionError(world)


def policy_value(policy: tuple[int, int]) -> Fraction:
    return Fraction(sum(policy_utility(policy, w) for w in range(4)), 4)


def verify_policy_regions() -> None:
    policies = list(product(range(3), repeat=2))
    q_full = max(policy_value(p) for p in policies)

    seed_low = (0, 0)
    seed_high = (1, 1)
    grammar_actions = ({0, 1}, {0, 1})
    grammar = [
        p
        for p in policies
        if p[0] in grammar_actions[0] and p[1] in grammar_actions[1]
    ]
    residual = [p for p in policies if p not in grammar]
    q_grammar = max(policy_value(p) for p in grammar)
    q_residual = max(policy_value(p) for p in residual)

    check(
        "policy grammar restricted optimum",
        q_full == Fraction(1, 2)
        and q_grammar == Fraction(1, 2)
        and q_residual == Fraction(1, 4),
        f"Q={q_full} QG={q_grammar} QR={q_residual}",
    )
    check(
        "grammar residual separation",
        q_grammar > q_residual,
        "exact grammar optimizer is globally optimal",
    )

    # The world-wise union of the two seed policies is a clairvoyant value 1,
    # while no lawful policy exceeds 1/2.
    fused = Fraction(
        sum(
            max(policy_utility(seed_low, w), policy_utility(seed_high, w))
            for w in range(4)
        ),
        4,
    )
    check(
        "policy pointwise fusion is not lawful coverage",
        fused == 1 and q_full == Fraction(1, 2),
        f"fused={fused} lawful={q_full}",
    )

    # Cylinder split at I0: parent is all policies; children fix action I0.
    child_values = {}
    child_sets = []
    for action in range(3):
        child = [p for p in policies if p[0] == action]
        child_sets.append(set(child))
        child_values[action] = max(policy_value(p) for p in child)
    union = set().union(*child_sets)
    disjoint = sum(len(s) for s in child_sets) == len(union)
    check(
        "policy cylinder partition",
        union == set(policies) and disjoint and q_full == max(child_values.values()),
        f"children={child_values}",
    )

    # Canonical first-deviation partition of residual: first coordinate where
    # action 2 appears.
    dev0 = {p for p in residual if p[0] == 2}
    dev1 = {p for p in residual if p[0] != 2 and p[1] == 2}
    check(
        "first-deviation residual partition",
        dev0.isdisjoint(dev1) and dev0 | dev1 == set(residual),
        f"dev0={len(dev0)} dev1={len(dev1)}",
    )


# ---------------------------------------------------------------------------
# Part 3 — exact seat-factor belief and factorized Bellman recursion
# ---------------------------------------------------------------------------


def all_deals(universe: frozenset[int]) -> list[tuple[frozenset[int], ...]]:
    """Three hidden seats, two cards each, disjoint cover of six cards."""
    out = []
    ordered = sorted(universe)
    for a_tuple in combinations(ordered, 2):
        a = frozenset(a_tuple)
        rem1 = universe - a
        for b_tuple in combinations(sorted(rem1), 2):
            b = frozenset(b_tuple)
            c = rem1 - b
            out.append((a, b, c))
    return out


def all_hands(universe: frozenset[int]) -> list[frozenset[int]]:
    return [frozenset(x) for x in combinations(sorted(universe), 2)]


def factor_a(hand: frozenset[int]) -> Fraction:
    return Fraction(2 if 0 in hand else 1)


def factor_b(hand: frozenset[int]) -> Fraction:
    return Fraction(3 if 1 in hand else 1)


def factor_c(hand: frozenset[int]) -> Fraction:
    return Fraction(2 if 5 in hand else 1)


def deal_weight(deal: tuple[frozenset[int], ...]) -> Fraction:
    a, b, c = deal
    return factor_a(a) * factor_b(b) * factor_c(c)


def field_a_action(hand: frozenset[int]) -> str:
    return "show-0" if 0 in hand else "other"


def completion_weight_for_a(
    a: frozenset[int], universe: frozenset[int]
) -> Fraction:
    total = Fraction(0)
    rem = universe - a
    for b_tuple in combinations(sorted(rem), 2):
        b = frozenset(b_tuple)
        c = rem - b
        total += factor_b(b) * factor_c(c)
    return total


def verify_factor_belief() -> None:
    universe = frozenset(range(6))
    deals = all_deals(universe)
    hands = all_hands(universe)
    check("factor belief deal count", len(deals) == 90, f"deals={len(deals)}")
    check("factor belief hand count", len(hands) == 15, f"hands={len(hands)}")

    z_explicit = sum((deal_weight(d) for d in deals), Fraction(0))
    z_hand = sum(
        (factor_a(a) * completion_weight_for_a(a, universe) for a in hands),
        Fraction(0),
    )
    check(
        "exact-cover partition function",
        z_explicit == z_hand,
        f"Z={z_explicit}",
    )

    # Branch masses by acting-hand contraction.
    bucket_hand: dict[str, Fraction] = defaultdict(Fraction)
    for a in hands:
        bucket_hand[field_a_action(a)] += (
            factor_a(a) * completion_weight_for_a(a, universe)
        )

    bucket_explicit: dict[str, Fraction] = defaultdict(Fraction)
    for deal in deals:
        bucket_explicit[field_a_action(deal[0])] += deal_weight(deal)

    check(
        "factorized action buckets equal world enumeration",
        dict(bucket_hand) == dict(bucket_explicit)
        and sum(bucket_hand.values(), Fraction(0)) == z_explicit,
        f"buckets={dict(bucket_hand)}",
    )

    # Posterior closure after observing show-0: multiplying A's local factor
    # by the action indicator must equal explicit Bayesian filtering.
    observed = "show-0"
    explicit_weights = {
        deal: (deal_weight(deal) if field_a_action(deal[0]) == observed else Fraction(0))
        for deal in deals
    }
    factorized_weights = {
        deal: (
            factor_a(deal[0])
            * int(field_a_action(deal[0]) == observed)
            * factor_b(deal[1])
            * factor_c(deal[2])
        )
        for deal in deals
    }
    check(
        "seat-factor posterior closure",
        explicit_weights == factorized_weights,
        "only actor A factor changed",
    )

    # Tiny exact Bellman game:
    # A announces show-0/other. Focal then guesses whether card 1 is in B.
    # The optimal action is one common guess per public announcement.
    branch_target: dict[str, list[Fraction]] = {
        "show-0": [Fraction(0), Fraction(0)],
        "other": [Fraction(0), Fraction(0)],
    }
    for deal in deals:
        action = field_a_action(deal[0])
        target = int(1 in deal[1])
        branch_target[action][target] += deal_weight(deal)

    bellman_numerator = sum(max(masses) for masses in branch_target.values())
    bellman_value = bellman_numerator / z_explicit

    # Explicitly enumerate the four lawful observation policies:
    # guess after show-0, guess after other.
    policy_values = []
    for guess_show, guess_other in product((0, 1), repeat=2):
        success = Fraction(0)
        for deal in deals:
            action = field_a_action(deal[0])
            guess = guess_show if action == "show-0" else guess_other
            target = int(1 in deal[1])
            if guess == target:
                success += deal_weight(deal)
        policy_values.append(success / z_explicit)
    explicit_best = max(policy_values)

    check(
        "factorized Bellman equals exact policy optimization",
        bellman_value == explicit_best,
        f"value={bellman_value} branch-masses={branch_target}",
    )


# ---------------------------------------------------------------------------
# Part 4 — counted cells, family threat/safety, and CEGAR
# ---------------------------------------------------------------------------


def verify_cell_calculus() -> None:
    # Pair outcomes on six equally weighted worlds.
    a = (1, 1, 0, 1, 0, 1)
    b = (1, 0, 1, 1, 0, 0)
    y = [x - z for x, z in zip(a, b)]
    exact_gap = Fraction(sum(y), len(y))

    # Coarse cells: exact equal on {0,3,4}; unknown on {1,2,5}.
    equal = {0, 3, 4}
    unknown = {1, 2, 5}
    lower0 = Fraction(-len(unknown), len(y))
    upper0 = Fraction(len(unknown), len(y))
    check(
        "coarse counted cell interval",
        lower0 <= exact_gap <= upper0,
        f"gap={exact_gap} interval=[{lower0},{upper0}]",
    )

    # Refine unknown into exact benefit/hazard cells.
    benefit = {i for i in unknown if y[i] == 1}
    hazard = {i for i in unknown if y[i] == -1}
    neutral = {i for i in unknown if y[i] == 0}
    lower1 = upper1 = Fraction(len(benefit) - len(hazard), len(y))
    check(
        "cell refinement narrows monotonically",
        lower0 <= lower1 == exact_gap == upper1 <= upper0
        and benefit | hazard | neutral == unknown,
        f"benefit={benefit} hazard={hazard} neutral={neutral}",
    )

    # Policy-family threat/safety bound.
    incumbent = (1, 1, 1, 1, 0)
    q1 = (1, 0, 1, 0, 0)
    q2 = (1, 1, 0, 0, 0)
    family = (q1, q2)
    threat = []
    safety = []
    for w in range(len(incumbent)):
        threat.append(int(incumbent[w] == 0 and any(q[w] == 1 for q in family)))
        safety.append(int(incumbent[w] == 1 and all(q[w] == 0 for q in family)))
    v_inc = Fraction(sum(incumbent), len(incumbent))
    q_family = max(Fraction(sum(q), len(q)) for q in family)
    rhs = Fraction(sum(threat) - sum(safety), len(incumbent))
    check(
        "policy-family threat safety upper",
        q_family - v_inc <= rhs and rhs < 0,
        f"actual={q_family-v_inc} upper={rhs}",
    )


def hand_action(hand: frozenset[int]) -> str:
    if 0 in hand:
        return "TRUMP"
    if sum(1 for x in hand if x % 2 == 0) >= 2:
        return "EVEN"
    return "OTHER"


def feature_signature(hand: frozenset[int], features: tuple[str, ...]) -> tuple:
    out = []
    for f in features:
        if f == "has0":
            out.append(0 in hand)
        elif f == "even_count":
            out.append(sum(1 for x in hand if x % 2 == 0))
        else:
            raise AssertionError(f)
    return tuple(out)


def classes_uniform(hands: list[frozenset[int]], features: tuple[str, ...]) -> bool:
    classes: dict[tuple, set[str]] = defaultdict(set)
    for hand in hands:
        classes[feature_signature(hand, features)].add(hand_action(hand))
    return all(len(actions) == 1 for actions in classes.values())


def verify_cegar() -> None:
    hands = all_hands(frozenset(range(6)))
    check(
        "cegar coarse class is unresolved",
        not classes_uniform(hands, ()),
    )
    check(
        "cegar first critical tile insufficient",
        not classes_uniform(hands, ("has0",)),
    )
    check(
        "cegar witnessed refinement reaches exact action classes",
        classes_uniform(hands, ("has0", "even_count")),
        "15 hands compressed by two contextual features",
    )


def main() -> int:
    verify_optimization_upper()
    verify_policy_regions()
    verify_factor_belief()
    verify_cell_calculus()
    verify_cegar()
    print("ALL CHECKS PASS")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as exc:  # deterministic failure signal with detail
        print(f"ERROR {type(exc).__name__}: {exc}")
        raise
