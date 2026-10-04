#!/usr/bin/env python3
"""New production-phone routing against exact pinned local WASM blob."""
import argparse,json,subprocess,sys
from pathlib import Path
sys.dont_write_bytecode=True
CHECK=Path(__file__).resolve().parent;HERE=CHECK.parent;ROOT=HERE.parents[1]
def semantic(value):
    if isinstance(value,dict):return {k:semantic(v)for k,v in value.items()if k not in['elapsed_us','solver_us','over_budget']}
    if isinstance(value,list):return [semantic(v)for v in value]
    return value
def call(binary,payload):
    cmd=['node',str(CHECK/'wasm_worker.mjs'),str(binary)]if binary.suffix=='.wasm'else[str(binary)]
    p=subprocess.run(cmd,input=json.dumps(payload)+'\n',capture_output=True,text=True,timeout=25)
    assert p.returncode==0,p.stderr
    v=json.loads(p.stdout);return v if 'result'in v else dict(result=v)
def main():
    parser=argparse.ArgumentParser();parser.add_argument('--records',default='phone-records.json');args=parser.parse_args()
    path=CHECK/args.records;assert not path.exists(),'preserve phone evidence'
    native=HERE/'adapter/target/release/higher-k-player';wasm=HERE/'adapter/target/wasm32-unknown-unknown/release/higher_k_player.wasm'
    pinned=ROOT/'experiments/astra-sol-20261004/reference/production-phone/walt-player.wasm'
    roots=json.loads((CHECK/'fresh-roots.json').read_text());records=[]
    for r in roots:
        plain=dict(request=r,worlds=40,partner=True,budget_ms=200)
        a=call(native,dict(plain,action='phone'));b=call(wasm,dict(plain,action='phone'));c=call(pinned,plain)
        records.append(dict(call=plain,native=a,wasm=b,pinned=c))
        path.write_text(json.dumps(records,separators=(',',':'))+'\n')
        assert 'error'not in a['result']and semantic(a['result'])==semantic(b['result'])==semantic(c['result'])
        assert b['checkpoints']and c['checkpoints']
        assert semantic(b['checkpoints'])==semantic(c['checkpoints'])
        for checkpoint in b['checkpoints']:
            assert checkpoint['legal']==b['result']['legal']and checkpoint['choice']in checkpoint['legal']
    print(json.dumps(dict(calls=9,all9declarations=True,nominal_budget_ms=200,full_nonclock_result_comparisons=18,wasm_checkpoint_sequences_match=True,excluded_fields=['elapsed_us','solver_us','over_budget'],scope='Finite unchanged production L1/partner profile with base8 samples at200ms; separate from ladderbase4. Local pinned blob, no device or current externaldeployment verification.'),indent=2))
if __name__=='__main__':main()
