//! FFI to the compiled delta-expectimax ladder (Jason's walt42x `turbo.cpp`, copied verbatim
//! into `csrc/`). Level 1 there = best response to uniform random legal play over sampled
//! void-consistent worlds, which is the same quantity Walt's level-0 modeled mind estimates
//! (declarer-make probability per legal play); `delta` controls whether opponent randomness
//! is branched exactly (child weight >= delta) or sampled (delta = 1, Walt-like).
use std::collections::HashMap;
use std::ffi::CStr;
use std::os::raw::{c_char, c_double, c_int, c_void};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use walt::rules::Seat;
use walt::solver::{net_hook::NetPolicy, Key};

extern "C" {
    fn walt_create(trump: c_int, ns: *const c_int, levels: c_int, delta: c_double, seed: u64, reuse: c_int, cache: c_int, cache_cap: u64, fiber_cap: u64, seconds: c_double) -> *mut c_void;
    fn walt_destroy(c: *mut c_void);
    fn walt_configure(h: *mut c_void, indexed: c_int, pruning: c_int, tables: c_int, node_cache: c_int) -> c_int;
    fn walt_action(h: *mut c_void, level: c_int, a: *const u64, hand: u32, values: c_int, scores: *mut c_double) -> c_int;
    fn walt_error(h: *mut c_void) -> *const c_char;
    fn walt_stats(h: *mut c_void, out: *mut u64);
    fn walt_metrics(h: *mut c_void, out: *mut u64);
    fn walt_set_enumerate(h: *mut c_void, cap: u64);
    fn walt_set_bid(h: *mut c_void, bid: c_int);
    fn walt_last_worlds(h: *mut c_void) -> u64;
    fn walt_set_samples(h: *mut c_void, level: c_int, n: c_int) -> c_int;
    fn walt_set_policy_hook(h: *mut c_void, level: c_int, f: Option<unsafe extern "C" fn(*mut c_void, c_int, *const u64, u32, u32) -> c_int>, ctx: *mut c_void);
    fn walt_set_leaf_hook(h: *mut c_void, f: Option<unsafe extern "C" fn(*mut c_void, c_int, *const u64, u32, u32) -> c_int>, ctx: *mut c_void);
    fn walt_set_leaf_value_hook(h: *mut c_void, f: Option<unsafe extern "C" fn(*mut c_void, c_int, *const u64, u32, u32, *mut c_double) -> c_int>, ctx: *mut c_void, tricks: c_int);
    fn walt_set_world_hook(h: *mut c_void, f: Option<unsafe extern "C" fn(*mut c_void, *const u64, u32, *const u32, c_int, *mut c_double) -> c_int>, ctx: *mut c_void, resample_n: c_int);
    fn walt_rank(a: *const u64, hand: u32, rank: u64, out: *mut u32, total: *mut u64) -> c_int;
}

pub const FIELD_SEED: u64 = 42;

pub struct Engine { handle: *mut c_void, pub levels: usize }
unsafe impl Send for Engine {}
impl Drop for Engine { fn drop(&mut self) { unsafe { walt_destroy(self.handle) } } }
impl Engine {
    /// `ns` is bottom rung first (inner, then outer). Addressed randomness + exact bounds on.
    pub fn new(trump: usize, ns: &[usize], delta: f64, seed: u64, fiber_cap: u64) -> Engine { Engine::with_enumerate(trump, ns, delta, seed, fiber_cap, 0) }
    /// `enumerate_cap > 0`: when the void-consistent support has at most that many worlds, use all of them once.
    pub fn with_enumerate(trump: usize, ns: &[usize], delta: f64, seed: u64, fiber_cap: u64, enumerate_cap: u64) -> Engine { Engine::with_caps(trump, ns, delta, seed, fiber_cap, enumerate_cap, 50_000) }
    /// Same engine with an explicit policy-cache capacity (0 = no policy cache). Cached results are deterministic
    /// functions of the query key (addressed randomness salted by key and configuration), so the capacity changes
    /// speed only, never an answer. Measured in log2 (40/8): 55 hits in 17M level-1 queries, so the teacher runs with 0.
    /// LAD6: the contract this engine searches (default 30). Terminal: bidding team >= bid, or defenders > 42 - bid.
    /// Worlds the top level used in its most recent query (the sample size, or the support size when enumerated).
    pub fn last_worlds(&self) -> u64 { unsafe { walt_last_worlds(self.handle) } }
    /// VARIANT (outer schedule): change the number of sampled worlds at `level` for the next queries.
    pub fn set_samples(&self, level: usize, n: usize) { assert_eq!(unsafe { walt_set_samples(self.handle, level as c_int, n as c_int) }, 0); }
    pub fn set_bid(&self, bid: u8) { assert!((30..=42).contains(&bid)); unsafe { walt_set_bid(self.handle, bid as c_int) } }
    pub fn with_caps(trump: usize, ns: &[usize], delta: f64, seed: u64, fiber_cap: u64, enumerate_cap: u64, cache_cap: u64) -> Engine {
        let ns: Vec<c_int> = ns.iter().map(|&n| n as c_int).collect();
        let h = unsafe { walt_create(trump as c_int, ns.as_ptr(), ns.len() as c_int, delta, seed, 0, (cache_cap > 0) as c_int, cache_cap, fiber_cap, 1e9) };
        assert!(!h.is_null(), "walt_create rejected the configuration");
        assert_eq!(unsafe { walt_configure(h, 1, 1, 1, 1) }, 0);
        if enumerate_cap > 0 { unsafe { walt_set_enumerate(h, enumerate_cap) } }
        Engine { handle: h, levels: ns.len() }
    }
    pub fn action(&self, level: usize, p: &[u64; 19], hand: u32, values: bool) -> Result<(u8, [f64; 28]), String> {
        let mut scores = [f64::NAN; 28];
        let t = unsafe { walt_action(self.handle, level as c_int, p.as_ptr(), hand, values as c_int, scores.as_mut_ptr()) };
        if t < 0 { return Err(unsafe { CStr::from_ptr(walt_error(self.handle)) }.to_string_lossy().into_owned()); }
        Ok((t as u8, scores))
    }
    /// Per level: queries, cache_hits, forced, nodes, fiber_visits, peak, sampled_deals, sample_cache_hits.
    /// pruned, action_cuts, pattern hits, pattern misses, pattern size, deduplicated, leaf-memo calls, leaf-memo hits, leaf-memo clears, leaf-memo entries.
    pub fn metrics(&self) -> [u64; 10] { let mut out = [0u64; 10]; unsafe { walt_metrics(self.handle, out.as_mut_ptr()) }; out }
    pub fn stats(&self) -> Vec<[u64; 8]> {
        let mut out = vec![0u64; 8 * self.levels];
        unsafe { walt_stats(self.handle, out.as_mut_ptr()) };
        out.chunks(8).map(|c| { let mut a = [0u64; 8]; a.copy_from_slice(c); a }).collect()
    }
}

// FORK-ONLY (speed): one thread-local Key reused across callbacks (its `plays` Vec keeps its capacity), so the
// callbacks allocate nothing after the first call. Field values are exactly what a fresh Key would carry.
thread_local! { static KEY: std::cell::RefCell<Key> = std::cell::RefCell::new(Key { played: 0, leader: 0, plays: Vec::with_capacity(4), banked_t1: 0, banked_t0: 0, voids: None, alive: 0 }); }
/// Runs `f` on the Key frame of turbo's packed public state `a` and the acting seat.
unsafe fn with_key<R>(a: *const u64, f: impl FnOnce(&Key, Seat) -> R) -> R {
    let a = std::slice::from_raw_parts(a, 19); let len = a[2] as usize;
    let seat = Seat::from_index((a[1] as usize + len) % 4).unwrap();
    KEY.with(|k| {
        let mut key = k.borrow_mut();
        key.played = a[0] as u32; key.leader = a[1] as u8; key.plays.clear(); key.plays.extend((0..len).map(|i| a[6 + i] as u8));
        key.banked_t1 = a[3] as u8; key.banked_t0 = a[4] as u8; key.voids = Some([a[10] as u32, a[11] as u32, a[12] as u32, a[13] as u32]); key.alive = 0;
        f(&key, seat)
    })
}
/// Callback: the modeled policy at `hook_level` is a distilled net (whole-decision policy).
unsafe extern "C" fn net_policy_cb(ctx: *mut c_void, _level: c_int, a: *const u64, hand: u32, legal: u32) -> c_int {
    let net: &crate::net::Net = &*(ctx as *const crate::net::Net);
    with_key(a, |key, seat| NetPolicy::choose(net, key, seat, hand, legal) as c_int)
}
/// VARIANT (inner sampling, Jason 2026-10-08): the modeled others play a sample from the net's pmake vector instead of its
/// argmax: tile t with weight exp((v_t - v_best) / tau), v = sigmoid(score) for the declaring side and 1 - that for the
/// other, over the legal tiles. tau is in make-probability units (tau = .02: a tile .02 worse has weight e^-1). The draw is
/// a hash of the public state and the hand, so the same state in the same world always plays the same tile (reproducible
/// h2h) while different worlds resolve near-ties differently.
pub struct SampledNet { pub net: &'static crate::net::Net, pub tau: f64 }
unsafe extern "C" fn net_policy_sampled_cb(ctx: *mut c_void, _level: c_int, a: *const u64, hand: u32, legal: u32) -> c_int {
    let sn: &SampledNet = &*(ctx as *const SampledNet);
    with_key(a, |key, seat| {
        let sc = sn.net.scores(key, seat, hand, legal); let maximize = seat.index() % 2 == 1;
        let mut v = [0f64; 28]; let mut best = f64::NEG_INFINITY;
        for t in 0..28 { if legal >> t & 1 == 1 { let p = 1.0 / (1.0 + (-(sc[t] as f64)).exp()); v[t] = if maximize { p } else { 1.0 - p }; best = best.max(v[t]); } }
        let mut w = [0f64; 28]; let mut tot = 0f64;
        for t in 0..28 { if legal >> t & 1 == 1 { w[t] = ((v[t] - best) / sn.tau).exp(); tot += w[t]; } }
        let mut h = walt::solver::SplitMix64(key.played as u64 ^ ((key.leader as u64) << 32) ^ ((hand as u64) << 36) ^ key.plays.iter().fold(0x9E37u64, |m, &p| m.wrapping_mul(1_000_003) ^ p as u64) ^ ((key.banked_t1 as u64) << 48) ^ ((key.banked_t0 as u64) << 56));
        let u = h.below(1 << 30) as f64 / (1u64 << 30) as f64 * tot; let mut acc = 0f64;
        for t in 0..28 { if legal >> t & 1 == 1 { acc += w[t]; if u < acc { return t as c_int; } } }
        (31 - legal.leading_zeros()) as c_int
    })
}
/// Callback (VARIANT, see `set_leaf_value_net`): the net's declarer-make estimate for the state, taken as the score of
/// the tile the net itself would choose for the actor (max for the declaring side, min for the other), as the
/// depth-limited leaf value of a level-1 search.
unsafe extern "C" fn net_value_cb(ctx: *mut c_void, _level: c_int, a: *const u64, hand: u32, legal: u32, out: *mut c_double) -> c_int {
    let net: &crate::net::Net = &*(ctx as *const crate::net::Net);
    with_key(a, |key, seat| {
        let s = net.scores(key, seat, hand, legal); let maximize = seat.index() % 2 == 1; let mut best: Option<f32> = None;
        for t in 0..28 { if legal >> t & 1 == 1 { let v = s[t]; best = Some(match best { None => v, Some(b) => if maximize { if v > b { v } else { b } } else if v < b { v } else { b } }); } }
        *out = best.unwrap_or(0.0) as f64; 0
    })
}
/// VARIANT (belief-weighted worlds): the belief head and the tempering exponent for the top-level world weights.
pub struct WorldBelief { pub net: crate::belief::BeliefNet, pub alpha: f64, pub ipf: usize }
/// Callback: normalised importance weight per sampled world, w_i ∝ exp(alpha * Σ_t [ln P_belief(holder_i(t)) − ln P_uniform(holder_i(t))])
/// over the unseen tiles (P_uniform = exact marginal of the uniform void-consistent support); records ESS/N by trick.
unsafe extern "C" fn world_weight_cb(ctx: *mut c_void, a: *const u64, hand: u32, worlds: *const u32, n: c_int, out: *mut c_double) -> c_int {
    let wb: &WorldBelief = &*(ctx as *const WorldBelief);
    let n = n as usize; let ws = std::slice::from_raw_parts(worlds, 4 * n); let out = std::slice::from_raw_parts_mut(out, n);
    with_key(a, |key, seat| {
        use crate::belief::*;
        let fr = frame(key, seat, hand); let pb = wb.net.probs(key, seat, hand, &fr); let pu = uniform_marginals(&fr); let r = if wb.ipf > 0 { ipf_log_tilt(&pb, &pu, &fr, wb.ipf, wb.alpha) } else { log_ratio(&pb, &pu, &fr, wb.alpha) };
        let mut mx = f64::NEG_INFINITY;
        for i in 0..n { let w = [ws[4 * i], ws[4 * i + 1], ws[4 * i + 2], ws[4 * i + 3]]; out[i] = world_log_weight(&r, &fr, &w); mx = mx.max(out[i]); }
        let mut s = 0.0; for x in out.iter_mut() { *x = (*x - mx).exp(); s += *x; }
        let mut s2 = 0.0; for x in out.iter_mut() { *x /= s; s2 += *x * *x; }
        let ess = 1.0 / s2 / n as f64; let trick = (key.played.count_ones() as usize / 4).min(6);
        let mut g = ESS.lock().unwrap(); g[trick][0] += 1.0; g[trick][1] += ess; g[trick][2] = g[trick][2].min(ess);
        0
    })
}
impl Engine {
    /// VARIANT: weight (resample_n == 0) or resample to `resample_n` worlds (resample_n > 0) the top-level sampled worlds
    /// by the belief; see `hookw` in turbo.cpp. The belief must outlive the engine (leaked by the caller).
    pub fn set_world_belief(&self, wb: &'static WorldBelief, resample_n: usize) {
        unsafe { walt_set_world_hook(self.handle, Some(world_weight_cb), wb as *const _ as *mut c_void, resample_n as c_int) }
    }
    /// Modeled others at `level` (1 = the rung below a level-2 search) play `net` instead of the sampled search.
    /// The net must outlive the engine (it is leaked by the caller for the engine's lifetime).
    pub fn set_net_policy(&self, level: usize, net: &'static crate::net::Net) {
        unsafe { walt_set_policy_hook(self.handle, level as c_int, Some(net_policy_cb), net as *const _ as *mut c_void) }
    }
    /// VARIANT (inner sampling): as `set_net_policy`, sampling from the pmake vector at temperature `sn.tau` (see `SampledNet`).
    pub fn set_net_policy_sampled(&self, level: usize, sn: &'static SampledNet) {
        unsafe { walt_set_policy_hook(self.handle, level as c_int, Some(net_policy_sampled_cb), sn as *const _ as *mut c_void) }
    }
    /// Inside level-1 searches, the modeled others' moves that were uniform random (dice) are chosen by `net` instead.
    pub fn set_leaf_net(&self, net: &'static crate::net::Net) {
        unsafe { walt_set_leaf_hook(self.handle, Some(net_policy_cb), net as *const _ as *mut c_void) }
    }
    /// VARIANT (not output-identical to anything): level-1 searches stop at the first trick boundary after `tricks`
    /// completed tricks past their root and value each fiber by `net` (clamped to [0,1]). Pair with `set_leaf_net` so the
    /// net also plays the modeled others inside the trick.
    pub fn set_leaf_value_net(&self, net: &'static crate::net::Net, tricks: usize) {
        unsafe { walt_set_leaf_value_hook(self.handle, Some(net_value_cb), net as *const _ as *mut c_void, tricks as c_int) }
    }
}

/// Pack Walt's level-0 Key (key frame: odd seats declare) into turbo's public state.
/// History words stay zero: they only salt turbo's RNG/cache identity, never its semantics.
pub fn pack(played: u32, leader: u8, plays: &[u8], t1: u8, t0: u8, voids: Option<[u32; 4]>) -> [u64; 19] {
    let mut a = [0u64; 19];
    a[0] = played as u64; a[1] = leader as u64; a[2] = plays.len() as u64; a[3] = t1 as u64; a[4] = t0 as u64; a[5] = played.count_ones() as u64;
    for (i, &t) in plays.iter().enumerate() { a[6 + i] = t as u64; }
    if let Some(v) = voids { for i in 0..4 { a[10 + i] = v[i] as u64; } }
    a
}
/// Number of hidden deals consistent with the public state and voids.
pub fn support_size(p: &[u64; 19], hand: u32) -> u64 { let mut out = [0u32; 4]; let mut total = 0u64; assert_eq!(unsafe { walt_rank(p.as_ptr(), hand, 0, out.as_mut_ptr(), &mut total) }, 0); total }
pub fn pack_key(key: &Key) -> [u64; 19] { pack(key.played, key.leader, &key.plays, key.banked_t1, key.banked_t0, key.voids) }

/// Inner-rung replacement: turbo level 1 answers Walt's level-0 calls. `delta < 1` is used only
/// at depth >= `mindepth`; a fiber-cap failure falls back to the delta = 1 engine (counted).
pub struct TurboInner { pub decl: usize, pub n: usize, pub delta: f64, pub mindepth: usize, pub enum_cap: u64, pub leaf: Option<&'static crate::net::Net>, exact: Mutex<Option<Engine>>, sampled: Mutex<Option<Engine>>, pub fallbacks: AtomicU64, pub calls: AtomicU64 }
impl TurboInner {
    pub fn new(decl: usize, n: usize, delta: f64, mindepth: usize) -> TurboInner {
        TurboInner { decl, n, delta, mindepth, enum_cap: 0, leaf: None, exact: Mutex::new(None), sampled: Mutex::new(None), fallbacks: AtomicU64::new(0), calls: AtomicU64::new(0) }
    }
    pub fn parse(decl: usize, spec: &str) -> TurboInner {
        // turbo:N[:DELTA[:MINDEPTH[:ENUMCAP]]]  (delta engine used at depth >= MINDEPTH; it enumerates supports <= ENUMCAP)
        let f: Vec<&str> = spec.split(':').collect();
        let n = f.get(1).map_or(8, |s| s.parse().unwrap()); let delta = f.get(2).map_or(1.0, |s| parse_delta(s)); let md = f.get(3).map_or(0, |s| s.parse().unwrap());
        let mut t = TurboInner::new(decl, n, delta, md); t.enum_cap = f.get(4).map_or(0, |s| s.parse().unwrap()); t
    }
}
pub fn parse_delta(s: &str) -> f64 {
    if let Some((a, b)) = s.split_once('/') { a.parse::<f64>().unwrap() / b.parse::<f64>().unwrap() } else { s.parse().unwrap() }
}
impl NetPolicy for TurboInner {
    fn choose(&self, key: &Key, seat: Seat, hand: u32, legal: u32) -> u8 {
        self.calls.fetch_add(1, Ordering::Relaxed);
        let p = pack_key(key);
        debug_assert_eq!((key.leader as usize + key.plays.len()) % 4, seat.index());
        let depth = key.played.count_ones() as usize;
        let mut tile: Option<u8> = None;
        if (self.delta < 1.0 || self.enum_cap > 0) && depth >= self.mindepth {
            let mut g = self.exact.lock().unwrap();
            let e = g.get_or_insert_with(|| Engine::with_enumerate(self.decl, &[self.n], self.delta, FIELD_SEED, 2_000_000, self.enum_cap));
            match e.action(1, &p, hand, false) { Ok((t, _)) => tile = Some(t), Err(_) => { self.fallbacks.fetch_add(1, Ordering::Relaxed); } }
        }
        let t = match tile { Some(t) => t, None => {
            let mut g = self.sampled.lock().unwrap();
            let e = g.get_or_insert_with(|| { let e = Engine::new(self.decl, &[self.n], 1.0, FIELD_SEED, 2_000_000); if let Some(nn) = self.leaf { e.set_leaf_net(nn); } e });
            e.action(1, &p, hand, false).unwrap_or_else(|m| panic!("turbo level 1 failed: {m} played={:#x} leader={} plays={:?} hand={hand:#x}", key.played, key.leader, key.plays)).0
        } };
        assert!(legal >> t & 1 == 1, "turbo chose an illegal tile {t} legal={legal:#x} played={:#x}", key.played);
        t
    }
}

/// Whole-player arm: turbo level 2 (best response to its own level 1) replaces Walt's outer
/// search for the hybrid partnership. `ns` bottom first: [inner n0, outer n].
pub struct L2Player { pub engine: Mutex<Engine>, pub fallback: Mutex<Option<Engine>>, pub n: Vec<usize>, pub delta: f64, pub decl: usize, pub fallbacks: AtomicU64 }
impl L2Player {
    pub fn parse(decl: usize, spec: &str) -> L2Player {
        // l2:OUTER:INNER[:DELTA]
        let f: Vec<&str> = spec.split(':').collect();
        let outer: usize = f[1].parse().unwrap(); let inner: usize = f[2].parse().unwrap(); let delta = f.get(3).map_or(1.0, |s| parse_delta(s));
        L2Player { engine: Mutex::new(Engine::new(decl, &[inner, outer], delta, FIELD_SEED, 2_000_000)), fallback: Mutex::new(None), n: vec![inner, outer], delta, decl, fallbacks: AtomicU64::new(0) }
    }
    pub fn choose(&self, p: &[u64; 19], hand: u32, legal: u32) -> u8 {
        let r = self.engine.lock().unwrap().action(2, p, hand, false);
        let t = match r { Ok((t, _)) => t, Err(_) => {
            self.fallbacks.fetch_add(1, Ordering::Relaxed);
            let mut g = self.fallback.lock().unwrap();
            let e = g.get_or_insert_with(|| Engine::new(self.decl, &self.n, 1.0, FIELD_SEED, 2_000_000));
            e.action(2, p, hand, false).unwrap_or_else(|m| panic!("turbo level 2 failed: {m}")).0
        } };
        assert!(legal >> t & 1 == 1, "turbo L2 chose an illegal tile");
        t
    }
}

/// Engines keyed by declaration for offline evaluation/labeling.
pub struct Pool { pub ns: Vec<usize>, pub delta: f64, pub seed: u64, pub fiber_cap: u64, pub enum_cap: u64, map: HashMap<usize, Engine> }
impl Pool {
    pub fn new(ns: Vec<usize>, delta: f64, seed: u64, fiber_cap: u64) -> Pool { Pool { ns, delta, seed, fiber_cap, enum_cap: 0, map: HashMap::new() } }
    pub fn get(&mut self, decl: usize) -> &Engine {
        let (ns, d, s, fc, ec) = (self.ns.clone(), self.delta, self.seed, self.fiber_cap, self.enum_cap);
        self.map.entry(decl).or_insert_with(|| Engine::with_enumerate(decl, &ns, d, s, fc, ec))
    }
}
