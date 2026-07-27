# Straight Points-and-Marks Texas 42 — Normative Rules

## 0. Authority and interpretation

This document defines the Straight 42 rules profile used by the package.

The rules are stated first in prose. The mathematical foundation makes them
precise. When the prose determines one result, the mathematics clarifies it.
When materially different games remain compatible with the prose, this profile
makes an explicit adopted-rule or configuration choice.

Statuses used here:

- **Adopted rule** — normative for this profile.
- **Configuration** — a finite value supplied when a match is created.
- **Mathematical chance rule** — the stochastic law used by the formal model.
- **Clarification** — a precise consequence of an adopted rule.
- **Excluded** — not represented in this profile.
- **Boundary** — an administrative or illegal-play issue outside the legal
  game graph.

## 1. Equipment, seats, and partnerships

**Adopted rule R-SET-01.** The game uses one standard double-six set of 28
physical dominoes.

**Adopted rule R-SEAT-01.** Four players occupy four seats in clockwise order.

**Adopted rule R-TEAM-01.** Opposite seats are partners. Each player has one
partner and two opponents.

**Configuration R-CONFIG-01.** A match has a finite positive mark target
\(T\). The customary value is 7.

**Configuration R-CONFIG-02.** A match has a finite positive maximum mark bid
\(m_{\max}\). The rules do not privilege a particular value. Under the adopted
one-round progression, values at least five are behaviorally equivalent because
no bid above five marks is reachable.

## 2. Shaker and deal

**Adopted rule R-SHAKER-01.** One seat is the shaker for each deal attempt.

A physical table may select the first shaker by drawing one domino per player
and choosing the highest pip total, redrawing ties. The abstract game may
instead receive the first shaker as setup input. Once the shaker is fixed,
this setup choice does not alter the auction or play rules.

**Adopted rule R-DEAL-01.** The 28 dominoes are randomized face down and
allocated so that each player receives seven. There is no boneyard.

**Mathematical chance rule R-DEAL-02.** Unless another chance law is explicitly
selected, every ordered partition of the 28 dominoes into four labeled
seven-domino hands is equally likely.

**Mathematical chance rule R-DEAL-03.** Unless another cross-deal law is
explicitly selected, conditional on all pre-attempt history and every
non-deal latent variable, the next ordered deal is an independent draw from
R-DEAL-02. This includes a new deal after an all-pass attempt.

Each player privately observes that player's own seven-domino hand. Ownership
of every other unplayed domino is hidden.

## 3. Auction

### 3.1 Order

**Adopted rule R-AUC-01.** The player immediately left of the shaker acts
first. Bidding proceeds clockwise.

**Adopted rule R-AUC-02.** Each player acts exactly once in an auction attempt,
choosing either pass or one legal bid.

Every bid and pass, including its actor, is public and remains part of public
history.

### 3.2 Bid forms

**Adopted rule R-AUC-03.** A point bid is an integer from 30 through 41
inclusive. Write it as \(P(n)\), \(30\le n\le41\).

**Adopted rule R-AUC-04.** A mark bid is a positive integer number of marks not
exceeding \(m_{\max}\). Write it as \(M(m)\),
\(1\le m\le m_{\max}\).

**Clarification R-AUC-05.** One mark is a bid to take all 42 hand points. A
mark bid \(M(m)\) has a 42-point contract threshold and a stake of \(m\)
marks.

For comparison:

\[
P(30)<\cdots<P(41)<M(1)<M(2)<\cdots<M(m_{\max}).
\]

### 3.3 Legal progression

Let the current high bid be the most recent nonpass bid.

**Adopted rule R-AUC-06.** Every nonpass bid must exceed the current high bid.

**Adopted rule R-AUC-07.** Before any mark bid has been made, a player may bid
at most two marks, subject to \(m_{\max}\). Thus the first mark bid may be
\(M(1)\) or \(M(2)\) when configured.

**Adopted rule R-AUC-08.** Once a mark bid exists, a later mark bid may
increase it by exactly one mark. Thus \(M(3)\) may follow \(M(2)\), but may not
be the first mark bid.

**Clarification R-AUC-09.** Pass is legal at every incomplete auction node.

### 3.4 Auction result

**Adopted rule R-AUC-10.** If at least one nonpass bid is made, the last
nonpass bidder wins the auction at that bid.

**Adopted rule R-AUC-11.** If all four players pass, the deal is abandoned
without a contract or mark award. The next clockwise player becomes shaker,
the dominoes are randomized and dealt again, and a new auction attempt begins.

A forced-30 pass-out rule is a different profile.

### 3.5 Reachable ceiling and finiteness boundary

**Clarification R-AUC-12.** The largest reachable mark bid is

\[
\min(m_{\max},5).
\]

**Reason.** At most four players act. A first mark bid is at most \(M(2)\),
and each later mark overcall adds exactly one. The longest possible mark chain
is therefore

\[
M(2),M(3),M(4),M(5).
\]

Consequently every cap \(m_{\max}\ge5\) induces the same legal auction tree.
A larger configured cap does not silently change the opening ceiling or raise
rule.

Each auction attempt contains exactly four actions and only finitely many legal
actions at each node. Attempt finiteness therefore follows from the one-round
structure; it does not depend on the configured cap being the source of a
global finite action list.

Repeated all-pass attempts can occur without a deterministic bound. Therefore:

- an auction attempt is finite;
- every contracted hand is finite;
- a full match including arbitrary pass-outs is not a finite-horizon tree
  without an additional bound or termination assumption.

## 4. Declaration and contract

**Adopted rule R-DECL-01.** After winning the auction and before the first
play, the bidder publicly chooses exactly one declaration:

- blanks trump;
- ones trump;
- twos trump;
- threes trump;
- fours trump;
- fives trump;
- sixes trump;
- doubles trump;
- no-trump / follow-me.

These are the nine Straight 42 declarations. Every declaration is legally
available after every straight point or mark bid; declaration legality does
not depend on hand content.

**Adopted rule R-CONTRACT-01.** A point bid \(P(n)\) requires the bidder's
partnership to take at least \(n\) hand points and has a stake of one mark.

**Adopted rule R-CONTRACT-02.** A mark bid \(M(m)\) requires the bidder's
partnership to take all 42 hand points and has a stake of \(m\) marks.

**Adopted rule R-LEAD-01.** The winning bidder leads the first trick.

## 5. Suits under a declaration

A domino has stable physical identity. Its effective suit role depends on the
declaration.

### 5.1 Pip trump

**Adopted rule R-SUIT-01.** When pip \(p\) is trump, every domino containing
\(p\) belongs to the trump suit for that hand.

A trump domino has been called out of its other natural pip suit. With fours
trump, for example, `4-2` is trump and cannot satisfy a twos lead.

### 5.2 Doubles trump

**Adopted rule R-SUIT-02.** When doubles are trump, the seven doubles form the
trump suit. Mixed dominoes remain in their natural pip incidences.

Doubles cannot follow a natural pip lead, and mixed dominoes cannot follow a
doubles lead.

### 5.3 No-trump

**Adopted rule R-SUIT-03.** In no-trump/follow-me, no domino is called into a
trump suit and no domino has trump power.

A mixed domino remains able to follow either natural pip it contains.

## 6. Tricks

A contracted hand has seven tricks. Each trick contains one play by each
player.

### 6.1 Turn order

**Adopted rule R-PLAY-01.** Play proceeds clockwise from the trick leader.

**Adopted rule R-PLAY-02.** The winner of a trick leads the next trick.

### 6.2 Leading

**Adopted rule R-PLAY-03.** A leader may play any domino remaining in that
leader's hand.

**Adopted rule R-PLAY-04.** A called domino leads the called suit.

**Adopted rule R-PLAY-05.** An uncalled domino leads the natural suit of its
higher pip. Thus in no-trump a mixed domino leads its higher end.

### 6.3 Following

**Adopted rule R-FOLLOW-01.** A player must play a domino that follows the led
effective suit whenever at least one such domino remains in that player's
hand.

**Adopted rule R-FOLLOW-02.** A player with no domino that follows the led
effective suit may play any remaining domino, including trump.

### 6.4 Winning

**Adopted rule R-WIN-01.** If one or more trump dominoes are played, the
highest trump wins.

**Adopted rule R-WIN-02.** Otherwise, the highest domino that follows the led
effective suit wins.

**Adopted rule R-WIN-03.** An off-suit discard without trump power cannot win.

**Adopted rule R-RANK-01.** In a natural pip suit, the double is highest. Mixed
dominoes are ordered by their other end, equivalently by pip sum.

**Adopted rule R-RANK-02.** In pip trump, the trump double is highest and the
other trumps are ordered by their other end.

**Adopted rule R-RANK-03.** In doubles trump, doubles rank `6-6` high through
`0-0` low.

The mathematical foundation proves that these rules give a unique winner for
every trick of four distinct physical dominoes.

## 7. Hand scoring

**Adopted rule R-SCORE-01.** The five count dominoes are:

- `5-5`: 10 points;
- `6-4`: 10 points;
- `5-0`: 5 points;
- `4-1`: 5 points;
- `3-2`: 5 points.

They total 35 points.

**Adopted rule R-SCORE-02.** Every completed trick is worth one additional
point to the partnership that wins it.

**Adopted rule R-SCORE-03.** A trick's value is one plus the count value of
every domino in that trick.

**Adopted rule R-SCORE-04.** All seven tricks are played. Final hand points are
the sum of the seven trick awards. Seven trick points plus 35 count points give
exactly 42 hand points.

## 8. Contract settlement

Let \(P_D\) be the declaring partnership's final hand points.

**Adopted rule R-SETTLE-01.** A point contract \(P(n)\) is made exactly when
\(P_D\ge n\).

**Adopted rule R-SETTLE-02.** A mark contract \(M(m)\) is made exactly when
\(P_D=42\).

**Clarification R-SETTLE-02A.** In this full-play profile, \(P_D=42\) is
equivalent to the declaring partnership winning all seven tricks. Winning all
tricks awards every one-point trick base and every count domino. Losing any
trick gives the defenders at least one point and makes 42 impossible. Thus the
42-point threshold and traditional sweep wording define the same success
event.

**Adopted rule R-SETTLE-03.** If the contract is made, the declaring
partnership receives the stake.

**Adopted rule R-SETTLE-04.** If the contract is set, the defending partnership
receives the stake.

The stake is one mark for a point bid and \(m\) marks for \(M(m)\).

## 9. Match progression

**Adopted rule R-MATCH-01.** A match begins at marks \((0,0)\). Marks
accumulate by partnership across contracted hands.

**Adopted rule R-MATCH-02.** A partnership wins as soon as its mark score
reaches or exceeds \(T\).

**Adopted rule R-MATCH-03.** If the match has not ended after a contracted
hand, the clockwise successor of the current shaker becomes shaker and a new
deal attempt begins.

Every contracted hand awards at least one mark to exactly one partnership.
Ignoring all-pass attempts, a match from \((0,0)\) to target \(T\) ends after
at most \(2T-1\) contracted hands.

## 10. Information and communication

**Adopted rule R-INFO-01.** Each player privately observes that player's own
hand on every deal attempt, including a deal later abandoned after four
passes.

**Adopted rule R-INFO-02.** The base public event stream contains:

- match and deal-attempt boundaries, with shaker and seat order;
- every bid and pass, with actor;
- the declaration, with actor;
- every played domino and actor, in order.

**Clarification R-INFO-02A.** All-pass, winning bidder and bid, acting seat,
current trick order, led suit, legal trick winner, trick points, cumulative
hand points, contract settlement, match marks, and match result are
deterministic functions of the base event stream, setup/configuration, and the
rules. An implementation may materialize these as validated public facts or
redundant lifecycle events. Doing so is information-preserving but does not
create a second source of truth or require a spoken count announcement.

**Adopted rule R-INFO-03.** Players have perfect recall. Within one deal they
remember their dealt hand and all public events. Across a match they remember
the ordered sequence of their privately observed hands and the complete public
history.

**Adopted rule R-COMM-01.** No private communication, table talk, physical
signal, or external side channel about hidden hands is part of the game model.

## 11. Legal-game boundary

The mathematical game contains legal actions only.

Renege, out-of-turn play, exposed-domino penalties, touch-move administration,
timing rules, and tournament discipline are not legal transitions in this
profile. A physical tournament may adjudicate them through a separate rules
layer.

## 12. Excluded contracts

Nello, plunge, splash, sevens, and all other special contracts are excluded.

This exclusion is structural. Such contracts can change:

- declaration semantics;
- trick objective;
- active-player count;
- who leads;
- legal bid eligibility;
- what private-hand facts become rule-visible;
- the form of exact information support.

They must not be added by toggling a flag on the Straight 42 object without
re-deriving the affected mathematics.
