"""Pinned independent engine histories, every actor view, and native decisions."""
import json
from pathlib import Path
import unittest
import tempfile
from unittest.mock import patch

from plunge_io import decode_hand, flag_root
from player import normalize
from rules import information_state, replay_record
from table_player import decide
from plunge_bridge import Store

FIXTURES = json.loads((Path(__file__).resolve().parents[2] /
    'walt/walt-player/tests/fixtures/nello.json').read_text())


class NelloTests(unittest.TestCase):
    def test_mac_receipts_rechecks_and_finished_flags_keep_contract(self):
        f=FIXTURES[0]
        req, played, _=flag_root(f['replay'],0,20)
        def answer(request, **kwargs):
            return dict(**information_state(request),choice=played,route='baseline',elapsed_us=0)
        with tempfile.TemporaryDirectory() as folder, patch('plunge_bridge.decide',side_effect=answer):
            store=Store(folder)
            body=dict(request=req,player='l1-default',game_id='nello',hand_number=1)
            receipt=store.decision(body)
            self.assertEqual(store.receipt(receipt['id']),receipt)
            self.assertEqual(receipt['identity']['request'],req)
            self.assertEqual(store.estimate(dict(request=req,worlds=40))['identity']['request'],req)
            flag=store.flag(dict(share_code=f['replay'],ply=0,seed=20,note='Nel-O',alternative=None,receipt_id=receipt['id']))
            self.assertEqual(flag['original_receipt'],receipt)
            self.assertEqual(store.analysis(flag['id'],'l1')['status'],'outside-scope')
            with self.assertRaises(ValueError): store.decision({**body,'request':{**req,'hands':f['hands']}})

    def test_pinned_replays_and_every_own_public_view(self):
        for f in FIXTURES:
            for code in (f['replay'], f['replay'].replace('v1c', 'v1l', 1)):
                game = decode_hand(code)
                self.assertEqual(game['contract'], 'nello')
                self.assertEqual(game['points'], f['points'])
                self.assertEqual(game['plays'], f['plays'])
                points, leader, remaining, trick = replay_record(f['hands'], f['plays'], 8, f['declarer'], 'nello')
                self.assertEqual(len(remaining[(f['declarer']+2)%4]), 7)
                self.assertEqual(leader == f['declarer'], f['kind'] == 'set')
                for ply in range(len(f['plays'])//2):
                    req, played, _ = flag_root(code, ply, 20)
                    state = information_state(req)
                    self.assertIn(played, state['legal'])
                    self.assertEqual(state['trick'], ply//3+1)
                with self.assertRaises(ValueError):
                    decode_hand(code.replace('Dn', 'D8'))
                with self.assertRaises(ValueError):
                    decode_hand(code+'00')

    def test_native_checks_and_complete_comparisons_for_both_roles(self):
        for f in FIXTURES:
            for ply in (0, 2):
                req, _, _ = flag_root(f['replay'], ply, 20)
                result = decide(req, budget_ms=14000, review='partner-rollout')
                self.assertEqual(result['route'], 'baseline')
                self.assertEqual(result['contract'], 'nello')
                self.assertEqual(result['review'], 'inapplicable-nello')
                self.assertEqual(result['evaluation']['outer_worlds'], 40)
                self.assertEqual(len(result['evaluation']['options']), len(result['legal']))

    def test_input_boundary_and_no_post_set_decision(self):
        req, _, _ = flag_root(FIXTURES[0]['replay'], 0, 20)
        for changes in ({'hands': FIXTURES[0]['hands']}, {'contract': 'straight'}, {'contract': None}):
            with self.assertRaises(ValueError):
                normalize({**req, **changes})
        for changes in ({'decl': 9}, {'bid': 30}, {'seat': 2}, {'plays': FIXTURES[0]['plays']},
                        {'plays': [0, 14, 2, 27]}):
            with self.assertRaises(ValueError):
                information_state({**req, **changes})
        missing = {k:v for k,v in req.items() if k != 'contract'}
        with self.assertRaises(ValueError): information_state(missing)


if __name__ == '__main__': unittest.main()
