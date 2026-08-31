# walt — the instrument inventory

[Home](Home.md) · owns: the walt instrument inventory — what the `walt/` program has
BUILT that survives and can be reused, independent of whether the experiment that
motivated it succeeded · Sources: [`walt/LOG.md`](../walt/LOG.md),
[`walt/UNIFICATION-CENSUS.md`](../walt/UNIFICATION-CENSUS.md),
[`walt/ARCHIVE.md`](../walt/ARCHIVE.md),
[`walt/ci/check.sh`](../walt/ci/check.sh),
[`walt/ci/check_m2_metal.sh`](../walt/ci/check_m2_metal.sh), the unified crate
sources under `walt/walt/`, the relocated result summaries under
`walt/probes/factory-results/`, the canonical GPU-track comparands under
`walt/receipts/`, and the rescued probe suites under `walt/probes/`.
Related: [walt hub](walt.md), [walt-foundation-era](walt-foundation-era.md),
[walt-factory-era](walt-factory-era.md), [walt-census-era](walt-census-era.md),
[walt-s6-era](walt-s6-era.md), [walt-decision-sparse](walt-decision-sparse.md),
[walt-seat-play](walt-seat-play.md),
[walt-scheme-fix](walt-scheme-fix.md), [walt-math-reference](walt-math-reference.md).

> **Layout note (2026-08-24).** The workspace was unified: seven crates became
> the seven modules of one `walt` crate (`rules` ← walt-core, `kernel` ←
> walt-kernel, `geom` ← walt-geom, `strat` ← walt-strat, `spec` ← walt-gpu-spec,
> `carrier` ← walt-m3-carrier, `solver` ← walt-m3-probe), and the
> `walt-factory` / `walt-skeleton` crates — every probe binary in this page's
> historical tables — were **deleted**. Their code is archive-only at producer
> commit `648f93a`; their tracked result summaries were relocated to
> `walt/probes/factory-results/`; regeneration follows the recompute queue in
> [`walt/ARCHIVE.md`](../walt/ARCHIVE.md). Crate names below are kept where they
> are historical truth, with the new home noted.

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

## The workspace (unified 2026-08-24)

Six crates, Rust 2021, in one Cargo workspace at `walt/Cargo.toml`: the unified
**`walt`** crate, **`walt-wasm`** and **`walt2-wasm`** (the browser oracles for
plunge — level 1 and level 2), and the GPU
trio **`walt-gpu-ref`** / **`walt-metal`** / **`walt-m2-runner`**. Inside the
unified crate the old strict import direction survives as the module order,
bottom-up: `rules` → `kernel` → `geom` → `strat` and `spec` → `carrier` →
`solver` (`walt/walt/src/lib.rs` states it). `overflow-checks = true` in dev,
release and test profiles: a silent wrap would be a wrong count, so it is a loud
panic in every profile instead. The core modules use only `num-bigint`,
`num-integer`, `num-rational` and `num-traits`; the GPU side reuses that exact
arithmetic and adds the lockfile-pinned `objc2`, `objc2-core-graphics`,
`objc2-foundation`, `dispatch2` and `objc2-metal` closure.

| Module / crate (former crate) | What it provides | Notable types and entry points |
| --- | --- | --- |
| `walt::rules` (`walt-core`) | The Straight 42 rules layer (v0.4 §1): pips, the 28 dominoes, seats and teams, the nine declarations, effective contexts, the rule algebra (incidence, follow, tier, rank, trick key, BEATS/THREAT), count and trick scoring, legality, receipt parsing and history replay. Imports nothing. Every derived view is a function of semantic state. | `pip`, `domino`, `seat`, `decl`, `context`, `set` (bitsets over 28 tiles / 8 contexts), `rules`, `receipt` (parser for rob's `verify_player.txt`), `replay` (re-derives a hand from rules alone) |
| `walt::kernel` (`walt-kernel`) | The viewer kernel and its current-remainder fiber Φ(C) (§2.1): known hand, hidden live pool, per-hidden-seat capacities, observable voids; exact enumeration, exact integer counting DP, exact uniform sampling. The PRNG only ever selects; it never computes a value. | `kernel::Kernel`, `fiber` (counting DP grouped by admissible-slot signature), `sample` (uniform draw weighted by exact completion counts, plus a rational fingerprint), `decision::ReceiptDecision` (kernel at an *arbitrary* transcript decision, mid-trick included) |
| `walt::geom` (`walt-geom`) | Exact one-parameter policy geometry (§8–§9, finite-first per §16.1): i128-backed rationals, affine lines in the valuation parameter, continuous piecewise-linear envelopes on the ray with endpoint ownership as a type invariant, argmax correspondences, 29-dimensional capture features, finite feature sets with support functions. Polytopes are carried as generating point sets; hulls are never materialized. | `rat::Q`, `line`, `envelope::Envelope` (+ `assert_invariants`), `correspond` (argmax at and after each event point), `feature::FeatureVec` |
| `walt::strat` (`walt-strat`) | The operators registry, kept deliberately distinct per §10.8. Decision nodes over fiber worlds, the canonical perfect-recall information partition, information-consistent policies keyed by opaque info-state ids (world-peeking is unconstructible by type), and the named operators with the information prices between them. | `pi::pi_root_values` (symbolic parametric PI), `scalar::ScalarPi` (+ `ScalarValuation`, `scalar_census`), `hidden::hidden_root_values` (symbolic H), `hidden_scalar::ScalarHidden` (scalar H, `action_values` and `action_values_dag`), `revealed::revealed_summary` (C and F), `price::information_prices`, `census::pi_census`, `info::{InfoPartition, Policy, policy_value_receipt}`, `label::{OperatorLabel, WeightingLabel}` |
| `walt::spec` (`walt-gpu-spec`) | The portable M0 exact-arithmetic and semantic-table layer: forbids unsafe code, denies float arithmetic. | `mass::U256Mass`, checked framed operations, SHA-256 anchors, `SemanticTablesCanonicalV2`, canonical table bytes and digests |
| `walt::carrier` (`walt-m3-carrier`) | The frozen hand-8 receipt carrier (freeze-57 M3 gate profile): two constructors that must agree byte-for-byte, KAT pins, support/profile machinery. The seat player's data source. | `constants`, `profile`, `replay`, `support`, `kat` |
| `walt::solver` (`walt-m3-probe`) | **The seat solver** — the sampling-stack machinery of `walt/SCENARIO-PLAYER.md`: scenario worlds, modeled level-k minds, exact best response under the pmake objective, and — since the 2026-08-24 calculated-evidence build ([walt-calculated-evidence](walt-calculated-evidence.md)) — the evidence-path modules (`evidence` exact CE-T1..T5 arithmetic, `adaptive` kernel adapter + fixed-pair evaluator + exact endpoint, `controller` m-candidate decision controller, `policy` FreezeTuple/PolicyId frozen policies, `field` FieldId field models, `exposure` coupled first-split replay plus the E0–E2 rungs and the exact split-reach route E4, `field_swap` the L2-T4 admissible screen, `calibrate` the §19 V5 cap-ladder law and §19 V6 per-fixed-pair E0 calibration, and — since 2026-08-24 — `act` the §16.4 controller as an **acting** player under the route-labelled action policy of [`walt/CONTROLLER-PLAYER.md`](../walt/CONTROLLER-PLAYER.md) [CE thread], and — since 2026-08-25 — `wakeup` the LEVEL2-PROBE detection layer (the CE §14 three-way wake-up split under both declared fields, 𝓘-only cost verdicts, exact-zero locks) [L2 thread], and `targeted` the assembled targeted field-1 controller (§8 Stages 1–5 per root: schedule-controlled rung spend with provably-useless escalation refusals, the L2-T4 screen, survivor-only σ1 work; instrument + library layer only, never a default) [L2 thread], and — since 2026-08-25 — `waking` the wake-gated thinking-teammate player composing act/wakeup/targeted (σ0 always computed; escalation only on positive decision-level evidence; never a default) [CE and L2 threads] and `bundle` the bundled world evaluator (one shared-tree walk per candidate over the whole fiber, per-world attribution exactly-once, focal purity asserted — the retained primitive of the speed campaign)) and twenty-seven bins (scenario, level1, level2, playout, playtable, webtable, walt_bridge, controller_bridge, divergence, ladder, bidcurve, tiltaudit, m3probe, shadow, fieldswap, fieldswap_screen, fieldswap_cancel, v5flip, e0cal, fieldswap_motifs, hazard_witness, wakeup, l2_controller, waking_bridge, ordering_bench, field_cache_bench, bundle_bench). | `walt/walt/src/solver/`, bins under `walt/walt/src/bin/` |
| `walt-wasm` (crate) | The browser decision oracle plunge ships: level-1 compiled to wasm with the calibrated bid default θ=11/16 and the opt-in race mode; Node smoke 28/28 byte-identical to the frozen native trace. | `pkg/walt.wasm`, `walt.ts` |
| `walt2-wasm` (crate, 2026-08-25) | The level-2 sibling: play evaluated with modeled level-1 minds (`Field::Level(1)`, `n_inner=[n0,n1]`) over the same outer sampling and seed formula as walt-wasm (CRN across levels); bid/declare are the walt-1 auction rules pinned byte-identical by a native equality test; same zero-import ABI export names, `walt2` request magic; measured trick-1 latency grid in its pkg README (defaults n=8/n1=4/n0=2). Purely additive — zero changes to the `walt` crate. Never a default; whether small-knob L2 beats big-knob L1 is an open head-to-head question for the client's strength ladder. | `pkg/walt2.wasm`, `walt2.ts` |
| `walt-gpu-ref` (crate) | The portable M1 reference projector plus the complete M2 carrier, bindings and canonical receipt codecs; rob appears only as a development-time prose-rules bridge (dev-dependency). | `projection`, `carrier`, `m2`, `m2_receipt::{receipt, records, transport, wire}`, M0/M1 receipt generation and strict M2 validation |
| `walt-metal` (crate) | The only Metal/Objective-C boundary: fixed scalar-word ABI, checked MSL kernels, retained completion evidence and safe runtime tokens around the contract's private unsafe operations. | `abi`, `bridge`, `runtime`, `error`; `shaders/00_u256.metal`, `01_opening_projector.metal`, `02_m3_wavefront.metal`, the deterministic build scripts and checked-in metallib |
| `walt-m2-runner` (crate) | The supervised freeze-56 executable. It assembles the complete carrier, runs smoke and official child profiles, validates typed progress/timeout/no-partial semantics, and constructs or adjudicates the closed receipt. | `assembly`, `observation`, `child`, `protocol`; `descriptor-verify`, `run-smoke`, `run-official`, `validate-receipt` and `adjudicate-receipts` modes |

**Deleted, archive-only** (producer commit `648f93a`; deletion commits
`ad355e9`/`fa3fe74`; recompute queue in [`walt/ARCHIVE.md`](../walt/ARCHIVE.md)):
`walt-skeleton` — the `ControlSkeleton` trait, the §12.1 soundness and §12.6
lumpability checkers, both atom vocabularies, the §12.9 synthesis search, and
the §12.6A equivariant census machinery (class DAG, railyard, suffix library);
and `walt-factory` — the regret walker, conflict/lesson/basin vocabulary, the
lesson database with its watched-feature index and rent ledger, §16.11 record
emission, and all 24 probe examples plus `walk_corpus`. Their eras closed
([factory](walt-factory-era.md), [census](walt-census-era.md),
[S6](walt-s6-era.md)); reaching for one now means checking out the producer
commit first.

The GPU side supports exactly one status sentence:
**M2 METAL PROJECTOR PARITY COMPLETE under freeze 56** — re-issued append-only
at the unified layout as freeze-56 v2 (FZ-A1..A6), with the standing M2 receipt
explicitly old-layout evidence ([[m2-receipt-reearn]]). It covers
arithmetic/projector parity only and computes no action value, selected lead,
optimal set, information net, continuation, performance claim or player.

Discipline carried by types, not by promises: policies map info-state ids, so a
world-peeking policy will not compile; caps exclude rather than sample; and in
the archived factory the ledger's deletion path needed a token whose only
constructor was a registry with a checker in it (the seat-honest observation
type lived in the deleted `walt-skeleton` and is archive-only with it).

## The solvers, and their honest performance characteristics

Five operators exist, and §10.8's rule is enforced socially and by type: a theorem
for one operator never silently transfers to another. PI-averaged action values are
the information-relaxed diagnostic, **not** the seat's hidden value Q^H; the gap is
the strategy-fusion gap and it is action-specific.

| Solver | File | What it computes |
| --- | --- | --- |
| Symbolic parametric PI | `walt/walt/src/strat/pi.rs` | Worldwise perfect-information backward induction over the whole valuation ray; every root action value is a continuous PWL envelope |
| Scalar PI | `walt/walt/src/strat/scalar.rs` | The same PI operator at one integer valuation, with a trick-boundary cache keyed on semantic state; the workhorse for whole-fiber and census work |
| Symbolic H | `walt/walt/src/strat/hidden.rs` | The actual hidden-information fixed-field treatment at the root, exact on the whole ray (pooled maximization decomposes because the canonical partition is a tree) |
| Scalar H (`dag-v1`) | `walt/walt/src/strat/hidden_scalar.rs` | Exact Q^H per legal action at arbitrary (including mid-trick) decision points, unit-fraction particle weights, budgeted, with pooled-state boundary memoization |
| Revealed C and F | `walt/walt/src/strat/revealed.rs` | Continuation- and root-revelation with the field held fixed, aggregated at the support level so no polytope is materialized |

A sixth solver family exists since 2026-08-17 and is the live one: the
**scenario-player stack** in `walt::solver` — exact best response over sampled
fiber worlds against modeled level-k minds under the pmake objective, with
race-then-refine as an opt-in. It is owned by
[walt-seat-play](walt-seat-play.md) and specified in `walt/SCENARIO-PLAYER.md`;
this page does not restate it.

Measured facts, each exploratory tier and each attached to its source. (The
`results/...` files cited below now live under `walt/probes/factory-results/`;
the `examples/*.rs` producers are archive-only at commit `648f93a`.)

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
  decisions (tree side 4.2e9 to 6.5e10 steps). Byte-identical Q^H was a CI-pinned
  invariant while the factory existed (`walt-factory/tests/h_value_transparency.rs`,
  sixteen decisions, including the `Q^H(2-1) = 80/7` vs `Q^H(3-2) = 202/21` pins);
  that test is archive-only at `648f93a` with the rest of the factory. The offline
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
- **Deadness detection is cheap, and now has a quotable instrument.** The often
  repeated "about 25 ns per detector call" is **contended and not quotable**, and
  it is not even in the results file: `results/deadness_2026-08-12.txt` records a
  RESUMED run and prints `0 ns over 0 calls`. Freeze 43's sequential timing rung
  (DS-A33) — the only quotable timing instrument — was subsequently run:
  `results/deadness_rung_2026-08-13.txt` records 17 ns/call over 384 calls at the
  declared grade-3 unit and 42 ns/call over 3,540,143 calls at the declared n=4
  unit, single uninterrupted process, declared selection rule, run complete. Those
  are the quotable figures; the 25 ns stays retired.

## The calculated-evidence instruments (live, added 2026-08-24)

Seven bins built during the §22 / §21 / slice-4 builds, each with a records directory
under `walt/probes/` carrying **its own fence** — exploratory instrument
output, below every evidentiary tier, cited by nothing above it. The results
are read out on [walt-calculated-evidence](walt-calculated-evidence.md), which
owns them; this table is the inventory of what exists and how to run it. All
of them write JSONL beside a stdlib-only `summarize.py` that recomputes the
published tables from the records, so no aggregate is hand-maintained.

| Bin | What it produces | Records |
| --- | --- | --- |
| `shadow` | The step-7 controller run beside the live player: frozen level-1 continuations (`ActionRule::PinnedThenLevel1`) at every multi-option focal decision, agreement recorded and never acted on | `probes/shadow/` |
| `fieldswap` | The slice-1 fixed-policy smoke: `FrozenPolicyExposure` under declared field models σ0/σ1 for two frozen focal pins per root — never a root-action bound, never screening input (L2-A4, O31) | `probes/fieldswap/` |
| `fieldswap_screen` | The slice-2 rung/screen instrument: exposure rungs E0–E2, the exact split-reach route E4 (`R_a` exactly), and the L2-T4 admissible screen with the full ordered-pair slack table, at one declared (σ0, σ1) epoch pair | `probes/fieldswap_screen/` |
| `fieldswap_cancel` | The slice-3 instrument [L2 thread]: the cancellation ladder `(d, r, c⁺, c⁻, c)` with \|c\| ≤ r ≤ d, pairwise `(B, H, q, g)` masses with the exact-enumeration-only dominance route, the six-label vocabulary, directional rungs (R±)^U with the sandwich and §36 winner stability, sampled E3 typed `estimate`, Stage-4 survivor-only σ1 work, and the asserted Λ = 31/1200. Adopted by PANEL-A7/A8 | `probes/fieldswap_cancel/` |
| `v5flip` / `e0cal` | Step 8: the §19 V5 cap-ladder replay with `assert_cap_ladder` (settled stays settled identically; never two caps settled differently), and the §19 V6 per-fixed-pair E0 calibration comparing exact fiber coordinates, exact-rational forecasts, and observed replicate settlements | `probes/step8/` |
| `fieldswap_motifs` | The slice-4c instrument [L2 thread]: six-motif first-split morphology + `Other` with all coordinate-difference flags over the (suffix-enriched) correction traces — partitions **correction mass, never exposure**; exact m_k± decomposition asserted against the committed cancel ladders; sampled histograms typed descriptive, locked out of the screen (TRIPLE-A6/A7) | `probes/fieldswap_motifs/` |
| `hazard_witness` | The slice-4b instrument [L2 thread]: the one-round trump-extraction producer run over the cancel corpus's pairwise comparisons, accepts carrying verifier-checked witnesses and declines carrying the failed-hypothesis histogram — honest narrowness recorded, never widened (TRIPLE-A4/A5) | `probes/hazard_witness/` |
| `wakeup` | The step-9 instrument [L2 thread]: the detection layer — per predeclared pair, the three wake-ups (response/value/decision, never collapsed) under a declared (σ0, σ1) epoch pair, exact complete-fiber route beside a dig-until-settled sampled route on one common paired stream, 𝓘-interval cost verdicts only, typed `DetectionRefusal`; σ0 chosen as step 8's exact field so the Stage-1 consumption (L2-A6) is asserted, not assumed | `probes/step9/` |
| `l2_controller` | The targeted-controller instrument [L2 thread]: per-root §8 Stages 1–5 with schedule-controlled rung spend (cheapest first; exact-E4 escalation refused `provably-useless` when the lower-witness admissible set proves it cannot prune — refusals re-verified against real E4 in the gates), honest degradation over the exact cap (`DeltaFrozenSet` δ-intervals, zero-hypothetical-gated sampled E3), typed refusals everywhere else; no default touched (CE-A7/§20.16) | `probes/l2_controller/` |
| `waking_bridge` | The waking-seat player surface [CE and L2 threads]: the controller_bridge line protocol plus a `driven` profile mode — one typed record per decision (path, wake evidence, escalation `StageFourOutcome`, per-phase integer-microsecond `PhaseSpend`, worlds consumed); the phase-conviction records that targeted the speed campaign | `probes/waking/` |
| `ordering_bench` | The reorder-not-cull A/B (E-A15): both `MoveOrdering` arms over receipt and synthetic roots, values asserted byte-identical, children/legal break counters as the clean signal | `probes/ordering/` |
| `field_cache_bench` | The surgical-levers A/B/C: bare field vs cached field vs cached+decided-cutoff over the same (world × candidate) replay grid, wins vectors asserted identical across arms | `probes/field_cache/` |
| `bundle_bench` | The bundled-evaluator A/B: per-world route vs `solver::bundle` on complete fibers under two field configurations, wins asserted equal, exact node/field-query/settlement counters | `probes/bundle/` |
| `rootinterval` | The counted-belief Slice A instrument [L2 thread, added 2026-08-30, CBS-A1..A9]: per legal root action, exact `Q_a` beside the δ-valid root interval — the pmake empirical-max upper (`sampled_root_optimum` inverted through the CE one-mean engine, no policy-count risk penalty) over a frozen-policy lower witness with the §6 discovery/evaluation lock — survivor-set evolution by prefix, worlds-to-singleton, and the upper-excess vs lower-shortfall attribution; ties come out `UnresolvedRootSet`, never a forced winner | `probes/root_interval/` |
| `factorbelief` | The counted-belief Slice C stage-C0/C1/C2 instrument [L2 thread, added 2026-08-30, C1 and C2 modes 2026-08-30, CBS-A6/A9]: exact one-ply branch masses `{t ↦ Z_ht}` by two routes — the §21 contraction (acting-seat root hands × exact completion binomials, each hand field-classified once) beside complete-world enumeration — with route parity and `Z_h = Σ_t Z_ht` asserted per row; the §22 opening-root demonstration (399,072,960 worlds → 116,280 hands, branch table in 8.7 ms trivially / 5.36 s under the σ0 mind, no world materialized) and the §26 cost coordinates; the `cache` mode is the stage-C1 study — first/repeat/bundled costs per root, the bundled one-ply oracle with full extensional cache identity, cross-history sharing (measured exactly 0 under the full §43 key), and the opening root's 200 ns/query repeat identity cost (×230 over first classification); the `c2` mode is the stage-C2 report — §46's seven required coordinates from ONE opening-root run under the σ0 field (116,280 hands asserted; contraction 5.9 ms of completion weights and 21.8 ms warm; classification 5.34 s derived by subtraction, 45 µs/hand, 99% of the cold pass; 20 distinct branch tiles; reuse ×245 at 187 ns/query; memory as a declared 23,563,392-byte cache accounting BESIDE a measured 63,340,544-byte maximum resident size, never one dressed as the other; conservation exact at 399,072,960) | `probes/factor_belief/` |
| `factorrecursion` | The counted-belief Slice D instrument [L2 thread, added 2026-08-30, CBS-A6/A9]: the §23 factorized fixed-policy recursion over the general support contraction (`SupportOracle` — §25.2's acting-hand loop generalized to conditioned completions) beside the bundled complete-world walk, value parity asserted on every row — six trick-5/6 roots × two frozen focal policies × two fields plus four trick-4 roots (deepest: fiber 34,650, 16 post-root plies, 121,868 conditionings under σ0) — with exact value pairs `M/Z`, per-route integer microseconds, and the recursion's node census; honest negatives recorded (bundled faster at worlds/hands ≈ 3; the recursion classifies record-consistent zero-completion hands the bundled walk never meets); the opening-root recursion deliberately not attempted | `probes/factor_belief/` |
| `factorresponse` | The counted-belief Slice E instrument [L2 thread, added 2026-08-30, CBS-A4/A6/A9]: the §48 factorized grammar best response (`grammar_success_mass` — the §23 recursion with the focal case's frozen action replaced by a max over the grammar's actions, lawful because focal children share `Z`) — per grammar root action the exact `Q^G_a` beside the Slice B enumeration split (parity asserted, §12 verdicts and `free`/`dev` from the split only, per the §48 fence: the recursion never maximizes over the full action set), each source's fixed-policy value beside the optimum (the dominance picture), and trick-4 depth rows; the finding: at trick-4 roots the grammar mix strictly beats every source (h4-t4 trivial `Q^G = Z = 34,650` — certain make — against 34,170 for the best source), while at trick-5/6 roots it never exceeds the best source and the two-source grammar saturates every reached undecided state; honest negative: the enumeration split is 30–40× faster at worlds/hands ≈ 3 | `probes/factor_belief/` |
| `factorcegar` | The counted-belief Slice F instrument [L2 thread, added 2026-08-30, CBS-A6/A9]: the §49 consequence CEGAR at the field-classification bottleneck (`refine_to_action_exact` — §28 hand classes under the starting vocabulary of critical-tile membership, trump count/highest trump, led-suit count, count pips, current-winner/ruff possibility; §30 witness-pair refinement, the lowest differing tile of each witnessed pair entering the §31 critical set) — per-root stage tables of classes, action-exact mass, and per-branch interval widths `[L_t, U_t]` down to the action-exact endpoint; the two-sided finding: at the opening root under σ0, 805‰ of the 399M-world posterior mass is action-exact at 36,923 classes (3 hands/class; 513‰ at 5,387 classes), but ZERO residual costs full fragmentation to 116,280 singleton classes (§51's falsifier for the tail under a SAMPLED mind — trivial-field endpoints on the same roots DO aggregate), vindicating §49's interval discipline; the instrument pays the full per-hand classification bill and claims representational structure only, never a faster classifier | `probes/factor_belief/` |
| `factorrefine` | The counted-belief Slice G instrument [L2 thread, added 2026-08-30, CBS-A6/A9]: the §50 integrated refinement controller (`refine_root` in `solver/refine.rs` — one typed interval `[L_a, U_a]` per legal root action, the buildable §33 work-item subset over Slice A's sampled δ bounds, the Slice D/E exact recursions as lowers, and the §36 EscalateExact endpoint `response_success_mass` gated to bundled-authority parity; §34 refusals recorded and never charged, §35 width-per-cost scheduling in exact rationals, budgets charging declared integer forecasts so every run is a pure function of its inputs) — per-root controller traces of decision width against cumulative declared cost (§53's central graph); the findings: the exact ladder settles all ten gated roots (twice WITHOUT escalating the winner — the winner's cheap exact lower cleared every rival's escalated point), the sampled tier settles small fibers before any exact recursion runs (correctly δ-qualified) while at trick 4 its uppers are too loose to prune, and at the opening root every exact item is refused by its own forecast with the honest UNRESOLVED surviving set returned, the fallback named and never promoted | `probes/factor_belief/` |
| `factorprofile` | The anytime proof-state Phase 2 instrument [L2 thread, added 2026-08-31, APS-A2]: the §18 fixed-policy 43-bin score profile (`viewer_score_profile` in `solver/factor_belief.rs` beside its Slice D/E/G siblings — bin `s` = the exact world mass banking exactly `s` declaring-team points; viewer-independent, bid-blind, so one run yields the whole bid-threshold curve) — per-root exact profiles under σ0 for two frozen focals with the tail-permille curve at every threshold, the exact expected score by the §3 tail-sum identity, the §10/§11 rescue and fragile-make band masses at the root's contract, and the honest price of forgoing the decided cutoff (~7–12% extra wall at trick 4 for ~double the nodes); findings: certain outcomes now carry their explanation (h12-t6's miss is exactly 20 points in every world), the σ0 make-mass spikes EXACTLY at the bid (the settled branch made visible — 445‰ at s = 30 on h8-t5), and cross-contract reuse is void under the bid-reading σ0 (the gate-3b frozen specimen) while exact under bid-blind semantics | `probes/factor_belief/` |
| `proofreport` | The anytime proof-state Phase 3 instrument [L2 thread, added 2026-08-31, APS-A6/A7]: the first §33 recommendation blocks on real roots — per root the proof state is fed RefineV1's two-tier facts plus exact lowest-first continuation profiles for up to three legal actions, and the report prints the recommended executable policy, its certified pmake floor `B_exec`, the global upper `U*`, the certified regret `Γ = U* − B_exec`, declaring score floor/ceiling with the §10/§11 d = 1 bands, proof class, and the sampled-scope summary; findings: certified regret ZERO far from certain make (h5-t6 at 444‰ — optimality certainty ≠ make certainty), §30's proof-bar/executable-bar gap on trace (h3-t4: settled action 3-1 at Q = 350‰, recommended executable policy starts 4-4 at floor 267‰, Γ = 83‰ — pmake belongs to the policy, not the first tile), and certified Γ = 0 in both certain directions | `probes/factor_belief/` |
| `grammarsplit` | The counted-belief Slice B instrument [L2 thread, added 2026-08-30, CBS-A4]: per legal root action the exact §12 triple `free`/`gram`/`dev` (unrestricted, grammar-restricted, and residual optima, `free = max(gram, dev)` asserted at every walk node), verdicts (`closes`/`ties`/`counterexample`/`root-off-grammar`), lazy first-deviation witnesses, the grammar-room census, root-closure lines (`Q^G` best vs exact best), the sampled-route triples, and the §8 residual-upper identity in the numbers (residual empirical-max upper == the full-class upper, byte-identical count paths); grammars G1/G2/G3 over lowest/highest preference, the pinned level-1 continuation, the σ0 mind, and count-preservation safety | `probes/grammar_residual/` |

Two declared-knob facts a future session must not inherit silently:

- **The shadow bin's `world_cap` default is 512** since PR #32 / `6e00528`
  (Jason's 2026-08-24 cap ruling: the 128/40/160 caps were phone-tier budget
  limits, not calibrated choices). The committed shadow records are the
  **world_cap = 128 epoch**; reproducing them byte-identically now requires
  passing `128` explicitly. A 512-epoch regeneration is a separate run and
  supersedes nothing — different epoch by construction, and every record
  carries its own config.
- **A cap is a resource limit, never a settlement rule** (CE-A3), and a
  declared field or policy schedule is part of the result's identity: a
  different `FieldId`/`PolicyId` is a different experiment, not the same
  statistic improving. The `fieldswap_screen` epoch pair is
  σ0 = `Level0{n0=8}`, σ1 = `Level1{n_outer=4, n0=2}`, frozen candidates
  `[8, 2]`.

The load-bearing gates for this machinery are ordinary tests, not the probe
directories: `walt/walt/tests/solver_calibrate.rs` (step 8),
`walt/walt/tests/solver_fieldswap_screen.rs` (the O32/O38 parity gates) and
`walt/walt/tests/solver_fieldswap_cancel.rs` (slice 3 — the ladder
inequalities, the sandwich, the extended ladder and the winner-stability
table are *asserted*, not printed), `walt/walt/tests/solver_e3_upper.rs`
(slice 4a — the worked specimen exact and the full 256-table sweep
reproducing worst undercoverage 11/128), `walt/walt/tests/solver_hazard_witness.rs`
(slice 4b — both x:024 Part-2 specimens plus verifier-rejection and
type-lock gates) and `walt/walt/tests/solver_fieldswap_motifs.rs`
(slice 4c — the P6 partition, decomposition identities, and the
descriptive-tier screen lock), `walt/walt/tests/solver_wakeup.rs` (step 9 —
the §14.4 worked fixture, the 12-unanimous-pivot settlement minimum, and the
wake-up type locks) and `walt/walt/tests/solver_panel_conformance.rs` (the
PANEL-A3/A5/A6 conformance gates, including the ported O26 batch-boundary
divergence witness), and `walt/walt/tests/solver_targeted.rs` (the
controller — the screen-input type lock, the pruning and no-prune worked
specimens, and the provably-useless refusals re-verified against real E4),
and — since 2026-08-25 — `walt/walt/tests/solver_waking.rs` (the waking
seat — nine gates plus two compile_fail locks on the wake rule and the
fallback recording), `walt/walt/tests/solver_ordering.rs` (the two-arm
value-equivalence gate and the ascending-baseline pins),
`walt/walt/tests/solver_field_cache.rs` (complete `SetEvaluation`
identity bare-vs-cached, cutoff-vs-full per world over two complete
fibers re-deriving the pinned wins vectors) and
`walt/walt/tests/solver_bundle.rs` (element-wise equality against the
exact per-world oracle, attribution completeness, focal-purity
rejection), and — since 2026-08-30 —
`walt/walt/tests/solver_root_interval.rs` (counted-belief Slice A: the
mirror-endpoint sweep at the adjudicated 11/128, realized L ≤ Q ≤ U
against `exact_root_value` on both frozen fixtures with the exact
optimizer surviving, the §6 same-stream lock as a refused construction,
malformed-count-path rejection, exact risk summation, and
starved-budget `UnresolvedRootSet` typing), and
`walt/walt/tests/solver_factor_belief.rs` (counted-belief Slice C stages
C0–C1: three-way uniform-mass parity — backend zero, the canonical count,
complete enumeration — with Theorem 23.1 focal invariance; branch-mass
parity against world-by-world classification under two trivial fields
and, on the smallest fibers, the σ0 level-0 mind; the Theorem 20.1
conditioning route recovering each branch mass with only the acting
factor touched; conditioned marginals against enumeration; the declared
C0 domain refusals — field-identity mismatch, focal/hidden node
confusion, and the two-table Slice-D boundary refused by panic; the
§22 opening root contracted to 116,280 hands with exact conservation,
never enumerated; and the four C1 cache laws — σ0 branch parity with the
bundled one-ply oracle on all six fibers with the two routes' caches
equal as maps, classification once per information state with
conditioning adding exactly the zero-completion support hands,
zero sharing across focal candidates or roots under the full §43
identity key, and the opening root's 116,280 hands classified by the σ0
mind exactly once with a repeat contraction that is pure cache
identity — stage C2 added no gate here by design, being a report stage
whose every in-run assertion this file already carries), and
`walt/walt/tests/solver_grammar.rs` (counted-belief Slice B: the §12
decomposition walked against `exact_root_value` on every fixture action
with the Theorem 9.1 identity asserted nodewise; a singleton grammar's
restricted optimum equal to its source's §6 replay count; sampled-route
parity with `sampled_root_optimum` by prefix; source-monotone `Q^G`;
off-grammar root actions typed all-residual with depth-0 witnesses; the
§8 residual-upper identity against the Slice A producer; the level-1
continuation dominated by its own grammar; and the empty-grammar and
illegal-source refusals), and
`walt/walt/tests/solver_factor_recursion.rs` (counted-belief Slice D:
the support backend extensionally equal to backend zero across the C0
domain including the opening root's contraction; surviving-world mass
parity beyond one table with backend zero's two-table refusal preserved
at the boundary; the §47 value gate — the factorized recursion's success
mass equal to the bundled walk's wins on every enumerable root under
two frozen focal policies and both the trivial and σ0 fields; and the
every-node checker — mass equals the surviving-world count and branch
masses equal the world partition at EVERY node of the recursion tree,
with the walk crossing the two-table boundary on every multi-trick
root), and
`walt/walt/tests/solver_factor_response.rs` (counted-belief Slice E:
per grammar root action the factorized `Q^G_a` equal to the Slice B
enumeration split's grammar optimum under σ0 with the root call the max
over grammar root actions; a singleton grammar collapsing exactly to
the Slice D fixed-policy recursion under both fields; every grammar
source dominated by the grammar optimum with the constraint proved to
BIND somewhere via singleton grammars — the two-source grammar ties the
free optimum on every enumerable root; and the every-node checker with
the §48 grammar-max structure enumerated at every focal node, the walk
crossing the two-table boundary), and
`walt/walt/tests/solver_factor_consequence.rs` (counted-belief Slice F:
Theorem 30.1's monotone narrowing — residual class mass nonincreasing,
exact mass nondecreasing, per-branch intervals `[L_t, U_t]` NESTED as
the critical set grows by exactly the witnessed discriminator, ending
action-exact with point intervals; endpoint parity — the fully refined
abstraction reproduces `branch_masses` tile for tile; §49's witness
requirement re-derived independently — same pre-stage class signature,
field actions re-derived through the field itself on hand-built
records, the discriminator held by exactly one hand and fresh, the
post-stage signatures genuinely split; and non-vacuity — the bare
vocabulary resolves positive mass, classes aggregate, and the
refinement loop fires in the gated corpus, all on the six enumerable
roots plus the four trick-4 roots under both declared fields), and
`walt/walt/tests/solver_factor_refine.rs` (counted-belief Slice G:
escalation parity — the §36 full-action-set factorized recursion
equals the bundled exact authority `exposure::exact_root_value` at
every gated root and action, with the containment chain fixed-policy ≤
grammar ≤ response; the §37 soundness invariant walked over full
controller runs — lowers only rise, uppers only fall, the bar
monotone, exclusions permanent, every exact bound independently
recomputed, result typing faithful to the surviving set; §34 steering
— the consequence-census item refused as presently useless at every
bar, no work on excluded actions, refusals charging nothing, and
bytewise run-determinism; and §36 step-12 honesty — starvation
returning the surviving set with the named never-promoted fallback,
work within budget, the δ ledger re-asserted through the shared
`assert_screen_risk_allocation` and zero when the sampled tier is
off), and
`walt/walt/tests/solver_factor_profile.rs` (anytime proof-state Phase
2: mass conservation `Σ H = Z` with the tail projecting to the Slice D
success mass under both viewer parities and the early decided cutoff
never firing; the §3 tail-sum identity exact on every computed
profile; §44 contract reuse exact under bid-blind semantics against
independent per-contract re-runs, including bids 0 and 42; the reuse
BOUNDARY as a frozen specimen — the bid-reading σ0 makes a
cross-contract answer a re-run, not a projection, h10-t6 threshold 42
projecting 12 against the exact 9; and entrywise parity with an
independent complete-world replay to true terminals under both
fields), and
`walt/walt/tests/solver_proof_state.rs` (the §49 architecture spike —
no instrument, the gates are the deliverable: the zero-budget top
state sound with bytewise serialize/resume; RefineV1 endpoints
imported as typed facts with closure reproducing survivors,
exclusions, bar, and typed result on every enumerable root under both
ample configurations; closure idempotent and
insertion-order-independent; §51 identity fences and content-hash
round-trips; a score-profile fact raising the executable bar through
§41 derivation with `B_exec ≤ B_proof` asserted in every closure; and
the open producer registry proven by a test-file banked-floor
producer closing a repriced root to Equivalent-at-1 without touching
the module), and
`walt/walt/tests/solver_proof_regret.rs` (anytime proof-state Phase 3
— exact profiles project exactly into the §33 recommendation at
independently recomputed values; `Γ = U* − B_exec` contains the exact
best-response regret against the bundled authority before and after
the RefineV1 import with `Q* ≤ U*`; Γ nonincreasing, `U*` nonrising,
`B_exec` nonfalling under fact-by-fact refinement; a non-executable
grammar lower raises only the proof bar and recommends nothing; and
cross-contract reuse of report quantities exactly under bid-blind
semantics, the σ0 boundary owned by the profile gates' specimen).
Per the receipt-discipline statements
below, none of these JSONL records is byte-diffed by CI and none becomes a
claim-tier result by existing.

## The playable controller surfaces (added 2026-08-24) [CE thread]

**[`walt/CONTROLLER-PLAYER.md`](../walt/CONTROLLER-PLAYER.md) is the owning
register** — the action-policy route table, the knob list, the surface flags
and the gate inventory live there and are not restated here; this page records
only that the surfaces exist and what class of object they are. All
exploratory tier, estimates and never receipts, and **no strength claim is
made or implied**: the old player remains the default (CE-A7/§20.16).

| Surface | What it is |
| --- | --- |
| `solver::act` | The library entry point — one decision in, one `ActDecision` out, carrying the `route` that chose the tile and a `settled` flag that is **false on every fallback**. δ-safe elimination is inside the correctness boundary; the level-1 ranking among survivors or exact ties is an ordering choice outside it (the W7/filtration license) |
| `bin/controller_bridge` | The arena/plunge bridge — the **same** line protocol as `walt_bridge`, so an external consumer seats it with zero changes on its side. `WALT_CTRL_LOG=<base>` appends one JSONL record per decision (route, settled flag, among-set, fallback options) — record-grade output, not a receipt |
| `webtable` / `playtable` `ctrl [cap=N]` | Controller seats at the AI chairs for the play phase (auction and trump pricing stay level-1); each play reports its route |

A third declared-knob fact to add to the two above: **the `ctrl` cap is a
think-time budget, not a settlement rule** (CE-A3/§1.5). The interactive
default is **128** and the batch default **512**; a low cap yields *more
honest fallbacks*, never a wrong settlement, and the cap is part of the
result's identity like every other declared knob on this page.

The same delivery domain-separated the deal and belief streams in `playout`,
`playtable` and `webtable` (the O27 audit finding), so a seat's belief sample
no longer depends on how many draws other consumers took. The separate §3.4
`playout` PiKey/banked-totals defect is **not** fixed and stays filed —
[walt-calculated-evidence](walt-calculated-evidence.md) carries both.

## The probe binaries (historical — all archive-only since 2026-08-24)

All lived under `walt/walt-factory/`, deleted by the unification. Regenerating
any output means checking out producer commit `648f93a` first, then
`cargo run --release -p walt-factory --example NAME [subcommand]`
(`walk_corpus` was a `src/bin/` binary); verify against the archive manifest
digest afterwards ([`walt/ARCHIVE.md`](../walt/ARCHIVE.md) — frozen seeds make
byte-identity the expected outcome, and a mismatch is a finding). The results
files listed live at `walt/probes/factory-results/`.

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

**Byte-frozen fixtures.** The factory's three (`walk_h0_S1.txt`,
`ci_corpus_pins.txt`, `lesson_h0_S1_t5.txt`, asserted for exact string equality
by its walker/lesson tests) are archive-only at `648f93a` with the crate that
checked them. The unified crate's surviving frozen fixtures are the exp5 census
samples under `walt/walt/tests/data/`, asserted by the `strat_exp5_census` and
`kernel_known_fibers` tests, plus the frozen native trace the wasm smoke
byte-compares.

**Results artifacts.** The tracked result summaries — every dated `.txt` this
page and the era pages cite, each opening with its own tier line, its binding
rulings, its declared scope and (for the later ones) its exact regenerate
command — were relocated intact to
[`walt/probes/factory-results/`](../walt/probes/factory-results/) (provenance
README there). That includes `certificates_2026-08-10/` — sixteen §16.11
records, one per lesson in the S5c-m3 working set (ten `cert_refutation_*`,
five `cert_win_*`, one `cert_checker_*`, filenames deterministic from content
keys), written against the self-contained `certificate-schema.md` beside them
(schema-v1, the historical filename that keeps walt's own "certificate" name)
so an independent implementation can check them. The untracked bulk (8.3G of
raw outputs, 514M of stores) lives at `~/data` and HuggingFace per
[`walt/ARCHIVE.md`](../walt/ARCHIVE.md), never in the repo.

**GPU-track comparands.** Portable M0/M1 has the canonical envelope, declared
stop and summary under `walt/receipts/gpu_native_trick1_m0_m1_v1/`. The separate
`gpu_native_trick1_gate0_2026-08-16.txt` is retained unchanged: its NO-GO remains
a true observation of the old Command-Line-Tools-only environment. Freeze 56 has
one committed binary receipt and external checksum under
`walt/receipts/gpu_native_trick1_m2_v1/`. That M2 receipt is executable evidence,
not a Lean theorem and not a persisted value for a solver to consume.

**What portable `walt/ci/check.sh` enforces.** It verifies immutable M0/M1 history
at the producing commit and the received-guide checksum; regenerates and
byte-diffs the M0/M1 comparands; runs formatting, warning-denied clippy,
source-level no-float gates and all release workspace tests; and builds the
trick-1 Lean targets and audits the M2 theorem axioms. Since freeze-56 v2
(FZ-A5) the *full* cumulative source-manifest closure is a **freeze-event**
verification — run `ci/verify_m2_sources.sh` when a freeze event re-issues the
manifest — because the unified crate contains the actively developed solver and
a per-commit full-digest closure would be red on every ordinary commit. CI never
skips unavailable Metal work into green and by itself issues no M2 result.

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

1. The legacy probe artifacts (now `walt/probes/factory-results/*.txt`) are
   **not** diffed by CI, and since the factory's deletion their byte-equality
   coverage by ordinary tests is archive-only too; none becomes a claim-tier
   result merely by existing.
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

**Stores.** The factory's gitignored caches (`store/endgame_l2.store` — the
level-2 endgame form store, `store/deadness_ckpt` — per-unit run checkpoints
with a freeze digest, `store/candidate_library.txt` — candidate library v1,
freeze 36: observation-record keys, no values, no verdicts, identity transport
only, cache never authority) went to the local archive with the crate
(`~/data/texas-42/walt-factory-archive-2026-08-24/store/`). Every headline
number keeps a cold-regenerate path that starts by deleting its store — at the
producer commit, per the recompute queue.

## The rescued Python probe suites

`walt/probes/` now holds all the frozen probe records: the two Python suites
below (preserved verbatim from the 2026-08-09 scratchpad before `/tmp` cleanup
could destroy the only copies), the relocated factory result summaries
(`factory-results/`), the seat-play result files (`m3/`), the bidcurve
corpus (`bidcurve/`), and the calculated-evidence instrument records
(`shadow/`, `fieldswap/`, `fieldswap_screen/`, `fieldswap_cancel/`, `step8/` —
the section above).
The Python suites' framing is the load-bearing part: they
are **frozen validators, never source**. walt reimplements from the definitions
in the frozen mathematical basis and pins its own results against the probe
records; a disagreement is a discrepancy to be recorded, never a reason to copy
probe code into the implementation.

| Suite | Contents | Role |
| --- | --- | --- |
| `probes/exp3a/` | `lambda_probe{,_v2,_v3}.py`, `v3_diag.py`, four `*_output_postfix.txt` runs, `lambda-probe-report.md`. `lambda_probe_v3.py` Part 1 is Experiment 3A: the 22-observable atom registry whose semantics live only in that file | Supplied the vocabulary S4 had recorded as lost; was ported into `walt-skeleton::atoms::Exp3aAtom` with the 90 → 33 → 8 reproduction as a live test — both archive-only at `648f93a` since the skeleton's deletion; the frozen Python records remain in place |
| `probes/exp5/` | `exp5_core.py` (bitmask PI minimax, exact counting/sampling DP), `exp5_rules.py`, `exp5_census.py`, `exp5_validate.py`, `exp5_report.py`, `exp5_pwl.py`, `exp5_exact.py`, `exp5_records.jsonl` (566 records), `exp5_results.md` | The designated second implementation for cross-checking; its census vectors (h1t3 = 10, h3t3 = 5,345) and 52 kernel fiber sizes are regression pins in `walt/walt/tests/strat_exp5_census.rs` and `walt/walt/tests/kernel_known_fibers.rs` |

Both are stdlib-only Python 3.12 with exact `Fraction`/integer arithmetic. Running
them creates `__pycache__` — clean it up (D15).

## What was mechanically blocked, and why that was safe (historical — the machinery is archive-only)

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
the block. While the factory existed the machinery was intact and exercised in
CI (`tests/economy_pins.rs`); since 2026-08-24 the whole apparatus — the lesson
DB, the watched-feature index under its candidate-completeness contract
(exhaustively cross-checked, 179 × 16 = 2,864 pairs), the dual H-primary rent
ledger with "unmeasured is never zero", and §16.11 record emission with
per-record checker-coverage annotations and H rows honestly marked
UNCHECKED-EXTERNALLY — is archive-only at `648f93a`, with the emitted
certificates preserved in-tree under `walt/probes/factory-results/`.

## How to run things

```
/bin/bash -p walt/ci/check.sh                  # portable gate (from repo root)
/bin/bash -p walt/ci/check_m2_metal.sh         # native Metal gate
cargo run --release -p walt --bin level1       # the seat (or scenario, level2,
                                               # playout, webtable, walt_bridge,
                                               # controller_bridge, divergence,
                                               # ladder, bidcurve, tiltaudit,
                                               # m3probe, playtable, shadow,
                                               # fieldswap, fieldswap_screen,
                                               # fieldswap_cancel, v5flip, e0cal)
/bin/bash -p walt/ci/verify_m2_sources.sh      # freeze-event manifest closure only
```

The historical factory probes are archive-only — `git switch --detach 648f93a`,
then `cargo run --release -p walt-factory --example NAME [sub]` per the
recompute queue in [`walt/ARCHIVE.md`](../walt/ARCHIVE.md). Do not run
`ci/check.sh` casually: it builds the workspace in release, runs the full test
suite and builds Lean. `check_m2_metal.sh` additionally requires the exact
native Metal toolchain and a real device; it is intentionally not portable.
Several probes are hours of compute; the factory's `h_dag_probe` was `#[ignore]`d
precisely because it is a declared manual run.

**Declared knobs a future session must state, not inherit silently.** These
were constants or CLI arguments in the factory sources (archive-only at
`648f93a`), and every one of them is part of the declared inputs its result is
quoted under:

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

- **Several historical instruments lived only inside example binaries, and all of
  those are archive-only now.** The three deadness detectors (`d0`, `d1_sym`,
  `d1_win`), the exclusion-predicate engine, the endgame form store and floor
  table, and the railyard level-step drivers were functions inside
  `deadness_probe.rs`, `fiber_probe.rs` and `census_run.rs`; the r3 retrograde
  class machinery and yard/suffix-library routines were library code in
  `walt-skeleton::equivariant`. All of it is at producer commit `648f93a` only.
  Reusing a detector now means retrieving it from the archive and lifting it into
  the unified crate first.
- **Legacy receipt-corpus statistics are pip-trump.** The corpus
  (`rob/receipts/verify_player.txt`) has no doubles-trump and no no-trump hand, so
  every statistic derived from that corpus validates the pip-trump path and
  nothing else. The one exception is the complete level-one alphabet run, which
  enumerates its own carrier — and is still declared pip-trump only. The
  independently generated freeze-56 M2 carrier has its own frozen scope and is
  not typed by this legacy corpus caveat.
- **The sequential timing rung has since been run.** An earlier revision of this
  page recorded `deadness_rung_2026-08-13.txt` as referenced-but-nonexistent; the
  freeze-43 rung (DS-A33) was subsequently executed and the file exists at
  `walt/probes/factory-results/` — 17 ns/call (grade-3 unit) and 42 ns/call (n=4
  unit) are the quotable figures. The ~25 ns/call figure remains contended and
  not quotable.
- **The weighted H re-solve over a pre-built class DAG — the number the
  belief/policy-iteration platform claim rests on — is still unmeasured.** The
  existing H solvers take a uniform fiber weighting only and the K-bar integration
  is unbuilt. This is stated in the results file itself (P-A14), not just here.
