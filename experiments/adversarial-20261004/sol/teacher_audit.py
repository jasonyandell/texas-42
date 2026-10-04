#!/usr/bin/env python3
"""Reproduce data generation and exact row split only; never train or save a net."""
import sys
sys.dont_write_bytecode=True
import ast,json,hashlib
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
SOURCE=ROOT/'experiments/astra-sol-20261004/phase2/incoming/rollout_net.py'
sys.path.insert(0,str(ROOT/'experiments/astra-sol-20261004/phase2/incoming/drive'))
import engine42 as engine
np=engine.np
scope={name:getattr(engine,name) for name in dir(engine) if not name.startswith('_')}
scope.update(dict(np=np))
scope['rng']=np.random.default_rng(42)
scope.update(dict(MY=2,BIDDER=1,BID=30,NSTATE=30))
deal=engine.random_deals(scope['rng'],1)[0];own=int(deal[2]);bidder_hand=int(deal[1])
counts=[sum(bool(bidder_hand>>t&1) and p in engine.TILES[t] for t in range(28)) for p in range(7)]
trump=max(range(7),key=lambda p:(counts[p],p))
scope.update(dict(my_hand=own,my_tiles=[t for t in range(28) if own>>t&1],rules=engine.Rules(trump)))
tree=ast.parse(SOURCE.read_text())
selected=[node for node in tree.body if isinstance(node,ast.FunctionDef) and node.name in
          ('enc_state_init','dense_feats','gen_uniform')]
assert len(selected)==3
# Retain the already-computed game ids solely for leakage accounting.
generate=next(node for node in selected if node.name=='gen_uniform')
assert isinstance(generate.body[-1],ast.Return)
generate.body[-1].value.elts.append(ast.Name(id='gi',ctx=ast.Load()))
module=ast.fix_missing_locations(ast.Module(body=selected,type_ignores=[]))
exec(compile(module,str(SOURCE)+' [functions only]','exec'),scope)
Xi,Xd,labels,game_labels,ids=scope['gen_uniform'](scope['rng'],14000)
# Initialization consumes random draws before the original line81 row split.
for shape in ((840,64),(10,64),(64,32),(32,)):
    scope['rng'].standard_normal(shape)
perm=scope['rng'].permutation(len(labels));val=perm[:20000];train=perm[20000:]
train_games=set(map(int,ids[train]));validation_games=set(map(int,ids[val]))
shared=train_games & validation_games
assert np.all(labels==game_labels[ids])
print(json.dumps(dict(source_sha256=hashlib.sha256(SOURCE.read_bytes()).hexdigest(),
    total_games=14000,own_hand=scope['my_tiles'],trump=trump,rows=len(labels),
    training_rows=len(train),validation_rows=len(val),training_games=len(train_games),
    validation_games=len(validation_games),games_in_both=len(shared),
    validation_rows_with_same_game_in_training=int(np.isin(ids[val],list(train_games)).sum()),
    same_game_label_repeated=True,parameters=840*64+10*64+64+64*32+32+32+1,
    trained=False,executed_top_level_source=False,scope='Data/split audit, not recovered model validation accuracy.'),indent=2))
