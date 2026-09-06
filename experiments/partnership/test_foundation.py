import copy
import sys
import tempfile
import unittest
from dataclasses import asdict
from pathlib import Path
from unittest.mock import patch

import campaign as c
import pool
from matchup import Player, pair
from player import decide
from runtime import DecisionSession


class FoundationTests(unittest.TestCase):
    def test_make_set_score_and_seat_swap(self):
        for a in [False, True]:
            for b in [False, True]:
                score = pair(a, b)
                self.assertEqual(score["seed_delta"], -pair(b, a)["seed_delta"])
                self.assertEqual(score["a_contract_wins"] + score["b_contract_wins"], 2)
                self.assertEqual(score["ties"], int(a == b))
        spec = {
            "players": {
                "a": asdict(
                    Player("a", inner_belief="voids-counted", selection="refine")
                ),
                "b": asdict(Player("b", mode="partner", modeled_selection="refine")),
            }
        }
        for bidder in range(4):
            for arm in c.arms_for(spec):
                ps = c.players_for(spec, arm, bidder)
                self.assertEqual(ps[bidder].name, "a" if arm == "declaring" else "b")
                self.assertEqual(ps[bidder], ps[(bidder + 2) % 4])
                self.assertNotEqual(
                    ps[bidder].inner_belief, ps[(bidder + 1) % 4].inner_belief
                )

    def test_configuration_rejects_silent_nonfeatures(self):
        for kw in [
            {"mode": "oops"},
            {"mode": "phone", "selection": "refine"},
            {"mode": "phone", "inner_belief": "voids-counted"},
            {"modeled_selection": "refine"},
            {"n": True},
            {"n1": 0},
            {"budget_ms": 15000},
        ]:
            with self.assertRaises(ValueError):
                Player("x", **kw)

    def test_panel_reuse_and_fixed_hand_grouping(self):
        with tempfile.TemporaryDirectory() as tmp:
            x, y = Path(tmp) / "x", Path(tmp) / "y"
            a, b = asdict(Player("a")), asdict(Player("b", selection="refine"))
            sx = c.initialize_match(x, a, b, 770000, 12, "worlds", 3)
            sy = c.initialize_match(y, b, a, 770000, 12, "worlds", 3)
            self.assertEqual(sx["panel_id"], sy["panel_id"])
            self.assertEqual(len(pool.tasks(x, sx)), 24)
            for seed in range(770000, 770012):
                self.assertEqual(c.fixture(sx, seed), c.fixture(sy, seed))
                f = c.fixture(sx, seed)
                g = c.fixture(sx, 770000 + f["group"] * 3)
                self.assertEqual(f["hands"][f["bidder"]], g["hands"][g["bidder"]])
                self.assertEqual((f["bidder"], f["decl"]), (g["bidder"], g["decl"]))

    def test_checkpoint_rejects_config_and_move_corruption(self):
        with tempfile.TemporaryDirectory() as tmp:
            p = Path(tmp)
            a = asdict(Player("a"))
            b = asdict(Player("b"))
            spec = c.initialize_match(p, a, b, 880000, 1)
            f = c.fixture(spec, 880000)
            seat = f["bidder"]
            req = {
                "decl": f["decl"],
                "bid": 30,
                "bidder": seat,
                "seat": seat,
                "hand": f["hands"][seat],
                "plays": [],
                "seed": 420600,
            }
            # An injected worker failure deliberately supplies the documented
            # legal fallback; this tests stored policy provenance, not strength.
            with patch("player.child", return_value=(None, "injected")):
                response = decide(req, mode="baseline")
            snap = {
                "campaign": spec["id"],
                "seed": 880000,
                "arm": "declaring",
                "decisions": [{"seat": seat, "response": response}],
            }
            c.validate_checkpoint(snap, spec, f, 880000, "declaring")
            for key, value in [
                ("choice", 99),
                ("selection", "refine"),
                ("points", [42, 0]),
            ]:
                bad = copy.deepcopy(snap)
                bad["decisions"][0]["response"][key] = value
                with self.assertRaises(AssertionError):
                    c.validate_checkpoint(bad, spec, f, 880000, "declaring")

    def test_timeout_kills_worker_and_next_request_restarts(self):
        code = "import sys,time,json\nfor line in sys.stdin:\n if line.strip():\n  if line.strip()=='slow': time.sleep(5)\n  print(json.dumps({'value':line.strip()}),flush=True)"
        command = [sys.executable, "-u", "-c", code]
        with DecisionSession() as session:
            result, status = session.call(command, "slow", 0.1)
            self.assertIsNone(result)
            self.assertEqual(status, "timeout")
            self.assertEqual(len(session.workers), 0)
            result, status = session.call(command, "fast", 2)
            self.assertEqual((result, status), ({"value": "fast"}, "completed"))
            worker = next(iter(session.workers.values()))
            pid = worker.process.pid
            result, status = session.call(command, "again", 2)
            self.assertEqual(worker.process.pid, pid)
            self.assertEqual(result, {"value": "again"})
        self.assertIsNotNone(worker.process.poll())


if __name__ == "__main__":
    unittest.main()
