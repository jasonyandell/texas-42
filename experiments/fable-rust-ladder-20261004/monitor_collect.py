#!/usr/bin/env python3
"""Collect ladder metrics into one snapshot JSON for the live monitor artifact.
Sources: models/*.json (training histories), results/h2h-*/summary.json (paired h2h), results/log2-*.out
(labeling rounds), running processes. Output: results/monitor/snap-<epoch>.json (one doc per push)."""
import glob, json, os, re, subprocess, time
from pathlib import Path
HERE = Path(__file__).resolve().parent
now = int(time.time())
def rows(path): return [json.loads(l) for l in open(path) if l.strip().startswith('{')]
# label record counts per data file (states), for nets' training-set sizes
nets = []
for mj in sorted(glob.glob(str(HERE / 'models/*.json')), key=os.path.getmtime):
    m = json.load(open(mj)); name = m['name']
    hist = m.get('history', []); step = max(1, len(hist) // 60)
    nets.append(dict(name=name, rows=m.get('train_rows'), hidden=m.get('hidden'), hidden2=m.get('hidden2'), layers=m.get('layers'), enc=m.get('encoding'), worlds=m.get('worlds'), selected=m.get('selected_update'), sel=m.get('selected'), train_s=round(m.get('train_seconds', 0)), mtime=int(os.path.getmtime(mj)), history=[dict(u=h['update'], agree=round(h.get('val_agree_hires', 0), 4), rmse=round(h.get('val_rmse', 0), 4), bce=round(h.get('val_bce', 0), 4)) for h in hist[::step]]))
h2h = []
for sj in sorted(glob.glob(str(HERE / 'results/h2h-*/summary.json')), key=os.path.getmtime):
    s = json.load(open(sj))
    h2h.append(dict(tag=s['tag'], net=s['net'], deals=s['deals_paired'], adv=s['mean_paired_advantage'], ci=s['ci95'], win=s.get('hybrid_overall_win_rate'), hyb_us=s['hybrid_decision_us']['median'], nat_us=s['native_decision_us']['median'], mtime=int(os.path.getmtime(sj))))
labels = []
for out in sorted(glob.glob(str(HERE / 'results/log2-*.out'))):
    tag = Path(out).stem[5:]; rs = rows(out); labels.append(dict(tag=tag, rounds=[dict(r=r['round'], wall=r['wall'], states=r['states']) for r in rs if 'round' in r], mtime=int(os.path.getmtime(out))))
try: procs = subprocess.run(['ps', '-axo', 'command='], capture_output=True, text=True).stdout.strip().split('\n')
except Exception: procs = []
counts = {}
for p in procs:
    if 'run_capped' in p or p.startswith('/bin/zsh') or p.startswith('zsh') or 'ps -axo' in p: continue
    if '/ladder ' in p:
        for k in ('log2', 'h2h', 'label', 'turbo-eval'):
            if f'ladder {k}' in p: counts[k] = counts.get(k, 0) + 1
    elif 'train.py' in p and 'python' in p: counts['train.py'] = counts.get('train.py', 0) + 1
# continuous pipeline runs: rounds landed, nets trained, h2h results (pipeline/<name>/log.jsonl)
pipelines = []
for lg in sorted(glob.glob(str(HERE / 'pipeline/*/log.jsonl'))):
    name = Path(lg).parent.name; ev = rows(lg)
    landed = [e for e in ev if e.get('event') == 'round_landed']
    labels.append(dict(tag=f'pipeline {name}', rounds=[dict(r=e['round'], wall=e.get('wall'), states=e.get('train_rows', 0) + e.get('val_rows', 0)) for e in landed], mtime=int(os.path.getmtime(lg))))
    pipelines.append(dict(name=name, events=[{k: v for k, v in e.items() if k != 'args'} for e in ev[-25:]], rounds=len(landed), nets=sum(1 for e in ev if e.get('event') == 'net')))
# artifact db documents are capped at 256 KB: keep training histories only for the newest 24 nets
for n in sorted(nets, key=lambda n: n.get('mtime') or 0)[:-24]: n.pop('history', None)
snap = dict(t=now, iso=time.strftime('%Y-%m-%d %H:%M:%S'), nets=nets, h2h=h2h, labels=labels, running=counts, pipelines=pipelines,
            refs=dict(teacher=0.077, teacher_ci=[0.048, 0.105], floor=-0.541, void_native=0.018))
out = HERE / 'results/monitor'; out.mkdir(exist_ok=True); p = out / f'snap-{now}.json'; p.write_text(json.dumps(snap)); print(p, 'nets', len(nets), 'h2h', len(h2h), 'labels', len(labels), 'running', counts, 'bytes', p.stat().st_size)
