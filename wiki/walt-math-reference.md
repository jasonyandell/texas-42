# walt mathematics — the reference map

[Home](Home.md) · owns: the map of walt's mathematical corpus — every named
object, where its full statement lives, what it binds, and what corrected it ·
Sources: `walt/CENSUS-RULINGS.md` (the adjudication record),
`walt/math/decision_sparse_exact_solving_v0.1_errata.md` (the maintained
mathematics), and the received documents under `walt/math/`. Related:
[walt hub](walt.md), [structure and transport](walt-math-structure-transport.md),
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

## Who this is for

This is the orientation page for the next walt-math adjudicator. It answers four
questions: what documents exist and which one governs; what has been proved and
where the proof lives; what was corrected and by what; and what is open. It
restates nothing — every entry is a pointer with just enough statement to let
you decide whether to open the source.

## The three documents, and which governs

walt's mathematics lives in three places with three different disciplines. The
distinction is load-bearing and is the first thing to internalise.

| Document | Discipline | Role |
|---|---|---|
| `walt/math/decision_sparse_exact_solving_v0.1.md` and `walt/math/decision_sparse_second_audit_v0.1.md` | **Received, verbatim, never edited** (DS-A18) | Handed-in documents. Preserved exactly as filed, for the same reason `ingest/` is: a corrected source destroys the record of what was corrected. |
| `walt/math/decision_sparse_exact_solving_v0.1_errata.md` | **Maintained** (DS-A28(iii)) | The repaired mathematics, with full statements and proofs. Hypotheses may be added and language narrowed *in place*, each change carrying a dated provenance marker naming its ruling. |
| `walt/CENSUS-RULINGS.md` | **Append-only** (DS-A28(i)) | The adjudication record. No ruling's text is ever rewritten; a corrected clause receives a bracketed dated pointer marker at its site. Also the home of the named lemmas from before the errata existed. |

**Citation rule (DS-A17), binding on every design and results file:** cite the
**errata theorem number** for the mathematics and the **DS-A ruling** for its
provenance. Where parent and errata differ, **the errata governs**. The rulings
file remains the adjudication record; it is no longer the home of the repaired
mathematics.

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

## The ruling families

Nineteen families, one adjudicator (walt-math), all exploratory. Ranges below are
**ruling-ID ranges**, which are append-only and do not move; the sections are
located by heading, never by line number, because line numbers drift with every
append. Two families span more than one section heading — `DS-A` runs across
three, `N4-A` across two — and the family, never the heading, is what inherits.

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
| `FT-A` | FT-A1..**FT-A28** | The fusion tax: inbox 016 adjudicated | 2026-08-14 | The upper side. Carries **Lemma FT-arrive**, **Lemma FT-trunc** + **Corollary FT-grade4**, **Proposition FT-flat**, **Proposition FT-tie**, **Lemma FT-post**, **Corollary FT-conv** and **Lemma FT-mix**; **freeze 38 v1** (FT-A17, the reservation discharged) and **freeze 50 v1.1** (FT-A18, amended at FT-A23 and FT-A24). The closing notes (A23–A28) adjudicate the returned run. |

Three numbering traps. **`E-A` and `E-Q` are spent** by the endgame-store section
of 2026-08-11 — SEP-A had to renumber the separation design's own `E-Q1..E-Q8`
to `SEP-Q1..SEP-Q8` to avoid a collision inside one file. `F2`'s internal
amendments are bare `A1`–`A4`, local to F2, not a prefixed family. And **standing
rulings inherit as whole families, by name, never as ranges**: two design headers
recite `X-A1..X-A17`, `E-A1..E-A20`, `S-A1..S-A18`, but the families actually run
to X-A19, E-A21 and S-A21. The full corpus binds regardless of a header's recited
range; those recited ranges are typos, not scope declarations.

## The supersession chain

Twelve pointer markers, governed by **DS-A28**. Every one is navigation, not a
rewrite: the original text stands and the marker names its replacement. Their
absence would be drift.

### In `CENSUS-RULINGS.md` — five markers

| Marked site | Marker | Corrected by | Durable replacement |
|---|---|---|---|
| **Lemma J**, clause (c) | GENERALISED | DS-A24 | **Lemma J(c′)**, errata §8.5(e). (c) is *sound as filed* — generalised, not repaired. |
| **DS-A7(iii)**, the sentence naming `revealed.rs` as needing an evaluator built | CORRECTED | SEP-A7 | **SEP-A7** with freeze 37 at SEP-A6. `revealed_summary().q_c[a]` has been the action-conditioned U_a since S3; what remained to build was the harness. |
| **DS-A9**, the cone clause "for every cone at once" | CORRECTED | DS-A24 | **Lemma E8**, errata §8.5. Equality is guaranteed only for valuation directions constant on the exchanged tiles. |
| **DS-A10**, the Experiment E receipt clause | SUPPLEMENTED | SEP-A12 | **SEP-A12 (R1)–(R5)**. The clause stands, but its two assertions hold by construction and are not receipts in PG-A8's sense. |
| **SEP-A13**, the third counted quantity | DISAMBIGUATED | SEP-A19 | **SEP-A19**. The third quantity is the distinct partition states the walk *reaches*; equality with `InfoPartition::len()` is unsatisfiable for any pruning policy. |

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

**The shape of the chain.** DS-A24 is the only mathematical correction of a
prior ruling by a later one, and it fires twice, both landing on errata §8.5.
DS-A20/A21/A23/A26/A27 are errata maintenance. SEP-A7/A12/A19 are the build-side
corrections — two from first-hand code reading, and one where a section corrects
itself after a build trip.

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
of these pages needs. **The session ledger itself lives in `walt/PLAN.md` and is
owned there, not here.**

| Run | What it was | The artifact that carries its numbers |
|---|---|---|
| **S5g** | the railyard | `walt/walt-factory/results/census_yard*_2026-08-10.txt` |
| **S5h** | the fiber-crush probe (three-arm baseline ladder) | `results/fiber_probe_2026-08-11.txt`, `fiber_probe_h_2026-08-11.txt` |
| **S5i** | the fiber-refinement probe (declared exclusion remnants) | `results/fiber_refine_2026-08-11.txt` |
| **S5j** | the endgame store (symmetry-reduced tablebase) | `results/endgame_store_2026-08-11.txt`, `endgame_floor_2026-08-11.txt` |
| **S5k** | the seat-level census | answered by proof; see Corollary S-rigid |
| **S6a** | the predictive-rank probe | `results/predictive_rank_2026-08-12.txt` |
| **S6b** | the policy-geometry probe | `results/policy_geometry_2026-08-12.txt` |
| **S6c** | the decision-deadness probe | `results/deadness_2026-08-12.txt` |
| **S6d** | the separation probe (Experiment E) | `results/separation_2026-08-13.txt` |

Every measured number quoted anywhere on these pages comes from one of those
files. **They sit one tier below even these pages** (probe output is exploratory
material cited by nothing above it), and a number becomes quotable as a *result*
only by brief amendment adding it to a verifier receipt. Where a page names a
measurement — the grade-3 predictive dimensions, the detector recall figures,
the singleton frontiers — treat it as a pointer into the artifact above, never
as a standing claim.

The design documents that each ruling family adjudicates (`walt/SEAT-CENSUS.md`,
`walt/PREDICTIVE-RANK.md`, `walt/POLICY-GEOMETRY.md`, `walt/DEADNESS-PROBE.md`,
`walt/SEPARATION-PROBE.md` and others) live at the top of `walt/`. A ruling that
says "the design" means the one named in its section's opening paragraph.

## Addendum, 2026-08-14 — what walt-math-11 inherits

Pointers only. Nothing below is restated from its source; open the ruling.

- **The `FT-A` family exists and is large.** `FT-A1..FT-A28` plus its closing
  notes, in `CENSUS-RULINGS.md` § "The fusion tax: inbox 016 adjudicated"
  (≈ lines 6567–7936 as of this date; the heading governs, not the line numbers).
  It adjudicates a received external note claim by claim, delivers eight named
  objects (indexed in the family table above), and is the home of the upper-side
  mathematics. **FT-A28 is the last ruling filed as of 2026-08-14** — but the
  file is append-only and a section grows after its own closing note, as this one
  did twice in a day, so **read to the end before assuming any ruling is the
  last one**.
- **Two freezes moved, and the versions are not interchangeable.** **Freeze 38
  stands at v1**, scoped — the gluing cut, its reservation discharged at FT-A17;
  feature penalties, multi-stage penalties, adaptive search beyond the first
  frontier and any cost model are explicitly *not* in it and re-enter as v2.
  **Freeze 50 stands at v1.1**, amended twice on the day it was fixed: clause (a)
  at FT-A23 (the enumeration governs, the sort clause struck) and clause (c) at
  FT-A24 (emission cut by content). **39 and 40 remain reserved.** Full text on
  [the freeze register](walt-math-freezes.md).
- **DS-A28(ii) is STILL OWED, and the queue got longer.** Corollary E4.1's filing
  as errata §4.3 has been carried since SEP-A2 and is still due at the next
  errata amendment. FT-A27(i) adds to the same queue: **Lemma FT-mix**, together
  with the FT mathematics listed at FT-A22(iii) — Lemma FT-arrive, Lemma
  FT-trunc, Proposition FT-flat, Proposition FT-tie, Lemma FT-post and Corollary
  FT-conv — and the confirmed first-layer mathematics, all as a new **errata §9**.
  Until that amendment lands, `CENSUS-RULINGS.md` is **their only authority**, and
  the DS-A17 citation rule cannot yet be followed for them.
- **FT-A28 is filed** (≈ lines 7825–7936) and ratifies the (FT-R7) two-half
  discharge. It versions the halves rather than renumbering them — **(FT-R7a)**
  the cross-run invariant receipt against a frozen table, whose scope is
  corrected *upward* to reach both `Σ_I δ_I` and `|supp δ_I|` per unit, and
  **(FT-R7b)** the in-run reproduction receipt, which reaches every individual
  value within one process. It names the residual the conjunction does not cover
  and defers the closure — **(FT-R7c)**, a per-unit frontier digest, **not owed
  for S6k** but **binding on the next FT run that regenerates a frontier**. It
  also rules that the orchestrator's byte-diff is an **audit note and never a
  receipt**, which is what let (FT-R7c) be deferred rather than demanded. **No
  re-emission is required** and nothing is routed back to the builder.
- **Two specification defects of one shape, both found by the build.** FT-A23(v)
  and FT-A28(i) are the same error twice: **naming a relation without naming its
  relata**. A freeze clause states a constant *or* a generating rule, never both;
  and a receipt comparing against a prior run must name the **carrier** of the
  reference value — a frozen table with its provenance line, or an in-run
  recomputation — never "the previous emission" unqualified. A prior run is not
  an object and its results text is not an interface.
- **T1-A12's implementation-versus-corpus risk is now inherited by four
  families.** Every statement in T1-A, LD-A, RW-A and FT-A is proved relative to
  walt's *implementation* of the rules, read from `rules.rs` at adjudication
  time, and **no receipt inside those sections can detect a disagreement with the
  rules corpus** because every receipt is computed by that same implementation.
  The corpus check is owed before any of it is cited outside walt, and (LD-R4)
  is a probe of the risk, never a discharge of it.

## Where the rest lives

- [The freeze register](walt-math-freezes.md) — all 50 issued freezes with
  content, version and declaring ruling (39 and 40 still reserved).
- [Open questions](walt-math-open-questions.md) — what is genuinely unresolved,
  and why none of it belongs in [open-problems](open-problems.md).
- [The walt hub](walt.md) — the build map, sessions, and the exploratory fence.
  Owned by another page; this reference does not restate it.
