import { connect, openPage, shot, log } from './lib.mjs';
import fs from 'node:fs';

const { browser, context } = await connect();
const page = await openPage(context, 'https://chatgpt.com/');

const inputs = await page.evaluate(() =>
  Array.from(document.querySelectorAll('input[type="file"]')).map(i => ({
    accept: i.accept, multiple: i.multiple, cls: i.className.slice(0, 80),
    visible: !!(i.offsetParent), id: i.id
  }))
);
log(`file inputs: ${JSON.stringify(inputs, null, 1)}`);

const tmp = '/private/tmp/claude-501/-Users-jason-code-texas-42/075055ca-1949-4011-9c77-aecfc5e6570b/scratchpad/rehearse-attach.txt';
fs.writeFileSync(tmp, 'attachment rehearsal file\n');

const input = page.locator('input[type="file"]').first();
if (await input.count() > 0) {
  await input.setInputFiles(tmp);
  log('setInputFiles done on first input');
  await page.waitForTimeout(6000);
  await shot(page, 'probe-attach-after');
  const dom = await page.evaluate(() => {
    const form = document.querySelector('form');
    return form ? form.innerText.slice(0, 500) : 'no form';
  });
  log(`form text after attach: ${JSON.stringify(dom)}`);
} else {
  log('NO file input found; will need to click + menu first');
  await shot(page, 'probe-attach-noinput');
}
await page.close();
await browser.close();
