// Node smoke: run the SAME full hand as tests/full_hand.rs through the
// actual pkg/walt.wasm binary via the pkg/walt.ts wrapper, and assert the
// contract, declaration, and all 28 plays are IDENTICAL to the native
// test's frozen trace. Exploratory tooling; not a receipt.
//
//   node walt-wasm/smoke.mjs        (Node >= 23.6 for .ts type stripping)

import { readFile } from 'node:fs/promises';
import { Walt } from './pkg/walt.ts';

// Frozen native trace (cargo test --release -p walt-wasm -- --nocapture):
const NATIVE = {
  handNo: 1n,
  bidder: 0,
  bid: 42,
  decl: 9,
  record:
    '0 2 1 7 2 15 3 11 0 0 1 21 2 13 3 10 0 27 1 22 2 24 3 23 0 4 1 8 2 12 3 5 3 9 0 6 1 1 2 18 3 14 0 25 1 19 2 17 3 26 0 16 1 3 2 20',
};
const N = 6, N0 = 2;
const SEED = 0x9e3779b9n;
const M = (1n << 64n) - 1n;

class SplitMix64 {
  constructor(s) { this.s = s & M; }
  next() {
    this.s = (this.s + 0x9e3779b97f4a7c15n) & M;
    let z = this.s;
    z = ((z ^ (z >> 30n)) * 0xbf58476d1ce4e5b9n) & M;
    z = ((z ^ (z >> 27n)) * 0x94d049bb133111ebn) & M;
    return (z ^ (z >> 31n)) & M;
  }
  below(n) {
    const zone = M - (M % n);
    for (;;) {
      const v = this.next();
      if (v < zone) return v % n;
    }
  }
}

function mix(h) {
  let z = (h + 0x9e3779b97f4a7c15n) & M;
  z = ((z ^ (z >> 30n)) * 0xbf58476d1ce4e5b9n) & M;
  z = ((z ^ (z >> 27n)) * 0x94d049bb133111ebn) & M;
  return (z ^ (z >> 31n)) & M;
}

function deal(handNo) {
  const rng = new SplitMix64(SEED ^ mix(handNo));
  const tiles = [...Array(28).keys()];
  for (let i = tiles.length - 1; i >= 1; i--) {
    const j = Number(rng.below(BigInt(i + 1)));
    [tiles[i], tiles[j]] = [tiles[j], tiles[i]];
  }
  const hands = [0, 1, 2, 3].map((s) =>
    tiles.slice(7 * s, 7 * s + 7).sort((a, b) => a - b),
  );
  return hands;
}

const assert = (cond, msg) => {
  if (!cond) { console.error(`FAIL: ${msg}`); process.exit(1); }
};

const walt = await Walt.load(
  await readFile(new URL('./pkg/walt.wasm', import.meta.url)),
);
const t0 = performance.now();
const hands = deal(NATIVE.handNo);
const seed = SEED ^ mix(NATIVE.handNo);

// Auction (same protocol as the native test).
let high = null;
for (let s = 0; s < 4; s++) {
  const need = high ? high.bid + 1 : 30;
  if (need > 42) break;
  const r = walt.bid({ hand: hands[s], need, n: N, n0: N0, seed });
  if (r.action === 'bid') high = { seat: s, bid: r.bid };
}
assert(high !== null, 'auction produced a winner');
assert(high.seat === NATIVE.bidder && high.bid === NATIVE.bid,
  `contract S${high.seat}@${high.bid} == native S${NATIVE.bidder}@${NATIVE.bid}`);

const d = walt.declare({ hand: hands[high.seat], bid: high.bid, n: N, n0: N0, seed });
assert(d.decl === NATIVE.decl, `decl ${d.decl} == native ${NATIVE.decl}`);

// Play all 28: probe for the seat to act at trick starts (the API rejects
// wrong-seat calls before evaluating), then follow turn order.
const record = [];
let leader = high.seat;
for (let trick = 0; trick < 7; trick++) {
  for (let pos = 0; pos < 4; pos++) {
    let resp = null;
    let actor = (leader + pos) % 4;
    if (pos === 0 && trick > 0) {
      for (let cand = 0; cand < 4; cand++) {
        try {
          resp = walt.play({
            decl: NATIVE.decl, bid: high.bid, seat: cand, bidder: high.seat,
            hand: hands[cand], plays: record, n: N, n0: N0, seed,
          });
          actor = cand;
          leader = cand;
          break;
        } catch { /* not the seat to act — probe the next one */ }
      }
      assert(resp !== null, `some seat leads trick ${trick + 1}`);
    } else {
      resp = walt.play({
        decl: NATIVE.decl, bid: high.bid, seat: actor, bidder: high.seat,
        hand: hands[actor], plays: record, n: N, n0: N0, seed,
      });
    }
    record.push(actor, resp.choice);
  }
}
assert(record.join(' ') === NATIVE.record, 'all 28 plays match the native trace');

const ms = Math.round(performance.now() - t0);
console.log(`walt.wasm smoke OK — contract S${high.seat} bid ${high.bid} decl ${d.decl}; 28/28 plays match native; ${ms}ms total`);
