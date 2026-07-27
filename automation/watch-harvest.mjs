// Wedge-proof watcher: ONE tab on ONE conversation, completion detected by
// observing the live DOM in place (no navigation), reload at most once per
// 10 min as a backstop, and — critically — the harvest happens HERE, in this
// same process, the moment completion is seen (see SKILL.md WEDGE LESSON).
// Usage: node watch-harvest.mjs <NNN-slug.submitted.json> [timeout-hours]
// Exit 0 = harvested (inbox file written), 3 = timed out, 1 = error.
import { connect, openPage, shot, log, ROOT } from './lib.mjs';
import fs from 'node:fs';
import path from 'node:path';

const metaFile = process.argv[2];
const timeoutH = parseFloat(process.argv[3] || '3');
const meta = JSON.parse(fs.readFileSync(metaFile, 'utf8'));
const tag = meta.tag;
const outFile = path.join(ROOT, 'exchange', 'inbox', `${tag}.md`);
const deadline = new Date(meta.submittedAt).getTime() + timeoutH * 3600 * 1000;

const { browser, context } = await connect();
await context.grantPermissions(['clipboard-read', 'clipboard-write'], { origin: 'https://chatgpt.com' }).catch(() => {});
const page = await openPage(context, meta.conversationUrl);
let lastReload = Date.now();
let lastLen = -1;

async function state() {
  return page.evaluate(() => {
    const asst = document.querySelectorAll('[data-message-author-role="assistant"]');
    const last = asst[asst.length - 1];
    const turn = last?.closest('article') || last?.parentElement?.parentElement;
    return {
      assistantCount: asst.length,
      lastLen: last?.innerText.length ?? 0,
      stop: !!document.querySelector('[data-testid="stop-button"], button[aria-label*="Stop"]'),
      copyBtnOnLast: !!(turn?.querySelector('[data-testid="copy-turn-action-button"]')),
    };
  });
}

try {
  while (Date.now() < deadline) {
    const s = await state().catch(e => ({ err: e.message }));
    if (s.err) {
      log(`${tag}: watch eval error (${s.err}); reloading`);
      await page.reload({ waitUntil: 'domcontentloaded' }).catch(() => {});
      await page.waitForTimeout(8000);
      lastReload = Date.now();
    } else {
      const complete = s.assistantCount > meta.baselineAssistantCount && !s.stop && s.copyBtnOnLast;
      if (complete) {
        log(`${tag}: COMPLETE detected in-place ${JSON.stringify(s)}; harvesting NOW in-process`);
        await page.evaluate(() => {
          const a = document.querySelectorAll('[data-message-author-role="assistant"]');
          a[a.length - 1]?.scrollIntoView({ block: 'end' });
        });
        await page.waitForTimeout(1000);
        let text = null, method = 'copy-button', caveats = [];
        const copied = await page.evaluate(async () => {
          await navigator.clipboard.writeText('SENTINEL-EMPTY');
          const a = document.querySelectorAll('[data-message-author-role="assistant"]');
          const last = a[a.length - 1];
          const turn = last?.closest('article') || last?.parentElement?.parentElement;
          const btn = turn?.querySelector('[data-testid="copy-turn-action-button"]');
          if (!btn) return null;
          btn.click();
          await new Promise(r => setTimeout(r, 1500));
          return navigator.clipboard.readText();
        }).catch(() => null);
        if (copied && copied !== 'SENTINEL-EMPTY') text = copied;
        if (!text) {
          text = await page.evaluate(() => {
            const a = document.querySelectorAll('[data-message-author-role="assistant"]');
            return a[a.length - 1]?.innerText ?? null;
          });
          method = 'innerText';
          caveats.push('copy-button unavailable; DOM innerText (markdown may be lossy)');
        }
        if (!text) throw new Error('completion detected but no message text extracted');
        await shot(page, `${tag}-final`);
        const header = [
          '---', `number: ${meta.number}`, `slug: ${meta.slug}`,
          `conversation: ${meta.conversationUrl}`,
          `submitted-at: ${meta.submittedAt}`,
          `harvested-at: ${new Date().toISOString()}`,
          `extraction: ${method}`,
          caveats.length ? `caveats: ${caveats.join('; ')}` : 'caveats: none',
          '---', '',
        ].join('\n');
        fs.writeFileSync(outFile, header + text + '\n');
        log(`${tag}: HARVESTED ${text.length} chars via ${method} -> ${outFile}`);
        console.log(`HARVESTED: ${tag}`);
        process.exit(0);
      }
      if (s.lastLen !== lastLen) {
        log(`${tag}: progress lastLen=${s.lastLen} stop=${s.stop}`);
        lastLen = s.lastLen;
        lastReload = Date.now(); // content moving; no backstop reload needed
      } else if (Date.now() - lastReload > 600000) {
        log(`${tag}: no change in 10 min; backstop reload`);
        await page.reload({ waitUntil: 'domcontentloaded' }).catch(() => {});
        await page.waitForTimeout(8000);
        lastReload = Date.now();
      }
    }
    await page.waitForTimeout(30000);
  }
  log(`${tag}: TIMEOUT after ${timeoutH}h without completion`);
  await shot(page, `${tag}-timeout`);
  console.log(`TIMEOUT: ${tag}`);
  process.exit(3);
} finally {
  await browser.close();
}
