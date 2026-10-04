/** Synchronous Rust runs only here. Host imports carry time and checkpoints,
 * never game state, hidden hands, or policy decisions. */
import wasmUrl from './walt-player.wasm?url';

interface Exports {
  memory: WebAssembly.Memory;
  walt_in_prepare(n: number): number;
  walt_call(): number;
  walt_out_ptr(): number;
}
const ctx = self as unknown as { onmessage: (e: MessageEvent) => void; postMessage(value: unknown): void };
const decoder = new TextDecoder();
let activeId = 0, busy = false;
let loaded: Promise<Exports> | undefined;
function load(): Promise<Exports> {
  return loaded ??= (async () => {
    let exports: Exports;
    const bytes = await (await fetch(wasmUrl)).arrayBuffer();
    const wasm = await WebAssembly.instantiate(bytes, { walt_host: {
      now_us: () => BigInt(Math.floor(performance.now() * 1000)),
      checkpoint: (ptr: number, len: number) => {
        const value = JSON.parse(decoder.decode(new Uint8Array(exports.memory.buffer, ptr, len))) as unknown;
        ctx.postMessage({ id: activeId, checkpoint: value });
      },
    } });
    exports = wasm.instance.exports as unknown as Exports;
    return exports;
  })();
}
ctx.onmessage = async ({ data }: MessageEvent<{ id: number; call: unknown }>) => {
  if (busy) { ctx.postMessage({ id: data.id, error: 'Worker already has a job.' }); return; }
  busy = true; activeId = data.id;
  try {
    const exports = await load();
    const input = new TextEncoder().encode(JSON.stringify(data.call));
    const ptr = exports.walt_in_prepare(input.length);
    new Uint8Array(exports.memory.buffer, ptr, input.length).set(input);
    const len = exports.walt_call();
    const result = JSON.parse(decoder.decode(new Uint8Array(exports.memory.buffer, exports.walt_out_ptr(), len))) as unknown;
    ctx.postMessage({ id: data.id, result });
  } catch (error) { ctx.postMessage({ id: data.id, error: String(error) }); }
  finally { busy = false; }
};
