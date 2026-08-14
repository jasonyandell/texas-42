# walt — the S6 era (S6a–S6d): predictive algebra, policy geometry, deadness, separation

[Home](Home.md) · owns: the four S6 probes as sessions — the predictive-rank dimension census and the Gate B
refutation (S6a), the policy-geometry probe at Gate E (S6b), the decision-deadness probe (S6c), the separation probe
(S6d) · Sources: `walt/LOG.md` (S6a–S6d); `walt/PREDICTIVE-RANK.md`, `walt/POLICY-GEOMETRY.md`,
`walt/DEADNESS-PROBE.md`, `walt/SEPARATION-PROBE.md`; `walt/math/predictive_algebra_v0.6.md`,
`walt/math/decision_sparse_exact_solving_v0.1.md` and its errata; `walt/CENSUS-RULINGS.md` (R-A1..R-A24,
PG-A1..PG-A18, J-A1..J-A18, DS-A1..DS-A36, SEP-A1..SEP-A19); the four results files under
`walt/walt-factory/results/`. Related: [walt](walt.md) (hub), [walt-census-era](walt-census-era.md) (what S6
answers), [walt-decision-sparse](walt-decision-sparse.md) (the architecture these probes serve),
[walt-math-reference](walt-math-reference.md) (formal statements), [walt-instruments](walt-instruments.md),
[walt-foundation-era](walt-foundation-era.md), [walt-factory-era](walt-factory-era.md),
[walt-scheme-fix](walt-scheme-fix.md).

> **Epistemic tier: EXPLORATORY — below every tier on
> [Home](Home.md#evidentiary-tiers--never-promoted-never-blurred).** Every number here comes from one Rust
> implementation over walt's own frozen exploratory basis (v0.4/v0.5/v0.6), measured on *fabricated void-free capacity
> fibers* whose members are **feasible and never reachable**. Nothing here is a corpus status, a kernel proof, an
> exchange-adjudicated result, or a rob receipt, and nothing here may be quoted in a brief, a dispatch, `FINDINGS.md`,
> or any claim-tier page. The named results — Lemma R and Corollary R-fold, Proposition G-flat and Lemma G, Lemma J
> with Propositions J-0/J-1/J-win, Corollary E4.1 — are walt-math adjudications *inside* that basis: proved arguments
> at exploratory tier, whose formal statements live in [walt-math-reference](walt-math-reference.md). This page
> records what was asked, how it was measured, and what came back.

## What the S6 era is

S5's compression program ended at Corollary S-rigid: the seat-side structural quotient at the first play is the
identity, because structural compression is bought with deadness and nothing is dead at the first play
([walt-census-era](walt-census-era.md)). S6 is the four sessions that took that negative seriously and asked four
smaller, sharper questions — getting, in order, a refutation, a stop, a measurement, and the branch's first positive
certification.

The arc in one sentence: the **value** side of the opening does not compress (S6a), the **decision** side collapses
almost everywhere and explodes exactly where the hand is tense (S6b), half of all mid-game free decisions are
provably choice-irrelevant and cheap detectors catch a third of them (S6c), and the root action can be proved exactly
without ever building the objects that exploded (S6d). Each session was adjudicated by walt-math **before** its build,
under the F7/NO-RESCUE policy: both outcomes are results, and a failure is carried back to the mathematics rather
than engineered around.

---

## S6a — 2026-08-12: predictive algebra (v0.6) and the dimension census

### Question

Jason filed a new math track, `walt/math/predictive_algebra_v0.6.md`: predictive state coordinates over ℚ —
continuation tests, exact predictive rank (the rank over ℚ of the continuation matrix; "predictive dimension"
hereafter, per R-A3), residual closure, and forward moments ψ(B) against backward policy vectors c_ρ with
J_ρ(B) = ψ(B)·c_ρ. The escape S-rigid had left open is v0.6 §5.4's hierarchy — linear rank ≤ positive realization
size ≤ partition-lump size ≤ |X| — so every behavioural row may be distinct (which S-rigid proved at the top) while
the rank stays small. **Gate B** is the pre-declared question: is the future low-rank? `walt/PREDICTIVE-RANK.md`
scoped part one to v0.6 Experiments 0 and 1 only, at grades 1–3, on void-free focal-lead roots.

### Method

The adjudication (rulings R-A1..R-A24) came back with a **theorem with teeth before any code ran**. Lemma R(c): in
Straight 42 every tile is eventually played and every play is publicly attributed, so a complete continuation record
determines the latent world; hence **any closure whose terminal seed contains a nonzero constant has predictive
dimension exactly |X|**. That makes v0.6 §6.2 verbatim and both distribution contracts — the trick-count distribution
(ii) and its next-leader-offset enrichment (iii) — **THEOREM rows, not measurements** (R-A24 governs how a degenerate
row is reported). Exactly one object remained measurable: **dim V^val**, the value closure of the count-free
expected-focal-trick contract, whose terminal seed is the zero space.

The same adjudication delivered the v0.6 proof audit (all SOUND; hypotheses H1–H3 became builder obligations; gaps
G1–G3 named), **Corollary R-fold** (predictive dimension is declaration-fold invariant, while bases and matrices are
freeze-relative and never fold-compared, R-A7/R-A21), the re-aiming of the concrete authority onto **treatment H**
(R-A10 — the design had equated the m3 solver with the P-A6 world-informed aggregate, which is strategy fusion,
P-Q2's trap in new dress, caught at adjudication), the root-only focal-lead fence (R-A8), and freezes 22–26 (R-A22
REJECTED the proposed 18–21 collision: spent numbers are never reassigned). The Gate B criterion was fixed in a
design addendum before any number existed (R-A20): with D(n) the per-grade maximum of dim V^val, **payoff CONFIRMED**
iff the dimension growth ratio is at most one third of the fiber growth ratio at both steps, **REFUTED** iff it
reaches two thirds at either, UNRESOLVED otherwise.

The build is `walt-factory/examples/predictive_rank.rs`, about 1,300 lines: exact sparse row reduction over
arbitrary-precision rationals (BigRational — walt's first use; `Ratio<i128>` survives only at declared boundaries),
the value closure via per-record generator families, the R-A18 correctness gate (per-lead Q against the ScalarHidden
dag-v1 authority through the exact affine bridge Q_diff = 2·Q_count − grade), plus Lemma R(b), Experiment-0 mass,
Lemma S-det bijection and 7-declaration fold receipts.

**Two builder errors were caught by the receipts mid-build, and both are instrument lessons.** First, the fold
transport is the **order isomorphism** of Lemma S-fold, not a pip transposition — run 1 crashed. Second, the
distinct-matrix census is **freeze-relative, not fold-comparable**: basis-dependent collisions moved counts by 1–2
across declinations, exactly as R-A7 had warned, and run 3 crashed; the lawful fold receipt is per-γ record counts,
γ being pip-free.

### Result

`results/predictive_rank_2026-08-12.txt`, 52,021 ms total, all receipts green, correctness gate MET at every
coordinate, fold 7/7 everywhere.

| grade | \|X\| | coordinates | dim V^val (full multiset) | behavioural rows |
|---|---|---|---|---|
| 1 | 6 | W = 12 | 1 at all twelve | 1 (six coords), 2 (six coords) |
| 2 | 90 | W = 6 | 42, 42, 52, 54, 56, 59 | 43, 54, 60, 61, 61, 72 |
| 3 | 1,680 | W = 3 | 1461, 1492, **1680** | 1470, 1505, 1680 |

At grade 2 the linear-over-partition win is real but modest: dimensions 42–59 against 43–72 behavioural rows. At
grade 3 one coordinate (idx = 0) sits at **FULL rank |X| = 1680 exactly**, the other two at roughly 87% and 89% of
|X|, and the behavioural rows have converged onto the dimensions — the partition/rank gap closes.

**Gate B verdict, at the pre-declared criterion: payoff REFUTED.** D(1) = 1, D(2) = 59, D(3) = 1680, so
D(2)/D(1) = 59 against a fiber ratio of 15, and D(3)/D(2) ≈ 28.5 against 56/3 ≈ 18.7. The dimension grows **at least
as fast as the fiber**, at both steps.

### What it taught

The reading is exploratory, coordinate-relative, and inside the R-A23 fence: the value closure **saturates by grade
3** — the span of lawful policy values is essentially all of ℚ^X. The mechanism rhymes with Lemma R(c) even though
the constant is excluded: the field-share weights w_o(ξ), products of 1/|legal|, are **world-discriminating**, and
hundreds of record-wise pullbacks inject nearly independent directions. Linear predictive compression under this
contract, field and observation model is dead at the depths that matter — and it is dead for **the same structural
reason the partition quotient was**: the game's public-attribution observation structure, now named twice from two
unrelated directions.

What was explicitly **not** killed, each of which became a later session or program: **root-action argmax partitions**
(the dropped-30 evidence says value spans can be full while the decision function is simple); **v0.6's dual policy
geometry**, Gate E, untouched and unmeasured, which is S6b; and **moment compilation for fixed shallow queries** —
the lead-recovery DP never needed a spanning basis — which is Gate D, still separate and unmeasured.

The fence travels with the numbers: a predictive dimension licenses **no** runtime or tractability claim, is not a
state count, not a class count, not an r3-style dynamics quotient and not a value partition; it promotes no v0.6
theorem; and by P-A21 three rungs are not a law and no dimension at any grade is quoted for the opening.

---

## S6b — 2026-08-12: the policy-geometry probe (v0.6 Gate E)

### Question

Jason's direction after the refutation restated what he had actually been hoping for: **similarity of outcomes** —
"likely to get 32 one way or the other", the melted-candlewax oracle PDFs, spiky and clustered with long dead flats —
not exact low dimension. Gate E is the exact, already-adjudicable fragment of that direction: even with the
predictive dimension full, an enormous information-consistent policy set may induce few distinct value vectors, and
fewer still ever optimal for any belief. `walt/POLICY-GEOMETRY.md` fixed four cardinalities that are never conflated:
**N_pol** (lawful deterministic information-consistent policies extending a root action), **N_vec** (distinct value
vectors), **N_par** (Pareto-undominated vectors), **N_exp** (vectors exposed as the maximum of E_β[V_ρ] for some
belief).

### Method

Rulings PG-A1..PG-A18 delivered two results that reshaped the probe before it ran.

**Proposition G-flat**: grades 1 and 2 carry **no policy geometry at all** — the continuations are forced — and at
grade 3 the only free layer is trick 2, with N_pol(a) = 2^k(a) exactly. The probe therefore has exactly **one**
measurement, at grade 3, and the growth-ratio criterion the design proposed was structurally unavailable; PG-A15
replaced it with absolute bands fixed before any number existed.

**Lemma G**: backward Pareto pruning is exact through the positive composition, the incremental fold is
**mandatory** (without it the fold materialises ∏|S_i| combinations before any pruning can occur), N_vec is destroyed
by any pruning, and convex pruning is lawful for N_exp only — and the design's own definition of exposure ("optimal
for some belief") was **not pruning-safe**; PG-A4 replaced it with the UNIQUE maximiser, a silent shrink caught at
adjudication. The exposure method was frozen as Lark's LP, exact-rational primal simplex with Bland's rule, witnesses
both ways (PG-A9/PG-A10), and PG-A13 fixed the cap discipline: a capped coordinate reports **no** N_par, because a
partial frontier bounds nothing in either direction.

### Result

`results/policy_geometry_2026-08-12.txt`, 74,365 ms. Of the nine measured grade-3 (coordinate, lead) pairs:

| coordinate | lead | k(a) | N_pol | N_par | N_exp |
|---|---|---|---|---|---|
| idx = 0 | 00 | 384 | 2^384 | 1 | 1 |
| idx = 0 | 10 | — | — | STOPPED | STOPPED |
| idx = 0 | 11 | — | — | STOPPED | STOPPED |
| idx = 1299709 | 21 | 7416 | 2^7416 | 1 | 1 |
| idx = 1299709 | 22 | 6018 | 2^6018 | 1 | 1 |
| idx = 1299709 | 63 | 19930 | 2^19930 | 1 | 1 |
| idx = 2599418 | 11 | 8748 | 2^8748 | 1 | 1 |
| idx = 2599418 | 22 | 7146 | 2^7146 | 1 | 1 |
| idx = 2599418 | 42 | 6018 | 2^6018 | 1 | 1 |

**Seven of nine: the Pareto frontier is a SINGLETON.** One policy weakly dominates every lawful alternative in every
one of the 1,680 worlds; N_par = N_exp = 1 against plan counts running to 2^19930. The receipts: the dominance spot
receipt at idx = 0 lead 00 (1,024 explicit policy variants, mask bits over free states, all pointwise under the
frontier singleton — HELD), and the G-flat receipt rows at grades 1 and 2, all reading 1 by proposition — and
PG-A16(ii) insists those are receipts, never evidence of collapse.

**Two of nine: STOPPED.** One coordinate (idx = 0) at leads 1-0 and 1-1 under 0-trump — the non-boss trump leads. The
running frontier exceeded the declared cap (4,096, then 16,384 on a declared raise); no N_par is reported for them.

**The formal verdict, per the pre-declared discipline, is STOPPED / NO VERDICT.** A measured coordinate hit a cap, and
a capped coordinate forbids the global claim (PG-A13). This dissent travels with every citation of the 7-of-9 number:
the session did not confirm strategy-side collapse, and it is never presented as 7/9 success.

### What it taught

**The bimodality is the real finding**, and it is what the verdict line cannot say. Total strategy-side collapse
almost everywhere; genuine frontier explosion **exactly where the 42 is genuinely tense** — leading a low trump
rather than the boss. Where the frontier explodes, the incomparability structure ("better except when X") is exactly
what the spike-anatomy frame wants to name, and it became Experiment B of the decision-sparse program.

No similarity or tolerance claim is made or supported (PG-A17, verbatim in the results file): "playing this domino
means I am likely to get 32 one way or the other" is a statement about score distributions under a tolerance, and
this probe measures neither — score is out of scope, and by Lemma R(c)–(d) the distribution contract has predictive
dimension |X| anyway. A vector here is an expected-trick profile over a declared fiber, not an outcome law and not
"an outcome"; δ-similarity remains future mathematics needing its own typed rulings. PG-A18 closes the loop: a
collapse verdict would not have rescued Gate B, transferred to the opening, or established anything about
similarity — **and a STOPPED verdict is none of those things either.**

---

## S6c — 2026-08-13: the decision-deadness probe

### Question

Jason's framing: "junk everywhere is a category of hands — *I can't beat anything and nothing I play will change any
outcome*". Can a seat detect that **very cheaply**, so fiber exploration prunes the focal branching to 1? The S6b
specimens grounded it: six of the seven singleton roots collapsed by **indifference**, and deadness plausibly claims
whole subtrees mid-playout wherever it claims a root.

**The binding constraint, Jason's verbatim intent: count vetoes elimination.** "If you have count in your hand at all
let's not eliminate the branch. Count changes outcomes a lot, even if we are blind to it at the moment we are
evaluating." Every detector carries `hand ∩ COUNT = ∅` as a **firing conjunct, not an option**.

### Method

Rulings J-A1..J-A18, design `DEADNESS-PROBE.md`, freezes 32–35. Three one-sided bitset detectors, each proved before
it ran, each firing only when the node is decision-dead and each for which UNKNOWN is always lawful:

- **D0** (Proposition J-0, no-possible-winner plus the guard): not on lead, no trumps, every focal tile beaten by
  every potential leader in every followable context — and **no exhaustion margin is needed**, because the beater is
  the led tile itself.
- **D1-sym** (Proposition J-1, the two-tile transposition isomorphism). The design's looser "order-exchangeability"
  D1 was REJECTED at J-A6 and replaced by this.
- **D1-win** (Proposition J-win, focal sweeps): **count-free only** — the guard provably does not rescue it, and its
  verdicts are void wholesale the instant count re-enters (E-A2).

Ground truth is the one-deviation tie classifier (argmax-indifference), and J-A10's typing of it is load-bearing: the
tie set is a **SUPERSET** of exact decision-deadness, so **recall against it UNDERSTATES the detectors**. So is
J-A1's three-way typing — forced (|legal| = 1, no decision, never in any dead fraction) / decision-dead (all policies
value-identical, the object) / dominant (one Pareto-optimal vector) — under which S6b's singleton frontiers are
**dominance, not deadness**; no sentence here presents a singleton-frontier count as a deadness count.

The runner is its own story. After the sequential run was killed twice, rulings DS-A29..DS-A36 (freezes 41–43)
rebuilt it parallel and resumable: per-unit checkpoints under a freeze digest, canonical-order assembly, a
byte-diffable deterministic block, every stop criterion a deterministic count and never wall-clock, and **contended
timings barred from the dividend with the bias direction named** (DS-A32 — contention discounts the detector's
cache-resident work relative to the memory-heavy solve). See [walt-decision-sparse](walt-decision-sparse.md) for how
that discipline generalised.

### Result

`results/deadness_2026-08-12.txt`, 45 units = 3 grade-3 coordinates plus 9 eligible n = 4 receipt-rung coordinates,
each crossed with its leads. n = 5 was a declared stop, not measured.

**Soundness: PERFECT.** 174,250,255 detector calls, 27,980,333 fires, and every fired site that ground truth could
classify was genuinely indifferent — **zero false positives**. Every J-A14 bit-exact V/Q assertion across the arms
held. The propositions held at census scale.

**The indifference is enormous.** Of 49,522,677 ground-truth-classified call sites, **25,255,316 — 51% — are
one-deviation ties**. Jason's "junk everywhere" is now a measured half of all mid-game free decisions at these
coordinates.

| detector | hits against the tie denominator (25,255,316) | note |
|---|---|---|
| D0 | 138,208 | rare, but sometimes total |
| D1-sym | 8,263,821 | the workhorse |
| D1-win | 0 | never fired, 0 of 45 units |
| any | **8,335,057 (~33%)** | and this understates, per J-A10 |

D1-sym is order-exchangeability at scale, reaching **97.5%** of a unit's ties (n4 hand = 12, lead = 65: 2,358,996 of
2,418,996). D0 is rare but sometimes **total**: one grade-3 root family (idx = 1299709) is wholly decision-dead and D0
certifies essentially all of it — 7,416 of 7,416 at lead 21, 19,924 of 19,930 at lead 63. D1-win never fired: the
sweep condition does not occur at these coordinates.

**Cost: about 25 ns per call**, with the honesty note attached because it is the whole point of freeze 43 — that
figure is **contended and therefore not quotable**, and the sequential timing rung, the only quotable cost
instrument, is unrun. Against solve arms in the 10^4–10^5 ms range at the n = 4 rungs, detection is effectively free,
and S5j's failure mode (detection cost eating the tablebase's dividend) did not recur.

**The open mechanism stays open (J-A8).** The trumpless-junk grade-3 family at idx = 0 has 276–1,773 ties per lead and
**zero** detector hits across all three leads. Its tie mechanism remains unidentified, and there is no fourth
detector without a proof.

Mechanism receipts, all green: DS-A30(iii) resume-validation PASS; DS-A36 byte-diff of the deterministic block across
two invocations IDENTICAL; the checkpoint cache survived a mid-run kill at 41 of 45 units, and the resumed run
reproduced the block byte-exactly.

### What it taught

Tags travel with verdicts per J-A3: D0 and D1-sym survive count re-entry under the guard (Lemma J(c), strengthened to
Lemma J(c′) at DS-A24 — constancy of the tile-value schedule on the focal hand, not vanishing), while D1-win verdicts
die the instant count re-enters. A deadness verdict is one of the very few walt objects that crosses the
Φ(C) ⊊ Φ(C₀) gap intact, its conditions being functions of the focal information state quantified over the whole live
set — but it is **relative to a field that does not condition on focal's tile identity**: against an opponent who
reads discards, the choice signals and the verdict does not transfer.

The connection to the architecture is direct: fired nodes are exactly where treatment-H solves lose their max nodes,
and **Experiment A of the decision-sparse program is complete with this run** — the detector family is the
deadness/dominance reduction source the candidate-library architecture asks for
([walt-decision-sparse](walt-decision-sparse.md)).

---

## S6d — 2026-08-13: the separation probe (Experiment E)

### Question

Can the root action be proved exactly — a **primal witness** L (a fixed lawful information-consistent policy, no
maximisation at any node below the root) against an **action-conditioned upper witness** U (treatment C: root action
held, world revealed before any later focal decision), with L_{a⋆} ≥ U_a for every competitor — **without** the
objects that could not be computed, namely S6b's exploded frontiers?

### Method

Design `SEPARATION-PROBE.md`, rulings SEP-A1..SEP-A19, freezes 36 and 37. Freeze 37 identified the *existing*
`walt_strat::revealed::revealed_summary().q_c` as the action-conditioned U — correcting DS-A7(iii)'s "must be built"
premise at SEP-A7; what remained to build was the harness and the receipts, not the evaluator. Freeze 36 fixed the
candidate-library v1 format: observation-record keys, no values, no verdicts, identity transport only, cache never
authority.

walt-math delivered **Corollary E4.1** at adjudication, and it reorganised the whole run. With H-argmax seeds the
primal witness is at its **ceiling** — L = Q^H necessarily — so L is a receipt tying two evaluators together rather
than a measurement, and **every separation verdict in this run is decided entirely by the U side**. Its third clause
is the sharper one: a NOT-SEPARATED pair is the exact statement Q^H(a⋆) < U_a, which is a **proof that no candidate
set whatsoever** separates that pair under relaxation C at that coordinate — not "this run's candidates were not
strong enough".

Two proposed receipts were rejected at adjudication and replaced. The two-solver bridge assertion would have asserted
a false identity, both H solvers being already in the differential convention — replaced by exact per-action equality
with **no bridge** (SEP-A5). The `is_affine` assertion is vacuous, one piece always sitting at zero slope — replaced
by the max-freedom singleton assertion and its counted receipt (SEP-A13, disambiguated at SEP-A19: the reached-set
count is an exact observable of the exhibited witness, tie-break-relative to freeze 26, never a term in the DS-A2
ladder).

### Result

`results/separation_2026-08-13.txt`, 3,942 ms, gate MET at every coordinate, all five receipt families (R1–R5) HELD
everywhere. **All three grade-3 coordinates SEPARATED — the first exact root-action certifications in the branch.**

- **idx = 0** (hand [00 10 11], 0-trump): root action **00** certified against both competitors, margins L − U of
  **449/1120** against lead 10 and **59/2240** against lead 11 — precisely the two leads whose Pareto frontiers S6b
  could **not** complete (cap 16,384).
- **idx = 1299709** (hand [21 22 63]): root **22** certified at the razor margin — the full **1/63** headroom against
  lead 63, because U is exactly tight at the runner-up; and 1/42 against lead 21.
- **idx = 2599418** (hand [11 22 42]): **both tied H-optima certified**, each against the other at margin 0 and
  against lead 42 at 1/21; Opt^H = {11, 22}, reported as H's fact and never as the witnesses'.

Every SEPARATED verdict carries the **member-not-set** caveat verbatim: non-strict separation certifies membership in
the optimal set and never uniqueness (Theorem E6.4).

**The price localization is the finding.** Of the nine per-action prices U_a − Q^H(a):

| coordinate | action | price U_a − Q^H(a) |
|---|---|---|
| idx = 0 | 00 (the boss lead) | 0 |
| idx = 0 | 10 | **11/1120** |
| idx = 0 | 11 | **29/420** |
| idx = 1299709 | 21, 22, 63 | 0, 0, 0 |
| idx = 2599418 | 11, 22, 42 | 0, 0, 0 |

**Seven of nine prices are exactly 0** — the C-relaxation is TIGHT at every action of both indifference-collapsed
coordinates and at idx = 0's boss lead. The only two nonzero prices sit **precisely at the two leads where S6b's
frontier exploded**. Action-conditioned information price and strategy-side frontier tension coincide exactly on
this three-coordinate, exploratory sample.

Candidate library v1 was written with four entries: the informative idx = 0 lead-00 playbook plus lawful-but-vacuous
tied-optimum entries. The run was scoped as a **certification probe, not a library harvest**, since six of the seven
completed S6b singleton roots are indifference-collapsed and there is nothing yet to transport.

### What it taught

**The frontier is unnecessary for the root decision** — the decision-sparse thesis in one measured instance. That is
one of two sentences the results file prints, and **neither stands for the other**. The second is the fence at
SEP-A15(ii), which must travel with the first every time: *this run does NOT test the parent's economy claim* ("the
solver does not need an exact solution for every action") *— it computes the exact H solve at every action because
DS-A10's receipts require it.* Related fences: no cost, timing, runtime or tractability claim of any kind, because
the run performs the very solve it would have to avoid (SEP-A15(iii)); wall-clock is provenance only. And the
provenance typing at SEP-A12: the separation's *validity* does not cite H, but this run's *witnesses* were produced
with H's help — L's seed is an H solve and receipts R1–R4 are H cross-checks.

Both outcomes were results in advance, and the positive one landed. The named next step is SEP-A17's successor: seed
L from a **non-exact** source and ask whether the sandwich still closes — see
[walt-decision-sparse](walt-decision-sparse.md).

---

## Read together

Put S6a beside S6b and the pair says something neither says alone. The **value span is full** — dim V^val saturates
to |X| by grade 3, Gate B refuted — while the **decision side collapses to a single dominant policy** at every
measured grade-3 root the frontier computation could finish. Value richness and decision simplicity coexist: the
dropped-30 lesson made exact at the strategy level, and the first exact evidence *for* the outcome-similarity
direction — while remaining, per PG-A17, no similarity claim at all.

S6d closes the loop the other way. The two leads where S6b's frontier exploded are exactly the two where the
information price is nonzero, and exactly the two competitors S6d certified against **without ever completing the
frontier**. Truth can be high-dimensional; the frontier can be uncomputable; the root decision can still be proved.
That is the whole content of the decision-sparse reframe, and S6 is the four nights that produced the evidence for
it — three of them by refusing to soften a negative.

---

## Source discrepancies flagged, not resolved

Drift between `walt/LOG.md` and the results files is a bug; these are recorded rather than silently reconciled.

- **S6b, G-flat receipt rows.** The LOG records "30 rows, all 1"; `policy_geometry_2026-08-12.txt` prints **24** —
  12 at grade 1 and 12 at grade 2.
- **S6b, the authority receipt.** The LOG records "the authority receipt (frontier max = treatment H exactly, every
  row)". No such line appears in the results file, which prints the dominance spot receipt and the `beta0-max
  vectors = 1` diagnostic (PG-A12) but no explicit frontier-max-equals-H receipt.
- **S6b, the two stops.** The LOG says both caps were hit "at the SAME trick-1 partial sum". The results file records
  them at records `[10 20]` and `[11 21]` and reports only the 16,384 cap; the earlier 4,096 cap and the partial-sum
  coincidence are LOG-only provenance.
- **S6c, detector cost.** The LOG quotes "~25 ns/call"; the results file, a RESUMED run, prints "0 ns over 0 calls".
  The 25 ns figure comes from a prior invocation, and is contended and not quotable either way (DS-A32).
- **S6c, solve-arm scale.** "10^4–10^5 ms" describes the n = 4 rung arms (33,247–503,876 ms in the timing block); the
  grade-3 arms are 40–308 ms.
- **S6c, run provenance.** "Survived a mid-run kill at 41/45" and "byte-diff across two invocations IDENTICAL" are
  LOG records; the results file shows `45 units loaded from checkpoint, 0 computed by this process` and
  `resume-validation: PASS` for one recomputed unit.
- **S6d, the "108-decision playbook" — since RESOLVED (EC-A12, 2026-08-13).** The separation results file reports the
  idx = 0 root-00 candidate as "50712 states, 384 with genuine choice", 22,920 reached; the 108 is 384 call sites minus
  S6c's 276 one-deviation ties. The adjudication confirmed the arithmetic and ruled how to name it: **use 384**, the
  receipt-backed free-decision count; 108 is a derived difference of two measured counts, inherits both scope fences,
  is present in no receipt, and is quotable only by brief amendment. Typing in full:
  [walt-math-deadness](walt-math-deadness.md#384-versus-108--how-to-name-that-playbook-ec-a12).
- **S6d, ruling range.** The LOG cites SEP-A1..SEP-A19; the results-file header cites SEP-A1..SEP-A18 while its body
  cites SEP-A19(b). SEP-A19 was ruled at build time, after the header text was fixed.
