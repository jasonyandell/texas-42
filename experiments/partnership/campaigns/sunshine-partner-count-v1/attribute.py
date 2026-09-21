"""Post-hoc model attribution on the old development gym, never policy selection."""
import argparse
from fractions import Fraction
import json
import os
from pathlib import Path
import subprocess
import sys

HERE=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(HERE))
import gym

p=argparse.ArgumentParser(description=__doc__)
p.add_argument('output',type=Path);args=p.parse_args()
cases=dict(gym.gallery(Path('/Users/jason/data/texas-42/partnership-bid-making-v1')))
binary=HERE.parents[1]/'walt/target/release/partner_review_ablation'
baseline=HERE/'campaigns/sunshine-partner-count-v1/gym-30.json'
base={r['scenario']:r['baseline'] for r in json.loads(baseline.read_text())['rows']}
manifest=dict(schema='sunshine-posthoc-attribution-v1',binary_sha256=gym.file_hash(binary),
              runner_sha256=gym.file_hash(__file__),baseline_sha256=gym.file_hash(baseline),
              cases_identity=gym.exam_identity(cases),scope='old selected declaring-side gym; post-hoc mechanism diagnostic')

def job(item):
    name=item['id'];case=cases[name]
    request=gym.pupil_request(case['request'])
    text='\n'.join(k+' '+' '.join(map(str,v if isinstance(v,list) else [v])) for k,v in request.items())+'\n'
    proc=subprocess.run([str(binary)],input=text,text=True,capture_output=True,timeout=10,
                        env={**os.environ,'RAYON_NUM_THREADS':'1'})
    if proc.returncode:raise RuntimeError(proc.stderr)
    result=json.loads(proc.stdout)
    expected={a['tile']:a['success_mass'] for a in case['key']['actions']}
    assert dict(result['arms'][1]['values'])==expected
    assert result['arms'][1]['field_id']==case['key']['field_id']
    assert result['worlds']==case['key']['worlds']
    choices=[]
    for arm in result['arms']:
        values=dict(arm['values']);choice=base[name]['choice']
        if choice not in case['target_actions']:
            for action in sorted(case['target_actions']):
                if values[action]>values[choice]:choice=action
        choices.append(choice)
    return dict(scenario=name,baseline=base[name],offers=case['target_actions'],
                l0_check_choice=choices[0],l1_partner_check_choice=choices[1],comparison=result)

with gym.run_lock(args.output):
    gym.pin(args.output,manifest)
    gym.bounded([dict(id=name) for name in cases],job,args.output,10,260,12)
    rows=[json.loads((args.output/'items'/(name+'.json')).read_text()) for name in cases]
    misses=[r for r in rows if not r['baseline']['optimal']]
    result=dict(manifest=manifest,original_misses=len(misses),
                l0_check_changes=sum(r['l0_check_choice']!=r['baseline']['choice'] for r in misses),
                l1_partner_check_changes=sum(r['l1_partner_check_choice']!=r['baseline']['choice'] for r in misses),
                rows=rows)
    gym.atomic(args.output/'report.json',result)
    print(json.dumps({k:v for k,v in result.items() if k!='rows'},indent=2))
