#!/usr/bin/env python3
"""Derive relational descriptions from witnessed cross-world action contrasts.

No tile literal or game-stage feature appears in a learned Scheme. Hidden
relations describe sampled worlds for analysis; they are not live move guards.
"""
import argparse
from collections import defaultdict
from fractions import Fraction as F
from functools import lru_cache
from itertools import combinations
import json
from pathlib import Path
import random
import subprocess

import whole_game as w
from rules import TILES, context, called, follows
from scheme_mine import key


def geometry(row):
    req=row['request'];n=len(req['plays'])//2%4
    led=context(req['plays'][-2*n+1],req['decl']) if n else context(row['baseline'],req['decl'])
    return req['decl'],led,row['baseline'],row['alternative'],sum(1<<t for t in set(range(28))-set(req['plays'][1::2]))


@lru_cache(None)
def properties(tile,decl,live):
    hi,lo=TILES[tile];trump=called(tile,decl)
    boss=trump and not any(live&(1<<t) and key(t,'trump',decl)>key(tile,'trump',decl) for t in range(28))
    return ('trump' if trump else '!trump','double' if hi==lo else '!double',
            f'count={hi+lo if hi+lo in (5,10) else 0}','boss' if boss else '!boss')


@lru_cache(None)
def allowed(desc,geom):
    _,beats_base,beats_alt,prop=desc;decl,led,base,alt,live=geom
    return sum(1<<t for t in range(28) if live&(1<<t)
        and (beats_base is None or (key(t,led,decl)>key(base,led,decl))==beats_base)
        and (beats_alt is None or (key(t,led,decl)>key(alt,led,decl))==beats_alt)
        and (prop is None or prop in properties(t,decl,live)))


def matches(desc,row):
    holder=(row['request']['seat']+desc[0])%4
    return bool(sum(1<<t for t in row['hands'][holder]) & allowed(tuple(desc),geometry(row)))


def source(desc):
    holder,bb,ba,prop=desc;role={1:'after',2:'mate',3:'before'}[holder]
    atoms=['(viewer me)','(successor me after)','(partner me mate)','(successor mate before)',
           '(own-legal baseline)','(own-legal alternative)',f'(holds {role} hidden)']
    for name,truth in (('baseline',bb),('alternative',ba)):
        if truth is not None:
            atom=f'(beats hidden {name} led)';atoms.append(atom if truth else f'(not {atom})')
    if prop is not None:
        name=prop.lstrip('!')
        atom='(in hidden q*)' if name=='trump' else '(double hidden)' if name=='double' else \
            '(boss hidden q*)' if name=='boss' else f'(count hidden {name.split("=")[1]})'
        atoms.append(f'(not {atom})' if prop.startswith('!') else atom)
    cases=['(case '+' '.join(atoms+extra)+')' for extra in
           (['(led-context led)'],['(leader me)','(leads-context baseline led)'])]
    return '(fix\n (roles (chair me) (chair after) (chair mate) (chair before) '+\
        '(domino baseline) (domino alternative) (domino hidden) (context led))\n'+\
        ' (out alternative baseline)\n '+'\n '.join(cases)+')\n'


def contrast_pairs(data):
    groups=defaultdict(list)
    for i,row in enumerate(data):groups[row['root'],row['alternative']].append(i)
    pools=defaultdict(list)
    for (root,alt),indices in groups.items():
        for a,b in combinations(indices,2):
            if data[a]['delta']==data[b]['delta']:continue
            identity=w.digest(['world-contrast-v1',root,alt,data[a]['sample'],data[b]['sample']])
            pools[data[a]['size']].append((identity,a,b))
    return [(a,b) for size in range(7,1,-1) for _,a,b in sorted(pools[size])[:12]]


def library(data,pairs):
    found={};contrasts=[]
    for a,b in pairs:
        left,right=data[a],data[b]
        owners=[{t:s for s,h in enumerate(r['hands']) for t in h} for r in (left,right)]
        changed=[t for t in range(28) if owners[0][t]!=owners[1][t]]
        record=dict(left_row=a,right_row=b,root=left['root'],alternative=left['alternative'],
            samples=[left['sample'],right['sample']],deltas=[left['delta'],right['delta']],
            changed_holders=[dict(tile=t,before=owners[0][t],after=owners[1][t]) for t in changed])
        contrasts.append(record)
        for side,row,other in ((0,left,right),(1,right,left)):
            decl,led,base,alt,live=geometry(row)
            for tile in changed:
                holder=(owners[side][tile]-row['request']['seat'])%4
                assert holder in (1,2,3)
                bb=key(tile,led,decl)>key(base,led,decl);ba=key(tile,led,decl)>key(alt,led,decl)
                for bits in ((bb,None),(None,ba),(bb,ba)):
                    for prop in (None,*properties(tile,decl,live)):
                        desc=(holder,*bits,prop)
                        # A nominated description must actually separate this
                        # observed contrast; an existential second tile can mask it.
                        if matches(desc,other):continue
                        found.setdefault(desc,dict(contrast=len(contrasts)-1,side=side,tile=tile))
    return sorted(found,key=w.p.canonical),found,contrasts


def centered(data):
    groups=defaultdict(list)
    for i,row in enumerate(data):groups[row['root'],row['alternative']].append(i)
    values=[0]*len(data)
    for indices in groups.values():
        assert len(indices)==8 and sorted(data[i]['sample'] for i in indices)==list(range(8))
        total=sum(data[i]['delta'] for i in indices)
        for i in indices:values[i]=8*data[i]['delta']-total
    return values,groups


def statistics(data,mask,values,groups):
    selected=[i for i in range(len(data)) if mask>>i&1];n=len(selected)
    total=sum(values[i] for i in selected)
    variable=sum(0<sum(bool(mask>>i&1) for i in indices)<8 for indices in groups.values())
    other=[i for i in range(len(data)) if not(mask>>i&1)]
    return dict(matches=n,rows=len(data),deals=len({data[i]['deal'] for i in selected}),
        sizes=sorted({data[i]['size'] for i in selected}),variable_action_pairs=variable,
        residual_sum=total,residual_per_match=str(F(total,8*n)) if n else None,
        matched_delta=str(F(sum(data[i]['delta'] for i in selected),n)) if n else None,
        unmatched_delta=str(F(sum(data[i]['delta'] for i in other),len(other))) if other else None)


def native_audit(data,queries,output,tag):
    folder=output/tag;folder.mkdir(parents=True,exist_ok=True)
    inputs=[{k:row[k] for k in ('request','hands','baseline','alternative')} for row in data]
    serialized=''.join(w.p.canonical(r)+'\n' for r in inputs);checks=0
    for offset in range(0,len(queries),48):
        batch=queries[offset:offset+48];paths=[]
        for q in batch:
            path=folder/(q['name']+'.scheme');path.write_text(q['source']);paths.append(str(path))
        run=subprocess.run([str(w.AUDITOR),*paths],input=serialized,text=True,capture_output=True,check=True,timeout=120)
        lines=run.stdout.splitlines();assert len(lines)==len(data)
        for row,line in zip(data,lines):
            result=json.loads(line)
            if 'error' in result:raise ValueError(result['error'])
            expected=[matches(q['descriptor'],row) for q in batch]
            if result['memberships']!=expected:raise ValueError('Native/Python Scheme membership mismatch')
            checks+=len(batch)
    value=dict(rows=len(data),queries=len(queries),checks=checks,mismatches=0,
        input_sha256=w.digest(inputs),binary_sha256=w.p.digest(w.AUDITOR))
    w.immutable_json(folder/'audit.json',value);return value


def fit(directory,destination):
    panel=w.read(directory/'panel.json');assert panel['split']=='train'
    data=w.rows(directory);values,groups=centered(data);pairs=contrast_pairs(data)
    descriptors,witnesses,contrasts=library(data,pairs)
    candidates=[];masks={}
    for index,desc in enumerate(descriptors):
        mask=sum(1<<i for i,row in enumerate(data) if matches(desc,row));masks[index]=mask
        candidates.append(dict(name=f'contrast-{index}',descriptor=desc,source=source(desc),
            witness=witnesses[desc],statistics=statistics(data,mask,values,groups)))
    eligible=[i for i,q in enumerate(candidates) if q['statistics']['matches']>=64
        and q['statistics']['deals']>=6 and len(q['statistics']['sizes'])>=3
        and q['statistics']['variable_action_pairs']>=12 and q['statistics']['residual_sum']!=0]
    eligible.sort(key=lambda i:(-F(candidates[i]['statistics']['residual_sum']**2,candidates[i]['statistics']['matches']),
                                w.p.canonical(candidates[i]['descriptor'])))
    selected=[]
    for i in eligible:
        if any((masks[i]&masks[j]).bit_count()*5 >= (masks[i]|masks[j]).bit_count()*4 for j in selected):continue
        selected.append(i)
        if len(selected)==3:break
    queries=[dict(candidates[i],sign=1 if candidates[i]['statistics']['residual_sum']>0 else -1) for i in selected]
    output=destination.parent/'analysis';output.mkdir(parents=True,exist_ok=True)
    witness_rows=[data[i] for i in sorted({i for pair in pairs for i in pair})]
    audits=[native_audit(witness_rows,candidates,output,'library-audit'),native_audit(data,queries,output,'training-audit')]
    value=dict(schema='whole-game-contrast-fit-v1',protocol_sha256=w.p.digest(w.PROTOCOL),panel_id=panel['id'],
        implementation=w.read(directory/'implementation.json'),analysis_source_sha256=w.p.digest(Path(__file__)),
        data_sha256=w.digest(data),rows=len(data),contrasts=contrasts,candidates=candidates,
        eligible=len(eligible),selected=queries,audits=audits)
    value['id']=w.digest(value);w.immutable_json(destination,value)
    w.immutable_json(output/'training-rows.json',data)
    return dict(id=value['id'],rows=len(data),candidates=len(candidates),eligible=len(eligible),
        contrasts=len(contrasts),selected=queries,audits=audits)


def permutation_counts(data,queries,masks,values,count=1999):
    """Shuffle whole action-outcome vectors jointly across worlds at each root."""
    groups=defaultdict(lambda:defaultdict(dict))
    for i,row in enumerate(data):groups[row['root']][row['alternative']][row['sample']]=i
    matrices=[]
    for root,actions in sorted(groups.items()):
        matrix=[[[sum(int(masks[q]>>indices[s]&1)*values[indices[t]] for indices in actions.values())
                   for t in range(8)] for s in range(8)] for q in range(len(queries))]
        matrices.append(matrix)
    observed=[sum(v for i,v in enumerate(values) if masks[q]>>i&1)*query['sign'] for q,query in enumerate(queries)]
    extreme=[1]*len(queries);rng=random.Random(420654)
    for _ in range(count):
        sums=[0]*len(queries)
        for matrix in matrices:
            order=list(range(8));rng.shuffle(order)
            for q in range(len(queries)):sums[q]+=sum(matrix[q][s][t] for s,t in enumerate(order))
        for q,query in enumerate(queries):extreme[q]+=query['sign']*sums[q]>=observed[q]
    return [str(F(n,count+1)) for n in extreme]


def evaluate(directory,fit_path,destination):
    fit_value=w.read(fit_path);assert fit_value['id']==w.digest({k:v for k,v in fit_value.items() if k!='id'})
    assert w.read(directory/'exam-fit.json')['id']==fit_value['id']
    assert w.read(directory/'implementation.json')==fit_value['implementation']
    assert w.read(directory/'panel.json')['split']=='fresh'
    data=w.rows(directory);values,groups=centered(data);queries=fit_value['selected']
    masks=[sum(1<<i for i,row in enumerate(data) if matches(q['descriptor'],row)) for q in queries]
    pvalues=permutation_counts(data,queries,masks,values)
    results=[]
    for q,mask,pvalue in zip(queries,masks,pvalues):
        stats=statistics(data,mask,values,groups);stages=[]
        for size in range(7,1,-1):
            indices=[i for i,r in enumerate(data) if r['size']==size and mask>>i&1]
            stages.append(dict(size=size,matches=len(indices),residual_per_match=
                str(F(sum(values[i] for i in indices),8*len(indices))) if indices else None))
        qualified=bool(stats['residual_per_match'] and q['sign']*F(stats['residual_per_match'])>=F(1,20)
            and stats['deals']>=5 and len(stats['sizes'])>=3 and F(pvalue)<=F(1,20*len(queries)))
        results.append(dict(name=q['name'],descriptor=q['descriptor'],source=q['source'],sign=q['sign'],
            statistics=stats,stages=stages,p_one_sided=pvalue,provisional_replication=qualified))
    output=destination.parent/'analysis'
    audit=native_audit(data,queries,output,'fresh-audit')
    value=dict(schema='whole-game-contrast-transfer-v1',fit_id=fit_value['id'],panel_id=w.read(directory/'panel.json')['id'],
        rows=len(data),data_sha256=w.digest(data),patterns=results,audit=audit)
    value['id']=w.digest(value);w.immutable_json(destination,value)
    w.immutable_json(output/'fresh-rows.json',data)
    return value


if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__);sub=parser.add_subparsers(dest='command',required=True)
    q=sub.add_parser('fit');q.add_argument('directory',type=Path);q.add_argument('destination',type=Path)
    q=sub.add_parser('evaluate');q.add_argument('directory',type=Path);q.add_argument('fit',type=Path);q.add_argument('destination',type=Path)
    args=parser.parse_args()
    result=fit(args.directory,args.destination) if args.command=='fit' else evaluate(args.directory,args.fit,args.destination)
    print(w.p.canonical(result),flush=True)
