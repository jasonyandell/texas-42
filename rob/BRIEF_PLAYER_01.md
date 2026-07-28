# rob — Player Brief 01: the whole-hand plan solver

This is the definitive first assignment for **rob the player** — the imperfect-
information player this repository was created to make possible. It opens a
**player track** (stages P1…) parallel to the census/kernel slice ladder;
[BRIEF.md](BRIEF.md) §1 governing synthesis, §2 toolchain, §5 invariants
INV-1..10, §9 receipt/CI rules, and §11 ambiguity protocol, together with
[BRIEF_SLICE_02.md](BRIEF_SLICE_02.md) §5 INV-11..14 and §9.1 tier labeling,
**remain binding verbatim**. Slices 01 and 02 are green and committed; this track
begins only from that state and consumes only their public surface.

Scope, in one sentence: **rob v1 — an exact information-set best-response plan
solver over the exact fiber, playing every decision from the first trick under
a budgeted exact window (full depth to the end of the hand whenever the budget
allows, which is guaranteed from three completed tricks on), against a fixed
deterministic field policy σ, under the uniform (W0) weighting, with a typed
non-tunable frontier leaf.**

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
  updated to say "baseline"; its receipt text does not change. The baseline
  plays **no part in rob's runtime** — rob plays every decision himself.
- **PLAN-NOT-TILE (the hand-as-unit law, binding).** The decision variable
  ranges over whole-hand contingent plans ρ — one action per viewer information
  set within the exact window — and the played tile is the plan's first move.
  No API anywhere in the workspace returns a scalar "value of a domino" (this
  sharpens BRIEF.md §5's HAND-07 guardrail from a modeling statement into an
  API prohibition). Any per-tile number shown to a human is a typed projection
  of a plan, produced only by the trace/display layer.
- **Fiber = information set.** rob's knowledge object at a decision is exactly
  the fiber Φ(C) of the derived cell system (CELL-05/07). No sampling, no
  approximation, anywhere in the solve path. The count-ratio sampler is not
  used by this track. Large fibers are **streamed** by rank/unrank
  (CELL-25/26), never materialized as vectors.
- **σ is fixed, deterministic, and humble.** The other three seats — including
  the partner — are modeled by one fixed deterministic policy σ (§7). σ's
  determinism is the compression that makes the solve exact and small; σ's
  quality is explicitly *not* a v1 goal.
- **W0 before W1 (term 2 before term 1).** The uniform weighting over the fiber
  ships first and is the v1 deliverable; the σ-consistency history filter (W1)
  is the stretch stage P6, with its ablation. Precedent: mk5's graded finding
  that belief tilt is only cashable through honest info-set continuation.
- **Whole-hand play under a budgeted exact window (decided 2026-07-28,
  superseding the endgame-takeover design of this brief's first commit).**
  rob solves at **every** decision from trick 1. The exact window depth `H` at
  a decision is the largest depth whose conservative work product (§7) fits
  the normative budget `B = 2³²`, capped at full depth. Consequences, stated
  as facts of the budget arithmetic (§8 bounds): at trick 1, H = 1; at trick
  2, H ≈ 3; from the third trick on, **full depth to the end of the hand is
  always affordable** — so the truncated regime is exactly rob's first two
  decisions. Within the window the solve is exact; at the frontier the leaf
  is banked points and nothing else (INV-P7). **Eyes wide open:** early plays
  may be window-greedy (spending control to cash count the leaf can see);
  that is a characterized, measured behavior (§8 P4 ablation), never a bug —
  and never a license to enrich the leaf.
- **Registered rent (from mk5's walt, honestly carried).** (a) Field-model
  rent: rob's opponents in the paired match are the *baseline*, not σ — so v1
  measures a best response to a wrong model against a real opponent, which is
  the honest experiment. (b) Partner rent: rob models his partner as field even
  though the partner seat is also rob; seats never communicate (legal play,
  same rule as humans). (c) Window rent: tricks 1–2, as above.

Authority order: this brief; then BRIEF.md + BRIEF_SLICE_02.md; then `wiki/`;
then ingest; then exchange results (tier-labeled). mk5 ranks below all of them
and is precedent only.

---

## 2. Language and toolchain (unchanged)

As BRIEF.md §2 in full. No new dependencies in any crate. No floats (INV-4);
all solver arithmetic is machine-integer with overflow checks — §8 shows the
bounds fit comfortably in `u64`/`i64`. `rob/crates/core` is **read-only** to
this track: the needed surface (`derive_rule_cells`, `fiber_worlds`,
`fiber_contains`, `rank_world`/`unrank_world`, the exact counting routes, the
mechanical machine, the declaration algebra) already exists. If a core
addition appears necessary, that is a finding under the ambiguity protocol,
not a local edit.

---

## 3. Project layout (additions only)

All additions live in `rob/crates/player` and `rob/crates/player/src/bin`, plus
one receipt file and the inspector page (§13 module map). `wiki/`, `ingest/`,
`exchange/`, and `rob/crates/core` are untouched.

---

## 4. Scope: player track v1 and its boundaries

Five gated stages plus one stretch stage. A stage begins only when the previous
stage's receipts are green and committed.

- **P1 — σ, the field policy**: `GreedySigma`, deterministic, total, legal;
  σ-self-play receipts over the 108-hand corpus deals.
- **P2 — position corpus + fiber accounting**: 756 trick-boundary positions
  from the S3 corpus (all seven boundaries 0..6); exact fiber enumeration
  cross-checked against the capacity-DP count where enumeration fits;
  count-only receipts where it does not.
- **P3 — the solver (W0), the Plan type, and the window schedule**: exact
  streaming backward induction over the info-set tree; the correctness gates.
- **P4 — rob at the table + paired match**: rolling re-solve at every
  decision from trick 1; schedule assertions; mirrored paired match vs the
  baseline; frozen margin; window ablation.
- **P5 — the contingency book**: Plan-tree trace emission and inspector view.
- **P6 (stretch, optional)** — the W1 σ-consistency history filter with
  empty-filter fallback and the W0-vs-W1 ablation match.

**Explicitly out of scope** (do not begin, do not scaffold speculative APIs
for):

- **Any leaf beyond banked points** — no count-in-hand terms, no trump-control
  terms, no tunable weights of any kind (INV-P7). A richer frontier is a
  future brief with its own ablation obligations.
- **Belief distributions** beyond P6's σ-consistency filter — no Bayes over
  augmented worlds, no posteriors, nothing from census slice 04; the 90-world
  posterior flip stays untouched.
- **Census slices 03–05** (folded trick / reduced viewer kernel, belief layer,
  symbolic-DAG census) — this track neither begins them nor depends on them.
- **NF/equivalence pooling of worlds inside the solve** (the "identical within
  the window" optimization that would deepen early windows) — registered as a
  later-brief idea; v1 streams the fiber plainly.
- **Bidding** — the placeholder auction stands; rob v1 is a play-phase player.
- Opponent modeling beyond the fixed σ; σ tuning; any learning; optimization
  caches before correctness — never.

---

## 5. Named invariants

INV-1..14 inherited unchanged. Seven new invariants, P-numbered to keep the
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
  system (ordered map); test `inv_p2_partition` walks every materialized P3
  plan asserting key uniqueness and bundle disjointness.
- **INV-P3 EXACT-NO-SAMPLING.** The solve path contains no randomness source
  and no division: values are integer *sums* of frontier utilities over world
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
- **INV-P5 BUNDLE-CONSERVATION.** At every internal node of a solve, the child
  bundles partition the node's bundle (each world flows to exactly one child);
  frontier bundle sizes sum to the root fiber cardinality, which equals the
  capacity-DP count (CELL-10) for the root cell system. Holds identically for
  the streaming solver (accumulator world-counts) and any materialized tree.
  *Enforcement:* accumulator totals asserted against the DP count in the
  solver; test `inv_p5_conservation` on every P2 position's solve.
- **INV-P6 WINDOW-EXACTNESS.** Within its window the solve is exact: every
  world in the fiber is visited (streamed, never sampled), σ is applied
  exactly, and when the window reaches the end of the hand every frontier
  utility is a settled hand's exact integer outcome (42-point conservation
  asserted per unroll). The window depth is the §7 budget formula's output —
  never a free parameter per call site. *Enforcement:* the solver computes
  `H` internally from the normative formula; callers cannot pass a depth;
  per-settled-leaf conservation `debug_assert!`; schedule receipts (§8 P3/P4).
- **INV-P7 FRONTIER-LEAF-IS-LAW.** The frontier leaf is the banked team
  points of the interrupted hand — one typed variant, no parameters, no
  tunable terms, no alternative leaf reachable from the solver. Window-greedy
  early play is a measured behavior, never a reason to enrich the leaf.
  *Enforcement:* the leaf is a unit-variant enum with a single constructor;
  INV-10-style grep forbids `LeafWeight`/`Heuristic`-style identifiers in the
  player crate; the P4 ablation receipt is the only sanctioned response to
  window pathology.

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
4. Math §11.4 (the baseline's contract, unchanged), Math §7.3 (fiber), and
   the CELL-25/26 rank/unrank surface (S4).

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
- **window** (`player/src/window.rs`): the normative budget formula.
  Work estimate for depth `h` at a decision: `fiber_count × Π_{i<h} b_i`
  where `b_i = max(1, viewer_hand_size − i)` (a conservative upper bound on
  viewer branching, deterministic and cheap). `H = ` the largest `h ≤ tricks
  remaining` with estimate `≤ B` (always ≥ 1; `fiber_count` from the
  capacity DP, CELL-10). `B` is a constant of this brief, changed only by
  amendment. **Amended 2026-07-28: `B = 2²⁸`** (was `2³²`: the original
  priced a work unit at ~1 ns; a real streamed world-segment costs
  ~50–100 ns, so `2³²`-scale windows are hours, not minutes — the amendment
  keeps every streamed solve seconds-scale and trades no exactness).
- **engine rule** (amended 2026-07-28, same commit): window 1 solves take
  the **response-class counting engine** — exact values by capacity-DP
  counts over σ-response classes ("the seat holds `r` and avoids `E`"),
  milliseconds at any fiber size, including the full 399,072,960-world
  trick-one fiber; windows ≥ 2 take the streaming engine (estimate ≤ `B`
  guaranteed by the formula). The two engines must agree plan-for-plan
  wherever both can run — receipt `r_sol_engines`.
- **plan** (`player/src/plan.rs`): `Observation` — the ordered sequence of
  (seat, tile) plays strictly between two consecutive viewer decisions
  (possibly empty when the viewer wins and immediately leads); `Plan` — root
  action plus an ordered map `Observation → Plan`, frontier variant carrying
  only the typed leaf (INV-P7); bundle audit data (world counts per node;
  world-index sets where materialized) carried **outside** semantic equality
  in the INV-1/INV-2 discipline, for receipts and the inspector.
- **solver** (`player/src/solver.rs`): `solve(state, lens) → Plan` for a
  certified mechanical state with the viewer to act: compute `H` (INV-P6),
  then exact backward induction by **streaming**: worlds visited via
  `unrank_world` (never a materialized fiber vector), each advanced in
  lockstep — σ at hidden seats, branching at viewer decisions — with integer
  sum-accumulators keyed by (viewer action sequence, observation prefix);
  the accumulator count is bounded by the smaller of the fiber count and the
  revealed-arrangement count, which is what makes trick-1 solves fit in
  memory. Argmax with the INV-P3 tie-break; returns the materialized window
  plan when its node count fits the P5 cap, else the plan with depth-truncated
  materialization (values exact either way). Lens: `Points` always;
  `ContractSuccess` permitted **only** when the window reaches the end of the
  hand (it is undefined at a truncated frontier — typed error otherwise).
  Receipts pin `Points`.
- **rob** (`player/src/rob.rs`): the player — at **every** viewer decision
  from trick 1, `solve` and play the root action (rolling re-solve; the
  materialized plan is the inspectable object, the re-solve is the runtime
  discipline). No baseline delegation anywhere.
- **match driver** (extension): heterogeneous seating (team A = rob, team B =
  baseline) and mirrored paired deals (same deal, teams swapped),
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
frozen S3 corpus — hard assertions), **closed-form bounds** (theorems), or
**rob-frozen** (generator-specific, frozen on first green run per the BRIEF.md
§8 S3 precedent, then byte-diffed forever).

The governing bounds, closed-form (no-void maxima; voids only shrink them):
fiber at trick boundary `t` completed tricks ≤ `(21−3t)!/((7−t)!)³` =
**399,072,960 / 17,153,136 / 756,756 / 34,650 / 1,680 / 90 / 6** for
t = 0..6. Largest per-node value sum ≤ 42 × 399,072,960 < 2³⁵ (`u64`/`i64`
comfortable). Budget consequences (the §7 formula at `B = 2²⁸`, amended):
H = **1** at t = 0 (budget-floored; counting engine), H **∈ {1, 2, 3}** at
t = 1 by that position's exact fiber count (distribution frozen in the
receipt), **full depth for every t ≥ 2** (5! × 756,756 ≈ 9.1×10⁷ ≤ 2²⁸ and
shrinking). These are receipt assertions, not commentary.

### P1 — σ (tests `r_sig_*`)

| Test | Assertion | Numbers | Source |
|---|---|---|---|
| `r_sig_selfplay` | from each of the 108 S3 contracted deals, a full σ-self-play hand (all four seats σ): every play legal, 7 tricks, 42-point conservation | **108** hands; **3,024** plays; 42 × 108 | R-FOLLOW/R-SETTLE; INV-P4 |
| `r_sig_deterministic` | the full σ-self-play trace set replayed twice is byte-identical | — | INV-P4 |
| `r_sig_total` | property test: σ returns a legal tile for arbitrary reachable states (proptest over generated hands) | proptest | INV-P4 |

### P2 — position corpus + fiber accounting (tests `r_pos_*`)

Position corpus, closed-form: each of the 108 S3 corpus hands truncated at
**every** trick boundary t = 0..6 (plays 0/4/8/12/16/20/24), viewer = the seat
to lead the next trick — **756** positions, 108 per depth.

| Test | Assertion | Numbers | Source |
|---|---|---|---|
| `r_pos_corpus` | 756 positions decode; every position's viewer is to act; tricks remaining ∈ {7..1} in equal counts | **756**; 7 × 108 | corpus-shape |
| `r_pos_count` | per position: capacity-DP fiber count ≤ its closed-form bound; at t = 0 the count **equals** 399,072,960 for all 108 (no information yet removes a world) | **756** bound checks; **108** equalities | CELL-10; Math §7 |
| `r_pos_fiber` | for the 432 positions with t ≥ 3: `fiber_worlds` cardinality equals the DP count; every enumerated world passes `fiber_contains`; no duplicates. For t ≤ 2, streaming visit via `rank_world`/`unrank_world` round-trips on a deterministic index sample and total visited count equals the DP count | **432** enumerated agreements; **324** streamed agreements | CELL-05/10/25/26 |
| `r_pos_schedule` | per position: the §7 window formula's H equals the value forced by the bounds table (1 at t = 0; 1–3 at t = 1, histogram rob-frozen; full depth for every t ≥ 2) | **756** | INV-P6; §7 amendment |

### P3 — the solver, W0 (tests `r_sol_*`)

Gates 2 and 3 run on the **216** tiny positions (t ∈ {5,6}: ≤ 2 tricks
remaining), where all of rob's pure plans are explicitly enumerable; gates 1
and 4 run on all **756** (gate 1 at each position's own H and leaf).

| Test | Assertion | Numbers | Source |
|---|---|---|---|
| `r_sol_known_world` | **known-world degeneracy**: per position, pin the cell system to one world (each P_s = that world's hand ⇒ fiber = 1); `solve` must equal direct perfect-info backward induction against σ *to the same window H with the same frontier leaf* — same value, same canonical action | **756** agreements | gate 1 |
| `r_sol_brute_force` | **exactness at tiny depth**: enumerate literally all of rob's pure plans, evaluate each by full unroll over the whole fiber; the solver's value equals the enumerated max and its canonical plan is in the argmax set | **216** positions | gate 2 |
| `r_sol_undominated` | **no pointwise-dominated plan**: on the same 216, the chosen plan is not pointwise-dominated (no enumerated plan weakly better in every world, strictly in one) | **216** | gate 3 |
| `r_sol_conservation` | **bundle conservation** (INV-P5): accumulator/bundle partitions at every node; frontier totals sum to the root fiber count = DP count — including the 108 trick-1 solves over the full 399,072,960-world fiber | **756** solves | gate 4 |
| `r_sol_deterministic` | double-solve byte-equal canonical plans on all 756; root values rob-frozen per depth | **756**; rob-frozen | INV-P3 |
| `r_sol_engines` | the counting and streaming engines produce identical plans (values, actions, bundles) at window 1 on every position whose fiber is enumerable (t ≥ 2) | **540** agreements | §7 engine rule |

### P4 — rob at the table + paired match (tests `r_mat_*`)

| Test | Assertion | Numbers | Source |
|---|---|---|---|
| `r_mat_rolling` | on deterministic full-hand self-plays, rob solves at every one of his decisions from trick 1 (including mid-trick), plays each root action, and the per-decision H matches the §7 formula; at full-depth decisions the realized world is in every followed node's bundle | per-hand assertions | INV-P6 |
| `r_mat_paired` | mirrored paired match, 100 deterministic deals × 2 seatings, rob team vs baseline team, `Points` lens: per-seating and net margins printed and rob-frozen on first green | **200** hands; rob-frozen margin | mk5 precedent, re-derived |
| `r_mat_window_ablation` | the same paired match with rob's budget artificially halved and doubled (B/2, 2B — window schedule shifts at the margins): margins rob-frozen. This receipt *prices the window*; it is the sanctioned response to early-game weirdness (INV-P7) | rob-frozen ×2 | window rent, measured |

The frozen margin is a *measurement*, not a target: if rob does not beat the
baseline, that is a reportable finding, never a reason to tune σ, the leaf,
the corpus, or the seeds (INV-5 discipline applies to the freeze).

### P5 — contingency book (tests `r_book_*`)

| Test | Assertion | Numbers | Source |
|---|---|---|---|
| `r_book_roundtrip` | plan-tree JSON emission round-trips (parse → re-emit byte-equal) for all 756 P2 plans (depth-truncated materializations included, marked as such in the JSON); every displayed number is typed as a projection | **756** | INV-P1 |

Materialization cap: a plan is fully materialized when its node count is
≤ 2²⁰; above the cap, materialization truncates by depth with an explicit
`truncated` marker (values remain exact — the cap affects display data only).
Inspector rendering is reviewed by eye (display only); the receipt covers the
emitted data.

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
asserted exactly forever; weakening forbidden (INV-5). Cost (measured
2026-07-28): the full P1–P3 receipt runs ≈ 8 minutes wall on a parallel
sweep (std threads, deterministic collection order) — dominated by the 108
boundary-2 full-depth streaming solves; trick-one decisions are
milliseconds via the counting engine. If wall-clock exceeds CI budget,
splitting `verify_rob` receipts across binaries is permitted (receipt text
unchanged); reducing coverage is not.

---

## 10. Pre-resolved decisions

1. **σ's exact definition is spec, not taste.** The §7 GreedySigma definition
   is normative; if it admits two readings on some state, that is an
   `ambiguity_sigma_*` finding, not a silent choice. Improving σ is out of
   scope, full stop.
2. **The budget formula is normative** (§7): the branching estimator is the
   stated conservative product, `B = 2³²`, no per-call overrides (INV-P6).
   Changing `B` or the estimator is an amendment to this brief, not a code
   decision.
3. **A losing frozen margin is a finding** (§8 P4). Report it; do not tune.
4. **Early-game weirdness is a finding, not a bug.** Window-greedy plays in
   tricks 1–2 are expected; they are characterized by the ablation receipt
   and reported plainly. The only sanctioned responses are (a) the future
   pooling brief that deepens early windows and (b) a future leaf brief with
   its own ablation obligations — never in-place tuning (INV-P7).
5. **Fiber blowup.** If any corpus position's DP count exceeds its closed-form
   bound, that is a bug in this brief's arithmetic or in the corpus decode —
   stop and report; the bounds are theorems, not estimates.
6. **Baseline untouched.** No behavior change to the baseline player or its
   receipts anywhere in this track (P4 extends the driver around it).
7. **Lens restriction.** `ContractSuccess` at a truncated frontier is a typed
   error, never a silent reinterpretation (§7 solver).

---

## 11. Ambiguity protocol

Unchanged from BRIEF.md §11.

---

## 12. Definition of done — player track v1

1. **Layout**: additions match §13; core, wiki, ingest, exchange untouched;
   nothing from §4's out-of-scope list.
2. **Green**: `rob/ci/check.sh` end-to-end including `verify_rob` byte-identical
   to `rob/receipts/verify_rob.txt` (P6's lines included iff built).
3. **Every number**: 108 / 3,024; 756 = 7 × 108; the bound septuple
   399,072,960 / 17,153,136 / 756,756 / 34,650 / 1,680 / 90 / 6; 108 trick-1
   count equalities; 432 enumerated + 324 streamed fiber agreements; 756
   schedule checks with H = 1 / ≥3 / full as forced; 756 known-world
   agreements; 216 brute-force and 216 dominance agreements; 756 conservation
   checks; 200 paired hands; the two ablation margins; and every rob-frozen
   line frozen and diffed.
4. **Invariants**: INV-P1..P7 each have their named enforcement present and
   green; INV-1..14 enforcement still green.
5. **Independence**: nothing transcribed from mk5; σ and all gates implemented
   from this repo's objects only.
6. **Documentation**: every new public item cites claim IDs where applicable
   (CELL-05/07/10/25/26, Math §7.3, §11.4) and states its role in this brief.
7. **Report**: commands run; every count; frozen values with their first-green
   provenance; the paired-match and ablation margins with plain-language
   reading; findings (including a losing margin or ugly early plays, if that
   is what happened); any `ambiguity_*` tests.

Do not begin census slice 03, a richer leaf, world pooling, or belief work
beyond P6.

---

## 13. Module map (additions)

```
rob/crates/player/src/
  sigma.rs        GreedySigma — fixed deterministic field policy   [INV-P4]
  window.rs       normative budget formula H(fiber_count, hand)    [INV-P6]
  plan.rs         Observation, Plan, typed frontier leaf, bundle
                  audit (outside Eq)                               [INV-P1/P2/P5/P7]
  solver.rs       exact W0 streaming info-set best response,
                  integer sums, canonical tie-break                [INV-P3/P6]
  rob.rs          the player: rolling re-solve from trick 1        [INV-P6]
  consistency.rs  (P6) W1 σ-consistency history filter + fallback  [CELL-07]
  match_driver.rs + heterogeneous seating, mirrored pairs
  trace.rs        + plan-tree projection emission                  [INV-P1]
  bin/verify_rob.rs

rob/receipts/verify_rob.txt
rob/inspector/index.html   + contingency-book view (display only)
```

Implement P1 → P2 → P3 → P4 → P5 (→ P6) in order; a stage's receipt must be
green and committed before the next stage's first line of code.
