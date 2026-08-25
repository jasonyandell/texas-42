# walt mathematics — open questions

[Home](Home.md) · owns: the inventory of genuinely open mathematical questions
in the walt branch, at the exploratory tier · Sources: `walt/CENSUS-RULINGS.md`
(the ruling that left each one open), `walt/math/decision_sparse_exact_solving_v0.1_errata.md`,
`walt/SCENARIO-PLAYER.md` §10, `walt/LEVEL2-PROBE.md`,
`walt/math/calculated_evidence_v0.1.md` and
`walt/math/targeted_level2_field_stability_v0.1.md` (items 13–15),
`exchange/README.md` (x:018).
Related: [the reference map](walt-math-reference.md),
[received artifacts and intakes](walt-math-intakes.md),
[decision-deadness](walt-math-deadness.md),
[structure and transport](walt-math-structure-transport.md),
[decision-sparse witnesses](walt-math-decision-sparse.md),
[the freeze register](walt-math-freezes.md).

> **Coverage (2026-08-24, the [[math-reorg]] pass; extended same day for the
> calculated-evidence era).** Questions 1–8 are the census/decision-sparse
> era's inventory and remain open as stated; the scenario-player era's
> additions are items 9–12 (item 10 annotated: its gate landed); the
> calculated-evidence era's additions are items 13–15. Era narrative and
> instrument records: [walt-calculated-evidence](walt-calculated-evidence.md).

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

### 9. The scenario-player obligations ledger — the era's whole proof debt

**Status: open by construction, and owned elsewhere.** The
`walt/SCENARIO-PLAYER.md` spec is explicit that its statements are definitions
and proof obligations, not established results; its **§10 ledger** (O1–O9
spec-native, O12–O19 filed from the signed-pivotal §14, **O20–O28 accepted at
CE-A4 and O29–O38 at L2-A2**, both 2026-08-24; O10–O11 permanently
retired at SP-A11) is the authoritative queue and is **not restated here** —
one page owns each topic. The mathematically load-bearing rows: **O2** (key
sufficiency, Lemma 2.4 — unproved on paper, candidate for exchange review),
**O4** (posterior semantics, Lemma 5.2 — "the load-bearing one"), **O7**
(execution-order invariance from purity plus rational arithmetic). Graduation
path per project law: paper proofs → wiki with tier labels → independent
re-verification → Lean for what earns mechanization.

### 10. The level-2 field-swap question

**Status: still open as a question — but its gate landed and its instruments
now exist (annotated 2026-08-24).** Where does pivotal mass wake up under a
field upgrade — q(level-0 field) ≈ 0 but q(level-1 field) > 0 — and does
modeling the partner's response *shrink* fixed-pair hardness H (making
level 2 cheaper to sample at equal confidence, not just stronger)? The
adaptive-sampling mathematics this was gated on **has landed and been
adjudicated** (CE-A1..A8; the wake-up notion split three ways at CE-A6 —
response q, value g, decision — with sampling cost compared by
`𝓘 = q·D_{1/2}(τ)`, never q̂ alone), and the targeting mathematics followed
the same day (L2-A1..A7; `walt/LEVEL2-PROBE.md` amended to the *detection
layer*, L2-A5). The first field-swap slice ran a fixed-policy smoke —
including one exact-fiber root where the fields never split, the targeting
phenomenon in the wild — but a `FrozenPolicyExposure` answers no wake-up
question at the root-action level (L2-A4).

**Movement (slice 2, 2026-08-24):** the second slice built the machinery that
*can* answer at the root-action level — exposure rungs E0–E2 and the exact
split-reach route E4, whose optimum **is** `R_a` — and on receipt-h7-t5 the
rung **E0 fired**: no reachable non-focal state disagrees after any legal root
action, so `R_a = 0` exactly over all information-consistent continuations.
That converts the smoke's two-policy d = 0 into an exact zero and answers the
wake-up question **at that one root, negatively**: the level-1 upgrade cannot
move its frozen-set values. What remains open is the general question — where
q wakes up across roots, and whether modeling the partner's response *shrinks*
fixed-pair hardness H. Three roots at one epoch pair is orientation; the
h8-t4 root's near-1 exposure shows the other end of the range. Queued as §22
step 9. Era page: [walt-calculated-evidence](walt-calculated-evidence.md). Two
tilt-audit roads are also still untested: **counted-boundary** (Phase F
predicate mining never run) and **policy-library** (`walt/TILT-AUDIT.md`
§ "Road verdict").

**Movement (slice 3, 2026-08-24) — and three questions it opened.** The Part
VI build (PANEL-A7/A8) added the cancellation ladder, pairwise masses and
directional upper rungs, so the field question now has vocabulary that keeps
behavioral, outcome and value irrelevance apart. It left three genuinely open
design questions, deferred loudly rather than approximated (carded as
[[slice3-deferred-producers]]; probe README owns the statements,
`walt/probes/fieldswap_cancel/README.md`): (i) **a δ-valid admissible-upper
E3** — the sampled route into a screen needs a valid upper bound on a
*supremum*, and a sup is not a mean, so the evidence engine has no ready
instrument for it; (ii) **a valid-bound route to `Dominated`** — PANEL-A7
admits one, but only the exact-enumeration producer exists, and a bound type
without a producer would invite misuse; (iii) **the §10 motif tags** — the
structural motif vocabulary (e.g. "reveal-response") is absent, not
approximated, and needs a design pass before any tagged aggregate means
anything.

**Movement (x:024, 2026-08-25) — all three answered at design level.** Pro's
response to the deferred-producers dispatch was adjudicated same-day (rulings
**TRIPLE-A1..A7**, `walt/CENSUS-RULINGS.md`; intake
`walt/math/response_deferred_producers_triple_v0.1_intake.md`; verifier 13/13
PASS as session evidence): (i) resolves as the **max-preserving upper CS** —
covering one fixed true maximizer suffices, so the branchwise-max endpoint
covers the supremum at the *same* δ with no Bonferroni split, and endpoint
monotonicity collapses the family to the empirical-optimum count the shipped
solver already produces (the sup *is* a one-mean problem, for a policy you
never need to name); (ii) resolves as the **Hazard-Exclusion Invariant** —
sound and semantically complete, one general verifier as the single authority,
cheap pattern producers (one-round trump extraction first) emitting witnesses
for it; (iii) resolves as a **six-motif first-split morphology + Other** with
mandatory orthogonal flags, partitioning correction mass only —
`RevealResponse` stays refused pending raw suffix enrichment.

**Movement (slices 4a/4b/4c, 2026-08-25, same night) — all three producers
BUILT with gates.** PRs #45/#46/#44, main `cbce1ae`, central gates green:
`solver::upper_cs` (the E3 admissible-upper + fused-directional variants),
`solver::hazard` (the invariant verifier as single authority + one-round
trump-extraction producer; first `Dominated` via the valid-bound route;
0/40 wild accepts — honest narrowness), `solver::motif` (classifier +
suffix enrichment closing item 11; 453/453 classified, residual 0). The
card [[slice3-deferred-producers]] closed on its done-when. Question 10's
three open design questions are now **answered and built**; what remains
open here is the follow-on tightening (an exact one-policy directional E3
solve; richer witness languages beyond the two-trick v1; the
`PartnerResponseCandidate` second layer over the now-persisted suffixes).

**Movement (step 9, 2026-08-25) — the detection layer ran; the wake-up
question has its first corpus-level data.** PR #49, `solver::wakeup` +
`walt/probes/step9/` (exploratory tier; the probe README owns the numbers).
The headline shape, on the predeclared corpus at one declared epoch pair:
the general wake-up is real but lives in the **value and decision channels,
not the response channel** — exact value wake 18/18 and decision wake 8/18
(five outright winner flips), while pivotal mass *drops* under σ1 on 13/18
pairs. Receipt h4's three pairs all reach **q₁ = 0 exactly** (lawful
enumeration-route exact-zeros): its level-0 disagreement structure was pure
field artifact — h7-t5's slice-2 negative answer now has company in the
opposite direction of the naive expectation. h7 (6-2 v 6-3) exhibits §14.4's
separation in the wild (dq = 0 with value+decision wake). On the
count-timing family the σ1 leg **newly-settles 2/6 decisions the σ0 leg
leaves fogged**, and 𝓘 runs field1-higher 5/6 — the "level 2 makes
decisions easier to sample" hypothesis's direction on exactly its motivating
family, while the exact route shows the opposite 𝓘 sign on 12/18 receipt
pairs. Family-specific, not a law. The question's remaining open form: does
the count-timing 𝓘 direction generalize across the near-tie regime, and
what does the partner-response channel look like at roots where the modeled
*non-bidder* seats carry the split mass?

**Movement (the targeted controller, 2026-08-25, same day):** the consumer
landed — `solver::targeted` (PR #51) assembles rungs, screen, and
survivor-only σ1 work into the per-root pay-only-where-it-matters pipeline,
with rung spend itself schedule-controlled (exact-E4 escalation refused
`provably-useless` when the lower-witness admissible set proves it cannot
prune). The follow-on list here grows by two: **directional confinement of
Stage-4 spend** (the PANEL-A8 directional screen is computed and reported
but does not yet confine which survivors get σ1 work), and the standing
caveat that `delta_frozen_baseline` (the `DeltaFrozenSet` Stage-1 producer,
the CE one-mean inversion at both endpoints) is build-level plumbing — any
future *theorem-tier* use of its two-sided interval requires its own
mathematical intake first.

### 11. The x:018 conditional-moment gap

**Status: named in correspondence, awaiting Pro's reply.** The fee-correlation
correspondence (x:018, 2026-08-14) asks what object carries the lower-witness
burden when the fee route is structurally unavailable (wide ties) — a
covering/fractional-covering dual over the core hypergraph with the fee as the
rank-one case? — and names the **conditional-moment gap blocking trick 1**. No
note has been received; nothing is adjudicated; the ask itself is indexed on
[received artifacts and intakes](walt-math-intakes.md).

### 12. Scenario-era conjectures awaiting their probes

**Status: conjectural, so labeled at the source, never quotable as results.**
Three from the pmake advisory ruling and the SP audit, each carried with its
own fence: the **belief-state growth extrapolation** (10⁷–10⁹ belief states at
mid boundaries — "conjecture only", from one measured 34× per-trick ratio; a
P-A21-shaped caution applies); the **pairing-wins hypothesis** (SP-A5: that
Cov(u_a, u_b) > 0 in practice is *a hypothesis the tilt audit itself
measures*, not a theorem); and the **§12.6A cross-carrier hope** (the
equivariant quotient "earns its keep across carriers and in late endgames" —
proved nearly trivial on the one carrier checked).

### 13. Exposure tightening at early tricks

**Status: still open — the rungs now exist and are measured, and the early-trick
half is untouched (annotated 2026-08-24 after slice 2).** At the driven trick-1
root the field-swap smoke's fixed-policy exposure is ≈ 1 — the richer field
wakes up almost everywhere, so the bound degenerates to the naive survivor set
(an honest, expected degeneracy per the level-2 parent's §8.1). The E0–E2
exposure rungs (exact equality → structural cover → clairvoyant reach) exist to
tighten this.

**Movement:** slice 2 **built and measured** the rungs, plus the exact
split-reach route E4, verifying the ladder E1 ≥ E2 ≥ E4 = R_a ≥ d_ρ with exact
rationals. The result splits by regime rather than settling the question. At
the **late** roots the rungs can be decisive: E0 fires on receipt-h7-t5 (`R_a`
= 0 exactly, cheapest possible screen input) and h4-t6 yields the first
pruning singleton. At the **split-heavy** root h8-t4 (trick 4) the exact E4
bound is still between 14/15 and 197/200, the screen prunes nothing, and the
rungs cost about half the naive σ1 pass — the screen does not yet earn its
keep, which is the parent's §17.2 falsifier direction.

So the sharpened open question is the original one, minus the "unbuilt"
excuse: **no rung has been run at tricks 1–3 at all**, where the shadow
instrument shows everything Unresolved and where a useful
`RootActionExposureUpper` would matter most — and the one trick-4 data point
suggests exposure grows toward 1 going backward. This remains the era's
central open cost question. Instrument records:
[walt-calculated-evidence](walt-calculated-evidence.md).

**Movement (step 9, 2026-08-25):** the sampled route's mechanism notes add
trick-1–2 data in the same direction, at fixed-policy tier: on the
count-timing family 255–256/256 worlds reach the field-disagreement
frontier, with first splits concentrated at the modeled bidder in tricks
1–2. That is frontier reachability under frozen pairs — *not* a rung run,
and no `RootActionExposureUpper` has yet been computed at tricks 1–3 — so
the open question stands unchanged, with one more point of evidence that
early-trick screens will not prune and the targeted controller must lean on
directional bounds and the sampled E3 route there.

**Movement (the targeted controller, 2026-08-25):** the *cost* half of this
question changed shape. The §17.2 falsifier direction — rungs costing half
a σ1 pass while pruning nothing — is answered **by schedule** rather than
by a tighter bound: on the first controller corpus exact E4 was never paid
at all (h4-t6 pruned to its singleton at the E2 screen; the h8-t4 no-prune
is now a cheap steering proof followed by a skip; the count-timing E3 walks
are refused as provably useless because the zero-hypothetical shows no
bound could prune). What remains open is unchanged and now isolated: a rung
that actually *tightens* at tricks 1–3 — where exposure runs at 1, the
screen cannot prune, and the controller's honest mode is δ-intervals and
typed refusals — does not exist. The question is now purely about better
mathematics there, not about wasted spend.

### 14. The per-epoch σ0 declaration

**Status: half-answered by declaration; the composition half is still open
(annotated 2026-08-24 after slice 2).** Every evidence record is model-relative
to a declared field model (the fieldswap smoke's σ0 = banked-correct level-0 at
n0 = 8; a different schedule is a different `FieldId` and a different
experiment), and candidate-set mutation starts a new epoch (CE §5.3) while
policy mutation invalidates evidence (§12.5).

**Movement:** slice 2 resolved the *local* half **by declaration** — **one
(σ0, σ1) pair per experiment epoch**, with both FieldIds riding every record,
and the probe epoch's pair written down explicitly (σ0 = `Level0{n0 = 8}`,
σ1 = `Level1{n_outer = 4, n0 = 2}`, frozen candidates `[8, 2]`; the tests
declare their own cheaper pair, likewise carried by its FieldIds). That is a
convention the instrument now enforces, not a ruling on composition.

What is **still not ruled** is the composed discipline: what happens to
accumulated evidence and the risk ledger when the declared field model itself
changes across epochs of one run. Until ruled, the safe reading stays the
strict one — a σ0 change is a new experiment, composing nothing.

**Movement (step 9, 2026-08-25):** the local convention was exercised at a
second declared epoch pair (σ0 = `Level0{n0=2}`, σ1 = `Level1{n_outer=4,
n0=2}`, both FieldIds on every record) — and step 9 shows the sound way to
*consume* across experiments without composing: its σ0 was chosen equal to
step 8's evaluation field, and the bin **asserts** the σ0 leg reproduces
step 8's recorded exact wins rather than importing them. Reproduction under
an identical declared field is not cross-epoch composition; the composition
half of this question remains unruled and the strict reading stands.

### 15. The cycle tripwire — adopted, armed by ruling, never run

**Status: pending by construction.** L2-A7 adopted the cycle discipline
(recurrence claims typed root / behavioral / local exact / global exact,
never promoted across those lines) and made the §13.5 tripwire — compare σ₁
vs σ₂ on the field-sensitive anchor corpus — a **standing precondition on
any broad level-3 work**. No tripwire run exists, and **no damping, mixtures,
or robust-cycle policies may be introduced without a separate mathematical
intake**. Recorded here so nobody builds level 3 first and looks for the
tripwire later.

*Annotation (2026-08-24, after slice 2):* the anchor corpus the tripwire needs
now has its first entries — the screen classifies roots, and h8-t4 came back
`FieldSensitive` 4/4 while h4-t6 came back `FieldStableExactFrozenSet`. Three
roots at one epoch pair is not a corpus, and h7-t5's `FieldSensitive` 3/3 is
an exact three-way tie at V₀ = 0 rather than value sensitivity — so the
precondition is nearer, not met, and the tripwire remains unrun.

*Annotation (2026-08-25, after step 9):* the corpus half moved materially —
10 roots / 24 pairs at one declared epoch pair now carry **typed wake
labels** (8/18 exact decision wakes including five outright winner flips;
2/6 sampled newly-settled), which is a real field-sensitive anchor corpus
in the making rather than three screen verdicts. Two gaps keep the
precondition unmet: it is still **one epoch pair**, and — the now-binding
one — the tripwire compares σ₁ against **σ₂**, and no level-2 field
machinery exists to supply the σ₂ leg. The tripwire stays unrun and level-3
work stays fenced; the precondition's remaining weight has shifted from
"grow the corpus" to "a σ₂ exists to compare."

---

## Two things that are settled and might look open

Recording these prevents a successor from re-opening closed questions.

- **The rank question is not open.** R-A16 and the v0.6 audit are explicit: for
  the distribution contracts the answer is |X|, by Lemma R(c). Those rows are
  **theorem rows, not measurements**, and a run returning anything else is a
  stop-and-report bug because the lemma says what it must return.
- **The magic-sample-count question is not open, it is answered by design
  (2026-08-24).** "How many worlds is enough" was the [[adaptive-sampling-intake]]
  card's question; the calculated-evidence intake answers it structurally —
  the required work is *calculated* from declared risk, the contender count,
  observed evidence, pivotal mass, and tilt (`𝓘 = q·D_{1/2}(τ)`), with
  monotone escalation to exactness — and CE-A5 removed fixed counts from the
  correctness path. What remains is **build and proof debt** (O20–O28), not an
  open mathematical question; a run returning to a magic n on the correctness
  path is a regression, not a choice. Step 8 (landed 2026-08-24) closed the
  demonstration half: on one epoch and one common stream at caps 40/160/640,
  the cap-ladder law holds mechanically and the historical 40-vs-160 flip
  comes back as an honest `Unresolved` near-tie at every cap — the flip was
  the magic count, not the game.
- **The exchangeability question is not open, it is repaired.** Parent §7.1 is
  unsound as written, and the repair is exact:
  [Theorem E1](walt-math-decision-sparse.md#1-order-exchange), generalised by
  E1′. What is open is only whether the general (H1)–(H3) form ever finds an
  instance beyond the J-1 transposition — and DS-A25 notes that
  [Corollary R-fold](walt-math-structure-transport.md#corollary-r-fold--the-predictive-dimension-is-declaration-fold-invariant)
  is the branch's only exhibited value-order isomorphism to date.
