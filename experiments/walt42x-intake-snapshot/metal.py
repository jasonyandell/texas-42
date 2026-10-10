"""Literal frontier translation to torch/MPS; random-policy rung only.

No CPU operator fallback. Host RNG and dynamic-shape/control synchronizations
remain intentional. Float32 weights (MPS), int64 masks/indices.
"""
import time
import numpy as np
import torch
import walt42x as ref

class FrontierLimit(RuntimeError):
    pass

def group_edges(keys, parent_count, device):
    if device != 'mps':
        return torch.unique(keys, sorted=True, return_inverse=True)
    # Observed torch 2.11 MPS sort/unique rounds integer VALUES above 2**24.
    # Preserve exact history identity with a dense parent x tile occupancy scan.
    # Prefix counts stay below the explicit frontier cap (well below 2**24).
    if len(keys) >= 2**24:
        raise ValueError('MPS prefix-count guard: split this frontier into batches')
    counts = torch.zeros(parent_count * 28, dtype=torch.int32, device=device)
    counts.scatter_add_(0, keys, torch.ones_like(keys, dtype=torch.int32))
    occupied = counts != 0
    # nonzero also rounds large coordinates on this MPS runtime. Keep each
    # coordinate small, then pack with integer arithmetic after compaction.
    parent, tile = torch.nonzero(occupied.reshape(parent_count, 28), as_tuple=True)
    unique = parent * 28 + tile
    prefix = occupied.cumsum(0, dtype=torch.int32)
    return unique, (prefix[keys] - 1).long()

class Backend:
    def __init__(self, device):
        self.device = device
        self.bits = self.array(np.arange(28))
        self.points = self.array(ref.POINTS)

    def array(self, value, dtype=torch.int64):
        return torch.as_tensor(value, dtype=dtype, device=self.device)

    def zeros(self, shape, dtype=torch.float32):
        return torch.zeros(shape, dtype=dtype, device=self.device)

    def sync(self):
        if self.device == 'mps':
            torch.mps.synchronize()

class Rules:
    def __init__(self, backend, trump):
        self.b = backend
        source = ref.Rules(trump)
        self.suit = backend.array(source.suit)
        self.lead = backend.array(source.lead)
        self.strength = backend.array(source.strength)

    def legal(self, hand, trick, tlen):
        follow = hand & self.suit[self.lead[trick[:, 0]]]
        mask = torch.where(tlen == 0, hand, torch.where(follow != 0, follow, hand))
        return ((mask[:, None] >> self.b.bits) & 1).bool()

class Pub:
    def __init__(self, backend, played, leader, trick, tlen, t1, t0):
        self.b = backend
        self.played, self.leader, self.trick = played, leader, trick
        self.tlen, self.t1, self.t0 = tlen, t1, t0

    @classmethod
    def from_ref(cls, backend, pub):
        return cls(backend, *(backend.array(getattr(pub, k)) for k in
                            ('played', 'leader', 'trick', 'tlen', 't1', 't0')))

    def take(self, i):
        return Pub(self.b, *(getattr(self, k)[i] for k in
                            ('played', 'leader', 'trick', 'tlen', 't1', 't0')))

    def turn(self):
        return (self.leader + self.tlen) % 4

    def outcome(self):
        return self.t1 >= ref.BID, self.t0 > 42 - ref.BID

    def play(self, rules, t):
        trick = self.trick.clone()
        trick[torch.arange(len(t), device=self.b.device), self.tlen] = t
        tlen, played = self.tlen + 1, self.played | (1 << t)
        full = tlen == 4
        pos = rules.strength[rules.lead[trick[:, 0]][:, None], trick].argmax(1)
        winner = (self.leader + pos) % 4
        won = 1 + self.b.points[trick].sum(1)
        t1 = self.t1 + torch.where(full & (winner % 2 == 1), won, 0)
        t0 = self.t0 + torch.where(full & (winner % 2 == 0), won, 0)
        return Pub(self.b, played, torch.where(full, winner, self.leader),
                   torch.where(full[:, None], 0, trick), torch.where(full, 0, tlen), t1, t0)

def search(rules, me, pub, deals, rng, delta=0.0, cap=500_000, seconds=60):
    b = rules.b
    start = time.perf_counter()
    N = len(deals)
    node = pub
    tri_node = b.zeros(N, torch.int64)
    tri_deal = torch.arange(N, device=b.device)
    tri_w = torch.ones(N, device=b.device)
    plies, census = [], []
    parent = tile = None
    while True:
        census.append({'ply': len(plies), 'nodes': len(node.played), 'fibers': len(tri_w)})
        if len(tri_w) > cap or time.perf_counter() - start > seconds:
            raise FrontierLimit(census)
        made, sett = node.outcome()
        term = made | sett
        leaf = b.zeros(len(term))
        m = made[tri_node]
        leaf.scatter_add_(0, tri_node[m], tri_w[m])
        plies.append((parent, tile, term, node.turn() == me, leaf))
        if bool(term.all()):
            break
        keep = ~term[tri_node]
        tn, td, tw = tri_node[keep], tri_deal[keep], tri_w[keep]
        tpub = node.take(tn)
        tseat = tpub.turn()
        thand = deals[td, tseat] & ~tpub.played
        mine = tseat == me
        P = rules.legal(thand, tpub.trick, tpub.tlen).float()
        oth = ~mine
        if bool(oth.any()):
            Po = P[oth]
            Po = Po / Po.sum(1, keepdim=True)
            small = tw[oth] * torch.where(Po > 0, Po, float('inf')).amin(1) < delta
            if bool(small.any()):
                Ps = Po[small]
                c = Ps.cumsum(1)
                r = b.array(rng.random(len(Ps)), torch.float32)[:, None] * c[:, -1:]
                idx = (c <= r).sum(1)
                Q = torch.zeros_like(Ps)
                Q[torch.arange(len(Ps), device=b.device), idx] = 1
                Po[small] = Q
            P[oth] = Po
        ti, t = torch.nonzero(P, as_tuple=True)
        if len(ti) > cap:
            raise FrontierLimit(census + [{'next_fibers': len(ti)}])
        cw = tw[ti] * torch.where(mine[ti], 1.0, P[ti, t])
        ukey, inv = group_edges(tn[ti] * 28 + t, len(node.played), b.device)
        parent, tile = ukey // 28, ukey % 28
        node = node.take(parent).play(rules, tile)
        tri_node, tri_deal, tri_w = inv, td[ti], cw
    V = plies[-1][4]
    for d in range(len(plies) - 1, 1, -1):
        parent, _, _, _, _ = plies[d]
        _, _, pterm, pme, pleaf = plies[d - 1]
        n = len(pterm)
        s = b.zeros(n)
        s.scatter_add_(0, parent, V)
        x = torch.full((n,), -float('inf') if me % 2 else float('inf'), device=b.device)
        x.scatter_reduce_(0, parent, V, reduce='amax' if me % 2 else 'amin')
        V = torch.where(pterm, pleaf, torch.where(pme, x, s))
    # cpu() is an explicit synchronized result transfer, not operator fallback.
    return dict(zip(plies[1][1].cpu().tolist(), V.cpu().tolist())), census
