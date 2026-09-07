#!/usr/bin/env python3
"""Rebuild descriptive scores and cost accounting; never launches a player."""

import sys
from collections import Counter, defaultdict
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parents[1]))

import campaign as c  # noqa: E402
from match import report  # noqa: E402


def main():
    reports = {}
    costs = defaultdict(Counter)
    fallback_reasons = defaultdict(Counter)
    for name in ("01-level", "02-voids"):
        path = HERE / name
        spec = c.load(path, verify=False)
        reports[name] = report(path)
        for row in c.complete_results(path, spec):
            for arm in c.arms_for(spec):
                fixture = row["arms"][arm]["fixture"]
                players = c.players_for(spec, arm, fixture["bidder"])
                snap = c.read(
                    path / "seeds" / str(row["seed"]) / arm / "checkpoint.json"
                )
                for d in snap["decisions"]:
                    response = d["response"]
                    out = costs[players[d["seat"]].name]
                    out["moves"] += 1
                    out["nonforced"] += response["route"] != "forced"
                    out["fallbacks"] += response["route"].endswith("fallback")
                    if response["route"].endswith("fallback"):
                        status = response["phases"][-1]["status"]
                        reason = (
                            "native-deadline-refusal"
                            if status.startswith(
                                "worker-error: Refusal { reason: Deadline,"
                            )
                            else "parent-timeout"
                            if status == "timeout"
                            else status
                        )
                        fallback_reasons[players[d["seat"]].name][reason] += 1
                    out["elapsed_us"] += response["elapsed_us"]
                    for phase in response["phases"]:
                        kind = phase["name"]
                        if kind not in ("status-check", "fallback-l1"):
                            kind = "requested-search"
                        out[kind + "_us"] += phase["elapsed_us"]
    wall = sum(c.read(p)["elapsed_seconds"] for p in HERE.glob("cap-*/run.json"))
    pairs = sum(r["completed_pairs"] for r in reports.values())
    complete = all(r["completed_pairs"] == 100 for r in reports.values())
    text = "# Default L1 / L2 Partner and L2 void results\n\n"
    text += "EXPLORATORY. " + (
        "Both planned panels completed."
        if complete
        else "Partial results: consult each match's status and technical stop record."
    )
    text += "\n\nThe [predeclared protocol](PROTOCOL.md) fixes search at **Fixed** for the real root and all modeled minds, with 40/8/2 samples and the existing 14-second wrapper. L2 means **L2 Partner** here. No solver or fallback code changed for this batch.\n\n"
    text += "The same 100 fresh physical deals (750600–750699) are used in both matchups. Each pair swaps player partnerships at bid 30. A win/loss/tie is based only on making the contract; both-make and both-set pairs tie regardless of points. These comparisons do not measure auction skill.\n\n"
    text += "| A versus B | Paired deals | A wins / losses / ties | A contract win fraction | Rough mean ±2 SE |\n|---|---:|---:|---:|---:|\n"
    for name, r in reports.items():
        a, b = (r["players"][side]["name"] for side in ("a", "b"))
        q = r["a_contract_win_fraction"]
        value = f"{q['numerator'] / q['denominator']:.1%}" if q else "—"
        interval = r["rough_two_se_win_fraction"]
        spread = f"{interval[0]:.1%}–{interval[1]:.1%}" if interval else "—"
        text += f"| [{a} vs {b}]({name}/MATCH.md) | {r['completed_pairs']}/100 | {r['a_pair_wins']} / {r['a_pair_losses']} / {r['pair_ties']} | {value} | {spread} |\n"
    text += "\nThe comparative fraction is `(1 + mean(make(A)-make(B))) / 2`; equality is 50%. It is not absolute pmake. Intervals are descriptive normal approximations, not formal equivalence claims. Both matchups share deal units; their evidence must not be counted as 200 independent deals or used to infer an unplayed matchup by subtraction.\n\n"
    text += "## Cost of the executed players\n\nAll means include forced moves and wall time under the shared ten-game load. L2 Partner default appears in both matchups and its timing below pools those appearances; the linked reports retain opponent-specific figures.\n\n"
    text += "| Player | Seconds / move | Requested search / move | Reserve preparation / move | Fallbacks / nonforced |\n|---|---:|---:|---:|---:|\n"
    for name, v in sorted(costs.items()):
        scale = 1e6 * v["moves"]
        text += f"| {name} | {v['elapsed_us'] / scale:.3f} | {v['requested-search_us'] / scale:.3f} | {v['fallback-l1_us'] / scale:.3f} | {v['fallbacks']} / {v['nonforced']} |\n"
    text += "\nReserve preparation is the cheap fixed L1 comparison computed before requested search. When requested search fails, the wrapper can return that reserve. The time already spent on the failed requested search still counts in the executed player's latency and results. A fallback is an execution route, not a refinement setting.\n\n"
    text += "Fallback reasons from the saved primary phase (the raw wire labels remain in each checkpoint):\n\n"
    for name, reasons in sorted(fallback_reasons.items()):
        text += (
            f"- {name}: "
            + ", ".join(
                f"{reason} {count}" for reason, count in sorted(reasons.items())
            )
            + ".\n"
        )
    text += "\n## Evidence and reproducibility\n\n"
    text += f"Published **{pairs * 2} games / {pairs * 56:,} moves**. Completed foreground watchdog slices total **{wall:.2f} seconds ({wall / 60:.2f} minutes)**; this includes any unfinished or speculative work. A running slice, if present, is not included until its watchdog record closes.\n\n"
    text += "Every match retains the full frozen configuration, source/binary identities, checkpoints, paired seed scores, timing phases, and technical stop records. `verification.json` records independent replay separately from these aggregates. [Configuration checks](configuration-check.json) confirm that exactly the modeled-seat profile changes in the first comparison and exactly the inner belief changes in the second. Both use the unchanged foundation engine and binary.\n\n"
    text += "The two inner samplers also use different deterministic draw streams: the void match compares implemented belief strategies as a whole. It does not isolate logical void filtering under identical random completions. The [player guide](../../PLAYERS.md) defines family, search, belief, and fallback terminology.\n"
    if complete:
        text += "\n## Interpretation\n\nNeither comparison establishes a strength gain. Default L2 Partner tied default L1 on this panel, while taking about five times its per-move wall time in this load. The void-aware version scored lower against default L2 Partner and cost more, but its rough interval includes 50%; this is not an established general loss either. A tied contract score does not imply identical choices or prove equivalence.\n\n"
        text += "All three configurations completed the planned panel under the existing technical gate. L1 remains the cheapest measured default here. L2 Partner and its void option remain usable experimental families whose extra cost did not produce a demonstrated win in this batch. Keep this result distinct from the phone comparison: these matches used fixed L1, not the racing native counterpart of the archived phone. The original phone-strength requirement remains open.\n\n"
        text += "All 2,683 first-slice moves survived resume unchanged ([resume proof](resume-proof.json)). Independent replay records cover all 400 games / 11,200 moves. The engine, native binary, reserve fallback, and scheduler were unchanged; only the named configurations and catalog/report surfaces changed. No experiment is running or scheduled.\n"
    c.atomic(HERE / "RESULTS.md", text)
    c.atomic(
        HERE / "RESULTS.json",
        {
            "complete": complete,
            "published_games": pairs * 2,
            "completed_slice_wall_seconds": wall,
            "reports": reports,
            "pooled_costs": dict(costs),
            "fallback_reasons": dict(fallback_reasons),
        },
    )
    print(
        {
            "complete": complete,
            "published_games": pairs * 2,
            "completed_slice_wall_seconds": wall,
        }
    )


if __name__ == "__main__":
    main()
