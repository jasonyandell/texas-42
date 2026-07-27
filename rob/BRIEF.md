# rob — Implementation Brief, Slice 01

This is the definitive first implementation assignment for **rob**, this repository's
exact Texas 42 engine. It supersedes both ingest packages'
`docs/50_CODEX_IMPLEMENTATION_PROMPT.md` (v0.7 and v0.7-reconstructed). Those prompts
were written for a different executor and predate the cross-package reconciliation
recorded in `wiki/`; every decision they left open or got wrong is closed here.
An implementation agent executes this brief without consulting ingest for decisions —
ingest and the wiki are consulted for *definitions, theorems, and claim IDs only*.

Citation convention (same as the wiki): **v0.7** =
`ingest/texas-42-foundations-source-of-truth-v0.7`, **rec** =
`ingest/texas-42-foundations-source-of-truth-v0.7-reconstructed`; `Math §x`, `Exec §x`,
`Rules §x` name each package's `20_MATHEMATICAL_FOUNDATION.md`,
`30_EXECUTABLE_SPECIFICATION.md`, `10_RULES.md`; claim IDs (`ALG-12`, `CELL-14`, …)
refer to `40_CLAIM_STATUS.md`.

---

## 1. Governing synthesis (already decided — do not relitigate)

The two ingest packages are divergent siblings of a common v0.6 ancestor
([wiki/package-provenance](../wiki/package-provenance.md)). The reconciliation verdict,
binding on rob:

> **rec's mathematics under v0.7's type discipline.**

Concretely, the discrepancy resolutions from
[wiki/discrepancies](../wiki/discrepancies.md) that affect implementation are adopted
as fixed decisions:

- **D1 — Reachability is a proof-irrelevant proposition (v0.7 wins).** No state
  carries an identity-bearing reachability certificate. Witnesses are erasable audit
  artifacts, excluded from equality, hashing, serialization, and transition
  (v0.7 Exec §10, §18, §25; 55_V06_REVIEW §3.1). rec Exec §10/§25's
  `ContractedHandOrigin` certificate design is rejected.
- **D2 — Cells, fiber, and normal form are derived views, never stored fields
  (v0.7 wins).** No `cells:` field in mechanical state; no dual
  `physicalSupport`/fiber storage. Caches live outside semantic equality with a
  coherence invariant (v0.7 Exec §15, §20, §25; CELL-17). rec's own Math §7.16.4
  agrees; only rec's executable spec predates the discipline.
- **D3 — Vocabulary: "necessary outer profile", never "outer reachability
  certificate" (v0.7 wins).** The 46-bit outer object is necessary-only and cannot
  construct reachability. rob's types must make this unmistakable
  (e.g. `ReachabilityOuterNecessaryProfile` with no path to a certified state).
- **D5 — Support dynamics is a refinement, not a contradiction (rec wins).**
  Standalone support is not a game state (REACH-03A), **but** given declaration,
  actor, played domino, and typed observation context, the successor support is
  uniquely determined (TRANS-08). Slice 02 will implement that calculus; slice 01
  must not expose any transition API on standalone support.
- **D11 — Slice-01 algebra content is rec's superset (rec wins).** `pip_sum`,
  `competitive_ordinal`, pip-trump transports, unscored mechanics classes, the
  looped-K7/antidiagonal identities are all in scope, alongside everything in v0.7's
  version. Keep v0.7's sentence that the ingest verifiers are external proof
  receipts, not proof-assistant kernel proofs.
- **D4 — Honesty about verifier dependencies.** rob's verification binaries state
  exactly what they share. The independent prose-rule resolver (§8, R-ALG-04) shares
  *nothing* with the algebra implementation beyond domino identity and the
  declaration enum.

Authority order for rob, highest first:

1. this brief (decisions and scope);
2. `wiki/` (reconciliation, merge order, discrepancy resolutions);
3. the ingest packages (definitions, theorems, claim IDs — immutable; where they
   disagree, the wiki's resolution applies).

rob **never redefines** a mathematical object. If rob's implementation appears to
require a definition that differs from ingest-as-reconciled, that is a finding to
report, not a local fix (§11).

---

## 2. Language and toolchain (decided)

**Rust, stable toolchain, pinned via `rust-toolchain.toml`.**

Why Rust and not the Python 3.12 the original prompts mandated:

1. **Type discipline is the point.** The v0.7 repairs (phase-indexed states,
   unconstructible illegal field combinations, equality through projected state only,
   derived views outside equality) are *enforceable* in Rust — sealed enums, newtypes,
   manual `Eq`/`Hash` impls, no reflection. In Python they are conventions; the
   package's own history (rec regressing v0.7's repairs) shows conventions do not
   hold.
2. **Independence of the receipts.** All ingest verifiers are Python. rob
   re-deriving every number in a different language makes "never copy the verifiers"
   structurally true and makes the reproduced receipts genuinely independent
   evidence.
3. **Exactness.** `i64`/`i128` plus `num-bigint`/`num-rational` give exact integers
   and rationals with no ambient float contagion. The 81-bit census total
   (~1.83 × 10²⁴) and the count-ratio sampler's rational weights are first-class.
4. **The roadmap needs speed.** Later slices target the symbolic support DAG and the
   open reachable-census count inside a 2⁴⁶-scale outer language (OPEN-11). Choosing
   a systems language now avoids a rewrite at the exact moment correctness capital
   is highest.
5. **Lean 4 companion track.** This repository will also host the Lean formalization
   ([wiki/proof-assistant-plan](../wiki/proof-assistant-plan.md)). The trust boundary
   places rob *outside* the kernel ("external receipts / production
   implementations") — so rob should not be written in Lean; it should emit stable,
   deterministic receipts and machine-readable test vectors that Lean-side
   reflection can consume. Cross-validation is by receipt agreement, not FFI.

Toolchain rules:

- Runtime dependencies of the core crate: `num-bigint`, `num-rational`,
  `num-integer`, `num-traits` — nothing else. Dev-dependencies: `proptest`.
- `#![forbid(unsafe_code)]` everywhere. Clippy at `deny(warnings)` in CI.
- **No floating point anywhere in the workspace** (INV-4): CI greps for `f32`/`f64`
  and fails on any hit outside comments; clippy `float_arithmetic` denied.
- Receipts are plain deterministic UTF-8 text in the ingest style (§9); an optional
  JSON emission mode is permitted but the text receipt is canonical.

---

## 3. Project layout (decided)

```
rob/
  README.md               authority order and orientation (exists)
  BRIEF.md                this document
  rust-toolchain.toml     pinned stable toolchain
  Cargo.toml              workspace
  crates/
    core/                 rob-core: the engine; pure, no I/O
    verify/               rob-verify: receipt binaries (one per stage)
  receipts/               committed expected receipts; CI diffs against fresh runs
  ci/check.sh             fmt + clippy + no-float grep + tests + receipt diffs
```

`rob/` is the only directory rob writes. `ingest/` is immutable; `wiki/` is written
by the wiki track, not by rob (findings are *reported*, §11).

---

## 4. Scope: slice 01 and its boundaries

Slice 01 is the first coherent slice per
[wiki/first-implementation-slice](../wiki/first-implementation-slice.md) and
FINDINGS §7: **declaration algebra through the support normal form and capacity DP**,
in four gated stages. A stage begins only when the previous stage's receipts are green.

- **S1 — Declaration algebra** (rec-superset assignment): universe, nine
  declarations, trick resolution, transports, mechanics classes.
- **S2 — Objective hand machine**: rules config, auction, contract, phase-indexed
  contracted-play states with certified lifecycle constructors (v0.7 Exec §§7–12,
  §10.1).
- **S3 — Cells as derived views + losslessness parity harness**: `derive_rule_cells`,
  typed-update algebra, replay parity corpus (Math §§6–7; CELL-05/06/07, TRANS-07).
- **S4 — Support normal form + capacity DP**: Hall feasibility, exact counting,
  marginal criterion, canonical reduction, NF trichotomy, SCC compiler, count-ratio
  sampler, censuses (Math §§7.7–7.12).

**Explicitly out of slice 01** (later slices, in order — do not begin, do not
scaffold speculative APIs for):

- **Slice 02** — support dynamics: matching-minor calculus, monotone deletion, the
  dynamic 63-edge-budget theorem, symbolic trace reachability, schedule language,
  necessary outer profiles (TRANS-08..14, REACH-06..16; receipts: 1,331 supports ×
  170,058 typed observations, 1,406,592 monotonicity checks, 108 hands / 3,024
  transitions / 6,804 deletions, outer-profile counts 7,124,838,074,989 per
  declaration and 64,123,542,674,901 total, REACH-10's 450-generator witness).
- **Slice 03** — folded trick and reduced viewer kernel (PLAY-12..17; receipts:
  737,100 trick cases + 2,211,300 sequential updates, 84 open-trick shapes, 3,132
  score-recovery prefixes, 8 dihedral frames, future-equivalence corpus
  5,898 machines / 17,560 pairs).
- **Slice 04** — belief and filtering: Bayes over augmented worlds, physics-only
  uniformity, and the **90-world posterior-flip regression** as a permanent guard
  test (STR-06..09; the support half of that witness lands in slice 01, §8 S3).
- **Slice 05** — solver and census frontier (OPEN-11 counting over the symbolic
  DAG, memoization on the reduced kernel).

No auction solver, no CLI, no UI, no optimization caches before correctness, ever in
slice 01.

---

## 5. Named invariants (non-negotiable, each with enforcement)

Every invariant below has (a) a mechanism that makes violation hard to write and
(b) a named test or CI check that fails loudly if it is violated anyway.

- **INV-1 DERIVED-NOT-STORED.** Cells, reduced support, normal form, fiber, and
  compiled tables are pure functions of semantic state (`derive_rule_cells`,
  `support_reduction`, `compile_exact_support`, `remainder_fiber`); no semantic
  struct stores them. Caches, if any, live in a separate `CompiledView` type with a
  coherence assertion, outside equality. *Enforcement:* code review rule — semantic
  structs contain only Exec-§-listed semantic fields; test `inv_derived_coherence`
  recomputes every view from scratch after each transition on the S3 corpus and
  asserts equality with any cached value. (v0.7 Exec §15/§25; CELL-17; D2.)
- **INV-2 PROJECTED-EQUALITY.** `Eq`/`Hash`/serialization of any certified state go
  through its projected semantic fields only; manual trait impls, never derived over
  witness- or cache-bearing wrappers. *Enforcement:* test `inv_projected_equality`
  constructs two states identical in semantic fields but differing in
  witness/cache/audit data and asserts equal + same hash. (v0.7 Exec §25.)
- **INV-3 PROOF-IRRELEVANT-REACHABILITY.** `Reachable` is a proposition. Certified
  lifecycle constructors (v0.7 Exec §10.1) are the only internal source of
  reachable states; lifecycle is never inferred from field values; external states
  enter only through exact validation after which tag and witness are erased. No
  stored runtime reachability flag. *Enforcement:* the certified-state type's
  constructor is private to the lifecycle module; test `inv_no_reachability_field`
  plus INV-2's hash test cover erasure. (REACH-03; D1.)
- **INV-4 EXACT-ARITHMETIC.** No floats anywhere; ranks are ordered ADTs
  (`Rank::Top` vs `Rank::PipSum(n)` vs `Rank::DoublePip(p)`), probabilities are
  `BigRational`, censuses are `BigUint`. *Enforcement:* CI grep + clippy deny as in
  §2; checked arithmetic (`overflow-checks = true` in all profiles).
- **INV-5 COUNTS-ARE-CI.** Every exhaustive count in this brief's harness (§8) is a
  hard equality assertion against the exact expected integer, in a test named after
  the receipt, and additionally printed in the stage receipt. Weakening, sampling
  down, or `>=`-ing an exact count is forbidden. *Enforcement:* receipts are
  committed and CI diffs byte-for-byte (§9).
- **INV-6 REACHABLE-IMPLIES-FEASIBLE.** For internally constructed certified states,
  an empty support fiber is an internal error (panic), never a value; the `Empty`
  normal form exists only on the external-validation path for foreign systems.
  *Enforcement:* `compile_exact_support` on a certified state returns the nonempty
  NF type or panics; the `Empty`-inclusive total NF type appears only in the
  validation API. (CELL-14's `Empty` is for the full schema, not for reachable
  states.)
- **INV-7 NO-RANK-FROM-ID.** `DominoId` (canonical triangular order `(0,0), (1,0),
  (1,1), …, (6,6)`) is identity only; game rank is never derived from id magnitude.
  *Enforcement:* newtype with no `Ord`; ordering exists only on declaration-relative
  trick keys. Test `inv_id_not_rank` checks a witness pair where id order and every
  declaration's key order disagree.
- **INV-8 ONE-SOURCE-OF-TRUTH.** Derived public facts (trick winner, running score,
  settlement) are functions of the base event stream; no second materialized
  authority (R-INFO-02A). *Enforcement:* events are emitted as explicit constructor
  return data (v0.7 Exec §10.1), and test `inv_event_replay` recomputes all derived
  facts from events alone and compares.
- **INV-9 TYPE-DISTINCT-DOMAINS.** Complete deal, current remainder world, fiber,
  and belief are distinct types; support is a set, never a measure; "feasible" and
  "reachable" are distinct predicates and the outer-profile type has no conversion
  to any certified type. *Enforcement:* type system; negative compile-test (a
  doc-test that fails to compile) for the forbidden conversions.
- **INV-10 VOCABULARY.** v0.7 naming throughout: "necessary outer profile" (never
  "certificate"), `NativeHandView` (never `NativeHand`-with-stored-fiber),
  `UnscoredMechanics` and `ScoredMechanics` as distinct structures that no code
  conflates (K4; D3). *Enforcement:* CI grep forbids `OuterCertificate`-style
  identifiers; review.

Additional standing guardrails inherited from both 50-prompts, still binding:

- Never copy code or tables from the ingest verifiers or any other Texas 42
  implementation; they are proof receipts, not source. (Transcribing *corpus-shape
  parameters and expected numbers* — which this brief already does — is permitted;
  transcribing resolver/update/compiler logic is not.)
- No hard-coded 28-by-declaration result tables as source data; derived immutable
  tables only when generated from implemented definitions.
- No packed-bit primary representation: sets/vectors first; bitmask codecs only via
  proved refinement (`decode ∘ encode = id` plus operation commutation), as separate
  types (proof-assistant-plan, "sets first").
- No global mutable state; no strategic value assigned to dominoes (HAND-07: no
  context-free scalar domino value exists); only the nine Straight declarations.
- Never call the `2↔3` transport a global symmetry or a literal numeric-rank
  isomorphism.
- Every public item's doc comment states the mathematical object it implements with
  claim IDs (e.g. `/// Implements ALG-06/ALG-07 effective-suit absorption.`).

---

## 6. Reading list for the implementer

Read in this order before writing code; everything else is consulted on demand.

1. `rob/README.md`, then this brief in full.
2. [wiki/Home](../wiki/Home.md), [wiki/rules-profile](../wiki/rules-profile.md),
   [wiki/declaration-algebra](../wiki/declaration-algebra.md),
   [wiki/support-fiber](../wiki/support-fiber.md),
   [wiki/capacity-dp](../wiki/capacity-dp.md),
   [wiki/minimal-support-normal-form](../wiki/minimal-support-normal-form.md).
3. `Rules` (byte-identical in both packages) — the normative game.
4. v0.7 Exec §§1–16, §25, §26 — the type contracts; rec Exec §§4–5 for the
   slice-01 algebra additions (looped-K7, transports, ordinals).
5. Math (either package; v0.7 notation) §§2–3, §§6–7 as each stage needs them;
   rec Math §3.10 for unscored mechanics.

---

## 7. Required API surface (semantic, Rust spelling free)

Stage S1 must expose at least the reconciled (rec-superset) surface:

domino layer — `PIPS`, `DOMINOES`, `Domino`, `DominoId`, `domino_id`,
`domino_from_id`, `contains`, `is_double`, `pip_sum`, `count_points`;
declarations — `Declaration` (`PipTrump(p)` ×7, `DoublesTrump`, `NoTrump`),
`GAME_DECLARATIONS`; per-declaration algebra — `algebra_for(declaration)` giving
`called`, `powered`, `effective_suits`, `led_suit`, `lead_contexts`, `lead_fiber`,
`follows`, `rank`, `tier`, `trick_key`, `beats`, `threat`, `resolve_trick`
(actor-preserving, returns winner plus trick points, rejects duplicates/malformed
tricks), `competitive_ordinal(led_suit, id)`; transports —
`pip_trump_transport(source, target)` with `pip_map`/`domino_map`/`context_map`,
`unscored_mechanics_class(declaration)`.

Mathematical definitions are exactly the ingest ones (called/powered sets, effective
suits with called absorption, led suit = called context or higher pip, follow =
effective-suit membership, total declaration-relative rank with `TOP` for off-trump
doubles, tier 2/1/0, lexicographic `(tier, rank)` key with `(0,0)` sloughs). Do not
re-derive them from prose alone — implement against Exec §§4–5 with the wiki as
guide.

Stages S2–S4 implement the contracts of v0.7 Exec §§7–12 (auction, contract,
lifecycle constructors, legal play, settlement), §§13–16 (public history,
remainder map, mechanical state with *derived* cells, fiber) and Math §§7.7–7.12
(feasibility, counting, reduction, normal form, SCC compilation, censuses), under
the invariants of §5. The S4 public surface must include: `hall_feasible`,
`assignment_count` (all three routes), `marginal_allowed`, `reduce` (canonical
reduction), `compile_normal_form` (one assignment + one SCC pass), `decode`
(NF → fiber), the linear ternary validator, `rank_world`/`unrank_world`, and the
exact count-ratio sampler parameterized by an exact-rational choice source.

---

## 8. Verification harness — receipts rob must reproduce

Every row is a named test asserting the exact number, and a line in the stage's
receipt binary. Numbers are from the committed ingest verifier outputs
(`verification/VERIFICATION_OUTPUT.txt`, both packages, plus rec's
`verify_reduced_kernel.py` output), all of which the wiki re-ran and confirmed
byte-for-byte on 2026-07-26.

### S1 — `verify_algebra` (binary), tests `r_alg_*`

| Test | Assertion | Exact numbers | Source |
|---|---|---|---|
| `r_alg_universe` | 28 unique dominoes, valid `0 ≤ low ≤ high ≤ 6`, id round-trips, natural incidences size 7, doubles 1 / mixed 2 memberships, count total | 28; 35 | Math §2; ALG-01.. |
| `r_alg_k7` | universe = Sym²(F₇) = looped-K₇ edge set; incidences are closed stars; `count(d) = pip_sum(d)` exactly when the sum is 5 or 10, else 0 | 28 edges; 10+10+5+5+5 = 35 | rec ALG-20/21 |
| `r_alg_contexts` | per declaration: effective suits nonempty; called ⇒ `{CALLED}`; exactly 7 leadable contexts; lead fibers partition with cardinality multiset `{1..7}`; doubles-trump natural context 0 nonempty but unleadable (follow table NOT deleted) | 7; {1,…,7} | REACH-05 |
| `r_alg_tiers` | tier 2 ⇔ powered, 1 ⇔ unpowered follower, 0 otherwise; key `(0,0)` exactly at tier 0; natural doubles top their live suit; trump double tops mixed trumps; doubles-trump order 6-6 … 0-0 | — | Math §3.4–3.5 |
| `r_alg_unique_winner` | every (declaration, lead, 3-subset of remaining 27) has a unique max key | 9 · 28 · C(27,3) = **737,100** | ALG-12 |
| `r_alg_prose_agreement` | an independently coded prose-rule resolver (Rules §R-WIN/R-RANK only; never calls `trick_key`/`tier`/`rank`/`beats`/`resolve_trick`; lives in `rob-verify`) agrees on all cases | **737,100** | ALG-12 |
| `r_alg_scoring` | count 35; seven base trick points; hand total 42; `resolve_trick` = 1 + count payload; sloughs never beat a valid lead; highest trump beats every nontrump; else highest follower wins | 35; 7; **42** | R-SCORE-01..04 |
| `r_alg_beats` | `e ∈ beats(q, d)` ⇔ `trick_key(e,q) > trick_key(d,q)` for every declaration, context, pair | all 9 · 8 · 28 · 28 | Math §3.7 |
| `r_alg_threat_witness` | NT `0-0` and `1-1` have empty when-led threat sets yet follow different natural suits; threat is an exact diagonal query, not a complete play ontology | — | ALG-15 |
| `r_alg_scored_transport` | of all 5,040 pip permutations exactly 2 preserve every count label (identity, `2↔3`); `2↔3` is a game-order isomorphism exactly between declaration layers 2 and 3 (order transported, not literal labels) | 5,040 → **2** | ALG-17/18/19 |
| `r_alg_unscored_transport` | all ordered pip-trump transports succeed on the count-blind relation surface (called/powered, incidence, led context, follow, all pairwise contextual order comparisons, round-trip bijectivity) | **49** transports; **307,328** comparisons | rec ALG-22 |
| `r_alg_mechanics_classes` | structural class signature computed from the implemented relation surface yields exactly 3 unscored classes — pip trumps / doubles trump / no-trump — separated by `(\|powered\|, #{unpowered one-suit tiles})` = (7,6)/(7,0)/(0,7) | **3** | rec ALG-23/24 |
| `r_alg_competitive_ordinal` | `competitive_ordinal` is order-isomorphic to `trick_key` within each context; max competitive-class size over all (declaration, context) | **13** | rec PLAY-12/13 |

### S2 — `verify_objective` (binary), tests `r_obj_*`

| Test | Assertion | Exact numbers | Source |
|---|---|---|---|
| `r_obj_deals` | ordered-deal domain size computed exactly (BigUint) from the deal definition | 28!/(7!)⁴ = **472,518,347,558,400** | Math §6 |
| `r_obj_hidden` | conditional hidden assignments for one viewer | 21!/(7!)³ = **399,072,960** | Math §7 |
| `r_obj_auction_census` | exhaustive auction tree per mark cap 1..7: terminal histories, reached mark maxima, caps 5/6/7 produce identical trees (reachable ceiling `min(m_max, 5)`) | **(2380, 3060, 3196, 3213, 3214, 3214, 3214)**; maxima **(1, 2, 3, 4, 5, 5, 5)** | R-AUC-12 |
| `r_obj_lifecycle` | certified constructors enforce v0.7 Exec §10.1 pre/postconditions; all-pass advances shaker, no marks; events emitted as return data; phases unconstructible out of order | — | Exec §10.1; INV-3/8 |
| `r_obj_legal_play` | property tests: legal plays = follow-if-possible on effective suits, else anything; leader = bidder on trick 1; winner leads next | proptest | R-FOLLOW, R-LEAD |
| `r_obj_conservation` | on every generated complete hand: 7 tricks, points sum exactly 42; `P_D = 42` ⇔ declaring side took all seven tricks | **42**; equivalence | R-SETTLE-02A |

### S3 — `verify_support` (binary), tests `r_cell_*`

| Test | Assertion | Exact numbers | Source |
|---|---|---|---|
| `r_cell_initial` | after any straight auction + declaration: `U` = 21 unseen tiles, every `P_s = U`, every `k_s = 7` (bids/declaration remove no deal by rule); initial support has exactly 21 × 3 = **63** holder edges (static half of the 63-edge budget; the dynamic deletion theorem is slice 02) | 21; 7; **63** | AUC-07/08; TRANS-12 |
| `r_cell_dependent` | dependent-cells negative witness: 2 seats, 2 tiles, capacity 1 each — only 2 of 4 componentwise assignments are worlds | 2 of 4 | CELL-01 |
| `r_cell_tiny_updates` | typed-update algebra exhausted on all tiny (universe ≤ 3) cell systems | **14,412** leads; **56,460** follows; **56,460** sloughs | TRANS-07 |
| `r_cell_parity` | replay parity corpus: 9 declarations × 12 deterministic contracted hands × prefix lengths 20..28 = **972** prefixes; for each, exact set equality `Φ(derive_rule_cells(state)) = ρ(Ω(I))` (fiber vs replayed compatible-deal image). rob writes its **own** deterministic hand generator; the corpus *shape* (972 = 9·12·9) is a hard assertion, the with-voids count is generator-specific — freeze rob's value in the committed receipt (ingest's generator yields 970) | **972** | CELL-05/07, CELL-07A |
| `r_cell_transitions` | along the parity corpus, per-play typed support transitions: 8 per hand × 108 hands; hidden plays never increase the fiber, viewer plays leave it unchanged (counts are corpus-shape-forced: plays 21–28 give each seat exactly 2 plays) | **864** total; **648** hidden nonincrease; **216** viewer equality | TRANS-01..05 |
| `r_cell_ninety_world_support` | the 90-world witness, support half: replay both legal auction histories α_A/α_B and the five fixed tricks (Math §10.4 data); assert both reach identical cells and fiber cardinality 6!/(2!)³ = **90**. The posterior-flip half (opposite optimal leads) is the slice-04 regression | **90** | STR-06; Math §10.4 |

### S4 — `verify_normal_form` (binary), tests `r_nf_*`

The tiny corpus family, defined closed-form so rob never reads the Python: for each
universe size n ∈ {1,2,3,4}, all (2ⁿ)³ triples of allowed subsets × all capacity
triples with k₀+k₁+k₂ = n — total Σ (2ⁿ)³ · C(n+2, 2) = 24 + 384 + 5,120 + 61,440 =
**66,968** systems, of which **14,578** are feasible with **22,620** total worlds.

| Test | Assertion | Exact numbers | Source |
|---|---|---|---|
| `r_nf_hall` | Hall biconditional ≡ direct enumeration on every tiny system (7 subset checks for 3 seats) | **66,968** | CELL-09/10 |
| `r_nf_count_routes` | generating-function, deletion-recurrence, and occupancy-DP counts all agree with direct enumeration | **66,968** agreements | CELL-10A/B/H |
| `r_nf_capacity_dp` | native occupancy DP over all 8³ capacity triples: bounds attained | **512** profiles; ≤ **512** occupancy states; **1,533** candidate-holder checks; **1,344** capacity-eligible updates; ≤ **48** live states/layer; max count **399,072,960** | CELL-10I/I1 |
| `r_nf_marginal` | marginal-edge criterion (`d ∈ P_s*` ⇔ forced successor Hall-feasible) ≡ world projection on the tiny corpus; plus the local-vs-marginal negative witness | **785,736** edges | CELL-10J/K |
| `r_nf_reduction` | canonical reduction `red(C)`: fiber-preserving, contractive, idempotent, normal form (equal fibers ⇔ equal reductions) on all tiny systems; plus the reduction-instability witness (reduced predecessor, unreduced raw successor) | **66,968** | CELL-10L/L1/N |
| `r_nf_sampler` | exact count-ratio sampler: every world probability on the feasible tiny corpus equals the telescoping integer-ratio product, as exact rationals | **22,620** | CELL-10E/F/G |
| `r_nf_quotient` | NF trichotomy (active seats 0/2/3, never 1); NF ↔ fiber bijection; SCC compilation (one assignment + one linear pass) ≡ per-edge Hall; every stored ternary exclusion essential; linear ternary validator ≡ matching search; rank/unrank round-trips | **22,620** SCC compilations; **2,151** essential exclusions; **22,620** rank/unrank | CELL-12..20, 25/26 |
| `r_nf_ternary_census` | native ternary signature census and S₃ quotient | **136,514** signatures; **1,667,666** matrices; max **114**/signature; orbits **23,842**; representative matrices **296,721**; stabilizer orbits **279,048**; max **103** | CELL-21/22/23 |
| `r_nf_census_81` | full-schema census from the Math §7.12.5 formulas (BigUint; computed, not hard-coded): empty = 1; determinate = **8,102,258,940,222,814**; binary = **11,495,078,055,913,018,482**; ternary = **1,830,955,704,129,296,418,354,864**; total = **1,830,967,207,309,611,271,596,161**; 2⁸⁰ < total < 2⁸¹ ⇒ fixed width **81 bits** | as listed | CELL-27 |
| `r_nf_capacity_profiles` | reachable hidden-capacity triples ⇔ max − min ≤ 1; exactly 8 + 7·6 labeled profiles (capacities derive from trick progress, never three free fields) | **50** | REACH-04 |
| `r_nf_floor` | universally reachable no-void floor families: C(28,21) + 3·C(28,20) + 3·C(28,19) + C(28,18) = **44,352,165** > 2²⁵ ⇒ ≥ 26 bits; four families pairwise disjoint; for one deterministic sample per family, construct an explicit legal reaching prefix with the S2 machine (the (6,6,6) case via the pigeonhole: a 10-tile complement with ≤ 2 doubles has a pip on ≥ 3 tiles). Full outer language and ceiling are slice 02 | **44,352,165** | REACH-12 |
| `r_nf_zero_supplemental` | derived-view coherence: along the S3 parity corpus, cells/reduction/NF recomputed from the mechanical state after every transition equal any cached view — support adds 0 supplemental bits relative to certified mechanical state | 0 bits | CELL-17 |

### Numbers that are *not* slice-01 targets

For avoidance of doubt, these committed receipt lines belong to later slices and must
not be chased now: outer profiles (7,124,838,074,989 / 64,123,542,674,901; ceilings
46/43/43/40), schedule counts `A_j`/`T_{j,1}`/`T_{j,2}`, the REACH-10
feasible-but-unreachable witness (450 generators, 2 matches), support-dynamics
receipts (1,331 / 170,058 / 157,809 / 1,406,592; 108 / 3,024 / 6,804), folded-trick
receipts (2,211,300; 84; 3,132), dihedral frames (8), future equivalence
(5,898 / 17,560), and the 90-world posterior flip (exact posteriors 1/7,4/7,2/7 vs
1/2,1/4,1/4 and opposite best leads under all four utilities).

---

## 9. Receipts and CI

- Each stage binary prints a compact deterministic receipt in the ingest style
  (`rob <stage> verification: PASS` followed by `name: exact-number` lines covering
  every table row above). Committed under `rob/receipts/`.
- `rob/ci/check.sh` runs, in order: `cargo fmt --check`; `cargo clippy` with the §2
  denials; the no-float grep; `cargo test --workspace` (unit + proptest +
  exhaustive integration tests); all four verify binaries, diffing output
  byte-for-byte against `rob/receipts/`. Any diff is a failure (INV-5).
- Exhaustive tests run in full in CI — none of them exceeds minutes-scale in Rust
  (the ingest Python versions already run in minutes).
- Receipts are the Lean cross-validation interface: formats, once committed, change
  only by explicit decision recorded in the receipt file's header comment.

---

## 10. Deviations from the ingest 50-prompts (deliberate, resolved here)

1. **Language/toolchain**: Python 3.12 stdlib + `unittest` → Rust + `num-*` +
   `proptest` (§2). The original choice served a one-shot Codex run; rob is a
   multi-slice engine with a Lean companion and a solver-scale roadmap.
2. **File allowlist**: the fixed `src/forty_two/…` list → the §3 workspace layout.
   The spirit (bounded scope, no out-of-scope files) is preserved by §4's stage
   gates and the slice boundary.
3. **Slice extent**: the 50-prompts end at the algebra ("do not begin the next
   slice"); rob's slice 01 deliberately extends through S4 (NF + capacity DP) per
   the project decision recorded in
   [wiki/first-implementation-slice](../wiki/first-implementation-slice.md) §"What
   comes after" and FINDINGS §7 — with the same discipline applied at the new
   boundary (§4's out-of-scope list).
4. **Ambiguity protocol**: the originals said "stop on any package inconsistency."
   The known package-level inconsistencies are now *resolved* (§1); rob stops only
   for **new** ambiguities (§11).
5. **Content**: rec's superset assignment is adopted (D11), so v0.7's narrower
   assignment is extended, and both are extended by S2–S4.

---

## 11. Ambiguity protocol (updated)

If required behavior is not determined by this brief + wiki + ingest-as-reconciled:

1. do not choose a plausible interpretation silently;
2. do not consult or copy another Texas 42 implementation;
3. add a failing or clearly-blocked test named `ambiguity_<topic>` demonstrating it;
4. record the exact conflicting passages (file + section) in the final report, for
   the wiki track to adjudicate;
5. continue on unaffected work.

The 15 known discrepancies (D1–D15) are pre-resolved in §1 and never trigger this
protocol.

---

## 12. Definition of done — slice 01

Slice 01 is complete exactly when all of the following hold:

1. **Layout**: `rob/` matches §3; no files outside `rob/`; no out-of-scope modules
   (nothing from slices 02–05).
2. **Green**: `rob/ci/check.sh` passes end-to-end — fmt, clippy denials, no-float
   grep, `cargo test --workspace`, and all four receipt binaries byte-identical to
   `rob/receipts/`.
3. **Every number**: all exact counts in §8's S1–S4 tables asserted and printed —
   including 737,100 (twice: unique winner and prose agreement), 35/7/42, 5,040→2,
   49/307,328/3, 13, 472,518,347,558,400, 399,072,960, the auction census septuple,
   972/864/648/216, 90, 66,968/14,578/22,620, 512/1,533/1,344/48, 785,736, 2,151,
   136,514/1,667,666/114, 23,842/296,721/279,048/103, the four census integers and
   81 bits, 50, 44,352,165, and 0 supplemental bits.
4. **Invariants**: INV-1 through INV-10 each have their named enforcement
   test/lint/grep present and green.
5. **Independence**: no logic translated from the ingest Python; the prose-rule
   resolver in `rob-verify` imports nothing from the algebra module except domino
   identity and the declaration enum.
6. **Documentation**: every public item cites its claim IDs; `cargo doc` builds
   without warnings.
7. **Dependencies**: runtime = `num-bigint`, `num-rational`, `num-integer`,
   `num-traits` only; `#![forbid(unsafe_code)]`; overflow checks on.
8. **Report**: a final report (message, not a committed file) listing commands run,
   every reproduced count, rob's frozen generator-specific values (e.g. the
   with-voids corpus count), deviations, and any `ambiguity_*` tests.

Do not begin slice 02.

---

## 13. Proposed module map

```
rob/crates/core/src/
  lib.rs             crate root; forbids unsafe; re-exports
  pip.rs             Pip (0..=6); permutations of pips
  domino.rs          Domino, DominoId (INV-7), universe, count/pip_sum,
                     natural incidences, looped-K7 view          [ALG-01, ALG-20/21]
  declaration.rs     Declaration, GAME_DECLARATIONS               [R-DECL-01]
  algebra/
    mod.rs           DeclarationAlgebra facade (algebra_for)
    suits.rs         called/powered/effective suits/led suit/
                     lead contexts/fibers/follow                  [ALG-05..08, REACH-05]
    order.rs         Rank ADT, tier, trick key, beats, threat,
                     competitive ordinal                          [ALG-09..15, PLAY-12]
    trick.rs         resolve_trick (actor-preserving)             [ALG-12]
    transport.rs     PipTrumpTransport, scored 2↔3 machinery,
                     UnscoredMechanics vs ScoredMechanics (INV-10)
                                                                  [ALG-17..24]
  rules.rs           RulesConfig (target, m_max), bids, legality  [Rules §§1–5]
  objective/
    events.rs        primitive public/private events (INV-8)      [Exec §13]
    auction.rs       auction machine + exhaustive tree            [Exec §7, R-AUC-12]
    contract.rs      Contract, AuctionWin                         [Exec §8]
    play.rs          phase-indexed contracted-play state, certified
                     lifecycle constructors (INV-3), legal play,
                     settlement                                   [Exec §§10–12]
  support/
    cells.rs         derive_rule_cells (pure; INV-1), typed updates
                                                                  [Math §7.1–7.5, CELL-05]
    fiber.rs         remainder map, intensional fiber, enumeration
                     as query                                     [Math §§6–7, CELL-02A]
    hall.rs          capacitated Hall (7 subset checks)           [CELL-09]
    count.rs         three exact counting routes + occupancy DP   [CELL-10A/B/H/I]
    reduce.rs        marginal criterion, canonical reduction      [CELL-10K/L]
    normal_form.rs   trichotomy NF, SCC compiler, decode, linear
                     ternary validator, rank/unrank (INV-6)       [CELL-12..20, 25/26]
    sampler.rs       exact count-ratio sampler (BigRational)      [CELL-10E/F]
    census.rs        ternary/S₃ censuses, 81-bit census, capacity
                     profiles, floor families                     [CELL-21..27, REACH-04/12]

rob/crates/verify/src/
  receipt.rs         deterministic receipt writer
  prose_resolver.rs  independent Rules-§-prose winner resolver (D4)
  corpus.rs          deterministic contracted-hand generator (S3)
  bin/verify_algebra.rs
  bin/verify_objective.rs
  bin/verify_support.rs
  bin/verify_normal_form.rs
```

Implement S1 → S2 → S3 → S4 in order; a stage's receipt must be green and committed
before the next stage's first line of code.
