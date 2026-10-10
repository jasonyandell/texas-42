"""Independent final replay, actor-normalized encoding and noise arithmetic."""
import gzip,hashlib,importlib.util,json,sys
from pathlib import Path
import numpy as np
sys.dont_write_bytecode=True
HERE=Path(__file__).resolve().parent.parent
sp=importlib.util.spec_from_file_location('pilot',HERE/'pilot.py');p=importlib.util.module_from_spec(sp);sp.loader.exec_module(p)
rows=p.positions();seen={};within=0;cross=[]
for row in rows:
 # encode normalizes actor-relative public seats, so byte duplicate check also
 # catches raw inputs equivalent under the implementation's seat rotation.
 key=p.encode(row['request']).tobytes()
 if key in seen:
  if row['split']!=seen[key]['split']:cross.append([seen[key]['index'],row['index']])
  else:within+=1
 else:seen[key]=row
assert not cross,cross
summary=json.loads((HERE/'results/h2h/summary.json').read_text());arena={};expected_phone='40d7a1eea658627f7bab37d3c1888ad3bf00216fb23a5d2ceeb7d2d1f4219119'
source_masks={tuple(sorted(p.mask(h) for h in s['hands'])) for s in json.loads(gzip.open(HERE/'sources.json.gz','rt').read())}
for variant in summary:
 games=[json.loads(f.read_text()) for f in (HERE/'results/h2h'/variant).glob('game-*.json')];groups={};phonecalls=0;moves=0
 for g in games:
  assert g['phone_wasm_sha256']==expected_phone and g['complete'];assert tuple(sorted(p.mask(h) for h in g['hands'])) not in source_masks
  assert g['candidate_model_sha256']==(None if variant=='teacher0' else p.sha(HERE/'models'/f'{variant}.npz'))
  remaining=list(map(set,g['hands']));points=[0,0];leader=g['bidder'];tr=[];record=[];team=(g['bidder']+(g['role']=='defending'))%2
  for move in g['moves']:
   actor=(leader+len(tr))%4;req=move['request'];response=move['response'];choice=response['choice'];legal=p.rules.legal_tiles(remaining[actor],tr,g['decl'])
   assert req['hand']==g['hands'][actor] and req['seat']==actor and req['plays']==record
   assert move['actor']==actor and move['candidate']==(actor%2==team)
   assert response['legal']==legal and response['points']==points and response['leader']==leader and choice in legal
   if not move['candidate']:assert not move['timing']['interrupted'];phonecalls+=1
   remaining[actor].remove(choice);record.extend([actor,choice]);tr.append((actor,choice));moves+=1
   if len(tr)==4:leader=p.rules.winner(tr,g['decl']);points[leader%2]+=p.rules.trick_points(tr);tr=[]
  assert len(record)==56 and sum(points)==42 and points==g['points'] and all(not h for h in remaining)
  assert g['made']==(points[g['bidder']%2]>=30)
  groups[g['seed'],g['rotation'],g['role']]=g
 seeds=sorted({g['seed'] for g in games});assert len(seeds)==2 and len(groups)==16
 pairs=[int(groups[s,rot,'declaring']['made'])-int(groups[s,rot,'defending']['made']) for s in seeds for rot in range(4)]
 for v,key in ((1,'pair_wins'),(-1,'pair_losses'),(0,'pair_ties')):assert pairs.count(v)==summary[variant][key]
 arena[variant]={'games':len(games),'independent_source_deals':len(seeds),'moves':moves,'phone_calls':phonecalls,'pair_wins':pairs.count(1),'pair_losses':pairs.count(-1),'pair_ties':pairs.count(0)}
diagnostic=json.loads((HERE/'results/diagnostics.json').read_text());low,_=p.dataset('labels0');control,W=p.dataset('control');lookup={int(i):j for j,i in enumerate(low['ids'])};squares=[];treg=[];nreg=[];n0=dict(np.load(HERE/'models/n0-32.npz'))
for j,idx in enumerate(control['ids']):
 assert rows[int(idx)]['split']=='train';legal=np.flatnonzero(control['M'][j]);q=control['Q'][j,legal];lq=low['Q'][lookup[int(idx)],legal]
 squares.extend(((q-q.mean())-(lq-lq.mean()))**2)
 treg.append(q.max()-q[np.argmax(lq)]);pred=p.predict(n0,control['X'][j:j+1])[0];nreg.append(q.max()-q[np.argmax(pred[legal])])
assert W==4096 and len(treg)==96
for name,val in [('centered_advantage_label_RMSE',np.sqrt(np.mean(squares))),('low_teacher_reference_regret',np.mean(treg)),('selected_n0_training_root_regret',np.mean(nreg))]:assert abs(float(val)-diagnostic[name])<1e-7
print(json.dumps({'raw_actor_normalized_encoding':{'roots':len(rows),'unique_vectors':len(seen),'within_split_duplicate_vectors':within,'cross_split_duplicate_vectors':len(cross)},'h2h':arena,'diagnostics':{'training_roots':96,'reference_worlds':W,'arithmetic':'pass'}},indent=2))
