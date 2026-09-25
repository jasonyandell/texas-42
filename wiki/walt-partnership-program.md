[Home](Home.md) · owns: the `experiments/partnership/` program — the launch packet and its contract, the preserved "phone" reference artifact, the native player families, every full-game battery and its numbers, the pool infrastructure, the policy-synthesis and relational-learning experiments, and what the program settled and left open · Sources: [`experiments/partnership/README.md`](../experiments/partnership/README.md), [`SCOPE.md`](../experiments/partnership/SCOPE.md), [`REPORT.md`](../experiments/partnership/REPORT.md), [`SESSION-STATUS.md`](../experiments/partnership/SESSION-STATUS.md), [`BASELINE.md`](../experiments/partnership/BASELINE.md), [`HEAD-TO-HEAD.md`](../experiments/partnership/HEAD-TO-HEAD.md), [`FOUNDATION.md`](../experiments/partnership/FOUNDATION.md), [`INNER-BELIEF.md`](../experiments/partnership/INNER-BELIEF.md), [`PLAYERS.md`](../experiments/partnership/PLAYERS.md) + [`players.json`](../experiments/partnership/players.json), [`POOL.md`](../experiments/partnership/POOL.md), [`CAMPAIGN.md`](../experiments/partnership/CAMPAIGN.md), [`POLICY-SYNTHESIS.md`](../experiments/partnership/POLICY-SYNTHESIS.md), [`RELATIONAL-LEARNING.md`](../experiments/partnership/RELATIONAL-LEARNING.md), [`fixtures.json`](../experiments/partnership/fixtures.json), [`SUMMARY.json`](../experiments/partnership/SUMMARY.json), [`reference/phone/PROVENANCE.md`](../experiments/partnership/reference/phone/PROVENANCE.md), the launch packet `packet/texas42-partnership-launch-v0.1/` and the relational packet `packet/texas42_relational_learning/`, and every `campaigns/*/RESULTS.md`, `MATCH.md`, `RESULTS.json`, `verification.json` named inline. Results files outrank prose; where the two disagree the page says so. Related: [walt-gym](walt-gym.md) (the exact gym), [walt-scheme-fix](walt-scheme-fix.md) (the language), [walt-gran-anchors](walt-gran-anchors.md) (G1/G2/G3), [walt-seat-play](walt-seat-play.md) (the level-1 seat, θ = 11/16, live play in plunge), [walt-instruments](walt-instruments.md), [walt-architecture](walt-architecture.md), [walt-negative-results](walt-negative-results.md).

# walt — the partnership program (2026-09-06 → 09-07)

> **Epistemic tier: EXPLORATORY — the whole page.** Everything under `experiments/partnership/` sits below every tier on [Home](Home.md#evidentiary-tiers--never-promoted-never-blurred): below corpus statuses, the Lean kernel, exchange-adjudicated CONFIRMED results, and rob's byte-diffed conformance receipts. The program's receipts are its own — independent Python rules replays, SHA-256-pinned artifact identities, resume proofs, and machine-readable `RESULTS.json`/`MATCH.json`/`measurements.json` files. They are evidence about *executed play under a modeled field*, never statements about exact values, never a status change, never quotable upward. Full Rust CI (`walt/ci/check.sh`) was **deliberately waived** for the whole program under the launch brief; every "gate" below is a focused check with the scope stated. A walt number on this page is quotable only through the results file or test named beside it; a number with no such anchor is labeled a probe record.

**Repository state as of 2026-09-07 (c00717d1).** The program is 31 commits, `d8400713` (2026-09-06 11:43 −0500) through `c00717d1` (2026-09-07 09:45 −0500), on branch `codex/partnership-launch` from `walt-gran` base `9d6a5a2`. `SESSION-STATUS.md` (written at `b0c7c0aa`) records the work through `b0c7c0aa` as merged directly into local `main`; at the book's snapshot `main` contains all 31 commits through `c00717d1` (branch containment verified read-only 2026-09-13). No live default changed (the standing ruling CE-A7/§20.16, restated CBS-A9, APS-A9, MB-A7, FH-A10 — [walt-calculated-evidence](walt-calculated-evidence.md)).

---

## 0. Three doorways

- **Newcomer to 42.** Walt is a computer player that samples the hands it cannot see and picks the domino most likely to make its team's bid ([game-of-42](game-of-42.md) explains bids, count, and partners). This program asked one question: does giving Walt a *thinking partner* — a model of its teammate that reasons rather than plays by rote — make it win more contracts? Under every fair test it ran, the answer was **no measurable gain**, at about five times the cost. What it left behind is a phone-faithful reference player, a battery machine that plays hundreds of verified games in minutes, and an exact exam for partnership positions. Read §1, §4's summary table, and §9.
- **Mathematician.** The objects are: a fixed-field best response on the Boolean pmake objective over sampled lawful completions (the level-1 seat of [walt-seat-play](walt-seat-play.md)); its level-2 partner variant (partner modeled at L1 — a best response to a *named* L0 field, never an equilibrium); a mirrored-contract pair score make(A) − make(B) with both-make and both-set pairs tied regardless of points; and, in §8, the finite performance-difference identity and interval bound of the relational packet. Read §2–§3, then §8.
- **Engineer.** `player.py` is a JSON-lines oracle; `match.py init` + `pool.py` + `verify_campaign.py` + `match.py report` is the battery recipe; `gym.py generate` regenerates the exact exam. Everything runs beneath a 295 s process-group watchdog. Read §3, §5–§6, §10.

---

## 1. The partner problem

**Tier: EXPLORATORY (packet provenance; program contract).**

### 1.1 The launch packet

The program began with a packet prepared outside the repository on 2026-09-06 for "a fresh Codex session on Jason's M5 Max" (`packet/texas42-partnership-launch-v0.1/START-HERE.md`; it did not pass through the `exchange/` dispatch ledger — an open bookkeeping question, §9). The packet is preserved byte-for-byte; all six entries of its `MANIFEST.sha256` verified at commit `d8400713`, again in the 2026-09-07 survey, and again with `shasum -a 256 -c` measured 2026-09-12 on this machine (`EXPERIMENT-BRIEF.md` 0193b56c…, `RESULTS-TEMPLATE.md` 9f2b00fa…, `START-HERE.md` 62e343d7…, `math/TEXAS42-IMPROVISATION-v0.1.md` 867db550…, `math/TEXAS42-UNIFIED-REVIEW-v0.1.md` daae4332…, `tools/run_capped.py` b4a5a17a…). Its two mathematical notes are source material whose companion-verifier claims were **not** independently rerun (`SCOPE.md`).

The brief's mission (`EXPERIMENT-BRIEF.md` §1): Walt 1 wins and is fun to play, but "its decisions do not adequately account for what its partner wants to accomplish or how a play changes the partner's information and response." The task: a playable trick-1 player on the M5 Max, information-consistent, showing useful partnership behavior, "at least comparable strength to Walt 1 at reasonably comparable sampling." The brief says in so many words that "an L2 that loses is useful knowledge."

**The four questions** the player should be able to distinguish (`EXPERIMENT-BRIEF.md` §2, verbatim):

1. "If I show this tile now, what uncertainty does that resolve for my partner, and what might they do differently?"
2. "Does this play support a continuation my partner can actually recognize and execute?"
3. "Am I preserving their route to the lead, spending a resource they need, or making them commit before useful information arrives?"
4. "Will giving count to a partner's winner or keeping a control tile change partnership contract success?"

**The two Gran positions** the brief names (from `walt/LEVEL2-PROBE.md`; the reconstructed fixtures are in [walt-gran-anchors](walt-gran-anchors.md)):

1. A partner can reveal 5-5 during play, relieving the bidder's uncertainty about an opponent holding it; the old evaluator saturation-ties choices whose informational consequences differ. In this program this corresponds to fixture `g3-t4-s2` (Gran holds 5-5 among four legal tiles at trick 4; the historical 160-world review saturation anchor).
2. Under the bidder's winning 6-6 lead, the partner holds 6-2 and the ten-count 6-4; the timing of giving count and revealing control affects what the bidder can infer. This is fixture `g1-t1-s2`.

### 1.2 The contract

| Term | Value | Source |
|---|---|---|
| Lawfulness and information | Every acting policy sees only its own seven original tiles, the public record, contract, declaration, and public randomness; the partner's hidden hand is never an input. Proposed continuations must agree across indistinguishable worlds. | brief §3; `README.md` "Decision interface" |
| Latency | ≤ 60 s of combined computation per four-play trick on the M5 Max, from trick 1; human thinking excluded; belief work, model calls, synchronization included | brief §1, §3 |
| Wrapper | ≤ 14 s per decision including validation, fallback, worker startup, cleanup (4 × 14 = 56 s < 60 s) | `SCOPE.md`; `README.md` |
| Bid | Fixed at 30 in every campaign; contracts chosen by a fixture rule, never an auction | `SCOPE.md`; `CAMPAIGN.md` |
| Strength reference | "Walt 1's actual phone version/configuration" — the preserved plunge WASM (§2) | brief §1 |
| Watchdog | Every build, check, and experiment beneath `run_capped.py` at ≤ 295 s (five seconds under the 300 s hard ceiling); no detached jobs | `SCOPE.md`; `packet/tools/run_capped.py` |
| CI | Full Rust CI **waived**; focused checks replace it | brief §1 ("CI" row); repeated in every report |
| Hardware | Apple M5 Max, 48 GiB, 18 cores; Rust/cargo 1.95.0; Python 3.9.6; Node 26.0.0; isolated native build 4.440 s | `SCOPE.md`; `SUMMARY.json` runs `006-build` |
| Tier | EXPLORATORY throughout; "no global optimality or strength claim follows from a legal, fast decision" | `SCOPE.md` |

**What the program measures.** Play, never the auction. The phone's auction is plunge's heuristic ladder, not Walt (§2.1); every comparison freezes bidder, trump, and bid 30, and every report repeats that no result here measures an auction improvement. The objective is pmake (ruled 2026-08-17, [walt-program](walt-program.md)): a contract is *made* when the declaring partnership banks ≥ 30; extra points never rank results.

---

## 2. What the phone is

**Tier: EXPLORATORY (artifact identity, SHA-256- and blob-pinned; source inspection of plunge).**

### 2.1 The artifact

| Coordinate | Value | Source |
|---|---|---|
| Files | `walt.wasm` SHA-256 `af0200af8dc99d5a95ac898cded12861ca4a36c38b336dbbd6a33a515c071d59`; `walt.ts` `f05aeca0e7f680878fdf601410e6d15ac5911ee8935094fc0fa233888901d606` | `reference/phone/MANIFEST.sha256`; re-verified with `shasum -a 256 -c`, measured 2026-09-12 on this machine: both OK |
| Copied from | `/Users/jason/code/plunge/src/ai/walt/` at plunge HEAD `122ea7a59362c15850e7b54e76ce4353ddb9d07a` (clean), 2026-09-06; the binary entered plunge at commit `1810da20` (2026-08-22) "copying the Texas 42 handoff" | `PROVENANCE.md`; `BASELINE.md` |
| Identified as | byte-identical to texas-42 `walt/walt-wasm/pkg/walt.wasm` at commit **`9a056f20`** (2026-08-19 01:36 −0500, "walt: theta calibrated — default 11/16"); git blob `714773c391ccd7e02b234218f3229a2c50a172fd` | commit `c51ff020` (2026-09-06 17:47); re-verified 2026-09-12: `git rev-parse 9a056f20:walt/walt-wasm/pkg/walt.wasm` = `git hash-object reference/phone/walt.wasm` = 714773c3… |
| The θ = 11/16 caveat | `9a056f20`'s subject is the bidcurve θ calibration ([walt-seat-play](walt-seat-play.md)); the body records "wasm rebuilt, smoke 28/28 vs native". The phone binary therefore carries the θ = 11/16 bid handler — but plunge's live auction never calls it (next row), so θ is irrelevant to what the phone bids. | `git show -s 9a056f20`; `BASELINE.md` |
| Live play configuration | `n = 40` outer worlds, `n0 = 8`, **`race = true`** (`src/ai/walt/requests.ts:35-38,139-157`; `index.ts:69`) | `BASELINE.md` |
| Play seed | FNV-1a 32-bit hash of `walt/${handNumber}/${shaker}/${marks[0]}-${marks[1]}` — public state, not the hidden deal (`requests.ts:108-113`, `src/engine/rng.ts:17-25`) | `BASELINE.md` |
| Auction | plunge's heuristic **`mediumBid`** ladder (`hard.ts:177-185` → `medium.ts:110+`), **not Walt**; declaration uses Walt at 40/8 | `BASELINE.md` |
| Review mode | full evaluator, `race = false`, `n = 40` or "look closer" 160, seed FNV-1a of `explain/…` — fresh estimates with a different seed/mode from played decisions | `BASELINE.md` |
| Historical solver source | `9a056f20` precedes the 2026-08-24 crate fold, so the source is `walt/walt-m3-probe/src/lib.rs`; a reproducible rebuild was **not** attempted | `HEAD-TO-HEAD.md`; `PROVENANCE.md` |
| Not verified | the version actually deployed on any phone; only the local checkout was inspected | `BASELINE.md`, `PROVENANCE.md` |

`phone.mjs` loads that wrapper and binary unchanged, answers one play request (or a `--stream` of them), selects only own-hand/public fields, requires an explicit unsigned decimal seed, and has no timeout logic — the parent owns the clock. Node ≥ 23.6 is needed for the TypeScript wrapper.

**Divergence from the current repository** (`BASELINE.md`, source anchors in the launch worktree): the current `walt/walt-wasm/pkg/walt.wasm` is `d7f61f22…` and `walt.ts` `9af4114d…` — different files; `api.rs:194-198` defaults to 40/8, seed `0xB7E151628AED2A6B`, **race off**, 120 s native budget; the current API bidder uses θ = 11/16 and walks the declaration upward. Two facts that matter for every comparison below: nominal `n = 40` never means only 40 worlds of work (exact-equal ties trigger 4× then 16× refinement; race mode supplies a `2n` cap and refinement base `n`, in blocks of 8), and **native deadlines expire while the WASM deadline never does** — a native deadline hit is not byte-equivalent evidence for the no-clock WASM path.

### 2.2 The calibration: fixed native L1 is behind the phone

Campaign `native-l1-vs-phone-620600-649` (seeds 620600–620649; `CALIBRATION.md`, `RESULTS.md`, `STATUS.md`): 50 paired deals, 150 games, 4,200 independently replay-verified moves, completed in **202.12 s**. Native `baseline` at fixed 40/8, voidless, **no tie refinement** versus the archived phone at 40/8 **with** race/refine.

| Native role | Favorable flips | Unfavorable | Ties |
|---|---:|---:|---:|
| declaring | 5 | 8 | 37 |
| defending | 3 | 7 | 40 |
| **combined** | **8** | **15** | **77** |

Net **−7.0 percentage points** per matched contract opportunity ((8 − 15)/100); descriptive ±2 SE band −14.0 to +0.0 pp — a rough normal approximation, not an equivalence test (the predeclared practical-equivalence band was ±5 pp, and absence of a detected difference would not have established equivalence). Phone declaring 31/50 makes, native 28/50, phone facing native defense 35/50. Decision time: phone 0.636 s vs native 0.080 s per move (0.964 vs 0.121 s nonforced). Fallbacks: phone 15/1843, native 0/908. At identical public histories 145/241 nonforced choices agreed (7 involved a fallback). Downside e-process 59049/8192 (threshold 20, never triggered).

`HEAD-TO-HEAD.md` recounts the two mixed arms as a direct head-to-head: contract wins native 43 vs phone 57 over 100 plays; mirrored pairs **3 W / 10 L / 37 T** — the same data, not new evidence (L1 contract wins = d + (1 − f); pair score d − f; the all-phone anchor cancels from the net).

**Label error carried from the survey (2026-09-07):** this campaign's `RESULTS.md` ends with the random campaign's boilerplate about excluding seeds 420601–420603, which does not apply to seeds 620600–649. The numbers above are unaffected.

### 2.3 The foundation: native `l1-race` reproduces the phone procedure

Commit `9236ca7f` (2026-09-06 18:44) made `solver/selection.rs` the single selection authority for the real root and every modeled level ≥ 1 (`FOUNDATION.md`; code detail belongs to [walt-architecture](walt-architecture.md)). The three rules, from `BASELINE.md`/`HEAD-TO-HEAD.md` and `selection.rs` (verified 2026-09-12, line 195): **Fixed** — one common-world bundle, exact ties to the lowest tile index; **Refine** — fresh 4× then 16× bundles on exact top ties, replacement not pooling; **RaceRefine** — common-random-number blocks of 8, a candidate eliminated once at least 6 paired blocks are in and the exact binomial tail is ≤ 1/128, up to a `2n` cap, survivors refined. L0 stays the fixed Dice response.

Parity evidence, all EXPLORATORY:

- **64 native/archived-phone decision comparisons agree** at matched information states — 32 race-refine choices and 32 full-refinement choices, including all available phone option values; native binary 7d5245ba…, phone af0200af… (`runs/foundation-parity.json`: `comparisons: 64`, 64 rows keyed seed/ply/rule/choice).
- In the 50-deal `l1-race` vs `phone` match (§4.5), **all 35/35 fallback-free mirrored pairs played identically for all 28 moves**; the 12 first divergences all began at decision 0 with a phone `l1-fallback` route; 1,064 common-prefix decisions (`campaigns/foundation-battery/phone-trajectory-audit.json`: `fallback_free_pairs 35`, `fallback_free_identical_pairs 35`; divergences at seeds 720601, 720603, 720605, 720606, 720607, 720617, 720622, 720632, 720636, 720644, 720646, 720648).
- Bounded source audit (`HEAD-TO-HEAD.md`): `level1_raced`, `level1_race_refined`, `level1_evaluate` preserve their sampling schedules and selection logic; `best_of` and `record_hash` unchanged; `sample_belief` adds a feasibility precheck; viewer recursion reorders children for earlier Boolean exits without pruning.

The program's own phrase for this: **"measured parity, not whole-program equivalence."** The 3/1 pair edge of `l1-race` over the phone (§4.5) is therefore about the two bounded implementations — the phone's wall-clock fallbacks — not a discovered difference in the completed decision rule. Native `l1-race` is the procedural anchor for controlled comparisons; the literal phone is retained for artifact checks; the older fixed-L1 measurement (§2.2) is not retroactively replaced.

---

## 3. The player families

**Tier: EXPLORATORY (definitions; `players.json` verified by `match.py players --all`, 18 presets, measured 2026-09-12).**

### 3.1 The coordinates

A player in this program is a point in five independent coordinates (`PLAYERS.md`):

| Coordinate | Choices | Meaning |
|---|---|---|
| **Family** | L1 / L2 Partner / L2 All | which other seats receive modeled L1 thinking; the rest are L0. L1 (`--mode baseline`) models all three at L0. **L2 Partner** (`--mode partner`) models the partner at L1 and opponents at L0. **L2 All** (`--mode all-l1`) models all three at L1 — a separate family, absent from the default battery. "L2" alone means L2 Partner in this program. |
| **Root search** | Fixed / Refine / Race | the selection rule at the real root (§2.3) |
| **Modeled search** | Fixed / Refine / Race | the same rule inside modeled L1 minds; inactive for L1 itself; L0 stays fixed |
| **Inner belief** | `voidless` / `voids-counted` | whether modeled minds' inner samples respect public void deductions (§3.3); outer samples respect voids in both cases |
| **Sample budget** | root `n` / L0 `n0` / L1 `n1` | default **40 / 8 / 2**; a modeled L1 uses a two-world base bundle, not the real L1's forty — "sharing a decision procedure does not make those two approximations identical" |
| Execution allowance | ≤ 14 s per move | the wrapper's deadline including reserve and cleanup |

"Default" means the fixed-search settings, not the best-scoring policy. A modeled L1 mind is an explicitly budgeted approximation; a modeled mind is a best response to its own named field (L0 minds, which best-respond to Dice) — a level, never an equilibrium.

### 3.2 Fallback semantics

A fallback is an *execution route*, never a search family (`PLAYERS.md`, `README.md`):

1. The wrapper derives legality locally, retains the lowest legal move, and first prepares a complete **8/2 fixed L1 reserve for up to 1.5 s**.
2. It then spends the remaining allowance (≤ 14 s) on the requested mode.
3. If the requested search does not finish it returns the labeled **`l1-fallback`**; if even the reserve failed, the labeled **`legal-fallback`**. Forced moves have their own route.
4. An incomplete evaluation contributes no partial ranking. **Modeled minds never fall back**: an incomplete nested solve aborts its host attempt; only the outer wrapper may choose a clock-dependent move. The reserve uses the same selected belief strategy.

Measured strength and time include the whole executed policy, fallbacks and the time spent on the failed requested search alike. The consequence, stated in `FOUNDATION.md` and borne out in §4.5: under a clock cap, strengthening a modeled procedure can *weaken* the executed player by increasing fallbacks.

### 3.3 The inner-belief option

Commit `dbcc698f` (2026-09-06 17:02; `INNER-BELIEF.md`) added a shared `InnerBelief` enum, selected once on `Shared` and inherited by every nested level: `voidless` (default; own hand, played tiles, remaining capacities; the historical shuffle stream) and `voids-counted` (opt-in; adds public void deductions and samples through the existing exact `Kernel`/`FiberDp`/`sample_with`). `Key` and `PiKey` carry `voids: Option<[u32;4]>` — `None` legacy, `Some([0;4])` tracked with no deductions yet, `Some(masks)` tracked; the last two are distinct from `None`. Gates: nine pre-change decisions reproduced exactly (`runs/voids-before/golden.json`), 11 partnership Rust gates, 36 focused Rust tests, the 252 Python/native position comparisons, a wasm32 compile check.

Two typed cautions travel with it. It is **uniform physical support, not a behavioral posterior** — support ≠ belief ([belief-vs-support](belief-vs-support.md)); no partner model is fitted. And the counted sampler uses a **different deterministic stream**, so a single changed move cannot be attributed to void logic alone; every voids comparison below compares two implemented strategies as wholes.

### 3.4 `players.json`, verbatim

| Preset | `mode` | `inner_belief` | `selection` | `modeled_selection` | `n` | `n0` | `n1` | `budget_ms` |
|---|---|---|---|---|---:|---:|---:|---:|
| `l1-default` | baseline | voidless | fixed | fixed | 40 | 8 | 2 | 14000 |
| `l2-partner-default` | partner | voidless | fixed | fixed | 40 | 8 | 2 | 14000 |
| `l2-partner-voids` | partner | voids-counted | fixed | fixed | 40 | 8 | 2 | 14000 |
| `l1-fixed` | (= `l1-default`) | | | | | | | |
| `l1-race` | | | race-refine | | | | | |
| `l1-race-small` | | | race-refine | | 8 | | | |
| `l1-race-voids` | | voids-counted | race-refine | | | | | |
| `l1-refine` | | | refine | | | | | |
| `l1-refine-voids` | | voids-counted | refine | | | | | |
| `partner-race` | partner | | race-refine | race-refine | | | | |
| `partner-race-fixedmind` | partner | | race-refine | fixed | | | | |
| `partner-race-fixedmind-voids` | partner | voids-counted | race-refine | fixed | | | | |
| `partner-race-small` | partner | | race-refine | race-refine | 8 | | 1 | |
| `partner-race-small-voids` | partner | voids-counted | race-refine | race-refine | 8 | | 1 | |
| `partner-race-voids` | partner | voids-counted | race-refine | race-refine | | | | |
| `partner-refine` | partner | | refine | refine | | | | |
| `partner-refine-voids` | partner | voids-counted | refine | refine | | | | |
| `phone` | phone | (archived, 40/8, race on) | | | | | | |

Blank cells inherit the first row's defaults (the file lists only the overriding keys; `matchup.py`'s `Player` dataclass validates `n` 1..640, `n0`/`n1` 1..64, `budget_ms` 100..14000; the phone's selection and belief are fixed). Everyday names: **L1 default** = `l1-default` (same behavior as historical `l1-fixed`); **L2 Partner default** = `l2-partner-default`; **L2 Partner with voids** = `l2-partner-voids`; **L1 Race** = `l1-race` (the recovered-phone procedural anchor; *not* an alias of L1 default). Initialization expands every preset into an immutable match manifest, so later catalog edits cannot change a saved experiment.

---

## 4. Batteries

**Tier: EXPLORATORY throughout — executed-policy outcomes under generated contracts; every game independently replay-verified by `verify_campaign.py`; none is a statement about exact values.**

**Reading the numbers.** A three-arm campaign (§4.1–§4.4) plays each deal three ways — all phone, candidate declaring, candidate defending — and counts *contract flips* against the all-phone reference: a favorable flip is a candidate make where the phone was set (declaring) or a set where the phone made (defending). A mirrored match (§4.5–§4.6) plays each deal twice with partnerships swapped; pair score = make(A) − make(B); **contract win fraction = (1 + mean pair score)/2**, a comparative score, *not* absolute pmake. In both, **both-make and both-set pairs tie regardless of points** (making 30 and making 42 tie; set at 7 and set at 29 tie). "±2 SE" bands are descriptive normal approximations clustered by deal, not anytime-valid intervals and not equivalence tests. Matchups that share a deal panel share units and must not be summed as independent deals or subtracted to infer an unplayed matchup.

The five deal panels are distinct and are kept so here:

| Panel | Seeds | Design | Candidate vs reference | Result | Combined | Record |
|---|---|---|---|---|---:|---|
| Launch | 420601–420603 (dev) | 3 deals × 3 lineups + diagnostics, 12 hands | partner-only 40/8/2 fixed vs phone | 0 favorable / 1 unfavorable / 5 unchanged | — | `REPORT.md`, `SUMMARY.json` |
| Random | 420600–420699 | 100 deals × 3 arms, 300 games; 97 fresh | partner-only 40/8/2 fixed vs phone | 15 / 23 / 156 | −4.1 pp | `campaigns/random-420600-699/` |
| Fixed-hand | 520600–520699 | 10 hands × 10 completions × 3 arms, 300 games | same | 21 / 27 / 152 | −3.0 pp | `campaigns/worlds-520600-699/` |
| Calibration | 620600–620649 | 50 deals × 3 arms, 150 games | fixed native L1 vs phone | 8 / 15 / 77 | −7.0 pp | `campaigns/native-l1-vs-phone-620600-649/` |
| Foundation | 720600–720649, 820600–820649, 730600–730624, 740600–740624 | mirrored matches, 524 published games | l1-race vs phone; race vs fixed; refine vs race; voids vs voidless; partner cost stops | 3/1/46; 4/5/41; 4/2/44; 9/5/36 and 5/8/37 | 52.0/49.0/52.0/54.0/47.0% | `campaigns/foundation-battery/` |
| Default | 750600–750699 | mirrored matches, 400 games | L2 Partner vs L1; L2 voids vs L2 | 14/14/72; 12/17/71 | 50.0%; 47.5% | `campaigns/default-partner-battery/` |

### 4.1 Launch batch — seeds 420601–420603 (0 / 1 / 5)

`REPORT.md`; `SUMMARY.json` (generated by `summarize.py` from `runs/000–015`); player checkpoint `cfb0fb25`, results `f5b6e11d`. Hardware and toolchain as in §1.2.

**Protocol.** Deals are `random.Random(seed).shuffle(range(28))`, seven consecutive tiles per seat; decision seed 420600 throughout, independent of the deal. 420601 used a preselected S0/sixes contract; after it proved uninformative at the contract level, 420602 (S3/ones) and 420603 (S1/threes) used a fixed fixture rule — longest pip-trump hand, then trump double, then off-trump doubles — "a fixture generator, not a lawful auction". Candidate = partner-only 40/8/2 fixed, voidless; six Rayon threads.

**Four separate findings** (`REPORT.md`):

| Dimension | Result | Limit |
|---|---|---|
| Lawfulness | 7 focused Rust gates; **252 native/Python position comparisons** across all declarations and seats; all **336** driven decisions legal with independent leader/score agreement; G1 referee replay reproduced 25–17 | focused evidence, not repository certification |
| Latency | 12 hands, **181.833 s** total gameplay computation; max four-play trick **24.238172 s** (hand `holdout-420601-1`); max decision **12.796294 s**; zero gameplay fallbacks | M5 Max, this suite only |
| Partnership behavior | the G1 6-2/6-4 root moves from an L1 tie to a strict partner-model preference for 6-4 (below) | the phone already chooses 6-4 with its own race/refine |
| Strength | **0 favorable contract flips, 1 unfavorable, 5 unchanged** across three fresh deals × two candidate placements; one extra mixed-team hand unchanged | three deal identities, not twelve |

Declaring-team points against bid 30 (all-phone / candidate declaring / vs candidate defense): 420601 **19 / 13 / 2** (all set); 420602 **4 / 23 / 36** — the candidate defenders let the declarer make 36 where the phone held it to 4, the regression; 420603 **25 / 24 / 15** (all set); partner-only-at-one-seat on 420603: 15. Candidate teams made 0/3 declaring and set 2/3 defending; phone teams 0/3 and 3/3. Regression ablation on 420602 (`SUMMARY.json` run `011-regression-ablation`): fixed-sample weaker-partner (`baseline`) defenders held the declarer to **25** (set); full L2 (`all-l1`) defenders allowed **36** (made) — the loss is not explained solely by removing the phone's race/refine.

**The G1 witness** (`g1-t1-s2`: sixes, bid 30, Jason led 6-6, S1 played 6-0, Gran chooses between 6-2 and the ten-count 6-4; seed 420600; same 40/8 outer evidence; `SUMMARY.json` run `007-matched-sampling-roots`):

| Mode | Estimated make after 6-2 | after 6-4 | Choice | Seconds |
|---|---:|---:|---|---:|
| Baseline (all L0) | 33/40 | 33/40 | 6-2 (index tie) | 0.095 |
| Partner-only, n1 = 2 | 34/40 | 37/40 | **6-4** | 0.790 |
| Full L2 (all-l1) | 38/40 | 36/40 | 6-2 | 1.555 |
| Phone (independent run) | — | — | 6-4 | — |

Changing only the partner model changes the ranking; modeling the opponents too reverses it again; these are sampled scores against a modeled field, not probabilities, and the ablation does not isolate revelation from score/resource effects. G3's trick-4 root (`g3-t4-s2`, legal 3-3/4-3/5-2/5-5) remained a four-way tie under all three native profiles (choice 3-3 by index) — the brief's saturation ambiguity was **not** resolved.

**The 602 one-world reversal** (fixture `regression-602-t3-s0`: defender S0, ones trump, S3 bid 30, identical public prefix through S3's 1-1 lead on trick 3; S0 may slough 3-2, 4-2, 6-2, 6-3, or 6-5; run `012-regression-root`):

| Mode | Declarer make estimate after 4-2 | after 6-5 | Defender's choice |
|---|---:|---:|---|
| Baseline | 20/40 | 21/40 | 4-2 |
| Partner-only | 20/40 | 19/40 | **6-5** |
| Full L2 | 17/40 | 16/40 | 6-5 |
| Phone | — | — | 4-2 |

A one-world-in-forty margin flips the choice; the complete-policy outcome regresses from set to made. `REPORT.md` is explicit that the first differing tile is a localized witness, not proof that this tile alone causes the loss. The proposed preregistered stability panel (independent public tapes, n1 = 2 vs 4) was **never run** (§9).

Also in the batch: a 100 ms decision returned its legal fallback in 53.577 ms; the table smoke drove all seven human prompts in 6.840 s (`014-table-smoke`); the watchdog smoke timed out as expected at 2.003557 s (`000-watchdog`).

### 4.2 Random panel — seeds 420600–420699 (15 / 23 / 156, −4.1 pp)

`campaigns/random-420600-699/RESULTS.md`, `STATUS.md`, `manifest.json`, `verification.json`; runner `campaign.py` (`CAMPAIGN.md`), completed `766168b7`. Three arms per seed, bid 30, contract by the fixture heuristic; candidate = partner-only 40/8/2 fixed, voidless; phone = archived WASM 40/8 race. Seeds 420601–420603 are flagged development and excluded from fresh evidence (420601 here uses the heuristic's twos contract, not the launch fixture's sixes; 420604, reserved and never consumed by the launch, appears as an ordinary fresh seed: S1/sixes, 42/42/36, tie).

| Measure | Value |
|---|---|
| Seeds / games / verified moves | 100/100 · 300 · 8,400 |
| Fresh seeds | 97 |
| Fresh paired outcome | **15 W / 23 L / 156 T** |
| Candidate declaring | 8 / 16 / 73 |
| Candidate defending | 7 / 7 / 83 |
| Phone made | 73/97 compared contracts |
| Combined estimate | **−4.1 pp** ((15 − 23)/(2 × 97)) |
| Fallbacks | 83 / 5,502 nonforced decisions |
| Longest decision / trick | 13.939 s / 33.961 s |
| Runner slices | 23.80 min |
| Downside e-process | 14348907/4194304 ≈ 3.42 (pause threshold 20, min ten fresh seeds; never triggered) |

The two role comparisons share a reference and are dependent; a seed's signed sum lies in {−1, 0, 1} and the seed is the statistical unit (`CAMPAIGN.md`). The e-process is a valid one-sided monitor only under the hypothesis that each fresh seed's conditional expected signed outcome is nonnegative, treating generated deals as fresh independent draws with the player and generator fixed. Execution changed from three games per seed to the ten-game shared pool after 48 committed seeds (§6); the report records this because deadline fallbacks can alter play.

Early witnesses (`campaigns/EARLY-WITNESSES.md`, at 24 seeds; phone / candidate declaring / vs candidate defense points): 420607 (S2 threes) 41 / 26 / 41 — declaration loses; 420612 (S0 blanks) 18 / 18 / 33 — defense loses, first divergence S3's 1-1 vs 2-2, no fallbacks (candidate defenders win the first four tricks for 9 points, then S0 takes 21 on trick five); 420615 (S1 sixes) 41 / 26 / 30; 420619 (S0 sixes) 25 / 35 / 42 — one win, one loss, net zero; 420622 (S3 fives) 35 / 35 / 24 — defense wins. At the ten-minute review (24 seeds, 21 fresh): 2 W / 4 L / 36 T.

### 4.3 Fixed-hand panel — worlds 520600–520699 (21 / 27 / 152, −3.0 pp)

`campaigns/worlds-520600-699/WORLD-RESULTS.md`, `RESULTS.md`, `PROTOCOL.md`; completed `fe17548c`. Ten opening hands (seeds 520600–520609, bidder rotating by group, pip trump by the heuristic from that hand alone, bid 30) × ten hidden completions each (the other 21 dominoes reshuffled; partner and opponent hands vary together) × three lineups = 300 games, 8,400 verified moves, **9.58 min** of pool slices. The private input at the opening is identical within a group and lineup; completed opening choices must agree across worlds.

| Measure | Value |
|---|---|
| Paired outcome | **21 W / 27 L / 152 T** (declaring 11/14/75; defending 10/13/77) |
| Combined | **−3.0 pp** ((21 − 27)/200) |
| Phone made | 57/100 |
| Fallbacks | 7 / 5,394; **zero opening fallbacks** |
| Longest decision / trick | 13.926 s / 30.950 s |
| Opening invariance | every lineup kept the same opening choice across its ten worlds |
| Mean decision latency | phone 0.704 s, native partner 0.590 s (different implementations and schedules, not a controlled cost ratio) |

Per hand (phone / candidate declaring / makes allowed by candidate defense, out of 10; paired W/L/T): H1 (S0, ones) 5/4/8, 1/5/14; H2 (S1, threes) 7/6/7, 1/2/17; H3 (S2, fives) 9/6/8, 3/5/12; H4 (S3, threes) 5/4/5, 1/2/17; H5 (S0, twos) 3/7/3, 6/2/12; H6 (S1, ones) 5/6/6, 1/1/18; H7 (S2, threes) 9/10/10, 1/1/18; H8 (S3, blanks) 4/3/5, 2/4/14; H9 (S0, fours) 5/4/3, 5/4/11; H10 (S1, blanks) 5/4/5, 0/1/19. H1's fixed candidate 2-1 lead (hand 1-0, 2-1, 2-2, 4-0, 5-1, 6-1, 6-2) yielded 14–42 declaring points and 4/10 makes; H5's candidate 6-6 lead improved makes from the phone's 3/10 to 7/10; H3 declined from 9/10 to 6/10.

**Unit of inference: ten hand clusters, not 100 independent opening positions.** The independent-deal sequential stop is disabled. The "native opening model scores" printed in the report (37/40, 39/40, 19/20, 1/1) are sampled scores against the modeled field, explicitly not calibrated probabilities.

### 4.4 Calibration — seeds 620600–620649 (8 / 15 / 77, −7.0 pp)

Given in full in §2.2. Three arms, 150 games, 202.12 s; fixed native L1 without refinement versus the racing phone. Its purpose was to anchor native L1 to the phone, and its result — a deficit — is what motivated the foundation. Because it and the partner campaigns (§4.2–§4.3) compared different candidates with the phone on **different panels**, their deficits cannot be subtracted to establish a partner-model gain (`SESSION-STATUS.md`).

### 4.5 Foundation battery — 720600–720649 / 820600–820649 / 730600–730624 / 740600–740624 (524 games)

`campaigns/foundation-battery/RESULTS.md`, `RESULTS.json` (`published_games 524`, `pool_wall_seconds 824.617`), `PROTOCOL.md`, `EXTENSION-1.md`, `EXTENSION-2.md`, `0N-*/MATCH.md`, `resume-proof.json`, `phone-trajectory-audit.json`; completed `e310e6ff`. Predeclared mirrored matches (`PROTOCOL.md`: six comparisons frozen before execution; no outcome-based stopping; technical stop at repeated worker failure or > 5% fallbacks for either player after 20 nonforced moves), all native profiles at 40/8/2 unless stated, 14 s wrapper, six native threads per game, ten-game pool. Random panel seeds 720600–720649 (50 deals, shared by all random matchups — **50 deal units, not 200**); conditional panel 820600–820649 (five focal hands × ten completions — **five units**). The pilot before it (`foundation-pilot/MATCH.md`): 6/6 deals, 0/0/6, `l1-race` 0.242 s vs phone 0.550 s per move, one phone fallback in 113 nonforced decisions; 10 live games paused and resumed with all 10 saved moves surviving.

**Completed panels:**

| A versus B | Paired deals | A W / L / T | A contract win fraction | Rough ±2 SE |
|---|---:|---:|---:|---:|
| `l1-race` vs `phone` (01-phone) | 50 | **3 / 1 / 46** | 52.0% | 48.0–56.0% |
| `l1-race` vs `l1-fixed` (02-fixed) | 50 | 4 / 5 / 41 | 49.0% | 42.9–55.1% |
| `l1-refine` vs `l1-race` (03-refine) | 50 | 4 / 2 / 44 | 52.0% | 47.1–56.9% |
| `l1-race-voids` vs `l1-race` (05-voids) | 50 | 9 / 5 / 36 | 54.0% | 46.5–61.5% |
| `l1-race-voids` vs `l1-race`, five focal hands × ten completions (07-worlds) | 50 | 5 / 8 / 37 | 47.0% | (suppressed: five units) |

All four random intervals include 50%: no strength ordering among fixed/refine/race is established, and the void option wins on random deals but loses on the focal-hand panel (per hand: 1/2/7, 1/2/7, 0/0/10, 2/2/6, 1/2/7 — three hands favor voidless by one pair, two tie). Phone fidelity: §2.3 (35/35 identical pairs; 12 divergences at phone fallbacks).

**Per-move wall latency under the ten-game pool** (forced moves and wrapper included; `RESULTS.md`):

| Player | s / move | Fallbacks / nonforced |
|---|---:|---:|
| `l1-fixed` | 0.137 | 0 / 929 |
| `l1-refine` | 0.313 | 0 / 926 |
| `l1-race` | 0.502 | 0 / 928 |
| `l1-race-voids` | 0.535 | 0 / 932 |
| `phone` | 0.778 | 17 / 923 |

**Seven partner cost stops — a cost finding, explicitly not a strength verdict.** Every race/refine partner configuration crossed the > 5% fallback gate after ≥ 20 nonforced decisions; no outcome was rerolled and no gate relaxed. Extensions 730600–730624 (n = 8, n1 = 1) and 740600–740624 (race root 40/8, modeled L1 one fixed n1 = 2 bundle) were declared before execution (`EXTENSION-1.md`, `EXTENSION-2.md`).

| Tested A | Completed pairs | A fallbacks / nonforced | s / move |
|---|---:|---:|---:|
| `partner-race` (04) | 2 | 10 / 33 | 3.582 |
| `partner-race-voids` (06) | 2 | 11 / 33 | 3.786 |
| `partner-race-small` matched (08) | 2 | 4 / 32 | 1.680 |
| `partner-race-small` anchor (09) | 2 | 3 / 31 | 1.562 |
| `partner-race-small-voids` (10) | 2 | 4 / 35 | 1.834 |
| `partner-race-fixedmind` (11) | 1 | 2 / 20 | 2.200 |
| `partner-race-fixedmind-voids` (12) | 1 | 2 / 20 | 2.285 |

(`04-partner/MATCH.md` shows 1/0/1 on its two pairs — two pairs are not evidence.) Even the final variant, which keeps modeled L1 at a single fixed bundle, exceeded the threshold on its first pair; root refinement over a partner field is itself costly, and the experiment does not isolate an implementation bottleneck.

**Totals:** 524 published games / 14,672 independently replay-verified moves in **824.62 s (13.74 min)** of four foreground pool slices (500 games in five completed panels + 24 in stopped prefixes); all **4,639** moves saved at the first slice boundary survived resume unchanged (`resume-proof.json`: `saved_games 178`, `saved_moves_preserved 4639`). `analyze.py` regenerates `RESULTS.md`/`RESULTS.json` from saved data and launches no players.

### 4.6 Default battery — seeds 750600–750699 (400 games)

`campaigns/default-partner-battery/RESULTS.md`, `RESULTS.json` (`published_games 400`, `completed_slice_wall_seconds 1098.449`), `PROTOCOL.md` (frozen `10950845`), `STRENGTH-ASSESSMENT.md`, `configuration-check.json`, `01-level/MATCH.md`, `02-voids/MATCH.md`, `resume-proof.json`; results `0b65e5b1`. Jason asked for the default-L1/L2 comparison and the void comparison at L2. Fixed search at the root and inside modeled minds throughout, 40/8/2, 14 s wrapper, the unchanged foundation engine and binary (`configuration-check.json`: exactly `mode` changes in match 1, exactly `inner_belief` in match 2; 18 presets validated; `source_and_binary_equal_to_foundation_battery true`). 100 fresh deals shared by both matchups — **100 deal units, not 200**.

| A versus B | Paired deals | A W / L / T | A contract win fraction | Rough ±2 SE |
|---|---:|---:|---:|---:|
| `l2-partner-default` vs `l1-default` (01-level) | 100 | **14 / 14 / 72** | **50.0%** | 44.7–55.3% |
| `l2-partner-voids` vs `l2-partner-default` (02-voids) | 100 | **12 / 17 / 71** | **47.5%** | 42.1–52.9% |

**Cost of the executed players** (`RESULTS.json` `pooled_costs`; per-match figures in the `MATCH.md` files):

| Player | s / move | requested search | reserve preparation | Fallbacks / nonforced | Fallback reasons |
|---|---:|---:|---:|---:|---|
| `l1-default` | **0.228** | 0.188 | 0.040 | 0 / 1,821 | — |
| `l2-partner-default` | **1.127** (1.128 in 01, 1.126 in 02) | 1.086 | 0.040 | 43 / 3,690 = 1.2% (24/1,852 + 19/1,838) | native-deadline-refusal 35, parent-timeout 8 |
| `l2-partner-voids` | **1.318** | 1.275 | 0.043 | 60 / 1,809 = 3.3% | native-deadline-refusal 50, parent-timeout 10 |

Neither comparison establishes a strength gain. L2 Partner tied L1 at about five times the per-move wall time; the void option scored lower and cost more but its interval includes 50%, so it is not an established loss either. A tied contract score does not imply identical choices: the trajectory diagnostics in `STRENGTH-ASSESSMENT.md` show different trajectories on 98/100 (L2/L1) and 100/100 (voids/legacy) deals, the same contract outcome despite different play on 70/100 and 71/100, first splits in tricks 1/2/3 on 69/25/4 and 78/20/2 deals. All 2,683 first-slice moves survived resume (`resume-proof.json`). The post-hoc Dirichlet(1,1,1) reading in `STRENGTH-ASSESSMENT.md` (posterior probability of a positive edge 50.0% and 18.1%, i.e. 81.9% that voidless L2 has the positive edge) is a model selected after seeing the results — quotable only as that document's own arithmetic, never as a strength result. **Practical ruling in `STRENGTH-ASSESSMENT.md`: L1 default "is a defensible operating default now without claiming a proved strategic ordering."**

These matches used fixed L1, not the racing native counterpart of the phone; the original phone-strength requirement remains open.

### 4.7 The two label errors (survey 2026-09-07; report as-is, not edited)

1. `campaigns/bench-pool10/STATUS.md` labels its 12 seeds "fresh" and prints 2 W / 2 L / 20 T; `campaigns/pool-benchmark.json` (`repeated_known_deals: true`) and `POOL.md` say they are repeats of seeds 420636–420647. The generic STATUS template is wrong; the JSON wins — the benchmark is throughput calibration, not strength evidence (§6).
2. `campaigns/native-l1-vs-phone-620600-649/RESULTS.md` carries the random campaign's 420601–420603 exclusion boilerplate (§2.2).

A third source of confusion is not an error: `README.md`'s "L2 Partner tied L1 on the 100-deal panel" refers to 750600–750699, not the 100-seed random campaign 420600–420699.

---

## 5. How the evidence is made trustworthy

**Tier: EXPLORATORY receipts — engineering checks, never rob receipts.** rob's receipts are byte-diffed in CI against verifier-generated files ([rob](rob.md)); these are focused checks run beneath a CI waiver, and the page says so each time.

| Mechanism | What it establishes | Where |
|---|---|---|
| Independent Python referee `rules.py` (hash `eed6291d…` pinned in every manifest) | every native and phone move re-checked for legality, leader, and points against the referee's own deal; **252** native/Python position comparisons across all declarations and seats | `checks.py`; `REPORT.md`; `campaigns/VERIFICATION.md` |
| Hidden-completion invariance (cold and shared-cache), banked-score/own-hand cache separation, all-zero/all-one field equivalence, CRN parity, deterministic complete actions, immediate deadline refusal, real subprocess timeout, invalid/private-input rejection, worker-startup failure, malformed-fallback rejection | the decision is a function of the seat's information; nothing hidden leaks in; fallbacks are what they say | 7 launch Rust gates; 11 partnership gates after `dbcc698f`; 44 focused Rust tests after `9236ca7f` (+1 pre-existing ignored) |
| Nested-mind purity | a modeled mind reconstructs its samples from its own key; the host's world vector is absent; an incomplete nested solve aborts the host; configuration fixed before any shared cache use | `FOUNDATION.md` design obligations 3–4 |
| Atomic per-move checkpoints; contiguous-seed commits | fast games cannot select the evidence the sequential rule sees; commit-time replay rejects a result that disagrees with its checkpoint | `POOL.md`; `CAMPAIGN.md` |
| Resume proofs | **4,639** moves (foundation), **2,683** (default), **406** decisions across 23 checkpoints at the random campaign's pool-slice boundary — all preserved exactly; cooperative 1 s pause byte-equivalent on resume (`calibration-t4/pause-proof.json`); hard kill after 2 s restarted to identical move/route sequences (`calibration-hard-stop`) | `resume-proof.json` files; `pool-resume-proof.json`; `VERIFICATION.md` |
| Pool lock; per-campaign locks | a second pool process is refused without launching workers | `POOL.md` |
| `verification.json` per campaign | independent replay of every committed game plus manifest identity and aggregate arithmetic | `verify_campaign.py` |
| Pinned identities | every manifest carries SHA-256 of the native binary, `player.py`, `rules.py`, `phone.mjs`, the phone artifacts, and (later) `matchup.py`, `pool.py`, `runtime.py`; source/binary changes refuse silent resume | Appendix B |
| Watchdog receipts | every run directory keeps command, elapsed, exit, stdout/stderr, binary identities, seeds/settings, work counters | `runs/*/run.json` |
| Threads | 4 vs 6 threads on seed 420601 gave identical moves and routes in all three games (summed decision seconds phone 10.775 vs 11.021; candidate declaring 14.441 vs 13.651; defending 11.601 vs 11.328) | `VERIFICATION.md` |

Why a fallback is part of the executed player: a deadline fallback is a lawful, timing-dependent move that the player actually made; it stays in the score and is never retried "until a preferred outcome appears" (`POOL.md`). The one caveat carried on every timing number: scheduling prevents a mathematical real-time guarantee; the legal reserve is the operational boundary and overruns are measured, not prevented.

---

## 6. Infrastructure costs and throughput

**Tier: EXPLORATORY (wall-clock measurements on the M5 Max under load; not CPU benchmarks).**

| Measurement | Value | Record |
|---|---|---|
| Shared ten-game pool vs the three-games-per-seed schedule, repeated seeds 420636–420647 | **36 games in 84.642 s** vs a 250.236 s critical path = **2.96×**; one snapshot 90.55% CPU busy, no swap; all 36 make/set outcomes matched; two trajectories changed (420644 phone and defending); fallbacks 8 → 14 (14/654 nonforced); longest decision 13.931 s, trick 19.698 s | `campaigns/pool-benchmark.json`; `POOL.md` |
| Random campaign, last 52 seeds | two pool slices, 371.31 s (6.19 min); 13 finished-but-uncommitted games retained and 10 partial games resumed at the boundary | `pool-resume-proof.json` |
| Random campaign, whole | 300 games, 23.80 min of slices (first main slice 13 seeds / 39 games in 270.093 s) | `RESULTS.md`; `VERIFICATION.md` |
| Fixed-hand panel | **300 games in 9.58 min** | `worlds-520600-699/RESULTS.md` |
| Calibration | 150 games in 202.12 s (3.37 min) | `CALIBRATION.md` |
| Foundation battery | **524 games in 13.74 min** (824.62 s) | `RESULTS.json` |
| Default battery | **400 games in 18.31 min** (1098.45 s) | `RESULTS.json` |
| Per-move latency by preset | §4.5 table (0.137 / 0.313 / 0.502 / 0.535 / 0.778 s) and §4.6 table (0.228 / 1.127 / 1.318 s) | match reports |
| Opening cost structure | dominated by modeled-policy calls and recursive nodes, not the outer sampler — no GPU or exact-integration port was needed to meet the trick target | `REPORT.md` |

Roughly 1.1 h of pool wall across the four big batteries (23.80 + 9.58 + 13.74 + 18.31 = 65.4 min). Persistent workers (`runtime.py`) reuse process startup, never evaluation state; every request builds fresh native evaluator state and supplies explicit phone randomness. The `pool-queue.json` (workers 10, seconds 260, retries 2) shares one Mac budget across campaigns; ≤ 2 retries then a STOP marker; interruptions do not count as failures.

**The lesson**, stated in `FOUNDATION.md` and `RESULTS.md` and worth its own line: **under a clock cap, a stronger modeled procedure can weaken the executed player** — the refined partner minds were unplayable inside the 14 s wrapper (§4.5), and fixed L2 Partner bought nothing measurable at five times L1's cost (§4.6).

---

## 7. Exact exercises: the Scheme gym

**Tier: EXPLORATORY (exact under a declared uniform mechanical belief and a fixed L1-40/8 teammate / L0-8 opponent field; diagnostic, field-relative, outcome-selected).** Owned by [walt-gym](walt-gym.md); the language by [walt-scheme-fix](walt-scheme-fix.md). Summary only.

`SCHEME-GYM-ASSESSMENT.md` (`0be750a9`, 2026-09-06) proposed using Scheme/Fix to describe *families* of partnership positions rather than single puzzles; Jason commissioned Scheme for expression the same evening (`b764665f`), and the gym followed (`c59f1115`). The ladder, each number in its `walt/gym/*.md` record: **6** starter count-offer exercises (three where offering count helps, three where holding it back helps; L1 default 5/6 with mean root regret 1/216 ≈ 0.463 pp, L2 Partner 6/6, L2 voids 5/6 — `walt/gym/RESULTS.md`); **170** distinct strict late-game coordinates discovered by three query files over 1,929 saved coordinates (`DISCOVERY.md`, `b0c7c0aa`); **433** outcome-only bid-making exercises, 367 with a unique best play, 26 certain make/set swings (`BID-MAKING.md`, `67f1e4ab`), reproduced from one specification in 67.707 s (`SPECIFICATIONS.md`, `75b6a3f1`); and the **30**-position composed exam requiring the best count-offer play to strictly beat the best non-offer, where default L1 chose optimally on **24/30** (mean regret 1643/205200 = 0.8007 pp) and L2 Partner on **26/30** (959/205200 = 0.4673 pp) — three improvements, one regression, a mean reduction of exactly 1/300, 60 decisions in 1.458 s with no fallbacks (`PARTNERSHIP-COMPOSITION.md`, `1df741db`). The gym is an exam surface for the players, not a player, and its 30 coordinates come from 21 source deals; it does not revise §4.

---

## 8. Learning a policy

**Tier: EXPLORATORY — negative strength results with instruments.** Both experiments are finite-sample-exact constructions under a frozen field; neither claims optimal Texas 42 play, and the existing L1/L2 player is unchanged by either. Vocabulary fence: the packet and the campaign reports use "interval regret certificate" and "certificate width" as their own terms for the bound (2)–(3) below; those are walt-internal packet terms, never the D3 sense, and are not imported upward (compare walt's own term of art Γ = U* − B_exec, *certified regret*, in [walt-counted-belief-era](walt-counted-belief-era.md)).

### 8.1 Policy synthesis (commit `08fad726`, 2026-09-07; `campaigns/policy-synthesis-v1/RESULTS.md`, `measurements.json`, `verification.json`; instrument `POLICY-SYNTHESIS.md`, `policy_campaign.py` + native `policy_lab`)

**Question.** Can a constructor of exact information-state tables retain or compose work as its training sample grows, while matching the policy obtained by solving the accumulated sample afresh? Three arms on identical nested training prefixes: `fresh` (empty cache), `persistent` (retains valid exact search state; must equal fresh), `compose` (restricted search over the union of every successful singleton-donor action; complete donors preserve the training optimum, `walt/scheme/COMPOSITION.md`).

| Panel | Roots | Training schedule | Test worlds/root | Campaign wall |
|---|---:|---|---:|---:|
| cheap fixed `hash-legal-v1` field, opening lead, sixes, bid 30 | 32 | 1…200 (nine stages) | 256 | 17.189 s |
| native L0-8 field | 12 | 1…32 | 64 | 64.590 s |

**Exactness:** across all 287 hash and 72 L0 three-arm comparisons, serialized policy identities and training/test outcome vectors agree exactly. One opening root hit the 2,000,000-node ceiling at 200 samples in fresh and compose; persistence completed it (excluded from the balanced timing).

| Complete paired schedules | Fresh | Persistent | Saving | Compose |
|---|---:|---:|---:|---:|
| hash field, 31 roots | 20.531 s | 9.664 s | **52.9%** (56.0% of nodes) | 21.831 s, **+6.3%** |
| L0-8, 12 roots | 114.369 s | 61.298 s | **46.4%** (49.4% of nodes) | 129.513 s, **+13.2%** |

Final-stage mean search 302 ms fresh vs 119 ms persistent at 200 samples; 4.744 vs 2.557 s at 32 samples in L0-8. Persistence halves the work with exact parity; donor composition produces the same program at greater cost.

**Generalization is the remaining issue:** at 200 samples training makes were **4,979/6,400 (77.8%)** vs **2,100/8,192 (25.6%)** on independent test draws; L0-8 **335/384 (87.2%)** vs **182/768 (23.7%)**. Going from one sample to the final count raised test makes 1,777 → 2,100 and 157 → 182. A 200-sample opening program stores ~880 decisions in ~0.34 MB. The constructor emits per-root exact tables with a lowest-legal fallback and infers no relational rules. On the 30 composed gym exercises (360 rows, 2.118 s; `policy_gym.py`): 1 sample 16/30 optimal first actions, first-action regret 9.763 pp, complete-policy gap 12.322 pp; 4: 22/30, 2.015, 3.551; 16: 25/30, 0.616, 1.299; 64: **26/30, 0.436, 1.008 pp** (exact 64-world regret 1649/378000, gap 3809/378000). Receipts: 222 full-game traces independently replayed; a real SIGINT after 1 of 6 seeds (runner exit 75, no native children) resumed to six seeds with the original result byte-unchanged.

### 8.2 Relational learning (commit `c00717d1`, 2026-09-07; `campaigns/relational-learning-v1/RESULTS.md`, `measurements.json`, `verification.json`, `packet-verification.json`; instrument `RELATIONAL-LEARNING.md`, `relational_campaign.py` + native `relational_lab`, `relational_exam.py`, `verify_relational.py`)

**The packet.** The Astra proposal `packet/texas42_relational_learning/PATH-TO-GENERALIZING-SCHEME-POLICIES-v0.1.md` (preserved unchanged in `14e01322`; its manifest names repository commit `08fad726`) supplies the mathematics. With `Q*(I,a)` the best lawful continuation value after `a`, `V*(I) = max_a Q*(I,a)`, and `c*(I,a) = V*(I) − Q*(I,a)`, for any complete lawful program `P` under its own induced trajectory distribution:

> **(1)** `V*(I₀) − V^P(I₀) = E_P Σ_t c*(I_t, P(I_t))` — the finite performance-difference identity specialized to this target (the packet says so: "not a novel general result").
>
> **(2)** with valid bounds `L(I,a) ≤ Q*(I,a) ≤ U(I,a)`, `c̄(I,a) = min{1, max_b U(I,b) − L(I,a)}` satisfies `0 ≤ c* ≤ c̄`; hence **(3)** `V*(I₀) − V^P(I₀) ≤ E_P Σ_t c̄(I_t, P(I_t))`.
>
> **(5)** for a prefix-coded policy grammar with code length `ℓ(P)` bits and Kraft sum ≤ 1, over `R` independent root bundles, with probability ≥ 1 − δ simultaneously for every program: `J(P) ≥ Ĵ_R(P) − sqrt((ℓ(P) ln 2 + ln(1/δ)) / (2R))` (Hoeffding + union bound + Kraft).

The packet's own `verify.py` passed 8,192 generic exact-rational policy checks (128 random 3-step games × 64 policies, seed 420907): the identity, the interval upper, a sample-fitting/zero-dual-gap example, a coarse-centering counterexample (pooling public information can invalidate a price), and a label-disagreement example (`packet-verification.json`, status PASS). These are mathematics checks, not 42 results.

**The learner.** A grammar of **14 mechanical clauses** (7 selectors × {Any, partner-currently-winning}); ≤ 3 clauses, 256 AST nodes, beam 6; one mode, no bindings, no exact keys; actor predicates see own/public information only, hidden Scheme events stay in the examiner. Design per panel: 144 source-deal groups = 32 train-a + 32 train-b (on-policy discovery) + 16 development + 64 untouched test; focal seat S0 with three dominoes, next to act, ≥ 2 legal plays, bid 30 unresolved, support cap 512, sixes; test fibers 11,836 worlds (gym field) and 8,892 (L0-8) — worlds, not independent hands; pipeline wall 39.411 s (10 workers) and 11.352 s (8). Three cost arms: exact regret, unpriced interval, priced interval. Frozen actors: 829 bytes (exact-regret: partner-winning count-10, then count-0, then master) and 707 bytes (interval); on L0-8, development selected the **empty 72-byte baseline** for all three arms.

**Held-out equal-root mean make probabilities** (`RESULTS.md`; exact fractions in `measurements.json`, e.g. table 90245471/195350400, actor 6364625549/15628032000, hybrid 7340057381/15628032000):

| Frozen actor / composition | Gym field | L0-8 |
|---|---:|---:|
| Sampled 16-world exact table + lowest-legal fallback | **46.197%** | **50.518%** |
| Shared exact-regret actor | 40.726% | 37.893% |
| Table + exact-regret shared fallback | **46.967%** | 50.518% |
| Shared interval-cost actor | 43.576% | 37.893% |
| Table + interval fallback | 46.228% | 50.518% |
| Exact lawful root optimum under the frozen field | 49.503% | 52.427% |

Paired root bootstrap (95%, descriptive): exact-regret actor − table **−5.47 pp** [−9.58, −1.76]; interval actor −2.62 pp [−4.52, −0.91]; L0-8 empty actor **−12.63 pp**. The one positive signal: the gym-field table + exact-regret fallback **+0.771 pp** [**−0.122, +1.909**], 11 roots improved, 9 harmed — "an uncertain positive signal, not an established strength improvement." Its mechanism is changed continuations after the table misses (first-action regret stays 0.693 pp; whole-policy gap 3.306 → 2.536 pp; first final fallback before resolution 45.50% → 24.83%). Priced and unpriced interval teaching selected identical programs.

**What prices told us.** The frozen four-event basis with coefficients `[0, 1, −1, 1]` tightened **0 of 64,806** action bounds across 128 discovery roots; mean unpriced upper-minus-Q width 0.186 pp (gym) and 0.378 pp (L0-8), unchanged by pricing; charged work 692,994 and 469,593 units unpriced vs 2,856,972 and 1,932,754 priced, about **4.1×** excluding the shared tree build. **The state-constant invariance:** with fixed continuation lowers, changing `max_b U(I,b)` adds the same constant to every action's cost at a state, so tighter uppers alone cannot rerank programs under this constructor — pinned by `walt/walt/tests/relational_learning.rs` (`changing_only_a_state_common_upper_cannot_rerank_programs`).

**On the 30-position partnership exam** (2.181 s, outcome-selected): table 25/30 optimal, gap 1.299 pp; shared exact-regret actor 21/30, 10.929 pp; table + exact fallback 25/30, 1.457 pp; shared interval actor **0/30**, 31.129 pp; table + interval fallback 25/30, 1.751 pp. "The learned rules are therefore not a partnership solution. Their weak transfer is useful counterexample material."

Validation: 66 focused Rust tests; 7 new + 12 existing Python runner tests; 8,694 retained full games and 25,935 focal traces independently replayed (4,032 + 4,032 + 630 games; 12,096 + 12,096 + 1,743 traces); SIGINT/resume returned status 75 and completed all 36 jobs with six durable results byte-unchanged. `verification.json` records that the current runner (24f8878a…) differs from the measured reference (afdcc85d…) by a later reentrant-signal fix; the native binary d0c6b02d… is unchanged. Raw job directories live outside the repository under `/Users/jason/data/texas-42/relational-*-v1` and `policy-synthesis-*-v1`; the checked-in `measurements.json` files are the durable record.

---

## 9. What is settled and what is open

**Settled (EXPLORATORY, as of 2026-09-07):**

- A lawful, information-consistent, playable partnership-aware player exists from trick 1 within the trick target (`player.py`, `table.py`; §4.1 latency).
- The phone is pinned: artifact af0200af… = texas-42 `9a056f20` (§2.1).
- Native `l1-race` reproduces the phone procedure — 64 decision checks, 35/35 identical fallback-free pairs — measured parity, not whole-program equivalence (§2.3).
- No tested partner model beat the phone or L1: 0/1/5, 15/23/156, 21/27/152 against the phone; 14/14/72 against L1 at ~5× cost; voids 12/17/71 (§4). Every race/refine partner configuration was unplayable under the 14 s wrapper (§4.5).
- L1 default is the defensible operating default (`STRENGTH-ASSESSMENT.md`); the live default is unchanged by ruling.
- The instruments: the ten-game pool with resume proofs (§5–§6), the exact gym (§7), the policy-synthesis and relational-learning laboratories (§8).

**Open (each with its record):**

1. **Does a cost-matched partner model add strength?** Fixed L2 Partner ties L1 (14/14/72); profiling the partner-field decisions is item 1 of `SESSION-STATUS.md` "What remains open"; nothing is running or scheduled.
2. **Is the 420602 trick-3 reversal n1 = 2 sampling noise or a stable model mismatch?** The preregistered stability panel in `REPORT.md` (independent public tapes, n1 = 2 vs 4) was never run.
3. **Void conditioning is confounded with the sample stream**: a coupled-world ablation is unbuilt (`INNER-BELIEF.md`, `PROTOCOL.md`).
4. **Selection order** among `l1-fixed` / `l1-refine` / `l1-race` (4/5/41, 4/2/44) is unresolved; `l1-fixed` stays default by cost.
5. **The deployed phone** was never independently verified; only plunge checkout `122ea7a5` was inspected.
6. **No pupil has run on the 433 or 170 collections** ("This pupil comparison has not been run", `BID-MAKING.md`).
7. **Relational learning**: whether the +0.771 pp hybrid survives a larger panel; whether any price basis helps beyond bound width; opening play, richer guards (score, history), mode induction, adaptive teacher allocation — all unbuilt (`RESULTS.md`).
8. **Bookkeeping**: neither packet (`d8400713`, `14e01322`) is in the `exchange/` ledger; `SUMMARY.json` covers only `runs/000–015`; the wiki ledgers carry no entry (no tier changed, so none is forced); this page is the owning chapter.
9. Auction strength and bid prediction are deliberately deferred; Brier scoring and oracle-makeability annotations were discussed, not implemented.

---

## 10. Reproduction

All commands from the worktree root; every long command beneath the watchdog with a fresh `--output-dir`. Requires Rust/cargo, Python 3.9+, Node ≥ 23.6. **Tests that spawn the native binary look for `walt/target/release/partnership` relative to the checkout root**; in a worktree without a built target they fail with `FileNotFoundError` (measured 2026-09-12 in this worktree: `python3 -m pytest -q experiments/partnership/test_gym_spec.py` → 7 passed, 1 failed on exactly that path; `player.py --help` and `match.py players` run without the binary). Build first, or run from the main checkout.

**Rebuild and check** (`README.md`):

```sh
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 295 --output-dir experiments/partnership/runs/my-build -- cargo build --release --manifest-path walt/Cargo.toml -p walt --bin partnership
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 60 --output-dir experiments/partnership/runs/my-checks -- python3 experiments/partnership/checks.py
```

**One decision** (`player.py`; one JSON object per line on stdin; the G1 6-2/6-4 root):

```sh
python3 experiments/partnership/player.py --mode partner
{"decl":6,"bid":30,"bidder":0,"seat":2,"hand":[1,5,11,12,17,23,25],"plays":[0,27,1,21],"seed":420600}
```

Options (verified `--help`, 2026-09-12): `--mode {partner,baseline,all-l1,phone}`, `--inner-belief {voidless,voids-counted}`, `--selection` / `--modeled-selection {fixed,refine,race-refine}`, `--n`, `--n0`, `--n1`, `--budget-ms 100..14000`. The response carries `choice`, `legal`, `leader`, `points`, `route`, timing, phase outcomes, and exact rational option values (declaring-team contract success; defenders minimize).

**Repeat a small experiment** (`experiment.py`):

```sh
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 120 --output-dir experiments/partnership/runs/my-roots -- python3 experiments/partnership/experiment.py roots --n 40 --n0 8 --n1 2 --modes baseline,partner,all-l1 --ids g1-t1-s2
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 295 --output-dir experiments/partnership/runs/my-hands -- python3 experiments/partnership/experiment.py hand --deal-seed 420601
```

**A mirrored match** (`match.py` + `pool.py`; `--panel worlds --worlds-per-hand 20 --count 200` for fixed-hand panels):

```sh
python3 experiments/partnership/match.py init experiments/partnership/campaigns/my-match --a l1-race --b phone --start 900600 --count 50
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 295 --output-dir experiments/partnership/runs/my-match-slice-1 -- python3 experiments/partnership/pool.py experiments/partnership/campaigns/my-match --workers 10 --seconds 270
python3 experiments/partnership/verify_campaign.py experiments/partnership/campaigns/my-match
python3 experiments/partnership/match.py report experiments/partnership/campaigns/my-match
python3 experiments/partnership/match.py players --all
```

Repeat the pool command with a new output directory to resume; `campaign.py stop PATH` / `resume PATH` pause and clear a match. The three-arm runner is `campaign.py init DIR --threads 6 [--start --count --panel random|worlds --candidate-mode --inner-belief]` then `advance DIR --seconds 260` beneath the watchdog (`CAMPAIGN.md`); `campaign_report.py`, `worlds_report.py`, `reference_report.py` refresh the reports.

**The gym** (`gym.py`; the exact exam is owned by [walt-gym](walt-gym.md)):

```sh
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 295 --output-dir experiments/partnership/runs/my-spec-1 -- python3 experiments/partnership/gym.py generate --spec walt/gym/specs/bid-making.json --output /tmp/my-gym --workers 10 --seconds 240
python3 experiments/partnership/gym.py run --gallery walt/gym/collections/bid-making-v1 --output /tmp/my-pupils --players l1-default l2-partner-default
```

**Policy synthesis and relational learning**: the `init`/`run` and `relational_campaign.py` recipes are in `POLICY-SYNTHESIS.md` and `RELATIONAL-LEARNING.md` (§8), with `policy_lab` and `relational_lab` built by `cargo build --release --manifest-path walt/Cargo.toml -p walt --bin <name>`.

**Play a hand** against the phone with the experimental partner: `python3 experiments/partnership/table.py [--deal-seed N] [--bid 31] [--decl 6] [--inner-belief …] [--selection …]` (seat 0, declare 30, no auction, no marks).

**Run-directory map** (`experiments/partnership/runs/`, 103 entries as of c00717d1): launch `000-watchdog` … `016-campaign-checks` (17); `voids-*` (12: before/golden, build, checks, compatibility, regressions, wasm-check); `foundation-*` (21 incl. `foundation-parity.json`, builds, clippy, gates, pilot slices, python, wasm/workspace checks) and `foundation-battery-verification`; `gym-*` (24); `scheme-discovery-*` (13); `bid-making-*` (4); `partnership-composition-*` (5); `policy-*` (6: `policy-lab-pilot-001`, `policy-lab-l0-pilot-001`, `policy-lab-u64-boundary`, `policy-synthesis-{opening,l0,gym}-v1`); `default-partner-verification`. Campaign trees under `campaigns/`: `random-420600-699`, `worlds-520600-699`, `native-l1-vs-phone-620600-649`, `bench-pool10`, `calibration-t4`, `calibration-hard-stop`, `foundation-pilot`, `foundation-battery/01…12`, `default-partner-battery/01-level,02-voids`, `policy-synthesis-v1`, `relational-learning-v1`, plus `pool-queue.json`, `pool-benchmark.json`, `EARLY-WITNESSES.md`, `VERIFICATION.md`.

---

## Appendix A — Seed ranges

| Range | Panel | Games | Deal algorithm |
|---|---|---:|---|
| 420600 | development decision seed (every native decision; public, independent of the deal) | — | — |
| 420601–420604 | held-out deals reserved before generation; 601–603 consumed by the launch, 604 never consumed by it | 12 (launch) | `random.Random(seed).shuffle(range(28))`, seven consecutive tiles per seat, sorted |
| 420600–420699 | random three-arm campaign (97 fresh) | 300 | same |
| 420636–420647 | pool throughput benchmark (repeats) | 36 | same |
| 520600–520699 | fixed-hand panel: opening-hand seeds 520600–520609, ten completions each | 300 | opening hand fixed, other 21 tiles reshuffled |
| 620600–620649 | native fixed L1 vs phone calibration | 150 | random |
| 720600–720649 | foundation random panel: four completed matchups (01, 02, 03, 05) share these 50 deals; the stopped `partner-race` / `partner-race-voids` prefixes (04, 06) also ran here | 400 completed + 8 stopped | random |
| 820600–820649 | foundation conditional panel (07): five focal hands × ten completions | 100 | fixed-hand |
| 730600–730624 | extension 1 (08, 09, 10: n = 8 / n1 = 1 partner profiles), stopped after two pairs each | 12 | random |
| 740600–740624 | extension 2 (11, 12: race root 40/8, one fixed n1 = 2 bundle in modeled L1), stopped after one pair each | 4 | random |
| 750600–750699 | default battery (shared by both matchups) | 400 | random |
| (per campaign) | policy-synthesis and relational-learning root seeds are recorded in each campaign's `measurements.json` / run JSON (e.g. the relational gym panel's `--seed 96000000`, the policy-synthesis interruption receipt's seeds 864000–864005); each root is an independent full-deal shuffle | — | full-deal shuffles per root |

## Appendix B — Manifest identities (SHA-256, first eight hex digits; full digests in the files named)

| Identity | Where pinned |
|---|---|
| Phone `walt.wasm` `af0200af…`; `walt.ts` `f05aeca0…`; git blob `714773c3…` | `reference/phone/MANIFEST.sha256`, `PROVENANCE.md`, every campaign manifest |
| Native `partnership` binary `b711b9b0…` — launch/random/worlds | `random-420600-699/manifest.json`, `worlds-520600-699/manifest.json` |
| Native `df2c4862…` — calibration 620600–649 (a later build; `player.py` `112fd800…`, `campaign.py` `e413a6b2…`) | `native-l1-vs-phone-620600-649/manifest.json` |
| Native `7d5245ba…` — foundation and default batteries (`player.py` `1a4cc0a9…`, `pool.py` `7d34094f…`, `runtime.py` `213b5760…`, `matchup.py` `0de89016…`, `phone.mjs` `e5063726…`) | `foundation-battery/*/manifest.json`, `default-partner-battery/*/manifest.json`, `runs/foundation-parity.json` |
| Native `relational_lab` `d0c6b02d…`; runner measured `afdcc85d…` / current `24f8878a…`; verifier `f1eeb01d…` | `relational-learning-v1/verification.json` |
| `policy_lab` pre-fix `a23cd6aa…` / post-fix `5529a91f…`; runner `e1f5d5bd…` / `d1fdb7b3…` | `policy-synthesis-v1/verification.json` |
| `rules.py` `eed6291d…` (every campaign); `player.py` `50da12bf…` and `phone.mjs` `f91cffd4…` (launch-era) | campaign manifests |
| Packet `run_capped.py` `b4a5a17a…`; brief `0193b56c…`; relational packet `PATH-…md` `deff51a3…`, `verify.py` `dbb0265c…` | packet `MANIFEST.sha256` files; `packet/texas42_relational_learning/manifest.json` |
| Current-repo (non-phone) `walt-wasm/pkg/walt.wasm` `d7f61f22…`, `walt.ts` `9af4114d…` | `BASELINE.md` |

## Appendix C — The fixtures with tile ids

`fixtures.json` (schema 1, tier EXPLORATORY): tile id = `high·(high+1)/2 + low` (0-0 → 0, 1-0 → 1, …, 6-6 → 27); teams {0, 2} vs {1, 3}; plays as flat chronological (actor, tile) pairs; decision seed 420600 is **new experiment provenance** — the original phone seed, shaker, marks, and auction were never recovered. The G1 and G2/G3 reconstructions themselves are owned by [walt-gran-anchors](walt-gran-anchors.md).

**G1** (`walt/probes/gran/g1.receipt.txt:24-32`; bid 30, sixes, bidder S0, recorded 25–17, set; complete deal, rules-validated in its source record; full hands are referee-only inputs):

| Seat | Ids | Tiles |
|---|---|---|
| S0 (bidder) | 0, 4, 9, 14, 22, 24, 27 | 0-0, 2-1, 3-3, 4-4, 6-1, 6-3, 6-6 |
| S1 | 2, 6, 13, 16, 19, 21, 26 | 1-1, 3-0, 4-3, 5-1, 5-4, 6-0, 6-5 |
| S2 (Gran, partner) | 1, 5, 11, 12, 17, 23, 25 | 1-0, 2-2, 4-1, 4-2, 5-2, 6-2, 6-4 |
| S3 | 3, 7, 8, 10, 15, 18, 20 | 2-0, 3-1, 3-2, 4-0, 5-0, 5-3, 5-5 |

Roots: `g1-t1-s0` (S0 leads; recorded 27 = 6-6), `g1-t1-s1` (after 6-6; recorded 21 = 6-0), **`g1-t1-s2`** (after 6-6, 6-0; legal 23 = 6-2 or 25 = 6-4; recorded 6-2), `g1-t1-s3` (recorded 7 = 3-1), `g1-t5-s2` (trick 5, recorded 12 = 4-2).

**G2/G3** (`g2g3.receipt.txt:11-46`; bid 31, sixes, bidder S0, recorded 36–0, made; a six-trick prefix, no complete deal asserted): S2's hand fully known — ids 8, 9, 13, 15, 17, 20, 25 = **3-2, 3-3, 4-3, 5-0, 5-2, 5-5, 6-4**; residual tiles 11, 14, 18 = 4-1, 4-4, 5-3 assignable to S0/S1/S3 six ways. Roots: `g2-t1-s2` (after 6-6, 6-2: 6-4 is the only six — forced, a legality sanity check) and **`g3-t4-s2`** (four legal: 9 = 3-3, 13 = 4-3, 17 = 5-2, 20 = 5-5; recorded 3-3; the 160-world review saturation anchor).

**`regression-602-t3-s0`** (from the launch batch, not Gran): defender S0, ones trump, S3 bid 30; S0's hand ids 8, 10, 11, 12, 23, 24, 26 = 3-2, 4-0, 4-1, 4-2, 6-2, 6-3, 6-5; after S3's 1-1 lead on trick 3 the legal sloughs are 3-2, 4-2, 6-2, 6-3, 6-5; `baseline_choice` 12 = 4-2, `partner_choice` 26 = 6-5.

## Appendix D — Commit timeline

| Date (−0500) | Commit | Event |
|---|---|---|
| 2026-08-19 01:36 | `9a056f20` | walt-wasm rebuilt at θ = 11/16; this `walt.wasm` (blob 714773c3…) is the phone's |
| 2026-08-22 | plunge `1810da20` | plunge copies the handoff `walt.wasm`/`walt.ts` |
| 2026-09-06 11:43 | `d8400713` | launch packet preserved (6/6 SHA-256); branch `codex/partnership-launch` from `9d6a5a2` |
| 12:04 | `cfb0fb25` | bounded partner-aware player, phone reference, independent referee |
| 12:17 | `f5b6e11d` | launch batch recorded: 12 hands, 0/1/5, G1 witness, `regression-602-t3-s0` |
| 13:07–13:20 | `d60bad42`, `e104cb21`, `e24cb45f` | resumable campaign runner, independent replay verifier, first 35 seeds with recovery evidence |
| 13:41–13:45 | `755d1869`, `1bab636b` | shared ten-game pool; 2.96× throughput |
| 13:52 | `766168b7` | random campaign complete: 300 games, 15/23/156 |
| 15:32–15:40 | `13255635`, `fe17548c` | fixed-hand panel complete: 21/27/152; walt-seat-play gains its partnership section |
| 17:02 | `dbcc698f` | selectable void-aware inner beliefs |
| 17:19 | `548ab6e8` | calibration 620600–649: 8/15/77; `SESSION-STATUS.md` |
| 17:47 | `c51ff020` | phone artifact identified as `9a056f20`; `HEAD-TO-HEAD.md` |
| 18:44 | `9236ca7f` | foundation: `solver/selection.rs`, mirrored arena, persistent workers, 64 parity checks |
| 19:12 | `e310e6ff` | foundation battery: 524 games, 3/1/46, 35/35 pairs, partner cost stops |
| 19:45–20:11 | `10950845`, `1c99cf6a`, `0b65e5b1` | default protocol frozen; player families named; default battery 14/14/72, 12/17/71 |
| 20:53, 21:31 | `5cbede4e`, `0be750a9` | `STRENGTH-ASSESSMENT.md`; `SCHEME-GYM-ASSESSMENT.md` |
| 22:04, 22:49 | `b764665f`, `c59f1115` | Scheme/Fix executable; first exact gym (6 exercises) |
| 2026-09-07 00:09 | `b0c7c0aa` | Scheme-driven discovery: 170 coordinates (`SESSION-STATUS.md`'s merged-into-`main` note is dated here; `main` now holds the whole series) |
| 00:44–01:14 | `67f1e4ab`, `75b6a3f1`, `1df741db` | 433 bid-making exercises; specifications; composed exam 24/30 vs 26/30 |
| 03:29 | `08fad726` | Scheme dynamics + persistent policy synthesis |
| 08:57 | `14e01322` | Astra relational-learning packet preserved |
| 09:45 | `c00717d1` | shared relational learner and information-price examiner; HEAD of the book's snapshot |

## Appendix E — The tier statement

Nothing on this page is quotable above the exploratory tier. The numbers are executed-play outcomes under modeled fields and finite-sample exact constructions under frozen fields; the receipts are independent replays, pinned hashes, and resume proofs produced beneath a deliberate full-CI waiver. A green receipt here is evidence that a program did what its record says, never a status change for any claim in [claim-ledger](claim-ledger.md) or [FINDINGS](FINDINGS.md). Sampled model scores (33/40, 37/40, 1/1) are not probabilities; contract win fractions are comparative, not pmake; "L2" is a best response to a named L0 field, never an equilibrium; `voids-counted` is uniform physical support, never a behavioral posterior; a phone-parity result is measured agreement on completed decisions, never whole-program equivalence. Where a survey map, a status file, or this page disagrees with a results file, the results file wins (§4.7).
