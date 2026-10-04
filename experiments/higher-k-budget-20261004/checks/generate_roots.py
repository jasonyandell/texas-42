#!/usr/bin/env python3
"""Prespecified reviewer roots independent of parent's heldout experiment panel."""
import json,random,sys
from pathlib import Path
sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT/'experiments/partnership'))
from rules import information_state,legal_tiles,winner
CHECK=Path(__file__).resolve().parent
def main():
    rows=[]
    declarations=(*range(8),9)
    for decl in declarations:
        for seed in range(8103300+decl*1000,8103300+decl*1000+1000):
            rng=random.Random(seed);deck=list(range(28));rng.shuffle(deck)
            hands=[sorted(deck[s*7:s*7+7])for s in range(4)]
            remaining=list(map(set,hands));bidder=seed%4;leader=bidder;trick=[];plays=[];found=False
            for ply in range(28):
                seat=(leader+len(trick))%4
                request=dict(decl=decl,bid=30,bidder=bidder,seat=seat,hand=hands[seat][:],plays=plays[:],seed=9010742104)
                state=information_state(request)
                if ply>=20 and len(state['legal'])>1 and state['points'][bidder%2]<30 and state['points'][1-bidder%2]<=12:
                    rows.append(request);found=True;break
                tile=rng.choice(legal_tiles(remaining[seat],trick,decl))
                remaining[seat].remove(tile);plays.extend([seat,tile]);trick.append((seat,tile))
                if len(trick)==4:leader=winner(trick,decl);trick=[]
            if found:break
        else:raise AssertionError(decl)
    assert len(rows)==9
    path=CHECK/'fresh-roots.json';assert not path.exists(),'preserve evidence'
    path.write_text(json.dumps(rows,indent=2)+'\n')
    print(json.dumps(dict(roots=len(rows),declarations=[r['decl']for r in rows],actors=sorted({r['seat']for r in rows}),scope='Fresh late two-trick roots for finite conformance, not heldout strength or latency.')))
if __name__=='__main__':main()
