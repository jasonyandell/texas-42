"""C++ policy ABI accepts only an acting hand and public state.

The physical deal lives in the game driver. The compiled policy is never given
that array, or an enclosing search's conditioned deal pool. Native policies use
SplitMix64 and DFS. The addressed variant makes branch randomness independent
of traversal so exact incumbent bounds can skip work at delta=1.
"""
import ctypes as ct
import json
from pathlib import Path
import time
import numpy as np
import repaired as r

ROOT=Path(__file__).resolve().parent
U64=np.ctypeslib.ndpointer(np.uint64,flags='C_CONTIGUOUS')
U32=np.ctypeslib.ndpointer(np.uint32,flags='C_CONTIGUOUS')
F64=np.ctypeslib.ndpointer(np.float64,flags='C_CONTIGUOUS')
I32=np.ctypeslib.ndpointer(np.int32,flags='C_CONTIGUOUS')
LIB=ct.CDLL(str(ROOT/'turbo.dylib'))
LIB.walt_create.argtypes=[ct.c_int,I32,ct.c_int,ct.c_double,ct.c_uint64,ct.c_int,ct.c_int,ct.c_uint64,ct.c_uint64,ct.c_double]
LIB.walt_create.restype=ct.c_void_p
LIB.walt_destroy.argtypes=[ct.c_void_p]
LIB.walt_action.argtypes=[ct.c_void_p,ct.c_int,U64,ct.c_uint32,ct.c_int,F64]
LIB.walt_action.restype=ct.c_int
LIB.walt_error.argtypes=[ct.c_void_p];LIB.walt_error.restype=ct.c_char_p
LIB.walt_stats.argtypes=[ct.c_void_p,U64]
LIB.walt_sample.argtypes=[ct.c_void_p,ct.c_int,U64,ct.c_uint32,U32]
LIB.walt_sample.restype=ct.c_int
LIB.walt_rank.argtypes=[U64,ct.c_uint32,ct.c_uint64,U32,U64]
LIB.walt_rank.restype=ct.c_int
LIB.walt_play.argtypes=[ct.c_int,U64,ct.c_int,U64]
LIB.walt_play.restype=ct.c_int
LIB.walt_evaluate_l1.argtypes=[ct.c_void_p,U64,U32,ct.c_int,ct.c_uint64,F64]
LIB.walt_evaluate_l1.restype=ct.c_int
LIB.walt_configure.argtypes=[ct.c_void_p,ct.c_int,ct.c_int,ct.c_int,ct.c_int]
LIB.walt_configure.restype=ct.c_int
LIB.walt_metrics.argtypes=[ct.c_void_p,U64]


def pack(pub,i=0):
    return np.array([pub.played[i],pub.leader[i],pub.tlen[i],pub.t1[i],pub.t0[i],pub.depth,
                     *pub.trick[i],*pub.voids[i],*pub.history[i]],np.uint64)


class Field:
    def __init__(self,rules,spec=r.Spec(),seconds=60,cache=True,cache_limit=50_000,fiber_cap=1_000_000,reuse_deals=False,addressed=False,prune=True,tables=True,node_cache=True):
        self.handle=None
        if not 0<=spec.seed<2**64: raise ValueError('native seed must fit uint64')
        if not np.isfinite(seconds) or seconds<0: raise ValueError('invalid deadline')
        self._rules=r.Rules(rules.trump);self._spec=spec;self._reuse_deals=bool(reuse_deals)
        self._addressed=bool(addressed)
        for name in ('suit','lead','strength'): getattr(self._rules,name).setflags(write=False)
        self.handle=LIB.walt_create(rules.trump,np.array(spec.samples,np.int32),len(spec.samples),
            spec.delta,spec.seed,reuse_deals,cache,cache_limit,fiber_cap,seconds)
        if not self.handle: raise ValueError('invalid native field configuration')
        assert LIB.walt_configure(self.handle,addressed,prune,tables,node_cache)==0
    @property
    def rules(self): return self._rules
    @property
    def spec(self): return self._spec
    @property
    def reuse_deals(self): return self._reuse_deals
    @property
    def addressed(self): return self._addressed
    def close(self):
        if self.handle: LIB.walt_destroy(self.handle);self.handle=None
    def __del__(self): self.close()
    def actions(self,level,pub,hands,return_values=False):
        if not self.handle: raise ValueError('native field is closed')
        actions=[];values=[]
        for i,hand in enumerate(hands):
            scores=np.empty(28)
            tile=LIB.walt_action(self.handle,level,pack(pub,i),int(hand),return_values,scores)
            if tile<0: raise r.Limit(LIB.walt_error(self.handle).decode())
            actions.append(tile);values.append({int(t):float(scores[t]) for t in np.flatnonzero(~np.isnan(scores))})
        out=np.array(actions,np.int64)
        return (out,values) if return_values else out
    def sample(self,level,pub,hand):
        if not self.handle: raise ValueError('native field is closed')
        if not 1<=level<=len(self.spec.samples): raise ValueError('invalid sample level')
        if len(pub.played)!=1: raise ValueError('sample takes one information state')
        out=np.empty((self.spec.samples[level-1],4),np.uint32)
        if LIB.walt_sample(self.handle,level,pack(pub),int(hand),out)<0:
            raise ValueError(LIB.walt_error(self.handle).decode())
        return out
    @property
    def stats(self):
        out=np.empty((len(self.spec.samples),8),np.uint64);LIB.walt_stats(self.handle,out)
        names=('queries','cache_hits','forced','nodes','fiber_visits','peak_node_fibers','sampled_deals','sample_cache_hits')
        return {f'L{i+1}_{name}':int(v) for i,row in enumerate(out) for name,v in zip(names,row)}
    @property
    def metrics(self):
        out=np.empty(6,np.uint64);LIB.walt_metrics(self.handle,out)
        return dict(zip(('pruned_nodes','pruned_actions','pattern_hits','pattern_misses','pattern_entries','deduplicated_policy_queries'),map(int,out)))


def game(seed=1,spec=r.Spec(),level=2,batch=256,seconds=60,cache=True,progress=None,fiber_cap=1_000_000,reuse_deals=False,addressed=False,prune=True,tables=True,node_cache=True):
    started=time.perf_counter();rules,hands=r.deal(seed)
    field=Field(rules,spec,seconds=seconds,cache=cache,fiber_cap=fiber_cap,reuse_deals=reuse_deals,addressed=addressed,prune=prune,tables=tables,node_cache=node_cache)
    pub=r.Pub.initial();moves=[];status='complete';reason=None
    try:
        while not any(bool(v[0]) for v in pub.outcome()):
            seat=int(pub.turn()[0]);hand=hands[seat]&~int(pub.played[0]);t0=time.perf_counter()
            if rules.legal(np.array([hand]),pub.trick,pub.tlen).sum()>1:
                action,scored=field.actions(level,pub,np.array([hand]),return_values=True);values=scored[0]
            else: action=field.actions(level,pub,np.array([hand]));values=None
            tile=int(action[0]);pub=pub.play(rules,action)
            row={'seat':seat,'tile':tile,'seconds':time.perf_counter()-t0,'t1':int(pub.t1[0]),'t0':int(pub.t0[0]),'values':values}
            moves.append(row)
            if progress: progress(row)
    except r.Limit as e: status='incomplete';reason=str(e)
    result={'seed':seed,'spec':{'samples':spec.samples,'delta':spec.delta,'seed':spec.seed},
        'policy_version':'walt42x-native-addressed-v2' if addressed else 'walt42x-native-dfs-v1','reuse_deals':reuse_deals,'cache_limit':50000,'fiber_cap':fiber_cap,
        'level':level,'batch':batch,'cache':cache,'engine':'cpp','status':status,'reason':reason,
        'seconds':time.perf_counter()-started,'trump':rules.trump,'hands':hands,'moves':moves,'stats':field.stats,'metrics':field.metrics,'prune':prune,'tables':tables,'node_cache':node_cache,
        'outcome':('made' if pub.t1[0]>=30 else 'set') if status=='complete' else None}
    field.close();return result
