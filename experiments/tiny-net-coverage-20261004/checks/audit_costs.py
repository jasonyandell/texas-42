"""Independent nominal/actual sample budget and label-noise arithmetic."""
import json,sys
from pathlib import Path
import numpy as np
sys.dont_write_bytecode=True
HERE=Path(__file__).resolve().parent.parent
cost=json.loads((HERE/'results/costs-and-diagnostics.json').read_text());budget_report={}
for kind,entry in cost['dataset_costs'].items():
 count=worlds=continuations=0;seconds=0
 for file in (HERE/'data'/kind).glob('*.npz'):
  data=dict(np.load(file));meta=json.loads(file.with_suffix('.json').read_text());seconds+=meta['seconds']
  for j,rec in enumerate(meta['records']):count+=1;worlds+=rec['worlds'];continuations+=int(data['M'][j].sum())*rec['worlds']
 assert entry['total']=={'roots':count,'root_worlds':worlds,'candidate_continuations':continuations}
 assert abs(seconds-entry['kernel_sampling_encoding_and_write_seconds_sum'])<1e-10
 budget_report[kind]={'roots':count,'root_worlds':worlds,'candidate_continuations':continuations,'summed_worker_wall_seconds':seconds}
assert cost['dataset_costs']['mixed128']['by_split']['train']['root_worlds']==cost['dataset_costs']['late128']['by_split']['train']['root_worlds']==cost['dataset_costs']['subset512']['by_split']['train']['root_worlds']==466944
for name,entry in cost['fit'].items():
 meta=json.loads((HERE/'models'/f'{name}.json').read_text());weights=dict(np.load(HERE/'models'/f'{name}.npz'))
 assert sum(a.size for a in weights.values())==entry['parameters'];assert sum(a.nbytes for a in weights.values())==entry['raw_float32_bytes']
 assert entry['max_updates']==meta['epochs']*((meta['train_roots']+255)//256)
 assert entry['selected_updates']==meta['selected_epoch']*((meta['train_roots']+255)//256)
noise=[];counts=[];lowq=[];highq=[]
for kind,W in [('subset128',128),('subset512',512)]:
 vals=[]
 for file in sorted((HERE/'data'/kind).glob('*.npz')):
  data=dict(np.load(file));meta=json.loads(file.with_suffix('.json').read_text())
  for j,rec in enumerate(meta['records']):
   if rec['worlds']!=W or (kind=='subset128' and int(data['ids'][j]) not in set(json.loads((HERE/'membership.json').read_text())['subset'])):continue
   legal=np.flatnonzero(data['M'][j]);Y=np.unpackbits(data['B'][j,legal],axis=1)[:,:W].astype(float);adv=Y-Y.mean(0,keepdims=True);vals.append(float(adv.var(1,ddof=1).mean()/W))
 assert len(vals)==912;noise.append(float(np.mean(vals)))
 assert abs(noise[-1]-cost['label_precision_diagnostic'][str(W)])<1e-8
print(json.dumps({'budgets':budget_report,'label_noise_unscaled_legal_centered_variances':noise,'maximum_vs_selected_updates_checked':True},indent=2))
