#!/usr/bin/env python3
"""Pinned mirrored full games against the actual current partnership wrapper.

The referee owns complete deals. Each player receives only its own original
hand and public history. Complete receipts and all fallbacks remain scored.
"""
import argparse
import hashlib
import json
import math
import os
from pathlib import Path
import platform
import random
import select
import statistics
import subprocess
import sys
import time

HERE=Path(__file__).resolve().parents[1]

def digest(path): return hashlib.sha256(Path(path).read_bytes()).hexdigest()
def atomic(path,value):
    tmp=path.with_suffix(path.suffix+'.tmp')
    tmp.write_text(json.dumps(value,sort_keys=True,separators=(',',':'))+'\n');tmp.replace(path)
def seed(domain,index):
    # Independent fixed namespaces; a player's seed never receives the deal seed.
    return int.from_bytes(hashlib.sha256(('walt-epochs-h2h-v1/'+domain+'/'+str(index)).encode()).digest()[:8],'little')
def quantile(xs,p): return sorted(xs)[min(len(xs)-1,max(0,math.ceil(p*len(xs))-1))] if xs else None

def summary_stats(xs):
    return {'count':len(xs),'mean_ms':statistics.mean(xs) if xs else None,
            **{name:quantile(xs,q) for name,q in [('median_ms',.5),('p90_ms',.9),('p95_ms',.95),('p99_ms',.99),('max_ms',1)]}}

class GpuWorker:
    def __init__(self,binary,errors):
        started=time.monotonic()
        self.errors=open(errors,'ab')
        self.p=subprocess.Popen([str(binary),'worker-gpu'],stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=self.errors,bufsize=0)
        self.ready=self.read(30)
        if not self.ready.get('ready'): raise RuntimeError('GPU did not initialize')
        self.startup_ms=(time.monotonic()-started)*1000
    def read(self,timeout):
        end=time.monotonic()+timeout;data=bytearray()
        while True:
            left=end-time.monotonic()
            if left<=0 or not select.select([self.p.stdout],[],[],left)[0]: raise TimeoutError('GPU response watchdog')
            chunk=os.read(self.p.stdout.fileno(),65536)
            if not chunk: raise RuntimeError('GPU worker exited '+str(self.p.poll()))
            data.extend(chunk)
            if len(data)>32_000_000: raise RuntimeError('GPU response too large')
            if b'\n' in data:
                if data.count(b'\n')!=1 or not data.endswith(b'\n'): raise RuntimeError('GPU framing violation')
                return json.loads(data)
    def call(self,request):
        self.p.stdin.write((json.dumps(request,separators=(',',':'))+'\n').encode());self.p.stdin.flush()
        return self.read(request['budget_ms']/1000+2)
    def close(self):
        if self.p.poll() is None:self.p.terminate()
        try:self.p.wait(timeout=2)
        except subprocess.TimeoutExpired:self.p.kill();self.p.wait()
        self.p.stdin.close();self.p.stdout.close();self.errors.close()

def verify_report(v):
    r=v.get('report')
    if r is None:return
    a=next(x for x in r['actions'] if x['action']==r['chosen'])
    assert r['chosen']==v['tile'] and all(0<=x['lower']<=x['upper']<=r['mass'] for x in r['actions'])
    assert r['incumbent_value']==(a['lower'] if a['priced'] else None)
    assert r['regret_bound']==max(x['upper'] for x in r['actions'])-a['lower']
    assert r['canonical_certified']==all(x['action']==r['chosen'] or
        (x['upper']<a['lower'] if x['action']<r['chosen'] else x['upper']<=a['lower']) for x in r['actions'])
    assert v['compute']['backend']=='gpu-epochs'

def play(f,arm,protocol,gpu,cpu,session,rules):
    hands=f['hands'];remaining=[set(h) for h in hands];record=[];moves=[]
    leader=f['bidder'];trick=[];points=[0,0]
    candidate_parity=f['bidder']%2 if arm=='declaring' else 1-f['bidder']%2
    start=time.monotonic()
    for turn in range(28):
        actor=(leader+len(trick))%4;candidate=actor%2==candidate_parity
        legal=rules.legal_tiles(remaining[actor],trick,f['decl'])
        public={'decl':f['decl'],'bid':30,'bidder':f['bidder'],'seat':actor,
                'hand':hands[actor],'plays':record[:],'seed':f['policy_seed']}
        t=time.monotonic()
        if candidate:
            req={'decl':f['decl'],'bid':30,'bidder':f['bidder'],'seat':actor,
                'hand':sorted(remaining[actor]),'original_hand':hands[actor],
                'history':[record[i:i+2] for i in range(0,len(record),2)],
                'seed':f['policy_seed'],'budget_ms':protocol['gpu_budget_ms'],
                'config':protocol['candidate_config']}
            response=gpu.call(req)
            if 'error' in response:raise RuntimeError('GPU decision failed: '+response['error'])
            choice=response['tile'];verify_report(response)
        else:
            req=public
            response=cpu.decide(req,mode='partner',n=40,n0=8,n1=2,budget_ms=protocol['cpu_budget_ms'],
                inner_belief='voidless',selection='fixed',modeled_selection='fixed',session=session,review='off')
            choice=response['choice']
        elapsed_ms=(time.monotonic()-t)*1000
        assert choice in legal and actor==(leader+len(trick))%4
        moves.append({'actor':actor,'tile':choice,'player':'gpu' if candidate else 'cpu',
            'trick':turn//4+1,'legal':legal,'elapsed_ms':elapsed_ms,'request':req,'response':response})
        remaining[actor].remove(choice);record.extend([actor,choice]);trick.append((actor,choice))
        if len(trick)==4:
            leader=rules.winner(trick,f['decl']);points[leader%2]+=rules.trick_points(trick);trick=[]
    elapsed_ms=(time.monotonic()-start)*1000
    replay=rules.replay_record(hands,record,f['decl'],f['bidder'])
    assert replay[0]==points and sum(points)==42 and all(not h for h in replay[2])
    return {'schema':'gpu-current-h2h-game-v1','fixture':f,'arm':arm,'record':record,
        'moves':moves,'points':points,'declaring_made':points[f['bidder']%2]>=30,
        'elapsed_ms':elapsed_ms,'independent_replay_passed':True}

def analyze(output,protocol,rules):
    games=[];pairs=[]
    for f in protocol['fixtures']:
        pair={}
        for arm in ['declaring','defending']:
            path=output/(str(f['index'])+'-'+arm+'.json')
            if not path.exists():continue
            game=json.loads(path.read_text())
            assert game['fixture']==f
            points,_,remaining,trick=rules.replay_record(f['hands'],game['record'],f['decl'],f['bidder'])
            assert len(game['record'])==56 and not trick and all(not h for h in remaining)
            assert game['points']==points and game['declaring_made']==(points[f['bidder']%2]>=30)
            for m in game['moves']:
                if m['player']=='gpu':verify_report(m['response'])
            games.append(game);pair[arm]=game
        if len(pair)==2:
            a=pair['declaring']['declaring_made'];b=pair['defending']['declaring_made']
            pairs.append({'index':f['index'],'decl':f['decl'],'bidder':f['bidder'],
                          'gpu_declaring_make':a,'cpu_declaring_make':b,'delta':int(a)-int(b)})
    moves=[m for g in games for m in g['moves']];by={p:[m for m in moves if m['player']==p] for p in ['gpu','cpu']}
    stats={}
    for name,ms in by.items():
        partnership=[sum(m['elapsed_ms'] for m in g['moves'] if m['player']==name) for g in games]
        fallback=sum(m['response'].get('fallback',False) if name=='gpu' else m['response']['route'].endswith('fallback') for m in ms)
        stats[name]={'moves':len(ms),'nonforced':sum(len(m['legal'])>1 for m in ms),'fallbacks':fallback,
            'decision_time':summary_stats([m['elapsed_ms'] for m in ms]),'partnership_time':summary_stats(partnership),
            'by_trick_ms':{str(t):sum(m['elapsed_ms'] for m in ms if m['trick']==t) for t in range(1,8)},
            'over_budget':sum(m['elapsed_ms']>protocol[name+'_budget_ms'] for m in ms)}
    gpu_stats={key:sum(m['response'].get('compute',{}).get('stats',{}).get(key,0) for m in by['gpu']) for key in
        ['batches','completed_batches','cancelled_batches','epochs','lanes_submitted','completed_lanes','field_requests','unique_field_queries','modeled_unique_queries','device_ms','field_ms','elapsed_ms']}
    ds=[p['delta'] for p in pairs];n=len(ds);mean=statistics.mean(ds) if ds else None
    fraction=(1+mean)/2 if n else None;epsilon=math.sqrt(math.log(40)/(2*n)) if n else None
    out={'schema':'gpu-current-h2h-summary-v1','complete':len(pairs)==len(protocol['fixtures']),
        'pairs':n,'games':len(games),'plays':len(moves),'all_replay_checks_pass':True,
        'wins':sum(d>0 for d in ds),'losses':sum(d<0 for d in ds),'ties':sum(d==0 for d in ds),
        'mean_paired_delta':mean,'comparative_contract_fraction':fraction,
        'fixed_n_hoeffding_95_interval':([max(0,fraction-epsilon),min(1,fraction+epsilon)] if n else None),
        'stats':stats,'gpu_compute':gpu_stats,'paired_results':pairs,
        'gpu_canonical_certified':sum(bool(m['response'].get('report') and m['response']['report']['canonical_certified']) for m in by['gpu']),
        'gpu_priced_decisions':sum(bool(m['response'].get('report') and m['response']['report']['incumbent_value'] is not None) for m in by['gpu'])}
    # This is measured latency, not an assumed consequence of equal allowances.
    if games:
        ratios={k:stats['gpu']['partnership_time'][k]/stats['cpu']['partnership_time'][k] for k in ['mean_ms','p95_ms']}
        out['partnership_latency_ratios']=ratios;out['proposed_10pct_latency_gate']=all(v<=1.1 for v in ratios.values())
    atomic(output/'summary.json',out)
    return out

def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('output',type=Path);p.add_argument('--cpu-root',type=Path,default=Path('/Users/jason/code/texas-42-partnership-launch'))
    p.add_argument('--binary',type=Path,default=HERE/'target/release/response-player')
    p.add_argument('--gpu-ms',type=int,default=100);p.add_argument('--cpu-ms',type=int,default=14000)
    p.add_argument('--deals-per-cell',type=int,default=2);p.add_argument('--panel',default='fresh-72-v1')
    p.add_argument('--threads',type=int,default=18)
    p.add_argument('--analyze-only',action='store_true')
    args=p.parse_args();sys.path.insert(0,str(args.cpu_root/'experiments/partnership'))
    import player as cpu
    import rules
    from runtime import DecisionSession
    assert cpu.BINARY.resolve()==(args.cpu_root/'walt/target/release/partnership').resolve()
    if args.analyze_only:
        result=analyze(args.output,json.loads((args.output/'protocol.json').read_text()),rules)
        print(json.dumps({k:v for k,v in result.items() if k not in ['paired_results','stats']},indent=2));return
    if args.threads < 1: raise ValueError('threads must be positive')
    os.environ['WALT_RAYON_THREADS']=str(args.threads)
    os.environ['RAYON_NUM_THREADS']=str(args.threads)
    args.output.mkdir(parents=True,exist_ok=False)
    fixtures=[]
    for repeat in range(args.deals_per_cell):
        for decl in [0,1,2,3,4,5,6,7,9]:
            for bidder in range(4):
                i=len(fixtures);deck=list(range(28));random.Random(seed('deals/'+args.panel,i)).shuffle(deck)
                fixtures.append({'index':i,'repeat':repeat,'decl':decl,'bidder':bidder,'policy_seed':seed('policy/'+args.panel,i),
                    'hands':[sorted(deck[s*7:s*7+7]) for s in range(4)]})
    protocol={'schema':'gpu-current-h2h-protocol-v1','fixtures':fixtures,'panel':args.panel,
        'gpu_budget_ms':args.gpu_ms,'cpu_budget_ms':args.cpu_ms,
        'candidate_config':{'field':'partner','outer':40,'n0':8,'n1':2,'plans':8,'horizon':7,'work':2000000},
        'cpu_profile':'current deployed L2 Partner 40/8/2 Fixed/Fixed Voidless, review off; actual reserve wrapper',
        'cpu_root':str(args.cpu_root),'cpu_head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=args.cpu_root,text=True).strip(),
        'cpu_binary':str(cpu.BINARY),'cpu_binary_sha256':digest(cpu.BINARY),
        'cpu_source_sha256':{name:digest(args.cpu_root/'experiments/partnership'/name) for name in ['player.py','runtime.py','rules.py','players.json']},
        'gpu_binary':str(args.binary.resolve()),'gpu_binary_sha256':digest(args.binary),
        'gpu_sources':{str(f.relative_to(HERE)):digest(f) for f in (HERE/'src').rglob('*') if f.is_file()},
        'harness_sha256':digest(__file__),'reference_archive_sha256':digest(HERE/'reference/native-v34-sources.tar.gz'),
        'rustc':subprocess.check_output(['rustc','-Vv'],text=True),'build_flags':'--release --features gpu; RUSTFLAGS=-C target-cpu=native',
        'cpu_threads':int(os.environ.get('WALT_RAYON_THREADS','6')),'gpu_cpu_threads':int(os.environ.get('RAYON_NUM_THREADS',os.cpu_count())), 'python':sys.version,'platform':platform.platform(),
        'analysis':'72-deal balanced DEVELOPMENT comparison, no promotion. Equal stratum weights; paired make/set; fixed-N Hoeffding interval. No outcome stopping or retries. Entire games retained.',
        'timing':'Persistent GPU/CPU processes; both receive one declared off-panel opening warmup to initialize device work and thread pools, recorded separately. Measured per-decision wall includes IPC/serialization and each player own validation/fallback. This is a per-move cap, NOT a whole-game cap. CPU default 14000ms anchor is deliberately not weakened to manufacture a quality win.',
        'warmup':{'decl':6,'bid':30,'bidder':1,'seat':1,'hand':[2,6,13,16,19,21,26],'plays':[],'seed':50129},
        'order':'AB/BA alternates by repeat within each declaration/bidder cell; GPU declaring first on even repeats.'}
    atomic(args.output/'protocol.json',protocol)
    gpu=GpuWorker(args.binary,args.output/'gpu.stderr')
    try:
        assert gpu.ready['cpu_query_threads']==args.threads
        with DecisionSession() as session:
            f=fixtures[0];req={'decl':f['decl'],'bid':30,'bidder':f['bidder'],'seat':f['bidder'],'hand':f['hands'][f['bidder']],'plays':[],'seed':f['policy_seed']}
            t=time.monotonic();_,status=session.call([str(cpu.BINARY)],cpu.native_text(req,'status'),5)
            if status!='completed':raise RuntimeError('CPU startup failed '+status)
            startup={'gpu':gpu.ready,'gpu_process_ms':gpu.startup_ms,'cpu_status_ms':(time.monotonic()-t)*1000}
            warm=protocol['warmup']
            t=time.monotonic()
            startup['cpu_warmup']=cpu.decide(warm,mode='partner',n=40,n0=8,n1=2,budget_ms=args.cpu_ms,
                inner_belief='voidless',selection='fixed',modeled_selection='fixed',session=session,review='off')
            startup['cpu_warmup_ms']=(time.monotonic()-t)*1000
            req={k:v for k,v in warm.items() if k!='plays'}
            req.update(original_hand=warm['hand'],history=[],budget_ms=args.gpu_ms,config=protocol['candidate_config'])
            t=time.monotonic();startup['gpu_warmup']=gpu.call(req);startup['gpu_warmup_ms']=(time.monotonic()-t)*1000
            if 'error' in startup['gpu_warmup']:raise RuntimeError('GPU warmup failed')
            atomic(args.output/'startup.json',startup)
            for f in fixtures:
                arms=['declaring','defending'] if f['repeat']%2==0 else ['defending','declaring']
                for arm in arms:
                    game=play(f,arm,protocol,gpu,cpu,session,rules);atomic(args.output/(str(f['index'])+'-'+arm+'.json'),game)
                atomic(args.output/'progress.json',{'completed_pairs':f['index']+1,'planned':len(fixtures)})
                print(str(f['index']+1)+'/'+str(len(fixtures))+' paired deals independently replayed',flush=True)
    finally:gpu.close()
    if digest(cpu.BINARY)!=protocol['cpu_binary_sha256'] or digest(args.binary)!=protocol['gpu_binary_sha256']:
        raise RuntimeError('player binary changed during comparison')
    result=analyze(args.output,protocol,rules)
    assert result['complete'] and result['gpu_compute']['completed_batches']>0
    print(json.dumps({k:v for k,v in result.items() if k not in ['paired_results','stats']},indent=2))
if __name__=='__main__':main()
