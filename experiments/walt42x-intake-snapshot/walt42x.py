"""User-supplied walt42x, preserved as the NumPy reference (2026-09-29).

Best response over sampled deals; delta=0 integrates the assumed policy.
Original algorithm and defaults; comments condensed. See README for audit.
"""
import sys
import numpy as np
import numpy as xp

TILES = [(h, l) for h in range(7) for l in range(h + 1)]
BID, DEALS, INNER_DEALS = 30, 30, 8
BITS = xp.arange(28)
HI = xp.array([h for h, l in TILES])
LO = xp.array([l for h, l in TILES])
POINTS = xp.where((HI + LO == 5) | (HI + LO == 10), HI + LO, 0)

def name(t):
    return "%d-%d" % TILES[t]

class Rules:
    def __init__(self, trump):
        trumps = sum(1 << t for t, d in enumerate(TILES) if trump in d)
        suit = [sum(1 << t for t, d in enumerate(TILES) if q in d) & ~trumps
                for q in range(7)] + [trumps]
        self.suit = xp.array(suit, dtype=xp.int64)
        self.lead = xp.array([7 if trumps >> t & 1 else TILES[t][0] for t in range(28)])
        pips = xp.where(HI == LO, 12, HI + LO)
        tier = xp.stack([xp.where((trumps >> BITS) & 1, 2, xp.where((suit[q] >> BITS) & 1, 1, 0))
                         for q in range(8)])
        self.strength = tier * 20 + pips

    def legal(self, hand, trick, tlen):
        follow = hand & self.suit[self.lead[trick[:, 0]]]
        mask = xp.where(tlen == 0, hand, xp.where(follow != 0, follow, hand))
        return ((mask[:, None] >> BITS) & 1).astype(bool)

class Pub:
    def __init__(self, played, leader, trick, tlen, t1, t0):
        self.played, self.leader, self.trick = played, leader, trick
        self.tlen, self.t1, self.t0 = tlen, t1, t0

    def take(self, i):
        return Pub(self.played[i], self.leader[i], self.trick[i], self.tlen[i], self.t1[i], self.t0[i])

    def turn(self):
        return (self.leader + self.tlen) % 4

    def outcome(self):
        return self.t1 >= BID, self.t0 > 42 - BID

    def play(self, rules, t):
        n = len(t)
        trick = self.trick.copy()
        trick[xp.arange(n), self.tlen] = t
        tlen, played = self.tlen + 1, self.played | (1 << t)
        full = tlen == 4
        pos = rules.strength[rules.lead[trick[:, 0]][:, None], trick].argmax(1)
        winner = (self.leader + pos) % 4
        won = 1 + POINTS[trick].sum(1)
        t1 = self.t1 + xp.where(full & (winner % 2 == 1), won, 0)
        t0 = self.t0 + xp.where(full & (winner % 2 == 0), won, 0)
        return Pub(played, xp.where(full, winner, self.leader),
                   xp.where(full[:, None], 0, trick), xp.where(full, 0, tlen), t1, t0)

def sample(seat, hand, played, leader, tlen, n, rng, voids):
    unseen = [t for t in range(28) if not (played | hand) >> t & 1]
    size = [7 - (bin(played).count("1") - tlen) // 4] * 4
    for i in range(tlen):
        size[(leader + i) % 4] -= 1
    deals = []
    while len(deals) < n:
        rng.shuffle(unseen)
        deal, at = [0] * 4, 0
        deal[seat] = hand
        for s in range(4):
            if s != seat:
                deal[s] = sum(1 << t for t in unseen[at:at + size[s]])
                at += size[s]
        if not any(deal[s] & voids[s] for s in range(4) if s != seat):
            deals.append(deal)
    return xp.array(deals, dtype=xp.int64)

def random_policy(rules, seat, hand, pub, rng, voids=None):
    L = rules.legal(hand, pub.trick, pub.tlen).astype(float)
    return L / L.sum(1, keepdims=True)

def flip(P, rng):
    c = P.cumsum(1)
    r = xp.asarray(rng.random(len(P)))[:, None] * c[:, -1:]
    idx = (c <= r).sum(1)
    Q = xp.zeros_like(P)
    Q[xp.arange(len(P)), idx] = 1
    return Q

def search(rules, me, pub, deals, assume, rng, delta=0.0):
    N = len(deals)
    node = pub
    tri_node, tri_deal, tri_w = xp.zeros(N, dtype=xp.int64), xp.arange(N), xp.ones(N)
    plies = []
    parent = tile = None
    while True:
        made, sett = node.outcome()
        term = made | sett
        leaf = xp.zeros(len(term))
        m = made[tri_node]
        xp.add.at(leaf, tri_node[m], tri_w[m])
        plies.append((parent, tile, term, node.turn() == me, leaf))
        if term.all():
            break
        keep = ~term[tri_node]
        tn, td, tw = tri_node[keep], tri_deal[keep], tri_w[keep]
        tpub = node.take(tn)
        tseat = tpub.turn()
        thand = deals[td, tseat] & ~tpub.played
        mine = tseat == me
        P = rules.legal(thand, tpub.trick, tpub.tlen).astype(float)
        oth = ~mine
        if oth.any():
            Po = assume(rules, tseat[oth], thand[oth], tpub.take(oth), rng)
            small = tw[oth] * xp.where(Po > 0, Po, xp.inf).min(1) < delta
            if small.any():
                Po[small] = flip(Po[small], rng)
            P[oth] = Po
        ti, t = xp.nonzero(P)
        cw = tw[ti] * xp.where(mine[ti], 1.0, P[ti, t])
        ukey, inv = xp.unique(tn[ti] * 28 + t, return_inverse=True)
        parent, tile = ukey // 28, ukey % 28
        node = node.take(parent).play(rules, tile)
        tri_node, tri_deal, tri_w = inv, td[ti], cw
    V = plies[-1][4]
    for d in range(len(plies) - 1, 1, -1):
        parent, _, _, _, _ = plies[d]
        _, _, pterm, pme, pleaf = plies[d - 1]
        n = len(pterm)
        s = xp.zeros(n)
        xp.add.at(s, parent, V)
        if me % 2:
            x = xp.full(n, -xp.inf)
            xp.maximum.at(x, parent, V)
        else:
            x = xp.full(n, xp.inf)
            xp.minimum.at(x, parent, V)
        V = xp.where(pterm, pleaf, xp.where(pme, x, s))
    return dict(zip(plies[1][1].tolist(), V.tolist()))

def walt(assume, n=INNER_DEALS, delta=0.0):
    def policy(rules, seat, hand, pub, rng, voids=(0, 0, 0, 0)):
        P = xp.zeros((len(seat), 28))
        for i in range(len(seat)):
            s, h = int(seat[i]), int(hand[i])
            deals = sample(s, h, int(pub.played[i]), int(pub.leader[i]), int(pub.tlen[i]), n, rng, voids)
            made = search(rules, s, pub.take(slice(i, i + 1)), deals, assume, rng, delta)
            P[i, (max if s % 2 else min)(made, key=made.get)] = 1
        return P
    return policy

def main(seed, delta, level):
    rng = np.random.default_rng(seed)
    order = list(range(28))
    rng.shuffle(order)
    hands = [sum(1 << t for t in order[7 * i:7 * i + 7]) for i in range(4)]
    tiles = lambda m: [t for t in range(28) if m >> t & 1]
    score, bidder, trump = max(
        ((sum(p in TILES[t] for t in tiles(hands[s])), hands[s] >> TILES.index((p, p)) & 1,
          sum(TILES[t][0] == TILES[t][1] for t in tiles(hands[s]))), s, p)
        for s in range(4) for p in range(7))
    hands = [hands[(bidder + i + 3) % 4] for i in range(4)]
    rules = Rules(trump)
    print("trump %ds, seats 1+3 bid %d, seat 1 leads   (L%d, delta=%g)" % (trump, BID, level, delta))
    for s in range(4):
        print("  seat %d: %s" % (s, " ".join(name(t) for t in tiles(hands[s]))))
    L0 = walt(random_policy, INNER_DEALS, delta)
    L1 = walt(L0, DEALS, delta)
    PLAYER = L1 if level else walt(random_policy, DEALS, delta)
    pub = Pub(*(xp.array([v], dtype=xp.int64) for v in (0, 1)), xp.zeros((1, 4), dtype=xp.int64),
              *(xp.array([v], dtype=xp.int64) for v in (0, 0, 0)))
    voids = [0] * 4
    while not any(int(o[0]) for o in pub.outcome()):
        seat, tlen = int(pub.turn()[0]), int(pub.tlen[0])
        hand = hands[seat] & ~int(pub.played[0])
        P = PLAYER(rules, xp.array([seat]), xp.array([hand], dtype=xp.int64), pub, rng, voids)
        t = int(P[0].argmax())
        if tlen:
            led = int(rules.lead[int(pub.trick[0, 0])])
            if not int(rules.suit[led]) >> t & 1:
                voids[seat] |= int(rules.suit[led])
        pub = pub.play(rules, xp.array([t]))
        print("seat %d plays %s" % (seat, name(t)), end="")
        print("   -> T1 %d, T0 %d" % (int(pub.t1[0]), int(pub.t0[0])) if not int(pub.tlen[0]) else "", flush=True)
    print("made" if int(pub.outcome()[0][0]) else "set")

if __name__ == "__main__":
    a = sys.argv[1:] + [None] * 3
    main(int(a[0] or 1), float(a[1] or 0.0), int(a[2] if a[2] is not None else 1))
