# walt mathematics — the reference map

[Home](Home.md) · owns: the map of walt's mathematical corpus — every named
object, where its full statement lives, what it binds, and what corrected it ·
Sources: `walt/CENSUS-RULINGS.md` (the adjudication record),
`walt/math/decision_sparse_exact_solving_v0.1_errata.md` (the maintained
mathematics), the received documents under `walt/math/`, the adjudicated
GPU-native portable contract `walt/GPU-NATIVE-TRICK1.md`, the exact M2 rebrief,
the frozen M2/M3 contracts `walt/GPU-NATIVE-TRICK1-M2.md` and
`walt/GPU-NATIVE-TRICK1-M3.md`, the signed-pivotal parent and intake companion,
and the scenario-era documents (`walt/SCENARIO-PLAYER.md`, `walt/TILT-AUDIT.md`,
`walt/LEVEL2-PROBE.md`). Related:
[walt hub](walt.md), [received artifacts and intakes](walt-math-intakes.md),
[structure and transport](walt-math-structure-transport.md),
[walt-seat-play](walt-seat-play.md),
[information geometry](walt-math-information-geometry.md),
[decision-deadness](walt-math-deadness.md),
[decision-sparse witnesses](walt-math-decision-sparse.md),
[the freeze register](walt-math-freezes.md),
[open questions](walt-math-open-questions.md).

> **Epistemic tier: EXPLORATORY — below every tier on
> [Home](Home.md#evidentiary-tiers--never-promoted-never-blurred).** Every
> statement indexed on these pages is exploratory, without exception. A proved
> lemma at the exploratory tier is still exploratory: it is proved *relative to*
> walt's own declared basis (v0.4/v0.5/v0.6, themselves exploratory), and it may
> not be quoted in a brief, a dispatch, [FINDINGS](FINDINGS.md), or any
> claim-tier page. These pages are a map, not a promotion.

> **Coverage (2026-08-24, the [[math-reorg]] pass).** This map now runs through
> the current end of `walt/CENSUS-RULINGS.md`: the S6n fee-correlation chapter
> (FC-A), the full GT1-A1..A24 chapter (freezes 55–57, closed), the
> signed-pivotal intake (SP-A1..A12) and the tilt audit it spawned, the
> freeze-56 v2 re-issue (FZ-A1..A6), and the scenario-player era's mathematics
> (the `walt/SCENARIO-PLAYER.md` spec and its obligations ledger, the pmake
> advisory ruling, the level-2 detector spec). The received artifacts and
> intakes behind all of it have their own first-class index:
> [received artifacts and intakes](walt-math-intakes.md). Still pending, and
> deliberately not pre-indexed: Jason's adaptive-sampling mathematics
> ([[adaptive-sampling-intake]]) — nothing has landed.

## Who this is for

This is the orientation page for the next walt-math adjudicator. It answers four
questions: what documents exist and which one governs; what has been proved and
where the proof lives; what was corrected and by what; and what is open. It
restates nothing — every entry is a pointer with just enough statement to let
you decide whether to open the source.

## The governing documents, and which governs

walt's decision-sparse mathematics lives in three places with three different
disciplines. The GPU-native trick-1 branch adds a received source, a maintained
portable contract, exact accepted M2/M3 rebriefs and frozen binding M2/M3
contracts; the signed-pivotal and scenario-player era adds a checksum-pinned
received parent with an intake companion, a maintained seat spec, and two probe
documents. The distinction is load-bearing and is the first thing to
internalise. The full artifact-by-artifact provenance map is
[received artifacts and intakes](walt-math-intakes.md).

| Document | Discipline | Role |
|---|---|---|
| `walt/math/decision_sparse_exact_solving_v0.1.md` and `walt/math/decision_sparse_second_audit_v0.1.md` | **Received, verbatim, never edited** (DS-A18) | Handed-in documents. Preserved exactly as filed, for the same reason `ingest/` is: a corrected source destroys the record of what was corrected. |
| `walt/math/gpu_native_trick1_implementers_guide_v0.2.md` | **Received, byte-frozen and checksum-gated** (GT1-A1) | GPU-native design input. Original source commit `ca18bc6807b974b31d4640786d7a2d63ae0b79fe`, intake commit `c230949c77ff7e8e22f912ed70f8206488ac9022`, SHA-256 `ee2e78da20eb7d087fb121f467a56bafc0179a45fb692ca0b938f4c4210b6a44`. It remains the record of what was received, not a silently repaired source. |
| `walt/math/decision_sparse_exact_solving_v0.1_errata.md` | **Maintained** (DS-A28(iii)) | The repaired mathematics, with full statements and proofs. Hypotheses may be added and language narrowed *in place*, each change carrying a dated provenance marker naming its ruling. |
| `walt/GPU-NATIVE-TRICK1.md` | **Maintained adjudicated contract v0.3** (GT1-A1) | Binding first-build authority for the GPU-native branch wherever it narrows, repairs or rejects the received v0.2 guide. It does not promote the branch, prove the Rust implementation, or report Metal. |
| `walt/math/gpu_native_trick1_m2_rebrief_v0.1.md` | **Exact accepted M2 rebrief** (GT1-A10) | The mandatory bridge from freeze 55 to M2: 44,079 bytes, SHA-256 `9183132529a42289a104a73d8f7e196eb95058ac2edda60bb42c715f1f8a139a`. It supersedes the old host observation only as current environment status; it does not rewrite that receipt. |
| `walt/GPU-NATIVE-TRICK1-M2.md` | **Frozen binding M2 contract v1** (GT1-A17) | The exact M2 arithmetic/projector parity authority, SHA-256 `aacb6df5e9106b3b6bf00ccfb496c71f762c0fb4644c13a17f76d2ac2f0326e3`. It neither widens the portable parent nor authorizes an action-value, controller, performance or player claim. |
| `walt/CENSUS-RULINGS.md` | **Append-only** (DS-A28(i)) | The adjudication record. No ruling's text is ever rewritten; a corrected clause receives a bracketed dated pointer marker at its site. Also the home of the named lemmas from before the errata existed. |
| `walt/math/gpu_native_trick1_m3_rebrief_v0.1.md` | **Exact M3 rebrief** (GT1-A18) | The mandatory bridge from freeze 56 to M3, SHA-256 `07b3c993…`. The binding contract supersedes it only where more specific. |
| `walt/GPU-NATIVE-TRICK1-M3.md` | **Frozen binding M3 contract v1** (GT1-A24) | The M3 perfect-recall-net gate authority, SHA-256 `79de73e9…`, under **freeze 57**. It authorizes only the gate and records **no M3 result**; the GT1 range is re-frozen closed at A1..A24. |
| `walt/math/signed_pivotal_geometry_v0.1.md` | **Received, verbatim, checksum-pinned** | Pro's signed-pivotal-geometry note (2026-08-18), SHA-256 `b9d93715…`. Audited at SP-A1..A12; SP amendments bind every consumer. |
| `walt/math/signed_pivotal_geometry_v0.1_intake.md` | **Intake companion** | The exact-rational verification of every boxed identity in the parent, which the SP-A audit takes as read. |
| `walt/SCENARIO-PLAYER.md` | **Maintained spec v0.1** | The sampling-stack seat's mathematical specification — definitions and **proof obligations, not established results**; its §10 obligations ledger (O1–O9, O12–O19) is the graduation queue. |
| `walt/TILT-AUDIT.md` | **Maintained probe document** | The E0 experiment as adopted and amended by SP-A1..A12; smoke design, objects, and the 2026-08-19 smoke results. Estimates, never receipts. |
| `walt/LEVEL2-PROBE.md` | **Spec only, deliberately not started** | The level-2 field-swap detector, gated on unification (done) and the adaptive-sampling mathematics (landed 2026-08-24, adjudication pending). |
| `walt/math/calculated_evidence_v0.1.md` | **Received, verbatim, checksum-pinned** | *Calculated Evidence for Unified Walt* (hand-ferried 2026-08-24), SHA-256 `9b32b14f…`. Anytime-valid adaptive settlement (CE-T1..T5), risk ledgers, the information rate `q·D_{1/2}(τ)`, exact-fiber escalation, result typing, O20–O28. **Adjudicated CE-A1..A8 same day**; the §22 sequence is the build program (CE-A7). |
| `walt/math/calculated_evidence_v0.1_intake.md` | **Intake companion** | The exact-rational verification of every boxed identity in the parent (18/18 PASS, three-way on the central closed form; `verify_calculated_evidence_v0.1.py`), vocabulary and O-numbering adjudication notes, verified code boundaries, and the Pro refinement agenda. |

**Citation rule (DS-A17), binding on every design and results file:** cite the
**errata theorem number** for the mathematics and the **DS-A ruling** for its
provenance. Where parent and errata differ, **the errata governs**. The rulings
file remains the adjudication record; it is no longer the home of the repaired
mathematics.

For portable M0/M1, cite v0.3 and GT1-A1..GT1-A9. For M2, cite the frozen M2
contract and GT1-A10..GT1-A17 with freeze 56; cite the exact rebrief when the
bridge from freeze 55 is the fact at issue. The received v0.2 guide is cited by
source identity when the fact at issue is what arrived, never as authority over a
repair. The historical Gate-0 NO-GO receipt remains true about the environment it
recorded and is never rewritten as though M2 had already existed.

A trap worth naming up front: the lemmas proved inside `CENSUS-RULINGS.md`
before the errata existed — Lemmas V, X, **E**, S, S-fold, S-det, R, G, J and
the S-rigid/R-fold corollaries — are a *different naming series* from the
errata's Lemma **E3**, **E4**, **E5.0**, **E7**, **E8** and Theorems **E1**,
**E6.1**–**E6.5**. `Lemma E` (structural isomorphism ⇒ count-free value
equality) has nothing to do with `Lemma E3` (the action-conditioned upper
witness). A naive grep for "Lemma E" conflates them.

## The complete object index

Every named object in the corpus, with the page that owns it. **All exploratory
tier.** "Full statement" names the file and section where the statement and
proof live; the owning page carries the compact form and the caveats.

### Structure, transport, and the quotient — [page](walt-math-structure-transport.md)

| Object | One line | Full statement |
|---|---|---|
| **Lemma V** | A value defined by a node rule reading only actor offset and canonically ordered (increment, successor value) pairs is constant on r3 classes. **Does not cover treatment H.** | `CENSUS-RULINGS.md` § "Fiber-probe rulings", Lemma V |
| **Lemma X** | Zero-contribution excision: worlds with world-informed value 0 contribute 0 to the unnormalised objective under every lawful policy, so deleting them preserves the argmax exactly. **One-sided.** | same file, § "Fiber-refinement rulings", Lemma X |
| **Lemma E** | Equal r1 canonical forms (as amended by F2 A1–A4) ⇒ isomorphic remaining games ⇒ equal count-free value. **Count-free only (E-A2).** | same file, § "Endgame-store rulings", Lemma E |
| **Lemma S** | The seat-side analogue of Lemma E: a seat transport carries every count-free censal question across. | same file, § "Seat-census rulings", Lemma S |
| **Corollary S-rigid** | The first-play seat transport group is **trivial**: the hand form *is* the hand. A proved negative. | same file, § "Seat-census rulings" |
| **Lemma S-fold** | The seven pip declarations fold exactly 7:1 under the unique order isomorphism. The S-A2 conditionality (freeze 18) is **form-level only**: transports of the recorded relational form are reading-dependent, but **values are not** — see Corollary S-fold-val. | same file, § "Seat-census rulings" |
| **Corollary S-fold-val** | Value transport along the declaration fold is **reading-independent** — Q^H, every fixed-policy L and every treatment-C U_a correspond exactly along φ whichever S-A2 reading is adopted. Extends Lemma S-fold and Corollary R-fold. | same file, § "Corollary S-fold-val" (delivered at EC-A4) |
| **Lemma S-det** | The landing state is a function of (δ, H, trick-1 record); a slough proves a void in the led context and nothing else. Determination holds; the alphabet is the raw record space. | same file, § "Seat-census rulings", inside S-Q2 |
| **Corollary R-fold** | Predictive dimension, behavioural-row counts and policy values are declaration-fold invariant. Bases and closure matrices are **not**. | same file, § "Predictive-rank probe rulings" |

### Information geometry and the width ladder — [page](walt-math-information-geometry.md)

| Object | One line | Full statement |
|---|---|---|
| **Lemma R** | Three continuation closures (event ⊇ observation ⊇ value); lawful policy values lie in V^val; and the separating-observation **degeneracy**: *where a complete continuation record determines the latent point* — as it does here — any closure seeded with a nonzero constant has dimension exactly \|X\|. | `CENSUS-RULINGS.md` § "Predictive-rank probe rulings", Lemma R |
| **Lemma G** | Backward Pareto pruning is exact (frontier preserved as a set, ties included), incremental folding is exact and mandatory, exposure is preserved — and N_vec is destroyed by both pruning rules actually in use (narrowed from "every rule" by DS-A26; duplicate-discarding rules preserve it trivially). | same file, § "Policy-geometry probe rulings", Lemma G; clause (6) as narrowed at errata §6, Theorem E6.2(c) |
| **Proposition G-flat** | Grades 1 and 2 carry no policy geometry at all; at grade 3 the only free layer is trick 2, giving N_pol = 2^k(a). | same file, § "Policy-geometry probe rulings" |
| **Definition E9** | The interface-local reachable decision width W^loc_reach(I,a) — the quantity that answers "how many policies must the seat retain *here*". | errata §8.4 |

### Decision-deadness and valuation scope — [page](walt-math-deadness.md)

| Object | One line | Full statement |
|---|---|---|
| **The three-property typing** | forced ⊂ decision-dead ⊂ dominant, both inclusions strict on the S6b evidence. Never fused. | `CENSUS-RULINGS.md` § "Decision-deadness probe rulings" |
| **Lemma J** | Non-interference (NI) ⇒ the node is decision-dead count-free; with the guard H ∩ COUNT = ∅, also under trick-plus-count. | same file, § "Decision-deadness probe rulings", Lemma J |
| **Lemma J(c′)** | The sharp valuation clause: the value is identical for every tile-value schedule **constant on H**, of which the guarded count schedule is the constant-0 case. | errata §8.5(e) |
| **Proposition J-0** | D0 — three bitset tests, exactly sound, **no exhaustion margin exists to get wrong**, because the beater is the led tile of the very trick focal plays to. | `CENSUS-RULINGS.md` § "Decision-deadness probe rulings" |
| **Proposition J-1** | D1-sym — a transposition preserving the still-leadable structure makes the two tiles' values equal; the guard lifts it to trick-plus-count because the transport moves only two zero-point tiles. | same file |
| **Proposition J-win** | D1-win — count-free only, and **this is where the guard fails**: focal winning tricks changes which of the *other* seats' count tiles fall where. | same file |
| **Lemma E8** | The exact valuation scope of J-0/J-1: w constant on the exchanged tiles makes the feature difference vanish, always — and is also necessary in the generic case. Gauge-stable, hence a condition on the valuation class. | errata §8.5 |

### Decision-sparse witnesses — [page](walt-math-decision-sparse.md)

| Object | One line | Full statement |
|---|---|---|
| **Theorem E1** + **Corollaries E1.1–E1.2** | Order exchange under a declared involution Θ. Repairs parent §7.1, which is **unsound as written**. E1.1 is the J-1 instance; E1.2 is count survival under the guard. | errata §1.3, §1.6 |
| **Theorem E1′** | E1 generalised to a fully transported involution (Θ_T, Θ_M, Θ_X), adding continuation equivariance (H4′) and utility invariance (H5′). | errata §8.1 |
| **Definition E2** + **Proposition E2.1** | d_adv as the **affine** dimension; the reference-based form is off by exactly one when the reference lies outside the affine hull. | errata §2 |
| **Lemma E3** + **Remark E3.1** | The action-conditioned upper witness U_a = E_β[V*_a] ≥ Q^H(a). The unconditioned aggregate is valid but **action-constant**, hence vacuous for separation. | errata §3.2–3.3 |
| **Corollary E3.2** | Zero global fusion gap ⇒ U_a = Q^H(a) = V^H for every H-optimal a, and U_a ≤ V^H for all a. At such a coordinate the whole remaining difficulty is primal. | errata §8.2 |
| **Lemma E4** + **Non-theorem E4′** | The primal witness L_a ≤ Q^H(a), and the inversion failure mode with a two-world witness showing the separated action can be **strictly worse** than the rejected one. | errata §4 |
| **Corollary E4.1** | The primal ceiling: an H-argmax-seeded candidate gives L_a = Q^H(a) exactly, for every tie-break; and if Q^H(a⋆) < U_a then **no candidate set whatsoever** separates that pair. | `CENSUS-RULINGS.md` § "Experiment E adjudication"; to be filed as errata §4.3 |
| **Proposition E5** + **Lemma E5.0** + **Corollaries E5.1–E5.2** | The Scheme-mass closure is degenerate in this game (atoms are singletons); (S) holds on the measured carrier; the binding negative is "**atom-mass linear filtering is noncompressive on this carrier**". | errata §5 |
| **Theorems E6.1–E6.5** | The parent's §§4.3, 6.3, 8.3, 8.4, 9.2 restated with their load-bearing hypotheses **inside the statements**: width monotonicity, backward pruning, the value sandwich, root-action separation (member-not-set), finite adaptive gluing. | errata §6 |
| **Lemma E7** | When dominance travels: an exhibited value-order isomorphism α_{Tρ}(Tξ) = α_ρ(ξ), plus a transported belief for belief-relative verdicts. | errata §8.3 |

### The nonanticipativity ladder — FT and SR — [page](walt-decision-sparse.md)

Sixteen objects delivered in one day, adjudicating two received external notes.
**All sixteen live only in `CENSUS-RULINGS.md`**: none has an errata number, so
the DS-A17 citation rule cannot yet be followed for any of them. **Fifteen of the
sixteen are queued for the errata §9 amendment** — Corollary FT-grade4 is named
in none of the queueing rulings. Their compact statements and the caveats that
travel with them are on the track page's two chapters.

| Object | One line | Full statement |
|---|---|---|
| **Lemma FT-arrive** | The arrival law at the *first* frontier is policy-independent, with an explicit inverse-legal-set product form. **Fails at depth two and below** — which is why the ladder is not a decomposition of the value. | `CENSUS-RULINGS.md` § "The fusion tax: inbox 016 adjudicated" |
| **Lemma FT-trunc** + **Corollary FT-grade4** | A suffix of forced decisions truncates the reveal-delay ladder; since the last focal decision is always forced, the last tax is identically zero. At grade 4 the ladder has **exactly two rungs**, so (Δ¹, Δ²) is the complete decomposition. | same section |
| **Proposition FT-flat** | An upper feature independent of the **frontier** action returns a bound at least U^C, so it can never improve the filed witness. The upper twin of Proposition T1-blind. | same section |
| **Proposition FT-tie** | A tied competitor closes **only** if the relaxation is exact — an all-or-nothing threshold. A fence on the reading, filed before the run. | same section |
| **Lemma FT-post** | The frontier posterior is legal-set weighted, **not uniform**. A residual witness priced as a fresh uniform coordinate prices the wrong measure and the composition is **void, not loose**. Two forms compose; nothing else does. | same section |
| **Corollary FT-conv** | Taxes scale with the valuation convention and verdicts do not; a differential tax is exactly twice its count value, and cross-convention comparison is void. | same section |
| **Lemma FT-mix** | Heterogeneous upper witnesses compose: the competitors' witnesses need not share a relaxation, an evaluator, a traversal or a run. Licenses a **mixed** verdict without stretching Theorem E6.4. | same section |
| **Lemma SR-coord** | Theorem 4.1's unnamed hypothesis, discharged for this engine: first-frontier states are distinct **information states** (not histories), so a lawful first-stage policy ranges over a free product; and a second-frontier state has a **unique parent**. | `CENSUS-RULINGS.md` § "The second rung: inbox 017 adjudicated" |
| **Lemma SR-forced** | Gluing a forced decision is free, and the two ladder indexings — by decisions, and by *nontrivial* decisions — agree. Fixes the counted-not-skipped convention. | same section |
| **Proposition SR-sep** | The policy-level minimum separates into the local formula because the first-stage policy ranges over a free product — **not** because of mutual exclusivity and fixed arrival, which do a different job. | same section |
| **Proposition SR-post** | The backward rung recursion is **occupancy-free**: everything below the first frontier enters through the policy-independent lawful posterior, and occupancy enters exactly once, at the first frontier. | same section |
| **Corollary SR-conv** | FT-conv at rung two, with the two bridges kept separate: a *difference* is exactly twice its count value, a *p-weighted value* maps affinely. Collapsing them is the failure mode most likely to look like a discovery. | same section |
| **Proposition SR-degen** | At grade 4 the second rung closes every binding pair **unconditionally**, strictly exactly at the untied ones. **No grade-4 experiment can test closure.** | same section |
| **Proposition SR-taut** | Which depth-two identities cannot fail. The two structural assertions of the received verifier are identities in its own recomputed quantities and hold for any input whatsoever — arithmetic remarks, never receipts. | same section |
| **Proposition SR-loc** | Escape is exactly where the safety inequality is strict, so the escape set is precisely the support of a naive witness's error and the error is the sum over that support **and nothing else**. | same section, closing note |
| **Proposition FF-blind** | An **action-blind fee** removes exactly zero fusion value, at every state and for every coefficient. The penalty-side twin of FT-flat and T1-blind. | `CENSUS-RULINGS.md` § "The feature-fee audition: Jason's control feature, specified" |
| **Lemma FF-min** | The fee objective is convex piecewise-linear, bounded below, and **exactly minimisable** by enumerating breakpoints — no grid, no search, no float. Supplies two genuine receipts, since the swept minimum and the filed tax are different computations. | same section |
| **Proposition FF-oracle** | Per-state fees bound shared fees, so **a low capture refutes conclusively and a high capture establishes nothing** about a usable family. Forces the name **oracle-θ capture** on every such column. | same section |
| **Proposition FF-degen** | **Zero breakpoints is exactly vacuity.** A zero capture with many breakpoints is a measurement; the same zero with none is a tautology. The diagnostic that made a defect catchable from a committed artifact. | same section, first closing note |
| **Proposition FF-corr** | Exactly what a fee bites on: capture is zero iff zero minimises the objective, which for unique clairvoyant argmax means the centred feature has zero mean along the clairvoyant policy. | same section, second closing note |
| **Proposition FC-drop** | The quantitative form of FF-corr: **capture is at least correlation times reach** — one directional slope times one breakpoint distance, computable with **no minimisation**. A **lower** bound at every state, so a large value proves a fee bites and a small value proves nothing. | `CENSUS-RULINGS.md` § "The fee-correlation chapter: what a fee bites on, measured" |
| **Corollary FC-null** | An action-blind feature has exactly zero correlation, recovering FF-blind and giving the diagnostic a null control whose value is fixed **by theorem** rather than by a filed number. | same section |
| **Proposition FC-width** | The subgradient's width is exactly the mass-weighted spread of the feature **across the clairvoyant tie**. Without ties it is a point, so zero capture needs an exact identity; with ties it has positive width and zero capture is **robust rather than coincidental**. | same section, closing note |
| **Proposition FC-tight** | The drop bound is **attained** exactly when the descent is a single linear piece. It is never *exact* as a property of the functional anywhere, and **which states attain it is not knowable without the captured amount** — so attainment is a fact about the gap's distribution, never a usable property of the instrument. | same section, closing note |

### GPU-native trick-1 portable foundation, M2 Metal parity, and the M3 gate — [page](walt-gpu-native-trick1.md)

| Object | One line | Full statement |
|---|---|---|
| **`OpeningRootV1`** | The only accepted first slice: focal = bidder = leader = actor, seven known tiles, empty public prefix, complete ordered hidden 7/7/7 support, derived point/mark loss budget, and closed evidence/prior/field/utility/horizon profiles. It is not a generic public-state API. | `GPU-NATIVE-TRICK1.md` §§2–4; GT1-A2/A3 |
| **Opening-response projector** | Exact `(response,e)` cells with separately typed `A`, `C` and `W=A*C`; exact `m=0..6` cell counts, hard maximum 11,730, and total scaled mass `399072960*420^3`. Same-context payload reuse never merges physical action identity. | same file §5; GT1-A4 |
| **Reduced parity carrier** | Complete direct-world parity at grades 2–4 below the 100,000-world cap; grade 5 is a 756,756-world preflight declared stop with zero partial output. This is a correctness carrier, never opening evidence. | same file §9 M1; GT1-A5 |
| **Portable receipt boundary** | Persist only a root/action/profile/table/freeze/build-bound envelope or a fully bound grade-5 stop. Raw projector payloads are non-persistable; validators canonicalize and fail closed. | GT1-A6 and freeze 55 |
| **Lean GT1 foundation** | Kernel proofs for the legal loss bound, live legal-set/420 facts, numeric widths, current-trick-aware unbanked points, exact cell counts and the stable interval algebra. Projector refinement, posterior factorization, information-key equivalence, Rust/Lean correspondence and Metal/Rust correspondence remain explicit debt. | `lean/Texas42/Trick1Foundation.lean`; GT1-A8 |
| **`U256MetalCorpusV1` and `OpeningChooseTableV1`** | A fixed 16,384-case edge/SplitMix arithmetic corpus with an independent BigUint oracle, plus a separately identified 22-by-22 extraction checked entrywise against unchanged `SemanticTablesCanonicalV2`. These are exact arithmetic/table evidence, not game values. | `GPU-NATIVE-TRICK1-M2.md` §§3–4; GT1-A12 |
| **`M2OpeningParityCarrierV1`** | The complete ordered Reduced, GradeMatching and SameContextPair carrier. One official work unit is one validated opening context and command buffer; physical-action binding remains distinct from reusable context payload. The complete official run has 614 tasks. | same file §§5–7; GT1-A11/A13 |
| **`M2SequentialRunnerV1`** | Completion-only reads, fixed arenas, stable host compaction, typed progress frames, separate CPU/command watchdogs and zero accepted evidence after any failure. It deliberately admits no atomics, concurrency, adaptive scheduling or performance conclusion. | same file §§6–10; GT1-A13..A16 |
| **M2 receipt boundary** | Two fresh complete Metal runs must produce one another's bytes and the immutable committed comparand exactly. The receipt is executable evidence under freeze 56, never a Lean theorem or a reusable projector value. | `walt/receipts/gpu_native_trick1_m2_v1/`; GT1-A16/A17 |
| **Lean M2 finite foundation** | Kernel proofs for the fixed arena bounds, GradeMatching coverage and count, at-most-ten matching-vector bound, stable-filter order, and all-or-nothing acceptance. The projector formulas and Rust/Lean and Metal/Rust correspondence remain proof debt. | `lean/Texas42/Trick1MetalFoundation.lean`; GT1-A17 |
| **M3 carrier and claim fence** | One grade-4 h8 carrier immediately before trick 4, uniform over the exact 1,200 compatible worlds, roots 21/31/33/55, objectives M3A (future-trick differential, strict `C > H` required) and M3B (P30 make), treatments H lawful-perfect-recall vs C world-revealed. The sole green sentence is the gate sentence; no trick-1 value, lead, performance, controller or player claim. | `GPU-NATIVE-TRICK1-M3.md`; GT1-A19 |
| **`M3PerfectRecallKeyV1` / `M3WorldRevealedKeyV1`** | The scoped S1 observation with complete own action-observation memory and no hidden-world identity, vs the disjoint world-revealed C type. Unique parent/action, complete face retention, sum-before-max license the exact unnormalized recurrence; no strategy fusion, key renormalization or cross-world C pooling admitted. | same file; GT1-A20 |
| **The two-family reduction algebra** | Exactly two noncoexisting REDUCE families (`MASS_BUCKET`, `BACKWARD_VALUE`); conservation and terminal-bucket checks are disjoint host folds, never a third family; count-one retirement, real epoch order, the 21-level/range proof and closed command/frame/byte caps. | same file; GT1-A21 |
| **M3 evidence, controls and reproducibility** | Sole-owner semantic streams rendered independently on CPU and post-Metal; the 36-control registry; closed receipt grammars with no partial salvage; two fresh builds and two fresh runs must produce byte-identical library and receipt bytes. | same file; GT1-A22 |
| **Lean M3 proof boundary** | `Texas42.Trick1PerfectRecallNet` must build and pass the axiom audit (codec/scoping, replay, unique parent, complete face, sum-before-max, mass, objective bridges, two-family census/range/compaction/cap proofs, all-or-nothing composition). Rust-to-Lean, Metal-to-Rust, general oracle correctness and grade-4-to-trick-1 transport remain named correspondence debt. | GT1-A23 |

### The scenario-player era — signed pivotal geometry, the tilt audit, and the level-2 detector

The era's mathematics lives in four documents rather than one probe chapter:
the received signed-pivotal parent and its intake companion, the
`walt/SCENARIO-PLAYER.md` spec, the pmake advisory ruling, and the two probe
documents (`walt/TILT-AUDIT.md`, `walt/LEVEL2-PROBE.md`). Artifact provenance
is on [received artifacts and intakes](walt-math-intakes.md); the track
narrative is on [walt-seat-play](walt-seat-play.md). **A status column is
mandatory here and nowhere else on this page**, because this era's named
objects are *mostly unproved*: the spec is explicit that its statements are
definitions and proof obligations, not established results, and each obligation
carries a ledger row.

| Object | One line | Status | Full statement |
|---|---|---|---|
| **The signed-pivotal boxed identities** | g = qτ; E[Y²] = q; Var(Y) = q − g²; H = 1/(qτ²) − 1; the world/tape projection; strata linearity g = Σ_j w_j μ_j; the cover identity w²·Var(Y\|P) = wq − g², hence H_P = w/(qτ²) − 1. | Verified by hand and on 2,000 random exact-rational instances at intake; SOUND at SP-A audit | `walt/math/signed_pivotal_geometry_v0.1.md` §§2, 4, 5, 6; intake companion; SP-A headline |
| **The SP-A5 paired-variance repair** | Var(Y) = Var(u_a) + Var(u_b) − 2·Cov(u_a,u_b): pairing is sharper **iff Cov > 0**. The parent's §2.1 "strictly sharper" is the corpus's one general claim FALSE as written; its own Case C is the counterexample and stays in the parent. | Ruled; binds every consumer | `CENSUS-RULINGS.md` § "Signed-pivotal intake adjudication", SP-A5 |
| **The adopted vocabulary** | **Pivotal mass** q, **tilt** τ, **gap** g, **fixed-pair hardness** H, **scenario** ξ = (ω, r), **panel**; **pivotal cover** (never "envelope"), **pivotal win share** (never bare θ), **frozen policy** (never "plan"). | Ruled | SP-A1..A4 |
| **SP-A6 tape typing** | The tape r is a seed assigned world-independently; the scenario law is the product law. Walt's single-stream derivation does **not** satisfy the split, so d(ω)/s(ω) estimates are undefined artifacts until it exists. | Ruled; the split is unbuilt | SP-A6 |
| **The no-tape structural finding** | The level-0 mind is a pure function of (seat, hand, record) — under the current field model a scenario IS a world, every world is tape-stable by construction, and Phase E is vacuous until a stochastic field model exists. | Smoke finding (2026-08-19), a measurement about the code, not a lemma | `walt/TILT-AUDIT.md` § "Smoke results" |
| **The spec's object definitions** | Public record R (2.1), information state I_s (2.2), solver key κ(R) (2.3), PiKey (3.1), level-0/level-k minds (3.2/3.3), seed discipline (3.6), fiber (4.1), outer sampler with its inline sampler-correctness obligation (4.2), the declared no-void inner simplification (4.3), level-k walt (6.1), objective and decided cutoffs (6.2). Plus two load-bearing remarks: **2.5** — banked is NOT a function of (played, leader, plays), the mathematical content of the PiKey defect; **4.4** — beliefs are lawfulness-only today, behavior-Bayes happens only on modeled continuations. **Numbering caveat (2026-08-24):** the sampler-correctness lemma is *unnumbered* in the §4 body (it sits inside Def 4.2) but the §10 ledger's O3 row cites it as "Lemma 4.2" — both readings are indexed here; neither text is edited. | Definitions; the sampler-correctness lemma is Obligation O3, the no-void cost Obligation O5 | `walt/SCENARIO-PLAYER.md` §§2–6 |
| **Lemma 2.4 (key sufficiency)** | Under the Boolean pmake objective with fixed dcl and b, the continuation value depends on R only through the reduced key κ(R) plus the alive set. | **OBLIGATION O2 — unproved on paper**, heavily exercised | `walt/SCENARIO-PLAYER.md` §2 |
| **Def 3.4 (cache purity invariant)** | Every π value cached under (k, PiKey) must be a pure function of that key — which requires the key to carry everything the computation reads, banked totals included. Carries the PiKey defect record (the documented invariant the code violated for a day). | Invariant + receipt on `f5fff91`; never an axiom | same file §3 |
| **Def 3.5 (Dice field / the tickertape)** | Field randomness keyed on the *record*, not the path: the same world at the same record plays the same tile in every branch, so worlds partition by drawn move instead of multiplying branches. | Definition | same file §3 |
| **Lemma 5.1 (conservation)** | The move buckets partition the alive set exactly. | Asserted at every node in every run | same file §5 |
| **Lemma 5.2 (posterior semantics)** | The bucket weight is the posterior probability of the bucket given the modeled move; the root value is the best-response expectation under "field seats play their modeled policies". | **OBLIGATION O4 — the load-bearing one**, unproved | same file §5 |
| **Lemma 5.3 (support safety at the bottom)** | Under Dice every legal move of every world has positive probability, so the level-0 bottom excludes no lawful world; deterministic higher levels refine support intentionally per 5.2. | Argued in place | same file §5 |
| **Def 6.3 (tie protocol)** | Saturation ties are never broken by tile index: tied candidates re-evaluate on fresh 4× samples until separated or bounded — support ≠ belief, and 1-on-sample is not certainty. | Definition; bias question is Obligation O8 | same file §6 |
| **Claim 8.1 (execution-order invariance)** | Cache purity + exact rationals + fixed argmax order + partition semantics ⇒ results invariant under any thread count and interleaving; only work statistics vary. | **Theorem-shaped claim, OBLIGATION O7**; byte-identical 1-vs-18-thread receipt | same file §8 |
| **The obligations ledger** | O1–O9 (spec-native: no-strategy-fusion, key sufficiency, sampler correctness, posterior semantics, no-void cost, sampling error, order invariance, tie bias, bid generalization) and O12–O19 (filed from the signed-pivotal §14; O10–O11 permanently retired, SP-A11). | The graduation queue; nothing graduates by existing | same file §10 |
| **The pmake verdicts P1–P4** | Decided cutoffs SOUND (with the totality bonus: at any terminal T1 + T0 = 42 forces a cutoff, so the recursion is total with no explicit terminal case); viewer early exit SOUND; pmake key reduction SOUND (and unsound for trick-differential, as claimed); gcd-normalized projective posteriors SOUND-WITH-CAVEAT (fail-closed overflow), exact by the projective lemma V(R, c·w) = V(R, w) for integer c ≥ 1. | Advisory, recorded outside the rulings file; against `ladder.rs` at 171cd22; no ruling family | `walt/math/WALT-MATH-RULING-2026-08-17-pmake-and-the-walk-to-trick-1.md` |
| **The path-dependence counterexample** | The exact posterior is a function of the full ordered record but NOT of the reduced boundary key: same reduced key, odds 1:1 on one trick order vs 3:4 on the other. The exact-posterior key is not redundant. | Advisory, concrete counterexample verified in the ruling | same file, "Bonus" |
| **The §12.6A invariance-lemma instance** | The concrete ECL instance the ladder should quotient by: (π, θ) fixing played tiles and the focal hand, transporting contexts, preserving trick-key order and count, gives V and Q preserved tile-for-tile — the v0.5 theorem's tile-feature role re-entry form. Honest negative: on the hand-8 carrier the group is provably almost trivial. | Advisory; proof shape stated, group computation hand-checkable | same file, §Q2; parent `walt/math/equivariant_lumpability_v0.5.md` |
| **The allowance-automaton coarsening** | Under pmake the banked pair collapses to the T0-allowance state a = 12 − banked_T0 ∈ {0..12} ∪ {busted}; one ≤14-valued coordinate where trick-differential needs the exact tally pair. | Advisory | same file, §Q2 close |
| **The minimal-sufficient-statistic negative** | Under a uniform-random field, worlds act only through hidden legal sets and distinct posteriors at equal public reductions have genuinely distinct continuation laws — the posterior is the minimal sufficient statistic, and **no coarser belief relation should be hunted**. The belief-class count is the irreducible size. | Advisory; the standing negative that shapes the cell-representation recommendation | same file, §Q2 close |
| **The racing instruments** | Three named modes from the tilt smoke: **replay-race** (frozen policies on a common panel — the audit-faithful object, replay ≈ re-solve without extraction), **block-race** (`level1_raced`, CRN blocks with paired sign-test elimination), **race-then-refine** (`level1_race_refined`, wired opt-in). Regime-dependent: pays off where evaluation is expensive and candidates separate; fixed-bid-30 self-play is its worst case. | Smoke measurements and engineering verdicts, never receipts | `walt/TILT-AUDIT.md` §§ "Racing", "Arena gate" |
| **The level-2 detector** | A decision is level-2-relevant exactly where pivotal mass wakes up under a field upgrade: q(level-0 field) ≈ 0 but q(level-1 field) > 0. Companion hypothesis: modeling the partner's response should grow g and drop H — level 2 may be *cheaper to sample* at equal confidence. | SPEC only, deliberately not started; gated on [[adaptive-sampling-intake]] | `walt/LEVEL2-PROBE.md` |

### Inline mathematics outside the probe chapters

Named or boxed mathematics that lives in a basis document, a design document,
or an era page rather than a ruling chapter. Rows marked **orphan** have no
ruling-ID home anywhere — they are indexed here so they stop being findable
only by accident, and indexing confers nothing.

| Object | One line | Home | Full statement |
|---|---|---|---|
| **The §12.6A theorem and condition (ECL)** | Jason's equivariant-controlled-lumpability theorem over declared role interfaces: the lossless count-free equivariant quotient, with role re-entry of tile features (count, trick-key order) as the lawful route back for non-count-free payoffs. | The v0.5 basis; concretely instantiated at the pmake ruling's Q2 lemma | `walt/math/equivariant_lumpability_v0.5.md`; canonical prose form at [the factory era](walt-factory-era.md) § "S5d — the re-tethering". **Dated caveat (2026-08-24):** two pages paraphrase it imprecisely — [walt-program](walt-program.md) §3 drops count-freeness and the legal-set/kernel conditions, and [the census era](walt-census-era.md) § "What the era left" reverses the gauge ordering (the §8 additive gauge acts only *after* the role-indexed valuation interface is declared). The v0.5 source and the factory-era prose govern; neither page is edited by this index. |
| **Walt's Appendix A reader notes** | Two walt-authored caveats on the v0.5 basis: coherence scope over the transports, and the abstract-policy-class optimization boundary. | **Orphan** — no ruling home | [the factory era](walt-factory-era.md) § "S5d — the re-tethering" |
| **The v0.4 objects as a user guide** | Purpose-soundness D(x) = D(y) ⇒ R*(x) = R*(y) (R* = R̄ ∘ D); strong controlled lumpability's two conditions; the Scheme/Fix grammar (Σ = (N_Q, N_C, N_D), O ⊆ Σ, S = (π, φ), F = S₁ ∨ … ∨ S_r, empty Fix false); §12.7's six conditions and its boxed three-part deliverable. | The v0.4 basis (its own §17 claim ledger governs status) | `walt/math/unified_information_geometry_v0.4.md`; worked restatement at [walt-scheme-fix](walt-scheme-fix.md) §§2–3, 7 |
| **The slack identity** | a⋆ separates iff g(a⋆, seed) ≤ s(a⋆), with the economy gap g := Q^H(a⋆) − L^seed(a⋆) and the separation slack s := Q^H(a⋆) − max_{a≠a⋆} U_a, proved in the design. | EC-A3 ("EC-Q3: R8 CONFIRMED"). Collision hazard: an unrelated receipt also named (R8) sits in the N4 section — the EC-A3 citation governs here | `walt/ECONOMY-SUCCESSOR.md` §1.2 |
| **Definition (walk-step)** | One unit charged as `bag.len()` at each `walk` entry — one unit per (particle, node) visit, deliberately the same unit as the scalar authority's particle-step. | Freeze 44 ([register](walt-math-freezes.md)) | `walt/SEPARATION-RUNG-N4.md` §3.1 |
| **The convention bridge Q_diff = 2·Q_count − grade** | The exact affine bridge between the differential and count conventions, asserted at reporting boundaries only; generalised in the rulings as the case α = 2, c = −grade. | Freeze 26 content, ratified at SEP-A3(iii)/SEP-A8 | [the register](walt-math-freezes.md), row 26; restated in `walt/SEPARATION-PROBE.md` and `walt/SEPARATION-RUNG-N4.md` |
| **The cardinality ladder** | N_pol ≥ N_vec ≥ N_par ≥ W_all ≥ W_reach ≥ 1, with forced ⊂ decision-dead (N_vec = 1) ⊂ dominant (N_par = 1) and the W_all = N_exp identification. | Owned by the information-geometry page | [information geometry](walt-math-information-geometry.md) § "The cardinality ladder" |
| **The rank reconciliation** | v0.4 §1.3's pip-sum order and the rules corpus's off-pip ranking are the same order: ranks compare only inside one tier, a tier fixes one context q, and pip_sum = q + off_pip is monotone in off_pip; with the doubles-sentinel argument. | **Orphan** — argued in place, no ruling ID | `walt/DISCREPANCIES.md` § "Rank of a mixed tile" |
| **The `q_points` class definition** | An exact PI root value vector under the real scoring differential — each trick worth ±(1 + count points of its four tiles), focal minus opponents. | **Orphan** — definition in place, no ruling ID | `walt/DISCREPANCIES.md` § "exp5 census pins"; restated at [the foundation era](walt-foundation-era.md) § "S3.5" |
| **The decisive-tile rule** | Decisive tile = the viewer tile whose led context touches the most hidden-pool tiles, ties to the higher tile. Explicitly typed **a choice, not a theorem**. | **Orphan** — declared in place, no ruling ID | `walt/DISCREPANCIES.md` § "exp3A descriptor pin"; [walt-scheme-fix](walt-scheme-fix.md) §5 rule 1 |

## The ruling families

Twenty-six indexed families or ruling series, one adjudicator (walt-math), all
exploratory. Ranges below are
**ruling-ID ranges**, which are append-only and do not move; the sections are
located by heading, never by line number, because line numbers drift with every
append. Three families span more than one section heading — `DS-A` runs across
three, `N4-A` across two, `GT1-A` across three — and the family, never the
heading, is what inherits.

| Family | Range | Section | Date | Scope |
|---|---|---|---|---|
| `F1`–`F7` + "Extra item" | F1..F7 | Census fork rulings | 2026-08-10 | The census fork: carrier, invariant list, transports by canonicalization, probability model, primitive-step granularity, quotable statistics, the **NO-RESCUE failure protocol (F7)**, and the empty output interface. |
| r3 `Q1`–`Q5` | Q1..Q5 | r3 — retrograde coarsest quotient | 2026-08-10 | The coarsest equivariantly-lumpable quotient: soundness and coarsestness, successor-class equality, the signature tuple, intrinsic vs carrier-relative reading. |
| `Y1`–`Y3` | Y1..Y3 | The railyard factoring | 2026-08-10 | Level = tricks remaining: one-trick contract and stacking, periodicity split into obligation vs measurement, the pruning operator. |
| `Definition`/`Q2`/`Q3` | — | Shape notion v2 | 2026-08-10 | The repaired instrument: the depth-*d* suffix library in two variants, the refutation criterion, the hereditary-shape rung. |
| `P-A` | P-A1..**P-A21** | Fiber-probe rulings | 2026-08-11 | The fiber probe. Carries **Lemma V**. |
| `X-A` | X-A1..**X-A19** | Fiber-refinement rulings | 2026-08-11 | Declared exclusion remnants. Carries **Lemma X** and the support/belief/exclusion typing. |
| `E-A` | E-A1..**E-A21** | Endgame-store rulings | 2026-08-11 | The symmetry-reduced tablebase. Carries **Lemma E** and **E-A2**, the count-free scope limit. |
| `S-A` | S-A1..**S-A21** | Seat-census rulings | 2026-08-11 | The seat-level census, counts only. Carries **Lemma S**, **S-rigid**, **S-fold**, **S-det**. |
| `R-A` | R-A1..**R-A24** | Predictive-rank probe rulings | 2026-08-12 | Predictive dimension. Carries the v0.6 proof audit, **Lemma R**, **R-fold**, and **R-A18**, the correctness gate. |
| `PG-A` | PG-A1..**PG-A18** | Policy-geometry probe rulings | 2026-08-12 | Policy counts. Carries **Proposition G-flat**, **Lemma G**, and **PG-A13**, the stop discipline. |
| `J-A` | J-A1..**J-A18** | Decision-deadness probe rulings | 2026-08-12 | Deadness detectors. Carries **Lemma J**, **J-0**, **J-1**, **J-win**. |
| `DS-A` | DS-A1..**DS-A36** | Three sections: Decision-sparse intake audit; Second-audit adjudication; S6c runner | 2026-08-13 | One continuously numbered family. Intake audit of the received v0.1 document (A1–A18); adjudication of the second audit (A19–A28, including **DS-A28**, the append-only protocol); execution scheduling and persistence (A29–A36). |
| `SEP-A` | SEP-A1..**SEP-A19** | Experiment E adjudication: the separation probe | 2026-08-13 | Root-action separation by primal and upper witnesses. Carries **Corollary E4.1**, freezes 36 and 37. |
| `N4-A` | N4-A1..**N4-A20** | Two sections: The n = 4 separation rung adjudication; The n = 4 rung return: the overnight pass | 2026-08-13 / 2026-08-14 | One continuously numbered family. The SEP-A10 successor rung (A1–A12), carrying **freeze 44** (the walk-step unit and budgeted-walk contract, now at **v2**) and **freeze 45** (the n = 4 coordinate identity); then the authorised overnight pass (A13–A20), carrying **Lemma N** and Corollaries N-1..N-3 and the raised `P_max v2`. |
| `EC-A` | EC-A1..**EC-A14** | The economy-successor adjudication | 2026-08-13 | The primal half of the economy claim. Carries **Corollary S-fold-val**, **freeze 46** (the closed arm list), **freeze 36 v2** (EC-A8, transport opened to the declaration fold), the 384-versus-108 typing (EC-A12), and the primal/full split (EC-A13). |
| `T1-A` | T1-A1..**T1-A12** | The trick-1 witness: the bounded sandwich, refuted and replaced | 2026-08-14 | The first-trick target. Carries **Lemma T1-run**, **Lemma T1-force**, **Proposition T1-blind** and **Proposition T1-corner** (the refutation, itself a filed result), **Theorem T1-draw**, **Corollary T1-ruff**, and **freeze 47** (the trick-1 carrier). **T1-A12 is the implementation-versus-corpus risk** and is inherited by everything below it. |
| `LD-A` | LD-A1..**LD-A13** | Lay downs: the characterization, and the four-laydown question | 2026-08-14 | The family term made exact. Carries **Theorem LD** ((L1) ∧ (L2)), **Corollary LD-fold**, **freeze 48** (the lay-down catalogue), the settled four-laydown question (LD-A11), and LD-A10(ii), which carries T1-A12's risk forward sharpened. |
| `RW-A` | RW-A1..**RW-A8** | The map-free rule walk, and what h9 already decided | 2026-08-14 | The rule-economy probe at the n = 4 carrier. Carries **freeze 49** (the n4 economy carrier), the closed rule argument list `(record, legal)`, RW-A3's label pair (NOT PRICED / RULE-EVALUATED, never merged), and h9's coordinate verdict filed from the S6h numbers alone. |
| `FT-A` | FT-A1..**FT-A29** | The fusion tax: inbox 016 adjudicated | 2026-08-14 | The upper side. Carries **Lemma FT-arrive**, **Lemma FT-trunc** + **Corollary FT-grade4**, **Proposition FT-flat**, **Proposition FT-tie**, **Lemma FT-post**, **Corollary FT-conv** and **Lemma FT-mix**; **freeze 38 v1** (FT-A17, the reservation discharged) and **freeze 50 v1.1** (FT-A18, amended at FT-A23 and FT-A24). The closing notes (A23–A28) adjudicate the returned run; **FT-A29** files two self-corrections to the section. |
| `SR-A` | SR-A1..**SR-A37** | The second rung: inbox 017 adjudicated | 2026-08-14 | The upper side, one rung deeper. Carries **Lemma SR-coord**, **Lemma SR-forced**, **Proposition SR-sep**, **Proposition SR-post**, **Corollary SR-conv**, **Proposition SR-degen**, **Proposition SR-taut** and **Proposition SR-loc**; **freeze 51** (SR-A22, the depth-two carrier) and **freeze 38 v1.1(d)** (SR-A21, a clarification — v1 not amended, v2 not opened). The closing note (A27–A32) adjudicates the returned run and discharges FT-A28 entire; **A33** and **A34** adjudicate two defects the build found in itself, **A35** types the companion's cross-process digest as an audit note, **A36** records the chapter's first pass with no specification conflict, and **A37** withdraws a carried obligation that was never owed. |
| `FF-A` | FF-A1..**FF-A33** | The feature-fee audition: Jason's control feature, specified | 2026-08-14 | Which cheap structural features price the first-layer tax, on a carrier where the perfect answer is filed. Carries **Proposition FF-blind**, **Lemma FF-min**, **Proposition FF-oracle**, **Proposition FF-degen** and **Proposition FF-corr**; **freeze 52** with amendments **v1.1** (FF-A15), **v1.2** (FF-A20), **v1.3** (FF-A23) and **v1.4** (FF-A33). Two closing notes: A10–A24 adjudicate the first run and the defect it exposed, A25–A33 the corrected re-run, the shared-θ fit, and the chapter's close. |
| `FC-A` | FC-A1..**FC-A23** | The fee-correlation chapter: what a fee bites on, measured | 2026-08-14 | Why a fee bites, measured where the exact answer is filed. Carries **Proposition FC-drop**, **Corollary FC-null**, **Proposition FC-width**, **Proposition FC-tight** and **freeze 53**. Delivers the branch's first **pre-fee screening statistic** and its first structural limit on the fee route. A22 binds the "attained, never exact" phrasing; **A23 closes the range and replaces adjective-led claims with the exact one-sided reading**. |
| `SS-A` | SS-A1..**SS-A18** | The seed survey: a hundred fresh coordinates, designed | 2026-08-15 / 2026-08-16 | The outcome-independent 100-seed, 400-unit grade-4 survey. Carries **freeze 54**, the repaired spreading generator, the **SS-R1..SS-R9 receipt series** (R1/R2 blocking pre-run; R5 amended at SS-A11, its stated content having been tautological), complete-face/tie-multiplicity receipts, the returned survey reading and its corrections. **SS-A18** repairs one cross-reference (`FF-A26(iv)`, not `FC-A26(iv)`) and closes the range without changing freeze 54. |
| `GT1-A` | GT1-A1..**GT1-A24** | Three sections: GPU-native trick-1: the bounded portable foundation; the binding M2 Metal parity gate; the binding M3 perfect-recall-net parity gate | 2026-08-16 / 2026-08-17 | One continuously numbered family. The received-v0.2/adjudicated-v0.3 parent through **freeze 55**, then the exact M2 rebrief, binding M2 contract, integer corpus, extracted choose table, typed ABI, complete parity carrier, sequential runner, persistence gate and finite Lean foundation through **freeze 56**. The historical Gate-0 NO-GO remains a true old-environment receipt. **M2 METAL PROJECTOR PARITY COMPLETE under freeze 56**; it computes no action value, selected lead, optimal set, information net, continuation, performance claim or player. A18–A24 add the exact M3 rebrief, the binding M3 contract, the perfect-recall/world-revealed key types, the two-family reduction algebra, the `Texas42.Trick1PerfectRecallNet` proof boundary and **freeze 57** — the gate only, **no M3 result recorded**. **RANGE RE-FROZEN at A1..A24 and the chapter closed**; any later GT1 ruling requires an explicit rebrief and another range re-freeze. |
| `SP-A` | SP-A1..**SP-A12** | Signed-pivotal intake adjudication | 2026-08-18 | The intake audit of the received `signed_pivotal_geometry_v0.1.md`, in the DS-A1..A16 shape. Central mathematics SOUND (boxed identities verified exactly at intake); exactly one general claim FALSE as written, repaired at **SP-A5**; renames **pivotal cover / pivotal win share / frozen policy** (SP-A1..A3); tape typing (SP-A6); the sandwich discipline inherited into the three locks (SP-A7); E0 **ADOPTED as the tilt audit** with corrections SP-A8..A10; O10–O11 permanently retired (SP-A11); Gate E concordance (SP-A12). |
| `FZ-A` | FZ-A1..**FZ-A6** | The freeze-56 v2 amendment | 2026-08-24 | The one-crate unification meets the source closure: freeze-56's cumulative source closure re-issued append-only at the post-fold layout (`…m0_m2_sources_v2.sha256`, a **new** build identity attested by no hardware receipt yet); the 32-entry fold-translation table as verifier amendment; drift disposition; full-closure checking demoted to **freeze-event** verification; the standing M2 receipt explicitly **old-layout evidence**, its re-earning deferred to [[m2-receipt-reearn]]. Unlike SS-A and GT1-A, neither SP-A nor FZ-A carries a range-close marker, so both ranges end open. |
| `CE-A` | CE-A1..**CE-A8** | Calculated-evidence intake adjudication | 2026-08-24 | The adjudication of the received `calculated_evidence_v0.1.md` (anytime-valid adaptive settlement; the parent embodied Jason's Pro refinement pass). Identities **SOUND** at intake, 18/18 exact (CE-A1); the **θ/ϑ split** adopted walt-wide (CE-A2, superseding the SP companion's bare-θ proposal); the six-way **result-type ladder binding** on the new correctness path (CE-A3); **O20–O28 accepted** into the SCENARIO-PLAYER ledger (CE-A4); fixed counts leave the correctness path, block racer narrowed to heuristic, §10.1 becomes gate fixture V7 (CE-A5); LEVEL2-PROBE amended with the three-way wake-up split and the `𝓘 = q·D_{1/2}(τ)` cost coordinate (CE-A6); the parent's **§22 adopted as the build program**, A.6 vertical slice first, old player stays default until gates justify a change on Jason's word (CE-A7); refinement-agenda disposition, no panel convened (CE-A8). **CE-A8 is the file's final ruling as of 2026-08-24**; the range ends open. |

Three numbering traps. **`E-A` and `E-Q` are spent** by the endgame-store section
of 2026-08-11 — SEP-A had to renumber the separation design's own `E-Q1..E-Q8`
to `SEP-Q1..SEP-Q8` to avoid a collision inside one file. `F2`'s internal
amendments are bare `A1`–`A4`, local to F2, not a prefixed family. And **standing
rulings inherit as whole families, by name, never as ranges**: two design headers
recite `X-A1..X-A17`, `E-A1..E-A20`, `S-A1..S-A18`, but the families actually run
to X-A19, E-A21 and S-A21. The full corpus binds regardless of a header's recited
range; those recited ranges are typos, not scope declarations.

## The supersession chain

Nineteen pointer markers, governed by **DS-A28**. Every one is navigation, not a
rewrite: the original text stands and the marker names its replacement. Their
absence would be drift.

### In `CENSUS-RULINGS.md` — twelve markers

| Marked site | Marker | Corrected by | Durable replacement |
|---|---|---|---|
| **Lemma J**, clause (c) | GENERALISED | DS-A24 | **Lemma J(c′)**, errata §8.5(e). (c) is *sound as filed* — generalised, not repaired. |
| **DS-A7(iii)**, the sentence naming `revealed.rs` as needing an evaluator built | CORRECTED | SEP-A7 | **SEP-A7** with freeze 37 at SEP-A6. `revealed_summary().q_c[a]` has been the action-conditioned U_a since S3; what remained to build was the harness. |
| **DS-A9**, the cone clause "for every cone at once" | CORRECTED | DS-A24 | **Lemma E8**, errata §8.5. Equality is guaranteed only for valuation directions constant on the exchanged tiles. |
| **DS-A10**, the Experiment E receipt clause | SUPPLEMENTED | SEP-A12 | **SEP-A12 (R1)–(R5)**. The clause stands, but its two assertions hold by construction and are not receipts in PG-A8's sense. |
| **SEP-A13**, the third counted quantity | DISAMBIGUATED | SEP-A19 | **SEP-A19**. The third quantity is the distinct partition states the walk *reaches*; equality with `InfoPartition::len()` is unsatisfiable for any pruning policy. |
| **SEP-A4(e)**, freeze-36 transport = identity only | AMENDED | EC-A8 | **Freeze 36 v2** additionally admits the declaration fold under its explicit image-key construction, receipts and Corollary S-fold-val; every further transport still re-enters. |
| **SEP-A17**, “the 108-decision playbook” | DISAMBIGUATED | EC-A12 | **108 is the strictly-mattering subset**, a derived difference of two measurements; the receipt-backed free-decision count is 384. |
| **N4-A1(i)**, the cross-traversal fence | NARROWED | Lemma N | Traversal counts remain incomparable unless the traversals are exhibited as the same; the partition build and envelope-H walk are one such exhibited identity. |
| **N4-A1(c)**, the six evaluator inventory | CLARIFIED | RW-A8 | Freeze 44(b)'s `Option`/no-partial contract binds every `walk`-based evaluator; the six were an inventory, not a closed type list. |
| **N4-A1(e)**, `P_max = 32,000,000` at insertion | AMENDED | N4-A16(vi) | **Freeze 44 v2:** `P_max = 192,000,000`, applied to the completed count-only result before allocation; insertion is a defensive stop only. |
| **N4-A4**, provenance of `P_max v1` | SUPERSEDED IN PART | N4-A16 | The v1 estimate was measured wrong; N4-A4's rule survives — v2 is declared in adjudication and is never derived from machine memory at run time. |
| **N4-A12(b)**, the three-coordinate fallback | SUPERSEDED | N4-A19 | Its wall gate was retired; the result-independent replacement is the full nine-coordinate pass with measured per-unit admission. |

### In the errata — seven in-place markers

| Marked site | Marker | Under | Effect |
|---|---|---|---|
| §3.1, the object | Naming clause added | DS-A20 | Treatment **C** reveals ω only; revealing (ω,z) is **C⁺** and never called C unqualified. |
| §4.2, the Obligation | Restated semantically | DS-A27 | The invariant is semantic; "no max node below the root" is demoted to a sufficient *implementation form* — a receipt for the invariant, not the invariant. |
| §5.1, Proposition E5 | Statement amended | DS-A20 | Hypothesis **(S)** added explicitly; **Lemma E5.0** (§5.1a) shows it holds on the measured carrier. |
| §5.2, Corollary E5.2 | Language narrowed | DS-A21 | The negative is "atom-mass **linear** filtering is noncompressive on this carrier", not "filtering and compression are incompatible". |
| §5.3, the reframe | Sharpened | DS-A21 | Predictive rank lower-bounds the **linear factorisation** target only, not unrestricted circuit size. |
| §6, Theorem E6.1's typing clause | Superseded in part | DS-A23 | The primary object is **Definition E9**'s interface-local width; the root-level W_reach is a different, fourth quantity. |
| §6, Theorem E6.2(c) | Narrowed | DS-A26 | Restricted to the two pruning rules actually in use; duplicate-discarding rules preserve the count trivially. |

**The shape of the chain.** DS-A24 is the mathematical correction inside the
original decision-sparse theorem chain, and it fires twice, both landing on
errata §8.5. DS-A20/A21/A23/A26/A27 are errata maintenance. The later markers
are implementation-boundary, freeze-version, scope or provenance repairs; none
is permission to rewrite the marked text or to transport a result across the
new boundary silently.

## Standing disciplines a successor must not relearn the hard way

Each of these is a ruling, not a convention, and each was bought with a mistake.

- **F7, NO-RESCUE.** Both outcomes of every experiment are results. A mismatch
  against the concrete authority is stop-and-report; never patched, never
  reconciled by adjustment.
- **DS-A1, vocabulary.** *witness* = a mathematical object exhibited to prove a
  claim; *receipt* = a machine-checked verification artifact regenerated by a
  run. **The word "certificate" is not used in walt artifacts written under this
  ruling** (D3); quotations of documents that use it are bracketed. DS-A1 was
  ruled on 2026-08-13 and **binds forward, not retroactively**, so a grep of
  `walt/` returns many hits and none of them means the rule is dead: received
  documents preserved verbatim (DS-A18), pre-reset code and artifacts, prose
  written before the ruling landed, and sentences that state the fence itself
  all contain the word legitimately. **No inventory of those hits is maintained
  here** — the tree moves, and any list would rot into a false completeness
  claim. The operative test is prospective: in anything you write, use *witness*,
  *receipt*, or *root-action separation*, and bracket the word when quoting a
  source that uses it.
- **PG-A8, "by construction is not a receipt."** An assertion that cannot fail
  is not evidence. SEP-A13 is the sharpest instance: an `is_affine()` check that
  is vacuous at the declared direction.
- **E-A2, the count boundary.** Structural transports preserve BEATS relations,
  not pip counts. If count re-enters, every form-keyed record is void
  **wholesale, never extended**. The one thing that survives is a *policy* as a
  primal-witness source (DS-A16) — the policies extend, the verdicts do not.
- **R-A2 and P-A1, the reachability fence.** The measured domain is the
  void-free capacity fiber, a declared cost domain. Its members are FEASIBLE and
  never reachable; no object here is identity-bearing. Restate it verbatim
  wherever a witness is reported.
- **PG-A13, a stop is a stop.** A capped coordinate reports no count at all —
  not a partial one and not a bound. Do not infer where the first split happens
  from where a capped run stopped.
- **R-A18, the correctness gate.** Treatment H is the concrete authority; a
  disagreement is a bug. If H does not complete within budget, that is a
  declared stop printed with what was reached, and every dependent row prints
  "correctness gate unmet" beside it — never silently.
- **DS-A15/Lemma E7, seeds versus witnesses.** Seeds are heuristics for
  *finding* witnesses; witnesses are validated by exact evaluation, always.
  Dominance does not travel with a policy alone.

## Run names, and where a quoted measurement comes from

Rulings and results files refer to runs by session label. The labels are not
self-explanatory and no page above decodes them, so here is the minimum a reader
of these pages needs. **The session ledger itself lives in `walt/LOG.md`** (the
retired `walt/PLAN.md` carried it historically; `git show 56e2173:walt/PLAN.md`).
Result artifacts cited by `results/...` basenames live at
`walt/probes/factory-results/` since the 2026-08-24 relocation.

| Run | What it was | The artifact that carries its numbers |
|---|---|---|
| **S5g** | the railyard | `results/census_yard*_2026-08-10.txt` |
| **S5h** | the fiber-crush probe (three-arm baseline ladder) | `results/fiber_probe_2026-08-11.txt`, `fiber_probe_h_2026-08-11.txt` |
| **S5i** | the fiber-refinement probe (declared exclusion remnants) | `results/fiber_refine_2026-08-11.txt` |
| **S5j** | the endgame store (symmetry-reduced tablebase) | `results/endgame_store_2026-08-11.txt`, `endgame_floor_2026-08-11.txt` |
| **S5k** | the seat-level census | answered by proof; see Corollary S-rigid |
| **S6a** | the predictive-rank probe | `results/predictive_rank_2026-08-12.txt` |
| **S6b** | the policy-geometry probe | `results/policy_geometry_2026-08-12.txt` |
| **S6c** | the decision-deadness probe | `results/deadness_2026-08-12.txt` |
| **S6d** | the separation probe (Experiment E) | `results/separation_2026-08-13.txt` |
| **S6e** | the economy-seed probe | `results/economy_seed_2026-08-14.txt` |
| **S6f** | the measured n = 4 admission rung | `results/separation_n4_rung_2026-08-14.txt` |
| **S6g** | the trick-1 drawing-family probe | `results/trick1_draw_2026-08-14.txt` |
| **S6h** | the full n = 4 overnight pass | `results/separation_n4_2026-08-14.txt`, `separation_n4_2026-08-14_deterministic_block.txt` |
| **S6i** | the lay-down characterization/catalogue | `results/laydown_2026-08-14.txt`, `laydown_catalogue_2026-08-14.txt` |
| **S6j** | the map-free rule-economy pass | `results/rule_economy_n4_2026-08-14.txt` |
| **S6k** | the first-rung fusion-tax probe | `results/fusion_tax_2026-08-14.txt` |
| **S6l** | the depth-two nonanticipativity rung | `results/second_rung_2026-08-14.txt` |
| **S6m** | the feature-fee audition and repaired rerun | `results/feature_fee_2026-08-14.txt`, `feature_fee_v11_2026-08-14.txt` |
| **S6n** | the fee-correlation diagnostic | `results/fc_correlation_2026-08-14.txt` |
| **Seed survey** | the unnumbered 2026-08-15 hundred-seed survey and its declared scratch cuts | `results/seed_survey_2026-08-15.txt`, `seed_survey_2026-08-15_cutA.txt` |
| **GT1 M0/M1** | the portable GPU-native trick-1 foundation | `walt/receipts/gpu_native_trick1_m0_m1_v1/` |
| **GT1 historical Gate 0** | the immutable NO-GO observation from the old Command-Line-Tools-only environment | `walt/receipts/gpu_native_trick1_gate0_2026-08-16.txt` |
| **GT1 M2** | the committed receipt for the exact freeze-56 status sentence; executable fixed-carrier evidence, not a theorem | `walt/receipts/gpu_native_trick1_m2_v1/` |
| **Scenario-player first day** | the level-1/level-2 ladders and the 3×384 arena vs the E[Q] champion (2026-08-17) — arena outcomes about play, never exact values | `walt/probes/m3/arena_results_2026-08-17.txt`, `level1_results_2026-08-17.txt`, `level2_results_2026-08-17.txt` |
| **Divergence miner** | 900 self-played hands / 4,156 level-2-shadowed decisions (2026-08-18) — the tilt audit's first anchor corpus | `walt/probes/m3/divergence_results_2026-08-18.txt`, corpus under `walt/probes/m3/mined/` |
| **Tilt-audit smoke** | the E0 smoke (2026-08-19): trick-6/trick-4 anchors, the no-tape finding, the racing bench and arena gate — estimates, never receipts | `walt/TILT-AUDIT.md` § "Smoke results", `walt/probes/tilt_arena_2026-08-19.log` |

Every measured number quoted anywhere on these pages comes from one of those
files. **They sit one tier below even these pages** (probe output is exploratory
material cited by nothing above it), and a number becomes quotable as a *result*
only by brief amendment adding it to a verifier receipt. Where a page names a
measurement — the grade-3 predictive dimensions, the detector recall figures,
the singleton frontiers — treat it as a pointer into the artifact above, never
as a standing claim.

The design documents that each ruling family adjudicates live at the top of
`walt/` (`walt/POLICY-GEOMETRY.md`, `walt/SEPARATION-PROBE.md` and others) —
except the seven retired 2026-08-24 after their probes closed (`CENSUS`,
`FIBER-PROBE`, `FIBER-REFINE`, `ENDGAME-STORE`, `SEAT-CENSUS`,
`PREDICTIVE-RANK`, `DEADNESS-PROBE`), whose bytes are preserved at
`git show 2de8a05:walt/<NAME>.md`. A ruling that says "the design" means the
one named in its section's opening paragraph.

## Addendum, 2026-08-17 — M2 closure over the portable trick-1 boundary

The portable boundary was recorded on 2026-08-16; the executable M2 conjunction
closed on 2026-08-17. Pointers only; the rulings and contracts govern.

- **The `GT1-A` family is closed at GT1-A17 and carries freezes 55 and 56.** The
  portable parent remains exactly the received-v0.2/adjudicated-v0.3 chain,
  narrow `OpeningRootV1`, generated semantics, U256 mass/frame ABI, opening-cell
  generator, reduced carrier and persistence boundary. M2 adds only the exact
  rebrief/contract, U256 corpus, extracted choose table, typed Metal ABI,
  `M2OpeningParityCarrierV1`, sequential runner, closed receipt and finite Lean
  obligations. Freeze 26 is cited unchanged; 39/40 remain reserved; freeze 44
  and M3+ are excluded.
- **`PORTABLE M0/M1 COMPLETE under freeze 55`.** The final checked source
  manifest, committed canonical envelope and stop, fresh byte comparison, Rust
  gate and Lean target passed together. This bounded status establishes no Metal
  result, perfect-recall net, controller, root value or opening action.
- **M2 METAL PROJECTOR PARITY COMPLETE under freeze 56**. This establishes only
  exact Metal/Rust arithmetic and opening-projector parity on the frozen carrier;
  it computes no action value, selected lead, optimal set, information net,
  continuation, performance claim or player. M3+ and every opening-root
  verdict remain untouched.
- **The historical Metal Gate-0 NO-GO remains true.** It is an immutable receipt
  of the old Command-Line-Tools-only environment, not the current host status and
  not a receipt to rewrite after the toolchain became available.
- **The Lean foundation is substantial and deliberately incomplete.** The parent
  module proves the legal budget/420/width/current-trick-point/cell-count/interval
  layer; `Trick1MetalFoundation` proves the finite arena/count/order/no-partial
  obligations. Neither proves the semantic opening partition and formulas,
  posterior refinement, information-key equivalence, canonical least-index
  verdict, Rust/Lean correspondence or Metal/Rust correspondence. The M2 receipt
  is executable fixed-carrier evidence bearing on the last relation, not a
  theorem or general correspondence proof.
- **The `SS-A` family is now administratively closed at SS-A18 and carries
  freeze 54.** SS-A18 corrects `FC-A26(iv)` to `FF-A26(iv)` in the non-null
  pairing provenance and closes the range; it changes no survey receipt, number,
  object or reading.
- **Freeze 44 is v2.** `P_max = 192,000,000` is applied to the completed exact
  count-only result before allocation; a larger unit is `NOT PRICED`, and the
  insertion check is a defensive stop rather than a receipt. The older 32M row
  is not the live freeze.

## Addendum, 2026-08-14 — what a walt-math successor inherits

Pointers only. Nothing below is restated from its source; open the ruling.
Written for walt-math-11 at the close of the FT chapter and extended the same day
as walt-math-11 filed the SR, FF and FC chapters. **Sections run newest first;
the FC items are the most likely to be needed.**

### The FC chapter (newest in the 2026-08-14 addendum)

- **The `FC-A` family** — `FC-A1..FC-A23`, in `CENSUS-RULINGS.md` § "The
  fee-correlation chapter: what a fee bites on, measured" (≈ lines 11170–12099 as
  of this date). Four named objects, indexed above, and **freeze 53**.
- **A PHRASING RULE BINDS ANYONE WRITING FROM THESE RULINGS, and it corrected the
  adjudicator's own text before it could reach a wiki page.** The drop bound is a
  **lower bound at every state**. Where it equals the frozen captured amount it is
  **ATTAINED**, never *exact* — "exact at 258 states" invites the reading that the
  screen predicts capture a fifth of the time, which is what the chapter exists to
  prevent. **And attainment is not identifiable in advance**: nothing in the
  emitted data says *which* states attain it without computing the very quantity
  the bound exists to avoid computing. So say *attained at 258 of 1,252*, say *a
  lower bound everywhere*, and say *which states attain it is not knowable without
  the captured amount*.
- **The generalised scope rule**: a figure's scope names **every** dimension it
  ranges over — state set, feature set and unit set — in the same sentence, and **a
  scope derived from an adjective rather than stated as a set is not a scope.** The
  chapter's own demonstration is the sharpest argument for it: one census reads
  1,010/322 over one state set and 252/322 over another, and the first invites
  "zero at most states" while the second says what is true.
- **Independent verification means an independent *predicate*, not an independent
  *party*.** Two agents running one grep are one check, however many agents there
  are. This was minted the hard way: a check written with a regex anchor where a
  literal was meant matched nothing and returned a clean, confident zero
  violations — **a wrong predicate returning exactly the answer being hoped for**,
  caught by implausibility across several simultaneous queries rather than by
  suspicion of the predicate. The operational tell: *a check that returns exactly
  the hoped-for answer with no exceptions deserves one more query by a different
  route before it is believed.* Its companion: **a predicate that fails loudly on a
  near-miss is strictly better than one that silently matches it.**
- **Two more disciplines**: receipt what the probe recomputes each run, and leave a
  documented one-time audit where the object is fixed source; and emit over the
  full set, read over the meaningful subset, **naming both**.
- **What the chapter leaves open**: the third coordinate is still uncommissioned
  and still needs its own freeze, but its selection criterion has changed — it is
  now chosen on **measured argmax multiplicity**, with the earlier trump-survival
  input demoted to a hypothesis about a correlate.

### The FF chapter

- **The `FF-A` family** — `FF-A1..FF-A33` with two closing notes, in
  `CENSUS-RULINGS.md` § "The feature-fee audition: Jason's control feature,
  specified" (≈ lines 9889–11167 as of this date). It delivers **five** named
  objects, indexed above, and **freeze 52** with four amendments. **FF-A32
  declares the chapter CLOSED**; nothing further is commissioned.
- **SR-A37 withdraws an obligation that was never owed**, and the withdrawal is
  the durable part. SR-A25(v) and SR-A32(iv) had twice filed "claim-ledger,
  FINDINGS and open-problems cross-references are owed"; the wiki owner declined
  with cause, walt-math verified the cause rather than accepting it, and the
  correct count is **zero** — `walt` appears zero times in all three pages, and
  no walt chapter has ever acquired a row. **The rule it yields: an obligation to
  write somewhere is asserted only after reading that destination's own `owns:`
  line and its rulings — a cross-reference list in governing text names
  candidates, never obligations.**
- **Three more disciplines this chapter filed, all general.** *(a)* **A results
  file may restate a reading-rule that a ruling has fixed; it may not originate
  one.** Numbers and provenance are the artifact's to assert; how a number may be
  read belongs to the rulings. *(b)* **A byte-diff between two emissions must be
  produced while both objects exist, or not at all** — it cannot be reconstructed
  after a sanctioned regeneration has overwritten its comparand. Name the check
  *and* the moment its comparand exists. *(c)* Where a superseded figure retains
  a legitimate use, that use is named exactly and every other use is closed: here
  the pre-amendment capture survives **only** as the historical measurement of
  the feature-as-frozen and hence as one term of the comparison that confirmed
  its own supersession.
- **Two disciplines from this chapter are general and should outlive it.**
  *(1)* **No capture figure, and no count, appears anywhere without the state set
  it ranges over named in the same sentence.** This file reports the same
  quantity over nested state sets, and scope mislabelling is its standing hazard —
  it caught the adjudicator twice in one chapter. *(2)* **A null control is
  complete only when paired with a case whose correct answer is known to be
  non-null**, because a control expecting zero cannot distinguish a working
  instrument from one that always returns zero.
- **Freeze 52's amendment sequence is worth reading before writing another
  freeze**: a scoping clause was written for a *family* after defining a term
  only *part* of the family uses, and was never checked against each member's own
  definition. It voided six of twelve measurement cells by construction. The
  repair is that **every feature carries its own domain clause**, and that a unit
  whose domain is empty is declared an **EMPTY TEST** — reported as a unit that
  did not run, **never as a zero**.
- **A pre-declared gate quantified over "both arms" silently assumes both arms
  are non-empty.** One did not fire as written here, and a narrower verdict was
  substituted in the open rather than the gate being read as satisfied. **A
  pre-declared outcome must either quantify over non-empty arms or carry an
  explicit empty-arm branch.**

### The SR chapter

- **The `SR-A` family** — `SR-A1..SR-A37` plus its closing note, in
  `CENSUS-RULINGS.md` § "The second rung: inbox 017 adjudicated" (≈ lines
  7986–9827 as of this date; the heading governs). It adjudicates a second
  received external note claim by claim and delivers **eight** named objects,
  indexed in the family table above.
- **SR-A30's discharge of FT-A28 depends on SR-A33, and the dependency is
  recorded rather than assumed.** The probe's own streaming SHA-256 had a
  buffered-length defect, caught by a FIPS known-answer self-check **before any
  carrier number existed**. That matters because a mis-buffering hash is still
  *deterministic*, so two runs would still have agreed and the digest receipt
  would have been **green and worthless** — a broken compression function may be
  wildly non-injective, and (FT-R7c)'s scope claim ("one scalar reaches every
  individual value across executions") is a statement about **the digest
  function**, not about the probe. The standing discipline: *a receipt whose
  assertion is an equality of digests carries a second, silent obligation — that
  the digest function is anchored to published known-answer vectors covering the
  code path actually used, including the streaming path if the receipt streams.*
  A one-shot-only vector set would have passed here. This is the same family as
  Proposition SR-taut and FT-A28(i): **a check is only a check against something
  it does not itself produce.**
- **The 8.8 GB companion's cross-process digest identity is an AUDIT NOTE of real
  weight, never a receipt.** Its evidentiary surface is far broader than the four
  frontier digests — every depth-two row across two processes — but it is not
  asserted in-run against a transcribed constant, not reproduced by any verify
  path, and does not survive into a future run. It is convertible at zero cost by
  carrying it in the frozen table, which is one of the four items owed below.
- **Freeze 51** (SR-A22(iii)) is the depth-two probe carrier. **Freeze 38 stands
  at v1 with clause (d) exhibited as v1.1(d)** (SR-A21(ii)) — *a clarification
  with no new content*: v1 is not amended and **v2 is not opened**. Both are now
  in [the freeze register](walt-math-freezes.md); 39 and 40 remain reserved.
- **FT-A28 is now FULLY DISCHARGED** by SR-A30 — the deferred frontier digest is
  carried by all four SR units, closing FT-A28(iii)'s named residual **by receipt
  rather than evidentially**. Nothing remains owed on that line.
- **The errata §9 queue has grown again.** DS-A28(ii) is still carried, and the
  queue now holds the seven FT objects *plus* **Lemma SR-coord, Lemma SR-forced,
  Proposition SR-sep, Proposition SR-post, Corollary SR-conv, Proposition
  SR-degen, Proposition SR-taut** and **Proposition SR-loc**, together with the
  confirmed second-rung mathematics of the received note.
- **Four obligations are owed on the next second-rung emission, and nothing is
  owed now** (consolidated at the end of the section; an earlier clause says
  "two" and was written before A33–A36): the escape column's `yes`/`no` case fix;
  the filed binding pairs transcribed into the frozen table and **asserted**,
  which converts a construction into a comparison against a named carrier; the
  companion digest carried in the same table; and — before any rung-three or
  longer-ladder build — a **re-design**, not a re-application, of the
  committed/companion emission split. The companion is 8.8 GB here against the
  previous chapter's 36 MB, and the growth is **not incidental**: the depth-two
  state count grows with the field plies between frontiers, so a longer ladder
  multiplies it again.
- **Grade 4 is exhausted as a test-bed.** Proposition SR-degen bars it from
  testing closure, and there is no rung three there. The next question needs a
  longer ladder, which makes FT-A21's three trick-1 obligations the binding
  constraint rather than a distant destination.

### The FT chapter

- **The `FT-A` family exists and is large.** `FT-A1..FT-A29` plus its closing
  notes, in `CENSUS-RULINGS.md` § "The fusion tax: inbox 016 adjudicated"
  (≈ lines 6567–7984 as of this date; the heading governs, not the line numbers).
  It adjudicates a received external note claim by claim, delivers eight named
  objects (indexed in the family table above), and is the home of the upper-side
  mathematics. **The section now runs to FT-A29** — the file is append-only and a
  section grows after its own closing note, as this one did three times in a day,
  so **read to the end before assuming any ruling is the last one**.
- **FT-A29 files two self-corrections to the section.** The first was found by an
  outside check, the second by a census of the results file: FT-A16(ii)'s "(LD-R4) remains owed" was wrong (the
  receipt had already run and held), and FT-A25(vi)'s commentary "ten of twelve
  pairs" undercounted — **the closure failed at eleven of twelve**. Neither
  touches a verdict, a receipt, a freeze or any results-file number. The
  discipline it yields is worth more than the corrections: **a ruling that creates
  an obligation is not evidence the obligation is still open — only the artifact
  is.** That is "by construction is not a receipt" transposed from claims to
  obligations, and it joins FT-A23(v) and FT-A28(i) as the third instance of one
  failure shape: *asserting a status from a text that governs it rather than from
  the object that carries it.*
- **Two freezes moved, and the versions are not interchangeable.** **Freeze 38
  stands at v1**, scoped — the gluing cut, its reservation discharged at FT-A17;
  feature penalties, multi-stage penalties, adaptive search beyond the first
  frontier and any cost model are explicitly *not* in it and re-enter as v2.
  **Freeze 50 stands at v1.1**, amended twice on the day it was fixed: clause (a)
  at FT-A23 (the enumeration governs, the sort clause struck) and clause (c) at
  FT-A24 (emission cut by content). **39 and 40 remain reserved.** Full text on
  [the freeze register](walt-math-freezes.md).
- **DS-A28(ii) is STILL OWED.** Corollary E4.1's filing as errata §4.3 has been
  carried since SEP-A2 and is still due at the next errata amendment. FT-A27(i)
  added the seven FT objects and the confirmed first-layer mathematics to the same
  queue as a new **errata §9**; the SR chapter has since added eight more, so read
  the SR section above for the current queue. Until that amendment lands,
  `CENSUS-RULINGS.md` is **their only authority**.
- **FT-A28** (≈ lines 7825–7936) ratifies the (FT-R7) two-half discharge,
  versioning the halves rather than renumbering them — **(FT-R7a)** the cross-run
  invariant receipt against a frozen table, whose scope is corrected *upward* to
  reach both `Σ_I δ_I` and `|supp δ_I|` per unit, and **(FT-R7b)** the in-run
  reproduction receipt, which reaches every individual value within one process.
  It names the residual the conjunction does not cover, defers the closure to
  **(FT-R7c)**, and rules that the orchestrator's byte-diff is an **audit note and
  never a receipt**. **All of this is now discharged** — see the SR section above;
  the bullet is kept because the *reasoning* is the precedent, not the status.
- **Three specification defects of one shape, all found by someone else.**
  FT-A23(v), FT-A28(i) and FT-A29(i) are one error three times: **asserting a
  status from a text that governs it rather than from the object that carries
  it**, or equivalently **naming a relation without naming its relata**. A freeze
  clause states a constant *or* a generating rule, never both; a receipt comparing
  against a prior run must name the **carrier** of the reference value — a frozen
  table with its provenance line, or an in-run recomputation — never "the previous
  emission" unqualified; and an obligation-creating clause is not evidence the
  obligation is still open. A prior run is not an object and its results text is
  not an interface.
- **T1-A12's implementation-versus-corpus risk is now inherited by seven
  families.** Every statement in T1-A, LD-A, RW-A, FT-A, SR-A, FF-A and FC-A is proved
  relative to walt's *implementation* of the rules, read from the code at
  adjudication time, and **no receipt inside those sections can detect a
  disagreement with the rules corpus** because every receipt is computed by that
  same implementation. It is sharper at SR: Lemma SR-coord, the hypothesis that
  makes the whole rung-two law true, was itself discharged by reading the
  implementation — and two agreeing traversals inside one implementation cannot
  detect a divergence from the corpus. The corpus check is owed before any of it
  is cited outside walt, and (LD-R4) is a probe of the risk, never a discharge.

  **2026-08-16 boundary:** GT1-A3 addresses T1-A12 for the portable M0/M1 slice
  with a separate prose-rules resolver and complete declared-domain comparisons
  for context/follow/winner/points. That is executable bridge evidence for this
  slice, not a retroactive proof of the seven earlier families and not a Lean
  rules-refinement theorem.

## Where the rest lives

- [Received artifacts and intakes](walt-math-intakes.md) — the first-class
  index of the frozen bases, Pro-channel intakes and rebriefs, their
  companions, and the pinned manifests: what each is, where the verbatim
  parent lives, what came of it.
- [The freeze register](walt-math-freezes.md) — all 57 issued freezes with
  content, version and declaring ruling (39 and 40 still reserved).
- [Open questions](walt-math-open-questions.md) — what is genuinely unresolved,
  and why none of it belongs in [open-problems](open-problems.md).
- [The walt hub](walt.md) — the build map, sessions, and the exploratory fence.
  Owned by another page; this reference does not restate it.
