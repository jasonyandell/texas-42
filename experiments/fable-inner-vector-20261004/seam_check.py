#!/usr/bin/env python3
"""Differential fidelity of the inner-vector seam. Run under run_capped.py.

For every request: (1) speedups build, check=true: seam choice == production
modeled_choice(0); (2) reference-paths build: values/choice byte-equal to the
speedups build; (3) legal set equals the independent referee's legal set and
the chosen tile is legal; (4) timing per call.
"""
import gzip, importlib.util, json, subprocess, sys, time
from pathlib import Path
sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent; EXP = HERE.parent
spec = importlib.util.spec_from_file_location('rules', EXP / 'partnership/rules.py'); rules = importlib.util.module_from_spec(spec); spec.loader.exec_module(rules)
SPEED = HERE / 'adapter/target-speedups/release/inner-vector-seam'; REF = HERE / 'adapter/target-reference/release/inner-vector-seam'
src = sys.argv[1] if len(sys.argv) > 1 else str(EXP / 'tiny-net-coverage-20261004/positions.json.gz')
limit = int(sys.argv[2]) if len(sys.argv) > 2 else 1024
out = HERE / 'results' / (sys.argv[3] if len(sys.argv) > 3 else 'seam-check.json')
rows = [r for r in json.load(gzip.open(src, 'rt')) if r['split'] == 'validation'][:limit]

def run(binary, requests, check):
    p = subprocess.Popen([str(binary)], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, bufsize=1)
    outs = []; t0 = time.perf_counter()
    for req in requests:
        p.stdin.write(json.dumps({'request': req, 'check': check, 'budget_ms': 2000}) + '\n'); p.stdin.flush()
        outs.append(json.loads(p.stdout.readline()))
    p.stdin.close(); p.wait(timeout=10)
    return outs, time.perf_counter() - t0

reqs = [r['request'] for r in rows]
a, ta = run(SPEED, reqs, True); b, tb = run(REF, reqs, False)
stats = dict(positions=len(reqs), errors=0, ineligible=0, production_agree=0, production_disagree=[], cross_build_identical=0, cross_build_differ=[], legal_mismatch=0, illegal_choice=0, seats={}, legal_sizes={}, ties_at_top=0, speedups_seconds=ta, reference_seconds=tb, elapsed_us_speedups=[], elapsed_us_reference=[])
for r, x, y in zip(rows, a, b):
    if 'error' in x or 'error' in y: stats['errors'] += 1; continue
    if 'ineligible' in x: stats['ineligible'] += 1; continue
    s = rules.information_state(r['request'])
    if x['legal'] != s['legal']: stats['legal_mismatch'] += 1
    if x['choice'] not in s['legal']: stats['illegal_choice'] += 1
    if x['agree']: stats['production_agree'] += 1
    else: stats['production_disagree'].append(dict(index=r['index'], seam=x['choice'], production=x['production_choice'], values=x['values']))
    if x['values'] == y['values'] and x['choice'] == y['choice']: stats['cross_build_identical'] += 1
    else: stats['cross_build_differ'].append(dict(index=r['index'], speedups=x['values'], reference=y['values']))
    best = max(x['counts']) if x['maximize'] else min(x['counts'])
    if x['counts'].count(best) > 1: stats['ties_at_top'] += 1
    stats['seats'][str(r['request']['seat'])] = stats['seats'].get(str(r['request']['seat']), 0) + 1
    stats['legal_sizes'][str(len(x['legal']))] = stats['legal_sizes'].get(str(len(x['legal'])), 0) + 1
    stats['elapsed_us_speedups'].append(x['elapsed_us']); stats['elapsed_us_reference'].append(y['elapsed_us'])
for k in ['elapsed_us_speedups', 'elapsed_us_reference']:
    v = sorted(stats[k]); stats[k] = dict(median=v[len(v) // 2], p95=v[int(.95 * len(v)) - 1], max=v[-1], mean=sum(v) / len(v)) if v else None
stats['sample'] = [dict(index=r['index'], seat=r['request']['seat'], legal=x['legal'], counts=x['counts'], choice=x['choice'], declaring=x['declaring']) for r, x in list(zip(rows, a))[:5]]
out.write_text(json.dumps(stats, indent=1) + '\n'); print(json.dumps({k: v for k, v in stats.items() if k not in ('production_disagree', 'cross_build_differ', 'sample')}))
print('disagreements', len(stats['production_disagree']), 'cross-build differences', len(stats['cross_build_differ']))
