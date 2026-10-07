// Re-creation of Walt's L1 baseline (SCENARIO-PLAYER.md Def 3.2-6.1) for the explainer.
// Straight 42, pip trumps 0..6, 7 = doubles trump, 8 = no trump.
(function (root) {
'use strict';
const HI = [], LO = [];
for (let h = 0; h <= 6; h++) for (let l = 0; l <= h; l++) { HI.push(h); LO.push(l); }
const COUNT = HI.map((h, i) => { const s = h + LO[i]; return s === 5 ? 5 : s === 10 ? 10 : 0; });
const isDouble = i => HI[i] === LO[i];
const name = i => HI[i] + '-' + LO[i];
const popcount = m => { m = m - ((m >>> 1) & 0x55555555); m = (m & 0x33333333) + ((m >>> 2) & 0x33333333); return (((m + (m >>> 4)) & 0xF0F0F0F) * 0x1010101) >>> 24; };
const bits = m => { const out = []; while (m) { const b = 31 - Math.clz32(m & -m); out.push(b); m &= m - 1; } return out; };
const FULL = (1 << 28) - 1;

function makeRules(decl) {
  const called = new Array(28), natural = [];
  for (let i = 0; i < 28; i++) called[i] = decl <= 6 ? (HI[i] === decl || LO[i] === decl) : decl === 7 ? isDouble(i) : false;
  let calledMask = 0; for (let i = 0; i < 28; i++) if (called[i]) calledMask |= 1 << i;
  for (let p = 0; p <= 6; p++) { let m = 0; for (let i = 0; i < 28; i++) if ((HI[i] === p || LO[i] === p) && !called[i]) m |= 1 << i; natural.push(m); }
  const incidence = q => q === 7 ? calledMask : natural[q];
  const ledCtx = i => called[i] ? 7 : HI[i];
  const rank = i => isDouble(i) ? (decl === 7 ? HI[i] : 12) : HI[i] + LO[i];
  const key = new Int32Array(8 * 28);
  for (let q = 0; q < 8; q++) for (let i = 0; i < 28; i++) {
    const tier = called[i] ? 2 : (incidence(q) >>> i) & 1 ? 1 : 0;
    key[q * 28 + i] = tier === 0 ? 0 : tier * 16 + rank(i);
  }
  const legal = (hand, led) => { if (led < 0) return hand; const f = hand & incidence(led); return f ? f : hand; };
  // winner offset within the trick: first strict maximum
  const winner = plays => { const q = ledCtx(plays[0]); let b = 0, bk = key[q * 28 + plays[0]]; for (let j = 1; j < plays.length; j++) { const k = key[q * 28 + plays[j]]; if (k > bk) { bk = k; b = j; } } return b; };
  return { decl, called, calledMask, ledCtx, key, legal, winner, incidence };
}

// 32-bit mixing (stand-in for SplitMix64; deterministic, not bit-identical to Walt)
function mix32(x) { x |= 0; x = Math.imul(x ^ (x >>> 16), 0x85ebca6b); x = Math.imul(x ^ (x >>> 13), 0xc2b2ae35); return (x ^ (x >>> 16)) >>> 0; }
function rng(seed) { let s = seed >>> 0; return { next() { s = (s + 0x9E3779B9) >>> 0; return mix32(s); }, below(n) { return this.next() % n; } }; }
function recordHash(st) { let h = mix32(st.played ^ 0x51ed27); h = mix32(h ^ (st.leader + 1) * 0x1000193); for (const p of st.plays) h = mix32(h ^ (0x100 | p)); return h; }

// Game state: {played, leader, plays:[], b:[t0,t1]} ; team T1 = bidder's team.
function makeGame(decl, bid, bidder) {
  const R = makeRules(decl);
  const bt = bidder & 1;
  const team = s => ((s & 1) === bt ? 1 : 0);
  const actor = st => (st.leader + st.plays.length) & 3;
  const terminal = st => st.b1 >= bid ? 1 : st.b0 > 42 - bid ? 0 : -1;
  function step(st, t) {
    const played = st.played | (1 << t);
    if (st.plays.length === 3) {
      const plays = st.plays.concat([t]);
      const w = (st.leader + R.winner(plays)) & 3;
      let pts = 1; for (const p of plays) pts += COUNT[p];
      const t1 = team(w) === 1;
      return { played, leader: w, plays: [], b1: st.b1 + (t1 ? pts : 0), b0: st.b0 + (t1 ? 0 : pts) };
    }
    return { played, leader: st.leader, plays: st.plays.concat([t]), b1: st.b1, b0: st.b0 };
  }
  function sizes(st) {
    const n = popcount(st.played), done = (n - st.plays.length) / 4, sz = [7 - done, 7 - done, 7 - done, 7 - done];
    for (let j = 0; j < st.plays.length; j++) sz[(st.leader + j) & 3]--;
    return sz;
  }
  const ledOf = st => st.plays.length ? R.ledCtx(st.plays[0]) : -1;
  const stKey = st => st.played + ',' + st.leader + ',' + st.plays.join('.') + ',' + st.b1 + ',' + st.b0;
  return { R, bid, bidder, team, actor, terminal, step, sizes, ledOf, stKey, start: { played: 0, leader: bidder, plays: [], b1: 0, b0: 0 } };
}

// Uniform sample of the other seats' hands (voids optional), shuffle-and-reject.
function sampleWorlds(G, viewer, hand, st, n, r, voids) {
  const sz = G.sizes(st);
  const tiles = bits(FULL & ~st.played & ~hand);
  const out = [];
  while (out.length < n) {
    for (let i = tiles.length - 1; i > 0; i--) { const j = r.below(i + 1); const t = tiles[i]; tiles[i] = tiles[j]; tiles[j] = t; }
    const w = [0, 0, 0, 0]; w[viewer] = hand; let off = 0, ok = true;
    for (let s = 0; s < 4; s++) { if (s === viewer) continue; let m = 0; for (let k = 0; k < sz[s]; k++) m |= 1 << tiles[off + k]; off += sz[s]; w[s] = m; if (voids && (m & voids[s])) { ok = false; break; } }
    if (ok) out.push(w);
  }
  return out;
}

// Visit order for the viewer (only speeds the search; never changes values).
function order(G, st, legalMask, viewer) {
  const R = G.R, ts = bits(legalMask);
  if (ts.length < 2) return ts;
  let pr;
  if (st.plays.length) {
    const q = R.ledCtx(st.plays[0]); let bk = -1, wi = 0, cnt = 0;
    st.plays.forEach((p, j) => { const k = R.key[q * 28 + p]; cnt += COUNT[p]; if (k > bk) { bk = k; wi = j; } });
    const own = G.team((st.leader + wi) & 3) === G.team(viewer);
    pr = t => R.key[q * 28 + t] > bk ? 1000 + cnt + COUNT[t] : own ? 20 + COUNT[t] : 10 - COUNT[t];
  } else pr = t => R.key[R.ledCtx(t) * 28 + t];
  return ts.map(t => [pr(t), t]).sort((a, b) => b[0] - a[0] || a[1] - b[1]).map(x => x[1]);
}

// Exact solver on a fixed set of sampled worlds. Masks are BigInt-free when n <= 30.
// field(st, seat, aliveIds) -> array tile per alive id.
function Solver(G, viewer, maximize, worlds, field) {
  const n = worlds.length, memo = new Map();
  const big = n > 30;
  const ONE = big ? 1n : 1;
  const bit = big ? (i => 1n << BigInt(i)) : (i => 1 << i);
  const ZERO = big ? 0n : 0;
  const cnt = big ? (m => { let c = 0; while (m) { m &= m - 1n; c++; } return c; }) : popcount;
  this.nodes = 0;
  const self = this;
  // returns made-mask (subset of alive)
  function solve(st, alive) {
    self.nodes++;
    const t = G.terminal(st);
    if (t >= 0) return t ? alive : ZERO;
    const k = G.stKey(st) + '|' + alive.toString(36);
    const c = memo.get(k); if (c !== undefined) return c;
    const seat = G.actor(st), led = G.ledOf(st);
    let res;
    if (seat === viewer) {
      const hand = worlds[0][viewer] & ~st.played;
      let best = null, bc = -1;
      const mass = cnt(alive);
      for (const t of order(G, st, G.R.legal(hand, led), viewer)) {
        const m = solve(G.step(st, t), alive); const v = cnt(m);
        if (best === null || (maximize ? v > bc : v < bc)) { best = m; bc = v; if (maximize ? v === mass : v === 0) break; }
      }
      res = best;
    } else {
      const ids = []; let a = alive, i = 0;
      while (a) { if (big ? (a & 1n) : (a & 1)) ids.push(i); a = big ? a >> 1n : a >>> 1; i++; }
      const tiles = field(st, seat, ids);
      const buckets = new Map();
      ids.forEach((id, j) => { const t = tiles[j]; buckets.set(t, (buckets.get(t) || ZERO) | bit(id)); });
      res = ZERO;
      for (const t of [...buckets.keys()].sort((x, y) => x - y)) res |= solve(G.step(st, t), buckets.get(t));
    }
    memo.set(k, res);
    return res;
  }
  this.all = big ? (1n << BigInt(n)) - 1n : (n === 0 ? 0 : (2 ** n - 1));
  this.solve = solve;
  this.count = cnt;
  // per-candidate made masks at a root where the viewer acts
  this.actions = (st, candidates) => candidates.map(t => solve(G.step(st, t), self.all));
  // the viewer's choice at a node for this alive set (for traces)
  this.viewerChoice = (st, alive) => {
    const hand = worlds[0][viewer] & ~st.played; let best = null, bc = -1, bt = -1;
    const mass = cnt(alive);
    for (const t of order(G, st, G.R.legal(hand, G.ledOf(st)), viewer)) {
      const m = solve(G.step(st, t), alive); const v = cnt(m);
      if (best === null || (maximize ? v > bc : v < bc)) { best = m; bc = v; bt = t; if (maximize ? v === mass : v === 0) break; }
    }
    return bt;
  };
}

const INNER_SEED = 0x243F6A88;
function diceField(G, worlds, seeds) {
  return (st, seat, ids) => {
    const led = G.ledOf(st), rh = recordHash(st);
    return ids.map(id => { const lm = G.R.legal(worlds[id][seat] & ~st.played, led); const ts = bits(lm); return ts.length === 1 ? ts[0] : ts[mix32(seeds[id] ^ rh) % ts.length]; });
  };
}

// The level-0 modeled mind (Def 3.2): n0 no-void worlds, dice field, fixed argmax.
function Level0(G, n0) {
  const cache = new Map();
  let calls = 0;
  function detail(seat, hand, st, opt) {
    const ws = opt && opt.worldSalt ? mix32(opt.worldSalt * 7919 + 1) : 0;
    const base = INNER_SEED ^ mix32(seat + 1) ^ mix32(hand) ^ recordHash(st) ^ mix32(st.b1 * 64 + st.b0);
    const r = rng(base ^ ws);
    const worlds = sampleWorlds(G, seat, hand, st, n0, r, null);
    const rd = opt && opt.diceSalt ? rng(base ^ ws ^ mix32(opt.diceSalt * 104729 + 3)) : r;
    const seeds = worlds.map(() => rd.next());
    const maximize = G.team(seat) === 1;
    const S = new Solver(G, seat, maximize, worlds, diceField(G, worlds, seeds));
    const cands = bits(G.R.legal(hand & ~st.played, G.ledOf(st)));
    const masks = cands.length > 1 ? S.actions(st, cands) : [S.solve(G.step(st, cands[0]), S.all)];
    const counts = masks.map(m => S.count(m));
    let bi = 0; for (let j = 1; j < cands.length; j++) if (maximize ? counts[j] > counts[bi] : counts[j] < counts[bi]) bi = j;
    return { seat, hand, worlds, seeds, maximize, cands, masks, counts, choice: cands[bi], solver: S, st };
  }
  function choose(st, seat, hand) {
    const lm = G.R.legal(hand, G.ledOf(st));
    if (popcount(lm) === 1) return bits(lm)[0];
    const k = seat + ':' + hand + ':' + G.stKey(st);
    let c = cache.get(k); if (c !== undefined) return c;
    calls++;
    c = detail(seat, hand, st).choice;
    cache.set(k, c); return c;
  }
  return { choose, detail, calls: () => calls, cacheSize: () => cache.size };
}

// Level-1 Walt (Def 6.1): n outer worlds (void-conditioned), field = level-0 minds.
function level1(G, viewer, hand, st, n, n0, seed) {
  const L0 = Level0(G, n0);
  const r = rng(seed);
  const worlds = sampleWorlds(G, viewer, hand, st, n, r, null);
  const field = (s, seat, ids) => ids.map(id => L0.choose(s, seat, worlds[id][seat] & ~s.played));
  const S = new Solver(G, viewer, G.team(viewer) === 1, worlds, field);
  const cands = bits(G.R.legal(hand & ~st.played, G.ledOf(st)));
  const masks = S.actions(st, cands);
  return { worlds, cands, masks, counts: masks.map(m => S.count(m)), solver: S, L0, field };
}

// Follow one world through the solved tree after a root candidate.
function trace(G, S, field, worlds, viewer, st0, cand, w) {
  const big = worlds.length > 30;
  const bit = i => big ? 1n << BigInt(i) : 1 << i;
  let st = G.step(st0, cand), alive = S.all;
  const steps = [{ seat: viewer, tile: cand, by: 'root', st: st0 }];
  // keep alive = worlds that agree with w's path so far
  const ids = [...Array(worlds.length).keys()];
  while (G.terminal(st) < 0) {
    const seat = G.actor(st);
    let t;
    if (seat === viewer) { t = S.viewerChoice(st, alive); steps.push({ seat, tile: t, by: 'viewer', st }); }
    else {
      const aliveIds = ids.filter(i => big ? (alive & bit(i)) : (alive & bit(i)));
      const tiles = field(st, seat, aliveIds);
      t = tiles[aliveIds.indexOf(w)];
      let nm = big ? 0n : 0; aliveIds.forEach((id, j) => { if (tiles[j] === t) nm |= bit(id); });
      steps.push({ seat, tile: t, by: 'field', st, split: [...new Set(tiles)].length, stay: aliveIds.filter((id, j) => tiles[j] === t).length, of: aliveIds.length });
      alive = nm;
    }
    st = G.step(st, t);
  }
  return { steps, made: G.terminal(st) === 1, end: st };
}

const api = { HI, LO, COUNT, name, bits, popcount, makeRules, makeGame, sampleWorlds, Solver, Level0, level1, trace, rng, mix32, recordHash, diceField };
if (typeof module !== 'undefined') module.exports = api; else root.WaltLite = api;
})(this);
