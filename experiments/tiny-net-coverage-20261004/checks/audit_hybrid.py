"""Independent complete-game replay and exact late-only route checking."""
import gzip,importlib.util,json,sys
from pathlib import Path
import numpy as np
sys.dont_write_bytecode=True
HERE=Path(__file__).resolve().parent.parent
sp=importlib.util.spec_from_file_location('cov',HERE/'coverage.py');c=importlib.util.module_from_spec(sp);sp.loader.exec_module(c);p=c.p
forbidden={s['source_id'] for folder in (HERE,c.BASE) for s in json.loads(gzip.open(folder/'sources.json.gz','rt').read())}
for f in (c.BASE/'results/h2h').rglob('game-*.json'):forbidden.add(c.canonical(json.loads(f.read_text())['hands']))
summary=json.loads((HERE/'results/h2h/summary.json').read_text());report={}
for variant in summary:
 groups={};eligible_count=override_count=phone_count=0
 model=None if variant=='teacher0' else dict(np.load(HERE/'models'/f'{variant}.npz'))
 for file in (HERE/'results/h2h'/variant).glob('game-*.json'):
  game=json.loads(file.read_text());hands=game['hands'];sid=c.canonical(hands);assert sid==game['source_id'] and sid not in forbidden
  assert game['phone_wasm_sha256']=='40d7a1eea658627f7bab37d3c1888ad3bf00216fb23a5d2ceeb7d2d1f4219119'
  assert game['candidate_model_sha256']==(None if model is None else p.sha(HERE/'models'/f'{variant}.npz'))
  assert game['complete'] and len(game['moves'])==28
  remain=list(map(set,hands));leader=game['bidder'];points=[0,0];tr=[];history=[];team=(game['bidder']+(game['role']=='defending'))%2
  for ply,move in enumerate(game['moves']):
   actor=(leader+len(tr))%4;legal=p.rules.legal_tiles(remain[actor],tr,game['decl']);req=move['request'];resp=move['response'];choice=resp['choice']
   assert req['hand']==hands[actor] and req['plays']==history and req['seat']==actor and req['bidder']==game['bidder']
   assert move['ply']==ply and move['actor']==actor and move['candidate']==(actor%2==team)
   eligible=16<=ply<24 and len(legal)>1 and points[game['bidder']%2]<30 and points[1-game['bidder']%2]<13
   override=eligible and actor%2==team;assert move['eligible']==eligible and move['override']==override;eligible_count+=eligible
   assert resp['points']==points and resp['leader']==leader and resp['legal']==legal and choice in legal
   if override:
    override_count+=1;assert move['route']==('T0-128' if model is None else variant)
    if model is None:acts,y,_,_=p.label(req,128,7042104+ply);expected=int(acts[np.argmax(y.mean(0))])
    else:z=p.predict(model,p.encode(req)[None])[0];expected=max(legal,key=lambda t:(z[t],-t))
    assert choice==expected
   else:phone_count+=1;assert move['route']=='phone' and not move['timing']['interrupted']
   remain[actor].remove(choice);history.extend([actor,choice]);tr.append((actor,choice))
   if len(tr)==4:leader=p.rules.winner(tr,game['decl']);points[leader%2]+=p.rules.trick_points(tr);tr=[]
  assert sum(points)==42 and points==game['points'] and all(not h for h in remain) and not tr
  assert game['made']==(points[game['bidder']%2]>=30)
  groups[game['seed'],game['rotation'],game['role']]=game
 seeds=sorted({key[0] for key in groups});assert len(seeds)==2 and len(groups)==16
 pairs=[int(groups[s,rot,'declaring']['made'])-int(groups[s,rot,'defending']['made']) for s in seeds for rot in range(4)]
 for value,key in ((1,'pair_wins'),(-1,'pair_losses'),(0,'pair_ties')):assert pairs.count(value)==summary[variant][key]
 assert summary[variant]['eligible_moves']==eligible_count and summary[variant]['overridden_moves']==override_count and summary[variant]['phone_calls']==phone_count
 report[variant]={'games':16,'independent_source_deals':2,'replayed_moves':448,'eligible_moves':eligible_count,'exact_replayed_overrides':override_count,'phone_calls':phone_count,'pair_wins_losses_ties':[pairs.count(1),pairs.count(-1),pairs.count(0)]}
print(json.dumps(report,indent=2))
