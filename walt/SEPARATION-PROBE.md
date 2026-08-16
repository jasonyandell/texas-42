# The separation probe: root-action certification by primal and upper
# witnesses (Experiment E)
# (adjudicated design)

Status: ADJUDICATED — walt-math rulings **SEP-A1..SEP-A18** in
`walt/CENSUS-RULINGS.md` (2026-08-13) bind this build; freezes **36**
(SEP-A4) and **37** (SEP-A6) are fixed there. The design's questions,
originally numbered E-Q1..E-Q8, are renumbered **SEP-Q1..SEP-Q8**
(SEP-A: the E-Q prefix was already spent by the endgame-store section).
Standing rulings inherit everything through DS-A1..DS-A36, the errata
(`walt/math/decision_sparse_exact_solving_v0.1_errata.md`, citation rule
DS-A17), PG-A1..PG-A18, J-A1..J-A18, R-A1..R-A23. Corollary **E4.1**
(the primal ceiling and the exact negative) is stated and proved in the
SEP adjudication section pending its errata filing as §4.3. Tier:
exploratory.

## The object

For one coordinate (declared belief β = uniform over the full void-free
fiber, declared field = uniform-legal, count-free expected-focal-tricks
valuation), and for each root action a:

- **L_a — the primal witness** (Lemma E4, DS-A14): the exact value of a
  FIXED lawful information-consistent policy with root action a,
  evaluated with no maximisation at any node below the root, maximised
  over a finite declared candidate set. L_a ≤ Q^H(a) always.
- **U_a — the upper witness** (Lemma E3, DS-A7, freeze 37): the
  action-conditioned treatment-C value E_β[V*_a] — root action held at
  a, world revealed before any later focal decision. Q^H(a) ≤ U_a
  always. On this carrier the latent is ξ = ω, so treatment C and the
  document's C⁺ coincide, and the results file says so (SEP-A6(b),
  DS-A20).

**The root-action separation** (Theorem E6.4): if L_{a⋆} ≥ U_a for every
a ≠ a⋆, then a⋆ ∈ Opt^H(B) — membership in the optimal set, never
uniqueness (non-strict separation; the member-not-set caveat is printed
verbatim beside every SEPARATED verdict). Vocabulary fence (DS-A1,
SEP-A1): witness, separation, receipt — the word "certificate" does not
appear.

## The primal ceiling (SEP-A2 — printed in the results header before
## any number)

The candidate at each H-optimal action is the H-argmax policy, so by
**Corollary E4.1(2)** L = Q^H exactly and necessarily — the primal
witness is at its ceiling, L is a receipt tying evaluators together and
not a measurement, and **every separation verdict in this run is decided
entirely by the upper witness**. Results-header sentence, verbatim:
*"the primal witness at each H-optimal action is an H-optimal policy
re-priced by the fixed-policy evaluator, so L = Q^H by Corollary
E4.1(2); the separation verdict at this coordinate is determined
entirely by the upper witness."*

By **Corollary E4.1(3)**, a NOT-SEPARATED pair (a⋆, a) is the exact
statement Q^H(a⋆) < U_a: a proof that **no candidate set whatsoever**
separates that pair under relaxation C at that coordinate (SEP-A16). The
results file prints the failing gap U_a − Q^H(a⋆) as an exact rational
with that sentence — never as "this run's candidates were not strong
enough". Failing pairs are exactly where a gluing cut (Theorem E6.5,
DS-A3) would have to bite — the input Experiment D needs.

## Prior data (S6a `predictive_rank_2026-08-12.txt`, S6b
## `policy_geometry_2026-08-12.txt`; quoted, not re-asserted;
## arithmetic verified at SEP-A14)

The three canonical grade-3 coordinates (idx 0, 1299709, 2599418; pip=0
derived, never a key component; full fiber |X| = 1680; H gate MET at all
three, S6a). Per-action Q^H (count convention) and the headroom
Q^H(a⋆) − Q^H(a):

| coord | Q^H per action | a⋆ | headroom per competitor | aggregate gap V^F − V^H |
|---|---|---|---|---|
| idx=0 | 00: 53/21, 10: 355/168, 11: 16319/6720 | 00 | 10: 23/56, 11: 641/6720 | 9301/120960 |
| idx=1299709 | 21: 1, 22: 43/42, 63: 127/126 | 22 | 21: 1/42, 63: 1/63 | 2663/181440 |
| idx=2599418 | 11: 15/14, 22: 15/14, 42: 43/42 | {11, 22} TIE | 42: 1/21; tied competitor: 0 | 23/420 |

**The aggregate-gap column is a one-sided screen and licenses nothing
else (SEP-A15(i), printed beside the column):** when the gap is zero,
Corollary E3.2 closes every H-optimal action with no gluing iteration;
when it is nonzero, U_a ≤ V^H + gap never establishes U_a ≤ V^H, and
gap-versus-headroom comparisons are **not evidence in either
direction**.

Three pre-declared readings, fixed before any number exists:

1. **idx=0 is the frontier-unnecessity test (scope per SEP-A15(ii)).**
   S6b's frontier computation STOPPED at leads 1-0 and 1-1 (frontier >
   16384, PG-A13). If separation closes here, the object S6b could not
   complete is proved **not needed** for the root decision. Two
   sentences are printed and neither stands for the other: *this run
   demonstrates the Pareto frontier at the competing leads is
   unnecessary for the root decision*; *this run does NOT test the
   parent's economy claim* [bracketed quotation: "the solver does
   **not** need an exact solution for every action"] *— it computes the
   exact H solve at every action because DS-A10's receipts require it.*
   The economy-claim experiment is SEP-A17's successor (non-exact seeds,
   transport under Lemma E7), named there so this run is never cited as
   it.
2. **idx=1299709 is the tightness test.** Headroom 1/63 against lead
   63: separation holds iff the per-action price U_63 − Q^H(63) is at
   most 1/63 − (U_63 − ... ) — precisely, iff U_63 ≤ 43/42. Either
   outcome is a result (F7, NO-RESCUE).
3. **idx=2599418 is the exact-tightness test (binding form per
   SEP-A9).** Two H-optimal actions tie at V^H = 15/14. Action 11
   separates iff U_22 ≤ 15/14 AND U_42 ≤ 15/14; action 22 separates iff
   U_11 ≤ 15/14 AND U_42 ≤ 15/14 (count convention). Since U ≥ Q^H
   always and Q^H = V^H at the tied competitor, the tied conjunct holds
   iff the competitor's per-action price is **exactly zero** — by
   Theorem E6.3 the sandwich collapses there, L = Q^H = U. The probe
   attempts each tied action symmetrically; a verdict "neither, though
   the pair {11, 22} exhausts Opt^H per treatment H" is reported as H's
   fact, never as the witnesses' (Theorem E6.4).

## The evaluators (freeze 37, SEP-A6; the DS-A7(iii) correction is
## SEP-A7)

**U_a:** `walt_strat::revealed::revealed_summary(kernel, focal,
dir).q_c[a]` read at the declared direction — identified as
E_β[V*_a] (Lemma E3). It fixes the root action, solves each world
revealed with the field untouched ((C4) by construction), and averages
over the FULL enumerated fiber. The root-maximising siblings are named
once and never confused with it: `v_f` = U^agg = V^F (same struct),
`fiber_probe.rs::aggregate` and `predictive_rank.rs::fused` (which
produced the S6a `fusion_gap` column). The per-action price
U_a − Q^H(a) is `price.rs::information_prices().g_cont_by_root[a]`,
already sign-asserted; the probe reads it, prints it, and **that column
is the measurement** (SEP-A5(ii)).

**L_a:** `walt-strat/src/info.rs::policy_value` — max-freedom is
structural: the focal callback returns `DominoSet::single(...)`, and a
`Policy` is a total function on the information partition, so it cannot
depend on the hidden world whatever its constructor knew (SEP-A13). The
**max-freedom receipt** (SEP-A13, mandatory form): at every focal
callback invocation the returned set is asserted a singleton, and the
run prints the counted receipt — focal decision states evaluated =
singleton expansions = states reached in `InfoPartition` — asserted
equal. `is_affine()` stays in code as a cheap invariant and is **never
reported as the receipt** (it is vacuous at `trick_diff`'s zero slope).

**Solver identification (freeze 37(g)):** the envelope path
`hidden::hidden_root_values` at `Direction::trick_diff()` and the scalar
authority `ScalarHidden::action_values_dag` at
`ScalarValuation::trick_only` under `AUTHORITY_BUDGET` (freeze 26) are
two independently built solvers in the **same units** — the
focal-minus-opponent trick differential — and are asserted **equal
exactly, per action, with NO bridge** (SEP-A5: the previously proposed
bridge assert would assert a false identity). The probe asserts the
root is trick-leading so both solvers price the same action list. The
freeze-26 bridge Q_diff = 2·Q_count − grade applies **only** where the
count and differential conventions meet: the S6a cross-check, the
extraction solve, and the reporting boundary. Reporting convention:
**count**, matching the S6a receipts; the bridge is affine with slope
2 > 0, so every verdict is convention-invariant.

**Budget honesty (freeze 37(h)):** the scalar authority is budgeted;
exhaustion is a declared stop printed R-A18-style with "correctness
gate unmet" beside every row it voids. `hidden.rs`, `revealed.rs`,
`price.rs` and `policy_value` carry no budget and no stop, and the
results file states that in place.

**T7 / (C2) (no decimation inside witnesses):** both L and U are
computed over the full |X| = 1680 fiber; decimation chose WHICH
coordinates (g = 1299709), which C2/T7 permit; no decimated world set
from any prior probe (`fiber_probe`'s W = 240 sets included) appears
inside any L or U.

## The candidate library (freeze 36, SEP-A4)

- **Key:** (grade, base index, declaration, root action) under the S6a
  unranking (freezes 22–25); pip derived, printed for humans, asserted
  against the unranking at load, never a key component.
- **Body:** total map observation-record → chosen tile over
  `InfoPartition::build(kernel, root)`, serialised as (record, tile)
  pairs sorted lexicographically by record under canonical ascending
  domino-index order; the record is the plays since the kernel decision
  point, root action first. `InfoStateId` **never appears** in a stored
  entry (it is an in-process handle; storing it would create a second
  authority over the partition's ordering).
- **Frame, on every entry:** observation contract (R-A11, freeze 26),
  field (§7.4 uniform-legal), belief (uniform over the freeze-7/23
  enumeration), |X|, freeze-set digest. Digest mismatch = **corrupt,
  not stale**: discarded entire (DS-A30 discipline).
- **No values, no verdicts, no ranks:** the file is a cache, never an
  authority (X-A17); a loaded entry is re-priced by `policy_value`
  before anything is reported.
- **Transport: identity only in v1** (SEP-A3(vi) cites Lemma E7: policy
  transport establishes lawfulness — all a primal witness needs;
  verdict transport additionally requires the exhibited value-order
  isomorphism with the belief transported). Dominance never travels.
- **Seed rule (SEP-A11(ii)):** the seed is the **unmemoized**
  argmax-recording pooled H solve over the same partition
  (`policy_inspect.rs::Ctx::solve` is that object), tie rule **cited to
  freeze 26** (least domino index among the argmax), never re-declared.
  `action_values_dag` can never supply the seed: its trick-boundary
  memo returns whole subtrees unexpanded, so a harvested policy would
  be partial. The seed contributes no number to any reported L.
  Convention note (SEP-A8): the extraction solve runs in the count
  convention, pricing in the differential; at any node the tricks
  remaining are action-independent, so V_diff = 2·V_count − (tricks
  remaining) has an action-independent offset and the argmax sets —
  hence the least-tile selection — are identical under the two
  conventions.
- **DS-A16 header note:** entries remain valid primal-witness sources
  under count re-entry, evaluated under the richer valuation; their
  count-free quality verdicts do not survive.

Six of the seven S6b singleton roots collapse by indifference, so as a
LIBRARY this run's harvest is one informative entry (idx=0 lead 00, the
108-decision playbook) plus vacuous-but-lawful entries; the run is
scoped as a certification probe, not a library harvest, and says so in
its header.

## The run

Sequential, single process, deterministic; grade-3 only (**the n=4 rung
is REJECTED for v1**, SEP-A10: unstated cost model, no declarable stop
in `revealed_summary`, authority completion unmeasured; if wanted it is
its own design with deterministic-unit budgets on every evaluator).
Output `walt-factory/results/separation_2026-08-13.txt`, regenerated by
`cargo run --release -p walt-factory --example separation_probe`. Per
coordinate:

1. Coordinate identity asserted first (SEP-A14(iii)): grade, base
   index, declaration, |X| = 1680, enumeration order.
2. Scalar H authority per action (freeze-26 budget; R-A18 gate line);
   root asserted trick-leading.
3. Envelope H per action; **(R1)** exact per-action equality with the
   scalar authority, no bridge.
4. **(R4)** S6a cross-check: recomputed Q^H asserted exactly equal to
   the S6a filed values, carried as a frozen table in the probe source
   with provenance line "quoted from `predictive_rank_2026-08-12.txt`,
   S6a, exploratory tier" (never re-parsed from results text), compared
   in the count convention across the freeze-26 bridge.
5. Candidate extraction at every H-optimal action (all tied argmaxes)
   by the freeze-36 seed rule; entries written to the library file with
   frame and digest.
6. **(R2)** L per H-optimal action via `policy_value`: asserted
   **exactly equal** to Q^H (Corollary E4.1(2) makes equality a theorem
   about the pipeline; strict inequality is stop-and-report naming the
   three possible defects — non-argmax extraction, partition
   disagreement, authority disagreement — per SEP-A11(i)). **(R5)** the
   counted singleton receipt printed.
7. **(R3) the measurement:** the per-action price U_a − Q^H(a) printed
   as an exact rational for every action, from `g_cont_by_root`, with
   its sign assertion.
8. The separation table: for each H-optimal a⋆ and each competitor a,
   L_{a⋆}, U_a, margin L_{a⋆} − U_a, verdict SEPARATED / NOT SEPARATED
   per pair; coordinate verdict SEPARATED iff some a⋆ separates against
   every competitor; NOT-SEPARATED pairs printed in the exact-negative
   form (SEP-A16). Certified action asserted ∈ argmax_H; member-not-set
   caveat verbatim beside every SEPARATED verdict.

Provenance/typing line printed once (SEP-A12): *the separation's
validity does not cite H, but this run's witnesses were produced with
H's help — L's seed is an H solve and R1–R4 are H cross-checks; the
logic of Theorem E6.4 is H-free, the provenance of these witnesses is
not.*

## The reading, pre-declared (amended per SEP-A15)

A SEPARATED coordinate certifies a⋆ ∈ Opt^H(B) at that coordinate under
the declared belief, field, valuation, and observation contract —
nothing else: no reachability assertion (R-A2 fence restated wherever a
witness is reported; fiber members are FEASIBLE, never reachable), no
uniqueness, no transport, no opening claim. **No cost, timing, runtime
or tractability claim of any kind** (SEP-A15(iii)): the run performs
the full exact solve it would have to avoid, so any timing comparison
is void by construction; wall-clock is printed as provenance only. A
NOT-SEPARATED coordinate prints its failing pairs in the
exact-negative form. Both outcomes are results (F7, NO-RESCUE). Nothing
is promoted; exploratory tier; a number becomes quotable only by brief
amendment adding it to a verifier receipt.

## Adjudication record (SEP-Q1..SEP-Q8 → rulings)

- SEP-Q1 (freeze 36): SEP-A3 accept-with-amendment ×6; frozen at
  SEP-A4.
- SEP-Q2 (freeze 37 + DS-A7(iii) refinement): SEP-A5 (identification
  ACCEPT; bridge assert REJECT), SEP-A6 (frozen), SEP-A7 (DS-A7(iii)
  corrected with pointer marker).
- SEP-Q3 (tie-break): SEP-A8 — rule accepted, re-freezing rejected;
  freeze 26 cited.
- SEP-Q4 (tied optimum): SEP-A9 — lawful, completed (both competitors)
  and simplified (primal conjunct automatic).
- SEP-Q5 (n=4 rung): SEP-A10 — REJECTED for v1.
- SEP-Q6 (L < Q^H typing): SEP-A11 — stop-and-report CONFIRMED,
  strengthened to a pipeline theorem violation.
- SEP-Q7 (receipts): SEP-A12/SEP-A13 — amended to the five contentful
  receipts R1–R5.
- SEP-Q8 (S6a values): SEP-A14 — ASSERTED exactly, frozen source table,
  coordinate identity first.
