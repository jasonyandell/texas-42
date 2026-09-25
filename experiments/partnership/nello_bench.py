"""Small reproducible Nel-O defense screen, not a calibrated strength result.

Two fixed low hands, eight hidden deals each; paired L1 and random defenders.
Both arms use the same L1 declarer. Retain whole hands and every own/public
request so illegal play, fallback and outcome failures remain replayable.
Run under the packet watchdog; output is incrementally replaced after a game.
"""
import argparse
import json
import random
import hashlib
from pathlib import Path

from rules import active_actor, legal_tiles, replay_record, winner
from table_player import decide, BINARY


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('output',type=Path)
    parser.add_argument('--panel',choices=('low','mixed'),default='mixed')
    args=parser.parse_args()
    assert not args.output.exists(), 'use a fresh receipt'
    rows=[]
    low_hands=[[0,1,2,3,4,6,7], [1,3,4,6,7,10,11]]
    if args.panel=='mixed':
        fixtures=json.loads((Path(__file__).resolve().parents[2]/'walt/walt-player/tests/fixtures/nello.json').read_text())
        low_hands=[fixtures[0]['hands'][0],fixtures[4]['hands'][0]]+[
            sorted(random.Random(428900+i).sample(range(28),7)) for i in range(4)]
    identity=dict(schema='nello-defense-screen-v1',panel=args.panel,binary_sha256=hashlib.sha256(BINARY.read_bytes()).hexdigest(),
                  player=dict(worlds=40,inner_worlds=8,budget_ms=14000,partner=False))
    for shape,own in enumerate(low_hands):
        for deal in range(8):
            seed=4208900+shape*100+deal
            rng=random.Random(seed)
            unseen=sorted(set(range(28))-set(own));rng.shuffle(unseen)
            bidder=deal%4
            hands=[None]*4;hands[bidder]=own
            for i,s in enumerate(s for s in range(4) if s!=bidder): hands[s]=unseen[i*7:i*7+7]
            for defense in ('random','walt'):
                remaining=[set(h) for h in hands]
                lead=bidder;trick=[];record=[];decisions=[];completed=0
                field=random.Random(seed+7000)
                while True:
                    seat=active_actor(lead,len(trick),bidder,'nello')
                    legal=legal_tiles(remaining[seat],trick,8)
                    req=dict(contract='nello',decl=8,bid=1,bidder=bidder,seat=seat,
                             hand=sorted(hands[seat]),plays=record.copy(),seed=seed)
                    if seat==bidder or defense=='walt':
                        response=decide(req)
                        tile=response['choice']
                    else:
                        tile=field.choice(legal)
                        response=dict(choice=tile,route='random',elapsed_us=0)
                    assert tile in legal
                    decisions.append(dict(request=req,response=response))
                    record.extend([seat,tile]);remaining[seat].remove(tile);trick.append((seat,tile))
                    if len(trick)==3:
                        lead=winner(trick,8);trick=[];completed+=1
                        if lead==bidder or completed==7: break
                points,check_lead,check_remaining,check_trick=replay_record(hands,record,8,bidder,'nello')
                assert (check_lead,check_remaining,check_trick)==(lead,remaining,trick)
                rows.append(dict(seed=seed,shape=shape,bidder=bidder,hands=hands,defense=defense,
                                 made=lead!=bidder,tricks=completed,points=points,decisions=decisions))
                args.output.write_text(json.dumps(dict(**identity,games=rows),indent=2)+'\n')
                print(f'{seed} {defense}: {"made" if lead!=bidder else "set"} in {completed}',flush=True)
    for defense in ('random','walt'):
        arm=[r for r in rows if r['defense']==defense]
        print(defense,dict(hands=len(arm),sets=sum(not r['made'] for r in arm)),flush=True)


if __name__=='__main__': main()
