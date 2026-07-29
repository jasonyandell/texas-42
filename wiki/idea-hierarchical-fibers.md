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

## 6. First contact — the rung-1 prototype (2026-07-28, exploratory)

*Everything in this section is exploratory instrument output
([analysis](analysis.md) tier): measured, reproducible, cited to files, and a receipt
row nowhere. Nothing here changes the idea's tier.*

Rung 1 now exists as code: `rob_player::solver::gate::counting_deep`
(`rob/crates/player/src/solver.rs`, gate-only — play never routes through it, INV-P6)
recurses the σ-response-class conditioning across trick boundaries. Every bundle is
carried intensionally as a conditioned cell system `(required, excluded)` over the
root pool; σ *leads* condition the same way σ responses do (hold `r`, hold nothing
lead-preferred over `r`); leaves are counted by the capacity DP and never enumerated;
zero-count extensions prune their subtrees exactly. Two depth-only subtleties the
window-1 engine never meets are handled and documented in the code: a candidate
already excluded for its seat must be skipped (the DP ignores exclusions on required
tiles, so requiring it would silently count a contradicted class), and tiles already
played can no longer preempt σ's choice.

**The §4 gate, passed in miniature** (`rob/crates/verify/tests/hierarchical_fiber_probe.rs`):
whole-`Plan` equality — values, actions, observation keys, bundle counts, leaf kinds —
between the intensional engine and the certified extensional engines, on every
position tested:

- **756** window-1 plans ≡ the receipted counting engine (all seven boundaries,
  including the 399,072,960-world boundary-0 fibers);
- **324** full-depth plans ≡ streaming (boundaries 4–6);
- **216** plans ≡ streaming at boundary 3 (windows 2 and 4, all 108 positions);
- **6** window-2 plans ≡ streaming at boundary 2 — the trick-3 wall's own fibers,
  up to 756,756 worlds.

**The open question of §5, answered at first contact.** Exact H = 2 at the trick-one
fiber is not minutes — it is **7–10 seconds** per decision (positions 0–2 of the P2
corpus, release build, single thread): 3.6–5.1 M capacity-DP calls, 0.9–1.2 M nonzero
leaf classes against 399,072,960 worlds (≈ 350× intensional compression), with
**72–74 % of all σ-class extensions pruned as exactly infeasible** — the pruning does
bite hardest where the naive product is largest. At all three positions the H = 2
opening *confirms* the H = 1 opening; the value ranking of the rejected plans
reshuffles (at position 0 the runner-up changes), which is what a window-rent
measurement at depth would want to know. One step deeper: exact **H = 3** at position
0 runs in **~4 minutes** (49.6 M leaf classes, 213.7 M DP calls, 73 % pruned, opening
confirmed again). The class tree grows ≈ 55× per depth step, so the intensional
compression against the fixed fiber erodes from ≈ 350× at H = 2 to ≈ 8× at H = 3 —
the trick-one crossover sits near H ≈ 4.

**The honest negative.** The same probe's class-growth table shows the intensional
representation *losing* to enumeration where fibers are small: at boundary 3, full
depth, 12 positions carry 256,690 worlds but 3.8 M leaf classes — the class tree
outgrows the fiber, because class counts multiply per trick while fibers only shrink.
Deep counting pays exactly where the streaming engine is priced out (early tricks,
huge fibers) and nowhere else. That is §2's hierarchy claim made quantitative: the
engine of interest is a *hybrid* that carries bundles intensionally while they are
large and opens them into worlds when they are small — the crossover is now a
measurable quantity, not a guess.

## 7. Rung 2, falsified as stated — and where the quotient actually lives (2026-07-29, exploratory)

*Exploratory instrument output (`rob/crates/verify/tests/strategic_exchange_probe.rs`);
tier as §6.*

Two results, one static and one measured.

**Static:** the §3 conjecture's orbit generators barely exist. Two distinct dominoes
have equal follow-sets in every context only when both are trump — a trump domino
belongs to the trump suit alone, while a non-trump domino follows both its natural
suits, and no two distinct dominoes share both pips. So "strategically identical"
pairs are only equal-count trump pairs adjacent in the live key order; everything
else was never exchangeable even in principle.

**Measured:** even those pairs do not pool. 160,012 cross-seat swaps over 36 solved
corpus positions, the solved plan replayed against σ in the world and its swap:
**28 % of swaps change the outcome.** The failure taxonomy is structural, not noise:

| swap class | equal | differ (pair collided in a trick) | differ (clean) |
|---|---|---|---|
| cross-team, id-gapped | 55,004 | 35,124 | 30 |
| cross-team, id-adjacent | 10,044 | 4,024 | 12 |
| same-team, id-gapped | 43,324 | 5,130 | 22 |
| same-team, id-adjacent | 6,620 | 672 | 6 |

- **Collision kills exchange**: when both pair members reach the same trick, the swap
  flips which *seat* wins it. Cross-team that moves points; same-team it still moves
  the **leader**, and the continuation diverges (5,802 same-team collision failures) —
  control is a coordinate, exactly as §5's "roles conditional on control" warned.
- **The clean channel is tie-break leakage**: σ breaks slough ties by raw id, and the
  viewer's plan branches on the observed tile identity, so even collision-free swaps
  can diverge (70 cases). No static side condition tested (team, id-adjacency)
  eliminates them.

Conclusion: **no per-world tile-exchange quotient survives for exact solving.** The
remaining node-level symmetry — merge observation branches whose conditioned support
and folded trick state coincide (the `(N, τ)` DAG licensed by
[support-dynamics](support-dynamics.md) TRANS-08/09 and PLAY-12's fold congruence) —
was then **measured, and it is weak within a single solve**: with folded-trick,
presentation-level node keys (`gate::counting_deep_dedup`, value equality asserted on
every collision — PLAY-12 held on all of them), dedup is **1.00×** at window 2
everywhere, **1.11×** at boundary 4 / window 3, **1.38×** at boundary 3 / full depth.
The explanation is structural: distinct observation paths almost always leave
distinct played-tile multisets, so their sub-fibers genuinely differ; within-window
transpositions are rare. What this leaves open, unmeasured and plausibly much
better, is **cross-solve reuse** — rob re-solves at every decision, and consecutive
solves share most of their (N, τ) frontier. Falsifying both forms cost nothing; the
exchange budget was not touched.

## 8. Feature factorization — "where's the count", priced (2026-07-29, exploratory)

*Exploratory instrument output (`rob/crates/verify/tests/fiber_factor_probe.rs`);
tier as §6. Origin: Jason's factored-fiber brainstorm (quad-tree over
where's-the-count × where-are-my-beaters).*

The coordinate: the assignment of the live count tiles to the three hidden seats —
≤ 3⁵ cells, each an intensional sub-fiber (tile→seat conditioning is the force move,
TRANS-08/09; and per [minimal-support-normal-form](minimal-support-normal-form.md),
tiles certain after reduction need no split, so the tree only branches on genuine
ambiguity). The probe prices the coordinate in the game's own currency:
**VOI(coordinate)** = the exact value of learning it before committing to an opening
(cell-best minus global-best margin sums, a lower bound — a cell-aware re-solve
would do better), against **VOI(full)**, the perfect-information gap. Replayed
margins reproduce every `solve_opening` value exactly (self-validation).

Measured (24 boundary-3 positions; 2 boundary-2 wall positions):

- **The coordinate is decision-relevant precisely at the wall.** At boundary 2,
  index 1: only **96‰ of worlds** lie in cells agreeing with the global opening —
  8 of 9 cells contested; the trick-3 decision *hangs* on where the count sits. At
  boundary 3 the agree-share ranges 254‰–1000‰ across positions.
- **Two coordinates capture up to ≈ 60 % of the gap.** Count-location alone:
  0–390‰ of VOI(full). Composed with where-are-the-beaters (of the opening tile,
  refining contested cells only): up to **613‰** at boundary 3, **592‰** at the
  wall — 1,587 sub-cells standing in for 72,072 worlds.
- **Zero is a result too**: at four boundary-3 positions VOI(coordinate) = 0 — the
  global opening is optimal in *every* cell, so the decision is provably insensitive
  to count location there, and a hierarchical engine would spend nothing on it.
  Where a coordinate captures nothing, refinement is pruned; where it captures much,
  the contested cells are exactly rung 3's refinement frontier.

Read together with §7: the factored fiber should be built from **feature coordinates
for the split policy** (count location, beater location, control) and **kernel
equality for the merge policy** (the `(N, τ)` DAG) — split where the decision is
contested, merge where continuations provably coincide.

## 9. Rung 3 without new theorems — sound bounds, measured at the wall (2026-07-29, exploratory)

*Exploratory instrument output (`fiber_factor_probe.rs`, the `bound_cover_*` tests);
tier as §6.*

§3 called rung 3 "furthest out" for want of sound bound theorems. Two bounds already
in the machinery need no new mathematics, and both **decompose over any cell
partition**:

- **Upper**: `U_a(S) = Σ_{w∈S}` (perfect-information best margin against σ, first
  move pinned to `a`) — the known-world gate's quantity, summed. Sound because one
  plan per information set can never beat per-world best play (max of sums ≤ sum of
  maxes). A ≤ 5!-path DFS per world; no theorems, no tuning.
- **Lower**: any concrete plan's replayed margins summed over `S`.

An opening is *decided* on `S` when the incumbent's lower clears every rival's
upper — exact branch-and-bound over the info-set tree, refined along §8's feature
coordinates exactly where the bounds fail to separate. Soundness (`U_a ≥ V_a`)
asserted against the exact solver at every position.

Measured (12 boundary-3 positions + the boundary-2 wall position, index 1):

- **Bounds alone settle the root decision at 10 of 13 positions** — every rival's
  ceiling below the exact best value. At the wall (72,072 worlds, the position §8
  found most contested): **root decided**, closest rival 165,504 against V* 174,554,
  in ≈ 6 s of DFS — so the trick-3 decision there needs *one* full exact solve (the
  incumbent) instead of five.
- **The clairvoyance premium `U − V` is small**: 0–353‰ of |V|, exactly 0 at two
  positions (the info-set plan attains per-world-perfect value), 64‰ at the wall.
  This number is also the exact strategy-fusion gap a PIMC-style sampler would
  silently pocket — now priced per position.
- Cell-level: decided-cell world-shares of 0–1000‰ at count-location granularity;
  the 0‰ position (boundary 3, index 6) is also §8's worst coordinate fit — where
  bounds and features both fail, the exact solver is genuinely needed, and that is
  the honest residue.

Together §§6–9 give the factored-fiber architecture its measured shape: **deep
counting where fibers are huge (§6), feature splits where the decision is contested
(§8), bound elimination before exact solving (§9) — and no world-level symmetry
quotient (§7).**

## 10. Order, and where it would land

Rung 1 first (the math is in hand and it is the cheapest way to learn whether the
intensional representation survives contact with the solver), then rung 2 alongside
census slice 03 — slice 03 builds the future-equivalence corpus the quotient theorem
would be tested against, so the two want to be interleaved rather than sequenced —
then rung 3, if the bound theorems materialize.

Landing site: a future player-track brief (P-series, successor to
`rob/BRIEF_PLAYER_01.md`), interleaved with census slice 03. Until such a brief exists
with named invariants and receipt rows, this page is the whole of it, at the tier
[ideas](ideas.md) declares.
