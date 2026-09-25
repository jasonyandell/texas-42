//! Opt-in Nel-O defender experiment. Only complete joint replans are published.
//! This is the probe's equal-weight witness mixture, not a probability correction.
use super::*;
use std::collections::BTreeSet;
use walt::{
    rules::{Decl, Seat},
    solver::{
        nello_counterexample::{Lab, Root, World},
        Contract, Deadline, Key,
    },
};

// Preview headroom goes to finding failures, without changing their mixture weight.
const ATTACK_WORLDS: usize = 540;
pub const MAX_MS: u64 = 4200;

pub fn review(
    req: &Request,
    worlds: usize,
    baseline: u64,
    ms: u64,
    mut checkpoint: impl FnMut(&Value),
) -> Value {
    let mut result = json!({"schema":"nello-counterexamples-v1","status":"unresolved",
        "baseline":baseline,"choice":baseline,"ordinary_worlds":worlds,"witnesses":0,"rounds":0,
        "options":[],"stop":"deadline","score_kind":"witness-mixture"});
    let completed = (|| -> Option<()> {
        let seed = match &req.seed {
            Seed::Integer(n) => *n,
            Seed::Decimal(s) => s.parse().ok()?,
        };
        let pairs = req
            .plays
            .chunks_exact(2)
            .map(|p| (p[0] as usize, p[1] as usize))
            .collect::<Vec<_>>();
        let st = solver::replay_contract(Decl::DoublesSuit, req.bidder as usize, &pairs, true);
        let original_hand = req.hand.iter().fold(0u32, |m, &t| m | (1u32 << t));
        let root = Root {
            key: Key {
                voids: None,
                played: st.played,
                leader: st.leader,
                plays: st.plays,
                banked_t1: st.banked_t1,
                banked_t0: st.banked_t0,
                alive: 0,
            },
            viewer: Seat::from_index((req.seat as usize + st.r) % 4)?,
            hand: original_hand & !st.played,
            original_hand,
            voids: st.voids,
            contract: Contract::Nello {
                declarer: Seat::from_index((req.bidder as usize + st.r) % 4)?,
            },
        };
        let deadline = Deadline::after(Duration::from_millis(ms));
        let lab = Lab::new(root, 8, deadline)?;
        let ordinary = lab.root.sample(worlds, seed, deadline)?;
        let mut plans = lab.train(&ordinary)?;
        let mut seen: BTreeSet<World> = ordinary.iter().copied().collect();
        let mut witnesses = Vec::new();
        for round in 0..3 {
            let pool = lab.root.sample(
                ATTACK_WORLDS,
                seed ^ solver::mix(0x41545441434b + round as u64),
                deadline,
            )?;
            let outcomes = pool
                .iter()
                .map(|w| {
                    plans
                        .iter()
                        .map(|p| Some(lab.replay(p, w)?.set))
                        .collect::<Option<Vec<_>>>()
                })
                .collect::<Option<Vec<_>>>()?;
            let mut found = 0;
            for offset in 0..plans.len() {
                if found == 4 {
                    break;
                }
                let index = (offset + round + seed as usize % plans.len()) % plans.len();
                let pick = (0..pool.len())
                    .find(|&i| {
                        !seen.contains(&pool[i])
                            && !outcomes[i][index]
                            && outcomes[i].iter().any(|&set| set)
                    })
                    .or_else(|| {
                        (0..pool.len()).find(|&i| !seen.contains(&pool[i]) && !outcomes[i][index])
                    });
                if let Some(i) = pick {
                    seen.insert(pool[i]);
                    witnesses.push(pool[i]);
                    found += 1;
                }
            }
            if found == 0 {
                result["stop"] = json!("no-new-witnesses");
                break;
            }
            let mut bundle = ordinary.clone();
            bundle.extend_from_slice(&witnesses);
            let next = lab.train(&bundle)?;
            let chosen = next.iter().min_by(|a, b| {
                a.declarer_value
                    .cmp(&b.declarer_value)
                    .then(a.action.cmp(&b.action))
            })?;
            result["status"] = json!("completed");
            result["choice"] = json!(chosen.action);
            result["rounds"] = json!(round + 1);
            result["witnesses"] = json!(witnesses.len());
            result["stop"] = json!("round-limit");
            result["options"] = json!(next
                .iter()
                .map(|p| json!([
                    p.action,
                    p.declarer_value.numer().to_string(),
                    p.declarer_value.denom().to_string()
                ]))
                .collect::<Vec<_>>());
            checkpoint(&result);
            plans = next;
        }
        Some(())
    })();
    if completed.is_none() {
        result["stop"] = json!("deadline");
    }
    result
}
