#!/usr/bin/env python3
"""Audit and summarize every paired full-game receipt in a fixed panel.

Examples:
  python3 tools/compare_panel.py results/reference.json results/candidate.json
  python3 tools/compare_panel.py --pair ref-a.json opt-a.json --pair ref-b.json opt-b.json

Each receipt may contain multiple repetitions. Every game is replayed by the
independent audit before any timing is reported. The nearest-rank p95 includes
the slowest observation for panels of 20 games or fewer.
"""

import argparse
import json
import math
import statistics
import sys
from pathlib import Path

from audit_receipt import audit_receipt, compare, require


def distribution(values):
    require(bool(values), "empty timing panel")
    ordered = sorted(values)
    return {"count": len(ordered), "median": statistics.median(ordered),
            "p95_nearest_rank": ordered[math.ceil(0.95 * len(ordered)) - 1],
            "range": [ordered[0], ordered[-1]]}


def usage_for(path):
    usage_path = path.with_name(path.stem + "-usage.json")
    if not usage_path.exists():
        return None
    usage = json.loads(usage_path.read_text())
    require(all(type(usage.get(k)) in (float, int) and usage[k] >= 0 for k in
                ("process_wall_s", "user_cpu_s", "system_cpu_s")),
            f"invalid process usage record: {usage_path}")
    return {"path": str(usage_path), "process_wall_s": usage["process_wall_s"],
            "user_cpu_s": usage["user_cpu_s"], "system_cpu_s": usage["system_cpu_s"],
            "total_cpu_s": usage["user_cpu_s"] + usage["system_cpu_s"]}


def run(pairs):
    rows = []
    batches = []
    player_settings = None
    for reference_path, candidate_path in pairs:
        reference_receipt, reference = audit_receipt(reference_path)
        candidate_receipt, candidate = audit_receipt(candidate_path)
        comparisons = compare(reference_receipt, candidate_receipt, reference, candidate)
        require(all(a["cache"] == b["cache"] for a, b in zip(reference, candidate)),
                f"cache mode differs: {reference_path} and {candidate_path}")
        batches.append({"reference": str(reference_path), "candidate": str(candidate_path),
                        "games": len(comparisons),
                        "reference_process_usage": usage_for(reference_path),
                        "candidate_process_usage": usage_for(candidate_path)})
        for i, (a, b, match) in enumerate(zip(reference, candidate, comparisons)):
            settings = {k: a[k] for k in ("profile", "cache", "samples", "selection",
                                          "modeled_selection", "inner_belief",
                                          "budget_ms_per_call")}
            if player_settings is None:
                player_settings = settings
            require(settings == player_settings,
                    f"game settings vary across panel: {reference_path}, repetition {i}")
            require(match["choices_equal"] and match["exact_values_equal"],
                    f"game {i}: exact comparison failed")
            rows.append({"reference": str(reference_path), "candidate": str(candidate_path),
                         "repetition": i, "fixture": a["fixture"],
                         "deal_seed": a["deal_seed"], "decl": a["decl"],
                         "bid": a["bid"], "bidder": a["bidder"],
                         "public_seed": a["public_seed"],
                         "exact_values_equal": True, "choices_equal": True,
                         "value_digest": a["value_digest"],
                         "reference_full_game_us": a["full_game_elapsed_us"],
                         "candidate_full_game_us": b["full_game_elapsed_us"],
                         "reference_call_wall_us": a["call_wall_us"],
                         "candidate_call_wall_us": b["call_wall_us"],
                         "reference_nodes": a["nodes"], "candidate_nodes": b["nodes"],
                         "speedup": match["speedup"]})
    return {"schema": "walt-full-game-panel-v1", "status": "verified",
            "scope": "direct native requested search; no outer L1 reserve or fallback",
            "player_settings": player_settings, "paired_games": len(rows),
            "all_choices_and_exact_values_equal": True, "batches": batches,
            "summary": {
                "reference_full_game_us": distribution([r["reference_full_game_us"] for r in rows]),
                "candidate_full_game_us": distribution([r["candidate_full_game_us"] for r in rows]),
                "paired_speedup": distribution([r["speedup"] for r in rows]),
            }, "games": rows}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("reference", nargs="?", type=Path)
    parser.add_argument("candidate", nargs="?", type=Path)
    parser.add_argument("--pair", action="append", nargs=2, type=Path, default=[],
                        metavar=("REFERENCE", "CANDIDATE"))
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    pairs = list(args.pair)
    if args.reference is not None or args.candidate is not None:
        if args.reference is None or args.candidate is None:
            parser.error("give both positional receipts or use --pair")
        if pairs:
            parser.error("positional receipts cannot be combined with --pair")
        pairs.insert(0, (args.reference, args.candidate))
    if not pairs:
        parser.error("give paired reference and candidate receipts")
    try:
        result = run(pairs)
    except (OSError, ValueError, KeyError, TypeError, IndexError, ZeroDivisionError) as exc:
        result = {"schema": "walt-full-game-panel-v1", "status": "failed", "error": str(exc),
                  "pairs": [[str(a), str(b)] for a, b in pairs]}
    output = json.dumps(result, indent=2) + "\n"
    if args.output:
        args.output.write_text(output)
    print(output, end="")
    return 0 if result["status"] == "verified" else 1


if __name__ == "__main__":
    sys.exit(main())
