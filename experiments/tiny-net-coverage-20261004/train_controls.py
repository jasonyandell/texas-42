#!/usr/bin/env python3
"""Sequential GPU controls, checkpointed individually under295second caps."""
import json,subprocess,sys
from pathlib import Path
from coverage import HERE,p
ROOT=HERE.parents[1];CAP=ROOT/'experiments/astra-sol-20261004/tools/run_capped.py';PY='/Users/jason/code/flux/python/.venv/bin/python'
protocol=[('mixed32','mixed128',32),('late32','late128',32),('late64','late128',64),('subset128','subset128',32),('subset512','subset512',32)]
for name,kind,H in protocol:
 model=HERE/'models'/f'{name}.npz';meta=model.with_suffix('.json')
 if model.exists() and meta.exists():continue
 log=HERE/'results'/f'train-{name}-log';assert not log.exists()
 cmd=[sys.executable,str(CAP),'--seconds','295','--output-dir',str(log),'--',PY,str(HERE/'coverage.py'),'train','--kind',kind,'--name',name,'--hidden',str(H),'--epochs','100']
 result=subprocess.run(cmd,timeout=299);assert result.returncode==0,name
freeze={}
for name,kind,H in protocol:
 meta=json.loads((HERE/'models'/f'{name}.json').read_text());freeze[name]=dict(weights_sha256=p.sha(HERE/'models'/f'{name}.npz'),dataset=kind,hidden=H,train_roots=meta['train_roots'],validation_roots=meta['validation_roots'],epochs=100,optimizer_updates=100*((meta['train_roots']+255)//256),selected_epoch=meta['selected_epoch'])
p.dump(HERE/'models/FROZEN.json',dict(tier='exploratory',selection='allcontrols frozen before test reference generation/read; late validationMSE only',models=freeze));print(json.dumps(freeze))
