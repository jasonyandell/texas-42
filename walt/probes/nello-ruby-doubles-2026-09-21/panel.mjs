import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import assert from 'node:assert/strict';
const dir='/private/tmp/nello-ruby-doubles-20260921';
const source='/Users/jason/.codex/worktrees/nello-player/plunge/src/ai/phone/';
const bytes=readFileSync(source+'walt-player.wasm');
const hash=createHash('sha256').update(bytes).digest('hex');
assert.equal(hash,'fe22d2d24c33e98327799e2e08972481b4e1ec4e48f40256b22b32d0ccd76398');
const module=await WebAssembly.compile(bytes),decoder=new TextDecoder();
const positions=JSON.parse(readFileSync(dir+'/positions.json')).positions;
const tiles=Array.from({length:7},(_,hi)=>Array.from({length:hi+1},(_,lo)=>`${hi}${lo}`)).flat();
const rows=[];
for(const pos of positions) for(const seed of [1,2,3,4,5,6,7,8]) {
  let x;const checkpoints=[];
  x=new WebAssembly.Instance(module,{walt_host:{now_us:()=>BigInt(Math.floor(performance.now()*1000)),checkpoint:(p,n)=>checkpoints.push(JSON.parse(decoder.decode(new Uint8Array(x.memory.buffer,p,n))))}}).exports;
  const call={request:{...pos.request,seed},worlds:160,partner:false,budget_ms:20000};
  const data=new TextEncoder().encode(JSON.stringify(call)),p=x.walt_in_prepare(data.length);
  new Uint8Array(x.memory.buffer,p,data.length).set(data);
  const n=x.walt_call();
  const result=JSON.parse(decoder.decode(new Uint8Array(x.memory.buffer,x.walt_out_ptr(),n)));
  assert.ok(!result.error,JSON.stringify(result));
  rows.push({ply:pos.ply,call,result,checkpoints});
  writeFileSync(dir+'/panel.json',JSON.stringify({wasm_sha256:hash,rows},null,2)+'\n');
  const comparisons=checkpoints.filter(c=>c.phases.at(-1).name==='baseline' && c.phases.at(-1).status==='completed').map(c=>({n:c.phases.at(-1).worlds,choice:tiles[c.choice],options:c.evaluation.options.map(([t,a,b])=>[tiles[t],`${BigInt(b)-BigInt(a)}/${b}`])}));
  console.log(JSON.stringify({ply:pos.ply,seed,comparisons,phases:result.phases,elapsed_us:result.elapsed_us}));
}
