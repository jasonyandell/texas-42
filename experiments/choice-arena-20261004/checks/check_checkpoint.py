# Adapted with attribution from preceding demand-bounds independent checker; fresh execution here.
#!/usr/bin/env python3
"""Fresh receipt arithmetic and counter audit; no implementation changes."""
import gzip,json,statistics,math
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
BASE=ROOT/'experiments/prefix-state-20261004/results'
def read(p):
    if p.exists(): return json.loads(p.read_text())
    with gzip.open(str(p)+'.gz','rt') as f:return json.load(f)
def eq(a,b):assert math.isclose(a,b,rel_tol=1e-12,abs_tol=1e-12),(a,b)
panels=[];vectors=0
for name in ['final-one-v2','final-four-v2','holdout-four','final-policy0-four','final-hybrid0-four','rung0-one','rung1-one','rung2-one']:
    summary=read(BASE/name/'summary.json');records=read(BASE/name/'records.json')
    for n in summary['arguments']['counts']:
        inputs=read(BASE/name/f'inputs-{n}.json')
        assert [x['seed'] for x in inputs]==summary['selected']
        assert all(len(x['worlds'])==n for x in inputs)
    for record in records:
        runs=record['runs'];expected=runs['recursive']['result']['answers']
        for variant in ['factored','factored-hash','predecessor']:
            assert runs[variant]['result']['answers']==expected;vectors+=len(expected)
        for key in ['row_edges','public_coordinates','unique_actor_misses_by_level','inner_worlds_by_level']:
            assert all(runs[v]['result']['cold_stats'][key]==runs['factored']['result']['cold_stats'][key] for v in ['factored-hash','predecessor'])
    for p in summary['panels']:
        runs=[r['runs'] for r in records if r['worlds']==p['worlds']]
        for variant,claims in p['variants'].items():
            vv=[r[variant] for r in runs]
            cold=statistics.median(x['result']['reference_total_us'] if variant=='recursive' else x['result']['frontier_cold_us'] for x in vv)/1e6
            standalone=statistics.median(sum(x[k] for k in ['serialization_s','host_s','output_parse_s','harness_overhead_s']) for x in vv)
            eq(cold,claims['cold_service_s']);eq(standalone,claims['standalone_s']);eq(standalone+p['generation_actual_n_s'],claims['total_with_generation_s'])
            for key in ['child_cpu_s','process_peak_rss_bytes','harness_overhead_s']:eq(statistics.median(x[key] for x in vv),claims[key])
        f=p['variants']['factored'];r=p['variants']['recursive']
        eq(r['cold_service_s']/f['cold_service_s'],p['recursive_over_factored'])
        eq(r['standalone_s']/f['standalone_s'],p['standalone_recursive_over_factored'])
        panels.append({'name':name,'worlds':p['worlds'],'native_over_linked':p['recursive_over_factored'],'standalone_native_over_linked':p['standalone_recursive_over_factored']})
matrix=read(BASE/'demand-matrix/records.json');matrix_checks=0;rows=[]
for r in matrix:
    expected=r['reference']['result']['answers'];details={}
    for variant,run in r['runs'].items():
        x=run['result'];stats=x.get('cold_stats',x.get('worker_stats',[{}])[0]);complete='answers' in x
        if complete:assert x['answers']==expected;matrix_checks+=len(expected)
        else:assert (r['worlds'],r['field'],variant)==(40,2,'full');assert x['error']=='actor query cap';assert stats['generated_actor_queries']==49534;assert stats['completed_actor_queries']==49202
        if variant=='full':
            assert stats['inner_worlds_by_level']==[a*b for a,b in zip(stats['unique_actor_misses_by_level'],[4,2,2,2])]
        details[variant]={'complete':complete,'unique_misses':stats['unique_actor_misses_by_level'],'actor_demands':stats['actor_demands_by_level'],'inner_worlds':stats['inner_worlds_by_level'],'native_misses':stats['native_core_pi_misses_by_level'],'native_reported_worlds':stats['native_core_reported_inner_worlds_by_level'],'generated':stats['generated_actor_queries'],'completed':stats['completed_actor_queries']}
    rows.append({'worlds':r['worlds'],'field':r['field'],'roots':r['roots'],'details':details})
assert matrix_checks==102
print(json.dumps({'retained_timing_vector_comparisons':vectors,'matrix_vector_comparisons':matrix_checks,'timing_panels':panels,'matrix':rows,'interpretation':'Finite retained receipts. Full samples count eager sampled queries; hybrid samples are native-reported. Refusal excludes root answers; no minimal-demand, phone, strength, or asymptotic claim.'},indent=2))
