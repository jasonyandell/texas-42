"""Publish compact receipts and selected portable witnesses, not the raw cache."""
from collections import Counter
from fractions import Fraction
import gzip
import json
from pathlib import Path
import shutil
import sys

sys.path.insert(0,str(Path(__file__).resolve().parents[2]))
import gym
from gym_compare import witness


def read(path):return json.loads(path.read_text())


def main(root):
    output=Path(__file__).resolve().parent
    validation=read(root/'validation.json')
    base=validation['receipts']['all']
    values=Path(base['evaluation'])
    rows=[read(p) for p in sorted((values/'items').glob('*.json'))]
    audits=[gym.verify(row) for row in rows]
    assert len(audits)==validation['positions']
    gym.atomic(output/'value-keys.json',[
        dict(id=r['id'],request=r['request'],key_sha256=gym.digest(r['key']),
             file_sha256=gym.file_hash(values/'items'/(r['id']+'.json')),
             worlds=r['key']['worlds'],best=r['key']['best'],
             values={str(a['tile']):a['success_mass'] for a in r['key']['actions']}) for r in rows])
    cases=dict(gym.gallery(Path(base['gallery'])))
    pupils=[read(p) for p in sorted((root/'pupils/items').glob('*.json'))]
    assert len(pupils)==len(cases)==76
    groups={label:dict(exercises=0,optimal=0,query_choices=0) for label in ('required','avoided','tied')}
    for pupil in pupils:
        case=cases[pupil['scenario']]
        assert pupil['grade']==gym.grade(case['key'],pupil['response']['choice'])
        gap=Fraction(gym.query_contrast(case['key'],case['target_actions'])['gap'])
        label='required' if gap>0 else 'avoided' if gap<0 else 'tied'
        group=groups[label];group['exercises']+=1;group['optimal']+=pupil['grade']['optimal']
        group['query_choices']+=pupil['response']['choice'] in case['target_actions']
    baseline=dict(groups=groups,optimal=sum(p['grade']['optimal'] for p in pupils),exercises=len(pupils),
                  mean_regret=str(sum(Fraction(p['grade']['regret']) for p in pupils)/len(pupils)),
                  mean_decision_seconds=sum(p['response']['elapsed_us'] for p in pupils)/len(pupils)/1e6,
                  manifest=read(root/'pupils/manifest.json'),responses=pupils)
    gym.atomic(output/'baseline.json',baseline)

    catalog=[];witnesses=[]
    # Include both a saved-world and a lost-world witness when a pair has both.
    selected=['advantage-06','advantage-29','disadvantage-03']
    for name in selected:
        case=cases[name]
        baseline_choice=next(p['response']['choice'] for p in pupils if p['scenario']==name)
        case['pair']=next(pair for pair in gym.case_pairs(case) if pair['comparison']==baseline_choice)
        case['example_comparison']='Measured baseline first choice; all action values unchanged.'
        case['outcome']=gym.outcome_profile(case['key'])
        case['paired_outcomes']=gym.paired_outcomes(case['key'],case['pair'])
        gym.verify(case)
        gym.atomic(output/'examples'/(name+'.json'),case)
        catalog.append(dict(id=name,file=name+'.json',sha256=gym.file_hash(output/'examples'/(name+'.json'))))
        witnesses.append(dict(id=name,coordinate=case['id'],**witness(case)))
    gym.atomic(output/'examples/catalog.json',dict(schema='gym-catalog-v1',scenarios=catalog,
               selection='Three named diagnostic examples from the full L1 continuation collection.'))
    gym.atomic(output/'witnesses.json',witnesses)
    comparisons={}
    for label in ('teacher-vs-l1','teacher-vs-l1-required','count-vs-five','required-vs-strong'):
        result=read(root/(label+'.json'));comparisons[label]=result['summary']
        with gzip.open(output/(label+'.json.gz'),'wt') as f:json.dump(result,f,sort_keys=True)
        shutil.copyfile(root/(label+'.md'),output/(label+'.md'))
    validation.update(comparisons=comparisons,value_identity=read(values/'manifest.json'),
                      independently_replayed_positions=len(audits),
                      audited_worlds=sum(a['worlds'] for a in audits),baseline_groups=groups,
                      raw_directory=str(root),artifact_index=dict(path=str(root/'value-artifact-hashes.json.gz'),
                          sha256=gym.file_hash(root/'value-artifact-hashes.json.gz')),
                      baseline_optimal=baseline['optimal'],baseline_exercises=len(pupils))
    gym.atomic(output/'validation.json',validation)
    gym.atomic(output/'run-receipts.json',{p.parent.name:read(p) for p in sorted((root/'runs').glob('*/run.json'))})
    print(gym.canonical(dict(audited_positions=len(audits),groups=groups,examples=selected)),flush=True)


if __name__=='__main__':main(Path(sys.argv[1]).resolve())
