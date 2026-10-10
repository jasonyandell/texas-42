#!/usr/bin/env python3
"""Bounded late-only hybrid smoke comparison with the pinned phone policy."""
import argparse,importlib.util,json,random,time
from collections import Counter
import numpy as np
from coverage import HERE,BASE,p,canonical
sp=importlib.util.spec_from_file_location('phone_arena',HERE.parent/'astra-sol-20261004/tools/arena.py');phone=importlib.util.module_from_spec(sp);sp.loader.exec_module(phone)
VARIANTS=['teacher0','mixed32','late32'];SEEDS=[61010731,61010732]
def run(a):
 out=HERE/'results/h2h'/a.variant/f'game-{a.seed}-{a.rotation}-{a.role}.json';out.parent.mkdir(parents=True,exist_ok=True);assert not out.exists()
 tiles=list(range(28));random.Random(a.seed).shuffle(tiles);base=[sorted(tiles[7*q:7*q+7]) for q in range(4)];hands=[base[(q-a.rotation)%4] for q in range(4)];sid=canonical(hands)
 # Explicit source separation from both experiments and initial phone games.
 import gzip
 forbidden={s['source_id'] for folder in [HERE,BASE] for s in json.loads(gzip.open(folder/'sources.json.gz','rt').read())}
 forbidden.update(canonical(json.loads(f.read_text())['hands']) for f in (BASE/'results/h2h').rglob('game-*.json'));assert sid not in forbidden
 bidder=a.rotation;team=(bidder+(a.role=='defending'))%2;decl=6;remain=list(map(set,hands));plays=[];points=[0,0];leader=bidder;moves=[]
 worker=phone.Worker(phone.PHONE);model=HERE/'models'/f'{a.variant}.npz';pn=dict(np.load(model)) if a.variant!='teacher0' else None
 manifest=json.loads((phone.ROOT/'reference/production-phone/manifest.json').read_text());assert p.sha(phone.PHONE)==manifest['wasm_sha256']
 start=time.monotonic();deadline=start+100
 try:
  for ply in range(28):
   assert time.monotonic()<deadline,'incomplete game excluded'
   npl=ply%4;actor=(leader+npl)%4;tr=list(zip(plays[-2*npl::2],plays[-2*npl+1::2])) if npl else [];legal=p.rules.legal_tiles(remain[actor],tr,decl)
   candidate=actor%2==team;req=phone.make_request(hands,actor,bidder,plays,decl);s=p.rules.information_state(req);assert s['points']==points and s['leader']==leader and s['legal']==legal
   eligible=16<=ply<24 and len(legal)>1 and points[bidder%2]<30 and points[1-bidder%2]<13;override=candidate and eligible;ts=time.perf_counter()
   if override:
    if pn is not None:v=p.predict(pn,p.encode(req)[None])[0];choice=max(legal,key=lambda z:(v[z],-z));route=a.variant
    else:acts,y,_,_=p.label(req,128,7042104+ply);choice=int(acts[np.argmax(y.mean(0))]);route='T0-128'
    response=dict(choice=choice,legal=legal,points=points.copy(),leader=leader,route=route);timing=dict(host_ms=(time.perf_counter()-ts)*1000,interrupted=False)
   else:
    response,timing=worker.call(phone.profile(req,True),deadline);choice=response['choice'];assert response['legal']==legal and response['points']==points and response['leader']==leader;route='phone'
   assert choice in legal;moves.append(dict(ply=ply,actor=actor,candidate=candidate,eligible=eligible,override=override,route=route,request=req,response=response,timing=timing));remain[actor].remove(choice);plays.extend([actor,choice]);tr.append((actor,choice))
   if len(tr)==4:leader=p.rules.winner(tr,decl);points[leader%2]+=p.rules.trick_points(tr)
 finally:worker.close()
 audited,_,remaining,tail=p.rules.replay_record(hands,plays,decl,bidder);assert audited==points and sum(points)==42 and not tail and all(not x for x in remaining)
 report=dict(tier='exploratory',schema='tiny-late-phone-hybrid-v1',variant=a.variant,seed=a.seed,source_id=sid,rotation=a.rotation,role=a.role,decl=decl,bid=30,bidder=bidder,hands=hands,complete=True,points=points,made=points[bidder%2]>=30,moves=moves,game_seconds=time.monotonic()-start,game_limit_seconds=100,phone_wasm_sha256=p.sha(phone.PHONE),phone_profile='a0d9fa80 native-partner thinkDeeper=false straight42',candidate_model_sha256=p.sha(model) if pn else None,match_source_sha256=p.sha(__file__),kernel_sha256=p.sha(HERE/'kernel.c'),referee_sha256=p.sha(HERE.parent/'partnership/rules.py'),adapter_sha256=p.sha(phone.ROOT/'tools/phone_worker.mjs'),policy_seed=phone.PUBLIC_POLICY_SEED,hybrid='candidate partnership overrides ONLY live nonforced plies16..23; every other actor/turn uses same pinned phone')
 p.dump(out,report);print(json.dumps({k:report[k] for k in ['variant','seed','rotation','role','made','points','game_seconds']}))
def summarize(a):
 output={}
 for variant in VARIANTS:
  rows=[json.loads(f.read_text()) for f in sorted((HERE/'results/h2h'/variant).glob('game-*.json'))];assert len(rows)==16
  groups={(r['seed'],r['rotation'],r['role']):r for r in rows};assert len(groups)==16 and all(r['complete'] and len(r['moves'])==28 for r in rows)
  pairs=[];dealmeans=[]
  for seed in SEEDS:
   ds=[]
   for rotation in range(4):ds.append(int(groups[(seed,rotation,'declaring')]['made'])-int(groups[(seed,rotation,'defending')]['made']))
   pairs+=ds;dealmeans.append(sum(ds)/4)
  moves=[m for r in rows for m in r['moves']];over=[m for m in moves if m['override']];ph=[m for m in moves if not m['override']]
  output[variant]=dict(games=len(rows),source_deals=2,pair_wins=pairs.count(1),pair_losses=pairs.count(-1),pair_ties=pairs.count(0),mean_paired_make_difference=float(np.mean(pairs)),deal_mean_differences=dealmeans,complete_balanced_blocks=True,strength_claim=False,conservative95_interval=[-1,1],uncertainty='two independent source deals; rotations/role swaps correlated; smoke only',total_moves=len(moves),eligible_moves=sum(m['eligible'] for m in moves),overridden_moves=len(over),phone_calls=len(ph),phone_interruptions=sum(m['timing']['interrupted'] for m in ph),override_host_ms=dict(median=float(np.median([m['timing']['host_ms'] for m in over])) if over else None,p95=float(np.quantile([m['timing']['host_ms'] for m in over],.95)) if over else None),game_seconds_sum=sum(r['game_seconds'] for r in rows),identity=dict(phone=rows[0]['phone_wasm_sha256'],model=rows[0]['candidate_model_sha256']),latency_qualification='routing counts endogenous to variant trajectories; phone startup/cost included; no equal-cost speed ratio')
 p.dump(HERE/'results/h2h/summary.json',output);print(json.dumps(output))
parser=argparse.ArgumentParser();sp=parser.add_subparsers(dest='command',required=True);r=sp.add_parser('run');r.add_argument('--variant',choices=VARIANTS,required=True);r.add_argument('--seed',type=int,choices=SEEDS,required=True);r.add_argument('--rotation',type=int,choices=range(4),required=True);r.add_argument('--role',choices=['declaring','defending'],required=True);sp.add_parser('summarize');a=parser.parse_args();globals()[a.command](a)
