# walt — the conflict-driven lesson factory (S5a–S5d, 2026-08-10)

[Home](Home.md) · owns: the conflict-driven lesson factory — the regret walker, the
`Lesson` type, the falsification tests, the label-fragility discovery, the lesson-DB
economy, and the 2026-08-10 re-tethering · Sources: [`walt/LOG.md`](../walt/LOG.md)
S5a–S5d, [`walt/PLAN.md`](../walt/PLAN.md) (the CDCL spine, the disciplines, the session
summaries), the run artifacts in
[`walt/walt-factory/results/`](../walt/walt-factory/results/),
[`walt/math/equivariant_lumpability_v0.5.md`](../walt/math/equivariant_lumpability_v0.5.md).
Related: [walt](walt.md) (hub), [walt-foundation-era](walt-foundation-era.md),
[walt-census-era](walt-census-era.md), [walt-instruments](walt-instruments.md),
[walt-math-reference](walt-math-reference.md) (theorems and proofs live there),
[walt-scheme-fix](walt-scheme-fix.md), [walt-decision-sparse](walt-decision-sparse.md).

> **Epistemic tier: EXPLORATORY — below every tier on
> [Home](Home.md#evidentiary-tiers--never-promoted-never-blurred).** Every number here is a
> walt-tier measurement over walt's own frozen basis and receipt corpus: not a corpus
> status, not a Lean kernel proof, not an exchange-adjudicated result, not a rob receipt.
> None of it may be quoted in FINDINGS, the claim ledger, a brief, or a dispatch. walt's
> pins are regression pins against rescued probe records, never axioms (TRUST-01).
> Vocabulary: the §16.11 *certificate* records below are walt's own record type and keep
> their name; the D3 concept is always the **necessary outer profile**.

One day built a machine that harvests refuted decisions, generalizes them into reusable
verdicts, prices them, and deletes the ones that do not pay — and then, the same evening,
established that the whole inventory had been measured at the wrong label and that the
build had drifted from its own mathematics. The era ends in two first-class negative
results and a direction reset, not in a shipped seat. Under the NO-RESCUE policy adopted at
its close, that is the intended shape of the work.

## The CDCL stance, for a reader who has never seen it

Conflict-driven clause learning is how the SAT community made exhaustive search industrial:
when a line is refuted you analyze *why*, generalize the reason into a clause forbidding a
family of similar lines, and carry it forward. walt imports the **stance**, not the
algorithm ([`PLAN.md`](../walt/PLAN.md), adopted 2026-08-10 by Jason and Claude).

A **conflict** is a refuted line: a decision with strictly positive fiber-expected regret
under *declared* continuation semantics and world weighting, or an S4 checker failure with
its witnessing pair and exact disagreeing event. A **lesson** is a generalized verdict: an
implicant — a conjunction of typed cells over decision identity and the atom vocabularies —
attached to a graded, labeled action verdict, produced by greedily dropping cells and
re-verifying exhaustively until a witness stops the widening (1UIP culture: a good cut
cheaply, never a minimal core expensively). Wins are learned too, the QBF-cube analog. The
database is a **working set, not an archive**: lessons pay rent or are deleted, application
is near-free through watched-feature indexing, search state is disposable and the lesson DB
is the asset. The rate regime is the load-bearing difference from SAT — one walt conflict
costs a real exact solve over a fiber of up to hundreds of millions of worlds, so the
declared regime is **few conflicts, deeply analyzed, maximally generalized**: lesson quality
per conflict is the metric, never throughput. The honesty rules are enforced in types, not
by review convention, which is why the negative results below could not be quietly avoided.

| Rule | How it is carried |
|---|---|
| Grades and labels travel with every verdict | `Grade` = (dominance-vs-expectation) × (operator, weighting) as *fields*; `walt-strat::label` `OperatorLabel {PI, H, C, F}`, `WeightingLabel` |
| A lesson never quotes above its grade | a sampled origin is never upgraded by a worldwise-verified lesson; sampling is always marked |
| Worldwise **loss** exports guarantee-negation; worldwise **win** never exports "guaranteed" | win verdicts render as per-world sufficiency, "never a seat-facing guarantee (§7.6)" |
| Budget caps are **exclusion**, never sampling | an over-budget solve returns nothing; the excluded count travels in every receipt |
| Application outside the verified domain is unconstructible | `lesson_applies` checks the stored `DomainSpec` first (TRUST-01 shape) |
| Unmeasured is never zero | capped lessons are held and never advance a deletion streak |

## Read the label story first

Everything measured through S5c-m2 was measured at **(C, minimax-omniscient)** — the
information-relaxed diagnostic, a best response computed separately in each hidden world
and then averaged. The seat's actual label is **(H, fixed-uniform-legal)**: one choice per
pooled information state against the whole particle set, against the §7.4 fixed
uniform-legal field. The gap is the strategy-fusion gap and it is action-specific. §12.4
makes basins label-relative, and that caveat rides every basin line the factory prints.

S5c-m2 re-measured the inventory at the seat label, found **10 survive / 0 fail / 5
unmeasured** of 15 value lessons, and concluded it was not label-fragile. **That conclusion
was SCALE-LIMITED.** The five unmeasured were four budget-capped big fibers plus one empty
basin — precisely where the labels have most room to diverge. S5c-m3 lifted the caps and got
**1 survive / 3 FAIL**, cross-validated byte-identically by an uncapped, unmemoized tree
walk. The failures are **real label fragility**, localized to big early-trick fibers, not a
memoization artifact.

## S5a — the regret walker (the conflict generator)

**Question.** Where does a seat playing a real receipt transcript go wrong, and can that be
recorded as typed, labeled conflict material rather than commentary?

**Method.** A new `walt-factory` crate over `ReceiptDecision` (the §2.1 viewer kernel at
*arbitrary* decision points, mid-trick included), `ScalarPi::action_values`, and
`walt-strat/src/label.rs` (declared knobs as data). The constructor was validated three ways
over all 13 × 4 × 7 = 364 decision points: exact `Kernel` equality with `from_receipt_trick`
at all 91 viewer-lead trick starts, the receipt's deal inhabiting the fiber everywhere,
fiber counts monotonically nonincreasing along every transcript. Per decision: an exact
fiber count, exhaustive enumeration at or below a declared threshold with recorded-seed
uniform samples above it (marked `Sampled`), exact-rational fiber expectations, the
transcript action's regret, strict worldwise-dominance pairs, and an all-actions-lose flag.

**Result, part 1** (CI config: tricks 3–7, exhaustive ≤ 40,000 worlds, 64-draw samples
above; pins in `walt-factory/tests/data/ci_corpus_pins.txt`, pip-trump-only caveat restated
there). 214 of 260 walked decisions zero-regret (82.3%); 18 of 52 seat-transcripts with zero
total regret; 162 strict dominance pairs; **11** decisions with a worldwise-dominated chosen
action across 9 transcripts; 25 of 52 transcripts earn a labeled lost verdict (9 declaring,
16 defending; earliest-walked trick t3: 7 of them). The byte-frozen fixture is hand 0 / seat
S1: total regret 10079/672, one worldwise-dominated choice (t5: 3-2 chosen, 2-1 dominates),
verdict "lost from t3 p3". **The count correction, in place:** the LOG's S5a line first read
"12 dominated"; S5b corrected it to **11**, because the committed pins sum to 11. Re-summing
that file reproduces 11 across exactly 9 transcripts, along with 214, 162, 18 and 25 — the
artifact wins over the prose, and the correction is made in place with its reason attached.

**Result, part 2 — the full walk, the kill, the resume.** The first full-corpus run died at
~22 of 52 transcripts, mid h5 S2 — memory pressure, not a fault: no panic, the last written
block internally complete, per-thread caches at 14–19 GB for a single horizon-6/7 decision.
Two provably output-neutral fixes: a per-thread 4M-entry cache bound (~4 GB; entries are
exact values of projected states, so trimming costs only recomputation) and a `walk_corpus`
resume, sound because sample seeds are a fixed function of (base seed, hand, seat, trick),
never of walk order — verified before restarting, h5 S0 re-walked byte-identical to the
killed block, the wall-ms field aside. The assembled walk (52 whole transcripts, threshold
10^6, 2,000-draw samples above): 276 of 364 decisions exhaustive, 88 sampled and marked;
**282/364 zero-regret (77.5%)**; 9 of 52 transcripts fully zero-regret against 18/52 on the
tricks-3-7 subset, so the early sampled tricks carry real regret; 82 conflicts (41
exact-expectation, 31 sampled-grade, 10 worldwise-dominance); 12 worldwise-dominated chosen
actions; 25 of 52 lost verdicts (9 declaring / 16 defending), earliest by trick {t2: 2,
t3: 7, t4: 2, t5: 3, t6: 6, t7: 5} — h0 S1 and h12 S3 are worldwise-lost by their *second*
decision. Largest transcript regret: h11 S3 defending, 87569113/2494800 (~35.1 valuation
units), still verdict-lost from t6, so the throwaway and the doom coexist. Part 1 ran ~70
min for 22 pairs and died; part 2 ran 20 min for the remaining 30 at 17% peak memory versus
39% — the cache bound made the walk faster.

**Adjudication alignment.** The dominance primitive became the world-count triple per
ordered action pair (T/W/S/I derived, conflicts firing on W); the localization primitive
became the live-world count with the win condition in the role-shifted strict-`<` form,
regenerated pins byte-identical, which pins the invariance; grades moved to the
`OperatorPair` product, so verdicts read "worldwise-dominance at (C, minimax-omniscient)".
The walk artifacts keep their recorded "PI" strings — they are receipts of the run that
produced them, with the mapping named in the assembled artifact's provenance header.

**What it taught.** Dominance is weighting-free but never *semantics*-free: the §14.5
record's `G^cont(2-1) ≡ 0` beside `G^cont(0-0) > 0` is the action-specific gluing-gap
mechanism, so PI dominance never implies Q^H dominance. Regret lives in tricks 1–2, where
fibers run to hundreds of millions of worlds and the walker must sample. And every headline
number is config-relative: the same h0 S1 transcript is "lost from t3 p3" at the CI config
and "lost(t2 p3)" at threshold 10^6.

## S5b — the `Lesson` type and the generalizer

**Question.** Can a conflict be generalized into a verdict that holds somewhere other than
where it was found?

**Method.** New walt-factory modules `lesson` / `basin` / `generalize` / `lesson_report`,
plus `Exp3aContext::try_eval` in walt-skeleton so atom semantics stay owned where they are
defined. A lesson is a two-sorted implicant — decision cells (hand, seat, declaration, role,
horizon, ply) and atom cells over the union vocabulary (holder, team and beater-count facts
per pool tile plus the ten exp3A control shapes) — mapping to a graded, labeled verdict and
carrying its origin conflict, the widening trace with every terminating witness, and the
measured basin. Quantifier placement is part of the verdict type: refutation and win
verdicts hold per matching (decision, world), the checker verdict per matching decision with
atom cells read fiber-valid. Domain: tricks 5–6, 104 decisions / 23,790 worlds.

**Result** (`lesson_basins_2026-08-10.txt`; 17 lessons — 11 refutation seeds, being every
dominated-chosen decision of the CI-config corpus, 5 win-form lessons, 1 checker seed):
refutation basins {0 ×6, 1 ×2, 2 ×2, 3 ×1} decisions; win basins {0 ×1, 1 ×2, 2 ×1, 5 ×1};
the chassis §12.6 lesson widens to the empty implicant with basin 13/13 eligible lead
kernels. Honest falsification verdict: **basins on this domain are TINY — median 0–1
decisions.** Six of eleven refutations never reach the domain: four are t3/t4 origins whose
load-bearing `horizon` cell pins them to their own horizon, two are tile-anchored pairs
never jointly legal at tricks 5–6.

**What it taught.** The transfer that exists is exactly selector-shaped: the h1 S0 lesson
("at ply 2, 5-2 beats the decisive tile" / "attains the optimum") crosses hands h1/h6/h7/h9
— up to 5 decisions, 651 worlds — and a *sampled* t3 conflict's lesson re-verifies worldwise
at h1 S2 t6, across horizons. The atom vocabulary is never load-bearing at these horizons:
its fiber-constant cells are mostly degenerate and all drop. The world-count triple caught a
real misclassification — h4 S1's "refutation" basin is (0, 1686, 0), class T, an
interchangeability-at-label statement whose receipt now carries §10.9's caveat that
label-level payoff ties do not make actions interchangeable for the seat; h4 S3's
(1, 1889, 0) is class W by a single strict world, visibly near-degenerate.

## S5b.1 — the design-call adjudication folded in

**Question.** Which of the seven S5b design calls survive review? **Result**
(`lesson_basins_2026-08-10_r2.txt`): four confirmed as built, three amended and implemented.
**Purpose-split basins** — each refutation basin splits into a refutation subbasin
(strict-somewhere decisions, the pruning content) and a safe-substitution subbasin (every
matched decision, weak dominance verified, zero loss per matched world): h4 S3 t5 p2 reads
refutation 1 / safe-substitution 2, the h4 S1 T-lesson refutation 0 / safe-substitution 2.
**DomainSpec-gated application** — the h2 S2 t4 empty-implicant lesson does *not* apply at
its own trick-4 origin even though both its tiles are legal there. **Per-carrier
denominators** — every basin line prints covered/eligible on the verdict's own carrier, so
the checker lesson reads 13/13 lead-kernel trees, never 13/104, separating empty carrier
(four tile-anchored refutations, eligible 0/0) from inhabited-but-unreached (h3 S3: 0/53;
h4 S0: 0/9).

**What it taught.** The adjudicated headline was *falsification deferred, falsifier
sharpened*: S5c's first milestone was re-scoped to **be** the falsification test, with the
standing label note recorded before any of it was measured — everything so far is at (C,
minimax-omniscient), so any positive is a hypothesis about (H, fixed-uniform-legal), not a
result.

## S5c-m1 — the falsification test proper

**Question.** On a domain where the atoms can discriminate, do conflicts generalize into
*atom-vocabulary* lessons that pay measurable rent — or does the direction die?

**Method.** **Order cells**: `horizon` and the registered numerics (`beaters-total(d)`,
`opp-beaters`) enter implicants as bound pairs, relaxed stepwise with witness termination
and held at the last verified value on refusal (`horizon>=5` held on h11 S1's win lesson) —
the S5b failure mode, where four zero-basin lessons died on a horizon pin that could only
keep or delete, is gone. **Cut refinement**: on a failed widening the generalizer may
*introduce* a world-selecting cell from the origin's registered vocabularies, at most 4 per
pass, rolled back entirely if the widening still fails, traced as `Introduce` steps distinct
from drops. **The t3–6 domain**: `DomainSpec` gains a fiber cap with exclusion semantics —
179 decisions / 924,813 worlds at cap 40,000, 29 in-range decisions excluded, all trick-3.
Rent is purpose-specific: refutation rent = applied + strict-applied + exact mean
matched-world improvement, never paid in T-coverage; win rent = worlds covered + actions
pruned; checker rent = applied.

**Result** (`falsification_2026-08-10.txt` and its amended regeneration `..._r2.txt`; 16
lessons = 10 refutation + 5 win + 1 checker). The seed count moved from 11 to 10 for an
instructive reason: at exhaustive threshold 100,000, h4 S1 t3 p2's sampled-basis dominance
did **not** survive exhaustive re-examination at 90,090 worlds — never-quote-above-grade,
demonstrated on the factory's own seed. On this domain at (C, minimax-omniscient) **the atom
vocabulary is expressively sufficient** (walt-math's sharpened phrasing: sufficiency, not
necessity): most final implicants are pure atom cells with the frame fully dropped, several
transferring across decisions and hands — including transfer that *excludes the lesson's own
origin* (h1 S2 t3: `beaters-total(1-0)<=1`, basin 3/89 frame-compatible). A minority are
equality-in-disguise re-descriptions of their origin, flagged `re-pinned` and excluded from
selection counts, giving the honest tally **10 of 16 lessons with selecting atom cells**, 12
having gained cells by cut refinement. Basins stay small: refutation {0 ×1, 1 ×7, 3 ×2}, win
{1 ×2, 3 ×2, 5 ×1}. Rent: each single-decision refutation basin's improvement equals its
origin regret *exactly* (65/112, 1243/1225, 1163/840, …), an independent cross-check against
the S5a fixture; cross-decision basins exceed it (h1 S2 t3: 16/15 over 3 applied); the
largest win rent covers 74,382 worlds / 109,032 pruned actions (h11 S1 t3).

**What it taught.** The direction survived its designed falsification point. The remaining
pressure was basin **scale**, which pointed m2 at the database economy rather than at more
conflicts — and at the seat-label re-measurement, since §12.4 made every number here
label-relative.

## S5c-m2 — honesty amendments and the seat-label re-measurement

**Question.** Do the lessons hold at the label the seat actually plays under?

**Method.** New `walt-strat/src/hidden_scalar.rs`, the scalar H solver: pooled-information
viewer maximization against the §7.4 fixed uniform-legal field, mid-trick roots,
unit-fraction particle weights so rational work concentrates at trick resolutions, budgeted
so an over-budget solve returns nothing — exclusion, never sampling; driven by
`walt-factory/src/label_transfer.rs`. At H the verdict quantifier changes shape by
necessity: a re-measured refutation is one inequality `Q^H(better) >= Q^H(worse)` per
matching decision and a win is `Q^H(action) = max`, atom cells read fiber-valid. Part A
folded in the m1 adjudication: witness exclusion CI-asserted per trace; load-bearing renamed
*surviving*; re-pinned pairs excluded from selection counts; intro-budget spent/4 on every
pin; and a control-bias annotation on every capped domain — all 29 exclusions are trick-3,
and fiber size anti-correlates with focal control (exp5 covariate), so the excluded set
skews low-control.

**Result** (`label_transfer_2026-08-10.txt`, budget 10^8 particle-steps per decision,
semantics tree-v0): **10 survive, 0 fail, 5 unmeasured of 15 value lessons.** Refutations
7/10 survive (h0 S1: Q^H(2-1) = 80/7 > 202/21 = Q^H(3-2); h3 S3: −75961447/3628800 >
−557701759/25401600; H-ties hold weakly where present, e.g. h1 S1 t5 p3 at −623/360 for
both). Measurable wins 3/3 — tile 5-2 is exactly H-optimal at all five basin decisions
across four hands. The five unmeasured are four budget-capped (fibers 16,632–34,650) and one
empty basin; the checker lesson is not re-measured, since §12.6 already lives at the fixed
field.

**What it taught — and what it got wrong.** The session concluded "the inventory is NOT
label-fragile on the measured subdomain", with the honest boundary attached: survival is
measured on each lesson's own small basin, and nothing promotes a (C)-graded claim. The
boundary was stated correctly and the conclusion was still **scale-limited** — the four
unmeasured decisions were exactly the big early-trick fibers where the labels diverge most,
and they were the ones that went on to fail. The recommendation that the capped fibers
pointed m3 at pooled-state H memoization rather than at new mathematics was right about the
method and wrong about the expected outcome.

## S5c-m3 — the memoized seat currency and the label-fragility discovery

**Question.** With the caps lifted, do the four unmeasured lessons transfer to the seat
label?

**Method.** `hidden_scalar.rs` gains `action_values_dag`: a per-measurement-call cache with
entries only at trick-boundary pooled states, keyed by (canonical weighted world-multiset ×
leader) with weights **mandatory** — pooled maximization depends on the weight profile, so a
weight-free key is unsound — in gcd-normalized unit-fraction projective normal form,
rescaled exactly on hit. Budget is redeclared as particle-steps over the memoized DAG
(semantics `dag-v1`): hits cost zero by unit definition, deterministically, so measurability
is a function of declared inputs and never of cache warmth, and every row carries the
semantics identifier plus `tree-equiv`, the exact tree-v0 cost of the identical computation.
Value transparency is CI-pinned: byte-identical Q^H against the unmemoized walk on all 16
m2-measured decisions.

**Result.** At dag-v1 10^8 every previously-measured value is byte-identical and the four
big fibers are honestly still capped (`label_transfer_..._r2.txt`). At dag-v1 10^9 — a
declared budget change only, never the same statistic improving — all four lift
(`..._r3.txt`): **1 survive, 3 fail, 0 unmeasured of 4 re-measured lessons.**

| Lesson | Seat-label measurement | Verdict |
|---|---|---|
| h1 S2 t4 refutation | Q^H(4-3) = 79/11 < Q^H(6-0) = 111269/13860 | FAILS — at the seat's label the "refuted" action is better |
| h1 S2 t4 win | Q^H(4-3) = 79/11; best is 1-1 at 2183/270 | FAILS |
| h11 S1 t3 win | at its origin Q^H(2-0) = −1927714337/319334400 loses to 5-1 = −547477589/91238400 | FAILS at the origin; its two transfer decisions hold exactly |
| h11 S1 t3 refutation | Q^H(2-0) = −1927714337/319334400 > Q^H(0-0) = −2897509283/479001600 | HOLDS |

**Cross-validation** (`h_tree_crossval_2026-08-10.txt`). The uncapped, unmemoized tree walk
independently reproduced every per-action Q^H vector byte-identically on all four decisions
— 15,486,288,612 / 10,766,263,412 / 4,214,899,874 / 65,449,828,676 tree particle-steps
(718.5 s / 549.3 s / 208.4 s / 3,693.8 s, the last ≈62 minutes of exact rationals) against
123,882,398 / 226,094,450 / 226,613,736 / 537,862,903 dag steps. The r3 file's
SINGLE-IMPLEMENTATION marking is discharged for all four decisions by an appended addendum.
**The FAILS are label fragility, not a memoization artifact.**

**The honest headline.** m2's "not label-fragile" was scale-limited. The combined tally at
the seat label — quoted across two declared budgets, r2 and r3 cited together — is **11
survive / 3 fail / 1 empty-basin of 15 value lessons**, and every failure lives on a big
early-trick fiber, exactly where the omniscient field (C) and the seat-facing field (H)
diverge. §12.4's label-relativity caveat was the truth. Registered-prediction check: the h1
S2 t4 failure separates on beater *totals* alone, so the §14.7 team-split-beaters prediction
registered in m2 is not yet triggered. **This is a first-class negative result:** an
inventory of verdicts, each exhaustively verified over its declared domain and each carrying
its labels honestly, was measured at the wrong label, and of the value lessons checkable at
the seat label three are false there. Nothing in the machinery broke — the type discipline
made the failure legible, and measuring label transfer *before* building the economy, as the
inventory's weakest load-bearing assumption, kept the failure cheap.

## The economy, honestly

**Method.** `db.rs` — a working set over an append-only archive, lesson identity = projected
content (canonical implicant cells, verdict, DomainSpec, operator-pair labels), **grade is
not identity**, the quotable grade is the max-grade archived derivation, and re-derivations
merge (demonstrated live: one entry, two archived derivations). `index.rs` — a
watched-feature index under a candidate-completeness contract, excluding only what the gate
provably refuses, cross-checked exhaustively in CI at 179 × 16 = 2,864 pairs, 893 candidates
covering 39 appliers at every decision. `ledger.rs` — the dual ledger. `certificate.rs` —
§16.11's eleven records per lesson, with the world-alignment-unchecked caveat, record 9
restored to per-world truth vectors in canonical world order after a walt-math dependency
check caught the compression, not-applicable records present-and-empty with reasons, H rows
honestly UNCHECKED-EXTERNALLY; 16 records against a self-contained schema-v1 written for a
future independent checker.

**The ledger's rules.** Pricing is **H-primary and never summed**: (H, fixed-uniform-legal)
rent is the currency, (C, minimax-omniscient) rent a recorded diagnostic on its own line.
**Unmeasured is never zero.** **Deletion requires N = 2 measured-consecutive zero-rent
epochs** with no intervening measured nonzero; capped epochs neither advance nor reset a
streak and are cited as gaps in the evidence pattern with their epoch ids. Every row carries
a single-implementation stamp with append-only clearance records, and a deletion may cite
only independently-cleared rows. Standalone rent is priced and overlap recorded but never
summed (7 overlap decisions). Restart-with-retention keeps DB, archive and ledger and
discards search state — memo tables *are* search state.

**Result** (`economy_2026-08-10_r2.txt`, ledger wired to dag-v1 at 10^9). All 15 value
lessons are measurable. The three H-fails price as **measured zeros with their reason
rendered** ("verdict FAILED at the H label"), and the deletion rule fires on all three
zero-streak lessons — the empty-basin refutation (h2 S2 t3) and both h1 S2 t4 lessons —
**each TRIGGERED and each mechanically BLOCKED**, for want of a registered independent H
checker. The crossval receipt is deliberately *not* a registered checker: it appears as
context lines that leave the single-implementation stamps unchanged. The h11 win prices
positive on its two holding decisions with its origin failure in the failed-count. The
earlier r1 run at tree-v0 10^8 triggered and blocked exactly one deletion, the empty-basin
lesson, because the h1 S2 t4 lessons were still capped and therefore held.

**What it taught.** The blocked state is safe by design and still stands: nothing was
deleted. The seat's own currency named as its first deletion candidates exactly the lessons
the seat's own label measured as worthless, then refused to act on its own measurement
without an independent check — "never trust the solver" expressed in types rather than in a
review checklist.

## S5d — the re-tethering (the era's hinge)

**Question** (Jason's, not the build's). Has the build come untethered from the mathematics?
**Method:** a direction session with no build — a fresh full read of the frozen v0.4 basis,
all 3,820 lines, against everything S5a–S5c had measured.

**Findings.** §12.4 with §17.5 had *already* ruled worldwise-PI classes the wrong
hidden-decision carrier: m3's label-fragility result re-confirmed a boundary the basis had
drawn in ink. §14.8 conclusion 10 and §18 already name the dynamic predictive quotient as
the target, and §12.8's exposed-policy geometry (7,848 information states, zero revelation
value, 15 exposed policies) is the standing down payment that the game is small on the right
carrier. The drift has a precise location: the S4 lumpability instantiation compared
observation and feature labels **RAW**, so only world-reconstructing skeletons could ever
pass. Jason's critique — "changing the domino changes the output; what must match is the
output under the quotient" — names the gap exactly: v0.4 quotients the state side but never
the interface alphabets.

**Jason authored §12.6A** in session: *equivariant controlled lumpability over declared role
interfaces*, sketched here only to orient the reader — theorem, proof and corollaries belong
to [walt-math-reference](walt-math-reference.md). Where v0.4 §12.6 demands literal equality
of legal-action and observation interfaces inside a descriptor class, §12.6A compares those
interfaces under **declared typed transports** Θ: two states with the same descriptor must
have legality preserved up to the action transport, `A(y) = Θᴬ(A(x))`, and must agree in the
joint distribution of (count-free increment, transported observation, successor descriptor)
— condition (ECL). The theorem gives exact compression: the pushed belief updates using the
quotient kernel alone; any abstract policy lifted through the transports is lawful and
induces the same joint law of observations, summed increment and terminal descriptor in the
concrete and abstract systems; hence the same law of the count-free terminal outcome. A
corollary descends the §8 additive valuation gauge to the quotient, and v0.4 §12.6 is
recovered at identity transports. The construction is count-free by design — the primitive
outcome alphabet is the trick coordinate alone, with tile anisotropy re-entering through
transported roles. walt checked the proofs and recorded two reader notes in Appendix A
(coherence scope over the transports; the abstract-policy-class optimization boundary); v0.4
stays frozen and the v0.5 track opens.

**The direction reset** (Jason's call): the goal is the **lossless count-free equivariant
quotient** — situations identical up to declared transports, outcomes compared under the
quotient, "hundreds, not 399M", player- or analyst-facing. Next work: find a nontrivial
(d, Θ) satisfying (ECL) on the existing probe kernels and count the classes,
counterexample-guided per §12.9, carrier per §11.2, Scheme/Fix as descriptor language per
§12.7 — which is where [walt-census-era](walt-census-era.md) picks up. Infrastructure
frozen: m4 (the independent Python H checker), S6 corpus-at-scale, and further economy
lifecycle work, all deferred, the economy left mechanically blocked because that state is
safe. `walt/math/implementers_guide.md` was commissioned as a derived, non-authoritative
companion.

**The NO-RESCUE policy** (Jason; now standing at the top of [`PLAN.md`](../walt/PLAN.md)):
when something doesn't work, that is not a thing to fix, spin, or assist with engineering —
it is a reason to go back to the math better informed. It is mathematics, so a failure is a
concrete counterexample to carry back to the basis, and that is a *good* outcome: "if the
whole thing falls on its face that's FINE". On verification: verify against the reference we
have, thousands of lines of proofs, but do not verify in triplicate; when independent
mechanical verification is genuinely needed the path is **Lean, not Python**, which retired
m4-as-Python (the `lean/` path is paved at 42/42 priority-0 rows kernel-proved). Standing
frame: the mathematics has proven the object **exists**, not its **utility** — if the utility
turns out bad, that is a conversation, and the instruments stay valuable either way (see
[walt-instruments](walt-instruments.md)).

## Source notes for this page

- **The LOG's entry order is not chronological.** In [`walt/LOG.md`](../walt/LOG.md) S5c-m3
  appears *before* S5c-m2. The dependency order is m1, then m2, then m3: m2 built the H solver and
  got 10/0/5; m3 memoized that solver, lifted the caps and corrected m2's conclusion. Read
  the entries in that order or the label story inverts.
- **Speed-ratio range.** The LOG's m3 entry summarizes the crossval as "28×–122× fewer dag
  steps"; dividing the receipt's own `tree-equiv` figures by its dag steps gives ≈125× /
  ≈47.6× / ≈18.6× / ≈121.7× across the four decisions, so the quoted lower bound is not
  reproduced from that receipt. Use the per-decision step counts above.
- **"10 of 16 lessons with selecting atom cells"** is read off the amended regeneration
  `falsification_2026-08-10_r2.txt`; the as-committed m1 artifact prints "12 with surviving
  atom cells", before the m2 amendment that flagged re-pinned pairs and excluded them from
  selection counts. Both files are committed; neither was edited.
- **Config-relative counts.** 11 dominated decisions is the CI config (tricks 3–7,
  exhaustive ≤ 40,000); 12 is the full-walk config (threshold 10^6); 10 is the m1 seed count
  at exhaustive threshold 100,000, after h4 S1 t3 p2's sampled-basis dominance failed
  exhaustive re-examination. Always carry the config with the number.
