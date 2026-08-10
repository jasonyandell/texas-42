#!/usr/bin/env python3
"""
exp5_core.py -- Experiment 5 kernel construction, exact fiber counting/sampling,
and the fast bitmask perfect-information minimax used for the census.

Exploratory probe tier.  Stdlib only.  Exact integers / Fractions; NO FLOATS
anywhere near a value, a rank, or a probability.  `random` is used only to
SELECT sample worlds (seeded, recorded); every class count computed on a
selected world is exact.

Declaration-relative rules come from exp5_rules.py, which is a frozen copy of
rules42.py (generalized from lambda_probe.py's TRUMP=3 hardcode) taken at the
start of this run so that concurrent edits by another probe cannot perturb it.
Its replay_validate_all() re-derives every trick of all 13 receipt hands and is
the evidence that the generalization matches the rob engine.
"""

from __future__ import annotations

import os
import random
import sys
from fractions import Fraction
from itertools import combinations
from math import comb, factorial

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import exp5_rules as R
import exp5_pwl as P

F = Fraction

TRUMP_ID = 7                      # local integer id for the trump suit


# ------------------------------------------------------------------ kernels

class Kernel:
    __slots__ = ("kid", "hand_id", "decl", "trick_no", "horizon", "focal",
                 "focal_team", "focal_hand", "unseen", "voids", "sizes",
                 "live", "true_world", "fiber_unconstrained", "fiber_size",
                 "hidden_seats")

    def __repr__(self):
        return (f"Kernel({self.kid} h{self.hand_id} {self.decl.label} "
                f"H={self.horizon} focal S{self.focal} |X|={self.fiber_size})")


def unconstrained_fiber_size(k):
    """|assignments of 3k tiles into three labelled seats of k| ."""
    return factorial(3 * k) // (factorial(k) ** 3)


def _allowed_mask(tile, seat, banned, dc):
    """True iff `tile` may sit in `seat` given that seat's observed voids."""
    for suit in banned[seat]:
        if R.in_suit(tile, suit, dc):
            return False
    return True


def _pattern_groups(unseen, seats, voids, dc):
    """Group unseen tiles by which hidden seats may legally hold them.

    Returns [(pattern_bits, [tiles])] where bit i of pattern_bits corresponds to
    seats[i].  Tiles with pattern 0 make the fiber empty.
    """
    banned = {s: {suit for (ss, suit) in voids if ss == s} for s in seats}
    groups = {}
    for t in unseen:
        pat = 0
        for i, s in enumerate(seats):
            if _allowed_mask(t, s, banned, dc):
                pat |= 1 << i
        groups.setdefault(pat, []).append(t)
    return sorted(groups.items())


def _split_counts(n, allowed_idx):
    """All (c0,c1,c2) with sum n and c_i = 0 for i not in allowed_idx."""
    out = []
    for c0 in (range(n + 1) if 0 in allowed_idx else (0,)):
        for c1 in (range(n - c0 + 1) if 1 in allowed_idx else (0,)):
            c2 = n - c0 - c1
            if c2 < 0:
                continue
            if c2 and 2 not in allowed_idx:
                continue
            out.append((c0, c1, c2))
    return out


def fiber_count_and_dp(unseen, sizes, voids, dc):
    """Exact size of the void-constrained fiber, plus the DP table that makes
    exact uniform sampling possible.  No enumeration: a DP over the >=1 and <=7
    'allowed-seat pattern' groups.

    Returns (total, groups, dp) where dp[i] maps (a0,a1,a2) -> integer count of
    ways to fill using groups[i:].
    """
    seats = sorted(sizes)
    k = [sizes[s] for s in seats]
    groups = _pattern_groups(unseen, seats, voids, dc)

    # dp from the last group backwards
    dp = [None] * (len(groups) + 1)
    dp[len(groups)] = {(0, 0, 0): 1}
    for i in range(len(groups) - 1, -1, -1):
        pat, tiles = groups[i]
        n = len(tiles)
        allowed = [j for j in range(3) if pat >> j & 1]
        splits = _split_counts(n, allowed)
        cur = {}
        nxt = dp[i + 1]
        for (c0, c1, c2) in splits:
            w = factorial(n) // (factorial(c0) * factorial(c1) * factorial(c2))
            for (a0, a1, a2), cnt in nxt.items():
                b = (a0 + c0, a1 + c1, a2 + c2)
                if b[0] > k[0] or b[1] > k[1] or b[2] > k[2]:
                    continue
                cur[b] = cur.get(b, 0) + w * cnt
        dp[i] = cur
    total = dp[0].get((k[0], k[1], k[2]), 0)
    return total, groups, dp


def fiber_sample(unseen, sizes, voids, dc, rng, dp_pack=None):
    """One exactly-uniform draw from the void-constrained fiber.

    Uniformity: the DP weights are exact integers counting completions, and the
    per-group split is drawn with probability weight/total using an integer
    randrange -- no floating point enters the probability.
    """
    seats = sorted(sizes)
    k = [sizes[s] for s in seats]
    if dp_pack is None:
        dp_pack = fiber_count_and_dp(unseen, sizes, voids, dc)
    total, groups, dp = dp_pack
    assert total > 0
    a = (k[0], k[1], k[2])          # remaining capacity
    out = [[] for _ in range(3)]
    for i, (pat, tiles) in enumerate(groups):
        n = len(tiles)
        allowed = [j for j in range(3) if pat >> j & 1]
        splits = _split_counts(n, allowed)
        nxt = dp[i + 1]
        opts, weights, tot = [], [], 0
        for (c0, c1, c2) in splits:
            if c0 > a[0] or c1 > a[1] or c2 > a[2]:
                continue
            rem = (a[0] - c0, a[1] - c1, a[2] - c2)
            cnt = nxt.get(rem, 0)
            if not cnt:
                continue
            w = (factorial(n) // (factorial(c0) * factorial(c1) * factorial(c2))) * cnt
            opts.append((c0, c1, c2))
            weights.append(w)
            tot += w
        assert tot > 0
        r = rng.randrange(tot)
        acc = 0
        for o, w in zip(opts, weights):
            acc += w
            if r < acc:
                pick = o
                break
        pool = list(tiles)
        rng.shuffle(pool)
        p = 0
        for j in range(3):
            for _ in range(pick[j]):
                out[j].append(pool[p])
                p += 1
        a = (a[0] - pick[0], a[1] - pick[1], a[2] - pick[2])
    assert a == (0, 0, 0)
    return {seats[j]: frozenset(out[j]) for j in range(3)}


def build_kernel(h, trick_no):
    """Kernel at the start of `trick_no`: focal seat = the actual trick leader,
    fiber = all void-consistent assignments of the unseen tiles to the three
    hidden seats at equal hand sizes."""
    hs, leader = R.state_before_trick(h, trick_no)
    k = 8 - trick_no
    assert all(len(hs[s]) == k for s in range(4))
    focal = leader
    others = [s for s in range(4) if s != focal]
    unseen = sorted(set().union(*(hs[s] for s in others)))
    voids = R.voids_before_trick(h, trick_no)
    sizes = {s: k for s in others}
    total, _g, _dp = fiber_count_and_dp(unseen, sizes, voids, h.decl)

    K = Kernel()
    K.kid = f"h{h.hid}t{trick_no}"
    K.hand_id = h.hid
    K.decl = h.decl
    K.trick_no = trick_no
    K.horizon = k
    K.focal = focal
    K.focal_team = R.TEAM_OF[focal]
    K.focal_hand = tuple(sorted(hs[focal]))
    K.unseen = unseen
    K.voids = voids
    K.sizes = sizes
    K.hidden_seats = others
    K.live = tuple(sorted(set(unseen) | set(K.focal_hand)))
    tw = [None] * 4
    tw[focal] = K.focal_hand
    for s in others:
        tw[s] = tuple(sorted(hs[s]))
    K.true_world = tuple(tw)
    K.fiber_unconstrained = unconstrained_fiber_size(k)
    K.fiber_size = total
    return K


def enumerate_worlds(K):
    """Full void-constrained fiber as a list of 4-tuples of sorted tile tuples."""
    fib = R.fiber(K.unseen, K.sizes, K.voids, K.decl)
    assert len(fib) == K.fiber_size, (K.kid, len(fib), K.fiber_size)
    out = []
    for w in fib:
        hh = [None] * 4
        hh[K.focal] = K.focal_hand
        for s in K.hidden_seats:
            hh[s] = tuple(sorted(w[s]))
        out.append(tuple(hh))
    return out


def sample_worlds(K, n, rng):
    """n exactly-uniform i.i.d. draws (with replacement) from the fiber."""
    pack = fiber_count_and_dp(K.unseen, K.sizes, K.voids, K.decl)
    out = []
    for _ in range(n):
        w = fiber_sample(K.unseen, K.sizes, K.voids, K.decl, rng, pack)
        hh = [None] * 4
        hh[K.focal] = K.focal_hand
        for s in K.hidden_seats:
            hh[s] = tuple(sorted(w[s]))
        out.append(tuple(hh))
    return out


def true_world_in_fiber(K):
    """Direct check that the actual receipt deal satisfies every void cut."""
    banned = {s: {suit for (ss, suit) in K.voids if ss == s}
              for s in K.hidden_seats}
    for s in K.hidden_seats:
        for t in K.true_world[s]:
            for suit in banned[s]:
                if R.in_suit(t, suit, K.decl):
                    return False
    return True


# ------------------------------------------------- fast bitmask PI minimax

class Solver:
    """Perfect-information minimax over a trick-suffix, on bitmask states.

    Value convention: focal team's total minus the opposing team's total, so
    focal-team seats maximise and the others minimise.  Two leaf valuations:

      mode 'trick'  -- symmetric trick differential: each trick is worth +-1.
      mode 'points' -- the real straight-42 scoring differential: each trick is
                       worth +-(1 + the count points of its four tiles).

    Subgame results are memoised at trick boundaries only, keyed by
    (four hand masks, leader).  The cache is shared across every world of a
    kernel: worlds whose divergent tiles have already been played collapse onto
    the same boundary state, and that collapse is what makes the census
    affordable.
    """

    __slots__ = ("dc", "focal_team", "tiles", "idx", "n", "is_trump",
                 "ledsuit", "suitmask", "rank", "rank7", "tmask", "pts",
                 "sign", "mode", "cache", "m", "cache_limit", "clears",
                 "max_cache")

    def __init__(self, decl, focal_team, live_tiles, mode, cache_limit=None):
        assert mode in ("trick", "points")
        self.dc = decl
        self.focal_team = focal_team
        self.mode = mode
        self.tiles = tuple(live_tiles)
        self.n = len(self.tiles)
        self.idx = {t: i for i, t in enumerate(self.tiles)}
        self.is_trump = [R.is_trump(t, decl) for t in self.tiles]
        self.ledsuit = [TRUMP_ID if self.is_trump[i] else self.tiles[i][0]
                        for i in range(self.n)]
        self.suitmask = [0] * 8
        self.rank = [[-1] * self.n for _ in range(8)]
        for s in range(8):
            suit = R.TRUMP_SUIT if s == TRUMP_ID else s
            for i, t in enumerate(self.tiles):
                if R.in_suit(t, suit, decl):
                    self.suitmask[s] |= 1 << i
                    self.rank[s][i] = R.rank_key(t, suit, decl)
        self.rank7 = self.rank[TRUMP_ID]
        self.tmask = self.suitmask[TRUMP_ID]
        # 'points' folds the trick's count points into the leaf; 'trick' does
        # not.  Stored per tile so the leaf is one lookup either way.
        self.pts = ([R.count_pts(t) for t in self.tiles] if mode == "points"
                    else [0] * self.n)
        self.sign = tuple(1 if R.TEAM_OF[s] == focal_team else -1
                          for s in range(4))
        self.cache = {}
        self.cache_limit = cache_limit
        self.clears = 0
        self.max_cache = 0
        self.m = [0, 0, 0, 0]

    # -- state helpers ----------------------------------------------------
    def mask(self, tile_seq):
        m = 0
        for t in tile_seq:
            m |= 1 << self.idx[t]
        return m

    def world_masks(self, world):
        return tuple(self.mask(world[s]) for s in range(4))

    def maybe_clear(self):
        """Bound resident memory.  Clearing only forfeits cross-world reuse;
        the within-world reuse that dominates is rebuilt immediately."""
        n = len(self.cache)
        if n > self.max_cache:
            self.max_cache = n
        if self.cache_limit is not None and n > self.cache_limit:
            self.cache.clear()
            self.clears += 1

    # -- search -----------------------------------------------------------
    # The mutable state `self.m` is mutated and restored (undo) rather than
    # copied, so the only allocation per node is the boundary cache key.

    def boundary(self, leader):
        m = self.m
        hl = m[leader]
        if not hl:
            return 0
        key = (m[0], m[1], m[2], m[3], leader)
        c = self.cache.get(key)
        if c is not None:
            return c
        maxi = self.sign[leader] == 1
        best = None
        h = hl
        while h:
            b = h & -h
            h ^= b
            m[leader] = hl ^ b
            v = self._d1(leader, b.bit_length() - 1)
            if best is None or (v > best if maxi else v < best):
                best = v
        m[leader] = hl
        self.cache[key] = best
        return best

    def _d1(self, leader, i0):
        m = self.m
        seat = (leader + 1) & 3
        led = self.ledsuit[i0]
        hs = m[seat]
        h = hs & self.suitmask[led]
        if not h:
            h = hs
        maxi = self.sign[seat] == 1
        best = None
        while h:
            b = h & -h
            h ^= b
            m[seat] = hs ^ b
            v = self._d2(leader, led, i0, b.bit_length() - 1)
            if best is None or (v > best if maxi else v < best):
                best = v
        m[seat] = hs
        return best

    def _d2(self, leader, led, i0, i1):
        m = self.m
        seat = (leader + 2) & 3
        hs = m[seat]
        h = hs & self.suitmask[led]
        if not h:
            h = hs
        maxi = self.sign[seat] == 1
        best = None
        while h:
            b = h & -h
            h ^= b
            m[seat] = hs ^ b
            v = self._d3(leader, led, i0, i1, b.bit_length() - 1)
            if best is None or (v > best if maxi else v < best):
                best = v
        m[seat] = hs
        return best

    def _d3(self, leader, led, i0, i1, i2):
        m = self.m
        seat = (leader + 3) & 3
        hs = m[seat]
        h = hs & self.suitmask[led]
        if not h:
            h = hs
        maxi = self.sign[seat] == 1
        best = None
        pts = self.pts
        sign = self.sign
        rank7 = self.rank7
        tmask = self.tmask
        rl = self.rank[led]
        sml = self.suitmask[led]
        p012 = pts[i0] + pts[i1] + pts[i2]
        while h:
            b = h & -h
            h ^= b
            i3 = b.bit_length() - 1
            m[seat] = hs ^ b
            # -- resolve the trick (same rule as R.trick_winner) ----------
            w = -1
            br = -1
            if led != TRUMP_ID:
                for off, i in ((0, i0), (1, i1), (2, i2), (3, i3)):
                    if (1 << i) & tmask:
                        r = rank7[i]
                        if r > br:
                            br = r
                            w = (leader + off) & 3
            if w < 0:
                for off, i in ((0, i0), (1, i1), (2, i2), (3, i3)):
                    if (1 << i) & sml:
                        r = rl[i]
                        if r > br:
                            br = r
                            w = (leader + off) & 3
            v = sign[w] * (1 + p012 + pts[i3]) + self.boundary(w)
            if best is None or (v > best if maxi else v < best):
                best = v
        m[seat] = hs
        return best

    def root_q(self, world_masks, leader, lead_index):
        m = self.m
        m[0], m[1], m[2], m[3] = world_masks
        m[leader] ^= 1 << lead_index
        return self._d1(leader, lead_index)

    def root_vector(self, world_masks, focal, root_indices):
        return tuple(self.root_q(world_masks, focal, i) for i in root_indices)


def argmax_set(vec, roots):
    best = max(vec)
    return tuple(r for r, v in zip(roots, vec) if v == best)


# ------------------------------------------------------ control covariates

def absolute_masters(K):
    """Focal tiles that no unseen tile can beat when led.

    If the led suit is not trump and any unseen trump is live, the tile can be
    trumped, so it is not a master.  Computed over the unseen SET, ignoring
    which hidden seat holds what -- a structural covariate, not a claim.
    """
    dc = K.decl
    unseen_trumps = [t for t in K.unseen if R.is_trump(t, dc)]
    out = []
    for t in K.focal_hand:
        s = R.led_suit(t, dc)
        if s == R.TRUMP_SUIT:
            r = R.rank_key(t, R.TRUMP_SUIT, dc)
            if all(R.rank_key(u, R.TRUMP_SUIT, dc) < r for u in unseen_trumps):
                out.append(t)
        else:
            if unseen_trumps:
                continue
            r = R.rank_key(t, s, dc)
            if all(not R.in_suit(u, s, dc) or R.rank_key(u, s, dc) < r
                   for u in K.unseen):
                out.append(t)
    return out


def covariates(K):
    dc = K.decl
    live_tr = [t for t in K.live if R.is_trump(t, dc)]
    foc_tr = [t for t in K.focal_hand if R.is_trump(t, dc)]
    hid_tr = [t for t in K.unseen if R.is_trump(t, dc)]
    masters = absolute_masters(K)
    top_trump = None
    if live_tr:
        top_trump = max(live_tr, key=lambda t: R.rank_key(t, R.TRUMP_SUIT, dc))
    suits = set()
    for t in K.focal_hand:
        suits.add(str(R.led_suit(t, dc)))
    return {
        "declaration": dc.label,
        "decl_class": dc.cls,
        "focal_hand_size": K.horizon,
        "n_legal_root_actions": len(K.focal_hand),
        "n_absolute_masters": len(masters),
        "absolute_masters": [R.tname(t) for t in masters],
        "live_trumps_total": len(live_tr),
        "live_trumps_focal": len(foc_tr),
        "live_trumps_hidden": len(hid_tr),
        "focal_holds_top_live_trump": bool(top_trump is not None
                                           and top_trump in K.focal_hand),
        "focal_distinct_suits": len(suits),
        "count_points_live": sum(R.count_pts(t) for t in K.live),
        "points_at_stake": sum(R.count_pts(t) for t in K.live) + K.horizon,
        "n_voids_observed": len(K.voids),
        "focal_hand": [R.tname(t) for t in K.focal_hand],
    }


# ------------------------------------- parametric (valued-tile) PI minimax

class ParamSolver:
    """Parametric perfect-information minimax on a short suffix.

    Value(lambda) = (focal trick differential) + lambda * (capture sign of the
    valued tile d), as an exact piecewise-linear function on the ray
    lambda >= 0.  Same shape as the mm3 machinery of lambda_probe_v3; kept on
    tile tuples because it is only ever run at horizon 2 and 3.
    """

    __slots__ = ("dc", "fteam", "d", "cache")

    def __init__(self, decl, focal_team, d):
        self.dc = decl
        self.fteam = focal_team
        self.d = d
        self.cache = {}

    def _sgn(self, seat):
        return 1 if R.TEAM_OF[seat] == self.fteam else -1

    def value(self, hands, leader, dcap):
        key = (hands, leader, dcap)
        c = self.cache.get(key)
        if c is not None:
            return c
        if not hands[leader]:
            res = [(F(0), F(0), F(0))]
            self.cache[key] = res
            return res
        vals = []
        for t in hands[leader]:
            h2 = list(hands)
            h2[leader] = tuple(x for x in hands[leader] if x != t)
            vals.append(self._trick(tuple(h2), leader, [(leader, t)], dcap))
        res = (P.pwl_max(vals) if R.TEAM_OF[leader] == self.fteam
               else _pwl_min(vals))
        self.cache[key] = res
        return res

    def _trick(self, hands, leader, plays, dcap):
        if len(plays) == 4:
            w = R.trick_winner(plays, self.dc)
            td = self._sgn(w)
            cap_now = (not dcap) and any(t == self.d for _s, t in plays)
            sub = self.value(hands, w, dcap or cap_now)
            slope_add = self._sgn(w) if cap_now else 0
            return [(x, sl + slope_add, ic + td) for x, sl, ic in sub]
        seat = (leader + len(plays)) % 4
        led = R.led_suit(plays[0][1], self.dc)
        vals = []
        for t in R.legal(hands[seat], led, self.dc):
            h2 = list(hands)
            h2[seat] = tuple(x for x in hands[seat] if x != t)
            vals.append(self._trick(tuple(h2), leader, plays + [(seat, t)],
                                    dcap))
        return (P.pwl_max(vals) if R.TEAM_OF[seat] == self.fteam
                else _pwl_min(vals))

    def root(self, world, leader, lead_tile):
        h2 = list(world)
        h2[leader] = tuple(x for x in world[leader] if x != lead_tile)
        return self._trick(tuple(h2), leader, [(leader, lead_tile)], False)


def _pwl_min(fs):
    r = fs[0]
    for f in fs[1:]:
        r = P.pwl_min2(r, f)
    return r


def param_targets(K, d, solver=None):
    """Per-world parametric root-Q signature and parametric optimal-action
    correspondence for valued tile d."""
    S = solver or ParamSolver(K.decl, K.focal_team, d)
    roots = list(K.focal_hand)

    def one(world):
        qs = {a: S.root(world, K.focal, a) for a in roots}
        sig = tuple(tuple(map(tuple, qs[a])) for a in roots)
        env = P.pwl_max([qs[a] for a in roots])
        xs = sorted({x for a in roots for x, _, _ in qs[a]}
                    | {x for x, _, _ in env})
        corr = []
        for x0 in xs:
            e = P.seg_at(env, x0)
            best = tuple(sorted(i for i, a in enumerate(roots)
                                if P.seg_at(qs[a], x0) == e))
            if not corr or corr[-1][1] != best:
                corr.append((str(x0), best))
        return sig, tuple(corr)

    return one


def highest_count_unseen(K):
    """The valued tile for the parametric target: highest count-point unseen
    tile, ties broken by the tile's total pips then its name (deterministic)."""
    return max(K.unseen, key=lambda t: (R.count_pts(t), t[0] + t[1], t))
