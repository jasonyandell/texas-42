#!/usr/bin/env python3
"""Independent exact statistics checker for the frozen v5 confirmation rule.

The input is two panel ``summary.json`` files.  Only their ``paired_results``
are used for inference; no outcome, timing, or subgroup is selected here.
All finite-count quantities remain Fractions until a named approximation is
requested.  This is intentionally independent of the arena aggregator.
"""

import argparse
from collections import defaultdict
from fractions import Fraction
import itertools
import json
import math
from pathlib import Path
import tempfile
import unittest


DECLARATIONS = (0, 1, 2, 3, 4, 5, 6, 7, 9)
DECLARATION_INDEX = {decl: index for index, decl in enumerate(DECLARATIONS)}
BIDDERS = range(4)
CELLS = tuple((decl, bidder) for decl in DECLARATIONS for bidder in BIDDERS)
PAIRS_PER_CELL = 16
PANEL_PAIRS = len(CELLS) * PAIRS_PER_CELL
Z99 = 2.3263478740408408
Z975 = 1.959963984540054


def rational(value):
    value = Fraction(value)
    return {"numerator": value.numerator, "denominator": value.denominator,
            "decimal": float(value)}


def fraction_from(value):
    return Fraction(value["numerator"], value["denominator"])


def panel_rows(path):
    data = json.loads(Path(path).read_text())
    rows = data.get("paired_results")
    if not isinstance(rows, list) or len(rows) != PANEL_PAIRS:
        raise ValueError(f"{path}: paired_results must contain exactly {PANEL_PAIRS} rows")
    if "pairs" in data and data["pairs"] != PANEL_PAIRS:
        raise ValueError(f"{path}: summary pairs disagrees with paired_results")
    cells = defaultdict(list)
    seen = set()
    for row in rows:
        if not isinstance(row, dict):
            raise ValueError(f"{path}: paired result is not an object")
        try:
            decl, bidder, index, delta = (row["decl"], row["bidder"],
                                          row["index"], row["delta"])
        except KeyError as exc:
            raise ValueError(f"{path}: missing paired result field {exc.args[0]}") from exc
        if type(decl) is not int or decl not in DECLARATIONS:
            raise ValueError(f"{path}: invalid declaration cell")
        if type(bidder) is not int or bidder not in BIDDERS:
            raise ValueError(f"{path}: invalid bidder cell")
        if type(index) is not int or not 0 <= index < PANEL_PAIRS:
            raise ValueError(f"{path}: invalid global pair index")
        if type(delta) is not int or delta not in (-1, 0, 1):
            raise ValueError(f"{path}: delta must be one of -1, 0, +1")
        cell_slot = DECLARATION_INDEX[decl] * len(BIDDERS) + bidder
        cell_base = cell_slot * PAIRS_PER_CELL
        repeat = index - cell_base
        if not 0 <= repeat < PAIRS_PER_CELL:
            raise ValueError(f"{path}: global index {index} is in the wrong cell")
        if index in seen:
            raise ValueError(f"{path}: duplicate global pair index {index}")
        seen.add(index)
        cells[(decl, bidder)].append((repeat, delta))
    if set(cells) != set(CELLS):
        missing = sorted(set(CELLS) - set(cells))
        extra = sorted(set(cells) - set(CELLS))
        raise ValueError(f"{path}: cell coverage mismatch missing={missing} extra={extra}")
    result = {}
    for cell in CELLS:
        entries = sorted(cells[cell])
        if [index for index, _ in entries] != list(range(PAIRS_PER_CELL)):
            raise ValueError(f"{path}: cell {cell} does not contain indices 0..15 exactly")
        result[cell] = [delta for _, delta in entries]
    return result


def unbiased_variance(values, mean=None):
    values = [Fraction(value) for value in values]
    if len(values) < 2:
        raise ValueError("unbiased variance requires at least two observations")
    mean = sum(values, Fraction()) / len(values) if mean is None else Fraction(mean)
    return sum((value - mean) ** 2 for value in values) / (len(values) - 1)


def sign_probability(values):
    wins = sum(value > 0 for value in values)
    losses = sum(value < 0 for value in values)
    discordant = wins + losses
    if discordant == 0:
        return {"wins": 0, "losses": 0, "discordant": 0, "probability": rational(Fraction(1))}
    numerator = sum(math.comb(discordant, k) for k in range(wins, discordant + 1))
    probability = Fraction(numerator, 2 ** discordant)
    return {"wins": wins, "losses": losses, "discordant": discordant,
            "probability": rational(probability)}


def summarize_cells(cells):
    reports = {}
    for (decl, bidder) in CELLS:
        values = [Fraction(value) for value in cells[(decl, bidder)]]
        mean = sum(values, Fraction()) / PAIRS_PER_CELL
        variance = unbiased_variance(values, mean)
        reports[f"decl-{decl}:bidder-{bidder}"] = {
            "decl": decl,
            "bidder": bidder,
            "n": PAIRS_PER_CELL,
            "mean": rational(mean),
            "variance": rational(variance),
            "wins": sum(value > 0 for value in values),
            "losses": sum(value < 0 for value in values),
            "ties": sum(value == 0 for value in values),
        }
    return reports


def summarize_panel(cells):
    reports = summarize_cells(cells)
    values = list(itertools.chain.from_iterable(cells[cell] for cell in CELLS))
    mean = sum(map(Fraction, values), Fraction()) / PANEL_PAIRS
    signs = sign_probability(values)
    return {
        "pairs": PANEL_PAIRS,
        "mean": rational(mean),
        "positive": mean > 0,
        "wins": sum(value > 0 for value in values),
        "losses": sum(value < 0 for value in values),
        "ties": sum(value == 0 for value in values),
        "sign": signs,
        "cells": reports,
    }


def summarize_two_panels(paths):
    if len(paths) != 2:
        raise ValueError("exactly two panel summaries are required")
    return summarize_cell_panels([panel_rows(path) for path in paths])


def summarize_cell_panels(panels):
    if len(panels) != 2:
        raise ValueError("exactly two panel cell maps are required")
    all_cells = {}
    for panel_index, cells in enumerate(panels):
        for cell in CELLS:
            all_cells[(panel_index, *cell)] = cells[cell]

    strata = []
    pooled_values = []
    for panel_index in range(2):
        for decl, bidder in CELLS:
            values = all_cells[(panel_index, decl, bidder)]
            mean = sum(map(Fraction, values), Fraction()) / PAIRS_PER_CELL
            variance = unbiased_variance(values, mean)
            strata.append((mean, variance))
            pooled_values.extend(values)

    stratum_count = len(strata)
    delta = sum((mean for mean, _ in strata), Fraction()) / stratum_count
    se2 = sum((variance / PAIRS_PER_CELL for _, variance in strata), Fraction()) / (stratum_count ** 2)
    se = math.sqrt(float(se2))
    if se2 == 0:
        normal = {
            "status": "unresolved_zero_variance",
            "se2": rational(se2),
            "se": 0.0,
            "lower99": None,
            "p_one_sided": None,
            "interval95": None,
        }
    else:
        z = float(delta) / se
        normal = {
            "status": "approximate",
            "se2": rational(se2),
            "se": se,
            "z": z,
            "lower99": float(delta) - Z99 * se,
            "p_one_sided": 0.5 * math.erfc(z / math.sqrt(2.0)),
            "interval95": [float(delta) - Z975 * se, float(delta) + Z975 * se],
        }
    n = stratum_count * PAIRS_PER_CELL
    hoeffding_half_width = math.sqrt(2.0 * math.log(2.0 / 0.05) / n)
    hoeffding = {
        "half_width": hoeffding_half_width,
        "interval95": [max(-1.0, float(delta) - hoeffding_half_width),
                        min(1.0, float(delta) + hoeffding_half_width)],
        "scope": "descriptive fixed-N bound for D in [-1,1]",
    }
    panel_reports = [summarize_panel(panel) for panel in panels]
    return {
        "panels": panel_reports,
        "pooled": {
            "strata": stratum_count,
            "pairs": n,
            "delta": rational(delta),
            "positive_each_panel": all(panel["positive"] for panel in panel_reports),
            "normal_mean_effect": normal,
            "sign": sign_probability(pooled_values),
            "hoeffding": hoeffding,
        },
    }


def _synthetic_panel(values):
    return {cell: list(values) for cell in CELLS}


class StatisticsChecks(unittest.TestCase):
    @staticmethod
    def fixture_rows(delta=0):
        rows = []
        for declaration_index, decl in enumerate(DECLARATIONS):
            for bidder in BIDDERS:
                cell_slot = declaration_index * len(BIDDERS) + bidder
                for repeat in range(PAIRS_PER_CELL):
                    rows.append({
                        "decl": decl,
                        "bidder": bidder,
                        "index": cell_slot * PAIRS_PER_CELL + repeat,
                        "delta": delta,
                    })
        return rows

    def write_fixture(self, rows):
        temporary = tempfile.TemporaryDirectory()
        path = Path(temporary.name) / "summary.json"
        path.write_text(json.dumps({"pairs": PANEL_PAIRS, "paired_results": rows}))
        return temporary, path

    def test_actual_global_index_fixture_is_accepted(self):
        temporary, path = self.write_fixture(self.fixture_rows())
        try:
            cells = panel_rows(path)
            self.assertEqual(len(cells), 36)
            self.assertEqual(len(cells[(9, 3)]), 16)
        finally:
            temporary.cleanup()

    def test_fixture_missing_row_is_rejected(self):
        rows = self.fixture_rows()[:-1]
        temporary, path = self.write_fixture(rows)
        try:
            with self.assertRaisesRegex(ValueError, "exactly 576"):
                panel_rows(path)
        finally:
            temporary.cleanup()

    def test_fixture_duplicate_global_index_is_rejected(self):
        rows = self.fixture_rows()
        rows[-1]["index"] = rows[-2]["index"]
        temporary, path = self.write_fixture(rows)
        try:
            with self.assertRaisesRegex(ValueError, "duplicate global"):
                panel_rows(path)
        finally:
            temporary.cleanup()

    def test_fixture_wrong_declaration_is_rejected(self):
        rows = self.fixture_rows()
        rows[0]["decl"] = 8
        temporary, path = self.write_fixture(rows)
        try:
            with self.assertRaisesRegex(ValueError, "invalid declaration"):
                panel_rows(path)
        finally:
            temporary.cleanup()

    def test_fixture_wrong_global_index_cell_is_rejected(self):
        rows = self.fixture_rows()
        rows[0]["index"] = 16
        temporary, path = self.write_fixture(rows)
        try:
            with self.assertRaisesRegex(ValueError, "wrong cell"):
                panel_rows(path)
        finally:
            temporary.cleanup()

    def test_all_ties_are_unresolved(self):
        result = summarize_cell_panels([_synthetic_panel([0] * 16), _synthetic_panel([0] * 16)])
        self.assertFalse(result["pooled"]["positive_each_panel"])
        self.assertEqual(result["pooled"]["sign"]["probability"]["numerator"], 1)
        self.assertEqual(result["pooled"]["sign"]["probability"]["denominator"], 1)
        self.assertEqual(result["pooled"]["normal_mean_effect"]["status"], "unresolved_zero_variance")

    def test_balanced_wins_losses_have_zero_mean(self):
        values = [1] * 8 + [-1] * 8
        result = summarize_cell_panels([_synthetic_panel(values), _synthetic_panel(values)])
        self.assertEqual(result["pooled"]["delta"]["numerator"], 0)
        self.assertFalse(result["pooled"]["positive_each_panel"])
        probability = fraction_from(result["pooled"]["sign"]["probability"])
        self.assertGreater(probability, Fraction(1, 2))
        self.assertLess(probability, Fraction(1))

    def test_uniform_positive_panel_has_positive_lower_bound(self):
        values = [1] * 15 + [0]
        result = summarize_cell_panels([_synthetic_panel(values), _synthetic_panel(values)])
        self.assertTrue(result["pooled"]["positive_each_panel"])
        self.assertEqual(result["pooled"]["delta"]["numerator"], 15)
        self.assertEqual(result["pooled"]["delta"]["denominator"], 16)
        self.assertLess(result["pooled"]["normal_mean_effect"]["lower99"], 1.0)
        self.assertGreater(result["pooled"]["normal_mean_effect"]["lower99"], 0.0)


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("panel", nargs="*", type=Path,
                        help="the two panel summary.json files")
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args(argv)
    if args.self_test:
        suite = unittest.defaultTestLoader.loadTestsFromTestCase(StatisticsChecks)
        result = unittest.TextTestRunner(verbosity=2).run(suite)
        raise SystemExit(0 if result.wasSuccessful() else 1)
    if len(args.panel) != 2:
        parser.error("provide exactly two panel summary.json paths")
    print(json.dumps(summarize_two_panels(args.panel), indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
