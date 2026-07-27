//! Phase-indexed contracted-play and match states with certified lifecycle
//! constructors.
//!
//! Implements Exec §10 (states and structural invariants), §10.1 (objective
//! lifecycle constructors — the normative source of reachable objective
//! states, INV-3), §11 (legal play and transition), and §12 (settlement).
//!
//! Reachability is a proof-irrelevant proposition (D1): the certified type
//! stores no flag, its constructor is private to this module, equality and
//! hashing go through the projected semantic state only (INV-2), and the
//! optional replay witness is an erasable audit artifact.

use crate::algebra::algebra_for;
use crate::algebra::trick::{Play, TrickResult};
use crate::declaration::Declaration;
use crate::domino::{domino_from_id, DominoId, DominoSet, DOMINO_COUNT};
use crate::objective::auction::{AuctionResult, AuctionState, AuctionWin, ObjectiveDealAttempt};
use crate::objective::contract::{contract_from_auction, settle, Contract, HandAward};
use crate::objective::deal::DealWorld;
use crate::objective::events::{BasePublicEvent, PrivateDealObservation};
use crate::objective::ObjectiveError;
use crate::rules::{AuctionAction, RulesConfig};
use crate::seat::Seat;
use std::hash::{Hash, Hasher};

/// Match phase (Exec §10 `MatchState.phase`).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MatchPhase {
    /// Waiting for a new deal.
    NeedDeal,
    /// An auction is in progress.
    Auction,
    /// A contracted hand is being played.
    Play,
    /// One partnership has reached the target.
    MatchComplete,
}

/// Match residue: marks, shaker, target, phase (Exec §10 `MatchState`).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct MatchState {
    marks: [u32; 2],
    shaker: Seat,
    target: u32,
    phase: MatchPhase,
}

impl MatchState {
    /// Start a match at `(0,0)` marks (R-MATCH-01), emitting
    /// `MATCH_STARTED` as return data (Exec §13).
    pub fn start(config: RulesConfig, initial_shaker: Seat) -> (MatchState, BasePublicEvent) {
        (
            MatchState {
                marks: [0, 0],
                shaker: initial_shaker,
                target: config.match_target(),
                phase: MatchPhase::NeedDeal,
            },
            BasePublicEvent::MatchStarted {
                target: config.match_target(),
                initial_shaker,
            },
        )
    }

    /// Current marks by partnership.
    pub fn marks(&self) -> [u32; 2] {
        self.marks
    }

    /// The current shaker.
    pub fn shaker(&self) -> Seat {
        self.shaker
    }

    /// The match target `T`.
    pub fn target(&self) -> u32 {
        self.target
    }

    /// The current phase.
    pub fn phase(&self) -> MatchPhase {
        self.phase
    }
}

/// Contracted-hand phase (Exec §10 `ContractedPlayState.phase`).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PlayPhase {
    /// Tricks remain to be played.
    Play,
    /// All 28 dominoes have been played.
    HandComplete,
}

/// The reduced contracted-play state (Exec §10). Constructible outside the
/// lifecycle only through [`UncertifiedContractedPlayStructure`], whose
/// validation establishes the structural invariants — never reachability.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ContractedPlayState {
    contract: Contract,
    remaining_hands: [DominoSet; 4],
    leader: Seat,
    current_trick: Vec<Play>,
    hand_points: [u32; 2],
    phase: PlayPhase,
}

impl ContractedPlayState {
    /// The certified contract.
    pub fn contract(&self) -> &Contract {
        &self.contract
    }

    /// One seat's remaining hand.
    pub fn remaining_hand(&self, seat: Seat) -> &DominoSet {
        &self.remaining_hands[seat.index()]
    }

    /// The current trick leader.
    pub fn leader(&self) -> Seat {
        self.leader
    }

    /// The current partial trick in play order.
    pub fn current_trick(&self) -> &[Play] {
        &self.current_trick
    }

    /// Banked hand points by partnership.
    pub fn hand_points(&self) -> [u32; 2] {
        self.hand_points
    }

    /// The hand phase.
    pub fn phase(&self) -> PlayPhase {
        self.phase
    }

    /// The seat to act when in play phase: `leader + |trick| mod 4`
    /// (Exec §10).
    pub fn current_actor(&self) -> Option<Seat> {
        match self.phase {
            PlayPhase::Play => Some(self.leader.offset(self.current_trick.len() as u8)),
            PlayPhase::HandComplete => None,
        }
    }

    /// Check every displayed structural invariant of Exec §10.
    fn check_structural(&self) -> Result<(), ObjectiveError> {
        let r: usize = Seat::ALL
            .iter()
            .map(|&s| self.remaining_hands[s.index()].len())
            .sum();
        let j = self.current_trick.len();
        if j > 3 {
            return Err(ObjectiveError::InvariantViolation);
        }
        let p = DOMINO_COUNT - r;
        if p < j || !(p - j).is_multiple_of(4) {
            return Err(ObjectiveError::InvariantViolation);
        }
        let t = (p - j) / 4;
        if t > 7 {
            return Err(ObjectiveError::InvariantViolation);
        }
        // Current-trick actors are leader + position mod 4.
        for (i, play) in self.current_trick.iter().enumerate() {
            if play.actor != self.leader.offset(i as u8) {
                return Err(ObjectiveError::InvariantViolation);
            }
        }
        // Hands pairwise disjoint; trick dominoes distinct, outside hands.
        for (i, hand) in self.remaining_hands.iter().enumerate() {
            for other in self.remaining_hands.iter().skip(i + 1) {
                if !hand.is_disjoint(other) {
                    return Err(ObjectiveError::InvariantViolation);
                }
            }
        }
        let mut trick_set = DominoSet::empty();
        for play in &self.current_trick {
            if trick_set.contains(play.domino) {
                return Err(ObjectiveError::InvariantViolation);
            }
            trick_set.insert(play.domino);
            for hand in &self.remaining_hands {
                if hand.contains(play.domino) {
                    return Err(ObjectiveError::InvariantViolation);
                }
            }
        }
        // Per-seat hand size: 7 - t - [seat appears in current trick].
        for &s in &Seat::ALL {
            let in_trick = self.current_trick.iter().any(|p| p.actor == s) as usize;
            if self.remaining_hands[s.index()].len() != 7 - t - in_trick {
                return Err(ObjectiveError::InvariantViolation);
            }
        }
        // Banked points equal completed-trick awards:
        // sum == t + count(completed dominoes).
        let mut completed = DominoSet::full();
        for &s in &Seat::ALL {
            completed = completed.difference(&self.remaining_hands[s.index()]);
        }
        completed = completed.difference(&trick_set);
        let completed_count: u32 = completed
            .iter()
            .map(|id| domino_from_id(id).count_points() as u32)
            .sum();
        if self.hand_points[0] + self.hand_points[1] != t as u32 + completed_count {
            return Err(ObjectiveError::InvariantViolation);
        }
        // Phase invariants.
        match self.phase {
            PlayPhase::HandComplete => {
                if r != 0 || j != 0 || t != 7 {
                    return Err(ObjectiveError::InvariantViolation);
                }
                if self.hand_points[0] + self.hand_points[1] != 42 {
                    return Err(ObjectiveError::InvariantViolation);
                }
            }
            PlayPhase::Play => {
                if r == 0 {
                    return Err(ObjectiveError::InvariantViolation);
                }
            }
        }
        Ok(())
    }
}

/// Raw contracted-play fields satisfying only structural invariants after
/// validation (Exec §10 `UncertifiedContractedPlayStructure`). Structural
/// validation alone never proves reachability.
#[derive(Clone, Debug)]
pub struct UncertifiedContractedPlayStructure {
    /// Claimed contract.
    pub contract: Contract,
    /// Claimed remaining hands by seat.
    pub remaining_hands: [DominoSet; 4],
    /// Claimed current leader.
    pub leader: Seat,
    /// Claimed current trick.
    pub current_trick: Vec<Play>,
    /// Claimed banked points.
    pub hand_points: [u32; 2],
    /// Claimed phase.
    pub phase: PlayPhase,
}

impl UncertifiedContractedPlayStructure {
    /// Validate the displayed structural invariants of Exec §10, returning
    /// a structurally consistent — but **not** reachability-certified —
    /// state.
    pub fn validate_structural(self) -> Result<ContractedPlayState, ObjectiveError> {
        let state = ContractedPlayState {
            contract: self.contract,
            remaining_hands: self.remaining_hands,
            leader: self.leader,
            current_trick: self.current_trick,
            hand_points: self.hand_points,
            phase: self.phase,
        };
        state.check_structural()?;
        Ok(state)
    }
}

/// The origin residue of a contracted hand, retained only inside erasable
/// audit records (Exec §10 `ContractedHandOrigin`; D1).
#[derive(Clone, Debug)]
pub struct ContractedHandOrigin {
    /// Deal attempt index.
    pub deal_attempt_index: u64,
    /// Shaker of the originating attempt.
    pub shaker: Seat,
    /// Marks before the hand.
    pub pre_hand_marks: [u32; 2],
    /// Match target.
    pub match_target: u32,
    /// The complete originating deal.
    pub deal: DealWorld,
    /// The declared interpretation.
    pub declaration: Declaration,
}

/// An erasable replay witness (Exec §10): origin plus the actor-attributed
/// legal play prefix. Never serialized into semantic state, never used by
/// transition logic, excluded from equality and hashing (D1; INV-2).
#[derive(Clone, Debug)]
pub struct AuditRecord {
    /// The originating contracted-hand origin.
    pub origin: ContractedHandOrigin,
    /// The actor-attributed legal play prefix replaying to the state.
    pub play_prefix: Vec<Play>,
}

/// A certified reachable contracted-play state (Exec §10; INV-3).
///
/// Constructed only by [`begin_contracted_play`] and [`apply_play`] within
/// this module. Observation, equality, and hashing go solely through the
/// projected [`ContractedPlayState`] (Exec §25; INV-2): the attempt index is
/// public lifecycle residue used for event emission, and the audit record is
/// an erasable proof artifact; neither participates in equality.
#[derive(Clone, Debug)]
pub struct ReachableContractedPlayState {
    state: ContractedPlayState,
    deal_attempt_index: u64,
    audit: Option<AuditRecord>,
}

impl PartialEq for ReachableContractedPlayState {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl Eq for ReachableContractedPlayState {}

impl Hash for ReachableContractedPlayState {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.state.hash(hasher);
    }
}

impl ReachableContractedPlayState {
    /// The projected semantic state (the sole equality carrier, Exec §25).
    pub fn state(&self) -> &ContractedPlayState {
        &self.state
    }

    /// Public lifecycle residue: the attempt index for event emission.
    pub fn deal_attempt_index(&self) -> u64 {
        self.deal_attempt_index
    }

    /// The optional erasable audit witness (never transition-authoritative).
    pub fn audit(&self) -> Option<&AuditRecord> {
        self.audit.as_ref()
    }

    /// Erase the audit witness. Semantically the identical state (D1).
    pub fn with_erased_audit(mut self) -> ReachableContractedPlayState {
        self.audit = None;
        self
    }
}

/// Begin one deal attempt (Exec §10.1 `beginDealAttempt`): requires
/// `NEED_DEAL`, emits `DEAL_STARTED` plus the four private deal
/// observations, and moves the match to `AUCTION` with marks, shaker, and
/// target unchanged.
pub fn begin_deal_attempt(
    match_state: &MatchState,
    deal: DealWorld,
    deal_attempt_index: u64,
) -> Result<
    (
        ObjectiveDealAttempt,
        MatchState,
        BasePublicEvent,
        [PrivateDealObservation; 4],
    ),
    ObjectiveError,
> {
    if match_state.phase != MatchPhase::NeedDeal {
        return Err(ObjectiveError::PhaseMismatch);
    }
    let attempt = ObjectiveDealAttempt {
        deal_attempt_index,
        deal,
        auction: AuctionState::new(match_state.shaker),
    };
    let next = MatchState {
        phase: MatchPhase::Auction,
        ..*match_state
    };
    let event = BasePublicEvent::DealStarted {
        deal_attempt_index,
        shaker: match_state.shaker,
    };
    let observations = Seat::ALL.map(|seat| PrivateDealObservation {
        deal_attempt_index,
        seat,
        hand: *deal.hand(seat),
    });
    Ok((attempt, next, event, observations))
}

/// Apply one auction action (Exec §10.1 `applyAuctionAction`): validates the
/// next actor and membership in the legal action set and appends exactly one
/// public `BID` event.
pub fn apply_auction_action(
    attempt: &ObjectiveDealAttempt,
    action: AuctionAction,
    config: RulesConfig,
) -> Result<(ObjectiveDealAttempt, BasePublicEvent), ObjectiveError> {
    let actor = attempt
        .auction
        .next_actor()
        .ok_or(ObjectiveError::InvalidAuctionAction)?;
    let auction = attempt.auction.apply(action, config)?;
    Ok((
        ObjectiveDealAttempt {
            deal_attempt_index: attempt.deal_attempt_index,
            deal: attempt.deal,
            auction,
        },
        BasePublicEvent::Bid {
            deal_attempt_index: attempt.deal_attempt_index,
            actor,
            action,
        },
    ))
}

/// A certified pending declaration: the completed auction with its win,
/// awaiting the bidder's public declaration (Exec §10.1 `closeAuction`
/// result). Constructed only by [`close_auction`].
#[derive(Clone, Debug)]
pub struct PendingDeclaration {
    attempt: ObjectiveDealAttempt,
    win: AuctionWin,
}

impl PendingDeclaration {
    /// The underlying deal attempt.
    pub fn attempt(&self) -> &ObjectiveDealAttempt {
        &self.attempt
    }

    /// The certified auction win.
    pub fn win(&self) -> &AuctionWin {
        &self.win
    }
}

/// Result of closing a completed auction (Exec §10.1 `closeAuction`).
#[derive(Clone, Debug)]
pub enum CloseAuctionOutcome {
    /// All four players passed: the shaker advances clockwise, no marks are
    /// awarded, and the match needs a new deal (R-AUC-11).
    AllPass(MatchState),
    /// A winning bid stands; declaration is pending.
    Pending(PendingDeclaration),
}

/// Close a completed auction (Exec §10.1 `closeAuction`).
pub fn close_auction(
    attempt: ObjectiveDealAttempt,
    match_state: &MatchState,
    _config: RulesConfig,
) -> Result<CloseAuctionOutcome, ObjectiveError> {
    if match_state.phase != MatchPhase::Auction {
        return Err(ObjectiveError::PhaseMismatch);
    }
    if attempt.auction.shaker() != match_state.shaker {
        return Err(ObjectiveError::PhaseMismatch);
    }
    match attempt.auction.result()? {
        AuctionResult::AllPass => Ok(CloseAuctionOutcome::AllPass(MatchState {
            shaker: match_state.shaker.next(),
            phase: MatchPhase::NeedDeal,
            ..*match_state
        })),
        AuctionResult::Win(win) => Ok(CloseAuctionOutcome::Pending(PendingDeclaration {
            attempt,
            win,
        })),
    }
}

/// Begin contracted play (Exec §10.1 `beginContractedPlay`): certifies the
/// contract from the auction win, seats the initial hands, sets the bidder
/// as leader (R-LEAD-01), and emits the public `DECLARATION` event.
pub fn begin_contracted_play(
    pending: PendingDeclaration,
    declaration: Declaration,
    match_state: &MatchState,
    config: RulesConfig,
) -> Result<(ReachableContractedPlayState, MatchState, BasePublicEvent), ObjectiveError> {
    if match_state.phase != MatchPhase::Auction {
        return Err(ObjectiveError::PhaseMismatch);
    }
    if pending.attempt.auction.shaker() != match_state.shaker {
        return Err(ObjectiveError::PhaseMismatch);
    }
    let contract = contract_from_auction(&pending.win, declaration, config)?;
    let state = ContractedPlayState {
        contract,
        remaining_hands: Seat::ALL.map(|s| *pending.attempt.deal.hand(s)),
        leader: contract.bidder(),
        current_trick: Vec::new(),
        hand_points: [0, 0],
        phase: PlayPhase::Play,
    };
    state.check_structural()?;
    let certified = ReachableContractedPlayState {
        state,
        deal_attempt_index: pending.attempt.deal_attempt_index,
        audit: Some(AuditRecord {
            origin: ContractedHandOrigin {
                deal_attempt_index: pending.attempt.deal_attempt_index,
                shaker: match_state.shaker,
                pre_hand_marks: match_state.marks,
                match_target: match_state.target,
                deal: pending.attempt.deal,
                declaration,
            },
            play_prefix: Vec::new(),
        }),
    };
    let next = MatchState {
        phase: MatchPhase::Play,
        ..*match_state
    };
    let event = BasePublicEvent::Declaration {
        deal_attempt_index: pending.attempt.deal_attempt_index,
        actor: contract.bidder(),
        declaration,
    };
    Ok((certified, next, event))
}

/// The exact legal play set (Exec §11 `legalPlays`; R-PLAY-03,
/// R-FOLLOW-01/02): the whole hand on a lead; otherwise the nonempty
/// follower subset of the led effective suit, else the whole hand.
pub fn legal_plays(state: &ReachableContractedPlayState) -> DominoSet {
    let s = &state.state;
    let Some(actor) = s.current_actor() else {
        return DominoSet::empty();
    };
    let hand = s.remaining_hands[actor.index()];
    if s.current_trick.is_empty() {
        return hand;
    }
    let algebra = algebra_for(s.contract.declaration());
    let q = algebra.led_suit(s.current_trick[0].domino);
    let followers = DominoSet::from_ids(hand.iter().filter(|&d| algebra.follows(d, q)));
    if followers.is_empty() {
        hand
    } else {
        followers
    }
}

/// Apply one legal play (Exec §11 `applyPlay`): removes the domino, appends
/// the actor-attributed play, resolves a fourth play exactly, banks trick
/// points, rotates the leader, and completes the hand at 28 plays requiring
/// the 42-point conservation. The primitive transition always plays all 28
/// dominoes — no make/set early termination.
///
/// Returns the successor state, the public `PLAY` event, and the exact trick
/// result when this play completed a trick (derived return data, INV-8).
pub fn apply_play(
    state: &ReachableContractedPlayState,
    domino: DominoId,
) -> Result<
    (
        ReachableContractedPlayState,
        BasePublicEvent,
        Option<TrickResult>,
    ),
    ObjectiveError,
> {
    let s = &state.state;
    if s.phase != PlayPhase::Play {
        return Err(ObjectiveError::PhaseMismatch);
    }
    let actor = s.current_actor().expect("play phase has an actor");
    if !legal_plays(state).contains(domino) {
        return Err(ObjectiveError::IllegalPlay);
    }

    let mut next = s.clone();
    next.remaining_hands[actor.index()].remove(domino);
    next.current_trick.push(Play { actor, domino });

    let mut trick_result = None;
    if next.current_trick.len() == 4 {
        let algebra = algebra_for(s.contract.declaration());
        let result = algebra
            .resolve_trick(&next.current_trick)
            .map_err(|_| ObjectiveError::InvariantViolation)?;
        next.hand_points[result.winner.team().index()] += result.points as u32;
        next.current_trick.clear();
        next.leader = result.winner;
        trick_result = Some(result);
        if Seat::ALL
            .iter()
            .all(|&s| next.remaining_hands[s.index()].is_empty())
        {
            if next.hand_points[0] + next.hand_points[1] != 42 {
                return Err(ObjectiveError::InvariantViolation);
            }
            next.phase = PlayPhase::HandComplete;
        }
    }
    next.check_structural()?;

    let audit = state.audit.clone().map(|mut a| {
        a.play_prefix.push(Play { actor, domino });
        a
    });
    Ok((
        ReachableContractedPlayState {
            state: next,
            deal_attempt_index: state.deal_attempt_index,
            audit,
        },
        BasePublicEvent::Play {
            deal_attempt_index: state.deal_attempt_index,
            actor,
            domino,
        },
        trick_result,
    ))
}

/// Result of hand settlement (Exec §12).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct HandResult {
    /// The settlement award.
    pub award: HandAward,
    /// Final hand points by partnership.
    pub final_points: [u32; 2],
}

/// Settle a completed hand and advance the match (Exec §12 `completeHand`):
/// awards the stake, ends the match when the target is reached, otherwise
/// advances the shaker clockwise and requests a new deal.
pub fn complete_hand(
    play_state: &ReachableContractedPlayState,
    match_state: &MatchState,
) -> Result<(HandResult, MatchState), ObjectiveError> {
    if match_state.phase != MatchPhase::Play {
        return Err(ObjectiveError::PhaseMismatch);
    }
    let s = &play_state.state;
    if s.phase != PlayPhase::HandComplete {
        return Err(ObjectiveError::PhaseMismatch);
    }
    let award = settle(&s.contract, s.hand_points)?;
    let mut marks = match_state.marks;
    marks[award.team.index()] += award.marks;
    let complete = marks[award.team.index()] >= match_state.target;
    let next = MatchState {
        marks,
        shaker: if complete {
            match_state.shaker
        } else {
            match_state.shaker.next()
        },
        target: match_state.target,
        phase: if complete {
            MatchPhase::MatchComplete
        } else {
            MatchPhase::NeedDeal
        },
    };
    Ok((
        HandResult {
            award,
            final_points: s.hand_points,
        },
        next,
    ))
}
