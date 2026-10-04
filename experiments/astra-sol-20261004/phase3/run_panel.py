#!/usr/bin/env python3
"""Bounded tape-coordinate experiment; launch with run_capped.py <=290s."""
import argparse, hashlib, json, sys, time
from pathlib import Path
from compiled_tape import Kernel,make_tape,ROOT,popcount

def timed(fn):
    t=time.perf_counter_ns();value=fn();return value,(time.perf_counter_ns()-t)/1000

def main():
    p=argparse.ArgumentParser();p.add_argument('--out',type=Path,required=True)
    p.add_argument('--limit',type=int,default=18);p.add_argument('--worlds',type=int,default=40)
    p.add_argument('--tapes',type=int,default=4);p.add_argument('--start',type=int,default=0)
    args=p.parse_args();assert 1<=args.worlds<=40 and 1<=args.tapes<=32
    args.out.mkdir(parents=True,exist_ok=False)
    source=ROOT/'phase2/results/cost-panel-a';plan=json.loads((source/'plan.json').read_text())
    fixtures=[];seen=set()
    for row in plan['rows']:
        key=(row['request']['decl'],row['ply'])
        if row['eligible'] and row['ply'] in (16,20) and key not in seen:
            seen.add(key);fixtures.append(row)
    fixtures=fixtures[args.start:args.start+args.limit]
    receipt_plan=dict(schema='walt-lawful-tape-coordinate-plan-v1',fixtures=fixtures,
        world_count=args.worlds,tape_seeds=list(range(790001,790001+args.tapes)),
        bids=list(range(30,43)),compilation_node_cap=100000,compilation_seconds=20,
        panel_seconds=270,source_panel_sha256=hashlib.sha256((source/'plan.json').read_bytes()).hexdigest(),
        python=sys.version,source_hashes={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in (Path(__file__),Path(__file__).with_name('compiled_tape.py'))},
        semantics='Fixed 64-bit nature tapes and same sampled hidden worlds; lawful shared focal decisions. Not exact integration over hidden-world posterior or nature, and not production Walt field.')
    (args.out/'plan.json').write_text(json.dumps(receipt_plan,indent=2)+'\n')
    began=time.monotonic();receipts=[]
    for row in fixtures:
        if time.monotonic()-began>240:
            receipts.append(dict(id=row['id'],status='panel_deadline'));continue
        case_began=time.monotonic()
        request=row['request'];rotation=1 if request['bidder']%2==0 else 0
        stored=json.loads((source/f"case-{row['id']:04}.json").read_text())['result']['worlds'][:args.worlds]
        worlds=[[w[(seat+rotation)%4] for seat in range(4)] for w in stored]
        kernel=Kernel(request,worlds,row['points'],request['seat'])
        result=dict(id=row['id'],ply=row['ply'],decl=request['decl'],worlds=worlds,request=request,tapes=[])
        try:
            universal,compile_us=timed(lambda:kernel.compile())
            result.update(universal_compile_us=compile_us,universal_statistics=universal.statistics())
            used={};visits=0;support_visits=0
            for seed in receipt_plan['tape_seeds']:
                tape=make_tape(seed,kernel.n,kernel.plies)
                references,reference_us=timed(lambda:{b:kernel.reference(tape,b) for b in receipt_plan['bids']})
                active,route_us=timed(lambda:universal.route(tape))
                for i,mask in active:used[i]=used.get(i,0)|mask
                visits+=len(active);support_visits+=sum(popcount(mask) for _,mask in active)
                vectors,reduce_us=timed(lambda:{b:universal.reduce(active,b) for b in receipt_plan['bids']})
                adaptive,adaptive_compile_us=timed(lambda:kernel.compile(tape))
                adaptive_active,adaptive_route_us=timed(lambda:adaptive.route(tape))
                adaptive_vectors,adaptive_reduce_us=timed(lambda:{b:adaptive.reduce(adaptive_active,b) for b in receipt_plan['bids']})
                assert vectors==adaptive_vectors=={b:v[0] for b,v in references.items()}
                assert len(active)==len(adaptive.nodes)
                single_reference,single_reference_us=timed(lambda:kernel.reference(tape,30))
                single_universal,single_universal_us=timed(lambda:universal.reduce(active,30))
                single_adaptive,single_adaptive_us=timed(lambda:adaptive.reduce(adaptive_active,30))
                assert single_reference[0]==single_universal==single_adaptive==vectors[30]
                # Control the reference's early-stop optimization separately.
                full_reference,full_us=timed(lambda:kernel.reference(tape,30,False))
                assert full_reference[0]==vectors[30]
                record=dict(seed=seed,tape=tape,reference_13_bids_us=reference_us,
                    reference_calls={b:v[1] for b,v in references.items()},
                    universal_route_us=route_us,universal_reduce_13_bids_us=reduce_us,
                    adaptive_compile_us=adaptive_compile_us,adaptive_route_us=adaptive_route_us,
                    adaptive_reduce_13_bids_us=adaptive_reduce_us,adaptive_statistics=adaptive.statistics(),
                    reference_bid30_us=single_reference_us,universal_reduce_bid30_us=single_universal_us,
                    adaptive_reduce_bid30_us=single_adaptive_us,
                    full_reference_bid30_us=full_us,full_reference_bid30_calls=full_reference[1],
                    values=vectors,all_values_equal=True)
                result['tapes'].append(record)
            result['coverage']=dict(union_coordinates=len(used),union_support_incidences=sum(popcount(mask) for mask in used.values()),
                tape_coordinate_visits=visits,tape_support_visits=support_visits,
                unused_coordinates=len(universal.nodes)-len(used),
                depth_counts={d:dict(physical=sum(n.depth==d for n in universal.nodes),used=sum(universal.nodes[i].depth==d for i in used)) for d in range(kernel.plies+1)})
            result['status']='completed'
        except TimeoutError as e:result.update(status='compilation_refused',error=str(e))
        result['attempt_seconds']=time.monotonic()-case_began
        (args.out/f"case-{row['id']:04}.json").write_text(json.dumps(result,indent=2)+'\n');receipts.append(result)
    complete=[r for r in receipts if r['status']=='completed']
    pairs=[t for r in complete for t in r['tapes']]
    summary=dict(planned=len(fixtures),completed=len(complete),statuses={s:sum(r['status']==s for r in receipts) for s in {r['status'] for r in receipts}},
        source_cases=[r['id'] for r in complete],tapes=len(pairs),root_action_vectors_compared=len(pairs)*13,
        all_complete_vectors_equal=all(t['all_values_equal'] for t in pairs),elapsed_seconds=time.monotonic()-began,
        refused_attempt_seconds=sum(r.get('attempt_seconds',0) for r in receipts if r['status']!='completed'),
        universal_compile_us=sum(r['universal_compile_us'] for r in complete),
        universal_route_us=sum(t['universal_route_us'] for t in pairs),
        universal_reduce_13_bids_us=sum(t['universal_reduce_13_bids_us'] for t in pairs),
        reference_13_bids_us=sum(t['reference_13_bids_us'] for t in pairs),
        adaptive_compile_us=sum(t['adaptive_compile_us'] for t in pairs),
        adaptive_route_us=sum(t['adaptive_route_us'] for t in pairs),
        adaptive_reduce_13_bids_us=sum(t['adaptive_reduce_13_bids_us'] for t in pairs),
        reference_bid30_us=sum(t['reference_bid30_us'] for t in pairs),
        universal_reduce_bid30_us=sum(t['universal_reduce_bid30_us'] for t in pairs),
        adaptive_reduce_bid30_us=sum(t['adaptive_reduce_bid30_us'] for t in pairs),
        universal_nodes=sum(r['universal_statistics']['nodes'] for r in complete),
        union_used_coordinates=sum(r['coverage']['union_coordinates'] for r in complete),
        unused_coordinates=sum(r['coverage']['unused_coordinates'] for r in complete),
        union_used_support_incidences=sum(r['coverage']['union_support_incidences'] for r in complete),
        tape_coordinate_visits=sum(r['coverage']['tape_coordinate_visits'] for r in complete),
        universal_max_python_object_bytes=max((r['universal_statistics']['python_object_bytes'] for r in complete),default=0),
        note='Python research timing, each vector uses same frozen worlds and tape; no phone strength or speed claim. Includes compile costs explicitly; universal graph may hit node/time cap.')
    (args.out/'summary.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps(summary,indent=2))
if __name__=='__main__':main()
