"""Specification laws: identity, parameters, exact filters, and cache/resume."""
import copy
import json
from pathlib import Path
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch

import gym
import gym_spec

SPEC = gym.ROOT / "walt/gym/specs/bid-making.json"


class SpecificationTests(unittest.TestCase):
    def test_reference_is_an_expectation_and_variants_are_explicit(self):
        spec, a, ref = gym_spec.load(SPEC, [])
        self.assertEqual(ref["count"], 433)
        _, equivalent, same = gym_spec.load(SPEC, ["selection.min_spread=0/2"])
        self.assertEqual(a, equivalent)
        self.assertEqual(ref, same)
        _, variant, no_reference = gym_spec.load(SPEC, ["selection.max_optimal=1", "selection.min_spread=1/4"])
        self.assertIsNone(no_reference)
        self.assertEqual(variant["selection"]["max_optimal"], 1)
        with tempfile.TemporaryDirectory() as d:
            changed = Path(d) / "changed.json"
            spec["arguments"]["evaluation"]["partner_worlds"] = 20
            changed.write_text(json.dumps(spec))
            with self.assertRaisesRegex(ValueError, "frozen reference"):
                gym_spec.load(changed, [])

    def test_bad_arguments_refuse_before_running(self):
        for value in ("selection.typo=1", "selection.max_optimal=true", "selection.min_spread=1/0",
                      "evaluation.contract=pi", "domain.tricks=[6,5]", "domain.seed=-1",
                      "source.paths=[]", "selection.certain=1", "selection.min_mistake=-1/4"):
            with self.subTest(value=value), self.assertRaises(ValueError):
                gym_spec.load(SPEC, [value])

    def test_filters_use_exact_outcomes_and_accept_all_tied_best_actions(self):
        _, a, _ = gym_spec.load(SPEC, [])
        s = a["selection"]
        case = dict(key=dict(worlds=12, actions=[dict(tile=1, success_mass=12),
                        dict(tile=2, success_mass=12), dict(tile=3, success_mass=8)]))
        self.assertTrue(gym_spec.accepts(case, s))
        self.assertFalse(gym_spec.accepts(case, {**s, "max_optimal": 1}))
        self.assertTrue(gym_spec.accepts(case, {**s, "min_mistake": "1/3"}))
        self.assertFalse(gym_spec.accepts(case, {**s, "min_mistake": "334/1000"}))
        self.assertFalse(gym_spec.accepts(case, {**s, "certain": True}))
        case["key"]["actions"][2]["success_mass"] = 0
        self.assertTrue(gym_spec.accepts(case, {**s, "certain": True}))

    def test_query_required_compares_against_best_alternative_not_a_weak_one(self):
        # A matching optimal move beating one weak nonmatch is not enough:
        # there must be no equally good nonmatching escape.
        key = dict(worlds=10, offers=[], best=[1, 2], actions=[
            dict(tile=1, success_mass=9), dict(tile=2, success_mass=9),
            dict(tile=3, success_mass=1)])
        case = dict(key=key, target_actions=[1], criterion="query-required")
        self.assertTrue(gym.classify(key, [1]))
        self.assertEqual(gym.case_pairs(case), [])
        key["actions"][1]["success_mass"] = 8
        key["best"] = [1]
        self.assertEqual(gym.query_contrast(key, [1])["gap"], "1/10")
        self.assertTrue(gym.case_pairs(case))
        self.assertEqual(gym.query_contrast(key, [1, 2])["best_targets"], [1])
        key["actions"][1]["success_mass"] = 9
        key["best"] = [1, 2]
        self.assertEqual(gym.query_contrast(key, [1, 2])["best_targets"], [1, 2])
        for targets in ([], [1, 2, 3]):
            self.assertIsNone(gym.query_contrast(key, targets))
            self.assertEqual(gym.case_pairs({**case, "target_actions": targets}), [])
        with self.assertRaises(ValueError):
            gym.query_contrast(key, [99])

    def test_regenerated_exam_identity_ignores_artifact_metadata_but_pins_question_and_key(self):
        original = dict(gym.gallery(gym.DEFAULT_GALLERY))
        rebuilt = copy.deepcopy(original)
        for c in rebuilt.values():
            c["elapsed_seconds"] = 999
            c["provenance"] = {"new_run": True}
        self.assertEqual(gym.exam_identity(original), gym.exam_identity(rebuilt))
        name = next(iter(rebuilt))
        rebuilt[name]["key"]["actions"][0]["success_mass"] += 1
        self.assertNotEqual(gym.exam_identity(original), gym.exam_identity(rebuilt))

    def test_partnership_specification_embeds_the_existing_expression_and_required_contrast(self):
        path = gym.ROOT / "walt/gym/specs/partnership-bid-making.json"
        _, a, ref = gym_spec.load(path, [])
        self.assertEqual(a["query"]["source"], (gym.ROOT / "walt/gym/queries/offer-count.scheme").read_text())
        self.assertEqual(a["selection"]["criterion"], "query-required")
        self.assertEqual(a["selection"]["side"], "declaring")
        self.assertEqual(ref["count"], 30)

    def test_source_identity_is_portable_and_detects_record_edits(self):
        _, a, _ = gym_spec.load(SPEC, [])
        with tempfile.TemporaryDirectory() as d:
            roots = [Path(d) / name for name in ("one", "two")]
            for root in roots:
                root.mkdir()
                (root / "result.json").write_text('{}\n')
                (root / "checkpoint.json").write_text('{}\n')
            hashes = [gym_spec.inputs({**a, "source": {"paths": [str(r)]}})[1] for r in roots]
            self.assertEqual(*hashes)
            (roots[1] / "checkpoint.json").write_text('{"changed":true}\n')
            self.assertNotEqual(hashes[1], gym_spec.inputs({**a, "source": {"paths": [str(roots[1])]}})[1])

    def test_generation_retries_failed_coordinate_and_filters_share_evaluations(self):
        # Two real roots: one unique best, one with two tied certain makes.
        first = dict(gym.gallery(gym.DEFAULT_GALLERY))["advantage-02"]
        second = next(c for n, c in gym.gallery(gym.ROOT / "walt/gym/collections/bid-making-v1")
                      if n == "bid-making-139")
        roots = [{k: copy.deepcopy(c[k]) for k in ("id", "request", "source")} for c in (first, second)]
        spec, _, _ = gym_spec.load(SPEC, [])
        spec["reference"] = None
        spec["arguments"]["domain"]["limit"] = 2
        with tempfile.TemporaryDirectory() as d, patch("builtins.print"):
            path, output = Path(d) / "spec.json", Path(d) / "run"
            path.write_text(json.dumps(spec))
            args = SimpleNamespace(spec=path, output=output, overrides=[], workers=2, seconds=30, case_seconds=15)
            native = gym.native
            failed = False
            calls = []

            def instrument(req, **kwargs):
                nonlocal failed
                if not kwargs.get("inspect"):
                    calls.append(gym.digest(req)[:20])
                    if req == second["request"] and not failed:
                        failed = True
                        raise ValueError("injected evaluation failure")
                return native(req, **kwargs)

            with patch.object(gym_spec, "coordinates", side_effect=lambda *_: iter(copy.deepcopy(roots))), \
                    patch.object(gym_spec, "inputs", return_value=([gym.DEFAULT_SOURCE], "0" * 64)), \
                    patch.object(gym, "native", side_effect=instrument):
                gym_spec.generate(args)
                pending = json.loads((output / "latest.json").read_text())
                self.assertFalse(pending["complete"])
                self.assertEqual(pending["pending"], 1)
                with self.assertRaisesRegex(ValueError, "incomplete"):
                    list(gym.gallery(output))
                gym_spec.generate(args)
                complete = json.loads((output / "latest.json").read_text())
                self.assertTrue(complete["complete"])
                self.assertEqual(complete["result"]["count"], 2)
                self.assertEqual(calls.count(first["id"]), 1)
                self.assertEqual(calls.count(second["id"]), 2)
                calls.clear()
                args.overrides = ["selection.max_optimal=1"]
                gym_spec.generate(args)
                filtered = json.loads((output / "latest.json").read_text())
                self.assertEqual(filtered["result"]["count"], 1)
                self.assertEqual(filtered["resolved"]["evaluation_id"], complete["resolved"]["evaluation_id"])
                self.assertNotEqual(filtered["resolved"]["collection_id"], complete["resolved"]["collection_id"])
                self.assertEqual(calls, [])
                self.assertEqual(len(list(gym.gallery(output))), 1)
                args.overrides += ["selection.certain=true"]
                gym_spec.generate(args)
                self.assertEqual(list(gym.gallery(output)), [])


if __name__ == "__main__":
    unittest.main()
