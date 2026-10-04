#!/usr/bin/env python3
"""Finite independent sample/full-vector/RNG/refusal audit, run under watchdog."""
import argparse,copy,hashlib,json,subprocess,sys
from pathlib import Path
sys.dont_write_bytecode=True
CHECK=Path(__file__).resolve().parent;ROOT=CHECK.parents[2]
sys.path.insert(0,str(ROOT/'experiments/partnership'))
from rules import information_state,replay_record
def call(binary,payload):
    host=CHECK/'wasm_worker.mjs'
    command=['node',str(host),str(binary)]if binary.suffix=='.wasm'else[str(binary)]
    p=subprocess.run(command,input=json.dumps(payload)+'\n',capture_output=True,text=True,timeout=35)
    assert p.returncode==0,(p.returncode,p.stderr)
    value=json.loads(p.stdout)
    if 'result'in value and payload.get('action')=='ladder' and 'error'not in value['result']:
        checkpoints=value['checkpoints'];assert len(checkpoints)==1
        assert checkpoints[0]['choice']==value['result']['legal'][0]and checkpoints[0]['legal']==value['result']['legal']and not checkpoints[0]['completed']
    return value.get('result',value)
def oracle():
    rows=json.loads((CHECK/'fresh-roots.json').read_text())
    calls=[dict(request=r,k=k)for r in rows for k in range(1,6)]
    runner=CHECK/'target/release/higher-k-independent-check'
    p=subprocess.run([str(runner)],input=''.join(json.dumps(r)+'\n'for r in calls),text=True,capture_output=True,timeout=250)
    assert p.returncode==0,p.stderr
    expected=[json.loads(line)for line in p.stdout.splitlines()]
    assert len(expected)==45 and all(e['evaluation'] for e in expected)
    path=CHECK/'independent-vectors.json';assert not path.exists(),'preserve evidence'
    path.write_text(json.dumps(expected,separators=(',',':'))+'\n')
    print(json.dumps(dict(roots=9,complete_vectors=45,ordered_samples_checked=1800,all_rungs=[1,2,3,4,5],runner_sha256=hashlib.sha256(runner.read_bytes()).hexdigest())))
def audit(binary):
    expected=json.loads((CHECK/'independent-vectors.json').read_text())
    ties=world_replays=zero=0
    for e in expected:
        r=e['request'];v=call(binary,dict(action='ladder',request=r,k=e['k'],budget_ms=30000,validate=True))
        assert v['completed'] and v['evaluation']==e['evaluation'],(e,v)
        assert v['worlds']==e['worlds'] and v['rng_final']==e['rng_final'] and v['outer_attempts']==e['outer_attempts']
        assert v['field_level']==e['k']-1 and v['inner_budgets']==[4,2,2,2,2,2]
        state=information_state(r)
        assert v['legal']==state['legal'] and v['points']==state['points'] and v['leader']==state['leader']
        assert v['pi_calls_by_level']==e['pi_calls_by_level'] and v['inner_worlds_by_level']==e['inner_worlds_by_level']
        counts=v['evaluation']['counts'];ties+=len(counts)!=len(set(counts))
        best=(max if r['seat']%2==r['bidder']%2 else min)(counts)
        assert v['choice']==v['evaluation']['tiles'][counts.index(best)]
        rotation=1 if r['bidder']%2==0 else 0
        for world in v['worlds']:
            hands=[sorted([t for t in range(28)if world[(s+rotation)%4]&(1<<t)]+[t for ss,t in zip(r['plays'][::2],r['plays'][1::2])if ss==s])for s in range(4)]
            points,leader,_,_=replay_record(hands,r['plays'],r['decl'],r['bidder'])
            assert hands[r['seat']]==sorted(r['hand']) and points==state['points'] and leader==state['leader'];world_replays+=1
        z=call(binary,dict(action='ladder',request=r,k=e['k'],budget_ms=0))
        assert not z['completed'] and 'evaluation'not in z and z['choice']==state['legal'][0] and z.get('refusal')=='outer-sampling-budget';zero+=1
    bad=[];base=expected[0]['request']
    for key,value in [('opponent_hands',[[1,2]]),('source_seed',101),('decl',8),('bid',286),('seat',4),('hand',[0]*7),('plays',[0])]:
        r=copy.deepcopy(base);r[key]=value;bad.append(dict(action='ladder',request=r,k=3,budget_ms=200))
    for k in [0,6]:bad.append(dict(action='ladder',request=base,k=k,budget_ms=200))
    bad.extend([dict(action='ladder',request=base,k=3,budget_ms=30001),dict(action='ladder',request=base,k=3,budget_ms=200,private_worlds=[])])
    for payload in bad:assert 'error'in call(binary,payload),payload
    print(json.dumps(dict(full_vectors=45,independent_roots=9,all9declarations=True,ordered_worlds_and_final_rng=True,
        independent_full_deal_replays=world_replays,tied_vectors=ties,zero_budget_lawful_refusals=zero,malformed_refusals=len(bad),
        binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),scope='Finite conformance and solver-returned refusal only; no browser/host-interruption, timing, or strength theorem.'),indent=2))
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--oracle',action='store_true');p.add_argument('--binary',type=Path);a=p.parse_args()
    oracle()if a.oracle else audit(a.binary)
