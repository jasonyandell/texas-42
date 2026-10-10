"""Semantic gates for CPU throughput variants, independent of their timings."""
import json
import numpy as np
import walt42x as ref
import fast_numpy as fast
from bench import fixture
from verify import oracle

reference_search=ref.search
cases=[]
for seed,played in [(2,16),(3,17),(5,18),(6,19),(7,20),(8,21),(9,22),(11,23)]:
    try: trump,seat,pub,deals,_=fixture(seed,played,8)
    except ValueError: continue
    rules=ref.Rules(trump)
    for delta in (0.0,0.125,0.5,1.0):
        rng=np.random.default_rng(123)
        expected=reference_search(rules,seat,pub,deals,ref.random_policy,rng,delta)
        state=rng.bit_generator.state
        for label,solve in [('numpy',fast.search),('compiled',fast.compiled_search)]:
            r=np.random.default_rng(123)
            actual=solve(rules,seat,pub,deals,ref.random_policy,r,delta)
            assert actual.keys()==expected.keys()
            err=max(abs(actual[k]-expected[k]) for k in expected)
            assert err<1e-10,(seed,played,delta,label,err)
            assert r.bit_generator.state==state,(seed,delta,label,'RNG mismatch')
            if delta==0 and played>=20:
                exact=oracle(trump,seat,pub,deals)
                assert max(abs(actual[k]-float(exact[k])) for k in actual)<1e-10
            cases.append({'seed':seed,'played':played,'delta':delta,'backend':label,'max_error':err})

# Generic assumed-policy compatibility on a bounded nested query. Preserves the
# reference's field semantics; this does not fix its missing voids/purity.
trump,seat,pub,deals,_=fixture(2,20,5)
rules=ref.Rules(trump)
nested=[]
for label,solve in [('reference',reference_search),('numpy',fast.search),('compiled',fast.compiled_search)]:
    ref.search=solve
    assume=ref.walt(ref.random_policy,8,1)
    rng=np.random.default_rng(345)
    values=solve(rules,seat,pub,deals,assume,rng,1)
    nested.append({'backend':label,'values':values,'rng':rng.bit_generator.state})
assert all(x['values']==nested[0]['values'] and x['rng']==nested[0]['rng'] for x in nested)
ref.search=reference_search
report={'status':'pass','cases':cases,'nested_compatibility':nested}
with open('results/throughput-verification.json','w') as f: json.dump(report,f,indent=2)
print(json.dumps({'status':'pass','root_cases':len(cases),'nested_cases':len(nested)}))
