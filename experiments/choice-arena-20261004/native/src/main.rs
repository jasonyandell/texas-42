use std::{io::{self,BufRead},time::Instant};
use serde::Deserialize;
use serde_json::{json,Value};
use native_frontier::{Fixture,Caps,Service,outer_query,reference};
use rayon::prelude::*;

#[derive(Deserialize)]
struct Input {
    rows:Vec<Fixture>,mode:String,
    #[serde(default)] field_level:usize,
    #[serde(default="budgets")] budgets:Vec<usize>,
    #[serde(default)] counted:bool,
    #[serde(default)] caps:Caps,
    #[serde(default)] reference:bool,
    #[serde(default)] parallel_reference:bool,
    #[serde(default)] warm:bool,
    #[serde(default)] reference_after:bool,
    #[serde(default)] native_choices:bool,
    #[serde(default)] bounded_choices:bool,
    #[serde(default)] eager_zero:bool,
    #[serde(default, alias="arena_choices")] with_arena_choices:bool,
    #[serde(default)] hashed_prefixes:bool,
    #[serde(default="one")] workers:usize,
    #[serde(default)] reference_only:bool,
    #[serde(default)] batch_reference:bool,
}
fn budgets()->Vec<usize>{vec![4,2,2,2]}
fn one()->usize {1}
fn combined(stats:&[Value])->Value {
    let mut out=serde_json::Map::new();for v in stats {for (key,value) in v.as_object().unwrap() {
        if let Some(a)=value.as_array() {let prior=out.entry(key.clone()).or_insert_with(||json!([])).as_array_mut().unwrap();if prior.len()<a.len() {prior.resize(a.len(),json!(0));}for (i,x) in a.iter().enumerate() {prior[i]=json!(prior[i].as_u64().unwrap()+x.as_u64().unwrap());}}
        else {let prior=out.entry(key.clone()).or_insert_with(||json!(0));*prior=json!(prior.as_u64().unwrap()+value.as_u64().unwrap());}
    }}Value::Object(out)
}
fn references(qs:&[native_frontier::Query],parallel:bool,batch:bool,seconds:u64)->Result<(Vec<native_frontier::Answer>,u128,u64),String> {
    let values=if batch {qs.par_iter().map(|q|reference(q,parallel,seconds)).collect::<Vec<_>>()} else {qs.iter().map(|q|reference(q,parallel,seconds)).collect::<Vec<_>>()};
    let mut out=Vec::new();let mut us=0;let mut nodes=0;for v in values {let (a,t,n)=v?;out.push(a);us+=t;nodes+=n;}Ok((out,us,nodes))
}
fn run(inp:Input)->Result<Value,String> {
    let started=Instant::now();
    if inp.rows.is_empty() || inp.rows.len()>2048 || inp.budgets.is_empty() || inp.budgets.len()>4 || inp.budgets.iter().any(|&n|n==0 || n>1024) || inp.field_level>2 || ![1,4].contains(&inp.workers) {return Err("input cap/schema".into());}
    let tick=Instant::now();let queries=inp.rows.iter().map(|r|outer_query(r,&inp.budgets,inp.counted,&inp.mode,inp.field_level)).collect::<Result<Vec<_>,_>>()?;let validation_us=tick.elapsed().as_micros();
    let mut expected=Vec::new();let mut ref_us=0;let mut ref_total_us=0;let mut ref_nodes=0;
    if inp.reference && (!inp.reference_after || inp.reference_only) {let t=Instant::now();let (v,us,nodes)=references(&queries,inp.parallel_reference,inp.batch_reference,inp.caps.seconds.min(10))?;ref_total_us=t.elapsed().as_micros();ref_us=us;ref_nodes=nodes;expected=v;}
    if inp.reference_only {if !inp.reference {return Err("reference_only requires reference".into());}return Ok(json!({"answers":expected,"outer_validation_us":validation_us,"reference_solve_us":ref_us,"reference_total_us":ref_total_us,"reference_nodes":ref_nodes,"request_total_us":started.elapsed().as_micros(),"reference_parallel":inp.parallel_reference,"batch_reference":inp.batch_reference}));}
    let tick=Instant::now();
    let mut chunks=vec![Vec::new();inp.workers];let mut indices=vec![Vec::new();inp.workers];for (i,q) in queries.iter().enumerate() {chunks[i%inp.workers].push(q.clone());indices[i%inp.workers].push(i);}
    if usize::from(inp.native_choices)+usize::from(inp.bounded_choices)+usize::from(inp.with_arena_choices)>1 {return Err("choice modes are exclusive".into());}
    let mut services=(0..inp.workers).map(|_|{let s=Service::new(inp.caps.clone())?;let s=if inp.hashed_prefixes {s.with_hashed_prefixes()} else {s};let s=if inp.bounded_choices {s.with_bounded_choices()} else {s};let s=if inp.with_arena_choices {s.with_arena_choices()} else {s};let s=if inp.eager_zero {s.with_eager_zero()} else {s};Ok(if inp.native_choices {s.with_native_choices()} else {s})}).collect::<Result<Vec<_>,String>>()?;
    let result=services.par_iter_mut().zip(chunks.par_iter()).map(|(s,q)|s.root_answers(q)).collect::<Vec<_>>();
    if let Some(error)=result.iter().find_map(|r|r.as_ref().err()) {return Ok(json!({"error":error,"refusal_us":tick.elapsed().as_micros(),"worker_stats":services.iter().map(|s|serde_json::to_value(&s.stats).unwrap()).collect::<Vec<_>>(),"completed_cache_entries":services.iter().map(Service::cache_len).sum::<usize>()}));}
    let mut ordered=vec![None;queries.len()];for (ids,r) in indices.iter().zip(result) {for (&i,a) in ids.iter().zip(r?) {ordered[i]=Some(a);}}let answer=ordered.into_iter().map(Option::unwrap).collect::<Vec<_>>();let cold_us=tick.elapsed().as_micros();
    if inp.reference && inp.reference_after {let t=Instant::now();let (v,us,nodes)=references(&queries,inp.parallel_reference,inp.batch_reference,inp.caps.seconds.min(10))?;ref_total_us=t.elapsed().as_micros();ref_us=us;ref_nodes=nodes;expected=v;}
    if inp.reference && expected!=answer {return Err(format!("native parity mismatch: expected {expected:?}, got {answer:?}"));}
    let worker_stats=services.iter().map(|s|serde_json::to_value(&s.stats).unwrap()).collect::<Vec<_>>();let cold_stats=combined(&worker_stats);
    let warm_us=if inp.warm {let tick=Instant::now();let warm=services.par_iter_mut().zip(chunks.par_iter()).map(|(s,q)|s.root_answers(q)).collect::<Vec<_>>();for (ids,r) in indices.iter().zip(warm) {for (&i,a) in ids.iter().zip(r?) {if a!=answer[i] {return Err("warm mismatch".into());}}}Some(tick.elapsed().as_micros())} else {None};
    let final_stats=combined(&services.iter().map(|s|serde_json::to_value(&s.stats).unwrap()).collect::<Vec<_>>());
    Ok(json!({"answers":answer,"cold_stats":cold_stats,"worker_stats":worker_stats,"workers":inp.workers,"final_stats":final_stats,"outer_validation_us":validation_us,"frontier_cold_us":cold_us,"frontier_warm_us":warm_us,"reference_solve_us":ref_us,"reference_total_us":ref_total_us,"reference_nodes":ref_nodes,"reference_parallel":inp.parallel_reference,"all_vectors_and_choices_equal":inp.reference,"request_total_us":started.elapsed().as_micros(),"prepared":[queries.iter().map(|q|json!({"actor":q.actor,"hand":q.hand,"seeds":match &q.nature {native_frontier::Nature::Native(s)=>Some(s),_=>None},"state":{"played":q.state.played,"leader":q.state.leader,"plays":q.state.plays[..q.state.len as usize],"t1":q.state.t1,"t0":q.state.t0}})).collect::<Vec<_>>()]}))
}
fn main(){
    rayon::ThreadPoolBuilder::new().num_threads(4).build_global().unwrap();
    for line in io::stdin().lock().lines() {let tick=Instant::now();let value=match line {Ok(s)=>serde_json::from_str::<Input>(&s).map_err(|e|e.to_string()).and_then(run),Err(e)=>Err(e.to_string())};let output=match value {Ok(v)=>v,Err(e)=>json!({"error":e,"refusal_us":tick.elapsed().as_micros()})};println!("{}",output);}
}
