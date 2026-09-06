#!/usr/bin/env python3
"""Small declared batches; always launch beneath packet/tools/run_capped.py."""
import argparse
import hashlib
import json
from pathlib import Path
import random
import time
from collections import Counter

from player import BINARY, HERE, INPUT_KEYS, decide
from rules import TILES, called, context, follows, legal_tiles, winner, trick_points, replay_record

PUBLIC_SEED = 420600  # independent of all hidden-deal seeds


def emit(value):
    print(json.dumps(value, separators=(",", ":")), flush=True)


def drive(hands, decl, bid, bidder, modes, args, name):
    remaining = [set(h) for h in hands]
    assert sorted(t for h in hands for t in h) == list(range(28))
    leader, record, points = bidder, [], [0, 0]
    all_decisions, trick_times, fallback_count = [], [], 0
    route_counts = Counter()
    begin = time.monotonic()
    emit({"event": "hand-start", "name": name, "hands": hands,
          "decl": decl, "bid": bid, "bidder": bidder, "modes": modes,
          "public_seed": PUBLIC_SEED})
    for ti in range(7):
        trick = []
        trick_begin = time.monotonic()
        for pos in range(4):
            seat = (leader+pos) % 4
            req = {"decl": decl, "bid": bid, "bidder": bidder, "seat": seat,
                   "hand": hands[seat], "plays": record[:], "seed": PUBLIC_SEED}
            # Referee knows full deal; this exact object is the complete worker
            # input. No hand seed or teammate hand crosses the boundary.
            mode = modes[seat]
            resp = decide(req, mode, 40 if mode == "phone" else args.n,
                          8 if mode == "phone" else args.n0, args.n1, args.budget_ms)
            expected_legal = legal_tiles(remaining[seat], trick, decl)
            assert resp["legal"] == expected_legal
            assert resp["leader"] == leader and resp["points"] == points
            assert resp["choice"] in expected_legal and not resp["over_budget"]
            assert resp["phases"][0]["status"] == "completed", "native/Python state conformance failed"
            fallback_count += "fallback" in resp["route"]
            route_counts[mode+":"+resp["route"]] += 1
            remaining[seat].remove(resp["choice"])
            record.extend([seat, resp["choice"]])
            trick.append((seat, resp["choice"]))
            item = {"event": "decision", "hand": name, "trick": ti+1,
                    "seat": seat, "request": req, "response": resp}
            emit(item)
            all_decisions.append(item)
        leader = winner(trick, decl)
        points[leader % 2] += trick_points(trick)
        elapsed = round((time.monotonic()-trick_begin)*1_000_000)
        trick_times.append(elapsed)
        assert elapsed < 60_000_000, "four-play trick exceeded one minute"
        emit({"event": "trick", "hand": name, "trick": ti+1,
              "winner": leader, "points": points[:], "elapsed_us": elapsed})
    assert sum(points) == 42 and not any(remaining)
    replayed, _, _, _ = replay_record(hands, record, decl, bidder)
    assert replayed == points
    result = {"event": "hand-result", "name": name, "modes": modes,
              "points": points, "made": points[bidder % 2] >= bid,
              "decl": decl, "bid": bid, "bidder": bidder, "plays": record,
              "decisions": 28, "fallbacks": fallback_count,
              "routes": dict(route_counts),
              "literal_phone_on_all_phone_turns": all(
                  d["response"]["route"] in ("phone", "forced")
                  for d in all_decisions if d["response"]["mode"] == "phone"),
              "candidate_team_success": {
                  str(team): (points[bidder % 2] >= bid) == (team == bidder % 2)
                  for team in {s % 2 for s, m in enumerate(modes) if m == "partner"}},
              "max_trick_us": max(trick_times), "trick_us": trick_times,
              "elapsed_us": round((time.monotonic()-begin)*1_000_000)}
    emit(result)
    return result


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("kind", choices=["roots", "hand"])
    p.add_argument("--ids", default="")
    p.add_argument("--modes", default="baseline,partner,phone")
    p.add_argument("--n", type=int, default=40)
    p.add_argument("--n0", type=int, default=8)
    p.add_argument("--n1", type=int, default=2)
    p.add_argument("--budget-ms", type=int, default=14000)
    p.add_argument("--deal-seed", type=int)
    p.add_argument("--decl", type=int, default=6)
    p.add_argument("--bid", type=int, default=30)
    p.add_argument("--bidder", type=int, default=0)
    p.add_argument("--lineups", default="phone:phone:phone:phone,partner:phone:partner:phone,phone:partner:phone:partner")
    args = p.parse_args()
    fixture = json.loads((HERE/"fixtures.json").read_text())
    emit({"event": "config", "args": vars(args), "public_seed": PUBLIC_SEED,
          "native_sha256": hashlib.sha256(BINARY.read_bytes()).hexdigest(),
          "phone_sha256": hashlib.sha256((HERE/"reference/phone/walt.wasm").read_bytes()).hexdigest()})
    if args.kind == "roots":
        selected = args.ids.split(",") if args.ids else [r["id"] for r in fixture["roots"]]
        found = [r for r in fixture["roots"] if r["id"] in selected]
        assert len(found) == len(selected), "unknown or repeated root id"
        for root in found:
            req = {k: v for k, v in root.items() if k in INPUT_KEYS}
            req["seed"] = PUBLIC_SEED
            if root["id"].startswith("g1-"):
                deal = fixture["full_deals"][0]
                pts, lead, remaining, trick = replay_record(deal["hands"], req["plays"], req["decl"], req["bidder"])
                assert req["hand"] == deal["hands"][req["seat"]]
                assert req["seat"] == (lead+len(trick)) % 4
            for mode in args.modes.split(","):
                resp = decide(req, mode, 40 if mode == "phone" else args.n,
                              8 if mode == "phone" else args.n0, args.n1, args.budget_ms)
                if root["id"].startswith("g1-"):
                    assert resp["points"] == pts and resp["leader"] == lead
                    assert resp["legal"] == legal_tiles(remaining[req["seat"]], trick, req["decl"])
                assert resp["phases"][0]["status"] == "completed", "native/Python state conformance failed"
                emit({"event": "root", "id": root["id"], "request": req, "response": resp})
    else:
        if args.deal_seed is None:
            source = fixture["full_deals"][0]
            hands = source["hands"]
            decl, bid, bidder = source["decl"], source["bid"], source["bidder"]
            name = source["id"]
        else:
            assert args.deal_seed in fixture["holdout"]["deal_seeds"]
            tiles = list(range(28))
            random.Random(args.deal_seed).shuffle(tiles)
            hands = [sorted(tiles[7*s:7*s+7]) for s in range(4)]
            decl, bid, bidder = args.decl, args.bid, args.bidder
            name = "holdout-" + str(args.deal_seed)
        for i, lineup in enumerate(args.lineups.split(",")):
            modes = lineup.split(":")
            assert len(modes) == 4
            drive(hands, decl, bid, bidder, modes, args, name+"-"+str(i))


if __name__ == "__main__":
    main()
