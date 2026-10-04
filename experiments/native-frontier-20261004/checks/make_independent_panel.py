#!/usr/bin/env python3
import json
from pathlib import Path
from scalar_oracle import ROOT,kernel_of,native_values,mix,record,MASK
from parallel_roots import fixture
from rng_boundaries import state_for_first
rows=[]
for i in range(128):
    row=fixture(963000+i,16+i%8,8)
    if row['status']!='ready' or row['request']['decl']>6:continue
    k=kernel_of(row)
    seeds=[mix(row['seed']^sid) for sid in range(k.n)]
    forced=None
    for action in k.moves(k.root,0):
        child=k.after(k.root,action)
        actor=(child.leader+len(child.trick))%4
        if actor!=k.focal:
            legal=k.moves(child,0)
            if len(legal)>1:
                n=len(legal);zone=MASK-MASK%n
                seeds[0]=state_for_first(zone)^record(child,k.bidder)
                forced={'action':action,'scenario':0,'legal_count':n,'first_rng_output':zone}
                break
    values,nodes=native_values(row,seeds)
    rows.append(dict(row,seeds=seeds,expected_native_focal_success={str(a):v for a,v in values.items()},oracle_nodes=nodes,forced_rejection=forced))
out=Path(__file__).resolve().parent/'independent-panel.json'
out.write_text(json.dumps({'fresh_deal_range':[963000,963127],'rows':rows,'scope':'Eight lawful outer worlds, plies16..23, pip trumps; independent grouped native-Dice scalar oracle, no higher-k evaluation.'},indent=2)+'\n')
print(json.dumps({'roots':len(rows),'partial_roots':sum(r['ply']%4!=0 for r in rows),'forced_rejection_roots':sum(r['forced_rejection'] is not None for r in rows),'bidder_counts':{str(i):sum(r['request']['bidder']==i for r in rows) for i in range(4)},'output':str(out)},indent=2))
