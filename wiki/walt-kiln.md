[Home](Home.md) · owns: Kiln — the opening bidding book for Plunge: the frozen scalar-price survey (`kiln-v1`), the actual-play campaign (`kiln-played-v1`), the sixes/36 calibration finding, the empirical bidder release in Plunge (2026-09-19), the sample-history instrument, the Sunshine Atlas and Pattern Workshop views of the played corpus · Sources: [`experiments/kiln/README.md`](../experiments/kiln/README.md), [`PLAYED.md`](../experiments/kiln/PLAYED.md), [`PLAYED-BIDDER-RELEASE.md`](../experiments/kiln/PLAYED-BIDDER-RELEASE.md) + [`played-bidder-release.json`](../experiments/kiln/played-bidder-release.json), [`PROGRESS.md`](../experiments/kiln/PROGRESS.md), [`CALIBRATION-SIX36.md`](../experiments/kiln/CALIBRATION-SIX36.md) + [`calibration-six36-summary.json`](../experiments/kiln/calibration-six36-summary.json), [`COUNTED-VALUES.md`](../experiments/kiln/COUNTED-VALUES.md), [`SAMPLE-HISTORY.md`](../experiments/kiln/SAMPLE-HISTORY.md) + [`sample-history-evidence.json`](../experiments/kiln/sample-history-evidence.json), [`ATLAS.md`](../experiments/kiln/ATLAS.md) + [`ATLAS-v1.json`](../experiments/kiln/ATLAS-v1.json), [`BICLUSTERS.md`](../experiments/kiln/BICLUSTERS.md) + [`BICLUSTERS-v1.json`](../experiments/kiln/BICLUSTERS-v1.json), [`survey-frozen-summary.json`](../experiments/kiln/survey-frozen-summary.json), [`one-third-audit-summary.json`](../experiments/kiln/one-third-audit-summary.json), [`halfway-audit-summary.json`](../experiments/kiln/halfway-audit-summary.json), [`played-completion.json`](../experiments/kiln/played-completion.json), [`played-startup-audit.json`](../experiments/kiln/played-startup-audit.json), [`played-live-audit.json`](../experiments/kiln/played-live-audit.json), [`extension-500-audit.json`](../experiments/kiln/extension-500-audit.json), [`extension-500-startup.json`](../experiments/kiln/extension-500-startup.json), [`extension-500-first-eight.json`](../experiments/kiln/extension-500-first-eight.json), [`backup-evidence.json`](../experiments/kiln/backup-evidence.json), [`completion-backup-evidence.json`](../experiments/kiln/completion-backup-evidence.json), [`phone-build-summary.json`](../experiments/kiln/phone-build-summary.json), [`backup.py`](../experiments/kiln/backup.py), [`played.py`](../experiments/kiln/played.py); [`walt/walt-player/README.md`](../walt/walt-player/README.md), `walt/walt-player/src/bin/kiln-worker.rs`, `kiln-play-worker.rs`; [`experiments/partnership/PLUNGE.md`](../experiments/partnership/PLUNGE.md), [`SUNSHINE-NOTES.md`](../experiments/partnership/SUNSHINE-NOTES.md) (2026-09-19 paragraphs); commit messages `101d805b..5ab08bbc` on `main` (2026-09-18 → 09-19). Results files (`*.json`) outrank prose; where they differ the page says so. Related: [walt-seat-play](walt-seat-play.md) (the deployed player Kiln measures), [walt-partnership-program](walt-partnership-program.md) (the Sunshine program Kiln serves), [walt-instruments](walt-instruments.md) (the uncurated landing paragraphs of 2026-09-18), [walt-scheme-fix](walt-scheme-fix.md).

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

<!-- §8 mining studies and §9 solver studies: appended in round 2 -->
