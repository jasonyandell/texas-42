"""Independent packed-label decision arithmetic and paired deal bootstrap."""
import datetime,gzip,hashlib,json,sys
from pathlib import Path
import numpy as np
sys.dont_write_bytecode=True
HERE=Path(__file__).resolve().parent.parent
rows=json.loads(gzip.open(HERE/'positions.json.gz','rt').read());saved={}
for f in (HERE/'data/reference0').glob('*.npz'):
 d=dict(np.load(f));W=int(d['W']);Y=np.unpackbits(d['B'],axis=2)[:,:,:W]
 for j,i in enumerate(d['ids']):saved[int(i)]=(Y[j].mean(1),d['M'][j])
frozen=json.loads((HERE/'models/FROZEN.json').read_text());adaptive=json.loads((HERE/'models/ADAPTIVE-FROZEN.json').read_text())
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
for name,meta in frozen['models'].items():assert sha(HERE/'models'/f'{name}.npz')==meta['weights_sha256']
for name,meta in adaptive['models'].items():
 path=HERE/'models'/f'{name}.npz';assert sha(path)==meta['sha256'];original=name.replace('updates1500','');assert path.read_bytes()==(HERE/'models'/f'{original}.npz').read_bytes()
 training=json.loads(path.with_suffix('.json').read_text());assert training['epochs']==375 and training['selected_epoch']==2 and meta['selected_updates']==8
freeze_time=max((HERE/'models/FROZEN.json').stat().st_mtime,(HERE/'models/ADAPTIVE-FROZEN.json').stat().st_mtime)
chronology=[]
for batch in (14,15):
 receipt=HERE/'results'/f'reference0-{batch:03d}-log/run.json'
 if receipt.exists():chronology.append(datetime.datetime.fromisoformat(json.loads(receipt.read_text())['started_utc']).timestamp()>freeze_time)
report={'frozen_models':{'original_controls':len(frozen['models']),'adaptive_controls':len(adaptive['models']),'adaptive_byte_identical':True,'test_reference_started_after_freeze_local_mtimes':bool(chronology) and all(chronology),'chronology_qualification':'local filesystem mtimes checked only when preserved; checkout/relocation can reset them; original passing final-test-metrics-audit receipt retains local chronological evidence'}}
snapshot_path=HERE/'results/chronology.json'
if snapshot_path.exists():
 snapshot=json.loads(snapshot_path.read_text());freeze_times=[]
 for name,entry in snapshot['freeze_snapshot'].items():
  assert sha(HERE/'models'/name)==entry['sha256'];freeze_times.append(datetime.datetime.fromisoformat(entry['original_local_mtime_utc']).timestamp())
 for value in snapshot['test_receipt_utc'].values():assert datetime.datetime.fromisoformat(value).timestamp()>max(freeze_times)
 report['frozen_models']['retained_chronology_snapshot_verified']=True
for folder in sorted((HERE/'results').iterdir()):
 if not (folder/'summary.json').exists() or not (folder/'details.json').exists():continue
 summary=json.loads((folder/'summary.json').read_text());details=json.loads((folder/'details.json').read_text());values={}
 for entry in details:
  idx=entry['position_index'];q,m=saved[idx];legal=np.flatnonzero(m);v=max(q[legal]);assert np.array_equal(q,np.array(entry['Q']))
  assert rows[idx]['split']==summary['split']
  for name,regret in entry['regrets'].items():
   val=v-q[legal].mean() if name=='uniform_random' else v-q[entry['choices'][name]]
   assert abs(val-regret)<1e-7;values.setdefault(name,{}).setdefault(entry['source_id'],[]).append(val)
 for name,deals in values.items():assert abs(np.mean([np.mean(v) for v in deals.values()])-summary['metrics'][name]['mean'])<1e-7
 result={'roots':len(details),'source_deals':len(values['uniform_random']),'packed_Q_regret_arithmetic':'pass','independent_paired_bootstrap10000':{}}
 for a,b in [('late32','mixed32'),('late64','late32'),('subset512','subset128'),('late32','subset512')]:
  if a not in values or b not in values:continue
  diff=np.array([np.mean(values[a][sid])-np.mean(values[b][sid]) for sid in sorted(values[a])]);rng=np.random.default_rng(9910677);boot=diff[rng.integers(len(diff),size=(10000,len(diff)))].mean(1)
  result['independent_paired_bootstrap10000'][a+'-minus-'+b]={'mean':float(diff.mean()),'ci95':np.quantile(boot,[.025,.975]).tolist()}
 if summary['gate'] is not None:
  ml=summary['metrics']['late32'];mm=summary['metrics']['mixed32'];delta=summary['gate']['paired_vs_mixed'];actual=ml['mean']<=.9*mm['mean'] and delta['ci95'][1]<0 and ml['vs_random']['ci95'][1]<0 and ml['vs_ascending']['ci95'][1]<0
  assert actual==summary['gate']['pass_gate'];result['primary_gate']=actual
 report[folder.name]=result
print(json.dumps(report,indent=2))
