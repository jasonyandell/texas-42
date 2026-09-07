"""Schema and information-boundary gates for the policy gym campaign."""

import copy
import json
from pathlib import Path
import unittest

import gym
import policy_gym


GALLERY = Path("/Users/jason/data/texas-42/partnership-bid-making-v1")


class PolicyGymTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.name, cls.case = next(iter(gym.gallery(GALLERY)))

    def native(self):
        case = self.case
        action = case["key"]["best"][0]
        best = max(item["success_mass"] for item in case["key"]["actions"])
        rows = []
        for samples in policy_gym.SAMPLES:
            for arm in ("fresh", "persistent", "compose"):
                rows.append({"samples": samples, "arm": arm, "status": "completed",
                    "policy_id": arm, "root_action": action,
                    "exact_policy_makes": best,
                    "exact_policy_worlds": case["key"]["worlds"], "nodes": 1})
        request = self.case["request"]
        return {"schema": "policy-lab-v1", "seed": request["seed"],
            "decl": request["decl"], "hand": sorted(request["hand"]),
            "field": self.case["key"]["field_id"],
            "root_worlds": self.case["key"]["worlds"],
            "root_history": [request["plays"][i:i+2]
                             for i in range(0, len(request["plays"]), 2)],
            "rows": rows}

    def test_request_contains_only_the_seven_public_fields(self):
        lines = policy_gym.request_text(self.case["request"]).splitlines()
        self.assertEqual({line.split()[0] for line in lines}, gym.INPUT_KEYS)
        self.assertNotIn("key", policy_gym.request_text(self.case["request"]))
        private = {**self.case["request"], "hands": [[0]]}
        with self.assertRaises(ValueError):
            policy_gym.request_text(private)

    def test_case_assessment_separates_root_regret_and_full_policy_gap(self):
        result = policy_gym.assess_case(self.name, self.case, self.native())
        self.assertEqual(result["schema"], "texas42-policy-gym-case-v1")
        self.assertEqual(len(result["rows"]), 12)
        self.assertTrue(all(row["root_action_optimal"] for row in result["rows"]))
        self.assertTrue(all(row["root_action_regret"] == "0" for row in result["rows"]))
        self.assertTrue(all("full_policy_gap_to_best_root_q" in row
                            for row in result["rows"]))

    def test_malformed_or_incomplete_native_case_is_rejected(self):
        native = self.native()
        for mutation in ("schema", "rows", "action", "worlds"):
            broken = copy.deepcopy(native)
            if mutation == "schema": broken["schema"] = "wrong"
            elif mutation == "rows": broken["rows"].pop()
            elif mutation == "action": broken["rows"][0]["root_action"] = 99
            else: broken["rows"][0]["exact_policy_worlds"] += 1
            with self.subTest(mutation=mutation), self.assertRaises(ValueError):
                policy_gym.assess_case(self.name, self.case, broken)

    def test_default_gallery_is_the_thirty_case_existing_panel(self):
        cases = list(gym.gallery(GALLERY))
        self.assertEqual(len(cases), 30)
        self.assertEqual(len({name for name, _ in cases}), 30)
        for name, case in cases:
            identity = policy_gym.case_identity(name, case)
            self.assertEqual(set(identity), {"name", "request_sha256", "key_sha256"})


if __name__ == "__main__":
    unittest.main()
