"""Independent Straight 42 rules and information-state validation."""

TILES = [(hi, lo) for hi in range(7) for lo in range(hi + 1)]


def called(tile, decl):
    hi, lo = TILES[tile]
    return decl in (hi, lo) if decl < 7 else decl == 7 and hi == lo


def context(tile, decl):
    return "trump" if called(tile, decl) else TILES[tile][0]


def follows(tile, led, decl):
    return called(tile, decl) if led == "trump" else not called(tile, decl) and led in TILES[tile]


def legal_tiles(hand, trick, decl):
    if not trick:
        return sorted(hand)
    following = [t for t in hand if follows(t, context(trick[0][1], decl), decl)]
    return sorted(following or hand)


def winner(trick, decl):
    led = context(trick[0][1], decl)

    def strength(play):
        t = play[1]
        hi, lo = TILES[t]
        tier = 2 if called(t, decl) else 1 if follows(t, led, decl) else 0
        rank = hi if hi == lo and decl == 7 else 12 if hi == lo else hi + lo
        return tier, rank

    return max(trick, key=strength)[0]


def trick_points(trick):
    return 1 + sum(sum(TILES[t]) if sum(TILES[t]) in (5, 10) else 0 for _, t in trick)


def replay_record(hands, record, decl, bidder):
    remaining = [set(h) for h in hands]
    assert sorted(t for hand in hands for t in hand) == list(range(28))
    points = [0, 0]
    lead, trick = bidder, []
    for actor, tile in zip(record[::2], record[1::2]):
        assert actor == (lead + len(trick)) % 4
        assert tile in legal_tiles(remaining[actor], trick, decl)
        remaining[actor].remove(tile)
        trick.append((actor, tile))
        if len(trick) == 4:
            lead = winner(trick, decl)
            points[lead % 2] += trick_points(trick)
            trick = []
    return points, lead, remaining, trick


def _request_fields(req):
    if not isinstance(req, dict):
        raise ValueError("request must be an object")
    for key in ("decl", "bid", "bidder", "seat"):
        if type(req.get(key)) is not int:
            raise ValueError(key + " must be an integer")
    decl, bid, bidder, viewer = (req[k] for k in ("decl", "bid", "bidder", "seat"))
    if decl not in (*range(8), 9) or not 30 <= bid <= 42:
        raise ValueError("invalid declaration or bid")
    if bidder not in range(4) or viewer not in range(4):
        raise ValueError("invalid bidder or seat")

    hand = req.get("hand")
    plays = req.get("plays", [])
    if not isinstance(hand, list) or len(hand) != 7:
        raise ValueError("hand needs seven tile ids")
    if any(type(tile) is not int or tile not in range(28) for tile in hand):
        raise ValueError("hand needs tile ids 0..27")
    if len(set(hand)) != 7:
        raise ValueError("duplicate hand tile")
    if not isinstance(plays, list) or len(plays) % 2 or len(plays) > 56:
        raise ValueError("invalid record length")
    if any(type(value) is not int or value < 0 for value in plays):
        raise ValueError("record needs unsigned integers")
    return decl, bidder, viewer, hand, plays


def _hidden_completion_exists(viewer, own_remaining, played, sizes, void_tiles):
    """Exact three-hand capacity DP: at most 21 tiles and 8^3 states."""
    others = [seat for seat in range(4) if seat != viewer]
    capacities = tuple(sizes[seat] for seat in others)
    unseen = sorted(set(range(28)) - played - own_remaining)
    if len(unseen) != sum(capacities):
        return False

    states = {(0, 0, 0)}
    for tile in unseen:
        next_states = set()
        for counts in states:
            for slot, seat in enumerate(others):
                if counts[slot] < capacities[slot] and tile not in void_tiles[seat]:
                    updated = list(counts)
                    updated[slot] += 1
                    next_states.add(tuple(updated))
        states = next_states
        if not states:
            return False
    return capacities in states


def information_state(req):
    """Derive a lawful current state from one original hand and public play."""
    decl, bidder, viewer, original_hand, record = _request_fields(req)
    own_remaining = set(original_hand)
    played = set()
    void_tiles = [set() for _ in range(4)]
    play_counts = [0, 0, 0, 0]
    points = [0, 0]
    leader, trick, completed = bidder, [], 0

    for actor, tile in zip(record[::2], record[1::2]):
        if actor not in range(4) or tile not in range(28):
            raise ValueError("invalid record actor or tile")
        if actor != (leader + len(trick)) % 4:
            raise ValueError("record violates turn order")
        if tile in played:
            raise ValueError("record repeats a tile")
        if play_counts[actor] >= 7:
            raise ValueError("a seat plays more than seven tiles")
        if tile in void_tiles[actor]:
            raise ValueError("record contradicts a revealed void")

        if actor == viewer:
            if tile not in own_remaining:
                raise ValueError("own historical tile is not in the original hand")
            if tile not in legal_tiles(own_remaining, trick, decl):
                raise ValueError("own historical play is illegal")
            own_remaining.remove(tile)
        elif tile in original_hand:
            raise ValueError("another seat played own tile")

        if trick:
            led = context(trick[0][1], decl)
            if not follows(tile, led, decl):
                void_tiles[actor].update(t for t in range(28) if follows(t, led, decl))

        played.add(tile)
        play_counts[actor] += 1
        trick.append((actor, tile))
        if len(trick) == 4:
            leader = winner(trick, decl)
            points[leader % 2] += trick_points(trick)
            trick = []
            completed += 1

    if completed == 7:
        raise ValueError("hand is complete")
    if viewer != (leader + len(trick)) % 4:
        raise ValueError("not this seat's turn")

    sizes = [7 - count for count in play_counts]
    if len(own_remaining) != sizes[viewer]:
        raise ValueError("own hand and public record disagree")
    if not _hidden_completion_exists(viewer, own_remaining, played, sizes, void_tiles):
        raise ValueError("public record has no hidden-hand completion")

    legal = legal_tiles(own_remaining, trick, decl)
    if not legal:
        raise ValueError("position has no legal move")
    return {"legal": legal, "leader": leader, "points": points, "trick": completed + 1}
