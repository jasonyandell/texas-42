// One-off: complete the 001 send on the tab already prepared (attachments
// uploaded, body expanded inline) after the upload-name-matching bug aborted
// submit.mjs before sending.
import { connect, shot, log, currentModelLabel, composerText, sendMessage,
         parseDispatch, chipStem, ROOT } from './lib.mjs';
import fs from 'node:fs';
import path from 'node:path';

const dispatchFile = path.join(ROOT, 'exchange/outbox/001-reachable-support-cardinality.md');
const { frontmatter: fm, body } = parseDispatch(dispatchFile);
const tag = '001-reachable-support-cardinality';
const attachments = fm.attachments;
const COUNT_FILE = path.join(ROOT, 'exchange', 'submission_count.txt');
const count = parseInt(fs.readFileSync(COUNT_FILE, 'utf8').trim() || '0', 10);
// Retired one-off, kept as-is for the record: the literal 10 below is the
// fixed lifetime cap Jason retired 2026-08-01 as wrong framing. It is NOT the
// protocol — see exchange/README.md § Dispatch quota and HARD_CAP in
// submit.mjs (the current batch's ceiling). Do not copy this guard.
if (count >= 10) { log('BUDGET EXHAUSTED'); process.exit(3); }

const { browser, context } = await connect();
try {
  const pages = context.pages().filter(p => /chatgpt\.com/.test(p.url()));
  let page = null;
  for (const p of pages) {
    const t = await p.evaluate(() => document.querySelector('form')?.innerText || '').catch(() => '');
    if (t.includes('10_RULES(') && t.includes('You are performing')) { page = p; break; }
  }
  if (!page) throw new Error('prepared 001 tab not found');

  const model = await currentModelLabel(page);
  if (!model || !/\bpro\b/i.test(model.text)) throw new Error(`model gate failed: ${model?.text}`);

  const norm = s => s.replace(/\s+/g, ' ').trim();
  const inComposer = norm(await composerText(page));
  const want = norm(body);
  if (!inComposer.includes(want.slice(0, 80)) || !inComposer.includes(want.slice(-80))) {
    throw new Error('composer does not match body ends');
  }
  const formText = await page.evaluate(() => document.querySelector('form')?.innerText || '');
  const missingChips = attachments.map(a => path.basename(a)).filter(n => !formText.includes(chipStem(n)));
  if (missingChips.length) throw new Error(`chips missing: ${missingChips}`);

  const baselineUser = await page.evaluate(() =>
    document.querySelectorAll('[data-message-author-role="user"]').length);

  await shot(page, `${tag}-pre-send`);
  log(`${tag}: pre-send verified (model=${model.text}, 3 chips, ${want.length} chars). SENDING.`);
  await sendMessage(page);

  let convUrl = null;
  for (let i = 0; i < 60; i++) {
    await page.waitForTimeout(1000);
    const u = page.url();
    const userCount = await page.evaluate(() =>
      document.querySelectorAll('[data-message-author-role="user"]').length);
    if (userCount > baselineUser && /\/c\//.test(u)) { convUrl = u; break; }
  }
  await shot(page, `${tag}-sent`);
  if (!convUrl) throw new Error(`${tag}: AMBIGUOUS SEND — inspect ${tag}-sent.png`);

  fs.writeFileSync(COUNT_FILE, String(count + 1) + '\n');
  const meta = {
    tag, number: fm.number, slug: fm.slug, channel: fm.channel || 'new-chat',
    conversationUrl: convUrl, submittedAt: new Date().toISOString(),
    baselineAssistantCount: 0, attachments,
  };
  fs.writeFileSync(dispatchFile.replace(/\.md$/, '.submitted.json'), JSON.stringify(meta, null, 2) + '\n');
  log(`${tag}: SUBMITTED count=${count + 1}/10 url=${convUrl}`);

  await page.goto(convUrl, { waitUntil: 'domcontentloaded', timeout: 60000 });
  await page.waitForTimeout(5000);
  const check = await page.evaluate(() => {
    const users = document.querySelectorAll('[data-message-author-role="user"]');
    const last = users[users.length - 1];
    const turn = last?.closest('article') || last?.parentElement?.parentElement;
    return turn?.innerText || '';
  });
  const missing = attachments.map(a => path.basename(a)).filter(n => !check.includes(chipStem(n)));
  const bodyOk = check.replace(/\s+/g, ' ').includes(want.slice(0, 80));
  await shot(page, `${tag}-verified`);
  if (!bodyOk || missing.length) {
    throw new Error(`${tag}: SENT but verify failed (bodyOk=${bodyOk}, missing=${missing})`);
  }
  log(`${tag}: post-send verification OK`);
  await page.close();
} finally {
  await browser.close();
}
