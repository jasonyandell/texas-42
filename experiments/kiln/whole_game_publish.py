#!/usr/bin/env python3
"""Publish witnessed whole-game contrasts and immutable sample-prefix history.

This reads completed evidence only. Eight-world best moves are descriptive
maxima over noisy estimates, never demonstrated regrets or policy gains.
"""
import argparse
from collections import Counter, defaultdict
from fractions import Fraction as F
import json
from pathlib import Path
import sqlite3
import zlib

import whole_game as w


def grouped(data):
    groups=defaultdict(lambda: defaultdict(list))
    for row in data:groups[row['root']][row['alternative']].append(row)
    for actions in groups.values():
        for rows in actions.values():rows.sort(key=lambda r:r['sample'])
    return groups


def summarize(directory):
    data=w.rows(directory);groups=grouped(data);stages=[]
    for size in range(7,1,-1):
        roots={r['root'] for r in data if r['size']==size}
        rows=[r for r in data if r['size']==size]
        stages.append(dict(size=size,roots=len(roots),action_world_rows=len(rows),
            helpful=sum(r['delta']==1 for r in rows),harmful=sum(r['delta']==-1 for r in rows),
            tied=sum(r['delta']==0 for r in rows),
            roots_with_any_action_flip=sum(any(r['delta'] for a in groups[rid].values() for r in a) for rid in roots),
            roots_with_apparent_better_mean=sum(max(sum(r['delta'] for r in a) for a in groups[rid].values())>0 for rid in roots)))
    audit=Counter();supports=[]
    for _,_,value,_ in w.receipts(directory):
        audit.update(value['independent_audit']);supports.append(int(value['support_worlds']))
    return dict(roots=len(groups),stages=stages,aggregate_audit=dict(audit),
                compatible_worlds=dict(minimum=min(supports),maximum=max(supports)))


def saved_world(db,root,index,baseline,alternative):
    raw=db.execute('SELECT payload,sha256 FROM jobs WHERE root=? AND world_index=?',(root,index)).fetchone()
    value=json.loads(zlib.decompress(raw[0]))
    traces=[t for t in value['traces'] if t['action'] in (baseline,alternative)]
    return dict(sample=index,sample_seed=value['sample_seed'],hands=value['worlds'][0],
                traces=traces,receipt_sha256=raw[1])


def witnesses(source,fit):
    rows=w.read(source/'analysis/training-rows.json')
    roots={r['id']:r for r in w.read(source/'train/panel.json')['roots']}
    db=sqlite3.connect(source/'train/contrasts.sqlite');answer=[]
    for index,contrast in enumerate(fit['contrasts']):
        row=rows[contrast['left_row']];root=roots[row['root']]
        worlds=[saved_world(db,row['root'],i,row['baseline'],row['alternative']) for i in contrast['samples']]
        # Include the complete forced branches. Decision indices address the
        # original complete response table in the hash-identified receipt.
        answer.append(dict(contrast=contrast,root=root,worlds=worlds,
            nominated_selected=[q['name'] for q in fit['selected'] if q['witness']['contrast']==index]))
    db.close();return dict(schema='whole-game-contrast-witnesses-v1',pairs=answer)


def refinement(source):
    directory=source/'train';data=w.rows(directory,24);groups=grouped(data)
    selected=w.read(directory/'refinement.json')['roots']
    roots={r['id']:r for r in w.read(directory/'panel.json')['roots']}
    original=w.read(source/'initial-receipt-hashes.json');db=sqlite3.connect(directory/'contrasts.sqlite')
    current=dict(db.execute('SELECT id,sha256 FROM jobs WHERE world_index<8'))
    assert original==current and len(current)==960
    cases=[]
    for rid in selected:
        actions=groups[rid];base=roots[rid]['baseline']
        totals={a:{n:sum(r['delta'] for r in rows[:n]) for n in (8,24)} for a,rows in actions.items()}
        # Freeze the originally preferred alternative before looking at the
        # added samples; do not reselect a flattering candidate at depth 24.
        old_alt=min(actions,key=lambda a:(-totals[a][8],a))
        best=lambda n:base if max(v[n] for v in totals.values())<=0 else min(actions,key=lambda a:(-totals[a][n],a))
        records=actions[old_alt];curve=[];total=0
        for row in records:
            total+=row['delta'];curve.append(dict(sample=row['sample'],delta=row['delta'],
                cumulative_delta=total,mean=str(F(total,row['sample']+1)),receipt_sha256=row['receipt_sha256']))
        additional=records[8:];example={}
        for sign,label in ((1,'help'),(-1,'harm')):
            match=next((r for r in additional if r['delta']==sign),None)
            if match:example[label]=saved_world(db,rid,match['sample'],base,old_alt)
        cases.append(dict(root=rid,deal=roots[rid]['deal_seed'],size=roots[rid]['own_remaining'],
            request=roots[rid]['request'],baseline=base,frozen_alternative=old_alt,
            apparent_best_8=best(8),apparent_best_24=best(24),
            frozen_delta_8=str(F(totals[old_alt][8],8)),frozen_delta_24=str(F(totals[old_alt][24],24)),
            added_counts=dict(help=sum(r['delta']==1 for r in additional),harm=sum(r['delta']==-1 for r in additional),
                              tie=sum(r['delta']==0 for r in additional)),
            actions={str(a):{str(n):str(F(v[n],n)) for n in (8,24)} for a,v in totals.items()},
            prefix_history=curve,new_world_examples=example))
    db.close()
    positive=[c for c in cases if F(c['frozen_delta_8'])>0]
    return dict(schema='whole-game-refinement-history-v1',preserved_initial_receipts=len(current),cases=cases,
        summary=dict(roots=len(cases),initial_positive=len(positive),
            initial_positive_still_positive=sum(F(c['frozen_delta_24'])>0 for c in positive),
            initial_positive_now_tied=sum(F(c['frozen_delta_24'])==0 for c in positive),
            initial_positive_now_negative=sum(F(c['frozen_delta_24'])<0 for c in positive),
            apparent_preferred_move_changed=sum(c['apparent_best_8']!=c['apparent_best_24'] for c in cases)))


def publish(source,destination):
    destination.mkdir(parents=True,exist_ok=True);fit=w.read(source/'fit.json')
    history=refinement(source)
    summary={split:summarize(source/split) for split in ('train','validation')}
    summary['refinement']=history['summary'];summary['preserved_initial_receipts']=history['preserved_initial_receipts']
    w.immutable_json(destination/'summary.json',summary)
    w.immutable_json(destination/'refinement.json',history)
    w.immutable_json(destination/'witnesses.json',witnesses(source,fit))
    for name in ('fit.json','result.json','train-audit.json','validation-audit.json','fresh-source-audit.json'):
        w.immutable_json(destination/name,w.read(source/name))
    for q in fit['selected']:
        path=destination/'queries'/(q['name']+'.scheme');path.parent.mkdir(exist_ok=True)
        if path.exists() and path.read_text()!=q['source']:raise ValueError('Published Scheme changed')
        path.write_text(q['source'])
    return summary


if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('source',type=Path);parser.add_argument('destination',type=Path)
    args=parser.parse_args();print(w.p.canonical(publish(args.source,args.destination)))
