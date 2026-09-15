use std::time::Duration;
use walt::policy_search::{partner_rollout as r, request};
use walt::solver::Deadline;

const MISSED: &str = "decl 3\nbid 30\nbidder 0\nseat 2\nhand 0 2 3 15 17 21 25\nplays 0 7 1 24 2 3 3 6 1 5 2 17 3 23 0 12 1 4 2 21 3 22 0 8 0 9 1 19 2 25 3 18 0 13 1 16\nseed 420600\n";

#[test]
fn higher_contracts_reach_the_root_and_continuation_policy() {
    for bid in [31, 36, 42] {
        let text = MISSED.replace("bid 30", &format!("bid {bid}"));
        let (f, seed) = request::from_text_with_seed(&text).unwrap();
        assert_eq!(f.exercise.position.bid, bid);
        let mut policy = r::L1::with_bid(
            f.exercise.position.decl,
            0,
            bid as u8,
            seed,
            Deadline::after(Duration::from_secs(14)),
        );
        let history = f
            .history
            .iter()
            .map(|(s, t)| (s.index(), t.index()))
            .collect::<Vec<_>>();
        let choice = policy.choose(2, f.original.bits(), &history).unwrap();
        let report: serde_json::Value = serde_json::from_str(
            &walt::solver::partnership_wire::run(&format!(
                "baseline\n{text}n 40\nn0 8\nn1 2\nbudget_ms 14000\n"
            ))
            .unwrap(),
        )
        .unwrap();
        assert_eq!(choice, report["choice"].as_u64().unwrap() as usize);
    }
}
