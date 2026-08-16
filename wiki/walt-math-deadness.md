# walt mathematics — decision-deadness and valuation scope

[Home](Home.md) · owns: the decision-deadness objects of the walt branch —
the three-property typing, Lemma J, Lemma J(c′), Propositions J-0, J-1, J-win,
and Lemma E8, the exact valuation boundary · Sources: `walt/CENSUS-RULINGS.md`
§ "Decision-deadness probe rulings"; `walt/math/decision_sparse_exact_solving_v0.1_errata.md`
§8.5. Related: [the reference map](walt-math-reference.md),
[structure and transport](walt-math-structure-transport.md),
[decision-sparse witnesses](walt-math-decision-sparse.md),
[open questions](walt-math-open-questions.md).

> **Tier: EXPLORATORY throughout**, below every tier on
> [Home](Home.md#evidentiary-tiers--never-promoted-never-blurred).

This family answers "when does the seat's choice not matter?" It is the one
group where the branch's compression actually comes from — and it carries the
branch's sharpest recorded boundary between what survives count re-entry and
what does not.

---

## The typing that governs everything here: three node properties, never fused

| Property | Definition | The distinction, and why it is drawn |
|---|---|---|
| **forced** | \|legal\| = 1: **no decision exists** | Free to detect and **worth nothing**. Counting forced nodes as harvested deadness inflates every coverage figure for free (J-A15). |
| **decision-dead** | every information-consistent policy from the node has the **identical value function on the node's fiber** (N_vec = 1) | The design's object, and **the only one that licenses collapsing the subtree**. |
| **dominant** | one Pareto-undominated vector (N_par = N_exp = 1) | **The S6b singleton roots are this, not deadness** — which is exactly why six of the seven were indifferent and the seventh resolved a playbook of **384** free decisions to "play 1-1 over 1-0" (see the note below on 384 versus 108). Dominance licenses fixing a choice only if the dominant choice can be identified cheaply, **which is the work itself**. |

**The ladder: forced ⊂ dead ⊂ dominant**, with both inclusions strict on the
S6b evidence. (N_par = 1 ⟺ N_exp = 1: if one vector is optimal at every belief,
taking point masses gives pointwise dominance.)

**J-A1 makes this mandatory.** The results file carries the typing verbatim and
states that S6b's singleton frontiers are dominance, not deadness — the seventh
specimen is the proof that the two differ. **No sentence may present a
singleton-frontier count as a deadness count.**

### 384 versus 108 — how to name that playbook (EC-A12)

The idx = 0 lead-00 entry has been called both "the 384-decision playbook" and
"the 108-decision playbook". Both numbers are real and they are different
objects. The arithmetic, verified against the files at adjudication time: the
extraction has **50,712** states, of which **384** carry a genuine two-tile
choice (`separation_2026-08-13.txt`; S6b's k = 384 agrees), and 50,328 forced
+ 384 = 50,712. The S6c ground-truth classifier reports 384 classified and
**276** tied under one deviation (`deadness_2026-08-12.txt`), so 384 − 276 =
**108** states where the choice strictly matters.

**Binding, in any walt artifact:** name the entry by **384**, its receipt-backed
free-decision count. Where 108 is mentioned it is typed as *the
strictly-mattering subset under the S6c one-deviation classifier — a derived
difference of two measured counts (384 carrying the S6b/extraction free-state
typing, 276 carrying J-A10's classifier-denominator typing), inheriting both
scope fences, not an independent measurement, and present in no receipt*. Its
provenance is a `policy_inspect` diagnostic that is self-labelled exploratory
with no results file, which is exactly why 108 appears in no receipt. Like every
other number here, 108 becomes quotable as a result only by brief amendment
adding it to a verifier receipt. SEP-A17's phrase "the 108-decision playbook" is
**DISAMBIGUATED, not corrected**.

---

## Lemma J — non-interference ⇒ decision-deadness, and when count survives

**Hypothesis (NI) at node i**, with the ruling's fiber notation spelled out in
words. For **every** world in the node's fiber, **every**
information-consistent focal policy and **every** legal
continuation: **(i)** focal is **not the leader of the current trick and never
becomes the leader of a later one**; **(ii)** no tile focal plays is ever the
maximal trick key of its trick.

**Conclusion (exploratory).**

- **(a)** The joint law of (each non-focal seat's play sequence, each trick's
  winner) is **identical under every focal policy** — for the declared
  uniform-legal field and for any field whose per-seat play distribution depends
  only on that seat's own hand, the led context and its position in the trick.
- **(b)** Hence the **count-free** value is identical for every policy: the node
  is decision-dead.
- **(c)** If moreover **H ∩ COUNT = ∅** for focal's remaining hand H, the value
  is identical for every valuation reading the play only through the trick
  winners and the point values of the tiles falling in each trick — in
  particular trick-plus-count.

**Full statement and proof:** `CENSUS-RULINGS.md` § "Decision-deadness probe
rulings", under "Lemma J". Clause (c) carries a **GENERALISED** pointer marker
to Lemma J(c′) below.

**The channel argument that makes (NI)(i) indispensable.** Focal's remaining
hand does change which contexts focal can follow — but that is focal-internal.
It touches the other seats through **exactly one channel, the led context**, and
only the leader sets that. So the non-interference conclusion holds under
(NI)(i) and **fails without it**: at a node where focal is on lead, its choice
of lead changes the other seats' legal sets, hence the field's masses, hence
which tiles they still hold in every later trick. **Any detector that omits the
not-on-lead conjunct is unsound, and no "focal never wins" hypothesis repairs
it.**

**Scope of the verdict, mandatory in the results file.**

- **Relative to** a field that does **not condition on focal's tile identity**.
  Against an opponent who draws inferences from what focal discards, the choice
  carries information and **the verdict does not transfer**.
- **Not relative to** any world, belief or support: the conditions are functions
  of the focal information state and quantify over the whole live set. A verdict
  therefore **survives the support gap** between the seat's actual support and
  the declared void-free cost domain, rather than being fenced by it, and is
  **lawful at a pooled node without ever touching a particle**.

---

## Lemma J(c′) — the corrected and strengthened valuation clause

**Statement (exploratory), verbatim.** "Under **(NI)**, the value is identical
for every lawful policy for **every valuation that reads the play through the
trick winners and a tile-value schedule w that is constant on H**; the ordinary
count schedule under the guard H ∩ COUNT = ∅ is the case w|_H ≡ 0."

*Proof.* The focal seat plays exactly one tile to each remaining trick, so if
w|_H ≡ k then each remaining trick receives the same tile-value contribution k
from the focal seat whichever tile it plays; the other three tiles of each trick
and the winner of each trick are policy-independent by Lemma J(a). ∎

**Full statement:** errata §8.5(e). Authorised by DS-A24.

This is **strictly stronger** than the clause it replaces — constancy, not
vanishing — and its proof is the same argument. **Lemma J(c) is the special case
k = 0 and is sound as filed. Only DS-A9's gloss overreached.**

---

## Lemma E8 — the exact valuation scope of J-0 and J-1

**Setup.** Work in feature coordinates: the trick count and, per tile, an
indicator that the focal partnership captures it. A valuation is a pair (b, w)
with w a tile-value schedule.

**Statement (exploratory).**

- **(a) The invariant part.** Under (NI), the trick count has the same
  expectation; each per-tile capture indicator has the same expectation for
  every tile **outside** H; and the **sum** of the capture indicators over H has
  the same expectation. The last follows from the conservation law that four
  tiles fall in every won trick.
- **(b) The exact condition.** The value difference between two lawful policies
  is Σ_{d∈H} w(d)·Δ_d with Σ_{d∈H} Δ_d = 0. **w constant on H is sufficient
  always**, and it is also **necessary** whenever some pair of lawful policies
  moves one tile of H into a partnership-won trick and another out of one with
  positive probability — which is the generic case, but is a hypothesis and not
  a theorem about every configuration.
- **(d) Gauge stability.** The condition is stable under the parent's valuation
  gauge, so it is a condition on the **valuation class**, not on a chosen
  representative.

(The lemma's clause **(c)** is not a mathematical clause: it is the commentary
recording what the lemma corrects, reproduced immediately below. The letters are
not missing one.)

**Full statement and proof:** errata §8.5. Authorised by DS-A24.

**What it corrects — the branch's only *mathematical* correction of one ruling by
a later one.**
DS-A9 stated that under J-0 or J-1 with the guard the feature difference
vanishes identically and therefore holds "for every cone at once". That is
**false in fixed physical coordinates**: focal's own tiles land in different
tricks under different policies, so two zero-count focal tiles exchanged between
tricks are indistinguishable to the ordinary count schedule but **not** to an
arbitrary per-tile valuation with different weights on them.

DS-A24 sharpened the correction in two ways beyond what was proposed. **(i)** The
exact condition is w **constant on the exchanged tiles** — not w = 0, and not
only w ∘ Θ = w — because the feature difference lies in the sum-zero subspace
supported on the focal hand, so a constant contributes nothing. The
transport-invariance form w ∘ Θ = w is correct for J-1's transposition and is
the special case; **J-0's deadness is not proved through any single transport**,
so the transport formulation does not reach it and the constancy formulation
does. **(ii)** Gauge stability, making it a condition on the valuation class.

**What survives.** Count-free trick value (w ≡ 0) and ordinary Straight count
under the guard (w|_H ≡ 0). Propositions J-0, J-1, J-win and every J-A ruling
**stand unchanged**. The superseded DS-A9 clause carries a pointer marker at its
site; its text is not rewritten.

---

## Proposition J-0 — D0, exact, and no margin is needed

**Statement (exploratory).** At node i let H be focal's remaining hand, T the
unresolved-trick tiles, L the tiles still in hands, κ_δ the trumps. Suppose:

- **(a)** focal is **not the leader** of the current trick (nor of the next, if
  a trick has just resolved);
- **(b)** H ∩ κ_δ = ∅;
- **(c)** for every t ∈ H, every context q with t ∈ σ̂_q, and every tile
  d ∈ ((L ∖ H) ∪ T) with ℓ(d) = q: **d beats t in q**. (For a trick already in
  progress, d is the tile actually led.)

Then (NI) holds at i; so by Lemma J the node is decision-dead under the
count-free contract, and with H ∩ COUNT = ∅ also under trick-plus-count.

**Full statement and proof:** `CENSUS-RULINGS.md` § "Decision-deadness probe
rulings", under "Proposition J-0".

**Why no exhaustion margin exists to get wrong (quoted — this is the headline
result of the section).** "The design proposed to certify that a live beater
'cannot be exhausted before t's last possible play,' which would need a counting
argument sound against adversarial play orderings. It is unnecessary: the beater
is not some tile that must survive in someone's hand, it is **the led tile of the
very trick focal is playing to**, which by definition is in the led context and
is present by construction. **Nothing can exhaust it.** The only quantification
left is over *potential* leaders — tiles d with ℓ(d) = q — and if no such tile
exists outside H then q can never be led and the clause is vacuously true,
correctly."

So D0 is **three bitset tests, exactly sound, with no constant to freeze**: one
AND for (b), and per context one comparison of max(H ∩ σ̂_q) against the min over
potential leaders of q. Seven contexts, no allocation, no world, no solve.

**One dismissal was struck (J-A5).** The design's sentence "D0 is sound but
misses the measured volume" was REJECTED as unsupported — a category error,
reasoning from the *root hand's* winning chances to a *node-local* condition
evaluated after the root action is spent. D0's recall must be measured
node-locally before any such ordering is asserted. When it was measured, D0 came
out **rare but sometimes total**: one grade-3 root family is wholly
decision-dead and D0 certifies essentially all of it.

---

## Proposition J-1 — D1-sym, the transposition form, and where the guard earns its keep

**Statement (exploratory).** Let H contain t₁ ≠ t₂ and let τ = (t₁ t₂) act on the
live-plus-table structure, fixing every other tile. Call a context q **still
leadable** if some tile outside H with ℓ = q remains in play, or q is the current
led context. Suppose τ preserves: trump membership; follow membership in every
still-leadable context; the winner-determining order in every still-leadable
context; the double flag as it enters that order; and — **unless Proposition J-0
already shows focal never leads** — the led-context map on t₁ and t₂.

Then playing t₁ now and playing t₂ now have **equal value in every world, for
every isomorphism-invariant valuation**, so the choice between them is dead. If
moreover H ∩ COUNT = ∅, the verdict holds under trick-plus-count as well.

**Full statement and proof:** `CENSUS-RULINGS.md` § "Decision-deadness probe
rulings", under "Proposition J-1".

**Where the guard earns its keep**, condensed from the ruling (its symbol for
the count decoration is written out here). "For count: E-A2 bars count-bearing
readouts under structural transports because a general transport does not
preserve the count decoration. Here τ moves **only t₁ and t₂** and fixes every
other tile, so the decoration is preserved **iff c(t₁) = c(t₂)** — and the guard
gives c(t₁) = c(t₂) = 0. The decoration is preserved and the verdict lifts."

That is the precise sense in which the count guard lifts E-A2's restriction: not
in general, but for a transport that moves exactly two zero-point tiles.

**What was rejected alongside it (J-A6).** "D1 as posed" was REJECTED:
order-exchangeability is **the conclusion restated, not a checkable condition**,
and naming it a detector would license an unproved test. The design's proposed
decomposition was also rejected — its first half is unsound on its own (two
tiles that never contest the same trick can still differ in which trick each
wins) and its second half is D0.

**The recorded cost estimate, and how it came out.** The lemma's own Remark says
the conditions are demanding — two distinct tiles agree on follow membership
only when their differing contexts are already inert, and they must be adjacent
in the surviving order; the specimen pair {2-2, 6-3} fails on the double flag
alone; "expect low recall and report it as measured." When measured, **D1-sym was
the workhorse**, firing millions of times and covering up to 97.5% of a unit's
ties. The prediction ran the other way, and that is recorded as measured rather
than smoothed over.

---

## Proposition J-win — D1-win, count-free only, and this is where the guard fails

**Statement (exploratory).** If at node i focal is certain to win every remaining
trick under every legal continuation and every order of its own tiles — a cheap
sufficient form being: every trump outside H is dead; for every still-leadable
context q, every t ∈ H lies in σ̂_q and beats every remaining tile of that
context outside H; and each t ∈ H beats every remaining tile of its own led
context outside H — then the count-free value from i is |H| under every policy,
so the node is decision-dead **under the count-free contract**.

**Full statement and proof:** `CENSUS-RULINGS.md` § "Decision-deadness probe
rulings", under "Proposition J-win".

**The attached negative — the guard does not rescue it, and the reason is exact
(quoted).** "Focal now wins tricks, so **(NI) fails and Lemma J does not apply.**
Different orders lead different contexts, the other seats' follow obligations
differ, and therefore *which* of their tiles — **including their count tiles** —
fall into the tricks focal wins **differs by focal's choice**. The guard bounds
only focal's own contribution. Hence: a D1-win verdict is **void the instant
count re-enters (E-A2, wholesale, never extended)**, and a solve that pruned on
it may not be quoted for any count-bearing valuation."

**The resulting rule (J-A3).** The conjecture that the count guard rescues
count-free verdicts in general is **REFUTED**. The honest statement: the guard
rescues exactly the verdicts whose soundness runs through **non-interference**
or through a **count-preserving transport**. Every detector carries a tag —
`D0: count-free and trick-plus-count`, `D1-sym: count-free and
trick-plus-count`, `D1-win: count-free only`. **Untagged verdicts are not
adjudicated verdicts.**

**Measured outcome.** D1-win **never fired** across the whole probe. The
expected-very-low recall was measured as zero. The sweep condition does not
occur at the measured coordinates.

---

## The detector ranking, and the gap that was refused rather than filled

**J-A8** ranks the accepted family: (1) **D0** — exact, cheapest, count-safe;
(2) **D1-sym** — exact, count-safe, the workhorse in practice; (3) **D1-win** —
exact, cheap, count-free only, recall measured at zero; and **(4) the specimens'
mechanism — UNIDENTIFIED.**

That fourth line is a deliberate refusal, and the discipline behind it is worth
carrying: **"No cheap sufficient structural condition is known at adjudication
time for the six specimens' ties, and none is invented here."** A full
one-deviation evaluation *is a solve and is therefore not a detector*. If the
accepted members leave the specimens uncovered, the run records the residual as
a **named open question with its witnesses**, and does not ship a fourth
detector without a proof of the shape given for the first three.

It did leave them uncovered. See
[open questions](walt-math-open-questions.md) for the standing item.
