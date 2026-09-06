"""Campaign invariants; real interruption/restart is also exercised in calibration."""

import itertools
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import campaign as c


class CampaignTests(unittest.TestCase):
    def test_only_make_ranks_and_seed_unit_is_bounded(self):
        for made in itertools.product((False, True), repeat=3):
            rows = [{"made": m, "points": [7, 35]} for m in made]
            p = c.paired(*rows)
            self.assertEqual(p["wins"] + p["losses"] + p["ties"], 2)
            self.assertIn(p["seed_delta"], (-1, 0, 1))
        self.assertEqual(
            c.paired(*[{"made": False, "points": [n, 42 - n]} for n in (7, 29, 1)])[
                "ties"
            ],
            2,
        )

    def test_same_hand_worlds_preserve_information(self):
        spec = {"panel": "worlds", "start": 420600, "worlds_per_hand": 10}
        for group in range(10):
            worlds = [c.fixture(spec, 420600 + 10 * group + i) for i in range(10)]
            first = worlds[0]
            for f in worlds:
                self.assertEqual(sorted(sum(f["hands"], [])), list(range(28)))
                self.assertEqual(
                    (f["bidder"], f["decl"], f["hands"][f["bidder"]]),
                    (first["bidder"], first["decl"], first["hands"][first["bidder"]]),
                )
            self.assertGreater(
                len({tuple(f["hands"][(f["bidder"] + 2) % 4]) for f in worlds}), 1
            )

    def test_atomic_completion_and_identity(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp)
            spec = c.initialize(path, start=420601, count=1)
            self.assertIsNone(c.commit_seed(path, spec, 420601))

            def legal_player(req, **kwargs):
                state = c.information_state(req)
                return {
                    **state,
                    **{k: v for k, v in kwargs.items() if k != "session"},
                    "choice": state["legal"][0],
                    "over_budget": False,
                    "elapsed_us": 1,
                    "route": "legal-fallback",
                    "phases": [{"status": "completed"}],
                }

            with patch("campaign.decide", side_effect=legal_player):
                for arm in c.ARMS:
                    self.assertEqual(c.arm_run(path, 420601, arm, path / "yield"), 0)
            result = c.commit_seed(path, spec, 420601)
            self.assertFalse(result["fresh"])
            self.assertEqual(result["paired"]["ties"], 2)
            self.assertEqual(len(c.complete_results(path, spec)), 1)
            spec["identities"]["native"] = "changed"
            c.atomic(path / "manifest.json", spec)
            with self.assertRaisesRegex(ValueError, "identity"):
                c.load(path)

    def test_early_stopping_and_world_cluster_exclusion(self):
        s = {
            "nonforced_decisions": 100,
            "fallbacks": 0,
            "fresh_seeds": 10,
            "downside_e_value": {"numerator": "6561", "denominator": "256"},
            "fresh_reference_makes": 0,
            "fresh_paired": {"wins": 0, "losses": 8, "ties": 12},
        }
        self.assertEqual(
            c.early_reason(s, {"panel": "random"}), "downside-e-value-at-least-20"
        )
        self.assertIsNone(c.early_reason(s, {"panel": "worlds"}))
        s["fresh_seeds"] = 9
        self.assertIsNone(c.early_reason(s, {"panel": "random"}))
        s["fallbacks"] = 6
        self.assertEqual(
            c.early_reason(s, {"panel": "random"}), "fallback-rate-exceeds-five-percent"
        )


if __name__ == "__main__":
    unittest.main()
