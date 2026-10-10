"""Frozen-input complete search timings: NumPy f64, torch CPU f32, MPS f32."""
import argparse
import inspect
import json
import os
import platform
import statistics
import time
import numpy as np
import torch
import walt42x as ref
from metal import Backend, Rules, Pub, search, FrontierLimit

def fixture(seed, played_count, n):
    rng = np.random.default_rng(seed)
    order = list(range(28))
    rng.shuffle(order)
    hands = [sum(1 << t for t in order[7*i:7*i+7]) for i in range(4)]
    tiles = lambda m: [t for t in range(28) if m >> t & 1]
    _, bidder, trump = max(
        ((sum(p in ref.TILES[t] for t in tiles(hands[s])), hands[s] >> ref.TILES.index((p,p)) & 1,
          sum(ref.TILES[t][0] == ref.TILES[t][1] for t in tiles(hands[s]))), s, p)
        for s in range(4) for p in range(7))
    hands = [hands[(bidder+i+3)%4] for i in range(4)]
    rules = ref.Rules(trump)
    pub = ref.Pub(np.array([0]), np.array([1]), np.zeros((1,4),dtype=np.int64),
                  np.array([0]),np.array([0]),np.array([0]))
    voids = [0]*4
    history = []
    for _ in range(played_count):
        seat = int(pub.turn()[0])
        hand = hands[seat] & ~int(pub.played[0])
        legal = np.flatnonzero(rules.legal(np.array([hand]),pub.trick,pub.tlen)[0])
        t = int(rng.choice(legal))
        if pub.tlen[0]:
            suit = int(rules.suit[rules.lead[pub.trick[0,0]]])
            if not (suit >> t & 1):
                voids[seat] |= suit
        history.append([seat,t])
        pub = pub.play(rules,np.array([t]))
    if any(bool(v[0]) for v in pub.outcome()):
        raise ValueError('fixture already terminal')
    seat = int(pub.turn()[0])
    deals = ref.sample(seat,hands[seat]&~int(pub.played[0]),int(pub.played[0]),
                       int(pub.leader[0]),int(pub.tlen[0]),n,rng,voids)
    return trump, seat, pub, deals, history

def bounded_numpy(cap, seconds):
    # Instrument the actual reference; keep algorithm and arithmetic unchanged.
    src = inspect.getsource(ref.search)
    src = src.replace('    while True:', '''    started = time.perf_counter()
    census = []
    while True:
        census.append({'ply': len(plies), 'nodes': len(node.played), 'fibers': len(tri_w)})
        if len(tri_w) > CAP or time.perf_counter() - started > SECONDS:
            raise FrontierLimit(census)''')
    src = src.replace('        cw =', '''        if len(ti) > CAP:
            raise FrontierLimit(census + [{'next_fibers': len(ti)}])
        cw =''')
    src = src.replace('return dict(zip(plies[1][1].tolist(), V.tolist()))',
                      'return dict(zip(plies[1][1].tolist(), V.tolist())), census')
    scope = dict(vars(ref), time=time, CAP=cap, SECONDS=seconds, FrontierLimit=FrontierLimit)
    exec(src, scope)
    return scope['search']

def run(args):
    if os.environ.get('PYTORCH_ENABLE_MPS_FALLBACK') == '1':
        raise RuntimeError('Disable CPU fallback for GPU measurement')
    torch.set_num_threads(1)
    trump, seat, pub, deals, history = fixture(args.seed,args.played,args.deals)
    report = {'seed':args.seed,'played':args.played,'deals':args.deals,'delta':args.delta,
              'trump':trump,'seat':seat,'history':history,'deal_masks':deals.tolist(),
              'torch':torch.__version__,'numpy':np.__version__,'platform':platform.platform(),
              'gpu':'Apple M5 Max / Metal MPS','cpu_threads':1,'fallback':False,
              'mps_grouping':'dense parent-tile occupancy and prefix scan; exact integer keys',
              'scope':'L0 random-policy full continuation; upload and readback included; sampling excluded',
              'backends':{}}
    for device in args.backends.split(','):
        init = time.perf_counter()
        if device == 'numpy':
            solve = bounded_numpy(args.cap,args.seconds)
            call = lambda: solve(ref.Rules(trump),seat,pub,deals,ref.random_policy,
                                 np.random.default_rng(987),args.delta)
            sync = lambda: None
        else:
            b = Backend(device)
            rules = Rules(b,trump)
            sync = b.sync
            call = lambda: search(rules,seat,Pub.from_ref(b,pub),b.array(deals),
                                  np.random.default_rng(987),args.delta,args.cap,args.seconds)
        sync()
        init_ms = (time.perf_counter()-init)*1000
        times=[]
        try:
            for rep in range(args.repeats+1):
                sync()
                start=time.perf_counter()
                values,census=call()
                sync()
                elapsed=(time.perf_counter()-start)*1000
                times.append(elapsed)
                print(json.dumps({'device':device,'rep':rep,'ms':elapsed,'peak_fibers':max(c['fibers'] for c in census),'values':values}),flush=True)
            row={'status':'complete','init_ms':init_ms,'cold_ms':times[0],
                 'warm_ms':times[1:],'median_ms':statistics.median(times[1:]),
                 'values':values,'census':census}
        except FrontierLimit as e:
            sync()
            row={'status':'capped','elapsed_ms':(time.perf_counter()-start)*1000,'census':e.args[0]}
            print(json.dumps({'device':device,**row}),flush=True)
        report['backends'][device]=row
    complete={k:v for k,v in report['backends'].items() if v['status']=='complete'}
    if 'numpy' in complete:
        oracle=complete['numpy']['values']
        for k,v in complete.items():
            assert v['values'].keys()==oracle.keys()
            error=max(abs(v['values'][t]-oracle[t]) for t in oracle)
            v['max_abs_error_vs_numpy']=error
            v['census_matches_numpy']=v['census']==complete['numpy']['census']
            assert error < 1e-6*args.deals+1e-5, (k,error)
            assert v['census_matches_numpy'], (k,'frontier census mismatch')
    if args.output:
        with open(args.output,'w') as f: json.dump(report,f,indent=2)
    return report

if __name__=='__main__':
    p=argparse.ArgumentParser()
    p.add_argument('--seed',type=int,default=1)
    p.add_argument('--played',type=int,default=0)
    p.add_argument('--deals',type=int,default=30)
    p.add_argument('--delta',type=float,default=1)
    p.add_argument('--cap',type=int,default=500_000)
    p.add_argument('--seconds',type=float,default=60)
    p.add_argument('--repeats',type=int,default=3)
    p.add_argument('--backends',default='numpy,cpu,mps')
    p.add_argument('--output')
    run(p.parse_args())
