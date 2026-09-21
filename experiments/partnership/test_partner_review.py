import unittest
from unittest.mock import Mock

from partner_review import ID, investigate, offers
from matchup import Player
from rules import information_state

REQUEST = dict(decl=3, bid=30, bidder=0, seat=2, hand=[0,2,3,15,17,21,25],
               plays=[0,7,1,24,2,3,3,6,1,5,2,17,3,23,0,12,1,4,2,21,3,22,0,8,0,9,1,19,2,25,3,18,0,13,1,16], seed=420600)


class ReviewTests(unittest.TestCase):
    def setUp(self):
        self.state = information_state(REQUEST)

    def answer(self, values=None, choice=15, status="changed"):
        return dict(schema=ID, baseline=2, choice=choice, offers=[15], worlds=210,
                    values=values if values is not None else [[2,131],[15,153]], status=status)

    def test_public_gate_and_already_offered_count_do_not_call_worker(self):
        self.assertEqual(offers(REQUEST, self.state), [15])
        send = Mock()
        choice, result = investigate(REQUEST, self.state, 15, 1, send)
        self.assertEqual(choice, 15)
        self.assertEqual(result['status'], 'inactive')
        send.assert_not_called()

    def test_completed_comparison_changes_choice_and_limits_allowance(self):
        send = Mock(return_value=(self.answer(), 'completed'))
        choice, result = investigate(REQUEST, self.state, 2, 10, send)
        self.assertEqual(choice, 15)
        self.assertEqual(result['status'], 'changed')
        self.assertLessEqual(send.call_args.args[2], .250)
        self.assertNotIn('hands ', send.call_args.args[1])

    def test_tie_preserves_baseline(self):
        send = Mock(return_value=(self.answer([[2,153],[15,153]],2,'retained'), 'completed'))
        self.assertEqual(investigate(REQUEST,self.state,2,1,send)[0],2)

    def test_timeout_and_no_time_preserve_completed_baseline(self):
        send = Mock(return_value=(None,'timeout'))
        self.assertEqual(investigate(REQUEST,self.state,2,1,send)[1]['status'],'unresolved')
        send.reset_mock()
        self.assertEqual(investigate(REQUEST,self.state,2,0,send)[0],2)
        send.assert_not_called()

    def test_incomplete_illegal_and_inconsistent_answers_are_rejected(self):
        for value in [None, {}, self.answer([[2,131]]), self.answer([[2,131],[15,153]],27),
                      self.answer([[2,131],[15,153],[15,153]]), self.answer([[2,131],[15,211]]),
                      self.answer([[2,153],[15,153]],15,'changed')]:
            send = Mock(return_value=(value,'completed'))
            choice, result = investigate(REQUEST,self.state,2,1,send)
            self.assertEqual(choice,2)
            self.assertTrue(result['status'].startswith('unresolved'))

    def test_refusal_is_not_negative_evidence(self):
        value = self.answer([],2,'unresolved-world-cap')
        value['worlds'] = 401
        choice, result = investigate(REQUEST,self.state,2,1,Mock(return_value=(value,'completed')))
        self.assertEqual(choice,2)
        self.assertEqual(result['status'],'unresolved-world-cap')

    def test_player_configuration_separates_review_from_mind_profile(self):
        self.assertEqual(Player('base').review,'off')
        self.assertEqual(Player('candidate',review='partner-count').mode,'baseline')
        with self.assertRaises(ValueError): Player('bad',mode='phone',review='partner-count')


if __name__ == '__main__': unittest.main()
