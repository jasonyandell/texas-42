#!/usr/bin/env python3
import hashlib,json,math,statistics
from pathlib import Path
HERE=Path(__file__).resolve().parent;BASE=HERE.parent/'results'
outputs=[]
for directory in sorted(BASE.glob('final-*')):
    if not (directory/'summary.json').exists():continue
    summary=json.loads((directory/'summary.json').read_text());records=json.loads((directory/'records.json').read_text());plan=json.loads((directory/'plan.json').read_text())
    assert summary['selected']==len(plan['selected'])
    for record in records:
        assert len(record['result'].get('answers',[])) in [0,record['roots']]
        if 'answers' in record['result']:
            assert record['result']['answers']==record['baseline']['result']['answers']
        else:assert 'error' in record['result']
    panels=[]
    for panel in summary['panels']:
        matching=[r for r in records if r['worlds']==panel['worlds']]
        complete=[r for r in matching if 'answers' in r['result']]
        assert panel['completed']==len(complete) and panel['refused']==len(matching)-len(complete)
        detail={k:panel[k] for k in ['worlds','roots','completed','refused']}
        if complete:
            front=statistics.median(r['result']['frontier_cold_us'] for r in complete)/1e6
            ref=statistics.median(r['baseline']['result']['reference_total_us'] for r in complete)/1e6
            fhost=statistics.median(r['serialization_s']+r['host_s']+r['output_parse_s'] for r in complete)
            bhost=statistics.median(r['baseline']['serialization_s']+r['baseline']['host_s']+r['baseline']['output_parse_s'] for r in complete)
            generation=panel['actual_world_generation_s']
            for expected,actual in [(front,panel['frontier_median_s']),(ref,panel['reference_total_median_s']),(ref/front,panel['reference_total_over_frontier']),(bhost/fhost,panel['standalone_ratio']),((generation+bhost)/(generation+fhost),panel['end_to_end_actual_generation_ratio'])]:assert math.isclose(expected,actual,rel_tol=1e-12)
            # Child CPU deltas are scoped per subprocess, unlike cumulative RSS.
            for r in complete:
                assert r['child_user_cpu_s']>=0 and r['child_system_cpu_s']>=0
                assert r['baseline']['child_user_cpu_s']>=0 and r['baseline']['child_system_cpu_s']>=0
            detail.update(frontier_cold_s=front,reference_wall_s=ref,frontier_over_reference_wall=front/ref,reference_over_frontier_standalone=bhost/fhost,generation_charged_ratio=(generation+bhost)/(generation+fhost),generated_queries=panel['stats']['generated_actor_queries'])
        else:detail['errors']=sorted({r['result']['error'] for r in matching})
        panels.append(detail)
    outputs.append({'directory':directory.name,'arguments':{k:plan['arguments'].get(k) for k in ['mode','field','workers','parallel_reference','batch_reference','standalone','native_choices']},'panels':panels,'summary_sha256':hashlib.sha256((directory/'summary.json').read_bytes()).hexdigest()})
result={'final_panels_checked':len(outputs),'all_completed_standalone_vectors_equal':True,'timing_formulas_reproduced':True,'results':outputs,'limits':['No positive speed claim supported by equal-worker reference wall controls.','reference_solve_us sum and common-max-generation ratios are legacy diagnostics, not batch wall throughput.','CHILDREN ru_maxrss is cumulative high-water so far across prior frontend and baseline processes.','Summed worker peak arrays are separate maxima; unique misses are per-worker, not globally deduplicated.','Full-frontier refusals are not zero-valued answers; all-complete-vectors label means completed runs only.']}
(HERE/'FINAL_RESULTS_AUDIT.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result,indent=2))
