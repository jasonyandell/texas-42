#!/usr/bin/env python3
"""Pinned exploratory mirrored-game smoke; not a playing-strength study."""
import argparse
import hashlib
import json
import statistics
import subprocess
from datetime import datetime, timezone
from pathlib import Path
from audit_game import audit

p=argparse.ArgumentParser()
p.add_argument('binary',type=Path)
p.add_argument('output',type=Path)
p.add_argument('--ms',type=float,default=500)
a=p.parse_args()
a.output.mkdir(parents=True,exist_ok=False)
fixtures=[{'seed': 930100+4*i+b, 'decl':d, 'bidder':b} for i,d in enumerate([6,7,9]) for b in range(4)]
protocol={'schema':'response-ladder-smoke-v1','created':datetime.now(timezone.utc).isoformat(),
 'binary':str(a.binary.resolve()),'binary_sha256':hashlib.sha256(a.binary.read_bytes()).hexdigest(),
 'fixtures':fixtures,'budget_ms_per_game':a.ms,'arms':['declarer','defender'],
 'config':{'outer':40,'n0':8,'n1':2,'plans':8,'horizon':7,'field':'partner'},
 'interpretation':'Twelve mirrored pilot deals, descriptive only; exact serial v34 with canonical reserve is the comparator, not deployed wrapper. Equal team wall banks; all fallbacks retained. No strength claim.'}
(a.output/'protocol.json').write_text(json.dumps(protocol,indent=2)+'\n')
rows=[]
for i,f in enumerate(fixtures):
    arms=['declarer','defender'] if i%2==0 else ['defender','declarer']
    for arm in arms:
        cmd=[str(a.binary.resolve()),'game','--seed',str(f['seed']),'--decl',str(f['decl']),
             '--bidder',str(f['bidder']),'--ms',str(a.ms),'--candidate-team',arm]
        r=subprocess.run(cmd,capture_output=True,text=True,timeout=30)
        path=a.output/(str(f['seed'])+'-'+arm+'.json')
        if r.returncode:
            path.with_suffix('.error.txt').write_text(r.stderr+'\n'+r.stdout)
            raise RuntimeError('game failed '+str(cmd))
        path.write_text(r.stdout)
        game=json.loads(r.stdout)
        row=audit(game)
        row['candidate_fallbacks']=sum(p['decision']['fallback'] for p in game['plays'] if p['decision']['mode']=='anytime')
        row['baseline_fallbacks']=sum(p['decision']['fallback'] for p in game['plays'] if p['decision']['mode']=='exact-v34')
        row['priced_candidate_decisions']=sum(p['decision']['report'] is not None and p['decision']['report']['incumbent_value'] is not None for p in game['plays'])
        rows.append(row)
        print(f"{len(rows)}/24 verified",flush=True)
pairs=[]
for f in fixtures:
    ar={r['candidate_team']:r for r in rows if r['seed']==f['seed']}
    pairs.append({'seed':f['seed'],'candidate_declaring_make':ar['declarer']['made'],
                  'baseline_declaring_make':ar['defender']['made'],
                  'paired_delta':int(ar['declarer']['made'])-int(ar['defender']['made'])})
summary={'games':len(rows),'plays':sum(r['plays'] for r in rows),'all_referee_checks_pass':True,
 'median_game_ms':statistics.median(r['elapsed_ms'] for r in rows),
 'candidate_fallbacks':sum(r['candidate_fallbacks'] for r in rows),
 'baseline_fallbacks':sum(r['baseline_fallbacks'] for r in rows),
 'priced_candidate_decisions':sum(r['priced_candidate_decisions'] for r in rows),
 'canonical_certified_decisions':sum(r['certified'] for r in rows),
 'pair_wins':sum(p['paired_delta']>0 for p in pairs),
 'pair_losses':sum(p['paired_delta']<0 for p in pairs),
 'pair_ties':sum(p['paired_delta']==0 for p in pairs),
 'pairs':pairs,'rows':rows,'interpretation':protocol['interpretation']}
(a.output/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps({k:v for k,v in summary.items() if k not in ('rows','pairs')},indent=2))
