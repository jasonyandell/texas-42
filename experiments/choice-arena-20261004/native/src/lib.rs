//! Experimental finite-bundle service. No production source changes.
//! Full public-prefix IDs, shared choice after integer scenario aggregation.
use std::{collections::{HashMap,BTreeMap}, mem::size_of, sync::Arc, time::{Duration, Instant}};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};
use walt::{rules::{Domino, Seat}, solver::{self, Contract, Deadline, Field, InnerBelief, Key, Shared, Solver, SplitMix64}};

const FULL: u32 = solver::FULL_MASK;
const STOP: u8 = 28;
mod bounds;
mod arena;
pub use bounds::certified_index;
pub type Res<T> = Result<T, String>;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct State {
    pub played: u32, pub leader: u8, pub plays: [u8; 3], pub len: u8,
    pub t1: u8, pub t0: u8, pub voids: Option<[u32; 4]>,
}
impl State {
    pub fn key(self) -> Key { Key { played:self.played,leader:self.leader,plays:self.plays[..self.len as usize].to_vec(),banked_t1:self.t1,banked_t0:self.t0,voids:self.voids,alive:0 } }
    pub fn actor(self) -> usize { (self.leader as usize+self.len as usize)%4 }
    pub fn terminal(self, bid:u8) -> Option<u64> { if self.t1>=bid {Some(1)} else if self.t0>42-bid {Some(0)} else {None} }
    pub fn from_key(k:&Key) -> Self { let mut p=[0;3];p[..k.plays.len()].copy_from_slice(&k.plays); Self {played:k.played,leader:k.leader,plays:p,len:k.plays.len() as u8,t1:k.banked_t1,t0:k.banked_t0,voids:k.voids} }
}

/// Immutable specialization of the native rule algebra, 28x28 trick keys.
pub struct Tables { incidence:[u32;28], rank:[[u8;28];28], count:[u8;28] }
impl Tables {
    pub fn new(decl:usize) -> Self {
        let d=solver::decl_of(decl); let mut out=Self {incidence:[0;28],rank:[[0;28];28],count:[0;28]};
        for lead in 0..28 { let q=d.led_context(Domino::from_index(lead).unwrap());
            out.incidence[lead]=solver::mask_of(d.effective_incidence(q));
            for tile in 0..28 {let k=d.trick_key(Domino::from_index(tile).unwrap(),q);out.rank[lead][tile]=(k.tier as u8)*16+k.rank.value();}
            out.count[lead]=Domino::from_index(lead).unwrap().count() as u8;
        } out
    }
    pub fn legal(&self, s:State, hand:u32) -> u32 { if s.len==0 {hand} else {let follow=hand & self.incidence[s.plays[0] as usize];if follow==0 {hand} else {follow}} }
    pub fn step(&self, mut s:State, tile:u8) -> State {
        let actor=s.actor();
        if let Some(mut v)=s.voids { if s.len>0 && (1u32<<tile)&self.incidence[s.plays[0] as usize]==0 {v[actor]|=self.incidence[s.plays[0] as usize];}s.voids=Some(v); }
        s.played |= 1u32<<tile;
        if s.len==3 {
            let p=[s.plays[0],s.plays[1],s.plays[2],tile];let mut best=0;
            for i in 1..4 {if self.rank[p[0] as usize][p[i] as usize]>self.rank[p[0] as usize][p[best] as usize] {best=i;}}
            let winner=(s.leader as usize+best)%4;let pts=1+p.iter().map(|&x|self.count[x as usize]).sum::<u8>();
            if winner%2==1 {s.t1+=pts;} else {s.t0+=pts;}s.leader=winner as u8;s.plays=[0;3];s.len=0;
        } else {s.plays[s.len as usize]=tile;s.len+=1;}s
    }
}
fn nth(mut mask:u32, index:u32) -> u8 { for _ in 0..index {mask &= mask-1;}mask.trailing_zeros() as u8 }
fn record(s:State)->u64 {let mut h=solver::mix(s.played as u64);h=solver::mix(h^((s.leader as u64)<<32));for &p in &s.plays[..s.len as usize] {h=solver::mix(h^(0x100|p as u64));}h}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Context { pub decl:usize, pub bid:u8, pub budgets:Vec<usize>, pub boundary:u32, pub boundary_size:usize, pub counted:bool }
impl Context {
    pub fn belief(&self) -> InnerBelief {if self.counted {InnerBelief::VoidsCounted} else {InnerBelief::Voidless}}
    pub fn sizes(&self, s:State) -> [usize;4] { Contract::Straight {bid:self.bid}.sizes(&s.key(),self.boundary,self.boundary_size) }
}
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Nature { Tape(Vec<Vec<u64>>), Native(Vec<u64>), Policy(usize) }
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Query { pub context:Context, pub state:State, pub actor:u8, pub hand:u32, pub worlds:Vec<[u32;4]>, pub nature:Nature }
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ActorSpec { pub context:Context, pub state:State, pub actor:u8, pub hand:u32, pub level:usize }
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct Answer { pub tiles:Vec<u8>, pub counts:Vec<u64>, pub choice:u8 }
impl Answer {
    fn new(tiles:Vec<u8>, counts:Vec<u64>, maximize:bool) -> Self {let mut b=0;for i in 1..tiles.len() {if if maximize {counts[i]>counts[b]} else {counts[i]<counts[b]} {b=i;}}let choice=tiles[b];Self {tiles,counts,choice}}
}

pub fn prepare_actor(a:&ActorSpec, deadline:Deadline) -> Res<Query> {
    let c=&a.context;validate_frame(c,a.state,a.actor,a.hand)?;if a.level>2 || a.level>=c.budgets.len() {return Err("unsupported actor rung".into());}let level_tag=if a.level==0 {0} else {solver::mix(0x4C32 ^ a.level as u64)};
    let mut rng=SplitMix64(solver::INNER_SEED ^ level_tag ^ solver::mix(a.actor as u64) ^ solver::mix(a.hand as u64) ^ solver::record_hash(&a.state.key()));
    let n=*c.budgets.get(a.level).ok_or("undeclared modeled budget")?;
    let worlds=c.belief().sample(solver::decl_of(c.decl),Seat::from_index(a.actor as usize).unwrap(),a.hand,&a.state.key(),c.sizes(a.state),n,&mut rng,deadline).ok_or("actor sampling deadline")?;
    let nature=if a.level==0 {Nature::Native((0..n).map(|_|rng.next_u64()).collect())} else {Nature::Policy(a.level-1)};
    Ok(Query {context:c.clone(),state:a.state,actor:a.actor,hand:a.hand,worlds,nature})
}

#[derive(Clone, Deserialize)]
pub struct Request {pub decl:usize,pub bid:u8,pub bidder:usize,pub seat:usize,pub hand:Vec<usize>,pub plays:Vec<usize>}
#[derive(Clone, Deserialize)]
pub struct Fixture {pub seed:u64,pub request:Request,pub worlds:Vec<[u32;4]>,pub tape:Vec<Vec<u64>>,#[serde(default)]pub seeds:Option<Vec<u64>>}

/// Independently replay every reconstructed full outer deal, without terminal
/// pruning. No reliance on the archived sampler or solver::replay assertions.
pub fn outer_query(row:&Fixture, budgets:&[usize], counted:bool, mode:&str, field:usize) -> Res<Query> {
    let r=&row.request;
    if r.decl>7 && r.decl!=9 || !(1..=42).contains(&r.bid) || r.bidder>3 || r.seat>3 || r.plays.len()%2!=0 || r.plays.len()>48 || r.plays.len()<24 || r.hand.len()!=7 || row.worlds.is_empty() || row.worlds.len()>1024 {return Err("unsupported outer frame".into());}
    let rot=if r.bidder%2==0 {1} else {0};let actor=((r.seat+rot)%4) as u8;
    let mut past=[0u32;4];let mut played=0;let mut pairs=Vec::new();
    for p in r.plays.chunks(2) {if p[0]>3 || p[1]>27 || played&(1<<p[1])!=0 {return Err("invalid exposed play".into());}played |= 1<<p[1];past[p[0]]|=1<<p[1];pairs.push((p[0],p[1]));}
    let mut original=0u32;for &t in &r.hand {if t>27 || original&(1<<t)!=0 {return Err("invalid original hand".into());}original|=1<<t;}
    if original & played != past[r.seat] {return Err("original hand disagrees with history".into());}
    let mut worlds=Vec::new();let table=Tables::new(r.decl);let mut final_state=None;
    for &remaining in &row.worlds {
        let mut full=[0;4];let mut union=0;
        for s in 0..4 { if remaining[s]&!FULL!=0 || remaining[s]&played!=0 {return Err("remaining hand outside pool".into());}
            full[s]=remaining[s]|past[s];if full[s].count_ones()!=7 || full[s]&union!=0 {return Err("full deal capacity/overlap".into());}union|=full[s]; }
        if union!=FULL || full[r.seat]!=original {return Err("deal conservation/own hand".into());}
        let mut state=State {played:0,leader:((r.bidder+rot)%4) as u8,plays:[0;3],len:0,t1:0,t0:0,voids:Some([0;4])};
        let mut hands=[0;4];for s in 0..4 {hands[(s+rot)%4]=full[s];}
        for &(s,t) in &pairs {let s=(s+rot)%4;if state.actor()!=s || table.legal(state,hands[s])&(1<<t)==0 {return Err("outer world fails complete historical legality replay".into());}hands[s]&=!(1<<t);state=table.step(state,t as u8);}
        if let Some(prior)=final_state {if prior!=state {return Err("public state divergence".into());}}final_state=Some(state);worlds.push(hands);
    }
    let mut state=final_state.unwrap();if state.actor()!=actor as usize || state.terminal(r.bid).is_some() {return Err("actor mismatch/settled outer root".into());}
    let st=solver::replay(solver::decl_of(r.decl),r.bidder,&pairs);
    let public=State::from_key(&Key {voids:Some(st.voids),played:st.played,leader:st.leader,plays:st.plays,banked_t1:st.banked_t1,banked_t0:st.banked_t0,alive:0});
    if public!=state {return Err("native public replay mismatch".into());}
    if !counted {state.voids=None;}
    let context=Context {decl:r.decl,bid:r.bid,budgets:budgets.to_vec(),boundary:st.trick_start_played,boundary_size:7-st.completed,counted};
    let nature=match mode {"tape"=>{if row.tape.len()!=worlds.len() || row.tape.iter().any(|t|t.len()!=28-played.count_ones() as usize) {return Err("tape dimensions".into());}Nature::Tape(row.tape.clone())},"native"=>{let s=row.seeds.clone().unwrap_or_else(||(0..worlds.len()).map(|sid|solver::mix(row.seed ^ sid as u64 ^ 77000000)).collect());if s.len()!=worlds.len() {return Err("native seed dimensions".into());}Nature::Native(s)},"policy"=>Nature::Policy(field),_=>return Err("unknown nature mode".into())};
    Ok(Query {context,state,actor,hand:original&!played,worlds,nature})
}

#[derive(Clone, Deserialize)]
pub struct Caps {pub rows:usize,pub work:u64,pub queries:usize,pub cache_entries:usize,pub seconds:u64}
impl Default for Caps {fn default()->Self {Self {rows:500000,work:5000000,queries:50000,cache_entries:50000,seconds:20}}}
#[derive(Default, Debug, Serialize)]
pub struct Stats {
    pub query_lookups:u64,pub cache_hits:u64,pub generated_actor_queries:u64,pub completed_actor_queries:u64,
    pub actor_demands_by_level:Vec<u64>,pub unique_actor_misses_by_level:Vec<u64>,pub inner_worlds_by_level:Vec<u64>,
    pub raw_policy_consultations_by_level:Vec<u64>,
    pub native_choice_calls:u64,pub native_choice_us:u128,pub native_choice_declared_budget_sum_by_level:Vec<u64>,
    pub native_core_nodes:u64,pub native_core_pi_misses_by_level:Vec<u64>,pub native_core_reported_inner_worlds_by_level:Vec<u64>,pub native_core_policy_cache_entries:usize,
    pub row_edges:u64,pub public_coordinates:u64,
    pub public_step_calls:u64,pub reused_public_steps:u64,pub public_record_calls:u64,pub avoided_record_calls:u64,pub prefix_lookup_probes:u64,pub peak_layer_rows:usize,pub peak_retained_array_bytes:usize,pub peak_counted_live_array_bytes:usize,
    pub preparation_us:u128,pub demand_grouping_us:u128,pub forward_us:u128,pub fold_us:u128,pub cache_lookup_us:u128,
    pub sample_us:u128,pub cache_key_bytes:usize,pub cache_entries:usize,pub refused_batches:u64,
    pub bounded_choice_calls:u64,pub bounded_expansions:u64,pub bounded_refinements:u64,
    pub bounded_children_created:u64,pub bounded_children_unresolved:u64,pub bounded_root_unresolved:u64,
    pub peak_bounded_live_bytes:usize,
    pub arena_setup_us:u128,pub arena_schedule_us:u128,pub arena_expand_us:u128,pub arena_lower_us:u128,
    pub arena_rounds:u64,pub arena_batch_unique:u64,pub arena_batch_duplicates:u64,pub arena_prepared_before_solve:u64,
}
#[derive(Clone, Copy)]
struct Row {w:u32,h:u32}
#[derive(Clone, Copy)]
struct Coordinate {s:State,q:u32}
#[derive(Clone, Copy)]
struct Frame {terminal:u8,seat:u8,mine:bool,record:u64,recorded:bool,own_legal:u32}
impl Frame {fn is_terminal(self)->bool {self.terminal<2}}
#[derive(Clone, Copy)]
struct Edge {parent:u32,history:u32,tile:u8,mine:bool}
struct Layer {edges:Vec<Edge>,parent_rows:usize,query_ids:Vec<u32>}
#[derive(Clone, Copy)]
struct Link {tile:u8,next:u32}
/// Ephemeral exact (parent prefix,tile) interner. Small child lists replace
/// repeated hash-map probes; no world/private hand participates in this key.
enum PrefixIndex {Hashed(HashMap<(u32,u8),u32>),Linked {heads:Vec<u32>,links:Vec<Link>}}
impl PrefixIndex {
    fn new(parents:usize,hashed:bool)->Self {if hashed {Self::Hashed(HashMap::new())} else {Self::Linked {heads:vec![u32::MAX;parents],links:Vec::new()}}}
    fn intern(&mut self,parent:u32,tile:u8)->(u32,bool,u64) {
        match self {
            Self::Hashed(ids)=>{let next=ids.len() as u32;match ids.entry((parent,tile)) {std::collections::hash_map::Entry::Occupied(e)=>(*e.get(),false,1),std::collections::hash_map::Entry::Vacant(e)=>(*e.insert(next),true,1)}},
            Self::Linked {heads,links}=>{
                let head=&mut heads[parent as usize];let mut id=*head;let mut probes=0;
                while id!=u32::MAX {probes+=1;let link=links[id as usize];if link.tile==tile {return (id,false,probes);}id=link.next;}
                let id=links.len() as u32;links.push(Link {tile,next:*head});*head=id;(id,true,probes)
            }
        }
    }
    fn counted_array_bytes(&self)->usize {match self {Self::Hashed(_)=>0,Self::Linked {heads,links}=>heads.capacity()*size_of::<u32>()+links.capacity()*size_of::<Link>()}}
}

pub struct Service {
    pub stats:Stats, caps:Caps, deadline:Instant,
    actor_cache:HashMap<ActorSpec,Answer>, root_cache:HashMap<Query,Answer>,tables:HashMap<usize,Tables>,live_parent_array_bytes:usize,
    hashed_prefixes:bool,native_choices:bool,choice_cache:HashMap<ActorSpec,u8>,native_contexts:HashMap<Context,Arc<Shared>>,
    arena_choices:bool,bounded_choices:bool,eager_zero:bool,bounded_live_bytes:usize,bounded_live_rows:usize,
    choice_trace:Option<Vec<(ActorSpec,u8)>>,
}
impl Service {
    pub fn new(caps:Caps)->Res<Self> {if caps.rows==0 || caps.rows>1000000 || caps.work==0 || caps.work>100000000 || caps.queries==0 || caps.queries>100000 || caps.cache_entries==0 || caps.cache_entries>100000 || caps.seconds==0 || caps.seconds>60 {return Err("invalid service budget".into());}Ok(Self {deadline:Instant::now()+Duration::from_secs(caps.seconds),caps,stats:Stats::default(),actor_cache:HashMap::new(),root_cache:HashMap::new(),tables:HashMap::new(),live_parent_array_bytes:0,hashed_prefixes:false,native_choices:false,choice_cache:HashMap::new(),native_contexts:HashMap::new(),arena_choices:false,bounded_choices:false,eager_zero:false,bounded_live_bytes:0,bounded_live_rows:0,choice_trace:None})}
    pub fn with_hashed_prefixes(mut self)->Self {assert_eq!(self.cache_len(),0);self.hashed_prefixes=true;self}
    pub fn with_native_choices(mut self)->Self {assert_eq!(self.cache_len(),0);self.native_choices=true;self}
    pub fn with_arena_choices(mut self)->Self {assert_eq!(self.cache_len(),0);self.arena_choices=true;self}
    pub fn with_bounded_choices(mut self)->Self {assert_eq!(self.cache_len(),0);self.bounded_choices=true;self}
    pub fn with_eager_zero(mut self)->Self {assert_eq!(self.cache_len(),0);self.eager_zero=true;self}
    /// Validation-only trace. Disabled in every timing process; its cloned specs
    /// and memory/work are charged only to the separate validation receipt.
    pub fn with_choice_trace(mut self)->Self {assert_eq!(self.cache_len(),0);self.choice_trace=Some(Vec::new());self}
    pub fn take_choice_trace(&mut self)->Vec<(ActorSpec,u8)> {self.choice_trace.as_mut().map(std::mem::take).unwrap_or_default()}
    pub fn actor_choices(&mut self,specs:&[ActorSpec])->Res<Vec<u8>> {for a in specs {validate_frame(&a.context,a.state,a.actor,a.hand)?;self.tables.entry(a.context.decl).or_insert_with(||Tables::new(a.context.decl));}let r=self.choices(specs);self.refresh_native_stats();if r.is_err(){self.stats.refused_batches+=1;}r}
    fn check(&self)->Res<()> {if Instant::now()>=self.deadline {Err("service deadline".into())} else {Ok(())}}
    fn charge(&mut self,n:usize)->Res<()> {self.stats.row_edges+=n as u64;if self.stats.row_edges>self.caps.work {return Err("service total row-work cap".into());}self.check()}
    pub fn cache_len(&self)->usize {self.actor_cache.len()+self.root_cache.len()+self.choice_cache.len()}
    pub fn root_answers(&mut self, queries:&[Query])->Res<Vec<Answer>> {
        let r=self.root_answers_inner(queries);self.refresh_native_stats();if r.is_err() {self.stats.refused_batches+=1;}r
    }
    fn root_answers_inner(&mut self, queries:&[Query])->Res<Vec<Answer>> {
        self.check()?;let tick=Instant::now();let mut unique=Vec::new();let mut ids=HashMap::new();let mut slots=Vec::new();let mut found=Vec::new();
        for q in queries {self.stats.query_lookups+=1;if let Some(a)=self.root_cache.get(q) {self.stats.cache_hits+=1;slots.push(usize::MAX);found.push(Some(a.clone()));} else {let i=*ids.entry(q.clone()).or_insert_with(||{let i=unique.len();unique.push(q.clone());i});slots.push(i);found.push(None);}}
        self.stats.cache_lookup_us+=tick.elapsed().as_micros();let values=self.solve_grouped(&unique)?;
        for (q,a) in unique.into_iter().zip(values.iter()) {if self.cache_len()<self.caps.cache_entries {self.stats.cache_key_bytes+=query_bytes(&q);self.root_cache.insert(q,a.clone());}}
        self.stats.cache_entries=self.cache_len();Ok(slots.into_iter().zip(found).map(|(i,v)|v.unwrap_or_else(||values[i].clone())).collect())
    }
    pub fn actors(&mut self, specs:&[ActorSpec])->Res<Vec<Answer>> {
        let r=self.actors_inner(specs);if r.is_err() {self.stats.refused_batches+=1;}r
    }
    fn actors_inner(&mut self, specs:&[ActorSpec])->Res<Vec<Answer>> {
        self.check()?;let tick=Instant::now();let mut unique=Vec::new();let mut ids=HashMap::new();let mut slots=Vec::new();let mut found=Vec::new();
        for a in specs {if a.level>2 || a.level>=a.context.budgets.len() {return Err("unsupported actor rung".into());}validate_frame(&a.context,a.state,a.actor,a.hand)?;self.stats.query_lookups+=1;grow(&mut self.stats.actor_demands_by_level,a.level);self.stats.actor_demands_by_level[a.level]+=1;
            if let Some(v)=self.actor_cache.get(a) {self.stats.cache_hits+=1;slots.push(usize::MAX);found.push(Some(v.clone()));} else {let i=*ids.entry(a.clone()).or_insert_with(||{let i=unique.len();unique.push(a.clone());i});slots.push(i);found.push(None);}}
        self.stats.cache_lookup_us+=tick.elapsed().as_micros();
        if self.stats.generated_actor_queries+unique.len() as u64>self.caps.queries as u64 {return Err("actor query cap".into());}
        let tick=Instant::now();let mut prepared=Vec::new();
        for a in &unique {self.check()?;let q=prepare_actor(a,Deadline::after(self.deadline.saturating_duration_since(Instant::now())))?;grow(&mut self.stats.inner_worlds_by_level,a.level);self.stats.inner_worlds_by_level[a.level]+=q.worlds.len() as u64;grow(&mut self.stats.unique_actor_misses_by_level,a.level);self.stats.unique_actor_misses_by_level[a.level]+=1;self.stats.generated_actor_queries+=1;prepared.push(q);}
        self.stats.sample_us+=tick.elapsed().as_micros();let values=self.solve_grouped(&prepared)?;
        for (a,v) in unique.into_iter().zip(values.iter()) {self.stats.completed_actor_queries+=1;if self.cache_len()<self.caps.cache_entries {self.stats.cache_key_bytes+=size_of::<ActorSpec>()+a.context.budgets.len()*size_of::<usize>();self.actor_cache.insert(a,v.clone());}}
        self.stats.cache_entries=self.cache_len();Ok(slots.into_iter().zip(found).map(|(i,v)|v.unwrap_or_else(||values[i].clone())).collect())
    }
    fn choices(&mut self,specs:&[ActorSpec])->Res<Vec<u8>> {
        if self.arena_choices {return self.arena_actor_choices(specs);}
        if self.bounded_choices {return self.bounded_actor_choices(specs);}
        if !self.native_choices {return Ok(self.actors(specs)?.into_iter().map(|a|a.choice).collect());}
        self.check()?;let mut replies=Vec::with_capacity(specs.len());
        for a in specs {
            if a.level>2 || a.level>=a.context.budgets.len() {return Err("unsupported actor rung".into());}validate_frame(&a.context,a.state,a.actor,a.hand)?;self.stats.query_lookups+=1;grow(&mut self.stats.actor_demands_by_level,a.level);self.stats.actor_demands_by_level[a.level]+=1;
            let tick=Instant::now();let hit=self.choice_cache.get(a).copied();self.stats.cache_lookup_us+=tick.elapsed().as_micros();if let Some(choice)=hit {self.stats.cache_hits+=1;replies.push(choice);continue;}
            self.check()?;if self.stats.generated_actor_queries>=self.caps.queries as u64 {return Err("actor query cap".into());}
            let c=&a.context;let remaining=self.deadline.saturating_duration_since(Instant::now());
            let sh=Arc::clone(self.native_contexts.entry(c.clone()).or_insert_with(||Arc::new(Shared::new(solver::decl_of(c.decl),c.bid,c.budgets.clone(),c.boundary,c.boundary_size,Deadline::after(remaining)).with_inner_belief(c.belief()))));
            let seat=Seat::from_index(a.actor as usize).unwrap();let solver=Solver::new(sh,seat,a.hand,a.actor%2==1,Vec::new(),Vec::new(),Field::Dice);
            let lm=self.tables[&c.decl].legal(a.state,a.hand);let tick=Instant::now();let choice=solver.modeled_choice(a.level,&a.state.key(),seat,a.hand,lm).ok_or("native demanded choice refused")?;self.stats.native_choice_us+=tick.elapsed().as_micros();self.check()?;
            self.stats.native_choice_calls+=1;self.stats.generated_actor_queries+=1;self.stats.completed_actor_queries+=1;grow(&mut self.stats.unique_actor_misses_by_level,a.level);self.stats.unique_actor_misses_by_level[a.level]+=1;
            // Declared budgets summed over completed API calls, not actual
            // samples: native policy-cache hits may do no fresh sampling.
            grow(&mut self.stats.native_choice_declared_budget_sum_by_level,a.level);self.stats.native_choice_declared_budget_sum_by_level[a.level]+=c.budgets[a.level] as u64;
            if self.cache_len()<self.caps.cache_entries {self.stats.cache_key_bytes+=size_of::<ActorSpec>()+c.budgets.len()*size_of::<usize>();self.choice_cache.insert(a.clone(),choice);}replies.push(choice);
        }
        self.stats.cache_entries=self.cache_len();Ok(replies)
    }
    fn refresh_native_stats(&mut self) {
        self.stats.native_core_nodes=self.native_contexts.values().map(|s|s.nodes.load(std::sync::atomic::Ordering::Relaxed)).sum();self.stats.native_core_policy_cache_entries=self.native_contexts.values().map(|s|s.pi_cache_len()).sum();
        self.stats.native_core_pi_misses_by_level.clear();self.stats.native_core_reported_inner_worlds_by_level.clear();
        for sh in self.native_contexts.values() {for (k,n) in sh.pi_calls_by_level().into_iter().enumerate() {grow(&mut self.stats.native_core_pi_misses_by_level,k);self.stats.native_core_pi_misses_by_level[k]+=n;}for (k,n) in sh.inner_worlds_by_level().into_iter().enumerate() {grow(&mut self.stats.native_core_reported_inner_worlds_by_level,k);self.stats.native_core_reported_inner_worlds_by_level[k]+=n;}}
    }
    fn solve_grouped(&mut self,queries:&[Query])->Res<Vec<Answer>> {
        let tick=Instant::now();let mut groups:BTreeMap<usize,Vec<usize>>=BTreeMap::new();for (i,q) in queries.iter().enumerate() {groups.entry(q.context.decl).or_default().push(i);}self.stats.demand_grouping_us+=tick.elapsed().as_micros();
        if groups.len()<=1 {return self.solve_batch(queries);}
        let mut answer:Vec<Option<Answer>>=vec![None;queries.len()];for ids in groups.values() {let group=ids.iter().map(|&i|queries[i].clone()).collect::<Vec<_>>();let values=self.solve_batch(&group)?;for (&i,v) in ids.iter().zip(values) {answer[i]=Some(v);}}
        Ok(answer.into_iter().map(Option::unwrap).collect())
    }
    fn solve_batch(&mut self, queries:&[Query])->Res<Vec<Answer>> {
        if queries.is_empty() {return Ok(Vec::new());}self.check()?;
        let tick=Instant::now();let mut rows=Vec::new();let mut coordinates=Vec::new();let mut answers=Vec::new();
        for (qid,q) in queries.iter().enumerate() {
            validate_prepared(q)?;self.tables.entry(q.context.decl).or_insert_with(||Tables::new(q.context.decl));
            answers.push(Answer {tiles:Vec::new(),counts:Vec::new(),choice:0});coordinates.push(Coordinate {s:q.state,q:qid as u32});
            for w in 0..q.worlds.len() {rows.push(Row {w:w as u32,h:qid as u32});}
        }
        if rows.len()>self.caps.rows {return Err("initial row cap".into());}self.stats.preparation_us+=tick.elapsed().as_micros();let mut levels=Vec::new();let mut depth=0;let mut retained=0;
        loop {
            self.check()?;let tick=Instant::now();
            // Derived local views; the public State has one authority per
            // collision-free prefix coordinate. Never identify by played mask.
            let mut frames=coordinates.iter().map(|c| {
                let q=&queries[c.q as usize];let terminal=c.s.terminal(q.context.bid);let seat=c.s.actor();let mine=terminal.is_none() && seat==q.actor as usize;
                Frame {terminal:terminal.map(|n|n as u8).unwrap_or(2),seat:seat as u8,mine,record:0,recorded:false,own_legal:if mine {self.tables[&q.context.decl].legal(c.s,q.hand&!c.s.played)} else {0}}
            }).collect::<Vec<_>>();
            if frames.iter().all(|f|f.is_terminal()) {break;}
            if depth>=16 {return Err("remaining-ply cap".into());}
            let mut specs=Vec::new();let mut spec_ids=HashMap::new();let mut demand=vec![None;rows.len()];let mut legal=Vec::with_capacity(rows.len());let mut projected=0usize;
            for (i,r) in rows.iter().enumerate() {
                let c=coordinates[r.h as usize];let q=&queries[c.q as usize];let f=frames[r.h as usize];let alive=!f.is_terminal();
                let hand=q.worlds[r.w as usize][f.seat as usize]&!c.s.played;
                let mask=if f.mine {f.own_legal} else if alive {self.tables[&q.context.decl].legal(c.s,hand)} else {0};
                if alive && mask==0 {return Err("no legal continuation".into());}legal.push(mask);projected+=if f.mine {mask.count_ones() as usize} else {1};
                if alive && !f.mine && mask.count_ones()>1 {
                    if let Nature::Policy(level)=q.nature {
                        grow(&mut self.stats.raw_policy_consultations_by_level,level);self.stats.raw_policy_consultations_by_level[level]+=1;
                        // h fixes the complete query/context/public prefix and
                        // rung. This local key avoids cloning those per world.
                        let id=*spec_ids.entry((r.h,hand)).or_insert_with(||{let id=specs.len();specs.push(ActorSpec {context:q.context.clone(),state:c.s,actor:f.seat as u8,hand,level});id});demand[i]=Some(id);
                    }
                }
            }
            self.stats.demand_grouping_us+=tick.elapsed().as_micros();if projected>self.caps.rows {return Err("expanded row cap".into());}self.charge(projected)?;
            let held=retained+rows.capacity()*size_of::<Row>()+coordinates.capacity()*size_of::<Coordinate>()+frames.capacity()*size_of::<Frame>()+legal.capacity()*size_of::<u32>()+demand.capacity()*size_of::<Option<usize>>();
            self.live_parent_array_bytes+=held;self.stats.peak_counted_live_array_bytes=self.stats.peak_counted_live_array_bytes.max(self.live_parent_array_bytes);
            let reply_result=self.choices(&specs);self.live_parent_array_bytes-=held;let replies=reply_result?;
            let tick=Instant::now();let mut next=Vec::with_capacity(projected);let mut next_coordinates=Vec::with_capacity(projected);let mut edges=Vec::with_capacity(projected);let mut ids=PrefixIndex::new(coordinates.len(),self.hashed_prefixes);
            for (i,r) in rows.iter().enumerate() {
                let c=coordinates[r.h as usize];let q=&queries[c.q as usize];let f=frames[r.h as usize];let terminal=f.is_terminal();let mask=legal[i];
                let selected=if terminal {STOP} else if f.mine {0} else if mask.count_ones()==1 {mask.trailing_zeros() as u8} else {match &q.nature {
                    Nature::Tape(t)=>nth(mask,((t[r.w as usize][depth] as u128*mask.count_ones() as u128)>>64) as u32),
                    Nature::Native(seeds)=>{let cached=&mut frames[r.h as usize];if cached.recorded {self.stats.avoided_record_calls+=1;} else {cached.record=record(c.s);cached.recorded=true;self.stats.public_record_calls+=1;}nth(mask,SplitMix64(seeds[r.w as usize]^cached.record).below(mask.count_ones() as u64) as u32)},
                    Nature::Policy(_)=>replies[demand[i].unwrap()],
                }};
                let mut choices=if f.mine {mask} else {0};
                loop {
                    let tile=if f.mine {if choices==0 {break;}let tile=choices.trailing_zeros() as u8;choices &= choices-1;tile} else {selected};
                    if !terminal && mask&(1<<tile)==0 {return Err("field reply illegal for actor".into());}
                    let (h,fresh,probes)=ids.intern(r.h,tile);self.stats.prefix_lookup_probes+=probes;
                    if fresh {
                        debug_assert_eq!(h as usize,next_coordinates.len());
                        let s=if terminal {c.s} else {self.stats.public_step_calls+=1;self.tables[&q.context.decl].step(c.s,tile)};
                        next_coordinates.push(Coordinate {s,q:c.q});
                    } else if !terminal {self.stats.reused_public_steps+=1;}
                    next.push(Row {w:r.w,h});edges.push(Edge {parent:i as u32,history:r.h,tile,mine:f.mine});
                    if !f.mine {break;}
                }
            }
            self.stats.public_coordinates+=next_coordinates.len() as u64;self.stats.peak_layer_rows=self.stats.peak_layer_rows.max(next.len());
            let query_ids=coordinates.iter().map(|c|c.q).collect::<Vec<_>>();
            retained+=edges.capacity()*size_of::<Edge>()+query_ids.capacity()*size_of::<u32>();
            let arrays=retained+ids.counted_array_bytes()+rows.capacity()*size_of::<Row>()+next.capacity()*size_of::<Row>()+coordinates.capacity()*size_of::<Coordinate>()+next_coordinates.capacity()*size_of::<Coordinate>()+frames.capacity()*size_of::<Frame>()+legal.capacity()*size_of::<u32>()+demand.capacity()*size_of::<Option<usize>>();
            self.stats.peak_retained_array_bytes=self.stats.peak_retained_array_bytes.max(arrays);self.stats.peak_counted_live_array_bytes=self.stats.peak_counted_live_array_bytes.max(self.live_parent_array_bytes+arrays);
            levels.push(Layer {edges,parent_rows:rows.len(),query_ids});coordinates=next_coordinates;rows=next;depth+=1;self.stats.forward_us+=tick.elapsed().as_micros();
        }
        if levels.is_empty() {return Err("query has settled root".into());}
        let tick=Instant::now();let mut values=rows.iter().map(|r|{let c=coordinates[r.h as usize];c.s.terminal(queries[c.q as usize].context.bid).unwrap()}).collect::<Vec<_>>();
        // Fold never reads a public State or a hidden hand. Integer sums for
        // every reachable world sharing h precede one common MAX/MIN choice.
        drop(rows);drop(coordinates);
        for (li,layer) in levels.iter().enumerate().rev() {
            self.check()?;let histories=layer.query_ids.len();let mut sums=vec![[0u64;28];histories];let mut seen=vec![0u32;histories];
            for (i,e) in layer.edges.iter().enumerate() {if e.mine {let h=e.history as usize;sums[h][e.tile as usize]+=values[i];seen[h]|=1<<e.tile;}}
            let mut best=vec![0u8;histories];for h in 0..histories {if seen[h]!=0 {let tiles=solver::mask_bits(seen[h]);let counts=tiles.iter().map(|&t|sums[h][t as usize]).collect();let a=Answer::new(tiles,counts,queries[layer.query_ids[h] as usize].actor%2==1);best[h]=a.choice;if li==0 {answers[h]=a;}}}
            let mut parent=vec![0u64;layer.parent_rows];
            for (i,e) in layer.edges.iter().enumerate() {if !e.mine || e.tile==best[e.history as usize] {parent[e.parent as usize]=values[i];}}
            let arrays=retained+values.capacity()*size_of::<u64>()+parent.capacity()*size_of::<u64>()+sums.capacity()*size_of::<[u64;28]>()+seen.capacity()*size_of::<u32>()+best.capacity();
            self.stats.peak_retained_array_bytes=self.stats.peak_retained_array_bytes.max(arrays);self.stats.peak_counted_live_array_bytes=self.stats.peak_counted_live_array_bytes.max(self.live_parent_array_bytes+arrays);
            values=parent;
        }
        self.stats.fold_us+=tick.elapsed().as_micros();if answers.iter().any(|a|a.tiles.is_empty()) {return Err("missing root comparison".into());}Ok(answers)
    }

}
fn grow(v:&mut Vec<u64>,i:usize) {if v.len()<=i {v.resize(i+1,0);}}
fn query_bytes(q:&Query)->usize {size_of::<Query>()+q.context.budgets.len()*size_of::<usize>()+q.worlds.len()*size_of::<[u32;4]>()+match &q.nature {Nature::Tape(t)=>t.iter().map(|x|size_of::<Vec<u64>>()+8*x.len()).sum(),Nature::Native(s)=>8*s.len(),Nature::Policy(_)=>0}}
fn validate_frame(c:&Context,s:State,actor:u8,hand:u32)->Res<()> {
    if c.decl>7 && c.decl!=9 || !(1..=42).contains(&c.bid) || c.budgets.is_empty() || c.budgets.len()>4 || c.budgets.iter().any(|&n|n==0 || n>1024) || c.boundary_size>7 || actor>3 || s.leader>3 || s.len>3 || s.played&!FULL!=0 || hand&!FULL!=0 || hand&s.played!=0 || c.counted!=s.voids.is_some() {return Err("unsupported query frame".into());}
    let exposed=s.played.count_ones() as usize;let boundary=c.boundary.count_ones() as usize;
    if exposed<12 || exposed>27 || exposed<s.len as usize || (exposed-s.len as usize)%4!=0 || c.boundary&!s.played!=0 || boundary%4!=0 || boundary>exposed-s.len as usize || c.boundary_size+boundary/4!=7 || s.actor()!=actor as usize {return Err("malformed boundary/public frame".into());}
    let mut trick=0;for &t in &s.plays[..s.len as usize] {if t>27 || trick&(1<<t)!=0 || s.played&(1<<t)==0 {return Err("invalid partial trick".into());}trick|=1<<t;}
    if s.plays[s.len as usize..].iter().any(|&t|t!=0) {return Err("noncanonical unused trick slots".into());}
    let total_points=solver::mask_bits(s.played&!trick).iter().map(|&t|Domino::from_index(t as usize).unwrap().count() as usize).sum::<usize>()+(exposed-s.len as usize)/4;
    if s.t1 as usize+s.t0 as usize!=total_points || s.terminal(c.bid).is_some() {return Err("invalid banked totals/settled query".into());}
    let sizes=c.sizes(s);if hand.count_ones() as usize!=sizes[actor as usize] {return Err("actor hand capacity".into());}
    if let Some(v)=s.voids {if v.iter().any(|&m|m&!FULL!=0) || hand&v[actor as usize]!=0 {return Err("invalid tracked void/own hand".into());}let d=solver::decl_of(c.decl);for mask in v {let mut union=0;for q in walt::rules::Context::ALL {let incidence=solver::mask_of(d.effective_incidence(q));if incidence!=0 && incidence&!mask==0 {union|=incidence;}}if union!=mask {return Err("tracked void not a context union".into());}}solver::belief_frame_feasibility(actor as usize,hand,s.played,sizes,v).map_err(|_|"empty tracked actor fiber")?;}
    Ok(())
}
fn validate_prepared(q:&Query)->Res<()> {
    let c=&q.context;validate_frame(c,q.state,q.actor,q.hand)?;if q.worlds.is_empty() || q.worlds.len()>1024 {return Err("unsupported prepared query".into());}
    if let Nature::Policy(k)=q.nature {if k>2 || k>=c.budgets.len() {return Err("unsupported field rung".into());}}
    if let Nature::Native(s)=&q.nature {if s.len()!=q.worlds.len() {return Err("seed dimensions".into());}}
    if let Nature::Tape(t)=&q.nature {if t.len()!=q.worlds.len() || t.iter().any(|x|x.len()!=28-q.state.played.count_ones() as usize) {return Err("tape dimensions".into());}}
    let sizes=c.sizes(q.state);
    for w in &q.worlds {let mut union=0;for i in 0..4 {if w[i]&union!=0 || w[i]&q.state.played!=0 || w[i]&!FULL!=0 || w[i].count_ones() as usize!=sizes[i] {return Err("query world capacity/overlap".into());}if let Some(v)=q.state.voids {if w[i]&v[i]!=0 {return Err("tracked belief world violates voids".into());}}union|=w[i];}if union!=FULL&!q.state.played || w[q.actor as usize]!=q.hand {return Err("query world/actor hand mismatch".into());}}
    Ok(())
}
pub fn reference(q:&Query, parallel:bool, seconds:u64)->Res<(Answer,u128,u64)> {
    let sh=Arc::new(Shared::new(solver::decl_of(q.context.decl),q.context.bid,q.context.budgets.clone(),q.context.boundary,q.context.boundary_size,Deadline::after(Duration::from_secs(seconds))).with_inner_belief(q.context.belief()));
    let (field,seeds)=match &q.nature {Nature::Native(s)=>(Field::Dice,s.clone()),Nature::Policy(k)=>(Field::Level(*k),Vec::new()),Nature::Tape(_)=>return Err("depth tape has Python reference".into())};
    let seat=Seat::from_index(q.actor as usize).unwrap();let solver=Solver::new(Arc::clone(&sh),seat,q.hand,q.actor%2==1,q.worlds.clone(),seeds,field);let solver=if parallel {solver.parallel()} else {solver};
    let table=Tables::new(q.context.decl);let tiles=solver::mask_bits(table.legal(q.state,q.hand));let tick=Instant::now();let vals=solver.action_values(&q.state.key(),&tiles).ok_or("pinned native reference deadline")?;let us=tick.elapsed().as_micros();
    let counts=vals.iter().map(|(_,v)|{let n=v*BigRational::from_integer(BigInt::from(q.worlds.len()));assert!(n.is_integer());n.to_integer().to_u64().unwrap()}).collect();Ok((Answer::new(tiles,counts,q.actor%2==1),us,sh.nodes.load(std::sync::atomic::Ordering::Relaxed)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use walt::rules::legal_plays;
    #[test] fn tables_match_native_all_leads_hands_and_tricks() {
        for decl in [0,1,2,3,4,5,6,7,9] {let t=Tables::new(decl);let d=solver::decl_of(decl);
            for lead in 0..28 {for hand in [FULL,0xaaaaaaa,0x5555555,1u32<<lead] {let s=State {played:0,leader:0,plays:[lead as u8,0,0],len:1,t1:0,t0:0,voids:None};let native=solver::mask_of(legal_plays(d,solver::set_of(hand),Some(d.led_context(Domino::from_index(lead).unwrap()))));assert_eq!(t.legal(s,hand),native);}
                for b in 0..28 {if b==lead {continue;}for c in 0..28 {if c==lead || c==b {continue;}for z in 0..28 {if z==lead || z==b || z==c {continue;}let mut k=Key {played:(1<<lead)|(1<<b)|(1<<c),leader:2,plays:vec![lead as u8,b as u8,c as u8],banked_t1:0,banked_t0:0,voids:None,alive:0};let s=State::from_key(&k);Contract::Straight {bid:30}.step(&mut k,d,Domino::from_index(z).unwrap());assert_eq!(t.step(s,z as u8),State::from_key(&k));}}}
            }
        }
    }
    #[test] fn prefix_indices_preserve_exact_pairs_and_order() {
        let mut hash=PrefixIndex::new(128,true);let mut linked=PrefixIndex::new(128,false);
        for round in 0..3 {for parent in 0..128 {for tile in 0..=STOP {let parent=(parent*13+round)%128;let (a,af,_)=hash.intern(parent,tile);let (b,bf,_)=linked.intern(parent,tile);assert_eq!((a,af),(b,bf));}}}
    }
    #[test] fn ascending_ties_both_teams() {for max in [false,true] {assert_eq!(Answer::new(vec![0,7,27],vec![4,4,4],max).choice,0);}}
    #[test] fn cache_identity_uses_scores_budgets_belief_and_frame() {
        let a=ActorSpec {context:Context {decl:2,bid:30,budgets:vec![4,2],boundary:0,boundary_size:7,counted:false},state:State {played:0,leader:1,plays:[0;3],len:0,t1:0,t0:0,voids:None},actor:1,hand:127,level:0};let mut all=std::collections::HashSet::new();assert!(all.insert(a.clone()));
        for field in 0..12 {let mut b=a.clone();match field {0=>b.context.decl=3,1=>b.context.bid=31,2=>b.context.budgets[0]=8,3=>b.context.boundary=15,4=>b.context.boundary_size=6,5=>b.context.counted=true,6=>b.state.t1=1,7=>b.state.t0=1,8=>b.state.voids=Some([0;4]),9=>b.actor=3,10=>b.hand=255,_=>b.level=1};assert!(all.insert(b));}
    }
}
