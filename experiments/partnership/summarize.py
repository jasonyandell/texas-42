#!/usr/bin/env python3
"""Summarize capped partnership runs without treating partial logs as results."""

from collections import Counter, defaultdict
import json
from pathlib import Path
import re


HERE = Path(__file__).resolve().parent
RUNS = HERE / "runs"
NATIVE_MODES = {"baseline", "partner", "all-l1"}


def seconds(microseconds):
    if not isinstance(microseconds, (int, float)):
        return None
    return round(microseconds / 1_000_000, 6)


def base_name(name):
    return re.sub(r"-\d+$", "", name)


def run_label(run_id):
    return re.sub(r"^\d+-", "", run_id)


def read_json(path):
    try:
        return json.loads(path.read_text())
    except (OSError, json.JSONDecodeError):
        return None


def read_events(path, run_id):
    events = []
    non_json = 0
    try:
        lines = path.open()
    except OSError:
        return events, non_json
    with lines:
        for line in lines:
            try:
                value = json.loads(line)
            except json.JSONDecodeError:
                non_json += bool(line.strip())
                continue
            if isinstance(value, dict) and isinstance(value.get("event"), str):
                events.append({**value, "_run": run_id})
    return events, non_json


def run_summary(run_dirs):
    items = []
    invalid_manifests = []
    unfinalized = []
    for directory in run_dirs:
        path = directory / "run.json"
        if not path.exists():
            unfinalized.append(directory.name)
            continue
        record = read_json(path)
        if not isinstance(record, dict):
            invalid_manifests.append(directory.name)
            continue
        elapsed = record.get("elapsed_seconds")
        ceiling = record.get("experiment_ceiling_seconds")
        items.append({
            "run": directory.name,
            "status": record.get("status", "unknown"),
            "seconds": round(elapsed, 6) if isinstance(elapsed, (int, float)) else None,
            "allowance_seconds": record.get("allowance_seconds"),
            "ceiling_seconds": ceiling,
            "ceiling_overrun": (
                isinstance(elapsed, (int, float))
                and isinstance(ceiling, (int, float))
                and elapsed >= ceiling
            ),
        })
    status = Counter(item["status"] for item in items)
    by_status_seconds = defaultdict(float)
    for item in items:
        if item["seconds"] is not None:
            by_status_seconds[item["status"]] += item["seconds"]
    numeric = [item["seconds"] for item in items if item["seconds"] is not None]
    overruns = [item["run"] for item in items if item["ceiling_overrun"]]
    return {
        "manifests": len(items),
        "status": dict(sorted(status.items())),
        "seconds": {
            "total": round(sum(numeric), 6),
            "max": max(numeric, default=None),
            "by_status": {key: round(value, 6) for key, value in sorted(by_status_seconds.items())},
        },
        "ceiling_overruns": {"count": len(overruns), "runs": overruns},
        "unfinalized_run_dirs": unfinalized,
        "invalid_manifests": invalid_manifests,
        "items": items,
    }


def root_summaries(events):
    grouped = defaultdict(list)
    for event in events:
        if event.get("event") == "root":
            grouped[(event["_run"], event.get("id"))].append(event)
    out = []
    for (run_id, root_id), rows in sorted(grouped.items()):
        request = rows[0].get("request", {})
        results = []
        for row in rows:
            response = row.get("response", {})
            evaluation = response.get("evaluation")
            if not isinstance(evaluation, dict):
                evaluation = {}
            phases = response.get("phases")
            if not isinstance(phases, list):
                phases = []
            results.append({
                "mode": response.get("mode"),
                "route": response.get("route"),
                "choice": response.get("choice"),
                "seconds": seconds(response.get("elapsed_us")),
                "over_budget": response.get("over_budget"),
                "primary_status": phases[-1].get("status") if phases and isinstance(phases[-1], dict) else None,
                "n": response.get("n"),
                "n0": response.get("n0"),
                "n1": response.get("n1"),
                "outer_worlds": evaluation.get("outer_worlds"),
                "outer_draw_attempts": evaluation.get("outer_draw_attempts"),
            })
        choices = [result["choice"] for result in results]
        out.append({
            "run": run_id,
            "root": root_id,
            "contract": {key: request.get(key) for key in ("decl", "bid", "bidder", "seat")},
            "legal": rows[0].get("response", {}).get("legal"),
            "all_choices_equal": len(set(choices)) <= 1,
            "results": results,
        })
    return out


def decision_metrics(decisions):
    elapsed = []
    route_counts = Counter()
    timeouts = 0
    over_budget = 0
    for event in decisions:
        response = event.get("response", {})
        value = response.get("elapsed_us")
        if isinstance(value, (int, float)):
            elapsed.append(value)
        mode, route = response.get("mode"), response.get("route")
        if isinstance(mode, str) and isinstance(route, str):
            route_counts[mode + ":" + route] += 1
        over_budget += response.get("over_budget") is True
        phases = response.get("phases")
        if isinstance(phases, list):
            timeouts += sum(
                isinstance(phase, dict) and "timeout" in str(phase.get("status", ""))
                for phase in phases
            )
    return {
        "routes": dict(sorted(route_counts.items())),
        "timeouts": timeouts,
        "fallbacks": sum(count for key, count in route_counts.items() if "fallback" in key),
        "per_decision_seconds": {
            "max": seconds(max(elapsed)) if elapsed else None,
            "mean": seconds(sum(elapsed) / len(elapsed)) if elapsed else None,
            "over_budget": over_budget,
            "observed": len(elapsed),
        },
    }


def native_teams(modes, bidder, points, made):
    profiles = defaultdict(set)
    for seat, mode in enumerate(modes):
        if mode in NATIVE_MODES:
            profiles[seat % 2].add(mode)
    teams = []
    for parity in sorted(profiles):
        role = "declarer" if parity == bidder % 2 else "defender"
        teams.append({
            "parity": parity,
            "role": role,
            "profiles": sorted(profiles[parity]),
            "points": points[parity],
            "success": made if role == "declarer" else not made,
        })
    return teams


def completed_hands(events):
    decisions = defaultdict(list)
    starts = {}
    results = []
    for event in events:
        key = (event["_run"], event.get("hand", event.get("name")))
        if event.get("event") == "decision":
            decisions[key].append(event)
        elif event.get("event") == "hand-start":
            starts[(event["_run"], event.get("name"))] = event
        elif event.get("event") == "hand-result":
            results.append(event)

    hands = []
    completed_keys = set()
    for result in results:
        run_id, name = result["_run"], result.get("name")
        key = (run_id, name)
        completed_keys.add(key)
        modes = result.get("modes", [])
        points = result.get("points", [])
        made = result.get("made") is True
        metrics = decision_metrics(decisions.get(key, []))
        recorded_routes = result.get("routes")
        if isinstance(recorded_routes, dict):
            metrics["routes"] = dict(sorted(recorded_routes.items()))
            metrics["fallbacks"] = result.get("fallbacks", metrics["fallbacks"])
        hand = {
            "run": run_id,
            "run_label": run_label(run_id),
            "name": name,
            "base": base_name(name),
            "lineup": ":".join(modes),
            "contract": {
                "decl": result.get("decl"),
                "bid": result.get("bid"),
                "bidder": result.get("bidder"),
                "made": made,
                "points": points,
            },
            "routes": metrics["routes"],
            "literal_phone_on_all_phone_turns": result.get("literal_phone_on_all_phone_turns"),
            "max_trick_seconds": seconds(result.get("max_trick_us")),
            "per_decision_seconds": metrics["per_decision_seconds"],
            "elapsed_seconds": seconds(result.get("elapsed_us")),
            "timeouts": metrics["timeouts"],
            "fallbacks": metrics["fallbacks"],
            "native_teams": native_teams(modes, result.get("bidder"), points, made),
        }
        hands.append(hand)

    incomplete = []
    for key, start in sorted(starts.items()):
        if key in completed_keys:
            continue
        incomplete.append({
            "run": key[0],
            "run_label": run_label(key[0]),
            "name": key[1],
            "base": base_name(key[1]),
            "lineup": ":".join(start.get("modes", [])),
            "contract": {key: start.get(key) for key in ("decl", "bid", "bidder")},
            "decisions_observed": len(decisions.get(key, [])),
        })
    return hands, incomplete


def contract_key(hand):
    contract = hand["contract"]
    return (hand["base"], contract["decl"], contract["bid"], contract["bidder"])


def paired_comparison(hand, reference):
    contract = hand["contract"]
    baseline = reference["contract"]
    teams = []
    for team in hand["native_teams"]:
        parity = team["parity"]
        baseline_success = baseline["made"] if team["role"] == "declarer" else not baseline["made"]
        teams.append({
            "parity": parity,
            "role": team["role"],
            "profiles": team["profiles"],
            "all_phone_success": baseline_success,
            "native_success": team["success"],
            "success_delta": int(team["success"]) - int(baseline_success),
            "all_phone_points": baseline["points"][parity],
            "native_points": contract["points"][parity],
            "points_delta": contract["points"][parity] - baseline["points"][parity],
        })
    return {
        "run": hand["run"],
        "run_label": hand["run_label"],
        "lineup": hand["lineup"],
        "all_phone_reference": {"run": reference["run"], "name": reference["name"]},
        "all_phone_made": baseline["made"],
        "native_lineup_made": contract["made"],
        "contract_outcome_changed": baseline["made"] != contract["made"],
        "native_teams": teams,
    }


def group_hands(hands):
    groups = defaultdict(list)
    references = defaultdict(list)
    for hand in hands:
        key = contract_key(hand)
        groups[key].append(hand)
        if hand["lineup"] == "phone:phone:phone:phone":
            references[key].append(hand)

    output = []
    for key, observations in sorted(groups.items()):
        base, decl, bid, bidder = key
        refs = references.get(key, [])
        comparisons = []
        if len(refs) == 1:
            comparisons = [
                paired_comparison(hand, refs[0])
                for hand in observations
                if hand is not refs[0] and hand["native_teams"]
            ]
        output.append({
            "base": base,
            "contract": {"decl": decl, "bid": bid, "bidder": bidder},
            "completed_observations": len(observations),
            "all_phone_references": [
                {"run": ref["run"], "name": ref["name"]} for ref in refs
            ],
            "reference_ambiguous": len(refs) > 1,
            "hands": sorted(observations, key=lambda hand: (hand["run"], hand["name"])),
            "paired_vs_all_phone": comparisons,
        })
    return output


def main():
    run_dirs = sorted(path for path in RUNS.glob("*") if path.is_dir())
    events = []
    non_json_lines = 0
    for directory in run_dirs:
        found, ignored = read_events(directory / "stdout.log", directory.name)
        events.extend(found)
        non_json_lines += ignored

    hands, incomplete = completed_hands(events)
    grouped = group_hands(hands)
    summary = {
        "schema": "texas42-partnership-summary-v1",
        "runs": run_summary(run_dirs),
        "roots": root_summaries(events),
        "hands": {
            "unique_deals": len({hand["base"] for hand in hands}),
            "unique_deal_contracts": len({contract_key(hand) for hand in hands}),
            "completed_result_rows": len(hands),
            "groups": grouped,
            "incomplete_starts": {"count": len(incomplete), "items": incomplete},
        },
        "ignored_non_json_stdout_lines": non_json_lines,
    }
    print(json.dumps(summary, separators=(",", ":"), sort_keys=True))


if __name__ == "__main__":
    main()
