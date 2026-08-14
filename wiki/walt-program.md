# walt — the program, its resets, and its method

[Home](Home.md) · owns: what the walt program is trying to do, every direction
reset it has taken and why, and the working method that governs how it builds ·
Sources: [`walt/PLAN.md`](../walt/PLAN.md), [`walt/LOG.md`](../walt/LOG.md), the
frozen bases under `walt/math/`, and the adjudication record in
`walt/CENSUS-RULINGS.md`. Related: [walt hub](walt.md),
[negative results](walt-negative-results.md),
[foundation era](walt-foundation-era.md), [factory era](walt-factory-era.md),
[census era](walt-census-era.md), [S6 era](walt-s6-era.md),
[decision-sparse](walt-decision-sparse.md), [instruments](walt-instruments.md),
[math reference](walt-math-reference.md), [lineage](lineage.md).

> **Epistemic tier: EXPLORATORY — below every tier on
> [Home](Home.md#evidentiary-tiers--never-promoted-never-blurred).** Everything
> under `walt/` sits on walt's own frozen exploratory bases. Nothing here may be
> quoted in a brief, a dispatch, [FINDINGS](FINDINGS.md), or any claim-tier page.

## The question walt exists to answer

[rob](rob.md) answers *what is exactly true* — the exact solver, receipts and
all. **walt is the seat**: the full-hand imperfect-information player that has to
act from one chair, seeing only what a chair legally sees. The name is continuity
rather than coincidence. In the predecessor project mk5, walt was the exact
four-tile endgame information-set solver — the first artifact that provably had a
plan and cashed it ([lineage](lineage.md)). This walt is that idea attempted at
full scale.

The obstacle is stated precisely on [the game page](game-of-42.md): a seat's
knowledge of the three hidden hands is exactly representable and cheap to count,
but knowledge is not enough to play by — the 90-world witness proves two
positions with identical exact knowledge can demand opposite play. So the seat
needs something else to carry, and the program is a sequence of increasingly
sharp guesses about what that something is.

Two commitments have never moved. **Math first**: nothing is built before it is
adjudicated, and every construction goes to a walt-math consultant as a design
document before a line of code. **Exactness**: integers and rationals only, no
floating point anywhere near a value, caps that exclude rather than sample, and
every declared stop printed in the artifact that reports the run.

## The resets

Six directions in five days. Each was a deliberate call by Jason after a result
came in, and each is recorded here with what forced it, because the sequence is
the actual shape of the program.

### 1. Freeze the basis and build the skeleton (2026-08-09)

The opening decision: freeze the mathematical basis at v0.4, build greenfield
Rust against it, and carry the **dynamic control skeleton from the jump** — a
descriptor is a transducer, state plus a closed update law, never a labeling. The
factory would grade every candidate on two axes, soundness for a response target
and lumpability of its update, both exhaustively checkable on finite kernels. If
no nontrivial lumpable skeleton existed on honest domains, that was to be a
reportable result rather than a failure. Delivered as
[the foundation era](walt-foundation-era.md); the first synthesis run returned
exactly the reportable negative the design had anticipated.

### 2. Conflict-driven lesson learning as the outer loop (2026-08-10)

With typed conflict objects coming out of both checkers, the factory's outer loop
was organized CDCL-style: harvest failure, generalize it, prune with it. This was
imported as a *stance*, not an algorithm — from the one community that made
exhaustive search industrial and whose safety culture (proof logging, independent
checkers, "never trust the solver") independently evolved this project's own
receipt discipline. The declared regime was few conflicts deeply analyzed, never
industrial throughput. Delivered as [the factory era](walt-factory-era.md).

### 3. The re-tethering: the lossless count-free equivariant quotient (2026-08-10, evening)

The hinge. After a fresh full read of the 3,820-line basis, Jason's diagnosis was
that the build had come untethered from the mathematics. Three findings supported
it. The basis had already ruled worldwise perfect-information classes the wrong
carrier for the hidden decision, so the label-fragility result had re-confirmed a
boundary drawn in ink. The basis already named the dynamic predictive quotient as
the target. And the lumpability instantiation compared observation and feature
alphabets **raw**, which is exactly why only world-reconstructing descriptors
could pass — the mathematics quotiented the state side but never the interface
alphabets.

Jason authored the missing theorem in session: **§12.6A, equivariant controlled
lumpability**, opening the v0.5 track and leaving v0.4 frozen. Two situations are
the same when, given what the seat knows and does not, one policy applied through
declared transports to any matching world produces the same outcome under the
quotient. The goal was restated as the **lossless count-free equivariant
quotient**, and the factory and economy infrastructure was frozen until the
compression question moved.

Two policies date from this session and still bind. The **no-rescue policy**: a
failure is a counterexample to carry back to the math, never a thing to fix,
spin, or engineer around. And its corollary on verification: verify against the
reference we have rather than in triplicate, and when independent mechanical
verification is genuinely needed the path is Lean, not Python — which retired the
planned Python checker and left the lesson economy's triggered deletions
mechanically blocked, safely, by design.

### 4. The bar, and the migration of its object (2026-08-10 through 2026-08-11)

With the goal restated, Jason declared the bar: **show or disprove that the count
of canonical situations is reasonably small, order 10^5** — either outcome a
result. The bar's *object* then moved three times as measurement came in, and
following that migration is the clearest way to read [the census
era](walt-census-era.md).

| The object the bar was tested against | What happened |
|---|---|
| World-level trick-six roots | Structural quotients merge none of 647; the retrograde quotient reaches 306 |
| World-level trick-five roots | Compression *weakens* going earlier, 1.25:1; the inventory is unconverged; trick-1 world-level roots extrapolate astronomically |
| The **seat**-level census at the first play | The bar's true object, per Jason's clarification: the situations facing the leader of the first hand |

The seat-level question was then answered **by proof rather than by
enumeration**, and answered no: the first-play structural quotient is the
identity, so the count is exactly C(28,7) = 1,184,040, about 11.84× over the bar.
The insight that came with it reframed the program —
[structural compression is bought with deadness](walt-negative-results.md), and
nothing is dead at the first play.

### 5. Predictive algebra: the linear escape, and its refutation (2026-08-12)

Jason brought a new mathematical track, v0.6: exact predictive rank over the
rationals. The escape it offered from the rigidity result is real in principle —
linear rank can sit far below partition-lump size, so every behavioral row can be
distinct while the rank stays small. Adjudication immediately converted most of
the program into theorems (any closure with a constant in its terminal seed is
degenerate at full dimension, because complete records determine worlds), leaving
one measurable object. That object was measured, and the pre-declared payoff
criterion was **refuted**: the value closure saturates by grade three.

### 6. Similarity of outcomes, then decision-sparse exact solving (2026-08-12 to 2026-08-13)

Jason's response to the refutation redirected rather than retreated: his actual
hope was never exact low dimension but **similarity of outcomes** — that a hand
tends to land near the same place regardless of the path. The exact, adjudicable
fragment of that is the dual policy geometry, and measuring it produced the
program's most suggestive result: at seven of nine measured grade-3 pairs a
single policy weakly dominates every lawful alternative in all 1,680 worlds,
against raw plan counts up to 2^19930, while the two exceptions explode exactly
where the 42 is genuinely tense. The dissent travels with the number: those two
exceptions blew past the declared frontier cap, a partial frontier bounds
nothing, and **the probe's formal verdict is STOPPED with no verdict** — the
bimodality is the finding, never a seven-of-nine success. Read beside the
dimension census, **value richness and decision simplicity coexist**.

That pairing is the thesis of the current track. Jason's
`decision_sparse_exact_solving_v0.1` moves the target from compressing truth to
**proving the root action**: sandwich a lawful lower witness against an
action-conditioned upper witness and certify the opening play without solving
every action exactly. It has been filed verbatim, twice audited, and its repaired
mathematics maintained in an errata beside it. Its Experiment A (deadness
detection at census scale) and Experiment E (root-action separation) are
complete; the rest is [the live track](walt-decision-sparse.md).

## The working method

The disciplines below are not house style; each was adopted after something went
wrong, and several are enforced in Rust's type system rather than by convention.

**Adjudicate before building.** Every probe is written as a design document —
question, construction, receipts, declared stops, and the criterion that would
count as failure — and adjudicated by a walt-math consultant before any code.
The rulings are binding and are recorded in an append-only file; a superseded
ruling is marked, never rewritten. This has repeatedly paid: proposed comparisons
have been ruled strawmen, proposed chains rejected in favour of a direct theorem,
proposed receipts rejected as vacuous, and in one case a proposed invariant list
had five gaps that would have produced a spuriously small count, all five caught
before the build.

**Declare the criterion in advance, and report the verdict, not the texture.**
The clearest instance is the policy-geometry probe, whose formal verdict is
STOPPED with no verdict even though seven of its nine measured pairs collapsed to
a singleton.

**Caps exclude, they never sample.** A budget cap or fiber cap removes work from
scope and the excluded set travels with every result; a sampled basis is always
marked as sampled and never silently upgraded. Unmeasured is never zero.

**Grades and labels travel with every verdict.** A verdict is worthless without
the operator pair and weighting it was measured at, because equivalence is
label-relative; a lesson never quotes above its grade, and a worldwise *loss*
exports guarantee-negation while a worldwise *win* never exports a guarantee.

**Determinism is declared, not hoped for.** Numbered freezes fix seeds, orders,
formats and stop criteria; every stop criterion is a deterministic count rather
than a wall clock; contended timings are recordable but not quotable, with the
bias direction named. The register is
[the freeze table](walt-math-freezes.md).

**Application outside a verified domain is unconstructible.** A verdict's scope
is its verified domain, gated in types — the same shape as the project's rule
that an external PASS is never imported as an axiom, applied one level down.

**Probes are validators, never source.** The rescued Python suites are frozen
regression records; walt reimplements from definitions and pins against them, and
a pin detects drift in walt while conferring status on nothing.

## Where the program stands

The standing epistemic frame, unchanged since S5d: the mathematics has proven
that the object exists; it has not proven its utility. If the utility turns out
bad, that is a conversation rather than a rescue, and the instruments stay
valuable for other explorations either way.

The summary of results lives on [the hub](walt.md#where-it-stands) and the
refutations at [negative results](walt-negative-results.md); the live question is
the **economy claim**, and since EC-A13 it is two questions rather than one. The
**primal half** — whether a root-action certification still closes when the
witness it starts from is not itself an exact solve — is what the economy
successor is designed to test, seeding the witness from transported entries,
fixed tile rules and a heuristic re-key rather than from a solve. The **full**
claim, a solver that avoids exact solves, additionally requires the upper side
cheapened, which is Experiment D's territory and untouched. Never write the two
as one: a sentence claiming "the economy claim was tested" without *primal* has
over-claimed. [The decision-sparse page](walt-decision-sparse.md) carries the
state and what gates it; the forward plan lives in
[`walt/PLAN.md`](../walt/PLAN.md).
