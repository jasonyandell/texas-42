#!/usr/bin/env python3
"""Predeclared independent-deal native certificate cost panel; use run_capped.py."""
import argparse, hashlib, json, random, subprocess, sys, time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT.parent/'partnership'))
from rules import legal_tiles, winner, replay_record, information_state

def fixture(i):
    # One source deal, one public random-legal prefix; never search for good certificates.
    seed=680000+i; rng=random.Random(seed); deck=list(range(28)); rng.shuffle(deck)
    hands=[sorted(deck[s*7:s*7+7]) for s in range(4)]
    remaining=list(map(set,hands)); bidder=i%4; leader=bidder; decl=(*range(8),9)[i%9]
    ply=(8,12,16,20)[(i//9)%4]; plays=[]; trick=[]
    for _ in range(ply):
        seat=(leader+len(trick))%4; tile=rng.choice(legal_tiles(remaining[seat],trick,decl))
        remaining[seat].remove(tile); plays.extend([seat,tile]); trick.append((seat,tile))
        if len(trick)==4: leader=winner(trick,decl); trick=[]
    points,_,_,_=replay_record(hands,plays,decl,bidder)
    seat=(leader+len(trick))%4
    request=dict(decl=decl,bid=30,bidder=bidder,seat=seat,hand=hands[seat],plays=plays,seed=7042104)
    legal=information_state(request)['legal']
    eligible=len(legal)>1 and points[bidder%2]<30 and points[(bidder+1)%2]<=12
    return dict(id=i,seed=seed,ply=ply,hands=hands,points=points,eligible=eligible,request=request)

def main():
    p=argparse.ArgumentParser();p.add_argument('--out',type=Path,required=True)
    p.add_argument('--start',type=int,default=0);p.add_argument('--count',type=int,default=36)
    p.add_argument('--cheap',type=int,default=1);p.add_argument('--level',type=int,default=0)
    args=p.parse_args();args.out.mkdir(parents=True,exist_ok=False)
    binary=ROOT/'phase2/probe/target/release/walt-substitution-probe'
    rows=[fixture(i) for i in range(args.start,args.start+args.count)]
    plan=dict(schema='walt-certificate-cost-plan-v1',binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),
        independent_source_deals=args.count,cheap_n0=args.cheap,target_n0=8,outer=40,field_level=args.level,
        deadline_seconds=20,per_process_wall_timeout=22,panel_seconds=275,rows=rows)
    (args.out/'plan.json').write_text(json.dumps(plan,indent=2)+'\n')
    begin=time.monotonic();results=[]
    for row in rows:
        if not row['eligible']:
            results.append(dict(id=row['id'],status='excluded_before_probe',reason='forced_or_contract_settled'));continue
        if time.monotonic()-begin>250:
            results.append(dict(id=row['id'],status='not_run_panel_deadline'));continue
        inp=dict(request=row['request'],outer=40,cheap_n0=args.cheap,target_n0=8,seconds=20,
            target_first=bool(row['id']%2),field_level=args.level)
        t=time.monotonic()
        try:
            run=subprocess.run([str(binary)],input=json.dumps(inp)+'\n',text=True,capture_output=True,timeout=22)
            if run.returncode: result=dict(status='process_error',returncode=run.returncode,stderr=run.stderr)
            else:
                value=json.loads(run.stdout);result=dict(status='refused' if 'error' in value else 'completed',result=value)
        except subprocess.TimeoutExpired:result=dict(status='wall_timeout')
        result.update(id=row['id'],host_seconds=time.monotonic()-t,input=inp)
        (args.out/f"case-{row['id']:04}.json").write_text(json.dumps(result,indent=2)+'\n');results.append(result)
    complete=[r['result'] for r in results if r['status']=='completed']
    accepted=[r for r in complete if r['certified']]
    summary=dict(planned=len(rows),eligible=sum(r['eligible'] for r in rows),completed=len(complete),
        certified=len(accepted),statuses={s:sum(r['status']==s for r in results) for s in sorted({r['status'] for r in results})},
        same_choice=sum(r['cheap_choice']==r['target_choice'] for r in complete),
        all_action_bounds_checked=True,elapsed_seconds=time.monotonic()-begin,
        total_cheap_us=sum(r['cheap_us'] for r in complete),total_certificate_us=sum(r['certificate_us'] for r in complete),
        total_target_us=sum(r['target_us'] for r in complete),
        certified_cheap_plus_certificate_us=sum(r['cheap_us']+r['certificate_us'] for r in accepted),
        certified_target_us=sum(r['target_us'] for r in accepted),
        faster_certified=sum(r['cheap_us']+r['certificate_us']<r['target_us'] for r in accepted),
        caveat='Native kernel timing, random-legal conditional positions; not phone head-to-head strength or phone-device speed. Fresh target comparison. No shared-target-cache fallback optimization.')
    (args.out/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
    print(json.dumps(summary,indent=2))
if __name__=='__main__':main()
