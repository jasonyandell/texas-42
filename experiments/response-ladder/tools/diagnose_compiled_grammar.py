#!/usr/bin/env python3
"""Bounded role and selector diagnostics for the frozen T0 lesson corpus.

This is a calibration report over exact finite teacher costs.  It does not
produce labels, fit a full-game policy, or claim H2H regret.
"""

import argparse
from collections import Counter, defaultdict
from fractions import Fraction
from itertools import permutations
import hashlib
import importlib.util
import json
from pathlib import Path
import sys
import time


HERE = Path(__file__).resolve().parent
_FIT_SPEC = importlib.util.spec_from_file_location("fit_compiled", HERE / "fit_compiled.py")
fit = importlib.util.module_from_spec(_FIT_SPEC)
assert _FIT_SPEC.loader is not None
_FIT_SPEC.loader.exec_module(fit)

try:
    from reference_referee import TILES, called, context, follows, legal
except ImportError:  # Running from another working directory.
    _REF_SPEC = importlib.util.spec_from_file_location("reference_referee", HERE / "reference_referee.py")
    referee = importlib.util.module_from_spec(_REF_SPEC)
    assert _REF_SPEC.loader is not None
    _REF_SPEC.loader.exec_module(referee)
    TILES, called, context, follows, legal = referee.TILES, referee.called, referee.context, referee.follows, referee.legal


BASELINE = (5, 8, 2)
CLAUSE_COUNT = 14


def rational(n, d):
    return fit.rational(n, d)


def role_of(row):
    state = row.get("normalized_state") or {}
    viewer = state.get("viewer")
    if type(viewer) is not int:
        request = row["request"]
        rotation = int(request["bidder"] % 2 == 0)
        viewer = (request["seat"] + rotation) % 4
    if viewer not in range(4):
        raise ValueError(f"invalid normalized viewer for {row['id']}")
    return "declaring" if viewer % 2 else "defending"


def load_rows(campaign):
    rows, statuses, sources = fit.read_rows(campaign)
    by_id = {}
    for path in sorted((campaign / "lessons").glob("*.json")):
        item = json.loads(path.read_text())
        if item.get("response", item).get("status") == "complete":
            by_id[item["id"]] = item["response"]
    for row in rows:
        response = by_id[row["id"]]
        row["normalized_state"] = response.get("normalized_state")
        row["role"] = role_of(row)
    return rows, statuses, sources


def select(rows, deadline=None):
    weighted, denominator = fit.weights(rows)
    candidates = []
    for length in range(4):
        for clauses in permutations(range(CLAUSE_COUNT), length):
            if deadline is not None and time.monotonic() >= deadline:
                raise TimeoutError("leave-one-group-out fitting exceeded diagnostic budget")
            candidates.append((fit.score(weighted, clauses), length, clauses))
    return min(candidates), weighted, denominator


def metrics(rows, clauses):
    weighted, denominator = fit.weights(rows)
    score = fit.score(weighted, clauses)
    positive = [(row, weight) for row, weight in weighted if row["costs"][fit.choose(row, clauses)] > 0]
    return {
        "rows": len(rows),
        "groups": len({row["group"] for row in rows}),
        "local_cost": rational(score, denominator),
        "consequential_rows": len(positive),
        "consequential_weight": rational(sum(weight * row["mass"] for row, weight in positive), denominator),
    }


def aggregate_cost(reports):
    values = [Fraction(r["local_cost"]["numerator"], r["local_cost"]["denominator"]) for r in reports]
    value = sum(values, Fraction()) / len(values)
    return {"numerator": value.numerator, "denominator": value.denominator, "decimal": float(value)}


def role_fit(train, development, deadline=None):
    roles = {}
    for role in ("declaring", "defending"):
        local_train = [row for row in train if row["role"] == role]
        local_dev = [row for row in development if row["role"] == role]
        selected, _, _ = select(local_train, deadline)
        clauses = selected[2]
        roles[role] = {"clauses": list(clauses), "train": metrics(local_train, clauses),
                       "development": metrics(local_dev, clauses)}
    return roles


def weighted_fraction(rows, predicate):
    weighted, denominator = fit.weights(rows)
    # fit.weights equalizes physical groups and each row represents `mass`
    # equally likely worlds; restore that mass for coverage rates.
    numerator = sum(weight * row["mass"] for row, weight in weighted if predicate(row))
    return rational(numerator, denominator)


def floor_report(rows):
    def clause_output(row):
        return any(action is not None for action in row["actions"])

    def clause_optimal(row):
        return any(action is not None and row["costs"][action] == 0 for action in row["actions"])

    def floor_optimal(row):
        return row["costs"][fit.choose(row, ())] == 0

    groups = defaultdict(list)
    for row in rows:
        groups[row["role"]].append(row)
    result = {}
    for role, local in list(groups.items()) + [("pooled", rows)]:
        result[role] = {
            "rows": len(local),
            "groups": len({row["group"] for row in local}),
            "clause_output": weighted_fraction(local, clause_output),
            "fallback_only": weighted_fraction(local, lambda row: not clause_output(row)),
            "teacher_optimal_with_clause": weighted_fraction(local, clause_optimal),
            "teacher_optimal_with_fallback": weighted_fraction(local, floor_optimal),
        }
    return result


def _strength(tile, decl, led):
    hi, lo = TILES[tile]
    tier = 2 if called(tile, decl) else 1 if follows(tile, led, decl) else 0
    rank = hi if hi == lo and decl == 7 else 12 if hi == lo else hi + lo
    return tier, rank


def beat_tile(row):
    """Return the least legal tile that beats the current trick winner.

    The current-winner and led context are taken from the normalized public
    state, and legal/follow rules are the independent referee rules.  A lead
    has no current winner and therefore has no selector output.
    """
    state = row.get("normalized_state") or {}
    public = state.get("public") or {}
    plays = public.get("plays")
    if not isinstance(plays, list) or not plays:
        return None
    if any(type(tile) is not int or tile not in range(28) for tile in plays):
        return None
    decl = row["request"]["decl"]
    hand = state.get("hand")
    if type(hand) is not int:
        hand = sum(1 << tile for tile in row["request"]["hand"])
    hand_tiles = [tile for tile in range(28) if hand & (1 << tile)]
    if len(public.get("history", [])) < len(plays) or public["history"][-len(plays):] != plays:
        return None
    led = context(plays[0], decl)
    winner = max(plays, key=lambda tile: _strength(tile, decl, led))
    options = legal(hand_tiles, [(0, tile) for tile in plays], decl)
    winners = [tile for tile in options if _strength(tile, decl, led) > _strength(winner, decl, led)]
    return min(winners) if winners else None


def beat_report(rows):
    groups = defaultdict(list)
    for row in rows:
        groups[row["role"]].append(row)
    result = {}
    for role, local in list(groups.items()) + [("pooled", rows)]:
        weighted, denominator = fit.weights(local)
        eligible = [(row, weight) for row, weight in weighted if row.get("normalized_state", {}).get("public", {}).get("plays")]
        outputs = [(row, weight, beat_tile(row)) for row, weight in eligible]
        outputs = [(row, weight, tile) for row, weight, tile in outputs if tile is not None]
        optimal = [(row, weight) for row, weight, tile in outputs if row["costs"].get(tile) == 0]
        def old_optimal(row):
            available = [action for action in row["actions"] if action is not None]
            available.append(row["fallback"])
            return any(row["costs"][action] == 0 for action in available)

        newly_covered = [(row, weight) for row, weight in optimal if not old_optimal(row)]
        uncovered = [(row, weight) for row, weight in weighted if not old_optimal(row)]
        old_loss = new_loss = 0
        for row, weight in weighted:
            available = {action for action in row["actions"] if action is not None} | {row["fallback"]}
            old_loss += weight * min(row["costs"][action] for action in available)
            extra = beat_tile(row)
            if extra is not None:
                if extra not in row["costs"]:
                    raise ValueError("referee beat selector emitted an action absent from teacher legal actions")
                available.add(extra)
            new_loss += weight * min(row["costs"][action] for action in available)
        result[role] = {
            "rows": len(local),
            "groups": len({row["group"] for row in local}),
            "eligible_mid_trick": len(eligible),
            "selector_output_rows": len(outputs),
            "selector_output_weight": rational(sum(weight * row["mass"] for row, weight, _ in outputs), denominator),
            "teacher_optimal_output_rows": len(optimal),
            "teacher_optimal_output_weight": rational(sum(weight * row["mass"] for row, weight in optimal), denominator),
            "old_unavailable_optimal_rows": len(uncovered),
            "newly_covered_rows_beyond_old_floor": len(newly_covered),
            "newly_covered_groups": len({row["group"] for row, _ in newly_covered}),
            "old_available_action_loss_floor": rational(old_loss, denominator),
            "extended_available_action_loss_floor": rational(new_loss, denominator),
            "newly_covered_weight_beyond_old_floor": rational(sum(weight * row["mass"] for row, weight in newly_covered), denominator),
            "scope": "Exact referee replay of current normalized trick; no labels or full-game claim.",
        }
    return result


def loo_stability(train, seconds):
    started = time.monotonic()
    deadline = started + seconds
    result = {"status": "complete", "roles": {}}
    try:
        for role in ("declaring", "defending"):
            local = [row for row in train if row["role"] == role]
            groups = sorted({row["group"] for row in local})
            selected = []
            for group in groups:
                remaining = [row for row in local if row["group"] != group]
                best, _, _ = select(remaining, deadline)
                selected.append(list(best[2]))
            counts = Counter(map(tuple, selected))
            result["roles"][role] = {
                "groups": len(groups),
                "selected_clause_counts": {"[" + ",".join(map(str, key)) + "]": value for key, value in counts.items()},
            }
    except TimeoutError as exc:
        result = {"status": "skipped", "reason": str(exc), "elapsed_seconds": time.monotonic() - started}
    result["elapsed_seconds"] = time.monotonic() - started
    return result


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("campaign", type=Path)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--loo-seconds", type=float, default=12.0)
    args = parser.parse_args(argv)
    started = time.monotonic()
    rows, statuses, sources = load_rows(args.campaign)
    train = [row for row in rows if row["split"] == "train"]
    development = [row for row in rows if row["split"] == "development"]
    pooled = {"clauses": list(BASELINE), "train": metrics(train, BASELINE),
              "development": metrics(development, BASELINE),
              "train_by_role": {}, "development_by_role": {}}
    for split_name, split_rows in (("train", train), ("development", development)):
        for role in ("declaring", "defending"):
            pooled[f"{split_name}_by_role"][role] = metrics([r for r in split_rows if r["role"] == role], BASELINE)
        pooled[f"{split_name}_role_mean"] = aggregate_cost(list(pooled[f"{split_name}_by_role"].values()))
    role_specific = role_fit(train, development)
    result = {
        "schema": "compiled-grammar-diagnostic-v2",
        "scope": "Frozen complete T0 local costs; role partition is a bounded diagnostic, not a promoted policy or H2H result.",
        "source_statuses": dict(statuses),
        "source_lesson_hashes": sources,
        "rows": {"complete": len(rows), "train": len(train), "development": len(development)},
        "groups": {name: len({row["group"] for row in split}) for name, split in (("train", train), ("development", development))},
        "roles": {role: {"train_rows": sum(row["role"] == role for row in train),
                           "development_rows": sum(row["role"] == role for row in development),
                           "train_groups": len({row["group"] for row in train if row["role"] == role}),
                           "development_groups": len({row["group"] for row in development if row["role"] == role})}
                 for role in ("declaring", "defending")},
        "pooled_baseline": pooled,
        "role_specific": role_specific,
        "role_specific_role_mean": {
            "train": aggregate_cost([role_specific[role]["train"] for role in ("declaring", "defending")]),
            "development": aggregate_cost([role_specific[role]["development"] for role in ("declaring", "defending")]),
        },
        "availability_floor": {"train": floor_report(train), "development": floor_report(development)},
        "beat_current_winner": {"train": beat_report(train), "development": beat_report(development)},
        "leave_one_group_out": loo_stability(train, args.loo_seconds),
        "elapsed_seconds": time.monotonic() - started,
    }
    result["fitter_sha256"] = hashlib.sha256((HERE / "fit_compiled.py").read_bytes()).hexdigest()
    args.output.mkdir(parents=True, exist_ok=False)
    (args.output / "diagnostic.json").write_text(json.dumps(result, sort_keys=True, indent=2) + "\n")
    print(json.dumps({"rows": result["rows"], "pooled": pooled, "role_specific": role_specific,
                      "loo": result["leave_one_group_out"], "elapsed_seconds": result["elapsed_seconds"]}, indent=2))


if __name__ == "__main__":
    main()
