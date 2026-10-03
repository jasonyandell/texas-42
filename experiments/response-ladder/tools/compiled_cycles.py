#!/usr/bin/env python3
"""Prepare and teach the small, resumable compiled-response calibration corpus.

The source receipts contain two arms for each physical fixture.  This tool keeps
those arms together, removes only duplicate public requests inside that fixture,
and sends the resulting requests to one persistent teacher worker per cycle.
It deliberately treats worker responses as opaque data: only the response
``status`` is inspected for resumability.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import selectors
import shlex
import subprocess
import sys
import tempfile
import time
from collections import Counter, defaultdict
from typing import Any, Iterable


EXPECTED_STATUSES = {"complete", "refused", "forced", "settled", "error"}
TERMINAL_STATUSES = {"complete", "refused", "forced", "settled"}
LABEL_STATUSES = {"complete", "forced", "settled"}
PUBLIC_FIELDS = ("decl", "bid", "bidder", "seat", "hand", "original_hand", "history", "seed")


def canonical(value: Any) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def json_document(value: Any) -> str:
    return json.dumps(value, sort_keys=True, indent=2, ensure_ascii=True) + "\n"


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def write_atomic(path: Path, value: Any, *, mode: str = "w") -> None:
    """Write JSON or text atomically in the destination directory."""
    path.parent.mkdir(parents=True, exist_ok=True)
    fd, temporary = tempfile.mkstemp(prefix=f".{path.name}.", dir=str(path.parent))
    try:
        with os.fdopen(fd, mode, encoding="utf-8") as stream:
            if mode == "w":
                if isinstance(value, str):
                    stream.write(value)
                else:
                    json.dump(value, stream, sort_keys=True, indent=2, ensure_ascii=True)
                    stream.write("\n")
            stream.flush()
            os.fsync(stream.fileno())
        os.replace(temporary, path)
    finally:
        try:
            os.unlink(temporary)
        except FileNotFoundError:
            pass


def read_json(path: Path) -> Any:
    with path.open(encoding="utf-8") as stream:
        return json.load(stream)


def source_receipts(source: Path) -> list[tuple[Path, dict[str, Any]]]:
    receipts: list[tuple[Path, dict[str, Any]]] = []
    for path in sorted(source.glob("*.json")):
        try:
            value = read_json(path)
        except (OSError, json.JSONDecodeError):
            continue
        if isinstance(value, dict) and isinstance(value.get("fixture"), dict) and isinstance(value.get("moves"), list):
            receipts.append((path, value))
    return receipts


def history_from_flat_plays(plays: Iterable[Any], bidder: int = 0) -> list[list[int]]:
    """Decode the historical CPU adapter's flat ``[seat,tile,...]`` form."""
    values = [int(value) for value in plays]
    if len(values) % 2:
        raise ValueError("flat plays must contain seat/tile pairs")
    history: list[list[int]] = []
    for offset in range(0, len(values), 2):
        history.append([values[offset], values[offset + 1]])
    return history


def authoritative_history(game: dict[str, Any], move_index: int) -> list[list[int]]:
    history: list[list[int]] = []
    all_tiles = {int(tile) for hand in game["fixture"]["hands"] for tile in hand}
    seen_tiles: set[int] = set()
    hand_counts = [Counter(int(tile) for tile in hand) for hand in game["fixture"]["hands"]]
    for prior_index, prior in enumerate(game["moves"][:move_index]):
        if not isinstance(prior, dict) or "actor" not in prior or "tile" not in prior:
            raise ValueError(f"move {prior_index} has no authoritative actor/tile")
        actor, tile = int(prior["actor"]), int(prior["tile"])
        if not 0 <= actor < 4 or tile not in all_tiles or tile in seen_tiles or hand_counts[actor][tile] <= 0:
            raise ValueError(f"move {prior_index} has an impossible actor/tile prefix")
        if tile not in {int(candidate) for candidate in prior.get("legal", [])}:
            raise ValueError(f"move {prior_index} tile is absent from its legal set")
        hand_counts[actor][tile] -= 1
        seen_tiles.add(tile)
        history.append([actor, tile])
    return history


def expected_remaining(fixture: dict[str, Any], history: list[list[int]], seat: int) -> list[int]:
    original = sorted(int(tile) for tile in fixture["hands"][seat])
    counts = Counter(original)
    for prior_seat, tile in history:
        if not 0 <= prior_seat < 4:
            raise ValueError(f"impossible prior seat {prior_seat}")
        if prior_seat == seat:
            if counts[tile] <= 0:
                raise ValueError(f"seat {seat} has an impossible prior hand")
            counts[tile] -= 1
    if any(count < 0 for count in counts.values()):
        raise ValueError(f"seat {seat} has an impossible prior hand")
    return sorted(tile for tile, count in counts.items() for _ in range(count))


def normalize_request(game: dict[str, Any], move: dict[str, Any], move_index: int, source_name: str) -> dict[str, Any]:
    fixture = game["fixture"]
    old = move.get("request") or {}
    actor = int(move.get("actor"))
    seat = int(old.get("seat", actor))
    if seat != actor:
        raise ValueError(f"{source_name}: move {move_index} request seat {seat} != actor {actor}")
    bidder = int(old.get("bidder", fixture["bidder"]))
    history = authoritative_history(game, move_index)
    expected_hand = expected_remaining(fixture, history, seat)
    fixture_original = sorted(int(tile) for tile in fixture["hands"][seat])
    current_tile = int(move["tile"])
    legal = {int(tile) for tile in move.get("legal", [])}
    if current_tile not in expected_hand or current_tile not in legal or not legal <= set(expected_hand):
        raise ValueError(f"{source_name}: move {move_index} has inconsistent legal hand state")
    original = old.get("original_hand")
    if original is not None and sorted(int(tile) for tile in original) != fixture_original:
        raise ValueError(f"{source_name}: move {move_index} original_hand disagrees with fixture")
    if old.get("hand") is not None:
        supplied_hand = sorted(int(tile) for tile in old["hand"])
        # CPU requests carry the original hand and flat plays; candidate
        # requests carry remaining hand plus original_hand/history pairs.
        candidate_shape = old.get("original_hand") is not None or old.get("history") is not None
        expected_supplied = expected_hand if candidate_shape else fixture_original
        if supplied_hand != expected_supplied:
            raise ValueError(f"{source_name}: move {move_index} hand disagrees with authoritative prefix")
    if old.get("history") is not None:
        supplied_history = [[int(pair[0]), int(pair[1])] for pair in old["history"]]
        if supplied_history != history:
            raise ValueError(f"{source_name}: move {move_index} history disagrees with authoritative prefix")
    if old.get("plays") is not None:
        supplied_history = history_from_flat_plays(old["plays"])
        if supplied_history != history:
            raise ValueError(f"{source_name}: move {move_index} flat plays disagree with authoritative prefix")

    bid = old.get("bid", game.get("bid", 30))
    seed = old.get("seed", fixture.get("policy_seed"))
    if seed is None:
        raise ValueError(f"{source_name}: move {move_index} has no seed")
    return {
        "decl": int(old.get("decl", fixture["decl"])),
        "bid": int(bid),
        "bidder": bidder,
        "seat": seat,
        "hand": expected_hand,
        "original_hand": fixture_original,
        "history": history,
        "seed": int(seed),
    }


def is_forced(move: dict[str, Any]) -> bool:
    response = move.get("response") or {}
    if response.get("reason") == "forced" or response.get("route") == "forced":
        return True
    return len(move.get("legal") or []) <= 1


def fixture_identity(fixture: dict[str, Any]) -> str:
    # Include all physical inputs so an index reused by a later source cannot
    # silently merge state with this calibration corpus.
    identity = {
        "index": fixture.get("index"),
        "repeat": fixture.get("repeat"),
        "decl": fixture.get("decl"),
        "bidder": fixture.get("bidder"),
        "hands": fixture.get("hands"),
        "policy_seed": fixture.get("policy_seed"),
    }
    return canonical(identity)


def prepare(args: argparse.Namespace) -> int:
    source = Path(args.source).expanduser().resolve()
    output = Path(args.output).expanduser().resolve()
    if not source.is_dir():
        raise SystemExit(f"source directory does not exist: {source}")
    receipts = source_receipts(source)
    if not receipts:
        raise SystemExit(f"no game receipts found in {source}")

    source_files = [
        {"path": str(path.relative_to(source)), "sha256": sha256_file(path), "bytes": path.stat().st_size}
        for path, _ in receipts
    ]
    source_manifest = sha256_bytes(canonical(source_files).encode())
    source_scope = source_manifest[:16]
    rows_by_group: dict[str, dict[str, Any]] = {}
    origins_by_id: defaultdict[str, list[dict[str, Any]]] = defaultdict(list)
    duplicate_count = 0
    fixtures: dict[str, dict[str, Any]] = {}

    for path, game in receipts:
        fixture = game["fixture"]
        repeat = fixture.get("repeat")
        if repeat not in (0, 1):
            raise SystemExit(f"{path}: fixture repeat must be 0 or 1, got {repeat!r}")
        fixture_key = fixture_identity(fixture)
        group = f"{source_scope}:fixture-{fixture.get('index')}:repeat-{repeat}"
        fixtures[fixture_key] = {"group": group, "index": fixture.get("index"), "repeat": repeat}
        split = "train" if repeat == 0 else "development"
        for move_index, move in enumerate(game["moves"]):
            if not isinstance(move, dict) or is_forced(move):
                continue
            try:
                request = normalize_request(game, move, move_index, path.name)
            except ValueError as exc:
                raise SystemExit(str(exc)) from exc
            semantic = canonical(request)
            request_id = sha256_bytes((fixture_key + "\n" + semantic).encode())[:32]
            origin = {
                "source": path.name,
                "arm": game.get("arm", move.get("player")),
                "move": move_index,
                "player": move.get("player"),
            }
            if request_id in rows_by_group:
                duplicate_count += 1
            else:
                rows_by_group[request_id] = {
                    "id": request_id,
                    "group": group,
                    "split": split,
                    "request": request,
                    "origins": [],
                }
            origins_by_id[request_id].append(origin)

    for request_id, origins in origins_by_id.items():
        rows_by_group[request_id]["origins"] = origins

    rows = sorted(rows_by_group.values(), key=lambda row: (row["split"], row["group"], row["id"]))
    requests_path = output / "requests.jsonl"
    text = "".join(canonical(row) + "\n" for row in rows)
    requests_digest = sha256_bytes(text.encode())

    counters: dict[str, Counter[str]] = {
        "handsize": Counter(),
        "decl": Counter(),
        "bidder": Counter(),
        "seat": Counter(),
        "split": Counter(),
    }
    for row in rows:
        req = row["request"]
        counters["handsize"][str(len(req["hand"]))] += 1
        counters["decl"][str(req["decl"])] += 1
        counters["bidder"][str(req["bidder"])] += 1
        counters["seat"][str(req["seat"])] += 1
        counters["split"][row["split"]] += 1
    manifest = {
        "schema": "compiled-calibration-manifest-v1",
        "kind": "calibration",
        "source": str(source),
        "source_scope": source_scope,
        "source_manifest_sha256": source_manifest,
        "source_files": source_files,
        "source_receipts": len(receipts),
        "physical_fixtures": len(fixtures),
        "requests_file": "requests.jsonl",
        "requests_sha256": requests_digest,
        "requests_total": len(rows),
        "deduplicated_within_fixture": duplicate_count,
        "split_rule": {"repeat_0": "train", "repeat_1": "development", "final_holdout": "none"},
        "counts": {name: dict(sorted(counter.items())) for name, counter in counters.items()},
        "all_data_role": "CALIBRATION",
        "teacher_values_used_for_split": False,
    }
    manifest_path = output / "manifest.json"
    if output.exists() and not output.is_dir():
        raise SystemExit(f"output is not a directory: {output}")
    if output.exists() and any(output.iterdir()):
        if not manifest_path.is_file() or not requests_path.is_file():
            raise SystemExit("refusing to prepare over an existing incomplete campaign")
        if manifest_path.read_text(encoding="utf-8") != json_document(manifest) or requests_path.read_text(encoding="utf-8") != text:
            raise SystemExit("existing campaign manifest/requests differ; refusing to overwrite")
        print(json.dumps({"output": str(output), "requests": len(rows), "fixtures": len(fixtures), "manifest": manifest["source_manifest_sha256"], "unchanged": True}))
        return 0
    output.mkdir(parents=True, exist_ok=True)
    write_atomic(requests_path, text)
    write_atomic(output / "manifest.json", manifest)
    print(json.dumps({"output": str(output), "requests": len(rows), "fixtures": len(fixtures), "manifest": manifest["source_manifest_sha256"]}))
    return 0


def load_requests(output: Path) -> tuple[dict[str, Any], list[dict[str, Any]]]:
    manifest_path = output / "manifest.json"
    requests_path = output / "requests.jsonl"
    if not manifest_path.is_file() or not requests_path.is_file():
        raise SystemExit(f"prepared corpus is incomplete: {output}")
    manifest = read_json(manifest_path)
    actual_digest = sha256_file(requests_path)
    if actual_digest != manifest.get("requests_sha256"):
        raise SystemExit("requests.jsonl digest does not match manifest; refusing to teach")
    rows = []
    seen_ids: set[str] = set()
    with requests_path.open(encoding="utf-8") as stream:
        for line_number, line in enumerate(stream, 1):
            if not line.strip():
                continue
            row = json.loads(line)
            if not isinstance(row, dict) or not {"id", "group", "split", "request"} <= row.keys():
                raise SystemExit(f"requests.jsonl line {line_number} has the wrong shape")
            if row["id"] in seen_ids:
                raise SystemExit(f"requests.jsonl contains duplicate id {row['id']}")
            seen_ids.add(row["id"])
            rows.append(row)
    if len(rows) != manifest.get("requests_total"):
        raise SystemExit("requests.jsonl count does not match manifest")
    return manifest, rows


def binary_identity(command: list[str]) -> dict[str, Any]:
    path = Path(command[0]).expanduser()
    if not path.is_absolute():
        resolved = subprocess.run(["which", str(path)], capture_output=True, text=True, check=False).stdout.strip()
        if resolved:
            path = Path(resolved)
    identity: dict[str, Any] = {"argv": command, "path": str(path.resolve())}
    if path.is_file():
        identity.update({"sha256": sha256_file(path), "bytes": path.stat().st_size})
    else:
        identity.update({"sha256": None, "bytes": None})
    inputs = {}
    for index, argument in enumerate(command[:-1]):
        if argument in ('--c0', '--c1'):
            actor_path = Path(command[index + 1]).expanduser().resolve()
            inputs[argument] = {'path': str(actor_path), 'sha256': sha256_file(actor_path)}
    if inputs:
        identity['inputs'] = inputs
    return identity


def read_existing_lessons(output: Path, rows_by_id: dict[str, dict[str, Any]]) -> dict[str, dict[str, Any]]:
    lessons: dict[str, dict[str, Any]] = {}
    lesson_dir = output / "lessons"
    lesson_dir.mkdir(parents=True, exist_ok=True)
    for path in sorted(lesson_dir.glob("*.json")):
        try:
            lesson = read_json(path)
        except (OSError, json.JSONDecodeError) as exc:
            raise SystemExit(f"cannot read lesson {path}: {exc}")
        if not isinstance(lesson, dict):
            raise SystemExit(f"lesson {path} is not an object")
        lesson_id = lesson.get("id")
        if lesson_id not in rows_by_id or path.stem != lesson_id:
            raise SystemExit(f"lesson identity is not present in requests.jsonl: {path}")
        row = rows_by_id[lesson_id]
        if lesson.get("group") != row["group"] or lesson.get("split") != row["split"] or lesson.get("request") != row["request"]:
            raise SystemExit(f"lesson request identity mismatch: {path}")
        status = lesson.get("status")
        if status != response_status(lesson.get("response")):
            raise SystemExit(f"lesson status does not match response status: {path}")
        if status in TERMINAL_STATUSES:
            lessons[lesson_id] = lesson
    return lessons


def response_status(response: Any) -> str:
    if isinstance(response, dict):
        status = response.get("status")
        if isinstance(status, str):
            return status
    return "error"


def next_attempt_number(output: Path, request_id: str) -> int:
    attempts = output / "attempts" / request_id
    highest = 0
    current = output / "lessons" / f"{request_id}.json"
    candidates = ([current] if current.is_file() else []) + list(attempts.glob("*.json"))
    for path in candidates:
        try:
            value = read_json(path)
            highest = max(highest, int(value.get("attempt", 0)))
        except (OSError, ValueError, TypeError, json.JSONDecodeError):
            continue
    return highest + 1


class Worker:
    def __init__(self, command: list[str], handshake_timeout: float, response_timeout: float):
        self.command = command
        self.handshake_timeout = handshake_timeout
        self.response_timeout = response_timeout
        self.process: subprocess.Popen[bytes] | None = None
        self.selector: selectors.BaseSelector | None = None
        self.read_buffer = bytearray()

    def start(self) -> dict[str, Any]:
        self.process = subprocess.Popen(
            self.command,
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            # Keep the worker in this process group so an outer watchdog can
            # terminate the whole bounded cycle.  The teacher worker does not
            # launch external grandchildren.
            start_new_session=False,
            bufsize=0,
        )
        assert self.process.stdout is not None
        self.selector = selectors.DefaultSelector()
        self.selector.register(self.process.stdout, selectors.EVENT_READ)
        ready = self.read_json(self.handshake_timeout)
        if not isinstance(ready, dict) or ready.get("ready") is not True:
            raise RuntimeError(f"teacher worker did not send ready:true: {ready!r}")
        return ready

    def read_json(self, timeout: float) -> Any:
        if self.selector is None or self.process is None or self.process.stdout is None:
            raise RuntimeError("teacher worker is not running")
        deadline = time.monotonic() + max(0.0, timeout)
        while True:
            if b"\n" in self.read_buffer:
                line, _, rest = self.read_buffer.partition(b"\n")
                self.read_buffer = bytearray(rest)
                try:
                    return json.loads(line.decode("utf-8"))
                except (UnicodeDecodeError, json.JSONDecodeError) as exc:
                    raise RuntimeError(f"teacher worker emitted invalid JSON: {exc}") from exc
            remaining = deadline - time.monotonic()
            if remaining <= 0:
                raise TimeoutError("teacher worker read timed out")
            events = self.selector.select(remaining)
            if not events:
                raise TimeoutError("teacher worker read timed out")
            chunk = os.read(self.process.stdout.fileno(), 65536)
            if not chunk:
                code = self.process.poll()
                raise RuntimeError(f"teacher worker exited before response (code {code})")
            self.read_buffer.extend(chunk)

    def request(self, payload: dict[str, Any], n: int, budget_ms: int, timeout: float) -> Any:
        if self.process is None or self.process.stdin is None:
            raise RuntimeError("teacher worker is not running")
        message = dict(payload)
        message["n"] = n
        message["budget_ms"] = budget_ms
        self.process.stdin.write((canonical(message) + "\n").encode("utf-8"))
        self.process.stdin.flush()
        return self.read_json(min(self.response_timeout, timeout))

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
            process.wait(timeout=0.5)
        except subprocess.TimeoutExpired:
            try:
                process.terminate()
            except OSError:
                pass
            try:
                process.wait(timeout=0.5)
            except subprocess.TimeoutExpired:
                try:
                    process.kill()
                except OSError:
                    pass
                try:
                    process.wait(timeout=0.5)
                except subprocess.TimeoutExpired:
                    pass
        if self.selector is not None:
            self.selector.close()
        self.process = None


def teach(args: argparse.Namespace) -> int:
    output = Path(args.output).expanduser().resolve()
    manifest, rows = load_requests(output)
    rows_by_id = {row["id"]: row for row in rows}
    command = shlex.split(args.binary)
    if not command:
        raise SystemExit("--binary must not be empty")
    identity = binary_identity(command)
    config = {
        "binary": identity,
        "n": args.n,
        "budget_ms": args.budget_ms,
        "handshake_timeout_ms": args.handshake_timeout_ms,
        "response_timeout_ms": args.response_timeout_ms,
    }
    config_digest = sha256_bytes(canonical(config).encode())
    config_path = output / "teacher-config.json"
    if config_path.is_file():
        old_config = read_json(config_path)
        if old_config != config:
            raise SystemExit("teacher binary/config identity differs from prior cycles; use a new output")
    else:
        write_atomic(config_path, config)

    lessons = read_existing_lessons(output, rows_by_id)
    progress_path = output / "progress.json"
    started = time.time()
    deadline = time.monotonic() + max(0.0, float(args.seconds))
    counts = Counter()
    counts["already_completed"] = len(lessons)
    stop_reason = "exhausted"
    ready_record: Any = None
    worker: Worker | None = None
    try:
        if len(lessons) < len(rows) and time.monotonic() < deadline:
            worker = Worker(command, args.handshake_timeout_ms / 1000.0, args.response_timeout_ms / 1000.0)
            try:
                ready_record = worker.start()
            except Exception as exc:  # handshake failures are recorded and never fabricated as labels
                stop_reason = "handshake_error"
                counts["failed"] += 1
                ready_record = {"ready": False, "error": str(exc)}
            if ready_record and ready_record.get("ready") is True:
                for row in rows:
                    if row["id"] in lessons:
                        continue
                    remaining = deadline - time.monotonic()
                    if remaining <= 0:
                        stop_reason = "budget"
                        break
                    attempt = next_attempt_number(output, row["id"])
                    error_obj: BaseException | None = None
                    try:
                        response = worker.request(row["request"], args.n, args.budget_ms, remaining)
                        status = response_status(response)
                    except (TimeoutError, RuntimeError, OSError, BrokenPipeError) as exc:
                        error_obj = exc
                        response = {"status": "error", "error": str(exc)}
                        status = "error"
                    lesson = {
                        "id": row["id"],
                        "group": row["group"],
                        "split": row["split"],
                        "request": row["request"],
                        "response": response,
                        "status": status,
                        "attempt": attempt,
                        "recorded_at": time.time(),
                    }
                    write_atomic(output / "attempts" / row["id"] / f"{attempt:06d}.json", lesson)
                    write_atomic(output / "lessons" / f"{row['id']}.json", lesson)
                    if status in TERMINAL_STATUSES:
                        lessons[row["id"]] = lesson
                        counts["responses"] += 1
                        if status in LABEL_STATUSES:
                            counts["completed_labels"] += 1
                        if status == "refused":
                            counts["refused"] += 1
                    else:
                        counts["failed"] += 1
                        # A timeout or worker error may leave a partial line in
                        # flight.  Do not send another request into that
                        # protocol; the outer cycle receipt makes the failure
                        # resumable on the next invocation.
                        stop_reason = "response_timeout" if isinstance(error_obj, TimeoutError) else "worker_error"
                        break
                    if status not in EXPECTED_STATUSES:
                        counts["unknown_status"] += 1
                    # A response can consume most of the cycle.  The next
                    # iteration checks the cooperative deadline before writing.
                else:
                    stop_reason = "exhausted"
                if len(lessons) < len(rows) and stop_reason == "exhausted":
                    stop_reason = "budget"
        elif len(lessons) == len(rows):
            stop_reason = "exhausted"
        else:
            stop_reason = "budget"
    except KeyboardInterrupt:
        stop_reason = "interrupted"
    finally:
        if worker is not None:
            worker.close()

    pending = len(rows) - len(lessons)
    cumulative_counts = {
        "total": len(rows),
        "completed_responses": len(lessons),
        "completed_labels": sum(1 for lesson in lessons.values() if lesson.get("status") in LABEL_STATUSES),
        "refused": sum(1 for lesson in lessons.values() if lesson.get("status") == "refused"),
        "pending": pending,
    }
    progress = {
        "schema": "compiled-cycle-progress-v1",
        "manifest_sha256": sha256_file(output / "manifest.json"),
        "requests_sha256": manifest["requests_sha256"],
        "config_sha256": config_digest,
        "counts": cumulative_counts,
        "cycle_counts": dict(counts),
        "last_stop_reason": stop_reason,
        "updated_at": time.time(),
    }
    write_atomic(progress_path, progress)
    receipt = {
        "schema": "compiled-cycle-receipt-v1",
        "started_at": started,
        "finished_at": time.time(),
        "seconds": args.seconds,
        "stop_reason": stop_reason,
        "config": config,
        "manifest_sha256": progress["manifest_sha256"],
        "requests_sha256": progress["requests_sha256"],
        "ready": ready_record,
        "counts": progress["counts"],
        "cycle_counts": progress["cycle_counts"],
    }
    with (output / "cycles.jsonl").open("a", encoding="utf-8") as stream:
        stream.write(canonical(receipt) + "\n")
        stream.flush()
        os.fsync(stream.fileno())
    write_atomic(output / "cycle-receipt.json", receipt)
    print(json.dumps({"output": str(output), "stop_reason": stop_reason, "pending": pending, "completed_labels": progress["counts"]["completed_labels"]}))
    return 0


def parser() -> argparse.ArgumentParser:
    command = argparse.ArgumentParser(description=__doc__)
    subparsers = command.add_subparsers(dest="command", required=True)
    prep = subparsers.add_parser("prepare", help="normalize the old 144-game corpus")
    prep.add_argument("--source", required=True, help="source receipt directory")
    prep.add_argument("--output", required=True, help="calibration output directory")
    prep.set_defaults(function=prepare)
    run = subparsers.add_parser("teach", help="run one bounded persistent teacher cycle")
    run.add_argument("--output", required=True, help="prepared calibration output directory")
    run.add_argument("--binary", required=True, help="compiled teacher worker command")
    run.add_argument("--seconds", type=float, default=55.0)
    run.add_argument("--n", type=int, default=8)
    run.add_argument("--budget-ms", type=int, default=50)
    run.add_argument("--handshake-timeout-ms", type=int, default=5000)
    run.add_argument("--response-timeout-ms", type=int, default=10000)
    run.set_defaults(function=teach)
    return command


def main(argv: list[str] | None = None) -> int:
    args = parser().parse_args(argv)
    return args.function(args)


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except BrokenPipeError:
        raise SystemExit(1)
