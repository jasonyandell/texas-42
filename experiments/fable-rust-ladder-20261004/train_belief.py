#!/usr/bin/env python3
"""Belief head trainer (MLX; EXPLORATORY). Run every call under run_capped.py; resumable across capped rounds.

Data: `ladder belief-log` 128-byte records (bytes 0..100 = train.py's REC, 100..112 = the true remaining hands of the
relative seats mover+1..3, 120 = eps-random flag, 121 = trick). Input = train.py's encoding-4 embedding sum (same
encoder as the policy); trunk = MLP (--layers 1|2, --hidden, --hidden2), optionally initialised from a policy .w
(--init-trunk); head = 84 logits z[3t + k] (tile t, relative seat k). Loss = cross-entropy of the true holder of every
unseen tile under the softmax over the seats that can hold it (not publicly void in its suit, still holding a tile).
Export: models/<name>.w with header [10, 1, enc, layers, H, H2] (README header table; Rust: ladder/src/belief.rs).
--eval-only --init X.w --dump-logits F.npy: CPU forward of a belief .w on --val (parity with `ladder belief-eval --dump`).
"""
import argparse, json, struct, time
from pathlib import Path
import numpy as np
import train as T

HERE = Path(__file__).resolve().parent
BREC = np.dtype([('rec', T.REC), ('others', '<u4', 3), ('deal_seed', '<u8'), ('random', 'u1'), ('trick', 'u1'), ('pad', 'u1', 6)])
assert BREC.itemsize == 128
TAG_BELIEF, BELIEF_VERSION = 10, 1


def frame(r):
    """-> holder [n,28] int8 (relative seat 0..2 of the true holder, -1 if not unseen), allowed [n,28,3] bool."""
    rec = r['rec']; n = len(r); ar = np.arange(28)
    seat = rec['seat'].astype(int); leader = rec['leader'].astype(int); npl = rec['npl'].astype(int)
    played = rec['played']; hand = rec['hand']
    pc = np.array([bin(x).count('1') for x in range(256)], np.int64)
    popc = lambda m: pc[m & 255] + pc[(m >> 8) & 255] + pc[(m >> 16) & 255] + pc[(m >> 24) & 255]
    base = 7 - (popc(played.astype(np.int64)) - npl) // 4
    caps = np.repeat(base[:, None], 3, 1)
    for i in range(4):
        s = (leader + i) % 4; k = (s - seat - 1) % 4; on = (i < npl) & (s != seat)
        caps[np.flatnonzero(on), k[on]] -= 1
    unseen = ~((played | hand)[:, None] >> ar & 1).astype(bool)
    allowed = np.zeros((n, 28, 3), bool); holder = np.full((n, 28), -1, np.int8)
    for k in range(3):
        vo = rec['voids'][np.arange(n), (seat + 1 + k) % 4]
        allowed[:, :, k] = unseen & ~((vo[:, None] >> ar) & 1).astype(bool) & (caps[:, k] > 0)[:, None]
        holder[((r['others'][:, k][:, None] >> ar) & 1).astype(bool)] = k
    assert np.all((holder >= 0) == unseen), 'holders do not cover the unseen tiles'
    hk = np.take_along_axis(allowed, np.maximum(holder, 0)[:, :, None].astype(np.int64), 2)[:, :, 0]
    assert np.all(hk | (holder < 0)), 'true holder not allowed'
    return holder, allowed


def load(path, limit=0):
    mm = np.memmap(path, dtype=BREC, mode='r'); n = len(mm) if not limit else min(limit, len(mm)); parts = []
    for c0 in range(0, n, 500000):
        r = np.array(mm[c0:min(n, c0 + 500000)]); idx, sc = T.encode_emb(r['rec'], T.E4); holder, allowed = frame(r)
        parts.append(dict(idx=idx.astype(np.int16), sc=sc, holder=holder, allowed=allowed, trick=r['trick'].astype(np.int8)))
    d = {k: np.concatenate([p[k] for p in parts]) for k in parts[0]}; d['n'] = n; return d


def layout(c):
    H = c['hidden']; H2 = c['hidden2'] or H; hout = H2 if c['layers'] == 2 else H
    L = [('w1', (T.INPUTS, H)), ('b1', (H,))]
    if c['layers'] == 2: L += [('wm', (H, H2)), ('bm', (H2,))]
    return L + [('w3', (hout, 84)), ('b3', (84,))]


def header(c): return (TAG_BELIEF, BELIEF_VERSION, 4, c['layers'], c['hidden'], c['hidden2'] or c['hidden'])


def write_w(path, c, p):
    with open(path, 'wb') as f:
        f.write(struct.pack('<6I', *header(c)))
        for k, _ in layout(c): f.write(np.asarray(p[k]).astype('<f4').tobytes())


def read_w(path):
    b = open(path, 'rb').read(); u = struct.unpack('<6I', b[:24]); assert u[:3] == (TAG_BELIEF, BELIEF_VERSION, 4), u
    c = dict(layers=u[3], hidden=u[4], hidden2=u[5]); off = 24; p = {}
    for k, s in layout(c):
        m = int(np.prod(s)); p[k] = np.frombuffer(b[off:off + 4 * m], '<f4').copy().reshape(s); off += 4 * m
    assert off == len(b), f'{path}: size mismatch'
    return c, p


def metrics(z, d):
    """Log loss (nats per unseen tile) and accuracy of the true holder, overall and by trick."""
    z = z.reshape(-1, 28, 3).astype(np.float64); al = d['allowed']
    mx_ = np.max(np.where(al, z, -1e300), 2, keepdims=True); ex = np.exp(np.where(al, z - mx_, -np.inf)); lp = np.log(np.maximum(ex / np.maximum(ex.sum(2, keepdims=True), 1e-300), 1e-12))
    h = d['holder'].astype(np.int64); m = h >= 0; hl = np.take_along_axis(lp, np.maximum(h, 0)[:, :, None], 2)[:, :, 0]
    pick = np.argmax(np.where(al, z, -np.inf), 2); acc = (pick == h) & m
    out = dict(ll=float(-(hl * m).sum() / m.sum()), acc=float(acc.sum() / m.sum()))
    for t in range(7):
        s = d['trick'] == t; mm_ = m[s]
        if mm_.sum(): out[f'll_t{t}'] = float(-(hl[s] * mm_).sum() / mm_.sum()); out[f'acc_t{t}'] = float(acc[s].sum() / mm_.sum())
    return out


def main(argv=None):
    ap = argparse.ArgumentParser()
    ap.add_argument('--train', default=''); ap.add_argument('--val', required=True); ap.add_argument('--val-limit', type=int, default=0); ap.add_argument('--name', default='BEL-x')
    ap.add_argument('--layers', type=int, default=2); ap.add_argument('--hidden', type=int, default=256); ap.add_argument('--hidden2', type=int, default=256)
    ap.add_argument('--init-trunk', default='', help='policy .w (MLP enc 4, same shape) whose w1/b1/wm/bm start the trunk')
    ap.add_argument('--init', default='', help='belief .w to start from (or to evaluate with --eval-only)')
    ap.add_argument('--updates', type=int, default=30000); ap.add_argument('--batch', type=int, default=2048); ap.add_argument('--lr', type=float, default=1e-3); ap.add_argument('--lr-end', type=float, default=1e-5)
    ap.add_argument('--l2', type=float, default=1e-6); ap.add_argument('--seed', type=int, default=4242); ap.add_argument('--eval-every', type=int, default=2000); ap.add_argument('--seconds', type=float, default=240)
    ap.add_argument('--resume', action='store_true'); ap.add_argument('--device', default='gpu', choices=['gpu', 'cpu'])
    ap.add_argument('--eval-only', action='store_true'); ap.add_argument('--dump-logits', default='')
    a = ap.parse_args(argv); import mlx.core as mx; mx.set_default_device(mx.gpu if a.device == 'gpu' else mx.cpu)
    c = dict(layers=a.layers, hidden=a.hidden, hidden2=a.hidden2)
    if a.init: c, p0 = read_w(a.init)
    t0 = time.perf_counter(); va = load(a.val, a.val_limit)
    rng = np.random.default_rng(a.seed); f32 = lambda x: mx.array(np.asarray(x, np.float32)); H = c['hidden']; H2 = c['hidden2'] or H; hout = H2 if c['layers'] == 2 else H
    p = {'w1': f32(rng.standard_normal((T.INPUTS, H)) * np.sqrt(2 / 32)), 'b1': mx.zeros(H)}
    if c['layers'] == 2: p['wm'] = f32(rng.standard_normal((H, H2)) * np.sqrt(2 / H)); p['bm'] = mx.zeros(H2)
    p['w3'] = f32(rng.standard_normal((hout, 84)) * np.sqrt(1 / hout) * .1); p['b3'] = mx.zeros(84)
    if a.init_trunk:
        arch, cfg, w = T.read_w(a.init_trunk); assert arch == 'mlp' and cfg['enc'] == 4 and cfg['layers'] == c['layers'] and cfg['hidden'] == H, cfg
        for k in ('w1', 'b1', 'wm', 'bm'):
            if k in p: p[k] = f32(w[k])
    if a.init: p = {k: f32(v) for k, v in p0.items()}
    ZROW = mx.zeros((1, H)); TR16 = T.E4.tile_rows + 16
    def fwd(pp, idx, sc):
        e = mx.take(mx.concatenate([pp['w1'], ZROW]), idx, axis=0)
        h = mx.maximum(e.sum(1) + sc @ pp['w1'][TR16:TR16 + 3] + pp['b1'], 0)
        if c['layers'] == 2: h = mx.maximum(h @ pp['wm'] + pp['bm'], 0)
        return h @ pp['w3'] + pp['b3']
    def logits(pp, d, CH=16384):
        zs = []
        for c0 in range(0, d['n'], CH):
            zc = fwd(pp, mx.array(d['idx'][c0:c0 + CH].astype(np.int32)), mx.array(d['sc'][c0:c0 + CH])); mx.eval(zc); zs.append(np.array(zc))
        return np.concatenate(zs)
    out = HERE / 'models' / a.name
    if a.eval_only:
        z = logits(p, va); print(json.dumps(dict(init=a.init, rows=va['n'], **metrics(z, va))))
        if a.dump_logits: np.save(a.dump_logits, z)
        return
    tr = load(a.train); N = tr['n']; print(json.dumps(dict(train_rows=N, val_rows=va['n'], load_seconds=round(time.perf_counter() - t0, 1))), flush=True)
    DEV = dict(idx=mx.array(tr['idx']), sc=mx.array(tr['sc']), holder=mx.array(tr['holder']), allowed=mx.array(tr['allowed']))
    def loss(pp, idx, sc, holder, allowed):
        z = fwd(pp, idx, sc).reshape(-1, 28, 3); zm = mx.where(allowed, z, -1e9); lp = zm - mx.logsumexp(zm, axis=2, keepdims=True)
        m = (holder >= 0); hl = mx.take_along_axis(lp, mx.maximum(holder, 0).astype(mx.int32)[:, :, None], axis=2)[:, :, 0]
        ce = -mx.sum(mx.where(m, hl, 0.0)) / mx.sum(m)
        return ce + a.l2 * sum(mx.sum(x ** 2) for k, x in pp.items() if k.startswith('w'))
    vg = mx.value_and_grad(loss); mom = {k: mx.zeros_like(x) for k, x in p.items()}; var = {k: mx.zeros_like(x) for k, x in p.items()}
    step = 0; hist = []; st_path = HERE / 'checkpoints' / f'{a.name}.state.npz'; best = float('inf')
    if a.resume and st_path.exists():
        z0 = np.load(st_path); step = int(z0['step']); best = float(z0['best'])
        for k in p: p[k] = f32(z0['p_' + k]); mom[k] = f32(z0['m_' + k]); var[k] = f32(z0['v_' + k])
        mp = out.with_suffix('.json')
        if mp.exists(): hist = json.loads(mp.read_text()).get('history', [])
    order = np.random.default_rng(a.seed + step).permutation(N); cur = 0; t1 = time.perf_counter(); recent = []
    while step < a.updates:
        if cur + a.batch > N: order = np.random.default_rng(a.seed + step + 1).permutation(N); cur = 0
        j = mx.array(order[cur:cur + a.batch]); cur += a.batch; step += 1
        val, g = vg(p, DEV['idx'][j].astype(mx.int32), DEV['sc'][j], DEV['holder'][j], DEV['allowed'][j])
        lr = a.lr_end + .5 * (a.lr - a.lr_end) * (1 + np.cos(np.pi * step / a.updates))
        for k in p:
            mom[k] = .9 * mom[k] + .1 * g[k]; var[k] = .999 * var[k] + .001 * g[k] * g[k]; p[k] = p[k] - lr * (mom[k] / (1 - .9 ** step)) / (mx.sqrt(var[k] / (1 - .999 ** step)) + 1e-8)
        mx.eval(p, mom, var, val); recent.append(float(val.item()))
        last = step == a.updates or time.perf_counter() - t1 > a.seconds
        if step % a.eval_every == 0 or last:
            m = metrics(logits(p, va), va); hist.append(dict(update=step, train_loss=float(np.mean(recent)), lr=float(lr), **m)); recent = []; print(json.dumps(hist[-1]), flush=True)
            if m['ll'] < best:
                best = m['ll']; pn = {k: np.array(x).astype('<f4') for k, x in p.items()}; write_w(out.with_suffix('.w'), c, pn)
            if last: break
    st_path.parent.mkdir(parents=True, exist_ok=True)
    np.savez(st_path, step=step, best=best, **{'p_' + k: np.array(p[k]) for k in p}, **{'m_' + k: np.array(mom[k]) for k in p}, **{'v_' + k: np.array(var[k]) for k in p})
    meta = dict(name=a.name, args=vars(a), header=list(header(c)), train_rows=N, val_rows=va['n'], global_step=step, best_val_ll=best, history=hist)
    out.with_suffix('.json').write_text(json.dumps(meta, indent=1)); print(json.dumps({k: v for k, v in meta.items() if k not in ('history', 'args')}))


if __name__ == '__main__': main()
