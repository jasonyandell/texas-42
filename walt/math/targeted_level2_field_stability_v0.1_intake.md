# Intake — *Targeted Level-2 Field-Swap Geometry for Unified Walt*, v0.1

**Status:** exploratory intake companion — the same tier as its parent.
Nothing here is promoted, and nothing here may be cited above exploratory
tier.
**Date:** 2026-08-24
**Parent:** `walt/math/targeted_level2_field_stability_v0.1.md` (filed
2026-08-24, verbatim; `targeted_level2_field_stability_v0.1.sha256` =
`597d33c3227f7ed4e7d6c9287cfdf0433c2777e59909e38f76543ddcc9509e58`).
**The parent stays verbatim.** Observations live beside it and are never
written into it.
**Provenance:** hand-ferried by Jason 2026-08-24 (side channel; not an
automation dispatch — `submission_count.txt` untouched). Same Pro-session
lineage as `calculated_evidence_v0.1.md`; the parent names that document as
its prerequisite and extends it without restating it.
**Snapshot note:** the parent self-describes its reviewed snapshot as main
at `4231cb2…` — the same caught-up main the calculated-evidence parent
cited. Since then the calculated-evidence build has itself landed: §22
steps 2–6 merged (mains `5baad99`, `bf432be`, `636d306` — `solver::evidence`,
kernel adapter, frozen policies/`PolicyId`, m-candidate controller, exact
endpoints), step 7 (shadow) in flight, steps 8+ pending. That is *ahead of*
what the parent assumes, in the direction it requires: its own Step 2 gate
("land the calculated-evidence prerequisite; the field-swap code must
consume the common evidence and exact-escalation authority") is satisfied
by machinery already on main.

---

## 1. Theorem verification

The parent's claims L2-T1..L2-T5 are structural theorems about coupled
executions of two deterministic field models, not numerical identities — so
the mechanical route is **exact finite-game model checking**, not identity
evaluation. `verify_targeted_level2_field_stability_v0.1.py` beside this
file (stdlib only, exact `fractions`, no floats, no randomness) builds
1,584 explicit small games — every ordered pair of the 16 possible field
functions on a one-field-step game crossed with six structurally diverse
Boolean payoffs, plus a two-field-step family that exercises the L2-T1
induction through a mid-game first-split fork — enumerates **every**
information-consistent focal policy and **every** world in each, and checks
every theorem instance exactly. **All 19 checks PASS.** The receipt
headline:

    RESULT: ALL 19/19 CHECKS PASS

What was verified, and at what multiplicity:

- **Model self-consistency:** the coupled execution's two payoffs equal the
  two direct single-field runs, in all 1,584 games (the coupling is the
  real object, not a shortcut).
- **L2-T1 (first-disagreement localization):** `|u1 − u0| ≤ D` pointwise on
  98,688 `(ρ, ω)` instances; §3.2 `|c_ρ| ≤ d_ρ` on 49,344 policies; §3.3
  `|Λ_ab| ≤ d_a + d_b` and the margin-transfer implication
  `g0 > d_a + d_b ⇒ g1 > 0` on 1,573,632 ordered policy pairs.
- **L2-T2 (root Lipschitz):** `|Q_a^(1) − Q_a^(0)| ≤ R_a` with `Q` and `R`
  computed as **exact maxima over all of Π_a** (the optimization lock is
  honored, not approximated), 3,072 root actions.
- **L2-T3 (winner stability):** every instance where the field-0 margin
  exceeds `R_a + R_b` produced a strict field-1 win — no counterexample in
  the family.
- **L2-T4 (safe screening):** the admissible set built from exact bounds
  never excludes a field-1-optimal action (1,536 constructions), and the
  same holds under deliberately loosened bounds (slack 1/8 on `L`, `U`, and
  `R^U`) — looseness cost pruning power, never soundness, exactly as the
  parent claims.
- **Rung E2 (§7.3):** exact `R_a ≤` clairvoyant split-reach mass on all
  3,072 bounds — the strategy-fusion direction is safe.
- **§9.2 ranges:** `Z = Y1 − Y0 ∈ [−2, 2]`, `X = Z/2 ∈ [−1, 1]`, all nine
  cases.
- **L2-T5 (eventual periodicity):** verified on explicit deterministic
  tower operators (fixed point, 2-cycle, tail-then-3-cycle).
- **L2-E0 existence (§16, items 1–8):** all eight fixture phenomena exist —
  never-disagreeing fields with `R = 0`; fields that split with zero payoff
  correction for every policy; positive and negative corrections; a margin
  that beats a *nonzero* exposure sum; a margin that fails the screen with
  an actual field-1 decision flip; a nontrivial L2-T4 exclusion verified
  nonoptimal by brute force; and a genuine deterministic best-response
  cycle (matching pennies, period 4).

The proofs in the parent were also step-checked by hand: L2-T1's induction,
L2-T2's two-sided sup argument, L2-T3's chain
`Q_a^(1) ≥ Q_a^(0) − R_a > Q_b^(0) + R_b ≥ Q_b^(1)`, L2-T4's bar argument,
and L2-T5's pigeonhole are all correct as written. All inequality
directions in §5 (interval form) and §5.1 (slack) are consistent.

## 2. Vocabulary audit

- **"certificate"** appears exactly once, in the parent's own reservation
  sentence ("the word *certificate* remains reserved elsewhere and is not
  used for the objects introduced here") — mention-only, D3-compliant.
- The parent adopts the SP-A vocabulary by name (frozen policy, pivotal
  mass `q`, tilt `τ`, gap `g`, pivotal cover, pivotal win share) and never
  uses bare `θ` for a threshold — consistent with CE-A2's θ/ϑ split, which
  it postdates and does not disturb.
- New named objects — **FieldId**, **field-disagreement frontier**
  `𝓕_{0,1}`, **field-exposure event** `D_ρ`, **field-swap lift** `Λ`,
  **field-stability slack** `S_{a,b}`, the three exposure tiers
  (`FrozenPolicyExposure` / `LibraryExposure` / `RootActionExposureUpper`),
  and the exposure rungs E0–E4 — are all fresh; no collisions anywhere in
  walt/ or wiki/ (grepped at intake).
- The parent's proposed result-type names (`FieldStableExactRoot` …
  `HeuristicFallback`) explicitly defer Rust naming to the project while
  binding the semantic distinctions — same posture CE-A3 took for the
  six-way ladder.

## 3. Numbering audit

- **O29–O38** continue the O20–O28 line (live in `walt/SCENARIO-PLAYER.md`;
  table currently ends at O28). O29–O38 are unclaimed everywhere in walt/
  and wiki/. O10–O11 remain permanently reserved and are untouched.
- **L2-T1..T5** and **L2-E0..E6** are fresh claim prefixes — no collision
  with CE-T1..T5, CE-A1..A8, SP-A1..A12, or any wiki claim ID.
- The parent self-labels its obligations "proposals for intake and
  adjudication, not self-issued rulings" — correct posture.

## 4. Verified code and document boundaries

- **Exists on main today:** `walt::solver::evidence` (anytime-valid
  arithmetic, ledgers), `walt::solver::{adaptive, policy, controller}`
  (streams, FreezeTuple/PolicyId, epoch identity, m-candidate controller
  with safe elimination), `walt::kernel` as the one fiber/count/stream
  authority. The parent's §18 assignment — `field_swap` consumes
  `solver::evidence` and never reimplements it; `kernel` remains the one
  authority — matches the landed architecture exactly.
- **Does not exist yet:** `solver::field`, `solver::exposure`,
  `solver::field_swap`, `FieldId`, any coupled replay, any split-reach
  solver. Green field.
- **`walt/LEVEL2-PROBE.md`:** the existing spec (plus its CE-A6 amendment —
  response/value/decision wake-up split, 𝓘 as the cost coordinate) is the
  *detection* program: paired q̂/τ̂/ĝ under both fields on a position
  corpus. The parent keeps that separation binding (§2) and adds the
  *targeting* layer above it: exposure bounds, the stability screen
  L2-T2..T4, first-split traces, and the survivor-only level-2 optimizer.
  The two documents are consistent; the probe becomes Stage-0 evidence
  inside the parent's controller. Adjudication should amend the probe spec
  to point here rather than letting two field-swap programs drift.
- **The Gran anchors (§1, §11):** the project record of the two Plunge
  specimens is `wiki/walt-seat-play.md` (the 40-vs-160-world near-tie flip
  and the all-100% saturation revelation tie). The **game seeds and full
  records are not in the repository** — G1–G4 require reconstruction from
  the plunge side before the anchor experiments can run. Until then the
  screenshots stay discovery artifacts, exactly as the parent instructs
  (§1.4).

## 5. Observations (nothing found that blocks intake)

1. **The parent is scoped to deterministic fields** and says so twice (§3,
   §13.6.3); stochastic fields require declared tape/coupling semantics and
   a new intake. The current level-0/level-1 field constructions are
   deterministic per information state, so the scope fits.
2. **§7.4's optimization-lock boundary** (a sampled lower witness to `R_a`
   is not an upper bound) is the load-bearing sentence of the exposure
   program; it is the same lock CE-A5 enforced for fixed counts. The
   three-tier exposure typing (§6) is what makes it mechanical.
3. **The split-reach objective** (terminal 1 at first split, 0 otherwise)
   is a genuine reuse of the existing controller with a different Boolean
   payoff — no new evidence mathematics is introduced anywhere in the
   parent. §9 routes everything through CE-T processes on complete signed
   or bounded differences (never sign frequency alone, consistent with the
   §10.1 counterexample in the CE parent).
4. **Cycling posture is conservative and correctly typed** (L2-T5 gives
   eventual periodicity only; no convergence claim; no mitigation
   authorized without a separate mathematical intake — §13.6, §22 item 12).
5. One reconciliation to rule on explicitly: the parent's §21 build order
   begins at "Step 1 — intake" while CE §22 steps 7–8 (shadow, V5 flip
   repair + E0 calibration) are still in flight. The parent's prerequisite
   is the CE *correctness path* (evidence + escalation authority), which is
   merged; the shadow/calibration steps validate the *player*, not the
   authority the field-swap code consumes. Adjudication should slot the
   field-swap build relative to CE steps 7–8 rather than leaving the two
   sequences implicitly parallel.

## 6. Adjudication agenda

1. **L2-T1..T5 soundness** — accept as SOUND at exploratory tier (19/19
   exact model-check receipt + step-checked proofs)?
2. **O29–O38** — accept into the SCENARIO-PLAYER obligations ledger,
   continuing the O20–O28 block?
3. **Result-type semantics** — bind the seven field-swap result kinds
   (§8 Stage 5) as semantic requirements with Rust naming free, extending
   the CE-A3 ladder discipline?
4. **Exposure-tier typing** — bind the three-tier distinction (§6) as a
   mechanical type requirement (only `RootActionExposureUpper` feeds
   L2-T2..T4)?
5. **LEVEL2-PROBE reconciliation** — amend the probe spec to become the
   detection layer inside the parent's targeted controller?
6. **Build-order slot** — where the §21 sequence enters relative to CE §22
   steps 7–8; whether the Gran anchor reconstruction (plunge-side seeds) is
   carded now.
7. **Cycle-tripwire posture** — adopt §13.5/§22-item-12 as the standing
   precondition on any future level-3 work?
