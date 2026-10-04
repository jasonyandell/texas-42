#!/usr/bin/env python3
"""Verify filled slots on a legal odd-bidder fixture, and true refusal at tries=0."""
import sys
sys.dont_write_bytecode=True
from pathlib import Path
import importlib.util,json,hashlib
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[2]
sys.path.insert(0,str(ROOT/'experiments/astra-sol-20261004/phase2/incoming/drive'))
import engine42 as engine
sys.path.insert(0,str(ROOT/'experiments/partnership'))
from rules import replay_record
SOURCE=HERE.parent/'incoming/drive-0815/batched.v2.py'
spec=importlib.util.spec_from_file_location('batched_v2',SOURCE);module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
np=engine.np;fixture=json.loads((HERE/'receipts/bidder-counterexample-numpy/stdout.log').read_text())
history=fixture['history'];played=[sum(1<<t for s,t in history if s==seat) for seat in range(4)]
seats=np.arange(4);hands=np.array([sum(1<<t for t in h) for h in fixture['actual']['remaining']])
played_by=np.tile(np.array(played),(4,1));hs=np.tile(np.array([s for s,t in history]),(4,1));ht=np.tile(np.array([t for s,t in history]),(4,1))
results=[]
for tries in (0,1,6):
    rem,filled=module.batched_worlds(np.random.default_rng(70002),engine.Rules(fixture['trump']),1,seats,hands,played_by,hs,ht,32,tries=tries)
    assert rem.shape==(4,32,4) and filled.shape==(4,32) and filled.dtype==np.bool_
    assert np.all(rem[~filled]==0)
    if tries==0:assert not filled.any()
    for i,k in zip(*np.nonzero(filled)):
        assert int(rem[i,k,i])==int(hands[i])
        full=[{t for t in range(28) if int(mask|played_by[i,s])>>t&1} for s,mask in enumerate(rem[i,k])]
        points,leader,remaining,trick=replay_record(full,[x for pair in history for x in pair],fixture['trump'],1)
        assert points==[7,17] and leader==0 and not trick
    results.append(dict(tries=tries,filled=int(filled.sum()),unfilled=int((~filled).sum()),
        all_reported_filled_worlds_independently_legal=True,unfilled_slots_zero=True))
print(json.dumps(dict(source_sha256=hashlib.sha256(SOURCE.read_bytes()).hexdigest(),
    engine_sha256=hashlib.sha256(Path(engine.__file__).read_bytes()).hexdigest(),results=results,
    caller_l2_present=False,scope='Valid unsettled bid30/pip-trump histories; fill-mask fixture check, not caller integration or universal sampler proof.'),indent=2))
