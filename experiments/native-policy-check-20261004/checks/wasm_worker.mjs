// Independent reviewer host. One fresh instance per request; compiled module reused.
import fs from 'node:fs';
import readline from 'node:readline';
import {performance} from 'node:perf_hooks';
const mod = new WebAssembly.Module(fs.readFileSync(process.argv[2]));
const enc=new TextEncoder(),dec=new TextDecoder();
for await (const text of readline.createInterface({input:process.stdin,crlfDelay:Infinity})) {
 try {
  let ex;const checkpoints=[];let ticks=0;
  const inst=new WebAssembly.Instance(mod,{walt_host:{now_us:()=>process.argv[3]==='expire'?BigInt(++ticks*100000):BigInt(Math.floor(performance.now()*1000)),checkpoint:(p,n)=>{checkpoints.push(JSON.parse(dec.decode(new Uint8Array(ex.memory.buffer,p,n))));}}});ex=inst.exports;
  const bytes=enc.encode(text);const p=ex.native_late_in_prepare(bytes.length);new Uint8Array(ex.memory.buffer,p,bytes.length).set(bytes);
  const len=ex.native_late_call();const result=JSON.parse(dec.decode(new Uint8Array(ex.memory.buffer,ex.native_late_out_ptr(),len)));
  process.stdout.write(JSON.stringify({result,checkpoints})+'\n');
 }catch(e){process.stdout.write(JSON.stringify({result:{error:String(e)}})+'\n');}
}
