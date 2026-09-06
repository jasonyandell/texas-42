# A partner-aware Texas 42 player

**Exploratory, playable native prototype.** It evaluates a thinking partner
while retaining level-0 modeled opponents, as in L1's field. The default is 40
outer worlds, 8 worlds per modeled L0 mind, and 2 per modeled L1 partner.
It keeps a legal move in reserve and allows at most 14 seconds per decision,
leaving headroom within the 60-second four-play trick target.

See [REPORT.md](REPORT.md) for measured behavior, latency, and actual gameplay
outcomes. [BASELINE.md](BASELINE.md) freezes the separate local phone artifact
and its 40/8/racing configuration. The complete original launch packet and
both mathematical notes are preserved in [packet/](packet/).

## Play a hand

From the worktree root:

```sh
python3 experiments/partnership/table.py
```

You sit at seat 0, select trump after seeing your hand, and declare 30.
Your experimental partner sits across from you; the opponents use the archived
phone Walt. The table displays your hand, legal choices, every public play,
trick winners, and points. Enter a numbered choice or a tile such as `6-4`.
`--bid 31` changes the contract. `--decl 6` preselects sixes;
7 means doubles and 9 means no trump. This table covers the play of one
straight-42 hand; it does not implement an auction or a marks match.
Ctrl-C closes it. Human thinking time is outside the player allowance.

## Decision interface

For integration, `player.py` reads one JSON object per line and writes one
JSON decision. Its only inputs are the acting seat's seven original tiles,
public play, contract, declaration, and public randomness. There is no field
for a partner's or opponent's hidden hand. Tile ids use `h*(h+1)/2+l`.

```sh
python3 experiments/partnership/player.py --mode partner
```

Example input, the reconstructed Gran 6-2/6-4 decision:

```json
{"decl":6,"bid":30,"bidder":0,"seat":2,"hand":[1,5,11,12,17,23,25],"plays":[0,27,1,21],"seed":420600}
```

The response includes `choice`, `legal`, `leader`, team `points`, `route`,
timing, phase outcomes, and completed action estimates with exact rational
numerators/denominators. Values estimate **declaring-team contract success**;
defenders minimize them. The public seed must not reveal the hidden deal.

Modes: `partner`, `baseline` (same fixed search with a weaker partner),
`all-l1` (ordinary L2 field), and `phone` (the preserved WASM). Candidate knobs
are `--n`, `--n0`, `--n1`; `--budget-ms` accepts 100 through 14000.
No saturation-tie refinement is used by the three native fixed-sample modes.
The phone retains its own racing and refinement semantics.

Every non-forced decision first attempts a complete 8/2 L1 fallback, for up
to 1.5 seconds. It then spends the remaining allowance on the requested mode.
An incomplete evaluation contributes no partial ranking. `l1-fallback` and
`legal-fallback` are explicitly labeled; neither means the deeper model ran
successfully. Timeout policies remain lawful but timing-dependent; the
modeled policies used inside completed evaluations have fixed sample counts
and never choose a clock-dependent fallback.

The modeled partner can respond to tiles becoming public and changing scores.
It uses its own hand and resamples what it cannot see. It currently forgets
past void information in its inner belief, following the existing L1/L2
approximation. It is a program model, not a fitted human partner model.

## Rebuild and check

Requires Rust/cargo, Python 3.9+, and Node 23.6+ for the preserved TypeScript
phone wrapper. From the worktree root, choose fresh run-directory names:

```sh
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 295 --output-dir experiments/partnership/runs/my-build -- cargo build --release --manifest-path walt/Cargo.toml -p walt --bin partnership
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 60 --output-dir experiments/partnership/runs/my-checks -- python3 experiments/partnership/checks.py
```

The focused check driver links the already-built library into the specific
partnership tests, avoiding builds of unrelated probe binaries. It also checks
the independent Python rules against native Rust across all declarations,
worker failures, fallback validation, and timeouts. Full Rust CI was
**deliberately not run**, as authorized in the launch brief.

## Repeat a small experiment

```sh
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 120 --output-dir experiments/partnership/runs/my-roots -- python3 experiments/partnership/experiment.py roots --n 40 --n0 8 --n1 2 --modes baseline,partner,all-l1 --ids g1-t1-s2
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 295 --output-dir experiments/partnership/runs/my-hands -- python3 experiments/partnership/experiment.py hand --deal-seed 420601
```

Each batch is entirely inside the watchdog. It records commands, clocks,
exits, timeouts, binary hashes, settings, worker routes, every executed move,
and completed outcomes. A timed-out batch's unfinished hand is not a result.
Hands are deterministic Python `random.Random(deal_seed).shuffle(range(28))`,
seven consecutive tiles per seat, sorted. Decision seed 420600 is fixed
independently of all deal seeds. Contracts are matched and fixed; no result
here measures an auction improvement. The harness sends only lawful requests
to workers and independently checks every move against the referee's deal.

Native worker invocations default to six Rayon threads; the resumable campaign
sets an explicit per-game thread count. Workers start afresh on
each decision, so caches are evaluation-local. Their startup, belief work,
fallback work, synchronization, and cleanup are included in decision timing.
The source packet is immutable; new findings live beside it.

## Resumable 100-deal evaluation

[CAMPAIGN.md](CAMPAIGN.md) defines the fixed-bid-30, make/set-only comparison,
parallel workers, atomic decision/seed checkpoints, stop/resume commands, and
early review rules. Live progress is in
[the campaign status](campaigns/random-420600-699/STATUS.md). Same-opening-hand
hidden-world panels are also supported separately.
