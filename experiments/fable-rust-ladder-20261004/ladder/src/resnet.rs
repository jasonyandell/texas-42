//! STU (student-side experiment): residual body for the level-0 net, trained by `train_stu.py --arch res`.
//! Weight file: header u32 [enc=4, 3, H, inner, blocks, ln], then w1 (INPUTS4 x H), b1 (H), per block
//! [g, be (H each, only when ln)] wa (H x inner), ba (inner), wb (inner x H), bb (H); [gf, bef (H each, only when ln)];
//! w2 (H x 28), b2 (28). Forward (same as the trainer):
//!   h = relu(first layer)  -- the embedding-sum first layer is `Net`'s, unchanged
//!   per block: x = ln ? LayerNorm(h) : h;  h += relu(x wa + ba) wb + bb
//!   z = (ln ? LayerNorm(h) : relu(h)) w2 + b2
//! Plain f32 loops (axpy form, no fused multiply-add); not bit-identical to the Metal trainer (reduction order
//! differs), checked against it by `ladder agree` (choice agreement) and `--dump` logits.
pub struct Block { pub g: Vec<f32>, pub be: Vec<f32>, pub wa: Vec<f32>, pub ba: Vec<f32>, pub wb: Vec<f32>, pub bb: Vec<f32> }
pub struct Res { pub h: usize, pub inner: usize, pub ln: bool, pub blocks: Vec<Block>, pub gf: Vec<f32>, pub bef: Vec<f32> }

pub const MAXW: usize = 2048;

fn layer_norm(x: &[f32], g: &[f32], be: &[f32], out: &mut [f32]) {
    let n = x.len() as f32; let mu = x.iter().sum::<f32>() / n;
    let var = x.iter().map(|v| (v - mu) * (v - mu)).sum::<f32>() / n; let r = 1.0 / (var + 1e-5).sqrt();
    for (((o, v), gg), bb) in out.iter_mut().zip(x).zip(g).zip(be) { *o = (v - mu) * r * gg + bb; }
}

impl Res {
    /// Parses the residual part after the shared first layer. `take(n)` yields the next n floats.
    pub fn read(h: usize, inner: usize, nblocks: usize, ln: bool, take: &mut dyn FnMut(usize) -> Vec<f32>) -> Res {
        let mut blocks = Vec::new();
        for _ in 0..nblocks {
            let (g, be) = if ln { (take(h), take(h)) } else { (Vec::new(), Vec::new()) };
            let wa = take(h * inner); let ba = take(inner); let wb = take(inner * h); let bb = take(h);
            blocks.push(Block { g, be, wa, ba, wb, bb });
        }
        let (gf, bef) = if ln { (take(h), take(h)) } else { (Vec::new(), Vec::new()) };
        Res { h, inner, ln, blocks, gf, bef }
    }
    /// `h0` = first-layer activations after ReLU (length H); writes 28 scores into `out` (which holds b2 on entry).
    pub fn forward(&self, h0: &[f32], w2: &[f32], out: &mut [f32; 28]) {
        let (h, inner) = (self.h, self.inner);
        let mut hs = [0f32; MAXW]; let hs = &mut hs[..h]; hs.copy_from_slice(h0);
        let mut xs = [0f32; MAXW]; let xs = &mut xs[..h];
        let mut us = [0f32; MAXW]; let us = &mut us[..inner];
        for b in &self.blocks {
            if self.ln { layer_norm(hs, &b.g, &b.be, xs); } else { xs.copy_from_slice(hs); }
            us.copy_from_slice(&b.ba);
            for (j, &x) in xs.iter().enumerate() { if x != 0.0 { for (u, w) in us.iter_mut().zip(&b.wa[j * inner..(j + 1) * inner]) { *u += x * *w; } } }
            for u in us.iter_mut() { if *u < 0.0 { *u = 0.0; } }
            let mut d = [0f32; MAXW]; let d = &mut d[..h]; d.copy_from_slice(&b.bb);
            for (i, &u) in us.iter().enumerate() { if u != 0.0 { for (dd, w) in d.iter_mut().zip(&b.wb[i * h..(i + 1) * h]) { *dd += u * *w; } } }
            for (hh, dd) in hs.iter_mut().zip(d.iter()) { *hh += *dd; }
        }
        if self.ln { layer_norm(hs, &self.gf, &self.bef, xs); } else { for (x, v) in xs.iter_mut().zip(hs.iter()) { *x = if *v < 0.0 { 0.0 } else { *v }; } }
        for (j, &a) in xs.iter().enumerate() { if a != 0.0 { for (o, w) in out.iter_mut().zip(&w2[j * 28..j * 28 + 28]) { *o += a * *w; } } }
    }
}
