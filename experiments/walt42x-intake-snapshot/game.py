"""Run the supplied main loop with its L0 searches on Metal."""
import json
import os
import time
import numpy as np
import torch
import walt42x as ref
from metal import Backend, Rules, Pub, search

assert os.environ.get('PYTORCH_ENABLE_MPS_FALLBACK') != '1'
torch.set_num_threads(1)
b=Backend('mps')
rows=[]

def gpu_walt(assume,n=8,delta=0):
    def policy(rules,seat,hand,pub,rng,voids=(0,0,0,0)):
        assert assume is ref.random_policy, 'Only random-policy rung ported'
        b.sync()
        started=time.perf_counter()
        r=Rules.__new__(Rules)
        r.b=b
        for k in ('suit','lead','strength'): setattr(r,k,b.array(getattr(rules,k)))
        P=np.zeros((len(seat),28))
        for i in range(len(seat)):
            s,h=int(seat[i]),int(hand[i])
            deals=ref.sample(s,h,int(pub.played[i]),int(pub.leader[i]),int(pub.tlen[i]),n,rng,voids)
            values,census=search(r,s,Pub.from_ref(b,pub.take(slice(i,i+1))),b.array(deals),rng,delta)
            t=(max if s%2 else min)(values,key=values.get)
            P[i,t]=1
        b.sync()
        rows.append({'seat':s,'tile':t,'ms':(time.perf_counter()-started)*1000,
                     'values':values,'peak_fibers':max(c['fibers'] for c in census)})
        return P
    return policy

if __name__ == '__main__':
    ref.walt=gpu_walt
    for run in ('cold','warm'):
        rows.clear()
        start=time.perf_counter()
        ref.main(1,1.0,0)
        b.sync()
        report={'seed':1,'delta':1.0,'level':0,'deals':30,'device':'mps','fallback':False,
                'run':run,'total_ms':(time.perf_counter()-start)*1000,'moves':rows}
        with open(f'results/fixed-game-mps-{run}.json','w') as f: json.dump(report,f,indent=2)
        print(json.dumps({'run':run,'total_ms':report['total_ms'],'moves':len(rows)}))
