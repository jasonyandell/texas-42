// Read-only test of extraction on the standing conversation.
import { connect, openPage, shot, log, assistantMessages, isResponseComplete } from './lib.mjs';

const STANDING = 'https://chatgpt.com/c/6a64ccec-2328-83ea-b0d1-917f487297a2';
const { browser, context } = await connect();
const page = await openPage(context, STANDING);
await page.waitForTimeout(5000);
const state = await isResponseComplete(page);
const msgs = await assistantMessages(page);
await shot(page, 'probe-standing-conv');
log(`state=${JSON.stringify(state)}`);
log(`assistant messages: ${msgs.length}; last length=${msgs.at(-1)?.length}; preview=${JSON.stringify((msgs.at(-1) || '').slice(0, 300))}`);
await page.close();
await browser.close();
