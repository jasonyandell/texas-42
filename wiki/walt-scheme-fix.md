# Scheme/Fix — the descriptor language, and how to use it

[Home](Home.md) · owns: the Scheme/Fix descriptor language — what a descriptor is, what the language is for, how to read and write one, and how much of it is built · Sources: walt v0.4 §3 (Scheme and Fix as typed relational queries), §12.1 (static descriptor factorization), §12.5 (dynamic control skeleton), §12.6 (controlled lumpability), §12.7 (Scheme/Fix as a control-skeleton language), §12.9 (counterexample-guided synthesis), §16.11 (experimental record schema), §17.4 (open questions); `walt/math/equivariant_lumpability_v0.5.md` (§12.6A); `walt/math/implementers_guide.md` §1.20–1.21; `walt/PLAN.md`, `walt/LOG.md`; `walt/walt-skeleton/src/`, `walt/walt-factory/src/lesson.rs`; the results files named inline.

> **EXPLORATORY TIER — the whole page.** walt sits on its own frozen exploratory-tier basis
> (`walt/math/`). Nothing here is a corpus status, a Lean kernel proof, an
> exchange-adjudicated CONFIRMED, or a rob conformance receipt. Every number below is
> computed evidence about one declared finite domain, from a single Rust implementation,
> and is never an axiom (TRUST-01). The fence is on the [walt hub](walt.md).

Siblings: [walt hub](walt.md) · [foundation era](walt-foundation-era.md) · [factory era](walt-factory-era.md) · [census era](walt-census-era.md) · [decision-sparse track](walt-decision-sparse.md) · [instruments](walt-instruments.md) · [math reference](walt-math-reference.md).

---

## 1. What is a descriptor?

A **descriptor** is a compressed stand-in for the hidden state of a hand. The seat cannot
see the other three hands and wants to carry something far smaller than "which of these 90
worlds am I in" while still deciding correctly.

The tempting shape is a *labeling*: a function that looks at the true world and returns a
summary. walt refuses that. Here a descriptor is a **transducer** — typed state plus a
closed update law, `d' = step(d, obs)` (v0.4 §12.5's `D_{t+1} = δ_D(D_t, a_t, o_{t+1})`).

The distinction is the whole point. A labeling is evaluated *against the world*; evaluate it
again after a trick and you consult the world again. That is fine for an analyst with
God's-eye access and useless for a seat, which never had the world to consult. A transducer
reads the world **once**, at the root, and thereafter advances on public observations alone.
Whatever it still knows at trick six, a real seat could also have known.

walt makes this a type property rather than a promise (`walt-skeleton/src/skeleton.rs`):

```rust
pub trait ControlSkeleton {
    type State: Clone + Ord + core::fmt::Debug;
    fn name(&self) -> String;
    fn kind(&self) -> UpdateKind;
    fn init(&self, kernel: &Kernel, world: &World) -> Self::State;
    fn step(&self, d: &Self::State, obs: ObservedPlay) -> Self::State;
}
```

`init` is the only place a latent `World` appears. `step` sees the state and one observed
play — so an implementation that wanted to recompute from the hidden world at trick four
cannot: it has no world in scope. Non-closed updates are *unconstructible*, not merely
discouraged.

One escape hatch exists and is labelled. `UpdateKind::StaticPassenger` marks a descriptor
whose `step` is the identity — a frozen root evaluation riding the dynamic harness (that is
what `StaticWrap` builds, naming it `static[...]`). Passengers are allowed so the older
static-compression experiments can re-run through the new machinery, but the search
objective prefers closed updates and every report prints the kind.

Two inherited disciplines: descriptor state is a **derived view**, never a stored authority
(equality and ordering through projected content only), and it is never an identity-bearing
record of reachability. And support is not belief — a descriptor cell is a set of
rule-compatible worlds, not a weighting over them.

## 2. What is Scheme/Fix for?

Scheme/Fix is the **language in which a class or a situation becomes sayable**. walt's
census machinery can hand you a class of situations and prove things about it, but cannot
yet *say what the class is* in any form shorter than the class itself. §12.7 names that gap
and proposes the language to close it.

The syntax comes from v0.4 §3:

- A **role schema** `Σ = (N_Q, N_C, N_D)` names effective-context roles, chair (seat) roles,
  and domino roles; an interpretation binds those names to concrete contexts, seats, tiles.
- An **output interface** `O ⊆ Σ` says which names are *returned* and which are merely
  internal existential witnesses. Load-bearing: internal proof choices must not leak out as
  extra referents, extra probability mass, tracked identities, valued objects, or public
  observations. `O = ∅` is a Boolean event query; `O = Σ` a full witnessed realization query.
- A **Scheme case** `S = (π, φ)` pairs an *equality pattern* π — which distinct role names
  may denote the same object — with a finite conjunction φ of registered atoms (`Live(e)`,
  `Holds(c,e)`, `In(e,q)`, `Beats(e,f,q)`, `Void(c,q)`, `Team(c,t)`, and registered derived
  predicates).
- A **Fix** `F = S_1 ∨ … ∨ S_r` is a finite disjunction of Scheme cases over one common
  schema and output interface. The empty Fix is false.

Two rules matter more than the grammar. Every derived continuation atom (companion,
forced-follower, beater chain, mobility) must declare its **horizon** and its **information
access**. And a predicate that calls the target solver or reads the response class is
**forbidden target leakage**: you may not define an atom as "the one that gets the answer
right."

The language sits *above* the physics. `walt/PLAN.md`: "Scheme/Fix as a query language
enters later inside walt-skeleton's descriptor vocabulary; it imports physics, never the
reverse." A Fix asks a typed relational question inside the worlds of a decision state; it
never replaces the decision state `B = (K, e, β)`, and its result is an *answer bundle over
worlds* — not automatically a Boolean, not a unique referent, not a distribution. A Scheme
cell is also not a symmetry orbit and not a purpose class (§11.1): it can contain part of one
orbit, several orbits, or several strategic classes, and there is no purpose-free canonical
class.

## 3. The two axes every descriptor is graded on

### Axis 1 — soundness (§12.1): does the answer survive the compression?

With a finite domain `X`, an exact target response `R*: X → Y`, and a descriptor `D: X → Z`,
`D` is **purpose-sound** when `D(x) = D(y)` implies `R*(x) = R*(y)`; equivalently, `R*`
factors uniquely as `R* = R̄ ∘ D`. Intuition: you may merge two worlds only if the answer you
care about is the same in both.

Soundness is *purpose-relative*. There is no sound descriptor, only a descriptor sound for a
stated target — walt's own runs show one descriptor sound for one target and unsound for
another on the same fiber.

`check_soundness` (`walt-skeleton/src/soundness.rs`) enumerates the entire fiber, asserts the
world count against the exact counting DP, and reports `(worlds, cells, responses)` — the
"90 to 33 to 8" shape. On failure it emits a §12.9 step-4 witness: two concrete worlds in one
cell with different responses.

### Axis 2 — lumpability (§12.6): does the compression survive the dynamics?

Soundness is about one moment; a seat plays a whole hand. §12.6's target is **strong
controlled lumpability**: whenever `d(x) = d(y)`, (1) the legal focal action sets agree,
`A(x) = A(y)`; and (2) for every legal action, feature increment, observation, and successor
class, the class-aggregated probabilities agree.

Condition 1 says two states in one class must offer the seat the same menu, or the class
cannot name an action. Condition 2 says the class-level transition law is well defined: from
anywhere inside the class, the chance of "score this much, see that, land in class `y'`" is
the same. When both hold, the abstract kernel exists, abstract belief updates using it alone,
the joint law of observations and accumulated features is identical concretely and
abstractly, and every utility and every optimization over the same abstract-policy class gets
the same value. Exact dynamic compression, not approximation — and the spec is explicit that
the bar is deliberately stringent: weaker belief-dependent or policy-relative quotients may
exist and would need their own theorems.

`check_lumpability` (`walt-skeleton/src/lumpability.rs`) builds the whole reachable carrier —
every fiber world, every reachable viewer-decision node, every legal action, every
positive-mass field segment, exact rationals, nothing sampled — and returns
`LumpabilityFailure::LegalSets` (condition 1), `LumpabilityFailure::Kernel` (condition 2,
with the disagreeing event and both masses), or a pass. The field is the fixed
uniform-over-legal chance law of §7.4; each kernel row is asserted to sum to one exactly.

**§12.6A, the equivariant version.** v0.4 §12.6 compares interfaces *literally*, so two
situations differing only by which tile plays a role can never merge — under that reading
only world-reconstructing skeletons pass. The v0.5 amendment replaces literal equality with
equality **up to declared typed transports** `Θ` on roles, actions, and observation labels,
with outcomes compared count-free; coherence is required, and the transports are explicitly
*not* claimed to be global symmetries of Straight 42. That is the axis the census work runs
on — see [census era](walt-census-era.md).

## 4. How to read a descriptor

No printed Scheme/Fix formula exists anywhere in walt, because the language is not built
(§7). What *is* printed is the **lesson implicant language**: a conjunction of typed cells
with a graded, labelled verdict. It is the closest existing thing to a Fix and is what you
meet in the results files. Verbatim from
`walt/walt-factory/results/lesson_basins_2026-08-10_r4.txt`:

```
walt lesson (S5b) — exploratory tier
origin: regret conflict h0 S1 t5 p0 fiber 1680 grade worldwise-dominance at (C, minimax-omniscient)
  chosen 3-2 better [0-0 2-1] regret 61/21
verdict: refutation: value(decisive) >= value(max-count) at every matching (decision, world)
grade: worldwise at (C, minimax-omniscient); weighting-free
domain: receipt corpus, hands 0-12, all seats, tricks 5-6, fiber <= 40000 — 104 decisions, 23790 worlds, all fibers exhaustively enumerated (0 in-range decisions excluded by the fiber cap)
initial implicant (25 cells): hand=0 & seat=S1 & decl=P3 & role=declaring & horizon>=3 & horizon<=3 & ply=0 & beaters-total(1-1)=0 & beaters-total(2-0)=4 & beaters-total(2-2)=0 & beaters-total(4-1)=2 & beaters-total(4-2)=1 & beaters-total(4-4)=0 & beaters-total(5-1)=1 & beaters-total(5-2)=0 & beaters-total(6-2)=0
```

- **`origin`** — the conflict being generalized. A *conflict* is a refuted line: at hand 0,
  seat S1, trick 5, ply 0 (the seat led), the walker chose 3-2 while 0-0 and 2-1 were better,
  exact rational regret 61/21 over a 1,680-world fiber.
- **`verdict`** — three shapes exist. `refutation` (one action's value is at least another's
  at every matching decision and world), `win` (an action attains the world optimum —
  per-world sufficiency, explicitly *never* a seat-facing guarantee, since §7.6's
  strategy-fusion gap means no single information-consistent strategy need achieve per-world
  values), and `not-lumpable` (a named descriptor family fails §12.6 at every matching
  decision).
- **`grade` and the operator pair** — the two labels no verdict may be quoted without. Grade
  is the rung it was verified at: `worldwise` (every world of every matching fiber,
  weighting-free), `exact-expectation` (under a declared weighting), `sampled` (marked as
  such), or `checker` (the exhaustive §12.6 checker). The operator pair
  `(C, minimax-omniscient)` is a *pair*: a focal-information coordinate (F, C, or H — what
  the focal side may condition on, §10.3) and a field coordinate (omniscient adversarial
  minimax, or the fixed uniform-legal chance law, §10.8). Never one rung alone — §12.4 makes
  results label-relative, and the same measurement at another pair can shatter or merge. The
  origin keeps its own grade unchanged: a lesson verified worldwise does not upgrade a
  sampled origin.
- **`domain`** — the DomainSpec, and the application gate.
  `DomainSpec::covers(trick_no, fiber)` is consulted *before anything else*: a decision
  outside the trick range, or with a fiber above the cap, was never verified, so the lesson
  simply does not apply there. Capped decisions are excluded, never sampled — and when a cap
  bites, runs print the control-bias annotation, because fiber size anti-correlates with
  focal control, so an excluded set skews low-control.
- **`initial implicant`** — the conjunction describing the origin. **Decision cells**
  (`hand`, `seat`, `decl`, `role`, `ply`, `horizon`) are public facts of the decision point.
  **Atom cells** (`beaters-total(2-0)=4`, `team(1-1)=false`, `bestkeep=true`) are latent
  facts from the atom vocabularies. An atom cell is **partial**: where its precondition fails
  — tile not in the hidden pool, companion undefined — it is *unsatisfied*, never defaulted.

The trace is where the generalizer's reasoning shows:

```
trace:
  drop hand=0 -> dropped
  drop seat=S1 -> dropped
  drop beaters-total(1-1)>=0 -> dropped
  drop beaters-total(1-1)<=0 -> dropped
  ...
  introduce beaters-total(2-0)>=4 (cut refinement) excluding witness h4 S1 t5 p0 world S0={4-3 4-4 5-4} S1={2-1 4-0 5-1} S2={0-0 1-1 2-0} S3={2-2 4-2 5-5} values 2-1=9 4-0=-11 5-1=9
  drop decl=P3 -> dropped
  drop ply=0 -> dropped
  introduce beaters-total(1-1)<=0 (cut refinement) excluding witness h10 S0 t5 p1 world S0={3-0 4-4 6-5} S1={2-0 3-2 6-1} S2={2-2 3-3 4-0} S3={5-3 6-3} values 3-0=-6 4-4=-6 6-5=4
  drop role=declaring -> dropped
  drop horizon>=3 -> dropped
  drop horizon<=3 -> dropped
final implicant (2 cells): beaters-total(2-0)>=4 & beaters-total(1-1)<=0
```

Start from a 25-cell description of one concrete decision and try to throw each cell away,
re-verifying exhaustively over the whole domain after every attempt.

- **`drop X -> dropped`**: the cell came out and the verdict still held everywhere.
- **`drop X -> SURVIVES; witness …`**: the drop was refuted by a concrete counterexample and
  the cell was restored — load-bearing, with the witness naming why.
- **`introduce X (cut refinement) excluding witness …`**: a widening failed, so rather than
  give up the generalizer *added* a world-selecting cell that is constant across the
  already-verified set and false-or-undefined at the witness, then re-verified. This is the
  lesson-level twin of §12.9 steps 5–6. There is a hard budget (`intro budget: 2/4 spent`)
  and the exclusion property is enforced by construction.

**Why bounds relax while equalities can only be kept or deleted.** Numeric facts enter as
*pairs* of one-sided bounds: `beaters-total(2-0)=4` is stored as `>=4` together with `<=4`
and printed as the equality only when both halves survive. An equality cell is atomic — keep
it or delete it. A bound cell has a **relaxation ladder** (`>=4` weakens to `>=3`, to `>=2`,
to vacuity), so the generalizer can widen one step at a time. That was adopted for a concrete
observed reason: under the earlier equality-only scheme, four lessons died with zero basin
because a horizon pin could not bend. A real relaxation, same file:

```
  relax horizon>=4 -> BOUND HELD at horizon>=4; witness h2 S0 t5 p1 world S0={3-0 3-1 4-3} S1={1-0 1-1 2-1} S2={2-2 3-2 6-6} S3={4-1 5-4} values 3-0=-1 3-1=-1 4-3=-13
  drop horizon<=4 -> dropped
final implicant (1 cells): horizon>=4
```

The upper half went entirely; the lower half relaxed as far as verification allowed, then
*held* at its original value, named with that value and the witness refuting the next step.

Finally the measurement:

```
carrier: selector-resolvable decisions x their fiber worlds — eligible 32 decisions / 14387 worlds (domain context: 104 decisions / 23790 worlds, not a rate base)
basin [at grade: worldwise at (C, minimax-omniscient); weighting-free]: decisions 1/32 worlds 1680/14387 triple (488,1192,0)
basin dominance class: W (weak, strict somewhere)
```

The **basin** is everything the final implicant matched and verified. Its rate base is the
verdict's own **carrier** (§11.1: measures are carrier-relative), never the whole domain,
which appears as labelled context and is explicitly "not a rate base". The **triple**
`(gt, eq, lt)` counts worlds where the better action beat, tied, and lost to the worse one;
`lt` is structurally 0 in any verified basin, since verification aborts with the witnessing
world instead. Class W is weak dominance with strictness somewhere; class T (tied everywhere)
is an interchangeability statement, not a refutation, and is never collapsed into one.

## 5. How to write one

Three vocabularies exist, all **registries** — closed lists of atoms with declared semantics,
not open-ended grammars.

**The walt-native registry** (`walt-skeleton/src/atoms.rs`, `enum Atom`) is genuine
transducer vocabulary: `HolderOf(tile)` (which hidden slot holds it, or shown),
`TeamOf(tile)`, `BeaterCounts(tile)` (per-slot counts of still-unshown tiles that beat it
when led). These update closed because possession is static until the tile appears: `init`
reads the world once, and each observed play only ever *removes* latent content. Alongside
rides the **public chassis** — viewer hand, current leader, current trick prefix — a pure
fold of the observation record carrying exactly what is needed to know the seat's own legal
set. Atom tiles must be in the hidden pool; facts about the viewer's own tiles are public and
belong to the chassis.

**The exp3A registry** (`enum Exp3aAtom`) is v0.4 §14.4's 22-observable vocabulary,
reimplemented from the preserved probe: a holder fact and a team fact per pool tile, plus ten
control shapes — `comp` (the tile sharing the valued tile's holder's hand), `comp-in-context`,
`comp-is-floor`, `comp-rank`, `focal-max`, `opp-max`, `focal-top`, `opp-beaters`, `bestkeep`,
`with-boss`. These are static root labelings, so descriptors over them are marked
`StaticPassenger`.

**The lesson vocabulary** (`walt-factory/src/lesson.rs`) unions the two, adds the decision
cells, and adds a small registry of *numeric* atoms (`beaters-total(tile)`, `opp-beaters`)
that order cells may bound.

Rules a new atom must respect:

1. **Kernel-generic definition.** The exp3A vocabulary was designed against one hand; walt
   re-derives its parameters at any kernel (the decisive tile is the viewer tile whose led
   context touches the most hidden-pool tiles, ties to the higher tile). That rule is
   *recorded* in the code, because it is a choice, not a theorem.
2. **Partial evaluation, never defaulting.** `Exp3aContext::try_eval` returns `Option`, and
   `None` exactly where the precondition fails — a holder or team fact about a tile no hidden
   slot holds, or any companion-family atom when the valued tile's holder does not hold
   exactly two tiles. The §14.4 vocabulary is native to capacity-2 kernels; elsewhere the
   companion is *undefined*, not zero, not false. An undefined atom satisfies no equality
   cell and no bound.
3. **No target leakage** — no atom may call the solver or read the response class (§3.3).
4. **Declared horizon and information access** for derived continuation atoms (§3.3, §12.7).
   Today this is prose in the doc comment, not a machine-readable field (see §7).
5. **One name per atom.** The lesson vocabulary wraps only the ten control shapes; holder and
   team coordinates come through the native variants, so no atom has two names.
6. **Registration gates numerics.** Only atoms listed in `NumericAtom` can carry order cells;
   adding one is a deliberate edit to that enum.
7. **Selectors before tiles.** A lesson names actions by `decisive`, `max-count`, or
   `min-count` before falling back to concrete `tile(x)`; an unresolved selector makes the
   decision inapplicable, never defaulted.

## 6. Worked examples

### 6.1 The exp3A four-atom descriptor: 90 worlds, 33 cells, 8 responses

From the preserved probe's own Part 1 output (`walt/probes/exp3a/v3_output_postfix.txt`):

```
  registered atom vocabulary (22):
    41_with_22, comp41, comp41_in_suit2, comp41_is_2-0, comp41_rank2, h(2-0), h(2-2), h(4-1), h(4-2), h(4-4), h(5-2), nbeat_opp, oppmax2, s3_top2, s3max2, t7w_bestkeep, team(2-0), team(2-2), team(4-1), team(4-2), team(4-4), team(5-2)
  targets: R8 = 8-class parametric root-Q signature; R3 = 3-class action correspondence

  -- exhaustive minimum-size search, target R8 --
    smallest sound descriptor size: 4 atom(s); 8 minimal solution(s)
      ['comp41', 'h(2-0)', 'h(4-2)', 's3max2']  (69 cells)
      ['comp41', 'h(2-0)', 's3max2', 'team(4-2)']  (53 cells)
      ['comp41', 'h(4-2)', 's3max2', 'team(2-0)']  (53 cells)
      ['comp41', 's3max2', 'team(2-0)', 'team(4-2)']  (33 cells)
```

The domain is one trick-six kernel with a 90-world fiber. Target R8 is the eight-class
parametric root-Q signature — each world's exact response class. An exhaustive size-ordered
search over all subsets of the 22-atom registry finds no sound descriptor at size one, two,
or three, and exactly eight at size four. The last, `{comp41, s3max2, team(2-0), team(4-2)}`,
cuts 90 worlds into 33 cells through which the eight-class answer factors exactly — the
`R* = R̄ ∘ D` of §12.1, the "90 to 33 to 8" of §12.3. The winning atoms are control-shaped:
companion, decisive-context partner strength, two team facts. Swap `team` for `holder` and
the count rises to 53 or 69 — still sound, less compressed, because holder is finer.

walt's own checker reproduces this independently: `check_soundness` on the ported descriptor
returns `(90, 33, 8)`, and `(90, 33, 3)` for the three-class action-correspondence target —
the *same* 33 cells serve both. The full size-≤4 search reproduces the probe's whole record
(minimal size 4, eight solutions, cells 69/53/53/33, both targets). Those equalities are
CI-asserted pins in `walt/walt-skeleton/tests/harness.rs`.

The caveat travels with the result: this descriptor is a **static passenger**, sound for a
root target and silent about dynamics. §12.4 records what the follow-up Experiment 4B did and
did not prove — genericized versions were sound on 30 of 72 fresh tasks against 21 of 72 for
a holder-only baseline (real partial signal), but removing the vocabulary ceiling collapsed
median held-out world compression to 1. That is one flat descriptor family failing for one
target, *not* a proof that Straight 42 has no compact exact representation.

### 6.2 A lesson with an introduced cell, and one with a relaxed bound

The §4 walkthrough is itself the first: 25 cells down to 2, and both survivors were
*introduced by cut refinement* rather than retained — the entire original description dropped
away, and the generalizer then spent 2 of its 4-introduction budget re-carving the class
around two witnesses.

Read honestly: the lesson generalized *in vocabulary* (from "hand 0, seat S1, declaration P3,
these ten exact beater counts" down to two one-sided bounds) but not *in reach* — it still
matches one decision of the 32 eligible on its carrier, and the 1,680 worlds are that single
decision's whole fiber. Vocabulary generality and basin size are different measurements, and
this run shows them coming apart.

The relaxation half is the `horizon>=4` block quoted in §4. **I found no single printed
lesson in this run carrying both a `BOUND HELD` relaxation and an `introduce` step in one
trace**; the two mechanisms are shown here from two different lessons in the same file. That
is a reporting fact about this run, not a claim about the machinery.

A cleaner survivor case from the same file, where the surviving cell reached five decisions:

```
  drop ply=2 -> SURVIVES; witness h5 S0 t5 p3 world S0={4-1 5-0 5-2} S1={1-1 4-2} S2={4-3 4-4} S3={5-1 5-5} values 4-1=-21 5-0=-11 5-2=-21
final implicant (1 cells): ply=2
surviving: [ply=2]
basin [at grade: worldwise at (C, minimax-omniscient); weighting-free]: decisions 5/11 worlds 651/2135 triple (0,651,0)
```

Everything except "the seat is third to play" dropped away; the triple `(0, 651, 0)` says the
named action tied the world optimum in all 651 matched worlds, the expected shape for a `win`.

### 6.3 Descriptors that failed

**The static passenger fails condition 1, structurally.** A frozen root evaluation gives the
same state to a world's root node and to that world's later nodes — but the legal set has
changed by then, so `A(x) ≠ A(y)` inside one class. `check_lumpability` returns
`LumpabilityFailure::LegalSets`, on *any* kernel with a future focal decision. This is
asserted in `walt/walt-skeleton/tests/harness.rs`
(`static_passenger_is_marked_and_fails_lumpability_on_legal_sets`), which also records the
static run for the same descriptor: `(90, 15, 8)`, not sound. Note this is a CI-asserted test
expectation, not a line in a results file — no dated file in `walt-factory/results/` prints a
`fail:legal-sets` verdict.

**The public chassis fails condition 2 everywhere it was checked**, from
`walt/walt-factory/results/falsification_2026-08-10_r2.txt`:

```
origin: §12.6 lumpability failure h0 t6 descriptor chassis — kernel witness at carrier nodes 325,327: action 0-0 increment -1 mass 1 != 0 (carrier rebuilds from the kernel)
verdict: not-lumpable: chassis fails §12.6 at every matching decision
grade: checker (§12.6 exhaustive lumpability, uniform-legal field, q_points valuation)
final implicant (0 cells): (empty)
carrier: lead-kernel trees (ply 0, horizon <= 2) — eligible 13 decisions / 647 worlds (domain context: 179 decisions / 924813 worlds, not a rate base)
basin [at grade: checker (§12.6 exhaustive lumpability, uniform-legal field, q_points valuation)]: decisions 13/13 worlds 647/647
```

(The `domain`, `initial implicant`, and 19-step `trace` lines, and the bookkeeping lines
after `final implicant`, are elided; nothing else is changed.) The witness is a condition-2
disagreement: two
carrier nodes in one chassis class where playing 0-0 for increment −1 has mass 1 at one node
and 0 at the other. Every cell of the 19-cell origin dropped, leaving the empty implicant —
the strongest form this verdict takes: *the public chassis alone is never lumpable*, on all
13 eligible lead-kernel trees of the domain. Quantifier discipline matters: the checker
verdict quantifies per matching *decision*, so atom cells would have to hold at every fiber
world to count, and the basin counts under exactly that quantifier.

**Vocabulary ceilings.** The pinned soundness table in
`walt/walt-skeleton/tests/synthesis_run.rs` records, per trick-six kernel and target, the
exhaustive minimum-size search over the walt-native registry:

```
"h0 fiber=90 q_points: UNSOUND at every size <= 4 (vocabulary ceiling)",
"h1 fiber=90 q_points: min-size=4 solutions=2 first=team(5-3)+holder(1-0)+holder(4-0)+holder(5-4) cells=42",
"h2 fiber=36 action: min-size=0 solutions=1 first={} cells=1",
"h11 fiber=36 q_points: UNSOUND at every size <= 4 (vocabulary ceiling)",
"h11 fiber=36 action: min-size=4 solutions=4 first=team(3-1)+team(6-6)+holder(5-5)+holder(6-3) cells=24",
```

Three readings. On hand 0 no subset of four or fewer native atoms is sound for the q_points
target — an honest ceiling report, §12.9's failure branch, and *not* a claim that no
descriptor exists. On hand 2 the action target is sound at size *zero*: every world agrees on
the action, so the empty descriptor factors it. Hand 11 shows purpose-relativity in one place
— unsound for q_points, sound at size four for the action target, same fiber.

The companion lumpability table shows both outcomes:

```
"h0 nodes=738 chassis: fail:kernel classes=110 merged=628 largest=360",
"h0 nodes=738 chassis+holder-all: LUMPABLE nontrivial classes=366 merged=372 largest=6",
```

Adding every holder fact to the chassis does produce a genuinely lumpable, genuinely
compressing descriptor on hand 0 — 738 carrier nodes into 366 classes. It also very nearly
reconstructs the world, which is exactly the tension §12.4 named.

## 7. What is not built

**§12.7 is not implemented.** There is no Scheme/Fix parser, no `Scheme` or `Fix` data type,
no role schema `Σ`, no output interface `O`, no equality-pattern quotient, no step compiler,
and no denotational-equality proof. `walt/walt-skeleton/src/lib.rs` says so itself:
"Scheme/Fix as a descriptor query language (§12.7) arrives later inside this crate's
vocabulary; it will import this physics, never the reverse." `walt/PLAN.md` and `walt/LOG.md`
name Scheme/Fix as the *intended* descriptor language for work still to come, not as an
available tool; `walt/CENSUS-RULINGS.md` and the [walt hub](walt.md) both record the
compact-description question as separate and open.

Against §12.7's six conditions:

| §12.7 condition | Built? |
|---|---|
| Output roles explicit | **No.** No role schema in code; atoms name concrete tiles and slots. |
| Rigid and fresh role semantics distinguished | **No** in the descriptor language. Declared transports exist in the equivariant census module for §12.6A, not as query semantics. |
| Exact support remains authoritative | **Yes.** The kernel and its exactly-counted fiber are the authority; descriptor state is a derived view. |
| Every derived continuation atom declares horizon and information access | **Partly.** Preconditions and horizons are documented in prose and enforced by partial evaluation; no machine-readable field on an atom. |
| Step compiler proved to preserve the answer relation | **No.** No step compiler exists, so nothing is proved about one. |
| Induced transition satisfies the selected theorem | **Yes, as an exhaustive finite check.** The two checkers decide §12.1 and §12.6 on a declared finite domain. |

Of §12.7's boxed three-part deliverable — descriptor semantics, exact update law,
response-preservation proof — walt has the first two as a type discipline and the third only
as an exhaustive machine check on one finite domain, by a single implementation, at
exploratory tier. Evidence, not proof.

**What exists instead** is three registries and one implicant language that is a single
conjunction with a graded, labelled verdict. In §3's terms that is roughly *one Scheme case
without a role schema and without an output interface*: no disjunction, so no Fix; no role
names, so no rigid transport, no equality patterns, and no way to say "some tile playing this
role" instead of "tile 2-0". That last gap is why the §6.2 lesson generalized in vocabulary
but not in reach — with concrete tile names in the cells, a lesson can only travel to
decisions whose pools still carry those exact tiles.

**Where the open work sits.** `walt/PLAN.md` names §12.7 compact descriptions as rung 3 of
the census track: "make classes sayable (descriptor semantics + update law + preservation
proof); needed for both the seat-level construction and analysis." The original target was
the 306 root classes of the trick-six retrograde census — "what IS a class, in words a player
could read" (`walt/LOG.md`, S5e). After the railyard results PLAN.md retargets it: "§12.7
descriptions now naturally target the PARTS catalog (small) rather than raw classes," since
the suffix-library measurement suggested classes are menus over a compact shared catalog of
parts. Both targets are open. The related v0.4 §17.4 questions are number 3 (find a compact
Scheme/Fix transducer satisfying an exact predictive or lumpability theorem on nontrivial
kernels), number 5 (which Scheme fragments are closed under exact observation transport), and
number 6 (purpose-*exact* descriptions, whose cells equal rather than merely refine the
response classes).

One naming note for readers coming from the rest of the wiki: walt's §16.11 experimental
record type is called a *certificate* in its own namespace (the files under
`walt/walt-factory/results/certificates_2026-08-10/`). That is walt's own name for a
replayable experiment record with per-record coverage labels; it is unrelated to the D3
necessary-outer-profile concept. That concept is called a **necessary outer profile**
throughout this wiki and in rob, where greps enforce it — the name is deliberate, since
one of the two ingest packages does call the object an "outer certificate" and D3 records
that naming as deprecated ([discrepancies](discrepancies.md), [reachability](reachability.md)).
