# The Gran anchors — the real Plunge "6-4" hands, run by the waking seat

**EXPLORATORY TIER.** This directory sits below every evidentiary tier and
is cited by nothing above it. Probe numbers here become quotable results
only by brief amendment adding them to a verifier receipt. Estimates,
never receipts; not a P-A21 statement. Every sampled number below carries
its world count and its epoch.

## What this means and what it cost

The anchor is the hand Jason nearly lost in a real Plunge game: his walt
partner (the live level-1 seat, screenshot-named **Gran**) held the 6-4
and played the **6-2** at trick 1, and Jason's play depended on knowing
where the 6-4 was. The waking seat (`solver::waking`) is the hypothesized
remedy. This is the first time the actual hand has been put in front of it
from the opening lead.

**At trick 1 the waking seat plays the 6-4.** That is the tile the real
partner withheld, and the flip is real: legal set `{6-2, 6-4}`, the seat
picks 6-4, the record played 6-2.

**But the wake did not fire, so the waking machinery did not cause it.**
The σ0 baseline — `solver::act` at the declared epoch, the same call the
live bridge makes — already picks 6-4 on its own. The trick-1 fiber is
46,558,512, far above the declared exact wake cap (1024), so the wake
check took the sampled route, spent its full 24 paired worlds, and
returned `sampled-open`: honestly unsettled, no wake, play the baseline.
So what this run shows at trick 1 is an **epoch/baseline difference**
between the waking seat's σ0 and whatever the live Plunge seat was running
when the screenshot was taken (its panel: 40 sampled worlds, 6-2 at 90%
against 6-4 at 80%). Those two numbers are from different samplers at
different world counts and **do not compose**. Read loudly: this run does
not demonstrate that partner-modelling fixes the 6-4 problem. It says the
choice was already close enough that the baseline's epoch decides it.

A second caveat that travels with the trick-1 result: the σ0 route there
is `unresolved-level1`. act's own evidence process did not settle at δ;
the 6-4 comes from the level-1 fallback ranking, not from a settled
selection.

**The one real wake landed at trick 5, and it agreed with the human
game.** Fiber 300, under the exact cap, so the check ran the exact route
and positively selected the rival: `exact-sigma1-selects-rival`. The
escalation returned `exact-survivors / provably-useless / exact-argmax`
and selected 4-2, moving the play off σ0's 1-0. The real Gran played 4-2.
The escalation cost 589,818 µs and its own phase vector puts the money in
the screen machinery, not in σ1 values: `steering` 237,596 µs and
`rung-e2` 198,873 µs against `stage4-sigma1` 968 µs.

**Whole-hand agreement with the record is 6 of 7.** Only trick 1 differs.

**Cost.** Seven decisions in the replay run took **25.04 s** of wall time,
trick 1 alone 12.55 s. The driven whole-hand run (the waking seat at all
four chairs) took **166.11 s** for 28 decisions, with trick 1 carrying
102.82 s — 619 permille of the run. Phase attribution splits differently
in the two modes: in the replay the **wake check** dominates (536
permille, against 440 for the baseline), because the walt seat's own
fibers are the small ones and the sampled checks above the exact cap are
what is left; in the driven run the **baseline** dominates (677 permille),
matching the `probes/waking` profile's read. The escalation is 23
permille of the replay and zero of the driven run.

**The driven whole hand is a different game.** With the waking seat in all
four chairs from the G1 deal, the declaring side banks **26** against a
bid of 30 — still set, one point better than the real 25. Zero wakes
across 28 decisions: every non-forced check either exhausted its sampled
budget (14) or settled on the baseline (4 `exact-sigma1-selects-baseline`,
1 `exact-sigma1-tie`), and agreement with σ0 is 28/28.

## Declared configuration

Identical, field for field, to `probes/waking/README.md`'s declared epoch
and to `waking_bridge`'s defaults, so the two censuses compose.

- **Epoch pair (live):** σ0 = `Level0 { n0 = 2 }` — the same field act's
  evaluation runs against; σ1 = `Level1 { n_outer = 4, n0 = 2 }`; frozen
  focal candidates at declared schedule [8, 2]
  (`ActionRule::PinnedThenLevel1`). **The `l2_controller` probe's epoch is
  DIFFERENT (σ0 n0 = 8): numbers do not compose across the two.**
- **Baseline:** `solver::act` at `ActConfig::interactive` (world cap 128,
  exact cap 2000, fallback 200×8), δ_run = 1/100 per hand.
- **Wake check:** budget 24 paired worlds (sampled route), exact route at
  fiber ≤ 1024; wake rule = the σ1 leg positively selects the rival.
  Waking risk budget δ = 1/20 per hand, split per-decision by the
  telescoping `decision_delta` convention under `wake:`-prefixed scopes.
- **Escalation:** `solver::targeted::targeted_root`, exact fiber cap 4096,
  baseline prefix 128, E3 prefix 24, directional off; ε = 1/20.
- **Bid:** 30 (the anchor's own bid, and the value both the bridge and
  this runner carry).
- **No wall clock in the value path.** The played line is a pure function
  of the anchor and the epoch; the microsecond columns are wall time and
  vary run to run, the choices do not.

The runner is `walt/src/bin/granrun.rs`, a new VARIANT surface. It touches
nothing: `waking_bridge`, `walt_bridge`, `controller_bridge`,
`solver::act`'s policy and the live default player are all unmodified. It
drives `solver::waking::WakingSeat` directly under the config above.

## The anchors, and how they were validated

Sources are three Plunge screenshots at
`~/data/texas-42/gran-anchors-2026-08-24/`, `MANIFEST.sha256` verified
before reading and never modified. The "How it went" grid shows every
seat's tile for every trick, so the grid *is* the record; the review
panels below it independently list the walt seat's option set at one
trick, which cross-checks that seat's hand.

Seat map, both anchors: **Y = S0** ("You" — Jason, the declarer), **E =
S1**, **G = S2** ("Gran", the walt seat, Jason's partner), **R = S3**.
Teams are S0/S2 against S1/S3, so the declaring team is T0 = {Jason,
Gran} — the screenshots' "Us".

### G1 — the failed hand (`g1.receipt.txt`) — COMMITTED AND VALIDATED

Bid 30, sixes; final 25–17, set. Sources
`gran-failed-hand-trick1-40w.png` and `gran-failed-hand-trick3-160w.png`,
which show the same hand and gave two independent reads of the grid.

```
granrun validate probes/gran/g1.receipt.txt
```

replays it through `rules::replay::replay_hand`, which trusts only the
tiles and who played them and re-derives everything else. It reproduces:
28 distinct tiles, 7 per seat, the full deal, every follow's legality
under sixes trump, all seven trick winners, all seven trick point values
(the +1/+6/+11 column), team totals 25–17, and the set verdict against
the bid of 30. The derived deal:

| seat | hand |
| --- | --- |
| S0 (Y, Jason, declarer) | 0-0 2-1 3-3 4-4 6-1 6-3 6-6 |
| S1 (E) | 1-1 3-0 4-3 5-1 5-4 6-0 6-5 |
| S2 (G, Gran — the walt seat) | 1-0 2-2 4-1 4-2 5-2 6-2 6-4 |
| S3 (R) | 2-0 3-1 3-2 4-0 5-0 5-3 5-5 |

A second, independent confirmation of S2's hand: the trick-3 review panel
lists S2's five options as 5-2 / 6-4 / 1-0 / 4-2 / 2-2, which is exactly
the derived S2 hand minus the 6-2 and 4-1 it had already played.

**Nothing was ambiguous.** Every pip resolved on the first read and the
28-tile partition closed with no tile missing and none repeated, which is
a strong check in itself — a single misread pip breaks it.

**Not recovered from the screenshots:** the shaker, the auction, and the
match's prior mark ledger. `shaker S3` in the fixture is a placeholder
(`replay_hand` never reads it) and the mark lines are fixture-local for a
single hand.

### G2 / G3 — the made hand (`g2g3.receipt.txt`) — COMMITTED, PARTIAL

Bid 31, sixes; 36–0, made. Source
`gran-made-hand-trick4-saturation.png`.

**The record stops at trick 6.** The declaring side had 36 against a bid
of 31 and the app ended the hand there, so only 24 of the 28 tiles are
recorded and the deal is **not** fully recovered.

```
granrun validate-partial probes/gran/g2g3.receipt.txt
```

re-derives every follow, every winner and every trick's points over the
recorded prefix, reproduces the 36–0 and the made verdict, and then
enumerates the residual honestly. The four unplayed tiles are `4-1 4-4
5-2 5-3`, one per seat. The trick-4 review panel pins one of them: it
lists S2's options as 3-3 / 4-3 / 5-2 / 5-5, so **S2's residual is the
5-2 and S2's complete hand is known**: `6-4 5-0 3-2 3-3 4-3 5-5 5-2`. The
remaining assignment of `{4-1, 4-4, 5-3}` to S0/S1/S3 is **6-way
ambiguous and mechanically undecidable from this record** — none of the
three is a six or a blank, and every failure to follow in the prefix was
on sixes or blanks, so all six assignments are equally legal. The runner
prints all six rather than picking one.

**Consequence for the anchor program.** The G2 root (S2's trick-1 early
6-4 reveal) and the G3 root (S2's trick-4 saturation panel) are
nonetheless fully determined **as information sets** — a waking-seat
decision at S2 needs S2's own hand, which is known in full, plus the
public record, which is known in full, and the three unassigned tiles are
exactly what that seat cannot see. What is missing is the *deal*, so a
driven whole-hand run on this anchor is not available and neither is a
`replay_hand` validation. G2/G3 decision runs were not part of this round
and are not reported here.

## The G1 replay: the waking seat at S2 against the recorded line

The other three seats play exactly what the record says; the waking seat
decides at each of S2's seven turns. Decision ordinals `d` are global
plies (1..28), so the telescoping risk split matches what the live bridge
would spend.

| trick | d | fiber | legal set | record | σ0 | waking | path | wake evidence | wall |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | 3 | 46,558,512 | 6-2 6-4 | **6-2** | 6-4 | **6-4** | no-wake-budget-exhausted | sampled-open, 24 worlds, rival 6-2 | 12.550 s |
| 2 | 7 | 432,432 | 4-1 4-2 | 4-1 | 4-1 | 4-1 | no-wake-budget-exhausted | sampled-open, 24 worlds, rival 4-2 | 5.298 s |
| 3 | 11 | 17,640 | 1-0 2-2 4-2 5-2 6-4 | 6-4 | 6-4 | 6-4 | no-wake-budget-exhausted | sampled-open, 24 worlds, rival 1-0 | 2.756 s |
| 4 | 13 | 8,820 | 1-0 2-2 4-2 5-2 | 2-2 | 2-2 | 2-2 | no-wake-budget-exhausted | sampled-open, 24 worlds, rival 1-0 | 1.581 s |
| 5 | 17 | 300 | 1-0 4-2 5-2 | 4-2 | 1-0 | **4-2** | **wake** | exact-sigma1-selects-rival, 300 worlds, rival 4-2 | 2.849 s |
| 6 | 23 | 12 | 1-0 5-2 | 1-0 | 1-0 | 1-0 | no-wake-settled | exact-sigma1-tie, 12 worlds | 9.550 ms |
| 7 | 26 | 2 | 5-2 | 5-2 | 5-2 | 5-2 | forced | — | 2 µs |

Agreement with the record: **6 of 7**. Agreement with σ0: 6 of 7 (trick 5
is the wake). Wake rate 1 of 6 checked decisions. σ1 action cache 9,489
entries at the end of the hand.

Phase attribution, integer microseconds, shares in exact permille (floor):

| phase | micros | permille |
| --- | --- | --- |
| baseline (σ0 act) | 11,021,108 | 440 |
| wake check | 13,433,307 | 536 |
| escalation | 589,818 | 23 |
| total | 25,044,233 | |

The trick-5 escalation's own `PhaseSpend` vector:

| phase | micros | items |
| --- | --- | --- |
| baseline-sigma0 | 152,301 | 3 |
| rung-e1 | 23 | 1 |
| rung-e2 | 198,873 | 3 |
| steering | 237,596 | 3 |
| stage4-sigma1 | 968 | 3 |
| stage4-ladders | 17 | 3 |

## The G1 driven run: the waking seat at all four chairs

Whole hand from the G1 deal, same epoch, same census record shape as
`probes/waking/driven.jsonl`.

Result: **T0 26 – T1 16**, set against the bid of 30. 28 decisions, 9 of
them forced. **Zero wakes.** Paths: 14 `no-wake-budget-exhausted`, 9
`forced`, 5 `no-wake-settled`. Wake evidence: 14 `sampled-open`, 4
`exact-sigma1-selects-baseline`, 1 `exact-sigma1-tie`. Agreement with σ0:
28/28. σ1 action cache 14,414 entries.

| phase | micros | permille |
| --- | --- | --- |
| baseline (σ0 act) | 112,579,826 | 677 |
| wake check | 53,533,144 | 322 |
| escalation | 0 | 0 |
| total | 166,112,970 | |

By trick, in microseconds (baseline / wake / escalation):

| trick | baseline | wake | escalation | trick total | permille of run |
| --- | --- | --- | --- | --- | --- |
| 1 | 71,477,455 | 31,346,991 | 0 | 102,824,446 | 619 |
| 2 | 30,239,919 | 15,025,792 | 0 | 45,265,711 | 272 |
| 3 | 7,820,717 | 4,278,668 | 0 | 12,099,385 | 72 |
| 4 | 2,376,678 | 1,588,967 | 0 | 3,965,645 | 23 |
| 5 | 661,584 | 1,280,770 | 0 | 1,942,354 | 11 |
| 6 | 3,473 | 11,956 | 0 | 15,429 | 0 |
| 7 | 0 | 0 | 0 | 0 | 0 |

The whole-hand line the waking seats produce is not the real one from
trick 1 onward — S0 opens 4-4 rather than 6-6 — so the 26 is not a
counterfactual "what if only the partner had been the waking seat". The
replay run is the one that isolates the partner's seat.

## How to reproduce

```
cargo build --release -p walt --bin granrun
./target/release/granrun validate         probes/gran/g1.receipt.txt
./target/release/granrun validate-partial probes/gran/g2g3.receipt.txt
./target/release/granrun replay probes/gran/g1.receipt.txt S2 probes/gran/g1-replay.jsonl
./target/release/granrun driven probes/gran/g1.receipt.txt    probes/gran/g1-driven.jsonl
python3 probes/gran/summarize.py probes/gran/g1-replay.jsonl
python3 probes/gran/summarize.py probes/gran/g1-driven.jsonl
```

## Files

| file | what |
| --- | --- |
| `g1.receipt.txt` | the failed hand, complete and `replay_hand`-validated |
| `g2g3.receipt.txt` | the made hand, 6-trick prefix, residual enumerated |
| `g1-replay.jsonl` | raw census, waking seat at S2 against the record |
| `g1-driven.jsonl` | raw census, waking seat at all four chairs |
| `summary-replay.txt` | `summarize.py` output, verbatim |
| `summary-driven.txt` | `summarize.py` output, verbatim |
| `summarize.py` | the summarizer (stdlib only, integers only) |

## What this round did not do

- No G2/G3 decision runs. The made-hand roots are available as
  information sets but were not put in front of the waking seat here.
- No comparison against the live level-1 seat on the same anchor. That
  would be the experiment that actually attributes the trick-1 flip, and
  it needs the level-1 bridge run on G1 at the screenshot's own epoch.
- No claim that the waking seat "solves" the Gran problem. The trick-1
  flip came from the baseline; the one wake reproduced the human play.
