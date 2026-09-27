import {readFileSync, writeFileSync, mkdirSync} from 'node:fs';
import {dirname} from 'node:path';
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';

const source = process.argv[2];
const ply = Number(process.argv[3]);
const seed = Number(process.argv[4]);
const worlds = Number(process.argv[5] ?? '100000');
const outPath = process.argv[6];
const budgetMs = Number(process.argv[7] ?? '240000');
if (!source || ![6, 9].includes(ply) || ![1,2,3,4,5,6,7,8].includes(seed) || ![10000, 100000].includes(worlds) || !outPath) {
  throw new Error('usage: node case.mjs WASM PLY SEED WORLDS OUTPUT_JSON [BUDGET_MS]');
}
const here = new URL('.', import.meta.url).pathname;
const root = new URL('..', import.meta.url).pathname;
const positions = JSON.parse(readFileSync(root + 'positions.json')).positions;
const pos = positions.find(p => p.ply === ply);
if (!pos) throw new Error(`unknown ply ${ply}`);
const bytes = readFileSync(source);
const wasmSha256 = createHash('sha256').update(bytes).digest('hex');
const module = await WebAssembly.compile(bytes), decoder = new TextDecoder();
let x;
const checkpoints = [];
let peakWasmMemoryBytes = 0;
let peakRssBytes = process.memoryUsage().rss;
const rss = () => {
  peakRssBytes = Math.max(peakRssBytes, process.memoryUsage().rss);
  return process.memoryUsage().rss;
};
const pressure = () => {
  try {
    return execFileSync('/usr/bin/memory_pressure', ['-Q'], {encoding: 'utf8', timeout: 3000}).trim();
  } catch {
    return null;
  }
};
const rssStart = rss();
const pressureStart = pressure();
x = new WebAssembly.Instance(module, {walt_host: {
  now_us: () => BigInt(Math.floor(performance.now() * 1000)),
  checkpoint: (p, n) => {
    peakWasmMemoryBytes = Math.max(peakWasmMemoryBytes, x.memory.buffer.byteLength);
    rss();
    checkpoints.push(JSON.parse(decoder.decode(new Uint8Array(x.memory.buffer, p, n))));
  },
}}).exports;
const request = {...pos.request, seed};
const call = {request, worlds, partner: false, budget_ms: budgetMs};
const data = new TextEncoder().encode(JSON.stringify(call));
const inputPtr = x.walt_in_prepare(data.length);
new Uint8Array(x.memory.buffer, inputPtr, data.length).set(data);
const started = performance.now();
const n = x.walt_call();
const wallMs = performance.now() - started;
peakWasmMemoryBytes = Math.max(peakWasmMemoryBytes, x.memory.buffer.byteLength);
const rssEnd = rss();
const pressureEnd = pressure();
const result = JSON.parse(decoder.decode(new Uint8Array(x.memory.buffer, x.walt_out_ptr(), n)));
const artifact = {
  schema: 'nello-ruby-doubles-case-v1',
  ply, seed, wasm_sha256: wasmSha256, call, result, checkpoints,
  timing: {
    wall_ms: wallMs,
    elapsed_us: result.elapsed_us ?? null,
    solver_us: result.evaluation?.solver_us ?? null,
    wasm_memory_bytes_peak: peakWasmMemoryBytes,
    rss_bytes_start: rssStart,
    rss_bytes_end: rssEnd,
    rss_bytes_peak_observed: peakRssBytes,
    resource_usage_max_rss_raw: process.resourceUsage().maxRSS,
    resource_usage_max_rss_bytes_estimate: process.resourceUsage().maxRSS * 1024,
  },
  host_observation: {memory_pressure_start: pressureStart, memory_pressure_end: pressureEnd},
};
mkdirSync(dirname(outPath), {recursive: true});
writeFileSync(outPath, JSON.stringify(artifact, null, 2) + '\n');
console.log(JSON.stringify({ply, seed, worlds: result.evaluation?.outer_worlds ?? null,
  draw_attempts: result.evaluation?.outer_draw_attempts ?? null,
  route: result.route ?? null, phase_statuses: result.phases ?? null,
  wall_ms: wallMs, solver_us: result.evaluation?.solver_us ?? null,
  wasm_memory_bytes_peak: peakWasmMemoryBytes,
  rss_bytes_peak_observed: peakRssBytes}));
if (result.error || result.evaluation?.outer_worlds !== worlds || result.evaluation?.outer_draw_attempts !== worlds || !result.phases?.every(p => p.status === 'completed')) process.exitCode = 2;
