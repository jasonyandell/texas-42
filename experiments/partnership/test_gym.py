"""Independent replay, information boundary, scoring, and resumability gates."""
import copy
from fractions import Fraction
import json
import os
from pathlib import Path
import signal
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch

import gym


class GymTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.cases = dict(gym.gallery(gym.DEFAULT_GALLERY))

    def test_every_published_key_has_exact_support_and_lawful_replays(self):
        self.assertGreaterEqual(len(self.cases), 6)
        for name, case in self.cases.items():
            with self.subTest(name=name):
                audit = gym.verify(case)
                self.assertEqual(audit, case["audit"])

    def test_full_optimal_set_and_set_outcomes_tie_without_point_tiebreak(self):
        key = {"worlds": 10, "actions": [
            {"tile": 1, "success_mass": 6, "score_bins": [7]},
            {"tile": 2, "success_mass": 6, "score_bins": [29]},
            {"tile": 3, "success_mass": 2, "score_bins": [42]}]}
        self.assertTrue(gym.grade(key, 1)["optimal"])
        self.assertTrue(gym.grade(key, 2)["optimal"])
        self.assertEqual(Fraction(gym.grade(key, 3)["regret"]), Fraction(2, 5))
        with self.assertRaises(ValueError):
            gym.grade(key, 4)

    def test_private_teacher_fields_are_refused(self):
        case = next(iter(self.cases.values()))
        for extra in ("hands", "key", "pair", "field_id", "source", "semantics"):
            with self.subTest(extra=extra), self.assertRaises(ValueError):
                gym.pupil_request({**case["request"], extra: case.get(extra)})
        for seed in (-1, 2**64, True, "420600"):
            with self.subTest(seed=seed), self.assertRaises(ValueError):
                gym.pupil_request({**case["request"], "seed": seed})

    def test_tampered_mass_missing_world_or_illegal_trace_is_rejected(self):
        original = next(iter(self.cases.values()))
        for mutation in ("mass", "world", "play", "pair"):
            case = copy.deepcopy(original)
            if mutation == "mass":
                case["key"]["actions"][0]["success_mass"] += 1
            elif mutation == "world":
                case["key"]["actions"][0]["traces"].pop()
            elif mutation == "play":
                case["key"]["actions"][0]["traces"][0]["plays"][1] = 99
            else:
                case["pair"]["gap_mass"] += 1
            with self.subTest(mutation=mutation), self.assertRaises(AssertionError):
                gym.verify(case)

    def test_native_key_agrees_with_published_key(self):
        for name in ("advantage-01", "disadvantage-01"):
            case = self.cases[name]
            key = gym.native(case["request"], max_worlds=400, partner_worlds=40, seconds=15)
            self.assertEqual(case["key"], key)

    def test_manifest_drift_refuses_resume(self):
        with tempfile.TemporaryDirectory() as d:
            directory = Path(d)
            gym.pin(directory, {"v": 1})
            gym.pin(directory, {"v": 1})
            with self.assertRaises(ValueError):
                gym.pin(directory, {"v": 2})
            with gym.run_lock(directory):
                with self.assertRaises(BlockingIOError), gym.run_lock(directory):
                    pass

    def test_report_refuses_foreign_catalog_and_tampered_grade(self):
        with tempfile.TemporaryDirectory() as d, patch("builtins.print"):
            directory = Path(d)
            args = SimpleNamespace(gallery=gym.DEFAULT_GALLERY, results=directory)
            gym.atomic(directory / "manifest.json", {"catalog": "foreign"})
            with self.assertRaises(ValueError):
                gym.report(args)
            gym.atomic(directory / "manifest.json", {"catalog": gym.digest(self.cases), "players": {"pupil": {}}})
            name, case = next(iter(self.cases.items()))
            gym.atomic(directory / "items/bad.json", {"id": name + "--pupil", "scenario": name,
                       "player": "pupil", "response": {"choice": case["key"]["best"][0]},
                       "grade": {"regret": "1"}})
            with self.assertRaises(ValueError):
                gym.report(args)

    def test_failed_item_retries_but_saved_item_is_not_recomputed(self):
        with tempfile.TemporaryDirectory() as d, patch("builtins.print"):
            directory, calls = Path(d), []
            items = [{"id": "a"}, {"id": "b"}]

            def failing(item):
                calls.append(item["id"])
                if item["id"] == "b":
                    raise ValueError("injected failure")
                return item

            gym.bounded(items, failing, directory, 1, 2, 0.1)
            self.assertEqual(calls, ["a", "b"])
            calls.clear()

            def success(item):
                calls.append(item["id"])
                return item

            gym.bounded(items, success, directory, 1, 2, 0.1)
            self.assertEqual(calls, ["b"])
            self.assertEqual(json.loads((directory / "items/a.json").read_text()), {"id": "a"})

    def test_interrupt_drains_current_coordinate_then_resume_completes(self):
        with tempfile.TemporaryDirectory() as d, patch("builtins.print"):
            directory, calls = Path(d), []
            items = [{"id": "a"}, {"id": "b"}, {"id": "c"}]

            def interrupt(item):
                calls.append(item["id"])
                os.kill(os.getpid(), signal.SIGINT)
                return item

            gym.bounded(items, interrupt, directory, 1, 2, 0.1)
            self.assertEqual(calls, ["a"])
            gym.bounded(items, lambda item: item, directory, 2, 2, 0.1)
            self.assertEqual(len(list((directory / "items").glob("*.json"))), 3)


if __name__ == "__main__":
    unittest.main()
