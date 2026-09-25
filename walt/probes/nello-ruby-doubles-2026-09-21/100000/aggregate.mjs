import {readdirSync, readFileSync, writeFileSync} from 'node:fs';
import {join} from 'node:path';

const dir = new URL('./cases/', import.meta.url).pathname;
const names = readdirSync(dir).filter(name => /^ply(?:6|9)-seed[1-8]\.json$/.test(name)).sort((a, b) => {
  const pa = Number(a.match(/^ply(\d+)/)[1]), pb = Number(b.match(/^ply(\d+)/)[1]);
  const sa = Number(a.match(/seed(\d+)/)[1]), sb = Number(b.match(/seed(\d+)/)[1]);
  return pa - pb || sa - sb;
});
if (names.length !== 16) throw new Error(`expected 16 complete case files, found ${names.length}`);
const rows = names.map(name => {
  const x = JSON.parse(readFileSync(join(dir, name)));
  if (x.result?.evaluation?.outer_worlds !== 100000 || x.result?.evaluation?.outer_draw_attempts !== 100000 || !x.result?.phases?.every(p => p.status === 'completed')) throw new Error(`incomplete ${name}`);
  return {ply: x.ply, call: x.call, result: x.result, checkpoints: x.checkpoints, timing: x.timing, host_observation: x.host_observation, case_file: `cases/${name}`};
});
writeFileSync(new URL('./panel100000.json', import.meta.url), JSON.stringify({schema: 'nello-ruby-doubles-panel-v1', wasm_sha256: rows[0].result ? JSON.parse(readFileSync(join(dir, names[0]))).wasm_sha256 : null, worlds: 100000, rows}, null, 2) + '\n');
console.log(JSON.stringify({rows: rows.length, outer_worlds: rows.every(r => r.result.evaluation.outer_worlds === 100000), phases_complete: rows.every(r => r.result.phases.every(p => p.status === 'completed'))}));
