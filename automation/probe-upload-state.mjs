import { connect, openPage, shot, log } from './lib.mjs';
import fs from 'node:fs';

const tmp = '/private/tmp/claude-501/-Users-jason-code-texas-42/075055ca-1949-4011-9c77-aecfc5e6570b/scratchpad/probe-upload.txt';
fs.writeFileSync(tmp, 'x'.repeat(200000));

const { browser, context } = await connect();
const page = await openPage(context, 'https://chatgpt.com/');
await page.waitForTimeout(2000);

async function sample() {
  return page.evaluate(() => {
    const form = document.querySelector('form');
    const send = document.querySelector('[data-testid="send-button"]');
    return {
      hasFile: /probe-upload/.test(form?.innerText || ''),
      sendDisabled: send ? send.disabled || send.getAttribute('aria-disabled') === 'true' : null,
      spinners: form ? form.querySelectorAll('svg animate, [class*="animate-spin"], [role="progressbar"]').length : -1,
    };
  });
}

await page.locator('#upload-files').setInputFiles(tmp);
for (let i = 0; i < 20; i++) {
  log(`t=${i * 500}ms ${JSON.stringify(await sample())}`);
  await page.waitForTimeout(500);
}
await shot(page, 'probe-upload-state');

// now clear composer draft text + attachment
const composer = page.locator('#prompt-textarea').first();
await composer.click();
await page.keyboard.press('Meta+KeyA');
await page.keyboard.press('Backspace');
await page.waitForTimeout(500);
// remove attachment chip via its remove button if present
const removed = await page.evaluate(() => {
  const btns = document.querySelectorAll('form button[aria-label*="Remove"], form button[aria-label*="remove"]');
  btns.forEach(b => b.click());
  return btns.length;
});
await page.waitForTimeout(1000);
const after = await page.evaluate(() => document.querySelector('form')?.innerText.slice(0, 200));
log(`removed ${removed} chips; form after clear: ${JSON.stringify(after)}`);
await shot(page, 'probe-cleared');
await page.close();
await browser.close();
