#!/usr/bin/env python3
"""Sample complete action contrasts at all six decision-bearing hand sizes.

The queue unit is one sampled world and *all* legal root actions. Source moves
are preserved; the shared native player owns every ordinary continuation.
"""
import argparse
import asyncio
from collections import Counter, defaultdict
from functools import lru_cache
import hashlib
import json
import os
from pathlib import Path
import shutil
import signal
import sqlite3
import tarfile
import time
import zlib

import played as p
import gym
from rules import called, context, follows, information_state, replay_record
from scheme_mine import immutable_json

HERE=Path(__file__).resolve().parent
PROTOCOL=HERE/'WHOLE-GAME-CONTRASTS-V1.md'
BINARY=p.ROOT/'walt/target/release/kiln-decision-worker'
AUDITOR=p.ROOT/'walt/target/release/kiln-scheme-contrast-worker'


def read(path):return json.loads(Path(path).read_text())
def digest(value):return gym.digest(value)


def connect(output):
    db=sqlite3.connect(output/'contrasts.sqlite',timeout=30);db.row_factory=sqlite3.Row
    db.executescript('PRAGMA journal_mode=WAL; PRAGMA synchronous=FULL; PRAGMA fullfsync=ON;')
    return db


def job_input(root,index):
    return dict(request=root['request'],queries=[],baseline=root['baseline'],
        sample_seed=p.seeded(f'whole-game-world-v1/{root["id"]}/{index}'))


def prepare(source,output,split):
    output.mkdir(parents=True,exist_ok=True)
    start=420600 if split=='train' else 420645
    db=sqlite3.connect(source.resolve().as_uri()+'/played.sqlite?mode=ro',uri=True)
    db.row_factory=sqlite3.Row
    records=db.execute('''SELECT j.id job_id,h.seed deal_seed,g.sha256,g.producer,g.payload
        FROM games g JOIN jobs j ON j.id=g.job_id JOIN cells c ON c.id=j.cell_id
        JOIN hands h ON h.id=c.hand_id WHERE j.trial=0 AND h.seed>=? AND h.seed<?
        ORDER BY h.seed,c.id''',(start,start+10)).fetchall();db.close()
    if len(records)!=360:raise ValueError('Need complete fixed 360-game source panel')
    candidates={};source_ids=[];counts=Counter()
    archive=sqlite3.connect(output/'source-receipts.sqlite')
    archive.execute('CREATE TABLE IF NOT EXISTS receipts(job_id INTEGER PRIMARY KEY,sha256 TEXT,producer TEXT,payload BLOB)')
    with archive:
        for row in records:
            raw=zlib.decompress(row['payload']);assert hashlib.sha256(raw).hexdigest()==row['sha256']
            game=json.loads(raw)
            p.validate_game({k:game[k] for k in ('profile','hands','bidder','decl','seed')},game)
            archive.execute('INSERT OR IGNORE INTO receipts VALUES (?,?,?,?)',
                (row['job_id'],row['sha256'],row['producer'],row['payload']))
            source_ids.append({k:row[k] for k in ('job_id','deal_seed','sha256','producer')})
            for ply,decision in enumerate(game['decisions']):
                req=decision['call']['request'];size=7-req['plays'][::2].count(req['seat'])
                counts['inspected']+=1
                if size<2:counts['last_tile']+=1;continue
                if req['seat']%2!=req['bidder']%2:counts['defending']+=1;continue
                state=information_state(req)
                if state['points'][req['bidder']%2]>=30 or state['points'][1-req['bidder']%2]>12:
                    counts['settled']+=1;continue
                if len(state['legal'])<2:counts['forced']+=1;continue
                identity=digest({k:v for k,v in req.items() if k!='seed'})
                candidates.setdefault(identity,dict(id=identity,request=req,baseline=decision['response']['choice'],
                    original_decision=decision,deal_seed=row['deal_seed'],own_remaining=size,job_id=row['job_id'],
                    source_sha256=row['sha256'],source_producer=row['producer'],ply=ply))
    archive.close()
    selected=[];coverage=[]
    for source_seed in range(start,start+10):
        for size in range(7,1,-1):
            pool=[r for r in candidates.values() if r['deal_seed']==source_seed and r['own_remaining']==size]
            pool.sort(key=lambda r:digest(['whole-game-select-v1',r['id']]))
            selected.extend(pool[:2]);coverage.append(dict(deal=source_seed,size=size,eligible=len(pool),selected=min(2,len(pool))))
    panel=dict(schema='whole-game-panel-v1',split=split,protocol_sha256=p.digest(PROTOCOL),source=str(source),
        source_receipts=source_ids,counts=dict(counts),coverage=coverage,roots=selected,
        eligible=[{k:r[k] for k in ('id','deal_seed','own_remaining','source_sha256','ply')} for r in candidates.values()])
    panel['id']=digest(panel);immutable_json(output/'panel.json',panel)
    queue=connect(output)
    queue.executescript('''CREATE TABLE IF NOT EXISTS jobs(id TEXT PRIMARY KEY,root TEXT,world_index INTEGER,
        input TEXT NOT NULL,payload BLOB,sha256 TEXT,error TEXT,attempts INTEGER NOT NULL DEFAULT 0,
        UNIQUE(root,world_index));''')
    with queue:
        for root in selected:
            for index in range(8):
                queue.execute('INSERT OR IGNORE INTO jobs(id,root,world_index,input) VALUES (?,?,?,?)',
                    (f'{root["id"]}-{index:03d}',root['id'],index,p.canonical(job_input(root,index))))
    queue.close()
    return dict(roots=len(selected),eligible=len(candidates),coverage=coverage,panel=panel['id'])


@lru_cache(maxsize=1024)
def support_count(serialized):
    """Independent tile-by-tile capacity DP, without enumerating assignments."""
    req=json.loads(serialized);information_state(req)
    played=set(req['plays'][1::2]);own=set(req['hand'])-played
    others=[s for s in range(4) if s!=req['seat']]
    capacities=tuple(7-req['plays'][::2].count(s) for s in others)
    forbidden=[set() for _ in range(4)];trick=[]
    for s,t in zip(req['plays'][::2],req['plays'][1::2]):
        if trick:
            led=context(trick[0],req['decl'])
            if not follows(t,led,req['decl']):forbidden[s].add(led)
        trick.append(t)
        if len(trick)==4:trick=[]
    states={(0,0,0):1}
    for tile in sorted(set(range(28))-played-own):
        allowed=[i for i,s in enumerate(others) if not any(follows(tile,q,req['decl']) for q in forbidden[s])]
        next_states=defaultdict(int)
        for sizes,n in states.items():
            for i in allowed:
                if sizes[i]<capacities[i]:
                    after=list(sizes);after[i]+=1;next_states[tuple(after)]+=n
        states=next_states
    return states.get(capacities,0)


def validate(job,value):
    req=job['request'];info=information_state(req)
    assert value['schema']=='kiln-sampled-contrast-v1' and value['request']==req
    assert value['sample_seed']==job['sample_seed'] and value['baseline']==job['baseline']
    assert value['baseline_decision'] is None and value['queries']=={}
    assert value['legal']==info['legal'] and len(value['worlds'])==1
    assert int(value['support_worlds'])==support_count(p.canonical(req))>0
    hands=value['worlds'][0];assert hands[req['seat']]==req['hand']
    replay_record(hands,req['plays'],req['decl'],req['bidder'])
    if job.get('sample_only',False):
        assert value['sample_only'] and not value['decisions'] and not value['traces'];return {}
    assert not value['sample_only']
    assert sorted(t['action'] for t in value['traces'])==info['legal']
    used=set();moves=0
    for decision in value['decisions']:
        call=decision['call'];r=call['request'];response=decision['response']
        assert set(r)==gym.INPUT_KEYS and call==dict(request=r,worlds=40,partner=True,budget_ms=14000)
        assert (r['decl'],r['bid'],r['bidder'],r['seed'])==(req['decl'],30,req['bidder'],req['seed'])
        state=information_state(r)
        assert response['player_version']=='walt-table-v2' and response['legal']==state['legal']
        assert response['choice'] in state['legal'] and response['points']==state['points'] and response['leader']==state['leader']
    prefix=len(req['plays'])
    for trace in value['traces']:
        assert trace['world']==0;record=trace['record']
        assert len(record)==56 and record[:prefix+2]==req['plays']+[req['seat'],trace['action']]
        assert len(trace['decisions'])==(56-prefix)//2-1
        for ply,index in enumerate(trace['decisions'],prefix//2+1):
            s,t=record[2*ply:2*ply+2];d=value['decisions'][index]
            assert d['call']['request']==dict(req,seat=s,hand=hands[s],plays=record[:2*ply])
            assert d['response']['choice']==t;used.add(index);moves+=1
        points,_,remaining,trick=replay_record(hands,record,req['decl'],req['bidder'])
        assert not any(remaining) and not trick and sum(points)==42
        assert trace['points']==points and trace['made']==(points[req['bidder']%2]>=30)
    assert used==set(range(len(value['decisions'])))
    return dict(samples=1,branches=len(value['traces']),continuation_moves=moves,decisions=len(used),
        over_budget=sum(bool(d['response']['over_budget']) for d in value['decisions']),
        fallbacks=sum(d['response']['route'] not in ('forced','baseline','baseline-reviewed') for d in value['decisions']))


def preserve(output):
    paths=sorted([*p.ROOT.glob('walt/walt/src/**/*.rs'),*p.ROOT.glob('walt/walt-player/src/**/*.rs'),
        p.ROOT/'walt/Cargo.toml',p.ROOT/'walt/Cargo.lock',p.ROOT/'walt/walt-player/Cargo.toml',
        Path(__file__),PROTOCOL,HERE/'played.py',HERE/'kiln.py',HERE/'scheme_mine.py',HERE/'threat_probe.py',
        p.ROOT/'experiments/partnership/rules.py',p.ROOT/'experiments/partnership/gym.py'])
    identity=dict(binary_sha256=p.digest(BINARY),auditor_sha256=p.digest(AUDITOR),
        sources={str(f.relative_to(p.ROOT)):p.digest(f) for f in paths})
    immutable_json(output/'implementation.json',identity)
    folder=output/'producer';folder.mkdir(exist_ok=True)
    for source,name in ((BINARY,'worker'),(AUDITOR,'auditor')):
        if not (folder/name).exists():shutil.copy2(source,folder/name)
        assert p.digest(folder/name)==p.digest(source)
    if not (folder/'sources.tar.gz').exists():
        with tarfile.open(folder/'sources.tar.gz','w:gz') as archive:
            for path in paths:archive.add(path,arcname=str(path.relative_to(p.ROOT)),recursive=False)
    return folder/'worker'


async def run(output,workers,seconds):
    with gym.run_lock(output):
        panel=read(output/'panel.json')
        if panel['split']=='fresh':
            fit=read(output.parent/'fit.json');assert fit['id']==digest({k:v for k,v in fit.items() if k!='id'})
            immutable_json(output/'exam-fit.json',dict(id=fit['id']))
        binary=preserve(output)
        if panel['split']=='fresh':assert read(output/'implementation.json')==fit['implementation']
        db=connect(output)
        todo=iter(db.execute('SELECT * FROM jobs WHERE payload IS NULL ORDER BY world_index,root').fetchall())
        stop=asyncio.Event();loop=asyncio.get_running_loop();started=time.monotonic();end=started+seconds
        for sig in (signal.SIGINT,signal.SIGTERM):loop.add_signal_handler(sig,stop.set)
        completed=errors=0
        async def worker(i):
            nonlocal completed,errors
            proc=None
            with (output/f'worker-{i}.stderr.log').open('ab') as err:
                try:
                    while not stop.is_set() and time.monotonic()<end:
                        job=next(todo,None)
                        if job is None:return
                        with db:db.execute('UPDATE jobs SET attempts=attempts+1 WHERE id=?',(job['id'],))
                        try:
                            if proc is None:
                                proc=await asyncio.create_subprocess_exec(str(binary),stdin=asyncio.subprocess.PIPE,
                                    stdout=asyncio.subprocess.PIPE,stderr=err,limit=64*1024*1024,
                                    env={**os.environ,'RAYON_NUM_THREADS':'1'})
                            proc.stdin.write((job['input']+'\n').encode());await proc.stdin.drain()
                            value=json.loads(await asyncio.wait_for(proc.stdout.readline(),120))
                            if 'error' in value:raise ValueError(value['error'])
                            value['independent_audit']=validate(json.loads(job['input']),value)
                            raw=p.canonical(value).encode()
                            with db:db.execute('UPDATE jobs SET payload=?,sha256=?,error=NULL WHERE id=?',
                                (zlib.compress(raw),hashlib.sha256(raw).hexdigest(),job['id']))
                            completed+=1
                        except asyncio.CancelledError:raise
                        except Exception as error:
                            with db:db.execute('UPDATE jobs SET error=? WHERE id=?',(repr(error),job['id']))
                            errors+=1;print(p.canonical(dict(job=job['id'],error=repr(error))),flush=True)
                            if proc and proc.returncode is None:proc.kill();await proc.wait()
                            proc=None
                finally:
                    if proc and proc.returncode is None:proc.kill();await proc.wait()
        tasks=[asyncio.create_task(worker(i)) for i in range(workers)]
        try:
            last=started
            while not all(t.done() for t in tasks) and not stop.is_set() and time.monotonic()<end:
                await asyncio.sleep(.2)
                if time.monotonic()-last>=10:
                    status=dict(completed=completed,errors=errors,seconds=round(time.monotonic()-started,2))
                    p.atomic_json(output/'status.json',status);print(p.canonical(status),flush=True);last=time.monotonic()
        finally:
            for t in tasks:t.cancel()
            statuses=await asyncio.gather(*tasks,return_exceptions=True)
            result=dict(completed=completed,errors=errors,seconds=round(time.monotonic()-started,3),
                total=db.execute('SELECT COUNT(*) FROM jobs WHERE payload IS NOT NULL').fetchone()[0],
                pending=db.execute('SELECT COUNT(*) FROM jobs WHERE payload IS NULL').fetchone()[0])
            p.atomic_json(output/'status.json',result);db.execute('PRAGMA wal_checkpoint(FULL)');db.close()
            for sig in (signal.SIGINT,signal.SIGTERM):loop.remove_signal_handler(sig)
            failure=next((s for s in statuses if isinstance(s,Exception)),None)
            if failure:raise failure
        return result


def receipts(output,depth=8):
    panel=read(output/'panel.json');assert panel['id']==digest({k:v for k,v in panel.items() if k!='id'})
    roots={r['id']:r for r in panel['roots']};db=connect(output)
    jobs=db.execute('SELECT * FROM jobs WHERE world_index<? ORDER BY root,world_index',(depth,)).fetchall();db.close()
    expected={(rid,i) for rid in roots for i in range(min(depth,8))}
    if depth>8 and (output/'refinement.json').exists():
        refinement=read(output/'refinement.json');assert refinement['source_panel']==panel['id']
        expected|={(rid,i) for rid in refinement['roots'] for i in range(8,min(depth,24))}
    if len(jobs)!=len(expected) or {(j['root'],j['world_index']) for j in jobs}!=expected:
        raise ValueError('Queue does not cover the complete panel')
    if any(j['payload'] is None for j in jobs):raise ValueError('Requested prefix is incomplete')
    for j in jobs:
        raw=zlib.decompress(j['payload']);assert hashlib.sha256(raw).hexdigest()==j['sha256']
        value=json.loads(raw);root=roots[j['root']]
        assert json.loads(j['input'])==job_input(root,j['world_index'])
        assert value['request']==root['request'] and value['baseline']==root['baseline']
        yield root,j['world_index'],value,j['sha256']


def rows(output,depth=8):
    answer=[]
    for root,index,value,sha in receipts(output,depth):
        outcomes={t['action']:int(t['made']) for t in value['traces']}
        for alt in value['legal']:
            if alt==root['baseline']:continue
            answer.append(dict(root=root['id'],request=root['request'],size=root['own_remaining'],
                deal=root['deal_seed'],sample=index,hands=value['worlds'][0],baseline=root['baseline'],
                alternative=alt,delta=outcomes[alt]-outcomes[root['baseline']],receipt_sha256=sha))
    return answer


def refine(output):
    assert read(output/'panel.json')['split']=='train'
    roots={r['id']:r for r in read(output/'panel.json')['roots']};data=rows(output)
    groups=defaultdict(list)
    for row in data:groups[row['root'],row['alternative']].append(row['delta'])
    selected=[]
    for size in range(7,1,-1):
        candidates=[]
        for rid,root in roots.items():
            if root['own_remaining']!=size:continue
            values=[v for (r,_),v in groups.items() if r==rid]
            discordance=sum(sum(abs(d) for d in v) for v in values)
            if not discordance:continue
            candidates.append((max(sum(v) for v in values),discordance,digest(['refine',rid]),rid))
        candidates.sort(key=lambda r:(-r[0],-r[1],r[2]));selected.extend(r[-1] for r in candidates[:4])
    record=dict(schema='whole-game-refinement-v1',roots=selected,depth=24,source_panel=read(output/'panel.json')['id'])
    immutable_json(output/'refinement.json',record)
    db=connect(output)
    with db:
        for rid in selected:
            for i in range(8,24):
                db.execute('INSERT OR IGNORE INTO jobs(id,root,world_index,input) VALUES (?,?,?,?)',
                    (f'{rid}-{i:03d}',rid,i,p.canonical(job_input(roots[rid],i))))
    db.close();return dict(roots=len(selected),new_world_jobs=len(selected)*16)


def audit(output):
    identity=read(output/'implementation.json')
    assert p.digest(output/'producer/worker')==identity['binary_sha256']
    assert p.digest(output/'producer/auditor')==identity['auditor_sha256']
    with tarfile.open(output/'producer/sources.tar.gz','r:gz') as archive:
        for name,sha in identity['sources'].items():assert hashlib.sha256(archive.extractfile(name).read()).hexdigest()==sha
    total=Counter()
    for root,index,value,_ in receipts(output,24 if (output/'refinement.json').exists() else 8):
        checked=validate(job_input(root,index),value);assert checked==value['independent_audit'];total.update(checked)
    return dict(verified=True,totals=dict(total))


if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__);sub=parser.add_subparsers(dest='command',required=True)
    q=sub.add_parser('prepare');q.add_argument('source',type=Path);q.add_argument('output',type=Path);q.add_argument('--split',choices=['train','fresh'],required=True)
    q=sub.add_parser('run');q.add_argument('output',type=Path);q.add_argument('--workers',type=int,default=18);q.add_argument('--seconds',type=float,default=60)
    for cmd in ('refine','audit'):
        q=sub.add_parser(cmd);q.add_argument('output',type=Path)
    args=parser.parse_args()
    if args.command=='prepare':result=prepare(args.source,args.output,args.split)
    elif args.command=='run':
        if not 1<=args.workers<=24 or not 1<=args.seconds<=240:parser.error('workers 1..24, seconds 1..240')
        result=asyncio.run(run(args.output,args.workers,args.seconds))
    elif args.command=='refine':result=refine(args.output)
    else:result=audit(args.output)
    print(p.canonical(result),flush=True)
