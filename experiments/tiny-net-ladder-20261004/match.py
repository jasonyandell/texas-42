#!/usr/bin/env python3
"""One complete net/teacher versus pinned phone game, capped externally."""
import argparse,importlib.util,json,random,time
from pathlib import Path
import numpy as np
from pilot import HERE,encode,predict,label,dump,sha,rules
sp=importlib.util.spec_from_file_location('phone_arena',HERE.parent/'astra-sol-20261004/tools/arena.py');phone=importlib.util.module_from_spec(sp);sp.loader.exec_module(phone)
def run(a):
 out=HERE/'results'/'h2h'/a.variant/f'game-{a.seed}-{a.rotation}-{a.role}.json';out.parent.mkdir(parents=True,exist_ok=True);assert not out.exists()
 tiles=list(range(28));random.Random(a.seed).shuffle(tiles);base=[sorted(tiles[7*q:7*q+7]) for q in range(4)];hands=[base[(q-a.rotation)%4] for q in range(4)];bidder=a.rotation;team=(bidder+(a.role=='defending'))%2;decl=6
 remain=list(map(set,hands));plays=[];points=[0,0];leader=bidder;moves=[];worker=phone.Worker(phone.PHONE);p=dict(np.load(HERE/'models'/f'{a.variant}.npz')) if a.variant!='teacher0' else None
 start=time.monotonic();deadline=start+100
 try:
  for ply in range(28):
   assert time.monotonic()<deadline,'incomplete game excluded'
   npl=ply%4;actor=(leader+npl)%4;tr=list(zip(plays[-2*npl::2],plays[-2*npl+1::2])) if npl else [];legal=rules.legal_tiles(remain[actor],tr,decl);candidate=actor%2==team
   req=phone.make_request(hands,actor,bidder,plays,decl);s=rules.information_state(req);assert s['points']==points and s['leader']==leader and s['legal']==legal
   ts=time.perf_counter()
   if candidate:
    if len(legal)==1:choice=legal[0];route='forced'
    elif points[bidder%2]>=30 or points[1-bidder%2]>=13:choice=legal[0];route='settled-ascending'
    elif p is not None:v=predict(p,encode(req)[None])[0];choice=max(legal,key=lambda z:(v[z],-z));route=a.variant
    else:
     # Policy seed independent of hidden deal seed. Every root all-action
     # search uses same belief/tape samples. Contract already settled fallback.
     acts,y,_,_=label(req,128,7042104+ply);choice=int(acts[np.argmax(y.mean(0))]);route='T0-128'
    response=dict(choice=choice,legal=legal,points=points.copy(),leader=leader,route=route);timing={'host_ms':(time.perf_counter()-ts)*1000,'interrupted':False}
   else:
    response,timing=worker.call(phone.profile(req,True),deadline);choice=response['choice'];assert response['legal']==legal and response['points']==points and response['leader']==leader
   assert choice in legal;moves.append(dict(actor=actor,candidate=candidate,request=req,response=response,timing=timing));remain[actor].remove(choice);plays.extend([actor,choice]);tr.append((actor,choice))
   if len(tr)==4:leader=rules.winner(tr,decl);points[leader%2]+=rules.trick_points(tr)
 finally:worker.close()
 audited,_,remaining,tail=rules.replay_record(hands,plays,decl,bidder);assert audited==points and sum(points)==42 and not tail and all(not x for x in remaining)
 assert phone.sha(phone.PHONE)==json.loads((phone.ROOT/'reference/production-phone/manifest.json').read_text())['wasm_sha256']
 report=dict(tier='exploratory',variant=a.variant,seed=a.seed,rotation=a.rotation,role=a.role,decl=decl,bid=30,bidder=bidder,hands=hands,complete=True,points=points,made=points[bidder%2]>=30,moves=moves,game_seconds=time.monotonic()-start,phone_wasm_sha256=sha(phone.PHONE),candidate_model_sha256=sha(HERE/'models'/f'{a.variant}.npz') if p else None,match_source_sha256=sha(__file__),kernel_sha256=sha(HERE/'kernel.c'),policy_seed=7042104)
 dump(out,report);print(json.dumps({k:report[k] for k in ['variant','seed','rotation','role','made','points','game_seconds']}))
def summarize(a):
 output={}
 for variant in ['teacher0','n0-32','n1-32']:
  rows=[json.loads(p.read_text()) for p in sorted((HERE/'results/h2h'/variant).glob('game-*.json'))];assert rows
  seeds=sorted({r['seed'] for r in rows});groups={(r['seed'],r['rotation'],r['role']):r for r in rows};pairs=[];dealmeans=[]
  for seed in seeds:
   ds=[]
   for rotation in range(4):
    x=groups[(seed,rotation,'declaring')];y=groups[(seed,rotation,'defending')];d=int(x['made'])-int(y['made']);pairs.append(d);ds.append(d)
   dealmeans.append(sum(ds)/4)
  output[variant]=dict(games=len(rows),source_deals=len(seeds),pair_wins=pairs.count(1),pair_losses=pairs.count(-1),pair_ties=pairs.count(0),mean_paired_make_difference=sum(pairs)/len(pairs),deal_mean_differences=dealmeans,phone_interruptions=sum(m['timing']['interrupted'] for r in rows for m in r['moves'] if not m['candidate']),candidate_host_ms=sum(m['timing']['host_ms'] for r in rows for m in r['moves'] if m['candidate']),phone_host_ms=sum(m['timing']['host_ms'] for r in rows for m in r['moves'] if not m['candidate']),identity=dict(phone=rows[0]['phone_wasm_sha256'],model=rows[0]['candidate_model_sha256']),complete_balanced_blocks=True,strength_claim=False,uncertainty='two independent source deals; four rotations/role swaps are correlated; conservative bounded95% interval[-1,1]; smoke only')
 dump(HERE/'results/h2h/summary.json',output);print(json.dumps(output))
p=argparse.ArgumentParser();sp=p.add_subparsers(dest='command',required=True);r=sp.add_parser('run');r.add_argument('--variant',choices=['n0-32','n1-32','teacher0'],required=True);r.add_argument('--seed',type=int,required=True);r.add_argument('--rotation',type=int,choices=range(4),required=True);r.add_argument('--role',choices=['declaring','defending'],required=True);sp.add_parser('summarize');a=p.parse_args();globals()[a.command](a)
