#!/usr/bin/env python3
"""Reproduce the declared optimizer-budget control, never replace primary gate."""
import json,math,subprocess,sys
from coverage import HERE,p
CAP=HERE.parents[1]/'experiments/astra-sol-20261004/tools/run_capped.py';PY='/Users/jason/code/flux/python/.venv/bin/python'
assert not (HERE/'models/ADAPTIVE-FROZEN.json').exists(), 'preserve existing freeze'
plan=json.loads((HERE/'adaptive-control-plan.json').read_text());assert plan['epochs']==375
gate=json.loads((HERE/'results/validation-controls/summary.json').read_text())['gate'];assert gate['pass_gate'] is False
assert not list((HERE/'data/reference0').glob('batch-01[45].npz')), 'test must remain unopened'
freeze={}
for kind in ['subset128','subset512']:
 name=kind+'updates1500';model=HERE/'models'/f'{name}.npz'
 if not model.exists():
  cmd=[sys.executable,str(CAP),'--seconds','295','--output-dir',str(HERE/'results'/f'train-{name}-log'),'--',PY,str(HERE/'coverage.py'),'train','--kind',kind,'--name',name,'--hidden','32','--epochs','375']
  result=subprocess.run(cmd,timeout=299);assert result.returncode==0,name
 meta=json.loads(model.with_suffix('.json').read_text());updates=math.ceil(meta['train_roots']/256)
 freeze[name]=dict(sha256=p.sha(model),max_updates=meta['epochs']*updates,selected_epoch=meta['selected_epoch'],selected_updates=meta['selected_epoch']*updates,same_as_100epoch_checkpoint=model.read_bytes()==(HERE/'models'/f'{kind}.npz').read_bytes())
p.dump(HERE/'models/ADAPTIVE-FROZEN.json',dict(before_test_reference=True,models=freeze));print(json.dumps(freeze))
