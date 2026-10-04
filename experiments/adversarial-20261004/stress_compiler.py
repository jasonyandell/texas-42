#!/usr/bin/env python3
"""Challenge original compiler on new roots, partial tricks and weighted copies."""
import json
import time
from pathlib import Path
from parallel_roots import kernel_of, jobs_of, solve_action

HERE=Path(__file__).resolve().parent
rows=json.loads((HERE/'results/parallel-panel/plan.json').read_text())['rows']
records=[]
for row in rows:
    if row['status']!='ready': continue
    kernel=kernel_of(row);tape=row['tape'];bid=30
    expected=kernel.reference(tape,bid)[0]
    start=time.perf_counter()
    try:
        graph=kernel.compile(tape,node_cap=100000,seconds=3)
        actual=graph.reduce(graph.route(tape),bid)
        assert actual==expected
        records.append(dict(seed=row['seed'],ply=row['ply'],status='equal',nodes=len(graph.nodes),
            specialized_nodes=sum(solve_action(j)['nodes'] for j in jobs_of([row],'compiled')),
            adaptive_seconds=time.perf_counter()-start))
    except TimeoutError:
        records.append(dict(seed=row['seed'],status='compile_refused',attempt_seconds=time.perf_counter()-start))
assert len(records)==134
print(json.dumps(dict(records=records,checked=len(records),
    equal=sum(r['status']=='equal' for r in records),
    original_full_nodes=sum(r.get('nodes',0) for r in records),
    specialized_nodes=sum(r.get('specialized_nodes',0) for r in records)),indent=2))
