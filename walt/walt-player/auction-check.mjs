/** Bounded auction/native/browser conformance; run with the packet watchdog. */
import {readFileSync,writeFileSync} from 'node:fs';
import {spawnSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
import assert from 'node:assert/strict';
const target=new URL('../target/',import.meta.url);
const module=await WebAssembly.compile(readFileSync(new URL('wasm32-unknown-unknown/release/walt_player.wasm',target)));
function wasm(call,clock=()=>BigInt(Math.floor(performance.now()*1000)),onCheckpoint=()=>{}) {
  const checkpoints=[];let x;
  x=new WebAssembly.Instance(module,{walt_host:{now_us:clock,checkpoint:(p,n)=>{const value=JSON.parse(new TextDecoder().decode(new Uint8Array(x.memory.buffer,p,n)));checkpoints.push(value);onCheckpoint(value);}}}).exports;
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
  const receipts=b.prices.map(([decl])=>{
    const job={auction_price:auction,decl,worlds:4,budget_ms:14000};
    const receipt=wasm(job).value;
    assert.deepEqual(receipt,native(job));
    return receipt;
  });
  const merge={auction_merge:auction,worlds:4,receipts};
  const merged=wasm({...merge,receipts:[...receipts].reverse()}).value;
  assert.deepEqual(merged,native(merge));
  assert.deepEqual(merged.prices,b.prices);assert.equal(merged.decl,b.decl);assert.equal(merged.eligible,b.eligible);
  const invalid=[{...merge,receipts:receipts.slice(1)},
    {...merge,receipts:[receipts[0],...receipts.slice(0,8)]},
    {...merge,worlds:12},{...merge,auction_merge:{...auction,seed:auction.seed+1}}];
  for(const field of ['schema','auction','worlds','inner_worlds','price']) {
    const bad=structuredClone(merge);
    bad.receipts[0][field]=({schema:'old',auction:{...auction,seat:(seat+1)%4},worlds:12,inner_worlds:2,price:[0,'1','0']})[field];
    invalid.push(bad);
  }
  for(const bad of invalid)assert.ok(wasm(bad).value.error,JSON.stringify(bad));
  // Identical ties cannot acquire an order preference from worker timing.
  const ties=receipts.map(r=>({...r,price:[r.price[0],'3','4']}));
  const tied=wasm({...merge,receipts:ties}).value;
  assert.equal(tied.eligible,true);
  assert.equal(wasm({...merge,receipts:ties.reverse()}).value.decl,tied.decl);
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
const live=wasm(call);assert.ok(live.value.elapsed_us<5500000);assert.ok([0,4,12,40,160].includes(live.value.worlds));
assert.equal(live.value.prices.length,live.value.worlds?9:0);
let ticks=0n;const stopped=wasm(call,()=>ticks+=100000n);
assert.equal(stopped.value.worlds,0);assert.equal(stopped.value.eligible,false);assert.equal(stopped.value.prices.length,0);
for(const auction of [{...call.auction,hands:[]},{...call.auction,bid:29},{...call.auction,hand:[1,1,8,19,20,23,27]}])assert.ok(wasm({auction}).value.error);
for(const job of [{decl:8,worlds:4,budget_ms:100},{decl:0,worlds:8,budget_ms:100},{decl:0,worlds:4,budget_ms:0}])
  assert.ok(wasm({auction_price:call.auction,...job}).value.error);
assert.equal(wasm({auction_merge:call.auction,worlds:0,receipts:[]}).value.worlds,0);
let jobTicks=0n;
assert.equal(wasm({auction_price:call.auction,decl:0,worlds:40,budget_ms:5},()=>jobTicks+=100000n).value.error,'deadline');
assert.equal(wasm({auction_price:call.auction,decl:0,worlds:160,budget_ms:20000},()=>jobTicks+=100000n).value.error,'deadline');
let stopDeeper=false;
const deeperRequest={...call.auction,bidder:0,plays:[],decl:5};
const ordinary=wasm({request:deeperRequest,worlds:40,partner:false,budget_ms:20000},()=>0n).value;
const held=wasm({request:deeperRequest,worlds:160,partner:false,budget_ms:20000},
  ()=>stopDeeper?25000000n:0n,value=>{if(value.phases?.some(p=>p.name==='baseline'&&p.worlds===40&&p.status==='completed'))stopDeeper=true;}).value;
assert.deepEqual(held.evaluation,ordinary.evaluation);assert.equal(held.choice,ordinary.choice);assert.equal(held.route,'baseline',JSON.stringify(held));
assert.equal(held.phases.at(-1).worlds,160);assert.notEqual(held.phases.at(-1).status,'completed');
const result={rows,live,stopped};if(process.argv[2])writeFileSync(process.argv[2],JSON.stringify(result,null,2)+'\n');
console.log('Auction/job/merge parity, order-independent ties, receipt validation, best-value shortcut, higher contracts, and clock interruption passed.');
