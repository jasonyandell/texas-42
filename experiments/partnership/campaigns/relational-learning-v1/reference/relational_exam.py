#!/usr/bin/env python3
"""Apply frozen shared actors to a maintained exact gym, without reselection."""
import argparse
from fractions import Fraction
import json
from pathlib import Path
import signal
import sys
import threading
import time

import gym
from policy_gym import request_text
from relational_campaign import Campaign, materialize, report, sha


class Exam(Campaign):
    def __init__(self, args):
        self.args=args
        self.output=args.output.resolve()
        self.binary=args.binary.resolve()
        self.stop=threading.Event(); self.lock=threading.Lock(); self.processes=set()
        self.deadline=time.monotonic()+args.seconds
        self.completed=0; self.active=set()
        self.cases=dict(gym.gallery(args.gallery.resolve()))
        frozen=json.loads((args.learned/'frozen/manifest.json').read_text())['actors']
        self.actors={k:v['path'] for k,v in frozen.items()}
        if any(sha(v['path'])!=v['sha256'] for v in frozen.values()):
            raise ValueError('frozen learned actor changed')
        self.manifest={'schema':'texas42-relational-exam-v1','binary_sha256':sha(self.binary),
            'learned_manifest_sha256':sha(args.learned/'frozen/manifest.json'),
            'actors':frozen,'source':str(args.gallery.resolve()),'exam_identity':gym.exam_identity(self.cases),
            'field':'gym','samples':args.samples,'work':args.work,
            'role':'post-freeze diagnostic; existing outcome-selected exercises, not an unbiased deployment sample',
            'sources':{str(p):sha(p) for p in [Path(__file__),Path(__file__).with_name('relational_campaign.py'),Path(__file__).with_name('policy_gym.py')]}}
        self.output.mkdir(parents=True,exist_ok=True)
        gym.pin(self.output,self.manifest)
        self.roots_map={}
        for name,case in self.cases.items():
            if Path(name).name!=name: raise ValueError('unsafe exercise id')
            req=self.output/'requests'/(name+'.request')
            materialize(req,request_text(case['request']))
            self.roots_map[name]={'request_path':str(req),'seed':case['request']['seed']}

    def run_exam(self):
        results=self.evaluate('exam',self.roots_map,self.actors,False)
        for name,value in results.items():
            key=self.cases[name]['key']
            expected=max(Fraction(a['success_mass'],key['worlds']) for a in key['actions'])
            if Fraction(value['optimum'])!=expected:
                raise ValueError(name+': native teacher changed the maintained exact optimum')
        summary=report(results,self.roots_map)
        summary['diagnostic_only']=True
        summary['coordinate_count']=len(results)
        summary['source_deal_groups']=len({c['request']['seed'] for c in self.cases.values()})
        summary['averaging']='equal coordinate weights; related coordinates share a source deal'
        summary.pop('paired',None)
        summary.pop('root_groups',None)
        for actor,row in summary['arms'].items():
            row['optimal_root_actions']=sum(Fraction(v['actors'][actor]['root_regret'])==0 for v in results.values())
        gym.atomic(self.output/'summary.json',summary)
        self.status('complete',cases=len(results))
        return summary


def main(argv=None):
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--output',type=Path,required=True)
    p.add_argument('--learned',type=Path,required=True)
    p.add_argument('--gallery',type=Path,default=Path('/Users/jason/data/texas-42/partnership-bid-making-v1'))
    p.add_argument('--binary',type=Path,default=Path(__file__).resolve().parents[2]/'walt/target/release/relational_lab')
    p.add_argument('--workers',type=int,default=8)
    p.add_argument('--seconds',type=float,default=285)
    p.add_argument('--timeout',type=float,default=180)
    p.add_argument('--samples',type=int,default=16)
    p.add_argument('--work',type=int,default=2000000)
    args=p.parse_args(argv);args.field='gym'
    if not 1<=args.workers<=10 or not 0<args.seconds<=290 or not 0<args.timeout<=290:p.error('invalid wall/worker bounds')
    exam=None;old={}
    try:
        exam=Exam(args)
        with gym.run_lock(exam.output):
            for sig in (signal.SIGINT,signal.SIGTERM,signal.SIGHUP):old[sig]=signal.signal(sig,lambda _s,_f:exam.kill())
            print(json.dumps(exam.run_exam(),sort_keys=True))
        return 0
    except InterruptedError as e:
        if exam:exam.status('incomplete',reason=str(e))
        print(str(e),file=sys.stderr);return 75
    except Exception as e:
        if exam:exam.kill();exam.status('failed',reason=str(e))
        print(type(e).__name__+': '+str(e),file=sys.stderr);return 2
    finally:
        for sig,handler in old.items():signal.signal(sig,handler)


if __name__=='__main__':sys.exit(main())
