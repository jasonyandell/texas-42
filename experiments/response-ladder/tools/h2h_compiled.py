#!/usr/bin/env python3
"""Bounded, resumable comparison for a compiled C0/C1 player.

Defaults to DEVELOPMENT; confirmation requires a predeclared pinned protocol.

The candidate and the maintained CPU partnership play the same mirrored
fixtures.  Complete games are independently replayed before publication;
failed arms are recorded and are never silently retried.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import os
from pathlib import Path
import platform
import random
import selectors
import shlex
import shutil
import statistics
import subprocess
import sys
import tempfile
import time
from collections import Counter
from typing import Any


HERE = Path(__file__).resolve().parents[1]
CPU_ROOT_DEFAULT = Path("/Users/jason/code/texas-42-partnership-launch")
CPU_MS = 14000
CHECKPOINT_RESERVE_S = 16.0
DECLARATIONS = [0, 1, 2, 3, 4, 5, 6, 7, 9]


def canonical(value: Any) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def document(value: Any) -> str:
    return json.dumps(value, sort_keys=True, indent=2, ensure_ascii=True) + "\n"


def digest(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def input_identity(path: Path) -> dict[str, Any]:
    path = path.expanduser().resolve()
    if path.is_file():
        return {"path": str(path), "kind": "file", "bytes": path.stat().st_size, "sha256": digest(path)}
    if path.is_dir():
        files = []
        for child in sorted(p for p in path.rglob("*") if p.is_file()):
            files.append({"path": str(child.relative_to(path)), "bytes": child.stat().st_size, "sha256": digest(child)})
        return {"path": str(path), "kind": "directory", "files": files,
                "sha256": hashlib.sha256(canonical(files).encode()).hexdigest()}
    raise SystemExit(f"candidate input does not exist: {path}")


def executable_identity(command: list[str]) -> dict[str, Any]:
    resolved = shutil.which(command[0]) or command[0]
    path = Path(resolved).expanduser()
    result = {"argv": command, "path": str(path.resolve())}
    if path.is_file():
        result.update({"bytes": path.stat().st_size, "sha256": digest(path)})
    else:
        result.update({"bytes": None, "sha256": None})
    return result


def atomic(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    fd, temporary = tempfile.mkstemp(prefix=f".{path.name}.", dir=str(path.parent))
    try:
        with os.fdopen(fd, "w", encoding="utf-8") as stream:
            stream.write(value if isinstance(value, str) else document(value))
            stream.flush()
            os.fsync(stream.fileno())
        os.replace(temporary, path)
    finally:
        try:
            os.unlink(temporary)
        except FileNotFoundError:
            pass


def append_jsonl(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("a", encoding="utf-8") as stream:
        stream.write(canonical(value) + "\n")
        stream.flush()
        os.fsync(stream.fileno())


def seed(domain: str, panel: str, index: int) -> int:
    data = f"compiled-h2h-v1/{panel}/{domain}/{index}".encode()
    return int.from_bytes(hashlib.sha256(data).digest()[:8], "little")


def quantile(values: list[float], probability: float) -> float | None:
    if not values:
        return None
    return sorted(values)[min(len(values) - 1, max(0, math.ceil(probability * len(values)) - 1))]


def summary_stats(values: list[float]) -> dict[str, Any]:
    return {"count": len(values), "mean_ms": statistics.mean(values) if values else None,
            **{name: quantile(values, probability) for name, probability in
               (("median_ms", .5), ("p90_ms", .9), ("p95_ms", .95), ("p99_ms", .99), ("max_ms", 1))}}


def extract_actor_paths(command: list[str], explicit_c0: str | None, explicit_c1: str | None) -> tuple[list[str], Path, Path]:
    values: dict[str, str] = {}
    for index, token in enumerate(command):
        for name in ("c0", "c1"):
            prefix = f"--{name}="
            if token.startswith(prefix):
                values[name] = token[len(prefix):]
            elif token == f"--{name}" and index + 1 < len(command):
                values[name] = command[index + 1]
    if explicit_c0 is not None:
        if "c0" in values and Path(values["c0"]).expanduser().resolve() != Path(explicit_c0).expanduser().resolve():
            raise SystemExit("explicit C0 path disagrees with --c0 in candidate command")
        values["c0"] = explicit_c0
    if explicit_c1 is not None:
        if "c1" in values and Path(values["c1"]).expanduser().resolve() != Path(explicit_c1).expanduser().resolve():
            raise SystemExit("explicit C1 path disagrees with --c1 in candidate command")
        values["c1"] = explicit_c1
    if "c0" not in values or "c1" not in values:
        raise SystemExit("candidate command must identify both --c0 and --c1 inputs (or pass --actor-c0/--actor-c1)")
    normalized = list(command)
    if not any(token == "--c0" or token.startswith("--c0=") for token in normalized):
        normalized += ["--c0", values["c0"]]
    if not any(token == "--c1" or token.startswith("--c1=") for token in normalized):
        normalized += ["--c1", values["c1"]]
    return normalized, Path(values["c0"]).expanduser().resolve(), Path(values["c1"]).expanduser().resolve()


class CandidateWorker:
    def __init__(self, command: list[str], read_timeout: float):
        self.command = command
        self.read_timeout = read_timeout
        self.process: subprocess.Popen[bytes] | None = None
        self.selector: selectors.BaseSelector | None = None
        self.buffer = bytearray()

    def start(self) -> dict[str, Any]:
        self.process = subprocess.Popen(self.command, stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                        stderr=subprocess.PIPE, bufsize=0, start_new_session=False)
        assert self.process.stdout is not None
        self.selector = selectors.DefaultSelector()
        self.selector.register(self.process.stdout, selectors.EVENT_READ)
        ready = self.read(self.read_timeout)
        if not isinstance(ready, dict) or ready.get("ready") is not True:
            raise RuntimeError(f"candidate worker did not send ready:true: {ready!r}")
        return ready

    def read(self, timeout: float) -> Any:
        if self.process is None or self.process.stdout is None or self.selector is None:
            raise RuntimeError("candidate worker is not running")
        deadline = time.monotonic() + timeout
        while True:
            if b"\n" in self.buffer:
                line, _, rest = self.buffer.partition(b"\n")
                self.buffer = bytearray(rest)
                try:
                    return json.loads(line.decode("utf-8"))
                except (UnicodeDecodeError, json.JSONDecodeError) as exc:
                    raise RuntimeError(f"candidate worker emitted invalid JSON: {exc}") from exc
            remaining = deadline - time.monotonic()
            if remaining <= 0 or not self.selector.select(remaining):
                raise TimeoutError("candidate worker response watchdog")
            chunk = os.read(self.process.stdout.fileno(), 65536)
            if not chunk:
                raise RuntimeError(f"candidate worker exited ({self.process.poll()})")
            self.buffer.extend(chunk)

    def call(self, request: dict[str, Any]) -> dict[str, Any]:
        if self.process is None or self.process.stdin is None:
            raise RuntimeError("candidate worker is not running")
        self.process.stdin.write((canonical(request) + "\n").encode())
        self.process.stdin.flush()
        value = self.read(min(self.read_timeout, request["budget_ms"] / 1000.0 + 2.0))
        if not isinstance(value, dict):
            raise RuntimeError("candidate response is not an object")
        return value

    def close(self) -> None:
        process = self.process
        if process is None:
            return
        try:
            if process.stdin is not None:
                process.stdin.close()
        except OSError:
            pass
        try:
            process.wait(timeout=.5)
        except subprocess.TimeoutExpired:
            process.terminate()
            try:
                process.wait(timeout=.5)
            except subprocess.TimeoutExpired:
                process.kill()
                process.wait(timeout=.5)
        if self.selector is not None:
            self.selector.close()
        for stream in (process.stdout, process.stderr):
            try:
                if stream is not None:
                    stream.close()
            except OSError:
                pass
        self.process = None


class BudgetPause(Exception):
    """The cycle is healthy but too close to its boundary for another move."""


def replay_partial(partial_path: Path, fixture: dict[str, Any], arm: str, rules: Any) -> tuple[list[int], list[dict[str, Any]], list[set[int]], int, list[tuple[int, int]], list[int], float]:
    partial = json.loads(partial_path.read_text())
    if partial.get("schema") != "compiled-h2h-partial-v1" or partial.get("fixture") != fixture or partial.get("arm") != arm:
        raise ValueError(f"partial checkpoint identity mismatch: {partial_path}")
    record = [int(value) for value in partial.get("record", [])]
    moves = partial.get("moves", [])
    if len(record) % 2 or len(moves) * 2 != len(record):
        raise ValueError(f"partial checkpoint prefix shape mismatch: {partial_path}")
    for index, move in enumerate(moves):
        prefix = record[:index * 2]
        points, leader, remaining, trick = rules.replay_record(fixture["hands"], prefix, fixture["decl"], fixture["bidder"])
        actor = (leader + len(trick)) % 4
        if int(move.get("actor")) != actor or int(move.get("tile")) != record[index * 2 + 1]:
            raise ValueError(f"partial checkpoint actor/tile mismatch: {partial_path}")
        legal = rules.legal_tiles(remaining[actor], trick, fixture["decl"])
        if move.get("legal") != legal or int(move["tile"]) not in legal:
            raise ValueError(f"partial checkpoint legal state mismatch: {partial_path}")
        request = move.get("request", {})
        history = [[record[offset], record[offset + 1]] for offset in range(0, len(prefix), 2)]
        if move.get("player") == "candidate":
            if request.get("hand") != sorted(remaining[actor]) or request.get("original_hand") != fixture["hands"][actor] or request.get("history") != history:
                raise ValueError(f"partial candidate request state mismatch: {partial_path}")
        elif move.get("player") == "cpu":
            if request.get("hand") != fixture["hands"][actor] or request.get("plays") != prefix:
                raise ValueError(f"partial CPU request state mismatch: {partial_path}")
        else:
            raise ValueError(f"partial checkpoint player mismatch: {partial_path}")
    points, leader, remaining, trick = rules.replay_record(fixture["hands"], record, fixture["decl"], fixture["bidder"])
    if partial.get("points") != points:
        raise ValueError(f"partial checkpoint points mismatch: {partial_path}")
    return record, moves, remaining, leader, trick, points, float(partial.get("elapsed_ms", 0.0))


def save_partial(path: Path, fixture: dict[str, Any], arm: str, record: list[int], moves: list[dict[str, Any]], points: list[int], elapsed_ms: float) -> None:
    atomic(path, {"schema": "compiled-h2h-partial-v1", "fixture": fixture, "arm": arm,
                  "record": record, "moves": moves, "points": points, "elapsed_ms": elapsed_ms,
                  "updated_at": time.time()})


def fixture_panel(panel: str, deals_per_cell: int) -> list[dict[str, Any]]:
    fixtures = []
    for decl in DECLARATIONS:
        for bidder in range(4):
            pairgroup = f"{panel}:decl-{decl}:bidder-{bidder}"
            for repeat in range(deals_per_cell):
                index = len(fixtures)
                deck = list(range(28))
                random.Random(seed("deals", panel, index)).shuffle(deck)
                fixtures.append({"index": index, "repeat": repeat, "pairgroup": pairgroup,
                                 "decl": decl, "bidder": bidder, "policy_seed": seed("policy", panel, index),
                                 "hands": [sorted(deck[seat * 7:seat * 7 + 7]) for seat in range(4)],
                                 "balanced_order": ["declaring", "defending"] if repeat % 2 == 0 else ["defending", "declaring"]})
    return fixtures


def play(fixture: dict[str, Any], arm: str, candidate: CandidateWorker, cpu: Any, session: Any,
         rules: Any, candidate_ms: int, config: dict[str, Any], deadline: float, partial_path: Path) -> dict[str, Any]:
    hands = fixture["hands"]
    prefix_elapsed = 0.0
    if partial_path.is_file():
        record, moves, remaining, leader, trick, points, prefix_elapsed = replay_partial(partial_path, fixture, arm, rules)
    else:
        remaining = [set(hand) for hand in hands]
        record, moves, leader, trick, points = [], [], fixture["bidder"], [], [0, 0]
    candidate_parity = fixture["bidder"] % 2 if arm == "declaring" else 1 - fixture["bidder"] % 2
    started = time.monotonic()
    save_partial(partial_path, fixture, arm, record, moves, points, prefix_elapsed)
    for turn in range(len(moves), 28):
        if deadline - time.monotonic() < CHECKPOINT_RESERVE_S:
            save_partial(partial_path, fixture, arm, record, moves, points,
                         prefix_elapsed + (time.monotonic() - started) * 1000)
            raise BudgetPause("cycle budget reserved for next game boundary")
        actor = (leader + len(trick)) % 4
        is_candidate = actor % 2 == candidate_parity
        legal = rules.legal_tiles(remaining[actor], trick, fixture["decl"])
        if is_candidate:
            request = {"decl": fixture["decl"], "bid": 30, "bidder": fixture["bidder"], "seat": actor,
                       "hand": sorted(remaining[actor]), "original_hand": hands[actor],
                       "history": [[record[i], record[i + 1]] for i in range(0, len(record), 2)],
                       "seed": fixture["policy_seed"], "budget_ms": candidate_ms, "config": config}
            move_started = time.monotonic()
            response = candidate.call(request)
            if "error" in response:
                raise RuntimeError(f"candidate decision failed: {response['error']}")
            if "tile" not in response:
                raise RuntimeError("candidate response omitted tile")
            choice = int(response["tile"])
        else:
            request = {"decl": fixture["decl"], "bid": 30, "bidder": fixture["bidder"], "seat": actor,
                       "hand": hands[actor], "plays": record[:], "seed": fixture["policy_seed"]}
            move_started = time.monotonic()
            response = cpu.decide(request, mode="partner", n=40, n0=8, n1=2, budget_ms=CPU_MS,
                                  inner_belief="voidless", selection="fixed", modeled_selection="fixed",
                                  session=session, review="off")
            choice = int(response["choice"])
        elapsed_ms = (time.monotonic() - move_started) * 1000
        if choice not in legal:
            raise RuntimeError(f"illegal player choice {choice} not in {legal}")
        moves.append({"actor": actor, "tile": choice, "player": "candidate" if is_candidate else "cpu",
                      "trick": turn // 4 + 1, "legal": legal, "elapsed_ms": elapsed_ms,
                      "request": request, "response": response})
        remaining[actor].remove(choice)
        record.extend([actor, choice])
        trick.append((actor, choice))
        if len(trick) == 4:
            leader = rules.winner(trick, fixture["decl"])
            points[leader % 2] += rules.trick_points(trick)
            trick = []
        save_partial(partial_path, fixture, arm, record, moves, points,
                     prefix_elapsed + (time.monotonic() - started) * 1000)
    replay = rules.replay_record(hands, record, fixture["decl"], fixture["bidder"])
    if replay[0] != points or sum(points) != 42 or trick or any(remaining):
        raise RuntimeError("independent replay failed")
    return {"schema": "compiled-h2h-game-v1", "fixture": fixture, "pairgroup": fixture["pairgroup"],
            "balanced_order": fixture["balanced_order"], "balancedorder": fixture["balanced_order"],
            "arm": arm, "record": record, "moves": moves, "points": points,
            "declaring_made": points[fixture["bidder"] % 2] >= 30,
            "elapsed_ms": prefix_elapsed + (time.monotonic() - started) * 1000, "independent_replay_passed": True}


def verify_game(game: dict[str, Any], rules: Any) -> None:
    fixture = game["fixture"]
    points, _, remaining, trick = rules.replay_record(fixture["hands"], game["record"], fixture["decl"], fixture["bidder"])
    if len(game["record"]) != 56 or trick or any(remaining) or game["points"] != points:
        raise ValueError(f"invalid game receipt {fixture['index']}-{game['arm']}")
    if game["declaring_made"] != (points[fixture["bidder"] % 2] >= 30):
        raise ValueError("declaring outcome mismatch")


def analyze(output: Path, protocol: dict[str, Any], rules: Any) -> dict[str, Any]:
    games = []
    by_fixture: dict[int, dict[str, Any]] = {}
    for fixture in protocol["fixtures"]:
        for arm in ("declaring", "defending"):
            path = output / f"{fixture['index']}-{arm}.json"
            if not path.is_file():
                continue
            game = json.loads(path.read_text())
            if game.get("fixture") != fixture:
                raise ValueError(f"fixture identity mismatch: {path}")
            verify_game(game, rules)
            games.append(game)
            by_fixture.setdefault(fixture["index"], {})[arm] = game
    pairs = []
    for fixture in protocol["fixtures"]:
        pair = by_fixture.get(fixture["index"], {})
        if len(pair) == 2:
            declaring = pair["declaring"]["declaring_made"]
            defending = pair["defending"]["declaring_made"]
            pairs.append({"index": fixture["index"], "pairgroup": fixture["pairgroup"], "decl": fixture["decl"],
                          "bidder": fixture["bidder"], "candidate_declaring_make": declaring,
                          "candidate_defending_make": defending, "delta": int(declaring) - int(defending)})
    moves = [move for game in games for move in game["moves"]]
    stats = {}
    for name in ("candidate", "cpu"):
        own_moves = [move for move in moves if move["player"] == name]
        partnership = [sum(move["elapsed_ms"] for move in game["moves"] if move["player"] == name) for game in games]
        if name == "candidate":
            fallbacks = sum(bool(move["response"].get("fallback", False)) for move in own_moves)
        else:
            fallbacks = sum(str(move["response"].get("route", "")).endswith("fallback") for move in own_moves)
        stats[name] = {"moves": len(own_moves), "nonforced": sum(len(move["legal"]) > 1 for move in own_moves),
                       "fallbacks": fallbacks, "decision_time": summary_stats([move["elapsed_ms"] for move in own_moves]),
                       "partnership_time": summary_stats(partnership),
                       "by_trick_ms": {str(trick): sum(move["elapsed_ms"] for move in own_moves if move["trick"] == trick) for trick in range(1, 8)}}
    deltas = [pair["delta"] for pair in pairs]
    mean = statistics.mean(deltas) if deltas else None
    compute_totals: Counter[str] = Counter()
    compute_backends: Counter[str] = Counter()
    for move in moves:
        if move["player"] != "candidate":
            continue
        compute = move["response"].get("compute", {})
        if not isinstance(compute, dict):
            continue
        backend = compute.get("backend")
        if isinstance(backend, str):
            compute_backends[backend] += 1
        stats_value = compute.get("stats", {})
        if isinstance(stats_value, dict):
            for key, value in stats_value.items():
                if isinstance(value, (int, float)) and not isinstance(value, bool):
                    compute_totals[key] += value
    candidate_compute = {"backends": dict(compute_backends), "totals": dict(compute_totals)}
    summary = {"schema": "compiled-h2h-summary-v1", "complete": len(pairs) == len(protocol["fixtures"]),
               "pairs": len(pairs), "games": len(games), "plays": len(moves),
               "all_replay_checks_pass": True, "wins": sum(delta > 0 for delta in deltas),
               "losses": sum(delta < 0 for delta in deltas), "ties": sum(delta == 0 for delta in deltas),
               "mean_paired_delta": mean, "stats": stats, "paired_results": pairs,
               "all_fallbacks": {name: stats[name]["fallbacks"] for name in stats},
               "allfallbacks": {name: stats[name]["fallbacks"] for name in stats},
               "outcome": {"wins": sum(delta > 0 for delta in deltas),
                           "losses": sum(delta < 0 for delta in deltas),
                           "ties": sum(delta == 0 for delta in deltas), "mean_paired_delta": mean},
               "partnership_time": {name: stats[name]["partnership_time"] for name in stats},
               "compute": {"candidate": candidate_compute},
               "development_only": True, "promotion": "none", "scope": "paired calibration comparison"}
    if protocol.get("confirmation_protocol"):
        summary.update({
            "development_only": False,
            "data_role": protocol.get("data_role", "CONFIRMATION"),
            "final_holdout": protocol.get("final_holdout", protocol["confirmation_protocol"].get("id")),
            "promotion": protocol.get("promotion", "none"),
            "scope": protocol.get("scope", "frozen confirmation comparison"),
            "confirmation_protocol": protocol["confirmation_protocol"],
        })
    atomic(output / "summary.json", summary)
    return summary


def source_identities(cpu_root: Path, cpu: Any, candidate_command: list[str], c0: Path, c1: Path) -> dict[str, Any]:
    source_names = ["experiments/partnership/player.py", "experiments/partnership/runtime.py",
                    "experiments/partnership/rules.py", "experiments/partnership/players.json"]
    cpu_binary = Path(cpu.BINARY)
    return {"candidate_command": executable_identity(candidate_command), "c0": input_identity(c0), "c1": input_identity(c1),
            "candidate_sources": {"src": input_identity(HERE / "src"),
                                  "Cargo.toml": input_identity(HERE / "Cargo.toml"),
                                  "Cargo.lock": input_identity(HERE / "Cargo.lock")},
            "cpu_root": str(cpu_root.resolve()), "cpu_binary": input_identity(cpu_binary),
            "cpu_head": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=cpu_root, text=True).strip(),
            "cpu_sources": {name: digest(cpu_root / name) for name in source_names},
            "harness_sha256": digest(Path(__file__))}


def load_config(path: str | None) -> tuple[dict[str, Any], dict[str, Any]]:
    if path is None:
        return {"outer": 40, "plans": 1, "horizon": 7, "work": 2_000_000}, {"path": None, "sha256": None}
    value_path = Path(path).expanduser().resolve()
    value = json.loads(value_path.read_text())
    if not isinstance(value, dict):
        raise SystemExit("--config-json must contain an object")
    return value, {"path": str(value_path), "sha256": digest(value_path)}


def load_confirmation_protocol(path_value: str) -> tuple[Path, dict[str, Any]]:
    path = Path(path_value).expanduser().resolve()
    try:
        value = json.loads(path.read_text())
    except (OSError, json.JSONDecodeError) as exc:
        raise SystemExit(f"invalid confirmation protocol: {path}: {exc}") from exc
    if not isinstance(value, dict) or value.get("schema") != "compiled-confirmation-protocol-v1":
        raise SystemExit("confirmation protocol schema must be compiled-confirmation-protocol-v1")
    if not isinstance(value.get("id"), str) or not value["id"]:
        raise SystemExit("confirmation protocol id must be nonempty")
    panels = value.get("panels")
    if (not isinstance(panels, list) or not panels or
            any(not isinstance(panel, str) or not panel for panel in panels) or len(set(panels)) != len(panels)):
        raise SystemExit("confirmation protocol panels must be a nonempty list of unique names")
    for key in ("deals_per_cell", "candidate_ms", "threads"):
        if type(value.get(key)) is not int or value[key] < 1:
            raise SystemExit(f"confirmation protocol {key} must be a positive integer")
    if not isinstance(value.get("candidate_config"), dict):
        raise SystemExit("confirmation protocol candidate_config must be an object")
    required = ("candidate_command", "c0", "c1", "candidate_sources", "cpu_root",
                "cpu_binary", "cpu_head", "cpu_sources")
    identities = value.get("identities")
    if not isinstance(identities, dict) or any(key not in identities for key in required):
        raise SystemExit("confirmation protocol identities are incomplete")
    if any(identities[key] is None for key in required):
        raise SystemExit("confirmation protocol identities contain null values")
    return path, value


def validate_confirmation_protocol(spec: dict[str, Any], args: argparse.Namespace,
                                   config: dict[str, Any], identities: dict[str, Any]) -> None:
    if args.panel not in spec["panels"]:
        raise SystemExit(f"panel {args.panel!r} is not nominated by confirmation protocol")
    for key, actual in (("deals_per_cell", args.deals_per_cell), ("candidate_ms", args.candidate_ms),
                        ("threads", args.threads)):
        if spec[key] != actual:
            raise SystemExit(f"confirmation protocol {key} drift: expected {spec[key]}, got {actual}")
    if "cpu_ms" in spec and spec["cpu_ms"] != CPU_MS:
        raise SystemExit(f"confirmation protocol cpu_ms drift: expected {CPU_MS}, got {spec['cpu_ms']}")
    if spec["candidate_config"] != config:
        raise SystemExit("confirmation protocol candidate config drift")
    if "fixture_hashes" in spec:
        hashes = spec["fixture_hashes"]
        if not isinstance(hashes, dict) or args.panel not in hashes:
            raise SystemExit(f"confirmation protocol fixture hash missing for {args.panel}")
        actual_hash = hashlib.sha256(canonical(fixture_panel(args.panel, args.deals_per_cell)).encode()).hexdigest()
        if hashes[args.panel] != actual_hash:
            raise SystemExit(f"confirmation protocol fixture hash drift for {args.panel}")
    if "authority_document" in spec:
        authority = spec["authority_document"]
        if (not isinstance(authority, dict) or not isinstance(authority.get("path"), str)
                or not isinstance(authority.get("sha256"), str)):
            raise SystemExit("confirmation protocol authority_document is malformed")
        authority_path = Path(authority["path"]).expanduser().resolve()
        if not authority_path.is_file() or digest(authority_path) != authority["sha256"]:
            raise SystemExit("confirmation protocol authority document drift")
    for key in ("candidate_command", "c0", "c1", "candidate_sources", "cpu_root",
                "cpu_binary", "cpu_head", "cpu_sources"):
        if identities.get(key) != spec["identities"][key]:
            raise SystemExit(f"confirmation protocol identity drift for {key}")


def verify_frozen(output: Path, protocol: dict[str, Any], args: argparse.Namespace, identities: dict[str, Any],
                  config_meta: dict[str, Any], confirmation_meta: dict[str, Any] | None = None) -> None:
    expected = protocol["identities"]
    for key in ("candidate_command", "c0", "c1", "candidate_sources", "cpu_root", "cpu_binary", "cpu_head", "cpu_sources", "harness_sha256"):
        if identities.get(key) != expected.get(key):
            raise SystemExit(f"frozen identity drift for {key}; use a new output")
    if (config_meta != protocol["config_source"] or args.panel != protocol["panel"] or
            args.deals_per_cell != protocol["deals_per_cell"] or args.candidate_ms != protocol["candidate_ms"] or
            args.threads != protocol["threads"]):
        raise SystemExit("frozen panel/deals/candidate config drift; use a new output")
    stored_confirmation = protocol.get("confirmation_protocol")
    if bool(stored_confirmation) != bool(confirmation_meta):
        raise SystemExit("confirmation mode drift; use the original confirmation protocol option")
    if stored_confirmation and stored_confirmation != confirmation_meta:
        raise SystemExit("confirmation protocol document drift; use a new output")


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--candidate-command")
    parser.add_argument("--actor-c0", "--c0", dest="actor_c0")
    parser.add_argument("--actor-c1", "--c1", dest="actor_c1")
    parser.add_argument("--cpu-root", type=Path, default=CPU_ROOT_DEFAULT)
    parser.add_argument("--panel", default="fresh-72-v1")
    parser.add_argument("--deals-per-cell", type=int, default=2)
    parser.add_argument("--seconds", type=float, default=55.0)
    parser.add_argument("--candidate-ms", type=int, default=5)
    parser.add_argument("--config-json")
    parser.add_argument("--threads", type=int, default=6)
    parser.add_argument("--confirmation-protocol", help="frozen compiled-confirmation-protocol-v1 document")
    parser.add_argument("--analyze-only", action="store_true")
    args = parser.parse_args(argv)
    if args.deals_per_cell < 1 or args.candidate_ms < 1 or args.threads < 1:
        raise SystemExit("deals-per-cell, candidate-ms, and threads must be positive")
    output = args.output.expanduser().resolve()
    if args.candidate_command is None and not args.analyze_only:
        raise SystemExit("--candidate-command is required for a comparison")
    command, c0, c1 = (extract_actor_paths(shlex.split(args.candidate_command), args.actor_c0, args.actor_c1)
                        if args.candidate_command is not None else ([], None, None))
    config, config_meta = load_config(args.config_json)
    if args.analyze_only and args.confirmation_protocol:
        raise SystemExit("--confirmation-protocol cannot be combined with --analyze-only; use the stored protocol")
    confirmation_path = confirmation_spec = None
    if args.confirmation_protocol:
        confirmation_path, confirmation_spec = load_confirmation_protocol(args.confirmation_protocol)
    sys.path.insert(0, str(args.cpu_root / "experiments/partnership"))
    import player as cpu
    import rules
    from runtime import DecisionSession
    os.environ["WALT_RAYON_THREADS"] = str(args.threads)
    os.environ["RAYON_NUM_THREADS"] = str(args.threads)
    protocol_path = output / "protocol.json"
    if args.analyze_only:
        if not protocol_path.is_file():
            raise SystemExit("missing protocol.json")
        summary = analyze(output, json.loads(protocol_path.read_text()), rules)
        print(json.dumps({key: value for key, value in summary.items() if key not in ("paired_results", "stats")}, indent=2))
        return 0
    assert c0 is not None and c1 is not None
    identities = source_identities(args.cpu_root, cpu, command, c0, c1)
    confirmation_meta = None
    if confirmation_spec is not None:
        validate_confirmation_protocol(confirmation_spec, args, config, identities)
        confirmation_meta = {"path": str(confirmation_path), "sha256": digest(confirmation_path),
                             "schema": confirmation_spec["schema"], "id": confirmation_spec["id"]}
    fixtures = fixture_panel(args.panel, args.deals_per_cell)
    if output.exists() and any(output.iterdir()):
        if not protocol_path.is_file():
            raise SystemExit("refusing to resume an output without protocol.json")
        protocol = json.loads(protocol_path.read_text())
        verify_frozen(output, protocol, args, identities, config_meta, confirmation_meta)
        if protocol["candidate_config"] != config:
            raise SystemExit("candidate config drift; use a new output")
        if protocol["fixtures"] != fixture_panel(args.panel, args.deals_per_cell):
            raise SystemExit("frozen fixture panel drift; use a new output")
    else:
        output.mkdir(parents=True, exist_ok=True)
        protocol = {"schema": "compiled-h2h-protocol-v1", "panel": args.panel, "deals_per_cell": args.deals_per_cell,
                    "candidate_ms": args.candidate_ms, "cpu_ms": CPU_MS, "threads": args.threads,
                    "candidate_config": config,
                    "config_source": config_meta, "fixtures": fixtures, "identities": identities,
                    "python": sys.version, "platform": platform.platform(),
                    "build_flags": os.environ.get("RUSTFLAGS", ""),
                    "data_role": "CONFIRMATION" if confirmation_meta else "DEVELOPMENT",
                    "final_holdout": confirmation_spec["id"] if confirmation_spec else "none",
                    "promotion": "none",
                    "development_only": False if confirmation_meta else True,
                    "confirmation_protocol": confirmation_meta,
                    "cpu_profile": "current deployed L2 Partner 40/8/2 Fixed/Fixed Voidless, review off; actual reserve wrapper",
                    "analysis": ("Frozen CONFIRMATION comparison; no automatic promotion." if confirmation_meta
                                 else "Balanced DEVELOPMENT paired comparison; no final promotion or holdout claim."),
                    "scope": ("frozen confirmation only" if confirmation_meta else "paired calibration comparison"),
                    "order": "declaring-first on even repeats, defending-first on odd repeats"}
        atomic(protocol_path, protocol)
    failed = set()
    failures_path = output / "failures.jsonl"
    if failures_path.is_file():
        for line in failures_path.read_text().splitlines():
            if line.strip():
                failed.add(json.loads(line)["key"])
    completed = {(fixture["index"], arm) for fixture in protocol["fixtures"] for arm in ("declaring", "defending")
                 if (output / f"{fixture['index']}-{arm}.json").is_file()}
    pending = [(fixture, arm) for fixture in protocol["fixtures"] for arm in fixture["balanced_order"]
               if (fixture["index"], arm) not in completed and f"{fixture['index']}-{arm}" not in failed]
    if not pending:
        summary = analyze(output, protocol, rules)
        print(json.dumps({"complete": summary["complete"], "pairs": summary["pairs"], "games": summary["games"]}))
        return 0
    deadline = time.monotonic() + max(0.0, args.seconds)
    worker = CandidateWorker(command, max(5.0, args.candidate_ms / 1000.0 + 2.0))
    stop_reason = "exhausted"
    cycle_start_identities = identities
    startup: dict[str, Any] = {"started_at": time.time(), "start_identities": cycle_start_identities,
                                "threads": args.threads, "python": sys.version, "platform": platform.platform(),
                                "build_flags": os.environ.get("RUSTFLAGS", "")}
    try:
        started = time.monotonic()
        startup["ready"] = worker.start()
        startup["startup_ms"] = (time.monotonic() - started) * 1000
        warmup = {"decl": 6, "bid": 30, "bidder": 1, "seat": 1, "hand": [2, 6, 13, 16, 19, 21, 26],
                  "original_hand": [2, 6, 13, 16, 19, 21, 26], "history": [], "seed": 50129,
                  "budget_ms": args.candidate_ms, "config": config}
        started = time.monotonic()
        startup["candidate_warmup"] = worker.call(warmup)
        startup["candidate_warmup_ms"] = (time.monotonic() - started) * 1000
        if "error" in startup["candidate_warmup"] or "tile" not in startup["candidate_warmup"]:
            raise RuntimeError("candidate warmup did not return a tile")
        with DecisionSession() as session:
            cpu_warmup = {"decl": 6, "bid": 30, "bidder": 1, "seat": 1,
                          "hand": [2, 6, 13, 16, 19, 21, 26], "plays": [], "seed": 50129}
            started = time.monotonic()
            startup["cpu_warmup"] = cpu.decide(cpu_warmup, mode="partner", n=40, n0=8, n1=2,
                                               budget_ms=CPU_MS, inner_belief="voidless", selection="fixed",
                                               modeled_selection="fixed", session=session, review="off")
            startup["cpu_warmup_ms"] = (time.monotonic() - started) * 1000
            startup["finished_at"] = time.time()
            atomic(output / "startup.json", startup)
            append_jsonl(output / "cycles.jsonl", {"started_at": startup["started_at"], "warmup": startup,
                                                    "candidate_config": config, "seconds": args.seconds})
            for fixture, arm in pending:
                if time.monotonic() >= deadline:
                    stop_reason = "budget"
                    break
                key = f"{fixture['index']}-{arm}"
                partial_path = output / f"{key}.partial.json"
                try:
                    game = play(fixture, arm, worker, cpu, session, rules, args.candidate_ms, config, deadline, partial_path)
                    atomic(output / f"{key}.json", game)
                    partial_path.unlink(missing_ok=True)
                    completed.add((fixture["index"], arm))
                    atomic(output / "progress.json", {"completed_arms": len(completed), "planned_arms": len(protocol["fixtures"]) * 2,
                                                        "failed_arms": len(failed), "last": key, "updated_at": time.time()})
                except BudgetPause:
                    stop_reason = "budget"
                    break
                except Exception as exc:
                    append_jsonl(failures_path, {"key": key, "fixture": fixture, "arm": arm,
                                                 "error": str(exc), "recorded_at": time.time()})
                    failed.add(key)
                    stop_reason = "game_error"
                    break
            else:
                stop_reason = "exhausted"
    except Exception as exc:
        stop_reason = "worker_error"
        startup["error"] = str(exc)
        startup["finished_at"] = time.time()
        atomic(output / "startup-error.json", startup)
    finally:
        worker.close()
    try:
        cycle_end_identities: Any = source_identities(args.cpu_root, cpu, command, c0, c1)
    except BaseException as exc:
        cycle_end_identities = {"error": str(exc)}
    identity_drift = cycle_end_identities != cycle_start_identities
    if identity_drift and stop_reason == "exhausted":
        stop_reason = "identity_drift"
    atomic(output / "progress.json", {"completed_arms": len(completed), "planned_arms": len(protocol["fixtures"]) * 2,
                                       "failed_arms": len(failed), "stop_reason": stop_reason, "updated_at": time.time()})
    summary = analyze(output, protocol, rules)
    receipt = {"schema": "compiled-h2h-cycle-v1", "started_at": startup["started_at"], "finished_at": time.time(),
               "seconds": args.seconds, "stop_reason": stop_reason, "completed_arms": len(completed),
               "failed_arms": len(failed), "planned_arms": len(protocol["fixtures"]) * 2,
               "summary_complete": summary["complete"], "start_identities": cycle_start_identities,
               "end_identities": cycle_end_identities, "identity_drift": identity_drift,
               "threads": args.threads, "python": sys.version, "platform": platform.platform(),
               "build_flags": os.environ.get("RUSTFLAGS", "")}
    append_jsonl(output / "cycles.jsonl", receipt)
    atomic(output / "cycle-receipt.json", receipt)
    print(json.dumps({"stop_reason": stop_reason, "completed_arms": len(completed), "failed_arms": len(failed),
                      "pairs": summary["pairs"], "games": summary["games"]}))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
