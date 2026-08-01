# Idea — Retrograde Induction over Rank Classes

[Home](Home.md) · owns: the retrograde-rank direction — endgame quotients keyed on
live rank structure, walked backward — and its probe record · Sources: none upward
(exploratory; cites proved facts as background only, per [ideas](ideas.md)).
Related: [idea-hierarchical-fibers](idea-hierarchical-fibers.md),
[reduced-viewer-kernel](reduced-viewer-kernel.md) (dead-cut, x:003),
[declaration-algebra](declaration-algebra.md) (competitive ordinal, PLAY-12/13),
[field/directions](field/directions.md) (world selection).

> **Epistemic tier: EXPLORATORY** — as [ideas](ideas.md) declares for everything it
> lists. Probe numbers below are instrument output
> (`rob/crates/verify/tests/retrograde_rank_probe.rs`), not receipt rows; they are
> quotable only here and in [analysis](analysis.md), and become results only by
> promotion.

## 1. The idea (Jason, 2026-07-31)

Classical backward induction seeds from a concrete 28-domino deal and walks pip-level
positions. The proposal inverts the seed: the *last* trick is not about pips — "if
you are leading and hold the boss of a live suit, that is literally the high trump's
situation when no trumps are live" — so walk backward over **abstract endgame
classes** keyed on the game's own coordinates: per-context rank cut through the
still-live tiles (the dead-cut coordinate x:003 proves observable), follow/slough
pattern, count carried, and control. Two hunches were stated with the idea:

- **Exactness**: the suffix minimax value factors through the live rank structure —
  no weakening to bounds will be needed (contra the burden rung 2's mid-game
  falsification suggests).
- **Reachability is where it bites**: walking backward over abstract classes, the
  interesting failures will be classes that admit few or *no* concrete realizations
  ("you couldn't add X — it doesn't lead to a valid beginning"), not value
  collisions.

The retrograde frame matters because it starts where the quotient is trivially exact
(the last trick has no future to leak) and walks toward where rung 2 broke — so it
*locates the boundary* of quotient exactness rather than conjecturing it.

## 2. Why this is not rung 2 again

The [rung-2 falsification](idea-hierarchical-fibers.md) (§7) killed per-world
**tile-exchange within a deal, replayed under a fixed plan against σ**: its static
no-go says two distinct dominoes share follow-sets over all 28 tiles only when both
are trump. Rank-preserving **substitution** is a different relation on a different
object: a live tile d is replaced by an *already-played* tile e, identity is required
only **relative to the live suffix** (follow pattern, slough pattern, and pairwise
trick-key order over L−d+e, per live lead context, equal count), and the compared
quantity is the pure both-teams-optimal minimax value — no σ, no plan replay, no
hidden-seat swap. The no-go does not apply, and the σ tie-break leakage channel that
produced rung 2's "clean" failures cannot exist here.

## 3. Probe record (frozen 2026-07-31, exploratory)

Instrument: `rob/crates/verify/tests/retrograde_rank_probe.rs` (catalog entry in
[analysis](analysis.md)). Corpus endgames `p2::boundary_position`, boundaries 6/5/4
(1/2/3 tiles per hand), 108 positions each, fiber worlds capped 6/90/16, suffix
points zeroed; minimax is an independent DFS over `RolloutPosition::legal/apply`,
cross-checked at boundary 6 against direct `resolve_trick` resolution on every
corpus world.

- **Exactness, as predicted: zero divergences.** 6,809 solved suffix worlds,
  **32,886 substitution checks, minimax value equal in every one** (b6: 4,258;
  b5: 22,884; b4: 5,744). Whole sweep ~2 s.
- **Scarcity funnel, first measurement**: the fraction of live tiles admitting any
  rank-preserving substitute *falls* as hands deepen — b6: 258/432 (60%), 3/108
  positions fully pinned; b5: 243/864 (28%), 14/108 pinned; b4: 201/1296 (16%),
  20/108 pinned. More of the live rank structure is load-bearing earlier; this is
  the first quantified surface of the reachability hunch.

What the record does **not** establish: nothing beyond boundary 4; nothing about
mid-game (windowed, imperfect-information) solves; no theorem — 32,886 agreements
are evidence the quotient conjecture is worth proving, not a proof. The natural
kernel-shaped statement ("suffix minimax factors through the live rank structure,
by structural induction over `legal`/`resolve_trick`") is a candidate for the Lean
spine or a dispatch, if promoted.

## 4. Where it goes next (unearned, listed for capture)

1. **Walk deeper** — boundaries 3/2 with memoization; find the first divergence, if
   any exists, or push the exactness frontier to the trick-3 wall.
2. **Cross-declaration substitution** — the same isomorphism check across different
   trump declarations would measure how much of the 9→3 transport collapse (x:004)
   extends dynamically to suffixes.
3. **True retrograde enumeration** — enumerate abstract rank classes directly at
   depth k, count concrete realizations per class (the reachability census of the
   idea), and back-value classes instead of positions. The scarcity funnel above
   prices the compression available.
4. **Convergence with world selection** — a rank class whose realizations all share
   a minimax value is exactly the "scenario" unit
   [field/directions](field/directions.md) wants for salience and world selection;
   the substitution test is the admissibility check for treating the class as one
   world.

## 5. Sharpening session (Jason + probe round 1, 2026-07-31 — captured, unearned)

The class-language design that came out of working the `1-0, 5-5, 3-2, 6-3` example:

- **Canonical object first, grammar second.** A last-trick class is the
  canonicalized live relation structure — for each live tile as hypothetical lead:
  who follows, who sloughs, pairwise rank order in that context; plus count,
  holder, leader. Canonical form = lexicographic minimum over suit relabelings
  (precedent: [minimal-support-normal-form](minimal-support-normal-form.md)). The
  `^A1, B1 & 10pts, C1 & D2 & 5pts, D1 & E1` notation is the *display syntax* for
  canonical objects, not the definition.
- **The class key is declaration-free.** Worked check: with `1-0` led against
  `5-5, 3-2, 6-3`, trumps 0s, 1s, and 4s give the identical relation pattern over
  the living (lone ranked tile, three sloughs) and the identical outcome — `A1`
  *specializes* to top-trump or boss-of-a-live-natural; it is not two classes.
  Trump is an existential answered by the realizability check, never a class
  coordinate. Outcome before trump, structure before both: the key is the relation
  pattern (outcome derives from it — the winner marker is a theorem, per dead-cut);
  keying on outcome alone would break composition. Consequence: realizations pool
  across declarations — the dynamic, suffix-level extension of the x:004 transport
  collapse, obtained by key design. Caution: the probe record (§3) is
  per-declaration evidence only; the cross-declaration check (§4.2) is therefore
  **load-bearing** for this key, not optional.
- **Realizability is a micro-CSP.** Enumerate candidate structures, then check by
  unification against the 28-tile universe (injective suit assignment + declaration
  + tile choice). Impossibilities (`A5` at the last trick, a 5-point double) are
  killed mechanically; two-suitedness interactions (a candidate trump absorbing a
  live tile's other end) are exactly why the check must be a machine, not a
  convention.
- **The missing theorem is commutation, not value-invariance.** Round 1 tested
  equal values on class-equivalent positions. Backward induction over classes with
  one representative per class needs more: class-equivalent positions must have
  equal *predecessor class sets* (abstraction commutes with the backward step —
  bisimulation). That is the round-2 probe question. Inter-trick glue is a single
  constraint: leader of trick t = winner of trick t−1, itself abstract-derivable.

An idea leaves this page only by promotion to a brief or a dispatch, per
[ideas](ideas.md).
