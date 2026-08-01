#!/usr/bin/env python3
from __future__ import annotations
from collections import Counter
from itertools import combinations, permutations
import sys
import traceback

PIPS = tuple(range(7))
PIP_DECLARATIONS = tuple(range(7))
DOUBLES_TRUMP = 7
NO_TRUMP = 8
DECLARATIONS = tuple(range(9))
DECLARATION_NAME = tuple(str(p) for p in PIPS) + ("DT", "NT")
CALLED_CONTEXT = 7
DOMINOES = tuple((h, l) for h in PIPS for l in range(h + 1))
N_DOMINOES = len(DOMINOES)
ALL_DOMINO_MASK = (1 << N_DOMINOES) - 1
COUNT_CODE = tuple(
    2 if h + l == 10 else 1 if h + l == 5 else 0 for h, l in DOMINOES
)
COUNT_VALUE = tuple(5 * x for x in COUNT_CODE)
IS_DOUBLE = tuple(h == l for h, l in DOMINOES)
HOLDER_PERMUTATIONS = tuple(permutations(range(4)))
PERMUTATION_INDEX = {p: i for i, p in enumerate(HOLDER_PERMUTATIONS)}
UNORDERED_PAIRS_4 = ((0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3))


def domino_name(d):
    h, l = DOMINOES[d]
    return f"{h}:{l}"


def iter_bits(mask):
    while mask:
        bit = mask & -mask
        yield bit.bit_length() - 1
        mask -= bit


def is_called_raw(d, declaration):
    h, l = DOMINOES[d]
    if declaration in PIP_DECLARATIONS:
        return h == declaration or l == declaration
    if declaration == DOUBLES_TRUMP:
        return h == l
    if declaration == NO_TRUMP:
        return False
    raise ValueError(declaration)


def build_rule_tables():
    called_table = []
    led_table = []
    follow_masks = []
    key_table = []
    follow_relation = []
    comparison_relation = []

    for declaration in DECLARATIONS:
        called = tuple(
            is_called_raw(d, declaration) for d in range(N_DOMINOES)
        )
        led = tuple(
            CALLED_CONTEXT if called[d] else DOMINOES[d][0]
            for d in range(N_DOMINOES)
        )

        dec_follow_masks = []
        for q in range(8):
            mask = 0
            for d, (h, l) in enumerate(DOMINOES):
                ok = (
                    q == CALLED_CONTEXT
                    if called[d]
                    else q != CALLED_CONTEXT and (h == q or l == q)
                )
                if ok:
                    mask |= 1 << d
            dec_follow_masks.append(mask)

        dec_keys = []
        for q in range(8):
            row = []
            for d, (h, l) in enumerate(DOMINOES):
                powered = declaration != NO_TRUMP and called[d]
                follows_q = bool(dec_follow_masks[q] & (1 << d))
                tier = 2 if powered else 1 if follows_q else 0
                if declaration == DOUBLES_TRUMP and h == l:
                    rank = h
                elif h == l:
                    rank = 14
                else:
                    rank = h + l
                row.append(tier * 16 + rank if tier else 0)
            dec_keys.append(tuple(row))

        dec_follow_relation = tuple(
            tuple(
                int(bool(dec_follow_masks[led[e]] & (1 << x)))
                for x in range(N_DOMINOES)
            )
            for e in range(N_DOMINOES)
        )

        dec_comparison = []
        for g in range(N_DOMINOES):
            keys = dec_keys[led[g]]
            dec_comparison.append(
                tuple(
                    tuple(
                        0 if keys[x] < keys[y]
                        else 1 if keys[x] == keys[y]
                        else 2
                        for y in range(N_DOMINOES)
                    )
                    for x in range(N_DOMINOES)
                )
            )

        called_table.append(called)
        led_table.append(led)
        follow_masks.append(tuple(dec_follow_masks))
        key_table.append(tuple(dec_keys))
        follow_relation.append(dec_follow_relation)
        comparison_relation.append(tuple(dec_comparison))

    return (
        tuple(called_table),
        tuple(led_table),
        tuple(follow_masks),
        tuple(key_table),
        tuple(follow_relation),
        tuple(comparison_relation),
    )


(
    CALLED,
    LED,
    FOLLOW_MASK,
    TRICK_KEY,
    FOLLOW_RELATION,
    COMPARISON_RELATION,
) = build_rule_tables()


def resolve_trick(trick, declaration):
    assert len(trick) == 4
    q = LED[declaration][trick[0][1]]
    keys = tuple(TRICK_KEY[declaration][q][d] for _, d in trick)
    top = max(keys)
    assert keys.count(top) == 1
    winner_index = keys.index(top)
    winner = trick[winner_index][0]
    award = 1 + sum(COUNT_VALUE[d] for _, d in trick)
    return winner, award


def prose_winner_domino(dominoes, declaration):
    def called(d):
        h, l = DOMINOES[d]
        if declaration in PIP_DECLARATIONS:
            return h == declaration or l == declaration
        if declaration == DOUBLES_TRUMP:
            return h == l
        return False

    lead = dominoes[0]
    q = CALLED_CONTEXT if called(lead) else DOMINOES[lead][0]
    powered = tuple(
        d for d in dominoes if declaration != NO_TRUMP and called(d)
    )

    if powered:
        if declaration == DOUBLES_TRUMP:
            def power_order(d):
                return DOMINOES[d][0]
        else:
            trump = declaration

            def power_order(d):
                h, l = DOMINOES[d]
                if h == l:
                    return 1, 0
                return 0, l if h == trump else h

        winner = max(powered, key=power_order)
        assert sum(power_order(d) == power_order(winner) for d in powered) == 1
        return winner

    def follows_prose(d):
        h, l = DOMINOES[d]
        if q == CALLED_CONTEXT:
            return called(d)
        return not called(d) and (h == q or l == q)

    followers = tuple(d for d in dominoes if follows_prose(d))
    assert lead in followers

    def natural_order(d):
        h, l = DOMINOES[d]
        if h == l:
            return 1, 0
        return 0, l if h == q else h

    winner = max(followers, key=natural_order)
    assert sum(natural_order(d) == natural_order(winner) for d in followers) == 1
    return winner


def check_unique_winner_exhaustive():
    checked = 0
    universe = tuple(range(N_DOMINOES))
    for declaration in DECLARATIONS:
        for lead in universe:
            others = tuple(x for x in universe if x != lead)
            q = LED[declaration][lead]
            key_row = TRICK_KEY[declaration][q]
            for tail in combinations(others, 3):
                dominoes = (lead,) + tail
                keys = tuple(key_row[x] for x in dominoes)
                top = max(keys)
                assert keys.count(top) == 1
                assert dominoes[keys.index(top)] == prose_winner_domino(
                    dominoes, declaration
                )
                checked += 1
    assert checked == 737_100
    return checked


def constellation_signature(tiles, declaration):
    a, b, c, d = tiles
    signature = (
        COUNT_CODE[a]
        | COUNT_CODE[b] << 2
        | COUNT_CODE[c] << 4
        | COUNT_CODE[d] << 6
    )
    shift = 8
    follows_rel = FOLLOW_RELATION[declaration]
    comparisons = COMPARISON_RELATION[declaration]

    for i in range(4):
        x = tiles[i]
        for j in range(4):
            signature |= follows_rel[tiles[j]][x] << shift
            shift += 1

    for g in range(4):
        comparison = comparisons[tiles[g]]
        for i, j in UNORDERED_PAIRS_4:
            signature |= comparison[tiles[i]][tiles[j]] << shift
            shift += 2

    assert shift == 72
    return signature


def outcome_from_signature(signature):
    codes = tuple((signature >> (2 * i)) & 3 for i in range(4))
    assert all(code <= 2 for code in codes)
    award = 1 + 5 * sum(codes)

    comparison = [[1] * 4 for _ in range(4)]
    shift = 24
    for i, j in UNORDERED_PAIRS_4:
        value = (signature >> shift) & 3
        assert value in (0, 1, 2)
        comparison[i][j] = value
        comparison[j][i] = 2 - value if value != 1 else 1
        shift += 2

    winners = [
        i
        for i in range(4)
        if all(i == j or comparison[i][j] == 2 for j in range(4))
    ]
    assert len(winners) == 1
    return winners[0] & 1, award


def forced_outcome(tiles, declaration):
    q = LED[declaration][tiles[0]]
    key_row = TRICK_KEY[declaration][q]
    keys = tuple(key_row[d] for d in tiles)
    top = max(keys)
    assert keys.count(top) == 1
    winner_offset = keys.index(top)
    return winner_offset & 1, 1 + sum(COUNT_VALUE[d] for d in tiles)


def hold_pattern_index(tiles):
    base = tuple(sorted(tiles))
    position = {tile: i for i, tile in enumerate(base)}
    return PERMUTATION_INDEX[tuple(position[tile] for tile in tiles)]


def enumerate_constellations():
    classes = {}
    raw_positions = 0
    sample_lists = [[[] for _ in range(24)] for _ in range(9)]
    sample_sets = [[set() for _ in range(24)] for _ in range(9)]
    hard_all_called = [None] * 8
    hard_nt_four_doubles = None

    for declaration in DECLARATIONS:
        for live_set in combinations(range(N_DOMINOES), 4):
            for permutation_index, permutation in enumerate(HOLDER_PERMUTATIONS):
                tiles = tuple(live_set[permutation[i]] for i in range(4))
                signature = constellation_signature(tiles, declaration)
                info = classes.get(signature)

                if info is None:
                    outcome = forced_outcome(tiles, declaration)
                    bad = outcome_from_signature(signature) != outcome
                    classes[signature] = [
                        1,
                        outcome,
                        declaration,
                        tiles,
                        permutation_index,
                        bad,
                    ]
                else:
                    info[0] += 1

                bucket = sample_lists[declaration][permutation_index]
                bucket_seen = sample_sets[declaration][permutation_index]
                if len(bucket) < 32 and signature not in bucket_seen:
                    bucket.append((signature, tiles))
                    bucket_seen.add(signature)

                if declaration != NO_TRUMP:
                    if (
                        hard_all_called[declaration] is None
                        and all(CALLED[declaration][x] for x in tiles)
                    ):
                        hard_all_called[declaration] = (
                            signature,
                            tiles,
                            permutation_index,
                        )
                elif (
                    hard_nt_four_doubles is None
                    and all(IS_DOUBLE[x] for x in tiles)
                ):
                    hard_nt_four_doubles = (
                        signature,
                        tiles,
                        permutation_index,
                    )

                raw_positions += 1

    assert raw_positions == 4_422_600
    assert all(x is not None for x in hard_all_called)
    assert hard_nt_four_doubles is not None
    return (
        classes,
        raw_positions,
        sample_lists,
        hard_all_called,
        hard_nt_four_doubles,
    )


def reverse_construct(declaration, final_tiles):
    led = LED[declaration]
    follow_mask = FOLLOW_MASK[declaration]
    trick_key = TRICK_KEY[declaration]
    future = [1 << final_tiles[seat] for seat in range(4)]
    failed_states = set()
    reverse_tricks = [None] * 6
    nodes = 0

    def search(next_leader, depth):
        nonlocal nodes
        nodes += 1
        if depth == 6:
            return True

        state = (
            future[0],
            future[1],
            future[2],
            future[3],
            next_leader,
            depth,
        )
        if state in failed_states:
            return False

        unused = ALL_DOMINO_MASK ^ (
            future[0] | future[1] | future[2] | future[3]
        )
        leader_order = (
            next_leader,
            (next_leader + 2) & 3,
            (next_leader + 1) & 3,
            (next_leader + 3) & 3,
        )
        leader_tiles = list(iter_bits(unused))
        leader_tiles.sort(
            key=lambda d: (follow_mask[led[d]] & unused).bit_count()
        )

        for leader in leader_order:
            for leader_tile in leader_tiles:
                q = led[leader_tile]
                chosen = [-1] * 4
                chosen[leader] = leader_tile
                remaining = unused ^ (1 << leader_tile)
                candidates = [0] * 4

                for seat in range(4):
                    if seat == leader:
                        candidates[seat] = 1 << leader_tile
                    elif future[seat] & follow_mask[q]:
                        candidates[seat] = remaining & follow_mask[q]
                    else:
                        candidates[seat] = remaining

                if next_leader == leader:
                    winning_key = trick_key[q][leader_tile]
                    possible = True
                    for seat in range(4):
                        if seat == leader:
                            continue
                        lower = 0
                        for d in iter_bits(candidates[seat]):
                            if trick_key[q][d] < winning_key:
                                lower |= 1 << d
                        candidates[seat] = lower
                        if not lower:
                            possible = False
                            break
                    if not possible:
                        continue

                    seats = [s for s in range(4) if s != leader]
                    seats.sort(key=lambda s: candidates[s].bit_count())

                    def choose_losers(index, available):
                        if index == 3:
                            for seat in range(4):
                                future[seat] |= 1 << chosen[seat]
                            reverse_tricks[depth] = leader, tuple(chosen)
                            if search(leader, depth + 1):
                                return True
                            for seat in range(4):
                                future[seat] ^= 1 << chosen[seat]
                            return False

                        seat = seats[index]
                        for d in iter_bits(candidates[seat] & available):
                            chosen[seat] = d
                            if choose_losers(
                                index + 1, available ^ (1 << d)
                            ):
                                return True
                        return False

                    if choose_losers(0, remaining):
                        return True

                else:
                    leader_key = trick_key[q][leader_tile]
                    winner_tiles = [
                        d
                        for d in iter_bits(candidates[next_leader])
                        if trick_key[q][d] > leader_key
                    ]
                    winner_tiles.sort(
                        key=lambda d: trick_key[q][d], reverse=True
                    )

                    for winner_tile in winner_tiles:
                        chosen[next_leader] = winner_tile
                        winning_key = trick_key[q][winner_tile]
                        available = remaining ^ (1 << winner_tile)
                        local = []
                        possible = True

                        for seat in range(4):
                            if seat in (leader, next_leader):
                                continue
                            lower = 0
                            for d in iter_bits(candidates[seat] & available):
                                if trick_key[q][d] < winning_key:
                                    lower |= 1 << d
                            if not lower:
                                possible = False
                                break
                            local.append((seat, lower))

                        if not possible:
                            continue

                        local.sort(key=lambda item: item[1].bit_count())
                        seat0, mask0 = local[0]
                        seat1, mask1 = local[1]

                        for tile0 in iter_bits(mask0 & available):
                            chosen[seat0] = tile0
                            for tile1 in iter_bits(
                                mask1 & available & ~(1 << tile0)
                            ):
                                chosen[seat1] = tile1
                                for seat in range(4):
                                    future[seat] |= 1 << chosen[seat]
                                reverse_tricks[depth] = (
                                    leader,
                                    tuple(chosen),
                                )
                                if search(leader, depth + 1):
                                    return True
                                for seat in range(4):
                                    future[seat] ^= 1 << chosen[seat]

        failed_states.add(state)
        return False

    success = search(0, 0)
    if not success:
        return None, nodes
    result = tuple(reversed(reverse_tricks))
    assert all(trick is not None for trick in result)
    return result, nodes


def validate_simple_auction(initial_leader):
    shaker = (initial_leader - 1) & 3
    actors = tuple((shaker + 1 + i) & 3 for i in range(4))
    assert actors[0] == initial_leader
    actions = ((actors[0], 30),) + tuple((actor, None) for actor in actors[1:])
    high = None
    bidder = None

    for actor, bid in actions:
        if bid is None:
            continue
        assert 30 <= bid <= 41
        assert high is None or bid > high
        high = bid
        bidder = actor

    assert bidder == initial_leader and high == 30
    return shaker, actions


def play_is_legal(hand, trick, domino, declaration):
    if domino not in hand:
        return False, "not-held"
    if not trick:
        return True, "lead"

    q = LED[declaration][trick[0][1]]
    mask = FOLLOW_MASK[declaration][q]
    has_follower = any(mask & (1 << tile) for tile in hand)
    tile_follows = bool(mask & (1 << domino))

    if has_follower:
        return (
            tile_follows,
            "follow" if tile_follows else "illegal-slough",
        )
    return (
        True,
        "follow" if tile_follows else f"slough(void {q})",
    )


def replay_witness(declaration, final_tiles, preceding_tricks, target_signature):
    hands = []
    for seat in range(4):
        hand = {final_tiles[seat]}
        hand.update(trick[1][seat] for trick in preceding_tricks)
        assert len(hand) == 7
        hands.append(hand)

    assert sum(len(hand) for hand in hands) == 28
    assert set().union(*hands) == set(range(N_DOMINOES))
    initial_hands = tuple(tuple(sorted(hand)) for hand in hands)
    initial_leader = preceding_tricks[0][0]
    shaker, auction = validate_simple_auction(initial_leader)
    leader = initial_leader
    scores = [0, 0]
    script_tricks = []
    all_tricks = tuple(preceding_tricks) + ((0, tuple(final_tiles)),)

    for trick_number, (expected_leader, tile_by_seat) in enumerate(
        all_tricks, 1
    ):
        assert leader == expected_leader
        trick = []
        annotated = []

        for offset in range(4):
            seat = (leader + offset) & 3
            domino = tile_by_seat[seat]
            legal, mode = play_is_legal(
                hands[seat], trick, domino, declaration
            )
            assert legal
            hands[seat].remove(domino)
            trick.append((seat, domino))
            annotated.append((seat, domino, mode))

        winner, award = resolve_trick(tuple(trick), declaration)
        scores[winner & 1] += award
        q = LED[declaration][trick[0][1]]
        script_tricks.append(
            (
                trick_number,
                leader,
                q,
                tuple(annotated),
                winner,
                award,
            )
        )
        leader = winner

    assert all(not hand for hand in hands)
    assert sum(scores) == 42
    assert constellation_signature(final_tiles, declaration) == target_signature
    final_team, final_award = forced_outcome(final_tiles, declaration)
    assert script_tricks[-1][5] == final_award
    assert (script_tricks[-1][4] & 1) == final_team

    return {
        "declaration": declaration,
        "shaker": shaker,
        "auction": auction,
        "initial_hands": initial_hands,
        "initial_leader": initial_leader,
        "tricks": tuple(script_tricks),
        "scores": tuple(scores),
    }


def select_sample_cases(
    classes,
    sample_lists,
    hard_all_called,
    hard_nt_four_doubles,
):
    class_keys = sorted(classes)
    class_id = {signature: i for i, signature in enumerate(class_keys)}
    selected = set()
    cases = []
    coverage = {}

    for declaration in DECLARATIONS:
        for permutation_index in range(24):
            choice = None
            for signature, final_tiles in sample_lists[declaration][
                permutation_index
            ]:
                if signature not in selected:
                    choice = (
                        signature,
                        declaration,
                        final_tiles,
                        permutation_index,
                        "coverage",
                    )
                    break
            assert choice is not None
            selected.add(choice[0])
            cases.append(choice)
            coverage[(declaration, permutation_index)] = choice

    hard_embeddings = []
    for declaration, item in enumerate(hard_all_called):
        signature, final_tiles, permutation_index = item
        hard_embeddings.append(
            (
                signature,
                declaration,
                final_tiles,
                permutation_index,
                "all-called",
            )
        )

    signature, final_tiles, permutation_index = hard_nt_four_doubles
    hard_embeddings.append(
        (
            signature,
            NO_TRUMP,
            final_tiles,
            permutation_index,
            "NT-four-doubles",
        )
    )

    for case in hard_embeddings:
        if case[0] not in selected:
            selected.add(case[0])
            cases.append(case)

    for signature in class_keys:
        if len(cases) >= 600:
            break
        if signature in selected:
            continue
        info = classes[signature]
        cases.append(
            (
                signature,
                info[2],
                info[3],
                info[4],
                "fill",
            )
        )
        selected.add(signature)

    assert len(cases) == len(selected) == 600
    assert {case[1] for case in cases} == set(DECLARATIONS)
    assert {case[3] for case in cases} == set(range(24))
    assert len(coverage) == 9 * 24
    return class_keys, class_id, tuple(cases), coverage, tuple(hard_embeddings)


def format_histogram(histogram):
    return " ".join(
        f"{size}:{histogram[size]}" for size in sorted(histogram)
    )


def format_auction(actions):
    return ", ".join(
        f"S{seat}:{'pass' if bid is None else f'P({bid})'}"
        for seat, bid in actions
    )


def print_hand_script(number, class_label, record):
    declaration = record["declaration"]
    print(
        f"HAND SCRIPT {number:02d} class={class_label} "
        f"declaration={DECLARATION_NAME[declaration]}"
    )
    print(
        f"  auction: shaker=S{record['shaker']}; "
        f"{format_auction(record['auction'])}; "
        f"bidder/first leader=S{record['initial_leader']}"
    )

    for seat, hand in enumerate(record["initial_hands"]):
        print(
            f"  deal S{seat}: "
            + " ".join(domino_name(domino) for domino in hand)
        )

    for trick_number, leader, q, plays, winner, award in record["tricks"]:
        play_text = " | ".join(
            f"S{seat}:{domino_name(domino)}[{mode}]"
            for seat, domino, mode in plays
        )
        print(
            f"  T{trick_number} leader=S{leader} context={q}: "
            f"{play_text} -> winner=S{winner}, award={award}"
        )

    print(
        f"  final partnership scores: even={record['scores'][0]}, "
        f"odd={record['scores'][1]}"
    )
    print(f"PASS script {class_label}")


def main():
    unique_cases = check_unique_winner_exhaustive()
    print(f"PASS unique-winner {unique_cases}")

    (
        classes,
        raw_positions,
        sample_lists,
        hard_all_called,
        hard_nt_four_doubles,
    ) = enumerate_constellations()

    class_keys = sorted(classes)
    class_id = {signature: i for i, signature in enumerate(class_keys)}
    histogram = Counter(info[0] for info in classes.values())
    outcome_failures = sum(bool(info[5]) for info in classes.values())

    print(f"REALIZABLE CLASS COUNT {len(classes)}")
    print(f"RAW ORIENTED POSITIONS {raw_positions}")
    print(f"DISTINCT CLASS SIZES {len(histogram)}")
    print("CLASS SIZE DISTRIBUTION " + format_histogram(histogram))
    print(
        f"PASS census sums classes={sum(histogram.values())} "
        f"positions={sum(s * n for s, n in histogram.items())}"
    )

    reflected = {}
    for signature in class_keys:
        info = classes[signature]
        final_tiles = info[3]
        reflected_signature = constellation_signature(
            (
                final_tiles[0],
                final_tiles[3],
                final_tiles[2],
                final_tiles[1],
            ),
            info[2],
        )
        assert reflected_signature in classes
        reflected[signature] = reflected_signature

    for signature in class_keys:
        assert reflected[reflected[signature]] == signature

    fixed_reflections = sum(
        reflected[signature] == signature for signature in class_keys
    )
    reflection_orbits = (len(class_keys) + fixed_reflections) // 2
    assert reflection_orbits == 15_680
    assert fixed_reflections == 163

    folded_histogram = Counter()
    folded_seen = set()

    for signature in class_keys:
        if signature in folded_seen:
            continue
        partner = reflected[signature]
        folded_seen.add(signature)
        folded_seen.add(partner)

        if partner == signature:
            assert classes[signature][0] % 2 == 0
            folded_size = classes[signature][0] // 2
        else:
            assert classes[signature][0] == classes[partner][0]
            folded_size = classes[signature][0]
        folded_histogram[folded_size] += 1

    assert sum(folded_histogram.values()) == reflection_orbits
    assert (
        sum(size * count for size, count in folded_histogram.items())
        == raw_positions // 2
    )

    print(
        "AUDIT extra 1<->3 opponent reflection "
        "(not part of literal ~): "
        f"orbits={reflection_orbits} fixed={fixed_reflections} "
        f"position-orbits={raw_positions // 2}"
    )
    print(
        "AUDIT OPPONENT-FOLDED CLASS SIZE DISTRIBUTION "
        + format_histogram(folded_histogram)
    )

    outcome_lines = []
    for signature in class_keys:
        identifier = class_id[signature]
        info = classes[signature]
        status = "FAIL" if info[5] else "PASS"
        team, award = info[1]
        outcome_lines.append(
            f"{status} outcome C{identifier:05d} "
            f"size={info[0]} team={team} award={award}"
        )
    sys.stdout.write("\n".join(outcome_lines) + "\n")

    if outcome_failures:
        print(f"FAIL outcome-constancy classes={outcome_failures}")
        return 1
    print(f"PASS outcome-constancy {len(classes)}/{len(classes)}")

    total_nodes = 0
    maximum_nodes = 0
    all_called_final_classes = 0
    two_called_at_trick6 = 0

    for signature in class_keys:
        identifier = class_id[signature]
        info = classes[signature]
        declaration = info[2]
        final_tiles = info[3]
        preceding_tricks, nodes = reverse_construct(
            declaration, final_tiles
        )

        if preceding_tricks is None:
            print(f"FAIL exhaustive reach C{identifier:05d} nodes={nodes}")
            return 1

        replay_witness(
            declaration,
            final_tiles,
            preceding_tricks,
            signature,
        )
        total_nodes += nodes
        maximum_nodes = max(maximum_nodes, nodes)

        if all(CALLED[declaration][x] for x in final_tiles):
            all_called_final_classes += 1

        trick6_tiles = preceding_tricks[5][1]
        if any(
            CALLED[declaration][final_tiles[seat]]
            and CALLED[declaration][trick6_tiles[seat]]
            for seat in range(4)
        ):
            two_called_at_trick6 += 1

    print(
        f"PASS exhaustive reachability {len(class_keys)}/{len(class_keys)} "
        f"search_nodes={total_nodes} max_per_class={maximum_nodes}"
    )
    print(
        "PASS hard-case coverage "
        f"all-called-final-classes={all_called_final_classes} "
        "primary-NT-representatives=0 "
        "witnesses-with-seat-retaining-two-called-at-trick6="
        f"{two_called_at_trick6}"
    )
    assert all_called_final_classes > 0
    assert two_called_at_trick6 > 0

    (
        selected_keys,
        selected_ids,
        sample_cases,
        coverage,
        hard_embeddings,
    ) = select_sample_cases(
        classes,
        sample_lists,
        hard_all_called,
        hard_nt_four_doubles,
    )
    assert selected_keys == class_keys
    assert selected_ids == class_id

    sample_records = {}
    sample_nodes = 0
    sample_lines = []

    for signature, declaration, final_tiles, pattern, reason in sample_cases:
        assert hold_pattern_index(final_tiles) == pattern
        preceding_tricks, nodes = reverse_construct(
            declaration, final_tiles
        )

        if preceding_tricks is None:
            print(
                f"FAIL reach C{class_id[signature]:05d} "
                f"declaration={DECLARATION_NAME[declaration]} "
                f"hold=P{pattern:02d}"
            )
            return 1

        record = replay_witness(
            declaration,
            final_tiles,
            preceding_tricks,
            signature,
        )
        sample_nodes += nodes
        sample_records[
            (signature, declaration, final_tiles, pattern)
        ] = record
        sample_lines.append(
            f"PASS reach C{class_id[signature]:05d} "
            f"declaration={DECLARATION_NAME[declaration]} "
            f"hold=P{pattern:02d} reason={reason} nodes={nodes}"
        )

    sys.stdout.write("\n".join(sample_lines) + "\n")
    print(
        f"PASS sample coverage cases={len(sample_cases)} "
        "declarations=9 hold-patterns=24 "
        f"declaration-x-pattern-cells={len(coverage)} "
        f"nodes={sample_nodes}"
    )

    script_cases = []
    script_seen = set()

    for case in hard_embeddings:
        key = case[:4]
        if key not in script_seen:
            script_seen.add(key)
            script_cases.append(case)

    for pattern in range(24):
        for declaration in DECLARATIONS:
            case = coverage[(declaration, pattern)]
            key = case[:4]
            if key not in script_seen:
                script_seen.add(key)
                script_cases.append(case)
            if len(script_cases) == 25:
                break
        if len(script_cases) == 25:
            break

    assert len(script_cases) == 25

    for number, case in enumerate(script_cases, 1):
        signature, declaration, final_tiles, pattern, _ = case
        key = (signature, declaration, final_tiles, pattern)
        record = sample_records.get(key)

        if record is None:
            preceding_tricks, _ = reverse_construct(
                declaration, final_tiles
            )
            assert preceding_tricks is not None
            record = replay_witness(
                declaration,
                final_tiles,
                preceding_tricks,
                signature,
            )

        print_hand_script(
            number,
            f"C{class_id[signature]:05d}",
            record,
        )

    print("PASS human-readable scripts 25")
    print(
        "PASS R1 every realizable class has a replayed "
        "legal full-hand witness"
    )
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except SystemExit:
        raise
    except BaseException as exc:
        print(f"FAIL fatal {type(exc).__name__}: {exc}")
        traceback.print_exc()
        raise SystemExit(1)
