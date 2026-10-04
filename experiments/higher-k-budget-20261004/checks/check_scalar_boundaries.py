#!/usr/bin/env python3
"""Invalid-k integers must not wrap into allowed rungs on WASM32."""
import argparse,json,sys
sys.dont_write_bytecode=True
from check_adapter import CHECK,call
HERE=CHECK.parent
def main():
    parser=argparse.ArgumentParser();parser.add_argument('--records',required=True);args=parser.parse_args()
    path=CHECK/args.records;assert not path.exists(),'preserve earlier scalar evidence'
    request=json.loads((CHECK/'fresh-roots.json').read_text())[0]
    native=HERE/'adapter/target/release/higher-k-player';wasm=HERE/'adapter/target/wasm32-unknown-unknown/release/higher_k_player.wasm';rows=[]
    bad=[2**32+1,2**32+2,2**32+5,2**64-1,-1,True,1.5]
    for k in bad:
        payload=dict(action='ladder',request=request,k=k,budget_ms=0)
        a=call(native,payload);b=call(wasm,payload);rows.append(dict(k=k,native=a,wasm=b))
        path.write_text(json.dumps(rows,separators=(',',':'))+'\n')
        assert 'error'in a and 'error'in b,(k,a,b)
    print(json.dumps(dict(invalid_scalars=len(bad),both_backends_reject=True,scope='Malformedinput boundary only, no validpolicy change.')))
if __name__=='__main__':main()
