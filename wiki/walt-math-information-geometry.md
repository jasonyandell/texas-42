# walt mathematics — information geometry and the width ladder

[Home](Home.md) · owns: the predictive-closure and policy-geometry objects of the
walt branch — Lemma R, Lemma G, Proposition G-flat, Definition E9, and the
cardinality ladder that keeps them apart · Sources: `walt/CENSUS-RULINGS.md`
§§ "Predictive-rank probe rulings", "Policy-geometry probe rulings";
`walt/math/decision_sparse_exact_solving_v0.1_errata.md` §8.4. Related:
[the reference map](walt-math-reference.md),
[structure and transport](walt-math-structure-transport.md),
[decision-sparse witnesses](walt-math-decision-sparse.md),
[the freeze register](walt-math-freezes.md).

> **Tier: EXPLORATORY throughout**, below every tier on
> [Home](Home.md#evidentiary-tiers--never-promoted-never-blurred). No number on
> this page licenses any runtime or tractability claim of any kind.

These are the lemmas that answer "how many objects does the seat actually need
to carry?" Two of them settle a question the branch had hoped was open, and
settle it negatively for a structural reason that is exactly identifiable. That
is the value of the group.

---

## Lemma R — three continuation closures, and the separating-observation degeneracy

**Setup (all hypotheses load-bearing).** Fix an information interface *i* of the
void-free rung at grade *n*, the uniform-legal field at the three hidden
offsets, the count-free valuation, and an observation contract.

**The three graded spaces of rational functions on X_i.**

1. **Event closure V^ev** — terminal seed = span{1, terminal readouts}; closed
   under per-event preexpectations.
2. **Observation closure V^obs** — as (1) with observation-aggregated
   preexpectations.
3. **Value closure V^val** — terminal seed = span{terminal readouts}, which
   **for the count-free expected-trick contract is the zero space**; closed
   under observation-aggregated residuals. **The constant 1 is not a generator.**

**The claims (exploratory).**

- **(a)** V^val ⊆ V^obs ⊆ V^ev, each closed under the residuals used to generate
  it, each admitting closure matrices at its own refinement. The policy
  evaluation and the root pairing use the observation-aggregated matrices only.
- **(b)** Every lawful continuation policy has its value function in V^val, and
  the evaluation runs over a basis of V^val **with no normaliser**.
- **(c) The degeneracy.** Suppose the observation contract is such that a
  complete continuation record determines the latent point — as it is here, for
  every contract at least as fine as "the four played tiles with their seats",
  since every tile is played before the hand ends and every play is publicly
  attributed. Then V^ev_i = V^obs_i = the full function space: the predictive
  dimension is **exactly |X_i|**. More generally this holds for **any closure
  whose terminal seed contains a nonzero constant function**.
- **(d) The ladder.** Under (c), the trick-count-distribution contract and its
  predicate enrichment have predictive dimension exactly |X_i|, because the
  point-mass terminal readout of a distribution contract *is* the constant
  function 1. **The expected-trick contract is not covered by (c): its terminal
  seed is 0.**

**Full statement and proof:** `CENSUS-RULINGS.md` § "Predictive-rank probe
rulings", under "Lemma R (three continuation closures, and the
separating-observation degeneracy)".

**Two consequences the builder must respect.**

1. **The repair is on the test-family side, never on the observation side.**
   Coarsening the observation to escape (c) would change the information model
   and therefore the operator — theorems for one operator do not transfer to
   another — and would change which policies are lawful.
2. **(c) is not a defect of the v0.6 document**, which declines to claim small
   rank. It is a fact about this game's observation structure, and it is **the
   predictive analogue of Corollary S-rigid**: the compression the design hoped
   to buy is absent for a structural reason, and the reason is exactly
   identifiable.

**What it does not do.** It does not promise that dim V^val is small. That is
the measurement, and the rulings are careful to leave it genuinely open. The S6a
run then measured grade-3 dim V^val ∈ {1461, 1492, 1680} against |X| = 1680 —
one coordinate at **full rank exactly** — and the Gate-B payoff was recorded
REFUTED. That measurement is a probe result at the exploratory tier and does not
change the lemma.

**Reporting discipline (R-A16, R-A24).** The degenerate contracts are reported
as **THEOREM rows** citing Lemma R(c)–(d) and are **not run**. A run returning
anything else for them is a stop-and-report bug, because Lemma R(c) says what it
must return.

---

## Lemma G — backward pruning: what it preserves and what it destroys

**Setup.** At a root interface under a root action, value vectors compose as
V_ρ = g + Σ_o T_o(V_{ρ_o}) with each T_o a **positive** linear operator and the
continuation chosen independently per observation. For a finite vector set S,
write max(S) for its Pareto-maximal elements and

> **Exp(S) = { v ∈ S : v is the *unique* maximiser of E_β[·] over S for some
> belief β }**.

**The six claims (exploratory).**

1. **Monotone composition.** v ≤ w pointwise ⇒ T_o(v) ≤ T_o(w) pointwise, and
   the immediate term does not depend on the continuation.
2. **Frontier preservation, ties included.** Building from the Pareto-maximal
   successor subsets reproduces the root Pareto frontier **exactly, as a set,
   not merely in cardinality**.
3. **Incremental pruning is exact, and is mandatory.** max((max(A ⊕ B)) ⊕ C) =
   max(A ⊕ B ⊕ C). Without it the fold materialises the full product of
   successor set sizes — 2^k(a) at grade 3 — before any pruning can occur, so
   per-interface pruning alone does not make the computation feasible.
4. **Exposure.** Exp(S) is the **unique minimal** subset whose upper envelope
   equals that of S; Exp(S) ⊆ max(S); Exp(max(S)) = Exp(S). So Pareto pruning
   preserves the exposed set exactly, over the whole simplex, boundary beliefs
   included. The **weak** variant — "attains the maximum for some β, ties
   allowed" — is a strictly larger count in general and is **not** preserved.
5. **Convex dominance.** v ∉ Exp(S) iff there are convex weights λ on S∖{v} with
   Σλ_w w ≥ v pointwise. Pruning by this rule preserves Exp exactly and is
   strictly stronger than Pareto pruning — **therefore it destroys the Pareto
   frontier count.**
6. **The negative.** As filed: "No pruning rule preserves N_vec. A run that
   prunes cannot report N_vec." **Narrowed in place by DS-A26** to the two rules
   actually in use — Pareto and convex-dominance — since rules discarding only
   duplicates preserve the count trivially. The binding consequence is
   unchanged: **a run
   that prunes cannot report that count** unless it maintains a separate
   complete unpruned accounting, which is the thing the pruning was adopted to
   avoid.

**Full statement and proof:** `CENSUS-RULINGS.md` § "Policy-geometry probe
rulings", under "Lemma G (backward pruning: what it preserves and what it
destroys)". Clauses (2) and (3) are restated with proof, self-contained, as
**Theorem E6.2(a)–(b)** in errata §6; clause (6) is narrowed there as E6.2(c)
under DS-A26 — the claim is about the two rules actually in use, since rules
discarding only duplicates preserve the count trivially.

**The one thing Lemma G does not license** — the sharpest caveat in the group,
condensed here with the ruling's own fiber notation spelled out in words:
"Exp is preserved for the **value function**, not for
the identity of every optimal policy: at a belief on a face, a Pareto-dominated
vector can tie for the maximum and is then optimal without being exposed. A seat
whose belief has support strictly inside the declared cost domain — which is the
real seat's situation — is exactly that case. N_exp is therefore a statement
about the declared cost domain's value function and **never a count of the
strategies any seat needs**."

**What it binds.** The redefinition of the exposed count as the *unique*-maximiser
form (PG-A4) — the design's "some belief" version was a silent shrink; backward
pruning accepted with the incremental fold mandatory (PG-A5); convex-dominance
pruning lawful only in a pass reporting exposure alone (PG-A6); the unpruned-only
rule for the distinct-vector count (PG-A7); and PG-A8(i)'s mandatory receipt —
compute the frontier both ways and **assert set equality, not merely equal
cardinality**.

---

## Proposition G-flat — grades 1 and 2 carry no policy geometry

**Statement (exploratory).** At a focal-lead root information interface of grade
*n* in the measured domain, the focal seat has exactly *n* plays, one per trick,
and at the final trick exactly one tile remains, so that decision point is
forced. Hence:

- **n = 1:** the root action set is a singleton; every policy cardinality is 1.
- **n = 2:** for each root action the whole continuation is forced; every
  cardinality is 1.
- **n = 3:** the only free choice layer is **trick 2**, whose focal information
  states are indexed by the trick-1 public record, each leaving one or two legal
  tiles. Therefore N_pol(a) = 2^k(a), with k(a) the number of records offering
  two legal tiles.

**Full statement and proof:** `CENSUS-RULINGS.md` § "Policy-geometry probe
rulings", under "Proposition G-flat".

**Three consequences the builder must carry.**

- **(i)** The grade-1 and grade-2 rows are **receipts, not measurements**: their
  values are known in advance and a discrepancy is stop-and-report.
- **(ii)** The plan/reduced-strategy distinction is **inert at n ≤ 3**, so the
  flat product for N_pol is correct here and **would stop being correct at
  n ≥ 4**.
- **(iii)** The whole of the "astronomically large policy set" is the single
  fold over the k(a) free records at grade 3. That is where backward pruning
  earns its place, **and it is the only place**.

**What it forced.** The design's growth-ratio criterion had one usable data
point and was **REJECTED** (PG-A15), replaced by absolute bands fixed before any
number existed. And the anti-strawman line is mandatory in the results file: the
grade-1 and grade-2 rows are 1 *by the proposition* — the seat has no choice
there — and are receipts, not evidence of collapse.

---

## Definition E9 — interface-local reachable decision width

**Statement (exploratory).** For an information interface I with latent domain
X_I, let A_{I,a} be the value vectors of lawful continuation policies
**beginning at I** with first action a, and B_reach(I) ⊆ Δ(X_I) the set of
posteriors with which I is actually reached under the declared initial belief,
the fixed field, legal focal policies and positive-probability observation
histories. Then

> **W^loc_reach(I,a) = min { |E| : E ⊆ A_{I,a}, max_{α∈E} ⟨β,α⟩ = Q_I(a;β) for
> every β ∈ B_reach(I) }.**

**Full statement:** errata §8.4. Authorised by DS-A23.

**Why it exists.** This is the quantity that answers "how many policies must the
seat retain *here*", because a posterior generated by a prefix evaluates
continuations that begin where that prefix ends. The root-level W_reach(B,a) of
the parent document asks how many *root* policy vectors preserve the root
envelope over a family of later posteriors — a question no seat asks. A pointer
marker at errata §6/Theorem E6.1 records the retyping.

**Three quantities, three names, never one row.** (i) the interface-local width;
(ii) a global summary such as the max over interfaces; (iii) the size of a
single transported policy library covering all reachable interfaces — smaller
than a sum and larger than a max in general, and **a library statistic, not a
width**. The root-level W_reach is a fourth.

---

## The cardinality ladder — seven names, and the two that are one object

Whenever more than one of these is quoted, the ladder and the pass that produced
each must be stated (PG-A7, DS-A2, DS-A4). A results file that reports two names
for one object has measured one thing twice.

> **N_pol ≥ N_vec ≥ N_par ≥ W_all ≥ W_reach ≥ 1**, with
> **forced ⊂ dead (N_vec = 1) ⊂ dominant (N_par = 1)**.

| Name | What it counts | Where it may be read from |
|---|---|---|
| **N_pol** | plans (closed form, 2^k(a) at grade 3 by G-flat) | closed form |
| **N_vec** | distinct value vectors | **only** an unpruned enumeration (Lemma G(6)) |
| **N_par** | the Pareto frontier | a Pareto-pruned run; **never** a convex-pruned one (Lemma G(5)); **never** a capped one (PG-A13) |
| **N_exp** | vectors uniquely optimal for some belief (Lemma G(4)) | the exposure programme applied to the frontier |
| **W_all** | the minimal envelope over the full simplex | **identical to N_exp** — one number, one name, the other a synonym (DS-A2) |
| **W^loc_reach** | the interface-local reachable width | Definition E9; needs exact enumeration of B_reach(I) |
| **d_adv** | the affine dimension of an envelope | Definition E2 (errata §2), *not* the parent's reference form |

**The fact that ties W_all and N_exp together** is Theorem E6.1's closing line:
at the full simplex the minimal envelope is unique and equals the set of vectors
uniquely optimal for some belief, so they are one object under two names.

---

## Two standing fences on everything above

- **PG-A12, the argmax fence.** Argmax sets here are response-equality objects.
  They are not a dynamics quotient, not an r3-style class count, and **may never
  be used as a solver's state partition**. No partition claim is made or implied.
- **R-A23, the predictive-dimension fence.** A predictive dimension is a
  statement about the linear span of a declared family of continuation tests
  over a declared coordinate's void-free capacity fiber, under the declared
  field, belief, count-free contract, observation contract and grade. **It
  licenses no runtime or tractability claim of any kind.** Moment compilation is
  a separate, unmeasured experiment, and a small dimension whose moments require
  enumerating the fiber solves nothing. Numbers are coordinate-relative and are
  never quoted for the opening or for any grade not measured.
