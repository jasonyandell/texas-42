"""Full-information diagnostic of this one deal, not a hidden-information policy."""
import sys,json
from pathlib import Path
from functools import lru_cache
sys.path.insert(0,'/Users/jason/.codex/worktrees/nello-player/texas-42/experiments/partnership')
from rules import TILES,legal_tiles,winner,active_actor
root=Path('/private/tmp/nello-ruby-doubles-20260921')
positions=json.loads((root/'positions.json').read_text())['positions']

@lru_cache(None)
def forced_set(hands,leader,trick):
    if len(trick)==3:
        lead=winner(trick,8)
        if lead==0:return True
        if not hands[0]:return False
        return forced_set(hands,lead,())
    actor=active_actor(leader,len(trick),0,'nello')
    outcomes=[]
    for t in legal_tiles(hands[actor],trick,8):
        next_h=list(hands);next_h[actor]=tuple(u for u in hands[actor] if u!=t)
        outcomes.append(forced_set(tuple(next_h),leader,trick+((actor,t),)))
    return all(outcomes) if actor==0 else any(outcomes)

rows=[]
for pos in positions:
    hands=tuple(tuple(h) for h in pos['remaining'])
    for t in hands[3]:
        next_h=list(hands);next_h[3]=tuple(u for u in hands[3] if u!=t)
        outcomes=[]
        for you in legal_tiles(hands[0],[(3,t)],8):
            for earl in legal_tiles(hands[1],[(3,t),(0,you)],8):
                outcomes.append({'you':''.join(map(str,TILES[you])),'earl':''.join(map(str,TILES[earl])),'winner':winner([(3,t),(0,you),(1,earl)],8)})
        rows.append({'ply':pos['ply'],'lead':''.join(map(str,TILES[t])),'full_information_forced_set':forced_set(tuple(next_h),3,((3,t),)),'first_trick':outcomes})
print(json.dumps(rows,indent=2))
(root/'actual.json').write_text(json.dumps({'rows':rows,'cache':str(forced_set.cache_info())},indent=2)+'\n')
