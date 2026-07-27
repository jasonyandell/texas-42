// Submit one dispatch to ChatGPT 5.6 Pro.
// Usage: node submit.mjs /abs/path/exchange/outbox/NNN-slug.md
// Refuses if budget exhausted. On success writes NNN-slug.submitted.json
// next to the dispatch, increments exchange/submission_count.txt, and
// appends a ledger row placeholder is left to the operator.
import { connect, openPage, shot, log, ensureProModel, currentModelLabel,
         pasteIntoComposer, composerText, attachFile, waitForUploads,
         clearComposer, sendMessage, parseDispatch, ROOT } from './lib.mjs';
import fs from 'node:fs';
import path from 'node:path';

const STANDING = 'https://chatgpt.com/c/6a64ccec-2328-83ea-b0d1-917f487297a2';
const COUNT_FILE = path.join(ROOT, 'exchange', 'submission_count.txt');
const HARD_CAP = 10;

const dispatchFile = process.argv[2];
if (!dispatchFile) { console.error('usage: submit.mjs <dispatch.md>'); process.exit(1); }
const { frontmatter: fm, body } = parseDispatch(dispatchFile);
const tag = path.basename(dispatchFile, '.md');

const count = parseInt(fs.readFileSync(COUNT_FILE, 'utf8').trim() || '0', 10);
if (count >= HARD_CAP) { log(`BUDGET EXHAUSTED (${count}/${HARD_CAP}); refusing to submit ${tag}`); process.exit(3); }

const attachments = Array.isArray(fm.attachments) ? fm.attachments
  : (fm.attachments ? [fm.attachments] : []);
for (const a of attachments) {
  const p = path.join(ROOT, a);
  if (!fs.existsSync(p)) { log(`missing attachment ${p}`); process.exit(1); }
}

const isContinuation = fm.channel === 'continuation';
const targetUrl = isContinuation ? STANDING : 'https://chatgpt.com/';

const { browser, context } = await connect();
try {
  await context.grantPermissions(['clipboard-read', 'clipboard-write'], { origin: 'https://chatgpt.com' }).catch(() => {});
  const page = await openPage(context, targetUrl);
  await page.waitForTimeout(3000);

  const baseline = await page.evaluate(() => ({
    assistant: document.querySelectorAll('[data-message-author-role="assistant"]').length,
    user: document.querySelectorAll('[data-message-author-role="user"]').length,
  }));

  await clearComposer(page);
  await ensureProModel(page);
  const model = await currentModelLabel(page);
  if (!model || !/\bpro\b/i.test(model.text)) throw new Error(`model gate failed: ${model?.text}`);
  log(`${tag}: model=${model.text}`);

  if (attachments.length) {
    await attachFile(page, attachments.map(a => path.join(ROOT, a)));
    log(`${tag}: attached ${attachments.length} files`);
  }
  await pasteIntoComposer(page, body);
  await page.waitForTimeout(1000);
  if (attachments.length) {
    await waitForUploads(page, attachments.map(a => path.basename(a).slice(0, 20)));
    log(`${tag}: uploads complete`);
  }

  // verify paste fidelity at both ends (whitespace-normalized)
  const norm = s => s.replace(/\s+/g, ' ').trim();
  const inComposer = norm(await composerText(page));
  const want = norm(body);
  if (!inComposer.includes(want.slice(0, 80)) || !inComposer.includes(want.slice(-80))) {
    await shot(page, `${tag}-paste-mismatch`);
    throw new Error(`${tag}: composer text does not match body ends`);
  }

  await shot(page, `${tag}-pre-send`);
  log(`${tag}: pre-send verified (model=Pro, ${attachments.length} attachments, ${want.length} chars). SENDING.`);

  await sendMessage(page);

  // confirm: user turn appears, composer empties, URL becomes /c/<id>
  let convUrl = null;
  for (let i = 0; i < 60; i++) {
    await page.waitForTimeout(1000);
    const u = page.url();
    const userCount = await page.evaluate(() =>
      document.querySelectorAll('[data-message-author-role="user"]').length);
    if (userCount > baseline.user && /\/c\//.test(u)) { convUrl = u; break; }
  }
  await shot(page, `${tag}-sent`);
  if (!convUrl) throw new Error(`${tag}: AMBIGUOUS SEND — inspect ${tag}-sent.png before retrying`);

  fs.writeFileSync(COUNT_FILE, String(count + 1) + '\n');
  const meta = {
    tag, number: fm.number, slug: fm.slug, channel: fm.channel || 'new-chat',
    conversationUrl: convUrl, submittedAt: new Date().toISOString(),
    baselineAssistantCount: baseline.assistant, attachments,
  };
  fs.writeFileSync(dispatchFile.replace(/\.md$/, '.submitted.json'), JSON.stringify(meta, null, 2) + '\n');
  log(`${tag}: SUBMITTED count=${count + 1}/${HARD_CAP} url=${convUrl}`);
  await page.close();
} finally {
  await browser.close();
}
