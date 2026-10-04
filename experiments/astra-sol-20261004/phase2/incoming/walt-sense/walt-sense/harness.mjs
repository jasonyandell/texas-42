// node harness.mjs <deals> <seed> [worldsWalt=40] [partner=0]   — real Plunge Walt (wasm) vs new player (./walt decide)
import fs from 'node:fs'; import { execFileSync } from 'node:child_process';
const [N = 2, SEED = 1, WORLDS = 40, PARTNER = 0] = process.argv.slice(2).map(Number);
const TILES = []; for (let h = 0; h < 7; h++) for (let l = 0; l <= h; l++) TILES.push([h, l]);   // triangular id = h(h+1)/2 + l
const PTS = TILES.map(([h, l]) => (h + l === 5 || h + l === 10) ? h + l : 0);
const BID = 30;
// ---- rules (walt42)
let TRUMP = 0, SUIT = [], LEAD = [], TRUMPS = 0;
function rules(trump) { TRUMP = trump; TRUMPS = 0; for (let t = 0; t < 28; t++) if (TILES[t].includes(trump)) TRUMPS |= 1 << t;
  SUIT = []; for (let q = 0; q < 7; q++) { let m = 0; for (let t = 0; t < 28; t++) if (TILES[t].includes(q) && !(TRUMPS >> t & 1)) m |= 1 << t; SUIT.push(m); } SUIT.push(TRUMPS);
  LEAD = TILES.map(([h], t) => (TRUMPS >> t & 1) ? 7 : h); }
const popc = m => { let c = 0; for (let t = 0; t < 28; t++) c += m >> t & 1; return c; };
const legal = (hand, led) => led < 0 ? hand : ((hand & SUIT[led]) || hand);
const strength = (t, led) => ((TRUMPS >> t & 1) ? 2 : (SUIT[led] >> t & 1) ? 1 : 0) * 100 + (TILES[t][0] === TILES[t][1] ? 12 : TILES[t][0] + TILES[t][1]);
// ---- contract (walt42): most trumps, trump double, most doubles; ties to higher seat, then higher pip
function contract(hs) { let bs = -1, bb = -1, bt = 0;
  for (let s = 0; s < 4; s++) for (let p = 0; p < 7; p++) { let cnt = 0, dbl = 0, hasd = 0;
    for (let t = 0; t < 28; t++) if (hs[s] >> t & 1) { if (TILES[t].includes(p)) cnt++; if (TILES[t][0] === TILES[t][1]) { dbl++; if (TILES[t][0] === p) hasd = 1; } }
    const sc = cnt * 100 + hasd * 10 + dbl; if (sc > bs || (sc === bs && (s > bb || (s === bb && p > bt)))) { bs = sc; bb = s; bt = p; } }
  return [bb, bt]; }
// ---- Walt (wasm)
const bytes = fs.readFileSync('/home/claude/plunge/src/ai/phone/walt-player.wasm');
let ex; const dec = new TextDecoder(), enc = new TextEncoder();
const wasm = await WebAssembly.instantiate(bytes, { walt_host: { now_us: () => BigInt(Math.floor(performance.now() * 1000)), checkpoint: () => {} } });
ex = wasm.instance.exports;
function walt(call) { const input = enc.encode(JSON.stringify(call)); const ptr = ex.walt_in_prepare(input.length);
  new Uint8Array(ex.memory.buffer, ptr, input.length).set(input); const len = ex.walt_call();
  return JSON.parse(dec.decode(new Uint8Array(ex.memory.buffer, ex.walt_out_ptr(), len))); }
// ---- rng (xorshift, deterministic)
let rs = 1n; const rnd64 = () => { rs ^= rs << 13n; rs &= (1n << 64n) - 1n; rs ^= rs >> 7n; rs ^= rs << 17n; rs &= (1n << 64n) - 1n; return rs; };
const rndint = n => Number((rnd64() >> 11n) % BigInt(n));
// ---- one hand. players[seat] = 'walt' | 'new'. Returns made (1/0). Seats are the ORIGINAL labels; bidder leads.
let waltMs = 0, waltCalls = 0, newMs = 0, newCalls = 0, checks = 0;
function playHand(hands, players, bidder, trump, seed) {
  rules(trump); const bidTeam = bidder % 2;
  let played = 0, leader = bidder, trick = [], pts = [0, 0], plays = [];
  const voidsIgnored = null;
  while (true) {
    if (pts[bidTeam] >= BID) return 1; if (pts[1 - bidTeam] > 42 - BID) return 0; if (popc(played) === 28) return pts[bidTeam] >= BID ? 1 : 0;
    const seat = (leader + trick.length) % 4; const hand = hands[seat] & ~played; const led = trick.length ? LEAD[trick[0]] : -1; const L = legal(hand, led);
    let t;
    if (popc(L) === 1) t = Math.log2(L) | 0;
    else if (players[seat] === 'walt') {
      const t0 = performance.now();
      const req = { decl: trump, bid: BID, bidder, seat, hand: [...Array(28).keys()].filter(i => hands[seat] >> i & 1), plays, seed: seed * 1000 + plays.length };
      const r = walt({ request: req, worlds: WORLDS, partner: !!PARTNER, budget_ms: 14000 });
      waltMs += performance.now() - t0; waltCalls++;
      t = r.choice;
      // rules cross-check: Walt's banked points [team 0&2, team 1&3] must equal ours
      if (r.points[0] !== pts[0] || r.points[1] !== pts[1] || r.leader !== leader) throw new Error(`rules mismatch: walt ${JSON.stringify([r.points, r.leader])} vs ${JSON.stringify([pts, leader])}`);
      checks++;
      if (!(L >> t & 1)) throw new Error('walt played an illegal tile');
    } else {
      const t0 = performance.now();
      // the C player assumes the bidding team is odd seats: relabel so bidder -> seat 1
      const rel = s => (s - bidder + 1 + 4) % 4;
      const args = ['decide', String(rel(seat)), String(BID), '1', String(trump), String(hands[seat] >>> 0), ...plays.flatMap((v, i) => i % 2 === 0 ? [String(rel(v))] : [String(v)])];
      t = Number(execFileSync('/home/claude/walt/walt', args).toString().trim());
      newMs += performance.now() - t0; newCalls++;
      if (!(L >> t & 1)) throw new Error('new player played an illegal tile');
    }
    played |= 1 << t; trick.push(t); plays.push(seat, t);
    if (trick.length === 4) { const ld = LEAD[trick[0]]; let best = 0; for (let i = 1; i < 4; i++) if (strength(trick[i], ld) > strength(trick[best], ld)) best = i;
      const w = (leader + best) % 4; pts[w % 2] += 1 + trick.reduce((a, x) => a + PTS[x], 0); leader = w; trick = []; }
  }
}
let wins = 0, hands = 0;
for (let h = 0; h < N; h++) {
  rs = BigInt(SEED) * 1000003n + BigInt(h) * 7919n + 12345n; rnd64();
  const order = [...Array(28).keys()]; for (let i = 27; i > 0; i--) { const j = rndint(i + 1); [order[i], order[j]] = [order[j], order[i]]; }
  const hs = [0, 0, 0, 0]; for (let i = 0; i < 4; i++) for (let k = 0; k < 7; k++) hs[i] |= 1 << order[7 * i + k];
  const [bb, bt] = contract(hs);
  for (let side = 0; side < 2; side++) {        // 'new' holds seats with parity == side
    const players = [0, 1, 2, 3].map(q => (q % 2) === side ? 'new' : 'walt');
    const made = playHand(hs, players, bb, bt, SEED * 100 + h * 2 + side);
    const newBids = (bb % 2) === side; hands++; wins += newBids ? made : 1 - made;
  }
}
const w = wins / hands;
console.log(`new vs PlungeWalt(wasm, worlds ${WORLDS}, partner ${PARTNER}): ${hands} hands, new wins ${wins} (${(100 * w).toFixed(1)}%)  | walt ${waltCalls} calls, ${(waltMs / Math.max(waltCalls, 1)).toFixed(0)} ms each; new ${newCalls} calls, ${(newMs / Math.max(newCalls, 1)).toFixed(0)} ms each; ${checks} rules checks passed`);
