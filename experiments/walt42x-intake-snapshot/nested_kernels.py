"""Integer support sampling and sparse expansion for the repaired ladder."""
import numpy as np
from numba import njit

@njit(cache=True)
def support_counts(allowed, caps):
    n=len(allowed)
    a,b,c=caps
    dp=np.zeros((n+1,a+1,b+1,c+1),np.int64)
    dp[n,0,0,0]=1
    for i in range(n-1,-1,-1):
        for x in range(a+1):
            for y in range(b+1):
                z=n-i-x-y
                if z<0 or z>c: continue
                total=0
                if x and allowed[i]&1: total+=dp[i+1,x-1,y,z]
                if y and allowed[i]&2: total+=dp[i+1,x,y-1,z]
                if z and allowed[i]&4: total+=dp[i+1,x,y,z-1]
                dp[i,x,y,z]=total
    return dp

@njit(cache=True)
def unrank_deals(tiles,allowed,caps,seats,seat,hand,dp,ranks):
    result=np.zeros((len(ranks),4),np.int64)
    for row in range(len(ranks)):
        a,b,c=caps
        rank=ranks[row]
        result[row,seat]=hand
        for i in range(len(tiles)):
            wa=dp[i+1,a-1,b,c] if a and allowed[i]&1 else 0
            wb=dp[i+1,a,b-1,c] if b and allowed[i]&2 else 0
            if rank<wa:
                s=seats[0];a-=1
            elif rank<wa+wb:
                rank-=wa;s=seats[1];b-=1
            else:
                rank-=wa+wb;s=seats[2];c-=1
            result[row,s]|=1<<tiles[i]
    return result

@njit(cache=True)
def prepare(tn,td,tw,deals,played,leader,trick,tlen,suit,lead,node_me,delta):
    n=len(tn)
    masks=np.empty(n,np.int64)
    counts=np.empty(n,np.int64)
    mine=np.empty(n,np.bool_)
    sampled=np.empty(n,np.bool_)
    for i in range(n):
        node=tn[i]
        seat=(leader[node]+tlen[node])%4
        hand=deals[td[i],seat]&~played[node]
        if tlen[node]:
            follow=hand&suit[lead[trick[node,0]]]
            if follow: hand=follow
        count=0;scan=hand
        while scan:
            count+=1;scan&=scan-1
        masks[i]=hand;counts[i]=count
        mine[i]=seat==node_me[node]
        sampled[i]=not mine[i] and count>0 and tw[i]*(1.0/count)<delta
    return masks,counts,mine,sampled

@njit(cache=True)
def deterministic_edges(tn,td,tw,masks,counts,mine,chosen):
    size=0
    for i in range(len(tn)): size+=counts[i] if mine[i] else 1
    keys=np.empty(size,np.int64)
    outdeal=np.empty(size,np.int64)
    weights=np.empty(size,np.float64)
    at=0
    for i in range(len(tn)):
        for t in range(28):
            if (mine[i] and masks[i]&(1<<t)) or (not mine[i] and chosen[i]==t):
                keys[at]=tn[i]*28+t;outdeal[at]=td[i];weights[at]=tw[i];at+=1
    return keys,outdeal,weights

@njit(cache=True)
def prepare_support_batch(played,leader,tlen,voids,hands,depth):
    m=len(hands);base=7-(depth-int(tlen[0]))//4
    unseen_count=28-depth-base
    tiles=np.empty((m,unseen_count),np.int64)
    allowed=np.empty((m,unseen_count),np.int64)
    seats=np.empty((m,3),np.int64);caps=np.empty((m,3),np.int64)
    acting=np.empty(m,np.int64)
    dp=np.zeros((m,unseen_count+1,base+1,base+1,base+1),np.int64)
    totals=np.empty(m,np.int64)
    for row in range(m):
        seat=(leader[row]+tlen[row])%4;acting[row]=seat
        count=0;scan=hands[row]
        while scan: count+=1;scan&=scan-1
        if count!=base or hands[row]&played[row] or hands[row]&voids[row,seat]:
            raise ValueError('own hand contradicts public information')
        at=0
        for s in range(4):
            if s==seat: continue
            size=base
            for i in range(tlen[row]):
                if (leader[row]+i)%4==s: size-=1
            seats[row,at]=s;caps[row,at]=size;at+=1
        at=0
        for t in range(28):
            if (played[row]|hands[row])&(1<<t): continue
            tiles[row,at]=t;mask=0
            for j in range(3):
                if not (voids[row,seats[row,j]]&(1<<t)): mask|=1<<j
            allowed[row,at]=mask;at+=1
        if at!=unseen_count: raise ValueError('public depth and tile mask disagree')
        a,b,c=caps[row];dp[row,unseen_count,0,0,0]=1
        for i in range(unseen_count-1,-1,-1):
            for x in range(a+1):
                for y in range(b+1):
                    z=unseen_count-i-x-y
                    if z<0 or z>c: continue
                    total=0;mask=allowed[row,i]
                    if x and mask&1: total+=dp[row,i+1,x-1,y,z]
                    if y and mask&2: total+=dp[row,i+1,x,y-1,z]
                    if z and mask&4: total+=dp[row,i+1,x,y,z-1]
                    dp[row,i,x,y,z]=total
        totals[row]=dp[row,0,a,b,c]
    return tiles,allowed,seats,caps,acting,dp,totals

@njit(cache=True)
def unrank_batch(tiles,allowed,seats,caps,acting,dp,hands,ranks):
    m,n=ranks.shape
    result=np.zeros((m*n,4),np.int64)
    for query in range(m):
        for j in range(n):
            row=query*n+j;rank=ranks[query,j];a,b,c=caps[query]
            result[row,acting[query]]=hands[query]
            for i in range(tiles.shape[1]):
                wa=dp[query,i+1,a-1,b,c] if a and allowed[query,i]&1 else 0
                wb=dp[query,i+1,a,b-1,c] if b and allowed[query,i]&2 else 0
                if rank<wa: seat=seats[query,0];a-=1
                elif rank<wa+wb: rank-=wa;seat=seats[query,1];b-=1
                else: rank-=wa+wb;seat=seats[query,2];c-=1
                result[row,seat]|=1<<tiles[query,i]
    return result
