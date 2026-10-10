# Fixed-budget tiny-net follow-through

Exploratory local experiment, 4 October 2026. The controls did not establish an
improvement from late-only training, four times more samples on the same roots,
or doubling width. The preregistered validation gate failed, so another rung was
not run. Some decisions survive distillation: mixed32 beats uniform random on
the fresh late panel, but retains much less useful information than its lawful
search teacher. This is neither a gameplay-strength result nor a disproof of
the ladder idea.

The preceding actual pilot remains unchanged at commit
`04cf0de2b2b56446105cce7db5659305b8378b58`; its
[report](../tiny-net-ladder-20261004/REPORT.md), original Fable sources,
accepted/contradicted history and private artifacts are preserved. That pilot
trained N0, froze it for all-actor T1 search and trained N1. N0 improved on random
under T0, while the first rung's N1-minus-N0 T1 regret difference was -0.001308
with 95% interval [-0.005984, 0.003683]. Ladder improvement was inconclusive.
Issue100 remains a related idea, not this experiment.

## Teacher, observation and targets

The copied native kernel is byte-identical to the reviewed original. Each actor
observes its original own seven-tile hand, public ordered plays, seat, bidder,
fixed bid, trump and public scores/leader/turn inferred by the referee. Models
receive the same raw 862-component information-set encoding, with no hidden
hands, source identity, deal seed or reference targets.

For each root, the sampler is uniform over remaining tile allocations consistent
with public legality constraints and remaining hand capacities. It omits the
likelihood of earlier action choices and bidding. This is a support-conditioned
surrogate belief, not a true gameplay posterior. Source histories are generated
off-policy by uniform play. The frozen continuation P-1 chooses uniformly from
legal actions for **every** future actor, including the root actor's future
turns. Each actor's legality uses its own sampled hand and public history.
There is no full-information maximizing continuation.

T0 evaluates every legal root action with the same sampled worlds and absolute-ply
random tapes. Q is the acting partnership's probability of winning the fixed
30-point contract under P-1; defenders win when the declarer's partnership fails.
Ascending tile ID resolves exact ties. Training regresses four times the vector
of Q minus its legal-action mean. Outputs are advantages, not calibrated win
probabilities. Squared-error conditional expectations depend on this training
distribution; no correct-belief or policy-improvement theorem is claimed.

## Frozen design and source separation

[plan.json](plan.json) was fixed before labels. An initial initializer failed
before labels because source 80 lacked three distinct live late roots within
16 histories. The exact [initial plan](initial-plan.json), failed receipt and
[initial code](history/coverage-initial.py) are retained. The pre-label repair
kept the intended 2,048 unconditional fresh source count and exactly 16 histories
per deal, separating the source RNG from the trajectory RNG before labels. The
initial failed generator shared one RNG and reached only source 80, so this
amendment changed the deal sequence after source zero. The repaired generator
then allocated 3,648 roots by deterministic round-robin quotas bounded
by available live late roots. There was no resampling of unavailable deals or
performance-based expansion.

Train/validation/test source counts are 1,536/256/256. All related histories,
root completions and seat symmetries stay with their source partition; sorted
full-hand partition hashes prevent source reuse across splits, the original
pilot and prior phone fixtures. All 8,430 retained actor-normalized inputs are
unique and disjoint from the original pilot. Fourteen training sources have no
eligible late root. Both main arms use exactly the same 1,522 eligible sources
and per-source quotas. Mixed roots have 1,522 early, 1,216 mid and 910 late roots;
late32 uses 3,648 late roots. Early/mid/late are absolute plies 0..7/8..15/16..23.
Only live, nonforced roots enter the panel.

Availability is label-independent but not outcome-independent: live-contract
survival depends on public scores. Results are conditional on this availability.
Validation has 512 late roots from all 256 deals. Fresh test has 508 late roots
from 254 eligible deals, retaining the two unavailable deals in the original
256-source manifest. The all-band test has 1,020 roots from all 256 deals.

| Frozen control | Train roots | Worlds/root | Hidden width | Maximum updates | Selected updates |
|---|---:|---:|---:|---:|---:|
| mixed32 | 3,648 matched-source mixed | 128 | 32 | 1,500 | 30 |
| late32 | 3,648 late | 128 | 32 | 1,500 | 30 |
| late64 | Same late roots/targets | 128 | 64 | 1,500 | 30 |
| subset128 | 912 fixed late roots | 128 | 32 | 400 | 8 |
| subset512 | Same 912 roots | 512 | 32 | 400 | 8 |

All share deterministic initialization, Adam, learning rate .002, batches of 256,
100 epochs and the same 512-root late-only validation MSE checkpoint selector.
Every selected checkpoint is epoch two. Overlapping labels are identical;
512-world samples extend the same first 128 worlds/tapes. Its validation still
uses 128 worlds, correctly distinguished in each record from padded storage.

The separate [adaptive optimizer diagnostic](adaptive-control-plan.json) was
declared after validation and before opening test: the same two subset models
trained for 375 epochs, permitting 1,500 updates without new data or labels.
Both selected epoch two/eight updates and are byte-identical to their respective
100-epoch checkpoints. Equal maximum budgets did not equalize selected exposure.
This diagnostic did not replace the failed primary gate. All seven weights were
frozen before test-reference generation; the successful independent chronology
audit and [captured timestamps](results/chronology.json) retain local evidence.
Filesystem mtimes reset after checkout; frozen hashes remain portable.

## Decision results

Every held-out reference evaluates all actions with 4,096 fresh paired worlds
and tapes, separate from label seeds. Regret is `max_a Q_reference(a) -
Q_reference(chosen)`, measured in acting-partnership win-probability units under
the stated teacher/belief. Means average within each original source deal first;
95% intervals bootstrap complete source deals. The finite-sample max reference
is noisy. These are teacher-relative decision metrics, not optimal-game regret.

| Control/baseline | Late validation regret | Fresh late test regret |
|---|---:|---:|
| Uniform random | .052244 | .058210 |
| Ascending tile | .051422 | .064920 |
| Highest tile | .048044 | .049653 |
| T0 search, 128 worlds | .001201 | .001644 |
| mixed32 | .045202 | .043524 |
| late32 | .044378 | .050478 |
| late64 | .046750 | .044973 |
| subset128 | .055670 | .049115 |
| subset512 | .056012 | .049665 |
| Frozen original N0 | .038453 | .045062 |

The **primary** gate required at least 10% lower validation regret for late32
than mixed32, a negative upper paired 95% difference bound, and resolved gains
over random and ascending. Late32 improved numerically by only 1.824%; its
difference was -.000824, interval [-.007076, .005022]. Gate **failed**.
Capacity/precision diagnostics cannot replace it; no new frozen-policy rung ran.

| Fresh late-test comparison | Regret difference | Paired 95% interval |
|---|---:|---:|
| late32 minus mixed32 | +.006954 | [-.003439, .017082] |
| late64 minus late32 | -.005505 | [-.017826, .006396] |
| subset512 minus subset128 | +.000550 | [-.001210, .002586] |
| late32 minus subset512 | +.000813 | [-.009298, .010772] |
| mixed32 minus original N0 | -.001538 | [-.011827, .008433] |

All of these intervals include zero. The validation coverage/precision tradeoff
favoring 3,648 low-sample late roots over 912 high-sample roots did not repeat on
fresh test. The comparison jointly changes source coverage and selected updates,
so even the validation result did not isolate coverage. Mixed32's late-test
difference from random is -.014686, interval [-.023496, -.005992], a 25.2%
numerical regret reduction. It is not evidence of beating the original N0 or
the highest-tile heuristic. These exploratory intervals are not multiplicity
corrected.

On the all-band test, mixed32/late32/late64/original N0 regret is
.040521/.045437/.040834/.040669. Late32-minus-mixed32 is +.004916,
interval [-.000464, .010182]; mixed32-minus-original N0 is -.000148,
interval [-.006273, .005584]. No new-model improvement is established there.

Action accuracy alone would obscure the loss. On 508 late-test roots, teacher
best-set agreement is 90.9%, mixed32 67.5%, late32 64.2% and late64 68.1%.
Only 226 roots pass the empirical paired `gap > 3*SE` heuristic. Within that
subset teacher/mixed32/late32/late64 agreement is 92.9%/46.9%/40.3%/50.0%.
This gap filter is a descriptive heuristic without multiplicity correction;
the actual regrets and disagreements remain the principal evidence.

## Noise, fit and capacity diagnosis

On the exact same 912 training roots, estimated unscaled centered-target sample
variance falls from .00024890 at 128 worlds to .00006225 at 512. Observed centered
target difference RMSE is .012896 and greedy teacher labels disagree on 6.69%
of roots. These paired targets share their 128 prefix, so this is label sensitivity,
not an independent truth estimate. Improved precision did not produce a resolved
decision gain from either selected net.

Late32's selected validation scaled advantage MSE is .147749. By epoch 100 its
training loss is .003344 while validation MSE rises to .228442. Late64 also fits
training cheaply, with final training loss .004199 and validation MSE .198673;
its best held-out checkpoint remains epoch two. Training loss includes L2 and
minibatch averaging, unlike unregularized validation MSE, so they are not exact
matching estimands. The curves still show poor generalization despite fitting
training labels; they do not establish simple undercapacity. More epochs and
doubling width did not solve the held-out decision problem. Nor do these controls
prove that label noise is irrelevant or determine the best representation/loss.

## Actual local compute and inference

The previous real hardware probe verified Apple M5 Max, 48 GiB unified memory,
MLX .31.2 Metal GPU and NumPy2.4.4 in the local Python3.12 runtime. All seven
new trainings actually succeeded on that GPU; no package installation, cloud,
paid services, credential creation or external transmission occurred.

| Dataset | Train root-world samples | Train all-action continuations | All rows generation worker-seconds |
|---|---:|---:|---:|
| mixed128 | 466,944 | 1,739,136 | 3.282 |
| late128 | 466,944 | 1,188,096 | 2.391 |
| subset128 | 116,736 | 297,600 | .680 |
| subset512 | 466,944 | 1,190,400 | .872 |

Generation times include exact sampling, rollouts, encoding, packing and writing
plus each arm's common validation panel, but exclude process startup. They are
summed worker wall times with at most two concurrent label workers, not total
elapsed or CPU time. Main root-world budgets are equal; action counts, horizons
and actual work are not. All-row mixed/late throughput is about162k/223k
root-world samples per summed worker-second. The independent4,096-world reference
uses8,372,224 root-world samples/27,746,304 all-action continuations and11.736
summed worker-seconds. Mining the fixed16-history pools took35.827 seconds.

MLX loop times for mixed32/late32/late64 are .641/.640/1.173 seconds; the full
five-model trainer campaign completed in 4.417 seconds including child startup.
Each 32-wide model has 28,540 parameters,114,160 raw float32 bytes and115,126 NPZ
bytes. Width 64 has 57,052 parameters and 228,208 raw bytes. Seven saved files include
two duplicate adaptive checkpoints, not seven distinct learned policies.

Sequential warm CPU benchmarks on 1,024 validation decisions give mixed32
network median 3.958 microseconds and validated replay+encoding+legal argmax
median 89.375 microseconds (p95 106.036). Late32 is 3.959/89.500 microseconds;
late64 is 3.917/89.063. This microbenchmark does not resolve width-dependent
speed differences and excludes loading/startup. Earlier concurrent benchmark
receipts remain preserved but are superseded by isolated reports. No linear
label-cost or ladder improvement claim follows.

Every executable was under an enforced process-group watchdog with allowance
<=295 seconds, game children 120 seconds, and no invocation over 300 seconds. The
cost snapshot includes 139 completed/failed receipts (138 completed, one failed
pre-label initializer); later diagnostics/review add receipts. A final packaging
check also initially failed on a manifest base-path error; its traceback is
retained and the path was corrected without rerunning models or labels. Largest
observed invocation was 35.827 seconds. Nested parent/child receipts overlap and cannot be
summed as total run time. No jobs remain running at delivery.

## Pinned-phone hybrid smoke

[phone-plan.json](phone-plan.json) fixed a post-test smoke with two further fresh
deals, four rotations and both candidate partnership assignments for teacher0,
mixed32 and late32:48 complete games. Candidate overrides apply only at live,
nonforced absolute plies16..23. Every other turn uses the same pinned production
phone profile, including early/mid, forced, settled and after23. The phone WASM
hash is `40d7a1eea658627f7bab37d3c1888ad3bf00216fb23a5d2ceeb7d2d1f4219119`,
Plunge `a0d9fa806166b0e63fe016bb49d93f91f47b1af8`, Rust
`cb1ef3b23072e4c268f31f625f2b61d5facc1929`.

All three variants had 0 paired wins/0 losses/8 ties. There were 1,344 valid moves,
125 overrides (41 teacher/42 mixed/42 late),1,219 phone calls and no interruptions.
This is only two independent source clusters, with conservative interval [-1,1];
ties do not establish strength equivalence. Eligibility and route counts depend
on each variant's trajectories. Full phone startup/cost remains in the hybrid,
so timings are not equal-cost or pure-inference speed comparisons.

## Independent review and next step

The explicitly requested independent Sol6.1 collaborator verified teacher
identity/lawfulness inherited from the first pilot, all source partitions and
retained legal roots, deterministic quotas, normalized-input separation, all
stored paired labels, sampled 128/512 world/tape prefixes, frozen weights,
fresh-reference metric arithmetic and all phone turns/overrides. Its own 10,000
source-bootstrap intervals also cross zero for every causal control comparison.
See [checks/REVIEW.md](checks/REVIEW.md) and capped audit receipts. No new
mathematical guarantee was proposed, so no convergence theorem or Lean claim is
used to justify these empirical conclusions.

The next useful experiment is a separately preregistered fixed-budget test of
symmetry handling and decision-aware objectives on the same raw information
boundary, with multiple initialization seeds and a new source-deal test. It
should aim to retain the teacher's resolved action gaps before investing in
another rung. This is a recommendation, not an automatically launched expansion.
Local work is complete. Private fixture export still requires the separately
pending user approval and was not attempted; production and task10 were untouched.
