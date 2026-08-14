# The n = 4 separation rung: Experiment E at the receipt-corpus
# coordinates (four tricks out)
# (design)

Status: **DESIGN, awaiting walt-math rulings.** This document is the
successor SEP-A10 prescribes: the n = 4 rung, rejected for the v1
separation probe, returned as its own design with its own adjudication.
Its questions are numbered **N4-Q1..N4-Q12** (the prefix is unused in
`walt/CENSUS-RULINGS.md`; checked by grep at authoring time). Nothing
here is built until walt-math rules.

Standing rulings inherit by name and are not restated: F1–F7, r3 Q1–Q5,
Y1–Y3, P-A1..P-A21, X-A1..X-A17, E-A1..E-A20, S-A1..S-A18,
R-A1..R-A24, PG-A1..PG-A18, J-A1..J-A18, DS-A1..DS-A36 and
SEP-A1..SEP-A19, together with Lemmas V, X, E, S, S-fold, S-det, R, G,
J, Propositions G-flat, J-0, J-1, J-win, Corollaries S-rigid, R-fold.
The mathematics is cited to the errata under DS-A17's citation rule:
**Lemma E3** and conditions **(C1)–(C4)** of §3.4, **Lemma E4** and
**Non-theorem E4′** with DS-A27's semantic obligation, **Corollary E4.1**
(SEP-A adjudication, pending its errata filing as §4.3), **Corollary
E3.2**, **Theorems E6.3–E6.5**. Freezes 1–43 are in force; this design
reserves content for **44** and **45** and fixes nothing itself.
**Tier: exploratory throughout.** No number this design produces becomes
quotable except by brief amendment adding it to a verifier receipt.
Vocabulary fence (D3, DS-A1, SEP-A1): witness, separation, receipt — the
word "certificate" does not appear in this design, in its code, in its
results file, or in any wiki text derived from it.

## What SEP-A10 rejected, and what this document answers

SEP-A10 rejected the n = 4 rung for v1 on three independent grounds and
prescribed a three-part repair. The repair is the spine of this
document and each part is answered in its own section:

| SEP-A10 ground | SEP-A10's required repair | Answered in |
|---|---|---|
| (i) the cost model is unstated, not merely large | (a) a measured single-world grade-4 timing rung **before** any full pass | §5 (the measured rung) |
| (ii) the U-side evaluator cannot declare a stop | (b) a declared per-(coordinate, action) budget in **deterministic units** on **every** evaluator, including the revealed path | §3 (the walk-step unit), §4 (the budgets) |
| (iii) the primary receipt is likely unavailable | (c) a declaration of what a coordinate reports when H does not complete | §6 (the H-unavailable regime) |

SEP-A10's closing sentence is the premise of this design and is printed
in the results header: *"Nothing about the n = 4 rung is unlawful; it is
unspecified."*

## 1. The object

The separation probe of `walt/SEPARATION-PROBE.md`, unchanged in its
mathematics, run at nine grade-4 coordinates instead of three grade-3
coordinates. For one coordinate (declared belief β = uniform over the
full void-free capacity fiber, declared field = uniform-legal, count-free
expected-focal-tricks valuation), and for each root action a:

- **L_a — the primal witness** (Lemma E4, DS-A14): the exact value of a
  FIXED lawful information-consistent policy with root action a,
  evaluated with no maximisation at any node below the root, maximised
  over a finite declared candidate set. L_a ≤ Q^H(a) always.
- **U_a — the upper witness** (Lemma E3, DS-A7, freeze 37): the
  action-conditioned treatment-C value E_β[V*_a] — root action held at
  a, world revealed before any later focal decision. Q^H(a) ≤ U_a
  always. The latent is ξ = ω on this carrier too, so treatment C and
  the document's C⁺ coincide (SEP-A6(b), DS-A20), and the results file
  says so.

**The root-action separation** (Theorem E6.4): if L_{a⋆} ≥ U_a for every
a ≠ a⋆ then a⋆ ∈ Opt^H(B) — membership, never uniqueness. The
member-not-set caveat is printed verbatim beside every SEPARATED verdict.

The primal ceiling of **Corollary E4.1(2)** applies here exactly as it
did at grade 3: the candidate at each H-optimal action is the H-argmax
policy, so L = Q^H necessarily, L is a receipt and not a measurement,
and every separation verdict is decided entirely by the upper witness.
SEP-A2's header sentence is printed verbatim and unchanged. (The
experiment in which that ceiling is deliberately abandoned is the
separate design `walt/ECONOMY-SUCCESSOR.md`, SEP-A17's successor; the
two are independent and neither is a prerequisite for the other.)

## 2. The coordinates, and what they are and are not

### 2.1 Provenance

Nine coordinates, quoted from
`walt-factory/results/fiber_probe_h_2026-08-11.txt` (S5h, exploratory
tier, quoted and not re-asserted): the rung-n=4 (trick 4) hands of rob's
receipt corpus `rob/receipts/verify_player.txt` whose leader **is** the
declaring seat, so H's root is the viewer's decision — **h0, h1, h2, h4,
h5, h6, h8, h9, h12**. Hands h3, h7, h10, h11 are out of scope (leader
is not the declaring seat) and are printed as such, as they are in the
S5h receipt. Each coordinate carries |X| = 34,650 = 12!/(4!·4!·4!)
worlds and four root actions. The kernel is built exactly as
`fiber_probe.rs::void_free_kernel` builds it: capacities from the
receipt state at the trick boundary, `voids: ContextSet::EMPTY`
deliberately, focal = the declaring seat (P-A4), pip-trump only (F1,
asserted in-run).

### 2.2 The fence that is specific to this rung, and it is the sharpest
### thing in this design

These coordinates come from **real deals**. Their hands and pools are a
real deal's hands and pools; **their belief is not the belief anyone
holds in that deal**, on two independent grounds, and both are printed
per coordinate before any verdict.

**Ground one — the voids are deliberately discarded.** `void_free_kernel`
sets `voids: ContextSet::EMPTY` although `voids_before_trick` computes
the voids the play record has already revealed (P-A2's void-free carrier;
`fiber_probe.rs` maintains both constructors side by side). At six of the
nine in-scope coordinates the discarded voids happen to remove nothing:
the S5h P-A2 receipt reports the void-filtered fiber equal to the
void-free fiber at h0, h1, h4, h6, h9 and h12. At the other three it
removes a great deal. Quoted exactly from that receipt, as exact
integers and exact rationals:

| coordinate | void-free fiber | void-filtered fiber | ratio |
|---|---|---|---|
| h0, h1, h4, h6, h9, h12 | 34,650 | 34,650 | 1 |
| h2 | 34,650 | 23,100 | 2/3 |
| h5 | 34,650 | 14,700 | 14/33 |
| h8 | 34,650 | 1,200 | 8/231 |

At h8 the declared carrier is 28 and 7/8 times the size of the set the
deal's own play record leaves feasible. A separation verdict at h8 is a
statement about a belief that assigns positive mass to 33,450 worlds the
deal has already excluded.

**Ground two — support is not belief, and this holds even at ratio 1.**
Even where the void-filtered and void-free fibers coincide, a uniform
belief over that support is a *declaration*, not the deal's information
state; the branch's 90-world witness (two legal histories, identical
support, opposite optimal leads) is the standing theorem that the two
are different objects. A coordinate at ratio 1 is not thereby a
statement about the deal.

**The printed fence, mandatory, verbatim, per coordinate and once in the
header:** *"the hand and pool of this coordinate are taken from a real
deal in rob's receipt corpus; its belief is not. The voids the play
record had already revealed are deliberately discarded (P-A2's void-free
carrier), and support is not belief in any case. No row in this file is
a statement about correct play in that deal, about reachability, or
about any belief other than the declared one. Fiber members are FEASIBLE
and never reachable (R-A2, P-A1)."*

The void-filtered fiber size is printed per coordinate as **provenance
only**, with the sentence *"this column licenses nothing: it is not a
belief, not a weight, not an error bar, and no verdict in this file is
conditioned on it."* It is printed because a reader who does not see it
will assume it is 34,650 everywhere.

### 2.3 Coordinate identity, and why freeze 36's key is not extended

The three grade-3 coordinates are named by the S6a unranking
`coordinate(grade, index)` (freezes 22–25), whose index encodes
(pip, live set, hand) and whose kernel always places the viewer at
`Seat::S0`. The receipt-corpus coordinates are **not** produced by that
unranking, and their viewer is whichever seat declared. Three routes
were considered.

- **Route A — rank the receipt coordinates into the S6a unranking.**
  Arithmetically available: `coordinate(4, index)` is a bijection between
  index and (pip, live set, hand) at grade 4, so a ranking function
  inverting `unrank_comb` would produce an index. It requires, however,
  that a coordinate whose viewer is S1, S2 or S3 be identified with its
  seat-rotated image at S0. That identification is almost certainly a
  transport — cyclic rotation preserves the partner-is-two-seats-away
  structure, the equal hidden capacities and the uniform-legal field —
  but the branch has **not exhibited** it. Corollary S-rigid and Lemma
  S-fold are tile transports, not seat rotations. Asserting an
  unexhibited transport to obtain a key is exactly the DS-A15 error, and
  this design does not do it. **Route A is declined and named** (N4-Q3
  asks whether walt-math wants it opened instead).
- **Route B — extend freeze 36's key with a corpus handle.** Rejected on
  SEP-A3(i)'s own ground: a corpus hand id and trick number are process
  handles, not canonical keys, and storing one makes the entry a second
  authority over an identity the semantic state already determines.
- **Route C — declare the n = 4 entries out-of-library (adopted).** No
  library entry is written at any n = 4 coordinate in this design.
  Freeze 36 is untouched, its key is unextended, and the grade-4 seed is
  extracted, priced, receipted and discarded within the unit. Three
  reasons, any one sufficient: (1) freeze 36(d) makes the file a cache
  and requires a loaded entry to be re-priced before use, so an entry
  buys nothing until a transport exists that could move it, and none
  does at grade 4; (2) the serialised size is prohibitive — the grade-3
  entries are 50,712 / 134,190 / 109,788 / 98,628 record lines
  (`walt-factory/store/candidate_library.txt`, 393,333 lines total), and
  §4.3's partition-size estimate of 24,825,150 states puts a single
  grade-4 entry near a gigabyte of text; (3) the key
  question above stays closed rather than being answered by an
  engineering convenience.

**The coordinate identity that IS printed and asserted** (freeze 45's
reserved content, N4-Q3): grade = 4; declaration pip; the viewer's hand
and the pool as canonical ascending-domino-index tile lists; the leader
offset from focal, asserted **0**; |X| = 34,650 asserted against
`kernel.count()`; the fiber enumeration order (freeze 7/23). The corpus
hand id and trick number are printed **as provenance only**, on their
own line, never as identity components — the same discipline SEP-A3(ii)
applies to pip. A reproduction of the coordinate from the printed
identity alone is asserted in-run: the probe rebuilds the kernel from
(pip, hand, pool, capacities, empty voids) and asserts it equals the
kernel `void_free_kernel` returned.

## 3. The deterministic budget unit: the walk-step

SEP-A10(ii) is the load-bearing repair and it is a change to
`walt-strat`, not to the probe. `walt-strat/src/info.rs::walk` is the
single traversal on which `hidden_root_values`, `revealed_world_root_values`,
`revealed_summary`, `InfoPartition::build` and `policy_value_receipt`
are all built. It takes no budget, returns no `Option`, and therefore
has no stop to declare — SEP-A10(ii)'s exact complaint.

### 3.1 The unit

**Definition (walk-step).** At each entry to `walk`, before any child is
visited, the traversal is charged `bag.len()` walk-steps: one unit per
(particle, node) visit.

This is deliberately **the same accounting rule** the scalar authority
already uses — `hidden_scalar.rs` charges `cost = parts.len()` at each
`node` and `node_dag` entry, and calls the result a particle-step. The
two are the same unit, and a walk-step count on the unmemoized envelope
walk is directly comparable to `MemoStats::tree_steps`, the exact
particle-step count the unmemoized tree walk would have charged. That
comparability is what makes §5's cost model quotable as arithmetic
rather than as guesswork.

The unit satisfies DS-A29 by construction: (a) it is a deterministic
budget unit and never wall-clock; (b) no clock, RNG or environment value
enters it; (c) it is a per-call local, so workers share no mutable state
through it; (d) it is an integer count and the values it guards are
exact rationals. No float appears anywhere in the accounting.

### 3.2 The budgeted-walk contract

`walk` gains `budget: &mut u64` and returns `Option<Envelope>`.

- **Charge:** at entry, `let cost = bag.len() as u64; if *budget < cost
  { return None } *budget -= cost;` — charged before any child call,
  exactly as `hidden_scalar::node` charges.
- **Propagation:** a `None` from any child returns `None` immediately.
  **No partial fold is retained**: no partially summed field node, no
  partially maximised focal node, no partial trick increment. This is
  not an optimisation, it is the correctness rule — a partially folded
  envelope is neither an upper nor a lower bound on the true one, and
  PG-A13's asymmetry is the precedent (an interrupted Pareto frontier
  bounds nothing in either direction).
- **Determinism of the stop point:** the charge depends only on the bag
  size and the traversal order, both functions of the kernel and the
  frozen enumeration, so the stop point is a function of (kernel,
  budget) alone. Two runs with the same budget stop at the same node.

Every caller becomes `Option`-returning, and every caller's `None` is a
declared stop printed R-A18-style. The six budgeted evaluators, all of
them, per SEP-A10(b)'s "every evaluator including the revealed path":

1. `hidden::hidden_root_values` — the envelope H path;
2. `revealed::revealed_world_root_values` and
   `revealed::revealed_summary` — the U path (§3.3);
3. `info::InfoPartition::build` — the partition traversal;
4. `info::policy_value_receipt` — the L path;
5. the probe's own unmemoized argmax-recording extraction solve
   (`separation_probe.rs::Extract::solve`, `policy_inspect.rs::Ctx::solve`'s
   shape), which has its own recursion and takes its own budget under the
   same charge rule;
6. `hidden_scalar::ScalarHidden::action_values_dag` — already budgeted
   under freeze 26, unchanged.

`price::information_prices` composes (1) and (2) and returns
`Option<InfoPrices>`; its two nonnegativity assertions and the §10.5
decomposition assertion are unchanged and fire only on a complete result.

### 3.3 The budget semantics `revealed_summary` must gain, in full

SEP-A10(ii) names `revealed_summary` specifically, so its contract is
specified rather than left to the caller.

**Signature.** `revealed_summary(kernel, focal, dir, budget: &mut u64)
-> Option<RevealedSummary>`.

**Budget scope.** One budget for the whole call — all 34,650 worlds and
all four root actions — decremented monotonically across the world loop
and the action loop inside `revealed_world_root_values`. It is **not**
per-world and **not** per-action. A per-world budget would make the stop
point depend on which world happened to be expensive rather than on the
declared total, and would let a call return after exhausting the budget
34,650 times over.

**The `None` return, and what is discarded.** `None` means the budget was
exhausted at some world index w and root action a in the frozen
enumeration order. On `None`:

- **all partial state is discarded** — no partial `q_c`, no partial
  `v_f`, no retained per-world envelope, no per-root accumulator. The
  function returns `None` and nothing else.
- The reason is (C2) of errata §3.4, quoted where the stop is printed:
  *"the same belief and the same world set are used on both sides — in
  particular no decimation may appear inside L or U: a sampled mean is
  neither a lower nor an upper bound, and a separation built on one is
  void."* A partial fiber sum is a sum over a proper sub-multiset of
  worlds. It is not U_a, it is not a bound on U_a in either direction,
  and it may not be printed as one, scaled into one, or carried forward.
- What **is** printed at the stop, because it is deterministic and costs
  nothing: the coordinate identity, the root action reached, the world
  index reached, the walk-steps charged, and the declared budget. These
  are counts of the run, not statements about the coordinate.

**Per-action subtotals.** The world-outer/action-inner loop makes
per-action walk-step subtotals free and deterministic; they are printed
as exact integers. They are an exact computational observable of the
declared traversal — in SEP-A19(b)'s class, alongside `InfoPartition::len()`
— and are never an information value, a decision width, a cost claim or
a term in the DS-A2 ladder.

**The freeze it needs.** Freeze **44**'s reserved content (N4-Q1): the
walk-step unit and its charge rule; the no-partial-fold propagation
rule; the `Option` contract on all six evaluators; the budget scope of
`revealed_summary`; and §4's budget and cap constants. Fixed by
walt-math, not here.

### 3.4 The regression the budget change owes the existing receipt

Adding a budget to `walk` changes the code path underneath the
**already-filed** grade-3 separation receipt. Mandatory, before any n = 4
number is produced:

**(R0) the grade-3 regression receipt.** `separation_probe` is re-run
under the budgeted `walk` with budgets set high enough not to bind
(asserted in-run: every evaluator's residual budget is printed and
asserted strictly positive), and
`walt-factory/results/separation_2026-08-13.txt` is reproduced
**byte-identically** except for its wall-clock provenance line. A
difference is stop-and-report — a defect in the budget change, never a
finding about the game (DS-A36's discipline, applied to a refactor
rather than to a resume).

## 4. The budgets, the caps, and where the numbers come from

### 4.1 The declared per-(coordinate, action) budget

Derivation, from `fiber_probe_h_2026-08-11.txt` (quoted, exploratory
tier). Its `tree-v0 steps` column is the exact particle-step charge an
unmemoized tree walk would have incurred at that coordinate over all
four root actions and the whole fiber, under the same charge rule §3.1
adopts. Over the nine in-scope coordinates the column runs from
1,855,419,966 (h6) to **16,211,488,002** (h9). Divided by four actions
and rounded up, the largest per-action figure is **4,052,872,001**.

**Declared budget B = 10,000,000,000 walk-steps per (coordinate, action)**,
for each of the four `walk`-based evaluators separately, and 4B for an
evaluator that covers all four actions in one pass (`revealed_summary`).
B exceeds twice the largest per-action figure above (2 × 4,052,872,001 =
8,105,744,002). The margin is deliberate and is not an estimate of
anything: the envelope walk's tree is not identical to the scalar
walk's, and a budget that binds by a small factor would turn a
correctness gate into a coin flip.

Two honesty clauses attached to B, both printed:

- B is a **ceiling, not a prediction**. The actual consumption is
  measured and printed per unit as an exact integer; nothing in this
  design claims what it will be.
- B is set from a **quoted exploratory receipt**. If the adjudicator
  prefers B derived from §5's measured rung instead — the DS-A33(i)
  pattern, where the rule is declared before the pass and the constant
  is fixed by the rung — that is N4-Q2 and this design accepts either.

### 4.2 The scalar authority's budget

Unchanged: `AUTHORITY_BUDGET` = 200,000,000 particle-steps, freeze 26,
cited and never re-declared. The measured n = 4 consumption is quoted
from the same S5h receipt and it is the reason §6 exists in a specific
form: the authority **completed at all nine** in-scope coordinates, with
`dag-v1` steps from 78,359,234 (h2) to **191,841,542 (h5)**. h5 sits
within 8,158,458 steps of the ceiling — less than one twentieth of the
budget remains (8,158,458 × 20 < 200,000,000).
The design records this as the standing risk it is: any change to the
solver, the valuation, the boundary memo or the budget can flip h5 from
GATE MET to GATE UNMET, and the gate is therefore asserted per run and
never inferred from this quotation.

### 4.3 The partition-size cap, and the memory problem this design does
### not hide

`InfoPartition::build` holds a `BTreeMap<Vec<Domino>, InfoStateId>` over
every reachable focal record, and the extraction solve holds a second
`BTreeMap<Vec<Domino>, Domino>` over the same key space. At grade 3 the
partitions were 50,712 / 134,190 / 109,788 / 98,628 states
(`separation_2026-08-13.txt`, quoted). The scalar authority's step count
grows from 1,033,720 at the largest grade-3 coordinate to 191,841,542 at
h5 in the quoted receipts, an integer growth factor of 185 by truncating
division; applying that factor to the largest grade-3 partition gives
134,190 × 185 = 24,825,150 states as an **estimate**, explicitly
labelled as one and licensing nothing. Two such maps
resident simultaneously, with heap-allocated `Vec<Domino>` keys, is a
multi-gigabyte working set.

Therefore:

- **Declared cap P_max = 32,000,000 partition states per (coordinate,
  action)**, checked incrementally at each insertion, with a declared
  stop on exceedance. PG-A13 governs what a capped unit may report: **no
  verdict for that action at all**, not a partial partition and not a
  bound.
- **A declared memory ceiling M_max** on the process, checked before the
  full pass and not during it, with the pass not run if §5's rung
  exceeds it. This design does **not** pick M_max — it is a property of
  Jason's machine, not of the mathematics, and N4-Q4 asks for it.
- The structural alternative is named rather than taken: the L walk
  looks up `partition.id(record)` only to index the policy, and the
  extraction map is already keyed by record, so pricing could run
  against the extraction map alone and halve the resident set. That
  would cost the SEP-A19 **totality** receipt its domain comparison
  (`choices.len() == InfoPartition::len()`), leaving only a cardinality
  comparison against a count-only partition pass — strictly weaker,
  since two different state sets of equal size would pass. This design
  does not weaken a receipt for memory. **N4-Q5** puts the trade to the
  adjudicator with the weakening named exactly.

### 4.4 Where the estimates are, and what they are not

Three estimates appear above: the per-action tree-walk figure behind B,
the grade-3-to-grade-4 partition growth factor, and the resident-set
consequence. All three are **cost-model inputs**, printed under a
heading that says so, and all three are marked with DS-A32/DS-A33's
discipline: they compare nothing against anything, they are not a
dividend, and no reading of this run may cite them. P-A21 additionally
binds and is printed: **three rungs are not a law, and no growth rate
measured at grades ≤ 4 is quoted for the opening** — including the
factor 186 above, which is arithmetic on two quoted receipts and not a
law of the game.

## 5. The measured single-world rung (SEP-A10(a)), and the go/no-go gate

Run **before** any full pass, at **W = 1 in a single uninterrupted
process**, from no checkpoint.

**Selection, declared in advance and never by result** (DS-A33(i)'s
rule, P-A15's style): the **first** coordinate in canonical unit order
(h0), its **first** root action in ascending domino index, and a
declared deterministic world sample — the 16 worlds at fiber indices
(i · g mod 34,650) for i = 0..15, with g declared coprime to 34,650 and
`gcd(g, 34650) = 1` asserted in-run (the freeze-8 decimation pattern).
"The two most interesting units" is forbidden by name.

**What it measures.**

1. `revealed_world_root_values` at each of the 16 sampled worlds: exact
   walk-steps per world (deterministic), and the exact min, max and sum
   over the sample.
2. `InfoPartition::build` at (h0, first action) to completion or to
   P_max: exact state count, exact walk-steps.
3. The extraction solve and one `policy_value_receipt` walk at the same
   unit, if (2) completed: exact walk-steps and the SEP-A13/SEP-A19
   counted receipt.
4. Wall-clock for each of the above, and the process's peak resident
   size, **as provenance**.

**The pre-declared extrapolation**, stated before any number exists:
estimated revealed walk-steps for the unit = (sum over the 16 sampled
worlds) × 34,650 / 16. It is an **estimate** and is labelled one: worlds
differ, the sample is 16 of 34,650, and the observed min/max spread is
printed beside the estimate so the reader can see how wide it is. It is
never a measurement of the unit.

**Typing, mandatory and printed at the head of the rung** (DS-A32,
DS-A33, SEP-A15(iii)): *"this rung is a cost model input. It produces no
ratio, compares no arm against any other arm, and is not a dividend. Its
walk-step counts are exact deterministic observables and are
load-invariant; its wall-clock and resident-size figures are
load-relative provenance and are quotable as nothing. No number in this
rung is a result about the game."* Because the rung contains a single
arm and forms no ratio, DS-A32's contention bias has nothing to bias —
which is stated, so that the absence of a `CONTENDED` label is not read
as an exemption.

**The go/no-go gate, declared before the rung runs.** The full pass is
**not run**, and this design returns for a runner adjudication, if any
of:

- the estimated wall-clock of the largest unit exceeds **ten minutes**
  (SEP-A10(i)'s own threshold, cited; `FIBER-PROBE.md`'s 5-minutes-per-check
  fast-iteration budget is background context and not the gate);
- the estimated walk-steps of any evaluator on the unit exceed its
  declared budget from §4.1;
- the partition state count exceeds P_max, or peak resident size exceeds
  M_max.

A gate failure is a **result** and is filed as one (F7, NO-RESCUE): it
is the measured cost model SEP-A10(i) said was missing, and it is
reported whether or not the pass follows.

## 6. What a coordinate reports when H does not complete (SEP-A10(c))

The separation logic is H-free — Theorem E6.4 needs only L_{a⋆} ≥ U_a —
but DS-A10's authorised receipts for Experiment E are H-conditioned, and
so is the provenance of the witnesses this probe builds (SEP-A12). The
regime is therefore specified in three tiers, and the tier is printed on
**every** row.

### 6.1 Tier 1 — GATE MET

`action_values_dag` completed within freeze 26's budget. Full receipts
(R1)–(R5) as SEP-A12 fixes them, minus the S6a cross-check (§6.4).
Verdicts printed normally. This is the tier the quoted S5h step counts
say to expect at all nine coordinates, and it is asserted, never assumed.

### 6.2 Tier 2 — GATE UNMET, witnesses complete

The scalar authority returned `None`, but the envelope H path, the
revealed path, the partition build, the extraction and the L walk all
completed within their §4.1 budgets.

- **What is asserted:** (R2) L = Q^H exactly per H-optimal action; (R3)
  the per-action price U_a − Q^H(a) with its sign assertion; (R5) the
  SEP-A13/SEP-A19 counted receipt; the certified action ∈ argmax of the
  envelope H path.
- **What is NOT asserted and is not printed:** (R1). The two-solver
  identification is unavailable, so the envelope path is the sole H
  authority at that coordinate and a defect in it would be invisible.
- **What survives, and it is worth naming:** (R2) is still a genuine
  cross-check. It ties `policy_value` — a third, structurally max-free
  code path — to the envelope H through the independently written
  extraction solve. R2 is the receipt that survives the authority gate;
  R1 is the one that does not.
- **What is printed beside every row of that coordinate**, R-A18-style:
  `correctness gate unmet` with the steps charged and the declared
  budget, never silently.
- **Tier language on the verdict**, mandatory and verbatim: *"VERDICT
  UNCROSSCHECKED. The validity of this separation does not cite H
  (Theorem E6.4 is H-free), and the verdict is mathematically sound
  under the declared belief, field, valuation and observation contract.
  Its provenance is a single uncrosschecked H solve: L's seed is an
  extraction solve and Q^H is the envelope path, with no independent
  authority agreeing with either. This row is outside the receipt set
  DS-A10 authorised for Experiment E, and it is exploratory tier as
  every row here is."*

### 6.3 Tier 3 — WITNESS STOP

Any of the envelope H path, `revealed_summary`, `InfoPartition::build`,
the extraction solve or the L walk returned `None`, or P_max was
exceeded. Then, precisely:

- **No U_a means no SEPARATED verdict involving a as competitor.** The
  separation condition is a conjunction over every competitor; an
  untested conjunct is untested, and a partial U bounds nothing (C2,
  §3.3). The coordinate verdict SEPARATED requires **every** competitor's
  U to have completed.
- **A NOT-SEPARATED verdict can still be reached from one completed
  pair.** If L_{a⋆} < U_a for a single competitor whose U completed, the
  pair verdict is exact and is printed in SEP-A16's exact-negative form,
  and the coordinate verdict is NOT-SEPARATED for that a⋆ regardless of
  what else stopped. Corollary E4.1(3) applies pairwise and does not
  need the other competitors.
- **No L_{a⋆} means no verdict for a⋆ at all**, in either direction.
- The asymmetry above is exact and is printed with the rows, in
  PG-A11-versus-PG-A13's style: *"a stop can complete a negative and can
  never complete a positive."*
- A stopped unit prints what was reached — evaluator, action, world
  index or state count, steps charged, declared budget — and nothing
  derived from partial state.

### 6.4 The receipt that has no analogue at this rung, said plainly

**(R4), the S6a cross-check, does not exist at n = 4.** The S6a receipts
`predictive_rank_2026-08-12.txt` file per-action Q values for the three
grade-3 unranking coordinates and for nothing else; there is no prior
filed value at any receipt-corpus coordinate to assert against. The S5h
receipt files step counts and fiber sizes, which are counts of a run and
not values of the game. Two consequences, both printed:

- the design does **not** invent a substitute and does not weaken R4
  into a comparison against a differently-defined prior quantity;
- at Tier 1 the cross-check burden rests on (R1) and (R2) together; at
  Tier 2 it rests on (R2) alone; at Tier 3 there is no verdict to
  cross-check.

`fiber_probe_h`'s `dag-v1` step counts and boundary-hit counts **are**
asserted as a determinism check where they are reproducible — the same
solver, same valuation, same budget, same coordinate should charge the
same steps — and that assertion is filed as **(R6) the step-determinism
receipt**, typed as a check on the runner and on DS-A29(a)–(b), never as
a check on any value. If the steps differ, the run stops and reports:
under DS-A29 a step count that moves without a declared cause is a
failure of load-invariance, not a finding.

## 7. The evaluators

Freeze 37 is unchanged and inherited entire: U_a is
`revealed::revealed_summary(...).q_c[a]` at the declared direction,
identified as E_β[V*_a] (Lemma E3); the root-maximising siblings (`v_f`
= U^agg = V^F, `fiber_probe.rs::aggregate`, `predictive_rank.rs::fused`)
are named once and never confused with it; the per-action price is
`price.rs::information_prices().g_cont_by_root[a]` and **that column is
the measurement** (SEP-A5(ii)). L_a is `info.rs::policy_value` with the
SEP-A13 singleton assertion at every focal callback and the SEP-A19
counted receipt. The two-solver identification of freeze 37(g) —
envelope H at `Direction::trick_diff()` against `ScalarHidden` at
`ScalarValuation::trick_only`, asserted **equal exactly, per action, with
no bridge** — is unchanged, and the root is asserted trick-leading so
both solvers price the same action list (which the coordinate selection
guarantees: leader offset 0).

The seed rule is freeze 36(f) unchanged: the **unmemoized**
argmax-recording pooled H solve over the same partition, tie rule cited
to freeze 26 and never re-declared, contributing no number to any
reported L; `action_values_dag` can never supply it (SEP-A11(ii)).
Reporting convention: **count**, with the freeze-26 bridge
Q_diff = 2·Q_count − grade asserted at the reporting boundary only, and
the bridge affine with slope 2 > 0 so every verdict is
convention-invariant. Here grade = 4, and the probe asserts the grade it
substitutes into the bridge against the coordinate's declared grade —
a grade-3 constant silently reused at grade 4 is the obvious way for
this rung to produce well-typed wrong numbers.

**T7 / (C2):** both L and U are computed over the full |X| = 34,650
fiber. No decimated world set from any probe — including
`fiber_probe`'s W = 240 sets at this very rung, and including §5's
16-world timing sample — appears inside any L or U. The timing sample is
used for timing and is asserted, in code, never to reach an accumulator
that feeds a reported value.

## 8. Checkpointing, resumption and parallelism

**Decision: checkpointing YES, at unit granularity = one (coordinate,
root action) pair — 36 units.** DS-A30's machinery from S6c is reused
(`walt-factory/store/` alongside `deadness_ckpt`).

Justification, since the brief asks for one rather than a default:

- The failure mode DS-A30 exists for is present here and was not present
  at grade 3. The grade-3 run completes in seconds (its wall-clock
  provenance line reads 3,942 ms for all three coordinates); §4.1's
  budget ceilings put a grade-4 unit three to four orders above that,
  and a pass that can be killed at an arbitrary instant is exactly
  DS-A30(iv)'s case.
- **(coordinate, action) is the smallest completed adjudicated unit.**
  DS-A30(v) forbids partial-unit checkpoints. The U side, the extraction,
  the partition, the L walk and the pair verdicts are all per-action; the
  authority gate is per-coordinate and is carried on every unit record of
  that coordinate. There is no smaller lawful boundary — in particular
  **no sub-world checkpoint inside `revealed_summary`**, which would
  persist a partial fiber sum, i.e. exactly the object (C2) forbids
  anyone from trusting.
- DS-A30(vi) applies directly: a unit's declared stop is itself a
  checkpointed outcome, reproduced by a resumed run rather than re-run
  into a different one, which §3.2's determinism guarantees.

Binding, inherited by name: freeze 41's record format with the
freeze-set digest on every record; a digest mismatch is **corrupt, not
stale** and the cache is discarded **entire** (DS-A30(i)–(ii)); atomic
write via temp file and rename (iv); the cache is a cache and never an
authority (X-A17), so a resumed run **re-runs a declared sample of
loaded units and asserts byte-identical non-timing output** before
quoting anything (iii). **The declared sample rule, fixed in advance:**
the first unit in canonical order that was loaded from cache — a
deterministic choice, never "an interesting one".

DS-A31's provenance lines are printed: FRESH or RESUMED; the freeze-set
digest and units loaded versus computed; the cold regenerate path from
an empty cache directory; and, for every timing quantity, the identity
of the process that produced it. A resumed run inherits counts and
receipts freely and **inherits no quotable timing at all** — which costs
this design nothing, since it quotes no timing as a dividend anywhere.

**Parallelism.** W-way over the 36 units is lawful and permitted, with
DS-A29(a)–(d) asserted in-run rather than assumed: the evaluators hold
no shared mutable cache (`action_values_dag`'s cache is per-call), no
clock or RNG enters any decision, every stop is in walk-steps or
particle-steps, and the arithmetic is exact rationals throughout. W is
**recorded, not frozen** (DS-A34), alongside CPU model, core count and
build profile (P-A19). Multi-process copies over one checkpoint
directory are OUT (DS-A35). §5's rung runs at W = 1 as part of its
declaration.

**Results-file structure (DS-A36):** two clearly separated blocks — a
**deterministic block** carrying every count, receipt outcome, exact
value, price, verdict and step count in canonical unit order, and a
**timing block**. The deterministic block is byte-identical across
fresh, resumed and any W, and the validation is the cheap one DS-A36
names: run one coordinate fresh, run it again resumed, byte-compare. A
difference is stop-and-report.

## 9. The run

Output `walt-factory/results/separation_n4_<date>.txt`, regenerated by
`cargo run --release -p walt-factory --example separation_probe -- n4`
(the same example, a declared rung argument; the grade-3 path is
unchanged and (R0) proves it).

Canonical unit order (freeze 44's reserved content): coordinates in
corpus order h0, h1, h2, h4, h5, h6, h8, h9, h12; within a coordinate,
root actions in ascending domino index. 36 units.

Header, before any number: the SEP-A2 primal-ceiling sentence verbatim;
the SEP-A12 provenance-typing sentence verbatim; the freeze-37(h) budget
honesty sentence, **amended** to state that under freeze 44 the
`walk`-based evaluators now carry budgets and declared stops, and that
the sentence "hidden/revealed/price/policy_value carry no budget and no
stop" is superseded for this rung and remains true of the grade-3
receipt as filed; §2.2's real-deal fence; the R-A2 reachability fence;
the treatment-C naming clause; the cost-model-input heading; SEP-A15(iii)'s
no-cost-claim sentence; and P-A21's no-growth-law sentence.

Per coordinate:

1. Coordinate identity asserted first (§2.3): grade 4, pip, hand, pool,
   leader offset 0, |X| = 34,650, enumeration order; kernel rebuilt from
   the printed identity and asserted equal. Corpus provenance printed on
   its own line. Void-filtered fiber size printed with its
   licenses-nothing sentence.
2. Scalar H authority per action, freeze-26 budget, R-A18 gate line;
   root asserted trick-leading; tier (§6) fixed and printed.
3. Envelope H per action; **(R1)** exact per-action equality with the
   scalar authority, no bridge — at Tier 1 only.
4. **(R6)** the step-determinism check against the quoted S5h `dag-v1`
   step count and boundary-hit count, where reproducible.
5. **(R3) the measurement:** the per-action price U_a − Q^H(a) as an
   exact rational for every action, from `g_cont_by_root`, with its sign
   assertion; the aggregate gap V^F − V^H printed with SEP-A15(i)'s
   one-sided-screen paragraph verbatim, including Corollary E3.2's zero
   case as the only implication it licenses.
6. Candidate extraction at every H-optimal action (all tied argmaxes) by
   the freeze-36(f) seed rule. **No library entry is written** (§2.3,
   Route C); the extraction map is asserted total on the partition and
   then dropped.
7. **(R2)** L per H-optimal action via `policy_value`, asserted
   **exactly equal** to Q^H; strict inequality is stop-and-report naming
   SEP-A11(i)'s three defects. **(R5)** the SEP-A13/SEP-A19 counted
   receipt — focal callback invocations = singleton expansions =
   distinct partition states reached — with SEP-A19(b)'s typing sentence
   beside it and the ratio not printed.
8. The separation table: per H-optimal a⋆ and competitor a, L_{a⋆},
   U_a, margin, pair verdict; coordinate verdict SEPARATED iff some a⋆
   separates against every competitor; NOT-SEPARATED pairs in SEP-A16's
   exact-negative form with the failing gap U_a − Q^H(a⋆) as an exact
   rational; certified action asserted ∈ argmax_H; member-not-set caveat
   verbatim beside every SEPARATED verdict; tier language per §6.
9. Per-unit walk-step consumption and residual budget, exact integers,
   in the deterministic block.

## 10. The reading, pre-declared before any number exists

**SEPARATED at a receipt coordinate** certifies a⋆ ∈ Opt^H(B) at that
coordinate under the declared belief, field, valuation and observation
contract — and nothing else. Specifically it is **not** a statement
about the deal the coordinate came from (§2.2, both grounds), not a
reachability assertion (R-A2; fiber members are FEASIBLE), not
uniqueness (Theorem E6.4's member-not-set caveat), not transport, not an
opening claim, and not a claim about grade 5 or the opening by any
growth argument (P-A21).

**NOT-SEPARATED** is the exact negative of Corollary E4.1(3): the pair
(a⋆, a) satisfies Q^H(a⋆) < U_a, which proves that **no candidate set
whatsoever** separates that pair under relaxation C at that coordinate.
The failing gap is printed as an exact rational with that sentence, and
never as "this run's candidates were not strong enough". Failing pairs
are exactly where a gluing cut would have to bite (Theorem E6.5, DS-A3's
cut typing — a cut constrains the relaxation, never the lawful policy
class and never the fiber) and are **the input Experiment D needs**;
the results file lists them under that heading. A rung that produced
several would be a more useful result than a clean sweep, and the design
says so in advance so that a sweep is not read as the better outcome.

**A STOPPED unit** is neither a positive nor a negative and is never
presented as a weak version of either (PG-A13's discipline). A gate
failure at §5 is likewise a result: it is the measured cost model.

**The three sentences that must not be blurred**, all printed:

1. This rung tests whether root-action separation closes **four tricks
   out at real-deal coordinates**, one grade above the adjudicated
   grade-3 run.
2. It does **not** test the parent's economy claim ["the solver does
   **not** need an exact solution for every action"]: it computes the
   exact H solve at every action, because Corollary E4.1(2) is how L is
   obtained and because §6's receipts are H-conditioned. That experiment
   is `walt/ECONOMY-SUCCESSOR.md`, SEP-A17's successor.
3. It licenses **no cost, timing, runtime or tractability claim of any
   kind** (SEP-A15(iii), P-A19, DS-A32, DS-A33). The measured rung of §5
   is a cost model input; wall-clock is provenance; step counts are
   exact observables of a declared traversal and are not a complexity
   statement.

**Both outcomes are results** (F7, NO-RESCUE). Nothing is promoted;
exploratory tier; a number becomes quotable only by brief amendment
adding it to a verifier receipt.

## 11. Questions for adjudication (N4-Q1..N4-Q12)

**N4-Q1 (freeze 44 — the walk-step unit and the budgeted-walk
contract).** Is §3's reserved content right and complete: the charge
rule `bag.len()` at node entry before any child; the no-partial-fold
propagation rule; the `Option` contract on all six evaluators of §3.2;
`revealed_summary`'s whole-call budget scope with per-action subtotals
printed; and the canonical unit order of §9? In particular, is the
identification of the walk-step with `hidden_scalar`'s particle-step —
same rule, different traversal — sound enough to license §4.1's
derivation of B from the quoted `tree-v0` column, or must the two units
be given different names and no arithmetic be carried between them?

**N4-Q2 (where B comes from).** §4.1 sets B = 10,000,000,000 walk-steps
per (coordinate, action) from the quoted S5h `tree-v0` maximum. The
alternative is DS-A33(i)'s pattern: freeze the *rule* now and let §5's
measured rung fix the *constant* before the pass. Which does walt-math
want, and if the second, what multiple of the measured figure?

**N4-Q3 (coordinate identity, and freeze 45).** §2.3 adopts Route C —
content-based identity, corpus handle as provenance only, and **no
library entry at n = 4**, leaving freeze 36's key unextended. Is that
right? Or does walt-math prefer Route A — opening a **seat-rotation
transport** ruling (viewer S1/S2/S3 ↦ S0 by cyclic rotation, which
preserves the partner-at-distance-two structure, equal hidden
capacities and the uniform-legal field) so that the receipt coordinates
acquire genuine S6a indices? Route A is declined here only because the
transport is unexhibited, not because it is doubted; DS-A15 forbids
assuming it.

**N4-Q4 (the memory ceiling M_max).** §4.3 declares a partition-state
cap P_max = 32,000,000 from an explicitly-labelled estimate, and
declines to pick a resident-size ceiling because that is a property of
the machine and not of the mathematics. What M_max should the go/no-go
gate use, and should P_max instead be derived from M_max rather than
from the growth-factor estimate?

**N4-Q5 (the partition/extraction memory trade, with the weakening
named).** Pricing could run against the extraction map alone, halving
the resident set, at the cost of degrading SEP-A19's **totality**
receipt from a domain comparison (`choices.len() == InfoPartition::len()`)
to a cardinality comparison against a count-only partition pass — under
which two different state sets of equal size would pass. This design
does not take the trade. Does walt-math accept the weakening if §5's
rung shows the two-map form does not fit, and if so with what
compensating receipt?

**N4-Q6 (the H-unavailable regime).** Is §6's three-tier scheme right,
and specifically: (i) at Tier 2, is (R2) correctly characterised as the
cross-check that survives the authority gate, and is the mandatory
verdict language ("VERDICT UNCROSSCHECKED", quoted in §6.2) the right
tier language; (ii) at Tier 3, is the asymmetry correct — a completed
failing pair suffices for NOT-SEPARATED, while SEPARATED requires every
competitor's U to have completed; (iii) is a Tier-2 row properly
described as outside the receipt set DS-A10 authorised, and should it
therefore be excluded from any wiki text at all rather than carried with
its label?

**N4-Q7 (R4 has no analogue).** §6.4 states plainly that there is no
prior filed value at any receipt-corpus coordinate, declines to invent a
substitute, and offers **(R6)**, the `dag-v1` step-determinism check
against the quoted S5h counts, typed as a check on the runner and on
DS-A29(a)–(b) and never on a value. Is (R6) lawful as typed, and is a
step-count mismatch correctly ruled stop-and-report rather than a
finding?

**N4-Q8 (the real-deal fence).** Is §2.2 sufficient? The concrete
number — at h8 the void-free carrier is 34,650 against a void-filtered
1,200, a ratio of 8/231 — suggests a stronger option: **excluding** h2,
h5 and h8 from the rung, or running them under a separately-typed
heading. This design keeps all nine and prints the column, on the
grounds that the carrier is declared and the verdict is about the
declared carrier, but a reader who conflates them is one careless
sentence away from a false claim about a real deal. Which does walt-math
want?

**N4-Q9 (checkpointing).** Is §8's decision — checkpoint at
(coordinate, action) granularity, 36 units, no sub-world checkpoint
inside `revealed_summary`, declared re-run sample = the first
cache-loaded unit in canonical order — correct under DS-A30? Is the
per-coordinate authority-gate outcome correctly carried on every unit
record of that coordinate rather than in a separate per-coordinate
record?

**N4-Q10 (the grade-3 regression receipt).** Is (R0) of §3.4 — re-run
the adjudicated grade-3 probe under the budgeted `walk` and reproduce
`separation_2026-08-13.txt` byte-identically except for the wall-clock
line, with every residual budget asserted strictly positive — the right
form, and should it be a blocking precondition of the n = 4 pass rather
than a receipt printed alongside it?

**N4-Q11 (the grade constant in the bridge).** §7 requires the probe to
assert the grade it substitutes into the freeze-26 bridge
Q_diff = 2·Q_count − grade against the coordinate's declared grade, on
the ground that a grade-3 constant silently reused at grade 4 produces
well-typed wrong numbers. Is that assertion sufficient, or should the
bridge be re-stated as a function of the coordinate rather than of a
constant anywhere in the code?

**N4-Q12 (the go/no-go gate, and what a gate failure is).** §5 declares
that a gate failure is itself a filed result — the measured cost model
SEP-A10(i) said was missing — and that the pass then returns for a
runner adjudication rather than proceeding with a reduced coordinate
set. Is that right, or should the design pre-declare a **reduced rung**
(for instance the three cheapest coordinates by quoted `tree-v0` steps:
h6 at 1,855,419,966, h4 at 2,442,873,158, h8 at 3,016,730,096) so that a
gate failure has a declared fallback rather than a return?
Pre-declaring the fallback now is the only way it can be selected by
rule rather than by result. **This question interacts with N4-Q8:** the
cheapest-three rule selects h8, which is the coordinate with the widest
deal-belief divergence (void-filtered 1,200 against void-free 34,650, a
ratio of 8/231). If N4-Q8 excludes h2, h5 and h8, the fallback set must
be re-derived under the exclusion rather than patched afterwards, and
the two rulings should be given together.
