import { connect, openPage, isLoggedIn, shot, log } from './lib.mjs';

const { browser, context } = await connect();
const page = await openPage(context, 'https://chatgpt.com/');
await page.waitForTimeout(4000);
const state = await isLoggedIn(page);
await shot(page, 'login-check');
log(`login state: ${state === true ? 'LOGGED IN' : state === false ? 'LOGGED OUT' : 'UNKNOWN'} url=${page.url()}`);
await page.close();
await browser.close();
