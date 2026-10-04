use native_frontier::{ActorSpec,Context,Nature,Query,State,Tables};
use serde_json::{json,Value};
use std::{fs,sync::Arc,time::Duration};
use walt::{rules::Seat,solver::{self,Contract,Deadline,Field,Key,Shared,Solver,SplitMix64}};
fn mix(x:u64)->u64 {let mut z=x.wrapping_add(0x9E3779B97F4A7C15);z=(z^(z>>30)).wrapping_mul(0xBF58476D1CE4E5B9);z=(z^(z>>27)).wrapping_mul(0x94D049BB133111EB);z^(z>>31)}
struct Rng(u64);
impl Rng {fn next(&mut self)->u64 {self.0=self.0.wrapping_add(0x9E3779B97F4A7C15);let z=self.0.wrapping_sub(0x9E3779B97F4A7C15);mix(z)}fn below(&mut self,n:u64)->usize {loop {let x=self.next();if x<u64::MAX-u64::MAX%n{return (x%n)as usize}}}}
fn record(k:&Key)->u64 {let mut x=mix(k.played as u64);x=mix(x^((k.leader as u64)<<32));for &p in &k.plays {x=mix(x^(0x100|p as u64));}x}
fn sample(actor:usize,hand:u32,k:&Key,sizes:[usize;4],voids:[u32;4],n:usize,rng:&mut Rng)->(Vec<[u32;4]>,usize) {
 let mut pool=(0..28).filter(|t|(solver::FULL_MASK&!k.played&!hand)&(1<<t)!=0).collect::<Vec<_>>();let mut rows=vec![];let mut attempts=0;
 while rows.len()<n {attempts+=1;assert!(attempts<1000000);for i in (1..pool.len()).rev(){let j=rng.below((i+1)as u64);pool.swap(i,j);}
 let mut w=[0;4];w[actor]=hand;let mut off=0;let mut ok=true;for s in 0..4 {if s!=actor {for &t in &pool[off..off+sizes[s]]{w[s]|=1<<t;}off+=sizes[s];if w[s]&voids[s]!=0 {ok=false;break;}}}if ok {rows.push(w)}
 }(rows,attempts)
}
fn u(v:&Value,k:&str)->usize {v[k].as_u64().unwrap()as usize}
fn main(){
 let rows:Vec<Value>=serde_json::from_str(&fs::read_to_string(std::env::args().nth(1).unwrap_or("experiments/resumable-choice-20261004/checks/integration-roots.json".into())).unwrap()).unwrap();let mut out=vec![];let mut actor_checks=0;let mut duplicates=0;let mut rejections=0;
 for (ri,r) in rows.iter().enumerate(){let decl=u(r,"decl");let bidder=u(r,"bidder");let seat=u(r,"seat");let pairs=r["plays"].as_array().unwrap().chunks(2).map(|p|(p[0].as_u64().unwrap()as usize,p[1].as_u64().unwrap()as usize)).collect::<Vec<_>>();let st=solver::replay(solver::decl_of(decl),bidder,&pairs);let actor=(seat+st.r)%4;let original=r["hand"].as_array().unwrap().iter().fold(0u32,|m,t|m|(1<<t.as_u64().unwrap()));let hand=original&!st.played;
 let key=Key {played:st.played,leader:st.leader,plays:st.plays,banked_t1:st.banked_t1,banked_t0:st.banked_t0,voids:None,alive:0};let sizes=Contract::Straight{bid:30}.sizes(&key,0,7);let init=r["seed"].as_u64().unwrap()^mix(original as u64)^record(&key);let mut rng=Rng(init);let (worlds,attempts)=sample(actor,hand,&key,sizes,st.voids,40,&mut rng);let mut prod=SplitMix64(init);assert_eq!(worlds,solver::sample_belief(actor,hand,key.played,sizes,st.voids,40,&mut prod).unwrap());assert_eq!(rng.0,prod.0);rejections+=attempts-40;duplicates+=40-worlds.iter().collect::<std::collections::HashSet<_>>().len();
 let context=Context{decl,bid:30,budgets:vec![4,2,2,2],boundary:st.trick_start_played,boundary_size:7-st.completed,counted:false};let state=State::from_key(&key);
 for level in 0..=2 {let a=ActorSpec{context:context.clone(),state,actor:actor as u8,hand,level};let p=native_frontier::prepare_actor(&a,Deadline::after(Duration::from_secs(5))).unwrap();let tag=if level==0{0}else{mix(0x4C32^level as u64)};let mut ir=Rng(solver::INNER_SEED^tag^mix(actor as u64)^mix(hand as u64)^record(&key));let (iw,_)=sample(actor,hand,&key,sizes,[0;4],a.context.budgets[level],&mut ir);assert_eq!(p.worlds,iw);if level==0{assert_eq!(p.nature,Nature::Native((0..4).map(|_|ir.next()).collect()));}
 if ri%4==0 {let expected=native_frontier::reference(&p,false,5).unwrap().0;let sh=Arc::new(Shared::new(solver::decl_of(decl),30,vec![4,2,2,2],context.boundary,context.boundary_size,Deadline::after(Duration::from_secs(5))));let s=Solver::new(sh.clone(),Seat::from_index(actor).unwrap(),hand,actor%2==1,worlds.clone(),vec![],Field::Level(2));let lm=Tables::new(decl).legal(state,hand);let choice=s.modeled_choice(level,&key,Seat::from_index(actor).unwrap(),hand,lm).unwrap();assert_eq!(choice,expected.choice);let before=sh.pi_calls_by_level();assert_eq!(choice,s.modeled_choice(level,&key,Seat::from_index(actor).unwrap(),hand,lm).unwrap());if level>0{assert_eq!(before,sh.pi_calls_by_level());}actor_checks+=1;}}
 let q=Query{context,state,actor:actor as u8,hand,worlds:worlds.clone(),nature:Nature::Policy(2)};let v=native_frontier::reference(&q,false,5).unwrap().0;out.push(json!({"request":r,"evaluation":v,"worlds":worlds,"final_rng":rng.0,"attempts":attempts}));
 }
 println!("{}",json!({"roots":rows.len(),"ordered_outer_worlds":rows.len()*40,"duplicate_outer_rows":duplicates,"outer_rejected_draws":rejections,"prepared_actor_rng_checks":rows.len()*3,"native_repeated_actor_choice_checks":actor_checks,"cases":out}));
}
