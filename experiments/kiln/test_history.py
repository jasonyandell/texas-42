import copy
import sys
from pathlib import Path
import unittest

sys.path.insert(0,str(Path(__file__).resolve().parent))
import history as h

CELL={'cell_id':1,'hand_id':0,'seat':0,'hand':[0,1,2,3,4,5,6],'decl':6}


def samples(scores):
    return [{'trial':i,'job_id':i+1,'score':score,'receipt_sha256':h.identity([i,score]),'producer':'test'}
            for i,score in enumerate(scores)]


class HistoryTests(unittest.TestCase):
    def test_added_make_can_lower_the_bid_through_denominator(self):
        rows=samples([36]*8+[30]*3)
        result=h.cell_history(CELL,rows,[4,5])
        change=result['changes'][-1]
        self.assertEqual(change['before']['games'],10)
        self.assertEqual(change['before']['recommended_bid'],36)
        self.assertEqual(change['after']['recommended_bid'],30)
        self.assertEqual(change['before']['tails30_42'][6],8)
        self.assertEqual(change['after']['tails30_42'][6],8)
        self.assertEqual(change['thresholds_crossed'],list(range(31,37)))
        self.assertEqual(change['added_game']['score'],30)
        self.assertEqual(h.cell_history(CELL,rows[:10],[4,5])['prefix_sha256'],change['before_prefix_sha256'])

    def test_recovery_and_non_changing_controls_are_retained(self):
        rows=samples([30]*3+[29,30])
        result=h.cell_history(CELL,rows,[4,5])
        self.assertEqual(len(result['observations']),5)
        self.assertEqual(result['changes'][0]['kind'],'initial-estimate')
        self.assertEqual(result['changes'][-2]['after']['recommended_bid'],None)
        self.assertEqual(result['changes'][-1]['after']['recommended_bid'],30)

    def test_completion_order_does_not_change_history_and_gaps_fail(self):
        rows=samples([40,35,30,29,38,42,30,31,36])
        expected=h.cell_history(CELL,rows,[4,5])
        self.assertEqual(h.cell_history(CELL,list(reversed(rows)),[4,5]),expected)
        self.assertIn('8',expected['milestones'])
        for bad in (rows[1:],rows+[rows[0]]):
            with self.assertRaises(ValueError):h.cell_history(CELL,bad,[4,5])
        altered=copy.deepcopy(rows);altered[-1]['receipt_sha256']='changed'
        self.assertNotEqual(h.cell_history(CELL,altered,[4,5])['prefix_sha256'],expected['prefix_sha256'])

    def test_all_prefix_states_match_direct_counts(self):
        rows=samples([i*17%43 for i in range(160)])
        result=h.cell_history(CELL,rows,[4,5])
        for n in range(1,161):
            got=h.cell_history(CELL,rows[:n],[4,5])['latest']
            counts=[sum(s['score']>=b for s in rows[:n]) for b in range(30,43)]
            bid=max((b for b,num in zip(range(30,43),counts) if num*5>=n*4),default=None)
            self.assertEqual(got['tails30_42'],counts)
            self.assertEqual(got['recommended_bid'],bid)
        self.assertEqual(set(result['milestones']),{'8','40','160'})


if __name__=='__main__':unittest.main()
