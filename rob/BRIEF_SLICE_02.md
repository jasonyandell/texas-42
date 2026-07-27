# rob — Implementation Brief, Slice 02

This is the definitive second implementation assignment for **rob**, this repository's
exact Texas 42 engine. It extends [BRIEF.md](BRIEF.md) (slice 01), whose §1 governing
synthesis, §2 toolchain, §5 invariants INV-1..10, §9 receipt/CI rules, and §11
ambiguity protocol **remain binding verbatim**. Slice 01 is green and committed
(`rob/receipts/`, recorded in [wiki/verification](../wiki/verification.md)
§"rob (Rust) — independent reproduction, slice 01"); slice 02 begins only from that
state.

Scope, in one sentence: **the dynamic support layer and symbolic reachability** —
the matching-minor calculus as the exact support transition on the normal form,
monotone holder-edge deletion with the 63-edge budget as a runtime invariant, the
symbolic trace validator as the external-state gate, the necessary outer language
with its exact censuses, two unreachability regressions, and transport-aware census
plumbing.

Citation convention as in BRIEF.md. One addition: **x:NNN** cites an
exchange-adjudicated result by ledger number in
[exchange/README.md](../exchange/README.md) (response text in
`exchange/inbox/NNN-*.md`).

---

## 1. Governing synthesis, extended (already decided — do not relitigate)

Everything in BRIEF.md §1 stands: rec's mathematics under v0.7's type discipline;
D1–D15 pre-resolved; rob never redefines a mathematical object. D5 now activates:
slice 01 was forbidden from exposing any transition API on standalone support; slice
02 implements exactly the typed calculus TRANS-08 licenses, and nothing untyped.

### 1.1 The exchange tier (new)

On 2026-07-27 five adversarial dispatches to an external strong reasoning model were
answered and adjudicated **CONFIRMED** — each by unmodified program execution
(`exchange/adjudication/programs/`) plus a 3/3 adversarial referee panel. Ledger:
[exchange/README.md](../exchange/README.md). Their standing, binding on rob:

> **"Exchange-adjudicated CONFIRMED" is citable for test expectations and
> constructions, but it is NOT a corpus theorem.** It ranks below ingest in the
> authority order and is never definitional. Where rob independently reproduces such
> a number in Rust, rob's receipt becomes *independent evidence* for it — record
> that in the final report for the wiki track; it still changes no claim's status.

The five results and what slice 02 does with each:

1. **x:001 — reachable-support floor 17,668,066,045.** A certified disjoint family
   of reachable supports > 2³⁴, improving the standalone interval from [26,46] to
   **[35,46] bits**. The witness-generation + split-zeta upward-closure counting
   machinery is validated and reusable. → optional stretch stage S10.
2. **x:002 — the four-check outer language is NOT tight.** A witness at
   (NT, capacities (6,6,6), V₁ = {6}) passes all four outer necessary checks —
   including lead witness — yet is unreachable under every declaration (450 static
   generators, 425,520 shallow traces, 0 realizers). A **fifth necessary condition**
   (follower-supply obstruction) explains it. → S7 (shallow check) + S8 (permanent
   second unreachability regression).
3. **x:003 — OPEN-01 resolved: COLLAPSE.** The reduced kernel's raw fold ordinal is
   not injective up to future equivalence under the support-aware output contract
   (dead-cut lemma: ordinals above the live cut of still-unplayed competitive tiles
   are unobservable until trick-boundary coalescence). → §6 fold-ordinal design
   note, binding on later slices' memo keys; **no slice-02 code**.
4. **x:004 — transport commutes with reachability:** `f_{t,u}(R_t) = R_u` for all
   ordered pip-trump pairs. Reachable-census work needs only **3 declaration
   classes**, not 9. The response's Step-15 overlap corollary is CONDITIONAL on an
   unproved cocycle lemma and is **excluded**. → S9.
5. **x:005 — census integer audit clean.** All 19 load-bearing census integers
   independently reproduced; **no corpus corrections needed** — slice-01 receipts
   stand untouched. A new Burnside decomposition (136,514 / 2,156 / 35 → 23,842)
   is adopted as a cheap supplementary assertion. → S7.

Authority order for slice 02, highest first:

1. this brief together with BRIEF.md (decisions and scope);
2. `wiki/` (reconciliation, resolutions, [support-dynamics](../wiki/support-dynamics.md),
   [reachability](../wiki/reachability.md));
3. the ingest packages (definitions, theorems, claim IDs — immutable);
4. exchange-adjudicated results (test expectations and constructions only,
   tier-labeled per §9.1).

If rob's implementation appears to **refute** an exchange-adjudicated result (a
realizer for the x:002 witness, a transport non-commutation, a floor-family
collision), that is not a bug to code around: stop the affected stage, keep the
failing test, and report immediately (§11) — it would be a finding of the first
order.

---

## 2. Language and toolchain (unchanged)

Rust, pinned stable toolchain, exactly as BRIEF.md §2: runtime dependencies of the
core crate remain `num-bigint`, `num-rational`, `num-integer`, `num-traits` and
nothing else; dev-dependency `proptest`; `#![forbid(unsafe_code)]`; clippy at
`deny(warnings)`; **no floats anywhere** (INV-4 grep + clippy `float_arithmetic`
deny + overflow checks). New magnitudes: every slice-02 count except the S4-style
big censuses fits `u64` (64,123,542,674,901 < 2⁴⁶); use `u64` for enumeration
tallies and `BigUint` where formulas are evaluated (outer-profile sums, B-table),
mirroring the S4 rule that census integers are *computed from the formulas, never
hard-coded*.

---

## 3. Project layout (additions only)

The BRIEF.md §3 layout stands. Slice 02 adds modules under
`rob/crates/core/src/support/`, one verify module + binary per stage, and committed
receipts (§14 module map). `rob/` remains the only directory rob writes; `wiki/`,
`ingest/`, and `exchange/` are read-only to rob. The existing `rob/crates/player`
may be touched **only** for the mechanical refactor in S5 (consume the new native
sampler); no player feature work in this slice.

---

## 4. Scope: slice 02 and its boundaries

Five gated stages plus one stretch stage. A stage begins only when the previous
stage's receipts are green and committed.

- **S5 — Matching-minor calculus**: the abstract typed transition on the normal
  form (force/delete/contract/reduce), extensional-conditioning equivalence,
  monotonicity, the game-typed wrapper, and the `sample_native_world` core-surface
  addition (TRANS-08..13).
- **S6 — Symbolic trace validator + full-hand dynamics**: deal-free reachability
  certificates as the external-state gate; 108-hand corpus; the 63-edge budget
  exercised end-to-end (REACH-14/15/16, TRANS-14).
- **S7 — Necessary outer language + censuses**: schedule language, lead-witness
  coefficients, per-profile `C(k)` counts, the five-check necessary-profile
  validator, ceilings, Burnside supplement (REACH-06/06A/07/11; x:002, x:005).
- **S8 — Unreachability regressions**: REACH-10 reproduced; the x:002 witness as a
  second, stronger regression with the follower-supply check.
- **S9 — Transport-aware census plumbing**: the 3-class declaration quotient for
  reachable-census work and a corpus transport-commutation receipt (x:004).
- **S10 (stretch, optional)** — the x:001 reachable floor reproduced in Rust:
  17,668,066,045 and the [35,46] interval. Not required for slice-02 done-ness.

**Explicitly out of slice 02** (do not begin, do not scaffold speculative APIs for):

- **Slice 03** — folded trick and reduced viewer kernel (PLAY-12..17): 737,100
  trick cases / 2,211,300 sequential updates, 84 open-trick shapes, 3,132
  score-recovery prefixes, 8 dihedral frames, future-equivalence corpus
  5,898 / 17,560 — including the x:003 `r = 7` vs `r = 6` collapse witness as a
  regression (it needs the fold to exist) and any live-cut quotient key type (§6).
- **Slice 04** — belief and filtering; the 90-world posterior flip.
- **Slice 05** — solver and census frontier: enumeration of the symbolic support
  DAG, any dynamic program toward exact `|R_Str^m|` (OPEN-11), memoization on the
  reduced kernel. S6 implements the *validator*, not the DAG; no DAG node type, no
  graded enumeration API.
- The **general (all-phase) formalization of the fifth condition** — S7 implements
  exactly the shallow-phase check the x:002 proof establishes, plus the witness
  regression; generalizing it is exchange/wiki work, not rob's.
- **x:004's Step-15 overlap corollary** (conditional on the unproved cocycle
  lemma) — the 3-class quotient is in scope; cross-class overlap arithmetic is not.
- Auction solver, CLI, UI, optimization caches before correctness — never.

---

## 5. Named invariants

INV-1 through INV-10 are inherited unchanged, with their existing enforcement.
Slice 02 extends INV-6's reading: the abstract matching-minor calculus and the
symbolic validator run on the **total** NF type (`Empty` is a value on the
validation/abstract path); on certified internal states an empty successor remains
a panic, never a value. Four new invariants:

- **INV-11 EDGE-BUDGET.** The dynamic holder graph only deletes: within one
  contracted attempt no holder edge ever reappears; each of the initial
  21 × 3 = 63 edges dies exactly once; at most 2 of a tile's edges die while the
  tile is live (≤ 42 informational deletions per hand); a completed hand's deletion
  ledger totals exactly 63. *Enforcement:* the transition maintains a deletion
  ledger in a separate audit type outside semantic equality (INV-1/INV-2
  discipline); `debug_assert!` in the update that every successor holder set is a
  subset of its predecessor; test `inv_edge_budget` over the S6 corpus asserts
  6,804 = 108·63 with zero reappearances. (TRANS-10/12/14.)
- **INV-12 MONOTONE-AMBIGUITY.** Certainty never reverts to ambiguity; a seat with
  zero residual ambiguity capacity never re-enters the ambiguity component; the tag
  moves only `Ternary → Binary → Determinate`. *Enforcement:* `debug_assert!` on
  every nonempty successor in `dynamics.rs`; asserted exhaustively in
  `r_dyn_monotone` (157,809 cases) and along the S6 corpus. (TRANS-11.)
- **INV-13 TYPED-TRANSITION-ONLY.** There is no untyped transition on standalone
  support. The abstract calculus takes an explicit `TypedHiddenObservation` (kind +
  abstract follow set); the game-level observation is constructible **only** from
  declaration + led context + play via the declaration algebra. No method on the NF
  type consumes a bare domino. *Enforcement:* type system; `compile_fail` doctest
  showing `nf.transition(domino)` does not exist; review. (D5; TRANS-08;
  REACH-03A.)
- **INV-14 FIVE-CHECKS-STILL-NECESSARY-ONLY.** The outer validator — now five
  checks — returns profile membership only; there is no path from any outer check
  to a certified or reachable type, and the x:002 witness is the permanent
  regression that passing every implemented outer check does not imply
  reachability. *Enforcement:* INV-9's negative compile-test extended to the new
  surface; `r_unr_002_*` green forever; the INV-10 vocabulary grep continues to
  forbid certificate-style identifiers. (D3; REACH-10; x:002.)

All standing guardrails from BRIEF.md §5 continue, with one clarification: the
"never copy verifiers" rule extends to the exchange Python
(`exchange/adjudication/programs/`, inbox code blocks). rob re-implements
constructions from their prose proofs and JSON witness/tables; transcribing
*witness data, corpus-shape parameters, and expected numbers* from the exchange
files is permitted exactly as it is for ingest receipts; transcribing their program
logic is not.

---

## 6. Fold-ordinal design note (FOLD-KEY — decided, binds slices 03+)

x:003 (adjudicated CONFIRMED) proves the reduced kernel's raw fold ordinal `r` is
**not** injective up to future equivalence for the support-aware output contract
with the named `P30_DECLARING_POINTS` accumulator: two reachable kernels differing
only in `r` (7 vs 6, NT context 6) are future-equivalent, by the **dead-cut
lemma** — if the sets `{d live : ord(d) > r₁}` and `{d live : ord(d) > r₂}` are
equal, the two folds make identical winner-update decisions until the ordinals
coalesce or the trick boundary discards `r` entirely. Only the ordinal's cut
through the still-live competitive tiles is observable. Decisions:

1. The semantic folded-trick type (slice 03) stores the **raw** ordinal `r` exactly
   as rec Math §7.16.1 defines it. It remains the semantic source of truth;
   `Eq`/`Hash` of semantic state stay on raw projected fields (INV-2). rob does not
   redefine the fold.
2. Any memoization/transposition key over kernel states (slices 03/05) **may**
   additionally quotient `r` to its live-cut re-ranking (the dead-cut compression),
   but **only** as a separate key type computed by a total function of the semantic
   state, introduced together with a proved-equivalence test obligation (equal
   compressed keys ⇒ identical output traces along the named contract, exercised
   exhaustively on a committed corpus) — exactly parallel to BRIEF.md §5's
   bitmask-codec rule: refinements enter as proved-equivalent separate types, never
   as replacements (D1/D2 discipline).
3. The compression is licensed **per output contract and accumulator interface**.
   x:003 establishes it for the support-aware contract with
   `P30_DECLARING_POINTS`; any other interface requires re-checking the lemma's
   hypotheses before reuse.
4. The `r = 7` vs `r = 6` witness becomes a permanent regression when slice 03
   implements the fold. Nothing in slice 02 implements any of this; this section
   exists so the slice-02 dynamics APIs are not shaped in a way that would force
   the raw ordinal out of the semantic state later.

---

## 7. Reading list for the implementer

1. BRIEF.md in full (it still governs), then this brief in full.
2. [wiki/support-dynamics](../wiki/support-dynamics.md),
   [wiki/reachability](../wiki/reachability.md),
   [wiki/open-problems](../wiki/open-problems.md),
   [wiki/reduced-viewer-kernel](../wiki/reduced-viewer-kernel.md) (context for §6
   only), [wiki/verification](../wiki/verification.md).
3. rec Math §7.13 (reachability, incl. §7.13.3 schedule language, §7.13.5 witness,
   §7.13.6 outer counting, §7.13.7 symbolic replay) and §7.14 (dynamics,
   §7.14.1–7.14.2) — the definitions rob implements; v0.7 Math §7.13 for the shared
   parts; v0.7 Exec §17–§18 for the external-validation trust boundary.
4. [exchange/README.md](../exchange/README.md) ledger; then inbox/002 (witness JSON
   + proof steps 1–13), inbox/004 (theorem statement + Step-17 scope boundary), and
   for S10 inbox/001 (proof steps 1–14 + JSON tables). inbox/003 only for §6
   context; inbox/005 for the Burnside decomposition.

---

## 8. Required API surface (semantic, Rust spelling free)

- **dynamics** (`support/dynamics.rs`): `TypedHiddenObservation` — kind
  `Lead | Follow | Slough` with an abstract follow set (INV-13);
  `matching_minor_update(nf, seat, tile, observation)` on the total NF type —
  force edge `d→s`, for a slough delete every edge `e→s` with `e` in the follow
  set, contract the played tile (remove `d`, decrement the seat's quota),
  recompile the matching-supported core (one assignment + one SCC pass, reusing
  S4's compiler), re-encode (TRANS-09); the certified-path variant returning
  nonempty-or-panic (INV-6); the deletion-ledger audit type (INV-11);
  `game_observation(declaration, led_context_or_boundary, actor, tile)` building
  the observation from the algebra with follow set `σ̂_q^δ ∩ U` (TRANS-08); viewer
  plays are the identity on hidden support.
- **cells** (addition): `sample_native_world(&RuleDerivedCellSystem, &mut dyn
  ExactRationalChoiceSource) -> RemainderWorld` — the exact count-ratio sampler
  lifted through the native offset↔`DominoId` bijection, so callers stop
  re-implementing the translation (slice-01 player-report ergonomic finding;
  CELL-10E/F). `rob/crates/player/src/worlds.rs` is refactored to consume it.
- **symbolic** (`support/symbolic.rs`): `SymbolicTraceCertificate` — viewer hand,
  contract/declaration, first leader, attributed public trace, claimed final NF;
  `validate_symbolic_trace(...)` — start from the unrestricted 21-tile support,
  accept a hidden action iff its typed conditioned successor support is nonempty,
  accept viewer actions iff legal in the known hand, require the final support to
  equal the claimed NF; on acceptance the certificate is erased and the result
  enters through the existing exact-validation path (REACH-14/15; D1; v0.7 Exec
  §18). Rejections carry a typed reason. This is the **only** gate by which a
  foreign support claim is accepted (OPEN-12: it still carries legal ancestry —
  no support-only criterion exists).
- **outer** (`support/outer.rs`): the schedule-language predicate and its projected
  censuses (REACH-06/06A); lead-witness check (REACH-07); the `B_{n,u}`
  lead-witness coefficient table and per-profile `C(k)` counts over the 50 capacity
  profiles, evaluated from the Math §7.13.6 formulas in exact arithmetic
  (REACH-11); the shallow-phase follower-supply check (x:002; documented
  exchange-tier, necessary-only); the five-check
  `ReachabilityOuterNecessaryProfile` validator (INV-14; D3 naming).
- **transport** (`support/transport_reach.rs`): trace transport under
  `pip_trump_transport` (S1's `f_{t,u}` extended pointwise to hands, plays, and
  contexts with called context fixed); `reachable_census_class(declaration)` — the
  3-class quotient for reachable-census work, consistent with S1's
  `unscored_mechanics_class` (x:004). Doc comments must state the Step-17 scope
  boundary: the transport preserves the unscored surface only — never count
  labels, trick points, contract outcomes, or anything score-conditioned.
- **floor** (stretch, `support/floor.rs`): admissible-module language, backward
  stitching, one-context cells, the three witness-family generators, and the split
  14+14 subset/superset-zeta upward-closure counter (x:001).

---

## 9. Verification harness — receipts rob must reproduce

Every row is a named test asserting the exact number and a line in the stage's
receipt binary. Corpus-anchored numbers come from the committed ingest verifier
outputs (re-confirmed byte-for-byte 2026-07-26); exchange-derived numbers carry the
`x-` receipt prefix (§9.1).

### 9.1 Receipt tier labeling (new rule)

Receipt lines whose expected value originates from an exchange-adjudicated result
are printed with an `x-` prefix (e.g. `x-002-realizers: 0`), and the receipt header
names the ledger entries it draws on (`# exchange: 002 (CONFIRMED 2026-07-27)`).
Corpus-anchored lines keep the slice-01 style. Weakening either kind is forbidden
(INV-5). This makes the evidentiary tier machine-visible in the committed receipt,
and makes rob's green `x-` lines identifiable as independent evidence for the
exchange results (§1.1).

### S5 — `verify_dynamics` (binary), tests `r_dyn_*`

The dynamics corpus, closed-form (extends the S4 tiny corpus by n = 0 and quotients
by NF equality): for each universe size n ∈ {0,1,2,3,4}, all (2ⁿ)³ allowed-subset
triples × all capacity triples with k₀+k₁+k₂ = n — 66,969 systems (66,968 + the
single empty system), of which 14,579 are feasible; their support normal forms
quotient to exactly **1,331 distinct feasible NFs** (rec Math §7.14.2's "canonical
universes of sizes zero through four"). Observation space, closed-form: for each of
the 1,331 NFs with reduced holder graph over universe `U`, for each hidden seat `s`
and each tile `d` marginally possible at `s`, one **lead** observation plus, for
every subset `F ⊆ U` (all 2^|U|, including ∅ and sets containing `d`), a **follow**
observation if `d ∈ F` else a **slough** observation (conditioning per Math
§7.14.1: possession for lead/follow; possession + complete void of `F` for slough).
Successor system: universe `U∖{d}`, actor capacity decremented.

| Test | Assertion | Exact numbers | Source |
|---|---|---|---|
| `r_dyn_corpus` | dynamics corpus sizes and distinct-NF census as defined above | **66,969**; **14,579**; **1,331** | TRANS-13; Math §7.14.2 |
| `r_dyn_observations` | matching-minor update (force/delete/contract/reduce) ≡ NF of extensional conditioning + pushforward, on every typed observation, including empty successors (both routes agree on `Empty`) | **170,058** observations; **157,809** nonempty successors | TRANS-08/09/13 |
| `r_dyn_monotone` | on every nonempty successor: every surviving tile's holder set ⊆ its predecessor (3 per-seat inclusion checks per tile); ambiguity rank never increases; inactive seats never reactivate | **1,406,592** edge checks; **157,809** rank checks | TRANS-10/11/13 |
| `r_dyn_typed_wrapper` | along the S3 parity corpus: the initial 63-edge support evolved purely by game-typed observations (`game_observation`, viewer plays as identity) equals the NF recompiled from derived cells after every play | **864** transitions (**648** hidden, **216** viewer) | TRANS-08; CELL-05 |
| `r_dyn_native_sampler` | `sample_native_world` ≡ abstract `sample_uniform_world` through the offset bijection with an identical deterministic exact-rational source, on every S3 parity state; sampled world is in the fiber | **972** agreements | CELL-10E/F |

### S6 — `verify_symbolic` (binary), tests `r_sym_*`

Corpus: rob's own deterministic 108-hand generator from S3 (9 declarations × 12
contracted hands), replayed in full (28 plays per hand). The rejection battery is
constructive and generator-independent: per hand, (m1) replace play 28's tile with
a tile the same actor already played — forces an empty conditioned successor /
duplicate play; (m2) replace one viewer play with a tile the viewer already played
in an earlier trick — viewer illegality; (m3) claim the initial unrestricted NF as
the final NF — final-support mismatch (the true final hidden pool is empty).

| Test | Assertion | Exact numbers | Source |
|---|---|---|---|
| `r_sym_corpus` | every hand's public trace is accepted by `validate_symbolic_trace`; at every transition the symbolic support, the S5 dynamics evolution, and the NF of `derive_rule_cells` on the mechanical state agree three ways | **108** hands; **3,024** transitions; **3,024** three-way agreements | REACH-14/15; TRANS-08 |
| `r_sym_budget` | per-hand deletion ledger totals exactly 63; grand total 108·63; no edge reappears; no tile loses more than 2 edges while live (INV-11) | **6,804** | TRANS-12/14 |
| `r_sym_reject` | all mutated certificates rejected with the expected typed reason | **324** = 3 × 108 | REACH-14; Exec §18 |

### S7 — `verify_outer` (binary), tests `r_out_*`

Definitions from Math §7.13.3 (schedule language over the 21 seat×context void
slots) and §7.13.6 (`B_{n,u}` lead-witness coefficients; per-profile `C(k)`), with
the 50 capacity profiles from S4. All big values computed from the formulas in
exact arithmetic, never hard-coded.

| Test | Assertion | Exact numbers | Source |
|---|---|---|---|
| `r_out_schedule` | projected schedule censuses for j = 0..7 | `A_j` = **(1, 50, 1079, 13084, 97119, 450066, 1273609, 2097152)**; `T_{j,1}` = **(8, 323, 5524, 51759, 286770, 947017, 1817216, 2097152)**; `T_{j,2}` = **(22, 743, 10844, 88159, 428562, 1244937, 2080768, 2097152)** | REACH-06/06A |
| `r_out_lead_witness` | `B_{n,u}` by two independent routes (inclusion–exclusion sum with out-of-range binomials zero, vs polynomial convolution) agree on the whole table | **176** = 22 × 8 entries | REACH-07/11; Math §7.13.6 |
| `r_out_profiles` | Σ over the 50 profiles of `C(k)` per declaration; × 9 declaration tags; max single-profile block; ceiling widths standalone / declaration-supplied / capacities-supplied / both | **7,124,838,074,989** (< 2⁴³); **64,123,542,674,901** (< 2⁴⁶); **839,220,930,919** (< 2⁴⁰); **46 / 43 / 43 / 40 bits** | REACH-11/11A |
| `r_out_five_checks` | the five-check validator: capacity shape (max−min ≤ 1), schedule admissibility, lead witness, Hall, shallow-phase follower supply — returns necessary-only membership; no conversion to certified types compiles (INV-14) | — | REACH-04/06/07; CELL-09; x:002 |
| `r_out_burnside` | S₃ fixed-signature counts and the Burnside average reproducing the S4 orbit census | x- **136,514** (identity); x- **2,156** (each transposition); x- **35** (each 3-cycle); (136,514 + 3·2,156 + 2·35)/6 = **23,842** | CELL-21/22; x:005 |

The interval receipt line remains **26..46 bits** (corpus-anchored REACH-12/13);
the [35,46] line belongs to S10 only, because rob prints only what rob computed.

### S8 — `verify_unreachable` (binary), tests `r_unr_*`

Static generator space at capacities (6,6,6), closed-form: 9 declarations × (1
no-void generator + 7 void contexts × 7 nonempty hidden-seat membership patterns)
= 9 · 50 = **450**; each decodes through rob's own cells/NF pipeline (Math
§7.13.5). The x:002 witness data (declaration NT, capacities (6,6,6), V₁ = {6},
the 18-tile pool `U = {6:0..6:5} ⊔ {0:0, 1:0, 1:1, 2:0, 2:1, 2:2, 3:0, 3:1, 3:2,
3:3, 4:0, 4:1}`) is transcribed from the inbox/002 JSON as witness data. Shallow
trace space per static match, closed-form (x:002 steps 7–12): the 10-tile
complement supplies the three hidden seats' distinct played tiles in 10·9·8 = 720
ordered ways (the remaining 7 tiles are the viewer's initial hand); prefix
skeletons: 1 (three hidden plays only) + 4·7 (leader × viewer tile, one completed
trick) + 4·7·6 (leader × ordered viewer-tile pair, completed trick + viewer lead)
= 197; replay checks actor order, viewer follow legality, declaration-relative
winner, derived voids, Hall, final NF equality on the S2 machine.

| Test | Assertion | Exact numbers | Source |
|---|---|---|---|
| `r_unr_reach10` | the REACH-10 witness (18-tile pool `σ₀ ∪ doubles ∪ {2:1, 3:1, 3:2, 4:1, 4:2}`, `P₁ = U∖σ₀`, `P₂ = P₃ = U`): Hall-feasible, already reduced; exactly 2 of 450 generators decode to it (zeros-trump called context; NT context 0; each with only hidden seat 1 void), lead-fiber sizes (7,1), and in both the entire lead fiber is inside the hidden pool ⇒ fails lead-witness necessity | **450**; **2**; **(7, 1)** | REACH-10; Math §7.13.5 |
| `r_unr_002_outer` | the x:002 witness NF passes **all four** classic outer checks via rob's own S7 validators (capacity shape 0 ≤ 1; one used context ≤ j = 1 admissible; lead witness `6:6 ∉ U`; Hall) and is Hall-feasible and already reduced | x- 4/4 pass | x:002 |
| `r_unr_002_static` | exactly 3 of the 450 generators decode to the witness NF: sixes-trump called context, doubles-trump context 6, NT context 6 — each with only hidden seat 1 void; the doubles-trump match already violates lead-witness necessity (`L_{DT,6} ⊆ U`) | x- **3**; x- 1 lead-witness kill | x:002 |
| `r_unr_002_traces` | complete shallow-prefix exhaustion over the remaining matches: 3 × 720 × 197 candidates, zero realizing traces ⇒ the witness is unreachable under every declaration | x- **425,520**; x- **0** | x:002 |
| `r_unr_002_supply` | the follower-supply obstruction, exhibited: for both surviving candidates the effective follow set has exactly one member outside the pool (`F ∖ U = {6:6}`), but any realizing trick needs two distinct public followers of that context (the lead plus one non-void hidden follower) | x- **1** < 2, both candidates | x:002 |

The completeness of the shallow search (why no deeper prefix can realize the
witness) is x:002's steps 7–9, resting on corpus theorems (capacities ⇒ each
hidden seat played exactly once; CELL-14 generator identification); rob's receipt
reproduces the search and carries the exchange tier for that completeness claim.

### S9 — `verify_transport` (binary), tests `r_tra_*`

Corpus: the 12 S3 hands of each source pip trump `t`, under all 49 ordered
pip-trump transports `(t,u)` (including the 7 identities). Counts are
corpus-shape-forced: 49 × 12 = 588 transported hands; × 28 plays = 16,464
transitions; × 29 prefix depths (0..28) = 17,052 NF comparisons.

| Test | Assertion | Exact numbers | Source |
|---|---|---|---|
| `r_tra_class_quotient` | `reachable_census_class` is constant on the 7 pip trumps and yields exactly 3 classes, agreeing with S1's `unscored_mechanics_class` partition | **3** | rec ALG-22/23; x:004 |
| `r_tra_corpus_commutation` | for every transported hand: the transported complete prefix is legal on the S2 machine; every transported transition is accepted by the S6 symbolic validator; and at every depth `f(N_t(prefix)) = N_u(f(prefix))` — transport commutes with the support normal form | x- **588** legal hands; x- **16,464** accepted transitions; x- **17,052** NF equalities | x:004 |
| `r_tra_unscored_only` | doc/negative test: no API transports count labels, trick points, or score-conditioned objects (a `compile_fail` doctest on the forbidden conversion) | — | x:004 Step 17 |

These finite receipts are conformance evidence for the x:004 theorem
(`f_{t,u}(R_t) = R_u`), not a proof of it; the 3-class census plumbing cites the
ledger entry in its doc comment.

### S10 (stretch) — `verify_floor` (binary), tests `r_flo_*`

Optional; slice 02 is complete without it (§13). Re-implement the x:001
construction from inbox/001's proof steps 1–14 and JSON tables (never from its
Python): admissible modules and backward stitching, the pair/m1/m2 trace
templates, one-context cells, the K8 vertex transport (declaration-0 module
language → the other six pip trumps and, via the 0↔D vertex swap, doubles trump),
the natural-context transport with its (7,7,7,7,7,7) histogram, and the exact
meet-in-the-middle upward-closure counter (28-bit masks split 14+14; low-half
subset zeta, then bit-parallel high-half superset zeta; popcount masks impose the
size and feasibility ranges). Cost assessment: the validated Python runs ≈ 40 s /
< 1 GB; the Rust tables are two 2¹⁴ × 2¹⁴ bit matrices (32 MB each) — minutes-scale,
but the module/template language is a substantial implementation, hence stretch.

| Test | Assertion | Exact numbers | Source |
|---|---|---|---|
| `r_flo_modules` | admissible-module receipts: declaration/group/desired-winner cases; one-context capacity/exclusion profiles | x- **3,808** = 8·119·4; x- **216** | x:001 |
| `r_flo_families` | the three pairwise-disjoint reachable subfamilies: unrestricted (no-void, incl. the four REACH-12 rows + the \|T\| = 11 row 3·C(28,11) = 64,422,540), called-suit void, natural-suit void | x- **559,316,142**; x- **8,387,350,664**; x- **8,721,399,239** | x:001 |
| `r_flo_total` | grand total; strict comparison against 2³⁴; resulting interval | x- **17,668,066,045** > 2³⁴ ⇒ floor **35 bits**; interval **[35,46]** | x:001; REACH-11 |

On green, rob's receipt is independent evidence for the [35,46] interval — report
it for the wiki track; the corpus-anchored interval statement (REACH-12/13) is
updated by the wiki, not by rob.

### Numbers that are *not* slice-02 targets

Folded-trick and kernel receipts (737,100 / 2,211,300; 84; 3,132; 8; 5,898 /
17,560), the 90-world posterior flip, the x:003 collapse witness replay, exact
`|R_Str^m|` or any tighter interval beyond S10's, x:001's unobtained `B1`/`B2`
sub-targets, and all belief/filtering numbers.

---

## 10. Receipts and CI

As BRIEF.md §9, extended: each new stage binary prints its deterministic receipt
(`rob <stage> verification: PASS` + `name: exact-number` lines, `x-` prefix per
§9.1); receipts committed under `rob/receipts/` and byte-diffed in
`rob/ci/check.sh`, which now also runs the new binaries. Exhaustive tests run in
full in CI; every S5–S9 workload is minutes-scale (the Python analogues run in
minutes; S8's 425,520 shallow replays are small). S10, if built, joins check.sh
like any other stage.

---

## 11. Pre-resolved discrepancy decisions

1. **S5 observation-space mismatch.** The corpus and observation space are stated
   closed-form in §9/S5. If rob's principled enumeration of *that stated space*
   does not reproduce 170,058 / 157,809 / 1,406,592 exactly: do **not** tune the
   space to hit the numbers; freeze rob's counts in the receipt, keep the
   equivalence/monotonicity assertions exhaustive over the stated space, and file
   `ambiguity_trans13_observation_space` with the definitional gap (the ingest
   claim ledger itself labels this corpus "stated by verifier"). The 1,331
   distinct-NF census is *not* covered by this escape hatch — it is derivable from
   the S4 corpus and must match exactly.
2. **Refutation of an exchange result.** A realizer among the 425,520 traces, a
   transport non-commutation, or a floor-family collision stops the stage per §1.1
   — keep the failing test, report, continue on unaffected stages.
3. **Corpus-shape-forced counts** (3,024; 6,804; 324; 588; 16,464; 17,052; 972;
   864/648/216) are hard assertions; they are arithmetic consequences of rob's own
   frozen S3 generator and the stated formulas, and any miss is a bug, not an
   ambiguity.
4. **Schedule/outer formula readings.** If Math §7.13.3/§7.13.6 admit two readings
   that change any S7 integer, that is a new ambiguity (§12) — the committed
   tuples/totals above adjudicate which reading the corpus intends, but rob records
   the ambiguity rather than silently choosing.
5. **Witness-data transcription.** The x:002 pool/NF JSON and x:001 JSON tables are
   witness *data* — transcribing them is permitted and does not weaken
   independence; deriving them programmatically is not required.

---

## 12. Ambiguity protocol

Unchanged from BRIEF.md §11, with §11.2 above as the one addition (exchange
refutations are findings of the first order and are reported immediately, not
batched into the final report).

---

## 13. Definition of done — slice 02

1. **Layout**: additions match §14; nothing from slices 03–05 (§4's exclusion
   list); `wiki/`, `ingest/`, `exchange/` untouched.
2. **Green**: `rob/ci/check.sh` end-to-end including the five new receipt binaries
   byte-identical to `rob/receipts/` (S10's included iff built).
3. **Every number**: 66,969 / 14,579 / 1,331; 170,058 / 157,809; 1,406,592;
   864/648/216; 972; 108 / 3,024 / 6,804 / 324; the three schedule 8-tuples; 176;
   7,124,838,074,989 / 64,123,542,674,901 / 839,220,930,919; 46/43/43/40;
   136,514 / 2,156 / 35 → 23,842; 450 / 2 / (7,1); 4/4 / 3 / 425,520 / 0 / 1;
   3 / 588 / 16,464 / 17,052; and, if S10 is built, 3,808 / 216 / 559,316,142 /
   8,387,350,664 / 8,721,399,239 / 17,668,066,045 / 35 bits.
4. **Invariants**: INV-1..10 enforcement still green; INV-11..14 each have their
   named test/lint/doctest present and green.
5. **Independence**: no ingest or exchange Python read as implementation source;
   constructions re-implemented from prose/JSON per §5's guardrail clarification.
6. **Tier hygiene**: every exchange-derived receipt line carries the `x-` prefix
   and its binary's header cites the ledger entries (§9.1).
7. **Documentation**: every new public item cites claim IDs (and x:NNN where
   applicable); `cargo doc` clean.
8. **Dependencies**: unchanged (`num-*` only; `#![forbid(unsafe_code)]`; overflow
   checks; no floats).
9. **Report** (message, not a committed file): commands run; every reproduced
   count with its tier (corpus-anchored vs x-); rob-frozen values, if any, from
   §11.1; ambiguities; and the explicit list of x-numbers now backed by rob's
   independent receipts, for the wiki track to record.

Do not begin slice 03.

---

## 14. Proposed module map (additions)

```
rob/crates/core/src/support/
  dynamics.rs        TypedHiddenObservation (INV-13), matching-minor update
                     (force/delete/contract/reduce), game-typed wrapper,
                     deletion-ledger audit type (INV-11/12)       [TRANS-08..13; D5]
  symbolic.rs        SymbolicTraceCertificate, validate_symbolic_trace —
                     the external-state gate                      [REACH-14/15; Exec §18]
  outer.rs           schedule language + censuses, B_{n,u}, C(k) profile
                     counts, five-check necessary-profile validator
                     (INV-14, D3 naming)                          [REACH-04/06/07/11; x:002]
  transport_reach.rs trace transport, reachable_census_class (3 classes),
                     unscored-only boundary                       [rec ALG-22/23; x:004]
  floor.rs           (stretch) module language, templates, split-zeta
                     upward-closure counter                       [x:001]
  cells.rs           + sample_native_world (native count-ratio sampler)
                                                                  [CELL-10E/F]

rob/crates/verify/src/
  s5.rs s6.rs s7.rs s8.rs s9.rs (s10.rs)
  bin/verify_dynamics.rs      S5
  bin/verify_symbolic.rs      S6
  bin/verify_outer.rs         S7
  bin/verify_unreachable.rs   S8
  bin/verify_transport.rs     S9
  bin/verify_floor.rs         S10 (stretch)

rob/receipts/
  verify_dynamics.txt verify_symbolic.txt verify_outer.txt
  verify_unreachable.txt verify_transport.txt (verify_floor.txt)
```

Implement S5 → S6 → S7 → S8 → S9 (→ S10) in order; a stage's receipt must be green
and committed before the next stage's first line of code.
