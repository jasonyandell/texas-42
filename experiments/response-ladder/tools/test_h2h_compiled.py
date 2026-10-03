#!/usr/bin/env python3
"""Focused tests for the compiled H2H harness without running games."""

import json
from pathlib import Path
import sys
import tempfile
import unittest
from unittest import mock

import h2h_compiled as harness
sys.path.insert(0, "/Users/jason/code/texas-42-partnership-launch/experiments/partnership")
import rules
from h2h_compiled import CandidateWorker, extract_actor_paths, fixture_panel, load_config


class CompiledH2HTest(unittest.TestCase):
    def test_fixture_panel_has_balanced_pairgroups(self):
        fixtures = fixture_panel("test-panel", 2)
        self.assertEqual(len(fixtures), 72)
        self.assertEqual(len({fixture["pairgroup"] for fixture in fixtures}), 36)
        for pairgroup in {fixture["pairgroup"] for fixture in fixtures}:
            pair = [fixture for fixture in fixtures if fixture["pairgroup"] == pairgroup]
            self.assertEqual([fixture["balanced_order"] for fixture in pair],
                             [["declaring", "defending"], ["defending", "declaring"]])
        self.assertEqual(load_config(None)[0], {"outer": 40, "plans": 1, "horizon": 7, "work": 2_000_000})

    def test_fake_persistent_candidate_worker(self):
        with tempfile.TemporaryDirectory() as temporary:
            c0 = Path(temporary) / "c0.json"
            c1 = Path(temporary) / "c1.json"
            c0.write_text("{}")
            c1.write_text("{}")
            code = (
                "import json,sys; print(json.dumps({'ready':True}),flush=True); "
                "[print(json.dumps({'tile':json.loads(line)['hand'][0],'fallback':False,'reason':'fake'}),flush=True) for line in sys.stdin]"
            )
            command, got_c0, got_c1 = extract_actor_paths(
                [sys.executable, "-u", "-c", code], str(c0), str(c1))
            self.assertEqual(got_c0, c0.resolve())
            self.assertEqual(got_c1, c1.resolve())
            worker = CandidateWorker(command, 2.0)
            try:
                self.assertTrue(worker.start()["ready"])
                response = worker.call({"hand": [4, 5], "budget_ms": 5})
                self.assertEqual(response["tile"], 4)
            finally:
                worker.close()

    def test_budget_pause_resume_matches_uninterrupted_moves(self):
        fixture = fixture_panel("pause-test", 1)[0]

        class FakeCandidate:
            def __init__(self):
                self.calls = 0

            def call(self, request):
                self.calls += 1
                clock[0] += 1.0
                record = [value for pair in request["history"] for value in pair]
                points, leader, remaining, trick = rules.replay_record(fixture["hands"], record, fixture["decl"], fixture["bidder"])
                legal = rules.legal_tiles(remaining[request["seat"]], trick, fixture["decl"])
                return {"tile": legal[0], "fallback": False, "reason": "fake"}

        class FakeCPU:
            def decide(self, request, **kwargs):
                clock[0] += 1.0
                record = request["plays"]
                points, leader, remaining, trick = rules.replay_record(fixture["hands"], record, fixture["decl"], fixture["bidder"])
                legal = rules.legal_tiles(remaining[request["seat"]], trick, fixture["decl"])
                return {"choice": legal[0], "route": "fixed"}

        clock = [0.0]

        def monotonic():
            return clock[0]

        with tempfile.TemporaryDirectory() as temporary, mock.patch.object(harness.time, "monotonic", side_effect=monotonic):
            partial = Path(temporary) / "0-declaring.partial.json"
            paused_candidate = FakeCandidate()
            with self.assertRaises(harness.BudgetPause):
                harness.play(fixture, "declaring", paused_candidate, FakeCPU(), object(), rules,
                             5, {}, 18.0, partial)
            saved = json.loads(partial.read_text())
            self.assertGreater(len(saved["moves"]), 0)
            self.assertLess(len(saved["moves"]), 28)
            resumed = harness.play(fixture, "declaring", paused_candidate, FakeCPU(), object(), rules,
                                   5, {}, 1000.0, partial)

            clock[0] = 0.0
            uninterrupted = harness.play(fixture, "declaring", FakeCandidate(), FakeCPU(), object(), rules,
                                         5, {}, 1000.0, Path(temporary) / "uninterrupted.partial.json")
            self.assertEqual(resumed["record"], uninterrupted["record"])
            self.assertEqual(resumed["moves"], uninterrupted["moves"])


if __name__ == "__main__":
    unittest.main()
