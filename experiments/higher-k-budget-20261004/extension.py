#!/usr/bin/env python3
"""Explicit adaptive follow-up: all frozen deals, one trick earlier; local only."""
import argparse,collections,json,math,statistics,subprocess,sys,time
from pathlib import Path
sys.dont_write_bytecode=True
import experiment as e
HERE=e.HERE;PLAN=HERE/'earlier-plan.json'
def initialize():
 e.save(PLAN,dict(schema='adaptive-earlier-horizon-plan-v1',reason='First frozen16-ply cost panel has96eligible roots from24/54source deals; k5 refused16 at20ms and none200ms. Follow-up samples the12-ply prefix already generated for EVERY same54source deal/4rotations, including ineligible roots. Outcome-independent inclusion within this adaptive domain choice.',prior_plan_sha256=e.sha(e.PLAN),fixture_sha256=e.sha(e.FIXTURES),root_summary_observed='216root positions;120ineligible;96eligible; k1..4complete96/96; k5at20complete80/refuse16; k5at200complete96.',seeds=e.load(e.PLAN)['seeds'],prefix_plies=12,k=[1,2,3,4,5],budgets_ms=[20,200],new_independent_deals=0,scope='Cost/completion/action changes only; no12-ply suffix strength panel. Distinct from original frozen16-ply panel.',policy=e.load(e.PLAN)['rung'],inner_budgets=[4,2,2,2,2,2],outer_worlds=40,selection='Fixed ascending-tile ties; Voidless inner belief',tier='exploratory adaptive follow-up'))
 print(e.sha(PLAN))
def run(seed):
 f=next(f for f in e.load(e.FIXTURES)if f['seed']==seed);w=e.Native();rows=[];start=time.monotonic()
 partial=e.RESULTS/'earlier-roots'/f'partial-{seed}.jsonl';partial.parent.mkdir(parents=True,exist_ok=True);assert not partial.exists()
 try:
  for rotation in range(4):
   hands,plays=e.rotated(f,rotation,12);req=e.req_for(hands,plays,f['decl'],rotation)
   for budget in [20,200]:
    kk=list(range(1,6));offset=(seed+rotation+(budget==200))%5;kk=kk[offset:]+kk[:offset]
    for k in kk:
     call=e.ladder(req,k,budget);v,t=w.call(call)
     r=dict(seed=seed,decl=f['decl'],rotation=rotation,plies=12,call=call,response=v,timing=t);rows.append(r)
     with partial.open('a')as stream:stream.write(json.dumps(r,separators=(',',':'))+'\n')
 finally:w.close()
 e.save(e.RESULTS/'earlier-roots'/f'deal-{seed}.json',dict(records=rows,usage=w.usage,elapsed_s=time.monotonic()-start,identity=e.identity(),earlier_plan_sha256=e.sha(PLAN),extension_sha256=e.sha(__file__)))
 print(json.dumps(dict(seed=seed,calls=len(rows),refusals=sum('refusal'in r['response']for r in rows),seconds=time.monotonic()-start)))
def campaign():
 for seed in e.load(PLAN)['seeds']:
  subprocess.run([sys.executable,str(e.ROOT/'experiments/astra-sol-20261004/tools/run_capped.py'),'--seconds','295','--output-dir',str(e.RESULTS/f'earlier-log-{seed}'),'--',sys.executable,__file__,'run','--seed',str(seed)],cwd=e.ROOT,check=True)
 print('Completed54separately capped earlier-root batches')
def summary():
 source=[e.load(p)for p in (e.RESULTS/'earlier-roots').glob('deal-*.json')];assert len(source)==54
 rows=[r for f in source for r in f['records']];cells=[]
 for b in [20,200]:
  for k in range(1,6):
   qs=[r for r in rows if(r['call']['k'],r['call']['budget_ms'])==(k,b)];assert len(qs)==216
   eligible=[r for r in qs if not r['response'].get('ineligible')]
   cells.append(dict(k=k,budget_ms=b,planned=216,eligible=len(eligible),completed=sum(r['response']['completed']for r in eligible),refused=sum('refusal'in r['response']for r in eligible),host_ms=e.stats([r['timing']['host_ms']for r in qs]),eligible_host_ms=e.stats([r['timing']['host_ms']for r in eligible]),sample_us=e.stats([r['response'].get('outer_sample_us',0)for r in eligible]),nodes=e.stats([r['response'].get('nodes',0)for r in eligible]),pi_computations_by_level=[sum(r['response'].get('pi_calls_by_level',[0]*6)[lev]for r in eligible)for lev in range(6)],inner_worlds_by_level=[sum(r['response'].get('inner_worlds_by_level',[0]*6)[lev]for r in eligible)for lev in range(6)],policy_cache_entries=e.stats([r['response'].get('policy_cache_entries',0)for r in eligible]),wrapper_overruns=e.stats([r['timing']['wrapper_overrun_ms']for r in qs])))
 e.save(e.RESULTS/'earlier-summary.json',dict(cells=cells,source_deals=54,new_independent_deals=0,root_positions=216,earlier_plan_sha256=e.sha(PLAN),worker_cpu_s=sum(u['cpu_s']for f in source for u in f['usage']),max_pid_rss_bytes=max(u['peak_rss_bytes']for f in source for u in f['usage']),batch_wall_s=sum(f['elapsed_s']for f in source)))
 print(json.dumps(cells))
if __name__=='__main__':
 p=argparse.ArgumentParser();p.add_argument('cmd',choices=['initialize','run','campaign','summary']);p.add_argument('--seed',type=int);a=p.parse_args()
 if a.cmd=='initialize':initialize()
 elif a.cmd=='run':run(a.seed)
 elif a.cmd=='campaign':campaign()
 else:summary()
