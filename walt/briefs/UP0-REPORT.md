# UP0-REPORT — the unified walt player, slice 0, as built

**Slice:** UP0, `walt/briefs/BRIEF-UP0.md`, authorized 2026-09-02 under
Jason's §76 GO. **Status: COMPLETE.** New module `solver/unified.rs`
(1,609 lines), new probe `bin/unifiedreport.rs`, 18 gate assertions in
`walt/walt/tests/solver_unified.rs` (17.0 s, release), committed
transcript `walt/probes/factor_belief/unified_run1.txt` (216 decisions,
31.5 s). `walt/ci/check.sh` **PASS** cold, 6 min 1 s, on the final tree.

**EXPLORATORY tier throughout.** Nothing in this slice is a play-strength
claim, and nothing in it compares the unified player to the existing one.
The probe is a TRANSCRIPT of decisions and their provenance. Arena work,
default changes and bridging remain Jason's word.

---

## THE SHAPE OF THE SLICE

There is one decision function. `UnifiedPlayer::decide(state, budget)`
answers at every legal state of a hand and returns the action together
with a `Provenance` naming the instrument that produced it, the field
consultations it spent, and every typed refusal it fell through. It never
panics on an instrument result and never truncates silently: the cascade
is total and ends at a σ0 consultation that always answers.

The five tiers, deepest certainty first, each entered only on declared
affordability and exited only by a typed refusal:

```
(a) DecidedArithmetic   decided_success settles the pmake indicator for
                        every continuation, or one legal tile exists
(b) EndgameExact        the store already proves this root (CONSUME a
                        zero-regret necessary outer profile), else the
                        exact world-space recursion where the fiber
                        affords enumeration
(c) MiddlegameMixture   MB1's exact model-space response under the carried
                        posterior and a declared read ceiling
(d) CertifiedRegret     the §33 recommendation off the store's facts,
                        within a declared Γ
(e) FieldFallback       the declared σ0 field's own choice — total
```

Every decision knows which of the two recursions it stands in.
`Recursion::direction()` answers in one word — `backward` for the
enumerable exactness of tiers (a), (b), (c) and (d), `forward` for the
field's own play at (e) — and `Recursion::space()` separates the two
backward recursions, `world` over the fiber `Φ(C)` and `model` over
`Ξ = Ω × Θ`. That is Jason's frame made a field of the record rather than
a remark about it.

---

## THE NUMBERS OF THE SLICE

### 1. The carry is the wall, and on a lean rung nothing reads it

The single most useful measurement UP0 produced was not about deciding.
Splitting the transcript's wall into time inside `decide` and time inside
`observe_play` — the price of ADVANCING the carried posterior past a play
— gives, per rung, over 72 decisions each:

| rung | deciding | carrying | mean decision | worst decision |
|------|---------:|---------:|--------------:|---------------:|
| lean  |    12,117 µs | 2,105,672 µs |     168 µs |     4,203 µs |
| ample | 20,001,864 µs |   904,569 µs | 277,803 µs | 11,269,958 µs |
| model |  6,712,759 µs | 1,616,137 µs |  93,232 µs |  3,500,838 µs |

On the lean rung **99.4% of the wall is carrying a posterior that no tier
consulted.** Advancing a model belief past a hidden play means classifying
the acting seat's support under every live profile, at every ply, for
every open line — and the lean rung's budget affords no tier that reads
the result. The join is not free and it is not free in the place one would
expect: the expensive part of carrying a belief is carrying it, not
reading it.

The lever is declared rather than hidden. An empty `TypeLibrary` opens no
line, and gate UP4 pins that it changes no action, no tier and no exact
value the world-space tiers produced. A lean unified player that declares
no library therefore pays 12 ms for those 72 decisions instead of 2.1 s.

### 2. Tier occupancy — where each recursion actually answered

216 decisions, seven receipt roots (MB0's six enumerable roots plus
h8-t4), three declared budget rungs:

```
 rung  | trick | decisions | (a) dec | (b) exact | (c) mix | (d) regret | (e) field
-------+-------+-----------+---------+-----------+---------+------------+----------
 lean  |  t4   |       4   |      2  |       0   |      0  |       0    |      2
 lean  |  t5   |      12   |      2  |       5   |      0  |       0    |      5
 lean  |  t6   |      28   |     11  |      15   |      0  |       0    |      2
 lean  |  t7   |      28   |     28  |       0   |      0  |       0    |      0
 ample |  t4   |       4   |      2  |       2   |      0  |       0    |      0
 ample |  t5   |      12   |      2  |      10   |      0  |       0    |      0
 ample |  t6   |      28   |     10  |      18   |      0  |       0    |      0
 ample |  t7   |      28   |     28  |       0   |      0  |       0    |      0
 model |  t4   |       4   |      2  |       0   |      0  |       0    |      2
 model |  t5   |      12   |      2  |       2   |      8  |       0    |      0
 model |  t6   |      28   |      9  |      11   |      8  |       0    |      0
 model |  t7   |      28   |     28  |       0   |      0  |       0    |      0
```

Three readings, in order of how much they matter.

**Trick 7 is entirely free.** Every one of the 28 last-trick decisions on
every rung is tier (a): by the seventh trick the pmake indicator is
settled for every continuation, or the play is forced. The unified player
spends nothing there and says so.

**Tier (c) never fires under a natural ladder, and the reason is
structural.** The declared cascade puts the world-space recursion above the
model-space one, so tier (c) can only answer where tier (b) has ALREADY
refused. Under lean and ample the enumeration cap sits above the mixture
cap, so that band is empty and the model-space recursion appears only as a
join reading, never as an answer. The third rung exists for exactly this
reason: it swaps the two structural caps, opening a band where (b) refuses
and (c) answers, and it is what makes gate UP6's tier-(c) verification
possible at all. A transcript in which a tier never fires has not
exercised it, and that is worth saying rather than shipping.

**Tier (d) never fired anywhere.** The §33 recommendation tier sits below
both exact tiers, so it is only reachable where both refused AND the store
already holds facts for the root — and on this corpus the store's facts
come from tier (b), which only deposits where it also answered. Tier (d)
is built, gated for its refusals, and structurally unreachable on this
corpus. Its domain is a store seeded by producers that are not (b): U1's
salvation masks, the doom census, the unbuilt §33 producers.

### 3. The join: 27 readings, 9 value moves, 2 argmax flips

Where the budget declares a join reading, tier (b) prices the model side
too and records both answers side by side. MB1 measured that values move
before argmaxes do; on a played line, both happen, and the corpus
separates them cleanly.

Nine readings moved the value. SEVEN of them moved the value and left the
argmax alone; the other two are the flips below, which moved both — the
probe's own "9 moved the value, 2 flipped the argmax" counts the flips in
both totals, and this report earlier miscounted the first group as nine.
The seven:

```
h8-t5 t5 p0 s3: Q(ν)=7/8   (875‰) vs fixed-field 91/92 (989‰)   argmax 5-3
h3-t5 t5 p1 s1: Q(ν)=29/40 (725‰) vs fixed-field 4/5   (800‰)   argmax 6-1  [GATED]
h3-t5 t5 p2 s2: Q(ν)=5/12  (416‰) vs fixed-field 32/75 (426‰)   argmax 4-1
h3-t5 t5 p3 s3: Q(ν)=1/12  ( 83‰) vs fixed-field 1/9   (111‰)   argmax 6-3
h8-t4 t5 p1 s2: Q(ν)=1/15  ( 66‰) vs fixed-field 2/15  (133‰)   argmax 4-0
h8-t4 t5 p2 s3: Q(ν)=31/54 (574‰) vs fixed-field 2/3   (666‰)   argmax 5-0
h8-t4 t6 p0 s3: Q(ν)=7/9   (777‰) vs fixed-field 6/7   (857‰)   argmax 0-0
```

**Tier marker, explicitly, because these are exact rationals:** exactly
one row of that table is carried by a gate — the `[GATED]` one, pinned in
`up3_the_value_move_specimen_is_pinned`. The other six are
TRANSCRIPT-OBSERVED and reproducible from
`probes/factor_belief/unified_run1.txt` under its declared header, and no
receipt carries them. They are listed to show the SHAPE of the finding —
that value movement is common and modest across the corpus — and the
shape is what the paragraph claims, not the individual numbers. Both
flips, by contrast, are gated, because the flips are the number-class
finding of the slice and are quoted as such in the ledger.

Two states where it flipped the ARGMAX (both gated):

```
h3-t5 t6 p2 s2: model 6-4 vs fixed-field 3-1 | Q(ν)=3/5     (600‰) vs 5/6    (833‰)
h8-t4 t4 p2 s3: model 0-0 vs fixed-field 6-2 | Q(ν)=617/864 (714‰) vs 173/216 (800‰)
```

Both flips are pinned exactly in gate UP3
(`up3_both_argmax_flip_specimens_are_pinned`), together with the live
profile count at each — a flip under an untouched prior (h8-t4, all eight
profiles live) and a flip under a posterior that has already zeroed two
profiles by observation (h3-t5, six live) are different evidence, and the
second is the stronger specimen. The h3-t5 flip was originally
transcript-only and its four rationals were quoted here as though a gate
carried them; the independent audit caught the overclaim and the gate now
carries both. See the note under Deviations, item 7.

Note what the numbers are and
are not: `Q(ν)` and the fixed-field optimum are values against DIFFERENT
opponent models — a mixture over `{F₀, F₁}` against a point mass at `F₀` —
so the lower number is not a worse answer, it is an answer to a harder
question. The mixture is uniformly the lower of the two everywhere on this
corpus, which is what a belief that admits a second, stronger opponent
type should do.

**The sequencing question this raises, and does not settle.** At the two
flip states the model-space recursion, under the belief the player
actually holds, prefers a different tile — and the declared cascade plays
tier (b)'s answer anyway. The cascade's order is the brief's, and UP0 keeps
it; but the ordering is a declared choice, not a theorem, and the
mathematics argues against reading it as one. MB0's point-mass parity gate
proves the world-space exact recursion is the `δ_{F₀}` SPECIAL CASE of the
model-space one. Tier (b) is therefore not "deeper certainty" than tier
(c); it is the same recursion under a strictly narrower belief. Whether a
unified player should invert the two wherever both are affordable is the
first question UP1 has to answer, and this slice hands over two exact
specimens on which to answer it. Gate UP3 records the choice in its own
assertion text so that a future inversion has to change a gate on purpose.

### 4. The declared type library is FALSIFIED by UP0's own play

A model belief's support is the set of actions its declared type library
would play. UP0 is not in that library, so when UP0 seats play each other
an observation can leave the support entirely — at which point
`ModelBelief::observe`'s "an observed action has positive augmented mass"
assertion would fire. UP0 checks the merged branch table FIRST and, on a
miss, RETIRES the line with a typed `Falsification` carrying the history,
the seat, what it played and what the library did support. The posterior is
never repaired, re-seeded or widened.

It happens, nine times across the three rungs, always at the two
deepest-fiber roots:

```
h3-t5 [lean]  t6 p2: seat 2 played 6-4, library supported {4-1}
h3-t5 [lean]  t7 p1: seat 1 played 6-5, library supported {4-1}
h3-t5 [ample] t5 p2: seat 2 played 4-1, library supported {3-1 6-4}
h3-t5 [ample] t6 p1: seat 1 played 6-5, library supported {3-1 3-2}
h8-t4 [ample] t5 p2: seat 3 played 5-0, library supported {2-0 5-3 6-0}
h3-t5 [model] t5 p2: seat 2 played 4-1, library supported {3-1 6-4}
h3-t5 [model] t6 p1: seat 1 played 6-5, library supported {3-1 3-2}
h3-t5 [model] t6 p2: seat 2 played 6-4, library supported {3-1 3-3}
h8-t4 [model] t6 p0: seat 1 played 3-1, library supported {0-0}
```

This is a finding, not a defect. A two-rung ladder of the same
architecture — σ0 and a σ1 built on it — does not contain the play of an
exact solver, and a belief over it is falsified the moment a seat plays
something neither rung would. The support sets are TIGHT (one to three
tiles out of a legal set that is usually larger), which is why falsification
arrives so quickly. Whatever library a future unified player carries has to
either contain the player itself or accept being falsified on its own
lines; MB1 already flagged "a type library chosen to disagree EARLIER" as
the cheap next experiment, and this is a second reason to run it.

### 5. The refusal census

Across 216 decisions, every refusal typed, none swallowed:

```
   63  ProofStateUnavailable      the §49 store is a TRICK-START object
   27  EnumerationUnaffordable    fiber above the declared cap
   12  MixtureUnaffordable        fiber above the declared mixture cap
    2  PosteriorFalsified         the library was retired earlier on this line
```

The dominant refusal is a scope boundary UP0 inherited and did not choose,
and it is the clearest single item of UP1 work — see Deviations, item 2.

---

## WHAT WAS BUILT, ITEM BY ITEM

### Item 1 — the decision core

`solver/unified.rs`. `UnifiedPlayer::decide(&mut self, state: &DrivenState,
budget: &MoveBudget) -> Decision`. `Decision` carries the action and a
`Provenance`; `Provenance` carries `Evidence`, the authority by name, a
`Spend`, the ordered `Vec<TierRefusal>`, a `PosteriorNote` and a
`DecisionFrame` (root identity, seat, trick, ply, fiber mass, legal action
count, budget label).

**The tier is a derived view.** There is no `tier` field anywhere.
`Provenance::tier()` is `Evidence::tier()`, and each `Evidence` variant
carries exactly what its tier can prove: the enumeration's exact mass pair
and re-priced mass, or the consumed fact's id and value, or the mixture's
weighted pair together with the ledger's measured spend, or the §33 block,
or nothing but the field's own name. `Provenance` and `Decision` hold
private members and no public constructor, and the single assembly site is
one private `finish`. A fabricated `Evidence` value is constructible — it
is a public sum type and a reader must be able to match on it — but there
is nowhere to put it. Gate UP6 pins both halves.

**Derived views, never stored state.** The player stores exactly two
things: the per-seat carried `ModelBelief` lineage (which the brief
requires be carried and never recomputed) and the `ReceiptStore` of §49
proof states. Every reported quantity — live profiles, marginals,
augmented mass, posterior masses — is read back off the carried belief at
the moment of reporting. Gate UP3 checks this the only way that means
anything: it rebuilds each seat's belief from that seat's root and the
public line alone, and demands agreement in history, live profiles,
augmented mass, per-seat type marginals and per-profile posterior masses.

### Item 2 — the tier cascade

Each tier is one private function with one job.

- **(a)** `decided_success(position, seat, banked, false)`. When the
  indicator is settled every continuation values alike, so the declared
  lowest-tile tie rule answers; a forced play takes the same route with
  `settled: None`. Free, and the gate asserts `spend().total() == 0`.
- **(b1) consume.** If the store holds this root and its §33
  recommendation carries `Γ = 0`, the decision is that recommendation:
  `Evidence::Consumed` with zero spend, the fact id, and the action's own
  upper after closure. Requires a trick-start root, because
  `ProofState::open` asserts an empty partial trick.
- **(b2) enumerate.** Where the fiber is at or below the declared cap, one
  `extract_success_policy` at the state gives the optimum and the argmax
  action in a single walk; `viewer_success_mass` re-prices the extracted
  policy through the independent fixed-policy evaluator, which is §63's
  re-pricing law and is what makes the number a receipt. A disagreement
  would be a typed `RepricingDisagreed` refusal rather than an assertion —
  the player degrades instead of dying. On success the walk DEPOSITS: an
  executable lower for the chosen action, and a deterministic upper for
  every legal action, because the state's optimum is the max over actions
  and therefore bounds each of them.
- **(c)** `mixture_response_budgeted` on the carried belief under the
  declared read ceiling; the action is the extracted mixture policy's
  choice at the belief's own history. MB1's refusal is carried verbatim.
- **(d)** `ProofState::recommend()` accepted only within the budget's
  declared `regret_acceptance`.
- **(e)** the declared field's `choose`, through the counting decorator so
  the one consultation is measured. Needs no kernel, which is what keeps
  the cascade total even where `driven_root` refuses.

### Item 3 — the join

A `ModelBelief` is constructed at each seat's FIRST decision (its own
root) and carried: `focal_play` when that seat acts, `observe` when
another does, via `observe_play`, which the driver calls after every ply.

Where the budget declares `join_reading`, tier (b) also prices the model
side and records a `JoinReading` holding both exact values, both argmax
actions, and the two booleans. `PosteriorNote::consulted` is true exactly
when the answering tier read the posterior or a join reading was taken.

The measurement discipline MB1 asked for is honoured literally: values and
argmaxes are recorded separately because they move separately, and where
no argmax flipped, the transcript says the absence is censused and
corpus-scoped rather than saying posteriors do not matter.

### Item 4 — budget discipline

`MoveBudget` holds five declared quantities and a label. Two are
STRUCTURAL fiber predicates, free to check and checked before any spend;
one is an ENFORCED ceiling in field consultations, MB1's own unit measured
at the dispatch; one is the exact-rational regret acceptance; one is the
join-reading flag. Never wall-clock — a wall-clock budget makes a decision
a function of the machine, and gate UP4 pins that the same state under the
same budget yields a byte-identical `Decision`.

The two kinds of affordability are NOT interchangeable and the refusals say
which fired. The world-space recursions take no ceiling: they are full
walks with no abort point that leaves a meaningful partial value, so their
affordability is the declared fiber cap and their spend is measured
afterwards, by a counting decorator around the declared field. Gate UP5
pins that the decorator is value-neutral — it forwards `id`, so it IS the
declared field (the belief machinery asserts one field identity governs a
belief's conditionings, §43), and a decorator that renamed what it measures
would be measuring something else.

### Item 5 — the self-play probe

`unifiedreport report` → `walt/probes/factor_belief/unified_run1.txt`.
Seven roots × three rungs, walked to terminal with UP0 choosing EVERY
seat's action, 216 decisions, 31.5 s. Per move: tier, recursion and its
direction, authority, fiber, exact value, the three read columns, refusals,
posterior carried/consulted/live/falsified, the join reading, and wall
split into deciding and carrying. Then tier occupancy by trick, recursion
occupancy, spend and wall per rung, the join table, and the refusal census.
Findings language only; the file's own header and footer say it is a
transcript and not an evaluation.

### Item 6 — the field-identity fence

UP0 transports nothing across a field identity, so MB1's
`couple_fixed_field_fact` is never called and `model_recursion.rs` is not
imported by the player at all. The fence stays where MB1 built it. The one
identity UP0 does author is `fixed_field_identity`, a single public
construction site shared by the player and by anything seeding the store,
so that "the store already proves this root" is a statement about the same
object on both sides. Its `field_id` is the declared field's own content
address, so a fact authored under a different field is rejected
`IdentityMismatch` by machinery that already existed.

---

## THE GATES (`walt/walt/tests/solver_unified.rs`, 18 assertions, 17.0 s)

**UP1 — totality.** Three rungs (starved, lean, model) × seven roots,
every walk reaching terminal, every decision legal, the frame naming its
own seat/trick/ply/budget, the free tier spending zero and the fallback
spending exactly one. The sweep is required to reach at least the decided,
exact, mixture and fallback tiers, so totality is not demonstrated by a
single always-answering path.

Plus the SOURCE gate: `solver/unified.rs` contains no `unwrap`, `panic!`,
`unreachable!` or `todo!` on any code line, and every line holding
`expect(` also holds the string `rules invariant`. Every instrument refusal
in the module is matched and typed; the only assertions that remain are
properties of the rules of 42.

**UP2 — the endgame receipts are consumed.** U0's `GodGapWalk` census is
run on h5-t6 and h4-t6, its `coordinate_facts` installed into a
`ProofState` under UP0's own identity constructor (every install asserted
`Ok`), and the store handed to the player. The decision comes back
`Evidence::Consumed` at **zero total spend**, with its executable value
equal to the action's own upper after closure, and equal to U0's own God
upper for that action — recomputed from the census in the gate, never
quoted. The census is rendered before and after the decision and compared
BYTE FOR BYTE: the godgap and doom instruments are unperturbed.

Two companions: the same root with an EMPTY store enumerates instead
(`Evidence::Enumerated`, `optimum == repriced`, real consultations, fiber
27) — the same tier, a different evidence variant, which is the
consume/recompute distinction made visible; and h10-t6, whose contract is
already settled at the root, answers at tier (a) without ever reading its
seeded receipts.

**UP3 — the join.** The derived-view law per seat against an independent
replay (history, live profiles, augmented mass, per-seat type marginals,
per-profile posterior masses), with the gate also requiring that at least
one carried line folded in a real observation, so the law is not
demonstrated against a constant. Then the same law against MB1's OWN
instrument: `trace_heaviest_line`'s descent is replayed manually through
`focal_play`/`observe` and must land on the trace's own state, profiles,
mass and marginals. Then the three pinned specimens — h3-t5's value move
(29/40 against 4/5, argmax unchanged) and BOTH argmax flips: h8-t4
t4-p2 (617/864 against 173/216, model 0-0 against fixed-field 6-2, eight
profiles live) and h3-t5 t6-p2 (3/5 against 5/6, model 6-4 against
fixed-field 3-1, six profiles live). Each flip pins its exact pair, its
two actions, its live profile count, and that the cascade played the
fixed-field answer.

**UP4 — budget honesty.** A starved budget: every undecided state reaches
the fallback, the fallback is named as the FORWARD recursion, the field is
named by its content address, at least one refusal is present and the
exact tier's refusal names both fiber and cap. Determinism: same state,
same budget, identical `Decision` including provenance, spend and refusals.
Monotonicity: where both rungs afford the exact tier they report the same
exact value, so a bigger budget buys more tiers and never a different
number. And the empty-library lever: no line carried, no action, tier or
value changed.

**UP5 — the consumed instruments.** MB1's `mixture_response` and the raw
σ0 `response_success_mass` reproduce their values, policy ids and node
censuses either side of a unified decision. The counting decorator is
value-neutral on three roots: the bare field's optimum, re-priced mass and
argmax action are the player's. The inherited SUITES are the `check.sh`
conjunction, recorded under Status below (a test cannot run other test
files — MB1's M5 note, unchanged).

**UP6 — provenance soundness.** Every claimed tier re-derived
independently over three rungs: a decided claim reproduces
`decided_success`'s own answer; an enumerated claim's fiber, optimum,
re-priced mass and argmax action are recomputed with the BARE field; a
consumed claim spends nothing; a mixture claim's `Q(ν)` and argmax are
reproduced by an independently constructed belief and its ledger spend
matches the evidence; a certified-regret claim is within its acceptance; a
field claim names the declared field and spends one. The sweep is required
to have verified the free tier, both endgame-exact evidences, the
model-space tier and the fallback. Plus the structural half: `Provenance`
and `Decision` hold no public field, and the module holds exactly one
`Provenance` assembly site, which is private.

---

## STATUS

- `walt/ci/check.sh`: **PASS**, cold, 6 min 1 s, on the final tree.
- `solver_unified.rs`: 18 passed, 17.0 s.
- The inherited suites, green and untouched: MB0 8, σ1-repair 7 (1
  ignored, as before), U0 6, MB1 7.
- `git diff` against the base commit `ccafde9` over `refine.rs`,
  `doom.rs`, `godgap.rs`, `model_belief.rs`, `model_recursion.rs`,
  `proof_state.rs`, `factor_belief.rs`, `adaptive.rs`, `field.rs` and the
  old player's four binaries: **EMPTY**. UP0 is additive. The only edit
  outside new files is one line in `solver/mod.rs` registering the module.

---

## DEVIATIONS FROM THE BRIEF, RECORDED

1. **`decide` takes a `DrivenState` and the budget per call.** The brief's
   signature is `decide(seat, history, hand, budget)`. In this crate those
   four are exactly `DrivenState`: the SEAT is the derived view
   `leader.plus(trick_plays.len())`, the HISTORY is the public record
   (`prior_played`, `trick_plays`, `banked`, `voids`), the HAND is
   `viewer_hand`. Passing them as four loose arguments would have created a
   second authority for a state `adaptive::driven_root` already owns. The
   budget is a `decide` parameter rather than a constructor field, against
   the brief's "declared at construction", so that one player can be walked
   under a ladder and every decision names in its own frame the budget it
   was taken under.

2. **The §49 proof state is a trick-START object, so tiers (b1) and (d)
   refuse at mid-trick decisions.** `ProofState::open` asserts
   `position.trick_plays.is_empty()` — "the spike's roots are trick-start
   roots with the viewer to move". UP0 does not touch `proof_state.rs`, so
   at a mid-trick decision it types `ProofStateUnavailable { plays_in_trick }`
   and moves on; tier (b2)'s exact enumeration needs no store and still
   runs. This is 63 of the 104 refusals in the transcript — the dominant
   one. It is a scope limit of the spike, not an inconsistency, so the
   ambiguity protocol's failing-test route does not apply; it is recorded
   here, typed in the refusal enum, and named as UP1's clearest single
   piece of work.

3. **A third budget rung was added.** The brief asks for "at least two
   budgets". Under two natural rungs tier (c) never fired at all, because
   the declared cascade only reaches it where tier (b) refused and both
   rungs put the enumeration cap above the mixture cap. The third rung
   swaps the two structural caps so the model-space tier answers, and gate
   UP6 verifies a tier-(c) claim against an independent walk. Shipping a
   transcript in which a built tier never fires would have been shipping an
   unexercised tier.

4. **Tier (b) keeps its place above tier (c), and the flip is recorded
   rather than acted on.** Discussed under the join above. The brief's
   ordering is kept; the mathematics that argues for inverting it is
   stated; the two specimens are pinned; the decision is Jason's or UP1's.

5. **`Evidence` variants are constructible; `Provenance` and `Decision`
   are not.** The module doc initially claimed `Evidence` had no public
   constructor. It does — enum variants always do — and the gate caught the
   claim. The doc now states the fence where it actually sits, one level
   out, and UP6 pins that level.

6. **The brief was amended under the slice, and the amendment cost
   nothing.** Commit `1327e86` (the orchestrator's, sitting between
   `ccafde9` and this slice's first checkpoint) tightened
   `model_belief.rs` and `model_recursion.rs` from EXTEND-ONLY to
   CONSUME-ONLY. It required no change to the work: UP0 modified neither
   file, imports `model_belief` only through its existing public API and
   does not import `model_recursion` at all, and the diff against base
   over both is empty. Auditors anchoring at `ccafde9` should note that
   the brief file differs by that one commit, which is not this slice's.

7. **An overclaim the independent audit caught, and the remedy taken.**
   This report and the FACTOR-BELIEF ledger both originally said "both
   flips pinned exactly in gate UP3". That was FALSE when written: the
   gate pinned only h8-t4 t4-p2, and the h3-t5 t6-p2 flip — four exact
   rationals, `3/5`, `5/6`, and the two actions — was transcript-only,
   because the h3-t5 gate walk stopped at depth 2 and never reached the
   trick-6 state. The gate itself was honestly named in the singular; the
   PROSE overclaimed, which is a tier violation of exactly the kind the
   house rules exist to prevent: exact numbers quoted for an ungated state
   as though a receipt carried them.

   The remedy is the stronger of the two the audit offered: the h3-t5 walk
   now runs seven plies to the trick-6 ply-2 state and the gate asserts
   that flip with its exact values, so the claim became TRUE rather than
   being softened. The gate is renamed
   `up3_both_argmax_flip_specimens_are_pinned` and both specimens now also
   pin their live profile count — h8-t4's flip sits under an untouched
   eight-profile prior while h3-t5's sits under a posterior that has
   already zeroed two profiles, which makes the newly gated one the
   stronger evidence of the pair. Cost: the UP0 suite moved from 16.4 s to
   21.1 s, all 18 assertions green.

   Worth recording as a process note rather than only as a fix: the gate
   and the prose were written at different times, and the prose
   generalized from two observed flips to "both pinned" without
   re-reading what the gate carried. A quoted rational is a claim on the
   receipt tier; the discipline that catches this is to write the number
   into the gate first and the prose second, never the reverse.

   Two more inaccuracies of the same family, found by self-checking the
   rest of the quoted numbers after the audit rather than by the audit
   itself, both now corrected above. First, the value-move table was
   introduced as "nine states where the posterior moved the value and
   left the argmax alone" — it is SEVEN; the probe's "9 moved the value"
   counts the two flips, which moved both, in that total as well. Second,
   that table's six ungated rows now carry an explicit tier marker naming
   the one row a gate does carry, instead of relying on a reader to infer
   the tier from the absence of a pinning claim. Under the standard the
   audit applied, exact rationals get their tier stated, not implied.

---

## WHAT UP1 AND UP2 NEED

### For UP1 (bridging and interactivity)

**Lift the trick-start boundary on the proof state.** 63 refusals in one
transcript is a boundary the player runs into constantly, and it costs
tiers (b1) and (d) three quarters of their reachable states. This is
additive work inside `proof_state.rs` with a real gate cost (its six gates
must not weaken), and it is the difference between a store that a live
player consults once per trick and one it consults every ply.

**Settle the (b)-versus-(c) ordering.** Two exact specimens exist. The
mathematics (MB0's point-mass parity) says tier (b) is tier (c) at
`δ_{F₀}`, which makes the current order a declared preference for a
narrower belief. Inverting it is a two-line change in the cascade and a
gate rewrite; NOT inverting it is also defensible, since the mixture's own
belief is a modelling assumption the probe's opponents falsify. The
decision wants Jason.

**Decide what a live player carries.** The carry measurement says a
posterior costs 2.1 s per 72 decisions to maintain and nothing to ignore.
Either the player consults it (in which case tier (c) must be reachable,
which means the ordering question above), or it declares an empty library
and pays nothing. Carrying it unread, which is what the lean rung does, is
the one combination with no case for it.

**Wall, for planning.** Mean decision: 168 µs (lean), 93 ms (model), 278 ms
(ample). Worst single decision: 11.3 s, at h8-t4's join reading. A tier-(b)
answer at a t6 fiber is sub-millisecond; at a t5 fiber, tens to hundreds of
milliseconds; a tier-(c) answer at a t5 fiber is seconds. Nothing here is
an interactive latency at trick 4, and nothing here needs to be.

### For UP2 (structural producers past the wall)

**Tier (d) is built and structurally unreachable on this corpus**, because
the only producer filling the store is tier (b), which only deposits where
it also answered. Tier (d) becomes the interesting tier the moment a
producer that is NOT the exact walk fills the store: U0's God uppers at a
root the enumeration cannot afford, the doom census's certified mass, the
unbuilt §33 producers. That is the first thing UP2 should aim at, and the
plumbing — `ReceiptStore::seed`, the shared `fixed_field_identity` — is
already gated by UP2's own seeded-store test.

**The corpus stops at trick 4 for a measured reason.** MB1 located the wall
between trick 4 and trick 3, and the transcript reproduces it from the
player's side: at h8-t4 the model tier refuses structurally on every rung
and the world tier costs 2.5 s for one decision. Anything earlier needs a
structural producer, not a bigger budget.

**A type library that contains the player.** Nine falsification events say
the registered two-rung ladder does not survive contact with UP0's own
play. MB1 already wanted a library "chosen to disagree EARLIER"; UP0 adds
that it also has to disagree WIDELY enough to still have support after a
few plies of strong play.

---

EXPLORATORY — below every evidentiary tier; quotable only via gate
receipts. Every number above is a measurement on the declared corpus,
budget ladder and type library named in the probe's header. Nothing here is
a claim about how well the unified player plays, and nothing here is a
comparison to any other player.
