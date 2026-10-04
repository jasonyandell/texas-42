#!/usr/bin/env python3
"""Shared priority plans: group by actual plan, sum scenarios, THEN optimize.

Correct finite objective on lawful support from parallel_roots; not a phone
player. Scalar oracle/cost baseline. All plans share each world's same tape.
"""
import argparse
from array import array
import hashlib
import itertools
import json
from pathlib import Path
import time
from parallel_roots import kernel_of, popcount, pick

HERE=Path(__file__).resolve().parent

def settled(kernel,state):
    b=kernel.request['bid'];return (state.points[kernel.bidder%2]>=b or
        state.points[1-kernel.bidder%2]>42-b or popcount(state.played)==28)

def rollout(kernel,tape,world,order):
    state=kernel.root;depth=0;trace=[]
    while not settled(kernel,state):
        legal=kernel.moves(state,world);actor=(state.leader+len(state.trick))%4
        tile=next(t for t in order if t in legal) if actor==kernel.focal else pick(tape,world,depth,legal)
        trace.append(tile);state=kernel.after(state,tile);depth+=1
    return int(kernel.success(state.points,kernel.request['bid'])),tuple(trace)

def tree_traces(kernel,tape,world):
    def rec(state,depth):
        if settled(kernel,state):return {()}
        legal=kernel.moves(state,world);actor=(state.leader+len(state.trick))%4
        actions=legal if actor==kernel.focal else [pick(tape,world,depth,legal)]
        return {(t,)+tail for t in actions for tail in rec(kernel.after(state,t),depth+1)}
    return rec(kernel.root,0)

def evaluate(row,keep_table=False,check_traces=False):
    kernel=kernel_of(row);tape=row['tape'];n=kernel.n;tick=time.perf_counter()
    own=[i for i in range(28) if row['worlds'][0][kernel.focal]>>i&1]
    plans=list(itertools.permutations(own));legal=kernel.moves(kernel.root,0)
    root=[next(t for t in plan if t in legal) for plan in plans]
    enumerate_s=time.perf_counter()-tick;P=len(plans);table=array('B',[0])*(n*P);visits=0
    tick=time.perf_counter();trace_checks=[]
    for w in range(n):
        realized=set()
        for p,plan in enumerate(plans):
            value,trace=rollout(kernel,tape,w,plan);table[w*P+p]=value;visits+=len(trace)
            if check_traces and w<2:realized.add(trace)
        if check_traces and w<2:
            tree=tree_traces(kernel,tape,w);assert realized==tree
            trace_checks.append(dict(world=w,orderings=P,distinct_traces=len(tree)))
    roll_s=time.perf_counter()-tick
    tick=time.perf_counter()
    sums=[sum(table[w*P+p] for w in range(n)) for p in range(P)]
    restricted={a:max(s for s,r in zip(sums,root) if r==a) for a in legal}
    fused={a:sum(max(table[w*P+p] for p,r in enumerate(root) if r==a) for w in range(n)) for a in legal}
    reduce_s=time.perf_counter()-tick
    tick=time.perf_counter();lawful,calls=kernel.reference(tape,kernel.request['bid']);lawful_s=time.perf_counter()-tick
    assert all(restricted[a]<=lawful[a]<=fused[a] for a in legal)
    chosen=max(legal,key=lambda a:(restricted[a],-a));true_chosen=max(legal,key=lambda a:(lawful[a],-a))
    result=dict(seed=row['seed'],ply=row['ply'],worlds=n,plans=P,rollout_rows=n*P,
        table_bytes=len(table),simulated_plies=visits,enumerate_s=enumerate_s,rollout_s=roll_s,
        reduction_s=reduce_s,plan_total_s=enumerate_s+roll_s+reduce_s,
        lawful_s=lawful_s,lawful_calls=calls,restricted=restricted,lawful=lawful,fused=fused,
        value_gap=max(lawful.values())-max(restricted.values()),
        chosen=chosen,lawful_chosen=true_chosen,action_regret=max(lawful.values())-lawful[chosen],
        trace_checks=trace_checks)
    if keep_table:result.update(plans_list=plans,root_by_plan=root,payoff_table=list(table),fixture=row)
    return result

def main():
    p=argparse.ArgumentParser();p.add_argument('--out',type=Path,required=True)
    p.add_argument('--limit',type=int,default=134);args=p.parse_args();args.out.mkdir(parents=True,exist_ok=False)
    rows=[r for r in json.loads((HERE/'results/parallel-panel/plan.json').read_text())['rows'] if r['status']=='ready'][:args.limit]
    records=[];witnesses=[];start=time.monotonic()
    (args.out/'plan.json').write_text(json.dumps(dict(seeds=[r['seed'] for r in rows],worlds=40,
        source_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        support='Uniform feasible support sampled after complete history legality replay; fixed uint64 Dice per world/ply.',
        scope='One shared ordering tuple across all worlds. Focal success count orientation makes all own optimizations MAX. Fixed bid30; no hidden-world plan index; no neural fit, phone strength, or GPU throughput inference.'),indent=2)+'\n')
    for i,row in enumerate(rows):
        if time.monotonic()-start>240:
            records.append(dict(seed=row['seed'],status='deadline_refused'));continue
        result=evaluate(row,check_traces=i<8)
        if (result['value_gap'] or result['action_regret']) and len(witnesses)<3:
            witness=evaluate(row,keep_table=True)
            name=f'witness-{row["seed"]}.json';(args.out/name).write_text(json.dumps(witness,indent=2)+'\n');witnesses.append(name)
        records.append(dict(result,status='completed'))
        (args.out/'results.json').write_text(json.dumps(records,indent=2)+'\n')
    complete=[r for r in records if r['status']=='completed']
    summary=dict(planned=len(rows),complete=len(complete),refused=len(records)-len(complete),
        vector_gap_cases=sum(r['restricted']!=r['lawful'] for r in complete),
        positive_value_gap_cases=sum(r['value_gap']>0 for r in complete),
        positive_action_regret_cases=sum(r['action_regret']>0 for r in complete),
        sum_value_gap_counts=sum(r['value_gap'] for r in complete),sum_action_regret_counts=sum(r['action_regret'] for r in complete),
        max_value_gap_counts=max(r['value_gap'] for r in complete),max_action_regret_counts=max(r['action_regret'] for r in complete),
        worlds=40,rollout_rows=sum(r['rollout_rows'] for r in complete),
        simulated_plies=sum(r['simulated_plies'] for r in complete),
        plan_total_s=sum(r['plan_total_s'] for r in complete),lawful_total_s=sum(r['lawful_s'] for r in complete),
        table_bytes_total=sum(r['table_bytes'] for r in complete),table_bytes_max=max(r['table_bytes'] for r in complete),
        trace_worlds_checked=sum(len(r['trace_checks']) for r in complete),witnesses=witnesses,elapsed_s=time.monotonic()-start)
    (args.out/'summary.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps(summary,indent=2))

if __name__=='__main__':main()
