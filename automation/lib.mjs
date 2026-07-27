import { chromium } from 'playwright-core';
import fs from 'node:fs';
import path from 'node:path';

export const ROOT = '/Users/jason/code/texas-42';
export const LOGS = path.join(ROOT, 'automation', 'logs');
const CDP = `http://127.0.0.1:${process.env.CDP_PORT || 9222}`;

export async function connect() {
  const browser = await chromium.connectOverCDP(CDP);
  const context = browser.contexts()[0];
  return { browser, context };
}

export async function shot(page, name) {
  const file = path.join(LOGS, `${name}.png`);
  await page.screenshot({ path: file });
  log(`screenshot ${file}`);
  return file;
}

export function log(msg) {
  const line = `[${new Date().toISOString()}] ${msg}`;
  console.log(line);
  fs.appendFileSync(path.join(LOGS, 'harness.log'), line + '\n');
}

// --- ChatGPT page helpers -------------------------------------------------

export async function openPage(context, url) {
  const page = await context.newPage();
  await page.goto(url, { waitUntil: 'domcontentloaded', timeout: 60000 });
  await page.waitForTimeout(3000);
  return page;
}

export async function isLoggedIn(page) {
  // Logged-out chatgpt.com shows "Log in" / "Sign up" buttons.
  const loginBtn = page.locator('[data-testid="login-button"], button:has-text("Log in")').first();
  const composer = page.locator('#prompt-textarea, [contenteditable="true"]').first();
  if (await composer.count() > 0 && await composer.isVisible().catch(() => false)) return true;
  if (await loginBtn.count() > 0 && await loginBtn.isVisible().catch(() => false)) return false;
  return null;
}

export function composerLocator(page) {
  return page.locator('#prompt-textarea').first();
}

// Model picker: button in composer area shows current model (e.g. "Pro").
// Menu is titled "Intelligence" with options Instant 5.5 / Medium / High /
// Extra High / Pro.
export async function currentModelLabel(page) {
  const btn = page.locator('[data-testid="model-switcher-dropdown-button"], button[aria-label*="Model"], button[aria-haspopup="menu"]:has-text("Pro"), button[aria-haspopup="menu"]');
  const n = await btn.count();
  for (let i = 0; i < n; i++) {
    const t = (await btn.nth(i).innerText().catch(() => '')).trim();
    if (/instant|medium|high|pro|5\.6|auto|thinking/i.test(t)) return { text: t, el: btn.nth(i) };
  }
  return null;
}

export async function ensureProModel(page) {
  const cur = await currentModelLabel(page);
  if (cur && /\bpro\b/i.test(cur.text)) return true;
  if (!cur) throw new Error('model picker button not found');
  await cur.el.click();
  await page.waitForTimeout(800);
  const proItem = page.locator('[role="menuitem"]:has-text("Pro"), [role="option"]:has-text("Pro")').first();
  await proItem.waitFor({ state: 'visible', timeout: 5000 });
  await proItem.click();
  await page.waitForTimeout(1000);
  const after = await currentModelLabel(page);
  if (!after || !/\bpro\b/i.test(after.text)) {
    throw new Error(`Pro not selected; picker shows: ${after?.text ?? 'nothing'}`);
  }
  return true;
}

// Paste text into the composer via clipboard (fidelity + speed).
export async function pasteIntoComposer(page, text) {
  const composer = composerLocator(page);
  await composer.click();
  await page.evaluate(async (t) => {
    await navigator.clipboard.writeText(t);
  }, text);
  const mod = 'Meta';
  await page.keyboard.press(`${mod}+KeyV`);
  await page.waitForTimeout(1500);
}

export async function composerText(page) {
  return (await composerLocator(page).innerText().catch(() => '')).trim();
}

// Attach a file via the hidden #upload-files input that backs the "+" menu.
export async function attachFile(page, filePath) {
  await page.locator('#upload-files').setInputFiles(filePath);
}

// Upload complete when every filename shows in the form AND the send button is
// enabled (ChatGPT disables send while any attachment is still uploading).
export async function waitForUploads(page, fileNames, timeoutMs = 180000) {
  const start = Date.now();
  while (Date.now() - start < timeoutMs) {
    const state = await page.evaluate(() => {
      const form = document.querySelector('form');
      const send = document.querySelector('[data-testid="send-button"]');
      return {
        formText: form?.innerText || '',
        sendDisabled: send ? (send.disabled || send.getAttribute('aria-disabled') === 'true') : true,
      };
    });
    const allNamed = fileNames.every(n => state.formText.includes(n));
    if (allNamed && !state.sendDisabled) return true;
    await page.waitForTimeout(2000);
  }
  throw new Error('attachments did not finish uploading in time');
}

// Clear any persisted draft text and stray attachment chips.
export async function clearComposer(page) {
  const composer = composerLocator(page);
  await composer.click();
  await page.keyboard.press('Meta+KeyA');
  await page.keyboard.press('Backspace');
  await page.evaluate(() => {
    document.querySelectorAll('form button[aria-label*="Remove"], form button[aria-label*="remove"]')
      .forEach(b => b.click());
  });
  await page.waitForTimeout(500);
}

export async function sendMessage(page) {
  const btn = page.locator('[data-testid="send-button"], button[aria-label*="Send"]').first();
  await btn.waitFor({ state: 'visible', timeout: 10000 });
  await btn.click();
}

// --- reading conversations ------------------------------------------------

export async function assistantMessages(page) {
  return page.evaluate(() => {
    const nodes = document.querySelectorAll('[data-message-author-role="assistant"]');
    return Array.from(nodes).map(n => n.innerText);
  });
}

export async function isResponseComplete(page) {
  return page.evaluate(() => {
    const stop = document.querySelector('[data-testid="stop-button"], button[aria-label*="Stop"]');
    const streaming = document.querySelector('.result-streaming, [data-testid*="streaming"]');
    const thinking = document.querySelector('[data-testid*="thinking"], [class*="thinking-indicator"]');
    const msgs = document.querySelectorAll('[data-message-author-role="assistant"]');
    return { stop: !!stop, streaming: !!streaming, thinking: !!thinking, assistantCount: msgs.length };
  });
}

export function parseDispatch(file) {
  const raw = fs.readFileSync(file, 'utf8');
  const m = raw.match(/^---\n([\s\S]*?)\n---\n?/);
  if (!m) throw new Error(`no frontmatter in ${file}`);
  const fm = {};
  let curKey = null;
  for (const line of m[1].split('\n')) {
    const kv = line.match(/^(\w[\w-]*):\s*(.*)$/);
    if (kv) {
      curKey = kv[1];
      const v = kv[2].trim();
      fm[curKey] = v === '' ? [] : v;
    } else if (curKey && /^\s*-\s+/.test(line)) {
      if (!Array.isArray(fm[curKey])) fm[curKey] = [];
      fm[curKey].push(line.replace(/^\s*-\s+/, '').trim());
    }
  }
  const body = raw.slice(m[0].length);
  return { frontmatter: fm, body };
}
