//! Observation-only addressed response ladder shared by native and wasm32.
mod rules;
mod search;
mod support;
#[cfg(test)]
mod tests;
use super::{PlayContract, Request, Seed};
use rules::{Public, Rules};
use search::{child_path, Search};
use serde_json::{json, Value};
use std::time::Duration;
use walt::clock::Instant;
type Mask = u32;
type World = [Mask; 4];
type Key = [u64; 13];
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Timeout,
    Invalid(&'static str),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Timeout => f.write_str("decision deadline"),
            Self::Invalid(s) => f.write_str(s),
        }
    }
}
impl std::error::Error for Error {}
#[inline]
fn mix(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}
fn key_hash(key: &Key) -> u64 {
    key.iter()
        .fold(0x243f6a8885a308d3, |h, &word| mix(h ^ word))
}
struct Random(u64);
impl Random {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        mix(self.0)
    }
    fn bounded(&mut self, n: u64) -> u64 {
        let threshold = n.wrapping_neg() % n;
        loop {
            let x = self.next();
            if x >= threshold {
                return x % n;
            }
        }
    }
}
/// Resumable between complete root alternatives. Internal lower queries remain
/// recursive; a future GPU executor needs explicit lower-query continuations.
/// No physical deal or enclosing-search fiber can enter this API.
pub struct Job {
    search: Search,
    public: Public,
    worlds: Vec<World>,
    fibers: Vec<usize>,
    path: u64,
    legal: Mask,
    pending: Mask,
    scores: [i32; 28],
    level: usize,
    shift: usize,
    start: Instant,
    failed: bool,
    finished: Option<Value>,
}
impl Job {
    pub fn new(req: &Request, samples: &[usize], budget_ms: u64) -> Result<Self, Error> {
        let start = Instant::now();
        let nello = req.contract == Some(PlayContract::Nello);
        let invalid_contract = if nello {
            req.decl != 8 || !(1..=9).contains(&req.bid)
        } else {
            ![0, 1, 2, 3, 4, 5, 6, 7, 9].contains(&req.decl) || !(30..=42).contains(&req.bid)
        };
        if req.bidder > 3
            || req.seat > 3
            || req.hand.len() != 7
            || !req.plays.len().is_multiple_of(2)
            || req.plays.len() > if nello { 40 } else { 54 }
            || budget_ms == 0
            || budget_ms > 30000
            || samples.is_empty()
            || samples.len() > 4
            || samples.iter().any(|&n| n == 0 || n > 4096)
            || invalid_contract
        {
            return Err(Error::Invalid("invalid observation, contract or profile"));
        }
        let seed = match &req.seed {
            Seed::Integer(s) => *s,
            Seed::Decimal(s) => s.parse().map_err(|_| Error::Invalid("invalid seed"))?,
        };
        let shift = (5 - req.bidder as usize) % 4;
        let me = (req.seat as usize + shift) % 4;
        let rules = Rules::new(req.decl as usize, req.bid as u8, nello);
        let mut p = Public::initial();
        let mut hand = 0;
        for &t in &req.hand {
            if t > 27 || hand & (1 << t) != 0 {
                return Err(Error::Invalid("invalid original hand"));
            }
            hand |= 1 << t;
        }
        for pair in req.plays.chunks_exact(2) {
            let (s, t) = (pair[0], pair[1]);
            if s > 3 || t > 27 || rules.outcome(p).is_some() {
                return Err(Error::Invalid("invalid public play"));
            }
            let actor = (s as usize + shift) % 4;
            let bit = 1 << t;
            if actor != rules.turn(p) || p.played & bit != 0 || p.voids[actor] & bit != 0 {
                return Err(Error::Invalid("public play contradicts history"));
            }
            if actor == me {
                if rules.legal(hand, p) & bit == 0 {
                    return Err(Error::Invalid("illegal own play"));
                }
                hand &= !bit;
            } else if hand & bit != 0 {
                return Err(Error::Invalid("another seat played our tile"));
            }
            p = rules.play(p, t as usize);
        }
        if rules.turn(p) != me || rules.outcome(p).is_some() {
            return Err(Error::Invalid("not a live decision for this seat"));
        }
        let search = Search::new(
            rules,
            req.decl as usize,
            samples.to_vec(),
            seed,
            start + Duration::from_millis(budget_ms),
        );
        let level = samples.len();
        let worlds = search.deals(level, p, hand)?;
        let fibers = (0..worlds.len()).collect();
        let path = search.path(level, p, hand);
        let legal = search.rules.legal(hand, p);
        Ok(Self {
            search,
            public: p,
            worlds,
            fibers,
            path,
            legal,
            pending: legal,
            scores: [-1; 28],
            level,
            shift,
            start,
            failed: false,
            finished: None,
        })
    }
    /// No scores are published until every root alternative completes.
    pub fn step(&mut self) -> Result<Option<Value>, Error> {
        if self.failed {
            return Err(Error::Invalid("job already failed"));
        }
        if let Some(value) = &self.finished {
            return Ok(Some(value.clone()));
        }
        let result = self.advance();
        if result.is_err() {
            self.failed = true;
        }
        result
    }
    fn advance(&mut self) -> Result<Option<Value>, Error> {
        self.search.check()?;
        let t = self.pending.trailing_zeros() as usize;
        let p = self.public;
        let me = self.search.rules.turn(p);
        let v = self.search.search(
            self.level,
            self.search.rules.play(p, t),
            me,
            &self.worlds,
            &self.fibers,
            child_path(self.path, t),
            None,
        )?;
        debug_assert!(v.exact);
        self.scores[t] = v.count;
        self.search.check()?;
        self.pending &= self.pending - 1;
        if self.pending != 0 {
            return Ok(None);
        }
        let legal = (0..28)
            .filter(|&t| self.legal & (1 << t) != 0)
            .collect::<Vec<_>>();
        let mut choice = legal[0];
        for &t in &legal {
            if if me % 2 == 1 {
                self.scores[t] > self.scores[choice]
            } else {
                self.scores[t] < self.scores[choice]
            } {
                choice = t;
            }
        }
        let points = [p.t0, p.t1];
        let worlds = self.worlds.len();
        let mut value = json!({"schema":"partnership-decision-v1","trick":p.depth/self.search.rules.trick_size()+1,"choice":choice,"legal":legal,"leader":(p.leader+4-self.shift)%4,
            "points":[points[self.shift%2],points[1-self.shift%2]],"elapsed_us":self.start.elapsed().as_micros() as u64,
            "player_version":"walt-table-v3","route":"baseline","inner_belief":"void-consistent",
            "profile":{"delta":1,"level":self.level,"samples":self.search.samples},
            "evaluation":{"outer_worlds":worlds,"options":legal.iter().map(|&t|json!([t,self.scores[t].to_string(),worlds.to_string()])).collect::<Vec<_>>()},
            "phases":[{"name":"baseline","status":"completed","worlds":worlds}],
            "metrics":{"nodes":self.search.nodes,"pruned":self.search.pruned}});
        if self.search.rules.nello {
            value["contract"] = json!("nello");
            value["inactive"] = json!((3 + 4 - self.shift) % 4);
        }
        self.finished = Some(value.clone());
        Ok(Some(value))
    }
}
/// Budgeted deployment procedure. A profile selects sample counts and opponent
/// models; it never selects a second implementation or changes the observation.
pub(super) fn decide(
    call: super::Call,
    mut checkpoint: impl FnMut(&Value),
) -> Result<Value, String> {
    let start = Instant::now();
    if !(100..=20000).contains(&call.budget_ms) || !(1..=640).contains(&call.worlds) {
        return Err("invalid time or sampling budget".into());
    }
    let profile = call
        .profile
        .clone()
        .unwrap_or_else(|| vec![24, call.worlds]);
    if profile.last() != Some(&call.worlds) {
        return Err("profile outer sample count differs from worlds".into());
    }
    let validated = Job::new(&call.request, &profile, call.budget_ms).map_err(|e| e.to_string())?;
    let legal = (0..28)
        .filter(|&t| validated.legal & (1 << t) != 0)
        .collect::<Vec<_>>();
    let p = validated.public;
    let points = [p.t0, p.t1];
    let shift = validated.shift;
    drop(validated);
    let nello = call.request.contract == Some(PlayContract::Nello);
    let mut value = json!({"schema":"partnership-decision-v1","trick":p.depth/(if nello{3}else{4})+1,"choice":legal[0],"legal":legal,"leader":(p.leader+4-shift)%4,
        "points":[points[shift%2],points[1-shift%2]],"route":if legal.len()==1{"forced"}else{"legal-fallback"},
        "player_version":"walt-table-v3","requested_profile":{"samples":profile},"budget_ms":call.budget_ms,
        "evaluation":null,"phases":[]});
    if nello {
        value["contract"] = json!("nello");
        value["inactive"] = json!((call.request.bidder + 2) % 4);
    }
    super::emit(&mut value, start, call.budget_ms, &mut checkpoint);
    if legal.len() == 1 {
        return Ok(value);
    }
    let defense =
        call.nello_counterexamples && nello && call.request.seat % 2 != call.request.bidder % 2;
    let reserve = if defense {
        super::counterexample::MAX_MS.min(call.budget_ms / 2)
    } else {
        0
    };
    let mut stages = vec![vec![call.worlds.min(8)]];
    if call.worlds > 8 {
        stages.push(vec![call.worlds]);
    }
    if profile != *stages.last().unwrap() {
        stages.push(profile.clone());
    }
    for samples in stages {
        let remaining = call
            .budget_ms
            .saturating_sub(start.elapsed().as_millis() as u64)
            .saturating_sub(reserve)
            .saturating_sub(50);
        let budget = if samples == vec![call.worlds.min(8)] {
            remaining.min(1500)
        } else {
            remaining
        };
        let run = (|| {
            if budget == 0 {
                return Err(Error::Timeout);
            }
            let mut job = Job::new(&call.request, &samples, budget)?;
            loop {
                if let Some(v) = job.step()? {
                    return Ok(v);
                }
            }
        })();
        match run {
            Ok(v) => {
                value = v;
                value["requested_profile"] = json!({"samples":profile});
                value["budget_ms"] = json!(call.budget_ms);
            }
            Err(e) => {
                value["interruption"] = json!(format!(
                    "L{} comparison stopped: {e}. Retained the last completed decision.",
                    samples.len()
                ));
            }
        }
        super::emit(&mut value, start, call.budget_ms, &mut checkpoint);
    }
    // Preserve the existing Nel-O witness defense and its separate score meaning.
    if defense && value["route"] == "baseline" {
        let ms = call
            .budget_ms
            .saturating_sub(start.elapsed().as_millis() as u64)
            .min(reserve);
        let baseline = value["choice"].as_u64().unwrap();
        let n = value["evaluation"]["outer_worlds"].as_u64().unwrap() as usize;
        let mut publish = |report: &Value| {
            if report["status"] == "completed" {
                value["choice"] = report["choice"].clone();
                value["route"] = json!("baseline-counterexamples");
            }
            value["counterexample_result"] = report.clone();
            super::emit(&mut value, start, call.budget_ms, &mut checkpoint);
        };
        let report = super::counterexample::review(&call.request, n, baseline, ms, &mut publish);
        publish(&report);
    }
    super::emit(&mut value, start, call.budget_ms, &mut checkpoint);
    Ok(value)
}
