#!/usr/bin/env python3
"""Independent legal-prefix occupancy counts from separately audited Python rules."""
import sys,json
from pathlib import Path
sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT/'experiments/native-frontier-20261004/checks'))
from scalar_oracle import kernel_of,draw
rows=json.loads((ROOT/'experiments/native-frontier-20261004/checks/independent-panel.json').read_text())['rows']
records=[]
for row in rows:
 k=kernel_of(row);bid=row['request']['bid'];seeds=row['seeds']
 layer=[(k.root,list(range(k.n)))];edges=coordinates=active_coordinates=0;layers=[]
 def terminal(state):return state.points[k.bidder%2]>=bid or state.points[1-k.bidder%2]>42-bid
 while any(not terminal(s) for s,_ in layer):
  next_layer=[];active=0
  for state,worlds in layer:
   if terminal(state):next_layer.append((state,worlds));continue
   active+=1;actor=(state.leader+len(state.trick))%4
   if actor==k.focal:
    legal=k.moves(state,worlds[0]);assert all(k.moves(state,w)==legal for w in worlds)
    next_layer.extend((k.after(state,t),worlds) for t in legal)
   else:
    buckets={}
    for w in worlds:buckets.setdefault(draw(seeds[w],state,k.bidder,k.moves(state,w)),[]).append(w)
    next_layer.extend((k.after(state,t),ws) for t,ws in sorted(buckets.items()))
  n=sum(len(ws) for _,ws in next_layer);edges+=n;coordinates+=len(next_layer);active_coordinates+=active
  layers.append({'rows':n,'coordinates':len(next_layer),'active_parent_coordinates':active});layer=next_layer
 records.append({'seed':row['seed'],'worlds':k.n,'row_edges':edges,'public_coordinates':coordinates,'active_parent_coordinates':active_coordinates,'layers':layers})
print(json.dumps({'roots':len(records),'row_edges':sum(r['row_edges'] for r in records),'public_coordinates':sum(r['public_coordinates'] for r in records),'records':records},indent=2))
