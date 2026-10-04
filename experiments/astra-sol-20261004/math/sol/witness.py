#!/usr/bin/env python3
"""Exact finite countermodels. No Texas 42 playing-strength claim."""
import json
from fractions import Fraction
from functools import lru_cache


def amplification(n):
    denom = 2 * n + 1
    exact = [Fraction(n + 1, denom), Fraction(n, denom)]
    approximate = list(reversed(exact))
    choose = lambda xs: max(range(2), key=lambda a: (xs[a], -a))
    old, new = choose(exact), choose(approximate)
    error = max(abs(a - b) for a, b in zip(exact, approximate))
    modeled_regret = exact[old] - exact[new]
    # Host knows hidden type 1; field's constant policy never receives it.
    host_success = lambda field_action: int(field_action != 1)
    assert old == 0 and new == 1
    assert error == modeled_regret == Fraction(1, denom)
    assert host_success(old) - host_success(new) == 1
    assert error < Fraction(1, n)
    return dict(n=n, denominator=denom, exact_values=list(map(str, exact)),
                approximate_values=list(map(str, approximate)),
                max_action_error=str(error), modeled_policy_regret=str(modeled_regret),
                exact_choice=old, approximate_choice=new, conditional_host_drop=1)


def unique_demands(k):
    keys = set()

    @lru_cache(maxsize=None)
    def query(remaining_level, complete_branch_path):
        key = remaining_level, complete_branch_path
        assert key not in keys
        keys.add(key)
        if remaining_level == 0:
            return sum(complete_branch_path) % 2
        return (query(remaining_level - 1, complete_branch_path + (0,))
                + query(remaining_level - 1, complete_branch_path + (1,)))

    value = query(k, ())
    # Repeat the exact query: a complete-key memo avoids all repeated work.
    assert query(k, ()) == value
    leaves = sum(level == 0 for level, _path in keys)
    assert leaves == 2 ** k
    assert len(keys) == 2 ** (k + 1) - 1
    return dict(k=k, unique_queries=len(keys), distinct_leaves=leaves,
                repeated_root_cache_hits=query.cache_info().hits)


def strategic_cycle():
    # Stationary unique best response in rock-paper-scissors.
    best_response = lambda opponent: (opponent + 1) % 3
    field = 0  # Real opponent remains rock.
    player = best_response(field)
    entries = []
    for k in range(8):
        success_against_rock = int(player == 1)
        entries.append(dict(k=k, action=player, success_against_fixed_rock=success_against_rock))
        player = best_response(player)
    assert entries[0]['success_against_fixed_rock'] == 1
    assert entries[1]['success_against_fixed_rock'] == 0
    assert [row['action'] for row in entries[:6]] == [1, 2, 0, 1, 2, 0]
    return entries


def incumbent_only_error_is_insufficient():
    # Same two equally weighted scenarios. Fields disagree only on histories
    # reached by action B. The incumbent A's replay therefore has no disagreement.
    old = {"A": [True, False], "B": [False, False]}
    new = {"A": [True, False], "B": [True, True]}
    values = lambda table: {a: sum(payoffs) for a, payoffs in table.items()}
    old_values, new_values = values(old), values(new)
    incumbent_error = sum(a != b for a, b in zip(old["A"], new["A"]))
    assert incumbent_error == 0
    assert old_values["A"] > old_values["B"]
    assert new_values["B"] > new_values["A"]
    return dict(old_counts=old_values, new_counts=new_values,
                incumbent_disagreement_mass=incumbent_error,
                competitor_disagreement_mass=2,
                invalid_claim="incumbent replay error bounds every root action")


print(json.dumps(dict(
    scope="exploratory abstract finite-model counterexamples; not Texas 42 measurements",
    amplification=[amplification(n) for n in [1, 2, 10, 1000, 1000000000]],
    unique_demand_tree=[unique_demands(k) for k in range(13)],
    stationary_best_response_cycle=strategic_cycle(),
    incumbent_only_error_counterexample=incumbent_only_error_is_insufficient(),
), indent=2))
