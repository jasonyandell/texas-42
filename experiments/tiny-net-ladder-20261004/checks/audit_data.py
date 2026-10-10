"""Structural provenance/split/paired-label audit; no policy test selection."""
import gzip,hashlib,importlib.util,json,sys
from pathlib import Path
import numpy as np
sys.dont_write_bytecode=True
HERE=Path(__file__).resolve().parent.parent
sp=importlib.util.spec_from_file_location('pilot',HERE/'pilot.py');p=importlib.util.module_from_spec(sp);sp.loader.exec_module(p)
with gzip.open(HERE/'sources.json.gz','rt') as f:sources=json.load(f)
rows=p.positions();source_split={s['source_id']:s['split'] for s in sources}
assert len(source_split)==len(sources)
assert len({r['information_key'] for r in rows})==len(rows)
for r in rows:
 s=sources[r['source_index']];assert s['source_id']==r['source_id'] and s['split']==r['split']
 req=r['request'];assert req['hand']==s['hands'][req['seat']] and req['decl']==s['decl'] and req['bidder']==s['bidder']
 p.rules.replay_record(s['hands'],req['plays'],req['decl'],req['bidder'])
 assert hashlib.sha256(json.dumps(req,sort_keys=True).encode()).hexdigest()==r['information_key']
assert sum(s['split']=='train' for s in sources)==1536
assert sum(s['split']=='validation' for s in sources)==256
assert sum(s['split']=='test' for s in sources)==256
report={'source_deals':len(sources),'roots':len(rows),'duplicate_information_keys':0,'datasets':{}}
for folder in sorted((HERE/'data').iterdir()):
 if not folder.is_dir():continue
 ids=[];versions=set();seeds=set();files=list(folder.glob('*.npz'))
 for file in sorted(files):
  meta=json.loads(file.with_suffix('.json').read_text());assert p.sha(file)==meta['dataset_sha256'];d=dict(np.load(file));W=int(d['W']);assert W==meta['worlds']
  assert len(d['ids'])==meta['positions']==len(meta['records']);versions.add((meta['source_kernel_sha256'],meta['binary_sha256'],meta['model_sha256']))
  for j,idx in enumerate(d['ids']):
   idx=int(idx);r=rows[idx];req=r['request'];rec=meta['records'][j];assert rec['position_index']==idx and rec['source_id']==r['source_id']
   assert np.array_equal(d['X'][j],p.encode(req));legal=p.public(req)[0]['legal'];assert np.flatnonzero(d['M'][j]).tolist()==legal
   outcomes=np.unpackbits(d['B'][j],axis=1)[:,:W];assert np.array_equal(d['Q'][j,legal],outcomes[legal].mean(1).astype(np.float32))
   assert not np.any(d['Q'][j,d['M'][j]==0]);seeds.add(rec['seed']);ids.append(idx)
 assert len(set(ids))==len(ids);assert len(versions)==1
 version=next(iter(versions))
 if folder.name in ('labels0','reference0'):assert version[2] is None
 if folder.name in ('labels1','reference1'):assert version[2]==p.sha(HERE/'models/n0-32.npz')
 report['datasets'][folder.name]={'batches':len(files),'roots':len(ids),'one_frozen_teacher':True,'unique_record_seeds':len(seeds)}
print(json.dumps(report,indent=2))
