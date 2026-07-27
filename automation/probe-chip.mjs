import { connect, openPage, shot, log } from './lib.mjs';

const { browser, context } = await connect();
const page = await openPage(context, 'https://chatgpt.com/');
await page.waitForTimeout(3000);

const info = await page.evaluate(() => {
  const form = document.querySelector('form');
  const testids = new Set();
  form.querySelectorAll('[data-testid]').forEach(n => testids.add(n.getAttribute('data-testid')));
  // find the attachment chip by filename text
  let chipHtml = null;
  const walker = document.createTreeWalker(form, NodeFilter.SHOW_ELEMENT);
  while (walker.nextNode()) {
    const el = walker.currentNode;
    if (el.childElementCount === 0 && /rehearse-attach/.test(el.textContent)) {
      let up = el; for (let i = 0; i < 4 && up.parentElement; i++) up = up.parentElement;
      chipHtml = up.outerHTML.slice(0, 2000);
      break;
    }
  }
  return { testids: [...testids], chipHtml, formText: form.innerText.slice(0, 300) };
});
log(JSON.stringify(info, null, 1));
await shot(page, 'probe-chip');
await page.close();
await browser.close();
