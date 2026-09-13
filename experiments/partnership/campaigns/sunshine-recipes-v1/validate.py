"""Validate recipe iteration after the full L1 run; run inside the watchdog.

Usage: python3 .../validate.py /path/to/sunshine-recipes-v1
Requires a completed base recipe in ROOT/cache and its interruption snapshot.
"""
from collections import Counter
import gzip
import json
from pathlib import Path
import subprocess
import sys

sys.path.insert(0,str(Path(__file__).resolve().parents[2]))
import gym
from gym_compare import comparison, snapshot, compare
from types import SimpleNamespace


def read(path):return json.loads(path.read_text())


def main(root):
    cache=root/'cache'
    base=read(cache/'latest.json')
    assert base['complete'] and base['resolved']['name']=='partnership-count-l1'
    assert base['resolved']['arguments']['selection']['criterion']=='query'
    receipts={'all':base}
    gym.atomic(root/'receipts/all.json',base)
    values=Path(base['evaluation'])
    frozen={str(p.relative_to(cache)):gym.file_hash(p) for p in values.rglob('*.json')
            if p.name not in ('status.json','manifest.json') and '/failures/' not in str(p)}
    interrupted=read(root/'interrupt-snapshot.json')
    assert all((cache/p).exists() and gym.file_hash(cache/p)==h for p,h in interrupted.items())
    query=(gym.ROOT/'walt/gym/queries/offer-five.scheme').read_text()
    variants=[('required',['selection.criterion=query-required']),
              ('avoided',['selection.criterion=query-avoided']),
              ('strong',['selection.criterion=query-required','selection.min_contrast=1/20']),
              ('five',['query.name=offer-five','query.source='+query])]
    for label,overrides in variants:
        command=[sys.executable,str(gym.HERE/'gym.py'),'generate','--spec',str(gym.ROOT/'walt/gym/specs/partnership-count-l1.json'),
                 '--output',str(cache),'--workers','10','--seconds','120','--case-seconds','30']
        for override in overrides:command+=['--set',override]
        subprocess.run(command,check=True)
        receipt=read(cache/'latest.json')
        assert receipt['complete'] and receipt['new_values']==0
        assert receipt['resolved']['evaluation_id']==base['resolved']['evaluation_id']
        receipts[label]=receipt;gym.atomic(root/'receipts'/(label+'.json'),receipt)
    assert all(gym.file_hash(cache/p)==h for p,h in frozen.items())
    current={str(p.relative_to(cache)) for p in values.rglob('*.json')
             if p.name not in ('status.json','manifest.json') and '/failures/' not in str(p)}
    assert set(frozen)==current
    with gzip.open(root/'value-artifact-hashes.json.gz','wt') as out:json.dump(frozen,out,sort_keys=True)

    subprocess.run([sys.executable,str(gym.HERE/'gym.py'),'generate','--spec',str(gym.ROOT/'walt/gym/specs/partnership-bid-making.json'),
                    '--output',str(cache),'--workers','10','--seconds','150','--case-seconds','30'],check=True)
    teacher=read(cache/'latest.json')
    assert teacher['complete'] and teacher['reference_verified']
    receipts['teacher']=teacher;gym.atomic(root/'receipts/teacher.json',teacher)
    for label,a,b in [('teacher-vs-l1','teacher','all'),('teacher-vs-l1-required','teacher','required'),
                      ('count-vs-five','all','five'),('required-vs-strong','required','strong')]:
        compare(SimpleNamespace(before=Path(receipts[a]['gallery']),after=Path(receipts[b]['gallery']),output=root/(label+'.json')))

    rows=[read(p) for p in (values/'items').glob('*.json')]
    byid={r['id']:r for r in rows}
    previous=gym.HERE/'campaigns/sunshine-gym-replay-v1'
    panel={r['id']:r for r in read(previous/'panel.json')['roots']}
    agreement=[]
    for r in read(previous/'summary.json')['roots']:
        rid=gym.digest(panel[r['id']]['request'])[:20]
        key=byid[rid]['key']
        observed={str(a['tile']):a['success_mass'] for a in key['actions']}
        assert observed==r['deployed_values'] and key['best']==r['deployed_best']
        agreement.append(dict(previous=r['id'],coordinate=rid,worlds=key['worlds'],values=observed))

    discovery=read(Path(base['gallery'])/'discovery.json')['rows']
    relations={}
    from gym_compare import transition
    for side in ('declaring','defending'):
        measured=[r for r in discovery if r['side']==side and 'outcome' in r]
        relations[side]=dict(measured=len(measured),query_relation=dict(Counter(transition(r) for r in measured)),
                             all_actions_tie=sum(not r['outcome']['strict'] for r in measured))
    summary=dict(schema='sunshine-recipe-validation-v1',value_identity=read(values/'manifest.json'),
                 validation_invocation_commit=subprocess.check_output(['git','rev-parse','HEAD'],cwd=gym.ROOT,text=True).strip(),
                 receipts=receipts,interrupted_files_unchanged=len(interrupted),
                 variant_files_unchanged=len(frozen),artifact_index='value-artifact-hashes.json.gz',
                 positions=len(rows),worlds=sum(r['key']['worlds'] for r in rows),
                 full_trajectories=sum(len(a['traces']) for r in rows for a in r['key']['actions']),
                 unique_decisions=len(list((values/'decisions').glob('*.json'))),
                 relations=relations,previous_replay_agreement=agreement)
    gym.atomic(root/'validation.json',summary)
    print(gym.canonical({k:v for k,v in summary.items() if k not in ('receipts','previous_replay_agreement')}),flush=True)


if __name__=='__main__':main(Path(sys.argv[1]).resolve())
