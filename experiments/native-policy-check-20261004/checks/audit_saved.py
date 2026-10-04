#!/usr/bin/env python3
"""Audit retained production conformance or reviewer capped-run/hash receipts."""
import argparse,gzip,hashlib,json,sys
from pathlib import Path
sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parents[3];HERE=ROOT/'experiments/native-policy-check-20261004';CHECK=HERE/'checks'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def clean(v):
 if isinstance(v,dict):return {k:clean(x)for k,x in v.items()if k not in {'elapsed_us','solver_us','over_budget'}}
 if isinstance(v,list):return [clean(x)for x in v]
 return v
def phone():
 path=HERE/'results/phone-conformance';summary=json.loads((path/'summary.json').read_text());records=json.loads(gzip.decompress((path/'records.json.gz').read_bytes()));assert len(records)==summary['calls']==42
 native=HERE/'adapter/target/release/native-late-player';wasm=HERE/'adapter/target/wasm32-unknown-unknown/release/native_late_player.wasm';pinned=ROOT/'experiments/astra-sol-20261004/reference/production-phone/walt-player.wasm'
 assert summary['native_sha256']==sha(native)and summary['rebuilt_wasm_sha256']==sha(wasm)and summary['pinned_wasm_sha256']==sha(pinned)
 for i,r in enumerate(records):
  assert r['index']==i and len(r['call']['request'])==7 and set(r['call']['request'])=={'decl','bid','bidder','seat','hand','plays','seed'}
  assert clean(r['native'])==clean(r['pinned'])==clean(r['rebuilt']);assert not r['pinned_timing']['interrupted']and not r['rebuilt_timing']['interrupted'];assert 'error'not in r['native']
 assert summary['complete_semantic_comparisons']==84
 print(json.dumps(dict(retained_calls=42,recursive_non_clock_comparisons=84,all9declarations=sorted({r['call']['request']['decl']for r in records}),public_only=True,phone_policy_distinct_from_late_policy=True,pinned_wasm_sha256=sha(pinned)),indent=2))
def history():
 counts={}
 for name in ['astra-sol-20261004','adversarial-20261004','native-frontier-20261004','prefix-state-20261004','demand-bounds-20261004','choice-arena-20261004','resumable-choice-20261004']:
  folder=ROOT/'experiments'/name;count=0
  for line in (folder/'SHA256SUMS').read_text().splitlines():
   expected,relative=line.split(maxsplit=1);relative=relative.strip().lstrip('*');path=ROOT/relative if relative.startswith('experiments/')else folder/relative
   assert path.is_file()and sha(path)==expected,str(path);count+=1
  counts[name]=count
 print(json.dumps(dict(historical_manifest_entries=counts,total=sum(counts.values()),all_file_bytes_preserved=True),indent=2))
def traces():
 native=HERE/'results/native-games';wasm=HERE/'results/wasm-games';games=moves=0
 for p in sorted(native.glob('game-*.json')):
  a=json.loads(p.read_text());b=json.loads((wasm/p.name).read_text());assert a['points']==b['points']and a['made']==b['made']and a['hands']==b['hands'];assert len(a['moves'])==len(b['moves'])==28
  for x,y in zip(a['moves'],b['moves']):
   assert x['call']==y['call']and x['candidate']==y['candidate']
   for k in ['choice','legal','points','leader','trick','route']:assert x['response'][k]==y['response'][k],k
   if x['response']['route']=='late-l3-native-40-4-2-2':assert x['response']['evaluation']==y['response']['evaluation']
   moves+=1
  games+=1
 assert games==144 and moves==4032
 print(json.dumps(dict(native_wasm_matching_complete_game_trajectories=games,matching_public_actor_requests_and_decisions=moves,independent_source_deals=18,scope='Finite same prespecified game panel, excluding clock/accounting fields; does not prove policies globally equivalent or stronger than pinnedphone.'),indent=2))
def receipts():
 rows=[]
 for p in sorted(CHECK.rglob('run.json')):
  if 'target'in p.parts:continue
  r=json.loads(p.read_text());assert 0<r['allowance_seconds']<=295 and r['elapsed_seconds']<300 and not r['cleanup_errors'];assert r['status']in ['completed','failed'];assert r['child_returncode']is not None
  rows.append(dict(path=str(p.relative_to(CHECK)),status=r['status'],elapsed=r['elapsed_seconds']))
 files=[p for p in CHECK.rglob('*')if p.is_file()and not any(t in p.parts for t in ['target','__pycache__'])and p.suffix in ['.rs','.py','.mjs','.lean','.toml']]
 files += [HERE/'adapter/src/lib.rs',HERE/'adapter/src/main.rs',HERE/'adapter/Cargo.toml',HERE/'play.py',HERE/'wasm_worker.mjs',HERE/'plan.json',ROOT/'experiments/resumable-choice-20261004/integration/src/main.rs',ROOT/'experiments/resumable-choice-20261004/player.py',ROOT/'experiments/resumable-choice-20261004/arena.py']
 hashes={str(p.relative_to(ROOT)):sha(p)for p in sorted(files)};hashes['experiments/native-policy-check-20261004/adapter/target/release/native-late-player']=sha(HERE/'adapter/target/release/native-late-player');hashes['experiments/native-policy-check-20261004/adapter/target/wasm32-unknown-unknown/release/native_late_player.wasm']=sha(HERE/'adapter/target/wasm32-unknown-unknown/release/native_late_player.wasm')
 (CHECK/'SOURCE_HASHES.json').write_text(json.dumps(hashes,indent=2)+'\n');print(json.dumps(dict(capped_receipts=len(rows),failed_receipts=[r for r in rows if r['status']=='failed'],all_allowances_at_most295=True,all_elapsed_below300=True,all_children_reaped=True,source_hashes=len(hashes),receipts=rows),indent=2))
if __name__=='__main__':
 p=argparse.ArgumentParser();p.add_argument('mode',choices=['phone','history','traces','receipts']);a=p.parse_args();phone()if a.mode=='phone'else history()if a.mode=='history'else traces()if a.mode=='traces'else receipts()
