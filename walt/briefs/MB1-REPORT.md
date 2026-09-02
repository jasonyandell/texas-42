# MB1-REPORT — the model-belief recursion joins the solver, as built

**Slice:** MB1, assignment `walt/briefs/BRIEF-MB1.md`. **Branch:**
`worktree-walt-s2` in the `walt-bundling` worktree, base `eaed5b1`.
**Theory:** `walt/math/model_belief_base_player_v0.1.md` §§16–23 and
§§29–33, under the intake companion's corrections and rulings MB-A1..A8
(`walt/CENSUS-RULINGS.md`); U0's field-specificity flag under SC-A7
(`walt/briefs/U0-REPORT.md`). **Tier:** EXPLORATORY throughout — every
number below is a measurement on a declared corpus and none of it is
theorem language, with the one exception noted and proved as such (the
§19 corollary, which is mathematics and is labelled as mathematics).

This document is the slice's report of record.

## THE NUMBER OF THE SLICE

**The model-fusion price is strictly positive at trick 4.** MB0 censused
`Φ = 0` at every one of fourteen root-action coordinates on its
trick-5/trick-6 corpus and reported its criterion 4 — "the point-mass
upper is sometimes strict" — as an honest NO. One stratum earlier, under
the same registered F₀/F₁ mixture and the same ν = (1/2,1/2) per hidden
seat, **every substantive coordinate tested is strict**:

| root | fiber | action | Q_a | U^sep_a | Φ_a | ‰ |
|---|---:|---|---|---|---|---:|
| h8-t4 | 1,200 | 2-1 | 8851/9600 | 8898/9600 | 47/9600 | 4 |
| h8-t4 | 1,200 | 3-1 | 8323/9600 | 8361/9600 | **38/9600** | 3 |
| h8-t4 | 1,200 | 3-3 | 9022/9600 | 9112/9600 | 90/9600 | 9 |
| h8-t4 | 1,200 | 5-5 | 8871/9600 | 8929/9600 | 58/9600 | 6 |
| h3-t4 | 11,550 | 3-1 | 563/1848 | 1781/5775 | 173/46200 | 3 |
| h3-t4 | 11,550 | 4-1 | 7121/30800 | 269/1155 | 157/92400 | 1 |
| h3-t4 | 11,550 | 4-4 | 4471/15400 | 1709/5775 | 37/6600 | 5 |
| h3-t4 | 11,550 | 6-4 | 284/1155 | 11507/46200 | 7/2200 | 3 |

The bolded row is gate M6's pinned specimen, re-derived by the gate
rather than cited from the probe. Eight substantive trick-4 coordinates,
eight strict prices, none vacuous.

Against that, **h12-t4's four coordinates are `Φ = 0` and are typed
VACUOUS**, because the whole fiber is already decided at the root: there
`U^sep` sits at an endpoint, every lawful policy attains every
point-mass optimum for an arithmetic reason, and the zero says nothing
about the price of model blindness. That is U0's degenerate-God-tightness
discipline carried across unchanged, and it earns its keep immediately —
without it, trick 4 would read as "8 strict, 4 zero" instead of
"8 strict, 0 substantive zeros".

The same lens turned backwards on MB0's own corpus is worth recording,
because it changes that headline too. Of MB0's fourteen zeros, **seven
are substantive and seven are vacuous**: h5-t6 and h4-t6 contribute two
each and h8-t5 three, while h12-t6 (`Q = 0`), h10-t6 and h3-t5 (`Q = 1`)
are all endpoint cases where nothing was ever at stake. MB0's "14 zero /
0 strict" is really "7 substantive zeros". The probe's Part 4 table
carries the full breakdown.

## The corollary that makes the finding load-bearing

Theorem 19.1 says `Φ_a(ν) = 0` iff one lawful policy attains `q_a(θ)`
for every θ in the support of ν. Take ν with FULL support and
`Φ_a(ν) = 0`. The attaining policy ρ* then satisfies `V_θ(ρ*) = q_a(θ)`
at every θ whatsoever, so for any other belief ν′,

```
Q_a(ν′) ≥ ⟨ν′, v_{ρ*}⟩ = Σ ν′(θ) q_a(θ) = U^sep_a(ν′),
```

and Theorem 18.1 gives the reverse inequality. Hence `Φ_a(ν′) = 0` for
every ν′ over the same types. ∎

This is mathematics, not a measurement, and it is what turns MB0's
result from "zero at the belief we happened to test" into **zero at
every belief over those types**. It also settles where a strict specimen
could have come from: not from re-weighting. MB1 carries the witness
explicitly — `CommonOptimizer`, present on exactly the zero-price
coordinates — and gate M6 sweeps a rational grid of ν to confirm that
the single witness policy attains `U^sep` at every point. So the search
that found the trick-4 specimen had to move roots, and did.

## Status

Complete. Nine WIP commits plus this report, clean tree, not pushed. One
of those commits (`f460fb0`) was made by the night-watch orchestrator as
a rescue checkpoint of the probe output while this session was blocked
on the probe run; the file is unmodified.
`walt/ci/check.sh` PASS (cold rebuild, `fmt --check`, clippy
`-D warnings -D clippy::float_arithmetic`, the no-float greps and
scanners, the full workspace release suite, and the Lean trick-1
foundations with the exact axiom audit).

| file | change |
|---|---|
| `walt/walt/src/solver/model_recursion.rs` | new |
| `walt/walt/tests/solver_model_belief_recursion.rs` | new |
| `walt/walt/src/bin/modelbeliefrecursionreport.rs` | new |
| `walt/probes/factor_belief/modelbelief_recursion_run1.txt` | new |
| `walt/walt/src/solver/model_belief.rs` | extended (MB0's module, items 3 and 4) |
| `walt/walt/src/solver/mod.rs` | +1 (module registration) |
| `walt/FACTOR-BELIEF.md` | status line + MB1 paragraph |

Untouched, verified by `git diff eaed5b1 HEAD --stat`: `solver/doom.rs`
(§47), `solver/godgap.rs` (U0's census), `solver/refine.rs` (freeze 58),
`probes/factor_belief/doomreport_run1.txt`,
`probes/factor_belief/godgap_run1.txt`, `tests/solver_doom.rs`,
`tests/solver_godgap.rs`, `ingest/`.

## What was built, item by item

### Item 1 — the posterior-carrying recursion, and the values as facts

MB0 already conditioned per profile inside its bundle walk; what it did
not have was a way to descend a line as an object, or a way for the
resulting values to leave the module. Both now exist.

`PosteriorTrace` / `trace_heaviest_line` descend a public line, taking
the heaviest merged public branch at each hidden state and the focal
policy's own choice at each focal state, and record the posterior after
every observation. Every recorded field is a DERIVED VIEW of the single
carried `ModelBelief` — `seat_type_marginals`, `posterior_profile_masses`,
`branch_masses`, `typed_branch_census` — so there is no second authority
to drift from, and the trace stores no belief of its own. The hidden
step goes through `ModelBelief::observe_with_survivors`, added to MB0's
module because a recursion that scatters per-profile masses back to
their parents needs the surviving index alignment that `observe` was
throwing away.

`ModelBeliefProducer` installs the census into the §49 store through the
open producer registry: per priced coordinate, `Q_a` as an executable
lower witnessed by the extracted mixture-argmax policy, `Q_a` again as a
deterministic upper (the mixture response IS the maximum over lawful
policies, so the two collapse the action's interval to a point), and
`U^sep_a` as a separately-authored Theorem 18.1 upper so the Φ record
survives in the store even where it is zero. A refused coordinate
proposes nothing at all — a refusal is not a bound, and gate M4 asserts
that a wholly refused census returns an empty proposal list rather than
a list of zeros.

### Item 2 — repricing instead of re-walking

`MixtureOutcome::reprice(weights)` is §16/§23 made two dot products: a
stored response vector's value under any other prior is
`(Σ w'·M_θ, Σ w'·Z_θ)`, exact, no walk, no division. `ResponseEnvelope`
is §21's column-and-cut lower `L^R(ν) = max_ρ ⟨ν, v_ρ⟩` as a reusable
object; the comparison is over MASSES rather than values, which is exact
and division-free because at one state every column shares the same
`Z_θ` (a focal choice changes no factor), so the argmax of the values is
the argmax of `Σ w·M_θ`.

What the sweep measured, on both h5-t6 and h8-t5: **one facet across
seven grid points.** A single argmax policy is optimal along the whole ν
line from near-δ_{F₀} to near-δ_{F₁}, so six of the seven beliefs are
answered by dot product alone. At h8-t5 the VALUE moves while the policy
does not — 770‰, 769‰, 767‰, 766‰, 764‰, 763‰, 762‰ across the line —
which is the case worth having: repricing is doing real work (seven
different exact values) at the cost of one walk. This is also what the
§19 corollary predicts wherever `Φ = 0` at full support, since the
common optimizer is then optimal at every ν; the interesting sweeps will
be at the strict coordinates, where more than one facet must exist.

`sweep_envelope` is the AUDITED sweep and walks at every grid point on
purpose. Before each walk it records what the library would have
answered; afterwards it asserts §21 (the library never exceeds the exact
response) and marks the point a FACET exactly when the library was
strictly below. The facet count it reports is therefore the number of
walks a cheap sweep would have needed — measured, not claimed — while
every value it returns is exact.

### Item 3 — tightening is the default on every classifying entry point

MB0 tightened the acting seat's factor to its positive joint support
inside `observe` and inside the bundle walk, but not in `branch_masses`
or `typed_branch_census`, which classify the acting seat's RAW support
and so could still reach the σ1 sampler's empty acceptance region. Both
now tighten first.

This is exactness-neutral by the zero-entry law and gate M5 proves it on
a specimen where tightening actually drops entries: every dropped hand
is shown to have exactly zero completion weight by an independent
`marginal` call, and the tightened branch table is shown to conserve the
augmented mass exactly. What does move is `typed_branch_census`'s
typed-row count, which is why that census is now DECLARED to count
positive-support rows — the representation every walk actually
classifies — rather than raw ones.

### Item 4 — budgeted reads, refused typed

`ReadLedger` records every `ProfileField` dispatch, keyed by behavior
type, in a ledger shared by a whole `ModelBelief` lineage: the same
object survives `focal_play`, `observe`, `with_seat_table` and the
per-profile walks inside `separated_upper`. It is APPEND-ONLY with no
reset and no decrement, so a reported count is always work actually
spent; callers take a baseline and read the difference. The count is
taken at the dispatch itself, so it measures the mind's real
consultations rather than the walk's estimate of them.

`MixtureRefusal::ReadBudget` carries the MEASURED spend, the declared
ceiling, and the public history the walk stopped at. It has no value
field, and no other variant exists, so a truncated number cannot be
reported even by accident. The ceiling is checked at the boundary of
every walked bundle node BEFORE the node spends anything; MB0's three
entry points are the same walk under an ABSENT ceiling, where
`WalkBudget::check` is total and the `Err` arm is unreachable — which is
what lets them keep returning a bare value with no behavioural change.

WHERE THE CEILING IS CHECKED, precisely, because the probe makes it
visible: at the boundary of every walked bundle node, before that node
is expanded. A node that passes the check then classifies the acting
seat's support for every live profile, which is itself many
consultations, so the ledger can pass the ceiling by up to ONE NODE'S
classification cost before the next check sees it. Two of h8-t3's five
refusals report 7,000,011 and 7,000,028 against a ceiling of 7,000,000
for exactly that reason. The ceiling is a budget, not a hard bound —
and the number reported is always the ledger's measured total, never the
ceiling and never a value rounded to it, which is the property that
actually matters.

Gate M4 pins four things: a zero ceiling refuses having spent zero (the
check is genuinely before the spend); a starved ceiling refuses with a
spend equal to the ledger's own measurement; an ample ceiling returns
exactly the uncapped value, so refusal is a function of the declared
budget and of nothing else (U0's G4 shape); and a starved `U^sep`
ceiling under an ample response ceiling yields a coordinate with `Q`
priced and NO `Φ` reported, because an upper that did not finish is not
an upper.

### Item 5 — the earlier roots

The finding is at the top of this report. What the strictly pre-t4
coordinate returned: **h8-t3 (fiber 59,976) refuses all five root
actions**, each on the response side, each having spent the declared
7,000,000-read ceiling — 35,000,039 field consultations and 1,889 s for
five typed refusals and no value. Every refusal names the public history
it stopped at, so the coordinate is a measured budget decision at a
named place rather than an absence. The answer to the question the
ceiling was chosen to ask is therefore a clean NO: a trick-3 coordinate
does not close within the most a trick-4 coordinate cost, and it is not
close — the walk was still descending at fourteen plies deep on one of
them.

The declared budgets:
12,000,000 field reads per trick-4 coordinate (comfortably above the
largest trick-4 coordinate measured while choosing it — 6,901,094 reads
at h3-t4 action 4-4), and 7,000,000 per strictly-pre-t4 coordinate,
which is the largest spend any trick-4 coordinate actually needed,
rounded up. That makes the trick-3 question a precise one — *does a
trick-3 coordinate close within the most a trick-4 coordinate cost?* —
rather than *will it close at any price*.

### Item 6 — non-product priors

No correlation machinery was built, and the interface that would carry
it was not broken. Gate M2 part three runs a maximally correlated prior
(weights 5,1,1,1,1,1,1,5 — shown non-product by the product identity
`w(000)·w(110) = w(100)·w(010)` that it fails) through
`from_profile_prior`: it prices exactly, Theorem 18.1 holds under it,
and — the part worth having — its response vector reprices to the
INDEPENDENT prior's own walked value, because a response vector is a
property of the policy and the state and knows nothing about which prior
produced it.

### Item 7 — the field-identity fence

U0 flagged that God-tightness and every doom-derived bound is
field-SPECIFIC under SC-A7's strictest class: each is an equality
against a doom upper computed under ONE declared σ0. A model belief is
not that field. Its profile fields are `profile:<type ids>` objects, and
the mixture over them is a third thing again.

The fence is structural where it can be. `CoupledFact` has private
members and no public constructor, so the only way to obtain one is
`couple_fixed_field_fact`, and a model-space consumer that takes a
`CoupledFact` cannot be handed a bare `Fact` at all. `FieldCoupling` has
exactly two variants — `Identical` and `PointMassParity` — and no
`Assumed`; adding one would be a mathematical claim, not a code change.
`CouplingRefusal` names five reasons.

Where structure cannot reach — the §49 store accepts any well-typed
`Fact` under a matching identity — the fence IS the identity:
`mixture_identity` gives a model-belief proof state a `field_id` of
`model-mixture:<content address>`, so a σ0-authored fact is rejected
`IdentityMismatch` by machinery that already existed. A REWEIGHTED
mixture is a different identity again, because the values established
under it are different values.

The one coupling this slice can discharge is the degenerate one, and it
is not free. A point-mass δ_θ is extensionally the fixed field its type
dispatches to, but "extensionally" is a claim about values and not an
identity of objects, so `PointMassParity` carries a re-run WITNESS with
both authorities' exact pairs recorded in it. Gate M7 runs all five
outcomes: refused into the mixture (naming its 8 live profiles), refused
into a point mass over the wrong parent field, refused with no witness,
refused with a DISAGREEING witness (reporting both sides), and coupled
only where a genuinely re-run `response_success_mass` under the raw σ0
`FieldModel` agrees with the model side.

**MB1 transports nothing.** The gate exists anyway, which is the point:
it is what keeps the first future transport honest, and building it
after the first transport would be building it too late.

## The gates (`walt/walt/tests/solver_model_belief_recursion.rs`)

**M1** — the posterior-carrying recursion against an INDEPENDENT (ω,θ)
pair enumeration written in the gate file, on the FULL six-root MB0
corpus: the augmented mass, all fourteen root actions' exact `Q_a`,
every per-profile point-mass optimum `q_a(θ)` behind every `U^sep_a`,
`Φ_a = U^sep_a − Q_a` exactly, the selected action, and the CARRIED
posterior — the merged branch table at the first hidden state of the
heaviest line against the enumeration's own partition, and each per-seat
type marginal against the surviving pairs' weighted count. Closes on
6 roots and 14 coordinates.

One honest wrinkle the gate had to be taught: at h12-t6 and h10-t6 the
decided cutoff settles the root before any focal node is reached, so the
extracted policy records no root choice. Asserting an argmax action
there would have been asserting a fabricated one; the gate asserts what
is actually true instead — that a root-decided fiber values every action
alike.

**M2** — §16/§23 repricing and the §21 envelope, on h5-t6, h4-t6, h8-t5:
a stored response vector reprices every point of a swept rational grid
to the value a full walk under a belief actually built with those
weights produces; the envelope reproduces the exact response at every
swept point; and the facet count is strictly below the grid size
somewhere on the corpus, which is the saving. Plus item 6's non-product
prior.

**M3** — point-mass collapse INSIDE the recursion, on h5-t6, h4-t6,
h8-t5, both endpoints. A δ belief is descended two hidden observations
and priced at that depth; the value is checked against the (ω,θ)
enumeration restricted to that type and to the worlds consistent with
the observed history, and — for F₀ — additionally against
`response_success_mass` on an INDEPENDENTLY conditioned `FactorBelief`
walked down the same public history under the raw σ0 `FieldModel`,
never through the profile field. A different object and a different
recursion. The F₁ half is enumeration-anchored, which is exactly MB0's
declared scope (the raw σ1 authority's refusal set, G2) inherited
unchanged rather than quietly widened.

**M4** — the budget gates described under item 4.

**M5** — the consumed instruments unperturbed: item 3's tightening
exactness-neutral on a specimen where it actually drops entries, with
every dropped entry's zero completion weight independently confirmed;
and `doom_enumeration` bit-identical either side of a model census
(§47/SC-A3, asserted from the MB1 side, the mirror of U0's G5).

**M6** — the earlier-root finding, pinned. Described at the top.

**M7** — the field-identity fence, both halves. Described under item 7.

**Runtime:** the MB1 suite is 20.8 s wall (the seven gates run in parallel), of which M6 alone is 20.5 s. That
is the honest cost of re-deriving a trick-4 coordinate rather than
citing it: about a million field consultations for the one coordinate
the gate pins. The other three coordinates of the same root, and all
four of h3-t4, are in the probe.

## The probe

`modelbeliefrecursionreport report` →
`walt/probes/factor_belief/modelbelief_recursion_run1.txt`. Five parts:
MB0's corpus under the posterior-carrying recursion (with the posterior
line and the MB0 root-value parity check per root), the ν sweeps, the
earlier roots, the Φ table, and the field-identity fence census.

A `measure <hand> <trick> <cap>` mode exists beside it and is the reason
the declared budgets are numbers rather than guesses.

Run 1 wall: **2,412 s total** (40 minutes, single-threaded driver with
the σ0/σ1 field caches shared within the run). Where it went, and the
distribution is the story: 31.5 minutes to h8-t3's five refusals, 7.0
minutes to h3-t4, 1.6 minutes to h8-t4, and everything else — the whole
six-root MB0 corpus, both ν sweeps, h12-t4 and the fence census —
under 5 seconds combined. **78% of the run bought five typed refusals.**
That is the same shape U0 reported (86% of its wall went to coordinates
returning `UnknownGodGap`) and it is not waste: the refusal is the
measurement.

Per root, measured: h12-t6 1.0 ms, h10-t6 1.1 ms, h5-t6 42.8 ms, h4-t6
93.5 ms, h8-t5 0.99 s, h3-t5 3.67 s, h8-t4 97.7 s, h12-t4 0.14 ms,
h3-t4 420.0 s, h8-t3 1,889.4 s. Field reads, measured: 416 / 512 /
10,592 / 29,920 / 93,340 / 313,265 at MB0's roots; 4,841,712 at h8-t4;
0 at h12-t4; 24,163,138 at h3-t4; 35,000,039 at h8-t3. The two
registered types draw almost exactly half the reads each everywhere
except h8-t3, where the ratio tips to 21.4M σ0 against 13.6M σ1 —
because the σ1 half of the bundle is refused into more often at that
depth, not because it is cheaper.

## Deviations from the brief, recorded

1. **"Pre-t4" was read as "earlier than MB0's corpus", with a strictly
   pre-t4 coordinate attempted and reported separately.** The brief asks
   for "at least two pre-t4 receipt-root coordinates (affordability
   permitting)". Taken literally that means trick ≤ 3, where the
   smallest receipt fiber is 59,976 against trick 4's 1,200 — and
   trick 4 already costs 97 s (h8-t4) to 416 s (h3-t4) per root. The
   slice therefore delivers eight fully priced coordinates at trick 4,
   which is the stratum U0 measured a positive God gap at and which MB0
   never entered, AND runs h8-t3 under a declared ceiling so that the
   strictly-pre-t4 answer is a typed refusal at a named coordinate
   rather than an absence. Both readings are covered; only the first is
   affordable.

2. **Items 3 and 4 landed inside MB0's module, not beside it.** The read
   ledger has to be recorded at the `ProfileField` dispatch and the
   budget has to be threaded through `mixture_walk`, both of which live
   in `solver/model_belief.rs`. Putting them in a sibling module would
   have meant a second walk — a fork in all but name. MB0's public API
   is unchanged and its eight gates are green untouched.

3. **M5 is a property gate, not a suite conjunction.** A test cannot run
   other test files. M5 asserts the specific invariants MB1's changes
   could have broken (tightening's exactness-neutrality, doom's
   purity); the actual conjunction of MB0's eight, σ1-repair's seven and
   U0's six is `check.sh`, and it is green.

4. **The ν grid excludes its endpoints.** A per-seat weight of zero is a
   belief with strictly smaller SUPPORT — a point mass, not a
   re-weighting — and its model belief is a different object with a
   response vector of a different length. The first version of the gate
   clamped zeros to one and compared two different beliefs; the clamp is
   gone and the endpoints are excluded by construction, with the δ
   endpoints reached by constructing them as point masses in M3 where
   they belong.

## Flags for MB2 and beyond

### The strict prices are thin, and that is informative

3–9‰ at h8-t4 against `U^sep` of 870–949‰; 1–5‰ at h3-t4 against a much
lower base. These sit in the same band as U0's twelve trick-4
information prices (6–22‰) and are commensurable with them without
being the same quantity: U0 prices not knowing the WORLD, MB1 prices not
knowing the MIND. A separation upper that cannot get inside a
single-digit permille band adds nothing at these coordinates — the same
warning U0 gave MB2, now with the model-space number beside the
world-space one.

### Where the next strict specimen should be looked for

Not at other ν (the §19 corollary forbids it), and not deeper (trick 5
and 6 are censused zero and now censused zero at every belief). Earlier,
or with more types, or with types that disagree more. The registered
F₀/F₁ pair is a two-rung ladder of the same architecture; the §33
disagreement frontier says model-belief compute is wasted before a
reachable field disagreement, and a type library chosen to disagree
EARLIER should raise the price at a fixed depth. That is a cheap
experiment against machinery that now exists.

### What the unified player would need from this module

The API shape is already the one a player wants: construct a
`ModelBelief` at a root, `focal_play` / `observe` down the line as play
proceeds (the posterior is carried, never recomputed), and call
`mixture_response_budgeted` with a ceiling chosen from the ledger. The
budget knobs are two — `response_cap` and `separated_cap`, both in field
consultations — and the refusal is typed all the way out, so a player
can fall back on a named refusal instead of a panic or a silent
truncation.

What it is NOT ready for is the clock. A trick-4 coordinate costs one to
seven million field consultations and one to seven minutes; a whole
trick-4 root is one to seven minutes times its action count. The
response-vector envelope is the one lever that is already built: a
policy library priced once reprices under any belief for free, so a
player that can afford the walk ONCE can afford to re-decide under a
moving posterior. Whether that is the right use of the lever is a
sequencing question, not something this slice establishes.

### For the census program

The vacuity discipline transferred from U0 without modification and
caught a real dilution on its first use (h12-t4). Any future
Φ-censusing slice should inherit it in the same form: a zero at an
endpoint is not evidence.

---

EXPLORATORY — below every evidentiary tier; quotable only via gate
receipts. The trick-4 fusion prices are measurements on a declared
corpus, never theorems. The §19 ν-invariance corollary IS a theorem and
is proved above; it is a consequence of Theorem 19.1 and carries that
theorem's status, not this slice's.
