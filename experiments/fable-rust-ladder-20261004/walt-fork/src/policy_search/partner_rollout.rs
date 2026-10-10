//! Bounded, paired continuation guesses around an already completed L1 move.
//!
//! Every compared world uses every legal first action and the SAME completed
//! deployed L1 procedure afterward. No perfect-information continuation is
//! substituted. A completed prefix of a shuffled finite support is a heuristic
//! sample, NOT a calibrated confidence claim; only a census is exact.
use std::collections::HashMap;
use crate::clock::Instant;

use super::{Fixture, State};
use crate::gym;
use crate::kernel::{SplitMix64 as KernelRng, World};
use crate::rules::{legal_plays, Domino, DominoSet, Seat};
use crate::solver::partnership::{self, Config, FieldProfile};
use crate::solver::selection::Rule;
use crate::solver::{self, mask_bits, mask_of, set_of, Deadline, Key};

pub const ID: &str = "partner-rollout-v1";
pub const FIELD: &str = "completed-deployed-l1-fixed-40-8-voidless-request-seed-v1";
pub const QUERY: &str = super::partner_review::QUERY;
pub const MAX_SUPPORT: u128 = 400;
pub const MAX_SAMPLES: usize = 64;
pub const MIN_SAMPLES: usize = 8;
pub const STREAM_DOMAIN: u64 = 0x5041_4952_524F_4C4C;

#[derive(Clone, Debug)]
pub struct Decision {
    pub seat: usize,
    pub original: u32,
    pub history: Vec<(usize, usize)>,
    pub choice: usize,
}

type Information = (usize, u32, Vec<(usize, usize)>);

/// Only own original hand and public history cross this policy boundary.
/// Configuration, declaration, bidder and seed are fixed for this instance.
pub struct L1 {
    decl: crate::rules::Decl,
    bidder: usize,
    bid: u8,
    seed: u64,
    deadline: Deadline,
    cache: HashMap<Information, usize>,
    pub decisions: Vec<Decision>,
    pub hits: usize,
}

impl L1 {
    pub fn new(decl: crate::rules::Decl, bidder: usize, seed: u64, deadline: Deadline) -> Self {
        Self::with_bid(decl, bidder, 30, seed, deadline)
    }

    pub fn with_bid(decl: crate::rules::Decl, bidder: usize, bid: u8, seed: u64, deadline: Deadline) -> Self {
        assert!((30..=42).contains(&bid));
        Self {
            decl,
            bidder,
            bid,
            seed,
            deadline,
            cache: HashMap::new(),
            decisions: Vec::new(),
            hits: 0,
        }
    }

    /// Same replay, rotation, sample seed and evaluator as `partnership`.
    /// A timed-out continuation is refused, never replaced by a guessed tile.
    pub fn choose(
        &mut self,
        actor: usize,
        original: u32,
        history: &[(usize, usize)],
    ) -> Result<usize, String> {
        if self.deadline.passed() {
            return Err("deadline".into());
        }
        let info = (actor, original, history.to_vec());
        if let Some(&choice) = self.cache.get(&info) {
            self.hits += 1;
            return Ok(choice);
        }
        let st = solver::replay(self.decl, self.bidder, history);
        let seat = Seat::from_index((actor + st.r) % 4).ok_or("invalid actor")?;
        if seat.index() != (st.leader as usize + st.plays.len()) % 4 || st.completed == 7 {
            return Err("not actor's turn".into());
        }
        let hand = original & !st.played;
        let led = st.plays.first().map(|&t| {
            self.decl
                .led_context(Domino::from_index(t as usize).unwrap())
        });
        let legal = mask_of(legal_plays(self.decl, set_of(hand), led));
        let choice = if legal.count_ones() == 1 {
            mask_bits(legal)[0] as usize
        } else {
            let key = Key {
                voids: None,
                played: st.played,
                leader: st.leader,
                plays: st.plays.clone(),
                banked_t1: st.banked_t1,
                banked_t0: st.banked_t0,
                alive: 0,
            };
            let mut sizes = [7 - st.completed; 4];
            for i in 0..st.plays.len() {
                sizes[(st.leader as usize + i) % 4] -= 1;
            }
            let cfg = Config {
                inner_belief: solver::InnerBelief::Voidless,
                selection: Rule::Fixed,
                modeled_selection: Rule::Fixed,
                profile: FieldProfile::Baseline,
                n_outer: 40,
                n0: 8,
                n1: 2,
                seed: self.seed ^ solver::mix(u64::from(original)) ^ solver::record_hash(&key),
                deadline: self.deadline,
            };
            partnership::evaluate(
                self.decl,
                self.bid,
                seat,
                hand,
                legal,
                &key,
                sizes,
                st.voids,
                st.trick_start_played,
                7 - st.completed,
                &cfg,
            )
            .map_err(|failure| match &failure.reason {
                partnership::RefusalReason::Deadline => "continuation deadline".to_owned(),
                partnership::RefusalReason::InfeasibleFrame(frame) => {
                    format!("infeasible continuation frame: {frame}")
                }
            })?
            .best() as usize
        };
        self.cache.insert(info, choice);
        self.decisions.push(Decision {
            seat: actor,
            original,
            history: history.to_vec(),
            choice,
        });
        Ok(choice)
    }
}

#[derive(Clone, Debug)]
pub struct Trace {
    pub hands: [DominoSet; 4],
    pub action: usize,
    pub history: Vec<(usize, usize)>,
    pub made: bool,
    pub banked: [u32; 2],
}

pub struct Review {
    pub choice: usize,
    pub offers: Vec<usize>,
    pub legal: Vec<usize>,
    pub values: Vec<usize>,
    pub paired: Vec<[usize; 4]>, // gained, lost, both make, both set vs baseline
    pub support: usize,
    pub samples: usize,
    pub requested: usize,
    pub status: &'static str,
    pub stop: &'static str,
    pub coverage: &'static str,
    pub traces: Vec<Trace>,
    pub decisions: Vec<Decision>,
    pub cache_hits: usize,
    pub elapsed_us: u128,
}

fn play(f: &Fixture, world: &World, action: usize, policy: &mut L1) -> Result<Trace, String> {
    let mut state = State::from_root(&f.exercise.position);
    let mut hands = world.hands();
    let mut original = hands;
    let mut history = f
        .history
        .iter()
        .map(|(s, t)| (s.index(), t.index()))
        .collect::<Vec<_>>();
    for (s, t) in &f.history {
        original[s.index()].insert(*t);
    }
    assert_eq!(
        original[f.exercise.root.kernel().viewer().index()],
        f.original
    );
    let tile = Domino::from_index(action).unwrap();
    let actor = state.actor();
    assert!(legal_plays(
        f.exercise.position.decl,
        hands[actor.index()],
        f.exercise.frame.led_context()
    )
    .contains(tile));
    hands[actor.index()].remove(tile);
    history.push((actor.index(), action));
    state = state.step(f.exercise.position.decl, tile);
    // The contract outcome is absorbing; completing irrelevant plays cannot
    // change make/set and needlessly spends the player's investigation budget.
    loop {
        if let Some(made) = state.success(&f.exercise.position) {
            return Ok(Trace {
                hands: world.hands(),
                action,
                history,
                made,
                banked: state.banked,
            });
        }
        let actor = state.actor();
        let next = policy.choose(actor.index(), original[actor.index()].bits(), &history)?;
        let tile = Domino::from_index(next).unwrap();
        let led = state
            .prefix
            .first()
            .map(|d| f.exercise.position.decl.led_context(*d));
        assert!(legal_plays(f.exercise.position.decl, hands[actor.index()], led).contains(tile));
        hands[actor.index()].remove(tile);
        history.push((actor.index(), next));
        state = state.step(f.exercise.position.decl, tile);
    }
}

pub fn review(
    f: &Fixture,
    seed: u64,
    baseline: usize,
    deadline: Deadline,
    max_samples: usize,
    audit: bool,
) -> Result<Review, String> {
    let start = Instant::now();
    if !(1..=400).contains(&max_samples) {
        return Err("samples outside 1..400".into());
    }
    let ex = &f.exercise;
    let viewer = ex.root.kernel().viewer();
    let legal = legal_plays(
        ex.position.decl,
        ex.root.kernel().viewer_hand(),
        ex.frame.led_context(),
    );
    let ids = legal.iter().map(Domino::index).collect::<Vec<_>>();
    let base_index = ids
        .iter()
        .position(|&t| t == baseline)
        .ok_or("baseline must be legal")?;
    let mut result = Review {
        choice: baseline,
        offers: Vec::new(),
        legal: ids.clone(),
        values: vec![0; ids.len()],
        paired: vec![[0; 4]; ids.len()],
        support: ex.root.count() as usize,
        samples: 0,
        requested: max_samples.min(ex.root.count() as usize),
        status: "inactive",
        stop: "scope",
        coverage: "none",
        traces: Vec::new(),
        decisions: Vec::new(),
        cache_hits: 0,
        elapsed_us: 0,
    };
    if viewer.team() != ex.position.declaring_team
        || ex.root.kernel().viewer_hand().len() > 3
        || legal.len() < 2
        || State::from_root(&ex.position)
            .success(&ex.position)
            .is_some()
    {
        return Ok(result);
    }
    let found = gym::match_query(ex, QUERY, MAX_SUPPORT, 1_000_000)?;
    assert!(found.public, "deployment query must be own/public");
    for (tile, mass) in found.presence {
        assert_eq!(mass, ex.root.count());
        result.offers.push(tile);
    }
    if result.offers.is_empty() || result.offers.len() == ids.len() {
        return Ok(result);
    }
    if ex.root.count() > MAX_SUPPORT {
        result.status = "unresolved";
        result.stop = "support-cap";
        return Ok(result);
    }
    let bidder = f
        .history
        .first()
        .ok_or("late-game history required")?
        .0
        .index();
    let mut policy = L1::with_bid(ex.position.decl, bidder, ex.position.bid as u8, seed, deadline);
    let mut worlds = ex.root.worlds().collect::<Vec<_>>();
    KernelRng::new(seed ^ ex.position.identity() ^ STREAM_DOMAIN).shuffle(&mut worlds);
    result.stop = if result.requested == result.support {
        "census"
    } else {
        "sample-cap"
    };
    for world in worlds.iter().take(result.requested) {
        if deadline.passed() {
            result.stop = "deadline";
            break;
        }
        let mut vector = Vec::new();
        for &action in &ids {
            match play(f, world, action, &mut policy) {
                Ok(trace) => vector.push(trace),
                Err(error) if error == "deadline" || error == "continuation deadline" => {
                    result.stop = "deadline";
                    break;
                }
                Err(error) => return Err(error),
            }
        }
        if vector.len() != ids.len() {
            break;
        }
        // Atomic evidence unit: never count a partially compared world.
        let baseline_made = vector[base_index].made;
        for (i, trace) in vector.iter().enumerate() {
            result.values[i] += usize::from(trace.made);
            let column = match (trace.made, baseline_made) {
                (true, false) => 0,
                (false, true) => 1,
                (true, true) => 2,
                (false, false) => 3,
            };
            result.paired[i][column] += 1;
        }
        result.samples += 1;
        if audit {
            result.traces.extend(vector);
        }
    }
    result.coverage = if result.samples == result.support {
        "census"
    } else {
        "budgeted-sample-prefix"
    };
    if result.samples < MIN_SAMPLES.min(result.support) {
        result.status = "unresolved";
    } else {
        let mut best = base_index;
        for i in 0..ids.len() {
            if result.values[i] > result.values[best] {
                best = i;
            }
        }
        result.choice = ids[best];
        result.status = if result.choice == baseline {
            "retained"
        } else {
            "changed"
        };
    }
    result.cache_hits = policy.hits;
    result.decisions = policy.decisions;
    result.elapsed_us = start.elapsed().as_micros();
    Ok(result)
}
