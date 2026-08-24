# walt mathematics — open questions

[Home](Home.md) · owns: the inventory of genuinely open mathematical questions
in the walt branch, at the exploratory tier · Sources: `walt/CENSUS-RULINGS.md`
(the ruling that left each one open), `walt/math/decision_sparse_exact_solving_v0.1_errata.md`.
Related: [the reference map](walt-math-reference.md),
[decision-deadness](walt-math-deadness.md),
[structure and transport](walt-math-structure-transport.md),
[decision-sparse witnesses](walt-math-decision-sparse.md),
[the freeze register](walt-math-freezes.md).

> **Coverage note (2026-08-24) — reorganization pending, see [[math-reorg]].**
> This inventory predates the scenario-player era. The open questions it lists
> remain open as stated, but the era added new ones it does not carry — the
> `walt/SCENARIO-PLAYER.md` §10 obligations ledger, the level-2 field-swap
> question (`walt/LEVEL2-PROBE.md`), and the conditional-moment gap the x:018
> correspondence names — and gathering those into one inventory is
> [[math-reorg]]'s charter.

> **Tier: EXPLORATORY throughout**, below every tier on
> [Home](Home.md#evidentiary-tiers--never-promoted-never-blurred).

## Why these are not in [open-problems](open-problems.md)

That page owns the **merged corpus and exchange-tier** OPEN inventory — the
questions the two immutable specification packages leave open, with their
adjudicated statuses. Everything on *this* page sits below every one of those
tiers. Putting a walt question there would blur the tier boundary in exactly the
direction the project forbids: an exploratory question would acquire, by
adjacency, the standing of a corpus-proved boundary.

The same reasoning explains why walt has **no entries in
[claim-ledger](claim-ledger.md)** and should acquire none. The ledger records
claim tiers; walt is below every tier, so its correct entry count is zero. A
successor who notices the absence should not "fix" it. If a walt result is ever
to be promoted, the path is the one [walt.md](walt.md) states — independent
re-verification, through Lean — and promotion is what creates a ledger entry,
never the other way round.

---

## The open questions

### 1. The specimens' tie mechanism — UNIDENTIFIED

**Status: open, and deliberately not filled.** The three adjudicated deadness
detectors — [J-0, J-1, J-win](walt-math-deadness.md) — do not explain the ties
observed at the trumpless-junk grade-3 family. At that coordinate the three
leads carry **276, 1,239 and 1,773 classified ties** and **zero detector hits**
between them. J-A8 ranks the family and lists the fourth entry as "the
specimens' mechanism — UNIDENTIFIED".

**Why it was left open rather than patched, and this is the part to preserve.**
The ruling states it plainly: *"No cheap sufficient structural condition is known
at adjudication time for the six specimens' ties, and none is invented here."*
A full one-deviation evaluation **is a solve and is therefore not a detector**.
If the accepted members leave the specimens uncovered, the run records the
residual as a named open question **with its witnesses**, and does not ship a
fourth detector without a proof of the shape given for the first three.

**What would close it.** A proved sufficient structural condition, in the shape
of J-0 or J-1: checkable on public data at the node, with a proof that it
implies value-invariance, and a declared count tag. Anything that requires
evaluating continuations is a solve wearing a detector's clothes.

### 2. Whether a coarser lawful seat equivalence reaches the bar

**Status: open, and explicitly declared out of scope by the census that raised
it.** [Corollary S-rigid](walt-math-structure-transport.md#corollary-s-rigid--the-first-play-transport-group-is-trivial)
proves the first-play seat transport group is trivial, so the finest seat-side
equivalence — structural form — gives exactly C(28,7) = 1,184,040 hands, missing
the 10⁵ bar by a factor of about 11.84. S-A20 fences the result: it answers the
bar **only for that finest equivalence**. It is not a count of any coarser
censal equivalence — not an r3-style dynamics quotient, not a value partition.
**Whether some coarser lawful equivalence reaches 10⁵ is OPEN and that census
does not address it.**

**The standing explanation, which shapes where to look.** S-A21's reading is
that compression in this project is bought with **deadness**, and at the first
play nothing is dead and nothing is inert. That suggests a coarser equivalence,
if one exists, will not be structural.

### 3. Is dim V^val small anywhere that matters?

**Status: measured negatively at grade 3, open beyond it — and the extrapolation
is forbidden.** [Lemma R](walt-math-information-geometry.md#lemma-r--three-continuation-closures-and-the-separating-observation-degeneracy)
proves the degenerate contracts have predictive dimension exactly |X| and
carefully leaves the value closure open. The S6a measurement then found grade-3
dimensions of {1461, 1492, 1680} against |X| = 1680 — one coordinate at full
rank exactly — and recorded the Gate-B payoff **REFUTED**.

**What remains genuinely open, and what does not.** Whether the value closure is
small at *higher* grades is not answered; but P-A21 binds hard here — **three
rungs are not a law**, an implied higher-grade dimension is an extrapolation at
the exploratory tier and is **never a statement about an unrun computation**,
and **no dimension at any grade is quoted for the opening**. So this is open in
the sense that it is unmeasured, not in the sense that a trend points anywhere.

### 4. Moment compilation — named, separate, and unmeasured

**Status: open, and never to be conflated with predictive rank.**
[Corollary E5.2 and the §5.3 reframe](walt-math-decision-sparse.md#5-the-scheme-mass-closure-is-degenerate-in-this-game-5-ds-a5-ds-a20-ds-a21)
establish what the degeneracy does *not* rule out: a compact arithmetic or
Boolean circuit, a factorised representation, a BDD/ZDD-style exact form,
fixed-prior symbolic propagation, purpose-specific moment compilation, or
fixed-policy weighted model counting. **Predictive rank lower-bounds the
corresponding linear factorisation target; it does not lower-bound unrestricted
nonlinear circuit size**, and no result in this branch supplies a technique that
does.

DS-A11 sketches the lawful form of the experiment: measure **circuit size
against world count** for a *declared purpose* — one likelihood, one posterior
moment family, one fixed lawful policy's value — with bit-exact agreement
against enumeration, and state in the header that no algebra-compression claim
is made and that exact normalised filtering re-enters the degeneracy.

### 5. The economy claim: the primal half is tested by design; the full claim is not

**Status: split, as of EC-A13 (2026-08-13).** The claim has two halves and they
now have different statuses, so the question is no longer simply open.

The parent document's economy claim is that a solver ["does **not** need an exact
solution for every action"]. SEP-A17 recorded that the separation probe does not
exercise it: by
[Corollary E4.1](walt-math-decision-sparse.md#corollary-e41--the-primal-ceiling-and-the-exact-negative)
its primal witness is an H-optimal policy re-priced, and the DS-A10 receipts
require treatment H at every action anyway.

**The primal half — tested by design.** Whether the *witness at a⋆* must be an
exact solve is exercised by the economy-successor design, at coordinates where
every claim is checkable: L is seeded from sources that are not exact solves at
a⋆ (transported library entries, four fixed tile rules, and a heuristic re-key),
and the question is whether the sandwich still closes. The transport clause it
needed is now open — freeze 36 v2 (EC-A8) admits the declaration fold, with
values licensed by
[Corollary S-fold-val](walt-math-structure-transport.md#lemma-s-fold--the-seven-pip-declarations-fold-exactly-71)
and verdict transport by Lemma E7. Note this is a statement about the *design*;
results are a separate matter.

**The full claim — untested, and a different object.** A solver that avoids
exact solves additionally requires the **U side cheapened**: a relaxation coarser
than treatment C, run down Theorem E6.5's ladder. That is Experiment D's
territory, with freeze 38 still reserved, and nothing has touched it.

**Binding on how this is written up (EC-A13):** a results file or wiki sentence
saying "the economy claim was tested" **without the word "primal" has
over-claimed**. The two halves are never collapsed into one sentence.

### 6. The reserved freezes 38–40

**Status: reserved, untouched, and each marks a designed-but-unbuilt object.**

- **38** — the gluing-cut language, the validity-proof obligation, and the cut
  ordering. Blocks Experiment D (adaptive gluing), which is designable given
  Theorem E6.5's two obligations but has no design filed.
- **39** — the circuit representation and its evaluation order. Blocks the
  moment-compilation experiment above.
- **40** — the reachable-belief family defining W_reach, with its deal-level
  typing. What remains before it can run is an **enumeration of the reachable
  posteriors on a carrier small enough to enumerate exactly** (DS-A23). The
  retyping itself is done: [Definition E9](walt-math-information-geometry.md#definition-e9--interface-local-reachable-decision-width).

### 7. δ-similarity — future mathematics, not a gap

**Status: named as out of scope, requiring its own typed rulings.** The
policy-geometry fence states it: *"'Playing this domino means I am likely to get
32 one way or the other' is a statement about score distributions under a
tolerance, and this probe measures neither: score is out of scope, and by
Lemma R(c)–(d) the distribution contract has predictive dimension |X|;
δ-similarity is future mathematics requiring its own typed rulings."*

This is a different kind of open item from the others: not a question the
existing machinery could answer if pushed, but a notion that does not yet have a
definition inside walt's type discipline. **No similarity claim and no tolerance
claim of any kind is made or supported anywhere in the branch.**

### 8. The count-and-score lift

**Status: not lawfully designable from the current documents.** DS-A12 rules
that the lift needs its own design and its own adjudication, minimally covering
the declared cone, the feature law's construction, which count-free verdicts are
re-derived versus inherited, and what happens to every form-keyed record. The
last is settled and severe: under E-A2 every form-keyed record is **void
wholesale, never extended**.

**What is already known about the boundary**, and it is more than when the
question was raised:
[Lemma E8 and Lemma J(c′)](walt-math-deadness.md) give the exact valuation scope
— constancy on the exchanged tiles, gauge-stable — and
[Corollary E1.2](walt-math-decision-sparse.md#corollaries-e11-and-e12-16) gives
count survival under the guard for the transposition instance. **Propositions
J-0 and J-1 survive count re-entry; J-win does not.** And DS-A16 identifies the
one asset that crosses intact: a lawful policy remains a valid primal witness
under any valuation. The policies extend; the verdicts do not.

---

## Two things that are settled and might look open

Recording these prevents a successor from re-opening closed questions.

- **The rank question is not open.** R-A16 and the v0.6 audit are explicit: for
  the distribution contracts the answer is |X|, by Lemma R(c). Those rows are
  **theorem rows, not measurements**, and a run returning anything else is a
  stop-and-report bug because the lemma says what it must return.
- **The exchangeability question is not open, it is repaired.** Parent §7.1 is
  unsound as written, and the repair is exact:
  [Theorem E1](walt-math-decision-sparse.md#1-order-exchange), generalised by
  E1′. What is open is only whether the general (H1)–(H3) form ever finds an
  instance beyond the J-1 transposition — and DS-A25 notes that
  [Corollary R-fold](walt-math-structure-transport.md#corollary-r-fold--the-predictive-dimension-is-declaration-fold-invariant)
  is the branch's only exhibited value-order isomorphism to date.
