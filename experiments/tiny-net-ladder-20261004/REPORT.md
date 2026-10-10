# Tiny action-vector net and one frozen-policy rung

**Exploratory local experiment, 2026-10-04.** A 28,540-parameter cross-hand
net retains useful decision signal at cheap inference. One search/redistillation
rung completed, but improvement is inconclusive and the phone-Walt smoke is
unfavorable. This is neither a competitive-player result nor a learning theorem.

The independent Sol6.1 [review](checks/REVIEW.md) accepts the lawful teacher,
whole-deal splits, paired-label arithmetic, frozen weights, independent-reference
metrics and phone replay. The [plan](plan.json), [provenance](PROVENANCE.json),
[code](pilot.py), [native rollout kernel](kernel.c), retained data, weights and
capped receipts reproduce the result. No production merge/deploy, cloud rental,
credential creation, external message or private-fixture export occurred. Task-10
was untouched. Work is isolated from base `92cc1ac3` in this local checkout.

## Recovery and preserved history

Original sources are retained, rather than rewritten: the original
[`rollout_net.py`](../astra-sol-20261004/phase2/incoming/rollout_net.py),
[`walt-sense.zip`](../astra-sol-20261004/phase2/incoming/walt-sense.zip), saved
[conversation provenance](../astra-sol-20261004/phase2/incoming/PROVENANCE.json),
the [Drive TINY-MODEL account](../astra-sol-20261004/phase2/incoming/drive/TINY-MODEL.md),
the [saved Drive packet](../adversarial-20261004/results/drive-refetch.json) and
the original [accepted](../adversarial-20261004/ACCEPTED.md) and
[contradicted](../adversarial-20261004/CONTRADICTED.md) histories. `PROVENANCE.json`
hashes recovered files and checks available canonical task-2 copies byte for byte.
Issue 100 remains a separate idea.

The historical account reported 79% outcome accuracy from 276k single-outcome
labels, near-random decision regret, an inadequate 8.7k policy-label fit and a
better high-sample paired control. Those later neural numbers remain historical
claims rather than freshly verified results. The independently recovered source
establishes 276,244 rows and that all 20,000 validation rows shared source games
with training. That diagnoses leakage, without quantifying its effect on accuracy.
The original uniform rollout teacher was lawful. Its archived `walt_decide`
comparator optimized own continuations separately inside hidden worlds and was
fused. Masking a student's inputs does not repair such a teacher.

The current task-9 native/WASM adapter at `92cc1ac3` and task-8 adapter at
`56794dc3` were inspected. Their modeled-level continuations are distinct from
this experiment's frozen uniform/net policies. They were not silently reused as
teacher labels. Actual pinned production phone WASM was used for the game smoke:
Plunge `a0d9fa806166b0e63fe016bb49d93f91f47b1af8`, Rust
`cb1ef3b23072e4c268f31f625f2b61d5facc1929`, WASM SHA256
`40d7a1eea658627f7bab37d3c1888ad3bf00216fb23a5d2ceeb7d2d1f4219119`.

## Precisely defined task and teacher

Scope is straight 42, pip trump 0–6, fixed bid 30. Source contracts choose trump
and bidder independently of the deal; bidding is not modeled. Source histories
come from uniform legal play, ending at contract settlement. Up to one live,
nonforced position is chosen in each absolute-ply band 0–7, 8–15 and 16–23.

An actor observes only its own current/original hand, ordered public plays,
contract, leader and publicly derived scores. The model receives 862 raw inputs:
one-hot tile states (own-current/unseen/played by relative seat and trick), trump,
relative bidder/leader, table offset, scores and bid. Relative-seat tile tags plus
contract/leader preserve public chronology in this rules scope. Neither source
deal identity, hidden hands, RNG seed nor label outcomes enter the model.

The belief used for every root is **uniform over capacity-compatible hidden
completions satisfying own hand and legal public history**. The DP sampler ignores
bidding and historical action likelihoods. This is an explicitly declared
support-conditioned surrogate; it is generally not the Bayesian posterior of
the observed gameplay policy. In particular, uniform legal play has
world-dependent history likelihoods, and frozen-net gameplay has different
likelihoods. No automatic correct-beliefs claim is made.

Rung indices are:

- P−1: every actor, including the root seat's future self, plays uniform legal.
- T0: evaluate every legal root action under frozen P−1; payoff is the acting
  partnership's contract win (bidder makes, or defender prevents the bid).
- N0: regress T0's legal action advantages; P0 is N0's legal argmax.
- T1: evaluate every legal root action under **frozen P0 for all future actors**.
- N1: regress T1's legal action advantages; same architecture and root distribution.

There is no hidden-world maximizing continuation. C net decisions are functions
of each actor's own information. Exact ties choose the ascending physical tile ID.
Every root's legal candidates share the same sampled worlds and absolute-future-
ply tapes. One tape is used per world. Finite-grid uniform draws use NumPy
float64 tapes; this is numerical Monte Carlo rather than exact rational payoff.
T1 is deterministic given a sampled world; retained tapes still identify the
shared experiment. The target teacher/version is frozen within each dataset.

Training minimizes legal-centered action-advantage MSE, with equal loss weight
per root and an explicit factor-four target scale. Outputs are scores for legal
argmax, not calibrated P(make) probabilities. Raw floating weights are isolated
in this exploratory experiment; no exact engine or proof authority was changed.

## Splits, labels and training

There are 2,048 unique original source deals: 1,536 train, 256 validation, 256
test. The split is assigned before deriving histories and labels. The source ID
hashes the unordered collection of original hand masks, grouping rotations and
seat permutations of the same partition. All 4,860 actor-normalized inputs are
unique across splits: 3,647 train, 608 validation, 605 test. Sampled completions
remain simulations attached to their source root; they never become additional
training examples or independent source clusters. Teacher root belief supports
are not artificially pruned to exclude mathematically possible worlds.

T0 and T1 each label all 19,622 legal actions across 4,860 roots with 128 paired
worlds. Both independent references label 1,213 held-out roots with 4,096 fresh
worlds and distinct seeds/tapes. Packed per-world Bernoulli action outcomes,
root/source IDs, sample hashes and exact generation seeds are retained in
`data/*`. Action gap SE uses paired within-world differences. All bootstrap
intervals use equal-weight means per original source deal, not independent rows.
The empirical top-two gap > 3 paired SE subset is a heuristic: it is not a
multiplicity-adjusted simultaneous confidence statement.

Both nets use **862 → 32 ReLU → 28**, 28,540 float32 parameters, 114,160 raw bytes,
115,126 bytes per saved NPZ. N0 and N1 each ran 100 epochs on MLX Metal; validation
MSE selected epoch 4 in both. Test roots were excluded from training and selection.
The N0 weights were independently retrained and reproduced byte identically.
A linear model trained on exactly the same raw encoding/targets is retained.

The preregistered validation gate required a useful low-budget teacher and a
net that reduced random-action regret at least 20%, with negative upper95% paired
net-minus-random difference and mean regret no worse than ascending. N0 passed:
0.038984 versus random 0.057029, delta CI [−0.024000, −0.012552]. The teacher's
regret was 0.003216. This allowed exactly one next rung; it did not promise that
N1 would improve or that rungs cost linearly.

## Held-out decisions

These are **teacher-model decision regrets against the maximum sampled reference
Q**, not optimal-game regret. The reference maximum has sampling/selection noise.
Every entry below uses the same 256 independent test source deals and 605 roots.

| Decision rule | Under T0 reference | Under T1 reference |
|---|---:|---:|
| Uniform random legal action (expected) | 0.054643 | 0.068568 |
| Ascending legal tile | 0.056979 | 0.069316 |
| Highest legal tile | 0.055864 | 0.070243 |
| Corresponding 128-world search teacher | **0.004139** | **0.004307** |
| Linear T0 student | 0.044490 | 0.059056 |
| N0 tiny net | **0.040842** | 0.055056 |
| N1 redistilled tiny net | 0.043051 | **0.053747** |

N0 reduces the random baseline's T0 regret by about 25.3%, retaining 27.3%
of the available random-to-search-teacher reduction. Its paired delta versus
random is −0.013801, 95% [−0.020323, −0.007247]. N0 chooses a sampled reference
best-set action on 51.4% of T0 test roots (47.8% on the heuristic resolved subset);
the teacher reaches 76.7% (89.9% resolved). Decision error remains material:
some individual net choices lose a full contract-win probability under the model.

**Ladder improvement is inconclusive.** On the same T1 reference, N1-minus-N0
regret is −0.001308, owner-bootstrap 95% [−0.005984, 0.003683]; independent Sol
bootstrap [−0.006175, 0.003831]. On T0, N1 is numerically worse by +0.002209,
95% [−0.001867, 0.006891]. Comparing absolute regrets across different reference
continuations would be invalid. N0's improvement over the linear control is also
inconclusive (T0 delta −0.003649, CI [−0.010848, 0.003216]).

Authoritative outputs are [validation0-final](results/validation0-final/summary.json),
[validation1-final](results/validation1-final/summary.json),
[test0](results/test0/summary.json) and [test1](results/test1/summary.json).

## Phone-Walt game smoke and diagnostics

The teacher, N0 and N1 each played 16 complete games against the actual pinned
phone: two new independent deals, four rotations, both partnership assignments.
Phone used its real opening160-world/partnerfalse profile, later40-world/
partnertrue. All 1,344 moves, 672 phone responses, scores and final 42 points were
checked with the independent referee. No interruptions or invalid games occurred.
Forced turns and already settled contracts use ascending legal actions for the
tiny candidates. Native teacher queries resample from an independent public
policy seed; hidden source deal seeds never enter actor requests.

| Candidate | Paired wins / losses / ties | Mean paired make difference |
|---|---:|---:|
| T0 128-world teacher | 0 / 1 / 7 | −0.125 |
| N0 | 0 / 4 / 4 | −0.500 |
| N1 | 0 / 4 / 4 | −0.500 |

This is an unfavorable smoke, with only two independent source deals. Rotations
and role swaps are correlated; a conservative bounded95% interval is [−1,1]. No
strength estimate or fair cost-normalized competition is claimed. [Raw results](results/h2h/summary.json).

A targeted control generated 4,096 fresh paired samples on 96 existing training
roots, without training or tuning another model. Low-label centered-advantage
RMSE was 0.01903; teacher choice regret 0.00346 versus selected N0 0.01764. Extra
label samples alone do not explain the large fit gap. Training loss falls while
new-deal validation worsens, and late positions are especially weak: in T0's last
band N0 regret is 0.06286, N1 0.07770, random 0.07078, teacher 0.00131. These
point toward coverage/generalization/representation limitations; they do not
causally isolate a capacity failure. [Diagnostics](results/diagnostics.json).

## Observed compute and bounded costs

The host is **Apple M5 Max, 48 GiB unified memory**. Existing Python3.12.13,
NumPy2.4.4 and MLX0.31.2 were used. Sandbox Metal access failed, the authorized
30-second host probe succeeded, and actual local Metal training completed.
No package downloads or paid GPU services were used.

| Work | Observed seconds | Qualification |
|---|---:|---|
| T0 labels, all 4,860 roots | 3.437 summed worker wall | 1,414 roots/s; two workers max |
| T1 labels, all 4,860 roots | 6.866 summed worker wall | 708 roots/s; frozen N0 continuation |
| T0 independent references | 10.253 summed worker wall | 4,096 worlds/root |
| T1 independent references | 34.183 summed worker wall | 4,096 worlds/root |
| N0 training | 3.029 workload / 3.169 capped process wall | 100 epochs incl compilation |
| N1 training | 0.629 workload / 0.747 capped process wall | warm runtime/compiler caches |
| Linear control training | 0.724 capped process wall | same dataset |
| Phone game campaign | 11.836 process wall | two concurrent games, 48 games |

Generation times include sampling, rollout, hashes and compression; summed worker
wall is not CPU time or parallel campaign elapsed. Training timings include all
100 epochs, not just the selected epoch. The first 300-root batch took 0.458s.
All builds, labels, training, diagnostics and benchmarks have POSIX process-group
watchdogs ≤295 seconds, actual invocation ceilings below300 seconds. There are no
background jobs left. Failed probes/development invocations are preserved in
[CONTRADICTED.md](CONTRADICTED.md).

Warm CPU deployment measurements over 1,024 requests: N0 network-only median
3.583µs/p95 4.083µs; validated raw encoding/legal decision median157.896µs/p95
308.916µs. N1 is similar (3.542µs and157.188µs medians). Model loading/process
startup are excluded and explicitly separated from this warmed measurement.
This demonstrates cheap inference here, not a phone-device or scaling benchmark.

No new mathematical policy-improvement, outer-loss or learning-convergence
guarantee is asserted, so no new Lean theorem was required. Small MSE is not used
to bound game loss; paired gaps, actual surrogate decision regrets and game
outcomes are retained instead. Regression estimates are conditional on their
training distribution.

The next useful experiment is a **fixed-budget coverage control focused on late
positions**, reserving fresh source deals and comparing the same model/labels
before expanding rung count. It should separate unseen-state coverage from label
precision and simple representation capacity, and keep the actual phone smoke.
There is no evidence here to automatically train a longer ladder or scale labels.
