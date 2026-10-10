#!/usr/bin/env python3
"""Inner-level (Walt level-0 mind) full-vector distillation. Exploratory.
Every command runs under experiments/astra-sol-20261004/tools/run_capped.py (<=295 s).
Teacher = the verified seam adapter (see STATUS.md); V0 control = the pilots' kernel.
"""
import argparse, gzip, hashlib, importlib.util, json, os, random, subprocess, sys, time
from pathlib import Path
sys.dont_write_bytecode = True
os.environ.setdefault('OPENBLAS_NUM_THREADS', '2')
import numpy as np
HERE = Path(__file__).resolve().parent; EXP = HERE.parent
_s = importlib.util.spec_from_file_location('ladder', EXP / 'fable-tiny-net-20261004/ladder.py'); L = importlib.util.module_from_spec(_s); _s.loader.exec_module(L)
p = L.p; rules = L.rules
SEAM = HERE / 'adapter/target-speedups/release/inner-vector-seam'
SEED = 61010804; BLOCKS = [('train', 2048), ('train', 2048), ('train', 2048), ('validation', 512), ('test', 512)]
HISTORIES = 8; KREF = 16; W0 = 128
ARCHS = L.ARCHS; BATCH = 256; MAX_UPDATES = 6000; EVAL_EVERY = 10
TARGETS = {'inner': 'exact production level-0 vector k/8 (acting-side oriented)', 'ref': 'declared reference: mean of 16 ref-seeded 8-world bundles (k/128)', 'v0': 'pilots V0: uniform all-future, 128 common worlds/tapes (acting-side)'}
ARMS = [('inner-h32', 'raw-h32', 'inner', 'bce'), ('inner-h256', 'raw-h256', 'inner', 'bce'), ('inner-d256', 'raw-d256', 'inner', 'bce'),
        ('inner-h32-centered', 'raw-h32', 'inner', 'centered'), ('v0-h32', 'raw-h32', 'v0', 'bce'), ('v0-h256', 'raw-h256', 'v0', 'bce'),
        ('ref-h32', 'raw-h32', 'ref', 'bce'), ('inner-feat', 'feat-s64', 'inner', 'bce')]
sha = L.sha; dump = L.dump; readgz = L.readgz; writegz = L.writegz; info = L.info; canonical = L.canonical

def forbidden():
    f = L.forbidden_sources() | {s['source_id'] for s in readgz(EXP / 'fable-tiny-net-20261004/sources.json.gz')}
    return f
def prior_keys():
    k = L.prior_information_keys()
    for name in ['positions.json.gz', 'positions-test.json.gz']: k |= {r['information_key'] for r in readgz(EXP / 'fable-tiny-net-20261004' / name)}
    return k

def initialize(a):
    assert not (HERE / 'plan.json').exists()
    forbid = forbidden(); rng = random.Random(SEED); sources = []; collisions = 0
    for b, (split, n) in enumerate(BLOCKS):
        made = 0
        while made < n:
            tiles = list(range(28)); rng.shuffle(tiles); hands = [sorted(tiles[7 * q:7 * q + 7]) for q in range(4)]
            decl = rng.randrange(7); bidder = rng.randrange(4); sid = canonical(hands)
            if sid in forbid: collisions += 1; continue
            forbid.add(sid); sources.append(dict(index=len(sources), source_id=sid, split=split, block=b, hands=hands, decl=decl, bidder=bidder)); made += 1
    writegz(HERE / 'sources.json.gz', sources)
    seam = json.loads((HERE / 'results/seam-check.json').read_text())
    plan = dict(tier='exploratory', schema='fable-inner-vector-v1', owner='Claude Fable 5.1 (claude-fable-5-1)', base_commit='4daafce0', seed=SEED,
        hypothesis='the corrected teacher boundary (Walt level-0 mind: own future committed per public node over an 8-world Voidless bundle, others seeded dice) versus the pilots V0 (all-future uniform), on full legal vectors with a separate deterministic selector',
        teacher=dict(name='WALT-INNER-L0MIND-n8-DICE-VOIDLESS-FIXED-BID30-STRAIGHT-DECLMAKE-v1', source='walt/walt/src/solver/mod.rs Solver::pi(0) generic tail; adapter/src/lib.rs; pinned cb1ef3b2 identical at HEAD', perspective='declarer-make k/8 exact; declarers maximize, defenders minimize, lowest tile on exact ties', belief='Voidless shuffle ignoring public voids (superset of support; information-lawful; not a posterior)', others='Field::Dice seeded uniform legal per world and node', own_future='bundle recursion (committed per public node, bundle-coupled)', seedless='deterministic function of (seat, remaining hand, public key); request seed unused', differential=dict(positions=seam['positions'], production_agree=seam['production_agree'], cross_build_identical=seam['cross_build_identical'], receipt='results/seam-check-log/run.json')),
        reference=dict(name='…-refseed-v1', definition='16 bundles with an extra mixed term in the production RNG seed; mean counts over 128 worlds; declared evaluation/secondary-target estimator, not production'),
        v0_control='the pilots T0/V0 kernel (fable-tiny-net-20261004/kernel.c, byte-identical to the ladder pilot), 128 common worlds/tapes, acting-partnership outcome',
        sources=dict(blocks=BLOCKS, deals=len(sources), histories_per_deal=HISTORIES, roots='at most one live nonforced root per absolute-ply band 0-7/8-15/16-23 per deal, actor = whichever seat is on turn (all seats covered)', exclusion='all source partitions and all actor-normalized inputs of tiny-net-ladder, tiny-net-coverage, fable-tiny-net and every phone game', collisions_skipped=collisions, test='block 4 mined and labeled only after freeze'),
        encoding='raw 862 own-current-hand/public encoding of the pilots (primary); 32 lawful per-action features (diagnostic arm only)',
        orientation='net targets are acting-side success s = p_make for declaring seats, 1 - p_make for defenders; p_make is recovered exactly as s or 1 - s and reported; selector = legal argmax of the acting-side score, lowest tile on exact ties',
        arms=[dict(name=n, arch=ar, target=t, loss=l) for n, ar, t, l in ARMS], targets=TARGETS,
        losses=dict(bce='masked soft-target binary cross-entropy on sigmoid outputs against k/N (a proper scoring rule; not claimed to be an exact binomial likelihood for bundle-coupled counts)', centered='the pilots loss: masked MSE on 4*(s - mean_legal s) with legally centered outputs; cannot be read as probabilities'),
        optimizer=dict(adam=[.9, .999, 1e-8], lr=.002, l2=1e-5, batch=BATCH, max_updates=MAX_UPDATES, eval_every=EVAL_EVERY, init='as fable-tiny-net', selection='minimum validation loss of the arm own loss against the arm own target; validation regret and fidelity recorded, not used'),
        gates=dict(G1_teacher='exact single-bundle choice vs 16-bundle majority agreement, overall and on resolved (ref gap > 3 paired-free SE) roots; reported, sets the fidelity ceiling',
                   G2_fidelity='primary: inner-h32 chosen tile in the exact teacher tie set on fresh test; secondary: exact-choice agreement, vector RMSE vs k/8, calibration (10-bin ECE), paired action-gap error; compare arms by source-deal bootstrap',
                   G2_hypothesis='inner-h32 vs v0-h32 and inner-h256 vs v0-h256 on the SAME reference regret (ref-mean, acting side): the teacher-boundary effect; inner-h32 vs inner-h32-centered: loss effect; inner-h256/d256 vs inner-h32: capacity',
                   G3_outer_substitution='NOT run in this phase; requires the net inside the Rust recursion; stated as undone',
                   G4_cost='inner call latency net vs seam (about 22 us median) on the same positions; parameters, bytes'),
        resources='every run under run_capped.py <=295 s, two label workers max, local MLX Metal, no cloud/installs/transmissions/production/export')
    dump(HERE / 'plan.json', plan); print(json.dumps(dict(sources=len(sources), collisions=collisions)))

def mine(a):
    out = HERE / 'positions' / f'block-{a.block:03d}.json.gz'; out.parent.mkdir(exist_ok=True); assert not out.exists()
    sources = [s for s in readgz(HERE / 'sources.json.gz') if s['block'] == a.block]; assert sources
    if BLOCKS[a.block][0] == 'test': assert (HERE / 'models/FROZEN.json').exists(), 'freeze before mining test'
    rows = []; start = time.perf_counter()
    for s in sources:
        si = s['index']; hands = s['hands']; decl = s['decl']; bidder = s['bidder']; trng = random.Random(SEED ^ ((si + 1) * 104729)); cands = [{}, {}, {}]
        for _ in range(HISTORIES):
            remain = list(map(set, hands)); leader = bidder; pts = [0, 0]; plays = []; tr = []
            for ply in range(28):
                if pts[bidder % 2] >= 30 or pts[1 - bidder % 2] >= 13: break
                actor = (leader + len(tr)) % 4; legal = rules.legal_tiles(remain[actor], tr, decl)
                if len(legal) > 1 and ply < 24:
                    req = dict(decl=decl, bid=30, bidder=bidder, seat=actor, hand=hands[actor], plays=plays.copy(), seed=7042104); cands[ply // 8].setdefault(info(req), req)
                t = trng.choice(legal); remain[actor].remove(t); plays.extend([actor, t]); tr.append((actor, t))
                if len(tr) == 4: leader = rules.winner(tr, decl); pts[leader % 2] += rules.trick_points(tr); tr = []
        for band, c in enumerate(cands):
            if not c: continue
            lst = list(c.items()); trng.shuffle(lst); key, req = lst[0]
            rows.append(dict(source_index=si, source_id=s['source_id'], split=s['split'], block=a.block, band=band, request=req, information_key=key))
    writegz(out, rows); print(json.dumps(dict(block=a.block, deals=len(sources), rows=len(rows), seconds=time.perf_counter() - start)))

def consolidate(a):
    old = prior_keys(); seen = set(); rows = []; dropped = 0
    if a.split == 'test':
        assert (HERE / 'models/FROZEN.json').exists(); old |= {r['information_key'] for r in readgz(HERE / 'positions.json.gz')}; blocks = [4]; out = HERE / 'positions-test.json.gz'
    else: blocks = [0, 1, 2, 3]; out = HERE / 'positions.json.gz'
    assert not out.exists()
    for b in blocks:
        for r in readgz(HERE / 'positions' / f'block-{b:03d}.json.gz'):
            k = r['information_key']
            if k in seen or k in old: dropped += 1; continue
            seen.add(k); r['index'] = len(rows); rows.append(r)
    writegz(out, rows); print(json.dumps(dict(split=a.split, rows=len(rows), dropped=dropped, deals=len({r['source_id'] for r in rows}), bands=[sum(r['band'] == b for r in rows) for b in range(3)], seats=[sum(r['request']['seat'] == q for r in rows) for q in range(4)])))

def positions(split='train'): return readgz(HERE / ('positions-test.json.gz' if split == 'test' else 'positions.json.gz'))

class Seam:
    def __init__(self): self.p = subprocess.Popen([str(SEAM)], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, bufsize=1)
    def call(self, req, ref_seed=None, check=False):
        q = {'request': req, 'budget_ms': 2000, 'check': check}
        if ref_seed is not None: q['ref_seed'] = ref_seed
        self.p.stdin.write(json.dumps(q) + '\n'); self.p.stdin.flush(); r = json.loads(self.p.stdout.readline()); assert 'error' not in r and 'ineligible' not in r, r; return r
    def close(self): self.p.stdin.close(); self.p.wait(timeout=10)

def generate(a):
    rows = [r for r in positions('test' if a.kind == 'test' else 'train') if r['block'] == a.block]; assert rows
    out = HERE / 'data' / a.kind; out.mkdir(parents=True, exist_ok=True); path = out / f'batch-{a.block:03d}.npz'; assert not path.exists()
    seam = Seam(); X = []; M = []; K8 = []; KR = []; RCH = []; Q0 = []; B0 = []; ids = []; DEC = []; CH = []; AGREE = 0; rec = []; start = time.perf_counter(); t_seam = 0; t_v0 = 0
    for r in rows:
        req = r['request']; t0 = time.perf_counter(); ex = seam.call(req, check=True); AGREE += ex['agree']
        legal = ex['legal']; k8 = np.zeros(28, np.float32); k8[legal] = ex['counts']; kr = np.zeros(28, np.float32); rch = []
        for s in range(1, KREF + 1):
            rr = seam.call(req, ref_seed=s); assert rr['legal'] == legal; kr[legal] += rr['counts']; rch.append(rr['choice'])
        t_seam += time.perf_counter() - t0; t0 = time.perf_counter()
        seed = int.from_bytes(hashlib.sha256(f"fable-inner-V0-v1:{r['information_key']}".encode()).digest()[:8], 'little'); acts, y, ws, tp = p.label(req, W0, seed, None)
        assert list(acts) == legal; q0 = np.zeros(28, np.float32); q0[acts] = y.mean(0); b0 = np.zeros((28, W0 // 8), np.uint8); b0[acts] = np.packbits(y.T, axis=1); t_v0 += time.perf_counter() - t0
        m = np.zeros(28, np.float32); m[legal] = 1
        X.append(p.encode(req)); M.append(m); K8.append(k8); KR.append(kr); RCH.append(rch); Q0.append(q0); B0.append(b0); ids.append(r['index']); DEC.append(ex['declaring']); CH.append(ex['choice'])
        rec.append(dict(position_index=r['index'], source_id=r['source_id'], v0_seed=seed, v0_worlds_sha256=hashlib.sha256(ws.tobytes()).hexdigest()))
    seam.close()
    np.savez_compressed(path, X=np.array(X, np.float32), M=np.array(M, np.float32), K8=np.array(K8, np.float32), KR=np.array(KR, np.float32), RCH=np.array(RCH, np.int16), Q0=np.array(Q0, np.float32), B0=np.array(B0, np.uint8), ids=np.array(ids, np.int32), DEC=np.array(DEC, bool), CH=np.array(CH, np.int16), W0=np.array(W0), KREF=np.array(KREF))
    meta = dict(kind=a.kind, block=a.block, positions=len(rows), production_agree=AGREE, seam_seconds=t_seam, v0_seconds=t_v0, seconds=time.perf_counter() - start, seam_sha256=sha(SEAM), kernel_sha256=sha(EXP / 'fable-tiny-net-20261004/kernel.c'), inner_sha256=sha(__file__), dataset_sha256=sha(path), records=rec)
    dump(path.with_suffix('.json'), meta); print(json.dumps({k: meta[k] for k in ['kind', 'block', 'positions', 'production_agree', 'seam_seconds', 'v0_seconds', 'seconds']}))

def features(a):
    rows = [r for r in positions('test' if a.kind == 'test' else 'train') if r['block'] == a.block]; out = HERE / 'features' / a.kind; out.mkdir(parents=True, exist_ok=True); path = out / f'batch-{a.block:03d}.npz'; assert not path.exists()
    F = np.array([L.featurize(r['request']) for r in rows], np.float16); np.savez_compressed(path, F=F, ids=np.array([r['index'] for r in rows], np.int32)); print(json.dumps(dict(kind=a.kind, block=a.block, positions=len(rows))))

def load_dir(path, keys):
    files = sorted(Path(path).glob('batch-*.npz')); assert files, path; parts = [dict(np.load(f)) for f in files]
    return {k: np.concatenate([d[k] for d in parts]) for k in keys}

KEYS = ('X', 'M', 'K8', 'KR', 'RCH', 'Q0', 'ids', 'DEC', 'CH')
def dataset(split):
    kind = 'test' if split == 'test' else 'train'; d = load_dir(HERE / 'data' / kind, KEYS); rows = positions(kind); look = {r['index']: r for r in rows}
    sel = np.array([j for j, i in enumerate(d['ids']) if look[int(i)]['split'] == split]); d = {k: v[sel] for k, v in d.items()}
    d['source'] = np.array([look[int(i)]['source_id'] for i in d['ids']]); d['band'] = np.array([look[int(i)]['band'] for i in d['ids']]); d['requests'] = [look[int(i)]['request'] for i in d['ids']]
    dec = d['DEC'][:, None]; m = d['M']
    d['s_inner'] = np.where(dec, d['K8'] / 8, 1 - d['K8'] / 8) * m; d['s_ref'] = np.where(dec, d['KR'] / (8 * KREF), 1 - d['KR'] / (8 * KREF)) * m; d['s_v0'] = d['Q0'] * m
    return d
def with_features(d, split):
    kind = 'test' if split == 'test' else 'train'; f = load_dir(HERE / 'features' / kind, ('F', 'ids')); lf = {int(i): j for j, i in enumerate(f['ids'])}; d['F'] = f['F'][[lf[int(i)] for i in d['ids']]].astype(np.float32); return d
def inputs(arch, d): return d['F'] if arch.startswith('feat') else d['X']
def target(t, d): return d[{'inner': 's_inner', 'ref': 's_ref', 'v0': 's_v0'}[t]]

def scores(arch, params, d): return L.forward_np(params, inputs(arch, d))
def choose(z, m): return np.argmax(np.where(m > 0, z, -np.inf), 1)

def train(a):
    import mlx.core as mx; mx.set_default_device(mx.gpu)
    name, arch, tgt, loss_kind = next(x for x in ARMS if x[0] == a.name); out = HERE / 'models' / name; out.parent.mkdir(exist_ok=True); assert not out.with_suffix('.npz').exists()
    t0 = time.perf_counter(); tr = dataset('train'); va = dataset('validation')
    if arch.startswith('feat'): tr = with_features(tr, 'train'); va = with_features(va, 'validation')
    load_s = time.perf_counter() - t0; rng = np.random.default_rng(12345); params = {k: mx.array(v) for k, v in L.init_params(arch, rng).items()}; nl = len(params) // 2
    Xtr = mx.array(inputs(arch, tr)); Mtr = mx.array(tr['M']); Ttr = mx.array(target(tgt, tr)); Xva = mx.array(inputs(arch, va)); Mva = mx.array(va['M']); Tva = mx.array(target(tgt, va))
    def fwd(pp, x):
        h = x
        for i in range(nl):
            h = h @ pp[f'w{i}'] + pp[f'b{i}']
            if i < nl - 1: h = mx.maximum(h, 0)
        return h[..., 0] if arch.startswith('feat') else h
    def loss_fn(pp, x, m, t):
        z = fwd(pp, x); n = mx.sum(m, axis=1)
        if loss_kind == 'bce':
            per = mx.logaddexp(0, z) - t * z  # softplus(z) - t z = BCE with logits
            core = mx.mean(mx.sum(per * m, axis=1) / n)
        else:
            zc = z - mx.sum(z * m, axis=1, keepdims=True) / n[:, None]; y = 4 * (t - mx.sum(t * m, axis=1, keepdims=True) / n[:, None]) * m
            core = mx.mean(mx.sum((zc - y) ** 2 * m, axis=1) / n)
        return core + a.l2 * sum(mx.sum(v * v) for v in pp.values())
    def vloss(pp): return float((loss_fn(pp, Xva, Mva, Tva) - a.l2 * sum(mx.sum(v * v) for v in pp.values())).item())
    vg = mx.value_and_grad(loss_fn); mom = {k: mx.zeros_like(v) for k, v in params.items()}; var = {k: mx.zeros_like(v) for k, v in params.items()}
    N = len(tr['ids']); order = rng.permutation(N); cur = 0; step = 0; best = float('inf'); hist = []; recent = []; sel = None; t1 = time.perf_counter()
    qref = va['s_ref']; vbest = np.max(np.where(va['M'] > 0, qref, -np.inf), 1); exact_set = (va['K8'] == np.where(va['DEC'][:, None], va['K8'].max(1, keepdims=True), np.where(va['M'] > 0, va['K8'], 99).min(1, keepdims=True))) & (va['M'] > 0)
    while step < a.updates:
        if cur + BATCH > N: order = rng.permutation(N); cur = 0
        j = mx.array(order[cur:cur + BATCH]); cur += BATCH; v, g = vg(params, Xtr[j], Mtr[j], Ttr[j]); step += 1
        for k in params:
            mom[k] = .9 * mom[k] + .1 * g[k]; var[k] = .999 * var[k] + .001 * g[k] * g[k]; params[k] = params[k] - a.lr * (mom[k] / (1 - .9 ** step)) / (mx.sqrt(var[k] / (1 - .999 ** step)) + 1e-8)
        mx.eval(params, mom, var, v); recent.append(float(v.item()))
        if step % EVAL_EVERY == 0 or step == a.updates:
            vl = vloss(params); pn = {k: np.array(v_) for k, v_ in params.items()}; c = choose(scores(arch, pn, va), va['M']); ar = np.arange(len(c))
            hist.append(dict(update=step, train_loss=float(np.mean(recent)), validation_loss=vl, validation_regret_ref=float(np.mean(vbest - qref[ar, c])), validation_in_exact_tie_set=float(np.mean(exact_set[ar, c])), validation_exact_agree=float(np.mean(c == va['CH'])))); recent = []
            if vl < best: best = vl; sel = step; np.savez(out.with_suffix('.npz'), **pn)
    pn = dict(np.load(out.with_suffix('.npz'))); h = next(x for x in hist if x['update'] == sel)
    meta = dict(name=name, arch=arch, target=tgt, loss=loss_kind, sizes=ARCHS[arch], train_roots=int(N), validation_roots=int(len(va['ids'])), lr=a.lr, l2=a.l2, batch=BATCH, max_updates=a.updates, eval_every=EVAL_EVERY, selected_update=sel, budget_limited=sel > .9 * a.updates, selected_validation_loss=best, selected_validation_regret_ref=h['validation_regret_ref'], selected_validation_in_exact_tie_set=h['validation_in_exact_tie_set'], selected_validation_exact_agree=h['validation_exact_agree'], epochs_equivalent=a.updates * BATCH / N, load_seconds=load_s, train_loop_seconds=time.perf_counter() - t1, parameter_count=int(sum(v.size for v in pn.values())), raw_float32_bytes=int(4 * sum(v.size for v in pn.values())), file_bytes=out.with_suffix('.npz').stat().st_size, weights_sha256=sha(out.with_suffix('.npz')), device='MLX Metal GPU', history=hist)
    dump(out.with_suffix('.json'), meta); print(json.dumps({k: v for k, v in meta.items() if k != 'history'}))

def freeze(a):
    assert not (HERE / 'models/FROZEN.json').exists() and not (HERE / 'positions-test.json.gz').exists()
    models = {n: dict(weights_sha256=sha(HERE / 'models' / f'{n}.npz'), **{k: json.loads((HERE / 'models' / f'{n}.json').read_text())[k] for k in ['arch', 'target', 'loss', 'selected_update', 'budget_limited', 'selected_validation_loss', 'selected_validation_regret_ref']}) for n, *_ in ARMS}
    dump(HERE / 'models/FROZEN.json', dict(tier='exploratory', frozen_utc=time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime()), rule='all arms frozen by their own validation loss before the test block is mined, labeled or read', models=models)); print(json.dumps(models))

def ci(v, clusters, seed=61010899, draws=5000):
    keys, inv = np.unique(clusters, return_inverse=True); means = np.bincount(inv, weights=v) / np.bincount(inv); rng = np.random.default_rng(seed); boot = means[rng.integers(len(means), size=(draws, len(means)))].mean(1)
    return dict(mean=float(means.mean()), ci95=[float(x) for x in np.quantile(boot, [.025, .975])], source_deals=int(len(keys)))

def evaluate(a):
    folder = HERE / 'results' / a.output; folder.mkdir(parents=True, exist_ok=False); d = dataset(a.split)
    if any(json.loads((HERE / 'models' / f'{n}.json').read_text())['arch'].startswith('feat') for n in a.models): d = with_features(d, a.split)
    if a.band != 'all':
        sel = np.flatnonzero(d['band'] == int(a.band)); d = {k: (v[sel] if isinstance(v, np.ndarray) else [v[i] for i in sel]) for k, v in d.items()}
    m = d['M']; n = m.sum(1); ar = np.arange(len(m)); cl = d['source']; qref = d['s_ref']; q8 = d['s_inner']; vbest = np.max(np.where(m > 0, qref, -np.inf), 1)
    exact_best = np.where(m > 0, q8, -np.inf).max(1, keepdims=True); exact_set = (np.abs(q8 - exact_best) < 1e-6) & (m > 0)
    second = np.sort(np.where(m > 0, qref, -np.inf), 1)[:, -2]; gap = vbest - second; se = np.sqrt(np.maximum(vbest * (1 - vbest) + second * (1 - second), 1e-9) / (8 * KREF)); resolved = gap > 3 * se
    rch = d['RCH']; maj = np.array([np.bincount(r, minlength=28).argmax() for r in rch]); majfrac = np.array([np.bincount(r, minlength=28).max() / KREF for r in rch])
    choices = {'ascending': np.argmax(m, 1), 'highest_tile': 27 - np.argmax(m[:, ::-1], 1), 'teacher_exact': d['CH'].astype(int), 'ref_majority': maj, 'ref_mean_argmax': choose(qref, m)}
    vec = {}; ident = {}
    for name in a.models:
        meta = json.loads((HERE / 'models' / f'{name}.json').read_text()); pn = dict(np.load(HERE / 'models' / f'{name}.npz')); z = scores(meta['arch'], pn, d); choices[name] = choose(z, m); ident[name] = dict(sha256=meta['weights_sha256'], arch=meta['arch'], target=meta['target'], loss=meta['loss'], parameters=meta['parameter_count'], selected_update=meta['selected_update'])
        if meta['loss'] == 'bce':
            pr = 1 / (1 + np.exp(-z)); err8 = (pr - q8) ** 2 * m; errr = (pr - qref) ** 2 * m; bins = np.minimum((pr * 10).astype(int), 9); ece = 0.0; tot = m.sum()
            for b in range(10):
                w = (bins == b) & (m > 0)
                if w.any(): ece += w.sum() / tot * abs(pr[w].mean() - q8[w].mean())
            gp = pr[ar, choices['ref_mean_argmax']] - pr[ar, choices['ascending']]; gt = qref[ar, choices['ref_mean_argmax']] - qref[ar, choices['ascending']]
            vec[name] = dict(rmse_vs_exact_k8=float(np.sqrt(err8.sum() / tot)), rmse_vs_ref_mean=float(np.sqrt(errr.sum() / tot)), ece10_vs_exact=float(ece), mean_abs_gap_error_vs_ref=float(np.mean(np.abs(gp - gt))), mean_pred=float((pr * m).sum() / tot), mean_target=float((q8 * m).sum() / tot))
    regret = {'uniform_random': vbest - (qref * m).sum(1) / n}; regret.update({k: vbest - qref[ar, c] for k, c in choices.items()}); metrics = {}
    for k, r in regret.items():
        e = ci(r, cl); e.update(root_mean=float(r.mean()))
        if k in choices: c = choices[k]; e.update(in_exact_tie_set=float(np.mean(exact_set[ar, c])), exact_agree=float(np.mean(c == d['CH'])), resolved_in_exact_tie_set=float(np.mean(exact_set[ar, c][resolved])) if resolved.any() else None, ref_majority_agree=float(np.mean(c == maj)), vs_random=ci(r - regret['uniform_random'], cl))
        if k in vec: e['vector'] = vec[k]
        metrics[k] = e
    pairs = {f'{x}-minus-{y}': ci(regret[x] - regret[y], cl) for i, x in enumerate(a.models) for y in a.models[i + 1:]}
    span = metrics['uniform_random']['mean'] - metrics['teacher_exact']['mean']; retained = {x: float((metrics['uniform_random']['mean'] - metrics[x]['mean']) / span) for x in a.models}
    teacher = dict(exact_vs_ref_majority_agree=float(np.mean(d['CH'] == maj)), exact_vs_ref_majority_agree_resolved=float(np.mean((d['CH'] == maj)[resolved])) if resolved.any() else None, exact_top_tie_fraction=float(np.mean(exact_set.sum(1) > 1)), ref_majority_mean_fraction=float(majfrac.mean()), resolved_roots=int(resolved.sum()), v0_argmax_in_exact_tie_set=float(np.mean(exact_set[ar, choose(d['s_v0'], m)])), v0_rank_regret_ref=float(np.mean(vbest - qref[ar, choose(d['s_v0'], m)])))
    res = dict(tier='exploratory', split=a.split, band=a.band, roots=int(len(m)), source_deals=int(len(np.unique(cl))), reference='mean of 16 ref-seeded 8-world bundles (128 worlds), acting-side', metrics=metrics, paired_differences=pairs, retained_fraction_random_to_exact_teacher=retained, teacher_noise=teacher, models=ident, qualification='teacher-relative fidelity to a named model estimator under a Voidless belief; not posterior, not strength; source-deal bootstrap, no multiplicity correction')
    dump(folder / 'summary.json', res); dump(folder / 'details.json', [dict(position_index=int(i), source_id=str(cl[j]), band=int(d['band'][j]), legal=np.flatnonzero(m[j]).tolist(), k8=d['K8'][j][m[j] > 0].tolist(), kref=d['KR'][j][m[j] > 0].tolist(), declaring=bool(d['DEC'][j]), choices={k: int(c[j]) for k, c in choices.items()}) for j, i in enumerate(d['ids'])])
    print(json.dumps(dict(split=a.split, band=a.band, roots=res['roots'], regret={k: round(e['mean'], 5) for k, e in metrics.items()}, tie_set={k: round(e['in_exact_tie_set'], 4) for k, e in metrics.items() if 'in_exact_tie_set' in e}, retained={k: round(v, 3) for k, v in retained.items()}, teacher=teacher)))

def benchmark(a):
    meta = json.loads((HERE / 'models' / f'{a.model}.json').read_text()); pn = dict(np.load(HERE / 'models' / f'{a.model}.npz')); arch = meta['arch']; reqs = [r['request'] for r in positions('train') if r['split'] == 'validation'][:128]
    enc = L.featurize if arch.startswith('feat') else p.encode; X = np.array([enc(r) for r in reqs], np.float32); fw = []; tot = []; L.forward_np(pn, X); seam = Seam(); sm = []
    for _ in range(8):
        for req, x in zip(reqs, X):
            t = time.perf_counter_ns(); L.forward_np(pn, x[None]); fw.append((time.perf_counter_ns() - t) / 1000)
            t = time.perf_counter_ns(); s = p.public(req)[0]; z = L.forward_np(pn, enc(req)[None])[0]; max(s['legal'], key=lambda tt: (z[tt], -tt)); tot.append((time.perf_counter_ns() - t) / 1000)
            t = time.perf_counter_ns(); seam.call(req); sm.append((time.perf_counter_ns() - t) / 1000)
    seam.close(); q = lambda v: dict(median=float(np.median(v)), p95=float(np.quantile(v, .95)))
    out = dict(model=a.model, sha256=meta['weights_sha256'], parameters=meta['parameter_count'], raw_float32_bytes=meta['raw_float32_bytes'], samples=len(fw), network_us=q(fw), validated_decision_us=q(tot), seam_roundtrip_us=q(sm), note='warm sequential CPU NumPy; seam round trip includes JSON pipe transport (in-process seam median about 22 us)')
    (HERE / 'results').mkdir(exist_ok=True); dump(HERE / 'results' / f'latency-{a.model}.json', out); print(json.dumps(out))

def main():
    ap = argparse.ArgumentParser(); sub = ap.add_subparsers(dest='cmd', required=True); sub.add_parser('initialize')
    s = sub.add_parser('mine'); s.add_argument('--block', type=int, required=True)
    s = sub.add_parser('consolidate'); s.add_argument('--split', choices=['train', 'test'], required=True)
    s = sub.add_parser('generate'); s.add_argument('--kind', choices=['train', 'test'], required=True); s.add_argument('--block', type=int, required=True)
    s = sub.add_parser('features'); s.add_argument('--kind', choices=['train', 'test'], required=True); s.add_argument('--block', type=int, required=True)
    s = sub.add_parser('train'); s.add_argument('--name', required=True); s.add_argument('--lr', type=float, default=.002); s.add_argument('--l2', type=float, default=1e-5); s.add_argument('--updates', type=int, default=MAX_UPDATES)
    sub.add_parser('freeze')
    s = sub.add_parser('evaluate'); s.add_argument('--split', choices=['validation', 'test'], required=True); s.add_argument('--band', default='all'); s.add_argument('--models', nargs='+', required=True); s.add_argument('--output', required=True)
    s = sub.add_parser('benchmark'); s.add_argument('--model', required=True)
    a = ap.parse_args(); globals()[a.cmd](a)
if __name__ == '__main__': main()
