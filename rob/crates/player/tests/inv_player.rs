//! Named invariant tests for the evening player (Jason's spec): every
//! rollout sums to 42; sampled worlds never violate a derived void. Plus
//! seeded determinism of the whole self-play stack.

use rob_core::{
    algebra_for, all_ids, apply_auction_action, apply_play, begin_contracted_play,
    begin_deal_attempt, close_auction, derive_rule_cells, initial_contracted_mechanical,
    legal_plays, update_support, CloseAuctionOutcome, DealWorld, DominoId, DominoSet,
    MechanicalState, Play, RulesConfig, Seat, DOMINO_COUNT,
};
use rob_player::{
    finish_hand, placeholder_auction_script, placeholder_declaration, sample_worlds,
    self_play_match, MonteCarloPlayer, RandomLegal, RolloutPosition, SplitMix64, UtilityLens,
};

fn deterministic_deal(rng: &mut SplitMix64) -> DealWorld {
    let mut ids: Vec<DominoId> = all_ids().collect();
    for i in (1..DOMINO_COUNT).rev() {
        ids.swap(i, rng.below((i + 1) as u64) as usize);
    }
    let hands: [DominoSet; 4] =
        core::array::from_fn(|s| DominoSet::from_ids(ids[s * 7..(s + 1) * 7].iter().copied()));
    DealWorld::new(hands).expect("valid deal")
}

/// Drive one placeholder-auction hand `plays` moves in through the S2
/// machine with random legal play, mirroring into viewer 0's mechanical
/// state.
fn mid_hand_state(seed: u64, plays: usize) -> MechanicalState {
    let config = RulesConfig::new(2, 7).expect("valid config");
    let mut rng = SplitMix64::new(seed);
    let deal = deterministic_deal(&mut rng);
    let (m0, _) = rob_core::MatchState::start(config, Seat::ALL[0]);
    let (mut attempt, m1, _, _) = begin_deal_attempt(&m0, deal, 0).expect("begin");
    for action in placeholder_auction_script() {
        let (next, _) = apply_auction_action(&attempt, action, config).expect("legal");
        attempt = next;
    }
    let pending = match close_auction(attempt, &m1, config).expect("closes") {
        CloseAuctionOutcome::Pending(p) => p,
        CloseAuctionOutcome::AllPass(_) => unreachable!("placeholder always bids"),
    };
    let bidder = pending.win().bidder();
    let declaration = placeholder_declaration(deal.hand(bidder));
    let (mut objective, _, _) =
        begin_contracted_play(pending, declaration, &m1, config).expect("begin play");
    let viewer = Seat::ALL[0];
    let mut state =
        initial_contracted_mechanical(viewer, *deal.hand(viewer), *objective.state().contract())
            .expect("seven tiles");
    for _ in 0..plays {
        let legal: Vec<DominoId> = legal_plays(&objective).iter().collect();
        let actor = objective.state().current_actor().expect("play phase");
        let domino = legal[rng.below(legal.len() as u64) as usize];
        let (next, _, _) = apply_play(&objective, domino).expect("legal");
        objective = next;
        state = update_support(&state, Play { actor, domino }).expect("update");
    }
    state
}

/// INVARIANT (Jason): every rollout sums to exactly 42 points — from fresh
/// deals and from mid-trick positions.
#[test]
fn inv_rollout_conservation() {
    for seed in 0..24u64 {
        let mut rng = SplitMix64::new(0xC0_5EED ^ seed);
        let deal = deterministic_deal(&mut rng);
        let bidder = Seat::ALL[(seed % 4) as usize];
        let declaration = placeholder_declaration(deal.hand(bidder));
        let algebra = algebra_for(declaration);
        let mut position = RolloutPosition {
            hands: core::array::from_fn(|s| *deal.hand(Seat::ALL[s])),
            leader: bidder,
            trick: Vec::new(),
            points: [0, 0],
        };
        // Advance a random number of plays before handing to the policy, so
        // mid-trick starts are covered too.
        let advance = (seed % 11) as usize;
        for _ in 0..advance {
            let legal = position.legal(&algebra);
            let choice = legal[rng.below(legal.len() as u64) as usize];
            position.apply(&algebra, choice);
        }
        let final_points = finish_hand(
            &algebra,
            position,
            &mut RandomLegal(SplitMix64::new(seed.wrapping_mul(0x9e37))),
        );
        assert_eq!(
            final_points[0] + final_points[1],
            42,
            "rollout conserves 42"
        );
    }
}

/// INVARIANT (Jason): sampled worlds never violate a derived void — checked
/// directly against the declaration's follow relation, independently of the
/// sampler's own fiber membership assertion, on positions deep enough that
/// real voids exist.
#[test]
fn inv_sampled_world_voids() {
    let mut states_with_voids = 0u32;
    for seed in 0..12u64 {
        let state = mid_hand_state(seed, 12 + (seed % 9) as usize);
        let algebra = algebra_for(state.contract().declaration());
        let cells = derive_rule_cells(&state);
        let hidden = state.hidden_seats();
        if hidden.iter().any(|&s| !state.public_voids(s).is_empty()) {
            states_with_voids += 1;
        }
        let mut rng = SplitMix64::new(0xB01D ^ seed);
        for world in sample_worlds(&cells, 40, &mut rng) {
            let mut union = DominoSet::empty();
            for (i, &seat) in hidden.iter().enumerate() {
                let hand = &world.hidden_hands[i];
                // Void consistency: no tile in the hand follows any derived
                // void context of that seat.
                for q in state.public_voids(seat).iter() {
                    assert!(
                        !hand.iter().any(|d| algebra.follows(d, q)),
                        "a sampled hand violates a derived void"
                    );
                }
                // Capacity, disjointness, conservation.
                assert_eq!(hand.len(), cells.capacity(i), "exact capacity");
                assert!(union.is_disjoint(hand), "hands are disjoint");
                union = union.union(hand);
            }
            assert_eq!(union, *cells.unseen_pool(), "hands exhaust the pool");
        }
    }
    assert!(
        states_with_voids >= 3,
        "the corpus must actually exercise derived voids (got {states_with_voids})"
    );
}

/// Seeded determinism: the full self-play stack replays byte-identically
/// from the same seeds.
#[test]
fn player_self_play_deterministic() {
    let config = RulesConfig::new(2, 7).expect("valid config");
    let player = MonteCarloPlayer {
        worlds_per_decision: 4,
        lens: UtilityLens::Points,
        seed: 11,
    };
    let a = self_play_match(config, &player, 99);
    let b = self_play_match(config, &player, 99);
    assert_eq!(a.lines, b.lines, "same seeds, same transcript");
    assert_eq!(a.final_marks, b.final_marks);
}
