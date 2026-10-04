#!/usr/bin/env python3
"""Finite-tape boundary oracle, root separation, duplicate mass and absorption."""
import sys
sys.dont_write_bytecode=True
from pathlib import Path
import copy,json,random,hashlib
HERE=Path(__file__).resolve().parent
sys.path.insert(0,str(HERE.parent))
import frontier
from parallel_roots import kernel_of
np=frontier.np
rng=random.Random(71091)
numbers={0,1,(1<<64)-1}
for bit in range(1,64):numbers.update(((1<<bit)-1,1<<bit,(1<<bit)+1))
numbers.update(rng.getrandbits(64) for _ in range(4096));u=np.array(sorted(numbers),np.uint64)
for n in range(1,8):
    got=(((u>>np.uint64(32))*np.uint64(n)+(((u&np.uint64(0xffffffff))*np.uint64(n))>>np.uint64(32)))>>np.uint64(32))
    expected=[(int(x)*n)>>64 for x in u]
    assert list(map(int,got))==expected and all(0<=k<n for k in expected)

class ObservedGame(frontier.Game):
    mixed_terminal_steps=0
    def play(self,tile):
        super().play(tile)
        if self.done.any() and not self.done.all():ObservedGame.mixed_terminal_steps+=1
frontier.Game=ObservedGame
rows=[r for r in json.loads((HERE.parent/'results/parallel-panel/plan.json').read_text())['rows'] if r['status']=='ready' and r['request']['decl']<=6]
cases=[]
for index,row in enumerate(rows[:12]):
    one=copy.deepcopy(row);one['seed']=row['seed']+30000000;one['worlds']=one['worlds'][:1];one['tape']=one['tape'][:1]
    duplicate=copy.deepcopy(one);duplicate['seed']=one['seed']+10000000
    weighted=copy.deepcopy(row);weighted['seed']=row['seed']+20000000
    weighted['worlds']=[w for w in row['worlds'] for _ in range(3)]
    weighted['tape']=[t for t in row['tape'] for _ in range(3)]
    batch=[one,duplicate,weighted,row]
    result=frontier.run(batch)['answers'];permuted=frontier.run(list(reversed(batch)))['answers']
    assert result==permuted
    for test in batch:
        expected={str(a):v for a,v in kernel_of(test).reference(test['tape'],30)[0].items()}
        assert result[str(test['seed'])]==expected
    assert result[str(one['seed'])]==result[str(duplicate['seed'])]
    assert result[str(weighted['seed'])]=={a:3*v for a,v in result[str(row['seed'])].items()}
    cases.append(dict(seed=row['seed'],ply=row['ply'],duplicate_roots_separate=True,
        mixed_sample_sizes=[1,1,120,40],duplicate_mass_exact=True,permutation_invariant=True))
assert ObservedGame.mixed_terminal_steps>0
print(json.dumps(dict(high_multiply_checks=len(u)*7,boundaries_and_random_tapes_exact=True,
    root_cases=len(cases),mixed_terminal_steps_observed=ObservedGame.mixed_terminal_steps,
    all_reference_vectors_equal=True,cases=cases,
    source_sha256=hashlib.sha256((HERE.parent/'frontier.py').read_bytes()).hexdigest(),
    scope='Finite frozen Dice vectors, including singleton/scenario duplication/root permutation; no continuous chance or higher-k claim.'),indent=2))
