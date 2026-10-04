#!/usr/bin/env python3
"""Direct deterministic replay of saved legal history and rejected true-support world."""
import sys
sys.dont_write_bytecode=True
from pathlib import Path
import importlib.util,json,hashlib
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[2]
sys.path.insert(0,str(ROOT/'experiments/partnership'))
from rules import replay_record
fixture=json.loads((HERE/'receipts/bidder-counterexample-numpy/stdout.log').read_text())
record=[x for play in fixture['history'] for x in play]
actual=replay_record(fixture['full_deal'],record,fixture['trump'],fixture['bidder'])
assert actual[0]==[7,17]
assert max(actual[0][0]-12,actual[0][1]-29)<=0
try: replay_record(fixture['falsely_accepted_deal'],record,fixture['trump'],fixture['bidder'])
except AssertionError: pass
else: raise AssertionError('Counterexample deal unexpectedly legal')
results=[]
for version in ('drive','walt-sense/walt-sense'):
    source=ROOT/'experiments/astra-sol-20261004/phase2/incoming'/version/'engine42.py'
    spec=importlib.util.spec_from_file_location('engine_'+version.replace('/','_'),source)
    engine=importlib.util.module_from_spec(spec);spec.loader.exec_module(engine);np=engine.np
    full=np.array([[sum(1<<t for t in h) for h in fixture['falsely_accepted_deal']]])
    traces=[]
    for suppress_stop in (False,True):
        g=engine.Game(engine.Rules(fixture['trump']),full,30,0);g.leader[:]=1
        checks=[]
        for s,t in fixture['history']:
            checks.append(bool(int(g.legal(np.array([s]))[0])>>t&1))
            g.play(np.array([t]))
            if suppress_stop:g.done[:]=False
        traces.append(dict(suppress_stop=suppress_stop,checks=checks,all_accepted=all(checks),tricks=int(g.trick[0])))
    assert traces[0]['all_accepted'] and not traces[1]['all_accepted']
    assert traces[1]['checks'][7] is False
    results.append(dict(version=version,sha256=hashlib.sha256(source.read_bytes()).hexdigest(),traces=traces))
print(json.dumps(dict(original_history_legal=True,actual_points=actual[0],
    bad_deal_independently_illegal=True,numpy=np.__version__,python=sys.version,
    versions=results),indent=2))
