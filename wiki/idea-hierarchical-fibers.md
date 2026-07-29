# Idea — Hierarchical Fibers: Sub-Fibers, Quotients, and Coverings

[Ideas](ideas.md) · [Home](Home.md) · owns: the pooled-fiber proposal and its
admissibility conditions · Origin: Jason's brainstorm, 2026-07-28 (the night the rob
player track went green). Related: [support-fiber](support-fiber.md),
[capacity-dp](capacity-dp.md), [support-dynamics](support-dynamics.md),
[reduced-viewer-kernel](reduced-viewer-kernel.md).

> **EXPLORATORY** — see the tier statement on [ideas](ideas.md). Every proved fact
> below is cited with its own tier and belongs to the layer it came from; the proposal
> itself has no tier and is not an expectation about what will work.

## 1. Background (cited; these are the facts, not the idea)

The rob **player track P1–P5 is green** as of 2026-07-28 (assignment
`rob/BRIEF_PLAYER_01.md`; receipt `rob/receipts/verify_rob.txt`), which is what makes
the proposal worth writing down at all — there is now a measured wall to aim at.

- **Strength** [rob receipt row `r_mat_paired`]: mirrored 200-hand paired match, rob
  vs baseline, seating margins 238 / 480, **net +718**.
- **Depth is the margin** [*exploratory ablation probes*,
  `rob/crates/verify/tests/ablation_probe.rs`, explicitly not receipt rows unless
  promoted by amendment]: myopic rob — window-1 counting at every decision, ≈27 ms per
  hand — scores **net −288** against the same baseline; full rob against myopic rob,
  mirrored, is **net +876**. Read together with `r_mat_paired`: essentially the entire
  margin is depth, and it concentrates in the solved endgame. The window-budget
  ablation in the receipt (`r_mat_window_ablation`: net 768 at B/2, 778 at 2B) says the
  same thing from the other side — the budget knob is flat where the depth knob is not.
- **The wall** [measured, P3/P4]: the trick-3 full-depth solve. The fiber bound there
  is **756,756** (bound septuple, receipt row `r_pos_bounds`) against roughly 120
  action sequences, ≈10–17 s per decision. That is the play-time ceiling; everything
  earlier is cheap and everything later is trivial.

The proposal is about that one number.

## 2. The observation

**One level of the hierarchy already exists in the code.** The P3 response-class
counting engine partitions the fiber by σ-response class — "this seat holds `r`, and
avoids `E`". That predicate *is itself a cell system* in the sense of
[support-fiber](support-fiber.md): forcing `r` into a seat drops that seat's capacity
by one and removes the tile from the pool; requiring the seat to avoid `E` shrinks its
allowed set. So the family of cell systems is **closed under σ-response conditioning**,
and the closure is not a new construction — it is exactly the force / delete / contract
moves of the slice-02 matching-minor calculus [Theorem — proved, rec Math §7.14.1–2,
TRANS-08/09; reproduced in Rust by `verify_dynamics`, conformance only]. The counting
engine is, unknowingly, one recursion step of a hierarchical engine.

**The plan tree and the fiber-refinement tree are the same tree.** Every node of a
plan (INV-P1/P2: a plan is a map from observation classes to actions) carries a bundle
— the worlds consistent with that observation path — and every such bundle is a
sub-fiber. The current engine carries bundles **extensionally**, as world lists, with
per-node conservation checked (INV-P5, receipt row `r_sol_conservation`: 58,609,267
nodes). A hierarchical engine would carry the same bundles **intensionally**, as cell
systems plus their exact counts from the capacity DP ([capacity-dp](capacity-dp.md)),
and open a bundle into worlds only where the value actually depends on which world it
is. Nothing about the plan type changes; only the representation of the bundle does.

## 3. Three rungs

Stated with their math-readiness, honestly, because the three are not equally close.

### Rung 1 — deep counting (σ-response recursion)

Recurse the response-class conditioning through successive tricks instead of stopping
after one: each node of the observation-path tree holds a cell system, its children are
the conditionings of that cell system by the next trick's response classes, and leaves
are **counted, never enumerated**. Branching is roughly pool³ per trick, so cost is
exponential in window depth — but Hall-infeasible classes prune for free (the DP
already reports infeasibility, CELL-18/19/20 via [capacity-dp](capacity-dp.md)), and
the pruning is expected to bite hardest exactly where the naive product is largest.
Rough estimate, unverified: exact H = 2 at trick 1 in minutes.

*Math readiness*: essentially available. TRANS-08's closure is the theorem this rung
needs, and it is proved in the corpus. What is missing is engineering and receipts, not
mathematics.

### Rung 2 — symmetry quotients

Two worlds that differ only by exchanging tiles that are strategically identical —
same key relations against every live tile in the declaration-relative order, same
count value — should be game-equivalent under any (ρ, σ). If so, the equivalence class
is a sub-fiber, the orbit, and the engine solves one representative per orbit.

*Math readiness*: **reserved, and not proved for this use.** The equivalence above is
stated here as a conjecture, not a theorem; the corpus has no claim that says it. The
machinery that would support it exists on both sides: `competitive_ordinal` and the
competitive-class bound [rec PLAY-12/13; rob receipt `r_alg_competitive_ordinal`, max
**13** competitive classes over all (declaration, context) pairs] gives the
declaration-relative order-isomorphism, and census slice 03's future-equivalence corpus
(5,898 machines / 17,560 state pairs, [reduced-viewer-kernel](reduced-viewer-kernel.md))
is the natural place to test a proposed equivalence exhaustively before believing it.
If it holds, this is likely the strongest single lever on the trick-3 wall, since the
13-class bound caps how much distinctness there can be per context.

### Rung 3 — coverings with sound bounds

Drop the requirement that the grouping be a partition. **Cover** the fiber, attach a
sound value interval to each element of the cover, and decide the action exactly when
one action's lower bound clears every rival's upper bound; refine only the contested
regions. This is exact branch-and-bound over the info-set tree: the answer is still
exact, the compute is spent only where the decision is close — pay extra compute
sometimes, rather than always.

*Math readiness*: furthest out. It needs sound bound theorems (a valid interval per
cover element, and a proof that refinement tightens rather than merely changes them);
none exist yet.

## 4. The admissibility law

**A grouping is admissible only with a theorem.** Either "same class ⇒ same
continuation value under (ρ, σ)" for a partition, or sound per-element bounds for a
covering. Without one of those, aggregating over classes does not approximate the
answer — it silently answers a *different question*, which is strategy-fusion's cousin:
the aggregate behaves as though one action could be chosen with knowledge that the
info-set does not contain. The project already has a witness for how badly
support-level aggregation can mislead about value
([belief-vs-support](belief-vs-support.md), the 90-world posterior flip: identical
support, opposite optimal leads), which is the reason to state this law before writing
any code, not after.

The discipline for doing it correctly already exists in the repo: the **FOLD-KEY
pattern** (`rob/BRIEF_SLICE_02.md` §6, following exchange result x:003). A refinement
enters as a **separate, proved-equivalent type** computed by a total function of the
semantic state, introduced together with an exhaustive corpus obligation — never as a
replacement for the semantic source of truth. A pooled fiber is exactly that shape of
object: a second representation, licensed per output contract, that must earn its
keep against the plain one.

The natural gate, in the repo's own idiom: **the pooled engine must produce the same
plan as the plain engine, plan-for-plan, on the frozen 756-position corpus** — the same
two-engine cross-check pattern already used by the receipt row `r_sol_engines` (540
counting-vs-streaming agreements at window 1). No pooling ships on an argument; it
ships on that diff being empty.

## 5. Candidate grouping keys (addendum, Jason 2026-07-28)

What should the quotients and coverings *key on*? Two candidates, both with
mathematics already in the corpus, plus one discipline:

- **Rank-in-suit, not pips.** Relevance is declaration-relative rank within a
  context, dynamically re-ranked as tiles fall — the 1-0 is the most important
  domino on the field in worlds where the higher blanks are gone and it leads
  the last trick into 25 points of count. The corpus already carries both
  halves: `competitive_ordinal` collapses any (declaration, context) into at
  most **13 competitive classes** [rec PLAY-12/13; rob receipt
  `r_alg_competitive_ordinal`], and the dead-cut lemma [x:003, adjudicated
  CONFIRMED] proves only the ordinal's **cut through still-live competitive
  tiles** is observable — "boss-among-the-living" is the game's own coordinate.
  A rung-2 quotient keyed this way inherits reserved math rather than inventing
  any.
- **Roles conditional on control.** "Highest live rank per suit" and "good
  lead" are candidate role features — but a good lead is worthless if you
  cannot lead, so role value is a property of (tile, control), never of the
  tile alone (HAND-07 forbids the context-free scalar for exactly this
  reason). The control coordinate is the kernel's leader state; path groups
  ([analysis](analysis.md)) expose it empirically — an action prefix exists
  only in the worlds whose intermediate tricks you won.
- **Evidence the target is small.** The P3 counting engine already factors
  the 399,072,960-world trick-one decision through a few hundred σ-response
  classes per action (the trick-one plan materializes at ~475 nodes,
  milliseconds). At window 1 the "effective world count" of the opening
  decision is therefore *measured* in the hundreds, exactly — the open
  question rung 1 answers is how that class count grows with window depth.

## 6. Order, and where it would land

Rung 1 first (the math is in hand and it is the cheapest way to learn whether the
intensional representation survives contact with the solver), then rung 2 alongside
census slice 03 — slice 03 builds the future-equivalence corpus the quotient theorem
would be tested against, so the two want to be interleaved rather than sequenced —
then rung 3, if the bound theorems materialize.

Landing site: a future player-track brief (P-series, successor to
`rob/BRIEF_PLAYER_01.md`), interleaved with census slice 03. Until such a brief exists
with named invariants and receipt rows, this page is the whole of it, at the tier
[ideas](ideas.md) declares.
