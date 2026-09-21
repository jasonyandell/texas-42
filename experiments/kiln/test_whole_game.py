import copy
import json
import math
import subprocess
import unittest

import whole_game as w
from test_decision_mine import REQUEST as LATE


class SampleTests(unittest.TestCase):
    def test_independent_count_matches_opening_combinatorics_and_known_late_fiber(self):
        req=dict(LATE,plays=[],seat=LATE['bidder'])
        self.assertEqual(w.support_count(w.p.canonical(req)),math.factorial(21)//math.factorial(7)**3)
        self.assertEqual(w.support_count(w.p.canonical(LATE)),6)

    def test_sample_identity_is_separate_from_policy_rng_and_prefix_stable(self):
        root=dict(id='root',baseline=26,request=LATE)
        first=w.job_input(root,0)
        for i in range(1,25):
            self.assertEqual(w.job_input(root,i)['request']['seed'],LATE['seed'])
            self.assertNotEqual(first['sample_seed'],w.job_input(root,i)['sample_seed'])
        self.assertEqual(first,w.job_input(root,0))

    @unittest.skipUnless(w.BINARY.exists(),'build worker first')
    def test_sampled_opening_and_void_constrained_late_worlds_are_replayable(self):
        jobs=[]
        for req in (dict(LATE,plays=[],seat=LATE['bidder']),LATE):
            baseline=w.information_state(req)['legal'][0]
            for seed in range(16):jobs.append(dict(request=req,baseline=baseline,queries=[],sample_seed=seed,sample_only=True))
        native=subprocess.run([str(w.BINARY)],input=''.join(json.dumps(j)+'\n' for j in jobs),
            capture_output=True,text=True,check=True,timeout=20)
        values=[json.loads(line) for line in native.stdout.splitlines()]
        self.assertEqual(len(values),len(jobs))
        for j,v in zip(jobs,values):w.validate(j,v)
        self.assertGreater(len({json.dumps(v['worlds']) for v in values[16:]}),1)
        repeated=subprocess.run([str(w.BINARY)],input=json.dumps(jobs[0])+'\n',capture_output=True,text=True,check=True,timeout=5)
        self.assertEqual(json.loads(repeated.stdout)['worlds'],values[0]['worlds'])

    @unittest.skipUnless(w.BINARY.exists(),'build worker first')
    def test_complete_late_contrast_and_corrupted_branch_rejection(self):
        job=dict(request=LATE,queries=[],sample_seed=18,baseline=w.information_state(LATE)['legal'][0])
        native=subprocess.run([str(w.BINARY)],input=json.dumps(job)+'\n',capture_output=True,text=True,check=True,timeout=20)
        value=json.loads(native.stdout);w.validate(job,value)
        bad=copy.deepcopy(value);bad['traces'].pop()
        with self.assertRaises(AssertionError):w.validate(job,bad)
        bad=copy.deepcopy(value);bad['decisions'][0]['call']['request']['hands']=value['worlds'][0]
        with self.assertRaises(AssertionError):w.validate(job,bad)


if __name__=='__main__':unittest.main()
