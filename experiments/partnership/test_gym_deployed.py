"""Continuation contracts, public-input locality, sample honesty and resumability."""
from copy import deepcopy
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import gym
import gym_compare
import gym_deployed as deployed
import gym_generation
import gym_spec


SETTINGS=dict(contract='deployed-gym-v1',max_worlds=400,
              players=dict(focal='l1-default',partner='l2-partner-default',opponents='l1-default'),
              sample_worlds=3,sample_seed=420600)


class DeployedTests(unittest.TestCase):
    def fake(self, request, **kwargs):
        self.assertEqual(set(request),gym.INPUT_KEYS)
        legal=gym.information_state(request)['legal']
        # Different partner procedure exposes incorrect seat/config cache sharing.
        return dict(choice=legal[-1] if kwargs['mode']=='partner' else legal[0],
                    over_budget=False,route='native',elapsed_us=0)

    def root(self):
        case=dict(gym.gallery(gym.DEFAULT_GALLERY))['advantage-02']
        return {k:deepcopy(case[k]) for k in ('id','request','source')}

    def test_contract_validates_presets_and_sample_claims(self):
        _,a,_=gym_spec.load(gym.ROOT/'walt/gym/specs/partnership-bid-making.json',[])
        a['evaluation']=deepcopy(SETTINGS)
        a['domain']={'own_remaining':[2,3],'seed':420600,'limit':5000}
        self.assertEqual(gym_spec.arguments(a),a)
        self.assertEqual(gym_spec.trick_range(a['domain']), (5,6))
        for update in ({'sample_worlds':True},{'sample_seed':-1},{'unused':1}):
            b=deepcopy(a);b['evaluation'].update(update)
            with self.assertRaises(ValueError):gym_spec.arguments(b)
        for name in ('missing','phone'):
            b=deepcopy(a);b['evaluation']['players']['partner']=name
            with self.assertRaises(ValueError):gym_spec.arguments(b)
        a['selection']['certain']=True
        with self.assertRaisesRegex(ValueError,'full-support'):gym_spec.arguments(a)

    def test_cache_identity_tracks_continuation_but_not_query_filters_or_corpus(self):
        _,a,_=gym_spec.load(gym.ROOT/'walt/gym/specs/partnership-bid-making.json',[])
        a['evaluation']=deepcopy(SETTINGS)
        original=gym_generation.identities(a)
        b=deepcopy(a);b['query']['source']='(different query)'
        b['selection']['min_spread']='1/5';b['source']['paths']=['different'];b['domain']['limit']=1
        changed=gym_generation.identities(b)
        self.assertEqual(original[0],changed[0]);self.assertNotEqual(original[1],changed[1])
        for update in ({'sample_seed':2},{'players':dict(focal='l1-default',partner='l1-default',opponents='l1-default')}):
            b=deepcopy(a);b['evaluation'].update(update)
            self.assertNotEqual(original[0],gym_generation.identities(b)[0])

    def test_samples_are_shared_unique_repeatable_and_not_certainty(self):
        req=self.root()['request'];worlds=sorted(gym.compatible_worlds(req))
        selected=deployed.selected_worlds(req,worlds,SETTINGS)
        self.assertEqual(len(set(selected)),3)
        self.assertEqual(selected,deployed.selected_worlds(req,worlds,SETTINGS))
        key=dict(worlds=3,coverage='sample-without-replacement',actions=[dict(tile=1,success_mass=3),dict(tile=2,success_mass=0)])
        profile=gym.outcome_profile(key)
        self.assertFalse(profile['certain_success_failure_swing'])
        self.assertEqual(profile['guaranteed_success'],[])
        self.assertEqual(profile['spread'],'1')

    def test_roundtrip_audit_checks_worlds_local_inputs_and_configs(self):
        with tempfile.TemporaryDirectory() as d,patch('sunshine_worlds.decide',side_effect=self.fake),patch('builtins.print'):
            output=Path(d);root=self.root()
            deployed.Evaluator(output,SETTINGS).run([root],2,40,20)
            case=gym_generation.read(output/'items'/(root['id']+'.json'))
            audit=deployed.verify(case)
            self.assertEqual((audit['worlds'],audit['support_worlds']),(3,12))
            key=case['key']
            decisions=list(key['decisions'].values())
            self.assertTrue(any(d['identity']['player']['mode']=='partner' for d in decisions))
            self.assertTrue(all(set(d['identity']['request'])==gym.INPUT_KEYS for d in decisions))
            for change in ('world','decision','score','prefix','best','sample'):
                altered=deepcopy(case);a=altered['key']['actions'][0]
                if change=='world':a['traces'][0]['hands'][0]=[]
                elif change=='decision':next(iter(altered['key']['decisions'].values()))['identity']['request']['seed']+=1
                elif change=='score':a['success_mass']+=1
                elif change=='prefix':altered['key']['prefix']=[]
                elif change=='best':altered['key']['best']=[]
                else:altered['key']['settings']['sample_worlds']=2
                with self.subTest(change=change),self.assertRaises((AssertionError,ValueError)):
                    deployed.verify(altered)

    def test_failed_trajectory_resume_preserves_all_completed_work(self):
        root=self.root();failed=False
        actual=deployed.Evaluator.play
        def fail_once(instance,job,seconds):
            nonlocal failed
            if not failed:
                failed=True
                raise ValueError('injected retry')
            return actual(instance,job,seconds)
        with tempfile.TemporaryDirectory() as d,patch('sunshine_worlds.decide',side_effect=self.fake),patch('builtins.print'):
            output=Path(d)
            with patch.object(deployed.Evaluator,'play',fail_once):
                deployed.Evaluator(output,SETTINGS).run([root],2,40,20)
            self.assertFalse((output/'items'/(root['id']+'.json')).exists())
            saved={p:p.read_bytes() for p in output.rglob('*.json') if '/failures/' not in str(p) and p.name!='status.json'}
            deployed.Evaluator(output,SETTINGS).run([root],2,40,20)
            self.assertTrue((output/'items'/(root['id']+'.json')).exists())
            self.assertTrue(all(p.read_bytes()==data for p,data in saved.items()))
            with patch('sunshine_worlds.decide',side_effect=AssertionError('unexpected recomputation')):
                deployed.Evaluator(output,SETTINGS).run([root],2,40,20)

    def test_avoided_requires_strictly_worse_than_best_other(self):
        key=dict(worlds=10,offers=[1],best=[2],actions=[dict(tile=1,success_mass=5),dict(tile=2,success_mass=6),dict(tile=3,success_mass=0)])
        case=dict(key=key,target_actions=[1],criterion='query-avoided')
        self.assertTrue(gym.case_pairs(case))
        key['actions'][0]['success_mass']=6;key['best']=[1,2]
        self.assertEqual(gym.case_pairs(case),[])

    def test_comparison_uses_coordinate_not_display_name_and_preserves_ties(self):
        base=dict(id='a',worlds=10,selected=True,outcome=dict(optimal=[1],actions=[]),query_contrast=dict(gap='1/10'))
        before=dict(directory='a',specification=None,rows={'a':base},cases={'a':('advantage-01',{})})
        after=dict(directory='b',specification=None,rows={'a':{**base,'query_contrast':dict(gap='0')}},cases={'a':('advantage-77',{})})
        # No selected changed case is necessary to compare raw controls.
        before['cases']={};after['cases']={}
        result=gym_compare.comparison(before,after)
        self.assertEqual(result['summary']['relation_changed'],1)
        self.assertEqual(result['rows'][0]['after_relation'],'tied')


if __name__=='__main__':unittest.main()
