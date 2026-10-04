#!/usr/bin/env python3
import json,subprocess
from pathlib import Path
HERE=Path(__file__).resolve().parent
rows=json.loads((HERE/'independent-panel.json').read_text())['rows']
cases=[]
for counted in [False,True]:
    for mode,field,hybrid in [('native',0,False)]+[('policy',k,h) for k in (0,1) for h in (False,True)]:
        subset=rows if mode=='native' else rows[:12]
        for workers in [1,4]:
            inp={'rows':subset,'mode':mode,'field_level':field,'native_choices':hybrid,'workers':workers,'counted':counted,'budgets':[4,2,2,2],'reference':True,'parallel_reference':True,'reference_after':workers==4,'warm':True,'caps':{'rows':500000,'work':5000000,'queries':50000,'cache_entries':50000,'seconds':20}}
            cases.append((counted,mode,field,hybrid,workers,inp))
run=subprocess.run([str(HERE/'runner-target/release/native-frontier')],input=''.join(json.dumps(x[-1])+'\n' for x in cases),capture_output=True,text=True,timeout=60)
assert run.returncode==0,run.stderr
answers=[json.loads(line) for line in run.stdout.splitlines()]
assert len(answers)==len(cases)
report=[];previous={}
for c,out in zip(cases,answers):
    counted,mode,field,hybrid,workers,inp=c
    assert 'error' not in out,(c[:5],out)
    assert out['all_vectors_and_choices_equal']
    key=c[:4]
    if workers==1:previous[key]=out['answers']
    else:assert out['answers']==previous[key]
    assert len(out['worker_stats'])==workers
    assert out['final_stats']['cache_hits']-out['cold_stats']['cache_hits']==len(inp['rows'])
    for name in ['peak_layer_rows','peak_retained_array_bytes','peak_counted_live_array_bytes','row_edges','native_core_nodes']:
        assert out['cold_stats'][name]==sum(w[name] for w in out['worker_stats'])
    if mode=='native':
        prepared=out['prepared'][0]
        for row,p,a in zip(inp['rows'],prepared,out['answers']):
            assert p['seeds']==row['seeds']
            for tile,count in zip(a['tiles'],a['counts']):
                focal=count if p['actor']%2 else len(row['worlds'])-count
                assert focal==row['expected_native_focal_success'][str(tile)]
    report.append({'counted':counted,'mode':mode,'field':field,'hybrid':hybrid,'workers':workers,'roots':len(out['answers']),'all_vectors_equal':True,'warm_hits':len(out['answers'])})
print(json.dumps({'requests':len(cases),'paired_worker1_worker4_checks':len(previous),'full_vectors_checked':sum(x['roots'] for x in report),'scalar_native_vectors_checked':sum(x['roots'] for x in report if x['mode']=='native'),'worker_stats_sum_verified':True,'cases':report},indent=2))
