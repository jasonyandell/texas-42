#!/usr/bin/env python3
"""Two-tile outcome contrasts, Scheme descriptions and a frozen fresh exam.

See DECISION-MINING-V1.md. Python >=3.10; complete roots are durable SQLite jobs.
No hidden actual deal is sent to the assessment worker, let alone the player.
"""
import argparse
import asyncio
from collections import Counter, defaultdict
from fractions import Fraction as F
import hashlib
from itertools import combinations
import json
import os
from pathlib import Path
import random
import shutil
import signal
import sqlite3
import subprocess
import tarfile
import time
import zlib

import played as p
import gym
from gym_replay import full_hands
from rules import TILES, called, context, follows, information_state, replay_record
from scheme_mine import immutable_json, key

HERE = Path(__file__).resolve().parent
BINARY = p.ROOT/'walt/target/release/kiln-decision-worker'
PROTOCOL = HERE/'DECISION-MINING-V1.md'
ATOM_NAMES = ('trump','!trump','double','!double','count=0','count=5','count=10','boss','!boss')


def read(path):
    return json.loads(Path(path).read_text())


def hashed(value):
    return gym.digest(value)


def queries():
    answer = []
    for size in (1,2):
        for props in combinations(ATOM_NAMES,size):
            roles, atoms = ['(domino a)'], ['(own-legal a)']
            if 'boss' in props or '!boss' in props:
                roles += ['(context led)']; atoms += ['(leads-context a led)']
            for prop in props:
                positive = prop.lstrip('!')
                atom = '(in a q*)' if positive=='trump' else '(double a)' if positive=='double' else \
                    '(boss a led)' if positive=='boss' else f'(count a {positive.split("=")[1]})'
                atoms.append(f'(not {atom})' if prop.startswith('!') else atom)
            source = '(fix (roles '+' '.join(roles)+') (out a) (case '+' '.join(atoms)+'))\n'
            answer.append(dict(name='action-'+str(len(answer)),props=list(props),source=source))
    for name in ('relational-0','relational-1'):
        answer.append(dict(name=name,source=(HERE/'mining-v2'/f'{name}.scheme').read_text()))
    return answer


def job_queries():
    return [{k:q[k] for k in ('name','source')} for q in queries()]


def action_props(req, action):
    live = set(range(28))-set(req['plays'][1::2])
    decl,led = req['decl'],context(action,req['decl'])
    boss = follows(action,led,decl) and not any(key(t,led,decl)>key(action,led,decl) for t in live)
    hi,lo = TILES[action]
    return {'trump' if called(action,decl) else '!trump','double' if hi==lo else '!double',
            f'count={hi+lo if hi+lo in (5,10) else 0}','boss' if boss else '!boss'}


def query_expected(query, req, hands):
    state = information_state(req)
    if 'props' in query:
        actions = [a for a in state['legal'] if set(query['props'])<=action_props(req,a)]
        return dict(exists=bool(actions),actions=actions)
    live = set(range(28))-set(req['plays'][1::2])
    top = [t for t in live if called(t,req['decl']) and not any(
        key(o,'trump',req['decl'])>key(t,'trump',req['decl']) for o in live)]
    holder = (req['seat']+(3 if query['name']=='relational-0' else 1))%4
    return dict(exists=bool(set(top)&set(hands[holder])),actions=[])


def prepare(source, output, split):
    output.mkdir(parents=True,exist_ok=True)
    start = 420600 if split=='train' else 420635
    db = sqlite3.connect(f'file:{source / "played.sqlite"}?mode=ro',uri=True)
    db.row_factory = sqlite3.Row
    records = db.execute('''SELECT j.id job_id,j.trial,h.seed deal_seed,g.sha256,g.producer,g.payload
        FROM games g JOIN jobs j ON j.id=g.job_id JOIN cells c ON c.id=j.cell_id
        JOIN hands h ON h.id=c.hand_id WHERE j.trial=0 AND h.seed>=? AND h.seed<?
        ORDER BY h.seed,c.id''',(start,start+10)).fetchall()
    db.close()
    if len(records)!=360: raise ValueError('Need complete fixed 360-game source panel')
    found, source_ids, counts = {}, [], Counter()
    archive = sqlite3.connect(output/'source-receipts.sqlite')
    archive.execute('CREATE TABLE IF NOT EXISTS receipts(job_id INTEGER PRIMARY KEY,sha256 TEXT,producer TEXT,payload BLOB)')
    with archive:
        for row in records:
            raw = zlib.decompress(row['payload'])
            if hashlib.sha256(raw).hexdigest()!=row['sha256']: raise ValueError('Source receipt hash')
            value = json.loads(raw)
            p.validate_game({k:value[k] for k in ('profile','hands','bidder','decl','seed')},value)
            archive.execute('INSERT OR IGNORE INTO receipts VALUES (?,?,?,?)',
                (row['job_id'],row['sha256'],row['producer'],row['payload']))
            source_ids.append({k:row[k] for k in ('job_id','deal_seed','sha256','producer')})
            for ply in range(20,24):
                req = value['decisions'][ply]['call']['request']
                state = information_state(req)
                counts['inspected']+=1
                if req['seat']%2!=req['bidder']%2: counts['defending']+=1; continue
                if state['points'][req['bidder']%2]>=30 or state['points'][1-req['bidder']%2]>12:
                    counts['settled']+=1; continue
                if len(state['legal'])!=2: counts['forced']+=1; continue
                identity = hashed({k:v for k,v in req.items() if k!='seed'})
                found.setdefault(identity,dict(id=identity,request=req,deal_seed=row['deal_seed'],
                    job_id=row['job_id'],source_sha256=row['sha256'],ply=ply))
    archive.close()
    selected = []
    for seed in range(start,start+10):
        pool = [v for v in found.values() if v['deal_seed']==seed]
        pool.sort(key=lambda r:hashed(['decision-mine-select-v1',r['id']]))
        selected.extend(pool[:12])
    ledger = dict(schema='kiln-decision-panel-v1',split=split,protocol_sha256=p.digest(PROTOCOL),
        source=str(source),source_receipts=source_ids,counts=dict(counts),eligible=list(found.values()),
        roots=selected,queries=queries())
    ledger['id']=hashed(ledger)
    immutable_json(output/'panel.json',ledger)
    queue = sqlite3.connect(output/'assessments.sqlite')
    queue.executescript('''PRAGMA journal_mode=WAL; PRAGMA synchronous=FULL;
        CREATE TABLE IF NOT EXISTS jobs(id TEXT PRIMARY KEY,root TEXT,rep INTEGER,input TEXT,
            payload BLOB,sha256 TEXT,error TEXT);
        CREATE TABLE IF NOT EXISTS meta(key TEXT PRIMARY KEY,value TEXT NOT NULL);''')
    with queue:
        for root in selected:
            for rep in range(2):
                req = dict(root['request'])
                if rep: req['seed']=p.seeded('decision-policy-v1/'+root['id'])&0xffffffff
                queue.execute('INSERT OR IGNORE INTO jobs(id,root,rep,input) VALUES (?,?,?,?)',
                    (root['id']+f'-{rep}',root['id'],rep,p.canonical(dict(request=req,queries=job_queries()))))
    queue.close()
    return dict(roots=len(selected),eligible=len(found),counts=dict(counts),panel=ledger['id'])


def validate(job, result):
    req = job['request']
    assert result['schema']=='kiln-decision-assessment-v1' and result['request']==req
    state = information_state(req)
    assert result['legal']==state['legal'] and len(state['legal'])==2
    worlds = sorted(gym.world_key(full_hands(req,w)) for w in gym.compatible_worlds(req))
    actual = [gym.world_key(w) for w in result['worlds']]
    assert len(actual)==len(set(actual)) and sorted(actual)==worlds and 0<len(worlds)<=90
    assert job['queries']==job_queries()
    assert set(result['queries'])=={q['name'] for q in queries()}
    memberships = 0
    for q in queries():
        row=result['queries'][q['name']]
        assert row['public']==('props' in q)
        assert row['memberships']==[query_expected(q,req,h) for h in actual]
        memberships += len(actual)
    for d in result['decisions']:
        call = d['call']; r = call['request']
        assert set(r)==gym.INPUT_KEYS
        assert call==dict(request=r,worlds=40,partner=True,budget_ms=14000)
        assert (r['decl'],r['bid'],r['bidder'],r['seed'])==(req['decl'],30,req['bidder'],req['seed'])
        info=information_state(r); response=d['response']
        assert response['player_version']=='walt-table-v2'
        assert response['legal']==info['legal'] and response['points']==info['points']
        assert response['leader']==info['leader'] and response['choice'] in info['legal']
    root_decision=result['decisions'][result['baseline_decision']]
    assert root_decision['call']['request']==req
    assert result['baseline']==root_decision['response']['choice']
    seen=set(); used={result['baseline_decision']}; moves=0
    for trace in result['traces']:
        w,a=trace['world'],trace['action']
        assert (w,a) not in seen and 0<=w<len(actual) and a in state['legal'];seen.add((w,a))
        record=trace['record']; prefix=len(req['plays'])
        assert len(record)==56 and record[:prefix+2]==req['plays']+[req['seat'],a]
        assert len(trace['decisions'])==(56-prefix)//2-1
        for ply,index in enumerate(trace['decisions'],prefix//2+1):
            s,t=record[2*ply:2*ply+2]; d=result['decisions'][index]
            expected=dict(req,seat=s,hand=list(actual[w][s]),plays=record[:2*ply])
            assert d['call']['request']==expected and d['response']['choice']==t
            used.add(index); moves+=1
        points,_,remaining,trick=replay_record(actual[w],record,req['decl'],req['bidder'])
        assert not any(remaining) and not trick and sum(points)==42
        assert points==trace['points'] and trace['made']==(points[req['bidder']%2]>=30)
    assert len(seen)==2*len(actual) and used==set(range(len(result['decisions'])))
    return dict(worlds=len(actual),traces=len(seen),continuation_moves=moves,query_checks=memberships,
        decisions=len(used),over_budget=sum(bool(d['response']['over_budget']) for d in result['decisions']),
        fallbacks=sum(d['response']['route'] not in ('forced','baseline','baseline-reviewed') for d in result['decisions']))


def preserve(output):
    paths = sorted([*p.ROOT.glob('walt/walt/src/**/*.rs'),*p.ROOT.glob('walt/walt-player/src/**/*.rs'),
        p.ROOT/'walt/Cargo.toml',p.ROOT/'walt/Cargo.lock',p.ROOT/'walt/walt-player/Cargo.toml',
        PROTOCOL,*p.ROOT.glob('experiments/partnership/*.py'),*HERE.glob('*.py')])
    identity=dict(binary_sha256=p.digest(BINARY),sources={str(f.relative_to(p.ROOT)):p.digest(f) for f in paths})
    immutable_json(output/'implementation.json',identity)
    parent=output/'producer';parent.mkdir(exist_ok=True)
    if not (parent/'worker').exists():shutil.copy2(BINARY,parent/'worker')
    assert p.digest(parent/'worker')==identity['binary_sha256']
    if not (parent/'sources.tar.gz').exists():
        with tarfile.open(parent/'sources.tar.gz','w:gz') as archive:
            for path in paths:archive.add(path,arcname=str(path.relative_to(p.ROOT)),recursive=False)
    return parent/'worker'


async def run(output,workers,seconds):
    with gym.run_lock(output):
        if read(output/'panel.json')['split']=='fresh':
            fit_value=read(output.parent/'fit.json')
            assert fit_value['id']==hashed({k:v for k,v in fit_value.items() if k!='id'})
            immutable_json(output/'exam-fit.json',dict(id=fit_value['id']))
        binary=preserve(output)
        if read(output/'panel.json')['split']=='fresh':
            assert read(output/'implementation.json')==fit_value['implementation']
        db=sqlite3.connect(output/'assessments.sqlite');db.row_factory=sqlite3.Row
        jobs=list(db.execute('SELECT * FROM jobs WHERE payload IS NULL ORDER BY id'))
        stop=asyncio.Event();loop=asyncio.get_running_loop()
        for sig in (signal.SIGINT,signal.SIGTERM):loop.add_signal_handler(sig,stop.set)
        todo=iter(jobs); started=time.monotonic(); end=started+seconds; done=errors=0
        async def worker(i):
            nonlocal done,errors
            proc=None
            try:
                while not stop.is_set() and time.monotonic()<end:
                    job=next(todo,None)
                    if job is None:return
                    try:
                        if proc is None:
                            proc=await asyncio.create_subprocess_exec(str(binary),stdin=asyncio.subprocess.PIPE,
                                stdout=asyncio.subprocess.PIPE,stderr=asyncio.subprocess.DEVNULL,limit=64*1024*1024,
                                env={**os.environ,'RAYON_NUM_THREADS':'1'})
                        proc.stdin.write((job['input']+'\n').encode());await proc.stdin.drain()
                        raw=await asyncio.wait_for(proc.stdout.readline(),60)
                        value=json.loads(raw)
                        if 'error' in value:raise ValueError(value['error'])
                        audit=validate(json.loads(job['input']),value)
                        value['independent_audit']=audit
                        raw=p.canonical(value).encode()
                        with db:db.execute('UPDATE jobs SET payload=?,sha256=?,error=NULL WHERE id=?',
                            (zlib.compress(raw),hashlib.sha256(raw).hexdigest(),job['id']))
                        done+=1
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
                    print(p.canonical(dict(completed=done,errors=errors,seconds=round(time.monotonic()-started,2))),flush=True)
                    last=time.monotonic()
        finally:
            for task in tasks:task.cancel()
            statuses=await asyncio.gather(*tasks,return_exceptions=True)
            failure=next((s for s in statuses if isinstance(s,Exception)),None)
            result=dict(completed=done,errors=errors,seconds=round(time.monotonic()-started,3),
                total=db.execute('SELECT COUNT(*) FROM jobs WHERE payload IS NOT NULL').fetchone()[0],
                pending=db.execute('SELECT COUNT(*) FROM jobs WHERE payload IS NULL').fetchone()[0])
            p.atomic_json(output/'status.json',result);db.execute('PRAGMA wal_checkpoint(FULL)');db.close()
            for sig in (signal.SIGINT,signal.SIGTERM):loop.remove_signal_handler(sig)
            if failure:raise failure
        return result


def rows(output):
    panel=read(output/'panel.json');roots={r['id']:r for r in panel['roots']}
    db=sqlite3.connect(output/'assessments.sqlite');db.row_factory=sqlite3.Row
    if db.execute('SELECT COUNT(*) FROM jobs WHERE payload IS NULL').fetchone()[0]:raise ValueError('Incomplete panel')
    answer=[]
    for row in db.execute('SELECT * FROM jobs ORDER BY root,rep'):
        raw=zlib.decompress(row['payload']);assert hashlib.sha256(raw).hexdigest()==row['sha256']
        v=json.loads(raw);n=len(v['worlds'])
        req=dict(roots[row['root']]['request'])
        if row['rep']:req['seed']=p.seeded('decision-policy-v1/'+row['root'])&0xffffffff
        assert json.loads(row['input'])==dict(request=req,queries=job_queries()) and v['request']==req
        vectors={a:[int(t['made']) for t in sorted(v['traces'],key=lambda t:t['world']) if t['action']==a]
                 for a in v['legal']}
        values={a:F(sum(bits),n) for a,bits in vectors.items()}
        presences={q:F(sum(m['exists'] for m in data['memberships']),n) for q,data in v['queries'].items()
                   if q.startswith('relational')}
        answer.append(dict(root=row['root'],rep=row['rep'],deal_seed=roots[row['root']]['deal_seed'],
            baseline=v['baseline'],values=values,vectors=vectors,threat=max(presences.values()),
            masks={name:data['memberships'][0]['actions'] for name,data in v['queries'].items() if data['public']},
            world_threat=[any(v['queries'][q]['memberships'][i]['exists'] for q in presences) for i in range(n)],
            audit=v['independent_audit'],sha256=row['sha256'],elapsed_us=v['elapsed_us']))
    db.close();return answer


def choose(rule,row):
    if rule is None or row['threat']<F(rule['threshold']):return row['baseline']
    actions=row['masks'][rule['query']]
    return actions[0] if len(actions)==1 else row['baseline']


def describe(rule,data):
    per_root=defaultdict(list);per_deal=defaultdict(list);per_rep=defaultdict(list);changed=set();changed_deals=set()
    for row in data:
        action=choose(rule,row);gain=row['values'][action]-row['values'][row['baseline']]
        per_root[row['root']].append(gain);per_rep[row['rep']].append(gain)
        per_deal[row['deal_seed']].append(gain)
        if action!=row['baseline']:changed.add(row['root']);changed_deals.add(row['deal_seed'])
    av=lambda xs:sum(xs,F(0))/len(xs)
    gains=[av(xs) for xs in per_root.values()]
    return dict(rule=rule,mean_gain=str(av(gains)),helped=sum(g>0 for g in gains),harmed=sum(g<0 for g in gains),
        tied=sum(g==0 for g in gains),roots=len(gains),changed_roots=len(changed),changed_deals=len(changed_deals),
        per_seed={str(k):str(av(v)) for k,v in per_rep.items()},
        per_deal={str(k):str(av(v)) for k,v in sorted(per_deal.items())})


def fit(output,destination):
    assert read(output/'panel.json')['split']=='train'
    data=rows(output);catalog=[q for q in queries() if 'props' in q]
    scores=[]
    for q in catalog:
        if not any(row['masks'][q['name']] for row in data):continue
        for threshold in ('0','1/3','2/3'):
            rule=dict(query=q['name'],threshold=threshold,props=q['props'],source=q['source'])
            scores.append(describe(rule,data))
    candidates=[s for s in scores if s['changed_roots']>=8 and s['changed_deals']>=3
                and all(F(v)>0 for v in s['per_seed'].values()) and sum(F(v)>0 for v in s['per_deal'].values())>=3]
    candidates.sort(key=lambda s:(-F(s['mean_gain']),len(s['rule']['props']),hashed(s['rule'])))
    selected=candidates[0] if candidates else describe(None,data)
    value=dict(schema='kiln-decision-fit-v1',protocol_sha256=p.digest(PROTOCOL),
        panel_id=read(output/'panel.json')['id'],implementation=read(output/'implementation.json'),
        receipt_hashes=[r['sha256'] for r in data],selected=selected,candidates=scores)
    value['id']=hashed(value);immutable_json(destination,value)
    return dict(id=value['id'],selected=selected,candidates=len(scores))


def diagnostics(data):
    rows_out=[]
    for threshold in ('1/3','2/3'):
        for match in (False,True):
            subset=[r for r in data if (r['threat']>=F(threshold))==match]
            regrets=[max(r['values'].values())-r['values'][r['baseline']] for r in subset]
            rows_out.append(dict(threshold=threshold,matched=match,assessments=len(subset),
                roots=len({r['root'] for r in subset}),misses=sum(r>0 for r in regrets),
                mean_regret=str(sum(regrets,F(0))/len(regrets)) if regrets else None))
    return rows_out


def stratification(data):
    total=within=F(0);eligible=0
    for row in data:
        alternative=next(a for a in row['values'] if a!=row['baseline'])
        diffs=[b-a for a,b in zip(row['vectors'][row['baseline']],row['vectors'][alternative])]
        n=len(diffs);mean=F(sum(diffs),n)
        total+=sum((F(d)-mean)**2 for d in diffs)/n
        if len(set(row['world_threat']))>1:eligible+=1
        for match in (False,True):
            group=[d for d,t in zip(diffs,row['world_threat']) if t==match]
            if group:
                m=F(sum(group),len(group));within+=sum((F(d)-m)**2 for d in group)/n
    return dict(assessments=len(data),mixed_threat_assessments=eligible,
        action_difference_variance=str(total/len(data)),within_threat_strata_variance=str(within/len(data)),
        explained_fraction=str(1-within/total) if total else None,
        interpretation='Retrospective variance decomposition, not held-out sample savings or an allocation policy.')


def report(output,fit_path,destination):
    fit_value=read(fit_path);assert fit_value['id']==hashed({k:v for k,v in fit_value.items() if k!='id'})
    panel=read(output/'panel.json');assert panel['split']=='fresh'
    assert read(output/'exam-fit.json')['id']==fit_value['id']
    assert read(output/'implementation.json')==fit_value['implementation']
    data=rows(output);score=describe(fit_value['selected']['rule'],data)
    rng=random.Random(420644)
    # Equal coordinates: resample whole source-deal groups, keeping their sizes.
    grouped=defaultdict(list)
    for r in data:grouped[r['deal_seed']].append(r['values'][choose(score['rule'],r)]-r['values'][r['baseline']])
    blocks=list(grouped.values());draws=[]
    for _ in range(1999):
        sampled=[b for group in rng.choices(blocks,k=len(blocks)) for b in group]
        draws.append(sum(sampled,F(0))/len(sampled))
    draws.sort();score['descriptive_cluster_95']=[str(draws[49]),str(draws[1949])]
    audit=Counter()
    for r in data:audit.update(r['audit'])
    value=dict(schema='kiln-decision-transfer-v1',fit_id=fit_value['id'],panel_id=panel['id'],
        score=score,detectors=diagnostics(data),variance_diagnostic=stratification(data),
        audit=dict(audit),elapsed_assessment_us=sum(r['elapsed_us'] for r in data),
        receipt_hashes=[r['sha256'] for r in data])
    value['id']=hashed(value);immutable_json(destination,value)
    return value


def audit(output):
    panel=read(output/'panel.json')
    assert panel['id']==hashed({k:v for k,v in panel.items() if k!='id'})
    implementation=read(output/'implementation.json')
    assert p.digest(output/'producer/worker')==implementation['binary_sha256']
    with tarfile.open(output/'producer/sources.tar.gz','r:gz') as archive:
        for name,sha in implementation['sources'].items():
            assert hashlib.sha256(archive.extractfile(name).read()).hexdigest()==sha
    data=rows(output)
    db=sqlite3.connect(output/'assessments.sqlite');db.row_factory=sqlite3.Row
    totals=Counter()
    for row in db.execute('SELECT * FROM jobs ORDER BY id'):
        value=json.loads(zlib.decompress(row['payload']))
        checks=validate(json.loads(row['input']),value)
        assert checks==value['independent_audit'];totals.update(checks)
    db.close()
    return dict(assessments=len(data),verified=True,totals=dict(totals))


def main():
    parser=argparse.ArgumentParser(description=__doc__);sub=parser.add_subparsers(dest='command',required=True)
    q=sub.add_parser('prepare');q.add_argument('source',type=Path);q.add_argument('output',type=Path);q.add_argument('--split',choices=['train','fresh'],required=True)
    q=sub.add_parser('run');q.add_argument('output',type=Path);q.add_argument('--workers',type=int,default=10);q.add_argument('--seconds',type=float,default=60)
    q=sub.add_parser('fit');q.add_argument('output',type=Path);q.add_argument('destination',type=Path)
    q=sub.add_parser('report');q.add_argument('output',type=Path);q.add_argument('fit',type=Path);q.add_argument('destination',type=Path)
    q=sub.add_parser('audit');q.add_argument('output',type=Path)
    args=parser.parse_args()
    if args.command=='prepare':value=prepare(args.source,args.output,args.split)
    elif args.command=='run':
        if not 1<=args.workers<=18 or not 1<=args.seconds<=240:parser.error('workers 1..18, seconds 1..240')
        value=asyncio.run(run(args.output,args.workers,args.seconds))
    elif args.command=='fit':value=fit(args.output,args.destination)
    elif args.command=='audit':value=audit(args.output)
    else:value=report(args.output,args.fit,args.destination)
    print(p.canonical(value),flush=True)


if __name__=='__main__':main()
