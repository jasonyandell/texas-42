#!/usr/bin/env python3
"""Finite verification for pip-transport/reachability commutation in Straight 42.

Self-contained, deterministic, standard-library only, and no file/network I/O.
It verifies the unscored pip-trump mechanics transport exhaustively and checks
trace/support-normal-form commutation on a deterministic 6,496-prefix corpus.
"""

from __future__ import annotations

from dataclasses import dataclass
import sys
from typing import Iterable

PIPS = tuple(range(7))
CALLED = 7
CONTEXTS = tuple(range(8))
DOMINOES = tuple((high, low) for high in PIPS for low in range(high + 1))
ID_OF = {domino: index for index, domino in enumerate(DOMINOES)}
N_DOMINOES = len(DOMINOES)
ALL_DOMINOES_MASK = (1 << N_DOMINOES) - 1
ALL_HOLDER_BITS = 0b111


class CheckFailure(Exception):
    def __init__(self, check: str, detail: str) -> None:
        super().__init__(f"{check}: {detail}")
        self.check = check
        self.detail = detail


def require(condition: bool, check: str, detail: str) -> None:
    if not condition:
        raise CheckFailure(check, detail)


def iter_bits(mask: int) -> Iterable[int]:
    while mask:
        bit = mask & -mask
        yield bit.bit_length() - 1
        mask ^= bit


def mask_of(dominoes: Iterable[int]) -> int:
    result = 0
    for domino in dominoes:
        result |= 1 << domino
    return result


def contains(domino: int, pip: int) -> bool:
    high, low = DOMINOES[domino]
    return high == pip or low == pip


def is_double(domino: int) -> bool:
    high, low = DOMINOES[domino]
    return high == low


def called(domino: int, trump: int) -> bool:
    return contains(domino, trump)


def rank(domino: int) -> int:
    high, low = DOMINOES[domino]
    return 14 if high == low else high + low


NATURAL_MASKS = tuple(
    mask_of(d for d in range(N_DOMINOES) if contains(d, pip)) for pip in PIPS
)
CALLED_MASKS = NATURAL_MASKS
EFFECTIVE_MASKS = tuple(
    tuple(
        CALLED_MASKS[trump]
        if context == CALLED
        else NATURAL_MASKS[context] & ~CALLED_MASKS[trump]
        for context in CONTEXTS
    )
    for trump in PIPS
)


def led_context(domino: int, trump: int) -> int:
    return CALLED if called(domino, trump) else DOMINOES[domino][0]


def follows(domino: int, context: int, trump: int) -> bool:
    return bool(EFFECTIVE_MASKS[trump][context] & (1 << domino))


def trick_key(domino: int, context: int, trump: int) -> tuple[int, int]:
    if called(domino, trump):
        return (2, rank(domino))
    if follows(domino, context, trump):
        return (1, rank(domino))
    return (0, 0)


def legal_mask(hand: int, trick: tuple[tuple[int, int], ...], trump: int) -> int:
    if not trick:
        return hand
    context = led_context(trick[0][1], trump)
    followers = hand & EFFECTIVE_MASKS[trump][context]
    return followers if followers else hand


def resolve_trick(trick: tuple[tuple[int, int], ...], trump: int) -> int:
    require(len(trick) == 4, "RULES", "resolve_trick called on non-four-tile trick")
    context = led_context(trick[0][1], trump)
    keys = tuple(trick_key(domino, context, trump) for _, domino in trick)
    maximum = max(keys)
    require(keys.count(maximum) == 1, "RULES", f"nonunique trick maximum: {trick}")
    return trick[keys.index(maximum)][0]


@dataclass(frozen=True)
class Transport:
    pips: tuple[int, ...]
    dominoes: tuple[int, ...]
    contexts: tuple[int, ...]


def make_transport(source: int, target: int) -> Transport:
    source_complement = tuple(p for p in PIPS if p != source)
    target_complement = tuple(p for p in PIPS if p != target)
    pip_map = [-1] * 7
    pip_map[source] = target
    for left, right in zip(source_complement, target_complement):
        pip_map[left] = right
    pips = tuple(pip_map)
    domino_map = []
    for high, low in DOMINOES:
        image = tuple(sorted((pips[high], pips[low]), reverse=True))
        domino_map.append(ID_OF[image])
    return Transport(pips, tuple(domino_map), pips + (CALLED,))


def map_tile_mask(mask: int, domino_map: tuple[int, ...]) -> int:
    result = 0
    for domino in iter_bits(mask):
        result |= 1 << domino_map[domino]
    return result


def map_context_mask(mask: int, context_map: tuple[int, ...]) -> int:
    result = 0
    for context in iter_bits(mask):
        result |= 1 << context_map[context]
    return result


Bid = tuple[str, int] | None


def legal_bid(history: tuple[Bid, ...], bid: Bid, cap: int = 5) -> bool:
    if bid is None:
        return True
    kind, value = bid
    high = next((old for old in reversed(history) if old is not None), None)
    if kind == "P":
        if not (30 <= value <= 41):
            return False
        return high is None or (high[0] == "P" and value > high[1])
    if kind != "M" or not (1 <= value <= cap):
        return False
    if high is None or high[0] == "P":
        return value <= 2
    return value == high[1] + 1


def validate_auction(
    shaker: int, auction: tuple[tuple[int, Bid], ...], cap: int = 5
) -> int:
    require(len(auction) == 4, "AUCTION", "auction does not have four actions")
    expected = tuple((shaker + 1 + index) % 4 for index in range(4))
    require(
        tuple(seat for seat, _ in auction) == expected,
        "AUCTION",
        f"wrong actor order: {auction}",
    )
    history: list[Bid] = []
    bidder = -1
    for seat, bid in auction:
        require(legal_bid(tuple(history), bid, cap), "AUCTION", f"illegal bid {bid}")
        history.append(bid)
        if bid is not None:
            bidder = seat
    require(bidder >= 0, "AUCTION", "all-pass auction cannot start contracted play")
    return bidder


@dataclass(frozen=True)
class Prefix:
    deal: tuple[int, int, int, int]
    shaker: int
    auction: tuple[tuple[int, Bid], ...]
    declaration: int
    viewer: int
    plays: tuple[tuple[int, int], ...]


@dataclass(frozen=True)
class ReplayState:
    hands: tuple[int, int, int, int]
    leader: int
    trick: tuple[tuple[int, int], ...]
    played_by: tuple[int, int, int, int]
    voids: tuple[int, int, int, int]
    sloughs: int


def validate_deal(deal: tuple[int, int, int, int]) -> None:
    require(all(hand.bit_count() == 7 for hand in deal), "DEAL", "non-seven-tile hand")
    union = 0
    for hand in deal:
        require(not (union & hand), "DEAL", "overlapping hands")
        union |= hand
    require(union == ALL_DOMINOES_MASK, "DEAL", "deal does not partition all dominoes")


def replay(prefix: Prefix) -> ReplayState:
    require(prefix.declaration in PIPS, "REPLAY", "non-pip declaration")
    require(prefix.viewer in range(4), "REPLAY", "viewer outside seat set")
    require(len(prefix.plays) <= 28, "REPLAY", "more than 28 plays")
    validate_deal(prefix.deal)
    bidder = validate_auction(prefix.shaker, prefix.auction)
    hands = list(prefix.deal)
    played_by = [0, 0, 0, 0]
    voids = [0, 0, 0, 0]
    leader = bidder
    trick: tuple[tuple[int, int], ...] = ()
    sloughs = 0

    for ply, (expected_actor, domino) in enumerate(prefix.plays):
        actor = (leader + len(trick)) % 4
        require(actor == expected_actor, "REPLAY", f"ply {ply}: actor {expected_actor}, expected {actor}")
        bit = 1 << domino
        require(bool(hands[actor] & bit), "REPLAY", f"ply {ply}: actor lacks domino {domino}")
        legal = legal_mask(hands[actor], trick, prefix.declaration)
        require(bool(legal & bit), "REPLAY", f"ply {ply}: illegal play {domino}")
        if trick:
            context = led_context(trick[0][1], prefix.declaration)
            if not follows(domino, context, prefix.declaration):
                voids[actor] |= 1 << context
                sloughs += 1
        hands[actor] ^= bit
        played_by[actor] |= bit
        trick = trick + ((actor, domino),)
        if len(trick) == 4:
            leader = resolve_trick(trick, prefix.declaration)
            trick = ()

    return ReplayState(
        tuple(hands),
        leader,
        trick,
        tuple(played_by),
        tuple(voids),
        sloughs,
    )


@dataclass(frozen=True)
class Cells:
    pool: int
    possible: tuple[int, int, int]
    capacities: tuple[int, int, int]


@dataclass(frozen=True)
class NormalForm:
    certain: tuple[int, int, int]
    tag: str
    ambiguous: int = 0
    inactive: int = -1
    split: int = -1
    r0: int = -1
    r1: int = -1
    exclusions: tuple[tuple[int, int], ...] = ()


def hidden_seats(viewer: int) -> tuple[int, int, int]:
    return ((viewer + 1) % 4, (viewer + 2) % 4, (viewer + 3) % 4)


def derive_cells(prefix: Prefix, state: ReplayState) -> Cells:
    hidden = hidden_seats(prefix.viewer)
    public = state.played_by[0] | state.played_by[1] | state.played_by[2] | state.played_by[3]
    pool = ALL_DOMINOES_MASK & ~(state.hands[prefix.viewer] | public)
    capacities = tuple(7 - state.played_by[seat].bit_count() for seat in hidden)
    possible_list: list[int] = []
    for seat in hidden:
        forbidden = 0
        for context in iter_bits(state.voids[seat]):
            forbidden |= EFFECTIVE_MASKS[prefix.declaration][context]
        possible_list.append(pool & ~forbidden)
    possible = tuple(possible_list)
    actual_pool = state.hands[hidden[0]] | state.hands[hidden[1]] | state.hands[hidden[2]]
    require(pool == actual_pool, "CELLS", "derived pool differs from actual hidden remainder")
    require(sum(capacities) == pool.bit_count(), "CELLS", "capacity/pool mismatch")
    for index, seat in enumerate(hidden):
        require(
            state.hands[seat].bit_count() == capacities[index],
            "CELLS",
            "actual hidden capacity mismatch",
        )
        require(
            not (state.hands[seat] & ~possible[index]),
            "CELLS",
            "actual hidden hand violates a public void",
        )
    return Cells(pool, possible, capacities)


def hall_feasible(pool: int, possible: tuple[int, int, int], capacities: tuple[int, int, int]) -> bool:
    if any(capacity < 0 for capacity in capacities):
        return False
    if sum(capacities) != pool.bit_count():
        return False
    if any(mask & ~pool for mask in possible):
        return False
    for seat_subset in range(1, 8):
        neighbors = 0
        demand = 0
        for seat in range(3):
            if seat_subset & (1 << seat):
                neighbors |= possible[seat]
                demand += capacities[seat]
        if neighbors.bit_count() < demand:
            return False
    return True


def marginal_possible(cells: Cells) -> tuple[int, int, int]:
    require(hall_feasible(cells.pool, cells.possible, cells.capacities), "NORMAL_FORM", "infeasible reachable cells")
    supported = [0, 0, 0]
    for seat in range(3):
        if cells.capacities[seat] == 0:
            continue
        for domino in iter_bits(cells.pool & cells.possible[seat]):
            bit = 1 << domino
            successor_pool = cells.pool ^ bit
            successor_possible = tuple(mask & ~bit for mask in cells.possible)
            successor_capacities = list(cells.capacities)
            successor_capacities[seat] -= 1
            if hall_feasible(successor_pool, successor_possible, tuple(successor_capacities)):
                supported[seat] |= bit
    union = supported[0] | supported[1] | supported[2]
    require(union == cells.pool, "NORMAL_FORM", "some live tile has no marginal holder")
    return tuple(supported)


def compile_normal_form(cells: Cells) -> NormalForm:
    supported = marginal_possible(cells)
    certain = [0, 0, 0]
    holder_bits: dict[int, int] = {}
    for domino in iter_bits(cells.pool):
        holders = 0
        for seat in range(3):
            if supported[seat] & (1 << domino):
                holders |= 1 << seat
        holder_bits[domino] = holders
        if holders and not (holders & (holders - 1)):
            certain[holders.bit_length() - 1] |= 1 << domino
    certain_tuple = tuple(certain)
    certain_union = certain[0] | certain[1] | certain[2]
    ambiguous = cells.pool & ~certain_union
    residual = tuple(
        cells.capacities[seat] - certain[seat].bit_count() for seat in range(3)
    )
    active = tuple(seat for seat in range(3) if residual[seat] > 0)

    if not ambiguous:
        require(residual == (0, 0, 0), "NORMAL_FORM", "determinate core has residual capacity")
        return NormalForm(certain_tuple, "Determinate")

    if len(active) == 2:
        inactive = next(seat for seat in range(3) if seat not in active)
        active_mask = (1 << active[0]) | (1 << active[1])
        for domino in iter_bits(ambiguous):
            require(holder_bits[domino] == active_mask, "NORMAL_FORM", "restricted binary tile")
        split = residual[active[0]]
        require(
            1 <= split < ambiguous.bit_count()
            and residual[active[1]] == ambiguous.bit_count() - split
            and residual[inactive] == 0,
            "NORMAL_FORM",
            "invalid binary residuals",
        )
        return NormalForm(certain_tuple, "Binary", ambiguous, inactive, split)

    require(len(active) == 3, "NORMAL_FORM", f"impossible active-seat count {len(active)}")
    exclusions: list[tuple[int, int]] = []
    for domino in iter_bits(ambiguous):
        holders = holder_bits[domino]
        require(holders in (0b111, 0b110, 0b101, 0b011), "NORMAL_FORM", "invalid ternary holder set")
        if holders != ALL_HOLDER_BITS:
            excluded_bit = ALL_HOLDER_BITS ^ holders
            require(not (excluded_bit & (excluded_bit - 1)), "NORMAL_FORM", "non-singleton exclusion")
            exclusions.append((domino, excluded_bit.bit_length() - 1))
    require(all(value > 0 for value in residual), "NORMAL_FORM", "ternary residual not positive")
    require(sum(residual) == ambiguous.bit_count(), "NORMAL_FORM", "ternary conservation failure")
    return NormalForm(
        certain_tuple,
        "Ternary",
        ambiguous,
        r0=residual[0],
        r1=residual[1],
        exclusions=tuple(exclusions),
    )


def transport_normal_form(form: NormalForm, domino_map: tuple[int, ...]) -> NormalForm:
    certain = tuple(map_tile_mask(mask, domino_map) for mask in form.certain)
    ambiguous = map_tile_mask(form.ambiguous, domino_map)
    exclusions = tuple(sorted((domino_map[domino], seat) for domino, seat in form.exclusions))
    return NormalForm(
        certain,
        form.tag,
        ambiguous,
        form.inactive,
        form.split,
        form.r0,
        form.r1,
        exclusions,
    )


def transport_prefix(prefix: Prefix, target: int) -> Prefix:
    transport = make_transport(prefix.declaration, target)
    return Prefix(
        tuple(map_tile_mask(hand, transport.dominoes) for hand in prefix.deal),
        prefix.shaker,
        prefix.auction,
        target,
        prefix.viewer,
        tuple((seat, transport.dominoes[domino]) for seat, domino in prefix.plays),
    )


def compare_transported_state(
    source: ReplayState,
    target: ReplayState,
    transport: Transport,
    detail: str,
) -> None:
    expected_hands = tuple(map_tile_mask(hand, transport.dominoes) for hand in source.hands)
    expected_played = tuple(map_tile_mask(mask, transport.dominoes) for mask in source.played_by)
    expected_voids = tuple(map_context_mask(mask, transport.contexts) for mask in source.voids)
    expected_trick = tuple((seat, transport.dominoes[domino]) for seat, domino in source.trick)
    require(target.hands == expected_hands, "TRACE_TRANSPORT", detail + " hands")
    require(target.played_by == expected_played, "TRACE_TRANSPORT", detail + " played attribution")
    require(target.voids == expected_voids, "TRACE_TRANSPORT", detail + " voids")
    require(target.leader == source.leader, "TRACE_TRANSPORT", detail + " leader")
    require(target.trick == expected_trick, "TRACE_TRANSPORT", detail + " current trick")
    require(target.sloughs == source.sloughs, "TRACE_TRANSPORT", detail + " slough count")


def compare_transported_cells(source: Cells, target: Cells, transport: Transport, detail: str) -> None:
    require(target.pool == map_tile_mask(source.pool, transport.dominoes), "SUPPORT_TRANSPORT", detail + " pool")
    require(target.capacities == source.capacities, "SUPPORT_TRANSPORT", detail + " capacities")
    expected_possible = tuple(map_tile_mask(mask, transport.dominoes) for mask in source.possible)
    require(target.possible == expected_possible, "SUPPORT_TRANSPORT", detail + " possible-holder cells")


class SplitMix64:
    def __init__(self, seed: int) -> None:
        self.state = seed & ((1 << 64) - 1)

    def next(self) -> int:
        self.state = (self.state + 0x9E3779B97F4A7C15) & ((1 << 64) - 1)
        value = self.state
        value = ((value ^ (value >> 30)) * 0xBF58476D1CE4E5B9) & ((1 << 64) - 1)
        value = ((value ^ (value >> 27)) * 0x94D049BB133111EB) & ((1 << 64) - 1)
        return value ^ (value >> 31)


def deterministic_deal(trump: int, case: int) -> tuple[int, int, int, int]:
    rng = SplitMix64(0x42D0B1E5A17C9F03 ^ (trump << 40) ^ (case * 0xD1342543DE82EF95))
    order = list(range(N_DOMINOES))
    for index in range(N_DOMINOES - 1, 0, -1):
        other = rng.next() % (index + 1)
        order[index], order[other] = order[other], order[index]
    return tuple(mask_of(order[seat * 7 : (seat + 1) * 7]) for seat in range(4))


def generate_complete_prefix(trump: int, case: int) -> Prefix:
    deal = deterministic_deal(trump, case)
    bidder = (case + 2 * trump) % 4
    shaker = (bidder - 1) % 4
    auction = tuple(
        ((bidder + offset) % 4, ("P", 30) if offset == 0 else None)
        for offset in range(4)
    )
    viewer = (3 * case + trump) % 4
    rng = SplitMix64(0xC6BC279692B5CC83 ^ (trump << 48) ^ case)
    hands = list(deal)
    leader = bidder
    trick: tuple[tuple[int, int], ...] = ()
    plays: list[tuple[int, int]] = []
    for ply in range(28):
        actor = (leader + len(trick)) % 4
        choices = tuple(iter_bits(legal_mask(hands[actor], trick, trump)))
        require(bool(choices), "GENERATOR", "empty legal set before terminal")
        selector = (rng.next() + 17 * ply + 11 * actor + 5 * case + trump) % len(choices)
        domino = choices[selector]
        hands[actor] ^= 1 << domino
        plays.append((actor, domino))
        trick = trick + ((actor, domino),)
        if len(trick) == 4:
            leader = resolve_trick(trick, trump)
            trick = ()
    require(all(hand == 0 for hand in hands) and not trick, "GENERATOR", "trace did not terminate cleanly")
    return Prefix(deal, shaker, auction, trump, viewer, tuple(plays))


def check_alg22() -> int:
    comparisons = 0
    for source in PIPS:
        for target in PIPS:
            transport = make_transport(source, target)
            require(sorted(transport.pips) == list(PIPS), "ALG-22", f"{source}->{target}: pip map not bijective")
            require(sorted(transport.dominoes) == list(range(N_DOMINOES)), "ALG-22", f"{source}->{target}: domino map not bijective")
            for context in CONTEXTS:
                image_context = transport.contexts[context]
                image_mask = map_tile_mask(EFFECTIVE_MASKS[source][context], transport.dominoes)
                require(
                    image_mask == EFFECTIVE_MASKS[target][image_context],
                    "ALG-22",
                    f"{source}->{target}: effective suit {context}",
                )
            for domino in range(N_DOMINOES):
                image = transport.dominoes[domino]
                require(called(domino, source) == called(image, target), "ALG-22", f"{source}->{target}: called {domino}")
                require(
                    transport.contexts[led_context(domino, source)] == led_context(image, target),
                    "ALG-22",
                    f"{source}->{target}: led context {domino}",
                )
                for context in CONTEXTS:
                    require(
                        follows(domino, context, source)
                        == follows(image, transport.contexts[context], target),
                        "ALG-22",
                        f"{source}->{target}: follow d={domino}, q={context}",
                    )
            for context in CONTEXTS:
                image_context = transport.contexts[context]
                for left in range(N_DOMINOES):
                    image_left = transport.dominoes[left]
                    left_key = trick_key(left, context, source)
                    image_left_key = trick_key(image_left, image_context, target)
                    for right in range(N_DOMINOES):
                        comparisons += 1
                        require(
                            (left_key < trick_key(right, context, source))
                            == (image_left_key < trick_key(transport.dominoes[right], image_context, target)),
                            "ALG-22",
                            f"{source}->{target}: order q={context}, d={left}, e={right}",
                        )
    return comparisons


def check_inverse() -> None:
    for source in PIPS:
        for target in PIPS:
            forward = make_transport(source, target)
            backward = make_transport(target, source)
            for pip in PIPS:
                require(backward.pips[forward.pips[pip]] == pip, "INVERSE", f"{source}->{target}: pip {pip}")
            for domino in range(N_DOMINOES):
                require(
                    backward.dominoes[forward.dominoes[domino]] == domino,
                    "INVERSE",
                    f"{source}->{target}: domino {domino}",
                )
            for context in CONTEXTS:
                require(
                    backward.contexts[forward.contexts[context]] == context,
                    "INVERSE",
                    f"{source}->{target}: context {context}",
                )


def check_prefix_corpus() -> tuple[int, int, int, int, tuple[int, ...]]:
    cases_per_trump = 32
    prefix_count = 0
    transport_checks = 0
    void_prefixes = 0
    full_slough_counts = [0] * 7
    depth_coverage = [set() for _ in PIPS]
    bidder_coverage = [set() for _ in PIPS]
    viewer_coverage = [set() for _ in PIPS]
    deal_signatures: set[tuple[int, int, int, int]] = set()

    for trump in PIPS:
        for case in range(cases_per_trump):
            full = generate_complete_prefix(trump, case)
            deal_signatures.add(full.deal)
            bidder_coverage[trump].add(validate_auction(full.shaker, full.auction))
            viewer_coverage[trump].add(full.viewer)
            full_state = replay(full)
            full_slough_counts[trump] += full_state.sloughs

            for depth in range(29):
                prefix = Prefix(
                    full.deal,
                    full.shaker,
                    full.auction,
                    full.declaration,
                    full.viewer,
                    full.plays[:depth],
                )
                source_state = replay(prefix)
                source_cells = derive_cells(prefix, source_state)
                source_form = compile_normal_form(source_cells)
                prefix_count += 1
                depth_coverage[trump].add(depth)
                if any(source_state.voids[seat] for seat in hidden_seats(prefix.viewer)):
                    void_prefixes += 1

                for target in PIPS:
                    transport = make_transport(trump, target)
                    image_prefix = transport_prefix(prefix, target)
                    image_state = replay(image_prefix)
                    detail = f"t={trump},u={target},case={case},depth={depth}"
                    compare_transported_state(source_state, image_state, transport, detail)
                    image_cells = derive_cells(image_prefix, image_state)
                    compare_transported_cells(source_cells, image_cells, transport, detail)
                    image_form = compile_normal_form(image_cells)
                    expected_form = transport_normal_form(source_form, transport.dominoes)
                    require(image_form == expected_form, "SUPPORT_COMMUTATION", detail)
                    transport_checks += 1

    require(prefix_count >= 5000, "GENERATOR", f"only {prefix_count} prefixes")
    require(all(depths == set(range(29)) for depths in depth_coverage), "GENERATOR", "depth coverage incomplete")
    require(all(seats == set(range(4)) for seats in bidder_coverage), "GENERATOR", "bidder coverage incomplete")
    require(all(seats == set(range(4)) for seats in viewer_coverage), "GENERATOR", "viewer coverage incomplete")
    require(all(count > 0 for count in full_slough_counts), "GENERATOR", f"a trump had no slough: {full_slough_counts}")
    require(void_prefixes > 0, "GENERATOR", "no prefix contained a public void")
    require(len(deal_signatures) >= 200, "GENERATOR", f"insufficient deal diversity: {len(deal_signatures)}")
    return prefix_count, transport_checks, void_prefixes, len(deal_signatures), tuple(full_slough_counts)


def main() -> int:
    try:
        require(N_DOMINOES == 28, "RULES", "domino universe is not size 28")
        comparisons = check_alg22()
        print(f"PASS ALG-22 49_ordered_pairs {comparisons}_contextual_comparisons")
        check_inverse()
        print("PASS INVERSE 49_ordered_pairs")
        prefixes, transports, void_prefixes, deals, sloughs = check_prefix_corpus()
        print(
            "PASS GENERATOR "
            f"{prefixes}_prefixes 7_trumps depths_0_28 {deals}_distinct_deals "
            f"{void_prefixes}_prefixes_with_voids sloughs_by_trump={sloughs}"
        )
        print(f"PASS TRACE_TRANSPORT {transports}_transported_prefixes_legal")
        print(f"PASS SUPPORT_COMMUTATION {transports}_normal_form_equalities")
        print("PASS ALL")
        return 0
    except CheckFailure as failure:
        print(f"FAIL {failure.check} {failure.detail}")
        return 1
    except Exception as failure:
        print(f"FAIL INTERNAL {type(failure).__name__}: {failure}")
        return 1


if __name__ == "__main__":
    sys.exit(main())
