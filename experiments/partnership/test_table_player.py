"""Live native transport, including failure after a completed checkpoint."""
import json
import subprocess
import unittest
from unittest.mock import patch
from table_player import decide, auction

REQUEST = dict(decl=3,bid=30,bidder=0,seat=2,hand=[0,2,3,15,17,21,25],
               plays=[0,7,1,24,2,3,3,6,1,5,2,17,3,23,0,12,1,4,2,21,3,22,0,8,0,9,1,19,2,25,3,18,0,13,1,16],seed=420600)

class SharedTableTests(unittest.TestCase):
    def test_native_transport_matches_the_completed_default_player(self):
        value=decide(REQUEST)
        self.assertEqual(value['player_version'],'walt-table-v2')
        self.assertEqual(value['route'],'baseline')
        self.assertEqual(value['choice'],2)
        self.assertEqual(value['evaluation']['outer_worlds'],40)
        self.assertEqual(value['fallback_evaluation']['outer_worlds'],8)

    def test_host_timeout_keeps_the_complete_checkpoint_and_rechecks_rules(self):
        value=decide(REQUEST)
        output=(json.dumps({'checkpoint':value})+'\n').encode()
        timeout=subprocess.TimeoutExpired('walt-table',18,output=output)
        with patch('table_player.subprocess.run',side_effect=timeout):
            recovered=decide(REQUEST)
        self.assertEqual(recovered['evaluation'],value['evaluation'])
        self.assertEqual(recovered['choice'],value['choice'])
        self.assertIn('interruption',recovered)
        value['points']=[99,0]
        broken=subprocess.TimeoutExpired('walt-table',18,output=(json.dumps({'checkpoint':value})+'\n').encode())
        with patch('table_player.subprocess.run',side_effect=broken):
            with self.assertRaisesRegex(ValueError,'independent rules'):decide(REQUEST)

    def test_unsupported_profile_is_not_silently_renamed_default(self):
        with self.assertRaises(ValueError):decide(REQUEST,inner_belief='voids-counted')
        with self.assertRaises(ValueError):decide({**REQUEST,'hands':[]})

    def test_auction_timeout_retains_a_complete_public_checkpoint(self):
        req=dict(hand=[1,6,8,19,20,23,27],seat=0,bid=31,seed=42)
        value=dict(schema='walt-auction-v1',**req,decl=5,worlds=4,eligible=True,
                   prices=[[d,'1','1'] for d in (*range(8),9)])
        timeout=subprocess.TimeoutExpired('walt-table',8.5,output=(json.dumps(dict(checkpoint=value))+'\n').encode())
        with patch('table_player.subprocess.run',side_effect=timeout):
            result=auction(dict(auction=req,worlds=160,budget_ms=20000))
        self.assertEqual(result['prices'],value['prices'])
        self.assertIn('interruption',result)
        with patch('table_player.subprocess.run',side_effect=timeout):
            self.assertEqual(auction(dict(auction=req,budget_ms=4500))['prices'],value['prices'])
        with self.assertRaises(ValueError):auction(dict(auction={**req,'hands':[]},worlds=160,budget_ms=20000))

if __name__=='__main__':unittest.main()
