#!/usr/bin/env python3
"""Conditional, resumable same-position/different-hidden-hands experiment.

Select positions by public opportunity only, before inspecting paired outcomes.
Replay both deployed procedures from each sampled complete deal. Persist each
first public-information decision to define a consistent frozen realization of
the budgeted players. Live latency belongs to the separate full-game arena.
"""
import argparse
from collections import Counter
from dataclasses import asdict
from fractions import Fraction
import json
import os
from pathlib import Path
import random
import threading

import campaign as c
import gym
from matchup import Player
from partner_review import offers
from player import decide
from rules import information_state, replay_record
from runtime import DecisionSession


def select_roots(source, maximum=12):
    spec=c.load(source)
    assert len(c.complete_results(source,spec))==spec['count'], 'source panel must be complete'
    roots=[]
    for seed in range(spec['start'],spec['start']+spec['count']):
        # In this arm A defends, where its review is disabled. Both declaring
        # seats are unmodified baseline L1. No selection by changed choice/value.
        snap=c.read(source/'seeds'/str(seed)/'defending'/'checkpoint.json')
        fixture=c.fixture(spec,seed)
        record=[]
        for item in snap['decisions']:
            seat=item['seat'];response=item['response']
            req=dict(decl=fixture['decl'],bid=30,bidder=fixture['bidder'],seat=seat,
                     hand=fixture['hands'][seat],plays=record.copy(),seed=c.PUBLIC_SEED)
            targets=offers(req,information_state(req))
            if targets and response['choice'] not in targets:
                worlds=sorted(gym.compatible_worlds(req))
                if len(worlds)<=400:
                    roots.append(dict(source_seed=seed,source_ply=len(record)//2,request=req,
                                      baseline_choice=response['choice'],offers=targets,
                                      worlds=worlds,source_checkpoint_sha256=gym.file_hash(
                                          source/'seeds'/str(seed)/'defending'/'checkpoint.json')))
                    break
            record.extend([seat,response['choice']])
        if len(roots)==maximum:break
    return roots


class Experiment:
    def __init__(self,source,output):
        self.output=output
        self.configs=[Player('l1-default'),Player('l1-partner-count-review',review='partner-count')]
        self.manifest=dict(schema='sunshine-conditional-worlds-v1',source=str(source.resolve()),
                           source_manifest=gym.file_hash(source/'manifest.json'),
                           sources={p:gym.file_hash(c.HERE/p) for p in
                                    ('sunshine_worlds.py','gym.py','runtime.py')},
                           identities=c.identities(),players=[asdict(p) for p in self.configs],
                           prior='uniform mechanical root fiber; no pre-root behavioral weighting',
                           sampling='16 draws with replacement per root; seed 20260913 + root index',
                           selection='first eligible baseline-arm coordinate per source deal, ordered seed/ply; first 12 deals',
                           policy='first realized own/public decision persisted by exact input and configuration',
                           roots=select_roots(source))
        gym.pin(output,self.manifest)
        self.lock=threading.Lock();self.locks={}

    def decision(self,request,config,session):
        identity=dict(request=request,player=asdict(config))
        key=c.digest(identity)
        with self.lock: lock=self.locks.setdefault(key,threading.Lock())
        with lock:
            path=self.output/'decisions'/(key+'.json')
            previous=c.read(path)
            if previous is not None:
                assert previous['identity']==identity
                assert c.digest(previous['response'])==previous['response_sha256']
                return previous['response'],key
            response=decide(request,**config.kwargs(),session=session)
            assert not response['over_budget'], 'decision exceeded overall deadline'
            state=information_state(request)
            assert response['choice'] in state['legal']
            c.atomic(path,dict(identity=identity,response=response,response_sha256=c.digest(response)))
            return response,key

    def jobs(self):
        jobs=[]
        for i,root in enumerate(self.manifest['roots']):
            rng=random.Random(20260913+i)
            for draw in range(16):
                jobs.append(dict(id=f'{i:02d}-{draw:02d}',root=i,draw=draw,
                                 world=rng.randrange(len(root['worlds']))))
        # Interleave roots so parallel worlds of one root can reuse decisions
        # without monopolizing the worker pool while waiting for the same key.
        return sorted(jobs,key=lambda j:(j['draw'],j['root']))

    def play(self,job):
        root=self.manifest['roots'][job['root']]
        req=root['request'];world=root['worlds'][job['world']]
        played=[set() for _ in range(4)]
        for s,t in zip(req['plays'][::2],req['plays'][1::2]):played[s].add(t)
        hands=[sorted(set(world[s])|played[s]) for s in range(4)]
        assert hands[req['seat']]==sorted(req['hand'])
        arms=[]
        with DecisionSession() as session:
            for declaring in self.configs:
                record=req['plays'].copy();decision_ids=[]
                while len(record)<56:
                    _,lead,_,trick=replay_record(hands,record,req['decl'],req['bidder'])
                    seat=(lead+len(trick))%4
                    config=declaring if seat%2==req['bidder']%2 else self.configs[0]
                    request=dict(decl=req['decl'],bid=30,bidder=req['bidder'],seat=seat,
                                 hand=hands[seat],plays=record.copy(),seed=c.PUBLIC_SEED)
                    response,key=self.decision(request,config,session)
                    record.extend([seat,response['choice']]);decision_ids.append(key)
                points,_,remaining,trick=replay_record(hands,record,req['decl'],req['bidder'])
                assert sum(points)==42 and not trick and all(not h for h in remaining)
                arms.append(dict(player=declaring.name,record=record,points=points,
                                 made=points[req['bidder']%2]>=30,decisions=decision_ids))
        payload=dict(**job,hands=hands,arms=arms,delta=int(arms[1]['made'])-int(arms[0]['made']))
        return dict(payload=payload,payload_sha256=c.digest(payload))

    def report(self):
        rows=[];hashes={};used=set()
        for job in self.jobs():
            path=self.output/'items'/(job['id']+'.json');value=c.read(path)
            if value is None:return None
            row=value['payload'];assert c.digest(row)==value['payload_sha256']
            assert all(row[k]==v for k,v in job.items())
            root=self.manifest['roots'][job['root']];req=root['request']
            for arm in row['arms']:
                assert arm['record'][:len(req['plays'])]==req['plays']
                points,_,remaining,trick=replay_record(row['hands'],arm['record'],req['decl'],req['bidder'])
                assert len(arm['record'])==56 and sum(points)==42 and not trick
                assert all(not h for h in remaining) and points==arm['points']
                assert arm['made']==(points[req['bidder']%2]>=30)
                for n,key in enumerate(arm['decisions']):
                    dpath=self.output/'decisions'/(key+'.json');d=c.read(dpath)
                    assert c.digest(d['identity'])==key
                    assert c.digest(d['response'])==d['response_sha256']
                    request=d['identity']['request'];ply=len(req['plays'])//2+n
                    actor,tile=arm['record'][2*ply:2*ply+2]
                    assert request==dict(decl=req['decl'],bid=30,bidder=req['bidder'],seat=actor,
                                         hand=row['hands'][actor],plays=arm['record'][:2*ply],seed=c.PUBLIC_SEED)
                    expected=self.configs[1] if arm['player']==self.configs[1].name and actor%2==req['bidder']%2 else self.configs[0]
                    assert d['identity']['player']==asdict(expected)
                    assert d['response']['choice']==tile
                    assert not d['response']['over_budget']
                    used.add(key);hashes[str(dpath)]=gym.file_hash(dpath)
            assert row['delta']==int(row['arms'][1]['made'])-int(row['arms'][0]['made'])
            hashes[str(path)]=gym.file_hash(path);rows.append(row)
        groups=[]
        for i,root in enumerate(self.manifest['roots']):
            group=[r for r in rows if r['root']==i]
            assert len(group)==16
            counts=Counter(r['delta'] for r in group)
            groups.append(dict(root=i,source_seed=root['source_seed'],source_ply=root['source_ply'],
                               worlds=len(root['worlds']),sampled=16,unique_sampled=len({r['world'] for r in group}),
                               delta=str(Fraction(sum(r['delta'] for r in group),16)),
                               wins=counts[1],losses=counts[-1],ties=counts[0]))
        result=dict(schema='sunshine-conditional-results-v1',manifest=self.manifest,groups=groups,
                    paired_worlds=len(rows),full_game_replays=2*len(rows),unique_decisions=len(used),
                    decision_uses=sum(len(a['decisions']) for r in rows for a in r['arms']),
                    mean_root_delta=str(sum((Fraction(g['delta']) for g in groups),Fraction())/len(groups)) if groups else None,
                    scope='Detector-positive selected source positions; finite sampled worlds; frozen deployed procedures; not population strength or live latency.',
                    artifact_hashes=hashes)
        c.atomic(self.output/'report.json',result)
        return result


def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('source',type=Path);p.add_argument('output',type=Path)
    p.add_argument('--workers',type=int,default=10);p.add_argument('--seconds',type=int,default=260)
    args=p.parse_args()
    if not 1<=args.workers<=10 or not 50<=args.seconds<=260:raise ValueError('worker/time bounds')
    os.environ['WALT_RAYON_THREADS']='2'
    args.output.mkdir(parents=True,exist_ok=True)
    with gym.run_lock(args.output):
        experiment=Experiment(args.source,args.output)
        gym.bounded(experiment.jobs(),experiment.play,args.output,args.workers,args.seconds,45)
        result=experiment.report()
        print(json.dumps({k:result[k] for k in ('paired_worlds','full_game_replays','unique_decisions','decision_uses','mean_root_delta','groups')}) if result else 'incomplete: resume unchanged')


if __name__=='__main__':main()
