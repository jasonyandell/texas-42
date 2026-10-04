#!/usr/bin/env python3
"""Final local source/receipt/complete-vector audit. Invoke via run_capped."""
import gzip,hashlib,json,subprocess,sys
from pathlib import Path
sys.dont_write_bytecode=True
HERE=Path(__file__).resolve().parent
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
 sums=[]
 for line in (HERE/'SHA256SUMS').read_text().splitlines():
  h,p=line.split('  ',1);assert sha(HERE/p)==h,p;sums.append(p)
 receipts=[]
 for p in sorted(HERE.rglob('run.json')):
  if any(x in p.parts for x in ['target','resumable-target','old-arena-target','old-demand-target','integration-target']):continue
  r=json.loads(p.read_text())
  assert 0<r['allowance_seconds']<=295 and r['elapsed_seconds']<300,p
  assert not r['cleanup_errors'],p
  receipts.append(dict(path=str(p.relative_to(HERE)),status=r['status'],elapsed_s=r['elapsed_seconds']))
 comparisons=0;refusals=0
 for name in ['final-one','final-four','final-holdout']:
  summary=json.loads((HERE/'results'/name/'summary.json').read_text())
  records=json.loads(gzip.decompress((HERE/'results'/name/'records.json.gz').read_bytes()))
  n=0
  for row in records:
   expected=row['runs']['recursive']['result']['answers']
   for variant,run in row['runs'].items():
    if variant=='recursive':continue
    result=run['result']
    if 'answers' in result:assert result['answers']==expected;n+=len(expected)
    else:assert 'error' in result;refusals+=1
  assert n==summary['vector_comparisons'];comparisons+=n
 game_dir=HERE/'results/h2h';games=[json.loads(p.read_text()) for p in sorted(game_dir.glob('game-*.json'))]
 assert len(games)==8 and {(g['rotation'],g['role']) for g in games}=={(r,s) for r in range(4) for s in ['declaring','defending']}
 assert len({g['binary_sha256'] for g in games})==1
 assert all(g['complete'] and len(g['moves'])==28 for g in games)
 for g in games:assert g['binary_sha256']==sha(HERE/'integration/target/release/late-choice-player')
 historical=subprocess.check_output(['git','diff','97c1c175','--','experiments/choice-arena-20261004','experiments/demand-bounds-20261004','experiments/prefix-state-20261004','experiments/native-frontier-20261004','experiments/adversarial-20261004','walt','ingest'],cwd=HERE.parents[1],text=True)
 assert not historical,'historical/production source changed'
 out=dict(manifest_entries=len(sums),receipts=receipts,completed_service_vector_comparisons=comparisons,refused_service_observations=refusals,complete_h2h_games=len(games),complete_h2h_moves=224,historical_and_production_diff_empty=True,current_binaries={str(p.relative_to(HERE)):sha(p) for p in [HERE/'native/target/release/native-frontier',HERE/'integration/target/release/late-choice-player',HERE/'checks/old-arena-target/release/native-frontier',HERE/'checks/old-demand-target/release/native-frontier']})
 print(json.dumps(out,indent=2))
if __name__=='__main__':main()
