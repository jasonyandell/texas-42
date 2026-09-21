#![cfg(feature="gpu")]
use std::time::Duration;
use walt::{rules::Decl,solver::SplitMix64};
use walt_response_ladder::{core::{self,Config,PolicyEvaluator},gpu_epochs::EpochEvaluator,mechanics::*,model,
    policy::{self,Policy}};
fn budget()->Budget{Budget::new(20_000_000,Duration::from_secs(30))}
fn fixture(decl:Decl,depth:usize,partial:usize,seed:u64,partner:bool)->(Problem,[usize;4]){
    let mut rng=SplitMix64(seed);let mut deck:Vec<u8>=(0..28).collect();
    for i in (1..28).rev(){let j=rng.below((i+1)as u64)as usize;deck.swap(i,j);}
    let hands:[u32;4]=std::array::from_fn(|s|deck[s*7..s*7+7].iter().fold(0,|m,&t|m|(1<<t)));
    let mut public=PublicState::opening((seed%4)as u8);
    for _ in 0..4*(7-depth)+partial{
        let legal=tiles(public.legal(decl,hands[public.actor()as usize]));
        let tile=legal[rng.below(legal.len()as u64)as usize];public=public.after(decl,tile);
    }
    let viewer=public.actor();let mut levels=[0;4];if partner{levels[(viewer as usize+2)%4]=1;}
    let bid=public.banked_t1+(42-public.banked_t1-public.banked_t0).div_ceil(2);
    let mut p=model::sample_problem(decl,bid,viewer,hands[viewer as usize]&!public.played,&public,4,seed^888,
        model::revision(levels,2,1),&mut budget()).unwrap().unwrap();
    p.scenarios[1].hands=p.scenarios[0].hands;
    for(i,s)in p.scenarios.iter_mut().enumerate(){s.weight=(i+1)as u64;}
    (p,levels)
}
fn pool(p:&Problem)->Vec<Policy>{
    let mut rows=vec![];
    for root in tiles(p.root.legal(p.decl,p.hand)){
        let order=tiles(p.hand&!(1<<root));rows.push(Policy::forced(p,root,order.clone()));
        if order.len()>1{rows.push(Policy::forced(p,root,order.into_iter().rev().collect()));}
    }rows
}
#[test]
fn modeled_epochs_match_full_cpu_traces_and_weighted_prices(){
    let mut gpu=EpochEvaluator::new().unwrap();let mut lanes=0;let mut teams=[false;2];let mut depths=[false;6];
    for(i,&decl)in Decl::ALL.iter().enumerate(){for partial in 0..4{for partner in [false,true]{
        let depth=2+(i+partial)%6;depths[depth-2]=true;
        let(p,levels)=fixture(decl,depth,partial,97500+i as u64*17+partial as u64,partner);teams[(p.viewer%2)as usize]=true;
        let rows=pool(&p);let mut b=budget();let mut field=model::FrozenModel::new(&p,levels,2,1,b.deadline).unwrap();
        let got=gpu.traces(&p,&rows,&mut field,&mut b).unwrap().expect("complete GPU traces");
        let mut reference=model::FrozenModel::new(&p,levels,2,1,budget().deadline).unwrap();
        for (policy,row)in rows.iter().zip(&got){for(scenario,trace)in p.scenarios.iter().zip(row){
            let replay=policy::replay_scenario(&p,scenario,policy,&mut reference,&mut budget()).unwrap().unwrap();
            assert_eq!(trace.payoff,replay.payoff,"{decl:?} depth{depth} partial{partial} partner{partner}");
            assert_eq!(trace.continuation,replay.terminal.history[p.root.history.len()..]);lanes+=1;
        }}
    }}}
    assert!(teams.into_iter().all(|x|x)&&depths.into_iter().all(|x|x));
    assert!(gpu.stats.unique_field_queries>0&&gpu.stats.field_requests>gpu.stats.unique_field_queries);
    assert!(gpu.stats.modeled_unique_queries>0&&gpu.stats.modeled_unique_queries<gpu.stats.unique_field_queries);
    println!("{lanes} modeled full traces matched; stats {:?}",gpu.stats);
}
#[test]
fn materialized_adaptive_policy_and_gpu_seeded_core_reprice_exactly(){
    let mut gpu=EpochEvaluator::new().unwrap();
    let(p,levels)=fixture(Decl::NoTrump,3,1,99117,true);
    let config=Config{max_horizon:7,priority_plans:2,scenario_upper:true,stop_when_certified:false};
    let mut b=budget();let mut field=model::FrozenModel::new(&p,levels,2,1,b.deadline).unwrap();
    let cpu=core::solve(&p,&mut field,&mut b,&config).unwrap();assert!(cpu.exact_vector);
    let policies:Vec<_>=cpu.actions.iter().map(|a|a.policy.clone()).collect();
    let prices=gpu.prices(&p,&policies,&mut field,&mut budget()).unwrap().unwrap();
    assert_eq!(prices,cpu.actions.iter().map(|a|a.lower).collect::<Vec<_>>());
    let mut b=budget();let mut field=model::FrozenModel::new(&p,levels,2,1,b.deadline).unwrap();
    let got=core::solve_with_evaluator(&p,&mut field,&mut b,&config,Some(&mut gpu)).unwrap();
    assert!(got.exact_vector);assert_eq!(got.chosen,cpu.chosen);
    assert_eq!(got.actions.iter().map(|a|(a.action,a.lower,a.upper)).collect::<Vec<_>>(),cpu.actions.iter().map(|a|(a.action,a.lower,a.upper)).collect::<Vec<_>>());
}
struct Abort{revision:String,calls:usize}
impl Field for Abort{fn revision(&self)->&str{&self.revision}fn choose(&mut self,_:FieldQuery<'_>,_:&mut Budget)->Option<u8>{self.calls+=1;None}}
#[test]
fn cancellation_publishes_no_partial_batch_and_persistent_device_recovers(){
    let(p,levels)=fixture(Decl::NoTrump,7,0,90211,true);let rows=pool(&p);let mut gpu=EpochEvaluator::new().unwrap();
    let mut f=Abort{revision:p.field_revision.clone(),calls:0};
    assert!(gpu.prices(&p,&rows,&mut f,&mut Budget::new(0,Duration::ZERO)).unwrap().is_none());
    assert_eq!(f.calls,0);
    assert!(gpu.prices(&p,&rows,&mut f,&mut budget()).unwrap().is_none());assert!(f.calls>0);
    let mut b=budget();let mut field=model::FrozenModel::new(&p,levels,2,1,b.deadline).unwrap();
    assert!(gpu.prices(&p,&rows,&mut field,&mut b).unwrap().is_some());
    assert_eq!(gpu.stats.cancelled_batches,2);assert_eq!(gpu.stats.completed_batches,1);
}

struct OneBatch<'a>{gpu:&'a mut EpochEvaluator,calls:usize}
impl PolicyEvaluator for OneBatch<'_>{
    fn prices(&mut self,p:&Problem,rows:&[Policy],f:&mut dyn Field,b:&mut Budget)->Result<Option<Vec<u64>>,String>{
        self.calls+=1;if self.calls>1{Ok(None)}else{self.gpu.prices(p,rows,f,b)}
    }
}
#[test]
fn completed_gpu_incumbents_survive_later_batch_cancellation(){
    let(p,levels)=fixture(Decl::NoTrump,3,0,99119,true);
    let mut gpu=EpochEvaluator::new().unwrap();let mut b=budget();
    let mut field=model::FrozenModel::new(&p,levels,2,1,b.deadline).unwrap();
    let mut one=OneBatch{gpu:&mut gpu,calls:0};
    let cfg=Config{max_horizon:7,priority_plans:2,scenario_upper:true,stop_when_certified:false};
    let r=core::solve_with_evaluator(&p,&mut field,&mut b,&cfg,Some(&mut one)).unwrap();
    assert!(r.interrupted&&r.incumbent_value.is_some());
    for a in &r.actions{
        assert!(a.priced);
        let value=policy::price(&p,&a.policy,&mut field,&mut budget()).unwrap().unwrap();
        assert_eq!(value,a.lower);
    }
}
#[test]
fn submitted_gpu_epoch_can_cancel_without_poisoning_the_next_batch(){
    let(p,levels)=fixture(Decl::NoTrump,4,0,8132,true);let rows=pool(&p);
    let mut gpu=EpochEvaluator::new().unwrap();let mut b=budget();
    let mut field=model::FrozenModel::new(&p,levels,2,1,b.deadline).unwrap();
    let mut tiny=Budget::new(1,Duration::from_secs(30));
    assert!(gpu.prices(&p,&rows,&mut field,&mut tiny).unwrap().is_none());
    assert_eq!(gpu.stats.epochs,1);
    assert!(gpu.prices(&p,&rows,&mut field,&mut b).unwrap().is_some());
}
struct Audit<'a>{root:&'a PublicState,inner:model::FrozenModel}
impl Field for Audit<'_>{
    fn revision(&self)->&str{self.inner.revision()}
    fn choose(&mut self,q:FieldQuery<'_>,b:&mut Budget)->Option<u8>{
        let mut replay=self.root.clone();
        for &tile in &q.public.history[self.root.history.len()..]{replay=replay.after(q.decl,tile);}
        assert_eq!(&replay,q.public,"GPU field query must carry exact full public information");
        self.inner.choose(q,b)
    }
}
#[test]
fn field_queries_carry_canonical_history_voids_scores_and_remaining_hand(){
    let(p,levels)=fixture(Decl::DoublesTrump,5,2,12312,true);let rows=pool(&p);
    let mut gpu=EpochEvaluator::new().unwrap();let mut b=budget();
    let inner=model::FrozenModel::new(&p,levels,2,1,b.deadline).unwrap();
    let mut f=Audit{root:&p.root,inner};
    assert!(gpu.prices(&p,&rows,&mut f,&mut b).unwrap().is_some());
}

#[test]
fn reachable_materialized_choice_overrides_priority_with_full_trace_parity(){
    let(p,levels)=fixture(Decl::NoTrump,7,0,90211,true);
    let mut field=model::FrozenModel::new(&p,levels,2,1,budget().deadline).unwrap();
    let mut witness=None;
    'rows: for row in pool(&p){for (index,s) in p.scenarios.iter().enumerate(){
        let baseline=policy::replay_scenario(&p,s,&row,&mut field,&mut budget()).unwrap().unwrap();
        for d in &baseline.focal_decisions {
            if d.history.len()==p.root.history.len(){continue;}
            let mut state=p.root.clone();
            for &t in &d.history[p.root.history.len()..]{state=state.after(p.decl,t);}
            let alternatives=state.legal(p.decl,p.hand)&!(1<<d.action);
            if alternatives==0{continue;}
            let mut changed=row.clone();
            changed.decisions.push(policy::Decision{history:d.history.clone(),action:alternatives.trailing_zeros()as u8});
            changed.decisions.sort_by(|a,b|a.history.cmp(&b.history));
            assert_ne!(changed.choose(p.decl,&state,p.hand),row.choose(p.decl,&state,p.hand));
            witness=Some((changed,index,baseline.terminal.history));break 'rows;
        }
    }}
    let (changed,changed_scenario,baseline)=witness.expect("reachable adaptive decision differing from priority");
    let mut gpu=EpochEvaluator::new().unwrap();
    let rows=gpu.traces(&p,&[changed.clone()],&mut field,&mut budget()).unwrap().unwrap();
    assert_ne!(rows[0][changed_scenario].continuation,baseline[p.root.history.len()..]);
    for (scenario,trace) in p.scenarios.iter().zip(&rows[0]){
        let cpu=policy::replay_scenario(&p,scenario,&changed,&mut field,&mut budget()).unwrap().unwrap();
        assert_eq!(trace.payoff,cpu.payoff);
        assert_eq!(trace.continuation,cpu.terminal.history[p.root.history.len()..]);
    }
}
