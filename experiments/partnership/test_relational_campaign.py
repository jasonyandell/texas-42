"""Durability and grouped-report checks for the relational campaign runner."""

import json
import os
import tempfile
import unittest
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import patch

import relational_campaign as campaign


def actor_row(value, *, digest, work):
    return {
        "value": value,
        "policy_regret": "0",
        "root_regret": "0",
        "first_fallback_probability": "0",
        "digest": digest,
        "inference_work": work,
    }


class RelationalCampaignTests(unittest.TestCase):
    def test_signal_during_spawn_is_reentrant_and_kills_newly_registered_child(self):
        with tempfile.TemporaryDirectory() as tmp:
            args=SimpleNamespace(output=Path(tmp),binary=Path(__file__),seconds=10,
                seed=1,train=1,dev=1,test=1,tiles=3,max_worlds=128,field='l0-8',
                samples=16,work=2000000,clauses=3,beam=6,timeout=10)
            c=campaign.Campaign(args)
            class Child:
                returncode=-9
                def communicate(self, timeout=None): return '', 'interrupted'
            child=Child()
            def spawn(*_args,**_kwargs):
                # Fail immediately instead of hanging if the lock regresses.
                self.assertTrue(c.lock.acquire(blocking=False))
                c.lock.release()
                c.kill()
                return child
            with patch.object(campaign.subprocess,'Popen',side_effect=spawn), patch.object(campaign,'kill_process_group') as kill:
                with self.assertRaises(InterruptedError): c.job('interrupted-fit','fit',{})
                kill.assert_called_once_with(child)
            self.assertFalse((Path(tmp)/'jobs/interrupted-fit/result.json').exists())

    def test_materialize_reuses_equal_immutable_file_without_touching_it(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "inputs" / "actor.policy"
            campaign.materialize(path, "fixed actor\n")
            # Give the file a deliberately recognizable timestamp so this test
            # detects an equal-content rewrite even on a coarse filesystem.
            os.utime(path, ns=(1_234_567_890_000_000_000,) * 2)
            before = path.stat()

            campaign.materialize(path, "fixed actor\n")

            after = path.stat()
            self.assertEqual(after.st_ino, before.st_ino)
            self.assertEqual(after.st_mtime_ns, before.st_mtime_ns)
            self.assertEqual(path.read_text(), "fixed actor\n")

    def test_materialize_refuses_identity_change_and_preserves_original(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "lessons.txt"
            campaign.materialize(path, "original\n")

            with self.assertRaisesRegex(ValueError, "immutable text input changed"):
                campaign.materialize(path, "replacement\n")

            self.assertEqual(path.read_text(), "original\n")

    def test_checked_payload_refuses_payload_and_artifact_corruption(self):
        with tempfile.TemporaryDirectory() as tmp:
            directory = Path(tmp) / "jobs" / "fit-a-exact"
            directory.mkdir(parents=True)
            artifact = directory / "attempt-1" / "candidate.policy"
            artifact.parent.mkdir()
            artifact.write_text("(policy p (initial shared) (fallback lowest-legal))\n")
            spec = {
                "operation": "fit",
                "options": {"lessons": "fixed.lessons"},
                "inputs": {"fixed.lessons": "input-digest"},
            }
            payload = {"schema": "relational-fit-v1", "candidates": ["p"]}
            record = {
                "spec": spec,
                "payload": payload,
                "payload_sha256": campaign.gym.digest(payload),
                "files": {str(artifact): campaign.sha(artifact)},
            }
            campaign.gym.atomic(directory / "result.json", record)
            self.assertEqual(campaign.checked_payload(directory, spec), payload)

            artifact.write_text("corrupt\n")
            with self.assertRaisesRegex(ValueError, "completed artifact changed"):
                campaign.checked_payload(directory, spec)

            artifact.write_text("(policy p (initial shared) (fallback lowest-legal))\n")
            record["payload"] = {"schema": "relational-fit-v1", "candidates": []}
            campaign.gym.atomic(directory / "result.json", record)
            with self.assertRaisesRegex(ValueError, "completed job payload changed"):
                campaign.checked_payload(directory, spec)

    def test_grouped_report_is_deterministic_across_completion_order(self):
        root_a = {
            "actors": {
                "table": actor_row("1/2", digest="table-a", work=0),
                "shared": actor_row("3/4", digest="shared", work=11),
            }
        }
        root_b = {
            "actors": {
                "shared": actor_row("1/4", digest="shared", work=13),
                "table": actor_row("1/2", digest="table-b", work=0),
            }
        }
        forward = {"test-0000": root_a, "test-0001": root_b}
        reverse = {"test-0001": root_b, "test-0000": root_a}

        first = campaign.report(forward, {})
        second = campaign.report(reverse, {})

        self.assertEqual(first, second)
        self.assertEqual(first["root_groups"], 2)
        self.assertEqual(first["arms"]["shared"]["value"], "1/2")
        self.assertEqual(first["arms"]["shared"]["inference_work"], 24)
        paired = first["paired"]["shared-minus-table"]
        self.assertEqual(paired["mean"], "0")
        self.assertEqual((paired["positive_roots"], paired["negative_roots"]), (1, 1))

    def test_completed_job_refuses_changed_resume_input_identity(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            directory = root / "jobs" / "fit-a-exact"
            directory.mkdir(parents=True)
            lessons = root / "fit-a-exact.lessons"
            lessons.write_text("fixed lesson\n")
            options = {"lessons": str(lessons), "work": "2000000"}
            original_spec = {
                "operation": "fit",
                "options": options,
                "inputs": {str(lessons): campaign.sha(lessons)},
            }
            payload = {"schema": "relational-fit-v1", "candidates": []}
            campaign.gym.atomic(
                directory / "result.json",
                {
                    "spec": original_spec,
                    "payload": payload,
                    "payload_sha256": campaign.gym.digest(payload),
                    "files": {},
                },
            )
            self.assertEqual(
                campaign.checked_payload(directory, original_spec), payload
            )

            lessons.write_text("changed lesson\n")
            resumed_spec = {
                "operation": "fit",
                "options": options,
                "inputs": {str(lessons): campaign.sha(lessons)},
            }
            with self.assertRaisesRegex(ValueError, "job input identity changed"):
                campaign.checked_payload(directory, resumed_spec)

    def test_frozen_actor_identity_is_immutable(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            actor = root / "selected.policy"
            actor.write_text("(policy p (initial shared) (fallback lowest-legal))\n")
            frozen = {
                "actors": {
                    "unpriced": {
                        "path": str(actor),
                        "sha256": campaign.sha(actor),
                    }
                },
                "manifest": "campaign-manifest-digest",
            }
            directory = root / "frozen"
            campaign.gym.pin(directory, frozen)
            before = json.loads((directory / "manifest.json").read_text())
            campaign.gym.pin(directory, frozen)
            self.assertEqual(json.loads((directory / "manifest.json").read_text()), before)

            changed = json.loads(json.dumps(frozen))
            changed["actors"]["unpriced"]["sha256"] = "different"
            with self.assertRaisesRegex(ValueError, "resume identity changed"):
                campaign.gym.pin(directory, changed)


if __name__ == "__main__":
    unittest.main()
