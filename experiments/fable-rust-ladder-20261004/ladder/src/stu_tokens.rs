//! STU tile-token student (header [9, 1, d, layers, heads, dff, nf]; trained by train_tok.py).
//! 28 tile tokens, each = Wf . features + E[decl, tile] + broadcast context; `layers` pre-LN transformer blocks;
//! one logit per token (shared head + per-tile bias). Plain f32 loops, no BLAS. Feature definitions mirror
//! `load()` in train_tok.py exactly.
use walt::solver::Key;

pub const NF: usize = 20; pub const NC: usize = 28; const MAXD: usize = 64; const MAXFF: usize = 128;
const HI: [u8; 28] = [0,1,1,2,2,2,3,3,3,3,4,4,4,4,4,5,5,5,5,5,5,6,6,6,6,6,6,6]; const LO: [u8; 28] = [0,0,1,0,1,2,0,1,2,3,0,1,2,3,4,0,1,2,3,4,5,0,1,2,3,4,5,6];
const CNT: [u8; 28] = { let mut c = [0u8; 28]; let mut t = 0; while t < 28 { let s = HI[t] + LO[t]; c[t] = if s == 10 { 2 } else if s == 5 { 1 } else { 0 }; t += 1; } c };

struct Layer { g1: Vec<f32>, be1: Vec<f32>, wq: Vec<f32>, wk: Vec<f32>, wv: Vec<f32>, wo: Vec<f32>, bo: Vec<f32>, g2: Vec<f32>, be2: Vec<f32>, w1: Vec<f32>, b1: Vec<f32>, w2: Vec<f32>, b2: Vec<f32> }
pub struct Tok { d: usize, heads: usize, dff: usize, wf: Vec<f32>, e: Vec<f32>, wc: Vec<f32>, bc: Vec<f32>, layers: Vec<Layer>, gf: Vec<f32>, bf: Vec<f32>, hw: Vec<f32>, tb: Vec<f32> }

impl Tok {
    /// `f(i)` reads the i-th f32 of the file; `i` is the offset of the first weight (after the 7-word header).
    pub fn read(d: usize, nl: usize, heads: usize, dff: usize, take: &mut dyn FnMut(usize) -> Vec<f32>) -> Tok {
        assert!(d <= MAXD && dff <= MAXFF && d % heads == 0);
        let wf = take(NF * d); let e = take(7 * 28 * d); let wc = take(NC * d); let bc = take(d);
        let mut layers = Vec::new();
        for _ in 0..nl { layers.push(Layer { g1: take(d), be1: take(d), wq: take(d * d), wk: take(d * d), wv: take(d * d), wo: take(d * d), bo: take(d), g2: take(d), be2: take(d), w1: take(d * dff), b1: take(dff), w2: take(dff * d), b2: take(d) }); }
        let gf = take(d); let bf = take(d); let hw = take(d); let tb = take(28);
        Tok { d, heads, dff, wf, e, wc, bc, layers, gf, bf, hw, tb }
    }
    /// Dispatch to the const-generic kernel for the trained shape.
    pub fn scores(&self, key: &Key, me: usize, hand: u32, legal: u32, decl: u8) -> [f32; 28] {
        match (self.d, self.dff, self.heads) {
            (32, 32, 2) => self.run::<32, 32, 2>(key, me, hand, legal, decl), (32, 64, 2) => self.run::<32, 64, 2>(key, me, hand, legal, decl), (32, 64, 4) => self.run::<32, 64, 4>(key, me, hand, legal, decl),
            (48, 48, 4) => self.run::<48, 48, 4>(key, me, hand, legal, decl), (48, 96, 2) => self.run::<48, 96, 2>(key, me, hand, legal, decl), (48, 96, 4) => self.run::<48, 96, 4>(key, me, hand, legal, decl),
            (64, 64, 4) => self.run::<64, 64, 4>(key, me, hand, legal, decl), (64, 128, 4) => self.run::<64, 128, 4>(key, me, hand, legal, decl),
            _ => panic!("stu_tokens: unsupported shape d={} dff={} heads={} (add it to Tok::scores)", self.d, self.dff, self.heads),
        }
    }
    /// Exact forward for the legal tokens. Layer inputs for all 28 tokens are needed as attention keys/values, but in the
    /// last layer only legal tokens act as queries (and need Wo, FFN and the head), which is the same arithmetic per
    /// legal token as the full forward -- illegal logits are never read (the selector masks them).
    fn run<const D: usize, const DFF: usize, const H: usize>(&self, key: &Key, me: usize, hand: u32, legal: u32, decl: u8) -> [f32; 28] {
        let d = D; let hd = D / H;
        // ---- features
        let mut st = [1u8; 28]; let mut rel = [0u8; 28];
        for t in 0..28 { if hand >> t & 1 == 1 { st[t] = 0; } else if key.played >> t & 1 == 1 { st[t] = 2; } }
        let leader = key.leader as usize;
        for (p, &t) in key.plays.iter().enumerate() { st[t as usize] = 3 + p as u8; rel[t as usize] = ((leader + p + 4 - me) % 4) as u8; }
        let voids = key.voids.unwrap_or([0; 4]);
        let dc = decl as usize;
        let trump = |t: usize| HI[t] == decl || LO[t] == decl;
        let ledsuit: usize = match key.plays.first() { None => 8, Some(&l) => if trump(l as usize) { 7 } else { HI[l as usize] as usize } };
        let mut cv = [0f32; D]; cv.copy_from_slice(&self.bc);
        { let mut addrow = |r: usize, s: f32| { for (c, w) in cv.iter_mut().zip(&self.wc[r * d..r * d + d]) { *c += s * *w; } };
          addrow(dc, 1.0); addrow(7 + (leader + 4 - me) % 4, 1.0); if me % 2 == 1 { addrow(11, 1.0); } addrow(12 + key.plays.len().min(3), 1.0); addrow(16 + ledsuit, 1.0);
          addrow(25, key.banked_t1 as f32 / 42.0); addrow(26, key.banked_t0 as f32 / 42.0); addrow(27, hand.count_ones() as f32 / 7.0); }
        let mut x = [[0f32; D]; 28];
        for t in 0..28 {
            let xt = &mut x[t];
            for j in 0..D { xt[j] = self.e[(dc * 28 + t) * d + j] + cv[j]; }
            let mut add = |f: usize, s: f32| { for (a, w) in xt.iter_mut().zip(&self.wf[f * d..f * d + d]) { *a += s * *w; } };
            add(st[t] as usize, 1.0);
            if rel[t] > 0 { add(6 + rel[t] as usize, 1.0); }
            for k in 0..3 { if voids[(me + 1 + k) % 4] >> t & 1 == 1 { add(10 + k, 1.0); } }
            if legal >> t & 1 == 1 { add(13, 1.0); }
            let tr = trump(t); if tr { add(14, 1.0); }
            let tsuit = if tr { 7 } else { HI[t] as usize }; if ledsuit < 8 && tsuit == ledsuit { add(15, 1.0); }
            if HI[t] == LO[t] { add(16, 1.0); }
            if HI[t] > 0 { add(17, HI[t] as f32 * (1.0 / 6.0)); } if LO[t] > 0 { add(18, LO[t] as f32 * (1.0 / 6.0)); } if CNT[t] > 0 { add(19, CNT[t] as f32 * 0.5); }
        }
        // ---- blocks
        let mut qs_legal = [0u8; 28]; let mut nl = 0usize; for t in 0..28u8 { if legal >> t & 1 == 1 { qs_legal[nl] = t; nl += 1; } }
        let mut qs_all = [0u8; 28]; for t in 0..28u8 { qs_all[t as usize] = t; }
        let scale = (hd as f32).powf(-0.5); let last = self.layers.len() - 1;
        let mut h = [[0f32; D]; 28]; let mut k = [[0f32; D]; 28]; let mut v = [[0f32; D]; 28];
        for (li, ly) in self.layers.iter().enumerate() {
            for t in 0..28 { h[t] = ln::<D>(&x[t], &ly.g1, &ly.be1); k[t] = mm::<D>(&h[t], &ly.wk, None); v[t] = mm::<D>(&h[t], &ly.wv, None); }
            let qs: &[u8] = if li == last { &qs_legal[..nl] } else { &qs_all[..] };
            for &t in qs {
                let t = t as usize; let q = mm::<D>(&h[t], &ly.wq, None); let mut ao = [0f32; D];
                for hh in 0..H {
                    let o = hh * hd; let mut s = [0f32; 28]; let mut mxv = f32::NEG_INFINITY;
                    for u in 0..28 { let mut a = 0f32; for j in 0..hd { a = q[o + j].mul_add(k[u][o + j], a); } s[u] = a * scale; if s[u] > mxv { mxv = s[u]; } }
                    let mut sum = 0f32; for u in 0..28 { s[u] = (s[u] - mxv).exp(); sum += s[u]; }
                    let inv = 1.0 / sum;
                    for u in 0..28 { let w = s[u] * inv; for j in 0..hd { ao[o + j] = w.mul_add(v[u][o + j], ao[o + j]); } }
                }
                let y = mm::<D>(&ao, &ly.wo, Some(&ly.bo)); for j in 0..D { x[t][j] += y[j]; }
                let h2 = ln::<D>(&x[t], &ly.g2, &ly.be2);
                let mut ff = mm::<DFF>(&h2, &ly.w1, Some(&ly.b1)); for a in ff.iter_mut() { if *a < 0.0 { *a = 0.0; } }
                let y = mm::<D>(&ff, &ly.w2, Some(&ly.b2)); for j in 0..D { x[t][j] += y[j]; }
            }
        }
        let mut out = [0f32; 28];
        for &t in &qs_legal[..nl] { let t = t as usize; let y = ln::<D>(&x[t], &self.gf, &self.bf); let mut a = 0f32; for j in 0..D { a = y[j].mul_add(self.hw[j], a); } out[t] = a + self.tb[t]; }
        out
    }
}
#[inline(always)]
fn ln<const D: usize>(x: &[f32; D], g: &[f32], b: &[f32]) -> [f32; D] {
    let n = D as f32; let m = x.iter().sum::<f32>() / n; let var = x.iter().map(|a| (a - m) * (a - m)).sum::<f32>() / n; let r = 1.0 / (var + 1e-5).sqrt();
    let mut out = [0f32; D]; for j in 0..D { out[j] = (x[j] - m) * r * g[j] + b[j]; } out
}
/// out = x . W (+ bias); W is (x.len() x O) row-major. Accumulators stay in registers over the input loop.
#[inline(always)]
fn mm<const O: usize>(x: &[f32], w: &[f32], bias: Option<&Vec<f32>>) -> [f32; O] {
    let mut acc = [0f32; O]; if let Some(b) = bias { acc.copy_from_slice(&b[..O]); }
    for (a, row) in x.iter().zip(w.chunks_exact(O)) { for j in 0..O { acc[j] = a.mul_add(row[j], acc[j]); } }
    acc
}
