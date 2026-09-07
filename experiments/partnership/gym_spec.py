"""Declarative, parameterized gym collections; generated exercises are artifacts.

This module owns specification validation, input identity, and orchestration.
The existing gym owns coordinate extraction, exact evaluation, auditing, and
publication. Selection arguments never enter an evaluation or pupil request.
"""
import argparse
import copy
from fractions import Fraction
from itertools import islice
import json
from pathlib import Path
import time
from types import SimpleNamespace

import gym


def fields(obj, names, where):
    if not isinstance(obj, dict) or set(obj) != set(names.split()):
        raise ValueError(f"{where}: expected fields {names}")


def integer(value, low, high, where):
    if type(value) is not int or not low <= value <= high:
        raise ValueError(f"{where}: expected integer {low}..{high}")


def probability(value, where, *, positive=False):
    if not isinstance(value, str):
        raise ValueError(f"{where}: use an exact rational string")
    try:
        p = Fraction(value)
    except (ValueError, ZeroDivisionError) as error:
        raise ValueError(f"{where}: invalid rational") from error
    if not 0 <= p <= 1 or (positive and p == 0):
        raise ValueError(f"{where}: outside probability range")
    return str(p)


def arguments(value):
    """Strict schema; unknown options must never be silently ignored."""
    a = copy.deepcopy(value)
    fields(a, "source domain query evaluation selection", "arguments")
    fields(a["source"], "paths", "source")
    paths = a["source"]["paths"]
    if not isinstance(paths, list) or not paths or any(not isinstance(p, str) or not p for p in paths):
        raise ValueError("source.paths: expected nonempty path list")
    d = a["domain"]
    fields(d, "tricks limit seed", "domain")
    if not isinstance(d["tricks"], list) or len(d["tricks"]) != 2:
        raise ValueError("domain.tricks: expected [first, last]")
    for t in d["tricks"]:
        integer(t, 5, 6, "domain.tricks")
    if d["tricks"][0] > d["tricks"][1]:
        raise ValueError("domain.tricks: reversed bounds")
    integer(d["limit"], 1, 10**9, "domain.limit")
    integer(d["seed"], 0, 2**64 - 1, "domain.seed")
    q = a["query"]
    fields(q, "name source min_presence", "query")
    if not isinstance(q["name"], str) or not q["name"] or not isinstance(q["source"], str) or not q["source"]:
        raise ValueError("query: name and source must be nonempty strings")
    q["min_presence"] = probability(q["min_presence"], "query.min_presence", positive=True)
    e = a["evaluation"]
    fields(e, "contract max_worlds partner_worlds", "evaluation")
    if e["contract"] != "partnership-gym-v1":
        raise ValueError("unsupported evaluation contract")
    integer(e["max_worlds"], 1, 10000, "evaluation.max_worlds")
    integer(e["partner_worlds"], 1, 640, "evaluation.partner_worlds")
    s = a["selection"]
    fields(s, "criterion side min_spread min_mistake max_optimal certain", "selection")
    if s["criterion"] not in ("query", "outcome") or s["side"] not in ("both", "declaring", "defending"):
        raise ValueError("unsupported selection criterion or side")
    for key in ("min_spread", "min_mistake"):
        s[key] = probability(s[key], "selection." + key)
    if s["max_optimal"] is not None:
        integer(s["max_optimal"], 1, 7, "selection.max_optimal")
    if type(s["certain"]) is not bool:
        raise ValueError("selection.certain: expected boolean")
    return a


def override(a, expression):
    """Override an existing leaf: exact rationals stay strings; no eval()."""
    path, sep, raw = expression.partition("=")
    if not sep:
        raise ValueError("argument override needs path=value")
    keys, node = path.split("."), a
    for key in keys[:-1]:
        if not isinstance(node, dict) or key not in node:
            raise ValueError("unknown argument path: " + path)
        node = node[key]
    key = keys[-1]
    if not isinstance(node, dict) or key not in node or isinstance(node[key], dict):
        raise ValueError("unknown or non-leaf argument path: " + path)
    if isinstance(node[key], str):
        node[key] = raw
    else:
        try:
            node[key] = json.loads(raw)
        except json.JSONDecodeError as error:
            raise ValueError("argument needs a JSON value: " + path) from error


def load(path, overrides):
    spec = json.loads(path.read_text())
    fields(spec, "schema name arguments reference", "specification")
    if spec["schema"] != "gym-spec-v1" or not isinstance(spec["name"], str) or not spec["name"]:
        raise ValueError("invalid gym specification")
    base = arguments(spec["arguments"])
    ref = spec["reference"]
    if ref is not None:
        fields(ref, "arguments_sha256 source_sha256 count coordinates_sha256 answers_sha256", "reference")
        integer(ref["count"], 0, 10**9, "reference.count")
        for key in ("arguments_sha256", "source_sha256", "coordinates_sha256", "answers_sha256"):
            h = ref[key]
            if not isinstance(h, str) or len(h) != 64 or any(c not in "0123456789abcdef" for c in h):
                raise ValueError("invalid reference digest: " + key)
        if gym.digest(base) != ref["arguments_sha256"]:
            raise ValueError("specification arguments changed under a frozen reference; update or remove reference")
    resolved = copy.deepcopy(base)
    for expression in overrides:
        override(resolved, expression)
    resolved = arguments(resolved)
    # The reference is a regression expectation, never a discovery filter.
    expected = ref if resolved == base else None
    return spec, resolved, expected


def inputs(a):
    """Fingerprint source records independently of checkout location."""
    paths = [(gym.ROOT / p).resolve() for p in a["source"]["paths"]]
    rows = []
    for i, path in enumerate(paths):
        if not path.is_dir():
            raise ValueError("missing source directory: " + str(path))
        for result in sorted(path.rglob("result.json")):
            checkpoint = result.with_name("checkpoint.json")
            if checkpoint.exists():
                rows.append(dict(source=i, path=str(result.relative_to(path)),
                                 result=gym.file_hash(result), checkpoint=gym.file_hash(checkpoint)))
    if not rows:
        raise ValueError("source has no completed campaign records")
    return paths, gym.digest(rows)


def coordinates(a, paths):
    first, last = a["domain"]["tricks"]
    for item in islice(gym.positions(paths, first, last), a["domain"]["limit"]):
        item["request"]["seed"] = a["domain"]["seed"]
        item["id"] = gym.digest(item["request"])[:20]
        yield item


def accepts(case, selection):
    p = gym.outcome_profile(case["key"])
    return (p["strict"]
            and Fraction(p["spread"]) >= Fraction(selection["min_spread"])
            and Fraction(p["nearest_mistake"]) >= Fraction(selection["min_mistake"])
            and (selection["max_optimal"] is None or p["optimal_count"] <= selection["max_optimal"])
            and (not selection["certain"] or p["certain_success_failure_swing"]))


def result_identity(cases):
    ordered = sorted(cases, key=lambda c: c["id"])
    return dict(count=len(ordered), coordinates_sha256=gym.digest([c["id"] for c in ordered]),
                answers_sha256=gym.digest([[c["id"], c["key"]] for c in ordered]))


def add_parser(sub):
    p = sub.add_parser("generate", help="Generate a repeatable collection from one parameterized specification")
    p.add_argument("--spec", type=Path, required=True)
    p.add_argument("--output", type=Path, required=True, help="Local evaluation cache and generated collection directory")
    p.add_argument("--set", dest="overrides", action="append", default=[], metavar="PATH=VALUE")
    p.add_argument("--workers", type=int, default=10)
    p.add_argument("--seconds", type=float, default=240)
    p.add_argument("--case-seconds", type=float, default=45)


def generate(args):
    if not (1 <= args.workers <= 10 and 20 <= args.seconds <= 270 and 14 <= args.case_seconds < args.seconds):
        raise ValueError("workers 1..10, seconds 20..270, case-seconds >=14 and <seconds; use the watchdog")
    started = time.monotonic()
    spec, a, expected = load(args.spec, args.overrides)
    paths, source_hash = inputs(a)
    if expected is not None and source_hash != expected["source_sha256"]:
        raise ValueError("reference source changed; create a new specification/reference for the expanded corpus")
    items = list(coordinates(a, paths))
    identity = dict(schema="gym-generation-v1", source_sha256=source_hash,
                    engine=gym.file_hash(gym.ENGINE), gym=gym.file_hash(gym.__file__),
                    generator=gym.file_hash(__file__), rules=gym.file_hash(gym.HERE / "rules.py"),
                    domain=a["domain"], query=a["query"], evaluation=a["evaluation"],
                    coordinates_sha256=gym.digest(items))
    evaluation_id = gym.digest(identity)
    view_id = gym.digest(dict(evaluation=evaluation_id, selection=a["selection"]))
    cache = args.output / "evaluations" / evaluation_id
    view = args.output / "collections" / view_id
    receipt_path = args.output / "latest.json"
    resolved = dict(schema="gym-resolved-spec-v1", name=spec["name"], arguments=a,
                    specification_sha256=gym.digest(spec), reference=expected,
                    source_sha256=source_hash, evaluation_id=evaluation_id, collection_id=view_id)
    # One coordinator owns the cache and its views. The inner gym locks protect
    # direct low-level access too; separate output roots can run independently.
    with gym.run_lock(args.output):
        receipt = dict(schema="gym-generation-receipt-v1", complete=False,
                       resolved=resolved, evaluation=str(cache.resolve()), gallery=None)
        gym.atomic(receipt_path, receipt)
        print(gym.canonical(dict(specification=spec["name"], reference_applicable=expected is not None,
                                planned=len(items), evaluation_id=evaluation_id, collection_id=view_id)), flush=True)
        first, last = a["domain"]["tricks"]
        d = SimpleNamespace(source=paths, min_trick=first, max_trick=last, limit=a["domain"]["limit"],
                output=cache, max_worlds=a["evaluation"]["max_worlds"],
                partner_worlds=a["evaluation"]["partner_worlds"], min_presence=a["query"]["min_presence"],
                case_seconds=args.case_seconds, seconds=args.seconds, workers=args.workers)
        gym.discover(d, coordinates=items, query_source=a["query"]["source"],
                     query_name=a["query"]["name"], identity=identity)
        saved = {p.stem for p in (cache / "items").glob("*.json")}
        pending = [i["id"] for i in items if i["id"] not in saved]
        receipt.update(saved=len(saved), pending=len(pending))
        if pending:
            gym.atomic(receipt_path, receipt)
            print(gym.canonical(dict(complete=False, pending=len(pending), resume="Repeat the same command")), flush=True)
            return
        if inputs(a)[1] != source_hash:
            raise ValueError("source changed during evaluation; rerun against a stable corpus")
        # Do not present a stale collection as a successful current generation.
        gym.select(SimpleNamespace(source=cache, output=view, criterion=a["selection"]["criterion"],
                                   side=a["selection"]["side"], all=True, each=1),
                   predicate=lambda c: accepts(c, a["selection"]), allow_empty=True)
        result = result_identity([c for _, c in gym.gallery(view)])
        if expected is not None and any(result[k] != expected[k] for k in result):
            receipt.update(result=result, error="reference result mismatch")
            gym.atomic(receipt_path, receipt)
            raise ValueError("generated collection differs from the frozen reference; see latest.json")
        gym.atomic(view / "specification.json", resolved)
        receipt.update(complete=True, gallery=str(view.resolve()), result=result,
                       reference_verified=expected is not None,
                       elapsed_seconds=round(time.monotonic() - started, 6))
        gym.atomic(receipt_path, receipt)
        print(gym.canonical(receipt), flush=True)
