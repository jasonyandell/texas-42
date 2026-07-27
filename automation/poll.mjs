// Poll one conversation for completion.
// Usage: node poll.mjs <NNN-slug.submitted.json>
// Prints a JSON status line. Exit 0 = complete, 2 = pending, 1 = error.
// "Complete" = a new assistant turn beyond the baseline exists, nothing is
// streaming, and the last turn has its copy button (only rendered when done).
import { connect, openPage, log } from './lib.mjs';
import fs from 'node:fs';

const metaFile = process.argv[2];
const meta = JSON.parse(fs.readFileSync(metaFile, 'utf8'));
const tag = meta.tag || `${meta.number}-${meta.slug}`;

const { browser, context } = await connect();
try {
  const page = await openPage(context, meta.conversationUrl);
  await page.waitForTimeout(6000);
  const s = await page.evaluate(() => {
    const asst = document.querySelectorAll('[data-message-author-role="assistant"]');
    const last = asst[asst.length - 1];
    const lastTurn = last?.closest('article') || last?.parentElement?.parentElement;
    return {
      assistantCount: asst.length,
      lastLen: last?.innerText.length ?? 0,
      stop: !!document.querySelector('[data-testid="stop-button"], button[aria-label*="Stop"]'),
      streaming: !!document.querySelector('.result-streaming, [data-testid*="streaming"]'),
      copyBtnOnLast: !!(lastTurn?.querySelector('[data-testid="copy-turn-action-button"]')),
      progress: document.body.innerText.match(/Research(ing)?|Thinking|Working/i)?.[0] ?? null,
    };
  });
  const complete = s.assistantCount > meta.baselineAssistantCount
    && !s.stop && !s.streaming && s.copyBtnOnLast;
  log(`${tag}: ${complete ? 'COMPLETE' : 'pending'} ${JSON.stringify(s)}`);
  await page.close();
  process.exit(complete ? 0 : 2);
} finally {
  await browser.close();
}
