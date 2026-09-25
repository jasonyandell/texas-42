// Render scene.html to an MP4, one deterministic frame at a time.
//
//   NODE_PATH=$(npm root -g) node render.mjs [out.mp4] [fps=30] [from_s] [to_s]
//   (from/to render one segment; render.sh runs segments in parallel and joins them)
//   NODE_PATH=$(npm root -g) node render.mjs --stills 10,25,60   # PNG spot checks
//
// Needs Playwright's Chromium and an ffmpeg with libx264 on PATH (or $FFMPEG).
import { createRequire } from 'module';
import { spawn } from 'child_process';
import { fileURLToPath } from 'url';
import path from 'path';

const require = createRequire(import.meta.url);
const { chromium } = require('playwright');
const here = path.dirname(fileURLToPath(import.meta.url));
const args = process.argv.slice(2);
const url = 'file://' + path.join(here, 'scene.html') + '?frames';

const browser = await chromium.launch(process.env.CHROMIUM ? { executablePath: process.env.CHROMIUM } : {});
const page = await browser.newPage({ viewport: { width: 1280, height: 720 } });
await page.goto(url);
await page.addStyleTag({ content: 'svg{width:1280px!important;height:720px!important;max-width:none!important;max-height:none!important}' });
const shot = async (t, type = 'png') => {
  await page.evaluate(t => window.render(t), t);
  return page.screenshot(type === 'png' ? { type } : { type, quality: 95 });
};

if (args[0] === '--stills') {
  const fs = await import('fs');
  for (const t of args[1].split(',').map(Number)) {
    fs.writeFileSync(path.join(args[2] || '.', `still-${t}.png`), await shot(t));
  }
} else {
  const out = args[0] || path.join(here, 'how-walt-sees-42.mp4');
  const fps = +(args[1] || 30);
  const duration = await page.evaluate(() => window.DURATION);
  const ff = spawn(process.env.FFMPEG || 'ffmpeg', ['-y', '-loglevel', 'error', '-f', 'image2pipe', '-c:v', 'mjpeg', '-framerate', String(fps),
    '-i', '-', '-c:v', 'libx264', '-pix_fmt', 'yuv420p', '-crf', '20', '-preset', 'medium', '-movflags', '+faststart', out],
    { stdio: ['pipe', 'inherit', 'inherit'] });
  const f0 = Math.round(+(args[2] || 0) * fps), f1 = Math.round(+(args[3] || duration) * fps);
  for (let f = f0; f < f1; f++) {
    const buf = await shot(f / fps, 'jpeg');
    if (!ff.stdin.write(buf)) await new Promise(r => ff.stdin.once('drain', r));
    if (f % (fps * 10) === 0) process.stderr.write(`${(f / fps).toFixed(0)}s `);
  }
  ff.stdin.end();
  await new Promise(r => ff.on('close', r));
  process.stderr.write(`\nwrote ${out}\n`);
}
await browser.close();
