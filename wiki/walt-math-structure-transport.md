# walt mathematics — structure, transport, and the quotient

[Home](Home.md) · owns: the structural-isomorphism and transport lemmas of the
walt branch — Lemmas V, X, E, S, S-fold, S-det and Corollaries S-rigid, R-fold ·
Sources: `walt/CENSUS-RULINGS.md` (statements and proofs live there; this page
is the map). Related: [the reference map](walt-math-reference.md),
[information geometry](walt-math-information-geometry.md),
[decision-deadness](walt-math-deadness.md),
[the freeze register](walt-math-freezes.md).

> **Tier: EXPLORATORY throughout**, below every tier on
> [Home](Home.md#evidentiary-tiers--never-promoted-never-blurred). Every
> statement on this page is proved relative to walt's own declared basis and is
> cited by nothing above the Ideas tier.

These are the lemmas that answer "when may two things be treated as one?" They
were proved across four probe adjudications between 2026-08-10 and 2026-08-12,
and the honest summary of the group is that **they are sound and they buy far
less compression than the designs that requested them hoped** — two of them are
proved negatives. That is a result, under F7.

---

## Lemma V — value descends to r3 classes

**Statement (exploratory).** Let an operator assign a value to each state by a
node rule reading only (i) the actor offset and (ii) the canonically ordered
family of per-move pairs (increment, value of successor). Then the value is
constant on r3 classes. Proof by induction on grade, using strict grading
(r3 Q2) and the signature tuple of r3 Q3.

**Load-bearing hypothesis.** The node rule reads *only* actor offset and the
canonically ordered (increment, successor-value) pairs — never tile identity,
never history, never a bag or a weighting.

**Full statement and proof:** `walt/CENSUS-RULINGS.md` § "Fiber-probe rulings",
under the heading "Lemma V (value descends to r3 classes)".

**What it binds.** P-A9's same-object receipt (assert `value(world) =
value(root class of that world)` for arm B in-run); P-A14's narrowing of the
amortisation claim; the `X_val0` predicate of the fiber-refinement probe;
Lemma X's proof, which is stated in terms of a Lemma-V value.

**The scope fence, and it is the point.** Lemma V covers the world-informed
focal-max / hidden-uniform-expectation operator and the perfect-information
minimax operator. **It does not cover treatment H**, the actual
hidden-information solve: H's value at an information state is a function of the
bag and its weighting, not of any single state's class. Under Y3, a change of
weighting for H is a **re-solve over a fixed DAG, never a re-fold**. The rulings
call this distinction the spine of the fiber probe and say it must not be
blurred.

---

## Lemma X — zero-contribution excision

**Statement (exploratory).** Under a non-negative valuation, let V*(ω) be ω's
Lemma-V value under the frozen world-informed operator, let U(ω,ρ) be the
field-expected focal trick count in ω under an information-consistent policy ρ,
and let Z = {ω : V*(ω) = 0}. Then U(ω,ρ) = 0 for every ρ and every ω ∈ Z;
consequently, for the **unnormalised** objective J(ρ) = Σ_ω β(ω)U(ω,ρ), deleting
Z leaves J unchanged as a function of ρ, so the argmax set and the unnormalised
optimal value are preserved exactly.

**Load-bearing hypotheses.** Non-negative valuation; ρ information-consistent
(so the pointwise strategy-fusion inequality U ≤ V* applies); V* taken under the
*frozen* operator; the objective **unnormalised**.

**Full statement and proof:** `CENSUS-RULINGS.md` § "Fiber-refinement rulings",
under "Lemma X (zero-contribution excision)".

**Its place in the standing rule.** The governing discipline is: *evaluate a
fixed policy on a remnant, never re-optimise over a remnant and call the result
a seat value.* Lemma X is the one exception, and it is one-sided.

**Three recorded limits.**

1. **One-sided.** V* is only an upper bound. V*(ω) = 0 forces the contribution
   to zero; V*(ω) = n forces nothing. The proposed dual predicate `X_val_max`
   is therefore **not** the symmetry the design called it, and its stated role
   was REJECTED (X-A3): excluding maximal-value worlds is a genuine information
   injection and would change the problem.
2. **Normalisation breaks it.** Under β(·|Z^c) the optimum is the unnormalised
   optimum divided by (1 − β(Z)) — a different number, and not any seat's value.
   Report unnormalised sums and the excluded mass β(Z).
3. **Policies preserved only up to vacuity.** Deleting Z can empty an
   information state. Values are preserved; policy identity at emptied states is
   not.

---

## Lemma E — structural isomorphism ⇒ count-free value equality

**Statement (exploratory).** If x and y have equal r1 canonical forms **as
amended by F2 A1–A4**, then there is a bijection of live ∪ unresolved-trick
tiles together with a rotation of seats carrying x to y and preserving: holders
by relative seat, the trump/non-trump split, follow membership, the led-context
map ℓ, pairwise trick keys, double flags, table order, current winner, and
focal ↔ focal. Every rule of the remaining game reads only those data, so the
map is an isomorphism of the remaining extensive games, and every count-free
value that is a fold over that tree is equal at x and y.

**Full statement and proof:** `CENSUS-RULINGS.md` § "Endgame-store rulings",
under "Lemma E (structural isomorphism ⇒ count-free value equality)". Note that
it *replaces* the design's proposed chain (form-equal ⇒ r1-equal ⇒ r3-equal ⇒
same value) and needs neither ECL nor r3.

**Load-bearing hypotheses.** The implemented canonical form must carry **every**
relation in F2's amended list. A dropped relation — the led-context map (A2),
the unresolved-trick tiles (A1), a reflected seat matching (A4) — breaks the
lemma. And any operator used with the store must be **declared
isomorphism-invariant** (E-A3); an operator that reads tile identity does not
qualify.

**The hard scope limit — E-A2, and it is the branch's most dangerous edge.**
Lemma E's bijection preserves BEATS relations, **not pip counts**. A
canonical-form-keyed store is therefore sound only for count-free valuations. If
count ever re-enters, **every record in the store becomes unsound and the store
is invalidated wholesale, never extended.** The rulings name this "the one
failure mode that would silently produce wrong numbers rather than a crash."

**Attribution (E-A1).** The escape belongs to **r1** — the finest structural
quotient, relabeling symmetry — not to r3 and not to equivariant lumpability.
S5h's finding that cone identity cannot short-circuit descent is unchanged and
is not rescued here.

---

## Lemma S — seat-side structural transport

**Statement (exploratory).** For a first-play seat situation x = (δ, H) with
|H| = 7, focal = declaring seat = leader, pool U = 𝒟∖H: a **seat transport**
φ : 𝒟 → 𝒟 from (δ,H) to (δ′,H′) is a bijection with φ(H) = H′ carrying the
δ-structure to the δ′-structure — preserving trump membership, follow
membership, the led-context map **on every live tile**, the double flag, and the
winner-determining order in every context — with the seat rotation the identity
(focal fixed; reflection forbidden). If such a φ exists then (1) φ carries the
seats' fibers, capacity-cell systems and exact support normal forms across;
(2) each pair (x⊕ω, y⊕φω) satisfies Lemma E's hypothesis, so the remaining games
are isomorphic; (3) hence **every count-free censal question** has the same
answer at x and y — legal-lead counts, realizable trick-1 records, the
records→landings map with multiplicities, the void structure at every landing,
and per-world count-free values for any isomorphism-invariant operator.

**Full statement and proof:** `CENSUS-RULINGS.md` § "Seat-census rulings", under
"Lemma S (seat-side structural transport)".

**Clause 4 — not preserved, and never to be claimed.** Any count-bearing
quantity (E-A2 applies verbatim to every seat-side form), and any
belief-relative quantity whose belief is not itself transported by φ. The
uniform-legal field is transported; an arbitrary β is not.

**Why the invariant list cannot be trimmed.** S-Q1 ruled the design's proposed
list INCOMPLETE with five gaps, the first two unsound. A run implementing the
list as proposed "would report a COUNT 1 far below 1,184,040 and could appear to
clear the 10⁵ bar. It would be clearing it by merging hands that face different
games."

---

## Corollary S-rigid — the first-play transport group is trivial

**Statement (exploratory).** For every pip-trump δ, the group of self-transports
of the δ-structure on the full 28-tile live set is **trivial**. Hence the
seat-side hand form at the first play **is the hand**, and the seat-side
structural quotient at the first play is the identity quotient.

Proof in three steps: for a non-trump d = q:r, ℓ_δ(d) = max(q,r), so the induced
context map commutes with max and is order-preserving, hence the identity; φ is
then the identity on non-trumps; and φ permutes the trumps preserving a strict
total order, so it is the identity there too.

**Full statement and proof:** `CENSUS-RULINGS.md` § "Seat-census rulings".

**Consequence — a proved negative, in F7's sense.** COUNT 1 = C(28,7) =
**1,184,040** is a *theorem, not a measurement*, missing the 10⁵ bar by a factor
of 29601/2500. The rulings are explicit that this "is not a defect of the
construction and not a reason to re-cut the invariant list." Steps (i) and (iii)
are independent routes — the led-context map alone pins the non-trumps, the
trump ranking alone pins the trumps — so **dropping either from the invariant
list does not rescue compression; it only makes the form unsound.**

**The explanation the branch settled on (S-A21).** Compression in this project
is bought with *deadness*. At the first play nothing is dead and nothing is
inert, so there is nothing to buy it with.

**Scope fence (S-A20).** COUNT 1 answers the bar only for the *finest*
seat-side equivalence, structural form. It is not a count of any coarser censal
equivalence — not an r3-style dynamics quotient, not a value partition.
**Whether some coarser lawful equivalence reaches 10⁵ is OPEN** and this census
does not address it. See [open questions](walt-math-open-questions.md).

---

## Lemma S-fold — the seven pip declarations fold exactly 7:1

**Statement (exploratory).** For pip trumps p, p′ let π_{p→p′} be π(p) = p′
together with the **unique order isomorphism** ℙ∖{p} → ℙ∖{p′}, and
φ_{p→p′}(a:b) = π(a):π(b). Then φ_{p→p′} is a seat transport from the δ=p
structure to the δ=p′ structure; **it is the unique one** (by Corollary
S-rigid); and the transports compose. Declaration orbits therefore have exactly
seven members, and COUNT 1 = 1,184,040 folded.

**Full statement and proof:** `CENSUS-RULINGS.md` § "Seat-census rulings".

**The dependence is exact.** The fold depends on **S-A2's comparison reading**
(freeze 18): the winner-determining order per live context, with all tier-0
tiles collapsed into one bottom class. Under the *literal* §1.3 reading, where
tier-0 tiles are ordered among themselves by pip sum, the fold **collapses to
the single pair δ=0 ↔ δ=6 and nothing else** — a counterexample is recorded
(under δ=0 the tiles 4:1 and 3:2 have equal trick keys in context 6, while their
images under φ_{0→3} have keys 4 and 3). The bar's answer is insensitive to the
choice, and S-A2 requires the run to print which reading it froze and what the
other would have given.

**But the dependence is form-level only (Corollary S-fold-val, EC-A4).** The
reading-dependence above is a statement about transports of the recorded
relational **form**; it has **no value-level content**.

> **Corollary S-fold-val (value transport along the declaration fold is
> reading-independent).** Let φ = φ_{p→p′} be the transport above, restricted to
> the live set of a rung coordinate as in Corollary R-fold. Then for *every*
> ordered pair of pip declarations the induced bijection of fibers satisfies
> α_{Tρ}(Tξ) = α_ρ(ξ) for every lawful ρ and every ξ; hence Q^H per corresponding
> action, every fixed-policy value L, and every treatment-C value U_a correspond
> exactly along φ — **independently of which S-A2 reading is adopted**.

*Proof sketch.* Dynamics read only legality (follow membership and the
led-context map), the double flag, trump membership, and the winner-determining
comparison. By S-A2's soundness clause the maximum trick key is always attained
at tier ≥ 1, so the mutual order of tier-0 tiles is read by no rule. Lemma
S-fold's preservation argument shows φ preserves every datum in that list for
every ordered pair — and the literal reading's counterexample above (4:1 versus
3:2 in context 6 under δ=0) concerns two tier-0 tiles, exactly the relation
dynamics never read. Corollary R-fold's proof then supplies the bijections of
legal sets, uniform field masses, observations and count-free outcomes. **Full
statement and proof:** `CENSUS-RULINGS.md` § "Corollary S-fold-val".

Two consequences worth carrying. First, the count of form transports verified at
adjudication time (49 under the operative reading, 9 under the literal) counts
**forms**, not value correspondences; reading it as the latter is the error EC-A4
was written to stop. Second, because no value observable distinguishes the
readings, a probe proposing to *measure* the difference measures nothing — which
is why the economy successor's transport arm is typed as **receipts** rather than
as a diagnostic, with a mismatch at any image being stop-and-report (a defect in
the rules, the fold implementation, or the key correspondence) and never a
finding about the game.

**Out of scope.** Doubles-trump and no-trump declarations are outside the
declared scope, and the fold says nothing about them.

---

## Lemma S-det — determination of the landing state

**Statement (exploratory)** — reproduced nearly whole, because the void clause is
subtle; the ruling's inline rule citations and its displayed equivariance
equation are elided. "The
landing state is a function of (δ, H, r) where r = (d₁,d₂,d₃,d₄) is the ordered
trick-1 play record: the landing hand is H ∖ {d₁}; the live pool is
U ∖ {d₂,d₃,d₄}; for each hidden offset i the observed void is 'void in ℓ_δ(d₁)'
exactly when d_{i+1} ∉ σ̂^δ_{ℓ_δ(d₁)} (**a slough proves a void in the led
context and proves nothing else**); and the next leader is the offset attaining
the maximum trick key. By Lemma S the assignment is equivariant, so (hand form,
interface element) determines the landing form."

**Full statement and proof:** `CENSUS-RULINGS.md` § "Seat-census rulings",
inside the S-Q2 ruling.

**What it binds.** The successor void content of the predictive-rank probe:
successor interfaces carry the induced voids, and S-det fixes them exactly.
There is an in-run receipt asserting that the surviving worlds of a record
biject with the successor fiber.

**The negative it carries (S-A9, verbatim).** "Determination holds with a
bounded alphabet, and the bound is the raw space. The first-trick interface
alphabet is the set of ordered 4-tuples of distinct tiles, at most 28·27·26·25 =
491,400 overall and at most 7·21·20·19 = 55,860 above a fixed hand form; because
the hand form's stabiliser is trivial (Corollary S-rigid), no coarser interface
element determines the landing. **The interface buys no compression at the top
of the game.** Both outcomes are results (F7); this is the negative one, and it
is proved rather than measured."

Two further fences: the realizability count is a legality census of the record
space, not a compression measurement (S-A10); and the interface element is a
*counting coordinate*, never a trick-level macro kernel — no value, increment
composition or kernel may be read off it (S-A12).

---

## Corollary R-fold — the predictive dimension is declaration-fold invariant

**Statement (exploratory).** Let φ = φ_{p→p′} be Lemma S-fold's declaration
transport, restricted to the live tile set of a rung coordinate. Then the image
is a coordinate, and φ induces a bijection of fibers carrying legal sets to
legal sets, uniform field masses to uniform field masses, observations to
observations, count-free increments to increments and focal to focal. Hence the
closure matrix at the image is the original with rows and columns permuted, and
**dim V is equal at the two coordinates**, with corresponding behavioural-row
counts and corresponding policy values.

**Full statement and proof:** `CENSUS-RULINGS.md` § "Predictive-rank probe
rulings", under "Corollary R-fold".

**Load-bearing hypotheses.** Lemma S-fold's preservation list (hence S-A2 /
freeze 18); the coordinate taken **at a trick boundary**, so F2 A1's
unresolved-trick extension is inert; identity seat rotation with focal fixed;
the uniform-legal field — an arbitrary field or belief is not transported;
count-free increments.

**What it buys (R-A7, the fold receipt).** The cheapest strong receipt available
at the coordinate level: the sample is closed under the declaration fold, and
the run asserts equal dimensions, equal behavioural-row counts and equal policy
values along all 49 ordered declaration pairs. A failure is stop-and-report per
NO-RESCUE — an implementation defect or an error in the corollary, **never a
finding about the game**.

**What does not transport.** The basis and the closure matrices. They are chosen
by a deterministic pivot rule over a declared enumeration order, which is not
φ-equivariant. So the in-run assertion is equality of dimensions and of values,
**never byte-equality of matrices**, and every sparsity figure is freeze-relative
(R-A21).

**Count re-entry kills the fold.** The count schedule is not pip-symmetric, so a
score decoration breaks R-fold. This is printed in the results file because it is
a failure mode that would produce wrong numbers rather than a crash.

**One instance, and only one.** Under Lemma E7 (DS-A25), Corollary R-fold is an
instance of a genuine value-order isomorphism and may be cited as one. **Nothing
else in the branch currently is.**

---

## What is in force

Each later ruling section names the standing set in its basis paragraph. As of the
Experiment E adjudication (2026-08-13), all eight objects on this page are in
force unchanged, alongside Lemmas R, G, J, Propositions G-flat, J-0, J-1, J-win
and the errata's E-series. The authoritative "what is in force" lines are the
basis paragraphs opening each dated section of `walt/CENSUS-RULINGS.md`.
