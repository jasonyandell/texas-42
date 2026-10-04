// Adapted prior demand-bounds checker with attribution; forest() is fresh arena-specific reasoning/testing.
use native_frontier::*;
use serde_json::{Value,json};
use std::{fs,time::Duration,collections::{HashSet,HashMap}};
use walt::solver::Deadline;
fn service()->Service {Service::new(Caps {seconds:60,work:100000000,..Caps::default()}).unwrap()}
fn winners(bounds:&[(u64,u64)],maximize:bool,values:&mut Vec<u64>,seen:&mut HashSet<usize>) {
 if values.len()==bounds.len() {let mut best=0;for i in 1..values.len(){if if maximize {values[i]>values[best]}else{values[i]<values[best]}{best=i;}}seen.insert(best);return;}
 let (lo,hi)=bounds[values.len()];for n in lo..=hi{values.push(n);winners(bounds,maximize,values,seen);values.pop();}
}
fn intervals(n:usize,bounds:&mut Vec<(u64,u64)>,cases:&mut u64) {
 if bounds.len()==n {for maximize in [false,true] {let mut seen=HashSet::new();winners(bounds,maximize,&mut Vec::new(),&mut seen);let expected=if seen.len()==1 {seen.iter().next().copied()}else{None};assert_eq!(certified_index(bounds,maximize),expected);*cases+=1;}return;}
 for lo in 0..=3{for hi in lo..=3 {bounds.push((lo,hi));intervals(n,bounds,cases);bounds.pop();}}
}
static EAGER: std::sync::atomic::AtomicBool=std::sync::atomic::AtomicBool::new(false);
fn config(s:Service)->Service {s.with_arena_choices()}
fn bounded_service()->Service {config(service())}
fn main() {run();trace();forest();extended_trace();}
fn trace() {
 let rows:Vec<Fixture>=serde_json::from_str(&fs::read_to_string("experiments/demand-bounds-20261004/checks/trace-input.json").unwrap()).unwrap();
 let queries=rows.iter().map(|r|outer_query(r,&[4,2,2,2],false,"policy",2).unwrap()).collect::<Vec<_>>();
 let wide=||Service::new(Caps {rows:1000000,work:100000000,queries:100000,cache_entries:100000,seconds:60}).unwrap();
 let mut candidate=config(wide()).with_choice_trace();let answers=candidate.root_answers(&queries).unwrap();
 for(q,a)in queries.iter().zip(&answers){assert_eq!(*a,reference(q,false,20).unwrap().0);}
 let trace=candidate.take_choice_trace();assert_eq!(trace.len() as u64,candidate.stats.actor_demands_by_level.iter().sum::<u64>());assert!(candidate.take_choice_trace().is_empty());
 let mut unique=Vec::new();let mut indices=HashSet::new();for(a,_)in &trace{if indices.insert(a.clone()){unique.push(a.clone());}}unique.sort_by_key(|a|a.level);
 let mut full=wide();let mut native=wide().with_native_choices();let mut expected=HashMap::new();let mut bylevel=Vec::new();
 for level in 0..=2 {let specs=unique.iter().filter(|a|a.level==level).cloned().collect::<Vec<_>>();let a=full.actors(&specs).unwrap();let b=native.actor_choices(&specs).unwrap();for((spec,v),n)in specs.into_iter().zip(a).zip(b){assert_eq!(v.choice,n);expected.insert(spec,v.choice);}bylevel.push(unique.iter().filter(|a|a.level==level).count());}
 for(a,choice)in &trace{assert_eq!(*choice,expected[a]);}
 println!("{}",json!({"all_demand_trace":true,"eager_zero":EAGER.load(std::sync::atomic::Ordering::Relaxed),"roots":queries.len(),"every_demanded_actor_call_compared":trace.len(),"distinct_actor_specs":unique.len(),"distinct_specs_by_level":bylevel,"all_trace_calls_equal_eager_full_and_native_choices":true,"validation_queries_cap":100000,"validation_work_cap":100000000,"trace_disabled_in_timings":true}));
}
fn run() {
 let mut interval_cases=0;for n in 1..=4{intervals(n,&mut Vec::new(),&mut interval_cases);}
 assert_eq!(certified_index(&[],true),None);for i in 0..4 {let mut bad=vec![(0,0);4];bad[i]=(2,1);assert_eq!(certified_index(&bad,true),None);assert_eq!(certified_index(&bad,false),None);}
 assert_eq!(certified_index(&[(u64::MAX,u64::MAX),(u64::MAX,u64::MAX)],true),Some(0));
 let panel:Value=serde_json::from_str(&fs::read_to_string("experiments/native-frontier-20261004/checks/independent-panel.json").unwrap()).unwrap();
 let rows=panel["rows"].as_array().unwrap();let mut roots=Vec::new();let mut scalar=0;
 for row in rows {
  let fixture:Fixture=serde_json::from_value(row.clone()).unwrap();let q=outer_query(&fixture,&[4,2,2,2],false,"native",0).unwrap();
  let got=bounded_service().root_answers(&[q.clone()]).unwrap().remove(0);
  for (&tile,&count) in got.tiles.iter().zip(&got.counts) {let focal=if q.actor%2==1 {count}else{q.worlds.len() as u64-count};assert_eq!(focal,row["expected_native_focal_success"][tile.to_string()].as_u64().unwrap());}
  roots.push(q);scalar+=1;
 }
 let original=bounded_service().root_answers(&roots).unwrap();
 let mut reversed=roots.clone();for q in &mut reversed {q.worlds.reverse();if let Nature::Native(s)=&mut q.nature{s.reverse();}}
 assert_eq!(bounded_service().root_answers(&reversed).unwrap(),original);
 let mut triples=roots.clone();for q in &mut triples {q.worlds=q.worlds.iter().flat_map(|w|[*w,*w,*w]).collect();if let Nature::Native(s)=&mut q.nature{*s=s.iter().flat_map(|v|[*v,*v,*v]).collect();}}
 let multiplied=bounded_service().root_answers(&triples).unwrap();for(a,b)in original.iter().zip(multiplied){assert_eq!(a.choice,b.choice);assert_eq!(a.tiles,b.tiles);assert_eq!(a.counts.iter().map(|n|3*n).collect::<Vec<_>>(),b.counts);}
 let mut direct=0;let mut root_policy=0;let mut policy_transforms=0;let mut cache_hits=0;let mut tied=0;let mut distinct_keys=0;let mut stats=Vec::new();
 for counted in [false,true] {
  for (i,row) in rows.iter().take(8).enumerate() {
   let fixture:Fixture=serde_json::from_value(row.clone()).unwrap();let base=outer_query(&fixture,&[4,2,2,2],counted,"policy",0).unwrap();
   for level in 0..=2 {
    let spec=ActorSpec {context:base.context.clone(),state:base.state,actor:base.actor,hand:base.hand,level};
    let q=prepare_actor(&spec,Deadline::after(Duration::from_secs(20))).unwrap();
    assert_eq!(q,prepare_actor(&spec,Deadline::after(Duration::from_secs(20))).unwrap());
    let full=service().actors(&[spec.clone()]).unwrap().remove(0);assert_eq!(full,reference(&q,false,20).unwrap().0);
    let mut bounded=bounded_service();let choice=bounded.actor_choices(&[spec.clone()]).unwrap()[0];assert_eq!(choice,full.choice);
    assert_eq!(service().with_native_choices().actor_choices(&[spec.clone()]).unwrap()[0],full.choice);
    let misses=bounded.stats.generated_actor_queries;let before=bounded.stats.cache_hits;
    assert_eq!(bounded.actor_choices(&[spec.clone(),spec.clone()]).unwrap(),vec![full.choice;2]);assert_eq!(misses,bounded.stats.generated_actor_queries);assert_eq!(bounded.stats.cache_hits-before,2);cache_hits+=2;
    if full.counts.iter().filter(|n|**n==full.counts[full.tiles.iter().position(|t|*t==full.choice).unwrap()]).count()>1 {tied+=1;}
    if i==0 {stats.push(json!({"counted":counted,"level":level,"stats":bounded.stats}));}
    direct+=1;
    // Same actor information is the complete input: parent opposing hands and
    // world index are absent from ActorSpec. Scenario order cannot seed actors.
    let mut reordered=base.clone();reordered.worlds.reverse();let privacy=ActorSpec {context:reordered.context,state:reordered.state,actor:reordered.actor,hand:reordered.hand,level};assert_eq!(spec,privacy);assert_eq!(q,prepare_actor(&privacy,Deadline::after(Duration::from_secs(20))).unwrap());
    if i<2 {
      let mut pq=base.clone();pq.nature=Nature::Policy(level);
      let eager=service().root_answers(&[pq.clone()]).unwrap();let bounded=bounded_service().root_answers(&[pq.clone()]).unwrap();assert_eq!(bounded,eager);assert_eq!(bounded[0],reference(&pq,false,20).unwrap().0);root_policy+=1;
      let mut rev=pq.clone();rev.worlds.reverse();assert_eq!(bounded_service().root_answers(&[rev]).unwrap(),bounded);policy_transforms+=1;
      let mut triple=pq.clone();triple.worlds=pq.worlds.iter().flat_map(|w|[*w,*w,*w]).collect();let t=bounded_service().root_answers(&[triple]).unwrap().remove(0);assert_eq!(t.choice,bounded[0].choice);assert_eq!(t.counts,bounded[0].counts.iter().map(|n|n*3).collect::<Vec<_>>());policy_transforms+=1;
      let mut mixed=bounded_service();assert_eq!(mixed.root_answers(&[pq.clone(),pq]).unwrap(),vec![bounded[0].clone();2]);policy_transforms+=2;
    }
    let mut keys=HashSet::new();keys.insert(spec.clone());
    for field in 0..8 {let mut b=spec.clone();match field {0=>b.context.decl=(b.context.decl+1)%7,1=>b.context.bid=if b.context.bid==42 {41}else{b.context.bid+1},2=>b.context.budgets[0]+=1,3=>b.context.boundary^=1,4=>b.context.boundary_size^=1,5=>b.context.counted=!b.context.counted,6=>b.state.voids=if b.state.voids.is_some(){None}else{Some([0;4])},_=>b.level=(level+1)%3};assert!(keys.insert(b));distinct_keys+=1;}
   }
  }
 }
 let mut limited=config(Service::new(Caps {queries:1,seconds:60,..Caps::default()}).unwrap());
 let q=&roots[0];let spec=ActorSpec {context:q.context.clone(),state:q.state,actor:q.actor,hand:q.hand,level:0};
 let answer=limited.actor_choices(&[spec.clone()]).unwrap();let mut other=spec.clone();other.context.budgets[0]+=1;
 assert!(limited.actor_choices(&[other]).is_err());assert_eq!(limited.actor_choices(&[spec.clone()]).unwrap(),answer);assert_eq!(limited.stats.generated_actor_queries,1);
 let mut invalid=spec.clone();invalid.state.plays[invalid.state.len as usize..].fill(27);if invalid.state.len<3 {assert!(bounded_service().actor_choices(&[invalid]).is_err());}
 let mut unsupported=spec;unsupported.level=3;assert!(bounded_service().actor_choices(&[unsupported]).is_err());
 let mut low=config(Service::new(Caps {rows:1,..Caps::default()}).unwrap());assert!(low.root_answers(&roots).is_err());assert_eq!(low.cache_len(),0);
 let mut key_specs=Vec::new();let base=ActorSpec {context:roots[0].context.clone(),state:roots[0].state,actor:roots[0].actor,hand:roots[0].hand,level:0};key_specs.push(base.clone());
 for which in 0..6 {let mut b=base.clone();match which {0=>b.context.budgets[0]+=1,1=>b.context.budgets[1]+=1,2=>b.context.budgets[2]+=1,3=>{b.context.boundary=0;b.context.boundary_size=7;},4=>b.context.decl=(b.context.decl+1)%7,_=>{b.context.counted=true;b.state.voids=Some([0;4]);}}if b!=base {key_specs.push(b);}}
 let mut memo=bounded_service();let mut semantic_keys=0;
 for a in &key_specs {let expected=service().actors(&[a.clone()]).unwrap()[0].choice;let misses=memo.stats.generated_actor_queries;assert_eq!(memo.actor_choices(&[a.clone()]).unwrap()[0],expected);assert_eq!(memo.stats.generated_actor_queries,misses+1);semantic_keys+=1;}
 for a in &key_specs {let misses=memo.stats.generated_actor_queries;assert_eq!(memo.actor_choices(&[a.clone()]).unwrap(),bounded_service().actor_choices(&[a.clone()]).unwrap());assert_eq!(misses,memo.stats.generated_actor_queries);}
 println!("{}",json!({"eager_zero":EAGER.load(std::sync::atomic::Ordering::Relaxed),"exhaustive_interval_certainty_cases":interval_cases,"scalar_roots":scalar,"permutation_and_tripled_multiplicity_vectors":2*scalar,"policy_transformed_vectors":policy_transforms,"direct_actor_full_bounded_native_comparisons":direct,"bounded_full_native_policy_root_comparisons":root_policy,"actual_tied_actor_cases":tied,"exact_choice_cache_hits":cache_hits,"key_distinction_checks":distinct_keys,"semantic_cache_key_miss_then_hit_checks":semantic_keys,"privacy_same_actor_view_sample_checks":direct,"refusal_cache_survival":true,"representative_bounded_stats":stats,"scope":"Finite component checks; sampler equality and privacy input audit, no Rust refinement or asymptotic claim."}));
}

// Fresh arena-specific tests: simultaneous trees have independent public groups,
// mixed complete contexts/teams/rungs, duplicate requests, and arbitrary order.
fn forest() {
 let panel:Value=serde_json::from_str(&fs::read_to_string("experiments/native-frontier-20261004/checks/independent-panel.json").unwrap()).unwrap();
 let wide=||Service::new(Caps {rows:1000000,work:100000000,queries:100000,cache_entries:100000,seconds:60}).unwrap();
 let mut specs=Vec::new();let mut teams=HashSet::new();
 for row in panel["rows"].as_array().unwrap().iter().take(12) {let f:Fixture=serde_json::from_value(row.clone()).unwrap();
  for counted in [false,true] {for level in 0..=2 {let q=outer_query(&f,&[4,2,2,2],counted,"policy",level).unwrap();teams.insert(q.actor%2);let s=ActorSpec {context:q.context,state:q.state,actor:q.actor,hand:q.hand,level};specs.push(s);}}
 }
 assert_eq!(teams.len(),2);let mut full=wide();let expected=full.actors(&specs).unwrap().into_iter().map(|a|a.choice).collect::<Vec<_>>();
 let mut duplicated=Vec::new();let mut dup_expected=Vec::new();for (i,s) in specs.iter().enumerate() {duplicated.push(s.clone());dup_expected.push(expected[i]);if i%3==0 {duplicated.push(s.clone());dup_expected.push(expected[i]);}}
 let mut arena=wide().with_arena_choices().with_choice_trace();assert_eq!(arena.actor_choices(&duplicated).unwrap(),dup_expected);assert!(arena.stats.arena_batch_duplicates>0);assert!(arena.stats.arena_prepared_before_solve>0);
 let generated=arena.stats.generated_actor_queries;let lookups=arena.stats.query_lookups;let hits=arena.stats.cache_hits;assert_eq!(arena.actor_choices(&duplicated).unwrap(),dup_expected);assert_eq!(arena.stats.generated_actor_queries,generated);assert_eq!(arena.stats.query_lookups-lookups,duplicated.len() as u64);assert_eq!(arena.stats.cache_hits-hits,duplicated.len() as u64);
 duplicated.reverse();dup_expected.reverse();assert_eq!(wide().with_arena_choices().actor_choices(&duplicated).unwrap(),dup_expected);
 let trace=arena.take_choice_trace();assert_eq!(trace.len() as u64,arena.stats.actor_demands_by_level.iter().sum::<u64>());let mut unique=trace.iter().map(|x|x.0.clone()).collect::<HashSet<_>>().into_iter().collect::<Vec<_>>();unique.sort_by_key(|s|s.level);
 let mut eager=wide();let mut native=wide().with_native_choices();let mut map=HashMap::new();for level in 0..=2 {let group=unique.iter().filter(|s|s.level==level).cloned().collect::<Vec<_>>();let a=eager.actors(&group).unwrap();let b=native.actor_choices(&group).unwrap();for ((s,v),n) in group.into_iter().zip(a).zip(b) {assert_eq!(v.choice,n);map.insert(s,n);}}
 for(s,v) in &trace {assert_eq!(*v,map[s]);}
 // A prepared forest can refuse before solving. Previously completed choices
 // must survive; uncompleted actor choices must never enter the cache.
 let first=specs.iter().find(|s|s.level==0).unwrap().clone();let mut second=first.clone();second.context.budgets[0]+=1;let mut third=second.clone();third.context.budgets[0]+=1;
 let mut limited=Service::new(Caps {queries:2,seconds:60,..Caps::default()}).unwrap().with_arena_choices();let answer=limited.actor_choices(&[first.clone()]).unwrap();assert!(limited.actor_choices(&[second.clone(),third]).is_err());assert_eq!(limited.stats.generated_actor_queries,2);assert_eq!(limited.stats.completed_actor_queries,1);assert_eq!(limited.actor_choices(&[first]).unwrap(),answer);assert!(limited.actor_choices(&[second]).is_err());
 println!("{}",json!({"fresh_mixed_forest_specs":specs.len(),"mixed_forest_both_teams":teams.len(),"duplicates_and_reversal":true,"every_mixed_forest_trace_call":trace.len(),"distinct_trace_specs":unique.len(),"every_trace_choice_equal_full_and_native":true,"cache_hits_after_complete_forest":duplicated.len(),"prepared_refusal_retains_only_prior_complete_choice":true}));
}

fn extended_trace() {
 let path="experiments/choice-arena-20261004/checks/trace-40.json";
 let rows:Vec<Fixture>=serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
 let wide=||Service::new(Caps {rows:1000000,work:100000000,queries:100000,cache_entries:100000,seconds:60}).unwrap();
 for counted in [false,true] {
  let qs=rows.iter().map(|r|outer_query(r,&[4,2,2,2],counted,"policy",2).unwrap()).collect::<Vec<_>>();
  let mut s=wide().with_arena_choices().with_choice_trace();let answers=s.root_answers(&qs).unwrap();
  for(q,a)in qs.iter().zip(&answers){assert_eq!(*a,reference(q,false,30).unwrap().0);}
  let trace=s.take_choice_trace();assert_eq!(trace.len() as u64,s.stats.actor_demands_by_level.iter().sum::<u64>());
  let mut unique=trace.iter().map(|x|x.0.clone()).collect::<HashSet<_>>().into_iter().collect::<Vec<_>>();unique.sort_by_key(|a|a.level);
  let mut full=wide();let mut native=wide().with_native_choices();let mut expected=HashMap::new();let mut bylevel=Vec::new();
  for level in 0..=2 {let specs=unique.iter().filter(|a|a.level==level).cloned().collect::<Vec<_>>();let a=full.actors(&specs).unwrap();let b=native.actor_choices(&specs).unwrap();for((spec,v),n)in specs.into_iter().zip(a).zip(b){assert_eq!(v.choice,n);expected.insert(spec,n);}bylevel.push(unique.iter().filter(|a|a.level==level).count());}
  for(a,c)in &trace{assert_eq!(*c,expected[a]);}
  let mut rev=qs.clone();for q in &mut rev{q.worlds.reverse();}assert_eq!(wide().with_arena_choices().root_answers(&rev).unwrap(),answers);
  println!("{}",json!({"extended_trace_worlds":40,"counted":counted,"roots":qs.len(),"every_call":trace.len(),"distinct_specs":unique.len(),"by_level":bylevel,"every_choice_full_and_native_equal":true,"complete_vectors_and_reversal":true,"stats":s.stats}));
 }
}
