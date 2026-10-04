#!/usr/bin/env python3
"""Pinned phone-WASM play-only arena. Run ONLY through run_capped.py.

Reuses experiments/partnership/rules.py's independent referee. Every invocation
plays one 28-move game; rotate each source deal through all four seats and swap
the candidate partnership. Aggregate only complete eight-game deal blocks.
"""
import argparse
from collections import Counter
import hashlib
import json
import math
import os
from pathlib import Path
import random
import select
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT.parent / 'partnership'))
from rules import legal_tiles, winner, trick_points, replay_record, information_state

PHONE = ROOT / 'reference/production-phone/walt-player.wasm'
PUBLIC_POLICY_SEED = 7042104  # independent of hidden source-deal seeds

def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def make_request(hands, seat, bidder, plays, decl):
    # Strict information boundary. Referee hands never enter this payload.
    return dict(decl=decl, bid=30, bidder=bidder, seat=seat,
                hand=list(hands[seat]), plays=list(plays), seed=PUBLIC_POLICY_SEED)

def profile(req, partner):
    # Plunge a0d9fa80 native.ts livePlayerCall, thinkDeeper=false, straight 42.
    opening = req['seat'] == req['bidder'] and not req['plays']
    return dict(request=req, worlds=160 if opening else 40,
                partner=False if opening else partner,
                budget_ms=20000 if opening else 14000)

class Worker:
    def __init__(self, wasm):
        self.wasm = wasm
        self.proc = None
        self.buffer = b''
    def start(self):
        self.proc = subprocess.Popen(['node', str(ROOT/'tools/phone_worker.mjs'), str(self.wasm)],
            stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, bufsize=0)
    def close(self):
        if self.proc:
            if self.proc.poll() is None: self.proc.kill()
            self.proc.wait(timeout=2)
            for stream in (self.proc.stdin, self.proc.stdout, self.proc.stderr): stream.close()
            self.proc = None
            self.buffer = b''
    def call(self, call, game_deadline):
        started = time.monotonic()
        if self.proc is None: self.start()
        deadline = min(game_deadline, started + call['budget_ms']/1000 + 4)
        self.proc.stdin.write((json.dumps(call)+'\n').encode())
        self.proc.stdin.flush()
        saved, checkpoints = None, 0
        def interrupted(reason):
            self.close()
            if saved is None: raise RuntimeError(reason + '; no retained checkpoint')
            return saved, dict(host_ms=(time.monotonic()-started)*1000,
                checkpoints=checkpoints, interrupted=True, interruption_reason=reason,
                initialization_ms=None)
        while True:
            left = deadline-time.monotonic()
            if left <= 0 or not select.select([self.proc.stdout], [], [], left)[0]:
                return interrupted('Host deadline; last completed decision retained')
            chunk = os.read(self.proc.stdout.fileno(), 65536)
            if not chunk: return interrupted('Phone worker exited')
            self.buffer += chunk
            if len(self.buffer) > 2_000_000: raise RuntimeError('Oversized response')
            while b'\n' in self.buffer:
                line, self.buffer = self.buffer.split(b'\n', 1)
                msg = json.loads(line)
                if 'error' in msg: return interrupted(msg['error'])
                if 'checkpoint' in msg:
                    saved = msg['checkpoint']; checkpoints += 1
                if 'result' in msg:
                    if 'error' in msg['result']: raise RuntimeError(msg['result']['error'])
                    return msg['result'], dict(host_ms=(time.monotonic()-started)*1000,
                        checkpoints=checkpoints, interrupted=False,
                        initialization_ms=msg.get('initialization_ms'))

def run(args):
    output = args.output.resolve()
    if output.exists(): raise ValueError('Refusing to overwrite an existing game')
    output.parent.mkdir(parents=True, exist_ok=True)
    manifest=json.loads((ROOT/'reference/production-phone/manifest.json').read_text())
    assert sha(PHONE)==manifest['wasm_sha256'], 'Pinned phone artifact hash mismatch'
    tiles = list(range(28)); random.Random(args.seed).shuffle(tiles)
    base = [sorted(tiles[7*s:7*s+7]) for s in range(4)]
    hands = [base[(s-args.rotation)%4] for s in range(4)]
    bidder = args.rotation
    candidate_team = (bidder + (args.role == 'defending')) % 2
    baseline, candidate = Worker(PHONE), Worker(args.candidate_wasm)
    workers = [baseline, candidate]
    remaining = [set(h) for h in hands]
    plays, moves, points = [], [], [0, 0]
    leader = bidder
    started = time.monotonic(); deadline = started+args.seconds
    try:
        for _ in range(7):
            trick = []
            for _ in range(4):
                if time.monotonic() >= deadline: raise TimeoutError('Game deadline; incomplete game excluded')
                seat = (leader+len(trick))%4
                is_candidate = seat%2 == candidate_team
                req = make_request(hands, seat, bidder, plays, args.decl)
                call = profile(req, args.candidate_partner if is_candidate else True)
                response, timing = workers[int(is_candidate)].call(call, deadline)
                state = information_state(req)
                legal = legal_tiles(remaining[seat], trick, args.decl)
                if response['choice'] not in legal or sorted(response['legal']) != legal:
                    raise AssertionError('Illegal move or legal-set disagreement')
                if response['points'] != points or response['leader'] != leader:
                    raise AssertionError('Score or leader disagreement')
                if sorted(state['legal']) != legal: raise AssertionError('Information-state legality mismatch')
                tile = response['choice']
                moves.append(dict(call=call, response=response, timing=timing, candidate=is_candidate))
                remaining[seat].remove(tile); plays.extend([seat,tile]); trick.append((seat,tile))
            leader = winner(trick, args.decl); points[leader%2] += trick_points(trick)
    finally:
        for worker in workers: worker.close()
    audited, _, remain, tail = replay_record(hands, plays, args.decl, bidder)
    assert audited == points and sum(points)==42 and not tail and all(not h for h in remain)
    report = dict(schema='walt-astra-sol-game-v1', seed=args.seed, rotation=args.rotation,
        role=args.role, decl=args.decl, bid=30, bidder=bidder, hands=hands,
        policy_seed=PUBLIC_POLICY_SEED, candidate_label=args.candidate_label,
        candidate_partner=args.candidate_partner, candidate_wasm_sha256=sha(args.candidate_wasm),
        phone_wasm_sha256=sha(PHONE), phone_profile='a0d9fa80 native-partner; thinkDeeper=false; straight-42',
        referee_sha256=sha(ROOT.parent/'partnership/rules.py'),
        arena_sha256=sha(__file__), adapter_sha256=sha(ROOT/'tools/phone_worker.mjs'),
        node_version=subprocess.check_output(['node','--version'],text=True,timeout=3).strip(),
        python_version=sys.version, game_seconds=time.monotonic()-started,
        game_limit_seconds=args.seconds, points=points, made=points[bidder%2]>=30,
        moves=moves, complete=True)
    output.write_text(json.dumps(report,indent=2)+'\n')
    print(json.dumps({k:report[k] for k in ('seed','rotation','role','points','made','game_seconds')}))

def summarize(args):
    rows=[json.loads(p.read_text()) for p in sorted(args.directory.glob('game-*.json'))]
    if not rows: raise ValueError('No complete game receipts')
    signatures={(r['candidate_label'],r['candidate_wasm_sha256'],r['candidate_partner'],
                 r['phone_wasm_sha256'],r['bid'],r['policy_seed'],r['phone_profile'],
                 r['arena_sha256'],r['adapter_sha256'],r['referee_sha256']) for r in rows}
    assert len(signatures)==1, 'Incompatible campaign identities'
    groups={}
    for r in rows:
        assert r['complete'] and len(r['moves'])==28
        key=(r['seed'],r['rotation'],r['role'])
        assert key not in groups, 'Duplicate game'; groups[key]=r
    plan_path=args.directory/'plan.json'
    plan=json.loads(plan_path.read_text()) if plan_path.exists() else None
    attempts=json.loads((args.directory/'attempts.json').read_text()) if plan else []
    seeds=sorted({g['seed'] for g in plan['games']} if plan else {r['seed'] for r in rows})
    cluster=[]; missing=[]; pairs=[]
    for seed in seeds:
        ds=[]
        for rotation in range(4):
            a=groups.get((seed,rotation,'declaring')); b=groups.get((seed,rotation,'defending'))
            if not a or not b: missing.append([seed,rotation]); continue
            assert a['decl']==b['decl']
            d=int(a['made'])-int(b['made']); ds.append(d)
        if len(ds)==4:
            cluster.append(sum(ds)/4);pairs.extend(ds)
    result=dict(games=len(rows), complete_source_deals=len(cluster), missing_pairs=missing,
        pair_wins=pairs.count(1), pair_losses=pairs.count(-1), pair_ties=pairs.count(0),
        planned_games=len(plan['games']) if plan else None,
        attempts=attempts, censored=bool(missing),
        strength_claim=False, uncertainty_unit='whole source deal, averaging four rotations')
    if cluster and not missing:
        mean=sum(cluster)/len(cluster)
        radius=math.sqrt(2*math.log(40)/len(cluster))
        result.update(mean_paired_make_advantage=mean,
            hoeffding_95=[max(-1,mean-radius),min(1,mean+radius)],
            uncertainty_assumption='independent source deals; fixed profiles, bounded differences [-1,1]; conservative smoke interval')
    elif missing:
        result['uncertainty_refusal']='Incomplete/censored panel; no population interval reported'
    for is_candidate,label in ((False,'phone'),(True,'candidate')):
        moves=[m for r in rows for m in r['moves'] if m['candidate']==is_candidate]
        times=sorted(m['timing']['host_ms'] for m in moves)
        result[label]=dict(moves=len(moves), mean_host_ms=sum(times)/len(times),
            median_host_ms=times[len(times)//2], p95_host_ms=times[math.ceil(.95*len(times))-1],
            routes=dict(Counter(m['response']['route'] for m in moves)),
            interruptions=sum(m['timing']['interrupted'] for m in moves),
            budget_ms=sum(m['call']['budget_ms'] for m in moves))
    print(json.dumps(result,indent=2))

def main():
    p=argparse.ArgumentParser(description=__doc__); sp=p.add_subparsers(dest='cmd',required=True)
    r=sp.add_parser('run')
    r.add_argument('--seed',type=int,required=True); r.add_argument('--rotation',type=int,choices=range(4),required=True)
    r.add_argument('--role',choices=['declaring','defending'],required=True)
    r.add_argument('--decl',type=int,choices=list(range(8))+[9],default=6)
    r.add_argument('--candidate-wasm',type=Path,default=PHONE)
    r.add_argument('--candidate-partner',action='store_true')
    r.add_argument('--candidate-label',default='phone-l1-ablation')
    r.add_argument('--seconds',type=float,default=240); r.add_argument('--output',type=Path,required=True)
    s=sp.add_parser('summarize'); s.add_argument('directory',type=Path)
    args=p.parse_args()
    if args.cmd=='run':
        if not 0<args.seconds<=270:p.error('game seconds must be in (0,270]')
        run(args)
    else:summarize(args)
if __name__=='__main__':main()
