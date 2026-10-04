#!/usr/bin/env python3
"""Final retained-checkpoint identity/caps/trace/cost audit. No timings rerun."""
import argparse,gzip,hashlib,json,math,subprocess,sys
from pathlib import Path
sys.dont_write_bytecode=True
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1]
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def semantic(v):
 if isinstance(v,dict):return {k:semantic(x)for k,x in v.items()if k not in {'elapsed_us','solver_us','over_budget','status_us','outer_sample_us','solve_us'}}
 if isinstance(v,list):return list(map(semantic,v))
 return v
def main():
 p=argparse.ArgumentParser();p.add_argument('--manifest',action='store_true');a=p.parse_args()
 previous='47db3c50ba8af31544e7de05bb2745b24f0abff5'
 changed=subprocess.check_output(['git','diff','--name-only',previous],cwd=ROOT,text=True).splitlines()
 assert all(x.startswith('experiments/native-policy-check-20261004/')for x in changed),changed
 assert not (ROOT/'.git/objects/info/alternates').exists()
 subprocess.run(['git','cat-file','-e',previous+'^{commit}'],cwd=ROOT,check=True,timeout=10)
 identities=json.loads((HERE/'results/phone-conformance/summary.json').read_text())
 assert sha(HERE/'adapter/target/release/native-late-player')==identities['native_sha256']
 assert sha(HERE/'adapter/target/wasm32-unknown-unknown/release/native_late_player.wasm')==identities['rebuilt_wasm_sha256']
 assert sha(ROOT/'experiments/astra-sol-20261004/reference/production-phone/walt-player.wasm')==identities['pinned_wasm_sha256']
 receipts=[]
 for item in sorted(HERE.rglob('run.json')):
  if any(s=='target'or s.endswith('-target')for s in item.parts):continue
  r=json.loads(item.read_text());assert 0<r['allowance_seconds']<=295 and r['elapsed_seconds']<300 and not r['cleanup_errors']
  assert r['status']in ['completed','failed']and r['child_returncode']is not None and r['runner_returncode']is not None
  receipts.append(dict(path=str(item.relative_to(HERE)),status=r['status'],elapsed_seconds=r['elapsed_seconds']))
 gamefiles=sorted((HERE/'results/native-games').glob('game-*.json'));assert len(gamefiles)==144
 matched=late=0
 for item in gamefiles:
  native=json.loads(item.read_text());wasm=json.loads((HERE/'results/wasm-games'/item.name).read_text())
  assert native['points']==wasm['points']and native['made']==wasm['made']and len(native['moves'])==len(wasm['moves'])==28
  for x,y in zip(native['moves'],wasm['moves']):
   assert x['call']==y['call']and x['candidate']==y['candidate']
   assert semantic(x['response'])==semantic(y['response']),(item,x,y)
   matched+=1;late+=x['response']['route']=='late-l3-native-40-4-2-2'
 assert matched==4032 and late==276
 summaries={n:json.loads((HERE/f'results/{n}-games/summary.json').read_text())for n in ['native','wasm']}
 for n,s in summaries.items():
  assert s['complete_source_deals']==18 and s['complete_games']==144 and not s['missing']and s['pair_wins']==0 and s['pair_losses']==2 and s['pair_ties']==70
  assert s['candidate']['late_completed']==276 and s['candidate']['late_refusals']==0 and s['candidate']['interruptions']==0
  assert s['mean_paired_make_advantage']==-1/36
  mean=-1/36;radius=math.sqrt(2*math.log(40)/18);assert s['hoeffding95']==[max(-1,mean-radius),min(1,mean+radius)]
 cold=json.loads((HERE/'results/cold-same-policy/summary.json').read_text());raw=json.loads(gzip.decompress((HERE/'results/cold-same-policy/records.json.gz').read_bytes()))
 assert cold['complete_vector_comparisons']==432 and cold['all_complete']and len(raw)==72
 cpu={n:sum(json.loads((d/'run.json').read_text())['elapsed_seconds']for d in (HERE/'results').glob(f'{n}-*-log')if d.name.split('-')[1].isdigit())for n in ['native','wasm']}
 print(json.dumps(dict(source_checkpoint=previous,independent_git_objects=True,historical_tracked_files_preserved=True,matched_native_wasm_games=144,matched_native_wasm_moves=matched,matched_late_vectors=late,source_deals=18,source_deals_not_doubled_across_backends=True,cold_vectors=432,production_policy_comparisons=84,capped_receipts=len(receipts),failed_developmental_receipts=[r for r in receipts if r['status']=='failed'],maximum_allowance_seconds=295,maximum_elapsed_seconds=max(r['elapsed_seconds']for r in receipts),complete_batch_walls_s=cpu,identities={k:identities[k]for k in ['native_sha256','rebuilt_wasm_sha256','pinned_wasm_sha256']}),indent=2))
 if a.manifest:
  paths=subprocess.check_output(['git','ls-files','--cached','--others','--exclude-standard','--','experiments/native-policy-check-20261004'],cwd=ROOT,text=True).splitlines()
  paths=sorted(set(paths)-{'experiments/native-policy-check-20261004/SHA256SUMS'})
  # Active receipt files are unfinished until the watchdog closes them; final
  # SHA256SUMS is written after this receipt closes, using the same path rules.
  print(json.dumps(dict(manifest_paths=len(paths))))
if __name__=='__main__':main()
