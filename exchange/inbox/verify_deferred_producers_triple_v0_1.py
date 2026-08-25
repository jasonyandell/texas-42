#!/usr/bin/env python3
"""
Exact-rational companion for RESPONSE-deferred-producers-triple-v0.1.md.

Part 1:
  * exact lower-tail e-process inversion on the finite population grid;
  * exhaustive finite-horizon check of the max-preserving E3 upper bound
    over every two-policy Boolean table on four worlds;
  * pathwise comparison against the clairvoyant fused E2 baseline.

Part 2:
  * a two-trick high-trump strict-dominance specimen certified by the
    one-round trump-extraction witness;
  * a three-trick exact-dominance specimen deliberately outside that
    witness's coverage.

Part 3:
  * six mutually exclusive first-split morphology motifs plus Other;
  * twenty hand-constructed trace dictionaries using exactly the declared
    top-level trace fields.

All correctness arithmetic is fractions.Fraction or integers.
No network, file input, randomness, or floating-point correctness logic.
"""

from __future__ import annotations

from collections import Counter
from dataclasses import dataclass
from fractions import Fraction
from itertools import product
from math import comb, factorial
import sys


def check(name: str, condition: bool, detail: str = "") -> None:
    if condition:
        print(f"PASS {name}" + (f" {detail}" if detail else ""))
    else:
        print(f"FAIL {name}" + (f" {detail}" if detail else ""))
        raise AssertionError(name)


# ---------------------------------------------------------------------------
# Part 1 — exact max-preserving upper confidence sequence
# ---------------------------------------------------------------------------

def e_upper(s: int, f: int, c: Fraction) -> Fraction:
    """E^>_{s,f}(c), exact finite sum, 0 < c < 1."""
    assert 0 < c < 1
    ratio = (1 - c) / c
    total = Fraction(0)
    for i in range(s + 1):
        total += (
            Fraction(comb(s, i))
            * ratio**i
            * Fraction(factorial(i) * factorial(f), factorial(i + f + 1))
        )
    return total


def e_lower(s: int, f: int, c: Fraction) -> Fraction:
    """Natural lower-tail evidence E^<_{s,f}(c)=E^>_{f,s}(1-c)."""
    assert 0 < c < 1
    return e_upper(f, s, 1 - c)


def upper_grid(s: int, n: int, population: int, delta: Fraction) -> Fraction:
    """
    Exact upper CS endpoint on G_N={0,1/N,...,1}.

    c=0 is never rejected. c=1 remains possible exactly when no failure
    has appeared. Interior c remains possible when lower-tail evidence
    has not crossed 1/delta.
    """
    assert 0 <= s <= n
    assert population >= 1
    assert 0 < delta < 1
    threshold = 1 / delta
    candidates = [Fraction(0)]
    for k in range(1, population):
        c = Fraction(k, population)
        if e_lower(s, n - s, c) < threshold:
            candidates.append(c)
    if s == n:
        candidates.append(Fraction(1))
    return max(candidates)


def path_bounds(
    table: tuple[tuple[int, ...], ...],
    stream: tuple[int, ...],
    population: int,
    delta: Fraction,
) -> tuple[list[Fraction], list[Fraction]]:
    """
    Nested E3 and E2 upper bounds along one stream.

    E3 uses S*_t=max_policy sum_{i<=t} X_policy(world_i).
    E2 uses F_t=sum_{i<=t} max_policy X_policy(world_i).
    """
    counts = [0] * len(table)
    fused_count = 0
    e3_bound = Fraction(1)
    e2_bound = Fraction(1)
    e3_history: list[Fraction] = []
    e2_history: list[Fraction] = []

    for t, world in enumerate(stream, start=1):
        values = [policy[world] for policy in table]
        for j, value in enumerate(values):
            counts[j] += value
        fused_count += max(values)

        e3_bound = min(
            e3_bound,
            upper_grid(max(counts), t, population, delta),
        )
        e2_bound = min(
            e2_bound,
            upper_grid(fused_count, t, population, delta),
        )
        e3_history.append(e3_bound)
        e2_history.append(e2_bound)

    return e3_history, e2_history


def verify_part1() -> None:
    population = 4
    horizon = 4
    delta = Fraction(1, 4)
    all_policies = list(product((0, 1), repeat=population))

    worst_undercoverage = Fraction(0)
    worst_table = None
    tables_checked = 0
    streams_checked = 0

    for policy0 in all_policies:
        for policy1 in all_policies:
            table = (policy0, policy1)
            true_supremum = max(
                Fraction(sum(policy), population) for policy in table
            )
            bad_streams = 0
            for stream in product(range(population), repeat=horizon):
                e3, e2 = path_bounds(table, stream, population, delta)
                check_pathwise = all(x <= y for x, y in zip(e3, e2))
                if not check_pathwise:
                    raise AssertionError(("E3 exceeded E2", table, stream, e3, e2))
                if any(true_supremum > bound for bound in e3):
                    bad_streams += 1
                streams_checked += 1

            probability = Fraction(bad_streams, population**horizon)
            if probability > worst_undercoverage:
                worst_undercoverage = probability
                worst_table = table
            if probability > delta:
                raise AssertionError(
                    ("undercoverage", table, true_supremum, probability, delta)
                )
            tables_checked += 1

    check(
        "part1 exhaustive finite-horizon validity",
        worst_undercoverage <= delta,
        f"tables={tables_checked} stream-evaluations={streams_checked} "
        f"worst={worst_undercoverage} delta={delta}",
    )
    check(
        "part1 E3 pathwise no looser than fused E2",
        True,
        "verified on every prefix of every enumerated stream",
    )

    # Strict tightness specimen.
    table = (
        (1, 1, 0, 0),
        (0, 0, 1, 1),
    )
    stream = (0, 1, 2, 3)
    true_supremum = Fraction(1, 2)
    e3, e2 = path_bounds(table, stream, population, delta)

    check("part1 specimen exact R", true_supremum == Fraction(1, 2))
    check(
        "part1 specimen exact evidence",
        e_lower(2, 2, Fraction(3, 4)) == Fraction(17, 15),
        "E^<_{2,2}(3/4)=17/15<4",
    )
    check(
        "part1 specimen E3 strict improvement",
        e3[-1] == Fraction(3, 4) and e2[-1] == Fraction(1),
        f"E3={e3[-1]} E2={e2[-1]} R={true_supremum}",
    )


# ---------------------------------------------------------------------------
# Part 2 — structural zero-hazard witness and non-coverage
# ---------------------------------------------------------------------------

TRUMP = "T"


@dataclass(frozen=True)
class Card:
    name: str
    suit: str
    rank: int


H = Card("H", TRUMP, 3)
M = Card("M", TRUMP, 2)
L1 = Card("L1", TRUMP, 1)
L0 = Card("L0", TRUMP, 0)
D = Card("D", "A", 3)
X = Card("X", "B", 1)
Y = Card("Y", "C", 1)
Z = Card("Z", "D", 1)


def legal_cards(hand: list[Card], led_suit: str) -> list[Card]:
    followers = [card for card in hand if card.suit == led_suit]
    return followers or list(hand)


def trick_key(card: Card, led_suit: str) -> tuple[int, int]:
    tier = 2 if card.suit == TRUMP else 1 if card.suit == led_suit else 0
    return tier, card.rank


def field_choice(hand: list[Card], led_suit: str | None = None) -> Card:
    if led_suit is None:
        return min(hand, key=lambda card: (card.suit, card.rank, card.name))
    legal = legal_cards(hand, led_suit)
    void_in_led = not any(card.suit == led_suit for card in hand)
    trumps = [card for card in legal if card.suit == TRUMP]
    if void_in_led and trumps:
        return min(trumps, key=lambda card: card.rank)
    return min(legal, key=lambda card: (card.rank, card.suit, card.name))


def focal_choice(
    hand: list[Card],
    declared_order: tuple[Card, ...],
    led_suit: str | None = None,
) -> Card:
    legal = set(hand if led_suit is None else legal_cards(hand, led_suit))
    for card in declared_order:
        if card in hand and card in legal:
            return card
    raise AssertionError("focal policy has no legal declared action")


def simulate_game(
    focal_hand: tuple[Card, ...],
    opponent_hand: tuple[Card, ...],
    focal_order: tuple[Card, ...],
) -> tuple[bool, tuple[tuple[str, str, str, str, str], ...]]:
    """
    Two-player trick-taking microgame.

    Must follow suit. Trump beats nontrump. The focal contract succeeds
    exactly when the focal seat wins every trick.
    """
    hands = {"F": list(focal_hand), "O": list(opponent_hand)}
    leader = "F"
    focal_wins = 0
    transcript = []

    while hands["F"]:
        follower = "O" if leader == "F" else "F"
        if leader == "F":
            lead = focal_choice(hands["F"], focal_order)
        else:
            lead = field_choice(hands["O"])
        hands[leader].remove(lead)

        if follower == "F":
            follow = focal_choice(hands["F"], focal_order, lead.suit)
        else:
            follow = field_choice(hands["O"], lead.suit)
        hands[follower].remove(follow)

        winner = (
            follower
            if trick_key(follow, lead.suit) > trick_key(lead, lead.suit)
            else leader
        )
        if winner == "F":
            focal_wins += 1
        transcript.append((leader, lead.name, follower, follow.name, winner))
        leader = winner

    return focal_wins == len(focal_hand), tuple(transcript)


def one_round_trump_extraction_witness(
    focal_hand: tuple[Card, ...],
    worlds: tuple[tuple[Card, ...], ...],
    action_a_order: tuple[Card, ...],
    action_b_order: tuple[Card, ...],
) -> bool:
    """
    Cheap incomplete producer.

    It certifies the one-round pattern only:
      * exactly two remaining tricks;
      * a leads the globally highest trump;
      * b leads a nontrump vulnerable card;
      * every hostile hand contains at most one trump;
      * absent a ruff, no hostile same-suit card can beat the vulnerable card.
    """
    if len(focal_hand) != 2:
        return False
    high = action_a_order[0]
    vulnerable = action_b_order[0]
    if high.suit != TRUMP or vulnerable.suit == TRUMP:
        return False

    all_cards = list(focal_hand) + [card for world in worlds for card in world]
    if any(
        card.suit == TRUMP and card.rank > high.rank
        for card in all_cards
    ):
        return False

    for world in worlds:
        if sum(card.suit == TRUMP for card in world) > 1:
            return False
        if any(
            card.suit == vulnerable.suit and card.rank > vulnerable.rank
            for card in world
        ):
            return False
        # This first producer declines when follow-suit blocks the proposed ruff.
        if (
            any(card.suit == TRUMP for card in world)
            and any(card.suit == vulnerable.suit for card in world)
        ):
            return False
    return True


def benefit_hazard(
    outcomes_a: list[bool],
    outcomes_b: list[bool],
) -> tuple[Fraction, Fraction]:
    assert len(outcomes_a) == len(outcomes_b)
    n = len(outcomes_a)
    benefit = sum(a and not b for a, b in zip(outcomes_a, outcomes_b))
    hazard = sum((not a) and b for a, b in zip(outcomes_a, outcomes_b))
    return Fraction(benefit, n), Fraction(hazard, n)


def verify_part2() -> None:
    # Worked strict-dominance specimen.
    worlds2 = ((L1, X), (X, Y))
    outcomes_a = [
        simulate_game((H, D), world, (H, D))[0] for world in worlds2
    ]
    outcomes_b = [
        simulate_game((H, D), world, (D, H))[0] for world in worlds2
    ]
    witness = one_round_trump_extraction_witness(
        (H, D), worlds2, (H, D), (D, H)
    )
    benefit, hazard = benefit_hazard(outcomes_a, outcomes_b)

    check("part2 one-round witness accepts", witness)
    check(
        "part2 worked example exact dominance",
        hazard == 0 and benefit == Fraction(1, 2),
        f"B={benefit} H={hazard}",
    )

    # Non-coverage: dominance needs two rounds of trump extraction.
    worlds3 = ((L1, L0, X), (X, Y, Z))
    outcomes_a3 = [
        simulate_game((H, M, D), world, (H, M, D))[0] for world in worlds3
    ]
    outcomes_b3 = [
        simulate_game((H, M, D), world, (D, H, M))[0] for world in worlds3
    ]
    witness3 = one_round_trump_extraction_witness(
        (H, M, D), worlds3, (H, M, D), (D, H, M)
    )
    benefit3, hazard3 = benefit_hazard(outcomes_a3, outcomes_b3)

    check("part2 noncoverage witness declines", not witness3)
    check(
        "part2 noncoverage enumeration still proves dominance",
        hazard3 == 0 and benefit3 == Fraction(1, 2),
        f"B={benefit3} H={hazard3}",
    )


# ---------------------------------------------------------------------------
# Part 3 — first-split morphology alphabet
# ---------------------------------------------------------------------------

# tile_id -> (effective suit, rank, count payload)
TILES: dict[int, tuple[str, int, int]] = {
    0: ("T", 9, 0),
    1: ("T", 5, 5),
    2: ("T", 2, 0),
    3: ("A", 9, 0),
    4: ("A", 5, 5),
    5: ("A", 2, 0),
    6: ("B", 9, 0),
    7: ("B", 5, 0),
    8: ("B", 2, 0),
    9: ("C", 9, 0),
    10: ("C", 5, 10),
    11: ("C", 2, 0),
    12: ("A", 5, 0),
    13: ("D", 5, 0),
    14: ("D", 2, 0),
    15: ("A", 5, 0),  # structural duplicate of 12 for an Other fixture
}

# root_id is part of the trace. A production classifier must resolve it
# against an immutable root-semantics table (or the trace schema must carry
# a pinned root snapshot). No motif is guessed if that lookup fails.
ROOTS = {
    "lead": {"trump": "T", "focal_seat": 0},
    "mid": {"trump": "T", "focal_seat": 1},
}


def tile_mask(tile_ids: list[int]) -> int:
    mask = 0
    for tile in tile_ids:
        mask |= 1 << tile
    return mask


def mask_tiles(mask: int) -> list[int]:
    return [tile for tile in TILES if (mask >> tile) & 1]


def team(seat: int) -> int:
    return seat % 2


def local_trick_key(tile: int, led_suit: str) -> tuple[int, int]:
    suit, rank, _ = TILES[tile]
    tier = 2 if suit == "T" else 1 if suit == led_suit else 0
    return tier, rank


def provisional_winner_team(
    record: list[tuple[int, int]],
    candidate: int,
    actor: int,
) -> int:
    plays = list(record) + [(actor, candidate)]
    led_suit = TILES[plays[0][1]][0]
    winner_seat, _ = max(
        plays,
        key=lambda seat_tile: local_trick_key(seat_tile[1], led_suit),
    )
    return team(winner_seat)


def residual_suit_shape(hand_mask: int, remove: int) -> tuple[tuple[str, int], ...]:
    counts = Counter(
        TILES[tile][0]
        for tile in mask_tiles(hand_mask)
        if tile != remove
    )
    return tuple(sorted(counts.items()))


MOTIFS = (
    "LeadContextFork",
    "ImmediateControlFork",
    "CountCommitmentFork",
    "TrumpCommitmentFork",
    "SuitShapeFork",
    "StrengthCommitmentFork",
)


def motif(trace: dict) -> str:
    root = ROOTS.get(trace["root_id"])
    if root is None:
        return "Other"

    split = trace["split"]
    tile0 = split["tile0"]
    tile1 = split["tile1"]
    if tile0 not in TILES or tile1 not in TILES:
        return "Other"

    record = split["record"]
    actor = split["seat"]
    hand = split["hand"]

    context0 = (
        TILES[tile0][0] if split["ply"] == 0 else TILES[record[0][1]][0]
    )
    context1 = (
        TILES[tile1][0] if split["ply"] == 0 else TILES[record[0][1]][0]
    )

    signature0 = (
        context0,
        provisional_winner_team(record, tile0, actor),
        TILES[tile0][2],
        TILES[tile0][0] == root["trump"],
        residual_suit_shape(hand, tile0),
        local_trick_key(tile0, context0),
    )
    signature1 = (
        context1,
        provisional_winner_team(record, tile1, actor),
        TILES[tile1][2],
        TILES[tile1][0] == root["trump"],
        residual_suit_shape(hand, tile1),
        local_trick_key(tile1, context1),
    )

    for index, label in enumerate(MOTIFS):
        if signature0[index] != signature1[index]:
            return label
    return "Other"


def make_trace(
    root_id: str,
    seat: int,
    ply: int,
    tile0: int,
    tile1: int,
    hand_tiles: list[int],
    record: list[tuple[int, int]],
    u0: int = 0,
    u1: int = 1,
    trick: int = 1,
) -> dict:
    # Exactly the declared top-level fields and split subfields.
    return {
        "root_id": root_id,
        "world": [0, 0, 0, 0],
        "action": tile0,
        "policy": "policy",
        "field0": "field0",
        "field1": "field1",
        "split": {
            "seat": seat,
            "trick": trick,
            "ply": ply,
            "tile0": tile0,
            "tile1": tile1,
            "hand": tile_mask(hand_tiles),
            "record": record,
        },
        "u0": u0,
        "u1": u1,
    }


def motif_fixtures() -> list[dict]:
    traces: list[dict] = []

    # LeadContextFork
    for pair in ((3, 6), (4, 9), (5, 13)):
        traces.append(
            make_trace("lead", 1, 0, pair[0], pair[1], [*pair, 0], [])
        )

    # ImmediateControlFork
    for pair in ((3, 5), (4, 5), (3, 4)):
        traces.append(
            make_trace("mid", 1, 1, pair[0], pair[1], [*pair, 6], [(0, 4)])
        )

    # CountCommitmentFork
    for pair in ((4, 3), (4, 12), (10, 9)):
        record = [(0, 5)] if TILES[pair[0]][0] == "A" else [(0, 11)]
        traces.append(
            make_trace("mid", 1, 1, pair[0], pair[1], [*pair, 6], record)
        )

    # TrumpCommitmentFork / possible co-differences are resolved by priority.
    for pair in ((2, 8), (1, 7), (0, 6)):
        traces.append(
            make_trace("mid", 2, 1, pair[0], pair[1], [*pair, 3], [(0, 3)])
        )

    # SuitShapeFork
    for pair in ((4, 7), (5, 8), (9, 6)):
        traces.append(
            make_trace("mid", 2, 1, pair[0], pair[1], [*pair, 0], [(0, 0)])
        )

    # StrengthCommitmentFork
    for pair in ((3, 12), (12, 5), (6, 7)):
        led_tile = 5 if TILES[pair[0]][0] == "A" else 8
        traces.append(
            make_trace(
                "mid", 1, 1, pair[0], pair[1], [*pair, 9], [(0, led_tile)]
            )
        )

    # Other — same declared local signature.
    traces.append(
        make_trace("mid", 1, 1, 12, 15, [12, 15, 6], [(0, 5)])
    )
    traces.append(
        make_trace(
            "mid", 1, 1, 15, 12, [12, 15, 7], [(0, 5)], u0=1, u1=0
        )
    )
    return traces


def verify_part3() -> None:
    traces = motif_fixtures()
    labels = [motif(trace) for trace in traces]
    counts = Counter(labels)

    check("part3 fixture count", len(traces) >= 20, f"n={len(traces)}")
    check(
        "part3 total primary partition",
        sum(counts.values()) == len(traces),
        str(dict(sorted(counts.items()))),
    )
    check(
        "part3 every named motif represented",
        all(counts[label] > 0 for label in MOTIFS),
        str(dict(sorted(counts.items()))),
    )
    check("part3 residual represented", counts["Other"] > 0)


def main() -> int:
    verify_part1()
    verify_part2()
    verify_part3()
    print("ALL CHECKS PASS")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError:
        raise SystemExit(1)
