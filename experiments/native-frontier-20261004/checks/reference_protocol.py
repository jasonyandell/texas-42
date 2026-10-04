#!/usr/bin/env python3
import json,subprocess
from pathlib import Path
HERE=Path(__file__).resolve().parent
rows=json.loads((HERE/'independent-panel.json').read_text())['rows']
cases=[]
for counted in [False,True]:
    for mode,field in [('native',0),('policy',0),('policy',1),('policy',2)]:
        subset=rows if mode=='native' else rows[:6]
        for batch,parallel in [(False,False),(True,False),(True,True)]:
            inp={'rows':subset,'mode':mode,'field_level':field,'workers':4,'counted':counted,'budgets':[4,2,2,2],'reference':True,'reference_only':True,'batch_reference':batch,'parallel_reference':parallel,'reference_after':True,'caps':{'rows':500000,'work':5000000,'queries':50000,'cache_entries':50000,'seconds':20}}
            cases.append((counted,mode,field,batch,parallel,inp))
        if mode=='policy' and field==2:
            # New bounded highest-rung panel gets an independent full-frontier comparison.
            inp=dict(cases[-1][-1],reference_only=False,workers=4)
            cases.append((counted,mode,field,True,True,inp))
run=subprocess.run([str(HERE/'runner-target/release/native-frontier')],input=''.join(json.dumps(c[-1])+'\n' for c in cases),capture_output=True,text=True,timeout=60)
assert run.returncode==0,run.stderr
answers=[json.loads(line) for line in run.stdout.splitlines()];assert len(answers)==len(cases)
expected={};report=[]
for c,out in zip(cases,answers):
    counted,mode,field,batch,parallel,inp=c;assert 'error' not in out,(c[:5],out)
    key=c[:3]
    if key in expected:assert out['answers']==expected[key]
    else:expected[key]=out['answers']
    assert len(out['answers'])==len(inp['rows'])
    assert 0<=out['reference_total_us']<=out['request_total_us']
    assert out['reference_total_us']+out['outer_validation_us']<=out['request_total_us']
    if mode=='native':
        for row,a in zip(inp['rows'],out['answers']):
            mine=row['request']['seat']%2==row['request']['bidder']%2
            for tile,count in zip(a['tiles'],a['counts']):assert (count if mine else len(row['worlds'])-count)==row['expected_native_focal_success'][str(tile)]
    report.append({'counted':counted,'mode':mode,'field':field,'batch':batch,'parallel':parallel,'reference_only':inp['reference_only'],'roots':len(out['answers']),'vectors_equal':True})
print(json.dumps({'requests':len(cases),'full_vectors_checked':sum(r['roots'] for r in report),'new_field2_frontier_roots':12,'reference_wall_scope_validated':True,'cases':report},indent=2))
