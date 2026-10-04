use native_frontier::*;
use std::{fs,sync::Arc,time::Duration};
use serde_json::{Value,json};
use walt::{rules::Seat,solver::{Shared,Deadline,Solver,Field}};
fn main() {
    let panel:Value=serde_json::from_str(&fs::read_to_string("experiments/native-frontier-20261004/checks/independent-panel.json").unwrap()).unwrap();
    let rows=panel["rows"].as_array().unwrap();let mut queries=Vec::new();let mut expected=Vec::new();
    for row in rows {
        let fixture:Fixture=serde_json::from_value(row.clone()).unwrap();
        let mut q=outer_query(&fixture,&[4,2,2,2],false,"native",0).unwrap();
        q.nature=Nature::Native(serde_json::from_value(row["seeds"].clone()).unwrap());
        expected.push(row["expected_native_focal_success"].clone());queries.push(q);
    }
    let mut service=Service::new(Caps {seconds:60,..Caps::default()}).unwrap();
    let answers=service.root_answers(&queries).unwrap();
    let mut native_refs=0;
    for (i,(q,a)) in queries.iter().zip(&answers).enumerate() {
        for (tile,count) in a.tiles.iter().zip(&a.counts) {
            let focal=if q.actor%2==1 {*count} else {q.worlds.len() as u64-*count};
            assert_eq!(focal,expected[i][tile.to_string()].as_u64().unwrap(),"scalar root {i} tile {tile}");
        }
        for parallel in [false,true] {let (reference,_,_)=reference(q,parallel,30).unwrap();assert_eq!(a,&reference,"native root {i} parallel {parallel}");native_refs+=1;}
    }
    let hits=service.stats.cache_hits;
    assert_eq!(service.root_answers(&queries).unwrap(),answers);
    assert_eq!(service.stats.cache_hits-hits,queries.len() as u64);
    let mut triples=queries.clone();
    for q in &mut triples {
        q.worlds=q.worlds.iter().flat_map(|w|[*w,*w,*w]).collect();
        if let Nature::Native(seeds)=&mut q.nature {*seeds=seeds.iter().flat_map(|s|[*s,*s,*s]).collect();}
    }
    let tripled=service.root_answers(&triples).unwrap();
    for (a,b) in answers.iter().zip(&tripled) {assert_eq!(a.tiles,b.tiles);assert_eq!(a.choice,b.choice);assert_eq!(a.counts.iter().map(|c|c*3).collect::<Vec<_>>(),b.counts);}
    let mut refusal=Service::new(Caps {rows:1,..Caps::default()}).unwrap();assert!(refusal.root_answers(&queries).is_err());assert_eq!(refusal.cache_len(),0);
    let mut changed=queries.clone();for q in &mut changed {q.context.bid=31;}changed.retain(|q|q.state.terminal(31).is_none());
    let changed_answers=service.root_answers(&changed).unwrap();
    for (q,a) in changed.iter().zip(changed_answers) {assert_eq!(a,reference(q,false,30).unwrap().0);}
    let mut actors_checked=0;
    for counted in [false,true] {
        let mut specs=Vec::new();
        for row in rows.iter().take(6) {
            let fixture:Fixture=serde_json::from_value(row.clone()).unwrap();let q=outer_query(&fixture,&[4,2,2,2],counted,"native",0).unwrap();
            for level in [0,1] {specs.push(ActorSpec {context:q.context.clone(),state:q.state,actor:q.actor,hand:q.hand,level});}
        }
        let actor_answers=service.actors(&specs).unwrap();
        for (spec,a) in specs.iter().zip(actor_answers) {
            let prepared=prepare_actor(spec,Deadline::after(Duration::from_secs(30))).unwrap();
            assert_eq!(a,reference(&prepared,false,30).unwrap().0,"actor vector level {} counted {counted}",spec.level);
            let sh=Arc::new(Shared::new(walt::solver::decl_of(spec.context.decl),spec.context.bid,spec.context.budgets.clone(),spec.context.boundary,spec.context.boundary_size,Deadline::after(Duration::from_secs(30))).with_inner_belief(spec.context.belief()));
            let seat=Seat::from_index(spec.actor as usize).unwrap();
            let host=Solver::new(sh,seat,spec.hand,spec.actor%2==1,Vec::new(),Vec::new(),Field::Level(spec.level));
            let mask=a.tiles.iter().fold(0u32,|m,&t|m|(1<<t));
            assert_eq!(host.modeled_choice(spec.level,&spec.state.key(),seat,spec.hand,mask).unwrap(),a.choice,"actor choice level {} counted {counted}",spec.level);
            actors_checked+=1;
        }
    }
    let mut hybrid_vectors=0;let mut hybrid_native_refs=0;
    for counted in [false,true] {for field in [0,1] {
        let qs=rows.iter().take(12).map(|row| {let f:Fixture=serde_json::from_value(row.clone()).unwrap();outer_query(&f,&[4,2,2,2],counted,"policy",field).unwrap()}).collect::<Vec<_>>();
        let mut full=Service::new(Caps {seconds:60,..Caps::default()}).unwrap();let expected=full.root_answers(&qs).unwrap();
        let mut hybrid=Service::new(Caps {seconds:60,..Caps::default()}).unwrap().with_native_choices();let got=hybrid.root_answers(&qs).unwrap();assert_eq!(got,expected,"hybrid counted {counted} field {field}");
        for (q,a) in qs.iter().zip(&got) {for parallel in [false,true] {assert_eq!(a,&reference(q,parallel,30).unwrap().0);hybrid_native_refs+=1;}}
        assert_eq!(hybrid.root_answers(&qs).unwrap(),got);hybrid_vectors+=got.len();
        let a=ActorSpec {context:qs[0].context.clone(),state:qs[0].state,actor:qs[0].actor,hand:qs[0].hand,level:1};let complete=hybrid.actors(&[a.clone()]).unwrap();assert!(!complete[0].counts.is_empty());assert_eq!(complete[0],reference(&prepare_actor(&a,Deadline::after(Duration::from_secs(30))).unwrap(),false,30).unwrap().0);
        let mut capped=Service::new(Caps {cache_entries:1,seconds:60,..Caps::default()}).unwrap().with_native_choices();assert_eq!(capped.root_answers(&qs[..2]).unwrap(),got[..2]);assert_eq!(capped.root_answers(&qs[..2]).unwrap(),got[..2]);
    }}
    let base=ActorSpec {context:queries[0].context.clone(),state:queries[0].state,actor:queries[0].actor,hand:queries[0].hand,level:0};
    let mut identities=std::collections::HashSet::new();assert!(identities.insert(base.clone()));
    for field in 0..18 {let mut b=base.clone();match field {
        0=>b.context.decl=9,1=>b.context.bid=31,2=>b.context.budgets[0]+=1,3=>b.context.boundary^=1,4=>b.context.boundary_size+=1,5=>b.context.counted=true,
        6=>b.state.played^=1,7=>b.state.leader=(b.state.leader+1)%4,8=>b.state.len=(b.state.len+1)%4,9=>b.state.plays[0]=(b.state.plays[0]+1)%28,
        10=>b.state.plays[1]=(b.state.plays[1]+1)%28,11=>b.state.plays[2]=(b.state.plays[2]+1)%28,12=>b.state.t1+=1,13=>b.state.t0+=1,
        14=>b.state.voids=Some([0;4]),15=>b.hand^=1,16=>b.level=1,_=>b.actor=(b.actor+1)%4};assert!(identities.insert(b),"actor identity field {field}");}
    let mut root_ids=std::collections::HashSet::new();assert!(root_ids.insert(queries[0].clone()));
    for field in 0..5 {let mut q=queries[0].clone();match field {0=>q.worlds[0][0]^=1,1=>q.worlds.push(q.worlds[0]),2=>{if let Nature::Native(seeds)=&mut q.nature {seeds[0]^=1;}},3=>q.nature=Nature::Policy(0),_=>q.nature=Nature::Tape(vec![vec![0;12];8])};assert!(root_ids.insert(q));}
    let mut invalids=Vec::new();
    let mut a=base.clone();a.actor=4;invalids.push(("actor4",a));
    let mut a=base.clone();a.context.boundary_size=usize::MAX;invalids.push(("boundary-size-overflow",a));
    let mut a=base.clone();a.level=usize::MAX;invalids.push(("rung-overflow",a));
    let mut a=base.clone();a.state.len=4;invalids.push(("trick-length4",a));
    let mut a=base.clone();a.state.leader=4;invalids.push(("leader4",a));
    let mut a=base.clone();a.context.counted=true;let mut v=[0;4];v[(a.actor as usize+1)%4]=1;a.state.voids=Some(v);invalids.push(("non-context-void-mask",a));
    let mut a=base.clone();a.context.counted=true;let mut v=[walt::solver::FULL_MASK;4];v[a.actor as usize]=0;a.state.voids=Some(v);invalids.push(("empty-counted-fiber",a));
    let mut a=base.clone();a.state.played|=1<<28;invalids.push(("played-high-bit",a));
    let mut a=base.clone();a.state.t1=42;invalids.push(("invalid-settled-score",a));
    let mut direct_guards=Vec::new();
    for (name,a) in invalids {let outcome=std::panic::catch_unwind(|| {let mut srv=Service::new(Caps::default()).unwrap();srv.actors(&[a])});assert!(outcome.is_ok(),"direct guard panicked: {name}");assert!(outcome.unwrap().is_err(),"direct guard accepted: {name}");direct_guards.push(name);}
    assert!(prepare_actor(&base,Deadline::after(Duration::ZERO)).is_err());
    let mut invalid_tape=queries[0].clone();invalid_tape.nature=Nature::Tape(vec![]);let mut invalid=Service::new(Caps::default()).unwrap();assert!(invalid.root_answers(&[invalid_tape]).is_err());assert_eq!(invalid.cache_len(),0);assert_eq!(invalid.stats.refused_batches,1);
    let mut expired=Service::new(Caps {seconds:1,..Caps::default()}).unwrap();expired.root_answers(&queries[..1]).unwrap();let completed_cache=expired.cache_len();std::thread::sleep(Duration::from_millis(1100));assert!(expired.root_answers(&queries[..1]).is_err());assert_eq!(expired.cache_len(),completed_cache);assert_eq!(expired.stats.refused_batches,1);
    println!("{}",json!({"fresh_scalar_roots":answers.len(),"native_serial_parallel_vectors":native_refs,"tripled_weight_vectors":tripled.len(),"bid31_reference_vectors":changed.len(),"actor_vectors_and_pinned_choices":actors_checked,"actor_levels":[0,1],"hybrid_full_frontier_vectors":hybrid_vectors,"hybrid_native_serial_parallel_vectors":hybrid_native_refs,"hybrid_field_levels":[0,1],"hybrid_full_actor_answers_preserved":true,"hybrid_cache_cap1_recompute_parity":true,"belief_modes":["Voidless","VoidsCounted"],"initial_row_refusal_uncached":true,"direct_actor_guards":direct_guards,"actor_identity_variants":identities.len(),"root_identity_variants":root_ids.len(),"expired_completed_cache_refuses":true,"expired_sampling_refuses":true,"direct_tape_dimensions_refuse":true,"warm_root_cache_hits":queries.len(),"stats":service.stats}));
}
