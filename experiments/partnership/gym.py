#!/usr/bin/env python3
"""Small exact partnership exercises. Own hand/public play in; audited keys out.

Mine and run are bounded, parallel, atomic per item, and safe to resume. Raw
mining runs belong outside git; selected, independently verified exercises are
portable JSON fixtures. See walt/gym/README.md for the mathematical contract.
"""
import argparse
from concurrent.futures import ThreadPoolExecutor, wait, FIRST_COMPLETED
from contextlib import contextmanager
from fractions import Fraction
import fcntl
import hashlib
from itertools import combinations
import json
import os
from pathlib import Path
import signal
import subprocess
import tempfile
import threading
import time

from rules import TILES, information_state, legal_tiles, replay_record, trick_points, winner

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
ENGINE = ROOT / "walt/target/release/partnership_gym"
DEFAULT_SOURCE = HERE / "campaigns/default-partner-battery/01-level"
DEFAULT_GALLERY = ROOT / "walt/gym/scenarios"
INPUT_KEYS = {"decl", "bid", "bidder", "seat", "hand", "plays", "seed"}
SEMANTICS = {
    "belief": "Uniform over every mechanically compatible current deal; no pre-root policy likelihood.",
    "field": "Fixed seat-local L1 fixed/voidless teammate, L0 opponents; deterministic declared state seed.",
    "continuation": "Full legal focal best response, one action per focal information state; future field-action conditioning.",
    "objective": "Viewer's team makes or sets bid 30; every equal success mass ties.",
    "grading": "Root-action regret with optimal focal continuation; not the pupil's complete policy value.",
    "count": "Diagnostic only; after contract settles the extracted focal policy completes with lowest legal plays.",
}
if not __debug__:
    raise RuntimeError("gym verification requires Python assertions; do not run with -O")


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"))


def digest(value):
    return hashlib.sha256(canonical(value).encode()).hexdigest()


def file_hash(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def atomic(path, value):
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    temp = path.with_suffix(path.suffix + ".tmp")
    with temp.open("w") as out:
        json.dump(value, out, sort_keys=True, separators=(",", ":"))
        out.write("\n")
        out.flush()
        os.fsync(out.fileno())
    temp.replace(path)
    fd = os.open(path.parent, os.O_RDONLY)
    try:
        os.fsync(fd)
    finally:
        os.close(fd)


@contextmanager
def run_lock(directory):
    directory.mkdir(parents=True, exist_ok=True)
    with (directory / ".lock").open("a") as lock:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        yield


def pin(directory, manifest):
    path = directory / "manifest.json"
    if path.exists():
        if json.loads(path.read_text()) != manifest:
            raise ValueError("resume identity changed; use a new output directory")
    else:
        atomic(path, manifest)


def pupil_request(req):
    # Reject accidental teacher fields instead of quietly forwarding them.
    if set(req) != INPUT_KEYS or req["bid"] != 30:
        raise ValueError("pupil request must contain only the seven public/own-hand fields; bid 30")
    if type(req["seed"]) is not int or not 0 <= req["seed"] < 2**64:
        raise ValueError("seed must be an unsigned 64-bit integer")
    information_state(req)
    return json.loads(canonical(req))


def native(req, *, inspect=False, max_worlds=400, partner_worlds=40, seconds=45, query=None):
    req = pupil_request(req)
    lines = []
    for key in sorted(req):
        v = req[key]
        lines.append(key + " " + " ".join(map(str, v if isinstance(v, list) else [v])))
    command = [str(ENGINE), "--max-worlds", str(max_worlds), "--partner-worlds", str(partner_worlds)]
    if inspect:
        command.append("--inspect")
    if query is not None:
        command.extend(["--query", str(query)])
    result = subprocess.run(command, input="\n".join(lines) + "\n", text=True,
                            capture_output=True, timeout=seconds,
                            env={**os.environ, "RAYON_NUM_THREADS": "1"})
    if result.returncode:
        raise ValueError(result.stderr.strip()[-2000:])
    return json.loads(result.stdout)


def candidates(source, max_trick=6):
    """Public pattern selection only. Actual hidden hands never enter a request."""
    seen = set()
    for path in sorted(source.glob("seeds/*/*/result.json")):
        result = json.loads(path.read_text())
        fixture = result["fixture"]
        if fixture["bid"] != 30:
            continue
        checkpoint = json.loads(path.with_name("checkpoint.json").read_text())
        record, trick, points = [], [], [0, 0]
        remaining = [set(h) for h in fixture["hands"]]
        for index, decision in enumerate(checkpoint["decisions"]):
            seat, tile = decision["seat"], decision["response"]["choice"]
            decl = fixture["decl"]
            legal = legal_tiles(remaining[seat], trick, decl)
            count = lambda t: sum(TILES[t]) in (5, 10)
            offers = [t for t in legal if count(t)
                      and len(trick) >= 2 and winner(trick, decl) == seat ^ 2
                      and winner(trick + [(seat, t)], decl) == seat ^ 2]
            declaring = fixture["bidder"] % 2
            if (5 <= index // 4 + 1 <= max_trick and offers and set(legal) - set(offers)
                    and points[declaring] < 30 and points[1 - declaring] <= 12):
                req = dict(decl=decl, bid=30, bidder=fixture["bidder"], seat=seat,
                           hand=fixture["hands"][seat], plays=record[:], seed=420600)
                key = digest(req)[:20]
                if key not in seen:
                    seen.add(key)
                    yield dict(id=key, request=req, source={
                        "path": str(path.relative_to(source)), "seed": result["seed"],
                        "decision": index, "result_sha256": file_hash(path),
                        "checkpoint_sha256": file_hash(path.with_name("checkpoint.json"))})
            assert tile in legal
            record.extend([seat, tile])
            remaining[seat].remove(tile)
            trick.append((seat, tile))
            if len(trick) == 4:
                points[winner(trick, decl) % 2] += trick_points(trick)
                trick = []


def positions(sources, min_trick=5, max_trick=6):
    """Generic legal-coordinate stream. No partnership pattern lives here.

    Scope filters are only stage, choice availability, and unsettled contract.
    Scheme is responsible for every pattern match in `discover`.
    """
    seen = set()
    for source in sources:
        for path in sorted(source.rglob("result.json")):
            checkpoint = path.with_name("checkpoint.json")
            if not checkpoint.exists():
                continue
            result = json.loads(path.read_text())
            fixture = result.get("fixture", {})
            if fixture.get("bid") != 30:
                continue
            record, trick, points = [], [], [0, 0]
            hands = [set(h) for h in fixture["hands"]]
            for index, decision in enumerate(json.loads(checkpoint.read_text())["decisions"]):
                seat, tile = decision["seat"], decision["response"]["choice"]
                decl, declaring = fixture["decl"], fixture["bidder"] % 2
                legal = legal_tiles(hands[seat], trick, decl)
                if (min_trick <= index // 4 + 1 <= max_trick and len(legal) > 1
                        and points[declaring] < 30 and points[1 - declaring] <= 12):
                    req = dict(decl=decl, bid=30, bidder=fixture["bidder"], seat=seat,
                               hand=sorted(fixture["hands"][seat]), plays=record[:], seed=420600)
                    canonical_req = canonical(req)
                    if canonical_req not in seen:
                        seen.add(canonical_req)
                        yield dict(id=digest(req)[:20], request=req, source={
                            "path": str(path.relative_to(source)), "corpus": str(source),
                            "seed": result["seed"], "decision": index,
                            "result_sha256": file_hash(path), "checkpoint_sha256": file_hash(checkpoint)})
                assert tile in legal
                record.extend([seat, tile])
                hands[seat].remove(tile)
                trick.append((seat, tile))
                if len(trick) == 4:
                    points[winner(trick, decl) % 2] += trick_points(trick)
                    trick = []


def world_key(hands):
    return tuple(tuple(sorted(h)) for h in hands)


def compatible_worlds(req):
    """Independent enumeration, checked by replaying each reconstructed deal.

    For the deliberately small (trick 5+) gym. Does not trust native voids,
    counts, or output traces to decide what belongs to the legal fiber.
    """
    information_state(req)
    played = [set() for _ in range(4)]
    for s, t in zip(req["plays"][::2], req["plays"][1::2]):
        played[s].add(t)
    viewer = req["seat"]
    own = set(req["hand"]) - played[viewer]
    if len(req["plays"]) < 32:
        raise ValueError("independent gym verifier requires trick 5 or later")
    pool = set(range(28)) - set(req["plays"][1::2]) - own
    others = [s for s in range(4) if s != viewer]
    hands = [set() for _ in range(4)]
    hands[viewer] = own
    answer = set()

    def visit(left, slots):
        if not slots:
            full = [sorted(hands[s] | played[s]) for s in range(4)]
            try:
                _, _, remaining, _ = replay_record(full, req["plays"], req["decl"], req["bidder"])
            except AssertionError:
                return
            answer.add(world_key(remaining))
            return
        s = slots[0]
        for choice in combinations(sorted(left), 7 - len(played[s])):
            hands[s] = set(choice)
            visit(left - hands[s], slots[1:])

    visit(pool, others)
    return answer


def classify(key, targets=None):
    masses = {a["tile"]: a["success_mass"] for a in key["actions"]}
    offers = set(key["offers"] if targets is None else targets)
    if not offers <= set(masses):
        raise ValueError("query target is not a legal action")
    pairs = []
    for offer in sorted(offers):
        for other in sorted(set(masses) - offers):
            if offer in key["best"] and masses[offer] > masses[other]:
                pairs.append(dict(category="advantage", preferred=offer, comparison=other,
                                  gap_mass=masses[offer] - masses[other]))
            elif other in key["best"] and masses[other] > masses[offer]:
                pairs.append(dict(category="disadvantage", preferred=other, comparison=offer,
                                  gap_mass=masses[other] - masses[offer]))
    return sorted(pairs, key=lambda p: (-p["gap_mass"], p["category"], p["preferred"], p["comparison"]))


def outcome_profile(key):
    """Derived decision geometry, using success mass alone. No point tie-break."""
    masses = {a["tile"]: a["success_mass"] for a in key["actions"]}
    total = key["worlds"]
    if total <= 0 or not masses or any(not 0 <= m <= total for m in masses.values()):
        raise ValueError("invalid outcome masses")
    best, worst = max(masses.values()), min(masses.values())
    optimal = sorted(t for t, m in masses.items() if m == best)
    gaps = [best - m for m in masses.values() if m < best]
    return dict(schema="gym-outcome-v1", legal_count=len(masses), optimal=optimal,
                optimal_count=len(optimal), strict=best > worst,
                unique_best=len(optimal) == 1 and bool(gaps),
                best_success=str(Fraction(best, total)), worst_success=str(Fraction(worst, total)),
                spread=str(Fraction(best - worst, total)),
                nearest_mistake=str(Fraction(min(gaps), total)) if gaps else None,
                guaranteed_success=sorted(t for t, m in masses.items() if m == total),
                guaranteed_failure=sorted(t for t, m in masses.items() if m == 0),
                certain_success_failure_swing=best == total and worst == 0,
                actions=[dict(tile=t, success=str(Fraction(m, total)),
                              regret=str(Fraction(best - m, total))) for t, m in sorted(masses.items())])


def case_pairs(case):
    """Keep structural contrast and pure outcome selection explicitly distinct."""
    criterion = case.get("criterion", "query")
    if criterion == "query":
        return classify(case["key"], case.get("target_actions"))
    if criterion != "outcome":
        raise ValueError("unknown exercise selection criterion")
    key, req = case["key"], case["request"]
    masses = {a["tile"]: a["success_mass"] for a in key["actions"]}
    profile = outcome_profile(key)
    category = "bid-making" if req["seat"] % 2 == req["bidder"] % 2 else "bid-setting"
    pairs = [dict(category=category, preferred=a, comparison=b, gap_mass=masses[a] - masses[b])
             for a in profile["optimal"] for b in sorted(masses) if masses[a] > masses[b]]
    return sorted(pairs, key=lambda p: (-p["gap_mass"], p["preferred"], p["comparison"]))


def paired_outcomes(key, pair):
    """Same-world outcomes of two attained lawful continuation policies."""
    actions = {a["tile"]: a for a in key["actions"]}
    good = {world_key(t["hands"]): t["success"] for t in actions[pair["preferred"]]["traces"]}
    other = {world_key(t["hands"]): t["success"] for t in actions[pair["comparison"]]["traces"]}
    assert good.keys() == other.keys() and len(good) == key["worlds"]
    counts = dict(gained=0, lost=0, both_success=0, both_failure=0)
    for world, succeeds in good.items():
        label = ("both_success" if other[world] else "gained") if succeeds else ("lost" if other[world] else "both_failure")
        counts[label] += 1
    assert counts["gained"] - counts["lost"] == pair["gap_mass"]
    return counts


def verify(case):
    """Independent support, rules, scores, and on-support information audit.

    Optimality is supplied by the exact Rust best-response recurrence. This
    audit verifies attained values and lawful witnesses, not a second optimizer.
    """
    req, key = pupil_request(case["request"]), case["key"]
    assert case["semantics"] == SEMANTICS
    state = information_state(req)
    assert key["schema"] == "partnership-gym-v1"
    assert key["legal"] == state["legal"]
    assert key["leader"] == state["leader"] and key["banked"] == state["points"]
    assert key["trick"] == state["trick"]
    worlds = compatible_worlds(req)
    assert len(worlds) == key["worlds"] > 0
    viewer, decl = req["seat"], req["decl"]
    declared = req["bidder"] % 2
    own_played = {t for s, t in zip(req["plays"][::2], req["plays"][1::2]) if s == viewer}
    assert key["remaining"] == sorted(set(req["hand"]) - own_played)
    assert sorted(a["tile"] for a in key["actions"]) == key["legal"]
    decisions = {}
    for action in key["actions"]:
        assert len(action["traces"]) == len(worlds)
        assert {world_key(t["hands"]) for t in action["traces"]} == worlds
        bins, successes = [0] * 43, 0
        for trace in action["traces"]:
            full = [list(h) for h in trace["hands"]]
            for s, t in zip(req["plays"][::2], req["plays"][1::2]):
                full[s].append(t)
            points, lead, hands, trick = replay_record(full, req["plays"], decl, req["bidder"])
            assert [t for _, t in trick] == key["prefix"]
            offers = [t for t in key["legal"] if sum(TILES[t]) in (5, 10)
                      and len(trick) >= 2 and winner(trick, decl) == viewer ^ 2
                      and winner(trick + [(viewer, t)], decl) == viewer ^ 2]
            assert key["offers"] == offers
            suffix = trace["plays"]
            assert len(suffix) % 2 == 0 and suffix[:2] == [viewer, action["tile"]]
            public, partner_count = req["plays"][:], 0
            for s, t in zip(suffix[::2], suffix[1::2]):
                assert s == (lead + len(trick)) % 4
                assert t in legal_tiles(hands[s], trick, decl)
                # Field choices must agree across all hidden worlds AND root
                # actions. Focal continuation may differ by forced root action.
                info = (action["tile"] if s == viewer else "field", s,
                        tuple(sorted(hands[s])), tuple(public))
                assert decisions.setdefault(info, t) == t, "hidden-information inconsistent choice"
                hands[s].remove(t)
                trick.append((s, t))
                public.extend([s, t])
                if len(trick) == 4:
                    lead = winner(trick, decl)
                    captured = trick_points(trick)
                    points[lead % 2] += captured
                    if lead == viewer ^ 2:
                        partner_count += captured - 1
                    trick = []
            assert not any(hands) and not trick and sum(points) == 42
            assert points == trace["banked"] and partner_count == trace["partner_count"]
            success = (points[declared] >= 30) == (viewer % 2 == declared)
            assert success == trace["success"]
            successes += success
            bins[points[declared]] += 1
        assert bins == action["score_bins"] and successes == action["success_mass"]
    best_mass = max(a["success_mass"] for a in key["actions"])
    assert key["best"] == [a["tile"] for a in key["actions"] if a["success_mass"] == best_mass]
    if "pair" in case:
        assert case["pair"] in case_pairs(case)
    if "categories" in case:
        assert case["categories"] == case_pairs(case)
    if "outcome" in case:
        assert case["outcome"] == outcome_profile(key)
    if "paired_outcomes" in case:
        assert case["paired_outcomes"] == paired_outcomes(key, case["pair"])
    if "query_match" in case:
        found = case["query_match"]
        assert found["worlds"] == key["worlds"]
        threshold = Fraction(case["min_presence"])
        assert 0 < threshold <= 1
        assert case["target_actions"] == [t for t, mass in found["presence"]
                                            if Fraction(mass, key["worlds"]) >= threshold]
        # Re-run the declared expression, not a pattern-specific Python copy.
        # Support and attained values above remain independently audited.
        with tempfile.TemporaryDirectory() as d:
            path = Path(d) / "query.scheme"
            path.write_text(found["source"])
            recomputed = native(req, inspect=True, query=path, max_worlds=key["worlds"], seconds=5)
        assert recomputed["query_match"] == found
    return dict(worlds=len(worlds), actions=len(key["actions"]), information_sets=len(decisions))


def grade(key, choice):
    masses = {a["tile"]: a["success_mass"] for a in key["actions"]}
    if choice not in masses:
        raise ValueError("illegal pupil choice")
    regret = Fraction(max(masses.values()) - masses[choice], key["worlds"])
    return dict(choice=choice, optimal=regret == 0,
                regret=str(regret), success=str(Fraction(masses[choice], key["worlds"])))


def bounded(items, job, directory, workers, seconds, case_seconds):
    """No new jobs after deadline or interrupt. Active jobs drain with timeout.

    Failed items retry on the next invocation, once per invocation. SIGKILL can
    lose only active items; complete JSON files remain intact and reusable.
    """
    stopped = threading.Event()
    deadline = time.monotonic() + seconds
    old = {sig: signal.signal(sig, lambda *_: stopped.set()) for sig in (signal.SIGINT, signal.SIGTERM)}
    iterator = iter(items)
    completed, failed = 0, 0
    try:
        with ThreadPoolExecutor(max_workers=workers) as pool:
            pending = {}
            exhausted = False
            while pending or not exhausted:
                while len(pending) < workers and not exhausted:
                    if stopped.is_set() or time.monotonic() + case_seconds >= deadline:
                        exhausted = True
                        break
                    item = next(iterator, None)
                    if item is None:
                        exhausted = True
                        break
                    path = directory / "items" / (item["id"] + ".json")
                    if path.exists():
                        continue
                    pending[pool.submit(job, item)] = (item, path)
                if not pending:
                    continue
                done, _ = wait(pending, timeout=0.25, return_when=FIRST_COMPLETED)
                for future in done:
                    item, path = pending.pop(future)
                    try:
                        result = future.result()
                        atomic(path, result)
                        completed += 1
                        print(canonical({"completed": item["id"], "category": result.get("categories"),
                                         "grade": result.get("grade")}), flush=True)
                    except Exception as error:
                        failed += 1
                        atomic(directory / "failures" / (item["id"] + ".json"),
                               {"id": item["id"], "error": str(error) or type(error).__name__})
                        print(canonical({"failed": item["id"], "error": str(error)}), flush=True)
                atomic(directory / "status.json", {"completed_this_slice": completed, "failed_this_slice": failed,
                       "saved_total": len(list((directory / "items").glob("*.json"))),
                       "active": [x[0]["id"] for x in pending.values()], "stopping": stopped.is_set()})
    finally:
        for sig, handler in old.items():
            signal.signal(sig, handler)


def mine(args):
    selected = list(candidates(args.source))[:args.limit]
    manifest = dict(schema="gym-mine-v1", engine=file_hash(ENGINE), runner=file_hash(__file__),
                    rules=file_hash(HERE / "rules.py"), candidates=selected,
                    max_worlds=args.max_worlds, partner_worlds=args.partner_worlds)

    def job(item):
        start = time.monotonic()
        info = native(item["request"], inspect=True, seconds=3)
        if info["worlds"] > args.max_worlds:
            return {**item, "skipped": "world cap", "worlds": info["worlds"]}
        key = native(item["request"], max_worlds=args.max_worlds,
                     partner_worlds=args.partner_worlds, seconds=args.case_seconds - 3)
        case = {**item, "key": key, "categories": classify(key), "semantics": SEMANTICS}
        case["audit"] = verify(case)
        case["elapsed_seconds"] = round(time.monotonic() - start, 6)
        return case

    with run_lock(args.output):
        pin(args.output, manifest)
        bounded(selected, job, args.output, args.workers, args.seconds, args.case_seconds)


def discover(args, *, coordinates=None, query_source=None, query_name=None, identity=None):
    """Scheme is the matcher; the exact grader is independent of its answers."""
    query_source = args.query.read_text() if query_source is None else query_source
    query_name = args.query.stem if query_name is None else query_name
    # Explicit limit counts coordinates examined, not successful matches.
    from itertools import islice
    selected = list(islice(positions(args.source, args.min_trick, args.max_trick)
                          if coordinates is None else coordinates, args.limit))
    if not selected:
        raise ValueError("no eligible recorded coordinates in source")
    manifest = dict(schema="gym-discovery-v1", engine=file_hash(ENGINE), runner=file_hash(__file__),
                    rules=file_hash(HERE / "rules.py"), candidates=selected,
                    query_source=query_source, query_name=query_name,
                    min_presence=str(Fraction(args.min_presence)), max_worlds=args.max_worlds,
                    partner_worlds=args.partner_worlds, min_trick=args.min_trick, max_trick=args.max_trick)
    if identity is not None:
        manifest["generator"] = identity
    threshold = Fraction(args.min_presence)

    def job(item):
        start = time.monotonic()
        info = native(item["request"], inspect=True, seconds=3)
        if info["worlds"] > args.max_worlds:
            return {**item, "skipped": "world cap", "worlds": info["worlds"]}
        matched = native(item["request"], inspect=True, query=args.output / "query.scheme",
                         max_worlds=args.max_worlds, seconds=5)["query_match"]
        targets = [tile for tile, mass in matched["presence"]
                   if Fraction(mass, matched["worlds"]) >= threshold]
        base = {**item, "query_match": matched, "target_actions": targets,
                "min_presence": str(threshold), "family": query_name}
        if not targets:
            return {**base, "skipped": "no query match"}
        key = native(item["request"], max_worlds=args.max_worlds,
                     partner_worlds=args.partner_worlds, seconds=args.case_seconds - 13)
        case = {**base, "key": key, "categories": classify(key, targets), "semantics": SEMANTICS}
        case["audit"] = verify(case)
        case["elapsed_seconds"] = round(time.monotonic() - start, 6)
        return case

    with run_lock(args.output):
        pin(args.output, manifest)
        snapshot = args.output / "query.scheme"
        if snapshot.exists() and snapshot.read_text() != query_source:
            raise ValueError("frozen query snapshot changed")
        snapshot.write_text(query_source)
        checked = subprocess.run([str(ENGINE), "--check-query", str(snapshot)], capture_output=True, text=True, timeout=3)
        if checked.returncode:
            raise ValueError(checked.stderr.strip())
        print(canonical({"coordinates": len(selected), "query": query_name,
                         "min_presence": str(threshold)}), flush=True)
        bounded(selected, job, args.output, args.workers, args.seconds, args.case_seconds)


def inventory(args):
    """Report every measured coordinate, including no-match and tie controls."""
    rows = [json.loads(p.read_text()) for p in sorted((args.source / "items").glob("*.json"))]
    manifest = json.loads((args.source / "manifest.json").read_text())
    from collections import Counter
    measured = [r for r in rows if "key" in r]
    failures = [p.stem for p in (args.source / "failures").glob("*.json")
                if not (args.source / "items" / p.name).exists()]
    report = dict(coordinates_planned=len(manifest["candidates"]), saved=len(rows),
                  exact_keys=len(measured), skipped=dict(Counter(r["skipped"] for r in rows if "skipped" in r)),
                  strict_advantage=sum(any(p["category"] == "advantage" for p in r["categories"]) for r in measured),
                  strict_disadvantage=sum(any(p["category"] == "disadvantage" for p in r["categories"]) for r in measured),
                  no_strict_pair=sum(not r["categories"] for r in measured), failed=len(failures))
    report["outcomes"] = {}
    for side in ("declaring", "defending"):
        profiles = [outcome_profile(r["key"]) for r in measured if side_of(r) == side]
        report["outcomes"][side] = dict(measured=len(profiles),
            strict=sum(p["strict"] for p in profiles), unique_best=sum(p["unique_best"] for p in profiles),
            certain_success_failure_swing=sum(p["certain_success_failure_swing"] for p in profiles),
            all_tied=sum(not p["strict"] for p in profiles))
    print(json.dumps(report, indent=2))


def side_of(case):
    req = case["request"]
    return "declaring" if req["seat"] % 2 == req["bidder"] % 2 else "defending"


def select(args, *, predicate=None, allow_empty=False):
    cases = [json.loads(p.read_text()) for p in sorted((args.source / "items").glob("*.json"))]
    manifest = json.loads((args.source / "manifest.json").read_text())
    criterion, side = args.criterion, args.side
    categories = (["bid-making", "bid-setting"] if criterion == "outcome"
                  else ["advantage", "disadvantage"])
    if criterion == "outcome" and side != "both":
        categories = ["bid-making" if side == "declaring" else "bid-setting"]
    chosen, used = [], set()
    for category in categories:
        for case in cases:
            if case["id"] in used or "key" not in case or (side != "both" and side_of(case) != side):
                continue
            if predicate is not None and not predicate(case):
                continue
            selection_case = {**case, "criterion": criterion}
            all_pairs = case_pairs(selection_case)
            pairs = [p for p in all_pairs if p["category"] == category]
            if not pairs:
                continue
            if args.all and all_pairs[0]["category"] != category:
                continue
            specimen = {**selection_case, "categories": all_pairs, "pair": pairs[0], "provenance": {
                "engine_sha256": manifest["engine"], "runner_sha256": manifest["runner"],
                "rules_sha256": manifest["rules"], "mining_manifest_sha256": digest(manifest)}}
            if criterion == "outcome":
                specimen["outcome"] = outcome_profile(case["key"])
                specimen["paired_outcomes"] = paired_outcomes(case["key"], pairs[0])
            verify(specimen)
            chosen.append(specimen)
            used.add(case["id"])
            if not args.all and sum(c["pair"]["category"] == category for c in chosen) == args.each:
                break
    if (not chosen and not allow_empty) or (not args.all and any(sum(c["pair"]["category"] == k for c in chosen) < args.each for k in categories)):
        raise ValueError("not enough verified examples in requested categories")
    with run_lock(args.output):
        catalog = []
        for case in chosen:
            category = case["pair"]["category"]
            number = 1 + sum(c["id"].startswith(category) for c in catalog)
            name = f"{category}-{number:02d}"
            atomic(args.output / (name + ".json"), case)
            catalog.append(dict(id=name, file=name + ".json", sha256=file_hash(args.output / (name + ".json"))))
        selection = ("All strict cases, one per coordinate, strongest comparison pair; diagnostic outcome-selected gallery."
                     if args.all else "First stable-id cases per category; diagnostic outcome-selected gallery, not a strength sample.")
        atomic(args.output / "catalog.json", dict(schema="gym-catalog-v1", scenarios=catalog,
               selection=selection, criterion=criterion, side=side))
        if criterion == "outcome":
            # Preserve denominators, including ties and cap skips, beside the
            # portable fixtures. Full unselected replays stay in the raw run.
            summaries = []
            for case in cases:
                row = dict(id=case["id"], side=side_of(case))
                if "key" in case:
                    row.update(worlds=case["key"]["worlds"], outcome=outcome_profile(case["key"]))
                else:
                    row.update(skipped=case.get("skipped"), worlds=case.get("worlds"))
                summaries.append(row)
            atomic(args.output / "discovery.json", dict(manifest=manifest, rows=summaries,
                   raw_directory=str(args.source.resolve()), criterion=criterion, side=side))
    print(f"Selected {len(chosen)} independently verified exercises into {args.output}")


def gallery(directory):
    receipt = directory / "latest.json"
    if receipt.exists():
        latest = json.loads(receipt.read_text())
        if not latest.get("complete") or not latest.get("gallery"):
            raise ValueError("specification generation is incomplete; resume before using its collection")
        directory = Path(latest["gallery"])
    catalog = json.loads((directory / "catalog.json").read_text())
    for entry in catalog["scenarios"]:
        path = directory / entry["file"]
        if file_hash(path) != entry["sha256"]:
            raise ValueError("scenario digest mismatch: " + entry["id"])
        case = json.loads(path.read_text())
        yield entry["id"], case


def run(args):
    from matchup import Player
    from player import BINARY, decide
    configs = json.loads((HERE / "players.json").read_text())
    cases = dict(gallery(args.gallery))
    for case in cases.values():
        verify(case)
    players = {name: Player(**configs[name]) for name in args.players}
    if any(p.mode == "phone" for p in players.values()):
        raise ValueError("this gym runner currently supports native players only")
    os.environ["WALT_RAYON_THREADS"] = "1"
    items = [dict(id=case_id + "--" + name, scenario=case_id, player=name)
             for case_id in cases for name in players]
    manifest = dict(schema="gym-pupils-v1", engine=file_hash(BINARY), runner=file_hash(__file__),
                    player=file_hash(HERE / "player.py"), rules=file_hash(HERE / "rules.py"),
                    runtime=file_hash(HERE / "runtime.py"), matchup=file_hash(HERE / "matchup.py"),
                    rayon_threads_per_process=1,
                    catalog=digest(cases), players={name: configs[name] for name in players})

    def job(item):
        case = cases[item["scenario"]]
        response = decide(pupil_request(case["request"]), **players[item["player"]].kwargs())
        return {**item, "response": response, "grade": grade(case["key"], response["choice"])}

    with run_lock(args.output):
        pin(args.output, manifest)
        bounded(items, job, args.output, args.workers, args.seconds, 16)


def tile(t):
    return "-".join(map(str, TILES[t]))


def report(args):
    cases = list(gallery(args.gallery))
    print("| Exercise | Role | Trick | Worlds | Query targets | Preferred | Comparison | Exact success | Gap |")
    print("|---|---|---:|---:|---|---|---|---|---|")
    for name, case in cases:
        key, pair, req = case["key"], case["pair"], case["request"]
        masses = {a["tile"]: a["success_mass"] for a in key["actions"]}
        role = "make 30" if req["seat"] % 2 == req["bidder"] % 2 else "set 30"
        preferred, comparison = pair["preferred"], pair["comparison"]
        print(f"| {name} | {role} | {key['trick']} | {key['worlds']} | {', '.join(tile(t) for t in case.get('target_actions', key['offers']))} | {tile(preferred)} | {tile(comparison)} | {masses[preferred]}/{key['worlds']} vs {masses[comparison]}/{key['worlds']} | {Fraction(pair['gap_mass'], key['worlds'])} |")
    if args.results:
        manifest = json.loads((args.results / "manifest.json").read_text())
        if manifest["catalog"] != digest(dict(cases)):
            raise ValueError("results belong to a different scenario catalog")
        rows = [json.loads(p.read_text()) for p in sorted((args.results / "items").glob("*.json"))]
        by_name = dict(cases)
        for row in rows:
            if (row["player"] not in manifest["players"]
                    or row["id"] != row["scenario"] + "--" + row["player"]
                    or row["grade"] != grade(by_name[row["scenario"]]["key"], row["response"]["choice"])):
                raise ValueError("saved pupil grade does not match the answer key")
        print("\n| Player | Exercises | Optimal choices | Mean root regret | Mean seconds |")
        print("|---|---:|---:|---:|---:|")
        for name in sorted({r["player"] for r in rows}):
            chosen = [r for r in rows if r["player"] == name]
            regret = sum(Fraction(r["grade"]["regret"]) for r in chosen) / len(chosen)
            elapsed = sum(r["response"]["elapsed_us"] for r in chosen) / len(chosen) / 1e6
            print(f"| {name} | {len(chosen)} | {sum(r['grade']['optimal'] for r in chosen)} | {float(regret):.4%} | {elapsed:.3f} |")


def show(args):
    name, case = next((n, c) for n, c in gallery(args.gallery) if args.id in n)
    req, key, pair = case["request"], case["key"], case["pair"]
    print(name, "\nSeat", req["seat"], "partner", req["seat"] ^ 2, "bidder", req["bidder"], "declaration", req["decl"])
    print("Own remaining:", " ".join(tile(t) for t in key["remaining"]), "banked:", key["banked"])
    print("Public history:", " ".join(f"{s}:{tile(t)}" for s, t in zip(req["plays"][::2], req["plays"][1::2])))
    for action in key["actions"]:
        print(f"  {tile(action['tile'])}: {action['success_mass']}/{key['worlds']} success", "optimal" if action["tile"] in key["best"] else "")
    if "outcome" in case:
        profile = outcome_profile(key)
        print("Optimal plays:", ", ".join(tile(t) for t in profile["optimal"]),
              "of", profile["legal_count"], "legal; probability spread", profile["spread"],
              "; smallest mistake", profile["nearest_mistake"])
        print("Same-world pair outcomes:", canonical(paired_outcomes(key, pair)))
    good = next(a for a in key["actions"] if a["tile"] == pair["preferred"])
    bad = next(a for a in key["actions"] if a["tile"] == pair["comparison"])
    bad_by_world = {world_key(t["hands"]): t for t in bad["traces"]}
    witness = next((t, bad_by_world[world_key(t["hands"])]) for t in good["traces"]
                   if t["success"] and not bad_by_world[world_key(t["hands"])]["success"])
    print("Examiner-only same-world witness (not shown to pupil):")
    for s, hand in enumerate(witness[0]["hands"]):
        print(f"  seat {s}:", " ".join(tile(t) for t in hand))
    for label, trace in zip(("preferred", "comparison"), witness):
        print(f"  {label}:", " ".join(f"{s}:{tile(t)}" for s, t in zip(trace["plays"][::2], trace["plays"][1::2])),
              "final", trace["banked"], "partner count", trace["partner_count"])


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)
    from gym_spec import add_parser, generate
    add_parser(sub)
    m = sub.add_parser("mine")
    m.add_argument("--source", type=Path, default=DEFAULT_SOURCE)
    m.add_argument("--output", type=Path, required=True)
    m.add_argument("--limit", type=int, default=60)
    m.add_argument("--max-worlds", type=int, default=400)
    m.add_argument("--partner-worlds", type=int, default=40)
    m.add_argument("--case-seconds", type=float, default=45)
    d = sub.add_parser("discover", help="Sweep legal coordinates using a supplied Scheme expression")
    d.add_argument("--query", type=Path, required=True)
    d.add_argument("--source", type=Path, nargs="+", default=[DEFAULT_SOURCE.parent])
    d.add_argument("--output", type=Path, required=True)
    d.add_argument("--limit", type=int, default=5000)
    d.add_argument("--max-worlds", type=int, default=400)
    d.add_argument("--partner-worlds", type=int, default=40)
    d.add_argument("--case-seconds", type=float, default=45)
    d.add_argument("--min-presence", default="1", help="Exact belief-presence threshold, e.g. 1 or 1/4")
    d.add_argument("--min-trick", type=int, default=5)
    d.add_argument("--max-trick", type=int, default=6)
    inventory_parser = sub.add_parser("inventory")
    inventory_parser.add_argument("--source", type=Path, required=True)
    s = sub.add_parser("select")
    s.add_argument("--source", type=Path, required=True)
    s.add_argument("--output", type=Path, default=DEFAULT_GALLERY)
    s.add_argument("--each", type=int, default=3)
    s.add_argument("--all", action="store_true", help="Publish every strict coordinate once, using its strongest pair")
    s.add_argument("--criterion", choices=("query", "outcome"), default="query",
                   help="Query target contrast, or any strict success-probability difference")
    s.add_argument("--side", choices=("both", "declaring", "defending"), default="both")
    r = sub.add_parser("run")
    r.add_argument("--output", type=Path, required=True)
    r.add_argument("--players", nargs="+", default=["l1-default", "l2-partner-default", "l2-partner-voids"])
    for p in (m, d, r):
        p.add_argument("--workers", type=int, default=10)
        p.add_argument("--seconds", type=float, default=240)
    v = sub.add_parser("verify")
    p = sub.add_parser("report")
    p.add_argument("--results", type=Path)
    h = sub.add_parser("show")
    h.add_argument("id")
    for p in (r, v, p, h):
        p.add_argument("--gallery", type=Path, default=DEFAULT_GALLERY)
    args = parser.parse_args()
    if args.command in ("mine", "discover", "run"):
        if not 1 <= args.workers <= 10 or not 20 <= args.seconds <= 270:
            parser.error("workers 1..10, seconds 20..270; run inside the session watchdog")
    if args.command in ("mine", "discover") and not (4 <= args.case_seconds < args.seconds and 1 <= args.max_worlds <= 10000 and 1 <= args.partner_worlds <= 640 and args.limit > 0):
        parser.error("invalid mining bounds")
    if args.command == "discover" and not (14 <= args.case_seconds and 5 <= args.min_trick <= args.max_trick <= 6 and 0 < Fraction(args.min_presence) <= 1):
        parser.error("discovery needs case-seconds >=14, tricks 5..6, and presence in (0,1]")
    if args.command == "select" and args.each < 1:
        parser.error("each must be positive")
    if args.command == "generate":
        generate(args)
    elif args.command == "verify":
        for name, case in gallery(args.gallery):
            print(name, verify(case))
    else:
        globals()[args.command](args)


if __name__ == "__main__":
    main()
