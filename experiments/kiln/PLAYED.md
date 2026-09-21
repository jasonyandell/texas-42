# Kiln: actual played-game performance

**2026-09-19 release:** the500-hand campaign completed at305,440 games and passed
its full independent audit. [The empirical bidder is deployed](PLAYED-BIDDER-RELEASE.md)
in Plunge, using all125 catalogue deals and the agreed80% score-tail rule. The
completed export is `played-book-500-final.json`; original receipts and the older
100-hand book are intact.75 capped-unsettled panels retain that label. Production
and monitoring remain stopped. Earlier sections below describe the build history.

Exploratory empirical evidence. User direction, 2026-09-18: the unit is the
player making bids. Its internal forecasts are not the outcome being measured.

For each fixed seven-domino bidder hand and each of nine declarations, uniformly
shuffle the other21 dominoes into three seven-domino hands. All four seats play
the deployed Walt procedure, targeting30, through **all28 moves**. Record the
declaring team's final points,0–42. The score distribution supplies every tail
frequency30–42 without another game for each prospective bid. Recommend the
highest threshold reached in at least4/5 of completed games. This is deliberately
a practical bid30-play heuristic, not a claim about a policy retargeted to36.

Initial scope: **100 individual bidder hands**, obtained from the first25 Plunge
deals at seeds420600–420624, four seats each. This preserves complete catalogue
deals for later game integration. Original other hands determine the catalogue
deal only; evaluation reshuffles them. Complete the first game across the panel
before the next. Keep prior games when adding samples. Initial depths8→40→160.

**2026-09-19 expansion:** append 400 bidder hands, for **500 total** from 125
complete deals at seeds420600–420724. All nine declarations gives 4,500 panels.
The 42,008 original games and original 100-hand export remain intact. This range
now includes earlier research exam source seeds; they are consumed research
data, not an untouched future confirmation set.

After the existing 8/40 screens, panels reach160 games. At160 and320, compare
each observed score tail30–42 with the4/5 recommendation cutoff using Wilson
intervals with z=1.96. If any interval contains0.8, add the next stage, up to640.
Check only at stage boundaries. The preselected2% audit cells bypass screening
and uncertainty stopping, reaching640 regardless. At the cap, retain an explicit
`capped-unsettled` label if ambiguity remains. These intervals allocate compute;
they are **not simultaneous or anytime confidence guarantees**, and adaptive
screening does not certify bid reliability. The underlying empirical bid rule,
deployed player and uniform completion sampler are unchanged.

`extend` acquires the writer lock, appends hands/cells and updates the database
manifest atomically. It saves the previous manifest and every original receipt
hash under `extensions/`, records the allocation plan and an extension event,
and is idempotent. Each subsequent run records its allocation plan separately.
The queue still prioritizes the lowest trial index, so new-hand coverage comes
before expensive refinements. Every completed game remains available.

Same hidden completion and policy seed across declarations for a given own hand
and trial index. The two seeds use independent labels. The player sees only its
own original hand and public plays; the full deal exists only in the host. Opening
160worlds/20s, later40worlds/14s with the existing partner review. Post-contract
play uses that same unchanged player and its existing ties. Retain original
per-move calls/responses, final scores, timing, and immutable source/binary bundles.

Early screening is explicitly heuristic: at8 games, at most one made30; at40,
fewer than20 made30. A preselected2% of cells bypass screening for deep audits.
`init --no-screen` disables screening for the whole campaign. Screened cells keep
their actual histograms and sample counts, never an invented zero probability.

One complete game is the atomic durable unit. A stop loses at most the unfinished
game in each worker; completed games survive even an abrupt coordinator death.
An exclusive writer lock and SQLite WAL/FULL with macOS fullfsync protect the
queue. Retries keep trial identities. Profile changes require a new campaign;
semantics-preserving host optimizations may add a producer to the same campaign.
The receipt auditor independently replays every move using the existing Python
rules and checks original actor-only requests, full-game scores and sample seeds.

## Commands

```sh
cargo build --release --locked --manifest-path walt/Cargo.toml -p walt-player --bin kiln-play-worker
python3 experiments/kiln/played.py init /Users/jason/data/texas-42/kiln-played-v1 --hands 100
python3 experiments/kiln/played.py run /Users/jason/data/texas-42/kiln-played-v1 --workers 18 --seconds 60 --games 8
python3 experiments/kiln/played.py status /Users/jason/data/texas-42/kiln-played-v1
python3 experiments/kiln/played.py run /Users/jason/data/texas-42/kiln-played-v1 --workers 18 --seconds 0 --games 160
python3 experiments/kiln/played.py audit /Users/jason/data/texas-42/kiln-played-v1
python3 experiments/kiln/played.py export /Users/jason/data/texas-42/kiln-played-v1 /Users/jason/data/texas-42/kiln-played-v1/book.json
```

Use `caffeinate -i` for unattended production. Ctrl-C/SIGTERM stops cleanly;
the same run command resumes. `status.json` is a replaceable live view, the
database is authoritative. Keep the database and its WAL together while running.
Failed jobs stop after three errors and remain visible; `run --retry-failed`
retries those same trials after the cause is addressed. Completed games are never
replaced by a retry. An explicit stop is not a request for a monitor to restart it.
Export is a separate empirical schema; the old model-book consumer must not
silently consume it. Plunge installation/deployment remains a later step.

For the500-hand adaptive campaign (use Python3.12; the system SQLite runtime has
previously failed to read this database's WAL correctly):

```sh
python3.12 experiments/kiln/played.py extend /Users/jason/data/texas-42/kiln-played-v1 --hands 500 --refine-games 640
python3.12 experiments/kiln/played.py run /Users/jason/data/texas-42/kiln-played-v1 --workers 18 --seconds 60 --games 160 --binary /Users/jason/data/texas-42/kiln-played-v1/producers/5a746760284713ca17397a985e8ec53dd3489ab3526aaecff467a5fec32a2c66/kiln-play-worker
# Same command with --seconds 0 resumes unattended production.
```

The stored allocation plan sets the640 ceiling; `--games 160` supplies its base
depth. Status/export distinguish screened, resolved, refining, audit-complete
and capped-unsettled panels. Completion means the planned allocation finished,
not that every panel reached640 or every bid was statistically resolved.

## Preserved survey

[Sample history](SAMPLE-HISTORY.md) reconstructs every empirical recommendation
change without altering production. Every game remains available, including
non-changing controls, and explanation packets preserve the exact before/after
prefixes plus original game receipts. This is the evidence base for later
experiments with sample allocation or belief proposals, not a sampler change now.

Both live campaigns reside outside git under `/Users/jason/data/texas-42/`.
`backup.py` makes consistent SQLite snapshots while production continues, includes
receipts and immutable producer/source bundles, and reads back every archive
member to verify its hash. Local backups live under `~/data/texas-42/backups/`.
Optional uploads use the existing **private** Hugging Face dataset
`jasonyandell/texas-42-walt-archive`, in a separate `kiln/snapshots/` namespace.
Uploads are checked by downloading and hashing the archives at the saved commit.

```sh
python3 experiments/kiln/backup.py --upload
# Later milestones need only the growing actual-play campaign:
python3 experiments/kiln/backup.py --campaign played --upload
# An authentication failure leaves the verified local snapshot intact:
python3 experiments/kiln/backup.py --existing /absolute/snapshot/path --upload
```

The previous `kiln-v1` database, receipts, producer bundles and partial export
remain intact:1,001,348 audited prices covering851 fully settled deals. See
[PROGRESS.md](PROGRESS.md) and survey-frozen-summary.json. No old scalar price is
counted as a played game. The observed calibration failure stays useful evidence.

## Throughput and verification, 2026-09-18

The first60-second firing retained382 games (10,696 independently checked moves),
6.35 games/second, zero errors, budget overruns or fallback moves. Opening choices
consumed75.5% of measured player time. The default player now reuses completed
inner-policy answers between its40- and160-world opening stages. The existing
seven context guards exclude incompatible caches; deadlines, outer samples,
search state and counters remain fresh. Retention lasts only for one decision
and is released before partner review. No policy or sample budget changed.

The first paired comparison matched all choices, action values, partner reviews
and game outcomes across72 full-game replays (2,016 moves), at1.147× aggregate /
1.143× median speedup. Timing and actual work counters are excluded from equality.
See played-stage-carry-benchmark.json and the final-build repeat. This optimization
is shared player source; the phone asset has not been rebuilt or deployed here.

A fixed72-game pool comparison measured18 workers in10.29s,12 in14.60s,24 in10.92s,
and a repeated18 in11.42s. Every intermediate semantic receipt matched across
layouts. Choose18 single-thread workers:24 has no clear repeatable advantage,
while12 is slower. These are bounded Mac measurements, not Pixel timings or a
claim of a universal optimal pool size. Scripts played_benchmark.py and
played_pool.py reproduce the comparisons within a one-minute cap.

Tests cover own-information boundaries and all9 declarations ×4 seats,
full-game scoring, exact sample identities, abrupt coordinator death, lock
exclusion, resume without replacing committed bytes, progressive sample prefixes,
score-tail export, screening audits, cache equivalence and fresh deadlines.
