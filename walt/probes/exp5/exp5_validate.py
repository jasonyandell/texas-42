#!/usr/bin/env python3
"""
exp5_validate.py -- an independent, deliberately naive scalar minimax.

No cache, no bitmasks, no PWL: plain tuples of tiles and direct calls into the
declaration-relative rule predicates.  Written separately from exp5_core.Solver
so that agreement between the two is evidence about the fast solver rather than
a restatement of it.  Used only for spot checks on a handful of worlds per
kernel -- it is far too slow to run the census with.
"""

from __future__ import annotations

import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import exp5_rules as R


def naive_value(hands, leader, dc, focal_team, mode):
    """hands: dict seat -> tuple of tiles.  Returns focal-minus-other total."""
    if not hands[leader]:
        return 0
    sgn = lambda s: 1 if R.TEAM_OF[s] == focal_team else -1
    vals = []
    for t in hands[leader]:
        h2 = dict(hands)
        h2[leader] = tuple(x for x in hands[leader] if x != t)
        vals.append(_naive_trick(h2, leader, [(leader, t)], dc, focal_team, mode))
    return max(vals) if sgn(leader) == 1 else min(vals)


def _naive_trick(hands, leader, plays, dc, focal_team, mode):
    sgn = lambda s: 1 if R.TEAM_OF[s] == focal_team else -1
    if len(plays) == 4:
        w = R.trick_winner(plays, dc)
        base = R.trick_points(plays) if mode == "points" else 1
        return sgn(w) * base + naive_value(hands, w, dc, focal_team, mode)
    seat = (leader + len(plays)) % 4
    led = R.led_suit(plays[0][1], dc)
    opts = R.legal(hands[seat], led, dc)
    vals = []
    for t in opts:
        h2 = dict(hands)
        h2[seat] = tuple(x for x in hands[seat] if x != t)
        vals.append(_naive_trick(h2, leader, plays + [(seat, t)], dc,
                                 focal_team, mode))
    return max(vals) if sgn(seat) == 1 else min(vals)


def naive_root_vector(world, focal, roots, dc, focal_team, mode):
    hands = {s: tuple(world[s]) for s in range(4)}
    out = []
    for a in roots:
        h2 = dict(hands)
        h2[focal] = tuple(x for x in hands[focal] if x != a)
        out.append(_naive_trick(h2, focal, [(focal, a)], dc, focal_team, mode))
    return tuple(out)
