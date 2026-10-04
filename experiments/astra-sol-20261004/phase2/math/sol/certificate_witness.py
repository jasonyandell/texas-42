#!/usr/bin/env python3
"""Exact finite games validating all-policy first-disagreement bounds.

One fixed root action; field, focal, field, then a Boolean terminal payoff.
Focal policies see complete public history, never world identity. Field private
types may differ across worlds. Repeated original scenarios preserve mass.
"""
from functools import lru_cache
from itertools import product
import json
import random

SCENARIOS = [(0, 3), (1, 1), (2, 2), (0, 1)]
MASS = sum(weight for _, weight in SCENARIOS)
PUBLIC_FOCAL_NODES = list(product(range(2), repeat=2))
POLICIES = [dict(zip(PUBLIC_FOCAL_NODES, actions)) for actions in product(range(2), repeat=4)]
FIELD_NODES = list(product(range(2), repeat=1)) + list(product(range(2), repeat=3))


def payoff(world, history):
    assert len(history) == 4
    return bool((history[0] + history[2] + 2 * history[3] + world) % 3 == 0)


def replay(world, root, field, policy):
    h = (root,)
    while len(h) < 4:
        action = policy[h] if len(h) == 2 else field[world, h]
        assert action in (0, 1)
        h += (action,)
    return h, payoff(world, h)


def make_certificate(cheap, target, refused=frozenset(), focal_actions=(0, 1)):
    stats = dict(target_queries=0, refused_queries=0)
    @lru_cache(maxsize=None)
    def certified(world, h):
        if len(h) == 4:
            return True
        if len(h) == 2:
            return all(certified(world, h + (a,)) for a in focal_actions)
        stats["target_queries"] += 1
        if (world, h) in refused:
            stats["refused_queries"] += 1
            return False  # conservative bad-world flag, never an unflagged result
        return cheap[world, h] == target[world, h] and certified(world, h + (cheap[world, h],))
    return certified, stats


def random_field(rng):
    return {(world, h): rng.randrange(2) for world in range(3) for h in FIELD_NODES}


cases = 0
checked_policy_root_pairs = 0
transport_acceptances = 0
query_totals = []
rng = random.Random(20261004)
for i in range(256):
    cheap = random_field(rng)
    target = {key: (value ^ (rng.randrange(8) == 0)) for key, value in cheap.items()}
    refused = frozenset(key for key in cheap if i % 5 == 0 and rng.randrange(20) == 0)
    certified, stats = make_certificate(cheap, target, refused)
    cheap_max, target_max, eps = {}, {}, {}
    for root in (0, 1):
        flags = {world: not certified(world, (root,)) for world, _ in SCENARIOS}
        eps[root] = sum(weight for world, weight in SCENARIOS if flags[world])
        cheap_values, target_values = [], []
        for policy in POLICIES:
            old = new = 0
            for world, weight in SCENARIOS:
                old_trace, old_payoff = replay(world, root, cheap, policy)
                new_trace, new_payoff = replay(world, root, target, policy)
                if not flags[world]:
                    assert old_trace == new_trace and old_payoff == new_payoff
                old += weight * old_payoff; new += weight * new_payoff
            assert abs(old - new) <= eps[root]
            cheap_values.append(old); target_values.append(new)
            checked_policy_root_pairs += 1
        cheap_max[root], target_max[root] = max(cheap_values), max(target_values)
        assert abs(cheap_max[root] - target_max[root]) <= eps[root]
    chosen = max((0, 1), key=lambda a: (cheap_max[a], -a))
    lower = max(0, cheap_max[chosen] - eps[chosen])
    accepted = all((min(MASS, cheap_max[b] + eps[b]) < lower if b < chosen
                    else min(MASS, cheap_max[b] + eps[b]) <= lower)
                   for b in (0, 1) if b != chosen)
    if accepted:
        actual = max((0, 1), key=lambda a: (target_max[a], -a))
        assert actual == chosen
        transport_acceptances += 1
    cases += 1
    query_totals.append(stats["target_queries"])

# Pinned failure: searching only focal action 0 misses the differing action 1.
cheap = {(world, h): 0 for world in range(3) for h in FIELD_NODES}
target = dict(cheap)
target[0, (0, 0, 1)] = 1
partial, _ = make_certificate(cheap, target, focal_actions=(0,))
full, _ = make_certificate(cheap, target)
assert partial(0, (0,)) and not full(0, (0,))
policy = {h: 1 for h in PUBLIC_FOCAL_NODES}
old_trace, old_payoff = replay(0, 0, cheap, policy)
new_trace, new_payoff = replay(0, 0, target, policy)
assert old_trace != new_trace and old_payoff != new_payoff

# Pinned canonical tie: equality cannot exclude a lower-index competitor.
old, eps = {0: 2, 1: 3}, {0: 0, 1: 1}
target = {0: 2, 1: 2}
assert old[1] - eps[1] >= old[0] + eps[0]
assert max(target, key=lambda a: (target[a], -a)) == 0
assert not old[0] + eps[0] < old[1] - eps[1]

print(json.dumps(dict(cases=cases, distinct_lawful_public_policies=len(POLICIES),
    checked_policy_root_pairs=checked_policy_root_pairs,
    scenario_mass=MASS, original_scenarios=len(SCENARIOS), physical_worlds=3,
    coupled_unflagged_replay_false_positives=0,
    canonical_transport_acceptances=transport_acceptances,
    canonical_transport_false_positives=0,
    refused_queries_count_as_bad=True, incomplete_focal_branch_counterexample=True,
    lower_index_equality_counterexample=True,
    max_certificate_target_queries=max(query_totals),
    scope="abstract exact finite-game correctness checks; not Walt speed or strength"), indent=2))
