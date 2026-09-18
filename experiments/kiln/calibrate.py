#!/usr/bin/env python3
"""Execute the table player on fresh hidden completions of one priced own hand.
Per-move durable checkpoints; original prediction is frozen before any outcomes.
"""
import argparse,asyncio,fcntl,hashlib,json,math,os,random,signal,sys,time
from pathlib import Path
from kiln import ROOT,atomic_json,canonical,digest
sys.path.insert(0,str(ROOT/'experiments/partnership'))
from rules import replay_record,legal_tiles

def seeded(label):return int.from_bytes(hashlib.sha256(label.encode()).digest()[:8],'little')
def completion(hand,seat,index,case_id):
    remaining=sorted(set(range(28))-set(hand))
    random.Random(seeded(f'kiln-calibration-hidden-v1/{case_id}/{index}')).shuffle(remaining)
    hands=[None]*4;hands[seat]=hand
    for j,s in enumerate(s for s in range(4) if s!=seat):hands[s]=sorted(remaining[j*7:j*7+7])
    return hands

def summary(directory,case):
    games=[]
    for path in sorted((directory/'games').glob('*.json')):
        g=json.loads(path.read_text())
        if g['status']=='complete':games.append(g)
    wins=sum(g['made'] for g in games);n=len(games)
    low=high=None
    if n:
        p=wins/n;z=1.96;den=1+z*z/n
        center=(p+z*z/(2*n))/den
        radius=z*math.sqrt(p*(1-p)/n+z*z/(4*n*n))/den
        low,high=center-radius,center+radius
    return {'schema':'kiln-calibration-summary-v1','case_id':case['id'],'completed':n,'requested':case['games'],
      'made':wins,'observed':wins/n if n else None,'wilson95':[low,high],
      'prediction':case['prediction'],'profile':case['profile'],
      'note':'Fixed own hand; fresh uniform hidden completions. Played outcomes assess calibration against the declared executed policy, not an exact win probability.'}

async def run(args):
    directory=Path(args.directory);directory.mkdir(parents=True,exist_ok=True)
    (directory/'games').mkdir(exist_ok=True)
    lock=(directory/'coordinator.lock').open('w')
    fcntl.flock(lock,fcntl.LOCK_EX|fcntl.LOCK_NB)
    source=json.loads(Path(args.price).read_text())
    if source.get('schema')!='kiln-price-v1':raise ValueError('Supply a completed, frozen Kiln price receipt')
    case={'schema':'kiln-calibration-case-v1','auction':source['auction'],'decl':source['price'][0],
          'prediction':{'num':int(source['price'][1]),'den':int(source['price'][2]),'worlds':source['worlds']},
          'price_receipt':source,'games':args.games,'binary_sha256':digest(args.binary),
          'profile':'deployed-l1-partner-40-8-opening160-budget14s-opening20s-v1'}
    case['id']=hashlib.sha256(canonical(case).encode()).hexdigest()
    case_path=directory/'case.json'
    if case_path.exists():
        if json.loads(case_path.read_text())!=case:raise ValueError('Calibration identity changed; use another directory')
    else:atomic_json(case_path,case)
    stop=asyncio.Event();loop=asyncio.get_running_loop()
    for sig in (signal.SIGINT,signal.SIGTERM):loop.add_signal_handler(sig,stop.set)
    end=time.monotonic()+args.seconds if args.seconds else float('inf')
    pending=asyncio.Queue()
    for i in range(args.games):
        path=directory/'games'/f'{i:04}.json'
        if not path.exists() or json.loads(path.read_text())['status']!='complete':pending.put_nowait(i)
    async def worker():
        proc=None
        async def choose(request,opening):
            nonlocal proc
            if proc is None:
                proc=await asyncio.create_subprocess_exec(args.binary,stdin=asyncio.subprocess.PIPE,stdout=asyncio.subprocess.PIPE,
                    stderr=asyncio.subprocess.DEVNULL,env={**os.environ,'RAYON_NUM_THREADS':'1'})
            call={'request':request,'worlds':160 if opening else 40,'partner':not opening,'budget_ms':20000 if opening else 14000}
            proc.stdin.write((canonical(call)+'\n').encode());await proc.stdin.drain()
            deadline=time.monotonic()+call['budget_ms']/1000+3
            while True:
                line=await asyncio.wait_for(proc.stdout.readline(),max(.01,deadline-time.monotonic()))
                if not line:raise RuntimeError('Table worker exited')
                envelope=json.loads(line)
                if 'result' in envelope:
                    result=envelope['result']
                    if 'error' in result:raise ValueError(result['error'])
                    return result
        try:
            while not pending.empty() and not stop.is_set() and time.monotonic()<end:
                i=pending.get_nowait();path=directory/'games'/f'{i:04}.json'
                seat=case['auction']['seat'];hand=case['auction']['hand'];bid=case['auction']['bid'];decl=case['decl']
                hands=completion(hand,seat,i,case['id'])
                game=json.loads(path.read_text()) if path.exists() else {'schema':'kiln-calibration-game-v1','case_id':case['id'],
                     'index':i,'hands':hands,'record':[],'decisions':[],'status':'playing'}
                assert game['hands']==hands and game['case_id']==case['id']
                try:
                    while not stop.is_set() and time.monotonic()<end:
                        points,leader,remaining,trick=replay_record(hands,game['record'],decl,seat)
                        made=points[seat%2]>=bid;lost=points[1-seat%2]>42-bid
                        if made or lost:
                            game.update(status='complete',made=made,points=points)
                            atomic_json(path,game);break
                        actor=(leader+len(trick))%4
                        request={'decl':decl,'bid':bid,'bidder':seat,'seat':actor,'hand':hands[actor],
                                 'plays':list(game['record']),'seed':seeded(f'kiln-calibration-policy-v1/{case["id"]}/{i}') & 0xffffffff}
                        result=await choose(request,not game['record'])
                        legal=legal_tiles(remaining[actor],trick,decl)
                        if result['choice'] not in legal or result['points']!=points or result['leader']!=leader:
                            raise ValueError('Table decision disagrees with independent rules')
                        game['record'].extend([actor,result['choice']])
                        game['decisions'].append({'ply':len(game['record'])//2-1,'request':request,'response':result})
                        game.pop('error',None);atomic_json(path,game)
                except asyncio.CancelledError:raise
                except Exception as error:
                    game['error']=str(error);atomic_json(path,game)
                    if proc is not None and proc.returncode is None:proc.kill();await proc.wait()
                    proc=None
        finally:
            if proc is not None and proc.returncode is None:proc.kill();await proc.wait()
    tasks=[asyncio.create_task(worker()) for _ in range(args.workers)]
    try:
        last=0
        while not all(t.done() for t in tasks) and not stop.is_set() and time.monotonic()<end:
            if time.monotonic()-last>10:
                view=summary(directory,case);atomic_json(directory/'status.json',view);print(canonical(view),flush=True);last=time.monotonic()
            try:await asyncio.wait_for(stop.wait(),.25)
            except asyncio.TimeoutError:pass
    finally:
        for t in tasks:t.cancel()
        outcomes=await asyncio.gather(*tasks,return_exceptions=True)
        for error in outcomes:
            if isinstance(error,Exception):print(str(error),file=sys.stderr)
        view=summary(directory,case);atomic_json(directory/'status.json',view);print(canonical(view),flush=True)
        lock.close()

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('directory');p.add_argument('--price',required=True);p.add_argument('--binary',default=str(ROOT/'walt/target/release/walt-table'));p.add_argument('--games',type=int,default=100);p.add_argument('--workers',type=int,default=8);p.add_argument('--seconds',type=float,default=60)
    a=p.parse_args()
    if a.games<1 or a.workers<1 or a.seconds<0:p.error('Invalid bounds')
    asyncio.run(run(a))
