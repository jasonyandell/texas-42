#!/usr/bin/env python3
"""Independently replay and compare native full-game speed receipts.

Referee rules follow experiments/partnership/rules.py (partnership-launch
95e90444). This script imports no solver and never trusts the receipt's legal
sets, score, or action-value digest as its own oracle.
"""

import argparse
import json
import math
import sys
from fractions import Fraction
from pathlib import Path

TILES = [(hi, lo) for hi in range(7) for lo in range(hi + 1)]
G1 = [[0, 4, 9, 14, 22, 24, 27], [2, 6, 13, 16, 19, 21, 26],
      [1, 5, 11, 12, 17, 23, 25], [3, 7, 8, 10, 15, 18, 20]]
INPUT_KEYS = {"decl", "bid", "bidder", "seat", "hand", "plays", "seed"}
MASK64 = (1 << 64) - 1


def require(test, message):
    if not test:
        raise ValueError(message)


def called(tile, decl):
    hi, lo = TILES[tile]
    return decl in (hi, lo) if decl < 7 else decl == 7 and hi == lo


def context(tile, decl):
    return "trump" if called(tile, decl) else TILES[tile][0]


def follows(tile, led, decl):
    return called(tile, decl) if led == "trump" else not called(tile, decl) and led in TILES[tile]


def legal(hand, trick, decl):
    if not trick:
        return sorted(hand)
    following = [t for t in hand if follows(t, context(trick[0][1], decl), decl)]
    return sorted(following or hand)


def winner(trick, decl):
    led = context(trick[0][1], decl)

    def strength(play):
        tile = play[1]
        hi, lo = TILES[tile]
        tier = 2 if called(tile, decl) else 1 if follows(tile, led, decl) else 0
        rank = hi if hi == lo and decl == 7 else 12 if hi == lo else hi + lo
        return tier, rank

    return max(trick, key=strength)[0]


def trick_points(trick):
    return 1 + sum(sum(TILES[t]) if sum(TILES[t]) in (5, 10) else 0 for _, t in trick)


def splitmix(state):
    state = (state + 0x9E3779B97F4A7C15) & MASK64
    z = state
    z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & MASK64
    z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & MASK64
    return state, z ^ (z >> 31)


def expected_hands(game):
    if game["fixture"] == "g1":
        require(game["deal_seed"] is None, "G1 has unexpected deal seed")
        return G1
    require(game["fixture"] == "shuffle", "unknown fixture")
    seed = game["deal_seed"]
    require(type(seed) is int and 0 <= seed <= MASK64, "invalid shuffle seed")
    tiles = list(range(28))
    for i in range(27, 0, -1):
        seed, value = splitmix(seed)
        j = value % (i + 1)
        tiles[i], tiles[j] = tiles[j], tiles[i]
    return [sorted(tiles[s * 7:s * 7 + 7]) for s in range(4)]


def digest_update(h, payload):
    # Rust serde_json Map uses lexical key order in this runner. The hash is
    # diagnostic FNV-1a, not a cryptographic content identifier.
    for byte in json.dumps(payload, sort_keys=True, separators=(",", ":")).encode():
        h = ((h ^ byte) * 0x100000001B3) & MASK64
    return h


def parse_wire(text):
    require(isinstance(text, str) and text.endswith("\n"), "malformed native wire request")
    lines = text.splitlines()
    require(lines and lines[0] in ("partner", "all-l1"), "native wire has wrong mode")
    fields = {}
    for line in lines[1:]:
        words = line.split()
        require(words and words[0] not in fields, "empty or duplicate native wire field")
        require(all(word.isdigit() for word in words[1:]), "native wire has noninteger field")
        fields[words[0]] = [int(word) for word in words[1:]]
    require(set(fields) == INPUT_KEYS | {"n", "n0", "n1", "budget_ms",
                                         "inner_belief", "selection", "modeled_selection"},
            "native wire fields differ from declared input and settings")
    return lines[0], fields


def exact_options(response, legal_set, declaring_turn):
    rows = response.get("options")
    require(isinstance(rows, list), "missing action options")
    require(len(rows) == len(legal_set), "action option count disagrees with legality")
    parsed = []
    for row in rows:
        require(isinstance(row, list) and len(row) == 3, "malformed action option")
        tile, numer, denom = row
        require(type(tile) is int and type(numer) is str and type(denom) is str,
                "option must carry tile and exact rational strings")
        require(numer.lstrip("-").isdigit() and denom.isdigit(), "noninteger rational")
        n, d = int(numer), int(denom)
        require(d > 0 and math.gcd(n, d) == 1 and 0 <= n <= d, "invalid success fraction")
        parsed.append((tile, Fraction(n, d)))
    require([tile for tile, _ in parsed] == legal_set, "options are not the ordered legal set")
    optimum = (max if declaring_turn else min)(value for _, value in parsed)
    expected = next(tile for tile, value in parsed if value == optimum)
    require(response["choice"] == expected, "choice is not the exact-vector best response")


def audit_game(game):
    require(game.get("status") == "complete" and game.get("error") is None,
            "game has refusal or error")
    require(game.get("profile") in ("partner", "all-l1"), "unknown field profile")
    require(game.get("cache") in ("cold", "carry"), "unknown cache setting")
    require(game.get("selection") == game.get("modeled_selection") == "fixed",
            "comparison requires fixed search")
    require(game.get("inner_belief") == "voidless", "comparison requires voidless inner belief")
    require(game.get("samples") == {"root": 40, "l0": 8, "l1": 2},
            "comparison requires current L2 40/8/2 settings")
    require(all(type(v) is int for v in game["samples"].values()),
            "sample counts must be integers")
    require(game.get("decl") in (*range(8), 9) and 30 <= game.get("bid", 0) <= 42,
            "invalid contract")
    require(type(game["decl"]) is int and type(game["bid"]) is int
            and type(game["public_seed"]) is int, "contract or public seed has wrong type")
    require(type(game.get("budget_ms_per_call")) is int
            and 0 <= game["budget_ms_per_call"] <= 20_000,
            "invalid native per-call deadline")
    bidder = game["bidder"]
    require(type(bidder) is int and bidder in range(4), "invalid bidder")
    hands = expected_hands(game)
    require(game["hands_referee_only"] == hands, "referee deal differs from fixture")
    require(all(type(tile) is int for hand in game["hands_referee_only"] for tile in hand),
            "referee deal has wrong tile type")
    require(sorted(t for hand in hands for t in hand) == list(range(28)), "deal is not a partition")
    decisions = game["decisions"]
    require(len(decisions) == 28, "game did not finish 28 plays")
    remaining = [set(h) for h in hands]
    record = []
    score = [0, 0]
    leader = bidder
    h = 0xCBF29CE484222325
    trick_rows = []
    for trick_no in range(1, 8):
        trick = []
        calls = []
        for offset in range(4):
            play_no = len(record) // 2 + 1
            seat = (leader + offset) % 4
            decision = decisions[play_no - 1]
            request = decision["request"]
            response = decision["response"]
            require(set(request) == INPUT_KEYS, f"play {play_no}: request includes hidden or extra data")
            require(all(type(request[k]) is int for k in ("decl", "bid", "bidder", "seat", "seed"))
                    and all(type(t) is int for t in request["hand"] + request["plays"]),
                    f"play {play_no}: request has wrong input type")
            require(request == {"decl": game["decl"], "bid": game["bid"],
                    "bidder": bidder, "seat": seat, "hand": hands[seat],
                    "plays": record, "seed": game["public_seed"]},
                    f"play {play_no}: request is not exactly own/public state")
            if "wire_request" in decision or "solver_settings" in decision:
                require("wire_request" in decision and "solver_settings" in decision,
                        f"play {play_no}: native wire/settings metadata incomplete")
                mode, fields = parse_wire(decision["wire_request"])
                expected_settings = {"mode": game["profile"], "n": game["samples"]["root"],
                                     "n0": game["samples"]["l0"], "n1": game["samples"]["l1"],
                                     "budget_ms": game["budget_ms_per_call"],
                                     "inner_belief": 0, "selection": 0,
                                     "modeled_selection": 0}
                require(decision["solver_settings"] == expected_settings,
                        f"play {play_no}: solver setting metadata changed")
                require(mode == game["profile"] and all(fields[key] ==
                        (request[key] if key in ("hand", "plays") else [request[key]])
                        for key in INPUT_KEYS)
                        and all(fields[key] == [expected_settings[key]] for key in
                                ("n", "n0", "n1", "budget_ms", "inner_belief",
                                 "selection", "modeled_selection")),
                        f"play {play_no}: actual native wire differs from lawful request/settings")
            choices = legal(remaining[seat], trick, game["decl"])
            require(decision["play"] == play_no and decision["trick"] == trick_no
                    and decision["seat"] == seat, f"play {play_no}: position metadata mismatch")
            require(decision["legal"] == choices and decision["forced"] == (len(choices) == 1),
                    f"play {play_no}: local legality mismatch")
            require(response["legal"] == choices and response["leader"] == leader
                    and response["points"] == score and response["trick"] == trick_no,
                    f"play {play_no}: solver state mismatch")
            require(response["selection"] == response["modeled_selection"] == "fixed"
                    and response["inner_belief"] == "voidless",
                    f"play {play_no}: actual solver policy changed")
            require(response["outer_worlds"] == game["samples"]["root"]
                    and response["outer_draw_attempts"] >= response["outer_worlds"],
                    f"play {play_no}: fixed outer sampler did not complete its bundle")
            require(len(response["pi_calls_by_level"]) == 2
                    and len(response["inner_worlds_by_level"]) == 2
                    and all(type(n) is int and n >= 0 for n in
                            response["pi_calls_by_level"] + response["inner_worlds_by_level"]),
                    f"play {play_no}: modeled-work counters malformed")
            tile = decision["choice"]
            require(type(tile) is int and tile in choices and response["choice"] == tile,
                    f"play {play_no}: illegal or inconsistent choice")
            exact_options(response, choices, seat % 2 == bidder % 2)
            h = digest_update(h, {"play": play_no, "seat": seat, "choice": tile,
                                  "options": response["options"]})
            calls.append(decision)
            remaining[seat].remove(tile)
            record.extend([seat, tile])
            trick.append((seat, tile))
        leader = winner(trick, game["decl"])
        score[leader % 2] += trick_points(trick)
        trick_rows.append({"trick": trick_no, "call_wall_us": sum(d["elapsed_us"] for d in calls),
                           "solver_us": sum(d["response"]["solver_us"] for d in calls),
                           "nodes": sum(d["response"]["nodes"] for d in calls),
                           "pi_calls_l0": sum(d["response"]["pi_calls_by_level"][0] for d in calls),
                           "pi_calls_l1": sum(d["response"]["pi_calls_by_level"][1] for d in calls),
                           "inner_worlds_l0": sum(d["response"]["inner_worlds_by_level"][0] for d in calls),
                           "inner_worlds_l1": sum(d["response"]["inner_worlds_by_level"][1] for d in calls),
                           "outer_worlds": sum(d["response"]["outer_worlds"] for d in calls),
                           "points_after": score.copy(), "winner": leader})
    require(record == game["plays"] and score == game["points"], "final history or score mismatch")
    require(sum(score) == 42 and all(not hand for hand in remaining), "game not physically complete")
    require(game["made"] == (score[bidder % 2] >= game["bid"]), "make/set mismatch")
    digest = f"{h:016x}"
    require(game["choice_value_digest_fnv1a64"] == digest, "choice/value digest mismatch")
    call_wall = sum(row["call_wall_us"] for row in trick_rows)
    full_wall = game["full_game_elapsed_us"]
    require(full_wall >= call_wall, "full game wall shorter than calls")
    return {"status": "verified", "fixture": game["fixture"], "profile": game["profile"],
            "cache": game["cache"], "deal_seed": game["deal_seed"],
            "decl": game["decl"], "bid": game["bid"], "bidder": bidder,
            "public_seed": game["public_seed"],
            "samples": game["samples"], "selection": game["selection"],
            "modeled_selection": game["modeled_selection"],
            "inner_belief": game["inner_belief"],
            "budget_ms_per_call": game["budget_ms_per_call"],
            "choices": [d["choice"] for d in decisions],
            "value_digest": digest, "points": score, "made": game["made"],
            "full_game_elapsed_us": full_wall,
            "fixture_preparation_us": game["fixture_preparation_us"],
            "play_elapsed_us": game["play_elapsed_us"],
            "call_wall_us": call_wall,
            "other_game_wall_us": full_wall - call_wall,
            "nodes": sum(row["nodes"] for row in trick_rows),
            "pi_calls_l0": sum(row["pi_calls_l0"] for row in trick_rows),
            "pi_calls_l1": sum(row["pi_calls_l1"] for row in trick_rows),
            "tricks": trick_rows,
            "slowest_moves": sorted(({"play":d["play"], "trick":d["trick"], "seat":d["seat"],
                                      "elapsed_us":d["elapsed_us"],
                                      "nodes":d["response"]["nodes"]} for d in decisions),
                                    key=lambda row: row["elapsed_us"], reverse=True)[:5]}


def audit_receipt(path):
    receipt = json.loads(Path(path).read_text())
    require(receipt.get("schema") == "walt-full-game-speed-v1" and receipt.get("complete") is True,
            "not a complete speed receipt")
    games = receipt.get("games")
    require(isinstance(games, list) and games, "receipt has no games")
    require([game.get("repetition") for game in games] == list(range(len(games))),
            "repetition sequence is not contiguous from zero")
    rows = [audit_game(game) for game in games]
    return receipt, rows


def compare(reference_receipt, candidate_receipt, reference, candidate):
    require(len(reference) == len(candidate), "game count differs")
    comparisons = []
    for i, (a, b) in enumerate(zip(reference, candidate)):
        identity = ("fixture", "deal_seed", "decl", "bid", "bidder", "public_seed",
                    "profile", "samples", "selection", "modeled_selection",
                    "inner_belief", "budget_ms_per_call")
        require(all(a[key] == b[key] for key in identity), f"game {i}: player or fixture differs")
        require(a["points"] == b["points"], f"game {i}: result differs")
        choices_equal = a["choices"] == b["choices"]
        value_rows_a = [d["response"]["options"] for d in reference_receipt["games"][i]["decisions"]]
        value_rows_b = [d["response"]["options"] for d in candidate_receipt["games"][i]["decisions"]]
        values_equal = value_rows_a == value_rows_b
        require((values_equal and choices_equal) ==
                (a["value_digest"] == b["value_digest"]),
                f"game {i}: exact choices/vectors and diagnostic digest disagree")
        comparisons.append({"game": i, "choices_equal": choices_equal,
                            "exact_values_equal": values_equal,
                            "reference_digest": a["value_digest"],
                            "candidate_digest": b["value_digest"],
                            "reference_full_game_us": a["full_game_elapsed_us"],
                            "candidate_full_game_us": b["full_game_elapsed_us"],
                            "speedup": a["full_game_elapsed_us"] / b["full_game_elapsed_us"]})
    require(all(r["choices_equal"] and r["exact_values_equal"] for r in comparisons),
            "candidate choices or exact rational values differ")
    return comparisons


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("reference", type=Path)
    parser.add_argument("--compare", type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    try:
        reference_receipt, reference = audit_receipt(args.reference)
        result = {"schema": "walt-full-game-audit-v1", "status": "verified",
                  "scope": "direct native requested search; no outer L1 reserve or fallback",
                  "reference": str(args.reference), "games": reference}
        if args.compare:
            candidate_receipt, candidate = audit_receipt(args.compare)
            result["candidate"] = str(args.compare)
            result["candidate_games"] = candidate
            result["comparisons"] = compare(reference_receipt, candidate_receipt,
                                            reference, candidate)
    except (OSError, ValueError, KeyError, TypeError, IndexError, ZeroDivisionError) as exc:
        result = {"schema": "walt-full-game-audit-v1", "status": "failed", "error": str(exc),
                  "reference": str(args.reference), "candidate": str(args.compare) if args.compare else None}
    text = json.dumps(result, indent=2)
    if args.output:
        args.output.write_text(text + "\n")
    print(text)
    return 0 if result["status"] == "verified" else 1


if __name__ == "__main__":
    sys.exit(main())
