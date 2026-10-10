"""Referee complete/incomplete benchmark receipts and compare equal-policy runs."""
import argparse
import json
from pathlib import Path
from verify import legal,play

def verify(j):
    hands=j['hands'];trump=j['trump'];state=(0,1,(),0,0)
    for row in j['moves']:
        seat,tile=row['seat'],row['tile']
        assert seat==(state[1]+len(state[2]))%4
        allowed=legal(hands[seat]&~state[0],state[2],trump)
        assert tile in allowed
        values=row.get('values')
        if values is not None:
            scores={int(k):v for k,v in values.items()}
            assert set(scores)==set(allowed)
            n=j['spec']['samples'][j['level']-1]
            assert all(0<=v<=n+1e-10 for v in scores.values())
            best=(max if seat%2 else min)(scores.values())
            assert tile==min(k for k,v in scores.items() if v==best)
        elif 'values' in row:
            assert len(allowed)==1,'unpriced non-forced decision'
        state=play(state,tile,trump)
        assert state[3:]==(row['t1'],row['t0'])
    if j['status']=='complete':
        assert state[3]>=30 or state[4]>12
        assert j['outcome']==('made' if state[3]>=30 else 'set')
    else:
        assert j['outcome'] is None and j['reason']
    return len(j['moves'])

def main(paths,output='results/repaired-game-verification.json'):
    rows=[];seen={};comparisons=0
    for path in paths:
        packet=json.loads(Path(path).read_text())
        games=packet.get('results',[packet])
        for j in games:
            count=verify(j)
            key=(j.get('policy_version','prototype'),j['seed'],json.dumps(j['spec'],sort_keys=True),j['level'],j.get('reuse_deals',False))
            trace=[(m['seat'],m['tile'],m['t1'],m['t0'],m.get('values')) for m in j['moves']]
            if j['status']=='complete':
                if key in seen:
                    assert trace==seen[key],(path,j['seed'],'schedule/cache changed the trace')
                    comparisons+=1
                seen[key]=trace
            rows.append({'path':path,'seed':j['seed'],'status':j['status'],'verified_moves':count})
    result={'status':'pass','runs':rows,'equal_policy_trace_comparisons':comparisons}
    Path(output).write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps({'status':'pass','runs':len(rows),'equal_policy_trace_comparisons':comparisons}))

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('paths',nargs='+')
    p.add_argument('--output',default='results/repaired-game-verification.json')
    args=p.parse_args();main(args.paths,args.output)
