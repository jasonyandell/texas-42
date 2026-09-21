const {chromium}=require('/Users/jason/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/node_modules/playwright');
const fs=require('fs'),assert=require('node:assert/strict');
const origin=process.env.PLUNGE_ORIGIN||'http://127.0.0.1:4284',out=process.env.RESULT_PREFIX||'/tmp/walt-cpu-live-release/local';
const manifest=JSON.parse(fs.readFileSync('/Users/jason/code/plunge/src/ai/phone/manifest.json'));
(async()=>{
 const browser=await chromium.launch({headless:true,executablePath:'/Applications/Google Chrome.app/Contents/MacOS/Google Chrome'});
 const context=await browser.newContext({viewport:{width:412,height:915}}),page=await context.newPage();
 await page.goto(origin,{waitUntil:'networkidle'});
 const cdp=await context.newCDPSession(page);await cdp.send('Emulation.setCPUThrottlingRate',{rate:4});
 const wasm='/assets/'+fs.readdirSync('/Users/jason/code/plunge/dist/assets').find(x=>x.endsWith('.wasm'));
 const worker='/assets/'+fs.readdirSync('/Users/jason/code/plunge/dist/assets').find(x=>/^worker-.*\.js$/.test(x));
 const edges=await page.evaluate(async({wasm,worker,expected})=>{
  const bytes=await (await fetch(wasm)).arrayBuffer();
  const digest=[...new Uint8Array(await crypto.subtle.digest('SHA-256',bytes))].map(x=>x.toString(16).padStart(2,'0')).join('');
  if(digest!==expected)throw Error('Served asset mismatch');
  const started=performance.now(),module=await WebAssembly.compile(bytes),compile_ms=performance.now()-started;
  const request={decl:6,bid:30,bidder:0,seat:0,hand:[1,6,8,19,20,23,27],plays:[],seed:420914};
  function solve(call,clock) {
   let x;const checkpoints=[];
   x=new WebAssembly.Instance(module,{walt_host:{now_us:clock,checkpoint:(p,n)=>checkpoints.push(JSON.parse(new TextDecoder().decode(new Uint8Array(x.memory.buffer,p,n))))}}).exports;
   const initial=x.memory.buffer.byteLength,input=new TextEncoder().encode(JSON.stringify(call)),p=x.walt_in_prepare(input.length);
   new Uint8Array(x.memory.buffer,p,input.length).set(input);const n=x.walt_call();
   return {result:JSON.parse(new TextDecoder().decode(new Uint8Array(x.memory.buffer,x.walt_out_ptr(),n))),checkpoints,initial_memory:initial,peak_linear_memory:x.memory.buffer.byteLength};
  }
  let ticks=0n;const forced=solve({request,worlds:160,partner:false,budget_ms:1000},()=>ticks+=5000n);
  if(!forced.checkpoints.length||!forced.result.legal.includes(forced.result.choice)||forced.result.evaluation!==null)throw Error('Deadline did not retain only complete fallback');
  const wall=performance.now(),live=solve({request,worlds:160,partner:false,budget_ms:20000},()=>BigInt(Math.floor(performance.now()*1000)));
  const live_ms=performance.now()-wall;
  if(live.result.evaluation?.outer_worlds!==160)throw Error('Deeper result incomplete');
  // Terminate the actual shipped worker after a real checkpoint, while a deeper
  // job is active. Confirm no late result arrives after termination.
  const cancellation=await new Promise((resolve,reject)=>{
   const w=new Worker(worker,{type:'module'});let stopped=false,late=0;
   const timer=setTimeout(()=>{w.terminate();reject(Error('No cancellation checkpoint'));},5000);
   w.onerror=e=>{clearTimeout(timer);reject(Error(e.message));};
   w.onmessage=({data})=>{
    if(stopped){late++;return;}
    if(data.checkpoint){stopped=true;w.terminate();clearTimeout(timer);setTimeout(()=>resolve({checkpoint:true,late_messages:late}),200);}
   };
   w.postMessage({id:0,call:{request,worlds:160,partner:false,budget_ms:20000}});
  });
  return {wasm_sha256:digest,compile_ms,forced_deadline:forced.result.route,checkpoint_count:forced.checkpoints.length,
          live_ms,completed_worlds:live.result.evaluation.outer_worlds,initial_memory:live.initial_memory,
          peak_linear_memory:live.peak_linear_memory,cancellation};
 },{wasm,worker,expected:manifest.wasm_sha256});
 assert.equal(edges.cancellation.late_messages,0);
 const old={v:2,hand:'v1t366615143403021656463533231106250423320110060555452444122.P303132D9222132624440644255515350',ply:0,seed:2642199461,note:'Old pre-bid-book saved question',alternative:null,receipt:null};
 await page.goto(origin+'/#q='+encodeURIComponent(JSON.stringify(old)),{waitUntil:'networkidle'});
 await page.getByRole('dialog',{name:'Hand review'}).waitFor();
 await page.getByRole('button',{name:'Think deeper',exact:true}).click();
 await page.getByText('A fresh look from Walt',{exact:true}).waitFor({timeout:25000});
 const result={...edges,browser_cpu_throttle:4,old_shared_link:true,old_link_deeper:true,scope:'Mac Chrome with renderer CPU throttling; not a physical phone measurement'};
 fs.writeFileSync(out+'-edges.json',JSON.stringify(result,null,2)+'\n');console.log(JSON.stringify(result));
 await browser.close();
})().catch(e=>{console.error(e);process.exit(1)});
