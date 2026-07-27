// Wedge-proof watcher: ONE tab on ONE conversation. Completion is detected by
// observing the live DOM in place; a backstop *full re-navigation* runs at most
// once per 10 min. Critically, the harvest happens HERE, in this same process,
// the instant completion is seen (see SKILL.md WEDGE LESSON) — never split
// detection from harvest across a process exit.
//
// Hardened after the 006 silent death (a backstop reload left the page
// un-hydrated, its assistant text read as 0, and the process later exited
// without harvesting or leaving any trace):
//   - re-baseline the DOM after EVERY (re)navigation: wait until the
//     conversation's known prior assistant turns are present again before
//     trusting any read (an un-hydrated page reports 0 and poisons tracking);
//   - treat a collapse of assistant text/count to 0 as a TRACKING FAILURE:
//     re-navigate and never trust the 0 as real progress or completion;
//   - NEVER exit without either harvesting or writing an explicit FAILED
//     marker (exchange/inbox/<tag>.FAILED.md) plus a logged reason;
//   - log the process exit reason on every path.
// Usage: node watch-harvest.mjs <NNN-slug.submitted.json> [timeout-hours]
// Exit 0 = harvested (inbox file written); 3 = timed out (FAILED marker);
//          1 = error (FAILED marker).
import { connect, openPage, shot, log, ROOT } from './lib.mjs';
import fs from 'node:fs';
import path from 'node:path';

const metaFile = process.argv[2];
if (!metaFile) { console.error('usage: watch-harvest.mjs <NNN-slug.submitted.json> [timeout-hours]'); process.exit(1); }
const timeoutH = parseFloat(process.argv[3] || '3');
const meta = JSON.parse(fs.readFileSync(metaFile, 'utf8'));
const tag = meta.tag;
const outFile = path.join(ROOT, 'exchange', 'inbox', `${tag}.md`);
const failFile = path.join(ROOT, 'exchange', 'inbox', `${tag}.FAILED.md`);
const deadline = new Date(meta.submittedAt).getTime() + timeoutH * 3600 * 1000;
const baseline = meta.baselineAssistantCount ?? 0;

let harvested = false;
let exitReason = 'process ended before reaching any terminal state';

// Any exit that is NOT a successful harvest must leave a visible FAILED marker
// and a logged reason, so a delayed death can never look like "still running".
function writeFailed(reason) {
  exitReason = reason;
  if (harvested) return;
  try {
    const rel = path.relative(ROOT, metaFile);
    fs.writeFileSync(failFile,
      `---\ntag: ${tag}\nconversation: ${meta.conversationUrl}\n` +
      `submitted-at: ${meta.submittedAt}\nfailed-at: ${new Date().toISOString()}\n` +
      `reason: ${reason}\n---\n\n` +
      `watch-harvest.mjs did NOT harvest ${tag}. Reason: ${reason}\n\n` +
      `The response may already be complete in the conversation. Re-run the\n` +
      `harvest manually (single process, in place):\n\n` +
      `  node automation/harvest.mjs ${rel}\n\n` +
      `Then delete this marker.\n`);
    log(`${tag}: FAILED marker written (${reason}) -> ${failFile}`);
  } catch (e) {
    log(`${tag}: could not write FAILED marker (${e.message}); reason was: ${reason}`);
  }
}

process.on('exit', code => log(`${tag}: process exit code=${code} harvested=${harvested} reason="${exitReason}"`));
process.on('uncaughtException', e => { writeFailed(`uncaughtException: ${e.message}`); log(`${tag}: uncaughtException ${e.stack || e}`); process.exit(1); });
process.on('unhandledRejection', e => { writeFailed(`unhandledRejection: ${e?.message || e}`); log(`${tag}: unhandledRejection ${e?.stack || e}`); process.exit(1); });
for (const sig of ['SIGTERM', 'SIGINT', 'SIGHUP']) process.on(sig, () => { writeFailed(`killed by ${sig}`); process.exit(1); });

const { browser, context } = await connect();
await context.grantPermissions(['clipboard-read', 'clipboard-write'], { origin: 'https://chatgpt.com' }).catch(() => {});
const page = await openPage(context, meta.conversationUrl);

async function readState() {
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

// After every (re)navigation, wait until the conversation's pre-existing
// assistant turns are present again (assistantCount >= baseline) before trusting
// any read. Returns the first ready state, or the last read if it never settles.
async function waitReady(why) {
  let last = null;
  for (let i = 0; i < 30; i++) {
    await page.waitForTimeout(1000);
    const s = await readState().catch(() => null);
    if (s) { last = s; if (s.assistantCount >= baseline) { log(`${tag}: re-baselined after ${why} (asst=${s.assistantCount}>=${baseline}, lastLen=${s.lastLen})`); return s; } }
  }
  log(`${tag}: WARNING did not re-baseline within 30s after ${why} (last=${JSON.stringify(last)})`);
  return last;
}

async function reNavigate(why) {
  log(`${tag}: re-navigating (${why})`);
  await page.goto(meta.conversationUrl, { waitUntil: 'domcontentloaded', timeout: 60000 }).catch(e => log(`${tag}: nav error ${e.message}`));
  return waitReady(why);
}

async function harvest(s) {
  log(`${tag}: COMPLETE detected in-place ${JSON.stringify(s)}; harvesting NOW in-process`);
  await page.evaluate(() => {
    const a = document.querySelectorAll('[data-message-author-role="assistant"]');
    a[a.length - 1]?.scrollIntoView({ block: 'end' });
  }).catch(() => {});
  await page.waitForTimeout(1000);
  let text = null, method = 'copy-button';
  const caveats = [];
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
    }).catch(() => null);
    method = 'innerText';
    caveats.push('copy-button unavailable; DOM innerText (markdown may be lossy)');
  }
  if (!text) throw new Error('completion detected but no message text extracted');
  await shot(page, `${tag}-final`).catch(() => {});
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
  harvested = true;
  exitReason = 'harvested';
  log(`${tag}: HARVESTED ${text.length} chars via ${method} -> ${outFile}`);
  console.log(`HARVESTED: ${tag}`);
}

try {
  await waitReady('initial load');
  let lastReload = Date.now();
  let lastLen = -1;
  let prevLastLen = -1;

  while (Date.now() < deadline) {
    const s = await readState().catch(e => ({ err: e.message }));

    if (s.err) {
      log(`${tag}: watch eval error (${s.err}); re-navigating`);
      await reNavigate('eval error');
      lastReload = Date.now(); lastLen = -1; prevLastLen = -1;
      await page.waitForTimeout(30000);
      continue;
    }

    // TRACKING-FAILURE guard: the known prior turns vanished (un-hydrated page),
    // or the last assistant text collapsed to 0 without a new turn appearing.
    // Never trust the 0 — re-navigate and re-baseline.
    const collapsed = (s.assistantCount < baseline)
      || (prevLastLen > 0 && s.lastLen === 0 && s.assistantCount <= baseline);
    if (collapsed) {
      log(`${tag}: DOM tracking failure (asst=${s.assistantCount} lastLen=${s.lastLen} baseline=${baseline}); re-navigating, not trusting the 0`);
      await reNavigate('lastLen/assistantCount collapse');
      lastReload = Date.now(); lastLen = -1; prevLastLen = -1;
      await page.waitForTimeout(30000);
      continue;
    }

    // Completion: a NEW assistant turn, not generating, with a copy button and
    // actual content. lastLen>0 avoids harvesting an empty bubble.
    if (s.assistantCount > baseline && !s.stop && s.copyBtnOnLast && s.lastLen > 0) {
      await harvest(s);
      await browser.close().catch(() => {});
      process.exit(0);
    }

    prevLastLen = s.lastLen;
    if (s.lastLen !== lastLen) {
      log(`${tag}: progress lastLen=${s.lastLen} stop=${s.stop} asst=${s.assistantCount}`);
      lastLen = s.lastLen;
      lastReload = Date.now(); // content moving; no backstop reload needed
    } else if (Date.now() - lastReload > 600000) {
      log(`${tag}: no change in 10 min; backstop re-navigation`);
      await reNavigate('10-min backstop');
      lastReload = Date.now();
    }
    await page.waitForTimeout(30000);
  }

  log(`${tag}: TIMEOUT after ${timeoutH}h without completion`);
  await shot(page, `${tag}-timeout`).catch(() => {});
  writeFailed(`timeout after ${timeoutH}h without a completed response`);
  await browser.close().catch(() => {});
  console.log(`TIMEOUT: ${tag}`);
  process.exit(3);
} catch (e) {
  log(`${tag}: watch loop exception ${e.stack || e}`);
  writeFailed(`exception in watch loop: ${e.message}`);
  await browser.close().catch(() => {});
  process.exit(1);
}
