"""Paired dropped-30 matches against the phone's deployed native Walt source.

Physical deals remain in this host. Each policy receives only its own hand,
public history, and a field seed independent of the physical deal seed.
An external process-group watchdog enforces a 30-second whole-game ceiling.
"""
import argparse
from concurrent.futures import ThreadPoolExecutor, as_completed
from collections import Counter
import hashlib
import json
import math
import os
from pathlib import Path
import select
import signal
import statistics
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parent
BASELINE = ROOT / 'h2h/target/release/walt-table'
TILES = [(h,l) for h in range(7) for l in range(h+1)]


class Lines:
    def __init__(self, pipe):
        self.pipe, self.buffer = pipe, b''

    def read(self, deadline):
        while b'\n' not in self.buffer:
            left = deadline-time.monotonic()
            if left <= 0 or not select.select([self.pipe], [], [], max(0,left))[0]:
                raise TimeoutError('whole-game wall deadline')
            chunk = os.read(self.pipe.fileno(), 65536)
            if not chunk:
                raise RuntimeError('worker closed output before a result')
            self.buffer += chunk
        line, self.buffer = self.buffer.split(b'\n',1)
        return json.loads(line)


def emit(row):
    print(json.dumps(row,separators=(',',':')),flush=True)


def fixture(seed):
    import numpy as np
    order = np.random.default_rng(seed).permutation(28).tolist()
    hands = [sorted(order[7*s:7*s+7]) for s in range(4)]
    # Dropped on seat 1: never search the other three hands for a better bidder.
    own = hands[1]
    trump = max(range(7),key=lambda p:(sum(p in TILES[t] for t in own),
                                      TILES.index((p,p)) in own,p))
    return {'seed':seed,'bidder':1,'bid':30,'trump':trump,'hands':hands}


def legal(hand,trick,trump):
    if not trick:return sorted(hand)
    lead=TILES[trick[0]]
    suit=trump if trump in lead else lead[0]
    follow=[t for t in hand if suit in TILES[t] and (suit==trump or trump not in TILES[t])]
    return sorted(follow or hand)


def advance(state,tile,trump):
    leader,trick,t0,t1=state
    trick=trick+(tile,)
    if len(trick)<4:return leader,trick,t0,t1
    lead=TILES[trick[0]];suit=trump if trump in lead else lead[0]
    def rank(t):
        h,l=TILES[t]
        return (2 if trump in (h,l) else 1 if suit in (h,l) else 0,h==l,h+l)
    winner=(leader+max(range(4),key=lambda i:rank(trick[i])))%4
    won=1+sum(sum(TILES[t]) for t in trick if sum(TILES[t]) in (5,10))
    return winner,(),t0+(won if winner%2==0 else 0),t1+(won if winner%2 else 0)


def child(job):
    import numpy as np
    import repaired as r
    import turbo
    f=fixture(job['seed']);hands=f['hands'];masks=[sum(1<<t for t in h) for h in hands]
    rules=r.Rules(f['trump']);pub=r.Pub.initial();state=(1,(),0,0)
    history=[];moves=[];times={'candidate':0.,'walt':0.};walt=None;field=None
    started=time.monotonic();deadline=started+job['timeout']
    emit({'event':'started','fixture':f,'job':job})
    status='complete';reason=None;made=None
    try:
        field=turbo.Field(rules,r.Spec(tuple(job['samples']),1.,42),
                          seconds=max(0,deadline-time.monotonic()),addressed=True)
        walt=subprocess.Popen([str(BASELINE)],stdin=subprocess.PIPE,stdout=subprocess.PIPE,
                              stderr=sys.stderr)
        lines=Lines(walt.stdout)
        while state[2]<=12 and state[3]<30:
            if time.monotonic()>=deadline:raise TimeoutError('whole-game wall deadline')
            seat=(state[0]+len(state[1]))%4
            hand=masks[seat]&~int(pub.played[0])
            allowed=legal([t for t in range(28) if hand>>t&1],state[1],f['trump'])
            is_candidate=(seat%2==1)==(job['arm']=='declarer')
            engine='candidate' if is_candidate else 'walt'
            begin=time.monotonic();response=None;call=None
            if is_candidate:
                tile=int(field.actions(2,pub,np.array([hand],np.int64))[0])
            else:
                call={'request':{'decl':f['trump'],'bid':30,'bidder':1,'seat':seat,
                     'hand':hands[seat],'plays':[v for pair in history for v in pair],'seed':42},
                     'worlds':160 if not history else 40,'partner':bool(history),
                     'budget_ms':20000 if not history else 14000}
                walt.stdin.write((json.dumps(call)+'\n').encode());walt.stdin.flush()
                while True:
                    message=lines.read(deadline)
                    if 'result' in message:
                        response=message['result'];break
                if 'error' in response:raise RuntimeError(response['error'])
                assert response['leader']==state[0]
                assert response['points']==[state[2],state[3]]
                assert sorted(response['legal'])==allowed
                tile=response['choice']
            elapsed=time.monotonic()-begin;times[engine]+=elapsed
            if time.monotonic()>deadline:raise TimeoutError('whole-game wall deadline')
            assert tile in allowed,(seat,tile,allowed)
            state=advance(state,tile,f['trump'])
            pub=pub.play(rules,np.array([tile],np.int64))
            assert (int(pub.leader[0]),tuple(map(int,pub.trick[0,:pub.tlen[0]])),int(pub.t0[0]),int(pub.t1[0]))==state
            history.append((seat,tile))
            row={'seat':seat,'tile':tile,'engine':engine,'elapsed_seconds':elapsed,
                 'points':[state[2],state[3]],'call':call,'response':response}
            moves.append(row);emit({'event':'move',**row})
        made=state[3]>=30
    except (TimeoutError,r.Limit) as e:
        status='timeout' if isinstance(e,TimeoutError) or 'deadline' in str(e) else 'resource_limit'
        reason=str(e)
    except Exception as e:
        status='error';reason=repr(e)
    elapsed=time.monotonic()-started
    if elapsed>job['timeout']:
        status='timeout';reason='whole-game wall deadline';made=None
    result={'event':'result','status':status,'reason':reason,'elapsed_seconds':elapsed,
            'made':made if status=='complete' else None,'points':[state[2],state[3]],
            'engine_seconds':times,'moves':len(moves),'stats':field.stats if field else None,
            'metrics':field.metrics if field else None}
    emit(result)
    if walt:
        walt.terminate();walt.wait()
    if field:field.close()


def run_game(job,out):
    stem=f"{job['seed']}-{job['arm']}"
    receipt=out/(stem+'.jsonl');stderr=out/(stem+'.stderr')
    if receipt.exists():
        rows=[json.loads(line) for line in receipt.read_text().splitlines()]
        assert rows[0]['job']==job,'existing receipt has different configuration'
        if rows[-1]['event']=='result':return audit(receipt)
        raise RuntimeError('unfinished receipt requires explicit investigation: '+str(receipt))
    with stderr.open('wb') as err,receipt.open('w') as log:
        process=subprocess.Popen([sys.executable,str(Path(__file__).resolve()),'worker'],
            stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=err,start_new_session=True)
        process.stdin.write((json.dumps(job)+'\n').encode());process.stdin.close()
        lines=Lines(process.stdout);started=None
        try:
            first=lines.read(time.monotonic()+30)
            assert first['event']=='started'
            started=time.monotonic();log.write(json.dumps(first)+'\n');log.flush()
            while True:
                row=lines.read(started+job['timeout'])
                log.write(json.dumps(row)+'\n');log.flush()
                if row['event']=='result':break
        except TimeoutError:
            if started is None:raise RuntimeError('worker startup timeout')
            row={'event':'result','status':'timeout','reason':'external 30-second process-group watchdog',
                 'elapsed_seconds':job['timeout'],'made':None}
            log.write(json.dumps(row)+'\n');log.flush()
        finally:
            try:os.killpg(process.pid,signal.SIGKILL)
            except ProcessLookupError:pass
            process.wait()
    return audit(receipt)


def audit(path):
    rows=[json.loads(line) for line in Path(path).read_text().splitlines()]
    initial=rows[0];job=initial['job'];f=initial['fixture'];result=rows[-1]
    assert f==fixture(job['seed'])
    assert sorted(t for h in f['hands'] for t in h)==list(range(28))
    hands=[set(h) for h in f['hands']];state=(1,(),0,0);times=Counter();routes=Counter();partial=0
    history=[]
    for row in rows[1:-1]:
        assert row['event']=='move' and state[2]<=12 and state[3]<30
        seat=row['seat'];tile=row['tile'];assert seat==(state[0]+len(state[1]))%4
        assert tile in legal(hands[seat],state[1],f['trump'])
        candidate=(seat%2==1)==(job['arm']=='declarer')
        assert row['engine']==('candidate' if candidate else 'walt')
        times[row['engine']]+=row['elapsed_seconds']
        if not candidate:
            response=row['response'];call=row['call'];req=call['request']
            assert set(req)=={'decl','bid','bidder','seat','hand','plays','seed'}
            assert req=={'decl':f['trump'],'bid':30,'bidder':1,'seat':seat,'hand':f['hands'][seat],
                         'plays':[v for pair in history for v in pair],'seed':42}
            assert response['choice']==tile and response['points']==[state[2],state[3]]
            assert response['leader']==state[0] and sorted(response['legal'])==legal(hands[seat],state[1],f['trump'])
            routes[response['route']]+=1
            partial+=int('interruption' in response)
        hands[seat].remove(tile);history.append((seat,tile));state=advance(state,tile,f['trump'])
        assert row['points']==[state[2],state[3]]
    assert result['event']=='result'
    if result['status']=='complete':
        assert state[2]>12 or state[3]>=30
        assert result['made']==(state[3]>=30)
        assert result['elapsed_seconds']<=job['timeout']
        assert result['points']==[state[2],state[3]]
    else:assert result['made'] is None
    return {'seed':job['seed'],'arm':job['arm'],'status':result['status'],'made':result['made'],
            'seconds':result['elapsed_seconds'],'engine_seconds':dict(times),'moves':len(history),
            'routes':dict(routes),'walt_interruptions':partial,'receipt':str(path)}


def summarize(rows):
    by_seed={}
    for row in rows:by_seed.setdefault(row['seed'],{})[row['arm']]=row
    pairs=[]
    for seed,arms in sorted(by_seed.items()):
        if len(arms)<2 or any(v['status']!='complete' for v in arms.values()):
            pairs.append({'seed':seed,'delta':None});continue
        pairs.append({'seed':seed,'delta':int(arms['declarer']['made'])-int(arms['defender']['made']),
                      'candidate_make':arms['declarer']['made'],'walt_make':arms['defender']['made']})
    complete=[r for r in rows if r['status']=='complete'];scored=[p for p in pairs if p['delta'] is not None]
    w=sum(p['delta']==1 for p in scored);l=sum(p['delta']==-1 for p in scored);t=len(scored)-w-l
    n=w+l;p=min(1.,2*sum(math.comb(n,k) for k in range(min(w,l)+1))/2**n) if n else 1.
    seconds=sorted(r['seconds'] for r in complete)
    return {'games':len(rows),'completed_games':len(complete),'timeouts':sum(r['status']=='timeout' for r in rows),
            'errors':sum(r['status'] not in ('complete','timeout') for r in rows),
            'completed_pairs':len(scored),'unscored_pairs':len(pairs)-len(scored),
            'pair_wins':w,'pair_losses':l,'pair_ties':t,'paired_net':w-l,
            'candidate_game_wins':2*w+t,'walt_game_wins':2*l+t,
            'exploratory_mcnemar_exact_two_sided_p':p,
            'median_seconds':statistics.median(seconds) if seconds else None,
            'p95_seconds':seconds[math.ceil(.95*len(seconds))-1] if seconds else None,
            'max_completed_seconds':max(seconds) if seconds else None,
            'median_candidate_seconds':statistics.median(r['engine_seconds'].get('candidate',0) for r in complete) if complete else None,
            'median_walt_seconds':statistics.median(r['engine_seconds'].get('walt',0) for r in complete) if complete else None,
            'walt_routes':dict(sum((Counter(r['routes']) for r in rows),Counter())),
            'walt_interruptions':sum(r['walt_interruptions'] for r in rows),'pairs':pairs}


def panel(args):
    out=Path(args.output).resolve();out.mkdir(parents=True,exist_ok=True)
    samples=list(map(int,args.samples.split(',')));assert len(samples)==2
    assert 0<args.timeout<=30
    identity={'candidate_sha256':hashlib.sha256((ROOT/'turbo.dylib').read_bytes()).hexdigest(),
              'baseline_sha256':hashlib.sha256(BASELINE.read_bytes()).hexdigest()}
    jobs=[]
    for index,seed in enumerate(range(args.seed,args.seed+args.pairs)):
        for arm in (['declarer','defender'] if index%2==0 else ['defender','declarer']):
            jobs.append({'seed':seed,'arm':arm,'samples':samples,'timeout':args.timeout,**identity})
    started=time.monotonic();rows=[]
    with ThreadPoolExecutor(args.workers) as pool:
        futures=[pool.submit(run_game,job,out) for job in jobs]
        for future in as_completed(futures):
            row=future.result();rows.append(row)
            result=summarize(rows)
            (out/'summary.json').write_text(json.dumps({**result,'wall_seconds':time.monotonic()-started,
                'samples_bottom_first':samples,'workers':args.workers,'requested_pairs':args.pairs,
                'finished':len(rows)==len(jobs),'rows':sorted(rows,key=lambda r:(r['seed'],r['arm']))},indent=2)+'\n')
            emit({k:v for k,v in result.items() if k not in ('pairs','walt_routes')})
    if result['errors']:raise RuntimeError('panel contains errors; inspect receipts')


if __name__=='__main__':
    parser=argparse.ArgumentParser();sub=parser.add_subparsers(dest='command',required=True)
    sub.add_parser('worker')
    run=sub.add_parser('run');run.add_argument('--samples',default='8,30');run.add_argument('--pairs',type=int,default=64)
    run.add_argument('--seed',type=int,default=9303001);run.add_argument('--workers',type=int,default=4)
    run.add_argument('--timeout',type=float,default=30);run.add_argument('--output',required=True)
    check=sub.add_parser('audit');check.add_argument('paths',nargs='+')
    args=parser.parse_args()
    if args.command=='worker':child(json.loads(sys.stdin.readline()))
    elif args.command=='audit':emit(summarize([audit(p) for p in args.paths]))
    else:panel(args)
