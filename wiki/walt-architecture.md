[Home](Home.md) · owns: the engineering of the `walt` crate — the six-crate workspace and the one crate's ten modules with their verified import order; the two solver stacks (sampled forward, counted backward) and the four seams where they meet; the invariants carried by types and CI rather than by convention; the gate (`walt/ci/check.sh`), its stages, its suite inventory and what it costs; the declared epochs every number is relative to; the named debts and the freezes by path · Sources: `walt/Cargo.toml`, `walt/walt/Cargo.toml`, `walt/walt/src/lib.rs`, `walt/walt/src/**/*.rs` module docs, `walt/walt/tests/*.rs` headers, `walt/ci/*`, `walt/UNIFICATION-CENSUS.md`, `walt/MAP.md`, `walt/SCENARIO-PLAYER.md` §3.4, `walt/CONTROLLER-PLAYER.md`, `walt/DISCREPANCIES.md`, `walt/briefs/{CI1-REPORT,FH3-REPORT,FH4-AUDIT,MB1-REPORT,UP1A-REPORT}.md`, `walt/probes/{waking,gran}/README.md`, `kanban/backlog/{gate-corpus-trim,ladder-policy-store}.md`, [walt-math-freezes](walt-math-freezes.md) rows 55–58, `git log` over `walt/walt/src`

# walt — the architecture: one crate, ten modules

**Tier.** Everything on this page is **EXPLORATORY** (`walt/` sits below every evidentiary tier of [Home](Home.md) and is cited by nothing above it). The numbers here are of three kinds, and each is labelled: **tree facts** (line counts, file counts, constants, import edges) measured 2026-09-13 on this checkout (re-checked 2026-09-13), whose `walt/walt/`, `walt/ci/`, crate, `walt/math/`, `walt/briefs/` and probe-record trees are byte-identical to `c00717d1` (2026-09-07): `git diff --stat c00717d1 HEAD -- walt/` names only the seven register documents this rewrite edited (`MAP.md`, `LOG.md`, `SCENARIO-PLAYER.md`, `CONTROLLER-PLAYER.md`, `DISCREPANCIES.md`, `ARCHIVE.md`, `probes/factor_belief/README.md`) and no source, gate, script or record; **gate-pinned values**, named with the test file that asserts them; and **machine figures** (wall clock, resident memory) from the briefs — measured on one 18-core / 48 GB machine, never receipts. A walt probe number is quotable as a result only through the gate that pins it; a green gate is evidence at a declared configuration, never a status change.

**Who this chapter is for.** A newcomer should read §1 and §2 to learn what the crate is and why it has two halves. A mathematician should read §2 and §3: the objects of [walt-math-reference](walt-math-reference.md) live in named modules, and the invariants of §3 are where the mathematics is made unforgeable in code. An engineer who wants to run the player should read §4 (the gate), §5 (which epoch a number belongs to) and §6 (what is owed), then go to [walt-instruments](walt-instruments.md) for every binary's invocation and record path. The programs that produced the code are told in Part III ([walt-calculated-evidence](walt-calculated-evidence.md), [walt-counted-belief-era](walt-counted-belief-era.md), [walt-focal-horizon-era](walt-focal-horizon-era.md), [walt-partnership-program](walt-partnership-program.md)); this page is the shape they left behind.

---

## 1. One crate, ten modules

### 1.1 The workspace: six crates, and why two stay separate

`walt/Cargo.toml` is a Cargo workspace (resolver 2, Rust 2021, toolchain pinned to `1.95.0` by `walt/rust-toolchain.toml`) with six members (tree fact):

| Crate | What it is | Depends on | Lints |
|---|---|---|---|
| `walt` | The whole seat: rules, kernel, and every solver stack, plus 54 binaries. Feature `parallel` (default) pulls in `rayon`; without it the crate compiles rayon out entirely (the wasm build). | `num-bigint`, `num-rational`, `num-traits`, optional `rayon` | `unsafe_code = "forbid"`, `float_arithmetic = "deny"` |
| `walt-wasm` | The level-1 browser oracle plunge ships (`cdylib` + `rlib`; `pkg/walt.wasm`, `walt.ts`). | `walt` with `default-features = false` | `unsafe_code = "deny"` (a scoped allow for `#[no_mangle]` export attributes — no unsafe block anywhere), `float_arithmetic = "deny"` |
| `walt2-wasm` | The level-2 sibling (2026-08-25): the same outer sampling and seed formula, every field seat a modeled level-1 mind. | `walt` (default-features off); dev-dependency on `walt-wasm` for the bid/declare byte-equality test | as `walt-wasm` |
| `walt-gpu-ref` | The portable M1 reference projector and the M2 carrier/receipt codecs of the GPU-native trick-1 track. | `walt` (spec/rules/kernel); dev-deps reach across the tree into `rob/crates/{core,verify}` | `unsafe_code = "forbid"`, `float_arithmetic = "deny"` |
| `walt-metal` | The only Metal / Objective-C boundary (`objc2` family, lockfile-pinned exact versions). | `walt`, `walt-gpu-ref`, `objc2-*`, `dispatch2` | `unsafe_code = "deny"` + `unsafe_op_in_unsafe_fn = "deny"`, `float_arithmetic = "deny"` |
| `walt-m2-runner` | The supervised freeze-56 M2 gate executable (`ci/check_m2_metal.sh` drives it). | `walt`, `walt-gpu-ref`, `walt-metal` | `unsafe_code = "forbid"`, `float_arithmetic = "deny"` |

The workspace profiles set `overflow-checks = true` in **dev, release and test** — the manifest's own comment: "a silent wrap is a wrong count, so check in every profile."

Why the fold of 2026-08-24 (commit `d1499d43`, "THE FOLD — one crate, seven modules; pure code motion, trace-identical") stopped at one crate rather than one workspace member is recorded in `walt/UNIFICATION-CENSUS.md` §5 and stands unchanged: **`walt-wasm` stays separate** because it needs the `cdylib` crate type, `default-features = false` to compile rayon out, and its own `build.sh` / `smoke.mjs` / `pkg` pipeline — folding it would force that feature gymnastics onto the whole crate; **the Metal pair stays separate** because the objc2/Metal stack and its `unsafe_code = "deny"` posture (versus `forbid` everywhere else) must not leak into the unified crate; and **`walt-gpu-ref` stays with the Metal pair** because its dev-dependencies reach into the rob tree, which would otherwise make the unified crate's test build depend on `rob/`. The fold was verified as code motion: 42 suites green and the wasm smoke 28/28 byte-identical to the frozen native trace (`walt/LOG.md`, 2026-08-24). The freeze-56 source closure, which pins the GPU track's sources by path, was re-issued the same day as freeze-56 v2 (`c92175ae`) so that the fold was an amendment to the freeze and not a reading of it (§6.2).

The pre-fold history, for provenance: the rules and kernel were born as `walt-core` / `walt-kernel` at S1 (`aac67828`, 2026-08-09); `walt-geom` at S2 (`6d0192ac`, 2026-08-09); `walt-strat` at S3 (same day); `walt-gpu-spec` and `walt-gpu-ref` closed portable M0/M1 at `3b4c6d60` (2026-08-16); `walt-metal` and the M2 gate at `813d5e81` (2026-08-17); `walt-m3-carrier` and the seat solver `walt-m3-probe` at `1aa409c8` (2026-08-17, "first lawful play"); `walt-wasm` at `e9b8c263` (2026-08-18); `walt2-wasm` at `33d541fe` (2026-08-25). The three crates that never built (`walt-m3-net`, `walt-m3-oracle-a`, `walt-m3-metal`, preserved at WIP `97ce321a`) and the two research crates (`walt-skeleton`, `walt-factory`) were archived at the fold; their eras are Part III's provenance pages.

### 1.2 The layer diagram and the verified import rules

Inside the crate the old strict crate boundaries survive as a module order. `lib.rs` declares ten public modules; the import direction below was regenerated on 2026-09-13 by grepping `use crate::<module>` in every file of each module (tree fact — the arrows are exhaustive, not illustrative):

```
policy_search  →  gym, scheme, solver, kernel, rules        (2026-09-07)
      gym      →  scheme, solver, kernel, rules             (2026-09-06)
    solver     →  kernel, rules                             (never geom, strat, spec, carrier, scheme)
    scheme     →  kernel, rules                             (2026-09-06; no solver dependency)
    carrier    →  spec, kernel, rules
     strat     →  geom, kernel, rules
  geom, spec   →  rules
    kernel     →  rules
     rules     →  (nothing)
```

Three consequences worth knowing before reading any module:

1. **The live player never touches `geom`, `strat`, `spec` or `carrier`.** `solver` imports only `rules` and `kernel`; the wasm crates import `solver` and `rules`. `strat` and `geom` are consumed by exactly four test files (`strat_exp4_information`, `strat_exp5_census`, `strat_trick6_census`, `geom_envelope_exhaustive`) and by no binary. `carrier` is consumed by `tests/carrier.rs` and by six binaries only — `ladder`, `level1`, `level2`, `m3probe`, `scenario`, `playout` — the fixed-carrier probes of the seat's birth week. Any older sentence calling the carrier "the seat player's data source" describes the census-era spine, not the player that runs today.
2. **`scheme` sits beside `solver`, not above it**, importing only `rules`/`kernel` ("no solver dependency; makes no compression claim", `scheme/mod.rs`). `gym` is where Scheme and the solver first meet (`gym.rs` imports `scheme` and `solver::{adaptive, factor_belief, field, partnership, policy, selection}`), and `policy_search` sits above both.
3. **Inside `solver`, `evidence.rs` knows nothing about 42** — "THIS MODULE MUST NOT KNOW TEXAS 42 RULES (parent §16.3). It imports no rules, kernel, or solver machinery: observations arrive as bare integers and rationals." It is the one module whose arithmetic could be lifted to any other game unchanged.

`lib.rs`'s doc comment is behind the code it sits in: it lists eight modules (`rules`, `kernel`, `scheme`, `geom`, `strat`, `spec`, `carrier`, `solver`) and says "originally seven modules, formerly seven crates"; the `pub mod gym;` and `pub mod policy_search;` declarations two lines below it are undocumented at the crate root (tree fact; §6.1 item 7). The fence — "Everything is exploratory tier; exact integers and rationals throughout — no floats, no clocks in the value path" — is stated there for the crate as a whole.

### 1.3 The module table

Line counts are `wc -l` over `walt/walt/src` on 2026-09-13 (total **82,324** lines of Rust in `src/`, of which 29,664 are the 54 binaries under `src/bin/`; the twelve rows below sum to the total exactly). "Created" is the commit that first added the module's root file to `walt/walt/src` (`git log --diff-filter=A`); for the seven modules that existed as crates before the fold that is the fold commit, with the crate's own birth in §1.1.

| Module | Lines (files) | Created as a module | Purpose, in the module's own words | Public API highlights (`pub` items at module root) |
|---|---|---|---|---|
| `rules` | 1,466 (10) | `d1499d43` 2026-08-24 (crate S1, 2026-08-09) | "The Straight 42 rules layer": pips, the 28 dominoes, seats and teams, the nine declarations, effective contexts, the rule algebra (incidence, follow, tier, rank, trick key, BEATS/THREAT), scoring, legality, history replay. "Imports nothing. Every derived view … is a function of the semantic state, never stored beside it." | `Domino`, `Pip`, `Seat`, `Team`, `Decl`, `DeclClass`, `Context`, `ContextSet`, `DominoSet`, `Trick`, `TrickKey`, `Rank`, `Tier`, `legal_plays`, `replay_hand`, `state_before_trick`, `voids_before_trick`, `Receipt`/`ReceiptHand` parsing (`locate_verify_player`, `parse_file`); constants `TOTAL_COUNT_POINTS = 35`, `HAND_TOTAL_POINTS = 42`, `TRICKS_PER_HAND = 7`, `HAND_SIZE = 7` (40 root items) |
| `kernel` | 816 (5) | fold (crate S1) | "The viewer kernel and its current-remainder fiber" (v0.4 §2.1): known hand, hidden live pool, per-seat capacities, observable voids; the fiber Φ(C) with exact enumeration, exact counting and exact uniform sampling. "The PRNG selects, it never computes a value." | `Kernel`, `Hidden`, `World`, `KernelError`, `HIDDEN_SEATS`, `MAX_CAPACITY`, `FiberDp`, `FiberIter`, `ReceiptDecision`, `SplitMix64`, `expected_distinct` (11) |
| `geom` | 966 (6) | fold (crate S2) | "Exact one-parameter policy geometry" (v0.4 §8–§9): rationals, affine lines, continuous piecewise-linear envelopes with endpoint ownership, argmax correspondences, capture features and finite feature sets. | `Q`, `q`, `qi`, `Line`, `Envelope`, `Piece`, `FeatureSet`, `FeatureVec`, `ArgmaxCorrespondence`, `argmax_correspondence`, `zero_information_at` (11) |
| `spec` | 1,226 (4) | fold (crate `walt-gpu-spec`, 2026-08-16) | "Portable, exact host-side contracts for the GPU-native Walt solver … deliberately contains no Metal binding or shader source." U256 masses at field scale, SHA-256, the semantic tables. | `U256Mass`, `ExactMass`, `ScaleFrame`, `OpeningResponseFrame`, `SemanticTables`, `sha256`; constants `FIELD_SCALE`, `U256_LIMBS`, `U256_BYTES`, `SCALE_FRAME_BYTES`, `OPENING_RESPONSE_FIELD_EXPONENT`, `TRICK1_FULL_HORIZON_EXPONENT`, `TABLE_FORMAT_VERSION`, `MAX_LEGAL_ACTIONS`, `MAX_CHOOSE_N` (28) |
| `strat` | 2,104 (11) | fold (crate S3) | "The operators registry": the four named operators of v0.4 §7.7/§10.3 kept deliberately distinct — PI (worldwise symbolic backward induction with fiber census), H (hidden-information fixed field), C/F (continuation/root revelation), the §10.5 information prices, scalar PI; `info` carries decision nodes, the perfect-recall partition and policies keyed by opaque info-state ids ("world-peeking is unconstructible by type"). Live only in the three census gates. | `pi_root_values`, `pi_census`, `hidden_root_values`, `revealed_summary`, `information_prices`, `scalar_census`, `InfoPartition`, `InfoStateId`, `Policy`, `ScalarPi`, `ScalarHidden` (34) |
| `carrier` | 1,645 (7) | fold (crate 2026-08-17) | "Frozen carrier construction for the freeze-57 M3 perfect-recall gate. This crate admits only the exact hand-8 receipt carrier … Two structurally independent support constructors must agree byte-for-byte before a carrier is returned." | `M3Carrier::from_receipt_bytes`, `CarrierFacts`, `CarrierSupport`, `ConstrainedSupportIter`, `carrier_profile_bytes`, `stream_digest`, KAT accessors, and the frozen constants (`RAW_RECEIPT_BYTES`, `RAW_RECEIPT_SHA256`, `ROOTS`, `FREEZE57_DESCRIPTOR_*`, `M0_M2_SOURCE_MANIFEST_SHA256`, …) (62) |
| `scheme` | 2,704 (7) | `b764665f` 2026-09-06 (dynamics/policy files `08fad726` 2026-09-07) | "Scheme/Fix: expressive, typed relations over exact Texas 42 worlds. Implements v0.4 §§3–5 as an executable query language, not a replacement state or a compression claim." Owned by [walt-scheme-fix](walt-scheme-fix.md). | `Scheme`, `Fix`, `Atom`, `Term`, `Role`, `Sort`, `Value`, `CompiledFix`, `Frame`, `Budget`, `Answers`, `Belief`, `Comparison`, `Registry`, `Predicate`, `PredicateSpec`, `PolicyProgram`, `CompiledPolicy`, `PolicyController`, `step_frame`, `step_belief`, `transport_answers`, `anchor_back` (51) |
| `solver` | 38,013 (39 = 38 modules + `mod.rs`) | fold (crate `walt-m3-probe`, 2026-08-17); modules 2026-08-24 → 09-06 | "EXPLORATORY seat-solver library — the sampling-stack machinery of SCENARIO-PLAYER.md … One solver, independent axes of configuration: `Field` …, `InnerBelief` …, `bid` (the pmake objective's threshold pair) …, `parallel`." | §1.4 |
| `gym` | 548 (1) | `c59f1115` 2026-09-06 | "Partnership gym: exact, field-relative answer keys and replayable witnesses. Scheme describes opportunities; the existing lawful evaluator grades them. Exploratory finite-domain evidence, not a joint-team oracle or strength rank." Owned by [walt-gym](walt-gym.md). | `OFFER_QUERY`, `compile_query`, `match_query`, `QueryMatches`, `ExerciseRoot`, `from_request`, `GymField`, `Assessment`, `assess`, `Trace`, `Action`, `registry` (12) |
| `policy_search` | 3,141 (7) | `08fad726` 2026-09-07 (`relational`, `prices`, `learning_*` at `c00717d1`) | "Sampled, information-consistent policy construction and incremental reuse. Exact only on the supplied finite sample against the declared frozen field. A memo belongs to one immutable root, field and ordered sample stream." Owned by [walt-partnership-program](walt-partnership-program.md) §8 and [walt-scheme-fix](walt-scheme-fix.md) §9–§11. | `Search`, `Work`, `Node`, `TablePolicy`, `ActionPool`, `Fixture`, `HashField`, `FiniteOracle`, `RelationalLearner`, `RelationalCandidate`, `PriceProgram`, `DecisionExample`, `replay`, `replay_program`, `export`, `program_digest`, `read_lessons` (~50) |
| `lib.rs` | 31 | fold | The crate root and the crate-level fence (§1.2). | — |
| `bin/` | 29,664 (54 `.rs` + `webtable.html`) | 2026-08-17 → 2026-09-07 | The instruments and player surfaces — catalogued by program on [walt-instruments](walt-instruments.md). | — |

### 1.4 The solver's thirty-eight modules

`solver/mod.rs` (1,943 lines) is the original sampling stack and still the live player's library: `Solver`, `Shared`, `Key`, `PiKey`, `Field::{Dice, Level(k), SeatLevels([usize; 4])}`, `Deadline` (a real monotonic deadline on native targets; on `wasm32` it never expires and the budget is carried by the sample counts), `INNER_SEED`, `level1_evaluate` (line 1340), `viewer_fiber_evaluate` (1419), `level1_race` / `level1_raced` / `level1_race_refined` (1666 / 1815 / 1886), `sample_open_belief` (1142). The thirty-eight modules beside it, by the stack they belong to (§2), with creation commit and the parent document each names in its header (tree facts, 2026-09-13):

| Module | Lines | Created | One line from its header |
|---|---|---|---|
| **Sampled forward stack — the live player's additions (2026-09-06)** | | | |
| `inner_belief` | 165 | `dbcc698f` 09-06 | "The belief strategy of modeled minds, independent of their level/field. `Voidless` freezes the historical shuffle stream. `VoidsCounted` uses public mechanical deductions and the existing kernel's exact uniform sampler." |
| `selection` | 246 | `9236ca7f` 09-06 | "One deterministic selection schedule for real and modeled minds" — `Rule::{Fixed, Refine, RaceRefine}`; "Values are sampled estimates. Racing is an exploratory elimination policy." |
| `partnership` | 331 | `cfb0fb25` 09-06 | "Bounded native evaluator for fixed, information-consistent seat-level fields" — `FieldProfile::{Baseline, PartnerOnly, AllLevel1}`; compiled out on `wasm32`. |
| **Calculated-evidence path (CE thread, 2026-08-24)** | | | |
| `evidence` | 361 | `51c86407` 08-24 | "The exact evidence arithmetic authority" — CE-T1…T5 of `calculated_evidence_v0.1.md`; knows no rules. |
| `adaptive` | 942 | `11bce2c8` 08-24 | "The calculated-evidence decision path, first slice" — the shared seam (§2.3): `SlicePolicy`, `DrivenState`, `PublicRecord`, `RootPosition`, `CanonicalRoot`, `decided_success`, `STREAM_DOMAIN`, `SAMPLER_ID`. |
| `policy` | 1,076 | `510d9b74` 08-24 | "The frozen policy authority (parent §16.2)" — `FreezeTuple`, `PolicyId`, `InfoKey`, `FrozenPolicy`, `ActionRule::{Preference, PinnedThenLevel1}`, `Level0Field`, `DISCOVERY_DOMAIN`, a vendored integer-only SHA-256. |
| `controller` | 1,245 | `6c9d4a5c` 08-24 | "The §16.4 decision controller: the fixed-candidate adaptive evaluator generalized to `m` frozen candidates with safe elimination." |
| `calibrate` | 1,085 | `6c56d2b9` 08-24 | "§22 step 8: the V5 flip-repair gate and the per-fixed-pair E0 calibration arithmetic" — `assert_cap_ladder`, `FLIP_FIXTURES`, `COUNT_TIMING_SEED`. |
| `field` | 506 | `89160df5` 08-24 | "Field-model identity and materialization (§21 step 3)" — `FieldKind::{Level0{n0}, Level1{n_outer,n0}}`, `FieldId`, `FieldStateKey`, `FieldModel`, `FIELD_DOMAIN`. |
| `act` | 663 | `23ba1c22` 08-24 | "The §16.4 decision controller as an ACTING player" — `ActConfig::{interactive, full}`, six labelled routes, `ACT_FALLBACK_SEED`. |
| `bundle` | 336 | `5bead5c8` 08-25 | "The bundled world evaluator: one shared-tree walk per candidate carrying a whole world set." |
| **Targeted level-2 field-swap family (L2 thread, 2026-08-24/25)** | | | |
| `exposure` | 1,892 | `89160df5` 08-24 | "The coupled pre-split replay, fixed-policy exposure (§21 step 4), and the exposure rung producers." |
| `field_swap` | 1,817 | `f0c35f6a` 08-24 | "The admissible level-2 action set and field-stability slack (§21 step 8)" — the screen whose every derived quantity is a function of its stored rows. |
| `upper_cs` | 500 | `668c5fb6` 08-25 | "The δ-valid admissible-upper E3 producer: the max-preserving upper confidence sequence" — "L2 consumes CE machinery; CE never consumes L2." |
| `hazard` | 1,213 | `cbce1ae3` 08-25 | "The Hazard-Exclusion Invariant verifier (the single dominance-bound authority)" and the one-round trump-extraction witness producer. |
| `motif` | 1,179 | `43967f30` 08-25 | "The six-motif first-split morphology classifier over correction traces." |
| `targeted` | 1,283 | `68f9d040` 08-25 | "The targeted field-1 controller: parent §8 Stages 1–5 assembled into one per-root pipeline." |
| `wakeup` | 1,634 | `43017541` 08-25 | "The level-2 detection layer (§22 step 9): typed wake-up records for one frozen action pair under a declared (σ0, σ1) field pair." |
| `waking` | 1,315 | `93d99563` 08-25 | "The waking seat: the first walt player variant that plays with a thinking-teammate model, escalating only where it can matter." |
| **Counted backward stack — the C→G ladder (2026-08-30)** | | | |
| `root_interval` | 773 | `a99fcb9f` 08-30 | "Root intervals and survivor sets: the counted-belief Slice A" — "Naming per CBS-A3: root interval and survivor set — never 'sandwich.'" |
| `grammar` | 1,049 | `2d6eb442` 08-30 | "The two-policy grammar and the residual split: the counted-belief Slice B (parent §45)." |
| `factor_belief` | 2,905 | `eba8c940` 08-30 | "The counted-belief Slices C through F: the seat-factored belief, the exact-cover contraction interface, … `SupportOracle` (Slice D), … `viewer_success_mass`, … `grammar_success_mass` (Slice E), … `refine_to_action_exact` (Slice F)" — the largest module, and the recursion every exact instrument runs on. |
| `refine` | 917 | `25b40d9f` 08-30 | "The counted-belief Slice G: the §50 integrated refinement controller" — **freeze 58**; exactly one commit in its history. |
| **The anytime proof-state program (2026-08-31 → 09-01)** | | | |
| `proof_state` | 1,466 | `d6320b36` 08-31 | "The §49 architecture spike … the smallest honest kernel of a persistent, serializable, identity-scoped proof state over one root, with an OPEN producer registry." |
| `extraction` | 135 | `1586ff78` 08-31 | "The §63 extraction producer (Phase 6) — the first shipped `ProofProducer`." |
| `frontier` | 707 | `1ceadb7c` 08-31 | "The §39–§43 work frontier (Phase 1)" — `Frontier::advance` is the anytime loop. |
| `residual` | 133 | `1ee6669a` 08-31 | "The §61 residual-Bellman producer (Phase 4)." |
| `covers` | 194 | `1ee6669a` 08-31 | "The §62 count-threat cover producer (Phase 5): the first safe, deliberately incomplete `CountThreatCover` source." |
| `laydown` | 241 | `a8987fd1` 08-31 | "The §16/§17/§64 laydown producer (Phase 7): the typed laydown hierarchy — four DISTINCT universal results that must never be blurred." |
| `opening` | 491 | `e14c1b35` 09-01 | "The §65 opening-root iterative run (Phase 8): one root, one append-only `ProofState`, a LADDER of declared budget stops." |
| `doom` | 1,048 | `eb5a459d` 09-01 | "The doom census — counterexample mass as a deterministic upper bound (the structural producer §70 asks for, and the ∀-fail dual of the §16 laydown hierarchy)." |
| **The closing round and the focal-horizon hierarchy (2026-09-01 → 09-04)** | | | |
| `model_belief` | 1,823 | `90255f52` 09-01 | "The MB0 exact finite-type vertical slice: the field model as a persistent hidden coordinate (§74's bounded assignment)." |
| `model_recursion` | 1,167 | `bfbb45ce` 09-02 | "MB1: the model-belief recursion joins the solver. A SIBLING of `model_belief` (MB0), which it consumes and never forks." |
| `godgap` | 933 | `c5d2f32a` 09-02 | "The God-gap census (slice U0) — where the information-consistency price actually appears." |
| `unified` | 1,763 | `3b4105ca` 09-02 | "The unified walt player, slice UP0 — one decision function, every exact instrument, provenance always." |
| `horizon` | 635 | `62abe028` 09-03 | "The in-solve horizon census (slice U0b) — the §38/§40 God-gap census … run at EVERY belief node the exact recursion reaches at a declared depth below a root." |
| `focal_horizon` | 758 | `1e213bdb` 09-04 | "The focal-horizon hierarchy, slice FH1 — the parent's §28 generic fixed-field engine (`focal_horizon_sandwich_v0.1.md`, cited by title only; the construction is the FOCAL-HORIZON HIERARCHY, FH-A2)." |
| `focal_ladder` | 1,143 | `dc515ac0` 09-04 | "The focal-horizon ladder, slice FH2 — … an append-only store of NODE FACTS" walked in budgeted passes; the root is a derived view. |

Two structural facts a reader of the table should hold: `adaptive` is imported by **33 of the other 37** solver modules (every one except `evidence`, `inner_belief`, `selection`, `partnership`); and `mod.rs` and its two 2026-09-06 companions are mutually dependent — `mod.rs` re-exports `inner_belief::InnerBelief` and stores a `selection::Rule` in `Shared`, while `selection.rs` uses `mod.rs`'s `best_of`, `BlockRace`, `Key` and `SplitMix64`. That is one module split across files, not a layering violation, but it is the only cycle in the crate and the table records it as such.

### 1.5 What the binaries consume

The 54 binaries under `src/bin/` are the instruments; [walt-instruments](walt-instruments.md) owns each one's invocation, record path and pinning gate. Their crate-module imports (tree fact, 2026-09-13) confirm the layer picture: six fixed-carrier probes import `carrier` + `rules` only (`ladder`, `level1`, `level2`, `m3probe`, `scenario`; `playout` adds `solver`); the `scheme` bin imports `kernel`, `rules`, `scheme`; `partnership_gym` imports `gym`, `rules`, `scheme`, `solver`; `policy_lab` imports `gym`, `policy_search`, `rules`, `scheme`, `solver`; `relational_lab` imports `policy_search`, `rules`, `solver`; every other binary imports `solver` and `rules` (most with `kernel`). No binary imports `geom`, `strat` or `spec` directly.

---

## 2. Two recursions

Jason's frame, now on record in the source (`unified.rs`, "THE TWO RECURSIONS"): 42 is two recursions running in opposite directions — enumerable exactness walking **backward** from terminal, and sampled or structural play walking **forward** from now. The crate is organized around exactly that split. What follows names the modules of each stack and the four places they touch. The mathematics of each object is on [walt-math-reference](walt-math-reference.md); the programs are in Part III.

### 2.1 The sampled forward stack (the live player and the CE thread)

**The player.** `solver::mod` is the seat as it plays: draw belief worlds from the kernel's exact uniform sampler, run a modeled mind of declared level in every non-viewer seat, and choose the tile maximizing the pmake indicator over the sample. The objective is wired as a threshold pair on `Shared.bid`: make ⇔ `banked_t1 ≥ bid` ⇔ `banked_t0 ≤ 42 − bid`, so bidding and play share one solver (`bidcurve` prices P(make b) per declaration with the same machinery). `Field` chooses the modeled minds' kind (`Dice`, `Level(k)`, or per-seat `SeatLevels`); `InnerBelief` chooses whether those minds sample voidless (the historical stream, the default) or count public voids; `selection::Rule` is the one schedule — fixed, refine-on-saturation-tie, or race — for the real root and for every modeled mind. `level1_evaluate` is the library authority the browser oracles and `webtable` call; the arena bridge and the terminal table carry their own copies (§6.1).

**The calculated-evidence path** (CE thread — "CE = sampling depth") turns the same sampling into anytime-valid evidence with a typed result ladder: `evidence` (exact e-processes, risk ledger, no fixed sample count anywhere) → `adaptive` (the canonical kernel adapter per root, one common indexed world stream, `DeltaSettled` versus `Unresolved`) → `policy` and `field` (frozen policies and modeled fields with immutable identities) → `controller` (`m` candidates, safe elimination) → `act` (the controller as a seat: six labelled routes, three of which are level-1 fallbacks that are **never** presented as settled winners). `calibrate` and `bundle` are its calibration arithmetic and its work-shared evaluator; the `shadow` binary is the controller run beside the live player that records and never acts.

**The targeted level-2 family** (L2 thread — "L2 = model choice") asks when the modeled field's level would change the play: `exposure` and `field_swap` (the screen), `upper_cs` (the max-preserving upper), `hazard` and `motif` (the two deferred producers of x:024), `targeted` (the per-root pipeline), `wakeup` (detection) and `waking` (the first player variant that escalates only where a wake fires). Every result read off these surfaces is labelled with its thread ([walt-calculated-evidence](walt-calculated-evidence.md)).

### 2.2 The counted backward stack (the exact instruments)

`root_interval` (Slice A: a lower from one frozen lawful policy, an upper from the empirical best response, survivor sets by Theorem 2.1) → `grammar` (Slice B: `Q_a = max(Q^G_a, Q^dev_a)` as a walked identity) → `factor_belief` (Slices C–F: the opening root's 399,072,960 worlds as 116,280 acting-seat hands with exact-cover counts — gate-pinned in `tests/solver_factor_belief.rs` — the `SupportOracle`, the §23 fixed-policy recursion `viewer_success_mass`, the §48 grammar best response, the exact full response `response_success_mass`, consequence CEGAR) → `refine` (Slice G, the integrated controller, frozen as RefineV1) → `proof_state` (append-only typed facts under one `SemanticsIdentity`; everything else a derived view) with its producers `extraction`, `residual`, `covers`, `laydown`, `frontier`, `opening` → `doom` (the ∀-fail dual: counterexample mass as a deterministic upper) → `godgap` and `horizon` (the U0/U0b censuses of the price of playing blind) → `focal_horizon` and `focal_ladder` (the one object `[L_{a,k}, U_{a,k}]` per root action, indexed by the number `k` of focal decisions made exact; `k = 0` is the fixed-policy value below and the God upper above; the collapse at `k ≥ h_f` is the exact `Q`) → `model_belief` and `model_recursion` (the field itself as a hidden coordinate Ξ = Ω×Θ; the model-fusion price Φ) → `unified` (the five-tier cascade that plays with all of them).

Above the solver, `gym` composes `factor_belief`'s exact recursion with Scheme-matched coordinates under a fixed `GymField`, and `policy_search` builds sampled information-consistent policies against a frozen field and re-prices them.

### 2.3 The four seams

The two stacks share code at exactly four places, each a typed interface rather than a copied routine:

1. **`adaptive.rs` — the shared root identity.** `SlicePolicy` (the trait every field, tail and frozen policy implements), `DrivenState` / `driven_root` (the live-root bridge: seat, public record and own hand in, a canonical kernel out), `PublicRecord`, `RootPosition`, `CanonicalRoot` / `root_identity`, and `decided_success` — the one pmake-decided predicate, which ruling FH-A6 makes the **same** predicate at every depth of every exact recursion and in the sampled player's decided cutoffs. It is the module 33 of 37 solver modules import.
2. **The sampled minds as tails and fields inside every exact recursion.** `policy::Level0Field` is "exactly the field the live level-1 player models, exposed through `Solver::modeled_choice` — one authority, never a copy": the σ0 that `viewer_success_mass`, the doom walk and the focal engine consult at every non-viewer node. `policy::FrozenPolicy` with `ActionRule::PinnedThenLevel1` is a frozen lawful policy that plays a pinned tile at its root state and then "continues as the level-1 player under this freeze tuple" — `solver::level1_evaluate` on the policy's own discovery stream — which is what makes the sampled level-1 mind usable as the **lower tail** π of a root interval or a focal-horizon interval. `field::FieldKind::{Level0, Level1}` gives the same two minds immutable identities for the L2 thread. Raising the level changes the model, never the rules (O36).
3. **`root_interval` / `upper_cs` / `grammar` invert the CE engine.** The counted stack's sampled endpoints — `grid_upper_endpoint` and `grid_lower_endpoint` on the grid `G_N` with `N = |Φ(I)|` known exactly from the fiber counter — are the CE one-mean evidence engine of `evidence.rs` inverted, and the modules say so in their headers: "**[L2 consuming CE machinery]** through the sanctioned one-directional crossing."
4. **`unified.rs`'s `FieldFallback`.** The cascade's last tier (e) is "the declared σ0 field's own choice — total": the exact player ends, when every exact instrument has refused, at the sampled level-0 mind, and its `Provenance` says so. The other direction never happens — `unified` imports nothing from `model_recursion`, `refine`, `doom` or `godgap`, which it consumes only as facts in a caller-seeded `ReceiptStore` ("consumed, not recomputed — checkable"), and from `model_belief` only named items (its doc says "exactly five pre-existing public items"; the import at `unified.rs:162` names six, `ReadLedger` having joined for the carry ledger — a doc drift, §6.1).

The rule that governs all four is stated in `upper_cs.rs` and `root_interval.rs`: **L2 consumes CE machinery; CE never consumes L2.** The forward stack has no import edge into the backward stack; the backward stack reaches the forward one only through `adaptive`, `policy`, `field` and `level1_evaluate`.

### 2.4 Why the cascade is ordered as it is: the horizon and the wall, measured

`unified.rs`'s tiers — (a) `DecidedArithmetic`, (b) `EndgameExact`, (c) `MiddlegameMixture`, (d) `CertifiedRegret`, (e) `FieldFallback` — are entered on affordability and exited only by a typed refusal, and the module doc insists the order is "MEASURED, not guessed":

- **The fusion horizon is trick 5.** U0 (`godgap`, gate `tests/solver_godgap.rs`, 6 tests) censused fourteen trick-5/6 receipt coordinates God-tight and twelve trick-4 coordinates carrying positive information prices of 6–22‰ with `d_policy = 0` at every one; U0b (`horizon`, `tests/solver_horizon.rs`, 5 tests) then found the trick-5 frontier **inside** a trick-4 solve not fusion-free (mass-weighted Φ 13–14‰ at the h3-t4 cut-4 frontier in `walt/probes/factor_belief/horizon_run1.txt`, and a cut that over-prices by 15‰ without flipping the root play at contract 30). Tier (b) is therefore the endgame instrument.
- **The affordability wall is between trick 4 and trick 3.** MB1 (`model_recursion`, `tests/solver_model_belief_recursion.rs`, 7 tests) closed every trick-4 mixture coordinate — the model-fusion price strictly positive there, the pinned specimen h8-t4 3-1 at Φ = 38/9600 — and refused all five h8-t3 actions at the declared 7,000,000-read ceiling after 1,864 s (`walt/briefs/MB1-REPORT.md`; the whole run 2,379 s). The single-field exact recursion does reach h8-t3 — solved exactly under σ0 in 289,407,472 field consultations, `Q* = 28859/29988` (962‰), argmax 1-1 — wall 797 s (13 min 17 s) in the record (`walt/probes/factor_belief/horizon_run1.txt`, the `h8-t3 contract 30` block) and 14 min 13 s in the standalone trick-3 scout that `walt/briefs/U0B-REPORT.md` quotes; probe record, pinned by no gate at that coordinate — which is why tier (b) is affordable one trick deeper than tier (c). Past the wall both refuse and the player falls through, "which is the honest behaviour, not a failure."
- **What the residual width is made of.** FH1/FH3 (`focal_horizon`, `focal_ladder`; gates `solver_focal_horizon.rs` 10, `solver_focal_ladder.rs` 10, `solver_focal_anchors.rs` 4) showed that at `k ≥ 1` the remaining width is the tail's policy gap (`Q − L` 9–41‰), not fusion price (`U − Q` 0–3‰); every live trick-4 coordinate settles by `k ≤ 2` and the trick-3 anchor only at the `k = 3` collapse. That finding — a better lawful tail buys more than a deeper search — is what the consolidation program (§6.1) is about.

The doom-census sentence that once closed this story ("the plateau's Γ ≈ 267‰ is overwhelmingly the info-consistency price") is recorded as an overclaim in `walt/DISCREPANCIES.md` (2026-09-03): a zero doom census moves only `d_phys`; the split of the 267‰ into `d_info + d_policy` is **unknown**. The correction travels with every quotation of the number.

---

## 3. Invariants carried by types, not promises

The crate's discipline is that a law which can be made unforgeable by a type, a private constructor or a CI stage is never left to convention. The instances below are the ones a reader will meet first; each is verified against the tree on 2026-09-13.

### 3.1 No floats

Five mechanisms, all in `walt/ci/check.sh` (§4.1): (i) `cargo clippy --workspace --all-targets -- -D warnings -D clippy::float_arithmetic`, backed by `float_arithmetic = "deny"` in every one of the six manifests; then the `== no-float gates` stage's four scans — (ii) a token grep for explicit `f32`/`f64` types and suffixes over `*.rs` and `*.toml` in `walt/` **and** `rob/crates/{core,player,verify}` (rob is bound as the independent oracle); (iii) an MSL grep rejecting the `half`/`float`/`double`/`bfloat` families (scalar, vector, matrix, packed, simdgroup) plus `ci/check_msl_no_float.awk` over the two Metal shaders; (iv) `ci/check_rust_no_float.py`, a fail-closed lexical scanner that rejects inferred float literals after stripping comments, strings, raw strings, byte strings and character literals, over all six crates and rob's three; (v) `ci/check_toml_no_float.awk` over every manifest and lockfile including `lean/lakefile.toml`. `grep -rnE '\b(f32|f64)\b' walt/walt/src` returns nothing (measured 2026-09-13). Exactness is `BigInt`/`BigRational` (`num-bigint`, `num-rational`) in the solver, `i128`-backed rationals in `geom`, `U256` limbs in `spec`.

### 3.2 Overflow and unsafe

`overflow-checks = true` in dev, release and test (workspace `Cargo.toml`): an integer wrap panics in the release binaries the probes run. `unsafe_code = "forbid"` in `walt`, `walt-gpu-ref` and `walt-m2-runner`; `"deny"` with a scoped allow only where `#[no_mangle]` exports (wasm) or the Metal ABI (`walt-metal`, which also denies `unsafe_op_in_unsafe_fn`) require it. `spec/mod.rs` and `carrier/mod.rs` additionally carry `#![forbid(unsafe_code)]` at the module root, a leftover of their crate days.

### 3.3 Derived views, never stored state

The rule (CLAUDE.md, "Code discipline") is that cells, fibers, normal forms and every summary are *functions* of the semantic state; storing both authorities is forbidden. Four load-bearing instances:

| Where | The stored authority | The derived view (never stored) |
|---|---|---|
| `proof_state.rs` — "THE LAW" | the append-only typed facts under one `SemanticsIdentity` | installed intervals, proof bar `B = max_a L_a`, executable bar `B_exec`, survivors, exclusions, the typed result — all recomputed by `ProofState::closure`; "closure is idempotent by construction — and gated anyway" |
| `field_swap.rs` — the admissible screen | the rows (action, baseline interval, exposure bound) | `L^(1)`, `U^(1)`, the bar, the admissible set, the slack table, the result kind — "no second authority"; an `Unresolved` baseline can never yield a `FieldStable*` kind |
| `unified.rs` — provenance | the `Evidence` sum type carried by a `Decision` | `Provenance::tier` — "There is no `tier` field to lie in"; `Provenance` and `Decision` have no public constructor, so the only decisions in existence are the ones `UnifiedPlayer::decide` returned |
| `focal_ladder.rs` — the ladder root | the node facts of the append-only store | `FocalLadder::root_view` — "a pure function of the fact set"; the law that makes resume ≡ uninterrupted (gated in `solver_focal_ladder.rs`) |

The same shape recurs in `rules` (effective incidence, legal set, void set and remaining hands are functions of the state), in `field.rs` ("the level is a derived view of the variant, never a second stored field") and in `unified.rs`'s carried posterior (UP1a: "the posterior is a derived view of (root, public line) and nothing else" — so it is materialized only when a tier reads it).

### 3.4 Seed discipline

No random stream in the crate reads a clock, a thread id or a call count. Every stream is a pure function of a **named domain constant**, the seat's **own hand**, and the **public record's hash** (`record_hash(&Key)` in `mod.rs`), which is what makes a session record-grade: "no decision's sample depends on how many decisions preceded it" (the O27 repair of 2026-08-24, `walt/CONTROLLER-PLAYER.md`). The constants, verified on the tree:

| Constant | Value | Where | Stream |
|---|---|---|---|
| `INNER_SEED` | `0x243F_6A88_85A3_08D3` | `solver/mod.rs:127` (`pub`); private copies of the same value in bins `level1`, `level2`, `playout`, `walt_bridge`, `playtable`, `divergence` | the modeled minds' inner belief worlds (SCENARIO-PLAYER Def 3.2/3.6) |
| `ACT_FALLBACK_SEED` | `0x4528_21E6_38D0_1377` | `solver/act.rs:68` | the acting controller's level-1 fallback discovery stream: `seed ^ mix(own hand) ^ record_hash(record)` |
| `STREAM_DOMAIN` | `0xCE00_51CE_0A6E_57A6` | `solver/adaptive.rs:49` | the CE evidence world stream; `SAMPLER_ID = "kernel-fiberdp-splitmix64-counter-v1"`; world `i` is a pure function of (root identity, epoch, `i`) |
| `DISCOVERY_DOMAIN` | `0xD15C_0FEE_D5EE_D001` | `solver/policy.rs:64` | frozen-policy discovery (§12.4): distinct from `STREAM_DOMAIN` by a gate |
| `FIELD_DOMAIN` | `0xF1E1_DFAC_E5EE_D003` | `solver/field.rs:63` | the σ1 field's inner evaluation; "the three derivations must never collide, and a test asserts the tags differ pairwise" |
| `COUNT_TIMING_SEED` | `0x40F1_1B24_2026_0823` | `solver/calibrate.rs:913` | the count-timing fixture family |
| `DEFAULT_SEED` | `0xB7E1_5162_8AED_2A6B` | `walt-wasm/src/api.rs:64`, `walt2-wasm/src/api.rs:60` | the browser oracles' decision stream (same formula in both, so walt1/walt2 price the same request over the same outer worlds) |

The discovery/evidence disjointness is the §12.4 law: a policy discovered on epoch `e` refuses evaluation on epoch `e` (`root_interval::PolicyProvenance::Discovered`), because "a same-stream selected argmax is not a lower witness."

### 3.5 Cache purity

Every modeled-mind value is cached under `(level, PiKey)` and must be a pure function of that key (SCENARIO-PLAYER Def 3.4). `PiKey` (`mod.rs:234`) carries `voids: Option<[u32; 4]>`, `seat`, `hand`, `played`, `leader`, `plays`, `banked_t1`, `banked_t0`; `Key` (`mod.rs:192`) carries `voids`, `played`, `leader`, `plays`, `banked_t1`, `banked_t0`, `alive`. The cautionary tale is in the source and the register: from `level1.rs`'s birth through 2026-08-18 `PiKey` **omitted the banked totals**; serial execution masked the defect as first-come aliasing (the first caller's banked context decided the cached policy for every later context sharing the reduced key), the rayon port made the alias racy, non-determinism surfaced within hours, and the fix (`f5fff915`, applied to every surface at `1fc23196`) restored purity — after which 1-thread and 18-thread runs are byte-identical. The 3×384 arena pool against the E[Q] champion was completed on the **pre-fix** binary and is so labelled ([walt-seat-play](walt-seat-play.md)). The 2026-09-06 additions extended the key rather than bypassing it — `voids` entered both `Key` and `PiKey` — and `Shared::with_inner_belief` / `with_modeled_selection` `assert_eq!(self.pi_cache_len(), 0, …)`: a strategy cannot change once anything is cached.

Two further cache laws are structural: the frozen-policy action cache (`policy::FrozenPolicy`) and the field action cache (`field::FieldModel`) are **insert-only** — private, one miss-then-insert write path, no replacing or removing API, so "changing a defined action is impossible by construction"; and `policy::InfoKey` and `field::FieldStateKey` have a single constructor taking the acting seat's own remaining hand plus a `PublicRecord` — neither module imports the kernel's `World` (they import only `SplitMix64` from `kernel`, tree fact), so "the evaluation world's hidden hands are unreachable from any key or any discovery derivation, by type."

### 3.6 Typed refusals; caps are resource limits

A cap is never a settlement rule (CE-A3/A5, `adaptive.rs`): a world cap produces `Unresolved`, a read ceiling produces a refusal, and the refusal is a **value** the caller must match on — `solver::refine::RefusalReason`, `targeted::{RefusalReason, TypedRefusal}`, `frontier::Refusal`, `godgap::Refusal`, `model_belief::MixtureRefusal`, `model_recursion::CouplingRefusal`, `focal_horizon::FocalRefusal`, `unified::TierRefusal`, `partnership::{RefusalReason, Refusal}` (nine refusal enums, tree fact). `unified.rs` holds no `unwrap`, `panic!`, `unreachable!` or `todo!`, and every `expect` it holds is annotated `(rules invariant)` — gate UP1 greps for exactly that. The CE result kinds are the six-way ladder of the parent, "mechanically distinct, serialized with the type preserved" (CE-A3), and the acting controller's `ActRoute::settled` is `false` on every fallback route.

### 3.7 The exploratory fence, in the source

`lib.rs` fences the crate ("Everything is exploratory tier"), and every binary's header repeats it. Inside `solver/`, 28 of the 39 files (`mod.rs` included) state "exploratory tier" in their own doc comment; the ten that carry no tier word at all (`covers`, `doom`, `extraction`, `frontier`, `godgap`, `inner_belief`, `laydown`, `opening`, `partnership`, `residual`; `selection.rs` says only "an exploratory elimination policy") and the seven pre-fold module roots (`rules`, `kernel`, `geom`, `spec`, `strat`, `carrier`) plus `policy_search.rs` rely on the crate-level fence and on their gate files' headers (tree fact, 2026-09-13; a doc gap, not a tier question — §6.1). The point of the fence is the citation rule at the top of this page: nothing above the ideas tier cites any of it.

### 3.8 Freeze 58 as a code freeze

Freeze 58 (APS-A9; the number issued by [walt-math-freezes](walt-math-freezes.md)) freezes an **implementation** semantically: `walt/walt/src/solver/refine.rs` as merged at `25b40d9f` (PR #69, 2026-08-30) — "no new fields, enum variants or work items, ever; bug fixes only with independent justification; its four gates in `tests/solver_factor_refine.rs` never weaken." The tree honours it: `refine.rs` has exactly one commit in its history, `unified.rs` and `opening.rs` state they never modify it and reach it only as a frozen oracle, and `tests/solver_factor_refine.rs` holds exactly four `#[test]` functions. The design reason is stated in the register: "the growing-enum temptation is refused by rule, not judgment" — new capability goes to the proof-state core, never to this file.

---

## 4. The gate and what it costs

### 4.1 `walt/ci/check.sh`, stage by stage

The gate re-executes itself in a clean environment (`env -i` with `HOME`, `PATH=/usr/bin:/bin:/usr/sbin:/sbin`, `LC_ALL=C`, `TMPDIR=/tmp`), refuses every ambient `RUSTFLAGS`/`CARGO_*`/`GIT_*`/`ELAN_*` variable and every `.cargo/config`, requires the rustup proxies at `~/.cargo/bin` and `lake` at `~/.elan/bin`, builds into a fresh `mktemp` target directory, and runs, in this order (tree fact, the script's own `echo "== …"` lines):

| # | Stage | What it does |
|---|---|---|
| 1 | immutable M0/M1 history at its producing commit | `ci/verify_m2_history.sh` — reads the freeze-55 bytes at commit `3b4c6d60` through `git cat-file`, never through current paths |
| 2 | frozen GPU-native trick-1 guide identity | `shasum -a 256 -c math/gpu_native_trick1_implementers_guide_v0.2.sha256` |
| — | *(freeze-event only)* | `ci/verify_m2_sources.sh` — the cumulative source closure; demoted to a freeze-event verification by FZ-A5 because the unified crate puts actively developed solver code inside the closure's package trees |
| 3 | deterministic M0/M1 compatibility replay | `cargo run -p walt-gpu-ref --example generate_m0_m1_receipts` into a temp dir, then `diff -r receipts/gpu_native_trick1_m0_m1_v1` — the one byte-diffed receipt stage walt has |
| 4 | `cargo fmt --all --check` | |
| 5 | `cargo clippy --workspace --all-targets -- -D warnings -D clippy::float_arithmetic` | target `aarch64-apple-darwin`, `--locked` |
| 6 | no-float gates | the four scans of §3.1 |
| 7 | `cargo test --workspace --release --no-run --message-format=json \| ci/run_test_binaries.py` | build every test executable, then run them concurrently (§4.2) |
| 8 | `cargo test --workspace --release --doc` | doc tests (13 at FH4) |
| 9 | Lean trick-1 foundations and exact axiom audit | `lake build Texas42.Trick1Foundation Texas42.Trick1MetalFoundation`, then `diff -u trick1_metal_foundation_axioms_v1.txt` against a fresh `lake env lean` run |

A failure with a `FAILURE_OUTPUT` argument is rendered by `ci/render_m2_failure.py` as a typed freeze-56 failure artifact (the bootstrap path, "no success encoding, never overwrites"); success prints `walt ci/check.sh: PASS`. The script carries **no vocabulary grep** — rob's does — which the FH4 audit noted after catching "sandwich" as an object name by hand (`walt/briefs/FH4-AUDIT.md`).

### 4.2 The concurrent runner and the recompute-once fixtures (CI1, 2026-09-04)

Cargo runs test binaries one after another, and walt's gate suites are dominated by a handful of long exact recursions, so serial execution *was* the wall (the 2026-09-04 23:09 serial log: 125 binaries, sum of suite walls 498 s plus FH1's 350 s suite; `walt/briefs/BRIEF-CI1.md`). The concurrent runner was wired at `e53752b5` the same day (gate wall 367 s with FH1's 311 s suite inside); BRIEF-CI1 then verified it and changed recomputation only — "same assertions, same corpora, same oracles" — in two parts:

- **`ci/run_test_binaries.py`** (stdlib Python, run under `python3 -I -B`) reads cargo's JSON stream, collects every `compiler-artifact` with `profile.test`, and runs them with a bounded pool of `max(2, cpu_count // 2)` workers — **9** on this 18-core machine (measured 2026-09-13). Each binary runs with its package directory as cwd and `CARGO_MANIFEST_DIR` set; output is captured and printed **in full** on failure between `FAIL … full output follows` and `---- end of output`; every binary runs even after a failure; the report ends with the eight slowest suites and the sum of suite walls; exit status 1 if any failed. "Nothing about which assertions run changes — this is a scheduler." A `HEAVY_FIRST` tuple orders the long suites first so the makespan approaches the longest single suite; it names nine suites, one of which — `solver_focal_budget` — **does not exist** under `tests/` (tree fact; under `walt/` the only occurrence of that string is the tuple itself — the others are this page, [walt-instruments](walt-instruments.md) and [[wiki-book-followups]] recording it). Because the tuple is "purely an ordering hint — every suite runs", the stale name is harmless; it is recorded in §6.1.
- **`tests/common/fixture.rs`** — one `LazyLock` fixture per heavy suite, built by `compute_all`, which evaluates a key list once across `available_parallelism` threads, heaviest keys first. "Independence that matters is between CODE PATHS, never between recomputations of one function; a gate whose law is 'two runs agree' keeps one fresh run and compares it to the fixture's." Suites include it by `#[path = "common/fixture.rs"] mod fixture;`. `tests/common/mod.rs` holds the receipt-derived kernels and the `EXP5_FIBERS` regression pins (fiber sizes per (hand, trick) from the exp5 probe corpus — "regression pins, not authority").

The fixtured suites and what stays fresh by law are tabulated in `walt/briefs/CI1-REPORT.md` item 2 (`solver_factor_refine`, `solver_horizon`, `solver_unified_carry`, `solver_focal_horizon`; the anchors suite followed at FH3).

### 4.3 The suite inventory

Tree facts, 2026-09-13: **70** integration-test files under `walt/walt/tests/` (36,631 lines), **503** `#[test]` functions (counted as attributes at line start), and **2** `#[ignore]` attributes (§8 records why the 2026-09-07 survey said five). The runner reports 121 test executables at CI1 and 123 at FH3/FH4 — every crate's lib and bin targets get a harness too, so the executable count exceeds the file count. The full list, grouped by the module each suite gates, with one line from each header (rows ordered as the files sort within a group; the test count is `#[test]` occurrences in the file):

**Rules, kernel, geometry, operators, ABI, carrier (20 files)**

| Suite | Tests | From its header |
|---|---|---|
| `rules_exhaustive` | 13 | Every exhaustive count the spec states is asserted here (v0.4 §1.1–§1.4). |
| `rules_receipt_replay` | 3 | The ground-truth bridge: all 13 hands of rob's `verify_player.txt` re-derived from the rules alone (read-only on the receipt). |
| `kernel_known_fibers` | 9 | Fiber sizes cross-checked against the exp5 probe corpus; what is authoritative is the internal agreement of counting DP, enumeration and sampler. |
| `kernel_receipt_decisions` | 1 | The arbitrary-decision-point kernel constructor (`ReceiptDecision`, v0.4 §2.1) against all 13 hands × 4 seats × 7 decisions — 364 decision points. |
| `kernel_sampler` | 5 | Every draw is a fiber member, and the collision fingerprint sits where exact uniform sampling puts it. |
| `geom_envelope_exhaustive` | 4 | Exhaustive small-case audit of the envelope `_combine` merge against scalar reference semantics. |
| `strat_exp4_information` | 7 | Cross-validation against v0.4 §14.5 (Experiment 3B) and §14.6 (Experiment 4A) on the trick-5 kernel of receipt hand 0. |
| `strat_exp5_census` | 4 | Cross-validation against the exp5 census probe suite (`walt/probes/exp5`, exploratory tier). |
| `strat_trick6_census` | 5 | Cross-validation against the v0.4 §14.2 trick-6 experiment on receipt hand 0. |
| `spec_m0` | 12 | (no doc header) frame, U256 and semantic-table key encodings of the M0 ABI. |
| `carrier` | 7 | (no doc header) the admitted carrier's frozen source identities, the hand-8 cut replay, and the support "exact, complete and independently reproduced". |
| `scheme` | 19 | Finite-domain semantic checks for the Scheme/Fix runtime; expected answers constructed independently of the query compiler. |
| `scheme_dynamics` | 10 | (no doc header) finite transitions: `step_frame`, `step_belief`, transport and back-anchoring of answers. |
| `scheme_policy` | 6 | (no doc header) text round-trip of the exact information key; exact rules precede relations and are checked for legality. |
| `gym` | 8 | The gym's own-hand/public-history bridge, Scheme descriptor, exact values, and the zero-completion regression first exposed by a third-hand offer. |
| `policy_search` | 9 | (no doc header) the sampled search against the fixture root: parity of persistent and fresh search, replay, export. |
| `information_prices` | 8 | (no doc header) the exact tree matches an independent search and retains all root actions; centered finite price bounds. |
| `relational_learning` | 6 | (no doc header) tied action costs do not force a relational split; changing only a state-common upper cannot rerank programs. |
| `relational_runtime` | 6 | (no doc header) the choose-trace distinguishes exact relation and final fallback; the empty and own-legal programs. |
| `learning_io` | 2 | (no doc header) public-lesson round trip preserves costs, weights and observation; the rational wire format refuses undefined or ambiguous numbers. |

**Solver — the sampled forward stack (6 files)**

| Suite | Tests | From its header |
|---|---|---|
| `solver_viewer_fiber` | 2 | Cross-fiber pricing lawfulness (`viewer_fiber_evaluate`, the cheap first-order detector of LEVEL2-PROBE.md). |
| `solver_ordering` | 4 | The `solve_viewer` visit order and its break instrumentation (reorder-not-cull; E-A15: the ORDER of evaluation is lawful to change, the legal SET is not). |
| `solver_panel_conformance` | 8 | The panel-response conformance audits (PANEL-A3/A5/A6). |
| `solver_sigma1_repair` | 8 | The σ1-repair slice: terminating the void-conditioned belief sampler and deduplicating its five copies onto one library authority. One test `#[ignore]`d: it regenerates the committed before-side evidence. |
| `solver_selection` | 7 | Exact synthetic schedules: independent expected work and winners. |
| `solver_partnership` | 12 | Focused gates for the bounded partnership evaluator. Exploratory tier. |

**Solver — the calculated-evidence path, CE thread (9 files)**

| Suite | Tests | From its header |
|---|---|---|
| `solver_evidence` | 8 | V-gates for `solver::evidence` — the exact-arithmetic acceptance of the CE slice (parent §19: V1, V2, V3, V7; CE-A1/A5). |
| `solver_adaptive` | 6 | V-gates for `solver::adaptive` — the vertical-slice acceptance (V8 mini, V9 mini, V4 mini; §22 steps 2/5/6 in miniature). |
| `solver_policy` | 7 | The frozen policy authority of parent §12 (§22 step 4; CE-A3/A5/A7; O13/O22/O27). |
| `solver_controller` | 14 | The §16.4 decision controller generalized to `m` candidates: all-pairs allocation, safe elimination, epoch mutation, practical equivalence. |
| `solver_calibrate` | 11 | §22 step 8: the V5 flip repair and the per-fixed-pair E0 calibration arithmetic. One test `#[ignore]`d — "blocked: plunge-side game seeds (L2-A6 [[gran-anchor-reconstruction]])". |
| `solver_shadow` | 10 | §22 step 7's library pieces: the live-root bridge (`driven_root`) and the frozen level-1 continuation policies (`ActionRule::PinnedThenLevel1`). |
| `solver_act` | 5 | `solver::act` — the §16.4 controller as an ACTING player; the action policy is a pure function of the controller result. |
| `solver_field_cache` | 5 | The two surgical levers in the acting hot path: the cached `FieldModel` and the decided cutoff. |
| `solver_bundle` | 5 | The bundled world evaluator against the enumerated per-world oracle, element-wise. |

**Solver — the targeted level-2 family, L2 thread (9 files)**

| Suite | Tests | From its header |
|---|---|---|
| `solver_fieldswap` | 11 | `solver::field` and `solver::exposure` — the field-swap vertical slice (§21 steps 3–4). |
| `solver_fieldswap_screen` | 9 | The exposure rung producers E0/E1/E2, the exact split-reach route E4, and the admissible level-2 action set. Its own gate epoch is the small pair σ0 `Level0 { n0 = 2 }`, σ1 `Level1 { n_outer = 2, n0 = 2 }` — not the probe bins' `n0 = 8` / `4×2` pair (§5). |
| `solver_fieldswap_cancel` | 12 | The cancellation ladder, pairwise benefit/hazard masses, the six-label cancellation vocabulary with the dominance type-lock. |
| `solver_e3_upper` | 8 | Slice 4a: the δ-valid admissible-upper E3 producer and its directional variants. |
| `solver_hazard_witness` | 7 | Slice 4b: the Hazard-Exclusion Invariant verifier, the δ = 0 `StructuralHazardZero` result, the exact benefit exhibit. |
| `solver_fieldswap_motifs` | 6 | Slice 4c: the six-motif first-split morphology classifier, the raw post-split suffix enrichment, root-frame resolution. |
| `solver_targeted` | 7 | The targeted field-1 controller: parent §8 Stages 1–5 assembled. |
| `solver_wakeup` | 9 | `solver::wakeup` — the level-2 detection layer (§22 step 9; CE-A6, role fixed by L2-A5). |
| `solver_waking` | 9 | The waking seat: act's σ0 baseline, the hard-budgeted wake check, the wake-gated σ1 escalation, the per-decision census. |

**Solver — the counted-belief ladder, Slices A–G (7 files)**

| Suite | Tests | From its header |
|---|---|---|
| `solver_root_interval` | 6 | Slice A: root intervals and survivor sets — the pmake empirical-max upper (CBS-A2), the frozen-policy lower witness with the §6 discovery/evaluation lock. |
| `solver_grammar` | 8 | Slice B: the two-policy grammar and the residual split — `Q_a = max(Q^G_a, Q^dev_a)` as a walked identity. |
| `solver_factor_belief` | 11 | Slice C, stages C0/C1: the factor belief and the exact-cover contraction — mass parity with the shipped counting DP; pins 399,072,960 worlds and 116,280 acting-seat hands at the opening root. |
| `solver_factor_recursion` | 5 | Slice D: the general support contraction (`SupportOracle`) and the §23 factorized fixed-policy recursion. |
| `solver_factor_response` | 4 | Slice E: the §48 factorized grammar best response. |
| `solver_factor_consequence` | 4 | Slice F: the §49 consequence CEGAR — hand classes refined by witness pairs to the action-exact endpoint. |
| `solver_factor_refine` | 4 | Slice G: the §50 integrated refinement controller; gate 1 is the C→G capstone (the §36 EscalateExact endpoint). The four gates of freeze 58. |

**Solver — the anytime proof-state program and the doom census (10 files)**

| Suite | Tests | From its header |
|---|---|---|
| `solver_proof_state` | 6 | The §49 architecture spike: the zero-budget top state is sound and serializes/resumes bytewise; imported RefineV1 facts reproduce it. |
| `solver_frontier` | 6 | Phase 1: declared solve goals type their debts exactly; §42 steering bounds; the exact solve as a degenerate schedule. |
| `solver_factor_profile` | 5 | Phase 2: the §18 fixed-policy 43-bin score profile — mass conservation and tail projection to the Slice D success mass. |
| `solver_proof_regret` | 5 | Phase 3: contract projection and certified regret (`ProofState::recommend`, the §33 block). |
| `solver_residual` | 6 | Phase 4: the §61 score-aware residual Bellman — every intermediate F stage a valid root interval, the intervals nest. |
| `solver_covers` | 3 | Phase 5: the §62 count-threat covers — accepted covers never understate the residual score gain. |
| `solver_extraction` | 6 | Phase 6: §63 argmax extraction — the extracted policy re-prices to the extraction optimum through the unchanged evaluator. |
| `solver_laydown` | 4 | Phase 7: the §16 typed laydown hierarchy proved on structural fixtures (a boss chain, an already-made root). |
| `solver_opening` | 5 | Phase 8: the §65 opening-root iterative run; the zero-budget stop round-trips deterministically. |
| `solver_doom` | 8 | The doom census — counterexample mass as a deterministic upper; an already-set root dooms every world. |

**Solver — the closing round and the focal-horizon hierarchy (9 files)**

| Suite | Tests | From its header |
|---|---|---|
| `solver_model_belief` | 8 | MB0: the field model as a persistent hidden coordinate over the counted-belief machinery. |
| `solver_model_belief_recursion` | 7 | MB1: the model-belief recursion joins the solver — plus the read ledger and the typed budget refusals it rests on; gate M6 pins Φ = 38/9600 at h8-t4 3-1. |
| `solver_godgap` | 6 | U0: the §8 three-part failure decomposition made mechanical, the four §48 result types; re-derives the §9 table from the committed record. |
| `solver_horizon` | 5 | U0b — the in-solve horizon census. Five gates (H1: the frontier re-descent reproduces the root). |
| `solver_unified` | 18 | UP0 — the unified walt player. Six gates (UP1 totality and the no-swallowed-refusal discipline, …). |
| `solver_unified_carry` | 5 | UP1a — the lazy carry. Five gates (UC1: nothing read, nothing paid). |
| `solver_focal_horizon` | 10 | FH1 — the focal-horizon hierarchy engine: the parent's gates FH1–FH6 plus the free ones. |
| `solver_focal_ladder` | 10 | FH2 — the ladder: budget honesty, interruption and resume, proof-state facts, exact suffix reuse. |
| `solver_focal_anchors` | 4 | FH3 — the FH8 anchors at the coordinates ruled in FH-A8; the fixture caps heavy h4-t4 jobs at `HEAVY_IN_FLIGHT = 5` (FH4-AUDIT N13). |

### 4.4 Wall and memory, stated as findings

Machine figures (18 cores / 48 GB), never receipts; each with its record:

| Measurement | Value | Record |
|---|---|---|
| Gate wall with the concurrent runner wired but before CI1's fixtures (`e53752b5`, 2026-09-04) | **367 s** (`solver_focal_horizon` alone 311 s in-gate); the serial test stage before the runner summed 498 s of suite walls plus FH1's 350 s | `e53752b5` commit message; `walt/briefs/CI1-REPORT.md` item 3; `BRIEF-CI1.md` findings |
| Gate wall after CI1 (concurrent runner + fixtures) | **230 s**; 121 binaries, 0 failed; runner makespan 177 s (`solver_unified_carry`); sum of suite walls 1,317 s contended | CI1-REPORT item 1(a), item 3 |
| Standalone suite walls, before → after CI1 | `solver_factor_refine` 146 → 70 s; `solver_horizon` 116 → 50 s; `solver_unified_carry` 61 → 48 s; `solver_focal_horizon` 350 → 137 s | CI1-REPORT item 3 |
| Gate wall after FH3's anchors gate | **308 s** (5 min 8 s); 123 binaries; sum of suite walls 1,367 → 2,126 s; the anchors suite 200.6 s in-gate, `solver_focal_horizon` 141 → 231 s under the contention | `walt/briefs/FH3-REPORT.md` "WALL" |
| Gate wall at the FH4 audit (clean tree, foreground) | **306.65 s**; 123 binaries; doc tests 13/13; Lean built | `walt/briefs/FH4-AUDIT.md` |
| `solver_focal_anchors` standalone | 82.6 s (FH3 said ~85 s), **18.22 GB** peak RSS (FH3 said 17.8 GB) | FH4-AUDIT, `/usr/bin/time -l` |
| Test-binary RSS inside the nine-at-a-time runner (FH4, before the FH5 cap) | up to 21.0 GB summed over the runner's nine processes (a 3 s sampler saw ≤ 21.0 GB), largest single 8.5 GB (`solver_focal_horizon`); largest single descendant peak 8.81 GB | FH4-AUDIT |
| After FH5 capped h4-t4 jobs at five in flight (`b6de5a25`) | anchors gate **18.21 → 8.76 GB** peak RSS; standalone wall 78.5 → 164.8 s (light h8-t4 jobs now queue behind the cap); in-gate 200 → 248 s; `check.sh` PASS, 123 binaries | `b6de5a25` commit message (N13); `walt/MAP.md` cost table; `kanban/backlog/gate-corpus-trim.md` |
| The FH3 record run (h8-t3 ladder, 33 coordinates × k ≤ 3) | 23 min 55 s; **19.4 GB** peak RSS; 3.82M facts, 3.55M memo receipts; the standalone h8-t3 ladder 17.1 GB | FH3-REPORT; `kanban/backlog/ladder-policy-store.md` |
| FH2 memory at h3-t4 | ladder 662 MB with the memo, 509 MB without, versus 411 MB for FH1's direct `k = 2` engine | `walt/briefs/FH2-REPORT.md` |
| The exact recursions' cost, by trick | trick-6 coordinates microseconds to 94 ms; trick 5 seconds; trick 4 minutes (0.66M σ0 reads h8-t4, ~10M h4-t4, `focal_run0.txt`); trick 3 the wall — h8-t3 289,407,472 reads, 13 min 17 s in the record run (14 min 13 s in the standalone scout) single-field; refused at 7M reads per action in the mixture | `walt/MAP.md`; MB1-REPORT; U0B-REPORT; `walt/probes/factor_belief/{focal_run0,horizon_run1}.txt` |

Two readings of the trend belong here rather than in an era page. First, **memory is the cost growing fastest** (MAP.md): each h4-t4 evaluation costs about 1.6 GB "whatever its kind" (the `HEAVY_IN_FLIGHT` comment in `solver_focal_anchors.rs`), the ladder copies a full policy table into every ancestor's lower fact (FH-int: every lower carries its witness), and the FH4 auditor's judgment was "acceptable to land with the card, NOT a BLOCK — it runs on the machine it targets — but the gate is one hardware change from silent failure." Second, **field classification is 99% of every bill** (the σ0 reads the full public record, so cross-history cache reuse measured exactly 0), and a warm σ0 instance runs a pass 15× faster at identical reads (FH3) — which is why the σ0 read-key study is MAP.md's named 10–100× lever (§6.1).

### 4.5 The gate-sizing law and its two cards

CLAUDE.md's rule (2026-09-04): a gate is sized to its laws, not to a census — **one coordinate per law plus a PINNED strictness witness**; a corpus sweep belongs in a probe record; expensive oracle values a suite needs in several gates are computed once in a shared fixture, and independence is between code paths, never between recomputations. Two backlog cards carry the debt this rule names:

- **[[gate-corpus-trim]]** (opened 2026-09-04): several suites still sweep ten roots × contracts × cuts so that "strict somewhere on the corpus" has something to find — `solver_factor_refine` (146 s serial at review), `solver_horizon` (116 s, H1's 40 censuses), `solver_unified_carry` (61 s), `solver_focal_horizon` (350 s at landing). Trim suite by suite when each is next touched, pinning the witness coordinate by name. Done when no suite runs above ~60 s without a stated reason in its module doc.
- **[[ladder-policy-store]]** (opened 2026-09-04): replace the ladder's copied policy tables with a version-referenced store where each fact references child policies by content id and the total policy is a derived view. FH4 N8 found two sinks to measure first (the fact store ≈ 98 MB and the memo's `FactorBelief` clones ≈ 153 MB at h3-t4). Done when FH2's gates and FH3's anchors pass unchanged, the h8-t3 record reproduces byte-for-byte, and peak RSS is under 2 GB at h8-t3 and under 4 GB for the anchors gate.

### 4.6 The 2026-09-06/07 landings, merged with the gate waived

Every entry in `walt/LOG.md` dated 2026-09-06 and 2026-09-07 — `walt::scheme` and the `scheme` bin (`b764665f`), `inner_belief` (`dbcc698f`), `partnership` (`cfb0fb25`), `selection` and `Field::SeatLevels` (`9236ca7f`), `gym` and `partnership_gym` (`c59f1115`), `policy_search` with `scheme/{dynamics,policy}` and `policy_lab` (`08fad726`), the relational learner and `relational_lab` (`c00717d1`) — ends with "full CI deliberately skipped/waived under the session waiver" or "full legacy Rust CI remains waived." Focused suites, clippy and fmt did pass per each entry (for example "61 distinct focused Rust tests", "eight gym + 19 Scheme Rust tests", "clippy and formatting passed"). **The last recorded green `walt/ci/check.sh` is the FH4 audit's 306.65 s run of 2026-09-04**, which predates every module listed in this paragraph. Whether the workspace gate is green at `c00717d1` is therefore not known from any record, and this page does not run it (it is not a sixty-second command). Recorded as the first open item of §8.

---

## 5. Declared epochs

Every walt number is relative to a declared epoch — the σ0/σ1 field identities, the frozen candidate schedule, the sample counts, the caps and the seeds — and the epoch is part of the result's identity (a record's header states it). Numbers from different epochs **do not compose**; `walt/probes/waking/README.md` and `walt/probes/gran/README.md` say it in bold about the pair below, and the rule generalizes. The table is the crate's epochs as declared in source and record headers (tree facts and record facts, 2026-09-13); the live level-1 surfaces' full configuration table is [walt-seat-play](walt-seat-play.md) §2.

| Epoch | σ0 (the level-0 modeled mind) | σ1 / frozen candidates | Caps and counts | Where declared | Used by |
|---|---|---|---|---|---|
| **L2-thread field-swap probes** | `Level0 { n0 = 8 }` | σ1 `Level1 { n_outer = 4, n0 = 2 }`; frozen `[8, 2]` | per slice | the probe bins' own headers (`bin/fieldswap_screen.rs:11`, `fieldswap_cancel.rs:19`, `fieldswap_motifs.rs:20`, `hazard_witness.rs:18`, `l2_controller.rs:12`); records under `walt/probes/{fieldswap_*,hazard_witness,l2_controller}/`. Every L2-thread gate file runs the smaller pair instead — σ0 `Level0 { n0 = 2 }`, σ1 `Level1 { n_outer = 2, n0 = 2 }`, frozen `[2, 2]` — declared in each header: `tests/solver_fieldswap_screen.rs:11`, `solver_fieldswap_cancel.rs:16`, `solver_fieldswap_motifs.rs:13`, `solver_e3_upper.rs:14`, `solver_hazard_witness.rs:19`, `solver_targeted.rs:7`, `solver_wakeup.rs:47–60`, `solver_waking.rs:10` (tree fact, 2026-09-13). Gate and probe epochs differ on this whole row; a value a gate pins here is pinned at the small pair. | `fieldswap_screen`, `fieldswap_cancel`, `fieldswap_motifs`, `hazard_witness`, `l2_controller` |
| **The acting / waking / gran epoch** | `Level0 { n0 = 2 }` — "the same field act's evaluation runs against" | σ1 `Level1 { n_outer = 4, n0 = 2 }`; frozen `[8, 2]` (`ActionRule::PinnedThenLevel1`) | `ActConfig::interactive`: world cap 128, exact cap 2000, fallback 200×8; δ_run = 1/100 per hand; wake budget 24 paired worlds, exact wake cap 1024, escalation caps 4096/128/24 | `solver/act.rs` (`ActConfig::interactive`), `solver/waking.rs` (`WakingConfig::live`), `bin/granrun.rs` header, `walt/probes/{waking,gran}/README.md` | `controller_bridge`, `waking_bridge`, `granrun`, the `ctrl` seats of `webtable`/`playtable`; `wakeup` (same pair) |
| **Batch controller** | as above | as above | `ActConfig::full`: world cap 512 (the `shadow` bin's default, raised to 512 after the committed 128-epoch run under the cap ruling; the committed shadow records need `world_cap=128` passed explicitly) | `solver/act.rs`; `bin/shadow.rs` (`knob(512)`); `walt/probes/shadow/README.md` | `shadow`, `v5flip` |
| **Counted-belief, anytime, horizon and focal instruments** | `Level0 { n0 = 2 }` under `SupportOracle` ("level0-modeled-mind-v1 (Level0 n0=2) under SupportOracle", `horizon_run1.txt` header) | root-interval lower witnesses `PinnedThenLevel1 [2, 2]` (FIXED provenance); δ = 1/20 per endpoint at the receipt roots, 1/100 at the opening root | node fiber cap 40,000 on the enumerable corpus; read ceilings per report | every `walt/probes/factor_belief/*.txt` header; `tests/solver_focal_anchors.rs` (`AMPLE_CAP = 40_000`) | `rootinterval` … `focalreport`, `doomreport`, `openingreport` |
| **Model belief (MB0/MB1) and the unified player** | F₀ = `Level0 { n0 = 2 }` | F₁ = `Level1 { n_outer = 2, n0 = 2 }`; prior (1/2, 1/2) per hidden seat | MB1 read caps 4M (MB0 roots) / 12M (trick 4) / 7M (trick 3) | `walt/briefs/{MB1,UP0}-REPORT.md`; `bin/{modelbeliefreport,modelbeliefrecursionreport,unifiedreport}.rs` | `modelbeliefreport`, `modelbeliefrecursionreport`, `unifiedreport` |
| **The gym field** | opponents `Level0Field::new(8)`; partner a fixed level-1 mind at `partner_worlds`/8 | — | `Rule::Fixed`, `InnerBelief::Voidless`, `FieldProfile::Baseline`; id `gym-field-v1/partner=<seat+2>/l1-fixed-<partner_worlds>-8/inner=voidless/opponents=<σ0 id>/seed=420600-state-v1/tie=lowest/fallback=none` | `src/gym.rs` (`GymField::new`) | `partnership_gym`, `policy_lab --field gym` |
| **Live level-1 surfaces** (argv/env defaults, tree facts) | the Dice field inside each modeled level-0 mind at `n0` | — | `walt_bridge` n_outer 50, n0 8, 120 s per move (`WALT_N_OUTER`/`WALT_N0`/`WALT_PER_MOVE`); `webtable` 100/8, `ctrl` cap 128, port 4242; `playtable` 100/8, seed 42, `ctrl` cap 128; `playout` 3 games / 300 s / 200 / 8; `walt-wasm` n 40, n0 8, `race` opt-in, θ = 11/16; `walt2-wasm` n 8, n1 4, n0 2; `partnership` takes n/n0/n1 from each request within n ≤ 640, n0, n1 ≤ 64, budget_ms ≤ 14,000 — the 40/8/2 family defaults are the Python driver's | `bin/walt_bridge.rs:957–959`, `bin/webtable.rs:1301–1314`, `bin/playtable.rs:1077–1092`, `bin/playout.rs:787–796`, `bin/partnership.rs:153–157`, `walt-wasm/src/api.rs` header, `walt2-wasm/src/api.rs` header | the seats people play — [walt-seat-play](walt-seat-play.md) §2; [walt-partnership-program](walt-partnership-program.md) §3 |

The do-not-compose rule in the records' own words (`walt/probes/gran/README.md`): "**The `l2_controller` probe's epoch is DIFFERENT (σ0 n0 = 8): numbers do not compose across the two.**" The Plunge panel's forty-world percentages on the Gran hand do not compose with any row above either ([walt-gran-anchors](walt-gran-anchors.md)). A receipt-corpus caveat travels with every row that reads `rob/receipts/verify_player.txt` for its roots: the receipt hands are pip-trump contracts only.

---

## 6. Named debts and frozen things

### 6.1 The debts, as the source names them

| # | Debt | Where it is written down | Status at `c00717d1` |
|---|---|---|---|
| 1 | **`level1_evaluate` is triplicated**: the library authority `solver::level1_evaluate` (`mod.rs:1340`) and two bin-local copies, `bin/walt_bridge.rs:514` and `bin/playtable.rs:604`, each doc-marked "STILL TRIPLICATED … a named debt, and the σ1-repair slice deliberately did not pay it." `policy.rs` and `field.rs` call the library copy "the live player's one authority" while the arena bridge that beat the E[Q] champion runs its own. | the three doc comments; [walt-seat-play](walt-seat-play.md) §9 | open; which copy "the live player" means depends on the surface |
| 2 | **`playout.rs`'s local `PiKey` copy omits the banked totals** — the §3.4 defect, "deliberately NOT fixed" (`playout.rs:21`); the O27 repair (`23ba1c22`) domain-separated its deal and belief streams but left this filed. The σ1-repair (`161b0195`, "dedup the five copies") rewrote the table bins' samplers; the copy is still present (tree fact: `struct PiKey` at `playout.rs:144` with `banked_t1`/`banked_t0` fields at 116–117 belonging to its `Key`, not to `PiKey`). | `walt/CONTROLLER-PLAYER.md` "O27 fix"; `playout.rs` header | open; `playout` is a viewer feed, not an arena seat |
| 3 | **`INNER_SEED` is a `pub const` in `mod.rs` and a private `const` of the same value in six bins** (`level1`, `level2`, `playout`, `walt_bridge`, `playtable`, `divergence`). Same value everywhere; a duplication that would let the surfaces drift. | tree fact | open (never carded) |
| 4 | **[[ladder-policy-store]]** — the ladder's memory (§4.4/§4.5). | `kanban/backlog/ladder-policy-store.md` | backlog |
| 5 | **[[gate-corpus-trim]]** — census-sized suites (§4.5). | `kanban/backlog/gate-corpus-trim.md` | backlog |
| 6 | **The consolidation slice** — MAP.md's tree-shake list: in the hierarchy's vocabulary U0 is `U_{a,0}`, U0b is `U_{a,m−1}` (FH-cut), the never-built U1 salvation-mask upper is `U_{a,1}`, rollout improvement is `L_k`, argmax extraction is `π_k`; so `godgap.rs` (933 lines), `horizon.rs` (635) and `extraction.rs` (135) are measurement scaffolding around one recursion, and `refine.rs` (917, freeze 58) was already declared removable; `doom.rs` stays as the God tail's engine. Jason's ruling of 2026-09-04: "follow through on what we have, then invest in a simplification/unification attempt" — no new mathematical parent until it lands. | `walt/MAP.md` "What the hierarchy makes redundant", "Next, in order" | not started; no kanban card; the 2026-09-06/07 work went to Scheme, the gym and the partnership program instead |
| 7 | **The σ0 read-key study** — does the level-0 field's answer depend on the full public record? If the key coarsens, "every recursion gets 10–100× cheaper." | `walt/MAP.md` "Next, in order" | not started; no card |
| 8 | **`lib.rs`'s doc omits `gym` and `policy_search`** and still says "originally seven modules." In the same family: `unified.rs`'s doc says it imports "exactly five pre-existing public items" from `model_belief` while the import line names six (`ReadLedger`, §2.3). | tree fact (§1.2, §2.3) | open |
| 9 | **`HEAVY_FIRST` names `solver_focal_budget`**, a suite that does not exist. Harmless (an ordering hint), stale. | `walt/ci/run_test_binaries.py:33` | open |
| 10 | **Ten solver modules and the pre-fold module roots carry no "exploratory" line of their own** (§3.7); the fence is crate-level and gate-level there. | tree fact | open (a doc gap) |
| 11 | **`check.sh` has no vocabulary grep**; rob's does. The FH4 audit's one BLOCK ("sandwich" as an object name in a gate name and reports) was caught by reading, and fixed at FH5 (`b6de5a25`). | `walt/briefs/FH4-AUDIT.md` | open |
| 12 | **`walt/MAP.md` was stale on two rows** at `c00717d1`: the size row ("37,260 lines, 36 modules (after FH2)") and objects-table row 10, which labelled `CONTROLLER-PLAYER.md` "the live default player" while that document says the old level-1 sampling-stack seat remains the default everywhere. | tree fact vs `walt/MAP.md` | fixed 2026-09-13 in this rewrite (size row 38,013 / 39 files; row 10 now the sampling-stack seat, row 10b the controller variant); MAP.md is rewritten by the orchestrating session at every landing |
| 13 | **The workspace gate has not been recorded green since the 2026-09-06/07 modules landed** (§4.6). | `walt/LOG.md` | open |
| 14 | **The committed browser oracles may be behind their source**: `pkg/walt.wasm` was last rebuilt 2026-08-24 and `pkg/walt2.wasm` 2026-08-25, while both crates' `api.rs` changed at `161b0195` (2026-09-02) and the `walt` crate they link changed again on 2026-09-06. `experiments/partnership/BASELINE.md` deliberately freezes the *phone* artifact as an external anchor, so a stale repo `pkg` may be intentional. | survey of 2026-09-07; [[plunge-walt-sync]] | not verified in this pass |

### 6.2 The freezes, by what each pins and where

The full register is [walt-math-freezes](walt-math-freezes.md); this table is the engineer's index of the four freezes that bind code paths in this crate.

| Freeze | Ruling | What it pins | By path or digest | Verified per run by |
|---|---|---|---|---|
| **55** — portable GPU-native trick-1 M0/M1 | GT1-A9 (2026-08-16, commit `3b4c6d60`) | the bytes of `GT1_FREEZE_SET_DESCRIPTOR_V1` and the v1 encodings: `U256MassV1` (8 × LE-u32) at field scale 420, `SemanticTablesCanonicalV2`, the opening-cell generator and 11,730 cap, `ReducedOpeningCarrierV1` grades 2..5, the 100,000 direct-world cap and 756,756-world grade-5 stop, the M1 tasks and the run-envelope / declared-stop / build-identity schemas | descriptor in `walt/walt-gpu-ref/src/receipt.rs`; comparands `walt/receipts/gpu_native_trick1_m0_m1_v1/`; guide checksum `walt/math/gpu_native_trick1_implementers_guide_v0.2.sha256`; ABI constants in `walt::spec` (`FIELD_SCALE = 420`, `TABLE_FORMAT_VERSION = 2`, …) | `check.sh` stages 1–3: history at `3b4c6d60` (`verify_m2_history.sh`), guide checksum, receipt regeneration and `diff -r` |
| **56** — M2 Metal parity | GT1-A17 (2026-08-17, `813d5e81`) | the exact 899 ASCII bytes of `GT1-M2-FREEZE-SET-V1` (SHA-256 `7bdc5e05…`), the binding contract, the parity tasks, the receipt and source-manifest schemas | `walt/receipts/gpu_native_trick1_m2_v1/` (one binary receipt plus external checksum); descriptor in `walt-gpu-ref/src/m2_receipt/receipt.rs` | `ci/check_m2_metal.sh` (needs a Metal device; not part of `check.sh`) |
| **56 v2** — the re-issue at the unified layout | FZ-A1..A6 (2026-08-24, `c92175ae`) | the cumulative source manifest `walt/math/gpu_native_trick1_m0_m2_sources_v2.sha256` (identity `8a780895…`) beside the byte-immutable v1; a 32-entry fold-translation table (`walt-core → walt/src/rules`, `walt-kernel → walt/src/kernel`, `walt-gpu-spec → walt/src/spec`); full-closure checking demoted to freeze events; the standing M2 receipt explicitly **old-layout evidence** ([[m2-receipt-reearn]]) | the v2 manifest; `ci/verify_m2_sources.sh` | at freeze events only (FZ-A5); the immutable-history and replay stages remain per-run |
| **57** — the M3 perfect-recall-net gate | GT1-A24 (2026-08-17) | the exact 962 ASCII bytes of `GT1-M3-FREEZE-SET-V1` (SHA-256 `e5efe6ce…`), the carrier profile `M3CarrierProfileV1` (hand 8, roots 21-31-33-55); the gate only — **no M3 result** | `walt::carrier::constants` (tree facts: `FREEZE57_DESCRIPTOR_LEN = 962`, `RAW_RECEIPT_BYTES = 6_650`, `CARRIER_HAND_ID = 8`, `CARRIER_TRICK = 4`, `SUPPORT_COUNT = 1_200` of `VOID_FREE_PARENT_COUNT = 34_650`, `ROOTS`, `RAW_RECEIPT_SHA256`, `CARRIER_PROFILE_SHA256`, …); the sole admitted artifact is `rob/receipts/verify_player.txt` | `tests/carrier.rs` (7 tests: frozen source identities exact; the hand-8 cut replayed; the support exact, complete and independently reproduced by two constructors) |
| **58** — RefineV1 | APS-A9 (2026-08-31); number issued by the register | `walt/walt/src/solver/refine.rs` as merged at `25b40d9f` — semantically frozen (§3.8) | the file itself; one commit in its history | `tests/solver_factor_refine.rs` (4 tests, never weakened) |

Freezes 1–54 belong to the archived eras (Part III) and to walt's mathematics; the register owns them.

---

## 7. The `walt/ci/` scripts, one line each

| Script | What it is |
|---|---|
| `check.sh` | The portable gate (§4.1): clean-env bootstrap, immutable M0/M1 history, guide checksum, M0/M1 receipt replay byte-diff, fmt, clippy with `-D float_arithmetic`, the four no-float scans, all workspace release tests run concurrently, doc tests, the Lean trick-1 build and axiom audit. Run as `/bin/bash -p walt/ci/check.sh [FAILURE_OUTPUT]` from any directory; expect 230–308 s (306.65 s at the FH4 audit) and, at FH4 before the FH5 cap, up to ~21 GB of test-binary RSS summed across the runner's nine processes on the 48 GB machine (the anchors suite alone 18.2 GB then, 8.8 GB after `b6de5a25`). |
| `run_test_binaries.py` | The concurrent test-binary scheduler (§4.2): cargo's JSON stream in, a bounded pool of `max(2, cpu_count // 2)` workers, full output on failure, every binary runs, exit 1 on any failure. `usage: run_test_binaries.py TARGET_DIR` (measured 2026-09-13: exit 2 without the argument). |
| `verify_m2_sources.sh` | The freeze-56 v2 cumulative source-closure verifier over the 282-entry v2 manifest with the fold-translation table; a **freeze-event** verification since FZ-A5, not a per-commit gate. |
| `verify_m2_history.sh` | The freeze-56 historical verifier: inspects the freeze-55 bytes (the Gate-0 NO-GO record and the M0/M1 receipt files) at their producing commit `3b4c6d60` through `git cat-file`, "never through current-path reinterpretation." Stage 1 of `check.sh`. |
| `check_m2_metal.sh` | The elevated freeze-56 conjunction (needs a Metal device): the portable conjunction, an immutable committed source snapshot, the release M2 runner, a checked host/tool/device descriptor and a two-build metallib, canonical Rust Gate 0 with U256 parity and negative controls, timeout / malformed-protocol / no-partial controls, a discarded maximum-projector smoke child, two fresh official M2 children, typed receipt adjudication against the immutable HEAD comparands, the final Lean audit and the final cumulative source identity. "Success exists only after the discarded smoke, two fresh complete official children, and exact receipt regeneration." Its final step is freeze-event dependent since v2 ([[m2-runner-trace]]). |
| `check_rust_no_float.py` | Fail-closed lexical scanner for inferred Rust float literals after stripping comments and literals; called by `check.sh` over the six walt crates and rob's core/player/verify. |
| `check_msl_no_float.awk` | Fail-closed scan of the two Metal shaders for decimal/exponent/inf/nan tokens; the MSL type-family rejection is a separate grep in `check.sh`. |
| `check_toml_no_float.awk` | Conservative no-float scan of every manifest, lockfile and the Lean manifests, continuation lines included; multiline or unterminated strings fail closed. |
| `render_m2_failure.py` | Renders one typed freeze-56 outer-gate failure without a Rust binary — the bootstrap path for failures before the checked runner is built; "no success encoding, never overwrites an existing artifact." |

---

## 8. Open items and what was not verified in this pass

- **Is the workspace gate green at `c00717d1`?** Unknown from any record (§4.6); not run here.
- **The survey of 2026-09-07 counted 5 `#[ignore]`d tests** (`solver_sigma1_repair` 3, `solver_calibrate` 1, `solver_wakeup` 1); on the tree, two of those five occurrences are doc-comment mentions of the word (`solver_sigma1_repair.rs:30, 325`) and one is a code comment (`solver_wakeup.rs:559`). The attributes that actually ignore a test are two: `solver_sigma1_repair.rs:329` (`capture_before_side_fixture`, "regenerates the committed before-side evidence") and `solver_calibrate.rs:419` (`v5_literal_count_timing_position_reconstructs`, "blocked: plunge-side game seeds (L2-A6 [[gran-anchor-reconstruction]])"; its blocker is discharged for G1 but the test is still ignored on main — [walt-gran-anchors](walt-gran-anchors.md) §10). Any page quoting "5 ignored" is quoting the loose count.
- **The L2-thread gate and probe epochs differ** (§5): the probe bins declare σ0 `n0 = 8` / σ1 `4×2`, the gate file `tests/solver_fieldswap_screen.rs` runs σ0 `n0 = 2` / σ1 `2×2`. Both are declared in their own headers, so nothing composes wrongly by accident — but a page that says "the fieldswap gates pin the n0 = 8 epoch" would be wrong. Checked 2026-09-13: every other L2 gate file (`fieldswap_cancel`, `fieldswap_motifs`, `e3_upper`, `hazard_witness`, `targeted`, `wakeup`, `waking`) also runs the small pair, each declaring it in its header (§5).
- **Whether the committed wasm packages match their sources** (§6.1 item 14) is not verified; it belongs to [[plunge-walt-sync]] and [walt-instruments](walt-instruments.md).
- **The four crate-birth dates** of §1.1 come from `git log --diff-filter=A` on the old crate manifests; `walt-strat`'s S3 date is LOG.md's (2026-08-09) rather than a manifest commit.
- **Is the trick-3 mixture price reachable at all?** MB1 refused every h8-t3 action at the 7,000,000-read ceiling (§2.4) while the single-field recursion solved the same root exactly, and FH2's suffix memo cut the h3-t4 `k = 2` pass from 2,829,306 to 421,175 reads (`walt/probes/factor_belief/focal_ladder_run1.txt`). Whether that reuse would carry the model-space recursion (`model_recursion`) past the trick-3 wall, or whether trick 3 is its standing wall, no record or brief answers (survey of 2026-09-07); the consolidation slice of §6.1 item 6 is where it would be measured.
