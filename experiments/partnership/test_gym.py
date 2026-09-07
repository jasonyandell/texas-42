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


    def test_declarative_offer_matches_legacy_detector_on_all_original_mining_roots(self):
        query = gym.ROOT / "walt/gym/queries/offer-count.scheme"
        roots = json.loads((gym.ROOT / "walt/gym/mining.json").read_text())["manifest"]["candidates"]
        for item in roots:
            found = gym.native(item["request"], inspect=True, query=query, seconds=3)
            self.assertTrue(found["query_match"]["public"])
            self.assertEqual(found["query_match"]["presence"], [[t, found["worlds"]] for t in found["offers"]])

    def test_generic_stream_includes_positions_outside_original_pattern(self):
        source = gym.DEFAULT_SOURCE
        legacy = {c["id"] for c in gym.candidates(source)}
        general = list(gym.positions([source]))
        self.assertTrue(legacy <= {c["id"] for c in general})
        self.assertGreater(len(general), len(legacy))
        for item in general[:10]:
            self.assertEqual(set(item["request"]), gym.INPUT_KEYS)

    def test_generic_targets_control_pair_categories(self):
        key = {"offers": [], "best": [1], "actions": [
            {"tile": 1, "success_mass": 4}, {"tile": 2, "success_mass": 1}]}
        self.assertEqual(gym.classify(key), [])
        self.assertEqual(gym.classify(key, [1])[0]["category"], "advantage")
        self.assertEqual(gym.classify(key, [2])[0]["category"], "disadvantage")
        self.assertEqual(gym.classify(key, [1, 2]), [])

    def test_outcome_geometry_retains_ties_and_distinguishes_certainty(self):
        key = {"worlds": 10, "offers": [3], "best": [1, 2], "actions": [
            {"tile": 1, "success_mass": 10, "score_bins": [30]},
            {"tile": 2, "success_mass": 10, "score_bins": [42]},
            {"tile": 3, "success_mass": 9, "score_bins": [41]},
            {"tile": 4, "success_mass": 0, "score_bins": [29]}]}
        p = gym.outcome_profile(key)
        self.assertEqual(p["optimal"], [1, 2])
        self.assertEqual(p["nearest_mistake"], "1/10")
        self.assertEqual(p["spread"], "1")
        self.assertFalse(p["unique_best"])
        self.assertTrue(p["certain_success_failure_swing"])
        case = dict(key=key, request=dict(seat=0, bidder=2), criterion="outcome", target_actions=[1, 2, 3, 4])
        self.assertEqual(len(gym.case_pairs(case)), 4)
        self.assertEqual(gym.case_pairs(case)[0]["category"], "bid-making")
        case["request"]["seat"] = 1
        self.assertEqual(gym.case_pairs(case)[0]["category"], "bid-setting")
        for mass in (0, 7, 10):
            for a in key["actions"]:
                a["success_mass"] = mass
            self.assertFalse(gym.outcome_profile(key)["strict"])
            self.assertIsNone(gym.outcome_profile(key)["nearest_mistake"])
            self.assertEqual(gym.case_pairs(case), [])

    def test_outcome_discovery_publication_and_resume_use_same_full_key(self):
        roots = [{k: c[k] for k in ("id", "request", "source")} for c in self.cases.values()]
        with tempfile.TemporaryDirectory() as d, patch("builtins.print"):
            raw, published = Path(d) / "raw", Path(d) / "gallery"
            args = SimpleNamespace(query=gym.ROOT / "walt/gym/queries/all-legal.scheme",
                source=[gym.DEFAULT_SOURCE], min_trick=5, max_trick=6, limit=6,
                max_worlds=400, partner_worlds=40, min_presence="1", case_seconds=15,
                workers=2, seconds=30, output=raw)
            with patch.object(gym, "positions", return_value=iter(roots)):
                gym.discover(args)
            self.assertEqual(len(list((raw / "items").glob("*.json"))), 6)
            with patch.object(gym, "positions", return_value=iter(roots)), patch.object(gym, "native", side_effect=AssertionError("completed item recomputed")):
                gym.discover(args)
            gym.select(SimpleNamespace(source=raw, output=published, all=True, each=3,
                                       criterion="outcome", side="declaring"))
            cases = list(gym.gallery(published))
            self.assertTrue(cases)
            original = {c["id"]: c for c in self.cases.values()}
            for _, c in cases:
                self.assertEqual(c["key"], original[c["id"]]["key"])
                self.assertEqual(gym.side_of(c), "declaring")
                self.assertEqual(c["target_actions"], c["key"]["legal"])
                self.assertEqual(set(gym.pupil_request(c["request"])), gym.INPUT_KEYS)
                gym.verify(c)
                c["outcome"]["spread"] = "999"
                with self.assertRaises(AssertionError):
                    gym.verify(c)
            self.assertEqual(len(json.loads((published / "discovery.json").read_text())["rows"]), 6)

    def test_published_bid_making_witness_is_certain_across_all_sixty_worlds(self):
        base = gym.ROOT / "walt/gym/collections/bid-making-v1"
        case = next(c for n, c in gym.gallery(base) if n == "bid-making-17")
        self.assertEqual(case["key"]["worlds"], 60)
        self.assertEqual(case["outcome"]["optimal"], [3])  # 2-0, trump
        self.assertEqual(case["outcome"]["guaranteed_failure"], [15, 18])
        self.assertEqual(case["paired_outcomes"], dict(gained=60, lost=0, both_success=0, both_failure=0))
        self.assertEqual(gym.native(case["request"], max_worlds=60, seconds=15), case["key"])
        gym.verify(case)
        case["paired_outcomes"]["gained"] -= 1
        with self.assertRaises(AssertionError):
            gym.verify(case)

    def test_published_discovery_is_distinct_by_coordinate_and_queries_broaden_it(self):
        base = gym.ROOT / "walt/gym/collections/scheme-v1"
        cases = dict(gym.gallery(base))
        self.assertEqual(len(cases), 170)
        self.assertEqual(len({c["id"] for c in cases.values()}), 170)
        self.assertTrue(any(not c["key"]["offers"] and c["target_actions"] for c in cases.values()))
        # The declared pattern is the selection authority; query changes must
        # refuse resume even when all other experiment inputs are identical.
        with tempfile.TemporaryDirectory() as d:
            gym.pin(Path(d), {"query_source": "a"})
            with self.assertRaises(ValueError):
                gym.pin(Path(d), {"query_source": "b"})

    def test_fractional_partner_boss_query_keeps_the_entire_grading_belief(self):
        base = gym.ROOT / "walt/gym/collections/scheme-v1/lead-to-partner-boss"
        case = next(c for _, c in gym.gallery(base) if c["id"] == "1bdec02c98fb0deb9df4")
        self.assertFalse(case["query_match"]["public"])
        self.assertEqual(case["query_match"]["presence"], [[21, 40], [25, 40]])
        self.assertEqual(case["key"]["worlds"], 140)
        self.assertEqual(gym.native(case["request"], max_worlds=140, seconds=15), case["key"])
        gym.verify(case)

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
