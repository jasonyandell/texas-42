#!/usr/bin/env python3
"""Exact receipt for a new two-context reachable-support lower family.

Standard library only. Deterministic. No network and no file I/O.
"""
from __future__ import annotations

from collections import defaultdict, Counter
from functools import lru_cache
from itertools import combinations, permutations, product
from math import comb
import gc
import sys
import time

PIPS = tuple(range(7))
DOMINOES = tuple((h,l) for h in PIPS for l in range(h+1))
INDEX = {d:i for i,d in enumerate(DOMINOES)}
ALL_MASK = (1<<28)-1
DECLARATIONS = PIPS + ("DT","NT")
MODULE_DECLARATIONS = PIPS + ("DT",)
CALLED = 7
HIDDEN = (1,2,3)


def C(n,k):
    return comb(n,k) if 0 <= k <= n else 0

def bit_indices(mask):
    while mask:
        bit = mask & -mask
        yield bit.bit_length()-1
        mask ^= bit

def tile_mask(tiles):
    out=0
    for d in tiles: out |= 1<<INDEX[tuple(d)]
    return out

def contains(d,p): return d[0]==p or d[1]==p

def is_double(d): return d[0]==d[1]

def count_label(d):
    if d in ((5,0),(4,1),(3,2)): return 5
    if d in ((6,4),(5,5)): return 10
    return 0

def called_mask(decl):
    if decl in PIPS:
        return sum(1<<i for i,d in enumerate(DOMINOES) if contains(d,decl))
    if decl == "DT":
        return sum(1<<i for i,d in enumerate(DOMINOES) if is_double(d))
    if decl == "NT": return 0
    raise ValueError(decl)

CALLED_MASK={d:called_mask(d) for d in DECLARATIONS}

def effective_mask(decl,q):
    if q==CALLED: return CALLED_MASK[decl]
    natural=sum(1<<i for i,d in enumerate(DOMINOES) if contains(d,q))
    return natural & ~CALLED_MASK[decl]

EFFECTIVE={(d,q):effective_mask(d,q) for d in DECLARATIONS for q in range(8)}

def led_context_index(decl,i):
    return CALLED if CALLED_MASK[decl]&(1<<i) else DOMINOES[i][0]

LED_CONTEXT={(d,i):led_context_index(d,i) for d in DECLARATIONS for i in range(28)}
LEAD_FIBER={(d,q):sum(1<<i for i in range(28) if LED_CONTEXT[(d,i)]==q)
            for d in DECLARATIONS for q in range(8)}

def rank_value(decl,i):
    h,l=DOMINOES[i]
    if decl=="DT" and h==l: return h
    if h==l and decl!="DT": return 100
    return h+l

def trick_key(decl,q,i):
    bit=1<<i
    if CALLED_MASK[decl]&bit: tier=2
    elif EFFECTIVE[(decl,q)]&bit: tier=1
    else: tier=0
    return (tier, rank_value(decl,i) if tier else 0)

def trick_winner(decl,plays):
    q=LED_CONTEXT[(decl,plays[0][1])]
    keys=[trick_key(decl,q,i) for _,i in plays]
    mx=max(keys)
    assert keys.count(mx)==1
    return plays[keys.index(mx)][0]

def legal_indices(decl, hand_mask, current_plays):
    if not current_plays: return hand_mask
    q=LED_CONTEXT[(decl,current_plays[0][1])]
    followers=hand_mask&EFFECTIVE[(decl,q)]
    return followers if followers else hand_mask


def check_rules():
    assert len(DOMINOES)==28 and len(set(DOMINOES))==28
    assert sum(count_label(d) for d in DOMINOES)==35
    for p in PIPS:
        assert sum(contains(d,p) for d in DOMINOES)==7
    expected={**{p:(set(PIPS)-{p})|{7} for p in PIPS},
              "DT":{1,2,3,4,5,6,7}, "NT":set(PIPS)}
    for decl in DECLARATIONS:
        qs={q for q in range(8) if LEAD_FIBER[(decl,q)]}
        assert qs==expected[decl]
        assert sorted(LEAD_FIBER[(decl,q)].bit_count() for q in qs)==list(range(1,8))
        union=0
        for q in qs:
            assert not union&LEAD_FIBER[(decl,q)]
            union |= LEAD_FIBER[(decl,q)]
        assert union==ALL_MASK
    cases=0
    for decl in DECLARATIONS:
        for lead in range(28):
            rest=[i for i in range(28) if i!=lead]
            q=LED_CONTEXT[(decl,lead)]
            for others in combinations(rest,3):
                keys=[trick_key(decl,q,i) for i in (lead,)+others]
                assert keys.count(max(keys))==1
                cases+=1
    assert cases==737_100
    return cases

# ------------------------------------------------------------------
# Exact support algebra
# ------------------------------------------------------------------

def hall_feasible(universe, possible, capacities):
    universe=frozenset(universe)
    if len(universe)!=sum(capacities): return False
    for sm in range(1,1<<len(capacities)):
        n=set(); quota=0
        for s in range(len(capacities)):
            if sm&(1<<s):
                n.update(possible[s]&universe); quota+=capacities[s]
        if len(n)<quota: return False
    return True

def marginal_sets(universe, possible, capacities):
    universe=frozenset(universe)
    if not hall_feasible(universe,possible,capacities): return None
    out=[set() for _ in capacities]
    for d in universe:
        for s,k in enumerate(capacities):
            if k==0 or d not in possible[s]: continue
            u=universe-{d}
            p=tuple(frozenset(x)-{d} for x in possible)
            kk=list(capacities); kk[s]-=1
            if hall_feasible(u,p,tuple(kk)): out[s].add(d)
    return tuple(frozenset(x) for x in out)

def fiber_count_dp(universe, possible, capacities):
    dp={(0,)*len(capacities):1}
    for d in sorted(universe):
        nxt=defaultdict(int)
        for occ,v in dp.items():
            for s in range(len(capacities)):
                if d in possible[s] and occ[s]<capacities[s]:
                    o=list(occ); o[s]+=1; nxt[tuple(o)]+=v
        dp=nxt
    return dp.get(tuple(capacities),0)

def enumerate_worlds(universe, possible, capacities):
    universe=tuple(sorted(universe)); out=set()
    def rec(s,rem,hands):
        if s==len(capacities)-1:
            h=frozenset(rem)
            if len(h)==capacities[s] and h<=possible[s]: out.add(tuple(hands+[h]))
            return
        allowed=sorted(set(rem)&set(possible[s]))
        for ch in combinations(allowed,capacities[s]):
            h=frozenset(ch)
            rec(s+1,tuple(d for d in rem if d not in h),hands+[h])
    rec(0,universe,[])
    return frozenset(out)

def find_world(universe,possible,capacities):
    order=sorted(universe,key=lambda d:sum(d in p for p in possible))
    hands=[set() for _ in capacities]
    rem=list(capacities)
    def rec(i):
        if i==len(order): return all(x==0 for x in rem)
        d=order[i]
        for s in range(len(capacities)):
            if rem[s] and d in possible[s]:
                hands[s].add(d); rem[s]-=1
                suffix=frozenset(order[i+1:])
                poss2=tuple(frozenset(p)&suffix for p in possible)
                if hall_feasible(suffix,poss2,tuple(rem)) and rec(i+1): return True
                rem[s]+=1; hands[s].remove(d)
        return False
    assert rec(0)
    return tuple(frozenset(h) for h in hands)


def check_one_context_cell_lemma():
    profiles=((5,5,6),(5,5,5),(4,5,5),(4,4,5),(4,4,4),(3,4,4),(3,3,4),(3,3,3))
    checked=0
    for capacities in profiles:
        n=sum(capacities); U=frozenset(range(n))
        for xcount in range(2,min(7,n)+1):
            X=frozenset(range(xcount)); N=U-X
            for msize in (1,2):
                for M in combinations(range(3),msize):
                    P=tuple(N if s in M else U for s in range(3))
                    if not hall_feasible(U,P,capacities): continue
                    marg=marginal_sets(U,P,capacities); assert marg is not None
                    comp=frozenset(s for s in range(3) if s not in M)
                    for d in X:
                        holders=frozenset(s for s in range(3) if d in marg[s])
                        assert holders==comp
                    checked+=1
    assert checked==216
    return checked

# ------------------------------------------------------------------
# Prior 001 module/upward-closure machinery
# ------------------------------------------------------------------

def admissible_groups(decl,size):
    groups=set()
    for q in range(8):
        lead=LEAD_FIBER[(decl,q)]
        if not lead: continue
        suit=EFFECTIVE[(decl,q)]
        members=tuple(bit_indices(suit))
        for choice in combinations(members,size):
            mask=sum(1<<i for i in choice)
            if mask&lead: groups.add(mask)
    return tuple(sorted(groups))

GROUPS={d:{s:admissible_groups(d,s) for s in (2,3,4)} for d in MODULE_DECLARATIONS}

@lru_cache(maxsize=None)
def module_unions(decl,count):
    unions={0}
    for _ in range(count):
        nxt=set()
        for used in unions:
            for g in GROUPS[decl][4]:
                if not used&g: nxt.add(used|g)
        unions=nxt
    return frozenset(unions)

HALF=14; NHALF=1<<HALF; LOW_MASK=NHALF-1

def make_target_masks():
    masks=[]
    for i in range(HALF):
        block=1<<i; chunk=(1<<block)-1; mask=0
        for start in range(0,NHALF,2*block): mask |= chunk<<(start+block)
        masks.append(mask)
    return tuple(masks)
TARGET_MASKS=make_target_masks()
POPCOUNT_MASKS=[0]*(HALF+1)
for h in range(NHALF): POPCOUNT_MASKS[h.bit_count()] |= 1<<h


def witness_superset_rows(witnesses):
    rows=[0]*NHALF
    for w in witnesses: rows[w&LOW_MASK] |= 1<<(w>>HALF)
    for i in range(HALF):
        bit=1<<i
        for low in range(NHALF):
            if low&bit: rows[low] |= rows[low^bit]
    for low,val in enumerate(rows):
        for i in range(HALF): val |= (val<<(1<<i))&TARGET_MASKS[i]
        rows[low]=val
    return rows

@lru_cache(maxsize=None)
def high_category_masks(suit_high,forbidden_high):
    cats=[[0]*8 for _ in range(15)]
    for high in range(NHALF):
        if high&forbidden_high: continue
        cats[high.bit_count()][(high&suit_high).bit_count()] |= 1<<high
    return tuple(tuple(x) for x in cats)

def count_from_rows(rows,total_size,suit_mask,allowed_suit_counts,forbidden_mask=0):
    sl=suit_mask&LOW_MASK; sh=suit_mask>>HALF
    fl=forbidden_mask&LOW_MASK; fh=forbidden_mask>>HALF
    cats=high_category_masks(sh,fh)
    total=0
    for low,row in enumerate(rows):
        if low&fl: continue
        hp=total_size-low.bit_count()
        if not 0<=hp<=HALF: continue
        ls=(low&sl).bit_count(); allowed=0
        for sc in allowed_suit_counts:
            hs=sc-ls
            if 0<=hs<8: allowed |= cats[hp][hs]
        total += (row&allowed).bit_count()
    return total


def build_no_void_witnesses(pattern):
    witnesses=set()
    for decl in MODULE_DECLARATIONS:
        lists=[GROUPS[decl][s] for s in pattern]
        def rec(pos,used):
            if pos==len(lists): witnesses.add(used); return
            for g in lists[pos]:
                if not used&g: rec(pos+1,used|g)
        rec(0,0)
    return witnesses

def coverage_counts(pattern,target_sizes):
    ws=build_no_void_witnesses(pattern); rows=witness_superset_rows(ws); counts={}
    for n in target_sizes:
        v=0
        for low,row in enumerate(rows):
            hp=n-low.bit_count()
            if 0<=hp<=HALF: v += (row&POPCOUNT_MASKS[hp]).bit_count()
        counts[n]=v
    return len(ws),counts

def check_no_void_lower():
    expected={
      (4,2):(25584,{12:30402400}),
      (4,3):(36128,{12:30294577}),
      (4,4):(19394,{13:34115923,14:39546166}),
      (4,4,2):(944482,{15:37400509}),
      (4,4,3):(985332,{15:37241110,16:30419732}),
      (4,4,4):(381140,{17:21408593})}
    actual={}
    for pat,(ew,ec) in expected.items():
        wc,counts=coverage_counts(pat,tuple(ec)); assert wc==ew and counts==ec
        actual[pat]=(wc,counts)
    early=C(28,7)+3*C(28,8)+3*C(28,9)+C(28,10)+3*C(28,11)
    assert early==108_774_705
    total=early
    total += 2*actual[(4,2)][1][12]+actual[(4,3)][1][12]
    total += actual[(4,4)][1][13]+3*actual[(4,4)][1][14]
    total += 2*actual[(4,4,2)][1][15]+actual[(4,4,3)][1][15]
    total += actual[(4,4,3)][1][16]+3*actual[(4,4,4)][1][17]
    assert total==559_316_142
    return total


def extra_masks(follow,lead,kind):
    followers=tuple(bit_indices(follow)); leaders=set(bit_indices(lead)); outsiders=tuple(bit_indices(ALL_MASK^follow))
    out=[]
    if kind=="pair":
        for l in leaders:
            for x in outsiders: out.append((1<<l)|(1<<x))
    elif kind=="m1":
        for a,b in combinations(followers,2):
            if a not in leaders and b not in leaders: continue
            pair=(1<<a)|(1<<b)
            for x in outsiders: out.append(pair|(1<<x))
    elif kind=="m2":
        for l in leaders:
            for a,b in combinations(outsiders,2): out.append((1<<l)|(1<<a)|(1<<b))
    else: raise ValueError(kind)
    return tuple(out)

def contextual_witnesses(decl,module_count,follow,lead,kind):
    out=set(); extras=extra_masks(follow,lead,kind)
    for modules in module_unions(decl,module_count):
        for e in extras:
            if not modules&e: out.add(modules|e)
    return out

CALLED_VOID_GROUPS=(
(1,"pair",(("C12_hidden_pair",12,(1,2,3,4,5),23966810,3),)),
(1,"m1",(("C12_viewer_m1",12,(2,3,4,5),19940291,1),("C13_m1",13,(2,3,4,5),29359456,3),("C14_m1",14,(2,3,4,5),34711089,6))),
(1,"m2",(("C12_viewer_m2",12,(1,2,3,4,5),23966810,1),("C13_m2",13,(2,3,4,5),31177027,3),("C14_m2",14,(3,4,5),29851710,3))),
(2,"pair",(("C15_hidden_pair",15,(1,2,3,4,5),26574471,3),)),
(2,"m1",(("C15_viewer_m1",15,(2,3,4,5),20479830,1),("C16_m1",16,(2,3,4,5),21582309,3),("C17_m1",17,(2,3,4,5),17212461,6))),
(2,"m2",(("C15_viewer_m2",15,(2,3,4,5),25760616,1),("C16_m2",16,(3,4,5),22693062,3),("C17_m2",17,(4,5),13118238,3))),
(3,"pair",(("C18_hidden_pair",18,(1,2,3,4,5),8905344,3),)),
(3,"m1",(("C19_m1",19,(2,3,4,5),4114740,3),)),
(3,"m2",(("C19_m2",19,(4,5),4233495,3),)))

def check_called_void_lower():
    follow=CALLED_MASK[0]; lead=follow; subtotal=0
    for mc,kind,queries in CALLED_VOID_GROUPS:
        ws=contextual_witnesses(0,mc,follow,lead,kind); rows=witness_superset_rows(ws)
        for name,n,suit_counts,expected,mult in queries:
            count=count_from_rows(rows,n,follow,suit_counts)
            assert count==expected,(name,count,expected)
            subtotal += 8*mult*count
        del rows,ws; gc.collect()
    assert subtotal==8_387_350_664
    return subtotal

NATURAL_GROUPS=(
(1,"pair",(("N12_hidden_pair",12,(1,2,3,4),(5760594,9210738,11360129,12660550,13404689,13812747),3),)),
(1,"m1",(("N12_viewer_m1",12,(2,3,4),(5205424,8170230,9851930,10700937,11015340,11015340),1),("N13_m1",13,(2,3,4),(7491473,11679933,13902336,14925464,15273873,15273873),3))),
(1,"m2",(("N12_viewer_m2",12,(1,2,3,4),(5760594,9210738,11360129,12660550,13404689,13812747),1),("N13_m2",13,(1,2,3,4),(7908238,12476855,15067893,16446391,17134605,17458845),3))),
(2,"m1",(("N15_viewer_m1",15,(2,3,4),(5711043,8584043,10057926,10705653,10917124,10917124),1),("N16_m1",16,(2,3,4),(5238902,7878214,9083898,9529061,9652578,9652578),3))),
(2,"m2",(("N15_viewer_m2",15,(1,2,3,4),(6175761,9384409,11150343,12046854,12471751,12662139),1),("N16_m2",16,(2,3,4),(5318483,8010800,9254244,9723064,9860356,9869049),3))))

def check_natural_void_lower():
    hist=Counter()
    for t in PIPS:
        for q in PIPS:
            if q!=t: hist[LEAD_FIBER[(t,q)].bit_count()]+=1
    assert hist==Counter({i:7 for i in range(1,7)})
    subtotal=0
    for mc,kind,queries in NATURAL_GROUPS:
        sums=[0]*len(queries)
        for q in range(1,7):
            follow=EFFECTIVE[(0,q)]; lead=LEAD_FIBER[(0,q)]
            omitted=1<<INDEX[(q,0)]
            ws=contextual_witnesses(0,mc,follow,lead,kind); rows=witness_superset_rows(ws)
            for qi,query in enumerate(queries):
                name,n,suit_counts,expected_by_size,mult=query
                count=count_from_rows(rows,n,follow,suit_counts,omitted)
                assert count==expected_by_size[q-1],(name,q,count,expected_by_size[q-1])
                sums[qi]+=7*count
            del rows,ws; gc.collect()
        for query,s in zip(queries,sums): subtotal += query[4]*s
    assert subtotal==8_721_399_239
    return subtotal

# ------------------------------------------------------------------
# Outer certificate anchors
# ------------------------------------------------------------------

FOLLOWER_MAXIMUM={
    frozenset():frozenset(),
    frozenset({0}):frozenset({0}),
    frozenset({1}):frozenset(),
    frozenset({2}):frozenset(),
    frozenset({0,1}):frozenset({0,1}),
    frozenset({0,2}):frozenset({0}),
    frozenset({1,2}):frozenset({2}),
    frozenset({0,1,2}):frozenset({1,2}),
}

def hidden_capacity_profiles():
    return tuple(k for k in product(range(8),repeat=3) if max(k)-min(k)<=1)

def profile_parameters(k):
    n=sum(k)
    if k[0]==k[1]==k[2]: return n,7-k[0],0
    high=max(k); low=frozenset(i for i,x in enumerate(k) if x==high-1)
    return n,7-high,len(FOLLOWER_MAXIMUM[low])

def polynomial_multiply(a,b):
    out=[0]*(len(a)+len(b)-1)
    for i,x in enumerate(a):
        for j,y in enumerate(b): out[i+j]+=x*y
    return out

def lead_witness_counts():
    counts=[[0]*8 for _ in range(29)]
    for um in range(1<<7):
        u=um.bit_count(); poly=[1]
        for i,s in enumerate(range(1,8)):
            f=[C(s,k) for k in range(s+1)]
            if um&(1<<i): f[-1]-=1
            poly=polynomial_multiply(poly,f)
        for n,v in enumerate(poly): counts[n][u]+=v
    for n in range(29): assert counts[n][0]==C(28,n)
    return tuple(tuple(x) for x in counts)

def outer_count_for_profile(k,B):
    if k==(0,0,0): return 1
    n,j,f=profile_parameters(k)
    total=sum(7**u*B[n][u] for u in range(j+1))
    if f:
        u=j+1; total += (7**u-(8-2**f)**u)*B[n][u]
    return total

def check_outer_anchors():
    floor=C(28,21)+3*C(28,20)+3*C(28,19)+C(28,18)
    assert floor==44_352_165
    prof=hidden_capacity_profiles(); assert len(prof)==50
    B=lead_witness_counts()
    per=sum(outer_count_for_profile(k,B) for k in prof)
    tagged=9*per
    assert per==7_124_838_074_989
    assert tagged==64_123_542_674_901 and tagged<2**46
    return floor,per,tagged

# ------------------------------------------------------------------
# New two-context family
# ------------------------------------------------------------------

MEMBERSHIPS=tuple(
    frozenset(c)
    for r in (1,2)
    for c in combinations(range(3),r)
)
MEMBERSHIP_PAIRS=tuple(
    (a,b)
    for a in MEMBERSHIPS
    for b in MEMBERSHIPS
    if a!=b
)

def reduced_two_category_signature(k,MA,MB,xA,xB):
    certain=[0,0,0]; excl=[0,0,0]
    for M,x in ((MA,xA),(MB,xB)):
        if len(M)==2:
            holder=next(iter(set(range(3))-set(M)))
            certain[holder]+=x
        elif len(M)==1:
            excl[next(iter(M))]+=x
        else:
            return False
    r=[k[s]-certain[s] for s in range(3)]
    if min(r)<=0: return False
    nW=sum(k)-sum(certain)
    if sum(r)!=nW: return False
    return all(nW-excl[s]>=r[s]+1 for s in range(3))

def leadable_rank_positions(l):
    return frozenset(range(max(0,l-1)))|{5}

def win_subset_count(l,b,r):
    L=leadable_rank_positions(l); total=0
    for S0 in combinations(range(6),b):
        S=set(S0)
        if any(sum(x<v for x in S)>=r-1 for v in S&L):
            total+=1
    return total

def lead_subset_count(l,b):
    return C(6,b)-C(6-l,b)

def new_family_count_p_l(p,l):
    low=frozenset(range(p))
    k=tuple(4 if s in low else 5 for s in range(3))
    total=0
    for MA,MB in MEMBERSHIP_PAIRS:
        mA,mB=len(MA),len(MB)
        z=len(MA&low)
        Areq=(4-mA) if p==0 else (5+p-mA-z)
        Creq=mA+mB+(z if p else 0)
        rB=4-mB
        for a in range(7):
            xA=7-a
            if xA<2 or a<Areq: continue
            for b in range(7):
                xB=6-b
                if xB<2 or b<rB: continue
                c=13+p-a-b
                if not 0<=c<=15 or c<Creq: continue
                if not reduced_two_category_signature(k,MA,MB,xA,xB):
                    continue
                wb=(
                    lead_subset_count(l,b)
                    if p==0
                    else win_subset_count(l,b,rB)
                )
                total += C(6,a)*wb*C(15,c)
    return total

def check_new_family_count():
    assert len(MEMBERSHIPS)==6 and len(MEMBERSHIP_PAIRS)==30
    expected={
      0:(131682525,205320765,241368660,255680100,259508925,259508925),
      1:(80301650,80301650,91351260,109960565,131417000,149785350),
      2:(59080450,59080450,66703780,79638845,94229135,105473225),
      3:(37323000,37323000,41801760,49607415,58371885,64510875),
    }
    table={
        p:tuple(new_family_count_p_l(p,l) for l in range(1,7))
        for p in range(4)
    }
    assert table==expected
    phase={p:7*sum(row) for p,row in table.items()}
    assert phase=={
        0:9_471_489_300,
        1:4_501_822_325,
        2:3_249_441_195,
        3:2_022_565_545,
    }
    total=sum(phase.values())
    assert total==19_245_318_365
    return table,phase,total


def actual_B_subset_count(t,q,b,r,need_win):
    B=EFFECTIVE[(t,q)]
    qkey=lambda i:trick_key(t,q,i)
    members=sorted(bit_indices(B),key=qkey)
    lead=set(bit_indices(LEAD_FIBER[(t,q)]))
    total=0
    for ch in combinations(members,b):
        S=set(ch)
        if not need_win:
            ok=bool(S&lead)
        else:
            ok=False
            for v in S&lead:
                if sum(qkey(x)<qkey(v) for x in S)>=r-1:
                    ok=True
                    break
        total+=ok
    return total

def check_rank_subset_tables():
    hist=Counter()
    checks=0
    for t in PIPS:
        for q in PIPS:
            if q==t: continue
            l=LEAD_FIBER[(t,q)].bit_count()
            hist[l]+=1
            for b in range(7):
                assert (
                    actual_B_subset_count(t,q,b,0,False)
                    == lead_subset_count(l,b)
                )
                for r in (2,3):
                    assert (
                        actual_B_subset_count(t,q,b,r,True)
                        == win_subset_count(l,b,r)
                    )
                    checks+=1
    assert hist==Counter({i:7 for i in range(1,7)})
    return checks


def abstract_category_reduction_check():
    checked=0
    for p in range(4):
        low=frozenset(range(p))
        k=tuple(4 if s in low else 5 for s in range(3))
        for MA,MB in MEMBERSHIP_PAIRS:
            mA,mB=len(MA),len(MB)
            z=len(MA&low)
            Areq=(4-mA) if p==0 else 5+p-mA-z
            Creq=mA+mB+(z if p else 0)
            rB=4-mB
            for a in range(7):
                xA=7-a
                if xA<2 or a<Areq: continue
                for b in range(7):
                    xB=6-b
                    if xB<2 or b<rB: continue
                    c=13+p-a-b
                    if not 0<=c<=15 or c<Creq: continue
                    if not reduced_two_category_signature(k,MA,MB,xA,xB):
                        continue
                    n=sum(k)
                    U=frozenset(range(n))
                    XA=frozenset(range(xA))
                    XB=frozenset(range(xA,xA+xB))
                    P=tuple(
                        frozenset(
                            d for d in U
                            if not (d in XA and s in MA)
                            and not (d in XB and s in MB)
                        )
                        for s in range(3)
                    )
                    marg=marginal_sets(U,P,k)
                    assert marg==P
                    checked+=1
    return checked


def choose_B_roles(t,q,TB,rB,need_win):
    lead=set(bit_indices(LEAD_FIBER[(t,q)]))
    qkey=lambda i:trick_key(t,q,i)
    for v in sorted(set(TB)&lead,key=qkey,reverse=True):
        others=[
            x for x in TB
            if x!=v and (not need_win or qkey(x)<qkey(v))
        ]
        if len(others)>=rB-1:
            return v,others[:rB-1]
    raise AssertionError("no B role assignment")

def construct_and_replay_case(p,l,MA0,MB0,a,b):
    t=0
    q=l
    MA=frozenset(s+1 for s in MA0)
    MB=frozenset(s+1 for s in MB0)
    A=CALLED_MASK[t]
    B=EFFECTIVE[(t,q)]
    assert not A&B

    e=INDEX[(q,0)]
    Aavail=[i for i in bit_indices(A) if i!=e]
    Cmask=ALL_MASK^A^B
    Cmembers=list(bit_indices(Cmask))

    low0=frozenset(range(p))
    k=tuple(4 if s in low0 else 5 for s in range(3))
    mA,mB=len(MA0),len(MB0)
    z=len(MA0&low0)
    c=13+p-a-b
    rB=4-mB

    TB=None
    for ch in combinations(tuple(bit_indices(B)),b):
        try:
            choose_B_roles(t,q,set(ch),rB,p>0)
            TB=set(ch)
            break
        except AssertionError:
            pass
    assert TB is not None

    TA=set(Aavail[:a])
    TC=set(Cmembers[:c])
    T=set(TA)|TB|TC
    assert len(T)==13+p and e not in T
    U=set(range(28))-T

    xA=len(U&set(bit_indices(A)))
    xB=len(U&set(bit_indices(B)))
    assert reduced_two_category_signature(k,MA0,MB0,xA,xB)

    first_A_count=4-mA
    third_A_count=0 if p==0 else 1+p-z
    assert len(TA)>=first_A_count+third_A_count

    orderedA=sorted(TA,key=lambda i:trick_key(t,7,i))
    v1=orderedA[-1]
    first_follow=orderedA[:first_A_count-1]
    usedA={v1,*first_follow}
    remainingA=[i for i in orderedA if i not in usedA]
    thirdA=remainingA[:third_A_count]

    v2,bfollowers=choose_B_roles(t,q,TB,rB,p>0)
    Citer=iter(sorted(TC))
    plays=[]

    current=[(0,v1)]
    plays.append((0,v1))
    af=iter(first_follow)
    hidden_public={1:[],2:[],3:[]}
    viewer_public=[v1]
    for s in HIDDEN:
        d=next(Citer) if s in MA else next(af)
        current.append((s,d))
        plays.append((s,d))
        hidden_public[s].append(d)
    assert trick_winner(t,current)==0

    current=[(0,v2)]
    plays.append((0,v2))
    viewer_public.append(v2)
    bf=iter(bfollowers)
    for s in HIDDEN:
        d=next(Citer) if s in MB else next(bf)
        current.append((s,d))
        plays.append((s,d))
        hidden_public[s].append(d)
    if p>0:
        assert trick_winner(t,current)==0

    if p>0:
        ta=iter(thirdA)
        v3=next(ta)
        plays.append((0,v3))
        viewer_public.append(v3)
        for s in range(1,p+1):
            d=next(Citer) if s in MA else next(ta)
            plays.append((s,d))
            hidden_public[s].append(d)

    used_public=set(viewer_public)
    for s in HIDDEN:
        used_public.update(hidden_public[s])
    viewer_current=T-used_public
    assert len(viewer_public)+len(viewer_current)==7

    Aset=set(bit_indices(A))
    Bset=set(bit_indices(B))
    P=tuple(
        frozenset(
            i for i in U
            if not (i in Aset and s+1 in MA)
            and not (i in Bset and s+1 in MB)
        )
        for s in range(3)
    )

    world=find_world(frozenset(U),P,k)
    hands={0:set(viewer_current)|set(viewer_public)}
    for s in HIDDEN:
        hands[s]=set(world[s-1])|set(hidden_public[s])

    assert all(len(hands[s])==7 for s in range(4))
    assert set().union(*hands.values())==set(range(28))
    assert sum(len(h) for h in hands.values())==28

    leader=0
    current=[]
    voids={1:set(),2:set(),3:set()}
    played_by={s:set() for s in range(4)}

    for actor,d in plays:
        expected=(leader+len(current))%4
        assert actor==expected

        legal=legal_indices(
            t,
            tile_mask(DOMINOES[i] for i in hands[actor]),
            current,
        )
        assert legal&(1<<d)

        if current and actor in HIDDEN:
            context=LED_CONTEXT[(t,current[0][1])]
            if not EFFECTIVE[(t,context)]&(1<<d):
                voids[actor].add(context)

        hands[actor].remove(d)
        played_by[actor].add(d)
        current.append((actor,d))

        if len(current)==4:
            leader=trick_winner(t,current)
            current=[]

    expected_voids={
        s:({7} if s in MA else set())|({q} if s in MB else set())
        for s in HIDDEN
    }
    assert voids==expected_voids

    U2=set(range(28))-hands[0]-set().union(*played_by.values())
    assert U2==U

    k2=tuple(7-len(played_by[s]) for s in HIDDEN)
    assert k2==k

    P2=[]
    for s in HIDDEN:
        forbidden=0
        for qq in voids[s]:
            forbidden |= EFFECTIVE[(t,qq)]
        P2.append(frozenset(U2-set(bit_indices(forbidden))))
    P2=tuple(P2)

    assert P2==P
    assert marginal_sets(frozenset(U2),P2,k2)==P2
    return True


def check_new_family_witness_templates():
    checked=0
    for p in range(4):
        low=frozenset(range(p))
        k=tuple(4 if s in low else 5 for s in range(3))
        for l in range(1,7):
            for MA,MB in MEMBERSHIP_PAIRS:
                mA,mB=len(MA),len(MB)
                z=len(MA&low)
                Areq=(4-mA) if p==0 else 5+p-mA-z
                Creq=mA+mB+(z if p else 0)
                rB=4-mB
                for a in range(7):
                    xA=7-a
                    if xA<2 or a<Areq: continue
                    for b in range(7):
                        xB=6-b
                        if xB<2 or b<rB: continue
                        c=13+p-a-b
                        if not 0<=c<=15 or c<Creq: continue
                        if not reduced_two_category_signature(
                            k,MA,MB,xA,xB
                        ):
                            continue
                        w=(
                            lead_subset_count(l,b)
                            if p==0
                            else win_subset_count(l,b,rB)
                        )
                        if not w: continue
                        assert construct_and_replay_case(
                            p,l,MA,MB,a,b
                        )
                        checked+=1
    return checked

# ------------------------------------------------------------------
# Restricted brute-force support crosscheck
# ------------------------------------------------------------------

def check_90_world_support_crosscheck():
    decl="NT"
    viewer_initial=frozenset({
        (6,3),(5,0),(4,0),(2,1),(5,1),(3,1),(4,1)
    })
    history=[
        [(0,(6,3)),(1,(6,1)),(2,(6,4)),(3,(6,0))],
        [(2,(0,0)),(3,(2,2)),(0,(5,0)),(1,(2,0))],
        [(2,(4,3)),(3,(4,2)),(0,(4,0)),(1,(5,4))],
        [(1,(1,1)),(2,(3,0)),(3,(3,3)),(0,(2,1))],
        [(1,(1,0)),(2,(6,6)),(3,(5,2)),(0,(5,1))],
    ]
    unseen=frozenset({
        (5,5),(4,4),(3,2),(6,5),(5,3),(6,2)
    })
    played={s:set() for s in range(4)}
    for tr in history:
        for s,d in tr:
            played[s].add(d)

    worlds=set()
    ordered=sorted(unseen)
    for h1 in combinations(ordered,2):
        r1=[d for d in ordered if d not in h1]
        for h2 in combinations(r1,2):
            h3=tuple(d for d in r1 if d not in h2)
            worlds.add((
                frozenset(h1),
                frozenset(h2),
                frozenset(h3),
            ))
    assert len(worlds)==90

    endpoint=set()
    global_voids={1:set(),2:set(),3:set()}

    for world in worlds:
        hands={
            0:set(viewer_initial),
            1:set(world[0])|played[1],
            2:set(world[1])|played[2],
            3:set(world[2])|played[3],
        }
        leader=0
        local={1:set(),2:set(),3:set()}

        for tr in history:
            assert tr[0][0]==leader
            cur=[]
            for off,(actor,d) in enumerate(tr):
                assert actor==(leader+off)%4
                i=INDEX[d]
                assert legal_indices(
                    decl,tile_mask(hands[actor]),cur
                )&(1<<i)

                if cur and actor in HIDDEN:
                    q=LED_CONTEXT[(decl,cur[0][1])]
                    if not EFFECTIVE[(decl,q)]&(1<<i):
                        local[actor].add(q)

                hands[actor].remove(d)
                cur.append((actor,i))

            leader=trick_winner(decl,cur)

        endpoint.add((
            frozenset(hands[1]),
            frozenset(hands[2]),
            frozenset(hands[3]),
        ))
        for s in HIDDEN:
            global_voids[s].update(local[s])

    assert endpoint==worlds
    assert global_voids=={1:set(),2:{1},3:{0,1}}

    P=[]
    for s in HIDDEN:
        forbidden=0
        for q in global_voids[s]:
            forbidden |= EFFECTIVE[(decl,q)]
        P.append(frozenset(
            d for d in unseen
            if not forbidden&(1<<INDEX[d])
        ))
    P=tuple(P)

    assert all(p==unseen for p in P)
    assert enumerate_worlds(
        unseen,P,(2,2,2)
    )==frozenset(worlds)
    assert fiber_count_dp(unseen,P,(2,2,2))==90
    return 90


def run_check(name,fn):
    try:
        value=fn()
    except Exception as exc:
        print(f"FAIL {name} {type(exc).__name__}: {exc}")
        return False,None
    print(f"PASS {name}"+(
        f" {value}" if value is not None else ""
    ))
    return True,value

def main():
    started=time.time()
    ok=True
    values={}
    checks=(
      ("rules",check_rules),
      ("one_context_cell_lemma",check_one_context_cell_lemma),
      ("new_rank_subset_tables",check_rank_subset_tables),
      ("new_abstract_reduction",abstract_category_reduction_check),
      ("new_witness_templates",check_new_family_witness_templates),
      ("new_family_count",check_new_family_count),
      ("support_crosscheck_90world",check_90_world_support_crosscheck),
      ("prior_no_void",check_no_void_lower),
      ("prior_called_void",check_called_void_lower),
      ("prior_natural_void",check_natural_void_lower),
      ("outer_anchors",check_outer_anchors),
    )

    for name,fn in checks:
        passed,val=run_check(name,fn)
        ok &= passed
        values[name]=val

    if not ok:
        return 1

    try:
        no_void=values["prior_no_void"]
        called=values["prior_called_void"]
        natural=values["prior_natural_void"]
        old=no_void+called+natural
        assert (
            no_void,called,natural,old
        )==(
            559_316_142,
            8_387_350_664,
            8_721_399_239,
            17_668_066_045,
        )

        table,phase,new=values["new_family_count"]
        lower=old+new
        assert new==19_245_318_365
        assert lower==36_913_384_410
        assert 2**35 < lower < 2**36

        floor,per,tagged=values["outer_anchors"]
        assert floor==44_352_165
        assert per==7_124_838_074_989
        assert tagged==64_123_542_674_901
    except Exception as exc:
        print(f"FAIL headline {type(exc).__name__}: {exc}")
        return 1

    print(f"PASS certified_prior_floor {old}")
    print(f"PASS certified_new_disjoint_family {new}")
    print(f"PASS combined_lower {lower}")
    print("PASS headline INTERVAL [36,46] bits")
    print(f"PASS runtime_seconds {time.time()-started:.3f}")
    return 0

if __name__=="__main__":
    sys.exit(main())
