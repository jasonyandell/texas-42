import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch
import campaign as c
import pool


class PoolTests(unittest.TestCase):
    def test_round_robin_campaign_fairness(self):
        self.assertEqual(list(pool.interleave([[1,2,3],[4,5]])),[1,4,2,5,3])

    def test_fast_later_seed_cannot_enter_early_evidence(self):
        with tempfile.TemporaryDirectory() as tmp:
            p=Path(tmp)
            spec={'start':1,'count':3}
            ready={2,3}
            def commit(path,spec,seed):
                if seed not in ready: return None
                result={'paired':{'seed_delta':1}}
                c.atomic(path/'results'/f'{seed}.json',result)
                return result
            with patch.object(c,'commit_seed',side_effect=commit),patch.object(c,'summarize',return_value={'completed_seeds':3}),patch.object(c,'early_reason',return_value=None):
                self.assertEqual(pool.commit_ready(p,spec),[])
                self.assertFalse((p/'results').exists())
                ready.add(1)
                self.assertEqual(pool.commit_ready(p,spec),[1,2,3])

    def test_pending_queue_skips_completed_arms_and_committed_seeds(self):
        with tempfile.TemporaryDirectory() as tmp:
            p=Path(tmp)
            c.atomic(p/'results/1.json',{})
            c.atomic(p/'seeds/2/phone/result.json',{})
            self.assertEqual(pool.tasks(p,{'start':1,'count':2}),[(p,2,'declaring'),(p,2,'defending')])

    def test_retry_count_is_durable_but_interrupts_are_not_failures(self):
        with tempfile.TemporaryDirectory() as tmp:
            p=Path(tmp)
            for i,status in enumerate(['failed','running','interrupted','completed','failed']):
                c.atomic(p/'pool-attempts'/f'{i}.json',{'status':status})
            self.assertEqual(pool.failed_attempts(p),2)


if __name__=='__main__': unittest.main()
