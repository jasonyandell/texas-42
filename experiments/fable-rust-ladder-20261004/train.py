#!/usr/bin/env python3
"""The one trainer for the level-0 student nets (MLX). Run every training call under run_capped.py.

Architectures (--arch), all exported in a format `ladder/src/net.rs` loads (header table in README.md):
  mlp  embedding-sum first layer over the encoded state (encoding --enc, default 4 = 1483 rows), --layers 1|2
       dense ReLU layers (--hidden, --hidden2), 28 logits. Header [enc, layers, H, H2].
  res  same first layer, then --blocks residual blocks h += relu(LN?(h) wa + ba) wb + bb (inner width --inner,
       pre-LayerNorm when --ln 1), final LN (or ReLU when --ln 0), 28 logits. Header [4, 3, H, inner, blocks, ln].
  tok  28 tile tokens (20 features + E[decl, tile] + broadcast 28-wide context) -> --layers pre-LN transformer
       blocks (--d, --heads, --dff) -> one logit per tile. Header [9, 1, d, layers, heads, dff, 20].
Losses (--loss), all = bw * BCE(sigmoid(z), k/N on legal tiles) + aux:
  bce     aux = 0.
  mix     aux = CE of the legal softmax (sign-flipped for minimizers) against the stored choice (lowest argmax),
          label-smoothed by --eps (0 = off).
  tce     as mix, target uniform over ALL argmax tiles.
  soft    as mix, target q_t ~ exp(beta * (v_t - v_best)).
  regret  aux = rw * expected regret sum_t p_t (v_best - v_t), p = legal softmax(sign * z / tau).
  --cew adds a CE term against the stored choice to any loss; --stakes weights rows by 1 + A * spread.
  v = k/N for maximizers, -k/N for minimizers (make-probability units). --tau also divides the CE logits.
Optimizers (--opt):
  adam        Adam (b1 .9, b2 .999), lr constant or cosine from --lr to --lr-end over --updates; --wd = decoupled
              decay of weight matrices applied after the step. The LAD5 recipe.
  adamw       Adam with --b2, linear --warmup then cosine to --lr-end over --updates (global step), weight
              decay inside the update. The scale-study recipe; resumable across capped rounds (--resume).
  mlx-adamw   mlx.optimizers.AdamW (decay on every parameter), --warmup linear then cosine to lr/50. The token
              recipe.
Checkpoints: models/<name>.w (+ .npz, parameter dict) = the selected net (best val --select), rewritten whenever it
improves; models/<name>.json = metadata and history. --save-last also writes <name>-last.w at the end of a call.
--state saves <state-dir>/<name>.state.npz (params, moments, step) at the end of the call; --resume continues it.
Data: comma-separated 100-byte label files; `path@K` = a fixed random K rows (seed 7); `path@K:stride` = K rows
by fixed stride. Other modes: --eval-only (metrics of --init on --val, --dump-logits), --relabel IN,OUT
(distillation labels from --init: N=10000, counts = round(10000 sigmoid(z)), choice = this net's pick).
The selector everywhere: highest legal logit for maximizers, lowest for minimizers, lowest tile on ties.
"""
import argparse, json, struct, sys, time
from pathlib import Path
import numpy as np

HERE = Path(__file__).resolve().parent

# ---------------------------------------------------------------- records and encodings
REC = np.dtype([('decl', 'u1'), ('seat', 'u1'), ('maximize', 'u1'), ('leader', 'u1'), ('npl', 'u1'), ('plays', 'u1', 4), ('bt1', 'u1'), ('bt0', 'u1'), ('hand', '<u4'), ('played', '<u4'), ('legal', '<u4'), ('N', '<u2'), ('counts', '<u2', 28), ('choice', 'u1'), ('prod', 'u1'), ('hasvoids', 'u1'), ('voids', '<u4', 4)])
assert REC.itemsize == 100


def maximize_of(r):
    """Byte 2 bit 0: the mover's side declares (LAD6 files keep the contract in bits 1..5, see bid_of)."""
    return (r['maximize'] & 1).astype(bool)


def bid_of(r):
    """Contract 30..42 of a record: byte 2 >> 1, plus 30 (LAD1-5 files have 0 there = bid 30)."""
    return 30 + (r['maximize'].astype(np.int32) >> 1)
TS, CTX = 7, 19
HI = [0,1,1,2,2,2,3,3,3,3,4,4,4,4,4,5,5,5,5,5,5,6,6,6,6,6,6,6]; LO = [0,0,1,0,1,2,0,1,2,3,0,1,2,3,4,0,1,2,3,4,5,0,1,2,3,4,5,6]


class Enc:
    """Embedding-sum input encodings. 1 = tile rows x 7 states; 2 = + declaration-indexed tile rows and led suit;
    3 = tile rows also indexed by the 3-bit void pattern of the other seats (multiplicative, worse); 4 = 2 plus
    ADDITIVE void rows, one per (relative seat, tile), gathered when that seat is publicly void of that tile."""
    def __init__(self, enc):
        self.enc = enc; self.v2 = enc >= 2; self.v3 = enc == 3; self.v4 = enc >= 4; self.v5 = enc == 5; self.vp = 8 if self.v3 else 1
        self.tile_rows = 7 * 28 * TS * self.vp if self.v2 else 28 * TS
        self.ctx2 = CTX + (8 if self.v2 else 0); self.void_rows = 84 if self.v4 else 0
        self.bid_rows = 13 if self.v5 else 0  # 5 (LAD6): 4 plus a one-hot contract row, bid 30..42
        self.inputs = self.tile_rows + self.ctx2 + self.void_rows + self.bid_rows; self.ncol = 33 + (84 if self.v4 else 0) + (1 if self.v5 else 0)


E4 = Enc(4); INPUTS = E4.inputs; TILE_ROWS = E4.tile_rows  # module constants other scripts read (encoding 4)


def encode_emb(r, E=E4):
    """Embedding-sum inputs for an array of records: row indices (absent = E.inputs, a zero row) and 3 scalars."""
    n = len(r); ar = np.arange(28)
    bits = ((r['hand'][:, None] >> ar) & 1).astype(bool); pl = ((r['played'][:, None] >> ar) & 1).astype(bool)
    st = np.full((n, 28), 1, np.int32); st[bits] = 0; st[pl & ~bits] = 2
    for p in range(4):
        t = r['plays'][:, p]; ok = t < 28; st[np.flatnonzero(ok), t[ok]] = 3 + p
    pat = np.zeros((n, 28), np.int32)
    if E.v3:
        for k in range(3):
            other = (r['seat'].astype(int) + 1 + k) % 4; vo = r['voids'][np.arange(n), other]
            pat |= (((vo[:, None] >> ar) & 1).astype(np.int32)) << k
    I = E.inputs; idx = np.full((n, E.ncol), I, np.int32)
    idx[:, :28] = (r['decl'].astype(np.int32)[:, None] * (28 * TS * E.vp) if E.v2 else 0) + ar * (TS * E.vp) + st * E.vp + pat
    c = E.tile_rows; idx[:, 28] = c + r['decl'].astype(np.int32); idx[:, 29] = c + 7 + ((r['leader'].astype(int) - r['seat'].astype(int)) % 4)
    idx[:, 30] = np.where(maximize_of(r), c + 11, I)
    idx[:, 31] = c + 12 + r['npl'].astype(np.int32)
    led = r['plays'][:, 0].astype(int); hi = np.array(HI)[np.minimum(led, 27)]; lo = np.array(LO)[np.minimum(led, 27)]; d = r['decl'].astype(int)
    ledsuit = np.where(led > 27, 8, np.where((hi == d) | (lo == d), 7, hi))  # 0..6 pip, 7 trump, 8 none (leading)
    idx[:, 32] = np.where(E.v2, c + 19 + np.minimum(ledsuit, 7), I); idx[:, 32] = np.where(E.v2 & (ledsuit == 8), I, idx[:, 32])
    if E.v4:
        for k in range(3):
            other = (r['seat'].astype(int) + 1 + k) % 4; vo = r['voids'][np.arange(n), other]
            vb = ((vo[:, None] >> ar) & 1).astype(bool)
            idx[:, 33 + 28 * k:33 + 28 * (k + 1)] = np.where(vb, E.tile_rows + E.ctx2 + 28 * k + ar, I)
    if E.v5: idx[:, 117] = E.tile_rows + E.ctx2 + 84 + (bid_of(r) - 30)
    sc = np.stack([r['bt1'] / 42.0, r['bt0'] / 42.0, bits.sum(1) / 7.0], 1).astype(np.float32)
    return idx, sc


NF, NC = 20, 28
FSCALE = np.array([1] * 17 + [1 / 6, 1 / 6, 1 / 2], np.float32)  # pips hi, lo /6; count units of 5 (0,1,2) /2


def encode_tok(r):
    """Tile-token inputs: per-tile features F [n,28,20] (uint8), context C [n,28] (f32), declaration d [n]."""
    n = len(r); ar = np.arange(28); HIa = np.array(HI); LOa = np.array(LO)
    bits = ((r['hand'][:, None] >> ar) & 1).astype(bool); pl = ((r['played'][:, None] >> ar) & 1).astype(bool)
    st = np.full((n, 28), 1, np.int64); st[bits] = 0; st[pl & ~bits] = 2
    seat = r['seat'].astype(int); leader = r['leader'].astype(int); rel = np.zeros((n, 28), np.int64)
    for p in range(4):
        t = r['plays'][:, p]; ok = np.flatnonzero(t < 28); st[ok, t[ok]] = 3 + p; rel[ok, t[ok]] = (leader[ok] + p - seat[ok]) % 4
    F = np.zeros((n, 28, NF), np.uint8)
    for s in range(7): F[:, :, s] = st == s
    for k in range(1, 4): F[:, :, 6 + k] = rel == k
    hv = r['hasvoids'].astype(np.uint32)
    for k in range(3):
        other = (seat + 1 + k) % 4; vo = r['voids'][np.arange(n), other] * hv; F[:, :, 10 + k] = (vo[:, None] >> ar) & 1
    F[:, :, 13] = (r['legal'][:, None] >> ar) & 1
    d = r['decl'].astype(int); trump = (HIa[None] == d[:, None]) | (LOa[None] == d[:, None]); F[:, :, 14] = trump
    led = r['plays'][:, 0].astype(int); lc = np.minimum(led, 27); ledsuit = np.where(led > 27, 8, np.where(trump[np.arange(n), lc], 7, HIa[lc]))
    tsuit = np.where(trump, 7, HIa[None]); F[:, :, 15] = (tsuit == ledsuit[:, None]) & (ledsuit[:, None] < 8)
    F[:, :, 16] = (HIa == LOa)[None]; F[:, :, 17] = HIa[None]; F[:, :, 18] = LOa[None]
    cnt = np.array([2 if HI[t] + LO[t] == 10 else 1 if HI[t] + LO[t] == 5 else 0 for t in range(28)])
    F[:, :, 19] = cnt[None]
    C = np.zeros((n, NC), np.float32); C[np.arange(n), d] = 1; C[np.arange(n), 7 + (leader - seat) % 4] = 1; C[:, 11] = maximize_of(r)
    C[np.arange(n), 12 + np.minimum(r['npl'].astype(int), 3)] = 1; C[np.arange(n), 16 + ledsuit] = 1
    C[:, 25] = r['bt1'] / 42.0; C[:, 26] = r['bt0'] / 42.0; C[:, 27] = bits.sum(1) / 7.0
    return F, C, d.astype(np.int32)


def targets(r):
    legal = ((r['legal'][:, None] >> np.arange(28)) & 1).astype(np.float32)
    y = (r['counts'] / r['N'][:, None]).astype(np.float32) * legal
    return legal, y


def load(path, E=E4):
    """Whole label file -> the legacy dict (idx, sc, legal, y, maximize, choice, prod, N, n). Kept for scripts."""
    r = np.fromfile(path, dtype=REC); idx, sc = encode_emb(r, E); legal, y = targets(r)
    return dict(idx=idx, sc=sc, legal=legal, y=y, maximize=maximize_of(r), choice=r['choice'].astype(int), prod=r['prod'].astype(int), N=int(r['N'][0]), n=len(r))


def choose(z, legal, maximize):
    """Selector: highest legal logit for maximizers, lowest for minimizers; lowest tile on ties."""
    mx_ = np.argmax(np.where(legal > 0, z, -np.inf), 1); mn = np.argmin(np.where(legal > 0, z, np.inf), 1)
    return np.where(maximize, mx_, mn)


def select_rows(n, spec):
    """Row selection for 'K' (fixed random K rows, seed 7, sorted) or 'K:stride' (first K rows of a fixed stride)."""
    if not spec: return None
    k, _, how = spec.partition(':'); k = int(k)
    if how == 'stride': return np.arange(0, n, max(1, n // k))[:k]
    assert how == '', spec
    return np.sort(np.random.default_rng(7).choice(n, k, replace=False)) if k < n else None


def load_one(spec, arch, E, cache_dir=None):
    """One label file spec -> compact arrays for `arch` (emb: idx int16, sc; tok: F, C, d) + legal, y, maximize, choice."""
    path, _, k = spec.partition('@')
    cpath = Path(cache_dir) / f"{Path(path).name}{'@' + k if k else ''}.{arch}-e{E.enc}.npz" if cache_dir else None
    if cpath is not None and cpath.exists():
        z = np.load(cpath); return {k_: z[k_] for k_ in z.files}
    mm = np.memmap(path, dtype=REC, mode='r'); sel = select_rows(len(mm), k); rows = sel if sel is not None else np.arange(len(mm)); parts = []
    for c0 in range(0, len(rows), 500000):
        r = np.array(mm[rows[c0:c0 + 500000]]); legal, y = targets(r)
        d = dict(legal=legal.astype(np.uint8), y=y, maximize=maximize_of(r), choice=r['choice'].astype(np.uint8))
        if arch == 'tok': d['F'], d['C'], d['d'] = encode_tok(r)
        else: idx, sc = encode_emb(r, E); d['idx'] = idx.astype(np.int16); d['sc'] = sc
        parts.append(d)
    out = {k_: np.concatenate([x[k_] for x in parts]) for k_ in parts[0]}
    if cpath is not None: cpath.parent.mkdir(parents=True, exist_ok=True); np.savez(cpath, **out)
    return out


def values(d):
    v = np.where(d['maximize'][:, None], d['y'], -d['y'])  # mover's value, make-probability units
    return np.where(d['legal'] > 0, v, -10.0).astype(np.float32)


def load_many(specs, arch, E, cache_dir=None):
    ds = [load_one(p, arch, E, cache_dir) for p in specs.split(',') if p]
    d = {k: np.concatenate([x[k] for x in ds]) for k in ds[0]}; d['sizes'] = [len(x['choice']) for x in ds]; del ds
    d['v'] = values(d); d['vbest'] = d['v'].max(1); d['choice'] = d['choice'].astype(np.int64)
    d['n'] = len(d['choice']); return d


def metrics(z, d):
    """agree = pick == stored choice (the pipeline's number); zero_regret = picked an argmax tile; regret = mean
    (v_best - v_pick) in make-probability units; bce = masked soft-target BCE; rmse of sigmoid(z) vs k/N."""
    legal = d['legal'].astype(np.float32); c = choose(z, legal, d['maximize']); vp = d['v'][np.arange(d['n']), c]; reg = d['vbest'] - vp
    per = np.logaddexp(0, z) - d['y'] * z; bce = float(((per * legal).sum(1) / legal.sum(1)).mean())
    pr = 1 / (1 + np.exp(-z.astype(np.float64))); rmse = float(np.sqrt((((pr - d['y']) ** 2) * legal).sum() / legal.sum()))
    return dict(agree=float(np.mean(c == d['choice'])), zero_regret=float(np.mean(reg <= 1e-9)), regret=float(reg.mean()), bce=bce, rmse=rmse)


# ---------------------------------------------------------------- weight files (mirror of ladder/src/net.rs)
TAG_TOKENS, TOKENS_VERSION, LAYERS_RESIDUAL = 9, 1, 3


def read_header(b):
    """-> (arch, cfg, header words). Same decision order as net.rs `parse_header`."""
    nw = len(b) // 4; u = struct.unpack(f'<{min(nw, 7)}I', b[:4 * min(nw, 7)])
    if nw > 7 and u[0] == TAG_TOKENS and u[1] == TOKENS_VERSION:
        return 'tok', dict(d=u[2], layers=u[3], heads=u[4], dff=u[5], nf=u[6]), 7
    if nw > 6 and u[0] == 4 and u[1] == LAYERS_RESIDUAL:
        return 'res', dict(enc=4, hidden=u[2], inner=u[3], blocks=u[4], ln=u[5]), 6
    if nw > 4 and 2 <= u[0] <= 5 and u[1] <= 2:
        return 'mlp', dict(enc=u[0], layers=u[1], hidden=u[2], hidden2=u[3]), 4
    if nw > 2 and u[0] in (1, 2):
        return 'mlp', dict(enc=1, layers=u[0], hidden=u[1], hidden2=u[1]), 2
    raise SystemExit(f'unknown weight-file header {u}')


def layout(arch, c):
    """[(key, shape)] in file order after the header."""
    if arch == 'tok':
        D, F = c['d'], c['dff']; L = [('Wf', (NF, D)), ('E', (7 * 28, D)), ('Wc', (NC, D)), ('bc', (D,))]
        for l in range(c['layers']):
            L += [(f'g1_{l}', (D,)), (f'be1_{l}', (D,)), (f'Wq_{l}', (D, D)), (f'Wk_{l}', (D, D)), (f'Wv_{l}', (D, D)), (f'Wo_{l}', (D, D)), (f'bo_{l}', (D,)),
                  (f'g2_{l}', (D,)), (f'be2_{l}', (D,)), (f'W1_{l}', (D, F)), (f'b1_{l}', (F,)), (f'W2_{l}', (F, D)), (f'b2_{l}', (D,))]
        return L + [('gf', (D,)), ('bf', (D,)), ('hw', (D,)), ('tb', (28,))]
    H = c['hidden']; L = [('w1', (Enc(c['enc']).inputs, H)), ('b1', (H,))]
    if arch == 'mlp':
        H2 = c['hidden2'] or H; hout = H2 if c['layers'] == 2 else H
        if c['layers'] == 2: L += [('wm', (H, H2)), ('bm', (H2,))]
        return L + [('w2', (hout, 28)), ('b2', (28,))]
    for i in range(c['blocks']):
        if c['ln']: L += [(f'g{i}', (H,)), (f'be{i}', (H,))]
        L += [(f'wa{i}', (H, c['inner'])), (f'ba{i}', (c['inner'],)), (f'wb{i}', (c['inner'], H)), (f'bb{i}', (H,))]
    if c['ln']: L += [('gf', (H,)), ('bef', (H,))]
    return L + [('w2', (H, 28)), ('b2', (28,))]


def header_words(arch, c):
    if arch == 'tok': return (TAG_TOKENS, TOKENS_VERSION, c['d'], c['layers'], c['heads'], c['dff'], NF)
    if arch == 'res': return (4, LAYERS_RESIDUAL, c['hidden'], c['inner'], c['blocks'], c['ln'])
    if c['enc'] == 1:  # v1 header [layers, H]: no encoding word, and a second layer must be H wide
        assert c['layers'] == 1 or (c['hidden2'] or c['hidden']) == c['hidden'], 'encoding 1 files need hidden2 == hidden'
        return (c['layers'], c['hidden'])
    return (c['enc'], c['layers'], c['hidden'], c['hidden2'] or c['hidden'])


def read_w(path):
    """-> (arch, cfg, {key: float32 array})."""
    b = open(path, 'rb').read(); arch, cfg, hw = read_header(b); off = 4 * hw; p = {}
    for k, shp in layout(arch, cfg):
        n = int(np.prod(shp)); p[k] = np.frombuffer(b[off:off + 4 * n], dtype='<f4').copy().reshape(shp); off += 4 * n
    if off != len(b): raise SystemExit(f'{path}: size {len(b)} does not match header {header_words(arch, cfg)} ({off} expected)')
    return arch, cfg, p


def write_w(path, arch, cfg, p):
    with open(path, 'wb') as f:
        f.write(struct.pack(f'<{len(header_words(arch, cfg))}I', *header_words(arch, cfg)))
        for k, _ in layout(arch, cfg): f.write(np.asarray(p[k]).astype('<f4').tobytes())


# ---------------------------------------------------------------- main
def parse(argv=None):
    ap = argparse.ArgumentParser(description=__doc__.split('\n')[0])
    g = ap.add_argument_group('data')
    g.add_argument('--train', default='', help='comma-separated label files (path, path@K, path@K:stride)'); g.add_argument('--val', required=True); g.add_argument('--val2', default='', help='extra val files, scored at the end of the call')
    g.add_argument('--name', default='x'); g.add_argument('--out-dir', default=str(HERE / 'models')); g.add_argument('--cache-dir', default='', help='cache encoded arrays here (npz per file spec)')
    g = ap.add_argument_group('architecture')
    g.add_argument('--arch', default='mlp', choices=['mlp', 'res', 'tok']); g.add_argument('--enc', type=int, default=4, help='mlp input encoding 1..4 (res and tok: 4)')
    g.add_argument('--hidden', type=int, default=256); g.add_argument('--hidden2', type=int, default=256); g.add_argument('--layers', type=int, default=2, help='mlp: dense layers 1|2; tok: transformer blocks')
    g.add_argument('--inner', type=int, default=256); g.add_argument('--blocks', type=int, default=2); g.add_argument('--ln', type=int, default=0)
    g.add_argument('--d', type=int, default=32); g.add_argument('--heads', type=int, default=2); g.add_argument('--dff', type=int, default=64)
    g = ap.add_argument_group('loss')
    g.add_argument('--loss', default='mix', choices=['bce', 'mix', 'tce', 'soft', 'regret']); g.add_argument('--bw', type=float, default=1.0, help='BCE weight')
    g.add_argument('--beta', type=float, default=50.0); g.add_argument('--tau', type=float, default=1.0); g.add_argument('--rw', type=float, default=10.0); g.add_argument('--eps', type=float, default=0.0, help='CE label smoothing')
    g.add_argument('--cew', type=float, default=0.0); g.add_argument('--stakes', type=float, default=0.0); g.add_argument('--l2', type=float, default=1e-6)
    g.add_argument('--dropout', type=float, default=0.0); g.add_argument('--edrop', type=float, default=0.0, help='drop whole input rows (mlp/res, training only)')
    g = ap.add_argument_group('optimization')
    g.add_argument('--opt', default='adam', choices=['adam', 'adamw', 'mlx-adamw']); g.add_argument('--updates', type=int, default=20000, help='schedule length (global step)')
    g.add_argument('--batch', type=int, default=1024); g.add_argument('--lr', type=float, default=.002); g.add_argument('--lr-end', type=float, default=0.0, help='adam: cosine to this (0 = constant); adamw: cosine end')
    g.add_argument('--warmup', type=int, default=0); g.add_argument('--wd', type=float, default=0.0); g.add_argument('--b2', type=float, default=0.999)
    g.add_argument('--seed', type=int, default=None, help='init/order seed (default 12345; tok 1)'); g.add_argument('--device', default='gpu', choices=['gpu', 'cpu'], help='cpu is byte-deterministic; gpu is not')
    g = ap.add_argument_group('schedule and checkpoints')
    g.add_argument('--eval-every', type=int, default=1000); g.add_argument('--select', default='regret', choices=['regret', 'agree', 'bce'])
    g.add_argument('--seconds', type=float, default=270.0, help='stop this call after this many training seconds'); g.add_argument('--stop-at', type=int, default=0, help='stop at this global step (0 = --updates)')
    g.add_argument('--init', default='', help='.npz parameter dict or any .w the Rust loader reads'); g.add_argument('--resume', action='store_true'); g.add_argument('--state', action='store_true', help='save optimizer state at the end')
    g.add_argument('--state-dir', default=str(HERE / 'checkpoints')); g.add_argument('--save-last', action='store_true')
    g = ap.add_argument_group('other modes')
    g.add_argument('--eval-only', action='store_true'); g.add_argument('--dump-logits', default=''); g.add_argument('--relabel', default='', help='IN,OUT')
    a = ap.parse_args(argv)
    if a.seed is None: a.seed = 1 if a.arch == 'tok' else 12345
    return a


def main(argv=None):
    a = parse(argv); import mlx.core as mx; mx.set_default_device(mx.gpu if a.device == 'gpu' else mx.cpu)
    # shape: from the init .w header when evaluating/relabeling, checked against the args when training
    if a.init.endswith('.w'):
        arch0, cfg0, _ = read_w(a.init)
        if a.eval_only or a.relabel:
            a.arch = arch0
            for k, v in cfg0.items():
                if k != 'nf': setattr(a, k, v)
    if a.arch != 'mlp': a.enc = 4
    cfg = dict(enc=a.enc, layers=a.layers, hidden=a.hidden, hidden2=a.hidden2, inner=a.inner, blocks=a.blocks, ln=a.ln, d=a.d, heads=a.heads, dff=a.dff)
    if a.init.endswith('.w') and not (a.eval_only or a.relabel):
        widen = a.arch == 'mlp' and arch0 == 'mlp' and cfg0['enc'] == 4 and a.enc == 5 and header_words(arch0, cfg0)[1:] == header_words(a.arch, cfg)[1:]  # LAD6: encoding 4 -> 5, new rows zero
        assert arch0 == a.arch and (widen or header_words(arch0, cfg0) == header_words(a.arch, cfg)), (a.init, header_words(arch0, cfg0), header_words(a.arch, cfg))
    E = Enc(a.enc); tok = a.arch == 'tok'; H = a.hidden; I = E.inputs
    t0 = time.perf_counter()
    tr = load_many(a.train, a.arch, E, a.cache_dir) if a.train and not (a.eval_only or a.relabel) else None
    va = load_many(a.val, a.arch, E, a.cache_dir)
    # ---- parameters (draw order is part of the seed contract)
    rng = np.random.default_rng(a.seed); f32 = lambda x: mx.array(np.asarray(x, np.float32))
    if tok:
        D, L, Hh, DFF = a.d, a.layers, a.heads, a.dff; dh = D // Hh
        g = lambda *s, sc=1.0: mx.array((rng.standard_normal(s) * sc).astype(np.float32)); zz = lambda *s: mx.zeros(s); oo = lambda *s: mx.ones(s)
        p = {'Wf': g(NF, D, sc=NF ** -.5), 'E': g(7 * 28, D, sc=.3), 'Wc': g(NC, D, sc=NC ** -.5), 'bc': zz(D)}
        for l in range(L):
            p.update({f'g1_{l}': oo(D), f'be1_{l}': zz(D), f'Wq_{l}': g(D, D, sc=D ** -.5), f'Wk_{l}': g(D, D, sc=D ** -.5), f'Wv_{l}': g(D, D, sc=D ** -.5), f'Wo_{l}': g(D, D, sc=.5 * D ** -.5), f'bo_{l}': zz(D),
                      f'g2_{l}': oo(D), f'be2_{l}': zz(D), f'W1_{l}': g(D, DFF, sc=(2 / D) ** .5), f'b1_{l}': zz(DFF), f'W2_{l}': g(DFF, D, sc=.5 * DFF ** -.5), f'b2_{l}': zz(D)})
        p.update({'gf': oo(D), 'bf': zz(D), 'hw': g(D, sc=D ** -.5), 'tb': zz(28)})
    else:
        p = {'w1': f32(rng.standard_normal((I, H)) * np.sqrt(2 / 32)), 'b1': mx.zeros(H)}
        if a.arch == 'mlp':
            H2 = a.hidden2 or H; hout = H2 if a.layers == 2 else H
            if a.layers == 2: p['wm'] = f32(rng.standard_normal((H, H2)) * np.sqrt(2 / H)); p['bm'] = mx.zeros(H2)
            p['w2'] = f32(rng.standard_normal((hout, 28)) * np.sqrt(1 / hout)); p['b2'] = mx.zeros(28)
        else:
            for i in range(a.blocks):
                p[f'wa{i}'] = f32(rng.standard_normal((H, a.inner)) * np.sqrt(2 / H)); p[f'ba{i}'] = mx.zeros(a.inner)
                p[f'wb{i}'] = f32(rng.standard_normal((a.inner, H)) * np.sqrt(1 / a.inner) / np.sqrt(2 * a.blocks)); p[f'bb{i}'] = mx.zeros(H)
                if a.ln: p[f'g{i}'] = mx.ones(H); p[f'be{i}'] = mx.zeros(H)
            if a.ln: p['gf'] = mx.ones(H); p['bef'] = mx.zeros(H)
            p['w2'] = f32(rng.standard_normal((H, 28)) * np.sqrt(1 / H)); p['b2'] = mx.zeros(28)
    if a.init.endswith('.npz'):
        z0 = np.load(a.init); assert set(z0.files) == set(p), (sorted(z0.files), sorted(p))
        for k in p:
            if k == 'w1' and z0[k].shape[0] < p[k].shape[0]:  # widened encoding (4 -> 5): new rows zero
                w1 = np.zeros(p[k].shape, np.float32); w1[:z0[k].shape[0]] = z0[k]; p[k] = f32(w1); continue
            assert tuple(z0[k].shape) == tuple(p[k].shape), k; p[k] = f32(z0[k])
    elif a.init:
        _, _, w = read_w(a.init)
        for k in p:
            if k == 'w1' and w[k].shape[0] < p[k].shape[0]:  # widened encoding: the new input rows start at zero (same logits at bid 30)
                w1 = np.zeros(p[k].shape, np.float32); w1[:w[k].shape[0]] = w[k]; p[k] = f32(w1)
            else: p[k] = f32(w[k])
    wkey = (lambda k: k.startswith('W')) if tok else (lambda k: k.startswith('w'))  # weight matrices (l2, decay)
    # ---- forward
    def lnorm(x, g_, be):
        mu = mx.mean(x, axis=-1, keepdims=True); xc = x - mu; var_ = mx.mean(xc * xc, axis=-1, keepdims=True)
        return xc * mx.rsqrt(var_ + 1e-5) * g_ + be
    def drop(x, train, rate):
        if not train or rate <= 0: return x
        return x * mx.random.bernoulli(1 - rate, x.shape).astype(mx.float32) / (1 - rate)
    if tok:
        FS = mx.array(FSCALE); AR = mx.arange(28)
        def ln_(x, gg, bb):
            m = x.mean(-1, keepdims=True); v = ((x - m) ** 2).mean(-1, keepdims=True); return (x - m) * mx.rsqrt(v + 1e-5) * gg + bb
        def fwd(pp, F, C, dcl, train=False):
            B = F.shape[0]; x = (F.astype(mx.float32) * FS) @ pp['Wf'] + mx.take(pp['E'], dcl[:, None] * 28 + AR[None], axis=0) + (C @ pp['Wc'] + pp['bc'])[:, None, :]
            for l in range(L):
                h = ln_(x, pp[f'g1_{l}'], pp[f'be1_{l}'])
                q = (h @ pp[f'Wq_{l}']).reshape(B, 28, Hh, dh).transpose(0, 2, 1, 3); k = (h @ pp[f'Wk_{l}']).reshape(B, 28, Hh, dh).transpose(0, 2, 1, 3); v = (h @ pp[f'Wv_{l}']).reshape(B, 28, Hh, dh).transpose(0, 2, 1, 3)
                att = mx.softmax((q @ k.transpose(0, 1, 3, 2)) * dh ** -.5, axis=-1); x = x + (att @ v).transpose(0, 2, 1, 3).reshape(B, 28, D) @ pp[f'Wo_{l}'] + pp[f'bo_{l}']
                h = ln_(x, pp[f'g2_{l}'], pp[f'be2_{l}']); x = x + drop(mx.maximum(h @ pp[f'W1_{l}'] + pp[f'b1_{l}'], 0), train, a.dropout) @ pp[f'W2_{l}'] + pp[f'b2_{l}']
            return ln_(x, pp['gf'], pp['bf']) @ pp['hw'] + pp['tb']
        def inputs(d, s): return mx.array(d['F'][s]), mx.array(d['C'][s]), mx.array(d['d'][s].astype(np.int32))
    else:
        ZROW = mx.zeros((1, H)); TR16 = E.tile_rows + 16
        def fwd(pp, idx, sc, train=False):
            w1 = mx.concatenate([pp['w1'], ZROW])  # dummy zero row for absent features (index INPUTS)
            e = mx.take(w1, idx, axis=0)
            if train and a.edrop > 0: e = e * mx.random.bernoulli(1 - a.edrop, idx.shape).astype(mx.float32)[:, :, None]
            h = mx.maximum(e.sum(1) + sc @ pp['w1'][TR16:TR16 + 3] + pp['b1'], 0)
            h = drop(h, train, a.dropout)
            if a.arch == 'mlp':
                if a.layers == 2: h = drop(mx.maximum(h @ pp['wm'] + pp['bm'], 0), train, a.dropout)
                return h @ pp['w2'] + pp['b2']
            for i in range(a.blocks):
                x = lnorm(h, pp[f'g{i}'], pp[f'be{i}']) if a.ln else h
                h = h + drop(mx.maximum(x @ pp[f'wa{i}'] + pp[f'ba{i}'], 0), train, a.dropout) @ pp[f'wb{i}'] + pp[f'bb{i}']
            h = lnorm(h, pp['gf'], pp['bef']) if a.ln else mx.maximum(h, 0)
            return h @ pp['w2'] + pp['b2']
        def inputs(d, s): return mx.array(d['idx'][s].astype(np.int32)), mx.array(d['sc'][s])
    def logits(pp, d, CH=16384):
        zs = []
        for c0 in range(0, d['n'], CH): zc = fwd(pp, *inputs(d, slice(c0, c0 + CH))); mx.eval(zc); zs.append(np.array(zc))
        return np.concatenate(zs)
    out = Path(a.out_dir) / a.name
    # ---- other modes
    if a.relabel:
        src, dst = a.relabel.split(','); mm = np.memmap(src, dtype=REC, mode='r'); outs = []
        for r0 in range(0, len(mm), 500000):
            rec = np.array(mm[r0:r0 + 500000]); legal, _ = targets(rec)
            dd = {'n': len(rec)}
            if tok: dd['F'], dd['C'], dd['d'] = encode_tok(rec)
            else: dd['idx'], dd['sc'] = encode_emb(rec, E)
            z = logits(p, dd).astype(np.float64); pr = 1 / (1 + np.exp(-z))
            rec['N'] = 10000; rec['counts'] = np.where(legal > 0, np.rint(pr * 10000), 0).astype(np.uint16); rec['choice'] = choose(z, legal, maximize_of(rec)).astype(np.uint8); outs.append(rec)
        np.concatenate(outs).tofile(dst); print(json.dumps(dict(relabeled=src, out=dst, rows=len(mm)))); return
    if a.eval_only:
        zv = logits(p, va); print(json.dumps(dict(init=a.init, arch=a.arch, rows=va['n'], **metrics(zv, va))))
        if a.dump_logits: np.save(a.dump_logits, zv)
        return
    # ---- loss
    def loss(pp, *args):
        *xin, legal, y, sign, onehot, v, vbest = args
        spread = vbest - mx.min(mx.where(legal > 0, v, 10.0), axis=1); w = 1 + a.stakes * spread; w = w / mx.mean(w)
        M = lambda x: mx.mean(w * x)
        z = fwd(pp, *xin, True); per = mx.logaddexp(0, z) - y * z; n = mx.sum(legal, axis=1)
        bce = M(mx.sum(per * legal, axis=1) / n)
        zs = z * sign[:, None] + (legal - 1) * 1e4; lse = mx.logsumexp(zs, axis=1); logp = zs - lse[:, None]
        if a.loss == 'bce': return a.bw * bce + reg2(pp)
        if a.loss == 'regret':
            pr = mx.softmax(zs / a.tau, axis=1); aux = a.rw * M(mx.sum(pr * (vbest[:, None] - v) * legal, axis=1))
        else:
            lq = logp if a.tau == 1.0 else (lambda s: s - mx.logsumexp(s, axis=1)[:, None])(z * sign[:, None] / a.tau + (legal - 1) * 1e4)
            if a.loss == 'mix': q = onehot
            elif a.loss == 'tce': q = (v >= vbest[:, None] - 1e-7).astype(mx.float32) * legal; q = q / mx.sum(q, axis=1, keepdims=True)
            else: q = mx.exp(a.beta * (v - vbest[:, None])) * legal; q = q / mx.sum(q, axis=1, keepdims=True)
            if a.eps > 0: q = (1 - a.eps) * q + a.eps * legal / n[:, None]
            aux = -M(mx.sum(lq * onehot, axis=1)) if (a.loss == 'mix' and a.eps <= 0) else -M(mx.sum(lq * q * legal, axis=1))
        if a.cew > 0: aux = aux - a.cew * M(mx.sum(logp * onehot, axis=1))  # + cew * CE vs the stored choice
        return a.bw * bce + aux + reg2(pp)
    def reg2(pp):
        if a.l2 == 0: return 0.0
        return a.l2 * sum(mx.sum(x ** 2) for k, x in pp.items() if wkey(k))
    vg = mx.value_and_grad(loss); mom = {k: mx.zeros_like(x) for k, x in p.items()}; var = {k: mx.zeros_like(x) for k, x in p.items()}
    # ---- fixed train subset for train-side metrics, device arrays
    sub = np.random.default_rng(3).choice(tr['n'], min(tr['n'], 32768), replace=False)
    trs = {k: tr[k][sub] for k in tr if k not in ('n', 'sizes', 'y')}; trs['y'] = tr['y'][sub]; trs['n'] = len(sub)
    keys_in = ('F', 'C', 'd') if tok else ('idx', 'sc')
    DEV = {k: mx.array(tr[k]) for k in keys_in + ('legal', 'y', 'maximize')}; TC = mx.array(tr['choice'].astype(np.int32))
    for k in keys_in + ('legal', 'y', 'v', 'vbest'): tr.pop(k, None)
    AR28 = mx.arange(28)
    def batch(j):
        legal = DEV['legal'][j].astype(mx.float32); y = DEV['y'][j]; sign = mx.where(DEV['maximize'][j], 1.0, -1.0); v = mx.where(legal > 0, sign[:, None] * y, -10.0)
        xin = (DEV['idx'][j].astype(mx.int32), DEV['sc'][j]) if not tok else (DEV['F'][j], DEV['C'][j], DEV['d'][j])
        return (*xin, legal, y, sign, (AR28[None, :] == TC[j][:, None]).astype(mx.float32), v, mx.max(v, axis=1))
    # ---- optimizer state / resume
    step0 = 0; hist = []; meta_path = out.with_suffix('.json'); state_path = Path(a.state_dir) / f'{a.name}.state.npz'
    if a.resume:
        zs_ = np.load(state_path); step0 = int(zs_['step'])
        for k in p: p[k] = f32(zs_['p_' + k]); mom[k] = f32(zs_['m_' + k]); var[k] = f32(zs_['v_' + k])
        if meta_path.exists(): hist = json.loads(meta_path.read_text()).get('history', [])
    if a.opt == 'mlx-adamw':
        assert not a.resume, '--resume is not supported with --opt mlx-adamw'
        import mlx.optimizers as optim
        cos = optim.cosine_decay(a.lr, max(1, a.updates - a.warmup), a.lr * .02)
        sched = optim.join_schedules([optim.linear_schedule(0, a.lr, a.warmup), cos], [a.warmup]) if a.warmup > 0 else cos
        opt = optim.AdamW(learning_rate=sched, weight_decay=a.wd)
    def lr_at(step):  # learning rate used for update number `step` (1-based)
        if a.opt == 'adam': return a.lr if a.lr_end <= 0 else a.lr_end + .5 * (a.lr - a.lr_end) * (1 + np.cos(np.pi * step / a.updates))
        if a.opt == 'adamw':
            s = step - 1
            if s < a.warmup: return a.lr * (s + 1) / a.warmup
            f = min(1.0, (s - a.warmup) / max(1, a.updates - a.warmup)); return a.lr_end + .5 * (a.lr - a.lr_end) * (1 + np.cos(np.pi * f))
        return float(sched(mx.array(step - 1)))
    crit_of = {'regret': lambda m: m['regret'], 'agree': lambda m: -m['agree'], 'bce': lambda m: m['bce']}[a.select]
    best = min([crit_of(h) for h in hist], default=float('inf')); sel = next((h['update'] for h in hist if crit_of(h) == best), None)
    N = tr['n']; end = a.stop_at or a.updates
    # data order: continue the init rng (fresh run, the stu contract); reseed by global step when resuming
    order = rng.permutation(N) if step0 == 0 else np.random.default_rng(a.seed + step0).permutation(N); cur = 0
    recent = []; t1 = time.perf_counter(); step = step0; out.parent.mkdir(parents=True, exist_ok=True)
    while step < end:
        if cur + a.batch > N: order = rng.permutation(N) if step0 == 0 else np.random.default_rng(a.seed + step).permutation(N); cur = 0
        j = mx.array(order[cur:cur + a.batch]); cur += a.batch; step += 1
        val, g = vg(p, *batch(j)); lr = lr_at(step)
        if a.opt == 'adam':
            for k in p:
                mom[k] = .9 * mom[k] + .1 * g[k]; var[k] = .999 * var[k] + .001 * g[k] * g[k]; p[k] = p[k] - lr * (mom[k] / (1 - .9 ** step)) / (mx.sqrt(var[k] / (1 - .999 ** step)) + 1e-8)
                if a.wd > 0 and wkey(k): p[k] = p[k] - lr * a.wd * p[k]
            mx.eval(p, mom, var, val)
        elif a.opt == 'adamw':
            mh = 1 - .9 ** step; vh = 1 - a.b2 ** step
            for k in p:
                mom[k] = .9 * mom[k] + .1 * g[k]; var[k] = a.b2 * var[k] + (1 - a.b2) * g[k] * g[k]
                upd = (mom[k] / mh) / (mx.sqrt(var[k] / vh) + 1e-8)
                if wkey(k): upd = upd + a.wd * p[k]
                p[k] = p[k] - lr * upd
            mx.eval(p, mom, var, val)
        else:
            opt.update(p, g); mx.eval(p, opt.state, val)
        recent.append(float(val.item()))
        last = step == end or time.perf_counter() - t1 > a.seconds
        if step % a.eval_every == 0 or last:
            m = metrics(logits(p, va), va); mt = {'train_' + k: x for k, x in metrics(logits(p, trs), trs).items()}
            hist.append(dict(update=step, train_loss=float(np.mean(recent)), lr=float(lr), **m, **mt)); recent = []
            print(json.dumps(hist[-1]), flush=True)
            if crit_of(m) < best:
                best = crit_of(m); sel = step; pn = {k: np.array(x).astype('<f4') for k, x in p.items()}
                np.savez(out.with_suffix('.npz'), **pn); write_w(out.with_suffix('.w'), a.arch, cfg, pn)
            if last: break
    pn = {k: np.array(x).astype('<f4') for k, x in p.items()}
    if a.save_last: write_w(out.parent / f'{a.name}-last.w', a.arch, cfg, pn)
    if a.state or a.resume:
        assert a.opt != 'mlx-adamw', '--state is not supported with --opt mlx-adamw'
        state_path.parent.mkdir(parents=True, exist_ok=True)
        np.savez(state_path, step=step, **{'p_' + k: pn[k] for k in p}, **{'m_' + k: np.array(mom[k]) for k in p}, **{'v_' + k: np.array(var[k]) for k in p})
    extra = {Path(s).name: metrics(logits(p, e), e) for s in a.val2.split(',') if s for e in [load_many(s, a.arch, E, a.cache_dir)]}
    meta = dict(name=a.name, args=vars(a), arch=a.arch, header=list(header_words(a.arch, cfg)), inputs=None if tok else I, params=int(sum(np.prod(x.shape) for x in p.values())),
                train_rows=N, val_rows=va['n'], selected_update=sel, selected=next((h for h in hist if h['update'] == sel), None), global_step=step, stopped_at=step,
                extra_val=extra, load_seconds=t1 - t0, train_seconds=time.perf_counter() - t1, history=hist)
    meta_path.write_text(json.dumps(meta, indent=1)); print(json.dumps({k: x for k, x in meta.items() if k not in ('history', 'args')}))


if __name__ == '__main__': main()
