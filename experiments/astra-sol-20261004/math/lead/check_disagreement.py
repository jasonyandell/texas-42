"""Independent exact finite checks of the Lean result and its strictness edge."""
from itertools import product
import json

couplings = 0
for weights in product(range(3), repeat=3):
    for f in product((0,1), repeat=3):
        for g in product((0,1), repeat=3):
            for d in product((0,1), repeat=3):
                if any(not z and a != b for a,b,z in zip(f,g,d)): continue
                mass=lambda v:sum(w*x for w,x in zip(weights,v))
                assert abs(mass(f)-mass(g)) <= mass(d)
                couplings+=1
transports=0
for old_a,old_b,new_a,new_b,e_a,e_b in product(range(5),repeat=6):
    if old_a > new_a+e_a or new_b > old_b+e_b: continue
    if old_b+e_a+e_b<old_a:
        assert new_b<new_a
    if old_b+e_a+e_b<=old_a:
        assert new_b<=new_a
    transports+=1
# At equality weak optimality is preserved; canonical optimality can fail.
old=[3,1];new=[2,2];errors=[1,1]
assert old[0]-old[1]==sum(errors) and new[0]==new[1]
# Candidate 0 has larger tile id; tie-breaking now favors candidate 1.
tiles=[9,2]
choose=lambda values:max(range(2),key=lambda a:(values[a],-tiles[a]))
assert choose(old)==0 and choose(new)==1
print(json.dumps(dict(weighted_couplings=couplings,action_transports=transports,
    equality_canonical_counterexample=dict(old=old,new=new,errors=errors,tiles=tiles),
    result='passed')))
