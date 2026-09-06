#!/usr/bin/env python3
"""Resumable make/set campaign. Run advance beneath the packet watchdog.

One seed at a time; independent lineups within it run concurrently. A durable
checkpoint after each decision keeps interruption loss inside the current seed.
"""

import argparse
import fcntl
import hashlib
import json
import os
import random
import signal
import subprocess
import sys
import time
import uuid
from collections import Counter
from dataclasses import asdict
from datetime import datetime, timezone
from fractions import Fraction
from pathlib import Path

from matchup import Player
from matchup import pair as match_pair
from player import BINARY, HERE, decide
from rules import TILES, information_state, replay_record
from runtime import DecisionSession

if not __debug__:
    raise RuntimeError("campaign validation requires Python assertions enabled")

ARMS = ("phone", "declaring", "defending")
PUBLIC_SEED = 420600
KNOWN = {420601, 420602, 420603}
SOURCE_FILES = [
    "campaign.py",
    "player.py",
    "rules.py",
    "phone.mjs",
    "runtime.py",
    "matchup.py",
    "pool.py",
    "reference/phone/walt.ts",
    "reference/phone/walt.wasm",
]


def arms_for(spec):
    return ("declaring", "defending") if "players" in spec else ARMS


def players_for(spec, arm, bidder):
    if "players" in spec:
        a, b = (Player(**spec["players"][side]) for side in ("a", "b"))
        declaring, defending = (a, b) if arm == "declaring" else (b, a)
        return [declaring if seat % 2 == bidder % 2 else defending for seat in range(4)]
    return [
        Player(
            name=mode,
            mode=mode,
            n=spec["n"],
            n0=spec["n0"],
            n1=spec["n1"],
            budget_ms=spec["budget_ms"],
            inner_belief="voidless"
            if mode == "phone"
            else spec.get("inner_belief", "voidless"),
        )
        for mode in modes_for(arm, bidder, spec.get("candidate_mode", "partner"))
    ]


def validate_checkpoint(snap, spec, f, seed, arm):
    assert (snap["campaign"], snap["seed"], snap["arm"]) == (spec["id"], seed, arm), (
        "checkpoint identity"
    )
    players = players_for(spec, arm, f["bidder"])
    decisions = snap["decisions"]
    assert len(decisions) <= 28, "checkpoint too long"
    record = []
    for decision in decisions:
        points, lead, _, trick = replay_record(
            f["hands"], record, f["decl"], f["bidder"]
        )
        seat = (lead + len(trick)) % 4
        response = decision["response"]
        assert decision["seat"] == seat, "checkpoint actor"
        request = {
            "decl": f["decl"],
            "bid": 30,
            "bidder": f["bidder"],
            "seat": seat,
            "hand": f["hands"][seat],
            "plays": record,
            "seed": PUBLIC_SEED,
        }
        state = information_state(request)
        assert all(
            response[k] == state[k] for k in ("legal", "leader", "points", "trick")
        ), "checkpoint state"
        assert response["choice"] in state["legal"], "checkpoint illegal move"
        assert all(response[k] == v for k, v in players[seat].kwargs().items()), (
            "checkpoint player configuration"
        )
        assert not response["over_budget"], "checkpoint over budget"
        record.extend((seat, response["choice"]))
    # Includes the final move (the per-prefix checks above stop before it).
    replay_record(f["hands"], record, f["decl"], f["bidder"])
    return decisions


def initialize_match(
    path,
    a,
    b,
    start,
    count=50,
    panel="random",
    worlds_per_hand=10,
    threads=6,
    warm=True,
):
    path = Path(path).resolve()
    a, b = Player(**a), Player(**b)
    if a.name == b.name:
        raise ValueError("players need distinct names")
    if (path / "manifest.json").exists():
        raise ValueError("campaign already exists")
    if (
        type(start) is not int
        or start < 0
        or not 1 <= count <= 1000
        or not 1 <= threads <= 18
    ):
        raise ValueError("invalid panel size or concurrency")
    if panel not in ("random", "worlds") or not 1 <= worlds_per_hand <= 1000:
        raise ValueError("invalid panel")
    if panel == "worlds" and count % worlds_per_hand:
        raise ValueError("a fixed-hand panel must contain complete groups")
    panel_spec = {
        "start": start,
        "count": count,
        "panel": panel,
        "worlds_per_hand": worlds_per_hand,
        "bid": 30,
        "public_seed": PUBLIC_SEED,
        "generator": "python-random-fisher-yates-v1-longest-trump",
    }
    spec = {
        **panel_spec,
        "schema": "partnership-head-to-head-v2",
        "players": {"a": asdict(a), "b": asdict(b)},
        "panel_id": digest(panel_spec),
        "threads": threads,
        "workers": 2,
        "warm_workers": bool(warm),
        "seed_cap_seconds": 270,
        "known_development_seeds": [],
        "identities": identities(),
        "created_utc": now(),
        "ranking": "mirrored make/set: make(A)-make(B); points never break ties",
        "early_rule": "technical failures and >5% fallbacks per player after 20 nonforced decisions; no outcome-based stopping",
    }
    spec["id"] = digest(spec)
    atomic(path / "manifest.json", spec)
    status(path)
    return spec


def now():
    return datetime.now(timezone.utc).isoformat()


def digest(value):
    return hashlib.sha256(
        json.dumps(value, sort_keys=True, separators=(",", ":")).encode()
    ).hexdigest()


def atomic(path, value):
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_name(path.name + "." + str(os.getpid()) + ".tmp")
    with temporary.open("w") as f:
        if isinstance(value, str):
            f.write(value)
        else:
            json.dump(value, f, sort_keys=True, separators=(",", ":"))
            f.write("\n")
        f.flush()
        os.fsync(f.fileno())
    os.replace(temporary, path)
    directory = os.open(str(path.parent), os.O_RDONLY)
    try:
        os.fsync(directory)
    finally:
        os.close(directory)


def read(path, default=None):
    try:
        return json.loads(Path(path).read_text())
    except FileNotFoundError:
        return default


def identities():
    return {
        **{
            p: hashlib.sha256((HERE / p).read_bytes()).hexdigest() for p in SOURCE_FILES
        },
        "native": hashlib.sha256(BINARY.read_bytes()).hexdigest(),
    }


def initialize(
    path,
    start=420600,
    count=100,
    panel="random",
    threads=4,
    workers=3,
    inner_belief="voidless",
    candidate_mode="partner",
):
    path = Path(path).resolve()
    if (path / "manifest.json").exists():
        raise ValueError("campaign already exists; use advance to resume")
    if not 1 <= threads <= 18 or not 1 <= workers <= 3 or not 1 <= count <= 1000:
        raise ValueError("invalid threads, workers, or count")
    if inner_belief not in ("voidless", "voids-counted"):
        raise ValueError("unknown inner belief strategy")
    if candidate_mode not in ("baseline", "partner", "all-l1"):
        raise ValueError("unknown native candidate mode")
    spec = {
        "candidate_mode": candidate_mode,
        "inner_belief": inner_belief,
        "schema": "partnership-campaign-v1",
        "start": start,
        "count": count,
        "panel": panel,
        "worlds_per_hand": 10,
        "bid": 30,
        "public_seed": PUBLIC_SEED,
        "n": 40,
        "n0": 8,
        "n1": 2,
        "budget_ms": 14000,
        "threads": threads,
        "workers": workers,
        "seed_cap_seconds": 270,
        "known_development_seeds": sorted(KNOWN) if panel == "random" else [],
        "identities": identities(),
        "created_utc": now(),
        "ranking": "paired make/set only; points never break ties",
        "early_rule": "fresh random seeds: pause at n>=10 and downside e-value>=20; pause uninformative all-set/no-flip after n>=20; technical errors pause immediately; fallback rate >5% after 20 nonforced decisions pauses",
    }
    spec["id"] = digest(spec)
    atomic(path / "manifest.json", spec)
    status(path)
    return spec


def load(path, verify=True):
    spec = read(Path(path) / "manifest.json")
    if not spec:
        raise ValueError("missing campaign manifest")
    if digest({k: v for k, v in spec.items() if k != "id"}) != spec["id"]:
        raise ValueError("manifest content does not match its identity")
    if verify and spec["identities"] != identities():
        raise ValueError(
            "source/binary identity changed; refuse silent resume under a different player"
        )
    return spec


def trump_rank(hand, trump):
    called = [t for t in hand if trump in TILES[t]]
    return (
        len(called),
        int((trump, trump) in [TILES[t] for t in called]),
        sum(TILES[t][0] == TILES[t][1] for t in hand if t not in called),
        -trump,
    )


def fixture(spec, seed):
    if spec["panel"] == "random":
        tiles = list(range(28))
        random.Random(seed).shuffle(tiles)
        hands = [sorted(tiles[7 * s : 7 * s + 7]) for s in range(4)]
        bidder, decl = max(
            ((s, d) for s in range(4) for d in range(7)),
            key=lambda sd: (*trump_rank(hands[sd[0]], sd[1])[:3], -sd[0], -sd[1]),
        )
        group = None
    else:
        group = (seed - spec["start"]) // spec["worlds_per_hand"]
        bidder = group % 4
        tiles = list(range(28))
        random.Random(spec["start"] + group).shuffle(tiles)
        own = sorted(tiles[:7])
        unseen = sorted(set(range(28)) - set(own))
        random.Random(seed ^ 0x574F524C44).shuffle(unseen)
        hands = [[] for _ in range(4)]
        hands[bidder] = own
        for i, seat in enumerate(s for s in range(4) if s != bidder):
            hands[seat] = sorted(unseen[7 * i : 7 * i + 7])
        decl = max(range(7), key=lambda d: trump_rank(own, d))
    return {"hands": hands, "bidder": bidder, "decl": decl, "bid": 30, "group": group}


def modes_for(arm, bidder, candidate_mode="partner"):
    if arm == "phone":
        return ["phone"] * 4
    parity = bidder % 2 if arm == "declaring" else 1 - bidder % 2
    return [candidate_mode if seat % 2 == parity else "phone" for seat in range(4)]


def record_of(decisions):
    return [v for d in decisions for v in (d["seat"], d["response"]["choice"])]


def arm_run(path, seed, arm, yield_path):
    if load(path).get("warm_workers", False):
        with DecisionSession() as session:
            return _arm_run(path, seed, arm, yield_path, session)
    return _arm_run(path, seed, arm, yield_path, None)


def _arm_run(path, seed, arm, yield_path, session):
    path = Path(path)
    spec = load(path)
    os.environ["WALT_RAYON_THREADS"] = str(spec["threads"])
    f = fixture(spec, seed)
    players = players_for(spec, arm, f["bidder"])
    location = path / "seeds" / str(seed) / arm
    checkpoint = location / "checkpoint.json"
    snap = read(
        checkpoint, {"campaign": spec["id"], "seed": seed, "arm": arm, "decisions": []}
    )
    decisions = validate_checkpoint(snap, spec, f, seed, arm)
    while len(decisions) < 28:
        if (path / "STOP.json").exists() or Path(yield_path).exists():
            return 75
        record = record_of(decisions)
        points, lead, remaining, trick = replay_record(
            f["hands"], record, f["decl"], f["bidder"]
        )
        seat = (lead + len(trick)) % 4
        req = {
            "decl": f["decl"],
            "bid": 30,
            "bidder": f["bidder"],
            "seat": seat,
            "hand": f["hands"][seat],
            "plays": record,
            "seed": PUBLIC_SEED,
        }
        response = decide(req, **players[seat].kwargs(), session=session)
        assert response["points"] == points and response["leader"] == lead
        assert response["choice"] in information_state(req)["legal"]
        assert not response["over_budget"], "decision deadline overrun"
        assert response["phases"][0]["status"] == "completed", (
            "independent rules disagreement"
        )
        for phase in response["phases"]:
            assert not phase["status"].startswith("rejected:"), (
                "invalid worker response"
            )
            if phase["status"].startswith("worker-error:"):
                assert "Deadline" in phase["status"], "unexpected worker failure"
        decisions.append({"seat": seat, "response": response})
        atomic(checkpoint, snap)
    result = result_from_decisions(spec, seed, arm, decisions, now())
    atomic(location / "result.json", result)
    return 0


def result_from_decisions(spec, seed, arm, decisions, finished_utc):
    assert len(decisions) == 28, "only complete hands produce results"
    f = fixture(spec, seed)
    players = players_for(spec, arm, f["bidder"])
    modes = [p.mode for p in players]

    record = record_of(decisions)
    points, _, remaining, _ = replay_record(f["hands"], record, f["decl"], f["bidder"])
    assert sum(points) == 42 and not any(remaining)
    trick_us = [
        sum(d["response"]["elapsed_us"] for d in decisions[t : t + 4])
        for t in range(0, 28, 4)
    ]
    assert max(trick_us) < 60_000_000, "four-play trick budget exceeded"
    counts = Counter(d["response"]["route"] for d in decisions)
    result = {
        "campaign": spec["id"],
        "seed": seed,
        "arm": arm,
        "fixture": f,
        "modes": modes,
        "points": points,
        "made": points[f["bidder"] % 2] >= 30,
        "overbid_points_diagnostic": max(0, points[f["bidder"] % 2] - 30),
        "decisions": 28,
        "routes": dict(counts),
        "trick_us": trick_us,
        "elapsed_us": sum(d["response"]["elapsed_us"] for d in decisions),
        "max_decision_us": max(d["response"]["elapsed_us"] for d in decisions),
        "opening_choice": decisions[0]["response"]["choice"],
        "opening_route": decisions[0]["response"]["route"],
        "fallbacks": sum(v for k, v in counts.items() if "fallback" in k),
        "finished_utc": finished_utc,
    }
    if "players" in spec:
        result["players"] = [p.name for p in players]
        result["player_counts"] = {}
        for side in ("a", "b"):
            name = spec["players"][side]["name"]
            ds = [d["response"] for d in decisions if players[d["seat"]].name == name]
            result["player_counts"][name] = {
                "moves": len(ds),
                "nonforced": sum(len(d["legal"]) > 1 for d in ds),
                "fallbacks": sum("fallback" in d["route"] for d in ds),
                "elapsed_us": sum(d["elapsed_us"] for d in ds),
            }
    return result


def paired(phone, declaring, defending):
    # +1 means a candidate-team win that the corresponding phone team missed.
    attack = int(declaring["made"]) - int(phone["made"])
    defense = int(phone["made"]) - int(defending["made"])
    return {
        "declaring_delta": attack,
        "defending_delta": defense,
        "wins": int(attack > 0) + int(defense > 0),
        "losses": int(attack < 0) + int(defense < 0),
        "ties": int(attack == 0) + int(defense == 0),
        "seed_delta": attack + defense,
    }


def commit_seed(path, spec, seed):
    arms = {
        a: read(path / "seeds" / str(seed) / a / "result.json") for a in arms_for(spec)
    }
    if not all(arms.values()):
        return None
    for a, r in arms.items():
        decisions = validate_checkpoint(
            read(path / "seeds" / str(seed) / a / "checkpoint.json"),
            spec,
            fixture(spec, seed),
            seed,
            a,
        )
        assert r == result_from_decisions(
            spec, seed, a, decisions, r["finished_utc"]
        ), "result disagrees with saved decisions"
        assert (r["campaign"], r["seed"], r["arm"]) == (spec["id"], seed, a)
        assert r["fixture"] == fixture(spec, seed)
    result = {
        "campaign": spec["id"],
        "seed": seed,
        "arms": arms,
        "fresh": seed not in spec["known_development_seeds"],
        "paired": match_pair(arms["declaring"]["made"], arms["defending"]["made"])
        if "players" in spec
        else paired(*(arms[a] for a in ARMS)),
        "committed_utc": now(),
    }
    if spec["panel"] == "worlds":
        for old in complete_results(path, spec):
            if (
                next(iter(old["arms"].values()))["fixture"]["group"]
                != next(iter(arms.values()))["fixture"]["group"]
            ):
                continue
            for a in arms_for(spec):
                if (
                    "fallback" not in old["arms"][a]["opening_route"]
                    and "fallback" not in arms[a]["opening_route"]
                ):
                    assert (
                        old["arms"][a]["opening_choice"] == arms[a]["opening_choice"]
                    ), "same-information opening changed across hidden worlds"
    atomic(path / "results" / (str(seed) + ".json"), result)
    return result


def complete_results(path, spec):
    out = []
    for seed in range(spec["start"], spec["start"] + spec["count"]):
        r = read(path / "results" / (str(seed) + ".json"))
        if r is not None:
            assert r["campaign"] == spec["id"]
            out.append(r)
    return out


def summarize(path, spec):
    results = complete_results(path, spec)
    fresh = [r for r in results if r["fresh"]]

    def tally(rows):
        return {
            k: sum(r["paired"][k] for r in rows) for k in ("wins", "losses", "ties")
        }

    wf, lf = (
        sum(r["paired"]["seed_delta"] > 0 for r in fresh),
        sum(r["paired"]["seed_delta"] < 0 for r in fresh),
    )
    e = Fraction(3**lf, 2 ** (lf + wf))
    arms = [a for r in results for a in r["arms"].values()]
    nonforced = sum(a["decisions"] - a["routes"].get("forced", 0) for a in arms)
    fallback = sum(a["fallbacks"] for a in arms)
    stop = read(path / "STOP.json")
    summary = {
        "campaign": spec["id"],
        "panel": spec["panel"],
        "completed_seeds": len(results),
        "target_seeds": spec["count"],
        "fresh_seeds": len(fresh),
        "completed_games": len(arms),
        "all_paired": tally(results),
        "fresh_paired": tally(fresh),
        "downside_e_value": {
            "numerator": str(e.numerator),
            "denominator": str(e.denominator),
        },
        "fresh_reference_makes": sum(
            r["arms"].get("phone", r["arms"]["defending"])["made"] for r in fresh
        ),
        "fallbacks": fallback,
        "nonforced_decisions": nonforced,
        "max_trick_us": max((max(a["trick_us"]) for a in arms), default=0),
        "max_decision_us": max((a["max_decision_us"] for a in arms), default=0),
        "stop": stop,
        "updated_utc": now(),
    }
    if "players" in spec:
        summary.pop("downside_e_value")
        summary.pop("fresh_reference_makes")
        totals = {}
        for arm in arms:
            for name, counts in arm["player_counts"].items():
                totals.setdefault(name, Counter()).update(counts)
        summary["player_counts"] = {k: dict(v) for k, v in totals.items()}
    return summary


def early_reason(s, spec):
    if "players" in spec:
        for name, counts in s["player_counts"].items():
            if (
                counts["nonforced"] >= 20
                and counts["fallbacks"] * 20 > counts["nonforced"]
            ):
                return "fallback-rate-exceeds-five-percent:" + name
        return None
    if (
        s["nonforced_decisions"] >= 20
        and s["fallbacks"] * 20 > s["nonforced_decisions"]
    ):
        return "fallback-rate-exceeds-five-percent"
    if spec["panel"] == "random":
        e = s["downside_e_value"]
        if s["fresh_seeds"] >= 10 and int(e["numerator"]) >= 20 * int(e["denominator"]):
            return "downside-e-value-at-least-20"
        if (
            s["fresh_seeds"] >= 20
            and s["fresh_reference_makes"] == 0
            and s["fresh_paired"]["wins"] + s["fresh_paired"]["losses"] == 0
        ):
            return "all-set-panel-uninformative-after-20-fresh-seeds"
    return None


def status(path, active=None):
    path = Path(path)
    spec = load(path, verify=False)
    s = summarize(path, spec)
    if active is not None:
        s["active_seed"] = active
        s["active_plies"] = {
            a: len(
                read(
                    path / "seeds" / str(active) / a / "checkpoint.json",
                    {"decisions": []},
                )["decisions"]
            )
            for a in ARMS
        }
    else:
        pending = [
            seed
            for seed in range(spec["start"], spec["start"] + spec["count"])
            if not (path / "results" / (str(seed) + ".json")).exists()
        ]
        if pending:
            s["next_seed"] = pending[0]
    atomic(path / "status.json", s)
    pair = s["fresh_paired"]
    if "players" in spec:
        atomic(path / "status.json", s)
        atomic(
            path / "STATUS.md",
            f"# {spec['players']['a']['name']} versus {spec['players']['b']['name']}\n\n"
            f"Completed: {s['completed_seeds']}/{spec['count']} mirrored deals. "
            f"A pair wins/losses/ties: {pair['wins']}/{pair['losses']}/{pair['ties']}. "
            f"Fallbacks: {s['fallbacks']}/{s['nonforced_decisions']} nonforced decisions.\n",
        )
        return s
    state = (
        "STOPPED: " + s["stop"]["reason"]
        if s["stop"]
        else "COMPLETE"
        if s["completed_seeds"] == s["target_seeds"]
        else "Resumable"
    )
    body = (
        f"# Texas 42 — {spec['panel']} campaign\n\n{state}\n\n"
        f"**{s['completed_seeds']} / {s['target_seeds']} seeds** · {s['completed_games']} completed games · {s['fresh_seeds']} fresh seeds\n\n"
        f"Fresh paired make/set: **{pair['wins']} wins / {pair['losses']} losses / {pair['ties']} ties**. Points never break ties. Bid always 30.\n\n"
        f"Fresh phone references made: {s['fresh_reference_makes']}. Fallbacks: {s['fallbacks']} / {s['nonforced_decisions']} nonforced decisions.\n\n"
        f"Maximum trick: {s['max_trick_us'] / 1e6:.3f}s; maximum decision: {s['max_decision_us'] / 1e6:.3f}s.\n\n"
        f"Workers: {spec['workers']} games within one seed, {spec['threads']} native threads per game.\n\n"
    )
    if active is not None:
        body += (
            f"Active seed **{active}**: "
            + ", ".join(f"{a} {v}/28" for a, v in s["active_plies"].items())
            + ".\n\n"
        )
    body += f"Updated {s['updated_utc']}. Full records: results/ and seeds/. Stop: `campaign.py stop PATH`; resume: `campaign.py resume PATH`, then a capped `advance`.\n"
    atomic(path / "STATUS.md", body)
    return s


def stop(path, reason):
    atomic(Path(path) / "STOP.json", {"reason": reason, "utc": now()})


def advance(path, seconds=260, max_seeds=100):
    if not 1 <= seconds <= 270:
        raise ValueError(
            "advance must be 1..270 seconds; launch beneath 295-second watchdog"
        )
    path = Path(path).resolve()
    spec = load(path)
    if "players" in spec:
        raise ValueError("two-player matches run through the shared pool")
    lock = (path / "runner.lock").open("a+")
    try:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
    except BlockingIOError:
        raise ValueError("another runner already owns this campaign")
    token = str(uuid.uuid4())
    yield_path = path / "control" / (token + ".yield")
    began = time.monotonic()
    session = {
        "id": token,
        "started_utc": now(),
        "command": sys.argv,
        "seconds": seconds,
        "status": "running",
    }
    session_path = path / "sessions" / (token + ".json")
    atomic(session_path, session)
    for sig in (signal.SIGTERM, signal.SIGINT):
        signal.signal(sig, lambda *_: stop(path, "user-interrupt"))
    completed_now = 0
    processes = {}
    files = []
    try:
        for seed in range(spec["start"], spec["start"] + spec["count"]):
            if (path / "results" / (str(seed) + ".json")).exists():
                continue
            if (
                (path / "STOP.json").exists()
                or time.monotonic() - began >= seconds
                or completed_now >= max_seeds
            ):
                break
            if commit_seed(path, spec, seed):
                completed_now += 1
                reason = early_reason(status(path), spec)
                if reason:
                    stop(path, reason)
                    break
                continue
            pending = [
                a
                for a in ARMS
                if not (path / "seeds" / str(seed) / a / "result.json").exists()
            ]
            seed_start = time.monotonic()
            last_status = 0
            while pending or processes:
                elapsed = time.monotonic() - began
                if (
                    elapsed >= seconds
                    or time.monotonic() - seed_start >= spec["seed_cap_seconds"]
                ) and not yield_path.exists():
                    atomic(yield_path, "pause this slice\n")
                paused = (path / "STOP.json").exists() or yield_path.exists()
                while pending and len(processes) < spec["workers"] and not paused:
                    arm = pending.pop(0)
                    folder = path / "seeds" / str(seed) / arm
                    folder.mkdir(parents=True, exist_ok=True)
                    log = (folder / (token + ".log")).open("w")
                    files.append(log)
                    cmd = [
                        sys.executable,
                        str(Path(__file__).resolve()),
                        "arm",
                        str(path),
                        "--seed",
                        str(seed),
                        "--arm",
                        arm,
                        "--yield-path",
                        str(yield_path),
                    ]
                    processes[arm] = subprocess.Popen(
                        cmd,
                        stdout=log,
                        stderr=subprocess.STDOUT,
                        env={**os.environ, "WALT_RAYON_THREADS": str(spec["threads"])},
                    )
                for arm, proc in list(processes.items()):
                    result = proc.poll()
                    if result is None:
                        continue
                    del processes[arm]
                    if result not in (0, 75) and not paused:
                        stop(
                            path,
                            "worker-failure:"
                            + str(seed)
                            + ":"
                            + arm
                            + ":"
                            + str(result),
                        )
                if paused and not processes:
                    break
                if time.monotonic() - last_status > 2:
                    status(path, seed)
                    last_status = time.monotonic()
                time.sleep(0.10)
            committed = commit_seed(path, spec, seed)
            if committed:
                completed_now += 1
                summary = status(path)
                print(
                    json.dumps(
                        {
                            "seed": seed,
                            "paired": committed["paired"],
                            "completed": summary["completed_seeds"],
                            "fresh": summary["fresh_paired"],
                        }
                    ),
                    flush=True,
                )
                reason = early_reason(summary, spec)
                if reason:
                    stop(path, reason)
            if yield_path.exists() or (path / "STOP.json").exists():
                break
        session["status"] = (
            "paused" if (path / "STOP.json").exists() else "slice-complete"
        )
    except BaseException as e:
        stop(path, "runner-error:" + str(e))
        atomic(yield_path, "abort\n")
        raise
    finally:
        for proc in processes.values():
            try:
                proc.wait(timeout=15)
            except subprocess.TimeoutExpired:
                proc.kill()
                proc.wait()
        for file in files:
            file.close()
        session["elapsed_seconds"] = time.monotonic() - began
        session["finished_utc"] = now()
        session["committed_this_slice"] = completed_now
        atomic(session_path, session)
        s = status(path)
        print(json.dumps(s), flush=True)
        lock.close()
    return s


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument(
        "command", choices=("init", "advance", "status", "stop", "resume", "arm")
    )
    p.add_argument("path", type=Path)
    p.add_argument("--start", type=int, default=420600)
    p.add_argument("--count", type=int, default=100)
    p.add_argument("--panel", choices=("random", "worlds"), default="random")
    p.add_argument(
        "--candidate-mode", choices=("baseline", "partner", "all-l1"), default="partner"
    )
    p.add_argument(
        "--inner-belief", choices=("voidless", "voids-counted"), default="voidless"
    )
    p.add_argument("--threads", type=int, default=4)
    p.add_argument("--workers", type=int, default=3)
    p.add_argument("--seconds", type=int, default=260)
    p.add_argument("--max-seeds", type=int, default=100)
    p.add_argument("--reason", default="user-request")
    p.add_argument("--seed", type=int)
    p.add_argument("--arm", choices=ARMS)
    p.add_argument("--yield-path")
    args = p.parse_args()
    if args.command == "init":
        initialize(
            args.path,
            args.start,
            args.count,
            args.panel,
            args.threads,
            args.workers,
            args.inner_belief,
            args.candidate_mode,
        )
    elif args.command == "advance":
        advance(args.path, args.seconds, args.max_seeds)
    elif args.command == "arm":
        sys.exit(arm_run(args.path, args.seed, args.arm, args.yield_path))
    elif args.command == "stop":
        stop(args.path, args.reason)
        print(json.dumps(status(args.path)))
    elif args.command == "resume":
        load(args.path)
        stopped = args.path / "STOP.json"
        if stopped.exists():
            atomic(
                args.path / "control" / (str(uuid.uuid4()) + "-resumed.json"),
                read(stopped),
            )
            stopped.unlink()
        print(json.dumps(status(args.path)))
    else:
        print(json.dumps(status(args.path)))


if __name__ == "__main__":
    main()
