//! Exploratory probe: WHY does rob slough the 5-0 (not the "obvious" 3-0)
//! at P4-stream deal 1, trick 4, position 2? Replays the chosen plan and
//! the rejected 3-0 plan against every world of the 8,400-world fiber
//! under the solve's own model (plan vs σ) and tallies where the nickel
//! ends up. Sum of per-world margins must reproduce each plan's exact
//! root value — the probe validates itself against the solver.

use rob_core::{
    algebra_for, apply_auction_action, begin_contracted_play, begin_deal_attempt, close_auction,
    derive_rule_cells, domino_id, initial_contracted_mechanical, update_support,
    CloseAuctionOutcome, DeclarationAlgebra, Domino, DominoId, DominoSet, MatchState, Pip, Play,
    RulesConfig, Seat,
};
use rob_player::player::UtilityLens;
use rob_player::solver::gate::solve_opening;
use rob_player::{greedy_sigma, PlanChild, PlanNode};
use rob_verify::corpus::{deterministic_deal, DetRng};

const DEAL_SEED: u64 = 0x0d41_0000;

fn tile(name: &str) -> DominoId {
    let (h, l) = name.split_once('-').expect("high-low");
    domino_id(Domino::new(
        Pip::new(h.parse().unwrap()).unwrap(),
        Pip::new(l.parse().unwrap()).unwrap(),
    ))
}

/// The 13 public plays before the decision (extracted from the rob trace,
/// deal index 1).
const PREFIX: [(usize, &str); 13] = [
    (2, "2-2"),
    (3, "6-2"),
    (0, "2-0"),
    (1, "5-2"),
    (2, "3-3"),
    (3, "6-4"),
    (0, "3-1"),
    (1, "3-2"),
    (3, "1-1"),
    (0, "2-1"),
    (1, "6-1"),
    (2, "1-0"),
    (3, "4-0"),
];

struct Sim {
    hands: [DominoSet; 4],
    leader: Seat,
    trick: Vec<Play>,
    banked: [u32; 2],
}

impl Sim {
    fn actor(&self) -> Seat {
        self.leader.offset(self.trick.len() as u8)
    }

    /// Apply one play; on resolution returns (winner, trick contained 5-0).
    fn apply(&mut self, algebra: &DeclarationAlgebra, d: DominoId) -> Option<(Seat, bool)> {
        let actor = self.actor();
        assert!(self.hands[actor.index()].contains(d));
        self.hands[actor.index()].remove(d);
        self.trick.push(Play { actor, domino: d });
        if self.trick.len() == 4 {
            let result = algebra.resolve_trick(&self.trick).expect("resolves");
            let nickel = self.trick.iter().any(|p| p.domino == tile("5-0"));
            self.banked[result.winner.team().index()] += result.points as u32;
            self.leader = result.winner;
            self.trick.clear();
            return Some((result.winner, nickel));
        }
        None
    }
}

/// Replay one plan against one world: returns (final margin for T0, team
/// that captured the 5-0, team that won the current trick).
fn replay(
    algebra: &DeclarationAlgebra,
    node: &PlanNode,
    mut sim: Sim,
    viewer: Seat,
) -> (i64, usize, usize) {
    let mut current = node;
    let mut nickel_team: Option<usize> = None;
    let mut trick4_team: Option<usize> = None;
    let mut obs: Vec<(u8, u8)> = Vec::new();
    let mut pending_action = Some(current.action);
    loop {
        if sim.hands.iter().all(DominoSet::is_empty) {
            let margin = sim.banked[viewer.team().index()] as i64
                - sim.banked[viewer.team().opponent().index()] as i64;
            return (
                margin,
                nickel_team.expect("all tiles played"),
                trick4_team.expect("first trick resolved"),
            );
        }
        let actor = sim.actor();
        let d = if actor == viewer {
            if pending_action.is_none() {
                // Descend to the child for the observed segment.
                match current.children.get(&obs).expect("realizable obs is a key") {
                    PlanChild::Node(n) => current = n,
                    PlanChild::Leaf(_) => unreachable!("full-depth plans settle"),
                }
                obs.clear();
            }
            pending_action.take().unwrap_or(current.action)
        } else {
            let choice = greedy_sigma(algebra, &sim.hands[actor.index()], &sim.trick);
            obs.push((actor.index() as u8, choice.index() as u8));
            choice
        };
        if let Some((winner, nickel)) = sim.apply(algebra, d) {
            if nickel {
                nickel_team = Some(winner.team().index());
            }
            trick4_team.get_or_insert(winner.team().index());
        }
        if actor == viewer {
            pending_action = None;
            // Mark that the next viewer turn needs a descent.
        }
    }
}

#[test]
fn nickel_autopsy() {
    // Reconstruct S0's decision state.
    let config = RulesConfig::new(7, 7).expect("valid config");
    let deal_index = 1u64;
    let shaker = Seat::ALL[(deal_index % 4) as usize];
    let mut rng = DetRng::new(DEAL_SEED + deal_index);
    let deal = deterministic_deal(&mut rng);
    let (match_state, _) = MatchState::start(config, shaker);
    let (mut attempt, m_auction, _, _) =
        begin_deal_attempt(&match_state, deal, deal_index).expect("NEED_DEAL");
    for action in rob_player::placeholder_auction_script() {
        let (next, _) = apply_auction_action(&attempt, action, config).expect("legal");
        attempt = next;
    }
    let pending = match close_auction(attempt, &m_auction, config).expect("closes") {
        CloseAuctionOutcome::Pending(p) => p,
        CloseAuctionOutcome::AllPass(_) => unreachable!(),
    };
    let bidder = pending.win().bidder();
    let declaration = rob_player::placeholder_declaration(deal.hand(bidder));
    let (objective, _, _) = begin_contracted_play(pending, declaration, &m_auction, config)
        .expect("AUCTION precondition");
    let contract = *objective.state().contract();
    let viewer = Seat::ALL[0];
    let mut state =
        initial_contracted_mechanical(viewer, *deal.hand(viewer), contract).expect("seven tiles");
    for (actor, name) in PREFIX {
        state = update_support(
            &state,
            Play {
                actor: Seat::ALL[actor],
                domino: tile(name),
            },
        )
        .expect("legal observation");
    }
    assert_eq!(
        *state.own_remaining_hand(),
        DominoSet::from_ids([tile("3-0"), tile("5-0"), tile("6-3"), tile("6-6")])
    );
    let cells = derive_rule_cells(&state);
    let worlds = cells.fiber_worlds();
    assert_eq!(worlds.len(), 8_400, "the decision's exact fiber");
    let algebra = algebra_for(declaration);

    for opening in ["5-0", "3-0", "6-3", "6-6"] {
        let plan = solve_opening(&state, UtilityLens::Points, tile(opening));
        assert!(!plan.truncated, "probe needs the full tree");
        assert_eq!(plan.root.action, tile(opening));
        let mut margin_sum = 0i64;
        let mut nickel_us = 0u64; // T0 (rob's team) captures the 5-0
        let mut trick4_us = 0u64;
        let mut nickel_us_margin = 0i64;
        let mut nickel_them_margin = 0i64;
        for world in &worlds {
            let mut hands = [DominoSet::empty(); 4];
            hands[viewer.index()] = *state.own_remaining_hand();
            for (i, &seat) in state.hidden_seats().iter().enumerate() {
                hands[seat.index()] = world.hidden_hands[i];
            }
            let sim = Sim {
                hands,
                leader: state.leader(),
                trick: state.current_trick().to_vec(),
                banked: state.hand_points(),
            };
            let (margin, nickel_team, trick4_team) = replay(&algebra, &plan.root, sim, viewer);
            margin_sum += margin;
            if nickel_team == 0 {
                nickel_us += 1;
                nickel_us_margin += margin;
            } else {
                nickel_them_margin += margin;
            }
            if trick4_team == 0 {
                trick4_us += 1;
            }
        }
        assert_eq!(
            margin_sum, plan.root.value_total,
            "replay reproduces the solver's exact value"
        );
        let nickel_them = worlds.len() as u64 - nickel_us;
        // Integer-only report (INV-4): sums and counts; centiavgs by
        // integer division.
        let centi = |sum: i64, n: u64| -> i64 {
            if n == 0 {
                0
            } else {
                sum * 100 / n as i64
            }
        };
        println!(
            "opening {opening}: value {} (centiavg {}) | nickel US {} worlds (margin sum {}, centiavg {}) / THEM {} worlds (margin sum {}, centiavg {}) | trick4 US {} worlds",
            plan.root.value_total,
            centi(plan.root.value_total, worlds.len() as u64),
            nickel_us,
            nickel_us_margin,
            centi(nickel_us_margin, nickel_us),
            nickel_them,
            nickel_them_margin,
            centi(nickel_them_margin, nickel_them),
            trick4_us,
        );
    }
}
