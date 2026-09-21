[Home](Home.md) · owns: Kiln — the opening bidding book for Plunge: the frozen scalar-price survey (`kiln-v1`), the actual-play campaign (`kiln-played-v1`), the sixes/36 calibration finding, the empirical bidder release in Plunge (2026-09-19), the sample-history instrument, the Sunshine Atlas and Pattern Workshop views of the played corpus · Sources: [`experiments/kiln/README.md`](../experiments/kiln/README.md), [`PLAYED.md`](../experiments/kiln/PLAYED.md), [`PLAYED-BIDDER-RELEASE.md`](../experiments/kiln/PLAYED-BIDDER-RELEASE.md) + [`played-bidder-release.json`](../experiments/kiln/played-bidder-release.json), [`PROGRESS.md`](../experiments/kiln/PROGRESS.md), [`CALIBRATION-SIX36.md`](../experiments/kiln/CALIBRATION-SIX36.md) + [`calibration-six36-summary.json`](../experiments/kiln/calibration-six36-summary.json), [`COUNTED-VALUES.md`](../experiments/kiln/COUNTED-VALUES.md), [`SAMPLE-HISTORY.md`](../experiments/kiln/SAMPLE-HISTORY.md) + [`sample-history-evidence.json`](../experiments/kiln/sample-history-evidence.json), [`ATLAS.md`](../experiments/kiln/ATLAS.md) + [`ATLAS-v1.json`](../experiments/kiln/ATLAS-v1.json), [`BICLUSTERS.md`](../experiments/kiln/BICLUSTERS.md) + [`BICLUSTERS-v1.json`](../experiments/kiln/BICLUSTERS-v1.json), [`survey-frozen-summary.json`](../experiments/kiln/survey-frozen-summary.json), [`one-third-audit-summary.json`](../experiments/kiln/one-third-audit-summary.json), [`halfway-audit-summary.json`](../experiments/kiln/halfway-audit-summary.json), [`played-completion.json`](../experiments/kiln/played-completion.json), [`played-startup-audit.json`](../experiments/kiln/played-startup-audit.json), [`played-live-audit.json`](../experiments/kiln/played-live-audit.json), [`extension-500-audit.json`](../experiments/kiln/extension-500-audit.json), [`extension-500-startup.json`](../experiments/kiln/extension-500-startup.json), [`extension-500-first-eight.json`](../experiments/kiln/extension-500-first-eight.json), [`backup-evidence.json`](../experiments/kiln/backup-evidence.json), [`completion-backup-evidence.json`](../experiments/kiln/completion-backup-evidence.json), [`phone-build-summary.json`](../experiments/kiln/phone-build-summary.json), [`backup.py`](../experiments/kiln/backup.py), [`played.py`](../experiments/kiln/played.py); [`walt/walt-player/README.md`](../walt/walt-player/README.md), `walt/walt-player/src/bin/kiln-worker.rs`, `kiln-play-worker.rs`; [`experiments/partnership/PLUNGE.md`](../experiments/partnership/PLUNGE.md), [`SUNSHINE-NOTES.md`](../experiments/partnership/SUNSHINE-NOTES.md) (2026-09-19 paragraphs); commit messages `101d805b..5ab08bbc` on `main` (2026-09-18 → 09-19). For §8 (added 2026-09-20): [`THREAT-PROBE.md`](../experiments/kiln/THREAT-PROBE.md), [`threat-probe-v1/RESULTS.md`](../experiments/kiln/threat-probe-v1/RESULTS.md) + `evidence.json`, `posthoc-ablations.json`, `candidate-0..7.scheme`, `posthoc-before-double-five.scheme`; [`MINING-PROBE-V2.md`](../experiments/kiln/MINING-PROBE-V2.md), [`MINING-PROBE-V2-CV.md`](../experiments/kiln/MINING-PROBE-V2-CV.md), [`MINING-V2-AMENDMENTS.md`](../experiments/kiln/MINING-V2-AMENDMENTS.md), [`mining-v2/RESULTS.md`](../experiments/kiln/mining-v2/RESULTS.md) + `result.json`, `cv-result.json`, `fit.json`, `catalog.json`, the 13 `.scheme` files; [`DECISION-MINING-V1.md`](../experiments/kiln/DECISION-MINING-V1.md), [`decision-mining-v1/RESULTS.md`](../experiments/kiln/decision-mining-v1/RESULTS.md) + `summary.json`, `result.json`, `fit.json`; [`WHOLE-GAME-CONTRASTS-V1.md`](../experiments/kiln/WHOLE-GAME-CONTRASTS-V1.md), [`whole-game-contrasts-v1/RESULTS.md`](../experiments/kiln/whole-game-contrasts-v1/RESULTS.md) + `summary.json`, `result.json`, `fit.json`; `PROGRESS.md` ("Outcome-first Scheme probe", "Fresh relational mining probes"); `SUNSHINE-NOTES.md` (2026-09-18 paragraphs); commits `231eb1b0`, `bd5b248b`, `ddb3cadc`, `3545a3fe`, `0eeafcb3`, `0a8e2b6a`, `5a8dd401`, `57196c4e`, `313a12dd`, `6fbb8294`, `b8172ef9`, `86792586`, `d534f2ba`, `95e90444`. For §9: [`HOT-PATH.md`](../experiments/kiln/HOT-PATH.md), [`CARRY-CACHE.md`](../experiments/kiln/CARRY-CACHE.md), [`COMPACT-CACHE.md`](../experiments/kiln/COMPACT-CACHE.md), [`SMALL-SUPPORT.md`](../experiments/kiln/SMALL-SUPPORT.md), the `*-parity-summary.json` / `*-timing-summary.json` pairs (allocation, bitset, carry-cache-{cold-parity,production,timing}, compact-cache, forced-dice, lto, packed-hash, rule-table-{parity,production,repeat,timing}, serial-counted, serial-grouping, small-support, sparse-buckets, visit-order), `counted-parity-summary.json`, `pool-timing-summary.json`, `compiler-target-summary.json`, `warm-checker-summary.json`, `phone-build-summary.json`, `sparse-phone-build-summary.json`, `rule-phone-build-summary.json`; `PROGRESS.md` ("Production", "Solver and phone verification"); [`walt/CPU-SPEEDUPS.md`](../walt/CPU-SPEEDUPS.md), [`walt/CPU-RELEASE-PLAN.md`](../walt/CPU-RELEASE-PLAN.md); commits `101d805b`, `9bb393d7`, `662f298c`, `7377ff32`, `1a43d763`, `f91533cc`, `43a5a297`, `446a53da`, `1dbea2c0`, `a8622306`, `850f589b`, `93cf7be2`, `8ee0cb71`, `91e8925e`, `b6c8b273`, `45f3c482`, `460fc77e`, `701e8589`. Results files (`*.json`) outrank prose; where they differ the page says so. Related: [walt-seat-play](walt-seat-play.md) (the deployed player Kiln measures), [walt-partnership-program](walt-partnership-program.md) (the Sunshine program Kiln serves), [walt-instruments](walt-instruments.md) (the uncurated landing paragraphs of 2026-09-18), [walt-scheme-fix](walt-scheme-fix.md).

# walt — Kiln, the opening bidding book for Plunge (2026-09-18 → 09-19)

**Tier: EXPLORATORY.** Everything here sits under `experiments/kiln/` and
`walt/walt-player/`, below every tier on [Home](Home.md); every number is
quoted from the file named beside it. **Sorting (curator, 2026-09-20): adds to
an area** — Jason's ruling names Kiln an area. Nothing here changes the rules
profile, the objective (pmake), the tier ladder, or the deployed player's
decision procedure; Kiln changed how Plunge *bids* for catalogue hands (§3).

## §0 What Kiln is

Kiln is a versioned opening bidding book for Plunge: for a fixed own seven-tile
hand and seat, what to bid, so the phone table bids by lookup instead of live
pricing. `README.md`: "Kiln removes bidding latency; it does not close the
partnership gap." It was built twice in two days. The first build (`kiln-v1`,
§1) priced bids with the same lawful L1/8-inner-world voidless evaluator as the
deployed player — "exploratory model pricing, not calibrated odds or a
perfect-information oracle" (`README.md`). Its one calibration check (§1.4)
found the model forecast 121/160 for a sixes/36 hand where the executed table
made 21/100; on Jason's direction of 2026-09-18 ("the unit is the player making
bids. Its internal forecasts are not the outcome being measured", `PLAYED.md`)
Kiln was rebuilt as `kiln-played-v1` (§2): play the deployed player at bid 30
through all 28 moves and record the declaring team's score distribution. That
build's 500-hand book is what Plunge bids from since 2026-09-19 (§3). What Kiln
is not, in its sources' words: not calibrated odds, not a perfect-information
oracle, "not a guarantee for higher contracts or humans"
(`played-bidder-release.json` `remaining_limit`), and "no partnership
improvement is claimed" (`SUNSHINE-NOTES.md`, 2026-09-19).

Where it lives: code and compact evidence in
[`experiments/kiln/`](../experiments/kiln/); the native workers
`walt/walt-player/src/bin/kiln-worker.rs` (a long-lived process wrapping
`walt_player::KilnPricer`; "only completed pure inner-policy answers may carry
across matching contexts") and `kiln-play-worker.rs` (the full-game transport;
"policy implementation remains `walt_player::decide`"); the game-side lookup in
the Plunge repository (`src/ai/kiln/book.ts`, `scripts/install-kiln.mjs`, per
`PROGRESS.md`). Raw data — SQLite databases and WAL, receipts, producer
bundles, full books, histories — live outside git under
`/Users/jason/data/texas-42/` (`kiln-v1/`, `kiln-played-v1/`); verified
snapshots under `~/data/texas-42/backups/` and, optionally, the **private**
Hugging Face dataset `jasonyandell/texas-42-walt-archive`, namespace
`kiln/snapshots/` (`PLAYED.md`, `backup.py`). Two further bins,
`kiln-decision-worker.rs` and `kiln-scheme-contrast-worker.rs`, belong to the
mining studies (§8). Commits: `101d805b` (2026-09-18, "Build durable Kiln bid
production and calibration with exact counted solver values") through
`5ab08bbc` (2026-09-19, "Record audited 500-hand completion and live empirical
bidder release"); Atlas `4e86a04f`, Pattern Workshop `4cd59ef7` (2026-09-19).

## §1 The scalar-price survey (`kiln-v1`) — superseded, frozen

### §1.1 Objective and contract (`README.md`)

"Historical model-survey objective (superseded)": 1,000 complete deals; all
four seats, nine declarations, targets 30–42; progressive 8 → 40 → 160 outer
samples; a versioned book validated against its receipts; instant bidding in
Plunge. Profile `kiln-l1-fixed-inner8-voidless-v1` (`survey-frozen-summary.json`).
Seeds 420600–421599 selected before evaluation; seed → first Plunge hand through
Plunge's Mulberry32 shuffle and initial-shaker draw; every deal retained. "All
prices explicitly condition on that own hand under Walt's uniform opening model,
not the smaller distribution induced by limiting play to a finite catalogue." A
shipping deal has all 468 entries (4 seats × 9 declarations × 13 targets).
Screening: eight-world prices at most 1/8 stop provisionally; 40-world prices
below 1/2 stop provisionally; others advance to 160; a preselected
deterministic 2% of all cells bypass screening and reach 160. "Screening is
heuristic and can miss viable bids; 0/8 is not impossibility. No score is
inferred for another target, and none is imputed from neighboring bids." Three
repeated own-hand identities exist among the 1,000 shuffles (seat 2 at seeds
420695/420778; seat 2 at 420702/421106; seat 1 at 421103/421573;
`PROGRESS.md`); Plunge reconciles repeats by the deepest saved price, never by
the actual opponent hands or the most favorable estimate.

### §1.2 The solver change that came with it (`COUNTED-VALUES.md`)

The survey worker's solver carries exact integer success counts inside the
recursion: at node K with alive set A(K) and C(K) successful indices the value
is exactly C(K)/|A(K)|; viewer actions share a denominator, modeled-seat
buckets partition A, weighted averaging cancels denominators, and the public
API returns the same reduced BigRational once at the boundary. "Implementation
proof sketch and exploratory regression evidence; not a new corpus theorem or a
claim of player strength." Validation: 117 retained price cases and four
additional 4/12/40/160-world comparisons matched prices, node counts,
policy-call counts and inner-world counts against the preserved rational
implementation (original binary `14939b7b…`; raw comparison
`/Users/jason/data/texas-42/kiln-v1/counted-parity.json`). The later
worker-speed studies are §9.

### §1.3 Where the survey stopped

`survey-frozen-summary.json` (`kiln-campaign-audit-v1`, `verified_structure:
true`, `complete: false`): **1,001,348 receipts; 851 settled deals** of 1,000
required; depth 8: 398,587; 40: 366,185; 160: 236,576; pending 69,430;
dispositions advance 598,832, audit-advance 3,946, screened-heuristic 161,994,
target-depth 236,576; preselected audit cells 9,420, at target depth 8,014;
of 3,299 preselected cells that either screen would have stopped, 78 reached
at least half and 0 reached the eligibility threshold (Wilson 95% upper bound
0.00116); 12 producer identities verified; audit elapsed 17.307551041 s;
partial book id `89717b76…`. `PROGRESS.md`: export
`/Users/jason/data/texas-42/kiln-v1/survey-book-partial.json` (14,420,032
bytes); "Old campaign stopped cleanly … Coordinator 69038 and sleep guard
69039 are gone." Its `note`, verbatim: "Sampling depth and model scores are
verified; calibration and actual player strength are separate evidence."

Checkpoints (all partial audits, no book id):

| checkpoint | commit | settled | receipts (8 / 40 / 160) | preselected cells a screen would have stopped: reached 75% / 50% | record |
|---|---|---|---|---|---|
| 100-deal milestone | `4bc60d64` | 101 | 152,680 | of 923 at depth 160, 376 would have stopped; **none** / 9 | `audit-100-deal-milestone.json` (campaign), `PROGRESS.md` |
| quarter | `be08f8ba` | 251 | 294,840 (117,534 / 107,909 / 69,397) | — | `PROGRESS.md` |
| one-third | `ec07af9a` | 335 | 394,287 (157,128 / 144,195 / 92,964) | of 1,317: **none** / 38 | [`one-third-audit-summary.json`](../experiments/kiln/one-third-audit-summary.json) |
| halfway | `619f222c` | 502 | 589,743 (234,992 / 215,711 / 139,040) | of 1,938: **none** / 55; 4,711 of 9,420 preselected at target depth | [`halfway-audit-summary.json`](../experiments/kiln/halfway-audit-summary.json) |
| frozen | — | 851 | 1,001,348 (398,587 / 366,185 / 236,576) | of 3,299: **none** / 78 | [`survey-frozen-summary.json`](../experiments/kiln/survey-frozen-summary.json) |

`PROGRESS.md` on the screening column: "These correlated cases remain
descriptive screening evidence, separate from actual-play calibration"; "This
supports retaining the current screen provisionally, not a proof of no misses";
the cell-wise Wilson intervals "are not calibrated campaign-wide bounds."

### §1.4 The calibration study: sixes, bid 36 (`CALIBRATION-SIX36.md`)

One hand, selected before any outcome was played: own hand
`[0,3,13,21,22,23,26]`, seat 3, sixes, bid 36 — "closest to .78 among six
initial .75-at-eight candidates re-evaluated at 160 worlds." Frozen model
forecast **121/160 = 75.625%**. Executed: **21 makes in 100 fresh hidden
completions**, Wilson 95% **14.17–29.98%** (`calibration-six36-summary.json`:
`made: 21`, `completed: 100`, `wilson95: [0.1417, 0.2998]`). "This score
substantially overestimated success against the executed table players. It is
not a calibrated probability of winning that match-up." "This is one hand, not
an aggregate calibration curve or a general estimate of player strength."
Execution: all four seats the deployed L1+partner player (profile
`deployed-l1-partner-40-8-opening160-budget14s-opening20s-v1`), each replanning
from its own hand and public record; every move checked against the
independent Python rules while running; a second audit replayed **1,808
decisions through Plunge's actual game engine**, all 100 games matching. "The
audit does not establish equivalence between modeled and executed continuation
policies." Case id `ecd47c3c…`; game-file digest `b69ab2b0…`; evidence under
`/Users/jason/data/texas-42/kiln-v1/calibration-six36/`.

Diagnostic follow-up (commit `466db3f1`, 2026-09-18, "Record calibration policy
and belief-update mismatches"): the saved games' own opening evaluations
averaged **76.7125%** for the move chosen (range 70–85.625%); tile 0 chosen 53
times (10 makes, mean model score 76.10%), tile 13 chosen 47 times (11 makes,
77.41%) — "the discrepancy is present across repeated forecasts for this hand,
not just the originally selected 121/160 receipt." All 100 opening evaluations
completed 160 worlds; all 1,195 non-opening evaluated decisions completed 40;
513 moves were forced; no budget overrun; partner review changed exactly one
move (game 39, ply 22): "Deadline truncation and frequent partner overrides
therefore do not describe this execution." Two structural mismatches confirmed
from source: (i) auction pricing uses `Field::SeatLevels([0; 4])` — modeled
level-0 policies against Dice on eight inner worlds — while actual seats used
the deployed L1 player and partner review; (ii) the forecast optimizes the
bidder's future choices over its original sampled worlds (`combine_buckets`
conditions them on policy-dependent moves) while actual play
(`partnership::evaluate`) samples anew at each decision "without carrying the
forecast's policy-conditioned posterior or contingent plan"
(`walt/SCENARIO-PLAYER.md` Remark 4.4 records the distinction). "The causal
contributions remain unmeasured … This inspection does not establish that any
one change repairs the gap." The isolation it proposes (held-out completions
with the deployed bidder against the same modeled field, then changed
partner/opponent policies, with a frozen bidder policy declared on unseen
histories) has not been run.

### §1.5 Survey-book game integration — implemented, never deployed

`PROGRESS.md`, "Game integration implemented, not deployed": Plunge's
`src/ai/kiln/book.ts` validates a versioned book and exposes an evaluator that
receives only own-hand/seat/bid/public-seed requests; `scripts/install-kiln.mjs`
imports a hash-addressed asset; the auction policy kept "the current
minimum-raise, partner-pass and 3/4 model-threshold policy … there is no new
aggressive maximum-bid rule"; a 133-deal local preview was installed
temporarily (three AI bids in about 3.8 s, later 3.15 s) and removed; "The book
manifest remains intentionally null." Repeated-hand reconciliation is Plunge
`cdf6bfe`. The release gate (`audit.py … --book`, all 1,000 deals settled) was
never reached: "The old 1,000-deal scalar-price production target is
superseded, not completed" (`PROGRESS.md`); "The previous scalar model-price
survey stays separate and is not consumed by Plunge"
(`PLAYED-BIDDER-RELEASE.md`); "No old scalar price is counted as a played game.
The observed calibration failure stays useful evidence" (`PLAYED.md`).

## §2 The actual-play campaign (`kiln-played-v1`)

### §2.1 The unit and the protocol (`PLAYED.md`)

"User direction, 2026-09-18: the unit is the player making bids. Its internal
forecasts are not the outcome being measured." For each fixed seven-domino
bidder hand and each of nine declarations, uniformly shuffle the other 21
dominoes into three seven-domino hands; all four seats play the deployed Walt
procedure targeting 30 through all 28 moves; record the declaring team's final
points 0–42. One score distribution per (hand, declaration) — a **panel** —
supplies every tail frequency 30–42 without another game per prospective bid.
Recommend the highest threshold reached in at least 4/5 of completed games.
"This is deliberately a practical bid-30-play heuristic, not a claim about a
policy retargeted to 36."

The player as Kiln used it: the shared deployed procedure
([`walt-player`](../walt/walt-player/README.md)) — opening 160 worlds / 20 s,
later decisions 40 worlds / 14 s with the existing partner review; "Post-contract
play uses that same unchanged player and its existing ties." The same hidden
completion and policy seed serve every declaration for a given own hand and
trial index (two independent seed labels); "The player sees only its own
original hand and public plays; the full deal exists only in the host."
Screening: at 8 games, at most one made 30; at 40, fewer than 20 made 30; a
preselected 2% of cells bypass; `init --no-screen` disables it. "Early
screening is explicitly heuristic"; "Screened cells keep their actual
histograms and sample counts, never an invented zero probability."
Durability: one complete game is the atomic durable unit; an exclusive writer
lock and SQLite WAL/FULL with macOS fullfsync; a stop loses at most the
unfinished game in each worker; failed jobs stop after three errors and stay
visible (`run --retry-failed`); completed games are never replaced; "Profile
changes require a new campaign; semantics-preserving host optimizations may
add a producer to the same campaign." The auditor replays every move with the
existing Python rules (`experiments/partnership/rules.py`).

### §2.2 The 100-hand campaign (2026-09-18, closed at `4cc4bd73`)

Scope: 100 bidder hands = the first 25 Plunge deals at seeds 420600–420624 ×
four seats; nine declarations → 900 panels; depths 8 → 40 → 160.

Throughput (`PLAYED.md`, "Throughput and verification, 2026-09-18"): first
60-second firing 382 games (10,696 independently checked moves), 6.35 games/s,
zero errors, overruns or fallbacks; opening choices 75.5% of player time; the
staged-cache reuse (completed inner-policy answers carried between the 40- and
160-world opening stages, released before partner review; "No policy or sample
budget changed") matched all choices, values, partner reviews and outcomes on
72 full-game replays (2,016 moves) at 1.147× aggregate / 1.143× median
(`played-stage-carry-benchmark.json`; `PROGRESS.md` reports 1.146× aggregate
for the final worker `91386e6e` on the second comparison — a different run,
not a disagreement). Pool: a fixed 72-game comparison measured 18 workers in
10.29 s, 12 in 14.60 s, 24 in 10.92 s, a repeated 18 in 11.42 s → 18
single-thread workers; "bounded Mac measurements, not Pixel timings or a claim
of a universal optimal pool size." Second firing 401 games at 6.67/s; 783
games / 21,924 moves audited (`played-startup-audit.json`); continuous
production from `170178e2` (coordinator 93902, sleep guard 93903, 18 workers);
live audit at 1,228 games / 34,384 moves (`played-live-audit.json`).

Completion (`played-completion.json`): **42,008 games / 1,176,224 audited
moves**, 900 panels; depths 311 at 8, 456 at 40, 133 at 160; three producers;
book id `5e26796c…`, export
`/Users/jason/data/texas-42/kiln-played-v1/played-book-final.json`; `made30:
18038`, `partner_changes: 831`, `fallback_moves: 0`, `over_budget_moves: 0`;
last run 6.7852 games/s over 6,075.701 s. Under the 4/5 cutoff,
`panel_recommendations` 30: 5, 31: 6, 36: 1, none: 888 (12 of 900 qualify);
`best_hand_recommendations` 30: 4, 31: 5, 36: 1, none: 90 (10 of 100 hands).
`PROGRESS.md`: "The other 90 do not clear this conservative empirical rule;
that does not establish that they are unbiddable under other risk thresholds
or policies. The outcomes are the bid-30 player's achieved scores, not
performance of retargeted higher bids." `interpretation`, verbatim: "Observed
score tails under bid30 play; 4/5 empirical recommendation, not a confidence
bound." History at completion: 693 recommendation changes, 254 after ≥ 8
observations, 111 after ≥ 40 — "movements of an estimate, not independent
strategic discoveries or causal explanations." Completion backup
`backups/20260918T154209.756236Z`, HF commit `77ae730a…`
(`completion-backup-evidence.json`, database at 42,008 games, integrity ok).
"At the 100-hand completion the monitor was PAUSED."

### §2.3 The 500-hand expansion (2026-09-19, `40be1356` → `5ab08bbc`)

`PLAYED.md`: append 400 bidder hands for **500 total** from 125 complete deals
at seeds 420600–420724, all nine declarations → **4,500 panels**; the 42,008
original games and the 100-hand export intact. "This range now includes
earlier research exam source seeds; they are consumed research data, not an
untouched future confirmation set."

Adaptive allocation: after the 8/40 screens, panels reach 160; at 160 and 320,
compare each score tail 30–42 with the 4/5 cutoff using Wilson intervals with
z = 1.96; if any interval contains 0.8, add the next stage, up to 640; check
only at stage boundaries; the preselected 2% audit cells reach 640 regardless;
at the cap, retain an explicit **`capped-unsettled`** label if ambiguity
remains. Verbatim: "These intervals allocate compute; they are **not
simultaneous or anytime confidence guarantees**, and adaptive screening does
not certify bid reliability. The underlying empirical bid rule, deployed player
and uniform completion sampler are unchanged." (`extension-500-first-eight.json`
`sampling_allocation`: interval "Wilson z=1.96, heuristic allocation only",
checkpoints `[160, 320, 640]`, threshold `[4, 5]`.) "Completion means the
planned allocation finished, not that every panel reached 640 or every bid was
statistically resolved." `extend` acquires the writer lock, appends hands and
cells, updates the manifest atomically, saves the previous manifest and every
original receipt hash under `extensions/`, and is idempotent. Python 3.12 is
required: "the system SQLite runtime has previously failed to read this
database's WAL correctly."

Launch and milestones (`PROGRESS.md`; `extension-500-startup.json`;
`extension-500-audit.json`; `extension-500-first-eight.json`): from the
100-hand data 17 uncertain panels and 10 audit controls were eligible for more
games, 106 more panels resolve at 160, 767 retain their screens; the bounded
60 s firing saved 399 games (6.62/s), no errors; coordinator 8878, sleep guard
8879, 18 workers, binary `91386e6e`, source `40be1356`; a live snapshot
replayed 42,407 games / 1,187,396 moves across four producers and 500 hands.
Eight-game coverage (`a25e4b0c`): all 4,500 panels at ≥ 8 games; 74,227 games
(history report 74,229; 2,539 recommendation changes, 339 after ≥ 8, 111 after
≥ 40); about 6.5 games/s; HF snapshot `20260919T072400.201412Z`, commit
`88627e9f…`.

Final state (`PLAYED-BIDDER-RELEASE.md`, "Frozen evidence";
`played-bidder-release.json`):

| field | value |
|---|---|
| games / independently replayed moves | **305,440 / 8,552,320** |
| hands / deals / panels | 500 / 125 / 4,500 |
| original receipt hashes preserved | 42,008; `integrity: ok` |
| depth counts | 1,575 panels at 8; 2,189 at 40; 541 at 160; 19 at 320; 176 at 640 |
| allocation states | 3,764 screened; 582 resolved; 79 audit-complete; **75 capped-unsettled** |
| 4/5 cutoff qualifiers | 80 panels, 61 bidder hands, 52 deals |
| producers / audit elapsed | 4 / 211.431 s |
| source book id | `77cb49c7a8d7f8b4548cf982d97f424d062ed1a88f9091bbb0ba0484528f2bc2` |
| export | `/Users/jason/data/texas-42/kiln-played-v1/played-book-500-final.json` (sha256 `e3629558…`); `played-book-final.json` unchanged |
| release evidence | `kiln-played-v1/releases/plunge-c7a1215/` |

"75 capped-unsettled panels retain that label" (`PLAYED.md`); "Capped
uncertainty is retained, not relabeled as confidence"
(`PLAYED-BIDDER-RELEASE.md`); "No failed games, fallback moves or budget
overruns. The producer completed normally; the monitor remains stopped at
Jason's request" (`PROGRESS.md`).

## §3 The empirical bidder release in Plunge (2026-09-19)

`PLAYED-BIDDER-RELEASE.md`, commit `5ab08bbc`; Plunge main
`c7a1215d4d1ece0e24e13fef693ed17f55475112`;
`https://plunge.jasonyandell.workers.dev/`. New matches and subsequent hands
deal from a shuffled 125-deal catalogue with 500 recorded bidder hands; every
covered auction is a table lookup; "No live declaration-pricing workers run."

Table behavior, verbatim: "Walt selects the highest legal 30–42 target
clearing the 4/5 frequency cutoff, then the declaration with the best recorded
tail at that target. It passes over partner's standing bid. A forced 30 uses
the best available panel even when below cutoff. At a qualifying 42, select
the cheapest legal plain-marks bid. Human bids, marks, shaker rotation, shared
replays and ordinary Walt play remain unchanged." The lookup receives "only own
seven tiles and seat, plus the public auction target; it does not inspect the
catalogue's actual opposing hands." Auction records carry an explicit empirical
schema, source book/profile, full nine-panel counts and allocation states.
Rotation: "Each match visits all 125 deals before repeating. Weak deals are
retained; all-pass reshakes remain normal. The 80% cutoff is selective." A
saved non-catalogue hand finishes with the existing fallback (live pricing,
`PLUNGE.md`); its next hand enters the catalogue.

Verification (`played-bidder-release.json`): typecheck; **201 tests in 25
files**; production build and Cloudflare dry run; `wasm_unchanged: true`;
local Pixel-size Chrome completed a real hand, reshuffled, reloaded and
resumed with `auction_workers: 0`, no JavaScript errors, no horizontal
overflow, one full auction and declaration in 3,396 ms including presentation
pauses and automated human response; the hosted build likewise (3,441 ms;
`deployment: success`, `ci: success`, workflow run `35488961822`); offline
reload and a fresh empirical auction with the network disabled passed. Verbatim
caveat: "This is a Mac browser at Pixel viewport size, not a new Pixel hardware
benchmark." Unchanged: "The playing WASM and manifest are unchanged"; the old
survey "is not consumed by Plunge"; "The stopped monitor was not restarted, no
new games were generated." `PLUNGE.md` (2026-09-19): "The book's measurements
are bid30 partner-profile score tails, not a higher-bid or human-opponent
guarantee"; "Both playing profiles remain available."

## §4 Sample history, Atlas, Pattern Workshop

**Sample history** (`SAMPLE-HISTORY.md`, `history.py`, commit `ec308ba7`,
2026-09-18; "Exploratory instrument and research direction"). Reconstructs
every sample prefix in trial order, independent of worker completion order;
records each recommendation change with before/after histograms and score
tails, the added receipt, crossed thresholds, and hashes binding both prefixes
to their original receipts; content-addressed reports under the campaign's
`history/`; no production mutation, restart or extra solve. First retained
witness (`sample-history-evidence.json`): cell 351, deal seed 420609, seat 2,
no trump, own tiles `[2,10,17,18,20,25,26]`; after 16 games 13 reached 35
(13/16 clears 4/5); game 17 scored 33, leaving 13/17 at 35 and 14/17 at 33, so
the recommendation fell from 35 to 33; the player targeted 30 throughout.
First snapshot: 12,056 games in 900 cells; 511 changes after the first
observation, 72 after ≥ 8 games, 0 after ≥ 40; `player_or_sampling_changed:
false`. "These counts describe threshold movement, not 511 independent
discoveries or confirmed strategic mechanisms." The page states the
importance-sampling identity a later world-proposal experiment would rest on
(`sum_w q(w)[p(w)/q(w)]f(w) = sum_w p(w)f(w)`; `choose(21,7)*choose(14,7) =
399072960` labeled opening completions; "a Monte Carlo target description, not
a theorem about finite PRNG frequencies") and separates three future
experiments — allocate empirical trials; change the world proposal; change
Walt's internal sampling — "not a sampler change now."

**Sunshine Atlas** (`ATLAS.md`, `ATLAS-v1.json`; commit `4e86a04f`,
2026-09-19; `https://sunshine-atlas.psychometrix.chatgpt.site`, owner-private;
source `/Users/jason/code/texas-42-sunshine-atlas` at `d47eda25…`; snapshot
`d75e8160…`). A static phone-friendly viewer over the played corpus: 4,500
panels; a two-dimensional event map; six descriptive NMF components (relative
reconstruction error 0.3614; two PCA axes retain 43.7% of variance; split-half
component cosines 0.9937–0.9993 — "descriptive stability, not six discovered
threat types or predictive validation"); count holder vs captor and all 84
tile/hidden-seat associations with 400 source-deal bootstrap replicates
("not effects of moving a tile between seats. Eight trials per panel remain
noisy"); 240 retained positions from the contrast corpus (2,304 worlds, 9,200
branches). The broad map uses exactly trials 0–7 for all 500 hands × nine
declarations, 36,000 games equally weighted by panel, avoiding adaptive depth
as a population weight; the source transaction held 271,321 games ("not a
claim that production has finished"). Verified: 36,000 source receipt hashes
and replays, 2,304 contrast receipts, 9,200 branch replays; hosted Chrome at
1440×1100 and 412×915, no JavaScript errors, no overflow;
`raw_campaign_unchanged: true`, `player_unchanged: true`. "Historical exam
sources now overlap the expanded campaign, so they are not untouched future
validation."

**Pattern Workshop** (`BICLUSTERS.md`, `BICLUSTERS-v1.json`; commit
`4cd59ef7`, 2026-09-19; `/biclusters/` in Site version 3, owner-private;
evidence `/Users/jason/data/texas-42/sunshine-biclusters-v1/`). Spectral
biclustering (6×6) of whole bidder hands under a declaration against hidden
holdings — all 84 tile/seat conditions and all 3,402 two-tile conjunctions —
scored as within-panel make-rate differences, shrunk and baseline-subtracted;
fit and check on disjoint trial windows ("regularized association scores, not
changes in a player's win rate"). Broad pilot: the Atlas's 36,000 games, 4,500
panels, trials 0–3 fit / 4–7 check; its strongest block falls from +5.719 to
−0.006 association points (the JSON stores the same values as fractions,
`discovery 0.057188`, `check -6.1e-05`); 16 of 24 shuffled refitted controls
exceed its checking score. Deeper study: 736 panels already at 160 games (all
79 preselected audits included), fit trials 40–99, check 100–159, 88,320 games,
733 panels vary in discovery; strongest block "B × 3": 143 panels from 85
source deals, 86 holdings, discovery/check +5.962/+5.587, bootstrap checking
interval [+3.738, +7.713], none of 24 shuffled controls reaches its checking
score, 32/36 blocks retain direction; the leading pair is partner holding both
5–5 and 6–4; 56 fours declarations in the row family; bidder double-four 48.95%
vs 34.11% across fitted panels — "a recognizable count/trump association to
inspect, not a proved tactic." "This selected cohort is not the full hand
population, and the two studies do not isolate a sample-size effect." "The
study does not yet assign new hands to families or establish a lawful threat
detector, causal swap effect, sample saving, or player improvement."

## §5 Commands and where things live

From the repository root; data under `/Users/jason/data/texas-42/`.

Actual-play campaign (`PLAYED.md`; subcommands `init`, `extend`, `run`,
`status`, `audit`, `export` per `played.py`):

```sh
cargo build --release --locked --manifest-path walt/Cargo.toml -p walt-player --bin kiln-play-worker
python3 experiments/kiln/played.py init /Users/jason/data/texas-42/kiln-played-v1 --hands 100
python3 experiments/kiln/played.py run /Users/jason/data/texas-42/kiln-played-v1 --workers 18 --seconds 60 --games 8
python3 experiments/kiln/played.py status /Users/jason/data/texas-42/kiln-played-v1
python3 experiments/kiln/played.py audit /Users/jason/data/texas-42/kiln-played-v1
python3 experiments/kiln/played.py export /Users/jason/data/texas-42/kiln-played-v1 /Users/jason/data/texas-42/kiln-played-v1/book.json
# 500-hand adaptive campaign (Python 3.12):
python3.12 experiments/kiln/played.py extend /Users/jason/data/texas-42/kiln-played-v1 --hands 500 --refine-games 640
python3.12 experiments/kiln/played.py run /Users/jason/data/texas-42/kiln-played-v1 --workers 18 --seconds 60 --games 160 --binary /Users/jason/data/texas-42/kiln-played-v1/producers/5a746760…/kiln-play-worker
```

`--seconds 0` runs unattended; Ctrl-C/SIGTERM stops cleanly and the same
command resumes; prefix `caffeinate -i` on a Mac; "`status.json` is a
replaceable live view, the database is authoritative"; "An explicit stop is not
a request for a monitor to restart it."

Sample history (`SAMPLE-HISTORY.md`):

```sh
python3 experiments/kiln/history.py report /Users/jason/data/texas-42/kiln-played-v1
python3 experiments/kiln/history.py explain /Users/jason/data/texas-42/kiln-played-v1 --cell 351 --n 17 --against-trial 0
```

Frozen survey (`README.md`; historical — `PROGRESS.md`: "Do not restart the
old campaign automatically"):

```sh
cargo build --release --locked --manifest-path walt/Cargo.toml -p walt-player --bin kiln-worker
python3 experiments/kiln/kiln.py status /Users/jason/data/texas-42/kiln-v1
python3 experiments/kiln/audit.py /Users/jason/data/texas-42/kiln-v1 /Users/jason/data/texas-42/kiln-v1/audit-partial.json --partial
```

Backups (`PLAYED.md`, `backup.py`): consistent SQLite online-backup snapshots
while production continues, receipts and immutable producer/source bundles
included, every archive member read back and hashed; local under
`~/data/texas-42/backups/`; optional upload to the private HF dataset,
verified by downloading and hashing at the saved commit.

```sh
python3 experiments/kiln/backup.py --upload
python3 experiments/kiln/backup.py --campaign played --upload
python3 experiments/kiln/backup.py --existing /absolute/snapshot/path --upload
```

Recorded snapshots: `20260918T141408.625440Z` (both campaigns; HF commit
`f4a347df…`; `backup-evidence.json`: kiln-v1 at 1,001,348 results, played at
8,161 games / 228,508 moves audited from archived tools alone);
`20260918T142655.519133Z` (`d03d9286…`, 13,320 games);
`20260918T154209.756236Z` (`77ae730a…`, 100-hand completion);
`20260919T072400.201412Z` (`88627e9f…`, eight-game coverage, 74,398 games).

Tests: `python3 -m unittest discover -s experiments/kiln -v` (`README.md`; the
Plunge checkout defaults to `/Users/jason/code/plunge-sunshine`). Cross-engine
calibration audit, in the Plunge checkout:
`KILN_CALIBRATION_DIR=/Users/jason/data/texas-42/kiln-v1/calibration-six36 npx vitest run tests/kiln-calibration-audit.test.ts`.

## §6 State as of 2026-09-20

- **Deployed:** the 500-hand empirical bidder, book `77cb49c7…`, Plunge
  `c7a1215d…`, live since 2026-09-19 (§3); playing WASM unchanged.
- **Stopped:** actual-play production and its monitor ("the monitor remains
  stopped at Jason's request. No more production was started for this
  release", `PROGRESS.md`). `PROGRESS.md`'s own heading reads "Production
  (revalidate; this file is not liveness evidence)"; `README.md`: "Check the
  live process before treating any old status snapshot as running."
- **Frozen:** the scalar-price survey at 1,001,348 receipts / 851 settled
  deals (§1.3); its book never installed; "superseded, not completed."
- **Preserved:** the 100-hand book `played-book-final.json`, every original
  receipt, the calibration case, the histories, the verified snapshots (§5).
- **Unchanged:** the deployed player's decision procedure, budgets, sample
  counts and partner review (every Kiln record repeats this).

`PROGRESS.md`, "Next actions" (the survey-era list; `PROGRESS.md` marks its
PIDs and restart suggestions "historical, superseded"): (1) revalidate PID
69038 / sleep guard 69039, database progress and git state; (2) keep the long
run producing all 1,000 deals and refinements; (3) audit the preselected 2%
deep cells, export the final book only when all 1,000 deals settle, audit with
`--book`, "Keep the calibration finding"; (4) install the final book in Plunge
with no preview flag; (5) merge/push and deploy Plunge, verify via browser
automation; (6) "Preserve Sunshine: quick lawful partner-aware play;
evidence-calibrated guesses, original receipts/questions feeding Scheme gym;
future belief/workshop work." (4)–(5) were done for the *played* book, not the
survey book; (1)–(3) are moot while the survey stays frozen. Named as future
and not started: the calibration-gap isolation (§1.4); the three separated
sampling experiments (§4); "which compact descriptions of these families
survive new hands and help an actual decision" (`BICLUSTERS.md`).

## §7 Cautions, verbatim, with sources

- "This is exploratory model pricing, not calibrated odds or a
  perfect-information oracle." / "Screening is heuristic and can miss viable
  bids; 0/8 is not impossibility." / "`status.json` and run logs are views, not
  authority." / "A partial audit never certifies completion." — `README.md`
- "Sampling depth and model scores are verified; calibration and actual player
  strength are separate evidence." — `survey-frozen-summary.json`,
  `halfway-audit-summary.json`, `one-third-audit-summary.json` (`note`)
- "This is a preserved model survey, not an empirical bid database or a
  complete 1,000-deal book." / "do not call an empirical threshold flip a causal
  strategic discovery." / "Production (revalidate; this file is not liveness
  evidence)" — `PROGRESS.md`
- "It is not a calibrated probability of winning that match-up." / "This is one
  hand, not an aggregate calibration curve or a general estimate of player
  strength." / "This policy mismatch is a candidate explanation, not a cause
  established by the experiment." / "This inspection does not establish that
  any one change repairs the gap." — `CALIBRATION-SIX36.md`
- "Played outcomes assess calibration against the declared executed policy,
  not an exact win probability." — `calibration-six36-summary.json` (`note`)
- "Implementation proof sketch and exploratory regression evidence; not a new
  corpus theorem or a claim of player strength." / "This change provides no
  permission to reuse node values across a different bundle, objective, field,
  or sampler profile." — `COUNTED-VALUES.md`
- "Exploratory empirical evidence." / "This is deliberately a practical
  bid-30-play heuristic, not a claim about a policy retargeted to 36." / "Early
  screening is explicitly heuristic" / "These intervals allocate compute; they
  are **not simultaneous or anytime confidence guarantees**, and adaptive
  screening does not certify bid reliability." / "Completion means the planned
  allocation finished, not that every panel reached 640 or every bid was
  statistically resolved." / "These are bounded Mac measurements, not Pixel
  timings or a claim of a universal optimal pool size." / "No old scalar price
  is counted as a played game." — `PLAYED.md`
- "Observed score tails under bid30 play; 4/5 empirical recommendation, not a
  confidence bound." — `played-completion.json` (`interpretation`)
- "Wilson z=1.96, heuristic allocation only" — `extension-500-first-eight.json`
  (`sampling_allocation.interval`)
- "The higher bids are this agreed empirical heuristic, not a measurement of
  retargeted policies or an 80% guarantee against humans. Capped uncertainty is
  retained, not relabeled as confidence." / "This is a Mac browser at Pixel
  viewport size, not a new Pixel hardware benchmark." —
  `PLAYED-BIDDER-RELEASE.md`
- "Empirical score tails from bid-30 play, not a guarantee for higher contracts
  or humans." — `played-bidder-release.json` (`remaining_limit`)
- "The book's measurements are bid30 partner-profile score tails, not a
  higher-bid or human-opponent guarantee." — `experiments/partnership/PLUNGE.md`
- "This delivers the bidding-speed objective; no partnership improvement is
  claimed." — `experiments/partnership/SUNSHINE-NOTES.md`
- "Exploratory instrument and research direction" / "These counts describe
  threshold movement, not 511 independent discoveries or confirmed strategic
  mechanisms." / "Contrasts are observations, not causal attributions." / "This
  is a Monte Carlo target description, not a theorem about finite PRNG
  frequencies." / "not a sampler change now" — `SAMPLE-HISTORY.md`
- "These counts are not a claim that production has finished." / "That is
  descriptive stability, not six discovered threat types or predictive
  validation." / "They are not effects of moving a tile between seats. Eight
  trials per panel remain noisy." / "These are sampled policy continuations,
  not optimal-play [necessary outer profiles] or demonstrated player
  improvements." (the source's own word at the bracket is the one D3 retires;
  see [vocabulary](vocabulary.md)) — `ATLAS.md`
- "These are regularized association scores, not changes in a player's win
  rate." / "This is a recognizable count/trump association to inspect, not a
  proved tactic." / "The study does not yet assign new hands to families or
  establish a lawful threat detector, causal swap effect, sample saving, or
  player improvement." — `BICLUSTERS.md`
- "Node WASM; not phone timing" / "Mac Node WASM timings, not Pixel timings." —
  `phone-build-summary.json` (`host`, `notes`)

## §8 Mining studies from played worlds (2026-09-18)

Four studies ran on 2026-09-18 over the games the actual-play campaign (§2)
had already recorded, all in `experiments/kiln/` and all EXPLORATORY: two mine
opening *outcomes* (which hidden-world ownership facts go with failing to make
30), two mine *decisions* (where a different move would have helped under the
same continuation). Every one of them emits its hypotheses as executable
Schemes in the `walt::scheme` language ([walt-scheme-fix](walt-scheme-fix.md);
the Viewer/World access split of its §5 is what "Viewer-only" and
"hidden-world query" mean below). The sources' phrase "threat Scheme" names an
outcome-associated ownership query over hidden worlds — an analysis and belief
instrument; every record repeats that such queries "are analyst/belief
instruments, never direct executable player guards" (`THREAT-PROBE.md`). None
of the four changed the player, the sampler, production scheduling or the
phone: "No player, proposal distribution, production queue, or bidding rule
changes" (`THREAT-PROBE.md`); "The actual phone player remains unchanged"
(`SUNSHINE-NOTES.md`). Order below is chronological by commit; the four
uncurated paragraphs on [walt-instruments](walt-instruments.md) (2026-09-18)
summarize the same records.

Common protocol shape, stated once: a protocol document is committed before
any held-out outcome is read; selection uses discovery data only; the selected
hypotheses are frozen in a content-identified `fit.json` (its own commit)
before fresh labels are opened; the fresh set is then consumed ("cannot be
recycled as untouched confirmation for another learned rule",
`decision-mining-v1/RESULTS.md`); every Scheme membership the Python matcher
computes is re-checked through the real Rust Scheme engine (the bin
`walt/walt/src/bin/scheme_worlds.rs`, commit `231eb1b0`) and publication is
refused on any mismatch; every game and branch is replayed by the independent
Python rules; raw artifacts live outside git under
`/Users/jason/data/texas-42/kiln-played-v1/<study>/` inside the campaign's
backup scope (§5), and the tracked directory holds compact evidence, the
frozen fit, the fresh result and the runnable `.scheme` queries.

### §8.1 Outcome-first Scheme discovery (`threat-probe-v1`)

Commit `231eb1b0` (2026-09-18, "Probe outcome-first Scheme discovery from
saved played worlds"): protocol [`THREAT-PROBE.md`](../experiments/kiln/THREAT-PROBE.md)
("Frozen protocol (before inspecting results)"), results
[`threat-probe-v1/RESULTS.md`](../experiments/kiln/threat-probe-v1/RESULTS.md),
evidence `evidence.json`, `posthoc-ablations.json`, the eight
`candidate-N.scheme` files and `posthoc-before-double-five.scheme`; runner
`threat_probe.py`, tests `test_threat_probe.py`. `SUNSHINE-NOTES.md` names the
direction: "observed outcomes → candidate Scheme → ablation/generalization →
matching new worlds → measured continuation outcomes."

**Question** (`THREAT-PROBE.md`): "whether ownership facts retained from
observed losing worlds can describe other worlds with elevated failure rates."

**Protocol.** Exactly trials 0–7 of all 900 cells of the 100-hand campaign —
7,200 games but **800 hidden completions**, "since declarations share the
completion and policy seed" — chosen to avoid the outcome-dependent 8/40
screening. The 25 source deal seeds sorted: discovery = the first 15 deals
(420600–420614, `evidence.json` `split`), trials 0–3, 2,160 games; same-hand
validation = their trials 4–7, 2,160 games; new-hand transfer = the other 10
deals (420615–420624), trials 0–7, 2,880 games; all four bidder hands of a deal
stay in one group. Target: failure to make 30 under the saved bid-30 player.
The learner takes 64 distinct losing discovery worlds by a fixed hash order,
describes each by its 21 hidden tile-ownership facts with chairs relative to
the bidder (`after`, `mate`, `before`), retains every one-, two- and
three-fact sub-conjunction ("bounded deletion generalization, not a library of
tactical patterns"; physical tile identities retained in this first grammar),
requires ≥ 36 matching games spanning ≥ 6 discovery deals, ranks by positive
squared within-cell failure excess divided by matching games (each cell
supplying its own baseline failure rate), keeps eight candidates with
coverage Jaccard ≥ 4/5 suppressed, and fixes the primary as the top discovery
rank before validation (the protocol says "The primary candidate is discovery
rank 1"; the results table numbers from 0 and names rank 0 primary — recorded
here as a numbering discrepancy between the two files, not resolved). Permutation check: 999 draws, trial outcome vectors shuffled
within bidder hand and jointly across declarations; one-sided p for the
primary, max-statistic adjusted values for all eight.

**Search and audit** (`evidence.json` `search`, `native_membership_audit`):
**57,008** distinct clauses searched, **5,250** eligible positive-association
clauses; the Rust Scheme engine agreed with the cached literal matcher on
**115,200** primary/seed memberships (16 queries × 7,200 world rows) and on
**129,600** post-hoc memberships, zero mismatches (`PROGRESS.md` sums these to
244,800); all 7,200 receipts re-passed full play/score/request validation; four
Python checks (cell-difficulty confounding, joint declaration permutation,
genuine seed-world ablation, held-out selection isolation) and one Rust
impossible-world rejection test. First end-to-end run 10.06 s on the Mac with
production running — "a single observed run, not a throughput benchmark."

**Result** (`RESULTS.md`, verbatim headline): "The discovery-and-ablation loop
works; transfer evidence is mixed." Values are percentage-point failure excess
over the matched mixture of each cell's baseline — "They are not win rates or
policy gains."

| discovery rank / retained ownership | discovery | new worlds, same hands | new hands | new-hand adjusted p |
|---|---:|---:|---:|---:|
| 0: after 4-4; mate 6-2 (**primary**) | +11.11 | +5.86 | +1.67 | .931 |
| 1: before 5-2 and 5-5 | +12.78 | +4.86 | +4.63 | .484 |
| 2: after 4-4; mate 2-1 and 6-2 | +16.67 | +1.39 | +15.28 | .390 |
| 3: after 1-1; mate 6-0; before 0-0 | +18.98 | +11.11 | +8.33 | .274 |
| 4: mate 6-2; before 5-5 | +11.27 | +7.32 | +6.61 | .052 |
| 5: mate 2-1; before 6-4 | +12.61 | +3.47 | +4.91 | .490 |
| 6: after 5-5 and 6-3; mate 2-0 | +15.97 | no matches | +3.61 | .898 |
| 7: mate 2-1; before 4-3 and 6-4 | +15.97 | +13.89 | +4.44 | .857 |

The primary's new-hand association is +1.67 points, unadjusted one-sided
permutation p = .376 (`evidence.json` 47/125): "It is not evidence of a
generally reliable learned threat." Rank 4 is "the interesting secondary
hypothesis": new-hand matches span 25 hidden completions, 225 games and all ten
reserved deals, 177 failures (78.67%) against a matched-cell baseline of
72.06%; same-hand 22 completions / 198 games, 80.30% against 72.98%;
familywise adjusted permutation values .004 (same-hand) and .052 (new-hand) —
"unresolved preliminary evidence, not a threshold crossing to promote or a
probability that the hypothesis is true."

**Post-hoc ablation** (`posthoc-ablations.json`; "descriptive, not another
held-out confirmation or a basis for replacing the frozen primary candidate"):
every immediate one-literal deletion of all eight candidates, 18 distinct
queries. For rank 4: the pair +11.27 / +7.32 / +6.61; `mate 6-2` alone
+4.39 / +2.46 / −0.61; `before 5-5` alone +4.96 / +4.12 / +5.78. On new hands
the simpler `before 5-5` query matches 75 completions / 675 games, 535 failures
(79.26%) against 73.48%. "This does **not** prove the pair adds nothing, nor
that possession of 5-5 caused these losses." "No one supplied 'opponent has
double-five' as a tactical rule … Humans still chose the primitive vocabulary,
outcome target, and search bounds; this is bounded discovery, not
assumption-free learning."

**Preserved.** Frozen rows, compressed receipts, producers, fitted hypotheses,
seed and learned Schemes, result identities and a source/binary snapshot at
`/Users/jason/data/texas-42/kiln-played-v1/threat-probe-v1/` (~35.5 MB);
primary result id `e7afedea…`, post-hoc `a2361db7…`; the seed descriptions and
removed facts per candidate in `evidence.json`. "Completed primary results are
not overwritten. New production games cannot alter this first-eight protocol."

```sh
cargo build --release --manifest-path walt/Cargo.toml -p walt --bin scheme_worlds
python3.12 experiments/kiln/threat_probe.py /Users/jason/data/texas-42/kiln-played-v1 --output /path/to/new-probe
python3.12 experiments/kiln/threat_probe.py /Users/jason/data/texas-42/kiln-played-v1 --output /path/to/new-probe --ablate
```

Its stated next question — freeze the simpler candidates and test them on
additional worlds without retuning; test relational lifting against
literal-only ablation on a new split — is §8.2.

### §8.2 Fresh relational mining (`mining-v2`)

Commits `bd5b248b` (2026-09-18, "Freeze fresh Scheme mining replication and
estimator probes": [`MINING-PROBE-V2.md`](../experiments/kiln/MINING-PROBE-V2.md),
[`MINING-PROBE-V2-CV.md`](../experiments/kiln/MINING-PROBE-V2-CV.md),
[`MINING-V2-AMENDMENTS.md`](../experiments/kiln/MINING-V2-AMENDMENTS.md),
`scheme_mine.py`, `scheme_mine_cv.py`, their tests) and `ddb3cadc` ("Record
replicated threat Schemes and lower-variance bid measurements":
[`mining-v2/RESULTS.md`](../experiments/kiln/mining-v2/RESULTS.md),
`result.json`, `cv-result.json`, `fit.json`, `catalog.json`, the thirteen
`literal-*.scheme` / `relational-*.scheme` files). `PROGRESS.md`: "User
authorized further probes toward a reusable mining rig."

**Question** (`MINING-PROBE-V2.md`, two probes, one fresh evaluation): (1)
replicate the eight v1 literal hypotheses and the post-hoc `before holds 5-5`
hypothesis on new hands and worlds, the latter being the literal primary; (2)
derive *relational* descriptions from the same losing discovery worlds by
"replacing a physical hidden tile by an existential role with mechanical
properties and optionally a beating relation to an own-hand lead", the top
discovery-ranked relational query being the second primary, with four diverse
relational candidates retained.

**Protocol and freezes.** Discovery stays v1's 15 deals / trials 0–3 (2,160
games); the rest of v1 is "now development evidence, not fresh confirmation."
Grammar v2.1: holder = one of three chairs relative to the bidder; zero, one or
two true properties from `double(d)`/¬, `count(d,0|5|10)`, `in(d,q*)`/¬
(called membership), `boss(d,q*)`/¬ (top live called follower); optionally an
observed `beats(d,own,led)` with `own-legal(own)` and `leads-context(own,led)`
and at most one property of the own tile — "a static beating relation, not a
claim the hidden holder can legally play d at that future turn or ultimately
capture the trick." One hidden role, at most one own role, single
conjunctions, no tile literals, no negated existentials, no history, scores or
outcome labels; "Some property combinations may still identify one physical
tile; report that honestly rather than calling every query a broad
abstraction." Same 64 losing worlds; ≥ 36 matching discovery games, ≥ 6 deals;
v1's ranking; four candidates at pairwise Jaccard < 4/5. Generation produced
**411** distinct relational descriptions, **223** eligible (`fit.json`
`eligible`); the two top candidates describe an opponent holding the live top
trump, the other two an opponent holding a double that beats an own-hand
trump lead. Fresh corpus: original deal seeds **420625–420634**, four bidder
hands each, all nine declarations, trials 0–7 — **2,880 games / 320 hidden
completions**; the same deployed bid-30 profile and pinned worker binary
`91386e6e…` as production; all seven tricks; "No screening,
conditional-world selection, or optional stopping"; a nested campaign at
`kiln-played-v1/mining-v2/fresh/` with its own durable queue, run while the
bidding campaign continued; two resumable firings, 907.13 s combined; no
overruns, fallbacks or worker errors. Gate, fixed in advance: excess ≥ 3
percentage points, ≥ 20 matching hidden completions, ≥ 5 source deals,
one-sided permutation p ≤ .025 (Bonferroni for two primaries), 1,999
fixed-seed draws; max-statistic adjusted values for all 13 queries as a
secondary audit. Freeze lineage (`MINING-V2-AMENDMENTS.md`): the first fit
`01cc319c…` (`analysis/fit.json`) counted role declarations in its
atom-complexity metadata; the corrected fit `173bd310…`
(`analysis-v2/fit.json`, the tracked `mining-v2/fit.json`) counts predicate
atoms (6, 6, 10, 10); "The primary and four selected relational queries did not
change; all 13 evaluated query sources and their order were compared and were
identical"; both fits preserved; "No fresh per-game outcomes or associations
had been inspected." The pre-label extension `MINING-PROBE-V2-CV.md` was added
after the fit and before any fresh analysis: for each primary the event is
possession of one identifiable tile by one hidden chair, with exact prevalence
p = 7/21 = 1/3 when the tile is unseen and 0 when the viewer holds it or the
declaration has no called boss; one coefficient β per primary fitted from
discovery data only (literal double-five 50/549, top trump 56/311) and the
corrected observation Z = Y − β(X − p), whose mean equals E[Y] for any fixed
β; no clipping; "This tests **measurement efficiency for the fixed played-game
bidder**."

**Result** (`mining-v2/RESULTS.md`; `result.json` `replication_gate_passed:
true`, `interpretation`: "Fresh fixed-size outcome association audit. No causal
or player-strength inference."). Values are failure-rate excess above the
matched mixture of cell baselines — "They are **not player gains**."

| query | matching games | matching completions | failure rate | cell baseline | excess |
|---|---:|---:|---:|---:|---:|
| **primary: before holds 5-5** (`literal-double-five`) | 747 | 83 | 74.70% | 70.05% | **+4.65 pp** |
| **primary: before holds top trump** (`relational-0`) | 653 | 294 | 83.92% | 73.79% | **+10.13 pp** |
| secondary: after holds top trump (`relational-1`) | 607 | 284 | 83.03% | 73.95% | +9.08 pp |
| secondary: before holds double beating own trump lead (`relational-2`) | 687 | 291 | 80.20% | 72.33% | +7.88 pp |
| secondary: after holds double beating own trump lead (`relational-3`) | 642 | 281 | 80.06% | 72.66% | +7.40 pp |

Both primaries matched all ten source-deal groups and passed the gate; both
one-sided permutation values were 1/2000, "the minimum resolution of this
1,999-draw check. These are not zero probabilities or probabilities that the
hypotheses are true." 13-query max-statistic values: .0125 (double-five),
.0005 (top trump). "Matching-completion counts for relational queries count a
completion once if it matches under any declaration; 653 games are not 653
independent worlds." Non-replications, retained in `result.json`: the old
strongest training candidate `after 4-4 AND mate 6-2` (`literal-0`) had
**−0.48 pp** excess on the fresh corpus; the v1 secondary `mate 6-2 AND before
5-5` (`literal-4`) had +5.93 pp but "was not a primary here and its 13-query
adjusted p=.214"; another literal (`literal-5`) had adjusted p = .0275 "but only
15 matching completions, below the registered coverage gate"; "none were
removed or silently promoted." `catalog.json` gives every query an explicit
status: the two primaries `replicated-outcome-association`, the other eleven
`unconfirmed-outcome-association`; "all queries remain **analysis-only**."

Measurement efficiency (`cv-result.json`; `interpretation`: "Fixed-player
estimator variance, not stronger/faster play. Ten-group bootstrap is
descriptive."): average within-cell sample variance of Z against Y over all
360 fresh cells — double-five correction **0.53%** reduction (descriptive 95%
interval −0.13% to 1.14%, inconclusive); learned top-trump correction
**2.34%** (1.66% to 3.06%); on the 240 cells where top-trump ownership is
uncertain, 3.70% (2.64%–4.84%). "prediction alone does not guarantee a useful
shortcut." "Lower variance suggests modest potential sample savings for this
fixed-player measurement; it is not a measured wall-time gain, a new
bid-selection rule, or a speedup of Walt's internal joint-sample planning."

**Verification and preservation.** The Scheme engine checked **1,012,464**
memberships with zero mismatches (`PROGRESS.md`: 946,944 full-library and
28,080 selected-training; `result.json` `native_audit`: 37,440 on the 13 fresh
queries × 2,880 rows); all **80,640** fresh plays passed the independent
actor-only request, legality, scoring, receipt-hash and producer audit; eight
focused tests. Full games, models, both fits, snapshots, audits and logs at
`/Users/jason/data/texas-42/kiln-played-v1/mining-v2/`; fresh result id
`52e8db03…`, efficiency result id `8bfa6571…`; also in the HF snapshot
`kiln/snapshots/20260918T154209.756236Z`, commit `77ae730a…` — the same
snapshot as the 100-hand completion (§2.2, `completion-backup-evidence.json`).
"V2 deliberately checks its fixed protocol bounds and refuses a different
dataset."

```sh
# Python 3.10+; use a NEW output directory for a changed protocol.
python3.12 experiments/kiln/scheme_mine.py fit V1_CORPUS --output ANALYSIS
python3.12 experiments/kiln/scheme_mine_cv.py fit --corpus V1_CORPUS --output ANALYSIS
# Only after the complete fixed fresh batch exists:
python3.12 experiments/kiln/scheme_mine.py evaluate FRESH_CAMPAIGN --output ANALYSIS
python3.12 experiments/kiln/scheme_mine_cv.py evaluate --output ANALYSIS
python3.12 experiments/kiln/scheme_mine.py publish ANALYSIS --output CATALOG
```

The record's own reading: "This supports continued investment in an
outcome-first Scheme mining rig. It does not yet establish stronger partnership
play or faster Walt-internal planning." Its next step — "mine **decisions with
consequences**" — is §8.3.

### §8.3 Paired decision mining (`decision-mining-v1`)

Commits `3545a3fe` (2026-09-18, "Freeze a bounded decision-mining protocol and
shared-player examiner": [`DECISION-MINING-V1.md`](../experiments/kiln/DECISION-MINING-V1.md)
and the native examiner `walt/walt-player/src/bin/kiln-decision-worker.rs`,
which "wraps the unchanged shared player"), `0eeafcb3` (`decision_mine.py`,
resumable batches and independent census audits, `test_decision_mine.py`),
`0a8e2b6a` ("Freeze decision-mining selection before fresh action comparisons":
`fit.json`), `5a8dd401` ("Record paired decision-mining results, misses and
negative policy search": [`decision-mining-v1/RESULTS.md`](../experiments/kiln/decision-mining-v1/RESULTS.md),
`summary.json`, `result.json`, `train-misses.json`, `validation-misses.json`,
`resumption-audit.json`, the audits, 45 `queries/action-N.scheme` and
`queries/relational-{0,1}.scheme`, `decision_mine_publish.py`), `57196c4e`
(`backup-evidence.json`).

**Question** (`DECISION-MINING-V1.md`): "Do the previously learned
opponent-top-trump Schemes identify decisions where the shared deployed Walt
misses a better play? Can a small, lawful action description learned from
several such decisions transfer to new deals?" "This is a bounded next Sunshine
cycle, not a change to the phone player."

**Protocol.** Development: source deals 420600–420609, trial 0, all four bidder
hands and nine declarations (360 games from the completed campaign). Fresh:
new deals 420635–420644, trial 0, the same 360-game design and pinned
full-game producer, generated separately ("no earlier mining run has used
them"). Roots: declaring-team decisions at plies 20–23 (two dominoes in hand),
two legal moves, an unsettled bid-30 contract; at most 12 per source deal by a
fixed hash of the coordinate, "without outcome, original choice or threat
filtering" — 120 per panel from 319 (development) / 304 (fresh) eligible
coordinates. Each root is assessed over its entire uniform mechanically
compatible support (at most 90 worlds; "a declared prior, not a
behavior-conditioned posterior"), under two policy seeds (the original and an
independently labelled hash) — **240 comparisons per panel, "not 240
independent positions."** One root action is forced; then all four seats
finish with the actual shared `walt_player::decide`, 40 worlds, partner
review, 14 s; identical calls within an assessment reuse their first complete
response; recorded hidden hands and the source's terminal outcome never enter
the player or the learner. Description and learning: the frozen `relational-0`
/ `relational-1` Schemes (§8.2) re-evaluated at the current position as the
presence of the *live* top trump with either opponent, integrated over the
support ("an explicit temporal extension of the opening association, whose
usefulness here remains a hypothesis"); an action grammar of nine mechanical
properties — called/not, double/not, count 0/5/10, boss/not boss in the
context the action would lead — as witnessed single atoms and pairs, Viewer-only
Schemes with no tile identities, "No handcrafted score bonuses, attention model
or count-offer override"; gates always / max opponent-boss presence ≥ 1/3 /
≥ 2/3; a candidate changes the baseline only when its Scheme selects exactly
one legal action; selection requires ≥ 8 changed coordinates over ≥ 3 deals,
positive mean gain under each seed and in ≥ 3 deals, else the empty baseline
is frozen. "This is supervised policy-fragment search; its training gains are
selected and optimistic."

**Result** (`RESULTS.md` headline): "A working contrast rig, thirteen positions
to study, no new move rule." Values are success probabilities "under the
uniform mechanical support and these recorded ordinary continuations."

| measurement | development | fresh |
|---|---:|---:|
| position/seed comparisons | 240 | 240 |
| both moves have different make probabilities | 116 | 89 |
| Walt chose the lower-valued move | **7** | **10** |
| distinct positions with a miss under either seed | **6** | **7** |
| mean baseline regret, all comparisons | 0.185 pp | 0.228 pp |

(`summary.json`: `strict_assessments` 116 / 89, `misses` 7 / 10,
`missed_roots` 6 / 7, `mean_regret` 1/540 and 2801/1231200.) "Walt was best or
tied in 230/240 fresh comparisons. There are useful exceptions, but little
average room for an indiscriminate two-tile override in this panel." All **17
missed position/seed cases across 13 distinct positions** are published with
paired witnesses (`train-misses.json`, `validation-misses.json`): the whole
information set, both action values, the original model response, a world
where the alternative helps and one where it harms whenever one exists. The
largest fresh gap: deal 420639, policy-seed replicate 1, blanks trump, bidder
leads from 5-0 / 6-4, Walt chose 5-0 — root model 3/10 = 30% for 5-0 vs 11/40
= 27.5% for 6-4; full-support deployed continuations 4/19 = 21.05% vs 6/19 =
31.58%; the change helped in three worlds, harmed in one, tied in fifteen, net
2/19 = 10.53 percentage points "under this particular seed/continuation
contract"; "A favorable actual world alone would have hidden the harmful
counterexample. This is a saved exercise, not a general instruction to lead
6-4 or to withhold trump."

Negative policy search: **114** witnessed query/gate candidates; "Every
candidate's mean development gain was nonpositive"; "None met the frozen
support and improvement criteria." The selected policy is the baseline
(`result.json` `score`: `changed_roots` 0, `helped` 0, `harmed` 0,
`mean_gain` 0) — "Its zero gain and zero interval are not evidence for a new
skill. The limited grammar lacks richer partner/order/continuation
relationships; failure here does not establish that such relationships cannot
be learned."

Detector diagnostic (misses / assessments by the max of the two opponents'
top-trump presences):

| gate | development | fresh |
|---|---:|---:|
| below 1/3 | 5/148 | 2/112 |
| at least 1/3 | 2/92 | 8/128 |
| at least 2/3 | 0/18 | 0/26 |

"The moderate gate enriched misses on fresh hands but not development. The
strong gate found none in either panel. This does **not** establish a reliable
'think harder here' detector. A condition associated with losing can describe
an unfavorable situation without identifying a fixable choice." The
sample-efficiency diagnostic: partitioning worlds by whether either opponent
holds the live top trump explained **10.27% / 9.87%** of the variation in the
difference between the two actions' outcomes (`summary.json`
`explained_fraction` 11362166/110624823 and 21941912617/222410185495;
`interpretation`: "Retrospective variance decomposition, not held-out sample
savings or an allocation policy."). "They justify a later controlled estimator
experiment, not a claim that Walt can now use 10% fewer samples."

**Engineering, audit, preservation.** New source games 360 in 49.17 s (18
workers); each paired panel 13.82 s (10 workers) — "separate stage timings …
not phone measurements"; **21,620** complete forced-action continuations,
**144,308** subsequent plays, **107,601** recorded unique-per-assessment
decisions, **508,070** Scheme membership checks (`summary.json` `query_checks`
255,680 + 252,390); 10,080 moves audited in the new source games; no worker
errors, over-budget decisions or fallbacks (`fallbacks: 0`, `over_budget: 0`
in both panels). Seven Python tests and one Rust rejection test. A deliberate
one-second stop saved 15 assessments with 85 pending; resumption finished the
85 and preserved all 155 completed receipt hashes (`resumption-audit.json`,
"not counted as additional research evidence"). Recomputing the frozen fit and
fresh result after a reader hardening "returned the exact same content IDs":
fit `83798ea0…`, fresh result `399511376…`. Raw data at
`/Users/jason/data/texas-42/kiln-played-v1/decision-mining-v1/` (`train/`,
`validation/`, `fresh/`, resumable `assessments.sqlite`); backed up to the HF
snapshot `kiln/snapshots/20260918T162757.222700Z`, commit `285444c2…`,
checksums and integrity checks passed (`backup-evidence.json`); the backup used
Python 3.12 after the system Python's SQLite refused read-only access — "no
source data was modified to work around that runtime difference."

```sh
python3.12 experiments/kiln/decision_mine.py prepare SOURCE OUTPUT --split train|fresh
python3.12 experiments/kiln/decision_mine.py run OUTPUT --workers 10 --seconds 60     # repeat unchanged to resume
python3.12 experiments/kiln/decision_mine.py fit TRAIN FIT.json
python3.12 experiments/kiln/decision_mine.py report FRESH FIT.json RESULT.json
python3.12 experiments/kiln/decision_mine.py audit OUTPUT
python3.12 experiments/kiln/decision_mine_publish.py CAMPAIGN DESTINATION
```

"This v1 recipe deliberately fixes its seeds, domain and grammar. A different
question needs a new protocol/output, not overwritten evidence." "The present
experiment supplies no reason to deploy one of its simple move overrides."
`SUNSHINE-NOTES.md`: "Distinguish learning difficult situations from learning
where a different action helps."

### §8.4 Whole-game contrast mining (`whole-game-contrasts-v1`)

Commits `313a12dd` (2026-09-18, "Define whole-game sampled contrasts and
preserve action/world witnesses": [`WHOLE-GAME-CONTRASTS-V1.md`](../experiments/kiln/WHOLE-GAME-CONTRASTS-V1.md),
the native `walt/walt-player/src/bin/kiln-scheme-contrast-worker.rs`, changes
to `kiln-decision-worker.rs`), `6fbb8294` (`whole_game.py`, resumable sampling
and independent support-count audits), `b8172ef9` (`whole_game_mine.py`,
"Derive shared action-relative Schemes from observed world contrasts"),
`86792586` ("Freeze witnessed whole-game Scheme candidates before fresh action
outcomes": `fit.json`), `d534f2ba` ("Publish whole-game contrasts, fresh
nonreplication, and preserved sample reversals":
[`whole-game-contrasts-v1/RESULTS.md`](../experiments/kiln/whole-game-contrasts-v1/RESULTS.md),
`summary.json`, `result.json`, `refinement.json`, `witnesses.json`, the audits,
`queries/contrast-{36,110,150}.scheme`, `whole_game_publish.py`), `95e90444`
(`backup-evidence.json`).

**Question** (`WHOLE-GAME-CONTRASTS-V1.md`): "The two-tile experiment is a
verified instrument, not our strategy-discovery domain. This cycle samples
every meaningful hand size, seven through two." "We have concrete differences
already: worlds, changed holdings, forced moves and complete resulting
trajectories. The unknown is a reusable relationship explaining those
differences."

**Protocol.** Development deals 420600–420609, fresh 420645–420654, each trial
0 across four bidder hands and nine declarations (360 games); eligible roots =
declaring-team turns (bidder or partner), unsettled bid 30, ≥ 2 legal moves,
own remaining size 7, 6, 5, 4, 3 or 2; two coordinates per deal per size by a
fixed hash → 20 per size, 120 per panel, from 2,872 / 2,788 eligible roots.
Baseline = **the original saved deployed move and its receipt** (160-world
opening evaluation where applicable; "not chosen afresh by a cheaper model").
Per root, eight independent uniform mechanically compatible worlds drawn
**with replacement** by the existing integer capacity-DP sampler, each sample
index with its own seed (`summary.json` `compatible_worlds`: minimum 1,
maximum **399,072,960** — "without enumerating them"); every legal root action
forced in the same sampled world, then ordinary shared Walt at all four seats
(40 worlds, partner review, 14 s) with the original public policy seed held
fixed. One complete all-action sampled world is the atomic resumable job.
Miner: rows `(coordinate, alternative, world, D)` with D = made(alternative)
− made(original) ∈ {−1, 0, 1}; at most 12 contrast pairs per hand size of
worlds at the same coordinate and action pair with different D → 72 witnessed
pairs; every tile whose hidden holder changed between the pair is retained
("a nomination mechanism, not a claim that one of those tiles alone caused the
effect") and lifted into a shared vocabulary — holder relative to viewer
(after/partner/before), zero or one tile property (called/not, double/not,
count 0/5/10, live top trump/not), and whether the tile beats each root action
in the current led context — with `(alternative, baseline)` output roles and
no tile names or hand-size conditions. **220** Schemes generated, **212**
eligible (`fit.json` `eligible`); ranking on D centered within its exact
coordinate/action pair ("generally hard hands or obviously bad alternatives do
not by themselves earn a pattern"); ≥ 64 matching rows, six deals, three hand
sizes, variable membership in ≥ 12 pairs; up to three retained at Jaccard <
4/5; **three frozen at `86792586` before fresh alternative labels.** Fresh
gate: same direction, ≥ 5 pp signed residual per match, ≥ 5 deals, ≥ 3 sizes,
p ≤ .05/3 under a 1,999-draw permutation that moves sample indices jointly
across all alternatives of a root. Separately, up to four development roots
per size deepened from 8 to 24 worlds, chosen by apparent alternative gain then
discordance — "diagnostic refinement, not an independent confirmation set."

**Result** (`RESULTS.md` headline): "Whole-game witnesses, reversible sample
estimates, no accepted new rule." Fresh panel, eight worlds per root
(`summary.json` `validation.stages`):

| own dominoes remaining | roots | roots with a sampled make/set action flip | roots with an apparently better alternative mean |
|---|---:|---:|---:|
| 7 | 20 | 20 | 14 |
| 6 | 20 | 19 | 9 |
| 5 | 20 | 15 | 4 |
| 4 | 20 | 12 | 4 |
| 3 | 20 | 12 | 3 |
| 2 | 20 | 9 | 1 |

"The last column maximizes over noisy eight-world estimates and is therefore
optimistic. It is a source of study candidates, **not a measured mistake
rate**." Refinement (`refinement.json`; `summary.json` `refinement`): 24
development roots, four per size; all **960** original world receipts retained
their exact hashes (`preserved_initial_receipts: 960`); of 18 roots that
initially favored an alternative, **13 remained positive, two became tied,
three became negative** at 24 worlds; the empirical preferred move changed at
**8 of 24** roots. The published opening example: deal 420603, seat 2 bidding
twos, root `9a6f580b…`, Walt leads 2-2, alternative 3-2 — first 8 worlds help
3 / hurt 0 / tie 5, +3/8 = +37.5 pp; the next 16 alone 0 / 4 / 12, −4/16 =
−25 pp; all 24: 3 / 4 / 17, −1/24 = −4.17 pp; the four harmful additions are
sample indices 11, 18, 22, 23, and at index 11 the original lead makes 34
while the alternative sets. "This reversal does not establish which lead is
truly best; it gives us specific counterexamples to the initial estimate
rather than losing that estimate when more samples arrive."

Transfer of the three frozen descriptions (mean centered action difference
among matching worlds — "not a direct recommendation or player win-rate
improvement"):

| witness-derived description | development | fresh | one-sided permutation p | registered gate |
|---|---:|---:|---:|---|
| partner holds live top trump that beats the alternative (`contrast-110`) | −4.39 pp | −3.92 pp | .0685 | Inconclusive |
| next opponent holds live top trump that beats the alternative | +3.95 pp | −2.80 pp | .9145 | Reversed |
| previous opponent holds a 10-count tile beating neither action | +4.64 pp | −1.81 pp | .7315 | Reversed |

(`result.json`: `contrast-110` `p_one_sided` 137/2000, `provisional_replication`
false; native audit 8,352 checks on 2,784 rows, 0 mismatches.) "All three
match all ten fresh deals and all six hand sizes." "Fresh per-size effects are
retained and are not uniform: for example, the partner-top-trump association
reverses at size five." "'Beats' here is a mechanical rank comparison … It does
not mean the hidden tile will be legally playable, or that its holder will
capture a later trick. These simple descriptions may omit the continuation
relationship that matters."

**Throughput, audit, preservation** (18 single-threaded workers on the Mac):
fresh source games 360 in 51.1 s; development first eight 960 all-action
worlds in 57.9 s; development additions 384 worlds in 28.5 s; fresh first eight
960 worlds in 53.4 s — **2,304 sampled worlds, 9,200 forced-action branches,
171,792 ordinary continuation moves** (`summary.json` records the two
first-eight panels alone: 3,760 / 3,744 branches, 69,432 / 69,592 continuation
moves, `fallbacks: 0`, `over_budget: 0`). Every sample, prefix, legal move,
actor input and terminal score passed the independent replay audit; native
Scheme evaluation agreed with Python on **46,452** memberships; four sampling,
three mining, seven decision and five Rust sampler tests passed — "These audits
check this implementation and recorded evidence, not a theorem of strategic
strength." Artifacts at `/Users/jason/data/texas-42/kiln-played-v1/whole-game-contrasts-v1/`;
fit id `93145118…`, result id `5db9b77d…`; HF snapshot
`20260918T174948.127967Z`, commit `82c3ebc1…` (`backup-evidence.json`).
`SUNSHINE-NOTES.md`: "We now have exactly which additions changed the estimate,
not yet a reusable explanation of why. The learning loop remains play, preserve
contrasts, derive descriptions, challenge them on new hands."

### §8.5 What these studies did not show

Verbatim, one line per source:

- "These are candidate **outcome associations**, not established threat
  mechanisms." / "This probe alone establishes neither causal threats nor
  cheaper/stronger play." / "This probe does not yet ask whether a learned
  proposal reduces Walt's sample cost." — `THREAT-PROBE.md`,
  `threat-probe-v1/RESULTS.md`
- "It is not evidence of a generally reliable learned threat." / "No universal
  statistical guarantee, causal interpretation, or out-of-domain validation is
  claimed." — `threat-probe-v1/RESULTS.md`
- "This is a research gate, not a theorem or a guarantee of player
  improvement." / "Replication establishes an outcome association under this
  player." — `MINING-PROBE-V2.md`
- "It does not show cheaper Walt-internal planning, stronger play, a causally
  correct threat, or a general solution to belief proposals." —
  `MINING-PROBE-V2-CV.md`
- "It does not yet establish stronger partnership play or faster Walt-internal
  planning." / "These are not zero probabilities or probabilities that the
  hypotheses are true." / "it is not a measured wall-time gain, a new
  bid-selection rule, or a speedup of Walt's internal joint-sample planning."
  — `mining-v2/RESULTS.md`
- "These are fixed-player outcome/measurement results, not stronger play or
  faster Walt planning." — `PROGRESS.md`
- "This is modest evidence of a useful shortcut for measurement, not a
  live-player improvement." — `SUNSHINE-NOTES.md`
- "none of this small action grammar's rules improved even development play."
  / "Its zero gain and zero interval are not evidence for a new skill." / "This
  does **not** establish a reliable 'think harder here' detector." / "They are
  not measured sample savings or a fitted allocation procedure." / "These
  results cannot establish the effect of applying a learned fragment repeatedly
  during a game." — `decision-mining-v1/RESULTS.md`, `DECISION-MINING-V1.md`
- "Threat strata explained about 10% of action-difference variation
  retrospectively, which is a possible measurement lead, not demonstrated new
  sample savings." — `SUNSHINE-NOTES.md`
- "None of the first three selected descriptions passed the registered fresh
  gate. The broader evidence and reproducible instrument are the result; a
  stronger player or sample-efficiency gain is not." / "This run does not yet
  demonstrate learned threats, improved sampling or stronger play." / "It is a
  source of study candidates, **not a measured mistake rate**." —
  `whole-game-contrasts-v1/RESULTS.md`
- "This tests an outcome relationship, not stronger play or sample savings." /
  "Patterns remain examiner/belief queries, not direct hidden-information move
  guards or deployed policies." — `WHOLE-GAME-CONTRASTS-V1.md`
- "no learned policy or sampling gain is claimed" / "This is a bounded analysis
  instrument, not a player strength gain or a new sampling policy." / "No claim
  of learned causal threats, revised beliefs, or stronger play" / "not live
  policy guards, causal threat proofs, or established player-strength gains" —
  the four 2026-09-18 paragraphs on [walt-instruments](walt-instruments.md)

## §9 Solver work measured through Kiln (2026-09-18)

While the scalar-price survey (§1) ran, its persistent native worker
(`kiln-worker`, wrapping `walt_player::KilnPricer`) was used as the bench for
a sequence of implementation changes to the shared sampled solver. Each was
measured the same way: a **parity** run (`parity.py`: retained production
receipts plus fresh 4/12/40/160-world cases repriced by the candidate binary,
exact prices *and* work counters — nodes, policy calls, inner worlds — required
to match; from `8ee0cb71` warm receipts are repriced by the baseline first, so
cold counters are compared to cold counters) and a **paired timing** run
(`benchmark.py` / `carry_benchmark.py`: identical retained 160-world requests
or 8/40/160 ladders, four concurrent pairs, alternating old/new order, a 60 s
bound, reported as median and geometric-mean speedup). Each pair of compact
`*-summary.json` files pins the raw evidence under
`/Users/jason/data/texas-42/kiln-v1/` by SHA-256, and names the baseline and
candidate producer identities, so the rows below chain: one row's candidate
hash is the next row's baseline. Every timing is a bounded Mac measurement on
the machine `compiler-target-summary.json` identifies (Apple M5 Max, rustc
1.95.0); `HOT-PATH.md`: the production one-minute firings "ran different
requests and sometimes overlapped builds/tests: their saved counts are
progress, not controlled speedup factors." The three phone builds are Mac
Node WASM comparisons under a frozen host clock — "Node WASM; not phone timing"
(`phone-build-summary.json` `host`). The standing caveat of every record, in
`COMPACT-CACHE.md`'s words: "This is an implementation-equivalence argument
for the current finite solver, not a proof of the field model's predictive
accuracy. The sixes/36 calibration mismatch remains a separate finding."
`HOT-PATH.md`: "These changes preserve the finite sampled evaluator and its
policy. They do not repair the calibration gap or claim stronger play."
Parity receipts here are portability and equivalence evidence, never evidence
of improved strength.

| experiment (commit, record) | what changed | parity receipt (baseline → candidate; cases; result) | timing (as the summary states it) | outcome (verbatim) | phone validation |
|---|---|---|---|---|---|
| counted values (`101d805b`; `COUNTED-VALUES.md`, §1.2) | exact integer success counts C(K)/\|A(K)\| inside the recursion, one reduced BigRational at the boundary | `counted-parity-summary.json`: `14939b7a…` → `0ecafddb…`, 121 cases (117 retained + 4 fresh 4/12/40/160), passed; `serial-counted-parity-summary.json`: `14939b7a…` → `2b0adffe…`, 121, passed | none recorded in a timing summary | adopted — the survey worker's solver from the first commit; "not a new corpus theorem or a claim of player strength" | — |
| compact cache (`662f298c`; `COMPACT-CACHE.md`) | memo and policy-cache keys store the unfinished trick inline (sentinel 1 + five bits per tile, ≤ 3 tiles in 16 bits; injectivity tested exhaustively for lengths 0–3); scalar-word hash routing lookups only, full equality on collision; new `solver/cache.rs` | `compact-cache-parity-summary.json`: `2b0adffe…` → `cc2bc646…`, 121, passed | `compact-cache-timing-summary.json`: 16/16 at 160 worlds, **1.2994× median / 1.2963× geometric**; production firing retained 404 prices, zero errors | adopted — "a paired workload measurement, not a claim about all game states or a four-player match" | — |
| allocation removal (`7377ff32`; `COMPACT-CACHE.md` last paragraph) | per-tile bucket headers on the stack, serial child sums without an intermediate vector, no unfinished-trick vector when the fourth domino completes a trick; `audit.py` added ("audit every Kiln receipt before release") | `allocation-parity-summary.json`: `cc2bc646…` → `51849e8b…`, 40, passed | `allocation-timing-summary.json`: 16/16, **1.2010× / 1.2002×** relative to the compact-cache binary; production firing 455 prices | adopted | — |
| small support (`1a43d763`; `SMALL-SUPPORT.md`) | eight-bit sample-ID set for supports of n ≤ 8 (support id M XOR F, bijective; ascending bit iteration equals the old sorted vectors); 28 stack masks for the serial Dice path; new `solver/support.rs` | `small-support-parity-summary.json`: `51849e8b…` → `751c14b2…`, 64, passed; exhaustive roundtrip of every subset n = 1..8 | `small-support-timing-summary.json`: 16/16, **1.6788× / 1.6716×** relative to the allocation-optimized binary; production firing 780 prices, zero errors | adopted — "It does not change Walt's field model, hidden-information policy, or empirical calibration claims." | — |
| forced Dice choices (`f91533cc`; `HOT-PATH.md`) | a sole legal domino is selected without reconstructing Dice's random stream; public-record hash computed lazily | `forced-dice-parity-summary.json`: `751c14b2…` → `2119905e…`, 64, passed | `forced-dice-timing-summary.json`: 16/16, 1.0017× / 1.0011×; production firing 796 prices | kept in the lineage (the bitset comparison's timing baseline is this candidate) — "**no meaningful measured benefit**" | — |
| bitset access (`f91533cc`; `HOT-PATH.md`) | `mask_of` returns `DominoSet`'s bits directly; `set_of` uses the checked constructor; "No rule is reimplemented" | `bitset-parity-summary.json`: `751c14b2…` → `f138bd42…`, 64, passed | `bitset-timing-summary.json` (baseline `2119905e…`): 16/16, **1.0832× / 1.0837×**; production firing 912 prices, zero errors | adopted | — |
| viewer move ordering (`f91533cc`; `HOT-PATH.md`) | candidates ordered in fixed stack storage; standing winner and count computed once per decision; "The priority formula, ascending-ID tie rule, legal set and visit sequence stay the same … no new partnership feature or heuristic" | `visit-order-parity-summary.json`: `f138bd42…` → `c4678091…`, 40, passed | `visit-order-timing-summary.json`: 16/16, **1.1004× / 1.0987×**; production firing 1,071 prices, zero errors | adopted | `phone-build-summary.json` (`43a5a297`): shared core at `f91533cc` as WASM `ce0e5a95…` vs the prior `2730dc30…`; 12/12 complete decisions, exact option vectors, counters and full partner review matched under a frozen clock; **3.7033× median / 3.7145× geometric** — "Mac Node WASM timings, not Pixel timings"; Plunge full tests 200 passed / 2 optional skipped (`PROGRESS.md`) |
| worker-pool layout (`43a5a297`; `HOT-PATH.md`, `pool_benchmark.py`) | none — a layout measurement of the `c4678091…` worker on a fixed 256-request 160-world workload, production stopped | all five runs completed the same 256 requests; every scalar matched; serial work counters matched | `pool-timing-summary.json`: 18×1 26.04 s; 9×2 33.59 s; 12×1 37.60 s; 24×1 28.72 s; 18×1 repeat 28.79 s | selected **18 processes × 1 thread** — "a measured practical choice, not a proof of universal optimality" | — |
| ThinLTO / one codegen unit (`446a53da`; `HOT-PATH.md` "Compiler profile experiment") | a temporary Cargo profile inheriting release's checked arithmetic; build 41.7 s under a 60 s watchdog | `lto-parity-summary.json`: `c4678091…` → `20722f67…`, 64, passed | `lto-timing-summary.json`: 32/32, 1.0129× / 1.0115× | "**retain the existing release build**" — profile removed; candidate preserved under producer `20722f67…` | — |
| sparse Dice buckets (`1dbea2c0`; `HOT-PATH.md`) | the small-support Dice path records occupied buckets in a 28-bit mask and visits set bits ascending; "Nonempty buckets, sample membership, visit order and success mass are identical" | `sparse-buckets-parity-summary.json`: `c4678091…` → `8b5e7875…`, 64, passed; 30 focused Rust tests | `sparse-buckets-timing-summary.json`: 32/32, **1.1967× / 1.1984×**, production stopped for the timing; separate one-minute candidate firing 1,102 prices, zero errors (`PROGRESS.md`) | adopted — "This changes implementation cost, not the bidding model or its calibration evidence." | `sparse-phone-build-summary.json` (`a8622306`): shared core `1dbea2c0`, WASM `3a6d7d67…` (769,304 bytes) vs `ce0e5a95…`; 12/12 matched; 1.1723× / 1.1536× — "Mac Node WASM, not Pixel timing"; Plunge build and 38 focused tests passed, one external-book test skipped (null manifest) |
| carry cache (`850f589b`; `CARRY-CACHE.md`) | the persistent worker keeps one completed inner-policy cache across the 8 → 40 → 160 refinements; `Shared::take_policy_cache_from` requires all seven shared-context coordinates to agree and moves only completed `pi` answers (never deadlines, counters, outer samples or memo tables); retention capped at 100,000 entries; `carried_policy_entries` recorded per receipt and bounded by the auditor; `solver/mod.rs`, `walt-player/src/auction.rs`, `kiln-worker.rs` | `carry-cache-cold-parity-summary.json`: `8b5e7875…` → `77f81f56…`, 36, passed; Rust tests for warm-vs-cold prices and all seven context rejections; 13 Python tests | `carry-cache-timing-summary.json` (`kiln-carry-timing-v1`, 8/40/160 ladders): 32/32, all 96 prices equal, **1.2445× median / 1.2262× geometric**; `carry-cache-production-summary.json`: 1,562 prices in a one-minute firing, 25.93/s, 0 errors, 18 workers | adopted for the native Kiln worker — "The existing bidder and phone entry points remain stateless"; "the installed phone WASM is intentionally unchanged because this carry slot belongs only to the native Kiln worker" (`PROGRESS.md`, `93cf7be2`) | none (by design, see outcome) |
| packed hash (`8ee0cb71`; `HOT-PATH.md`) | private keys' scalar fields hashed as two packed words with a final avalanche, all fields retained in equality | `packed-hash-parity-summary.json`: `8b5e7875…` → `bec66bf9…`, 64, passed, `adopted: false` | `packed-hash-timing-summary.json` (baseline `77f81f56…`, ladders): 32/32, 96 prices equal in 24.9 s, 1.0118× / 1.0111× | "**retain the current key implementation**" — reverted; archived as `bec66bf9…`; "This negative result does not establish a universal performance ceiling." | — |
| warm checker (`8ee0cb71`; `warm-checker-summary.json`) | `parity.py` reprices warm receipts with the baseline before comparing counters; `benchmark.py` compares two fresh workers | 32 actual warm production receipts, 36 parity cases and 4 paired cases passed; all 32 had different cold node counts, "confirming why the old counter comparison was inappropriate" | `timing_is_performance_evidence: false` | instrument correction, no solver change | — |
| precomputed trick strengths (`91e8925e`; `HOT-PATH.md`) | `Decl::trick_key` reads a compile-time table of 9 declarations × 8 led contexts × 28 dominoes (2,016 entries, 4,032 bytes) generated from the original `tier`/`rank` definitions; `rules/rules.rs`, `tests/rules_exhaustive.rs`; all 2,016 entries checked against the direct rule algebra | `rule-table-parity-summary.json`: `77f81f56…` → `e6cea7cf…`, 64, passed; 44 focused Rust tests | `rule-table-timing-summary.json`: 32/32 ladders, 1.0444× / 1.0438×, all 32 pairs improved; `rule-table-repeat-summary.json`: 64/64, **1.0471× / 1.0451×**, all 192 stage prices matching, production stopped; `rule-table-production-summary.json`: firing of 2,436 prices in a minute, 40.46/s, 0 errors, 218 settled / 218 covered deals | adopted — "Their small but consistent gain supports retaining the lookup"; "an implementation speedup, not a change to the calibration finding or a new claim about playing strength" | `rule-phone-build-summary.json` (`b6c8b273`): shared core `91e8925e`, WASM `95192ca8…` (796,381 bytes) vs `3a6d7d67…`, imported in Plunge `a0437a1`; 12/12 matched; 1.1041× / 1.0968× — "Mac Node WASM, not Pixel timing"; production build and 43 focused game tests passed, one external-book test skipped |
| serial grouping (`45f3c482`; `HOT-PATH.md`) | skip populating the distinct-hand grouping used only by parallel policy preloading when running serially | `serial-grouping-parity-summary.json`: `e6cea7cf…` → `d94470d0…`, 64, passed, `adopted: false` | `serial-grouping-timing-summary.json`: 64/64 ladders, all 192 prices matching, 1.0044× / 1.0042× in 39.6 s | "**no useful gain**" — reverted; snapshot `d94470d0…` | — |
| compiler target inspection (`460fc77e`; `compiler-target-summary.json`) | none — an inspection: the default Mac target already has LSE atomics and NEON; `target-cpu=native` adds `bf16`, `bti`, `i8mm` and "reports the host as apple-m4 even though macOS identifies the machine as Apple M5 Max" | — | — ("It is not a timing comparison") | `decision`: "No new compiler build: default already provides modern integer atomics and NEON; no obvious missing instruction feature for this scalar workload." | — |

`PROGRESS.md`'s one-line digest of the adopted chain: "Forced-Dice draw
elimination measured no useful speedup; bitset access measured about 8%, then
move-order allocation removal about 10%." Two lineage facts the summaries
state and prose does not: the bitset row's parity baseline is the
small-support binary `751c14b2…` while its timing baseline is the forced-Dice
candidate `2119905e…`; the packed-hash row's parity baseline is the
sparse-bucket binary `8b5e7875…` while its timing baseline is the carry-cache
binary `77f81f56…`. `PROGRESS.md` on build identity: "Use the original `-p
walt-player --bin kiln-worker` build invocation for this identity; adding `-p
walt` enables its default feature flag and changes build identity even though
the implemented evaluator is unchanged."

These changes landed on `main` as shared solver source, not as Kiln-local
code: `walt/walt/src/solver/mod.rs` (every adopted row), the new
`walt/walt/src/solver/cache.rs` (`662f298c`) and
`walt/walt/src/solver/support.rs` (`1a43d763`), `walt/walt/src/rules/rules.rs`
with `walt/walt/tests/rules_exhaustive.rs` (`91e8925e`), and
`walt/walt-player/src/{auction.rs,lib.rs,bin/kiln-worker.rs}` (`101d805b`,
`850f589b`); the phone WASM was rebuilt from the same source at `f91533cc`,
`1dbea2c0` and `91e8925e` (table). They are therefore part of the player at
`5ab08bbc` that the v34 CPU integration of 2026-09-20 (`701e8589`,
`f1a0fb04`) names as its baseline — `walt/CPU-SPEEDUPS.md`: "The baseline was
the unmodified current player at `5ab08bbc`"; `walt/CPU-RELEASE-PLAN.md`: "CPU
implementation `701e8589` and receipts `f1a0fb04` are already integrated with
our preceding Kiln work. Reapplying or cherry-picking them is unnecessary",
and its Node panel "includes accumulated Kiln changes, not just the latest CPU
bundle." v34 then rewrote the same modules again (`cache.rs`, `support.rs`,
`mod.rs`, `auction.rs` in `701e8589`'s stat) and carries its own, larger lookup
artifact — the 5,531,904-byte completed-trick table
`solver/compact_dice/trick_table.bin` — distinct from the 4,032-byte
`trick_key` table of `91e8925e`. The release, its receipts under
`walt/receipts/cpu-speedups-v34/` and the hosted phone verification are owned
by [walt-instruments](walt-instruments.md); the module layout by
[walt-architecture](walt-architecture.md); neither is restated here.
