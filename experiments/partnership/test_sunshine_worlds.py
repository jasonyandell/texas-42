from dataclasses import asdict
import json
from pathlib import Path
import tempfile
import threading
import unittest
from unittest.mock import patch

import campaign as c
from matchup import Player
from rules import information_state
from sunshine_worlds import Experiment
from test_partner_review import REQUEST


class DecisionCacheTests(unittest.TestCase):
    def test_manifest_nested_worlds_resume_after_json_round_trip(self):
        with tempfile.TemporaryDirectory() as path:
            source=Path(path)/'source';source.mkdir();(source/'manifest.json').write_text('{}')
            output=Path(path)/'output';output.mkdir()
            with patch('sunshine_worlds.select_roots',return_value=[{'worlds':[((1,2),(3,4))]}]), patch('sunshine_worlds.c.identities',return_value={'test':1}):
                first=Experiment(source,output)
                resumed=Experiment(source,output)
                self.assertEqual(first.manifest,resumed.manifest)

    def make_experiment(self,path):
        e=Experiment.__new__(Experiment)
        e.output=Path(path);e.lock=threading.Lock();e.locks={}
        return e

    def fake(self,request,**kwargs):
        return dict(choice=information_state(request)['legal'][0],over_budget=False)

    def test_reuse_is_scoped_to_own_public_information_and_player(self):
        with tempfile.TemporaryDirectory() as path, patch('sunshine_worlds.decide',side_effect=self.fake) as run:
            e=self.make_experiment(path);base=Player('l1-default')
            first,key=e.decision(REQUEST,base,None)
            self.assertEqual(e.decision(REQUEST,base,None),(first,key))
            self.assertEqual(run.call_count,1)
            _,other=e.decision(REQUEST,Player('candidate',review='partner-count'),None)
            self.assertNotEqual(other,key)
            earlier={**REQUEST,'plays':REQUEST['plays'][:-8]}
            _,earlier_key=e.decision(earlier,base,None)
            self.assertNotEqual(earlier_key,key)
            self.assertEqual(run.call_count,3)
            # A restart keeps the same frozen realization.
            again=self.make_experiment(path)
            self.assertEqual(again.decision(REQUEST,base,None),(first,key))
            self.assertEqual(run.call_count,3)

    def test_corrupted_completed_decision_is_refused(self):
        with tempfile.TemporaryDirectory() as path, patch('sunshine_worlds.decide',side_effect=self.fake):
            e=self.make_experiment(path);base=Player('l1-default')
            _,key=e.decision(REQUEST,base,None)
            file=Path(path)/'decisions'/(key+'.json')
            content=json.loads(file.read_text());content['response']['choice']=27
            file.write_text(json.dumps(content))
            with self.assertRaises(AssertionError):e.decision(REQUEST,base,None)

    def test_failed_call_does_not_publish_a_decision(self):
        with tempfile.TemporaryDirectory() as path, patch('sunshine_worlds.decide',side_effect=RuntimeError('interrupted')):
            e=self.make_experiment(path)
            with self.assertRaises(RuntimeError):e.decision(REQUEST,Player('l1-default'),None)
            self.assertFalse(list(Path(path).rglob('*.json')))


if __name__=='__main__':unittest.main()
