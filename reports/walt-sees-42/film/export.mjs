// Export film.html to MP4: the page's own synth -> fx.wav, frames -> H.264, mixed with vo.wav.
//   node export.mjs audio            # just fx.wav
//   node export.mjs video [fps] [from] [to] [out]   # one segment of frames
import { createRequire } from 'module'; import path from 'path'; import fs from 'fs'; import { spawn } from 'child_process';
const require = createRequire(import.meta.url); const { chromium } = require('playwright');
const FF = process.env.FFMPEG || 'ffmpeg';
const [mode, fpsA, fromA, toA, outA] = process.argv.slice(2);
const browser = await chromium.launch({ executablePath: process.env.CHROMIUM, args: ['--disable-gpu-vsync'] });
const page = await browser.newPage({ viewport: { width: 1920, height: 1080 } });
page.on('pageerror', e => console.error('PAGEERROR', e.message));
await page.goto('file://' + path.resolve('film.html') + '?export');
await page.waitForFunction(() => window.FILM_READY);
if (mode === 'audio') {
  const t0 = Date.now();
  const b64 = await page.evaluate(() => {
    const { L, R, sr } = window.FILM.synthesize(48000); const n = L.length, buf = new Int16Array(n * 2);
    for (let i = 0; i < n; i++) { buf[2 * i] = Math.max(-1, Math.min(1, L[i])) * 32767; buf[2 * i + 1] = Math.max(-1, Math.min(1, R[i])) * 32767; }
    const u8 = new Uint8Array(buf.buffer); let s = ''; for (let i = 0; i < u8.length; i += 0x8000) s += String.fromCharCode.apply(null, u8.subarray(i, i + 0x8000)); return btoa(s);
  });
  const pcm = Buffer.from(b64, 'base64'), h = Buffer.alloc(44);
  h.write('RIFF', 0); h.writeUInt32LE(36 + pcm.length, 4); h.write('WAVE', 8); h.write('fmt ', 12); h.writeUInt32LE(16, 16); h.writeUInt16LE(1, 20); h.writeUInt16LE(2, 22);
  h.writeUInt32LE(48000, 24); h.writeUInt32LE(48000 * 4, 28); h.writeUInt16LE(4, 32); h.writeUInt16LE(16, 34); h.write('data', 36); h.writeUInt32LE(pcm.length, 40);
  fs.writeFileSync('fx.wav', Buffer.concat([h, pcm])); console.log('fx.wav', (Date.now() - t0) + 'ms');
} else {
  const fps = +(fpsA || 30), dur = await page.evaluate(() => window.FILM.DUR);
  const f0 = Math.round(+(fromA || 0) * fps), f1 = Math.min(Math.round(+(toA || dur) * fps), Math.round(dur * fps));
  const ff = spawn(FF, ['-y', '-loglevel', 'error', '-f', 'image2pipe', '-c:v', 'mjpeg', '-framerate', String(fps), '-i', '-',
    '-c:v', 'libx264', '-pix_fmt', 'yuv420p', '-crf', '17', '-preset', 'slow', '-tune', 'animation', outA || 'seg.mp4'], { stdio: ['pipe', 'inherit', 'inherit'] });
  for (let f = f0; f < f1; f++) {
    const d = await page.evaluate(t => { window.FILM.render(t); return document.getElementById('film').toDataURL('image/jpeg', 0.95); }, f / fps);
    if (!ff.stdin.write(Buffer.from(d.slice(d.indexOf(',') + 1), 'base64'))) await new Promise(r => ff.stdin.once('drain', r));
  }
  ff.stdin.end(); await new Promise(r => ff.on('close', r));
  console.log('frames', f0, '->', f1);
}
await browser.close();
