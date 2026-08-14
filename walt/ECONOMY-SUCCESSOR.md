# The economy successor: does the sandwich close on a primal witness
# that is not an exact solve?
# (design)

Status: **DESIGN, awaiting walt-math rulings.** This document is the
successor **SEP-A17** names, in SEP-A17's own scope words: *"seed L from
a source that is **not** an exact solve at a⋆ — a transported library
entry, a hand-authored playbook, a cheap heuristic — and ask whether the
sandwich still closes"*, at coordinates where treatment H still
completes so every claim remains checkable. Its questions are numbered
**EC-Q1..EC-Q13** (the prefix is unused in `walt/CENSUS-RULINGS.md`;
checked by grep at authoring time). Nothing here is built until
walt-math rules.

Standing rulings inherit by name and are not restated: F1–F7, r3 Q1–Q5,
Y1–Y3, P-A1..P-A21, X-A1..X-A17, E-A1..E-A20, S-A1..S-A18,
R-A1..R-A24, PG-A1..PG-A18, J-A1..J-A18, DS-A1..DS-A36 and
SEP-A1..SEP-A19, together with Lemmas V, X, E, S, S-fold, S-det, R, G,
J, Propositions G-flat, J-0, J-1, J-win, Corollaries S-rigid, R-fold.
Mathematics is cited to the errata under DS-A17: **Lemma E3** and
conditions **(C1)–(C4)** of §3.4, **Lemma E4** and **Non-theorem E4′**
with DS-A27's semantic obligation, **Corollary E4.1**, **Corollary
E3.2**, **Lemma E7** (§8.3, when dominance travels), **Theorems
E6.3–E6.5**. Freezes 1–43 are in force; this design reserves content for
**46** and fixes nothing itself. **Tier: exploratory throughout.**
Vocabulary fence (D3, DS-A1, SEP-A1): witness, separation, receipt — the
word "certificate" does not appear.

This design is independent of `walt/SEPARATION-RUNG-N4.md`; neither is a
prerequisite for the other, and they share no freeze.

## 1. The object, and the one thing that changes

Experiment E's sandwich, unchanged: L_{a⋆} ≥ U_a for every a ≠ a⋆
implies a⋆ ∈ Opt^H(B) (Theorem E6.4, membership never uniqueness).
U_a is unchanged in every respect — freeze 37 entire, the
action-conditioned treatment-C value E_β[V*_a] read from
`revealed_summary().q_c[a]` (Lemma E3), over the full enumerated fiber,
with no decimation inside it ((C2)).

**What changes is the seed of L, and therefore the whole typing of the
primal side.** In the adjudicated grade-3 run the candidate at each
H-optimal action was the H-argmax policy, so by **Corollary E4.1(2)**
L = Q^H exactly and necessarily: the primal witness sat at its ceiling,
L was a receipt and not a measurement, and every verdict was decided
entirely by the upper witness (SEP-A2). Here the candidate is
deliberately **not** an exact solve at a⋆. Corollary E4.1(2) does not
apply. **L < Q^H is expected**, the shortfall is this run's
measurement, and both sides of the sandwich decide the verdict.

### 1.1 The retyping of the primal side, stated as it must be printed

**SEP-A2's header sentence is REPLACED.** It is not amended, and it is
not carried alongside — a results file that printed both would assert
that L is simultaneously at its ceiling and below it. The replacement,
printed verbatim before any number:

> *"the primal witness at each seeded action is a NON-EXACT lawful
> policy re-priced by the fixed-policy evaluator. Corollary E4.1(2) does
> not apply and L < Q^H is expected: the quantity Q^H(a⋆) − L is this
> run's measurement, the economy gap. Unlike the adjudicated grade-3
> run, the separation verdict here is decided by BOTH sides of the
> sandwich, and a failure to separate is a failure of THIS candidate,
> not the exact negative of Corollary E4.1(3)."*

The last clause is the one most easily lost, and §5.2 states it again in
the reading. SEP-A16's exact-negative form belongs to the U side only.

### 1.2 The two quantities the reading turns on, and both are already
### exact

Define, per coordinate and per H-optimal action a⋆:

- the **economy gap** g(a⋆, seed) := Q^H(a⋆) − L^seed(a⋆) ≥ 0 — how far
  the cheap witness falls short of the exact one;
- the **certification slack** s(a⋆) := Q^H(a⋆) − max_{a ≠ a⋆} U_a — how
  far a witness at a⋆ may fall short and still separate.

**Exact identity, asserted in-run (R8 of §4):**

> a⋆ separates under a given seed **iff** g(a⋆, seed) ≤ s(a⋆).

*Proof.* SEPARATED means L_{a⋆} ≥ U_a for every a ≠ a⋆, i.e.
L_{a⋆} ≥ max_{a≠a⋆} U_a, i.e. Q^H(a⋆) − L_{a⋆} ≤ Q^H(a⋆) − max_{a≠a⋆} U_a. ∎

s(a⋆) is **already known exactly** at the three grade-3 coordinates: it
is the minimum margin column of `separation_2026-08-13.txt`, where
L = Q^H made every margin equal to Q^H(a⋆) − U_a. Quoted, not
re-asserted (exploratory tier), count convention:

| coordinate | a⋆ | Q^H(a⋆) | competitor margins | **slack s(a⋆)** |
|---|---|---|---|---|
| idx = 0 | 00 | 53/21 | 449/1120 (vs 10), 59/2240 (vs 11) | **59/2240** |
| idx = 1299709 | 22 | 43/42 | 1/42 (vs 21), 1/63 (vs 63) | **1/63** |
| idx = 2599418 | 11 | 15/14 | 0 (vs 22), 1/21 (vs 42) | **0** |
| idx = 2599418 | 22 | 15/14 | 0 (vs 11), 1/21 (vs 42) | **0** |

That table fixes the entire pre-declared reading before any seed exists,
and it makes the three coordinates a **graded** test rather than three
samples of one thing:

1. **idx = 0 is the loose rung.** A seed may fall 59/2240 short of the
   exact value at lead 00 and still separate.
2. **idx = 1299709 is the tight rung**, at 1/63.
3. **idx = 2599418 is the ZERO-SLACK CONTROL, and its outcome is
   pre-declared as a theorem, not a measurement.** The two H-optimal
   actions tie, so s = 0 at both, and by §1.2's identity a seed
   separates there **iff its economy gap is exactly zero** — iff the
   seed is exactly optimal. A cheap seed that separates at idx = 2599418
   has not demonstrated economy; it has demonstrated that it happened to
   be optimal. This is printed **before** the arm runs so that a
   coincidence is not read as a result.

Everything above is arithmetic on quoted receipts and licenses nothing
by itself; it is the pre-declaration, not a finding.

## 2. The seed sources, precisely typed

Four arms. Each produces a candidate that is a **total lawful map on the
information partition** — a `Policy` in `info.rs`'s sense, one action per
`InfoStateId`, so that world-peeking is unconstructible by type
(SEP-A13's stronger reason: a policy assigns one action per information
state and therefore cannot depend on the hidden world whatever its
constructor knew). Every candidate, whatever its origin, is re-priced by
`policy_value` before anything is reported (freeze 36(d): the library is
a cache and never an authority, X-A17).

### 2.1 Arm T — the transported entry, and exactly what Lemma E7
### obligates

**The declared transport is the declaration fold** φ_{p→p′} of **Lemma
S-fold**: π(p) = p′ together with the unique order isomorphism
ℙ∖{p} → ℙ∖{p′}, and φ(a:b) = π(a):π(b). Lemma S-fold proves φ is a seat
transport from the δ = p structure to the δ = p′ structure, that it is
the unique one, and that the fold is composable.

**The value-order isomorphism is exhibited, and it is Corollary R-fold.**
Restricted to a rung coordinate's live set, φ carries legal sets to
legal sets, uniform field masses to uniform field masses, observations
to observations, count-free increments to increments and focal to focal;
the seat rotation is the identity. Hence the induced bijection of fibers
X → X′ satisfies α_{Tρ}(Tξ) = α_ρ(ξ) for every lawful ρ and every ξ —
which is precisely the displayed identity **Lemma E7** requires. Lemma
E7's own binding clause names this: *"Corollary R-fold is an instance of
exactly this and may be cited as one; nothing else in the branch
currently is."*

**What Lemma E7 therefore obligates, and what is and is not claimed.**

- **Lawfulness transports with the policy, and that alone is what a
  primal witness needs** (Lemma E7's binding form, SEP-A3(vi)). This
  half needs no isomorphism and is claimed for the transported entry
  unconditionally.
- **Verdict transport additionally requires the exhibited value-order
  isomorphism, together with transport of the belief for
  belief-relative verdicts.** Both are available here: the isomorphism
  is Corollary R-fold, and the belief transports because φ is a
  bijection of the fiber carrying the uniform belief to the uniform
  belief (β′ = T_*β, Lemma E7(2)). So for **this** transport, and only
  this one, dominance, optimality at the belief, exposure and decision
  width all transport.
- **What is NOT claimed, ever:** nothing transports between the three
  base indices 0, 1299709 and 2599418 — they are unrelated hands and no
  isomorphism between them is exhibited or suspected. Dominance never
  travels with a policy alone (DS-A15). A run that reported an action as
  separated at one coordinate because a policy had separated at another
  would be asserting a transported optimality claim no theorem in the
  branch supports.

**The honest headline, printed at the head of arm T, and it is the thing
the brief asked to be honest about:** *because the transport is a value
order isomorphism, the transported entry is an exact relabelling of an
exact solve. Its economy gap is **zero by theorem**, not by measurement.
Arm T is a STRUCTURE-PROVING arm — it opens freeze 36's transport clause,
exhibits the branch's only non-identity value-order isomorphism in
running code, and produces receipts — and it is **not** an economy arm.
The economy content of this design is arms P and R.*

**The canonical key correspondence.** The S6a index encodes the
coordinate: `coordinate(grade, index)` computes
pip = index / (live_c · hand_c), then unranks the live set from
(index mod (live_c·hand_c)) / hand_c and the hand positions from the
remainder (freezes 22–25). The image key is computed, not searched:
unrank the source coordinate, apply φ tilewise to the live set and to
the hand, re-rank both with a ranking function inverting `unrank_comb`,
and assemble
index′ = p′·(live_c·hand_c) + rank(φL)·hand_c + rank_within(φH).
Mandatory receipts on the correspondence (part of R9, §4):
`unrank_comb(rank_comb(S)) == S` asserted for both combinations
(round-trip), `coordinate(3, index′)` asserted to reproduce
(pip = p′, φL, φH) exactly, and |X′| = 1680 asserted against
`kernel.count()`.

**One caveat that must be asserted rather than assumed.** φ is not
order-preserving on domino indices, so the freeze-7/23 fiber enumeration
orders at source and image correspond by a **permutation**, not
identically. Corollary R-fold's own scope clause says the same thing
about its matrices: *"the in-run assertion is equality of dimensions and
of values, never byte-equality."* Accordingly the design asserts
**value** identities (Q^H per corresponding action, L, U) and never
per-world byte-equality or index-by-index correspondence.

**Which transports are actually available.** The library
(`walt-factory/store/candidate_library.txt`, digest
`SEP-lib-v1|freezes-22-26-36-37|...|grade=3`) holds exactly four
entries, all at declaration `PipTrump(0)`: (idx = 0, root 00), (idx =
1299709, root 22), (idx = 2599418, root 11), (idx = 2599418, root 22).
Only the first is informative; the other three are the
lawful-but-vacuous tied/indifference-collapsed entries SEP-A17 describes.
Each entry has **six** declaration images, p′ ∈ {1,…,6}. That is
twenty-four candidate transports, all of them exact relabellings.

**The fold-reading split, and the diagnostic it makes free.** Lemma
S-fold records that the seven-fold orbit depends on S-A2's reading: under
the **operative** reading all seven declarations fold; under the
**literal** reading of §1.3 (tier-0 tiles ordered among themselves by pip
sum) the orbits collapse to {0, 6} and five singletons. Which reading
`walt-core`'s `Decl::PipTrump` realises is not settled in this branch.
Therefore arm T is split:

- **T-receipt: p′ = 6 only**, for all four entries. The pair 0 ↔ 6 folds
  under **both** readings, so the R9 equalities are receipts and a
  mismatch is stop-and-report.
- **T-diagnostic: p′ ∈ {1,…,5}** for the idx = 0 entry only. Under the
  operative reading these hold; under the literal reading they fail.
  This arm is a **declared measurement of which reading the
  implementation realises**, both of whose outcomes are results, and it
  is explicitly **not** stop-and-report. It discharges S-A2's standing
  obligation that a run print which reading it froze and what the other
  would have given. (EC-Q4 asks whether walt-math accepts this typing,
  which is the one place in this design where a failing assertion is not
  a bug.)

### 2.2 Arm P — the hand-authored playbook

A declared rule family, each member a total function of the
**observation record** and the legal set, and of nothing else. This is
where the information-consistency argument is load-bearing rather than
structural, and each rule carries its argument:

- **P1 least-tile.** Play the least legal tile by canonical ascending
  domino index. Total and lawful trivially. (Note it coincides with
  freeze 26's tie rule read as a global policy; it is cited to freeze 26
  and not re-declared.)
- **P2 greatest-tile.** Play the greatest legal tile by the same order.
- **P3 beat-if-able.** If the viewer is not the trick leader and some
  legal tile beats the best tile so far played in the current trick
  under the declaration, play the least such tile; otherwise play the
  least legal tile. *Information-consistency argument, mandatory:* the
  tiles played so far in the current trick and their seats are part of
  the observation record (R-A11's full public record), and the
  declaration is fixed, so "the best tile so far" and "beats it" are
  functions of the record alone. No hidden hand is consulted. This is
  exactly the place a world-peek could hide, which is why the argument
  is written out rather than asserted.
- **P4 trump-hoard.** Play the least legal non-trump if one exists,
  otherwise the least legal trump. Trump membership is a function of the
  declaration and the tile.

Each is total by construction — defined at every information state from
the record and the legal set — so `Policy::build` succeeds and the
totality property holds **by construction**, which per PG-A8 makes it
**not** a receipt for this arm. The contentful receipts for arm P are
R2′, R5 and R7 (§4).

Arm P is the primary economy content of this design.

### 2.3 Arm R — the E7-free re-key, named and fenced

Moving the idx = 0 playbook to a **different hand** by a record
correspondence that is not a transport (matching records positionally,
repairing illegal or missing choices by a declared fallback to P1) is
lawful **as a heuristic seed source** and is not a transport. Lemma E7's
hypotheses fail: there is no bijection of lawful policy classes and no
value-order isomorphism, so nothing about the source coordinate travels
— not its value, not its dominance, not its verdict. Calling it a
transport would be precisely the DS-A15 error this design exists to
avoid.

If run, arm R is labelled **HEURISTIC RE-KEY (NOT A TRANSPORT)** on
every row, and its only claim is the same as arm P's: whatever lawful
total map comes out, priced exactly, yields an exact L. It is optional
(EC-Q5).

### 2.4 Arm B — the bid-derived seed, named and DECLINED

Jason's contagion frame reads a bid as a lower-bound claim, and a
lower-bound claim is exactly the shape of a primal witness. It does not
convert on this carrier, for a reason that is structural rather than
practical:

- a 42 bid is a claim about **points and marks** — the scored,
  count-bearing game;
- this carrier is **count-free expected focal tricks**;
- E-A2 governs the gap between them, and the count and score lift is
  **Experiment G**, which DS-A12 rules *not lawfully designable from the
  parent document alone* — it needs its own design and its own rulings
  covering the declared cone, the feature law, which count-free verdicts
  are re-derived versus inherited, and what happens to every form-keyed
  record.

There is no clean count-free surrogate: "I can take at least k tricks"
is not what a bid of 30 asserts. **Arm B is therefore declined here, by
name, with the reason**, rather than approximated. Recording the
decline is the point — so that "did walt ever try the contagion seed?"
is answered by this paragraph and not by an invented conversion.
EC-Q6 asks whether walt-math sees a lawful count-free surrogate the
design has missed.

## 3. The coordinates

**The three grade-3 coordinates of the adjudicated run** — idx = 0,
1299709, 2599418, all at `PipTrump(0)`, |X| = 1680, grade 3 — plus
**the image coordinates of arm T** (four at p′ = 6; five more for the
T-diagnostic).

Justification, since the brief asks for one:

- SEP-A17's own scope requires *"coordinates where treatment H still
  completes so every claim remains checkable"*. H completes at all three
  in well under a second (`separation_2026-08-13.txt`: 3,942 ms total
  wall-clock for all three coordinates, provenance only), so every arm
  can be run against a fully cross-checked exact authority.
- Both quantities of §1.2 are already exact there, so the reading is
  pre-declarable to the last rational **before any seed is written** —
  which is the strongest form of pre-declaration available.
- The tie at idx = 2599418 supplies a **zero-slack control** for free
  (§1.2, item 3). A design without a control at s = 0 could not
  distinguish "the cheap seed was good enough" from "the cheap seed was
  optimal".
- The transport images are grade-3 too, so their H solves are equally
  cheap and equally cross-checkable.

**The natural second rung, named and not taken:** the nine
receipt-corpus n = 4 coordinates of `walt/SEPARATION-RUNG-N4.md`, once
their H, U and slack values exist. The economy question is far more
interesting where the exact solve is expensive. It is not taken here
because SEP-A17 scopes this successor to checkable coordinates and
because that rung is unadjudicated. The two designs are deliberately
independent; if both are built, the economy arms port to the n = 4
coordinates by re-declaration and nothing in §1–§5 changes but the
coordinate list.

**One structural consequence of the choice, stated up front so its
absence is not read as evidence (§5.2).** All three grade-3 coordinates
already SEPARATED with the exact seed. Therefore the outcome
"not certifiable by any seed" — Corollary E4.1(3)'s exact negative —
**cannot arise at this carrier**. It is pre-declared for completeness
and its unavailability is printed in the header.

## 4. The receipts

R1, R3, R4 and R5 are inherited from SEP-A12 unchanged. **R2 is
replaced.** Four receipts are new.

- **(R1) solver identification** — envelope H equals the scalar
  authority exactly, per action, no bridge, root asserted trick-leading
  (SEP-A6(g)). Unchanged.
- **(R2′) the primal sanity assert, now a genuine receipt.** L ≤ Q^H
  asserted exactly at every seeded action wherever H completes (Lemma
  E4). In the adjudicated run this held as *equality by construction*
  and PG-A8 therefore denied it receipt status. Here it is a real
  receipt: with a non-exact seed the equality is gone, and a violation
  would prove **Non-theorem E4′'s inversion** — a world-informed
  evaluator leaking into the primal path, which is the branch's named
  soundness failure. A violation is stop-and-report under NO-RESCUE,
  never patched and never reconciled, and the stop message names the
  three defects it can indicate: a world-informed evaluator on the L
  path, a partition disagreement, or a seed that is not a total lawful
  map.
- **(R3) the per-action price** — U_a − Q^H(a) as an exact rational for
  every action from `g_cont_by_root`, with its sign assertion.
  Unchanged, and now doubly load-bearing: it determines s(a⋆).
- **(R4) the S6a cross-check** — SEP-A14 entire, at the three base
  coordinates. The transport image coordinates have no filed S6a value;
  **R9 substitutes there and the file says so** rather than leaving a
  blank column.
- **(R5) the max-freedom counted receipt** — SEP-A13 as disambiguated by
  SEP-A19: focal callback invocations = singleton expansions = distinct
  partition states reached, with SEP-A19(b)'s typing sentence beside the
  `reached X of partition Y` line and the ratio not printed. Unchanged,
  and more important here than in the adjudicated run: a hand-authored
  policy is exactly the kind of object whose evaluator could be given a
  maximisation by accident.
- **(R6) seed lawfulness and totality.** The seed map is asserted total
  on `InfoPartition::build`'s state set (`choices.len() ==
  partition.len()`, the SEP-A19 totality half) and every chosen tile is
  asserted legal at its state (`Policy::build` already asserts this).
  For arms P and R this is the receipt that the rule family really is a
  total lawful map and not a rule with a hole.
- **(R7) the economy gap — THE MEASUREMENT.** g(a⋆, seed) =
  Q^H(a⋆) − L^seed(a⋆) printed as an exact rational for every
  (coordinate, a⋆, seed source). This column is what this run exists to
  produce.
- **(R8) the slack identity.** s(a⋆) = Q^H(a⋆) − max_{a≠a⋆} U_a printed
  as an exact rational per (coordinate, a⋆), and the equivalence of
  §1.2 — SEPARATED iff g ≤ s — **asserted against the independently
  computed pairwise verdicts**. This is not bookkeeping: it ties the
  summary column to the pairwise comparisons through two different
  computations, so a sign error or a convention slip in either shows up
  as a stop rather than as a plausible table.
- **(R9) the transport receipts** (arm T). The key round-trip of §2.1;
  `coordinate(3, index′)` reproducing (p′, φL, φH); |X′| = 1680; and the
  **exhibited-isomorphism equalities** — Q^H per corresponding action
  and L of the transported policy asserted **exactly equal** at source
  and image, in the count convention. Value equalities only; never
  per-world byte-equality (§2.1's caveat). At T-receipt (p′ = 6) a
  mismatch is stop-and-report; at T-diagnostic (p′ ∈ {1..5}) it is the
  measurement (§2.1).

The SEP-A12 provenance-typing sentence is printed unchanged and is more
pointed here: *the separation's validity does not cite H, but this run's
witnesses were produced with H's help* — the slack s(a⋆) is computed
from H and U, R1/R3/R4 are H cross-checks, and R2′ is asserted against
H. The logic of Theorem E6.4 is H-free; the provenance of these
particular numbers is not.

## 5. The run and the pre-declared reading

### 5.1 The run

Sequential, single process, deterministic. Output
`walt-factory/results/economy_seed_<date>.txt`, regenerated by
`cargo run --release -p walt-factory --example economy_seed`. Freeze
**46**'s reserved content (EC-Q1): the arm list and each rule's exact
definition, the declared transport φ and the image-key construction, the
canonical run order (coordinate ascending by index, then a⋆ ascending by
domino index, then arm in the order T-receipt, P1, P2, P3, P4,
T-diagnostic, R), and the results-file column set.

Header, before any number: §1.1's replacement sentence verbatim;
§2.1's arm-T honesty headline; §3's unavailability notice for the exact
negative; the SEP-A12 provenance-typing sentence; SEP-A15(i)'s
one-sided-screen paragraph wherever the aggregate gap column appears;
the R-A2 reachability fence (fiber members are FEASIBLE, never
reachable); the treatment-C naming clause (ξ = ω, so C and C⁺ coincide);
SEP-A15(iii)'s no-cost-claim sentence; and DS-A16's header note (entries
remain valid primal-witness sources under count re-entry; their
count-free quality verdicts do not survive).

Per (coordinate, a⋆): the exact table of §1.2 recomputed rather than
quoted, with R3, R4 and R8; then one row per arm carrying L^seed, R7's
economy gap, the pair-by-pair comparison against every competitor's U,
the verdict, and R2′/R5/R6 outcomes; then R9 for the transport rows.

### 5.2 The reading, pre-declared before any seed is written

Four outcomes per (coordinate, a⋆), fixed now:

1. **CERTIFIED-CHEAP** — some non-exact seed has g ≤ s and separates.
   The statement, and its fence, both printed: *at this coordinate an
   exact solve at a⋆ was not necessary to obtain the primal witness; the
   parent's economy claim* ["the solver does **not** need an exact
   solution for every action"] *is exercised on the PRIMAL side at a⋆.*
   **The fence, mandatory and adjacent:** *this run still computes U
   exactly at every competitor and still computes H at every action for
   its receipts, so the RUN is not cheap. What is tested is whether the
   WITNESS must be exact, which is SEP-A17's scope and is narrower than
   "the solver is cheap".* No timing, cost, runtime or tractability
   claim of any kind follows (SEP-A15(iii), P-A19); no arm is compared
   against any other arm by cost.
2. **CERTIFIED-EXACT-ONLY** — no arm separates, and the exact seed
   (recomputed in this run, so the comparison is within one pass) does.
   The statement: *at this coordinate the declared cheap family does not
   supply a witness within the slack.* **The fence, mandatory:** this is
   **not** an exact negative and is not Corollary E4.1(3). It says these
   four rules failed; it says nothing about candidate sets in general,
   and a better candidate could close it. This asymmetry with the U side
   is exact and is stated wherever it appears: **a primal failure is a
   failure of the candidate; only a U-side failure is a proof about all
   candidates.**
3. **NOT-CERTIFIABLE-BY-ANY-SEED** — Corollary E4.1(3)'s exact negative,
   Q^H(a⋆) < U_a. **Structurally unavailable at this carrier** (§3), and
   the header says so before any row is read, so that its absence is
   never read as evidence about seeds.
4. **ZERO-SLACK COINCIDENCE** — a seed separates at idx = 2599418, where
   s = 0. Printed as: *the seed is exactly optimal at this action; this
   is not economy* (§1.2, item 3).

**The per-source economy gap column** is the run's measurement and is
reported per (coordinate, a⋆, arm) as an exact rational. It is a
distance between two exact values at one declared coordinate under one
declared belief. It is **not** a quality score for the rule, not
transferable to any other coordinate, not a policy ranking, and not a
term in the DS-A2 ladder. A rule with a small gap at idx = 0 has
demonstrated nothing whatever about idx = 1299709, and the file says so
where the column appears.

**Both outcomes are results** (F7, NO-RESCUE) — and here "both" means
all four above. A run in which every cheap arm fails at every coordinate
is a clean, filed answer to the economy question on this carrier: it
would say the primal side is where the difficulty lives, which is
exactly Corollary E3.2's "consequence (the useful one)" read in the
other direction. Nothing is promoted; exploratory tier; a number becomes
quotable only by brief amendment adding it to a verifier receipt.

## 6. Questions for adjudication (EC-Q1..EC-Q13)

**EC-Q1 (freeze 46).** Is the reserved content of §5.1 right and
complete — the arm list and each rule's exact definition, the declared
transport and image-key construction, the canonical run order, and the
column set? Should the rule family of §2.2 be frozen as written, or
declared open so that a later arm may be added without a re-freeze?

**EC-Q2 (the retyping, and the replaced header).** Is §1.1's replacement
of SEP-A2's header sentence correct as written, and is the receipt set
of §4 right — specifically, is **(R2′)** correctly promoted from a
by-construction equality to a genuine receipt against Non-theorem E4′'s
inversion, now that Corollary E4.1(2) no longer forces equality?

**EC-Q3 (the slack identity R8).** Is §1.2's identity — SEPARATED iff
g(a⋆, seed) ≤ s(a⋆) — correct as stated and proved, and is asserting it
against the independently computed pairwise verdicts the right receipt
form? Is s(a⋆) correctly read off the adjudicated run's margin column,
given that L = Q^H there made every margin equal Q^H(a⋆) − U_a?

**EC-Q4 (the fold-reading diagnostic, the one non-stop-and-report
assertion in this design).** §2.1 splits arm T into T-receipt (p′ = 6,
valid under both S-A2 readings, mismatch = stop-and-report) and
T-diagnostic (p′ ∈ {1..5}, whose outcome measures which reading
`walt-core` realises and is therefore **not** a bug either way). Is that
typing lawful? It is the only place in this design where a failing
equality is a result rather than a defect, and it is uncomfortable
enough to want an explicit ruling.

**EC-Q5 (arm R).** Should the E7-free re-key (§2.3) be run at all? It is
fenced as a heuristic and not a transport, and its label is mandatory on
every row, but it invites exactly the confusion DS-A15 forbids. Drop it,
run it with the fence, or run it only if arms P1–P4 all fail?

**EC-Q6 (arm B, the bid-derived seed).** §2.4 declines it: a bid is a
points-and-marks claim, this carrier is count-free expected focal
tricks, and the lift is Experiment G, which DS-A12 rules not lawfully
designable yet. Is there a lawful count-free surrogate the design has
missed, or is the decline correct?

**EC-Q7 (what arm T is worth).** §2.1 states that arm T's economy gap is
zero **by theorem**, so it proves the transport machinery rather than
the economy claim. Is that the right disposition — keep it as a
structure-proving arm that opens freeze 36's transport clause and
discharges S-A2's reading obligation — or is a design whose only genuine
transport is value-preserving better served by deferring transport
entirely until a non-isomorphic lawful correspondence exists?

**EC-Q8 (opening freeze 36's transport clause).** Freeze 36(e) fixes
"identity only in v1" and says cross-coordinate transport re-enters with
its own adjudication. This design requests exactly that re-entry, for
the declaration fold only, on the ground that Corollary R-fold exhibits
Lemma E7's identity. Is the request granted as scoped, and should the
amended clause name the fold explicitly rather than admitting
"transports with an exhibited isomorphism" as a class?

**EC-Q9 (the coordinates).** Is §3's carrier right — the three grade-3
coordinates plus the transport images — given that it makes outcome 3 of
§5.2 structurally unavailable? The alternative is to add one coordinate
where the exact seed did **not** separate, so that the exact negative is
reachable; no such coordinate exists in the branch today, which is why
the design does not propose one.

**EC-Q10 (the zero-slack control).** Is idx = 2599418 correctly typed as
a control whose outcome is a theorem rather than a measurement, and is
"ZERO-SLACK COINCIDENCE" the right verdict name for a seed that
separates there?

**EC-Q11 (the primal/upper asymmetry in the reading).** §5.2 outcome 2
insists that a primal failure is a failure of the candidate while only a
U-side failure is a proof about all candidates. Is the wording strong
enough, and should CERTIFIED-EXACT-ONLY additionally record what would
be needed to make it a statement about candidate sets — a lower bound on
the achievable L over some declared class — or is that a separate
experiment?

**EC-Q12 (a provenance discrepancy in SEP-A17's own text).** SEP-A17 and
`SEPARATION-PROBE.md` describe the informative library entry as *"the
108-decision playbook"* at idx = 0, lead 00. Both machine receipts
report **384**: S6b's `policy_geometry_2026-08-12.txt` gives k = 384 for
that coordinate and lead, and `separation_2026-08-13.txt` reports the
extraction as "50712 states, 384 with genuine choice". The number 108
appears in no results file. Since arm T's source is exactly this entry,
the design would rather have the discrepancy ruled than quietly use one
number: is 108 a stale figure to be corrected with a pointer marker
(DS-A28(i)), or does it count something the design has not identified?

**EC-Q13 (the economy claim's scope, said once and correctly).** §5.2
outcome 1 claims the parent's economy claim is exercised **on the primal
side at a⋆** and fences that the run still computes U exactly at every
competitor and H at every action. Is that the right scope sentence, and
is it now safe to answer the standing question "did walt ever test the
economy claim?" by pointing at this design — or does a genuine test
additionally require the U side to be cheapened, which would be a
further successor (a relaxation coarser than C, i.e. Theorem E6.5's
gluing ladder run from above)?
