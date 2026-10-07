const E = require('./engine.js');
const fs=require('fs');
const G = E.makeGame(5, 30, 3);
const handT=[6,14,15,16,17,21,26];
const hand = handT.reduce((m,t)=>m|1<<t,0);
const SEED=2;
const res = E.level1(G, 3, hand, G.start, 40, 8, SEED);
const out={decl:5,bid:30,bidder:3,viewer:3,hand:handT,seed:SEED,n:40,n0:8,
 worlds:res.worlds.map(w=>w.map(m=>E.bits(m))),
 cands:res.cands, counts:res.counts,
 made:res.masks.map(m=>{const a=[];for(let i=0;i<40;i++)a.push(Number((m>>BigInt(i))&1n));return a;}),
 traces:[], nodes:res.solver.nodes, calls:res.L0.calls()};
for(const c of res.cands){ const row=[]; for(let w=0;w<40;w++){ const tr=E.trace(G,res.solver,res.field,res.worlds,3,G.start,c,w); row.push({t:tr.steps.map(s=>s.tile),m:tr.made?1:0,b:[tr.end.b0,tr.end.b1]}); } out.traces.push(row); }
// consistency check: trace made == mask
let bad=0; out.traces.forEach((row,j)=>row.forEach((tr,w)=>{if(tr.m!==out.made[j][w])bad++;})); console.log('mismatch',bad);
fs.writeFileSync('data.json',JSON.stringify(out));
console.log(fs.statSync('data.json').size);
