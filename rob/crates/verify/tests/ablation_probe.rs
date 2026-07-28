//! Exploratory ablation probes (not receipt rows unless promoted by
//! amendment): counting-only rob (window 1 everywhere) vs the baseline,
//! and full rob vs counting-only rob head-to-head.

use rob_core::{
    apply_auction_action, apply_play, begin_contracted_play, begin_deal_attempt, close_auction,
    complete_hand, initial_contracted_mechanical, update_support, CloseAuctionOutcome, MatchState,
    Play, RulesConfig, Seat, DOMINO_COUNT,
};
use rob_player::player::UtilityLens;
use rob_player::solver::gate::solve_with_budget;
use rob_player::{placeholder_auction_script, placeholder_declaration};
use rob_verify::corpus::{deterministic_deal, DetRng};

const DEAL_SEED: u64 = 0x0d41_0000;
const DEALS: u64 = 100;

/// One hand, both teams played by rob with per-team window budgets.
fn rob_vs_rob_hand(deal_index: u64, budget_by_team: [u128; 2]) -> [u32; 2] {
    let config = RulesConfig::new(7, 7).expect("valid config");
    let shaker = Seat::ALL[(deal_index % 4) as usize];
    let mut rng = DetRng::new(DEAL_SEED + deal_index);
    let deal = deterministic_deal(&mut rng);
    let (match_state, _) = MatchState::start(config, shaker);
    let (mut attempt, m_auction, _, _) =
        begin_deal_attempt(&match_state, deal, deal_index).expect("NEED_DEAL");
    for action in placeholder_auction_script() {
        let (next, _) = apply_auction_action(&attempt, action, config).expect("legal");
        attempt = next;
    }
    let pending = match close_auction(attempt, &m_auction, config).expect("closes") {
        CloseAuctionOutcome::Pending(p) => p,
        CloseAuctionOutcome::AllPass(_) => unreachable!("the placeholder always bids"),
    };
    let bidder = pending.win().bidder();
    let declaration = placeholder_declaration(deal.hand(bidder));
    let (mut objective, m_play, _) = begin_contracted_play(pending, declaration, &m_auction, config)
        .expect("AUCTION precondition");
    let contract = *objective.state().contract();
    let mut viewers: [rob_core::MechanicalState; 4] = core::array::from_fn(|s| {
        initial_contracted_mechanical(Seat::ALL[s], *deal.hand(Seat::ALL[s]), contract)
            .expect("seven-tile hands")
    });
    for _ in 0..DOMINO_COUNT as u64 {
        let actor = objective.state().current_actor().expect("play phase");
        let budget = budget_by_team[actor.team().index()];
        let plan = solve_with_budget(&viewers[actor.index()], UtilityLens::Points, budget)
            .expect("Points lens solves");
        let domino = plan.root.action;
        let (next, _, _) = apply_play(&objective, domino).expect("legal play");
        objective = next;
        for viewer in &mut viewers {
            *viewer = update_support(viewer, Play { actor, domino }).expect("support update");
        }
    }
    let (result, _) = complete_hand(&objective, &m_play).expect("settles");
    result.final_points
}

fn par_deals<T: Send>(f: impl Fn(u64) -> T + Sync) -> Vec<T> {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Mutex;
    let workers = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
        .min(6);
    let cursor = AtomicUsize::new(0);
    let results: Mutex<Vec<Option<T>>> = Mutex::new((0..DEALS as usize).map(|_| None).collect());
    std::thread::scope(|scope| {
        for _ in 0..workers {
            scope.spawn(|| loop {
                let i = cursor.fetch_add(1, Ordering::Relaxed);
                if i >= DEALS as usize {
                    break;
                }
                let value = f(i as u64);
                results.lock().expect("workers healthy")[i] = Some(value);
            });
        }
    });
    results
        .into_inner()
        .expect("workers healthy")
        .into_iter()
        .map(|v| v.expect("filled"))
        .collect()
}

#[test]
fn myopic_rob_vs_baseline() {
    // Budget 1 floors every window at one trick: counting engine at every
    // decision — "counting the whole way down".
    let start = std::time::Instant::now();
    let (a, b, net, decisions, _) = rob_verify::p4::paired_match(Some(1));
    println!(
        "MYOPIC vs BASELINE: seatings {a} / {b}; NET {net} over 200 hands ({decisions} decisions) in {:?}",
        start.elapsed()
    );
}

#[test]
fn full_rob_vs_myopic_rob() {
    const FULL: u128 = 1 << 28;
    let start = std::time::Instant::now();
    let margins = par_deals(|deal| {
        // Mirrored: full rob on team 0 then team 1, same deal.
        let p0 = rob_vs_rob_hand(deal, [FULL, 1]);
        let p1 = rob_vs_rob_hand(deal, [1, FULL]);
        (p0[0] as i64 - p0[1] as i64) + (p1[1] as i64 - p1[0] as i64)
    });
    let net: i64 = margins.iter().sum();
    println!(
        "FULL vs MYOPIC (rob vs rob, mirrored 200 hands): NET {net} for full-depth in {:?}",
        start.elapsed()
    );
}
