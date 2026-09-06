// One isolated call to the preserved local phone solver. Node >= 23.6 is
// required for the unmodified TypeScript wrapper. The parent process owns
// wall-clock enforcement and accounting; this process has no timeout logic.
import { readFile } from 'node:fs/promises';

function positiveInteger(value, name) {
  if (!Number.isSafeInteger(value) || value < 1) {
    throw new Error(`${name} must be a positive safe integer`);
  }
  return value;
}

function seedOf(value) {
  if (typeof value === 'number') {
    if (!Number.isSafeInteger(value) || value < 0) {
      throw new Error('Numeric seed must be a nonnegative safe integer; use a decimal string for u64');
    }
    value = String(value);
  }
  if (typeof value !== 'string' || !/^[0-9]+$/.test(value)) {
    throw new Error('An explicit unsigned decimal seed is required');
  }
  const seed = BigInt(value);
  if (seed > 0xffffffffffffffffn) throw new Error('Seed exceeds u64');
  return seed;
}

try {
  let inputText = '';
  for await (const chunk of process.stdin) inputText += chunk;
  const input = JSON.parse(inputText);
  const knobs = input.knobs ?? {};
  const race = knobs.race ?? input.race ?? true;
  if (typeof race !== 'boolean') throw new Error('race must be a boolean');
  // Select only actor-private and public fields. Complete referee deals and
  // fixture metadata cannot enter the oracle through this boundary.
  const req = {
    decl: input.decl,
    bid: input.bid,
    seat: input.seat,
    bidder: input.bidder,
    hand: input.hand,
    plays: input.plays,
    n: positiveInteger(knobs.n ?? input.n ?? 40, 'n'),
    n0: positiveInteger(knobs.n0 ?? input.n0 ?? 8, 'n0'),
    race,
    seed: seedOf(knobs.seed ?? input.seed),
  };
  const { Walt } = await import('./reference/phone/walt.ts');
  const bytes = await readFile(new URL('./reference/phone/walt.wasm', import.meta.url));
  const walt = await Walt.load(bytes);
  const response = walt.play(req);
  process.stdout.write(`${JSON.stringify(response)}\n`);
} catch (error) {
  process.stdout.write(`${JSON.stringify({status: 'error', error: error instanceof Error ? error.message : String(error)})}\n`);
  process.exitCode = 1;
}
