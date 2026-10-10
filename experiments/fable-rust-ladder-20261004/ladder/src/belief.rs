//! Belief head (EXPLORATORY, flagged): for each unseen tile, a distribution over which of the three OTHER seats
//! holds it (relative seats k = 0..2 = mover+1+k in the key frame), conditioned on the same encoding-4 public state
//! + own hand the policy nets read. Used only by the belief-weighted search arm (`l2b:` in main.rs) and the
//! `belief-*` subcommands; nothing else loads it.
//!
//! File (`.w`, little-endian): header `[10, 1, enc, layers, H, H2]` (net.rs `TAG_BELIEF`, `BELIEF_VERSION`), then the
//! MLP trunk arrays exactly as an MLP policy file (`w1 [INPUTS(enc)][H]`, `b1 [H]`, if layers = 2 `wm [H][H2]`,
//! `bm [H2]`), then `w3 [Hout][84]`, `b3 [84]`; output `z[3t + k]` is the logit that relative seat k holds tile t.
//! Probabilities: softmax over k of `z[3t + k]` restricted to the seats that can hold t (not publicly void in t's
//! suit, still holding at least one tile); tiles that are in the mover's hand or played get no distribution.
use crate::net::{inputs_for, parse_header, Body, Header, Net};
use walt::rules::Seat;
use walt::solver::Key;

pub struct BeliefNet { pub trunk: Net, pub w3: Vec<f32>, pub b3: Vec<f32> }

/// Public facts the belief and the uniform support share: unseen tiles, per-tile allowed relative seats (bit k),
/// and how many tiles each relative seat still holds.
#[derive(Clone, Copy, Debug)]
pub struct Frame { pub me: usize, pub unseen: u32, pub allowed: [u8; 28], pub caps: [usize; 3] }

pub fn frame(key: &Key, seat: Seat, hand: u32) -> Frame {
    let me = seat.index(); let voids = key.voids.unwrap_or([0; 4]);
    let base = 7 - ((key.played.count_ones() as usize - key.plays.len()) / 4);
    let mut caps = [base; 3];
    for i in 0..key.plays.len() { let s = (key.leader as usize + i) % 4; if s != me { caps[(s + 3 - me) % 4] -= 1; } }
    let unseen = !(key.played | hand) & 0x0fff_ffff; let mut allowed = [0u8; 28];
    for t in 0..28 { if unseen >> t & 1 == 1 { for k in 0..3 { if voids[(me + 1 + k) % 4] >> t & 1 == 0 && caps[k] > 0 { allowed[t] |= 1 << k; } } } }
    Frame { me, unseen, allowed, caps }
}

impl BeliefNet {
    pub fn load(path: &str, decl: u8) -> std::io::Result<BeliefNet> {
        let bad = |m: String| std::io::Error::new(std::io::ErrorKind::InvalidData, m);
        let bytes = std::fs::read(path)?; let (header, mut i) = parse_header(&bytes)?;
        let Header::Belief { enc, layers, h, h2 } = header else { return Err(bad(format!("{path}: not a belief-head file ({header:?})"))) };
        if !(2..=4).contains(&enc) || !(1..=2).contains(&layers) { return Err(bad(format!("{path}: belief header enc {enc} layers {layers}"))); }
        let nw = bytes.len() / 4; let f = |i: usize| f32::from_le_bytes([bytes[4 * i], bytes[4 * i + 1], bytes[4 * i + 2], bytes[4 * i + 3]]);
        let mut short = false;
        let mut take = |n: usize| { if i + n > nw { short = true; i = nw; return vec![0f32; n]; } let v: Vec<f32> = (0..n).map(|j| f(i + j)).collect(); i += n; v };
        let w1 = take(inputs_for(enc) * h); let b1 = take(h);
        let (wm, bm) = if layers == 2 { (take(h * h2), take(h2)) } else { (Vec::new(), Vec::new()) };
        let hout = if layers == 2 { h2 } else { h };
        let w3 = take(hout * 84); let b3 = take(84);
        if short || i != nw || bytes.len() % 4 != 0 { return Err(bad(format!("{path}: size {} does not match belief header", bytes.len()))); }
        let trunk = Net { enc, h2: hout, h, w1, b1, wm, bm, wm_t: Vec::new(), generic: true, w2: Vec::new(), b2: Vec::new(), decl, bid: 30, body: Body::Mlp };
        Ok(BeliefNet { trunk, w3, b3 })
    }
    /// Raw logits `z[3t + k]` (generic loops; output sums over j ascending, `o + a*w`).
    pub fn logits(&self, key: &Key, seat: Seat, hand: u32) -> [f32; 84] {
        let act = self.trunk.trunk_generic(key, seat, hand); let mut out = [0f32; 84]; out.copy_from_slice(&self.b3);
        for (j, &a) in act.iter().enumerate() { if a != 0.0 { for (o, w) in out.iter_mut().zip(&self.w3[j * 84..j * 84 + 84]) { *o += a * *w; } } }
        out
    }
    /// P(relative seat k holds t) for unseen t, masked to the frame's allowed seats (0 elsewhere).
    pub fn probs(&self, key: &Key, seat: Seat, hand: u32, fr: &Frame) -> [[f64; 3]; 28] {
        let z = self.logits(key, seat, hand); let mut p = [[0f64; 3]; 28];
        for t in 0..28 {
            let al = fr.allowed[t]; if al == 0 { continue; }
            let m = (0..3).filter(|&k| al >> k & 1 == 1).map(|k| z[3 * t + k] as f64).fold(f64::NEG_INFINITY, f64::max);
            let mut s = 0.0; for k in 0..3 { if al >> k & 1 == 1 { p[t][k] = (z[3 * t + k] as f64 - m).exp(); s += p[t][k]; } }
            for k in 0..3 { p[t][k] /= s; }
        }
        p
    }
}

/// Exact marginals of the uniform distribution over void- and count-consistent hidden deals (what turbo's sampler
/// draws from): P(relative seat k holds t). Forward/backward counts over the unseen tiles in index order.
pub fn uniform_marginals(fr: &Frame) -> [[f64; 3]; 28] { tilted_marginals(fr, &[[1.0; 3]; 28]) }

/// Exact marginals of the tilted support distribution q(world) ∝ Π_t w[t][holder(t)] over the same deals
/// (w = 1 everywhere gives the uniform support). Same forward/backward recursion with multiplicative weights.
pub fn tilted_marginals(fr: &Frame, w: &[[f64; 3]; 28]) -> [[f64; 3]; 28] {
    let tiles: Vec<usize> = (0..28).filter(|&t| fr.unseen >> t & 1 == 1).collect(); let n = tiles.len();
    let (c0, c1, c2) = (fr.caps[0], fr.caps[1], fr.caps[2]); assert_eq!(c0 + c1 + c2, n, "caps do not match unseen count");
    // f[i][a][b]: ways to place tiles[..i] with a on seat 0, b on seat 1 (rest on seat 2, at most c2).
    let mut f = vec![[[0f64; 8]; 8]; n + 1]; f[0][0][0] = 1.0;
    for i in 0..n { let al = fr.allowed[tiles[i]]; for a in 0..=c0.min(i) { for b in 0..=c1.min(i - a) { let v = f[i][a][b]; if v == 0.0 { continue; } let c = i - a - b;
        let wt = &w[tiles[i]];
        if al & 1 != 0 && a < c0 { f[i + 1][a + 1][b] += v * wt[0]; } if al & 2 != 0 && b < c1 { f[i + 1][a][b + 1] += v * wt[1]; } if al & 4 != 0 && c < c2 { f[i + 1][a][b] += v * wt[2]; } } } }
    // g[i][a][b]: ways to place tiles[i..] with exactly a on seat 0, b on seat 1, the rest on seat 2.
    let mut g = vec![[[0f64; 8]; 8]; n + 1]; g[n][0][0] = 1.0;
    for i in (0..n).rev() { let al = fr.allowed[tiles[i]]; let r = n - i; for a in 0..=c0.min(r) { for b in 0..=c1.min(r - a) { let c = r - a - b; if c > c2 { continue; } let mut v = 0.0;
        let wt = &w[tiles[i]];
        if al & 1 != 0 && a > 0 { v += wt[0] * g[i + 1][a - 1][b]; } if al & 2 != 0 && b > 0 { v += wt[1] * g[i + 1][a][b - 1]; } if al & 4 != 0 && c > 0 { v += wt[2] * g[i + 1][a][b]; } g[i][a][b] = v; } } }
    let total = g[0][c0][c1]; assert!(total > 0.0, "empty support"); let mut p = [[0f64; 3]; 28];
    for i in 0..n { let al = fr.allowed[tiles[i]]; let wt = &w[tiles[i]]; let mut m = [0f64; 3];
        for a in 0..=c0.min(i) { for b in 0..=c1.min(i - a) { let v = f[i][a][b]; if v == 0.0 { continue; } let c = i - a - b;
            if al & 1 != 0 && a < c0 { m[0] += v * wt[0] * g[i + 1][c0 - a - 1][c1 - b]; } if al & 2 != 0 && b < c1 { m[1] += v * wt[1] * g[i + 1][c0 - a][c1 - b - 1]; } if al & 4 != 0 && c < c2 { m[2] += v * wt[2] * g[i + 1][c0 - a][c1 - b]; } } }
        for k in 0..3 { p[tiles[i]][k] = m[k] / total; } }
    p
}

/// Per-tile log tilt that makes the tilted support distribution's marginals match the belief (iterative
/// proportional fitting, `iters` sweeps, each a full exact-marginal pass; start = the naive ratio pb/pu). Returns
/// log w[t][k] (0 where not allowed), scaled by `alpha`; `world_log_weight` then gives a world's log importance weight
/// against the uniform proposal.
pub fn ipf_log_tilt(pb: &[[f64; 3]; 28], pu: &[[f64; 3]; 28], fr: &Frame, iters: usize, alpha: f64) -> [[f64; 3]; 28] {
    let mut lw = log_ratio(pb, pu, fr, 1.0);
    for _ in 0..iters {
        let mut w = [[1f64; 3]; 28]; for t in 0..28 { for k in 0..3 { w[t][k] = lw[t][k].exp(); } }
        let q = tilted_marginals(fr, &w);
        for t in 0..28 { for k in 0..3 { if fr.allowed[t] >> k & 1 == 1 && q[t][k] > 0.0 { lw[t][k] += pb[t][k].max(P_FLOOR).ln() - q[t][k].ln(); } } }
        // keep the scale bounded: per tile, subtract the max (a per-tile constant does not change q)
        for t in 0..28 { if fr.allowed[t] != 0 { let m = (0..3).filter(|&k| fr.allowed[t] >> k & 1 == 1).map(|k| lw[t][k]).fold(f64::NEG_INFINITY, f64::max); for k in 0..3 { if fr.allowed[t] >> k & 1 == 1 { lw[t][k] -= m; } } } }
    }
    for t in 0..28 { for k in 0..3 { lw[t][k] *= alpha; } }
    lw
}
/// Marginals of the tilted support for a log tilt table (diagnostic).
pub fn marginals_of_log_tilt(fr: &Frame, lw: &[[f64; 3]; 28]) -> [[f64; 3]; 28] {
    let mut w = [[1f64; 3]; 28]; for t in 0..28 { for k in 0..3 { w[t][k] = lw[t][k].exp(); } } tilted_marginals(fr, &w)
}
/// Floor on a belief probability inside a log weight (a world the belief calls impossible keeps a tiny weight).
pub const P_FLOOR: f64 = 1e-6;

/// Per-tile log ratio table `alpha * (ln max(pb, floor) - ln pu)`; a world's log weight is the sum over unseen tiles of
/// the entry for the relative seat that holds the tile there.
pub fn log_ratio(pb: &[[f64; 3]; 28], pu: &[[f64; 3]; 28], fr: &Frame, alpha: f64) -> [[f64; 3]; 28] {
    let mut r = [[0f64; 3]; 28];
    for t in 0..28 { for k in 0..3 { if fr.allowed[t] >> k & 1 == 1 && pu[t][k] > 0.0 { r[t][k] = alpha * (pb[t][k].max(P_FLOOR).ln() - pu[t][k].ln()); } } }
    r
}
pub fn world_log_weight(r: &[[f64; 3]; 28], fr: &Frame, world: &[u32; 4]) -> f64 {
    let mut s = 0.0;
    for k in 0..3 { let mut m = world[(fr.me + 1 + k) % 4] & fr.unseen; while m != 0 { let t = m.trailing_zeros() as usize; m &= m - 1; s += r[t][k]; } }
    s
}

/// Effective-sample-size statistics of the belief weights, per trick of the decision (0..6): decisions, sum of
/// ESS/N, minimum ESS/N; read by `h2h` for its worker summary.
pub static ESS: std::sync::Mutex<[[f64; 3]; 7]> = std::sync::Mutex::new([[0.0, 0.0, f64::INFINITY]; 7]);
