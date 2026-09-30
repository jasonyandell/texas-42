/** Native reference values through the actual deployable Rust WASM ABI. */
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {spawnSync} from 'node:child_process';
const wasm=process.argv[2] ?? new URL('../../target/wasm32-unknown-unknown/release/walt_player.wasm',import.meta.url);
const binary=process.argv[3] ?? new URL('../../target/release/walt-table',import.meta.url).pathname;
const module=await WebAssembly.compile(readFileSync(wasm));
const golden=JSON.parse(readFileSync(new URL('../src/ladder/golden.json',import.meta.url))).cases;
const nello=JSON.parse(readFileSync(new URL('../src/ladder/nello.json',import.meta.url)));
function run(call){
  let x;
  const instance=new WebAssembly.Instance(module,{walt_host:{now_us:()=>BigInt(Math.floor(performance.now()*1000)),checkpoint:()=>{}}});
  x=instance.exports;const bytes=new TextEncoder().encode(JSON.stringify(call));const ptr=x.walt_in_prepare(bytes.length);
  new Uint8Array(x.memory.buffer,ptr,bytes.length).set(bytes);const len=x.walt_call();
  const value=JSON.parse(new TextDecoder().decode(new Uint8Array(x.memory.buffer,x.walt_out_ptr(),len)));
  assert.equal(value.error,undefined);return value;
}
const cases=[...golden,...nello];
for(const c of cases){
  const call={request:c.request,worlds:c.samples.at(-1),profile:c.samples,partner:false,budget_ms:20000};
  const native=spawnSync(binary,{input:JSON.stringify(call)+'\n',encoding:'utf8',timeout:30000});
  assert.equal(native.status,0,native.stderr);
  const expected=JSON.parse(native.stdout.trim().split('\n').at(-1)).result;
  const actual=run(call);
  for(const field of ['choice','legal','points','leader','trick','profile','evaluation','contract','inactive'])assert.deepEqual(actual[field],expected[field],`${field}: ${JSON.stringify(call)}`);
  if(c.choice!==undefined){assert.equal(actual.choice,c.choice);if(actual.evaluation)assert.deepEqual(actual.evaluation.options,c.options);}
}
console.log(`Rust native/WASM parity: ${cases.length} own-view L1-L4 and Nel-O cases`);
