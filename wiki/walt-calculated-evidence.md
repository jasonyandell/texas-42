# walt — the calculated-evidence era

[Home](Home.md) · owns: the calculated-evidence program (2026-08-24 onward) —
anytime-valid adaptive settlement as walt's new correctness path, the §22 build
(steps 2–8, with commits), the step-7 shadow instrument and its records, the
step-8 V5 flip repair and E0 calibration, the live-player audit findings, the
targeted level-2 extension with both field-swap slices (the fixed-policy smoke
and the rung/screen slice), the sampling-cap analysis and Jason's 512 ruling as
applied, and what is in flight · Sources:
`walt/math/calculated_evidence_v0.1.md` (received parent,
adjudicated **CE-A1..A8**) and `walt/math/targeted_level2_field_stability_v0.1.md`
(received parent, adjudicated **L2-A1..A7**) with their intake companions,
`walt/CENSUS-RULINGS.md` (the two adjudication chapters),
[`walt/probes/shadow/README.md`](../walt/probes/shadow/README.md),
[`walt/probes/fieldswap/README.md`](../walt/probes/fieldswap/README.md),
[`walt/probes/step8/README.md`](../walt/probes/step8/README.md) and
[`walt/probes/fieldswap_screen/README.md`](../walt/probes/fieldswap_screen/README.md)
(instrument records), `walt/SCENARIO-PLAYER.md` §10 (the obligations ledger),
`kanban/` cards. Related: [walt](walt.md) (hub and fence),
[walt-seat-play](walt-seat-play.md) (the live track this era instruments),
[received artifacts and intakes](walt-math-intakes.md) (artifact-by-artifact
lineage — not restated here), [the reference map](walt-math-reference.md)
(ruling families CE-A and L2-A),
[open questions](walt-math-open-questions.md).

> **Epistemic tier: EXPLORATORY — the hub's fence applies unchanged, and most
> of this page sits *below* even that.** The two adjudications (CE-A1..A8,
> L2-A1..A7) are the project's **own exploratory-fence rulings** — one
> adjudicator, no adversary panel, never CONFIRMED-tier, never promotable by
> being cited. Every count quoted from `walt/probes/shadow/`,
> `walt/probes/fieldswap/`, `walt/probes/step8/` or
> `walt/probes/fieldswap_screen/` is an **instrument record** — probe output
> that sits below every evidentiary tier and is cited by nothing above it; the
> READMEs' own fences and caveats travel with every number here. Nothing on
> this page is a receipt, a strength claim, or a statement about exact values.

## What the program is

The scenario-player seat ([walt-seat-play](walt-seat-play.md)) sampled a
**fixed magic number of worlds** per decision — 40 here, 160 there, 200/8 at
the live defaults — and its review specimens showed the cost: a near-tie that
*flips* between 40 and 160 worlds is not a measurement, it is an artifact of
the count. The calculated-evidence program is the replacement Jason's Pro
session delivered on 2026-08-24 (hand-ferried, verbatim, checksum-pinned —
lineage on [the intakes page](walt-math-intakes.md#6-the-calculated-evidence-thread--side-channel-hand-ferried)):
**stop declaring n; calculate the work from the decision itself.**

The pieces, in the project's voice (all exploratory, proved relative to walt's
own frozen bases; identities verified exactly at intake, 18/18):

- **Exact anytime-valid evidence** (CE-T1..T5): exact-rational betting
  supermartingales — Bernoulli-threshold, signed-pivotal, bounded-mean — that
  may be *watched while they run*. Stopping early never invalidates the risk
  statement; that is what "anytime-valid" buys and fixed-n analysis does not.
- **A risk ledger, not a per-test α**: a declared run risk δ_run divides across
  decisions (δ_d = δ_run/(d(d+1))) and, within a decision, across the directed
  pairs of a frozen candidate set. Exact results spend no risk (§6.1).
- **The six-way result-type ladder** (CE-A3, binding on the new path):
  `ExactFiberRoot` / `ExactFrozenSet` / `DeltaSettled` / `EpsilonEquivalent` /
  `Unresolved` / `HeuristicFallback`, mechanically distinct in every API and
  log. A sample cap is a **resource limit, never a proof rule**; `Unresolved`
  is a *successful* output that says where the next unit of compute would go
  (the §8.5 refinement vector). Existing fixed-n play paths are
  `HeuristicFallback`-status until retyped — nothing was deleted.
- **The information rate** `𝓘 = q·D_{1/2}(τ)` — pivotal mass times the
  Bernoulli divergence of the pivotal sign from fair — as the true
  sampling-cost coordinate (the SP-A vocabulary: pivotal mass q, tilt τ, gap
  g = qτ, fixed-pair hardness H; **θ is the pivotal win share** (1+τ)/2 and
  **ϑ is an auction/policy threshold** — the θ/ϑ split is CE-A2, walt-wide).
  The leading-order raw-world forecast is ln(T)/𝓘 — a forecast, never a
  stopping rule.
- **Monotone escalation to exactness**: when the calculated remaining sampling
  work exceeds the exact cost, switch to full-fiber enumeration (§11.3); small
  fibers pre-route to the exact frozen-set endpoint directly. Exactness stays
  three-dimensional — exact over the fiber, for a frozen candidate set, under
  a declared field model — and no route ever claims more than its dimensions.
- **Frozen-policy identity** (`FreezeTuple`/`PolicyId`): every evidence record
  names the exact policies and inner schedules it is about; policy mutation
  invalidates evidence; candidate-set mutation starts a new epoch (§5.3).
- **The Phase-1 fence** (§18): outer adaptive settlement first; the *inner*
  minds stay declared sampled approximations (visible in every result
  identity), so no winner is quotable without its schedule. Recursing the
  adaptive machinery inward is step 10, deliberately last.

Adjudication highlights beyond the above: O20–O28 accepted into the
SCENARIO-PLAYER obligations ledger (CE-A4 — obligations, not results); fixed
sample counts leave the correctness path and the block sign racer is narrowed
to heuristic status, its §10.1 sign-vs-mean counterexample now gate fixture V7
(CE-A5); the level-2 probe amended with the three-way wake-up split and the 𝓘
cost coordinate (CE-A6); **§22 adopted as the build program, and the old
player stays the default until arena and conformance gates justify a change,
on Jason's word** (CE-A7, §20.16); no adversary panel convened now (CE-A8).

## The second drop: targeted level-2 field stability

Later the same day, the same Pro-session lineage delivered the targeting
mathematics for level 2 (verbatim, checksum-pinned; exact model-check 19/19
over 1,584 enumerated finite games, ~1.57M pair instances; adjudicated
L2-A1..A7 under the standing same-lineage go — the authorization note travels
with the rulings). The frame it was adopted for (L2-A1): **only worlds and
branches that can reach a field-disagreement state can carry any level-2
correction** — level 2 is a calculated refinement, never a universal re-solve.
The instruments: first-disagreement localization (L2-T1), the root-action
field Lipschitz bound |Q_a⁽¹⁾ − Q_a⁽⁰⁾| ≤ R_a (L2-T2), winner stability under
margin > exposure sum (L2-T3), safe admissible-set screening (L2-T4), and
typed eventual periodicity of best-response towers (L2-T5).

The bindings that matter downstream: **exposure-tier typing** (L2-A4) —
`FrozenPolicyExposure`, `LibraryExposure`, and `RootActionExposureUpper` are
mechanically distinct, **only `RootActionExposureUpper` may feed the
L2-T2..T4 screen**, and every bound names its derivation rung (E0–E4); the
seven field-swap result kinds binding (L2-A3); `walt/LEVEL2-PROBE.md` amended
to the *detection layer* inside the targeted controller (L2-A5); the
field-swap build slotted after CE §22 step 7 (L2-A6); and cycle discipline
with the §13.5 level-3 tripwire — **no damping, mixtures, or robust-cycle
policies without a separate mathematical intake** (L2-A7). Level-model typing
per O36: a level-2 result is a best response to a named σ₁ — never
"equilibrium," "convergence," or monotone improvement. O29–O38 joined the
obligations ledger (L2-A2).

## The build: §22 executed through step 8

All merged to main 2026-08-24; commits are provenance pointers, not receipts.
Step 1 (intake before code) is the two intake/adjudication pairs above
(PRs #15/#16 and #22/#23 — `328ba02`/`125cb72`, `ed08296`/`9dd89f1`).

| §22 step | What landed | PR / merge commit |
|---|---|---|
| 2–3 + A.6 vertical slice | `solver::evidence` (exact evidence arithmetic: CE-T1..T5, ledgers, debt, h±_min) and `solver::adaptive` (canonical kernel adapter, fixed-pair evaluator, exact endpoint) | #19 / `5baad99` |
| 4 | Lazy frozen policies: `FreezeTuple`/`PolicyId`, information-consistent keys, immutable action cache | #20 / `bf432be` |
| 5–6 | The m-candidate decision controller: safe elimination, epoch identity, exact endpoints and switch parity gates on the fiber-90 and fiber-1120 roots | #21 / `636d306` |
| 7 | The shadow harness: frozen level-1 continuations (`ActionRule::PinnedThenLevel1`) + the live-root bridge, `bin/shadow`, and the run's records + README | #24 / `0794ff8` |
| 8 | `solver::calibrate` — the §19 V5 cap-ladder law and the §19 V6 per-fixed-pair E0 calibration; `bin/v5flip`, `bin/e0cal`, and the step-8 records + README | #31 / `e5a5f52` |
| (L2 §21 steps 3–5) | The field-swap slice 1: `solver::field` (FieldId), `solver::exposure` (coupled first-split replay), `bin/fieldswap` + smoke records | #26 / `ffdc002` |
| (L2 §21 steps 6–8) | The field-swap slice 2: exposure rungs E0–E2 and the exact split-reach route E4 in `solver::exposure`, the L2-T4 admissible screen in `solver::field_swap`, `bin/fieldswap_screen` + screen records | #30 / `ca0483d` |
| (cap ruling applied) | Shadow bin `world_cap` default 128 → 512; the committed 128-epoch outputs stay reproducible by passing `128` explicitly | #32 / `6e00528` |

## The shadow instrument (step 7): the controller beside the live player

**Instrument records only** — `walt/probes/shadow/README.md` is the owning
artifact and its fences are quoted, not summarized away: no record is a
receipt, no strength claim is made or implied, the old player remains the
default (§20.16), and **every winner is model-relative to sampled inner minds
at declared schedules (the Phase-1 fence) — no record's winner is quotable
without its schedule.**

The design: 33 hands (13 frozen `verify_player` receipt deals + 20 driven
scenario hands), the focal seat played by the live `level1_evaluate` at 200/8;
at every multi-option focal decision the §16.4 controller *also* runs one
frozen level-1 continuation per legal root action (declared inner schedule
8/2) under δ_run = 1/100 per hand — fibers ≤ 2,000 pre-route to the exact
frozen-set endpoint, larger fibers run the adaptive controller at
world_cap 128 (a resource limit producing honest `Unresolved`, never a
settlement rule). Agreement is recorded, never acted on.

The headline counts (plain counts over the records, regenerable by
`summarize.py`, 2026-08-24):

- **183 decisions shadowed**: **ExactFrozenSet 67 / Unresolved 116 /
  DeltaSettled 0** (ε-mode not configured; §11.3 escalations fired 0 times —
  the h±-based bound is deliberately conservative and small fibers reach
  exactness through the pre-route).
- **The trick gradient**: tricks 1–3 all Unresolved; trick 4 mixed (7 exact,
  21 unresolved); tricks 5–6 all exact. The exact regime and the sampling
  regime partition the hand almost cleanly by fiber size.
- **Live agreement**: a controller winner exists at 27 decisions (all exact
  route); the live choice matched it **23/27**. The 4 disagreements (fibers
  28, 60, 1,750, and one driven root) are exact-for-the-frozen-set references
  where the live 200/8 estimate picked a different tile — the step-8/V5
  fixture class. By design they conflate live outer noise, the frozen
  candidates' smaller inner schedule, and the declared evaluation field;
  separating those is step 8's calibration work, not this instrument's claim.
- **Among survivors 116/116**: at every decision left open at the cap, the
  live player's choice was among the surviving candidates — the controller
  never eliminated the live line at δ_run = 1/100.
- **40 honest exact ties** of the 67 exact settlements — short-horizon roots
  where several pinned continuations win identical world counts; the path
  reports the tie instead of index-breaking (support ≠ belief; a tie is a
  finding, not a defect).
- Timing, instrument-grade orientation only (contended parallel run): live
  eval median ≈ 0.21 s, shadow eval median ≈ 10.3 s.

## What the shadow work found in the live player

Audit findings from the step-7 session (2026-08-24), verified against the
source at `ffdc002` — findings about code, filed as such; the library solver
and `walt_bridge` path that plays the arena and plunge is **clean** on all of
these:

- **The §3.4 defect persists in `playout.rs`'s standalone copy.** The library
  PiKey was repaired 2026-08-17 (the parallel-port catch,
  [walt-seat-play](walt-seat-play.md)), but `walt/walt/src/bin/playout.rs`
  carries its own embedded `PiKey` (line ~158) with fields
  seat/hand/played/leader/plays and **no banked totals** — the exact defect
  the spec's Def 3.4 documents, alive in the probe bin's copy.
- **One RNG threaded through deal + belief sampling** in `playout.rs`,
  `playtable.rs`, and `webtable.rs` — a seat's belief sample depends on how
  many draws other consumers took from the shared stream. This violates the
  O27 sampling-randomness semantics (worlds by counter index from a declared
  seed, evidence/discovery streams domain-separated). Weak coupling, not a
  demonstrated bias — but the discipline exists so the question never arises.
- **`playout.rs`'s `all1` mode is information-inconsistent**: every seat
  plays level-1 "from its own chair," but the per-seat evaluations are not
  pure functions of the acting seat's information state (the shared stream
  above is the visible mechanism), contra the information-consistency
  discipline O29 makes explicit for field models.

These are recorded so they are never re-discovered; repairs route through the
obligations ledger, not silent patches.

## Step 8: the V5 flip repair, and what a fixed-pair costs

**Instrument records only** — [`walt/probes/step8/README.md`](../walt/probes/step8/README.md)
is the owning artifact, its fence binding: exploratory instrument output,
below every evidentiary tier, cited by nothing above it; forecasts are
forecasts and settlement is governed solely by the exact evidence threshold.
Step 8 is gated by the parent's §19 V5 (the historical 40-vs-160 flip) and §19
V6 (per-fixed-pair cost calibration — never pooled), with `solver::calibrate`
as the library authority and `walt/walt/tests/solver_calibrate.rs` as the gate.

> **Two different "E0"s appear on this page, and they are unrelated.** In step
> 8, **E0** is the tilt audit's standing experiment ID (SP-A4), whose parent
> correction is V6's per-fixed-pair discipline — a *cost calibration*. In the
> field-swap sections below, **E0** is the lowest **exposure rung** (exact
> equality of the two fields' choices at every reachable state) — a *bound
> derivation*. Neither name is being reused for the other; both are quoted from
> their own parents, and no statement crosses between them.

**The V5 law, asserted mechanically.** Each flip-shaped root is re-run under
the §16.4 controller on **one epoch and one common indexed stream**, truncated
at an ascending cap ladder 40/160/640 (40 and 160 are the historical
coordinates, kept as replay fixtures per CE-A5 — resource limits, never
settlement rules), with §11.3 exact escalation armed. `assert_cap_ladder`
enforces the law on every ladder: unresolved may settle later; settled stays
settled *identically*; exact stays exact identically; never two caps
"settled" with different answers. **No cap-dependent flip occurred anywhere.**

The step-7 shadow run's four exact-route disagreements were the natural
fixtures, and each is reconstructed by rules replay from its recorded deal and
line prefix — the reconstruction pinned byte-for-byte by asserting that the
**recomputed §5.3 evaluation epoch equals the shadow record's epoch hash**
(a test assertion, not a claim). Three of the four escalate to the exact
winner, which is the honest reading of "the live 200/8 estimate picked a
different tile":

- **receipt h4 d3** (fiber 60, live 1-0): Unresolved at cap 40, §11.3
  escalation fires at stream index 80 → `ExactFrozenSet` winner **2-1**,
  identical at 160 and 640.
- **receipt h7 d5** (fiber 28, live 6-3): escalates at index 16 at every cap →
  `ExactFrozenSet` winner **6-2**, identical everywhere.
- **driven h14 d4** (fiber 50, live 3-1): Unresolved at 40, escalation at
  index 64 → `ExactFrozenSet` winner **2-1** at 160 and 640.
- **receipt h11 d4** (fiber 1750, live 4-2): honest **`Unresolved` at all three
  caps**, and the E0 table says why — the exact-winner-vs-live pair (3-0 vs
  4-2) has τ = 11/175 ≈ 0.06, forecasting ~8k–14k worlds even at the per-pair
  T = 400. The exact reference (3-0 by 1448/1750 to 1415/1750) stands beside
  it, unhidden.

**The count-timing family, and the flip explained.** The 2026-08-23 plunge
review's trick-1 near-tie — slough-the-count (6-2) vs hold-the-count-trump
(6-4), the specimen that motivated the level-2 probe — runs as a deterministic
six-member shape family `g0..g5` over the 46,558,512-world fiber (no exact
route). All six are honestly `Unresolved` at every ladder cap, with pair
counts at q̂ ≈ 0.3 and |τ̂| ≈ 0.01–0.25. **The flip mode is gone**: at both
historical coordinates the controller now returns `Unresolved` rather than a
winner, so the recorded episode is explained as *a near-tie forced through a
phone-tier cap* — not sampler inconsistency, and not a decision the old player
was entitled to make either way. The **literal** plunge position's game seeds
live plunge-side, so the family carries the specimen's shape under an honest
label and the literal reconstruction is filed as a **blocked test**
(`v5_literal_count_timing_position_reconstructs`, `#[ignore]`d), waiting on
[[gran-anchor-reconstruction]] (L2-A6).

**E0 calibration (§19 V6): 18 pairs × 3 replicates at T = 400.** Each record
puts the exact fiber coordinates `(a, b), q, τ, g, H` beside the initial
evidence state (`E± = 1`, `R_debt = T`, `h±_min(0,0;T) = 12` at δ_pair =
1/200), the exact-rational forecasts (§7 information-rate bounds and the §8.4
forecast DP at γ = 1/2 and 9/10), and three observed replicate runs at world
cap 1024. Reading:

- **45 of 54 replicates `DeltaSettled`; the 9 `Unresolved` are exactly the
  three |τ| ≤ 0.11 pairs × 3 replicates** — the near-ties, failing to settle
  where forecast and observation agree they should be expensive (4k–14k
  worlds; the DP reports no crossing within the declared horizon).
- **45/45 settled winners agree with the sign of the exact τ.** This is
  regression evidence consistent with the evidence theorem's δ-validity, *not*
  the theorem and not a distributional test.
- Forecasts bracket observations on strong pairs (|τ| ≥ ~0.7 — e.g. driven h14
  (0,3): DP 33/44, leading-order ~[19,33], observed 30/28/24) and scatter
  around the forecast scale on middling ones (receipt h4 (0,1): leading-order
  ~[203,335], observed 224/150/621) exactly as an exponential-tailed stopping
  time should. **The forecast is a scale, not a promise** — that is its
  declared type, and three replicates per pair is orientation-grade
  calibration of that scale.

Step 9 consumes these per-pair baselines as the field-swap build's Stage-1
evidence layer (L2-A6).

## The field-swap slice 1: three regimes on the first smoke

**Instrument records only** — `walt/probes/fieldswap/README.md` owns the
numbers, and its tier fence is binding and travels verbatim: everything is a
**`FrozenPolicyExposure`** — fixed-policy exposure and correction under
declared field models σ0 (banked-correct level-0 at n0 = 8) and σ1 (level-1
per non-focal seat at 4×2), for two frozen focal pins per root (reveal-shaped
vs retain-shaped) — **NEVER a root-action bound and never screening input**
(L2-A4, O31). The Gran anchor seeds are not yet reconstructed, so the smoke
ran on roots reconstructable from the shadow run. Three regimes appeared on
the first probe:

- **receipt-h7-t5 — the targeting phenomenon, observed in the wild.** On the
  complete 1,680-world fiber the two fields **never split** under either
  policy: d_ρ = 0, hence c_ρ = 0, **exactly** — the level-1 upgrade cannot
  move this root's value for these frozen policies. The parent's central
  targeting claim, exhibited on the first exact-fiber root probed.
- **receipt-h8-t4 — the reveal-response motif.** Exact over the 1,200-world
  fiber: the field upgrade helps the reveal continuation (ĉ_ρ = +1/300) and
  hurts the retain continuation (−9/400); after the 5-5 lead the first field
  split concentrates overwhelmingly at the seat that must answer it first
  (s2, an opponent of the S1 bidder — 898 of 1,138 exposed worlds, trick 4
  ply 1). The motif is reveal-response-shaped, but *which* seat's model wakes
  up is a measurement, **not the Gran partner hypothesis confirmed** (L2-A6);
  and |Λ| = 31/1200 is a fixed-pair statement — no decision claim
  (corrected from 41/1200 per the x:019–023 response §32; the component
  counts +30/−26 and +45/−72 were always correct, the difference was
  mis-added; re-verified from the raw probe records).
- **driven-h0-t1 — the honest trick-1 degeneracy.** On a 64-world stream
  prefix of the 399,072,960-world fiber, exposure ≈ 1: at trick 1 the richer
  field wakes up almost everywhere, so the fixed-policy bound degenerates to
  the naive survivor set. Expected (parent §8.1); tightening is the E0–E2
  exposure rungs' job, not this smoke's.

## The field-swap slice 2: the rungs, an exact zero, and the first singleton

**Instrument records only** — [`walt/probes/fieldswap_screen/README.md`](../walt/probes/fieldswap_screen/README.md)
owns the numbers and its fence is binding: exploratory instrument output below
every evidentiary tier, cited by nothing above it, no strength claim made or
implied. The load-bearing O32/O38 parity gates are the tests
(`walt/walt/tests/solver_fieldswap_screen.rs`); the probe is the instrument
view of the same machinery at one declared epoch pair. Slice 2 is §21 steps
6–8 of the level-2 parent: the exposure **rungs** E0–E2, the exact split-reach
route **E4**, and the L2-T4 **admissible screen**.

**The epoch pair is declared, closing slice 1's loose end.** One (σ0, σ1) pair
per experiment epoch, by declaration: **σ0 = `Level0 { n0 = 8 }`** (the
banked-correct level-0 mind the shadow bin drives non-focal seats with),
**σ1 = `Level1 { n_outer = 4, n0 = 2 }`**, frozen focal candidates at declared
schedule `[8, 2]`. Both FieldIds ride every record; a different schedule is a
different FieldId and a different experiment.

**The rung ladder is type-distinct all the way down, and it verified.** Per
root action the probe produces E1 (counted structural covers), E0/E2 from one
shared pre-split reach walk over the complete fiber, and E4 — the §7.4
hit-frontier objective maximized over information-consistent continuations,
per-node action choice on the public-history tree, **no strategy fusion**
(O34) — whose optimal value *is* `R_a`, exactly. The ladder
**E1 ≥ E2 ≥ E4 = R_a ≥ d_ρ** was verified with exact rationals on every root,
and per parent §7.5 the exact route is typed **E4** — type-distinct from any
future *sampled* E3, which is a lower witness and can never be an upper bound.

Three regimes on the three exact parity roots:

- **receipt-h7-t5 — E0 FIRES, and slice 1's observation becomes a theorem-backed
  exact zero.** After every legal root action there is **no reachable non-focal
  information state at which the two declared fields disagree**, so
  **R_a = 0 exactly over ALL information-consistent continuations** — the
  level-1 upgrade cannot move this root's frozen-set values at all. Slice 1
  saw d = 0 for *two* frozen policies; the rung walk upgrades that to a
  statement over the whole continuation space, in ~0.13 s (the walk never
  splits, so the free-branching tree stays one merged field line — E0's proof
  is the cheapest possible screen input). The screen nonetheless reports
  `FieldSensitive` 3/3, and honestly: all three candidates tie exactly at
  V₀ = 0, so with R = 0 the admissible set is precisely the σ0-argmax set —
  **an exact tie, not field sensitivity in the value sense.**
- **receipt-h4-t6 (fiber 90) — `FieldStableExactFrozenSet`, admissible 1/2: the
  first pruning singleton in the wild.** Survivor **1-1**; its lower field-1
  bound (37/45) clears the excluded action's upper field-1 bound (26/45), with
  ordered-pair slack **S(1-1, 0-0) = 11/45 > 0**. The σ1 parity pass confirms
  the exclusion by brute force — V₁(0-0) = 17/45 < V₁(1-1) = 13/15, strictly
  σ1-nonoptimal — and the excluded action consumed no σ1 optimization. The
  screen result is identical at the cheapest sound bounds and at the exact E4
  bounds.
- **receipt-h8-t4 (fiber 1200) — `FieldSensitive`, admissible 4/4: the honest
  §8.1 degeneracy at a split-heavy root.** Exact R_a between 14/15 and 197/200
  (E2 covers larger still) degenerates the screen to the naive survivor set.
  It prunes nothing and says so; the L2-T2 bound |Q⁽¹⁾ − Q⁽⁰⁾| ≤ R_a and the
  exclusion audit still pass with exact numbers, vacuously maintained.

**On cost, honestly** (instrument-grade microseconds, §12.1): the rungs are
nearly free on h7-t5 (0.13 s of rungs against a 0.008 s σ1 pass, but E0's
proof is the point); they cost about half the naive σ1 pass on h8-t4 and prune
nothing — at split-heavy roots the screen **does not yet earn its keep**,
which is exactly the §17.2 falsifier direction to keep watching; and on h4-t6
the pruning is real but the fiber is 90, where the naive σ1 pass is far
cheaper than the rungs. §12.1's economy question only becomes real at fibers
where C₁(a) dominates. Three roots at one epoch pair is orientation, not a
conclusion. Baseline tier throughout is `exact-frozen-set` — every screen
statement is about the *named frozen candidate set*, never an exact
field-stable root (§15.3), which needs the exact root optimizer and a later
slice. Nothing here says σ1 is a better mind (O36).

## The cap analysis, and Jason's 512 ruling

Mining the persisted §8.5 refinement vectors of the 116 `Unresolved` shadow
records (a 2026-08-24 session analysis over the committed JSONL —
instrument-grade, regenerable from the records, not a separate committed
artifact): the forecast median is **~139 additional worlds** to isolate a
winner; a cap of **512 would settle ~108 of the 116**; and **~22% of open
directed edges are true fog** (|τ̂| ≤ 0.1) that no cap fixes — those are
honest near-ties where the ladder's ε-equivalence and tie modes are the
correct answers, not more sampling.

**Jason's ruling (2026-08-24): world_cap 512 is the way forward.** The
128-era caps were phone-budget artifacts, not statistical choices — and step
8's ladder run bears the same reading out at 40/160/640, where the historical
flip dissolves into an honest `Unresolved` near-tie.

**Applied 2026-08-24 (PR #32 / `6e00528`), after step 8 landed:** the shadow
bin's `world_cap` default is now **512**. The cap is a resource limit either
way (CE-A3), so this changed a schedule, not a correctness rule. The committed
step-7 outputs remain the **world_cap = 128 epoch** and stay reproducible
byte-identically by passing `128` explicitly; a 512-epoch regeneration is a
separate run that supersedes nothing, because it is a different epoch by
construction and every record carries its own config.

## In flight, and deliberately not presented as results

- **Step 9 — the level-2 probe** as the detection layer (L2-A5), consuming
  slice 2's rung/screen machinery and step 8's per-pair E0 baselines as its
  Stage-1 evidence layer (L2-A6); **step 10 — recurse inward** — last, per the
  Phase-1 fence.
- **The controller as an opt-in play mode** is what step 8's gates were the
  precondition for; it has not been built, and per CE-A7/§20.16 the old player
  remains the default regardless until arena and conformance gates justify a
  change, on Jason's word.
- **Gran anchors — carded, artifacts in hand, no results.** Three Plunge
  screenshots pinned at `~/data/texas-42/gran-anchors-2026-08-24/`
  (`MANIFEST.sha256`); the reconstruction path needs **no seeds** — the
  "How it went" grid *is* the complete deal, transcribed and validated by the
  rules engine ([[gran-anchor-reconstruction]]). Until reconstructed, the
  screenshots remain discovery artifacts (parent §1.4, L2-A6) and the G1–G4
  experiments stay gated. Step 8 filed the concrete dependency as a **blocked
  test** — `v5_literal_count_timing_position_reconstructs`, `#[ignore]`d —
  so the literal plunge count-timing position is a named, failing-by-omission
  obligation rather than a silently approximated one.

## Where this leaves the program

The era's one-sentence shape: the seat that already plays now has an
**instrumented correctness path growing beside it** — exact where the fiber
is small, honestly unresolved where it is not, every number typed, every
winner schedule-relative — and the live player is untouched until the gates
say otherwise, on Jason's word. Two of its oldest embarrassments came back
answered rather than explained away: the 40-vs-160 flip is now a priced
near-tie the controller declines to decide, and the "the fields never split
here" observation is now an exact zero over every information-consistent
continuation. Neither is a strength claim; both are the instrument telling
the truth about a position the old player had no way to be honest about. The
open questions this era created (exposure tightening at early tricks,
per-epoch field declaration, the armed-but-unrun cycle tripwire) are
inventoried on [open questions](walt-math-open-questions.md); the obligations
O20–O38 are the ledger debt it took on.
