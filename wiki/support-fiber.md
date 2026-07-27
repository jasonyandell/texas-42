# Capacity Cells and the Current-Remainder Fiber

[Home](Home.md) · owns: the cells, the fiber, the losslessness theorem · Sources:
both packages Math §§6–7 (shared). Related: [capacity-dp](capacity-dp.md),
[minimal-support-normal-form](minimal-support-normal-form.md),
[belief-vs-support](belief-vs-support.md).

## Three domains that must not be conflated

```
complete initial deal  →(remainder map under fixed public history)→
current hidden-remainder world  →(collect compatible ones)→
current-remainder fiber
```
A deal is fixed for the attempt; a remainder *changes type* as plays remove tiles; the
fiber is a set of remainders. Posterior belief is a measure pushed onto the fiber, not
the fiber itself [Definitions/Boundaries, Math §6.3, §6.7; README "Domain discipline"].

## The cells

Fix viewer `m` during a declared hand. From the mechanical state derive
[Definition, Math §7.1]:

- unseen pool `U` = all tiles minus own hand minus all publicly played tiles;
- per hidden seat `s`: public void set `V_s` (contexts where `s` visibly failed to
  follow), locally allowed set `P_s = U \ ⋃_{q∈V_s} σ̂_q^δ`, and capacity
  `k_s = 7 − |played by s|`.

Cells are **dependent** — they share one conserved pool (two seats, two tiles,
capacity 1 each: only 2 of 4 "independent" assignments are worlds)
[Constructed counterexample, Math §7.2, CELL-01].

The **fiber** `Φ(C)` is the *intensional* set of triples of disjoint hidden hands with
`H_s ⊆ P_s`, `|H_s| = k_s`, union `U` [Definition, Math §7.3]. Enumeration is a query
on it (up to `21!/(7!)³ = 399,072,960` worlds), never its definition; no hidden caps or
sampling [README "Correctness before tractability"]. The fiber factors through the cell
system: equal cells ⇒ equal fibers even when leader/score/contract differ [CELL-02A].

## The losslessness theorem (the keystone)

**[Theorem — proved by induction on public plays, Math §7.5, CELL-05]**
For every legal Straight public prefix in scope:

```
Φ(c) = ρ(Ω(I))
```

— the cell fiber equals the image of all rule-compatible complete deals under the
remainder map. The proof's case analysis: viewer actions don't touch hidden hands;
a hidden lead/follow removes the played tile (the tile itself is the witness — **no
positive "still holds a follower" clause survives**, CELL-06); a failure to follow
deletes the whole follow set from that seat's allowed set. Each update is reversible,
giving the **fixed-history bijection** between compatible deals and current remainders
[Corollary — proved, CELL-07].

Scope [Boundary, §7.6]: one viewer, three hidden active seats, legal Straight play,
no hand-content bid eligibility, public actor attribution, rule support only. A
contract like plunge (whose legal bid reveals a hand predicate) is outside.

Receipts: 972 deterministic reachable prefixes across all nine declarations, plays
20–28, exact set equality between fiber and replayed deal set (970 with derived voids)
[Finite verification receipt — stated corpus, CELL-07A]; typed-update algebra exhausted
on all tiny (≤3-tile) cell systems: 14,412 lead + 56,460 follow + 56,460 slough cases
[Theorem — exhaustive finite verification, TRANS-07].

## Typed transitions

Hidden actor playing `d`: the removal map is a **bijection** from the legal
predecessor subset onto the successor fiber (inverse: add `d` back). Viewer action:
identity on hidden remainders. Hence within one attempt, play never increases fiber
cardinality (hidden: ≤, viewer: =) — but note the naive `Φ(c') ⊆ Φ(c)` is
*type-incorrect* for hidden acts since pool/capacities change [Corollaries — proved,
Math §7.14, TRANS-01..05]. Literal deal-set inclusion does hold on the fixed
complete-deal domain [TRANS-04].

**rec-only strengthening**: the support *normal form* is itself a closed transition
state given declaration + typed observation context — see
[support-dynamics](support-dynamics.md).

## Initial cells

After any straight auction + declaration: `U` = the 21 unseen tiles, every `P_s = U`,
every `k_s = 7` — straight bids/declarations remove no deal by rule (they can only
*reweight* under a policy model) [Theorem — proved, Math §4.4, §7.4, AUC-07/08].
