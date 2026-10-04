//! Independently prepared ordered outer stream and direct frozen native oracle.
use serde_json::{json,Value};
use std::{io::{self,BufRead},sync::Arc,time::Duration};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::ToPrimitive;
use walt::{rules::Seat,solver::{self,Contract,Deadline,Field,Key,Shared,Solver,SplitMix64}};
fn mix(x:u64)->u64 {let mut z=x.wrapping_add(0x9E3779B97F4A7C15);z=(z^(z>>30)).wrapping_mul(0xBF58476D1CE4E5B9);z=(z^(z>>27)).wrapping_mul(0x94D049BB133111EB);z^(z>>31)}
struct Rng(u64);
impl Rng {
    fn next(&mut self)->u64 {let x=self.0;self.0=self.0.wrapping_add(0x9E3779B97F4A7C15);mix(x)}
    fn below(&mut self,n:u64)->usize {loop {let x=self.next();if x<u64::MAX-u64::MAX%n{return (x%n)as usize}}}
}
fn record(key:&Key)->u64 {let mut x=mix(key.played as u64);x=mix(x^((key.leader as u64)<<32));for &tile in &key.plays{x=mix(x^(0x100|tile as u64))}x}
fn number(v:&Value,k:&str)->usize {v[k].as_u64().unwrap()as usize}
fn main() {
 for line in io::stdin().lock().lines() {
  let call:Value=serde_json::from_str(&line.unwrap()).unwrap();let r=&call["request"];
  assert_eq!(r.as_object().unwrap().len(),7);let k=number(&call,"k");assert!((1..=5).contains(&k));
  let decl=number(r,"decl");let bid=number(r,"bid")as u8;let bidder=number(r,"bidder");
  let pairs=r["plays"].as_array().unwrap().chunks(2).map(|p|(p[0].as_u64().unwrap()as usize,p[1].as_u64().unwrap()as usize)).collect::<Vec<_>>();
  let st=solver::replay(solver::decl_of(decl),bidder,&pairs);let actor=(number(r,"seat")+st.r)%4;
  let original=r["hand"].as_array().unwrap().iter().fold(0u32,|m,t|m|(1<<t.as_u64().unwrap()));let hand=original&!st.played;
  let key=Key{played:st.played,leader:st.leader,plays:st.plays,banked_t1:st.banked_t1,banked_t0:st.banked_t0,voids:None,alive:0};
  let sizes=Contract::Straight{bid}.sizes(&key,0,7);
  let initial=r["seed"].as_u64().unwrap()^mix(original as u64)^record(&key);let mut rng=Rng(initial);
  let mut pool=(0..28).filter(|t|(solver::FULL_MASK&!key.played&!hand)&(1<<t)!=0).collect::<Vec<_>>();let mut worlds=vec![];let mut attempts=0;
  while worlds.len()<40 {
   attempts+=1;assert!(attempts<1000000);
   for i in (1..pool.len()).rev(){let j=rng.below((i+1)as u64);pool.swap(i,j)}
   let mut w=[0u32;4];w[actor]=hand;let mut off=0;let mut lawful=true;
   for s in 0..4 {if s!=actor {for &t in &pool[off..off+sizes[s]]{w[s]|=1<<t}off+=sizes[s];if w[s]&st.voids[s]!=0{lawful=false;break}}}
   if lawful{worlds.push(w)}
  }
  let mut production=SplitMix64(initial);assert_eq!(worlds,solver::sample_belief(actor,hand,key.played,sizes,st.voids,40,&mut production).unwrap());assert_eq!(rng.0,production.0);
  let led=key.plays.first().copied().map(|x|solver::decl_of(decl).led_context(walt::rules::Domino::from_index(x as usize).unwrap()));
  let legal=solver::mask_bits(solver::mask_of(walt::rules::legal_plays(solver::decl_of(decl),solver::set_of(hand),led)));
  let sh=Arc::new(Shared::new(solver::decl_of(decl),bid,vec![4,2,2,2,2,2],st.trick_start_played,7-st.completed,Deadline::after(Duration::from_secs(30))));
  let solver=Solver::new(sh.clone(),Seat::from_index(actor).unwrap(),hand,actor%2==1,worlds.clone(),vec![],Field::Level(k-1));
  let values=solver.action_values(&key,&legal);
  let evaluation=values.map(|values|{
   let counts=values.iter().map(|(_,v)|{let n=v*BigRational::from_integer(BigInt::from(40));assert!(n.is_integer());n.to_integer().to_u64().unwrap()}).collect::<Vec<_>>();
   json!({"tiles":legal,"counts":counts,"choice":solver::best_of(&values,actor%2==1)})
  });
  println!("{}",json!({"request":r,"k":k,"field_level":k-1,"worlds":worlds,"rng_final":rng.0.to_string(),"outer_attempts":attempts,"evaluation":evaluation,"pi_calls_by_level":sh.pi_calls_by_level(),"inner_worlds_by_level":sh.inner_worlds_by_level()}));
 }
}
