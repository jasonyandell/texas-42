#!/usr/bin/env python3
"""Lawful sampled best response: fixed worlds, public-history coordinates.

Research prototype only. A tape is a private nature scenario, not information
available to the focal policy. A shared MAX follows aggregation over that
history's surviving scenarios. Universal coordinates retain every legal other-
seat edge; tape-specific coordinates retain only realized other-seat edges.
Both preserve full public history and play to completion for payoff reuse.
"""
from dataclasses import dataclass
from pathlib import Path
import random, sys, time
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT.parent/'partnership'))
from rules import legal_tiles, winner, trick_points
popcount=getattr(int,'bit_count',lambda x:bin(x).count('1'))

def bits(mask):
    while mask:
        one=mask & -mask;yield one.bit_length()-1;mask-=one

@dataclass(frozen=True)
class State:
    played:int
    leader:int
    trick:tuple
    points:tuple

@dataclass
class Node:
    key:int
    depth:int
    support:int
    actor:int
    focal:bool
    children:dict
    legal:dict
    score:object

class Kernel:
    def __init__(self,request,worlds,points,leader,trick=()):
        self.request=request;self.worlds=worlds;self.n=len(worlds)
        self.focal=request['seat'];self.bidder=request['bidder'];self.decl=request['decl']
        self.root=State(sum(1<<t for t in request['plays'][1::2]),leader,tuple(trick),tuple(points))
        self.plies=28-popcount(self.root.played)
        assert self.n and 0<self.plies<=16
        known=sum(1<<t for t in request['hand']) & ~self.root.played
        for world in worlds:
            assert world[self.focal]==known and not (sum(world)&self.root.played)
            assert sum(world)==((1<<28)-1)^self.root.played
            union=0
            for hand in world:
                assert hand&union==0;union|=hand

    def moves(self,state,w):
        seat=(state.leader+len(state.trick))%4
        return legal_tiles(tuple(bits(self.worlds[w][seat]&~state.played)),state.trick,self.decl)

    def after(self,state,tile):
        seat=(state.leader+len(state.trick))%4;trick=state.trick+((seat,tile),)
        leader=state.leader;points=list(state.points)
        if len(trick)==4:
            leader=winner(trick,self.decl);points[leader%2]+=trick_points(trick);trick=()
        return State(state.played|(1<<tile),leader,trick,tuple(points))

    def success(self,score,bid):
        made=score[self.bidder%2]>=bid
        return made if self.focal%2==self.bidder%2 else not made

    def compile(self,tape=None,*,node_cap=100000,seconds=20):
        nodes=[];deadline=time.monotonic()+seconds
        def rec(state,support,key,depth):
            if len(nodes)>=node_cap or time.monotonic()>deadline:raise TimeoutError('structural compilation cap')
            i=len(nodes);actor=(state.leader+len(state.trick))%4
            node=Node(key,depth,support,actor,actor==self.focal,{}, {},None);nodes.append(node)
            if popcount(state.played)==28:
                assert sum(state.points)==42 and not state.trick
                node.score=state.points;return i
            groups={}
            for w in bits(support):
                legal=self.moves(state,w);node.legal[w]=tuple(legal)
                choices=legal if node.focal or tape is None else [pick(tape,w,depth,legal)]
                for tile in choices:groups[tile]=groups.get(tile,0)|(1<<w)
            if node.focal:assert len(set(node.legal.values()))==1
            for tile,mask in sorted(groups.items()):
                node.children[tile]=rec(self.after(state,tile),mask,key*29+tile+1,depth+1)
            return i
        rec(self.root,(1<<self.n)-1,0,0)
        assert len({n.key for n in nodes})==len(nodes)
        return Compiled(self,nodes,tape)

    def reference(self,tape,bid,early_stop=True):
        """Independent recursive control: split worlds first, share own MAX."""
        calls=0
        def rec(state,worlds,depth):
            nonlocal calls;calls+=1
            if early_stop and (state.points[self.bidder%2]>=bid or state.points[1-self.bidder%2]>42-bid):
                return int(self.success(state.points,bid))*len(worlds)
            if popcount(state.played)==28:return int(self.success(state.points,bid))*len(worlds)
            actor=(state.leader+len(state.trick))%4
            if actor==self.focal:
                legal=self.moves(state,worlds[0])
                assert all(self.moves(state,w)==legal for w in worlds)
                return max(rec(self.after(state,t),worlds,depth+1) for t in legal)
            groups={}
            for w in worlds:
                legal=self.moves(state,w);tile=pick(tape,w,depth,legal);groups.setdefault(tile,[]).append(w)
            return sum(rec(self.after(state,t),ws,depth+1) for t,ws in groups.items())
        assert (self.root.leader+len(self.root.trick))%4==self.focal
        values={t:rec(self.after(self.root,t),list(range(self.n)),1) for t in self.moves(self.root,0)}
        return values,calls

def pick(tape,w,depth,legal):
    # Exact function of the saved finite tape. No claim of exact integration
    # over continuous random numbers, posterior fibers, or all future draws.
    return legal[(tape[w][depth]*len(legal))>>64]

def make_tape(seed,worlds,plies):
    rng=random.Random(seed)
    return tuple(tuple(rng.getrandbits(64) for _ in range(plies)) for _ in range(worlds))

class Compiled:
    def __init__(self,kernel,nodes,tape):self.kernel=kernel;self.nodes=nodes;self.tape=tape

    def route(self,tape):
        """Forward lookup pass; returns only reached coordinates and masks."""
        if self.tape is not None:
            assert tape==self.tape,'Tape-specific coordinates must not be reused for a different tape'
            return [(i,n.support) for i,n in enumerate(self.nodes)]
        reached=[];todo=[(0,(1<<self.kernel.n)-1)]
        while todo:
            i,mask=todo.pop();node=self.nodes[i];reached.append((i,mask))
            assert mask and mask&~node.support==0
            if node.score is not None:continue
            if node.focal:
                for j in node.children.values():todo.append((j,mask))
            else:
                groups={}
                for w in bits(mask):
                    t=pick(tape,w,node.depth,node.legal[w]);groups[t]=groups.get(t,0)|(1<<w)
                for t,m in groups.items():todo.append((node.children[t],m))
        return reached

    def reduce(self,reached,bid):
        """Reverse lookup pass: aggregate scenarios before one shared MAX."""
        values={}
        for i,mask in reversed(reached):
            n=self.nodes[i]
            if n.score is not None:v=int(self.kernel.success(n.score,bid))*popcount(mask)
            elif n.focal:v=max(values[j] for j in n.children.values())
            else:v=sum(values.get(j,0) for j in n.children.values())
            values[i]=v
        return {tile:values[j] for tile,j in self.nodes[0].children.items()}

    def statistics(self):
        # Object-size census, not process RSS or a compact native format.
        seen=set()
        def size(x):
            if id(x) in seen:return 0
            seen.add(id(x));n=sys.getsizeof(x)
            if isinstance(x,dict):return n+sum(size(k)+size(v) for k,v in x.items())
            if isinstance(x,(list,tuple)):return n+sum(map(size,x))
            if isinstance(x,Node):return n+size(vars(x))
            return n
        return dict(nodes=len(self.nodes),edges=sum(len(n.children) for n in self.nodes),
            leaves=sum(n.score is not None for n in self.nodes),
            support_incidences=sum(popcount(n.support) for n in self.nodes),
            python_object_bytes=size(self.nodes),max_depth=max(n.depth for n in self.nodes))
