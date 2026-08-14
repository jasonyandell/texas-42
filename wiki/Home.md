# Texas 42 Foundations — Wiki

The project: solve straight points-and-marks Texas 42 as an imperfect-information
game, on mathematics proved before code is trusted. Two immutable specification
packages in [`ingest/`](../ingest/) are the source of truth; this wiki is the
reconciled map over them — what is proved, at what evidentiary tier, what is open,
and what each other layer of the repo does about it. Nothing under `ingest/` is ever
modified (each package carries a verifying `MANIFEST.sha256`).

**Never played 42, or here to find out what any of this is for?** Start at
[the game of 42, mathematically](game-of-42.md) — the human-facing account of the
game, what has been proved and measured about it, and what can be done with it
now. The rest of this page assumes the vocabulary that one builds.

## The repo in six layers

| Layer | What it is |
|---|---|
| [`ingest/`](../ingest/) | Two immutable spec packages, **v0.7** and **rec** (citation convention below). All definitions, theorems, and claim IDs live here. |
| `wiki/` (this) | The reconciled synthesis: [merge order](package-provenance.md), [discrepancies](discrepancies.md), [claim tiers](claim-ledger.md), [current findings](FINDINGS.md). |
| [`rob/`](../rob/README.md) ([rob](rob.md)) | The Rust engine — an executable spec with proof receipts. Slices 01+02 green: **twelve byte-diffed receipts** reproducing every ingest number ([verification](verification.md)) — ten stage receipts, the evening-player transcript, and the P1–P5 player-track receipt `verify_rob.txt`; `rob/ci/check.sh` diffs all twelve. Plus the **evening player v0** (fixed-field Monte Carlo best response on exact uniform fiber sampling) and an HTML **game inspector** (per-seat perspectives, exact fiber counts and marginals, trump display, shareable URL state). |
| [`exchange/`](../exchange/README.md) | Courier channel to ChatGPT 5.6 Pro for adversarial research. Dispatches 001–015: 001–008 adjudicated (007/008 CONFIRMED with caveats), 009 **PARTIAL** (C1 proof chain survived 3/3; backward-commutation refutation CONFIRMED; corroboration artifacts quarantined), 010 **CONFIRMED** (R1: realizable = reachable at k=1, seed table needs no filter), the Lean thread 011/013/015 iterating without a panel (011 an honest refusal that caught a dispatch spec error and proposed the staged build; 013 Stage 1 and 015 Stage 2 both GREEN after local repair — see the `lean/` row), 012 **CONFIRMED** (carrier-skeleton staircase), 014 an informal exploratory capture (UNADJUDICATED — see the ledger's informal-captures section). Budget: quota is monthly pacing cleared with Jason per batch (fixed lifetime cap retired 2026-08-01); count 16, batch ceiling 17; ledger in the README. |
| [`walt/`](walt.md) | The imperfect-information **seat** (rob stays the exact solver), built math-first on its own frozen exploratory-tier bases under `walt/math/`. Established: the seat's opening situation space does not compress — not structurally (the first-play quotient is the identity, so the count is exactly C(28,7) = 1,184,040) and not linearly (the value closure saturates by grade three) — both for the same reason, 42's public-attribution structure. Measured instead: the **decision** side collapses where the value side does not, and the root action has now been certified exactly at three coordinates (walt's own sense of certification — an exact separation of one action from every competitor, not the D3 sense). **Everything exploratory** — the [walt hub](walt.md) has the fence and the page map; the refutations are collected at [negative results](walt-negative-results.md). |
| [`lean/`](../lean/README.md) ([lean](lean.md)) | Lean 4 + mathlib kernel formalization. **Priority-0 scoreboard: 42 of 42 rows kernel-proved** (2026-08-02) — the mechanization ledger's first-release target is closed. Spans the domino and declaration algebra through the unique trick winner, the objective hand machine and 42-point conservation, cell losslessness, the support normal form with compile/decode inverses, strategic sufficiency, and the **90-world posterior-flip witness** (PA-E10) internalized whole. No `sorry`, no `native_decide`, standard axioms only. Open: the priority-1 tiers and the PA-A12/B04 reflection targets ([proof-assistant-plan](proof-assistant-plan.md)). |

[lineage.md](lineage.md) explains the prior project (mk5), the champion, and the
wall this repo exists to answer — context only; no code or definitions cross.

## The object in one paragraph

Straight points-and-marks Texas 42 is modeled as a **declaration-indexed physical
game** plus, per player, an **imperfect-information game over hidden deals and current
hidden remainders**. The load-bearing discovery chain: a viewer's rule knowledge about
the three hidden hands is *exactly* captured by three dependent capacity cells
(pool, per-seat allowed sets, capacities); that cell support has a **globally minimal
canonical normal form** (certain tiles + a determinate/binary/ternary ambiguity core);
the set of normal forms **legal play can actually reach** is a strict subset of the
Hall-feasible ones, its exact cardinality open inside a corpus-proved 26–46-bit
interval — narrowed to **[36,45] bits** at the exchange-adjudicated tier
([reachability](reachability.md)); and **support is not belief** — two legal histories
can share the same exact 90-world support yet require opposite optimal leads under
every named utility ([belief-vs-support](belief-vs-support.md)).

## Evidentiary tiers — never promoted, never blurred

1. **Corpus statuses** — the packages' own labels ("Theorem — proved",
   "Theorem — exhaustive finite verification", …); the ground truth.
2. **Proof-assistant kernel** — the target tier; external `PASS` is never imported
   as an axiom (TRUST-01). First theorems landed in `lean/`.
3. **Exchange-adjudicated CONFIRMED** — external result; program executed ALL_PASS
   plus 3/3 adversarial referees SOUND. Not a corpus theorem, not a kernel proof.
4. **rob conformance receipts** — byte-diffed Rust reproductions; `x-` prefixed
   lines back exchange numbers. Evidence, never a status change.

Full vocabulary and the per-result caveats: [claim-ledger](claim-ledger.md).

## Citation convention

- **v0.7** = [`texas-42-foundations-source-of-truth-v0.7`](../ingest/texas-42-foundations-source-of-truth-v0.7/) — the *proof-assistant boundary revision*.
- **rec** = [`texas-42-foundations-source-of-truth-v0.7-reconstructed`](../ingest/texas-42-foundations-source-of-truth-v0.7-reconstructed/) — the *reduced play/support foundation*.
- `Math §x` / `Rules §x` / `Exec §x` = that package's `20_MATHEMATICAL_FOUNDATION.md` / `10_RULES.md` / `30_EXECUTABLE_SPECIFICATION.md`; claim IDs like `CELL-14` refer to `40_CLAIM_STATUS.md`; `x:NNN` cites an exchange result by ledger number.
- Every substantive statement carries its tier label.

## Pages

Two doorways, depending on what you came for. [**The game of 42,
mathematically**](game-of-42.md) is the human-facing account — what the game is,
what has been proved and measured about it, and what can be done with it now,
written for a technical reader who has never played. [**FINDINGS.md**](FINDINGS.md)
is the internal assessment: the full state of the mathematics, strongest results,
risks, and next questions.

### Game and algebra
- [rules-profile](rules-profile.md) — the normative Straight 42 rules (byte-identical in both packages).
- [declaration-algebra](declaration-algebra.md) — nine declarations as relational algebras; unique winner; transports; three mechanics classes.

### Exact hidden-information support
- [support-fiber](support-fiber.md) — capacity cells, the remainder fiber, the losslessness theorem.
- [capacity-dp](capacity-dp.md) — exact counting (≤512 states), Hall feasibility, exact uniform sampling.
- [minimal-support-normal-form](minimal-support-normal-form.md) — the coarsest exact support quotient; 81-bit census.
- [reachability](reachability.md) — feasible ≠ reachable; the witnesses; the [36,45]-bit interval; necessary outer profiles (never "certificates" — D3), and rec's separate symbolic-reachability construction under its own name.
- [support-dynamics](support-dynamics.md) — (rec) the matching-minor calculus; monotone 63-edge budget.

### Viewer state, belief, and value
- [reduced-viewer-kernel](reduced-viewer-kernel.md) — (rec) folded trick, reduced kernel, future-equivalence minimality, the OPEN-01 collapse.
- [belief-vs-support](belief-vs-support.md) — Bayes filtering; the 90-world posterior-flip counterexample.
- [strategic-state](strategic-state.md) — the exact decision state (c, e, β); utility lenses; quotients and gauges.

### Meta
- [package-provenance](package-provenance.md) — how the two packages relate; the authoritative merge order.
- [discrepancies](discrepancies.md) — all 16 disagreements found, with resolutions.
- [claim-ledger](claim-ledger.md) — status vocabulary; merged claim inventory; exchange-adjudicated results table.
- [verification](verification.md) — every verifier and receipt: ingest Python, rob Rust, exchange programs.
- [proof-assistant-plan](proof-assistant-plan.md) — trust boundary, K0–K15 spine, Lean status.
- [first-implementation-slice](first-implementation-slice.md) — the original slice-01 assignment (historical; rob executed it).
- [open-problems](open-problems.md) — merged unresolved claims and boundaries.
- [ideas](ideas.md) — exploratory capture of unproven directions; below every tier above, cited by nothing.
- [analysis](analysis.md) — the game-analysis hub: probes, rigs, and dashboards over the exact machinery; display/exploratory tier, cited by nothing above ideas.
- [field/](field/Home.md) — rob beyond the repo: the 2026-07-30 first-contact measurements vs the mk5 champion, lessons, and the direction map; field-measurement/exploratory tier, cited by nothing above ideas.

### The artifacts

- [rob](rob.md) — the Rust exact engine: layout, binaries, the CI gate, the receipt discipline.
- [rob-slices](rob-slices.md) — rob's build history: what each brief assigned, what each stage established.
- [lean](lean.md) — the kernel mechanization: layout, the decide-and-kernel discipline, build and extension.
- [lean-row-index](lean-row-index.md) — mechanization-ledger rows mapped to the Lean declarations that discharge them.

### walt — the imperfect-information seat (exploratory tier, cited by nothing above ideas)

Everything below sits beneath every tier above; the fence is stated on the hub.

- [walt](walt.md) — the hub: what walt is, the fence, and the map of these pages.
- [walt-program](walt-program.md) — the goal, every direction reset and why, and the working method.
- [walt-negative-results](walt-negative-results.md) — the refutations as first-class findings.
- [walt-instruments](walt-instruments.md) — what exists and can be reused, and how to run it.
- Eras: [foundation](walt-foundation-era.md) (S1–S4.5) · [factory](walt-factory-era.md) (S5a–S5d) · [compression](walt-census-era.md) (S5e–S5k) · [S6](walt-s6-era.md) (S6a–S6k).
- [walt-scheme-fix](walt-scheme-fix.md) — the descriptor language: a user guide, with worked examples and an honest account of what is built.
- [walt-decision-sparse](walt-decision-sparse.md) — the live track: architecture, audit history, experiment program.
- [walt-math-reference](walt-math-reference.md) — the map of walt's mathematics, with [structure and transport](walt-math-structure-transport.md), [information geometry](walt-math-information-geometry.md), [decision-deadness](walt-math-deadness.md), [decision-sparse witnesses](walt-math-decision-sparse.md), [the freeze register](walt-math-freezes.md), and [open questions](walt-math-open-questions.md).
