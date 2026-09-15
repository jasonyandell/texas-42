/** Bounded auction/native/browser conformance; run with the packet watchdog. */
import {readFileSync,writeFileSync} from 'node:fs';
import {spawnSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
import assert from 'node:assert/strict';
const target=new URL('../target/',import.meta.url);
const module=await WebAssembly.compile(readFileSync(new URL('wasm32-unknown-unknown/release/walt_player.wasm',target)));
function wasm(call,clock=()=>BigInt(Math.floor(performance.now()*1000))) {
  const checkpoints=[];let x;
  x=new WebAssembly.Instance(module,{walt_host:{now_us:clock,checkpoint:(p,n)=>checkpoints.push(JSON.parse(new TextDecoder().decode(new Uint8Array(x.memory.buffer,p,n))))}}).exports;
  const data=new TextEncoder().encode(JSON.stringify(call));
  const p=x.walt_in_prepare(data.length);new Uint8Array(x.memory.buffer,p,data.length).set(data);
  const n=x.walt_call();return {value:JSON.parse(new TextDecoder().decode(new Uint8Array(x.memory.buffer,x.walt_out_ptr(),n))),checkpoints};
}
function native(call) {
  const p=spawnSync(fileURLToPath(new URL('release/walt-table',target)),[],{input:JSON.stringify(call)+'\n',encoding:'utf8',timeout:19000,env:{...process.env,RAYON_NUM_THREADS:'2'}});
  assert.equal(p.status,0,p.stderr);return JSON.parse(p.stdout.trim().split('\n').at(-1)).result;
}
const rows=[];
for(const [bid,seat] of [[30,0],[36,2],[42,3]]) {
  const auction={bid,seat,hand:[1,6,8,19,20,23,27],seed:420914};
  const call={auction,budget_ms:14000,worlds:4};
  const a=native(call),b=wasm(call).value;
  assert.equal(a.worlds,4);assert.equal(b.worlds,4);assert.deepEqual(a.prices,b.prices);assert.equal(a.decl,b.decl);
  // Root-value shortcut must equal max of the SAME complete action vector.
  for(const [decl,num,den] of b.prices) {
    const full=wasm({request:{...auction,bidder:seat,plays:[],decl},worlds:4,partner:false}).value;
    assert.equal(full.route,'baseline');
    const chosen=full.evaluation.options.find(row=>row[0]===full.choice);
    assert.deepEqual([num,den],chosen.slice(1));
  }
  rows.push({bid,seat,native_us:a.elapsed_us,wasm_us:b.elapsed_us,prices:b.prices});
  console.log(JSON.stringify(rows.at(-1)));
}
const call={auction:{bid:30,seat:0,hand:[1,6,8,19,20,23,27],seed:420914},budget_ms:4500};
const live=wasm(call);assert.ok(live.value.elapsed_us<5500000);assert.ok([0,4,12,40].includes(live.value.worlds));
assert.equal(live.value.prices.length,live.value.worlds?9:0);
let ticks=0n;const stopped=wasm(call,()=>ticks+=100000n);
assert.equal(stopped.value.worlds,0);assert.equal(stopped.value.eligible,false);assert.equal(stopped.value.prices.length,0);
for(const auction of [{...call.auction,hands:[]},{...call.auction,bid:29},{...call.auction,hand:[1,1,8,19,20,23,27]}])assert.ok(wasm({auction}).value.error);
const result={rows,live,stopped};if(process.argv[2])writeFileSync(process.argv[2],JSON.stringify(result,null,2)+'\n');
console.log('Auction parity, best-value shortcut, higher contracts, and clock interruption passed.');
