/**
 * walt2 — the level-2 Texas 42 decision oracle, compiled to WebAssembly.
 *
 * EXPLORATORY tier: sampled estimates, never receipts. Straight
 * points-and-marks 42 only (pip trumps, doubles, no-trump); for anything
 * else the caller falls back to its own AI.
 *
 * The level-2 sibling of walt.wasm: the same outer belief sampling, but
 * every opponent and partner inside a sampled world is a modeled LEVEL-1
 * mind — a mind that itself samples worlds and reasons about what it
 * cannot see. Bidding and trump naming are the walt-1 rules, byte-
 * identical to walt.wasm's (the auction sees an empty record).
 *
 * Zero dependencies, no wasm-bindgen: the module exports the same four
 * plain functions as walt.wasm and a linear memory, and this file is the
 * whole ABI client. Calls are SYNCHRONOUS and take substantially longer
 * than walt.wasm's at the same knobs (cost grows roughly with n * n1) —
 * run this inside a Web Worker, never on the UI thread, and keep the
 * knobs small (defaults: n=8, n1=4, n0=2; see README.md for measured
 * trick-1 latencies per knob setting).
 *
 * Tile ids are walt's canonical triangular order: id(high, low) =
 * high*(high+1)/2 + low, so (0,0)=0, (1,0)=1, (1,1)=2, ..., (6,6)=27.
 * Use tileOf/pipsOf to convert from "65"-style high/low pairs.
 *
 * Seats are the caller's own labels 0..3 (partners face each other:
 * 0&2 vs 1&3). walt2 handles its internal frame rotation itself.
 * Declaration ids: 0..6 = pip trump (blanks..sixes), 7 = doubles-trump,
 * 9 = no-trump.
 */

export function tileOf(high: number, low: number): number {
  const [h, l] = high >= low ? [high, low] : [low, high];
  return (h * (h + 1)) / 2 + l;
}

export function pipsOf(tile: number): [number, number] {
  let h = 0;
  while (((h + 1) * (h + 2)) / 2 <= tile) h++;
  return [h, tile - (h * (h + 1)) / 2];
}

export interface PlayRequest {
  /** Declaration id: 0..6 pip trump, 7 doubles, 9 no-trump. */
  decl: number;
  /** Contract bid 30..=42 — the make/set thresholds walt2 plays to. */
  bid: number;
  /** Seat to decide for (0..3, caller's labels). */
  seat: number;
  /** Auction winner's seat (leads trick one). */
  bidder: number;
  /** The seat's 7 ORIGINALLY DEALT tile ids (not the remaining hand). */
  hand: number[];
  /** Chronological flat (actor, tile) pairs of every play so far. */
  plays: number[];
  /** Outer belief sample size (default 8). Larger = stronger + slower. */
  n?: number;
  /** Modeled level-1 mind sample size (default 4) — the level-2 knob. */
  n1?: number;
  /** The level-1 minds' modeled level-0 sample size (default 2). */
  n0?: number;
  /** Decision-stream seed; fix it for replayable games. Same formula as
   * walt.wasm: identical requests draw identical outer worlds at both
   * levels (common random numbers across levels). */
  seed?: bigint | number;
  /** Wall budget — only enforced in native builds; inert in wasm. */
  budgetMs?: number;
}

export interface PlayResponse {
  /** Always 2 — the evaluation depth marker. */
  level: number;
  /** Chosen tile id — always legal at the seat's information state. */
  choice: number;
  /** True if there was only one legal play (no evaluation ran). */
  forced: boolean;
  /** [tile, basisPoints] for every legal play (empty when forced). */
  opts: [number, number][];
  /** walt2's independently derived current trick leader (caller labels). */
  leader: number;
  /** Banked points [team of seats 0&2, team of seats 1&3] — assert these
   * against your own engine every call: it proves rules conformance. */
  points: [number, number];
}

export interface BidRequest {
  hand: number[];
  /** Minimum viable bid (current high + 1, or 30). */
  need: number;
  /** Bid while P(make) >= theta, as [num, den]. Default [11, 16]. */
  theta?: [number, number];
  n?: number;
  n0?: number;
  seed?: bigint | number;
  budgetMs?: number;
}

export interface BidResponse {
  action: 'bid' | 'pass';
  /** Final walked-up bid (action 'bid' only). */
  bid?: number;
  /** The declaration the bid leans on (arena id; not yet binding). */
  decl?: number;
  /** P(make bid) in basis points on the sample (action 'bid' only). */
  bp?: number;
  /** [declId, basisPoints] for all nine declarations at `need`. */
  prices: [number, number][];
}

export interface DeclareRequest {
  hand: number[];
  /** The won contract's bid. */
  bid: number;
  n?: number;
  n0?: number;
  seed?: bigint | number;
  budgetMs?: number;
}

export interface DeclareResponse {
  /** Chosen declaration id. */
  decl: number;
  /** [declId, basisPoints] for all nine declarations at the bid. */
  prices: [number, number][];
}

interface WaltExports {
  memory: WebAssembly.Memory;
  walt_in_prepare: (len: number) => number;
  walt_call: () => number;
  walt_out_ptr: () => number;
}

export class Walt2 {
  private readonly ex: WaltExports;

  private constructor(ex: WaltExports) {
    this.ex = ex;
  }

  /** Instantiate from compiled module, raw bytes, or a fetch Response. */
  static async load(
    src: WebAssembly.Module | BufferSource | Response | Promise<Response>,
  ): Promise<Walt2> {
    let module: WebAssembly.Module;
    if (src instanceof WebAssembly.Module) {
      module = src;
    } else if (src instanceof Response || src instanceof Promise) {
      module = await WebAssembly.compileStreaming(src as Response);
    } else {
      module = await WebAssembly.compile(src as BufferSource);
    }
    const instance = await WebAssembly.instantiate(module, {});
    return new Walt2(instance.exports as unknown as WaltExports);
  }

  /** Raw request/response (the line protocol documented in the crate). */
  raw(request: string): string {
    const bytes = new TextEncoder().encode(request);
    const inPtr = this.ex.walt_in_prepare(bytes.length);
    new Uint8Array(this.ex.memory.buffer, inPtr, bytes.length).set(bytes);
    const outLen = this.ex.walt_call();
    // Re-read memory AFTER the call: it may have grown and moved.
    const outPtr = this.ex.walt_out_ptr();
    const out = new Uint8Array(this.ex.memory.buffer, outPtr, outLen);
    return new TextDecoder().decode(out);
  }

  private call(lines: string[]): Record<string, unknown> {
    const resp = JSON.parse(this.raw(lines.join('\n') + '\n')) as Record<
      string,
      unknown
    >;
    if (typeof resp['error'] === 'string') {
      throw new Error(`walt2: ${resp['error']}`);
    }
    return resp;
  }

  play(req: PlayRequest): PlayResponse {
    const lines = [
      'walt2 play',
      `decl ${req.decl}`,
      `bid ${req.bid}`,
      `seat ${req.seat}`,
      `bidder ${req.bidder}`,
      `hand ${req.hand.join(' ')}`,
      `plays ${req.plays.join(' ')}`,
    ];
    if (req.n1 !== undefined) lines.push(`n1 ${req.n1}`);
    pushCommon(lines, req);
    return this.call(lines) as unknown as PlayResponse;
  }

  bid(req: BidRequest): BidResponse {
    const lines = [
      'walt2 bid',
      `hand ${req.hand.join(' ')}`,
      `need ${req.need}`,
    ];
    if (req.theta) lines.push(`theta ${req.theta[0]} ${req.theta[1]}`);
    pushCommon(lines, req);
    return this.call(lines) as unknown as BidResponse;
  }

  declare(req: DeclareRequest): DeclareResponse {
    const lines = [
      'walt2 declare',
      `hand ${req.hand.join(' ')}`,
      `bid ${req.bid}`,
    ];
    pushCommon(lines, req);
    return this.call(lines) as unknown as DeclareResponse;
  }
}

function pushCommon(
  lines: string[],
  req: { n?: number; n0?: number; seed?: bigint | number; budgetMs?: number },
): void {
  if (req.n !== undefined) lines.push(`n ${req.n}`);
  if (req.n0 !== undefined) lines.push(`n0 ${req.n0}`);
  if (req.seed !== undefined) lines.push(`seed ${req.seed.toString()}`);
  if (req.budgetMs !== undefined) lines.push(`budget_ms ${req.budgetMs}`);
}
