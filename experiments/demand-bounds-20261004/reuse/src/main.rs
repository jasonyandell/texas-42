use native_frontier::*;
use serde_json::{json,Value};
use std::{collections::HashSet,fs,time::Instant};
fn service()->Service {Service::new(Caps {seconds:30,work:20_000_000,..Caps::default()}).unwrap().with_bounded_choices()}
fn main() {
    let path=std::env::args().nth(1).expect("local fixture path");
    let input:Value=serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
    let fixtures=input["rows"].as_array().unwrap().iter().map(|r|serde_json::from_value::<Fixture>(r.clone()).unwrap()).collect::<Vec<_>>();
    let mut records=Vec::new();let mut vectors=0;
    for counted in [false,true] {for level in 0..=2 {
        let queries=fixtures.iter().map(|r|outer_query(r,&[4,2,2,2],counted,"policy",level).unwrap()).collect::<Vec<_>>();
        assert_eq!(queries.iter().collect::<HashSet<_>>().len(),queries.len());
        let mut warm=service();let mut independent_misses=0;let mut independent_inner_worlds=0u64;let mut warm_total_us=0;let mut cold_total_us=0;let mut root_records=Vec::new();
        for (q,row) in queries.iter().zip(&fixtures) {
            let mut cold=service();let t=Instant::now();let expected=cold.root_answers(&[q.clone()]).unwrap();cold_total_us+=t.elapsed().as_micros();
            assert_eq!(expected[0],reference(q,false,20).unwrap().0);
            let before=(warm.stats.generated_actor_queries,warm.stats.cache_hits,warm.stats.query_lookups,warm.cache_len(),warm.stats.inner_worlds_by_level.iter().sum::<u64>());
            let t=Instant::now();let got=warm.root_answers(&[q.clone()]).unwrap();warm_total_us+=t.elapsed().as_micros();assert_eq!(got,expected);vectors+=1;
            independent_misses+=cold.stats.generated_actor_queries;
            independent_inner_worlds+=cold.stats.inner_worlds_by_level.iter().sum::<u64>();
            root_records.push(json!({"seed":row.seed,"public_played":q.state.played,"boundary":q.context.boundary,"boundary_size":q.context.boundary_size,"actor":q.actor,"cold_misses":cold.stats.generated_actor_queries,"reused_misses":warm.stats.generated_actor_queries-before.0,"cold_inner_worlds":cold.stats.inner_worlds_by_level.iter().sum::<u64>(),"reused_inner_worlds":warm.stats.inner_worlds_by_level.iter().sum::<u64>()-before.4,"reused_hits":warm.stats.cache_hits-before.1,"lookups":warm.stats.query_lookups-before.2,"new_completed_cache_entries":warm.cache_len()-before.3}));
        }
        assert!(warm.stats.generated_actor_queries<=independent_misses);
        records.push(json!({"counted":counted,"field":level,"roots":queries.len(),"independent_cold_misses":independent_misses,"successive_service_misses":warm.stats.generated_actor_queries,"actor_queries_avoided_by_cross_root_cache":independent_misses-warm.stats.generated_actor_queries,"independent_inner_worlds":independent_inner_worlds,"successive_inner_worlds":warm.stats.inner_worlds_by_level.iter().sum::<u64>(),"inner_worlds_avoided":independent_inner_worlds-warm.stats.inner_worlds_by_level.iter().sum::<u64>(),"cold_service_sum_us":cold_total_us,"successive_service_sum_us":warm_total_us,"final_stats":warm.stats,"roots_detail":root_records}));
    }}
    println!("{}",json!({"vector_checks":vectors,"records":records,"scope":"Distinct successive public positions from same fixed local deals, all full root vectors equal to independent cold and pinned native. Exact actor cache reuse is cold-miss difference, not raw cache hit count. One observed sequence; cache is retained throughout sequence. Timings illustrative interleaved, no speed inference. Sampling/serialization profile attached separately."}));
}
