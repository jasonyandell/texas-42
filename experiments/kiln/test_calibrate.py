import json, tempfile, unittest
from pathlib import Path
from calibrate import completion, summary
from kiln import atomic_json, deal

class CalibrationTests(unittest.TestCase):
    def test_fresh_completions_preserve_only_own_hand(self):
        _,hands=deal(420600);seen=set()
        for i in range(100):
            h=completion(hands[2],2,i,'case')
            self.assertEqual(h[2],hands[2]);self.assertEqual(sorted(t for s in h for t in s),list(range(28)))
            self.assertEqual(h,completion(hands[2],2,i,'case'))
            seen.add(json.dumps(h))
        self.assertEqual(len(seen),100)
    def test_summary_does_not_count_unfinished_games(self):
        with tempfile.TemporaryDirectory() as folder:
            p=Path(folder);(p/'games').mkdir()
            for i in range(101):atomic_json(p/'games'/f'{i}.json',{'status':'complete' if i<100 else 'playing','made':i<78})
            r=summary(p,{'id':'case','games':100,'prediction':{'num':78,'den':100},'profile':'test'})
            self.assertEqual(r['completed'],100);self.assertEqual(r['made'],78)
            self.assertLess(r['wilson95'][0],.78);self.assertGreater(r['wilson95'][1],.78)
if __name__=='__main__':unittest.main()
