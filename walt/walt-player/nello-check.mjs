/** Frozen engine histories: same own/public decisions on native and WASM.
 * Run through the packet watchdog. Timings describe this host, not a phone. */
import {readFileSync, writeFileSync} from 'node:fs';
import {spawnSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
import assert from 'node:assert/strict';
const fixtures=JSON.parse(readFileSync(new URL('tests/fixtures/nello.json',import.meta.url)));
const target=new URL('../target/',import.meta.url);
const module=await WebAssembly.compile(readFileSync(new URL('wasm32-unknown-unknown/release/walt_player.wasm',target)));
const decoder=new TextDecoder();
function wasm(call,clock=()=>BigInt(Math.floor(performance.now()*1000))) {
  let x;const checkpoints=[];
  x=new WebAssembly.Instance(module,{walt_host:{now_us:clock,checkpoint:(p,n)=>checkpoints.push(JSON.parse(decoder.decode(new Uint8Array(x.memory.buffer,p,n))))}}).exports;
  const data=new TextEncoder().encode(JSON.stringify(call)),p=x.walt_in_prepare(data.length);
  new Uint8Array(x.memory.buffer,p,data.length).set(data);
  const n=x.walt_call();
  return {result:JSON.parse(decoder.decode(new Uint8Array(x.memory.buffer,x.walt_out_ptr(),n))),checkpoints};
}
function request(f,ply) {
  const seat=f.plays[ply*2];
  return {contract:'nello',decl:8,bid:1,bidder:f.declarer,seat,hand:[...f.hands[seat]].sort((a,b)=>a-b),plays:f.plays.slice(0,ply*2),seed:20};
}
const rows=[];
for(const f of fixtures) {
  for(let ply=0;ply<f.plays.length/2;ply++) {
    const req=request(f,ply),call={request:req,worlds:40,partner:true,budget_ms:14000};
    const process=spawnSync(fileURLToPath(new URL('release/walt-table',target)),[],{input:JSON.stringify(call)+'\n',encoding:'utf8',timeout:18000,env:{...globalThis.process.env,RAYON_NUM_THREADS:'2'}});
    assert.equal(process.status,0,process.stderr);
    const a=JSON.parse(process.stdout.trim().split('\n').at(-1)).result;
    const b=wasm(call).result;
    assert.ok(!a.error && !b.error,JSON.stringify({a,b}));
    for(const k of ['choice','legal','points','leader','trick','contract','inactive','review','route']) assert.deepEqual(a[k],b[k],`${f.kind}/${f.declarer}/${ply}/${k}`);
    assert.equal(a.review,'inapplicable-nello');
    for(const k of ['evaluation','fallback_evaluation']) assert.deepEqual(a[k]?.options,b[k]?.options);
    assert.ok(['forced','baseline'].includes(a.route));
    rows.push({kind:f.kind,declarer:f.declarer,ply,choice:a.choice,route:a.route,native_us:a.elapsed_us,wasm_us:b.elapsed_us,options:a.evaluation?.options});
  }
  console.log(`Matched ${f.kind}, declarer ${f.declarer}: ${f.plays.length/2} positions`);
}
let ticks=0n;
const stopped=wasm({request:request(fixtures[4],0),worlds:40,partner:true,budget_ms:1000},()=>ticks+=5000n);
assert.ok(['legal-fallback','l1-fallback'].includes(stopped.result.route));
assert.equal(stopped.result.evaluation,null);
assert.ok(stopped.checkpoints.length);
for(const c of [...stopped.checkpoints,stopped.result]) {
  assert.ok(c.legal.includes(c.choice));assert.equal(c.contract,'nello');
}
const req=request(fixtures[0],0);
for(const bad of [{...req,contract:null},{...req,decl:9},{...req,bid:30},{...req,hands:fixtures[0].hands},
  {...req,plays:fixtures[0].plays},{...req,seat:2},{...req,plays:[0,14,2,27]}]) assert.ok(wasm({request:bad}).result.error);
if(process.argv[2]) writeFileSync(process.argv[2],JSON.stringify({rows,deadline:stopped},null,2)+'\n');
console.log(`Passed ${rows.length} native/WASM Nel-O comparisons, deadline and boundary checks.`);
