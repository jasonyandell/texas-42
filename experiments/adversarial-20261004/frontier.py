#!/usr/bin/env python3
"""Ply-synchronous full own-choice tree; shared-history backward fold.

Exact conditional finite-tape objective. Collision-free history IDs are built
from (previous history ID,tile) at EACH depth; initial IDs include root identity.
Every root keeps its complete scenario bundle at each shared focal MAX.
"""
import argparse, hashlib, json, resource, time
from pathlib import Path
from parallel_roots import kernel_of
from vector_orders import Game,Rules,popcount,kth_set_bit,np

HERE=Path(__file__).resolve().parent

def run(rows):
    assert len({r['seed'] for r in rows})==len(rows),'Each output root needs a unique ID'
    assert all(r['request']['bid']==30 and 0<=r['request']['decl']<=6 for r in rows),'Prototype supports bid30 and pip trumps only'
    start=time.perf_counter();groups={};answers={};peak_rows=0;incidences=0;peak_array=0;grouped_coords=0
    prep_s=0;forward_s=0;fold_s=0
    for row in rows:groups.setdefault((row['request']['decl'],row['request']['bidder']),[]).append(row)
    for (decl,bidder),batch in sorted(groups.items()):
        tick=time.perf_counter();hands=[];tapes=[];focals=[];scores0=[];scores1=[];leaders=[];tables=[];tricks=[];hists=[]
        for rootid,row in enumerate(batch):
            kernel=kernel_of(row);N=kernel.n
            hands.extend(row['worlds']);tapes.extend([list(t)+[0]*(16-len(t)) for t in row['tape']])
            focals.extend([kernel.focal]*N);scores0.extend([row['points'][bidder%2]]*N);scores1.extend([row['points'][1-bidder%2]]*N)
            leaders.extend([row['leader']]*N);tables.extend([[t for s,t in row['trick']]+[-1]*(4-len(row['trick']))]*N)
            tricks.extend([row['ply']//4]*N);hists.extend([rootid]*N)
        tape=np.array(tapes,np.uint64);focal=np.array(focals);history=np.array(hists)
        g=Game(Rules(decl),np.array(hands,np.int64),30,bidder,scores=(np.array(scores0),np.array(scores1)),
            leader=np.array(leaders),table=np.array(tables),trick=np.array(tricks))
        widx=np.arange(g.R);levels=[];depth=0
        prep_s+=time.perf_counter()-tick;tick=time.perf_counter()
        while not g.done.all():
            assert depth<16
            parentR=g.R;seat=g.seat_to_play();L=g.legal(seat);mine=(seat==focal)&~g.done
            n=np.where(mine,popcount(L),1);idx=np.repeat(np.arange(parentR),n)
            if len(idx)>1500000 or time.perf_counter()-start>220:raise TimeoutError('frontier row/time cap')
            j=np.arange(len(idx))-np.repeat(np.cumsum(n)-n,n)
            parenthist=history[idx];L=L[idx];mine=mine[idx];widx=widx[idx];focal=focal[idx]
            old=g
            g=Game(old.rules,old.hands[idx],30,bidder,scores=(old.bid_pts[idx],old.def_pts[idx]),
                leader=old.leader[idx],table=old.table[idx],trick=old.trick[idx]);g.done=old.done[idx].copy()
            no=popcount(L).astype(np.uint64);u=tape[widx,depth]
            k=(((u>>np.uint64(32))*no+(((u&np.uint64(0xffffffff))*no)>>np.uint64(32)))>>np.uint64(32)).astype(np.int64)
            tile=np.where(mine,kth_set_bit(L,j),kth_set_bit(L,k));tile=np.where(g.done,-1,tile)
            # Reindex full prefix equivalence, never hash or truncate base29 histories.
            keys,history=np.unique(np.stack((parenthist,tile),axis=1),axis=0,return_inverse=True)
            levels.append((idx,parenthist,mine,tile,parentR))
            g.play(tile);depth+=1
            peak_rows=max(peak_rows,g.R);incidences+=g.R;grouped_coords+=len(keys)
        forward_s+=time.perf_counter()-tick;tick=time.perf_counter()
        val=np.where(focal%2==bidder%2,g.made(),1-g.made()).astype(np.int64)
        for li in range(len(levels)-1,-1,-1):
            idx,parenthist,mine,tile,parentR=levels[li]
            group,gid=np.unique(parenthist[mine],return_inverse=True)
            sums=np.zeros((len(group),28),np.int64);seen=np.zeros((len(group),28),bool)
            np.add.at(sums,(gid,tile[mine]),val[mine]);seen[gid,tile[mine]]=True
            if li==0:
                assert mine.all() and group.tolist()==list(range(len(batch)))
                for rootid,row in enumerate(batch):
                    answers[str(row['seed'])]={str(t):int(sums[rootid,t]) for t in np.nonzero(seen[rootid])[0]}
                break
            best=np.where(seen,sums,-1).argmax(1)
            keep=tile[mine]==best[gid]
            nextval=np.zeros(parentR,np.int64)
            # Nonfocal nature is deterministic per frozen scenario: one successor.
            nextval[idx[~mine]]=val[~mine]
            nextval[idx[mine][keep]]=val[mine][keep]
            val=nextval
        fold_s+=time.perf_counter()-tick
        arrays=[tape,focal,history,g.hands,g.bid_pts,g.def_pts,g.table,g.leader,g.trick,g.done,widx,val]
        arrays.extend(a for level in levels for a in level[:-1])
        peak_array=max(peak_array,sum(a.nbytes for a in arrays))
    return dict(answers=answers,wall_s=time.perf_counter()-start,preparation_s=prep_s,forward_s=forward_s,fold_s=fold_s,
        peak_scenario_rows=peak_rows,total_scenario_ply_rows=incidences,grouped_coordinates_by_layer=grouped_coords,
        peak_counted_array_bytes=peak_array,process_peak_rss_bytes=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss)

def main():
    p=argparse.ArgumentParser();p.add_argument('--out',type=Path,required=True);p.add_argument('--plan',type=Path,default=HERE/'results/parallel-panel/plan.json')
    args=p.parse_args();args.out.mkdir(parents=True,exist_ok=False)
    source=json.loads(args.plan.read_text());rows=[r for r in source['rows'] if r['status']=='ready' and r['request']['decl']<=6]
    # Same interpreter, same scenario bundles; reference evaluation order alternates.
    timings=[];expected=None
    for repeat in range(4):
        def reference():
            t=time.perf_counter();answer={str(r['seed']):{str(a):v for a,v in kernel_of(r).reference(r['tape'],30)[0].items()} for r in rows}
            return answer,time.perf_counter()-t
        if repeat%2:
            result=run(rows);ref,ref_s=reference()
        else:
            ref,ref_s=reference();result=run(rows)
        assert result['answers']==ref
        if expected is not None:assert ref==expected
        expected=ref;result.pop('answers');result.update(reference_s=ref_s,repeat=repeat);timings.append(result)
    summary=dict(roots=len(rows),partial_roots=sum(r['ply']%4!=0 for r in rows),
        all_full_action_vectors_equal=True,runs=timings,
        source_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        input_plan_sha256=hashlib.sha256(args.plan.read_bytes()).hexdigest(),
        scope='Pip trumps0..6, bid30, same full-history finite Dice bundle. Full own-choice tree; no restricted ordering class. Input preparation included; common fixture generation/import excluded. Sorting group IDs+branch materialization fully charged. Absorbed terminal rows remain until group completion. No higher-k or phone-strength claim.')
    (args.out/'summary.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps(summary,indent=2))
if __name__=='__main__':main()
