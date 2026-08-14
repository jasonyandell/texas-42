# rob — the Rust Exact Engine

[Home](Home.md) · owns: the rob artifact — its workspace and module layout, its
binaries, the brief family that governs it, the CI gate, and the receipt
discipline · Sources: [rob/BRIEF.md](../rob/BRIEF.md),
[rob/BRIEF_SLICE_02.md](../rob/BRIEF_SLICE_02.md),
[rob/BRIEF_PLAYER_01.md](../rob/BRIEF_PLAYER_01.md); both packages' Exec §§.
Related: [verification](verification.md), [rob-slices](rob-slices.md),
[analysis](analysis.md), [lean](lean.md).

**rob** is this repository's exact Texas 42 engine, written from scratch in Rust.
It is an *executable mathematical specification with proof receipts*, not a game
server and not a product. Its job is to reproduce — independently, exactly, and
reproducibly to the byte — every number the ingest packages certify, under
invariants the packages state in prose but could not enforce in prose.

This page owns rob as an artifact: how it is built, what runs, what checks it.
Two neighbours own the things it produces. [verification](verification.md) owns
the receipt inventory and every exact integer in it; [analysis](analysis.md)
owns the probes and instruments that take a solve apart. Read this page for
*how rob works*, those pages for *what rob has shown*.

## Where rob sits in the evidence hierarchy

rob is the **bottom** of the four claim-bearing tiers on
[Home](Home.md#evidentiary-tiers--never-promoted-never-blurred), stated there as:

> 4. **rob conformance receipts** — byte-diffed Rust reproductions; `x-` prefixed
>    lines back exchange numbers. Evidence, never a status change.

This is the single most important thing to understand about rob, and it is a
deliberate design position rather than a limitation. A green receipt raises
confidence that rob conforms and that the finite claim is true as stated; it
never promotes a claim, never becomes a theorem, and is never a premise for
anything upstream. In the other direction, TRUST-01 forbids the reverse import:
an external `PASS` — from an ingest Python verifier, from an exchange dispatch,
from rob itself — is never taken into the Lean kernel as an axiom
([proof-assistant-plan](proof-assistant-plan.md)). rob sits *outside* the kernel
trust boundary on purpose, so that it can be wrong without corrupting anything
above it.

The practical consequence for anyone writing here: a rob result is always
introduced with its tier attached, in the house form used throughout the wiki —
"…**conformance evidence, not a status change** ([verification](verification.md))."

## Authority order

rob conforms to the layers above it and never resolves a specification conflict
locally.

1. The binding brief for the work in hand —
   [rob/BRIEF.md](../rob/BRIEF.md) (slice 01),
   [rob/BRIEF_SLICE_02.md](../rob/BRIEF_SLICE_02.md) (slice 02),
   [rob/BRIEF_PLAYER_01.md](../rob/BRIEF_PLAYER_01.md) (the player track).
   These fix language, layout, invariants, receipt rows, discrepancy
   resolutions, and definition of done.
2. This wiki — package provenance, merge order, discrepancy resolutions.
3. [`ingest/`](../ingest/) — the two immutable packages. Definitions, theorems,
   and claim IDs live there and nowhere else.

When a spec turns out to be internally inconsistent, rob does not pick the
plausible reading: it adds a failing or blocked test, reports the exact
conflicting passages, and continues elsewhere (BRIEF §11). rob never copies
ingest verifier code — the package verifiers are proof receipts, not source, and
translating them would destroy the independence that makes rob's agreement mean
anything.

## Workspace layout

A three-crate Cargo workspace at [`rob/`](../rob/README.md), pinned to Rust
1.95.0 by `rob/rust-toolchain.toml`, with `overflow-checks = true` in *all*
profiles including release (INV-4). Runtime dependencies are four exact-arithmetic
crates and nothing else: `num-bigint`, `num-rational`, `num-integer`,
`num-traits`. `proptest` is the only dev dependency. Every crate declares
`unsafe_code = "forbid"` and `float_arithmetic = "deny"`.

| Path | What it is |
|---|---|
| `rob/crates/core` | `rob-core` — the pure engine: rules, algebra, objective machine, support machinery. RNG-free, dependency-light, `#![forbid(unsafe_code)]`, `#![warn(missing_docs)]`. |
| `rob/crates/player` | `rob-player` — the seat-playing code: the fixed field policy σ, the exact information-set solver, rob's rolling re-solve, the demoted Monte Carlo baseline, and every line of seeded randomness in the repository. |
| `rob/crates/verify` | `rob-verify` — the receipt harness: a deterministic hand generator, an independent prose-rule trick resolver, and fifteen stage modules (`s1`–`s10`, `p1`–`p5`). |
| `rob/receipts/` | The committed expected outputs, byte-diffed in CI. |
| `rob/ci/check.sh` | The gate. Nothing is "done" until this is green. |
| `rob/inspector/` | A self-contained `file://` HTML viewer over a generated trace. Display data only. |

### `rob-core` modules

The core crate is organised in four layers that follow the mathematics rather
than any implementation convenience.

| Layer | Modules | What they carry |
|---|---|---|
| Universe | `pip.rs`, `domino.rs`, `declaration.rs`, `seat.rs`, `rules.rs` | The pip set and its permutation group, the 28-tile universe as the looped-K₇ edge set, the nine Straight declarations, seats with fixed opposite partnerships, bid forms and ordering. |
| Algebra | `algebra/mod.rs`, `suits.rs`, `order.rs`, `trick.rs`, `transport.rs` | The declaration-indexed relational algebra: called and powered sets, effective suits, the lead context, the rank ADT with `TOP` above every integer, the lexicographic trick key, actor-preserving trick resolution, and pip transports with the scored/unscored split. |
| Objective machine | `objective/mod.rs`, `events.rs`, `auction.rs`, `contract.rs`, `deal.rs`, `play.rs` | Primitive public events and private deal observations as a single source of truth; the one-round auction; contract certification and settlement; complete deal worlds; and the phase-indexed contracted-play state with certified lifecycle constructors. |
| Support | `support/mod.rs`, `cells.rs`, `count.rs`, `reduce.rs`, `normal_form.rs`, `sampler.rs`, `census.rs`, `dynamics.rs`, `symbolic.rs`, `outer.rs`, `transport_reach.rs`, `floor.rs` | Capacity cells as a derived view; the counting DP; canonical reduction; the normal-form trichotomy with its compiler, decoder and linear ternary validator; the exact count-ratio uniform sampler; the censuses; the matching-minor calculus; the symbolic trace validator; the necessary outer language; transport-aware reachability; and the x:001 floor families. |

The mathematics each of these implements is owned elsewhere in this wiki —
[declaration-algebra](declaration-algebra.md), [support-fiber](support-fiber.md),
[capacity-dp](capacity-dp.md),
[minimal-support-normal-form](minimal-support-normal-form.md),
[support-dynamics](support-dynamics.md), [reachability](reachability.md).

### `rob-player` modules

`sigma.rs` (the fixed deterministic field policy σ, points-blind and history-blind
beyond the current trick), `window.rs` (the exact-window budget formula and
`WINDOW_BUDGET`), `plan.rs` (the materialised whole-hand contingent plan ρ),
`solver.rs` (the exact information-set best-response solver, two engines,
plus a `gate` submodule exposing budget-parameterised entry points for probes),
`rob.rs` (rolling re-solve at every decision), `player.rs` (the baseline Monte
Carlo player and the utility lenses), `policy.rs` (the common-continuation-policy
trait), `rollout.rs`, `worlds.rs`, `rng.rs`, `bidding.rs` (a deliberate
placeholder, not a modelled policy), `match_driver.rs`, `observer.rs`,
`trace.rs`, `book.rs` (the contingency book).

## The invariants

Both census briefs and the player brief carry numbered invariants. The census
invariants INV-1 … INV-14 are each enforced by a named test, a lint, or a CI grep
rather than by review, and [verification](verification.md) owns that
invariant-to-enforcement map. The player invariants INV-P1 … INV-P7 are weaker in
this respect: only INV-P1 and INV-P7 have CI greps, and the rest are asserted
inside the receipt stages rather than by a test named for them. The five that
shape the code most visibly:

- **Derived views, never stored state.** Cells, the fiber, and the normal form
  are *functions* of the semantic state. Storing both authorities is forbidden;
  where a cache exists it lives outside equality and is re-derived to check
  coherence.
- **Projected equality.** Equality, hashing and serialization go through the
  projected semantic state only, so no audit artifact can leak into identity.
- **Proof-irrelevant reachability.** The certified play-state type stores no
  flag, its constructor is private, and its optional replay witness is an
  erasable audit record. There is no value that *carries* reachability around.
- **Exact arithmetic.** No floats anywhere near ranks or probabilities. Clippy
  denies `float_arithmetic`, a CI grep denies the type names outright, and
  probabilities are `BigRational` built from integer weights.
- **Vocabulary.** The outer object is a **necessary outer profile**, never the
  c-word; there is no per-domino scalar value API and no tunable leaf. CI greps
  enforce all three by identifier.

## The binaries

Fifteen binaries, all auto-discovered from `src/bin/`; there is no `[[bin]]`
section anywhere. Twelve write receipts, three do not.

| Binary | Crate | Writes |
|---|---|---|
| `verify_algebra` | verify | `receipts/verify_algebra.txt` |
| `verify_objective` | verify | `receipts/verify_objective.txt` |
| `verify_support` | verify | `receipts/verify_support.txt` |
| `verify_normal_form` | verify | `receipts/verify_normal_form.txt` |
| `verify_dynamics` | verify | `receipts/verify_dynamics.txt` |
| `verify_symbolic` | verify | `receipts/verify_symbolic.txt` |
| `verify_outer` | verify | `receipts/verify_outer.txt` |
| `verify_unreachable` | verify | `receipts/verify_unreachable.txt` |
| `verify_transport` | verify | `receipts/verify_transport.txt` |
| `verify_floor` | verify | `receipts/verify_floor.txt` |
| `verify_rob` | verify | `receipts/verify_rob.txt` (the whole player track, P1–P5) |
| `verify_player` | player | `receipts/verify_player.txt` (the baseline self-play transcript) |
| `trace_rob` | verify | `inspector/trace.js` + `inspector/trace.json` — the default inspector view |
| `trace_player` | player | the same two files, from the baseline-only match |
| `rob_bridge` | verify | nothing — an explicitly non-normative integer line protocol for seating rob against an outside opponent |

Ten of the twelve receipt binaries are a single line: they print their stage
module's `receipt()`. `verify_rob` prints five stage receipts in sequence (P1
through P5), and `verify_player` drives the baseline self-play match directly.
Either way the receipt content is a pure function of committed code — no I/O, no
clock, no environment.

## The receipt discipline

This is the load-bearing habit of the whole engine, and it is simple enough to
state in four sentences.

1. **A receipt is generated, never written.** Every file under `rob/receipts/` is
   stdout from a binary. Hand-editing one is the single most damaging thing you
   can do here, because it converts a check into a wish while leaving it looking
   green.
2. **CI byte-diffs every receipt against a fresh run.** No tolerance, no
   normalisation, no "close enough" — `diff -u` on the exact bytes.
3. **Every exhaustive count in the spec is an assertion.** Counts live in
   `#[test]` bodies in `rob/crates/verify/tests/` and in the receipt text, so a
   number can drift only by failing the suite *and* failing the diff.
4. **Exchange-sourced expectations are marked.** A receipt line whose expected
   value originates in an exchange-adjudicated result carries an `x-` prefix, and
   the receipt's second line names the ledger entries it depends on. This keeps
   the provenance of every number visible in the artifact itself rather than only
   in the wiki.

To regenerate a receipt after an intentional change, run the binary and redirect
into its own file — from `rob/`:

```sh
cargo run --quiet --release --bin verify_algebra > receipts/verify_algebra.txt
```

…and the same shape for each of the other eleven receipts. There is no
regeneration script by design: regenerating is meant to be a deliberate, per-stage act, and
the diff you then read in `git diff` is the review artifact. Then run the gate.

## The gate: `rob/ci/check.sh`

Run it from anywhere; it `cd`s to `rob/` itself. Nothing in rob is done until
this prints `rob ci/check.sh: PASS`.

```sh
rob/ci/check.sh
```

Seven steps, in order, each fatal (`set -euo pipefail`):

| Step | Command or check |
|---|---|
| 1. Format | `cargo fmt --check` |
| 2. Lint | `cargo clippy --workspace --all-targets -- -D warnings -D clippy::float_arithmetic` |
| 3. No-float grep | fails on any mention of the 32- or 64-bit float type names in `crates/**/*.rs` (INV-4) |
| 4. Vocabulary grep | fails on the three banned c-word identifier spellings for the outer object (INV-10) |
| 5. Vocabulary grep | fails on the banned per-domino-scalar and tunable-leaf identifier spellings (INV-P1, INV-P7) |
| 6. Tests | `cargo test --workspace --release` |
| 7. Receipt diffs | for each `receipts/verify_*.txt`, run the same-named binary into a temp dir and `diff -u`; prints `receipt <stage>: byte-identical` per file |

Two notes that save time. The greps scan `crates/` Rust source only — not
markdown, not receipts — so they police *identifier spellings in code*; the prose
rules they stand for are broader and are enforced by review. And step 7 keys the
binary name off the receipt filename, which is why `verify_player` is covered
even though it lives in a different crate than the other eleven receipt binaries:
binary names are unique workspace-wide.

### What the gate costs

**This is an hours-long job, not a quick check**, and knowing that before you
start one is part of using it.

Step 7 regenerates *every* receipt and diffs it, so a full run re-does all the
exhaustive work the receipts attest to. `verify_rob` dominates by a wide margin,
and its own rows say why: it re-derives a 44,722,908,161-state census
(`r_pos_census`), checks 58,609,267 solver nodes across 756 solves
(`r_sol_conservation`), and round-trips 6,001,465,196 canonical plan-book bytes
(`r_book_roundtrip`). Those magnitudes are read from `receipts/verify_rob.txt`
and are stable; the wall-clock is not yet characterised. As a dated observation:
a run on 2026-08-13 passed four hours of CPU on `verify_rob` alone and was still
going, with no completed timing recorded.

The failure mode this creates is concrete and has nearly bitten twice. The
script's output is buffered behind a long-running child, so a fresh session sees
the gate print nothing for an hour and concludes it has hung. **It has not
hung.** Before killing a quiet gate run, inspect the process tree: a
`target/release/verify_rob` whose parent is `bash ./ci/check.sh`, in state `R`
and burning CPU, is a healthy gate doing exactly what it is supposed to do.
Killing it wastes the whole run.

Two practical consequences. Don't start a full gate casually late in a session,
and don't confuse this with [walt](walt.md)'s `walt/ci/check.sh` — a
seconds-to-minutes gate of the same shape and name, which the walt pages simply
call "the gate". Two very different jobs wearing one name is how a four-hour run
gets mistaken for a hang.

## Frozen determinism values

Some numbers rob prints are not ingest numbers and never will be. They are
properties of rob's own deterministic generators, frozen at their first green run
so that any later drift shows up as a failing diff.

The named one is `FROZEN_WITH_VOIDS = 970` in
`rob/crates/verify/src/s3.rs`, the with-voids parity count for rob's generator;
the surrounding 972-prefix corpus shape *is* a hard corpus assertion, but the
with-voids count depends on which generator produced the prefixes. The other
freezes are unnamed and held purely by the byte-diff: the boundary-1 window
histogram, the solver's deterministic root values, the paired-match margins, the
contingency-book round-trip totals, and the entire 13-hand `verify_player`
transcript together with its frozen inputs (12 worlds per decision, lens Points,
player seed 7, match seed 42).

Two things follow. **Never cite a frozen value as an ingest number** — the trap
has bitten before and is recorded as such in [QUICKSTART](../QUICKSTART.md). And
**a frozen margin is a measurement, never a target**: the paired-match numbers
are frozen so that changes to them are *visible*, not so that they should go up.
Optimising against a frozen measurement is how a determinism freeze quietly turns
into a benchmark.

The paired-match row `r_mat_paired` is worth naming, because it is rob's most
widely quoted number and spent some time circulating without an address: the
mirrored-match margin cited on [Home](Home.md) and in
[QUICKSTART](../QUICKSTART.md) is a row of `verify_rob.txt`, inventoried at
[verification](verification.md#the-twelfth-receipt-the-player-track-verify_rob).
A figure quoted everywhere and sourced nowhere is exactly how a frozen
measurement drifts into being treated as a score.

## Running rob

From `rob/`:

```sh
cargo test --workspace --release          # the suite, including every count assertion
cargo run --release --bin verify_rob      # the player-track receipt, printed
cargo run --release --bin trace_rob       # regenerate the inspector trace
ci/check.sh                               # the full gate
```

The inspector is then `rob/inspector/index.html`, openable directly over
`file://`. Its honesty rule is worth knowing: the JavaScript recomputes no game
logic whatsoever — everything displayed is emitted from Rust, so the viewer
cannot disagree with the engine. [analysis](analysis.md) owns what the inspector
and the probe rigs show.

## rob, walt, and Lean

Three tracks, deliberately separated by what they are allowed to assume.

**rob is the value.** It computes what is exactly true about a position — the
fiber, the counts, the exact information-set best response. It is the ground
truth other work is measured against, and its usefulness depends on it having no
heuristics to defend.

**walt is the seat.** The imperfect-information player, built math-first on its
own frozen basis, lives at [walt](walt.md) and is **exploratory throughout** —
it is cited by nothing above the ideas tier. rob stays the exact solver; walt
asks what a seat can maintain and decide without the exact machinery. When walt
needs mechanical verification, the path is Lean rather than a new rob receipt.

**Lean is the kernel.** [lean](lean.md) is the independent verification path,
and it is independent precisely because it may not read rob's answers. rob's
receipts are evidence *for us* that a finite claim holds; only a kernel proof
makes it a kernel theorem.

## Known documentation drift

Reported here, not resolved, so that a reader who notices the mismatch knows it
is known rather than mysterious.

- **Twelve receipts, one of them easy to miss — corrected 2026-08-13.**
  `rob/receipts/` contains twelve files and `ci/check.sh` byte-diffs all twelve.
  Four pages said "eleven" until the correction landed; the uncounted one was
  `verify_rob.txt`, the P1–P5 player-track receipt, now inventoried at
  [verification](verification.md#the-twelfth-receipt-the-player-track-verify_rob).
  Two things about it are worth carrying. Its only prior mention anywhere in the
  wiki was on an ideas-tier page — a tier-4 receipt documented below every tier,
  which is the sort of inversion that hides a receipt in plain sight. And
  `verification.md`'s earlier section heading "The eleventh receipt" is *correct*
  as an ordinal: it refers to `verify_player`, which really is the eleventh.
- **The player invariants are not enforced the way the census ones are.**
  INV-1 … INV-14 each have a named test, lint or grep, mapped at
  [verification](verification.md). Of INV-P1 … INV-P7, only INV-P1 and INV-P7
  have CI greps; the others are asserted inside the P-stage receipt code, and no
  test is named for them. [verification](verification.md) does not mention the
  INV-P series at all.
- **`rob/README.md`'s status paragraph was stale** through slice 02 and the
  player track; it has been rewritten as a pointer to this page.
- **Brief-internal drift.** `BRIEF.md` §9 speaks of "all four receipt binaries"
  (there are twelve), §13 maps two support modules that were never created, and
  `BRIEF_PLAYER_01.md` §§1 and 10.2 still quote the pre-amendment window budget
  that §7 and the code amended downward on 2026-07-28. The briefs are the binding
  historical assignments and are not edited to match later work; read them with
  their own amendment notes.
- **`rob/inspector/README.md` names `trace_player` as the regeneration command,**
  but the committed trace is `trace_rob` output and `index.html` says so.
  Following the README as written would overwrite the committed rob trace with
  the baseline view.
