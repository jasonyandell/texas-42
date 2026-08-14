# walt — the imperfect-information seat

[Home](Home.md) · owns: the hub for the [`walt/`](../walt/PLAN.md) build — what
walt is, the fence around it, and the map of its pages · Sources: none upward
(this page cites; nothing above the Ideas tier ever cites it). Related:
[ideas](ideas.md), [idea-seat-context](idea-seat-context.md),
[idea-retrograde-rank](idea-retrograde-rank.md), [lineage](lineage.md),
[analysis](analysis.md), [field/](field/Home.md), [rob](rob.md).

> **Epistemic tier: EXPLORATORY — below every tier on
> [Home](Home.md#evidentiary-tiers--never-promoted-never-blurred).** Everything
> under `walt/` is exploratory: the frozen mathematical bases in `walt/math/`,
> the Rust workspace, every pin, every measured number, and every receipt-shaped
> artifact (each marked exploratory in its own header). walt's
> cross-implementation pins are **regression pins against probe records, never
> axioms** — TRUST-01 applies unchanged. Nothing under this hub may be quoted in
> a brief, a dispatch, [FINDINGS](FINDINGS.md), or any claim-tier page. Promotion
> path: independent re-verification, and per the no-rescue policy of 2026-08-10
> new mechanical verification goes through **Lean** ([lean](lean.md)); the
> preserved Python probe *records* remain frozen cross-implementation pins.

## What walt is

[rob](rob.md) answers *what is exactly true*. **walt is the seat** — the
full-hand imperfect-information player that has to act from one chair, seeing
only what a chair legally sees, built math-first on its own frozen basis. The
name is continuity: in the predecessor project, walt was the exact four-tile
endgame solver, the first artifact that provably had a plan and cashed it
([lineage](lineage.md)).

The ingest packages never name walt — implementation names are deliberately
excluded from the corpus ([package-provenance](package-provenance.md)) — so this
hub cites `walt/`'s own documents only.

New here? Read [the program and its resets](walt-program.md) first; it is the
narrative spine. For the game itself in plain language, and what any of this is
for, read [the game of 42, mathematically](game-of-42.md).

## The pages

**Orientation**

| Page | Scope |
|---|---|
| [walt-program](walt-program.md) | What walt is trying to do, every direction reset and why, and the working method that governs how it builds |
| [walt-negative-results](walt-negative-results.md) | The refutations as first-class findings — what each refuted, at what scope, and what it redirected |
| [walt-instruments](walt-instruments.md) | The inventory of what exists and can be reused: crates, solvers, probe binaries, stores, frozen artifacts, and how to run them |

**The build, era by era**

| Page | Scope |
|---|---|
| [walt-foundation-era](walt-foundation-era.md) | S1–S4.5: the rules-to-operators stack, the control skeleton, and its two exhaustive checkers |
| [walt-factory-era](walt-factory-era.md) | S5a–S5d: the conflict-driven lesson factory, the label-fragility discovery, the lesson economy, and the re-tethering |
| [walt-census-era](walt-census-era.md) | S5e–S5k: the situation censuses, the retrograde quotient and railyard, the fiber and endgame probes, and the seat census resolved by proof |
| [walt-s6-era](walt-s6-era.md) | S6a–S6l: the predictive-rank dimension census, the policy-geometry probe, the deadness detectors, the first root-action certifications, the cheap-seed and map-free rule probes, the trick-1 and lay-down theorems, and the two gluing rungs |

**Reference**

| Page | Scope |
|---|---|
| [walt-scheme-fix](walt-scheme-fix.md) | The descriptor language: what a descriptor is, how to read and write one, worked examples, and how much of §12.7 is actually built |
| [walt-decision-sparse](walt-decision-sparse.md) | The live track: the decision-sparse architecture, its objects, its audit history, and the state of its experiment program |
| [walt-math-reference](walt-math-reference.md) | The map of walt's mathematics — every named object, where its statement lives, what it binds. Siblings: [structure and transport](walt-math-structure-transport.md), [information geometry](walt-math-information-geometry.md), [decision-deadness](walt-math-deadness.md), [decision-sparse witnesses](walt-math-decision-sparse.md), [the freeze register](walt-math-freezes.md), [open questions](walt-math-open-questions.md) |

## The sources under `walt/`

| Piece | What it is |
|---|---|
| [`walt/math/`](../walt/math/) | The frozen mathematical bases, never edited: `unified_information_geometry_v0.4.md` (the ~3,800-line basis with its own §17 claim ledger), `equivariant_lumpability_v0.5.md` (§12.6A, authored by Jason), `predictive_algebra_v0.6.md`, `decision_sparse_exact_solving_v0.1.md` and its maintained errata, the second-audit record, and the derived `implementers_guide.md` (non-authoritative; the basis wins on any conflict) |
| [`walt/PLAN.md`](../walt/PLAN.md) | The forward plan: disciplines, crate map, the CDCL spine, one-line session summaries, next milestones |
| [`walt/LOG.md`](../walt/LOG.md) | The session index. Since this reorganization it carries a few summary lines per session and points at the owning page here; the full per-session records live in git history |
| `walt/CENSUS-RULINGS.md` | The append-only adjudication record — every ruling, freeze and theorem that governs a probe. Mapped by [walt-math-reference](walt-math-reference.md) |
| `walt/*.md` design docs | One per probe: `CENSUS`, `FIBER-PROBE`, `FIBER-REFINE`, `ENDGAME-STORE`, `SEAT-CENSUS`, `PREDICTIVE-RANK`, `POLICY-GEOMETRY`, `DEADNESS-PROBE`, `SEPARATION-PROBE`, plus the queued `SEPARATION-RUNG-N4` and `ECONOMY-SUCCESSOR` |
| [`walt/DISCREPANCIES.md`](../walt/DISCREPANCIES.md) | Spec-versus-reference reconciliations, same protocol as the corpus: never pick a plausible reading silently |
| `walt/probes/` | The rescued Python probe suites — frozen validators, never source |
| `walt/walt-*` | The Rust workspace, strict import direction: core → kernel → geom → strat → skeleton → factory ([instruments](walt-instruments.md)) |
| `walt/ci/check.sh` | The gate: fmt, clippy `-D warnings -D float_arithmetic`, no-float grep, release tests |

## Where it stands

The seat's opening situation space **does not compress** — not structurally
(Corollary S-rigid makes the first-play quotient the identity, so the count is
exactly C(28,7) = 1,184,040) and not linearly (the value closure saturates by
grade three, refuting the predictive-rank payoff at its pre-declared threshold).
Both failures have the same cause: 42's public-attribution structure means
complete records determine worlds. Structural compression is bought with
deadness, and nothing is dead at the first play.

What is measured instead is that the **decision** side collapses where the value
side does not. About half of mid-game free decisions are one-deviation ties;
proved one-sided detectors certify roughly a third of them with zero false
positives over 174 million calls, at a cost the probe declares contended and
therefore not quotable; and at seven of nine measured grade-3 pairs one policy
weakly dominates every lawful alternative in all 1,680 worlds. That last figure
travels with its dissent: the other two pairs blew past the declared frontier
cap, a partial frontier bounds nothing, and **the probe's formal verdict is
STOPPED with no verdict** — never a seven-of-nine success.

That pairing redirected the program from compressing truth to **proving the root
action**, and seven coordinates have now had their root action certified exactly
— three at grade 3, four more one grade deeper on real-deal hands, including one
where the objects the previous probe could not compute turned out to be
unnecessary. ("Certification" here is walt's own object, an exact separation of
one action from every competitor; it is not the D3 sense fenced on
[reachability](reachability.md), and nothing in it is identity-bearing.) The
negatives came with them and are the more useful half: at four further
coordinates it is now *proved* that **no candidate set whatsoever** separates the
pair under the standing relaxation, which is what tells the program where the
remaining work has to happen.

The live question is the economy claim, which since EC-A13 splits in two and is
never written as one thing. Its **primal half** — whether the certification still
closes when the witness it starts from is not itself an exact solve — is
answered as far as it has been asked: at every coordinate where separation is
possible at all, at trick 4, on real deals, a four-word rule certifies it. The
**full** claim, a solver that avoids exact solves altogether, additionally needs
the upper side cheapened. That side is no longer untouched — one competitor has
been excluded by an information tax rather than by an exact solve — but it is not
settled either, since the run that did it still prices the upper witness exactly
everywhere else. Any sentence saying "the economy claim was tested" without the
word *primal* has over-claimed.

Both rungs of that information tax have now been computed exactly at the depth
where walt's exact answers already exist, and that has retired the question
rather than answered it: at that depth the ladder is only two rungs long and both
its endpoints were already filed, so it can instantiate the mathematics but can no
longer decide anything about how far the architecture reaches. Progress needs a
**longer ladder** — a coordinate where a rung's value is not already known before
it is computed — which puts the first trick, and the three obligations blocking
it, back in front of everything else.

Every count above is carrier-relative, coordinate-relative, and exploratory. Full
numbers, scope caveats and dissents live on the era pages; the refutations are
collected at [negative results](walt-negative-results.md).
