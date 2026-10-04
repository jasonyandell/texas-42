//! Lazy public-prefix continuations. All rows in a focal information set stay
//! together. Bounds describe total unit-mass wins, never per-world actions.
use super::*;

/// Input order is the canonical ascending legal-tile order. Earlier competitors
/// require strict separation; later competitors allow equality.
pub fn certified_index(bounds:&[(u64,u64)],maximize:bool)->Option<usize> {
    for (a,&(lo,hi)) in bounds.iter().enumerate() {
        if lo>hi {return None;}
        if bounds.iter().enumerate().all(|(b,&(bl,bh))| {
            bl<=bh && (a==b || if maximize {if b<a {lo>bh} else {lo>=bh}}
            else if b<a {hi<bl} else {hi<=bl})
        }) {return Some(a);}
    }None
}

enum Kind {Fresh,Focal(Vec<(u8,Node)>),Nature(Vec<(u8,Node)>),Settled}
struct Node {state:State,worlds:Vec<u32>,depth:usize,lo:u64,hi:u64,kind:Kind}
impl Node {
    fn bounds(&self)->(u64,u64) {(self.lo,self.hi)}
    fn unresolved(&self)->u64 {
        let children=match &self.kind {Kind::Focal(v)|Kind::Nature(v)=>v.iter().map(|(_,n)|n.unresolved()).sum(),_=>0};
        children+u64::from(self.lo!=self.hi)
    }
    fn fold(&mut self,maximize:bool) {
        match &self.kind {
            Kind::Focal(v)=>{
                self.lo=if maximize {v.iter().map(|(_,n)|n.lo).max()} else {v.iter().map(|(_,n)|n.lo).min()}.unwrap();
                self.hi=if maximize {v.iter().map(|(_,n)|n.hi).max()} else {v.iter().map(|(_,n)|n.hi).min()}.unwrap();
            }
            Kind::Nature(v)=>{self.lo=v.iter().map(|(_,n)|n.lo).sum();self.hi=v.iter().map(|(_,n)|n.hi).sum();}
            _=>{}
        }
    }
}

impl Service {
    pub(super) fn bounded_actor_choices(&mut self,specs:&[ActorSpec])->Res<Vec<u8>> {
        // Controlled ablation: there are no lower-rung policies inside L0, so
        // batched full vectors there cannot create avoidable actor demand.
        if self.eager_zero && specs.iter().all(|a|a.level==0) {let replies=self.actors(specs)?.into_iter().map(|v|v.choice).collect::<Vec<_>>();if let Some(trace)=&mut self.choice_trace {trace.extend(specs.iter().cloned().zip(replies.iter().copied()));}return Ok(replies);}
        self.check()?;let mut out=Vec::with_capacity(specs.len());
        for a in specs {
            if a.level>2 || a.level>=a.context.budgets.len() {return Err("unsupported actor rung".into());}
            validate_frame(&a.context,a.state,a.actor,a.hand)?;
            self.stats.query_lookups+=1;grow(&mut self.stats.actor_demands_by_level,a.level);self.stats.actor_demands_by_level[a.level]+=1;
            let t=Instant::now();let hit=self.choice_cache.get(a).copied().or_else(||self.actor_cache.get(a).map(|v|v.choice));self.stats.cache_lookup_us+=t.elapsed().as_micros();
            if let Some(v)=hit {self.stats.cache_hits+=1;if let Some(trace)=&mut self.choice_trace {trace.push((a.clone(),v));}out.push(v);continue;}
            if self.stats.generated_actor_queries>=self.caps.queries as u64 {return Err("actor query cap".into());}
            self.check()?;let t=Instant::now();let q=prepare_actor(a,Deadline::after(self.deadline.saturating_duration_since(Instant::now())))?;
            self.stats.sample_us+=t.elapsed().as_micros();self.stats.generated_actor_queries+=1;
            grow(&mut self.stats.unique_actor_misses_by_level,a.level);self.stats.unique_actor_misses_by_level[a.level]+=1;
            grow(&mut self.stats.inner_worlds_by_level,a.level);self.stats.inner_worlds_by_level[a.level]+=q.worlds.len() as u64;
            self.tables.entry(q.context.decl).or_insert_with(||Tables::new(q.context.decl));
            let choice=self.bounded_query_choice(&q)?;self.stats.bounded_choice_calls+=1;self.stats.completed_actor_queries+=1;
            if self.cache_len()<self.caps.cache_entries {self.stats.cache_key_bytes+=size_of::<ActorSpec>()+a.context.budgets.len()*size_of::<usize>();self.choice_cache.insert(a.clone(),choice);}
            self.stats.cache_entries=self.cache_len();out.push(choice);
            if let Some(trace)=&mut self.choice_trace {trace.push((a.clone(),choice));}
        }Ok(out)
    }
    fn new_bound_node(&mut self,q:&Query,state:State,worlds:Vec<u32>,depth:usize)->Res<Node> {
        if self.bounded_live_rows.checked_add(worlds.len()).ok_or("bounded row overflow")?>self.caps.rows {return Err("bounded retained row cap".into());}
        self.bounded_live_rows+=worlds.len();self.bounded_live_bytes+=size_of::<(u8,Node)>()+worlds.capacity()*size_of::<u32>();
        self.stats.peak_bounded_live_bytes=self.stats.peak_bounded_live_bytes.max(self.bounded_live_bytes);
        self.stats.peak_counted_live_array_bytes=self.stats.peak_counted_live_array_bytes.max(self.live_parent_array_bytes+self.bounded_live_bytes);
        self.stats.bounded_children_created+=1;
        let n=worlds.len() as u64;let (lo,hi,kind)=match state.terminal(q.context.bid) {Some(v)=>(v*n,v*n,Kind::Settled),None=>(0,n,Kind::Fresh)};
        Ok(Node {state,worlds,depth,lo,hi,kind})
    }
    fn expand_bound_node(&mut self,q:&Query,node:&mut Node)->Res<()> {
        self.check()?;if !matches!(node.kind,Kind::Fresh) {return Ok(());}
        if node.depth>=16 {return Err("remaining-ply cap".into());}
        self.stats.bounded_expansions+=1;let seat=node.state.actor();let mut children=Vec::new();
        if seat==q.actor as usize {
            let mask=self.tables[&q.context.decl].legal(node.state,q.hand&!node.state.played);
            if mask==0 {return Err("no legal bounded continuation".into());}
            self.charge(node.worlds.len()*mask.count_ones() as usize)?;
            for tile in solver::mask_bits(mask) {
                self.stats.public_step_calls+=1;let state=self.tables[&q.context.decl].step(node.state,tile);
                children.push((tile,self.new_bound_node(q,state,node.worlds.clone(),node.depth+1)?));
            }node.kind=Kind::Focal(children);
        } else {
            // Deduplicate complete actor views inside this occupied prefix before
            // requesting lower choices. No opponent-world frame enters ActorSpec.
            let mut specs=Vec::new();let mut ids=HashMap::new();let mut demand=Vec::with_capacity(node.worlds.len());let mut masks=Vec::with_capacity(node.worlds.len());
            for &w in &node.worlds {
                let hand=q.worlds[w as usize][seat]&!node.state.played;let mask=self.tables[&q.context.decl].legal(node.state,hand);
                if mask==0 {return Err("no legal bounded continuation".into());}masks.push(mask);
                let id=if mask.count_ones()>1 {if let Nature::Policy(level)=q.nature {
                    grow(&mut self.stats.raw_policy_consultations_by_level,level);self.stats.raw_policy_consultations_by_level[level]+=1;
                    Some(*ids.entry(hand).or_insert_with(||{let id=specs.len();specs.push(ActorSpec {context:q.context.clone(),state:node.state,actor:seat as u8,hand,level});id}))
                }else {None}} else {None};demand.push(id);
            }
            let replies=self.bounded_actor_choices(&specs)?;let hash=if matches!(q.nature,Nature::Native(_)) {self.stats.public_record_calls+=1;record(node.state)}else {0};
            let mut groups:BTreeMap<u8,Vec<u32>>=BTreeMap::new();
            for (i,&w) in node.worlds.iter().enumerate() {
                let mask=masks[i];let tile=if mask.count_ones()==1 {mask.trailing_zeros() as u8} else {match &q.nature {
                    Nature::Native(seeds)=>nth(mask,SplitMix64(seeds[w as usize]^hash).below(mask.count_ones() as u64) as u32),
                    Nature::Tape(t)=>nth(mask,((t[w as usize][node.depth] as u128*mask.count_ones() as u128)>>64) as u32),
                    Nature::Policy(_)=>replies[demand[i].unwrap()],
                }};
                if mask&(1<<tile)==0 {return Err("bounded field reply illegal".into());}groups.entry(tile).or_default().push(w);
            }
            self.charge(node.worlds.len())?;
            for (tile,worlds) in groups {self.stats.public_step_calls+=1;let state=self.tables[&q.context.decl].step(node.state,tile);children.push((tile,self.new_bound_node(q,state,worlds,node.depth+1)?));}
            node.kind=Kind::Nature(children);
        }
        node.fold(q.actor%2==1);Ok(())
    }
    fn refine_bound_node(&mut self,q:&Query,node:&mut Node)->Res<()> {
        self.check()?;self.stats.bounded_refinements+=1;
        if node.lo==node.hi {return Ok(());}
        if matches!(node.kind,Kind::Fresh) {return self.expand_bound_node(q,node);}
        let maximize=q.actor%2==1;
        let children=match &mut node.kind {Kind::Focal(v)|Kind::Nature(v)=>v,_=>return Err("unresolved settled node".into())};
        let idx=if node.state.actor()==q.actor as usize {promising(children,maximize)}else {children.iter().position(|(_,n)|n.lo!=n.hi)}.ok_or("bounded refinement stalled")?;
        self.refine_bound_node(q,&mut children[idx].1)?;node.fold(maximize);Ok(())
    }
    fn bounded_query_choice(&mut self,q:&Query)->Res<u8> {
        validate_prepared(q)?;let saved=(self.bounded_live_rows,self.bounded_live_bytes);
        let result=(|| {
            let mut root=self.new_bound_node(q,q.state,(0..q.worlds.len() as u32).collect(),0)?;self.expand_bound_node(q,&mut root)?;
            let children=match &mut root.kind {Kind::Focal(v)=>v,_=>return Err("actor root is not a focal decision".into())};
            loop {
                self.check()?;let bounds=children.iter().map(|(_,n)|n.bounds()).collect::<Vec<_>>();
                if let Some(i)=certified_index(&bounds,q.actor%2==1) {
                    self.stats.bounded_children_unresolved+=children.iter().map(|(_,n)|n.unresolved()).sum::<u64>();
                    self.stats.bounded_root_unresolved+=children.iter().filter(|(_,n)|n.lo!=n.hi).count() as u64;
                    return Ok(children[i].0);
                }
                let i=promising(children,q.actor%2==1).ok_or("choice bounds stalled")?;self.refine_bound_node(q,&mut children[i].1)?;
            }
        })();
        // Restore on success AND refusal; only completed choices were cached.
        self.bounded_live_rows=saved.0;self.bounded_live_bytes=saved.1;result
    }
}
fn promising(children:&[(u8,Node)],maximize:bool)->Option<usize> {
    let mut best=None;
    for (i,(_,n)) in children.iter().enumerate() {
        if n.lo==n.hi {continue;}
        if best.is_none_or(|b:usize|if maximize {n.hi>children[b].1.hi} else {n.lo<children[b].1.lo}) {best=Some(i);}
    }best
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]fn asymmetric_ties_and_invalid_intervals() {
        assert_eq!(certified_index(&[(2,2),(2,4)],true),None);
        assert_eq!(certified_index(&[(2,2),(2,2)],true),Some(0));
        assert_eq!(certified_index(&[(1,2),(2,2)],true),None);
        assert_eq!(certified_index(&[(0,1),(2,2)],true),Some(1));
        assert_eq!(certified_index(&[(2,2),(0,2)],false),None);
        assert_eq!(certified_index(&[(2,2),(2,2)],false),Some(0));
        assert_eq!(certified_index(&[(2,4),(2,2)],false),None);
        assert_eq!(certified_index(&[(3,4),(2,2)],false),Some(1));
        assert_eq!(certified_index(&[(2,1),(0,0)],true),None);
    }
    #[test]fn every_small_integer_interval_certificate_is_sound() {
        for max in [false,true] {for al in 0..=3 {for ah in al..=3 {for bl in 0..=3 {for bh in bl..=3 {for cl in 0..=3 {for ch in cl..=3 {
            let bs=[(al,ah),(bl,bh),(cl,ch)];if let Some(i)=certified_index(&bs,max) {
                for a in al..=ah {for b in bl..=bh {for c in cl..=ch {let ans=Answer::new(vec![0,7,27],vec![a,b,c],max);assert_eq!(ans.choice,[0,7,27][i]);}}}
            }
        }}}}}}}
    }
}
