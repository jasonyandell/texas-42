# walt — the imperfect-information seat

[Home](Home.md) · owns: the map of the [`walt/`](../walt/PLAN.md) build — the
full-hand imperfect-information seat, its frozen mathematical basis, and its
conflict-driven lesson factory · Sources: none upward (this page cites; nothing
above the Ideas tier ever cites it). Related: [ideas](ideas.md),
[idea-seat-context](idea-seat-context.md),
[idea-retrograde-rank](idea-retrograde-rank.md), [lineage](lineage.md),
[analysis](analysis.md), [field/](field/Home.md).

> **Epistemic tier: EXPLORATORY — below every tier on
> [Home](Home.md#evidentiary-tiers--never-promoted-never-blurred).** Everything
> under `walt/` is exploratory: the v0.4 mathematical basis (its own §17 claim
> ledger separates prose proofs from reported finite evidence from open), the
> Rust workspace, every pin, and every receipt-shaped artifact (each marked
> exploratory in its own header). walt's cross-implementation pins are
> **regression pins against probe records, never axioms** (TRUST-01 applies
> unchanged). Nothing here may be quoted in a brief, a dispatch, FINDINGS, or
> any claim-tier page. Promotion path: independent re-verification — the
> preserved Python probes are the designated second implementation; a Lean
> checker is the long-term crown.

## What walt is

[rob](../rob/README.md) answers *what is exactly true* — the exact solver,
receipts and all. **walt is the seat**: the full-hand imperfect-information
player that has to act from one chair, seeing only what a chair legally sees.
The name is continuity, not coincidence: mk5's walt was the exact ≤4-tile
endgame info-set solver — the first artifact that provably *has a plan and
cashes it* ([lineage](lineage.md)). This walt is that idea attempted at full
scale, built math-first on a frozen basis.

Decision 2026-08-09 (Jason): freeze the basis, build greenfield Rust, **dynamic
control skeleton from the jump** — a descriptor is a transducer (state + closed
update law), not a labeling, and the factory grades every candidate on both the
soundness and the lumpability axis.

The ingest packages never name walt (implementation names are deliberately
excluded from the corpus — [package-provenance](package-provenance.md)); this
page cites `walt/`'s own documents only.

## The pieces

| Piece | What it is |
|---|---|
| [`walt/math/unified_information_geometry_v0.4.md`](../walt/math/unified_information_geometry_v0.4.md) | The frozen basis (~3,800 lines): fixed-field/revealed/hidden operator hierarchy, information prices, descriptor soundness and controlled lumpability, the synthesis loop, the §16.11 certificate schema, the §17 claim ledger. Never edited. |
| [`walt/PLAN.md`](../walt/PLAN.md) | The build plan: disciplines, crate map, the CDCL spine, one-line session summaries, next milestones. |
| [`walt/LOG.md`](../walt/LOG.md) | Full per-session build records (moved from PLAN 2026-08-10). |
| [`walt/DISCREPANCIES.md`](../walt/DISCREPANCIES.md) | Spec-vs-reference reconciliations, same protocol as the corpus: never pick a plausible reading silently. |
| `walt/probes/` | Rescued scratchpad-era Python probe suites (exp3A, exp5) — **frozen validators, never source**; walt reimplements from definitions and pins against their records. |
| `walt/walt-*` | The Rust workspace, strict import direction: core (rules) → kernel (fibers) → geom (exact rationals, envelopes) → strat (operators H/C/F, PI, prices) → skeleton (ControlSkeleton + §12.1/§12.6 checkers) → factory (walker, lessons, economy). |
| `walt/ci/check.sh` | The gate: fmt, clippy `-D warnings -D float_arithmetic`, no-float grep, release tests. PASS at every commit. |

## State (2026-08-10, sessions S1–S5c-m2)

The rules-to-operators stack is complete and pinned against every rescued
record: the 13-hand receipt replay, the trick-6 §14.2 census, the full exp4
§14.5–14.6 information-price record, the exp5 sampled censuses, and exp3A's
90 → 33 → 8 descriptor result, all reproduced exactly through walt's own
machinery. The S4 synthesis run produced the first structural finding: at that
candidate scale the only lumpable skeletons are world-reconstructing — the
genuine compression found lives in *history-forgetting* (5,887 nodes → 2,857
classes), not state-coarsening. The S5 factory walked the full 52-transcript
corpus for exact fiber-expected regret (77.5% zero-regret decisions; 25/52
transcripts worldwise-lost from some decision on), and the lesson machinery
survived its two designed falsification points: the atom vocabulary is
expressively sufficient on a discriminative domain (pure-atom implicants that
transfer across hands, rent equal to origin regret), and the lesson inventory
is **not label-fragile** — every measurable lesson survived re-measurement at
the seat-facing (H, fixed-uniform-legal) label (10 survive / 0 fail).

All of it exploratory; details and exact numbers in [`walt/LOG.md`](../walt/LOG.md).

## The spine

The factory's outer loop is conflict-driven lesson learning (CDCL as stance,
not algorithm): **conflicts** (regretted decisions, checker failures — typed,
graded, labeled) are generalized into **lessons** (implicants over atom
vocabularies, witness-terminated widening, cut refinement) that pay **rent**
(measured, purpose-specific pruning) or are deleted. The regime is few
conflicts deeply analyzed, never industrial throughput. Standing honesty
rules, enforced in types where Rust can carry them:

- grades and labels travel with every verdict — worldwise dominance > exact
  expectation > sampled, always at a named (focal-info, field) operator pair;
  basins are label-relative (v0.4 §12.4); a lesson never quotes above its
  grade, and a verified lesson never upgrades its sampled origin;
- worldwise loss exports guarantee-negation; worldwise *win* never exports
  "guaranteed" (the §7.6 strategy-fusion asymmetry);
- budget caps and fiber caps are **exclusion, never sampling** — what was
  excluded travels with every result; sampled bases are always marked;
- application outside a lesson's verified domain is unconstructible
  (DomainSpec-gated), the TRUST-01 shape at the lesson level.

## Next

m3 (in progress): pooled-state H memoization for the budget-capped big
fibers, then the lesson database as a working set priced in seat-facing H
rent — indexing, purpose-specific rent, deletion, restart-with-retention,
§16.11 certificate emission with the Python probes as independent checkers.
Then S6+: corpus at scale across all nine declarations, the dynamic skeleton
search proper, seat chassis wiring. The forward plan lives in
[`walt/PLAN.md`](../walt/PLAN.md).
