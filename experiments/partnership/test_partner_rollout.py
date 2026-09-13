import copy
import unittest
from unittest.mock import Mock

import partner_rollout as r
from matchup import Player
from rules import information_state
from test_partner_review import REQUEST


class RolloutTests(unittest.TestCase):
    def setUp(self):
        self.state = information_state(REQUEST)

    def answer(self, baseline=2, samples=8, support=210, masses=(3, 3, 5)):
        legal = self.state['legal']
        values = dict(zip(legal, masses))
        best = baseline
        if samples >= min(r.MIN_SAMPLES, support):
            for t in legal:
                if values[t] > values[best]: best = t
        paired = []
        for t in legal:
            both = min(values[t], values[baseline])
            paired.append([values[t]-both, values[baseline]-both, both,
                           samples-max(values[t], values[baseline])])
        return dict(schema=r.ID, field=r.FIELD, baseline=baseline, choice=best,
                    offers=[15], legal=legal, support=support, samples=samples,
                    requested=min(r.MAX_SAMPLES, support), values=list(map(list, values.items())),
                    paired=paired, coverage='census' if samples == support else 'budgeted-sample-prefix',
                    stop='census' if samples == support else 'deadline',
                    status=('changed' if best != baseline else 'retained')
                    if samples >= min(r.MIN_SAMPLES, support) else 'unresolved')

    def investigate(self, value, baseline=2):
        send = Mock(return_value=(value, 'completed'))
        result = r.investigate(REQUEST, self.state, baseline, 10, send)
        self.assertLessEqual(send.call_args.args[2], .500)
        self.assertEqual(set(line.split()[0] for line in send.call_args.args[1].splitlines()), set(REQUEST))
        return result

    def test_offering_and_withholding_are_both_open_to_revision(self):
        self.assertEqual(self.state['legal'], [0, 2, 15])
        self.assertEqual(self.investigate(self.answer())[0], 15)
        self.assertEqual(self.investigate(self.answer(15, masses=(3, 5, 2)), 15)[0], 2)

    def test_ties_keep_the_actual_baseline(self):
        for baseline in self.state['legal']:
            self.assertEqual(self.investigate(self.answer(baseline, masses=(4, 4, 4)), baseline)[0], baseline)

    def test_small_census_is_enough_but_incomplete_small_sample_is_not(self):
        self.assertEqual(self.investigate(self.answer(samples=3, support=3, masses=(1, 1, 2)))[0], 15)
        value = self.answer(samples=3, masses=(1, 1, 2))
        self.assertEqual(self.investigate(value)[1]['status'], 'unresolved')
        value['coverage'] = 'census'
        self.assertEqual(self.investigate(value)[1]['status'], 'unresolved-rejected')

    def test_malformed_or_incomplete_evidence_cannot_change_a_move(self):
        valid = self.answer()
        bads = [{}, [], {**valid, 'choice': 27}, {**valid, 'field': 'teacher'},
                {**valid, 'samples': True}, {**valid, 'requested': 65},
                {**valid, 'values': valid['values'][:-1]}, {**valid, 'paired': [[0, 0, 0, 8]]*3}]
        bad = copy.deepcopy(valid); bad['paired'][0][0] += 1; bads.append(bad)
        for bad in bads:
            with self.subTest(bad=bad):
                choice, report = self.investigate(bad)
                self.assertEqual(choice, 2)
                self.assertEqual(report['status'], 'unresolved-rejected')

    def test_refusal_and_deadline_preserve_baseline(self):
        value = self.answer(samples=0, support=401, masses=(0, 0, 0))
        value.update(stop='support-cap', coverage='none')
        self.assertEqual(self.investigate(value)[1]['status'], 'unresolved')
        send = Mock(return_value=(None, 'timeout'))
        self.assertEqual(r.investigate(REQUEST, self.state, 2, 1, send)[0], 2)
        send.reset_mock()
        self.assertEqual(r.investigate(REQUEST, self.state, 2, .01, send)[1]['status'], 'unresolved-no-time')
        send.assert_not_called()

    def test_configuration_does_not_silently_model_a_different_player(self):
        self.assertEqual(Player('candidate', review='partner-rollout').mode, 'baseline')
        for kwargs in [dict(mode='partner'), dict(n=41), dict(n0=9),
                       dict(inner_belief='voids-counted'), dict(selection='race-refine')]:
            with self.assertRaises(ValueError): Player('bad', review='partner-rollout', **kwargs)


if __name__ == '__main__': unittest.main()
