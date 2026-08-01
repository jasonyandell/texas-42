---
number: 9
slug: constellation-suffix-factorization
conversation: https://chatgpt.com/c/6a6da164-3b7c-83e8-9dcb-fb6477903bd3
submitted-at: 2026-08-01T07:33:58.039Z
harvested-at: 2026-08-01T08:18:20.928Z
extraction: copy-button
caveats: none
---
# MACHINE-CHECKABLE ARTIFACTS

**FINAL ANSWER: TRUE (C1 proved)**

**SECONDARY: FALSE.** Equivalent suffixes need not have the same realizable predecessor-constellation set. An explicit zeroes-trump/doubles-trump witness is given below.

The proof uses exactly the attached Straight 42 declaration, follow, comparison, scoring, and physical-transition rules.  The mathematical foundation supplies the exact legal-set formulation, unique-winner theorem, finite graded game structure, and backward-induction framework used here.  The supplied verifier’s 737,100-case unique-winner receipt is independently recomputed by the new program rather than imported. 

Observed execution of the program below:

* SHA-256: `3a15b15692c72584f74242d1158b7de0a157cf9eb5e08b9f09c3b15f3dfc34de`
* Elapsed time here: 26.47 seconds
* Peak resident memory here: approximately 135 MB
* Exit status: 0

It printed:

* `PASS unique-winner cases=737100`
* `PASS k1-census positions=2211300 classes=19329 distinct_outcomes=14`
* `PASS k2-value-constancy positions=20001 total_classes=10496 multi_groups=9495 cross_declaration_groups=9495 largest_group=4 solved_members=19000`
* `PASS legality-transports pairs=5000 paired_nodes=255420`
* `PASS resolution-transports pairs=5000 completed_trick_edges=93258`
* `PASS hereditariness pairs=5000 successor_restrictions=93258`
* `PASS backward-counterexample ... fixed_partial_maps=4 full_embeddings=0 legal_embeddings=0`
* `PASS all-checks`

## Proof

1. **Normalized seats and strengthened value.**
   **[USES: suffix-position and constellation definitions in §2 of the task; partnership definition in §1; exact finite minimax recursion in the mathematical foundation.]**

   Normalize every suffix position so that its current leader has seat offset (0). Thus offsets (0,2) form one partnership and offsets (1,3) the other.

   For (\eta\in{0,1}), define (V_\eta(X)) to be the exact minimax margin of the partnership whose offsets have parity (\eta), with that partnership maximizing and the other minimizing. The conjectured value is

   [
   v(X)=V_0(X).
   ]

   This strengthening is useful because after a trick the new leader may belong to either partnership. It prevents an implicit sign error when the suffix is re-based around that new leader.

2. **Lift of a boundary isomorphism to open-trick states.**
   **[USES: constellation-equivalence definition; hold and clockwise actor definitions; task obligation 1.]**

   Let

   [
   \phi:\operatorname{Con}(X)\longrightarrow\operatorname{Con}(X')
   ]

   be a constellation isomorphism at a trick boundary.

   During an open trick, do **not** immediately discard the already played trick tiles from the relational arena. Instead retain them as a transient ordered prefix

   [
   C=((s_0,d_0),\ldots,(s_{j-1},d_{j-1})),\qquad 0\le j\le3.
   ]

   Its corresponding prefix is

   [
   \phi(C)=((s_0,\phi(d_0)),\ldots,(s_{j-1},\phi(d_{j-1}))).
   ]

   The remaining hand of seat offset (s) is determined by:

   * the original `hold` labels;
   * the tiles already played by that seat in the mapped history.

   Since (\phi) preserves `hold`, it maps every actor’s current shrinking hand bijectively to the corresponding shrinking hand. The two paired states also have the same current leader offset and prefix length, so they have the same acting seat.

   Thus the boundary constellation plus the transient trick-prefix marking contains everything needed at a mid-trick node. No new static relation is required.

3. **Obligation 1 — Legality transports.**
   **[USES: Step 2; leader freedom; exact follow-if-possible rule; definition of `FOLLOW(d,e)`.]**

   At a lead node, every tile in the leader’s remaining hand is legal. Step 2 gives a bijection between those hands, so leader legality transports.

   At a follower node, let (e=d_0) be the trick’s lead tile and let (R_s) be the acting follower’s current remaining hand. Its follower subset is

   [
   M_s(e)={d\in R_s:\operatorname{FOLLOW}(d,e)=1}.
   ]

   The exact legal set is

   [
   A_s=
   \begin{cases}
   M_s(e),&M_s(e)\ne\varnothing,\
   R_s,&M_s(e)=\varnothing.
   \end{cases}
   ]

   Because (\phi) maps (R_s) bijectively and preserves `FOLLOW`,

   [
   \phi(M_s(e))=M'_s(\phi(e)).
   ]

   In particular, nonemptiness is preserved. Therefore

   [
   \phi(A_s)=A'_s
   ]

   at every follower decision, including after earlier players have reduced their hands. This proves exact legal-action transport at all mid-trick depths.

4. **Obligation 2 — Resolution transports.**
   **[USES: `ORD` definition; unique-winner theorem; preservation of (c); Step 3.]**

   For a completed trick with lead (g), `ORD(d,e,g)` is exactly the comparison of the two contextual trick keys

   [
   \tau_\delta(d,\ell_\delta(g))
   \quad\text{and}\quad
   \tau_\delta(e,\ell_\delta(g)).
   ]

   The winner is the unique maximal played tile under this comparison. Numeric ranks are not needed: the complete strict/equal comparison table is sufficient.

   Since (\phi) preserves all values of `ORD`, it maps the unique maximum to the unique maximum. The winner therefore has the same seat offset in both tricks.

   The award is

   [
   1+\sum_{d\text{ played}}c(d),
   ]

   and (\phi) preserves every (c(d)), so the award is identical. The winner’s partnership parity is also identical because the winner’s seat offset is identical.

5. **Re-basing `hold` after resolution.**
   **[USES: Step 4; definition of `hold` relative to the current leader.]**

   Suppose the winner has old-leader offset (w). A surviving tile with old hold offset (h) has successor hold offset

   [
   h_{\mathrm{new}}=h-w\pmod4.
   ]

   Since (\phi) preserves the old value (h), and Step 4 shows that both tricks have the same winner offset (w),

   [
   \operatorname{hold}_{\mathrm{new}}(\phi(d))
   ===========================================

   \operatorname{hold}_{\mathrm{new}}(d).
   ]

   Thus the same restricted tile bijection preserves the re-based holder labels.

6. **Obligation 3 — Hereditariness.**
   **[USES: static declaration mechanics; Steps 4–5; task obligation 3.]**

   Let (T) be the four completed-trick tiles and (R=L\setminus T) the survivor universe.

   The successor’s count decoration is plainly (c|_R).

   For (d,e\in R),

   [
   \operatorname{FOLLOW}_{\mathrm{succ}}(d,e)
   ==========================================

   # F_\delta(d,\ell_\delta(e))

   \operatorname{FOLLOW}_{\mathrm{pred}}(d,e).
   ]

   The declaration, called set, effective suits, led-context function, and ranks do not change when other tiles die.

   Likewise, for (d,e,g\in R),

   [
   \operatorname{ORD}_{\mathrm{succ}}(d,e,g)
   =========================================

   \operatorname{ORD}_{\mathrm{pred}}(d,e,g).
   ]

   Only contexts led by future surviving tiles can be queried in a future trick. No relation involving a removed tile is required after the current trick closes.

   Consequently, except for the explicitly handled re-basing of `hold`, the successor constellation is literally the induced restriction

   [
   \operatorname{Con}(X)|_R.
   ]

   By Step 5,

   [
   \phi|_R:\operatorname{Con}(Y)\cong\operatorname{Con}(Y')
   ]

   for the two successor suffixes.

7. **Base case (k=1).**
   **[USES: Steps 3–4; one tile per hand.]**

   Each player has one tile. The leader has one legal action, and every follower also has one legal action whether that tile follows or is a legal slough. Hence the entire trick is forced.

   `ORD` determines the same winner and (c) determines the same award. Therefore

   [
   V_\eta(X)=V_\eta(X')
   ]

   for both (\eta=0,1).

8. **Induction and strong bisimulation.**
   **[USES: Steps 2–7; minimax definition.]**

   Assume equality of (V_\eta) for every equivalent suffix at depth (k-1) and both values of (\eta).

   At depth (k), Steps 2–4 give an isomorphism of the complete four-ply first-trick decision trees:

   * corresponding actors have the same partnership parity;
   * legal actions correspond bijectively;
   * completed branches have the same winner offset (w);
   * completed branches have the same award (g).

   For reference parity (\eta), the immediate signed reward is

   [
   \begin{cases}
   +g,&w\equiv\eta\pmod2,\
   -g,&w\not\equiv\eta\pmod2.
   \end{cases}
   ]

   After re-basing around winner offset (w), the old reference partnership has successor-relative parity

   [
   \eta'=\eta-w\pmod2.
   ]

   Step 6 gives equivalent successor suffixes, so the induction hypothesis gives equal continuation values (V_{\eta'}). Every corresponding completed first-trick branch therefore has equal total value.

   Applying the same maximizing or minimizing operator at each corresponding actor node preserves equality back to the roots. Hence

   [
   V_\eta(X)=V_\eta(X')
   ]

   for both parities.

   The construction is stronger than mere equality of root values: it is a strong bisimulation of the action-labeled, actor-labeled, reward-labeled finite game trees.

9. **Conclusion for C1.**
   **[USES: Step 8 with (\eta=0).]**

   Taking the current leader’s partnership as the reference partnership,

   [
   v(X)=V_0(X)=V_0(X')=v(X').
   ]

   Therefore:

   [
   \boxed{X\sim X'\Longrightarrow v(X)=v(X').}
   ]

10. **Canonicalization used by the verifier.**
    **[USES: constellation-equivalence definition.]**

    For a finite suffix, any admissible node relabeling must preserve both `hold` and (c). Therefore every admissible relabeling is a product of permutations within the equal-((\operatorname{hold},c)) blocks.

    The verifier:

    1. orders those blocks canonically;
    2. enumerates every permutation within each block;
    3. serializes the resulting `hold`, (c), `FOLLOW`, and `ORD` tables injectively;
    4. takes the minimum byte string.

    Two structures have the same minimum exactly when an admissible relabeling makes all four constellation components equal. At (k=2), each hand has two tiles, so there are at most (2^4=16) candidate block permutations.

11. **Secondary backward statement — precise formulation.**
    **[USES: suffix and constellation definitions; full 28-tile universe.]**

    Let (\operatorname{Pred}(X)) be the set of pooled constellation classes ([\operatorname{Con}(Y)]) such that:

    * (Y) has one additional tile in every hand;
    * the four additional tiles are distinct and disjoint from (L_X);
    * playing exactly those four tiles as one legal trick from (Y) leaves the hands of (X);
    * that trick’s winner becomes the leader of (X);
    * (Y) uses the same declaration as (X).

    The proposed backward commutation is

    [
    X\sim X'\Longrightarrow
    \operatorname{Pred}(X)=\operatorname{Pred}(X').
    ]

    This statement is false.

12. **Equivalent current suffixes for the backward counterexample.**
    **[USES: explicit `FOLLOW`/`ORD` verification in the program.]**

    Let (X) use **zeroes trump**, and let (X') use **doubles trump**. Each hand contains one tile:

    | Hold offset | (X), zeroes trump | (X'), doubles trump |
    | ----------: | ----------------: | ------------------: |
    |           0 |           (0!:!0) |             (2!:!2) |
    |           1 |           (1!:!1) |             (1!:!0) |
    |           2 |           (1!:!0) |             (0!:!0) |
    |           3 |           (2!:!0) |             (1!:!1) |

    Define (\phi) rowwise by this table. Every tile has count label zero. The program verifies all

    [
    4^2=16
    ]

    `FOLLOW` entries and all

    [
    4^3=64
    ]

    `ORD` entries, as well as `hold` and (c). Thus

    [
    X\sim X'.
    ]

13. **A predecessor class belonging to (\operatorname{Pred}(X)).**
    **[USES: zeroes-trump legality and resolution rules.]**

    Let the previous leader be current seat (1). Relative to that previous leader, take the predecessor hands:

    | Previous hold | Survivor | Newly resurrected tile |
    | ------------: | -------: | ---------------------: |
    |             0 |  (1!:!1) |                (2!:!1) |
    |             1 |  (1!:!0) |                (2!:!2) |
    |             2 |  (2!:!0) |                (3!:!1) |
    |             3 |  (0!:!0) |                (3!:!0) |

    Play the resurrected tiles in order:

    [
    2!:!1,\quad2!:!2,\quad3!:!1,\quad3!:!0.
    ]

    The lead is natural twos. Tile (2!:!2) follows. The next two hands have no natural-two follower because their zero-bearing tiles have been called into trump. Their plays are legal sloughs. Tile (3!:!0) is trump and wins.

    Its holder is current seat (0), so after removing the four resurrected tiles and re-basing around that winner, the remaining suffix is exactly (X). If this predecessor is (Y), then

    [
    [\operatorname{Con}(Y)]\in\operatorname{Pred}(X).
    ]

14. **Exhaustive exclusion from (\operatorname{Pred}(X')).**
    **[USES: Step 13’s explicit predecessor constellation; finite 28-tile universe; exact verifier search.]**

    Any doubles-trump predecessor of (X') has two tiles in each predecessor-relative hold:

    * one fixed survivor belonging to (X');
    * one tile chosen from the 24-tile complement.

    There are four possible previous leaders. For each previous leader, an isomorphism from (\operatorname{Con}(Y)) must choose which of the two nodes in each of (Y)’s four hold cells maps to the fixed survivor. Thus all possible restrictions are covered by

    [
    4\cdot2^4=64
    ]

    partial mappings.

    The program checks each partial mapping against exact preservation of (c), `FOLLOW`, and every fully instantiated `ORD` tuple. Exactly four partial mappings survive.

    For each survivor, it exhaustively backtracks over all distinct images in the 24-tile complement, checking every newly instantiated relation after every assignment. It finds:

    [
    \text{full constellation embeddings}=0.
    ]

    Therefore no doubles-trump predecessor containing the fixed suffix (X') has constellation equivalent to (Y), even before first-trick legality is imposed. In particular,

    [
    [\operatorname{Con}(Y)]\notin\operatorname{Pred}(X').
    ]

    Hence

    [
    \boxed{\operatorname{Pred}(X)\ne\operatorname{Pred}(X').}
    ]

    The failure is exactly realizability-shaped: the forward continuation structure factors through the constellation, but the ability to embed that finite structure backward into the unused portion of the physical 28-tile declaration layer does not.

[Download the self-contained verifier](sandbox:/mnt/data/verify_c1.py)

```python
#!/usr/bin/env python3
"""C1 / backward-step verifier. Python 3 stdlib only; deterministic; no I/O."""
from collections import defaultdict
from functools import lru_cache
from itertools import combinations, permutations, product
from random import Random
import sys

P = tuple(range(7)); DT = 7; NT = 8; DECLS = tuple(range(9)); CALLED = 7
D = tuple((h, l) for h in P for l in range(h + 1))
ID = {d: i for i, d in enumerate(D)}
CC = {0: 0, 5: 1, 10: 2}


def did(a, b): return ID[(max(a, b), min(a, b))]
def name(d): return f"{D[d][0]}:{D[d][1]}"
def contains(d, p): return p in D[d]
def double(d): return D[d][0] == D[d][1]

def count(d):
    if D[d] in ((5, 5), (6, 4)): return 10
    if D[d] in ((5, 0), (4, 1), (3, 2)): return 5
    return 0
def called(d, dec):
    return contains(d, dec) if dec < 7 else double(d) if dec == DT else False
def powered(d, dec): return dec != NT and called(d, dec)
def led(d, dec): return CALLED if called(d, dec) else D[d][0]

def follows_q(d, q, dec):
    if called(d, dec): return q == CALLED
    return q in D[d]
def rank(d, dec):
    h, l = D[d]
    if dec == DT and h == l: return h
    if h == l: return 14
    return h + l
def tkey(d, q, dec):
    if powered(d, dec): return (2, rank(d, dec))
    if follows_q(d, q, dec): return (1, rank(d, dec))
    return (0, 0)
def cmp(a, b): return (a > b) - (a < b) + 1

# FOLLOW[dec][d][g] = F_dec(d, ell_dec(g)); ORD[dec][g][d][e] is <,=,>.
FOLLOW = {}; ORD = {}; KEYS = {}
for dec in DECLS:
    kbg = []
    obg = []
    for g in range(28):
        ks = tuple(tkey(d, led(g, dec), dec) for d in range(28))
        kbg.append(ks)
        obg.append(tuple(tuple(cmp(ks[a], ks[b]) for b in range(28)) for a in range(28)))
    KEYS[dec] = tuple(kbg)
    ORD[dec] = tuple(obg)
    FOLLOW[dec] = tuple(tuple(int(follows_q(d, led(g, dec), dec)) for g in range(28)) for d in range(28))


def norm(hands): return tuple(tuple(sorted(h)) for h in hands)
def legal(hand, trick, dec):
    if not trick: return hand
    fs = tuple(d for d in hand if FOLLOW[dec][d][trick[0][1]])
    return fs or hand
def resolve(trick, dec):
    ks = KEYS[dec][trick[0][1]]
    vals = tuple(ks[d] for _, d in trick)
    assert vals.count(max(vals)) == 1
    i = vals.index(max(vals))
    return trick[i][0], 1 + sum(count(d) for _, d in trick)


def hmap(hands, leader):
    out = {}
    for s, hand in enumerate(hands):
        for d in hand:
            assert d not in out
            out[d] = (s - leader) % 4
    return out


def relcode(dec, order, labels):
    out = bytearray(labels)
    f = FOLLOW[dec]
    for a in order:
        for g in order: out.append(f[a][g])
    o = ORD[dec]
    for g in order:
        for a in order:
            for b in order: out.append(o[g][a][b])
    return bytes(out)


def canon(dec, hands, leader=0):
    hm = hmap(hands, leader)
    opts = []; labels = []
    for h in range(4):
        for c in range(3):
            xs = tuple(sorted(d for d in hm if hm[d] == h and CC[count(d)] == c))
            if not xs: continue
            labels.extend((h, c) * len(xs))
            opts.append((xs,) if len(xs) == 1 else tuple(permutations(xs)))
    best = order_best = None
    for blocks in product(*opts):
        order = tuple(d for block in blocks for d in block)
        code = relcode(dec, order, bytes(labels))
        if best is None or code < best: best, order_best = code, order
    return (b"", ()) if best is None else (best, order_best)


def map_ok(da, ha, la, db, hb, lb, phi):
    ma, mb = hmap(ha, la), hmap(hb, lb)
    if set(phi) != set(ma) or set(phi.values()) != set(mb): return False
    for d in ma:
        e = phi[d]
        if ma[d] != mb[e] or count(d) != count(e): return False
    for a in ma:
        for g in ma:
            if FOLLOW[da][a][g] != FOLLOW[db][phi[a]][phi[g]]: return False
    for g in ma:
        for a in ma:
            for b in ma:
                if ORD[da][g][a][b] != ORD[db][phi[g]][phi[a]][phi[b]]: return False
    return True


def solve(dec, hands):
    hands = norm(hands)
    @lru_cache(None)
    def rec(hs, leader, trick):
        if all(not h for h in hs):
            assert not trick
            return 0
        actor = (leader + len(trick)) % 4
        vals = []
        for d in legal(hs[actor], trick, dec):
            nh = [list(h) for h in hs]; nh[actor].remove(d)
            nt = trick + ((actor, d),); nl = leader; reward = 0
            if len(nt) == 4:
                w, award = resolve(nt, dec)
                reward = award if w % 2 == 0 else -award
                nl, nt = w, ()
            vals.append(reward + rec(norm(nh), nl, nt))
        return max(vals) if actor % 2 == 0 else min(vals)
    return rec(hands, 0, ())


def check_unique():
    n = 0; universe = set(range(28))
    for dec in DECLS:
        for lead in range(28):
            ks = KEYS[dec][lead]
            for rest in combinations(sorted(universe - {lead}), 3):
                vals = tuple(ks[d] for d in (lead,) + rest)
                assert vals.count(max(vals)) == 1
                n += 1
    assert n == 737_100
    print(f"PASS unique-winner cases={n}")


def sig1(dec, order):
    cp = 0
    for d in order: cp = 3 * cp + CC[count(d)]
    fp = 0
    for a in order:
        for g in order: fp = (fp << 1) | FOLLOW[dec][a][g]
    op = 0
    for g in order:
        for a in order:
            for b in order: op = 3 * op + ORD[dec][g][a][b]
    return cp, fp, op
def out1(dec, order):
    ks = KEYS[dec][order[0]]
    vals = tuple(ks[d] for d in order)
    return vals.index(max(vals)) % 2, 1 + sum(count(d) for d in order)


def check_k1():
    classes = {}; outcomes = set(); n = 0
    for live in combinations(range(28), 4):
        live = set(live)
        for leadtile in live:
            r = live - {leadtile}
            for partner in r:
                opp = sorted(r - {partner})
                order = (leadtile, opp[0], partner, opp[1])
                for dec in DECLS:
                    s, o = sig1(dec, order), out1(dec, order)
                    if s in classes and classes[s] != o:
                        raise AssertionError(f"k1 class divergence {dec} {tuple(map(name, order))}")
                    classes[s] = o; outcomes.add(o); n += 1
    assert n == 2_211_300
    print(f"PASS k1-census positions={n} classes={len(classes)} distinct_outcomes={len(outcomes)}")


SWAP_P = (0, 1, 3, 2, 4, 5, 6)
SWAP_D = tuple(ID[tuple(sorted((SWAP_P[h], SWAP_P[l]), reverse=True))] for h, l in D)
def rand_hands(rng):
    xs = rng.sample(range(28), 8); rng.shuffle(xs)
    return norm(xs[2*s:2*s+2] for s in range(4))
def swap_hands(hs): return norm((SWAP_D[d] for d in h) for h in hs)

def family_k2():
    rng = Random(0x42C1B15); fam = []; seen = set()
    while len(seen) < 9500:
        h = rand_hands(rng)
        if h in seen: continue
        seen.add(h); fam += [(2, h), (3, swap_hands(h))]
    for dec in (0, 1, 4, 5, 6, DT, NT):
        local = set()
        while len(local) < 143:
            h = rand_hands(rng)
            if h in local: continue
            local.add(h); fam.append((dec, h))
    assert len(fam) == 20_001
    return fam


def check_k2():
    groups = defaultdict(list); dc = defaultdict(int)
    for dec, hs in family_k2():
        key, order = canon(dec, hs)
        groups[key].append((dec, hs, order)); dc[dec] += 1
    assert set(dc) == set(DECLS)
    multi = largest = solved = cross = 0
    for ms in groups.values():
        largest = max(largest, len(ms))
        if len(ms) < 2: continue
        multi += 1; solved += len(ms)
        cross += len({m[0] for m in ms}) >= 2
        vals = [solve(dec, hs) for dec, hs, _ in ms]
        if len(set(vals)) != 1: raise AssertionError(f"k2 divergence {vals}")
    assert multi >= 5000
    counts = tuple(dc[d] for d in DECLS)
    print("PASS k2-value-constancy "
          f"positions=20001 total_classes={len(groups)} multi_groups={multi} "
          f"cross_declaration_groups={cross} largest_group={largest} "
          f"solved_members={solved} declaration_counts={counts}")
    return groups


def bisim_pair(A, B):
    da, ha, oa = A; db, hb, ob = B; phi = dict(zip(oa, ob))
    assert map_ok(da, ha, 0, db, hb, 0, phi)
    seen = set(); stats = [0, 0]
    def rec(h1, h2, leader, t1, t2):
        state = (h1, h2, leader, t1, t2)
        if state in seen: return
        seen.add(state); stats[0] += 1
        if all(not h for h in h1):
            assert all(not h for h in h2) and not t1 and not t2
            return
        actor = (leader + len(t1)) % 4
        assert actor == (leader + len(t2)) % 4
        l1, l2 = legal(h1[actor], t1, da), legal(h2[actor], t2, db)
        assert tuple(sorted(phi[d] for d in l1)) == tuple(sorted(l2))
        for d1 in l1:
            d2 = phi[d1]
            n1 = [list(h) for h in h1]; n2 = [list(h) for h in h2]
            n1[actor].remove(d1); n2[actor].remove(d2)
            n1, n2 = norm(n1), norm(n2)
            u1, u2, nl = t1 + ((actor, d1),), t2 + ((actor, d2),), leader
            if len(u1) == 4:
                w1, a1 = resolve(u1, da); w2, a2 = resolve(u2, db)
                assert (w1, a1) == (w2, a2); stats[1] += 1; nl = w1; u1 = u2 = ()
                live = {d for h in n1 for d in h}
                rphi = {d: phi[d] for d in live}
                assert map_ok(da, n1, nl, db, n2, nl, rphi)
            rec(n1, n2, nl, u1, u2)
    rec(ha, hb, 0, (), ())
    return stats


def check_bisim(groups):
    pairs = []
    for ms in groups.values():
        for i in range(len(ms)):
            for j in range(i + 1, len(ms)):
                if ms[i][0] != ms[j][0]: pairs.append((ms[i], ms[j])); break
            if len(pairs) >= 5000: break
        if len(pairs) >= 5000: break
    assert len(pairs) >= 5000
    nodes = tricks = 0
    for A, B in pairs[:5000]:
        n, t = bisim_pair(A, B); nodes += n; tricks += t
    print(f"PASS legality-transports pairs=5000 paired_nodes={nodes}")
    print(f"PASS resolution-transports pairs=5000 completed_trick_edges={tricks}")
    print(f"PASS hereditariness pairs=5000 successor_restrictions={tricks}")


# Backward counterexample helpers.
def first_trick(dec, current, added, old_leader):
    if len(set(current + added)) != 8: return None
    hs = [list(h) for h in norm((current[s], added[s]) for s in range(4))]
    trick = ()
    for i in range(4):
        actor = (old_leader + i) % 4; d = added[actor]
        if d not in legal(tuple(hs[actor]), trick, dec): return None
        hs[actor].remove(d); trick += ((actor, d),)
    w, _ = resolve(trick, dec)
    return w, norm(hs), trick


def partial_ok(dt, dc, mp):
    xs = tuple(mp)
    if len(set(mp.values())) != len(mp): return False
    if any(count(x) != count(mp[x]) for x in xs): return False
    for a in xs:
        for g in xs:
            if FOLLOW[dt][a][g] != FOLLOW[dc][mp[a]][mp[g]]: return False
    for g in xs:
        for a in xs:
            for b in xs:
                if ORD[dt][g][a][b] != ORD[dc][mp[g]][mp[a]][mp[b]]: return False
    return True


def backward_stats(dt, target_hands, target_leader, dc, current):
    hm = hmap(target_hands, target_leader); by = {h: [] for h in range(4)}
    for d in sorted(hm): by[hm[d]].append(d)
    assert all(len(v) == 2 for v in by.values())
    complement = tuple(d for d in range(28) if d not in set(current))
    fixed = full = legal_full = 0
    for leader in range(4):
        survivors = {h: current[(leader + h) % 4] for h in range(4)}
        for bits in product((0, 1), repeat=4):
            mp = {}; unknown = []; ok = True
            for h, bit in enumerate(bits):
                x, u = by[h][bit], by[h][1-bit]; y = survivors[h]
                if count(x) != count(y): ok = False; break
                mp[x] = y; unknown.append((h, u))
            if not ok or not partial_ok(dt, dc, mp): continue
            fixed += 1; cands = {}
            for _, u in unknown:
                ys = []
                for y in complement:
                    if y in mp.values(): continue
                    trial = dict(mp); trial[u] = y
                    if partial_ok(dt, dc, trial): ys.append(y)
                cands[u] = tuple(ys)
            unknown.sort(key=lambda hu: len(cands[hu[1]]))
            def bt(i, cur, used):
                nonlocal full, legal_full
                if i == 4:
                    full += 1; added = [None] * 4
                    for h, u in unknown: added[(leader + h) % 4] = cur[u]
                    z = first_trick(dc, current, tuple(added), leader)
                    legal_full += z is not None and z[0] == 0
                    return
                _, u = unknown[i]
                for y in cands[u]:
                    if y in used: continue
                    cur[u] = y
                    if partial_ok(dt, dc, cur): bt(i + 1, cur, used | {y})
                    del cur[u]
            bt(0, dict(mp), set(mp.values()))
    return fixed, full, legal_full


def check_backward():
    A = (did(0,0), did(1,1), did(1,0), did(2,0))
    B = (did(2,2), did(1,0), did(0,0), did(1,1))
    ha, hb = norm((d,) for d in A), norm((d,) for d in B)
    assert map_ok(0, ha, 0, DT, hb, 0, dict(zip(A, B)))
    old_leader = 1
    added = (did(3,0), did(2,1), did(2,2), did(3,1))
    z = first_trick(0, A, added, old_leader)
    assert z is not None and z[0] == 0 and z[1] == ha
    pred_hands = norm((A[s], added[s]) for s in range(4))
    fixed, full, legal_full = backward_stats(0, pred_hands, old_leader, DT, B)
    assert full == legal_full == 0
    print("PASS backward-counterexample "
          f"current-map=preserved predecessor-trick={tuple(name(d) for _, d in z[2])} "
          f"fixed_partial_maps={fixed} full_embeddings={full} legal_embeddings={legal_full}")


def main():
    try:
        assert len(D) == 28 and sum(count(d) for d in range(28)) == 35
        check_unique(); check_k1(); groups = check_k2(); check_bisim(groups); check_backward()
        print("PASS all-checks"); return 0
    except Exception as exc:
        print(f"FAIL check {type(exc).__name__}: {exc}"); return 1


if __name__ == "__main__": sys.exit(main())
```

