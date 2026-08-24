# Walt unification census (2026-08-24)

Status: WORKING INVENTORY, exploratory tier throughout — analysis for
the reorganization, no status change to any result. Owns: the crate
census, dependency map, artifact inventory, and fold plan for the
one-walt unification (mandated in `LOG.md` and `LEVEL2-PROBE.md`).
Sources: direct survey of `walt/` at merge `dcdc14b` (post PR #6/#7);
`Cargo.toml` workspace manifest; `ci/check.sh`; `PLAN.md` (stale, see
flags); commit 97ce321 (the preserved WIP orphans).

## Headline

The "17 crates" are really **three disjoint stacks plus three
orphans**, and only one stack is the thing walt currently is (the
player that beat the eq champion and runs in plunge). Three crates do
not build at all: `walt-m3-net` and `walt-m3-oracle-a` are refused by
cargo outright, and `walt-m3-metal` only compiles because it hides
behind its own nested `[workspace]`. All three are preserved remains of
one WIP commit (97ce321, 2026-08-17, "mid-flight, does not build").

## 1. Crate census

| crate | purpose | member? | deps (path) | status |
| --- | --- | --- | --- | --- |
| walt-core | Straight-42 rules layer: pips, 28 dominoes, seats/teams, 9 declarations, rule algebra, scoring, legality, replay | yes | — | LIVE — foundation of all three stacks |
| walt-kernel | Viewer kernel + current-remainder fiber: enumeration, exact counting DP, exact uniform sampling | yes | core | LIVE |
| walt-geom | Exact rationals, PWL envelopes, capture features, feature sets as polytopes | yes | core | LIVE (research stack) |
| walt-strat | Operators registry: PI, hidden H, revealed C/F, information prices, scalar PI | yes | core, kernel, geom | LIVE (research stack; 19 mentions in CENSUS-RULINGS) |
| walt-skeleton | S4: ControlSkeleton transducer trait, §12.1 soundness + §12.6 lumpability checkers | yes | core, kernel, geom, strat | SCAFFOLD/dormant — S4 era closed (negative result); 3 doc mentions, all historical |
| walt-factory | Corpus generation, census pipelines, synthesis loops, certificate emission; the regret walker | yes | core, kernel, geom, strat, skeleton | LIVE (research stack); owns 8.8 GB of results |
| walt-gpu-spec | M0: U256 ABI, role/frame types, generated semantic tables, sha256 | yes | core | LIVE — load-bearing for GPU branch AND player spine |
| walt-gpu-ref | M1: portable scalar reference projector + parity carrier | yes | core, kernel, gpu-spec; dev-deps ../../rob/crates/{core,verify} (CROSS-TREE) | LIVE (GPU branch) |
| walt-metal | Metal binding/runtime for the M2 parity gate (objc2) | yes | gpu-spec, gpu-ref | LIVE (GPU branch); PLAN.md's denial of its existence is stale |
| walt-m2-runner | CLI boundary for the supervised freeze-56 M2 Metal gate | yes | gpu-ref, gpu-spec, metal | LIVE inferred, NOT CONFIRMED (zero doc mentions; ci/check_m2_metal.sh exists) |
| walt-m3-carrier | Frozen hand-8 receipt carrier, freeze-57 M3 gate; two constructors must agree byte-for-byte | yes | core, kernel, gpu-spec | LIVE — the player spine's data source |
| walt-m3-probe | THE SEAT SOLVER — sampling-stack machinery of SCENARIO-PLAYER.md | yes | core, m3-carrier | LIVE — this is walt |
| walt-m3-metal | Host-side admission boundary for the freeze-57 M3 Metal ABI | NO (nested workspace) | — | SCAFFOLD — WIP 97ce321; builds only in isolation; zero doc mentions |
| walt-m3-net | Production semantics for the M3 perfect-recall net | NO | core, gpu-spec | SCAFFOLD — DOES NOT BUILD |
| walt-m3-oracle-a | M3A independent H-authority adapter (carrier → fresh ScalarHidden solve) | NO | core, geom, strat, m3-carrier | SCAFFOLD — DOES NOT BUILD |
| walt-wasm | Browser decision oracle for plunge; ships pkg/walt.wasm (302 KB) + walt.ts | yes | core, m3-probe (default-features=false) | LIVE |
| walt-wasm-spike | Feasibility spike: "does level-1 fit on a phone?" | yes | core, m3-probe | SCAFFOLD — superseded by walt-wasm |

## 2. Dependency arrows

    walt-core        -> (nothing)
    walt-kernel      -> core
    walt-geom        -> core
    walt-strat       -> core, kernel, geom
    walt-skeleton    -> core, kernel, geom, strat
    walt-factory     -> core, kernel, geom, strat, skeleton

    walt-gpu-spec    -> core
    walt-gpu-ref     -> core, kernel, gpu-spec   [dev: rob-core, rob-verify — CROSS-TREE]
    walt-metal       -> gpu-spec, gpu-ref
    walt-m2-runner   -> gpu-ref, gpu-spec, metal

    walt-m3-carrier  -> core, kernel, gpu-spec
    walt-m3-probe    -> core, m3-carrier
    walt-wasm        -> core, m3-probe
    walt-wasm-spike  -> core, m3-probe

    walt-m3-net      -> core, gpu-spec                 [orphan, unbuildable]
    walt-m3-oracle-a -> core, geom, strat, m3-carrier  [orphan, unbuildable]
    walt-m3-metal    -> (none)                         [nested workspace]

**The structurally important fact:** the live player stack never
touches geom, strat, skeleton, or factory. `walt-m3-probe` reaches
core and m3-carrier only. The research stack and the player stack
share nothing above walt-core/walt-kernel. `walt-gpu-spec` is the one
genuine crossover (m3-carrier uses it for sha256), and
`walt-m3-oracle-a` is the only thing that would have bridged player
and research — and it doesn't build.

## 3. Binaries

`walt-m3-probe`: default bin (main.rs — exact lawful best-response
solve of the frozen M3 carrier) plus eleven in src/bin/:

| bin | what it drives | cited by |
| --- | --- | --- |
| ladder | walks hand 8's exact solve backward from the trick-4 boundary to find where exact enumeration dies | 48× CENSUS-RULINGS + PLAN, SCENARIO-PLAYER, ECONOMY-SUCCESSOR, FIBER-PROBE, PREDICTIVE-RANK, GPU-NATIVE-TRICK1, SEPARATION-RUNG-N4, LOG |
| scenario | level-0 scenario solver (samples deal + frozen per-scenario field seed) | CENSUS-RULINGS, TILT-AUDIT, LOG, LEVEL2-PROBE |
| level1 | the level-1 solver — best response vs level-0 field; the champion-beater | SCENARIO-PLAYER |
| level2 | field seats become level-1 minds | SCENARIO-PLAYER |
| divergence | level-2 divergence miner (shadow a seat, log disagreements) | CENSUS-RULINGS, TILT-AUDIT, LOG, SEPARATION-RUNG-N4 |
| bidcurve | P(make b) per (declaration, bid) over common random worlds | SCENARIO-PLAYER, LOG |
| tiltaudit | the E0 tilt audit (q, τ, g, H) under SP-A1..A12 | TILT-AUDIT, LOG, LEVEL2-PROBE |
| playout | full-game JSON for the walt viewer | DEADNESS-PROBE, CENSUS-RULINGS, LOG |
| walt_bridge | subprocess bridge (rob_bridge line protocol) — walt in the mk5 arena | SCENARIO-PLAYER, LOG |
| webtable | localhost HTTP table: auction + trump naming + level-1 play | SCENARIO-PLAYER, LOG |
| playtable | terminal table, one human + three level-1 seats | SCENARIO-PLAYER |

Elsewhere: `walt-factory/src/bin/walk_corpus.rs` (resumable corpus
walk); `walt-m2-runner` (M2 Metal gate CLI); `walt-wasm-spike` (the
spike timer). Plus **24 `walt-factory/examples/*.rs`** — the actual
producers of everything in results/.

## 4. Receipts / computations inventory

- **walt/receipts/** — 756 KB, 6 files. KEEP VERBATIM; the only true
  receipts here (gate0 2026-08-16; m0_m1_v1 bins + summary; m2 metal
  parity bin + sha256). Regenerated and byte-checked by ci/check.sh.
- **walt/probes/** — 1.9 MB. Mostly NOT walt output: `exp3a/` and
  `exp5/` are the rescued Python probes — PLAN.md-designated
  first-line external certificate checkers, i.e. **input authority,
  not recomputable walt results**. `bidcurve/` (three calibration
  logs, ~750 KB, + predeclared analysis) and
  `tilt_arena_2026-08-19.log` ARE walt output. Transient to sweep:
  `probes/bidcurve/__pycache__/` (the D15 trap analog).
- **walt-factory/results/** — 8.3 GiB, 67 files. THE ARCHIVE PROBLEM.
  One file is 96% of the bulk: `second_rung_frontier_2026-08-14.txt`
  at 8.2 GiB (companion to a 2.9 MB summary;
  CENSUS-RULINGS:9359 cites it under ci/check.sh PASS). Everything
  non-frontier totals under ~50 MB. Producers map cleanly by filename
  stem to `walt-factory/examples/*` (census_run, deadness_probe,
  economy_run, falsification_run, fc_correlation, feature_fee,
  fiber_probe, fusion_tax, label_transfer_run, laydown_probe,
  lesson_run, policy_geometry, predictive_rank, rule_economy_n4,
  second_rung, seed_survey, separation_probe, trick1_draw) and the
  walk_corpus bin.
- **walt-factory/store/** — 514 MB: `endgame_l2.store` (499 MB memo
  store, producer fiber_probe), `candidate_library.txt` (14 MB,
  cited by digest in ECONOMY-SUCCESSOR:204, SEPARATION-RUNG-N4:183,
  CENSUS-RULINGS:4377/:4706), two disposable resume checkpoints.
- **The recompute-command template already exists**:
  `fiber_probe.rs:1398` emits its own cold-regenerate command into its
  output (filed as E-A17):
  `rm -f walt-factory/store/endgame_l2.store && cargo run --release -p walt-factory --example fiber_probe endgame`.
  Every recompute-queue entry should take this form.

## 5. Recommended fold plan

**Fold into the unified walt crate (as modules):** core → rules,
kernel, geom, strat, gpu-spec → spec, m3-carrier → carrier, m3-probe →
solver (bins as one `walt` binary with subcommands, or kept in
src/bin/). The live spine's import direction is already strict and
acyclic (v0.4 §16.2) — the fold is mechanical.

**Keep separate, deliberately:**

- `walt-wasm` — needs cdylib crate-type, default-features=false to
  compile rayon out, its own build.sh/smoke.mjs/pkg pipeline. Folding
  it forces the feature gymnastics onto the whole crate.
- `walt-metal` + `walt-m2-runner` — merge into each other; the
  objc2/Metal stack and unsafe_code="deny" posture (vs "forbid"
  everywhere else) shouldn't leak into the unified crate.
- `walt-gpu-ref` — judgment call: its dev-deps reach across into
  `../../rob/crates`; folding it makes the unified walt's test build
  depend on the rob tree. Recommend keeping it with the Metal pair.

**Archive — delete code, keep artifacts, queue recompute:**

- `walt-skeleton` — S4 era closed. If factory folds in, skeleton's
  checkers come along as a submodule (factory depends on them); if
  factory archives, skeleton goes with it.
- `walt-factory` — THE BIG DECISION. Recommendation: archive the
  examples, keep the results, queue every artifact for recompute; the
  census era is closed and the level-2 program doesn't call into it.
- `walt-wasm-spike` — pure archive; question answered, walt-wasm
  shipped. Delete.
- `walt-m3-net`, `walt-m3-oracle-a`, `walt-m3-metal` — archive all
  three: don't build, zero citations, own commit message says freeze
  57 authorizes no implementation result. Already preserved at
  97ce321. No artifacts to keep.

**Archive-and-queue mechanics:** move results/ and store/ out of the
crate tree to `walt/archive/<date>/`; one queue entry per artifact:
(a) path + digest, (b) exact regenerate command in E-A17 form, (c) the
commit that last built its producer. Special-case the 8.2 GiB
frontier: archive the 2.9 MB summary + recompute command and drop the
body IF nothing cites frontier rows individually — that single call
reclaims 96% of the bulk. (Verify the citation condition before
acting.)

## Flags

1. **CI coverage gap.** `ci/check.sh:308` runs the no-float grep over
   exactly ten crates; the ENTIRE live player stack (m3-carrier,
   m3-probe, wasm, wasm-spike) is outside that grep (TOML grep list
   at :313 shorter still). Workspace clippy still denies
   float_arithmetic via their own manifests, so the guarantee holds,
   but the greps were never extended. Unification is the moment to
   fix it.
2. **PLAN.md is stale** on the GPU branch ("There is no walt-metal
   crate while Gate 0 is closed" — walt-metal exists; Gate 0 receipt
   dated 2026-08-16) and its crate map predates every m3-* and wasm
   crate.
3. **walt-m2-runner status is inferred, not confirmed** — trace
   whether ci/check_m2_metal.sh still runs in any active gate before
   deciding its fate.
4. **walt/viewer/** (two HTML files) belongs wherever playout/webtable
   land — sibling asset directory, not src/.
5. **walt/math/** (18 files) is source-of-truth mathematics, untouched
   by any fold.
6. **BLOCKER — the freeze-56 source closure pins the crate layout by
   path.** The census §5 fold plan was written without accounting for
   `ci/verify_m2_sources.sh`, which `ci/check.sh` runs as both its
   second and its final phase. See "Execution (2026-08-24)" below; the
   fold cannot proceed on a code-motion mandate alone.

## Execution (2026-08-24)

Status: **BLOCKED, no code motion performed.** Exploratory tier; this
section records a survey and a blocked gate, and changes no result's
status. Sources: direct execution of `ci/verify_m2_sources.sh` at
`114bacd`; digest audit of both source manifests against the worktree.

### Why the fold stopped before Stage 1

`ci/check.sh:257` and `:340` both run `ci/verify_m2_sources.sh`, which
enforces the freeze-56 cumulative source closure
(`math/gpu_native_trick1_m0_m2_sources_v1.sha256`, 381 entries). Three
of its enforcements collide with the mandate:

1. **Every manifest path must still be a regular file at that exact
   path** (`require_regular_relative`, invoked at :200 on each entry).
   Deleting `walt-factory` removes 117 pinned entries and
   `walt-skeleton` removes 12.
2. **`package_roots` (:64-78) hardcodes** `walt/walt-core`,
   `walt/walt-factory`, `walt/walt-geom`, `walt/walt-kernel`,
   `walt/walt-skeleton`, `walt/walt-strat` among others, and
   `require_directory_relative` fails if any is not a real directory.
3. **The immutable 184-path check (:212-229) is the hard one.** Every
   path in the M0/M1 manifest must remain represented in the cumulative
   closure, translated `walt/`-relative → repository-relative. That
   manifest's bytes are pinned by `ci/verify_m2_history.sh:63`
   (`eccf0a37…`) and re-derived from blobs at a fixed parent commit, so
   it cannot be edited. It names 14 files under `walt-core/`, 11 under
   `walt-kernel/`, and 7 under `walt-gpu-spec/` — the three crates the
   fold moves into `walt/walt/src/{rules,kernel,spec}/`. After the move
   those paths do not exist, and the closure fails with "historical
   source path omitted."

Enforcement 3 is mechanical, not a judgment call: **no regeneration of
the cumulative manifest can satisfy it**, because the requirement is
path presence and the pinned side of the comparison is immutable.
Satisfying the fold requires amending `verify_m2_sources.sh` itself
with an explicit old-path → new-path translation table — an amendment
to a freeze verifier, which is a freeze-level ruling and not within a
code-motion mandate.

### Second-order cost: regenerating the manifest is not bookkeeping

The manifest's exact bytes *are* `M2BuildIdentityV1`
(`verify_m2_sources.sh:3`; GPU-NATIVE-TRICK1-M2.md §11:898). That
identity is hashed into the M2 receipt records
(GPU-NATIVE-TRICK1-M2.md:393, :400, :830) and occupies bytes 64-95 of
the receipt (:566). Re-pinning the closure therefore changes the build
identity that the committed
`receipts/gpu_native_trick1_m2_v1/m2_metal_parity_v1.bin` (421 KB)
attests to. `ci/check.sh` does not byte-diff that receipt — the closure
deliberately excludes it (:192) — so this does not by itself turn
check.sh red, but it does mean the standing M2 Metal parity evidence no
longer corresponds to the sources it names. Whether that is a
re-issue-and-rerun event for `ci/check_m2_metal.sh` (614-task carrier,
twice, on hardware) is a freeze-56 ruling for Jason, per
GPU-NATIVE-TRICK1-M2.md:4 — "SHA-256 is named by the append-only
freeze-56 ruling."

### Pre-existing: the gate is already red at `114bacd`

Independently of the unification, the closure is broken in 71 places
before any change of ours:

- **6 digest mismatches.** `lean/Texas42.lean`,
  `walt-strat/src/lib.rs`, and `walt-strat/src/hidden_scalar.rs` were
  all changed by **97ce321** ("WIP: M3 perfect-recall net scaffolding,
  mid-flight, does not build"), which is an ancestor of this branch;
  `lean/Texas42.lean` at `97ce321^` hashes to exactly the frozen
  `25480c8b…`, confirming that commit as the origin. `CENSUS-RULINGS.md`,
  `Cargo.toml`, and `Cargo.lock` drifted through ordinary later work.
- **65 missing files**, all `walt-factory/results/*`, from the
  2026-08-24 archive move to `~/data` (uncommitted working-tree
  deletions at the time of survey). Every one is a pinned closure
  entry.

The code itself is healthy: `cargo test --workspace --release` at
`114bacd` exits 0, no failures. What is red is the freeze machinery
around it, not the crates being reorganized.

Note the internal tension this exposes: `CENSUS-RULINGS.md` is
append-only and pinned by *full* digest in the source closure, while
`verify_m2_history.sh:150` checks only its frozen *prefix*. Any append
to the rulings log breaks the source closure. That implies the closure
was designed to be re-pinned as work proceeds — but §11's coupling of
the manifest bytes to the receipt-bearing build identity gives
re-pinning a cost that routine appends should not carry. Recording as a
conflict, not resolving it.

### Scope table for the eventual ruling

Closure exposure per planned action, counted against both manifests:

| action | freeze-56 entries | immutable-184 entries |
| --- | --- | --- |
| fold walt-core → `rules` | 14 | 14 |
| fold walt-kernel | 11 | 11 |
| fold walt-gpu-spec → `spec` | 7 | 7 |
| fold walt-geom | 9 | 0 |
| fold walt-strat | 18 | 0 |
| fold walt-m3-carrier → `carrier` | 0 | 0 |
| fold walt-m3-probe → `solver` | 0 | 0 |
| delete walt-factory | 117 | 0 |
| delete walt-skeleton | 12 | 0 |
| delete walt-m3-{net,oracle-a,metal} | 0 | 0 |
| delete walt-wasm-spike | 0 | 0 |

`walt/ci/check.sh`, `walt/Cargo.toml`, and `walt/Cargo.lock` are
themselves pinned entries, so even the Stage 1 manifest-and-CI edits
touch the closure. `walt/PLAN.md` is **not** in the closure — its
retirement is closure-clean.

**The closure-clean subset** of the mandate is therefore: retire
`PLAN.md`, and delete `walt-m3-net`, `walt-m3-oracle-a`,
`walt-m3-metal`, and `walt-wasm-spike` (all four postdate freeze-56 and
carry zero pinned entries). Everything else — the seven-module fold,
the `walt-factory`/`walt-skeleton` deletions, and the flag-1 grep
extension in `ci/check.sh` — waits on the ruling. This subset was left
undone rather than committed piecemeal: with `check.sh` unable to reach
green, small-green-commit staging has nothing to certify against, and
splitting the deletions across two rulings costs more than it saves.

### What a ruling needs to decide

1. Is freeze-56 re-issued at the post-fold layout — new manifest, new
   `M2BuildIdentityV1`, `package_roots` and `required_paths` rewritten?
2. Does `verify_m2_sources.sh` gain an old-path → new-path translation
   table so the immutable 184 stay represented after the fold, and does
   that table count as amending the freeze or as reading it correctly?
3. Does the standing M2 Metal parity receipt get re-earned on hardware,
   or is it explicitly carried forward as evidence-for-the-old-layout?
4. Independently of the fold: is the 97ce321 drift repaired (revert the
   three files) or absorbed into the re-issue, and are the 65 archived
   `results/` entries dropped from the closure as the archive ruling
   implies?
