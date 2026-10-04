// The phone's exact ABI and host clock, with a fresh instance for each decision.
// Compiled-module startup is reused; Node timings are not phone-device timings.
import { readFile } from 'node:fs/promises';
import { createInterface } from 'node:readline';
import { performance } from 'node:perf_hooks';
const started = performance.now();
const module = await WebAssembly.compile(await readFile(process.argv[2]));
const initializationMs = performance.now() - started;
const encoder = new TextEncoder(), decoder = new TextDecoder();
for await (const line of createInterface({ input: process.stdin, crlfDelay: Infinity })) {
  if (!line.trim()) continue;
  try {
    let exports;
    const instance = await WebAssembly.instantiate(module, { walt_host: {
      now_us: () => BigInt(Math.floor(performance.now() * 1000)),
      checkpoint: (ptr, len) => process.stdout.write(JSON.stringify({checkpoint:
        JSON.parse(decoder.decode(new Uint8Array(exports.memory.buffer, ptr, len)))}) + '\n'),
    }});
    exports = instance.exports;
    const input = encoder.encode(line), ptr = exports.walt_in_prepare(input.length);
    new Uint8Array(exports.memory.buffer, ptr, input.length).set(input);
    const len = exports.walt_call();
    const result = JSON.parse(decoder.decode(new Uint8Array(exports.memory.buffer, exports.walt_out_ptr(), len)));
    process.stdout.write(JSON.stringify({result, initialization_ms: initializationMs}) + '\n');
  } catch (error) {
    process.stdout.write(JSON.stringify({error: String(error)}) + '\n');
  }
}
