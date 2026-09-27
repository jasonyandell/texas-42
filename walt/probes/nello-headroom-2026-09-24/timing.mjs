/** Exploratory latency only. Own/public calls, real WASM clock, fresh instance per move. */
import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
const [asset,output,profile='before']=process.argv.slice(2);
const bytes=readFileSync(asset), module=await WebAssembly.compile(bytes);
const fixtures=JSON.parse(readFileSync(new URL('../../walt-player/tests/fixtures/nello.json',import.meta.url)));
const ruby=JSON.parse(readFileSync(new URL('../nello-counterexample-2026-09-22/panel-v2/inputs/ply6-n160-seed1.json',import.meta.url))).request;
const cases=[
 {name:'Ruby first trick',request:{...ruby,plays:ruby.plays.slice(0,4)}},
 ...[1,2].map(seed=>({name:`early free defender seed ${seed}`,request:{contract:'nello',decl:8,bid:1,bidder:0,seat:1,hand:fixtures[0].hands[1],plays:[0,12],seed}})),
 {name:'Ruby double-six lead',request:ruby},
 {name:'Ruby double-four lead',request:{...ruby,plays:[...ruby.plays,3,27,0,9,1,2]}}
];
const rows=[];
const save=()=>writeFileSync(output,JSON.stringify({schema:'nello-headroom-timing-v1',host:'Node WASM on this Mac; not phone timing or playing strength',profile,sha256:createHash('sha256').update(bytes).digest('hex'),rows},null,2)+'\n');
for(const c of cases) for(const worlds of profile==='before'?[40,160]:[40,500]) {
 let x;const decoder=new TextDecoder();const start=performance.now();
 const instance=new WebAssembly.Instance(module,{walt_host:{now_us:()=>BigInt(Math.floor(performance.now()*1000)),checkpoint:()=>{}}}); x=instance.exports;
 const call={request:c.request,worlds,partner:false,nello_counterexamples:true,budget_ms:worlds===40?14000:20000};
 const input=new TextEncoder().encode(JSON.stringify(call));const ptr=x.walt_in_prepare(input.length);
 new Uint8Array(x.memory.buffer,ptr,input.length).set(input);const len=x.walt_call();
 const result=JSON.parse(decoder.decode(new Uint8Array(x.memory.buffer,x.walt_out_ptr(),len)));
 if(result.error)throw new Error(JSON.stringify(result));
 const row={name:c.name,call,ms:Math.round(performance.now()-start),memory_mib:x.memory.buffer.byteLength/1048576,result};rows.push(row);save();
 console.log(JSON.stringify({name:row.name,worlds,ms:row.ms,memory_mib:row.memory_mib,completed:result.evaluation?.outer_worlds,rounds:result.counterexample_result?.rounds,witnesses:result.counterexample_result?.witnesses,stop:result.counterexample_result?.stop}));
}
