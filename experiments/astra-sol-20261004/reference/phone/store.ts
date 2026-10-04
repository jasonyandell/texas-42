/**
 * UI store: a small pure reducer that orchestrates the engine.
 *
 * ALL game logic lives in src/engine — this file only:
 *   - routes screens and settings,
 *   - applies human actions via applyAction,
 *   - steps one AI action per 'ai' event (scheduling/timers live in App.tsx,
 *     so everything here is testable without timers),
 *   - pauses briefly after a completed trick so the table can show it.
 */

import type { Difficulty } from '../ai';
import { chooseAction } from '../ai/table';
import { catalogueDeal } from '../ai/catalogue';
import { isNative, requestOf, requestKey, checkedAction, type NativeReceipt, type FlagRecord } from '../ai/native';
import { auctionKey, type AuctionDecision, type AuctionEvidence } from '../ai/auction';
import {
  type Action,
  type Bid,
  type Contract,
  type Declaration,
  type GameConfig,
  type GameState,
  type PlayRecord,
  type Seat,
  CALLED_SUIT,
  CASUAL_CONFIG,
  TOURNAMENT_CONFIG,
  PLUNGE_CONFIG,
  applyAction,
  fromId,
  ledSuitOf,
  mulberry32,
  newGame,
  teamOf,
} from '../engine';

export const HUMAN_SEAT = 0 as Seat;

/** Seat 0 = you (bottom). Clockwise: 1 = left, 2 = across, 3 = right. */
export const SEAT_NAMES: readonly [string, string, string, string] = [
  'You',
  'Earl',
  'Gran',
  'Ruby',
];

export type Preset = 'casual' | 'tournament';

export interface Settings {
  readonly difficulty: Difficulty;
  readonly preset: Preset;
  readonly thinkDeeper: boolean;
}

export const DEFAULT_SETTINGS: Settings = { difficulty: 'native-partner', preset: 'tournament', thinkDeeper: false };

/** Rotate the bidder with the shaker; use real engine auction transitions. */
export function practice30(game: GameState): GameState {
  if (game.phase !== 'bidding' || game.bids.length) return game;
  game = applyAction(game, { type: 'bid', bid: { kind: 'points', value: 30 } });
  for (let i = 0; i < 3; i++) game = applyAction(game, { type: 'bid', bid: { kind: 'pass' } });
  return game;
}

export function configFor(preset: Preset): GameConfig {
  return preset === 'tournament' ? TOURNAMENT_CONFIG : CASUAL_CONFIG;
}

export type Screen = 'home' | 'table' | 'how' | 'about';

export interface AppState {
  readonly screen: Screen;
  readonly settings: Settings;
  readonly seed: string;
  readonly game: GameState | null;
  /** Count of AI actions taken this game — seeds the AI's deterministic rand. */
  readonly aiMoves: number;
  /** True while the just-completed trick is being shown before play resumes. */
  readonly showTrick: boolean;
  /**
   * A shared hand opened from a link — view-only review. Displayed instead
   * of `game` on the table, never persisted, never steps the AI, and the
   * player's own in-progress game stays untouched underneath.
   */
  readonly scenarioGame: GameState | null;
  readonly sessionId: string;
  readonly nativeReceipts: Record<string, string>;
  readonly scenarioFlag: FlagRecord | null;
  readonly auctionSurveys: Record<string, AuctionEvidence>;
}

export type ChooseFn = typeof chooseAction;

export type AppEvent =
  | { readonly type: 'go'; readonly screen: Screen }
  | { readonly type: 'set-difficulty'; readonly difficulty: Difficulty }
  | { readonly type: 'set-preset'; readonly preset: Preset }
  | { readonly type: 'set-think-deeper'; readonly enabled: boolean }
  | { readonly type: 'new-game'; readonly seed: string; readonly sessionId?: string }
  | { readonly type: 'resume' }
  | { readonly type: 'human'; readonly action: Action }
  /** Step exactly one AI action (if one is pending). `choose` is injectable for tests. */
  | { readonly type: 'ai'; readonly choose?: ChooseFn | undefined }
  | { readonly type: 'trick-shown' }
  /** Open a shared hand (from a share link) in view-only review. */
  | { readonly type: 'view-scenario'; readonly game: GameState; readonly flag?: FlagRecord }
  | { readonly type: 'native-ai'; readonly receipt: NativeReceipt }
  | { readonly type: 'auction-ai'; readonly decision: AuctionDecision };

export function initialApp(saved?: SavedState | null): AppState {
  if (saved && !isNative(saved.settings.difficulty)) saved = null;
  return {
    screen: 'home',
    settings: { ...DEFAULT_SETTINGS, ...saved?.settings },
    seed: saved?.seed ?? 'plunge',
    // Apply the house rule to a resumed auction; preserve already-played hands.
    game: saved?.game?.phase === 'bidding'
      ? { ...saved.game, config: PLUNGE_CONFIG } : saved?.game ?? null,
    aiMoves: saved?.aiMoves ?? 0,
    showTrick: false,
    scenarioGame: null,
    scenarioFlag: null,
    sessionId: saved?.sessionId ?? 'legacy',
    nativeReceipts: saved?.nativeReceipts ?? {},
    auctionSurveys: saved?.auctionSurveys ?? {},
  };
}

/** Did `next` complete a trick mid-hand (worth pausing to look at)? */
export function trickJustCompleted(prev: GameState, next: GameState): boolean {
  return next.tricks.length > prev.tricks.length && next.phase === 'playing';
}

/**
 * The AI seat that should act next, or null. Null while the human is up,
 * while a finished trick is on display, off the table screen, and in
 * hand-over / game-over (advancing to the next hand is the human's tap).
 */
export function pendingAiSeat(s: AppState): Seat | null {
  const g = s.game;
  if (!g || s.screen !== 'table' || s.showTrick || s.scenarioGame) return null;
  if (g.phase !== 'bidding' && g.phase !== 'declaring' && g.phase !== 'playing') return null;
  if (g.turn === null || g.turn === HUMAN_SEAT) return null;
  return g.turn;
}

/** Deterministic rand for the nth AI action of a game. */
export function aiRand(game: GameState, aiMoves: number): () => number {
  return mulberry32((game.rngState ^ Math.imul(aiMoves + 1, 0x9e3779b9)) >>> 0);
}

export function reducer(s: AppState, e: AppEvent): AppState {
  switch (e.type) {
    case 'go':
      // Leaving for home closes any shared-hand review.
      return { ...s, screen: e.screen, scenarioGame: e.screen === 'home' ? null : s.scenarioGame,
        scenarioFlag: e.screen === 'home' ? null : s.scenarioFlag };
    case 'set-difficulty':
      return { ...s, settings: { ...s.settings, difficulty: e.difficulty,
        preset: isNative(e.difficulty) ? 'tournament' : s.settings.preset } };
    case 'set-preset':
      return { ...s, settings: { ...s.settings, preset: e.preset } };
    case 'set-think-deeper':
      return { ...s, settings: { ...s.settings, thinkDeeper: e.enabled } };
    case 'new-game':
      return {
        ...s,
        screen: 'table',
        seed: e.seed,
        aiMoves: 0,
        showTrick: false,
        scenarioGame: null,
        scenarioFlag: null,
        sessionId: e.sessionId ?? `seed-${e.seed.replace(/[^a-zA-Z0-9_-]/g, '').slice(0,60) || 'game'}`,
        nativeReceipts: {},
        auctionSurveys: {},
        game: isNative(s.settings.difficulty)
          ? catalogueDeal(newGame(PLUNGE_CONFIG,e.seed),e.seed)
          : newGame(configFor(s.settings.preset),e.seed),
      };
    case 'resume':
      return s.game ? { ...s, screen: 'table', scenarioGame: null, scenarioFlag: null } : s;
    case 'view-scenario':
      return { ...s, screen: 'table', showTrick: false, scenarioGame: e.game, scenarioFlag: e.flag ?? null };
    case 'trick-shown':
      return { ...s, showTrick: false };
    case 'human': {
      if (!s.game || s.scenarioGame) return s; // shared hands are view-only
      let game: GameState;
      try {
        game = applyAction(s.game, e.action);
        if (e.action.type === 'next-hand' && isNative(s.settings.difficulty)) game=catalogueDeal({ ...game, config: PLUNGE_CONFIG },s.seed);
      } catch {
        return s; // defensive: stale tap / double tap — ignore
      }
      return { ...s, game, showTrick: trickJustCompleted(s.game, game),
        auctionSurveys: e.action.type === 'next-hand' ? {} : s.auctionSurveys };
    }
    case 'auction-ai': {
      const seat=pendingAiSeat(s), g=s.game, d=e.decision;
      if (seat===null || !g || !['bidding','declaring'].includes(g.phase) || d.key!==auctionKey(g,s.sessionId)) return s;
      if ((g.phase==='bidding' && d.action.type!=='bid') || (g.phase==='declaring' && d.action.type!=='declare')) return s;
      try {
        const game=applyAction(g,d.action);
        return {...s,game,aiMoves:s.aiMoves+1,auctionSurveys:d.survey ? {...s.auctionSurveys,[`${g.handNumber}:${seat}`]:d.survey} : s.auctionSurveys};
      } catch { return s; }
    }
    case 'native-ai': {
      const seat = pendingAiSeat(s);
      if (seat === null || !s.game || s.game.phase !== 'playing' || !isNative(s.settings.difficulty)) return s;
      const req = requestOf(s.game, seat, s.sessionId);
      const wanted = s.settings.difficulty === 'native-l1' ? 'l1-default' : 'l1-partner-rollout';
      if (requestKey(req) !== requestKey(e.receipt.identity.request) || e.receipt.identity.game_id !== s.sessionId
        || e.receipt.identity.hand_number !== s.game.handNumber || e.receipt.identity.player.name !== wanted) return s;
      const game = applyAction(s.game, checkedAction(s.game, req, e.receipt));
      const key = `${s.game.handNumber}:${req.plays.length / 2}`;
      return { ...s, game, aiMoves: s.aiMoves + 1, showTrick: trickJustCompleted(s.game, game),
        nativeReceipts: { ...s.nativeReceipts, [key]: e.receipt.id } };
    }
    case 'ai': {
      const seat = pendingAiSeat(s);
      if (seat === null || !s.game) return s;
      const choose = e.choose ?? chooseAction;
      const action = choose(s.game, seat, s.settings.difficulty, aiRand(s.game, s.aiMoves));
      const game = applyAction(s.game, action);
      return {
        ...s,
        game,
        aiMoves: s.aiMoves + 1,
        showTrick: trickJustCompleted(s.game, game),
      };
    }
  }
}

// ---------------------------------------------------------------------------
// Timing (used by App.tsx; pure functions of state so they're testable)
// ---------------------------------------------------------------------------

/** Readable AI pacing, ~500–900ms, with a little deterministic variety. */
export function aiDelayMs(s: AppState): number {
  const base = s.game?.phase === 'declaring' ? 700 : 550;
  return base + (s.aiMoves % 4) * 80; // 550..790 / 700..940 capped below
}

/** How long a completed trick stays on display (interruptible by human taps). */
export const TRICK_SHOW_MS = 850;

// ---------------------------------------------------------------------------
// Persistence (storage-agnostic so tests can pass a fake)
// ---------------------------------------------------------------------------

export interface SavedState {
  readonly v: 1;
  readonly settings: Settings;
  readonly seed: string;
  readonly game: GameState | null;
  readonly aiMoves: number;
  readonly sessionId?: string;
  readonly nativeReceipts?: Record<string, string>;
  readonly auctionSurveys?: Record<string, AuctionEvidence>;
}

export interface StorageLike {
  getItem(key: string): string | null;
  setItem(key: string, value: string): void;
  removeItem(key: string): void;
}

export const STORAGE_KEY = 'plunge:save:v1';

export function toSaved(s: AppState): SavedState {
  return { v: 1, settings: s.settings, seed: s.seed, game: s.game, aiMoves: s.aiMoves,
    sessionId: s.sessionId, nativeReceipts: s.nativeReceipts, auctionSurveys: s.auctionSurveys };
}

export function saveApp(storage: StorageLike, s: AppState): void {
  try {
    storage.setItem(STORAGE_KEY, JSON.stringify(toSaved(s)));
  } catch {
    // storage full / private mode — losing the save is fine
  }
}

const DIFFICULTIES: readonly Difficulty[] = ['easy', 'medium', 'hard', 'onyx', 'walt', 'native-l1', 'native-partner'];
const PRESETS: readonly Preset[] = ['casual', 'tournament'];

export function loadApp(storage: StorageLike): SavedState | null {
  try {
    const raw = storage.getItem(STORAGE_KEY);
    if (!raw) return null;
    const p = JSON.parse(raw) as SavedState;
    if (p === null || typeof p !== 'object' || p.v !== 1) return null;
    if (!DIFFICULTIES.includes(p.settings?.difficulty)) return null;
    if (!PRESETS.includes(p.settings?.preset)) return null;
    if (p.settings.thinkDeeper !== undefined && typeof p.settings.thinkDeeper !== 'boolean') return null;
    if (typeof p.seed !== 'string' || typeof p.aiMoves !== 'number') return null;
    if (p.sessionId !== undefined && (typeof p.sessionId !== 'string' || !/^[a-zA-Z0-9_-]{1,80}$/.test(p.sessionId))) return null;
    if (p.nativeReceipts !== undefined && (typeof p.nativeReceipts !== 'object' || p.nativeReceipts === null
      || Object.entries(p.nativeReceipts).some(([k,v]) => !/^\d+:\d+$/.test(k) || typeof v !== 'string' || !/^[a-f0-9]{64}$/.test(v)))) return null;
    if (p.game !== null) {
      const g = p.game;
      if (typeof g !== 'object' || typeof g.phase !== 'string') return null;
      if (!Array.isArray(g.hands) || g.hands.length !== 4) return null;
      if (!Array.isArray(g.marks) || g.marks.length !== 2) return null;
    }
    return { ...p, settings: { ...p.settings, thinkDeeper: p.settings.thinkDeeper ?? false } };
  } catch {
    return null;
  }
}

// ---------------------------------------------------------------------------
// Friendly copy (pure, testable)
// ---------------------------------------------------------------------------

export const PIP_SUIT_NAMES: readonly string[] = [
  'blanks', 'aces', 'deuces', 'treys', 'fours', 'fives', 'sixes',
];

export function bidLabel(bid: Bid): string {
  switch (bid.kind) {
    case 'pass':
      return 'Pass';
    case 'points':
      return String(bid.value);
    case 'marks': {
      const base = bid.value === 1 ? '1 mark' : `${bid.value} marks`;
      if (bid.special === 'plunge') return `Plunge! (${base})`;
      if (bid.special === 'splash') return `Splash! (${base})`;
      if (bid.special === 'nello') return `Nel-O (${base})`;
      return base;
    }
  }
}

export function contractLabel(c: Contract): string {
  const marks = c.kind === 'points' ? 1 : c.value;
  const m = marks === 1 ? '1 mark' : `${marks} marks`;
  switch (c.kind) {
    case 'points': return `${c.value}`;
    case 'marks': return m;
    case 'nello': return `Nel-O, ${m}`;
    case 'plunge': return `Plunge, ${m}`;
    case 'splash': return `Splash, ${m}`;
    case 'sevens': return `Sevens, ${m}`;
  }
}

export function declLabel(d: Declaration): string {
  switch (d.type) {
    case 'pip': return PIP_SUIT_NAMES[d.pip] ?? String(d.pip);
    case 'doubles': return 'doubles';
    case 'no-trump': return 'follow me';
    case 'nello': return 'Nel-O';
    case 'sevens': return 'sevens';
  }
}

/**
 * Trump chip for the in-play info bar — what was called, kept short.
 * Spells out the doubles treatment for no-trump and Nel-O, where it matters.
 */
export function trumpChip(g: GameState): string | null {
  const d = g.declaration;
  if (!d) return null;
  switch (d.type) {
    case 'pip':
      return `trump: ${PIP_SUIT_NAMES[d.pip]}`;
    case 'doubles':
      return 'trump: doubles';
    case 'no-trump':
      switch (g.config.noTrumpDoubles) {
        case 'high': return 'no trump — doubles high';
        case 'low': return 'no trump — doubles low';
        case 'own-suit': return 'no trump — doubles own suit';
      }
      break;
    case 'nello':
      switch (g.config.nelloDoubles) {
        case 'own-suit': return 'Nel-O — doubles own suit';
        case 'high': return 'Nel-O — doubles high';
        case 'low': return 'Nel-O — doubles low';
        case 'own-suit-inverted': return 'Nel-O — doubles own suit, 0-0 high';
      }
      break;
    case 'sevens':
      return 'Sevens — closest to 7 wins';
  }
}

/**
 * The suit the displayed trick's lead calls for ("sixes", "trumps",
 * "doubles"), or null when nothing is on the table.
 */
export function ledChip(g: GameState, plays: readonly PlayRecord[]): string | null {
  const lead = plays[0];
  if (!lead || !g.rules) return null;
  const led = ledSuitOf(fromId(lead.domino), g.rules);
  if (led === CALLED_SUIT) {
    return g.rules.called.kind === 'doubles' ? 'doubles' : 'trumps';
  }
  return PIP_SUIT_NAMES[led] ?? null;
}

/**
 * Shown (with animated dots) while a slow AI — walt at an opening lead, or
 * pricing an auction — is genuinely computing, so a long pause reads as
 * thought, not a hang.
 */
export function thinkingCopy(seat: Seat): string {
  return `${SEAT_NAMES[seat] ?? 'Somebody'}'s thinking it over`;
}

export interface HandOverCopy {
  readonly title: string;
  readonly detail: string;
}

export function handOverCopy(g: GameState): HandOverCopy {
  if (g.thrownIn) {
    return {
      title: 'Nobody wanted it',
      detail: "All four passed — shake 'em up and go again.",
    };
  }
  const r = g.handResult;
  if (!r) return { title: 'Hand over', detail: '' };
  const declTeam = teamOf(r.declarer);
  const declName = SEAT_NAMES[r.declarer] ?? 'Somebody';
  const marks = r.marks === 1 ? '1 mark' : `${r.marks} marks`;
  const usWon = r.team === 0;
  if (r.made) {
    if (declTeam === 0) {
      const who = r.declarer === HUMAN_SEAT ? 'You made it!' : `${declName} made it!`;
      return { title: who, detail: `${cap(r.reason)}. ${marks} for us.` };
    }
    return { title: 'They made it', detail: `${cap(r.reason)}. ${marks} their way.` };
  }
  if (usWon) {
    const held =
      r.contract.kind === 'points'
        ? `Set! Y'all held 'em to ${g.points[declTeam] ?? 0}`
        : 'Set!';
    return { title: held, detail: `${cap(r.reason)}. ${marks} for us.` };
  }
  return { title: 'Set.', detail: `${cap(r.reason)}. ${marks} their way.` };
}

export function gameOverCopy(g: GameState): HandOverCopy {
  if (g.winner === 0) {
    return {
      title: "Y'all win!",
      detail: 'Gran gives you a wink across the table. That makes ALL.',
    };
  }
  return {
    title: "They got y'all this time",
    detail: 'Earl and Ruby tip their hats. Shake it back and run it again.',
  };
}

function cap(s: string): string {
  return s.length > 0 ? s[0]!.toUpperCase() + s.slice(1) : s;
}
