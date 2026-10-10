#!/usr/bin/env python3
"""Follow-through controls. Every command must run under run_capped.py."""
import argparse,gzip,hashlib,importlib.util,json,random,shutil,sys,time
from pathlib import Path
sys.dont_write_bytecode=True
import numpy as np
HERE=Path(__file__).resolve().parent;BASE=HERE.parent/'tiny-net-ladder-20261004'
sp=importlib.util.spec_from_file_location('base_pilot',BASE/'pilot.py');p=importlib.util.module_from_spec(sp);sp.loader.exec_module(p)
p.HERE=HERE
def read(name):
 with gzip.open(HERE/name,'rt') as f:return json.load(f)
def write(name,data):
 with gzip.open(HERE/name,'wt') as f:json.dump(data,f)
def positions():return read('positions.json.gz')
p.positions=positions
def canonical(hands):return hashlib.sha256(json.dumps(sorted(p.mask(h) for h in hands)).encode()).hexdigest()
def info(req):return hashlib.sha256(p.encode(req).tobytes()).hexdigest()
def initialize(a):
 assert not (HERE/'plan.json').exists()
 plan=dict(tier='exploratory',schema='tiny-coverage-followthrough-v1',base_commit='04cf0de2b2b56446105cce7db5659305b8378b58',seed=61010611,source_deals={'train':1536,'validation':256,'test':256},trajectories_per_deal=16,train_roots=3648,training_worlds=128,reference_worlds=4096,
  controls={'mixed32':'3648roots128worldsH32, frozen cyclic early/mid/late priorities','late32':'3648late roots128worldsH32, identical eligible source-deal quotas','late64':'same late128dataH64 capacity diagnostic','subset128':'same912late roots128worldsH32','subset512':'same912late roots512worldsH32,128samples prefix-identical'},
  quotas='mine16histories per unconditional source, preserve missing late sources. Roundrobin3648quotas across sorted train sources with >=1latecandidate, eachquota<=latecount. Matched source coverage/multiplicity. Mixed priority [0,2,1] if sourceindex%3==0 else[0,1,2], cycling unique roots. Subset one late root from first912eligibletrain sources.',
  population='conditional live-late availability, label-independent but not outcome-independent: contract survival depends on public scores. No resampling unavailable deals, no all-deal strength claim.',
  contract='fixedbid30; trumpuniform0..6/bidderuniform0..3 independent of hands; no bidding',belief='uniform legal-compatible completions, ignores historical action likelihood/bidding; support-conditioned surrogate, not gameplay posterior',
  teacher0='frozen P-1 uniform legal for ALL future actors incl own; root all-action Q for acting partnership contract win; shared worlds and absolute-ply tapes; ascending exact ties',
  model='raw862inputs from previous pilot, no hidden/source/seed features;862->32or64ReLU->28 advantage regression; factor4targets, identical optimizer100epochs, validation-MSE checkpoint on same late128panel',
  evaluation='up to2late roots per heldout source plus early/mid generalization panel; missing late sources retained in256denominators. Selections independent of labels/models. Pair all arms on same reference; equal-weight whole-deal means and paired bootstrap; empirical3SEgap subset heuristic.',
  rung_gate='PRIMARY late32 only: late validation reference regret <=.9*mixed32, paired late32-minus-mixed32 upper95%<0, and beats random and ascending by paired upper95%<0. Capacity/precision arms diagnostic only. Freeze ALL controls before opening test.',
  rung='if gate passes: freeze late32 for ALL actors to generate one T1 all-action search dataset; redistill on EXACT late training roots and validation panel; keep fresh reference1 and same original reference0 comparisons. No improvement theorem.',
  resources='same nominal root-world main budgets3648*128; subset512=912*512, subset128quarter-budget; actual candidate continuations/ply/wall differ and recorded. Every executable<=295seconds process-group cap,2label workersmax, localMLXMetal, no transmissions/cloud/production. No extra model scaling beyond64diagnostic.')
 p.dump(HERE/'plan.json',plan);shutil.copyfile(BASE/'kernel.c',HERE/'kernel.c')
 old=json.loads(gzip.open(BASE/'sources.json.gz','rt').read());forbid={s['source_id'] for s in old};oldX={p.encode(r['request']).tobytes() for r in json.loads(gzip.open(BASE/'positions.json.gz','rt').read())}
 for f in (BASE/'results/h2h').rglob('game-*.json'):forbid.add(canonical(json.loads(f.read_text())['hands']))
 rng=random.Random(plan['seed']);sources=[];rows=[];membership={k:[] for k in ['mixed','late','subset','validation','test']};seenX=set();mining=[];pools=[]
 def register(req,si,band,split):
  key=info(req);x=p.encode(req).tobytes()
  if x in seenX or x in oldX:return None
  seenX.add(x);i=len(rows);rows.append(dict(index=i,source_index=si,source_id=sources[si]['source_id'],split=split,band=band,request=req,information_key=key));return i
 for si in range(2048):
  tiles=list(range(28));rng.shuffle(tiles);hands=[sorted(tiles[7*q:7*q+7]) for q in range(4)];sid=canonical(hands);assert sid not in forbid;forbid.add(sid);split='train' if si<1536 else 'validation' if si<1792 else 'test';decl=rng.randrange(7);bidder=rng.randrange(4)
  sources.append(dict(index=si,source_id=sid,split=split,hands=hands,decl=decl,bidder=bidder));candidates=[{}, {}, {}];trng=random.Random(plan['seed']^((si+1)*104729))
  for traj in range(16):
   remain=list(map(set,hands));leader=bidder;pts=[0,0];plays=[];tr=[]
   for ply in range(28):
    if pts[bidder%2]>=30 or pts[1-bidder%2]>=13:break
    actor=(leader+len(tr))%4;legal=p.rules.legal_tiles(remain[actor],tr,decl)
    if len(legal)>1 and ply<24:
     req=dict(decl=decl,bid=30,bidder=bidder,seat=actor,hand=hands[actor],plays=plays.copy(),seed=7042104);key=info(req);candidates[ply//8].setdefault(key,req)
    t=trng.choice(legal);remain[actor].remove(t);plays.extend([actor,t]);tr.append((actor,t))
    if len(tr)==4:leader=p.rules.winner(tr,decl);pts[leader%2]+=p.rules.trick_points(tr);tr=[]
  mining.append(dict(source_index=si,unique_candidates_by_band=[len(c) for c in candidates]))
  lists=[list(c.values()) for c in candidates]
  for lst in lists:trng.shuffle(lst)
  pools.append(lists)
 eligible=[si for si in range(1536) if pools[si][2]];quotas={si:0 for si in range(1536)}
 remaining=3648
 while remaining:
  allocated=0
  for si in eligible:
   if quotas[si]<len(pools[si][2]):quotas[si]+=1;remaining-=1;allocated+=1
   if remaining==0:break
  assert allocated, 'fixed16trajectory pool lacks3648late roots; stop instead of automatic expansion'
 assert len(eligible)>=912
 for si,s in enumerate(sources):
  split=s['split'];lists=pools[si];quota=quotas.get(si,0)
  if split=='train' and quota:
   before=len(membership['mixed']);priority=[0,2,1] if si%3==0 else [0,1,2];offset=[0,0,0];band_cursor=0
   while len(membership['mixed'])-before<quota:
    b=priority[band_cursor%3];band_cursor+=1
    if offset[b]>=len(lists[b]):continue
    req=lists[b][offset[b]];offset[b]+=1;i=register(req,si,b,split)
    if i is not None:membership['mixed'].append(i)
   # If a mixed root is itself late, share that exact information/root with
   # the late arm. No split duplication; arm membership may overlap.
   shared=[i for i in membership['mixed'][before:] if rows[i]['band']==2]
   membership['late']+=shared;remaining_quota=quota-len(shared)
   for req in lists[2]:
    if remaining_quota==0:break
    i=register(req,si,2,split)
    if i is not None:membership['late'].append(i);remaining_quota-=1
   assert remaining_quota==0,('missing latequota',si)
   if len(membership['subset'])<912:membership['subset'].append(next(i for i in membership['late'][-quota:] if rows[i]['source_index']==si))
  elif split!='train':
   # Validation/test main late panel is two label-independent roots/deal.
   for b,number in [(0,1),(1,1),(2,2)]:
    selected=0
    for req in lists[b]:
     i=register(req,si,b,split)
     if i is not None:
      if b==2:membership[split].append(i)
      selected+=1
     if selected==number:break
    if b==2:mining[si]['selected_late_roots']=selected
 assert len(membership['mixed'])==len(membership['late'])==3648 and len(membership['subset'])==912
 write('positions.json.gz',rows);write('sources.json.gz',sources);p.dump(HERE/'membership.json',membership);p.dump(HERE/'mining.json',mining);p.dump(HERE/'training-quotas.json',quotas)
 print(json.dumps(dict(source_deals=len(sources),unique_rows=len(rows),membership={k:len(v) for k,v in membership.items()},mixed_bands={str(b):sum(rows[i]['band']==b for i in membership['mixed']) for b in range(3)})))
def selected_rows(kind,batch):
 rs=positions();mem=json.loads((HERE/'membership.json').read_text())
 if kind in ['mixed128','late128','subset128','subset512']:
  key={'mixed128':'mixed','late128':'late','subset128':'subset','subset512':'subset'}[kind];ids=set(mem[key]+mem['validation'])
 elif kind=='late1':ids=set(mem['late']+mem['validation'])
 elif kind.startswith('reference') or kind=='teacher128':ids={r['index'] for r in rs if r['split']!='train'}
 else:raise ValueError(kind)
 return [r for r in rs if r['index'] in ids and r['source_index']//128==batch]
def generate(a):
 rows=selected_rows(a.kind,a.batch);out=HERE/'data'/a.kind;out.mkdir(parents=True,exist_ok=True);path=out/f'batch-{a.batch:03d}.npz';assert not path.exists() and rows
 X=[];M=[];Q=[];B=[];ids=[];record=[];start=time.perf_counter()
 for r in rows:
  W=4096 if a.kind.startswith('reference') else 512 if a.kind=='subset512' and r['split']=='train' else 128
  domain='T1-label-v1' if a.kind=='late1' else 'T1-reference-v1' if a.kind=='reference1' else 'T0-reference-v1' if a.kind.startswith('reference') else 'T0-label-v1'
  seed=int.from_bytes(hashlib.sha256(f"{domain}:{r['information_key']}".encode()).digest()[:8],'little');acts,y,ws,tp=p.label(r['request'],W,seed,a.model)
  # Uniform128 labels shared by overlapping arms;512 extends SAME128worlds/
  # tapes. Store padded packing so common128validation coexists with512train.
  storageW=512 if a.kind=='subset512' else W;m=np.zeros(28,np.float32);m[acts]=1;q=np.zeros(28,np.float32);q[acts]=y.mean(0);b=np.zeros((28,storageW//8),np.uint8);b[acts,:W//8]=np.packbits(y.T,axis=1)
  X.append(p.encode(r['request']));M.append(m);Q.append(q);B.append(b);ids.append(r['index']);record.append(dict(position_index=r['index'],source_id=r['source_id'],worlds=W,seed=seed,worlds_sha256=hashlib.sha256(ws.tobytes()).hexdigest(),tapes_sha256=hashlib.sha256(tp.tobytes()).hexdigest()))
 np.savez_compressed(path,X=np.array(X,np.float32),M=np.array(M,np.float32),Q=np.array(Q,np.float32),B=np.array(B,np.uint8),ids=np.array(ids,np.int32),W=np.array(storageW))
 meta=dict(kind=a.kind,positions=len(rows),legal_actions=int(sum(len(p.public(r['request'])[0]['legal']) for r in rows)),root_worlds=sum(r['worlds'] for r in record),candidate_continuations=sum(len(p.public(row['request'])[0]['legal'])*rec['worlds'] for row,rec in zip(rows,record)),seconds=time.perf_counter()-start,kernel_sha256=p.sha(HERE/'kernel.c'),binary_sha256=p.sha(HERE/'kernel.dylib'),base_pilot_sha256=p.sha(BASE/'pilot.py'),coverage_sha256=p.sha(__file__),model_sha256=p.sha(a.model) if a.model else None,dataset_sha256=p.sha(path),records=record)
 p.dump(path.with_suffix('.json'),meta);print(json.dumps({k:meta[k] for k in ['kind','positions','root_worlds','candidate_continuations','seconds']}))
def train(a):
 # Reuse frozen initial GPU trainer and raw encoding exactly. New membership
 # data contains ONLY intended train IDs + common late validation panel.
 p.train(a)
def main():
 parser=argparse.ArgumentParser();sub=parser.add_subparsers(dest='cmd',required=True);sub.add_parser('initialize');g=sub.add_parser('generate');g.add_argument('--kind',required=True);g.add_argument('--batch',type=int,required=True);g.add_argument('--model',type=Path);t=sub.add_parser('train');t.add_argument('--kind',required=True);t.add_argument('--name',required=True);t.add_argument('--hidden',type=int,default=32);t.add_argument('--epochs',type=int,default=100);a=parser.parse_args();globals()[a.cmd](a)
if __name__=='__main__':main()
