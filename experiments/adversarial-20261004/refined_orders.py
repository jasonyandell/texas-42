#!/usr/bin/env python3
"""Nested shared-plan family, conditioned on public partner-currently-winning.

Base static plans are included. Add three adjacent-priority swaps per base
ordering, selected only when the current partial trick is won by the partner.
Plans are actual (base tuple, alternative tuple) keys shared across worlds.
"""
import argparse, hashlib, itertools, json, resource, sys, time
from pathlib import Path
from shared_orders import kernel_of, settled, pick
from rules import winner
from vector_orders import Game, Rules, popcount, kth_set_bit, first_in_order, np

HERE=Path(__file__).resolve().parent

def plans_for(own):
    plans=[]
    for order in itertools.permutations(own):
        plans.append((order,order))
        for i in range(len(order)-1):
            alt=list(order);alt[i],alt[i+1]=alt[i+1],alt[i]
            plans.append((order,tuple(alt)))
    assert len(set(plans))==len(plans)
    return plans

def partner_winning(kernel,state):
    return bool(state.trick) and winner(state.trick,kernel.decl)==(kernel.focal+2)%4

def scalar(row,plans):
    kernel=kernel_of(row);table=[]
    for w in range(kernel.n):
        for base,alt in plans:
            state=kernel.root;depth=0
            while not settled(kernel,state):
                legal=kernel.moves(state,w);actor=(state.leader+len(state.trick))%4
                if actor==kernel.focal:
                    order=alt if partner_winning(kernel,state) else base
                    tile=next(t for t in order if t in legal)
                else:tile=pick(row['tape'],w,depth,legal)
                state=kernel.after(state,tile);depth+=1
            table.append(int(kernel.success(state.points,30)))
    return table

def run(rows,retain=False):
    start=time.perf_counter();groups={};answers={};peak_arrays=0;total_rows=0;row_steps=0
    for row in rows:groups.setdefault((row['request']['decl'],row['request']['bidder']),[]).append(row)
    preparation_s=0;simulation_s=0;reduction_s=0
    for (decl,bidder),batch in sorted(groups.items()):
        tick=time.perf_counter();hands=[];orders=[];alternatives=[];tapes=[];focals=[];scores0=[];scores1=[];leaders=[];tables=[];tricks=[];bounds=[]
        at=0
        for row in batch:
            kernel=kernel_of(row);own=[t for t in range(28) if row['worlds'][0][kernel.focal]>>t&1]
            plans=plans_for(own);P=len(plans);N=kernel.n;R=N*P
            hands.append(np.repeat(np.array(row['worlds'],np.int64),P,axis=0))
            orders.append(np.tile(np.array([p[0] for p in plans],np.int64),(N,1)))
            alternatives.append(np.tile(np.array([p[1] for p in plans],np.int64),(N,1)))
            tape=np.array([list(t)+[0]*(16-len(t)) for t in row['tape']],np.uint64)
            tapes.append(np.repeat(tape,P,axis=0))
            focals.extend([kernel.focal]*R);scores0.extend([row['points'][bidder%2]]*R);scores1.extend([row['points'][1-bidder%2]]*R)
            leaders.extend([row['leader']]*R)
            table=[t for s,t in row['trick']]+[-1]*(4-len(row['trick']))
            tables.extend([table]*R);tricks.extend([row['ply']//4]*R)
            legal=kernel.moves(kernel.root,0);bit=partner_winning(kernel,kernel.root)
            root=[next(t for t in pair[int(bit)] if t in legal) for pair in plans]
            bounds.append((row,at,at+R,N,P,root,legal,plans));at+=R
        hands=np.concatenate(hands);orders=np.concatenate(orders);alternatives=np.concatenate(alternatives);tapes=np.concatenate(tapes);focals=np.array(focals)
        rules=Rules(decl)
        g=Game(rules,hands,30,bidder,scores=(np.array(scores0),np.array(scores1)),
               leader=np.array(leaders),table=np.array(tables),trick=np.array(tricks))
        preparation_s+=time.perf_counter()-tick
        tick=time.perf_counter();depth=0
        while not g.done.all():
            assert depth<16
            seat=g.seat_to_play();legal=g.legal(seat);n=popcount(legal).astype(np.uint64)
            u=tapes[:,depth]
            k=(((u>>np.uint64(32))*n+(((u&np.uint64(0xffffffff))*n)>>np.uint64(32)))>>np.uint64(32)).astype(np.int64)
            tile=kth_set_bit(legal,k)
            q=rules.led_code[np.maximum(g.table[:,0],0)]
            strengths=np.where(g.table>=0,rules.strength[q[:,None],np.maximum(g.table,0)],-1)
            winning_seat=(g.leader+strengths.argmax(1))%4
            bit=(g.npl>0)&(winning_seat==(focals+2)%4)
            mine=(seat==focals)&~g.done
            own_tile=np.where(bit,first_in_order(legal,alternatives),first_in_order(legal,orders))
            tile=np.where(mine,own_tile,tile)
            row_steps+=int((~g.done).sum());g.play(np.where(g.done,-1,tile));depth+=1
        simulation_s+=time.perf_counter()-tick
        tick=time.perf_counter();outcome=np.where(focals%2==bidder%2,g.made(),1-g.made())
        for row,a,b,N,P,root,legal,plans in bounds:
            table=outcome[a:b].reshape(N,P);sums=table.sum(0)
            result=dict(values={str(t):int(max(sums[p] for p,r in enumerate(root) if r==t)) for t in legal},plans=P)
            if retain:result.update(table=table.astype(np.uint8).ravel().tolist(),plan_pairs=plans)
            answers[str(row['seed'])]=result
        reduction_s+=time.perf_counter()-tick
        arrays=[hands,orders,alternatives,tapes,focals,g.hands,g.bid_pts,g.def_pts,g.leader,g.table,g.npl,g.trick,g.done,outcome]
        arrays.extend(a for pair in g.ply_log for a in pair)
        peak_arrays=max(peak_arrays,sum(a.nbytes for a in arrays));total_rows+=at
    return dict(answers=answers,wall_s=time.perf_counter()-start,preparation_s=preparation_s,
        simulation_s=simulation_s,reduction_s=reduction_s,world_plan_rows=total_rows,active_row_steps=row_steps,
        peak_counted_array_bytes=peak_arrays,process_peak_rss_bytes=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,
        group_count=len(groups))

def main():
    p=argparse.ArgumentParser();p.add_argument('--out',type=Path,required=True);args=p.parse_args();args.out.mkdir(parents=True,exist_ok=False)
    rows=[r for r in json.loads((HERE/'results/parallel-panel/plan.json').read_text())['rows'] if r['status']=='ready' and r['request']['decl']<=6]
    base={str(r['seed']):r for r in json.loads((HERE/'results/shared-orders/results.json').read_text())}
    result=run(rows,retain=True);answers=result.pop('answers');checks=[]
    # Full scalar matrix on two preselected roots; 12 deterministic plan columns
    # on every other root. No outcome-dependent audit case selection.
    for i,row in enumerate(rows):
        got=answers[str(row['seed'])];plans=got['plan_pairs'];P=len(plans)
        indices=list(range(P)) if i<2 else list(range(0,P,8))
        expected=scalar(row,[plans[p] for p in indices])
        actual=[got['table'][w*P+p] for w in range(40) for p in indices]
        assert actual==expected
        checks.append(dict(seed=row['seed'],cells=len(actual)))
    comparisons=[]
    for row in rows:
        seed=str(row['seed']);rr=answers[seed]['values'];bb=base[seed]['restricted'];qq=base[seed]['lawful']
        assert all(bb[a]<=rr[a]<=qq[a] for a in rr)
        chosen=max(rr,key=lambda a:(rr[a],-int(a)))
        comparisons.append(dict(seed=row['seed'],base=bb,refined=rr,lawful=qq,
            base_gap=max(qq.values())-max(bb.values()),refined_gap=max(qq.values())-max(rr.values()),
            base_regret=base[seed]['action_regret'],refined_regret=max(qq.values())-qq[chosen],refined_choice=int(chosen)))
    runs=[result]
    for _ in range(2):
        r=run(rows);a=r.pop('answers');assert all(a[s]['values']==answers[s]['values'] for s in a);runs.append(r)
    summary=dict(roots=len(rows),worlds=40,plans_per_root=96,
        base_value_gap=sum(r['base_gap'] for r in comparisons),refined_value_gap=sum(r['refined_gap'] for r in comparisons),
        base_action_regret=sum(r['base_regret'] for r in comparisons),refined_action_regret=sum(r['refined_regret'] for r in comparisons),
        roots_value_improved=sum(r['refined_gap']<r['base_gap'] for r in comparisons),
        base_gap_roots=sum(r['base_gap']>0 for r in comparisons),refined_gap_roots=sum(r['refined_gap']>0 for r in comparisons),
        root_regret_improved=sum(r['refined_regret']<r['base_regret'] for r in comparisons),
        root_regret_worsened=sum(r['refined_regret']>r['base_regret'] for r in comparisons),
        scalar_cells_checked=sum(c['cells'] for c in checks),all_scalar_cells_equal=True,runs=runs,
        source_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        caveat='Nested policy class ensures nondecreasing objective, not monotone rootaction regret. Public partner-winning bit only. Same development roots used to select next hypothesis; independent holdout needed before a strength claim. Python/NumPy CPU timings, preprocessing included, imports/fixture generation excluded. First run retains matrix output; latter two timings omit that extra serialization.')
    (args.out/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
    (args.out/'comparisons.json').write_text(json.dumps(comparisons,indent=2)+'\n')
    (args.out/'scalar-checks.json').write_text(json.dumps(checks,indent=2)+'\n')
    print(json.dumps(summary,indent=2))
if __name__=='__main__':main()
