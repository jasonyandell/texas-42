"""Compare a complete Metal hand with the supplied NumPy player and referee it."""
import contextlib
import io
import json
import time
import numpy as np
import walt42x as ref
from verify import legal, play

original=ref.walt
moves=[]
def recording(assume,n=8,delta=0):
    base=original(assume,n,delta)
    def policy(rules,seat,hand,pub,rng,voids=(0,0,0,0)):
        P=base(rules,seat,hand,pub,rng,voids)
        moves.append((int(seat[0]),int(P[0].argmax())))
        return P
    return policy

if __name__=='__main__':
    ref.walt=recording
    transcript=io.StringIO()
    start=time.perf_counter()
    with contextlib.redirect_stdout(transcript): ref.main(1,1,0)
    elapsed=(time.perf_counter()-start)*1000
    gpu=json.load(open('results/fixed-game-mps-cold.json'))
    assert moves==[(m['seat'],m['tile']) for m in gpu['moves']]
    warm=json.load(open('results/fixed-game-mps-warm.json'))
    assert moves==[(m['seat'],m['tile']) for m in warm['moves']]
    # Independent legality/scoring, seeded original hands and contract as printed.
    hands=[]
    for line in transcript.getvalue().splitlines():
        if line.startswith('  seat '):
            hands.append(sum(1 << ref.TILES.index(tuple(map(int,x.split('-'))))
                             for x in line.split(': ')[1].split()))
    trump=int(transcript.getvalue().splitlines()[0].split()[1][0])
    state=(0,1,(),0,0)
    for seat,tile in moves:
        assert seat==(state[1]+len(state[2]))%4
        assert tile in legal(hands[seat]&~state[0],state[2],trump)
        state=play(state,tile,trump)
    assert state[3:]==(30,12)
    report={'status':'pass','moves':len(moves),'numpy_trace_matches_gpu':True,
            'independent_final_score':list(state[3:]),'numpy_whole_game_ms':elapsed}
    with open('results/game-verification.json','w') as f: json.dump(report,f,indent=2)
    print(json.dumps(report))
