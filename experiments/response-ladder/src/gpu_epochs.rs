//! GPU policy evolution with exact, lawful CPU field-query resolution.
//! The device survives decisions. Query caches and deadline-bearing minds do not.
use crate::{core::PolicyEvaluator, gpu::DiceRollout, mechanics::*, policy::Policy};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::mpsc, time::{Duration, Instant}};
use walt::rules::{Context, Domino};
use wgpu::util::DeviceExt;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EpochStats {
    pub batches: u64, pub completed_batches: u64, pub cancelled_batches: u64,
    pub epochs: u64, pub lanes_submitted: u64, pub completed_lanes: u64,
    pub field_requests: u64, pub unique_field_queries: u64,
    pub modeled_unique_queries: u64,
    pub device_ms: f64, pub field_ms: f64, pub elapsed_ms: f64,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EpochTrace { pub payoff: u64, pub continuation: Vec<u8> }

// Two fixed-width phase records per actor. Legacy actors repeat the same
// program, so the public leading/following branch changes no old choices.
fn pack_compiled_actor(actor: &crate::compiled::Actor) -> Result<[u32; 8], String> {
    actor.validate()?;
    let mut record = [0; 8];
    for (phase, leading) in [true, false].into_iter().enumerate() {
        let clauses = actor.clauses_for_phase(leading);
        record[phase * 4] = clauses.len() as u32;
        for (i, &clause) in clauses.iter().enumerate() {
            record[phase * 4 + 1 + i] = clause as u32;
        }
    }
    Ok(record)
}

pub struct EpochEvaluator {
    gpu: DiceRollout,
    layout: wgpu::BindGroupLayout,
    pipeline: wgpu::ComputePipeline,
    pub stats: EpochStats,
    pub initialization_ms: f64,
}
impl EpochEvaluator {
    pub fn new() -> Result<Self,String> {
        let start=Instant::now();
        let gpu=DiceRollout::new()?;
        let entries:Vec<_>=(0..4).map(|binding|wgpu::BindGroupLayoutEntry {
            binding,visibility:wgpu::ShaderStages::COMPUTE,
            ty:wgpu::BindingType::Buffer { ty:wgpu::BufferBindingType::Storage{read_only:binding!=2},
                has_dynamic_offset:false,min_binding_size:None },count:None }).collect();
        let layout=gpu.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {label:Some("modeled field epochs"),entries:&entries});
        let pl=gpu.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label:Some("epoch layout"),bind_group_layouts:&[Some(&layout)],immediate_size:0});
        let shader=gpu.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label:Some("resumable lawful policies"),source:wgpu::ShaderSource::Wgsl(include_str!("epochs.wgsl").into())});
        let pipeline=gpu.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label:Some("field pause and resume"),layout:Some(&pl),module:&shader,entry_point:Some("epoch"),
            compilation_options:Default::default(),cache:None});
        Ok(Self{gpu,layout,pipeline,stats:EpochStats::default(),initialization_ms:start.elapsed().as_secs_f64()*1000.})
    }
    pub fn adapter_name(&self)->&str {self.gpu.adapter_name()}
    pub fn reset_stats(&mut self) {self.stats=EpochStats::default();}
    pub fn traces(&mut self,p:&Problem,policies:&[Policy],field:&mut dyn Field,budget:&mut Budget)
        ->Result<Option<Vec<Vec<EpochTrace>>>,String> {
        let started=Instant::now(); self.stats.batches+=1;
        let result=self.run(p,policies,field,budget);
        self.stats.elapsed_ms+=started.elapsed().as_secs_f64()*1000.;
        match &result { Ok(Some(_))=>{self.stats.completed_batches+=1;self.stats.completed_lanes+=(p.scenarios.len()*policies.len())as u64;},
            Ok(None)=>self.stats.cancelled_batches+=1, _=>{} }
        result
    }
    fn run(&mut self,p:&Problem,policies:&[Policy],field:&mut dyn Field,budget:&mut Budget)
        ->Result<Option<Vec<Vec<EpochTrace>>>,String> {
        p.validate()?;check_field(p,field)?;
        if budget.exhausted(){return Ok(None);}
        if policies.is_empty(){return Ok(Some(vec![]));}
        let count=p.scenarios.len().checked_mul(policies.len()).ok_or("lane count overflow")?;
        let limits=self.gpu.device.limits();
        let x=count.div_ceil(64).min(limits.max_compute_workgroups_per_dimension as usize);
        let y=count.div_ceil(64).div_ceil(x);
        if y>limits.max_compute_workgroups_per_dimension as usize {return Err("epoch dispatch exceeds device limits".into());}
        let mut packet=vec![0u32;20];
        packet[0]=walt::rules::Decl::ALL.iter().position(|&d|d==p.decl).unwrap()as u32;
        packet[1]=p.bid.into();packet[2]=p.viewer.into();packet[4]=p.root.played;
        packet[5]=p.root.plays.iter().enumerate().fold(0,|m,(i,&t)|m|(u32::from(t)<<(5*i)));
        packet[6]=p.root.plays.len()as u32;packet[7]=p.root.leader.into();
        packet[8]=p.root.banked_t1.into();packet[9]=p.root.banked_t0.into();
        packet[10]=u32::try_from(p.scenarios.len()).map_err(|_|"too many scenarios")?;
        packet[11]=u32::try_from(policies.len()).map_err(|_|"too many policies")?;packet[12]=(x*64)as u32;
        packet[16..20].copy_from_slice(&p.root.voids);
        for s in &p.scenarios {packet.extend(s.hands);packet.extend([s.tape as u32,(s.tape>>32)as u32]);}
        let bases=packet.len();packet.resize(bases+policies.len()*12,0);
        let mut tail_actors = Vec::with_capacity(policies.len());
        for (i,policy) in policies.iter().enumerate() {
            if policy.viewer!=p.viewer || policy.decisions.windows(2).any(|w|w[0].history>=w[1].history) {
                return Err("policy viewer/history ordering mismatch".into());
            }
            policy.validate()?;
            tail_actors.push(policy.compiled_tail.clone());
            let b=bases+i*12;let mut seen=0u32;let mut priority=Vec::new();
            for &t in &policy.priority {
                if t<28 && p.hand&(1<<t)!=0 && seen&(1<<t)==0 {priority.push(t);seen|=1<<t;}
            }
            packet[b]=priority.len()as u32;
            for (j,&t) in priority.iter().enumerate(){packet[b+1+j]=t.into();}
            packet[b+8]=u32::try_from(packet.len()).map_err(|_|"policy packet too large")?;
            for d in &policy.decisions {
                if let Some(h)=d.history.strip_prefix(p.root.history.as_slice()) {
                    if h.len()>28 || h.iter().any(|&t|t>=28) {return Err("invalid policy history".into());}
                    packet[b+9]+=1;
                    let mut record=[0u32;32];record[0]=h.len()as u32;record[1]=d.action.into();
                    for (j,&t) in h.iter().enumerate(){record[2+j]=t.into();}packet.extend(record);
                }
            }
        }
        // A nonzero header word 14 names an appended immutable compiled-field
        // block. Zero preserves the legacy CPU-query epoch protocol exactly.
        let compiled_specs = field
            .compiled_actors()
            .map(|actors| {
                let mut packed = [[0u32; 8]; 4];
                for (seat, actor) in actors.iter().enumerate() {
                    packed[seat] = pack_compiled_actor(actor)?;
                }
                Ok::<_, String>(packed)
            })
            .transpose()?;
        if let Some(actors) = compiled_specs {
            let offset =
                u32::try_from(packet.len()).map_err(|_| "compiled actor packet offset overflow")?;
            packet[14] = offset;
            for actor in actors {
                packet.extend(actor);
            }
            // Keep these masks in packet space so the frozen walt rule table
            // offsets remain byte-for-byte unchanged for legacy epochs.
            packet.extend(Domino::ALL.into_iter().map(|tile| p.decl.threat(tile).bits()));
            for context in Context::ALL {
                packet.extend(
                    Domino::ALL
                        .into_iter()
                        .map(|tile| p.decl.beats(context, tile).bits()),
                );
            }
        }
        let mut tail_offsets: Vec<(crate::compiled::Actor, u32)> = Vec::new();
        for (index, actor) in tail_actors.into_iter().enumerate() {
            let Some(actor) = actor else { continue; };
            let offset = if let Some((_, offset)) =
                tail_offsets.iter().find(|(known, _)| known == &actor)
            {
                *offset
            } else {
                let offset = u32::try_from(packet.len())
                    .map_err(|_| "compiled tail packet offset overflow")?;
                let packed = [pack_compiled_actor(&actor)?; 4];
                for record in packed {
                    packet.extend(record);
                }
                packet.extend(Domino::ALL.into_iter().map(|tile| p.decl.threat(tile).bits()));
                for context in Context::ALL {
                    packet.extend(
                        Domino::ALL
                            .into_iter()
                            .map(|tile| p.decl.beats(context, tile).bits()),
                    );
                }
                tail_offsets.push((actor, offset));
                offset
            };
            packet[bases + index * 12 + 10] = offset;
        }
        let bytes=count.checked_mul(40*4).ok_or("lane storage overflow")?as u64;
        if bytes>u64::from(limits.max_storage_buffer_binding_size) || (packet.len()*4)as u64>u64::from(limits.max_storage_buffer_binding_size) {
            return Err("epoch batch exceeds storage limits".into());
        }
        let input=self.gpu.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {label:Some("policy batch input"),contents:bytemuck::cast_slice(&packet),usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_DST});
        let lanes=self.gpu.device.create_buffer(&wgpu::BufferDescriptor {label:Some("original scenario lanes"),size:bytes,usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_SRC,mapped_at_creation:false});
        let replies=self.gpu.device.create_buffer(&wgpu::BufferDescriptor {label:Some("lawful field answers"),size:(count*4)as u64,usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});
        let read=self.gpu.device.create_buffer(&wgpu::BufferDescriptor {label:Some("epoch readback"),size:bytes,usage:wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::MAP_READ,mapped_at_creation:false});
        let bind=self.gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {label:Some("epoch batch"),layout:&self.layout,entries:&[
            wgpu::BindGroupEntry{binding:0,resource:input.as_entire_binding()},wgpu::BindGroupEntry{binding:1,resource:self.gpu.rules.as_entire_binding()},
            wgpu::BindGroupEntry{binding:2,resource:lanes.as_entire_binding()},wgpu::BindGroupEntry{binding:3,resource:replies.as_entire_binding()}]});
        // Deliberately local. Full public history and tape make this safe for any
        // declared deterministic Field, including a history-dependent test field.
        let mut cache:HashMap<(PublicState,u32,u64),u8>=HashMap::new();
        self.stats.lanes_submitted+=count as u64;
        for epoch in 0..=28u32 {
            if budget.tick().is_none(){return Ok(None);}
            self.gpu.queue.write_buffer(&input,13*4,bytemuck::bytes_of(&epoch));
            let device_start=Instant::now();
            let mut enc=self.gpu.device.create_command_encoder(&wgpu::CommandEncoderDescriptor{label:Some("advance to field frontier")});
            {let mut pass=enc.begin_compute_pass(&wgpu::ComputePassDescriptor{label:Some("lawful policy lanes"),timestamp_writes:None});
                pass.set_pipeline(&self.pipeline);pass.set_bind_group(0,&bind,&[]);pass.dispatch_workgroups(x as u32,y as u32,1);}
            enc.copy_buffer_to_buffer(&lanes,0,&read,0,bytes);self.gpu.queue.submit([enc.finish()]);
            self.stats.epochs+=1;
            let (tx,rx)=mpsc::channel();read.slice(..).map_async(wgpu::MapMode::Read,move|v|{let _=tx.send(v);});
            loop {
                self.gpu.device.poll(wgpu::PollType::Poll).map_err(|e|format!("epoch device poll: {e}"))?;
                match rx.try_recv() {
                    Ok(result)=>{result.map_err(|e|format!("epoch map: {e}"))?;break;},
                    Err(mpsc::TryRecvError::Disconnected)=>return Err("epoch map channel closed".into()),
                    Err(mpsc::TryRecvError::Empty)=>{},
                }
                if budget.exhausted(){read.unmap();self.stats.device_ms+=device_start.elapsed().as_secs_f64()*1000.;return Ok(None);}
                std::thread::sleep(Duration::from_micros(50));
            }
            let view=read.slice(..).get_mapped_range().map_err(|e|e.to_string())?;
            let words=bytemuck::cast_slice::<u8,u32>(&view).to_vec();drop(view);read.unmap();
            self.stats.device_ms+=device_start.elapsed().as_secs_f64()*1000.;
            if budget.exhausted(){return Ok(None);}
            check_field(p,field)?;
            if words.chunks_exact(40).all(|l|l[0]==2) {
                let traces:Result<Vec<_>,String>=words.chunks_exact(40).map(|l| {
                    let state=public_state(p,l)?;
                    Ok(EpochTrace{payoff:state.payoff(p.bid,p.viewer).ok_or("GPU terminal lacks payoff")?,continuation:state.history[p.root.history.len()..].to_vec()})
                }).collect();
                let traces=traces?;
                if budget.exhausted(){return Ok(None);}
                return Ok(Some(traces.chunks_exact(p.scenarios.len()).map(<[EpochTrace]>::to_vec).collect()));
            }
            let field_start=Instant::now();
            let mut answers=vec![u32::MAX;count];
            let mut pending: HashMap<(PublicState,u32,u64),usize> = HashMap::new();
            let mut queries=Vec::new(); let mut targets:Vec<Vec<usize>>=Vec::new();
            for (id,l) in words.chunks_exact(40).enumerate() {
                if l[0]==2{continue;}
                if l[0]!=1{return Err("GPU lane is invalid/unsettled".into());}
                if budget.exhausted(){self.stats.field_ms+=field_start.elapsed().as_secs_f64()*1000.;return Ok(None);}
                let state=public_state(p,l)?;let seat=state.actor();
                if seat==p.viewer{return Err("GPU requested a focal decision from field".into());}
                let scenario=&p.scenarios[id%p.scenarios.len()];
                let hand=scenario.hands[seat as usize]&!state.played;
                self.stats.field_requests+=1;
                let key=(state.clone(),hand,scenario.tape);
                if let Some(&tile)=cache.get(&key) { answers[id]=tile.into(); }
                else if let Some(&at)=pending.get(&key) { targets[at].push(id); }
                else {
                    pending.insert(key,queries.len()); targets.push(vec![id]);
                    queries.push(OwnedFieldQuery{decl:p.decl,bid:p.bid,seat,hand,public:state,tape:scenario.tape});
                }
            }
            self.stats.unique_field_queries+=queries.len() as u64;
            check_field(p,field)?;
            let solved=field.choose_batch(&queries,budget);
            self.stats.modeled_unique_queries+=field.last_batch_modeled_queries().unwrap_or(0) as u64;
            check_field(p,field)?;
            let Some(solved)=solved else {self.stats.field_ms+=field_start.elapsed().as_secs_f64()*1000.;return Ok(None);};
            if solved.len()!=queries.len(){return Err("field returned partial query batch".into());}
            for ((q,ids),tile) in queries.into_iter().zip(targets).zip(solved) {
                if tile>=28 || q.public.legal(q.decl,q.hand)&(1<<tile)==0{return Err("field supplied illegal GPU continuation".into());}
                cache.insert((q.public,q.hand,q.tape),tile);
                for id in ids {answers[id]=tile.into();}
            }
            self.stats.field_ms+=field_start.elapsed().as_secs_f64()*1000.;
            if budget.exhausted(){return Ok(None);}
            self.gpu.queue.write_buffer(&replies,0,bytemuck::cast_slice(&answers));
        }
        Err("GPU epochs exceeded finite physical game".into())
    }
}
fn check_field(p:&Problem,f:&dyn Field)->Result<(),String> {
    if p.field_revision==f.revision(){Ok(())}else{Err("epoch field revision changed".into())}
}
fn public_state(p:&Problem,l:&[u32])->Result<PublicState,String> {
    if l[3]>3 || l[4]>=4 || l[5]+l[6]>42 || l[7]>28 {return Err("invalid GPU public state".into());}
    let mut history=p.root.history.clone();
    for &t in &l[12..12+l[7]as usize] {if t>=28{return Err("invalid GPU trace tile".into());}history.push(t as u8);}
    Ok(PublicState{played:l[1],plays:(0..l[3]).map(|i|((l[2]>>(5*i))&31)as u8).collect(),leader:l[4]as u8,
        banked_t1:l[5]as u8,banked_t0:l[6]as u8,voids:l[8..12].try_into().unwrap(),history})
}
impl PolicyEvaluator for EpochEvaluator {
    fn prices(&mut self,p:&Problem,policies:&[Policy],field:&mut dyn Field,budget:&mut Budget)->Result<Option<Vec<u64>>,String> {
        let Some(rows)=self.traces(p,policies,field,budget)? else{return Ok(None);};
        let values=rows.into_iter().map(|row|row.iter().zip(&p.scenarios).map(|(v,s)|v.payoff*s.weight).sum()).collect();
        if budget.exhausted(){return Ok(None);}Ok(Some(values))
    }
}
