import {readFileSync, writeFileSync, mkdirSync} from 'node:fs';
import {createHash} from 'node:crypto';
import assert from 'node:assert/strict';

const outDir = new URL('.', import.meta.url).pathname;
mkdirSync(outDir, {recursive: true});
const source = process.argv[2];
const worlds = Number(process.argv[3] ?? '2000');
if (!source || ![640, 2000].includes(worlds)) throw new Error('usage: node panel.mjs WASM [640|2000]');
const dir = new URL('..', import.meta.url).pathname;
const bytes = readFileSync(source);
const hash = createHash('sha256').update(bytes).digest('hex');
const module = await WebAssembly.compile(bytes), decoder = new TextDecoder();
const positions = JSON.parse(readFileSync(dir + 'positions.json')).positions;
const prior = JSON.parse(readFileSync(dir + 'panel640.json')).rows;
const tiles = Array.from({length: 7}, (_, hi) => Array.from({length: hi + 1}, (_, lo) => `${hi}${lo}`)).flat();
const rows = [];
const panelPath = outDir + `panel${worlds}.json`;

for (const pos of positions) for (const seed of [1,2,3,4,5,6,7,8]) {
  let x;
  const checkpoints = [];
  let peakWasmMemoryBytes = 0;
  x = new WebAssembly.Instance(module, {walt_host: {
    now_us: () => BigInt(Math.floor(performance.now() * 1000)),
    checkpoint: (p, n) => {
      peakWasmMemoryBytes = Math.max(peakWasmMemoryBytes, x.memory.buffer.byteLength);
      checkpoints.push(JSON.parse(decoder.decode(new Uint8Array(x.memory.buffer, p, n))));
    },
  }}).exports;
  const request = {...pos.request, seed};
  const call = {request, worlds, partner: false, budget_ms: 20000};
  const data = new TextEncoder().encode(JSON.stringify(call));
  const p = x.walt_in_prepare(data.length);
  new Uint8Array(x.memory.buffer, p, data.length).set(data);
  const started = performance.now();
  const n = x.walt_call();
  const wallMs = performance.now() - started;
  peakWasmMemoryBytes = Math.max(peakWasmMemoryBytes, x.memory.buffer.byteLength);
  const result = JSON.parse(decoder.decode(new Uint8Array(x.memory.buffer, x.walt_out_ptr(), n)));
  assert.ok(!result.error, JSON.stringify(result));
  assert.equal(result.evaluation.outer_worlds, worlds);
  assert.equal(result.route, 'baseline');
  assert.ok(result.phases.every(p => p.status === 'completed'));
  const row = {ply: pos.ply, call, result, checkpoints, timing: {
    wall_ms: wallMs,
    wasm_memory_bytes_peak: peakWasmMemoryBytes,
    solver_us: result.evaluation.solver_us,
    elapsed_us: result.elapsed_us,
  }};
  if (worlds === 640) {
    const old = prior.find(r => r.ply === pos.ply && r.call.request.seed === seed);
    assert.deepEqual(result.evaluation.options, old.result.evaluation.options);
    assert.equal(result.choice, old.result.choice);
    row.parity_with_panel640 = {options: true, choice: true};
  }
  rows.push(row);
  writeFileSync(panelPath, JSON.stringify({schema: 'nello-ruby-doubles-panel-v1', wasm_sha256: hash, worlds, rows}, null, 2) + '\n');
  console.log(JSON.stringify({ply: pos.ply, seed, worlds, choice: tiles[result.choice], wall_ms: wallMs, solver_us: result.evaluation.solver_us, peak_wasm_memory_bytes: peakWasmMemoryBytes}));
}
