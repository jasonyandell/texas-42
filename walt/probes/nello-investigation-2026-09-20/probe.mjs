// Exploratory mechanics/scope audit, not a player or a strength benchmark.
// node probe.mjs /path/to/plunge /path/to/output-directory
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { createRequire } from 'node:module';
import { resolve, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

assert.equal(process.argv.length, 4, 'pass the Plunge checkout and output directory');
const plunge = resolve(process.argv[2]), output = resolve(process.argv[3]);
mkdirSync(output, { recursive: true });
const require = createRequire(join(plunge, 'package.json'));
const esbuild = require('esbuild');
const bundle = join(output, 'plunge-probe.mjs');
await esbuild.build({
  stdin: { resolveDir: plunge, sourcefile: 'nello-audit.ts', loader: 'ts', contents: `
import assert from 'node:assert/strict';
import { CASUAL_CONFIG, PLUNGE_CONFIG, applyAction, legalActions, newGame, newDealtGame,
  mulberry32, toSeed, buildRules, legalPlays, trickWinnerIndex } from './src/engine';
import { encodeReplay, decodeReplay } from './src/engine/replay-code';
import { observe } from './src/ai/observation';
import { playRequestOf } from './src/ai/walt/requests';
import { explainScope } from './src/ai/review-request';
import { validQuestion } from './src/questions/model';

function start(dealt, declarer, config = CASUAL_CONFIG) {
  let g = newDealtGame(config, dealt, (declarer + 3) % 4);
  g = applyAction(g, {type:'bid', bid:{kind:'marks',value:1}});
  for (let i=0;i<3;i++) g = applyAction(g,{type:'bid',bid:{kind:'pass'}});
  assert(legalActions(g).some(a => a.type==='declare' && a.decl.type==='nello'));
  return applyAction(g,{type:'declare',decl:{type:'nello'}});
}
// Nominate one make and one first-trick set; freeze their complete replays.
// The search uses random legal moves, so these are mechanics witnesses only.
const found = {};
for (let i=0;i<20000 && (!found.make || !found.set);i++) {
  let g = start(newGame(CASUAL_CONFIG,'nello-audit-'+i).dealt,0);
  const rand = mulberry32(toSeed('nello-audit-play-'+i)), actions=[];
  while(g.phase==='playing') {
    const legal = legalActions(g), action = legal[Math.floor(rand()*legal.length)];
    actions.push(action); g = applyAction(g,action);
  }
  if(g.handResult.made && !found.make) found.make={seed:i,dealt:g.dealt,actions};
  if(!g.handResult.made && g.tricks.length===1 && !found.set) found.set={seed:i,dealt:g.dealt,actions};
}
assert(found.make && found.set,'find both bounded witness types');
const fixtures=[]; let roundtrips=0, observations=0;
for(const [kind,witness] of Object.entries(found)) for(let declarer=0;declarer<4;declarer++) {
  const dealt=Array.from({length:4},(_,s)=>witness.dealt[(s-declarer+4)%4]);
  let g=start(dealt,declarer);
  const inactive=(declarer+2)%4;
  for(const action of witness.actions) {
    assert.notEqual(g.turn,inactive);
    const obs=observe(g,g.turn); observations++;
    assert.equal(obs.handSizes[inactive],7);
    assert.deepEqual(obs.handSizes,g.hands.map(h=>h.length));
    assert.equal(obs.unseen.length,obs.handSizes.reduce((a,b)=>a+b,0)-obs.hand.length);
    assert.equal(playRequestOf(obs,{n:40,n0:8}),null);
    g=applyAction(g,action);
    const code=encodeReplay(g); assert(code);
    const back=decodeReplay(code); assert(back); roundtrips++;
    for(const field of ['dealt','hands','tricks','currentTrick','points','contract','sittingOut','handResult']) {
      assert.deepEqual(back[field],g[field],field);
    }
  }
  assert(g.tricks.every(t=>t.plays.length===3 && t.plays.every(p=>p.seat!==inactive)));
  assert.deepEqual(g.hands[inactive],dealt[inactive]);
  assert.equal(g.handResult.made,kind==='make');
  assert.equal(g.tricks.length,kind==='make'?7:1);
  assert.equal(explainScope(g),false);
  const replay=encodeReplay(g);
  let questionError='';
  try { validQuestion({schema:'plunge-question-v1',id:'a'.repeat(32),created:'2026-09-20T00:00:00Z',
    game_id:'nello-audit',hand_number:1,ply:0,seed:1,snapshot:replay,replay,note:'audit',alternative:null,
    receipt_id:null,receipt:null,build:'audit'}); } catch(e) { questionError=e.message; }
  assert.equal(questionError,'This play cannot be examined.');
  fixtures.push({kind,declarer,inactive,seed:witness.seed,replay,points:g.points,
    trickWinners:g.tricks.map(t=>t.winner),remaining:g.hands.map(h=>h.length),questionError});
}
const rules=buildRules({type:'nello'},CASUAL_CONFIG);
assert.deepEqual(legalPlays(['66','65'],'60',rules),['65']);
assert.deepEqual(legalPlays(['66','65'],'00',rules),['66']);
assert.equal(trickWinnerIndex([{seat:0,domino:'60'},{seat:1,domino:'66'},{seat:3,domino:'65'}],rules),2);
assert.equal(trickWinnerIndex([{seat:0,domino:'00'},{seat:1,domino:'11'},{seat:3,domino:'66'}],rules),2);
// A naive forced-30 + open-Nel-O config has no v1 replay preset today.
const proposed={...PLUNGE_CONFIG,nello:'open'};
const customStart=start(found.make.dealt,0,proposed);
const customReplay=encodeReplay(customStart);
assert(customReplay && customReplay.startsWith('v1c'));
assert.equal(decodeReplay(customReplay).config.allPass,'reshake');
export const result={fixtures,roundtrips,observations,rulesChecks:4,
  proposedConfigReplay:{intended:proposed.allPass,decoded:decodeReplay(customReplay).config.allPass}};
` },
  bundle: true, platform: 'node', format: 'esm', outfile: bundle, logLevel: 'silent',
});
const { result } = await import(pathToFileURL(bundle).href);
const manifest = JSON.parse(readFileSync(join(plunge,'src/ai/phone/manifest.json'),'utf8'));
const bytes = readFileSync(join(plunge,'src/ai/phone/walt-player.wasm'));
assert.equal(createHash('sha256').update(bytes).digest('hex'),manifest.wasm_sha256);
const module = await WebAssembly.compile(bytes);
const decoder = new TextDecoder();
function callWasm(request) {
  let x;
  const instance = new WebAssembly.Instance(module,{walt_host:{
    now_us:()=>BigInt(Math.floor(performance.now()*1000)),checkpoint:()=>{},
  }});
  x=instance.exports;
  const data=new TextEncoder().encode(JSON.stringify({request,worlds:1,partner:false,budget_ms:100}));
  const p=x.walt_in_prepare(data.length);new Uint8Array(x.memory.buffer,p,data.length).set(data);
  const n=x.walt_call();
  return JSON.parse(decoder.decode(new Uint8Array(x.memory.buffer,x.walt_out_ptr(),n)));
}
const base={decl:9,bid:30,bidder:0,seat:0,hand:[1,6,8,19,20,23,27],plays:[],seed:1};
const rejected=[
  {label:'explicit Nel-O contract is not in the wire schema',request:{...base,contract:'nello'}},
  {label:'unassigned declaration id 8 is rejected',request:{...base,decl:8}},
  {label:'one mark cannot be passed as the straight points threshold',request:{...base,bid:1}},
].map(({label,request})=>{const response=callWasm(request);assert(response.error);return {label,response};});
const importer = fileURLToPath(new URL('../../../experiments/partnership/', import.meta.url));
const pythonImporterRejections = JSON.parse(execFileSync('python3', ['-c', `
import json,sys
sys.path.insert(0,sys.argv[1])
from plunge_io import decode_hand
rows=[]
for fixture in json.load(sys.stdin):
    try:
        decode_hand(fixture['replay'])
        raise AssertionError('Nel-O unexpectedly imported')
    except ValueError as error:
        rows.append(dict(kind=fixture['kind'],declarer=fixture['declarer'],error=str(error)))
print(json.dumps(rows))
`, importer], {input:JSON.stringify(result.fixtures),encoding:'utf8',env:{...process.env,PYTHONDONTWRITEBYTECODE:'1'}}));
assert.equal(pythonImporterRejections.length,8);
const receipt={schema:'nello-investigation-v1',scope:'mechanics witnesses and current support boundaries; no strength claim',
  plungeCommit:execFileSync('git',['-C',plunge,'rev-parse','HEAD'],{encoding:'utf8'}).trim(),
  texasCommit:execFileSync('git',['-C',importer,'rev-parse','HEAD'],{encoding:'utf8'}).trim(),
  manifest,result,rejected,pythonImporterRejections};
writeFileSync(join(output,'results.json'),JSON.stringify(receipt,null,2)+'\n');
console.log(JSON.stringify({fixtures:result.fixtures.length,roundtrips:result.roundtrips,
  observations:result.observations,rulesChecks:result.rulesChecks,wasmRejections:rejected.length,
  pythonImporterRejections:pythonImporterRejections.length,
  proposedConfigReplay:result.proposedConfigReplay,output:join(output,'results.json')}));
