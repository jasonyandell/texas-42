[Home](Home.md) · owns: how walt plays — the seat's one decision procedure and the three axes every variant is a point in; every configuration of the level-1 player that coexists (arena, phone, browser, tables, experiments, fallbacks); the 2026-08-17 arena match against the E[Q] champion; the level-2 question; bidding and declaring; the variant seats that are not the default (controller, waking, unified); the play surfaces and the gates that pin their determinism; changes to the live code since 2026-08-25; debts and honest gaps — 2026-08-17 → 2026-09-07 at c00717d1 · Sources: [`walt/SCENARIO-PLAYER.md`](../walt/SCENARIO-PLAYER.md) and [`walt/CONTROLLER-PLAYER.md`](../walt/CONTROLLER-PLAYER.md) (the specs); result files [`walt/probes/m3/`](../walt/probes/m3/) (`arena_results_2026-08-17.txt`, `level2_results_2026-08-17.txt`, `divergence_results_2026-08-18.txt`, `level1_results_2026-08-17.txt`, `scenario_results_2026-08-17.txt`, `ladder_results_2026-08-17.txt`, `sampling_results_2026-08-17.txt`), [`walt/probes/bidcurve/ANALYSIS-2026-08-19.txt`](../walt/probes/bidcurve/ANALYSIS-2026-08-19.txt), `walt/probes/tilt_arena_2026-08-19.log`, [`walt/probes/waking/README.md`](../walt/probes/waking/README.md), [`walt/probes/gran/README.md`](../walt/probes/gran/README.md), `walt/probes/shadow/README.md`; [`walt/TILT-AUDIT.md`](../walt/TILT-AUDIT.md), [`walt/LEVEL2-PROBE.md`](../walt/LEVEL2-PROBE.md); source `walt/walt/src/solver/{mod,selection,inner_belief,act,waking,partnership}.rs`, `walt/walt/src/bin/{walt_bridge,webtable,playtable,playout,controller_bridge,waking_bridge,granrun,partnership,level1,level2,divergence,bidcurve,tiltaudit}.rs`, `walt/walt-wasm/src/api.rs`, `walt/walt2-wasm/src/api.rs` and the two `pkg/README.md`; `experiments/partnership/{players.json,PLAYERS.md,BASELINE.md,FOUNDATION.md,INNER-BELIEF.md,SCOPE.md}` and its campaign `RESULTS.md` files; `walt/briefs/BRIEF-SIGMA1-REPAIR.md`, `walt/briefs/MORNING-2026-09-05.md`, `walt/briefs/UP0-REPORT.md`; rulings in `walt/CENSUS-RULINGS.md` (CE-A7, CBS-A9, APS-A9, MB-A7, FH-A10). Related: [walt](walt.md) (hub and fence), [walt-program](walt-program.md), [walt-instruments](walt-instruments.md), [walt-architecture](walt-architecture.md), [walt-calculated-evidence](walt-calculated-evidence.md), [walt-focal-horizon-era](walt-focal-horizon-era.md), [walt-gran-anchors](walt-gran-anchors.md), [walt-partnership-program](walt-partnership-program.md), [walt-gym](walt-gym.md), [walt-negative-results](walt-negative-results.md), [lineage](lineage.md), [rob](rob.md).

# How walt plays

**Tier: EXPLORATORY, throughout.** Everything on this page sits below the
corpus, kernel, exchange-CONFIRMED and rob-receipt tiers and is cited by
nothing above it. Four fences hold on every number here. (1) *Exact on the
sample, an estimate off it*: walt's value at a decision is an exact
rational computed over the worlds it sampled; as a statement about the
whole fiber it carries unquantified sampling error (`SCENARIO-PLAYER.md`
§7, obligation O6). (2) *Model-relative, never game-theoretic*: a level-k
walt is a best response to a named level-(k−1) field model; raising k
changes the model, not the game; nothing here is an equilibrium,
convergence or monotone-improvement claim (§7, O36). (3) *Support is not
belief*: the sampled fiber is the set of lawful completions of the seat's
information, never a posterior over what any opponent would actually do
(§4, Remark 4.4). (4) *An arena or campaign outcome is a receipt about
play against a modeled field, never a statement about an exact value, and
no gate anywhere pins strength* — the gates listed in §7 below pin
lawfulness, determinism and byte-identity only. The obligations ledger
(`SCENARIO-PLAYER.md` §10, summarized in §9) is the path by which any of
this could graduate; nothing graduates by being on this page.

The reader this chapter serves: a newcomer who wants to know what the
seat does at its turn; a mathematician who wants the objects and the
fences; an engineer who wants to run the player and know which of its
several configurations they are running.

## 1. The one decision procedure

Plainly: at its turn walt asks one question — *of my legal tiles, which
one gives my team the best chance of making the bid (or, on defense, of
setting it), if the other three seats play the way I model them?* It
answers by imagining many complete deals consistent with what it has
seen, playing each one out exactly against its model of the other seats,
and choosing the tile that makes in the largest share of them. Every
walt that has ever played a hand is this procedure at some setting of
three dials.

Precisely (`SCENARIO-PLAYER.md`, the spec written 2026-08-18 after the
build of 2026-08-17; the objective ruled 2026-08-17, [walt-program](walt-program.md)):

- **Information state** (Def 2.2). Seat s decides from `I_s = (h_s, R)`:
  its own remaining hand and the public record — the dated sequence of
  (seat, tile) plays, which determines the played mask, the current
  trick, the leader, the banked totals `(b₁, b₀)` and the void sets
  `V(R)`. Nothing else. No mind at any level conditions on another seat's
  tiles (obligation O1, the no-strategy-fusion invariant).
- **Belief support** (Def 4.1). `fiber(I_s)` is the set of assignments of
  the unseen tiles to the other three seats with the hand sizes forced by
  R and no seat holding a tile in its void set — the lawful-completion set.
  The **outer sampler** (Def 4.2) draws `n_outer` worlds uniformly from it
  by shuffle-and-reject; since the σ1 repair of 2026-09-02 (§8) it first
  runs an exact Hall-condition feasibility check and returns a typed
  refusal on an infeasible frame instead of spinning.
- **Objective** (Def 6.2, pmake). With bid b held by team T1, *make ⇔
  banked(T1) ≥ b* (for b = 30, make ⇔ banked(T0) ≤ 12). The value of a
  position is P(make) under the model: a `BigRational`, exact on the
  sample, with decided cutoffs (value 1 once banked(T1) ≥ b, value 0 once
  banked(T0) > 42 − b) that are sound by count conservation. No floats
  anywhere. A trick-difference proxy is never the target. The Boolean
  has one designed-in consequence the book keeps in view: at exact
  indifference P(make) pins at 1 and has no gradient, which is the G1/G2
  lesson of §4 (`walt/briefs/MORNING-2026-09-05.md` item 4).
- **The field** — the model of the other three seats, a parameter (§3).
  *Dice* (Def 3.5): each non-viewer seat in world w plays uniformly among
  its legal tiles by a deterministic tickertape `SplitMix64(seed_w ⊕
  hash(κ(R))) mod |legal|`, keyed on the record so the same world plays
  the same tile in every branch and worlds partition instead of
  multiplying branches. *Level-0 mind* (Def 3.2): draws `n₀` worlds from
  its own information state and best-responds to Dice. *Level-k mind*
  (Def 3.3): draws `n_k` worlds and best-responds to level-(k−1) minds;
  the stack bottoms out at Dice. *Level-k walt* (Def 6.1) is the real
  seat with field = level-(k−1) minds: level 1 is the match champion and
  the live default; level 2 is the first level whose modeled partner
  coordinates back. Since 2026-09-06, `Field::SeatLevels` assigns a level
  per seat (`solver/partnership.rs`: `FieldProfile::{Baseline,
  PartnerOnly, AllLevel1}` — all L0; partner L1 and opponents L0; all L1).
- **The solver** (§5). Sampled worlds are solved together as one tree
  keyed on the reduced record `κ(R) = (played, leader, current-trick
  plays, banked_t1, banked_t0)` plus an alive-set id (Def 2.3). At a field
  node the alive worlds partition by the modeled move (Lemma 5.1) and the
  node's value is the bucket-weighted sum — behavior-refinement Bayes
  inside the model (Lemma 5.2), with Dice at the bottom guaranteeing no
  lawful world is ever excluded (Lemma 5.3). Every modeled mind's policy
  is cached under `(k, PiKey)` and must be a pure function of that key
  (Def 3.4) — which is why banked totals sit in the key (Remark 2.5) and
  why the 2026-08-18 PiKey defect (§4) was a real bug.
- **The decision.** The argmax of P(make) over legal tiles (argmin for a
  defending seat), with the tie convention below.

### The three axes

Every variant on this page is a point in this space; the book never
says "level-1 walt" without naming a point (§2).

| axis | choices | where in the code | what the live level-1 player uses |
|---|---|---|---|
| **field model** | `Dice`; `Level(k)`; `SeatLevels` with profile Baseline / PartnerOnly / AllLevel1 | `Field` in `solver/mod.rs`; `FieldProfile` in `solver/partnership.rs` (2026-09-06) | `Level(0)` — every other seat a level-0 mind over Dice |
| **inner belief** (what a *modeled* mind samples) | `Voidless` — the no-void sizes-fiber of Def 4.3 (the declared simplification, obligation O5); `VoidsCounted` — the public-void-conditioned support via the kernel's exact `FiberDp` sampler | `solver/inner_belief.rs` (dbcc698f, 2026-09-06); chosen once per `Shared` (`with_inner_belief`), inherited by every nested level; `Key`/`PiKey` carry `voids: Option<[u32;4]>` (`None` = legacy identity, `Some([0;4])` = tracked opening, `Some(masks)` = tracked deductions) | `Voidless` (default; the phone WASM cannot select the other) |
| **selection** (how a mind picks among candidates) | `Fixed` — one bundle of n worlds, argmax; `Refine` — on an exact top tie, fresh bundles of 4× size on the tied set only, replacing (not pooling) values and reconsidering the full comparison, until the tie breaks or the size reaches 16n; `RaceRefine` — common-random-number blocks of 8 worlds up to a cap of 2n, a rival eliminated once it trails the leader on k ≥ 6 differing blocks with binomial tail ≤ 1/128, then `Refine` on any surviving tie | `solver/selection.rs` (9236ca7f, 2026-09-06); `level1_evaluate` calls `select(Rule::Refine, …)`; `Solver::pi` uses `Fixed` at k = 0 against Dice and `Shared::modeled_selection` (default `Fixed`) at k ≥ 1 | root `Refine` in `level1_evaluate` (arena, tables, browser default); `RaceRefine` where `race` is on (the phone); `Fixed` in the experiments' `l1-default` |

The outer sample count `n_outer` and the per-level inner counts `n₀, n₁,
…` are declared per run and are part of every result's identity
(Def 3.3). Seeds are frozen constants mixed with structural coordinates
— seat, own dealt hand, record hash, level tag — never a wall clock or
global RNG (Def 3.6): `INNER_SEED = 0x243F_6A88_85A3_08D3` is shared by
every surface so level-0 minds are seeded bit-identically across
`level1.rs`, `level2.rs`, the bridge and the tables; the level tag is 0 at
k = 0 and `mix(0x4C32 ^ k)` above; the per-decision outer stream on the
arena bridge is `BRIDGE_SEED ^ mix(dealt hand) ^ record_hash(key)`. A
decision is therefore a pure function of the information state and the
declared epoch: reruns reproduce it bit for bit, and wall times are the
only thing that varies (`arena_results_2026-08-17.txt`, honesty notes).

### The tie convention, stated honestly

Def 6.3 says a saturation tie (several candidates equal at the top,
typically at 1-on-sample) is "never broken by tile index: tied candidates
are re-evaluated on fresh, larger samples (4× per round, bounded) until
separated or the bound is hit" — and as written on 2026-08-18 the spec
did not say what happens at the bound. The code does (`selection::refine`: `if ties.len() == 1 ||
size >= cap { break; }`, then `best_of`; `best_of`'s reduce replaces the
incumbent only on *strict* improvement, "first-listed on exact ties — the
ascending-tile-order convention of the whole stack"; candidates are
passed as `mask_bits(legal)`, ascending). So the honest statement is:
**saturation ties are refined on 4× fresh samples up to 16× n; a tie that
survives the cap — or an exact tie under `Fixed` selection — is broken
toward the lowest tile index**, where the index is `hi·(hi+1)/2 + lo`
(the 6-4 is index 25 of 0..27, the 5-5 is 20; only 6-5 and 6-6 sit above
the 6-4). The gate `fixed_ties_use_tile_order_without_extra_sampling`
(`walt/walt/tests/solver_selection.rs`) pins the `Fixed` case;
`FieldModel::new` and `FrozenPolicy::new` assert `TieRule::LowestTileIndex`
names what the algorithm does (`solver/policy.rs`, `solver/act.rs`
`act_field_spec` / `continuation_tuple`). The `webtable.rs`/`playtable.rs`
headers and Def 6.3's own sentence ("never index-broken") overstate the
rule for the post-cap case; the correction was read from source on
2026-09-05 (`walt/briefs/MORNING-2026-09-05.md` item 2 — a source
reading, never executed at a tied node) and `SCENARIO-PLAYER.md` now
carries it as a note under Def 6.3 dated 2026-09-13 ("What happens at
the bound"). The rationale for refining first is still
right: support ≠ belief, 1-on-sample is not certainty, and an index
break injects an arbitrary preference exactly where the estimate is
least informative (the level-2 n = 200 three-way tie of §4 would have
led 1-1). Its consequence at *exact* indifference — where no worlds
remain to add and P(make) pins at 1 — is that the seat deterministically
discards count tiles like the 6-4 last; that is the Gran finding, owned
by [walt-gran-anchors](walt-gran-anchors.md) and summarized in §4.

## 2. The configurations that coexist

There is one procedure and one library authority for its parts
(`solver::level1_evaluate`, `solver::sample_belief`, `solver::selection`,
`solver::best_of`), but at least ten distinct settings of it act from
trick 1 somewhere in the repository. The table is read from source at
c00717d1 (paths under `walt/walt/src/` unless stated; every default is a
source constant, none is a gate assertion). n / n₀ / n₁ = outer worlds /
level-0 inner worlds / level-1 inner worlds.

| surface | n / n₀ (/ n₁) | root selection | bid rule | declaration rule | per-move budget | seed constant(s) | entry point |
|---|---|---|---|---|---|---|---|
| **`walt_bridge`** — the mk5 arena seat, the 3×384 champion | 50 / 8 | local copy of the 4×/16× refine loop (`walt_bridge.rs:572`) | bid-value-blind: always P(make 30) | `declare bidder h0..h6` → argmax P(make 30) over pip trumps 0..6 (doubles 7 / no-trump 9 only with `WALT_DECLARE_FULL=1`) from `n_declare` = 100 open-belief worlds, 4×/16× refinement (`walt_bridge.rs:916–928`) | 120 s | `BRIDGE_SEED 0xB7E1_5162_8AED_2A6B`, `INNER_SEED` | `walt_bridge [n_outer=50] [n0=8] [per_move_secs=120] [n_declare=100]` or env `WALT_N_OUTER / WALT_N0 / WALT_PER_MOVE / WALT_N_DECLARE`; arena spec `rob:<path>`; `WALT_BRIDGE_LOG=<base>` writes JSONL per decision |
| **plunge's `walt.wasm` — "the phone"** (artifact SHA-256 `af0200af…` = `walt/walt-wasm/pkg/walt.wasm` at 9a056f20, 2026-08-19) | 40 / 8 | **race on** (`race: true` → `level1_race_refined`: block race to cap 2n = 80, refine base 40) | plunge's own heuristic `mediumBid` ladder — *not* walt (`experiments/partnership/BASELINE.md`) | walt's `declare` at 40/8 | `budgetMs` inert in wasm | plunge-side: FNV-1a 32-bit of `walt/${handNumber}/${shaker}/${marks[0]}-${marks[1]}` (public state) | plunge `src/ai/walt/` (copied at plunge 1810da20, 2026-08-22); preserved copy `experiments/partnership/reference/phone/` |
| **`walt-wasm` repository default** (`walt-wasm/src/api.rs`; committed `pkg/walt.wasm` SHA-256 `d7f61f22…`, *not* the phone's bytes) | 40 / 8 | `Refine` (`level1_evaluate`); `race 0` | `walt1 bid`: price all nine declarations at `need` (default 30) over n = 40 common open worlds; pass if best < θ, else walk that declaration's bid up while P(make b+1) ≥ θ; θ default 11/16, a request parameter | `walt1 declare`: argmax over nine declarations at the contract bid, 4×/16× refinement | `budget_ms` 120000, inert in wasm | `DEFAULT_SEED 0xB7E1_5162_8AED_2A6B` | string-line API (§7); `sh walt-wasm/build.sh`; `node walt-wasm/smoke.mjs` |
| **`webtable`** — localhost human table with a real auction | 100 / 8 | `Refine` (`level1_evaluate`) | AI seats price nine declarations at the minimum viable bid over `n_auct = clamp(n_outer/2, 24, 60)` open worlds (50 at the default), pass if best < 11/16, else walk up; deal rotates so the winner sits at internal S1 | trump by best-lead pricing with 16× tie refinement (`webtable.rs:555`) | 120 s (fixed) | `WEB_BELIEF_SEED 0x3F84_D5B5_B547_0917`, `WEB_AUCT_SEED 0xBE54_66CF_34E9_0C6C`, `WEB_TRUMP_SEED 0xC0AC_29B7_C97C_50DD` | `webtable [port=4242] [n_outer=100] [n0=8] [seed=42] [human_seat=0] [ctrl] [cap=N]`; open `http://127.0.0.1:<port>` |
| **`playtable`** — terminal table | 100 / 8 | local copy of the refine loop (`playtable.rs:662`) | none: contract frozen to receipt hand 8 — fives, P30 by T1, S1 leads; `fresh` re-deals | none (frozen) | 180 s (fixed) | `INNER_SEED`, `TABLE_BELIEF_SEED 0x9216_D5D9_8979_FB1B` | `playtable [human_seat=0] [n_outer=100] [n0=8] [seed=42] [fresh] [ctrl] [cap=N]` |
| **experiments `l1-default`** (= `l1-fixed`) | 40 / 8 (n₁ = 2 inactive) — these are the Python driver's defaults (`experiments/partnership/players.json`); the native `partnership` bin has no defaults of its own and takes `n` / `n0` / `n1` / `budget_ms` per request within `n ≤ 640`, `n0, n1 ≤ 64`, `budget_ms ≤ 14 000` (`bin/partnership.rs`) | `Fixed` / modeled `Fixed` | none: bid fixed at 30 | fixed by the fixture rule or the match protocol (heuristic trump); never an auction | 14 000 ms wrapper, with an 8/2 level-1 reserve prepared first (≤ 1.5 s) | request `seed` (e.g. 420600); deals from Python `random.Random(seed)` | `python3 experiments/partnership/player.py --mode baseline`; native `partnership [--stream]` (§7) |
| experiments `l1-race` / `l1-refine` | 40 / 8 | `RaceRefine` (cap 80) / `Refine` | as above | as above | 14 000 ms | as above | `--selection race-refine` / `refine`; `l1-race` is the procedural anchor that reproduced the phone (§4, §8) |
| **controller fallback** inside `solver::act` (`controller_bridge`, `ctrl` seats) | 200 / 8 (the live ordering); frozen candidates at 8 / 2 | `Refine` via `level1_evaluate` among survivors or tied maxima | none (bridge is bid-blind) | `declare` kept from `walt_bridge` through the library solver | `per_move` 120 s; controller `world_cap` 128 interactive / 512 batch, `exact_cap` 2000 | `ACT_FALLBACK_SEED 0x4528_21E6_38D0_1377` | `controller_bridge [world_cap=128] [exact_cap=2000] [fallback_n_outer=200] [fallback_n0=8] [n_declare=100] [per_move=120]`; env `WALT_CTRL_WORLD_CAP` / `WALT_CTRL_EXACT_CAP` / `WALT_N_OUTER` / `WALT_N0` / `WALT_N_DECLARE` / `WALT_PER_MOVE`, frozen schedule `WALT_CTRL_N_OUTER_FROZEN` 8 / `WALT_CTRL_N0_FROZEN` 2 (env only), log `WALT_CTRL_LOG` |
| **σ1 inside the waking seat** (`solver::waking`, `waking_bridge`, `granrun`) | σ0 = `Level0{n₀ = 2}`; σ1 = `Level1{n_outer = 4, n₀ = 2}`; candidates at [8, 2] | act's baseline (as above); σ1 = `level1_evaluate` per non-focal seat | none | level-1 auction policy (bid 30 in driven mode) | wake check 24 paired worlds, exact route at fiber ≤ 1024; escalation exact cap 4096 | `WAKING_DRIVEN_SEED 0x51EE_D42A_11FE_600D`, `WAKING_DECLARE_SEED 0x7A3E_9B21_5C48_D6F1` | `waking_bridge [controller knobs]`; `waking_bridge driven <out.jsonl> [n_hands]`; `granrun replay|driven` |
| **`walt2-wasm`** — level 2 in the browser | 8 / 2 / n₁ = 4 | `Refine` over `Field::Level(1)`, `n_inner = [n₀, n₁]`, same outer worlds and seed formula as `walt-wasm` (CRN across levels) | byte-identical to `walt-wasm`'s `bid` (pinned by `auction_matches_walt1`) | byte-identical to `walt-wasm`'s `declare` | inert | `DEFAULT_SEED` as above | `walt2 play` with the extra knob `n1`; response carries `level: 2`; never a default (PR #58, 2026-08-25) |
| **experiments `l2-partner-default`** (family "L2 Partner") / `l2-partner-voids` | 40 / 8 / 2 (`players.json` defaults, same bin bounds as above) | `Fixed` / modeled `Fixed`; inner belief `voidless` / `voids-counted` | none (bid 30) | fixed, as above | 14 000 ms | as `l1-default` | `--mode partner [--inner-belief voids-counted]`; `--mode all-l1` is the separate family "L2 All" |

Probe epochs that are not players but appear in results: `level1.rs`
(fixed carrier, receipt hand 8; up to n = 2000 / n₀ = 16 in
`level1_results_2026-08-17.txt`), `level2.rs` (fixed carrier, `Field::Level(1)`,
**no tie refinement by design**; the ladder of record at n = 200/800/3200
with n₁ = 8, n₀ = 4; `OUTER_SEED 0x8CB9_2BA7_2F3D_8DD7`), `divergence.rs`
(four level-1 walts at 40 / 8 with one 4× refinement, a level-2 shadow at
40 / 8 / 4 on common random numbers; `MINER_SEED 0x9216_D5D9_8979_FB1B`),
`playout.rs` (viewer JSON; level-1 at 200 / 8 with a stale PiKey, §9).

Three consequences the book keeps in view. First, **the arena champion,
the phone and the experiments' "L1 default" are three different
procedures** (50/8 with refinement; 40/8 with racing; 40/8 fixed), and
they measurably differ: native fixed L1 lost to the preserved phone 8
favorable / 15 unfavorable / 77 ties on 50 mirrored deals (−7.0 points per
matched contract opportunity), while native `l1-race` reproduced the phone
move for move in all 35 fallback-free mirrored pairs of a 50-deal match
(`experiments/partnership/campaigns/native-l1-vs-phone-620600-649/CALIBRATION.md`,
`foundation-battery/RESULTS.md`; EXPLORATORY campaign records). Second,
**cost differs by an order of magnitude across the table**: under the
ten-game pool, `l1-fixed` 0.137 s per move, `l1-refine` 0.313 s, `l1-race`
0.502 s, `l1-race-voids` 0.535 s, the phone 0.778 s (`foundation-battery/RESULTS.md`);
level 2 at 40/8/2 fixed 1.127 s; the controller at cap 128 about ten
seconds at trick 1; the waking seat minutes per hand (§6). Third, **no
default has changed since 2026-08-19** (θ = 11/16 at 9a056f20): the seat
plunge runs and the seat the mk5 arena runs are level-1 walt at the
first two rows; everything below them is a variant (§6, §8).

## 3. The match: walt vs the E[Q] champion, 2026-08-17/18

Plainly: two days after it first played a lawful hand, level-1 walt sat
in the mk5 arena against the strongest player the prior project had
produced — the E[Q] n = 10 lens that had beaten rob by 6.5σ on 2026-07-30
([lineage](lineage.md)) — under the identical protocol, and won more
games than it lost, with the loss-of-points, gain-of-marks shape its
objective predicts. Record of record:
[`walt/probes/m3/arena_results_2026-08-17.txt`](../walt/probes/m3/arena_results_2026-08-17.txt)
(EXPLORATORY; arena outcomes are receipts about play, never receipts
about values, and no CI gate pins any number below).

**Protocol.** Harness mk5-main at 594ee5e9, `python -m arena.cli`. Team A
= bid30 + `walt_bridge` (level-1, n_outer 50, n₀ 8, 120 s per move, frozen
`BRIDGE_SEED`, the arena's rob adapter unchanged: `rob:<path-to-walt_bridge>`).
Team B = bid30 + `lens:ev n_samples=10`, device mps (the dropped-30
champion: learned double-dummy oracle `domino-qval-large-3.3M`, E[Q] over
10 uniform worlds). **Both teams declare via the arena's best-pip-trump
heuristic** (identical to the 2026-07-30 rob-vs-E[Q] protocol; walt's own
`declare` is wired but was not used). Deals mirrored across halves; one
mark per hand; games race to 7 marks. Rules conformance: every
`walt_bridge` reply carries walt's independently derived trick leader and
team points and the arena asserts them against its zeb engine on every
decision — zero divergences across pilot and match, about 15,000
decisions (a conformance cross-check, never an axiom; TRUST-01).

| run | games | walt wins | mark margin / game, 95% CI | point margin / hand | offense make rate walt vs lens | paired contracts: walt-made/lens-set vs lens-made/walt-set | McNemar z |
|---|---|---|---|---|---|---|---|
| 48-game pilot, seed 1 (527 hands, 20.8 min) | 48 | 32 (66.7%) | +0.92 [+0.06, +1.75] | −4.28 | 37.0% vs 28.5% | 36 vs 19 (of 249) | +2.29 |
| seed 1 (4295 hands, 116.6 min) | 384 | 211 (54.9%) | +0.38 [+0.05, +0.70] | −4.55 | 36.9% vs 33.6% | 235 vs 152 (of 2007) | +4.22 |
| seed 2 (4319 hands, 120.1 min) | 384 | 212 (55.2%) | +0.44 [+0.15, +0.74] | −4.82 | 36.8% vs 32.9% | 231 vs 151 (of 2036) | +4.09 |
| seed 3 (4237 hands, 139.4 min; timings contended by concurrent level-2 work) | 384 | 207 (53.9%) | +0.36 [+0.03, +0.69] | −4.74 | 37.9% vs 34.7% | 221 vs 170 (of 1972) | +2.58 |
| **pooled 3×384** | **1152** | **630 (54.7%)**, game-level z = +3.18 | mean **+0.39**; every seed's CI excludes zero | (file prints no pooled figure; "≈ −4.7/hand" reads the three seeds) | — | **687 vs 473 (of 6,015)** | **+6.28** |

The same-seed comparison the file draws: rob on 2026-07-30 won 182/384
(47.4%), mark margin −0.27 [−0.59, +0.06], made 33.0% on offense, held the
champion to 35.4%, and lost points narrowly at −0.54/hand; walt on
2026-08-17 won 211/384, made 36.9%, held the champion to 33.6%, and lost
points at −4.55/hand. **The signature: point margin negative, mark margin
positive.** walt optimizes P(make 30), the Boolean the mark pays on, and
spends meaningless points freely to buy it; rob played the Points lens
(signed differential) and lost the match at pure play. The objective, not
only the engine, is doing work — the pmake ruling of 2026-08-17 made
visible in data. A rotation-parity audit (walt offense 36.5% with the
bidder even vs 37.2% odd; defense 35.1% vs 32.2%) found no bridge parity
defect; the stronger-first-half asymmetry (61.5%/48.4% at seed 1) recurs
in all three seeds and in rob's 2026-07-30 run (55.2%/39.6%), so it is a
protocol property.

**Honesty notes, verbatim from the record.** A 4-game pilot first came
out 0/4 with the lens making 59% against walt's defense; at 48 games that
inverted decisively ("22-hand samples lie"). A hand-level forensic on
pilot g0h0 (deuces) found walt-S0 at trick 3 provably knowing the bidder
held every remaining trump and still leading 5-5 (ten count) into it,
rating that lead best (20.5%) and the junk 1-1 lead worst (40%); whether
that ordering is a level-0-model artifact or exact-over-sample truth is
UNRESOLVED. All three seeds played the identical **pre-fix** bridge
binary — the PiKey banked-aliasing fix (§4) was deliberately deferred
until the pool closed so seeds 1–3 are internally consistent; post-fix
play (1fc23196, 2026-08-18) is a new baseline, not comparable hand for
hand, and **no post-fix re-match against the E[Q] champion has been run**
(§9). walt's decisions are deterministic per information state; rerunning
the pilot reproduced it bit for bit.

**What it does and does not establish.** It establishes that this
configuration of level-1 walt, on this protocol, beats the champion in
paired marks at the bar the champion itself set against rob. It does not
establish an exact value of anything, a strength ordering among walt's
own configurations (every later variant has been measured only against
walt or the phone, §8), or the "demonstrated strategic reason" that
[lineage](lineage.md)'s winning condition demands — a plan held and cashed,
not a scoreline; that judgment is deliberately not ruled.

## 4. The level-2 question

Plainly: level 1 models the other seats as minds that each imagine the
world and best-respond to dice; level 2 models them as level-1 minds — so
for the first time walt's model of its *partner* is a mind that models
walt back and can coordinate. Does that change the play, and is it
worth the cost? The answer so far: on the frozen carrier no; in mined
self-play it moves decisions where partnership is load-bearing; at the
table it ties level 1 at about five times the cost; and at exact
indifference it cannot help at all.

**The ladder agrees (2026-08-17).** `level2.rs` on the frozen carrier
(receipt hand 8, fives, P30, S1 = viewer with {5-2, 5-4, 1-1, 2-1, 3-1,
3-3, 5-5}), n₁ = 8 / n₀ = 4, walking the boundary back toward trick 1
(`level2_results_2026-08-17.txt`, CORRECTED ladder at f5fff91; the serial
ladder at 178722e kept as correction history): t = 4 EXACT over the
1,200-deal support given the level-1 field policy — lead 3-3 at ~96.58%
(3.1 s on 18 threads), agreeing with level 1 and with the exact scenario
truth; t = 3, n = 500, 3-3 ~96.60%; t = 2 saturated at ~100% (no
decision); t = 1 (support 399M raw assignments) at n = 200 a **three-way
saturation tie 1-1 / 3-1 / 5-5 at 200/200** — a lowest-index break would
have led 1-1; at n = 800 the tie breaks, 5-5 unique at 800/800; at
n = 3200 (960.2 s, 18.4B nodes, 156M π evaluations) 5-5 still 3200/3200.
Verdict in the file: level 2 agrees with level 1 at every rung; one
carrier, one hand; "says nothing yet about hands where coordination is
load-bearing". The n = 200 tie and its break at n = 800 is the standing
argument for tie refinement (§1). Costs: the rayon port ran t = 1 n = 800
in 214.9 s against 1211.3 s serial (~5.6×), byte-identical across 1 and
18 threads; t = 4 3.1 s vs 16.0 s single-threaded. No level-2-to-level-1
cost multiplier is quotable from this record — it gives absolute timings
only (a "≈ 25–50×" figure once stated here had no source and is
withdrawn).

**The correction the parallel port caught (Def 3.4).** The serial probe's
PiKey — documented as the modeled mind's "entire information state" —
omitted the banked totals its pmake objective conditions on, which are
not derivable from the reduced record. Serial execution masked this as
deterministic first-come cache aliasing; the parallel port made the alias
racy and visible (values drifted between runs, which exact arithmetic
forbids). Fix: `banked_t1`/`banked_t0` in PiKey; results byte-identical
across 1 thread, 18 threads and reruns; argmax unchanged at every rung;
values moved by up to ~1.7 points (t = 4 lead 3-1: 85.83% → 87.50%). The
incident is the standing argument for the spec: a stated invariant,
checked at review, catches drift that testing hides. The same latent
alias in `level1.rs`, `playtable.rs`, `webtable.rs`, `walt_bridge.rs` was
fixed at 1fc23196 after the arena pool closed; `playout.rs` keeps its
local banked-less PiKey to this day, filed not patched (§9).

**Divergence mining (2026-08-18).** Rather than a head-to-head, 900
self-played hands with four level-1 walts (40 / 8, one 4× refinement) and
a level-2 shadow (40 CRN worlds, n₁ = 8 / n₀ = 4) at one seat cycling
self-bid / partner-bid / defense by hand index
(`divergence_results_2026-08-18.txt`, miner at 30f1409, corpus
`walt/probes/m3/mined/`): 4,156 shadowed decisions, zero timeouts.
Level-1's choice strictly sub-optimal on level-2's table 1154/4156 =
27.8% (self-bid 27.6% / partner-bid 26.8% / defense 28.9% — flat); by
trick t1 38.4%, t2 42.2%, t3 39.4%, t4 29.5%, t5 16.1%, t6 3.2%. The base
rate is n = 40 noise; the signal is in the tail: at gap ≥ 500 bp 12.98% /
16.73% / 17.63%, ≥ 1000 bp 4.83% / 6.38% / 6.47%, **≥ 1500 bp 1.40% /
2.18% / 2.69%; pooling partner-bid + defense against self-bid at ≥ 1500 bp,
2.44% vs 1.40% (about 1.7×), two-proportion z ≈ 2.3** — moderate,
consistent, in the predicted direction ("situations where you say 'my
partner will X'"). Gap distribution over the 1,154 divergences: 506 under
500 bp, 406 in [500, 1000), 228 in [1000, 2000), 14 at 2000 bp+; argmax
sets fully disjoint in 1,078. The top case, hand 82 (blanks trump,
partner-bid; shadow = S3 void in trump at trick 2, partner winning with
4-0 and holding both remaining trumps): level 1 plays 5-3 (65.0%) and
hoards the 4-1 five-count (62.5%); level 2 wants 4-1 at 85.0% vs 5-3 at
55.0% (gap 3000 bp) — drop the count on the partner's winning trump
trick; the hand made 31:11. Caveats that travel: self-graded on level-2's
own table (the mirrored-replay referee was never run); trajectories are
level-1 self-play, so positions level 2 would have steered into are
unexplored; and the **level mismatch in the mirror** (`SCENARIO-PLAYER.md`
§7, item 3): inside a
level-2 walt the modeled partner reads walt itself as level 0, so
signaling value found at level 2 is signaling into a simplified reader —
a lower bound on matched-reader coordination.

**walt2-wasm's cost grid (2026-08-25, PR #58 / 33d541fe).** Level 2 in
the browser: play against `Field::Level(1)`, `n_inner = [n₀, n₁]`, the same
outer worlds and seed formula as `walt-wasm` so the two levels share
common random numbers; bid and declare byte-identical to walt-wasm's,
pinned by a native equality test. Cost grows roughly with n · n₁. Trick-1
decisions observed, single-threaded wasm under Node on an M-series
(single-shot instrument readings, not statistics; `walt2-wasm/pkg/README.md`):

| n | n₁ | n₀ | trick-1 decisions observed |
|---|---|---|---|
| 4 | 2 | 2 | 0.5 s / 3.6 s / 1.4 s / 0 s |
| 8 | 2 | 2 | 3.8 s / 1.6 s / 0 s / 0 s |
| **8** | **4** | **2** | 6.3 s / 10.1 s / 2.1 s / 0 s (the defaults) |
| 16 | 4 | 2 | 13.0 s / 1.2 s / 0 s / 2.9 s |
| 16 | 8 | 2 | 26.0 s / 11.9 s / 7.2 s / 0 s |

Its own README refuses the inference the grid invites: "Whether
small-knob level 2 beats big-knob level 1 is an empirical question … do
not assume depth beats samples at equal latency." Never a default.

**The 2026-09-06 verdict at the table.** The partnership program
(owned by [walt-partnership-program](walt-partnership-program.md)) ran
the cost-matched comparison on 100 fresh mirrored deals at bid 30,
fixed search throughout, 40 / 8 / 2, 14 s wrapper
(`experiments/partnership/campaigns/default-partner-battery/RESULTS.md`):
**L2 Partner vs L1: 14 wins / 14 losses / 72 ties** (contract-win
fraction 50.0%, rough ±2 SE 44.7–55.3%); L2 Partner with counted voids vs
without: 12 / 17 / 71 (47.5%, 42.1–52.9%). Per-move wall under the shared
ten-game load: L1 0.228 s (0/1821 fallbacks), L2 Partner 1.127 s
(43/3690), L2 Partner with voids 1.318 s (60/1809) — "about five times"
L1's cost with no observed net contract advantage; the practical ruling
in `default-partner-battery/STRENGTH-ASSESSMENT.md` is that L1 default
"is a defensible operating default now without claiming a proved
strategic ordering" (that document's own reading, not a strength result
— a "proved" ordering is exactly what nothing here has). Refined partner minds
are unplayable under the wrapper: every race/refine partner configuration
crossed the > 5% fallback gate within one or two pairs (`partner-race`
10/33 fallbacks at 3.582 s per move; `partner-race-voids` 11/33 at 3.786 s;
`partner-race-fixedmind` 2/20 at 2.200 s; `foundation-battery/RESULTS.md`),
a cost stop, explicitly not a strength verdict — under a clock cap,
strengthening a modeled procedure can weaken the executed player by
raising its fallback rate. The composed partnership exam of 2026-09-07
([walt-gym](walt-gym.md)) is the one instrument on which L2 Partner
scores above L1 — 26/30 optimal (mean exact regret 959/205200) against
24/30 (1643/205200) — a selected, field-relative diagnostic, not a
strength result.

**The G1/G2 lesson (2026-09-04/05).** On the real Gran hand
([walt-gran-anchors](walt-gran-anchors.md); level-2 runs on the unmerged
branch `walt-g1-l2`, `walt/briefs/MORNING-2026-09-05.md`), level 2 *holds*
the 6-4 at both nodes where it is legal — at trick 3 exactly over all
17,640 void-consistent deals, 5-2 at 654‰ against 6-4 at 640‰ — declining
a guaranteed +11; and the made hand G2 is exactly locked from trick 3
(all 280 deals at trick 4 make whatever Gran plays, and level 1 gives the
identical tie set). At an exact tie at P(make) = 1 the objective has no
gradient; a modeled partner's reading can reach the decision only through
P(make), which cannot exceed 1; refinement is the wrong instrument (no
worlds remain to add); so the play falls out of tile-index order and the
count tile is hoarded deterministically (§1). The design conclusion
recorded on 2026-09-05: **another rung of modelling is not the remedy;
the levers are the objective (Boolean, pins at 1) and an explicit
tie-break at exact indifference. Neither is a feature; Jason's call is
pending.**

## 5. Bidding and declaring

Plainly: the same solver that plays the hand can price a contract before
it starts — "if I named this trump and bid this much, how often would we
make it?" — and walt bids by walking that price up until it drops below a
threshold. The threshold was calibrated once, on a frozen corpus, and
has not moved since 2026-08-19.

**The substrate.** Def 6.2's bid level b parameterizes the same solver:
`bidcurve.rs` computes P(make b) for all nine declarations and every b in
30..=42 at the auction point, over common random worlds per hand, with a
level-1 seat (`n_inner = [8]`). The curves are not monotone in b — the
analysis counts monotonicity violations per pass (1,357 / 1,822 / 1,536 of
23,400 cells at n = 12 / 40 / 200; `ANALYSIS-2026-08-19.txt`) — which is
what one expects when the bid sets the decided cutoffs every modeled mind
plays to (Def 6.2), and is why the rule below prices each bid level
rather than assuming a shape.

**The θ walk** (`walt-wasm/src/api.rs` `handle_bid`; `webtable.rs`):
price all nine declarations at the bid needed over common open worlds;
if the best is below θ, pass; otherwise take that declaration and raise
the bid while P(make b+1) ≥ θ. θ is a request parameter with default
11/16.

**The single-look calibration (2026-08-18 15:10 → 2026-08-19 01:34 CDT;
`walt/probes/bidcurve/`).** Three nested-common-random-number passes,
n = 12 / 40 / 200, over the same 200 frozen hands (23,400 cells each, zero
DIED); one predeclared analysis (obligation O14 — no peeking, no
resampling to a verdict). Auction simulation on the n = 40 curves,
scored by the n = 200 reference at the chosen cell:

| θ | bids | overbids (ref < 5000 bp) | missed bids | mean final bid |
|---|---|---|---|---|
| 1/2 | 200 | 37 | 0 | ~41 |
| 9/16 | 200 | 17 | 0 | ~41 |
| 5/8 | 200 | 3 | 0 | ~40 |
| **11/16** | **200** | **0** | **0** (mean reference 7270 bp) | ~40 |
| 3/4 | 199 | 0 | 1 | ~39 |

θ = 1/2 is the saturation overbid quantified; **11/16 is the first rung
with zero overbids and zero missed bids** and became the default in
`walt-wasm` `bid`, `walt2-wasm` `bid` and `webtable` (`THETA_NUM/THETA_DEN`)
at 9a056f20. n = 12 is unfixable by θ (11/200 overbids at 11/16 and at
3/4). Declaration is noisier than bidding: first-max declaration
agreement with n = 200 is 122/200 at n = 12 and 159/200 at n = 40. The
solo-auction protocol caveat travels with every number: the corpus
simulates one seat bidding against the reference curve, not a four-seat
auction.

**Declaring.** `walt_bridge`'s `declare` is argmax P(make 30) over pip
trumps 0..6 (doubles and no-trump only with `WALT_DECLARE_FULL=1`) from
`n_declare` = 100 open-belief worlds with 4×/16× tie refinement;
`walt-wasm`'s `declare` prices all nine declarations at the contract bid;
`webtable` names trump by best-lead pricing with refinement. **In the
3×384 arena pool neither team used walt's declare** — both declared by
the arena's best-pip-trump heuristic (§3; a `WaltDeclareBidder` patch was
drafted for the arena and never applied). In plunge, the auction is the
app's heuristic `mediumBid` ladder and only the declaration goes through
walt at 40/8 (`experiments/partnership/BASELINE.md`), so the phone runs
the 11/16 auction default without exercising it. The partnership
campaigns fix bid 30 and match contracts, so nothing since 2026-08-19 has
measured auction strength (obligation O9, bid-level generalization, is
still "trivial by conservation; assert in CI when built").

## 6. The variant seats that are not the default

The ruling that governs all of them: **the live default player is
untouched until arena and conformance gates justify a change, on Jason's
word** — CE-A7/§20.16 (2026-08-24), restated at CBS-A9, APS-A9, MB-A7 and
FH-A10 ("The live default player is untouched by this lineage until arena
and conformance gates justify a change on Jason's word", 2026-09-04;
`walt/CENSUS-RULINGS.md`). "Untouched" is a statement about the default,
not the code — §8 lists what changed inside the shared library under
parity gates. Two of the three variants below speak the same line
protocol as `walt_bridge`, so plunge or the arena could seat them with
zero external changes; none has an arena result.

### The controller player (`solver::act`, PR #37 / 23ba1c22, 2026-08-24; CE thread)

Spec: [`walt/CONTROLLER-PLAYER.md`](../walt/CONTROLLER-PLAYER.md); parent
mathematics `walt/math/calculated_evidence_v0.1.md` §16.4, rulings
CE-A1..A8; era readout [walt-calculated-evidence](walt-calculated-evidence.md).
Per decision it builds one frozen level-1 continuation policy per legal
tile (`ActionRule::PinnedThenLevel1` at the declared schedule 8 / 2), runs
the §16.4 evidence controller under a run-scoped strict risk plan
(`δ_d = δ_run/(d(d+1))`, δ_run = 1/100 per hand, d = plies + 1), and routes:

| controller result | route | tile chosen by | correctness boundary |
|---|---|---|---|
| one legal tile | `forced` | the rules | inside |
| `ExactFrozenSet`, unique max | `exact-winner` | the exact winner | inside |
| `DeltaSettled` | `delta-settled` | the δ-settled winner | inside |
| `ExactFrozenSet`, `winner: null` | `exact-tie-level1` | live `level1_evaluate` (200 / 8) among the tied maxima | **outside** |
| `Unresolved` at the cap | `unresolved-level1` | live `level1_evaluate` among the δ-survivors | **outside** |
| `EpsilonEquivalent` (ε-mode only) | `epsilon-level1` | live rank among survivors | **outside** |

δ-safe eliminations are inside the boundary — a candidate is removed only
by a settled directed edge at the declared risk; the level-1 ranking
among survivors is a scheduling choice outside it and is never presented
as a settled winner (`ActRoute::settled()` is false on every fallback;
every surface logs the route). Fibers ≤ `exact_cap` (2000) run the exact
frozen-set endpoint directly. `world_cap` is a **think-time budget**:
interactive 128 (`ActConfig::interactive`), batch 512 (`ActConfig::full`,
Jason's 2026-08-24 ruling); a low cap buys more honest fallbacks, never a
wrong settlement — trick-1/2 decisions at cap 512 cost minutes, at 128
about ten seconds. Its read-only precursor, the shadow instrument of
step 7 (`walt/probes/shadow/README.md`: 183 decisions beside the live
200/8 player at cap 128 — ExactFrozenSet 67 / Unresolved 116 /
DeltaSettled 0; live choice = controller winner in 23/27 settled
decisions and among the survivors in 116/116 open ones; 40 honest exact
ties; live evaluation median ≈ 0.21 s vs shadow ≈ 10.3 s), is what this
delivery made act behind a stable API. Surfaces: `controller_bridge`
(the `walt_bridge` line protocol; `WALT_CTRL_LOG` JSONL with route,
settled flag, consumed worlds and fallback options), and `ctrl [cap=N]`
seats on `webtable` and `playtable` (play only; auction and trump stay
level-1). Gates: `walt/walt/tests/solver_act.rs` (5: forced routing
without the controller; small roots preroute exact with route matching
result; the capped sampled route falls back among survivors; determinism
per information state; the 28-ply ordinal allocation stays under δ_run)
plus unit tests in `act.rs` (the route alphabet is six labels with
exactly three settled; no wildcard arm). **No arena run, no conformance
gate, no strength number of any kind exists for it.**

### The waking seat (`solver::waking`, PR #54 / 93d99563, 2026-08-25; CE and L2 threads)

The first walt with a thinking-teammate model. Per non-forced decision:
act's σ0 choice is always computed; a hard-budgeted **wake check** asks
whether a σ1 (level-1) reading of the position positively selects a
rival tile (exact paired detection at fiber ≤ 1024, else 24 paired worlds
on the sampled route); only on that positive evidence does it escalate
through `solver::targeted::targeted_root` (exact fiber cap 4096, baseline
prefix 128, E3 prefix 24, ε = 1/20); unsettled means play σ0, recorded.
Declared epoch pair (`WakingConfig::live`, identical in `waking_bridge`
and `granrun` so censuses compose): σ0 = `Level0{n₀ = 2}`, σ1 =
`Level1{n_outer = 4, n₀ = 2}`, candidates [8, 2], act at
`ActConfig::interactive`, waking risk δ = 1/20 per hand under
`wake:`-prefixed scopes asserted disjoint from act's. The exact wake cap
was retuned 64 → 1024 after the first smoke hand: under the telescoping
risk the sampled probe would need a net pivotal margin of roughly 10–17
worlds of 24 to settle, so its honest outcome is almost always
`no-wake-budget-exhausted`, and the wake gate's real coverage is the
exact route.

*Natural-play profile* (`walt/probes/waking/README.md`, `summary.txt`;
2 driven hands, 56 decisions, live epoch; the scaled census deliberately
skipped): **σ0 baseline 729‰ of decision compute** (206,971,014 of
283,899,641 µs), wake check 269‰, escalation 1‰; tricks 1–2 alone 926‰;
wake rate 1/34 checked decisions (the one wake, trick 5, exact route,
moved the play; agreement with σ0 55/56); the exact route settled 13/13
checks it reached (10 ties, 2 baseline-confirms, 1 rival) while all 21
sampled checks above fiber 1024 stayed open; per-decision total p50
14,337 µs, p90 21,665,288 µs, max 68,829,911 µs. **Affordability: NO, not
as-is — minutes per natural hand.** The profile's purpose was
attribution, and it targeted the 2026-08-25 speed campaign (#53/#55/#56),
whose conclusion was that the modeled minds are the bill.

*The first real hand* (`walt/probes/gran/README.md`, 2026-09-04,
commits 32aa14f1 / 8174fa83; owned by [walt-gran-anchors](walt-gran-anchors.md)).
Replaying the validated G1 record with the waking seat at S2 (Gran's
chair) and the other three seats on the record: **agreement with the
record 6/7**; at trick 1 it plays the 6-4 where the live seat played 6-2
— but the wake did *not* fire there (fiber 46,558,512 sent the check down
the sampled route, which spent its 24 worlds and returned open; σ0 via
route `unresolved-level1` already picked 6-4, so the flip is an
epoch/baseline difference against a 40-world phone panel that does not
compose with it); the one real wake landed at trick 5 (fiber 300, exact
route, `exact-sigma1-selects-rival`) and moved the play from σ0's 1-0 to
4-2, the human play. Replay wall 25.04 s (trick 1 alone 12.55 s;
baseline 440‰ / wake check 536‰ / escalation 23‰). Driven whole hand
with the waking seat at all four chairs: T0 26 – T1 16, still set
against 30; 28 decisions, zero wakes, agreement with σ0 28/28, 166.11 s
(trick 1 = 619‰). The record says loudly: this does not demonstrate that
partner modelling fixes the 6-4 problem. Gates:
`walt/walt/tests/solver_waking.rs` (9: forced play emits a record and runs
no detection; within-budget-unsettled plays exactly σ0; exact settled
agreement is no wake; a settled wake escalates and plays the selection; a
refused escalation falls back with the refusal recorded; stage-four
routing; open δ-survivors route to a recorded fallback never a pick;
census JSONL round-trip; waking scopes disjoint from act's) plus
compile-fail doc locks (no forged `WakeEvidence`; no numeric accessor on
`RecordedFallback`). Never a default; MORNING item 6 records "wake
retired" with no commit or record saying what exactly was retired.

### The unified proof-state player (`solver/unified.rs`, UP0 PR #86 / 3b4105ca, 2026-09-02; UP1a 62abe028, 2026-09-03)

A separate player track, owned by
[walt-focal-horizon-era](walt-focal-horizon-era.md): one decision function
`UnifiedPlayer::decide(state, budget)` over every instrument of the
counted-belief program — a total five-tier cascade (decided arithmetic →
endgame exact → mixture → certified regret Γ = U* − B_exec → σ0
fallback), every answer carrying a `Provenance` naming the instrument
and every typed refusal it fell through (`walt/briefs/UP0-REPORT.md`; 18
gates in `tests/solver_unified.rs`; transcript
`walt/probes/factor_belief/unified_run1.txt`, 216 decisions). It has no
bridge, no table, no arena run and compares itself to nothing;
`walt/MAP.md` keeps it distinct from "the live default player" (row 10,
"untouched by everything above"). It is listed here because it can act
at every legal state of a hand; it is not a seat anyone has played.

## 7. Surfaces and reproducibility

**The rob_bridge line protocol** (`walt_bridge.rs` header; shared by
`controller_bridge` and `waking_bridge`, so the mk5 arena's `rob:<path>`
adapter and plunge consume any of them unchanged). Request:
`seat decl bidder h0 h1 h2 h3 h4 h5 h6 n (actor domino)*n` — the viewer
seat 0..4, the declaration id (0..=6 pip trump, 7 doubles, 9 no-trump),
the auction winner, the viewer's seven originally dealt tile ids in
canonical triangular order (`(0,0)=0, (1,0)=1, …, (6,6)=27`), then n
chronological (actor, tile) pairs including the viewer's own. Reply:
`domino leader points0 points1` — the chosen tile and walt's
independently derived trick leader and team points (team = seat % 2), so
the harness can assert rules conformance on every decision. Extra kind
`declare bidder h0..h6` → one declaration id. The bidding team must be
internal T1 (seats 1, 3): when the arena's bidder sits on an even seat
every label is rotated by +1 internally and rotated back in replies
(audited in the arena record). Refusals (`Level1Refusal::{Deadline,
InfeasibleFrame}`) play the lowest legal tile with a stderr note. Measured
2026-09-13 on this machine, release binary under
`/Users/jason/code/texas-42/walt/target/release/` (mtime 2026-09-07 11:54,
after HEAD c00717d1; its provenance against c00717d1 is not verified):
`printf 'declare 0 1 5 11 12 17 23 25\n2 6 0 1 5 11 12 17 23 25 2 0 27 1 21\n' | walt_bridge 8 2 20 8`
answers `2` (deuces for Gran's dealt hand at n_declare = 8) and
`25 0 0 0` (the 6-4 at the G1 trick-1 root, leader 0, points 0/0) in
about a second — a protocol and lawfulness check at a toy sample, never
a value.

**The wasm string API** (`walt-wasm/pkg/README.md`, `src/api.rs`; ~250 KB,
zero imports, serial solver). One line per request: `walt1 play|bid|declare`
with fields `decl`, `bid`, `seat`, `bidder`, `hand`, `plays`, `n`, `n0`,
`seed`, `budget_ms`, `race`, `theta num den`, and optional `viewer` /
`viewer_hand` (the "How'd I do? Ask walt" review: the acting seat's
options priced from the *viewer's* fiber, returned as `viewer_opts =
[tile, bp|null, support]`; `bp` is null where the viewer's fiber cannot
lawfully place the tile). Every `play` response carries `choice`,
`forced`, `raced`, `opts` in basis points (omitted in race mode), and
walt's `leader` and `points` — **the integration contract is to assert
those against the client's engine on every decision** ("this is exactly
how the mk5 arena runs the subprocess bridge"). `walt2` is the same API
with `n1` and `level: 2`. Plunge integrated the level-1 oracle on
2026-08-22 (plunge 1810da20) and Jason played it at length from
2026-08-23; the sync between plunge's copy and the repository's is a
standing card ([[plunge-walt-sync]], §9).

**The human tables.** `webtable` (localhost HTTP; `/state`, `/step`,
`/play?t=`, `/bid?b=|pass`, `/pick?d=`, `/hint`, `/auto`, `/review?p=`,
`/new?seat=`; dominoes drawn clockwise, trump deliberation visible; this
is the table at which Jason's 2026-08-17 verdict — "that's a good player
buddy. a good player." — was given) and `playtable` (terminal, one human
seat, contract frozen to receipt hand 8; `hint`/`auto` run the same
evaluation from the human's chair). Both grew `ctrl [cap=N]` seats on
2026-08-24. Since PR #37 all three of `playout`, `playtable`, `webtable`
domain-separate the deal stream from the per-decision belief streams
(constant ^ session ^ own hand ^ record hash — the `walt_bridge` pattern),
so session output is record-grade (the O27 audit finding).

**The partnership oracle** (`walt/walt/src/bin/partnership.rs` +
`experiments/partnership/player.py`, 2026-09-06). Native worker: a text
request (`mode status|baseline|partner|all-l1`, `decl`, `bid` 30..42,
`seat`, `bidder`, `hand`, `plays`, `seed`, `n ≤ 640`, `n0 ≤ 64`, `n1 ≤ 64`,
`budget_ms ≤ 14000`, `inner_belief 0|1`, `selection 0|1|2`,
`modeled_selection 0|1|2`), validated against the library replay and
frame feasibility, evaluated by `solver::partnership::evaluate` with
`Field::SeatLevels`, answered with the choice, every option's exact
rational and work statistics; `--stream` keeps one process per match.
`player.py` wraps it as JSON lines, cross-checks legality, leader and
points against the **independent Python referee** (`rules.py`), prepares a
`legal-fallback` reserve and then a complete 8/2 level-1 `l1-fallback`
(≤ 1.5 s) before spending the remaining allowance on the requested mode;
`--mode phone` drives the archived plunge WASM through `phone.mjs` at the
phone's 40/8 race-on settings. A fallback is an execution route, never a
search family; modeled minds never fall back — an incomplete nested solve
aborts the host attempt (`PLAYERS.md`). Measured 2026-09-13 on this
machine (same binary as above): `status` on the G1 root returns
`{"legal":[23, 25],"leader":0,"points":[0, 0],"trick":1}`.

**The mirrored-pair arena and its pool discipline**
(`experiments/partnership/{match,pool,verify_campaign,campaign_report}.py`,
9236ca7f). `match.py init` creates a two-player mirrored match: each deal
played twice with the policy teams swapped, bid 30, pair score
make(A) − make(B), both-make and both-set pairs tie regardless of points;
panels `random` or `worlds` (a fixed opening hand × hidden completions).
`pool.py` runs up to ten games concurrently under the packet's process-group
watchdog (`run_capped.py --seconds 295`), with per-move atomic checkpoints,
retries, a pool lock, and results committed only as a contiguous seed
prefix so fast games cannot select the evidence; `verify_campaign.py`
replays every game against the referee. The recipe is in
`experiments/partnership/README.md`; the Python tests that spawn the
native binary need a built `walt/target/release/partnership` (in a
worktree, wire `CARGO_TARGET_DIR`). Details and every campaign:
[walt-partnership-program](walt-partnership-program.md).

**The smoke and gate inventory that pins determinism.** None of these
pins strength.

| gate | pins | count / fixture |
|---|---|---|
| `walt-wasm/smoke.mjs` (Node ≥ 23.6) | the committed `walt.wasm` reproduces the frozen native full-hand trace (contract S0 bid 42 decl 9) play for play | 28/28 — measured 2026-09-13 on this machine: OK, 874 ms (934 ms on 2026-09-12; wall times are the only thing that varies) |
| `walt2-wasm/smoke.mjs` | same for `walt2.wasm` on its own trace | 28/28 — measured 2026-09-13: OK, 7436 ms, slowest decision 3842 ms (trick 1 pos 1) at n = 4 / n₁ = 2 / n₀ = 2 (6931 ms / 3593 ms on 2026-09-12) |
| `walt-wasm/tests/full_hand.rs` | full-hand lawfulness and conformance (`full_hand_all_walt`), the viewer review (`play_viewer_fiber_review`), the race mode (`full_hand_all_walt_raced`) | 3 |
| `walt2-wasm/tests/full_hand.rs` | `full_hand_all_walt2`, `auction_matches_walt1` (bid/declare byte-identical to walt-wasm), `protocol_rejections` | 3 |
| `walt/walt/tests/solver_selection.rs` | the three selection schedules: fixed ties use tile order without extra sampling; refinement reconsiders formerly lower candidates; unresolved ties exhaust exactly the historical fresh bundles; racing uses paired blocks and both objectives; a saturation race refines only tied survivors; a refusal discards previous rounds; a forced race never requests an evaluation | 7 |
| `walt/walt/tests/solver_sigma1_repair.rs` | R1–R6 of the σ1 repair: the pinned infeasible specimen refuses instead of spinning; feasible-frame draws bit-identical to the before-side fixture `tests/data/sigma1_before_v1.txt` (48 frames, 91 KB, captured before the patch); dedup (a local `fn sample_belief` is a compile error); the feasibility oracle | 8 `#[test]`, one `#[ignore]`d fixture regenerator |
| `walt/walt/tests/solver_partnership.rs` | counted inner samples belong to independently replayed receipt fibers; voids survive trick resolution with declaration-relative following; the inner cache separates void profiles and ignores host worlds; counted belief runs all three seat profiles and refuses expired sampling; modeled L1 inherits selection while L0 keeps its fixed Dice boundary; seat profiles upgrade exactly the declared seats; all-zero = uniform level-0 and all-one = uniform level-1 fields; modeled level 1 independent of outer completions and cache order; the baseline evaluator uses the exact common world stream; the bounded evaluator is deterministic and reports every legal action; a zero deadline aborts before sampling | 12 |
| `walt/walt/tests/solver_act.rs` + unit tests in `act.rs` | route alphabet and fixtures (§6) | 5 + 6 |
| `walt/walt/tests/solver_waking.rs` | the waking seat (§6) | 9 + compile-fail locks |
| `walt/walt/tests/solver_viewer_fiber.rs`, `solver_policy.rs`, `solver_ordering.rs`, `solver_panel_conformance.rs` | cross-fiber pricing; frozen-policy identity and the pinned 78/34 split replay; reorder-not-cull value equivalence; panel conformance | 2 / 7 / 4 / 8 |

The whole gate is `walt/ci/check.sh` (fmt, clippy `-D warnings
-D float_arithmetic`, the no-float grep, vocabulary greps, release tests,
byte-diffed receipts, Lean); last recorded green runs at 230 s (CI1,
2026-09-04) and 308 s (FH3, 2026-09-04). It was **waived** for the
2026-09-06/07 partnership and Scheme sessions (§9).

## 8. Since 2026-08-25: changes to the live code, with no default change

Each entry is parity-gated; none changed what plunge or the arena runs.

- **2026-08-25 — walt2-wasm (PR #58 / 33d541fe).** Level 2 in the browser
  (§4). Additive; never a default.
- **2026-08-25 — the waking seat (PR #54 / 93d99563).** `solver::waking`,
  `waking_bridge`, the 2-hand profile (§6).
- **2026-09-02 — the σ1 repair (PR #83 / 161b0195; authorized 2026-09-01
  under "breaking the live player temporarily is entirely approved";
  `walt/briefs/BRIEF-SIGMA1-REPAIR.md`).** The shipped `sample_belief` was an
  unbounded shuffle-and-reject loop that spins forever at
  zero-joint-mass information states (hands record-consistent for their
  own seat but jointly uncompletable — the MB0 slice was the first caller
  to reach one; pinned specimen seat S3 hand {4-2, 4-4} after [4-1, 4-3,
  1-1]). It existed in **five byte-identical copies** (`solver/mod.rs`,
  `walt_bridge.rs`, `playout.rs`, `playtable.rs`, `divergence.rs`). Now:
  one library authority with an exact Hall-condition feasibility precheck
  (`belief_frame_feasibility`, 8 subsets, no randomness consumed) returning
  a typed `InfeasibleFrame`; the draw sequence on feasible frames
  bit-identical (fixture R2); `Level1Refusal::{Deadline, InfeasibleFrame}`
  threaded through `level1_evaluate`; the live bridges play lowest-legal on
  refusal and log it. Verified on this tree 2026-09-13: `fn sample_belief`
  is defined only in `solver/mod.rs` (`solver/partnership.rs` carries
  `sample_belief_bounded`, a deadline-checked wrapper that preserves the
  stream); `fn level1_evaluate` is defined in `solver/mod.rs`,
  `walt_bridge.rs` and `playtable.rs` — **the triplication the slice
  deliberately did not pay** (§9).
- **2026-09-04 — Gran G1 replay (32aa14f1, 8174fa83).** `granrun` added as
  a variant surface touching nothing (§6; [walt-gran-anchors](walt-gran-anchors.md)).
- **2026-09-06 — the partnership player (cfb0fb25).** `solver::partnership`
  (`FieldProfile`, `Field::SeatLevels`, `evaluate`), the `partnership` bin,
  `player.py` and the referee (§7).
- **2026-09-06 — selection unification (9236ca7f, 170 files, +4279/−873;
  record `experiments/partnership/FOUNDATION.md`, commit body empty).** New
  `solver/selection.rs` (§1). `level1_evaluate`'s inline 4×/16× refine loop
  became `selection::select(Rule::Refine)`; `level1_raced` and
  `level1_race_refined` were rewritten over `selection::race` /
  `race_refine` (block 8, k_min 6, δ = 1/128; the older frozen-continuation
  `level1_race` keeps `RACE_BLOCK` 16 / `RACE_KMIN` 8); `Shared` gained
  `modeled_selection` (asserted before any cache use) and
  `inner_worlds_by_level`; `Solver::pi` routes modeled levels k ≥ 1 through
  the same selector at their own frozen budgets while k = 0 stays `Fixed`
  against Dice; new `Solver::action_values`. Parity: nine pre-change
  fixed-policy comparisons retain every value and move; 64 native /
  archived-phone decision comparisons agree; 44 focused Rust tests. The
  local copies in `walt_bridge.rs` and `playtable.rs` were not rerouted —
  behaviorally the same loop by construction, but now a divergence
  surface.
- **2026-09-06 — `InnerBelief` (dbcc698f; record
  `experiments/partnership/INNER-BELIEF.md`; `SCENARIO-PLAYER.md` note).**
  `InnerBelief::{Voidless, VoidsCounted}` (§1); `Key` and `PiKey` gained
  `voids: Option<[u32;4]>`, every surface's `Key` literal got `voids: None`
  (walt-wasm, walt2-wasm, webtable, controller_bridge, waking_bridge,
  bidcurve, tiltaudit, ordering_bench); `inner_belief::after_play` is the
  single void-update authority; `Solver::check_belief_key` asserts the
  invariant. Nine pre-change decisions reproduced exactly
  (`runs/voids-before/golden.json`). It implements an *alternative* for
  obligation O5 and does not discharge O5's cost/strength measurement; the
  outer sampler is unchanged; the two inner samplers use different
  deterministic streams, so no single-move difference can be attributed
  to void logic alone. Foundation battery: `l1-race-voids` vs `l1-race`
  9 / 5 / 36 on 50 random deals and 5 / 8 / 37 on five focal hands × ten
  completions; default battery 12 / 17 / 71 (§4). No strength gain
  established; default stays `Voidless`.

## 9. Debts and honest gaps

| debt | state at c00717d1 | where filed |
|---|---|---|
| **`level1_evaluate` triplicated** | library `solver/mod.rs:1340`, `walt_bridge.rs:514`, `playtable.rs:604`, each headed "STILL TRIPLICATED"; the 2026-09-06 selection reroute touched only the library copy | `BRIEF-SIGMA1-REPAIR.md`; source headers |
| **`playout.rs`'s PiKey omits banked totals** (Def 3.4) | deliberately filed, not fixed; the `all1` mode's information-inconsistency finding also stays filed | `playout.rs` header; CE-era audit |
| **O5 measured only on branch `walt-o5`** (9 commits, tip 2981e090, 2026-09-05, not in main) | after a seed repair, live epoch (50 / 8, 72 deals) 33 / 19 / 20 pairs void-aware ahead, +262 of 6048 points; reduced epoch (16 / 4, 192 deals) 76 / 83 / 33 blind ahead, −226 of 16128; "dead heat" withdrawn; flag off; the `SCENARIO-PLAYER.md` O5 row's route is still "ablation probe", with a note added 2026-09-13 recording both implementations and that neither discharges the obligation | `walt/briefs/MORNING-2026-09-05.md` item 5; [walt-gran-anchors](walt-gran-anchors.md) |
| **Two void-aware implementations** | `walt-o5`'s `Level0Field::void_aware` + shuffle-and-reject with `Key::voids`, and main's `InnerBelief::VoidsCounted` via `FiberDp`, both add `voids: Option<[u32;4]>` to `Key`/`PiKey` with different semantics — a merge-conflict surface; Jason's 2026-09-05 ruling asked for rank/unrank over a counting DP, which main's kernel path may already satisfy (no record says so) | `walt-o5:walt/kanban/backlog/inner-voids.md` |
| **`walt-g1-l2` unmerged** (8 commits, tip 6abdd78f) | `level2.rs` `fixture` mode and the L2 Gran records exist only there | MORNING item 6 |
| **The champion binary was never re-matched** | the 3×384 pool ran the pre-PiKey-fix bridge; every player since has been measured only against walt or the phone | §3 |
| **CI waived 2026-09-06/07** | `walt/ci/check.sh` deliberately not run for the partnership and Scheme sessions (focused suites only: 44 then 36 Rust tests, 14 Python tests, clippy, wasm32 compile); no record says the central gate has been green at or after 9236ca7f / dbcc698f | `experiments/partnership/SCOPE.md`, `FOUNDATION.md` |
| **[[plunge-walt-sync]] drift** | the committed `walt-wasm/pkg/walt.wasm` (SHA-256 `d7f61f22…`, rebuilt 2026-08-24 with the review column) differs from the phone's bytes (`af0200af…` = 9a056f20); the version on any actual phone was never independently verified (only the local plunge checkout 122ea7a5); race wiring is plunge's decision | kanban card, opened 2026-08-24 |
| **G2/G3 deal unrecoverable** | six-trick prefix validated; S2's hand fully known; the residual {4-1, 4-4, 5-3} is 6-way ambiguous and mechanically undecidable; no driven run or `replay_hand` validation possible unless plunge recovers the seed | [[gran-anchor-reconstruction]] (open on two items) |
| **`v5_literal_count_timing_position_reconstructs` still `#[ignore]`d** | `tests/solver_calibrate.rs:420`; its stated blocker (G1 reconstruction) was discharged 2026-09-04; whether G1 is the literal position it names is unverified | [walt-calculated-evidence](walt-calculated-evidence.md) |
| **Tie-break at exact indifference unruled** | the lever named 2026-09-05 (objective + tie-break); Jason's calls (A)–(F) from the MORNING brief have no recorded decision after 2026-09-05 | MORNING items 4 and "Your calls" |
| **"Level-1 walt" is not one thing** | §2; the book always names the configuration | — |
| **Spec staleness** | `SCENARIO-PLAYER.md` was written 2026-08-18 and patched by dated notes rather than rewritten: §9's paths were corrected 2026-09-13 (the binaries moved from `walt/walt-m3-probe/` to `walt/walt/src/bin/` at d1499d43, 2026-08-24), Def 6.3 gained its post-cap note 2026-09-13, the O5 row its two-implementation note; the definitions themselves still describe the 2026-08-18 build | — |

**The obligations ledger (`SCENARIO-PLAYER.md` §10) is the graduation
path.** Nothing on this page is promoted by its own existence; each
obligation names its kind and route. O10–O11 are permanently retired
(SP-A11; retired numbers are never reused).

| block | filed | obligations (kind) |
|---|---|---|
| spec-after-build | 2026-08-18 | O1 no-strategy-fusion (invariant audit, eventually Lean-shaped); O2 key sufficiency (paper proof); O3 sampler correctness (proof); O4 posterior semantics — "the load-bearing one" (proof); O5 cost of the no-void inner simplification (measurement — see the O5 row above); O6 sampling error on root values (design + math: per-world make indicators admit binomial-style intervals); O7 execution-order invariance (proof); O8 tie-refinement bias (analysis); O9 bid-level generalization (proof + tests) |
| signed-pivotal geometry, SP-A1..A12 | 2026-08-18 | O12 frozen-plan typing; O13 discovery/evaluation separation; O14 sequential validity (anytime-valid or predeclared — applied to the bidcurve single look and E0 racing); O15 scenario/world domain match; O16 envelope containment; O17 conditional-generator correctness and cost; O18 optimization-lock accounting (exact frozen-pair results never labeled exact root-action results); O19 behavioral census |
| calculated evidence, CE-A4 | 2026-08-24 | O20 exact evidence theorem; O21 risk-ledger completeness; O22 frozen-policy identity; O23 canonical fiber and sampler domain; O24 exact-escalation correctness; O25 result typing and fallback separation; O26 execution-order invariance of evidence; O27 sampling randomness semantics (the audit finding filed under this number — one RNG shared by deal and belief sampling — was repaired in PR #37; the design note remains); O28 recursive inner-risk accounting |
| targeted level-2 field stability, L2-A2 | 2026-08-24 | O29 field identity and purity; O30 first-disagreement localization; O31 exposure-bound typing; O32 root field-stability screening; O33 field-correction evidence; O34 split-reach optimization correctness; O35 mechanism-trace fidelity; O36 level-model typing (a level-2 result is a best response to σ₁, never equilibrium); O37 cycle-detection discipline; O38 targeted completeness |

Per project law the path is paper proofs → wiki with tier labels →
independent re-verification (an exchange batch on Jason's authorization)
→ Lean for what earns mechanization. None of O1–O38 has walked it.

## 10. The partnership program lives elsewhere

The 2026-09-06/07 program under `experiments/partnership/` — the launch
packet and its phone reference, the player families, the calibration,
foundation and default batteries, the pool infrastructure, the Scheme gym,
policy synthesis and relational learning — is owned by
[walt-partnership-program](walt-partnership-program.md) (and the gym by
[walt-gym](walt-gym.md)). This page keeps only what those results say
about the seat: the configuration rows in §2, the level-2 verdict in §4,
the surfaces in §7 and the code changes in §8.

## 11. The record of the track, 2026-08-17 → 2026-08-24

Kept as provenance; every number is verified against its file and the
tier fence applies unchanged.

**2026-08-17/18: the seat, in two days.** After the 2026-08-10 reset froze
the compression search ([walt-program](walt-program.md)), the sampling-stack
player went from first lawful hand to a playing seat (level 1 at a web
table, Jason playing at length), the arena match (§3), the level-2 ladder
and the PiKey correction (§4), and divergence mining (§4). The path to
trick 1 ran through three probes (`walt/probes/m3/`): the exact
posterior-carrying ladder solves t = 4 in 4.305 s over 1,200 worlds and
dies at t = 3 (59,976 worlds, killed at ~600 s and 300M nodes), t = 2
(7,399,392 worlds) and t = 1 (399,072,960 raw assignments over the 30M cap;
`ladder_results_2026-08-17.txt`); the sampled ladder at n = 1200 of
59,976 picks 1-1 (96.48%) with 3-3 close (96.33%) in 513.7 s, and the
t = 1 wall is the ~10⁹-node public tree, independent of n
(`sampling_results_2026-08-17.txt`); the **scenario solver** — sampling the
deal *and* the field dice per scenario — collapsed that wall: t = 1 argmax
5-5 at every n from 4,000 to 256,000, failing 70 of 256,000 scenarios
against 187 for 3-3 in 64.0 s, with ~+1.1 pp value optimism against the
exact t = 4 truth (3-3 at 96.92%; `scenario_results_2026-08-17.txt`,
`results_2026-08-17.txt`). The level-1 probe then ran t = 4 over all 1,200
deals (3-3 optimal at n₀ = 32, 96.41%, and n₀ = 128, 96.66%) and t = 1 at
n = 2000 / n₀ = 16 (5-5 at 100.00%, zero losses, every other lead losing
≥ 4, 80.8 s; `level1_results_2026-08-17.txt`). `SCENARIO-PLAYER.md` was
written 2026-08-18, after the build. walt-wasm shipped the same day with
the player logic consolidated into the library (`level1_evaluate`,
`best_of`, `replay`).

**2026-08-19: race-then-refine, the θ calibration, the tilt audit.**
Signed-pivotal racing applied to the seat as `level1_raced` (block racing
on common random numbers with exact binomial elimination) and
`level1_race_refined` (survivor ties get the 16× refinement): in a native
bench the block race decided opening leads in 745 ms against the full
evaluator's 1230 ms while consuming 100 worlds against 40, disagreeing
only on saturation ties; the replay-race variant cost 5.9 s vs 0.46 s at
trick 2 (replay ≈ re-solve). Shipped as the walt-wasm opt-in `race 1`
with a full-hand conformance test; the default path stays byte-identical.
The arena gate (`tiltaudit arena`, 24 mirrored deals at bid 30, n = 40,
n₀ = 8; `walt/probes/tilt_arena_2026-08-19.log`): paired makes as bidder
race-only 1, full-only 2, both 11 — **a strength dead heat** — at decision
cost race 80,475 ms / 453 decisions (mean 177 ms) vs full 51,658 ms / 443
(mean 116 ms): the race *loses* time in the tie-saturated bid-30 regime,
so its edge is regime-dependent (openings, high bids) and the opt-in
posture stands; plunge turned it on. The θ calibration is §5. The tilt
audit smoke (`walt/TILT-AUDIT.md`, commits 81a1943 / f60b2e3, under the
signed-pivotal SP-A rulings) found the modeled level-0 field deterministic
in (seat, hand, record) — no tape, scenario = world — so its Phase E is
vacuous until a stochastic field exists; trick-6 roots are pure Case B
(q = 2000–3000 bp, τ = +1000‰ exactly, H ≤ 4); trick-4 strong gaps
(D = 25–32/400) recover the winner 4/4 at 25 worlds while near-ties
(D = 4–9/400) hover at chance until ~100 worlds; and hand 0 caught a live
discovery-selection error (discovery majority 6-5 where the 800-replay
panel prefers 6-2) — an instrument catch, not a verdict.

**2026-08-23: live in plunge.** Two review specimens from Jason's play —
a 100%-saturation revelation tie, and a 40-vs-160-world near-tie flip on
a count-timing choice (the trick-1 6-2 vs 6-4 of what became G1) —
motivated the level-2 field-swap probe, filed as spec only
(`walt/LEVEL2-PROBE.md`) and gated on the adaptive-sampling mathematics
([[adaptive-sampling-intake]]).

**2026-08-24: unification and the calculated-evidence era.** The seat's
crates (`walt-m3-carrier`, `walt-m3-probe`, the research stack) were
folded into the one `walt` crate at d1499d43 — pure code motion,
trace-identical, wasm smoke 28/28 byte-identical, the seat solver now at
`walt/walt/src/solver/`; freeze-56 re-issued append-only as v2. The same
day the adaptive-sampling mathematics landed twice and was adjudicated
(CE-A1..A8, L2-A1..A7) and its §22 build ran through step 8; that work is
owned by [walt-calculated-evidence](walt-calculated-evidence.md). What it
did to this track's episodes:

- *The 40/160 flip resolved as a near-tie, not a winner.* The shadow
  instrument reproduced the phenomenon class beside the live player (four
  exact-route disagreements at small fibers; 40 honest exact ties reported
  as ties), and step 8 (PR #31 / e5a5f521; records `walt/probes/step8/`)
  landed the V5 flip repair and per-pair E0 calibration: re-running the
  count-timing shape family on one epoch and one common stream at a cap
  ladder 40 / 160 / 640, the V5 law is asserted mechanically and no
  cap-dependent flip occurs — all six family members honestly `Unresolved`
  at every cap (q̂ ≈ 0.3, |τ̂| ≈ 0.01–0.25). The recorded episode was a
  near-tie forced through a phone-tier cap; the old player was never
  entitled to either answer. Three of the four shadow disagreements
  escalate to the exact winner (h4 d3 → 2-1, h7 d5 → 6-2, driven h14 d4 →
  2-1); h11 d4 (fiber 1750, τ = 11/175) is honestly Unresolved at all caps.
  The literal plunge position's test stays `#[ignore]`d (§9).
- *The saturation-tie episode* keeps Def 6.3's protocol ("look closer");
  the evidence path types it honestly (`ExactFrozenSet` with a null winner
  is a finding, not a defect).
- *The live player was audited, not changed*: `walt_bridge` CLEAN
  (information-consistent per-decision streams); `playout`'s PiKey and
  `all1` findings filed; the O27 one-RNG finding repaired in PR #37.
- *The Gran review specimens* were carded as anchors (three screenshots
  pinned with a manifest, PR #25 / 31b6a885; the no-seed path: the "How it
  went" grid is the complete deal). **Superseding the old bullet here:**
  G1 was transcribed, mechanically validated by `rules::replay::replay_hand`
  and played on 2026-09-04 (§6); G2/G3 committed as a validated six-trick
  partial with S2's information set complete; the card stays open on the
  G2/G3 deal and one pointer ([walt-gran-anchors](walt-gran-anchors.md)).
- *`walt/LEVEL2-PROBE.md`* became the detection layer inside the targeted
  level-2 controller (L2-A5): the fixed-policy smoke (never root-action
  screening, L2-A4); at PR #30 / ca0483d7 the exposure rungs E0–E2, the
  exact split-reach route E4 and the L2-T4 admissible screen; at PR #38 /
  151ea4f the Part VI cancellation ladder (PANEL-A7/A8) — on the h7-t5 root
  the smoke's "the fields never split" became an exact zero over all
  information-consistent continuations, one root produced the first
  pruning singleton in the wild, and slice 3 added the first
  `FieldDecisionChanged`, `FieldStableExactRoot` and `Dominated` — all
  frozen-candidate-set statements at a declared field pair, never
  play-strength claims and never "σ1 is a better mind" (O36). Records
  `walt/probes/fieldswap_screen/`, `walt/probes/fieldswap_cancel/`.

## 12. Where this sits

The seat is real and it wins the match it was built to win, and every
one of its numbers is a sampled estimate against a modeled field — not an
exact value, not an equilibrium, not a demonstrated strategic reason.
Since 2026-08-19 nothing has changed what the phone or the arena runs;
what has changed is the shared library under parity gates (§8), the
inventory of variants beside it (§6), and the understanding of where the
next unit of strength does *not* come from: not from another rung of
partner modelling at the table (14/14/72 at five times the cost, §4), not
from the waking seat's escalation (1‰ of its compute, §6), and not from
refinement at exact indifference (§1, §4). The open levers are the
objective and the tie-break (Jason's call), the void-aware inner belief
(measured, unresolved, off), and a post-fix re-match against the champion
that no one has run. [rob](rob.md) remains the exact-truth solver; walt
remains the seat; neither impersonates the other.
