// Exact production host clock/checkpoint contract; fresh instance per call.
// New experimental ABI names coexist with linked unchanged walt-player ABI.
import {readFile} from 'node:fs/promises';
import {createInterface} from 'node:readline';
import {performance} from 'node:perf_hooks';
const start=performance.now();
const module=await WebAssembly.compile(await readFile(process.argv[2]));
const initializationMs=performance.now()-start;
const encoder=new TextEncoder(), decoder=new TextDecoder();
for await (const line of createInterface({input:process.stdin,crlfDelay:Infinity})) {
  if (!line.trim()) continue;
  try {
    let exports;
    const instance=await WebAssembly.instantiate(module,{walt_host:{
      now_us:()=>BigInt(Math.floor(performance.now()*1000)),
      checkpoint:(ptr,len)=>process.stdout.write(JSON.stringify({checkpoint:JSON.parse(decoder.decode(new Uint8Array(exports.memory.buffer,ptr,len)))})+'\n'),
    }});
    exports=instance.exports;
    const own=typeof exports.native_late_call==='function';
    const prepare=own?exports.native_late_in_prepare:exports.walt_in_prepare;
    const call=own?exports.native_late_call:exports.walt_call;
    const out=own?exports.native_late_out_ptr:exports.walt_out_ptr;
    const input=encoder.encode(line),ptr=prepare(input.length);
    new Uint8Array(exports.memory.buffer,ptr,input.length).set(input);
    const len=call();
    const result=JSON.parse(decoder.decode(new Uint8Array(exports.memory.buffer,out(),len)));
    process.stdout.write(JSON.stringify({result,initialization_ms:initializationMs,wasm_memory_bytes:exports.memory.buffer.byteLength,experimental_abi:own})+'\n');
  } catch(error) {process.stdout.write(JSON.stringify({error:String(error)})+'\n');}
}
