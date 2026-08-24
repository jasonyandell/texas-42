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
[walt-seat-play](walt-seat-play.md).

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

## 7. The pinned manifests

The `.sha256` files under `walt/math/` are **pinned freeze artifacts** — never
edited, superseded only by append-only re-issue.

Only the four received parents below carry companion `.sha256` files; the
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

## 8. Pending, and deliberately not indexed as landed

- **The field-swap build** — the level-2 field-stability lineage is
  adjudicated (L2-A1..A7) but not yet built: `solver::field` /
  `exposure` / `field_swap` enter after the calculated-evidence shadow
  step merges (L2-A6).
- **The Gran anchor reconstruction** — G1–G4 require the two Plunge game
  seeds and full records, which live on the plunge side and are not in
  this repository. The screenshots remain discovery artifacts. Carded
  as [[gran-anchor-reconstruction]].
- **The calculated-evidence build** — the lineage is adjudicated
  (CE-A1..A8) but not yet applied: `walt/LEVEL2-PROBE.md`'s gate is
  satisfied only when the outer adaptive controller lands with
  conformance gates green ([[adaptive-sampling-intake]] tracks the
  full arc; the CE-A7 build program is the path).
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
