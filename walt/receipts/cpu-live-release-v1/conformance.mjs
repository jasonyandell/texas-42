import {readFileSync,writeFileSync} from 'node:fs';
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
const [before,after,fixtures,output]=process.argv.slice(2);
const buffers=[before,after].map(p=>readFileSync(p));
const modules=await Promise.all(buffers.map(b=>WebAssembly.compile(b)));
const rows=[];
function solve(module,call) {
 const started=performance.now();
 const x=new WebAssembly.Instance(module,{walt_host:{now_us:()=>0n,checkpoint:()=>{}}}).exports;
 const input=new TextEncoder().encode(JSON.stringify(call));
 const p=x.walt_in_prepare(input.length);new Uint8Array(x.memory.buffer,p,input.length).set(input);
 const n=x.walt_call();
 const value=JSON.parse(new TextDecoder().decode(new Uint8Array(x.memory.buffer,x.walt_out_ptr(),n)));
 assert.ok(!value.error,JSON.stringify(value));
 return {value,ms:performance.now()-started,memory_bytes:x.memory.buffer.byteLength};
}
const cases=[];
for (const [name,bid] of [['g1',30],['decl-0',30],['decl-7',36],['decl-9',42]]) {
 const game=JSON.parse(readFileSync(`${fixtures}/${name}-baseline.json`)).games[0];
 assert.equal(game.decisions.length,28);
 for(const d of game.decisions)cases.push({name,ply:d.play,call:{request:{...d.request,bid},
   worlds:d.play===1?160:40,partner:d.play!==1,budget_ms:d.play===1?20000:14000}});
}
function semantic(v) {
 const pick=(x,keys)=>x==null?x:Object.fromEntries(keys.map(k=>[k,x[k]]));
 return {...pick(v,['choice','legal','points','leader','trick','route']),
 evaluation:pick(v.evaluation,['choice','legal','options']),
 fallback:pick(v.fallback_evaluation,['choice','legal','options']),
 review:pick(v.review_result,['status','choice','values','paired','samples','support','coverage'])};
}
const save=()=>writeFileSync(output,JSON.stringify({schema:'walt-release-trajectory-parity-v1',
 before: createHash('sha256').update(buffers[0]).digest('hex'),
 after:createHash('sha256').update(buffers[1]).digest('hex'),
 host:'Node WASM, frozen clock; not Pixel timing',
 scope:'Four complete legal reference trajectories; modified targets on doubles/no-trump. Each call compares actual live wrapper outputs, not reference L2 outputs.',
 requested:cases.length,completed:rows.length,all_completed:rows.length===cases.length,rows},null,2)+'\n');
for(const c of cases) {
 const got=[];
 for(const i of rows.length%2?[1,0]:[0,1])got[i]=solve(modules[i],c.call);
 try {assert.deepEqual(semantic(got[0].value),semantic(got[1].value));}
 catch(e) {writeFileSync(output+'.failure.json',JSON.stringify({c,got},null,2));throw e;}
 rows.push({...c,choice:got[0].value.choice,route:got[0].value.route,
 before_ms:got[0].ms,after_ms:got[1].ms,before_memory:got[0].memory_bytes,after_memory:got[1].memory_bytes});
 save();
 if(c.ply===28)console.log(`All 28 positions passed: ${c.name}`);
}
console.log(`Exact live decisions and completed reviews agree: ${rows.length} positions`);
