"""Independent local checks; execute under the inherited process-group cap."""
import ctypes, importlib.util, itertools, json, sys
from collections import Counter
from pathlib import Path
import numpy as np
sys.dont_write_bytecode=True
HERE=Path(__file__).resolve().parent
sp=importlib.util.spec_from_file_location('pilot',HERE.parent/'pilot.py')
p=importlib.util.module_from_spec(sp);sp.loader.exec_module(p)
r=p.rules
lib=ctypes.CDLL(str(HERE/'wrapper.dylib'))
lib.audit_legal.argtypes=[ctypes.c_uint32,ctypes.c_int,ctypes.c_int]
lib.audit_strength.argtypes=[ctypes.c_int,ctypes.c_int,ctypes.c_int]
lib.audit_net.argtypes=[ctypes.c_uint32]+[ctypes.c_int]*5+[ctypes.c_void_p]*3+[ctypes.c_int,ctypes.c_uint32,ctypes.c_int,ctypes.c_void_p]
rng=np.random.default_rng(991601)
checks={}

# Exhaustive tile-pair winner and following comparisons, all supported trumps.
n=0
for d in range(7):
 for ledtile in range(28):
  led=7 if r.called(ledtile,d) else r.TILES[ledtile][0]
  for a,b in itertools.combinations(range(28),2):
   h=(1<<a)|(1<<b)
   expect=p.mask(r.legal_tiles({a,b},[(0,ledtile)],d))
   assert lib.audit_legal(h,led,d)==expect
   sa=lib.audit_strength(a,led,d);sb=lib.audit_strength(b,led,d)
   def pystrength(t):
    hi,lo=r.TILES[t]
    return (2 if r.called(t,d) else 1 if r.follows(t,r.context(ledtile,d),d) else 0,12 if hi==lo else hi+lo)
   if sa or sb:
    assert (sa>sb)-(sa<sb)==(pystrength(a)>pystrength(b))-(pystrength(a)<pystrength(b))
   n+=1
checks['rule_pairs']=n

# Tiny exact sampler support: enumerate every valid capacity assignment.
u=p.arrays(range(6),np.int32);allow=p.arrays([7,3,6,5,7,7],np.int32);others=p.arrays([0,1,2,3],np.int32);need=p.arrays([2,2,2],np.int32)
support=[]
for assign in itertools.product(range(3),repeat=6):
 if any(assign.count(q)!=2 for q in range(3)) or any(not(int(allow[i])&(1<<q)) for i,q in enumerate(assign)):continue
 masks=tuple(sum(1<<i for i,q0 in enumerate(assign) if q0==q) for q in range(3))
 support.append(masks)
W=30000;out=np.zeros((W,4),np.uint32)
assert p.loadkernel().sample(1<<10,p.ptr(u),6,p.ptr(allow),p.ptr(others),p.ptr(need),W,712331,p.ptr(out))==W
freq=Counter(tuple(map(int,x[:3])) for x in out)
assert set(freq)==set(support) and np.all(out[:,3]==1<<10)
chi=sum((freq[x]-W/len(support))**2/(W/len(support)) for x in support)
assert chi<3*len(support), (chi,len(support))
checks['sampler']={'samples':W,'exact_support_size':len(support),'chi_square':chi}

def python_roll(world,req,action,tape):
 points,leader,_,trick=r.replay_record([set(h) for h in req['_hands']],req['plays'],req['decl'],req['bidder'])
 hands=[{t for t in range(28) if int(world[q])>>t&1} for q in range(4)]
 for ply in range(len(req['plays'])//2,28):
  actor=(leader+len(trick))%4;legal=r.legal_tiles(hands[actor],trick,req['decl'])
  tile=action if ply==len(req['plays'])//2 else legal[int(tape[ply]*len(legal))]
  assert tile in legal;hands[actor].remove(tile);trick.append((actor,tile))
  if len(trick)==4:
   leader=r.winner(trick,req['decl']);points[leader%2]+=r.trick_points(trick);trick=[]
  if points[req['bidder']%2]>=30 or points[1-req['bidder']%2]>=13:break
 made=points[req['bidder']%2]>=30
 return int(made if req['seat']%2==req['bidder']%2 else not made)

roots=outcomes=encodings=0
for deal in range(28):
 tiles=rng.permutation(28);hands=[sorted(map(int,tiles[7*q:7*q+7])) for q in range(4)];d=deal%7;bidder=deal%4;remaining=list(map(set,hands));leader=bidder;plays=[];tr=[];pts=[0,0]
 for ply in range(24):
  if pts[bidder%2]>=30 or pts[1-bidder%2]>=13:break
  actor=(leader+len(tr))%4;legal=r.legal_tiles(remaining[actor],tr,d)
  if ply%3==0 and len(legal)>1:
   req=dict(decl=d,bid=30,bidder=bidder,seat=actor,hand=hands[actor],plays=plays.copy(),seed=1)
   acts,y,ws,tp=p.label(req,17,deal*99+ply+500)
   rr={**req,'_hands':hands}
   for w in range(17):
    for j,a in enumerate(acts):assert int(y[w,j])==python_roll(ws[w],rr,int(a),tp[w]);outcomes+=1
   # Native and Python policy receive exactly the same raw own/public encoding.
   H=5;model={k:v.astype(np.float32) for k,v in {'w1':rng.normal(size=(862,H)),'b1':rng.normal(size=H),'w2':rng.normal(size=(H,28)),'b2':rng.normal(size=28)}.items()}
   flat=p.arrays(np.concatenate([model['w1'].ravel(),model['b1'],model['w2'].T.ravel(),model['b2']]),np.float32)
   s,played,current=p.public(req);pa=p.arrays([q for q,t in played],np.int32);pt=p.arrays([t for q,t in played],np.int32);points=p.arrays(s['points'],np.int32)
   pred=p.predict(model,p.encode(req)[None])[0];best=min(s['legal'],key=lambda t:(-pred[t],t))
   native=lib.audit_net(p.mask(current),actor,d,bidder,leader,len(tr),p.ptr(points),p.ptr(pa),p.ptr(pt),len(played),p.mask(s['legal']),H,p.ptr(flat))
   assert native==best,(native,best);encodings+=1;roots+=1
  t=int(rng.choice(legal));remaining[actor].remove(t);plays.extend([actor,t]);tr.append((actor,t))
  if len(tr)==4:leader=r.winner(tr,d);pts[leader%2]+=r.trick_points(tr);tr=[]
checks['native_python']={'roots':roots,'paired_outcomes':outcomes,'net_encoding_choices':encodings,'bidder_parities':[0,1]}
# Actual frozen nets: actor decision depends on own/public information only.
for model_path in sorted((HERE.parent/'models').glob('n?-32.npz')):
 model=dict(np.load(model_path));H,flat=p.flatweights(model_path);actual=0
 for row in p.positions():
  req=row['request'];s,played,current=p.public(req);actor=req['seat']
  pa=p.arrays([q for q,t in played],np.int32);pt=p.arrays([t for q,t in played],np.int32);pts=p.arrays(s['points'],np.int32)
  pred=p.predict(model,p.encode(req)[None])[0];best=min(s['legal'],key=lambda t:(-pred[t],t))
  native=lib.audit_net(p.mask(current),actor,req['decl'],req['bidder'],s['leader'],len(played)%4,p.ptr(pts),p.ptr(pa),p.ptr(pt),len(played),p.mask(s['legal']),H,p.ptr(flat))
  assert native==best,(row['index'],native,best);actual+=1
 checks['actual_frozen_'+model_path.stem]={'source_sha256':p.sha(model_path),'all_position_choices':actual,'hidden_hand_inputs':0}
print(json.dumps(checks,indent=2))
