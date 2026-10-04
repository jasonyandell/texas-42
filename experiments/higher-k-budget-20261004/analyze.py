#!/usr/bin/env python3
"""Descriptive cost/counter/action witnesses; no new strength/complexity theorem."""
import collections,json,math,statistics
from pathlib import Path
import experiment as e
HERE=e.HERE
def receipt_for(panel,seed):
 labels=([f'root-log-{seed}',f'roots-log-{seed}']if panel=='fixed-roots'else [f'earlier-log-{seed}']if panel=='earlier-roots'else [f'unconstrained-log-{seed}'])
 paths=[e.RESULTS/label/'run.json'for label in labels if(e.RESULTS/label/'run.json').exists()];assert len(paths)==1
 return e.load(paths[0])
out=dict(tier='exploratory finite panel; all source-deal clusters retained',strength_claim=False,complexity_lower_bound_claim=False)
for panel in ['fixed-roots','earlier-roots','unconstrained']:
 sources=[e.load(p)for p in (e.RESULTS/panel).glob('deal-*.json')];rows=[r for f in sources for r in f['records']];cells=[]
 for plies in sorted({r['plies']for r in rows}):
  for budget in sorted({r['call']['budget_ms']for r in rows}):
   for k in range(1,6):
    allq=[r for r in rows if(r['plies'],r['call']['budget_ms'],r['call']['k'])==(plies,budget,k)]
    qq=[r for r in allq if not r['response'].get('ineligible')];complete=[r for r in qq if r['response']['completed']]
    cells.append(dict(plies=plies,budget_ms=budget,k=k,planned=len(allq),eligible=len(qq),eligible_source_deals=len({r['seed']for r in qq}),completed=len(complete),refused=sum('refusal'in r['response']for r in qq),complete_host_ms=e.stats([r['timing']['host_ms']for r in complete]),eligible_host_ms=e.stats([r['timing']['host_ms']for r in qq]),complete_nodes=e.stats([r['response']['nodes']for r in complete]),complete_pi_computations=e.stats([sum(r['response']['pi_calls_by_level'])for r in complete]),complete_pi_by_level=[sum(r['response']['pi_calls_by_level'][lev]for r in complete)for lev in range(6)],complete_cache_entries=e.stats([r['response']['policy_cache_entries']for r in complete]),sample_us=e.stats([r['response'].get('outer_sample_us',0)for r in qq]),outer_attempts=e.stats([r['response'].get('outer_attempts',0)for r in qq]),wrapper_overrun_ms=e.stats([r['timing']['wrapper_overrun_ms']for r in qq]),host_overrun_ms=e.stats([r['timing']['host_overrun_ms']for r in qq])))
 out[panel]=dict(cells=cells,capped_batch_wall_s=sum(receipt_for(panel,f['records'][0]['seed'])['elapsed_seconds']for f in sources),worker_cpu_s=sum(u['cpu_s']for f in sources for u in f['usage']),max_pid_rss_bytes=max(u['peak_rss_bytes']for f in sources for u in f['usage']))
 if panel!='unconstrained':
  indexes={(r['seed'],r['rotation'],r['call']['budget_ms'],r['call']['k']):r for r in rows};diffs=[]
  for b in [20,200]:
   for k in range(2,6):
    base=[r for r in rows if r['call']['k']==1 and r['call']['budget_ms']==b and not r['response'].get('ineligible')];witness=[]
    for r in base:
     other=indexes[r['seed'],r['rotation'],b,k]
     if r['response']['choice']!=other['response']['choice']:witness.append(dict(seed=r['seed'],rotation=r['rotation'],k1_choice=r['response']['choice'],higher_choice=other['response']['choice'],higher_completed=other['response']['completed']))
    diffs.append(dict(k=k,budget_ms=b,eligible=len(base),different_actions=len(witness),witnesses=witness))
  out[panel]['action_differences']=diffs
parallel=[e.load(p)for p in (e.RESULTS/'parallel').glob('workers-*.json')];summary=[]
clock_keys={'status_us','outer_sample_us','solve_us','elapsed_us'}
def semantic(v):return{k:q for k,q in v.items()if k not in clock_keys}
reference=sorted(parallel,key=lambda x:(x['workers'],x['rep']))[0]['records']
assert all([q['call']for q in f['records']]==[q['call']for q in reference]and[semantic(q['response'])for q in f['records']]==[semantic(q['response'])for q in reference]for f in parallel)
for workers in [1,4]:
 ff=[f for f in parallel if f['workers']==workers];walls=[e.load(e.RESULTS/f"parallel-log-{workers}-{f['rep']}"/'run.json')['elapsed_seconds']*1000 for f in ff]
 summary.append(dict(workers=workers,repetitions=len(ff),queries_per_batch=36,eligible_per_batch=sum(not q['response'].get('ineligible')for q in ff[0]['records']),complete_per_batch=sum(q['response']['completed']for q in ff[0]['records']),refusals_per_batch=sum('refusal'in q['response']for q in ff[0]['records']),batch_ms=e.stats([f['batch_ms']for f in ff]),complete_cap_wall_ms=e.stats(walls),cpu_ms=e.stats([f['cpu_ms']for f in ff]),sum_pid_peak_rss_bytes=e.stats([f['sum_pid_peak_rss_bytes']for f in ff])))
out['parallel']=dict(cells=summary,complete_cap_wall_speed_ratio=summary[0]['complete_cap_wall_ms']['median']/summary[1]['complete_cap_wall_ms']['median'],scope='Frozen36complete-query local batch from9source deals, same30s/query. Median6balanced repetitions. Whole capped walls charge Python import/referee/input construction/worker startup/sampling/cache/JSON/hash/output/reaping. CPU does not improve; sumPID peaks is an upper envelope, not measured simultaneous aggregate memory. No single-turn/device/browser speed claim.',full_nonclock_response_parity=True)
summary=e.load(e.RESULTS/'summary.json');witnesses=[]
for cell in summary['matches']:
 for cluster in cell['clusters']:
  if cluster['mean']:
   witnesses.append(dict(reference=cell['reference'],candidate=cell['candidate'],budget_ms=cell['budget_ms'],**cluster))
out['paired_outcome_witnesses']=witnesses
out['games_resources']=dict(capped_batch_wall_s=sum(e.load(p)['elapsed_seconds']for p in (e.RESULTS).glob('game*-log-*/run.json')),suffix_games=3024,suffix_moves=36288)
# Source-file raw games explicitly retained; all aggregate intervals are pointwise.
e.save(e.RESULTS/'analysis.json',out)
print(json.dumps(dict(parallel=out['parallel'],earlier=[{k:r[k]for k in ['k','budget_ms','eligible','completed','refused','complete_host_ms']}for r in out['earlier-roots']['cells']]),indent=2))
