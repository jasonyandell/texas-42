"""Staged tests only; set H0_DIAGNOSTIC_FITTER to the selected fitter source."""
import copy
from fractions import Fraction
import importlib.util
import json
import os
from pathlib import Path
import tempfile
import unittest

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('h0_diagnostic', HERE/'historical_h0_diagnostic.py')
D = importlib.util.module_from_spec(spec)
spec.loader.exec_module(D)
FITTER = Path(os.environ.get('H0_DIAGNOSTIC_FITTER',
    '/Users/jason/.codex/worktrees/walt-response-ladder/texas-42/experiments/response-ladder/tools/fit_compiled.py'))
FIT = D.load_fitter(FITTER)


def row(i, group, mass, cost0, seat=1, split='development'):
    return {'id':i,'group':group,'mass':mass,'costs':{0:cost0,1:0},
            'actions':(0,)+tuple([None]*15),'fallback':0,'split':split,
            'request':{'seat':seat,'bidder':1,'hand':[0,1],'history':[]},
            'feature_schema':'scheme-relational-actor-v2'}


def campaign(rows, missing=(), refused=()):
    requests,lessons,index = {},{},{}
    for r in rows:
        i = r['id']
        request = {'id':i,'group':r['group'],'split':r['split'],'request':r['request']}
        requests[i] = request
        best = r['mass']
        response = {'status':'complete','maxcount':best,'mass':r['mass'],
            'action_counts':[[a,best-c] for a,c in sorted(r['costs'].items())],
            'normalized_state':{'public':{'voids':[0,0,0,0]}},
            'bundle':{'seed':71,'scenarios':[{'hands':[1,2,4,8],'tape':s,'weight':1} for s in range(r['mass'])]}}
        lessons[i] = {**request,'response':response}
        index[i] = r
    for i in missing:
        requests[i] = {'id':i,'group':i,'split':'development',
                       'request':{'seat':1,'bidder':1,'hand':[0,1],'history':[]}}
    for i in refused:
        req = {'id':i,'group':i,'split':'development',
               'request':{'seat':0,'bidder':1,'hand':[0,1],'history':[]}}
        requests[i] = req
        lessons[i] = {**req,'response':{'status':'refused'}}
    return {'rows':index,'lessons':lessons,'requests':requests,'hashes':{},'authority':{}}


def actor():
    return {'schema':'scheme-role-actors-v1',
            'declaring':{'schema':'scheme-relational-actor-v2','clauses':[0]},
            'defending':{'schema':'scheme-relational-actor-v2','clauses':[0]}}


class HistoricalDiagnosticTests(unittest.TestCase):
    def population(self):
        # Declaring: group A has two rows, group B one. Per-group normalized
        # costs are 1/2 and0, hence declaring1/4. Defending has one row cost1.
        return [row('a1','A',4,4),row('a2','A',8,0),row('b','B',2,0),row('c','C',5,5,seat=0)]

    def test_mass_and_group_weighting_and_role_mean_are_not_raw_averages(self):
        rows = self.population()
        declaring = [r for r in rows if r['request']['seat']==1]
        choices = {r['id']:0 for r in rows}
        self.assertEqual(D.weighted_cost(declaring,choices,FIT),Fraction(1,4))
        score = D.score_actor(actor(),rows,FIT)
        self.assertEqual(D.unrational(score['equal_role_mean_cost']),Fraction(5,8))
        self.assertEqual(score['raw_canonical_hits'],2)
        self.assertEqual(score['raw_optimal_hits'],2)
        # Pooled row cost is1/2; pooled group cost2/3. Neither is the role mean.
        self.assertNotEqual(D.unrational(score['equal_role_mean_cost']),Fraction(1,2))
        self.assertNotEqual(D.unrational(score['equal_role_mean_cost']),Fraction(2,3))

    def test_pair_cross_cost_divides_by_each_target_mass_and_retains_censoring(self):
        right = campaign(self.population(),missing=['m'],refused=['r'])
        left = copy.deepcopy(right)
        for r in left['rows'].values():
            r['costs'] = {0:0,1:r['mass']}
            left['lessons'][r['id']]['response']['action_counts'] = [[0,r['mass']],[1,0]]
        metadata = D.match_campaigns({'left':left,'right':right})
        result = D.pair_metrics(left,right,set(metadata),metadata,FIT)
        self.assertEqual(result['paired_complete'],4)
        self.assertEqual(result['raw_canonical_matches'],2)
        self.assertEqual(result['raw_optimal_set_intersections'],2)
        self.assertEqual(D.unrational(result['equal_role_mean_cost']['left_choice_cost_on_right']),Fraction(5,8))
        self.assertEqual(D.unrational(result['equal_role_mean_cost']['right_choice_cost_on_left']),Fraction(5,8))
        coverage = D.coverage({'left':left,'right':right},metadata)
        self.assertEqual(coverage['requests'],6)
        self.assertEqual(coverage['campaigns']['left'],{'complete':4,'missing':1,'refused':1})
        self.assertEqual(len(D.witnesses(left,right,metadata,maximum=1)),1)

    def test_exact_counts_and_normalized_vectors_are_distinct(self):
        a = campaign([row('x','A',4,2)])
        b = campaign([row('x','A',8,4)])
        metadata = D.match_campaigns({'a':a,'b':b})
        result = D.pair_metrics(a,b,{'x'},metadata,FIT)
        self.assertEqual(result['exact_vector_matches'],0)
        self.assertEqual(result['normalized_vector_matches'],1)
        self.assertEqual(result['raw_canonical_matches'],1)
        self.assertIsNone(result['equal_role_mean_cost']['left_choice_cost_on_right'])

    def test_no_void_requires_ordered_columns_tapes_seed_and_exact_vector(self):
        a = campaign([row('x','A',4,2)])
        b = copy.deepcopy(a)
        metadata = D.match_campaigns({'a':a,'b':b})
        self.assertEqual(D.no_void_invariant(a,b,metadata)['checked'],1)
        for mutation in ('seed','tape','order','vector'):
            broken = copy.deepcopy(b)
            response = broken['lessons']['x']['response']
            if mutation=='seed': response['bundle']['seed'] += 1
            if mutation=='tape': response['bundle']['scenarios'][0]['tape'] += 100
            if mutation=='order': response['bundle']['scenarios'].reverse()
            if mutation=='vector': response['action_counts'][0][1] += 1
            with self.assertRaisesRegex(ValueError,'no-void'):
                D.no_void_invariant(a,broken,metadata)
        metadata['x']['public_void'] = 'yes'
        self.assertEqual(D.no_void_invariant(a,broken,metadata)['checked'],0)

    def test_identity_mismatch_and_missing_feature_are_errors_not_exclusions(self):
        a = campaign(self.population())
        for key in ('group','split','request'):
            b = copy.deepcopy(a)
            b['requests']['a1'][key] = {'changed':True} if key=='request' else 'changed'
            with self.assertRaises(ValueError): D.match_campaigns({'a':a,'b':b})
        r = row('x','A',8,1)
        r['actions'] = r['actions'][:14]
        old = {'schema':'scheme-relational-actor-v1','clauses':[0]}
        self.assertEqual(D.actor_action(old,r,FIT),0)
        with self.assertRaisesRegex(ValueError,'missing clause'):
            D.actor_action({'schema':'scheme-relational-actor-v2','clauses':[14]},r,FIT)
        with self.assertRaises(ValueError):
            D.rational(Fraction(9,8))

    def test_authoritative_role_costs_checked_exactly_including_unequal_groups(self):
        rows = self.population()
        rows += [{**copy.deepcopy(r),'id':'t-'+r['id'],'group':'t-'+r['group'],'split':'train'} for r in rows]
        c = campaign(rows)
        expected = D.rational(Fraction(5,8))
        audit = {'source_lesson_hashes':{},'campaign_authority_hashes':{},
                 'role_mean_cost':{'train':expected,'development':expected},'roles':{}}
        for name,cost in [('declaring',Fraction(1,4)),('defending',Fraction(1))]:
            audit['roles'][name] = {'selected':{'clauses':[0],
                'train_local_cost':D.rational(cost),'development_local_cost':D.rational(cost)}}
        self.assertEqual(D.verify_fit(actor(),audit,c,FIT)['status'],'passed')
        audit['role_mean_cost']['development'] = D.rational(Fraction(1,2))
        with self.assertRaisesRegex(ValueError,'role-mean'):
            D.verify_fit(actor(),audit,c,FIT)

    def test_loader_reuses_fitter_validation_and_manifest_request_matching(self):
        with tempfile.TemporaryDirectory() as temporary:
            p = Path(temporary); (p/'lessons').mkdir()
            r = row('x','A',8,2)
            item = {'id':'x','group':'A','split':'development','request':r['request']}
            text = json.dumps(item)+'\n'; (p/'requests.jsonl').write_text(text)
            (p/'manifest.json').write_text(json.dumps({'requests_total':1,'requests_sha256':D.digest(p/'requests.jsonl')}))
            response = {'status':'complete','schema':'compiled-teacher-v2','feature_schema':'scheme-relational-actor-v2',
                'target':'compatible-root-reset-v1','revision':'t0-dice-exact-v1','field_revision':'historical-dice-v1',
                'request':{**r['request'],'n':8},'mass':8,'denominator':8,'maxcount':8,'bundle':{'mass':8},
                'action_counts':[[0,6],[1,8]],'canonical_action':1,'clause_actions':list(r['actions']),
                'costs':[{'action':0,'numerator':2,'denominator':8},{'action':1,'numerator':0,'denominator':8}]}
            path = p/'lessons/x.json'
            path.write_text(json.dumps({**item,'status':'complete','response':response}))
            loaded = D.load_campaign(p,FIT,D.EXPECTED['t0'],8)
            self.assertEqual(loaded['rows']['x']['mass'],8)
            response['costs'][0]['numerator'] = 1
            path.write_text(json.dumps({**item,'status':'complete','response':response}))
            with self.assertRaisesRegex(ValueError,'costs disagree'):
                D.load_campaign(p,FIT,D.EXPECTED['t0'],8)
            changed = {**item,'group':'other','response':response}
            path.write_text(json.dumps(changed))
            with self.assertRaisesRegex(ValueError,'request/group/split'):
                D.load_campaign(p,FIT,D.EXPECTED['t0'],8)


if __name__ == '__main__':
    unittest.main()
