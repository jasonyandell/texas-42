# walt mathematics — received artifacts and intakes

[Home](Home.md) · owns: the first-class index of walt's received mathematical
artifacts — the frozen bases, the Pro-channel intakes and rebriefs, and their
companions — what each is, where the verbatim parent lives, where its companion
lives, what came of it, and at what tier — including received notes that have
NOT been intaken · Sources: `walt/math/` (the artifacts themselves, with their
`.sha256` pins and intake companions), `walt/CENSUS-RULINGS.md` (the
adjudications, through "The focal-horizon adjudication (2026-09-04)"),
`walt/DISCREPANCIES.md` (the two 2026-09 corrections carried in §6 and §8),
`exchange/README.md` (the courier ledger for the ferried dispatches),
`experiments/partnership/packet/` (the manifests of the unintaken notes in §9);
repository state as of 2026-09-07 (`c00717d1`), pins and verifiers re-run
2026-09-12. Related: [the reference map](walt-math-reference.md),
[the freeze register](walt-math-freezes.md),
[decision-sparse witnesses](walt-math-decision-sparse.md),
[the GPU-native track](walt-gpu-native-trick1.md),
[walt-seat-play](walt-seat-play.md),
[walt-calculated-evidence](walt-calculated-evidence.md) (the era the two
2026-08-24 intakes opened),
[walt-counted-belief-era](walt-counted-belief-era.md) and
[walt-focal-horizon-era](walt-focal-horizon-era.md) (where the builds the
intakes authorized are narrated), [exchange](exchange.md) (the courier
channel as a whole).

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
| Decision-sparse (Pro) | `decision_sparse_exact_solving_v0.1.md`, `decision_sparse_second_audit_v0.1.md`, `exchange/inbox/016…`, `exchange/inbox/017…` | `decision_sparse_exact_solving_v0.1_errata.md` | DS-A1..A36, SEP-A, FT-A, SR-A | The witness mathematics — primal/upper witnesses, root-action separation, Theorem E6.3 (adjudicated name "value sandwich") — repaired where unsound, extended two rungs; the ancestor of every later root interval |
| GPU-native (Pro) | `gpu_native_trick1_implementers_guide_v0.2.md`, the M2 and M3 rebriefs | `walt/GPU-NATIVE-TRICK1.md` (v0.3), `-M2.md`, `-M3.md` (frozen contracts) | GT1-A1..A24 | Portable M0/M1 and M2 Metal parity complete; M3 gate frozen, no M3 result |
| Signed-pivotal (side-channel) | `signed_pivotal_geometry_v0.1.md` | `signed_pivotal_geometry_v0.1_intake.md` (intake audit) | SP-A1..A12 | Sound but for one repaired claim; spawned the tilt audit and obligations O12–O19 |
| In-house question/ruling | `WALT-MATH-QUESTION-2026-08-17-…` | `WALT-MATH-RULING-2026-08-17-…` (advisory) | none — advisory, no ruling family | P1–P4 sound; the §12.6A instance stated; the path-dependence counterexample |
| Calculated evidence (side-channel) | `calculated_evidence_v0.1.md` (SHA-256 `9b32b14f…`) | `calculated_evidence_v0.1_intake.md` (intake audit) + `verify_calculated_evidence_v0.1.py` | **CE-A1..A8** (2026-08-24, same-day; the parent already embodied Jason's Pro refinement pass) | Anytime-valid adaptive settlement: 18/18 exact checks PASS; identities SOUND (CE-A1); θ/ϑ split adopted (CE-A2); O20–O28 accepted (CE-A4); §22 is the build program (CE-A7) |
| Targeted level-2 field stability (side-channel, same lineage) | `targeted_level2_field_stability_v0.1.md` (SHA-256 `597d33c3…`) | `targeted_level2_field_stability_v0.1_intake.md` (intake audit) + `verify_targeted_level2_field_stability_v0.1.py` | **L2-A1..A7** (2026-08-24, same-day, under the standing same-lineage go) | First-disagreement localization L2-T1..T5 SOUND, 19/19 exact model-check (~1.57M pair instances); exposure bounds turn level 2 into a calculated refinement; O29–O38 accepted (L2-A2); build slots after CE shadow (L2-A6) |
| Panel response x:019–023 (Pro, adversary panels on both threads) | `exchange/inbox/019-023-response-panel-and-cancellation-v0.1.md` (SHA-256 `a3f468aa…`) | `response_walt_panel_and_cancellation_v0.1_intake.md` + `exchange/inbox/verify_walt_panel_response_v0_1.py` (scratch tier) | **PANEL-A1..A8** (2026-08-24, same-day, standing same-lineage go) | Four briefs certified (020 wording narrowed, 023 with the τ coupling repair); Claim D counterexampled → future-only/preallocated opening binding (PANEL-A3); W7–W11 adopted (PANEL-A5); Part VI cancellation ladder \|c\| ≤ r ≤ d, (B,H,q,g) masses, dominance theorem, directional R± bounds adopted for slice 3 (PANEL-A7/A8); committed lift corrected 41/1200 → **31/1200** |
| Deferred-producers response x:024 (Pro, three-part design brief on the slice-3 deferrals) | `exchange/inbox/024-response-deferred-producers-triple-v0.1.md` (SHA-256 `337296a7…`) | `response_deferred_producers_triple_v0.1_intake.md` + `exchange/inbox/verify_deferred_producers_triple_v0_1.py` (scratch tier) | **TRIPLE-A1..A7** (2026-08-25, same-day, hand-delivered by Jason) | All three deferrals answered at design level: max-preserving upper CS (no Bonferroni; the shipped split-reach count is already S\*_n) for the E3 producer; Hazard-Exclusion Invariant (sound + semantically complete, one verifier authority) for the dominance bound; six-motif first-split morphology + Other for §10 tags, `RevealResponse` refused pending suffix enrichment; branch-mixture upper route retired (wrong orientation); verifier 13/13 PASS incl. exhaustive 65,536-stream sweep; all three producers built with gates same night (slices 4a/4b/4c, main `cbce1ae`) |
| Counted-belief refinement calculus (Pro, side-channel) | `counted_belief_sandwich_v0.1.md` (SHA-256 `4d2dfbe0…`) | `counted_belief_sandwich_v0.1_intake.md` + `verify_counted_belief_sandwich_v0.1.py` (scratch tier) | **CBS-A1..A9** (2026-08-30, same-day, hand-delivered by Jason) | The dual-refinement solver design — root intervals and survivor sets over policy regions and factorized belief: Theorem 5.1 recognized as x:024 M1/M2 over pmake (no new statistics; the pmake sampled optimizer is Slice A's one new producer); seat-factor posterior closure + factorized Bellman adopted (the new mathematics — 116,280 acting-seat hands vs 399,072,960 deals at trick 1); `FiberDp` recognized as the uniform-root `ExactCoverOracle`; §0 probe citations amended (READMEs stay the authority); "sandwich" naming retired for **root interval / survivor set**; verifier 20/20 PASS; build = §56 Slice A + Slice C skeleton, queued on Jason's word |
| Anytime proof-state / score calculus (Pro, side-channel, CBS follow-on) | `anytime_proof_state_score_v0.1.md` (SHA-256 `7a8c60fb…`) | `anytime_proof_state_score_v0.1_intake.md` + `verify_anytime_proof_state_score_v0.1.py` (scratch tier) | **APS-A1..A9** (2026-08-31, same-day, hand-delivered by Jason) | The epistemic container: append-only serializable proof state, facts-only authority, score layer (43-bin profiles, W_ρ(c) steering, rescue/fragile bands), typed laydown hierarchy, proof-bar/executable-bar split with certified regret Γ = U* − B_exec; §49 spike gate before commitment; RefineV1 frozen as reference; executed as Phases 0–8, COMPLETE (see [walt-counted-belief-era](walt-counted-belief-era.md)) |
| **Model-belief base player** (Pro, side-channel — book-one closing round, half one) | `model_belief_base_player_v0.1.md` (SHA-256 `1ffabf86…`) | `model_belief_base_player_v0.1_intake.md` + `verify_model_belief_base_player_v0.1.py` (scratch tier) | **MB-A1..A8** (2026-09-01, same-day, hand-delivered by Jason; build authorized same-day) | The field model becomes hidden state: Ξ = Ω×Θ(×Z) lifts every fixed-field theorem verbatim (Thm 7.1); hand-type factors with exact posterior closure; the ladder demoted to basis (D = `FieldModel::Dice`, F₀ = σ0, F₁ = level-1 — registration corrected at intake, MB-A3); sep upper, type gluing, mandatory residual Other, VOI; verifier 40/40 PASS; build = MB0–MB5 restructure with §76 as the go/no-go |
| **Salvation complex / information-cut calculus** (Pro, side-channel — book-one closing round, half two; arrived mid-adjudication resolving MB-A2) | `salvation_complex_v0.1.md` (SHA-256 `eca69bd5…`) | `salvation_complex_v0.1_intake.md` (**no verifier shipped** — U0's gates are the executable checks, SC-A2) | **SC-A1..A8** (2026-09-01, same session, same standing go) | The unifying fixed-field geometry: 1−Q = minimum belief-mass transversal of the salvation-conflict hypergraph; doom = singleton cuts, gluing = higher-order cuts, column-and-cut solver; God-tightness receiptable as lower-meets-doom-upper; §9's fourteen-coordinate d_info = 0 table verified against committed doomreport per-world truth (h5-t6 444‰ corroborated); the fusion horizon named as the round's key empirical object; build = U0 beside MB0, §47 doom preservation ruling adopted |
| **Focal-horizon hierarchy** (Pro, side-channel — parent title "The Focal-Horizon Sandwich"; a design-with-measurements intake, Jason: "here is an idea to try with measurements") | `focal_horizon_sandwich_v0.1.md` (SHA-256 `892bc343…`) | `focal_horizon_sandwich_v0.1_intake.md` (intake + walt-math review) + `verify_focal_horizon_sandwich_v0.1.py` (scratch tier; its printed "24 CHECK FAMILIES" is a label — the nine sweep families over 4,096 × 8 are the theorem checks) | **FH-A1..A11** (2026-09-04, same-day, hand-delivered by Jason; BRIEF-FH0) | One canonical refinement hierarchy indexed by focal decisions: lawful tail below, world-revealed God tail above, `L_k ≤ Q ≤ U_k` nesting to exact collapse at `k ≥ h_f`; Theorems 1–6 proved in full, God tail proved a Bellman supersolution (FH-God), §23 interruption proved under intersection (FH-int); "sandwich" retired as a name (FH-A2); U0b's ply cuts identified as `U_{a,0}`/`U_{a,1}` on viewer-lead roots (FH-cut) and trick 7's forced layer shown to collapse trick-4 roots at k = 2 (FH-last); the U1 salvation-mask slice subsumed as an upper producer; tails ruled σ0-as-focal primary / lowest-first gate-only; build FH1–FH3 on the orchestrator's split, FH1 affordable-or-refuse first — all built the same day; the finding: at k ≥ 1 the residual width is the tail's policy gap, not the fusion price |
| **Unintaken received notes** (Pro, via the partnership experiment's packet — NOT under `walt/math/`) | `experiments/partnership/packet/texas42-partnership-launch-v0.1/math/TEXAS42-UNIFIED-REVIEW-v0.1.md`, `…/TEXAS42-IMPROVISATION-v0.1.md` (2026-09-05), `experiments/partnership/packet/texas42_relational_learning/PATH-TO-GENERALIZING-SCHEME-POLICIES-v0.1.md` (2026-09-07) — packet manifests only, see §9 | **none** | **none — MISSING intake** | Received text, below even the intaken corpus: no companion, no ruling family, no wiki index other than §9 below; already consumed by `experiments/partnership/` and `walt/scheme/INFORMATION-PRICES.md` at the exploratory-unadjudicated level |

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

The thread that produced the witness mathematics — primal and upper witnesses,
root-action separation, Theorem E6.3 (adjudicated name "value sandwich") and
the lineage that became the root interval, certified regret and the
focal-horizon interval (traced on [decision-sparse witnesses](walt-math-decision-sparse.md)
§ "What Theorems E6.3 and E6.4 became"). Full object index on that page and
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
  first. Rulings are against `ladder.rs` as of commit `171cd22` — then at
  `walt/walt-m3-probe/src/bin/ladder.rs`. **That crate no longer exists on
  main:** the 2026-08-24 unification (commit `d1499d4`, "THE FOLD — one
  crate, seven modules; pure code motion, trace-identical") folded the file
  into `walt/walt/src/bin/ladder.rs`, where its doc header still names the
  2026-08-17 ruling and the pmake objective. The ruling's line citations
  (e.g. `ladder.rs:402` in P1) are against the `171cd22` revision and resolve
  with `git show 171cd22:walt/walt-m3-probe/src/bin/ladder.rs`. Delivers the P1–P4 soundness verdicts
  (decided cutoffs, viewer early exit, pmake key reduction, gcd-normalized
  posteriors), the **path-dependence counterexample** (the exact posterior is
  not a function of the reduced boundary key), the concrete **§12.6A invariance
  lemma** instance with its ECL clauses, the allowance-automaton coarsening,
  and the honest negative on hand 8. Indexed with the scenario era on
  [the reference map](walt-math-reference.md#the-scenario-player-era--signed-pivotal-geometry-the-tilt-audit-and-the-level-2-detector).

## 6. The side-channel intakes, 2026-08-24 → 2026-09-04 — hand-ferried, never courier dispatches

Seven parents in eleven days, each hand-delivered by Jason from his Pro
session, each intaken the day it arrived under one protocol: verbatim parent
with a `.sha256`, a maintained companion that governs where it narrows, a
scratch-tier verifier re-run twice (session evidence, never a receipt —
TRUST-01), same-day rulings, obligations into a ledger. Two exchange
*responses* of the same days — the panel response x:019–023 (PANEL-A1..A8,
2026-08-24) and the deferred-producers response x:024 (TRIPLE-A1..A7,
2026-08-25) — are indexed in the table above with their companions
(`response_walt_panel_and_cancellation_v0.1_intake.md`,
`response_deferred_producers_triple_v0.1_intake.md`) and narrated on
[walt-calculated-evidence](walt-calculated-evidence.md); they are not repeated
here.

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
  reconstruction pending on the plunge side — **a note now stale**: the two
  Plunge hands were transcribed tile by tile from the screenshots,
  mechanically validated with the rules engine and committed 2026-09-04 at
  `walt/probes/gran/` (commits `32aa14f1`, `8174fa83`): **G1 complete** —
  the 28-tile partition closed exactly, 25–17 reproduced; **G2/G3 partial**
  — the made hand's record stops at trick 6, so the deal is not recovered
  but both roots are fully determined as information sets; seed provenance
  unavailable for all three. The companion's gap note itself has **not**
  been repointed at those files — the open item on
  [[gran-anchor-reconstruction]]; narrative on
  [walt-gran-anchors](walt-gran-anchors.md)), and the seven-point
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
  untouched (APS-A9). **What came of it:** Phases 0–8 all landed
  2026-08-31 → 09-01 (Phase 0 = freeze 58; Phase 8's opening-root verdict:
  play 6-5, floor 732‰, at most 267‰ unclaimed, honest UNRESOLVED at
  ε = 1/4 — probe record `walt/probes/factor_belief/openingreport_run1.txt`,
  5 gates), then the doom census — narrated on
  [walt-counted-belief-era](walt-counted-belief-era.md), recorded in
  `walt/FACTOR-BELIEF.md`; pointers and the one correction in §8.

- **`walt/math/model_belief_base_player_v0.1.md`** — *Model-Belief Walt
  and the Closing of the First Book* (received verbatim, hand-delivered
  by Jason 2026-09-01 — "big math intake coming… this next round is about
  intake and collaboration"; checksum-pinned, SHA-256 `1ffabf86…`; not a
  courier dispatch — the ledger is untouched; the upload arrived as
  `THEORYwaltmodelbeliefbaseplayerv0.1.md`, filed snake_case, recorded
  not repaired). Written against main `08fe3d2` (the PR #80 wiki
  first-pass merge). The field model becomes hidden state: the augmented
  latent Ξ = Ω×Θ (×Z under persistent randomness); **Theorem 7.1**, the
  augmented-world reduction — a finite model-belief problem *is* a
  fixed-semantics walt problem on Ξ, so every theorem whose proof used only
  finiteness, lawful focal information, bounded utility and a fixed latent
  distribution lifts verbatim; hand-type factors with exact posterior
  closure (Thm 12.1 — the CBS closure over (H, θ)); the response-vector
  geometry (Thm 17.1 convexity of Q in ν, Thm 18.1 the type-revealed
  "sep" upper, Thm 19.1 zero model-fusion price iff one lawful policy is
  pointwise optimal for every type); type-partition gluing; the mandatory
  residual Other type (§15); value of information; the "gloriously boring
  base player" (§43) with §48's completion criteria as book one's finish
  line; slices M0–M7; obligations MB-O1..O20 and identity invariants
  MB-I1..I10 (Part XII). Thread: primarily L2 — the field model as the
  hidden coordinate; CE machinery consumed only through §20's risk-ledger
  crossing.
- **`walt/math/model_belief_base_player_v0.1_intake.md`** — **the intake
  companion** (maintained) + `verify_model_belief_base_player_v0.1.py`
  (SHA-256 `b5c81fb6…`, scratch tier). Verifier **40/40 PASS** twice at
  intake, upload and filed location (re-run 2026-09-12 on this machine:
  `40 CHECKS / ALL CHECKS PASS`, 0.06 s, no `__pycache__` created).
  Load-bearing sweeps: all 64 Boolean 2-policy × 3-type value matrices ×
  3 priors (point-mass upper dominates the mixture optimum; partition-
  lattice endpoints and refinement monotonicity; convexity of Q in ν; the
  zero-fusion-price biconditional per matrix) and the 256-utility
  transfer-bound sweep. The companion records that several of the 40 are
  **definitional illustrations, not theorem verifications** (one asserts
  `4 == 4`, one `1 − 1 == 0`), so the count is never quoted as forty theorem
  checks. Theorems 7.1, 12.1, 17.1, 18.1, 19.1 and the §13/§15 identities
  step-checked sound. **Adjudicated same-day at MB-A1..A8**
  (`walt/CENSUS-RULINGS.md` § "The model-belief base-player adjudication
  (2026-09-01)"; Jason: "overall, agreed on your rulings", build "full
  go"): the substrate adopted and the best-response ladder demoted from
  ontology to basis — a fixed field is the point-mass case ν = δ_θ
  (MB-A1); the cited-but-absent sibling parent delivered and intaken the
  same session (MB-A2); **the rung registration CORRECTED at intake** —
  the first reading "Dice = σ0" was wrong and retracted: D =
  `FieldModel::Dice`, F₀ = BR(D) = σ0 (the banked-correct level-0 modeled
  mind), F₁ = BR(F₀) = level-1 walt, F₂ = BR(F₁) unbuilt (MB-A3); two
  transcription errata — the §8 display `Q_a(δ_{F_k})` never closes, §34's
  disagreement operator prints as `e` for ≠ — recorded, companion governs
  (MB-A4); the residual Other type mandatory from the first post-MB0 slice
  (MB-A5); model type is genuine semantic hidden state, content-addressed
  identities, one type per declared persistence scope (MB-A6); branch by
  public action never by hidden type — the O34/CBS fence over Θ — and a
  model-belief lower enters the executable bar only with a materialized
  policy (MB-A7); the build restructured M0–M7 → **MB0–MB5** with MB0
  gated by §76 verbatim and **U0 running beside MB0** (MB-A8); MB-O1..O20
  accepted into the Lean side-project ledger (which, as of 2026-09-07,
  exists as no file — §8). **What came of it:** MB0 (2026-09-01, PR #82,
  the §74 exact vertical slice), σ1 repair (PR #83), **MB1** (2026-09-02,
  PR #85 — the model-belief recursion joined to the solver, and the
  model-fusion price Φ_a = U^sep_a − Q_a found **strictly positive at
  trick 4** under the registered F₀/F₁ mixture, ν = (½, ½) per hidden
  seat: the h8-t4 3-1 specimen **38/9600 is gate-pinned** as
  `M6_SPECIMEN = (8_323, 8_361, 9_600)` in
  `walt/walt/tests/solver_model_belief_recursion.rs`; the other rows —
  h8-t4 2-1 47/9600, 3-3 90/9600, 5-5 58/9600, h3-t4 3-1 173/46200,
  4-1 157/92400, 4-4 37/6600, 6-4 7/2200 — are probe record,
  `walt/briefs/MB1-REPORT.md` and
  `walt/probes/factor_belief/modelbelief_recursion_run1.txt`), then UP0
  (PR #86) and UP1a, the unified player consuming both recursions.
  MB2–MB5 unbuilt as of 2026-09-07. Narrative:
  [walt-focal-horizon-era](walt-focal-horizon-era.md); running record
  `walt/FACTOR-BELIEF.md`.

- **`walt/math/salvation_complex_v0.1.md`** — *The Salvation Complex and
  Information-Cut Calculus of Walt* (received verbatim, hand-delivered by
  Jason 2026-09-01 mid-adjudication of the sibling — "oops missed a
  file!" — resolving MB-A2; checksum-pinned, SHA-256 `eca69bd5…`; not a
  courier dispatch; upload `THEORYwaltsalvationcomplexv0.1.md`, filed
  snake_case, recorded not repaired). Written against main `08fe3d2`; its
  stated doom-census hash `eb5a459…` is not in main's history — a
  branch-state hash from Pro's inspection, harmless and recorded (SC-A6);
  the census landed via PR #79. The fixed-field unification: salvation
  sets and the salvation-conflict hypergraph; Thm 6.1 (max-weight face);
  Thm 7.1 (**God-tight** iff nonempty common intersection over saveable
  worlds); the §8 three-part failure decomposition
  1 − V(ρ) = d_phys + d_info + d_policy(ρ); **Thm 12.1: 1 − Q = the
  minimum belief-mass transversal of the salvation-conflict hypergraph**;
  the §13 split 1 − Q = β(D) + τ(H_{≥2}) (doom = singleton cuts,
  information price = higher-order cuts); §14 any verified conflict family
  gives an admissible upper; §15 rational packing as the cheap dual; the
  §23 counterexample ({00,11} vs {01,10}: G neither submodular nor
  supermodular) and the §26 counterexample ({00,01} vs {00,10}: optimizer
  disagreement is not a cut); the fusion-free-suffix hypothesis (§37–38);
  the tower as dynamics on complexes (§41–46); slices U0–U4 / T0–T1;
  obligations SC-O1..O16 and a §60 Lean tranche. Thread: L2 throughout —
  every salvation object is relative to one named fixed field.
- **`walt/math/salvation_complex_v0.1_intake.md`** — **the intake
  companion** (maintained). **No verifier was shipped with this parent** —
  every theorem was walked by hand at intake (6.1, 7.1, 12.1, §13, §14,
  §15, 19.1, §20, both counterexamples re-derived, §31 filtration nesting,
  §44 periodicity), and SC-A2 makes U0's gate suite the executable check.
  The §9 fourteen-coordinate d_info = 0 table verified against the
  **per-world truth** column of
  `walt/probes/factor_belief/doomreport_run1.txt` — all fourteen match;
  h5-t6 gives 1 − 15/27 = 12/27 = 444‰, exactly Phase 3's recorded value;
  Phase 6's "structural saturation" is these roots being God-tight.
  **Correction on the record** (`walt/DISCREPANCIES.md`, "salvation-complex
  intake companion: two divergence points named, three in the record",
  2026-09-02): the companion names two coordinates where the class census
  fell short of truth (h4-t6 0-0: truth 60/90, census 56; h8-t5 5-3: truth
  1/92, census 0); U0's G1 gate found a **third**, h8-t5 0-0 (census 17 of
  21, printed in the record's own recovery column as 809‰). The table is
  unaffected — it cites truth at every coordinate — and the gate
  `walt/walt/tests/solver_godgap.rs::the_section_nine_table_is_re_derived_from_the_committed_record`
  asserts all three by exact value; the companion is a dated record and
  was not rewritten. **Adjudicated same session at SC-A1..A8**
  (`walt/CENSUS-RULINGS.md` § "The salvation-complex adjudication
  (2026-09-01)"): the geometry adopted and the record corroborates it
  (SC-A1); no verifier — the gates *are* the checks, U0 must re-derive the
  §9 table mechanically, §60's Lean tranche accepted into the side-project
  ledger (SC-A2); §47's immediate ruling — do not broaden the doom census
  at the opening root; doom preserved as singleton-conflict producer, exact
  God-upper truth on enumerable roots, suffix-candidate detector, and the
  empty-mask base case of an action-indexed ceiling producer (SC-A3);
  **the fusion horizon is an EMPIRICAL object first** — a theorem may be
  proposed only after adversarial counterexample search; `UnknownGodGap` is
  a distinct result type, and zero certified doom with no exact Q is never
  `PositiveGodGap` (SC-A4); the two fences binding — an optimizer
  disagreement is not a cut, and zero one-glue gain proves nothing, so the
  scheduler must permit short gluing coalitions (SC-A5); provenance notes
  (SC-A6); §45's three-way fact typing across the tower and tie-inertial
  exact selection (SC-A7); the U/T program adopted with **U0 pulled
  forward beside MB0** — §29's two-oracle race, construct a God-tight
  policy vs exhibit a salvation conflict (SC-A8). **What came of it:**
  **U0** (2026-09-02, PR #84 — `solver/godgap.rs`, the §8 decomposition
  made mechanical; on the receipt-root corpus fourteen t5/t6 coordinates
  God-tight, twelve trick-4 coordinates carrying information price 6–22‰
  with d_policy = 0, the opening root typed `UnknownGodGap` on all seven
  actions — record `walt/probes/factor_belief/godgap_run1.txt`, gates
  `walt/walt/tests/solver_godgap.rs`); **U0b** (2026-09-03, the in-solve
  horizon census — inside a trick-4 solve the conditioned trick-5 frontier
  is **not** fusion-free, 13–14‰ mass-weighted at the receipt contract, and
  the trick-6 frontier still flips the root play in 2 of 30 rows; h8-t3
  solved exactly under σ0 for the first time, Q* = 28859/29988 (962‰),
  argmax 1-1, 289,407,472 reads, 14 min 13 s — record
  `walt/probes/factor_belief/horizon_run1.txt`, gates
  `walt/walt/tests/solver_horizon.rs`; the exact value itself is probe
  record and FH3's reproduction, pinned by no gate as of `c00717d1` — a
  search for `28859` over `walt/walt/tests` finds nothing, 2026-09-12).
  The fusion-free-suffix hypothesis therefore stands exactly as SC-A4 typed
  it — empirical — and is false as stated inside trick-4 solves. U1 was
  subsumed as an upper producer by FH Theorem 5 (FH-A5); U2+, T0 and T1
  unbuilt. Narrative: [walt-focal-horizon-era](walt-focal-horizon-era.md).

- **`walt/math/focal_horizon_sandwich_v0.1.md`** — *The Focal-Horizon
  Sandwich: a Canonical Anytime Refinement Calculus for Walt* — the
  parent's title; the object's adjudicated name is the **focal-horizon
  hierarchy** (FH-A2). Received verbatim, hand-delivered by Jason
  2026-09-04 ("here is an idea to try with measurements"; "you're the
  engineer in charge"); checksum-pinned, SHA-256 `892bc343…`; not a
  courier dispatch; upload `DESIGN-walt-focal-horizon-sandwich-v0.1.md`
  with `verify_walt_focal_horizon_sandwich_v0_1.py`, filed snake_case,
  recorded not repaired. Written against merged PR #87 `a80b9829…`
  (UP1a + U0b), exactly main at intake. One canonical refinement
  hierarchy indexed by focal decisions: a lawful tail π below and the
  world-revealed God tail G above, `L_k(B) ≤ Q(B) ≤ U_k(B)` with k the
  number of focal decisions made exact, nesting to exact collapse at
  `k ≥ h_f` (the focal depth); **action intervals** `[L_{a,k}, U_{a,k}]`,
  **bar** `B_k`, **survivor set** `S_k`; Theorems 1–6 (lower and upper
  validity and monotonicity, the interval, finite exact collapse, the
  one-step God upper = the salvation-mask upper `max_a Pr(S_a)`, survivor
  monotonicity); §18's exact-action criterion; §19 certified regret `Γ_k`;
  §22's exact-mass form; §23's interruption rule; §25 continuation
  substitution; gates FH1–FH8; the §38 report of record. Thread:
  fixed-field L2 with no sampling anywhere in the hierarchy; the Ω×Θ lift
  deferred and staying deferred.
- **`walt/math/focal_horizon_sandwich_v0.1_intake.md`** — **the intake
  companion and walt-math review** (maintained; `walt/briefs/BRIEF-FH0.md`)
  + `verify_focal_horizon_sandwich_v0.1.py` (SHA-256 `7700a35e…`, scratch
  tier). Verifier run twice at intake, exit 0 both (re-run 2026-09-12 on
  this machine: `ALL CHECKS PASS`, 0.66 s, no `__pycache__` — it imports
  only `fractions`). **Its printed "24 CHECK FAMILIES" is a literal, never
  a count**: 31 asserts in 18 blocks, of which nine inside the exhaustive
  sweep (4,096 Boolean payoff systems × 8 lower tails on a three-world
  two-layer toy) are theorem checks, two post-sweep specimens are genuine
  strictness witnesses, and seven are illustrations carrying no weight
  (FH-A1). Every Part I empirical citation checked against the committed
  records (`UP1A-REPORT.md` gate UC5, `horizon_run1.txt`, `MB1-REPORT.md`)
  — all MATCH, two restated loosely and sharpened. The companion carries
  **full proofs** (P0–P14) of Theorems 1–6, §18, §19, §22, §23, §25 and
  delivers five propositions, restated in standing form in the rulings
  file: **FH-God** (under a deterministic field the world-revealed
  continuation G is terminal-exact, public-branch harmonic and focally
  optimistic, hence Q ≤ G and U_0 ≡ G); **FH-int** (§23 is sound under the
  INTERSECTION discipline, with a stored policy travelling with every
  lower fact; resume ≡ uninterrupted); **FH-tie** (the survivor set is
  exact iff every survivor has collapsed); **FH-cut** (on viewer-lead
  uniform roots the U0b ply cut at 4m plays equals `U_{a,m−1}`: cut-4 =
  `U_{a,0}`, cut-8 = `U_{a,1}`); **FH-last** (trick 7 forced ⇒ the
  hierarchy is exact at k = 6 − T: trick-4 roots at k = 2, trick-3 at
  k = 3). **Adjudicated same-day at FH-A1..A11** (`walt/CENSUS-RULINGS.md`
  § "The focal-horizon adjudication (2026-09-04)"): intake accepted at
  instrument tier, the verifier's count ruled (FH-A1); **"sandwich" is not
  a citable object name** — hierarchy / interval / action interval / bar /
  survivor set / focal depth adopted (FH-A2); the upper tail admissible by
  FH-God and **the trivial upper 1 is a refusal, never a fact** (FH-A3);
  the lower tail ruled — σ0 driving the viewer seat, its identity
  including the contract, `lowest_first` gate-only (FH-A4); existing
  instruments identified — U0's `GodUpper` IS `U_{a,0}`, `price_node`'s
  upper IS G, Theorem 5 subsumes the queued U1 slice, FH-cut corrects the
  orchestrator's "cut-8 is not `U_{a,1}`" (FH-A5); `h_f` ruled through the
  same `decided_success` predicate the value recursions use, forced focal
  nodes consuming a unit (FH-A6); `TieRule::LowestTileIndex` materializes
  `π_k`, FH5 is the lower-side no-fusion gate (FH-A7); the anchors
  CONFIRMED from `horizon_run1.txt` and **the answer deliberately not
  pinned** — the law already says k = 1 cannot settle 2-1 at h8-t4 bids
  36/39 because `U_{5-5,1} = 757‰ > Q_{2-1} = 750‰` (FH-A8);
  interruption, preserved facts and the suffix identity — which must
  include the posterior itself; record alone is the PiKey defect reborn —
  ruled (FH-A9); non-goals binding, no live default change (FH-A10); FH1
  lands first only as affordable-or-refuse (FH-A11). **No freeze was
  issued** by this lineage; FH-A6/A7/A9 fix identity coordinates carried
  in code — see [the freeze register](walt-math-freezes.md). **What came
  of it, the same day** (branch `walt-fh`, commits `fc171e1f` FH3,
  `8aae7c79` FH4, `b6de5a25` FH5): FH1 (`solver/focal_horizon.rs`, 10
  gates, `walt/probes/factor_belief/focal_run0.txt`), CI1, FH2
  (`solver/focal_ladder.rs`, 9 gates at landing plus a tenth from FH5,
  `focal_ladder_run1.txt`), FH3 (the report of record over 33 (root,
  contract) coordinates × k ≤ 3, `focal_run1.txt`, 4 anchor gates in
  `walt/walt/tests/solver_focal_anchors.rs`), FH4 (independent audit,
  PASS with one vocabulary BLOCK — a gate named with "sandwich" — fixed
  before the PR), FH5 (post-audit fixes). The direction-changing
  measurement (probe record, `walt/briefs/FH3-REPORT.md`): every live
  trick-4 coordinate settles by k ≤ 2 (five at k = 0, six at k = 1 with
  Γ ≤ 45‰, three at the k = 2 collapse); h8-t3 settles only at k = 3
  (survivors 5/5/3/1, Γ 141/100/34/0‰); the two U0b ply-cut flips are
  upper-side artifacts, never certified; and **at k ≥ 1 the residual
  width is the tail's policy gap (Q − L 9–41‰ at trick 4, 12–33‰ at
  trick 3), not the fusion price (U − Q 0–3‰ / 1–2‰)**. Costs as
  findings: 3.82M facts and 19.4 GB peak RSS at h8-t3
  ([[ladder-policy-store]]); `check.sh` 230 s → 308 s
  ([[gate-corpus-trim]]). Jason's ruling the same day: **no new
  mathematical parent until a consolidation slice lands** (`walt/MAP.md`
  § "Next, in order"); the letter back to Pro is a DRAFT at
  `walt/briefs/FH-RESPONSE-TO-PRO.md`, asking three questions — the
  cheapest lawful tail and a tail-improvement ladder; σ0's sufficient
  statistic of the record; the honest guarantee of the live decision at
  tricks 1–3. Narrative: [walt-focal-horizon-era](walt-focal-horizon-era.md).

## 7. The pinned manifests

The `.sha256` files under `walt/math/` are **pinned freeze artifacts** — never
edited, superseded only by append-only re-issue.

Twelve `.sha256` files sit under `walt/math/` as of 2026-09-07: **nine
received parents** carry a companion pin, and three are the GPU source
manifests. The M2 and M3 rebriefs' hashes are **ruling-carried only** (GT1-A10,
GT1-A18). All nine parent pins were re-verified byte-exact on 2026-09-12 on
this machine (`cd walt && shasum -a 256 -c math/<name>.sha256` — the paths
inside the files are relative to `walt/`; nine `OK`, in about a second).

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
| `model_belief_base_player_v0.1.sha256` | The received model-belief parent's byte identity (`1ffabf86…`) | the 2026-09-01 intake (MB-A1) |
| `salvation_complex_v0.1.sha256` | The received salvation-complex parent's byte identity (`eca69bd5…`) | the 2026-09-01 intake, same session (SC-A1) |
| `focal_horizon_sandwich_v0.1.sha256` | The received focal-horizon parent's byte identity (`892bc343…`; re-hashed at intake, matches) | the 2026-09-04 intake (FH-A1) |

The nine parent digests in full, as read from the `.sha256` files and matched
against the parents on 2026-09-12 (verifier digests are in the companions and
are *not* pinned by a `.sha256`):

```
9b32b14ffddbb19af42a4c0ec90edc0bf3d27506ec98c7d5c5058222a1b9e8f8  calculated_evidence_v0.1.md
597d33c3227f7ed4e7d6c9287cfdf0433c2777e59909e38f76543ddcc9509e58  targeted_level2_field_stability_v0.1.md
4d2dfbe0fd9a5fab1d555cda9095ee93f6b82f0755545070008df1cb190eab7c  counted_belief_sandwich_v0.1.md
7a8c60fba5a37f6d8d4451cbf55d478f9d7faaf401b13e724e98c261df9d2e8b  anytime_proof_state_score_v0.1.md
1ffabf86e3d46d81c78627e712ecf8663f5cdf9075d19fca6f33038e7836c1bd  model_belief_base_player_v0.1.md
eca69bd581d9d02466ea6dba29f4ba8fc19bf9e851639fd77d6741fac900604c  salvation_complex_v0.1.md
892bc343f1ada12013b2bbd674d46962bc0256a55170aa48075fea17c2592f04  focal_horizon_sandwich_v0.1.md
b9d93715bf65cc29b2bbb1ce6775d00070449a01ef2fe58f7b3bba2a2b7f9630  signed_pivotal_geometry_v0.1.md
ee2e78da20eb7d087fb121f467a56bafc0179a45fb692ca0b938f4c4210b6a44  gpu_native_trick1_implementers_guide_v0.2.md
```

## 8. Where the builds are recorded, and what is genuinely pending

Until 2026-09-12 this section was a chronological build log — Slices A–G,
Phases 0–8 and the doom census, some 550 lines — that duplicated two owning
records and had started to drift (it still listed the calculated-evidence
build's "step 9 pending" and the field-swap slices as if current, and it
carried one sentence later corrected on the record). The log was collapsed to
pointers; the builds are recorded once each, on the pages and files below.

| Build | Owning wiki page | Running engineering record |
|---|---|---|
| Calculated-evidence §22 program (steps 2–8 landed 2026-08-24; step 9, the level-2 probe as detection layer, became the targeted controller) and the two field-swap slices (`solver::field`, `solver::exposure`, `solver::field_swap`) | [walt-calculated-evidence](walt-calculated-evidence.md) | `walt/probes/step8/`, `walt/probes/fieldswap*/`, `walt/probes/l2_controller/` |
| Counted-belief C→G ladder — Slices A, C0–C2, B, D, E, F, G, all 2026-08-30 | [walt-counted-belief-era](walt-counted-belief-era.md) § "The C→G ladder" | `walt/FACTOR-BELIEF.md`; gates `walt/walt/tests/solver_root_interval.rs`, `solver_factor_belief.rs`, `solver_grammar.rs`, `solver_factor_recursion.rs`, `solver_factor_response.rs`, `solver_factor_consequence.rs`, `solver_factor_refine.rs`; records `walt/probes/root_interval/`, `walt/probes/grammar_residual/`, `walt/probes/factor_belief/{c2,cache,recursion,response,cegar,refine}_run1.txt` |
| Anytime proof-state Phases 0–8, 2026-08-31 → 09-01 (Phase 0 = **freeze 58**, the RefineV1 semantic freeze — [register](walt-math-freezes.md)) | same page § "The anytime proof-state program" | `walt/FACTOR-BELIEF.md`; `solver::{proof_state,extraction,frontier,residual,covers,laydown,opening}`; records `walt/probes/factor_belief/{profile,proofreport,extractreport,frontierreport,bellmanreport,laydownreport,openingreport}_run1.txt` |
| Doom census, 2026-09-01 — the ∀-fail dual of the laydown hierarchy | same page § "The doom census" | `walt/FACTOR-BELIEF.md`; `solver::doom`, 8 gates `walt/walt/tests/solver_doom.rs`; record `walt/probes/factor_belief/doomreport_run1.txt` |
| MB0 / σ1 repair / U0 / MB1 / UP0 / UP1a / U0b (2026-09-01 → 09-03, PRs #82–#87) and FH0–FH5 (2026-09-04) | [walt-focal-horizon-era](walt-focal-horizon-era.md) | `walt/FACTOR-BELIEF.md`, `walt/MAP.md`, `walt/briefs/*-REPORT.md`; records `walt/probes/factor_belief/{modelbelief,modelbelief_recursion,godgap,horizon,unified,focal,focal_ladder}_run*.txt` |
| The Gran anchors G1–G3 and the waking seat's first real hand (2026-09-04), the 6-4 readout (2026-09-05) | [walt-gran-anchors](walt-gran-anchors.md) | `walt/probes/gran/README.md`, `walt/briefs/MORNING-2026-09-05.md` |
| Every instrument by name, invocation and record path | [walt-instruments](walt-instruments.md) | — |

### Two corrections the collapsed log carried, kept here so they are not lost

Both are in `walt/DISCREPANCIES.md` under "Reconciled, not discrepancies"; the
records they concern were not rewritten.

- **The doom-census sentence (2026-09-01), corrected 2026-09-03.** The log's
  doom entry — like the paragraph in `walt/FACTOR-BELIEF.md` it mirrored —
  closed with: the opening root's remaining Γ ≈ 267‰ "is the info-consistency
  price, purchasable by floor work and info-consistency-aware uppers, never by
  counterexample counting." That outran what was established. The salvation
  parent adjudicated the same session (SC-A1) says in its §8–§9 that a zero
  doom census moves only `d_phys` and does **not** by itself show the remaining
  gap is information-consistency price — the unclaimed mass is
  `d_info + d_policy`, and zero doom does not distinguish them; U0 then typed
  the opening root `UnknownGodGap` on all seven actions (SC-A4). **The 267‰ is
  UNKNOWN in its split**: the sampled 512-world optimization lock's looseness
  (the opening upper of 999‰ is that lock, not a doom bound) and the policy gap
  are both live candidates, and U0b's later finding that the residual width at
  trick 4 is mostly policy gap (FH3) points one way without settling the
  opening. What stands from the census: it recovers 809–1000‰ of per-world doom
  truth on enumerable receipt roots and certifies an honest zero at h0-t1
  (probe record `doomreport_run1.txt`; 8 gates); its working domain is the
  endgame and in-play middlegame.
- **The salvation-complex companion counts two truth-vs-census divergence
  points; the record holds three** (h8-t5 0-0 joins h4-t6 0-0 and h8-t5 5-3).
  Nothing above it moves; the gate asserts all three. Details under the SC
  entry in §6.

### Genuinely pending, as of 2026-09-07 (`c00717d1`)

- **x:018, the fee-correlation correspondence** (2026-08-14) — still awaiting
  Pro's reply: no inbox file exists, `exchange/submission_count.txt` reads 24,
  and the last harvested response is 024 (2026-08-25). Every parent since (CE,
  L2, CBS, APS, MB, SC, FH) bypassed the courier ledger by hand delivery, so
  the ledger's row 018 is the last open dispatch. Indexed in §2 above; the
  covering/fractional-covering-dual question it asked may survive in the
  salvation complex's §15 rational packing — unexamined.
- **The three partnership-packet notes** (2026-09-05/07) — received, hashed by
  their own manifests, never intaken: §9 below.
- **The consolidation slice** — Jason, 2026-09-04: no new mathematical parent
  until it lands ("follow through on what we have, then invest in a
  simplification/unification attempt"). Its shape per `walt/MAP.md` § "Next,
  in order": the σ0 read-key study first (does σ0's answer depend on the full
  record? if not the cache key coarsens and every recursion gets 10–100×
  cheaper), then retire `godgap.rs` (933 lines), `horizon.rs` (635) and
  `extraction.rs` (135) as measurement scaffolding around one recursion, with
  `refine.rs` (917, freeze 58) already frozen and removable and `doom.rs`
  (1,048) staying as the God tail's engine — line counts re-measured
  2026-09-12. Not started as of `c00717d1`. The letter to Pro,
  `walt/briefs/FH-RESPONSE-TO-PRO.md`, is a DRAFT and had not been sent.
- **The Lean side-project ledger exists as no file.** Four ruling families
  accept obligations "into the Lean side-project ledger" — CBS-O1..O15 (CBS-A9),
  PS-T1..T15 plus the 42-instance layer (APS-A9), MB-O1..O20 (MB-A8), the SC
  §60 tranche (SC-A2) — but as of 2026-09-07 the identifiers occur only in
  `walt/CENSUS-RULINGS.md`, `walt/FACTOR-BELIEF.md` and this page; nothing
  under `lean/` or on [lean](lean.md) names them, and
  `kanban/backlog/lean-catchup.md` is the only card. Where the ledger lives is
  an open question ([open questions](walt-math-open-questions.md)).
- **Errata §9 and §4.3 still owed** under DS-A28(ii) since 2026-08-14 — the FT
  and SR objects (FT-A27(i), SEP-A2) and Corollary E4.1. Verified unfiled at
  `c00717d1`: the errata's headings end at §8.6. Until the amendment lands,
  `walt/CENSUS-RULINGS.md` is the only authority for those objects
  ([decision-sparse witnesses](walt-math-decision-sparse.md)).
- **The Gran anchors** — records committed and validated 2026-09-04 (G1
  complete; G2/G3 partial, roots determined as information sets only), but the
  L2 intake companion's "Gran-anchor gap" note is not yet repointed at
  `walt/probes/gran/` and seed provenance is unavailable for all three;
  [[gran-anchor-reconstruction]] stays open on those two items. The branches
  the 2026-09-05 readout reported as unmerged (`walt-gran`, `walt-o5`,
  `walt-g1-l2`) are outside this page's scope — see
  [walt-gran-anchors](walt-gran-anchors.md).
- **Cards the builds left open:** [[ladder-policy-store]] (FH2's 3.82M facts
  and 19.4 GB peak at h8-t3), [[gate-corpus-trim]] (`check.sh` at 308 s),
  [[m2-receipt-reearn]] (the standing M2 receipt is old-layout evidence since
  freeze 56 v2), and [[adaptive-sampling-intake]], still in `kanban/doing/`
  although its done-when (parent filed, verified, adjudicated, applied with
  gates) was met on 2026-08-24.

## 9. Unintaken received notes — the partnership packet (MISSING intake)

Three mathematical notes from Pro arrived **outside** `walt/math/` on
2026-09-05 and 2026-09-07, inside the partnership experiment's packet
(`experiments/partnership/packet/`). **None has an intake companion, a
`.sha256` beside it under the `walt/math/` convention, a `CENSUS-RULINGS.md`
family, or a wiki index other than this section.** Their tier is therefore
below even the intaken corpus — EXPLORATORY and **UNADJUDICATED**: received
text, of which this project has checked nothing beyond the byte identities
below. They are indexed here because this page's convention is that every
received parent gets a row, and because code already consumes their ideas:
`walt/scheme/INFORMATION-PRICES.md` ("Finite information-price teacher",
`policy_search::prices`) implements an information-price instrument in the
IMPROVISATION/PATH vocabulary and cites no adjudication — a vocabulary and
tier hazard the book should watch (the notes use "certificate" freely; walt's
D3 rule does not bind received text, but it binds anything built from it).
Digests are the packet's own manifests, re-verified 2026-09-12 on this machine.

| Note | Path | Dated | Manifest digest (SHA-256) | Companions | Status |
|---|---|---|---|---|---|
| *Texas 42: A Unified Mathematical Core — review, corrections, and a falsifiable research program*, v0.1 (809 lines). Claims one response-vector core gives the same object to the belief, continuation, gluing, sampling, score and model-belief work; separates a semantic question (answered yes under explicit assumptions) from a cost question (not established) | `experiments/partnership/packet/texas42-partnership-launch-v0.1/math/TEXAS42-UNIFIED-REVIEW-v0.1.md` | 2026-09-05 | `daae4332685f178c79205c6007747ac5abc70196800220ddd0ac85be9d73f0de` | names `verify_unification.py` and `verification_results.json` — **neither is in the packet** (the SP-A11 precedent for an unfiled import would apply at intake) | **MISSING intake**; "certificate" 10 times |
| *Texas 42: Integrate the Worlds, Compress the Information Price*, v0.1 (528 lines). A capacity-saturated inclusion–exclusion (covering-convolution) exact normalizer for the posterior — it reports all 116,280 acting-hand completion weights matching an independent enumeration of all 399,072,960 legal ordered deals — and a centred-potential information-price upper; states that the broader compression hypothesis "is not established here" | `…/math/TEXAS42-IMPROVISATION-v0.1.md` | 2026-09-05 | `867db550e11c868c3490c0b666ecfd2e0bef9d6675cab9b265d5c99c3e419528` | its checks are described in-text; none filed in the packet | **MISSING intake**; "certificate" 4 times; aimed at the moment-compilation question ([open questions](walt-math-open-questions.md) item 4) and at the same posterior CBS Theorem 20.1 already factorizes |
| *From sampled decision tables to generalizing Scheme policies*, v0.1 (404 lines; inspected `main` at `08fad726`). Regret rather than teacher-label agreement as the training cost for a small lawful controller; information prices bound the alternatives it may have missed; three objects kept separate (actor, teacher/evaluator, information-price program) | `experiments/partnership/packet/texas42_relational_learning/PATH-TO-GENERALIZING-SCHEME-POLICIES-v0.1.md` | 2026-09-07 | `deff51a34075204e0af2240938c03305914d01941d0eb3d51ea21fbb144246e1` (per `manifest.json`) | `verify.py` (`dbb0265cdeb4836519cd08caaa97a9f3c3ecabf5e611e96b1a60ef7bdec400bc`), `results.json` (`60dfdaf96887ca35a326e83b5b0cb0e923b5615e5126a4bff1cc17e115fde8b8`), `partner-count-candidate.scheme` (`a44737b8c271ea2b0ec7acae3174a5cb87b2971f661cd1620c92ae45204e66e0`) | **MISSING intake**; "certificate" 2 times; referenced only by `experiments/partnership/RELATIONAL-LEARNING.md` |

**What was checked, and what was not.** The launch packet's
`MANIFEST.sha256` verifies 6/6 (`shasum -a 256 -c`, run from the packet
directory, 2026-09-12: `EXPERIMENT-BRIEF.md`, `RESULTS-TEMPLATE.md`,
`START-HERE.md`, both math notes, `tools/run_capped.py`); the relational
packet's `manifest.json` entries match `shasum -a 256` of its four files
(2026-09-12); `verify.py` runs under `python3 -I -B` in 0.42 s, exit 0,
printing a JSON block that ends with `constant_action_maximum_regret 1/100`,
and creates no `__pycache__`. That is a session observation of a received
program, never a receipt (TRUST-01), and it verifies the note's own generic
exact-rational examples — not any claim about walt. Nothing in the three notes
has been step-checked, and none of their numbers is quotable anywhere in this
wiki. An intake would follow the §0 convention exactly: verbatim copies under
`walt/math/` with `.sha256` pins, a companion per note, the two missing
UNIFIED-REVIEW companions retrieved or declared unfiled, a scratch-tier re-run,
rulings, and an obligations ledger. Whether to intake them at all is Jason's
call and is blocked in any case by the consolidation ruling of 2026-09-04.

## Out of scope here, by the fence above

- **Exchange 001–015, the informal 014 capture, and the 2026-08-03
  constellation-theory capture**
  (`exchange/informal/2026-08-03-domino-constellations-theory.md` with its
  `.REVIEW.md`, both UNADJUDICATED; the review memo inherits the capture's
  tier) — owned by [claim-ledger](claim-ledger.md)'s informal-captures section
  and `exchange/README.md`. A retained failed harvest (`inbox/010-…FAILED.md`)
  likewise stays with the ledger's record. The exchange as a whole, dispatch
  table 001–024, is the subject of [exchange](exchange.md).
