/** Run under run_capped.py. Uses frozen time for parity and an accelerated
 * clock after a completed round to exercise checkpoint retention. */
import { readFileSync, writeFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import assert from 'node:assert/strict';
import { fileURLToPath } from 'node:url';
const module = await WebAssembly.compile(readFileSync(process.argv[2]));
const nativePath = new URL('../target/release/walt-table', import.meta.url);
const request = {contract:'nello',decl:8,bid:1,bidder:0,seat:3,hand:[4,7,12,14,16,25,27],
  plays:[0,3,1,23,3,12,1,11,3,25,0,13,3,27,0,9,1,2],seed:1};
function wasm(call, stopAfterRound=false, reserveTest=false) {
  let x, ticks=0n, stop=false;
  const checkpoints=[],decoder=new TextDecoder();
  const instance=new WebAssembly.Instance(module,{walt_host:{now_us:()=>stop ? ticks+=7000000n : ticks,
    checkpoint:(ptr,len)=>{const v=JSON.parse(decoder.decode(new Uint8Array(x.memory.buffer,ptr,len)));
      checkpoints.push(v);
      if(reserveTest && v.evaluation?.outer_worlds===40 && !v.counterexample_result?.rounds) ticks=14000000n;
      if(stopAfterRound && v.counterexample_result?.rounds===1)stop=true;}}});
  x=instance.exports;
  const bytes=new TextEncoder().encode(JSON.stringify(call)); const ptr=x.walt_in_prepare(bytes.length);
  new Uint8Array(x.memory.buffer,ptr,bytes.length).set(bytes);
  const len=x.walt_call();
  return {value:JSON.parse(decoder.decode(new Uint8Array(x.memory.buffer,x.walt_out_ptr(),len))),checkpoints};
}
const call={request,worlds:160,partner:false,nello_counterexamples:true,budget_ms:20000};
const full=wasm(call), interrupted=wasm(call,true);
assert.equal(full.value.counterexample_result.rounds,3);
assert.equal(full.value.counterexample_result.witnesses,12);
assert.equal(full.value.choice,16);
const p=spawnSync(fileURLToPath(nativePath),[],{input:JSON.stringify(call)+'\n',encoding:'utf8',timeout:25000});
assert.equal(p.status,0,p.stderr);
const native=JSON.parse(p.stdout.trim().split('\n').at(-1)).result;
assert.deepEqual(native.counterexample_result,full.value.counterexample_result);
assert.deepEqual(native.evaluation.options,full.value.evaluation.options);
const last=interrupted.checkpoints.find(c=>c.counterexample_result?.rounds===1);
assert.equal(interrupted.value.counterexample_result.rounds,1);
assert.equal(interrupted.value.counterexample_result.stop,'deadline');
assert.equal(interrupted.value.choice,last.choice);
assert.deepEqual(interrupted.value.counterexample_result.options,last.counterexample_result.options);
assert.ok(interrupted.value.legal.includes(interrupted.value.choice));
// Simulate ordinary work consuming fourteen seconds after its completed 40-world
// checkpoint. The 500-world stage must yield to the reserved counterexample phase.
const reserved=wasm({...call,worlds:500},false,true);
assert.equal(reserved.value.evaluation.outer_worlds,40);
assert.ok(reserved.value.phases.some(p=>p.worlds===500 && p.status==='no-time'));
assert.equal(reserved.value.counterexample_result.status,'completed');
assert.equal(reserved.value.counterexample_result.ordinary_worlds,40);
assert.equal(reserved.value.counterexample_result.rounds,3);
const deeper=wasm({...call,worlds:500});
assert.equal(deeper.value.evaluation.outer_worlds,500);
assert.equal(deeper.value.counterexample_result.ordinary_worlds,500);
const ordinary=wasm({...call,nello_counterexamples:false}).value;
assert.equal(ordinary.counterexample_result,undefined);
assert.equal(ordinary.choice,7);
const declarer=wasm({request:{...request,seat:0,hand:[0,3,9,10,13,18,24],plays:[]},worlds:1,partner:false,nello_counterexamples:true}).value;
assert.equal(declarer.error,undefined);
assert.equal(declarer.counterexample_result,undefined);
const output={native,wasm:full.value,interrupted:interrupted.value,reserved:reserved.value,deeper:deeper.value,declarer,checks:'native/wasm parity; complete-round retention; reserved counterexample time and 500-world completion; ordinary mode and declarer isolation'};
writeFileSync(process.argv[3],JSON.stringify(output,null,2)+'\n');
console.log(output.checks);
