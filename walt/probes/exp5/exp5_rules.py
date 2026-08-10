#!/usr/bin/env python3
"""
rules42.py -- declaration-relative Straight 42 rules, generalized from
lambda_probe.py's threes-trump-only implementation (which hardcoded TRUMP=3).

Exploratory probe tier. Stdlib only; exact integers/Fractions; no floats.

Scope of the generalization:
  * pip trump  P0..P6      -- implemented and replay-validated
  * doubles trump          -- implemented (class `Decl(kind='D')`) but NOT
                              exercised: the receipt corpus contains no such
                              hand, so it is reported as UNVALIDATED.
  * no trump                -- implemented (class `Decl(kind='N')`) but NOT
                              exercised for the same reason.
Coverage is reported honestly by `declaration_coverage()`.

The proof that the generalization matches rob is `replay_validate_all()`:
every trick of all 13 receipt hands is re-derived (actor order, follow
legality, trick winner, trick points, cumulative hand points, declaring-side
made/set verdict) from these rules alone.
"""

from __future__ import annotations
import re
from fractions import Fraction

F = Fraction

RECEIPT = "/Users/jason/code/texas-42/rob/receipts/verify_player.txt"

TEAM_OF = (0, 1, 0, 1)          # S0,S2 -> team 0 ; S1,S3 -> team 1


# ----------------------------------------------------------------- tiles

def T(s):
    a, b = s.split("-")
    hi, lo = int(a), int(b)
    assert lo <= hi, s
    return (hi, lo)


def tname(t):
    return f"{t[0]}-{t[1]}"


def has_pip(t, p):
    return t[0] == p or t[1] == p


def is_double(t):
    return t[0] == t[1]


def count_pts(t):
    tot = t[0] + t[1]
    if tot == 5:
        return 5
    if tot == 10:
        return 10
    return 0


ALL_TILES = tuple(sorted(((a, b) for a in range(7) for b in range(a + 1)),
                         reverse=False))
assert len(ALL_TILES) == 28


# ------------------------------------------------------------ declaration

class Decl:
    """A declaration.  kind 'P' with pip p = pip trump; 'D' = doubles trump;
    'N' = no trump (doubles high in their own suit)."""

    __slots__ = ("kind", "pip", "label")

    def __init__(self, kind, pip=None):
        assert kind in ("P", "D", "N")
        if kind == "P":
            assert pip is not None and 0 <= pip <= 6
        self.kind = kind
        self.pip = pip
        self.label = f"P{pip}" if kind == "P" else ("D" if kind == "D" else "N")

    def __repr__(self):
        return f"Decl({self.label})"

    def __eq__(self, o):
        return isinstance(o, Decl) and (self.kind, self.pip) == (o.kind, o.pip)

    def __hash__(self):
        return hash((self.kind, self.pip))

    @property
    def cls(self):
        return {"P": "pip-trump", "D": "doubles-trump", "N": "no-trump"}[self.kind]


TRUMP_SUIT = "TRUMP"            # sentinel suit id for the trump suit


def parse_decl(s):
    m = re.fullmatch(r"P(\d)", s)
    if m:
        return Decl("P", int(m.group(1)))
    if s == "D":
        return Decl("D")
    if s in ("N", "NT"):
        return Decl("N")
    raise ValueError(s)


# ------------------------------------------------------- suits and ranking

def is_trump(t, dc):
    if dc.kind == "P":
        return has_pip(t, dc.pip)
    if dc.kind == "D":
        return is_double(t)
    return False


def led_suit(t, dc):
    """Suit established by leading tile t."""
    if is_trump(t, dc):
        return TRUMP_SUIT
    return t[0]                 # higher pip names the suit


def in_suit(t, s, dc):
    """Membership under absorption: a trump tile belongs to the trump suit only."""
    if s == TRUMP_SUIT:
        return is_trump(t, dc)
    if is_trump(t, dc):
        return False
    return has_pip(t, s)


def rank_key(t, s, dc):
    """Order within suit s: the suit's double is highest, then by the off-pip."""
    assert in_suit(t, s, dc), (t, s, dc)
    if s == TRUMP_SUIT:
        if dc.kind == "P":
            if is_double(t):
                return 100
            return t[1] if t[0] == dc.pip else t[0]
        # doubles trump: rank doubles by their pip
        assert dc.kind == "D" and is_double(t)
        return t[0]
    if is_double(t):
        return 100
    return t[1] if t[0] == s else t[0]


def trick_winner(plays, dc):
    """plays: [(seat, tile)] in play order.  Returns the winning seat."""
    led = led_suit(plays[0][1], dc)
    trumps = [(rank_key(t, TRUMP_SUIT, dc), s) for s, t in plays
              if in_suit(t, TRUMP_SUIT, dc)]
    if trumps and led != TRUMP_SUIT:
        return max(trumps)[1]
    if led == TRUMP_SUIT:
        return max(trumps)[1]
    ins = [(rank_key(t, led, dc), s) for s, t in plays if in_suit(t, led, dc)]
    return max(ins)[1]


def trick_points(plays):
    return 1 + sum(count_pts(t) for _, t in plays)


def legal(hand, led, dc):
    """led = suit id or None (on lead).  Must follow suit if able."""
    if led is None:
        return list(hand)
    follows = [t for t in hand if in_suit(t, led, dc)]
    return follows if follows else list(hand)


# ---------------------------------------------------------- receipt parsing

HAND_RE = re.compile(
    r"hand (\d+): shaker S(\d), bidder S(\d), bid (\S+), declaration (\S+)")
TRICK_RE = re.compile(
    r"\s+trick (\d+): (\S+) (\S+) (\S+) (\S+) -> (S\d) \+(\d+)")
RESULT_RE = re.compile(
    r"\s+result: declaring (T\d) (\d+) points -> (made|set)")


class Hand:
    __slots__ = ("hid", "shaker", "bidder", "decl", "tricks", "decl_team",
                 "decl_points", "verdict")

    def __init__(self, hid, shaker, bidder, decl):
        self.hid = hid
        self.shaker = shaker
        self.bidder = bidder
        self.decl = decl
        self.tricks = []        # [(plays, winner_seat, points)]

    def __repr__(self):
        return f"Hand({self.hid}, {self.decl.label})"


def parse_receipt(path=RECEIPT):
    hands = []
    cur = None
    with open(path) as fh:
        for line in fh:
            m = HAND_RE.match(line)
            if m:
                cur = Hand(int(m.group(1)), int(m.group(2)), int(m.group(3)),
                           parse_decl(m.group(5)))
                hands.append(cur)
                continue
            m = TRICK_RE.match(line)
            if m and cur is not None:
                plays = []
                for tok in m.group(2, 3, 4, 5):
                    seat, tile = tok.split(":")
                    plays.append((int(seat[1]), T(tile)))
                cur.tricks.append((plays, int(m.group(6)[1]), int(m.group(7))))
                continue
            m = RESULT_RE.match(line)
            if m and cur is not None:
                cur.decl_team = int(m.group(1)[1])
                cur.decl_points = int(m.group(2))
                cur.verdict = m.group(3)
    assert len(hands) == 13, len(hands)
    for h in hands:
        assert len(h.tricks) == 7, (h.hid, len(h.tricks))
    return hands


# --------------------------------------------------------- replay validation

def initial_hands(h):
    hands = {s: set() for s in range(4)}
    for plays, _, _ in h.tricks:
        for s, t in plays:
            hands[s].add(t)
    return hands


def replay_validate(h, verbose=False):
    """Full re-derivation of one receipt hand. Raises AssertionError on any
    mismatch.  Returns a dict of the derived facts."""
    dc = h.decl
    hands = initial_hands(h)
    assert all(len(v) == 7 for v in hands.values()), \
        (h.hid, {s: len(v) for s, v in hands.items()})
    seen = set()
    for v in hands.values():
        assert not (seen & v)
        seen |= v
    assert len(seen) == 28

    leader = h.tricks[0][0][0][0]
    assert leader == h.bidder, (h.hid, leader, h.bidder)
    pts = {0: 0, 1: 0}
    tricks_won = {0: 0, 1: 0}
    for tno, (plays, rec_w, rec_p) in enumerate(h.tricks, 1):
        assert plays[0][0] == leader, (h.hid, tno, plays[0][0], leader)
        order = [(leader + k) % 4 for k in range(4)]
        assert [s for s, _ in plays] == order, (h.hid, tno)
        led = led_suit(plays[0][1], dc)
        for s, t in plays:
            assert t in hands[s], (h.hid, tno, s, t)
            if s != leader:
                assert t in legal(hands[s], led, dc), (h.hid, tno, s, t, led)
            hands[s].discard(t)
        w = trick_winner(plays, dc)
        p = trick_points(plays)
        assert w == rec_w, (h.hid, tno, w, rec_w)
        assert p == rec_p, (h.hid, tno, p, rec_p)
        pts[TEAM_OF[w]] += p
        tricks_won[TEAM_OF[w]] += 1
        leader = w
    assert all(len(v) == 0 for v in hands.values())
    assert pts[0] + pts[1] == 42, (h.hid, pts)
    assert pts[h.decl_team] == h.decl_points, (h.hid, pts, h.decl_points)
    made = h.decl_points >= 30
    assert (h.verdict == "made") == made, (h.hid, h.verdict, h.decl_points)
    if verbose:
        print(f"  hand {h.hid:2d} ({dc.label}): 7 tricks re-derived; "
              f"points T0 {pts[0]} / T1 {pts[1]}; declaring T{h.decl_team} "
              f"{h.decl_points} -> {h.verdict}  OK")
    return {"points": pts, "tricks": tricks_won}


def replay_validate_all(hands=None, verbose=False):
    hands = hands if hands is not None else parse_receipt()
    for h in hands:
        replay_validate(h, verbose=verbose)
    return hands


def declaration_coverage(hands):
    cov = {}
    for h in hands:
        cov.setdefault(h.decl.cls, []).append(h.hid)
    return cov


# ------------------------------------------------- kernel / fiber machinery

def state_before_trick(h, tno):
    """Hands and leader immediately before trick `tno` (1-based)."""
    hands = initial_hands(h)
    leader = h.tricks[0][0][0][0]
    for plays, w, _ in h.tricks[: tno - 1]:
        for s, t in plays:
            hands[s].discard(t)
        leader = w
    return hands, leader


def voids_before_trick(h, tno):
    """{(seat, suit)} observable from tricks 1..tno-1: a seat that failed to
    follow the led suit holds no tile of that suit (absorption respected)."""
    dc = h.decl
    out = set()
    for plays, _, _ in h.tricks[: tno - 1]:
        led = led_suit(plays[0][1], dc)
        ldr = plays[0][0]
        for s, t in plays:
            if s != ldr and not in_suit(t, led, dc):
                out.add((s, led))
    return out


def fiber(unseen, sizes, voids, dc):
    """All assignments of `unseen` to the seats of `sizes` (dict seat->count)
    consistent with the observed voids.  Returns a list of dicts seat->frozenset."""
    seats = sorted(sizes)
    unseen = sorted(unseen)
    assert sum(sizes.values()) == len(unseen)

    banned = {s: {suit for (ss, suit) in voids if ss == s} for s in seats}

    def ok(seat, tile):
        for suit in banned[seat]:
            if in_suit(tile, suit, dc):
                return False
        return True

    out = []

    def rec(i, remaining, acc):
        if i == len(seats):
            assert not remaining
            out.append(dict(acc))
            return
        s = seats[i]
        k = sizes[s]
        cand = [t for t in remaining if ok(s, t)]
        from itertools import combinations
        for combo in combinations(cand, k):
            cs = set(combo)
            acc[s] = frozenset(cs)
            rec(i + 1, [t for t in remaining if t not in cs], acc)
            del acc[s]

    rec(0, unseen, {})
    return out


if __name__ == "__main__":
    hs = replay_validate_all(verbose=True)
    print("\nall 13 receipt hands replay-validated under the generalized "
          "declaration-relative rules")
    cov = declaration_coverage(hs)
    print("declaration coverage in the receipt corpus:")
    for cls in ("pip-trump", "doubles-trump", "no-trump"):
        ids = cov.get(cls, [])
        print(f"  {cls:15s}: {len(ids)} hands {ids if ids else '(ABSENT)'}")
    from collections import Counter
    print("  by declaration label:",
          dict(Counter(h.decl.label for h in hs)))
