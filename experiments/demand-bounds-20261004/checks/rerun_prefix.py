#!/usr/bin/env python3
"""Fresh native/full/hybrid parity run over the prior six-root demand panel."""
import json,sys
from pathlib import Path
sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT/'experiments/prefix-state-20261004'))
from compare import call,fixture
binary=ROOT/'experiments/demand-bounds-20261004/checks/prefix-target/release/native-frontier'
retained=json.loads((ROOT/'experiments/demand-bounds-20261004/checks/checkpoint-audit-native/stdout.log').read_text())
records=[];vectors=0
for n in [2,8,40]:
    rows=[fixture(962000+i,16+i%4,n) for i in range(32)]
    rows=[r for r in rows if r['status']=='ready' and r['request']['decl']<=6]
    assert len(rows)==6
    for field in [0,1,2]:
        inp=dict(rows=rows,mode='policy',field_level=field,budgets=[4,2,2,2],counted=False,warm=False,workers=1,caps=dict(rows=500000,work=20000000,queries=50000,cache_entries=50000,seconds=30))
        reference=call(binary,inp|{'reference':True,'reference_only':True})
        variants={}
        previous=next(r for r in retained['matrix'] if (r['worlds'],r['field'])==(n,field))
        for name in ['full','hybrid']:
            run=call(binary,inp|{'native_choices':name=='hybrid'});x=run['result'];stats=x.get('cold_stats',x.get('worker_stats',[{}])[0])
            complete='answers' in x;assert complete==previous['details'][name]['complete']
            if complete:assert x['answers']==reference['result']['answers'];vectors+=len(rows)
            else:assert x['error']=='actor query cap';assert 'answers' not in x
            for current,old in [('unique_actor_misses_by_level','unique_misses'),('actor_demands_by_level','actor_demands'),('inner_worlds_by_level','inner_worlds'),('native_core_pi_misses_by_level','native_misses'),('native_core_reported_inner_worlds_by_level','native_reported_worlds')]:assert stats[current]==previous['details'][name][old],(n,field,name,current)
            variants[name]=run
        records.append(dict(worlds=n,field=field,variants=variants,reference=reference))
        print(json.dumps({'worlds':n,'field':field,'status':'parity and demand counts reproduced'}),flush=True)
(Path(__file__).resolve().parent/'prefix-rerun-records.json').write_text(json.dumps({'vectors':vectors,'records':records},indent=2)+'\n')
assert vectors==102
print(json.dumps({'completed_vectors':vectors,'full_refusals':1,'all_completed_choices_and_vectors_equal':True}))
