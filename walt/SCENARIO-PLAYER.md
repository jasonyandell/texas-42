# SCENARIO-PLAYER.md — the sampling-stack seat, mathematical specification v0.1

**EXPLORATORY tier** — below every evidentiary tier, cited by nothing above
the Ideas tier. This is a *spec written after the build* (built 2026-08-17/18
in `walt/walt-m3-probe/`; specified 2026-08-18): the engineering leapt the
chasm and this document is the first plank of the bridge back. Its purpose is
to state the objects and invariants precisely enough that drift between claim
and code is catchable at review — the standing example being the PiKey defect
of 2026-08-18 (§3.4), where a documented invariant ("the mind's entire
information state") was violated by the code for a day and was caught only by
the parallel port's determinism check. Statements here are definitions and
proof obligations, not established results; the obligations table (§10) says
what each needs to graduate.

Vocabulary discipline: *support ≠ belief*, *feasible ≠ reachable*,
*possible ≠ probable* are typed distinctions throughout. "Necessary outer
profile," never "certificate," where D3 vocabulary is at issue (not used
here). Sampled numbers are ESTIMATES, never receipts; nothing here is a
P-A21 statement.

## 1. Setting

Straight points-and-marks 42, one hand, a fixed declaration `dcl` and a fixed
bid `b` held by team T1 (internal convention: bidder occupies an odd internal
seat; external seatings are rotated into this frame and back — the bridge's
rotation, audited in `arena_results_2026-08-17.txt`). Rules semantics (legal
plays, trick winner, count values, void attribution) are delegated to
`walt-core` (zeb lineage); every bridge decision cross-checks walt-core
against the arena's independent rules engine (~15k decisions, zero
divergences — a conformance receipt, never an axiom; TRUST-01).

Total points in a hand are 42: one per trick (7) plus the count tiles
(5-5 and 6-4 at ten, 5-0, 4-1 and 3-2 at five — 35). The Boolean payoff for the
bidding team at bid `b`: **make ⇔ banked(T1) ≥ b**; equivalently fail ⇔
banked(T0) > 42 − b. (For b = 30: make ⇔ banked_T1 ≥ 30 ⇔ banked_T0 ≤ 12.)

## 2. Public record, information state, and the key reduction

**Def 2.1 (public record).** R = the dated sequence of plays from the hand's
first lead: for each play, (seat, tile), grouped into tricks with leaders
determined by trick winners. R determines: the played mask P(R) ⊆ 28 tiles,
the current-trick prefix, the current leader, the banked totals
(b₁(R), b₀(R)), and the void sets V(R) = (V₀..V₃) where Vₛ is the union of
effective-incidence masks of contexts seat s failed to follow.

**Def 2.2 (information state).** The information state of seat s is
I_s = (h_s, R): own remaining hand plus the public record. *Nothing else.*
No mind at any level of this architecture conditions on any other seat's
tiles — this is the no-strategy-fusion invariant (Obligation O1).

**Def 2.3 (solver key).** The solver keys nodes on the reduced record
κ(R) = (played, leader, current-trick plays, banked_t1, banked_t0) plus an
alive-set id (§5). κ discards the order of completed tricks.

**Lemma 2.4 (key sufficiency — obligation).** Under the Boolean pmake
objective with fixed dcl and b, the value of the continuation game depends on
R only through κ(R) (and the alive set). *Sketch:* legality depends on hands
and the current trick only; future trick resolution depends on remaining
tiles; the payoff threshold depends on banked totals; no rule of straight 42
references completed-trick order. Status: unproved on paper; heavily
exercised (memo hits across transpositions produce consistent exact values).

**Remark 2.5 (what κ does NOT determine).** banked is *not* a function of
(played, leader, plays): different orderings of the same played set can bank
the same points to different teams. banked must therefore appear explicitly
in every key that feeds a banked-sensitive computation. This is §3.4's bug.

## 3. Modeled minds and their caches

**Def 3.1 (π-state).** The decision state of a modeled mind at seat s is
PiKey = (s, h_s, κ(R)) — its chair, its remaining hand, the reduced record
*including banked totals*.

**Def 3.2 (level-0 mind).** A level-0 mind at PiKey draws n₀ worlds from the
*no-void sizes-fiber* of its information state (Def 4.3, the stated
simplification: uniform over unseen-tile assignments consistent with hand
sizes, ignoring V(R)), then best-responds (argmax over its team's objective,
fixed low-index-first evaluation order, strict improvement to replace) to a
**Dice field** (Def 3.5), evaluating with the full solver of §5–6 on its
sampled worlds.

**Def 3.3 (level-k mind, k ≥ 1).** As level-0, with n_k sampled worlds and
field = level-(k−1) minds. The stack bottoms out at Dice. Per-level sample
sizes (n₀, n₁, …) are declared per run.

**Def 3.4 (cache purity invariant).** Every π value is cached under (k,
PiKey) and MUST be a pure function of that key: same key, same tile, on any
execution, any thread interleaving, any call order. This requires the key to
carry everything the computation reads — in particular banked (Remark 2.5).
**Defect record:** from level1.rs's birth through 2026-08-18, PiKey omitted
banked; serial execution masked this as deterministic first-come aliasing
(the first caller's banked context decided the cached policy for all later
contexts sharing the reduced-minus-banked key). The parallel port made the
alias racy, non-determinism surfaced within hours, and the fix (banked in
PiKey) restored purity — after which 1-thread and 18-thread runs are
byte-identical (a receipt on `f5fff91`, not an axiom). The 3×384 arena pool
was completed on the pre-fix binary for internal consistency and is so
labeled; post-fix play is a new baseline.

**Def 3.5 (Dice field / the tickertape).** In a level-0 mind's solver, each
non-viewer seat in world w plays uniformly among its legal tiles, decided by
a deterministic stream: SplitMix64(seed_w ⊕ hash(κ(R))) mod |legal|. Keying
the stream on the *record* (not the path) makes the dice a fixed tickertape:
the same world at the same record plays the same tile in every branch, so
worlds *partition* by drawn move instead of multiplying branches (this
killed the >48 GB union-tree of the pre-08-17 attempts).

**Def 3.6 (seed discipline).** All randomness is derived from frozen
constants (INNER_SEED, OUTER_SEED, BRIDGE_SEED, MINER_SEED) mixed with
structural coordinates (seat, hand, record hash, level tag; level tag = 0
at k = 0 so level-0 minds are seeded bit-identically across level1.rs,
level2.rs, and the bridge). No wall-clock, no global RNG state.

## 4. Beliefs: fibers and samplers

**Def 4.1 (fiber).** fiber(I_s) = the set of assignments of the unseen tiles
(28 ∖ P(R) ∖ h_s) to the other three seats, with the hand sizes forced by R,
such that no seat holds a tile in its void set Vₛ(R). This is the lawful-
completion set of the information state — walt's belief support.

**Def 4.2 (outer sampler).** The outer player samples worlds uniformly from
fiber(I) by shuffle-and-reject: uniform assignment respecting sizes, reject
on void violation. **Lemma (sampler correctness — obligation):** rejection
from the uniform sizes-fiber restricted to the void-consistent subset is
uniform on that subset (standard rejection argument; finite, nonempty —
the true deal is always a member, so the fiber is never empty).

**Def 4.3 (no-void simplification, inner minds).** Modeled minds sample the
sizes-fiber *without* void conditioning. This is a declared approximation
(cheapness inside 3.2/3.3), not an error; its cost is unmeasured
(Obligation O5).

**Remark 4.4 (beliefs are lawfulness-only today).** The outer player
conditions on *voids* observed in R — never on the hypothesis that observed
opponents play any particular policy. Behavior-Bayes (§5) happens only on
*modeled* continuations inside the tree, not on the observed prefix. The
"beliefs chapter" (declaration-aware and policy-consistent posteriors over
the observed prefix) is future work, deliberately out of scope here.

## 5. The alive-set partition (behavior-refinement Bayes)

At a field node (a non-viewer seat inside a solver), each alive world w
yields a move m(w): the tickertape draw (Dice) or π(field-level, PiKey(w))
(policy field). Worlds partition into buckets by move; the node's value is
Σ_m (|bucket_m| / |alive|) · value(child_m with alive = bucket_m).

**Lemma 5.1 (conservation).** The buckets partition the alive set exactly
(asserted at every node in every run).

**Lemma 5.2 (posterior semantics — obligation).** With uniform prior on the
sampled worlds, the child weight is the posterior probability of bucket_m
given that the modeled seat played m; recursively, the value computed at the
root is the expectation of the viewer's best-response payoff under the
model "field seats play their modeled policies," with beliefs updated by
observation of modeled play. Deterministic modeled policies make this a
hard partition (worlds inconsistent with an observed modeled move are
excluded *within the model* — intended Bayes, not a defect).

**Lemma 5.3 (support safety at the bottom).** Under Dice, every legal move
of every world has positive probability, so no lawful world is excluded by
the dice field: the level-0 bottom is support-safe. (Deterministic higher
levels refine support intentionally per 5.2; the bottom's agnosticism is
what guarantees the refinement chain starts from full support. This is also
the principled answer to "why not a stronger base than uniform": any
opinionated belief-free base can zero lawful worlds; dice cannot.)

## 6. The outer player and the objective

**Def 6.1 (level-k walt).** The seat's actual decision procedure at
information state I: draw n_outer worlds from fiber(I) (void-conditioned,
Def 4.2), and for each legal candidate tile, compute the exact value of the
child under the solver with field = level-(k−1) minds; play the argmax for
the seat's team objective. Level-1 (field = level-0) is the match champion;
level-2 (field = level-1) is the first level whose modeled partner
coordinates back.

**Def 6.2 (objective and cutoffs).** The value is P(make) under the model:
BigRational, exact on the sample. Decided cutoffs — value 1 when
banked_T1 ≥ b, value 0 when banked_T0 > 42 − b — are sound for the Boolean
objective by count conservation. The bid level b parameterizes the same
solver (b = 30 throughout the current artifacts); this parameterization is
the substrate of baseline bidding (planned; not yet built).

**Def 6.3 (tie protocol).** A saturation tie (several candidates equal at
the top, typically at 1-on-sample) is never broken by tile index: tied
candidates are re-evaluated on fresh, larger samples (4× per round, bounded)
until separated or the bound is hit. Rationale: support ≠ belief — 1-on-
sample is not certainty, and index-breaking injects an arbitrary preference
exactly where the estimate is least informative. (The level-2 t=1 episode:
a three-way 100% tie at n=200 whose refinement at n=800 separated 5-5
uniquely; an index-break would have led 1-1.)

## 7. Estimator semantics — what walt's numbers mean

The root value is a plug-in estimator: exact best-response value *on the
sampled worlds* against the declared field model. Three fences:

1. **Exact-on-sample, estimate-off-sample.** The rational number is exact
   for the sample; as an estimate of the fiber-wide value it carries
   unquantified sampling error (no CIs are currently computed —
   Obligation O6).
2. **Model-relative, not game-theoretic.** The n→∞ limit is the best
   response *to the field model* on the true fiber — not an equilibrium
   strategy, and not the exact info-set best response against any actual
   opponent. Raising k changes the model, not the game. Walt's strength
   claims are empirical (arena receipts), never derived from these values.
3. **Level mismatch in the mirror.** Inside a level-k walt's model, the
   modeled minds model *their* others at level k−2 — in particular a
   level-2 walt's modeled partner reads walt itself as level-0. Signaling
   value discovered at level 2 is signaling into a simplified reader
   (lower bound on matched-reader coordination; divergence-mining caveat).

## 8. Parallel execution (level2.rs and successors)

**Theorem-shaped claim 8.1 (execution-order invariance — obligation).**
Given (i) cache purity (3.4), (ii) exact rational arithmetic (sums are
order-invariant), (iii) fixed argmax evaluation order at every
decision, and (iv) a partition (not race) semantics for memo/alive-set
identity, the solver's results are invariant under any thread count and
interleaving; only work statistics (node counts, duplicate π computations
racing the same key) may vary. Evidence: byte-identical result lines,
1 vs 18 threads and repeated runs (receipt on `f5fff91`). Duplicate work on
transposition races is wasted, not wrong (same pure value re-derived).

## 9. Current implementations bound by this spec

`walt/walt-m3-probe/src/bin/`: `level1.rs` (fixed-carrier ladder, level-1),
`level2.rs` (fixed-carrier ladder, level-k parameterized, parallel),
`playtable.rs` / `webtable.rs` (interactive tables, level-1),
`walt_bridge.rs` (mk5 arena seat, level-1, rotation frame §1),
`divergence.rs` (self-play miner, level-1 trajectory + level-2 shadow).
All post-`1fc2319` binaries carry the banked-correct PiKey. Frozen match
and probe results: `arena_results_2026-08-17.txt`,
`level2_results_2026-08-17.txt`, `divergence_results_2026-08-18.txt`,
corpus under `mined/`.

## 10. Obligations ledger (the bridge still to build)

| # | Obligation | Kind | Route |
|---|---|---|---|
| O1 | No-strategy-fusion: no computation reads hidden tiles outside its declared world | invariant audit | code audit + (eventually) Lean-shaped statement over the solver's data flow |
| O2 | Key sufficiency (Lemma 2.4) | proof | paper proof; candidate for exchange review |
| O3 | Sampler correctness (Lemma 4.2) | proof | short paper proof |
| O4 | Posterior semantics (Lemma 5.2) | proof | paper proof; the load-bearing one |
| O5 | Cost of the no-void inner simplification | measurement | ablation probe (void-conditioned inner minds vs current) |
| O6 | Sampling error quantification on root values | design + math | per-world make indicators admit binomial-style intervals; correlation caveats to state |
| O7 | Execution-order invariance (8.1) | proof | from O2/purity + rational-arithmetic lemmas |
| O8 | Tie-refinement bias (does conditional re-sampling bias the reported top value?) | analysis | small-sample analysis; label reported values accordingly |
| O9 | Bid-level generalization correctness (6.2 cutoffs for all b) | proof + tests | trivial by conservation; assert in CI when built |

Filed 2026-08-18 from `math/signed_pivotal_geometry_v0.1.md` §14 (intake:
`math/signed_pivotal_geometry_v0.1_intake.md`; O10–O11 are permanently
retired — a numbering artifact of the side-channel hops, ruled SP-A11 in
`CENSUS-RULINGS.md`; retired numbers are never reused):

| # | Obligation | Kind | Route |
|---|---|---|---|
| O12 | Frozen-plan typing: every pivotal estimate names its exact frozen plan pair; root-action claims state whether active-plan stability was checked | discipline | enforce in E0 tooling and reports |
| O13 | Discovery/evaluation separation: fixed-plan confidence claims use scenarios not used to construct or alter those plans | discipline | sample splitting in E0; any replacement argument must be explicit |
| O14 | Sequential validity: resample-until-separated protocols use anytime-valid methods or predeclared checkpoint/alpha-spending schedules | discipline + math | applies to E0 racing and to bidcurve θ sweeps |
| O15 | Scenario/world domain match: every exact structural count is over the same fiber and belief the conditional sampler targets; tape integration explicit | invariant audit | E0 Phase E; world/tape seed separation prerequisite |
| O16 | Envelope containment: pivotal-only sampling requires proved containment; otherwise the complement keeps a sampling floor or rigorous bound | proof per envelope | E0 Phase F promotes no predicate without it |
| O17 | Conditional generator correctness and cost: uniformity and wall-clock tested separately from exact weights | measurement | E0 Phase F audit columns |
| O18 | Optimization-lock accounting: exact frozen-pair results are never labeled exact root-action results until competing continuation bounds are closed | discipline | tier labels in every E0 report |
| O19 | Behavioral census: plan simplicity claims use behavioral and signed-boundary equivalence, never syntactic plan counts alone | discipline | shared with POLICY-GEOMETRY.md Gate E (E-A8) |

Filed 2026-08-24 from `math/calculated_evidence_v0.1.md` §21 (intake:
`math/calculated_evidence_v0.1_intake.md`; accepted at CE-A4 in
`CENSUS-RULINGS.md`):

| # | Obligation | Kind | Route |
|---|---|---|---|
| O20 | Exact evidence theorem: CE-T1..T5 implementations match the stated exact formulas and anytime-valid hypotheses | math + tests | paper proof in the parent; exact-rational intake verification (done 2026-08-24); implementation tests |
| O21 | Risk-ledger completeness: every probabilistic settlement reconstructs its full allocation (run → decision → epoch → pair direction → equivalence tests); total allocated risk bounded by the declared scope budget | invariant audit | exact-rational ledger tests |
| O22 | Frozen-policy identity: every evidence observation names immutable PolicyIds; evaluation cannot mutate, re-discover, or world-condition focal actions while retaining old evidence | discipline + tests | data-flow audit, cache immutability tests, adversarial hidden-world tests |
| O23 | Canonical fiber and sampler domain: evidence stream, exact count, and exhaustive endpoint target the same information-state fiber and belief; the unified kernel is the authority | proof + tests | construction proof; count/sample/enumeration parity tests; domain assertions |
| O24 | Exact-escalation correctness: switching to enumeration at any stream index equals cold full enumeration; sampled multiplicities never double-counted | proof + tests | V9 plus a short bookkeeping proof |
| O25 | Result typing and fallback separation: the six result kinds are mechanically distinct; no UI or bridge erases the type before persistence | discipline + tests | API and serialization tests |
| O26 | Execution-order invariance of evidence: for a fixed world stream and frozen candidate set, batching, scheduling, and pause/resume change nothing | proof + tests | indexed world stream plus V8 |
| O27 | Sampling randomness semantics: the probability space behind every δ claim is explicit — sampler uniformity, replacement, seed provenance, PRNG assumption declared | design note | plus O3 integration |
| O28 | Recursive inner-risk accounting: adaptive inner-mind decisions draw risk from a complete nested ledger; outer δ claims never silently ignore inner stochastic error | design + audit | Phase-2 design and cache audit |

Graduation path per project law: paper proofs → wiki with tier labels →
independent re-verification (exchange batch on Jason's authorization) →
Lean for what earns mechanization. Nothing in this document is promoted by
its own existence.
