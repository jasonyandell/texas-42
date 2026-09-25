[Home](Home.md) · owns: the rob artifact — its place in the evidence hierarchy, why it exists, the invariants it carries as types, the receipt discipline, the S1–S10 ladder and the P1–P5 player track, its instruments, its cost, and where it stands · Sources: [rob/BRIEF.md](../rob/BRIEF.md), [rob/BRIEF_SLICE_02.md](../rob/BRIEF_SLICE_02.md), [rob/BRIEF_PLAYER_01.md](../rob/BRIEF_PLAYER_01.md); `rob/receipts/*.txt`; `rob/ci/check.sh`; the commit history of `rob/` (read-only, as of 2026-09-07, c00717d1). Related: [verification](verification.md) (every receipt integer), [rob-slices](rob-slices.md) (the build history), [analysis](analysis.md) (the probes), [field/](field/Home.md) (first contact), [lean](lean.md), [walt](walt.md).

# rob — the exact-truth engine

**In one sentence.** rob is the program that computes what is *exactly* true about a Texas 42 position — which hidden deals are still possible, how many, and the exact best contingent plan against a fixed model of the other seats — and proves it did so by printing numbers that are byte-compared against committed files every time the gate runs.

For the three readers of this book:

- *Newcomer.* rob never guesses. It does not estimate, sample, or tune. When it says a position has 8,400 possible hidden worlds, that is the count; when it says a plan is worth 5,970 of them, every world was visited. It is not the project's player any more — [walt](walt.md) is — but it is the ground truth walt's play is measured against.
- *Mathematician.* rob is an executable specification of the two ingest packages' finite mathematics, built as **rec's mathematics under v0.7's type discipline** ([package-provenance](package-provenance.md)), in a language different from the packages' verifiers, with the packages' prose invariants made into types. Its receipts are the fourth and lowest claim-bearing tier: evidence that a finite claim holds as stated, never a status change.
- *Engineer.* A three-crate Rust workspace at `rob/`, pinned to Rust 1.95.0, 20,264 lines of source (measured 2026-09-12: `rob-core` 5,825, `rob-player` 4,121, `rob-verify` 10,318), fifteen binaries, twenty-five test targets, twelve byte-diffed receipts, one gate script. Untouched since 2026-08-01.

## 1. Where rob sits in the evidence hierarchy

rob is the **bottom** of the four claim-bearing tiers on [Home](Home.md#evidentiary-tiers--never-promoted-never-blurred):

> 4. **rob conformance receipts** — byte-diffed Rust reproductions; `x-` prefixed lines back exchange numbers. Evidence, never a status change.

Three consequences, each deliberate:

1. **A green receipt raises confidence; it promotes nothing.** When rob reproduces 737,100 unique-winner cases, ALG-level claims stay at corpus status; the receipt is evidence that the finite claim is true as stated and that rob conforms. It is never a premise upstream.
2. **rob is outside the Lean kernel's trust boundary** (TRUST-01, [proof-assistant-plan](proof-assistant-plan.md)). No `PASS` — from an ingest verifier, an exchange program, or rob — is ever imported into [lean](lean.md) as an axiom. rob can be wrong without corrupting anything above it, which is exactly why it is allowed to be fast and large.
3. **Everything under `walt/`, `experiments/`, the ideas pages, and the field area sits below rob.** rob's probes (§9) and its field encounters (§10) are exploratory too — the receipt tier is the *receipts*, not the crate.

The house form for citing a rob number anywhere in the wiki: the value, its receipt file and row, and the words **conformance evidence, not a status change**.

## 2. Why a second implementation, in Rust

The ingest packages already carry Python verifiers that reproduce their own numbers. rob exists because a verifier that shares code, language, and authors with its specification cannot be *independent* evidence. `rob/BRIEF.md` §2 (2026-07-26) gives five reasons for a fresh implementation in Rust rather than the Python 3.12 the packages' own implementation prompts mandated:

| # | Reason | What it buys |
|---|---|---|
| 1 | **Type discipline is the point.** | v0.7's repairs — phase-indexed states, unconstructible illegal combinations, equality through projected state only, derived views outside equality — are *enforceable* in Rust (sealed enums, newtypes, manual `Eq`/`Hash`, no reflection). In Python they are conventions, and the packages' own history (rec regressing v0.7's repairs, [discrepancies](discrepancies.md) D1/D2) shows conventions do not hold. |
| 2 | **Independence of the receipts.** | All ingest verifiers are Python. Re-deriving every number in a different language makes "never copy the verifiers" structurally true. |
| 3 | **Exactness.** | `i64`/`i128` plus `num-bigint`/`num-rational` give exact integers and rationals with no float contagion; the 81-bit census total 1,830,967,207,309,611,271,596,161 and the count-ratio sampler's rational weights are first-class values. |
| 4 | **The roadmap needs speed.** | The later slices were aimed at the symbolic support DAG and OPEN-11 inside a 2⁴⁶-scale outer language; a systems language avoids a rewrite at the point of highest correctness capital. |
| 5 | **The Lean companion.** | The trust boundary places rob outside the kernel, so rob should not be written in Lean; it should emit stable receipts and machine-readable test vectors. Cross-validation is by receipt agreement, never FFI. |

**The independence conditions** (BRIEF §5 guardrails; BRIEF_SLICE_02 §5 extends them to the exchange Python):

- No ingest verifier code was read or translated; no exchange program code either. Only committed numbers, corpus-shape parameters, and witness data (for example the x:002 pool and the x:001 tables, transcribed from inbox JSON as *data*) were consumed.
- Trick resolution is cross-checked against a **separately coded prose-rule resolver** (`rob/crates/verify/src/prose_resolver.rs`, D4) written from the rules text; it never calls `trick_key`/`tier`/`rank`/`beats`/`resolve_trick`, uses the explicit five-tile count list, and shares only domino identity, the declaration enum, and seat labels with the algebra. It agrees on winner *and* trick points in all 737,100 cases (`r_alg_prose_agreement`).
- rob's hand corpus is its own deterministic generator (SplitMix64, `rob/crates/verify/src/corpus.rs`); only the corpus *shape* (972 = 9·12·9 prefixes; 108 = 9·12 hands) is an ingest-anchored assertion.

The same resolver is the reason rob is not fully dormant today: walt's `walt-gpu-ref` crate dev-depends on `rob-core` and `rob-verify` so that `tests/prose_bridge.rs` (T1-A12, GT1-A6) compares walt's rules against rob's independent prose resolver for every declaration, leader, lead, and three-subset (§11).

### Authority order

rob conforms to the layers above it and never resolves a specification conflict locally.

1. The binding brief for the work in hand — `BRIEF.md` (slice 01), `BRIEF_SLICE_02.md` (slice 02), `BRIEF_PLAYER_01.md` (the player track). These fix language, layout, invariants, receipt rows, discrepancy resolutions, and definition of done. They are read-only history: not edited to match later work, read with their own amendment notes.
2. This wiki — package provenance, merge order, discrepancy resolutions.
3. `ingest/` — the two immutable packages. Definitions, theorems, and claim IDs live there and nowhere else.
4. Exchange results, tier-labelled (x:NNN), citable for test expectations and constructions but never as corpus theorems (BRIEF_SLICE_02 §1.1).

When a spec is internally inconsistent, rob does not pick the plausible reading: it adds a failing or blocked test, reports the exact conflicting passages, and continues elsewhere (BRIEF §11). The protocol was never triggered — no `ambiguity_*` test exists anywhere in rob (grep, 2026-09-07).

## 3. The merge made executable

The seventeen resolved disagreements between the two packages live at [discrepancies](discrepancies.md). Five of them are not prose in rob; they are types, and a program that violates them does not compile.

| Discrepancy | Ruling | How rob carries it |
|---|---|---|
| **D1** Reachability evidence | proof-irrelevant, never identity-bearing | The certified play-state type stores no flag; its constructor is private to the lifecycle module; the optional replay witness is an erasable audit record outside equality (INV-3). |
| **D2** Derived support views vs stored cells | derived, never stored | Cells, fiber, reduced support, normal form, and compiled tables are *functions* of semantic state (`derive_rule_cells`, `support_reduction`, `compile_exact_support`, `remainder_fiber`); no semantic struct stores them (INV-1). |
| **D3** "certificate" vs necessary outer profile | **necessary outer profile**, never the c-word | The outer object's type has no conversion to any certified or reachable type — a `compile_fail` doctest in `support/outer.rs` — and `ci/check.sh` greps the banned identifier spellings out of the source (INV-10, INV-14). |
| **D5** Transition-sufficiency of reachable support | typed observations only | No method on a normal-form type consumes a bare domino; the game-level observation is constructible only from declaration + led context + play through the declaration algebra — a `compile_fail` doctest in `support/dynamics.rs` (INV-13). |
| **D11** Which implementation prompt | rec's algebra as the superset | `pip_sum`, `competitive_ordinal`, the pip transports with the scored/unscored split, and `unscored_mechanics_class` are all implemented (S1 rows `r_alg_scored_transport`, `r_alg_unscored_transport`, `r_alg_mechanics_classes`, `r_alg_competitive_ordinal`). |

### The invariants and how each is enforced

The census briefs carry fourteen numbered invariants and the player brief seven more. The table is the map from each to the mechanism that fails when it is violated; test names were verified by grep on 2026-09-07 and 2026-09-12. [verification](verification.md#enforcement-mechanisms) owns the census map in prose; this table adds the player series, which that page does not cover.

| Invariant | Statement | Enforcement |
|---|---|---|
| INV-1 DERIVED-NOT-STORED | cells / reduced support / NF / fiber are functions of semantic state; caches live outside equality | review rule; `inv_derived_coherence` (`verify/tests/inv_support.rs`) recomputes every view after each S3-corpus transition |
| INV-2 PROJECTED-EQUALITY | `Eq`/`Hash`/serialization through projected semantic fields only | `inv_projected_equality` (`verify/tests/inv_objective.rs`): two states differing only in witness/cache data are equal with equal hash |
| INV-3 PROOF-IRRELEVANT-REACHABILITY | reachability is a proposition; no stored flag; witnesses erasable | private constructor; `inv_no_reachability_field`; INV-2's hash test covers erasure |
| INV-4 EXACT-ARITHMETIC | no floats; ranks are ordered ADTs; probabilities `BigRational`; censuses `BigUint` | gate step 3 grep for the float type names; clippy `float_arithmetic` deny; `overflow-checks = true` in every profile |
| INV-5 COUNTS-ARE-CI | every exhaustive count is a hard equality in a test named for its receipt row, and printed in the receipt | 62 distinct `r_*`/`x_*` test functions (9 of them `x_`; grep 2026-09-12); gate step 7 byte-diffs every receipt |
| INV-6 REACHABLE-IMPLIES-FEASIBLE | an empty fiber on the certified path is a panic, never a value | `inv_reachable_implies_feasible_validation_path` and `_certified_path` (`should_panic`) in `verify/tests/inv_normal_form.rs` |
| INV-7 NO-RANK-FROM-ID | `DominoId` is identity only | newtype with no `Ord` (`compile_fail` doctest in `core/src/domino.rs`); `inv_id_not_rank` (`core/tests/inv.rs`) |
| INV-8 ONE-SOURCE-OF-TRUTH | winner, score, settlement are functions of the event stream | events returned as constructor data (Exec §10.1); `inv_event_replay` |
| INV-9 TYPE-DISTINCT-DOMAINS | deal ≠ remainder world ≠ fiber ≠ belief; feasible ≠ reachable; outer profile converts to nothing certified | `compile_fail` doctest in `support/cells.rs` (`DealWorld` → `RemainderWorld` does not exist) and in `support/outer.rs` |
| INV-10 VOCABULARY | "necessary outer profile"; `NativeHandView`; `UnscoredMechanics` ≠ `ScoredMechanics` | gate step 4 grep (`OuterCertificate`, `ReachabilityCertificate`, `OuterReachabilityCertificate`); review |
| INV-11 EDGE-BUDGET | holder edges only die; 63 per hand; ≤2 per live tile | deletion ledger in an audit type outside equality; `debug_assert!` subset check; `inv_edge_budget` (`verify/tests/r_sym.rs`): 6,804 = 108·63, zero reappearances |
| INV-12 MONOTONE-AMBIGUITY | `Ternary → Binary → Determinate` only | `debug_assert!` on every nonempty successor in `dynamics.rs`; `r_dyn_monotone` 157,809 rank checks |
| INV-13 TYPED-TRANSITION-ONLY | no untyped transition on standalone support | `compile_fail` doctest in `support/dynamics.rs` (`nf.transition(domino)` does not exist) |
| INV-14 FIVE-CHECKS-STILL-NECESSARY-ONLY | the outer validator returns membership only; the x:002 witness is the permanent regression | `compile_fail` doctest in `support/outer.rs`; `x_r_unr_002_*` tests green forever; the INV-10 grep |
| INV-P1 PLAN-NOT-TILE | the solver's public output is a `Plan`; no per-domino scalar API | `compile_fail` doctest in `player/src/plan.rs`; gate step 5 grep (`TileValue`, `DominoScore`, `fn tile_value`, `fn domino_score`) |
| INV-P2 ONE-INFOSET-ONE-ACTION | a plan is an ordered map observation → action; sibling bundles disjoint | the ordered map type; bundle partition asserted inside P3 gate 4 (`verify/src/p3.rs`). The brief names a test `inv_p2_partition`; **no test of that name exists** |
| INV-P3 EXACT-NO-SAMPLING | no randomness and no division on the solve path; ties break to lowest `DominoId` | `r_sol_deterministic` (756 double-solves byte-equal); receipts byte-diffed; the brief's "no `Rng` reachable from the solver" is review, not a CI grep |
| INV-P4 FIELD-FIXED | σ is a pure function of (own hand, public state), fixed before the solve | σ is a function, not a stateful object; `r_sig_deterministic` (108 traces byte-equal). The brief's `inv_p4_determinism` **does not exist under that name** |
| INV-P5 BUNDLE-CONSERVATION | child bundles partition every node; root equals the capacity-DP count | accumulator totals asserted in the solver; `r_sol_conservation` (756 solves, 58,609,267 nodes). The brief's `inv_p5_conservation` **does not exist under that name** |
| INV-P6 WINDOW-EXACTNESS | every fiber world visited within the window; `H` computed from the normative formula, never a call-site parameter | `window::window_depth` computes `H` internally (callers cannot pass a depth); `r_pos_schedule`; `r_mat_rolling` 2,800 window agreements |
| INV-P7 FRONTIER-LEAF-IS-LAW | the frontier leaf is banked team points — one unit variant, no parameters | `FrontierLeaf::BankedPoints` is the enum's only variant (`plan.rs`); gate step 5 grep (`LeafWeight`); `r_mat_window_ablation` is the only sanctioned response to window pathology |

Two further named invariant tests belong to the baseline player and appear on no other page's map: `inv_rollout_conservation` and `inv_sampled_world_voids` (`rob/crates/player/tests/inv_player.rs`).

The honest summary of the two series: INV-1 … INV-14 are each enforced by a named test, a lint, a `compile_fail` doctest, or a CI grep. INV-P1 and INV-P7 have CI greps; INV-P2 … INV-P6 are asserted inside the P-stage receipt code and by the receipt rows named above, but the three test names the brief promised were never created under those names. This is a documented gap, not a violation.

## 4. Workspace and modules

A three-crate Cargo workspace at [`rob/`](../rob/README.md), pinned to Rust 1.95.0 by `rob/rust-toolchain.toml` (the toolchain is installed on this machine, checked 2026-09-12; no `rob/target/` directory exists in this worktree, so no rob binary was run for this pass). `overflow-checks = true` in *all* profiles including release (INV-4). Runtime dependencies are four exact-arithmetic crates and nothing else: `num-bigint`, `num-rational`, `num-integer`, `num-traits`. `proptest` is the only dev dependency. Every crate declares `unsafe_code = "forbid"` and `float_arithmetic = "deny"`.

| Path | What it is |
|---|---|
| `rob/crates/core` | `rob-core` — the pure engine: rules, algebra, objective machine, support machinery. RNG-free, `#![forbid(unsafe_code)]`, `#![warn(missing_docs)]`; every public item's doc comment cites the claim IDs it implements. |
| `rob/crates/player` | `rob-player` — the seat-playing code: the fixed field policy σ, the exact information-set solver, rob's rolling re-solve, the demoted Monte Carlo baseline, and every line of seeded randomness in the repository. |
| `rob/crates/verify` | `rob-verify` — the receipt harness: the deterministic hand generator, the independent prose-rule resolver, and fifteen stage modules (`s1`–`s10`, `p1`–`p5`). |
| `rob/receipts/` | The twelve committed expected outputs, byte-diffed in CI (12,859 bytes in total). |
| `rob/ci/check.sh` | The gate. Nothing is done until this is green. |
| `rob/inspector/` | A self-contained `file://` HTML viewer over a generated trace. Display data only. |

### `rob-core` modules

| Layer | Modules | What they carry |
|---|---|---|
| Universe | `pip.rs`, `domino.rs`, `declaration.rs`, `seat.rs`, `rules.rs` | The pip set and its permutation group, the 28-tile universe as the looped-K₇ edge set, the nine Straight declarations, seats with fixed opposite partnerships, bid forms and ordering. |
| Algebra | `algebra/mod.rs`, `suits.rs`, `order.rs`, `trick.rs`, `transport.rs` | The declaration-indexed relational algebra: called and powered sets, effective suits, the lead context, the rank ADT with `TOP` above every integer, the lexicographic trick key, actor-preserving trick resolution, and pip transports with the scored/unscored split. |
| Objective machine | `objective/mod.rs`, `events.rs`, `auction.rs`, `contract.rs`, `deal.rs`, `play.rs` | Primitive public events and private deal observations as the single source of truth; the one-round auction; contract certification and settlement; complete deal worlds; the phase-indexed contracted-play state with certified lifecycle constructors. |
| Support | `support/mod.rs`, `cells.rs`, `count.rs`, `reduce.rs`, `normal_form.rs`, `sampler.rs`, `census.rs`, `dynamics.rs`, `symbolic.rs`, `outer.rs`, `transport_reach.rs`, `floor.rs` | Capacity cells as a derived view; the counting DP; canonical reduction; the normal-form trichotomy with its compiler, decoder and linear ternary validator; the exact count-ratio uniform sampler; the censuses; the matching-minor calculus; the symbolic trace validator; the necessary outer language; transport-aware reachability; the x:001 floor families. |

The mathematics each layer implements is owned by [declaration-algebra](declaration-algebra.md), [support-fiber](support-fiber.md), [capacity-dp](capacity-dp.md), [minimal-support-normal-form](minimal-support-normal-form.md), [support-dynamics](support-dynamics.md), and [reachability](reachability.md).

### `rob-player` modules

`sigma.rs` (the fixed deterministic field policy σ), `window.rs` (the budget formula and `WINDOW_BUDGET = 2²⁸`), `plan.rs` (the whole-hand contingent plan, `FrontierLeaf::BankedPoints`, `MATERIALIZATION_CAP = 2²⁰`), `solver.rs` (1,709 lines: the exact information-set best-response solver, its two engines, and the `#[doc(hidden)]` `gate` submodule for probes), `rob.rs` (rolling re-solve at every decision), `player.rs` (the baseline `MonteCarloPlayer` and the `UtilityLens` — `Points` or `ContractSuccess`), `policy.rs` (the common-continuation-policy trait), `rollout.rs`, `worlds.rs`, `rng.rs` (SplitMix64; rejection-sampled exact choice), `bidding.rs` (a deliberate placeholder — every synthesized contract is P(30) with the most-tiles pip trump; not a modelled policy), `match_driver.rs`, `observer.rs`, `trace.rs`, `book.rs` (the contingency book).

### The binaries

Fifteen binaries, all auto-discovered from `src/bin/` (no `[[bin]]` section; `cargo metadata`, 2026-09-12). Twelve write receipts, three do not.

| Binary | Crate | Writes |
|---|---|---|
| `verify_algebra` … `verify_floor` (ten) | verify | `receipts/verify_<stage>.txt`, one stage each (S1–S10) |
| `verify_rob` | verify | `receipts/verify_rob.txt` — the whole player track, P1–P5, in one file |
| `verify_player` | player | `receipts/verify_player.txt` — the baseline's self-play transcript |
| `trace_rob` | verify | `inspector/trace.js` + `inspector/trace.json` — the committed default inspector view (rob with the contingency book) |
| `trace_player` | player | the same two files from the baseline-only seed-42 match (overwrites the rob view) |
| `rob_bridge` | verify | nothing — a non-normative integer line protocol for seating rob in a foreign harness (§10) |

Ten of the twelve receipt binaries are a single line: they print their stage module's `receipt()`. Receipt content is a pure function of committed code — no I/O, no clock, no environment.

## 5. The receipt discipline

This is the habit the rest of the repository is asked to grow into ([Home](Home.md) calls rob "the aspirational engineering example"). Four rules:

1. **A receipt is generated, never written.** Every file under `rob/receipts/` is stdout from a binary. Hand-editing one converts a check into a wish while leaving it looking green.
2. **CI byte-diffs every receipt against a fresh run.** `diff -u` on the exact bytes; no tolerance, no normalisation.
3. **Every exhaustive count in the spec is an assertion** in a test named for its receipt row *and* a line in the receipt text, so a number can drift only by failing the suite and the diff together (INV-5).
4. **Exchange-sourced expectations are marked in the artifact.** A receipt line whose expected value originates in an exchange-adjudicated result carries an `x-` prefix, and the receipt's second line names the ledger entries it depends on (`# exchange: 001 (CONFIRMED 2026-07-27)`). The tier of every number is machine-visible without the wiki.

To regenerate a receipt after an intentional change, from `rob/`:

```sh
cargo run --quiet --release --bin verify_algebra > receipts/verify_algebra.txt
```

There is no regeneration script by design: regenerating is a deliberate per-stage act and the diff you then read is the review artifact.

### Frozen determinism values

Some numbers rob prints are not ingest numbers and never will be. They are properties of rob's own deterministic generators, frozen at first green so that later drift shows up as a failing diff:

| Frozen value | Where | Note |
|---|---|---|
| `FROZEN_WITH_VOIDS = 970` | `verify/src/s3.rs`; row `r_cell_parity` | the with-voids parity count of rob's generator. It coincides numerically with the ingest generators' 970, but only the 972-prefix corpus *shape* is an ingest assertion |
| the `verify_player` transcript | 13 hands, T0 7–6 T1; 12 worlds/decision, lens Points, player seed 7, match seed 42, first shaker S0 | the baseline's self-play, frozen 2026-07-27 |
| the t=1 window histogram | `r_pos_schedule`: H 1/2/3 = 85/23/0 | a fact of the budget arithmetic on rob's corpus |
| the per-depth root value sums | `r_sol_deterministic`: 160,910,645,319 / 14,433,377,305 / 779,629,916 / 21,941,700 / 1,112,428 / 46,340 / 3,716 | the solver's determinism freeze |
| the paired-match margins | `r_mat_paired`: 238 / 480, net +718; `r_mat_window_ablation`: 768 at B/2, 778 at 2B | **a measurement, never a target** (§7) |
| the contingency-book total | `r_book_roundtrip`: 6,001,465,196 canonical bytes | |
| `r_pos_census` | 44,722,908,161 | the sum of the 756 capacity-DP fiber counts (§11) |

**Never cite a frozen value as an ingest number** — [QUICKSTART](../QUICKSTART.md) records the trap. And a frozen margin is frozen so that changes are *visible*, not so that it should go up: optimising against a frozen measurement is how a determinism freeze turns into a benchmark.

## 6. The ladder S1–S10

rob was built in gated stages: a stage begins only when the previous stage's receipts are green, and each stage's receipt rows were specified in the brief *before* the code existed ([rob-slices](rob-slices.md) has the assignment-by-assignment history). All ten stage receipts are dated 2026-07-27; the exact integers are owned by [verification](verification.md) and the table below gives the headline of each so a reader can locate a number.

Tier for every row: **rob conformance receipt (tier 4)**. Rows marked `x-` back an exchange-adjudicated CONFIRMED result; rob is independent cross-language evidence for those numbers and never their definition.

| Stage | Receipt (bytes) | Commit | Mathematics (owner page) | Headline rows |
|---|---|---|---|---|
| **S1** declaration algebra | `verify_algebra.txt` (729) | 91b7c991 | ALG-01..24; rec PLAY-12/13 ([declaration-algebra](declaration-algebra.md)) | 28 dominoes, count 35 = 10+10+5+5+5; 7 leadable contexts, lead fibers 1..7; **737,100** unique-winner cases and **737,100** prose-resolver agreements; scoring 35/7/42; 56,448 BEATS equivalences; 5,040 pip permutations of which exactly 2 preserve count (identity, 2↔3 — an order isomorphism only between layers 2 and 3); 49 unscored transports / 307,328 comparisons; 3 unscored mechanics classes; competitive ordinal max 13 |
| **S2** objective hand machine | `verify_objective.txt` (488) | b18d6359 | Math §6–7; R-AUC-12; R-SETTLE-02A ([rules-profile](rules-profile.md)) | **472,518,347,558,400** ordered deals; **399,072,960** hidden assignments; auction census (2380, 3060, 3196, 3213, 3214, 3214, 3214), maxima (1, 2, 3, 4, 5, 5, 5), caps 5..7 identical; certified constructors enforce Exec 10.1; 200 hands legal play; 201 hands conserve 42 with P_D = 42 iff seven-trick sweep |
| **S3** cells as a derived view | `verify_support.txt` (376) | 651824fb | CELL-05/07, TRANS-01..07, STR-06 ([support-fiber](support-fiber.md)) | 108 deals, U = 21, 63 holder edges; typed updates 14,412 leads / 56,460 follows / 56,460 sloughs; **972** parity prefixes (970 with public voids — rob-frozen); 864 transitions = 648 hidden nonincrease + 216 viewer equality; the **90-world** support witness |
| **S4** normal form + capacity DP | `verify_normal_form.txt` (949) | 6639c833 | CELL-09..27, REACH-04/12 ([minimal-support-normal-form](minimal-support-normal-form.md), [capacity-dp](capacity-dp.md)) | tiny corpus 66,968 / 14,578 feasible / 22,620 worlds; DP 512 profiles / 1,533 / 1,344 / 48, max count 399,072,960; 785,736 marginal edges; 22,620 SCC compilations, 2,151 essential exclusions; ternary census 136,514 / 1,667,666 / 23,842 orbits / 296,721 / 279,048; 81-bit census total **1,830,967,207,309,611,271,596,161** (computed from the Math §7.12.5 formulas in `BigUint`, never hard-coded); 50 capacity profiles; floor **44,352,165** (26 bits); **0** supplemental bits |
| **S5** matching-minor calculus | `verify_dynamics.txt` (328) | 05d1df86 | TRANS-08..13, CELL-10E/F ([support-dynamics](support-dynamics.md)) | 66,969 systems / 14,579 feasible / 1,331 distinct NFs; **170,058** observations, 157,809 nonempty successors; **1,406,592** edge checks; 864/648/216; 972 native-sampler agreements. The TRANS-13 counts were reproduced without invoking BRIEF_SLICE_02 §11.1's frozen-count escape hatch |
| **S6** symbolic trace validator | `verify_symbolic.txt` (175) | 513a2e01 | REACH-14/15, TRANS-12/14 ([reachability](reachability.md)) | 108 hands, 3,024 transitions, 3,024 three-way agreements; **6,804** = 108·63 deletions, no reappearance; 324 = 3·108 mutated traces rejected with typed reasons. `validate_symbolic_trace` is the only door by which a foreign support claim enters; on acceptance the witness is erased (D1) |
| **S7** necessary outer language | `verify_outer.txt` (680) | dd40bfa6 | REACH-06/06A/07/11/11A ([reachability](reachability.md)) | schedule censuses A_j / T_j1 / T_j2 (three 8-tuples ending 2,097,152); 176 lead-witness entries by two routes; **7,124,838,074,989** per declaration, **64,123,542,674,901** total, 839,220,930,919 max block, ceilings 46/43/43/40 bits; five-check validator returns necessary-only membership; interval 26..46 bits. `x-r_out_burnside`: 136,514 / 2,156 / 35 → 23,842 (backs **x:005**) |
| **S8** unreachability regressions | `verify_unreachable.txt` (465) | e5a69d5c | REACH-10; x:002 ([reachability](reachability.md)) | `r_unr_reach10`: 450 generators, 2 matches, lead-fiber sizes (7, 1), all lead tiles hidden. `x-r_unr_002_*` (backs **x:002**): 4/4 classic outer checks pass and the fifth fails; 3 static matches, 1 lead-witness kill; **425,520** shallow candidates, **0** realizers; follower supply 1 < 2. This is the regression that keeps *feasible ≠ reachable* honest |
| **S9** transport-aware census | `verify_transport.txt` (287) | 4ce6e16c | rec ALG-22/23; x:004 | `r_tra_class_quotient`: 3 classes on 9 declarations (all seven pip trumps in one); `x-r_tra_corpus_commutation` (backs **x:004**): 588 = 49·12 transported hands, 16,464 accepted transitions, 17,052 NF equalities f(N_t(prefix)) = N_u(f(prefix)); no scored-transport API (`compile_fail`) |
| **S10** (stretch) reachable floor | `verify_floor.txt` (361) | fa027ab7 | x:001 | all rows `x-` (backs **x:001**): 3,808 module cases; 369 one-context profiles (a principled superset of the 216 the exchange program tabled); families 559,316,142 / 8,387,350,664 / 8,721,399,239; total **17,668,066,045** > 2³⁴, floor 35 bits, interval [35,46] |

Three notes on the ladder's edges:

- **839,220,930,919** (the S7 max single-profile block) is printed by rob and reproduced by exchange 005, but appears in neither ingest `VERIFICATION_OUTPUT.txt` (surveyor grep, 2026-09-07); the brief sources it to Math §7.13.6 / REACH-11A. Do not call it a verifier-output line.
- **559,316,142** in `x-r_flo_families` is x:001's grammar-subfamily count, later superseded by x:008's exact no-void census 624,892,870 (D17). rob's line prints the x:001 figure by design — it reproduces x:001, not x:008 — so a reader diffing it against x:008 has found a scope difference, not an error.
- rob's interval [35,46] is x:001's. The wiki-level interval tightened to **[36,45]** at the exchange tier the same day (x:006 floor 36,913,384,410, 2026-07-27T14:53Z; x:007 ceiling 33,297,009,347,414 < 2⁴⁵, 2026-07-27T18:15Z); rob reproduces neither ([reachability](reachability.md)).

### What rob does *not* reproduce

Stated plainly because [Home](Home.md) and [QUICKSTART](../QUICKSTART.md) have both said "every ingest number", which overclaims. rob reproduces every slice-01 and slice-02 target (BRIEF §8, BRIEF_SLICE_02 §9). It does not reproduce:

1. **The reduced-kernel (slice 03) numbers** — BRIEF §8 lists them as explicitly *not* slice-01 targets: folded-trick receipts (2,211,300; 84; 3,132), dihedral frames (8), future equivalence (5,898 / 17,560), and the x:003 collapse witness that needs the fold to exist before it can be stated ([reduced-viewer-kernel](reduced-viewer-kernel.md)). (rob's constellation k=1 census enumerates its own 2,211,300 forced last tricks = 20,475·12·9 — an exploratory instrument count, §9 — not this receipt line.)
2. **The 90-world posterior flip** (exact posteriors 1/7, 4/7, 2/7 vs 1/2, 1/4, 1/4 and opposite best leads under all four utilities) — the belief layer was never built; rob has support and no belief, which keeps *support ≠ belief* a typed distinction rather than an assertion ([belief-vs-support](belief-vs-support.md)).
3. **The post-x:001 interval work** — x:006, x:007, x:008 and the [36,45] interval, above.

## 7. The player track P1–P5

`BRIEF_PLAYER_01.md` was written on 2026-07-28 (48107b6f) as an endgame plan solver and amended the same day (0c7f03fb) to whole-hand play from trick 1 under a budgeted exact window. Four stages went green that day (P1 ae6af953, P2 392bdee8, P3 4140d203, P4+P5 6d3409f1); all five print into the single receipt `verify_rob.txt`, the twelfth. Tier: rob conformance receipt; the self-play, root-value, and match figures are rob-internal determinism freezes.

**The naming law.** The player specified by this brief **is rob**. The earlier fixed-field Monte Carlo player was demoted to *baseline*, kept as the paired-match opponent and as the owner of `verify_player.txt`, and given no name. The baseline plays no part in rob's runtime.

**PLAN-NOT-TILE (INV-P1).** The decision variable ranges over whole-hand contingent plans ρ — one action per viewer information set within the window — and the played tile is the plan's first move. No API anywhere returns a scalar "value of a domino"; any per-tile number shown to a human is a typed projection of a plan produced by the trace layer alone.

**Fiber = information set.** rob's knowledge object at a decision is exactly the fiber Φ(C) of the derived cell system (CELL-05/07). No sampling anywhere on the solve path (INV-P3). Large fibers are streamed by rank/unrank (CELL-25/26), never materialised.

**σ, the field policy (P1).** Every non-viewer seat — the partner included — is modelled by one fixed deterministic policy, defined through the S1 algebra and never through pip arithmetic: *leading*, the legal tile with the highest trick key under its own led context, tie lowest id; *following or sloughing*, the lowest tile whose key beats the current best, else the lowest key, tie lowest id. Points-blind, history-blind beyond the current trick. σ's determinism is the compression that makes the solve exact and small; σ's quality is explicitly not a goal (§10.1: improving σ is out of scope). `r_sig_selfplay`: 108 hands, 3,024 plays, 42×108 points conserved; `r_sig_deterministic`: 108 traces byte-equal.

**The budget and the engine rule (P2).** The window depth `H` at a decision is the largest `h` ≤ tricks remaining with `fiber_count × Π_{i<h} max(1, hand − i) ≤ B`, floored at 1 (`window.rs`; INV-P6 — callers cannot pass a depth). `B` was amended from 2³² to **2²⁸ = 268,435,456** on 2026-07-28: the original priced a work unit at ~1 ns, a real streamed world-segment costs ~50–100 ns, so 2³²-scale windows were hours. The amendment traded no exactness, only the arithmetic route: **window-1 solves take the response-class counting engine** (exact values by capacity-DP counts over σ-response classes — "the seat holds r and avoids E" — milliseconds at any fiber size, including the full 399,072,960-world trick-one fiber); **windows ≥ 2 take the streaming engine**. The receipt's schedule: H = 1 at t = 0; H ∈ {1, 2, 3} at t = 1 with histogram 85/23/0; full depth to the end of the hand from t ≥ 2. The 756-position corpus (7 boundaries × 108 hands, viewer to act) has closed-form fiber bounds 399,072,960 / 17,153,136 / 756,756 / 34,650 / 1,680 / 90 / 6, all 756 counts within bounds with 108 boundary-0 equalities; 432 enumerated fiber agreements and 324 streamed rank/unrank round-trips.

**The four correctness gates (P3, `verify/src/p3.rs`).**

| Gate | Row | What it checks |
|---|---|---|
| 1 known-world degeneracy | `r_sol_known_world`: 756 agreements | with the fiber pinned to one world the solver equals an independent perfect-information best-response recursion against σ — same value, same canonical action |
| 2 tiny-depth exactness | `r_sol_brute_force`: 216 positions, 324 pure plans enumerated | at ≤ 2 tricks remaining every pure plan is enumerated literally and the solver's value equals the maximum |
| 3 no dominated plan | `r_sol_undominated`: 216 positions | the solver's plan is not pointwise-dominated by any enumerated plan |
| 4 bundle conservation (INV-P5) | `r_sol_conservation`: 756 solves, 58,609,267 nodes, 62 truncated | child bundles partition every node; the root equals the capacity-DP count; 62 plans exceeded `MATERIALIZATION_CAP = 2²⁰` and were truncated for materialisation only |
| plus determinism and engines | `r_sol_deterministic`: 756 double-solves byte-equal; `r_sol_engines`: 540 agreements | the counting and streaming engines produce identical plans (values, actions, bundles) at window 1 on every enumerable position |

**rob at the table (P4).** Rolling re-solve at every one of rob's decisions from trick 1 — `r_mat_rolling`: 2,800 decisions (200 hands × 14), 2,800 window agreements with the §7 formula, and at every decision the true dealt world passes `fiber_contains` (CELL-05 losslessness, live). The mirrored paired match — 100 deterministic deals × 2 seatings, rob's team vs the baseline (12 worlds, Points, seed 7), Points lens — froze `r_mat_paired`: **seating margins 238 / 480, net +718 points**. The window ablation re-plays the same match at B/2 and 2B: net **768** and **778**. Per BRIEF_PLAYER_01 §8/§10.3, **the frozen margin is a measurement, never a target**: if rob had lost, that would have been a reportable finding, never a reason to tune σ, the leaf, the corpus, or the seeds. The ablation's flatness is what prices the window: halving or doubling the budget moves the margin by 50 or 60 points on 200 hands.

**The contingency book (P5).** Deterministic plan-tree JSON emission with a strict parser and capped projection: `r_book_roundtrip`, 756 plans byte-equal, **6,001,465,196** canonical bytes; `r_book_trace`, 4 hands, 112 decisions, 56 plans embedded (3 levels / 24 branches per projection) — the inspector's default view.

**The rents, registered in the brief (§1) and carried honestly.** (a) *Field-model rent*: rob's opponents in the paired match are the baseline, not σ — so the match measures a best response to a wrong model against a real opponent, which is the honest experiment. (b) *Partner rent*: rob models his partner as field even though the partner seat is also rob; seats never communicate. (c) *Window rent*: tricks 1–2 are played myopically by budget — H = 1 at trick 1 — with the banked-points leaf (INV-P7). This is **an exact one-trick window with a banked-points leaf**, not a heuristic evaluator: every fiber world is visited, σ is applied exactly, and only the horizon is short. "Window-greedy" early play is a characterised, measured behaviour (the ablation row), never a bug and never a license to enrich the leaf.

**P6 (stretch)** — the W1 σ-consistency history filter, tests `r_w1_*` — was never built; no such rows exist. Nothing since has scheduled it (§11).

## 8. The baseline and the one law

The baseline (evening player v0, `MonteCarloPlayer`, committed b22d4b3d on 2026-07-27, demoted 2026-07-28) is the champion's law from [lineage](lineage.md) rebuilt on exact fiber sampling: Math §11.4's anti-strategy-fusion law — **one common continuation policy, fixed before any world is drawn; average, then max** — made *structural* rather than conventional. A single policy object is cloned with the identical private tape to replay every seat in every simulated world, and no per-world action hook exists anywhere in the player API. Worlds are drawn uniformly from the exact fiber by the count-ratio sampler with rejection (no modulo bias), so zero impossible worlds are ever evaluated (CELL-05). Its receipt is the 13-hand transcript (`verify_player.txt`, 6,650 bytes; T0 7–6 T1) plus a non-normative statistics block; its two named invariant tests are `inv_rollout_conservation` and `inv_sampled_world_voids`.

It was kept, untouched (BRIEF_PLAYER_01 §10.6), as the paired-match opponent because a best response needs a real opponent to be measured against, and a second exact solver would have measured only σ.

## 9. Instruments and the promotion path

[analysis](analysis.md) owns the instruments in full; this is the map. Everything in this section is **exploratory — below every tier** — and the road from an instrument's number to a receipt row has exactly one gate: **a brief amendment** that names the invariant, fixes the seed and corpus, and adds the row to a verifier's receipt. No probe number has ever taken that road.

**The inspector** (`rob/inspector/index.html`, fed by `trace_rob`) steps every play of every hand from any seat's masked view or omniscient truth, with exact fiber counts and holder marginals; at each rob decision it shows the window, the plan tree (action / observation keys / bundle sizes / exact values per node), and the **openings table** — the exact best-plan value for every legal opening, chosen and rejected. Its honesty rule: the JavaScript recomputes no game logic; per-seat masking is emitted by the Rust tracer from that seat's own state, so a viewer bug cannot manufacture a fact or leak a hidden tile.

**`gate::solve_opening(state, lens, action)`** (`solver.rs`, `#[doc(hidden)]`) returns the best plan restricted to one fixed opening — what makes "why not that tile?" exactly answerable. Play never routes through the gate module (INV-P6).

| Probe file (`rob/crates/verify/tests/`) | Commit, date | Frozen exploratory finding | In the normal suite? |
|---|---|---|---|
| `ablation_probe.rs` | cf75e71f, 2026-07-28 | myopic rob (window 1 everywhere, ≈27 ms/hand) vs baseline over the same mirrored 200 hands: **net −288**; full rob vs myopic rob: **net +876** — essentially the entire margin is depth | yes (both tests run; one plays 200 hands with rolling full re-solve) |
| `nickel_probe.rs` | 4a7536ec, 2026-07-28 | P4-stream deal 1, trick 4: rob sloughs 5-0 over the "obvious" 3-0 because partner wins the trick in **5,970 of 8,400** fiber worlds under σ; self-validating against `solve_opening` | yes |
| `sigma_counterfactual_probe.rs` | ee98cd6a, 2026-07-28 | the same contest under a max-trump shut-out responder: partner wins exactly **4,200 of 8,400** — the opponent model in the denominator | yes |
| `hierarchical_fiber_probe.rs` | 8ea14e8a, 2026-07-29 | rung 1 of [idea-hierarchical-fibers](idea-hierarchical-fibers.md): `gate::counting_deep` reproduces the certified engines plan-for-plan on 756 + 324 + 216 + 6 positions; exact H = 2 at the 399,072,960-world trick-one fiber in 7–10 s (≈350× intensional compression, 72–74 % zero-pruned); class tree grows ≈55× per depth step | 6 of 8 tests run (the cross-checks); the trick-one H = 2 / H = 3 timings are `#[ignore]`d |
| `strategic_exchange_probe.rs` | 8ea14e8a, 2026-07-29 | rung 2 **falsified as stated**: 160,012 tile swaps, 28 % change the plan-vs-σ outcome | no (`#[ignore]`) |
| `fiber_factor_probe.rs` | 2026-07-29 | where-is-the-count captures 0–390 ‰ of the perfect-information gap, up to 613 ‰ composed with beaters; rung-3 bounds settle 10 of 13 root decisions, the wall position closing with rival U 165,504 < V* 174,554 | no (4 tests, all `#[ignore]`) |
| `retrograde_rank_probe.rs` | ddd2f96a, 2026-07-31 | standing-preserving substitution at boundaries 6/5/4: 32,886 checks, **zero** minimax divergences; substitutable fraction 60 % → 28 % → 16 % ([idea-retrograde-rank](idea-retrograde-rank.md)) | smoke test runs; the sweep is `#[ignore]`d |
| `constellation_k1_census.rs` | 4cf3b464, 2026-08-01 | all 2,211,300 forced last tricks → 15,680 fine / 1,753 coarse keys / 14 outcomes, zero collisions pooled across nine declarations; carriers 486 / 4,767 (later corroborated at exchange tier by x:009 referees and x:012 — the instrument stays exploratory) | smoke runs; full census `#[ignore]`d |
| `constellation_k2_probe.rs` | cd51ce2e, 2026-08-01 | 2,041,200 positions in 454,920 groups (279,732 cross-declaration), **817,896** exact minimax checks, zero divergences — the last change to rob's code | smoke runs; full probe `#[ignore]`d |

## 10. First contact, 2026-07-30 — and the sequel

On 2026-07-29 `rob_bridge` (6f7d3469) gave rob a dependency-free seat: one decision per stdin line (`seat decl bidder h0..h6 n (actor domino)*n` → `domino leader points0 points1`), the reply carrying rob's *own* derived trick leader and team points so a foreign harness can assert rules conformance at every decision. The next day rob was seated in the mk5 arena against the champion it was built to answer, E[Q] n=10 ([lineage](lineage.md)). The full record with every caveat is [field/first-contact](field/first-contact.md); the tier is **field measurement, exploratory** — computed by mk5 arena code, not by rob, and carrying zero evidentiary weight for the mathematics.

- **Encounter 1, the full hand** (dropped-30 protocol: every hand forced to a 30 bid, declaration the forced bidder's best pip trump, mirrored deals, 3 seeds × 384 games = 1,152 games): the champion won. rob 525/1152 games (45.6 %); on 6,028 mirrored deal pairs rob made 32.4 % vs 36.1 %, discordant 459 vs 679, McNemar z = −6.52, make-rate edge −3.65 pp [−4.75, −2.55]; negative in all seven declarations.
- **Encounter 2, mid-hand takeover** (256 positions per takeover trick from a shared heuristic prefix, both offence and defence): 93/256 vs 91/256 at trick 3, 84 vs 87 at trick 4, 89 vs 91 at trick 5, every |z| ≤ 0.6, pooled gap −0.4 pp ± 1.7 pp. A dead heat from rob's exact window onward.
- **What the pair pins down:** the whole full-hand deficit lives in tricks 1–2(–3), where rob plays the exact one-trick window with a banked-points leaf (the window rent of §7). Nothing the mathematics certifies was outplayed. Dead heat ≠ optimal: "exact" is exact given rob's opponent model.
- **The bridge as instrument:** ~180,000+ decisions with zero rules divergences between two independent implementations — cross-check evidence recorded only in the field prose; no log in this repository pins the figure.

**The sequel (2026-08-17, exploratory).** Nobody replaced rob's opening. Instead the project pivoted: [walt](walt.md) became the player with pmake as the ruled objective, and `walt_bridge` — speaking the same line protocol as `rob_bridge`, zero arena changes, ~15k decisions rules-clean — beat the same E[Q] n=10 champion under the same dropped-30 3×384 protocol: **walt 630/1152 games (54.7 %), McNemar z = +6.28 over 6,015 paired contracts**, every seed's CI excluding zero, losing ~4.7 points per hand and winning the marks ([walt-seat-play](walt-seat-play.md); `walt/probes/m3/arena_results_2026-08-17.txt`). An arena outcome about play at the exploratory tier, never a statement about exact values.

## 11. What rob costs and where it stands

### The gate: `rob/ci/check.sh`

Run it from anywhere; it `cd`s to `rob/`. Seven steps, each fatal (`set -euo pipefail`):

| Step | Check |
|---|---|
| 1 | `cargo fmt --check` |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings -D clippy::float_arithmetic` |
| 3 | grep: no mention of the 32- or 64-bit float type names in `crates/**/*.rs` (INV-4) |
| 4 | grep: none of the three banned c-word identifier spellings for the outer object (INV-10) |
| 5 | grep: no `TileValue` / `DominoScore` / `fn tile_value` / `fn domino_score` / `LeafWeight` (INV-P1, INV-P7) |
| 6 | `cargo test --workspace --release` — every count assertion, plus the non-ignored probes |
| 7 | for each `receipts/verify_*.txt`, run the same-named binary into a temp dir and `diff -u`; prints `receipt <stage>: byte-identical` |

It ends with `rob ci/check.sh: PASS`. The greps police *identifier spellings in code* (`crates/` only, not markdown or receipts); the prose rules they stand for are enforced by review. Step 7 keys the binary off the receipt filename, which is why `verify_player` (a different crate) is covered: binary names are unique workspace-wide.

### What it costs

**This is an hours-long job, not a quick check.** The wall-clock has never been recorded to completion: BRIEF_PLAYER_01 §9 measured P1–P3 at ≈ 8 minutes wall on 2026-07-28 (before P4 and P5 existed), and a run on 2026-08-13 had spent four CPU-hours on `verify_rob` alone and was still going. The date of the last complete green `PASS` is not recorded anywhere; receipts were last regenerated on 2026-07-28 and code changed through 2026-08-01, so whether a full green run exists after 2026-08-01 is unknown.

Where the time goes, read from the receipt rows and the stage code: **`verify_rob` dominates because it solves 756 positions exactly (58,609,267 nodes, P3), plays three mirrored 200-hand matches with rolling re-solve at every rob decision (2,800 decisions per match — the normative budget, B/2, and 2B; P4), and re-solves all 756 positions again to round-trip 6,001,465,196 bytes of plan JSON (P5).** The earlier explanation on this page and in [QUICKSTART](../QUICKSTART.md) — that `verify_rob` "re-derives a 44,722,908,161-state census" — was a misreading: `r_pos_census` is merely the sum of the 756 capacity-DP fiber counts (`count_check` in `rob/crates/verify/src/p2.rs` adds `count_occupancy_dp(...).count` over the corpus), computed cheaply. Corrected here 2026-09-12. Step 6 is not free either: `ablation_probe.rs` runs in the normal suite and plays 200 hands of full rob against myopic rob.

The failure mode this creates: the script's output is buffered behind a long child, so a fresh session sees nothing for an hour and concludes the gate hung. It has not. Before killing a quiet run, look for a `target/release/verify_rob` whose parent is `bash ./ci/check.sh`, in state `R`, burning CPU — a healthy gate. Do not confuse it with `walt/ci/check.sh`, a seconds-to-minutes gate of the same name.

### Where it stands (as of 2026-09-07, c00717d1)

| Fact | Evidence |
|---|---|
| Code unchanged since **2026-08-01** (cd51ce2e, the k=2 constellation probe) | the commit history of `rob/`; the only later commit touching `rob/` is the 2026-08-13 documentation pass (55855c87) |
| Receipts unchanged since **2026-07-28** (6d3409f1) | the commit history of `rob/receipts/` |
| Slices 03–05 and stage P6 **not begun, and not scheduled by any ruling** | no scaffolding exists; no kanban card, brief amendment, or ruling parks or cancels them. The 2026-08-17 pivot made walt the player and said nothing about rob's ladder. This is an **open call** for Jason ([rob-slices](rob-slices.md)) |
| Still compiled by walt | `walt/walt-gpu-ref/Cargo.toml` dev-depends on `rob-core` and `rob-verify` for `tests/prose_bridge.rs`; `walt/ci/check.sh` lists the three rob crates among its watched paths; a 2026-09 partnership build log shows the rob crates being checked. Dependency direction is walt → rob only |
| Still spoken to | `walt_bridge` reimplements the `rob_bridge` line protocol verbatim |
| Never played under the ruled objective | rob's `ContractSuccess` lens was never used at the table (a typed error at a truncated window); pmake became the objective on 2026-08-17. Whether a pmake-lens rob match is worth freezing is an open question |
| The promotion path has never been taken | the ablation −288 / +876 has been "the standing worked example" since 2026-07-28 with no amendment |

## 12. Running rob

From `rob/`:

```sh
cargo test --workspace --release          # the suite, including every count assertion and the non-ignored probes
cargo run --release --bin verify_rob      # the player-track receipt, printed (the expensive one)
cargo run --release --bin verify_algebra  # any single stage receipt, seconds
cargo run --release --bin trace_rob       # regenerate the inspector trace, then open inspector/index.html
cargo run --release --bin rob_bridge      # the seat: one decision per stdin line (§10), replies on stdout; mk5's arena/rob_play.py drove an 8-worker pool of these
ci/check.sh                               # the full gate — hours
cargo test --release --test hierarchical_fiber_probe -- --ignored --nocapture   # an ignored probe, on purpose
```

Every `verify_*` binary is deterministic and environment-free; a stage receipt regenerated on any machine must be byte-identical to the committed file or something is wrong.

## 13. rob, walt, and Lean

**rob is the value.** It computes what is exactly true about a position — the fiber, the counts, the exact information-set best response against σ — and its usefulness depends on having no heuristics to defend.

**walt is the seat.** The imperfect-information player since 2026-08-17, exploratory throughout, cited by nothing above the ideas tier ([walt](walt.md)). When walt needs mechanical verification the path is Lean, not a new rob receipt. Where walt reuses rob it reuses the independent prose resolver and the bridge protocol, never rob's answers.

**Lean is the kernel.** [lean](lean.md) is independent precisely because it may not read rob's answers. rob's receipts are evidence *for us* that a finite claim holds; only a kernel proof makes it a kernel theorem.

## 14. Known documentation drift

Recorded, not hidden, so a reader who notices a mismatch knows it is known.

- **"Every ingest number" is an overclaim** in [Home](Home.md)'s `rob/` row and [QUICKSTART](../QUICKSTART.md); the correct statement is §6's — every slice-01 and slice-02 target, and not the three things listed there. [first-implementation-slice](first-implementation-slice.md) was corrected 2026-09-12.
- **QUICKSTART's "evening player v0 beats baseline net +718"** has the names reversed: the evening player v0 *is* the baseline; +718 is rob (the exact plan solver) over the baseline, `r_mat_paired`.
- **The "44,722,908,161-state census" cost story** is corrected in §11 above; QUICKSTART still carries it.
- **Twelve receipts, corrected 2026-08-13.** Four pages said "eleven" until then; the uncounted one was `verify_rob.txt`, whose only prior wiki mention was on an ideas-tier page. `verification.md`'s heading "The eleventh receipt" is correct as an ordinal (it is `verify_player`), though its text still uses the pre-demotion name.
- **The player invariants** are not enforced the way the census ones are (§3): INV-P2/P4/P5's promised test names do not exist; the receipt rows carry them.
- **Brief-internal drift.** `BRIEF.md` §9 speaks of "all four receipt binaries" (there are twelve), §13 maps two support modules that were never created, and `BRIEF_PLAYER_01.md` §§1 and 10.2 still quote the pre-amendment budget 2³² that §7 and the code amended to 2²⁸ on 2026-07-28. Briefs are historical assignments and are not edited.
- **`rob/inspector/README.md`** named `trace_player` as the regeneration command from 2026-07-27 until 2026-09-12; it now documents `trace_rob`, the plan tree, and the openings table, with `trace_player` as the optional baseline view.
