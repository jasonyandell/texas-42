# Texas 42 Foundations — Wiki

The project: solve straight points-and-marks Texas 42 as an imperfect-information
game, on mathematics proved before code is trusted. Two immutable specification
packages in [`ingest/`](../ingest/) are the source of truth; this wiki is the
reconciled map over them — what is proved, at what evidentiary tier, what is open,
and what each other layer of the repo does about it. Nothing under `ingest/` is ever
modified (each package carries a verifying `MANIFEST.sha256`).

## The repo in five layers

| Layer | What it is |
|---|---|
| [`ingest/`](../ingest/) | Two immutable spec packages, **v0.7** and **rec** (citation convention below). All definitions, theorems, and claim IDs live here. |
| `wiki/` (this) | The reconciled synthesis: [merge order](package-provenance.md), [discrepancies](discrepancies.md), [claim tiers](claim-ledger.md), [current findings](FINDINGS.md). |
| [`rob/`](../rob/README.md) | The Rust engine — an executable spec with proof receipts. Slices 01+02 green: eleven byte-diffed receipts reproducing every ingest number ([verification](verification.md)). Plus the **evening player v0** (fixed-field Monte Carlo best response on exact uniform fiber sampling) and an HTML **game inspector** (per-seat perspectives, exact fiber counts and marginals, trump display, shareable URL state). |
| [`exchange/`](../exchange/README.md) | Courier channel to ChatGPT 5.6 Pro for adversarial research. Through dispatch 012: 001–008 adjudicated (007/008 CONFIRMED with caveats), 009 **PARTIAL** (C1 proof chain survived 3/3; backward-commutation refutation CONFIRMED; corroboration artifacts quarantined), 010 **CONFIRMED** (R1: realizable = reachable at k=1, seed table needs no filter), 011 in flight (Lean), 012 **CONFIRMED** (carrier-skeleton staircase). Budget: quota is monthly pacing cleared with Jason per batch (fixed lifetime cap retired 2026-08-01); count 13, batch ceiling 17; ledger in the README. |
| [`lean/`](../lean/README.md) | Lean 4 + mathlib kernel formalization, K0–K3: domino algebra (`card Domino = 28`, covering, `∑ countPoints = 35`) and the full declaration algebra through the **unique trick winner**, proved by key injectivity, no `sorry` ([proof-assistant-plan](proof-assistant-plan.md)). |

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

Start with [FINDINGS.md](FINDINGS.md) — the full state of the mathematics, strongest
results, risks, and next questions.

### Game and algebra
- [rules-profile](rules-profile.md) — the normative Straight 42 rules (byte-identical in both packages).
- [declaration-algebra](declaration-algebra.md) — nine declarations as relational algebras; unique winner; transports; three mechanics classes.

### Exact hidden-information support
- [support-fiber](support-fiber.md) — capacity cells, the remainder fiber, the losslessness theorem.
- [capacity-dp](capacity-dp.md) — exact counting (≤512 states), Hall feasibility, exact uniform sampling.
- [minimal-support-normal-form](minimal-support-normal-form.md) — the coarsest exact support quotient; 81-bit census.
- [reachability](reachability.md) — feasible ≠ reachable; the witnesses; the [36,45]-bit interval; symbolic certificates.
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
