use std::{io::{self,BufRead},time::Instant};
use serde::Deserialize;
use serde_json::{json,Value};
use native_frontier::{Fixture,Caps,Service,outer_query,reference};

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
}
fn budgets()->Vec<usize>{vec![4,2,2,2]}
fn run(inp:Input)->Result<Value,String> {
    let started=Instant::now();
    if inp.rows.is_empty() || inp.rows.len()>2048 || inp.budgets.is_empty() || inp.budgets.len()>4 || inp.budgets.iter().any(|&n|n==0 || n>1024) || inp.field_level>2 {return Err("input cap/schema".into());}
    let tick=Instant::now();let queries=inp.rows.iter().map(|r|outer_query(r,&inp.budgets,inp.counted,&inp.mode,inp.field_level)).collect::<Result<Vec<_>,_>>()?;let validation_us=tick.elapsed().as_micros();
    let mut expected=Vec::new();let mut ref_us=0;let mut ref_nodes=0;
    if inp.reference && !inp.reference_after {for q in &queries {let (v,us,nodes)=reference(q,inp.parallel_reference,inp.caps.seconds.min(10))?;ref_us+=us;ref_nodes+=nodes;expected.push(v);}}
    let service=Service::new(inp.caps.clone())?;let mut service=if inp.native_choices {service.with_native_choices()} else {service};let tick=Instant::now();let answer=match service.root_answers(&queries) {Ok(v)=>v,Err(e)=>return Ok(json!({"error":e,"refusal_us":tick.elapsed().as_micros(),"stats":service.stats,"completed_cache_entries":service.cache_len()}))};let cold_us=tick.elapsed().as_micros();
    if inp.reference && inp.reference_after {for q in &queries {let (v,us,nodes)=reference(q,inp.parallel_reference,inp.caps.seconds.min(10))?;ref_us+=us;ref_nodes+=nodes;expected.push(v);}}
    if inp.reference && expected!=answer {return Err(format!("native parity mismatch: expected {expected:?}, got {answer:?}"));}
    let cold_stats=serde_json::to_value(&service.stats).unwrap();let warm_us=if inp.warm {let tick=Instant::now();let warm=service.root_answers(&queries)?;if warm!=answer {return Err("warm mismatch".into());}Some(tick.elapsed().as_micros())} else {None};
    Ok(json!({"answers":answer,"cold_stats":cold_stats,"final_stats":service.stats,"outer_validation_us":validation_us,"frontier_cold_us":cold_us,"frontier_warm_us":warm_us,"reference_solve_us":ref_us,"reference_nodes":ref_nodes,"reference_parallel":inp.parallel_reference,"all_vectors_and_choices_equal":inp.reference,"request_total_us":started.elapsed().as_micros(),"prepared":[queries.iter().map(|q|json!({"actor":q.actor,"hand":q.hand,"seeds":match &q.nature {native_frontier::Nature::Native(s)=>Some(s),_=>None},"state":{"played":q.state.played,"leader":q.state.leader,"plays":q.state.plays[..q.state.len as usize],"t1":q.state.t1,"t0":q.state.t0}})).collect::<Vec<_>>()]}))
}
fn main(){
    rayon::ThreadPoolBuilder::new().num_threads(4).build_global().unwrap();
    for line in io::stdin().lock().lines() {let tick=Instant::now();let value=match line {Ok(s)=>serde_json::from_str::<Input>(&s).map_err(|e|e.to_string()).and_then(run),Err(e)=>Err(e.to_string())};let output=match value {Ok(v)=>v,Err(e)=>json!({"error":e,"refusal_us":tick.elapsed().as_micros()})};println!("{}",output);}
}
