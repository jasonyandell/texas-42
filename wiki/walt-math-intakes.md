# walt mathematics — received artifacts and intakes

[Home](Home.md) · owns: the first-class index of walt's received mathematical
artifacts — the frozen bases, the Pro-channel intakes and rebriefs, and their
companions — what each is, where the verbatim parent lives, where its companion
lives, what came of it, and at what tier · Sources: `walt/math/` (the artifacts
themselves), `walt/CENSUS-RULINGS.md` (the adjudications), `exchange/README.md`
(the courier ledger for the ferried dispatches). Related:
[the reference map](walt-math-reference.md),
[the freeze register](walt-math-freezes.md),
[decision-sparse witnesses](walt-math-decision-sparse.md),
[the GPU-native track](walt-gpu-native-trick1.md),
[walt-seat-play](walt-seat-play.md),
[walt-calculated-evidence](walt-calculated-evidence.md) (the era the two
2026-08-24 intakes opened).

> **Epistemic tier: EXPLORATORY — below every tier on
> [Home](Home.md#evidentiary-tiers--never-promoted-never-blurred).** Every
> artifact indexed here is exploratory, without exception. The per-row labels
> below — *received verbatim*, *maintained companion*, *frozen contract*,
> *advisory* — are **provenance disciplines, not evidentiary tiers**: they say
> how a document may be edited (or that it may not be), never how much it is
> believed. Nothing is promoted by being received, audited, or indexed.

## The convention, and what this page may not do

Every received line under `walt/math/` follows one structure, ruled at DS-A18
and GT1-A1 and never varied since:

- **The verbatim parent** is preserved exactly as filed — for the same reason
  `ingest/` is: a corrected source destroys the record of what was corrected.
  Some parents are additionally **checksum-pinned** by a `.sha256` beside them.
- **The companion** (an errata, an intake audit, an adjudicated contract) is
  the *maintained* document, edited only under dated provenance markers, and it
  **governs wherever it repairs or narrows the parent** (DS-A17's citation
  rule, and its GT1 analogue).

This page is the organization layer over that convention: it points, and it
never edits. No entry here is authority over any parent or companion.

**Scope fence.** Exchange dispatches 001–015 — the corpus-adjacent
adjudications (CONFIRMED results, the Lean thread, the informal 014 capture) —
are owned by [claim-ledger](claim-ledger.md) and `exchange/README.md` and are
**not indexed here**. This page owns the *walt-tier mathematical intakes and
received bases* only: everything below was adjudicated (or is queued) inside
walt's exploratory fence, never the CONFIRMED pipeline.

## The lineages at a glance

| Lineage | Verbatim parent(s) | Maintained companion(s) | Adjudication | Outcome in one line |
|---|---|---|---|---|
| The frozen bases (Jason) | `unified_information_geometry_v0.4.md`, `equivariant_lumpability_v0.5.md`, `predictive_algebra_v0.6.md` | `implementers_guide.md` (derived, non-authoritative) | F-series census fork onward; R-A1..R-A24 for v0.6 | The substrate: everything census-era and after is proved relative to these |
| Decision-sparse (Pro) | `decision_sparse_exact_solving_v0.1.md`, `decision_sparse_second_audit_v0.1.md`, `exchange/inbox/016…`, `exchange/inbox/017…` | `decision_sparse_exact_solving_v0.1_errata.md` | DS-A1..A36, SEP-A, FT-A, SR-A | The witness/sandwich mathematics; repaired where unsound, extended two rungs |
| GPU-native (Pro) | `gpu_native_trick1_implementers_guide_v0.2.md`, the M2 and M3 rebriefs | `walt/GPU-NATIVE-TRICK1.md` (v0.3), `-M2.md`, `-M3.md` (frozen contracts) | GT1-A1..A24 | Portable M0/M1 and M2 Metal parity complete; M3 gate frozen, no M3 result |
| Signed-pivotal (side-channel) | `signed_pivotal_geometry_v0.1.md` | `signed_pivotal_geometry_v0.1_intake.md` (intake audit) | SP-A1..A12 | Sound but for one repaired claim; spawned the tilt audit and obligations O12–O19 |
| In-house question/ruling | `WALT-MATH-QUESTION-2026-08-17-…` | `WALT-MATH-RULING-2026-08-17-…` (advisory) | none — advisory, no ruling family | P1–P4 sound; the §12.6A instance stated; the path-dependence counterexample |
| Calculated evidence (side-channel) | `calculated_evidence_v0.1.md` (SHA-256 `9b32b14f…`) | `calculated_evidence_v0.1_intake.md` (intake audit) + `verify_calculated_evidence_v0.1.py` | **CE-A1..A8** (2026-08-24, same-day; the parent already embodied Jason's Pro refinement pass) | Anytime-valid adaptive settlement: 18/18 exact checks PASS; identities SOUND (CE-A1); θ/ϑ split adopted (CE-A2); O20–O28 accepted (CE-A4); §22 is the build program (CE-A7) |
| Targeted level-2 field stability (side-channel, same lineage) | `targeted_level2_field_stability_v0.1.md` (SHA-256 `597d33c3…`) | `targeted_level2_field_stability_v0.1_intake.md` (intake audit) + `verify_targeted_level2_field_stability_v0.1.py` | **L2-A1..A7** (2026-08-24, same-day, under the standing same-lineage go) | First-disagreement localization L2-T1..T5 SOUND, 19/19 exact model-check (~1.57M pair instances); exposure bounds turn level 2 into a calculated refinement; O29–O38 accepted (L2-A2); build slots after CE shadow (L2-A6) |
| Panel response x:019–023 (Pro, adversary panels on both threads) | `exchange/inbox/019-023-response-panel-and-cancellation-v0.1.md` (SHA-256 `a3f468aa…`) | `response_walt_panel_and_cancellation_v0.1_intake.md` + `exchange/inbox/verify_walt_panel_response_v0_1.py` (scratch tier) | **PANEL-A1..A8** (2026-08-24, same-day, standing same-lineage go) | Four briefs certified (020 wording narrowed, 023 with the τ coupling repair); Claim D counterexampled → future-only/preallocated opening binding (PANEL-A3); W7–W11 adopted (PANEL-A5); Part VI cancellation ladder \|c\| ≤ r ≤ d, (B,H,q,g) masses, dominance theorem, directional R± bounds adopted for slice 3 (PANEL-A7/A8); committed lift corrected 41/1200 → **31/1200** |
| Deferred-producers response x:024 (Pro, three-part design brief on the slice-3 deferrals) | `exchange/inbox/024-response-deferred-producers-triple-v0.1.md` (SHA-256 `337296a7…`) | `response_deferred_producers_triple_v0.1_intake.md` + `exchange/inbox/verify_deferred_producers_triple_v0_1.py` (scratch tier) | **TRIPLE-A1..A7** (2026-08-25, same-day, hand-delivered by Jason) | All three deferrals answered at design level: max-preserving upper CS (no Bonferroni; the shipped split-reach count is already S\*_n) for the E3 producer; Hazard-Exclusion Invariant (sound + semantically complete, one verifier authority) for the dominance bound; six-motif first-split morphology + Other for §10 tags, `RevealResponse` refused pending suffix enrichment; branch-mixture upper route retired (wrong orientation); verifier 13/13 PASS incl. exhaustive 65,536-stream sweep; all three producers built with gates same night (slices 4a/4b/4c, main `cbce1ae`) |
| Counted-belief refinement calculus (Pro, side-channel) | `counted_belief_sandwich_v0.1.md` (SHA-256 `4d2dfbe0…`) | `counted_belief_sandwich_v0.1_intake.md` + `verify_counted_belief_sandwich_v0.1.py` (scratch tier) | **CBS-A1..A9** (2026-08-30, same-day, hand-delivered by Jason) | The dual-refinement solver design — root intervals and survivor sets over policy regions and factorized belief: Theorem 5.1 recognized as x:024 M1/M2 over pmake (no new statistics; the pmake sampled optimizer is Slice A's one new producer); seat-factor posterior closure + factorized Bellman adopted (the new mathematics — 116,280 acting-seat hands vs 399,072,960 deals at trick 1); `FiberDp` recognized as the uniform-root `ExactCoverOracle`; §0 probe citations amended (READMEs stay the authority); "sandwich" naming retired for **root interval / survivor set**; verifier 20/20 PASS; build = §56 Slice A + Slice C skeleton, queued on Jason's word |

## 1. The frozen bases

These are not exchange dispatches: they are the project's mathematical basis,
frozen and never edited, on which every walt probe and adjudication since the
census reset is stated. **Provenance below is what each artifact itself
carries** — only v0.5 has an author line; attributions beyond that are marked
as inferred.

- **`walt/math/unified_information_geometry_v0.4.md`** — the v0.4 basis
  (2026-08-09, ~3,820 lines, with its own §17 claim ledger; **no author line —
  the artifact attributes itself to no one**). A clean
  reconstruction after the first four experiment families; supersedes v0.3 plus
  Amendment A for conceptual use (the earlier files remain provenance
  artifacts). Frozen. Everything the [census era](walt-census-era.md) and after
  proves is proved *relative to* this document's objects.
- **`walt/math/equivariant_lumpability_v0.5.md`** — the **§12.6A** amendment
  (2026-08-10; the one basis file with an explicit author line — **Jason
  Yandell**, "statement, definitions, theorem, corollaries, and proofs
  delivered in-session and recorded verbatim"): equivariant controlled
  lumpability over declared role
  interfaces — the ECL condition and the lossless equivariant quotient theorem.
  Reads as §12.6A between v0.4 §12.6 and §12.7; v0.4 stays authoritative for
  everything it states. **What came of it:** the concrete instance was finally
  stated at the [pmake ruling](#5-the-in-house-questionruling-pair)'s Q2
  invariance lemma (2026-08-17), together with an honest negative — on the
  hand-8 carrier its transport group is provably almost trivial, so it "earns
  its keep across carriers and in late endgames," not on that walk.
- **`walt/math/predictive_algebra_v0.6.md`** — the predictive-coordinates
  track (**filed** by Jason 2026-08-12 per [the S6 era](walt-s6-era.md); the
  file carries no author line): continuation tests, exact predictive dimension
  over ℚ, residual closure, forward moments. Two internal inconsistencies a
  reader will hit, recorded here rather than repaired: the header reads
  "Status: … **v0.1**" against the filename's v0.6 (the v0.6 names the *basis
  track* position), and its dependency line cites
  `straight_42_unified_information_geometry_v0.4.md`, a filename that does not
  exist — the filed artifact is `unified_information_geometry_v0.4.md`.
  **What came of it:** the R-A1..R-A24
  adjudication delivered the **v0.6 proof audit** (all SOUND; hypotheses H1–H3
  became builder obligations; gaps G1–G3 named — a subsection of
  § "Predictive-rank probe rulings", not its own section), Lemma R made both
  distribution contracts theorem rows at dimension |X|, and the S6a census
  measured dim V^val and **REFUTED** Gate B — see
  [information geometry](walt-math-information-geometry.md) and the
  [S6 era](walt-s6-era.md#s6a--2026-08-12-predictive-algebra-v06-and-the-dimension-census).
- **`walt/math/implementers_guide.md`** — **derived companion, NOT
  authoritative**: states what an implementer must represent, with a §-citation
  on every line; on any conflict the v0.4 basis and the v0.5 amendment win. It
  reproduces no proofs. It has **no adjudication anywhere** — its only status
  record is narrative
  ([the factory era](walt-factory-era.md): commissioned as a derived,
  non-authoritative companion).

## 2. The decision-sparse thread — Pro channel

The thread that produced the witness/sandwich mathematics. Full object index on
[decision-sparse witnesses](walt-math-decision-sparse.md) and
[the track page](walt-decision-sparse.md); this section is the artifact map.

- **`walt/math/decision_sparse_exact_solving_v0.1.md`** — received verbatim
  (DS-A18; filed as **Jason's handoff, verbatim** — commit `8ee1c9e`), audited
  claim-by-claim at **DS-A1..A18**. Its §7.1 is **unsound as
  written** (repaired by Theorem E1); its economy claim was later split primal
  vs full at EC-A13.
- **`walt/math/decision_sparse_second_audit_v0.1.md`** — the received second
  audit (**Pro's review of the errata, verbatim** — commit `314ea65`),
  adjudicated at **DS-A19..A28** — including DS-A28, the append-only /
  maintained-errata protocol that governs every correction since.
- **`walt/math/decision_sparse_exact_solving_v0.1_errata.md`** — **the
  maintained companion** (DS-A28(iii)): the repaired mathematics with full
  statements and proofs, every change carrying a dated provenance marker.
  **Citation rule (DS-A17):** cite the errata theorem number for mathematics,
  the DS-A ruling for provenance; where parent and errata differ, the errata
  governs. The errata §9 amendment queue (the FT and SR objects) is still owed
  — see [the reference map](walt-math-reference.md#the-ft-chapter). Authored by
  walt-math (commit `f2971ac`) — the one maintained document in this thread.
- **`exchange/inbox/016-decision-sparse-nonanticipativity-taxes.md`** — Pro's
  first-rung nonanticipativity-taxes note (x:016, hand-ferried 2026-08-14, so
  no conversation URL exists — structurally, for all of 016–018),
  adjudicated same day at **FT-A1..A29** into walt's exploratory tier (no
  panel, never the CONFIRMED pipeline); its Experiment 15.1 became the S6k
  fusion-tax probe.
- **`exchange/inbox/017-second-rung-gluing.md`** — Pro's second-rung
  interchange-law note (x:017, hand-ferried 2026-08-14), adjudicated same day
  at **SR-A1..A37**; the SR depth-two probe ran the same day. A resolver trap:
  its frontmatter `extends:` names the 016 note's own *internal* title
  (`decision_sparse_nonanticipativity_taxes_and_plan_calculus_v0.1.md`), which
  is not a `walt/math/` path — do not resolve it against `walt/math/`.

  Both inbox files open with a `<!-- HARVEST METADATA -->` block recording
  `status: UNADJUDICATED` at harvest and the caveat that Pro's
  self-classification labels are **Pro's, not ours, until confirmed** — the
  standing intake-header convention.
- **x:018, the fee-correlation correspondence** — colleague correspondence
  (2026-08-14, no machine-checkable deliverable), reporting the
  FT/SR/FF/FC arc back to Pro and asking the covering-dual question; **awaiting
  Pro's reply — there is no inbox file**, and the only artifact is
  `exchange/outbox/018-fee-correlation-update.md`. It names the
  **conditional-moment gap** blocking trick 1 — an
  open item carried on [open questions](walt-math-open-questions.md).

## 3. The GPU-native thread — Pro channel

Artifact map only; the adjudicated content lives on
[walt-gpu-native-trick1](walt-gpu-native-trick1.md) and in the GT1-A family.

- **`walt/math/gpu_native_trick1_implementers_guide_v0.2.md`** — received
  design input, **byte-frozen and checksum-gated** (GT1-A1): SHA-256
  `ee2e78da…` pinned by the `.sha256` beside it; original source commit
  `ca18bc68…`, intake commit `c230949c…`. Cited by source identity only, never
  as authority over a repair. Its maintained contract is
  **`walt/GPU-NATIVE-TRICK1.md`** (adjudicated v0.3, binding for M0/M1).
- **`walt/math/gpu_native_trick1_m2_rebrief_v0.1.md`** — the exact accepted M2
  rebrief (GT1-A10; 44,079 bytes, SHA-256 `91831325…`), the mandatory bridge
  from freeze 55 to M2. Its frozen contract is **`walt/GPU-NATIVE-TRICK1-M2.md`**
  (v1, SHA-256 `aacb6df5…`, GT1-A17, freeze 56).
- **`walt/math/gpu_native_trick1_m3_rebrief_v0.1.md`** — the exact M3 rebrief
  (GT1-A18; SHA-256 `07b3c993…`), the mandatory bridge from freeze 56 to M3.
  Its binding contract is **`walt/GPU-NATIVE-TRICK1-M3.md`** (v1, SHA-256
  `79de73e9…`, GT1-A24, **freeze 57**). The contract authorizes **only the
  gate** — it records no M3 result, and the GT1 range is re-frozen closed at
  A1..A24.

## 4. The signed-pivotal thread — side-channel, not an exchange dispatch

- **`walt/math/signed_pivotal_geometry_v0.1.md`** — the signed-pivotal
  geometry handoff (filed 2026-08-18, commit `eaf9b23`), **received verbatim
  and checksum-pinned** (SHA-256 `b9d93715…` in the `.sha256` beside it). Its
  own provenance line is a **"house-mathematician pass"** over the unfiled
  import `HANDOFF-plan-geometry-and-names.md` — a Claude side-channel session
  plus ChatGPT 5.6 Pro, per the filing — **not a pure Pro note and not an
  exchange-ledger dispatch** (the ledger stops at 018; no dispatch ever
  corresponded to it). The numbering gap its transit left (O10–O11)
  is permanently retired by SP-A11.
- **`walt/math/signed_pivotal_geometry_v0.1_intake.md`** — **the intake
  companion**: the exact-rational verification of every boxed identity in the
  parent — g = qτ, E[Y²] = q, Var(Y) = q − g², H = 1/(qτ²) − 1, the world/tape
  projection, strata linearity, the cover identity — by hand and on 2,000
  random exact-rational instances, which the SP-A audit takes as read.
- **Adjudication: SP-A1..A12** (2026-08-18, `walt/CENSUS-RULINGS.md`
  § "Signed-pivotal intake adjudication"): the central mathematics **SOUND**;
  exactly one general claim FALSE as written (§2.1's "strictly sharper",
  repaired at SP-A5 — pairing helps iff Cov > 0); two vocabulary collisions
  resolved by rename (**pivotal cover**, **pivotal win share**, **frozen
  policy** — SP-A1..A3); E0 adopted as **the tilt audit** with three design
  corrections (SP-A8..A10); obligations **O12–O19** filed into
  `walt/SCENARIO-PLAYER.md` §10.
- **What came of it:** the tilt-audit smoke (`walt/TILT-AUDIT.md`, run
  2026-08-19 — the "modeled field has no tape" structural finding, the racing
  verdicts) and the level-2 field-swap detector (`walt/LEVEL2-PROBE.md`, spec
  only). Object index for both on
  [the reference map](walt-math-reference.md#the-scenario-player-era--signed-pivotal-geometry-the-tilt-audit-and-the-level-2-detector).
- **The unfiled import.** `HANDOFF-plan-geometry-and-names.md` — named by the
  parent, never filed — **stays unfiled, is not being retrieved** (Jason,
  2026-08-18), and is unciteable (SP-A11); its literature mappings are
  unciteable with it (§15 verdict). Retired numbers are never reused, which is
  why the parent's filed obligations start at O12.

## 5. The in-house question/ruling pair

- **`walt/math/WALT-MATH-QUESTION-2026-08-17-pmake-and-the-walk-to-trick-1.md`**
  and **`…-RULING-…`** — the question and its same-day answer (walt-math-12,
  recorded verbatim below the rule). **Advisory mathematics recorded outside
  the rulings file**: no CENSUS-RULINGS family, no `.sha256`, no probe number
  promoted, and — before this index — no reference to the pair anywhere outside
  `walt/math/` (not the wiki, not `walt/LOG.md`, not kanban). This page and
  [the reference map](walt-math-reference.md) are now its only inbound
  pointers; if its content is ever consumed by a design, it needs a series home
  first. Rulings are against `ladder.rs` as of commit `171cd22`. Delivers the P1–P4 soundness verdicts
  (decided cutoffs, viewer early exit, pmake key reduction, gcd-normalized
  posteriors), the **path-dependence counterexample** (the exact posterior is
  not a function of the reduced boundary key), the concrete **§12.6A invariance
  lemma** instance with its ECL clauses, the allowance-automaton coarsening,
  and the honest negative on hand 8. Indexed with the scenario era on
  [the reference map](walt-math-reference.md#the-scenario-player-era--signed-pivotal-geometry-the-tilt-audit-and-the-level-2-detector).

## 6. The calculated-evidence thread — side-channel, hand-ferried

- **`walt/math/calculated_evidence_v0.1.md`** — *Calculated Evidence for
  Unified Walt* (received verbatim 2026-08-24, hand-ferried by Jason;
  checksum-pinned, SHA-256 `9b32b14f…`; not an automation dispatch —
  the courier ledger is untouched). The adaptive-settlement mathematics
  the board queued as [[adaptive-sampling-intake]]: exact-rational
  anytime-valid evidence processes (CE-T1..T5 — Bernoulli-threshold,
  signed-pivotal, and bounded-mean betting supermartingales), a
  decision/run risk-ledger discipline, the information rate
  `𝓘 = q·D_{1/2}(τ)` as the true sampling-cost coordinate, monotone
  escalation from sampling to exact full-fiber enumeration, the six-way
  result-type ladder, frozen-policy identity (`FreezeTuple`/`PolicyId`),
  the level-2 wake-up decomposition (response/value/decision), and
  proposed obligations **O20–O28** continuing the SCENARIO-PLAYER line.
  Self-describes its reviewed snapshot as main `4231cb2…` — verified at
  intake to be exactly the post-reorganization state.
- **`walt/math/calculated_evidence_v0.1_intake.md`** — **the intake
  companion** (maintained). All 18 mechanical identity checks PASS
  exactly (`verify_calculated_evidence_v0.1.py`, stdlib rationals, no
  floats; the central closed form verified three independent ways over
  the V1 grid). Records the vocabulary adjudication proposal (θ = pivotal
  win share / ϑ = auction threshold, resolving the collision the
  signed-pivotal companion flagged), the O20–O28 numbering check, the
  verified current-code boundaries (the legacy `sample_belief` seam has
  three expressions, not one; the `16×` literal sits at two sites), and
  the six-point adjudication agenda.
  **Adjudicated same-day at CE-A1..A8** (`walt/CENSUS-RULINGS.md`,
  2026-08-24): the parent came out of Jason's Pro session iterating on
  the post-reorganization state, so the iteration-policy refinement pass
  was already embodied; Jason ruled go. Identities SOUND (CE-A1); θ/ϑ
  vocabulary split adopted walt-wide (CE-A2); the result-type ladder
  binding on the new correctness path (CE-A3); O20–O28 accepted into the
  SCENARIO-PLAYER ledger (CE-A4); fixed counts leave the correctness
  path and the block racer is narrowed to heuristic status (CE-A5); the
  level-2 probe amended with the wake-up split and the `𝓘` cost
  coordinate (CE-A6); the §22 sequence adopted as the build program,
  A.6 vertical slice first, old player stays default until gates justify
  a change on Jason's word (CE-A7); the refinement-agenda items
  dispositioned, no panel convened now (CE-A8).

- **`walt/math/targeted_level2_field_stability_v0.1.md`** — *Targeted
  Level-2 Field-Swap Geometry for Unified Walt* (received verbatim
  2026-08-24, hand-ferried by Jason later the same day; checksum-pinned,
  SHA-256 `597d33c3…`; same Pro-session lineage — the parent names the
  calculated-evidence handoff as its prerequisite and extends it). The
  targeting mathematics for level 2: the field-disagreement frontier
  `𝓕_{0,1}`, first-disagreement localization (**L2-T1**), the root-action
  field Lipschitz bound `|Q_a^(1) − Q_a^(0)| ≤ R_a` (**L2-T2**), winner
  stability under margin > exposure sum (**L2-T3**), safe admissible-set
  screening (**L2-T4**), eventual periodicity of deterministic
  best-response towers (**L2-T5**), three mechanically distinct exposure
  tiers, exposure rungs E0–E4 (exact equality → structural cover →
  clairvoyant reach → information-consistent split-reach solve → exact
  closure), first-split traces as model-grounded explanations, the Gran
  anchor experiments G1–G4, the L2-E0..E6 experiment program, and
  proposed obligations **O29–O38**. Cycling is typed and instrumented,
  never assumed away and never damped without a separate intake.
- **`walt/math/targeted_level2_field_stability_v0.1_intake.md`** — **the
  intake companion** (maintained). The theorems are structural, so the
  mechanical route is exact finite-game model checking
  (`verify_targeted_level2_field_stability_v0.1.py`, stdlib rationals, no
  floats, no randomness): 1,584 enumerated games, every
  information-consistent focal policy and world, **19/19 checks PASS** —
  L2-T1 on 98,688 pointwise instances, the pair bound on 1,573,632
  ordered pairs, L2-T2..T4 with exact suprema over the full policy sets,
  screening soundness under deliberately loosened bounds, the E2
  clairvoyant rung, and all eight L2-E0 fixture phenomena exhibited
  (including a genuine period-4 best-response cycle). Records the
  vocabulary audit (one mention-only "certificate" = the reservation
  sentence itself; no bare-θ usage), the O29–O38 and L2-prefix freshness
  checks, the code-boundary audit (consumes landed `solver::evidence` /
  `controller`; `solver::field` / `exposure` / `field_swap` are green
  field), the LEVEL2-PROBE reconciliation (probe = detection layer inside
  the targeted controller), the Gran-anchor gap (game seeds not in-repo;
  reconstruction pending on the plunge side), and the seven-point
  adjudication agenda.
  **Adjudicated same-day at L2-A1..A7** (`walt/CENSUS-RULINGS.md`,
  2026-08-24, filed under the standing same-lineage go — the
  authorization note travels with the rulings): L2-T1..T5 SOUND and
  the targeting frame adopted (L2-A1); O29–O38 accepted (L2-A2); the
  seven field-swap result kinds binding with Rust naming free (L2-A3);
  exposure-tier typing binding, only `RootActionExposureUpper` feeds
  the screen (L2-A4); LEVEL2-PROBE amended to the detection layer
  (L2-A5); the field-swap build slots after the CE shadow step, Gran
  anchors carded as [[gran-anchor-reconstruction]] (L2-A6); cycle
  discipline and the level-3 tripwire adopted, no mitigation without a
  separate intake (L2-A7).

- **`walt/math/counted_belief_sandwich_v0.1.md`** — *Counted Belief
  Sandwiches and the Refinement Calculus for Walt* (received verbatim,
  hand-delivered by Jason 2026-08-29/30; checksum-pinned, SHA-256
  `4d2dfbe0…`; not a courier dispatch — the ledger is untouched; the
  parent self-names `DESIGN-walt-counted-belief-sandwich-v0.1.md`,
  recorded not repaired). Written against main `a1d2219` after the
  speed campaign convicted the explicit-world representation. The
  correctness-preserving path from sampled orientation to factorized
  exact best response: root intervals and survivor sets (Part I),
  the optimization-lock upper over the full information-consistent
  policy class (Part II — recognized at intake as x:024 M1/M2 over
  the pmake objective), policy cylinders and grammars (Part III),
  counted consequence cells and threat/hazard covers (Part IV), the
  seat-factor posterior-closure theorem and factorized Bellman
  recursion (Part V — the genuinely new mathematics), the
  `ExactCoverOracle` contraction interface (Part VI), consequence
  CEGAR (Part VII), the slice program A–G (Part XI), and proof
  obligations CBS-O1..O15 with a Lean module map (Part XIII).
- **`walt/math/counted_belief_sandwich_v0.1_intake.md`** — **the
  intake companion** (maintained). Verifier re-run 20/20 PASS from the
  filed location (exhaustive 65,536-stream coverage sweep, worst
  undercoverage 11/128 < 1/4 on both endpoints; 90-deal factor-belief
  closure, Z = 282 both ways; factorized Bellman = explicit
  optimization at 30/47). Records the identity finding (Theorem 5.1 =
  adjudicated M1/M2), the code-boundary audit (split-reach S* ships,
  the pmake sampled optimizer is green field; shipped fields verified
  seat-local; `FiberDp` attribution amended from `kernel.rs` to
  `fiber.rs`), the §0 probe-citation audit (three of four READMEs'
  own negative verdicts softened by the parent — the corrected record
  strengthens the thesis), and the vocabulary sweep (zero bare
  "certificate"; the "sandwich" collision resolved by adopted names).
  **Adjudicated same-day at CBS-A1..A9** (`walt/CENSUS-RULINGS.md`,
  2026-08-30, standing same-lineage go plus Jason's explicit
  in-session word): intake accepted at instrument tier (CBS-A1);
  Part II recognized, §44 step 1 amended (CBS-A2); result types
  adopted, root interval / survivor set naming ruled (CBS-A3);
  cylinders/grammars adopted with the O34 fence restated (CBS-A4);
  counted cells and covers adopted (CBS-A5); posterior closure adopted
  with binding boundary obligations, `FiberDp` = the uniform-root
  backend (CBS-A6); CEGAR adopted, witnesses decide (CBS-A7); §0
  probe citations amended, READMEs stay the authority (CBS-A8); §56
  build program adopted — Slice A + Slice C skeleton first, default
  player untouched, CBS-O1..O15 to the Lean ledger, GPU under the
  ripcord discipline (CBS-A9).

- **`walt/math/anytime_proof_state_score_v0.1.md`** — *Anytime
  Proof-State Walt: count-aware score bounds, certified regret, laydown
  semantics, and iterative refinement* (received verbatim,
  hand-delivered by Jason 2026-08-31 ("this is a biggie");
  checksum-pinned, SHA-256 `7a8c60fb…`; not a courier dispatch — the
  ledger is untouched; the verifier docstring self-names the parent
  `DESIGN-walt-anytime-proof-state-and-score-calculus-v0.1.md`,
  recorded not repaired). Written against main `25b40d9` — the Slice G
  merge itself — the day after the C→G ladder closed. The central
  chain: behavior uncertainty → score uncertainty → contract
  uncertainty, only the last controlling pmake. The score layer
  beneath the contract (Parts I–III: 43-bin exact profiles, the
  tail-sum identity, contract-sensitive residual mass W_ρ(c), rescue
  and fragile-make bands, count-threat covers), the typed laydown
  hierarchy (Part IV), the score-profile Bellman calculus with the
  envelope-is-not-a-policy fence (Part V), the persistent proof state
  as the primary theorem object (Part VI), proof bar vs executable bar
  with certified pmake regret (Part VII), the debt taxonomy and
  declared solve goals (Part VIII), closure-aware work selection
  (Part IX), the fresh-orchestration-core architecture ruling behind a
  spike gate (Parts X–XI), phases 0–8 (Part XII), and the Lean program
  PS-T1..T15 (Part XV).
- **`walt/math/anytime_proof_state_score_v0.1_intake.md`** — **the
  intake companion** (maintained). Verifier re-run 36/36 PASS from the
  filed location (256-signature census, exhaustive rescue/fragile
  sweep, 65,625-case certified-regret sweep, the 63/2
  threshold-envelope non-realizability specimen re-derived,
  merge-before-max and closure-aware-scheduling counterexamples; weak
  checks recorded). Records the identity findings (§29 survivor
  theorem = CBS 2.1, Slice G's shipping law; §23 merge-before-max =
  the O34 fence over score profiles), the code-boundary audit (the
  shipped `bar_of` IS the proof bar — sound for G's action-selection
  scope, with the executable-bar split becoming load-bearing at Phase
  3; the selection-debt definitional variation recorded; §0's
  engineering summaries verified ACCURATE — no probe softening found,
  a first for this lineage), the §12 wording caveat (the companion
  governs), and the vocabulary sweep (zero bare "certificate";
  "laydown" enters as a typed hierarchy with the bare word reserved
  for the universal quantifier). **Adjudicated same-day at
  APS-A1..A9** (`walt/CENSUS-RULINGS.md`, 2026-08-31): score layer
  sound and adopted (APS-A2); covers under the CBS-A7 discipline
  (APS-A3); the envelope fence binding (APS-A4); laydown typing
  (APS-A5); the bar split with the audit finding (APS-A6); certified
  regret as the finite-budget deliverable (APS-A7); closure-aware
  usefulness amending the steering doctrine, G's refusal rule
  correct-in-scope (APS-A8); the greenfield proof-state core adopted
  as candidate behind the §49 spike, `solver::refine` frozen as the
  RefineV1 reference, phases queued on Jason's word, default player
  untouched (APS-A9).

## 7. The pinned manifests

The `.sha256` files under `walt/math/` are **pinned freeze artifacts** — never
edited, superseded only by append-only re-issue.

Only the six received parents below carry companion `.sha256` files; the
M2 and M3 rebriefs' hashes are **ruling-carried only** (GT1-A10, GT1-A18).

| File | What it pins | Fixed at |
|---|---|---|
| `gpu_native_trick1_implementers_guide_v0.2.sha256` | The received v0.2 guide's byte identity (`ee2e78da…`) | GT1-A1 |
| `signed_pivotal_geometry_v0.1.sha256` | The received signed-pivotal parent's byte identity (`b9d93715…`) | the 2026-08-18 intake |
| `gpu_native_trick1_m0_m1_sources_v1.sha256` | The portable M0/M1 source closure at the pre-fold layout — `BuildIdentityV1`, identity `eccf0a37…` | GT1-A9 / freeze 55 |
| `gpu_native_trick1_m0_m2_sources_v1.sha256` | The cumulative M0–M2 source closure, **byte-immutable** — the `M2BuildIdentityV1` the standing M2 receipt names | GT1-A17 / freeze 56 |
| `gpu_native_trick1_m0_m2_sources_v2.sha256` | The post-fold re-issue at the unified layout — a **new** build identity (`8a780895…`), attested by no hardware receipt yet | FZ-A1..A6; re-earning deferred to [[m2-receipt-reearn]] |
| `calculated_evidence_v0.1.sha256` | The received calculated-evidence parent's byte identity (`9b32b14f…`) | the 2026-08-24 intake |
| `targeted_level2_field_stability_v0.1.sha256` | The received level-2 field-stability parent's byte identity (`597d33c3…`) | the 2026-08-24 intake (second drop, same day) |
| `counted_belief_sandwich_v0.1.sha256` | The received counted-belief parent's byte identity (`4d2dfbe0…`) | the 2026-08-30 intake |
| `anytime_proof_state_score_v0.1.sha256` | The received anytime proof-state parent's byte identity (`7a8c60fb…`) | the 2026-08-31 intake |

## 8. Pending, and deliberately not indexed as landed

- **The calculated-evidence build — steps 2–8 executed, step 9 pending**
  *(updated 2026-08-24, later the same day)*: the §22 program landed
  through the shadow instrument and its calibration (`solver::evidence` /
  `adaptive`, frozen policies, the decision controller, exact endpoints,
  the step-7 shadow run, and `solver::calibrate` for step 8 — mains
  `5baad99`/`bf432be`/`636d306`/`0794ff8`/`e5a5f52`). Step 8's gates —
  the V5 cap-ladder law and per-fixed-pair E0 calibration — passed with
  instrument records at `walt/probes/step8/`; the old player stays the
  default regardless (CE-A7/§20.16), and the opt-in play mode those gates
  were the precondition for is unbuilt. Step 9 (the level-2 probe as
  detection layer) is next. Build narrative and instrument records:
  [walt-calculated-evidence](walt-calculated-evidence.md).
- **The field-swap build — two slices landed** *(updated 2026-08-24,
  later the same day)*: slice 1 merged `solver::field` and
  `solver::exposure` plus the `fieldswap` bin (main `ffdc002`, its L2-A6
  slot satisfied by the step-7 merge) and ran the §21 step-5 fixed-policy
  smoke (`walt/probes/fieldswap/README.md` — `FrozenPolicyExposure` tier
  only, never root-action screening, L2-A4). Slice 2 (§21 steps 6–8, main
  `ca0483d`) added the exposure rungs E0–E2, the exact split-reach route
  E4, and the L2-T4 admissible screen in `solver::field_swap`, with
  records at `walt/probes/fieldswap_screen/`. Still unbuilt: the exact
  root optimizer the `ExactRoot` tier needs (§15.3), and the targeted
  field-1 controller the rungs are inputs to.
- **The Gran anchor reconstruction** — G1–G4 required the two Plunge
  game records; the three screenshots are now archived with a manifest
  (`~/data/texas-42/gran-anchors-2026-08-24/`, `MANIFEST.sha256`) and
  the carded path needs **no seeds** — the "How it went" grid is the
  complete deal, transcribed then rules-engine-validated. Until the
  validated records are committed the screenshots remain discovery
  artifacts (parent §1.4, L2-A6). Carded as
  [[gran-anchor-reconstruction]].
- **The counted-belief build — Slice A landed** *(2026-08-30, same day,
  on Jason's word)*: `solver::root_interval` (root intervals, survivor
  sets, the typed decision ladder, the §6 discovery/evaluation lock, the
  mirror lower endpoint) plus `exposure::sampled_root_optimum` (the
  pmake empirical optimum — CBS-A2's one green-field producer),
  gates at `tests/solver_root_interval.rs` (6/6, incl. realized
  L ≤ Q ≤ U against `exact_root_value` and the adjudicated-11/128
  mirror sweep), instrument `bin/rootinterval.rs` with records at
  `walt/probes/root_interval/` (h4-t6 settles to the exact optimum in
  8 worlds; four of six receipt roots are exact ties, honestly
  `UnresolvedRootSet`), and the Slice C design skeleton
  `walt/FACTOR-BELIEF.md` (§56's second output — types, oracle trait,
  gates; no implementation). The default player is untouched (CBS-A9).
- **The counted-belief build — Slice C stage C0 landed** *(2026-08-30,
  the following session)*: `solver::factor_belief` — the §43-identity
  `FactorBelief`, the `ExactCoverOracle` contraction trait, and backend
  zero (`FiberOracle` wrapping the shipped `FiberDp` for 0/1 factors,
  per CBS-A6), with the Theorem 20.1 conditioning route and the §46
  mass-conservation gate asserted at every contraction. Gates at
  `tests/solver_factor_belief.rs` (7/7, incl. branch-mass parity with
  complete-world enumeration under three fields and the two-table
  Slice-D boundary refused by panic); instrument `bin/factorbelief.rs`
  with records at `walt/probes/factor_belief/` — the §22 opening root
  contracted to its exact branch table in 8.7 ms (trivial field) and
  5.6 s (σ0 level-0 classification of all 116,280 hands, the C2 shape),
  399,072,960 worlds never materialized, conservation exact. Register
  `walt/FACTOR-BELIEF.md` updated with the build-time trait deviations.
  The default player is untouched (CBS-A9).
- **The counted-belief build — Slice B landed** *(2026-08-30, the same
  session's next round)*: `solver::grammar` — §11's induced
  `PolicyGrammar` over `SlicePolicy` sources (legal combination by
  information state, never by hidden world — type-enforced), one walk
  producing the §12 triple `free`/`gram`/`dev` with Theorem 9.1's
  `free = max(gram, dev)` asserted at EVERY node, verdicts under the
  decided-truncation quotient, lazy first-deviation witnesses (CBS-A4),
  the grammar-room census, the `CountPreservation` safety source, and
  `residual_empirical_max_upper` — the §8 identity made mechanical: the
  sampled residual upper IS the full-class upper (off-sample deviation
  realizes the unrestricted optimum inside the residual; Corollary 5.2
  refuses anything smaller), so sampled partitioning tightens nothing
  and genuine residual bounds are the exact side's (`dev` over the
  fiber). Gates at `tests/solver_grammar.rs` (8/8, incl. nodewise
  identity against `exact_root_value`, singleton-grammar == §6 replay
  count, and the frozen sweep finding: the two-preference grammar
  leaves NO exact counterexample at tricks 5–6 while the singleton
  does); instrument `bin/grammarsplit.rs` with records at
  `walt/probes/grammar_residual/` — root closure attained by G2/G3 on
  all six fixtures, the §12 boxed exclusion realized exactly with
  margin 1 (h8-t5: gram 71 / dev 70), and one true counterexample with
  its depth-4 witness. Recorded deviation: the level-2/waking grammar
  source of §45 awaits a `SlicePolicy`-shaped continuation; the σ0 mind
  stands in. The default player is untouched (CBS-A9).
- **The counted-belief build — stage C1 landed** *(2026-08-30, the same
  session's next round)*: the cache study proper, with ZERO library
  code added — the C0 contraction plus `FieldModel`'s insert-only cache
  already classify once per information state; C1 is the gates and the
  measurements. Four new gates at `tests/solver_factor_belief.rs`
  (11/11): σ0 branch parity with the bundled one-ply oracle
  (`solver::bundle`'s field-ply partition idiom at one ply) on ALL six
  receipt fibers, with the two routes' caches asserted EQUAL AS MAPS —
  the feasible root hands exactly, one action each; classification once
  per state (a repeat classifies nothing; conditioning adds exactly the
  zero-completion support hands, once); the §43 identity law — zero
  sharing across focal candidates or roots, because the full key
  carries the public history; and the opening root's 116,280 hands
  classified by σ0 exactly once. Instrument: `factorbelief cache` mode,
  record `walt/probes/factor_belief/cache_run1.txt` — §26 coordinates:
  46 µs/hand first classification (5.36 s total at the opening), 200
  ns/query repeat identity cost (23.3 ms, ×230), conditioning at the
  voidless opening materializes 0 new states (pure table filtering),
  cross-history hits exactly 0 of 36 — the honest negative that routes
  classifier compression to Slice F's proven state reductions, never a
  looser key. The default player is untouched (CBS-A9).
- **The counted-belief build — stage C2 landed, Slice C complete**
  *(2026-08-30, the same session's next round)*: the §46 opening-root
  report, again with ZERO library code and no new gate — C2 is a REPORT
  stage, and every invariant its run asserts is already gated (gate 10
  carries the opening root's hand count, once-per-hand σ0
  classification, repeat-is-pure-identity, and `Z_h = Σ_t Z_ht` over
  399,072,960). New `factorbelief c2` mode, record
  `walt/probes/factor_belief/c2_run1.txt`: all seven coordinates §46
  requires reported separately from ONE run at the frozen h0-t1 root
  under the σ0 `Level0 { n0 = 2 }` field — 116,280 acting-seat hands
  (asserted); contraction 5,933 µs for the completion weights alone and
  21,818 µs warm (weights plus full §43-key identity, zero
  classifications); field classification 5,339,731 µs derived by
  subtraction from the 5,361,549 µs cold pass, 45 µs/hand and 99% of
  the bill; 20 distinct branch tiles; cache reuse ×245 at 187 ns/query;
  memory as TWO figures kept apart — a DECLARED ACCOUNTING of
  23,563,392 bytes for the action cache (88-byte entries, 262,144
  buckets by the documented map growth policy, one control byte each,
  plus the key's one-tile history Vec) beside a MEASURED 63,340,544-byte
  maximum resident size under `/usr/bin/time -l` (agreeing to the byte
  with the in-run `/bin/ps` reading at exit; peak footprint 62,390,680
  bytes); conservation exact at 399,072,960. The memory coordinate C1
  deferred is thereby discharged, with no estimate presented as a
  measurement. Beyond the seven, §26 item 5: conditioning on the
  heaviest branch (1-0, mass 125,370,960) leaves 36,530 of 116,280 hands
  in support, 0 new states. `walt/FACTOR-BELIEF.md` now reads SLICE C
  COMPLETE. The default player is untouched (CBS-A9). Still unbuilt:
  Slices E–G; Slice D's recursion landed the same day (next entry).
- **The counted-belief build — Slice D landed** *(2026-08-30, the same
  session's next round)*: the general support contraction
  (`SupportOracle` — §25.2's acting-hand loop generalized to
  conditioned completions, walking explicit supports so backend zero's
  two-table refusal stays intact as the C0 boundary) and the §23
  factorized fixed-policy recursion (`viewer_success_mass` — the
  viewer-objective success mass `M` with `V = M/Z` the exact integer
  pair, §23 cleared of denominators by conservation, no rationals
  anywhere). Five gates at `tests/solver_factor_recursion.rs`:
  C0-domain extensional parity with backend zero (through the opening
  root's contraction); surviving-world mass parity beyond one table,
  with backend zero refusing at the boundary; the §47 value gate — the
  recursion equal to the bundled walk (`bundled_set_outcomes`) on every
  enumerable root × two frozen focal policies × the trivial and σ0
  fields; and the every-node checker — mass equals the surviving-world
  count and branch masses equal the world partition at EVERY node of
  the recursion tree. One law discovered at depth: `condition` now
  restricts its support walk to hands consistent with the public
  record — such hands are provably zero-mass and their action
  likelihood is undefined; σ0's type-enforced information-state
  constructor caught the unlawful classification, and the filter is a
  no-op at one ply, so C1's conditioning-support law is unchanged.
  Instrument: `factorrecursion report`, record
  `walt/probes/factor_belief/recursion_run1.txt` — parity on every row
  including trick-4 roots (deepest: fiber 34,650, 16 post-root plies,
  121,868 conditionings under σ0); honest negatives: the bundled walk
  is faster at worlds/hands ≈ 3, and the recursion classifies
  record-consistent zero-completion hands the bundled route never
  meets — the contraction advantage remains the worlds-to-hands ratio
  (3,432 at the opening), and the opening-root recursion is
  deliberately not attempted. The default player is untouched
  (CBS-A9). Slice E landed in a following round (next entry).
- **The counted-belief build — Slice E landed** *(2026-08-30, the round
  after Slice D)*: the §48 factorized grammar best response
  (`grammar_success_mass` in `solver/factor_belief.rs`) — the §23
  recursion with the focal case's single frozen action replaced by a
  MAX over the grammar's actions, `M^G(B) = max_{t ∈ G(I)} M^G(B·t)`,
  lawful on the cleared side because every focal child shares `Z(B)`
  (a focal play changes no factor), with nodewise max equal to the §12
  policy-class optimum `Q^G` by the cylinder-partition argument; one
  new library function plus a stats carrier, per-root-action values
  needing no separate producer (`Q^G_a` is the recursion after
  `focal_play(a)`). Four gates at `tests/solver_factor_response.rs`:
  per-action parity with Slice B's `exact_grammar_split` grammar
  optimum under σ0 (the enumeration-side authority), singleton-grammar
  collapse to the Slice D recursion, source dominance with the
  constraint proved to bind via singleton grammars, and the every-node
  checker with the grammar-max structure enumerated. The §48 fence
  kept: nothing maximizes over the full action set, and no
  argmax/policy is extracted (that needs a declared tie order — not a
  Slice E claim). Instrument: `factorresponse report`, record
  `walt/probes/factor_belief/response_run1.txt` — the finding is that
  AT DEPTH THE MIX PAYS: at trick-4 roots the grammar optimum strictly
  beats every source (h4-t4 trivial: `Q^G = Z = 34,650`, certain make,
  against 34,170 for the best source; gaps 90–753 worlds of
  make-mass), while at trick-5/6 roots it never exceeds the best
  source and the two-source grammar saturates every reached undecided
  state (every §12 verdict "closes", no deviating continuation
  exists). Honest negative: the Slice B enumeration split answers
  30–40× faster at worlds/hands ≈ 3 — the contraction advantage
  remains the ratio, and the opening-root recursion stays not
  attempted. The default player is untouched (CBS-A9). Slice F landed
  in a following round (next entry).
- **The counted-belief build — Slice F landed** *(2026-08-30, the round
  after Slice E)*: the §49 consequence CEGAR
  (`refine_to_action_exact` in `solver/factor_belief.rs`) — §28's
  feature map `κ` as `ClassSignature` (the §49 starting vocabulary:
  critical-tile membership, trump count/highest trump, led-suit count,
  count-tile possession, current-winner/ruff possibility) partitioning
  the acting seat's support at the field-classification bottleneck
  (99% of the opening-root bill, per C2), with the §30 loop aggregating
  action-uniform classes exactly and splitting the largest-mass
  non-uniform class by a WITNESS PAIR whose lowest differing tile
  enters the §31 critical set — termination ≤ 28 refinements because a
  witnessed discriminator is provably outside the critical set. Four
  gates at `tests/solver_factor_consequence.rs`: Theorem 30.1's
  monotone narrowing with nested per-branch intervals `[L_t, U_t]`;
  endpoint parity with `branch_masses` tile for tile; the witness
  requirement re-derived independently (the field itself re-consulted
  on hand-built records); and non-vacuity. Instrument: `factorcegar
  report`, record `walt/probes/factor_belief/cegar_run1.txt` — the
  two-sided finding: MASS CONCENTRATES BUT THE TAIL FRAGMENTS. At the
  opening root under σ0, 513‰ of the 399,072,960-world posterior mass
  is action-exact at 5,387 classes (21 hands/class) and 805‰ at 36,923
  (3 hands/class), with max branch-interval width falling 828‰ → 81‰ of
  Z — §51's success signal ("most posterior mass in action-exact
  classes"); but ZERO residual costs full fragmentation to 116,280
  singleton classes over 15 refinements — §51's falsifier for the last
  slice of mass, and it is a property of the SAMPLED σ0 mind, not the
  vocabulary (trivial-field endpoints on the same roots aggregate:
  255/495, 246/330, 147/495 at trick 4). Read as design guidance, §49's
  own measurement discipline wins: carry small residual as sound
  per-branch intervals (gated to nest, so any budgeted stop is sound);
  don't chase the action-exact endpoint. The instrument pays the full
  per-hand classification bill (5.4 s at the opening; the 16-stage
  refinement loop itself is 183 ms of partition arithmetic) and claims
  representational structure only — the §29 class-verifier interface is
  named, not built. `count_cell` stays deferred: a hand class is a
  one-seat predicate `marginal` counts exactly. The default player is
  untouched (CBS-A9). Slice G landed in a following round (next
  entry).
- **The counted-belief build — Slice G landed, the C→G ladder
  complete** *(2026-08-30, the round after Slice F)*: the §50
  integrated refinement controller (`refine_root` in
  `solver/refine.rs`), unifying the ladder's authorities under Part
  VIII — per legal root action one TYPED interval `[L_a, U_a]` (§42's
  constructor discipline: sampled δ bounds carry their full Slice A
  record, exact bounds carry integer masses over the shared root `Z`),
  the bar `B = max_a L_a`, exclusion exactly when `U_a < B`, permanent
  because lowers only rise and uppers only fall. Work items (the
  buildable §33 subset): `SampledLower`/`SampledUpper` (Slice A's
  frozen-policy witness and optimization-lock upper, each endpoint a
  distinct `ScopedDelta` against a declared root risk scope),
  `ExactFixed`/`ExactGrammar` (the Slice D/E factorized recursions as
  exact lowers), `EscalateExact` (the §36 endpoint: the full-action-set
  recursion `response_success_mass` — §48's fence lifted on §48's own
  sequencing, gated to extensional parity with the bundled exact
  authority `exposure::exact_root_value` at every gated root and
  action, the C→G cross-representation capstone), and
  `ConsequenceCensus` (carried precisely so §34 can refuse it: zero
  declared root-width reduction at every bar). Scheduling is §35 —
  best-case reduction of the declared decision-width scalar
  `D = (|survivors| − 1) + Σ (U_a − B)` per declared integer forecast,
  exact rationals cross-multiplied; budgets charge FORECASTS, never
  wall time, so a run is a pure function of its inputs. Results:
  `Settled` / `Equivalent` (deterministic point intervals at the bar) /
  `Unresolved` (honest surviving set, NAMED fallback, never promoted —
  §37.9), `DeltaQualified` whenever a sampled side was decisive. Four
  gates at `tests/solver_factor_refine.rs` (escalation parity +
  containment; the §37 invariant walked with independent recomputation
  of every exact bound; §34 refusals + bytewise determinism;
  starvation honesty + the δ ledger through
  `assert_screen_risk_allocation`). Instrument: `factorrefine report`,
  record `walt/probes/factor_belief/refine_run1.txt` — findings: the
  exact ladder settles all ten gated roots (six SETTLED, four honest
  exact EQUIVALENT ties), twice WITHOUT escalating the winner (h4-t6,
  h4-t4: the winner's exact-fixed lower cleared every rival's
  escalated point — §36's one-witness promise on trace); the sampled
  tier settles small fibers before ANY exact recursion runs (h4-t6 at
  64 work units against 420 exact-only; h8-t5 at 3,776 against
  13,860), correctly δ-qualified, while at trick 4 its uppers are too
  loose to prune and the exact ladder does the work; and at the
  opening root h0-t1 the controller walks the affordability cliff
  honestly — every exact item refused by its own declared forecast
  (the §40 walls, labeled), fourteen sampled endpoints producing real
  δ intervals over the 399,072,960-world fiber, UNRESOLVED returned
  with all seven actions and risk 7/10 inside the declared 4/5 scope.
  The controller manufactures no bound (§37.8); grammar/residual
  uppers are not wired as a work item (before escalation the only
  nontrivial uppers are the sampled ones); the existing controller
  player remains the fallback surface, and the default player is
  untouched (CBS-A9). The Part XI C→G program of
  `counted_belief_sandwich_v0.1.md` is COMPLETE; beyond it sit the §29
  class verifier, the unbuilt §33 producers, cross-root reuse, and —
  before any default change — arena and conformance gates.
- **The anytime proof-state build — Phases 0 and 2 landed** *(2026-08-31,
  the same day as the intake, on Jason's word — "we go with your
  approach")*: Phase 0 is **freeze 58** (the freeze register's
  2026-08-31 addendum): `solver/refine.rs` as merged at `25b40d9` is
  the semantically frozen **RefineV1** reference — no new fields,
  variants, or work items ever, gates never weaken, the coming
  proof-state core must reproduce it wherever scopes overlap and stays
  removable. The adopted landing shape (Jason's protect-Walt instinct,
  sharpened at the engineering call): in-crate additive modules, never
  a sibling crate — authorities extended only BESIDE themselves behind
  parity gates, protection living in gates-plus-freeze, not in
  Cargo.toml. Phase 2 is `viewer_score_profile` in
  `solver/factor_belief.rs` beside its D/E/G siblings — the §18
  fixed-policy recursion carrying the exact 43-bin declaring-score
  object (viewer-independent, bid-blind: one run = the whole
  bid-threshold curve), walking past the decided cutoff to true
  terminals (§18's caveat; the probe priced the whole curve at ~7–12%
  extra wall at trick 4). Five gates
  (`tests/solver_factor_profile.rs`): conservation + tail projection
  both parities, the tail-sum identity, bid-blind contract reuse
  against independent re-runs, the reuse boundary as a frozen specimen
  (σ0 reads the bid — its settled/desperation branches — so h10-t6's
  projection at 42 gives 12 where the exact evaluation gives 9: under
  a bid-reading field re-pricing is a re-run), and entrywise
  complete-world replay parity. Probe `factorprofile report`
  (`profile_run1.txt`, all ten gated roots × two focals, no drops):
  certain outcomes now carry their explanation (h12-t6's miss = exactly
  20 points in every world, entirely inside the d = 10 rescue band);
  the σ0 make-mass SPIKES exactly at the bid (445‰ at s = 30 on h8-t5
  — the modeled mind's settled branch made visible in the score
  domain); first §10/§11 rescue/fragile band masses on real roots.
  Next on Jason's word: the §49 architecture spike (in-crate
  `solver::proof_state`, open producer registry), then Phase 3
  (contract projection + certified regret).
- **The anytime proof-state build — the §49 spike PASSED** *(2026-08-31,
  the round after Phases 0+2, on Jason's "move on to the next
  step")*: `solver::proof_state` — a persistent, serializable,
  identity-scoped proof state over one root with an OPEN producer
  registry (the deliberate break from RefineV1's closed work-item
  enum). Six gates prove all seven §49 requirements: top-state
  soundness with bytewise serialize/resume (`walt-proof-state-v1`,
  exact rationals, per-fact FNV-1a content hashes re-validated on
  parse); RefineV1's endpoints imported as typed facts — sampled
  endpoints keep their full ScopedDelta provenance, exact
  fixed-policy lowers are executable, grammar/response optima are
  proof-bar-only (the APS-A6 audit finding, now load-bearing) — with
  closure reproducing the controller's survivors, exclusions, bar,
  and typed result on every enumerable root under both ample
  configurations; closure idempotent and insertion-order-independent
  (facts are the ONLY stored authority — installed intervals, bars,
  survivors, results are derived views, per the repo's derived-views
  rule); §51 identity fences (any-coordinate mismatch rejects) and
  malformed-value fences, with full round-trips; §41 closure-aware
  derivation live — a score-profile fact projects to a deterministic
  EXECUTABLE lower, `B_exec ≤ B_proof` asserted inside every closure;
  and the open registry proven literally: a banked-floor structural
  producer (§5: a banked contract makes every continuation) defined
  in the TEST FILE closes a repriced root to the exact
  Equivalent-at-1 tie with no best-response solve and no module edit.
  The §37 assert earned its keep in construction — it rejected a
  contradictory toy fixture in the gate's first draft. Verdict:
  in-crate shape CONFIRMED, zero duplication pressure, module
  deletable (nothing imports it but the crate root). Phase 1
  fleshing and Phase 3 (contract projection + certified regret)
  queued on Jason's word.
- **The anytime proof-state build — Phase 3 landed** *(2026-08-31, the
  round after the spike, on Jason's "go ahead with the next phase")*:
  contract projection and certified regret. The closure carries the
  §31 global upper `U* = max_a U_a` and `Γ = U* − B_exec` (vacuous
  floor at zero executable work); `ProofState::recommend()` derives
  the full §33 block — recommended action and policy, pmake floor,
  global upper, Γ, declaring score floor/ceiling, the §7 residual
  (exactly 0 for exact profiles), the §10/§11 d = 1 bands, proof
  class, sampled-scope summary. Five gates
  (`tests/solver_proof_regret.rs`): exact projection at independently
  recomputed values; regret containment against the bundled authority
  (`Q* ≤ U*`, `0 ≤ Q* − V(π̂) ≤ Γ`) before and after the RefineV1
  import; Γ/U*/B_exec monotone under fact-by-fact refinement; the
  grammar fence (non-executable lowers raise only the proof bar —
  nothing executable, nothing recommended); bid-blind cross-contract
  reuse (the σ0 boundary stays with the profile gates' frozen
  specimen). Probe `proofreport report` (`proofreport_run1.txt`, seven
  roots, 14.1 s): **certified regret ZERO far from certain make**
  (h5-t6: floor = upper = 444‰ — optimality certainty and make
  certainty split, on trace); **§30's gap made flesh** (h3-t4: the
  settled best ACTION is 3-1 at Q = 350‰ while the best MATERIALIZED
  policy starts 4-4 at floor 267‰, Γ = 83‰ — 3-1's naive continuation
  prices below 4-4's, so pmake belongs to the policy, not the first
  tile; the next §33 work item is Phase 6's argmax extraction);
  certain outcomes certify Γ = 0 in both directions with their score
  explanations on the same block. Remaining phases (1's work
  frontier/solve goals, 4+ envelopes and covers) queued on Jason's
  word; default player untouched throughout.
- **The anytime proof-state build — Phase 6 landed** *(2026-08-31,
  same day, on Jason's "phase 6 it is")*: §63 argmax extraction and
  residual policy bounds — the answer to Phase 3's h3-t4 finding.
  `extract_success_policy` returns the §48/§36 optimum WITH one
  policy attaining it (the argmax DAG under the declared
  lowest-tile-index tie rule, history-keyed, on the Slice B decided
  quotient, a total `SlicePolicy` re-priced unchanged by the
  fixed-policy evaluators; content-addressed id — one realizable
  policy, never an envelope); `residual_split` computes the exact
  `(M*, D)` residual pair (empty deviating class = `None`; hidden
  nodes deviate in at least one branch via the cheapest-downgrade
  rule); `solver::extraction::ExtractionProducer` is the first
  shipped ProofProducer — extract, re-price, install, and the
  executable bar meets the proof bar (§30's bridge). Six gates
  (`tests/solver_extraction.rs`), two forced discoveries: RefineV1
  settles on cross-action dominance so a settled root can keep the
  WINNER'S vacuous upper (Γ honestly positive until a §36 upper fact
  prices it — h4-t6 at 2/15), and Slice E's "two-source grammar ties
  free at t5/t6" is structural saturation (≤ 2-tile focal states ⇒
  the deviating class is literally empty). Probe `extractreport
  report` (`extractreport_run1.txt`, 29.2 s): **h3-t4 Γ 83‰ → 0‰
  exactly** — the 12,420-state DAG materializes 3-1's optimal
  continuation, `B_exec` rises 267‰ → 350‰, and the recommendation
  switches from 4-4 to 3-1 under the extracted content id; h8-t5 Γ
  282‰ → 10‰; the two-source grammar ESCAPES on every h3-t4 action
  (the residual proves where trick-4 grammar room is real). Phase 4
  envelope cells and Phase 5 count-threat covers stay unbuilt (the
  exact residual is its own tightest cover); remaining phases queued
  on Jason's word; default player and RefineV1 untouched.
- **The anytime proof-state build — Phase 1 landed** *(2026-08-31,
  same day, on Jason's "phase 1 up next then!")*: the Part IX work
  frontier (the §58 skeleton half was the §49 spike's).
  `solver::frontier`: four typed solve goals with debts in their own
  units (§39's fence — never one scalar), four deterministic work
  items (baseline profile; §36 exact value keeping the §30
  executable/proof-bar split; §63 targeted extraction; the §41 macro
  `ExactValueSurvivors`, load-bearing from the first step — every
  standalone exact upper is provably useless for U* while any other
  upper is vacuous), a declared Z/3Z forecast cost model, and
  `Frontier::advance` — refuse zero-potential (§34 as amended by
  §41), buy best bound-per-cost, install through the fence, assert
  the §42 law per purchase. Six gates
  (`tests/solver_frontier.rs`) including refusal honesty
  (hand-executing a refused item moves exactly nothing — the specimen
  is §39's sentence inverted: exact values are unconditionally
  irrelevant to the profile goal), §43 containment with
  byte-identical deterministic replays, and §44
  resume-equals-uninterrupted. Probe `frontierreport`
  (`frontierreport_run1.txt`, 13.3 s): **goal separation is real
  money** — h10-t6/h3-t5 certify Γ = 0 for 1Z while SelectAction
  costs 7–10Z; **h3-t4 SelectAction settles at 16Z without buying any
  extraction** (only the ε-goal pays for the DAG, then reads 3-1 at
  Γ = 0); h4-t6 SelectAction is 5Z. One honest waste recorded: under
  vacuous uppers the §42 bounds cannot rank extractions, so h3-t4's
  ε-goal spent 28Z where uppers-first pays ~15Z (§43 verbatim: a poor
  forecast wastes, never weakens; bound refinement pricing
  upper-information value = future frontier work). Remaining: Phases
  4/5 (envelopes, covers), 7 (laydowns), 8 (opening-root iterative
  run), all on Jason's word; default player and RefineV1 untouched.
- **x:018's reply** — awaiting Pro; the correspondence itself is indexed in
  §2 above.
- **Exchange 001–015, the informal 014 capture, and the 2026-08-03
  constellation-theory capture**
  (`exchange/informal/2026-08-03-domino-constellations-theory.md` with its
  `.REVIEW.md`, both UNADJUDICATED; the review memo inherits the capture's
  tier) — out of scope here by the fence above; owned by
  [claim-ledger](claim-ledger.md)'s informal-captures section and
  `exchange/README.md`. A retained failed harvest
  (`inbox/010-…FAILED.md`) likewise stays with the ledger's record.
