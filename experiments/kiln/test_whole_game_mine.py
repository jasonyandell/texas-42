import unittest
import whole_game_mine as m


class ContrastTests(unittest.TestCase):
    def test_hard_roots_do_not_masquerade_as_action_interactions(self):
        data=[dict(root=str(group),alternative=4,sample=i,delta=sign,deal=group,size=7)
              for group,sign in enumerate((1,-1)) for i in range(8)]
        values,groups=m.centered(data)
        stats=m.statistics(data,255,values,groups)
        self.assertEqual(stats['matched_delta'],'1')
        self.assertEqual(stats['residual_per_match'],'0')

    def test_duplicate_alternatives_do_not_create_independent_permutation_evidence(self):
        data=[dict(root='root',alternative=4,sample=i,delta=d) for i,d in enumerate((-1,-1,0,0,1,1,1,1))]
        mask=sum(1<<i for i in range(4,8));q=[dict(sign=1)]
        values,_=m.centered(data)
        first=m.permutation_counts(data,q,[mask],values,count=99)
        twice=data+[dict(r,alternative=5) for r in data]
        values,_=m.centered(twice)
        second=m.permutation_counts(twice,q,[mask|(mask<<8)],values,count=99)
        self.assertEqual(first,second)

    def test_no_physical_tile_or_hand_size_enters_expression(self):
        text=m.source((2,True,False,'count=5'))
        self.assertIn('(partner me mate)',text)
        self.assertIn('(out alternative baseline)',text)
        self.assertIn('(not (beats hidden alternative led))',text)
        self.assertNotIn('5-5',text)
        self.assertNotIn('quota',text)


if __name__=='__main__':unittest.main()
