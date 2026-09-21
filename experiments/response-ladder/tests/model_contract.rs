use std::{sync::Arc,time::Duration};
use walt::{rules::{Decl,Seat},solver::{self,Shared,Solver,SplitMix64}};
use walt_response_ladder::{core::{self,Config},mechanics::*,model};
use num_traits::ToPrimitive;

fn root(decl:Decl, seed:u64)->(PublicState,[u32;4]) {
    let mut rng=SplitMix64(seed);
    let mut deck:Vec<u8>=(0..28).collect();
    for i in (1..28).rev() { let j=rng.below((i+1)as u64)as usize;deck.swap(i,j); }
    let hands:[u32;4]=std::array::from_fn(|s|deck[s*7..s*7+7].iter().fold(0,|m,&t|m|(1<<t)));
    let mut state=PublicState::opening((seed%4)as u8);
    for _ in 0..20 {
        let legal=tiles(state.legal(decl,hands[state.actor()as usize]));
        let tile=legal[rng.below(legal.len()as u64)as usize];
        state=state.after(decl,tile);
    }
    { let remaining=std::array::from_fn(|s|hands[s]&!state.played); (state,remaining) }
}
#[test]
fn outer_stream_matches_historical_sampler_all_declarations() {
    for (i,&decl) in Decl::ALL.iter().enumerate() {
        let (state,hands)=root(decl,700+i as u64);
        let actor=state.actor(); let seed=12344+i as u64;
        let mut budget=Budget::new(1_000_000,Duration::from_secs(10));
        let problem=model::sample_problem(decl,30,actor,hands[actor as usize],&state,8,seed,
            "historical-dice-v1".into(),&mut budget).unwrap().unwrap();
        let (_,_,sizes)=model::frame(&state).unwrap();
        let mut rng=SplitMix64(seed);
        let worlds=solver::sample_belief(actor as usize,hands[actor as usize],state.played,sizes,state.voids,8,&mut rng).unwrap();
        assert_eq!(problem.scenarios.iter().map(|s|s.hands).collect::<Vec<_>>(),worlds);
        let tapes:Vec<u64>=(0..8).map(|_|rng.next_u64()).collect();
        assert_eq!(problem.scenarios.iter().map(|s|s.tape).collect::<Vec<_>>(),tapes);
    }
}
#[test]
fn completed_ladder_matches_frozen_modeled_fields() {
    let mut nontrivial=0;
    for (i,&decl) in Decl::ALL.iter().enumerate() {
        for partner in [false,true] {
            let (state,hands)=root(decl,880+i as u64);
            let actor=state.actor();
            let bid=state.banked_t1+(42-state.banked_t1-state.banked_t0).div_ceil(2);
            let mut levels=[0;4]; if partner {levels[(actor as usize+2)%4]=1;}
            let identity=model::revision(levels,2,1);
            let mut budget=Budget::new(10_000_000,Duration::from_secs(30));
            let p=model::sample_problem(decl,bid,actor,hands[actor as usize],&state,3,9137,
                identity,&mut budget).unwrap().unwrap();
            let mut field=model::FrozenModel::new(&p,levels,2,1,budget.deadline).unwrap();
            let report=core::solve(&p,&mut field,&mut budget,&Config {max_horizon:7,priority_plans:2,scenario_upper:true,stop_when_certified:false}).unwrap();
            assert!(report.exact_vector,"{decl:?} {partner}: {report:?}");
            let (boundary,size,_)=model::frame(&state).unwrap();
            let sh=Arc::new(Shared::new(decl,bid,vec![2,1],boundary,size,budget.deadline));
            let sol=Solver::new(sh,Seat::from_index(actor as usize).unwrap(),p.hand,actor%2==1,
                p.scenarios.iter().map(|s|s.hands).collect(),p.scenarios.iter().map(|s|s.tape).collect(),solver::Field::SeatLevels(levels));
            let values=sol.action_values(&state.key(),&tiles(state.legal(decl,p.hand))).unwrap();
            for ((a,v),got) in values.iter().zip(&report.actions) {
                let t1=(v*num_rational::BigRational::from_integer(3.into())).to_integer().to_u64().unwrap();
                let expected=if actor%2==1{t1}else{3-t1};
                assert_eq!((*a,expected),(got.action,got.lower)); assert_eq!(got.lower,got.upper);
            }
            assert_eq!(report.chosen,solver::best_of(&values,actor%2==1));
            if values.len()>1 {nontrivial+=1;}
        }
    }
    assert!(nontrivial>=4);
}
