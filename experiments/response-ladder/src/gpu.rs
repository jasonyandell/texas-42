//! Optional native Metal harness for portable WGSL/WebGPU fixed-policy lanes.
//! Only the historical Dice field is implemented. CPU owns all integer weights,
//! reductions, policy construction, and bounds. This is not a GPU L2 player.
use crate::mechanics::{Problem, PublicState};
use crate::rollout::{self, LaneTrace, PayoffMatrix};
use std::sync::mpsc;
use std::time::{Duration, Instant};
use wgpu::util::DeviceExt;

pub struct DiceRollout {
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) rules: wgpu::Buffer,
    layout: wgpu::BindGroupLayout,
    payoff: wgpu::ComputePipeline,
    trace: wgpu::ComputePipeline,
    rng: wgpu::ComputePipeline,
    adapter: String,
    /// Device setup, rule construction/upload, and all pipeline creation.
    pub initialization_ms: f64,
}

impl DiceRollout {
    pub fn new() -> Result<Self, String> {
        let started = Instant::now();
        let mut desc = wgpu::InstanceDescriptor::new_without_display_handle();
        desc.backends = wgpu::Backends::METAL;
        let instance = wgpu::Instance::new(desc);
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            ..Default::default()
        })).map_err(|e| format!("Metal adapter: {e}"))?;
        let info = adapter.get_info();
        let adapter_name = format!("{} / {:?}", info.name, info.backend);
        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("fixed-priority rollout"),
            required_limits: wgpu::Limits::default(),
            required_features: wgpu::Features::empty(),
            ..Default::default()
        })).map_err(|e| format!("Metal device: {e}"))?;
        let rules = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("frozen canonical rules"),
            contents: bytemuck::cast_slice(&rollout::rules_table()),
            usage: wgpu::BufferUsages::STORAGE,
        });
        let entries: Vec<_> = (0..3).map(|binding| wgpu::BindGroupLayoutEntry {
            binding, visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: binding < 2 },
                has_dynamic_offset: false, min_binding_size: None,
            }, count: None,
        }).collect();
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("fixed rollout layout"), entries: &entries,
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("fixed rollout pipeline layout"),
            bind_group_layouts: &[Some(&layout)], immediate_size: 0,
        });
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("stackless fixed-priority WGSL"),
            source: wgpu::ShaderSource::Wgsl(include_str!("rollout.wgsl").into()),
        });
        let pipeline = |label, entry_point, trace| device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some(label), layout: Some(&pipeline_layout), module: &shader,
            entry_point: Some(entry_point),
            compilation_options: wgpu::PipelineCompilationOptions {
                constants: &[("TRACE", f64::from(trace))], ..Default::default()
            }, cache: None,
        });
        let payoff = pipeline("payoff matrix", "run", false);
        let trace = pipeline("canonical trace validation", "run", true);
        let rng = pipeline("directed arithmetic validation", "rng_check", false);
        Ok(Self { device, queue, rules, layout, payoff, trace, rng, adapter: adapter_name,
            initialization_ms: started.elapsed().as_secs_f64()*1000.0 })
    }

    pub fn adapter_name(&self) -> &str { &self.adapter }

    /// Includes validation, packing, buffer allocation/upload, dispatch, mapped
    /// readback, and row construction. Each original scenario gets one column.
    pub fn evaluate_dice(&self, problem: &Problem, plans: &[Vec<u8>]) -> Result<PayoffMatrix, String> {
        let (packet, lanes) = self.packet(problem, plans)?;
        if lanes == 0 { return Ok(vec![]); }
        let words = self.dispatch(&packet, lanes, 1, &self.payoff, true)?;
        if words.iter().any(|&x| x > 1) { return Err("unsettled or invalid GPU lane".into()); }
        Ok(words.chunks_exact(problem.scenarios.len()).map(<[u32]>::to_vec).collect())
    }

    /// Diagnostic pipeline emits chronological continuations for stronger
    /// differential tests than payoff-only agreement. Not the timed kernel.
    pub fn evaluate_dice_traces(&self, problem: &Problem, plans: &[Vec<u8>])
        -> Result<Vec<Vec<LaneTrace>>, String>
    {
        let (packet, lanes) = self.packet(problem, plans)?;
        if lanes == 0 { return Ok(vec![]); }
        let words = self.dispatch(&packet, lanes, 32, &self.trace, true)?;
        let mut traces = Vec::with_capacity(lanes);
        for lane in words.chunks_exact(32) {
            if lane[0] > 1 || lane[1] > 28 { return Err("unsettled or invalid GPU trace".into()); }
            let plays = lane[2..2+lane[1] as usize].iter().map(|&t| t as u8).collect();
            traces.push(LaneTrace { payoff: lane[0], plays });
        }
        Ok(traces.chunks_exact(problem.scenarios.len()).map(<[LaneTrace]>::to_vec).collect())
    }

    fn packet(&self, problem: &Problem, plans: &[Vec<u8>]) -> Result<(Vec<u32>, usize), String> {
        rollout::validate(problem, plans)?;
        if problem.field_revision != "historical-dice-v1" { return Err("GPU requires historical-dice-v1".into()); }
        let n = problem.scenarios.len();
        let lanes = n.checked_mul(plans.len()).ok_or("lane count overflow")?;
        let _ = u32::try_from(lanes).map_err(|_| "too many GPU lanes")?;
        let root = &problem.root;
        let mut words = vec![0; 16];
        words[0] = walt::rules::Decl::ALL.iter().position(|&d| d == problem.decl).unwrap() as u32;
        words[1] = problem.bid.into(); words[2] = problem.viewer.into();
        words[3] = problem.hand.count_ones(); words[4] = root.played;
        words[5] = pack_plays(root); words[6] = root.plays.len() as u32;
        words[7] = root.leader.into(); words[8] = root.banked_t1.into(); words[9] = root.banked_t0.into();
        words[10] = n as u32; words[11] = plans.len() as u32;
        words[12] = (lanes.div_ceil(64).min(self.device.limits().max_compute_workgroups_per_dimension as usize)*64) as u32;
        for s in &problem.scenarios {
            words.extend(s.hands);
            words.push(s.tape as u32); words.push((s.tape >> 32) as u32);
        }
        for plan in plans {
            words.extend(plan.iter().map(|&t| u32::from(t)));
            words.resize(words.len()+8-plan.len(), 0);
        }
        Ok((words, lanes))
    }

    fn dispatch(&self, packet: &[u32], lanes: usize, stride: usize,
        pipeline: &wgpu::ComputePipeline, rectangular: bool) -> Result<Vec<u32>, String>
    {
        let limits = self.device.limits();
        let input_size = packet.len().checked_mul(4).ok_or("input size overflow")? as u64;
        let output_size = lanes.checked_mul(stride).and_then(|n| n.checked_mul(4)).ok_or("output size overflow")? as u64;
        if input_size > u64::from(limits.max_storage_buffer_binding_size)
            || output_size > u64::from(limits.max_storage_buffer_binding_size)
            || output_size > limits.max_buffer_size || input_size > limits.max_buffer_size
        { return Err("batch exceeds device buffer limits; split the plan/scenario batch".into()); }
        let groups = lanes.div_ceil(64);
        let x = groups.min(limits.max_compute_workgroups_per_dimension as usize);
        let y = groups.div_ceil(x);
        if y > limits.max_compute_workgroups_per_dimension as usize || (!rectangular && y > 1) {
            return Err("batch exceeds dispatch limits".into());
        }
        let input = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("root, original scenarios, priority plans"),
            contents: bytemuck::cast_slice(packet), usage: wgpu::BufferUsages::STORAGE,
        });
        let output = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("rollout matrix"), size: output_size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC, mapped_at_creation: false,
        });
        let read = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("rollout readback"), size: output_size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ, mapped_at_creation: false,
        });
        let bind = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("fixed-policy batch"), layout: &self.layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: input.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: self.rules.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 2, resource: output.as_entire_binding() },
            ],
        });
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("fixed-priority rollout") });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: Some("one physical path per lane"), timestamp_writes: None });
            pass.set_pipeline(pipeline); pass.set_bind_group(0, &bind, &[]);
            pass.dispatch_workgroups(x as u32, y as u32, 1);
        }
        encoder.copy_buffer_to_buffer(&output, 0, &read, 0, output_size);
        let submission = self.queue.submit([encoder.finish()]);
        let (tx, rx) = mpsc::channel();
        read.slice(..).map_async(wgpu::MapMode::Read, move |result| { let _ = tx.send(result); });
        self.device.poll(wgpu::PollType::Wait { submission_index: Some(submission), timeout: Some(Duration::from_secs(60)) })
            .map_err(|e| format!("GPU wait: {e}"))?;
        rx.recv_timeout(Duration::from_secs(1)).map_err(|e| format!("map callback: {e}"))?
            .map_err(|e| format!("readback map: {e}"))?;
        let view = read.slice(..).get_mapped_range().map_err(|e| format!("readback view: {e}"))?;
        let words = bytemuck::cast_slice::<u8, u32>(&view).to_vec();
        drop(view); read.unmap();
        Ok(words)
    }

    /// Directed arithmetic probe: (mix(seed), historical below(n), record_hash).
    pub fn arithmetic_vectors(&self, vectors: &[(u64, u32, PublicState)]) -> Result<Vec<(u64,u32,u64)>, String> {
        if vectors.is_empty() { return Ok(vec![]); }
        let mut packet = Vec::with_capacity(vectors.len()*8);
        for (seed, n, public) in vectors {
            if !(1..=28).contains(n) || public.plays.len() > 3 || public.plays.iter().any(|&t| t >= 28) || public.leader >= 4 {
                return Err("invalid arithmetic vector".into());
            }
            packet.extend([*seed as u32, (seed >> 32) as u32, *n, public.played,
                pack_plays(public), public.plays.len() as u32, public.leader.into(), 0]);
        }
        Ok(self.dispatch(&packet, vectors.len(), 8, &self.rng, false)?.chunks_exact(8)
            .map(|v| (u64::from(v[0]) | (u64::from(v[1]) << 32), v[2], u64::from(v[3]) | (u64::from(v[4]) << 32))).collect())
    }
}

fn pack_plays(public: &PublicState) -> u32 {
    public.plays.iter().enumerate().fold(0, |bits, (i, &tile)| bits | (u32::from(tile) << (5*i)))
}
