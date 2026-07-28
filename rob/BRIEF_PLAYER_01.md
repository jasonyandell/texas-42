# rob — Player Brief 01: the endgame plan solver

This is the definitive first assignment for **rob the player** — the imperfect-
information player this repository was created to make possible. It opens a
**player track** (stages P1…) parallel to the census/kernel slice ladder;
[BRIEF.md](BRIEF.md) §1 governing synthesis, §2 toolchain, §5 invariants
INV-1..10, §9 receipt/CI rules, and §11 ambiguity protocol, together with
[BRIEF_SLICE_02.md](BRIEF_SLICE_02.md) §5 INV-11..14 and §9.1 tier labeling,
**remain binding verbatim**. Slices 01 and 02 are green and committed; this track
begins only from that state and consumes only their public surface.

Scope, in one sentence: **rob v1 — an exact information-set best-response plan
solver over the exact fiber, taking over at four tricks remaining and playing a
materialized whole-hand contingent plan to the end of the hand, against a fixed
deterministic field policy σ, under the uniform (W0) weighting.**

Citation convention as in the prior briefs, plus **mk5** = Jason's prior
perfect-information project (`~/code/mk5-main/wiki/`), citable for design
precedent only — never for definitions, numbers, or code.

---

## 1. Governing synthesis (already decided — do not relitigate)

Decided 2026-07-27/28, recorded here as binding:

- **Naming law.** The player specified by this brief **is rob**. The slice-01
  fixed-field Monte Carlo player (`MonteCarloPlayer`, Math §11.4) is demoted to
  **baseline / receipt mode**: it keeps its receipts (`verify_player.txt`) and
  its role as paired-match opponent, and it gets no name. Doc comments may be
  updated to say "baseline"; its receipt text does not change.
- **PLAN-NOT-TILE (the hand-as-unit law, binding).** The decision variable
  ranges over whole-hand contingent plans ρ — one action per viewer information
  set — and the played tile is the plan's first move. No API anywhere in the
  workspace returns a scalar "value of a domino" (this sharpens BRIEF.md §5's
  HAND-07 guardrail from a modeling statement into an API prohibition). Any
  per-tile number shown to a human is a typed projection of a plan, produced
  only by the trace/display layer.
- **Materialized plan.** rob's solver returns the plan as an explicit tree
  (`Plan`), not an implicit re-solve convention. Decided for inspectability,
  receipts, and gate enforcement.
- **Fiber = information set.** rob's knowledge object at a decision is exactly
  the fiber Φ(C) of the derived cell system (CELL-05/07). No sampling, no
  approximation, anywhere in the solve path. The count-ratio sampler is not
  used by this track.
- **σ is fixed, deterministic, and humble.** The other three seats — including
  the partner — are modeled by one fixed deterministic policy σ (§7). σ's
  determinism is the compression that makes the solve exact and small; σ's
  quality is explicitly *not* a v1 goal.
- **W0 before W1 (term 2 before term 1).** The uniform weighting over the fiber
  ships first and is the v1 deliverable; the σ-consistency history filter (W1)
  is the stretch stage P6, with its ablation. Precedent: mk5's graded finding
  that belief tilt is only cashable through honest info-set continuation.
- **Endgame takeover, exact to the end.** rob v1 solves only when **≤ 4 tricks
  remain**; the unroll runs to the end of the hand, so there is **no leaf
  evaluation function in this brief**. Before takeover, rob's seat is played by
  the baseline. Deeper horizons, leaf evaluations, and adaptive-H budgeting are
  a later brief, listed out of scope in §4.
- **Registered rent (from mk5's walt, honestly carried).** (a) Field-model
  rent: rob's opponents in the paired match are the *baseline*, not σ — so v1
  measures a best response to a wrong model against a real opponent, which is
  the honest experiment. (b) Partner rent: rob models his partner as field even
  though the partner seat is also rob; seats never communicate (legal play,
  same rule as humans).

Authority order: this brief; then BRIEF.md + BRIEF_SLICE_02.md; then `wiki/`;
then ingest; then exchange results (tier-labeled). mk5 ranks below all of them
and is precedent only.

---

## 2. Language and toolchain (unchanged)

As BRIEF.md §2 in full. No new dependencies in any crate. No floats (INV-4);
all solver arithmetic is machine-integer with overflow checks — §8 shows the
bounds fit comfortably in `u64`/`i64`. `rob/crates/core` is **read-only** to
this track: the needed surface (`derive_rule_cells`, `fiber_worlds`,
`fiber_contains`, the exact counting routes, the mechanical machine, the
declaration algebra) already exists. If a core addition appears necessary, that
is a finding under the ambiguity protocol, not a local edit.

---

## 3. Project layout (additions only)

All additions live in `rob/crates/player` and `rob/crates/player/src/bin`, plus
one receipt file and the inspector page (§14 module map). `wiki/`, `ingest/`,
`exchange/`, and `rob/crates/core` are untouched.

---

## 4. Scope: player track v1 and its boundaries

Five gated stages plus one stretch stage. A stage begins only when the previous
stage's receipts are green and committed.

- **P1 — σ, the field policy**: `GreedySigma`, deterministic, total, legal;
  σ-self-play receipts over the 108-hand corpus deals.
- **P2 — endgame position corpus + fiber enumeration**: 432 trick-boundary
  positions from the S3 corpus; exact fiber enumeration cross-checked against
  the capacity-DP count.
- **P3 — the solver (W0) and the Plan type**: exact backward induction over the
  info-set tree; the four correctness gates.
- **P4 — the composite player + paired match**: baseline opening, endgame
  takeover, plan followed to the end of the hand; mirrored paired match vs the
  baseline; frozen margin.
- **P5 — the contingency book**: Plan-tree trace emission and inspector view.
- **P6 (stretch, optional)** — the W1 σ-consistency history filter with
  empty-filter fallback and the W0-vs-W1 ablation match.

**Explicitly out of scope** (do not begin, do not scaffold speculative APIs
for):

- Any decision with **more than 4 tricks remaining** beyond delegating to the
  baseline: no leaf evaluation function, no adaptive horizon, no early-game
  solver. (The `Plan` type has no leaf/heuristic variant — takeover exactness
  is structural, §5 INV-P6.)
- **Belief distributions** beyond P6's σ-consistency filter — no Bayes over
  augmented worlds, no posteriors, nothing from census slice 04; the 90-world
  posterior flip stays untouched.
- **Census slices 03–05** (folded trick / reduced viewer kernel, belief layer,
  symbolic-DAG census) — this track neither begins them nor depends on them.
- **NF/equivalence pooling of worlds inside the solve** (the "identical within
  the window" optimization) — registered as a later-brief idea; v1 iterates the
  enumerated fiber plainly.
- **Bidding** — the placeholder auction stands; rob v1 is a play-phase player.
- Opponent modeling beyond the fixed σ; σ tuning; any learning; optimization
  caches before correctness — never.

---

## 5. Named invariants

INV-1..14 inherited unchanged. Six new invariants, P-numbered to keep the
player track's ledger separate from the census ladder's:

- **INV-P1 PLAN-NOT-TILE.** The solver's public output type is `Plan`; no
  public item in the workspace returns a per-domino scalar value; display
  projections exist only in the trace layer under a type whose name says
  projection. *Enforcement:* `decide` returns `Plan`; a `compile_fail` doctest
  shows the forbidden `tile_value(domino)`-style call does not exist; INV-10's
  vocabulary grep extended to forbid `TileValue`/`DominoScore`-style
  identifiers; review.
- **INV-P2 ONE-INFOSET-ONE-ACTION.** A plan is a map from observation
  sequences to actions; one action per key, keys unique by the map type;
  sibling subtrees are keyed by distinct observations. *Enforcement:* type
  system (ordered map); test `inv_p2_partition` walks every P3 plan asserting
  key uniqueness and bundle disjointness.
- **INV-P3 EXACT-NO-SAMPLING.** The solve path contains no randomness source
  and no division: values are integer *sums* of terminal utilities over world
  bundles, compared only between actions at the same node (identical bundle,
  identical denominator). Ties break to the lowest `DominoId`, recursively, so
  the canonical plan is deterministic. *Enforcement:* no `Rng`-bearing type is
  reachable from the solver module (review + grep); receipts are byte-diffed;
  double-solve determinism test.
- **INV-P4 FIELD-FIXED.** σ is a pure deterministic function of (seat's own
  hand, public state); it is chosen before the solve and immutable during it;
  the same σ is applied to all three hidden seats at every depth.
  *Enforcement:* σ is a function, not a trait object with state; test
  `inv_p4_determinism` replays σ-self-play twice and asserts byte-equal
  traces.
- **INV-P5 BUNDLE-CONSERVATION.** At every internal node of a plan, the child
  bundles partition the node's bundle (each world flows to exactly one child);
  leaf bundle sizes sum to the root fiber cardinality, which equals the
  capacity-DP count (CELL-10) for the root cell system. *Enforcement:*
  `debug_assert!` in the solver; test `inv_p5_conservation` asserts the sum
  and the DP cross-check on every P2 position's solved plan.
- **INV-P6 TAKEOVER-EXACTNESS.** The solver accepts only states with ≤ 4
  tricks remaining and always unrolls to the end of the hand; every leaf's
  utility is a settled hand's exact integer outcome (42-point conservation
  asserted per unroll, reusing the S-obj machinery). *Enforcement:* the solver
  constructor rejects deeper states (typed error); the `Plan` type has no
  non-terminal leaf variant; per-leaf conservation `debug_assert!`.

Standing guardrails continue, one clarification: mk5 is a *design* precedent.
Transcribing anything from mk5 — code, tables, numbers, σ heuristics — is
forbidden; walt's gates are re-derived here from this repo's own objects.

---

## 6. Reading list for the implementer

1. BRIEF.md and BRIEF_SLICE_02.md in full (they govern), then this brief.
2. [wiki/support-fiber](../wiki/support-fiber.md),
   [wiki/capacity-dp](../wiki/capacity-dp.md),
   [wiki/belief-vs-support](../wiki/belief-vs-support.md) (for what P6 is and
   is not), [wiki/verification](../wiki/verification.md) §player.
3. `rob/crates/player` as it stands (the baseline, worlds, rollout, match
   driver, trace) — P1–P5 extend this crate.
4. Math §11.4 (the baseline's contract, unchanged) and Math §7.3 (fiber).

---

## 7. Required API surface (semantic, Rust spelling free)

- **sigma** (`player/src/sigma.rs`): `GreedySigma` — the fixed field policy.
  Definition (deterministic, total, points-blind, by declaration-relative
  trick key): *leading*, play the legal tile with the highest trick key under
  its own led context, tie-break lowest `DominoId`; *following or sloughing*,
  if some legal tile's trick key beats the current best play's key, play the
  lowest such tile by key (then lowest id); otherwise play the legal tile with
  the lowest key (then lowest id). Implemented against the S1 algebra
  (`trick_key`, `beats`); never against pip arithmetic.
- **plan** (`player/src/plan.rs`): `Observation` — the ordered sequence of
  (seat, tile) plays strictly between two consecutive viewer decisions
  (possibly empty when the viewer wins and immediately leads); `Plan` — root
  action plus an ordered map `Observation → Plan`; bundle audit data
  (world-index sets per node) carried **outside** semantic equality in the
  INV-1/INV-2 discipline, for receipts and the inspector.
- **solver** (`player/src/solver.rs`): `solve(state, lens) → Plan` for a
  certified mechanical state with the viewer to act and ≤ 4 tricks remaining
  (INV-P6): enumerate the fiber (`fiber_worlds`), advance every world in
  lockstep — σ at hidden seats, branching at viewer decisions, partitioning
  bundles by observation — and back-induct integer sum-values, argmax with the
  INV-P3 tie-break. Lens-generic over the baseline's existing `UtilityLens`;
  receipts pin `Points` (team trick-point total).
- **rob** (`player/src/rob.rs`): the composite player — baseline decisions
  while > 4 tricks remain; at the viewer's first decision with ≤ 4 tricks
  remaining, `solve` once, then **follow the materialized plan to the end of
  the hand** (each realized observation selects the child; a missing key is a
  panic — it would mean a fiber/σ bookkeeping bug, not a recoverable state).
- **match driver** (extension): heterogeneous seating (team A = rob composite,
  team B = baseline) and mirrored paired deals (same deal, teams swapped),
  deterministic seeds, existing scoring unchanged.
- **trace** (P5): plan-tree emission in the existing trace JSON, typed as
  projections (INV-P1); `rob/inspector/index.html` gains a contingency-book
  view — the plan as an expandable tree, each node showing its action, its
  observation key, and its bundle size. Display only; no new computation in
  the page.

---

## 8. Verification harness — receipts

One new binary `verify_rob` printing `rob player-p<stage> verification: PASS`
lines per §9. This track has no corpus-anchored magic integers to reproduce;
its receipt values are **corpus-shape-forced** (arithmetic consequences of the
frozen S3 corpus — hard assertions) or **rob-frozen** (generator-specific,
frozen on first green run per the BRIEF.md §8 S3 precedent, then byte-diffed
forever). Bounds below justify machine integers: the largest endgame fiber is
12!/(4!4!4!) = **34,650** worlds (no-void, 4 tricks remaining), the largest
per-node value sum is ≤ 42 × 34,650 < 2²¹, and boundary fibers at 3/2/1 tricks
remaining are ≤ **1,680** / **90** / **6**.

### P1 — σ (tests `r_sig_*`)

| Test | Assertion | Numbers | Source |
|---|---|---|---|
| `r_sig_selfplay` | from each of the 108 S3 contracted deals, a full σ-self-play hand (all four seats σ): every play legal, 7 tricks, 42-point conservation | **108** hands; **3,024** plays; 42 × 108 | R-FOLLOW/R-SETTLE; INV-P4 |
| `r_sig_deterministic` | the full σ-self-play trace set replayed twice is byte-identical | — | INV-P4 |
| `r_sig_total` | property test: σ returns a legal tile for arbitrary reachable states (proptest over generated hands) | proptest | INV-P4 |

### P2 — endgame positions + fiber (tests `r_pos_*`)

Position corpus, closed-form: each of the 108 S3 corpus hands truncated at the
trick boundaries after tricks 3, 4, 5, 6 (plays 12/16/20/24), viewer = the
seat to lead the next trick — **432** positions, of which 108 per
tricks-remaining depth 4/3/2/1.

| Test | Assertion | Numbers | Source |
|---|---|---|---|
| `r_pos_corpus` | 432 positions decode; every position's viewer is to act; tricks remaining ∈ {4,3,2,1} in equal counts | **432**; 4 × 108 | corpus-shape |
| `r_pos_fiber` | per position: `fiber_worlds` cardinality equals the capacity-DP count (CELL-10 route); every enumerated world passes `fiber_contains`; no duplicates; size ≤ the closed-form bound for its depth | **432** count agreements; bounds 34,650/1,680/90/6 | CELL-05/10 |
| `r_pos_census` | total worlds across all 432 fibers (rob-frozen on first green) | rob-frozen | CELL-10 |

### P3 — the solver, W0 (tests `r_sol_*`)

The four gates. Gates 2 and 3 run on the **216** tiny positions (boundaries 5
and 6: ≤ 2 tricks remaining), where all of rob's pure plans are explicitly
enumerable; gates 1 and 4 run on all 432.

| Test | Assertion | Numbers | Source |
|---|---|---|---|
| `r_sol_known_world` | **known-world degeneracy**: per position, pin the cell system to the first enumerated world (each P_s = that world's hand ⇒ fiber = 1); `solve` must equal direct perfect-info backward induction against σ — same value, same canonical action | **432** agreements | gate 1 |
| `r_sol_brute_force` | **exactness at tiny depth**: enumerate literally all of rob's pure plans, evaluate each by full unroll over the whole fiber; the solver's value equals the enumerated max and its canonical plan is in the argmax set | **216** positions | gate 2 |
| `r_sol_undominated` | **no pointwise-dominated plan**: on the same 216, the chosen plan is not pointwise-dominated (no enumerated plan weakly better in every world, strictly in one) | **216** | gate 3 |
| `r_sol_conservation` | **bundle conservation** (INV-P5): partitions at every node; leaf bundle sizes sum to the root fiber count = DP count | **432** plans | gate 4 |
| `r_sol_deterministic` | double-solve byte-equal plans on all 432; plan values (rob-frozen totals per depth) | **432**; rob-frozen | INV-P3 |

### P4 — composite + paired match (tests `r_mat_*`)

| Test | Assertion | Numbers | Source |
|---|---|---|---|
| `r_mat_takeover` | on deterministic full-hand self-plays, rob's seat delegates to baseline until ≤ 4 tricks remain, solves exactly once, and follows the plan to the hand's end; every realized observation is a present key; the realized world is in every followed node's bundle | per-hand assertions | INV-P6 |
| `r_mat_paired` | mirrored paired match, 100 deterministic deals × 2 seatings, rob team vs baseline team, `Points` lens: per-seating and net margins printed and rob-frozen on first green | **200** hands; rob-frozen margin | mk5 precedent, re-derived |

The frozen margin is a *measurement*, not a target: if rob does not beat the
baseline, that is a reportable finding, never a reason to tune σ, the corpus,
or the seeds (INV-5 discipline applies to the freeze).

### P5 — contingency book (tests `r_book_*`)

| Test | Assertion | Numbers | Source |
|---|---|---|---|
| `r_book_roundtrip` | plan-tree JSON emission round-trips (parse → re-emit byte-equal) for all 432 P2 plans; every displayed number is typed as a projection | **432** | INV-P1 |

Inspector rendering is reviewed by eye (it is display only); the receipt
covers the emitted data.

### P6 (stretch) — W1 σ-consistency filter (tests `r_w1_*`)

The filter: reconstruct each fiber world's full deal by CELL-07 (hidden hands
now ∪ tiles played per seat), replay the recorded history, and keep the world
iff σ would have produced every recorded hidden-seat play. If the filter
empties the fiber, fall back to W0 unfiltered (the recorded opponents are the
baseline, not σ — emptiness is expected sometimes and is counted, not hidden).

| Test | Assertion | Numbers | Source |
|---|---|---|---|
| `r_w1_filter` | per P2 position: filtered fiber ⊆ fiber; kept/discarded/empty-fallback counts (rob-frozen); on σ-self-play-generated positions the true world always survives | rob-frozen; survival **always** | CELL-07 |
| `r_w1_ablation` | the P4 paired match repeated with rob-W1 vs rob-W0 and rob-W1 vs baseline; margins rob-frozen | rob-frozen | term-1-vs-term-2 measurement |

---

## 9. Receipts and CI

As BRIEF.md §9: `verify_rob` prints its deterministic receipt; committed as
`rob/receipts/verify_rob.txt`; `rob/ci/check.sh` runs it and byte-diffs.
Rob-frozen lines follow the S3/970 precedent: computed on first green, then
asserted exactly forever; weakening forbidden (INV-5). Everything is
minutes-scale: the heaviest stage (P3 over 432 positions, worst fiber 34,650)
is bounded by ≤ 24 viewer action sequences × 34,650 worlds × ≤ 16 plays per
unroll per position — well under CI budget in Rust.

---

## 10. Pre-resolved decisions

1. **σ's exact definition is spec, not taste.** The §7 GreedySigma definition
   is normative; if it admits two readings on some state, that is an
   `ambiguity_sigma_*` finding, not a silent choice. Improving σ is out of
   scope, full stop.
2. **Mid-trick takeover.** The P2/P3 receipt corpus uses trick-boundary
   positions (viewer leads) for closed-form shape; the composite player may
   also first-solve mid-trick (viewer follows in trick 4). The solver handles
   any ≤-4-tricks viewer-to-act state; the P4 takeover assertions exercise the
   mid-trick case naturally.
3. **A losing frozen margin is a finding** (§8 P4). Report it; do not tune.
4. **Fiber blowup.** If any corpus position's fiber exceeds its closed-form
   bound, that is a bug in this brief's arithmetic or in the corpus decode —
   stop and report; the bounds are theorems, not estimates.
5. **Baseline untouched.** No behavior change to the baseline player or its
   receipts anywhere in this track (P4 extends the driver around it).

---

## 11. Ambiguity protocol

Unchanged from BRIEF.md §11.

---

## 12. Definition of done — player track v1

1. **Layout**: additions match §14; core, wiki, ingest, exchange untouched;
   nothing from §4's out-of-scope list.
2. **Green**: `rob/ci/check.sh` end-to-end including `verify_rob` byte-identical
   to `rob/receipts/verify_rob.txt` (P6's lines included iff built).
3. **Every number**: 108 / 3,024; 432 / 4×108; bounds 34,650 / 1,680 / 90 / 6;
   432 fiber-count agreements; 432 known-world agreements; 216 brute-force and
   216 dominance agreements; 432 conservation checks; 200 paired hands; and
   every rob-frozen line frozen and diffed.
4. **Invariants**: INV-P1..P6 each have their named enforcement present and
   green; INV-1..14 enforcement still green.
5. **Independence**: nothing transcribed from mk5; σ and all gates implemented
   from this repo's objects only.
6. **Documentation**: every new public item cites claim IDs where applicable
   (CELL-05/07/10, Math §7.3, §11.4) and states its role in this brief.
7. **Report**: commands run; every count; frozen values with their first-green
   provenance; the paired-match margins with plain-language reading; findings
   (including a losing margin, if that is what happened); any `ambiguity_*`
   tests.

Do not begin census slice 03, a deeper-horizon player, or belief work beyond
P6.

---

## 13. Module map (additions)

```
rob/crates/player/src/
  sigma.rs        GreedySigma — fixed deterministic field policy   [INV-P4]
  plan.rs         Observation, Plan, bundle audit (outside Eq)     [INV-P1/P2/P5]
  solver.rs       exact W0 info-set best response, ≤ 4 tricks,
                  integer sums, canonical tie-break                [INV-P3/P6]
  rob.rs          composite player: baseline opening + endgame
                  takeover + plan following                        [INV-P6]
  consistency.rs  (P6) W1 σ-consistency history filter + fallback  [CELL-07]
  match_driver.rs + heterogeneous seating, mirrored pairs
  trace.rs        + plan-tree projection emission                  [INV-P1]
  bin/verify_rob.rs

rob/receipts/verify_rob.txt
rob/inspector/index.html   + contingency-book view (display only)
```

Implement P1 → P2 → P3 → P4 → P5 (→ P6) in order; a stage's receipt must be
green and committed before the next stage's first line of code.
