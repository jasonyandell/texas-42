"""Checks against misleading discovery: strata, paired declarations, held-out data."""
import copy
from fractions import Fraction
import unittest

import threat_probe as t


def row(seed, trial, decl=0, score=None):
    hands = [list(range(21,28)),list(range(7)),list(range(7,14)),list(range(14,21))]
    if trial % 2:
        hands[1][0],hands[2][0] = hands[2][0],hands[1][0]
    return {'deal_seed':seed,'hand_id':seed,'trial':trial,'decl':decl,'seat':0,
            'cell_id':seed*10+decl,'job_id':seed*100+decl*8+trial,
            'hands':hands,'score':score if score is not None else 20 if trial % 2 == 0 else 35}


class DiscoveryChecks(unittest.TestCase):
    def test_hand_difficulty_alone_has_zero_within_cell_excess(self):
        rows = [row(1,i,score=20) for i in range(4)] + [row(2,i,score=35) for i in range(4)]
        index = t.Index(rows)
        for mask in (15, 240, 255, 1, 17):
            self.assertEqual(index.excess(mask),0)
            self.assertEqual(index.merit(mask),0)

    def test_joint_permutation_does_not_count_declarations_as_independent(self):
        rows = [row(1,i,d) for d in range(9) for i in range(4)]
        result = t.permutations(t.Index(rows),[(28,)],count=599)[0]
        probability = Fraction(*result['one_sided_p'])
        # Exactly 2/4 trials have the feature and fail. The one-hand null has
        # only six arrangements; nine declarations must not create 6**9.
        self.assertGreater(probability,Fraction(1,10))
        self.assertLess(probability,Fraction(1,4))

    def test_candidates_are_subsets_of_actual_losing_worlds(self):
        rows = [row(s,i,d) for s in range(15) for d in range(9) for i in range(4)]
        fit = t.fit(rows)
        self.assertTrue(fit['selected'])
        lookup = {r['job_id']:r for r in rows}
        for candidate in fit['selected']:
            seed = lookup[candidate['seed_job_id']]
            self.assertLess(seed['score'],30)
            self.assertTrue(set(candidate['clause']) <= set(t.hidden_facts(seed)))
            self.assertIn(len(candidate['clause']),(1,2,3))

    def test_heldout_outcomes_do_not_change_selected_queries(self):
        rows = [row(s,i,d) for s in range(25) for d in range(9) for i in range(8)]
        folds = t.split(rows)
        fit = t.fit(folds['discovery'])
        changed = copy.deepcopy(rows)
        for r in changed:
            if r['deal_seed'] >= 15 or r['trial'] >= 4: r['score'] = 42-r['score']
        self.assertEqual(fit,t.fit(t.split(changed)['discovery']))
        self.assertFalse({r['deal_seed'] for r in folds['discovery']} &
                         {r['deal_seed'] for r in folds['new_hand']})


if __name__ == '__main__': unittest.main()
