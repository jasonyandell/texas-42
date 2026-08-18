# walt — the instrument inventory

[Home](Home.md) · owns: the walt instrument inventory — what the `walt/` program has
BUILT that survives and can be reused, independent of whether the experiment that
motivated it succeeded · Sources: [`walt/PLAN.md`](../walt/PLAN.md),
[`walt/LOG.md`](../walt/LOG.md), [`walt/ci/check.sh`](../walt/ci/check.sh),
[`walt/ci/check_m2_metal.sh`](../walt/ci/check_m2_metal.sh), the crate sources
under `walt/walt-*/`, the results artifacts under `walt/walt-factory/results/`,
the canonical GPU-track comparands under `walt/receipts/`, and the rescued probe
suites under `walt/probes/`.
Related: [walt hub](walt.md), [walt-foundation-era](walt-foundation-era.md),
[walt-factory-era](walt-factory-era.md), [walt-census-era](walt-census-era.md),
[walt-s6-era](walt-s6-era.md), [walt-decision-sparse](walt-decision-sparse.md),
[walt-scheme-fix](walt-scheme-fix.md), [walt-math-reference](walt-math-reference.md).

> **Epistemic tier: EXPLORATORY — below every tier on
> [Home](Home.md#evidentiary-tiers--never-promoted-never-blurred).** Every number
> on this page is walt-tier computed evidence at a declared configuration on a
> declared finite domain, never a corpus status, never a kernel proof, never an
> exchange adjudication, never a rob receipt. Timings in particular are machine
> facts about one run, not properties of the mathematics. Nothing here may be
> cited by anything above the exploratory tier (TRUST-01).

Vocabulary note, because two different things share a word. "Certificate" in this
page refers **only** to walt's own §16.11 lesson record type (`certificate.rs`,
`docs/certificate-schema.md`) — that type keeps its own name. It is never the D3
concept, which is the **necessary outer profile**, and no walt object is an
identity-bearing witness of reachability.

## Why this page exists

Several walt probes returned negative results and left working machinery behind on
purpose. The fiber-crush probe found the class DAG slower than a plain cache at
first build and concluded that "the class store is a storage/transport object,
never a first-build accelerator" (S5h). The fiber-refinement probe found its
declared exclusions biting near-zero worlds and recorded that "the predicate ENGINE
is proven either way" (S5i). The endgame store lost to the plain cache on speed and
still produced the first direct size data for the seat-level census (S5j).
`PLAN.md`'s deferral note says it plainly: "Nothing is discarded: the memoized H
solver + tree cross-validation are retained as the seat-label ground-truth
instrument." This page is the catalog — read it before building anything under
`walt/`.

## The workspace

Ten crates, Rust 2021, in one Cargo workspace at `walt/Cargo.toml`. The original
six-crate import direction remains strict and matches v0.4 §16.2: **core to
kernel to geom to strat to skeleton to factory** — core imports nothing; kernel
and geom import core only; strat imports core, kernel, geom; skeleton imports all
four; factory imports all five. The four-crate GPU side branch is
`walt-gpu-spec` to `walt-gpu-ref` to `walt-metal` to `walt-m2-runner`, with the
exact additional parent edges listed below. `overflow-checks = true` in dev,
release and test profiles: a silent wrap would be a wrong count, so it is a loud
panic in every profile instead. The original six use only `num-bigint`,
`num-integer`, `num-rational` and `num-traits`; the GPU side reuses that exact
arithmetic and adds the lockfile-pinned `objc2`, `objc2-core-graphics`,
`objc2-foundation`, `dispatch2` and `objc2-metal` closure.

| Crate | What it provides | Notable types and entry points |
| --- | --- | --- |
| `walt-core` | The Straight 42 rules layer (v0.4 §1): pips, the 28 dominoes, seats and teams, the nine declarations, effective contexts, the rule algebra (incidence, follow, tier, rank, trick key, BEATS/THREAT), count and trick scoring, legality, receipt parsing and history replay. Imports nothing. Every derived view is a function of semantic state. | `pip`, `domino`, `seat`, `decl`, `context`, `set` (bitsets over 28 tiles / 8 contexts), `rules`, `receipt` (parser for rob's `verify_player.txt`), `replay` (re-derives a hand from rules alone) |
| `walt-kernel` | The viewer kernel and its current-remainder fiber Φ(C) (§2.1): known hand, hidden live pool, per-hidden-seat capacities, observable voids; exact enumeration, exact integer counting DP, exact uniform sampling. The PRNG only ever selects; it never computes a value. | `kernel::Kernel`, `fiber` (counting DP grouped by admissible-slot signature), `sample` (uniform draw weighted by exact completion counts, plus a rational fingerprint), `decision::ReceiptDecision` (kernel at an *arbitrary* transcript decision, mid-trick included) |
| `walt-geom` | Exact one-parameter policy geometry (§8–§9, finite-first per §16.1): i128-backed rationals, affine lines in the valuation parameter, continuous piecewise-linear envelopes on the ray with endpoint ownership as a type invariant, argmax correspondences, 29-dimensional capture features, finite feature sets with support functions. Polytopes are carried as generating point sets; hulls are never materialized. | `rat::Q`, `line`, `envelope::Envelope` (+ `assert_invariants`), `correspond` (argmax at and after each event point), `feature::FeatureVec` |
| `walt-strat` | The operators registry, kept deliberately distinct per §10.8. Decision nodes over fiber worlds, the canonical perfect-recall information partition, information-consistent policies keyed by opaque info-state ids (world-peeking is unconstructible by type), and the named operators with the information prices between them. | `pi::pi_root_values` (symbolic parametric PI), `scalar::ScalarPi` (+ `ScalarValuation`, `scalar_census`), `hidden::hidden_root_values` (symbolic H), `hidden_scalar::ScalarHidden` (scalar H, `action_values` and `action_values_dag`), `revealed::revealed_summary` (C and F), `price::information_prices`, `census::pi_census`, `info::{InfoPartition, Policy, policy_value_receipt}`, `label::{OperatorLabel, WeightingLabel}` |
| `walt-skeleton` | The declared deliverable layer: the `ControlSkeleton` trait (typed relational state with closed update `step(d, obs)`), the §12.1 static soundness checker, the exhaustive §12.6 controlled lumpability checker, descriptor vocabularies, the §12.9 synthesis search, and the §12.6A equivariant census machinery with its class DAG and railyard routines. | `skeleton::{ControlSkeleton, UpdateKind, StaticWrap, fold_record}`, `soundness::check_soundness` (+ `PurityCounterexample`), `lumpability::check_lumpability` (+ `LumpabilityFailure`), `atoms::{Atom, Exp3aAtom}`, `synth::{sound_search, exp3a_sound_search}`, `equivariant::{Situation, canonicalize, build_carrier, closure_carrier, check_ecl, build_r3, class_dag, check_ecl_r3, r1_refines_r3, yard_tree, yard_shape, suffix_library, trick_six_kernels}` |
| `walt-factory` | The factory layer: the regret walker, the typed conflict vocabulary, the lesson type and its generalizer, the basin domain, the lesson database with its watched-feature index and rent ledger, §16.11 record emission, and every probe binary. | `walker::{walk_seat, WalkerConfig, DecisionRecord}`, `conflict::{Conflict, Grade}`, `lesson::Lesson`, `generalize::{generalize_regret, lesson_applies, measure_rent}`, `basin::{BasinDomain, DomainSpec}`, `db::LessonDb`, `index::WatchIndex`, `ledger::{Ledger, HCheckerRegistry, HCheckerToken}`, `certificate::emit_certificate`, `label_transfer::remeasure_at_h` |
| `walt-gpu-spec` | The portable M0 exact-arithmetic and semantic-table layer. It imports `walt-core`, forbids unsafe code and denies float arithmetic. | `mass::U256Mass`, checked framed operations, SHA-256 anchors, `SemanticTablesCanonicalV2`, canonical table bytes and digests |
| `walt-gpu-ref` | The portable M1 reference projector plus the complete M2 carrier, bindings and canonical receipt codecs. It imports `walt-core`, `walt-kernel` and `walt-gpu-spec`; Rob appears only as a development-time prose-rules bridge. | `projection`, `carrier`, `m2`, `m2_receipt::{receipt, records, transport, wire}`, M0/M1 receipt generation and strict M2 validation |
| `walt-metal` | The only Metal/Objective-C boundary: fixed scalar-word ABI, checked MSL kernels, retained completion evidence and safe runtime tokens around the contract's private unsafe operations. It imports `walt-gpu-spec`, `walt-gpu-ref` and the exact pinned `objc2` feature closure. | `abi`, `bridge`, `runtime`, `error`; `shaders/00_u256.metal`, `01_opening_projector.metal`, the deterministic build script and checked-in metallib |
| `walt-m2-runner` | The supervised freeze-56 executable. It assembles the complete carrier, runs smoke and official child profiles, validates typed progress/timeout/no-partial semantics, and constructs or adjudicates the closed receipt. It imports the other three GPU-side crates. | `assembly`, `observation`, `child`, `protocol`; `descriptor-verify`, `run-smoke`, `run-official`, `validate-receipt` and `adjudicate-receipts` modes |

Together those four crates support exactly one new status sentence:
**M2 METAL PROJECTOR PARITY COMPLETE under freeze 56**. It covers arithmetic/projector
parity only and computes no action value, selected lead, optimal set, information
net, continuation, performance claim or player.

Discipline carried by types, not by promises: the seat's observation type
(`walt-skeleton::obs`) cannot express a hidden hand, so every `step` is seat-honest
by construction; policies map info-state ids, so a world-peeking policy will not
compile; the ledger's deletion path needs a token whose only constructor is a
registry with a checker in it.

## The solvers, and their honest performance characteristics

Five operators exist, and §10.8's rule is enforced socially and by type: a theorem
for one operator never silently transfers to another. PI-averaged action values are
the information-relaxed diagnostic, **not** the seat's hidden value Q^H; the gap is
the strategy-fusion gap and it is action-specific.

| Solver | File | What it computes |
| --- | --- | --- |
| Symbolic parametric PI | `walt-strat/src/pi.rs` | Worldwise perfect-information backward induction over the whole valuation ray; every root action value is a continuous PWL envelope |
| Scalar PI | `walt-strat/src/scalar.rs` | The same PI operator at one integer valuation, with a trick-boundary cache keyed on semantic state; the workhorse for whole-fiber and census work |
| Symbolic H | `walt-strat/src/hidden.rs` | The actual hidden-information fixed-field treatment at the root, exact on the whole ray (pooled maximization decomposes because the canonical partition is a tree) |
| Scalar H (`dag-v1`) | `walt-strat/src/hidden_scalar.rs` | Exact Q^H per legal action at arbitrary (including mid-trick) decision points, unit-fraction particle weights, budgeted, with pooled-state boundary memoization |
| Revealed C and F | `walt-strat/src/revealed.rs` | Continuation- and root-revelation with the field held fixed, aggregated at the support level so no polytope is materialized |

Measured facts, each exploratory tier and each attached to its source:

- **Ordinary transposition memoisation is the manyfold, and it compounds with
  depth.** Arm A1 (identity-key boundary cache) against A0 (plain tree) has wall
  medians 0.166, 0.024, 0.010 at n = 4, 5, 6 tricks remaining — roughly 6× to 100×.
  Source: `results/fiber_probe_2026-08-11.txt`, produced by
  `examples/fiber_probe.rs`.
- **The class DAG is not a first-build accelerator.** Arm B (r3-signature
  content-addressed class DAG) against A1 is ≈ 4.3–4.9 at every rung (medians 4.7 /
  4.3 / 4.9) — identical values at about five times the cost. The reason is
  structural, not an implementation accident: class identity is a function of the
  future cone, so it is computable only after full expansion. The class store is a
  storage and transport object — reuse across coordinates, hands and weightings.
  Interior collapse is nonetheless real (n=4 hand 0: 1.50M situations to 129k
  classes). Same source file.
- **Canonicalization dominates in the endgame store.** The symmetry-reduced
  tablebase arms run 1.57–2.69× slower than the plain A1 cache: about 4.6 µs per
  canonical form against about 0.1 µs per state-key probe, and under an A1 memo the
  subtree a hit saves is already collapsed. Convergence itself is real (830,399
  form hits, 38–73% form-hit rates). Source:
  `results/endgame_store_2026-08-11.txt`.
- **Closed-form last-trick resolution beats a floor table.** Floor-table lookup
  1,430 ns against the closed-form control 35 ns — a 41× negative, reported as one;
  the closed-form bottom is the one arm that beat the control end to end (0.88–0.99
  of T0). Source: `results/endgame_floor_2026-08-11.txt`.
- **The memoized H solver is value-transparent and much cheaper.** `dag-v1` did
  13–125× less work than the unmemoized `tree-v0` walk on the fiber-probe
  coordinates, and 28–122× fewer steps on the four big-fiber cross-validation
  decisions (tree side 4.2e9 to 6.5e10 steps). Byte-identical Q^H is a CI-pinned
  invariant (`walt-factory/tests/h_value_transparency.rs`, sixteen decisions,
  including the `Q^H(2-1) = 80/7` vs `Q^H(3-2) = 202/21` pins). The offline
  cross-validation receipt is `results/h_tree_crossval_2026-08-10.txt`, produced by
  the `#[ignore]`d `tests/h_dag_probe.rs::crosscheck_tree_uncapped`.
- **Cold treatment H completed at four tricks remaining.** The seat's actual
  pooled hidden-information solve completed on full 34,650-world void-free fibers
  at every eligible n=4 coordinate, in roughly 7 to 17 seconds each, inside a
  declared 200M particle-step budget. No tractability claim follows from this or
  any other measurement in the branch (SEP-A15(iii), R-A23): it is one wall-clock
  observation at declared coordinates under a declared budget. Source:
  `results/fiber_probe_h_2026-08-11.txt` (which records 9 coordinates COMPLETED
  and 4 out of scope; `walt/LOG.md`'s "8 of 13" was an error, corrected here).
- **Store-based exclusion predicates are essentially free.** A predicate pass over
  a built store costs 0.1–3.7 ms against multi-second builds, roughly 100–1000×
  cheaper than the cheapest storeless route (200–960 ms); reachability and
  confinement predicates have no storeless alternative at all. Source:
  `results/fiber_refine_2026-08-11.txt`.
- **Deadness detection is cheap, and its cost has no quotable figure.** The often
  repeated "about 25 ns per detector call" is **contended and not quotable**, and
  it is not even in the results file: `results/deadness_2026-08-12.txt` records a
  RESUMED run and prints `0 ns over 0 calls`, so the 25 ns comes from a prior
  invocation that left no artifact. Freeze 43's sequential timing rung is the
  quotable instrument and it is unrun. What the file does support is the ratio's
  direction — detector calls are orders of magnitude below the solve arms they
  displace — and nothing more precise.

## The probe binaries

All under `walt/walt-factory/`. Examples are run with
`cargo run --release -p walt-factory --example NAME [subcommand]`; `walk_corpus` is
a `src/bin/` binary.

| Binary | What it measures | Results file | Session |
| --- | --- | --- | --- |
| `bin/walk_corpus` | Full corpus regret walk, 13 hands × 4 seats, whole transcripts; resumable by `[start_hand [start_seat [max_pairs]]]`, seeds a fixed function of (base seed, hand, seat, trick) so parts concatenate | `full_walk_2026-08-10*.txt` (part 1, part 2, assembled) | S5a |
| `gen_fixtures` | Regenerates the frozen walker fixtures; never hand-edit the outputs | `tests/data/ci_corpus_pins.txt`, `tests/data/walk_h0_S1.txt` | S5a |
| `thread_independence` | One-off determinism check: designated walk under one worker thread vs full parallelism must be byte-identical | none (prints) | S5a |
| `lesson_run` | Generalizes walker conflicts into lessons and measures basins on the tricks-5–6 exhaustive domain | `lesson_basins_2026-08-10*.txt`, `tests/data/lesson_h0_S1_t5.txt` | S5b |
| `falsification_run` | The falsification test proper on the tricks-3–6 fiber-capped domain, with relaxation ladders and cut refinement | `falsification_2026-08-10*.txt` | S5c-m1 |
| `label_transfer_run` | Re-measures every lesson basin at (H, fixed-uniform-legal); mode `r3` re-measures only the capped decisions at a raised declared budget | `label_transfer_2026-08-10{,_r2,_r3}.txt` | S5c-m2 / m3 |
| `economy_run` | Lesson DB as a working set: watched index, dual-ledger rent epochs, deletion rule with the checker block, restart-with-retention, §16.11 record emission | `economy_2026-08-10.txt` + `certificates_2026-08-10/` | S5c-m3 |
| `economy_run_r2` | The same 16-lesson working set re-priced at `dag-v1` / 10^9 | `economy_2026-08-10_r2.txt` | S5c-m3c |
| `census_run` | The §12.6A situation census. Subcommands: (default) r1 finest quotient, `r2` declared coarsenings, `r3` retrograde coarsest, `t5` the trick-five climb, `prune` live sub-DAG, `yard` the railyard factoring, `yard2` the suffix library, `a1` the complete level-one alphabet | `census_2026-08-10{,_r2,_r3}.txt`, `census_t5_2026-08-10.txt`, `census_pruned_2026-08-10.txt`, `census_yard_2026-08-10.txt`, `census_yard_v2_2026-08-10.txt`, `census_a1_complete_2026-08-11.txt` | S5e–S5g |
| `fiber_probe` | Default: the three-arm cost ladder (A0 / A1 / B) at n = 4, 5, 6. Subcommands: `h` cold treatment H, `refine` declared exclusion remnants, `endgame` symmetry-reduced tablebase, `floor` the level-1 floor table | `fiber_probe_2026-08-11.txt`, `fiber_probe_h_2026-08-11.txt`, `fiber_refine_2026-08-11.txt`, `endgame_store_2026-08-11.txt`, `endgame_floor_2026-08-11.txt` | S5h–S5j |
| `predictive_rank` | Dimension census of the value closure V^val at grades 1–3 (v0.6 Gate B) | `predictive_rank_2026-08-12.txt` | S6a |
| `policy_geometry` | Policy-geometry probe (Gate E): the four never-conflated cardinalities N_pol / N_vec / N_par / N_exp, with an exact-rational simplex under Bland's rule | `policy_geometry_2026-08-12.txt` | S6b |
| `policy_inspect` | Exploratory diagnostic, cited by nothing: reads out the dominant policies at the singleton-frontier roots and interrogates them against trivial rules | none (prints) | S6b |
| `deadness_probe` | Three one-sided deadness detectors (D0, D1-sym, D1-win) at census scale against the one-deviation tie classifier; parallel and resumable with per-unit checkpoints | `deadness_2026-08-12.txt` (a `deadness_rung_2026-08-13.txt` sequential-timing path exists in the code and has not been run) | S6c |
| `separation_probe` | Experiment E: exact root-action certification by a primal witness against an action-conditioned upper witness; writes candidate library v1 | `separation_2026-08-13.txt` | S6d |

## Frozen artifacts, receipts, and what CI actually checks

**Byte-frozen fixtures**, three of them, all under `walt-factory/tests/data/` and
all asserted for exact string equality inside ordinary tests: `walk_h0_S1.txt` (the
designated full-transcript walk, hand 0 seat S1 the bidder, under
`WalkerConfig::fixture()`, frozen by `tests/walker_fixture.rs`),
`ci_corpus_pins.txt` (one summary line per hand and seat at `WalkerConfig::ci()`,
frozen by `tests/walker_corpus.rs`), and `lesson_h0_S1_t5.txt` (the designated
lesson record, frozen by `tests/lesson_pins.rs`). Regenerate through `gen_fixtures`
or `lesson_run`; never hand-edit.

**Results artifacts.** Thirty-two `.txt` files under `walt-factory/results/`, each
opening with its own tier line, its binding rulings, its declared scope and (for
the later ones) its exact regenerate command. Plus
`results/certificates_2026-08-10/` — sixteen §16.11 records, one per lesson in the
S5c-m3 working set (ten `cert_refutation_*`, five `cert_win_*`, one
`cert_checker_*`, filenames deterministic from content keys), written against the
self-contained `walt-factory/docs/certificate-schema.md` (schema-v1) so an
independent implementation can check them.

**GPU-track comparands.** Portable M0/M1 has the canonical envelope, declared
stop and summary under `walt/receipts/gpu_native_trick1_m0_m1_v1/`. The separate
`gpu_native_trick1_gate0_2026-08-16.txt` is retained unchanged: its NO-GO remains
a true observation of the old Command-Line-Tools-only environment. Freeze 56 has
one committed binary receipt and external checksum under
`walt/receipts/gpu_native_trick1_m2_v1/`. That M2 receipt is executable evidence,
not a Lean theorem and not a persisted value for a solver to consume.

**What portable `walt/ci/check.sh` enforces.** It verifies immutable M0/M1 history
at the producing commit, the received-guide checksum and the cumulative M2 source
manifest; regenerates and byte-diffs the M0/M1 comparands; runs formatting,
warning-denied clippy, source-level no-float gates and all release workspace
tests; builds both trick-1 Lean targets and audits the M2 theorem axioms; then
rechecks the source manifest. It never skips unavailable Metal work into green
and by itself issues no M2 result.

**What elevated `walt/ci/check_m2_metal.sh` adds.** Starting from an immutable
HEAD snapshot, it runs the portable conjunction, checks the host/tool descriptor,
rebuilds the metallib twice and compares both builds with the committed library,
runs canonical Rust Gate 0, the full U256 corpus, malformed/timeout/no-partial
controls and a discarded maximum smoke, then runs the complete 614-task carrier
twice from fresh process state. Both receipts must equal each other and the
immutable committed comparand byte-for-byte; the checksum, Lean build/axiom
audit and final source identity are rechecked before success. That conjunction
licenses **M2 METAL PROJECTOR PARITY COMPLETE under freeze 56** and nothing about
an action value, selected lead, optimal set, information net, continuation,
performance claim or player.

**Three accurate statements about walt's receipt discipline**, which differ from
rob's:

1. The legacy `walt-factory/results/*.txt` probe artifacts are **not** diffed by
   CI. Their byte-equality coverage remains the three ordinary tests over
   `tests/data/` fixtures; none becomes a claim-tier result merely by existing.
2. Portable M0/M1 now does have a byte-diffed receipt stage: `ci/check.sh`
   regenerates the complete canonical directory in fresh state and compares it
   recursively with the committed comparands.
3. M2 has a stricter native stage: two fresh complete receipts must match each
   other, their external checksum and the immutable HEAD comparand, with typed
   failure output and zero partial acceptance. Receipt-shaped walt artifacts
   remain exploratory and say so in their own headers —
   `report.rs` and `lesson_report.rs` exist to make rendering byte-stable, and
   every results file opens with an explicit "exploratory tier" line. A green walt
   run is evidence at a declared configuration; the M2 receipt is likewise
   executable evidence rather than a theorem.

**Gitignored caches** (`walt/.gitignore` covers `target/` and
`walt-factory/store/`): `store/endgame_l2.store` (the level-2 endgame form store),
`store/deadness_ckpt` (per-unit run checkpoints with a freeze digest), and
`store/candidate_library.txt` (candidate library v1, freeze 36 — observation-record
keys, no values, no verdicts, identity transport only, cache never authority).
Every headline number has a cold-regenerate path that starts by deleting its store.

## The rescued Python probe suites

`walt/probes/` holds two suites preserved verbatim from the 2026-08-09 scratchpad,
before `/tmp` cleanup could destroy the only copies. Their framing is the load-
bearing part: they are **frozen validators, never source**. walt reimplements from
the definitions in the frozen mathematical basis and pins its own results against
the probe records; a disagreement is a discrepancy to be recorded, never a reason
to copy probe code into the implementation.

| Suite | Contents | Role |
| --- | --- | --- |
| `probes/exp3a/` | `lambda_probe{,_v2,_v3}.py`, `v3_diag.py`, four `*_output_postfix.txt` runs, `lambda-probe-report.md`. `lambda_probe_v3.py` Part 1 is Experiment 3A: the 22-observable atom registry whose semantics live only in that file | Supplied the vocabulary S4 had recorded as lost; ported into `walt-skeleton::atoms::Exp3aAtom`, and the 90 → 33 → 8 reproduction is now a live test (`walt-skeleton/tests/harness.rs`) |
| `probes/exp5/` | `exp5_core.py` (bitmask PI minimax, exact counting/sampling DP), `exp5_rules.py`, `exp5_census.py`, `exp5_validate.py`, `exp5_report.py`, `exp5_pwl.py`, `exp5_exact.py`, `exp5_records.jsonl` (566 records), `exp5_results.md` | The designated second implementation for cross-checking; its census vectors (h1t3 = 10, h3t3 = 5,345) and 52 kernel fiber sizes are regression pins in `walt-strat/tests/exp5_census.rs` and `walt-kernel/tests/known_fibers.rs` |

Both are stdlib-only Python 3.12 with exact `Fraction`/integer arithmetic. Running
them creates `__pycache__` — clean it up (D15).

## What is mechanically blocked, and why that is safe

The lesson economy's deletion rule fired on three lessons in the re-priced run
(the empty-basin refutation and both h1 S2 t4 lessons, all measured-zero at the
seat-facing label). All three deletions are **TRIGGERED and each mechanically
BLOCKED**. The block is enforced by type in `walt-factory/src/ledger.rs`: executing
an H-priced deletion requires an `HCheckerToken`, whose only constructor is
`HCheckerRegistry::token`, which returns one exactly when an independent H checker
is registered. An empty registry can only produce `DeletionBlocked` records. The
uncapped tree cross-validation receipt is deliberately **not** a registered checker
— it is context only, and every at-collection stamp stays SINGLE-IMPLEMENTATION.

The intended independent checker was "m4", a Python H checker. It is **retired** by
the NO-RESCUE policy (`PLAN.md`): if independent mechanical verification of H is
ever genuinely needed, the path is Lean, not Python. Until then the triggered
deletions stay blocked — which is safe by design, since deletion is an economy
action over working-set membership only. The archive is append-only: certificates,
traces and origins never leave it, readmission is cheap, and no evidence is lost by
the block. The machinery itself is intact and exercised in CI
(`tests/economy_pins.rs`).

Also standing, intact rather than removed: the lesson DB, the watched-feature index
under its candidate-completeness contract (exhaustively cross-checked in CI,
179 × 16 = 2,864 pairs), the dual H-primary rent ledger with "unmeasured is never
zero", and §16.11 record emission with per-record checker-coverage annotations and
H rows honestly marked UNCHECKED-EXTERNALLY.

## How to run things

```
/bin/bash -p walt/ci/check.sh                             # portable gate
/bin/bash -p walt/ci/check_m2_metal.sh                    # native Metal gate
cargo run --release -p walt-factory --example NAME [sub]  # any probe
cargo run --release -p walt-factory --bin walk_corpus [start_hand [seat [max]]]
cargo test --release -p walt-factory --test h_dag_probe -- --ignored crosscheck_tree_uncapped
```

Run the two scripts above from the repository root. Do not run `ci/check.sh`
casually: it builds the workspace in release, runs the full test suite and builds
Lean. `check_m2_metal.sh` additionally requires the exact native Metal toolchain
and a real device; it is intentionally not portable. Several probes are hours of
compute; `h_dag_probe` is `#[ignore]`d precisely because it is a declared manual
run.

**Declared knobs a future session must state, not inherit silently.** These are
constants or CLI arguments today, and every one of them is part of the declared
inputs a result is quoted under:

| Knob | Where | Current declared value |
| --- | --- | --- |
| H particle-step budget | `label_transfer_run` CLI arg 2; `ledger::H_DAG_BUDGET_PARTICLE_STEPS` | 10^8 default, 10^9 in the r3 supplement |
| H budget semantics | `label_transfer::BudgetSemantics` | `tree-v0` (unmemoized) or `dag-v1` (memoized); a cap at one and a measurement at the other is a semantics change, never the same statistic improving |
| Cold-H and authority-receipt budgets | `fiber_probe.rs::H_BUDGET`; `AUTHORITY_BUDGET` in `predictive_rank.rs`, `policy_geometry.rs`, `separation_probe.rs` | 200,000,000 particle-steps |
| Frontier cap / unpruned bound / LP pivot caps | `policy_geometry.rs` | 16,384 / 1,024 / 200,000 and 4,000,000 |
| Deadness support bound and receipt budget | `deadness_probe.rs::GT_SUPPORT_BOUND`, `RECEIPT_BUDGET` | 400 / 50 |
| Carrier and state stops | `census_run.rs::T5_CARRIER_STOP`, `A1_STATE_STOP` | 20,000,000 / 100,000,000 |
| Walker exhaustive threshold, sample draws, base seed | `WalkerConfig::{ci, fixture}`; `walk_corpus` header line | CI: exhaustive ≤ 40,000, 64 draws, seed `0x5ea7425a`; full walk: 1,000,000 / 2,000 draws |
| Domain fiber cap | `basin::DomainSpec` | tricks 3–6, fiber ≤ 40,000 in the falsification and economy domains |
| Deterministic decimation | `predictive_rank.rs`, `policy_geometry.rs::DECIMATION` | `(7919, 12)`, `(104729, 6)`, `(1299709, 3)` — deterministic strides, adopted after prefix sampling was rejected |
| Freeze digests | `deadness_probe.rs::FREEZE_DIGEST`, `fiber_probe.rs::STORE_FREEZES`, `separation_probe.rs::LIB_DIGEST` | Written into every checkpoint and store so a stale cache cannot be silently reused |

**The discipline that goes with the knobs, and that must survive any reuse:**

- **Caps are exclusion, never sampling.** An over-budget H measurement returns
  nothing and the decision is recorded as capped; it is never quietly replaced by a
  sample. `Unmeasured` is never `Measured(0)`.
- **Every declared stop is printed.** Carrier stops, budget exhaustion, out-of-
  scope coordinates and excluded decisions all appear in the results file with
  their counts, and control-bias annotations travel with capped domains (the
  fiber-cap exclusions skew low-control).
- **Where sampling does happen it is marked in the type.** The walker's above-
  threshold fibers use the kernel's exact uniform sampler at a recorded
  per-decision seed and every downstream quantity is graded `Sampled`.
- **Determinism is structural.** Reductions are exact integer sums and counts, so
  thread partition and schedule cannot move a result (checked by
  `thread_independence`); caches store exact values of projected states, so
  trimming one cannot change an output; the deadness runner's deterministic block
  is byte-identical across invocations and survived a mid-run kill at 41/45.
- **Raising a budget is lawful; coarsening a key is not.** The r3 supplement is the
  worked example — same solver, same semantics, larger declared budget, recorded as
  a budget change.

## Caveats a future session should know before reaching for something

- **Several retained instruments live only inside example binaries, not in library
  crates.** The three deadness detectors (`d0`, `d1_sym`, `d1_win`), the exclusion-
  predicate engine, the endgame form store and floor table, and the railyard level-
  step drivers are functions inside `deadness_probe.rs`, `fiber_probe.rs` and
  `census_run.rs`. The r3 retrograde class machinery and the yard/suffix-library
  routines *are* library code (`walt-skeleton::equivariant`), but the runners around
  them are not. Reusing a detector means lifting it into a crate first.
- **Legacy receipt-corpus statistics are pip-trump.** The corpus
  (`rob/receipts/verify_player.txt`) has no doubles-trump and no no-trump hand, so
  every statistic derived from that corpus validates the pip-trump path and
  nothing else. The one exception is the complete level-one alphabet run, which
  enumerates its own carrier — and is still declared pip-trump only. The
  independently generated freeze-56 M2 carrier has its own frozen scope and is
  not typed by this legacy corpus caveat.
- **`deadness_rung_2026-08-13.txt` is referenced by the code and does not exist in
  `results/`.** The sequential timing rung (freeze 43) is the quotable cost
  instrument and it has not been run; the ~25 ns/call figure is contended and
  explicitly not quotable.
- **The weighted H re-solve over a pre-built class DAG — the number the
  belief/policy-iteration platform claim rests on — is still unmeasured.** The
  existing H solvers take a uniform fiber weighting only and the K-bar integration
  is unbuilt. This is stated in the results file itself (P-A14), not just here.
