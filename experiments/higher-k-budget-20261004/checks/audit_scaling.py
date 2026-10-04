#!/usr/bin/env python3
"""Independent saved unconstrained/whole-query parallel arithmetic; no timing."""
import json,statistics,sys
sys.dont_write_bytecode=True
from audit_data import HERE,read,identity,rotated,request,response,close,stats
def semantic(value):
    return {key:item for key,item in value.items()if key not in ['elapsed_us','status_us','outer_sample_us','solve_us']}
def main():
    plan=read(HERE/'plan.json');fixtures=read(HERE/'fixtures.json');lookup={f['seed']:f for f in fixtures}
    files=[read(p)for p in sorted((HERE/'results/unconstrained').glob('deal-*.json'))]
    assert len(files)==9
    records=[];seen=set()
    for f in files:
        identity(f['identity']);rows=f['records'];seed=rows[0]['seed'];assert seed in plan['seeds'][:9]and seed not in seen;seen.add(seed)
        assert len(rows)==25;keys=set();fixture=lookup[seed]
        for q in rows:
            plies=q['plies'];r=q['rotation'];k=q['call']['k'];key=(plies,r,k);assert key not in keys;keys.add(key)
            assert q['seed']==seed and q['call']==dict(action='ladder',request=request(*rotated(fixture,r,plies),fixture['decl'],r),k=k,budget_ms=30000,validate=False)
            response(q['call']['request'],q['response'],30000)
        assert keys=={(16,r,k)for r in range(4)for k in range(1,6)}|{(12,0,k)for k in range(1,6)}
        records+=rows
    panels=[]
    for plies in[12,16]:
        for k in range(1,6):
            rows=[q for q in records if q['plies']==plies and q['call']['k']==k];eligible=[q for q in rows if not q['response'].get('ineligible')];complete=[q for q in eligible if q['response']['completed']]
            panels.append(dict(plies=plies,k=k,planned=len(rows),eligible=len(eligible),complete=len(complete),censored=sum('refusal'in q['response']for q in eligible),completed_host_ms=stats([q['timing']['host_ms']for q in complete]),completed_nodes=stats([q['response']['nodes']for q in complete]),completed_pi_computations=stats([sum(q['response']['pi_calls_by_level'])for q in complete]),completed_inner_samples=stats([sum(q['response']['inner_worlds_by_level'])for q in complete])))
    batches=[read(p)for p in sorted((HERE/'results/parallel').glob('workers-*-rep-*.json'))]
    assert len(batches)==12;index={(b['workers'],b['rep']):b for b in batches};assert set(index)=={(w,r)for w in[1,4]for r in range(6)}
    expected=[]
    for fixture in fixtures[:9]:
        for r in range(4):expected.append(dict(action='ladder',request=request(*rotated(fixture,r,12),fixture['decl'],r),k=5,budget_ms=30000,validate=False))
    comparisons=0
    for b in batches:
        identity(b['identity']);assert len(b['records'])==36 and len(b['usage'])==b['workers']
        close(b['cpu_ms'],1000*sum(u['cpu_s']for u in b['usage']));assert b['sum_pid_peak_rss_bytes']==sum(u['peak_rss_bytes']for u in b['usage'])
        for i,q in enumerate(b['records']):
            assert q['index']==i and q['call']==expected[i];response(q['call']['request'],q['response'],30000)
            assert not q['response'].get('refusal'), 'Parallel timing censored: full-vector parity unavailable'
            assert semantic(q['response'])==semantic(index[1,0]['records'][i]['response']);comparisons+=1
    costs={str(w):dict(repetitions=6,batch_ms=stats([index[w,r]['batch_ms']for r in range(6)]),cpu_ms=stats([index[w,r]['cpu_ms']for r in range(6)]),sum_pid_peak_rss_bytes=stats([index[w,r]['sum_pid_peak_rss_bytes']for r in range(6)]))for w in[1,4]}
    ratio=statistics.median(index[1,r]['batch_ms']for r in range(6))/statistics.median(index[4,r]['batch_ms']for r in range(6))
    capwalls={str(w):stats([1000*read(HERE/'results'/f'parallel-log-{w}-{r}'/'run.json')['elapsed_seconds']for r in range(6)])for w in[1,4]}
    complete_ratio=capwalls['1']['median']/capwalls['4']['median']
    vector_comparisons=sum(q['response']['completed']for b in batches for q in b['records'])
    ineligible_responses=sum(bool(q['response'].get('ineligible'))for b in batches for q in b['records'])
    print(json.dumps(dict(unconstrained_calls=225,unconstrained_cost_panels=panels,parallel_prespecified_queries=36,parallel_eligible_queries_per_batch=vector_comparisons//12,parallel_repetitions_per_worker_count=6,semantic_response_comparisons=comparisons,completed_full_vector_comparisons=vector_comparisons,lawful_ineligible_response_comparisons=ineligible_responses,parallel_costs=costs,complete_capped_walls_ms=capwalls,complete_capped_wall_median_speed_ratio=complete_ratio,internal_batch_median_speed_ratio=ratio,scope='Finite unchanged whole-query policy batches. Complete cap wall charges Python driver/startup/hash/receipt; internal batch secondary excludes those. Sum PID maxima is an upper envelope rather than simultaneous RSS. Shared first9 deals are not additional independent strength observations. Censored cells excluded from unconstrained-complete cost.'),indent=2))
if __name__=='__main__':main()
