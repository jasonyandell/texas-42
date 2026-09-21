"""Exact finite controls for the measurement-efficiency calculation."""
from fractions import Fraction as F
import unittest

import scheme_mine as m
import scheme_mine_cv as cv
from test_threat_probe import row


class ControlVariateChecks(unittest.TestCase):
    def test_perfect_control_has_zero_variance_and_preserves_target_mean(self):
        # P5 boss is 5-5 (20), unseen by a viewer holding 21..27. Place it
        # at each hidden chair with equal mass, retaining the same own hand.
        worlds=[]
        for holder in (1,2,3):
            r=row(1,0,5)
            if holder != 3:
                other=r['hands'][holder][0]
                r['hands'][holder][0]=20
                r['hands'][3][-1]=other
            r['trial']=holder-1
            r['score']=20 if holder == 3 else 35
            worlds.append(r)
        query=m.query_record((3,'none',('boss',)),'boss')
        self.assertEqual(cv.beta(worlds,query),1)
        result=cv.measure(worlds,query,F(1))['summaries']['all']
        self.assertEqual(result['variance_ratio'],[0,1])
        self.assertEqual(result['mean_raw_failure'],[1,3])
        self.assertEqual(result['mean_corrected_failure'],[1,3])

    def test_zero_coefficient_is_exactly_the_original_estimator(self):
        worlds=[row(1,i,5) for i in range(4)]
        query=m.query_record((3,'none',('boss',)),'boss')
        result=cv.measure(worlds,query,F(0))['summaries']['all']
        self.assertEqual(result['variance_ratio'],[1,1])
        self.assertEqual(result['mean_raw_failure'],result['mean_corrected_failure'])

    def test_impossible_event_and_unsupported_probability_are_distinct(self):
        query=m.query_record((3,'none',('boss',)),'boss')
        self.assertEqual(cv.prevalence(query,row(1,0,9)),0)
        with self.assertRaises(ValueError):
            cv.prevalence(m.query_record((3,'none',('double',)),'double'),row(1,0,5))
        result=cv.measure([row(1,i,9) for i in range(4)],query,F(1))
        self.assertEqual(result['summaries']['eligible']['cells'],0)


if __name__ == '__main__': unittest.main()
