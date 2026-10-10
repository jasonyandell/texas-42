"""Compiled public-history L1 frontier; same float/RNG order as pure-v2.

Deals are fibers, never independent maximization states. Child identity is
exactly (parent public node, played tile). No fastmath.
"""
import numpy as np
from numba import njit
from compiled_edges import expand
from nested_kernels import prepare


@njit(cache=True)
def group_edges(keys, parent_count):
    """Counting scatter replaces unique/sort; preserves fiber input order."""
    mapping = np.full(parent_count * 28, -1, np.int64)
    for key in keys:
        mapping[key] = 0
    count = 0
    for key in range(len(mapping)):
        if mapping[key] == 0:
            mapping[key] = count
            count += 1
    unique = np.empty(count, np.int64)
    for key in range(len(mapping)):
        if mapping[key] >= 0:
            unique[mapping[key]] = key
    inverse = np.empty(len(keys), np.int64)
    for i in range(len(keys)):
        inverse[i] = mapping[keys[i]]
    return unique, inverse


@njit(cache=True)
def play_nodes(state, keys, lead, strength, points):
    out = np.empty((len(keys), 9), np.int64)
    for i in range(len(keys)):
        parent, tile = keys[i] // 28, keys[i] % 28
        out[i] = state[parent]
        out[i, 0] |= 1 << tile
        tlen = out[i, 2]
        out[i, 5 + tlen] = tile
        if tlen == 3:
            suit = lead[out[i, 5]]
            best, winner, won = -1, 0, 1
            for pos in range(4):
                t = out[i, 5 + pos]
                rank = strength[suit, t]
                if rank > best:
                    best, winner = rank, (out[i, 1] + pos) % 4
                won += points[t]
            out[i, 3 if winner % 2 else 4] += won
            out[i, 1], out[i, 2] = winner, 0
            out[i, 5:9] = 0
        else:
            out[i, 2] += 1
    return out


@njit(cache=True)
def solve_one(state, deals, rng, suit, lead, strength, points, delta, cap):
    me = (state[0, 1] + state[0, 2]) % 4
    tn = np.zeros(len(deals), np.int64)
    td = np.arange(len(deals))
    tw = np.ones(len(deals))
    parent = np.empty(0, np.int64)
    parents, mine_layers, leaves = [], [], []
    root_tiles = np.empty(0, np.int64)
    visits, peak = 0, 0
    for depth in range(29):
        if len(tn) > cap:
            return root_tiles, np.empty(0), visits, peak, False
        visits += len(tn)
        peak = max(peak, len(tn))
        count = len(state)
        term = (state[:, 3] >= 30) | (state[:, 4] > 12)
        leaf = np.zeros(count)
        for i in range(len(tn)):
            if state[tn[i], 3] >= 30:
                leaf[tn[i]] += tw[i]
        parents.append(parent)
        mine_layers.append((state[:, 1] + state[:, 2]) % 4 == me)
        leaves.append(leaf)
        if np.all(term):
            break
        keep = ~term[tn]
        tn, td, tw = tn[keep], td[keep], tw[keep]
        masks, counts, mine, small = prepare(tn, td, tw, deals,
            state[:, 0], state[:, 1], state[:, 5:9], state[:, 2],
            suit, lead, np.full(count, me), delta)
        uniform = rng.random(np.sum(small))
        keys, td, tw = expand(tn, td, tw, masks, counts, mine, small, uniform)
        if len(keys) > cap:
            return root_tiles, np.empty(0), visits, peak, False
        keys, tn = group_edges(keys, count)
        parent = keys // 28
        if depth == 0:
            root_tiles = keys % 28
        state = play_nodes(state, keys, lead, strength, points)
    values = leaves[-1]
    for depth in range(len(leaves) - 1, 1, -1):
        parent, mine = parents[depth], mine_layers[depth - 1]
        out = leaves[depth - 1].copy()
        previous = -1
        for i in range(len(parent)):
            p = parent[i]
            if p != previous:
                out[p] = values[i]
            elif mine[p]:
                out[p] = max(out[p], values[i]) if me % 2 else min(out[p], values[i])
            else:
                out[p] += values[i]
            previous = p
        values = out
    return root_tiles, values, visits, peak, True


@njit(cache=True)
def solve_batch(state, deals, rngs, suit, lead, strength, points, delta, cap):
    roots = len(state)
    me = (state[:, 1] + state[:, 2]) % 4
    tn = np.repeat(np.arange(roots), deals.shape[1])
    flat_deals = deals.reshape((-1, 4))
    td = np.arange(len(flat_deals))
    tw = np.ones(len(flat_deals))
    owner = np.arange(roots)
    parent = np.empty(0, np.int64)
    parents = [np.empty(0, np.int64) for _ in range(29)]
    mine_layers = [np.empty(0, np.bool_) for _ in range(29)]
    odd_layers = [np.empty(0, np.bool_) for _ in range(29)]
    leaves = [np.empty(0, np.float64) for _ in range(29)]
    root_keys = np.empty(0, np.int64)
    scores = np.full((roots, 28), np.nan)
    visits, peak = 0, 0
    for depth in range(29):
        if len(tn) > cap:
            return scores, visits, peak, False
        visits += len(tn)
        peak = max(peak, len(tn))
        count = len(state)
        term = (state[:, 3] >= 30) | (state[:, 4] > 12)
        leaf = np.zeros(count)
        for i in range(len(tn)):
            if state[tn[i], 3] >= 30:
                leaf[tn[i]] += tw[i]
        parents[depth] = parent
        mine_layers[depth] = (state[:, 1] + state[:, 2]) % 4 == me[owner]
        odd_layers[depth] = me[owner] % 2 == 1
        leaves[depth] = leaf
        if np.all(term):
            break
        keep = ~term[tn]
        tn, td, tw = tn[keep], td[keep], tw[keep]
        masks, counts, mine, small = prepare(tn, td, tw, flat_deals,
            state[:, 0], state[:, 1], state[:, 5:9], state[:, 2],
            suit, lead, me[owner], delta)
        uniform = np.empty(np.sum(small))
        at = 0
        for i in range(len(tn)):
            if small[i]:
                uniform[at] = rngs[owner[tn[i]]].random()
                at += 1
        keys, td, tw = expand(tn, td, tw, masks, counts, mine, small, uniform)
        if len(keys) > cap:
            return scores, visits, peak, False
        keys, tn = group_edges(keys, count)
        parent = keys // 28
        if depth == 0:
            root_keys = keys
        state = play_nodes(state, keys, lead, strength, points)
        owner = owner[parent]
    values = leaves[depth]
    for depth in range(depth, 1, -1):
        parent = parents[depth]
        mine, odd = mine_layers[depth - 1], odd_layers[depth - 1]
        out = leaves[depth - 1].copy()
        previous = -1
        for i in range(len(parent)):
            p = parent[i]
            if p != previous:
                out[p] = values[i]
            elif mine[p]:
                out[p] = max(out[p], values[i]) if odd[p] else min(out[p], values[i])
            else:
                out[p] += values[i]
            previous = p
        values = out
    for i in range(len(root_keys)):
        scores[root_keys[i] // 28, root_keys[i] % 28] = values[i]
    return scores, visits, peak, True


def solve(field, node, deal_batches, rngs):
    state = np.column_stack((node.played, node.leader, node.tlen,
                             node.t1, node.t0, node.trick))
    points = np.array([h+l if h+l in (5, 10) else 0
                       for h in range(7) for l in range(h+1)], np.int64)
    scores, visits, peak, complete = solve_batch(state, np.asarray(deal_batches), rngs,
        field.rules.suit, field.rules.lead, field.rules.strength, points,
        field.spec.delta, field.fiber_cap)
    field.check()
    field.stats['L1_fiber_visits'] += visits
    field.stats['L1_peak_fibers'] = max(field.stats['L1_peak_fibers'], peak)
    if not complete:
        from repaired import FrontierLimit
        raise FrontierLimit('compiled L1 frontier fiber cap')
    return [{t: v for t, v in enumerate(row) if v == v} for row in scores.tolist()]
