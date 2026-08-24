# walt — the compression era (S5e–S5k)

[Home](Home.md) · owns: the compression program — situation censuses, the retrograde quotient and railyard, the fiber
and endgame store probes, and the seat-level census resolved by proof · Sources: `walt/LOG.md` (S5e–S5k);
the probe design docs `CENSUS`, `FIBER-PROBE`, `FIBER-REFINE`, `ENDGAME-STORE`, `SEAT-CENSUS` (retired 2026-08-24 after their probes closed; bytes preserved at `git show 2de8a05:walt/<NAME>.md` — this page and [walt-math-reference](walt-math-reference.md) are the owners of their still-true content);
`walt/CENSUS-RULINGS.md` (F1–F7, Q1–Q5, Y1–Y3, P-A1..P-A21, X-A1..X-A19, E-A1..E-A21, S-A1..S-A21);
`walt/walt-factory/results/*.txt`; `walt/PLAN.md`. Related: [walt](walt.md) (hub),
[walt-foundation-era](walt-foundation-era.md), [walt-factory-era](walt-factory-era.md),
[walt-math-reference](walt-math-reference.md), [walt-instruments](walt-instruments.md),
[walt-decision-sparse](walt-decision-sparse.md), [walt-scheme-fix](walt-scheme-fix.md).

> **Epistemic tier: EXPLORATORY — below every tier on
> [Home](Home.md#evidentiary-tiers--never-promoted-never-blurred).** Every number here comes from one Rust
> implementation on walt's own frozen v0.4/v0.5 basis, below the corpus, kernel, exchange-adjudicated and rob-receipt
> tiers alike. Nothing on this page may be quoted in a brief, a dispatch, `FINDINGS.md`, or any claim-tier page. The
> theorems named (Lemmas V, X, E, S, S-fold, S-det, Corollary S-rigid) are walt-math adjudications *inside* that
> exploratory basis — proved arguments, not corpus statuses and not kernel proofs; formal statements and proof
> provenance live in [walt-math-reference](walt-math-reference.md).

## The program and its migrating bar

After the 2026-08-10 direction reset the walt track had one goal: a **lossless count-free equivariant quotient** — a
descriptor `d` and transports `Θ` satisfying equivariant controlled lumpability (ECL, v0.5 §12.6A), losing nothing the
count-free dynamics reads. Jason's bar was one sentence: show or disprove that the number of count-free canonical
situations is reasonably small, **order 10^5**. The standing was explicit from the first design (`walt/CENSUS.md`,
ruling F7, the NO-RESCUE policy): **either outcome is a result**, and a failure is a counterexample carried back to
the mathematics, never a thing to engineer around.

The load-bearing thing to carry out of this era is that **the bar's object migrated**, and the answers at the three
stations are not comparable: **world-level trick-1 roots** (S5e, S5f — root-stratum class counts on carriers reachable
from the receipt kernels, the growth curve read as the bar's test); then the **SEAT-level census** (S5f's redirection
— once world-level roots were seen to extrapolate astronomically, Jason clarified that 10^5 counts situations facing
the *trick-1 leader*, one seat, against the ~399M-world fiber behind it, making the world-level class DAG substrate
rather than answer); then **answered by proof** (S5k — at the finest seat-side structural equivalence the quotient is
the identity, so the count is a theorem, `C(28,7) = 1,184,040`, about 11.84× over the bar, with no build required).

Every count below is **carrier-relative** unless stated otherwise: classes are dynamics-equivalence classes named by
their future cones on a declared carrier, need not be closed under any tile relabeling, and carry no structural
description (§12.7 compact descriptions are separate and open). They are not hidden-decision PI classes (§12.4) — the
equivalence is dynamics, not response equality. Declared stops are printed in every results file; saturation figures
are store- and order-relative.

## S5e — the situation census: r1, r2, r3 (2026-08-10, late evening)

**Question.** Does a nontrivial equivariantly lumpable `(d, Θ)` exist at all, and how much does it compress? The v0.5
track's OPEN existence item, and the first measurement against the bar.

**Method.** Design `walt/CENSUS.md`, forks F1–F7 pre-adjudicated before any code. Carrier: every situation reachable
from the 13 pip-trump trick-six receipt kernels under primitive steps, pooled — **15,253 situations, 647 roots** —
with a fixed uniform-legal field, exact rational mass 1/|L| per hidden play, a count-free contract, and the banked
increment treated as **emission, not state** (the F5 amendment; storing it would split trick-7 classes by trick-6
outcome and destroy real merges). Three declared rounds: r1 the finest structural relabeling quotient, r2 two
coarsenings, r3 Jason's retrograde direction.

**Result.**

- **r1: ECL PASS, exhaustively.** The existence item resolves YES on this domain — **11,949 classes** over 15,253
  situations, **670 cross-kernel** (the same certified situation under different hands and different trumps), but
  **zero root merges, 647/647**. The identity-interface control gives 15,253 classes: dividend 1.276 full-carrier,
  **1.000 at roots**.
- **r2: all three declared coarsenings ECL PASS.** c2 (drop the double flag) 11,380; c3 (drop beaten table-tile
  identities) 9,125; c2+c3 **8,659**, largest class 200 — and still **647/647 at roots**, dividend exactly 1.000 at
  plies 0–2. Structural matching cannot compress decision-rich states on this corpus.
- **r3: the retrograde coarsest quotient.** Backward induction from hand end over the graded carrier with
  content-addressed hereditary signatures (grade, actor offset; per move the count-free increment, the play
  classification, the successor class) and position-matching transports. Full carrier **15,253 → 1,459 classes**;
  roots **647 → 306** with 132 merging classes; the trick-7 lead target alphabet **1,275 → 63**.

| grade | ply | stratum | situations | r3 classes |
|---|---|---|---|---|
| 8 | 0 | trick-6 lead (kernel roots) | 647 | 306 |
| 7 | 1 | trick-6 mid-trick | 1,294 | 406 |
| 6 | 2 | trick-6 mid-trick | 2,056 | 360 |
| 5 | 3 | trick-6 mid-trick | 3,216 | 213 |
| 4 | 4 | trick-7 boundary (play forced) | 2,010 | 63 |
| 3 | 5 | trick-7 mid-trick | 2,010 | 63 |
| 2 | 6 | trick-7 mid-trick | 2,010 | 32 |
| 1 | 7 | trick-7 mid-trick | 2,010 | 16 |
| 0 | — | hand end | — | 1 (terminal, by ruling) |

Two receipts make those counts quotable: **r1-refines-r3 HOLDS** (each of r1's 11,949 classes lands inside exactly one
r3 class) and an **independent ECL re-check over the r3 partition PASSES** — 1,013 classes, 13,794 pairs per
condition, 0 counterexamples.

**What it taught.** Existence is settled on this domain and the retrograde direction is where compression lives — but
compression is concentrated **late**: deep strata collapse hard (2,010 → 16) while the root stratum barely moves. Next
rungs named at the time: the trick-5 climb, whose root-class growth curve is the bar's real test, and §12.7 compact
descriptions for the 306 root classes.

## S5f — the trick-five climb, and the bar's true object (2026-08-10/11, night)

**Question.** How does root-class compression behave one trick earlier? A saturation curve — grow the carrier hand by
hand in declared order, watch the new-class rate fall — was the companion probe.

**Method.** The same adjudicated r3 construction, unchanged, over grades 12 down to 0 on the 1,680-world trick-five
kernels; declared stop of 20,000,000 situations, not reached.

**Result.** Carrier **2,651,280 situations / 16,112 roots**. Roots collapse **16,112 → 12,924 classes** — **1.25:1**
against trick six's **2.1:1**, so root compression *weakens* going earlier. The class DAG by grade runs 12,924 /
27,178 / 40,938 / 37,848 / 23,592 / 11,943 / 5,393 / 1,704 / 64 / 64 / 32 / 16 / 1 — peaking inward at grade 10,
collapsing to the stable 63–64-class trick-7 alphabet. Measured inside this run, the **trick-6 lead stratum is 179,936
situations → 23,592 classes**: the trick-six census's 306 was a 647-root keyhole onto a roughly 24,000-class
inventory, carrier-relativity demonstrated exactly as the Q4 caveat warned. Receipts green again: r1-refines-r3 HOLDS
over 2,001,355 finest classes; independent ECL re-check PASS (90,003 classes, 2,489,584 pairs per condition, 0
counterexamples). A **focal-alignment caveat** is recorded — this run's focal is each hand's trick-five leader, the
earlier run's the trick-six leader — so class-identity comparisons across runs are not quoted, only counts.

Saturation: the trick-six root curve is near-flat at the end (+3 on the final hand), while the trick-five curve is far
from flat — fresh 1,680-world fibers still contributed on the order of 1,300–1,600 new classes each (h6 +1,504, h7
+1,494, h9 +1,600). The trick-five inventory is much larger than 12,924 and unconverged at 13 hands.

**What it taught.** On the record: world-level trick-1 roots extrapolate astronomically (roughly 370× per trick at the
lead stratum), so **the bar's object is the SEAT-level census** — pushed beliefs over world classes, with v0.5
conclusion 1 as the bridge. At the trick-1 lead the seat's raw space is `C(28,7) = 1,184,040` hands × declaration, and
order 10^5 up to lawful equivalence is exactly the plausible regime. The seat-level census became the next
construction, to walt-math before build.

## S5g — the railyard: pruning platform, P1, and the parts catalog (2026-08-11, small hours)

**Question.** Jason's direction: every trick is the same machine — "the railway yard stacked on itself." Derive the
yard once, prune it with the actual situation, search the remnant exhaustively. Does the machine repeat, and does the
state inventory repeat with it?

**Method.** Three walt-math-adjudicated rounds (Y1–Y3 and the shape-v2 section). Y1 fixed the one-trick contract as
the **four-primitive-step machine** with per-step interface typing, never a trick-level macro step, and typed the yard
as a *refactoring* of r3 that inherits its receipts on the measured carrier only. Y2 split Jason's periodicity claim
into **P1** (grade-freeness, a provable obligation) and **P2** (realized self-similarity, a measurement). Y3 confirmed
the pruning operator with a mandatory vocabulary discipline: the live sub-DAG is a **support** object, never a belief
— belief can only concentrate it further, never widen it.

**Result.**

- **The pruning probe.** Restricting the verified r3 class DAG by each real kernel's fiber, with class-successor
  agreement re-asserted in-run rather than assumed: trick six, median live sub-DAG **179 classes** per actual
  situation (range 28–453); trick five, median **16,782 classes / 30,812 edges** against a median 241,762 raw
  reachable situations, extremes 312:1 and 8:1. The exhaustive-search platform exists at trick five — milliseconds
  where raw walks were quarter-million-situation problems.
- **P1 discharged in code.** ONE shared routine taking no grade and no level argument reproduces r3's partition
  **byte-for-byte at every level of both rungs**.
- **The first shape notion was vacuous, and that was caught in-run.** At a lead every tile in hand is legal, so a
  level-j tree has root arity exactly j *by rule* and shapes from different levels cannot be equal; the zero
  cross-level overlap rows are a definitional consequence, flagged as vacuous in the results file and carried back to
  walt-math rather than reinterpreted — the leak is systemic.
- **The hereditary rung is degenerate for a structural reason.** Level-1 trees are forced single-leaf paths, so
  hereditary shapes *are* the r3 classes (ratio 1.000 at every level of both rungs); no recurrence exists between
  shapes and classes to locate.
- **The recurrence lives in the PARTS.** On the one non-carrier-limited step (trick five, level 1→2), v2-open depth-3
  suffix **library growth 31.6× against class growth 368.6×, ω = 1.000 at all depths** — every one of level 1's 129
  parts recurs inside level 2, and 4,071 open-depth-3 parts sit under 23,592 classes. Criterion answer on the clean
  step: **shared-machinery payoff SUPPORTED** — classes are menus over a compact shared catalog.
- **The complete level-one alphabet** (a new declared exhaustive carrier — the one number in the census with no corpus
  caveat): 55,036,800 enumerated level-one situations carry exactly **64 classes**, over a complete parts catalog of
  **21 / 57 / 129** suffixes at depths 1 / 2 / 3. The trick-five corpus realizes all 64; the trick-six corpus misses
  exactly one.

**What it taught.** The yard is real as a program (P1) and as a search platform (pruning); the state inventory
compresses through the parts, not through whole-tree shapes. Scope: one clean step, and the suffix libraries are
**instrument tier** — neither variant satisfies ECL, and no class or value claim may be read off a library size. The
t4 climb is what supplies the second clean step.

## S5h — the fiber-crush probe (2026-08-11)

**Question.** Raw fiber enumeration is quick at 4 tricks remaining, untenable at 6 (17.2M worlds), intractable at 7
(399M — the deal itself), and belief/policy iteration means countless re-evaluations. Does class-memoized evaluation
crush that cost?

**Method.** Design `walt/FIBER-PROBE.md` sent to walt-math *before* build; the rulings reshaped it hard. The proposed
raw-vs-class comparison was ruled a **STRAWMAN** — the honest control is the ordinary transposition cache the project
already banked — so the probe runs **three arms** (A0 plain tree, A1 identity-key boundary cache, B the r3-class DAG)
and *the equivariance dividend proper is B:A1, never B:A0*. Other binding amendments: the object is the **void-free
capacity fiber Φ(C₀)**, a declared cost domain and a superset of any seat's actual support, never "the seat's fiber"
(feasible is not reachable); pruning was rejected as a mechanism; the re-weighting is the **declared fold weighting
(timing instrument)** — neither support nor belief, an aggregation argument; prefix sampling was rejected for
deterministic decimation; Lemma V (per-world operator values descend to r3 classes) was stated and used as the in-run
receipt; and the amortisation claim was narrowed, with the platform claim resting on treatment H.

**Result** (receipts bit-exact everywhere, 240/240 · 24/24 · 6/6).

- **The memoisation dividend is the manyfold, and it grows with depth.** A1:A0 wall medians **0.166** (n=4), **0.024**
  (n=5), **0.010** (n=6) — 6× to 100×. The shared interior is real and compounds.
- **The equivariance dividend proper is NEGATIVE at build time.** B:A1 rung medians **4.737 / 4.297 / 4.903**
  (per-coordinate range 3.983–5.041): the class DAG computes identical values at roughly five times the cost of the
  plain cache. Root collapse on evaluated sets is nil (240 → 239–240), as the rulings predicted.
- **Interior collapse is real anyway**: at n=4 h0 a carrier of 1,502,362 situations carries 128,860 classes (11.7×).
  The inventory compresses; the build cost does not.
- **The H row is the day's surprise.** Cold treatment H — the seat's actual pooled hidden-information solve under the
  m3 dag-v1 memoized scalar solver, uniform weighting — COMPLETED on the full 34,650-world void-free fiber at every
  eligible n=4 coordinate, in roughly **7 to 17 seconds each** (6.86 s to 16.63 s), inside the declared 200M
  particle-step budget; dag-v1 did 13× to 125× less work than tree-v0. The **weighted re-solve over the fixed class
  DAG — the number the platform claim actually rests on — remains UNMEASURED**, machinery absent, stated as such per
  P-A14.
- **The P-A2 gap lines** show how far the declared cost domain sits from a real seat: at n=4 hand 8 the voided support
  is **1,200 of 34,650 = 3.4%** of the void-free capacity fiber.
- **Extrapolation** (declared one-more-step law, hand 0; labeled extrapolation, never a claim about an unrun
  computation): implied n=7 per-world A0 cost about **2,970 s** — raw is dead, as expected. A1 growth about 10.5× per
  rung on near-cold small-W samples; full-fiber saturation is a declared stop, not run.

**What it taught.** The structural reason for the negative, and it is not an implementation accident: **class identity
is a function of the future cone, so it is only computable after full expansion** — retrograde identity cannot
short-circuit descent the way a state key can. **The class store is a storage/transport object — reuse across
coordinates, hands and weightings — never a first-build accelerator.**

## S5i — the fiber-refinement probe (2026-08-11)

**Question.** Jason's hope in spirit: "everything that could happen, except X." Refine fibers by declared exclusions —
smaller fibers, not just fewer. Explicitly allowed to go nowhere.

**Method.** Design `walt/FIBER-REFINE.md`; walt-math's rulings delivered a genuine theorem, **Lemma X
(zero-contribution excision)**: under the non-negative q_trick valuation, deleting worlds whose Lemma-V value is zero
leaves the unnormalised objective *and* its argmax exactly unchanged for every information-consistent policy. It is
**one-sided** — the value-max dual forces nothing, and X_val_max was re-typed as bite-only.

The typing amendment must survive intact. Remnants are **analyst conditioning (§6.8) — a third thing besides support
and belief**. Excluded does not mean impossible (that is support) and does not mean improbable (that is belief); one
may evaluate a fixed policy on a remnant but never re-optimise over a remnant and call the result a seat value, and
excluding X never places X's falsity into any seat's information state. Predicates carry their quantifier in their
name (X_reach∃ versus X_conf∀); branch-level exclusion was rejected because it changes the operator; pass-2 economics
must be measured against the cheapest **storeless** alternative (anti-strawman); and a persistence discipline was
accepted — append-only content-addressed, collision-verified across runs, cache-never-authority, cone-intrinsic
records only, warm across coordinates but store-relative.

**Result** (n=4 all 13 hands, n=5 four hands; flag receipts at stride 97 through the independent A1 path agree
everywhere, 32–3,134 classes per coordinate; Lemma-X objective agreement asserted).

- **The machinery is essentially free.** Every predicate pass over a built store costs **0.1–3.7 ms** against
  multi-second builds (as low as 0.005 ms on the degenerate 3,053-class h3 store), while the cheapest storeless route
  for the value predicates costs **197–958 ms** — pass 2 over the store is roughly 100× to 1000× cheaper than the best
  alternative. Reachability and confinement have **no** storeless alternative at all: cone identity is not on a state
  key. Jason's multi-pass economics is measured and real.
- **The declared X's bite classes, not worlds, at these rungs.** X_val0 flags **0.1–3.5% of classes** — zero-value
  sub-cones do exist — but **0 of the 3,120 + 96 evaluated WORLDS** across both rungs, so the Lemma-X excision fired
  on nothing. Structural reading: `V* = 0` requires losing all n remaining tricks under world-informed play, which
  gets harder as n grows, and the corpus focal is the DECLARER, a strong hand by construction. X_reach∃(F0) is nearly
  vacuous at roots (you can always play badly); X_conf∀(F0) bites up to 15.8% of classes and about 0 worlds; X_val_max
  ("lay-down from here") bites a wide, coordinate-dependent share of worlds — real inventory signal, and not
  excisable.
- **The honest row**: exclusion saves nothing at evaluation time once the store is paid — remnant summation runs in
  microseconds either way. That is a result under F7.

**What it taught.** The predicate **engine** is proven and Lemma X stands as a theorem regardless of bite; what the
run found is **zero bite on the measured coordinates**. Where bite might live instead is named and left OPEN, not
claimed: count-bearing X's after role re-entry ("loses the bid"), non-declarer focal seats (weak hands lose everything
far more often), and the seat's real voided support.

## S5j — the endgame store (2026-08-11)

**Question.** Jason's direction: build from the end back and memorize it, then pathfind forward to known solutions —
same fiber, different enumeration order leveraging precomputed outcomes. Precompute only the cheap end and fill the
rest lazily. "Lots and lots of convergence late game"; walls welcome.

**Method.** Design `walt/ENDGAME-STORE.md`; walt-math delivered the day's third theorem, **Lemma E (structural
isomorphism ⇒ count-free value equality)**: equal r1 canonical forms give a tile bijection plus seat rotation carrying
one remaining game to the other, so every count-free fold is equal. It **replaces** the design's proposed r1→ECL→r3
chain, rejected because ECL is checked rather than proved and is carrier-scoped. Attribution is mandatory: this
measures the **structural transport dividend** — a symmetry-reduced tablebase — not the r3 machinery, and S5h's
negative stands unrescued. Hard scope: **count-free only**; if count re-enters every record becomes unsound and the
store is invalidated wholesale. The floor's honest competitor is closed-form last-trick resolution, not a four-ply
forced walk. First implementation of the persistence discipline (gitignored cache, cold regenerate path for every
headline).

**Result** (four-arm bit-exact equality at all 17 coordinates; 1,685 hit receipts re-expanded to terminals, all
bit-exact).

- **The tablebase arms LOSE at evaluation: 1.57×–2.69× SLOWER** than the plain A1 cache at every coordinate. The
  attribution is unambiguous — **canonicalization dominates** (about 4.6 µs per form against roughly 0.1 µs per
  state-key probe); under an A1 memo the subtree a hit saves is already collapsed, so the per-hit saving is smaller
  than the per-state canonicalization cost.
- **The convergence Jason smelled is REAL**: **830,399 form-hits** across the traversal — an aggregate hit rate of
  about **38%** of level-2 boundary probes, reaching **73%** at hand 3 — relabeling-symmetric repeats a state key
  cannot see. The negative is about *where* to spend it, not whether it exists.
- Warm cross-coordinate increment is small but nonzero and growing (n=5 h1: 8,580 warm hits against 2,966 cold). The
  store holds **1,358,231 level-2 records** after 17 coordinates with near-linear growth — **saturation NOT reached**,
  and the curve is store- and order-relative by ruling.
- **T1′ (closed-form last-trick bottom) is a real if modest win** — 0.88 to 0.99 of the T0 control, the one arm that
  beat it.
- **The floor is complete**: 55,036,800 states enumerated in 72 s, re-deriving the 64-class alphabet and byte-agreeing
  with the a1 record, and yielding the new number **32,532 distinct r1 canonical forms at level 1** — the form space
  is 508× finer than the 64-class alphabet, about 2–3 MB as a table. On the declared sample, floor-table lookup costs
  **1,430 ns against the closed-form control's 35 ns**: the floor TABLE is a **41× negative**, reported as one, and
  the closed-form control is what the arms actually use.
- Lemma E's *implementation* is validated by the receipts: the one canonical-form code path never disagreed with
  re-expansion to terminals across 1,685 samples spanning both layers.

**What it taught.** The lesson completing S5h's: cone identity cannot short-circuit descent, but **structural identity
can** — and harvesting it costs a canonicalization per distinct state, which under a plain memo exceeds the harvest at
level 2. Symmetry pays where the per-hit saving is large (deeper boundaries), where the form is cheap, or where the
object of interest *is* the form inventory. The 32,532-against-64 split and the 1.36M-record level-2 curve are the
first direct data for the seat-level census's size question.

## S5k — the seat-level census, answered by proof (2026-08-11, evening)

**Question.** The bar's actual object at last: how many situations face the trick-1 leader, up to lawful seat-side
equivalence?

**Method.** Design `walt/SEAT-CENSUS.md`, on Jason's flat-stack framing — three layer alphabets (hand forms, the
first-trick interface, the landing), counted each, **composed never**, because the nested object stays irreplaceable
for values. Adjudicated by a fresh walt-math (the predecessor retired; the rulings file is the inherited memory),
which returned S-A1..S-A21 plus three theorems and **no build needed for the headline**.

**Result.** Four adjudicated statements; formal versions in [walt-math-reference](walt-math-reference.md).

- **Lemma S (seat-side transport).** A transport of the declared seat-side structure carries every count-free censal
  question across; equal forms give equal answers.
- **Corollary S-rigid.** The pip-trump structure on all 28 live tiles has **no nontrivial self-transport**, and fixing
  the focal seat kills rotation — so the seat-side structural quotient at the first play is the **IDENTITY**, and
  **COUNT 1 = C(28,7) = 1,184,040 is a THEOREM**, missing the 10^5 bar by about **11.84×**. The design's proposed
  invariant list had five gaps that would have produced a spuriously small count; all five were caught at
  adjudication.
- **Lemma S-fold.** The seven pip declarations fold exactly **7:1** under the map sending p to p′ together with the
  unique order isomorphism on the remaining six pips. The fold is **comparison-reading-dependent**: under the literal
  §1.3 tier-0 reading only 0↔6 folds, giving 7,104,240 instead — and the bar's answer is insensitive to the choice,
  both figures being far above 10^5.
- **Lemma S-det.** Interface determination holds — the landing is a function of the declaration, the hand and the
  ordered trick-1 record — but the bounded first-trick alphabet **IS** the raw record space: no compression at the
  top.

**What it taught — the insight that reframed everything.** Structural compression in this project has always been
bought with **DEADNESS**: dead tiles, inert erased contexts. That is why level 1 gives 55M situations → 32,532 forms →
64 classes, and it is why the first play gives nothing — **nothing is dead at the first play**, no context is inert,
and the full structure is rigid. The identity quotient is not a failure of technique; it is the statement that the
abstraction level was too fine for the question. Named OPEN: whether a **coarser lawful equivalence** — dynamics-style
(needing descent, per S5h) or value-partition-style — reaches 10^5. The receipt build is PARKED; the theorems stand
without it. Jason, on the record: "I'm actually thrilled by this negative... proof that what we have doesn't represent
what I'm thinking. That's an opportunity."

## The negatives, kept first-class

Four of this era's results are negative and are treated as findings rather than as
setbacks: the class DAG is not a first-build accelerator (S5h), the endgame store loses at evaluation while the
convergence it was chasing is real (S5j), the seat-level quotient is the identity (S5k), and Lemma X's engine is
proven but had zero bite on the measured coordinates (S5i). Each is stated in full above, in its own session section,
with its attribution intact.

They are collected across eras, with what each one refuted and what it redirected, at
[walt-negative-results](walt-negative-results.md) — which owns that topic. This page owns the sessions.

## What the era left

**Instruments** (all exploratory or below, cited by nothing above them; see [walt-instruments](walt-instruments.md)):
the adjudicated r3 retrograde quotient with its refinement and ECL re-check receipts; the one grade-free yard routine
(P1) reproducing r3 byte-for-byte at every level; the support-pruning platform (median 16,782-class live sub-DAG at
trick five, exhaustively searchable); the suffix parts catalog; the complete level-one alphabet and its 32,532-form
floor; the three-arm cost ladder with its decimation and timing discipline; the declared-exclusion predicate engine;
and the persistence discipline with its cache-never-authority rule.

**Open**, as this era left it: a **coarser lawful equivalence** reaching 10^5 at the seat — dynamics-style (needing
descent, per S5h) or value-partition-style, which S5k names as the live question; **§12.7 compact descriptions**,
making a class sayable with descriptor semantics, an update law and a preservation proof, and after S5g naturally
targeting the small parts catalog rather than raw classes; **role re-entry**, declaring a nonempty output interface on
the census classes and bringing count and valuation back through the §12.6A gauge corollary; the **weighted-H re-solve
over a fixed class DAG**, still UNMEASURED — the number the belief/policy-iteration platform claim actually rests on,
machinery absent as of S5h; and the **t4 second clean step**, which would turn S5g's single non-carrier-limited parts
measurement into a curve.
