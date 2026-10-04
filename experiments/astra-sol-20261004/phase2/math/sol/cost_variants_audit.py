#!/usr/bin/env python3
"""Independent audit of prerecorded native certificate-cost inputs/results."""
from collections import Counter
from fractions import Fraction
import hashlib
import json
from pathlib import Path
import random
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT.parent.parent / "partnership"))
from rules import legal_tiles, winner, replay_record, information_state, context, follows

panel = ROOT / "results" / sys.argv[1]
plan = json.loads((panel / "plan.json").read_text())
summary = json.loads((panel / "summary.json").read_text())
binary = ROOT / "probe/target/release/walt-substitution-probe"
assert hashlib.sha256(binary.read_bytes()).hexdigest() == plan["binary_sha256"]
assert plan["cheap_n0"] in (1, 4, 8) and plan["target_n0"] == 8
assert plan["outer"] == 40 and plan["field_level"] in (0, 1)
assert [r["id"] for r in plan["rows"]] == list(range(100, 100+plan["independent_source_deals"]))
assert len({r["seed"] for r in plan["rows"]}) == plan["independent_source_deals"]

eligible = []
declarations = Counter()
depths = Counter()
actor_teams = Counter()
accepted_ids = []
same_choice = 0
totals = Counter()
all_results = []
for row in plan["rows"]:
    i = row["id"]
    rng = random.Random(680000 + i)
    deck = list(range(28)); rng.shuffle(deck)
    hands = [sorted(deck[7*s:7*s+7]) for s in range(4)]
    assert row["hands"] == hands and row["seed"] == 680000 + i
    bidder = i % 4
    decl = list(range(8)) + [9]
    decl = decl[i % 9]
    ply = (8, 12, 16, 20)[(i // 9) % 4]
    plays, trick, leader = [], [], bidder
    remaining = [set(h) for h in hands]
    for _ in range(ply):
        actor = (leader + len(trick)) % 4
        tile = rng.choice(legal_tiles(remaining[actor], trick, decl))
        remaining[actor].remove(tile); plays.extend([actor, tile]); trick.append((actor, tile))
        if len(trick) == 4:
            leader = winner(trick, decl); trick = []
    points, replay_leader, _, tail = replay_record(hands, plays, decl, bidder)
    assert replay_leader == leader and not tail and row["points"] == points
    actor = leader
    req = dict(decl=decl, bid=30, bidder=bidder, seat=actor, hand=hands[actor], plays=plays, seed=7042104)
    assert row["request"] == req and row["ply"] == ply
    state = information_state(req)
    valid = len(state["legal"]) > 1 and points[bidder % 2] < 30 and points[(bidder+1) % 2] <= 12
    assert row["eligible"] == valid
    file = panel / f"case-{i:04}.json"
    if not valid:
        assert not file.exists()
        continue
    eligible.append(i)
    receipt = json.loads(file.read_text())
    assert receipt["status"] == "completed" and receipt["id"] == i
    expected_input = dict(request=req, outer=40, cheap_n0=plan["cheap_n0"], target_n0=8, seconds=20,
                          target_first=bool(i % 2), field_level=plan["field_level"])
    assert receipt["input"] == expected_input
    r = receipt["result"]
    assert r["outer"] == 40 and r["cheap_n0"] == plan["cheap_n0"] and r["target_n0"] == 8
    assert r["field_level"] == plan["field_level"] and r["target_first"] == bool(i % 2)
    assert r["value_orientation"] == "focal team success counts"
    tiles = r["tiles"]
    assert tiles == state["legal"]
    assert all(len(r[k]) == len(tiles) for k in
               ("cheap_counts", "target_counts", "error_counts", "lower", "upper", "bad_scenario_flags"))
    errors = [sum(flags) for flags in r["bad_scenario_flags"]]
    assert errors == r["error_counts"]
    assert all(len(flags) == 40 and all(type(v) is bool for v in flags)
               for flags in r["bad_scenario_flags"])
    cheap, target = r["cheap_counts"], r["target_counts"]
    assert all(type(v) is int and 0 <= v <= 40 for v in cheap + target)
    lower = [max(0, v-e) for v, e in zip(cheap, errors)]
    upper = [min(40, v+e) for v, e in zip(cheap, errors)]
    assert lower == r["lower"] and upper == r["upper"]
    assert all(abs(a-b) <= e and lo <= b <= hi
               for a, b, e, lo, hi in zip(cheap, target, errors, lower, upper))
    c = max(range(len(tiles)), key=lambda j: (cheap[j], -tiles[j]))
    t = max(range(len(tiles)), key=lambda j: (target[j], -tiles[j]))
    certified = all(j == c or upper[j] < lower[c]
                    or (upper[j] <= lower[c] and tiles[c] < tiles[j])
                    for j in range(len(tiles)))
    assert r["cheap_choice"] == tiles[c] and r["target_choice"] == tiles[t]
    assert r["certified"] == certified
    if certified:
        assert c == t
        accepted_ids.append(i)
    same_choice += c == t
    # Residual worlds are stored in internal rotated seats. Check disjointness,
    # capacity, known actor hand, and public void compatibility independently.
    rotation = 1 if bidder % 2 == 0 else 0
    played = set(plays[1::2]); played_mask = sum(1 << tile for tile in played)
    voids = [set() for _ in range(4)]
    history_trick = []
    for seat, tile in zip(plays[::2], plays[1::2]):
        if history_trick:
            led = context(history_trick[0][1], decl)
            if not follows(tile, led, decl):
                voids[seat].update(t for t in range(28) if follows(t, led, decl))
        history_trick.append((seat, tile))
        if len(history_trick) == 4: history_trick = []
    assert len(r["worlds"]) == 40
    for world in r["worlds"]:
        assert len(world) == 4
        union = 0
        for internal, mask in enumerate(world):
            absolute = (internal-rotation) % 4
            assert type(mask) is int and mask >= 0 and mask >> 28 == 0
            assert mask & union == 0 and mask & played_mask == 0
            union |= mask
            assert bin(mask).count("1") == 7 - ply // 4
            assert not mask & sum(1 << tile for tile in voids[absolute])
            if absolute == actor:
                assert mask == sum(1 << tile for tile in remaining[actor])
        assert union == ((1 << 28)-1) ^ played_mask
    for k in ("cheap_us", "certificate_us", "target_us", "sampling_us"):
        totals[k] += r[k]
    totals["cold_fallback_us"] += r["cheap_us"] + r["certificate_us"] + (0 if certified else r["target_us"])
    totals["certified_method_us"] += (r["cheap_us"] + r["certificate_us"]) if certified else 0
    totals["certified_target_us"] += r["target_us"] if certified else 0
    totals["certificate_visits"] += r["certificate_work"]["visits"]
    totals["certificate_target_queries"] += r["certificate_work"]["target_distinct_queries"]
    declarations[decl] += 1; depths[ply] += 1
    actor_teams["declaring" if actor % 2 == bidder % 2 else "defending"] += 1
    all_results.append((i, receipt))

assert len(eligible) == len(all_results) == summary["eligible"] == summary["completed"]
assert len(accepted_ids) == summary["certified"]
assert same_choice == summary["same_choice"]
assert summary["statuses"] == {"completed":len(eligible), "excluded_before_probe":len(plan["rows"])-len(eligible)}
assert totals["cheap_us"] == summary["total_cheap_us"]
assert totals["certificate_us"] == summary["total_certificate_us"]
assert totals["target_us"] == summary["total_target_us"]
assert totals["certified_method_us"] == summary["certified_cheap_plus_certificate_us"]
assert totals["certified_target_us"] == summary["certified_target_us"]
assert totals["cold_fallback_us"] == totals["cheap_us"] + totals["certificate_us"] + totals["target_us"] - totals["certified_target_us"]

# Repeat the one accepted input to check deterministic scalar correctness.
accepted = next(receipt for i, receipt in all_results if i in accepted_ids)
run = subprocess.run([str(binary)], input=json.dumps(accepted["input"])+"\n",
                     text=True, capture_output=True, check=True, timeout=5)
fresh = json.loads(run.stdout)
for k in ("tiles", "cheap_counts", "target_counts", "error_counts", "lower", "upper",
          "bad_scenario_flags", "worlds", "cheap_choice", "target_choice", "certified"):
    assert fresh[k] == accepted["result"][k]
# Equal-field control must collapse all first-disagreement flags to zero.
equal_input = dict(accepted["input"], cheap_n0=8)
run = subprocess.run([str(binary)], input=json.dumps(equal_input)+"\n",
                     text=True, capture_output=True, check=True, timeout=5)
equal = json.loads(run.stdout)
assert not any(equal["error_counts"])
assert equal["cheap_counts"] == equal["target_counts"]
assert equal["certified"]

ratio = Fraction(totals["cold_fallback_us"], totals["target_us"])
print(json.dumps(dict(panel=panel.name, planned_source_deals=len(plan["rows"]), audited_eligible=len(eligible),
    excluded_before_probe=len(plan["rows"])-len(eligible), eligible_by_declaration=dict(declarations),
    eligible_by_prefix_plies=dict(depths), eligible_by_actor_team=dict(actor_teams),
    accepted_ids=accepted_ids, same_choice=same_choice, all_bounds_and_flags_audited=True,
    cold_fallback_us=totals["cold_fallback_us"], fresh_target_us=totals["target_us"],
    cold_fallback_ratio_exact=str(ratio), cost_totals=dict(totals),
    acceptance_reproduced=True, equal_field_zero_disagreement_control=True,
    interpretation="This certificate + fresh fallback is slower on the observed conditional panel; not a general impossibility or strength result."), indent=2))
