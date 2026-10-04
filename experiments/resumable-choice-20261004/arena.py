#!/usr/bin/env python3
"""One complete play-only hybrid versus pinned-phone game per capped run.

Adapted from astra-sol tools/arena.py; original source unchanged. Fixed bid30,
four rotations, both partnership assignments make a complete eight-game block.
"""
import argparse,hashlib,json,random,sys,time
from pathlib import Path
sys.dont_write_bytecode=True
from player import Player,phone
HERE=Path(__file__).resolve().parent

def run(a):
 if a.out.exists():raise ValueError('preserve existing game')
 a.out.parent.mkdir(parents=True,exist_ok=True)
 assert phone.sha(phone.PHONE)==json.loads((phone.ROOT/'reference/production-phone/manifest.json').read_text())['wasm_sha256']
 tiles=list(range(28));random.Random(a.seed).shuffle(tiles)
 base=[sorted(tiles[7*s:7*s+7]) for s in range(4)]
 hands=[base[(s-a.rotation)%4] for s in range(4)];bidder=a.rotation
 team=(bidder+(a.role=='defending'))%2
 workers=[phone.Worker(phone.PHONE),Player(a.variant)]
 remain=list(map(set,hands));plays=[];moves=[];points=[0,0];leader=bidder
 started=time.monotonic();deadline=started+240
 try:
  for _ in range(7):
   trick=[]
   for _ in range(4):
    if time.monotonic()>=deadline:raise TimeoutError('game cap; incomplete block excluded')
    seat=(leader+len(trick))%4;candidate=seat%2==team
    req=phone.make_request(hands,seat,bidder,plays,a.decl)
    call=phone.profile(req,True)
    response,timing=workers[int(candidate)].call(call,deadline)
    legal=phone.legal_tiles(remain[seat],trick,a.decl)
    assert response['choice'] in legal and response['legal']==legal
    assert response['points']==points and response['leader']==leader
    moves.append(dict(call=call,response=response,timing=timing,candidate=candidate))
    tile=response['choice'];remain[seat].remove(tile);plays.extend([seat,tile]);trick.append((seat,tile))
   leader=phone.winner(trick,a.decl);points[leader%2]+=phone.trick_points(trick)
 finally:
  for w in workers:w.close()
 audited,_,remaining,tail=phone.replay_record(hands,plays,a.decl,bidder)
 assert audited==points and sum(points)==42 and not tail and all(not h for h in remaining)
 report=dict(schema='resumable-complete-hybrid-game-v1',seed=a.seed,rotation=a.rotation,role=a.role,decl=a.decl,bid=30,bidder=bidder,hands=hands,points=points,made=points[bidder%2]>=30,moves=moves,complete=True,game_seconds=time.monotonic()-started,
  candidate_policy='pinned-phone early/forced/settled/refusal; late uniform40 / voidless [4,2,2,2] Field::Level(2), ascending ties',
  variant=a.variant,policy_seed=phone.PUBLIC_POLICY_SEED,
  binary_sha256=phone.sha(HERE/'integration/target/release/late-choice-player'),phone_wasm_sha256=phone.sha(phone.PHONE),
  source_sha256={p.name:phone.sha(p) for p in [Path(__file__),HERE/'player.py',HERE/'integration/src/main.rs',phone.ROOT.parent/'partnership/rules.py']})
 a.out.write_text(json.dumps(report,indent=2)+'\n')
 print(json.dumps({k:report[k] for k in ['seed','rotation','role','points','made','game_seconds']}))
def summarize(a):
 rows=[json.loads(p.read_text()) for p in sorted(a.directory.glob('game-*.json'))]
 assert rows and all(r['complete'] and len(r['moves'])==28 for r in rows)
 identity={(r['candidate_policy'],r['variant'],r['policy_seed'],r['binary_sha256'],r['phone_wasm_sha256'],json.dumps(r['source_sha256'],sort_keys=True)) for r in rows};assert len(identity)==1
 groups={(r['seed'],r['rotation'],r['role']):r for r in rows};assert len(groups)==len(rows)
 pairs=[];missing=[]
 for seed in sorted({r['seed'] for r in rows}):
  for rotation in range(4):
   x=groups.get((seed,rotation,'declaring'));y=groups.get((seed,rotation,'defending'))
   if x and y:pairs.append(int(x['made'])-int(y['made']))
   else:missing.append([seed,rotation])
 candidate=[m for r in rows for m in r['moves'] if m['candidate']]
 baseline=[m for r in rows for m in r['moves'] if not m['candidate']]
 late=[m for m in candidate if m['response']['route']=='late-l3-resumable-40-4-2-2']
 out=dict(games=len(rows),moves=sum(len(r['moves']) for r in rows),source_deals=len({r['seed'] for r in rows}),missing_pairs=missing,pair_wins=pairs.count(1),pair_losses=pairs.count(-1),pair_ties=pairs.count(0),late_completed=len(late),late_refusals=sum(bool(m['timing'].get('late_refusal')) for m in candidate),strength_claim=False,
  uncertainty='One source-deal smoke block; four rotations are correlated. Conservative bounded independent-deal 95% interval is [-1,1]; no strength inference.',
  candidate_host_ms=sum(m['timing']['host_ms'] for m in candidate),phone_host_ms=sum(m['timing']['host_ms'] for m in baseline),late_host_ms=sum(m['timing']['host_ms'] for m in late),identity=rows[0]['candidate_policy'],binary_sha256=rows[0]['binary_sha256'],phone_wasm_sha256=rows[0]['phone_wasm_sha256'])
 if not missing:out['mean_paired_make_difference']=sum(pairs)/len(pairs)
 (a.directory/'summary.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out))
if __name__=='__main__':
 p=argparse.ArgumentParser();sp=p.add_subparsers(dest='cmd',required=True)
 r=sp.add_parser('run');r.add_argument('--seed',type=int,required=True);r.add_argument('--rotation',type=int,choices=range(4),required=True);r.add_argument('--role',choices=['declaring','defending'],required=True);r.add_argument('--decl',type=int,default=6);r.add_argument('--variant',default='resumable');r.add_argument('--out',type=Path,required=True)
 s=sp.add_parser('summarize');s.add_argument('directory',type=Path)
 a=p.parse_args();run(a) if a.cmd=='run' else summarize(a)
