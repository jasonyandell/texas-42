"""Addressed-field oracle, pruning, partial-cache and operation-order checks."""
import argparse,json,time
from pathlib import Path
import numpy as np
import repaired as r
import turbo
from verify_turbo import scalar,sample,fixed_values
from verify_repaired import position
from verify import oracle

def run(output='results/amortize-verification.json'):
    started=time.perf_counter();cases=[];cuts=0;fixture=None
    for seed in range(1,41):
        for depth in (20,21,22,23):
            rules,hands,p=position(seed,depth)
            if any(bool(v[0]) for v in p.outcome()):continue
            hand=hands[int(p.turn()[0])]&~int(p.played[0])
            for reuse in (False,True):
                for delta in (0.,.125,1.):
                    spec=r.Spec((3,4,3,2),delta,57)
                    f=turbo.Field(rules,spec,addressed=True,reuse_deals=reuse)
                    raw=turbo.Field(rules,spec,addressed=True,reuse_deals=reuse,prune=False,tables=False,cache=False,node_cache=False)
                    for level in (1,2,3,4):
                        assert np.array_equal(f.sample(level,p,hand),sample(f,level,p,hand))
                        expected=scalar(f,level,p,hand)
                        first=f.actions(level,p,np.array([hand]))
                        a,vals=f.actions(level,p,np.array([hand]),True)
                        b,full=raw.actions(level,p,np.array([hand]),True)
                        assert np.array_equal(a,first) and np.array_equal(a,b)
                        assert vals==full
                        error=max(abs(v-vals[0][t]) for t,v in expected.items())
                        assert error<1e-10,(level,delta,reuse,expected,vals)
                        best=(max if int(p.turn()[0])%2 else min)(expected.values())
                        assert a[0]==min(t for t,v in expected.items() if v==best)
                        cases.append({'seed':seed,'depth':depth,'level':level,'delta':delta,'reuse':reuse,'error':error})
                    cuts+=f.metrics['pruned_actions'];f.close();raw.close()
                    fixture=(rules,p,hand)
            if len(cases)>=192:break
        if len(cases)>=192:break
    assert len(cases)>=192 and cuts>0
    # Mixed histories and seats in a single contract, cache pollution and
    # full-value queries after action-only calls must preserve the fixed field.
    group=[]
    for path in range(1,25):
        rules,hands,p=position(2,16,path)
        if not any(bool(v[0]) for v in p.outcome()):
            group.append((p,hands[int(p.turn()[0])]&~int(p.played[0])))
        if len(group)==8:break
    spec=r.Spec((4,8,4),1.,42)
    f=turbo.Field(rules,spec,addressed=True)
    reference=turbo.Field(rules,spec,addressed=True,prune=False,cache=False,tables=False,node_cache=False)
    expected=[reference.actions(2,p,np.array([h]),True) for p,h in group]
    for i in list(range(len(group)))[::-1]+list(range(len(group))):
        p,h=group[i];chosen=f.actions(2,p,np.array([h]))
        a,v=f.actions(2,p,np.array([h]),True)
        assert np.array_equal(a,chosen) and np.array_equal(a,expected[i][0]) and v==expected[i][1]
    # Changing only enclosing sample counts cannot change a lower field.
    other=turbo.Field(rules,r.Spec((4,99)),addressed=True)
    p,h=group[0]
    assert f.actions(1,p,np.array([h]),True)[1]==other.actions(1,p,np.array([h]),True)[1]
    for obj in (f,reference,other):obj.close()
    # Preserve the explicit witness where per-world optimization is wrong.
    rules,hands,p=position(7,16);me=int(p.turn()[0]);hand=hands[me]&~int(p.played[0])
    f=turbo.Field(rules,r.Spec((8,),0.,73),addressed=True)
    worlds=f.sample(1,p,hand);actual=fixed_values(f,p,worlds);exact=oracle(rules.trump,me,p,worlds)
    assert max(abs(actual[t]-float(exact[t])) for t in exact)<1e-10
    clairvoyant=sum(fixed_values(f,p,[world])[12] for world in worlds)
    assert abs(actual[12]-clairvoyant)>1e-8
    f.close()
    report={'status':'pass','seconds':time.perf_counter()-started,'oracle_cases':cases,
            'action_then_full_value_checks':len(cases)+len(group)*2,'root_bound_stops':cuts,
            'mixed_history_order_cache_checks':len(group)*2,
            'lower_field_identity_invariant':True,'strategy_fusion_witness_preserved':True}
    Path(output).write_text(json.dumps(report,indent=2)+'\n')
    print(json.dumps({**{k:v for k,v in report.items() if k!='oracle_cases'},'oracle_cases':len(cases)}))

if __name__=='__main__':
    parser=argparse.ArgumentParser()
    parser.add_argument('--output',default='results/amortize-verification.json')
    run(parser.parse_args().output)
