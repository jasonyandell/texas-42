# Executable Specification

## 0. Purpose

This document translates the normative rules and mathematical foundation into
computing contracts.

It is intentionally direct, explicit, exact, unoptimized, language-neutral,
and hostile to hidden approximation. It is not a server engine, model input
format, packed state, database schema, or performance architecture.

## 1. Semantic rules

1. Every executable object corresponds to a named mathematical object.
2. Distinct mathematical domains receive distinct types.
3. Constructors validate invariants.
4. Operations return exact results or fail explicitly.
5. No method silently samples, caps, truncates, canonicalizes, or drops
   information.
6. A caller-supplied predicate is explicit in the type or call.
7. Enumeration may exhaust resources.
8. Resource failure does not change semantics.
9. Derived data may be cached only when equality with its defining function is
   preserved.
10. Global domino identity is stable; local storage order has no game meaning.
11. A complete initial deal is never silently substituted for a current
    hidden-hand assignment.
12. A path-free mechanical state is never called a perfect-recall information
    state without a separate theorem.

## 2. Core finite types

```text
Pip            := integer in [0, 6]
Domino         := canonical unordered pair (high, low), 0 <= low <= high <= 6
DominoId       := integer in [0, 27] under one documented bijection
Seat           := integer modulo 4
Team           := integer modulo 2
LedSuit        := Pip | CALLED
Declaration    := PIP_TRUMP(Pip) | DOUBLES_TRUMP | NO_TRUMP
PointAmount    := integer in [30, 41]
MarkAmount     := positive integer
MatchTarget    := positive integer
```

Required canonical identity order for the first implementation:

```text
(0,0),
(1,0), (1,1),
(2,0), (2,1), (2,2),
...
(6,0), ..., (6,6)
```

Identifier magnitude never determines game rank.

## 3. Rules configuration

```text
RuleConfig:
    maxMarkBid: positive integer
    matchTarget: positive integer
    allPassRule: RESHAKE_NEXT
    dealLaw: UNIFORM_ORDERED_DEALS
    crossDealLaw: INDEPENDENT_DEALS
```

Invariants:

```text
maxMarkBid >= 1
matchTarget >= 1
```

Derived value:

```text
reachableMaxMarkBid(config) = min(config.maxMarkBid, 5)
```

No default numerical cap is mathematically required. Under the adopted
one-round auction progression, every configured cap at least five yields the
same legal auction tree; bids above five remain valid members of the configured
bid domain but are unreachable. Reaching them requires a different auction
profile.

## 4. Domino universe

```text
DominoUniverse:
    dominoes: immutable ordered collection of 28 unique Domino values
    idOf(domino) -> DominoId
    dominoOf(id) -> Domino
    contains(id, pip) -> bool
    isDouble(id) -> bool
    pipSum(id) -> int
    highPip(id) -> Pip
    countPoints(id) -> 0 | 5 | 10
```

Invariants:

```text
len(dominoes) == 28
idOf(dominoOf(id)) == id
dominoOf(idOf(domino)) == domino
sum(countPoints(d) for d in dominoes) == 35
```

## 5. Declaration algebra

```text
DeclarationAlgebra:
    declaration: Declaration
    called: FrozenSet[DominoId]
    powered: FrozenSet[DominoId]

    effectiveSuits(domino) -> nonempty FrozenSet[LedSuit]
    ledSuit(domino) -> LedSuit
    follows(domino, ledSuit) -> bool
    rank(domino) -> Rank
    tier(domino, ledSuit) -> 0 | 1 | 2
    trickKey(domino, ledSuit) -> comparable key
    beats(ledSuit, currentWinner) -> FrozenSet[DominoId]
    threat(domino) -> FrozenSet[DominoId]
    resolveTrick(ordered four plays) -> TrickResult
```

Required identities:

```text
follows(d, q) == (q in effectiveSuits(d))
called dominoes have effectiveSuits(d) == {CALLED}
ledSuit(d) == CALLED iff d in called
tier(d, q) == 2 iff d in powered
trickKey(d, q) == (0, 0) iff tier(d, q) == 0
```

`rank(domino)` is declaration-relative and total:

```text
DOUBLES_TRUMP double p-p: p
any other double: TOP
mixed domino: pip sum
```

Its value is ignored for tier zero.

`resolveTrick`:

- receives exactly four distinct dominoes with actors in clockwise order;
- derives led suit from the first domino;
- requires a unique maximum trick key;
- returns winning actor and
  `1 + sum(countPoints(each played domino))`.

Duplicate dominoes, malformed actor sequence, or wrong trick length are
errors.

## 6. Complete deal worlds and chance

```text
DealWorld:
    initialHands: mapping Seat -> FrozenSet[DominoId]
```

Invariants:

```text
each hand has exactly 7 dominoes
hands are pairwise disjoint
union of hands is the whole universe
```

```text
UniformOrderedDealLaw:
    probability(deal) -> 1 / orderedDealCount
    orderedDealCount -> 28! / (7!)^4
```

```text
IndependentDealSequenceLaw:
    conditionalProbability(nextDeal, preAttemptHistory, nonDealLatent)
        -> UniformOrderedDealLaw.probability(nextDeal)
    finitePrefixProbability(
        finiteDealSequence,
        preSequenceHistory,
        nonDealLatent
    ) -> product of per-attempt conditional probabilities
```

For an unbounded sequence of attempts, the law is specified by its conditional
kernels, equivalently by consistent probabilities of finite-prefix cylinder
events. It does not expose `probability(infiniteSequence)` as a product of
point probabilities. A concrete shuffle is an implementation of a chance law,
not the law's definition.

## 7. Auction

```text
Bid(config) :=
    PASS
    | POINT(value: PointAmount)
    | MARK(value: MarkAmount where value <= config.maxMarkBid)

AuctionState:
    shaker: Seat
    actions: ordered tuple of at most four (Seat, Bid)

AuctionWin:
    bidder: Seat
    winningBid: POINT(PointAmount) | MARK(MarkAmount)
    completedAuction: AuctionState

ObjectiveDealAttempt:
    dealAttemptIndex: nonnegative integer
    deal: DealWorld
    auction: AuctionState
```

Derived operations:

```text
nextActor(state) -> Seat | None
currentHighBid(state) -> nonpass Bid | None
legalBids(state, config) -> FrozenSet[Bid]
reachableMaxMarkBid(config) -> min(config.maxMarkBid, 5)
isComplete(state) -> bool
result(state) -> ALL_PASS | AuctionWin
```

Required behavior:

```text
nextActor = shaker + 1 + len(actions) mod 4
actors occur in exactly that prefix order
PASS is legal at every incomplete node
nonpass bids strictly exceed current high bid
before a mark bid: MARK(1) and MARK(2) may be legal, subject to cap
after MARK(r): only MARK(r+1) may overcall, subject to cap
after four actions: complete
last nonpass bid wins
four passes produce ALL_PASS
```

The largest reachable mark bid is `min(maxMarkBid, 5)`. Configurations with
caps 5, 6, 7, or any larger value have identical legal auction histories under
this profile. One attempt is finite because it has depth four and every reachable node
has only finitely many legal continuations. This remains true even for an
abstract uncapped mark domain; no implementation may generate an unbounded
irrelevant bid list.

`ALL_PASS` is not a contract. The match controller advances from the current
shaker to its clockwise successor, records the hand that each player privately
observed in the abandoned attempt, and requests a new deal world.

## 8. Contract

```text
Contract:
    bidder: Seat
    winningBid: POINT(PointAmount) | MARK(MarkAmount)
    declaration: Declaration

    declaringTeam() -> Team
    threshold() -> int
    stake() -> int
```

Definitions:

```text
POINT(n): threshold=n, stake=1
MARK(m): threshold=42, stake=m
```

```text
settle(contract, finalPointsByTeam) -> HandAward
```

Invariant:

```text
finalPointsByTeam[0] + finalPointsByTeam[1] == 42
```

The declaring team receives the stake iff declaring points meet the threshold;
otherwise the defending team receives it.

A legal `Contract` is created through:

```text
contractFromAuction(
    win: AuctionWin,
    declaration: Declaration,
    config: RuleConfig
) -> Contract
```

The constructor validates that `completedAuction` is a complete legal auction
under `config`, that `win` is its actual result, and that the declaration lies
in the nine-declaration Straight domain. A raw record with plausible fields is
not evidence that its bid was reachable.

In a certified full-play hand:

```text
contract.winningBid is MARK(m)
    implies
(contract made iff declaring partnership won all seven tricks)
```

This follows because every trick is worth at least one point and all 42 points
are awarded. The executable contract may use the 42-point threshold; the sweep
predicate must agree exactly.

## 9. Full location state

```text
Location :=
    IN_HAND(Seat)
    | CURRENT_TRICK(position: 0..3)
    | COMPLETED_TRICK(trickIndex: 0..6, position: 0..3)

CompletedTrick:
    trickIndex: 0..6
    leader: Seat
    plays: ordered tuple[Play] of length 4
    result: TrickResult

FullLocationState:
    locationOf: mapping DominoId -> Location
    leader: Seat
    currentTrick: ordered tuple[Play]
    completedTricks: ordered tuple[CompletedTrick]
    handPoints: pair[int, int]
    contract: Contract
```

Invariants:

```text
every DominoId has exactly one location
completed trick indices are 0..len(completedTricks)-1
first completed trick leader == contract.bidder
each later completed trick leader == previous result.winner
completed/current play actors equal leader + position mod 4
all recorded plays use distinct dominoes and agree with locationOf
each completed result exactly recomputes from its four plays
current leader == contract.bidder when no trick has completed
current leader == last completed winner otherwise
banked partnership points equal completed trick awards
```

This state preserves exact physical location, completed-trick order, and public
actor attribution. It is not the same object as the reduced contracted-play
state.

## 10. Reduced contracted-play and match states

```text
Play:
    actor: Seat
    domino: DominoId

ContractedPlayState:
    contract: Contract
    remainingHands: mapping Seat -> FrozenSet[DominoId]
    leader: Seat
    currentTrick: ordered tuple[Play] of length 0..3
    handPoints: pair[int, int]
    phase: PLAY | HAND_COMPLETE
```

```text
MatchState:
    marks: pair[int, int]
    shaker: Seat
    target: positive integer
    phase: NEED_DEAL | AUCTION | PLAY | MATCH_COMPLETE
```

Let

```text
r = sum(len(remainingHands[s]) for s in Seats)
j = len(currentTrick)
p = 28 - r
t = (p - j) / 4
```

Core contracted-play structural invariants:

```text
0 <= j <= 3
p >= j
p - j is divisible by 4
0 <= t <= 7
current-trick actors are leader + position mod 4
remaining hands are pairwise disjoint
current-trick dominoes are distinct and outside all remaining hands
for each seat s:
    len(remainingHands[s])
      == 7 - t - indicator(s appears in currentTrick)
completedDominoes
  == wholeUniverse - union(remainingHands) - currentTrickDominoes
sum(handPoints)
  == t + sum(countPoints(d) for d in completedDominoes)
handPoints are nonnegative
```

Phase invariants:

```text
phase == HAND_COMPLETE iff r == 0 and j == 0 and t == 7
when phase == HAND_COMPLETE: sum(handPoints) == 42
when phase == PLAY: current actor = leader + j mod 4
```

These algebraic invariants establish structural consistency, not arbitrary
state reachability or the partnership attribution of every completed trick.
Use distinct executable types:

```text
UncertifiedContractedPlayStructure:
    raw fields satisfying only displayed structural invariants

ReachableContractedPlay(state): Proposition
    # true exactly when some valid contracted-hand origin and legal public
    # play prefix replay to `state`

ReachableContractedPlayState:
    proof-irrelevant certified subtype
        { state: ContractedPlayState // ReachableContractedPlay(state) }

ContractedHandOrigin:
    dealAttemptIndex: nonnegative integer
    shaker: Seat
    preHandMarks: pair[int, int]
    matchTarget: positive integer
    deal: DealWorld
    auctionWin: AuctionWin
    declaration: Declaration

ContractedPlayReachabilityWitness:
    origin: ContractedHandOrigin
    actorAttributedLegalPlayPrefix: tuple[Play]
    claimedState: ContractedPlayState
```

A standalone constructor that accepts raw fields must return the uncertified
type. Structural validation alone may not prove `ReachableContractedPlay`.
`beginContractedPlay`, exact replay validation, and `applyPlay` establish or
preserve the proposition. The proof is not identity-bearing data: two certified
values with the same projected `ContractedPlayState` are the same semantic
state even when validated by different origins, histories, or proof terms.

A replayable witness may be retained as a separate audit artifact. It must not
be serialized into the semantic state, used by transition logic, exposed to a
player, or included in equality or hashing. In a proof assistant, use a
proof-irrelevant proposition/subtype. In a host language without proof
irrelevance, the wrapper must define observation, equality, hashing, and
serialization solely through the projected physical state.

Match-state invariants:

```text
target >= 1
marks are nonnegative
if phase != MATCH_COMPLETE: both marks < target
if phase == MATCH_COMPLETE: exactly one partnership has marks >= target
```

The contracted-play state is sufficient for future physical play within the
hand. `MatchState.shaker` is additionally required to continue after settlement
or all-pass. Under the baseline independent-deal chance law, these objects also
determine the objective next-attempt chance kernel and public phase transition.
A different cross-deal chance law may require additional retained state.

### 10.1 Objective lifecycle constructors

Every lifecycle operation returns its state result together with the primitive
public event or events it creates, and `beginDealAttempt` also returns the four
seat-indexed private deal observations. The signatures below suppress that
common result wrapper for readability. Event production is explicit return
data, never an untracked side effect.

```text
beginDealAttempt(
    matchState,
    deal: DealWorld,
    dealAttemptIndex
) -> (ObjectiveDealAttempt, MatchState)
```

Preconditions and result:

```text
matchState.phase == NEED_DEAL
deal is valid
attempt.auction.shaker == matchState.shaker
attempt.auction.actions is empty
returned matchState.phase == AUCTION
marks, shaker, and target are unchanged
```

The transition emits `DEAL_STARTED`; each player's information-state builder
records that player's own hand as a private deal observation.

```text
applyAuctionAction(attempt, bid, config) -> ObjectiveDealAttempt
```

This validates the next actor and membership in `legalBids` and appends exactly
one public `BID` event.

```text
closeAuction(attempt, matchState, config) ->
    ALL_PASS_TRANSITION(nextMatchState)
    | PENDING_DECLARATION(attempt, win: AuctionWin)
```

For all-pass, `nextMatchState.shaker = matchState.shaker + 1 mod 4` and phase is
`NEED_DEAL`; no marks are awarded. Otherwise the result retains the same deal
and a certified `AuctionWin`.

```text
beginContractedPlay(
    pendingDeclaration,
    declaration,
    matchState,
    config
) -> (ReachableContractedPlayState, MatchState)
```

Preconditions and result:

```text
matchState.phase == AUCTION
pending deal attempt shaker == matchState.shaker
win is certified from that attempt
contract = contractFromAuction(win, declaration, config)
remainingHands == pending deal.initialHands
leader == contract.bidder
currentTrick is empty
handPoints == (0, 0)
play phase == PLAY
returned matchState.phase == PLAY
marks, shaker, and target are unchanged
```

This transition emits the public `DECLARATION` event. These constructors are
the normative source of reachable objective states; lifecycle state may not be
inferred merely from matching field values.

## 11. Legal play and objective transition

```text
legalPlays(state: ReachableContractedPlayState) -> FrozenSet[DominoId]
```

Pseudocode:

```text
actor = leader + len(currentTrick) mod 4
hand = remainingHands[actor]

if currentTrick is empty:
    return hand

q = algebra.ledSuit(currentTrick[0].domino)
followers = {d in hand | algebra.follows(d, q)}
return followers if followers is nonempty else hand
```

```text
applyPlay(
    state: ReachableContractedPlayState,
    domino
) -> ReachableContractedPlayState
```

Pseudocode:

```text
require state is a valid ReachableContractedPlayState
require state.phase == PLAY
actor = currentActor(state)
require domino in legalPlays(state)

remove domino from actor hand
append Play(actor, domino) to current trick

if trick length < 4:
    return updated state

result = algebra.resolveTrick(trick)
add result.points to winning partnership
clear trick
set leader = result.winner

if all remaining hands empty:
    require hand points sum to 42
    phase = HAND_COMPLETE
```

The primitive transition always plays all 28 dominoes. No make/set early
termination occurs in `applyPlay`.

## 12. Hand settlement and public match transition

```text
completeHand(
    playState: ReachableContractedPlayState,
    matchState
) -> (HandResult, MatchState)
```

Preconditions:

```text
matchState.phase == PLAY
playState is certified reachable from beginContractedPlay for this same deal
attempt, shaker, match target, and pre-hand mark score
playState.phase == HAND_COMPLETE
all remaining hands empty
current trick empty
hand points sum to 42
contract and declaration equal the certified auction/declaration result
```

A supplied `FullLocationState` witness must project exactly to `playState` and
contain seven completed tricks. Raw structural fields without a proof of reachability are not accepted for
settlement.

Settlement awards the contract stake. If the updated mark score reaches the
target, phase becomes `MATCH_COMPLETE`. Otherwise:

```text
next shaker = current matchState.shaker + 1 mod 4
phase = NEED_DEAL
```

An all-pass attempt performs the same shaker increment and phase transition but
awards no marks. The previously observed private hand remains in each
player's match-global information record.

## 13. Public history and perfect-recall information

The base public event stream stores setup/chance boundaries and player
actions directly:

```text
BasePublicEvent :=
    MATCH_STARTED(target, initialShaker)
    | DEAL_STARTED(dealAttemptIndex, shaker)
    | BID(dealAttemptIndex, actor, bid)
    | DECLARATION(dealAttemptIndex, actor, declaration)
    | PLAY(dealAttemptIndex, actor, domino)

PublicHistory:
    baseEvents: immutable ordered tuple[BasePublicEvent]
```

Deterministic public facts are derived, not independent observations:

```text
derivePublicFacts(history, rules) ->
    all-pass and attempt boundaries, winning bidder and bid, acting seats,
    current trick, led suit, trick winners, trick points, cumulative hand
    points, current shaker, contract settlement, marks, result, and phase
```

An implementation may materialize those facts as a validated cache or as
redundant lifecycle events such as `ALL_PASS`, `AUCTION_WON`, `HAND_SETTLED`,
or `MATCH_ENDED`. They must recompute exactly from base events, configuration,
and rules; materializing them must not create a second source of truth.

```text
PrivateDealObservation:
    dealAttemptIndex: nonnegative integer
    hand: FrozenSet[DominoId]

InformationState:
    viewer: Seat
    privateObservations: immutable ordered tuple[PrivateDealObservation]
    publicHistory: PublicHistory
    currentDealAttemptIndex: nonnegative integer | None
```

For a deal considered in isolation, or conditional on a fixed pre-deal private
record, a deal-local view may expose only the current private hand and
current-deal public prefix. It must retain or reference the match-global record
whenever strategy, belief, or utility can depend on earlier private hands.

Private field state or randomization state is not a game observation unless
the selected field model declares it to be part of the actor's type. Such state
belongs in the augmented field domain, not silently inside `PublicHistory`.

Never substitute a path-free mechanical object for `InformationState` without
naming it as an abstraction or proving exact sufficiency for the consumer.

## 14. Complete-deal support and the remainder map

```text
CompatibleDealSet:
    dealAttemptIndex: nonnegative integer
    dealLocalInformation: viewer initial hand + current-attempt public prefix
    matchInformation: reference to InformationState

    contains(deal: DealWorld) -> bool
    enumerate(predicate = TRUE) -> Iterator[DealWorld]
    restrict(predicate) -> RestrictedCompatibleDealSet
```

This is rule support on complete deals for one current attempt, not a set of
whole-match latent histories. Under the baseline independent-deal rule,
earlier private hands do not add a current-deal rule constraint; they remain
available through `matchInformation` for belief and policy models. A selected
chance law with structural zeros is represented by an explicit prior-support
restriction, not by silently changing `CompatibleDealSet` or the cell fiber.

```text
playedBySeat(currentDealPublicPrefix, seat) -> FrozenSet[DominoId]

RemainderWorld:
    hiddenHands: mapping hidden Seat -> FrozenSet[DominoId]

remainderOf(deal, viewer, currentDealPublicPrefix) -> RemainderWorld
```

Definition:

```text
remainderOf(deal)[s] = deal.initialHands[s] - playedBySeat(history, s)
for each hidden seat s
```

Under the Straight 42 cell-theorem scope and a fixed actor-attributed history:

```text
reconstructDeal(remainder, viewerInitialHand, currentDealPublicPrefix) -> DealWorld
```

is the inverse operation. It must fail outside that scope rather than invent
missing attribution.

## 15. Mechanical/support state and cells

```text
CapacityCell:
    possible: FrozenSet[DominoId]  # allowed holder edges in this cell system
    capacity: nonnegative integer

CellSystem:
    unseenPool: FrozenSet[DominoId]
    cells: mapping hidden Seat -> CapacityCell

    isFeasible() -> bool
    marginalHolderSupport() -> mapping hidden Seat -> FrozenSet[DominoId]
    supportReduction() -> SupportReducedCellSystem

RuleDerivedCellSystem implements CellSystem:
    # possible sets are the local upper bounds derived from public voids

SupportReducedCellSystem implements CellSystem:
    source: CellSystem
    # possible sets equal exact marginal holder support of source
    # source is provenance only and is excluded from semantic equality

AuctionMechanicalState:
    viewer: Seat
    ownInitialHand: FrozenSet[DominoId]
    auction: AuctionState
    matchState: MatchState
    declarationBundle: all nine DeclarationAlgebra values
    phase: AUCTION

MechanicalState:
    viewer: Seat
    ownRemainingHand: FrozenSet[DominoId]
    contract: Contract
    leader: Seat
    currentTrick: ordered tuple[Play]
    handPoints: pair[int, int]
    matchState: MatchState | None
    playedBySeat: mapping Seat -> FrozenSet[DominoId]
    publicVoids: mapping Seat -> FrozenSet[LedSuit]
    phase: PLAY | HAND_COMPLETE

deriveAuctionCells(state: AuctionMechanicalState) -> RuleDerivedCellSystem
    # unseen pool is the 21 tiles outside the viewer hand;
    # every hidden possible set is that pool and every capacity is 7

deriveRuleCells(state: MechanicalState) -> RuleDerivedCellSystem
    # exactly the formulas below; this is the semantic source of support

MechanicalCompiledView:
    state: AuctionMechanicalState | MechanicalState
    optionalCellsCache: RuleDerivedCellSystem | None
    cacheProof: optional proof that the cache equals the applicable derivation
```

`playedBySeat` includes dominoes currently in the trick. `currentTrick`
provides their order. The reference mechanical coordinate intentionally omits
the order of completed tricks and the losing auction sequence; it does not
omit actor attribution of played dominoes.

Cells, reduced cells, the exact support normal form, and the fiber are derived
views of these semantic fields. An implementation may cache them, but cache
contents and cache proofs are excluded from semantic equality, hashing,
serialization, transition authority, and information-state identity. A cache
mismatch is an invariant violation, never an alternative state.

Every `CellSystem` satisfies the structural invariants:

```text
cell keys are exactly the three hidden seats
possible[s] subset of unseenPool
capacity[s] >= 0
sum(capacity[s] for hidden s) == len(unseenPool)
```

The `RuleDerivedCellSystem` returned by `deriveRuleCells` additionally
satisfies:

```text
all playedBySeat sets are pairwise disjoint
current-trick plays occur in the corresponding playedBySeat sets
ownRemainingHand is disjoint from every played set
unseenPool
  == wholeUniverse - ownRemainingHand - union(playedBySeat.values())
capacity[s] == 7 - len(playedBySeat[s])
possible[s]
  == unseenPool
     - union(effectiveSuit(q) for q in publicVoids[s])
```

Here `possible[s]` is the locally rule-allowed upper bound produced by public
voids. It is not promised to equal the exact marginal set of dominoes that
seat `s` can hold in some globally conserved world. Conservation and other
seats' capacities can make a locally allowed holder edge globally impossible.

`publicVoids` is derived from actor-attributed nonfollowing plays under the
selected declaration. A projection from history must recompute it; a raw
constructor must validate it against a supplied public-prefix witness.

Feasibility is a separate exact property:

```text
Hall inequality holds for every subset of hidden seats
union(possible[s] for hidden s) == unseenPool
    # follows from full-set Hall plus total capacity when feasible
```

A structurally well-formed `CellSystem` may be infeasible and then denotes an
empty fiber. Mechanical states projected from legal histories must be feasible.
`isFeasible` must implement exact Hall/max-flow feasibility; a constructor or
consumer requiring nonempty support may raise `InfeasibleCellSystem`. With
three hidden seats, checking the seven nonempty seat subsets is sufficient.

`marginalHolderSupport()` returns, for each hidden seat `s`, exactly the tiles
that occur in that seat in at least one fiber world. Two exact implementations
are permitted:

1. force each locally allowed edge `d -> s` and retain it exactly when the
   successor passes Hall feasibility, requiring at most
   `3 * len(unseenPool) <= 63` edge tests; or
2. obtain one feasible assignment and apply the one-assignment strongly
   connected component compiler specified above.

The second method recovers every marginal edge with one feasible assignment
and one linear graph pass. Neither method may retain merely local edges that
occur in no globally conserved world.

`supportReduction()` keeps the same pool and capacities and replaces every
`possible[s]` by `marginalHolderSupport()[s]`. It must preserve the fiber
exactly and is the unique coordinatewise least possible-set system with the
same pool, capacities, and cell schema.

Literal cells are not the globally minimal support representation. The exact
support quotient is represented by the following canonical tagged form. Hidden
seat order is clockwise relative to the viewer.

```text
DeterminateAmbiguity:
    # no ambiguous dominoes and every residual capacity is zero

BinaryAmbiguity:
    inactiveSeat: hidden Seat
    ambiguousPool: FrozenSet[DominoId]
    firstActiveResidual: positive integer
        # active seats are the canonical ordered complement of inactiveSeat
        # second residual == len(ambiguousPool) - firstActiveResidual

TernaryAmbiguity:
    ambiguousPool: FrozenSet[DominoId]
    residual0: positive integer
    residual1: positive integer
        # residual2 == len(ambiguousPool) - residual0 - residual1
    excludedSeat: finite mapping DominoId -> hidden Seat
        # missing key means all three hidden seats are possible
        # a present key removes exactly that one seat

FeasibleSupportNormalForm:
    certainBySeat: mapping hidden Seat -> FrozenSet[DominoId]
    ambiguity:
        DeterminateAmbiguity
        | BinaryAmbiguity
        | TernaryAmbiguity

TotalSupportNormalForm:
    EMPTY
    | FEASIBLE(FeasibleSupportNormalForm)
```

Compilation is exact:

```text
compileExactSupport(
    cells: CellSystem,
    internalFeasibleWorld: RemainderWorld | None = None
) -> TotalSupportNormalForm
```

- If the cells are infeasible, return `EMPTY` under ordinary extensional
  support semantics.
- Otherwise obtain one feasible assignment, either from the optional internal
  witness or by an exact feasibility construction.
- Orient each used holder edge `seat -> domino` and each unused locally allowed
  edge `domino -> seat`.
- An unused edge is marginally supported exactly when its endpoints lie in the
  same strongly connected component. Used edges are supported by the witness.
- Extract every singleton exact holder set as `certainBySeat`, subtract those
  tiles from capacities, and compile the remaining ambiguity into the unique
  determinate, binary, or ternary branch.

The optional witness is compiler-private. It may be the actual hidden remainder
inside a certified simulation, but it must not be exposed, serialized into the
player-facing state, or allowed to affect output beyond the witness-independent
normal form.

Validation contracts:

```text
Common:
    certainBySeat keys are exactly the three hidden seats in canonical order
    certainBySeat sets are pairwise disjoint
    every certain or ambiguous tile is a valid DominoId
    W == empty for Determinate, otherwise the stored ambiguousPool
    W is disjoint from every certainBySeat set
    hiddenPool == W union union(certainBySeat.values())
    len(hiddenPool) <= 21
    reconstructedCapacity[s]
      == len(certainBySeat[s]) + reconstructedResidual[s]
    0 <= reconstructedCapacity[s] <= 7 for every hidden seat
    sum(reconstructedCapacity.values()) == len(hiddenPool)

Determinate:
    W is empty
    every reconstructed residual capacity is zero

Binary:
    inactiveSeat is one of the hidden seats
    W is nonempty
    1 <= firstActiveResidual < len(W)
    active seats are the canonical complement of inactiveSeat
    active residuals are firstActiveResidual and len(W)-firstActiveResidual
    inactive residual is zero
    every ambiguous domino can occupy either active seat

Ternary:
    residual0, residual1, residual2 are positive
    residual0 + residual1 + residual2 == len(W)
    every exclusion key lies in W
    every exclusion value is a hidden seat
    each domino excludes at most one seat (enforced by the finite-map type)
    for each seat s:
        len(W) - exclusionCount[s] >= residual[s] + 1
```

A value satisfying these contracts is `WellFormedFeasibleSupportNormalForm`.
Its decoder reconstructs one feasible reduced cell system. The canonical laws
required of the implementation are:

```text
decode(compileExactSupport(cells)) has the same fiber as cells
compileExactSupport(decode(normalForm)) == FEASIBLE(normalForm)
    for every well-formed feasible normal form
wellFormed(compileExactSupport(cells).payload)
    whenever cells are feasible
```

The ternary inequalities are necessary and sufficient for a nonempty,
support-reduced ambiguity component. They are a linear validator, not an
approximation to Hall feasibility. For a singleton seat subset they are exactly
strict Hall. Every pair of seats sees all ambiguous tiles because each tile
excludes at most one seat, and the omitted seat has positive residual capacity;
therefore pair Hall is also strict. After forcing any stored allowed edge, these
strict inequalities imply Hall for the residual system, so every allowed edge
occurs in some world. Every stored ternary exclusion is essential: removing one
strictly enlarges the decoded fiber.

`TotalSupportNormalForm` has exactly one semantic state per distinct exact
support fiber over the native three-seat cell schema. Every exact deterministic
support representation must refine it. The full native standalone quotient has
an exact fixed-width minimum of 81 bits. That global number includes feasible
cell-schema states that legal Straight play cannot reach; Section 18 gives the
strict reachable-domain result.

When a containing certified mechanical state retains the fields from which
cells are derived, cells, reduced cells, the fiber, and this normal form are all
deterministic views. Their minimal supplemental semantic state is zero bits.
Materializing one is a cache or compiled view, not an additional game-state
factor.

For cell systems with fixed pool and capacities it obeys:

```text
supportReduction(C).possible[s] subseteq C.possible[s]
supportReduction(supportReduction(C)) == supportReduction(C)
C.possible subseteq Q.possible
    implies supportReduction(C).possible subseteq supportReduction(Q).possible
fiber(C) == fiber(Q)
    iff supportReduction(C) == supportReduction(Q)
```

This is a contractive canonical **reduction**, not an extensive closure
operator. Semantic equality of `SupportReducedCellSystem` compares pool,
capacities, and reduced possible sets; its optional `source` field is provenance
only.

The result is a `SupportReducedCellSystem`, not a
`RuleDerivedCellSystem`. Globally implied edge removals are not necessarily
publicly observed suit voids. A support-reduced predecessor can also acquire new
unsupported edges after an otherwise exact raw play update. Therefore code
that elects to remain in canonical reduced form must perform the typed play
update and then call `supportReduction()` again. It must not overwrite
`publicVoids` or silently substitute the reduced cells into a mechanical state
whose invariant requires the rule-derived formula.

## 16. Current-remainder fiber

```text
RemainderFiber:
    cells: CellSystem

    contains(world: RemainderWorld) -> bool
    isFeasible() -> bool
    enumerate() -> Iterator[RemainderWorld]
    count() -> nonnegative integer
    restrict(predicate) -> RestrictedRemainderFiber

RestrictedRemainderFiber:
    base: RemainderFiber
    predicate: explicit world predicate

    contains(world) -> bool
    enumerate() -> Iterator[RemainderWorld]
    count() -> nonnegative integer  # exact or explicit failure

RemainderFiber.fromNormalForm(normalForm: FeasibleSupportNormalForm)
    -> RemainderFiber
```

A `FeasibleSupportNormalForm` and every cell system that compiles to it
decode to exactly the same fiber. Branch-specific exact compiled forms are
permitted:

```text
Determinate count == 1
Binary count == binomial(len(ambiguousPool), firstActiveResidual)
Ternary count depends only on residual capacities and the three
    excluded-seat category counts; domino identities are restored only when
    constructing a world
```

A normal-form-local ordered completion automaton needs only the residual quota
vector. In the native ternary case that vector has a universal exact nine-bit
direct code; a fiber-local world rank needs at most 29 bits. These are compiled
query states and world codes, not replacements for the containing game state.

The fiber is a function of `CellSystem`, not of leader, score, contract,
mechanical history residue, or belief. Convenience constructors may accept an
`AuctionMechanicalState` or `MechanicalState`, but they must extract only its
cells. Equal cell systems denote equal remainder fibers even when the
surrounding mechanical states differ.

`contains` is true exactly when the displayed Straight 42 constraints hold:

```text
world hands are subsets of their possible sets
world hand sizes equal capacities
world hands are pairwise disjoint
world hand union equals unseenPool
```

Semantics:

- the fiber is intensional;
- enumeration and counting are exact queries;
- unrestricted `count()` equals the generating-function coefficient, deletion
  recurrence, and capacity dynamic program in Mathematical Foundation §7.8;
- native `count()` must use or equal the exact bounded dynamic-program result,
  not enumerate worlds merely to count them;
- with three hidden seats, at most 21 unseen dominoes, and capacities at most
  seven, one unrestricted count visits at most 512 occupancy states over the
  entire run, performs at most 1,533 candidate-holder checks and 1,344
  capacity-eligible extension updates, has at most 48 live occupancy states in
  any layer, and returns at most 399,072,960;
- iteration order has no game meaning;
- no implicit world cap, horizon, time limit, or memory limit exists;
- extensional enumeration may exhaust time or memory and is permitted to fail
  explicitly without changing the fiber;
- an arbitrary restricted predicate can require a different exact method and
  may fail explicitly when unsupported or resource-exhausted;
- sampling is a separate operation and return type;
- `restrict(predicate)` is exact set intersection and keeps a reference to the
  unrestricted fiber.

```text
RemainderSamplingLaw:
    fiber: RemainderFiber
    probability(world) -> exact nonnegative value
    totalMass() -> 1
    positive-mass support is a subset of fiber

UniformFiberLaw.from(fiber) -> RemainderSamplingLaw
    require fiber.count() > 0
    probability(world in fiber) == 1 / fiber.count()

    holderProbability(domino, hiddenSeat)
        -> fiber.count(after assigning domino to hiddenSeat) / fiber.count()

ExactRationalChoiceSource:
    choose(weights: finite tuple of nonnegative integers) -> index
        # returns index i with exact probability weights[i] / sum(weights)
        # requires sum(weights) > 0

RemainderWorldSampler:
    sample(law: RemainderSamplingLaw, exactRandomSource)
        -> RemainderWorld
        # exact only when the sampler/source explicitly supports the law's
        # probability representation

ExactUniformFiberSampler:
    sample(fiber, source: ExactRationalChoiceSource) -> RemainderWorld
```

A bare fiber never selects a distribution. `UniformFiberLaw` is an explicit
model choice and is justified as a physics-only posterior only under the
assumptions of Mathematical Foundation §8.4. A sampler never satisfies the
exact enumeration interface and may not accept only a fiber unless its method
name and contract explicitly select a law.

`ExactUniformFiberSampler` implements the count-ratio theorem of Mathematical
Foundation §7.8: choose any deterministic remaining domino, compute the exact
successor count for every locally allowed holder, choose the holder with those
nonnegative integer weights, and recurse. Zero-count holder edges receive zero
probability. The resulting world is exactly uniform without first materializing
the whole fiber. Every native unrestricted count used by this algorithm has
the explicit capacity-DP bounds above. A floating or modulo-biased choice
mechanism cannot claim this exact interface; approximate sampling belongs to a
separately named type. A generic exact sampler may reject a law whose exact
probability representation it cannot realize.

## 17. Projection, exact support, and typed update

```text
projectAuctionInformation(infoState) -> AuctionMechanicalState
projectContractedInformation(infoState) -> MechanicalState
```

`projectAuctionInformation` requires the current attempt to be in auction
phase. It retains the current private hand, public auction state, match residue,
and all nine declaration algebras. `deriveAuctionCells` returns the initial
unconstrained three-seat cell system; it is not stored as an independent field.
Straight bid legality does not reduce rule support.

`projectContractedInformation` requires a declared contracted-hand phase. It
replays primitive public history and derives:

- current private hand;
- current trick, leader, contract, score, and match residue;
- actor-attributed played sets;
- public voids;
- the fields from which hidden capacities, unseen pool, and possible-holder
  sets are derived exactly by `deriveRuleCells`.

A phase-dispatching convenience function may return the tagged union
`AuctionMechanicalState | MechanicalState`; it may not pretend the
post-declaration `MechanicalState` exists during auction.

For a certified contracted state, projection also validates the reachability
reductions proved in Mathematical Foundation §7.13:

```text
max(hidden capacities) - min(hidden capacities) <= 1
publicVoids[s] is a subset of the declaration's seven leadable contexts
capacity profile is derived from completed-trick/current-prefix progress
```

There are exactly 50 labeled hidden-capacity profiles. Every Straight
declaration has exactly seven leadable contexts, whose lead-fiber sizes are the
multiset `1..7`. In doubles-trump, natural effective suit 0 is nonempty but
unleadable and therefore has no public-void bit. A dense declaration-relative
void mask uses seven bits per hidden seat, not eight.

Exactness requirement for the contracted projection:

```text
set(remainderOf(deal, viewer, currentDealPrefix)
    for deal in CompatibleDealSet(currentDealComponent(infoState)))
==
set(RemainderFiber(deriveRuleCells(
    projectContractedInformation(infoState))))
```

This equality is semantic even when neither side is practically enumerable.

Every support update also performs the same public physical transition as the
objective game: it appends the actor-attributed play, resolves a fourth play,
banks trick points, updates the leader, clears the trick, and updates phase.
Support bookkeeping is not a substitute for that transition.

For a hidden actor `s != viewer` playing `d`:

```text
updateHiddenSupport(preState, observedPlay) -> postState
```

Pseudocode:

```text
require preState.phase == PLAY
require observed actor == current actor
preCells = deriveRuleCells(preState)
require d in preCells.unseenPool
require d in preCells[actor].possible

q = led suit if the pre-action trick is nonempty else None
followed = (q is None) or algebra.follows(d, q)

append Play(actor, d) to the public trick residue
add d to playedBySeat[actor]

if q is not None and not followed:
    add q to publicVoids[actor]

if the public trick now has four plays:
    resolve it exactly
    bank its points
    set leader to its winner
    clear the current trick

postCells = deriveRuleCells(postState)
if ownRemainingHand is empty and every postCells capacity is zero:
    require current trick empty and hand points sum to 42
    phase = HAND_COMPLETE

recompute or validate every derived structural invariant
require postCells.isFeasible()
```

The hidden-pool removal, capacity decrement, possible-set deletion, and new
void exclusion are consequences of the updated semantic fields and
`deriveRuleCells`; they are not separately mutable state updates.

Within the Straight cell-theorem scope, post-state feasibility plus the fixed
inverse operation certifies a nonempty typed predecessor set. An empty typed
preimage is an impossible observation, not an empty-but-accepted legal
successor.

For a viewer action:

```text
updateViewerSupport(preState, ownPlay) -> postState
```

The operation requires `ownPlay.domino` to be legal in the known viewer hand,
removes it from `ownRemainingHand`, appends the public play, updates
`playedBySeat`, records a viewer void when a legal slough proves one, and
performs the same trick/score/leader/phase transition above. The hidden pool,
hidden capacities, hidden possible sets, and every hidden remainder world are
unchanged. Its typed world map is the identity.

Exact refinement contracts:

```text
CompatibleDealSet(afterAction) subseteq CompatibleDealSet(beforeAction)

hidden actor:
    postFiber is bijective with the legal predecessor subset
    len(postFiber) <= len(preFiber)

viewer actor:
    postFiber is the identity image of preFiber
    len(postFiber) == len(preFiber)
```

The length statements are mathematical cardinality claims; an implementation
need not enumerate either fiber merely to certify them from the typed theorem.
The literal subset statement applies to fixed complete-deal domains, not to
hidden remainder worlds whose pool and capacities change type.

## 18. Straight reachability certification

Support feasibility and Straight reachability are different types.

```text
UncertifiedMechanicalState:
    structurally parsed state whose legal ancestry has not been established

ReachableMechanical(state): Proposition
    # some valid deal, contract, declaration, and legal actor-attributed
    # public prefix replay exactly to `state`

ReachableMechanicalState:
    proof-irrelevant subtype
        { state: MechanicalState // ReachableMechanical(state) }

ReachableSupport(normalForm): Proposition
    # some reachable mechanical state derives this exact normal form

ReachableSupportNormalForm:
    proof-irrelevant subtype
        { normalForm: FeasibleSupportNormalForm // ReachableSupport(normalForm) }

StraightSupportReachabilityWitness:
    viewer: Seat
    completeDeal: DealWorld
    contract and declaration residue
    actor-attributed legal public play prefix
    claimedSupport: FeasibleSupportNormalForm

ReachabilityOuterNecessaryProfile:
    declaration: Declaration
    hiddenCapacities: one of the 50 reachable profiles
    hiddenPool: FrozenSet[DominoId]
    voidMembershipByLeadContext:
        mapping leadable LedSuit -> nonempty subset of hidden seats
        # omitted context means no hidden public void in that context
```

Only these operations construct a `ReachableMechanicalState`:

```text
initialContractedState(validDeal, legalContract, declaration)
    -> ReachableMechanicalState

applyLegalPlay(pre: ReachableMechanicalState, action)
    -> ReachableMechanicalState

projectReachableSupport(pre: ReachableMechanicalState)
    -> ReachableSupportNormalForm

validateReachability(witness: StraightSupportReachabilityWitness)
    -> ReachableMechanicalState
```

`validateReachability` must replay the objective transition system from the
valid deal and contract, project the rule-derived cells, compile the exact
support normal form, and require exact equality with `claimedSupport`. It is
sound and complete for the witness supplied. Membership of a candidate support
in the full reachable image is decidable by finite exhaustive witness search,
but that search is not required as an ordinary API.

A reachable value stores no `reachable: bool`. Legal constructors preserve the
proposition inductively. An external value remains `Uncertified` until exact
validation succeeds. The validating witness and proof term are erasable and do
not refine the semantic state. Two certified wrappers with equal projections
are equal for every game-semantic purpose; witness identity is available only
in a separate audit record.

`ReachableSupportNormalForm` is not independently transitionable. It certifies
only that some legal prefix realizes the exact support. It omits declaration,
actor, current trick, score, and the realizing path. Only
`ReachableMechanicalState` may be passed to `applyLegalPlay`; no API may define
an exact game transition from a standalone support identifier and action.

The following are exact necessary checks and should reject impossible external
objects early:

```text
hidden capacity profile is one of the 50 profiles with range <= 1
all public void contexts are declaration-leadable
void-context count obeys the completed-trick/current-prefix schedule language
for every used void context q:
    at least one tile in leadFiber(q) is outside the current hidden pool
support cells are Hall-feasible
```

The schedule check is exact for its projection and must use the following
contract. Let `h = max(capacities)`, `j = 7 - h`, and let `B` be the hidden
seats at capacity `h - 1` when the profile is nonconstant. For an equal profile,
use `B = empty`. Let `Q` be the contexts occurring in at least one hidden-seat
void mask. Define the largest hidden-follower set available in the current
partial trick by:

```text
B = {}           -> F = {}
B = {h1}         -> F = {h1}
B = {h2}         -> F = {}
B = {h3}         -> F = {}
B = {h1,h2}      -> F = {h1,h2}
B = {h1,h3}      -> F = {h1}
B = {h2,h3}      -> F = {h3}
B = {h1,h2,h3}   -> F = {h2,h3}
```

Then:

```text
scheduleAdmissible ==
    len(Q) <= j
    or (
        len(Q) == j + 1
        and some q in Q has a nonempty void-membership set contained in F
    )
```

This check projects away tile availability and trick winners. Passing it does
not certify physical reachability.

These conditions are **not** collectively an exact reachability validator.
There exist support-reduced feasible cells satisfying capacity constraints but
having no legal Straight ancestry. `ReachabilityOuterNecessaryProfile.checkNecessary()`
checks exactly this necessary outer language. Its decoder constructs raw cells
from the hidden pool and void memberships and then compiles their exact support
normal form. It must never promote that result directly to a reachable type.

For support semantics alone, restrict the global exact support normal form to
the image of legal Straight prefixes. This restricted normal form remains the
coarsest exact deterministic support representation on that domain. Simultaneous
seat rotation identifies all four viewer-indexed domains, so a support payload
whose hidden seats are already viewer-relative carries no absolute viewer field.
The exact reachable cardinality is unresolved, but proved fixed-width bounds
are:

```text
standalone reachable support:       at least 26 bits, at most 46 bits
declaration supplied:                              at most 43 bits
capacity profile supplied:                         at most 43 bits
both declaration and capacity supplied:            at most 40 bits
complete certified mechanical state supplied: exactly 0 supplemental bits
```

The upper numbers rank necessary outer profiles and are constructive
ceilings, not prescribed byte layouts or exact minima. The 26-bit lower bound
comes from disjoint universally reachable no-void families.

## 19. Policy likelihood and belief types

```text
InheritedFieldLatent:
    opaque exact state at the start of the current deal attempt containing
    any persistent type, remembered private observations of hidden actors,
    memory, correlation device, or unresolved prior-attempt state required
    by the field

AugmentedAttemptRootWorld:
    currentDeal: DealWorld
    inheritedLatent: InheritedFieldLatent
```

Earlier match evidence is absorbed into the distribution over
`InheritedFieldLatent` and into the prior for the current attempt. If the
field resets between attempts, this latent may be trivial.

The implementation must choose one of two exact randomness representations.

### Kernel field

```text
KernelFieldModel:
    actionStateKernel(
        actorInformation,
        currentFieldState,
        publicHistory
    ) -> exact probability kernel over (legalAction, successorFieldState)

    jointHistoryLikelihood(rootWorld, publicPrefix) -> nonnegative value
        # may be derived by exact kernel composition
```

The current field state may be stochastic conditional on the root world and
public prefix. Local action probabilities may be multiplied only when they are
the exact chain-rule conditionals supplied by a sufficient retained state; a
hidden state path must otherwise be summed or integrated.

### Seed-augmented field

```text
SeedAugmentedRootWorld:
    currentDeal: DealWorld
    inheritedLatent: InheritedFieldLatent
    randomTape: complete private/correlated randomness used by the field

SeedAugmentedFieldModel:
    currentFieldState(rootWorld, publicHistory) -> CurrentFieldState
    generatedAction(rootWorld, publicHistory) -> legal action
    generatedSuccessorState(rootWorld, publicHistory) -> CurrentFieldState
```

Use this representation only when the selected stochastic kernel has a
specified measurable randomization realization. This is automatic for the
finite models used by the verifier and available for the usual standard-Borel
kernels once the realization contract is named. Conditional on the full random
tape, action likelihood is the zero-one indicator that the generated action
equals the observation. Marginalizing the tape must reproduce the declared
kernel law exactly. Do not additionally multiply by the unconditioned
stochastic action probability after conditioning on the tape. When no valid
realization has been supplied, use `KernelFieldModel`.

```text
CurrentAttemptBelief:
    domain: ambient exact measurable domain over the chosen augmented
            root-world type
    totalMass() -> positive exact value
    integrate(integrand: SupportedExactIntegrand)
        -> exact value in the declared scalar/measure class
    normalized() -> CurrentAttemptBelief
    pushforward(mapping: SupportedMeasurableMapping) -> exact belief/measure

DiscreteCurrentAttemptBelief extends CurrentAttemptBelief:
    mass(world) -> nonnegative exact value in the supported scalar class
    probability(world) -> normalized value
```

Pointwise `mass` or `probability` is required only for a finite or countable
latent model. A continuous latent model must expose measure/integration
semantics rather than pretending that point probabilities specify the measure.
An exact implementation may explicitly reject an unsupported integrand,
mapping, scalar class, or measure class. `integrate` does not promise symbolic
integration of an arbitrary host-language function, and `pushforward` requires
a declared measurable mapping. Approximate quadrature, Monte Carlo, sampling,
or floating estimates belong to a distinct `ApproximateBelief` interface and
must not satisfy these exact contracts.

The initial `CurrentAttemptBelief` is conditioned on the complete match-global
information available at the start of the attempt, including the viewer's
newly observed hand. Its current-deal marginal follows the selected deal chance
law.

```text
filterRootBelief(
    belief,
    observedAction,
    currentDealPublicPrefix,
    policyField
) -> CurrentAttemptBelief
```

Required root-domain semantics:

1. reject worlds where the action was illegal;
2. multiply by the exact conditional action likelihood under the chosen
   randomness representation;
3. fail explicitly if total likelihood is zero;
4. normalize on the same augmented root domain;
5. append the public event.

```text
CurrentAugmentedBelief:
    normalized exact measure over (RemainderWorld, CurrentFieldState)
```

In a valid seed-augmented realization, this is a deterministic pushforward
of root belief. In a kernel field, construct it from the exact joint conditional
distribution of root world and current field state; it need not be a graph
pushforward.

```text
RemainderBelief:
    normalized physical marginal over RemainderWorld
```

Marginalizing `CurrentAugmentedBelief` to `RemainderBelief` is allowed, but the
physical marginal alone is not generally sufficient for later filtering or
continuation.

```text
filterCurrentAugmentedBelief(
    belief,
    observedAction,
    policyField,
    typedPhysicalUpdate
) -> CurrentAugmentedBelief
```

This operation conditions the exact action/state kernel, applies hidden-removal
or viewer-identity on the physical component, transitions field state, and sums
all predecessor mass sharing a successor. In valid seed-augmented form the
kernel is deterministic; in kernel form it is not.

```text
startNextAttemptBelief(
    filteredAbandonedAttemptBelief,
    persistentStateTransition,
    dealChanceLaw,
    newlyObservedViewerHand
) -> CurrentAttemptBelief
```

An all-pass action is filtered on the old attempt first. The next attempt is
then created by the persistent-state transition and a new-deal chance kernel;
it is not a remainder update on the abandoned deal.

## 20. Native marked hand

```text
NativeHandView:
    algebra: DeclarationAlgebra
    mechanical: MechanicalState
    cells: deriveRuleCells(mechanical)          # derived view
    fiber: RemainderFiber(cells)                # derived view
```

This view keeps the full 28-node algebra available. `ownRemainingHand` is a
marking on the ambient universe, not an isolated graph. The fiber is a distinct
mathematical type and query interface, but not an independent degree of freedom
once the mechanical state is fixed: it is derived through `deriveRuleCells`.
`NativeHandView` couples those exact views without creating another source of
truth.

Exact queries may include:

```text
ownedNodes()
follows(node, context)
beats(context, node)
liveThreatsInWorld(node, world) -> FrozenSet[DominoId]
possibleLiveThreats(node) -> union over all worlds in fiber
certainLiveThreats(node) -> intersection over all worlds in fiber
locallyAllowedHolders(domino) -> hidden seats whose raw cell allows domino
possibleHolders(domino) -> hidden seats holding domino in at least one world
certainHolder(domino) -> hidden seat | None
physicalSuccessor(node, objectiveWorld) -> objective successor
supportSuccessor(observedPlay) -> typed successor MechanicalState/Fiber
```

Existential, universal, point-world, and belief-weighted queries are different
operations and must not be merged behind a parameter named `worldOrSupport`.
Queries involving posterior weight, expected threat, or action value belong to
a belief-bearing decision object, not to `NativeHandView` alone. There is no
unqualified `residualAfter(node)`: the result depends on the objective world,
observed transition, or chosen quantifier over support.

## 21. Strategic belief, utility, and value interfaces

```text
Utility:
    immediateReward(transition, perspective) -> ordered scalar
    terminalValue(terminalState, perspective) -> ordered scalar
    # values must be measurable and integrable under every reachable exact
    # belief used by the selected decision problem; bounded values suffice
```

The implementation must state whether utility is terminal-only or additive so
that banked rewards are not counted twice.

Named utilities may include:

```text
DECLARING_POINTS
SIGNED_POINT_DIFFERENTIAL
CONTRACT_SUCCESS
HAND_MARKS
MATCH_WIN
```

```text
ContinuationRecord:
    exact viewer-known observation residue, beyond MechanicalState, that the
    selected decision strategy, continuation field, or utility can still
    consult

    publicComponent: retained common public-history residue
    viewerPrivateComponent: retained viewer-private observation residue
    transition(viewerObservationSegment) -> ContinuationRecord
```

The record may be an explicit empty value, a proved sufficient summary, or
the relevant full slice of `InformationState`. It is not automatically
minimal. Hidden actors' private records and uncertain field state remain in
`AugmentedCurrentBelief`; they are not copied into this viewer-known record.
Omission is exact only after proving that the selected decision strategy,
continuation field, and utility are blind to what was omitted, or after fixing
that residue in the conditioned subproblem.

```text
AugmentedCurrentBelief:
    ambientDomain: admissible subset of
                   (RemainderWorld, CurrentFieldState)
    normalized measure on ambientDomain
    finite/countable implementations may expose positiveMassSupport()
    general implementations expose exact measure/null-set semantics and do
        not invent a topological support without declaring a topology
    integration and pushforward operations appropriate to its exact measure
    class

DecisionState:
    mechanical: MechanicalState
    continuationRecord: ContinuationRecord
    belief: AugmentedCurrentBelief
    utilityResidue: exact retained object already included semantically in
                    mechanical or continuationRecord
```

`DecisionState` is an explicitly factored executable representation of the
exact strategic state \((c,e,\beta)\) relative to a fixed rules profile,
continuation model, utility, and allowed decision-strategy class. `utilityResidue` is a typed view of
information that the mathematical theorem requires to be retained inside
\(c\) or \(e\); it is not an unaccounted fourth semantic input.

Hard compatibility constraints determine `ambientDomain`. Probabilistic
correlation is a property of `belief`, not of set support alone: a correlated
law may assign positive mass throughout a full Cartesian domain. The belief
may nevertheless be concentrated on a proper measurable subset because of
chance or likelihood zeros. On a finite/countable domain this is represented
by a possibly smaller positive-mass support; in a general measurable domain no
topological support is assumed unless a topology is part of the contract.

A finite/countable implementation may expose normalized point weights. A
continuous field-state component must retain measure semantics rather than
being coerced into a discrete table.

Exact factored convenience representations include:

```text
GraphFieldBelief:
    remainderBelief: RemainderBelief
    fieldStateOfWorld: RemainderWorld -> CurrentFieldState

ConditionalFieldBelief:
    remainderBelief: RemainderBelief
    fieldStateKernel: exact conditional law
                      CurrentFieldState | RemainderWorld
```

`GraphFieldBelief` is exact when field state is almost surely the displayed
known function of the remainder world; one constant state is a special case.
`ConditionalFieldBelief` is exact for finite/countable domains by the
conditional-kernel factorization and for general spaces when the required
regular conditional exists. A separate world marginal and field-state marginal
without the conditional coupling are exact only after proving independence and
its preservation by filtering and transition. Otherwise use the coupled
`AugmentedCurrentBelief`. The `continuationRecord` component may be omitted
from notation only when it is proved trivial or fixed for the entire
conditioned problem.

A Bellman or rollout interface that bundles several events into one
viewer-observation segment must return the cumulative additive reward over
exactly that segment and the successor `ContinuationRecord` at the same
boundary. The segment may include public events and newly observed
viewer-private events.
A terminal-only utility instead assigns zero intermediate reward. Mixing
boundaries or counting banked reward twice is an error.

Value is computed from these objects. It is never a mutable property of
`Domino`, `NativeHandView`, `RemainderFiber`, or `MechanicalState`.

## 22. Perfect-information backward induction

```text
solvePerfectInformationHistory(
    completeInformationHistoryNode,
    utility,
    nodeOperatorByActor
) -> HistoryValue and ActionValues
```

`nodeOperatorByActor` must explicitly identify one of:

```text
FIXED_POLICY_EXPECTATION
MAXIMIZE
MINIMIZE
CUSTOM_FINITE_OPERATOR
```

For a two-partnership zero-sum solve, one team's actors maximize and the other
team's actors minimize the same oriented scalar utility.

Backward induction on complete-information history nodes is valid because total
remaining hand size strictly decreases.

A memoized state-DAG interface may also be provided:

```text
solvePerfectInformationState(
    compressedState,
    utilityResidue,
    stateMarkovNodeOperators
) -> StateValue and ActionValues
```

It is exact only after proving that utility residue and every node operator
factor through the compressed state and commute with its transition quotient.
History-dependent policies or utilities may require a larger state or the
history-node solver.

“Oracle” without utility, actor/operator semantics, and the claimed
history-to-state quotient is an invalid interface specification.

## 23. Explicit predicates, frontiers, and quotients

```text
StatePredicate:
    test(state or world) -> bool
```

Exact operations include:

```text
fiber.restrict(predicate)
collectStatesMatching(predicate)
traverseUntil(predicate) -> Frontier
```

A frontier is not a terminal result. Terminal values require a separately
named quotient.

```text
OutcomeQuotient:
    applies(state, utility, continuationModel) -> bool
    terminalReplacement(state) -> quotient terminal state
```

The built-in make/set quotient may claim unconditional preservation only for:

- current-hand contract success;
- current-hand mark award;
- immediate match-score update;
- whether the current award ends the match.

Preservation of later-hand match value requires an explicit continuation-model
assumption that omitted plays and observations cannot affect future behavior.

## 24. Errors

At minimum define explicit errors for:

```text
InvalidConfiguration
InvalidDomino
InvalidDeal
InvalidAuctionAction
InvalidDeclaration
InvalidContract
IllegalPlay
InvariantViolation
InfeasibleCellSystem
ImpossibleObservation
DealOutsideSupport
RemainderOutsideFiber
MissingHistoryForReconstruction
MissingSamplingLaw
EmptySamplingDomain
UncertifiedState
InvalidReachabilityWitness
UnreachableSupport
PhaseMismatch
ZeroLikelihoodObservation
UnsupportedExactIntegration
UnsupportedExactPredicateCount
UnsupportedMeasurableMapping
UnsupportedExactSamplingLaw
UnsupportedCorrelationFactorization
MixedRandomnessRepresentation
ResourceExhaustion  # optional wrapper around host failure
UnsupportedRuleProfile
```

Do not coerce invalid inputs into nearby valid states.

## 25. Equality and identity

- Domino equality is stable physical identity.
- Current-deal-world equality compares all four initial hands for one attempt.
- Full-location equality compares exact location and order fields.
- Contracted-play equality compares all retained physical fields.
- Equality of a certified reachable state is equality of its projected
  physical/mechanical state. Reachability proofs and replay witnesses are
  proof-irrelevant and are excluded from semantic equality, hashing, and
  serialization. Separate audit records may compare provenance explicitly.
- Match-state equality includes shaker, marks, target, and phase.
- Information-state equality compares viewer, every private deal observation,
  and full public history.
- Mechanical-state equality compares its explicit semantic fields only.
  Derived cells, normal forms, fibers, compiled tables, and proof/cache fields
  are excluded and must agree by invariant when materialized.
- Feasible-support-normal-form equality is structural equality of its canonical
  tagged payload after well-formedness validation.
- Compatible-deal-set equality means set equality on initial deals.
- Fiber equality means set equality on current remainder worlds.
- Belief equality means equality of normalized measures under the stated
  augmented-world correspondence.
- Hash equality is never mathematical equality unless collision freedom is
  proved for the exact finite domain.

## 26. Initial exhaustive acceptance surface

The first implementation slice must exhaustively verify:

1. 28 unique dominoes;
2. natural-incidence covering facts;
3. count total 35;
4. all nine Straight 42 declaration algebras;
5. effective-suit absorption;
6. follow equivalence;
7. unique trick winner for every declaration, lead, and three-domino subset;
8. agreement with a separately coded prose-rule winner resolver on the same
   737,100 cases;
9. exact trick point calculation;
10. contextual `BEATS` equivalence;
11. identity pip transport;
12. count-preserving pip permutations and scoped `2 <-> 3` game-order
    isomorphism;
13. the negative witness that when-led threat is not a complete ontology.

No auction, deal generator, objective state, fiber, belief, or solver code
belongs in the first slice.

The repository-level file `verification/verify_foundation.py` is an independent
finite proof receipt for the mathematical package. It is not source code for
the first implementation slice and must not be copied as the implementation.
