#!/usr/bin/env python3
"""Local-only bounded pilot; invoke each command through run_capped.py."""
import argparse,ctypes,gzip,hashlib,importlib.util,json,os,random,sys,time
from pathlib import Path
sys.dont_write_bytecode=True
os.environ.setdefault('OPENBLAS_NUM_THREADS','2')
import numpy as np
HERE=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('referee',HERE.parent/'partnership/rules.py')
rules=importlib.util.module_from_spec(spec);spec.loader.exec_module(rules)
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def dump(p,x):Path(p).write_text(json.dumps(x,indent=2)+'\n')
def mask(h):return sum(1<<t for t in h)
def arrays(v,dtype):return np.ascontiguousarray(v,dtype=dtype)
def ptr(a):return a.ctypes.data_as(ctypes.c_void_p)
def loadkernel():
 k=ctypes.CDLL(str(HERE/'kernel.dylib'))
 k.sample.argtypes=[ctypes.c_uint32,ctypes.c_void_p,ctypes.c_int,ctypes.c_void_p,ctypes.c_void_p,ctypes.c_void_p,ctypes.c_int,ctypes.c_uint64,ctypes.c_void_p];k.sample.restype=ctypes.c_int
 k.rollout.argtypes=[ctypes.c_void_p,ctypes.c_int,ctypes.c_void_p,ctypes.c_int,ctypes.c_int,ctypes.c_int,ctypes.c_int,ctypes.c_void_p,ctypes.c_void_p,ctypes.c_int,ctypes.c_int,ctypes.c_void_p,ctypes.c_void_p,ctypes.c_int,ctypes.c_void_p,ctypes.c_void_p]
 return k
def public(req):
 s=rules.information_state(req);played=list(zip(req['plays'][::2],req['plays'][1::2]));current=set(req['hand'])-{t for q,t in played if q==req['seat']}
 return s,played,current
def encode(req):
 s,played,current=public(req);actor=req['seat'];st=np.ones(28,dtype=np.int32)
 for t in current:st[t]=0
 for p,(q,t) in enumerate(played):st[t]=2+((q-actor)%4)*7+p//4
 x=np.zeros(862,np.float32);x[np.arange(28)*30+st]=1
 x[840+req['decl']]=1;x[847+(req['bidder']-actor)%4]=1;x[851+(s['leader']-actor)%4]=1;x[855+len(played)%4]=1
 x[859]=s['points'][actor%2]/42;x[860]=s['points'][1-actor%2]/42;x[861]=req['bid']/42
 return x
def worlds(req,W,seed):
 s,played,current=public(req);viewer=req['seat'];others=[q for q in range(4) if q!=viewer];void=[set() for _ in range(4)];tr=[];counts=[0]*4
 for q,t in played:
  if tr and not rules.follows(t,rules.context(tr[0][1],req['decl']),req['decl']):void[q].update(z for z in range(28) if rules.follows(z,rules.context(tr[0][1],req['decl']),req['decl']))
  counts[q]+=1;tr.append((q,t))
  if len(tr)==4:tr=[]
 unseen=sorted(set(range(28))-current-{t for q,t in played});allow=[sum(1<<i for i,q in enumerate(others) if t not in void[q]) for t in unseen]
 u=arrays(unseen,np.int32);a=arrays(allow,np.int32);o=arrays(others+[viewer],np.int32);need=arrays([7-counts[q] for q in others],np.int32);out=np.zeros((W,4),np.uint32)
 assert loadkernel().sample(mask(current),ptr(u),len(u),ptr(a),ptr(o),ptr(need),W,seed,ptr(out))==W
 return out
def flatweights(model):
 if model is None:return 0,np.zeros(1,np.float32)
 p=dict(np.load(model));return len(p['b1']),arrays(np.concatenate([p['w1'].ravel(),p['b1'],p['w2'].T.ravel(),p['b2']]),np.float32)
def label(req,W,seed,model=None):
 s,played,current=public(req);ws=worlds(req,W,seed);rng=np.random.default_rng(seed^0xDEADBEEF);tapes=arrays(rng.random((W,28)),np.float64)
 acts=arrays(s['legal'],np.int32);pa=arrays([q for q,t in played],np.int32);pt=arrays([t for q,t in played],np.int32);pts=arrays(s['points'],np.int32);out=np.zeros((W,len(acts)),np.uint8);H,p=flatweights(model)
 loadkernel().rollout(ptr(ws),W,ptr(acts),len(acts),req['decl'],req['bidder'],req['seat'],ptr(pa),ptr(pt),len(played),s['leader'],ptr(pts),ptr(tapes),H,ptr(p),ptr(out))
 return acts,out,ws,tapes
def initialize(a):
 assert not (HERE/'plan.json').exists()
 plan=dict(schema='tiny-net-ladder-plan-v1',tier='exploratory',source_base_commit='92cc1ac3',source_deals=2048,split_deals={'train':1536,'validation':256,'test':256},roots_per_deal_max=3,root_strata=[[0,7],[8,15],[16,23]],contract='fixed bid30; trump uniform0..6; bidder uniform0..3 independent of hands; no bidding model',label_worlds=128,reference_worlds=4096,seed=61010411,
  belief_model='uniform over full deals compatible with own original hand and legal public history, ignores action likelihood and bidding; support-conditioned surrogate, not true gameplay posterior',
  rung_indexing='P-1=uniform legal continuation for every actor; T0=all-action one-deviation Q under frozen P-1; N0=advantage regression of T0; P0=legal argmax N0; T1=one-deviation Q under frozen P0 for ALL actors; N1=regression T1',
  ties='ascending physical tile id at exact argmax; random baseline expected uniform legal action',
  gate='validation only: T0-128 teacher beats uniform random and ascending; N0 reduces mean reference regret >=20% vs uniform random, paired source-deal bootstrap upper95% net-minus-random <0, and net mean regret <= ascending; otherwise diagnostic capacity/noise control, no automatic rung',
  resources='each command process-group watchdog <=295sec, native labels at most two workers, GPU MLX training; no cloud/no external sends; initial 2048deals, no test tuning',
  encoding='862 raw inputs: tile x own-current/unseen/relative-seat-by-trick (30 states), trump7, bidder4, leader4, table-offset4, own/opp points and bid (3 scaled scalars); no hidden hands/source id/rng seed',
  architecture='862->32 ReLU->28, masked centered action-advantage MSE per root; also linear and ascending/highest/random baselines')
 dump(HERE/'plan.json',plan);rng=random.Random(plan['seed']);rows=[];sources=[];seen_source=set();seen_info=set()
 for i in range(plan['source_deals']):
  tiles=list(range(28));rng.shuffle(tiles);hands=[sorted(tiles[7*q:7*q+7]) for q in range(4)]
  sid=hashlib.sha256(json.dumps(sorted(mask(h) for h in hands)).encode()).hexdigest();assert sid not in seen_source;seen_source.add(sid)
  split='train' if i<1536 else 'validation' if i<1792 else 'test';decl=rng.randrange(7);bidder=rng.randrange(4);remain=list(map(set,hands));leader=bidder;pts=[0,0];plays=[];candidates=[[],[],[]]
  for ply in range(28):
   if pts[bidder%2]>=30 or pts[1-bidder%2]>=13:break
   actor=(leader+ply%4)%4;tr=list(zip(plays[-2*(ply%4)::2],plays[-2*(ply%4)+1::2])) if ply%4 else []
   legal=rules.legal_tiles(remain[actor],tr,decl)
   if len(legal)>1 and ply<24:
    req=dict(decl=decl,bid=30,bidder=bidder,seat=actor,hand=hands[actor],plays=list(plays),seed=7042104);candidates[ply//8].append(req)
   tile=rng.choice(legal);remain[actor].remove(tile);plays.extend([actor,tile]);tr.append((actor,tile))
   if len(tr)==4:leader=rules.winner(tr,decl);pts[leader%2]+=rules.trick_points(tr)
  for band,c in enumerate(candidates):
   if not c:continue
   req=rng.choice(c);key=hashlib.sha256(json.dumps(req,sort_keys=True).encode()).hexdigest()
   if key in seen_info:continue
   seen_info.add(key);rows.append(dict(index=len(rows),source_index=i,source_id=sid,split=split,band=band,request=req,information_key=key))
  sources.append(dict(index=i,source_id=sid,split=split,hands=hands,decl=decl,bidder=bidder))
 with gzip.open(HERE/'positions.json.gz','wt') as f:json.dump(rows,f)
 with gzip.open(HERE/'sources.json.gz','wt') as f:json.dump(sources,f)
 print(json.dumps(dict(positions=len(rows),deals=len(sources),splits={q:sum(r['split']==q for r in rows) for q in ['train','validation','test']})))
def positions():
 with gzip.open(HERE/'positions.json.gz','rt') as f:return json.load(f)
def generate(a):
 out=HERE/'data'/a.kind;out.mkdir(parents=True,exist_ok=True);path=out/f'batch-{a.batch:03d}.npz';assert not path.exists()
 plan=json.loads((HERE/'plan.json').read_text());start=time.perf_counter();rows=[r for r in positions() if r['source_index']//128==a.batch]
 if a.kind.startswith('reference'):rows=[r for r in rows if r['split']!='train']
 if a.kind=='control':rows=rows[:96]
 W=a.worlds or (plan['reference_worlds'] if a.kind.startswith('reference') else plan['label_worlds']);X=[];M=[];Q=[];B=[];ids=[];record=[]
 for r in rows:
  seed=int.from_bytes(hashlib.sha256(f"{a.kind}:{r['information_key']}:v1".encode()).digest()[:8],'little');legal,y,ws,tp=label(r['request'],W,seed,a.model)
  m=np.zeros(28,np.float32);m[legal]=1;q=np.zeros(28,np.float32);q[legal]=y.mean(0);b=np.zeros((28,(W+7)//8),np.uint8);b[legal]=np.packbits(y.T,axis=1)
  X.append(encode(r['request']));M.append(m);Q.append(q);B.append(b);ids.append(r['index']);record.append(dict(position_index=r['index'],source_id=r['source_id'],seed=seed,worlds_sha256=hashlib.sha256(ws.tobytes()).hexdigest(),tapes_sha256=hashlib.sha256(tp.tobytes()).hexdigest()))
 np.savez_compressed(path,X=np.array(X,np.float32),M=np.array(M,np.float32),Q=np.array(Q,np.float32),B=np.array(B,np.uint8),ids=np.array(ids,np.int32),W=np.array(W))
 meta=dict(kind=a.kind,worlds=W,positions=len(rows),actions=int(np.sum(M)),seconds=time.perf_counter()-start,source_kernel_sha256=sha(HERE/'kernel.c'),binary_sha256=sha(HERE/'kernel.dylib'),pilot_sha256=sha(__file__),model_sha256=sha(a.model) if a.model else None,dataset_sha256=sha(path),records=record)
 dump(path.with_suffix('.json'),meta);print(json.dumps({k:meta[k] for k in ['kind','worlds','positions','actions','seconds']}))
def dataset(kind):
 files=sorted((HERE/'data'/kind).glob('*.npz'));assert files,kind;data=[dict(np.load(p)) for p in files]
 return {k:np.concatenate([d[k] for d in data]) for k in ['X','M','Q','B','ids']},int(data[0]['W'])
def predict(p,X):return (np.maximum(X@p['w1']+p['b1'],0)@p['w2']+p['b2']) if 'w1' in p else X@p['w']+p['b']
def train(a):
 import mlx.core as mx
 from mlx.utils import tree_flatten
 mx.set_default_device(mx.gpu)
 data,W=dataset(a.kind);rows=positions();split=np.array([rows[int(i)]['split'] for i in data['ids']]);tr=np.flatnonzero(split=='train');va=np.flatnonzero(split=='validation');assert len(tr) and len(va)
 rng=np.random.default_rng(12345);X=mx.array(data['X'][tr]);M=mx.array(data['M'][tr]);Q=mx.array(data['Q'][tr]);num=mx.sum(M,axis=1,keepdims=True);Y=4*(Q-mx.sum(Q*M,axis=1,keepdims=True)/num)*M
 H=a.hidden;D=862
 if H:p={'w1':mx.array((rng.standard_normal((D,H))*.08).astype(np.float32)),'b1':mx.zeros(H),'w2':mx.array((rng.standard_normal((H,28))*.04).astype(np.float32)),'b2':mx.zeros(28)}
 else:p={'w':mx.zeros((D,28)),'b':mx.zeros(28)}
 def loss(p,x,m,y):
  z=mx.maximum(x@p['w1']+p['b1'],0)@p['w2']+p['b2'] if H else x@p['w']+p['b']
  z=z-mx.sum(z*m,axis=1,keepdims=True)/mx.sum(m,axis=1,keepdims=True)
  return mx.mean(mx.sum((z-y)**2*m,axis=1)/mx.sum(m,axis=1))+1e-5*sum(mx.sum(v*v) for v in p.values())
 vg=mx.value_and_grad(loss);mom={k:mx.zeros_like(v) for k,v in p.items()};var={k:mx.zeros_like(v) for k,v in p.items()};step=0;start=time.perf_counter();best=float('inf');history=[];out=HERE/'models'/a.name;out.parent.mkdir(exist_ok=True);assert not out.with_suffix('.npz').exists()
 for ep in range(a.epochs):
  order=rng.permutation(len(tr));vals=[]
  for off in range(0,len(tr),256):
   j=mx.array(order[off:off+256]);v,g=vg(p,X[j],M[j],Y[j]);step+=1
   for k in p:
    mom[k]=.9*mom[k]+.1*g[k];var[k]=.999*var[k]+.001*g[k]*g[k];p[k]=p[k]-.002*(mom[k]/(1-.9**step))/(mx.sqrt(var[k]/(1-.999**step))+1e-8)
   mx.eval(p,mom,var,v);vals.append(float(v.item()))
  pn={k:np.array(v) for k,v in p.items()};z=predict(pn,data['X'][va]);m=data['M'][va];q=data['Q'][va];z-=np.sum(z*m,axis=1,keepdims=True)/m.sum(1,keepdims=True);y=4*(q-np.sum(q*m,axis=1,keepdims=True)/m.sum(1,keepdims=True));vl=float(np.mean(np.sum((z-y)**2*m,1)/m.sum(1)))
  history.append(dict(epoch=ep+1,train_loss=float(np.mean(vals)),validation_advantage_mse_scaled=vl))
  if vl<best:best=vl;np.savez(out.with_suffix('.npz'),**pn);chosen=ep+1
  if (ep+1)%10==0:print(json.dumps(history[-1]),flush=True)
 meta=dict(name=a.name,kind=a.kind,hidden=H,device='MLX Metal GPU',epochs=a.epochs,selected_epoch=chosen,selection='minimum validation centered advantage MSE, test excluded',seconds=time.perf_counter()-start,parameter_count=sum(v.size for v in pn.values()),raw_float32_bytes=4*sum(v.size for v in pn.values()),file_bytes=out.with_suffix('.npz').stat().st_size,weights_sha256=sha(out.with_suffix('.npz')),history=history,train_roots=len(tr),validation_roots=len(va))
 dump(out.with_suffix('.json'),meta);print(json.dumps({k:v for k,v in meta.items() if k!='history'}))
def main():
 p=argparse.ArgumentParser();sub=p.add_subparsers(dest='cmd',required=True);sub.add_parser('initialize')
 g=sub.add_parser('generate');g.add_argument('--kind',default='labels0');g.add_argument('--batch',type=int,required=True);g.add_argument('--worlds',type=int);g.add_argument('--model',type=Path)
 t=sub.add_parser('train');t.add_argument('--kind',default='labels0');t.add_argument('--name',required=True);t.add_argument('--hidden',type=int,default=32);t.add_argument('--epochs',type=int,default=80)
 a=p.parse_args();globals()[a.cmd](a)
if __name__=='__main__':main()
