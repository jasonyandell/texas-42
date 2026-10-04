//! First-disagreement certificate using the pinned solver's public policy API.
//! Per-world focal branching is ONLY an upper bound on disagreement reachability.
//! It is never substituted for a lawful sampled-response payoff.
use std::{collections::HashMap, io::{self, BufRead}, sync::{Arc, atomic::Ordering}, time::{Duration,Instant}};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::ToPrimitive;
use serde::Deserialize;
use serde_json::{json,Value};
use walt::{rules::{Decl,Domino,Seat,Team,legal_plays},solver::{self,Key,PiKey,Shared,Solver,Field,Deadline,SplitMix64}};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request { decl:usize,bid:u8,bidder:usize,seat:usize,hand:Vec<usize>,plays:Vec<usize>,seed:u64 }
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Input { request:Request, #[serde(default="forty")] outer:usize,
    #[serde(default="one")] cheap_n0:usize, #[serde(default="eight")] target_n0:usize,
    #[serde(default="seconds")] seconds:u64, #[serde(default)] target_first:bool,
    #[serde(default)] field_level:usize }
fn forty()->usize{40} fn one()->usize{1} fn eight()->usize{8} fn seconds()->u64{20}

fn legal(dcl:Decl,hand:u32,key:&Key)->u32 {
    let led=key.plays.first().map(|t|dcl.led_context(Domino::from_index(*t as usize).unwrap()));
    solver::mask_of(legal_plays(dcl,solver::set_of(hand),led))
}
fn state_key(key:&Key)->(u32,u8,Vec<u8>,u8,u8){(key.played,key.leader,key.plays.clone(),key.banked_t1,key.banked_t0)}
type StateKey=(u32,u8,Vec<u8>,u8,u8);
struct Certificate<'a>{ cheap:&'a Solver,target:&'a Solver,worlds:&'a [[u32;4]],viewer:Seat,
    level:usize, memo:HashMap<(usize,StateKey),bool>,
    cheap_queries:HashMap<PiKey,u8>,target_queries:HashMap<PiKey,u8>,
    visits:u64,hits:u64,forced:u64,disagreements:u64,deadline:Deadline }
impl Certificate<'_>{
    fn field(&mut self,key:&Key,actor:Seat,hand:u32,moves:u32,cheap:bool)->Result<u8,String>{
        let info=PiKey{voids:None,seat:actor.index() as u8,hand,played:key.played,
            leader:key.leader,plays:key.plays.clone(),banked_t1:key.banked_t1,banked_t0:key.banked_t0};
        let cache=if cheap {&mut self.cheap_queries}else{&mut self.target_queries};
        if let Some(a)=cache.get(&info){self.hits+=1;return Ok(*a)}
        let solver=if cheap{self.cheap}else{self.target};
        let choice=solver.modeled_choice(self.level,key,actor,hand,moves).ok_or("modeled query refused")?;
        assert!(moves&(1<<choice)!=0);
        cache.insert(info,choice);Ok(choice)
    }
    fn bad(&mut self,world:usize,key:&Key)->Result<bool,String>{
        self.visits+=1;
        if self.deadline.passed(){return Err("certificate deadline; no certificate".into())}
        if self.cheap.sh.contract.terminal(key).is_some(){return Ok(false)}
        let mk=(world,state_key(key));
        if let Some(v)=self.memo.get(&mk){self.hits+=1;return Ok(*v)}
        let actor=Seat::from_index((key.leader as usize+key.plays.len())%4).unwrap();
        let hand=self.worlds[world][actor.index()]&!key.played;
        let moves=legal(self.cheap.sh.dcl,hand,key);assert_ne!(moves,0);
        let result=if actor==self.viewer {
            let mut found=false;
            for tile in solver::mask_bits(moves){
                let child=self.cheap.child_after_play(key,Domino::from_index(tile as usize).unwrap(),0);
                if self.bad(world,&child)?{found=true;break}
            }
            found
        }else{
            let (a,b)=if moves.count_ones()==1 {self.forced+=1;let t=moves.trailing_zeros() as u8;(t,t)}
                else {(self.field(key,actor,hand,moves,true)?,self.field(key,actor,hand,moves,false)?)};
            if a!=b {self.disagreements+=1;true}
            else {let child=self.cheap.child_after_play(key,Domino::from_index(a as usize).unwrap(),0);self.bad(world,&child)?}
        };
        self.memo.insert(mk,result);Ok(result)
    }
}

fn make_solver(dcl:Decl,bid:u8,n0:usize,boundary:u32,boundary_size:usize,deadline:Deadline,
    seat:Seat,hand:u32,worlds:Vec<[u32;4]>,level:usize)->Solver{
    let sh=Arc::new(Shared::new(dcl,bid,vec![n0,2],boundary,boundary_size,deadline));
    Solver::new(sh,seat,hand,seat.team()==Team::T1,worlds,Vec::new(),Field::Level(level))
}
fn counts(solver:&Solver,key:&Key,tiles:&[u8],n:usize)->Result<Vec<u64>,String>{
    solver.action_values(key,tiles).ok_or("comparison refused")?.iter().map(|(_,r)|{
        let v=r*BigRational::from_integer(BigInt::from(n));assert!(v.is_integer());
        let v=v.to_integer().to_u64().unwrap();
        Ok(if solver.sh.contract.is_nello(){unreachable!()}else{v})
    }).collect()
}
fn work(solver:&Solver)->Value{json!({"pi_calls":solver.sh.pi_calls_by_level(),
    "inner_worlds":solver.sh.inner_worlds_by_level(),"nodes":solver.sh.nodes.load(Ordering::Relaxed)})}
fn best(tiles:&[u8],values:&[u64])->u8{
    let mut ix=0;for i in 1..tiles.len(){if values[i]>values[ix]{ix=i}}
    tiles[ix]
}
fn run(input:Input)->Result<Value,String>{
    let start=Instant::now();
    if !(1..=40).contains(&input.outer)||!(1..=8).contains(&input.cheap_n0)
        ||!(1..=8).contains(&input.target_n0)||!(1..=60).contains(&input.seconds)||input.field_level>1 {
        return Err("invalid probe budget".into())
    }
    let req=input.request;
    if req.hand.len()!=7||req.plays.len()%2!=0||req.bidder>3||req.seat>3||!(30..=42).contains(&req.bid)
        ||!([0,1,2,3,4,5,6,7,9].contains(&req.decl)){return Err("invalid request".into())}
    let dcl=solver::decl_of(req.decl);
    let mut original=0u32;for t in req.hand{if t>=28||original&(1<<t)!=0{return Err("invalid hand".into())}original|=1<<t;}
    let pairs:Vec<_>=req.plays.chunks(2).map(|p|(p[0],p[1])).collect();
    let st=solver::replay(dcl,req.bidder,&pairs);
    let seat=Seat::from_index((req.seat+st.r)%4).unwrap();
    if seat.index()!=(st.leader as usize+st.plays.len())%4{return Err("not actor".into())}
    let hand=original&!st.played;
    let key=Key{voids:None,played:st.played,leader:st.leader,plays:st.plays.clone(),
        banked_t1:st.banked_t1,banked_t0:st.banked_t0,alive:0};
    let tiles=solver::mask_bits(legal(dcl,hand,&key));
    let contract=solver::Contract::Straight{bid:req.bid};
    if tiles.len()<2||contract.terminal(&key).is_some(){return Err("requires an unsettled free decision".into())}
    let sizes=contract.sizes(&key,st.trick_start_played,7-st.completed);
    let deadline=Deadline::after(Duration::from_secs(input.seconds));
    // Same stream as the production fixed evaluator. Outer watchdog and a
    // positive-fiber precheck bound rejection sampling at the process boundary.
    let mut rng=SplitMix64(req.seed^solver::mix(u64::from(original))^solver::record_hash(&key));
    let sampling=Instant::now();
    let worlds=solver::sample_belief(seat.index(),hand,st.played,sizes,st.voids,input.outer,&mut rng).map_err(|e|format!("{e:?}"))?;
    let sampling_us=sampling.elapsed().as_micros();
    let create=|n0|make_solver(dcl,req.bid,n0,st.trick_start_played,7-st.completed,deadline,seat,hand,worlds.clone(),input.field_level);
    let cheap=create(input.cheap_n0);let cert_target=create(input.target_n0);let exact=create(input.target_n0);
    let mut exact_us=0;let mut exact_counts=Vec::new();
    if input.target_first {let t=Instant::now();exact_counts=counts(&exact,&key,&tiles,input.outer)?;exact_us=t.elapsed().as_micros();}
    let t=Instant::now();let mut cheap_counts=counts(&cheap,&key,&tiles,input.outer)?;let cheap_us=t.elapsed().as_micros();
    let cheap_work=work(&cheap);
    let t=Instant::now();
    let mut cert=Certificate{cheap:&cheap,target:&cert_target,worlds:&worlds,viewer:seat,level:input.field_level,
        memo:HashMap::new(),cheap_queries:HashMap::new(),target_queries:HashMap::new(),
        visits:0,hits:0,forced:0,disagreements:0,deadline};
    let mut errors=Vec::new();let mut flags=Vec::new();
    for tile in &tiles{
        let child=cheap.child_after_play(&key,Domino::from_index(*tile as usize).unwrap(),0);
        let mut bad=Vec::new();for w in 0..worlds.len(){bad.push(cert.bad(w,&child)?)}
        errors.push(bad.iter().filter(|x|**x).count() as u64);flags.push(bad);
    }
    let cert_us=t.elapsed().as_micros();
    let cert_work=json!({"visits":cert.visits,"memo_states":cert.memo.len(),"cache_hits":cert.hits,
        "cheap_distinct_queries":cert.cheap_queries.len(),"target_distinct_queries":cert.target_queries.len(),
        "forced_steps":cert.forced,"disagreement_stops":cert.disagreements,
        "target_work":work(&cert_target),"cheap_total_work":work(&cheap)});
    if !input.target_first{let t=Instant::now();exact_counts=counts(&exact,&key,&tiles,input.outer)?;exact_us=t.elapsed().as_micros();}
    if seat.team()!=Team::T1 {for x in &mut cheap_counts{*x=input.outer as u64-*x}for x in &mut exact_counts{*x=input.outer as u64-*x}}
    let cheap_choice=best(&tiles,&cheap_counts);let exact_choice=best(&tiles,&exact_counts);
    let incumbent=tiles.iter().position(|a|*a==cheap_choice).unwrap();
    let lower:Vec<_>=cheap_counts.iter().zip(&errors).map(|(v,e)|v.saturating_sub(*e)).collect();
    let upper:Vec<_>=cheap_counts.iter().zip(&errors).map(|(v,e)|(*v+*e).min(input.outer as u64)).collect();
    let certified=(0..tiles.len()).all(|i|i==incumbent||upper[i]<lower[incumbent]
        ||(upper[i]<=lower[incumbent]&&cheap_choice<tiles[i]));
    for i in 0..tiles.len(){assert!(cheap_counts[i].abs_diff(exact_counts[i])<=errors[i]);assert!(lower[i]<=exact_counts[i]&&exact_counts[i]<=upper[i]);}
    if certified{assert_eq!(cheap_choice,exact_choice)}
    Ok(json!({"schema":"walt-first-disagreement-v1","outer":input.outer,"cheap_n0":input.cheap_n0,
        "target_n0":input.target_n0,"field_level":input.field_level,"target_first":input.target_first,
        "tiles":tiles,"cheap_counts":cheap_counts,"target_counts":exact_counts,"error_counts":errors,
        "lower":lower,"upper":upper,"bad_scenario_flags":flags,"worlds":worlds,
        "cheap_choice":cheap_choice,"target_choice":exact_choice,"certified":certified,
        "sampling_us":sampling_us,"cheap_us":cheap_us,"certificate_us":cert_us,"target_us":exact_us,
        "cheap_work":cheap_work,"certificate_work":cert_work,"target_work":work(&exact),
        "total_us":start.elapsed().as_micros(),"value_orientation":"focal team success counts"}))
}
fn main(){for line in io::stdin().lock().lines(){let line=line.unwrap();if line.trim().is_empty(){continue}
    let value=serde_json::from_str::<Input>(&line).map_err(|e|e.to_string()).and_then(run);
    println!("{}",match value{Ok(v)=>v,Err(e)=>json!({"error":e,"certified":false})});
}}
