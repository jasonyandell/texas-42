#!/usr/bin/env python3
"""Independent re-check of the Fable diagnosis evidence. Run under run_capped.py.

Checks: fresh test sources disjoint from every prior source and from train;
test information sets disjoint from train and both prior experiments; every
receipt within the 300 s ceiling; frozen hashes match model files; freeze
precedes every test receipt; a sample of test reference labels replays
byte-identically from the kernel; feature lawfulness (features are a function
of the request only and ignore hidden hands by construction: verified by
recomputing features for test roots and comparing with the stored arrays).
"""
import datetime, hashlib, importlib.util, json, sys
from pathlib import Path
import numpy as np
sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('ladder', HERE / 'ladder.py'); L = importlib.util.module_from_spec(spec); spec.loader.exec_module(L)
out = {}
train_rows = L.positions('train'); test_rows = L.positions('test')
prior_sources = L.forbidden_sources(); train_sources = {r['source_id'] for r in train_rows}; test_sources = {r['source_id'] for r in test_rows}
assert not (test_sources & prior_sources) and not (test_sources & train_sources) and not (train_sources & prior_sources)
out['sources'] = dict(train=len(train_sources), test=len(test_sources), prior=len(prior_sources), disjoint=True)
prior_keys = L.prior_information_keys(); train_keys = {r['information_key'] for r in train_rows}; test_keys = {r['information_key'] for r in test_rows}
assert len(train_keys) == len(train_rows) and len(test_keys) == len(test_rows)
assert not (test_keys & train_keys) and not (test_keys & prior_keys) and not (train_keys & prior_keys)
assert all(r['information_key'] == L.info(r['request']) for r in test_rows)
out['information_sets'] = dict(train=len(train_keys), test=len(test_keys), disjoint=True, test_keys_recomputed=True)
receipts = {str(f.relative_to(HERE)): json.loads(f.read_text()) for f in HERE.rglob('run.json')}
assert all(r['allowance_seconds'] <= 295 and r['elapsed_seconds'] <= 300 for r in receipts.values())
out['receipts'] = dict(count=len(receipts), statuses={s: sum(r['status'] == s for r in receipts.values()) for s in sorted({r['status'] for r in receipts.values()})}, largest_elapsed_seconds=max(r['elapsed_seconds'] for r in receipts.values()), largest_allowance=max(r['allowance_seconds'] for r in receipts.values()))
frozen = json.loads((HERE / 'models/FROZEN.json').read_text())
for name, m in frozen['models'].items(): assert L.sha(HERE / 'models' / f'{name}.npz') == m['weights_sha256'], name
freeze_utc = datetime.datetime.strptime(frozen['frozen_utc'], '%Y-%m-%dT%H:%M:%SZ').replace(tzinfo=datetime.timezone.utc)
test_receipts = {k: datetime.datetime.fromisoformat(r['started_utc']) for k, r in receipts.items() if any(t in k for t in ['mine-012', 'consolidate-test', 'test-teacher', 'test-reference', 'test-features', 'test-all-log', 'test-late-log'])}
assert test_receipts and all(t >= freeze_utc for t in test_receipts.values())
out['freeze'] = dict(models=len(frozen['models']), hashes_match=True, frozen_utc=frozen['frozen_utc'], earliest_test_receipt_utc=min(test_receipts.values()).isoformat(), freeze_precedes_all_test_receipts=True)
ref, W = L.load_dir(HERE / 'data/test-reference'); meta = json.loads((HERE / 'data/test-reference/batch-000.json').read_text()); look = {int(i): j for j, i in enumerate(ref['ids'])}; rows = {r['index']: r for r in test_rows}
replayed = 0
for rec in meta['records'][:3]:
    r = rows[rec['position_index']]; acts, y, ws, tp = L.p.label(r['request'], W, rec['seed'], None); j = look[rec['position_index']]
    assert hashlib.sha256(ws.tobytes()).hexdigest() == rec['worlds_sha256'] and hashlib.sha256(tp.tobytes()).hexdigest() == rec['tapes_sha256']
    q = np.zeros(28, np.float32); q[acts] = y.mean(0); assert np.array_equal(q, ref['Q'][j]); replayed += 1
out['reference_replay'] = dict(roots=replayed, worlds=W, byte_identical=True)
f, _ = L.load_dir(HERE / 'features/test', keys=('F', 'ids')); lf = {int(i): j for j, i in enumerate(f['ids'])}
sample = test_rows[::97]; assert all(np.array_equal(L.featurize(r['request']).astype(np.float16), f['F'][lf[r['index']]]) for r in sample)
out['features'] = dict(recomputed_roots=len(sample), identical=True, inputs='request fields only (decl, bid, bidder, seat, own original hand, public plays)')
Y = np.unpackbits(ref['B'], axis=2)[:, :, :W].astype(np.float32); assert np.allclose(Y.mean(2) * (ref['M'] > 0), ref['Q'], atol=1e-6)
out['packed_outcomes_reproduce_Q'] = True
s = json.load(open(HERE / 'results/test-all/summary.json')); d = json.load(open(HERE / 'results/test-all/details.json'))
reg = np.array([max(x['Q'][a] for a in x['legal']) - x['Q'][x['choices']['feat-s64-d16']] for x in d]); src = np.array([x['source_id'] for x in d])
means = np.array([reg[src == k].mean() for k in np.unique(src)]); assert abs(means.mean() - s['metrics']['feat-s64-d16']['mean']) < 1e-9
out['metric_arithmetic_recomputed'] = dict(model='feat-s64-d16', mean=float(means.mean()))
L.dump(HERE / 'checks/audit-result.json', out); print(json.dumps(out))
