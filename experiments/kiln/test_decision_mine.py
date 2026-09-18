import copy
from fractions import Fraction as F
import json
import subprocess
import unittest

import decision_mine as m

REQUEST = dict(bid=30,bidder=1,decl=1,hand=[3,5,9,21,23,26,27],
    plays=[1,9,2,18,3,24,0,8,1,3,2,1,3,17,0,12,2,0,3,10,0,25,1,21,
           2,19,3,20,0,14,1,26,3,2,0,11,1,5,2,7,3,4,0,22],seat=1,seed=3977870046)


class LearningTests(unittest.TestCase):
    def test_gate_uses_integrated_belief_not_actual_world(self):
        row=dict(baseline=4,threat=F(1,2),masks={'x':[8]},world_threat=[True,False])
        rule=dict(query='x',threshold='1/3')
        self.assertEqual(m.choose(rule,row),8)
        self.assertEqual(m.choose(rule,dict(row,world_threat=[False,True])),8)
        self.assertEqual(m.choose(dict(rule,threshold='2/3'),row),4)
        self.assertEqual(m.choose(rule,dict(row,masks={'x':[4,8]})),4)

    def test_paired_objective_keeps_ties_and_harms(self):
        rule=dict(query='x',threshold='0')
        data=[]
        for i,values in enumerate(({4:F(1,2),8:F(1,2)},{4:F(1),8:F(0)},{4:F(0),8:F(1,2)})):
            for rep in range(2):
                data.append(dict(root=str(i),rep=rep,deal_seed=i,baseline=4,threat=F(0),
                    masks={'x':[8]},values=values))
        result=m.describe(rule,data)
        self.assertEqual((result['helped'],result['harmed'],result['tied']),(1,1,1))
        self.assertEqual(F(result['mean_gain']),F(-1,6))
        self.assertEqual(m.describe(None,data)['mean_gain'],'0')

    def test_strata_without_action_information_explain_nothing(self):
        row=dict(baseline=4,values={4:F(0),8:F(1,2)},vectors={4:[0,0,0,0],8:[1,0,1,0]},
                 world_threat=[True,True,False,False])
        self.assertEqual(m.stratification([row])['explained_fraction'],'0')


@unittest.skipUnless(m.BINARY.exists(),'build kiln-decision-worker first')
class NativeAuditTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.job=dict(request=REQUEST,queries=m.job_queries())
        run=subprocess.run([str(m.BINARY)],input=json.dumps(cls.job)+'\n',capture_output=True,text=True,check=True,timeout=20)
        cls.result=json.loads(run.stdout)

    def test_native_census_queries_and_player_boundary_match_independent_rules(self):
        result=m.validate(self.job,self.result)
        self.assertEqual((result['worlds'],result['traces']),(6,12))
        self.assertEqual(result['query_checks'],282)

    def test_corrupt_outcome_or_hidden_player_input_is_rejected(self):
        corrupt=copy.deepcopy(self.result)
        corrupt['traces'][0]['made']=not corrupt['traces'][0]['made']
        with self.assertRaises(AssertionError):m.validate(self.job,corrupt)
        corrupt=copy.deepcopy(self.result)
        corrupt['decisions'][0]['call']['request']['hands']=corrupt['worlds'][0]
        with self.assertRaises(AssertionError):m.validate(self.job,corrupt)

    def test_omitted_world_cannot_be_published_as_census(self):
        corrupt=copy.deepcopy(self.result);corrupt['worlds'].pop()
        with self.assertRaises(AssertionError):m.validate(self.job,corrupt)


if __name__=='__main__':unittest.main()
