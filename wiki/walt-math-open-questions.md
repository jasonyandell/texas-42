# walt mathematics — open questions

[Home](Home.md) · owns: the inventory of genuinely open mathematical questions
in the walt branch, at the exploratory tier — each with the ruling or report
that left it open and the measurement that last moved it · Sources:
`walt/CENSUS-RULINGS.md` (the ruling that left each one open — J-A8, S-A20,
R-A/P-A21, DS-A11/A13/A28, EC-A13, FT-A17, L2-A7, CE-A6, MB-A3, SC-A4,
FH-A2..A11), `walt/math/decision_sparse_exact_solving_v0.1_errata.md`,
`walt/SCENARIO-PLAYER.md` §10, `walt/LEVEL2-PROBE.md`,
`walt/math/calculated_evidence_v0.1.md` and
`walt/math/targeted_level2_field_stability_v0.1.md` (items 13–15),
`exchange/README.md` (x:018); for items 16–28 the reports and records of
2026-09-01 → 09-07: `walt/briefs/MORNING-2026-09-05.md`,
`walt/briefs/FH-RESPONSE-TO-PRO.md` (the three questions put to Pro, quoted
verbatim), `walt/briefs/{MB1,U0,U0B,UP0,FH3}-REPORT.md`, `walt/briefs/FH4-AUDIT.md`,
`walt/MAP.md` ("Next, in order"), `walt/DISCREPANCIES.md`, the records under
`walt/probes/factor_belief/`, `walt/probes/gran/`, `walt/probes/m3/`, the
branch-only records `walt-g1-l2:walt/probes/gran/level2_{g1,g2,lock}.txt` (tip
`6abdd78f`) and `walt-o5:walt/probes/o5/README.md` (tip `2981e090`),
`experiments/partnership/campaigns/{foundation-battery,default-partner-battery}/RESULTS.md`,
`experiments/partnership/packet/` (the three unintaken notes),
`kanban/backlog/{lean-catchup,ladder-policy-store,gran-anchor-reconstruction}.md`.
Repository state as of 2026-09-07 (`c00717d1`); checks made for this page are
labelled "measured 2026-09-13 on this machine".
Related: [the reference map](walt-math-reference.md) (Chapter 12 points here),
[received artifacts and intakes](walt-math-intakes.md),
[decision-deadness](walt-math-deadness.md),
[structure and transport](walt-math-structure-transport.md),
[decision-sparse witnesses](walt-math-decision-sparse.md),
[the freeze register](walt-math-freezes.md),
[the focal-horizon era](walt-focal-horizon-era.md),
[the Gran anchors](walt-gran-anchors.md),
[the partnership program](walt-partnership-program.md),
[negative results](walt-negative-results.md).

> **Coverage.** Items 1–8 are the census/decision-sparse era's inventory
> (2026-08-10 → 08-14); items 9–12 the scenario-player era's; items 13–15 the
> calculated-evidence era's (the 2026-08-24/25 [[math-reorg]] pass and its
> same-day extension). This rewrite (2026-09-13, against the repository at
> `c00717d1`) keeps every item still open as stated, marks what has been
> **ANSWERED** or **CORRECTED** with the measurement and its date, adds the
> movement of 2026-08-30 → 09-07 to items 4, 9, 10, 12, 13 and 15, and adds
> items 16–28 for the questions the model-belief, focal-horizon, Gran/O5 and
> partnership programs opened. Era narratives and instrument records live on
> the era pages; this page states the question, the ruling that fixed its
> shape, and where the last number came from.

> **Tier: EXPLORATORY throughout**, below every tier on
> [Home](Home.md#evidentiary-tiers--never-promoted-never-blurred). A number on
> this page is a probe record unless the sentence names the gate file that
> pins it; a green gate is evidence, never a status change.

## The inventory at a glance

| # | Question | Status at `c00717d1` | Last movement | Owning page |
|---|---|---|---|---|
| 1 | The specimens' tie mechanism | OPEN — deliberately unfilled | J-A8, 2026-08-12 | [deadness](walt-math-deadness.md) |
| 2 | A coarser lawful seat equivalence reaching 10⁵ | OPEN — out of scope by S-A20 | 2026-08-11 | [structure and transport](walt-math-structure-transport.md) |
| 3 | Is `dim V^val` small anywhere that matters | OPEN beyond grade 3; negative at grade 3 | R-A, 2026-08-12 | [information geometry](walt-math-information-geometry.md) |
| 4 | Moment compilation | OPEN; one unadjudicated proposal aimed at it | 2026-09-05 (received note, no intake) | [decision-sparse](walt-math-decision-sparse.md) |
| 5 | The economy claim | SPLIT — primal half tested by design, full half untested | EC-A13, 2026-08-13; **freeze 38 filled 2026-08-14** | this page |
| 6 | The reserved freezes | **CORRECTED** — 39 and 40 reserved; 38 FILLED at FT-A17 | 2026-08-14 | [freeze register](walt-math-freezes.md) |
| 7 | δ-similarity | Future mathematics, no definition yet | PG-A, 2026-08-12 | this page |
| 8 | The count-and-score lift | Not lawfully designable from current documents | DS-A12, E-A2 | this page |
| 9 | The obligations ledgers | OPEN by construction; four newer ledgers exist in no file; O5 measured | 2026-09-05 | [intakes](walt-math-intakes.md) |
| 10 | The level-2 field-swap question | OPEN in general; first hard prices and table numbers | MB1 2026-09-02; partnership 2026-09-06 | [calculated evidence](walt-calculated-evidence.md), [focal-horizon era](walt-focal-horizon-era.md) |
| 11 | The x:018 conditional-moment gap | Awaiting Pro's reply since 2026-08-14 | — | [intakes](walt-math-intakes.md) |
| 12 | Scenario-era conjectures | Belief-state growth: **ANSWERED in the other direction** (h8-t3 solved exactly); the other two unchanged | 2026-09-03 | [focal-horizon era](walt-focal-horizon-era.md) |
| 13 | Exposure tightening at early tricks | **RESHAPED** — at k ≥ 1 the width is the tail's policy gap | FH3, 2026-09-04 | [focal-horizon era](walt-focal-horizon-era.md) |
| 14 | The per-epoch σ0 declaration | Half-answered by declaration; composition unruled | 2026-08-25 | [calculated evidence](walt-calculated-evidence.md) |
| 15 | The cycle tripwire | Pending by construction — no σ₂ exists (F₂ unbuilt, MB-A3) | 2026-09-01 | [seat play](walt-seat-play.md) |
| 16 | What a pmake seat should do at exact indifference | **DESIGN CALL pending** (Jason's call (C)) | 2026-09-05 | [Gran anchors](walt-gran-anchors.md) |
| 17 | Whether void-aware inner belief pays at the table (O5) | UNRESOLVED — non-composing epochs point opposite ways | 2026-09-05 / 09-06 | [Gran anchors](walt-gran-anchors.md), [partnership](walt-partnership-program.md) |
| 18 | σ0's sufficient statistic — the read-key study | OPEN, not started (Pro question 2) | named 2026-09-04 | [focal-horizon era](walt-focal-horizon-era.md) |
| 19 | The tail question — a tail-improvement ladder | OPEN (Pro question 1) | FH3, 2026-09-04 | [focal-horizon era](walt-focal-horizon-era.md) |
| 20 | The honest guarantee of the live decision at tricks 1–3 | OPEN (Pro question 3) | 2026-09-04 | [seat play](walt-seat-play.md) |
| 21 | The fusion-free-suffix hypothesis | EMPIRICAL by ruling; trick 5 at receipt roots, not inside trick-4 solves | U0b, 2026-09-03 | [focal-horizon era](walt-focal-horizon-era.md) |
| 22 | The (b)/(c) ordering in the unified player | Jason's call; two flip specimens pinned | UP0, 2026-09-02 | [focal-horizon era](walt-focal-horizon-era.md) |
| 23 | The divergence referee protocol | Never run | defined 2026-08-18 | [seat play](walt-seat-play.md) |
| 24 | G2/G3's exact deal, and G4 | Unavailable without the plunge seed; G4 does not exist | 2026-09-05 | [Gran anchors](walt-gran-anchors.md) |
| 25 | Record debts — errata §9/§4.3; freeze 58 register-only | Unfiled since 2026-08-14; unnamed since 2026-08-31 | verified 2026-09-13 | [reference map](walt-math-reference.md) |
| 26 | Lean catch-up after the GT1 tree | Nothing after GT1 has kernel coverage; ledgers in no file | card opened 2026-08-24 | [lean](lean.md) |
| 27 | The three unintaken packet notes | Received, hashed, never intaken | 2026-09-05 / 09-07 | [intakes](walt-math-intakes.md) §9 |
| 28 | The ladder's memory — which sink dominates | Unmeasured; 19.4 GB at h8-t3 | FH4 N8, 2026-09-04 | [focal-horizon era](walt-focal-horizon-era.md) |

**Where else these are listed.** [open-problems](open-problems.md) carries,
since 2026-09-12, an *exploratory-tier pointer block* naming several of items
16–28 for readers of the ledgers; that block is a map, this page is the
statement. Neither cites the other upward. Where a number here and a record
disagree, the record wins and the disagreement is a bug to report.

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

**Movement (2026-09-05) — an unadjudicated proposal aimed squarely at this
question.** The partnership launch packet carries a received note,
*TEXAS42-IMPROVISATION-v0.1* (`experiments/partnership/packet/texas42-partnership-launch-v0.1/math/TEXAS42-IMPROVISATION-v0.1.md`,
SHA-256 `867db550…`, 528 lines; the packet's `MANIFEST.sha256` verifies 6/6 OK,
measured 2026-09-13 on this machine), which proposes a capacity-saturated
inclusion–exclusion **exact normalizer** for the posterior (its §4) and exact
moments of separable structural features (§11), and reports in-text that the
normalizer reproduces all 116,280 acting-hand completion weights against an
independent enumeration of all 399,072,960 legal ordered deals (uniform
normalizer 399,072,960; a hash-weighted test normalizer 1,345,725,937). It has
**no intake companion, no ruling family and no wiki index beyond
[walt-math-intakes](walt-math-intakes.md) §9**; none of its checks is filed in
the packet; it uses the word "certificate". Until intaken it is received text
below even the intaken corpus, and this question's status does not move. Two
cautions for whoever intakes it: the same posterior is already *represented*
exactly by CBS Theorem 20.1's seat-factor closure
([walt-counted-belief-era](walt-counted-belief-era.md)) — a representation,
which is the E5.2 escape hatch, not a compression of `dim V^val` and not a
circuit-size-against-world-count measurement of the DS-A11 shape — and
`walt/scheme/INFORMATION-PRICES.md` already builds on the note's ideas without
citing an adjudication (item 27).

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
and the question is whether the interval still closes (the Theorem E6.3
object; "sandwich" is that theorem's adjudicated name and not an object name,
CBS-A3). The transport clause it
needed is now open — freeze 36 v2 (EC-A8) admits the declaration fold, with
values licensed by
[Corollary S-fold-val](walt-math-structure-transport.md#lemma-s-fold--the-seven-pip-declarations-fold-exactly-71)
and verdict transport by Lemma E7. Note this is a statement about the *design*;
results are a separate matter.

**The full claim — untested, and a different object.** A solver that avoids
exact solves additionally requires the **U side cheapened**: a relaxation coarser
than treatment C, run down Theorem E6.5's ladder. That is Experiment D's
territory, and nothing has touched it.

*Correction (2026-09-13; the fact is from 2026-08-14).* EC-A13's phrase "with
freeze 38 still reserved" was true on 2026-08-13 and false a day later:
**freeze 38 was FILLED at FT-A17 (2026-08-14, v1, scoped — the cut language,
the canonical family, the validity-proof obligation and the cut ordering, all
supplied by the x:016 note; a v1.1(d) clarification at SR-A21, v1 not amended,
v2 not opened)**. The cut language Experiment D needs therefore *exists*. What
blocks Experiment D now is design, not a reserved number: no adaptive-gluing
design is filed and Theorem E6.5's two obligations are discharged by nothing.
The earlier text of this item and of item 6 said the freeze blocked it — the
contradiction with [the freeze register](walt-math-freezes.md) that the
2026-09-07 audit found. The full claim remains untested; the primal/full
split of EC-A13 is unchanged.

**Binding on how this is written up (EC-A13):** a results file or wiki sentence
saying "the economy claim was tested" **without the word "primal" has
over-claimed**. The two halves are never collapsed into one sentence.

### 6. The reserved freezes — 39 and 40 (38 is filled)

**Status: two numbers reserved and untouched, each marking a
designed-but-unbuilt object; one former reservation discharged.** *Corrected
2026-09-13:* the earlier text of this item listed 38 among the reserved
freezes. The register ([walt-math-freezes](walt-math-freezes.md)) and the
rulings file agree it is not, and have since 2026-08-14.

- **38 — FILLED at FT-A17 (2026-08-14), v1, scoped.** DS-A13 reserved it for
  "the gluing-cut language, the validity-proof obligation, and the cut
  ordering"; SEP-A18, T1-A11, LD-A9 and RW-A8 each confirmed it reserved; the
  x:016 note supplied all three correctly typed and the number was spent. A cut
  is a declared partition of the latent world set of one focal information
  state, an identification of action variables that removes no world; the
  reveal-delay ladder `C^(k)` is the canonical family. What it *does not* do is
  build Experiment D: the freeze is a language, and the adaptive-gluing design
  that would use it has still not been filed (item 5).
- **39** — the circuit representation and its evaluation order. Blocks the
  moment-compilation experiment above (item 4); still reserved.
- **40** — the reachable-belief family defining W_reach, with its deal-level
  typing. What remains before it can run is an **enumeration of the reachable
  posteriors on a carrier small enough to enumerate exactly** (DS-A23). The
  retyping itself is done: [Definition E9](walt-math-information-geometry.md#definition-e9--interface-local-reachable-decision-width).
  Still reserved.

The register's one-line state as of 2026-09-07 (`c00717d1`): **58 numbers
issued, 56 spent, 39 and 40 reserved**; no freeze 59 exists anywhere; the
focal-horizon program of 2026-09-04 issued no freeze (its ladder identity and
forced-node convention are declared identity coordinates, not freezes). Freeze
58 — the RefineV1 semantic freeze of 2026-08-31 — is issued by the register at
APS-A9 and is named nowhere in `walt/CENSUS-RULINGS.md` (item 25).

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

**Movement (2026-08-30 → 2026-09-05) — four more ledgers, and the first O-row
with a measurement.** Four further obligation ledgers were accepted by ruling
into what the rulings call the Lean side-project ledger: **CBS-O1..O15**
(CBS-A9, 2026-08-30), **PS-T1..T15** plus the 42-instance layer (APS-A9,
2026-08-31), **MB-O1..O20 / MB-I1..I10** (MB-A8, 2026-09-01) and the salvation
complex's §60 tranche, indexed as **SC-O1..O16** on
[walt-math-intakes](walt-math-intakes.md) (SC-A2, 2026-09-01). **That ledger
exists in no file**: the identifiers occur in `walt/CENSUS-RULINGS.md`, the
CBS parent and companion, `walt/FACTOR-BELIEF.md`, `solver/factor_belief.rs`,
and on the pages that record the same absence (the intakes page,
[lean](lean.md) §10, `kanban/backlog/lean-catchup.md`); nothing under `lean/`
names them, and the catch-up card (opened 2026-08-24) is still backlog
(grep `CBS-O1`, measured 2026-09-13 on this machine). Where the ledger lives, and whether that
card owns it, is item 26. On the scenario-player ledger itself no row is marked
discharged, and exactly one row now has a measurement: **O5**, the cost of the
no-void inner simplification, run on 2026-09-05 on branch `walt-o5` (unmerged
at `c00717d1`; record `walt/probes/o5/README.md` at tip `2981e090`, probe
record, not gate-pinned). Its answers in order of confidence: the dead fraction
of the modeled minds' belief is exact and large (over 392 decision points the
median decision at tricks 3–6 has a third of its modeled belief refuted by the
record, the worst 988 of 1000); the σ0 flip rate over 243 unforced points
*grows* with n0 (102‰ at n0 = 2, 189‰ at n0 = 8, 164‰ at n0 = 32 — a bias
floor, not variance); the live level-1 player's recommendation moves at
90–454‰ of decisions by trick, through two channels (record-proved voids, and
voids proved *inside* the modeled continuation); and whether it plays better is
unresolved (item 17).

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

**Movement (MB-A3, 2026-09-01; MB1, 2026-09-02; UP0, 2026-09-02; the
partnership program, 2026-09-06) — the vocabulary is fixed by ruling and the
question has its first hard prices and its first table numbers.**

(i) **The rung registration** (MB-A3, corrected at intake — the first reading
"Dice = σ0" was wrong and retracted): `D = FieldModel::Dice` (uniform dice,
frozen per-scenario draws); **`F₀ = BR(D) = σ0`**, the banked-correct level-0
modeled mind; **`F₁ = BR(F₀)`** = level-1 walt; **`F₂ = BR(F₁)` = the unbuilt
level-2 rung**, on Jason's word. Every "level-2" object in this branch is a
best response to a *named* σ1 (L2-A7, O36) — none of them is F₂, and no σ₂
exists (item 15).

(ii) **The model-fusion price is strictly positive at trick 4** (MB1,
`walt/briefs/MB1-REPORT.md`). With the field itself a hidden coordinate —
`Ξ = Ω × Θ`, one FactorBelief per type profile, exact integer priors, the
registered F₀/F₁ mixture at ν = (1/2, 1/2) per hidden seat — the price
`Φ_a = U^sep_a − Q_a` of treating the model separately is strictly positive on
all eight substantive trick-4 coordinates: h8-t4 (fiber 1,200) 47/9600,
**38/9600**, 90/9600, 58/9600 on 2-1 / 3-1 / 3-3 / 5-5; h3-t4 (11,550)
173/46200, 157/92400, 37/6600, 7/2200 — one to nine per mille — and exactly
zero at every trick-5/6 receipt coordinate (seven substantive zeros, seven
vacuous; Theorem 19.1's corollary: zero at one full-support belief implies zero
at every belief). **The 38/9600 at h8-t4 3-1 is gate-pinned** — gate M6 in
`walt/walt/tests/solver_model_belief_recursion.rs`, `M6_SPECIMEN =
(8_323, 8_361, 9_600)`, regenerated by re-running the gate, never hand-edited;
the other seven rows are probe record
(`walt/probes/factor_belief/modelbelief_recursion_run1.txt`). Two boundaries
travel with it: a trick-3 mixture coordinate (h8-t3) refused all five root
actions at the declared 7,000,000-read ceiling (31 min) — the mixture's
affordability wall — and MB1's own flag is that the next strict specimen comes
from earlier roots, more types, or types that disagree earlier, never from
other ν and never from deeper. UP0 added the library-side finding: the declared
F₀/F₁ library was **falsified nine times by the player's own play** (an
observed action outside the library's support), so a type library must
contain the player.

(iii) **At the table, no partner model has beaten level 1** — the results
files outrank prose here
([walt-partnership-program](walt-partnership-program.md)). The 400-game default
battery on 100 mirrored deals at bid 30 records **L2 Partner vs L1 =
14 / 14 / 72** (wins / losses / ties, first-named player; 50.0%) at about five
times L1's cost per move, and **L2 Partner-with-voids vs L2 Partner =
12 / 17 / 71** (47.5%) —
`experiments/partnership/campaigns/default-partner-battery/RESULTS.md` rows for
`01-level/MATCH.md` and `02-voids/MATCH.md` (2026-09-06). No strength gain is
established in either direction; L1 default remains the operating default;
every race/refine partner configuration crossed the >5% fallback gate. The
Gran level-2 runs (item 16) are the same object at two anchors: level 2 holds
the 6-4 at both legal G1 nodes (trick 3 exact over 17,640 deals, 5-2 at 654‰
vs 6-4 at 640‰) and moves the release margin at the synthetic lock.

The general question — where q wakes up across roots and whether modeling the
partner's response *shrinks* fixed-pair hardness H — is unchanged, and now has
a measured price on both sides of it.

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

**Movement (2026-09-03) — the belief-state growth extrapolation now has hard
data, and it points the other way.** The conjecture's basis was the
explicit-world ladder of 2026-08-17, whose t = 3 line died at about 600 s,
300M nodes and 162M memo entries (`walt/math/WALT-MATH-QUESTION-2026-08-17…`
§Q2). The counted-belief representation then solved that same trick-3 root
exactly with no world list anywhere: **h8-t3 (Z = 59,976 worlds, contract 30)
under σ0, `Q* = 28859/29988` (962‰), argmax 1-1, 289,407,472 field reads,
14 min 13 s** (`walt/briefs/U0B-REPORT.md`; record
`walt/probes/factor_belief/horizon_run1.txt`, census line for h8-t3 — the
report's 14 min 13 s is the standalone scout wall, the record line's own wall
field reads 797,430,769 µs for the census pass; reads are exact, wall is the
only approximate number). FH3 reproduced the value independently by the k = 3
collapse (`L = U = 28859/29988` at 1-1, `focal_run1.txt`) — a probe
reproduction: the h8-t3 anchor is PROBE-ONLY, the anchors gate
`walt/walt/tests/solver_focal_anchors.rs` covers the seven h8-t4/h4-t4
coordinates, and h8-t3's §41 law checks run inside `focalreport.rs` against
`Q_a` *cited* from `horizon_run1.txt` (FH4 audit N5). No file under
`walt/walt/tests/` contains `28859` (grep, measured 2026-09-13). So the counted recursion's cost at trick 3 is not a belief-state count
at all: it is field reads — 99% of them σ0's per-hand classification — and
memory (item 28). The conjecture keeps its label; it is no longer the reason
trick 3 is expensive, and it is not the reason tricks 1–2 are unaffordable
(item 20). The pairing-wins hypothesis and the §12.6A cross-carrier hope are
unchanged.

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

**Movement (the waking seat + speed campaign, 2026-08-25):** the cost is
now *quantified* at instrument tier, and the redundancy hypothesis is
spent. The waking seat's phase conviction (2 hands / 56 decisions,
[walt-calculated-evidence](walt-calculated-evidence.md)) puts 926‰ of all
decision compute in tricks 1–2 and 729‰ in the σ0 baseline evaluation
itself — the L2 machinery (wake check + escalation) is not where the
microseconds go. Four E-A15-lawful sharing levers were then built and
gated value-identical (visit reordering, field caching, the decided
cutoff, the bundled shared-tree evaluator), and their honest combined
effect on the convicted trick-1 regime is small constants (~4%, ~3%,
~1.04× — the probe READMEs own the numbers). The mechanical reading:
early-trick information states are (hand, full public record), so
distinct modeled-mind deliberations almost never recur — there is no
redundancy for a lawful lever to remove. What remains is exactly this
section's question (mathematics that tightens at tricks 1–3) or batching
the distinct solves (the GPU shape, on Jason's word); sharing is
exhausted.

**Movement (FH3, 2026-09-04) — the question is RESHAPED: at k ≥ 1 the
remaining width is the tail's policy gap, not the fusion price.** The
focal-horizon hierarchy ([walt-focal-horizon-era](walt-focal-horizon-era.md);
parent `walt/math/focal_horizon_sandwich_v0.1.md`, rulings FH-A1..A11 with the
delivered propositions FH-God/int/tie/cut/last) gives every root action an
interval `[L_{a,k}, U_{a,k}]` — one lawful tail policy evaluated exactly
below, the world-revealed continuation above, `k` focal decisions made exact,
collapsing to the exact `Q_a` at `k ≥ h_f`. On the report of record — 33
(root, contract) coordinates at k ≤ 3, `walt/probes/factor_belief/focal_run1.txt`,
anchors gated in `walt/walt/tests/solver_focal_anchors.rs`, the independent
FH4 audit PASS with one vocabulary BLOCK fixed — every live trick-4 coordinate
settles by k ≤ 2 (five at k = 0 with no search, six at k = 1 with certified
regret Γ₁ ≤ 45‰, three at the k = 2 collapse), and per action, once one focal
layer is explicit, **`U − Q` is 0–3‰ at trick 4 and 1–2‰ at trick 3, while
`Q − L` is 9–41‰ at trick 4 and 12–33‰ at trick 3** (the audit's corrected
per-action columns, N4; the report's first wording quoted the width column).
The two ply-cut flips U0b had reported (h8-t4 at bids 36/39) are upper-side
artifacts: k = 1 stays honestly Unresolved there exactly as FH-A8's law
requires, k = 2 settles the right action, and no coordinate anywhere certifies
a wrong one. **A better lawful tail buys more than a deeper search, everywhere
measured.**

That reshapes this item. The early-trick cost question is no longer "which
upper bound tightens at tricks 1–3"; it splits into the tail (item 19) and the
honest live guarantee at tricks 1–3 (item 20), with the fusion price a
second-order term at every depth measured. What has still never happened, and
is the residue of the original question: no exposure rung and no focal-horizon
interval has been computed at tricks 1–3 (h8-t3 is the shallowest coordinate
on any record), and the opening root is honest UNRESOLVED at ε = 1/4 (Phase 8:
play 6-5, floor 732‰, at most 267‰ unclaimed, the sampled tier plateaued at
p = 512 — `walt/probes/factor_belief/openingreport_run1.txt`) — with the split
of that 267‰ between information price and policy gap **UNKNOWN** by the
2026-09-03 correction (`walt/DISCREPANCIES.md`, first "Reconciled" entry:
a zero doom census moves only `d_phys`; U0 typed the opening `UnknownGodGap` on
all seven actions, SC-A4). Nothing at tricks 1–3 has the trick-4 pattern's
shape measured for it; extrapolating the pattern there is exactly what P-A21
forbids.

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

*Annotation (2026-09-01, MB-A3; 2026-09-05, the Gran runs):* the binding gap
is now a ruling rather than an observation — **no σ₂ exists because
`F₂ = BR(F₁)` is unbuilt** (MB-A3's registration table, item 10; built only on
Jason's word). The Gran level-2 records on branch `walt-g1-l2` (item 16) are
best-response-to-a-named-σ1 objects and supply no σ₂ leg either. Nothing else
changed: the tripwire stays unrun, level-3 work stays fenced, and the
precondition's weight is where 2026-08-25 left it.

---

### 16. What should a pmake seat do at exact indifference?

**Status: a design question with no design, no intake and no ruling — Jason's
call (C) in the 2026-09-05 readout, pending.** Owner of the story:
[walt-gran-anchors](walt-gran-anchors.md) §6; the levers were named in
`walt/briefs/MORNING-2026-09-05.md` item 4. The measurement that raised it
lives on branch `walt-g1-l2` (`walt/probes/gran/level2_g2.txt`, tip
`6abdd78f`, 8 commits not in main at `c00717d1`; probe record, not gate-pinned).

**The lock, in exact numbers.** The made Gran hand G2 (bid 31 on sixes,
36–0) is exactly locked from trick 3 at Gran's seat, over the complete
void-consistent support, at both model levels:

| trick | legal | support (deals) | level 2 | level-1 control |
|---|---|---|---|---|
| 2 | 6 | 36,036 | 800-world estimate: 5-0 / 5-5 tie at 1000 (sampled saturation); the exact row **died** at 7,595,251,011 nodes / 98,391,159 π evaluations in 560.558 s | **exact: 5-0 alone at 1/1**, others 984–994‰ — the tile Gran led |
| 3 | 5 | 6,930 | exact: 3-2 / 4-3 / 5-2 / 5-5 all at **1/1**; 3-3 at 996‰ | identical tie set |
| 4 | 4 | **280** | exact: **all four at 1/1** — the four tiles of the trick-4 screenshot panel ("every option 100% on 160 worlds"), sharpened to every deal of the support | identical |
| 5 | 3 | 60 | exact: all three at 1/1 | identical |
| 6 | 2 | 12 | exact: 5-5 at 1/1 alone; 5-2 at 3/4 | identical |

Trick 1 was forced (S0 led the 6-6, sixes trump, the 6-4 was S2's only six), so
G2 never asked a 6-4 question at all; the lock is the answer regardless of the
scenario. It is indifference *under one field model* — every deal the seat
considers possible makes the bid whatever she plays — and never a laydown
claim (∀σ; APS-A5).

**Why the objective cannot break it.** pmake is Boolean and pins at 1. At an
exact tie at 1 every option is optimal and the objective has no gradient; a
modeled partner's reading of the play reaches the decision only *through*
P(make), and P(make) cannot exceed 1 — so level 2 gives the identical tie set
and is worth nothing over level 1 here. Tie refinement (the 2026-08-17 ladder's
discipline for *sampled* saturation) cannot rescue it: these ties are exact
over the whole support and there are no worlds to add.

**What the seat then does — `TieRule::LowestTileIndex`, a source reading.**
Read on main at `c00717d1` and never executed at a tied node: `solver/act.rs`
declares `tie_rule: TieRule::LowestTileIndex` in the two constructors that fix
act's σ0 field identity (lines 288 and 315; `TieRule` is defined in
`solver/policy.rs` and is part of a policy's frozen identity); an honest exact
tie routes to `ActRoute::ExactTieLevel1`, a level-1 ranking among the tied
maxima at the fallback epoch 200/8; `solver::best_of` then replaces the
incumbent only on *strict* improvement, first-listed on residual ties, and
candidate order is legal-tile order ascending by domino index
(`index = hi·(hi+1)/2 + lo`; the 6-4 is **25** of 0..27, above it only 6-5 and
6-6). A tied seat therefore discards almost everything before the 6-4 —
count tiles are high-index by construction — and the same convention is
stack-wide (SCENARIO-PLAYER Def 3.2's modeled minds; `solver/selection.rs`'s
`Rule::Fixed`). No harness drives `solver::act` at an arbitrary constructed
information state, so the reading is unexecuted; G2's exact ties are the
natural specimen.

**The levers, and why another rung is not one of them.** From the readout and
the record: (a) the **objective** — P(make) goes flat exactly where the
partner's information problem is sharpest; a margin term or a
partner-information term would not; (b) an explicit **tie-break at exact
indifference** — for instance preferring to release count when tied. Neither
is a feature; neither has a design. The synthetic lock (`level2_lock.txt`,
constructed by hand, never G4) is the evidence *for* Jason's mechanism rather
than against it: the release margin (6-4 value minus best alternative) shifts
from level 1 to level 2 by +30 / −15 / +40 / +134 / +43 / 0 per mille at tricks
1–6; at trick 5, exact over all 210 deals at both levels, L1 holds the 6-4
(2-1 at 561‰) and L2 releases it (6-4 at 509‰ — one world of 210, a direction,
not a verdict); at trick 4, exact over all 1,260, the release line is the only
one that rises (556 → 596‰) while every hold line falls about 100‰. No node in
three constructions saturated, so the literal "100s at L1, not at L2" was
untestable (precondition failed, not the mechanism); the record's first
headline that saturation and the 6-4 mattering are "mutually exclusive" was
withdrawn in-record five minutes later as a single-model statement.

**What would close it.** A ruled objective for the saturated regime or a ruled
tie-break; a harness executing `act` at G2's tied nodes; then a measurement
against the level-1 seat on the same anchor at the screenshot's epoch (the
experiment that would attribute the G1 trick-1 flip, not yet run).

### 17. Does void-aware inner belief pay at the table? (obligation O5)

**Status: UNRESOLVED, and every measurement is non-composing.** Owner:
[walt-gran-anchors](walt-gran-anchors.md) §8 with
[walt-partnership-program](walt-partnership-program.md) §4. The defect O5
names: SCENARIO-PLAYER Def 4.3's modeled minds sample a **no-void** fiber the
public record has already refuted — a bias floor that does not shrink with n0
(item 9). The correctness case for the fix is strong; the play case is unmade.

| measurement | configuration | result (first-named first) | record |
|---|---|---|---|
| O5 mirrored match, **live epoch** | n_outer 50 / n0 8, 72 mirrored deals, shared outer seed | aware **33** / blind **19** / level **20** pairs; **+262 of 6,048** points to aware; both halves agree in sign | `walt-o5:walt/probes/o5/README.md` (`2981e090`) |
| O5 mirrored match, **reduced epoch** | n_outer 16 / n0 4, 192 mirrored deals | blind **83** / aware **76** / level **33**; **−226 of 16,128** for aware | same |
| foundation battery, random deals | `l1-race-voids` vs `l1-race`, 50 deals | **9 / 5 / 36** (54.0%) | `experiments/partnership/campaigns/foundation-battery/RESULTS.md`, `05-voids/MATCH.md` |
| foundation battery, focal hands | five focal hands × ten completions | **5 / 8 / 37** (47.0%) | same, `07-worlds/MATCH.md` |
| default battery | L2 Partner-with-voids vs L2 Partner, 100 mirrored deals | **12 / 17 / 71** (47.5%) | `…/default-partner-battery/RESULTS.md`, `02-voids/MATCH.md` |

The first O5 match was reported as a "dead heat"; the match seed lacked the
deal index, the run was repaired and rerun, and the verdict changed — **"dead
heat" is withdrawn**, and the two epochs point opposite ways with per-pair
margins centred on zero and tails past ±60 in both. The flag stays off; a
default swap is Jason's call (D). The rows do not compose: different epochs
(n_outer / n0), different deal sets, and **two implementations** — the branch's
`Level0Field::void_aware` with shuffle-and-reject `sample_belief` keyed by
`Key::voids`, and main's `InnerBelief::VoidsCounted` through the kernel's exact
counted sampler (`dbcc698f`, 2026-09-06) — whose draw streams differ, so no
row isolates logical void conditioning from a change of sample stream (a
coupled-world ablation is unbuilt). Jason's 2026-09-05 ruling: before the flag
can go default it carries a rank/unrank sampler over a counting DP (no
rejection loop, `N = |lawful fiber|` free at every node); main's `FiberDp`
route may already satisfy it and no record says so. The named next steps —
1000+ deals at *both* epochs, a sweep over n_outer, a third reference seat, an
auction instead of fixed P30 — have not run. A memo price exists at the live
epoch only (8× keys, 6× wall at n_outer 200 / n0 4; readout item 5).

### 18. σ0's sufficient statistic — the read-key study

**Status: open, not started; the next slice named in `walt/MAP.md` after the
focal-horizon PR and before consolidation; no card.** Pro's question 2, quoted
from `walt/briefs/FH-RESPONSE-TO-PRO.md` (draft for hand-ferry, 2026-09-04;
sent/unsent status unrecorded):

> **The field's sufficient statistic.** σ0 reads the full public record, so
> its cache is keyed by the full record and gets no reuse across histories,
> while within one history it is the entire cost. Which coordinates of the
> record does a level-0 modeled mind's decision actually depend on? If there
> is a small sufficient statistic, every recursion in the stack gets ten to a
> hundred times cheaper without changing a single value. Any theory of what
> that statistic must contain would be worth more to us right now than any
> new bound.

Why it is the lever: σ0's per-hand classification is 99% of every bill since
the first counting slice; the cross-history cache reuse is exactly zero because
the full §43 identity key carries the public history (`cache_run1.txt`, gate
`solver_factor_belief.rs::the_full_identity_key_shares_nothing_across_candidates_or_roots`);
a pass on a σ0 instance already warmed by an earlier pass over the same
coordinate runs 15× faster at identical reads (FH3). Why it is dangerous: a
coarser key is a *proven* state reduction or it is the PiKey defect again —
Remark 2.5 (banked is not a function of (played, leader, plays)) and the
path-dependence counterexample (same reduced key, odds 1:1 on one trick order
and 3:4 on the other). Under a uniform-random field the posterior is the
minimal sufficient statistic (the 2026-08-17 ruling §Q2); σ0 is not
uniform-random, and the question is what *its* decision depends on.

### 19. The tail question — is there a tail-improvement ladder?

**Status: open; the direction FH3 pointed the program in; fenced by the
consolidation ruling.** Pro's question 1, verbatim:

> **The tail.** Theorem 1 says any lawful policy is an admissible tail, and the
> data says the tail is the whole residual. What is the cheapest lawful tail
> that closes most of the policy gap — an extracted `π_k` from a settled
> neighbor state reused as the tail of the next decision? a tail improved by
> one focal layer and then frozen? Is there a monotone "tail-improvement
> ladder" on the lower side with the same economics as your upper-side gluing
> staircase, and does it converge to `Q` faster than `k` does?

What is fixed already: FH-A4 rules the primary tail for the report of record
to be σ0 driving the viewer seat (σ0-as-focal), `lowest_first` gate-only;
FH-A7 makes FH5 the lower-side fusion gate (a tail is one lawful policy,
re-priced; FH-int's discipline is that every lower fact carries the policy
attaining it). The specimen that shows how little a better tail needs to buy:
h4-t4 at bid 39 needs k = 1 only because the σ0 tail is worth 651‰ against
4-0's 655‰ upper — four per mille. The constraint: **no new mathematical parent
until the consolidation slice lands** (Jason, 2026-09-04: "follow through on
what we have, then invest in a simplification/unification attempt"), so this
is a question for after consolidation, or for Pro's instincts in reply.

### 20. The honest guarantee of the live decision at tricks 1–3

**Status: open — the question the whole stack exists to answer, put to Pro as
question 3.** Verbatim:

> **The live decision at tricks 1–3.** Exact is unaffordable there and will
> stay so. Is "tail + k = 0 or 1 interval + certified regret" the right live
> decision, and what is the honest statement of its guarantee to a player who
> wants to know how wrong walt might be on this trick?

What exists: the unified player's five-tier cascade (decided arithmetic →
endgame exact → middlegame mixture → certified regret → σ0 fallback) with
provenance on every decision (UP0/UP1a, gates `solver_unified.rs`,
`solver_unified_carry.rs`); certified regret `Γ = U* − B_exec`, whose guarantee
is model-relative — APS-A7: on the joint-validity event
`0 ≤ Q* − V(π̂) ≤ Γ`, and `Γ ≤ ε` certifies ε-optimality **under the declared
field and belief** — never a game-theoretic bound. What the numbers say at the
top of the game: the opening root's Γ is 267‰ at the p = 512 plateau with its
split unknown (item 13); the sampled route at tricks 1–3 is honestly
Unresolved almost everywhere (the step-7 shadow: tricks 1–3 all Unresolved,
33/33/29 of decisions); no focal-horizon interval exists shallower than
trick 3. A player asking "how wrong might walt be" today gets Γ under σ0 at
trick 4 and deeper, and an honest refusal earlier. The open half is the
statement itself — what a certified-regret figure under one modeled field
honestly promises a human at the table — and whether the letter's candidate
decision rule is the right one.

### 21. The fusion-free-suffix hypothesis

**Status: an EMPIRICAL object by ruling; measured on one axis; the cheapest
falsifier not run.** SC §37–38 conjectures that beyond some horizon physical
doom is the only unavoidable failure and one lawful policy realizes every
saveable world — the most consequential hypothesis of that round, since it is
what would make F₂ affordable (SC §46). **SC-A4** rules it a census target: a
theorem may be proposed only after adversarial counterexample search, and
`UnknownGodGap` is a distinct result type — zero certified doom with no exact
Q is never `PositiveGodGap`.

Measured (probe records; gates `solver_godgap.rs`, `solver_horizon.rs`):
U0 (`godgap_run1.txt`, 2026-09-02) found on the receipt-root corpus of 37
coordinates that **the earliest fusion-free depth is trick 5** — all 14
trick-5/6 coordinates God-tight (12 substantively, 2 whole-fiber doom), all 12
substantive trick-4 coordinates carrying an information price `Φ = d_info` of
6–22‰ (max 43/1925 at h3-t4 4-4) with `d_policy = 0` at every one, the opening
root `UnknownGodGap` on all seven actions. U0b (`horizon_run1.txt`, 2026-09-03)
then found that **inside a trick-4 solve the trick-5 frontier is NOT
fusion-free**: positive-price frontier nodes 40/466 (9%) at h8-t4, 171/779
(22%) at h3-t4, 384/1,228 (31%) at h4-t4 at bid 30, mass-weighted 13–14‰ at
the receipt contract and up to 105‰ of root value at higher contracts (h4-t4
at bid 39); even the trick-6 frontier at 0–7‰ flips the root play in 2 of 30
rows. So "fusion-free beyond trick 5" is true of the fourteen uniform receipt
roots and false of the trick-5 nodes a trick-4 solve actually reaches — the
horizon is a property of the node, not of the depth. The cheapest falsifier
U0 named — a corpus that varies the **contract** at receipt roots, same
machinery, no new mathematics — has not been run (U0b varied contracts only at
cut depth). In the hierarchy's vocabulary these censuses are `U_{a,0}` and
`U_{a,m−1}` (FH-cut) and are retired as instruments by the consolidation
slice; the hypothesis is untouched by that.

### 22. The (b)/(c) ordering in the unified player

**Status: Jason's call (or UP1's); recorded, not acted on.** UP0's cascade
keeps tier (b), `EndgameExact` — the world-space exact recursion under the
point mass `δ_{F₀}` — above tier (c), `MiddlegameMixture`, MB1's exact
model-space response under the carried belief. MB0's point-mass parity makes
(b) the `δ_{F₀}` special case of (c): "the same recursion under a strictly
narrower belief", so tier (b) is not deeper certainty than (c), and the
mathematics argues for inverting the order. Two specimens where the carried
posterior flips the argmax are gate-pinned
(`up3_both_argmax_flip_specimens_are_pinned`; the value-move specimen h3-t5
29/40 vs 4/5, `up3_the_value_move_specimen_is_pinned`); under a natural ladder
tier (c) never fires at all (a third "model" rung swaps the structural caps so
gate UP6 can verify a tier-(c) claim against an independent walk). The brief's
ordering was kept and the flip recorded (`walt/briefs/UP0-REPORT.md`, "What
UP1 needs" item 4). Unresolved at `c00717d1`. Adjacent and also not done: the
§49 proof state is a trick-START object (`ProofState::open` asserts an empty
partial trick), which cost tiers (b1)/(d) 63 of UP0's 104 refusals — additive
work in `proof_state.rs`.

### 23. The divergence referee protocol — never run

**Status: defined 2026-08-18, never run; needs Jason's sense of whether to
spend the compute.** The level-2 divergence mining
(`walt/probes/m3/divergence_results_2026-08-18.txt`: 900 self-played deals,
4,156 level-2-shadowed decisions, corpus under `walt/probes/m3/mined/`) is
**self-graded** — every "divergent" verdict is level 2's own value table, and
its header says which mind is *right* "needs the mirrored-replay referee, not
yet run". The protocol as written there: replay the divergent hands mirrored
(level 1 vs level 2 in the shadow chair, all else identical), McNemar on the
discordant pairs — all signal, no agreement-noise. Still not run at
`c00717d1`. Until it runs, no divergence number is evidence that level 2 is
better anywhere; the only level-2 strength numbers that exist are the
table results of item 10(iii), which say it is not. Owner:
[walt-seat-play](walt-seat-play.md) §4.

### 24. G2/G3's exact deal, and G4

**Status: unavailable from the artifact; waits on the plunge side.** The made
hand was committed as a validated six-trick partial (`walt/probes/gran/g2g3.receipt.txt`,
`granrun validate-partial`): the app ended the hand at 36 against 31, so 24 of
28 tiles are recorded; the unplayed four are `4-1 4-4 5-2 5-3`, one per seat;
the trick-4 panel pins S2's as the 5-2, so **S2's whole hand is known**
(`6-4 5-0 3-2 3-3 4-3 5-5 5-2`); the assignment of `{4-1, 4-4, 5-3}` to
S0/S1/S3 is **six-way ambiguous and mechanically undecidable** from the record
(none is a six or a blank; every failure to follow in the prefix was on sixes
or blanks). The G2 and G3 roots are therefore fully determined *as information
sets* — which is why item 16's lock could be computed — but no driven
whole-hand G2 run and no `replay_hand` validation is possible until the seed is
recovered; no G2/G3 decision run by the waking seat exists. **G4 — the game
Jason remembers, Gran locked at 100s while holding the 6-4 — was never
captured and does not exist as a record**; the synthetic lock is explicitly
never G4 (call (F): "G4 when the real game recurs"). The anchor card
(`kanban/backlog/gran-anchor-reconstruction.md`) keeps two open items: seed
provenance, and the intake companion's "Gran-anchor gap" pointer.

### 25. Record debts — the errata's §9 and §4.3, and freeze 58's number

**Status: two owed filings and one naming question, all administrative, none
blocking, all real.**

- **Errata §9 and §4.3.** DS-A28(ii) requires corrected mathematics to be
  filed in the maintained errata with a full statement and proof. Owed under
  that rule since 2026-08-14: **§9**, the FT/SR/FF/FC nonanticipativity objects
  (FT-A27(i)), and **§4.3**, Corollary E4.1 (SEP-A2). Neither is filed:
  `walt/math/decision_sparse_exact_solving_v0.1_errata.md`'s headings end at
  "## 8.6 Index addendum" with no §9 and no E4.1 (verified 2026-09-07 by the
  survey; re-checked 2026-09-13 on this machine). `CENSUS-RULINGS.md` is
  therefore the only authority for Lemmas FT-*, SR-*, FF-*, FC-* and Corollary
  E4.1 a month on. The question: is the errata still the intended home, or has
  the rulings file become their permanent authority by default? Either answer
  should be written down; the current state is neither.
- **Freeze 58's register-only issuance.** APS-A9 froze `solver/refine.rs`
  "semantically as the RefineV1 reference" without a number; the number 58 was
  issued by [the freeze register](walt-math-freezes.md) (2026-08-31 addendum)
  and is cited as "freeze 58" by `walt/FACTOR-BELIEF.md`, `walt/MAP.md`,
  `walt/LOG.md`, fourteen briefs and five solver source/test files (`grep -il
  'freeze 58'`, measured 2026-09-13 on this machine) — while a
  search for "freeze 58" over `walt/CENSUS-RULINGS.md` returns nothing
  (0 hits, measured 2026-09-13 on this machine). The question: intended, or
  should a one-line append in the rulings file name the number so the
  register is not the sole issuing authority?

### 26. Lean catch-up for everything after the GT1 tree

**Status: open; nothing mechanized after 2026-08-17; the obligation ledgers
that would scope it exist in no file.** Lean's coverage of walt is the GT1
trick-1 modules only — the `Texas42.Trick1PerfectRecallNet` proof boundary and
`Trick1MetalFoundation` — kernel-facing finite foundations, not implementation
refinement, and the M3 Lean tree has never been demonstrated to build
([walt-gpu-native-trick1](walt-gpu-native-trick1.md)). Everything after has no
kernel coverage: the signed-pivotal identities, pmake's P1–P4, CBS Theorems
2.1/9.1/20.1/23.1/30.1, the APS score layer and certified regret, MB Theorem
7.1, SC Theorems 7.1/12.1, FH Theorems 1–6 and the five delivered
propositions. The four ledgers into which the rulings accepted their
obligations — CBS-O1..O15, PS-T1..T15, MB-O1..O20 / MB-I1..I10, SC-O1..O16 —
are named in the rulings and on [walt-math-intakes](walt-math-intakes.md) and
nowhere under `lean/` (item 9). `kanban/backlog/lean-catchup.md` (opened
2026-08-24) has as its done-when "a triaged P1 list in `lean/` referencing the
math index" — not done. T1-A12's risk is inherited by all of it: every walt
statement is proved relative to walt's *implementation* of the rules, and no
receipt computed by that implementation can detect a disagreement with the
rules corpus. The graduation path is unchanged (item 9).

### 27. The three unintaken packet notes

**Status: received, hashed, consumed, never intaken.** Three mathematical notes
arrived inside the partnership experiment's packet, outside `walt/math/`, and
none has an intake companion, a ruling family or a scratch-tier verifier filed
under the walt convention (index: [walt-math-intakes](walt-math-intakes.md)
§9, which carries the full digests):

| note | date | lines | packet hash (first 8) | the word "certificate" | consumed by |
|---|---|---|---|---|---|
| *TEXAS42-UNIFIED-REVIEW-v0.1* — one response-vector core for belief, continuation, gluing, sampling, score and model-belief work; a semantic yes under explicit assumptions, cost "not established" | 2026-09-05 | 809 | `daae4332` | ≈10 occurrences | — ; names `verify_unification.py` and `verification_results.json`, **neither in the packet** (the SP-A11 unfiled-import precedent would apply at intake) |
| *TEXAS42-IMPROVISATION-v0.1* — the inclusion–exclusion exact normalizer and a centred-potential information-price upper (item 4) | 2026-09-05 | 528 | `867db550` | 4 | `walt/scheme/INFORMATION-PRICES.md`, without an adjudication |
| *PATH-TO-GENERALIZING-SCHEME-POLICIES-v0.1* — regret rather than teacher-label agreement as the training cost for a small lawful controller; information prices bound missed alternatives | 2026-09-07 (against main `08fad726`) | 404 | `deff51a3` (per `manifest.json`, with `verify.py`, `results.json`, `partner-count-candidate.scheme`) | 2 | `experiments/partnership/RELATIONAL-LEARNING.md` |

The launch packet's `MANIFEST.sha256` verifies 6/6 OK (measured 2026-09-13 on
this machine). The question is whether they should be intaken under the
`walt/math` convention at all — verbatim parent + `.sha256` + companion +
same-day rulings + obligations into a ledger — given that two of them are
already load-bearing for exploratory code, that their vocabulary collides with
D3, and that the "no new parent until consolidation" ruling of 2026-09-04
stands. Until that is decided they sit below even the intaken corpus, and the
book quotes nothing from them as a result.

### 28. The ladder's memory — which sink dominates?

**Status: unmeasured; the cost that grew fastest in the focal-horizon program;
carded.** The ladder stores a full policy table beside every node's lower fact
(the FH-int requirement that every lower carries its witness) by copying the
subtree's choice table into each ancestor's fact. At h8-t3 that is **3.82M
facts and 19.4 GB peak RSS** (FH3 record; standalone ladder 17.1 GB; the
anchors gate 17.8 GB by FH3's own figure, 18.22 GB as the FH4 audit measured
it, with five h4-t4 ladders in flight; capped to 8.8 GB after FH5); at h3-t4 the numbers are 662 MB memo-on, 509 MB memo-off, 411 MB for the
direct engine. The FH4 audit (N8) found **two sinks** — the fact store's
per-node policy tables (~98 MB at h3-t4) and the memo's `FactorBelief` clones
(~153 MB) — and rules "measure which dominates at h8-t3 before fixing"; within
one ladder a memo hit is exactly "a collapsed priced fact exists at this node
with `completed_at < j`", so a collapsed-⇒-return-the-fact clause would deliver
the same hits with no clone. Nothing has been measured. Done-when
(`kanban/backlog/ladder-policy-store.md`): FH2's gates and FH3's anchors gate
unchanged, the h8-t3 record byte-identical, peak RSS under 2 GB at h8-t3 and
under 4 GB for the gate. A neighbouring unverified item: the gym's 2026-09-06
change to `factor_belief.rs::condition_via` (`c59f1115`, pruning zero-completion
hands for every single-field conditioning) postdates every committed σ0-only
record; masses should be unchanged, the "σ0 states materialized" and per-route
wall coordinates plausibly not — no record has been regenerated to check.

---

## Things that are settled and might look open

Recording these prevents a successor from re-opening closed questions — or
re-closing open ones.

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
- **Freeze 38 is not reserved.** It was filled at FT-A17 on 2026-08-14 (items
  5 and 6, corrected on this page 2026-09-13). Only 39 and 40 are reserved.
- **The O5 "dead heat" is not a result in either direction.** It was
  withdrawn after the seed repair; what stands is two epochs disagreeing
  (item 17). Quoting a dead heat, or quoting either epoch alone as a strength
  verdict, is a regression.
- **The G1 trick-1 flip is attributed.** The waking seat plays the 6-4 at
  trick 1 where live Gran played the 6-2 because σ0 already chose 6-4 at the
  replay's epoch — an epoch/baseline difference, not partner modelling (the
  wake did not fire there; fiber 46,558,512 exceeds the exact cap). The
  experiment that would attribute it against the live level-1 seat at the
  screenshot's epoch has not run (item 16), but the waking-seat replay's flip
  is not evidence of anything about partner modelling
  ([walt-gran-anchors](walt-gran-anchors.md) §7).
- **The h8-t3 exact value is not a receipt.** `28859/29988` is a probe record
  (`horizon_run1.txt`), reproduced by a second probe (`focal_run1.txt`, the k = 3
  collapse) whose own law checks *cite* it (`focalreport.rs`, audit N5); the
  anchors gate `solver_focal_anchors.rs` covers seven other coordinates and
  no gate under `walt/walt/tests/` contains `28859` (grep, 2026-09-13). It is
  quotable at exactly that standing and no higher (item 12).
- **The doom-census diagnosis is corrected, not open.** "The plateau's
  remaining Γ ≈ 267‰ is overwhelmingly the info-consistency price" outran
  SC-A1/SC-A4 and was reconciled on 2026-09-03 (`walt/DISCREPANCIES.md`): the
  split is UNKNOWN. What is open is the split (item 13); the sentence itself
  is settled as an over-claim.
