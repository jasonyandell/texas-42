"""Mathematical contract tests for the finite compiled-actor fitter."""

from contextlib import redirect_stdout
from fractions import Fraction
import importlib.util
import io
import json
from itertools import permutations
from pathlib import Path
import sys
import tempfile
import unittest
from unittest import mock


HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("fit_compiled", HERE / "fit_compiled.py")
assert SPEC is not None and SPEC.loader is not None
FIT = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(FIT)


def row(row_id, group, mass, actions, costs, fallback=0):
    return {
        "id": row_id,
        "group": group,
        "mass": mass,
        "actions": tuple(actions),
        "costs": dict(costs),
        "fallback": fallback,
        "request": {},
        "teacher_contract": FIT.teacher_contract({
            "target": "compatible-root-reset-v1",
            "revision": "t0-dice-exact-v1",
            "field_revision": "historical-dice-v1",
        }),
    }


class FitterMathTests(unittest.TestCase):
    def test_historical_contract_is_explicit_and_cannot_mix_with_compatible_labels(self):
        def lesson(identifier, target, revision):
            contract = FIT.TEACHER_CONTRACTS[(target, revision)]
            metadata = dict(zip(('mode', 'prior', 'seed_rule'), contract))
            request = {'seat': 1, 'bidder': 1, 'n': 8}
            response = {
                'schema': 'compiled-teacher-v2', 'feature_schema': 'scheme-relational-actor-v2',
                'status': 'complete', 'target': target, 'revision': revision,
                'field_revision': 'historical-dice-v1', **metadata, 'request': request,
                'mass': 8, 'denominator': 8, 'maxcount': 8, 'canonical_action': 2,
                'action_counts': [[1, 7], [2, 8]],
                'costs': [{'action': 1, 'numerator': 1, 'denominator': 8},
                          {'action': 2, 'numerator': 0, 'denominator': 8}],
                'clause_actions': [2] + [None] * 15,
                'bundle': {'target': target, 'revision': revision,
                           'field_revision': 'historical-dice-v1', **metadata,
                           'mass': 8, 'n': 8, 'seed': 17,
                           'scenarios': [{'hands': [127, 16256, 2080768, 266338304],
                                          'tape': i, 'weight': 1} for i in range(8)]},
            }
            return {'id': identifier, 'group': identifier, 'split': 'train',
                    'request': request, 'response': response}

        historical = lesson('h0', 'historical-voidless-h0-v1', 'h0-native-dice-exact-v1')
        compatible = lesson('t0', 'compatible-root-reset-v1', 't0-dice-exact-v1')
        diagnostic = lesson('diag', 'compatible-root-h0-seed-v1', 't0-h0-seed-dice-exact-v1')
        with tempfile.TemporaryDirectory() as temporary:
            campaign = Path(temporary)
            lessons = campaign / 'lessons'; lessons.mkdir()
            path = lessons / 'first.json'
            for accepted in (historical, compatible, diagnostic):
                path.write_text(json.dumps(accepted))
                rows, statuses, _ = FIT.read_rows(campaign)
                self.assertEqual(len(rows), 1)
                self.assertEqual(statuses['complete'], 1)
                self.assertEqual(rows[0]['teacher_contract']['target'], accepted['response']['target'])
            path.write_text(json.dumps(historical))
            (lessons / 'second.json').write_text(json.dumps(compatible))
            with self.assertRaisesRegex(ValueError, 'mixed teacher'):
                FIT.read_rows(campaign)
            # Censoring a differently targeted row must not hide the mixture.
            mixed_refusal = json.loads(json.dumps(compatible))
            mixed_refusal['response']['status'] = 'refused'
            (lessons / 'second.json').write_text(json.dumps(mixed_refusal))
            with self.assertRaisesRegex(ValueError, 'mixed teacher'):
                FIT.read_rows(campaign)
            (lessons / 'second.json').unlink()
            mutations = [
                ('target', 'historical-but-undeclared'),
                ('revision', 't0-dice-exact-v1'),
                ('prior', 'public-void-compatible-shuffle-reject'),
                ('seed_rule', 'request-seed-v1'),
                ('field_revision', 'unknown-dice'),
            ]
            for key, value in mutations:
                invalid = json.loads(json.dumps(historical))
                invalid['response'][key] = value
                path.write_text(json.dumps(invalid))
                with self.assertRaises(ValueError, msg=key):
                    FIT.read_rows(campaign)
            invalid = json.loads(json.dumps(historical))
            invalid['response']['bundle']['scenarios'].pop()
            path.write_text(json.dumps(invalid))
            with self.assertRaisesRegex(ValueError, 'original columns'):
                FIT.read_rows(campaign)
            invalid = json.loads(json.dumps(historical))
            invalid['response']['request']['n'] = 32
            path.write_text(json.dumps(invalid))
            with self.assertRaisesRegex(ValueError, 'fixed n=8'):
                FIT.read_rows(campaign)

    def test_extended_vocabulary_and_role_fit_ignore_development_preferences(self):
        rows = []
        for seat in (0, 1):
            for split in ('train', 'development'):
                # The two roles need opposite actions in training. Development
                # deliberately prefers the reverse, and cannot choose programs.
                costs = {1: seat, 2: 1 - seat}
                if split == 'development':
                    costs = {action: 1 - value for action, value in costs.items()}
                item = row(f'{seat}-{split}', split, 1, [None] * 14 + [2, None], costs, fallback=1)
                item.update(split=split, feature_schema='scheme-relational-actor-v2',
                            request={'seat': seat, 'bidder': 1})
                rows.append(item)
        with tempfile.TemporaryDirectory() as temporary:
            campaign = Path(temporary) / 'campaign'
            campaign.mkdir()
            for name in ('manifest.json', 'requests.jsonl', 'teacher-config.json'):
                (campaign / name).write_text('{}')
            output = Path(temporary) / 'result'
            argv = ['fit_compiled.py', str(campaign), '--output', str(output), '--roles']
            with mock.patch.object(FIT, 'read_rows', return_value=(rows, {}, {})), mock.patch.object(sys, 'argv', argv), redirect_stdout(io.StringIO()):
                FIT.main()
            actor = json.loads((output / 'actor.json').read_text())
            audit = json.loads((output / 'audit.json').read_text())
            self.assertEqual(actor['schema'], 'scheme-role-actors-v1')
            self.assertEqual(actor['declaring']['clauses'], [14])
            self.assertEqual(actor['defending']['clauses'], [])
            self.assertEqual(audit['roles']['declaring']['programs'], 3617)
            self.assertEqual(audit['role_mean_cost']['train']['decimal'], 0)
            self.assertEqual(audit['role_mean_cost']['development']['decimal'], 1)

    def test_group_weights_are_equal_after_per_row_mass_normalization(self):
        rows = [
            row("a0", "A", 2, [None] * 14, {0: 2}),
            row("a1", "A", 4, [None] * 14, {0: 4}),
            row("b0", "B", 3, [None] * 14, {0: 3}),
        ]

        weighted, denominator = FIT.weights(rows)

        # Raw denominator is lcm(2*2, 4*2, 3*1)=24.  The returned value
        # includes the two-group average denominator, 24*2=48.
        self.assertEqual(denominator, 48)
        self.assertEqual([weight for _, weight in weighted], [6, 3, 8])
        # Each group's normalized total is one unit: A contributes 24 and B
        # contributes 24 to the raw numerator.  The global mean is one.
        self.assertEqual(FIT.score(weighted, ()), 48)
        self.assertEqual(
            Fraction(FIT.score(weighted, ()), denominator),
            Fraction(1),
        )

    def test_signature_lower_bound_is_below_every_ordered_program(self):
        none = [None] * 14
        rows = [
            row("a0", "A", 2, [0] + none[1:], {0: 0, 1: 2}, fallback=1),
            row("a1", "A", 2, [0] + none[1:], {0: 2, 1: 0}, fallback=1),
            row("b0", "B", 1, [1] + none[1:], {0: 1, 1: 0}, fallback=0),
        ]

        audit = FIT.signature_audit(rows)
        lower = Fraction(
            audit["optimistic_empirical_loss_lower"]["numerator"],
            audit["optimistic_empirical_loss_lower"]["denominator"],
        )
        # Group A has two conflicting rows with the same signature.  Its
        # relaxed optimum is raw cost 2; group B can achieve zero.  The
        # returned two-group average is therefore 2/8 = 1/4.
        self.assertEqual(lower, Fraction(1, 4))

        weighted, denominator = FIT.weights(rows)
        for length in range(4):
            for clauses in permutations(range(14), length):
                objective = Fraction(FIT.score(weighted, clauses), denominator)
                self.assertGreaterEqual(objective, lower, clauses)

    def test_program_selection_uses_training_only(self):
        with tempfile.TemporaryDirectory() as temporary:
            campaign = Path(temporary) / "campaign"
            lessons = campaign / "lessons"
            lessons.mkdir(parents=True)

            (campaign / "manifest.json").write_text("{}", encoding="utf-8")
            (campaign / "requests.jsonl").write_text("{}\n", encoding="utf-8")
            (campaign / "teacher-config.json").write_text("{}", encoding="utf-8")

            def write_lesson(lesson_id, group, split, counts):
                best = max(count for _, count in counts)
                response = {
                    "schema": "compiled-teacher-v1",
                    "status": "complete",
                    "request": {"id": lesson_id},
                    "target": "compatible-root-reset-v1",
                    "revision": "t0-dice-exact-v1",
                    "field_revision": "historical-dice-v1",
                    "mass": 2,
                    "bundle": {"mass": 2},
                    "action_counts": counts,
                    "costs": [
                        {
                            "action": action,
                            "numerator": best - count,
                            "denominator": 2,
                        }
                        for action, count in counts
                    ],
                    "maxcount": best,
                    "denominator": 2,
                    "canonical_action": min(
                        action for action, count in counts if count == best
                    ),
                    "clause_actions": [2] + [None] * 13,
                }
                value = {
                    "id": lesson_id,
                    "group": group,
                    "split": split,
                    "request": {"id": lesson_id},
                    "response": response,
                }
                (lessons / f"{lesson_id}.json").write_text(
                    json.dumps(value), encoding="utf-8"
                )

            # Clause 0 is optimal on train, while fallback is optimal on dev.
            # Selection must still choose clause 0 and report dev only as a
            # diagnostic.
            write_lesson("train", "train-group", "train", [[1, 1], [2, 2]])
            write_lesson("development", "dev-group", "development", [[1, 2], [2, 1]])
            output = Path(temporary) / "audit"

            argv = ["fit_compiled.py", str(campaign), "--output", str(output)]
            with mock.patch.object(sys, "argv", argv):
                with redirect_stdout(io.StringIO()):
                    FIT.main()

            actor = json.loads((output / "actor.json").read_text(encoding="utf-8"))
            audit = json.loads((output / "audit.json").read_text(encoding="utf-8"))
            self.assertEqual(actor["clauses"], [0])
            self.assertEqual(audit["selected"]["train_local_cost"]["decimal"], 0.0)
            self.assertEqual(
                audit["selected"]["development_local_cost"]["decimal"], 0.5
            )

    def test_lead_follow_selection_keeps_parent_role_group_weights(self):
        def phase_row(identifier, group, history, costs):
            item = row(identifier, group, 2, [0] + [None] * 15, costs, fallback=1)
            item.update(split='train', feature_schema='scheme-relational-actor-v2',
                        request={'seat': 1, 'bidder': 1, 'history': history})
            return item

        # Group A has four rows but only one lead row; group B has two rows and
        # one lead row.  Parent-role weights therefore make the lead rows tie
        # (and canonical selection chooses fallback), while re-normalizing the
        # phase would choose clause 0.  The latter is the bug this contract
        # prevents.
        parent = [
            phase_row('a-lead', 'A', [], {0: 0, 1: 2}),
            phase_row('a-f1', 'A', [[0, 1]], {0: 0, 1: 0}),
            phase_row('a-f2', 'A', [[0, 2]], {0: 0, 1: 0}),
            phase_row('a-f3', 'A', [[0, 3]], {0: 0, 1: 0}),
            phase_row('b-lead', 'B', [], {0: 1, 1: 0}),
            phase_row('b-f1', 'B', [[0, 4]], {0: 0, 1: 0}),
        ]
        phase = [parent[0], parent[4]]
        selected, denominator = FIT.select_phase_program(parent, phase)
        self.assertEqual(selected[2], ())
        phase_weighted, phase_denominator = FIT.weights(phase)
        self.assertLess(FIT.score(phase_weighted, (0,)), FIT.score(phase_weighted, ()))
        self.assertNotEqual(denominator, phase_denominator)

    def test_lead_follow_development_cost_cannot_select_programs(self):
        rows = []
        for split, costs in (
                ('train', {0: 0, 1: 1}),
                ('development', {0: 1, 1: 0})):
            rows.extend([
                row(f'{split}-lead', f'{split}-lead-group', 1,
                    [0] + [None] * 15, costs, fallback=1),
                row(f'{split}-follow', f'{split}-follow-group', 1,
                    [0] + [None] * 15, costs, fallback=1),
            ])
        for item in rows:
            item.update(split=item['id'].split('-')[0], feature_schema='scheme-relational-actor-v2',
                        request={'seat': 1, 'bidder': 1,
                                 'history': [] if item['id'].endswith('lead') else [[0, 1]]})
        actor, audit = FIT.fit_lead_follow_role(rows)
        self.assertEqual(actor['schema'], 'scheme-relational-lead-follow-v1')
        self.assertEqual(actor['clauses'], [0])
        self.assertEqual(actor['lead_clauses'], [0])
        self.assertEqual(audit['selected']['train_local_cost']['decimal'], 0.0)
        self.assertEqual(audit['selected']['development_local_cost']['decimal'], 1.0)


if __name__ == "__main__":
    unittest.main()
