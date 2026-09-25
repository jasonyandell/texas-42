// Render PNG stills of film.html at given times:  node stills.mjs outdir 1.5,4,...
import { createRequire } from 'module'; import path from 'path'; import fs from 'fs';
const require = createRequire(import.meta.url); const { chromium } = require('playwright');
const [out, times] = process.argv.slice(2);
const browser = await chromium.launch({ executablePath: process.env.CHROMIUM });
const page = await browser.newPage({ viewport: { width: 1920, height: 1080 } });
page.on('pageerror', e => console.error('PAGEERROR', e.message)); page.on('console', m => m.type() === 'error' && console.error('CONSOLE', m.text()));
await page.goto('file://' + path.resolve('film.html') + '?export');
await page.waitForFunction(() => window.FILM_READY);
fs.mkdirSync(out, { recursive: true });
for (const t of times.split(',').map(Number)) {
  const d = await page.evaluate(t => { window.FILM.render(t); return document.getElementById('film').toDataURL('image/jpeg', 0.85); }, t);
  fs.writeFileSync(`${out}/s_${t.toFixed(2)}.jpg`, Buffer.from(d.split(',')[1], 'base64'));
}
await browser.close();
