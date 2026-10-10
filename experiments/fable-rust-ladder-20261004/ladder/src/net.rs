//! Level-0 student nets: weight-file format, loader and forward pass.
//!
//! Input is the exact Key the solver hands to `pi(0)`: own remaining hand, played mask, current-trick tiles in
//! order, leader, banked totals, declaration, role, and (encoding 4) the publicly known voids of the other seats.
//!
//! # Weight-file format
//!
//! A file is a little-endian u32 header followed by little-endian f32 arrays, nothing else (no padding, no trailer;
//! the loader rejects a file whose size differs from what the header implies). Matrices are row-major `[in][out]`.
//! The header kind is decided by its first words, in this order (`parse_header` is the only dispatch point):
//!
//! | kind      | header words                                   | condition                          | body      |
//! |-----------|------------------------------------------------|------------------------------------|-----------|
//! | tokens    | `[9, 1, d, layers, heads, dff, nf]`            | `w0 == TAG_TOKENS && w1 == TOKENS_VERSION` | `stu_tokens.rs` |
//! | residual  | `[4, 3, H, inner, blocks, ln]`                 | `w0 == 4 && w1 == LAYERS_RESIDUAL` | `resnet.rs` |
//! | mlp v2-4  | `[enc, layers, H, H2]`, enc 2..=5, layers 1..=2 | `w0 in ENC_MIN..=ENC_MAX && w1 <= 2` | below   |
//! | mlp v1    | `[layers, H]`, layers 1..=2 (encoding 1)       | anything else                      | below     |
//! | belief    | `[10, 1, enc, layers, H, H2]`                  | `w0 == TAG_BELIEF && w1 == BELIEF_VERSION` | `belief.rs` (not a policy: `Net::load` rejects it) |
//!
//! MLP arrays: `w1 [INPUTS(enc)][H]`, `b1 [H]`, then (layers 2 only) `wm [H][H2]`, `bm [H2]`, then
//! `w2 [Hout][28]`, `b2 [28]` with `Hout = H2` for two layers and `H` for one (H2 is ignored for one layer).
//! Residual arrays: `w1 [INPUTS4][H]`, `b1 [H]`, per block `[g, be (H each, only when ln = 1)] wa [H][inner],
//! ba [inner], wb [inner][H], bb [H]`, then `[gf, bef (H each, only when ln = 1)]`, `w2 [H][28]`, `b2 [28]`.
//! Token arrays: see `stu_tokens.rs` (`Tok::read`).
//!
//! First layer (MLP and residual) is an embedding sum: `relu(b1 + sum of the w1 rows the state activates
//! + banked_t1/42 * w1[c+16] + banked_t0/42 * w1[c+17] + hand_size/7 * w1[c+18])`, with the active rows listed
//! by `row_program` (one tile-state row per tile, then context rows; `c` = number of tile rows). MLP: optional
//! `relu(h wm + bm)`, then `z = h w2 + b2`. Residual: see `resnet.rs`. The selector picks the legal tile with the
//! highest score when the mover's side declares (odd seat in the key frame) and the lowest otherwise; ties go to
//! the lowest tile index.
//!
//! # Environment
//!
//! `LADDER_NET_GENERIC=1` forces the portable scalar loops instead of the NEON kernels. Both paths are
//! bit-identical (checked with `agree --dump`); the switch exists only as that reference check.
use walt::rules::Seat;
use walt::solver::{net_hook::NetPolicy, Key};

pub const TILE_STATES: usize = 7; // own, unseen, played, table pos 0..3
pub const CTX: usize = 19; // decl 7, leader-rel 4, maximize 1, npl 4, banked_t1/42, banked_t0/42, hand/7
pub const INPUTS: usize = 28 * TILE_STATES + CTX; // v1
pub const TILE_ROWS2: usize = 7 * 28 * TILE_STATES; pub const CTX2: usize = CTX + 8; pub const INPUTS2: usize = TILE_ROWS2 + CTX2; // v2
pub const TILE_ROWS3: usize = 7 * 28 * TILE_STATES * 8; pub const INPUTS3: usize = TILE_ROWS3 + CTX2; // v3: void pattern of other seats
pub const INPUTS4: usize = INPUTS2 + 84; // v4: v2 plus additive void rows (relative seat k, tile t) at INPUTS2 + 28k + t
pub const INPUTS5: usize = INPUTS4 + 13; // v5 (LAD6): v4 plus a one-hot contract row, bid 30..42 at INPUTS4 + bid - 30

/// Header word 0 of a tile-token file.
pub const TAG_TOKENS: u32 = 9;
/// Header word 1 of a tile-token file (format version).
pub const TOKENS_VERSION: u32 = 1;
/// Header word 1 of an encoding-4 file whose body is the residual net (an MLP file has 1 or 2 there).
pub const LAYERS_RESIDUAL: u32 = 3;
/// Encodings that carry an explicit `[enc, layers, H, H2]` header (encoding 1 files predate it).
pub const ENC_MIN: u32 = 2;
pub const ENC_MAX: u32 = 5;
/// Header word 0 of a belief-head file (`belief.rs`): MLP trunk as in the MLP kind, 84 outputs (28 tiles x 3 relative seats).
pub const TAG_BELIEF: u32 = 10;
/// Header word 1 of a belief-head file (format version).
pub const BELIEF_VERSION: u32 = 1;

/// Parsed weight-file header (see the module docs for the table).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Header {
    Mlp { enc: usize, layers: usize, h: usize, h2: usize },
    Residual { h: usize, inner: usize, blocks: usize, ln: bool },
    Tokens { d: usize, layers: usize, heads: usize, dff: usize, nf: usize },
    Belief { enc: usize, layers: usize, h: usize, h2: usize },
}

/// The single dispatch point for weight files: returns the header and the number of header words.
pub fn parse_header(bytes: &[u8]) -> std::io::Result<(Header, usize)> {
    let bad = |m: String| std::io::Error::new(std::io::ErrorKind::InvalidData, m);
    let nw = bytes.len() / 4;
    let u = |i: usize| -> u32 { if i < nw { u32::from_le_bytes([bytes[4 * i], bytes[4 * i + 1], bytes[4 * i + 2], bytes[4 * i + 3]]) } else { u32::MAX } };
    let z = |i: usize| u(i) as usize;
    if nw > 7 && u(0) == TAG_TOKENS && u(1) == TOKENS_VERSION {
        return Ok((Header::Tokens { d: z(2), layers: z(3), heads: z(4), dff: z(5), nf: z(6) }, 7));
    }
    if nw > 6 && u(0) == TAG_BELIEF && u(1) == BELIEF_VERSION {
        return Ok((Header::Belief { enc: z(2), layers: z(3), h: z(4), h2: z(5) }, 6));
    }
    if nw > 6 && u(0) == 4 && u(1) == LAYERS_RESIDUAL {
        if u(5) > 1 { return Err(bad(format!("residual header: ln must be 0 or 1, got {}", u(5)))); }
        return Ok((Header::Residual { h: z(2), inner: z(3), blocks: z(4), ln: u(5) == 1 }, 6));
    }
    if nw > 4 && (ENC_MIN..=ENC_MAX).contains(&u(0)) && u(1) <= 2 {
        return Ok((Header::Mlp { enc: z(0), layers: z(1), h: z(2), h2: z(3) }, 4));
    }
    if nw > 2 && (1..=2).contains(&u(0)) {
        return Ok((Header::Mlp { enc: 1, layers: z(0), h: z(1), h2: z(1) }, 2));
    }
    Err(bad(format!("unknown weight-file header {:?}", (0..nw.min(7)).map(u).collect::<Vec<_>>())))
}

/// Number of first-layer input rows for an MLP encoding.
pub fn inputs_for(enc: usize) -> usize { match enc { 5 => INPUTS5, 4 => INPUTS4, 3 => INPUTS3, 2 => INPUTS2, _ => INPUTS } }

/// FORK-ONLY (speed): outputs per register block in the middle layer (8 NEON vectors of f32).
pub const LANES: usize = 32;
/// Marker in the first-layer row program for the three scalar-weighted rows.
const TRIPLE: u32 = u32::MAX;
pub fn hi(t: u8) -> u8 { HI[t as usize] } pub fn lo(t: u8) -> u8 { LO[t as usize] }
const HI: [u8; 28] = [0,1,1,2,2,2,3,3,3,3,4,4,4,4,4,5,5,5,5,5,5,6,6,6,6,6,6,6]; const LO: [u8; 28] = [0,0,1,0,1,2,0,1,2,3,0,1,2,3,4,0,1,2,3,4,5,0,1,2,3,4,5,6];

/// What follows the shared embedding-sum first layer (or replaces it, for tokens).
pub enum Body {
    /// One or two dense layers (`wm`/`bm` empty for one); the NEON path when shapes allow.
    Mlp,
    /// Residual blocks (`resnet.rs`); first layer and `w2`/`b2` live in `Net`.
    Residual(Box<crate::resnet::Res>),
    /// Tile-token transformer (`stu_tokens.rs`); `Net`'s weight vectors are empty.
    Tokens(Box<crate::stu_tokens::Tok>),
}

pub struct Net {
    pub enc: usize,
    pub h2: usize,
    pub h: usize,
    pub w1: Vec<f32>, // INPUTS x h, row-major
    pub b1: Vec<f32>,
    pub wm: Vec<f32>, // h x h2 when two layers, else empty
    pub bm: Vec<f32>,
    /// FORK-ONLY (speed): `wm` re-laid out chunk-major, `LANES` outputs per chunk: wm_t[(c*h + j)*LANES + l] = wm[j*h2 + c*LANES + l].
    /// Empty when h2 is not a multiple of LANES (the generic loop is used then).
    pub wm_t: Vec<f32>,
    /// True forces the generic (scalar) MLP loops; set from env `LADDER_NET_GENERIC=1` at load (bit-identity reference).
    pub generic: bool,
    pub w2: Vec<f32>, // hout x 28
    pub b2: Vec<f32>,
    pub decl: u8,
    /// Contract (30..=42); read only by encoding 5 (one-hot input row), 30 for every other net.
    pub bid: u8,
    pub body: Body,
}

pub fn encode_states(key: &Key, hand: u32) -> [u8; 28] {
    let mut st = [1u8; 28];
    for t in 0..28u32 {
        if hand >> t & 1 == 1 { st[t as usize] = 0; } else if key.played >> t & 1 == 1 { st[t as usize] = 2; }
    }
    for (p, &t) in key.plays.iter().enumerate() { st[t as usize] = 3 + p as u8; }
    st
}

impl Net {
    /// `load` for a contract at `bid` (30..=42): encoding-5 nets read it as an input row, every other net ignores it.
    pub fn load_bid(path: &str, decl: u8, bid: u8) -> std::io::Result<Net> { assert!((30..=42).contains(&bid)); let mut n = Net::load(path, decl)?; n.bid = bid; Ok(n) }
    pub fn load(path: &str, decl: u8) -> std::io::Result<Net> {
        let bytes = std::fs::read(path)?;
        let (header, mut i) = parse_header(&bytes)?;
        let f = |i: usize| f32::from_le_bytes([bytes[4 * i], bytes[4 * i + 1], bytes[4 * i + 2], bytes[4 * i + 3]]);
        let nw = bytes.len() / 4;
        let mut short = false;
        let mut take = |n: usize| { if i + n > nw { short = true; i = nw; return vec![0f32; n]; } let v: Vec<f32> = (0..n).map(|j| f(i + j)).collect(); i += n; v };
        let empty = Vec::new;
        let net = match header {
            Header::Belief { .. } => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{path}: belief-head file, not a policy (load it with belief::BeliefNet)"))),
            Header::Tokens { d, layers, heads, dff, nf } => {
                if nf != crate::stu_tokens::NF { return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("tokens: nf {nf} != {}", crate::stu_tokens::NF))); }
                let tok = crate::stu_tokens::Tok::read(d, layers, heads, dff, &mut take);
                Net { enc: 4, h2: d, h: d, w1: empty(), b1: empty(), wm: empty(), bm: empty(), wm_t: empty(), generic: true, w2: empty(), b2: empty(), decl, bid: 30, body: Body::Tokens(Box::new(tok)) }
            }
            Header::Residual { h, inner, blocks, ln } => {
                let w1 = take(INPUTS4 * h); let b1 = take(h);
                let res = crate::resnet::Res::read(h, inner, blocks, ln, &mut take);
                let w2 = take(h * 28); let b2 = take(28);
                Net { enc: 4, h2: h, h, w1, b1, wm: empty(), bm: empty(), wm_t: empty(), generic: true, w2, b2, decl, bid: 30, body: Body::Residual(Box::new(res)) }
            }
            Header::Mlp { enc, layers, h, h2 } => {
                let w1 = take(inputs_for(enc) * h); let b1 = take(h);
                let (wm, bm) = if layers == 2 { (take(h * h2), take(h2)) } else { (Vec::new(), Vec::new()) };
                let hout = if layers == 2 { h2 } else { h };
                let w2 = take(hout * 28); let b2 = take(28);
                let wm_t = if layers == 2 && hout % LANES == 0 { let mut t = vec![0f32; h * hout]; for c in 0..hout / LANES { for j in 0..h { for l in 0..LANES { t[(c * h + j) * LANES + l] = wm[j * hout + c * LANES + l]; } } } t } else { Vec::new() };
                let generic = std::env::var("LADDER_NET_GENERIC").map_or(false, |v| v == "1");
                Net { enc, h2: hout, h, w1, b1, wm, bm, wm_t, generic, w2, b2, decl, bid: 30, body: Body::Mlp }
            }
        };
        if short || i != nw || bytes.len() % 4 != 0 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{path}: weight file size {} bytes does not match header {header:?}", bytes.len())));
        }
        Ok(net)
    }
    /// Number of nonzero first-hidden-layer activations (bench statistic). Uses the same row program as `scores`
    /// (every active row, the three scalar rows); 0 for token nets, which have no such layer.
    pub fn hidden_nonzero(&self, key: &Key, seat: Seat, hand: u32) -> usize {
        if matches!(self.body, Body::Tokens(_)) { return 0; }
        self.first_layer_generic(key, seat, hand).iter().filter(|&&a| a > 0.0).count()
    }
    /// First-layer "row program": the ordered list of w1 rows the input activates (one entry per active input, in
    /// the order the original scalar loop added them), with `TRIPLE` marking the position of the three scalar-weighted
    /// rows (banked_t1/42, banked_t0/42, hand/7). Returns the entry count.
    fn row_program(&self, key: &Key, seat: Seat, hand: u32, rows: &mut [u32; 160]) -> usize {
        let st = encode_states(key, hand); let mut n = 0usize;
        let mut push = |r: usize| { rows[n] = r as u32; n += 1; };
        if self.enc == 3 {
            let voids = key.voids.unwrap_or([0; 4]); let me = seat.index();
            let tile_base = self.decl as usize * 28 * TILE_STATES * 8;
            for t in 0..28 {
                let mut pat = 0usize;
                for k in 0..3 { if voids[(me + 1 + k) % 4] >> t & 1 == 1 { pat |= 1 << k; } }
                push(tile_base + t * TILE_STATES * 8 + st[t] as usize * 8 + pat);
            }
        } else {
            let tile_base = if self.enc >= 2 { self.decl as usize * 28 * TILE_STATES } else { 0 };
            for t in 0..28 { push(tile_base + t * TILE_STATES + st[t] as usize); }
        }
        let c = self.ctx_base();
        push(c + self.decl as usize);
        push(c + 7 + ((key.leader as usize + 4 - seat.index()) % 4));
        if seat.index() % 2 == 1 { push(c + 11); }
        push(c + 12 + key.plays.len());
        push(TRIPLE as usize);
        if self.enc >= 4 { let voids = key.voids.unwrap_or([0; 4]); let me = seat.index(); for k in 0..3 { let v = voids[(me + 1 + k) % 4]; for t in 0..28 { if v >> t & 1 == 1 { push(INPUTS2 + 28 * k + t); } } } }
        if self.enc == 5 { push(INPUTS4 + (self.bid.clamp(30, 42) - 30) as usize); }
        if self.enc >= 2 { if let Some(&led) = key.plays.first() { let (hi, lo) = (HI[led as usize], LO[led as usize]); let suit = if hi == self.decl || lo == self.decl { 7 } else { hi as usize }; push(c + 19 + suit); } }
        n
    }
    /// Trunk activations for the belief head (`belief.rs`): the generic first layer, then the optional middle layer
    /// (`relu(h wm + bm)`, generic loop). Same arithmetic as the generic MLP path of `scores` before its output layer.
    pub(crate) fn trunk_generic(&self, key: &Key, seat: Seat, hand: u32) -> Vec<f32> {
        let h2 = self.h2; let mut acc = self.first_layer_generic(key, seat, hand);
        if !self.wm.is_empty() {
            let mut mid = self.bm.clone();
            for (j, &a) in acc.iter().enumerate() { if a != 0.0 { for (m, w) in mid.iter_mut().zip(&self.wm[j * h2..j * h2 + h2]) { *m += a * *w; } } }
            for m in mid.iter_mut() { if *m < 0.0 { *m = 0.0; } }
            acc = mid;
        }
        acc
    }
    /// First context row (= number of tile rows) for this encoding.
    fn ctx_base(&self) -> usize { match self.enc { 3 => TILE_ROWS3, 2 | 4 | 5 => TILE_ROWS2, _ => 28 * TILE_STATES } }
    /// The three scalar-weighted first-layer rows and their scales.
    fn triple(&self, key: &Key, hand: u32) -> ((&[f32], &[f32], &[f32]), (f32, f32, f32)) {
        let (h, c) = (self.h, self.ctx_base());
        ((&self.w1[(c + 16) * h..(c + 17) * h], &self.w1[(c + 17) * h..(c + 18) * h], &self.w1[(c + 18) * h..(c + 19) * h]),
         (key.banked_t1 as f32 / 42.0, key.banked_t0 as f32 / 42.0, hand.count_ones() as f32 / 7.0))
    }
    /// Generic (reference) first layer: relu(b1 + rows in row-program order, the scalar triple at its marker).
    fn first_layer_generic(&self, key: &Key, seat: Seat, hand: u32) -> Vec<f32> {
        let h = self.h; let ((r1, r0, rh), (s1, s0, sh)) = self.triple(key, hand);
        let mut rows = [0u32; 160]; let n = self.row_program(key, seat, hand, &mut rows);
        let mut acc = self.b1.clone();
        for &r in &rows[..n] {
            if r == TRIPLE { for (((a, w1), w0), wh) in acc.iter_mut().zip(r1).zip(r0).zip(rh) { *a += *w1 * s1 + *w0 * s0 + *wh * sh; } }
            else { let r = r as usize; for (a, w) in acc.iter_mut().zip(&self.w1[r * h..r * h + h]) { *a += *w; } }
        }
        for a in acc.iter_mut() { if *a < 0.0 { *a = 0.0; } }
        acc
    }
    /// Declarer-make scores for every tile (only legal ones are meaningful; illegal ones are 0).
    /// MLP on aarch64: the three layers run as explicit NEON kernels (`neon` module below), LANES outputs per
    /// register block with the inputs as the inner loop. Every per-element operation and its order is the generic
    /// loop's (rows in the same sequence, `m + (a*w)` with separate multiply and add, hidden and output sums over j
    /// ascending, `if m < 0 { 0 } else { m }` as a compare-select), so the scores are bit-identical to the generic
    /// loops kept below (other widths/architectures, or `LADDER_NET_GENERIC=1`); checked with `agree --dump`.
    pub fn scores(&self, key: &Key, seat: Seat, hand: u32, legal: u32) -> [f32; 28] {
        let mut out = match &self.body {
            Body::Tokens(t) => return t.scores(key, seat.index(), hand, legal, self.decl),
            Body::Residual(res) => {
                let acc = self.first_layer_generic(key, seat, hand);
                let mut out = [0f32; 28]; out.copy_from_slice(&self.b2[..28]);
                res.forward(&acc, &self.w2, &mut out);
                out
            }
            Body::Mlp => self.mlp_scores(key, seat, hand),
        };
        for t in 0..28 { if legal >> t & 1 == 0 { out[t] = 0.0; } }
        out
    }
    fn mlp_scores(&self, key: &Key, seat: Seat, hand: u32) -> [f32; 28] {
        let mut out = [0f32; 28]; out.copy_from_slice(&self.b2[..28]);
        #[cfg(target_arch = "aarch64")]
        {
            const MAXH: usize = 1024;
            let (h, h2) = (self.h, self.h2);
            if !self.generic && h % LANES == 0 && h <= MAXH && (self.wm.is_empty() || (h2 % LANES == 0 && !self.wm_t.is_empty())) {
                let (trip, scales) = self.triple(key, hand);
                let mut rows = [0u32; 160]; let n = self.row_program(key, seat, hand, &mut rows); let rows = &rows[..n];
                let mut acc = std::mem::MaybeUninit::<[f32; MAXH]>::uninit(); let mut mid = std::mem::MaybeUninit::<[f32; MAXH]>::uninit();
                let tp = rows.iter().position(|&r| r == TRIPLE).unwrap();
                debug_assert!(rows.iter().all(|&r| r == TRIPLE || (r as usize + 1) * h <= self.w1.len()));
                // SAFETY: row indices come from row_program and lie below INPUTS (w1 has INPUTS*h entries); buffers are MAXH >= h, h2 wide;
                // wm_t has h*h2 entries laid out in blocks of h*LANES; w2 has h2*28.
                unsafe {
                    let acc = std::slice::from_raw_parts_mut(acc.as_mut_ptr() as *mut f32, h);
                    neon::first_layer(&self.w1, &self.b1, h, &rows[..tp], &rows[tp + 1..], trip, scales, acc);
                    let act: &[f32] = if self.wm.is_empty() { acc } else {
                        let mid = std::slice::from_raw_parts_mut(mid.as_mut_ptr() as *mut f32, h2);
                        let mut nzv = std::mem::MaybeUninit::<[f32; MAXH]>::uninit(); let mut nzi = std::mem::MaybeUninit::<[u32; MAXH]>::uninit();
                        let (nzv, nzi) = (std::slice::from_raw_parts_mut(nzv.as_mut_ptr() as *mut f32, h), std::slice::from_raw_parts_mut(nzi.as_mut_ptr() as *mut u32, h));
                        let mut n = 0usize;
                        for (j, &a) in acc.iter().enumerate() { if a != 0.0 { nzv[n] = a; nzi[n] = j as u32; n += 1; } }
                        neon::mid_layer(&self.wm_t, &self.bm, h, h2, &nzv[..n], &nzi[..n], mid);
                        mid
                    };
                    neon::out_layer(&self.w2, act, &mut out);
                }
                return out;
            }
        }
        // Generic path (reference semantics).
        let h2 = self.h2;
        let mut acc = self.first_layer_generic(key, seat, hand);
        if !self.wm.is_empty() {
            let mut mid = self.bm.clone();
            for (j, &a) in acc.iter().enumerate() { if a != 0.0 { for (m, w) in mid.iter_mut().zip(&self.wm[j * h2..j * h2 + h2]) { *m += a * *w; } } }
            for m in mid.iter_mut() { if *m < 0.0 { *m = 0.0; } }
            acc = mid;
        }
        for (j, &a) in acc.iter().enumerate() { for (o, w) in out.iter_mut().zip(&self.w2[j * 28..j * 28 + 28]) { *o += a * *w; } }
        out
    }
}

/// FORK-ONLY (speed): NEON kernels for `Net::scores`. Each keeps a block of LANES (= 8 x 4) partial sums in registers
/// and streams the inputs; multiply and add are separate instructions (no fused multiply-add), so every lane computes
/// exactly the generic loop's `m + (a * w)` chain in the same order.
#[cfg(target_arch = "aarch64")]
mod neon {
    use super::LANES;
    use std::arch::aarch64::*;
    const NV: usize = LANES / 4;
    /// Rows ahead to prefetch in the middle layer (the nonzero-row pattern defeats the hardware prefetcher; the
    /// 256x256 matrix does not fit L1, so without this every row chunk is an L2 fetch on the critical path).
    const AHEAD: usize = 8;
    #[inline(always)] unsafe fn prefetch(p: *const f32) { std::arch::asm!("prfm pldl1keep, [{0}]", in(reg) p, options(nostack, preserves_flags, readonly)); }
    #[inline(always)] unsafe fn relu_store(m: &[float32x4_t; NV], dst: *mut f32) {
        let zero = vdupq_n_f32(0.0);
        for i in 0..NV { let neg = vcltzq_f32(m[i]); vst1q_f32(dst.add(4 * i), vbslq_f32(neg, zero, m[i])); } // if m < 0 { 0 } else { m }
    }
    /// acc[k] = relu(b1[k] + sum over pre rows + (r1*s1 + r0*s0 + rh*sh) + sum over post rows), rows in list order.
    pub unsafe fn first_layer(w1: &[f32], b1: &[f32], h: usize, pre: &[u32], post: &[u32], (r1, r0, rh): (&[f32], &[f32], &[f32]), (s1, s0, sh): (f32, f32, f32), acc: &mut [f32]) {
        let (w1p, b1p) = (w1.as_ptr(), b1.as_ptr());
        let (s1v, s0v, shv) = (vdupq_n_f32(s1), vdupq_n_f32(s0), vdupq_n_f32(sh));
        for cb in 0..h / LANES {
            let o = cb * LANES; let mut m = [vdupq_n_f32(0.0); NV];
            for i in 0..NV { m[i] = vld1q_f32(b1p.add(o + 4 * i)); }
            for (i, &r) in pre.iter().enumerate() {
                if i + AHEAD < pre.len() { let q = w1p.add(pre[i + AHEAD] as usize * h + o); prefetch(q); prefetch(q.add(16)); }
                let p = w1p.add(r as usize * h + o); for i in 0..NV { m[i] = vaddq_f32(m[i], vld1q_f32(p.add(4 * i))); }
            }
            for i in 0..NV {
                let t = vaddq_f32(vaddq_f32(vmulq_f32(vld1q_f32(r1.as_ptr().add(o + 4 * i)), s1v), vmulq_f32(vld1q_f32(r0.as_ptr().add(o + 4 * i)), s0v)), vmulq_f32(vld1q_f32(rh.as_ptr().add(o + 4 * i)), shv));
                m[i] = vaddq_f32(m[i], t);
            }
            for (i, &r) in post.iter().enumerate() {
                if i + AHEAD < post.len() { let q = w1p.add(post[i + AHEAD] as usize * h + o); prefetch(q); prefetch(q.add(16)); }
                let p = w1p.add(r as usize * h + o); for i in 0..NV { m[i] = vaddq_f32(m[i], vld1q_f32(p.add(4 * i))); }
            }
            relu_store(&m, acc.as_mut_ptr().add(o));
        }
    }
    /// mid[k] = relu(bm[k] + sum over nonzero activations (a_j * wm[j][k]) in ascending j); wm_t is block-major.
    pub unsafe fn mid_layer(wm_t: &[f32], bm: &[f32], h: usize, h2: usize, nzv: &[f32], nzi: &[u32], mid: &mut [f32]) {
        let (wp, bp) = (wm_t.as_ptr(), bm.as_ptr());
        for cb in 0..h2 / LANES {
            let o = cb * LANES; let block = wp.add(cb * h * LANES); let mut m = [vdupq_n_f32(0.0); NV];
            for i in 0..NV { m[i] = vld1q_f32(bp.add(o + 4 * i)); }
            for (i, (&a, &j)) in nzv.iter().zip(nzi).enumerate() {
                if i + AHEAD < nzi.len() { let q = block.add(nzi[i + AHEAD] as usize * LANES); prefetch(q); prefetch(q.add(16)); }
                let av = vdupq_n_f32(a); let p = block.add(j as usize * LANES);
                for i in 0..NV { m[i] = vaddq_f32(m[i], vmulq_f32(av, vld1q_f32(p.add(4 * i)))); }
            }
            relu_store(&m, mid.as_mut_ptr().add(o));
        }
    }
    /// out[t] = b2[t] + sum over j (a_j * w2[j][t]) in ascending j (out arrives holding b2). Activations that are
    /// exactly zero are skipped: a ReLU output is +0.0, whose product with any finite weight is a signed zero, and
    /// adding a signed zero changes a value only when that value is itself an exact zero (then only its sign). So
    /// every nonzero score is bit-identical to the all-j sum and every comparison between scores is unchanged.
    pub unsafe fn out_layer(w2: &[f32], act: &[f32], out: &mut [f32; 28]) {
        let wp = w2.as_ptr(); let mut o = [vdupq_n_f32(0.0); 7];
        for i in 0..7 { o[i] = vld1q_f32(out.as_ptr().add(4 * i)); }
        for (j, &a) in act.iter().enumerate() {
            if a == 0.0 { continue; }
            let av = vdupq_n_f32(a); let p = wp.add(j * 28);
            for i in 0..7 { o[i] = vaddq_f32(o[i], vmulq_f32(av, vld1q_f32(p.add(4 * i)))); }
        }
        for i in 0..7 { vst1q_f32(out.as_mut_ptr().add(4 * i), o[i]); }
    }
}
impl NetPolicy for Net {
    fn choose(&self, key: &Key, seat: Seat, hand: u32, legal: u32) -> u8 {
        let s = self.scores(key, seat, hand, legal); let maximize = seat.index() % 2 == 1;
        let mut best = 255u8; let mut bv = 0f32;
        for t in 0..28u8 {
            if legal >> t & 1 == 0 { continue; }
            if best == 255 || (if maximize { s[t as usize] > bv } else { s[t as usize] < bv }) { best = t; bv = s[t as usize]; }
        }
        best
    }
}

/// Degenerate inner mind for control: lowest legal tile, no evaluation.
pub struct Ascending;
impl NetPolicy for Ascending { fn choose(&self, _k: &Key, _s: Seat, _h: u32, legal: u32) -> u8 { legal.trailing_zeros() as u8 } }
