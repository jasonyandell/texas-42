# Quickstart

Orientation for a fresh session. The [wiki](wiki/Home.md) is the reference; this page
is the on-ramp. Read this, then follow links only as your task needs them.

## What this project is

Solve straight points-and-marks Texas 42 as an imperfect-information game, on
mathematics proved *before* code is trusted. Two immutable spec packages under
`ingest/` are the ground truth; everything else reconciles, reproduces, or extends
them. Why this project exists at all: [lineage](wiki/lineage.md) — the prior project
(mk5) hit "the wall" (E[Q] players that can't hold a plan), and this repo answers it
with exact information-set machinery.

## The five layers

| Layer | What it is | Touch it? |
|---|---|---|
| `ingest/` | Two immutable spec packages, **v0.7** and **rec** | **Never modify.** Each has a verifying `MANIFEST.sha256` |
| `wiki/` | Reconciled map: what's proved, at what tier, what's open | Yes — it's the living synthesis |
| `rob/` | The Rust engine: executable spec + byte-diffed receipts, the evening player v0, the HTML inspector | Yes, per its BRIEFs |
| `exchange/` | Courier channel to ChatGPT 5.6 Pro for adversarial research; hard 10-dispatch lifetime budget (8 used) | Per [pro-exchange protocol](exchange/README.md) |
| `lean/` | Lean 4 kernel formalization, just begun | Yes |

## Non-negotiables (every session, every task)

- **Never edit `ingest/`.** Discrepancies between the packages are resolved in the
  wiki ([discrepancies](wiki/discrepancies.md)), never by editing sources.
- **Evidentiary tiers are never promoted or blurred** ([Home](wiki/Home.md#evidentiary-tiers--never-promoted-never-blurred)):
  corpus statuses > proof-assistant kernel > exchange-adjudicated CONFIRMED > rob
  conformance receipts. A green receipt is *evidence*, never a status change; "PASS"
  is never imported as an axiom (TRUST-01). Label every substantive claim you write.
- **Citation convention** ([Home](wiki/Home.md#citation-convention)): **v0.7** = the
  type-discipline package, **rec** = the new-mathematics package; claim IDs like
  `CELL-14`; `x:NNN` cites an exchange result.
- **Merge rule** ([package-provenance](wiki/package-provenance.md)): rec's mathematics
  under v0.7's type discipline. Concretely: derived views, never stored cells;
  reachability proof-irrelevant (no identity-bearing certificates); say "necessary
  outer profile," never "certificate"; no floats near ranks or probabilities.
- **Exploratory stays exploratory**: [ideas](wiki/ideas.md) and
  [analysis](wiki/analysis.md) sit below every tier and are cited by nothing above
  them. Probe numbers may not be quoted as results without promotion by brief
  amendment.

## The mathematical object, in one breath

Declaration selects one of nine relational algebras over the 28 dominoes
([declaration-algebra](wiki/declaration-algebra.md) — only 3 classes for count-blind
mechanics). A viewer's exact knowledge of the three hidden hands is a capacity cell
system whose fiber is lossless ([support-fiber](wiki/support-fiber.md), CELL-05 — the
keystone), countable and exactly samplable without enumeration
([capacity-dp](wiki/capacity-dp.md)), with a globally minimal canonical normal form —
81 bits standalone, 0 bits given mechanical state
([minimal-support-normal-form](wiki/minimal-support-normal-form.md)). Play updates it
as a monotonically deleting 63-edge graph
([support-dynamics](wiki/support-dynamics.md)). Legal play reaches strictly fewer
supports than Hall allows; the exact count is the flagship open problem, boxed to
**[36,45] bits** at the exchange tier ([reachability](wiki/reachability.md), OPEN-11).
And **support is not belief**: the 90-world witness gives two legal histories with
identical support but opposite optimal leads
([belief-vs-support](wiki/belief-vs-support.md)) — the theorem that guards every
shortcut. The minimal exact decision state is the reduced viewer kernel, proved
strictly finer than the true quotient via the dead-cut lemma
([reduced-viewer-kernel](wiki/reduced-viewer-kernel.md), x:003).

## Current state (2026-07-28)

- **rob slices 01+02 green**: eleven byte-diffed receipts reproducing every ingest
  number, plus the x:001 floor family (S10). Full inventory:
  [verification](wiki/verification.md). CI: `rob/ci/check.sh`.
- **Player track P1–P5 green**: evening player v0 (fixed-field Monte Carlo best
  response on exact fiber sampling) beats baseline **net +718** over 200 mirrored
  hands (`r_mat_paired`). The inspector (`rob/inspector/`) shows every decision's
  plan tree and openings table; probes and rigs are cataloged in
  [analysis](wiki/analysis.md).
- **Exchange: all 8 dispatches adjudicated CONFIRMED** (001–008; 2 remain in the
  lifetime budget). Results table: [claim-ledger](wiki/claim-ledger.md). Headlines:
  interval [36,45] bits (x:001/006/007), no-void stratum exactly 624,892,870
  (x:008), outer language not tight + fifth condition (x:002), kernel-vs-quotient
  COLLAPSE (x:003), transport 9→3 collapse (x:004), all 19 census integers
  independently reproduced (x:005).
- **Lean begun**: `card Domino = 28` and `∑ countPoints = 35`, no `sorry`
  ([proof-assistant-plan](wiki/proof-assistant-plan.md) — the K0–K15 spine).

## The live frontier

- **OPEN-11** — exact reachable-support census; void-context strata still open
  ([open-problems](wiki/open-problems.md), [FINDINGS Q1](wiki/FINDINGS.md)).
- **The trick-3 solve wall** (~756,756-world fibers, 10–17s/decision) and the
  [hierarchical-fibers](wiki/idea-hierarchical-fibers.md) idea aimed at it (rung 1
  math is in hand; rungs 2–3 need theorems).
- **rob slice 03 targets**: reproduce x:007 (filtered census) and x:008 (no-void
  slice) in Rust; the belief/filtering layer with the 90-world regression is the
  unassigned slice 6 ([first-implementation-slice](wiki/first-implementation-slice.md)).
- **Mechanization**: Priority-0 ledger rows, the 90-world witness as a named kernel
  theorem.

## Traps that have bitten before

- rec's verifiers create `verification/__pycache__`, which then fails
  `audit_package.py`'s no-transients check ([discrepancies D15](wiki/discrepancies.md)).
- rec's executable spec contradicts rec's own math on stored state — v0.7's
  discipline controls (D1/D2).
- x:001's "exactly 559,316,142" is a grammar-subfamily count, not the no-void slice
  (D17). One exchange panel (REACH-20) was 2/3 SOUND, not 3/3 — carry the dissent.
- Frozen generator values (`FROZEN_WITH_VOIDS` 970, the `verify_player` transcript)
  are rob-internal determinism freezes, **not** ingest numbers.

For the full assessment — strongest results ranked, suspicious spots, what to build
next — read [FINDINGS](wiki/FINDINGS.md).
