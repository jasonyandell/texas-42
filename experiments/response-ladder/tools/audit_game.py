#!/usr/bin/env python3
"""Independent Python referee for response-ladder game receipts."""
import importlib.util
import json
import sys
from pathlib import Path

source = Path(__file__).resolve().with_name('reference_referee.py')
spec = importlib.util.spec_from_file_location('canonical_referee', source)
ref = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ref)

def audit(game):
    hands = [set(h) for h in game['hands']]
    assert len(set.union(*hands)) == 28 and sum(map(len, hands)) == 28
    leader = game['bidder']
    trick = []
    bank = [0, 0]
    assert len(game['plays']) == 28
    for play in game['plays']:
        actor, tile = play['actor'], play['tile']
        assert actor == (leader + len(trick)) % 4
        assert tile in ref.legal(hands[actor], trick, game['decl'])
        assert play['decision']['tile'] == tile
        report = play['decision']['report']
        if report:
            assert report['chosen'] == tile
            assert report['regret_bound'] >= 0
            assert all(0 <= a['lower'] <= a['upper'] <= report['mass'] for a in report['actions'])
            own = next(a for a in report['actions'] if a['action'] == tile)
            assert report['incumbent_value'] == (own['lower'] if own['priced'] else None)
            assert report['regret_bound'] == max(a['upper'] for a in report['actions']) - own['lower']
            # Check each competitor explicitly, preserving canonical tie order.
            expected_cert = all(a['action'] == tile or
                                (a['upper'] < own['lower'] if a['action'] < tile else a['upper'] <= own['lower'])
                                for a in report['actions'])
            assert report['canonical_certified'] == expected_cert
        hands[actor].remove(tile)
        trick.append((actor, tile))
        if len(trick) == 4:
            leader = ref.winner(trick, game['decl'])
            bank[leader % 2] += ref.trick_points(trick)
            trick = []
    assert sum(bank) == 42 and all(not h for h in hands)
    assert game['bidder_points'] == bank[game['bidder'] % 2]
    assert game['made'] == (bank[game['bidder'] % 2] >= game['bid'])
    return {'seed': game['seed'], 'decl': game['decl'], 'bidder': game['bidder'],
            'candidate_team': game['candidate_team'], 'made': game['made'],
            'elapsed_ms': game['elapsed_ms'], 'fallbacks': game['fallback_count'],
            'certified': game['certified_count'], 'plays': 28, 'passed': True}

if __name__ == '__main__':
    print(json.dumps([audit(json.loads(Path(p).read_text())) for p in sys.argv[1:]], indent=2))
