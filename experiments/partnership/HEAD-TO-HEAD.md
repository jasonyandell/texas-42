# Fast head-to-head: analysis and proposed format

2026-09-06. EXPLORATORY. Jason requested inspection of the historical phone
code and existing arena, then discussion of a fast, parallel, interruptible
head-to-head battery. This document records that analysis. No new matches ran
and no generalized runner was implemented during this analysis.

**Subsequent implementation:** the [foundation](FOUNDATION.md) now supplies the
shared selector, independent player specifications, persistent workers, and
this mirrored format through the existing pool. The [fresh battery](campaigns/foundation-battery/RESULTS.md)
records the measurements. The analysis below is retained as the pre-build
design and historical-source assessment; its future-tense statements describe
that earlier point in the session.

## Historical phone source

The preserved WASM exactly matches `walt/walt-wasm/pkg/walt.wasm` at
`9a056f20461fbe951544e4145ee49b726e0f6852` (August 19). See
[provenance](reference/phone/PROVENANCE.md) for both hashes. The commit records
the rebuild and its native smoke comparison. It precedes the August 24 crate
fold, so the historical solver is `walt/walt-m3-probe/src/lib.rs`.

Direct function comparisons against current `walt/walt/src/solver/mod.rs` found:

- `level1_raced`, `level1_race_refined`, and `level1_evaluate` preserve their
  sampling schedules and selection logic. Differences in these functions are
  typed refusal/error propagation around sampler infeasibility and deadlines.
- `best_of` and `record_hash` are unchanged.
- `sample_belief` adds a feasibility precheck before the same rejection loop.
  This is distinct from the opt-in counted sampler for modeled inner minds.
- Viewer recursion now reorders legal children to reach Boolean early exits
  sooner; it does not prune actions from the legal set. The new inner-belief
  option also propagates void context when selected.
- The WASM API gained optional cross-fiber review and follows the shared
  sampler/refusal changes; its play race cap is still `2*n` with refinement
  base `n`. The historical source confirms blocks of 8 and 4x/16x refinement.

This is a bounded source audit, not whole-program equivalence or a fresh build.
The candidate's fixed-sample `baseline` deliberately bypasses the phone's
race/refine procedure. A direct strength match needs no further reconciliation.

## Existing instruments

The older full-game arena is in `/Users/jason/code/mk5-main/arena/`.
Its maintained entry is `wiki/entities/arena.md` in that repository.

- `engine.py:run_paired` swaps team assignments on matched deal seeds.
- `rob_play.py:RobPlay` already uses a persistent bridge pool (default 8)
  through a thread executor. It is not serial across every game.
- `engine.py:_run_lockstep` waits for an entire side's batch before applying
  moves and running the other side. The slowest solve can hold up other games.
- The inspected engine/CLI/bridge path has no durable per-move stop/resume
  protocol, timeout/restart supervisor, or direct archived-WASM adapter.
- Full auctions and matches to seven marks are useful for broader game play,
  but unnecessary for the current fixed-bid-30 play comparison.

The local [pool](POOL.md) already supplies asynchronous game scheduling,
ten-game operation, per-move atomic checkpoints, bounded slices, retries,
cross-campaign fairness, source identities, and live progress. It is the
recommended foundation. Reuse its referee and durability; generalize the
currently hardwired phone/candidate campaign schema.

## Direct head-to-head already in the saved batch

The two mixed arms of
[the 50-deal calibration](campaigns/native-l1-vs-phone-620600-649/CALIBRATION.md)
are direct phone/native-L1 games with the same physical deal and contract,
then swapped policy teams. Recounting only those arms yields:

| Measure | Native L1 | Phone | Tied |
|---|---:|---:|---:|
| Contract wins, 100 plays | 43 | 57 | — |
| Mirrored-deal wins, 50 pairs | 3 | 10 | 37 |

These are existing outcomes, not new independent evidence. A contract win is
a make when declaring or a set when defending. Fallbacks remain part of the
executed players. Every mirrored pair contributes one paired observation.

The recount uses `d = declaring.made`, `f = defending.made` from each saved
result: L1 contract wins are `d + (1-f)`; its pair score is `d-f`.
This also explains why the previous all-phone anchor cancels from the net:
`(d-phone.made) + (phone.made-f) = d-f`. The anchor is useful for attribution,
but unnecessary for direct head-to-head ranking.

## Proposed reusable comparison

Freeze two independently specified players, A and B. Each identity includes
engine/artifact, modeled-seat profile, inner-belief strategy, sample counts,
decision allowance, and fallback policy. The runtime session records game
concurrency and native threads. A native voids setting belongs to its player,
not to an entire campaign shared by both sides.

For every complete deal, freeze bidder, trump, and bid 30 before either game:

1. A occupies the declaring partnership, B the defending partnership.
2. Replay the exact deal and contract with B declaring and A defending.

Keep physical hands, seats, and opening leader fixed; swap the player policies.
No auction runs. Both teammates use their team's named player. Preserve the
own-hand/public-record-only boundary for every decision request.

Let `mA` and `mB` be the make indicators in those two games. The deal score is
`s = mA-mB`: +1 for an A pair win, 0 for a tie, -1 for an A pair loss. If both make
or both are set, the pair is tied, regardless of points. Report pair W/L/T,
mean pair score, contract win fraction `(1+mean(s))/2`, and separate declaring
make rates. Points over 30 are diagnostics only. No opponent-independent
rating is inferred from one matchup; a battery initially produces a matrix
of paired results rather than assuming strength is transitive.

Separate the reusable deal panel from the matchup so later player pairs can
use identical physical deals. Keep decision randomness independent of hidden
deal seeds. Scheduling order must not select a player's random stream.

Support two panel types:

- Independent random deals, for breadth.
- Same-hand, different-world panels (hidden-hand completions): freeze one
  focal hand plus public contract/history, vary the other three actual hands.
  At trick 1 this includes the partner's hand. Make the number of focal hands
  and completions per hand independent parameters, replacing the current
  hardwired ten. Mid-hand panels additionally require history-consistent
  completions and reconstruction of original hands.

Within each fixed-hand group, average paired scores across completions. For
broader strength uncertainty, the independent units are the focal hands, not
every completion treated as an unrelated position. Balanced complete groups
should feed that aggregate. Technical monitoring can update after every move;
strength summaries should use complete pairs/groups in predeclared order to
avoid selecting evidence by game speed. Any formal outcome-based stopping
rule needs to be specified for this new statistic before running it.

## Wall time and durability

Start with the measured ten-game pool. Give idle slots the next game without
a batch barrier. Preserve each move before scheduling the next; resume from
that record after interruption. A hard stop can lose the currently computing
decision in each active worker, not saved moves or completed games. Retry
process failures from the checkpoint; do not rerun valid timeout fallbacks
until a preferred outcome appears. Track crashes and fallback rates by player.

The first speed gain is omitting the third all-phone arm. The existing mixed
100 games consumed about 1,006 aggregate decision-seconds; all 150 games
finished in 202 seconds at ten workers. At comparable occupancy, 100 new
mirrored deals / 200 games would plausibly take roughly 3–5 minutes with the
current players/settings. This is a planning estimate, not a measured new run;
L2 and voids-aware configurations need their own short throughput measurement.

The next candidate optimization is a persistent Node/WASM worker per active
game, loading the unchanged archived module once. Native workers can also
retain their process/thread pool. Keep evaluator state and RNG scoped to each
request; check repeated-request/history-interleaving parity, memory growth,
and killed-worker resume before relying on the warm path. Its speedup has not
been measured. The parent must retain timeout/restart control over synchronous
WASM. Avoid a solver rewrite solely to obtain faster dispatch.

First implementation slice: two player specifications, two mirrored arms,
the paired report and generalized panel configuration, using the existing
pool/checkpoints. Then profile warm workers and concurrency on a small fixed
timing panel before running the battery. Every new matchup should preserve
its configuration and distinguish development panels from fresh evaluation.
