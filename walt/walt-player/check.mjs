/** Native/browser conformance, frozen-time partner check, and accelerated
 * browser-deadline test. Run under the experiment watchdog. */
import { readFileSync, writeFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import assert from 'node:assert/strict';
import { fileURLToPath } from 'node:url';
const root = new URL('../target/', import.meta.url);
const bytes = readFileSync(new URL('wasm32-unknown-unknown/release/walt_player.wasm',root));
const module = await WebAssembly.compile(bytes);
const decoder = new TextDecoder();
function wasm(call, clock = () => BigInt(Math.floor(performance.now()*1000))) {
  const checkpoints=[];
  let x;
  const instance = new WebAssembly.Instance(module, { walt_host: { now_us: clock, checkpoint: (p,n) => checkpoints.push(JSON.parse(decoder.decode(new Uint8Array(x.memory.buffer,p,n)))) } });
  x=instance.exports;
  const data=new TextEncoder().encode(JSON.stringify(call));
  const ptr=x.walt_in_prepare(data.length);
  new Uint8Array(x.memory.buffer,ptr,data.length).set(data);
  const n=x.walt_call();
  return { value:JSON.parse(decoder.decode(new Uint8Array(x.memory.buffer,x.walt_out_ptr(),n))), checkpoints };
}
function native(call) {
  const r=spawnSync(fileURLToPath(new URL('release/walt-table',root)),[],{input:JSON.stringify(call)+'\n',encoding:'utf8',timeout:19000,env:{...process.env,RAYON_NUM_THREADS:'2'}});
  assert.equal(r.status,0,r.stderr);
  return JSON.parse(r.stdout.trim().split('\n').at(-1)).result;
}
const opening={bid:30,bidder:0,seat:0,hand:[1,6,8,19,20,23,27],plays:[],seed:420914};
const late={decl:3,bid:30,bidder:0,seat:2,hand:[0,2,3,15,17,21,25],plays:[0,7,1,24,2,3,3,6,1,5,2,17,3,23,0,12,1,4,2,21,3,22,0,8,0,9,1,19,2,25,3,18,0,13,1,16],seed:420600};
const requests=[...[0,1,2,3,4,5,6,7,9].map(decl=>({...opening,decl})),late];
const rows=[];
for(const request of requests) {
  const call={request,worlds:40,partner:false};
  const a=native(call), b=wasm(call).value;
  assert.equal(a.route,'baseline');assert.equal(b.route,'baseline');
  for(const key of ['choice','legal','points','leader','trick'])assert.deepEqual(a[key],b[key]);
  for(const field of ['evaluation','fallback_evaluation'])assert.deepEqual(a[field].options,b[field].options);
  const row={decl:request.decl,plays:request.plays.length/2,choice:a.choice,native_us:a.elapsed_us,wasm_us:b.elapsed_us};
  rows.push(row);console.log(JSON.stringify(row));
}
// Freeze browser time to prove the check, not scheduling, matches the saved
// 64-world policy. Live-clock behavior is separately exercised below.
const checked=wasm({request:late,worlds:40,partner:true},()=>0n).value;
const a=native({request:late,worlds:40,partner:true});
const wire=Object.entries(late).map(([k,v])=>k+' '+(Array.isArray(v)?v.join(' '):v)).join('\n')+'\n';
const reviewProcess=spawnSync(fileURLToPath(new URL('release/partner_rollout',root)),['--baseline','2','--milliseconds','14000','--samples','64'],{input:wire,encoding:'utf8',timeout:18000});
assert.equal(reviewProcess.status,0,reviewProcess.stderr);
const completeReview=JSON.parse(reviewProcess.stdout);
assert.equal(completeReview.samples,64);
assert.deepEqual(completeReview.values,checked.review_result.values);
assert.deepEqual(completeReview.paired,checked.review_result.paired);
assert.equal(completeReview.choice,checked.choice);
assert.equal(checked.review_result.samples,64);
if(a.review_result.samples===64) {
  assert.deepEqual(a.review_result.values,checked.review_result.values);
  assert.equal(a.choice,checked.choice);
}
let ticks=0n;
const stopped=wasm({request:{...opening,decl:5},worlds:40,partner:true,budget_ms:100},()=>ticks+=5000n);
assert.ok(['legal-fallback','l1-fallback'].includes(stopped.value.route));
assert.ok(stopped.checkpoints.length>=1);assert.ok(stopped.value.legal.includes(stopped.value.choice));
assert.equal(stopped.value.evaluation,null);
for(const request of [{...opening,decl:5,hands:[]},{...opening,decl:5,seed:-1},{...opening,decl:5,hand:[1,1,8,19,20,23,27]}])assert.ok(wasm({request}).value.error);
const result={rows,partner:{native:a.review_result,wasm:checked.review_result,complete_native:completeReview},deadline:stopped.value};
if(process.argv[2])writeFileSync(process.argv[2],JSON.stringify(result,null,2)+'\n');
console.log('Native/WASM, paired partner check, malformed input, and clock interruption checks passed.');
