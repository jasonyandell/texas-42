# walt — the calculated-evidence era

[Home](Home.md) · owns: the calculated-evidence program (2026-08-24 onward) —
anytime-valid adaptive settlement as walt's new correctness path, the §22 build
(steps 2–7, with commits), the step-7 shadow instrument and its records, the
live-player audit findings, the targeted level-2 extension and the field-swap
slice's smoke, the sampling-cap analysis and Jason's 512 ruling, and what is in
flight · Sources: `walt/math/calculated_evidence_v0.1.md` (received parent,
adjudicated **CE-A1..A8**) and `walt/math/targeted_level2_field_stability_v0.1.md`
(received parent, adjudicated **L2-A1..A7**) with their intake companions,
`walt/CENSUS-RULINGS.md` (the two adjudication chapters),
[`walt/probes/shadow/README.md`](../walt/probes/shadow/README.md) and
[`walt/probes/fieldswap/README.md`](../walt/probes/fieldswap/README.md)
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
> being cited. Every count quoted from `walt/probes/shadow/` or
> `walt/probes/fieldswap/` is an **instrument record** — probe output that sits
> below every evidentiary tier and is cited by nothing above it; the READMEs'
> own fences and caveats travel with every number here. Nothing on this page is
> a receipt, a strength claim, or a statement about exact values.

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

## The build: §22 executed through step 7

All merged to main 2026-08-24; commits are provenance pointers, not receipts.
Step 1 (intake before code) is the two intake/adjudication pairs above
(PRs #15/#16 and #22/#23 — `328ba02`/`125cb72`, `ed08296`/`9dd89f1`).

| §22 step | What landed | PR / merge commit |
|---|---|---|
| 2–3 + A.6 vertical slice | `solver::evidence` (exact evidence arithmetic: CE-T1..T5, ledgers, debt, h±_min) and `solver::adaptive` (canonical kernel adapter, fixed-pair evaluator, exact endpoint) | #19 / `5baad99` |
| 4 | Lazy frozen policies: `FreezeTuple`/`PolicyId`, information-consistent keys, immutable action cache | #20 / `bf432be` |
| 5–6 | The m-candidate decision controller: safe elimination, epoch identity, exact endpoints and switch parity gates on the fiber-90 and fiber-1120 roots | #21 / `636d306` |
| 7 | The shadow harness: frozen level-1 continuations (`ActionRule::PinnedThenLevel1`) + the live-root bridge, `bin/shadow`, and the run's records + README | #24 / `0794ff8` |
| (L2 §21 steps 3–5) | The field-swap slice: `solver::field` (FieldId), `solver::exposure` (coupled first-split replay), `bin/fieldswap` + smoke records | #26 / `ffdc002` |

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

## The field-swap slice: three regimes on the first smoke

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
  and |Λ| = 41/1200 is a fixed-pair statement — no decision claim.
- **driven-h0-t1 — the honest trick-1 degeneracy.** On a 64-world stream
  prefix of the 399,072,960-world fiber, exposure ≈ 1: at trick 1 the richer
  field wakes up almost everywhere, so the fixed-policy bound degenerates to
  the naive survivor set. Expected (parent §8.1); tightening is the E0–E2
  exposure rungs' job, not this smoke's.

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
128-era caps were phone-budget artifacts, not statistical choices. The raise
is applied **after step 8 lands** — the cap is a resource limit either way
(CE-A3), so this is a scheduling ruling, not a correctness change.

## In flight, and deliberately not presented as results

- **Step 8 — in build, no results**: the V5 fixture (the historical
  40-vs-160-world flip, replayed under evidence semantics) and per-pair E0
  calibration. Only after these pass does the controller become an opt-in
  play mode; the four exact-route disagreements above are its natural
  fixtures. Nothing about step 8 is quotable yet.
- **Step 9 — the level-2 probe** as the detection layer (L2-A5) with the
  field-swap machinery above; **step 10 — recurse inward** — last, per the
  Phase-1 fence.
- **Gran anchors — carded, artifacts in hand, no results.** Three Plunge
  screenshots pinned at `~/data/texas-42/gran-anchors-2026-08-24/`
  (`MANIFEST.sha256`); the reconstruction path needs **no seeds** — the
  "How it went" grid *is* the complete deal, transcribed and validated by the
  rules engine ([[gran-anchor-reconstruction]]). Until reconstructed, the
  screenshots remain discovery artifacts (parent §1.4, L2-A6) and the G1–G4
  experiments stay gated.

## Where this leaves the program

The era's one-sentence shape: the seat that already plays now has an
**instrumented correctness path growing beside it** — exact where the fiber
is small, honestly unresolved where it is not, every number typed, every
winner schedule-relative — and the live player is untouched until the gates
say otherwise, on Jason's word. The open questions this era created (exposure
tightening at early tricks, per-epoch field declaration, the armed-but-unrun
cycle tripwire) are inventoried on
[open questions](walt-math-open-questions.md); the obligations O20–O38 are the
ledger debt it took on.
