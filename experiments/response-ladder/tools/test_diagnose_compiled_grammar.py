#!/usr/bin/env python3
import importlib.util
from pathlib import Path
import unittest


ROOT = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("diagnose_compiled_grammar", ROOT / "diagnose_compiled_grammar.py")
diagnostic = importlib.util.module_from_spec(spec)
assert spec.loader is not None
spec.loader.exec_module(diagnostic)


def row(identifier, role, group, split="train", actions=None, costs=None, plays=None, hand=None):
    actions = actions or [None] * 14
    costs = costs or {3: 0, 4: 1}
    viewer = 1 if role == "declaring" else 0
    public = {"history": list(plays or []), "plays": list(plays or [])}
    return {
        "id": identifier,
        "group": group,
        "split": split,
        "mass": max(1, max(costs.values())),
        "costs": costs,
        "actions": tuple(actions),
        "fallback": min(costs),
        "request": {"decl": 9, "bidder": 1, "seat": viewer, "hand": list(hand or [3, 4])},
        "normalized_state": {"viewer": viewer, "hand": sum(1 << t for t in (hand or [3, 4])), "public": public},
        "role": role,
    }


class CompiledGrammarDiagnosticTests(unittest.TestCase):
    def test_role_partition_uses_normalized_viewer_and_rotation(self):
        plain = row("a", "declaring", "g")
        self.assertEqual(diagnostic.role_of(plain), "declaring")
        no_state = dict(plain)
        no_state["normalized_state"] = None
        no_state["request"] = {"bidder": 0, "seat": 2}
        self.assertEqual(diagnostic.role_of(no_state), "declaring")  # (2 + even-bidder rotation) % 4 = 3.

    def test_role_fit_is_independent_and_group_normalized(self):
        actions = [None] * 14
        actions[0] = 3
        actions[1] = 4
        rows = [
            row("d1", "declaring", "g1", actions=actions, costs={2: 2, 3: 0, 4: 4}),
            row("d2", "declaring", "g2", actions=actions, costs={2: 2, 3: 0, 4: 4}),
            row("f1", "defending", "g3", actions=actions, costs={2: 2, 3: 4, 4: 0}),
            row("f2", "defending", "g4", actions=actions, costs={2: 2, 3: 4, 4: 0}),
        ]
        selected = diagnostic.role_fit(rows, rows)
        self.assertEqual(selected["declaring"]["clauses"], [0])
        self.assertEqual(selected["defending"]["clauses"], [1])
        self.assertEqual(selected["declaring"]["train"]["groups"], 2)

    def test_floor_and_teacher_optimal_coverage_distinguish_fallback(self):
        actions = [None] * 14
        actions[0] = 3
        rows = [
            row("with", "declaring", "g1", actions=actions, costs={2: 2, 3: 0, 4: 1}),
            row("floor", "declaring", "g2", actions=[None] * 14, costs={2: 0, 3: 1, 4: 1}),
        ]
        report = diagnostic.floor_report(rows)["declaring"]
        self.assertEqual(report["fallback_only"]["numerator"], 1)
        self.assertEqual(report["teacher_optimal_with_clause"]["numerator"], 1)
        self.assertEqual(report["teacher_optimal_with_fallback"], {"numerator": 1, "denominator": 2, "decimal": 0.5})

    def test_binary_coverage_restores_row_mass(self):
        a = row("a", "declaring", "g1")
        b = row("b", "declaring", "g1")
        b["mass"] = 17
        self.assertEqual(diagnostic.weighted_fraction([a, b], lambda _: True),
                         {"numerator": 1, "denominator": 1, "decimal": 1.0})
        self.assertEqual(diagnostic.weighted_fraction([a, b], lambda r: r["id"] == "a"),
                         {"numerator": 1, "denominator": 2, "decimal": 0.5})

    def test_new_selector_closes_an_unavailable_optimal_action(self):
        actions = [3] * 14
        r = row("new", "declaring", "g1", plays=[0], hand=[3, 6],
                actions=actions, costs={3: 1, 6: 0})
        # No-trump 0-0 is the led double and cannot be beaten: no false coverage.
        report = diagnostic.beat_report([r])["declaring"]
        self.assertEqual(report["newly_covered_rows_beyond_old_floor"], 0)
        r = row("new", "declaring", "g1", plays=[3, 8], hand=[4, 12],
                actions=[4] * 14, costs={4: 2, 12: 0})
        report = diagnostic.beat_report([r])["declaring"]
        self.assertEqual(report["old_unavailable_optimal_rows"], 1)
        self.assertEqual(report["newly_covered_rows_beyond_old_floor"], 1)
        self.assertEqual(report["extended_available_action_loss_floor"]["numerator"], 0)

    def test_beat_selector_requires_mid_trick_and_matches_referee_legality(self):
        # Decl 9, lead 1-0: tile 4 (2-1) follows and beats the lead.
        # Tile 5 (2-2) is off suit and therefore illegal while 4 is held.
        plays = [1]
        r = row("beat", "declaring", "g1", plays=plays, hand=[4, 5])
        self.assertEqual(diagnostic.beat_tile(r), 4)
        lead = row("lead", "declaring", "g2", plays=[])
        self.assertIsNone(diagnostic.beat_tile(lead))
        malformed = row("bad", "declaring", "g3", plays=plays, hand=[4, 5])
        malformed["normalized_state"]["public"]["history"] = [2, 1, 2]
        self.assertIsNone(diagnostic.beat_tile(malformed))


if __name__ == "__main__":
    unittest.main()
