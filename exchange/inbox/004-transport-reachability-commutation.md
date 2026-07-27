---
number: 4
slug: transport-reachability-commutation
conversation: https://chatgpt.com/c/6a66e821-29c8-83ea-9f6f-4be358e6b9f7
submitted-at: 2026-07-27T05:09:54.953Z
harvested-at: 2026-07-27T13:03:23.191Z
extraction: copy-button
caveats: none
---
The conjecture is true. The transport preserves the entire **unscored legal-prefix generator**, and the induced transport of the exact hidden-hand fiber commutes with the canonical support normal form. The only dangerous point—transporting `high(d)`—is valid precisely on the uncalled domain and is not needed on called tiles.

The proof uses the hand-independent auction/declaration rules and bidder-first lead from the normative profile, together with ALG-22, ALG-23, losslessness, and the canonical support normal form/CELL-14 from the mathematical foundation.   I also ran the supplied verifier unchanged; it passed its complete existing test suite. 

## MACHINE-CHECKABLE ARTIFACTS

### 1. FINAL ANSWER

**FINAL ANSWER: TRUE (bijection proved)**

### 2. Proof

**1. Define the complete transport.**
**[USES: definition of (f_{t,u}); domino extension; context extension; declaration transport]**

Fix (t,u\in\mathbb P), write (f=f_{t,u}), and define the context map

[
\widehat f(q)=
\begin{cases}
f(q),&q\in\mathbb P,\
7,&q=7.
\end{cases}
]

The map (f) sends the singleton ({t}) bijectively to ({u}) and sends
(\mathbb P\setminus{t}) increasingly and bijectively onto
(\mathbb P\setminus{u}). Hence (f) is a permutation of (\mathbb P).
Its endpoint-wise extension is consequently a permutation of (\mathcal D).

For a complete legal generator

[
G=
\left(
(H_s^0)*{s\in\mathbb Z/4\mathbb Z},
\text{shaker},
\text{auction},
t,
((s_i,d_i))*{i<k}
\right),
]

define

[
T_{t,u}(G)=
\left(
(f(H_s^0))*s,
\text{same shaker},
\text{same auction},
u,
((s_i,f(d_i)))*{i<k}
\right).
]

Seat labels, the shaker, auction actors, bids, passes, and play actors are unchanged. Only domino identities and the declaration are transported.

Because the endpoint map is a permutation, the four transported hands remain disjoint seven-tile hands covering all 28 dominoes.

---

**2. The auction, declaration, and first leader transport unchanged.**
**[USES: hand-independent auction legality; hand-independent declaration legality; bidder leads trick 1]**

The transported auction has exactly the same public actions in exactly the same seat order. Straight bid legality depends only on the preceding public auction state and configured bid cap, not on hand contents. Therefore every transported auction action is legal exactly when its source action is legal.

The final nonpass bidder is unchanged. Declaration (u) is legally available after the same winning bid because every Straight declaration is available after every legal winning bid, independently of the bidder’s hand.

Thus the transported contracted hand has the same bidder and the same first-trick leader.

No scoring claim is made here: count labels may change under (f), but auction legality, declaration legality, and first-lead identity do not consult them.

---

**3. Called membership, effective suits, and follow incidence commute.**
**[USES: ALG-22]**

ALG-22 supplies, for every (d\in\mathcal D) and (q\in Q),

[
d\in\kappa_t
\iff
f(d)\in\kappa_u,
]

[
f!\left(\widehat\sigma_q^t\right)
=================================

\widehat\sigma_{\widehat f(q)}^u,
]

and hence

[
F_t(d,q)
========

F_u(f(d),\widehat f(q)).
]

In particular, called tiles map to called tiles, uncalled tiles map to uncalled tiles, and each source effective incidence maps to the corresponding target effective incidence.

---

**4. Led-context transport, including the `high` subtlety.**
**[USES: definition of (\ell_\delta); definition of (f_{t,u}); Step 3]**

For every (d),

[
\ell_u(f(d))=\widehat f(\ell_t(d)).
]

There are two cases.

If (d\in\kappa_t), Step 3 gives (f(d)\in\kappa_u), so

[
\ell_t(d)=7,
\qquad
\ell_u(f(d))=7
==============

\widehat f(7).
]

No assertion about `high` is needed for called tiles.

Now suppose (d\notin\kappa_t). Then neither endpoint of (d) is (t). Since (f^{-1}(u)={t}), neither endpoint of (f(d)) is (u), so (f(d)\notin\kappa_u).

Both endpoints of (d) lie in (\mathbb P\setminus{t}), and (f) is order-preserving on exactly that complement. Therefore, writing the endpoints as (a,b),

[
\max(f(a),f(b))
===============

f(\max(a,b)).
]

Consequently,

[
\ell_u(f(d))
============

# \operatorname{high}(f(d))

# f(\operatorname{high}(d))

\widehat f(\ell_t(d)).
]

Thus `high` does **not** need to commute globally. It commutes on the only domain where the led-context rule consults it: uncalled tiles. Called tiles are handled by the separate value (7).

---

**5. Whole-hand follow legality and slough legality commute.**
**[USES: Steps 3–4; exact legal-action formula]**

Let (H) be the acting hand.

On lead, the legal set is (H), whose transported legal set is plainly (f(H)).

For a follower in a trick led in context (q), let

[
G_t(H,q)=H\cap\widehat\sigma_q^t.
]

By Step 3,

[
f(G_t(H,q))
===========

f(H)\cap\widehat\sigma_{\widehat f(q)}^u.
]

Since (f) is a bijection,

[
G_t(H,q)=\varnothing
\iff
f(H)\cap\widehat\sigma_{\widehat f(q)}^u=\varnothing.
]

Therefore:

* if the source hand contains followers, the target hand contains exactly their images, and the mandatory-follow legal set maps exactly;
* if the source hand contains no follower, the target hand contains no follower, and the whole-hand slough legal set maps exactly.

Writing (A_t(H,q)) and (A_u(f(H),\widehat f(q))) for the corresponding legal sets,

[
f(A_t(H,q))
===========

A_u(f(H),\widehat f(q)).
]

Thus a particular source play is legal exactly when its transported play is legal. A source failure to follow is transported to a target failure to follow, not merely to an off-suit tile whose play might otherwise have been illegal.

---

**6. Trick winners and subsequent leaders commute.**
**[USES: ALG-22 strict contextual-order preservation; unique-winner theorem; Step 4]**

Consider a trick whose source led context is (q). By Step 4, its transported led context is (\widehat f(q)).

ALG-22 gives, for every pair (d,e),

[
\tau_t(d,q)<\tau_t(e,q)
\iff
\tau_u(f(d),\widehat f(q))
<
\tau_u(f(e),\widehat f(q)).
]

Let (d_i) be the unique source maximum. Then for every (j\ne i),

[
\tau_t(d_j,q)<\tau_t(d_i,q).
]

Transporting these strict inequalities gives

[
\tau_u(f(d_j),\widehat f(q))
<
\tau_u(f(d_i),\widehat f(q))
]

for every (j\ne i). Therefore (f(d_i)) is the unique transported maximum.

The actors attached to the four plays are unchanged, so the same seat wins the transported trick and becomes the next leader.

Winner determination uses only the strict contextual order and the association of tiles with actors. It does not use count labels or literal numerical rank coordinates.

---

**7. Induction over the complete public play prefix.**
**[USES: Steps 2, 4–6; set-difference compatibility with a bijection]**

After every source prefix of length (j) and its transported prefix, the following invariants hold:

[
H_s^{\prime,j}=f(H_s^j)
\quad\text{for every seat }s,
]

[
B_s'=f(B_s)
\quad\text{for every seat }s,
]

the current leader is the same seat, the current-trick actor sequence is the same, and every current-trick domino is the image of its source domino.

The base case (j=0) follows from Steps 1–2.

Assume the invariants before the next play. The acting seat is determined by the common leader and current-trick length, so it is identical in both prefixes. The played image tile belongs to the transported current hand. Step 5 shows that it is legal.

Removing the played tile commutes with transport:

[
f(H_s^j\setminus{d})
====================

f(H_s^j)\setminus{f(d)}.
]

Appending the play preserves the mapped current trick. If the trick remains incomplete, the leader is unchanged. If it completes, Step 6 gives the same winning seat, and both current tricks are cleared.

Induction proves all invariants for every prefix length (0\le j\le28). In particular, the transported viewer’s remaining hand is exactly the image of the source viewer’s remaining hand, and the complete transported prefix is legal.

---

**8. Public void sets transport exactly.**
**[USES: definition of (V_s); Steps 4–5; one-to-one correspondence of play events from Step 7]**

A context (q) enters (V_s) exactly when, at some follower event led in context (q), seat (s) legally plays a tile (d) satisfying

[
F_t(d,q)=0.
]

The corresponding transported event is led in (\widehat f(q)), has the same actor (s), and plays (f(d)). By Step 3,

[
F_u(f(d),\widehat f(q))=0.
]

Step 5 additionally proves that the transported hand had no target follower, so the transported nonfollow is legal.

Conversely, every target-prefix follower event is the image of the event at the same trace position, and follow/nonfollow equivalence runs in both directions. Therefore

[
V_s'=\widehat f(V_s)
]

for every seat (s).

---

**9. Pools, capacities, and raw possible-holder cells commute.**
**[USES: Step 7; Step 8; effective-suit transport from Step 3]**

Let (m) be the viewer and let (B=\bigcup_s B_s). The transported unseen pool is

[
\begin{aligned}
U'
&=
\mathcal D\setminus\left(f(H_m)\cup f(B)\right)\
&=
f!\left(
\mathcal D\setminus(H_m\cup B)
\right)\
&=
f(U),
\end{aligned}
]

because (f) is a permutation of (\mathcal D).

Actor attribution and prefix length are unchanged, so

[
k_s'=7-|B_s'|
=============

# 7-|B_s|

k_s.
]

Using Step 8 and effective-suit transport,

[
\begin{aligned}
P_s'
&=
U'\setminus
\bigcup_{q'\in V_s'}
\widehat\sigma_{q'}^u\
&=
f(U)\setminus
\bigcup_{q\in V_s}
\widehat\sigma_{\widehat f(q)}^u\
&=
f(U)\setminus
\bigcup_{q\in V_s}
f(\widehat\sigma_q^t)\
&=
f!\left(
U\setminus
\bigcup_{q\in V_s}\widehat\sigma_q^t
\right)\
&=
f(P_s).
\end{aligned}
]

Thus the complete transported cell system is

[
\mathbf C'
==========

\left(
f(U);
(f(P_s),k_s)_{s\ne m}
\right).
]

---

**10. The current hidden-hand fibers transport bijectively.**
**[USES: definition of (\Phi); Step 9; losslessness]**

Define

[
f_*:
(H_{h_1},H_{h_2},H_{h_3})
\longmapsto
(f(H_{h_1}),f(H_{h_2}),f(H_{h_3})).
]

For every source world:

* (H_s\subseteq P_s) implies (f(H_s)\subseteq f(P_s)=P_s');
* (|f(H_s)|=|H_s|=k_s);
* pairwise disjointness is preserved by injectivity;
* conservation is preserved because
  [
  \bigsqcup_s f(H_s)
  ==================

  # f!\left(\bigsqcup_sH_s\right)

  # f(U)

  U'.
  ]

The inverse permutation of (f) gives the reverse map. Hence

[
f_*:\Phi(\mathbf C)\xrightarrow{;\cong;}\Phi(\mathbf C')
]

is a bijection.

By losslessness, these fibers are exactly the compatible current hidden remainders of the two public prefixes, rather than merely abstract supersets of their legal deal support.

---

**11. The CELL-14 support normal form commutes.**
**[USES: Step 10; CELL-14/exact support normal form]**

Let

[
N=\mathcal N(\mathbf C)
=======================

\left((K_s)*s,\mathcal K*{\rm amb}\right).
]

A tile (d) is marginally held only by hidden seat (s) in the source fiber exactly when (f(d)) is marginally held only by (s) in the transported fiber. Therefore

[
K_s'=f(K_s).
]

It follows that

[
W'=f(W).
]

Tile counts and seat labels are unchanged, so residual capacities are unchanged. Consequently:

* `Determinate` remains `Determinate`;
* in a `Binary` form, the inactive seat and canonical active-seat order remain unchanged, and the stored split (q) remains unchanged;
* in a `Ternary` form, (r_0,r_1) remain unchanged and
  [
  \varepsilon'(f(d))=\varepsilon(d).
  ]

Thus the componentwise object denoted (f(N)) decodes to exactly

[
f_*(\Phi(\mathbf C)).
]

By Step 10 this is (\Phi(\mathbf C')). CELL-14 makes the feasible tagged normal form the unique canonical representation of that nonempty support fiber. Therefore

[
\boxed{
\mathcal N(\mathbf C')=f(\mathcal N(\mathbf C)).
}
]

---

**12. Explicit proof of the inverse formula.**
**[USES: definition of (f_{t,u})]**

Let

[
A=\mathbb P\setminus{t},
\qquad
B=\mathbb P\setminus{u}.
]

The restriction

[
f_{t,u}|_A:A\to B
]

is the unique order-preserving bijection between these two finite ordered sets. Its inverse

[
(f_{t,u}|_A)^{-1}:B\to A
]

is also order-preserving: if (b_1<b_2) but their inverse images satisfied
(a_1\ge a_2), strict monotonicity of (f_{t,u}|_A) would contradict
(b_1<b_2).

Moreover,

[
f_{t,u}^{-1}(u)=t.
]

Therefore (f_{t,u}^{-1}) sends (u) to (t) and is order-preserving from
(\mathbb P\setminus{u}) to (\mathbb P\setminus{t}). This is exactly the defining characterization of (f_{u,t}). Hence

[
\boxed{
f_{u,t}=f_{t,u}^{-1}.
}
]

The equality also holds when (t=u), in which case both maps are the identity.

Endpoint extension, set extension, context extension, and normal-form extension preserve composition, so the same inverse identity holds on every transported object used above.

---

**13. Forward inclusion of reachable images.**
**[USES: definition of (R_t); Steps 1–11]**

Take (S\in R_t). By definition, there is a legal contracted-hand generator (G), with declaration (t), viewer (m), and some play-prefix length, such that

[
S=\mathcal N(\mathbf C(G,m)).
]

Steps 1–7 show that (T_{t,u}(G)) is a legal contracted-hand prefix with declaration (u). Step 11 gives

[
\mathcal N(\mathbf C(T_{t,u}(G),m))
===================================

f_{t,u}(S).
]

Therefore

[
f_{t,u}(S)\in R_u,
]

and hence

[
f_{t,u}(R_t)\subseteq R_u.
]

---

**14. Reverse inclusion and bijectivity.**
**[USES: Step 12; Step 13 applied with (u,t)]**

Applying Step 13 in the reverse direction gives

[
f_{u,t}(R_u)\subseteq R_t.
]

Apply (f_{t,u}) to both sides. By Step 12,

[
R_u
===

f_{t,u}(f_{u,t}(R_u))
\subseteq
f_{t,u}(R_t).
]

Together with Step 13,

[
\boxed{
f_{t,u}(R_t)=R_u.
}
]

The restriction (f_{t,u}|*{R_t}) is injective because its global inverse on normal forms is (f*{u,t}), and it is surjective by the displayed equality. Therefore

[
\boxed{
f_{t,u}|_{R_t}:R_t\xrightarrow{;\cong;}R_u
}
]

is a bijection with inverse (f_{u,t}|_{R_u}).

---

**15. Exact three-class census consequence.**
**[USES: Step 14; ALG-23]**

For all pip trumps (t,u),

[
|R_t|=|R_u|.
]

Write this common value as

[
r_{\rm pip}=|R_t|.
]

Choose a canonical pip trump (t_0). Then

[
R_t=f_{t_0,t}(R_{t_0})
]

and

[
R_{\rm pip}
:=
\bigcup_{t\in\mathbb P}R_t
==========================

\bigcup_{t\in\mathbb P}f_{t_0,t}(R_{t_0}).
]

The untagged reachable image has the precise three-term form

[
\boxed{
R=R_{\rm pip}\cup R_{\rm DT}\cup R_{\rm NT}.
}
]

This is not asserted to be a disjoint union.

For the genuinely declaration-tagged disjoint census

[
\widetilde R
============

\bigsqcup_{\delta}
{\delta}\times R_\delta,
]

one has the exact cardinality

[
\boxed{
|\widetilde R|
==============

7r_{\rm pip}
+
|R_{\rm DT}|
+
|R_{\rm NT}|.
}
]

If tagged pip objects are then quotiented by

[
(t,S)\sim
\left(u,f_{t,u}(S)\right),
]

each pip orbit has exactly one representative with declaration (t_0). The resulting mechanics-class quotient has cardinality

[
\boxed{
r_{\rm pip}
+
|R_{\rm DT}|
+
|R_{\rm NT}|.
}
]

Thus the theorem collapses nine independent reachability analyses to three mechanics classes. It does **not** by itself permit replacing (7r_{\rm pip}) by (r_{\rm pip}) when the exact physical pip declaration remains part of a disjoint tagged state.

---

**16. A rigorous quantified overlap shared by all nine declarations.**
**[USES: hand-independent declarations; initial cells; CELL-14]**

Before any play, fix an arbitrary viewer hand (H\subseteq\mathcal D) with (|H|=7). Under every one of the nine declarations,

[
U=\mathcal D\setminus H,
\qquad
k_1=k_2=k_3=7,
\qquad
V_1=V_2=V_3=\varnothing,
\qquad
P_1=P_2=P_3=U.
]

Every unseen tile is marginally possible at every hidden seat. The common normal form is therefore

[
K_1=K_2=K_3=\varnothing,
]

[
\mathsf{Ternary}
\left(
W=\mathcal D\setminus H,,
r_0=7,,
r_1=7,,
\varepsilon=\varnothing
\right),
]

with (r_2=7) by conservation.

Different viewer hands give different ambiguous pools, hence different normal forms. Therefore

[
\boxed{
\left|
\bigcap_{\delta\in\Delta_{\rm straight}}R_\delta
\right|
\ge
\binom{28}{7}
=============

1{,}184{,}040.
}
]

Let

[
a=\binom{28}{7}.
]

The common initial family gives the rigorous union bound

[
\boxed{
\max\left(
r_{\rm pip},
|R_{\rm DT}|,
|R_{\rm NT}|
\right)
\le |R|
\le
7r_{\rm pip}
+
|R_{\rm DT}|
+
|R_{\rm NT}|
------------

8a.
}
]

The subtraction follows because the same (a)-element family occurs in all nine declaration images. Additional overlap—among distinct pip-trump images and between pip, DT, and NT—remains uncontrolled and belongs to the unresolved structure of OPEN-11.

---

**17. Scope boundary.**
**[USES: count-blind definition of support; stated nonpreservation of (c)]**

The theorem is exact for legal play prefixes, capacity cells, fibers, and support normal forms. It does not transport:

* count labels in general;
* trick point totals;
* contract outcomes;
* score-conditioned objects;
* beliefs or policy likelihoods;
* strategic values.

A complete 28-play trace transports to another complete legal 28-play trace, but the two hands may have different point allocations because (c(f(d))) need not equal (c(d)).

### 3. Self-contained Python 3 verification program

The deterministic generator uses:

* 32 SplitMix64/Fisher–Yates deals per pip trump;
* all seven pip-trump declarations;
* rotating bidders and viewers covering all four seats under every trump;
* a deterministic legal-play selector;
* every depth (0,\ldots,28) from every complete trace;
* all seven target trumps (u) for every generated prefix.

That gives:

[
7\cdot32\cdot29=6{,}496
]

source prefixes and

[
6{,}496\cdot7=45{,}472
]

transport checks. The observed run found 4,870 prefixes with public voids and hundreds of sloughs under every trump.

Observed output:

```text
PASS ALG-22 49_ordered_pairs 307328_contextual_comparisons
PASS INVERSE 49_ordered_pairs
PASS GENERATOR 6496_prefixes 7_trumps depths_0_28 224_distinct_deals 4870_prefixes_with_voids sloughs_by_trump=(294, 278, 311, 290, 304, 295, 290)
PASS TRACE_TRANSPORT 45472_transported_prefixes_legal
PASS SUPPORT_COMMUTATION 45472_normal_form_equalities
PASS ALL
```

The program ran in 10.83 seconds in the verification environment. It performs no network or file I/O and exits zero exactly on complete success.

[Download the exact verifier](sandbox:/mnt/data/verify_transport_reachability.py)
SHA-256: `c56c0c5003f49582c988efbb8600f68f498979ade1a0354c9f50ad94a96541d3`

```python
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
```

### 4. Witnesses

Not applicable: the conjecture is true, so there is no FALSE witness or non-membership exhaustion JSON.

