# L1 with a bounded partnership continuation check

`l1-partner-rollout` is an optional sampling-stack preset. It keeps ordinary
fixed L1 40/8 as its baseline, then sometimes investigates how offering or
withholding count affects the partnership's chance of making 30. The
[sunshine experiment](campaigns/sunshine-rollout-v1/RESULTS.md) records its
measured scope, costs, improvements, and mistakes. L1 default is unchanged.

## The player

The check wakes when the declaring-side player has at most three own tiles,
partner is currently winning the trick, the contract is still unresolved,
and legal choices include both a count offer and a non-offer. The own/public
detector specializes the existing [offer-count Scheme](../../walt/gym/queries/offer-count.scheme).
The native worker checks that detector against the actual Scheme expression
before changing a move. Already offering count does not skip investigation.

For a uniform mechanical support of at most 400 compatible worlds, it shuffles
worlds without replacement and considers at most 64. Each world compares
**every legal root action**, followed by completed default L1 decisions at
every seat. Each future chooser receives only that seat's original hand and
public play history, with the live L1 seed formula. The shared native L1
evaluator decides; there is no perfect-information move chooser in this layer.
The simulator uses world hands only to deal tiles, enforce rules, and score.

The worker caches identical own/public decisions within the check. It stops
each trajectory when make/set becomes irreversible. A world's evidence counts
only when every root action has completed its trajectory. At least eight
complete worlds are needed, unless the entire support is smaller. Choose a
strictly higher observed make count; equal values retain the baseline. Points
beyond the bid never break a tie.

## What the answer means

A census gives exact values **for these completed L1 continuations and this
uniform mechanical belief**. Neither the future player's behavior nor the
hidden-hand distribution is guaranteed to describe a human table. This belief
respects capacities and public voids but does not infer likelihood from earlier
choices. Inner L1 samples remain the default voidless strategy.

A sampled decision is a fallible guess. The deadline can truncate the shuffled
prefix according to computation cost; it is not claimed to be an unbiased
fixed-size estimate or a calibrated confidence statement. The output reports
coverage, support, completed worlds, all action values, and paired saves/losses.
No answer-key values or actual source-game hidden hands enter the live request.

## Time and failure

The extra allowance is at most 500 milliseconds, including process startup,
inside the existing 14-second move ceiling. The worker gets a soft deadline
35 milliseconds earlier. A future L1 timeout discards the current world's
incomplete comparison. Too little evidence, large support, malformed output,
or worker failure preserves the completed baseline. The check only runs after
the requested full L1 completed; it never spends time reviewing a fallback.

This candidate models completed default L1 at future seats. If the assembled
player encounters this check again later, or an actual future move falls back
under load, that behavior differs from the frozen continuation model. Full
arena games measure the assembled behavior separately from the gym's root
interventions.

## Run and inspect

Build the two native executables:

```sh
cargo build --manifest-path walt/Cargo.toml --release -p walt \
  --bin partnership --bin partner_rollout
```

Use `player.py --review partner-rollout` with an ordinary seven-field request,
or choose `l1-partner-rollout` in the [arena](README.md). Only fixed L1 40/8
with default voidless inner belief is accepted; configuration mismatches are
errors. See [player names](PLAYERS.md) for the distinction from L2 Partner and
the older `l1-partner-count-review` experiment.

`review_result` retains the investigation. `baseline-reviewed` means it changed
the move; `retained`, `inactive`, or an explicit unresolved status keeps L1.
The native `partner_rollout --baseline TILE --audit` also emits complete
retained traces and the exact own/public decision inputs, for independent
parity checks. `rollout_study.py` measures against materialized complete gym
keys and supports atomic per-root progress and resume.

Run workloads through the [watchdog](packet/texas42-partnership-launch-v0.1/tools/run_capped.py),
with a new output directory for each slice and at most 295 seconds. The shared
pool supports ten simultaneous games, per-move checkpoints, and explicit
stop/resume. The deadline is a ceiling, not a target.
