const {chromium}=require('/Users/jason/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/node_modules/playwright');
const fs=require('fs'),assert=require('node:assert/strict');
const origin=process.env.PLUNGE_ORIGIN||'http://127.0.0.1:4284';
const out=process.env.RESULT_PREFIX||'/tmp/walt-cpu-live-release/local';
const manifest=JSON.parse(fs.readFileSync('/Users/jason/code/plunge/src/ai/phone/manifest.json'));
(async()=>{
 const browser=await chromium.launch({headless:true,executablePath:'/Applications/Google Chrome.app/Contents/MacOS/Google Chrome'});
 const context=await browser.newContext({viewport:{width:412,height:915}});
 const page=await context.newPage(),errors=[];
 page.on('pageerror',e=>errors.push(e.message));
 // Exercise durable question saving without creating test questions on the server.
 await context.route('**/api/questions**',r=>r.fulfill({status:503,body:'Offline test'}));
 await page.addInitScript(()=>{
  const Original=Worker;window.__workerCalls=[];window.__workerResults=[];window.__terminated=0;
  window.Worker=class extends Original {
   constructor(...args){super(...args);this.started=performance.now();this.addEventListener('message',({data})=>{
    if(data.result)window.__workerResults.push({call:this.call,result:data.result,wall_ms:performance.now()-this.started});
   });}
   postMessage(v,...rest){this.call=v.call;window.__workerCalls.push(v.call?.auction?'auction':v.call?.request?'play':'other');return super.postMessage(v,...rest);}
   terminate(){window.__terminated++;return super.terminate();}
  };
 });
 await page.goto(origin,{waitUntil:'networkidle'});
 await page.getByRole('button',{name:'Deal me in',exact:true}).click();
 const state=()=>page.evaluate(()=>JSON.parse(localStorage.getItem('plunge:save:v1')));
 let s,rounds=0,played=false;
 for(let spin=0;spin<350;spin++){
  s=await state();const g=s?.game;if(!g){await page.waitForTimeout(100);continue;}
  if(g.phase==='bidding'&&g.turn===0)await page.getByRole('button',{name:'Pass',exact:true}).click();
  if(g.phase==='declaring'&&g.turn===0)throw Error('Human won passing auction');
  if(g.phase==='hand-over'){
   if(g.tricks.length){played=true;break;}
   if(++rounds>25)throw Error('No playable AI bid');
   await page.getByRole('button',{name:'Shake the next hand'}).click();
  }
  if(g.phase==='playing'&&g.turn===0){const tile=page.locator('.hand button:not([disabled])').first();if(await tile.count())await tile.click();}
  if(await page.getByRole('button',{name:'Retry',exact:true}).count())throw Error('Player displayed retry');
  await page.waitForTimeout(200);
 }
 assert.ok(played,'completed a hand');s=await state();const completed=s.game;
 const gameResults=await page.evaluate(()=>window.__workerResults);
 assert.ok(gameResults.some(r=>r.call.worlds===160&&r.result.evaluation?.outer_worlds===160));
 assert.ok(gameResults.some(r=>r.call.worlds===40));
 assert.equal((await page.evaluate(()=>window.__workerCalls)).includes('auction'),false);
 await page.getByRole('button',{name:'See how it went',exact:true}).click();
 await page.locator('.native-history .hist-tap').first().click();
 await page.getByText('What Walt saw when it played',{exact:true}).waitFor();
 const deeperStarted=Date.now();
 await page.getByRole('button',{name:'Think deeper',exact:true}).click();
 await page.getByText('A fresh look from Walt',{exact:true}).waitFor({timeout:25000});
 const deeper_ms=Date.now()-deeperStarted;
 await page.screenshot({path:out+'-review.png',fullPage:true});
 const results=await page.evaluate(()=>window.__workerResults);
 fs.writeFileSync(out+'-worker-results.json',JSON.stringify(results,null,2));
 const receipts=await page.evaluate(()=>new Promise((resolve,reject)=>{
  const r=indexedDB.open('plunge-walt',1);r.onerror=()=>reject(r.error);r.onsuccess=()=>{
   const q=r.result.transaction('receipts').objectStore('receipts').getAll();q.onsuccess=()=>resolve(q.result);
  };
 }));
 assert.ok(receipts.length>0);
 for(const r of receipts){assert.equal(r.identity.implementation.wasm_sha256,manifest.wasm_sha256);assert.ok(!('hands' in r.identity.request));}
 await page.getByText('Save or share this move',{exact:true}).click();
 await page.getByRole('textbox').fill('CPU release acceptance - retained locally, server upload disabled');
 await page.getByRole('button',{name:'Save this question',exact:true}).click();
 await page.waitForTimeout(300);
 fs.writeFileSync(out+'-after-question.txt',await page.locator('body').innerText());
 // Resume the saved table after question UI and reload; result remains available.
 await page.reload({waitUntil:'networkidle'});
 await page.getByRole('button',{name:'Resume your game',exact:true}).click();
 s=await state();assert.deepEqual(s.game.points,completed.points);
 await page.getByRole('button',{name:'Shake the next hand'}).click();
 await page.waitForFunction(n=>JSON.parse(localStorage.getItem('plunge:save:v1'))?.game?.handNumber===n,completed.handNumber+1);
 const final=await state();
 assert.equal(errors.length,0,errors.join(';'));
 const overflow=await page.evaluate(()=>document.documentElement.scrollWidth>innerWidth+1);assert.equal(overflow,false);
 const result={origin,completed_hand:true,points:completed.points,reshakes:rounds,
  first_opening:gameResults.find(r=>r.call.worlds===160),deeper_ms,receipt_count:receipts.length,
  source:manifest.source_commit,wasm:manifest.wasm_sha256,auction_workers:0,reload:true,
  next_hand:final.game.handNumber,question_ui:await page.locator('body').innerText(),javascript_errors:errors,overflow};
 delete result.question_ui;
 fs.writeFileSync(out+'-browser.json',JSON.stringify(result,null,2)+'\n');
 console.log(JSON.stringify({...result,first_opening_ms:result.first_opening.wall_ms,first_opening:undefined}));
 await browser.close();
})().catch(e=>{console.error(e);process.exit(1)});
