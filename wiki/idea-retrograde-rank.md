# Idea — Retrograde Induction over Constellations

[Home](Home.md) · owns: the retrograde direction — endgame quotients keyed on
constellations (canonical arrangements of standings among the living), walked
backward — and its probe record · Sources: none upward (exploratory; cites proved
facts as background only, per [ideas](ideas.md)).
Related: [idea-hierarchical-fibers](idea-hierarchical-fibers.md),
[reduced-viewer-kernel](reduced-viewer-kernel.md) (dead-cut, x:003),
[declaration-algebra](declaration-algebra.md) (competitive ordinal, PLAY-12/13),
[minimal-support-normal-form](minimal-support-normal-form.md) (lex-min precedent),
[field/directions](field/directions.md) (world selection).

> **Epistemic tier: EXPLORATORY** — as [ideas](ideas.md) declares for everything it
> lists. Probe numbers below are instrument output
> (`rob/crates/verify/tests/retrograde_rank_probe.rs`), not receipt rows; they are
> quotable only here and in [analysis](analysis.md), and become results only by
> promotion. The probe file and its test names predate the vocabulary in §1 and are
> kept as the frozen instrument's identity.

## 1. Ubiquitous language (settled with Jason, 2026-07-31 → 2026-08-01)

- **Standing** — a tile's relational position among the *living* tiles: who it
  follows, who it sloughs against, who takes precedence over it, per lead context.
  Standings are inherently **partially ordered**; equal standings and incomparable
  standings both exist. "Boss" survives as the informal word for top standing — a
  singleton exactly because standing isn't. NOT "rank": `competitive_ordinal`
  ([declaration-algebra](declaration-algebra.md)) owns that word (static 1..13 over
  all 28 tiles); standing is relative to the living only.
- **Constellation** — the canonical class object: the arrangement of standings,
  counts, holders, and lead. Defined like its namesake: identity lives in the
  **relations among points, not in which stars fill them**. Same constellation,
  different tiles. Round 1's finding, restated: *the suffix minimax value is a
  function of the constellation, not the tiles.*
- **Suit labels are colors, not letters.** The `A, B, C` of the display grammar are
  green/blue/gold: distinct, **unordered**, meaningless beyond distinctness.
  Canonicalization is lex-min over color permutations *because* colors carry no
  order of their own.
- **Realization** — a concrete assignment of actual tiles (and a declaration)
  filling a constellation. Trump is an **existential answered by realization**,
  never part of a constellation's identity (§5).
- The winner marker (`^`) is **derivable from the constellation** (dead-cut,
  x:003), never specified.
- **"Coordinate" is retired on this page — it was a type error (Jason,
  2026-08-01).** A coordinate presupposes a space of independent axes in which a
  position is a tuple of values; a constellation has no axes — its identity is a
  pattern of relations with the names rubbed out (lat/lon does not identify a
  constellation in the sky, either). So the honest statement is not "trump is not a
  constellation coordinate" but stronger: *nothing* is. The forward algebra's
  pip-level description remains **a** true representation of 42 — exact and
  useful — that had been silently promoted to *the* representation; the
  constellation lens is a second chart on the same territory, with its own
  symmetries and compressions unexplored.

## 2. The idea (Jason, 2026-07-31)

Classical backward induction seeds from a concrete 28-domino deal and walks pip-level
positions. The proposal inverts the seed: the *last* trick is not about pips — "if
you are leading and hold the boss of a live suit, that is literally the high trump's
situation when no trumps are live" — so walk backward over **constellations**: the
game described in its own relational terms — standings among the still-live tiles
(the dead-cut x:003 proves observable), follow/slough pattern, count carried, and
control. Two hunches were stated with the idea:

- **Exactness**: the suffix minimax value factors through the constellation — no
  weakening to bounds will be needed (contra the burden rung 2's mid-game
  falsification suggests).
- **Reachability is where it bites**: walking backward over constellations, the
  interesting failures will be constellations that admit few or *no* concrete
  realizations ("you couldn't add X — it doesn't lead to a valid beginning"), not
  value collisions.

The retrograde frame matters because it starts where the quotient is trivially exact
(the last trick has no future to leak) and walks toward where rung 2 broke — so it
*locates the boundary* of quotient exactness rather than conjecturing it.

## 3. Why this is not rung 2 again

The [rung-2 falsification](idea-hierarchical-fibers.md) (§7) killed per-world
**tile-exchange within a deal, replayed under a fixed plan against σ**: its static
no-go says two distinct dominoes share follow-sets over all 28 tiles only when both
are trump. Standing-preserving **substitution** is a different relation on a
different object: a live tile d is replaced by an *already-played* tile e, identity
is required only **relative to the live suffix** (follow pattern, slough pattern,
and pairwise trick-key order over L−d+e, per live lead context, equal count), and
the compared quantity is the pure both-teams-optimal minimax value — no σ, no plan
replay, no hidden-seat swap. The no-go does not apply, and the σ tie-break leakage
channel that produced rung 2's "clean" failures cannot exist here.

## 4. Probe record (frozen 2026-07-31, exploratory)

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
  standing-preserving substitute *falls* as hands deepen — b6: 258/432 (60%), 3/108
  positions fully pinned; b5: 243/864 (28%), 14/108 pinned; b4: 201/1296 (16%),
  20/108 pinned. More of the constellation is load-bearing earlier; this is the
  first quantified surface of the reachability hunch.

What the record does **not** establish: nothing beyond boundary 4; nothing about
mid-game (windowed, imperfect-information) solves; no theorem — 32,886 agreements
are evidence the quotient conjecture is worth proving, not a proof. Caveat
(Jason, 2026-08-01): the whole record, scarcity funnel included, was measured
**trumps-declared-first** — inside a per-declaration coordinate frame. It is
evidence about that frame; it is not a measurement of the constellation space of
§5, which pools declarations, and conclusions should not be carried across
without the cross-declaration check (§6.2). The natural
kernel-shaped statement ("suffix minimax factors through the constellation, by
structural induction over `legal`/`resolve_trick`") is a candidate for the Lean
spine or a dispatch, if promoted.

## 5. The pattern shape (sharpened 2026-07-31 → 2026-08-01)

The definition that came out of working the `1-0, 5-5, 3-2, 6-3` example, reworked
into the shape of mathematics already in the corpus (lex-min canonicalization,
unification, the suit-permutation transports).

- **Canonical object first, grammar second.** A constellation's carrier is a small
  edge-labeled multigraph: vertices are suit-colors, a live tile is an **edge**
  between the two suits on its ends — a double is a **loop** — with each end
  carrying that tile's standing in that suit, each edge carrying its count points
  and holder, and the lead marked. On top of the carrier sits the live relation
  structure the round-1 probe already checks: per-lead-context follow pattern,
  slough pattern, and pairwise precedence among the living. Identity is the whole
  thing **up to relabeling the vertices** — that is the entire content of
  colors-not-letters. Canonical form = lex-min over color relabelings (precedent:
  [minimal-support-normal-form](minimal-support-normal-form.md)).
- **The universe is K7-with-loops.** The 28 dominoes are exactly the edges of the
  complete graph on the 7 pips, loops included (21 + 7). A **realization** is an
  embedding of the constellation's graph into K7-with-loops — injective on colors —
  together with a declaration, such that the image's induced standings, counts, and
  precedence relations are exactly the pattern's. The realizability micro-CSP *is*
  this embedding problem, and impossibilities die mechanically at the right layer:
  `A5` at the last trick demands five live edges at one vertex among four tiles
  (unsatisfiable before pips are even mentioned); "a 5-point double" demands a loop
  carrying 5 count (no such loop exists in K7's labeling). Two-suitedness needs no
  side conditions — a tile touching two suits is an edge touching two vertices, and
  "whatever is on the other end of the led tile differs from B..E" is just which
  vertex that edge touches.
- **The grammar is serialization, not definition.** `^A1, B1 & 10pts, C1 & D2 &
  5pts, D1 & E1` is display syntax — a linearization of the canonical graph (`B1 &
  10pts` is the loop at B with 10 count; `C1 & D2 & 5pts` is the C–D edge; the D
  shared with `D1 & E1` is a shared vertex, i.e. graph adjacency). Two strings that
  differ by a color permutation are the same constellation. Enumerating last-trick
  classes is enumerating little graphs up to relabeling, then filtering by
  embeddability — not generating strings.
- **Trump lives in witnesses, not in the pattern.** Worked check (Jason): `1-0` led
  against `5-5, 3-2, 6-3` — trumps 0s, 1s, and 4s all realize the identical
  relation pattern over the living (lone tile of top precedence, three sloughs) and
  the identical outcome; `A1` specializes to led-trump or boss-of-a-live-natural
  per witness. Trump-ness enters the constellation only through the precedence
  relations it induces; the pattern keeps the relations and forgets the name.
  Outcome before trump, structure before both: the winner marker is a theorem
  (dead-cut), and keying on outcome alone would break composition. Consequence:
  realizations pool across declarations — the dynamic, suffix-level extension of
  the x:004 transport collapse, obtained by key design. Caution: the probe record
  (§4) is per-declaration evidence only; the cross-declaration check (§6.2) is
  therefore **load-bearing** for this pooling, not optional.
- **The backward step concretizes and re-abstracts — and audits itself.** To back
  up from a constellation: enumerate its realizations, un-play one trick per legal
  predecessor (inter-trick glue is a single constraint: leader of trick t = winner
  of trick t−1, itself abstract-derivable), re-abstract each predecessor position,
  memoize. Backing up resurrects dead tiles into the living set, which can demote
  standings that were only boss-among-the-living — the backward step is where the
  relations-not-stars framing is stress-tested hardest. Run from *every*
  realization of a class, the loop generates the commutation evidence as a
  byproduct: agreement of the abstracted predecessor sets across a class's
  realizations, checked per class, is exactly the license for
  one-representative-per-class induction (abstraction commutes with the backward
  step — bisimulation). That is the round-2 gate, and it falls out of the
  enumeration architecture rather than preceding it; where realizations disagree,
  the predicted reachability-shaped failure is caught with the witness pair in
  hand.
- **Two quotients are in play; induction keeps the finer one.** A follower's suit
  membership is not intrinsic (`A2|*`: second standing in the led suit and off-suit
  chaff both lose to `A1` at the last trick) — some distinctions are
  value-irrelevant at k=1 but dynamics-relevant when backing up. The constellation
  (full relation pattern) is the candidate dynamics-fine quotient; the value-coarse
  one is the salience/bound-separation sibling idea
  ([field/directions](field/directions.md)), deliberately queued behind this.

## 6. Where it goes next (unearned, listed for capture)

1. **Walk deeper** — boundaries 3/2 with memoization; find the first divergence, if
   any exists, or push the exactness frontier to the trick-3 wall.
2. **Cross-declaration substitution** — the same isomorphism check across different
   trump declarations, grouped by constellation; load-bearing for §5's pooling
   claim and the measure of how much of the 9→3 transport collapse (x:004) extends
   dynamically to suffixes.
3. **True retrograde enumeration** — enumerate constellations directly at depth k,
   count realizations per constellation via the embedding check (the reachability
   census of the idea), and back-value constellations instead of positions, with
   the per-class commutation audit of §5 asserted inside the loop. The scarcity
   funnel above prices the compression available.
4. **Convergence with world selection** — a constellation whose realizations all
   share a minimax value is exactly the "scenario" unit
   [field/directions](field/directions.md) wants for salience and world selection;
   the substitution test is the admissibility check for treating the class as one
   world.

An idea leaves this page only by promotion to a brief or a dispatch, per
[ideas](ideas.md).
