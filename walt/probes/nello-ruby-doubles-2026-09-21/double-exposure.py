"""Exact combinatorial exposure under the engine's uniform legal-deal prior.

Uses only public plays and Ruby's own hand; no actual hidden hands. This counts
mechanically feasible allocations, not bidding-conditioned probabilities.
"""
import json
import math
import sys
from fractions import Fraction
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
sys.path.insert(0, str(ROOT / 'experiments/partnership'))
from rules import TILES, context, follows, information_state, active_actor, winner

rows = []
for pos in json.loads((HERE / 'positions.json').read_text())['positions']:
    req = pos['request']
    information_state(req)
    played = set(req['plays'][1::2])
    own = set(req['hand']) - played
    unseen = set(range(28)) - played - own
    sizes = [7 - req['plays'][::2].count(s) for s in range(4)]
    doubles = {t for t in unseen if TILES[t][0] == TILES[t][1]}
    leader, trick = req['bidder'], []
    for seat, tile in zip(req['plays'][::2], req['plays'][1::2]):
        assert seat == active_actor(leader, len(trick), req['bidder'], 'nello')
        # A no-void record makes every allocation with these sizes admissible.
        if trick:
            assert follows(tile, context(trick[0][1], 8), 8)
        trick.append((seat, tile))
        if len(trick) == 3:
            leader, trick = winner(trick, 8), []
    assert not trick and leader == req['seat']
    n, d, h = len(unseen), len(doubles), sizes[req['bidder']]
    denominator = math.comb(n, h)
    no_double = Fraction(math.comb(n-d, h), denominator)
    # 55 is the only unseen double above 44; a sole 55 forces a declarer win.
    assert [TILES[t] for t in doubles if TILES[t][0] > 4] == [(5, 5)]
    forced55 = Fraction(math.comb(n-d, h-1), denominator)
    others = [s for s in range(4) if s not in (req['seat'], req['bidder'])]
    completions = denominator * math.comb(n-h, sizes[others[0]])
    row = {'ply':pos['ply'], 'unknown_tiles':n,
           'unseen_doubles':[list(TILES[t]) for t in sorted(doubles)],
           'declarer_remaining':h, 'feasible_allocations':completions,
           'declarer_void_doubles':str(no_double),
           'declarer_only_double_55':str(forced55),
           'claim_scope':'uniform mechanically feasible allocations, not behavioral belief'}
    rows.append(row)
    print(json.dumps(row))
(HERE / 'double-exposure.json').write_text(json.dumps(rows, indent=2)+'\n')
