"""EXPLORATORY — below every evidentiary tier, cited by nothing above it.

walt on one page: a ladder of best responses that bottoms out at random.

    decide(seat, hand, public, level)
        sample deals consistent with what `seat` can see (never the real hand)
        return the legal tile whose value is best for seat's team

    value(me, public, deals, level)             -> how many of `deals` make
        bid decided?            count it
        my turn                 one tile for ALL deals I can't tell apart (max / min)
        another seat's turn     ask it what it plays in each deal:
                                  level 0: a uniformly random legal tile
                                  level k: decide(that seat, its hand in the deal, public, k - 1)
                                group the deals by the tile played and add up the groups

Live walt is decide(me, ..., level=1). Exact integer counts; no floats.

Memo switches (the ablation):
  memo_value   remember value(me, public, deals, level)  - the position memo
  memo_decide  remember decide(level, seat, hand, public) - walt's policy cache
With both off nothing is ever replayed: every random draw and every modeled
decision is made fresh on every visit.

Rules: straight 42, pip trumps, bid 30 held by the team of seats 1 and 3,
following the suit algebra in walt/walt/src/rules/rules.rs:
  called set   = tiles carrying the trump pip
  led suit     = called if the led tile is a trump, else the led tile's high pip
  follows      = tile is in the led suit's effective set (a natural suit minus trumps)
  trick key    = (tier, rank): tier 2 trump, 1 follows, 0 slough;
                 rank = 12 for a double, else pip sum; highest key wins
  points       = 1 per trick + 5 for tiles summing to 5, 10 for tiles summing to 10
"""

import random

TILES = [(h, l) for h in range(7) for l in range(h + 1)]  # 28 tiles, hi >= lo
FULL = (1 << 28) - 1
BID = 30
CALLED = 7  # the context index for "trump was led"


def tiles_of(mask):
    return [t for t in range(28) if mask >> t & 1]


def count(t):
    s = TILES[t][0] + TILES[t][1]
    return s if s in (5, 10) else 0


class Rules:
    """The suit algebra for one pip-trump declaration, as lookup tables."""

    def __init__(self, trump):
        self.trump = trump
        called = sum(1 << t for t, (h, l) in enumerate(TILES) if trump in (h, l))
        # follow[q]: the tiles that follow context q (0..6 natural, 7 called)
        self.follow = [
            sum(1 << t for t, (h, l) in enumerate(TILES) if q in (h, l)) & ~called
            for q in range(7)
        ] + [called]
        self.led = [CALLED if called >> t & 1 else TILES[t][0] for t in range(28)]
        self.key = [
            [
                (
                    2 if called >> t & 1 else 1 if self.follow[q] >> t & 1 else 0,
                    12 if TILES[t][0] == TILES[t][1] else TILES[t][0] + TILES[t][1],
                )
                for t in range(28)
            ]
            for q in range(8)
        ]

    def legal(self, hand, plays):
        if not plays:
            return hand
        must = hand & self.follow[self.led[plays[0]]]
        return must or hand


# Public state: (played mask, leader, current-trick plays, points T1, points T0).
# Everyone sees it; the hands are hidden.


def to_act(state):
    return (state[1] + len(state[2])) % 4


def decided(state):
    if state[3] >= BID:
        return True
    if state[4] > 42 - BID:
        return False
    return None


def step(rules, state, tile):
    played, leader, plays, t1, t0 = state
    plays = plays + (tile,)
    played |= 1 << tile
    if len(plays) < 4:
        return (played, leader, plays, t1, t0)
    q = rules.led[plays[0]]
    win = max(range(4), key=lambda i: rules.key[q][plays[i]])
    winner = (leader + win) % 4
    pts = 1 + sum(count(t) for t in plays)
    if winner % 2 == 1:
        t1 += pts
    else:
        t0 += pts
    return (played, winner, (), t1, t0)


def hand_sizes(state):
    played, leader, plays = state[0], state[1], state[2]
    done = (bin(played).count("1") - len(plays)) // 4
    sizes = [7 - done] * 4
    for i in range(len(plays)):
        sizes[(leader + i) % 4] -= 1
    return sizes


def sample_deals(seat, hand, state, n, rng, voids=None):
    """Uniform deals consistent with the public record and `seat`'s own hand
    (and, if given, observed voids). The real deal is never consulted."""
    unseen = tiles_of(FULL & ~state[0] & ~hand)
    sizes = hand_sizes(state)
    others = [s for s in range(4) if s != seat]
    deals = []
    while len(deals) < n:
        rng.shuffle(unseen)
        deal, at, ok = [0, 0, 0, 0], 0, True
        deal[seat] = hand
        for s in others:
            deal[s] = sum(1 << t for t in unseen[at : at + sizes[s]])
            at += sizes[s]
            if voids and deal[s] & voids[s]:
                ok = False
                break
        if ok:
            deals.append(tuple(deal))
    return deals


class Walt:
    def __init__(self, rules, n_outer=50, n0=8, memo_value=False, memo_decide=False,
                 seed=0, inner_seed=None):
        self.rules = rules
        self.n_outer, self.n0 = n_outer, n0
        self.memo_value, self.memo_decide = memo_value, memo_decide
        # Two streams: the real seat's own deals, and everything inside the
        # model (modeled seats' deals and the random bottom).
        self.root_rng = random.Random(seed)
        self.rng = random.Random(seed if inner_seed is None else inner_seed)
        self.values, self.decisions = {}, {}
        self.nodes = 0

    def decide(self, seat, hand, state, level, voids=None, detail=False):
        key = (level, seat, hand, state)
        if self.memo_decide and key in self.decisions and not detail:
            return self.decisions[key]
        n = self.n_outer if voids is not None else self.n0
        rng = self.root_rng if voids is not None else self.rng
        deals = tuple(sample_deals(seat, hand, state, n, rng, voids))
        maximize = seat % 2 == 1
        options = []
        for t in tiles_of(self.rules.legal(hand, state[2])):
            made = self.value(seat, step(self.rules, state, t), deals, level)
            options.append((t, made))
        best = (max if maximize else min)(options, key=lambda o: o[1])[0]
        if self.memo_decide:
            self.decisions[key] = best
        return (best, options, len(deals)) if detail else best

    def value(self, me, state, deals, level):
        """Number of `deals` in which the bid is made from here."""
        done = decided(state)
        if done is not None:
            return len(deals) if done else 0
        key = (me, state, deals, level)
        if self.memo_value and key in self.values:
            return self.values[key]
        self.nodes += 1
        seat = to_act(state)
        played, plays = state[0], state[2]
        if seat == me:
            maximize = me % 2 == 1
            hand = deals[0][me] & ~played  # my hand is the same in every deal
            best = None
            for t in tiles_of(self.rules.legal(hand, plays)):
                v = self.value(me, step(self.rules, state, t), deals, level)
                if best is None or (v > best if maximize else v < best):
                    best = v
                if best == (len(deals) if maximize else 0):
                    break  # cannot do better
            result = best
        else:
            groups = {}
            for deal in deals:
                hand = deal[seat] & ~played
                legal = tiles_of(self.rules.legal(hand, plays))
                if len(legal) == 1:
                    move = legal[0]
                elif level == 0:
                    move = self.rng.choice(legal)
                else:
                    move = self.decide(seat, hand, state, level - 1)
                groups.setdefault(move, []).append(deal)
            result = sum(
                self.value(me, step(self.rules, state, m), tuple(g), level)
                for m, g in groups.items()
            )
        if self.memo_value:
            self.values[key] = result
        return result


def deal_and_contract(seed):
    """Deal, then the battery's forced contract: the seat/pip-trump with the
    most trumps, then the trump double, then the most doubles. Hands are
    rotated so the bidder sits at seat 1 (team T1 = seats 1 and 3) and leads."""
    rng = random.Random(seed)
    tiles = list(range(28))
    rng.shuffle(tiles)
    phys = [sum(1 << t for t in tiles[7 * i : 7 * i + 7]) for i in range(4)]
    best = None
    for s in range(4):
        for p in range(7):
            mine = [TILES[t] for t in tiles_of(phys[s])]
            score = (
                sum(p in d for d in mine),
                (p, p) in mine,
                sum(h == l for h, l in mine),
            )
            if best is None or score > best[0]:
                best = (score, s, p)
    _, bidder, trump = best
    hands = [phys[(bidder + i + 3) % 4] for i in range(4)]
    return Rules(trump), hands


def observe_void(rules, voids, seat, plays, tile):
    if plays and not rules.follow[rules.led[plays[0]]] >> tile & 1:
        voids[seat] |= rules.follow[rules.led[plays[0]]]
