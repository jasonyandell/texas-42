"""Recalculate decision deltas from packed independent-reference outcomes."""
import gzip,hashlib,json,sys
from pathlib import Path
import numpy as np
sys.dont_write_bytecode=True
HERE=Path(__file__).resolve().parent.parent
with gzip.open(HERE/'positions.json.gz','rt') as f:rows=json.load(f)
results={}
for name in ('validation0','validation1','test0','test1'):
 folder=HERE/'results'/name
 if not (folder/'summary.json').exists():continue
 summary=json.loads((folder/'summary.json').read_text());refkind='reference1' if name.endswith('1') else 'reference0';saved={}
 for f in (HERE/'data'/refkind).glob('*.npz'):
  d=dict(np.load(f));W=int(d['W']);Y=np.unpackbits(d['B'],axis=2)[:,:,:W]
  for j,idx in enumerate(d['ids']):saved[int(idx)]=(Y[j].mean(1),d['M'][j])
 values={};details=json.loads((folder/'details.json').read_text())
 for row in details:
  idx=row['position_index'];q,m=saved[idx];legal=np.flatnonzero(m);v=max(q[legal]);assert np.allclose(row['Qref'],q,atol=0,rtol=0)
  for policy,regret in row['regrets'].items():
   actual=v-q[legal].mean() if policy=='uniform_random' else v-q[row['choices'][policy]]
   assert abs(actual-regret)<1e-7
   values.setdefault(policy,{}).setdefault(row['source_id'],[]).append(actual)
 for policy,deals in values.items():
  estimate=float(np.mean([np.mean(v) for v in deals.values()]));assert abs(estimate-summary['metrics'][policy]['mean'])<1e-7
 result={'verified_roots':len(details),'source_deals':len(values['uniform_random']),'packed_Q_regret_arithmetic':'pass'}
 if 'n1-32' in values and 'n0-32' in values:
  diff=np.array([np.mean(values['n1-32'][sid])-np.mean(values['n0-32'][sid]) for sid in sorted(values['n1-32'])]);rng=np.random.default_rng(619901)
  boot=diff[rng.integers(len(diff),size=(10000,len(diff)))].mean(1)
  result['n1_minus_n0_same_reference_regret']={'mean':float(diff.mean()),'independent_bootstrap10000_ci95':np.quantile(boot,[.025,.975]).tolist(),'lower_is_better':True}
 results[name]=result
print(json.dumps(results,indent=2))
