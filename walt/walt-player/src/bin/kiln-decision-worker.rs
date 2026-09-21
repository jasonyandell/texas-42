//! Bounded continuation examiner. The player receives only its normal Call.
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::io::{self, BufRead, Write};
use walt::rules::{legal_plays, Domino, DominoSet, Seat};
use walt::scheme::{Access, Budget, Fix, Value as Answer};
use walt::{gym, solver};
use walt::kernel::SplitMix64;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    decl: usize, bid: usize, bidder: usize, seat: usize,
    hand: Vec<usize>, plays: Vec<usize>, seed: u32,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Query { name: String, source: String }
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Job {
    request: Request, queries: Vec<Query>,
    #[serde(default)] sample_seed: Option<u64>,
    #[serde(default)] baseline: Option<usize>,
    #[serde(default)] sample_only: bool,
}

fn tile(n: usize) -> Result<Domino, String> {
    Domino::from_index(n).ok_or_else(|| "invalid tile".into())
}
fn seat(n: usize) -> Result<Seat, String> {
    Seat::from_index(n).ok_or_else(|| "invalid chair".into())
}
fn policy_call(req: &Request, actor: usize, hand: &[usize], pairs: &[(usize, usize)]) -> Value {
    json!({"request":{"decl":req.decl,"bid":30,"bidder":req.bidder,"seat":actor,
        "hand":hand,"plays":pairs.iter().flat_map(|&(a,t)| [a,t]).collect::<Vec<_>>(),
        "seed":req.seed},"worlds":40,"partner":true,"budget_ms":14000})
}

fn run(text: &str) -> Result<Value, String> {
    assess(text, |call| walt_player::decide(serde_json::from_value(call).map_err(|e| e.to_string())?, |_| {}))
}

fn assess(text: &str, mut choose: impl FnMut(Value) -> Result<Value,String>) -> Result<Value,String> {
    let start = std::time::Instant::now();
    let job: Job = serde_json::from_str(text).map_err(|e| e.to_string())?;
    let req = &job.request;
    if req.bid != 30 || ![0,1,2,3,4,5,6,7,9].contains(&req.decl)
        || req.plays.len()>52 || req.plays.len()%2 != 0
        || req.hand.len() != 7 || job.queries.len() > 80 {
        return Err("expected bid30, unfinished history and at most 80 queries".into());
    }
    let hand: DominoSet = req.hand.iter().map(|&t| tile(t)).collect::<Result<Vec<_>,_>>()?.into_iter().collect();
    let history = req.plays.chunks_exact(2).map(|p| Ok((seat(p[0])?,tile(p[1])?)))
        .collect::<Result<Vec<_>,String>>()?;
    let dcl = solver::decl_of(req.decl);
    let ex = gym::from_request(dcl, seat(req.bidder)?, seat(req.seat)?, hand, &history)?;
    let legal = legal_plays(dcl, ex.root.kernel().viewer_hand(), ex.frame.led_context())
        .iter().map(Domino::index).collect::<Vec<_>>();
    let support = ex.root.count();
    let worlds = if let Some(seed) = job.sample_seed {
        if legal.len()<2 || !job.baseline.is_some_and(|a|legal.contains(&a)) {
            return Err("sampled comparison requires a saved legal baseline and alternatives".into());
        }
        vec![ex.root.kernel().sample(&mut SplitMix64::new(seed)).ok_or("empty support")?]
    } else {
        if !(40..=46).contains(&req.plays.len()) || ex.root.kernel().viewer_hand().len()!=2
            || legal.len()!=2 || support>90 || support==0 || job.baseline.is_some() || job.sample_only {
            return Err("census assessment requires two legal tiles and 1..90 worlds".into());
        }
        ex.root.worlds().collect::<Vec<_>>()
    };
    let mut query_results = BTreeMap::new();
    for q in &job.queries {
        if query_results.contains_key(&q.name) { return Err("duplicate query".into()); }
        let compiled = q.source.parse::<Fix>().map_err(|e|e.to_string())?
            .compile(&gym::registry()).map_err(|e|e.to_string())?;
        let public = compiled.predicate_specs().iter().all(|s|s.access==Access::Viewer);
        let mut memberships = Vec::new();
        for world in &worlds {
            let answers = compiled.evaluate(&ex.frame, world, &mut Budget::new(1_000_000))
                .map_err(|e| e.to_string())?;
            let mut actions = Vec::new();
            for answer in &answers {
                match answer.0.as_slice() {
                    [] => {},
                    [Answer::Domino(d)] if legal.contains(&d.index()) => actions.push(d.index()),
                    _ => return Err("query must return existence or legal actions".into()),
                }
            }
            memberships.push(json!({"exists":!answers.is_empty(),"actions":actions}));
        }
        if public && memberships.windows(2).any(|w|w[0]!=w[1]) {
            return Err("Viewer query depends on hidden world".into());
        }
        query_results.insert(q.name.clone(), json!({"public":public,"memberships":memberships}));
    }
    let mut cache = BTreeMap::<String,usize>::new();
    let mut decisions = Vec::<Value>::new();
    let mut decision = |call: Value| -> Result<(usize,usize),String> {
        let key = call.to_string();
        let index = if let Some(&index) = cache.get(&key) { index } else {
            let response = choose(call.clone())?;
            let index = decisions.len();
            decisions.push(json!({"call":call,"response":response}));
            cache.insert(key,index);
            index
        };
        let choice = decisions[index]["response"]["choice"].as_u64().ok_or("missing choice")? as usize;
        Ok((choice,index))
    };
    let pairs = history.iter().map(|(s,t)|(s.index(),t.index())).collect::<Vec<_>>();
    let (baseline,baseline_index) = if let Some(a) = job.baseline { (a,None) } else {
        let (a,i)=decision(policy_call(req,req.seat,&req.hand,&pairs))?; (a,Some(i))
    };
    if !legal.contains(&baseline) { return Err("illegal baseline".into()); }
    let mut traces = Vec::new();
    let mut full_worlds = Vec::new();
    for (world_index,world) in worlds.iter().enumerate() {
        let mut hands = world.hands().map(|h|h.iter().map(Domino::index).collect::<Vec<_>>());
        for &(s,t) in &pairs { hands[s].push(t); }
        for h in &mut hands { h.sort_unstable(); }
        if hands[req.seat] != req.hand { return Err("original hand mismatch".into()); }
        full_worlds.push(hands.clone());
        if job.sample_only { continue; }
        for &action in &legal {
            let mut record = pairs.clone();
            record.push((req.seat,action));
            let mut ids = Vec::new();
            while record.len()<28 {
                let state = solver::replay(dcl,req.bidder,&record);
                let leader = (state.leader as usize + 4 - state.r)%4;
                let actor = (leader+state.plays.len())%4;
                let mask = hands[actor].iter().fold(0u32,|m,&t|m|(1<<t)) & !state.played;
                let led = state.plays.first().map(|&t|dcl.led_context(Domino::from_index(t as usize).unwrap()));
                let allowed = solver::mask_of(legal_plays(dcl,solver::set_of(mask),led));
                let (choice,index) = decision(policy_call(req,actor,&hands[actor],&record))?;
                if choice>=28 || allowed & (1<<choice)==0 { return Err("illegal continuation".into()); }
                record.push((actor,choice)); ids.push(index);
            }
            let state = solver::replay(dcl,req.bidder,&record);
            let points = if state.r==0 {[state.banked_t0,state.banked_t1]} else {[state.banked_t1,state.banked_t0]};
            traces.push(json!({"world":world_index,"action":action,
                "record":record.iter().flat_map(|&(a,t)|[a,t]).collect::<Vec<_>>(),
                "decisions":ids,"points":points,"made":points[req.bidder%2]>=30}));
        }
    }
    Ok(json!({"schema":if job.sample_seed.is_some() {"kiln-sampled-contrast-v1"} else {"kiln-decision-assessment-v1"},
        "request":serde_json::from_str::<Value>(text).unwrap()["request"],
        "legal":legal,"worlds":full_worlds,"queries":query_results,"baseline":baseline,
        "baseline_decision":baseline_index,"decisions":decisions,"traces":traces,
        "support_worlds":support.to_string(),"sample_seed":job.sample_seed,"sample_only":job.sample_only,
        "elapsed_us":start.elapsed().as_micros() as u64}))
}

fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.expect("read assessment");
        let answer = if line.len()>100_000 { Err("job too large".into()) } else { run(&line) };
        println!("{}", answer.unwrap_or_else(|error|json!({"error":error})));
        io::stdout().flush().expect("flush assessment");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_hidden_inputs_and_early_roots_before_calling_player() {
        let req = json!({"decl":6,"bid":30,"bidder":0,"seat":0,"hand":[0,1,2,3,4,5,6],"plays":[],"seed":1});
        let base = json!({"request":req,"queries":[]});
        assert!(assess(&base.to_string(), |_|panic!("invalid root reached player")).is_err());
        let mut hidden = base;
        hidden["request"]["hands"] = json!([[0,1,2,3,4,5,6]]);
        assert!(serde_json::from_value::<Job>(hidden).is_err());
    }
}
