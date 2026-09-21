"""Independent import of Plunge's straight-contract finished-hand links."""
from rules import TILES, legal_tiles, replay_record, winner
from player import normalize


def tile_id(text):
    if len(text)!=2 or any(c not in '0123456' for c in text): raise ValueError('invalid domino')
    pair=tuple(map(int,text))
    if pair not in TILES: raise ValueError('domino must use high pip first')
    return TILES.index(pair)


def decode_hand(code):
    if not isinstance(code,str) or len(code)>1000 or code[:3] not in ('v1c','v1t','v1f'):
        raise ValueError('expected a Plunge v1 hand code')
    if len(code)<61 or code[3] not in '0123' or code[60]!='.': raise ValueError('malformed hand header')
    shaker=int(code[3]);hands=[[tile_id(code[4+14*s+2*j:6+14*s+2*j]) for j in range(7)] for s in range(4)]
    if sorted(t for hand in hands for t in hand)!=list(range(28)): raise ValueError('deal does not partition all dominoes')
    cursor=61;bid=0;bidder=None;strength=0;marks=0
    for i in range(4):
        if code[cursor:cursor+1]=='P':cursor+=1;continue
        token=code[cursor:cursor+2]
        if len(token)==2 and token[0]=='M' and token[1] in '123456789':
            level=int(token[1]);value=42
            if (level>2 and level!=marks+1) or 41+level<=strength:raise ValueError('invalid marks auction')
            marks=level;strength=41+level
        else:
            if len(token)!=2 or not token.isascii() or not token.isdigit(): raise ValueError('research import supports straight bids only')
            value=int(token)
            if not 30<=value<=41 or value<=strength: raise ValueError('invalid auction')
            strength=value
        bid=value;bidder=(shaker+1+i)%4;cursor+=2
    if bidder is None: raise ValueError('hand has no winning bidder')
    if code[cursor:cursor+1]!='D' or code[cursor+1:cursor+2] not in tuple('012345679'):
        raise ValueError('research import needs a straight declaration')
    decl=int(code[cursor+1]);cursor+=2
    if not 8<=len(code)-cursor<=56 or (len(code)-cursor)%8:
        raise ValueError('expected complete tricks from a finished hand')
    remaining=[set(h) for h in hands];leader=bidder;trick=[];record=[]
    while cursor<len(code):
        tile=tile_id(code[cursor:cursor+2]);cursor+=2;seat=(leader+len(trick))%4
        if tile not in legal_tiles(remaining[seat],trick,decl): raise ValueError('illegal recorded play')
        record.extend([seat,tile]);remaining[seat].remove(tile);trick.append((seat,tile))
        if len(trick)==4:
            leader=winner(trick,decl);trick=[]
            points,_,_,_=replay_record(hands,record,decl,bidder)
            if (points[bidder%2]>=bid or points[1-bidder%2]>42-bid) and cursor<len(code):
                raise ValueError('record continues after the table settled the contract')
    points,_,remaining,trick=replay_record(hands,record,decl,bidder)
    if trick or not (points[bidder%2]>=bid or points[1-bidder%2]>42-bid):
        raise ValueError('hand has not settled the contract')
    return dict(decl=decl,bid=bid,bidder=bidder,hands=hands,plays=record,points=points,shaker=shaker)


def flag_root(code, ply, seed):
    if type(ply) is not int or not 0<=ply<28: raise ValueError('play index outside 0..27')
    game=decode_hand(code)
    if 2*ply>=len(game['plays']):raise ValueError('play index after the finished hand')
    seat,played=game['plays'][2*ply:2*ply+2]
    request=normalize(dict(decl=game['decl'],bid=game['bid'],bidder=game['bidder'],seat=seat,
                           hand=sorted(game['hands'][seat]),plays=game['plays'][:2*ply],seed=seed))
    return request,played,game
