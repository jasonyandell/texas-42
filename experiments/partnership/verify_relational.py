#!/usr/bin/env python3
"""Audit a completed relational campaign without running the native solver."""

import argparse
from fractions import Fraction
import hashlib
import json
import os
from pathlib import Path
import random
import re
import tempfile

import gym
from rules import legal_tiles, replay_record, trick_points, winner


HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"))


def digest(value):
    return hashlib.sha256(canonical(value).encode()).hexdigest()


def require(condition, message):
    if not condition:
        raise ValueError(message)


def require_pinned_source(path, expected, label):
    path = Path(path)
    candidates = [path]
    reference = HERE / "campaigns" / "relational-learning-v1" / "reference" / path.name
    if reference not in candidates and reference.exists():
        candidates.append(reference)
    matches = [candidate for candidate in candidates if candidate.exists() and sha(candidate) == expected]
    require(matches, "frozen source digest: " + label)
    return str(matches[0])


def request_fields(path):
    fields = {}
    for line in Path(path).read_text().splitlines():
        words = line.split()
        if words:
            require(words[0] not in fields, "duplicate request field: " + words[0])
            fields[words[0]] = [int(value) for value in words[1:]]
    require(
        set(fields) == {"decl", "bid", "bidder", "seat", "hand", "plays", "seed"},
        "request contains non-public or missing fields: " + str(path),
    )
    for name in ("decl", "bid", "bidder", "seat", "seed"):
        require(len(fields[name]) == 1, "request scalar has wrong arity: " + name)
        fields[name] = fields[name][0]
    return fields


def policy_initial(result, actor):
    wanted = result["payload"]["actors"][actor]["digest"]
    matches = [
        Path(path)
        for path, value in result["files"].items()
        if value == wanted and path.endswith(".policy")
    ]
    require(matches, "saved actor source absent for digest " + wanted)
    match = re.search(r"\(initial\s+([^\s()]+)\)", matches[0].read_text())
    require(match is not None, "saved actor has no initial mode: " + str(matches[0]))
    return match.group(1)


def resolution(points, bid, declaring_team):
    declared = points[declaring_team]
    unbanked = 42 - sum(points)
    if declared >= bid:
        return True
    if declared + unbanked < bid:
        return False
    return None


def audit_replay(request, replay, initial_mode, label):
    prefix = list(zip(request["plays"][::2], request["plays"][1::2]))
    history = [tuple(play) for play in replay["history"]]
    require(history[: len(prefix)] == prefix, label + ": root history is not a prefix")
    require(len(history) == 28, label + ": replay does not contain 28 actions")

    root_remaining = [set(hand) for hand in replay["remaining_hands"]]
    require(len(root_remaining) == 4, label + ": remaining_hands needs four seats")
    hands = [set(hand) for hand in root_remaining]
    for actor, tile in prefix:
        require(tile not in hands[actor], label + ": played root tile remained in hand")
        hands[actor].add(tile)
    require(all(len(hand) == 7 for hand in hands), label + ": reconstructed hand size")
    require(
        sorted(tile for hand in hands for tile in hand) == list(range(28)),
        label + ": reconstructed deal is not the full deck",
    )
    require(
        hands[request["seat"]] == set(request["hand"]),
        label + ": reconstructed viewer hand disagrees with request",
    )

    traces = iter(replay["focal_decisions"])
    next_trace = next(traces, None)
    remaining = [set(hand) for hand in hands]
    points = [0, 0]
    lead = request["bidder"]
    trick = []
    depth = 0
    prior_mode = initial_mode
    for index, (actor, tile) in enumerate(history):
        require(actor == (lead + len(trick)) % 4, label + ": turn order")
        require(tile in legal_tiles(remaining[actor], trick, request["decl"]), label + ": illegal play")
        if index >= len(prefix) and actor == request["seat"]:
            require(next_trace is not None, label + ": missing focal trace")
            require(next_trace["depth"] == depth, label + ": focal depth")
            require(next_trace["action"] == tile, label + ": focal action")
            require(next_trace["mode_before"] == prior_mode, label + ": controller mode chain")
            require(
                next_trace["contract_resolved"]
                == resolution(points, request["bid"], request["bidder"] % 2),
                label + ": contract status",
            )
            source = next_trace["source"]
            require(
                source in ("exact", "fallback") or source.startswith("rule:"),
                label + ": unknown provenance",
            )
            if source in ("exact", "fallback"):
                require(
                    next_trace["mode_after"] == next_trace["mode_before"],
                    label + ": exact/fallback changed mode",
                )
            require(type(next_trace["work"]) is int and next_trace["work"] >= 0, label + ": work")
            prior_mode = next_trace["mode_after"]
            depth += 1
            next_trace = next(traces, None)
        remaining[actor].remove(tile)
        trick.append((actor, tile))
        if len(trick) == 4:
            lead = winner(trick, request["decl"])
            points[lead % 2] += trick_points(trick)
            trick = []
    require(next_trace is None, label + ": extra focal trace")
    require(not trick and all(not hand for hand in remaining), label + ": incomplete terminal state")
    require(sum(points) == 42, label + ": score does not conserve 42 points")
    require(replay["made"] == (points[request["bidder"] % 2] >= request["bid"]), label + ": made")

    independent_points, _, independent_remaining, independent_trick = replay_record(
        hands,
        [value for play in history for value in play],
        request["decl"],
        request["bidder"],
    )
    require(independent_points == points, label + ": independent score parity")
    require(not independent_trick and all(not hand for hand in independent_remaining), label + ": independent terminal parity")
    return depth


def audit_jobs(campaign):
    jobs = sorted((campaign / "jobs").glob("*/result.json"))
    evaluations = 0
    replays = 0
    focal_traces = 0
    for path in jobs:
        result = json.loads(path.read_text())
        require(digest(result["payload"]) == result["payload_sha256"], "payload digest: " + str(path))
        for artifact, expected in result["files"].items():
            require(sha(artifact) == expected, "artifact digest: " + artifact)
        actors = result["payload"].get("actors")
        if actors is None:
            continue
        evaluations += 1
        request = request_fields(result["spec"]["options"]["request"])
        for actor, report in actors.items():
            initial_mode = policy_initial(result, actor)
            for index, replay in enumerate(report["replays"]):
                label = f"{path.parent.name}/{actor}/replay-{index}"
                focal_traces += audit_replay(request, replay, initial_mode, label)
                replays += 1
    return {
        "completed_jobs": len(jobs),
        "evaluation_jobs": evaluations,
        "retained_replays": replays,
        "focal_decision_traces": focal_traces,
    }


def audit_splits(campaign, manifest):
    roots = json.loads((campaign / "roots.json").read_text())
    counts = {name: 0 for name in manifest["counts"]}
    hands = {}
    for name, root in roots.items():
        split = root["split"]
        require(split in counts and name.startswith(split + "-"), "root split/name mismatch")
        counts[split] += 1
        request = request_fields(root["request_path"])
        require(root["request"] == Path(root["request_path"]).read_text(), "root request parity")
        hand = tuple(sorted(request["hand"]))
        require(hand not in hands or hands[hand] == split, "starting hand crosses data splits")
        hands[hand] = split
    require(counts == manifest["counts"], "root split counts disagree with manifest")
    require(len(roots) == sum(counts.values()), "root count")
    return counts


def audit_summary(campaign, manifest):
    summary = json.loads((campaign / "summary.json").read_text())
    test = {}
    for path in sorted((campaign / "jobs").glob("test-*/result.json")):
        test[path.parent.name.removeprefix("test-")] = json.loads(path.read_text())["payload"]
    require(summary["root_groups"] == len(test) == manifest["counts"]["test"], "test root count")
    names = sorted(next(iter(test.values()))["actors"])
    require(sorted(summary["arms"]) == names, "summary actor set")
    for name in names:
        rows = [test[key]["actors"][name] for key in sorted(test)]
        mean = lambda key: str(sum((Fraction(row[key]) for row in rows), Fraction()) / len(rows))
        expected = {
            "value": mean("value"),
            "policy_regret": mean("policy_regret"),
            "root_regret": mean("root_regret"),
            "first_fallback_probability": mean("first_fallback_probability"),
            "digests": sorted({row["digest"] for row in rows}),
            "inference_work": sum(row["inference_work"] for row in rows),
        }
        require(summary["arms"][name] == expected, "summary arm parity: " + name)
        if name == "table":
            continue
        differences = [
            Fraction(test[key]["actors"][name]["value"])
            - Fraction(test[key]["actors"]["table"]["value"])
            for key in sorted(test)
        ]
        rng = random.Random(420907)
        bootstrap = sorted(
            float(sum((rng.choice(differences) for _ in differences), Fraction()) / len(differences))
            for _ in range(2000)
        )
        expected_pair = {
            "mean": str(sum(differences, Fraction()) / len(differences)),
            "bootstrap_95_percentile": [bootstrap[50], bootstrap[1949]],
            "positive_roots": sum(value > 0 for value in differences),
            "negative_roots": sum(value < 0 for value in differences),
            "interpretation": "descriptive paired root bootstrap; narrow conditional endgame target",
        }
        require(summary["paired"][name + "-minus-table"] == expected_pair, "paired parity: " + name)

    frozen = json.loads((campaign / "frozen" / "manifest.json").read_text())
    require(frozen["actors"] == summary["frozen"], "summary/frozen actor parity")
    require(frozen["manifest"] == digest(manifest), "frozen manifest identity")
    for actor in frozen["actors"].values():
        require(sha(actor["path"]) == actor["sha256"], "frozen actor digest")
    require(summary["manifest_sha256"] == digest(manifest), "summary manifest identity")
    return names


def audit_exam_summary(campaign, manifest):
    summary = json.loads((campaign / "summary.json").read_text())
    results = {
        path.parent.name.removeprefix("exam-"): json.loads(path.read_text())["payload"]
        for path in sorted((campaign / "jobs").glob("exam-*/result.json"))
    }
    require(summary["diagnostic_only"] is True, "exam must remain diagnostic-only")
    require(summary["coordinate_count"] == len(results), "exam coordinate count")
    require(
        summary["averaging"] == "equal coordinate weights; related coordinates share a source deal",
        "exam averaging authority",
    )
    names = sorted(next(iter(results.values()))["actors"])
    require(sorted(summary["arms"]) == names, "exam actor set")
    for name in names:
        rows = [results[key]["actors"][name] for key in sorted(results)]
        mean = lambda key: str(sum((Fraction(row[key]) for row in rows), Fraction()) / len(rows))
        expected = {
            "value": mean("value"),
            "policy_regret": mean("policy_regret"),
            "root_regret": mean("root_regret"),
            "first_fallback_probability": mean("first_fallback_probability"),
            "digests": sorted({row["digest"] for row in rows}),
            "inference_work": sum(row["inference_work"] for row in rows),
            "optimal_root_actions": sum(Fraction(row["root_regret"]) == 0 for row in rows),
        }
        require(summary["arms"][name] == expected, "exam summary arm parity: " + name)

    cases = dict(gym.gallery(Path(manifest["source"])))
    require(set(cases) == set(results), "exam/source coordinate identity")
    require(gym.exam_identity(cases) == manifest["exam_identity"], "exam identity digest")
    require(
        summary["source_deal_groups"] == len({case["request"]["seed"] for case in cases.values()}),
        "exam source-deal groups",
    )
    for name, case in cases.items():
        expected = max(
            Fraction(action["success_mass"], case["key"]["worlds"])
            for action in case["key"]["actions"]
        )
        require(Fraction(results[name]["optimum"]) == expected, "exam maintained optimum: " + name)

    for actor in manifest["actors"].values():
        require(sha(actor["path"]) == actor["sha256"], "exam frozen actor digest")
    learned = None
    for parent in Path(next(iter(manifest["actors"].values()))["path"]).parents:
        candidate = parent / "frozen" / "manifest.json"
        if candidate.exists():
            learned = candidate
            break
    require(learned is not None, "exam learned frozen manifest not found")
    require(sha(learned) == manifest["learned_manifest_sha256"], "exam learned manifest digest")
    return names, len(cases)


def atomic_json(path, value):
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    fd, temporary = tempfile.mkstemp(prefix=path.name + ".", dir=path.parent)
    try:
        with os.fdopen(fd, "w") as stream:
            json.dump(value, stream, sort_keys=True, indent=2)
            stream.write("\n")
            stream.flush()
            os.fsync(stream.fileno())
        os.replace(temporary, path)
    finally:
        if os.path.exists(temporary):
            os.unlink(temporary)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("campaign", type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    campaign = args.campaign.resolve()
    manifest = json.loads((campaign / "manifest.json").read_text())
    status = json.loads((campaign / "status.json").read_text())
    require(status["phase"] == "complete" and not status["active_jobs"], "campaign is incomplete")
    if manifest["schema"] == "texas42-relational-campaign-v1":
        require(sha(manifest["binary"]) == manifest["binary_sha256"], "frozen binary digest")
        pinned_sources = {}
        for source, expected in manifest["sources"].items():
            pinned_sources[source] = require_pinned_source(ROOT / source, expected, source)
        groups = audit_splits(campaign, manifest)
        actors = audit_summary(campaign, manifest)
        group_field = {"splits": groups}
    elif manifest["schema"] == "texas42-relational-exam-v1":
        require(
            sha(ROOT / "walt/target/release/relational_lab") == manifest["binary_sha256"],
            "exam frozen binary digest",
        )
        pinned_sources = {}
        for source, expected in manifest["sources"].items():
            pinned_sources[source] = require_pinned_source(source, expected, source)
        actors, coordinates = audit_exam_summary(campaign, manifest)
        group_field = {"exam_coordinates": coordinates}
    else:
        raise ValueError("unsupported relational manifest schema")
    job_counts = audit_jobs(campaign)
    report = {
        "schema": "texas42-relational-static-verification-v1",
        "campaign": str(campaign),
        "manifest_sha256": digest(manifest),
        "binary_sha256": manifest["binary_sha256"],
        "verified_source_paths": pinned_sources,
        "status": "verified",
        **group_field,
        "test_actors": actors,
        **job_counts,
        "checks": [
            "immutable job payload and artifact digests",
            "declared root groups or maintained exam coordinates",
            "frozen actor, manifest, and summary parity",
            "full-deal reconstruction from root-remaining hands plus public prefix",
            "independent 28-play legality, 42-point score, empty-hand, and made parity",
            "focal depth, action, provenance, controller mode, contract state, and work",
        ],
    }
    if args.output:
        atomic_json(args.output, report)
    print(json.dumps(report, sort_keys=True))


if __name__ == "__main__":
    main()
