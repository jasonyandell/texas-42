// Harvest the final assistant message of a completed conversation.
// Usage: node harvest.mjs <NNN-slug.submitted.json>
// Writes exchange/inbox/NNN-slug.md and automation/logs/NNN-final.png.
// Prefers the copy button (markdown source via clipboard); falls back to
// DOM innerText with a caveat note.
import { connect, openPage, shot, log, ROOT } from './lib.mjs';
import fs from 'node:fs';
import path from 'node:path';

const metaFile = process.argv[2];
const meta = JSON.parse(fs.readFileSync(metaFile, 'utf8'));
const tag = `${meta.number}-${meta.slug}`;
const outFile = path.join(ROOT, 'exchange', 'inbox', `${tag}.md`);

const { browser, context } = await connect();
try {
  await context.grantPermissions(['clipboard-read', 'clipboard-write'], { origin: 'https://chatgpt.com' }).catch(() => {});
  const page = await openPage(context, meta.conversationUrl);
  await page.waitForTimeout(6000);

  let text = null, method = null, caveats = [];

  // via copy button on the last assistant turn
  try {
    await page.evaluate(() => {
      const asst = document.querySelectorAll('[data-message-author-role="assistant"]');
      asst[asst.length - 1]?.scrollIntoView({ block: 'end' });
    });
    await page.waitForTimeout(1000);
    const copied = await page.evaluate(async () => {
      await navigator.clipboard.writeText('SENTINEL-EMPTY');
      const asst = document.querySelectorAll('[data-message-author-role="assistant"]');
      const last = asst[asst.length - 1];
      const turn = last?.closest('article') || last?.parentElement?.parentElement;
      const btn = turn?.querySelector('[data-testid="copy-turn-action-button"]');
      if (!btn) return null;
      btn.click();
      await new Promise(r => setTimeout(r, 1500));
      return navigator.clipboard.readText();
    });
    if (copied && copied !== 'SENTINEL-EMPTY') { text = copied; method = 'copy-button'; }
  } catch (e) { caveats.push(`copy-button failed: ${e.message}`); }

  if (!text) {
    text = await page.evaluate(() => {
      const asst = document.querySelectorAll('[data-message-author-role="assistant"]');
      return asst[asst.length - 1]?.innerText ?? null;
    });
    method = 'innerText';
    caveats.push('extracted via DOM innerText (markdown formatting may be lossy)');
  }
  if (!text) throw new Error(`${tag}: no assistant message found`);

  await shot(page, `${tag}-final`);
  const header = [
    '---',
    `number: ${meta.number}`,
    `slug: ${meta.slug}`,
    `conversation: ${meta.conversationUrl}`,
    `submitted-at: ${meta.submittedAt}`,
    `harvested-at: ${new Date().toISOString()}`,
    `extraction: ${method}`,
    caveats.length ? `caveats: ${caveats.join('; ')}` : 'caveats: none',
    '---',
    '',
  ].join('\n');
  fs.writeFileSync(outFile, header + text + '\n');
  log(`${tag}: HARVESTED ${text.length} chars via ${method} -> ${outFile}`);
  await page.close();
} finally {
  await browser.close();
}
