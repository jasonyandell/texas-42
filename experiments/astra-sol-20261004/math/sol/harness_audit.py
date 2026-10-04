#!/usr/bin/env python3
"""Independent bounded smoke audit of production ABI, profiles, and requests."""
import hashlib
import json
from pathlib import Path
import random
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tools"))
import arena

expected_hash = "40d7a1eea658627f7bab37d3c1888ad3bf00216fb23a5d2ceeb7d2d1f4219119"
assert arena.sha(arena.PHONE) == expected_hash
manifest = json.loads((ROOT / "reference/production-phone/manifest.json").read_text())
assert manifest["wasm_sha256"] == expected_hash
assert manifest["source_commit"] == "cb1ef3b23072e4c268f31f625f2b61d5facc1929"

worker = arena.Worker(arena.PHONE)
started = time.monotonic()
checked = 0
routes = {}
try:
    for decl in list(range(8)) + [9]:
        for bidder in range(4):
            tiles = list(range(28))
            random.Random(920004 + decl * 4 + bidder).shuffle(tiles)
            hands = [sorted(tiles[7*s:7*s+7]) for s in range(4)]
            opening = arena.make_request(hands, bidder, bidder, [], decl)
            assert arena.profile(opening, True) == dict(request=opening, worlds=160,
                                                        partner=False, budget_ms=20000)
            remaining = [set(h) for h in hands]
            leader, plays, points = bidder, [], [0, 0]
            # Build a legal public prefix through six complete tricks using
            # the independent referee; no Walt output constructs this prefix.
            for _ in range(6):
                trick = []
                for offset in range(4):
                    seat = (leader + offset) % 4
                    tile = arena.legal_tiles(remaining[seat], trick, decl)[0]
                    remaining[seat].remove(tile)
                    plays.extend([seat, tile])
                    trick.append((seat, tile))
                leader = arena.winner(trick, decl)
                points[leader % 2] += arena.trick_points(trick)
            trick = []
            for offset in range(4):
                seat = (leader + offset) % 4
                req = arena.make_request(hands, seat, bidder, plays, decl)
                assert set(req) == {"decl", "bid", "bidder", "seat", "hand", "plays", "seed"}
                assert req["seed"] == arena.PUBLIC_POLICY_SEED
                # Syntactic information boundary: arbitrary changes to other
                # hands cannot change the generated actor request.
                altered = [list(h) for h in hands]
                others = [s for s in range(4) if s != seat]
                altered[others[0]], altered[others[1]] = altered[others[1]], altered[others[0]]
                assert arena.make_request(altered, seat, bidder, plays, decl) == req
                state = arena.information_state(req)
                call = arena.profile(req, bool(checked % 2))
                assert (call["worlds"], call["budget_ms"]) == (40, 14000)
                response, timing = worker.call(call, started + 40)
                legal = arena.legal_tiles(remaining[seat], trick, decl)
                assert len(legal) == 1 and type(response["choice"]) is int
                assert response["choice"] == legal[0]
                assert response["legal"] == state["legal"] == legal
                assert response["leader"] == state["leader"] == leader
                assert response["points"] == state["points"] == points
                assert response["route"] == "forced" and not timing["interrupted"]
                routes[response["route"]] = routes.get(response["route"], 0) + 1
                tile = response["choice"]
                remaining[seat].remove(tile)
                plays.extend([seat, tile])
                trick.append((seat, tile))
                checked += 1
            leader = arena.winner(trick, decl)
            points[leader % 2] += arena.trick_points(trick)
            replay_points, _, replay_remaining, tail = arena.replay_record(hands, plays, decl, bidder)
            assert points == replay_points and sum(points) == 42
            assert not tail and all(not hand for hand in replay_remaining)
finally:
    worker.close()

print(json.dumps(dict(production_wasm_sha256=expected_hash,
    checked_decisions=checked, configurations=36, declarations=9, bidder_rotations=4,
    routes=routes, fresh_instances=True, policy_information_boundary="own original hand and public record",
    limitation="forced suffix ABI/profile checks; no playing-strength or nonforced solve claim"), indent=2))
