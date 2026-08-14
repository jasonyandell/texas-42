# The Game of 42, Mathematically

[Home](Home.md) · owns: the human-facing introduction to straight Texas 42 as a
mathematical object — what the game is, what is proved, what is measured, and what
the machinery can now do · Sources: [rules-profile](rules-profile.md),
[FINDINGS](FINDINGS.md), [claim-ledger](claim-ledger.md), and the pages linked
throughout; every claim carries the tier of the page that owns it. Related:
[rob](rob.md), [lean](lean.md), [walt hub](walt.md), [lineage](lineage.md).

This page is the doorway. It is written for someone technically literate who has
never played 42 and never read a game-theory paper, and who wants an honest
account rather than a pitch. Terms of art are explained where they first appear;
nothing is stated more strongly than the page that owns it states it; and where
the evidence is weak or contested, this page says so in the same sentence as the
result. Deep pages are linked rather than summarised.

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
idea pages, the probes, and field measurements against outside opponents. Those
are labelled as such wherever they appear and may not be quoted as results.

One more rule, applied literally below: **dissents travel with results**. Where an
external panel was not unanimous, this page says so in the same breath as the
number.

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
([support-fiber](support-fiber.md), CELL-05 — corpus theorem, now also
kernel-proved in Lean).

The proof pivots on a nice observation. When a hidden player follows suit you
learn nothing positive you did not already see — the played tile is its own
witness that they could follow. The durable information is *negative*: a failure
to follow tells you permanently that a seat holds none of that suit. So
hidden-information tracking in 42 is monotone; it only ever deletes possibilities,
never restores them, and the total is capped at 63 deletions per hand
([support-dynamics](support-dynamics.md)). Three consequences make an efficient
exact implementation obviously possible rather than merely hoped for: consistency
is a seven-check condition, counting the fiber exactly needs no enumeration and at
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
tests: passing every check we know how to write does not make a state reachable.
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
we meant. The second is a real condition — a formalization can be perfectly proved
and about the wrong object — and the project's answer is its trust boundary rule:
an external `PASS` is never imported as an axiom, and finite claims enter the
kernel only by direct proof, a proved-sound decision procedure, or proved
reflection. What it does not guarantee: the priority-1 rows are not covered; the
two big exhaustive receipts (the 737,100-case rule-reader agreement, the auction
census) are deliberate later reflection targets; and none of the exchange-tier
reachability numbers are kernel-proved.

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
agreeing on an exact integer is genuinely informative and still not a proof.

Beyond corpus and kernel, the project runs an adversarial exchange with a separate
strong reasoning model, whose results form their own tier. Standing results: the
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
because measuring is how the project decides what to try to prove next.

The measurements come from **walt**, the seat-level player program — the attempt
to play 42 from one chair, seeing only what a chair legally sees, on top of the
exact machinery ([walt](walt.md), [walt/LOG.md](../walt/LOG.md)). Two scoping
rules apply throughout. Counts are **carrier-relative**: counts over a specific
declared representation at specific probe coordinates, not global facts about 42.
And where a probe declares a stopping criterion in advance, this page reports the
formal verdict, not the encouraging part of the texture.

Three pieces of walt's own vocabulary appear below and are worth having in hand.
A **coordinate** is one fixed starting point for a probe — a particular hand, in a
particular declaration, with a particular seat to act; probes run at a handful of
them and every number is relative to the ones chosen. A **grade** is how far the
probe looks ahead, counted in decision layers, so grade 3 is deeper and vastly
larger than grade 1; the fibers behind the three grades measured below contain 6,
90 and 1,680 worlds respectively. And a **policy** is a complete rule for playing
out the rest of the hand from a position — not a single move, but an answer for
every situation that could follow. Counting policies is how the decision side of
the game gets measured.

### 5.1 About half of mid-game free decisions do not matter

The question, in Jason's framing, was "junk everywhere": can a seat cheaply detect
that no choice available to it changes any outcome, so search can skip the
branching entirely? Measured at census scale across 45 probe units: of 49,522,677
decision sites that ground truth could classify, **25,255,316 — 51% — are
one-deviation ties.** Changing the move changes nothing about the outcome.

### 5.2 Detecting indifference is essentially free, and provably never wrong

Three cheap bit-level detectors were each *proved* one-sided before being run:
each fires only when genuinely certain, and stays silent otherwise. Over
**174,250,255 detector calls with 27,980,333 fires there were zero false
positives** — every site a detector called dead, and that ground truth could
classify, was genuinely indifferent. Cost was about 25 nanoseconds per call
against solve arms measured in tens of thousands of milliseconds, so detection is
effectively free. (That timing was taken under contention and is explicitly not
quotable as a benchmark; the clean instrument has not been run.)

Reported recall is 8,335,057 of the 25,255,316 ties, about 33% — and that figure
**understates the detectors**, because the ground truth compared against is
one-deviation indifference, a *superset* of the strict deadness the detectors are
proved to certify. Measuring a narrow correct thing against a wider yardstick
makes it look weaker than it is. One detector never fired at all: its triggering
condition does not arise at these coordinates. And one family of positions, the
"trumpless junk" case, has hundreds of ties per lead and zero detector hits; its
tie mechanism is **unidentified**, and the standing rule is that no fourth
detector is written without a proof first. That family sits at the same probe
coordinate as the two uncompletable frontiers of the next section — the one place
in these measurements where 42 is genuinely tense, seen once from the
indifference side and once from the strategy side.

### 5.3 Value is rich; the decision is often trivial

Two probes landed opposite-looking results in the same week, and read together
they are the most interesting thing walt has measured.

The first asked whether the *values* a position can produce compress. Give each
policy the list of values it achieves, one entry per possible world — at grade 3
that is a list of 1,680 numbers — and ask how many independent directions those
lists span. If the answer were small, a seat could carry a short summary instead
of the whole position, because everything else would be a combination of what it
already had. The answer is not small: the span came out between 1,461 and 1,680
directions out of a possible 1,680, with one coordinate at the full 1,680. So the
pre-declared criterion for the approach paying off was **refuted**. Value in 42
is genuinely high-dimensional, and the reason is structural — every tile is
eventually played and publicly attributed, so a complete record of how a hand
finished pins down what everyone held.

The second probe asked about the *decision* side instead: of all the policies
available at a position, how many are worth considering at all? A policy is
dismissible if some other policy does at least as well in every single world and
strictly better somewhere; the survivors of that test are the ones a player could
ever have a reason to choose. At **7 of 9** measured position-and-opening-lead
pairs, exactly **one** policy survives — it weakly dominates every lawful
alternative in all 1,680 worlds — against raw policy counts as large as 2^19930.
Under all that combinatorial weight sits a single playbook.

But the probe's formal verdict is **STOPPED, not 7-of-9 success**, and the
distinction matters. The other 2 of 9 pairs — both openings that lead a low trump
rather than the boss trump, at the one coordinate where the game is genuinely
tense — ran past the declared limit on how large a survivor set the probe would
track, twice. A partially computed survivor set bounds nothing at all, and under
the pre-declared discipline one uncompleted coordinate forbids the global claim,
so the probe reports no verdict. The texture is the finding: total collapse
almost everywhere, genuine explosion exactly where 42 is hard. Read with the
first probe, **value richness and decision simplicity coexist** — the space of
outcomes is enormous while the right move is usually obvious.

### 5.4 Root actions can be certified — and compression buys nothing at the start

**Three exact root-action certifications** were obtained: at three probe
coordinates the chosen opening was proved at least as good as every alternative,
by sandwiching a lower witness (the exact value of one fixed lawful policy)
against an upper witness (an action-conditioned relaxation) for every competitor.
One of them covers precisely the two leads whose undominated-policy frontiers the
previous probe could not finish — proving the frontier unnecessary for the root
decision. Seven of nine per-action information prices came out exactly zero; the
only two nonzero prices sit exactly at the two frontier-explosion leads.

Separately, the **seat census** question — how many genuinely distinct situations
does a seat face at the very first play? — was answered by proof rather than
enumeration, and the answer is discouraging in an illuminating way. The structural
symmetry quotient at the first play is the **identity**: no two distinct starting
hands can be identified. The count is therefore exactly **C(28,7) = 1,184,040**,
about 11.84× over the declared target of order 10^5, with the only available
symmetry an exact 7:1 fold across the seven pip declarations. The insight
generalises: **structural compression is bought with deadness.** Later in a hand
tiles are spent and contexts inert, and large collapses become available — one
measured level went 55 million states to 32,532 forms to 64 classes. At the first
play nothing is dead yet, so nothing merges. The identity quotient is not a
failure of technique; it is the statement that this notion of sameness is too fine
for the question. Whether a *coarser* lawful notion reaches a manageable size is
explicitly open.

### 5.5 First contact: the exact solver has been beaten

In July 2026 the engine was seated against the champion of the predecessor project
— a strong heuristic that averages a perfect-information evaluation over ten
sampled consistent worlds and takes the best move. This is **field-measurement
tier**, computed by the other project's arena code, not by certified Rust, and it
carries zero evidentiary weight for the mathematics ([field/](field/Home.md)).

Over 1,152 mirrored games and 12,866 hands, with the auction removed so both
players faced identical exogenous contracts, **the champion won**, at about 6.5
standard deviations. Paired on 6,028 mirrored deals, the exact solver made 32.4%
of its contracts against the champion's 36.1% *on the same deals* — a gap of
−3.65 percentage points, McNemar z = −6.52 — negative in all seven declarations
and on raw points, which was its own objective.

The same hands were then re-run with a takeover: a shared heuristic plays all four
seats through the opening, the position is frozen, and it is played out twice from
the byte-identical state, once with the solver's team declaring and once
defending. From trick 3 onward, across 768 paired positions, the two are
**statistically indistinguishable** — a pooled make-rate gap of −0.4 ± 1.7
percentage points, points agreeing to tenths. Together the two results localise
the entire deficit to the first two or three tricks, which the engine plays with a
fast stand-in never intended to survive and slated for replacement rather than
tuning. A side effect worth recording: the bridge asserted the engine's
independently derived trick leader and team score against the other project's
engine on every decision — **about 180,000 decisions, zero divergences** between
two independently written implementations of the rules.

The caveats travel. The forced-contract protocol is not a realistic distribution
of contracts; a dead heat against a strong baseline bounds the distance between
two players, not either one's distance from optimal; and "exact" here means exact
best response *given a model of the opponent*, so when it loses, the model lost,
not the solve.

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
cites; inventoried at [walt-math-open-questions](walt-math-open-questions.md):

- **A lawful notion of "same situation" that makes the seat's world manageable.**
  The structural notion is the identity at the first play; the linear one saturates
  by the third grade. Whether a dynamics-based or value-based notion reaches a
  workable size is open — and the two obvious candidates have been measured and
  found wanting.
- **The trumpless-junk tie mechanism.** Half of mid-game decisions at the measured
  coordinates are ties, and one identifiable family of them resists every proved
  detector. Nobody knows why those positions are indifferent.
- **Whether a strong plan can be transported rather than recomputed.** A policy
  provably right at one position is, so far, only a *seed* elsewhere — a heuristic
  for finding a witness, never a verdict. "This policy was dominant over there"
  currently has no status here.

---

## 7. What we can do with 42 now

An honest inventory, separating what exists today from what is a direction.

**Today.**

- **Solve an endgame exactly from a seat.** Given a position, the exact fiber and a
  stated utility, the machinery computes the exact best response over the whole
  information set — no sampling, no floating point, no impossible worlds. The
  practical wall is around trick 3, where fibers reach hundreds of thousands of
  worlds; how long a decision takes there is a matter of observed wall-clock on
  particular runs rather than a claim this project makes, since no result in it
  supports a cost or tractability claim.
- **Certify a root action against every alternative** (exploratory tier). Three
  such certifications exist: the move is proved at least as good as each
  competitor without enumerating the space of plans — which matters, because at
  those positions that space runs to 2^19930. Note the word: this is walt's own
  sense of *certification*, an exact separation of one action from every
  competitor, and deliberately not the sense fenced in §4.4 above; nothing in it
  is identity-bearing.
- **Detect that a decision does not matter, at negligible cost** (exploratory
  tier). Proved-sound one-sided detectors with zero false positives over 174
  million calls. Their per-call cost is measured only under contention and is not
  quotable as a benchmark; what is established is the direction, that detection
  costs orders of magnitude less than the solve it displaces.
- **Price information exactly** (exploratory tier). For a given action you can
  compute exactly what it would be worth to be told the hidden world; across the
  nine measured action prices seven are exactly zero, and the two nonzero ones sit
  precisely where the strategy is genuinely tangled.
- **Inspect any position's exact fiber, and ask counterfactuals.** The engine ships
  an HTML inspector that steps through a game from any seat's perspective, showing
  the exact fiber count, the exact marginals, the plan tree, and the exact best
  value for every legal opening — chosen and rejected alike, so "why not the other
  tile?" is answerable rather than rhetorical. Every number is computed by the
  certified Rust and emitted; the display recomputes no game logic, so it cannot
  manufacture a fact or leak information the viewer does not hold
  ([analysis](analysis.md)).
- **Play a full seat and measure it honestly against a strong opponent**, with a
  mirrored-pairs protocol whose takeover knob turns "why did it lose?" into a
  bisection rather than a debate — and **re-verify the rules against an independent
  implementation** continuously, as a free side effect of playing.

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
  play seeds the candidate library; exact evaluation certifies.** A library carries
  policies, never verdicts. Its one robust property is that a policy's validity as
  a lower witness depends only on its lawfulness, not on the valuation under which
  it was discovered — so a candidate found in a simplified setting survives into
  the full scored game as a witness, even though its quality verdict does not. A
  related idea — reading a bid as a lower-bound claim about a hand, the same shape
  as a witness — has been named and, on the current count-free carrier,
  **explicitly declined** with its reason: a bid asserts something about points and
  marks that the count-free machinery cannot yet express
  ([walt/ECONOMY-SUCCESSOR.md](../walt/ECONOMY-SUCCESSOR.md) §2.4). Nothing here is
  built; it is a frame with two measurements under it.
- **Choosing which worlds matter.** Trying every world is wasteful and most do not
  matter — but selection coupled to the solver's own output can bias the answer,
  which is why it has not been attacked. As exactness-preserving pruning over world
  *classes* rather than raw worlds, it would open new classes of search
  ([idea-hierarchical-fibers](idea-hierarchical-fibers.md)).
- **Working backwards.** Backward induction seeded from canonical arrangements of
  the last trick rather than from concrete deals — "the pips don't matter, the
  relationships do" ([idea-retrograde-rank](idea-retrograde-rank.md)).
- **Beliefs, then partnership.** Everything above is the belief-free slice; the
  measurements deliberately deleted the auction to make two players comparable.
  Bidding, inference from bids, and partnership convention are where the real game
  is played, and the support-versus-belief separation is the vocabulary for
  approaching them honestly, not a solution to them.

---

## 8. Where this stands

What the project has: a proved, machine-checked account of what a 42 player can
exactly know; a canonical smallest form for that knowledge; an exact and cheap way
to count and sample the possibilities it leaves open; a proof that the knowledge
alone is not enough to play by; an engine that reproduces every number
independently and is byte-compared on every change; and a growing body of exact
measurement about which decisions in 42 actually matter.

What it does not have: the exact size of the reachable state space, a compression
of the seat's situation down to a workable size, a treatment of bidding or
partnership, or a player that beats the strongest opponent it has faced. On that
last point the record is unambiguous and this page will not soften it: over 1,152
mirrored games, **a heuristic champion beat the exact solver**, decisively. The
localisation is the interesting part — from the point where the exact machinery
takes over, the two are a dead heat, and the whole deficit sits in the opening
tricks the engine currently plays with a placeholder. So the honest reading is
neither "exactness wins" nor "exactness loses": exactness, applied to the part of
the game it can currently reach, is worth about as much as the best heuristic
anyone has built, and everything still to be won lies in the part of the hand
where exact solving is not yet affordable — which is precisely where the
mathematics on this page is aimed.

The other honest reading is about method. Every negative above — the identity
quotient, the refuted dimension payoff, the frontier explosion, the lost match —
was designed to be a result in advance, reported at its declared verdict rather
than at its most flattering angle, and carried back to the mathematics instead of
engineered around. That discipline is the asset. The theorems are what the project
has proved; the discipline is how it finds out what it has not.
