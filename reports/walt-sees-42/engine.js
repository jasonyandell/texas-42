// A small, exact Texas 42 engine for the "How Walt sees 42" live presentation.
// EXPLORATORY teaching code: it mirrors walt's sampling stack
// (walt/SCENARIO-PLAYER.md §§3–6) at toy sizes so a room can watch it run.
// It is NOT walt's solver and nothing here is a receipt; the numbers it prints
// are exact on its own tiny sample and nothing more.
//
// Straight 42, fives trump, bid 30 held by team T1 (seats 1 and 3).
// Rules as in walt/walt/src/rules/rules.rs: a trump leads the trump suit,
// anything else leads its high pip; follow the led suit if able; trumps beat
// followers beat sloughs; doubles top their suit, else rank by pip sum.
// Counts: 5-5 and 6-4 are 10, pip-sum-five tiles are 5, each trick is 1.
(function (root) {
  'use strict';
  const TRUMP = 5, BID = 30;
  const HI = [], LO = [];
  for (let h = 0; h <= 6; h++) for (let l = 0; l <= h; l++) { HI.push(h); LO.push(l); } // index = h(h+1)/2 + l
  const idx = (h, l) => (h >= l ? h * (h + 1) / 2 + l : l * (l + 1) / 2 + h);
  const name = t => `${HI[t]}-${LO[t]}`;
  const isTrump = t => HI[t] === TRUMP || LO[t] === TRUMP;
  const count = t => { const s = HI[t] + LO[t]; return s === 5 ? 5 : s === 10 ? 10 : 0; };
  const ledSuit = t => (isTrump(t) ? 7 : HI[t]);
  const follows = (t, suit) => (suit === 7 ? isTrump(t) : !isTrump(t) && (HI[t] === suit || LO[t] === suit));
  const rank = t => (HI[t] === LO[t] ? 20 : HI[t] + LO[t]);
  const key = (t, suit) => (isTrump(t) ? 200 : follows(t, suit) ? 100 : 0) + rank(t);
  const team = s => s & 1; // seats 1,3 → T1 (the bidders); 0,2 → T0
  const bits = m => { const o = []; for (let i = 0; i < 28; i++) if (m & (1 << i)) o.push(i); return o; };
  const pop = m => { let c = 0; while (m) { m &= m - 1; c++; } return c; };
  const ALL = (1 << 28) - 1;

  function legal(hand, trick) {
    if (trick.length === 0) return hand;
    const suit = ledSuit(trick[0]);
    let f = 0;
    for (const t of bits(hand)) if (follows(t, suit)) f |= 1 << t;
    return f || hand;
  }
  function winnerOf(trick, leader) {
    const suit = ledSuit(trick[0]);
    let best = 0;
    for (let k = 1; k < 4; k++) if (key(trick[k], suit) > key(trick[best], suit)) best = k;
    return (leader + best) & 3;
  }

  // ---- the public record (shared by every world at a node) ----
  const start = leader => ({ played: 0, leader, trick: [], banked: [0, 0], log: [] });
  const turn = st => (st.leader + st.trick.length) & 3;
  function play(st, t) {
    const seat = turn(st);
    const trick = st.trick.concat([t]);
    const log = st.log.concat([{ seat, t }]);
    const played = st.played | (1 << t);
    if (trick.length < 4) return { played, leader: st.leader, trick, banked: st.banked, log };
    const w = winnerOf(trick, st.leader);
    const pts = 1 + trick.reduce((a, x) => a + count(x), 0);
    const banked = st.banked.slice(); banked[team(w)] += pts;
    return { played, leader: w, trick: [], banked, log, lastTrick: { trick, leader: st.leader, winner: w, pts } };
  }
  const decided = st => (st.banked[1] >= BID ? 1 : st.banked[0] > 42 - BID ? 0 : -1);
  // κ(R): what the tape is keyed on — what's been played, who leads, the trick so far, the score
  const recordKey = st => `${st.played}|${st.leader}|${st.trick.join(',')}|${st.banked[0]},${st.banked[1]}`;

  // ---- hashing: a deterministic scrambler (splitmix-style, 32-bit) ----
  function mix(x) {
    x = (x ^ (x >>> 16)) >>> 0; x = Math.imul(x, 0x7feb352d) >>> 0;
    x = (x ^ (x >>> 15)) >>> 0; x = Math.imul(x, 0x846ca68b) >>> 0;
    return (x ^ (x >>> 16)) >>> 0;
  }
  function hashStr(s) { let h = 0x811c9dc5; for (let i = 0; i < s.length; i++) h = Math.imul(h ^ s.charCodeAt(i), 16777619) >>> 0; return mix(h); }
  function rng(seed) { let s = seed >>> 0; return () => { s = (s + 0x9e3779b9) >>> 0; return mix(s) / 4294967296; }; }

  // THE TICKERTAPE: a die roll that is a fixed function of (world, record).
  // Same world + same record → same roll, in every branch and every rerun.
  const tapeRoll = (worldSeed, st, n) => mix((worldSeed ^ hashStr(recordKey(st))) >>> 0) % n;

  // ---- worlds: lawful completions of what a seat can see ----
  // A world = { seed, hands: [m0,m1,m2,m3] } (full dealt hands; current hand = hand & ~played).
  // voids: per-seat masks of tiles known impossible (walt's outer sampler respects them;
  // level-0 minds ignore them — the declared shortcut).
  function sampleWorlds(viewer, viewerHand, st, n, seed, voidsFor) {
    const out = [];
    const r = rng(seed);
    const seen = st.played | viewerHand;
    const unseen = bits(ALL & ~seen);
    // hand sizes forced by the record: 7 minus what each seat has played
    const playedBy = [0, 0, 0, 0]; for (const p of st.log) playedBy[p.seat]++;
    const need = [0, 1, 2, 3].map(s => (s === viewer ? 0 : 7 - playedBy[s]));
    let guard = 0;
    while (out.length < n && guard++ < n * 5000) {
      const tiles = unseen.slice();
      for (let i = tiles.length - 1; i > 0; i--) { const j = Math.floor(r() * (i + 1)); [tiles[i], tiles[j]] = [tiles[j], tiles[i]]; }
      const hands = [0, 0, 0, 0]; let k = 0, ok = true;
      for (let s = 0; s < 4 && ok; s++) {
        if (s === viewer) continue;
        for (let c = 0; c < need[s]; c++) {
          const t = tiles[k++];
          if (voidsFor && voidsFor[s] & (1 << t)) { ok = false; break; }
          hands[s] |= 1 << t;
        }
      }
      if (!ok) continue;
      // put back what each seat already played so a world holds full dealt hands
      for (const p of st.log) hands[p.seat] |= 1 << p.t;
      hands[viewer] = viewerHand | bitsOf(st.log.filter(p => p.seat === viewer).map(p => p.t));
      out.push({ seed: mix(seed + out.length * 7919 + guard), hands });
    }
    return out;
  }
  const bitsOf = ts => ts.reduce((m, t) => m | (1 << t), 0);
  // public voids: a seat that failed to follow the led suit is void in it
  function publicVoids(st) {
    const v = [0, 0, 0, 0];
    const tricks = [];
    for (let i = 0; i < st.log.length; i += 4) tricks.push(st.log.slice(i, i + 4));
    for (const tr of tricks) {
      const suit = ledSuit(tr[0].t);
      for (const p of tr.slice(1)) if (!follows(p.t, suit)) for (let t = 0; t < 28; t++) if (follows(t, suit)) v[p.seat] |= 1 << t;
    }
    return v;
  }

  // ---- fields: how the non-viewer seats are modeled ----
  // dice: uniform over legal tiles, rolled from the tape (or, for the demo, fresh each time)
  function diceField(opts = {}) {
    let fresh = opts.freshSeed >>> 0;
    const f = (seat, world, st) => {
      const hand = world.hands[seat] & ~st.played;
      const L = bits(legal(hand, st.trick));
      const i = opts.fresh ? (fresh = mix(fresh + 0x9e3779b9)) % L.length : tapeRoll(world.seed, st, L.length);
      if (f.onRoll) f.onRoll({ seat, world, st, legal: L, roll: i, tile: L[i] });
      return L[i];
    };
    f.kind = opts.fresh ? 'fresh' : 'tape';
    return f;
  }

  // ---- the solver: sampled worlds solved together, viewer best-responds ----
  // Counts how many alive worlds make 30 (integers — exact on the sample).
  // Viewer nodes pick one tile for ALL alive worlds (no peeking: its hand is the
  // same in every world). Field nodes split the alive worlds by what the field plays.
  function makeSolver(viewer, worlds, field, opts = {}) {
    const memo = new Map();
    const noMemo = field.kind === 'fresh';
    const viewerHand = worlds[0].hands[viewer];
    const maximize = team(viewer) === 1;
    let nodes = 0;
    function solve(st, alive) {
      nodes++;
      const d = decided(st);
      if (d >= 0) return d ? pop(alive) : 0;
      const k = noMemo ? null : recordKey(st) + '#' + alive;
      if (k && memo.has(k)) return memo.get(k).v;
      const seat = turn(st);
      let v, best = -1;
      if (seat === viewer) {
        const L = bits(legal(viewerHand & ~st.played, st.trick));
        const all = pop(alive);
        v = maximize ? -1 : Infinity;
        for (const t of L) {
          const c = solve(play(st, t), alive);
          if (maximize ? c > v : c < v) { v = c; best = t; }
          if (maximize ? v === all : v === 0) break; // decided cutoff: can't do better
        }
      } else {
        const buckets = new Map();
        for (const w of bits(alive)) {
          const t = field(seat, worlds[w], st);
          buckets.set(t, (buckets.get(t) || 0) | (1 << w));
        }
        v = 0;
        for (const [t, m] of buckets) v += solve(play(st, t), m);
      }
      if (k) memo.set(k, { v, best });
      return v;
    }
    // per-candidate values at a viewer node (what a mind compares before choosing)
    function candidates(st, alive = (1 << worlds.length) - 1) {
      const L = bits(legal(viewerHand & ~st.played, st.trick));
      return L.map(t => ({ t, makes: solve(play(st, t), alive), of: pop(alive) }));
    }
    // follow world w down the solved tree (viewer takes its chosen tile, field its move)
    function trace(st, w, alive = (1 << worlds.length) - 1, firstTile) {
      const path = [];
      let s = st;
      if (firstTile !== undefined) { path.push({ seat: turn(s), t: firstTile, alive }); s = play(s, firstTile); }
      while (s.log.length < 28 && decided(s) < 0) {
        const seat = turn(s);
        let t;
        if (seat === viewer) {
          solve(s, alive);
          t = memo.get(recordKey(s) + '#' + alive).best;
          path.push({ seat, t, alive, choice: true });
        } else {
          const buckets = new Map();
          for (const x of bits(alive)) { const tx = field(seat, worlds[x], s); buckets.set(tx, (buckets.get(tx) || 0) | (1 << x)); }
          t = field(seat, worlds[w], s);
          alive = buckets.get(t);
          path.push({ seat, t, alive });
        }
        s = play(s, t);
      }
      return { path, end: s };
    }
    return { solve, candidates, trace, memo, get nodes() { return nodes; }, viewerHand };
  }

  // ---- level-0 mind: samples its own n0 worlds (no voids) and best-responds to dice ----
  function level0Field(n0, opts = {}) {
    const cache = new Map();
    const dice = diceField();
    const f = (seat, world, st) => {
      const hand = world.hands[seat] & ~st.played;
      const k = seat + ':' + hand + ':' + recordKey(st);
      if (cache.has(k)) return cache.get(k).t;
      const L = bits(legal(hand, st.trick));
      let pick = L[0], vals = null, inner = null;
      if (L.length > 1) {
        const seed = mix(0x243f6a88 ^ mix(seat * 977 + hand) ^ hashStr(recordKey(st)));
        inner = sampleWorlds(seat, hand, st, n0, seed, null);
        const S = makeSolver(seat, inner, dice);
        vals = S.candidates(st);
        const max = team(seat) === 1;
        let bv = max ? -1 : Infinity;
        for (const c of vals) if (max ? c.makes > bv : c.makes < bv) { bv = c.makes; pick = c.t; } // ties → lowest tile index
      }
      cache.set(k, { t: pick, vals, inner });
      if (f.onThink) f.onThink({ seat, st, legal: L, vals, pick, inner });
      return pick;
    };
    f.kind = 'level0'; f.cache = cache;
    return f;
  }

  const api = { TRUMP, BID, HI, LO, idx, name, isTrump, count, ledSuit, follows, legal, winnerOf, team, bits, pop, bitsOf,
    start, turn, play, decided, recordKey, mix, hashStr, rng, tapeRoll, sampleWorlds, publicVoids,
    diceField, level0Field, makeSolver };
  if (typeof module !== 'undefined') module.exports = api; else root.T42 = api;
})(this);
