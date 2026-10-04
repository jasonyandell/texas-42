// New finite check: completed trees survive a later refusal in the same forest.
use native_frontier::*;
use serde_json::{Value,json};
use std::{fs,collections::HashSet};
fn main() {
 let panel:Value=serde_json::from_str(&fs::read_to_string("experiments/native-frontier-20261004/checks/independent-panel.json").unwrap()).unwrap();
 let mut specs=Vec::new();let mut keys=HashSet::new();for row in panel["rows"].as_array().unwrap() {let f:Fixture=serde_json::from_value(row.clone()).unwrap();let q=outer_query(&f,&[4,2,2,2],false,"policy",0).unwrap();let spec=ActorSpec {context:q.context,state:q.state,actor:q.actor,hand:q.hand,level:0};if keys.insert(spec.clone()){specs.push(spec);}}
 let full=Service::new(Caps {work:100000000,seconds:60,..Caps::default()}).unwrap().actors(&specs).unwrap();
 let mut observations=Vec::new();let mut surviving=0;
 for cap in [1000,2000,4000,8000,16000] {let mut s=Service::new(Caps {work:cap,seconds:60,..Caps::default()}).unwrap().with_resumable_choices();let result=s.actor_choices(&specs);if result.is_ok(){continue;}let completed=s.stats.completed_actor_queries;let cache=s.cache_len();assert_eq!(cache,completed as usize);let mut warm=0;
 for(spec,answer)in specs.iter().zip(&full) {let misses=s.stats.generated_actor_queries;if let Ok(v)=s.actor_choices(&[spec.clone()]){assert_eq!(v,vec![answer.choice]);if misses==s.stats.generated_actor_queries {warm+=1;}}}
 assert_eq!(warm,cache);if warm>0{surviving+=1;}observations.push(json!({"work_cap":cap,"forest_refused":true,"completed_choices_before_refusal":completed,"every_cached_choice_retained_and_full_equal":warm}));
 }
 assert!(surviving>0,"no partial-completion refusal witness observed");println!("{}",json!({"fresh_failed_forest_complete_cache_survival":true,"independent_actor_views":specs.len(),"refusal_witnesses":observations,"scope":"Finite post-completion work refusal, no rollback claim. Cached results retain exact full-eager choices while incomplete forest refuses."}));
}
