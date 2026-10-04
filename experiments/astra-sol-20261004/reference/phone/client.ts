import type { NativeDecision, NativeRequest } from '../native';
import type { AuctionCall, AuctionSurvey } from '../auction';
import { runAuctionPool, auctionPoolSize, type AuctionPoolOptions } from './auction-pool';

export interface PlayerCall { request: NativeRequest; worlds: number; partner: boolean; budget_ms?: number }

/** One worker owns one decision. Ending a request also releases all solver
 * memory; requests cannot queue behind an abandoned calculation. */
export function runPlayer(call: PlayerCall, signal?: AbortSignal): Promise<NativeDecision> {
  return runInWorker(() => new Worker(new URL('./worker.ts', import.meta.url), { type: 'module' }), call, signal);
}

export function runAuction(call: AuctionCall, signal?: AbortSignal, options?: AuctionPoolOptions): Promise<AuctionSurvey> {
  return runAuctionPool(() => new Worker(new URL('./worker.ts', import.meta.url), { type: 'module' }), call, signal, auctionPoolSize(), options);
}

export function runInWorker<T extends { interruption?: string } = NativeDecision>(create: () => Worker, call: PlayerCall | AuctionCall, signal?: AbortSignal): Promise<T> {
  return new Promise((resolve, reject) => {
    if (signal?.aborted) { reject(new DOMException('Stopped', 'AbortError')); return; }
    const worker = create();
    let saved: T | undefined;
    let settled = false;
    const finish = (result?: T, error?: unknown): void => {
      if (settled) return;
      settled = true; clearTimeout(timer); worker.terminate(); signal?.removeEventListener('abort', abort);
      if (result) resolve(result); else reject(error ?? new Error('Walt did not finish. Please retry.'));
    };
    const abort = (): void => finish(undefined, new DOMException('Stopped', 'AbortError'));
    const interrupted = (reason: string): void => {
      if (saved) finish({ ...saved, interruption: reason });
      else finish(undefined, new Error(reason));
    };
    const timer = setTimeout(() => interrupted('The host stopped the calculation; the last completed decision was retained.'), (call.budget_ms ?? 14000) + 4000);
    signal?.addEventListener('abort', abort, { once: true });
    worker.onmessage = ({ data }: MessageEvent) => {
      const message = data as { id: number; checkpoint?: T; result?: T & { error?: string }; error?: string };
      if (message.id !== 0) return;
      if (message.checkpoint) saved = message.checkpoint;
      if (message.result?.error) finish(undefined, new Error(message.result.error));
      else if (message.result) finish(message.result);
      else if (message.error) interrupted(message.error);
    };
    worker.onerror = (event) => { event.preventDefault(); interrupted(event.message); };
    worker.postMessage({ id: 0, call });
  });
}
