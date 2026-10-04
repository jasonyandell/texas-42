// Reviewer-local host: fresh instance; synchronous borrowed-buffer copies.
import fs from 'node:fs';
import readline from 'node:readline';
import {performance} from 'node:perf_hooks';
const module=new WebAssembly.Module(fs.readFileSync(process.argv[2]));
const encoder=new TextEncoder(),decoder=new TextDecoder();
for await (const text of readline.createInterface({input:process.stdin,crlfDelay:Infinity})) {
  try {
    let ex;let ticks=0;const checkpoints=[];
    const instance=new WebAssembly.Instance(module,{walt_host:{
      now_us:()=>process.argv[3]==='expire'?BigInt(++ticks*100000):BigInt(Math.floor(performance.now()*1000)),
      checkpoint:(ptr,len)=>checkpoints.push(JSON.parse(decoder.decode(new Uint8Array(ex.memory.buffer,ptr,len))))
    }});ex=instance.exports;
    const own=typeof ex.native_late_call==='function';
    const prepare=own?ex.native_late_in_prepare:ex.walt_in_prepare;
    const execute=own?ex.native_late_call:ex.walt_call;
    const output=own?ex.native_late_out_ptr:ex.walt_out_ptr;
    const bytes=encoder.encode(text);const ptr=prepare(bytes.length);
    new Uint8Array(ex.memory.buffer,ptr,bytes.length).set(bytes);
    const len=execute();const result=JSON.parse(decoder.decode(new Uint8Array(ex.memory.buffer,output(),len)));
    process.stdout.write(JSON.stringify({result,checkpoints})+'\n');
  }catch(error){process.stdout.write(JSON.stringify({result:{error:String(error)}})+'\n');}
}
