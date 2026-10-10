#!/usr/bin/env python3
"""Recompute local source provenance and bounded resource accounting."""
import hashlib,json
from pathlib import Path
from pilot import HERE,dump,sha
ROOT=HERE.parents[1];canonical=Path('/Users/jason/Documents/Codex/2026-10-04/task-2/research-worktree')
sources=[
 'experiments/astra-sol-20261004/phase2/incoming/rollout_net.py',
 'experiments/astra-sol-20261004/phase2/incoming/walt-sense.zip',
 'experiments/astra-sol-20261004/phase2/incoming/PROVENANCE.json',
 'experiments/astra-sol-20261004/phase2/incoming/drive/TINY-MODEL.md',
 'experiments/astra-sol-20261004/phase2/incoming/drive/HANDOFF.md',
 'experiments/adversarial-20261004/results/drive-refetch.json',
 'experiments/adversarial-20261004/incoming/drive-final/engine42.v2.py',
 'experiments/adversarial-20261004/sol/REPORT.md',
 'experiments/higher-k-budget-20261004/adapter/src/lib.rs',
 'experiments/native-policy-check-20261004/REPORT.md',
 'experiments/astra-sol-20261004/reference/production-phone/walt-player.wasm',
 'experiments/partnership/rules.py']
records=[]
for rel in sources:
 p=ROOT/rel;assert p.exists(),rel;old=canonical/rel
 rec=dict(relative_path=rel,bytes=p.stat().st_size,sha256=sha(p),canonical_task2_exists=old.exists())
 if old.exists():rec['canonical_task2_sha256']=sha(old);assert rec['canonical_task2_sha256']==rec['sha256'],rel
 records.append(rec)
resource={}
for kind in ['labels0','labels1','reference0','reference1','control']:
 ms=[json.loads(p.read_text()) for p in (HERE/'data'/kind).glob('*.json')];seconds=sum(m['seconds'] for m in ms);roots=sum(m['positions'] for m in ms)
 resource[kind]=dict(batches=len(ms),roots=roots,legal_action_vectors=sum(m['actions'] for m in ms),root_world_samples=sum(m['positions']*m['worlds'] for m in ms),candidate_continuations=sum(m['actions']*m['worlds'] for m in ms),summed_worker_generation_seconds=seconds,roots_per_summed_worker_second=roots/seconds,note='generation includes exact sampler, native rollouts, hashes, compression. Sum is worker wall, not CPU time or parallel elapsed. At most two workers.')
receipts=[json.loads(p.read_text()) for p in (HERE/'results').rglob('run.json')]+[json.loads(p.read_text()) for p in (HERE/'checks/results').rglob('run.json')]
assert all(r['allowance_seconds']<=295 and r['elapsed_seconds']<300 for r in receipts)
dump(HERE/'PROVENANCE.json',dict(tier='exploratory',base_commit='92cc1ac3ae2c5bd4b0f5f9d0aadc56eb9583a299',canonical_recovery_root=str(canonical),sources=records,production=dict(plunge_commit='a0d9fa806166b0e63fe016bb49d93f91f47b1af8',rust_commit='cb1ef3b23072e4c268f31f625f2b61d5facc1929',wasm_sha256=sha(ROOT/sources[-2])),library_cloud_export=False,task10_touched=False,resource=resource,watchdog=dict(receipts=len(receipts),maximum_allowance=max(r['allowance_seconds'] for r in receipts),maximum_observed_seconds=max(r['elapsed_seconds'] for r in receipts),statuses={status:sum(r['status']==status for r in receipts) for status in sorted({r['status'] for r in receipts})}),recovery='original tiny source, archive, saved conversation provenance, current Drive packet and original accepted/contradicted files preserved; no external refetch/transmission. Issue100 remains separate.'))
print(json.dumps(resource,indent=2))
