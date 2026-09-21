from copy import deepcopy
from dataclasses import asdict
from pathlib import Path
import tempfile
import threading
import unittest
from unittest.mock import patch

import campaign as c
from gym_replay import ReplayExperiment, full_hands, paired
from matchup import Player
from rules import information_state

PANEL = Path(__file__).resolve().parent / 'campaigns/sunshine-gym-replay-v1/panel.json'


class ReplayTests(unittest.TestCase):
    def experiment(self, path):
        experiment = ReplayExperiment.__new__(ReplayExperiment)
        experiment.output = Path(path)
        experiment.configs = [Player('l1-default'), Player('l1-partner-count-review', review='partner-count')]
        experiment.panel = {'roots': [deepcopy(c.read(PANEL)['roots'][0])]}
        experiment.lock = threading.Lock()
        experiment.locks = {}
        return experiment

    def fake(self, request, **kwargs):
        self.assertEqual(set(request), {'decl', 'bid', 'bidder', 'seat', 'hand', 'plays', 'seed'})
        legal = information_state(request)['legal']
        # Different lawful procedures make erroneous configuration sharing visible.
        choice = legal[-1] if kwargs['review'] == 'partner-count' else legal[0]
        return dict(choice=choice, over_budget=False, elapsed_us=0)

    def test_paired_counts_keep_harmed_worlds_despite_positive_net_gain(self):
        self.assertEqual(paired([False, False, True, True], [True, True, False, True]),
                         dict(before=2, after=3, worlds=4, wins=2, losses=1, ties=1, delta='1/4'))

    def test_jobs_cover_every_world_action_and_review_once(self):
        with tempfile.TemporaryDirectory() as path:
            experiment = self.experiment(path)
            root = experiment.panel['roots'][0]
            jobs = experiment.jobs()
            expected = {(w, 'action', a) for w in range(len(root['worlds'])) for a in root['legal']}
            expected |= {(w, 'review', None) for w in range(len(root['worlds']))}
            self.assertEqual({(j['world'], j['kind'], j['action']) for j in jobs}, expected)
            self.assertEqual(len(jobs), len(expected))
            self.assertEqual(len({j['id'] for j in jobs}), len(jobs))

    def test_all_arms_replay_and_cached_decisions_survive_resume(self):
        with tempfile.TemporaryDirectory() as path, patch('sunshine_worlds.decide', side_effect=self.fake) as decide:
            experiment = self.experiment(path)
            for job in [j for j in experiment.jobs() if j['world'] == 0]:
                saved = experiment.play(job)
                row, _ = experiment.audit_job(job, saved)
                self.assertEqual(len(row['record']), 56)
                self.assertEqual(sum(row['points']), 42)
                calls = decide.call_count
                resumed = self.experiment(path)
                self.assertEqual(resumed.play(job), saved)
                self.assertEqual(decide.call_count, calls)

    def test_forced_action_cannot_be_relabelled_as_deployed_review(self):
        with tempfile.TemporaryDirectory() as path, patch('sunshine_worlds.decide', side_effect=self.fake):
            experiment = self.experiment(path)
            job = next(j for j in experiment.jobs() if j['kind'] == 'action')
            saved = experiment.play(job)
            saved['payload']['kind'] = 'review'
            saved['payload_sha256'] = c.digest(saved['payload'])
            with self.assertRaises(AssertionError): experiment.audit_job(job, saved)

    def test_auditor_rejects_wrong_world_even_with_recomputed_payload_hash(self):
        with tempfile.TemporaryDirectory() as path, patch('sunshine_worlds.decide', side_effect=self.fake):
            experiment = self.experiment(path)
            job = experiment.jobs()[0]
            saved = experiment.play(job)
            root = experiment.panel['roots'][0]
            saved['payload']['hands'] = full_hands(root['request'], root['worlds'][1])
            saved['payload_sha256'] = c.digest(saved['payload'])
            with self.assertRaises(AssertionError): experiment.audit_job(job, saved)

    def test_player_configuration_scope_and_missing_report(self):
        with tempfile.TemporaryDirectory() as path:
            experiment = self.experiment(path)
            self.assertEqual(experiment.config('action', 0, 0), experiment.configs[0])
            self.assertEqual(experiment.config('review', 0, 0), experiment.configs[1])
            self.assertEqual(experiment.config('review', 2, 0), experiment.configs[1])
            self.assertEqual(experiment.config('review', 1, 0), experiment.configs[0])
            self.assertIsNone(experiment.report())

    def test_manifest_roundtrip_resumes_but_changed_panel_is_refused(self):
        with tempfile.TemporaryDirectory() as path:
            output = Path(path) / 'output'
            output.mkdir()
            first = ReplayExperiment(PANEL, output)
            self.assertEqual(first.manifest, ReplayExperiment(PANEL, output).manifest)
            altered = Path(path) / 'altered.json'
            data = c.read(PANEL)
            data['roots'][0]['request']['seed'] += 1
            c.atomic(altered, data)
            with self.assertRaises(ValueError): ReplayExperiment(altered, output)


if __name__ == '__main__':
    unittest.main()
