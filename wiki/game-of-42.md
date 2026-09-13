# The Game of 42, Mathematically

[Home](Home.md) · owns: the human-facing introduction to straight Texas 42 as a
mathematical object — what the game is, what is proved, what is measured, and what
the machinery can now do · Sources: [rules-profile](rules-profile.md),
[FINDINGS](FINDINGS.md), [claim-ledger](claim-ledger.md), [timeline](timeline.md),
[vocabulary](vocabulary.md), and the pages linked throughout; every claim carries
the tier of the page that owns it, and every walt number names the record or gate
that pins it. Related: [rob](rob.md), [lean](lean.md), [walt hub](walt.md),
[lineage](lineage.md). Repository state as of 2026-09-07 (c00717d1); fresh
measurements on this page are dated 2026-09-12.

This page is the doorway. It is written for someone technically literate who has
never played 42 and never read a game-theory paper, and who wants an honest
account rather than a pitch. Terms of art are explained where they first appear;
nothing is stated more strongly than the page that owns it states it; and where
the evidence is weak or contested, this page says so in the same sentence as the
result. Deep pages are linked rather than summarised. Three readers are kept in
mind throughout: a newcomer to 42 (§§1–2 and the plain sentences that open each
later item), a mathematician (§§3–4, §6, and the precise objects named in §5),
and an engineer who wants to run the player (§7, then
[walt-instruments](walt-instruments.md)).

---

## 1. The game

**The tiles.** A domino is a tile split into two halves, each bearing a number of
dots — *pips* — from zero to six. A *double-six set* has one tile for every
unordered pair drawn from 0 through 6, including matched pairs: 0-0, 1-0, 1-1,
2-0, up to 6-6. That is 28 tiles; the seven with matching halves are *doubles*.
Formally the set is the 2-element multisets over {0,…,6}, hence C(8,2) = 28.

**The seating.** Four players; those sitting opposite are partners, so two
partnerships. All 28 tiles are shuffled face down and dealt seven to each player,
none left over. You see your seven and nobody else's.

**The bid.** Before any tile is played, each player in one turn around the table
either passes or names a bid higher than the current high. A *point bid* is a
number from 30 to 41: a promise that your partnership will take at least that many
of the hand's 42 points. Above every point bid sit the *mark bids*, which promise
all 42; a mark is the unit the match itself is scored in. The auction is a single
round, and if all four pass the hand is thrown in and redealt
([rules-profile](rules-profile.md) has the fine print, including the proved fact
that the ladder of mark bids can never climb past five in one auction).

**The declaration.** The auction winner announces one of nine options, and this is
what makes 42 strange and interesting. Seven of them name a pip — say, fours — and
every tile containing a four becomes *trump*, outranking everything that is not;
note what that does, since 4-2 stops being a "two" or a "four" and becomes purely
a trump, pulled out of its ordinary suits entirely. The eighth option makes the
seven doubles the trump suit. The ninth, *no-trump*, makes nothing trump. The
declaration is legal regardless of what you hold.

**Suits and tricks.** Play runs seven *tricks*. The auction winner leads the
first; the others follow clockwise. The led tile fixes a *suit*: a trump leads
trumps, and any other tile leads the suit of its higher pip. Each following player
must play a tile of that suit if they hold one — a non-trump tile counts as
belonging to both of its numbers for this purpose — and otherwise may play
anything, including a trump. The highest trump wins the trick; if nobody trumps,
the highest tile of the led suit wins, "highest" meaning the double first and then
by pip total. The winner leads the next trick. That a trick always has exactly one
winner is not an assumption here: it is a theorem, proved from injectivity of the
ranking and separately checked against an independently coded rule-reader on all
737,100 possible cases (ALG-12).

**The points.** Each trick is worth one point for being taken — seven across the
hand. Five tiles carry extra value, called *count*: 5-5 and 6-4 are worth ten
each, 5-0, 4-1 and 3-2 are worth five each, totalling 35. Seven plus thirty-five
is 42, which is the name of the game and also a conservation law: every completed
hand distributes exactly 42 points, no more and no less.

**Winning.** If the declaring partnership takes at least what it bid, it wins a
mark; otherwise the other side does. Marks accumulate and the first partnership to
the match target (customarily seven) wins. A small theorem worth noting: a mark
bid succeeds exactly when the declaring partnership takes all seven tricks, so the
threshold wording and the traditional "take everything" wording describe the same
event (R-SETTLE-02A).

This project studies *straight* 42 — the plain points-and-marks game above.
Variants such as nello, plunge and sevens are outside the formal object, and
outside it *structurally*: they can change what a declaration means, how many
players are active, and the shape of the hidden information, so no theorem below
transfers to them automatically.

---

## 2. Why the game is mathematically interesting

42 sits in a productive size band: small enough to be exactly finite — there are
472,518,347,558,400 ways to deal the tiles to four labelled seats, a large number
but a completely definite one — and large enough that brute force gets nowhere.
The interesting part, though, is not the size. It is that **you cannot see the
other hands**. If everyone could see everything, 42 would be a finite tree and a
computer would solve it by working backwards from the end, the way checkers was
solved. The difficulty is entirely the hidden information: on your turn you know
your own seven tiles, every tile played and by whom, and what everyone bid — and
from that you must act, without knowing which of the many possible arrangements of
the remaining tiles you are actually in.

That collection of indistinguishable situations is the central object. Game theory
calls it an **information set**: the set of complete situations that look
identical from where you sit. In 42 it is concrete. Suppose 21 tiles are
unaccounted for — not yours, not yet played. Any split of them into three hands of
seven is a candidate for the truth, and there are 21!/(7!)³ = 399,072,960 of them.
Your information set is the subset consistent with everything you have seen; this
project calls that subset the **fiber** ([support-fiber](support-fiber.md)).

Two distinctions do a lot of work below, and this page keeps them apart rather
than smoothing them over, because the mathematics treats them as different types.

- **Possible is not probable.** The fiber is a set — a yes/no verdict about which
  worlds the rules permit. It carries no probabilities of its own, and assigning
  likelihoods is a separate act not forced by the rules; that is proved, not
  assumed ([capacity-dp](capacity-dp.md), CELL-10D). The fiber is called the
  **support**, the probability assignment is called the **belief**, and they are
  never merged.
- **Feasible is not reachable.** "Is there some hidden arrangement consistent with
  this description?" and "could legal play have produced this description?" are
  different questions with different answers. The second is strictly stronger, and
  there are explicit descriptions that pass every feasibility test and are still
  impossible ([reachability](reachability.md)).

---

## 3. How to read the evidence here

Claims never get promoted between evidence levels, and a claim's level travels
with it. Four levels, strongest first
([Home](Home.md#evidentiary-tiers--never-promoted-never-blurred)):

| Level | What it means |
|---|---|
| **Corpus theorem** | Proved in the two immutable specification packages that ground the project, carrying that package's own label ("Theorem — proved", "Theorem — exhaustive finite verification", …). |
| **Proof-assistant kernel** | Re-proved inside Lean 4, so a machine checked every step from the axioms. The target tier. |
| **Exchange-adjudicated CONFIRMED** | An external result from a separate strong reasoning model, whose verification program ran clean here and whose argument survived three adversarial referees. Real evidence; not a corpus theorem and not a kernel proof. |
| **rob conformance receipt** | An independent Rust re-derivation, byte-compared in continuous integration. Evidence that two independent implementations agree; never a change in any claim's status. |

Below all four sits the **exploratory tier**: the walt seat-player program, the
partnership experiments, the idea pages, the probes, and field measurements
against outside opponents. Those are labelled as such wherever they appear and
may not be quoted as results. Within that tier one further rule applies on this
page: a walt number is quotable as a result only through the gate or test file
that pins it, or the verifier receipt that carries it; otherwise it is a *probe
record* — the path is named so a reader can open it, and the number is not a
result. Results files outrank prose: where a record and a summary disagree, the
record wins and the disagreement is noted.

One more rule, applied literally below: **dissents travel with results**. Where an
external panel was not unanimous, this page says so in the same breath as the
number; where a later ruling corrected an earlier gloss, the correction is quoted
beside the original.

---

## 4. What has been proved

### 4.1 The rules themselves are theorems

Every trick has exactly one winner, proved structurally rather than by
enumeration. The 42 points are conserved exactly. A mark contract succeeds exactly
on a seven-trick sweep.

One more is a small pleasure. Mark bids are numbered — one mark, two marks, and
so on — and a table has to fix a house limit on how high that ladder may go. It
turns out the limit is irrelevant above five: no legal auction can ever reach a
sixth mark, because each mark bid must raise and there are not enough bidders left
to get there. So the number of distinct complete auctions, as the house limit goes
1, 2, 3, 4, 5, 6, 7, runs 2380, 3060, 3196, 3213, **3214, 3214, 3214** — it stops
moving at five and stays there forever. Every table with a limit of five or more
is playing the same game. All corpus theorems.

A less obvious one: the nine declarations are not nine different games. Strip away
the count values and look only at the mechanics of following suit and winning
tricks, and all seven pip-trump declarations become isomorphic; the nine options
collapse to exactly **three** mechanical classes — pip trump, doubles trump, no
trump ([declaration-algebra](declaration-algebra.md), ALG-22/23). Anything touching
the count values does not enjoy the collapse; anything not touching them does.

### 4.2 The keystone: knowledge of three hidden hands is a small object

Naively, tracking what you know about the other three hands means tracking up to
four hundred million worlds, or replaying the whole history every time you ask a
question. Neither is necessary. The keystone theorem is that your exact
rule-knowledge is captured **losslessly** by a small bookkeeping structure: the
pool of unseen tiles, the subset each hidden seat is still allowed to hold, and
how many tiles each still holds. The worlds computable from that small structure
are exactly the worlds consistent with the full history
([support-fiber](support-fiber.md), CELL-05 — corpus theorem, and also
kernel-proved in Lean).

The proof pivots on a nice observation. When a hidden player follows suit you
learn nothing positive you did not already see — the played tile is its own
witness that they could follow. The durable information is *negative*: a failure
to follow tells you permanently that a seat holds none of that suit. So
hidden-information tracking in 42 is monotone; it only ever deletes possibilities,
never restores them, and the total is capped at 63 deletions per hand. That cap is
a separate theorem with a lower tier than the keystone: it is the 63-edge budget
of the rec package ([support-dynamics](support-dynamics.md), TRANS-12 —
**corpus theorem, not mechanized in Lean**), and a reader should not inherit
CELL-05's kernel status for it. Three consequences make an efficient exact
implementation obviously possible rather than merely hoped for: consistency is a
seven-check condition, counting the fiber exactly needs no enumeration and at
most 512 internal states, and a *uniformly random* world can be drawn from the
fiber exactly without ever materialising it, in integer ratios with no
floating-point arithmetic ([capacity-dp](capacity-dp.md)).

### 4.3 That object has a canonical smallest form

Reduce your knowledge: some tiles you know for certain who holds, and the rest
form an ambiguous pool with a little structure on it. The result is a **normal
form** — a canonical representative — and the theorem is that it is *the* coarsest
exact representation there is. Every other exact deterministic representation
factors through it, and no two distinct normal forms can be merged without losing
exactness ([minimal-support-normal-form](minimal-support-normal-form.md), CELL-14,
kernel-proved in Lean). Standalone, over the full space of well-formed such
states, there are exactly 1,830,967,207,309,611,271,596,161 of them, so **81 bits**
are necessary and sufficient.

A note on that unit, since it recurs below: sizes here are quoted as bits, meaning
the base-two logarithm of a count — the number of yes/no answers you would need to
pin down which one you have, and equivalently the width of the smallest index that
could address them all. Eighty-one bits is that count above; forty-five bits is
about thirty-five trillion; thirty-six bits is about sixty-nine billion. Nine bits
of difference is a factor of five hundred and twelve, which is why the width of an
interval matters as much as its endpoints.

A companion corollary reads oddly the first time. The 81 bits are what it costs to
write a knowledge state down *from nothing*. But if you are already holding the
game state it was derived from — the played tiles, who played them, and the seat
whose view it is — then the normal form costs **zero** extra bits, because it can
be recomputed from those fields whenever it is wanted. It is a *view*, not a
second copy. That is not a curiosity but an engineering rule: this project forbids
storing both the derived view and the state it derives from, because two copies of
one fact can disagree and then there is no telling which is the authority.

### 4.4 Legal play reaches strictly less than combinatorics allows

That 81-bit census counts states that are internally consistent. It does not ask
whether any sequence of legal plays could produce them. That question —
**reachability** — is strictly harder, and many internally-consistent states are
simply not producible.

There is an explicit witness: a knowledge state that passes the consistency test,
is already fully reduced, and that no legal history can generate (REACH-10, corpus
theorem). A sharper one came from the external channel: a state passing *all four*
known necessary tests and still unreachable — 425,520 candidate histories
enumerated, zero realising it (exchange-adjudicated CONFIRMED, program green, 3 of
3 referees SOUND; independently reproduced in Rust). Both are permanent regression
tests: passing every check anyone knows how to write does not make a state reachable.
A vocabulary note, enforced by automated checks in this repository: the
conjunction of necessary tests defines a **necessary outer profile**, deliberately
*not* called a certificate. A profile is something every reachable state must
have; it is not a guarantee that a state having one is reachable, and the second
witness is exactly why.

How many reachable states are there? Nobody knows, and the project refuses to
guess. What exists is an interval, with its two tiers kept distinct.
**Corpus-proved: between 26 and 46 bits.** **Exchange-adjudicated: between 36 and
45 bits**. The two endpoints are counts of different objects, which is exactly how
a bound of this shape works: the floor comes from exhibiting 36,913,384,410
states that are genuinely reachable, in two structurally disjoint families, so the
true count is at least that; the ceiling comes from counting 33,297,009,347,414
*necessary outer profiles* — descriptions that every reachable state must satisfy,
some of which no reachable state actually inhabits — so the true count is at most
that. Neither number is the answer; the answer is somewhere between them. One slice is exactly closed: restricted to
states where nobody has yet revealed a void, the count is exactly **624,892,870**,
and it is *saturated* — every consistent pool in that regime is genuinely
reachable. That result carries the heaviest caveat in the ledger, and the panel
was **not unanimous: 2 of 3 referees rated it SOUND, the third UNVERIFIABLE while
finding no defect**. The dissent is recorded, and this page carries it rather than
rounding up.

Why it matters: 36 bits versus 45 bits is the difference between a precomputed
index of every reachable knowledge state being comfortable and being out of reach.
It is the flagship open problem ([open-problems](open-problems.md), OPEN-11).

### 4.5 The theorem that guards every shortcut: support is not belief

This one deserves room, because it keeps the rest of the project honest. The
temptation, once you have an exact canonical representation of what you know, is
to treat it as the whole story — to key a solver off it and throw the history
away. The 90-world witness proves you cannot.

The construction is a fully legal hand. Seat 3 bids 31 and declares no-trump. Five
tricks are played along a fixed line. At the resulting position six tiles are
unaccounted for, split two-two-two among the hidden seats with no relevant voids,
so there are exactly 6!/(2!)³ = **90** possible worlds. Seat 3 is on lead, holding
3-1 and 4-1, and must choose.

Now take two different auctions that both reach that same position, differing only
in *which* of two losing seats opened with a bid of 30. The mechanical position is
identical. The set of 90 possible worlds is identical. Under a fixed, stated model
of how players bid, both posteriors give **strictly positive probability to all 90
worlds** — so even the support of the belief is identical. Nothing in the
rule-knowledge distinguishes them.

But the probabilities differ. In one history the holder of 4-4 is distributed
1/7, 4/7, 2/7 across the hidden seats; in the other, 1/2, 1/4, 1/4. And exact
backward induction gives **opposite optimal leads**: history A prefers 4-1,
history B prefers 3-1. The flip survives all four named ways of scoring the hand —
expected points, signed differential, contract success, hand marks — and after the
lead every subsequent action is forced, so there is no wriggle room about how the
rest of the strategy composes ([belief-vs-support](belief-vs-support.md),
STR-06..09; the utility lenses are catalogued in
[strategic-state](strategic-state.md)).

The moral is sharp. An exact *support* state is not an exact *strategic* state.
Two positions with literally identical rule-knowledge can demand opposite play.
Any proposed shortcut that discards history must survive this witness — and the
witness is now **kernel-proved in Lean**: the fiber equality, all 90 legal
replays, both posteriors, and the value columns from 180 machine-evaluated
rollouts, closing in a single named theorem. A pointwise companion kills a whole
genre of heuristic: the same physical tile, at the same position, has exact
world-conditional values of −22 and +22 in two different members of the same
fiber. **There is no context-free scalar value of a domino.**

### 4.6 What the Lean formalization does and does not guarantee

As of 2026-08-02, **all 42 rows on the priority-0 mechanization scoreboard are
kernel-proved** in Lean 4 with mathlib ([lean](lean.md),
[proof-assistant-plan](proof-assistant-plan.md)) — from the 28 tiles and the count
values through the declaration algebra and the unique trick winner, the auction
machine, contract and play with 42-point conservation, the cell losslessness
keystone, the support normal form with its compile/decode inverse laws, strategic
sufficiency, and the 90-world witness internalised whole.

Precisely what that guarantees: every one of those theorems depends only on Lean's
three standard axioms — propositional extensionality, choice, and quotient
soundness — with **no `sorry`** (no admitted gaps) and **no `native_decide`** (no
appeal to compiled code the kernel does not re-check). Where a finite fact is
discharged by computation, the kernel performs the computation itself. So the
trust being extended is: Lean's kernel is correct, and the *statements* say what
was meant. The second is a real condition — a formalization can be perfectly proved
and about the wrong object — and the project's answer is its trust boundary rule:
an external `PASS` is never imported as an axiom, and finite claims enter the
kernel only by direct proof, a proved-sound decision procedure, or proved
reflection. What it does not guarantee: the priority-1 rows are not covered; the
two big exhaustive receipts (the 737,100-case rule-reader agreement, the auction
census) are deliberate later reflection targets; and none of the exchange-tier
reachability numbers are kernel-proved. Nothing under `rob/`, `lean/` or `ingest/`
changed between 2026-08-24 and 2026-09-07: every landing in that window is
exploratory, and the claim-tier picture above is exactly as the ledgers left it.

### 4.7 The Rust engine, and the external channel

**rob** is the exact engine: an executable specification written from the prose
proofs rather than translated from any existing verifier, whose job is to
reproduce independently every number the specification certifies
([rob](rob.md), [verification](verification.md)). Twelve byte-compared receipt
files live under `rob/receipts/`, regenerated and diffed in continuous integration
on every change; hand-editing one is forbidden, since it
would turn a check into a wish while still looking green. They reproduce the
737,100 unique-winner cases with independent agreement on winner *and* points, the
deal and auction censuses, the losslessness parity between derived fiber and
replayed deal set, the 81-bit census computed from formulas in exact big integers
rather than hard-coded, and both unreachability witnesses — all labelled
**conformance evidence** and nothing more, because two independent implementations
agreeing on an exact integer is genuinely informative and still not a proof. What
rob has *not* yet reproduced is stated on [rob-slices](rob-slices.md): the
reduced-kernel (slice 03) numbers and the 90-world posterior flip on the belief
side.

Beyond corpus and kernel, the project runs an adversarial exchange with a separate
strong reasoning model, whose results form their own tier
([exchange](exchange.md); 24 dispatches through 2026-08-25). Standing results: the
[36,45]-bit interval; the exact no-void slice with its recorded dissent; the
refutation of tightness for the outer language plus a new fifth necessary
condition; the transport theorem collapsing the reachable census from nine
declaration tags to three classes; an independent audit reproducing all 19
load-bearing census integers by two computation routes each; and a negative answer
to whether the compact "reduced viewer kernel" state is theoretically minimal — it
is strictly finer than the minimum ([reduced-viewer-kernel](reduced-viewer-kernel.md)).
Every row carries a verification-tier caveat in
[claim-ledger](claim-ledger.md): several reachability families close by prose
argument plus adversarial replay of sampled representatives, not by end-to-end
machine replay of all ~19 billion members. The caveats are part of the result.

---

## 5. What has been measured

Everything in this section is **exploratory tier**. It sits below all four levels
of section 3, is cited by nothing above it, and may not be quoted as a result. It
is here because it is the part a curious reader will find most surprising, and
because measuring is how the project decides what to try to prove next. It is
told as dated eras, one plain paragraph each with the precise objects and their
record paths beside it; [timeline](timeline.md) has every landing with its commit
and pull request, and each era has an owning chapter.

The measurements come from **walt**, the seat-level player program — the attempt
to play 42 from one chair, seeing only what a chair legally sees, on top of the
exact machinery ([walt](walt.md), [walt-program](walt-program.md),
[walt/LOG.md](../walt/LOG.md)). Three scoping rules apply throughout. Counts are
**carrier-relative**: counts over a specific declared representation at specific
probe coordinates, not global facts about 42. Where a probe declares a stopping
criterion in advance, this page reports the formal verdict, not the encouraging
part of the texture. And the **live default player has not changed since
2026-08-19**: every later player on this page is a variant, by standing ruling
(CE-A7 in `walt/CENSUS-RULINGS.md`, restated at every landing since), until arena
and conformance gates justify a change on Jason's word.

Some of walt's own vocabulary appears below and is worth having in hand
([vocabulary](vocabulary.md) has the full list with the ruling that fixed each).
A **coordinate** is one fixed starting point for a probe — a particular hand, in a
particular declaration, with a particular seat to act; probes run at a handful of
them and every number is relative to the ones chosen (`h8-t4` is receipt hand 8
at trick 4). A **grade** is how far a probe looks ahead, counted in decision
layers; the fibers behind the three grades measured in 2026-08 contain 6, 90 and
1,680 worlds. A **policy** is a complete rule for playing out the rest of the hand
from a position — not a single move, but an answer for every situation that could
follow. A **field** is the model of the other three minds a seat plays against;
**σ0** is the cheapest one, a level-0 mind that samples a few worlds and
best-responds to dice; a **level-1** seat best-responds to a field of σ0 minds,
and a **level-2** seat to a field of level-1 minds — always a best response to a
*named* field, never an equilibrium. The **objective is pmake**, the probability
of making the bid (ruled 2026-08-17); trick points are a proxy. An **epoch** is a
declared configuration of sampler and field; numbers from different epochs do
not compose. And four typed distinctions recur: an **estimate** (sampled, never a
receipt) is not a **receipt**; **exact-for-the-frozen-set** (exact over a fixed
sample of worlds) is not an **exact root**; **decision-dead** (no choice changes
any outcome) is not **decided** (the outcome is arithmetic) is not a **laydown**
(the contract is made against every world, every policy, every field).

### 5.1 First contact, 2026-07-30: the exact solver is beaten

In July 2026 the engine was seated against the champion of the predecessor project
— a strong heuristic that averages a perfect-information evaluation over ten
sampled consistent worlds and takes the best move ([lineage](lineage.md)). This is
**field-measurement tier**, computed by the other project's arena code, not by
certified Rust, and it carries zero evidentiary weight for the mathematics
([field/](field/Home.md), [first-contact](field/first-contact.md)).

Over 1,152 mirrored games and 12,866 hands, with the auction removed so both
players faced identical exogenous contracts, **the champion won**, at about 6.5
standard deviations: rob took 525 of 1,152 games. Paired on 6,028 mirrored deals,
the exact solver made 32.4% of its contracts against the champion's 36.1% *on the
same deals* — a gap of −3.65 percentage points, McNemar z = −6.52 — negative in
all seven declarations and on raw points, which was its own objective.

The same hands were then re-run with a takeover: a shared heuristic plays all four
seats through the opening, the position is frozen, and it is played out twice from
the byte-identical state, once with the solver's team declaring and once
defending. From trick 3 onward, across 768 paired positions, the two are
**statistically indistinguishable** — a pooled make-rate gap of −0.4 ± 1.7
percentage points, points agreeing to tenths. Together the two results localise
the entire deficit to the first two or three tricks, which the engine plays with a
fast stand-in never intended to survive. A side effect worth recording: the
bridge asserted the engine's independently derived trick leader and team score
against the other project's engine on every decision — **about 180,000 decisions,
zero divergences** between two independently written implementations of the rules
(stated in the field pages; no artifact in this repository records the count).

The caveats travel. The forced-contract protocol is not a realistic distribution
of contracts; a dead heat against a strong baseline bounds the distance between
two players, not either one's distance from optimal; and "exact" here means exact
best response *given a model of the opponent*, so when it loses, the model lost,
not the solve.

### 5.2 The frozen-basis programs, 2026-08-09 → 08-16: what compresses and what does not

Before walt played a hand it spent a week measuring the seat's problem on frozen
representations ([walt-pre-pivot-results](walt-pre-pivot-results.md) by result;
[walt-decision-sparse](walt-decision-sparse.md), [walt-s6-era](walt-s6-era.md),
[walt-census-era](walt-census-era.md) as provenance). Four findings stand, all
probe records unless a gate is named.

**About half of mid-game free decisions do not matter.** The question, in
Jason's framing, was "junk everywhere": can a seat cheaply detect that no choice
available to it changes any outcome, so search can skip the branching entirely?
Measured at census scale across 45 probe units: of 49,522,677 decision sites that
ground truth could classify, **25,255,316 — 51% — are one-deviation ties.**
Changing the move changes nothing about the outcome.

**Detecting indifference is essentially free, and provably never wrong.** Three
cheap bit-level detectors were each *proved* one-sided before being run: each
fires only when genuinely certain, and stays silent otherwise. Over
**174,250,255 detector calls with 27,980,333 fires there were zero false
positives** — every site a detector called dead, and that ground truth could
classify, was genuinely indifferent. The census timing (about 25 ns per call)
was taken under contention and is not quotable; the clean instrument *was* run
on 2026-08-13 as the S6c sequential timing rung
(`walt/probes/factory-results/deadness_rung_2026-08-13.txt`, freeze 43, DS-A33 —
a single uninterrupted process, selection rule declared in advance): **17 ns per
call** over 384 calls at the first grade-3 unit and **42 ns per call** over
3,540,143 calls at the first n = 4 unit, against solve arms of 26 ms and 28 s
respectively. That rung is a probe record, not a gate; the direction it
establishes — detection costs orders of magnitude less than the solve it
displaces — is the result. Reported recall is 8,335,057 of the 25,255,316 ties,
about 33% — and that figure **understates the detectors**, because the ground
truth compared against is one-deviation indifference, a *superset* of the strict
deadness the detectors are proved to certify. One detector never fired at all:
its triggering condition does not arise at these coordinates. And one family of
positions, the "trumpless junk" case, has hundreds of ties per lead and zero
detector hits; its tie mechanism is **unidentified**, and the standing rule is
that no fourth detector is written without a proof first
([walt-math-deadness](walt-math-deadness.md)).

**Value is rich; the decision is often trivial.** Two probes landed
opposite-looking results in the same week. The first asked whether the *values* a
position can produce compress: give each policy the list of values it achieves,
one entry per possible world — at grade 3 a list of 1,680 numbers — and ask how
many independent directions those lists span. The span came out between 1,461 and
1,680 directions out of a possible 1,680, with one coordinate at the full 1,680,
so the pre-declared criterion for the approach paying off was **refuted**
([walt-negative-results](walt-negative-results.md)). Value in 42 is genuinely
high-dimensional, and the reason is structural — every tile is eventually played
and publicly attributed, so a complete record of how a hand finished pins down
what everyone held. The second probe asked about the *decision* side: of all the
policies available at a position, how many are worth considering at all? A
policy is dismissible if some other policy does at least as well in every world
and strictly better somewhere. At **7 of 9** measured position-and-opening-lead
pairs exactly **one** policy survives, against raw policy counts as large as
2^19930. But the probe's formal verdict is **STOPPED, not 7-of-9 success**: the
other 2 of 9 pairs — both openings that lead a low trump rather than the boss
trump, at the one coordinate where the game is genuinely tense — ran past the
declared limit on how large a survivor set the probe would track, and under the
pre-declared discipline one uncompleted coordinate forbids the global claim. The
texture is the finding: total collapse almost everywhere, genuine explosion
exactly where 42 is hard.

**Root actions can be separated exactly, and compression buys nothing at the
start.** Three exact root-action verdicts (walt's records call them root
certifications, in walt's own sense — an exact separation of one action from
every competitor, nothing identity-bearing, deliberately not the §4.4 sense) were
obtained by bracketing a lower witness (the exact value of one fixed lawful
policy) against an upper witness (an action-conditioned relaxation) for every
competitor. One covers precisely the two leads whose survivor sets could not be
finished — proving the frontier unnecessary for the root decision. Seven of nine
per-action information prices came out exactly zero; the two nonzero prices sit
exactly at the two frontier-explosion leads. Separately, the **seat census** — how
many genuinely distinct situations does a seat face at the very first play? — was
answered by proof rather than enumeration: the structural symmetry quotient at
the first play is the **identity**, so the count is exactly **C(28,7) =
1,184,040**, about 11.84× over the declared target of order 10^5, with the only
available symmetry an exact 7:1 fold across the seven pip declarations.
**Structural compression is bought with deadness**: later in a hand tiles are
spent and contexts inert, and large collapses become available — one measured
level went 55 million states to 32,532 forms to 64 classes — but at the first
play nothing is dead yet, so nothing merges. Whether a *coarser* lawful notion of
sameness reaches a manageable size is explicitly open
([walt-math-open-questions](walt-math-open-questions.md) §2).

### 5.3 The seat plays and wins the marks, 2026-08-17

The first-contact loss got an answer, and the answer changed the objective before
it changed the algorithm. In August 2026 the walt program pivoted from compressing
truth to **building the seat that plays** ([walt-seat-play](walt-seat-play.md)),
with one ruling doing most of the work: the objective is **the probability of
making the bid** — pmake — because 42 is scored in marks, and trick points are
only a proxy for them. The resulting player is a sampling stack: an exact best
response, in exact rationals, over sampled rule-consistent worlds, against a
modeled field of simpler minds (level 1: n_outer = 50 worlds, n0 = 8 inner
worlds per modeled mind, in the arena bridge).

Seated against the same champion that had beaten the exact solver, under the same
dropped-30 mirrored protocol, **the seat won**: 630 of 1,152 games (54.7%) pooled
across three seeds of 384, McNemar z = +6.28 over 6,015 paired contracts
(discordant pairs 687 walt-made-champion-set against 473 the other way), every
seed's mark-margin confidence interval excluding zero
(`walt/probes/m3/arena_results_2026-08-17.txt`, "POOLED 3x384 VERDICT"; harness
mk5-main @ 594ee5e9). The signature is the objective made visible — walt *loses*
about 4.7 trick points per hand (−4.55, −4.82, −4.74 by seed) and *wins* the
marks, exactly what optimizing pmake rather than points should look like. Jason
has since played it at length at a web table and live inside the plunge product
on his phone. Every caveat of §5.1 still travels, and two more: all three seeds
played a bridge binary whose modeled-mind cache key lacked the banked totals (the
PiKey defect, fixed after the pool closed; the pool is internally consistent and
any later arena run is a new baseline), and the arena's own rules engine agreed
with walt's on every one of ~15,000 decisions — a conformance cross-check, never an
axiom. This is an **arena outcome about play at the exploratory tier** — sampled
estimates against a modeled field — never a statement about exact values, and
never quotable above this section.

### 5.4 Calculated evidence, 2026-08-24 → 08-29: estimates become evidence with a stopping rule

Plainly: until this week the seat's numbers were fixed-sample-count estimates
with no honest way to stop early or late. Two parents written by the external
collaborator, hand-ferried by Jason and adjudicated the same day inside the
exploratory fence (CE-A1..A8 and L2-A1..A7, `walt/CENSUS-RULINGS.md`), replaced
that with **anytime-valid settlement**: exact-rational betting evidence that stays
valid at whatever moment you stop, and a binding ladder of result types —
`ExactFiberRoot` / `ExactFrozenSet` / `DeltaSettled` / `EpsilonEquivalent` /
`Unresolved` / `HeuristicFallback` (CE-A3) — in which a sample cap is a resource
limit and never a proof rule, `Unresolved` is a *successful* output, and
exact-for-the-frozen-set is typed apart from an exact root. The build ran through
the parent's §22 in one day (PRs #15–#59, 2026-08-24/25). Its instrument of
record is the **shadow**: a controller run beside the live player over 33 hands
(`walt/probes/shadow/README.md`, PR #24) — 183 decisions shadowed, 67 settled
exactly over their frozen world set, 116 honestly `Unresolved`, 0 δ-settled at a
world cap of 128; tricks 1–3 all unresolved, tricks 5–6 all exact; the live
player's choice matched the controller's winner on 23 of 27 winner-bearing
decisions. Mining those 116 open records forecast that a cap of 512 would settle
about 108 of them and that about 22% of open comparisons are true fog no cap
fixes; **Jason's ruling of 2026-08-24 made world_cap 512 the way forward** (PR
#32 / `6e00528`) — the 128-era caps were phone-budget artifacts, not statistical
choices ([walt-calculated-evidence](walt-calculated-evidence.md), "The cap
analysis"). The L2 thread asked what changes when the *modeled field* is swapped:
three field-swap slices ending in a cancellation ladder whose lift at h8-t4,
Λ = 31/1200, is asserted in the probe binary (corrected from 41/1200 by the
adjudication PANEL-A1..A8 of exchange dispatches x:019–023;
`walt/probes/fieldswap_cancel/README.md`, "ALL 386 CHECKS PASS"; gate
`walt/walt/tests/solver_fieldswap_cancel.rs`), with the first `FieldDecisionChanged`,
`FieldStableExactRoot` and `Dominated` verdicts in the wild. The controller became
a seatable player (`solver::act`, six route labels, three settled and three
level-1 fallbacks — a capability with no arena run and no strength number), and
on 2026-08-25 the **waking seat** followed: σ0 always computed, escalation only on
positive decision-level evidence. Its natural-play profile
(`walt/probes/waking/README.md`, 2 driven hands, 56 decisions) put a decision at
p50 14.3 ms but p90 21.7 s and max 68.8 s — minutes per hand, affordability
verdict **NO, not as-is**; the speed campaign that followed found the modeled
minds are the bill and the sharing levers exhausted. A level-2 seat was compiled
for the browser (walt2-wasm) beside level 1; level 2 is a best response to a
named σ1, never an equilibrium, and whether small-knob level 2 beats big-knob
level 1 was left an open head-to-head question. No default changed.

### 5.5 Counted belief and anytime proof states, 2026-08-30 → 09-01: stop listing worlds, count them

Plainly: the seat stopped representing the hidden deal as a list of worlds and
stopped returning one-shot answers. Two more hand-ferried parents were adjudicated
the same day (CBS-A1..A9 on 2026-08-30, APS-A1..A9 on 2026-08-31;
[walt-math-intakes](walt-math-intakes.md)); by ruling CBS-A3 the objects are the
**root interval** [L, U] over pmake and the **survivor set**; the parents' own
names for these objects were retired and are not used here. The arithmetic that makes counting possible:
at a trick-1 root the 399,072,960 worlds are 116,280 possible hands for one
hidden seat times exactly 3,432 completions each (gate-pinned in
`walt/walt/tests/solver_factor_belief.rs`), and Theorem 20.1 (CBS-A6, the
parent's genuinely new mathematics) says that under a seat-local field,
conditioning on an observed hidden action multiplies only the acting seat's
factor — so the posterior stays a product of seat factors. The opening root's
exact one-ply branch table is therefore computed with no complete world
materialised: 8.7 ms under a trivial field (`walt/probes/factor_belief/run1.txt`
line 118: 8,671 µs) and about 5.4 s under σ0, because classifying the 116,280
hands through the modeled mind is 99% of every bill; cross-history cache reuse
measured exactly zero. The C→G ladder landed in one day (PRs #61–#69): exact
success *masses* with V = M/Z and no rationals in the recursion, a grammar best
response that strictly beats every single source policy at trick-4 roots,
consequence refinement whose honest negative is that driving residual mass to
zero fragments the opening root into 116,280 singleton classes, and an integrated
controller `refine_root` frozen as **RefineV1** (freeze 58,
[walt-math-freezes](walt-math-freezes.md)). Then the anytime program (PRs
#71–#78): a **proof state** is an append-only store of typed facts whose closure
(bars, survivors, exclusions, the recommendation) is a pure derived view,
serializable and resumable bytewise (gates in `walt/walt/tests/solver_proof_state.rs`);
its deliverable is a recommendation with **certified regret** Γ = U* − B_exec —
walt's own term of art, the gap between the best upper bound over all actions and
the floor witnessed by a policy the seat can actually execute, so Γ ≤ ε certifies
ε-optimality under the declared field and belief (gates
`solver_proof_regret.rs`). Argmax extraction closed one such gap exactly: at
h3-t4, Γ 83‰ → 0‰ and the recommendation switched 4-4 → 3-1
(`walt/probes/factor_belief/extractreport_run1.txt`; gates `solver_extraction.rs`).
The capstone is **the opening-root verdict** (Phase 8, PR #78,
`walt/probes/factor_belief/openingreport_run1.txt`; the ladder's laws — monotone
narrowing, resume ≡ uninterrupted, honest cliff — gated in `solver_opening.rs`,
the numbers themselves probe record): at receipt hand 0, trick 1, contract 30,
across budget stops p = 16/64/256/512 the executable bar climbs 0 → 407 → 594 →
732‰ while Γ falls 1000 → 592 → 405 → 267‰, the recommendation migrates 0-0 →
2-1 → 6-5, nothing prunes (7 of 7 leads survive), the sampled tier plateaus at
p = 512, and the verdict at every stop is **honest UNRESOLVED at ε = 1/4 — play
6-5, floor 732‰, at most 267‰ unclaimed**; all 29 refusals are pure
affordability; the whole state is 56 facts in 10,439 bytes, and the last stop
cost about 76 minutes. The **doom census** that followed (PR #79, 2026-09-01)
counts worlds in which the contract fails against every continuation and turns
them into deterministic upper bounds; on enumerable late roots it recovers most
per-world doom, at the opening root it certifies **zero** doomed worlds on all
seven leads. A correction travels with that verdict verbatim: the ledger's
follow-on gloss that the remaining 267‰ is "overwhelmingly the info-consistency
price" **was retracted on 2026-09-03** (`walt/DISCREPANCIES.md`) — zero doom
moves only the physical term, and the split of the 267‰ between the price of
playing without the hidden information and the gap of the executed policy is
**UNKNOWN**. Boundaries the era never crossed: no exact recursion at the opening
root, deterministic fields only, no arena, no default change
([walt-counted-belief-era](walt-counted-belief-era.md)).

### 5.6 Two recursions in opposite directions, 2026-09-01 → 09-05: the focal-horizon era

Jason's frame is on record in the briefs (`walt/briefs/BRIEF-MB1.md`): "42 is 2
recursions running in opposite directions" — the late tricks are enumerable and
hold no uncertainty about the other minds, so exactness runs backward from the
end; the model of the other minds lives in the early tricks, where play runs
forward on samples and structure. The fortnight built three things on the proof
state and measured where the two recursions trade dominance
([walt-focal-horizon-era](walt-focal-horizon-era.md)). **Model belief** (MB0,
MB1): the opponent model itself becomes a hidden coordinate with exact integer
priors, and the **model-fusion price** Φ — what a seat loses by playing one
response against a mixture of models instead of a separate response per model —
is exactly zero at all fourteen trick-5/6 coordinates and **strictly positive at
trick 4**: h8-t4 lead 3-1 has Φ = 38/9600 (about 4‰), pinned by gate M6 in
`walt/walt/tests/solver_model_belief_recursion.rs`; the other seven trick-4 rows
(1–9‰) are record only (`walt/probes/factor_belief/modelbelief_recursion_run1.txt`).
A corollary of the parent's mathematics makes that load-bearing: once Φ is zero
at one full-support prior it is zero at every prior over the same types, so the
zeros could never have been re-weighted away. The **God-gap censuses** (U0, U0b)
measured the price of playing blind against a viewer who sees the world: at
receipt roots the fusion horizon is trick 5 (every trick-5/6 coordinate
God-tight, every substantive trick-4 coordinate a positive gap of 6–22‰), but
*inside* a
trick-4 solve the trick-5 frontier is not fusion-free (13–14‰ mass-weighted), and
a cut there can flip the play. Along the way the single-field recursion solved
**h8-t3 exactly** — fiber 59,976, Q* = 28859/29988 (962‰), argmax 1-1, in
289,407,472 field reads and 797 s of wall (`walt/probes/factor_belief/horizon_run1.txt`
line 11932; about 13–14 minutes) — the program's first exact trick-3 value, where
a trick-4 cut over-prices the root by 31‰ and flips the play to 3-3. The
**unified player** (UP0, UP1a) put all of it under one five-tier decision
cascade with unforgeable provenance — decided arithmetic, then endgame exact,
then middlegame mixture, then certified regret, then the σ0 fallback that always
answers — and its first probe found 99.4% of the lean rung's wall spent carrying
a posterior no tier ever read; the lazy carry fixed that to 0 µs with lazy ≡
eager on all 216 decisions (gates `solver_unified_carry.rs`). Then, on
2026-09-04, the **focal-horizon hierarchy** (FH0–FH5; rulings FH-A1..A11; by
FH-A2 the object's name is the hierarchy, and the parent's own name for it was
retired): for each root action an
interval [L_k, U_k] indexed by k, the number of the seat's own future decisions
made exact — a lawful tail below, the world-revealed continuation above — that
collapses to the exact value at the last layer, and that subsumes the God-gap
censuses as its k = 0 and k = m−1 uppers. The report of record
(`walt/probes/factor_belief/focal_run1.txt`, 33 coordinates; laws gated at seven
anchor coordinates in `solver_focal_anchors.rs`): **every live trick-4 coordinate
settles by k ≤ 2** — five at k = 0 with no search at all, six at k = 1 with
certified regret at most 45‰, three at k = 2 — while the trick-3 anchor h8-t3
settles only at k = 3 (Γ 141 → 100 → 34 → 0‰), reproducing the 14-minute value.
The finding that redirected the program: **at k ≥ 1 the remaining width is the
tail's policy gap, not fusion price** — per action Q − L is 9–41‰ while U − Q is
0–3‰ (`walt/briefs/FH1-REPORT.md`, scout finding 2) — so a better lawful tail buys
more than a deeper search on this corpus. The two ply-cut flips the censuses had
found live entirely on the upper side and are never certified. What it cost, as a
finding: 3.82 million facts and 19.4 GB peak resident memory at h8-t3, the gate
growing from 230 s to 308 s. An independent audit (FH4) returned one BLOCK — the
vocabulary — and thirteen notes, all fixed the same day; the program landed on
main as PR #88 on 2026-09-07. Jason's ruling of 2026-09-04: follow through, then
a consolidation slice; no new mathematical parent until it lands.

### 5.7 The Gran hands: the 6-4 problem

In a real game on the phone, Jason's walt partner — named "Gran" on screen —
held the 6-4, a ten-count tile, and played the 6-2 at trick 1 instead. Jason had
bid 30 in sixes; his play depended on knowing where the 6-4 was; the hand was set
25–17. The question that hand poses is the partnership question in one tile: a
seat that best-responds to a field of simple minds can be exactly indifferent
between giving its partner the count and keeping it, and *something* has to break
the tie. The hand became anchor **G1** ([walt-gran-anchors](walt-gran-anchors.md);
`walt/probes/gran/README.md`): its full deal was transcribed from two screenshots
pinned in the data home and mechanically validated — 28 distinct tiles, every
follow legal, every winner and every trick's points re-derived
(`walt/probes/gran/g1.receipt.txt`; `granrun validate` re-run 2026-09-12 on this
machine: VALIDATED, well under a second). Replayed by the waking seat on
2026-09-04, the seat plays the 6-4 at trick 1 — but the wake did *not* fire (the
trick-1 fiber is 46,558,512 worlds, above the exact cap of 1,024), and the σ0
baseline picks 6-4 on its own; so the flip is an epoch/baseline difference from
whatever the phone was running, not a demonstration that partner modelling fixes
the problem. The one real wake landed at trick 5 (fiber 300) and chose 4-2, the
human's play; whole-hand agreement 6 of 7; the seat in all four chairs still goes
set, 26–16. G2/G3 (bid 31 in sixes, 36–0) is a six-trick prefix whose deal could
not be recovered; the trick-1 6-4 there was forced. The morning readout of
2026-09-05 (`walt/briefs/MORNING-2026-09-05.md`; probe records on the unmerged
branches walt-o5 and walt-g1-l2, not on main) named the mechanism: G2 is exactly
locked from trick 3 — every deal makes whatever the seat plays — and at exact
indifference the tie is broken by `TieRule::LowestTileIndex`, so a tied seat
deterministically keeps the 6-4 (tile 25 of 28) and plays the lower tile. The
design conclusion is that another rung of modelling is not the remedy; the levers
are the **objective** and the **tie-break**. The same readout repaired a seed
defect in the O5 match (the cost of the no-void inner belief) and withdrew its
"dead heat": live epoch +262 of 6,048 pairs void-aware ahead, reduced epoch −226
of 16,128 behind — measured only on the unmerged branch. On 2026-09-06 the
partnership program's matched roots at native 40/8 showed the same knife-edge:
baseline 33/40 vs 33/40 → 6-2 by index tie, partner-only 34/40 vs 37/40 → 6-4,
full level 2 38/40 vs 36/40 → 6-2 (`experiments/partnership/REPORT.md`) — sampled
scores against a modeled field, not probabilities.

### 5.8 Partnership and the gym, 2026-09-06/07: first instruments, no strength result

A two-day program under a deliberately waived Rust gate — its receipts are
independent Python rules replays, pinned hashes and resume proofs, never rob
receipts ([walt-partnership-program](walt-partnership-program.md); results files
under `experiments/partnership/campaigns/`). First it pinned the reference: the
**phone** is a byte-for-byte copy of the walt the plunge app ships (walt.wasm
SHA-256 af0200af…, byte-identical to this repository's build at commit 9a056f20
of 2026-08-19), playing level 1 at 40 outer / 8 inner worlds with racing on, and
bidding by plunge's own heuristic, not walt's. Native fixed level 1 turned out to
trail that phone — 8 favorable / 15 unfavorable / 77 ties over 50 deals (−7.0
points per matched contract), at 0.080 s versus 0.636 s per move
(`campaigns/native-l1-vs-phone-620600-649/CALIBRATION.md`) — because the phone
races and refines; one selection authority (`solver/selection.rs`: fixed, refine,
race-refine) then made native **l1-race reproduce the phone move for move in all
35 of 35 fallback-free mirrored pairs** (`campaigns/foundation-battery/RESULTS.md`),
3/1/46 on the pairs overall. The partner-aware candidate — the partner modeled as
a level-1 mind, opponents as level 0 — is lawful, information-consistent and fast
enough for the 14 s wrapper, and **has not beaten its reference**: against the
phone 15 favorable / 23 unfavorable / 156 ties on 97 fresh deals (−4.1 points),
21/27/152 on a fixed-hand panel; against native level 1 on 100 shared deals,
**L2 Partner 14 wins / 14 losses / 72 ties (50.0%) at about five times the cost**
(1.127 s versus 0.228 s per move; `campaigns/default-partner-battery/RESULTS.md`);
void-aware inner belief 12/17/71 against voidless. Every refined partner
configuration exceeded the fallback gate. These are negative results and are
stated as such; no default changed; level 1 is the defensible operating default.
Beside the players stands the **exact partnership gym**
([walt-gym](walt-gym.md); `walt/gym/`): a saved late-game position — one seat's
hand plus the public record, never the hidden hands — becomes an exercise whose
answer key is the exact pmake of every legal play under a declared belief and a
frozen field (teammate = level 1 at 40/8, opponents = level 0), audited three
ways, and the playable players are graded on root-action regret. Six starter
count-offer exercises (level 1 5/6, L2 Partner 6/6, L2 with voids 5/6;
`walt/gym/RESULTS.md`); 170 coordinates discovered by Scheme queries over 1,929
saved positions in about 13 s (`walt/gym/DISCOVERY.md`); **433 bid-making
exercises** with strict make-probability differences (367 with a unique best
play, 26 certain make-versus-set swings) from one four-line query in 50.6 s
(`walt/gym/BID-MAKING.md`); and a composed 30-position exam where offering count
is required for optimality, on which **level 1 scores 24/30 (mean regret
1643/205200) and L2 Partner 26/30 (959/205200)** — three improved, one worsened
(`walt/gym/PARTNERSHIP-COMPOSITION.md`). The gym is a diagnostic under one field,
not a strength ranking. Its query language is **Scheme/Fix**
([walt-scheme-fix](walt-scheme-fix.md)), invented to compress hidden state and
commissioned on 2026-09-06 to *express* it instead: an executable relational
language over the exact finite worlds of a position, 28 predicates, exact event
probabilities, explicit selection, counterexample worlds, and refusal rather than
partial reports when a work budget is hit. Two learners closed the fortnight
with negative results: sampled exact-table policies fit their training worlds far
better than held-out ones (77.8% versus 25.6% makes;
`campaigns/policy-synthesis-v1/RESULTS.md`), and a shared relational actor trails
a 16-sample table by 5.47 points, with one hybrid gaining an uncertain +0.771
points whose interval [−0.122, +1.909] includes zero
(`campaigns/relational-learning-v1/RESULTS.md`); an information-price basis
tightened 0 of 64,806 bounds. Bidding and partnership now have their first
instruments; neither has a strength result.

### 5.9 The eras at a glance

| Era | Dates | What it settled, plainly | Record of record | Owning chapter |
|---|---|---|---|---|
| First contact | 2026-07-30 | The exact solver (points objective) lost to the E[Q] champion, 525/1,152; dead heat from trick 3 on | `wiki/field/first-contact.md` | [field](field/Home.md) |
| Frozen-basis programs | 2026-08-09 → 08-16 | Half of mid-game decisions are ties; detectors free and sound; value does not compress, the decision usually does; first-play quotient is the identity | `walt/probes/factory-results/` | [walt-pre-pivot-results](walt-pre-pivot-results.md) |
| The seat plays | 2026-08-17 | Level-1 walt under pmake beat the champion 630/1,152, z = +6.28 over 6,015 paired contracts | `walt/probes/m3/arena_results_2026-08-17.txt` | [walt-seat-play](walt-seat-play.md) |
| Calculated evidence | 2026-08-24 → 08-29 | Anytime-valid settlement; shadow 67 exact / 116 unresolved of 183; world cap 512; field-swap lift 31/1200; waking seat unaffordable as-is | `walt/probes/shadow/`, `fieldswap_cancel/`, `waking/` | [walt-calculated-evidence](walt-calculated-evidence.md) |
| Counted belief and proof states | 2026-08-30 → 09-01 | 116,280 hands × 3,432 = 399,072,960; certified regret; opening verdict play 6-5, floor 732‰, ≤ 267‰ unclaimed, UNRESOLVED at ε = 1/4; the 267‰ split UNKNOWN | `walt/probes/factor_belief/openingreport_run1.txt`; `walt/DISCREPANCIES.md` | [walt-counted-belief-era](walt-counted-belief-era.md) |
| Focal horizon | 2026-09-01 → 09-05 | Fusion price strictly positive at trick 4 (38/9600 pinned); h8-t3 exact at 962‰; every trick-4 coordinate settles by k ≤ 2; residual width is the tail's policy gap | `walt/probes/factor_belief/focal_run1.txt`, `horizon_run1.txt` | [walt-focal-horizon-era](walt-focal-horizon-era.md) |
| Gran anchors | 2026-08-24 (screenshots) → 09-05 | G1 validated and played; the 6-4 flip is an epoch fact; hoarding = exact indifference + lowest-index tie-break; levers are objective and tie-break | `walt/probes/gran/`; `walt/briefs/MORNING-2026-09-05.md` | [walt-gran-anchors](walt-gran-anchors.md) |
| Partnership and gym | 2026-09-06 → 09-07 | Phone pinned; l1-race = phone; L2 Partner ties level 1 14/14/72 at ~5× cost; gym 6/170/433/30 with level 1 24/30 vs L2 26/30; learners trail tables | `experiments/partnership/campaigns/*/RESULTS.md`; `walt/gym/*.md` | [walt-partnership-program](walt-partnership-program.md), [walt-gym](walt-gym.md) |

---

## 6. What is still open

These are two different kinds of open, and the difference is the tier boundary of
section 3, so they are listed separately rather than interleaved. Mixing them
would let an exploratory question acquire, by adjacency alone, the standing of a
corpus-proved boundary.

**Open at the corpus and exchange tiers** — inventoried at
[open-problems](open-problems.md):

- **The exact reachable count.** How many knowledge states can legal play actually
  produce? Open, boxed to 36–45 bits externally and 26–46 at the corpus tier, with
  the no-void slice the only stratum exactly closed. The flagship (OPEN-11).
- **A support-only reachability test.** Deciding reachability still requires
  replaying a history — now a history free of any hidden deal, which is progress,
  but still a history (OPEN-12).
- **Beliefs off the beaten path, and the match horizon.** What a player should
  believe after an event their model called impossible must be *chosen*, not
  derived; and a match with repeated all-pass redeals has no bounded length
  without an added assumption. Both are boundaries, not bugs.
- **Minimality of the 90-world witness.** The witness is kernel-proved; whether 90
  is the smallest fiber exhibiting the flip is untouched.

**Open at the exploratory tier** — walt's own questions, which nothing above
cites. The standing inventory is
[walt-math-open-questions](walt-math-open-questions.md) (fifteen questions with
the ruling that left each open, from the trumpless-junk tie mechanism through the
level-2 field-swap question and the per-epoch σ0 declaration); the exploratory
block of [open-problems](open-problems.md) carries the gym's and the partnership
program's questions. The ones a reader of this page has just met:

- **The 267‰.** At the opening root the seat's floor is 732‰ and at most 267‰ is
  unclaimed. How much of that is the price of playing without the hidden
  information and how much is the executed policy's own gap is **unknown**
  (`walt/DISCREPANCIES.md`, 2026-09-03); the doom census showed only that the
  physical term is zero.
- **The cheapest lawful tail.** At every trick-4 coordinate the residual width is
  the tail's policy gap, not fusion price; which cheap tail closes most of it, and
  what a tail-improvement ladder looks like, is the question the focal-horizon
  program handed back (`walt/briefs/FH-RESPONSE-TO-PRO.md`, a draft letter for
  hand-ferry; whether it was sent is not recorded in the repository).
- **σ0's sufficient statistic of the public record.** Field classification is 99%
  of every exact recursion's bill and cross-history reuse is exactly zero because
  the cache key carries the whole history; if the modeled mind's answer depends on
  less than the full record, every recursion gets cheaper by one to two orders of
  magnitude. Named as the read-key study; not run.
- **The 6-4 levers.** The hoarding mechanism is exact indifference broken by tile
  index; which objective or tie-break rule should break it — and whether the fix
  is free — is Jason's open call, with the O5 measurement living only on an
  unmerged branch.
- **A lawful notion of "same situation" that makes the seat's world manageable.**
  The structural notion is the identity at the first play; the linear one
  saturates by the third grade; the counted-belief factorisation made the opening
  root *countable* without making it small. Whether a dynamics-based or
  value-based notion reaches a workable size is open.
- **The trumpless-junk tie mechanism.** Half of mid-game decisions at the measured
  coordinates are ties, and one identifiable family of them resists every proved
  detector. Nobody knows why those positions are indifferent.
- **The gym's own questions.** 433 exact bid-making exercises exist and no pupil
  has yet been graded on them; the 460 defending-side keys are computed but
  unpublished; every key is relative to one declared field, and how sensitive the
  keys are to that field is unmeasured.
- **Whether a strong plan can be transported rather than recomputed.** A policy
  provably right at one position is, so far, only a *seed* elsewhere; sampled
  exact tables fit their training worlds and not held-out ones; the relational
  learner's rules transfer weakly. "This policy was right over there" currently
  has no status here.

---

## 7. What can be done with 42 now

An honest inventory, separating what exists today from what is a direction. Where
an item names a binary, release builds live under `walt/target/release/` (or
`cargo run --release -p walt --bin <name>` from `walt/`);
[walt-instruments](walt-instruments.md) has every invocation, record path and
gate, and [walt-architecture](walt-architecture.md) the crate they live in.
Everything below the first two items is exploratory tier.

**Today.**

- **Reproduce every proved number, and inspect any position's exact fiber.** rob
  regenerates its twelve receipts in CI, and ships an HTML inspector that steps
  through a game from any seat's perspective, showing the exact fiber count, the
  exact marginals, the plan tree, and the exact best value for every legal
  opening — chosen and rejected alike, so "why not the other tile?" is answerable
  rather than rhetorical. Every number is computed by the certified Rust and
  emitted; the display recomputes no game logic ([analysis](analysis.md)). The
  Lean build (`lake build` in `lean/`) re-checks all 42 priority-0 rows from the
  axioms.
- **Solve an endgame exactly from a seat.** Given a position, the exact fiber, a
  stated field and the pmake objective, walt computes the exact best response
  over the whole information set — no floating point, no impossible worlds,
  integer masses throughout. On the receipt corpus trick-6 roots take
  microseconds to milliseconds, trick 5 seconds, trick 4 seconds to minutes, and
  one trick-3 root has been solved exactly in about fourteen minutes (and, when
  the focal-horizon ladder walks it, 19 GB of memory); those are walls of
  particular runs, not claims.
- **Ask for a recommendation with certified regret.** `openingreport`,
  `proofreport` and the unified player return a recommended executable policy
  with its floor B_exec, the global upper U*, and Γ = U* − B_exec, under a
  declared budget; "honest UNRESOLVED, play 6-5, floor 732‰, at most 267‰
  unclaimed" is what that looks like at the opening root. The answer improves
  monotonically as budget is added and never claims more than its facts support.
- **Keep a proof state.** The facts behind a recommendation live in an
  append-only, content-addressed store (`solver::proof_state`) that serializes,
  resumes bytewise after interruption, and admits new producers without editing
  the core; the opening-root state is 56 facts in 10,439 bytes.
- **Bracket every root action at a chosen horizon.** `focalreport` computes the
  focal-horizon interval [L_k, U_k] per action for k = 0, 1, 2, …, with the
  survivor set and Γ_k at each rung, settling a trick-4 root by k ≤ 2 and
  collapsing to the exact value at the last layer; a per-root fact store with
  exact suffix reuse cuts the k = 2 pass by roughly 5–7× in reads.
- **Price the model, and the information.** The model-fusion price Φ of playing
  one response against a mixture of opponent models, and the God gap of playing
  without the hidden world, are both computable per root action on enumerable
  roots; both are exactly zero at trick 5–6 receipt roots and positive at trick 4.
- **Detect that a decision does not matter, at negligible cost.** Proved-sound
  one-sided detectors with zero false positives over 174 million calls, at 17–42
  ns per call on the clean rung; the typed hierarchy above them classifies a root
  as decided, forced-make, or a true laydown (`solver::laydown`, deterministic
  only — no sampled route ever constructs a laydown).
- **Query a position in Scheme.** `scheme --query walt/scheme/examples/partner-count.scheme
  --hand 0 --trick 6 --seat 0` asks, at receipt hand 0 trick 6 from seat 0,
  whether the partner holds a count tile: six lawful worlds, event probability
  exactly 1/3, answer 4-1, work 1,084 units (measured 2026-09-12 on this machine:
  0.01 s). Any relation over chairs, dominoes and contexts is expressible the same
  way; the program refuses rather than reports partially when a budget is hit.
- **Grade a player in the gym.** `python3 experiments/partnership/gym.py report`
  prints the pupils' scores on the published exercises;
  `gym.py run --players l1-default l2-partner-default …` grades fresh choices
  against exact keys; `gym.py verify` re-derives every key independently in
  Python; a specification file regenerates a whole collection (the 433
  bid-making exercises in 67.7 s).
- **Play a seat.** The live default is level-1 walt: `walt_bridge` seats it in
  the mk5 arena over a line protocol; `walt-wasm` is the same seat compiled for
  the browser and shipped inside plunge (the "phone", 40/8 with racing on);
  `webtable` (localhost, real auction at ϑ = 11/16, trump pricing) and
  `playtable` (terminal) let a human sit at a table with it. Variants, none
  default: `walt2-wasm` (level 2), `controller_bridge` (route-labelled
  controller), `waking_bridge` (the thinking-teammate seat, minutes per hand),
  `granrun` (the waking seat on the Gran anchors), and the partnership player
  (`experiments/partnership/player.py --mode partner`, with `match.py` for
  mirrored matches and `players --all` for the eighteen presets).
- **Measure a player honestly.** Mirrored-pairs protocols with a takeover knob
  (mk5 arena) and resumable paired matches with independent Python replay of
  every move (`match.py`) turn "why did it lose?" into a bisection rather than a
  debate, and re-verify the rules against an independent implementation as a
  free side effect of playing.

**Directions, not results.**

- **The contagion frame.** Jason's observation is that good 42 is *contagious* —
  you watch a strong player, adopt something, and your play improves — while the
  strong players themselves often cannot transfer their policy by explanation. Two
  measurements above sharpen why. Observation supplies trajectories, not
  counterfactuals: you see what was played, never what would have happened
  otherwise. And since about half of free decisions are ties, an observer cannot
  reliably tell load-bearing moves from arbitrary ones — the teachable content
  concentrates in the tense positions, which are exactly the rare ones. The lawful
  form of contagion is therefore a strict division of labour: **observed strong
  play seeds the candidate library; exact evaluation decides.** A library carries
  policies, never verdicts. Its one robust property is that a policy's validity as
  a lower witness depends only on its lawfulness, not on the valuation under which
  it was discovered — so a candidate found in a simplified setting survives into
  the full scored game as a witness, even though its quality verdict does not. The
  gym's exercises and Scheme's expressible policies are the first concrete pieces
  of such a library; the relational learner's weak transfer is the first
  counterexample material. A related idea — reading a bid as a lower-bound claim
  about a hand — was **explicitly declined** on the count-free carrier of 2026-08
  ([walt/ECONOMY-SUCCESSOR.md](../walt/ECONOMY-SUCCESSOR.md) §2.4) and has not
  been revisited on the counted basis.
- **Consolidation before new mathematics.** The God-gap census, the in-solve
  horizon census and argmax extraction are, in the hierarchy's vocabulary, its
  k = 0 upper, its k = m−1 upper and its π_k — measurement scaffolding around one
  recursion — and RefineV1 is already declared removable. The consolidation slice
  that tree-shakes them, and the σ0 read-key study before it, are the queue
  (`walt/MAP.md`); Jason's ruling is that no new parent lands first.
- **Choosing which worlds matter.** The counted-belief factorisation and the
  consequence refinement of §5.5 are the first lawful forms of this: exact masses
  over classes of worlds rather than raw worlds, with refinement that stops at a
  residual interval instead of chasing the endpoint
  ([idea-hierarchical-fibers](idea-hierarchical-fibers.md) is the 2026-07 seed;
  its "coverings with sound bounds" rung is what the focal-horizon intervals now
  implement, on walt's own basis).
- **Working backwards.** Backward induction seeded from canonical arrangements of
  the last trick rather than from concrete deals — "the pips don't matter, the
  relationships do" ([idea-retrograde-rank](idea-retrograde-rank.md)); the
  backward half of "two recursions in opposite directions", and Scheme's
  relational dynamics are its nearest kin.
- **Beliefs, then partnership.** Everything measured against the champion deleted
  the auction to make two players comparable. Bidding, inference from bids, and
  partnership convention are where the real game is played; the support-versus-
  belief separation is the vocabulary for approaching them honestly. As of
  2026-09-07 both have first instruments — the calibrated bid rule at ϑ = 11/16,
  the partner-aware player, the gym, Scheme — and no strength result.

---

## 8. Where this stands

What the project has: a proved, machine-checked account of what a 42 player can
exactly know; a canonical smallest form for that knowledge; an exact and cheap way
to count and sample the possibilities it leaves open; a proof that the knowledge
alone is not enough to play by; an engine that reproduces every slice-01 and
slice-02 number independently and is byte-compared on every change; a seat that
plays a full hand from one chair under the pmake objective; a way of turning that
seat's estimates into recommendations with certified regret; and a growing body
of exact measurement about which decisions in 42 actually matter and where the
cost of not seeing the hidden hands actually lives.

What it does not have: the exact size of the reachable state space; a compression
of the seat's situation down to a workable size (the opening root is countable
now, not small); a strength result for bidding or partnership; or a player whose
strength is established against anything but one champion and itself.

On the matches the record is in three parts, and this page will not blur them.
In July 2026, over 1,152 mirrored games, **a heuristic champion beat the exact
perfect-information solver** — rob, playing the points objective — decisively, with
the whole deficit in the opening tricks and a dead heat from trick 3 on. In August
2026, under the same protocol and the same bar, **the seat beat that champion** —
walt, level 1, playing pmake — 630 of 1,152 games, losing points and winning
marks; that is an arena outcome at the exploratory tier, against one opponent,
with the auction removed, and is never a statement about exact values. In
September 2026 **the partner-aware candidate did not beat its reference**: a tie
against level 1 at five times the cost, a small deficit against the phone, and
two learners that trail their own tables. The honest reading of the three
together is neither "exactness wins" nor "exactness loses": the objective did the
first piece of work, the seat's mathematics — counted beliefs, certified regret,
the focal-horizon intervals — now says precisely how much of the opening is
unclaimed (at most 267‰ at one root, split unknown) and where the residual sits
(in the tail's policy, not in the fusion of models), and everything still to be
won lies in the part of the hand where exact solving is not yet affordable —
which is precisely where the mathematics on this page is aimed.

The other honest reading is about method. Every negative above — the identity
quotient, the refuted dimension payoff, the frontier explosion, the lost match,
the retracted doom gloss, the unaffordable waking seat, the tied partner
candidate, the overfit tables — was designed to be a result in advance, reported
at its declared verdict rather than at its most flattering angle, and carried
back to the mathematics instead of engineered around
([walt-negative-results](walt-negative-results.md)). That discipline is the asset.
The theorems are what the project has proved; the discipline is how it finds out
what it has not.
