"""Audit and measure the live continuation check against complete gym keys."""
import argparse
from dataclasses import asdict
from fractions import Fraction
import json
from pathlib import Path
import subprocess

import campaign as c
import gym
from gym_replay import full_hands
from matchup import Player
from partner_rollout import BINARY, MAX_SECONDS
from player import decide
from runtime import DecisionSession


def read(path):return json.loads(path.read_text())


def audit(args):
    reference=read(args.reference) if args.reference else read(gym.HERE/'campaigns/sunshine-recipes-v1/validation.json')['receipts']['all']
    cache=Path(reference['evaluation'])
    results=[]
    for name,case in gym.gallery(args.gallery):
        req=case['request'];baseline=case['pair']['comparison']
        payload='\n'.join(k+' '+' '.join(map(str,v if isinstance(v,list) else [v])) for k,v in req.items())+'\n'
        completed=subprocess.run([str(BINARY),'--baseline',str(baseline),'--samples','400',
                                  '--milliseconds','14000','--audit'],input=payload,text=True,capture_output=True,timeout=15,
                                 env={**__import__('os').environ,'RAYON_NUM_THREADS':'2'})
        if completed.returncode:raise ValueError(completed.stderr)
        value=json.loads(completed.stdout)
        assert value['coverage']=='census',value
        assert value['values']==[[a['tile'],a['success_mass']] for a in case['key']['actions']]
        expected={(a['tile'],gym.world_key(t['hands'])):t for a in case['key']['actions'] for t in a['traces']}
        assert len(value['traces'])==len(expected)
        for trace in value['traces']:
            known=expected.pop((trace['action'],gym.world_key(trace['hands'])))
            assert (req['plays']+known['plays'])[:len(trace['record'])]==trace['record']
            assert known['success']==trace['made']
            hands=full_hands(req,trace['hands'])
            points,_,_,_=gym.replay_record(hands,trace['record'],req['decl'],req['bidder'])
            assert points==trace['banked']
            team=req['bidder']%2
            assert (points[team]>=30) if trace['made'] else (points[1-team]>12)
        assert not expected
        for decision in value['decisions']:
            request={**req,**{k:decision[k] for k in ('seat','hand','plays')}}
            identity=dict(request=request,player=asdict(Player('l1-default')))
            saved=read(cache/'decisions'/(gym.digest(identity)+'.json'))
            assert saved['identity']==identity
            assert saved['response']['choice']==decision['choice']
        gym.atomic(args.output/(name+'.json'),value)
        results.append(dict(scenario=name,worlds=value['samples'],traces=len(value['traces']),
                            decisions=len(value['decisions']),cache_hits=value['cache_hits'],elapsed_us=value['elapsed_us']))
    gym.atomic(args.output/'summary.json',dict(reference=reference,results=results))
    print(gym.canonical(results),flush=True)


def roots(source):
    for path in sorted((source/'items').glob('*.json')):
        case=read(path)
        if 'key' in case and gym.side_of(case)=='declaring':
            yield case


def measure(args):
    cases=list(roots(args.source))
    items=[dict(id=case['id']) for case in cases]
    byid={case['id']:case for case in cases}
    manifest=dict(schema='rollout-study-v1',source=str(args.source.resolve()),
                  cases={case['id']:gym.digest(case) for case in cases},identities=c.identities(),
                  runner=gym.file_hash(__file__),max_review_seconds=MAX_SECONDS,
                  phase=args.phase,groups=args.groups)
    gym.pin(args.output,manifest)
    def job(item):
        case=byid[item['id']];req=gym.pupil_request(case['request'])
        with DecisionSession() as session:
            before=decide(req,mode='baseline',session=session)
            after=decide(req,mode='baseline',review='partner-rollout',session=session)
        assert not before['over_budget'] and not after['over_budget']
        assert before['route']=='baseline' and after['route'] in ('baseline','baseline-reviewed')
        assert before['choice']==after['review_result']['baseline']
        b,a=before['choice'],after['choice']
        masses={x['tile']:x['success_mass'] for x in case['key']['actions']}
        return dict(id=item['id'],request=req,source=case.get('source'),before=before,after=after,
                    grade_before=gym.grade(case['key'],b),grade_after=gym.grade(case['key'],a),
                    delta=str(Fraction(masses[a]-masses[b],case['key']['worlds'])),
                    worlds=case['key']['worlds'],before_makes=masses[b],after_makes=masses[a])
    with gym.run_lock(args.output):
        gym.bounded(items,job,args.output,args.workers,args.seconds,30)
    report(args.output,byid)


def report(output,byid=None):
    manifest=read(output/'manifest.json')
    rows=[read(output/'items'/(rid+'.json')) for rid in manifest['cases'] if (output/'items'/(rid+'.json')).exists()]
    if len(rows)!=len(manifest['cases']):
        print(gym.canonical(dict(complete=False,saved=len(rows),planned=len(manifest['cases']))));return
    deltas=[Fraction(r['delta']) for r in rows]
    groups={}
    for r in rows:
        group=str(r['source']['seed']) if r.get('source') else r['id']
        groups.setdefault(group,[]).append(Fraction(r['delta']))
    summary=dict(complete=True,roots=len(rows),source_seeds=len(groups),
        baseline_optimal=sum(r['grade_before']['optimal'] for r in rows),candidate_optimal=sum(r['grade_after']['optimal'] for r in rows),
        improved=sum(d>0 for d in deltas),harmed=sum(d<0 for d in deltas),tied=sum(d==0 for d in deltas),
        changed=sum(r['before']['choice']!=r['after']['choice'] for r in rows),
        mean_delta=str(sum(deltas,Fraction())/len(rows)) if rows else None,
        mean_regret_before=str(sum((Fraction(r['grade_before']['regret']) for r in rows),Fraction())/len(rows)) if rows else None,
        mean_regret_after=str(sum((Fraction(r['grade_after']['regret']) for r in rows),Fraction())/len(rows)) if rows else None,
        equal_seed_mean_delta=str(sum((sum(v,Fraction())/len(v) for v in groups.values()),Fraction())/len(groups)) if groups else None,
        mean_baseline_seconds=sum(r['before']['elapsed_us'] for r in rows)/len(rows)/1e6 if rows else None,
        mean_candidate_seconds=sum(r['after']['elapsed_us'] for r in rows)/len(rows)/1e6 if rows else None,
        statuses=dict(__import__('collections').Counter(r['after']['review_result']['status'] for r in rows)),
        census=sum(r['after']['review_result'].get('coverage')=='census' for r in rows),
        source_seed_deltas={k:str(sum(v,Fraction())/len(v)) for k,v in groups.items()})
    if byid is not None:
        totals=dict(gained=0,lost=0,both_success=0,both_failure=0)
        for r in rows:
            key=byid[r['id']]['key']; b,a=r['before']['choice'],r['after']['choice']
            traces={x['tile']:{gym.world_key(t['hands']):t['success'] for t in x['traces']} for x in key['actions']}
            pair=dict(gained=0,lost=0,both_success=0,both_failure=0)
            for w,succeeds in traces[a].items():
                label=('both_success' if traces[b][w] else 'gained') if succeeds else ('lost' if traces[b][w] else 'both_failure')
                pair[label]+=1;totals[label]+=1
            r['paired']=pair
        summary['pooled_world_pairs']=totals
    gym.atomic(output/'report.json',dict(summary=summary,manifest=manifest,rows=rows))
    print(gym.canonical(summary),flush=True)


def main():
    p=argparse.ArgumentParser(description=__doc__);sub=p.add_subparsers(dest='command',required=True)
    a=sub.add_parser('audit');a.add_argument('--gallery',type=Path,required=True);a.add_argument('--output',type=Path,required=True)
    a.add_argument('--reference',type=Path,help='Complete deployed-gym generation receipt; defaults to the development census')
    m=sub.add_parser('measure');m.add_argument('--source',type=Path,required=True);m.add_argument('--output',type=Path,required=True)
    m.add_argument('--workers',type=int,default=10);m.add_argument('--seconds',type=int,default=240)
    m.add_argument('--phase',choices=['development','fresh'],required=True);m.add_argument('--groups',default='source seed')
    args=p.parse_args()
    if args.command=='measure' and not(1<=args.workers<=10 and 40<=args.seconds<=270):p.error('invalid workload bounds')
    __import__('os').environ['WALT_RAYON_THREADS']='2'
    globals()[args.command](args)


if __name__=='__main__':main()
