#!/usr/bin/env python3
"""Bulk (root,world,shared-plan) rollouts using recovered pip-trump mechanics.

Avoid the broken archived sampler. Inputs are independently legally replayed
finite fixtures. uint64 high multiplication exactly matches compiled_tape.pick.
"""
import argparse, hashlib, itertools, json, resource, sys, time
from pathlib import Path
from shared_orders import evaluate, kernel_of

HERE=Path(__file__).resolve().parent
SOURCE=HERE.parent/'astra-sol-20261004/phase2/incoming/drive'
sys.path.insert(0,str(SOURCE))
from engine42 import Game, Rules, popcount, kth_set_bit, first_in_order
import numpy as np

def run(rows):
    start=time.perf_counter();groups={};answers={};peak_arrays=0;total_rows=0;row_steps=0
    for row in rows:groups.setdefault((row['request']['decl'],row['request']['bidder']),[]).append(row)
    preparation_s=0;simulation_s=0;reduction_s=0
    for (decl,bidder),batch in sorted(groups.items()):
        tick=time.perf_counter();hands=[];orders=[];tapes=[];focals=[];scores0=[];scores1=[];leaders=[];tables=[];tricks=[];bounds=[]
        at=0
        for row in batch:
            kernel=kernel_of(row);own=[t for t in range(28) if row['worlds'][0][kernel.focal]>>t&1]
            plans=list(itertools.permutations(own));P=len(plans);N=kernel.n;R=N*P
            hands.append(np.repeat(np.array(row['worlds'],np.int64),P,axis=0))
            orders.append(np.tile(np.array(plans,np.int64),(N,1)))
            tape=np.array([list(t)+[0]*(16-len(t)) for t in row['tape']],np.uint64)
            tapes.append(np.repeat(tape,P,axis=0))
            focals.extend([kernel.focal]*R);scores0.extend([row['points'][bidder%2]]*R);scores1.extend([row['points'][1-bidder%2]]*R)
            leaders.extend([row['leader']]*R)
            table=[t for s,t in row['trick']]+[-1]*(4-len(row['trick']))
            tables.extend([table]*R);tricks.extend([row['ply']//4]*R)
            legal=kernel.moves(kernel.root,0);root=[next(t for t in plan if t in legal) for plan in plans]
            bounds.append((row['seed'],at,at+R,N,P,root,legal));at+=R
        hands=np.concatenate(hands);orders=np.concatenate(orders);tapes=np.concatenate(tapes);focals=np.array(focals)
        g=Game(Rules(decl),hands,30,bidder,scores=(np.array(scores0),np.array(scores1)),
               leader=np.array(leaders),table=np.array(tables),trick=np.array(tricks))
        preparation_s+=time.perf_counter()-tick
        tick=time.perf_counter();depth=0
        while not g.done.all():
            assert depth<16
            seat=g.seat_to_play();legal=g.legal(seat);n=popcount(legal).astype(np.uint64)
            u=tapes[:,depth]
            k=(((u>>np.uint64(32))*n+(((u&np.uint64(0xffffffff))*n)>>np.uint64(32)))>>np.uint64(32)).astype(np.int64)
            tile=kth_set_bit(legal,k)
            mine=(seat==focals)&~g.done
            tile=np.where(mine,first_in_order(legal,orders),tile)
            row_steps+=int((~g.done).sum());g.play(np.where(g.done,-1,tile));depth+=1
        simulation_s+=time.perf_counter()-tick
        tick=time.perf_counter();outcome=np.where(focals%2==bidder%2,g.made(),1-g.made())
        for seed,a,b,N,P,root,legal in bounds:
            table=outcome[a:b].reshape(N,P);sums=table.sum(0)
            answers[str(seed)]=dict(values={str(t):int(max(sums[p] for p,r in enumerate(root) if r==t)) for t in legal},
                table=table.astype(np.uint8).ravel().tolist())
        reduction_s+=time.perf_counter()-tick
        # Resident NumPy arrays and retained engine trace arrays, no Python object census.
        arrays=[hands,orders,tapes,focals,g.hands,g.bid_pts,g.def_pts,g.leader,g.table,g.npl,g.trick,g.done,outcome]
        arrays.extend(a for pair in g.ply_log for a in pair)
        peak_arrays=max(peak_arrays,sum(a.nbytes for a in arrays));total_rows+=at
    return dict(answers=answers,wall_s=time.perf_counter()-start,preparation_s=preparation_s,
        simulation_s=simulation_s,reduction_s=reduction_s,world_plan_rows=total_rows,active_row_steps=row_steps,
        peak_counted_array_bytes=peak_arrays,process_peak_rss_bytes=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,
        group_count=len(groups))

def main():
    p=argparse.ArgumentParser();p.add_argument('--out',type=Path,required=True);args=p.parse_args();args.out.mkdir(parents=True,exist_ok=False)
    allrows=[r for r in json.loads((HERE/'results/parallel-panel/plan.json').read_text())['rows'] if r['status']=='ready']
    rows=[r for r in allrows if r['request']['decl']<=6]
    expected={str(r['seed']):evaluate(r,keep_table=True) for r in rows}
    reports=[]
    for repeat in range(3):
        result=run(rows)
        for seed,r in expected.items():
            assert result['answers'][seed]['values']=={str(k):v for k,v in r['restricted'].items()}
            assert result['answers'][seed]['table']==r['payoff_table']
        result.pop('answers');reports.append(result)
    report=dict(planned=len(allrows),pip_roots=len(rows),excluded_unsupported_declarations=len(allrows)-len(rows),
        source_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        engine_sha256=hashlib.sha256((SOURCE/'engine42.py').read_bytes()).hexdigest(),numpy=np.__version__,
        scalar_plan_s=sum(r['plan_total_s'] for r in expected.values()),grouped_reference_s=sum(r['lawful_s'] for r in expected.values()),
        values_and_full_tables_equal=True,runs=reports,
        caveat='Pip trumps only; source engine supports0..6. Direct independently validated states bypass consistent_worlds. Cold data-array preparation included; interpreter/NumPy import and common fixture generation excluded. No GPU or native-Walt comparison.')
    (args.out/'summary.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report,indent=2))
if __name__=='__main__':main()
