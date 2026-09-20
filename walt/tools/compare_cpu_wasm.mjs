/** Compare complete play decisions across two WASM builds. Run under the
 * experiment watchdog; each completed pair is saved before the next starts.
 * Frozen host time compares algorithms, not device-dependent review prefixes.
 */
import {readFileSync, writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import assert from 'node:assert/strict';

const [beforePath, afterPath, output] = process.argv.slice(2);
if (!beforePath || !afterPath || !output) throw new Error('Usage: compare-builds.mjs BEFORE AFTER OUTPUT');
const bytes = [beforePath, afterPath].map(p => readFileSync(p));
const identities = bytes.map(b => createHash('sha256').update(b).digest('hex'));
const modules = await Promise.all(bytes.map(b => WebAssembly.compile(b)));
function solve(module, call) {
  const start = performance.now();
  const instance = new WebAssembly.Instance(module, {walt_host: {now_us: () => 0n, checkpoint: () => {}}});
  const x = instance.exports;
  const data = new TextEncoder().encode(JSON.stringify(call));
  const ptr = x.walt_in_prepare(data.length);
  new Uint8Array(x.memory.buffer, ptr, data.length).set(data);
  const len = x.walt_call();
  const value = JSON.parse(new TextDecoder().decode(new Uint8Array(x.memory.buffer, x.walt_out_ptr(), len)));
  return {value, milliseconds: performance.now() - start};
}
const opening = {bid:30, bidder:0, seat:0, hand:[1,6,8,19,20,23,27], plays:[], seed:420914};
const late = {decl:3,bid:30,bidder:0,seat:2,hand:[0,2,3,15,17,21,25],plays:[0,7,1,24,2,3,3,6,1,5,2,17,3,23,0,12,1,4,2,21,3,22,0,8,0,9,1,19,2,25,3,18,0,13,1,16],seed:420600};
const calls = [
  ...[0,1,2,3,4,5,6,7,9].map(decl => ({request:{...opening,decl},worlds:40,partner:false})),
  {request:late,worlds:40,partner:true},
  ...[36,42].map(bid => ({request:{...opening,decl:6,bid},worlds:40,partner:false})),
];
const rows = [];
const save = () => writeFileSync(output, JSON.stringify({schema:'walt-cpu-build-comparison-v1',
  before:identities[0], after:identities[1], clock:'frozen', host:'Node WASM; not phone timing',
  requested:calls.length, completed:rows.length, all_completed:rows.length===calls.length, rows},null,2)+'\n');
for (const [index,call] of calls.entries()) {
  const got=[];
  for (const i of index%2 ? [1,0] : [0,1]) got[i]=solve(modules[i],call);
  const [a,b]=got.map(g=>g.value);
  assert.equal(a.route, call.partner ? 'baseline-reviewed' : 'baseline');
  assert.equal(b.route,a.route);
  for(const key of ['choice','legal','points','leader','trick']) assert.deepEqual(a[key],b[key]);
  for(const field of ['evaluation','fallback_evaluation']) {
    for(const key of ['choice','legal','options'])
      assert.deepEqual(a[field][key],b[field][key],`${field}.${key}`);
  }
  if(call.partner) {
    for(const key of ['choice','values','paired','samples','support','coverage'])
      assert.deepEqual(a.review_result[key],b.review_result[key],`review.${key}`);
  }
  rows.push({call,choice:a.choice,before_ms:got[0].milliseconds,after_ms:got[1].milliseconds});
  save();
  console.log(JSON.stringify({completed:rows.length,decl:call.request.decl,bid:call.request.bid}));
}
console.log('Complete decisions, exact values and full partner review agree. Diagnostic work counters may change.');
