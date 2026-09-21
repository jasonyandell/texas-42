# Known partnership mistakes under deployed continuations

2026-09-13. Exploratory full-support diagnostic under the
[fixed protocol](PROTOCOL.md). This measures competence on rare partnership
situations, not their contribution to average ordinary-game win rate.

**Two of the six originally missed offers still improve making the bid when
deployed L1 players continue; four reverse and become worse.** The bounded
review makes five changes, including four harmful ones under these
continuations, and times out on the strongest useful correction. This identifies
a real transfer problem between the original gym teacher and the deployed
players, alongside a computation-budget problem on one valuable case.

We also retain a concrete instance of the intended annoyance: in one compatible
world of advantage-27, giving partner 6–4 produces 36 declaring points and a
make, while keeping it and playing 6–5 produces 25 and a set. All four players,
hidden hands, and public history are held fixed apart from that first move.
Across all 90 worlds at that position, the offer saves one contract and loses
none. The reviewed player selects it. [Full paired plays](WITNESSES.md).

## The six original misses

The table forces the named first move, then lets **ordinary deployed L1 play
every seat**. Counts cover every mechanically compatible hidden completion of
the original root. Each root retains its original information, bid 30, seed,
and equal world weights.

| Original exercise | Baseline play | Offer | Baseline makes | Offer makes | Offer saves / loses | Actual root review |
|---|---|---|---:|---:|---:|---|
| advantage-06 | 1–1 | 5–0 | 129/210 | **151/210** | 28 / 6 | Times out; keeps baseline |
| advantage-20 | 2–0 | 3–2 | **28/40** | 27/40 | 0 / 1 | Chooses offer |
| advantage-22 | 4–0 | 6–4 | **55/76** | 51/76 | 4 / 8 | Chooses offer |
| advantage-27 | 6–5 | 6–4 | 57/90 | **58/90** | 1 / 0 | Chooses offer |
| advantage-29 | 6–1 | 6–4 | **167/210** | 160/210 | 13 / 20 | Chooses offer |
| advantage-30 | 6–0 | 6–4 | **108/120** | 107/120 | 2 / 3 | Chooses offer |

The offers are uniquely best among all tested legal root moves in advantage-06
and advantage-27. The original baseline is uniquely best under L1 continuation
in the other four. These are action values for that fixed continuation policy,
not best-response values against arbitrary players.

The same strong case can contain both gains and losses. Advantage-06 has a
world where offering 5–0 changes declaring points from 18 to 40, and another
where it changes them from 40 to 23. The offer remains the better guess under
the declared uniform root prior: its 28 saves exceed its six losses. Those
individual outcomes do not make the actor omniscient or invalidate the choice.

## First-move repair versus complete reviewed partnership

| Comparison on the six misses | Saves | Losses | Ties | Mean change across the six roots |
|---|---:|---:|---:|---:|
| Always take the original teacher's preferred offer, then L1 | 48 | 38 | 660 | −0.0571 percentage points |
| Take the reviewer's actual root choice, then L1 | 20 | 32 | 694 | −1.8031 points |
| Use the reviewed declaring team throughout | 21 | 32 | 693 | −1.7238 points |

Means give each root equal weight. Pooled saves/losses give each enumerated
world equal weight, so they can have a different sign when roots have different
support sizes. In particular, 48 saves versus 38 losses is a pooled net gain
of 10/746, but not an equal-root gain. Neither weighting estimates ordinary-game
prevalence or supplies independent-trial confidence bounds.

The reviewer retains the baseline at advantage-06 because its extra 250 ms
allowance expires. Later review adds one saved world there. The other five
roots have identical outcomes under the root-only repair and whole reviewed
partnership. Thus later deployed reviews do not rescue the four adverse root
changes in this panel.

## Withholding controls and the certain example

All **38 withholding controls** retain their baseline root move. Both the
root-only and full reviewed-team comparisons tie baseline on all **1,170
worlds**: the intervention causes no additional harm on these selected controls.

Their original teacher labels also need qualification under deployed play.
Compared with the saved baseline, the teacher-preferred offer remains worse
in 34 roots, ties in two, and becomes better in two. Across these controls,
forcing those offers saves nine worlds and loses 159. One reversal has an even
better nonoffer available; it does not make count-offering uniquely necessary.
The other, disadvantage-23, favors offering 6–4 over keeping 6–0 by 109/120
versus 107/120. The current reviewer preserves the older withholding judgment
there. This is an additional missed opportunity under the deployed continuation.

The positive control **advantage-09** retains its strong distinction under
actual L1 continuation: give partner 5–0 and make in **3/3** worlds, or overtake
with 3–1 and make in **0/3**. Both baseline and reviewed L1 already give the
count. No improvement is available for this candidate at that root.

## What this resolves

The Scheme expression still identifies the intended public relationship:
giving count while leaving partner currently ahead. The old evaluator also
answers its declared question correctly on the retained finite traces. What
changed here is the continuation: deployed L1 opponents replace the teacher's
L0 opponents; deployed focal L1 replaces an optimal lawful focal continuation;
and live-wrapper field seed schedules differ from the teacher's schedules.
This experiment changes those together and cannot assign the reversals to one
of them individually.

The old label “missed partnership play” was conditional on those assumptions.
Four labels fail to transfer even on their exact original public positions.
The explanation is therefore more specific than sparse opportunities in random
games. The query detects a useful kind of opportunity, but the current checker's
value judgment is not reliably calibrated to the players that will continue.

For this skill, retain the exact deployed-continuation exam and the gain/loss
witnesses. The next repair should make its continuation assumptions match the
intended partner and opponents, or validate them before overriding L1. The
strong advantage-06 case separately supplies a concrete target for making a
useful check fit its budget. These are next questions; no player or review cap
was changed in this experiment, and the optional candidate remains experimental.

## Execution, reproducibility, and limits

- Fixed panel: six misses, 38 withholding controls, and one certain control;
  **45 distinct roots from 24 source seeds**, covering **1,919 root/world pairs**.
  Every legal action plus the reviewed-team arm was completed: **7,338 full
  trajectories**, 41,602 unique frozen decisions, and 64,980 continuation decision
  uses. Worlds and trajectories are not independent source deals.
- A deliberate SIGINT stopped after 13 durable trajectories. Resume preserved
  all **121 saved trajectory/decision files byte-for-byte**. No job failed.
  Interrupted execution took 0.485 seconds and resumed execution 57.712 seconds,
  for **58.20 seconds** including the final complete replay audit. Panel
  preparation and original-trace checks took another 0.750 seconds.
- Root review statuses: 38 retained, five changed, one timeout, one inactive.
  Mean root invocation time was 7.52 ms for L1 and 42.47 ms for reviewed L1 on
  this selected endgame panel. Cached continuation speed is not live arena
  latency. No saved decision exceeded its overall budget or used an original
  fallback. The one root-review timeout is retained and counted.
- **76 Python tests passed**, including seven new coverage, pairing, forced-action
  provenance, world-integrity and resume tests. Independent rules audited each
  reconstructed deal and complete record, every own/public player request and
  configuration, all 42 points and make flags. All completed root-review values
  match their original teacher values and field IDs. Player source and binary
  identities match the previous frozen sunshine experiment.
- Full-support values are exact for these frozen decision realizations and the
  uniform mechanical prior. They do not remove timed-execution variation,
  selected-case bias, possible behavioral reweighting of earlier public play,
  or dependence on the fixed future players. No Lean or population-strength
  claim is made.

Use the [replay guide](../../GYM-REPLAY.md) to rerun or inspect the instrument.
The [portable panel](panel.json), [summary](summary.json),
[all retained witnesses](witnesses.json), [run receipts](run-receipts.json), and
[resume verification](resume-verification.json) are checked in. A deterministic
compressed [artifact hash index](artifact-hashes.json.gz) covers every audited
raw trajectory and used decision. The complete raw report and traces remain in
`/Users/jason/data/texas-42/sunshine-gym-replay-v1`; the summary pins that report's
hash. `publish.py` checks the raw artifact and interruption hashes before
producing these evidence files.
