#!/usr/bin/env python3
"""One bounded sequential pilot. All children inherit the outer watchdog group."""
import argparse
import json
from pathlib import Path
import subprocess
import sys
import time

ROOT=Path(__file__).resolve().parents[1]
p=argparse.ArgumentParser(description=__doc__)
p.add_argument('--output',type=Path,required=True)
p.add_argument('--seeds',type=int,nargs='+',default=[104200,104201,104202,104203])
p.add_argument('--candidate-partner',action='store_true')
p.add_argument('--candidate-label',default='phone-l1-ablation')
args=p.parse_args()
args.output.mkdir(parents=True,exist_ok=False)
games=[]
for i,seed in enumerate(args.seeds):
    for rotation in range(4):
        roles=['declaring','defending'] if rotation%2==0 else ['defending','declaring']
        for role in roles:games.append(dict(seed=seed,decl=[6,0,7,9][i%4],rotation=rotation,role=role))
plan=dict(scope='play-only; fixed bid30; four rotations and partnership swap per source deal',
    candidate_label=args.candidate_label,candidate_partner=args.candidate_partner,
    games=games, policy_seed=7042104, panel_limit_seconds=280, game_limit_seconds=240)
(args.output/'plan.json').write_text(json.dumps(plan,indent=2)+'\n')
attempts=[];start=time.monotonic()
for game in games:
    left=280-(time.monotonic()-start)
    if left<1:break
    name=f"game-{game['seed']}-r{game['rotation']}-{game['role']}"
    command=[sys.executable,str(ROOT/'tools/arena.py'),'run',
        '--candidate-label',args.candidate_label,'--output',str(args.output/(name+'.json')),
        '--seconds',str(min(240,left))]
    if args.candidate_partner:command.append('--candidate-partner')
    for k,v in game.items():command.extend(['--'+k,str(v)])
    attempt=dict(**game,status='running',command=command)
    attempts.append(attempt)
    (args.output/'attempts.json').write_text(json.dumps(attempts,indent=2)+'\n')
    tick=time.monotonic()
    with (args.output/(name+'.log')).open('w') as log:
        try:
            result=subprocess.run(command,stdout=log,stderr=subprocess.STDOUT,timeout=left)
            attempt.update(status='completed' if result.returncode==0 else 'failed',exit_code=result.returncode)
        except subprocess.TimeoutExpired:
            attempt.update(status='timeout',exit_code=124)
    attempt['elapsed_seconds']=time.monotonic()-tick
    (args.output/'attempts.json').write_text(json.dumps(attempts,indent=2)+'\n')
    print(json.dumps({k:v for k,v in attempt.items() if k!='command'}),flush=True)
    if attempt['status']=='timeout':break
if len(attempts)!=len(games) or any(a['status']!='completed' for a in attempts):raise SystemExit(1)
