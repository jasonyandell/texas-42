# The partnership gym

**Outcome-only extension, 2026-09-07:** [find the bid-making play](BID-MAKING.md).
The all-legal Scheme query and exact success-only selector found **433**
declaring-side exercises, including 367 unique best plays and 26 certain
make/set swings under the declared field. The same runner accepts
`--gallery walt/gym/collections/bid-making-v1`; no new pupil comparison yet.

**Expanded 2026-09-07:** [Scheme now drives discovery](DISCOVERY.md). Three
ordinary query files found 170 distinct late-game coordinates with strict
outcome differences. The [expanded catalog](collections/scheme-v1/catalog.json)
is ready for the same player runner via `--gallery walt/gym/collections/scheme-v1`.
The six starter exercises and their original results below remain preserved.

Six working exercises: three where offering count to partner helps, three
where holding it back helps. Each has every legal alternative, an exact
make/set answer key under a declared field, and complete same-world paired
replays. [RESULTS.md](RESULTS.md) records the first L1/L2 comparison.

The gym uses **Scheme to express a situation**, the existing lawful exact
evaluator to price decisions, and the existing playable player to take the
exam. It adds no count bonus or hand-crafted decision feature to the player.
Everything here is exploratory finite-domain evidence.

## Use it

From the worktree root:

```sh
python3 experiments/partnership/gym.py report
python3 experiments/partnership/gym.py show advantage-01
python3 experiments/partnership/gym.py show disadvantage-01
python3 experiments/partnership/gym.py verify
```

`show` displays the player's own remaining hand, declaration, public history,
score, all action values, and an explicitly examiner-only hidden-deal witness.
Seats 0/2 and 1/3 are partners. Dominoes display as pip pairs; stored IDs use
the repository's triangular 0–27 ordering. Declaration 0–6 calls that pip,
7 calls doubles, and 9 is no trump. The bid is always 30.

The [scenario catalog](scenarios/catalog.json) names the six portable JSON
fixtures. It records their hashes; each fixture contains its own original
hand/public request, source-deal provenance, exact support size, all action
values, full optimal set, policy identities, and terminal replays. No raw
mining directory is needed to read, verify, or grade the published exercises.

Build both native entry points if needed:

```sh
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 295 --output-dir experiments/partnership/runs/my-gym-build -- cargo build --release --manifest-path walt/Cargo.toml -p walt --bin partnership --bin partnership_gym
```

Run the same exercises with the named native players:

```sh
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 295 --output-dir experiments/partnership/runs/my-gym-slice-1 -- python3 experiments/partnership/gym.py run --output /tmp/my-gym-pupils --workers 10 --seconds 240 --players l1-default l2-partner-default l2-partner-voids
python3 experiments/partnership/gym.py report --results /tmp/my-gym-pupils
```

Repeat the run with the **same output directory** and a fresh watchdog output
directory to resume. A source, binary, configuration, or catalog change refuses
silent resume; choose a new output directory for a new experiment. Each player
still has the existing 14-second move ceiling. The gym runs one native thread
per process, with up to ten simultaneous coordinates.

## What an answer means

A coordinate is one original seven-tile hand plus the actor-attributed public
history, declaration, bidder, and seat to move. It does **not** contain the
actual other hands. The initial belief is uniform over **all mechanically
compatible remaining deals**, including public voids and exact capacities.
It is an explicit experimental prior; earlier observed actions are not
reweighted under an assumed historical player.

The fixed future field is:

- Teammate: the shared playable L1 evaluator, fixed search, 40 outer worlds,
  eight inner L0 worlds, voidless inner modeling, lowest-index ties.
- Opponents: the existing L0-8 seat-local field.
- A deterministic, declared per-information-state seed derived from 420600.
  This differs from the live wrapper's original-hand seed derivation. The
  teacher is a specified policy, not a promise of identical live random tapes.

The teammate's outer sampled beliefs obey public voids; “voidless” above
refers to the L0 minds it models. The pupil may be any named native profile.

For each legal root action `a`, the key computes

`Q(a) = max over lawful focal continuations P(viewer's team succeeds | a, fixed field)`.

The maximum chooses **one action for the whole focal information state**.
Future hidden-seat branches are public actions, weighted by their likelihood
under the declared field. Neither the examiner's actual hidden deal nor a
perfect-information choice enters the focal maximization. This is an exact
focal best response to a declared field, not a joint-team optimality claim.

The score is `max_a Q(a) − Q(chosen)`: **root-action regret** in make/set
probability. Zero means the choice belongs to the entire optimal set. Two
sets tie whether they bank 7 or 29; all makes tie as well. Final score bins and
partner count are diagnostics only. The pupil's future moves are not graded
here: this prices its root choice followed by an optimal lawful focal
continuation. A complete-policy benchmark is a distinct exam.

The starter gallery was selected by outcome category before running the
pupils. It is a diagnostic collection, not a representative strength sample
or a held-out generalization test. No independence claim treats its hidden
worlds as separate games. Teachers and pupils may model different teammates;
changing the field changes the exam and requires a new answer key.

## First Scheme family: offering count

[offer-count.scheme](offer-count.scheme) calls the registered
`offer-count-to-partner` predicate. It is true for a legal count tile when:

1. We play third or fourth in the current trick.
2. Partner is currently winning.
3. Our candidate leaves partner winning immediately after our play.

This is a public/own-hand, one-play relation. It never asks which action is
best and never reads the other hands. When playing third, the last opponent
may still overtake partner; the descriptor makes no promise about that.
The registry declares the predicate's access, types, horizon, and version.
Scheme's presence probability is checked to be 0 or 1 across the full belief.

**Advantage** means an offered-count action is optimal and strictly improves
success over a non-offer alternative. **Disadvantage** means an optimal
non-offer strictly improves success over an offered-count action. All-tied
coordinates are retained by mining as controls, not labeled as advantages.
The label is about the specified action pair under this field. Full replays
show whether immediate count capture, a later lead, or a different continuation
explains a specimen; a pattern label alone is not a causal proof.

## Mine and monitor more

The miner scans completed saved campaigns; it does not need to play new games.
By default it takes the first 60 qualifying trick-5/6 coordinates from the
saved L1/L2 battery, deduplicated by public/own-hand request. The maximum exact
support is 400 worlds. Larger coordinates are explicitly skipped.

```sh
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 295 --output-dir experiments/partnership/runs/my-gym-mine-1 -- python3 experiments/partnership/gym.py mine --source experiments/partnership/campaigns/default-partner-battery/01-level --output /tmp/my-gym-mine --limit 100 --max-worlds 400 --partner-worlds 40 --workers 10 --seconds 240 --case-seconds 45
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 60 --output-dir experiments/partnership/runs/my-gym-select -- python3 experiments/partnership/gym.py select --source /tmp/my-gym-mine --output /tmp/my-gym-gallery --each 3
python3 experiments/partnership/gym.py report --gallery /tmp/my-gym-gallery
```

Progress streams one completion/failure per coordinate. `status.json` in the
output directory reports active work and saved totals; `items/` is the durable
result set and `failures/` records failed attempts. A successful retry may
leave an earlier failure record; a completed item is authoritative.

Ctrl-C stops new admissions and lets each active coordinate finish within its
subprocess timeout. The outer watchdog kills the whole process group if
necessary. Abrupt termination loses at most the **active coordinate per
worker**. Each complete item is flushed and atomically renamed; nothing
partial is accepted. Failed items retry once on the next invocation, while
completed items are skipped. A filesystem lock prevents two coordinators from
writing the same run. Tests inject both failure and interruption.

## Engineering and verification

The language remains independent of the solver: `walt::gym` imports Scheme
and the existing evaluator, not the reverse. The native entry point accepts
only one own hand plus public play. Python dispatch rejects teacher fields.

Every accepted key is checked through:

- Exact policy extraction and separate fixed-policy score repricing.
- Direct complete Rust replays over every legal hidden world.
- Independent Python enumeration by reconstructing full original deals and
  legally replaying history, without trusting native support counts or voids.
- Independent terminal scoring and an information-consistency audit across
  all worlds and root alternatives. Identical seat information must produce
  identical field actions; each focal continuation has the same constraint.

The Python audit verifies attained values and lawful witnesses. Optimality
comes from the existing exact recurrence and its independent enumeration
regression gates; this is not a second independent optimizing implementation.
After make/set is settled, extracted focal tails use lowest legal moves to
finish score diagnostics. Those tails are not count-optimal policies.

Integration exposed the historical zero-completion boundary in single-field
conditioning, already guarded by the model-belief path. The common evaluator
now removes zero-marginal hands **before** calling a field policy, retaining
original factor weights rather than marginal weights. This preserves the joint
measure. The old impossible frame is still independently checked; raw Level1
now completes and agrees with the mixture/world enumeration on all six receipt
roots instead of refusing four of them. The playable selection rules and
defaults did not change.

Further exercise families can use the same request, key, audit, and grading
interfaces: lead transfer, entry preservation, promotion, and partner-policy
counterfactuals. They need their own mechanical Scheme definitions and verified
examples. No automated descriptor generalizer or compact belief transducer is
claimed by this first family.

## Transition machinery, in context

Scheme currently answers a query at a supplied frame and belief. After a play,
someone must advance the public state, update capacities/voids, condition the
belief, and decide whether a selected role still refers to the same tile or is
rebound. That is transition machinery. A **compact** transition system would
additionally have to do this from a smaller sufficient state without reopening
the full hidden worlds. The gym uses existing exact state/belief transitions
and fresh Scheme evaluations; it does not claim that compact replacement.
