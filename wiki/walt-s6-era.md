# walt — the S6 era (S6a–S6n): predictive algebra, policy geometry, deadness, separation, the two gluing rungs, and the fee line

[Home](Home.md) · owns: the S6 probes as sessions — the predictive-rank dimension census and the Gate B
refutation (S6a), the policy-geometry probe at Gate E (S6b), the decision-deadness probe (S6c), the separation probe
(S6d), the economy-seed probe (S6e), the freeze-44 refactor and n = 4 gate result (S6f), the trick-1 draw probe
(S6g), the n = 4 separation pass (S6h), the lay-down catalogue (S6i), the rule-economy probe (S6j), the
fusion-tax probe (S6k), the second-rung probe (S6l), the feature-fee audition (S6m), and the fee-correlation chapter (S6n) · Sources:
`walt/LOG.md` (S6a–S6n);
`walt/POLICY-GEOMETRY.md`, `walt/SEPARATION-PROBE.md`, `walt/SEPARATION-RUNG-N4.md`, `walt/ECONOMY-SUCCESSOR.md`,
and the retired design docs `PREDICTIVE-RANK`, `DEADNESS-PROBE` (retired 2026-08-24 after their probes closed; bytes preserved at `git show 2de8a05:walt/<NAME>.md`);
`walt/math/predictive_algebra_v0.6.md`,
`walt/math/decision_sparse_exact_solving_v0.1.md` and its errata; `walt/CENSUS-RULINGS.md` (R-A1..R-A24,
PG-A1..PG-A18, J-A1..J-A18, DS-A1..DS-A36, SEP-A1..SEP-A19, N4-A1..N4-A20, EC-A1..EC-A14, T1-A1..T1-A12,
LD-A1..LD-A13, RW-A1..RW-A8, FT-A1..FT-A29, SR-A1..SR-A37, FF-A1..FF-A33, FC-A1..FC-A22); the results files now at
`walt/probes/factory-results/` (produced under the old `walt-factory/results/` path — the producing crate is
archive-only at commit `648f93a` since the 2026-08-24 unification; path and crate names below are historical truth).
Related: [walt](walt.md) (hub), [walt-census-era](walt-census-era.md) (what S6
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
([walt-census-era](walt-census-era.md)). S6 opens with four sessions that took that negative seriously and asked
four smaller, sharper questions — getting, in order, a refutation, a stop, a measurement, and the branch's first
positive certification.

The arc of those four in one sentence: the **value** side of the opening does not compress (S6a), the **decision**
side collapses almost everywhere and explodes exactly where the hand is tense (S6b), half of all mid-game free
decisions are provably choice-irrelevant and cheap detectors catch a third of them (S6c), and the root action can be
proved exactly without ever building the objects that exploded (S6d).

The seven that follow push on the two halves of the sandwich in turn. S6e and S6j cheapen the **primal** side —
first at grade 3, then map-free at grade 4, where a four-word rule certifies every coordinate that is certifiable at
all. S6f, S6h and S6k work the **upper** side: a budget contract and a gate failure filed as a result, then the
branch's first exact negatives, then the first gluing cut that closes a pair those negatives had proved no candidate
could ever close. S6g and S6i step outside the sandwich entirely, proving first-trick plays and a family conjecture
by characterization rather than by search.

Every session was adjudicated by walt-math **before** its build, under the F7/NO-RESCUE policy: both outcomes are
results, and a failure is carried back to the mathematics rather than engineered around. Three of these sessions had
their proposal refuted or their gate failed, and each of those is filed here as a result rather than smoothed over.

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

## S6e — 2026-08-14: the economy-seed probe (SEP-A17's successor)

**The question.** Does the sandwich close on a primal witness that is *not* an exact solve? Adjudicated as
EC-A1..EC-A14 (freeze 46, the closed arm list; freeze 36 v2, opening transport for the declaration fold under the
newly delivered **Corollary S-fold-val** — value transport along the fold is reading-independent, so every fold image
is a receipt, and the design's proposed S-A2 diagnostic was rejected as measuring nothing). The central instrument is
the slack identity (R8): a seed separates **iff** its economy gap g = Q^H(a⋆) − L ≤ the certification slack
s(a⋆) = Q^H(a⋆) − max U — and s was already exact from S6d's margin column, so the whole reading was pre-declared
to the last rational before any seed existed: idx = 0 loose at 59/2240, idx = 1299709 tight at 1/63, idx = 2599418 a
zero-slack control where only an exactly-optimal seed can separate, by theorem.

**The result** (`economy_seed_2026-08-14.txt`, all receipts held, nine fold-image coordinates receipt-clean):
**CERTIFIED-CHEAP at both positive-slack coordinates.** At idx = 0 — the coordinate with 384 genuinely free decision
states — **P2 greatest-tile and P4 trump-hoard certify the lead at economy gap zero**: two four-word rules
independently reproduce the dominant playbook's cash-the-boss, hoard-the-trump line everywhere it matters, while P1
least-tile and P3 beat-if-able fail by 3/7 of a trick, sixteen times the slack — exactly matching the S6c/S6b
diagnostic that the playbook disagrees with both. At the tight rung every cheap arm certifies (the
indifference-collapsed coordinate: any lawful continuation attains Q^H, and the root choice is carried by the fixed
a⋆). The zero-slack control fired as pre-declared: **ZERO-SLACK: SEED EXACTLY OPTIMAL (NOT ECONOMY)**. The re-key arm
ran fenced (HEURISTIC RE-KEY, NOT A TRANSPORT) with its fallback counts printed — essentially all-P1 at the other
coordinates, which is itself honest data about how little a record correspondence carries between unrelated hands.

**Scope, exact (EC-A13):** the **primal half** of the parent's economy claim is exercised — the *witness* at a⋆ need
not be an exact solve. The run itself still computes U exactly at every competitor and H at every action for its
receipts. The **full** claim — a solver that avoids exact solves — additionally needs the U side cheapened: Theorem
E6.5's gluing ladder, Experiment D, freeze 38 still reserved, untested. Any sentence saying "the economy claim was
tested" without the word *primal* has over-claimed.

## S6f — 2026-08-14: the freeze-44 refactor and the n = 4 gate result

**The build.** N4-A1..N4-A12 fixed freezes 44 and 45, and the refactor landed: every walk-based evaluator now carries
a deterministic walk-step budget (one unit per (particle, node) visit, the same rule as the scalar authority's
particle-step) with declared stops and no partial folds; `revealed_summary` takes one whole-call budget and discards
everything on exhaustion ((C2): a partial fiber sum bounds nothing); the partition build checks P_max at each
insertion; the freeze-26 bridge is a function of the declared grade. **(R0), the blocking regression, PASSED**: the
filed grade-3 receipt reproduced under the budgeted evaluators with exactly the two enumerated permitted differences,
and the candidate library byte-identical.

**The §5 measured rung** (W = 1; M_max = 40 GiB declared by the run owner per N4-A4; decimation prime g = 15,485,863,
freeze 44(e)): **NO-GO — and the gate failure is filed as a result**, the measured cost model SEP-A10(i) said was
missing. What it measured: the **U side is affordable** at n = 4 (estimated whole-fiber revealed cost ≈ 4.33 × 10⁹
walk-steps against the 4 × 10¹⁰ budget); the blocking objects are the **partition** — at (h0, action 0-0) the state
count exceeds P_max = 32,000,000, where the design's explicitly-labelled estimate said 24.8M — and **per-unit
wall-clock** (the largest unit extrapolates to ~58 minutes against the ten-minute gate). The declared fallback
{h6, h4, h8} fails its own gate arithmetic at h8 (~648 s against 600 s by the quoted tree-v0 scaling), and per
N4-A12(c) a second gate failure is a **return to the rulings file** — no second fallback, no nudging (F7). The n4
machinery (three-tier H regime, checkpointing with the shared-call clause, DS-A36 blocks, real-deal fences) is built
and committed, gated behind the return.

**What the failure teaches**, for the return: the sandwich's upper witness scales; the primal pipeline's *state
enumeration* is what does not — the two-map partition/extraction form and the E4.1 exact-seed route hit their
declared ceilings one grade above where they were designed. The pre-adjudicated N4-A5 digest fallback halves memory
but cannot lower a state *count* below P_max; the levers that remain are adjudicator's constants (the ten-minute
gate, P_max), W-parallelism across units, checkpointed long passes, or a primal route that does not enumerate the
partition — which is precisely the direction S6e's cheap seeds point (a rule needs no partition to be *stated*; only
its exact pricing walks the tree).

---

## S6g — 2026-08-14 (overnight): the trick-1 draw probe

**The question.** Jason's oldest target, taken directly: prove a first-trick play. The build proposed a *bounded
sandwich* — a per-world adversarial-field lower bound and a cooperative-field upper bound, summed exactly over the
trick-1 fiber of |X| = 399,072,960 worlds — and asked for a night of compute.

**It was refuted by proof before any compute ran**, and the refutation is itself the session's first filed result.
Both corner bounds are sound (T1-A2), but **Proposition T1-blind** shows a lower witness valid at *every* root
action — which every hand-only counting guarantee is — can never strictly exclude any competitor, because
U_a ≥ Q^H(a) ≥ L for that same a; and **Proposition T1-corner** shows the corner sandwich closes at trick 1 only
when the focal holds the entire trump suit. A 399-million-world scan would have measured exactly that. This is the
lower-side half of a pair of negatives; its upper-side twin arrives at S6k.

**What replaced it needs no relaxation at all.** **Theorem T1-draw**: on a closed, fully enumerated family of **294
declared trick-1 coordinates**, the focal seat takes all seven tricks against **every** field behaviour in **every**
world, so Q^H(a) = +7 — the maximum of the valuation — for every trump lead, and the upper witness membership needs
is the trivial U_a ≤ 7. **Corollary T1-ruff** prices the double lead by the ruff it invites, which supplies the
strict exclusion.

**The result** (`trick1_draw_2026-08-14.txt`, 159,678 ms). Of the 294 coordinates, seven are all-trump hands and
are labelled TRIVIAL — every action takes every trick, no competitor exists, no decision is certified. On the
remaining **287, Opt^H is determined EXACTLY**: every trump lead at +7, and every double lead strictly excluded with
its exact q from an exhaustive integer count over all 399,072,960 worlds, no decimation, one final rational. At the
flagship coordinate — declaration `PipTrump(6)`, focal hand {6:6, 6:5, 6:4, 6:3, 6:2, 6:1, 5:5} —
**Q^H(5:5) = 7 − 143/5814** exactly against Q^H(trump) = 7, so Opt^H is exactly the six trump leads. Theorem E6.4's
member-not-set caveat is **DISCHARGED, not waived**: both sides are exact values, not bounds. The reduced-grade
authority ladder (T1-R2) held to the rational at grades 2, 3 and 4; grade 5 was a **declared stop**, the authority
budget of 200,000,000 exhausted at 199,999,988 steps, reported either way.

**The corpus arm is the measurement, and it specifies the frontier.** None of the 13 real trick-1 hands satisfies
T1-draw's hypotheses — expected, and filed as a result. What each yields is its exact **corner gap** 7 − k − E_β[f],
computed by exhaustive fiber sums with no decimation, ranging from **4** at the tightest to **19/3** at the widest.
Those thirteen rationals are the filed specification of what a tighter relaxation must beat, and they are what the
next dispatch was aimed at.

**Two fences travel with all of it.** The membership half is **belief-free and field-free** — it holds pointwise in
every world against every field behaviour, and because the trick-1 fiber is the complete set of deals consistent
with the focal hand it is a statement about **the rules** rather than the model; this is the one place in walt where
R-A2's feasible-versus-reachable fence does not bind a verdict, not because it was relaxed but because the statement
ranges over everything. The exclusion half **is** model-relative, under the declared uniform belief and
uniform-random legal field, and no row lets the first half's strength leak onto the second. And, verbatim: *a
drawing hand is a hand that plays itself; the theorem certifies a first-trick play where no search is needed to find
it, and says nothing whatever about hands that require judgement.*

**The risk, carried (T1-A12).** Every statement is proved relative to **walt's implementation of the rule algebra**,
read from `rules.rs` at adjudication time. If the implementation and the rules corpus disagree, this section is
wrong in a way **no receipt inside it can detect**, because every receipt is computed by the same implementation.
(T1-R2) is a partial guard — an independently written solver, not the corpus. **The corpus check is mandatory
before any of this is cited outside walt.**

---

## S6h — 2026-08-14 (overnight): the n = 4 separation pass

**The question.** S6f's gate said NO-GO. The 2026-08-14 return (N4-A13..N4-A20) authorised the pass at W = 8,
checkpointed, with admission by **measured** count rather than estimate: Experiment E at all nine real-deal n = 4
coordinates, four tricks out.

**The result** (`separation_n4_2026-08-14.txt`). **Every coordinate reached Tier 1** — the authority gate MET, R6
step-determinism HELD, R1 solver identification HELD, and **R7 held at all nine**, which is **Lemma N confirmed to
the step**: the whole-fiber revealed charge equals the quoted tree-v0 count exactly, a same-traversal comparison.
The verdict set is genuinely mixed, and that is what makes it maximally informative:

| outcome | coordinates | what it is |
|---|---|---|
| **SEPARATED** | h1 root 11, h4 root 65, h5 root 55, h8 root 55 | root action certified against every competitor; **member-not-set** verbatim on each |
| **EXACT NEGATIVE** | h0, h2, h6, h12 | **the branch's first**: Corollary E4.1(3) fired for real |
| **NOT PRICED** | h9 | a stop that is a result, with its exact count printed |

**The exact negatives are the news.** A NOT-SEPARATED pair here is not "this run's candidates were too weak" — it is
the exact statement Q^H(a⋆) < U_a, and by Corollary E4.1(3) that proves **no candidate set whatsoever** separates
the pair under relaxation C at that coordinate. The remaining lever is a gluing cut. The failing gaps are small:
the tightest is **9557/554400** at h2 (the tied pair 53 against 54, and symmetrically 54 against 53), with h6's
**8524657/479001600** next — the one a gluing cut would later close. Experiment D of the decision-sparse program
consumes exactly these, and its inputs had been empty since it was designed; they are not empty now.

**h9 is the informative stop.** Its extraction map was measured at **517,562,322 partition states** — 2.7× the
raised cap P_max v2 = 192,000,000 — by a **COMPLETED count-only pass**, so the primal pipeline never ran and no
verdict is reported. The count is typed as an exact computational observable of the declared traversal: never an
information value, never a decision width, never a cost claim.

**The real-deal fence travels where it bites.** Three coordinates — h2, h5, h8 — carry the marker inline on every
pair row and every verdict line, SEPARATED and NOT SEPARATED alike, because at those the void-filtered fiber is a
proper subset of the full 34,650 (23,100; 14,700; and 1,200 respectively). The other six carry the full fiber. The
column licenses nothing either way: it is not a belief, not a weight, not an error bar, and no verdict in the file
is conditioned on it.

**Mechanisms, all green.** Resume-validation PASS (h0 recomputed whole-call, all non-timing fields equal the loaded
records); the DS-A36 deterministic block **byte-identical** fresh versus resumed; **36 of 36 units** checkpointed and
reloaded. The timing block is provenance only and every line under W ≥ 2 is contended, so nothing in it forms a
ratio and nothing is quoted.

---

## S6i — 2026-08-14: the lay-down catalogue and the four-laydown theorem

**The question is family lore.** Jason's family reached, by hand enumeration, the belief that a single deal holds
**at most three** lay downs — a *lay down* being a hand you can put on the table because it takes every trick. The
directive was to formalise the term and settle the conjecture.

**Theorem LD** (walt-math) is an **exact characterization**, and it is two bitset tests: a hand is a lay down **iff**
**(L1)** its top trump run is at least as long as the outstanding trump set, and **(L2)** every non-trump's threat
lies inside trumps ∪ hand. Two corollaries matter for orientation: **every lay down holds at least 4 trumps**, and
**T1-draw is a strict inner class** of it. That second one settles a question Jason had raised directly — his
missing-6:5 hand **is** a lay down, because banking a non-trump needs a second trump to hide behind, which is
precisely what (L1) measures.

**The catalogue** (`laydown_2026-08-14.txt`, `laydown_catalogue_2026-08-14.txt`, 234 ms). **Exactly 301 lay downs per
declaration**, 2,107 (hand, declaration) pairs. (LD-R1) held: the `PipTrump(6)` count came back at exactly 301
against LD-A9(ii)'s independently derived closed form — a receipt that could have failed. (LD-R2) held: all 294
freeze-47 T1-draw members are present under their own declarations and the containment is **strict at every
declaration**, 42 of 301. (LD-R3) held: the LD plan swept every trick in every world against **every** field
behaviour at the declared reduced analogues, up to 362,880 adversarial leaves, zero tricks lost. (LD-R4) held: all
seven per-declaration counts equal, which is **Corollary LD-fold** receipted rather than an observed regularity —
the lay-down predicate is declaration-fold invariant, so the seven-fold constancy is a theorem, and it is
additionally the cheapest available probe of the implementation risk below, since an implementation whose ranks
were not functions of the pip order would break the equality.

**Phase 2: NO FOUR-LAYDOWN DEAL EXISTS** — exhaustive over the complete catalogue, from every full-suit anchor,
every declaration triple, every disjoint pair, with the forced fourth hand tested under every remaining declaration.
**The family's ≤ 3 conjecture is PROVED**, and the maximum of 3 is **exhibited**: blanks `[00 10 11 20 30 40 50]`
under `PipTrump(0)`, twos `[21 22 32 33 42 44 62]` under `PipTrump(2)`, fives `[51 52 53 54 55 65 66]` under
`PipTrump(5)`, with the leftover fourth hand `[31 41 43 60 61 63 64]` unconstrained.

**Two caveats travel verbatim.** The question is **combinatorial, not a situation**: in a dealt hand only the bid
winner declares and only one seat leads trick 1, so four lay downs can never be *realised* together — the question
is whether the 28 tiles can be partitioned so that each hand **would** sweep if it were the one to declare and
lead. And **T1-A12's risk carries in full, sharper here**: Theorem LD is a claim about `rules.rs`'s rank, tier,
follow and compelled-follow semantics, and the probe computes its own evidence from that same implementation. The
tier order, `DOUBLE_TOP`, the effective-incidence subtraction, the compelled follow and `threat` must be checked
against the rules package by a reader before any of this is cited outside walt. Until then everything here is
proved **relative to walt's implementation of the rules and not relative to the rules**.

---

## S6j — 2026-08-14: the rule-economy probe at the n = 4 carrier

**The question.** S6e exercised the primal half of the economy claim at grade 3, where the candidate could still be
priced through a materialised information-partition map. At n = 4 that map is exactly what broke — so: can a
four-word rule be priced **map-free**, by a walk that reads only `(record, legal)` and holds no state, and does it
still certify? Adjudicated as RW-A1..RW-A8, freeze 49.

**Before any code ran, walt-math filed a verdict the previous pass had left on the table.** From S6h's own numbers
alone, h9 is **NOT SEPARATED at either H-optimal action**, binding margin −2116837/8870400, so by Corollary
E4.1(3) no candidate set whatsoever separates that coordinate — the **worst margin of the nine**, an exact negative
derived from Q^H and U with no new computation.

**(RW-R2), blocking, HELD**: at the declared shared ground the rule walk and the materialised map price
**identically** at the callback and through the 50,712-state map. Only after that was any h9 number quoted.

**THE RESULT** (`rule_economy_n4_2026-08-14.txt`): **every coordinate where separation is possible at all is
certified by a four-word rule.** P2 greatest-tile separates at all four positive-margin coordinates — at **h1 with
economy gap exactly ZERO** — and at h4 all four rules separate. The exact-solve seed was never needed anywhere
certifiable, at trick 4, on real deals.

**At h9, Jason's dumb-heuristic bar is set.** The exact route cannot price there at all, so the question becomes how
close a rule gets to an optimum it cannot certify against. **At the two H-optimal actions** — 41 and 54, which are
the ones the separation question turns on — the best rule (P2 greatest-tile) lands within **1202339/8870400** of the
exact optimum, about 0.136 of a trick. That is the bar, and it is stated at the binding actions on purpose: rule
gaps elsewhere at h9 go lower (down to 177253/3326400 at action 61) and lower gaps at non-binding actions buy
nothing. The rule walks at the binding actions reach 37M–105M states in **O(1) memory**,
against the 517M-state map that broke the cap — which is what "map-free" buys, stated as a reached-count and never
as a cost claim.

**The typing is the discipline here, and nothing is conflated.** At the five negative-margin coordinates the
separation column is a **receipt of Corollary E4.1(3)** — a theorem about what no candidate can do, read from the
filed Q^H and U — while the **gap column is a genuine measurement everywhere**. A rule failure is
**candidate-failure, never class-failure**; the exact negative is never obtainable from a rule failure. And the
run's exact side is **quoted, not recomputed**, so EC-A13's fence stands unchanged: this tests the **primal** half.

---

## S6k — 2026-08-14: the fusion-tax probe, and the first gluing-cut closure

**The question.** S6h had produced four exact negatives — coordinates where Corollary E4.1(3) proves no candidate
can ever close the pair, leaving a gluing cut as the only remaining lever. Freeze 38 had been reserved for that
lever since the intake audit. An external note arrived on the same day proposing exactly the missing machinery, was
adjudicated as **FT-A1..FT-A29** — eight named results delivered with it, four lemmas, two propositions and two
corollaries — **filled freeze 38 v1**, and was built and run the same night.

**THE RESULT: the first gluing-cut closure in the branch.** At h6 — pip 4, hand `[11 40 43 53]` — the first-layer
tax excludes competitor 11 **strictly**, with surplus `4930081/479001600`. Composed with S6h's frozen treatment-C
rows for the other two competitors under Lemma FT-mix and receipt (FT-R8), that gives **Opt^H(h6) = {40}**,
uniqueness — the two-sided architecture (one lawful plan, one information tax per competitor) closing end-to-end
for the first time.

**The mandatory sentence travels with that verdict, verbatim** (FT-A25(vi)): *this coordinate's optimal set was
already determined by the filed `Q^H` column; what this verdict demonstrates is that the two-sided proof
architecture now closes here, and that the lever was a gluing cut and never a better candidate — which is exactly
what Corollary E4.1(3) proved was the only lever available.* The closure **could** have failed — it did at the
other eleven pairs — so the run is a genuine test; the **conclusion** could not have come out otherwise, so it is
not evidence about the game.

**The other eleven pairs, and the run's other findings** — the reveal-delay ladder, the exactly-two-rung
decomposition at grade 4, all ten tied pairs NOT CLOSED with shortfall exactly Δ², the 4.49% tax sparsity with its
selection fence, the binary fusion cores, and (FT-R1)'s independent confirmation of h9's filed U — are the
architecture's story and are owned by
[walt-decision-sparse](walt-decision-sparse.md#the-fusion-tax-chapter-s6k-2026-08-14). **The next open target has a
name: Δ².**

---

## S6l — 2026-08-14: the second-rung probe, and the escape actions

**The question.** S6k had named Δ² as the next target and proved it was exactly the part of the fusion gap the first
rung cannot reach. Asking Pro for it produced the second external note of the same day; it was adjudicated as
**SR-A1..SR-A36**, delivering **eight** named results, fixing **freeze 51** and clarifying freeze 38's clause (d) as
**v1.1(d)** — a clarification with no new content, since the reveal-delay ladder already had k = 2 as a member and
its validity was discharged for the whole family at once. Freeze 38 stands at **v1**; **v2 was not opened**.

**Before the build, a fence that reshaped it.** **Proposition SR-degen**: at grade 4 the second rung closes every
binding pair *unconditionally* — L ≥ U^(2) follows from two already-filed columns — so **no grade-4 experiment can
test whether rung two closes anything.** The probe was therefore re-specified as an instantiation receipt and a
measurement of structure, with **no closure verdict reported at all**. A second a-priori fence went with it: at grade
4 the seat holds two tiles at the second frontier, so every positive-tax minimal core has size exactly 2 **by
arithmetic** — the received note's open question about core sizes is unmeasurable here, and the run may not be
reported as answering it. Both are the S6k lesson caught *before* a run instead of after one.

**The result** (`second_rung_2026-08-14.txt`). Four units — two at h2, two at h9 — **all ten receipts HELD at every
unit**, with the reduced-grade cross-check running **blocking** before any carrier number existed. **Arm 2 completed;
no declared stop occurred.** walt-math re-derived every quantity independently from the committed rows and found
**zero deviations at all 3,300 states**. The ladder identity reproduces the filed Q^H at both coordinates exactly and
the slack–tax interchange law
reproduces the filed Δ², and both recover the rung-one columns as by-products of a depth-two traversal.

**THE FINDING, which nobody asked for: escape actions are present.** The one genuinely open question the probe could
decide was whether the minimising first action ever leaves the rung-one optimal face. It does — **36 of 330 states at
h2, 498 of 1,320 at h9** — making this the first measured instance of *policy adjustment* in the branch. The
pre-declared consequence now binds: **every future rung-two lower witness must cover every first action**, not the
tie-broken optimiser and not even the complete optimal face. **Proposition SR-loc** prices the alternative exactly: a
witness taxing only the optimal face would have overstated the true tax by **4.0459%** at h2 and **11.7881%** at h9,
and since that quantity is an upper bound, such a witness *claims to have shaved more than it did*.

**The census is not a rate, and the caveat travels verbatim.** At h2 all 36 escapes carry **one signature**; at h9 the
escape action is the single tile 61 at every one of the 498. These are **one structural phenomenon reached by many
field continuations, not many phenomena** — the honest statement is *"escape occurs, at these coordinates, with this
structure"*. The selection fence binds in its sharpest form yet: **five coordinates chosen by negative binding margin
are a carrier, not a sample, and the selection criterion is correlated with the quantity being described**; two of
the five are in scope, and neither the escape rate nor the tax density may be read as a distribution over coordinates
or hands. Nothing causal is claimed and nothing measured at grade 4 is quoted for trick 1 or the opening.

**h9 gets a second independent reconstruction — and NOT PRICED is untouched.** The depth-two decomposition reproduces
h9's filed Q^H exactly, by a different traversal and a different theorem; with S6k's reconstruction of h9's U, both
of its filed columns have now each been independently reconstructed once, by two different routes. The clause that
must travel: **h9's NOT PRICED label stands verbatim and is not weakened by any of it.** NOT PRICED is about the
*primal* pipeline, and reconstructing a value twice on the *dual* side says nothing about it — **a cross-check is not
a witness.**

**Two closures.** **FT-A28 is fully discharged**: the deferred frontier digest is carried by all four units, closing
the named across-process residual *by receipt rather than evidentially*. And the blocking reduced-grade check did
work no filed number could do — at grade 3 every second-frontier state is forced, and the ladder collapse held
against **the engine's own H operator**, the only check in the build whose answer was known by proof rather than by a
filed rational. Its point, in walt-math's words: *a build whose strongest checks are all against filed numbers can be
self-consistently wrong; this is the check that is not.*

**Two defects the build found in itself, both before any carrier number existed.** The probe's own streaming SHA-256
clobbered its buffered length across calls; a published known-answer self-check caught it and the run hung rather
than filing a wrong digest. That check turns out to be **load-bearing for the FT-A28 discharge**: a mis-buffering
hash is still deterministic, so two runs would still have agreed and the digest receipt would have been *green and
worthless*. The second was an `a⋆` selection that, at a tied coordinate, would have compared an action with itself in
the printed pair-typing line — verified at adjudication time to have touched **no receipt and no number**, but typed
as a carrier-identity defect rather than a typo, and notable because the obvious guard (a sign assertion) provably
would **not** have caught it. Both were found by checks the builder wrote against itself and were reported in full
rather than quietly fixed. The chapter also records its **first pass with no specification conflict** — informative
only because the previous three defects were found by the build rather than the adjudicator, so a builder that
reports conflicts when they exist makes the absence of a report mean something.

**Where it leaves the track.** There is **no rung three at grade 4**, both rungs are now exact, and SR-degen bars
grade 4 from testing closure — so the carrier that produced the whole FT/SR sequence is exhausted as a test-bed. The
next real question needs a **longer ladder**, which is where the trick-1 program's three standing obligations stop
being a distant destination and become the binding constraint. Full architecture account:
[walt-decision-sparse](walt-decision-sparse.md#the-second-rung-chapter-s6l-2026-08-14).

---

## S6m — 2026-08-14: the feature-fee audition

**The question, and where it came from.** Jason derived a control-flavoured feature at the table, reasoning through
one real hand, and asked for a short test of whether it bites. The adjudication granted it as the right experiment at
the right moment — the cheapest one grade 4 has left — and re-shaped it: **measure which structural features
approximate the perfect penalties, on a carrier where the perfect answer is already filed**, before any counting
problem is faced. The typing on the provenance is stated up front and is not a formality: **table reasoning is a
perfectly good source of a hypothesis and no kind of evidence for it.**

**Two of the four requested elements did not survive contact with the mathematics, and both failures were useful.**
One of the two candidate features is a function of the world alone, and **Proposition FF-blind** proves an
action-blind fee removes exactly zero — so auditioning it live would have burned the run rediscovering a theorem. It
was repurposed as a **null control** with a pre-declared exact prediction of zero. And the requested centring was
unsound as written: the fee must be centred **per action**, not per state, or the penalty theorem's hypothesis fails
and the number bounds nothing in either direction. That was the single most likely way the build could have produced
a plausible wrong number.

**The reading was fixed before any number existed.** **Proposition FF-oracle**: optimising the coefficient per state
spends one free rational per information state, which is a lookup table rather than a feature basis — so a **low**
capture refutes conclusively while a **high** capture establishes nothing about a usable family and licenses exactly
one follow-on. Both outcomes are results; they are not results of equal strength.

**The first run exposed a defect in the frozen feature list.** Its no-outstanding-trump fallback was attached to all
three features, but only two reference the boss-trump holder; the third is well defined without one, and that is
when it is most interesting. **Six of the twelve (feature, unit) cells were vacuous by construction**, typed as
**unmeasured, not zero**. Freeze 52 went to v1.1, v1.2 and v1.3 in response. What made this catchable from the
committed file rather than by re-running is **Proposition FF-degen**: zero breakpoints is exactly vacuity, so a zero
capture with many breakpoints is a measurement and the same zero with none is a tautology.

**Jason's feature is REFUTED where it has a domain.** At h0's **574 leading states** — the only part of this carrier
where a boss trump survives to the frontier — it was genuinely swept across 23,016 breakpoints, and its oracle-θ
capture over those 574 states is **3,673 ppm**, about a third of one percent. A family that cannot break 0.37% with
574 free parameters cannot break it with one. **The scope discovery is worth more than the refutation**: a
boss-keyed feature has a *shrinking domain precisely as the hand simplifies*, which is the opposite of where a cheap
witness is wanted. And the fence travels — the feature was priced as a fee against one specific object, and losing
that job is not a verdict on the reasoning that proposed it.

**The sibling feature — *can my action be beaten?* — did bite, and the corrected re-run separates it into three
regimes.** Every figure carries the state set it ranges over, which is a binding rule of this chapter:

| where | oracle-θ capture | reading |
|---|---|---|
| h0, **574 leading** states | **76.4628%** | the measurement the chapter rests on |
| h0, **758 following** states | **29.2679%** | same feature, same unit, same sweep |
| h2, **216 swept** states, at each of the two units | **exactly 0**, with **3,126 breakpoints at each** | **refuted conclusively** |

**h2's zero is a refutation, not an empty test, and the breakpoint count is the whole of the difference.** In the
first run the same zero came with *zero* breakpoints and was a tautology. **The same number means opposite things in
the two files.**

**THE RESULT: one shared coefficient does almost all of the work.** Over h0's 574 leading states, a single pooled
θ* = −56/45 gives **76.3608%** against the per-state oracle's **76.4628%** — **about 99.87% of the oracle survives
collapsing 574 free rationals to one.** The per-state optimum takes only 27 distinct values, none zero, over a narrow
range. This is the first time in the branch that a **small** fee family has been shown to carry a first-layer tax.

**The sentence worth carrying out of the chapter**, and the only comparison here free of between-coordinate
confounds because it is within one unit and one pass: on **the same 574 states**, the two action-conditioned
candidates return **0.3673%** and **76.46%** — a ratio of about **208×**. *At h0's leading frontier states, the
first-layer Jensen gap is substantially aligned with whether the focal seat's action can be beaten, and essentially
not at all with whether the boss-trump holder can follow it.*

**What none of it establishes**, and the fences bind hard here because a capture fraction reads exactly like a rate:
one coordinate and one part of it; two coordinates selected by negative binding margin, so **a carrier and not a
sample**; **nothing quoted for trick 1 or the opening**, which is the fence most at risk given that trick 1 is the
entire motive; **no grade-4 verdict moved and none could**; and it prices the **first layer only**. A 99.87%
shared/oracle ratio at one coordinate part is a licence to test at a second coordinate, not a licence to believe.
**Nothing further is commissioned** — a third coordinate is a new carrier and wants its own freeze and its own
pre-declared readings. Full architecture account:
[walt-decision-sparse](walt-decision-sparse.md#the-feature-fee-chapter-s6m-2026-08-14).

---

## S6n — 2026-08-14: the fee-correlation chapter, and why a fee bites

**The question.** S6m left an unexplained exact zero: the surviving feature captured about three quarters of the
first-layer tax at h0's leading frontier and **exactly zero** at h2, twice over, with thousands of breakpoints
proving the fee genuinely varied. S6m had made the diagnostic that would explain it optional; **this chapter reverses
that call**, on the ground that an exact rational identity holding at 432 independent states is the most informative
unexplained fact in the branch and the instrument that would explain it costs seconds.

**It also declined the obvious next step, and the reason is the interesting part.** A third coordinate — the
experiment S6m named — was **deferred, not refused**: a third observation taken before the mechanism is measured
*enlarges* the confound that the previous chapter identified rather than resolving it. What was commissioned instead
is the correlation diagnostic on the carrier already in hand.

**The instrument. Proposition FC-drop** turns the earlier zero-test into a **quantitative lower bound on capture**
computable with no minimisation at all: capture is at least **correlation times reach** — one directional slope,
measuring how far the feature leans on the clairvoyant choice, times one breakpoint distance, measuring how far the
fee can be pushed before that choice starts changing. It is the first quantity in the branch that predicts where a
fee can bite **without computing whether it does**. **Corollary FC-null** gives it a null control whose value is
fixed by theorem rather than by a filed number.

**THE ANSWER: the zero is TIE-DRIVEN, and it is unanimous.** At both h2 units, over each unit's **216 swept states**,
the two slopes **strictly straddle zero at every state, with neither slope zero anywhere**. The pre-declared
alternative — genuine orthogonality between the feature and the clairvoyant choice — is **refuted at every state of
the carrier**, not merely unsupported, which is the stronger of the two ways a pre-declared reading can fail.

**THE MECHANISM, which is what makes it a result rather than an observation. Proposition FC-width**: the width of the
subgradient is exactly the mass-weighted spread of the feature across the clairvoyant tie. **Without ties that
interval is a point**, so a zero capture would demand an exact rational identity — not plausible at 216 states twice
over. **With ties it has positive width**, and zero capture requires only that zero fall *inside* it, which is robust
rather than coincidental. h2 carries a non-singleton clairvoyant argmax at **236,784 of the 362,880 (state, world)
arrivals at each of its two units' 216 swept states (65.25%)**, against **59,776 of 266,132 at h0's one unit over
its 1,332 swept states (22.46%)**; h2's straddle holds at
216 of 216 states per unit where h0's fails at 1,252 of its 1,332, all four counts at the beatability feature. **Mechanism and measurement agree.**

**THE CONSEQUENCE FOR THE PROGRAMME, and it is the most quotable thing in the chapter.** The h2 refutation **was
never about that feature.** No fee keyed on the clairvoyant choice can be expected to bite where the face is widely
non-singleton, because FC-width widens the subgradient for *any* such feature. So the branch now holds a **pre-fee
screening statistic** — the argmax cardinality profile, a property of the coordinate's world structure, measurable
**before any fee is built**. The fee route **is not to be expected to bite, robustly so**, at such
coordinates, and that is not a fixable defect of any candidate. The modality is exact and the ruling corrected its
own closing text for it: the width result makes zero capture **robust**, not positive capture **impossible** — a
feature whose mean slope exceeded the half-width would still bite despite widespread ties. The programme's first question at a new coordinate is no longer *which feature* but **"is
the clairvoyant choice pinned down enough for any fee to bite"** — a question cheaper than building a fee, and one
that now has an exact statistic. It is the first thing this branch has that says where **not** to spend the attempt.

**How tight the screen is, in the exact words the ruling binds.** Over the **1,252 straddle-false states of h0's one
unit at the beatability feature**, the bound is **attained** — it equals the frozen captured amount — at **258 of
them (20.61%)**, and the summed bound recovers **14.873%** of the summed capture. **Proposition FC-tight** says what
those 258 are: exactly the states where the descent is a single linear piece. Three things must travel with those
figures and are stated here because the wording matters:

- **The bound is a lower bound at every state, the 258 included.** It is never *exact* as a property of the
  functional anywhere. "Exact at 258 states" invites the reading that the screen predicts capture a fifth of the
  time, which is precisely what this chapter exists to prevent.
- **Which states attain it is not knowable without the captured amount** — the very quantity the bound exists to
  avoid computing. So the 258 is a fact about the distribution of the gap and **never a usable property of the
  instrument**: in use the screen is exactly as weak as its aggregate 14.873% and no weaker.
- The reach is measured to the nearest *candidate* breakpoint, which may fall short of a true kink, so the bound
  carries **a second, independent conservatism** beyond the descent running past it. Both make it smaller, never
  larger. This belongs with any citation of the 14.873%.

The bound is **one-sided**, and that is the whole of what may be said about it: **a positive bound PROVES a fee
bites at that state; a zero or small bound proves NOTHING.** No false positives, unbounded false negatives. The
substantive point underneath is unchanged — **screening and estimating are different jobs** — but the ruling
replaced every graded phrasing of it after two attempts, because *"exact"* and *"usable"* are each true under one
reading, invite a stronger one, and are silent about which. **The house form: do not grade an instrument — state
what follows from a positive reading and what follows from a negative one.** A sentence built that way cannot be
excerpted into something stronger than itself, which is the only durable protection, since escorts do not travel
and sentences do.

**The other results, each with its full scope named.** The **null control** held with both slopes exactly zero at
**all 1,764 swept states across the three units**, blocking and first, its answer fixed by theorem. The **non-null
pairing receipt** — which exists because at h2 the pairing had held only by luck, and is now required by design —
held over **h0's 574 leading states at the beatability feature**, with slopes not both zero at **518** of them. The
**graded boss-keyed feature is not refuted**: over **h0's 574 domain-nonempty swept states** its bound is positive at
**322**, so its capture is **proved positive by theorem, with no sweep**. But the calibration the adjudicator
attached governs how that reads: **the already-refuted binary form is straddle-false at 374 of the same 574 states —
more states than the graded form — and that binary cashed out at 0.367%.** **Proved-positive and negligible are
entirely compatible, and here that is the likely reading.** The pre-declared sweep was **declined**.

**The fences, undiminished.** Three units at two coordinates chosen by negative binding margin are **a carrier and
not a sample**; **grade 4, so no verdict moved and none could**; the whole chapter prices the **first layer only**;
and **nothing is quoted for trick 1 or for the opening** — which binds hardest here, because a screening functional
is *for* trick 1 and multiplicity is now the variable everyone will want to extrapolate. And nothing whatever is
claimed about whether Jason's reading of that hand at the table was correct. Full architecture account:
[walt-decision-sparse](walt-decision-sparse.md#the-fee-correlation-chapter-s6n-2026-08-14).

**Four disciplines this chapter minted**, all from defects caught rather than principles proposed, and all now
binding on later chapters: a receipt's scope names **every** dimension it ranges over, not just the state set;
receipt what the probe recomputes each run and leave a documented one-time audit where the object is fixed source;
emit over the full set, read over the meaningful subset, and **name both**; and **independent verification means an
independent predicate, not an independent party** — two agents running one grep are one check, however many agents
there are. That last one was minted the hard way: a check written with a regex anchor where a literal was meant
matched nothing and returned a clean, confident zero violations. **A wrong predicate returned exactly the answer
being hoped for**, and it was caught by implausibility across several simultaneous queries rather than by suspicion
of the predicate.

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
- **S6g, the corner-gap range.** The LOG records "the 13 exact corner gaps (4 .. 92/15)". The results file's
  thirteen t1-B rows are 4, 5, 89/15, 6 (×4), 92/15 (×5) and **19/3**, so the widest gap is 19/3 ≈ 6.333, not
  92/15 ≈ 6.133. **The results file governs**; the page uses 4 to 19/3.
- **S6h, the tightest failing gap.** The LOG records "failing gaps as tight as 8524657/479001600". That is h6's
  gap; the results file's tightest exact negative is **9557/554400 at h2** — over the common denominator
  479001600 that is 8257248, strictly below h6's 8524657. **The results file governs**; the page uses h2's.
- **S6j, the h9 heuristic bar.** The LOG records "best rule within 1202339/8870400 (~0.136 tricks) of the exact
  optimum" without naming where. It holds **at the two H-optimal actions**, which is what the bar is about; the
  smallest rule gap anywhere at h9 is 177253/3326400, at the non-binding action 61. The page states the
  qualifier, because an unqualified reading of the number is false.
- **S6k, "ten of twelve".** FT-A25(vi)'s surrounding commentary says the closure "did [fail] at ten of twelve
  pairs". Its own closing note counts one CLOSED, **ten tied NOT CLOSED and one untied (h0) NOT CLOSED** — eleven
  failures of twelve, the tied ten plus h0. The commentary appears to count only the tied pairs. The **mandatory
  sentence itself is unaffected** and is quoted verbatim above; the surrounding count is restated here as eleven.
- **S6i, (LD-R4)'s status.** FT-A16(ii), ruled later the same day, says "**(LD-R4) remains owed**" and that "2,107"
  is therefore licensed "as a proof plus one receipted count, not by seven receipted counts". The laydown results
  file prints **(LD-R4) — HELD**, with all seven per-declaration counts equal and 2,107 explicitly receipted. The
  receipt was added at LD-A12 and evidently ran before the FT section was written; the FT sentence is stale rather
  than contradicted. What is **not** affected either way is LD-A13's open item: (LD-R4) probes the
  implementation-versus-corpus risk and does not discharge it.
