#!/usr/bin/env python3
"""Check retained Lean outputs, source hashes, and parent timing accounting."""
from pathlib import Path
import hashlib,json,re,statistics
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[2]
files={
 'phase1-lead':'math/lead/Disagreement.lean',
 'phase1-sol':'math/sol/ArgmaxAmplification.lean',
 'phase2-coupling':'phase2/math/sol/FirstDivergence.lean',
 'phase2-teacher':'phase2/TeacherFusion.lean',
 'phase3-dag':'phase3/math/sol/CompiledDAG.lean',
 'phase3-inner':'phase3/math/sol/InnerFiber.lean'}
proofs=[]
for name,relative in files.items():
    source=ROOT/'experiments/astra-sol-20261004'/relative
    text=source.read_text();run=json.loads((HERE/'receipts'/name/'run.json').read_text())
    assert run['status']=='completed' and run['child_returncode']==0
    assert not re.search(r'\b(?:sorry|admit|native_decide)\b|^\s*axiom\s',text,re.M)
    output=(HERE/'receipts'/name/'stdout.log').read_text()
    assert 'sorryAx' not in output
    proofs.append(dict(file=relative,sha256=hashlib.sha256(source.read_bytes()).hexdigest(),
        declarations=len(re.findall(r'^theorem ',text,re.M)),elapsed=run['elapsed_seconds'],axioms=output.splitlines()))
assert sum(p['declarations'] for p in proofs)==30
additional=[]
for name,filename in (('settled-pruning','SettledPruning.lean'),('priority-realization','PriorityRealization.lean'),('nested-plans','NestedPlans.lean')):
    source=HERE/filename;text=source.read_text();run=json.loads((HERE/'receipts'/name/'run.json').read_text())
    output=(HERE/'receipts'/name/'stdout.log').read_text()
    assert run['status']=='completed' and run['child_returncode']==0 and 'sorryAx' not in output
    assert not re.search(r'\b(?:sorry|admit|native_decide)\b|^\s*axiom\s',text,re.M)
    additional.append(dict(file=filename,sha256=hashlib.sha256(source.read_bytes()).hexdigest(),
        declarations=len(re.findall(r'^theorem ',text,re.M)),elapsed=run['elapsed_seconds'],axioms=output.splitlines()))
assert sum(p['declarations'] for p in additional)==9
summary=json.loads((HERE.parent/'results/parallel-panel/summary.json').read_text())
timings=json.loads((HERE.parent/'results/parallel-panel/timings.json').read_text())
assert len(timings)==20 and len({t['nodes'] for t in timings})==1
assert all(t['vectors']==timings[0]['vectors'] for t in timings)
assert len(timings[0]['vectors'])==134
medians={f'{mode}/{workers}':statistics.median(t['wall_s'] for t in timings if t['mode']==mode and t['workers']==workers)
    for mode in ('recursive','compiled') for workers in (0,4)}
native=json.loads((HERE.parent/'results/demand-mid/results.json').read_text())
assert len(native)==144 and all(r['status']=='complete' for r in native)
for seed in {r['seed'] for r in native}:
    for level in range(4):
        pair=[r['result'] for r in native if r['seed']==seed and r['level']==level]
        assert len(pair)==2 and pair[0]['counts']==pair[1]['counts'] and pair[0]['tiles']==pair[1]['tiles']
native_totals={f'{level}/{parallel}':{field:sum(r['result'][field] for r in native if r['level']==level and r['parallel']==parallel)
    for field in ('nodes','solve_us','retained_policy_cache_entries')}
    for level in range(4) for parallel in (False,True)}
print(json.dumps(dict(proofs=proofs,original_declarations=30,additional_proofs=additional,additional_declarations=9,total_declarations=39,
    parallel_wall_medians_s=medians,ready=summary['ready'],partial=summary['partial_trick_roots'],
    nodes=timings[0]['nodes'],all_vectors_identical=True,
    generation_s=summary['generation_s'],
    recursive_wall_ratio=medians['recursive/0']/medians['recursive/4'],
    compiled_wall_ratio=medians['compiled/0']/medians['compiled/4'],
    recursive_with_common_sampling_ratio=(summary['generation_s']+medians['recursive/0'])/(summary['generation_s']+medians['recursive/4']),
    native_runs=len(native),native_roots=len({r['seed'] for r in native}),native_mode_pairs_equal=True,native_totals=native_totals,
    scope='Arithmetic/source/receipt audit; not an independent timing rerun.'),indent=2))
