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

The [partnership gym](../../walt/gym/README.md) is now implemented: six first
count-offer exercises, exact lawful make/set answer keys under a declared
field, independent replay audits, and a resumable ten-worker pupil runner.
The [first results](../../walt/gym/RESULTS.md) compare the three default players.
The [Scheme-driven discovery extension](../../walt/gym/DISCOVERY.md) now sweeps
saved late-game positions using query files and publishes 170 distinct strict
exercises across count offers, overtaking partner, and possible partner entries.
The [bid-making extension](../../walt/gym/BID-MAKING.md) publishes 433 declaring
exercises selected only by make-probability differences, including all optimal
ties, exact regret, and 26 certain make/set swings under the declared field.
Its maintained definition is now [one parameterized specification](../../walt/gym/SPECIFICATIONS.md),
with a single generate command, full-answer reproduction checks, and shared
evaluation caches for filtered collections.
[SCHEME-GYM-ASSESSMENT.md](SCHEME-GYM-ASSESSMENT.md) preserves the preceding
investigation and the wider proposed exercise families.

[SESSION-STATUS.md](SESSION-STATUS.md) maps the current deliverables and open
strength question back to the launch request, including the native-L1 / phone
calibration. The native `l1-race` configuration now matches the recovered phone
procedure in 64 decision checks and all 35 fallback-free pairs in a fresh
50-deal match. This is measured parity, not whole-program equivalence.

[HEAD-TO-HEAD.md](HEAD-TO-HEAD.md) records the recovered historical phone source,
the existing arena/pool assessment, a direct recount of the saved mixed games,
and the original two-player comparison design. That format is now implemented
in the shared pool. The [foundation battery](campaigns/foundation-battery/RESULTS.md)
records 524 verified games, selection/void comparisons, and the cost stops for
refined partner modeling.


## Unified selection and fast head-to-head

Start with the [player families](PLAYERS.md): **L1 default**, **L2 Partner
default**, and **L2 Partner with voids**. `match.py players` shows their exact
settings; add `--all` for historical and advanced presets. "Default" means
fixed search. The archived phone and native L1 Race are separate named choices.
The [default-player battery](campaigns/default-partner-battery/RESULTS.md)
completed 400 games: L2 Partner tied L1 on the 100-deal panel, and its void
option scored 12 wins / 17 losses / 71 ties against default L2 Partner. Neither
comparison establishes a strength gain; timing and uncertainty are reported
separately.

The native player now exposes the same selection machinery at the real root
and inside modeled L1 minds. `--selection fixed|refine|race-refine` selects the
root rule; `--modeled-selection` selects the rule for modeled levels >=1.
L0 retains its fixed Dice response. `--n` and `--n1` are independent base
bundle sizes, so a modeled L1 mind is an explicitly budgeted approximation.
Both rules are also available on `table.py` and `experiment.py`.

The new [foundation record](FOUNDATION.md) explains the invariants and gates.
[players.json](players.json) contains named configurations. Existing defaults
remain fixed/voidless; the literal phone keeps its archived policy.

Create a mirrored match, then run the shared pool under its watchdog:

```sh
python3 experiments/partnership/match.py init experiments/partnership/campaigns/my-match --a l1-race --b phone --start 900600 --count 50
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 295 --output-dir experiments/partnership/runs/my-match-slice-1 -- python3 experiments/partnership/pool.py experiments/partnership/campaigns/my-match --workers 10 --seconds 270
python3 experiments/partnership/verify_campaign.py experiments/partnership/campaigns/my-match
python3 experiments/partnership/match.py report experiments/partnership/campaigns/my-match
```

Repeat the pool command with a new watchdog output directory to resume. A
normal slice yields after the current move. `campaign.py stop PATH` pauses an
individual match; `campaign.py resume PATH` clears its stop marker. An abrupt
interrupt loses at most the active decision per worker; validated saved moves
are retained. Source/configuration changes refuse silent resume.

`--panel worlds --worlds-per-hand 20 --count 200` creates ten fixed opening
hands with twenty hidden-hand completions each. The report groups those
completions by focal hand when estimating uncertainty. A two-player match
uses two games per deal; both teams making, or both getting set, is a pair tie.
Per-player void choices, sampling budgets, decision rules, and fallback costs
are explicit. Persistent workers reuse startup, not evaluation state.

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
All three native profiles default to fixed sampling; refinement and racing are
explicit independent options. The phone retains its archived decision rule.

Every non-forced decision first attempts a complete 8/2 L1 fallback, for up
to 1.5 seconds. It then spends the remaining allowance on the requested mode.
An incomplete evaluation contributes no partial ranking. `l1-fallback` and
`legal-fallback` are explicitly labeled; neither means the deeper model ran
successfully. Timeout policies remain lawful but timing-dependent; the
modeled policies used inside completed evaluations have frozen sampling and
selection rules and never choose a clock-dependent fallback. A refined rule
can use more samples when estimates tie; its schedule does not depend on time.

The modeled partner can respond to tiles becoming public and changing scores.
It uses its own hand and resamples what it cannot see. The default
`--inner-belief voidless` preserves the existing L1/L2 approximation. Select
`--inner-belief voids-counted` to track public voids through every modeled
level and use the existing exact uniform sampler over valid hands. The same
flag is available on the table, experiment runner, and campaign initialization;
archived phone seats stay on the reference. See [the strategy and cache
contract](INNER-BELIEF.md). It is a program model, not a fitted human partner model.

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
sets an explicit per-game thread count. Single-decision calls start workers
afresh; new two-player campaigns retain a process per active game while
constructing fresh evaluation state for every request. Belief work, fallback
work, synchronization, and any startup are included in decision timing.
The source packet is immutable; new findings live beside it.

## Resumable 100-deal evaluation

[CAMPAIGN.md](CAMPAIGN.md) defines the fixed-bid-30, make/set-only comparison,
parallel workers, atomic decision/seed checkpoints, stop/resume commands, and
early review rules. Live progress is in
[the campaign status](campaigns/random-420600-699/STATUS.md). Same-opening-hand
hidden-world panels are also supported separately.

For throughput across many games or experiments, use the ten-worker
[shared pool](POOL.md). It measured roughly three times the earlier scheduling
speed on repeated deals, retains all checkpoints, retries failed workers, and
supports multiple campaigns under one shared concurrency limit.

The completed [same-opening-hand panel](campaigns/worlds-520600-699/WORLD-RESULTS.md)
holds ten hands fixed across ten different hidden completions each. Its report
compares conditional make/set frequencies, unchanged opening choices, point
ranges, and sampled opening model scores. Use `worlds_report.py PATH` for this
grouped view; completions within a hand are not independent positions.
