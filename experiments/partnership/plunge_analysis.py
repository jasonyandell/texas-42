"""A resumable, capped, full-support comparison for one independently imported flag."""
import argparse
from pathlib import Path

import campaign as c
import gym
from gym_deployed import Evaluator, configurations
from plunge_io import flag_root

FUTURES={
    'l1':dict(focal='l1-default',partner='l1-default',opponents='l1-default'),
    'partner-l2':dict(focal='l1-default',partner='l2-partner-default',opponents='l1-default'),
    'reviewed':dict(focal='l1-partner-rollout',partner='l1-partner-rollout',opponents='l1-partner-rollout'),
}


def identity_for(req,future):
    """Only evaluation dependencies govern reuse, never frontend presentation."""
    settings=dict(contract='deployed-gym-v1',max_worlds=400,players=FUTURES[future],sample_worlds=None,sample_seed=req['seed'])
    return dict(schema='plunge-analysis-v1',request=req,settings=settings,players=configurations(settings),
                implementation=c.identities(),sources={name:gym.file_hash(gym.HERE/name) for name in
                ('plunge_analysis.py','plunge_io.py','gym.py','gym_deployed.py','gym_replay.py','sunshine_worlds.py')},
                gym_native=gym.file_hash(gym.ENGINE))


def analyze(flag_path,output,future,seconds=240):
    flag=c.read(flag_path);req,played,_=flag_root(flag['share_code'],flag['ply'],flag['request']['seed'])
    if req!=flag['request'] or played!=flag['played']:raise ValueError('flag replay changed')
    identity=identity_for(req,future);settings=identity['settings']
    with gym.run_lock(output):
        gym.pin(output,identity)
        info=gym.native(req,inspect=True,seconds=3)
        if info['worlds']>400:
            result=dict(status='outside-scope',support=info['worlds'],limit=400,
                        message='Saved for the gym. This position has too many compatible hands for the current full comparison.')
        else:
            item=dict(id=gym.digest(req)[:20],request=req)
            Evaluator(output,settings).run([item],4,seconds,45)
            case=c.read(output/'items'/(item['id']+'.json'))
            if case is None:result=dict(status='partial',message='The slice ended; completed trajectories are saved. Resume to continue.')
            else:
                key=case['key']; worlds=key['worlds'];best=max(a['success_mass'] for a in key['actions'])
                result=dict(status='complete',support=worlds,coverage='census',future=future,players=FUTURES[future],
                            request=req,played=played,alternative=flag['alternative'],
                            objective='make' if req['seat']%2==req['bidder']%2 else 'set',
                            actions=[dict(tile=a['tile'],makes=a['success_mass'],worlds=worlds,best=a['success_mass']==best,
                                          played=a['tile']==played,alternative=a['tile']==flag['alternative']) for a in key['actions']],
                            meaning='Exact for uniform compatible hands and these frozen continuation players; a model-relative comparison.')
        gym.atomic(output/'result.json',result)
        print(gym.canonical(result),flush=True)


if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('flag',type=Path);p.add_argument('output',type=Path)
    p.add_argument('--future',choices=FUTURES,default='l1');p.add_argument('--seconds',type=int,default=240)
    a=p.parse_args()
    if not 50<=a.seconds<=250:p.error('seconds must be 50..250, inside the external 295-second watchdog')
    analyze(a.flag,a.output,a.future,a.seconds)
