// Dry-run capability proof. NEVER sends a message.
// Proves: (a) open new chat (b) Pro model selection (c) paste into composer
// (d) attach a file (e) read assistant message from the standing conversation.
import { connect, openPage, shot, log, ensureProModel, currentModelLabel,
         pasteIntoComposer, composerText, attachFile, waitForUploads,
         assistantMessages, isResponseComplete } from './lib.mjs';
import fs from 'node:fs';

const STANDING = 'https://chatgpt.com/c/6a64ccec-2328-83ea-b0d1-917f487297a2';
const { browser, context } = await connect();

try {
  // (a) new chat
  const page = await openPage(context, 'https://chatgpt.com/');
  log('(a) new chat opened');

  // (b) model = Pro
  await context.grantPermissions(['clipboard-read', 'clipboard-write'], { origin: 'https://chatgpt.com' }).catch(e => log(`grantPermissions failed: ${e.message}`));
  const before = await currentModelLabel(page);
  log(`(b) picker shows: ${JSON.stringify(before?.text)}`);
  await ensureProModel(page);
  await shot(page, 'rehearse-model-pro');
  log('(b) Pro verified');

  // (c) paste multi-line text
  const sample = 'REHEARSAL line one\n\nline two with unicode — ∀x∈S — and `code`\n\nline three';
  await pasteIntoComposer(page, sample);
  const got = await composerText(page);
  log(`(c) composer now contains ${got.length} chars; startsWith REHEARSAL=${got.startsWith('REHEARSAL')}`);
  if (!got.includes('line three')) throw new Error('paste incomplete');

  // (d) attach a file
  const tmp = '/private/tmp/claude-501/-Users-jason-code-texas-42/075055ca-1949-4011-9c77-aecfc5e6570b/scratchpad/rehearse-attach.txt';
  fs.mkdirSync(require_dirname(tmp), { recursive: true });
  fs.writeFileSync(tmp, 'attachment rehearsal file\n');
  await attachFile(page, tmp);
  await waitForUploads(page, 1);
  await shot(page, 'rehearse-composer-full');
  log('(d) attachment uploaded');

  // clean up composer WITHOUT sending: just close the tab
  await page.close();
  log('composer tab closed unsent');

  // (e) read standing conversation
  const conv = await openPage(context, STANDING);
  await conv.waitForTimeout(4000);
  const state = await isResponseComplete(conv);
  const msgs = await assistantMessages(conv);
  await shot(conv, 'rehearse-standing-conv');
  log(`(e) standing conv: state=${JSON.stringify(state)}; last msg preview: ${JSON.stringify((msgs.at(-1) || '').slice(0, 200))}`);
  await conv.close();

  log('REHEARSAL PASSED');
} finally {
  await browser.close();
}

function require_dirname(p) { return p.slice(0, p.lastIndexOf('/')); }
