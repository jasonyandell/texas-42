"""42 engine, vectorized over rows (worlds x lines). Tiles are ints 0..27; hands are 28-bit masks.
One hand is fixed: trump, bid, bidder seat. The 'tape' is a (rows, 28) array of u in [0,1)."""
import numpy as np

TILES = [(a, b) for a in range(7) for b in range(a + 1)]      # index -> (hi, lo)
HI = np.array([a for a, b in TILES]); LO = np.array([b for a, b in TILES])
NAME = [f'{a}-{b}' for a, b in TILES]
IDX = {n: i for i, n in enumerate(NAME)}
COUNT = np.where(HI + LO == 5, 5, np.where(HI + LO == 10, 10, 0))
BIT = (1 << np.arange(28)).astype(np.int64)

class Rules:
    """Precomputed tables for a given trump pip (0..6)."""
    def __init__(self, trump):
        self.trump = trump
        self.is_trump = (HI == trump) | (LO == trump)
        # led suit code: 0..6 = pip suit, 7 = trump suit
        self.led_code = np.where(self.is_trump, 7, HI)
        # followers[q] : bool over tiles
        F = np.zeros((8, 28), bool)
        for q in range(7):
            F[q] = ((HI == q) | (LO == q)) & ~self.is_trump
        F[7] = self.is_trump
        self.follow = F
        self.follow_mask = np.array([int((F[q] * BIT).sum()) for q in range(8)], dtype=np.int64)
        # strength[q, tile]
        S = np.zeros((8, 28), np.int32)
        for q in range(8):
            for t in range(28):
                a, b = TILES[t]
                if self.is_trump[t]:
                    S[q, t] = 100 + (7 if a == b else a + b - trump)
                elif q < 7 and F[q, t]:
                    S[q, t] = 1 + (7 if a == b else a + b - q)   # +1 so a legitimately following 0 beats off-suit
                else:
                    S[q, t] = 0
        self.strength = S

def popcount(x):
    x = x.astype(np.int64)
    c = np.zeros_like(x)
    for i in range(28):
        c += (x >> i) & 1
    return c

def kth_set_bit(mask, k):
    """tile index of the k-th (0-based) set bit of each mask. Assumes k < popcount."""
    out = np.full(mask.shape, -1, np.int64); cnt = np.zeros(mask.shape, np.int64)
    for i in range(28):
        has = (mask >> i) & 1
        hit = (has == 1) & (cnt == k) & (out < 0)
        out[hit] = i
        cnt += has
    return out

def first_in_order(mask, order):
    """order: (rows, L) tile ids (-1 = none). Returns first tile in order whose bit is set, else -1."""
    out = np.full(mask.shape, -1, np.int64)
    for j in range(order.shape[1]):
        t = order[:, j]
        ok = (out < 0) & (t >= 0) & (((mask >> np.maximum(t, 0)) & 1) == 1)
        out[ok] = t[ok]
    return out

class Game:
    """Vectorized game state over R rows."""
    def __init__(self, rules, hands, bid, bidder, scores=None, leader=None, table=None, trick=0):
        self.R = hands.shape[0]; self.rules = rules
        self.hands = hands.copy()                      # (R,4) int64 masks
        self.bid = bid; self.bidder = bidder
        self.bid_pts = np.zeros(self.R, np.int64) if scores is None else scores[0].copy()
        self.def_pts = np.zeros(self.R, np.int64) if scores is None else scores[1].copy()
        self.leader = np.full(self.R, bidder, np.int64) if leader is None else np.asarray(leader, np.int64).copy()
        self.table = np.full((self.R, 4), -1, np.int64) if table is None else table.copy()
        self.npl = (self.table >= 0).sum(1)
        self.trick = np.full(self.R, trick, np.int64) if np.isscalar(trick) else trick.copy()
        self.done = np.zeros(self.R, bool)
        self.ply_log = []                              # list of (seat, tile) arrays, for tapes/keys

    def seat_to_play(self):
        return (self.leader + self.npl) % 4

    def legal(self, seat):
        h = self.hands[np.arange(self.R), seat]
        led = self.table[:, 0]
        q = np.where(led >= 0, self.rules.led_code[np.maximum(led, 0)], -1)
        fm = np.where(q >= 0, self.rules.follow_mask[np.maximum(q, 0)], -1)
        f = h & fm
        return np.where(f != 0, f, h)

    def play(self, tile):
        """tile: (R,) tile per row (-1 for rows that are done). Applies the play and resolves the trick if full."""
        r = np.arange(self.R); seat = self.seat_to_play()
        act = (tile >= 0) & ~self.done
        self.hands[r[act], seat[act]] &= ~BIT[tile[act]]
        self.table[r[act], self.npl[act]] = tile[act]
        self.npl[act] += 1
        self.ply_log.append((np.where(act, seat, -1), np.where(act, tile, -1)))
        full = act & (self.npl == 4)
        if full.any():
            tb = self.table[full]; led = tb[:, 0]
            q = self.rules.led_code[led]
            st = self.rules.strength[q[:, None], tb]              # (n,4)
            wpos = st.argmax(1)
            wseat = (self.leader[full] + wpos) % 4
            pts = COUNT[tb].sum(1) + 1
            bidside = (wseat % 2) == (self.bidder % 2)
            self.bid_pts[full] += np.where(bidside, pts, 0)
            self.def_pts[full] += np.where(~bidside, pts, 0)
            self.leader[full] = wseat
            self.table[full] = -1; self.npl[full] = 0; self.trick[full] += 1
            self.done[full] = (self.bid_pts[full] >= self.bid) | (self.def_pts[full] >= 43 - self.bid) | (self.trick[full] >= 7)

    def made(self):
        return (self.bid_pts >= self.bid).astype(np.int64)

def run_out(g, tape, my_seat=None, orders=None, policy=None):
    """Play to the end. Others draw from tape[:, ply]. If my_seat is given, rows play my_seat by 'orders'
    (first legal in order) or by 'policy' (callable(g, legal_mask)->tile). Returns g."""
    ply = 0
    while not g.done.all():
        seat = g.seat_to_play()
        L = g.legal(seat)
        n = popcount(L)
        k = np.minimum((tape[:, ply] * n).astype(np.int64), np.maximum(n - 1, 0))
        t = kth_set_bit(L, k)
        if my_seat is not None:
            mine = (seat == my_seat) & ~g.done
            if mine.any():
                if orders is not None:
                    t_mine = first_in_order(L, orders)
                else:
                    t_mine = policy(g, L)
                t = np.where(mine, t_mine, t)
        t = np.where(g.done, -1, t)
        g.play(t); ply += 1
    return g

# ---------------------------------------------------------------- deals and consistency
def random_deals(rng, n, fixed_seat=None, fixed_hand=None):
    """n deals as (n,4) masks. If fixed_hand (28-bit mask) given, that seat always holds it."""
    out = np.zeros((n, 4), np.int64)
    if fixed_hand is None:
        for i in range(n):
            p = rng.permutation(28)
            for s in range(4): out[i, s] = int(BIT[p[7*s:7*s+7]].sum())
        return out
    rest = np.array([t for t in range(28) if not (fixed_hand >> t) & 1])
    others = [s for s in range(4) if s != fixed_seat]
    for i in range(n):
        p = rng.permutation(rest)
        out[i, fixed_seat] = fixed_hand
        for j, s in enumerate(others): out[i, s] = int(BIT[p[7*j:7*j+7]].sum())
    return out

def consistent_worlds(rng, rules, my_seat, my_hand_now, history, n_want, max_tries=40):
    """Sample full deals consistent with: my current hand, and every play in history being legal for its seat.
    history: list of (seat, tile). Returns (n,4) masks of CURRENT remaining hands and the full original deals."""
    played_by = [[t for s, t in history if s == q] for q in range(4)]
    seen = set(t for _, t in history) | set(t for t in range(28) if (my_hand_now >> t) & 1)
    unseen = np.array([t for t in range(28) if t not in seen])
    need = {s: 7 - len(played_by[s]) for s in range(4) if s != my_seat}
    assert sum(need.values()) == len(unseen)
    acc_now, acc_full = [], []
    for _ in range(max_tries):
        m = max(n_want * 4, 256)
        perms = np.array([rng.permutation(unseen) for _ in range(m)])
        rem = np.zeros((m, 4), np.int64); full = np.zeros((m, 4), np.int64)
        off = 0
        for s in range(4):
            if s == my_seat:
                rem[:, s] = my_hand_now; full[:, s] = my_hand_now | int(BIT[played_by[s]].sum()) if played_by[s] else my_hand_now
                continue
            k = need[s]; sl = perms[:, off:off+k]; off += k
            rem[:, s] = (BIT[sl]).sum(1)
            full[:, s] = rem[:, s] | (int(BIT[played_by[s]].sum()) if played_by[s] else 0)
        # replay legality
        ok = np.ones(m, bool)
        hands = full.copy(); table = []; leader = None
        # we need leader per trick: replay with scoring to know winners
        g = Game(rules, hands, 30, 0)   # bid/bidder irrelevant for legality
        g.leader[:] = history[0][0] if history else 0
        if not history:
            acc_now.append(rem); acc_full.append(full); break
        for s, t in history:
            L = g.legal(np.full(m, s))
            ok &= ((L >> t) & 1) == 1
            g.play(np.full(m, t))
        acc_now.append(rem[ok]); acc_full.append(full[ok])
        if sum(len(a) for a in acc_now) >= n_want: break
    now = np.concatenate(acc_now)[:n_want]; fl = np.concatenate(acc_full)[:n_want]
    return now, fl

def replay(rules, deal_full, bid, bidder, history):
    """Return a 1-row Game at the position after history, from a full deal (1,4)."""
    g = Game(rules, deal_full, bid, bidder)
    g.leader[:] = bidder
    for s, t in history:
        g.play(np.array([t]))
    return g

# ---------------------------------------------------------------- Walt reference: sampled best response to uniform
import itertools
def walt_decide(rng, rules, bid, bidder, my_seat, history, my_hand_now, n_worlds=128):
    """Returns (counts dict tile->worlds where bid made, chosen tile, legal tiles). Our side: defend -> min, bid -> max."""
    rem, full = consistent_worlds(rng, rules, my_seat, my_hand_now, history, n_worlds)
    W = len(rem)
    # position state from replaying history on one deal (public state is the same in all worlds)
    g1 = replay(rules, full[:1], bid, bidder, history)
    my_tiles = [t for t in range(28) if (my_hand_now >> t) & 1]
    L1 = g1.legal(np.array([my_seat]))[0]
    legal = [t for t in my_tiles if (L1 >> t) & 1]
    # lines: orderings of my remaining tiles
    orders = np.array(list(itertools.permutations(my_tiles)), np.int64)   # (P, k)
    P = len(orders)
    tape = rng.random((W, 28))                                           # common random numbers across lines
    # rows = worlds x lines
    hands = np.repeat(rem, P, axis=0); tp = np.repeat(tape, P, axis=0)
    ords = np.tile(orders, (W, 1))
    g = Game(rules, hands, bid, bidder,
             scores=(np.repeat(g1.bid_pts, W*P), np.repeat(g1.def_pts, W*P)),
             leader=np.repeat(g1.leader, W*P), table=np.repeat(g1.table, W*P, axis=0), trick=np.repeat(g1.trick, W*P))
    # record the root tile each line actually plays
    seat = g.seat_to_play(); L = g.legal(seat)
    root = first_in_order(L, ords)
    run_out(g, tp, my_seat=my_seat, orders=ords)
    made = g.made().reshape(W, P); root = root.reshape(W, P)
    bidside = (my_seat % 2) == (bidder % 2)
    counts = {}
    for t in legal:
        sel = (root == t)
        per_world = np.where(sel, made, 9 if not bidside else -9)
        best = per_world.min(1) if not bidside else per_world.max(1)
        counts[t] = int(best.sum())
    if bidside:
        choice = max(legal, key=lambda t: (counts[t], -t))
    else:
        choice = min(legal, key=lambda t: (counts[t], t))
    return counts, choice, legal, W
