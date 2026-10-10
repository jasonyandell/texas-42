# What blocks tiny-net distillation: representation first, coverage second, capacity last

**Exploratory local experiment, 2026-10-04.** Owner: Claude Fable 5.1 (runtime
model ID `claude-fable-5-1`), via local Claude Code under Jason's credentials, on
request of Jeb. Worktree `task-15/fable-worktree`, branch
`experiment/fable-tiny-net-ladder`, base `461028ed`. The two prior experiments
(`../tiny-net-ladder-20261004`, `../tiny-net-coverage-20261004`) were read in
full and are unmodified; the pilot's teacher, surrogate belief, raw encoding and
label kernel are inherited byte-identically. Nothing here is a strength claim, a
learning theorem, or a status change at any tier; it is a bounded diagnosis with
one fresh, preregistered test opening.

## Answer in one paragraph

Capacity is not what blocked the ladder. On a fresh 512-deal test the tiny
28,540-parameter raw-encoding net retains 26% of the random-to-teacher regret
reduction at the prior data scale, and regular-sized nets of 228k and 294k
parameters retain 20% and 24% on the same data (paired intervals span zero).
Source-deal coverage is a real but secondary blocker: 16x more fresh deals
(labels cost 40 summed worker-seconds) lifts the tiny net to 42%. The dominant
blocker is **representation**: a 6,337-parameter per-action scorer over 32 lawful
hand-crafted features retains 77% at the prior data scale and 88% at 16x
(regret .0095 versus random .0543 and teacher .0032), with held-out R² .63
against about .07 for every raw net. A third, smaller factor is the
**selection objective**: validation-MSE early stopping picks raw checkpoints
hundreds of updates before decision regret stops improving; regret-selected
regular-sized raw nets reach 54 to 56% at 16x, while regret selection does nothing
for the tiny raw net. Label noise is not a factor at this fidelity (128-world
sampling floor .004 against a .149 zero-predictor MSE). No frozen-policy rung was
run, by plan.

## Pre-run diagnosis on committed artifacts

Before any new run, the committed coverage-experiment validation panel (512 late
roots, 4,096-world reference) was re-read. Scaled centered-advantage MSE of the
zero predictor is .1491; the committed mixed32, late32 and late64 nets reach
.1476, .1477 and .1485; the 128-world sampling floor is about .004. Held-out R² is
therefore about 1% for every prior net while training loss falls below .01. That
is a generalization failure at 3,648 roots, not a fit failure, which is why the
prior width-32-to-64 and late-only controls could not move: they stayed inside
the same data and representation regime. This motivated a learning curve in
fresh source deals crossed with capacity and with one representation change.

## Frozen design

[plan.json](plan.json) was written before any label (commit `53d21479`), with
two recorded amendments, both before freeze and before any test root existed.

- **Sources.** 24,576 fresh train deals and 512 fresh test deals drawn from one
  RNG, excluding all 4,100 prior source partitions (both experiments plus every
  phone-game deal); zero collisions. The test deals were drawn up front but mined
  and labeled only after freeze. Eight uniform-legal histories per deal; at most
  one live, nonforced root per absolute-ply band 0-7/8-15/16-23 per deal, chosen
  by the per-deal RNG. 72,909 train roots (24,576/24,566/23,767 per band) and
  1,520 test roots (512/512/496); zero duplicate information sets within or across
  splits or against both prior experiments ([audit](checks/audit-result.json)).
- **Validation.** The committed coverage experiment's validation panel: 1,024
  roots over 256 deals, all bands, 4,096-world reference and teacher128 labels.
  Used for checkpoint selection and validation reporting only.
- **Data arms** (nested): d1 = first 1,536 deals (4,545 roots, the prior scale),
  d4 = 6,144 deals (18,208 roots), d16 = 24,576 deals (72,909 roots). Labels:
  T0 at 128 worlds, fresh seed domains.
- **Architectures.** raw-h32 862→32→28 (28,540 parameters, the prior net);
  raw-h256 862→256→28 (228,124); raw-d256 862→256→256→28 (293,916); feat-s64, a
  shared MLP 32→64→64→1 applied per legal action (6,337). The 32 features are
  functions of the request only (own hand, public plays, trump, bidder, leader,
  public scores): trump/double/count/pips, rank in context, follows/leading, wins
  the current trick, unseen tiles that beat it, unseen and own followers, master
  flag, players after, partner played/winning/after, opponent winning, table
  points, partner and opponent revealed voids in context, unseen and own trumps,
  team bidding, scores, tricks remaining, hand size, points needed, unseen count,
  legal count. No hidden hands, source identity or labels.
- **Optimizer, matched.** Adam lr .002, L2 1e-5, batch 256, 6,000 updates for
  every arm, deterministic init, centered masked advantage MSE with the factor-4
  target scale, exactly as the pilot. Selection = minimum validation MSE against
  the 4,096-world reference on a 10-update grid (A1: the frozen 50-update grid
  made every d1 arm select its first evaluation; the three coarse runs are kept
  in `results/pre-amendment-eval50/`). Secondary: raw-h256-reg-d1 (lr .0005,
  L2 1e-3) as a regularization check.
- **A2 (post hoc on validation, pre-freeze).** Validation curves showed MSE
  minima within a few hundred updates while validation regret kept falling for
  thousands on the larger raw nets. Twelve arms were re-run with `--also-regret`,
  saving the minimum-validation-regret checkpoint as `rs-<arm>.regret`. The
  MSE-selected weights of all twelve re-runs are byte-identical to the frozen
  arms (determinism receipt). These regret-selected arms are optimistic on
  validation by construction and are labeled secondary; the fresh test is their
  only unbiased read.
- **Freeze and test.** All 25 weight files were hashed into
  [FROZEN.json](models/FROZEN.json) at 16:48:43Z; the test block was mined,
  deduplicated, labeled (teacher128 and a 4,096-world reference with separate
  seeds) and featurized only afterwards, in one sequential chain. Pre-freeze
  state is also committed as `5b8217c7`.

## Fresh test, all bands (1,520 roots, 512 deals, 794 resolved-gap roots)

Regret is `max_a Q_ref(a) − Q_ref(chosen)` in acting-partnership contract-win
probability under the stated surrogate teacher; means are equal-weight per
source deal; intervals are 5,000-draw source-deal bootstraps. "Retained" is the
fraction of the random-to-teacher reduction kept. Not multiplicity corrected.

| Decision rule | Regret [95%] | Retained | Best-set agree | Resolved agree | Held-out R² |
|---|---:|---:|---:|---:|---:|
| Uniform random | .05429 [.05042, .05825] | 0 | | | |
| Highest legal tile | .04976 | .08 | .457 | .377 | |
| T0 teacher, 128 worlds | **.00320** [.00271, .00371] | 1.00 | .800 | .896 | |
| Prior committed mixed32 | .04458 [.03936, .04998] | .19 | .539 | .474 | .006 |
| Prior committed N0 | .04393 | .20 | .546 | .494 | .006 |
| raw-h32 d1 / d4 / d16 | .04113 / .03869 / **.03300** | .26 / .31 / .42 | .558 / .573 / .598 | .507 / .526 / .581 | .016 / .036 / .068 |
| raw-h256 d1 / d4 / d16 | .04427 / .03629 / .03130 | .20 / .35 / .45 | .532 / .579 / .598 | .484 / .546 / .573 | .005 / .030 / .065 |
| raw-d256 d1 / d4 / d16 | .04186 / .04127 / .03392 | .24 / .26 / .40 | .553 / .553 / .579 | .501 / .489 / .537 | .021 / .043 / .075 |
| raw-h256-reg d1 | .04061 | .27 | .557 | .506 | .033 |
| **feat-s64 d1 / d4 / d16** | **.01489 / .01120 / .00946** | **.77 / .84 / .88** | .689 / .701 / .728 | .717 / .744 / .780 | .448 / .528 / .630 |
| A2 regret-selected raw-h32 d16 | .03291 | .42 | .600 | .593 | −.094 |
| A2 regret-selected raw-h256 d16 | .02579 [.02212, .02973] | .56 | .618 | .604 | −.000 |
| A2 regret-selected raw-d256 d16 | .02660 | .54 | .628 | .622 | .085 |
| A2 regret-selected feat-s64 d16 | **.00784** [.00658, .00920] | .91 | .735 | .795 | .637 |

Preregistered paired comparisons (positive means the first is worse):

| Comparison | Difference [95%] | Reading |
|---|---:|---|
| raw-h32 d1 minus d16 | +.00813 [+.00330, +.01301] | H_coverage confirmed |
| raw-h32 d1 minus d4 | +.00244 [−.00144, +.00642] | 4x alone not resolved |
| raw-h32 d1 minus raw-h256 d1 | −.00314 [−.00770, +.00123] | H_capacity: bigger not better |
| raw-h32 d1 minus raw-d256 d1 | −.00073 [−.00586, +.00438] | same |
| raw-h32 d16 minus raw-h256 d16 | +.00169 [−.00266, +.00584] | capacity x data, MSE-selected: null |
| raw-h32 d16 minus raw-d256 d16 | −.00093 [−.00553, +.00362] | null |
| raw-h32 d1 minus feat-s64 d1 | +.02624 [+.02115, +.03120] | H_representation confirmed |
| raw-d256 d16 minus feat-s64 d16 | +.02446 [+.01988, +.02923] | features beat the largest raw net at 16x |
| raw-h32 d1 minus raw-h256-reg d1 | +.00052 [−.00423, +.00518] | regularization check: null |
| feat-s64 d1 minus d16 | +.00543 [+.00260, +.00851] | features also gain from coverage |
| raw-h32 d16 minus A2 raw-h256 d16 | +.00721 [+.00258, +.01182] | capacity helps only with data and regret selection |
| raw-h32 d16 minus A2 raw-h32 d16 | +.00009 [−.00435, +.00451] | regret selection does nothing for the tiny raw net |
| feat-s64 d16 minus A2 feat-s64 d16 | +.00162 [+.00018, +.00330] | small, resolved |
| raw-h32 d16 minus prior mixed32 | −.01159 [−.01663, −.00683] | fresh-deal d16 tiny net beats the committed net |
| feat-s64 d16 minus prior mixed32 | −.03512 [−.04066, −.02979] | |

## Fresh test, late band only (496 roots, 496 deals, 221 resolved)

Random .05544, highest tile .04537, teacher .00147 [.00097, .00202]. The tiny raw
net does **not** gain from data in late positions: d1/d4/d16 regret .03978/
.04271/.04415 (retained .29/.24/.21; d1 minus d16 −.00437 [−.01449, +.00544]).
The larger raw nets gain a little (raw-h256 d16 .03655, raw-d256 d16 .03645,
retained .35). feat-s64 d1/d4/d16 reach .02108/.01293/.01205 (retained .64/.79/
.80; raw-d256 d16 minus feat-s64 d16 +.02440 [+.01535, +.03462]); the A2
regret-selected feat-s64 d16 reaches .00804 (retained .88). The committed mixed32
and N0 retain .10 on this band. Late positions are exactly where the prior
report found the nets weakest; the feature representation removes most of that
weakness, the raw encoding at any tested size does not.

Validation results (1,024 roots, 256 deals) are in
[validation-all](results/validation-all/summary.json) and agree in ordering:
random .0515, teacher .0028, raw-h32 d1/d4/d16 .0403/.0314/.0302, feat-s64
.0125/.0106/.0104, mixed32 .0436, N0 .0369.

## Reading: which factor blocks what

- **Capacity.** At the prior data scale, 8x and 10x more parameters do not
  help and numerically hurt; selected checkpoints come after 20 to 60 updates
  and validation MSE rises from there. With 16x deals and the preregistered MSE
  selector the three raw sizes are indistinguishable. Only with 16x deals *and*
  regret-based selection do the regular-sized raw nets pull ahead of the tiny
  one (56% versus 42%), and even then they are far below the 6,337-parameter
  feature scorer. Capacity is the last factor, not the first.
- **Coverage.** Fresh source deals help the tiny raw net (26% to 42%) and the
  feature net (77% to 88%); 4x alone is not resolved for the raw net. Labels are
  cheap enough that this axis costs seconds, not minutes.
- **Representation.** The raw 862-wide one-hot encoding forces the net to learn
  trick-taking, following, voids and counts from a few thousand roots. Thirty-two
  lawful features that state those facts explicitly move held-out R² from .02 to
  .45 at the same 4,545 roots and raise resolved-gap agreement from 51% to 72%.
  This is a representation prior, hand-built from the rules; it says the
  encoding was the blocker, not which minimal feature set suffices.
- **Selection objective.** Advantage MSE and decision regret decouple on the
  raw nets: the regret-best checkpoints have negative held-out R² yet better
  argmax. Early stopping on MSE discards most of what the bigger raw nets learn.
  On the feature net the two objectives mostly agree.
- **Label noise.** Not retested, bounded: sampling floor .004 scaled against a
  zero-predictor .149; the prior same-root 512-world control was null; feat-s64
  reaches R² .63 on 128-world labels.
- **Optimization.** Matched 6,000-update budgets; the only budget-limited
  selections are feat-s64 d16 (update 5,550) and two A2 copies, so the feature
  arm is still improving at the budget edge. Lower learning rate with stronger
  L2 at d1 did nothing for raw-h256.

## Teacher objective and belief, assessed rather than endorsed

The teacher T0 is the all-action contract-win probability under uniform random
legal play by every future actor, including the acting seat, averaged over
hidden completions drawn **uniformly over the capacity-compatible support** of
own hand and public legality. That belief is a support-conditioned surrogate:
it ignores the likelihood of the observed history under the generating policy
(even under uniform play, a history's probability varies across worlds through
each hidden actor's legal-set sizes) and ignores bidding. It is not the Bayesian
posterior of any gameplay, and nothing here changes that. Both labels and the
independent reference use the same surrogate and the same uniform continuation,
so every regret in this report is **distillation fidelity to a weak,
self-consistent teacher**, not strength and not optimal-game regret. A net that
matches T0 plays a one-step best response to random opponents under a biased
belief; the earlier phone-hybrid ties are consistent with that. The ladder's
premise (iterate T_k under frozen P_{k−1}) carries no improvement guarantee in
this imperfect-information setting, and iterating sharpens the belief mismatch
because P_k's own history likelihoods are ignored by the sampler. What this
experiment establishes is narrower and prior to that question: with the raw
encoding the rungs could not even copy their teacher, so rung-to-rung deltas
were dominated by distillation loss; with a structured representation the
fidelity gap closes enough that a rung comparison would measure the teacher
rather than the student.

## Cost, size and latency

| Work | Observed | Qualification |
|---|---:|---|
| Mining 12 train blocks (16,384 histories each) | 200.1 s summed, ≤ 17.1 s per capped run | pure Python; test block 4.0 s |
| T0 labels, 72,909 train roots, 128 worlds | 39.6 summed worker-s; 9.33M root-worlds; 33.7M continuations | two workers max; includes encoding, packing, hashing |
| Test teacher128 / reference 4,096 | 0.85 s / 10.0 s summed | 6.2M root-worlds, 22.4M continuations for the reference |
| Features, train / test | 9.0 s / 0.19 s | Python featurizer |
| Training loop per arm | 2.7 s (raw-h32) to 6.6 s (feat-s64) for 6,000 updates and 600 evaluations | MLX Metal; load ≤ 0.5 s; 25 arms ≈ 100 s total |

Warm sequential CPU NumPy latency over 1,024 single decisions, excluding
loading and startup ([results/latency-*.json](results/)):

| Model | Parameters / raw bytes | Network-only median µs | Validated decision median µs (p95) |
|---|---:|---:|---:|
| raw-h32 d16 | 28,540 / 114,160 | 4.33 | 89.7 (103.8) |
| raw-h256 d16 | 228,124 / 912,496 | 6.00 | 90.4 (105.8) |
| raw-d256 d16 | 293,916 / 1,175,664 | 8.62 | 91.6 (106.2) |
| feat-s64 d16 | 6,337 / 25,348 | 8.67 | 128.6 (151.7) |

The feature net is the smallest file by 4.5x; its decision time is dominated by
the Python featurizer (about 40 µs over the raw encoder), and its network-only
figure reflects NumPy call overhead on a 28×32 by 32×64 shape, not arithmetic.
None of these are phone or scaling benchmarks.

## Receipts and process

All 91 `run.json` receipts have allowance ≤ 295 s; the largest actual run was
17.09 s. 89 completed; two failed receipts (a shell word-splitting error and a
stale empty output folder) are preserved under `results/failed/`. Twelve earlier
misnamed failed receipts from the same word-splitting mistake were deleted before
the cause was understood; that deletion is recorded here. No background jobs
were left running. The [audit](checks/audit.py) re-verified, under its own
capped receipt: source and information-set disjointness against both prior
experiments and across splits, receipt caps, all 25 frozen hashes, freeze time
preceding every test receipt (the freeze stamp has one-second resolution; the
sequential chain and commit `5b8217c7` are the stronger chronology), byte-identical
replay of three test reference roots from the kernel, feature recomputation on
16 test roots, packed outcomes reproducing every Q, and one headline mean.
Metal was reachable from this sandbox under the capped runner. No package
installs, cloud, paid resources, credentials, transmissions, production changes
or fixture exports occurred; the unrelated exact synthetic fixture export was
not attempted.

## Limitations

- Teacher-relative, surrogate-belief, T0-only fidelity; no rung, no games, no
  strength statement. The regret ceiling is the reference maximum's own noise.
- feat-s64 encodes rules knowledge by hand. It demonstrates that the raw
  encoding is the blocker; it does not identify a minimal feature set, test a
  learned structured encoding, or bound what features cannot express.
- The d1 arm reconstructs the prior data scale (1,536 deals, one root per band)
  rather than reproducing mixed32's exact 3,648-root quota mix; the committed
  mixed32 and N0 are evaluated directly on the fresh test for that comparison.
- A2 arms were chosen after reading validation; their test numbers are unbiased
  for the fixed weights, but the decision to report regret selection was not
  preregistered. Many comparisons, no multiplicity correction.
- Late band has 496 roots and wide intervals; the tiny raw net's late-band
  null under data scaling is a point estimate with an interval spanning zero.
- feat-s64 d16 is budget-limited; its ceiling under more updates or more deals
  is unmeasured. Feature latency is a Python artifact.

## Bounded hypothesis and what I would do next

**Hypothesis (bounded):** a lawful structured representation at d16 scale
retains at least 85% of T0's resolved decision value in a model under 10k
parameters; the raw one-hot encoding cannot exceed about 60% at any size tested
under any selection rule here. This is exactly what the test shows, and the
next falsifier is a rung, not another control. The rung requires porting the
per-action feature scorer into `kernel.c`'s continuation policy (it currently
supports only dense raw nets), then T1 under frozen feat-s64 with a fresh
reference, keeping the phone comparison for after a candidate survives. That is
Jason's call; it was not started.

## Model identities

All 25 frozen weight files are hashed in [FROZEN.json](models/FROZEN.json); the
committed comparators are `mixed32` (`2215a881b5c9`) and `n0-32`
(`70c37e26d80b`). Headline arms:

| Arm | SHA-256 prefix | Selected update |
|---|---|---:|
| raw-h32-d1 | `af426eb6a7cb` | 60 |
| raw-h32-d16 | `addf323cfd5e` | 520 |
| raw-h256-d16 | `c04c7a449bb7` | 240 |
| raw-d256-d16 | `4af8ace49c2d` | 250 |
| feat-s64-d1 | `30b17428319b` | 600 |
| feat-s64-d16 | `b0510478b614` | 5550 |
| rs-raw-h256-d16.regret | `57f1c27a7744` | 5760 |
| rs-feat-s64-d16.regret | `99d539522154` | 5420 |

## Session link

No claude.ai session URL was generated by this runtime. The environment exposes
only local identifiers (Claude Code 2.1.280, entrypoint `sdk-cli`, session ID
`806d702b-7f6e-4451-8289-53f4645ab580`, named "Jeb tiny-net ladder handoff to
Fable"); no web link appears in the session metadata or transcript, and none was
invented.
