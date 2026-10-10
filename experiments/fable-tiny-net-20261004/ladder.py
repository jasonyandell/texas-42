#!/usr/bin/env python3
"""Fable tiny-net diagnosis: data-scale x capacity x representation grid.

Exploratory tier. Every command must run under
experiments/astra-sol-20261004/tools/run_capped.py with --seconds <= 295.
Teacher, belief surrogate, raw encoding and label kernel are inherited unchanged
from experiments/tiny-net-ladder-20261004 (pilot.py, kernel.c).
"""
import argparse, gzip, hashlib, importlib.util, json, os, random, shutil, sys, time
from pathlib import Path
sys.dont_write_bytecode = True
os.environ.setdefault('OPENBLAS_NUM_THREADS', '2')
import numpy as np

HERE = Path(__file__).resolve().parent
EXP = HERE.parent
LADDER = EXP / 'tiny-net-ladder-20261004'
COVER = EXP / 'tiny-net-coverage-20261004'
_spec = importlib.util.spec_from_file_location('base_pilot', LADDER / 'pilot.py')
p = importlib.util.module_from_spec(_spec); _spec.loader.exec_module(p)
p.HERE = HERE  # kernel.dylib resolves inside this experiment
rules = p.rules
TILES = rules.TILES

SEED = 61010704
TRAIN_DEALS = 24576
TEST_DEALS = 512
BLOCK = 2048
HISTORIES = 8
DATA_ARMS = {'d1': 1536, 'd4': 6144, 'd16': 24576}
ARCHS = {'raw-h32': [862, 32, 28], 'raw-h256': [862, 256, 28], 'raw-d256': [862, 256, 256, 28], 'feat-s64': [32, 64, 64, 1]}
NF = 32
MAX_UPDATES = 6000
EVAL_EVERY = 10  # amendment A1 (pre-freeze): was 50; see plan.json amendments
BATCH = 256
PRIMARY = [f'{arch}-{d}' for arch in ['raw-h32', 'raw-h256', 'raw-d256'] for d in DATA_ARMS]
SECONDARY = [f'feat-s64-{d}' for d in DATA_ARMS] + ['raw-h256-reg-d1']
REGRET_SELECTED = [f'rs-{n}.regret' for n in PRIMARY + [f'feat-s64-{d}' for d in DATA_ARMS]]  # amendment A2


def sha(path): return hashlib.sha256(Path(path).read_bytes()).hexdigest()
def dump(path, x): Path(path).write_text(json.dumps(x, indent=1, sort_keys=False) + '\n')
def readgz(path):
    with gzip.open(path, 'rt') as f: return json.load(f)
def writegz(path, data):
    with gzip.open(path, 'wt') as f: json.dump(data, f)
def canonical(hands): return hashlib.sha256(json.dumps(sorted(p.mask(h) for h in hands)).encode()).hexdigest()
def info(req): return hashlib.sha256(p.encode(req).tobytes()).hexdigest()
def count_value(t):
    s = sum(TILES[t]); return s if s in (5, 10) else 0
def tile_key(t, led, d):
    hi, lo = TILES[t]
    tier = 2 if rules.called(t, d) else 1 if rules.follows(t, led, d) else 0
    return (tier, 12 if hi == lo else hi + lo)


# ----------------------------------------------------------------------------
# Sources and roots
# ----------------------------------------------------------------------------
def forbidden_sources():
    forbid = set()
    for base in (LADDER, COVER):
        forbid |= {s['source_id'] for s in readgz(base / 'sources.json.gz')}
        for f in (base / 'results/h2h').rglob('game-*.json'):
            forbid.add(canonical(json.loads(f.read_text())['hands']))
    return forbid


def prior_information_keys():
    keys = set()
    for base in (LADDER, COVER):
        keys |= {info(r['request']) for r in readgz(base / 'positions.json.gz')}
    return keys


def initialize(a):
    assert not (HERE / 'plan.json').exists(), 'plan already frozen'
    shutil.copyfile(COVER / 'kernel.c', HERE / 'kernel.c')
    assert sha(HERE / 'kernel.c') == sha(LADDER / 'kernel.c')
    forbid = forbidden_sources(); rng = random.Random(SEED); sources = []; collisions = 0
    while len(sources) < TRAIN_DEALS + TEST_DEALS:
        tiles = list(range(28)); rng.shuffle(tiles)
        hands = [sorted(tiles[7 * q:7 * q + 7]) for q in range(4)]
        decl = rng.randrange(7); bidder = rng.randrange(4); sid = canonical(hands)
        if sid in forbid: collisions += 1; continue
        forbid.add(sid); i = len(sources)
        sources.append(dict(index=i, source_id=sid, split='train' if i < TRAIN_DEALS else 'test',
                            block=i // BLOCK if i < TRAIN_DEALS else TRAIN_DEALS // BLOCK, hands=hands, decl=decl, bidder=bidder))
    writegz(HERE / 'sources.json.gz', sources)
    plan = dict(
        tier='exploratory', schema='fable-tiny-net-diagnosis-v1', owner='Claude Fable 5.1 (claude-fable-5-1)',
        base_commit='461028ed8b1ecde27b5fbca7c36d4ca186620e39', seed=SEED,
        question='Does representation capacity, source-deal coverage, label noise, or optimization block distillation of the lawful T0 teacher into a cheap net? Capacity is a candidate, not a default.',
        inherited='teacher T0 = all-action Q under frozen P-1 uniform legal play for every future actor; belief = uniform over capacity-compatible hidden completions given own hand and legal public history (support-conditioned surrogate, NOT the gameplay posterior); raw 862 information-set encoding; kernel.c byte-identical to both prior experiments; fixed bid 30, pip trump 0..6, no bidding model.',
        prior_diagnosis='On the committed coverage validation panel the prior nets mixed32/late32/late64 reach held-out scaled centered-advantage MSE .1476/.1477/.1485 against a zero-predictor MSE .1491 and a 128-world sampling floor of about .004: held-out R^2 about 1%. Training loss falls to <.01. This is a generalization failure at 3,648 roots, not a fit failure; it motivates a learning curve in source deals crossed with capacity.',
        sources=dict(train_deals=TRAIN_DEALS, test_deals=TEST_DEALS, block=BLOCK, histories_per_deal=HISTORIES, roots_per_deal='at most one live nonforced root per absolute-ply band 0-7/8-15/16-23, chosen uniformly by the per-deal trajectory RNG among unique information sets', exclusion='all source partitions of both prior experiments and all prior phone-game deals; all prior actor-normalized inputs; duplicates across this experiment keep first occurrence; test roots duplicating any train root are dropped', collisions_skipped=collisions, test_deals_drawn_now_but_mined_and_labeled_only_after_freeze=True),
        validation='the committed coverage experiment validation panel: 1,024 roots (256/256/512 early/mid/late) over 256 deals, reference0 4,096-world Q and teacher128 labels (coverage batches 012-013). Used for checkpoint selection and validation reporting only.',
        data_arms={k: dict(deals=v, nested=True, note='first v source deals of the frozen train sequence') for k, v in DATA_ARMS.items()},
        labels=dict(worlds=128, domains=dict(train='fable-T0-label-v1', test_teacher='fable-T0-teacher128-v1', test_reference='fable-T0-reference-v1'), reference_worlds=4096, teacher_worlds=128),
        archs={k: dict(sizes=v, kind='per-action feature scorer with shared MLP over 32 lawful features' if k.startswith('feat') else 'dense raw-encoding MLP') for k, v in ARCHS.items()},
        features='32 per-action features computed only from own current hand, public ordered plays, trump, bidder, leader, public scores: trump/double/count/pips, universe rank in context, follows/leading, wins current trick, unseen tiles that beat it, unseen followers, own followers, master flag, own higher, players after, partner played/winning/after, opponent winning, table points, partner/opponent revealed voids in context, unseen/own trumps, team bidding, scores, tricks remaining, hand size, bidder/defender points needed, unseen count, legal count. No hidden hands, no source identity, no labels.',
        optimizer=dict(adam=[.9, .999, 1e-8], lr=.002, l2=1e-5, batch=BATCH, max_updates=MAX_UPDATES, eval_every=EVAL_EVERY, init='first raw layer std .08, hidden std sqrt(2/fan_in), output std .04*sqrt(32/fan_in), zero bias, NumPy seed 12345', selection='minimum validation scaled centered-advantage MSE against the 4,096-world validation reference, evaluated every 50 updates; validation regret is recorded, not used for selection', budget_note='identical maximum update budget for every arm; selected update count and whether selection hit the final evaluation are reported; an arm that selects within the last 10% of the budget is flagged budget-limited'),
        trainings=dict(primary=PRIMARY, secondary=SECONDARY, secondary_reg='raw-h256-reg-d1 = raw-h256 at d1 with lr .0005 and l2 1e-3 as a single optimization/regularization check'),
        hypotheses=dict(
            H_capacity_at_prior_scale='at d1, raw-h256 and raw-d256 do not have paired upper 95% bound < 0 against raw-h32 on fresh test regret (capacity alone does not unblock)',
            H_coverage='raw-h32 at d16 has paired upper 95% bound < 0 against raw-h32 at d1 on fresh test regret (source-deal coverage is a blocker)',
            H_capacity_x_data='at d16, raw-d256 versus raw-h32 paired difference; direction not preregistered, reported',
            H_representation='feat-s64 at d1 versus raw-h32 at d1 paired; feat-s64 at d16 versus raw-d256 at d16 paired; secondary',
            noise='bounded, not retested: the 128-world scaled sampling floor is about .004 against a .149 zero-predictor MSE; the prior subset512 control showed no decision gain'),
        metrics='regret = max_a Q_ref(a) - Q_ref(chosen) in acting-partnership contract-win probability under the stated surrogate teacher, all-band and late-band; equal-weight means per source deal; 5,000-draw source-deal bootstrap; best-set and resolved (gap > 3 paired SE, heuristic) agreement; held-out R^2 of scaled centered advantage; warm CPU latency (network-only, validated decision); parameters and bytes; training and label wall seconds. Not multiplicity corrected. Teacher-relative, not optimal-game regret.',
        rung='no frozen-policy rung is run in this phase under any outcome; the rung decision returns to Jason with the diagnosis',
        resources='every invocation under run_capped.py <= 295 s process-group cap; at most two concurrent label workers; local MLX Metal only; no cloud, no installs, no transmissions, no production, no fixture export')
    dump(HERE / 'plan.json', plan)
    print(json.dumps(dict(sources=len(sources), collisions=collisions, blocks=sorted({s['block'] for s in sources}))))


def mine(a):
    out = HERE / 'positions' / f'block-{a.block:03d}.json.gz'; out.parent.mkdir(exist_ok=True); assert not out.exists()
    sources = [s for s in readgz(HERE / 'sources.json.gz') if s['block'] == a.block]; assert sources
    rows = []; stats = []; start = time.perf_counter()
    for s in sources:
        si = s['index']; hands = s['hands']; decl = s['decl']; bidder = s['bidder']
        trng = random.Random(SEED ^ ((si + 1) * 104729)); candidates = [{}, {}, {}]
        for _ in range(HISTORIES):
            remain = list(map(set, hands)); leader = bidder; pts = [0, 0]; plays = []; tr = []
            for ply in range(28):
                if pts[bidder % 2] >= 30 or pts[1 - bidder % 2] >= 13: break
                actor = (leader + len(tr)) % 4; legal = rules.legal_tiles(remain[actor], tr, decl)
                if len(legal) > 1 and ply < 24:
                    req = dict(decl=decl, bid=30, bidder=bidder, seat=actor, hand=hands[actor], plays=plays.copy(), seed=7042104)
                    candidates[ply // 8].setdefault(info(req), req)
                t = trng.choice(legal); remain[actor].remove(t); plays.extend([actor, t]); tr.append((actor, t))
                if len(tr) == 4: leader = rules.winner(tr, decl); pts[leader % 2] += rules.trick_points(tr); tr = []
        stats.append(dict(source_index=si, unique_candidates_by_band=[len(c) for c in candidates]))
        for band, c in enumerate(candidates):
            if not c: continue
            lst = list(c.items()); trng.shuffle(lst); key, req = lst[0]
            rows.append(dict(source_index=si, source_id=s['source_id'], split=s['split'], block=a.block, band=band, request=req, information_key=key))
    writegz(out, rows); dump(out.with_suffix('').with_suffix('.stats.json'), dict(block=a.block, deals=len(sources), rows=len(rows), seconds=time.perf_counter() - start, bands=[sum(r['band'] == b for r in rows) for b in range(3)], mining=stats))
    print(json.dumps(dict(block=a.block, deals=len(sources), rows=len(rows), seconds=time.perf_counter() - start)))


def consolidate(a):
    old = prior_information_keys(); seen = set(); rows = []; dropped = 0
    if a.split == 'train':
        blocks = range(TRAIN_DEALS // BLOCK); out = HERE / 'positions.json.gz'
    else:
        assert (HERE / 'models/FROZEN.json').exists(), 'freeze all models before touching test roots'
        old |= {r['information_key'] for r in readgz(HERE / 'positions.json.gz')}; blocks = [TRAIN_DEALS // BLOCK]; out = HERE / 'positions-test.json.gz'
    assert not out.exists()
    for b in blocks:
        for r in readgz(HERE / 'positions' / f'block-{b:03d}.json.gz'):
            k = r['information_key']
            if k in seen or k in old: dropped += 1; continue
            seen.add(k); r['index'] = len(rows); rows.append(r)
    writegz(out, rows)
    print(json.dumps(dict(split=a.split, rows=len(rows), dropped_duplicates=dropped, deals=len({r['source_id'] for r in rows}), bands=[sum(r['band'] == b for r in rows) for b in range(3)])))


def positions(split='train'):
    return readgz(HERE / ('positions.json.gz' if split == 'train' else 'positions-test.json.gz'))


# ----------------------------------------------------------------------------
# Labels (inherited kernel) and lawful per-action features
# ----------------------------------------------------------------------------
def batch_rows(kind, batch):
    if kind == 'train': return [r for r in positions('train') if r['block'] == batch]
    rows = positions('test'); return [r for r in rows if (r['source_index'] - TRAIN_DEALS) // 128 == batch]


def generate(a):
    rows = batch_rows('train' if a.kind == 'train' else 'test', a.batch); assert rows
    out = HERE / 'data' / a.kind; out.mkdir(parents=True, exist_ok=True); path = out / f'batch-{a.batch:03d}.npz'; assert not path.exists()
    W = {'train': 128, 'test-teacher128': 128, 'test-reference': 4096}[a.kind]
    domain = {'train': 'fable-T0-label-v1', 'test-teacher128': 'fable-T0-teacher128-v1', 'test-reference': 'fable-T0-reference-v1'}[a.kind]
    X = []; M = []; Q = []; B = []; ids = []; record = []; start = time.perf_counter(); conts = 0
    for r in rows:
        seed = int.from_bytes(hashlib.sha256(f"{domain}:{r['information_key']}".encode()).digest()[:8], 'little')
        acts, y, ws, tp = p.label(r['request'], W, seed, None)
        m = np.zeros(28, np.float32); m[acts] = 1; q = np.zeros(28, np.float32); q[acts] = y.mean(0)
        b = np.zeros((28, W // 8), np.uint8); b[acts] = np.packbits(y.T, axis=1); conts += len(acts) * W
        X.append(p.encode(r['request'])); M.append(m); Q.append(q); B.append(b); ids.append(r['index'])
        record.append(dict(position_index=r['index'], source_id=r['source_id'], worlds=W, seed=seed, worlds_sha256=hashlib.sha256(ws.tobytes()).hexdigest(), tapes_sha256=hashlib.sha256(tp.tobytes()).hexdigest()))
    np.savez_compressed(path, X=np.array(X, np.float32), M=np.array(M, np.float32), Q=np.array(Q, np.float32), B=np.array(B, np.uint8), ids=np.array(ids, np.int32), W=np.array(W))
    meta = dict(kind=a.kind, batch=a.batch, positions=len(rows), worlds=W, root_worlds=len(rows) * W, candidate_continuations=conts, seconds=time.perf_counter() - start, kernel_sha256=sha(HERE / 'kernel.c'), binary_sha256=sha(HERE / 'kernel.dylib'), base_pilot_sha256=sha(LADDER / 'pilot.py'), ladder_sha256=sha(__file__), dataset_sha256=sha(path), records=record)
    dump(path.with_suffix('.json'), meta); print(json.dumps({k: meta[k] for k in ['kind', 'batch', 'positions', 'root_worlds', 'candidate_continuations', 'seconds']}))


def featurize(req):
    """Lawful per-action features: own current hand + public record only."""
    s, played, current = p.public(req)
    d = req['decl']; me = req['seat']; partner = (me + 2) % 4; bidder = req['bidder']
    played_set = {t for q, t in played}; npl = len(played) % 4; table = played[len(played) - npl:] if npl else []
    unseen = set(range(28)) - current - played_set
    void = [set() for _ in range(4)]; tr = []
    for q, t in played:
        if tr:
            led0 = rules.context(tr[0][1], d)
            if not rules.follows(t, led0, d): void[q].update(z for z in range(28) if rules.follows(z, led0, d))
        tr.append((q, t))
        if len(tr) == 4: tr = []
    leader = s['leader']; order = [(leader + k) % 4 for k in range(4)]; after = order[npl + 1:]
    pts = s['points']; own_pts = pts[me % 2]; opp_pts = pts[1 - me % 2]; bid_pts = pts[bidder % 2]; def_pts = pts[1 - bidder % 2]
    led = rules.context(table[0][1], d) if table else None
    table_pts = sum(count_value(t) for q, t in table)
    best = max(table, key=lambda qt: tile_key(qt[1], led, d)) if table else None
    partner_winning = bool(table) and best[0] == partner
    opp_winning = bool(table) and best[0] % 2 != me % 2
    partner_played = any(q == partner for q, t in table)
    n_unseen_tr = sum(rules.called(t, d) for t in unseen); n_own_tr = sum(rules.called(t, d) for t in current)
    tricks_remaining = 8 - s['trick']
    F = np.zeros((28, NF), np.float32)
    for t in s['legal']:
        ctx = led if led is not None else rules.context(t, d); key = tile_key(t, ctx, d)
        n_beats = sum(1 for u in unseen if tile_key(u, ctx, d) > key)
        follow_universe = {u for u in range(28) if rules.follows(u, ctx, d)}
        n_follow_unseen = len(follow_universe & unseen); own_ctx = [o for o in current if o in follow_universe]
        n_own_higher = sum(1 for o in own_ctx if tile_key(o, ctx, d) > key)
        wins_now = 1.0 if (not table or key > tile_key(best[1], ctx, d)) else 0.0
        own_c = rules.context(t, d); own_follow_universe = {u for u in range(28) if rules.follows(u, own_c, d)}
        universe_higher = sum(1 for u in own_follow_universe if tile_key(u, own_c, d) > tile_key(t, own_c, d))
        partner_after = partner in after
        partner_void = 1.0 if (partner_after and (follow_universe & void[partner])) else 0.0
        opp_void = 1.0 if any((follow_universe & void[q]) for q in after if q % 2 != me % 2) else 0.0
        F[t] = [rules.called(t, d), TILES[t][0] == TILES[t][1], count_value(t) / 10, sum(TILES[t]) / 12, universe_higher / 7,
                1.0 if (led is None or rules.follows(t, led, d)) else 0.0, 1.0 if led is None else 0.0, wins_now, n_beats / 21, n_follow_unseen / 21,
                len(own_ctx) / 7, 1.0 if n_beats == 0 else 0.0, n_own_higher / 7, len(after) / 3, partner_played, partner_winning, opp_winning, table_pts / 10,
                partner_void, opp_void, n_unseen_tr / 7, n_own_tr / 7, 1.0 if me % 2 == bidder % 2 else 0.0, own_pts / 42, opp_pts / 42, tricks_remaining / 7,
                len(current) / 7, max(0, 30 - bid_pts) / 30, max(0, 13 - def_pts) / 13, 1.0 if partner_after else 0.0, len(unseen) / 21, len(s['legal']) / 7]
    return F


def features(a):
    rows = batch_rows('train' if a.kind == 'train' else 'test', a.batch); assert rows
    out = HERE / 'features' / a.kind; out.mkdir(parents=True, exist_ok=True); path = out / f'batch-{a.batch:03d}.npz'; assert not path.exists()
    start = time.perf_counter(); F = np.array([featurize(r['request']) for r in rows], np.float16); ids = np.array([r['index'] for r in rows], np.int32)
    np.savez_compressed(path, F=F, ids=ids)
    dump(path.with_suffix('.json'), dict(kind=a.kind, batch=a.batch, positions=len(rows), seconds=time.perf_counter() - start, nf=NF, ladder_sha256=sha(__file__), dataset_sha256=sha(path)))
    print(json.dumps(dict(kind=a.kind, batch=a.batch, positions=len(rows), seconds=time.perf_counter() - start)))


# ----------------------------------------------------------------------------
# Datasets
# ----------------------------------------------------------------------------
def load_dir(path, keys=('X', 'M', 'Q', 'B', 'ids')):
    files = sorted(Path(path).glob('batch-*.npz')); assert files, path
    parts = [dict(np.load(f)) for f in files]
    return {k: np.concatenate([d[k] for d in parts]) for k in keys}, int(parts[0]['W']) if 'W' in parts[0] else None


def coverage_rows():
    return readgz(COVER / 'positions.json.gz')


def validation_panel(arch):
    """Committed coverage validation panel: 4,096-world reference Q, teacher128 choices."""
    ref, W = load_dir(COVER / 'data/reference0'); rows = coverage_rows()
    sel = np.array([j for j, i in enumerate(ref['ids']) if rows[int(i)]['split'] == 'validation'])
    panel = dict(ids=ref['ids'][sel], X=ref['X'][sel], M=ref['M'][sel], Q=ref['Q'][sel], B=ref['B'][sel], W=W,
                 band=np.array([rows[int(i)]['band'] for i in ref['ids'][sel]]), source=np.array([rows[int(i)]['source_id'] for i in ref['ids'][sel]]),
                 requests=[rows[int(i)]['request'] for i in ref['ids'][sel]])
    t, _ = load_dir(COVER / 'data/teacher128', keys=('M', 'Q', 'ids')); look = {int(i): j for j, i in enumerate(t['ids'])}
    panel['teacher'] = np.array([int(np.argmax(np.where(t['M'][look[int(i)]] > 0, t['Q'][look[int(i)]], -np.inf))) for i in panel['ids']])
    if arch.startswith('feat'): panel['F'] = np.array([featurize(r) for r in panel['requests']], np.float32)
    return panel


def test_panel(arch):
    ref, W = load_dir(HERE / 'data/test-reference'); rows = positions('test'); look_rows = {r['index']: r for r in rows}
    panel = dict(ids=ref['ids'], X=ref['X'], M=ref['M'], Q=ref['Q'], B=ref['B'], W=W,
                 band=np.array([look_rows[int(i)]['band'] for i in ref['ids']]), source=np.array([look_rows[int(i)]['source_id'] for i in ref['ids']]),
                 requests=[look_rows[int(i)]['request'] for i in ref['ids']])
    t, _ = load_dir(HERE / 'data/test-teacher128', keys=('M', 'Q', 'ids')); look = {int(i): j for j, i in enumerate(t['ids'])}
    panel['teacher'] = np.array([int(np.argmax(np.where(t['M'][look[int(i)]] > 0, t['Q'][look[int(i)]], -np.inf))) for i in panel['ids']])
    if arch.startswith('feat'):
        f, _ = load_dir(HERE / 'features/test', keys=('F', 'ids')); lf = {int(i): j for j, i in enumerate(f['ids'])}
        panel['F'] = f['F'][[lf[int(i)] for i in panel['ids']]].astype(np.float32)
    return panel


def train_data(arch, deals):
    data, W = load_dir(HERE / 'data/train'); rows = positions('train'); src = np.array([rows[int(i)]['source_index'] for i in data['ids']])
    keep = np.flatnonzero(src < deals); out = {k: data[k][keep] for k in ['X', 'M', 'Q', 'ids']}; out['W'] = W; out['deals'] = int(len(np.unique(src[keep])))
    if arch.startswith('feat'):
        f, _ = load_dir(HERE / 'features/train', keys=('F', 'ids')); assert np.array_equal(f['ids'], data['ids']); out['F'] = f['F'][keep]
    return out


# ----------------------------------------------------------------------------
# Models
# ----------------------------------------------------------------------------
def init_params(arch, rng):
    sizes = ARCHS[arch]; params = {}
    for i, (fi, fo) in enumerate(zip(sizes[:-1], sizes[1:])):
        last = i == len(sizes) - 2
        std = .04 * np.sqrt(32 / fi) if last else (.08 if (i == 0 and arch.startswith('raw')) else np.sqrt(2 / fi))
        params[f'w{i}'] = (rng.standard_normal((fi, fo)) * std).astype(np.float32); params[f'b{i}'] = np.zeros(fo, np.float32)
    return params


def forward_np(params, x):
    n = len(params) // 2; h = x
    for i in range(n):
        h = h @ params[f'w{i}'] + params[f'b{i}']
        if i < n - 1: h = np.maximum(h, 0)
    return h[..., 0] if h.shape[-1] == 1 else h


def model_input(arch, panel): return panel['F'] if arch.startswith('feat') else panel['X']


def choose(arch, params, panel):
    z = forward_np(params, model_input(arch, panel)); return np.argmax(np.where(panel['M'] > 0, z, -np.inf), 1)


def load_model(path):
    path = Path(path); meta = json.loads(path.with_suffix('.json').read_text()); params = dict(np.load(path))
    if 'w1' in params and 'w0' not in params:  # committed prior-experiment format (862->H->28)
        params = {'w0': params['w1'], 'b0': params['b1'], 'w1': params['w2'], 'b1': params['b2']}
        meta = dict(meta, arch='raw-h32' if params['w0'].shape[1] == 32 else 'raw-h64', data_arm='prior-committed', selected_update=meta.get('selected_epoch'))
    return meta['arch'], params, meta


def train(a):
    import mlx.core as mx
    mx.set_default_device(mx.gpu)
    arch = a.arch; deals = DATA_ARMS[a.deals]; out = HERE / 'models' / a.name; out.parent.mkdir(exist_ok=True); assert not out.with_suffix('.npz').exists()
    t0 = time.perf_counter(); data = train_data(arch, deals); val = validation_panel(arch); load_s = time.perf_counter() - t0
    rng = np.random.default_rng(12345); params = {k: mx.array(v) for k, v in init_params(arch, rng).items()}
    Xtr = mx.array(model_input(arch, data).astype(np.float32)); Mtr = mx.array(data['M']); Qtr = mx.array(data['Q'])
    ntr = mx.sum(Mtr, axis=1, keepdims=True); Ytr = 4 * (Qtr - mx.sum(Qtr * Mtr, axis=1, keepdims=True) / ntr) * Mtr
    Xva = mx.array(model_input(arch, val).astype(np.float32)); Mva = mx.array(val['M']); Qva = mx.array(val['Q'])
    nva = mx.sum(Mva, axis=1, keepdims=True); Yva = 4 * (Qva - mx.sum(Qva * Mva, axis=1, keepdims=True) / nva) * Mva
    nlayers = len(params) // 2

    def fwd(pp, x):
        h = x
        for i in range(nlayers):
            h = h @ pp[f'w{i}'] + pp[f'b{i}']
            if i < nlayers - 1: h = mx.maximum(h, 0)
        return h[..., 0] if arch.startswith('feat') else h

    def centered(z, m, n): return z - mx.sum(z * m, axis=1, keepdims=True) / n

    def mse(pp, x, m, y, n): return mx.mean(mx.sum((centered(fwd(pp, x), m, n) - y) ** 2 * m, axis=1) / n[:, 0])

    def loss(pp, x, m, y, n): return mse(pp, x, m, y, n) + a.l2 * sum(mx.sum(v * v) for v in pp.values())

    vg = mx.value_and_grad(loss); mom = {k: mx.zeros_like(v) for k, v in params.items()}; var = {k: mx.zeros_like(v) for k, v in params.items()}
    N = len(data['ids']); order = rng.permutation(N); cursor = 0; step = 0; best = float('inf'); history = []; recent = []; selected = None; t1 = time.perf_counter()
    best_regret = float('inf'); regret_selected = None
    vq = val['Q']; vm = val['M']; vbest = np.max(np.where(vm > 0, vq, -np.inf), 1)
    while step < a.updates:
        if cursor + BATCH > N: order = rng.permutation(N); cursor = 0
        j = mx.array(order[cursor:cursor + BATCH]); cursor += BATCH
        v, g = vg(params, Xtr[j], Mtr[j], Ytr[j], ntr[j]); step += 1
        for k in params:
            mom[k] = .9 * mom[k] + .1 * g[k]; var[k] = .999 * var[k] + .001 * g[k] * g[k]
            params[k] = params[k] - a.lr * (mom[k] / (1 - .9 ** step)) / (mx.sqrt(var[k] / (1 - .999 ** step)) + 1e-8)
        mx.eval(params, mom, var, v); recent.append(float(v.item()))
        if step % EVAL_EVERY == 0 or step == a.updates:
            vl = float(mse(params, Xva, Mva, Yva, nva).item()); pn = {k: np.array(v_) for k, v_ in params.items()}
            c = choose(arch, pn, val); regret = float(np.mean(vbest - vq[np.arange(len(vq)), c]))
            history.append(dict(update=step, train_loss=float(np.mean(recent)), validation_mse=vl, validation_regret=regret)); recent = []
            if vl < best: best = vl; selected = step; np.savez(out.with_suffix('.npz'), **pn)
            if a.also_regret and regret < best_regret: best_regret = regret; regret_selected = step; np.savez(out.with_suffix('.regret.npz'), **pn)
    train_s = time.perf_counter() - t1; pn = dict(np.load(out.with_suffix('.npz')))
    zero = float(np.mean(np.sum(np.array(Yva) ** 2 * val['M'], 1) / val['M'].sum(1)))
    meta = dict(name=a.name, arch=arch, sizes=ARCHS[arch], data_arm=a.deals, deals_requested=deals, deals_present=data['deals'], train_roots=int(N), validation_roots=int(len(vq)), device='MLX Metal GPU',
                lr=a.lr, l2=a.l2, batch=BATCH, max_updates=a.updates, eval_every=EVAL_EVERY, selected_update=selected, budget_limited=selected is not None and selected > .9 * a.updates,
                selected_validation_mse=best, validation_zero_predictor_mse=zero, selected_validation_r2=1 - best / zero,
                selected_validation_regret=next(h['validation_regret'] for h in history if h['update'] == selected), epochs_equivalent=a.updates * BATCH / N,
                load_seconds=load_s, train_loop_seconds=train_s, parameter_count=int(sum(v.size for v in pn.values())), raw_float32_bytes=int(4 * sum(v.size for v in pn.values())),
                file_bytes=out.with_suffix('.npz').stat().st_size, weights_sha256=sha(out.with_suffix('.npz')), history=history)
    dump(out.with_suffix('.json'), meta); print(json.dumps({k: v for k, v in meta.items() if k != 'history'}))
    if a.also_regret:  # amendment A2: validation-regret-selected checkpoint, post hoc on validation, declared before freeze/test
        rmeta = dict(meta, name=a.name + '.regret', selection='amendment A2: minimum validation regret over the 10-update grid (post hoc on validation; optimistic by construction)', selected_update=regret_selected, selected_validation_regret=best_regret,
                     selected_validation_mse=next(h['validation_mse'] for h in history if h['update'] == regret_selected), budget_limited=regret_selected > .9 * a.updates, weights_sha256=sha(out.with_suffix('.regret.npz')), file_bytes=out.with_suffix('.regret.npz').stat().st_size)
        rmeta['selected_validation_r2'] = 1 - rmeta['selected_validation_mse'] / zero; dump(out.with_suffix('.regret.json'), rmeta)


def freeze(a):
    assert not (HERE / 'models/FROZEN.json').exists()
    assert not (HERE / 'positions-test.json.gz').exists() and not (HERE / 'data/test-reference').exists()
    models = {}
    for name in PRIMARY + SECONDARY + REGRET_SELECTED:
        meta = json.loads((HERE / 'models' / f'{name}.json').read_text())
        models[name] = dict(weights_sha256=sha(HERE / 'models' / f'{name}.npz'), arch=meta['arch'], data_arm=meta['data_arm'], selected_update=meta['selected_update'], budget_limited=meta['budget_limited'], selected_validation_mse=meta['selected_validation_mse'], selected_validation_regret=meta['selected_validation_regret'])
    dump(HERE / 'models/FROZEN.json', dict(tier='exploratory', frozen_utc=time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime()), rule='all arms frozen by validation MSE before any test root is mined, labeled or read', models=models))
    print(json.dumps(models))


# ----------------------------------------------------------------------------
# Evaluation
# ----------------------------------------------------------------------------
def ci(v, clusters, seed=61010799, draws=5000):
    keys, inv = np.unique(clusters, return_inverse=True); sums = np.bincount(inv, weights=v); cnt = np.bincount(inv); means = sums / cnt
    rng = np.random.default_rng(seed); boot = means[rng.integers(len(means), size=(draws, len(means)))].mean(1)
    return dict(mean=float(means.mean()), ci95=[float(x) for x in np.quantile(boot, [.025, .975])], source_deals=int(len(keys)))


def evaluate(a):
    folder = HERE / 'results' / a.output; folder.mkdir(parents=True, exist_ok=False)
    models = [load_model(m) for m in a.models]; archs = {arch for arch, _, _ in models}
    panel = (test_panel if a.split == 'test' else validation_panel)('feat-s64' if any(x.startswith('feat') for x in archs) else 'raw-h32')
    sel = np.arange(len(panel['ids'])) if a.band == 'all' else np.flatnonzero(panel['band'] == int(a.band))
    sub = {k: (panel[k][sel] if isinstance(panel[k], np.ndarray) else panel[k]) for k in panel}
    m = sub['M']; q = sub['Q']; n = m.sum(1); clusters = sub['source']; best = np.argmax(np.where(m > 0, q, -np.inf), 1); v = q[np.arange(len(q)), best]
    Y = np.unpackbits(sub['B'], axis=2)[:, :, :sub['W']].astype(np.float32); second = np.argsort(np.where(m > 0, q, -np.inf), 1)[:, -2]
    gap = v - q[np.arange(len(q)), second]; ds = Y[np.arange(len(q)), best] - Y[np.arange(len(q)), second]; se = ds.std(1, ddof=1) / np.sqrt(sub['W']); resolved = gap > 3 * se
    y = 4 * (q - np.sum(q * m, 1, keepdims=True) / n[:, None]) * m; zero = np.sum(y ** 2 * m, 1) / n
    choices = {'ascending': np.argmax(m, 1), 'highest_tile': 27 - np.argmax(m[:, ::-1], 1), 'teacher128': sub['teacher']}; r2 = {}; ident = {}
    for (arch, params, meta), path in zip(models, a.models):
        name = Path(path).stem; z = forward_np(params, model_input(arch, sub)); choices[name] = np.argmax(np.where(m > 0, z, -np.inf), 1)
        zc = (z - np.sum(z * m, 1, keepdims=True) / n[:, None]) * m; mse = np.sum((zc - y) ** 2 * m, 1) / n; r2[name] = dict(mse=float(mse.mean()), zero_mse=float(zero.mean()), r2=float(1 - mse.mean() / zero.mean()))
        ident[name] = dict(path=str(Path(path).resolve().relative_to(HERE.parent.parent)), sha256=sha(path), arch=arch, data_arm=meta.get('data_arm'), parameters=meta.get('parameter_count'), selected_update=meta.get('selected_update'))
    regret = {'uniform_random': v - (q * m).sum(1) / n}; regret.update({k: v - q[np.arange(len(q)), c] for k, c in choices.items()})
    metrics = {}
    for name, r in regret.items():
        e = ci(r, clusters); e.update(root_mean=float(r.mean()), p95=float(np.quantile(r, .95)), maximum=float(r.max()))
        if name in choices: e.update(best_set_agreement=float(np.mean(r == 0)), resolved_best_set_agreement=float(np.mean(r[resolved] == 0)) if resolved.any() else None, vs_random=ci(r - regret['uniform_random'], clusters))
        if name in r2: e.update(held_out_fit=r2[name])
        metrics[name] = e
    names = [Path(x).stem for x in a.models]; pairs = {}
    for i, na in enumerate(names):
        for nb in names[i + 1:]: pairs[f'{na}-minus-{nb}'] = ci(regret[na] - regret[nb], clusters)
    for na in names: pairs[f'{na}-minus-mixed32'] = ci(regret[na] - regret['mixed32'], clusters) if 'mixed32' in regret else None
    span = metrics['uniform_random']['mean'] - metrics['teacher128']['mean']
    retained = {na: float((metrics['uniform_random']['mean'] - metrics[na]['mean']) / span) for na in names}
    result = dict(tier='exploratory', split=a.split, band=a.band, roots=int(len(sel)), source_deals=int(len(np.unique(clusters))), reference_worlds=sub['W'], resolved_gap_roots=int(resolved.sum()), metrics=metrics, paired_differences=pairs, retained_fraction_of_random_to_teacher_reduction=retained, models=ident,
                  qualification='teacher-relative decision regret under the support-conditioned surrogate belief and uniform-play T0 continuation; noisy finite-sample max reference; source-deal bootstrap; empirical 3 SE gap heuristic; no multiplicity correction; exploratory tier')
    details = [dict(position_index=int(i), source_id=str(clusters[j]), band=int(sub['band'][j]), legal=np.flatnonzero(m[j]).tolist(), Q=q[j].tolist(), best=int(best[j]), gap=float(gap[j]), resolved=bool(resolved[j]), choices={k: int(c[j]) for k, c in choices.items()}) for j, i in enumerate(sub['ids'])]
    dump(folder / 'summary.json', result); dump(folder / 'details.json', details); print(json.dumps(dict(split=a.split, band=a.band, roots=result['roots'], means={k: round(e['mean'], 6) for k, e in metrics.items()}, retained=retained)))


def benchmark(a):
    arch, params, meta = load_model(a.model); rows = coverage_rows(); reqs = [r['request'] for r in rows if r['split'] == 'validation' and r['band'] == 2][:128]
    enc = featurize if arch.startswith('feat') else p.encode; X = np.array([enc(r) for r in reqs], np.float32); forward = []; total = []; forward_np(params, X)
    for _ in range(8):
        for req, x in zip(reqs, X):
            t = time.perf_counter_ns(); forward_np(params, x[None]); forward.append((time.perf_counter_ns() - t) / 1000)
            t = time.perf_counter_ns(); s = p.public(req)[0]; z = forward_np(params, enc(req)[None])[0]; max(s['legal'], key=lambda tt: (z[tt], -tt)); total.append((time.perf_counter_ns() - t) / 1000)
    out = dict(model=Path(a.model).name, sha256=sha(a.model), arch=arch, parameters=meta['parameter_count'], raw_float32_bytes=meta['raw_float32_bytes'], file_bytes=meta['file_bytes'], device='local CPU NumPy, warm, sequential; excludes loading/startup', samples=len(forward),
               network_us=dict(median=float(np.median(forward)), p95=float(np.quantile(forward, .95))), validated_decision_us=dict(median=float(np.median(total)), p95=float(np.quantile(total, .95))))
    (HERE / 'results').mkdir(exist_ok=True); dump(HERE / 'results' / f'latency-{Path(a.model).stem}.json', out); print(json.dumps(out))


def main():
    ap = argparse.ArgumentParser(); sub = ap.add_subparsers(dest='cmd', required=True)
    sub.add_parser('initialize')
    s = sub.add_parser('mine'); s.add_argument('--block', type=int, required=True)
    s = sub.add_parser('consolidate'); s.add_argument('--split', choices=['train', 'test'], required=True)
    s = sub.add_parser('generate'); s.add_argument('--kind', choices=['train', 'test-teacher128', 'test-reference'], required=True); s.add_argument('--batch', type=int, required=True)
    s = sub.add_parser('features'); s.add_argument('--kind', choices=['train', 'test'], required=True); s.add_argument('--batch', type=int, required=True)
    s = sub.add_parser('train'); s.add_argument('--arch', choices=list(ARCHS), required=True); s.add_argument('--deals', choices=list(DATA_ARMS), required=True); s.add_argument('--name', required=True); s.add_argument('--lr', type=float, default=.002); s.add_argument('--l2', type=float, default=1e-5); s.add_argument('--updates', type=int, default=MAX_UPDATES); s.add_argument('--also-regret', action='store_true')
    sub.add_parser('freeze')
    s = sub.add_parser('evaluate'); s.add_argument('--split', choices=['validation', 'test'], required=True); s.add_argument('--band', default='all', choices=['all', '0', '1', '2']); s.add_argument('--models', nargs='+', required=True); s.add_argument('--output', required=True)
    s = sub.add_parser('benchmark'); s.add_argument('--model', required=True)
    a = ap.parse_args(); globals()[a.cmd](a)


if __name__ == '__main__':
    main()
