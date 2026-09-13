# FINDINGS — Texas 42 Foundations Ingest Review

[Home](Home.md) · owns: the overall assessment — what the object is, strongest
results, risks, next questions. Written 2026-07-26 (exhaustive read of both packages,
all verifiers executed, doc-by-doc comparison); updated 2026-07-27 with the exchange
adjudications and rob receipts; addenda 2026-08-01/-02 (the constellation exchange
batch, §4; the PA-E10 kernel tier, §4 item 3); scope note 2026-08-24, extended
2026-09-13 to every walt era page; §3's verifier caveat corrected 2026-09-13 in step
with [discrepancies](discrepancies.md) D15; §8's Q1 progress text collapsed to a
pointer at [open-problems](open-problems.md) OPEN-11 the same day. Nothing in the
assessment itself has changed since 2026-08-02. Citations per [Home](Home.md)
(v0.7 / rec).

*Scope note (2026-08-24, extended later the same day; pointer list extended
2026-09-13).* This page assesses the claim-tier mathematics only. The project's
player build — walt, primary since 2026-08-17 — lives entirely at the exploratory
tier behind [the walt hub](walt.md)'s fence, and per that fence nothing from it is
cited here. The walt-side findings are owned at that tier by the era pages, and
this paragraph is a **map pointer only** — not a citation; it quotes no number from
behind the fence, and none of the pages it names is quotable above it:
[walt-calculated-evidence](walt-calculated-evidence.md) (2026-08-24 → 08-29: the
calculated-evidence build, its shadow and field-swap instrument records, the
live-player audit findings), [walt-counted-belief-era](walt-counted-belief-era.md)
(2026-08-30 → 09-01: counted belief, the anytime proof state, the doom census),
[walt-focal-horizon-era](walt-focal-horizon-era.md) (2026-09-01 → 09-05: model
belief, the unified player, the focal-horizon hierarchy),
[walt-gran-anchors](walt-gran-anchors.md) (the 6-4 problem, exact indifference,
obligation O5), [walt-partnership-program](walt-partnership-program.md) and
[walt-gym](walt-gym.md) (2026-09-06/07), and [walt-seat-play](walt-seat-play.md)
(how the player decides — the natural map entry for the player itself). The
exploratory *questions* those programs leave open are inventoried on
[walt-math-open-questions](walt-math-open-questions.md), never on
[open-problems](open-problems.md)'s claim-tier list (that page carries only a
fenced pointer block). ChatGPT 5.6 Pro's walt-side parents (the calculated-evidence,
level-2, counted-belief, anytime, model-belief, salvation-complex and focal-horizon
notes) were hand-ferried outside the numbered courier protocol and are indexed
only on [walt-math-intakes](walt-math-intakes.md), at the exploratory tier; the
numbered dispatches this page cites are the claim-tier exchange
([exchange](exchange.md)).

*Exploratory-tier note (2026-09-13; no claim attached).* The partnership program's
"phone" reference artifact — the `walt.wasm` plunge deploys — is pinned by
`experiments/partnership/BASELINE.md` to this repository's commit `9a056f20`
(2026-08-19), identified byte-for-byte on 2026-09-06. That is a provenance fact
about an artifact below the fence, recorded here so the ledgers know the pin
exists; it establishes no result and changes no tier.

---

## 1. What this object actually is, mathematically

Not a game engine spec. It is a **formal information-theory of one imperfect-
information game**, built so that an exact solver can be *proved* correct before it is
built. The mathematical skeleton:

1. **A declaration-indexed family of finite relational algebras** over 28 stable
   nodes (`Sym²(F₇)` = looped-K₇ edges). Declaration selects the algebra; a domino's
   strategic type is relational, not intrinsic. Unique trick winner is a theorem of
   key injectivity, exhaustively cross-checked (737,100 cases).
2. **An exact epistemic layer**: the viewer's rule knowledge of three hidden hands is
   *losslessly* a dependent capacitated-matching problem (pool, allowed sets,
   quotas) — the cell losslessness theorem (CELL-05), proved by a four-case induction
   whose key observation is that a played tile is its own follower-witness, so no
   positive constraints survive; only monotone negative ones (voids) do.
3. **A canonical minimal support state**: certain-tiles + determinate/binary/ternary
   ambiguity core is *the* quotient of cell systems by support equality (CELL-14) —
   every exact deterministic representation factors onto it. Standalone it needs
   exactly 81 bits over the feasible schema (census 1.83×10²⁴ states); relative to a
   sufficient mechanical state it needs **zero** bits (derived view).
4. **A reachability theory**: legal play reaches a strict subset of feasible supports
   (explicit witness), with an exact outer necessary language giving 26 ≤ bits ≤ 46,
   exact cardinality open. rec adds a *symbolic* replay machine: the support itself
   carries the existential deal through the public trace, giving what rec calls
   deal-free "exact reachability certificates" (rec's own term, quoted; the
   wiki's name for the outer object is *necessary outer profile*, D3) and a
   finite graded support DAG.
5. **A dynamics** (rec): the minimal support state is closed under typed public
   observations via a force/delete/contract/reduce **matching-minor calculus**, with
   proved monotone edge deletion and a hard 63-edge-per-hand budget. Hidden-information
   tracking in 42 is literally a monotonically deleting 63-edge bipartite graph.
6. **A strict support/belief separation**: support selects no probability law
   (proved); belief is Bayes over an augmented latent domain; and the 90-world
   witness proves mechanical+support state is *not* a strategic state — identical
   support, identical posterior support, opposite optimal leads under four utilities.
7. **A minimality meta-theory** (rec): "minimal state" is only meaningful per output
   contract, via a mechanical Myhill–Nerode future-equivalence quotient; the reduced
   viewer kernel `(δ, hand, support-NF, folded-trick, utility-accumulator)` is proved
   exact but not yet proved equal to any quotient.

The intellectual signature throughout is **claim hygiene**: every statement carries a
status; finite receipts are never promoted to theorems; minimality claims name their
representation class; "possible ≠ probable" and "feasible ≠ reachable" are enforced by
types.

## 2. Package relationship (full analysis: [package-provenance](package-provenance.md))

**Divergent siblings of v0.6, not a linear succession.**

- **v0.7** = v0.6 + adversarial review (55) + *type-boundary repairs* (proof-irrelevant
  reachability, derived support views, total normal-form well-formedness,
  external-verification trust boundary, naming fixes) + mechanization plan (60/65) +
  continuity/provenance (70, mandate). No new mathematics; all numerics unchanged.
- **rec** = v0.6 + *major new mathematics* (unscored mechanics classes, matching
  kernel, symbolic reachability, dynamic support + monotonicity, folded trick +
  reduced kernel, D₄ gauge, future equivalence) + two new verifiers — but it still
  contains, verbatim, the v0.6 executable-spec defects v0.7 repaired
  (identity-bearing certificates, cells stored in state, "dependency-free" and
  "certificate" overclaims, broken table pipes).
- Neither references the other's unique content. The correct v0.8 is: **rec's
  mathematics under v0.7's type discipline** — that merge is what this wiki's
  [merge order](package-provenance.md) and [discrepancies](discrepancies.md) specify.

## 3. Verifier status ([verification](verification.md))

All mathematical verifiers **pass** and reproduce committed outputs byte-for-byte
(modulo the header lines in the committed transcripts): `verify_foundation` (both,
identical), `verify_minimality_and_reachability` (both), `verify_reduced_kernel`
(rec). Both MANIFESTs verify clean. One operational trap, not a package defect:
running the rec verifiers *in place* with a default `python3` invocation writes
`verification/__pycache__`, after which rec's `audit_package.py` fails its
no-transients check. The checked-in `ingest/` tree is clean and always has been —
git has never tracked a `__pycache__` directory or `.pyc` file under it (an earlier
wording of this sentence said the copies "contain" such directories; that described
a working tree after an in-place run, never the repository, and is withdrawn). Run
the verifiers from a copy or with `python3 -B`; either keeps the audit green and
reproduces `AUDIT_OUTPUT.txt` exactly ([discrepancies](discrepancies.md) D15, which
carries the measured reproduction). Reproduction requires nothing but Python 3.12
stdlib.

## 4. Strongest results (my ranking)

*Addendum 2026-08-01 (exchange batch, [claim-ledger](claim-ledger.md) rows 9–11 —
the three constellation rows x:009, x:010, x:012; "rows 9–10" until 2026-09-13):*
**C1 — suffix minimax factors through the declaration-free constellation**
(adversarially step-checked proof, external tier, Lean pending), with its
counterweight: **backward commutation for the pooled key REFUTED**
(exchange-adjudicated CONFIRMED witness) — value pools across declarations
forward, but the backward step must go through realizations. Plus the
**carrier-skeleton staircase in closed form** (x:012 CONFIRMED: a₄=37, b₄=486,
b₈=126,657, role-decorated 4,767, Σa=79,264). And **R1 — realizable = reachable
at k=1** (x:010 CONFIRMED): the retrograde seed table needs no reachability
filter (convention caveat 31,197 ordered / 15,680 swap-pooled; legal-play sense
only, no REACH-\* impact). Owning page:
[idea-retrograde-rank](idea-retrograde-rank.md) §§5,7.

1. **Cell losslessness + fixed-history bijection** (CELL-05/07) — the license for
   everything downstream: exact hidden-state tracking with no history replay.
2. **The global support quotient + 81-bit census** (CELL-14, CELL-27) — a genuine
   representation-theory result: *the* coarsest exact support state, with a
   constructive rank/unrank attaining the bound.
3. **The 90-world posterior-flip witness** (STR-06..09) — a fully legal, fully
   exhaustively-verified counterexample killing coordinate-only value; unusually
   strong because both histories keep all 90 worlds at positive mass. **Now also
   proof-assistant kernel tier** (PA-E10, 2026-08-02): internalized whole in
   `lean/Texas42/Witness.lean` — fiber equality, 90 legal replays, both posteriors,
   and the value columns from 180 kernel-evaluated rollouts, closing with
   `ninety_world_witness`. No external receipt imported (TRUST-01); its
   *minimality* remains open ([open-problems](open-problems.md)).
4. **Feasible ≠ reachable with an explicit reduced witness** (REACH-10) plus the
   **[36,45]-bit interval** (corpus-proved [26,46], narrowed to [36,45] at the
   exchange-adjudicated tier by REACH-17 + REACH-18 — combined floor 36,913,384,410 >
   2³⁵ — and REACH-19 — filtered outer census 33,297,009,347,414 < 2⁴⁵) with honest
   refusal to guess the exact count (REACH-11..13, REACH-17/18/19). The no-void
   stratum is the exception: exactly counted and saturated at **624,892,870**
   (REACH-20; panel 2/3 SOUND + 1 UNVERIFIABLE-no-defect, carried as such and never
   presented as 3/3). A second feasible-but-unreachable witness that *passes*
   lead-witness (exchange 002) shows the outer language is not even tight.
5. **rec's dynamic-support package** (TRANS-08..14): support-NF as a closed transition
   state, matching-minor update ≡ conditioning, monotone deletion, 63-edge budget —
   this is what makes an efficient exact implementation *obviously* possible.
6. **One-assignment SCC marginal compiler** (CELL-15) — replaces 63 Hall solves with
   one feasibility solve + one linear pass; the alternating-cycle proof is clean.
7. **Three unscored mechanics classes** (rec ALG-22/23) — collapses 9 declarations to
   3 for all count-blind reasoning.

## 5. Load-bearing unresolved claims

- **OPEN-11**: exact `|R_Str^m|` inside **[36,45] bits** (exchange-narrowed; corpus-
  proved [26,46]) — controls the feasibility of a precomputed reachable-support index (a
  45-bit space is enumerable; a 36-bit one comfortably so). rec's symbolic support DAG
  is the counting substrate. By the transport theorem (exchange 004) the counting DP
  need only enumerate one pip-trump class plus DT and NT — restate the feasibility
  window against `7·r_pip + |R_DT| + |R_NT|` rather than nine independent classes.
- **OPEN-01 (rec) — RESOLVED (COLLAPSE, exchange 003)**: the reduced kernel is strictly
  finer than the future-equivalence quotient for the support-aware contract, so the
  fold-ordinal coordinate is not an optimal memoization key (dead-cut lemma).
- **OPEN-12**: no support-only reachability criterion — external states must replay a
  trace (now deal-free, but still ancestry-bearing).
- **Off-path beliefs (OPEN-07)** and the **match-level horizon** (unbounded all-pass;
  termination only under an ε-assumption) — both must be *chosen*, not derived, by
  any full-match solver.
- The **retained-evidence record `e`** has no minimality theory (OPEN-02/03): a
  history-reading opponent model can force arbitrarily much history into the
  strategic state; only the support component is minimized.

## 6. Anything mathematically suspicious?

I found **no incorrect theorem**. Specific scrutiny:

- The **(6,6,6) universal-reachability pigeonhole** in the 26-bit floor (Math
  §7.13.6): a 10-tile complement with ≤2 doubles has ≥18 pip incidences over 7 pips ⇒
  some pip on ≥3 tiles ⇒ legal 3-play prefix. Checks out, including the ≥3-doubles
  branch (declare doubles).
- The **REACH-10 exhaustion** (450 static generators): its completeness relies on the
  proved fact that capacities (6,6,6) admit at most one void context, and that cells
  are determined by (declaration, context, membership pattern) — sound.
- The **strict-Hall / essential-exclusion** proofs (CELL-18/19/20) — the slack
  bookkeeping is right; the linear ternary validator's sufficiency argument holds
  because each tile excludes ≤1 seat so every 2-seat subset sees all of `W`.
- The **schedule-language theorem** (REACH-06) is carefully scoped as
  projection-exact only — it would be wrong as a physical-reachability claim, and the
  packages never claim that.

Weaker spots worth watching (asserted with prose proofs + only small-domain receipts,
not yet mechanized — the risk is a scope gap, not a known bug):

- **TRANS-08/09** (support-NF dynamic sufficiency, matching-minor ≡ conditioning):
  exhausted only on ≤4-tile supports; the slough case interacts with re-reduction in
  a way a proof assistant should re-derive carefully.
- **PLAY-17** (reduced-kernel sufficiency): the proof composes four congruences; the
  utility-accumulator interface ("supplies exactly the utility residue not represented
  as transition reward") is the kind of boundary where double-counting bugs live
  (the packages themselves flag Bellman double-counting, STR-03).
- **rec's executable spec** is internally inconsistent with rec's own math on state
  duplication (Math §7.16.4 vs Exec §15) — resolved by adopting v0.7's discipline
  ([discrepancies D2](discrepancies.md)).
- The **big census integers** (N_det/N_bin/N_ter, outer-profile counts) are verified
  by the same code that motivated them; an independent re-derivation is cheap
  insurance (Q5 below).

## 7. What an implementation ("rob") needs first

See [first-implementation-slice](first-implementation-slice.md) for the original
assignment. *Status: steps 1–5 below are executed and green (rob slices 01+02,
twelve byte-diffed receipts — [verification](verification.md)); step 6's
belief/filtering layer with the 90-world posterior regression remains for a later
slice (S3 already reproduces the witness's support side).* Order of construction,
with the reason:

1. **Slice 01 — declaration algebra** (assigned in 50_CODEX, rec superset): universe,
   nine algebras, unique winner + independent prose resolver (737,100), transports,
   3 mechanics classes, `verify_algebra.py` receipt. Everything else consumes this.
2. **Objective hand machine** with phase-indexed states and the certified-constructor
   discipline (v0.7 Exec §10/§18: reachability proof-irrelevant, no stored flags).
3. **Cells as derived views + losslessness parity harness**: replay-based equality of
   `Φ(deriveRuleCells(state))` vs deal-set image on generated prefixes (the package's
   972-prefix receipt is the model).
4. **Support normal form + SCC compiler + capacity DP + count-ratio sampler** —
   bounded, exactly testable, and the memo-key foundation.
5. **rec's dynamic layer**: matching-minor update with the 63-edge budget as an
   invariant; symbolic trace validator as the external-state gate.
6. **Belief/filtering + the 90-world regression** as a permanent guard test.

Non-negotiables from the merge: one semantic source of truth (derive, don't store,
cells/fiber/NF); equality/hashing through projected state only; every exhaustive count
in the spec becomes a CI assertion; no floats anywhere near ranks or probabilities
(exact rationals); "Empty support" is an error state for internal states (reachable ⇒
feasible).

## 8. Highest-value questions for an external strong reasoning model

Phrased as adversarial, concrete tasks (each is self-contained given the two
packages):

- **Q1 (close OPEN-11).** Design and prove correct a dynamic program over the
  symbolic play/support DAG (rec Math §7.13.7) that computes `|R_Str^m|` exactly —
  e.g. by canonicalizing states to (support-NF, folded-trick residue, capacities) and
  proving the projection's fibers countable in closed form. Deliverable: either the
  exact integer (with an independently checkable enumeration strategy) or a proved
  tighter interval than 26–46 bits. Partial credit: the exact count restricted to
  no-void states or to `j ≤ 2` completed tricks.
  *Progress (exchange-adjudicated):* narrowed to **[36,45] bits** with the no-void
  stratum exactly closed. The full statement — floor, ceiling, the stratum count,
  the 2/3-SOUND panel carried verbatim, and the transport-theorem restatement of
  the feasibility window — is owned by [open-problems](open-problems.md) OPEN-11
  and [reachability](reachability.md); §5 above gives the one-line form. (This
  paragraph duplicated OPEN-11 nearly verbatim until 2026-09-13; every figure it
  carried is on those two pages and on [exchange](exchange.md).)
- **Q2 (attack REACH-11's language).** Construct a feasible, support-reduced normal
  form that passes **all** outer necessary checks — reachable capacity shape (range
  ≤1), schedule-admissible void masks, lead-witness tiles outside the pool, Hall —
  yet is not Straight-reachable (the packages' only witness fails the lead-witness
  check, so it does not test the full conjunction). Alternatively, prove the
  conjunction is *sufficient* for the phase `j = 1`, one void context. Either outcome
  materially moves the 46-bit ceiling.
  *Resolved (exchange-adjudicated, negative):* the witness (NT, (6,6,6), V₁={6})
  passes **all four** outer checks yet is unreachable — refuting the sufficiency
  direction (B) for the `j=1` equal-capacity one-void phase. New fifth necessary
  condition established: the follower-supply obstruction (exchange 002).
- **Q3 (attack OPEN-01/PLAY-17).** Construct two distinct reduced viewer kernels
  `K₁ ≠ K₂` (same declaration and utility interface) that are future-equivalent under
  the full support-aware output contract (legality + support output + trick reward +
  terminal label) — or prove `K` is injective up to future equivalence, i.e. the
  kernel *is* the Myhill–Nerode quotient. Note the candidate collapse directions:
  utility-accumulator redundancy, fold-ordinal coincidences across contexts, and
  unscored-transport coincidences.
  *Answered (exchange-adjudicated):* the requested pair exists, is reachable, and is
  machine-verified — two distinct reduced kernels (18-tile full-Ternary support normal
  form, identical `ε`, `r0=r1=6`) that are future-equivalent under the support-aware
  contract, so the kernel is strictly finer than the Myhill–Nerode quotient (COLLAPSE),
  via the fold-ordinal / dead-cut mechanism (exchange 003).
- **Q4 (gauge-reduce the census).** Prove or refute: the order-preserving complement
  transport `f_{t,u}` (rec Math §3.10) maps `R_Str^m` for pip trump `t` bijectively
  onto that for `u` — i.e. legal-prefix generation commutes with unscored transport.
  If true, the declaration-tagged reachable census collapses from 9 tags to 3
  classes, simplifying Q1 and shaving the outer-profile count; if false, exhibit a
  reachable support whose transport is unreachable (that would be a *very*
  interesting asymmetry, since support semantics is count-blind).
  *Answered (exchange-adjudicated, affirmative):* `f_{t,u}(R_t)=R_u` is machine-certified
  — the transport commutes with legal-prefix generation, collapsing the census from 9
  tags to 3 classes; the Step-15 cocycle gap is closed by finite check over all 343
  triples (exchange 004, `programs/004-cocycle.py`).
- **Q5 (independent audit of the load-bearing integers).** Re-derive from scratch,
  without consulting the Python, the four census families: (a) the full-schema counts
  N_det = 8,102,258,940,222,814, N_bin = 11,495,078,055,913,018,482,
  N_ter = 1,830,955,704,129,296,418,354,864 (Math §7.12.5 formulas); (b) the
  outer-profile totals 7,124,838,074,989 and 64,123,542,674,901 (Math §7.13.6
  `B_{n,u}`/`C(k)` sums); (c) the 44,352,165 floor and the disjointness of its four
  families; (d) the 136,514 / 23,842 / 1,667,666 / 296,721 / 279,048 signature-census
  chain. Report any integer that does not reproduce, since scripts and text share
  provenance and could share an error.

(Secondary, if capacity remains: prove or refute minimality of the 90-world flip
witness; and formalize the match process to characterize exactly which policy classes
violate almost-sure termination without the ε-assumption.)

Of these, Q2/Q3/Q4/Q5 are answered (exchange 002/003/004/005, all CONFIRMED); Q1 is
narrowed to [36,45] (floor: exchange 001+006; ceiling: exchange 007, the first
ceiling movement) with its no-void stratum exactly closed at 624,892,870 (exchange
008). The wiki page map lives on [Home](Home.md).
