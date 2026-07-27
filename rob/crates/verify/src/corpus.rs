//! Deterministic hand corpus generator (BRIEF §8 S2/S3).
//!
//! rob's own generator — independent of the ingest verifiers. All choices
//! come from an explicit integer PRNG; nothing here selects a probability
//! law (sampling exactness claims live in the core sampler, not in this
//! test-corpus machinery).

use rob_core::{
    all_ids, apply_auction_action, apply_play, begin_contracted_play, begin_deal_attempt,
    close_auction, complete_hand, AuctionAction, BasePublicEvent, BidValue, CloseAuctionOutcome,
    DealWorld, Declaration, DominoId, DominoSet, HandResult, MatchState, ObjectiveError, Play,
    PointAmount, ReachableContractedPlayState, RulesConfig, Seat, TrickResult, DOMINO_COUNT,
};

/// A small deterministic integer PRNG (SplitMix64). Test-corpus machinery
/// only: it makes no exactness or uniformity claim (INV-4 is untouched —
/// integer arithmetic only).
pub struct DetRng(u64);

impl DetRng {
    /// Seeded constructor.
    pub fn new(seed: u64) -> DetRng {
        DetRng(seed)
    }

    /// Next raw 64-bit value.
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// Deterministic choice in `0..n` (requires `n > 0`).
    pub fn below(&mut self, n: usize) -> usize {
        assert!(n > 0);
        (self.next_u64() % n as u64) as usize
    }
}

/// A deterministic complete deal: Fisher–Yates over the 28 identities, four
/// labeled seven-tile slices.
pub fn deterministic_deal(rng: &mut DetRng) -> DealWorld {
    let mut ids: Vec<DominoId> = all_ids().collect();
    for i in (1..DOMINO_COUNT).rev() {
        ids.swap(i, rng.below(i + 1));
    }
    let hands: [DominoSet; 4] =
        core::array::from_fn(|s| DominoSet::from_ids(ids[s * 7..(s + 1) * 7].iter().copied()));
    DealWorld::new(hands).expect("slices of a permutation form a valid deal")
}

/// One recorded play step of a full hand.
pub struct PlayStep {
    /// The acting seat.
    pub actor: Seat,
    /// The chosen domino.
    pub domino: DominoId,
    /// The exact legal set at that node, in canonical id order.
    pub legal: Vec<DominoId>,
    /// The actor's hand before the play.
    pub hand_before: DominoSet,
    /// The current trick before the play.
    pub trick_before: Vec<Play>,
    /// The trick result when this play completed a trick.
    pub trick_result: Option<TrickResult>,
}

/// One fully played contracted hand with its complete evidence trail.
pub struct PlayedHand {
    /// The declared interpretation.
    pub declaration: Declaration,
    /// The winning bidder.
    pub bidder: Seat,
    /// The dealt world.
    pub deal: DealWorld,
    /// Every primitive public event, in order.
    pub events: Vec<BasePublicEvent>,
    /// All 28 play steps.
    pub steps: Vec<PlayStep>,
    /// The trajectory of certified states: initial state plus one successor
    /// per play (29 states).
    pub states: Vec<ReachableContractedPlayState>,
    /// The settled result.
    pub result: HandResult,
    /// The post-settlement match state.
    pub post_match: MatchState,
}

/// Drive one complete contracted hand through the certified lifecycle
/// (Exec §10.1): auction script, declaration, then 28 plays chosen by
/// `choose` (an index into the canonical-order legal set).
pub fn play_hand(
    config: RulesConfig,
    shaker: Seat,
    deal: DealWorld,
    auction_script: &[AuctionAction; 4],
    declaration: Declaration,
    mut choose: impl FnMut(&ReachableContractedPlayState, &[DominoId]) -> usize,
) -> Result<PlayedHand, ObjectiveError> {
    let (match_state, start_event) = MatchState::start(config, shaker);
    let mut events = vec![start_event];

    let (mut attempt, match_state, deal_event, _observations) =
        begin_deal_attempt(&match_state, deal, 0)?;
    events.push(deal_event);
    for &action in auction_script {
        let (next, event) = apply_auction_action(&attempt, action, config)?;
        attempt = next;
        events.push(event);
    }
    let pending = match close_auction(attempt, &match_state, config)? {
        CloseAuctionOutcome::Pending(p) => p,
        CloseAuctionOutcome::AllPass(_) => return Err(ObjectiveError::InvalidContract),
    };
    let (mut state, match_state, decl_event) =
        begin_contracted_play(pending, declaration, &match_state, config)?;
    events.push(decl_event);
    let bidder = state.state().contract().bidder();

    let mut steps = Vec::with_capacity(28);
    let mut states = vec![state.clone()];
    for _ in 0..DOMINO_COUNT {
        let legal: Vec<DominoId> = rob_core::legal_plays(&state).iter().collect();
        let actor = state.state().current_actor().expect("play phase");
        let hand_before = *state.state().remaining_hand(actor);
        let trick_before = state.state().current_trick().to_vec();
        let domino = legal[choose(&state, &legal)];
        let (next, event, trick_result) = apply_play(&state, domino)?;
        events.push(event);
        steps.push(PlayStep {
            actor,
            domino,
            legal,
            hand_before,
            trick_before,
            trick_result,
        });
        state = next;
        states.push(state.clone());
    }
    let (result, post_match) = complete_hand(&state, &match_state)?;
    Ok(PlayedHand {
        declaration,
        bidder,
        deal,
        events,
        steps,
        states,
        result,
        post_match,
    })
}

/// A deterministic auction script guaranteed to produce a winner: random
/// legal actions, with the final action forced to the first nonpass legal
/// bid if everything else passed.
pub fn deterministic_auction_script(
    rng: &mut DetRng,
    config: RulesConfig,
    shaker: Seat,
) -> [AuctionAction; 4] {
    let mut auction = rob_core::AuctionState::new(shaker);
    let mut script = [AuctionAction::Pass; 4];
    for (k, slot) in script.iter_mut().enumerate() {
        let legal = auction.legal_actions(config);
        let mut action = legal[rng.below(legal.len())];
        if k == 3 && auction.current_high_bid().is_none() && action == AuctionAction::Pass {
            action = *legal
                .iter()
                .find(|a| !matches!(a, AuctionAction::Pass))
                .expect("a nonpass bid is always legal at an empty auction");
        }
        auction = auction.apply(action, config).expect("legal action");
        *slot = action;
    }
    script
}

/// The deterministic sweep hand (BRIEF §8 `r_obj_conservation` positive
/// witness): the bidder holds the whole sixes incidence and declares sixes
/// trump, so the declaring side takes all seven tricks and exactly 42.
pub fn sweep_hand(config: RulesConfig) -> Result<PlayedHand, ObjectiveError> {
    let shaker = Seat::ALL[0];
    let bidder = shaker.next(); // acts first, bids, everyone else passes
    let trump_hand = rob_core::natural_incidence(rob_core::PIPS[6]);
    let rest: Vec<DominoId> = all_ids().filter(|&d| !trump_hand.contains(d)).collect();
    let mut hands = [DominoSet::empty(); 4];
    hands[bidder.index()] = trump_hand;
    let mut cursor = 0;
    for &s in Seat::ALL.iter().filter(|&&s| s != bidder) {
        hands[s.index()] = DominoSet::from_ids(rest[cursor..cursor + 7].iter().copied());
        cursor += 7;
    }
    let deal = DealWorld::new(hands)?;
    let bid = AuctionAction::Bid(BidValue::Point(PointAmount::new(30).expect("30")));
    let script = [
        bid,
        AuctionAction::Pass,
        AuctionAction::Pass,
        AuctionAction::Pass,
    ];
    play_hand(
        config,
        shaker,
        deal,
        &script,
        Declaration::PipTrump(rob_core::PIPS[6]),
        |_, _| 0,
    )
}
