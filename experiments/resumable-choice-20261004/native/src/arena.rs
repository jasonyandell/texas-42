//! Round-wise forest of bounded actor choices. Nodes and ordered world indices
//! live in contiguous arenas. Rounds preserve each tree's refinement order;
//! preparing the forest ahead of solving can change global demand on refusal.
use super::*;
use std::ops::Range;
#[derive(Clone)]
struct Node {state:State,worlds:Range<usize>,depth:usize,lo:u64,hi:u64,children:Range<usize>,fresh:bool}
#[derive(Default)]
struct Arena {nodes:Vec<Node>,worlds:Vec<u32>,edges:Vec<(u8,usize)>}
impl Arena {
    fn promising(&self,id:usize,max:bool)->Option<usize> {let mut best=None;for &(_,c) in &self.edges[self.nodes[id].children.clone()] {let n=&self.nodes[c];if n.lo==n.hi {continue;}if best.is_none_or(|b:usize|if max {n.hi>self.nodes[b].hi}else{n.lo<self.nodes[b].lo}) {best=Some(c);}}best}
    fn fold(&mut self,id:usize,q:&Query) {
        let n=&self.nodes[id];if n.fresh || n.children.is_empty(){return;}
        let focal=n.state.actor()==q.actor as usize;let max=q.actor%2==1;
        let mut values=self.edges[n.children.clone()].iter().map(|&(_,c)|(self.nodes[c].lo,self.nodes[c].hi));
        let (mut lo,mut hi)=values.next().unwrap();
        for (l,h) in values {if focal {if max {lo=lo.max(l);hi=hi.max(h);}else{lo=lo.min(l);hi=hi.min(h);}}else{lo+=l;hi+=h;}}
        self.nodes[id].lo=lo;self.nodes[id].hi=hi;
    }
    fn target(&self,id:usize,q:&Query,path:&mut Vec<usize>)->Res<usize> {path.push(id);let n=&self.nodes[id];if n.fresh{return Ok(id);}let c=if n.state.actor()==q.actor as usize {self.promising(id,q.actor%2==1)}else{self.edges[n.children.clone()].iter().map(|x|x.1).find(|&c|self.nodes[c].lo!=self.nodes[c].hi)}.ok_or("arena refinement stalled")?;self.target(c,q,path)}
    fn unresolved(&self,id:usize)->u64 {let n=&self.nodes[id];u64::from(n.lo!=n.hi)+self.edges[n.children.clone()].iter().map(|e|self.unresolved(e.1)).sum::<u64>()}
}
impl Service {
    fn arena_node(&mut self,a:&mut Arena,q:&Query,state:State,worlds:Vec<u32>,depth:usize)->Res<usize> {
        if self.bounded_live_rows.checked_add(worlds.len()).ok_or("arena row overflow")?>self.caps.rows{return Err("bounded retained row cap".into());}
        self.bounded_live_rows+=worlds.len();self.bounded_live_bytes+=size_of::<Node>()+size_of::<(u8,usize)>()+worlds.len()*size_of::<u32>();
        self.stats.peak_bounded_live_bytes=self.stats.peak_bounded_live_bytes.max(self.bounded_live_bytes);self.stats.peak_counted_live_array_bytes=self.stats.peak_counted_live_array_bytes.max(self.live_parent_array_bytes+self.bounded_live_bytes);self.stats.bounded_children_created+=1;
        let mass=worlds.len() as u64;let start=a.worlds.len();a.worlds.extend(worlds);let end=a.worlds.len();let v=state.terminal(q.context.bid);let id=a.nodes.len();a.nodes.push(Node {state,worlds:start..end,depth,lo:v.unwrap_or(0)*mass,hi:v.unwrap_or(1)*mass,children:0..0,fresh:v.is_none()});Ok(id)
    }
    pub(super) fn arena_actor_choices(&mut self,specs:&[ActorSpec])->Res<Vec<u8>> {
        self.check()?;let mut unique=Vec::new();let mut ids=HashMap::new();let mut routes=Vec::new();let mut queries=Vec::new();
        for s in specs {validate_frame(&s.context,s.state,s.actor,s.hand)?;if s.level>2 || s.level>=s.context.budgets.len(){return Err("unsupported actor rung".into());}self.stats.query_lookups+=1;grow(&mut self.stats.actor_demands_by_level,s.level);self.stats.actor_demands_by_level[s.level]+=1;
            if let Some(v)=self.choice_cache.get(s).copied().or_else(||self.actor_cache.get(s).map(|v|v.choice)){self.stats.cache_hits+=1;routes.push(Ok(v));continue;}
            if let Some(&id)=ids.get(s){self.stats.arena_batch_duplicates+=1;routes.push(Err(id));continue;}
            if self.stats.generated_actor_queries>=self.caps.queries as u64{return Err("actor query cap".into());}self.check()?;let t=Instant::now();let q=prepare_actor(s,Deadline::after(self.deadline.saturating_duration_since(Instant::now())))?;self.stats.sample_us+=t.elapsed().as_micros();self.stats.generated_actor_queries+=1;grow(&mut self.stats.unique_actor_misses_by_level,s.level);self.stats.unique_actor_misses_by_level[s.level]+=1;grow(&mut self.stats.inner_worlds_by_level,s.level);self.stats.inner_worlds_by_level[s.level]+=q.worlds.len() as u64;self.tables.entry(q.context.decl).or_insert_with(||Tables::new(q.context.decl));let id=unique.len();ids.insert(s.clone(),id);unique.push(s.clone());queries.push(q);routes.push(Err(id));
        }
        self.stats.arena_batch_unique+=unique.len() as u64;self.stats.arena_prepared_before_solve+=queries.len().saturating_sub(1) as u64;
        let saved=(self.bounded_live_rows,self.bounded_live_bytes);let values=self.arena_queries(&queries);self.bounded_live_rows=saved.0;self.bounded_live_bytes=saved.1;let values=values?;
        for (s,&v) in unique.iter().zip(&values){self.stats.completed_actor_queries+=1;self.stats.bounded_choice_calls+=1;if self.cache_len()<self.caps.cache_entries {self.stats.cache_key_bytes+=size_of::<ActorSpec>()+s.context.budgets.len()*size_of::<usize>();self.choice_cache.insert(s.clone(),v);}}
        self.stats.cache_entries=self.cache_len();let out=routes.into_iter().map(|r|r.unwrap_or_else(|id|values[id])).collect::<Vec<_>>();if let Some(t)=&mut self.choice_trace {t.extend(specs.iter().cloned().zip(out.iter().copied()));}Ok(out)
    }
    fn arena_queries(&mut self,qs:&[Query])->Res<Vec<u8>> {
        let setup=Instant::now();let mut a=Arena::default();let mut roots=Vec::new();for q in qs {validate_prepared(q)?;roots.push(self.arena_node(&mut a,q,q.state,(0..q.worlds.len() as u32).collect(),0)?);}
        self.stats.arena_setup_us+=setup.elapsed().as_micros();let mut out=vec![None;qs.len()];
        while out.iter().any(Option::is_none) {self.check()?;self.stats.arena_rounds+=1;let schedule=Instant::now();let mut targets=Vec::new();
            for (i,q) in qs.iter().enumerate(){if out[i].is_some(){continue;}let r=roots[i];if !a.nodes[r].fresh {let edges=&a.edges[a.nodes[r].children.clone()];let bs=edges.iter().map(|e|(a.nodes[e.1].lo,a.nodes[e.1].hi)).collect::<Vec<_>>();if let Some(c)=certified_index(&bs,q.actor%2==1){out[i]=Some(edges[c].0);self.stats.bounded_children_unresolved+=edges.iter().map(|e|a.unresolved(e.1)).sum::<u64>();self.stats.bounded_root_unresolved+=edges.iter().filter(|e|a.nodes[e.1].lo!=a.nodes[e.1].hi).count() as u64;continue;}}
                let mut path=Vec::new();let id=if a.nodes[r].fresh {r}else{let c=a.promising(r,q.actor%2==1).ok_or("arena choice stalled")?;a.target(c,q,&mut path)?};self.stats.bounded_refinements+=path.len() as u64;targets.push((i,id,path));
            }
            self.stats.arena_schedule_us+=schedule.elapsed().as_micros();let expand=Instant::now();let mut demands=Vec::new();let mut ids=HashMap::new();let mut plans=Vec::new();
            for &(i,id,_) in &targets {let q=&qs[i];let n=&a.nodes[id];if n.depth>=16{return Err("remaining-ply cap".into());}self.stats.bounded_expansions+=1;let seat=n.state.actor();let mut rows=Vec::new();if seat!=q.actor as usize {for &w in &a.worlds[n.worlds.clone()] {let hand=q.worlds[w as usize][seat]&!n.state.played;let mask=self.tables[&q.context.decl].legal(n.state,hand);if mask==0{return Err("no legal arena continuation".into());}let d=if mask.count_ones()>1 {if let Nature::Policy(level)=q.nature {grow(&mut self.stats.raw_policy_consultations_by_level,level);self.stats.raw_policy_consultations_by_level[level]+=1;let spec=ActorSpec{context:q.context.clone(),state:n.state,actor:seat as u8,hand,level};Some(*ids.entry(spec.clone()).or_insert_with(||{let d=demands.len();demands.push(spec);d}))}else{None}}else{None};rows.push((w,mask,d));}}plans.push(rows);}
            self.stats.arena_expand_us+=expand.elapsed().as_micros();let lower=Instant::now();let replies=self.arena_actor_choices(&demands)?;self.stats.arena_lower_us+=lower.elapsed().as_micros();let expand=Instant::now();
            for ((i,id,path),rows) in targets.into_iter().zip(plans) {let q=&qs[i];let n=a.nodes[id].clone();let mut groups=BTreeMap::<u8,Vec<u32>>::new();if n.state.actor()==q.actor as usize {let mask=self.tables[&q.context.decl].legal(n.state,q.hand&!n.state.played);if mask==0{return Err("no legal arena continuation".into());}self.charge(n.worlds.len()*mask.count_ones() as usize)?;for tile in solver::mask_bits(mask){groups.insert(tile,a.worlds[n.worlds.clone()].to_vec());}}else{let hash=if matches!(q.nature,Nature::Native(_)){self.stats.public_record_calls+=1;record(n.state)}else{0};for (w,mask,d) in rows {let tile=if mask.count_ones()==1 {mask.trailing_zeros() as u8}else{match &q.nature {Nature::Native(seeds)=>nth(mask,SplitMix64(seeds[w as usize]^hash).below(mask.count_ones() as u64) as u32),Nature::Tape(t)=>nth(mask,((t[w as usize][n.depth] as u128*mask.count_ones() as u128)>>64) as u32),Nature::Policy(_)=>replies[d.unwrap()]}};if mask&(1<<tile)==0{return Err("arena field reply illegal".into());}groups.entry(tile).or_default().push(w);}self.charge(n.worlds.len())?;}
                let mut edges=Vec::new();for (tile,worlds) in groups {self.stats.public_step_calls+=1;let state=self.tables[&q.context.decl].step(n.state,tile);let c=self.arena_node(&mut a,q,state,worlds,n.depth+1)?;edges.push((tile,c));}let start=a.edges.len();a.edges.extend(edges);a.nodes[id].children=start..a.edges.len();a.nodes[id].fresh=false;a.fold(id,q);for p in path.into_iter().rev(){a.fold(p,q);}a.fold(roots[i],q);
            }
            self.stats.arena_expand_us+=expand.elapsed().as_micros();
        }Ok(out.into_iter().map(Option::unwrap).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn spec(actor:u8,level:usize)->ActorSpec {
        let played=(1u32<<20)-1;
        let points=solver::mask_bits(played).iter().map(|&t|Domino::from_index(t as usize).unwrap().count()).sum::<u32>()+5;
        let t1=(points/2) as u8;
        ActorSpec {context:Context {decl:2,bid:30,budgets:vec![2,2,2],boundary:played,boundary_size:2,counted:false},state:State {played,leader:actor,plays:[0;3],len:0,t1,t0:points as u8-t1,voids:None},actor,hand:3<<20,level}
    }
    #[test] fn forest_matches_pure_and_complete_with_duplicates_and_both_teams() {
        for level in 0..=2 {let specs=vec![spec(1,level),spec(0,level),spec(1,level)];
            let mut pure=Service::new(Caps::default()).unwrap().with_bounded_choices();
            let mut arena=Service::new(Caps::default()).unwrap().with_arena_choices().with_choice_trace();
            let expected=pure.actor_choices(&specs).unwrap();assert_eq!(arena.actor_choices(&specs).unwrap(),expected);
            assert!(arena.stats.arena_batch_duplicates>=1);assert_eq!((arena.bounded_live_rows,arena.bounded_live_bytes),(0,0));
            let trace=arena.take_choice_trace();let mut complete=Service::new(Caps::default()).unwrap();
            for (s,v) in trace {assert_eq!(complete.actors(&[s]).unwrap()[0].choice,v);}
            let generated=arena.stats.generated_actor_queries;assert_eq!(arena.actor_choices(&specs).unwrap(),expected);assert_eq!(arena.stats.generated_actor_queries,generated);
        }
    }
    #[test] fn refused_forest_restores_live_accounting_and_does_not_cache_roots() {
        let mut service=Service::new(Caps {rows:1,..Caps::default()}).unwrap().with_arena_choices();assert!(service.actor_choices(&[spec(1,1)]).is_err());assert_eq!((service.bounded_live_rows,service.bounded_live_bytes),(0,0));assert!(service.choice_cache.is_empty());
    }
}
