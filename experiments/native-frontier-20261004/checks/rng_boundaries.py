#!/usr/bin/env python3
"""Construct targeted SplitMix seeds for native rejection-boundary checks."""
import json
from scalar_oracle import MASK,STEP,mix,below

def unxor(value,shift):
    result=value;p=shift
    while p<64:
        result^=value>>p;p+=shift
    return result

def state_for_first(value):
    z=unxor(value,31)
    z=(z*pow(0x94d049bb133111eb,-1,1<<64))&MASK
    z=unxor(z,27)
    z=(z*pow(0xbf58476d1ce4e5b9,-1,1<<64))&MASK
    z=unxor(z,30)
    return (z-STEP)&MASK

if __name__=='__main__':
    cases=[]
    for n in range(1,29):
        zone=MASK-MASK%n
        for first in [0,n-1,n,zone-1,zone,MASK]:
            seed=state_for_first(first)
            assert mix(seed)==first
            state=seed;draws=[]
            while True:
                value=mix(state);state=(state+STEP)&MASK;draws.append(value)
                if value<zone:break
            expected=value%n
            assert below(seed,n)==expected
            cases.append({'n':n,'first':first,'seed':seed,'draw_count':len(draws),'expected':expected})
    output={'roundtrips':len(cases),'forced_rejection_cases':sum(c['draw_count']>1 for c in cases),'cases':cases}
    print(json.dumps(output,indent=2))
