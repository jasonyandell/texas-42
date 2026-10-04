use native_frontier::*;
use serde_json::{Value,json};
use std::fs;
fn service()->Service {Service::new(Caps {seconds:60,..Caps::default()}).unwrap()}
fn main() {
 let panel:Value=serde_json::from_str(&fs::read_to_string("experiments/native-frontier-20261004/checks/independent-panel.json").unwrap()).unwrap();
 let rows=panel["rows"].as_array().unwrap();let mut qs=Vec::new();let mut scalar_vectors=0;let mut independent_row_edges=0;let mut independent_public_coordinates=0;
 for row in rows {
  let fixture:Fixture=serde_json::from_value(row.clone()).unwrap();
  let q=outer_query(&fixture,&[4,2,2,2],false,"native",0).unwrap();
  let mut single=service();let a=single.root_answers(&[q.clone()]).unwrap().remove(0);independent_row_edges+=single.stats.row_edges;independent_public_coordinates+=single.stats.public_coordinates;
  for (&tile,&count) in a.tiles.iter().zip(&a.counts) {let focal=if q.actor%2==1 {count}else{q.worlds.len() as u64-count};assert_eq!(focal,row["expected_native_focal_success"][tile.to_string()].as_u64().unwrap());}
  scalar_vectors+=1;qs.push(q);
 }
 let independently_counted:Value=serde_json::from_str(&fs::read_to_string("experiments/prefix-state-20261004/checks/prefix-counts/stdout.log").unwrap()).unwrap();
 assert_eq!(independent_row_edges,independently_counted["row_edges"].as_u64().unwrap());assert_eq!(independent_public_coordinates,independently_counted["public_coordinates"].as_u64().unwrap());
 let baseline=service().root_answers(&qs).unwrap();let mut mutations=0;
 // Scenario permutation preserves seed/world pairing and all scenario multiplicities.
 let mut reversed=qs.clone();for q in &mut reversed {q.worlds.reverse();if let Nature::Native(s)=&mut q.nature{s.reverse();}}
 assert_eq!(service().root_answers(&reversed).unwrap(),baseline);mutations+=qs.len();
 // Duplicate weights must multiply integer counts; this catches dropped repeated worlds.
 let mut triples=qs.clone();for q in &mut triples {q.worlds=q.worlds.iter().flat_map(|w|[*w,*w,*w]).collect();if let Nature::Native(s)=&mut q.nature{*s=s.iter().flat_map(|v|[*v,*v,*v]).collect();}}
 let got=service().root_answers(&triples).unwrap();for(a,b)in baseline.iter().zip(got){assert_eq!(a.tiles,b.tiles);assert_eq!(a.choice,b.choice);assert_eq!(a.counts.iter().map(|c|3*c).collect::<Vec<_>>(),b.counts);}mutations+=qs.len();
 // Different root order and duplicate query keys must not fuse public-coordinate IDs.
 let mut mixed=qs.clone();mixed.reverse();mixed.extend(qs.clone());let mut expected=baseline.clone();expected.reverse();expected.extend(baseline.clone());assert_eq!(service().root_answers(&mixed).unwrap(),expected);mutations+=mixed.len();
 // Refusal keeps completed cache valid and publishes no partial root vector.
 let mut capped=Service::new(Caps {rows:1,..Caps::default()}).unwrap();assert!(capped.root_answers(&qs).is_err());assert_eq!(capped.cache_len(),0);
 let mut invalid=qs[0].clone();invalid.state.plays[invalid.state.len as usize..].fill(27);if invalid.state.len<3 {assert!(service().root_answers(&[invalid]).is_err());}
 let mut interner_comparisons=0;
 for input in [&qs,&reversed,&triples,&mixed] {let mut linked=service();let mut hashed=service().with_hashed_prefixes();assert_eq!(linked.root_answers(input).unwrap(),hashed.root_answers(input).unwrap());assert_eq!(linked.stats.row_edges,hashed.stats.row_edges);assert_eq!(linked.stats.public_coordinates,hashed.stats.public_coordinates);assert_eq!(linked.stats.public_step_calls,hashed.stats.public_step_calls);assert_eq!(linked.stats.public_record_calls,hashed.stats.public_record_calls);interner_comparisons+=input.len();}
 let mut reference_vectors=0;let mut policy_vectors=0;let mut metrics=Vec::new();
 for counted in [false,true] {for field in [0,1] {
  let input=rows.iter().take(8).map(|row| {let f:Fixture=serde_json::from_value(row.clone()).unwrap();outer_query(&f,&[4,2,2,2],counted,"policy",field).unwrap()}).collect::<Vec<_>>();
  let mut full=service();let a=full.root_answers(&input).unwrap();let mut hybrid=service().with_native_choices();let b=hybrid.root_answers(&input).unwrap();assert_eq!(a,b);
  let mut hashed_full=service().with_hashed_prefixes();let mut hashed_hybrid=service().with_hashed_prefixes().with_native_choices();assert_eq!(a,hashed_full.root_answers(&input).unwrap());assert_eq!(b,hashed_hybrid.root_answers(&input).unwrap());assert_eq!(full.stats.row_edges,hashed_full.stats.row_edges);assert_eq!(full.stats.public_coordinates,hashed_full.stats.public_coordinates);assert_eq!(full.stats.actor_demands_by_level,hashed_full.stats.actor_demands_by_level);assert_eq!(full.stats.unique_actor_misses_by_level,hashed_full.stats.unique_actor_misses_by_level);interner_comparisons+=2*input.len();
  for(q,v)in input.iter().zip(&a){assert_eq!(*v,reference(q,false,30).unwrap().0);reference_vectors+=1;}
  policy_vectors+=a.len();metrics.push(json!({"counted":counted,"field":field,"stats":full.stats}));
 }}
 let mut cache=service();assert_eq!(cache.root_answers(&qs).unwrap(),baseline);let hits=cache.stats.cache_hits;assert_eq!(cache.root_answers(&qs).unwrap(),baseline);assert_eq!(cache.stats.cache_hits-hits,qs.len() as u64);
 println!("{}",json!({"independently_counted_row_edges":independent_row_edges,"independently_counted_public_coordinates":independent_public_coordinates,"hashed_linked_vector_comparisons":interner_comparisons,"scalar_vectors":scalar_vectors,"transformed_query_vectors":mutations,"policy_vectors":policy_vectors,"native_reference_vectors":reference_vectors,"policy_stats":metrics,"note":"Exploratory finite-component tests, not phone or playing-strength evidence."}));
}
