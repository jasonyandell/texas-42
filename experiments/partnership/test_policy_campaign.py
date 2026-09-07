"""Durability and orchestration gates for policy_campaign."""

import json
import os
from pathlib import Path
from types import SimpleNamespace
import tempfile
import unittest
from unittest import mock

import policy_campaign as campaign
from process_groups import kill_process_group


FAKE = r'''#!/usr/bin/env python3
import argparse, json, os, pathlib, time
p = argparse.ArgumentParser()
p.add_argument("--seed", type=int, required=True)
p.add_argument("--tiles", type=int, required=True)
p.add_argument("--samples", required=True)
p.add_argument("--test-worlds", type=int, required=True)
p.add_argument("--node-budget", type=int, required=True)
p.add_argument("--decl", type=int, required=True)
p.add_argument("--mode", required=True)
p.add_argument("--field", required=True)
p.add_argument("--artifact-dir", required=True)
p.add_argument("--hand")
a = p.parse_args()
pathlib.Path(a.artifact_dir).mkdir(parents=True, exist_ok=True)
(pathlib.Path(a.artifact_dir) / "policy.txt").write_text(str(a.seed))
if os.environ.get("POLICY_FAKE_SLEEP"):
    time.sleep(float(os.environ["POLICY_FAKE_SLEEP"]))
if os.environ.get("POLICY_FAKE_BAD"):
    print("bad json")
else:
    rows = []
    for n in map(int, a.samples.split(",")):
        for arm in ("fresh", "persistent", "compose"):
            makes = n - 1 if os.environ.get("POLICY_FAKE_MISMATCH") and arm == "persistent" else n
            rows.append(dict(samples=n, arm=arm, train_makes=makes, train_worlds=n,
                test_makes=n % (a.test_worlds + 1), test_worlds=a.test_worlds,
                nodes=n, cache_hits=0, policy_states=1, elapsed_ms=1,
                solved_samples=n, policy_id=("same" if arm != "compose" else "compose"),
                status="completed"))
    print(json.dumps(dict(schema="policy-lab-v1", seed=a.seed, tiles=a.tiles,
        decl=a.decl, field=("hash-legal-v1" if a.field == "hash-legal" else "field:level0-n8-v1"),
        hand=list(range(7)), rows=rows)))
'''


class PolicyCampaignTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.binary = self.root / "fake_policy_lab.py"
        self.binary.write_text(FAKE)
        self.binary.chmod(0o755)

    def tearDown(self):
        self.temp.cleanup()

    def initialize(self, **changes):
        values = dict(output=self.root / "campaign", binary=self.binary,
            mode="random-own-hand", field="hash-legal", seeds=[10, 11], tiles=2, samples=[1, 2],
            test_worlds=5, node_budget=100, decl=6, hand=None,
            seed_timeout=2.0, workers=2)
        values.update(changes)
        return campaign.initialize(SimpleNamespace(**values))

    def test_complete_results_resume_without_recomputation(self):
        manifest = self.initialize()
        self.assertEqual(campaign.run(self.root / "campaign", 5), 0)
        paths = [campaign.result_path(self.root / "campaign", seed)
                 for seed in manifest["configuration"]["seeds"]]
        mtimes = [path.stat().st_mtime_ns for path in paths]
        self.assertEqual(campaign.run(self.root / "campaign", 5), 0)
        self.assertEqual(mtimes, [path.stat().st_mtime_ns for path in paths])
        status = json.loads((self.root / "campaign/status.json").read_text())
        self.assertEqual(status["completed_seeds"], 2)
        self.assertEqual(status["arms"]["fresh"]["test_makes"]["n"], 4)
        self.assertEqual(status["schedule_totals"]["persistent_minus_fresh_nodes"], 0)
        self.assertEqual(status["paired_by_samples"]["1"]["persistent_policy_mismatches"], 0)

    def test_corrupt_completed_seed_refuses_resume(self):
        manifest = self.initialize(seeds=[10])
        path = campaign.result_path(self.root / "campaign", 10)
        campaign.atomic(path, {"schema": campaign.RESULT_SCHEMA,
                               "campaign": manifest["id"], "seed": 999,
                               "result": {}})
        with self.assertRaisesRegex(ValueError, "wrong campaign or seed"):
            campaign.run(self.root / "campaign", 5)

    def test_failed_attempt_never_becomes_a_complete_seed(self):
        self.initialize(seeds=[10])
        old = os.environ.get("POLICY_FAKE_BAD")
        os.environ["POLICY_FAKE_BAD"] = "1"
        try:
            self.assertEqual(campaign.run(self.root / "campaign", 5), 75)
        finally:
            if old is None:
                os.environ.pop("POLICY_FAKE_BAD", None)
            else:
                os.environ["POLICY_FAKE_BAD"] = old
        self.assertFalse(campaign.result_path(self.root / "campaign", 10).exists())
        attempt = json.loads((self.root / "campaign/seeds/10/attempt.json").read_text())
        self.assertEqual(attempt["returncode"], 0)
        self.assertIn("bad json", attempt["stdout"])
        self.assertEqual(campaign.run(self.root / "campaign", 5), 0)

    def test_timeout_is_recorded_and_process_is_reaped(self):
        self.initialize(seeds=[10], seed_timeout=0.05)
        old = os.environ.get("POLICY_FAKE_SLEEP")
        os.environ["POLICY_FAKE_SLEEP"] = "2"
        try:
            self.assertEqual(campaign.run(self.root / "campaign", 2), 75)
        finally:
            if old is None:
                os.environ.pop("POLICY_FAKE_SLEEP", None)
            else:
                os.environ["POLICY_FAKE_SLEEP"] = old
        attempt = json.loads((self.root / "campaign/seeds/10/attempt.json").read_text())
        self.assertTrue(attempt["timed_out"])
        self.assertLess(attempt["elapsed_seconds"], 1)
        self.assertFalse(campaign.result_path(self.root / "campaign", 10).exists())

    def test_exact_persistent_training_mismatch_is_not_committed(self):
        self.initialize(seeds=[10])
        old = os.environ.get("POLICY_FAKE_MISMATCH")
        os.environ["POLICY_FAKE_MISMATCH"] = "1"
        try:
            self.assertEqual(campaign.run(self.root / "campaign", 5), 75)
        finally:
            if old is None:
                os.environ.pop("POLICY_FAKE_MISMATCH", None)
            else:
                os.environ["POLICY_FAKE_MISMATCH"] = old
        self.assertFalse(campaign.result_path(self.root / "campaign", 10).exists())
        status = json.loads((self.root / "campaign/status.json").read_text())
        self.assertIn("exact persistent value differs", status["failures"]["10"])

    def test_binary_and_manifest_drift_are_rejected(self):
        self.initialize(seeds=[10])
        self.binary.write_text(FAKE + "\n# changed\n")
        with self.assertRaisesRegex(ValueError, "binary fingerprint changed"):
            campaign.load(self.root / "campaign")
        manifest_path = self.root / "campaign/manifest.json"
        manifest = json.loads(manifest_path.read_text())
        manifest["configuration"]["samples"] = [99]
        campaign.atomic(manifest_path, manifest)
        with self.assertRaisesRegex(ValueError, "manifest is corrupt"):
            campaign.load(self.root / "campaign", verify=False)

    def test_configuration_boundaries(self):
        with self.assertRaisesRegex(ValueError, "between 1 and 10"):
            self.initialize(workers=11)
        with self.assertRaisesRegex(ValueError, "requires exactly seven"):
            self.initialize(mode="fixed-root-hand", hand=None)
        with self.assertRaisesRegex(ValueError, "seven distinct"):
            self.initialize(mode="fixed-root-hand", hand=[0] * 7)

    def test_permission_race_is_suppressed_only_after_child_exit(self):
        exited = SimpleNamespace(pid=123, poll=mock.Mock(side_effect=[None, 0]))
        with mock.patch("process_groups.os.killpg", side_effect=PermissionError(1, "denied")):
            self.assertFalse(kill_process_group(exited))

        live = SimpleNamespace(pid=124, poll=mock.Mock(return_value=None))
        with mock.patch("process_groups.os.killpg", side_effect=PermissionError(1, "denied")):
            with self.assertRaises(PermissionError):
                kill_process_group(live)


if __name__ == "__main__":
    unittest.main()
