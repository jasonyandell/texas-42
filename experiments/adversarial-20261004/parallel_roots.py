#!/usr/bin/env python3
"""Exploratory exact frozen-bundle root-action parallelism. Run with run_capped.py.

Each worker receives ALL scenarios for one root action. It never separately
optimizes a world or tape and then averages. Packed graphs specialize one bid;
they cannot be reused after changing the bid, field, root, or tape.
"""
import argparse
from array import array
from concurrent.futures import ProcessPoolExecutor
import hashlib
import json
import multiprocessing
import os
from pathlib import Path
import pickle
import random
import resource
import sys
import time

HERE = Path(__file__).resolve().parent
SOURCE = HERE.parent / 'astra-sol-20261004'
sys.path.insert(0, str(SOURCE / 'phase3'))
from compiled_tape import Kernel, make_tape, pick, popcount
from rules import legal_tiles, winner, replay_record, information_state


def fixture(seed, ply, n):
    rng = random.Random(seed)
    deck = list(range(28)); rng.shuffle(deck)
    hands = [sorted(deck[7*s:7*s+7]) for s in range(4)]
    remaining = list(map(set, hands))
    bidder = seed % 4; leader = bidder; decl = (*range(8), 9)[seed % 9]
    trick = []; plays = []
    for _ in range(ply):
        seat = (leader + len(trick)) % 4
        tile = rng.choice(legal_tiles(remaining[seat], trick, decl))
        remaining[seat].remove(tile); plays.extend([seat, tile]); trick.append((seat, tile))
        if len(trick) == 4: leader = winner(trick, decl); trick = []
    points, lead, remain, tail = replay_record(hands, plays, decl, bidder)
    seat = (lead + len(tail)) % 4
    req = dict(decl=decl, bid=30, bidder=bidder, seat=seat, hand=hands[seat], plays=plays)
    state = information_state(req)
    if len(state['legal']) < 2 or points[bidder % 2] >= 30 or points[1-bidder % 2] > 12:
        return dict(seed=seed, ply=ply, status='preexcluded_forced_or_settled')
    # Rejection from a uniformly shuffled partition is uniform over feasible
    # remaining deals. Full-history replay checks all exposed voids. This is
    # support sampling, NOT action-likelihood posterior inference.
    hidden = sorted(t for s in range(4) if s != seat for t in remain[s])
    past = [[t for s,t in zip(plays[::2],plays[1::2]) if s == a] for a in range(4)]
    worlds = []; attempts = 0
    sample_rng = random.Random(seed + 88000000)
    while len(worlds) < n and attempts < 100000:
        attempts += 1; sample_rng.shuffle(hidden); pos = 0; full = []
        for s in range(4):
            if s == seat: full.append(hands[s])
            else:
                full.append(past[s] + hidden[pos:pos+len(remain[s])]); pos += len(remain[s])
        try: pp, ll, rr, tt = replay_record(full, plays, decl, bidder)
        except AssertionError: continue
        assert pp == points and ll == lead and tt == tail
        worlds.append([sum(1 << t for t in h) for h in rr])
    if len(worlds) != n:
        return dict(seed=seed, ply=ply, status='sampling_refused', attempts=attempts)
    return dict(seed=seed, ply=ply, status='ready', request=req, worlds=worlds,
                points=points, leader=lead, trick=tail, sampling_attempts=attempts,
                tape=make_tape(seed+77000000,n,28-ply))


def kernel_of(row):
    return Kernel(row['request'], row['worlds'], row['points'], row['leader'],
                  tuple(tuple(x) for x in row['trick']))


def solve_action(job):
    row, action, mode = job
    began = time.perf_counter(); cpu = time.process_time()
    kernel = kernel_of(row); tape = row['tape']; bid = row['request']['bid']
    nodes = 0; edges_count = 0
    # LEAF=0, SUM=1, MAX=2. Every edge points to an earlier node.
    ops = bytearray(); starts = array('I'); ends = array('I'); leaves = array('I'); edges = array('I')
    deadline = time.monotonic() + 12

    def rec(state, worlds, depth):
        nonlocal nodes, edges_count
        nodes += 1
        if nodes > 500000 or (nodes % 1024 == 0 and time.monotonic() > deadline):
            raise TimeoutError('action node/time cap')
        if (state.points[kernel.bidder % 2] >= bid or
                state.points[1-kernel.bidder % 2] > 42-bid or popcount(state.played) == 28):
            op = 0; value = int(kernel.success(state.points,bid))*len(worlds); children = []
        elif (state.leader + len(state.trick)) % 4 == kernel.focal:
            legal = kernel.moves(state, worlds[0])
            assert all(kernel.moves(state,w) == legal for w in worlds)
            children = [rec(kernel.after(state,t),worlds,depth+1) for t in legal]
            op = 2; value = 0
        else:
            groups = {}
            for w in worlds:
                tile = pick(tape,w,depth,kernel.moves(state,w)); groups.setdefault(tile,[]).append(w)
            children = [rec(kernel.after(state,t),ws,depth+1) for t,ws in sorted(groups.items())]
            op = 1; value = 0
        edges_count += len(children)
        if mode == 'recursive':
            return value if op == 0 else sum(children) if op == 1 else max(children)
        starts.append(len(edges)); edges.extend(children); ends.append(len(edges))
        ops.append(op); leaves.append(value)
        return len(ops)-1

    root = rec(kernel.after(kernel.root,action),list(range(kernel.n)),1)
    compile_s = time.perf_counter()-began
    reduction_s = 0; packed_bytes = 0
    if mode == 'compiled':
        tick = time.perf_counter(); values = array('I',[0])*len(ops)
        for i, op in enumerate(ops):
            children = (values[edges[j]] for j in range(starts[i],ends[i]))
            values[i] = leaves[i] if op == 0 else sum(children) if op == 1 else max(children)
        value = values[root]; reduction_s = time.perf_counter()-tick
        packed_bytes = len(ops)+sum(len(a)*a.itemsize for a in (starts,ends,leaves,edges,values))
    else: value = root
    return dict(seed=row['seed'],action=action,value=value,nodes=nodes,edges=edges_count,
                compile_s=compile_s,reduction_s=reduction_s,wall_s=time.perf_counter()-began,
                cpu_s=time.process_time()-cpu,packed_payload_bytes=packed_bytes,
                worker_peak_rss_bytes=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,
                pid=os.getpid())


def jobs_of(rows, mode):
    return [(r,t,mode) for r in rows for t in kernel_of(r).moves(kernel_of(r).root,0)]


def benchmark(jobs, workers):
    start = time.perf_counter()
    if workers == 0: results = list(map(solve_action,jobs))
    else:
        with ProcessPoolExecutor(max_workers=workers,mp_context=multiprocessing.get_context('spawn')) as pool:
            results = list(pool.map(solve_action,jobs,chunksize=1))
    return dict(wall_s=time.perf_counter()-start, worker_cpu_s=sum(r['cpu_s'] for r in results),
                nodes=sum(r['nodes'] for r in results),edges=sum(r['edges'] for r in results),
                compile_s=sum(r['compile_s'] for r in results),reduce_s=sum(r['reduction_s'] for r in results),
                packed_payload_bytes=sum(r['packed_payload_bytes'] for r in results),
                summed_worker_peak_rss_bytes=sum(max(r['worker_peak_rss_bytes'] for r in results if r['pid']==pid)
                    for pid in {r['pid'] for r in results}),
                serialized_job_bytes=sum(len(pickle.dumps(j)) for j in jobs),
                vectors={str(seed):{str(r['action']):r['value'] for r in results if r['seed']==seed}
                    for seed in sorted({r['seed'] for r in results})})


def main():
    p=argparse.ArgumentParser(); p.add_argument('--out',type=Path,required=True)
    p.add_argument('--start',type=int,default=930000);p.add_argument('--count',type=int,default=96)
    p.add_argument('--worlds',type=int,default=40);p.add_argument('--rounds',type=int,default=3)
    p.add_argument('--min-ply',type=int,default=12);p.add_argument('--ply-span',type=int,default=8)
    args=p.parse_args(); args.out.mkdir(parents=True,exist_ok=False)
    tick=time.perf_counter()
    rows=[fixture(args.start+i,args.min_ply+i%args.ply_span,args.worlds) for i in range(args.count)]
    generation_s=time.perf_counter()-tick
    ready=[r for r in rows if r['status']=='ready']
    plan=dict(arguments=vars(args)|{'out':str(args.out)},rows=rows,generation_s=generation_s,
        source_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        baseline_sha256=hashlib.sha256((SOURCE/'phase3/compiled_tape.py').read_bytes()).hexdigest(),
        cpu_count=os.cpu_count(),python=sys.version,
        caveat='Uniform support; finite frozen Dice field, no higher-k or phone-player claim. Cold worker startup, serialization and teardown included. macOS ru_maxrss is bytes; sum of peaks is an upper bound, not simultaneous RSS.')
    (args.out/'plan.json').write_text(json.dumps(plan,indent=2)+'\n')
    # Independent original recursive evaluator, with exact same finite scenarios.
    tick=time.perf_counter(); expected={str(r['seed']):{str(k):v for k,v in kernel_of(r).reference(r['tape'],30)[0].items()} for r in ready}
    reference_s=time.perf_counter()-tick
    # Duplicating each (world,tape) scenario 3 times must triple every value.
    duplicated=[]
    for r in ready[:12]:
        d=dict(r,worlds=[w for w in r['worlds'] for _ in range(3)],tape=[t for t in r['tape'] for _ in range(3)])
        values=kernel_of(d).reference(d['tape'],30)[0]
        assert {str(k):v for k,v in values.items()} == {k:3*v for k,v in expected[str(r['seed'])].items()}
        for job in jobs_of([d],'compiled'):
            rr=solve_action(job);assert rr['value']==values[rr['action']]
        duplicated.append(r['seed'])
    timings=[]; rng=random.Random(704210499)
    for repeat in range(args.rounds):
        order=[('recursive',0),('recursive',4),('compiled',0),('compiled',4)];rng.shuffle(order)
        for mode, workers in order:
            result=benchmark(jobs_of(ready,mode),workers)
            assert result['vectors']==expected
            result.update(round=repeat,mode=mode,workers=workers)
            timings.append(result)
            (args.out/'timings.json').write_text(json.dumps(timings,indent=2)+'\n')
            print(json.dumps({k:result[k] for k in ('round','mode','workers','wall_s','worker_cpu_s','nodes')}),flush=True)
    summary=dict(planned=len(rows),ready=len(ready),statuses={s:sum(r['status']==s for r in rows) for s in sorted({r['status'] for r in rows})},
        partial_trick_roots=sum(r['ply']%4!=0 for r in ready),generation_s=generation_s,original_reference_s=reference_s,
        duplicate_weight_cases=duplicated,action_vectors_checked=len(expected),all_values_equal=True,
        timing_rows=[{k:v for k,v in r.items() if k!='vectors'} for r in timings])
    (args.out/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')


if __name__=='__main__': main()
