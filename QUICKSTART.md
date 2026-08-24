# Quickstart

Orientation for a fresh session. The [wiki](wiki/Home.md) is the reference; this page
is the on-ramp. Read this, then follow links only as your task needs them.

**New to 42 itself, or want the plain-language version of what this project knows?**
Read [the game of 42, mathematically](wiki/game-of-42.md) — the game, what has been
proved and measured about it, and what can be done with it now, written for a
technical reader who has never played. This page assumes that vocabulary.

## What this project is

Solve straight points-and-marks Texas 42 as an imperfect-information game, on
mathematics proved *before* code is trusted. Two immutable spec packages under
`ingest/` are the ground truth; everything else reconciles, reproduces, or extends
them. Why this project exists at all: [lineage](wiki/lineage.md) — the prior project
(mk5) hit "the wall" (E[Q] players that can't hold a plan), and this repo answers it
with exact information-set machinery.

## The six layers

| Layer | What it is | Touch it? |
|---|---|---|
| `ingest/` | Two immutable spec packages, **v0.7** and **rec** | **Never modify.** Each has a verifying `MANIFEST.sha256` |
| `wiki/` | Reconciled map: what's proved, at what tier, what's open | Yes — it's the living synthesis |
| `rob/` | The Rust engine: executable spec + byte-diffed receipts, the evening player v0, the HTML inspector. Artifact guide: [rob](wiki/rob.md) | Yes, per its BRIEFs |
| `walt/` | The imperfect-information **seat** — the player that acts from one chair, on its own frozen exploratory bases. Hub: [walt](wiki/walt.md) | Yes, per `kanban/` and `walt/UNIFICATION-CENSUS.md` — but **everything under it is exploratory tier**, below every tier below |
| `exchange/` | Courier channel to ChatGPT 5.6 Pro for adversarial research; dispatches authorized in batches, quota agreed per batch (count in `exchange/submission_count.txt`; batch ceiling `HARD_CAP` in `automation/submit.mjs`) | Per [pro-exchange protocol](exchange/README.md) |
| `lean/` | Lean 4 + mathlib kernel formalization — all 42 priority-0 rows kernel-proved (2026-08-02). Artifact guide: [lean](wiki/lean.md) | Yes, per [lean/PROOFS.md](lean/PROOFS.md) |

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
- **Exploratory stays exploratory**: [ideas](wiki/ideas.md),
  [analysis](wiki/analysis.md), [field/](wiki/field/Home.md) and everything under
  [walt/](wiki/walt.md) sit below every tier and are cited by nothing above them.
  Probe numbers may not be quoted as results without promotion by brief amendment.
  A walt number never appears in a brief, a dispatch, [FINDINGS](wiki/FINDINGS.md),
  or any claim-tier page.

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

## Current state (2026-08-13)

- **rob slices 01+02 green**: twelve byte-diffed receipts reproducing every ingest
  number, plus the x:001 floor family (S10). Full inventory:
  [verification](wiki/verification.md); artifact guide: [rob](wiki/rob.md). CI:
  `rob/ci/check.sh`.
- **Player track P1–P5 green**: evening player v0 (fixed-field Monte Carlo best
  response on exact fiber sampling) beats baseline **net +718** over 200 mirrored
  hands (`r_mat_paired`). The inspector (`rob/inspector/`) shows every decision's
  plan tree and openings table; probes and rigs are cataloged in
  [analysis](wiki/analysis.md).
- **Exchange: dispatches 001–015 sent** (count 16 — one double-send — against batch
  ceiling 17). 001–008 adjudicated CONFIRMED; the 2026-08-01 constellation batch:
  009 **PARTIAL** (C1 proof chain survived 3/3; pooled-key backward commutation
  REFUTED), 010 **CONFIRMED** (R1: realizable = reachable at k=1), 012 **CONFIRMED**
  (carrier-skeleton staircase); the Lean thread 011/013/015 iterated without a panel
  (Stages 1–2 GREEN); 014 an informal capture (unadjudicated). Results table:
  [claim-ledger](wiki/claim-ledger.md). Standing headlines: interval [36,45] bits
  (x:001/006/007), no-void stratum exactly 624,892,870 (x:008), outer language not
  tight + fifth condition (x:002), kernel-vs-quotient COLLAPSE (x:003), transport
  9→3 collapse (x:004), all 19 census integers independently reproduced (x:005).
- **Lean: all 42 priority-0 rows kernel-proved** (2026-08-02; last row PA-E10, the
  90-world witness internalized whole; no `sorry`, no `native_decide`, standard
  axioms only) — plus the constellation thread's self-contained Stage 1/2 files
  (x:013/x:015, not yet reconciled with the main layers).
  [proof-assistant-plan](wiki/proof-assistant-plan.md) has the scoreboard;
  [lean](wiki/lean.md) is the artifact guide.
- **walt: sessions S1–S6d complete, all exploratory** ([walt hub](wiki/walt.md);
  the arc and its six direction resets are at [walt-program](wiki/walt-program.md)).
  Established negatively — the seat's opening situation space does not compress,
  neither structurally (the first-play quotient is the identity, so the count is
  exactly C(28,7) = 1,184,040) nor linearly (the value closure saturates by grade
  three), both from 42's public-attribution structure. Measured positively — the
  *decision* side collapses where the value side does not: about half of mid-game
  free decisions are one-deviation ties, proved one-sided detectors certify roughly
  a third of them with zero false positives over 174M calls, and the root action
  has now been certified exactly at three coordinates. The refutations are
  first-class findings at [walt-negative-results](wiki/walt-negative-results.md);
  before building anything under `walt/`, read
  [walt-instruments](wiki/walt-instruments.md) so you don't rebuild what exists.

## The live frontier

- **OPEN-11** — exact reachable-support census; void-context strata still open
  ([open-problems](wiki/open-problems.md), [FINDINGS Q1](wiki/FINDINGS.md)).
- **The constellation direction** (idea tier —
  [idea-retrograde-rank](wiki/idea-retrograde-rank.md)): backward induction over
  constellations, with exchange-tier anchors C1 (x:009 proof chain), R1 (x:010),
  and the x:012 staircase; C1's Lean mechanization is iterating in the exchange
  Lean thread. The seat-level frame is captured at
  [idea-seat-context](wiki/idea-seat-context.md) (deliberately unresolved).
- **The trick-3 solve wall** (~756,756-world fibers, 10–17s/decision) and the
  [hierarchical-fibers](wiki/idea-hierarchical-fibers.md) idea aimed at it
  (round 1 complete: rung 1 priced, rung 2 falsified as stated, rung 3 realized
  via decomposable bounds — see the idea page).
- **walt's live track — the economy claim** (exploratory tier,
  [walt-decision-sparse](wiki/walt-decision-sparse.md)): the target has moved from
  compressing truth to *proving the root action* by sandwiching a lawful lower
  witness against an action-conditioned upper witness. Experiments A and E are
  complete; the open question is whether the sandwich still closes when the witness
  it starts from is **not** itself an exact solve. Note that walt's design method is
  adjudicate-before-build — every probe goes to a mathematics consultant as a design
  document, with declared receipts and a declared failure criterion, before code.
- **rob slice 03 targets**: reproduce x:007 (filtered census) and x:008 (no-void
  slice) in Rust; the belief/filtering layer with the 90-world regression is the
  unassigned slice 6 ([first-implementation-slice](wiki/first-implementation-slice.md)).
- **Mechanization**: the priority-1 tiers and the PA-A12/B04 reflection targets
  ([proof-assistant-plan](wiki/proof-assistant-plan.md) — priority 0 is closed).

## Traps that have bitten before

- rec's verifiers create `verification/__pycache__`, which then fails
  `audit_package.py`'s no-transients check ([discrepancies D15](wiki/discrepancies.md)).
- rec's executable spec contradicts rec's own math on stored state — v0.7's
  discipline controls (D1/D2).
- x:001's "exactly 559,316,142" is a grammar-subfamily count, not the no-void slice
  (D17). One exchange panel (REACH-20) was 2/3 SOUND, not 3/3 — carry the dissent.
- Frozen generator values (`FROZEN_WITH_VOIDS` 970, the `verify_player` and
  `verify_rob` transcripts) are rob-internal determinism freezes, **not** ingest
  numbers.
- **`rob/ci/check.sh` is an hours-long job, not a quick check.** It regenerates and
  byte-diffs every receipt, and `verify_rob` alone re-derives a 44,722,908,161-state
  census, checks 58,609,267 solver nodes, and round-trips 6,001,465,196 canonical
  plan-book bytes. A run observed on 2026-08-13 passed four hours of CPU and was
  still going. It is not hung — that is what the gate costs. Budget for it, and
  don't start one casually late in a session. (`walt/ci/check.sh` is a different and
  much cheaper gate; don't confuse the two.)
- **In `walt/`, the results files outrank the prose.** Several headline numbers in
  the session log disagree with the artifacts they cite — a coordinate count, a
  detector timing that appears in no results file because that run was resumed, a
  receipt-row count, a speedup ratio that does not recompute. The wiki pages use the
  file values; the seven known cases are listed at the foot of
  [walt-s6-era](wiki/walt-s6-era.md) and in the era pages. Check the artifact before
  quoting a walt number.

For the full assessment — strongest results ranked, suspicious spots, what to build
next — read [FINDINGS](wiki/FINDINGS.md). For the human-facing account of the game
and what can be done with it, read [game-of-42](wiki/game-of-42.md).
