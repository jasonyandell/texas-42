#!/usr/bin/env python3
"""Shared relational policy learning with group splits and atomic resumable jobs.

The final test phase is reachable only after the selected programs are frozen.
Each native job has one public root or one bounded fit. See RELATIONAL-LEARNING.md.
"""
import argparse
from concurrent.futures import ThreadPoolExecutor, wait, FIRST_COMPLETED
from fractions import Fraction
import hashlib
import json
import math
import os
from pathlib import Path
import random
import signal
import subprocess
import sys
import tempfile
import threading
import time

import gym
from process_groups import kill_process_group

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
LOWEST = '(policy lowest (initial shared) (fallback lowest-legal))\n'
VARIANTS = ('exact', 'unpriced', 'priced')


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def materialize(path, content):
    """Publish immutable text once; resuming never truncates a valid input."""
    path = Path(path)
    if path.exists():
        if path.read_text() != content:
            raise ValueError('immutable text input changed: ' + str(path))
        return
    path.parent.mkdir(parents=True, exist_ok=True)
    fd, temporary = tempfile.mkstemp(prefix=path.name+'.', dir=path.parent)
    try:
        with os.fdopen(fd, 'w') as out:
            out.write(content)
            out.flush()
            os.fsync(out.fileno())
        os.replace(temporary, path)
    finally:
        if os.path.exists(temporary):
            os.unlink(temporary)


def checked_payload(directory, spec):
    result = directory / 'result.json'
    if not result.exists():
        return None
    record = json.loads(result.read_text())
    if gym.digest(record['payload']) != record.get('payload_sha256'):
        raise ValueError('completed job payload changed: ' + str(directory))
    if record['spec'] != spec:
        raise ValueError('job input identity changed: ' + str(directory))
    for path, digest in record['files'].items():
        if sha(path) != digest:
            raise ValueError('completed artifact changed: ' + path)
    return record['payload']


class Campaign:
    def __init__(self, args):
        self.args = args
        self.output = args.output.resolve()
        self.binary = args.binary.resolve()
        self.stop = threading.Event()
        self.lock = threading.Lock()
        self.processes = set()
        self.deadline = time.monotonic() + args.seconds
        self.completed = 0
        self.active = set()
        self.manifest = {
            'schema': 'texas42-relational-campaign-v1',
            'binary': str(self.binary), 'binary_sha256': sha(self.binary),
            'sources': {str(p.relative_to(ROOT)): sha(p) for p in [Path(__file__), HERE/'gym.py', HERE/'rules.py', HERE/'process_groups.py']},
            'seed': args.seed, 'counts': {'train-a': args.train, 'train-b': args.train, 'dev': args.dev, 'test': args.test},
            'tiles': args.tiles, 'max_worlds': args.max_worlds,
            'field': args.field, 'bid': 30, 'decl': 6,
            'samples': args.samples, 'node_work': args.work,
            'price_basis': 'scheme-information-events-v1', 'price_coefficients': [0, 1, -1, 1],
            'cost_arms': ['exact optimal action regret', 'PI-upper minus current actor continuation lower', 'priced-upper minus same actor continuation lower'],
            'clauses': args.clauses, 'beam': args.beam, 'ast': 256,
            'selection': 'equal-root full-policy value on dev; ties fewer clauses then bytes then digest',
            'root_prior': 'exact uniform mechanical root fiber; no field likelihood before continuation initialization',
            'root_filter': 'unresolved contract, >=2 own legal actions, bounded mechanical support; no action-value filter',
            'cohort_generator': 'disjoint 1000-seed ranges, first root satisfying only public filter',
            'grammar': 'fixed native standard Viewer library; no exact patches in learned actor',
        }
        self.output.mkdir(parents=True, exist_ok=True)
        gym.pin(self.output, self.manifest)
        base = self.output / 'lowest.policy'
        materialize(base, LOWEST)
        self.lowest = base

    def kill(self):
        self.stop.set()
        with self.lock:
            for process in list(self.processes):
                kill_process_group(process)

    def status(self, phase, **extra):
        gym.atomic(self.output/'status.json', {'phase': phase, 'completed_jobs_this_run': self.completed,
            'active_jobs': sorted(self.active), 'interrupted': self.stop.is_set(), **extra})

    def job(self, name, operation, options, inputs=None):
        directory = self.output/'jobs'/name
        directory.mkdir(parents=True, exist_ok=True)
        options = {k: str(v) for k, v in options.items()}
        spec = {'operation': operation, 'options': options,
                'inputs': {str(p): sha(p) for p in inputs or []}}
        cached = checked_payload(directory, spec)
        if cached is not None:
            return cached
        if self.stop.is_set() or time.monotonic() >= self.deadline:
            raise InterruptedError('campaign allowance reached')
        attempt = Path(tempfile.mkdtemp(prefix='attempt-', dir=directory))
        command = [str(self.binary), operation]
        for k, v in options.items():
            command.extend(['--'+k, str(v)])
        if operation in ('evaluate', 'fit'):
            command.extend(['--output', str(attempt)])
        with self.lock:
            if self.stop.is_set():
                raise InterruptedError('campaign interrupted')
            process = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                                       text=True, start_new_session=True)
            self.processes.add(process)
        try:
            stdout, stderr = process.communicate(timeout=min(self.args.timeout, max(.01, self.deadline-time.monotonic())))
        except subprocess.TimeoutExpired:
            kill_process_group(process)
            stdout, stderr = process.communicate()
            gym.atomic(attempt/'attempt.json', {'command':command, 'timeout':True, 'stdout':stdout, 'stderr':stderr})
            raise InterruptedError('native job timeout: '+name)
        finally:
            with self.lock:
                self.processes.discard(process)
        gym.atomic(attempt/'attempt.json', {'command':command, 'returncode':process.returncode, 'stdout':stdout, 'stderr':stderr})
        if process.returncode:
            if self.stop.is_set():
                raise InterruptedError('campaign interrupted')
            raise ValueError(name+': '+stderr[-1500:])
        payload = json.loads(stdout)
        if operation == 'generate':
            req = attempt/'root.request'
            req.write_text(payload['request'])
            payload['request_path'] = str(req)
        artifacts = {str(p):sha(p) for p in attempt.rglob('*') if p.is_file()}
        gym.atomic(directory/'result.json', {'spec':spec, 'payload':payload, 'payload_sha256':gym.digest(payload), 'files':artifacts})
        with self.lock:
            self.completed += 1
        return payload

    def parallel(self, phase, jobs):
        results = {}
        self.status(phase, target_jobs=len(jobs))
        with ThreadPoolExecutor(max_workers=self.args.workers) as pool:
            pending = iter(jobs)
            futures = {}
            exhausted = False
            while not exhausted or futures:
                while not exhausted and len(futures)<self.args.workers and not self.stop.is_set():
                    try:
                        key, operation, options, inputs = next(pending)
                    except StopIteration:
                        exhausted = True
                        break
                    self.active.add(key)
                    futures[pool.submit(self.job, phase+'-'+key,operation,options,inputs)] = key
                if self.stop.is_set() or time.monotonic()>=self.deadline:
                    self.kill()
                    raise InterruptedError('campaign allowance reached')
                if not futures:
                    break
                done,_ = wait(futures, timeout=.25, return_when=FIRST_COMPLETED)
                for future in done:
                    key=futures.pop(future)
                    self.active.discard(key)
                    try:
                        results[key]=future.result()
                    except Exception:
                        self.kill()
                        raise
                self.status(phase,target_jobs=len(jobs),phase_completed=len(results))
        return results

    def roots(self):
        jobs=[]
        index=0
        splits={}
        for split,n in self.manifest['counts'].items():
            for i in range(n):
                name=f'{split}-{i:04}'
                splits[name]=split
                jobs.append((name,'generate',{'seed':self.args.seed+1000*index,'tiles':self.args.tiles,
                    'max-worlds':self.args.max_worlds,'attempts':1000,'decl':6},[]))
                index+=1
        values=self.parallel('roots',jobs)
        seen_hands={}
        for name,value in values.items():
            value['split']=splits[name]
            if value['hand'] in seen_hands and seen_hands[value['hand']]!=value['split']:
                raise ValueError('starting-hand overlap across groups; reject this campaign split')
            seen_hands[value['hand']]=value['split']
        gym.atomic(self.output/'roots.json',values)
        return values

    def actors_file(self,name,actors):
        path=self.output/(name+'.actors')
        content=''.join(f'{name}\t{Path(p).resolve()}\n' for name,p in sorted(actors.items()))
        materialize(path, content)
        return path

    def evaluate(self,phase,roots,actors,prices):
        path=self.actors_file(phase,actors)
        inputs=[path,*map(Path,actors.values())]
        jobs=[(name,'evaluate',{'request':value['request_path'],'policies':path,'field':self.args.field,
            'seed':value['seed'],'samples':self.args.samples,'work':self.args.work,
            'prices':'on' if prices else 'off'},[Path(value['request_path']),*inputs]) for name,value in sorted(roots.items())]
        return self.parallel(phase,jobs)

    def fit(self,phase,example_paths):
        actors={}
        for variant,paths in example_paths.items():
            lesson=self.output/f'{phase}-{variant}.lessons'
            content=''.join(Path(p).read_text() for p in sorted(paths))
            materialize(lesson, content)
            result=self.job(phase+'-'+variant,'fit',{'lessons':lesson,'clauses':self.args.clauses,
                'beam':self.args.beam,'ast':256,'work':2000000},[lesson])
            for c in result['candidates']:
                actors[f'{variant}-{c["id"]}']=c['path']
        return actors

    def promote(self,phase,dev,actors):
        values=self.evaluate(phase,dev,actors,False)
        selected={}
        detail={}
        for variant in VARIANTS:
            candidates=[]
            for name,path in actors.items():
                if not name.startswith(variant+'-'):
                    continue
                rows=[v['actors'][name] for v in values.values()]
                mean=sum((Fraction(r['value']) for r in rows),Fraction())/len(rows)
                source=Path(path).read_text()
                candidates.append((-mean,source.count('(rule '),len(source),sha(path),name,path))
            winner=min(candidates)
            selected[variant]=winner[-1]
            detail[variant]={'candidate':winner[-2],'path':winner[-1],'dev_value':str(-winner[0]),'sha256':winner[-3]}
        gym.atomic(self.output/(phase+'-selection.json'),detail)
        return selected

    def run(self):
        roots=self.roots()
        subset=lambda split:{k:v for k,v in roots.items() if v['split']==split}
        a=self.evaluate('discovery-a',subset('train-a'),{'lowest':self.lowest},True)
        first_paths={variant:[r['actors']['lowest']['lessons'][variant] for r in a.values()] for variant in VARIANTS}
        candidates=self.fit('fit-a',first_paths)
        chosen=self.promote('dev-a',subset('dev'),candidates)
        b=self.evaluate('discovery-b',subset('train-b'),chosen,True)
        second_paths={variant:first_paths[variant]+[r['actors'][variant]['lessons'][variant] for r in b.values()] for variant in VARIANTS}
        candidates=self.fit('fit-b',second_paths)
        chosen=self.promote('dev-b',subset('dev'),candidates)
        frozen={name:{'path':path,'sha256':sha(path)} for name,path in chosen.items()}
        gym.pin(self.output/'frozen',{'actors':frozen,'manifest':gym.digest(self.manifest)})
        testing=self.evaluate('test',subset('test'),chosen,False)
        result=report(testing,roots)
        result['frozen']=frozen
        result['manifest_sha256']=gym.digest(self.manifest)
        gym.atomic(self.output/'summary.json',result)
        self.status('complete',test_roots=len(testing))
        return result


def report(testing, roots):
    names=sorted(next(iter(testing.values()))['actors'])
    summary={'schema':'texas42-relational-results-v1','root_groups':len(testing),'arms':{},'paired':{}}
    for name in names:
        rows=[testing[k]['actors'][name] for k in sorted(testing)]
        mean=lambda key:str(sum((Fraction(r[key]) for r in rows),Fraction())/len(rows))
        summary['arms'][name]={'value':mean('value'),'policy_regret':mean('policy_regret'),
            'root_regret':mean('root_regret'),'first_fallback_probability':mean('first_fallback_probability'),
            'digests':sorted({r['digest'] for r in rows}),
            'inference_work':sum(r['inference_work'] for r in rows)}
    for name in names:
        if name=='table': continue
        differences=[Fraction(testing[k]['actors'][name]['value'])-Fraction(testing[k]['actors']['table']['value']) for k in sorted(testing)]
        rng=random.Random(420907)
        samples=sorted(float(sum((rng.choice(differences) for _ in differences),Fraction())/len(differences)) for _ in range(2000))
        summary['paired'][name+'-minus-table']={'mean':str(sum(differences,Fraction())/len(differences)),
            'bootstrap_95_percentile':[samples[50],samples[1949]],
            'positive_roots':sum(d>0 for d in differences),'negative_roots':sum(d<0 for d in differences),
            'interpretation':'descriptive paired root bootstrap; narrow conditional endgame target'}
    return summary


def main(argv=None):
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--output',type=Path,required=True)
    p.add_argument('--binary',type=Path,default=ROOT/'walt/target/release/relational_lab')
    p.add_argument('--seed',type=int,default=94000000)
    p.add_argument('--train',type=int,default=24)
    p.add_argument('--dev',type=int,default=12)
    p.add_argument('--test',type=int,default=24)
    p.add_argument('--tiles',type=int,default=3)
    p.add_argument('--max-worlds',type=int,default=512)
    p.add_argument('--field',choices=['gym','l0-8'],default='gym')
    p.add_argument('--samples',type=int,default=16)
    p.add_argument('--work',type=int,default=2000000)
    p.add_argument('--clauses',type=int,default=3)
    p.add_argument('--beam',type=int,default=6)
    p.add_argument('--workers',type=int,default=10)
    p.add_argument('--seconds',type=float,default=285)
    p.add_argument('--timeout',type=float,default=180)
    args=p.parse_args(argv)
    if not (1<=args.workers<=10 and 0<args.seconds<=290 and math.isfinite(args.seconds) and 0<args.timeout<=290):
        p.error('workers 1..10; finite wall and job limits <=290 seconds')
    if min(args.train,args.dev,args.test)<1 or max(args.train,args.dev,args.test)>1000 or not 0<=args.seed<2**64-4000000:
        p.error('invalid group counts or seed range')
    campaign=None
    old={}
    try:
        campaign=Campaign(args)
        with gym.run_lock(campaign.output):
            for sig in (signal.SIGINT,signal.SIGTERM,signal.SIGHUP):
                old[sig]=signal.signal(sig,lambda _s,_f:campaign.kill())
            result=campaign.run()
            print(json.dumps(result,sort_keys=True))
        return 0
    except InterruptedError as e:
        if campaign: campaign.status('incomplete',reason=str(e))
        print(str(e),file=sys.stderr)
        return 75
    except Exception as e:
        if campaign: campaign.kill(); campaign.status('failed',reason=str(e))
        print(type(e).__name__+': '+str(e),file=sys.stderr)
        return 2
    finally:
        for sig,handler in old.items(): signal.signal(sig,handler)


if __name__=='__main__': sys.exit(main())
