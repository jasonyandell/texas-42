"""Independent structural and finite mining/paired-data coverage checks."""
import gzip,hashlib,importlib.util,json,random,sys
from collections import Counter
from pathlib import Path
import numpy as np
sys.dont_write_bytecode=True
HERE=Path(__file__).resolve().parent.parent
spec=importlib.util.spec_from_file_location('cov',HERE/'coverage.py');c=importlib.util.module_from_spec(spec);spec.loader.exec_module(c)
p=c.p;rows=c.positions();sources=c.read('sources.json.gz');membership=json.loads((HERE/'membership.json').read_text());mining=json.loads((HERE/'mining.json').read_text());quotas={int(k):v for k,v in json.loads((HERE/'training-quotas.json').read_text()).items()};plan=json.loads((HERE/'plan.json').read_text())
assert p.sha(HERE/'kernel.c')==p.sha(c.BASE/'kernel.c')
old_sources=c.read('../tiny-net-ladder-20261004/sources.json.gz');old_parts={c.canonical(s['hands']) for s in old_sources}
for f in (c.BASE/'results/h2h').rglob('game-*.json'):old_parts.add(c.canonical(json.loads(f.read_text())['hands']))
oldX={p.encode(r['request']).tobytes() for r in c.read('../tiny-net-ladder-20261004/positions.json.gz')}
source_ids=set();input_keys=set();rng=random.Random(plan['seed'])
for i,s in enumerate(sources):
 tiles=list(range(28));rng.shuffle(tiles);hands=[sorted(tiles[7*q:7*q+7]) for q in range(4)];decl=rng.randrange(7);bidder=rng.randrange(4)
 assert hands==s['hands'] and decl==s['decl'] and bidder==s['bidder']
 key=c.canonical(hands);assert key==s['source_id'] and key not in old_parts and key not in source_ids;source_ids.add(key)
 assert s['split']==('train' if i<1536 else 'validation' if i<1792 else 'test')
for row in rows:
 req=row['request'];s=sources[row['source_index']];assert s['source_id']==row['source_id'] and s['split']==row['split']
 assert req['hand']==s['hands'][req['seat']] and req['decl']==s['decl'] and req['bidder']==s['bidder']
 pts,lead,rem,table=p.rules.replay_record(s['hands'],req['plays'],req['decl'],req['bidder']);assert req['seat']==(lead+len(table))%4
 assert len(p.rules.legal_tiles(rem[req['seat']],table,req['decl']))>1 and pts[req['bidder']%2]<30 and pts[1-req['bidder']%2]<13
 assert row['band']==len(req['plays'])//16 and len(req['plays'])//2<24
 x=p.encode(req).tobytes();assert x not in oldX and x not in input_keys;input_keys.add(x);assert hashlib.sha256(x).hexdigest()==row['information_key']
for key,ids in membership.items():assert len(ids)==len(set(ids))
mc=Counter(rows[i]['source_index'] for i in membership['mixed']);lc=Counter(rows[i]['source_index'] for i in membership['late'])
assert mc==lc==Counter({i:q for i,q in quotas.items() if q})
assert len(membership['mixed'])==len(membership['late'])==3648
assert all(rows[i]['band']==2 and rows[i]['split']=='train' for i in membership['late'])
assert set(membership['subset'])<=set(membership['late']) and len(membership['subset'])==912
assert [rows[i]['source_index'] for i in membership['subset']]==sorted(mc)[:912]
for split in ('validation','test'):
 assert all(rows[i]['split']==split and rows[i]['band']==2 for i in membership[split])
 assert max(Counter(rows[i]['source_index'] for i in membership[split]).values())<=2
eligible=[i for i in range(1536) if mining[i]['unique_candidates_by_band'][2]];expected={i:0 for i in range(1536)};left=3648
while left:
 for i in eligible:
  if expected[i]<mining[i]['unique_candidates_by_band'][2]:expected[i]+=1;left-=1
  if not left:break
assert expected==quotas
# Independently replay all 16 trajectories on 24 predetermined sources,
# including unavailable sources, to check candidate-count commitments.
sample=sorted(set([0,80,575,576,911,912,1200,1535,1536,1791,1792,2047]+[i for i in range(1536) if not mining[i]['unique_candidates_by_band'][2]][:12]))
for si in sample:
 s=sources[si];trng=random.Random(plan['seed']^((si+1)*104729));seen=[set(),set(),set()]
 for traj in range(16):
  rem=list(map(set,s['hands']));lead=s['bidder'];pts=[0,0];table=[];history=[]
  for ply in range(28):
   if pts[s['bidder']%2]>=30 or pts[1-s['bidder']%2]>=13:break
   actor=(lead+len(table))%4;legal=p.rules.legal_tiles(rem[actor],table,s['decl'])
   if len(legal)>1 and ply<24:
    req=dict(decl=s['decl'],bid=30,bidder=s['bidder'],seat=actor,hand=s['hands'][actor],plays=history.copy(),seed=7042104);seen[ply//8].add(c.info(req))
   tile=trng.choice(legal);rem[actor].remove(tile);history.extend([actor,tile]);table.append((actor,tile))
   if len(table)==4:lead=p.rules.winner(table,s['decl']);pts[lead%2]+=p.rules.trick_points(table);table=[]
 assert list(map(len,seen))==mining[si]['unique_candidates_by_band']
 assert all(row['information_key'] in seen[row['band']] for row in rows if row['source_index']==si)
report={'source_deals':len(sources),'unique_normalized_inputs':len(input_keys),'old_source_or_input_overlap':0,'matched_train_source_deals':len(mc),'unavailable_train_sources':1536-len(mc),'main_training_roots':3648,'mixed_bands':dict(Counter(rows[i]['band'] for i in membership['mixed'])),'mining_16trajectory_replayed_sources':len(sample),'late_panels':{split:{'roots':len(membership[split]),'source_deals':len({rows[i]['source_id'] for i in membership[split]}),'unconditional_denominator':256} for split in ('validation','test')},'datasets':{}}
cache={}
if (HERE/'data').exists():
 for folder in sorted((HERE/'data').iterdir()):
  if not folder.is_dir():continue
  ids=[];versions=set();records={}
  for file in sorted(folder.glob('*.npz')):
   meta=json.loads(file.with_suffix('.json').read_text());assert p.sha(file)==meta['dataset_sha256'];data=dict(np.load(file));versions.add((meta['kernel_sha256'],meta['binary_sha256'],meta['model_sha256']))
   for j,idx in enumerate(data['ids']):
    idx=int(idx);row=rows[idx];rec=meta['records'][j];W=rec['worlds'];assert rec['position_index']==idx and rec['source_id']==row['source_id']
    legal=p.public(row['request'])[0]['legal'];assert np.flatnonzero(data['M'][j]).tolist()==legal and np.array_equal(data['X'][j],p.encode(row['request']))
    y=np.unpackbits(data['B'][j],axis=1)[:,:W];assert np.array_equal(data['Q'][j,legal],y[legal].mean(1).astype(np.float32))
    ids.append(idx);records[idx]=(data['B'][j],data['Q'][j],rec)
  if ids:
   assert len(set(ids))==len(ids) and len(versions)==1;cache[folder.name]=records
   if folder.name in ('mixed128','late128','subset128','subset512'):
    key={'mixed128':'mixed','late128':'late','subset128':'subset','subset512':'subset'}[folder.name]
    assert set(ids)==set(membership[key]+membership['validation'])
    assert next(iter(versions))[2] is None
    for idx,(b,q,rec) in records.items():
     expectedW=512 if folder.name=='subset512' and rows[idx]['split']=='train' else 128
     assert rec['worlds']==expectedW
   report['datasets'][folder.name]={'roots':len(ids),'frozen_identity':True}
 for kind in ('mixed128','subset128','subset512'):
  if 'late128' in cache and kind in cache:
   for idx in set(cache['late128'])&set(cache[kind]):
    b0,q0,r0=cache['late128'][idx];b,q,rec=cache[kind][idx];assert np.array_equal(b0,b[:,:16]) and r0['seed']==rec['seed']
    if rec['worlds']==128:assert np.array_equal(q0,q) and r0['worlds_sha256']==rec['worlds_sha256'] and r0['tapes_sha256']==rec['tapes_sha256']
 if 'subset512' in cache and 'late128' in cache:
  selected=sorted(i for i in cache['subset512'] if rows[i]['split']=='train')[:12]
  for idx in selected:
   rec=cache['subset512'][idx][2];seed=rec['seed'];ws=p.worlds(rows[idx]['request'],512,seed);tp=np.random.default_rng(seed^0xDEADBEEF).random((512,28))
   lowrec=cache['late128'][idx][2]
   assert hashlib.sha256(ws[:128].tobytes()).hexdigest()==lowrec['worlds_sha256']
   assert hashlib.sha256(tp[:128].tobytes()).hexdigest()==lowrec['tapes_sha256']
  report['precision_prefix_sample_hash_replays']=len(selected)
print(json.dumps(report,indent=2))
