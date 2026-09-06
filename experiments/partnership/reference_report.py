#!/usr/bin/env python3
"""Describe a native-L1 / phone calibration from frozen completed games."""
import argparse
from collections import Counter, defaultdict
import json
from pathlib import Path
import statistics

import campaign as c


def report(path):
    path = Path(path).resolve()
    spec = c.load(path)
    assert spec.get("candidate_mode") == "baseline"
    assert spec.get("inner_belief") == "voidless"
    rows = c.complete_results(path, spec)
    assert rows
    times = defaultdict(list)
    routes = Counter()
    paired = {role: Counter() for role in ("declaring", "defending")}
    agreement = Counter()
    seed_deltas = []
    makes = Counter()
    for row in rows:
        seed_deltas.append(row["paired"]["seed_delta"] / 2)
        for role in paired:
            paired[role][row["paired"][role + "_delta"]] += 1
        reference = {}
        for arm in c.ARMS:
            result = row["arms"][arm]
            makes[arm] += result["made"]
            decisions = c.read(path / "seeds" / str(row["seed"]) / arm / "checkpoint.json")["decisions"]
            history = []
            for decision in decisions:
                response = decision["response"]
                mode = response["mode"]
                key = (decision["seat"], tuple(history))
                times[mode].append(response["elapsed_us"] / 1e6)
                routes[(mode, response["route"])] += 1
                if len(response["legal"]) > 1:
                    times[mode + "_nonforced"].append(response["elapsed_us"] / 1e6)
                if arm == "phone":
                    reference[key] = response
                elif mode == "baseline" and key in reference:
                    old = reference[key]
                    if len(response["legal"]) > 1:
                        # Prefix-conditioned diagnostic; not an independent
                        # sample of all later game states or strength trials.
                        agreement["nonforced_total"] += 1
                        agreement["same_choice"] += response["choice"] == old["choice"]
                        agreement["with_fallback"] += "fallback" in response["route"] or "fallback" in old["route"]
                history.extend((decision["seat"], response["choice"]))
    n = len(rows)
    cap = c.read(path / "cap/run.json", {})
    mean = statistics.mean(seed_deltas)
    # One observation per deal; declaring/defending placements share that
    # deal and must not be treated as independent for this rough SE.
    se = statistics.stdev(seed_deltas) / n**0.5 if n > 1 else None
    summary = {
        "campaign": spec["id"], "seeds": n, "games": 3*n,
        "batch_wall_seconds": cap.get("elapsed_seconds"),
        "make_counts": dict(makes),
        "roles": {role: {"wins": counts[1], "losses": counts[-1], "ties": counts[0]}
                  for role, counts in paired.items()},
        "paired": c.summarize(path, spec)["fresh_paired"],
        "net_percentage_points": mean*100,
        "rough_two_se_band_pp": [(mean-2*se)*100, (mean+2*se)*100] if se is not None else None,
        "mean_decision_seconds": {k: statistics.mean(v) for k, v in times.items()},
        "decision_counts": {k: len(v) for k, v in times.items()},
        "routes": {mode: {route: count for (m, route), count in routes.items() if m == mode}
                   for mode in ("phone", "baseline")},
        "common_history_action_agreement": dict(agreement),
        "fully_completed_target": n == spec["count"],
    }
    c.atomic(path / "CALIBRATION.json", summary)
    text = "# Native L1 versus the preserved phone reference\n\n"
    text += f"Exploratory fixed-budget calibration: **{n} paired deals / {3*n} games**, bid 30 throughout.\n\n"
    if cap.get("elapsed_seconds") is not None:
        text += f"The externally capped batch completed in **{cap['elapsed_seconds']:.2f} seconds**, below the 295-second watchdog allowance.\n\n"
    text += "Native L1 uses `baseline`, 40/8, voidless inner beliefs, fixed samples and no tie refinement. Phone uses the archived WASM at 40/8 with its original racing/refinement.\n\n"
    text += "| Native role | Favorable flips | Unfavorable flips | Ties |\n|---|---:|---:|---:|\n"
    for role, counts in summary["roles"].items():
        text += f"| {role} | {counts['wins']} | {counts['losses']} | {counts['ties']} |\n"
    p = summary["paired"]
    text += f"\nCombined: **{p['wins']} favorable / {p['losses']} unfavorable / {p['ties']} ties**; native net **{mean*100:+.1f} percentage points** per matched contract opportunity.\n\n"
    text += f"Phone declaring: {makes['phone']}/{n} makes. Native declaring: {makes['declaring']}/{n}. Phone facing native defense: {makes['defending']}/{n}.\n\n"
    if se is not None:
        low, high = summary["rough_two_se_band_pp"]
        text += f"A descriptive mean ±2 standard-error band, with one paired observation per deal, is **{low:+.1f} to {high:+.1f} percentage points**. This is a rough normal approximation, not an anytime-valid interval or a formal equivalence test. "
        if not summary["fully_completed_target"]:
            text += "The target did not fully complete; the run's stopping/censoring further limits that band. "
        text += "The predeclared practical equivalence band was ±5 points; absence of a detected difference would not establish equivalence.\n\n"
    text += "| Measured decision time | Phone | Native L1 |\n|---|---:|---:|\n"
    for label, suffix in [("All moves", ""), ("Nonforced decisions", "_nonforced")]:
        text += f"| {label} | {statistics.mean(times['phone'+suffix]):.3f}s | {statistics.mean(times['baseline'+suffix]):.3f}s |\n"
    for mode in ("phone", "baseline"):
        fallbacks = sum(count for (m, route), count in routes.items() if m == mode and "fallback" in route)
        text += f"\n{mode}: {fallbacks} fallbacks / {len(times[mode + '_nonforced'])} nonforced decisions.\n"
    total = agreement["nonforced_total"]
    text += f"\nAt identical public histories encountered by both implementations, {agreement['same_choice']}/{total} nonforced choices agreed; {agreement['with_fallback']} comparisons included a fallback. This prefix-conditioned action diagnostic establishes neither population action agreement nor playing-strength equivalence.\n\n"
    text += "All primary rows use the complete contiguous seed prefix. Per-move records, source/binary identities, independent replay verification, fallbacks, and the external cap are retained here.\n"
    c.atomic(path / "CALIBRATION.md", text)
    print(json.dumps(summary, indent=2))


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("path", type=Path)
    report(parser.parse_args().path)
