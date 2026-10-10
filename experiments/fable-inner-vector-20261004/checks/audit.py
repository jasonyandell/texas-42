#!/usr/bin/env python3
"""Independent re-check of the inner-vector pilot evidence (run under run_capped.py)."""
import datetime, hashlib, importlib.util, json, subprocess, sys
from pathlib import Path
import numpy as np
sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parents[1]; EXP = HERE.parent
spec = importlib.util.spec_from_file_location('inner', HERE / 'inner.py'); I = importlib.util.module_from_spec(spec); spec.loader.exec_module(I)
out = {}
train = I.positions('train'); test = I.positions('test')
prior = I.forbidden(); ts = {r['source_id'] for r in test}; trs = {r['source_id'] for r in train}
assert not (ts & prior) and not (ts & trs) and not (trs & prior); out['sources'] = dict(train=len(trs), test=len(ts), prior=len(prior), disjoint=True)
pk = I.prior_keys(); tk = {r['information_key'] for r in test}; trk = {r['information_key'] for r in train}
assert len(tk) == len(test) and len(trk) == len(train) and not (tk & trk) and not (tk & pk) and not (trk & pk)
assert all(r['information_key'] == I.info(r['request']) for r in test); out['information_sets'] = dict(train=len(trk), test=len(tk), disjoint=True)
receipts = {str(f.relative_to(HERE)): json.loads(f.read_text()) for f in HERE.rglob('run.json')}
assert all(r['allowance_seconds'] <= 295 and r['elapsed_seconds'] <= 300 for r in receipts.values())
out['receipts'] = dict(count=len(receipts), statuses={s: sum(r['status'] == s for r in receipts.values()) for s in sorted({r['status'] for r in receipts.values()})}, largest_elapsed_seconds=max(r['elapsed_seconds'] for r in receipts.values()))
frozen = json.loads((HERE / 'models/FROZEN.json').read_text())
for n, m in frozen['models'].items(): assert I.sha(HERE / 'models' / f'{n}.npz') == m['weights_sha256'], n
fz = datetime.datetime.strptime(frozen['frozen_utc'], '%Y-%m-%dT%H:%M:%SZ').replace(tzinfo=datetime.timezone.utc)
tr = {k: datetime.datetime.fromisoformat(r['started_utc']) for k, r in receipts.items() if any(t in k for t in ['mine-004', 'consolidate-test', 'test-labels', 'test-features', 'test-all-log', 'test-late-log'])}
assert tr and all(t >= fz for t in tr.values()); out['freeze'] = dict(models=len(frozen['models']), hashes_match=True, frozen_utc=frozen['frozen_utc'], earliest_test_receipt=min(tr.values()).isoformat(), precedes_all_test_receipts=True)
# Independent replay of test labels through BOTH seam builds and the production fast path.
d = I.load_dir(HERE / 'data/test', I.KEYS); look = {r['index']: r for r in test}; ref = HERE / 'adapter/target-reference/release/inner-vector-seam'
idx = np.arange(0, len(d['ids']), 37); replay = dict(roots=int(len(idx)), exact_identical=0, production_agree=0, reference_build_identical=0, ref_seed_identical=0)
sp = subprocess.Popen([str(I.SEAM)], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, bufsize=1); rp = subprocess.Popen([str(ref)], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, bufsize=1)
def call(proc, q): proc.stdin.write(json.dumps(q) + '\n'); proc.stdin.flush(); return json.loads(proc.stdout.readline())
for j in idx:
    req = look[int(d['ids'][j])]['request']; a = call(sp, dict(request=req, check=True, budget_ms=2000)); b = call(rp, dict(request=req, budget_ms=2000)); m = d['M'][j] > 0
    k8 = np.zeros(28); k8[a['legal']] = a['counts']; replay['exact_identical'] += int(np.array_equal(k8[m], d['K8'][j][m]) and a['choice'] == d['CH'][j] and a['declaring'] == bool(d['DEC'][j]))
    replay['production_agree'] += int(bool(a['agree'])); replay['reference_build_identical'] += int(a['values'] == b['values'])
    kr = np.zeros(28)
    for s in range(1, I.KREF + 1):
        r = call(sp, dict(request=req, ref_seed=s, budget_ms=2000)); kr[r['legal']] += r['counts']
    replay['ref_seed_identical'] += int(np.array_equal(kr[m], d['KR'][j][m]))
for pr in (sp, rp): pr.stdin.close(); pr.wait(timeout=10)
assert replay['exact_identical'] == replay['production_agree'] == replay['reference_build_identical'] == replay['ref_seed_identical'] == len(idx); out['label_replay'] = replay
# Referee legality of every test root and every teacher choice; declaring flag against seat parity.
rules = I.rules; bad = 0
for j, i in enumerate(d['ids']):
    req = look[int(i)]['request']; s = rules.information_state(req); legal = np.flatnonzero(d['M'][j]).tolist()
    bad += int(legal != s['legal'] or int(d['CH'][j]) not in s['legal'] or bool(d['DEC'][j]) != (req['seat'] % 2 == req['bidder'] % 2))
assert bad == 0; out['referee'] = dict(test_roots=int(len(d['ids'])), legal_and_perspective_consistent=True)
# One headline mean recomputed from details.
s = json.load(open(HERE / 'results/test-all/summary.json')); det = json.load(open(HERE / 'results/test-all/details.json'))
reg = []; src = []
for x in det:
    kref = dict(zip(x['legal'], x['kref'])); q = {t: (v if x['declaring'] else 128 - v) / 128 for t, v in kref.items()}; reg.append(max(q.values()) - q[x['choices']['inner-h32']]); src.append(x['source_id'])
reg = np.array(reg); src = np.array(src); means = np.array([reg[src == k].mean() for k in np.unique(src)]); assert abs(means.mean() - s['metrics']['inner-h32']['mean']) < 1e-9
out['metric_recomputed'] = dict(model='inner-h32', mean=float(means.mean()))
I.dump(HERE / 'checks/audit-result.json', out); print(json.dumps(out))
