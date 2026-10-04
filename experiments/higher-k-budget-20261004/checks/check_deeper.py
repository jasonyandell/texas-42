#!/usr/bin/env python3
"""Independent native oracle at all eligible first-nine frozen twelve-ply roots.
Also check controlled-clock WASM exhaustion retains the initial public checkpoint.
"""
import argparse,json,subprocess,sys
sys.dont_write_bytecode=True
from audit_data import CHECK,HERE,read,rotated,request,replay,legal
from check_adapter import call
def main():
    parser=argparse.ArgumentParser();parser.add_argument('--vectors',default='deep-independent-vectors.json');args=parser.parse_args()
    fixtures=read(HERE/'fixtures.json')[:9];calls=[];excluded=0
    for f in fixtures:
        hands,plays=rotated(f,0,12);r=request(hands,plays,f['decl'],0)
        remain,_,trick,points=replay(hands,plays,f['decl'],0)
        if len(legal(remain[r['seat']],trick,f['decl']))<2 or points[0]>=30 or points[1]>12:
            excluded+=1;continue
        calls.extend(dict(request=r,k=k)for k in range(1,6))
    p=subprocess.run([str(CHECK/'target/release/higher-k-independent-check')],input=''.join(json.dumps(c)+'\n'for c in calls),capture_output=True,text=True,timeout=250)
    assert p.returncode==0,p.stderr
    rows=[json.loads(line)for line in p.stdout.splitlines()];assert len(rows)==len(calls)
    path=CHECK/args.vectors;assert not path.exists();path.write_text(json.dumps(rows,separators=(',',':'))+'\n')
    native=HERE/'adapter/target/release/higher-k-player';wasm=HERE/'adapter/target/wasm32-unknown-unknown/release/higher_k_player.wasm'
    comparisons=0
    for e in rows:
        assert e['evaluation'], 'Independent30s oracle censored'
        payload=dict(action='ladder',request=e['request'],k=e['k'],budget_ms=30000,validate=True)
        for binary in[native,wasm]:
            v=call(binary,payload)
            assert v['completed']and v['evaluation']==e['evaluation']and v['worlds']==e['worlds']and v['rng_final']==e['rng_final']and v['outer_attempts']==e['outer_attempts']
            assert v['pi_calls_by_level']==e['pi_calls_by_level']and v['inner_worlds_by_level']==e['inner_worlds_by_level'];comparisons+=1
    exhausted=0
    for r in read(CHECK/'fresh-roots.json'):
        payload=dict(action='ladder',request=r,k=5,budget_ms=20)
        p=subprocess.run(['node',str(CHECK/'wasm_worker.mjs'),str(wasm),'expire'],input=json.dumps(payload)+'\n',capture_output=True,text=True,timeout=10)
        assert p.returncode==0,p.stderr;v=json.loads(p.stdout);result=v['result'];initial=v['checkpoints'][0]
        assert len(v['checkpoints'])==1 and not result['completed']and 'evaluation'not in result and result['refusal']=='outer-sampling-budget'
        assert result['choice']==initial['choice']==result['legal'][0]and result['legal']==initial['legal'];exhausted+=1
    print(json.dumps(dict(planned_first9_roots=9,ineligible_roots=excluded,eligible_roots=len(calls)//5,independent_rung_vectors=len(rows),native_wasm_full_vector_sample_rng_counter_comparisons=comparisons,controlled_clock_checkpoint_refusals=exhausted,scope='Finite12ply refinement on existing frozen source deals; no additional independent strength samples, actual browser interruptions or timings.'),indent=2))
if __name__=='__main__':main()
