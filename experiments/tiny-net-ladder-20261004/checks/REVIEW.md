# Independent Sol6.1 review

Exploratory tier. Independent source, implementation, complete paired-data,
decision-metric, phone-smoke and diagnostic audit.

## Historical evidence preserved

The original `adversarial-20261004/sol/REPORT.md` sections 3–4 independently
identify a lawful uniform rollout teacher: every acting seat chooses uniformly
from its own legal hand. Simulation access to hidden hands alone is not fusion.
Its archived `walt_decide` comparator instead optimized own orderings separately
inside each hidden world. Masking the root input does not make that continuation
lawful. Historical regret against that comparator is not production-phone regret.

The original 276,244-row outcome dataset split rows across 14,000 games. All
20,000 validation rows had another position from the same game in training.
This establishes split leakage, not the size of any resulting accuracy inflation.
The historical 79% outcome accuracy and 8.7k policy-label failure do not establish
cross-hand paired action-value distillation performance.

## Required interpretation and checks

1. Each actor observes only own remaining/original hand and public history,
   contract and score. Every continuation action must be a fixed function of
   that information and documented random tape; test hidden-world invariance.
2. Uniform compatible completions is a declared support-based belief model.
   It is generally not the posterior of the observed gameplay policy: even
   uniform legal play has world-dependent history likelihoods. Bidding and
   action-likelihood omissions must remain explicit.
3. All legal root actions share worlds and tapes. Retain paired action outcomes
   or differences so uncertainty of decision gaps can be measured directly.
4. Split entire original source deals before deriving positions, symmetries or
   completions. Identify exact information-state duplicates across splits.
5. Freeze source version, continuation weights, tie rule and dataset manifest.
   High-budget evaluation uses fresh worlds/tapes, independent of labels.
6. Measure decision regret and action gaps, including a resolved-gap subset;
   outcome accuracy or regression error alone is insufficient. Use source-deal
   clusters for uncertainty, not independent position rows.
7. Establish the low-budget teacher's held-out advantage over simple baselines
   before claiming its decisions are useful; select rung gating using validation.
8. Measure model bytes, single-position warmed inference and complete training
   and label costs separately. Deployment speedup does not imply cheaper
   total training or a linear ladder cost.

No policy-improvement or correct-belief guarantee follows automatically from
squared-error regression. No mathematical learning theorem is claimed here.

## Native implementation checks

The stable first `kernel.c` version uses uniform legal continuation for every
future actor in T0; there is no maximization inside hidden worlds. T1 invokes
`netchoice` with only that actor's current hand and public record/contract/score.
Opponent hand masks are not passed to the network. Exact ascending ties are
implemented with a strict greater-than scan in ascending tile order. Root actor
utility correctly follows bidder parity rather than a hard-coded team number.
All candidate actions use identical world and absolute-ply tape coordinates.

`checks/results/independent-audit/run.json` records a successful 0.736-second
bounded run. Independent checks passed:

- 74,088 native legality/rank comparisons against the existing Python referee,
  across all seven supported pip trumps.
- 30,000 exact-DP sampler draws on an exhaustively enumerated 30-completion
  support; exact support matched, chi-square 33.412. This is a finite
  distribution smoke check, not a proof of general sampler uniformity.
- 8,636 native paired Bernoulli outcomes across 120 fresh roots exactly match
  Python gameplay on the same worlds/tapes, covering both bidder parities.
- 120 random-network native/Python encoded legal-choice comparisons match.

The capacity-DP recurrence counts each capacity-compatible tile assignment
once; weighted suffix counts select completions. Bounds fit the declared
seven-tile four-seat game (at most 21 unseen tiles; capacities at most seven).
Its callers derive void exclusions from public legal play. It implements the
documented support-conditioned surrogate, without history-likelihood weighting.

## Initial dataset audit

`checks/results/data-audit/run.json` records a successful 1.026-second bounded
structural audit of 2,048 unique original deals and 4,860 roots. Train,
validation and test contain 1,536/256/256 complete deals. Every retained history
replays legally in its original deal, and all exact information keys are unique.
No symmetry augmentation or completion-derived training roots is generated.

All 16 T0-label batches (4,860 roots) use one frozen kernel/binary/continuation
identity. Saved packed paired outcomes exactly reconstruct Q; encoding and
legal masks match independent recomputation. The initial two reference batches
(608 roots) passed the same audit. Later datasets require a fresh audit receipt.

## Metric implementation review

`evaluate.py` uses paired outcome differences for within-root gap standard
errors, with one tape per independently sampled world. Across roots it bootstraps
complete source deals, giving each retained source deal equal weight. Paired
regret differences cancel the sampled reference maximum's common optimism.
Absolute regret remains regret against a finite sampled surrogate reference,
not optimal lawful-game truth. The report states this limitation explicitly.

The selected top-two gap greater than three estimated standard errors defines a
useful diagnostic subset. It does not include multiple-comparison or
winner-selection correction and must not be described as simultaneous confidence
that every selected action is truly best. Validation controls epoch selection and
rung gating; test has no role in the implemented training selector.

## Frozen N0 and first gate

`checks/results/frozen-n0-audit/run.json` records a successful 1.336-second run
including every one of the 4,860 retained positions. The actual frozen N0
weights (`70c37e26d80bfb9f3cd393f62d07a9be74f5849f3731db7bfa8f5cb4cf57174c`)
give identical C/Python legal choices everywhere. Raw float32 numerical
evaluation is not a general exact-arithmetic guarantee, but this checks the
weights actually used by T1 on the complete retained root population.

The declared validation gate passes on 256 complete source deals (608 roots):
teacher128 reference regret 0.003216, expected uniform random 0.057029, and
N0-32 0.038984. N0-minus-random paired deal-bootstrap interval is approximately
[-0.0240,-0.0126]. These are decisions under the frozen uniform-continuation
surrogate, not production gameplay strength. Training selected epoch four by
validation advantage MSE; no test epoch selection is implemented. The gate
justifies the predeclared one frozen-net/search/redistill attempt.

The pinned native/WASM comparator has a separate policy identity from this
teacher. Its inherited `native-policy-check-20261004/checks/REVIEW.md` checks
actor-view preparation and native/WASM parity, including Voidless lower beliefs
and the rotated odd bidder-team convention. Those findings are inherited finite
evidence, not newly reexecuted here. Comparator timing or head-to-head results
must name its exact phone/partner configuration and cannot be attributed to
this teacher or to a physical phone measurement.

## Complete rung and holdout audit

`checks/results/data-final-rungs-audit/run.json` passes all final T0/T1 data:
16 batches and 4,860 roots in each label dataset; four batches and 1,213 roots
in each independent reference dataset (608 validation, 605 test). T1 and
reference1 continuation hashes explicitly equal the frozen N0 weights. Teacher
source/binary/weight versions do not vary within a dataset. Both frozen N0 and
N1 give C/Python choice parity on all 4,860 roots in
`checks/results/frozen-both-nets-audit/run.json` (2.159 seconds).

Independent `audit_metrics.py` reconstructs Q from packed Bernoulli outcomes and
recalculates every retained decision regret and equal-deal summary mean.
`checks/results/metric-arithmetic-audit/run.json` passes validation0, validation1,
test0 and test1 arithmetic. Independent 10,000-draw deal bootstraps give:

| Comparison on one unchanged reference | Mean N1-minus-N0 regret | 95% interval |
|---|---:|---:|
| T1 validation reference | -0.000870 | [-0.005487, 0.003713] |
| T1 test reference | -0.001308 | [-0.006175, 0.003831] |
| T0 test reference | +0.002209 | [-0.002050, 0.006921] |

All three intervals include zero. A useful compressed T0 decision signal
survives whole-deal test holdout (N0 regret 0.040842 versus uniform random
0.054643; low-budget teacher 0.004139). The frozen-net rung was implemented and
redistilled, but it has not established better decisions than N0. Absolute T0
and T1 regret values cannot establish ladder improvement because continuation
policies change the target. The same-reference paired comparisons above are
the relevant evidence.

The reviewer found a reporting issue: the first evaluation selector preferred
N0 whenever it appeared in a model list, even for T1 gate reporting. The owner
changed it to use the explicitly ordered first model. Both N0/N1 already passed
the numerical validation survival thresholds, so this did not alter the initial
rung authorization or the independent arithmetic. Earlier receipts remain
historical evidence; an acceptance report must identify the assessed gate model.

Verdict: accept the bounded local pilot as reproducible exploratory evidence of
cross-hand paired-value distillation with lawful frozen continuation and complete
deal holdouts. Reject any claim of demonstrated ladder/gameplay improvement,
optimal beliefs, or linear total cost from these results. The next step should
diagnose remaining teacher-to-net decision loss and evaluate a common gameplay
objective before expanding the ladder.

## Final actor-normalized duplicate, gameplay and diagnostic checks

`checks/results/final-h2h-encoding-noise-audit/run.json` passed in 0.509 seconds.
All 4,860 raw actor-normalized 862-input vectors are byte-distinct, including
across train/validation/test. This is stronger than exact serialized request
uniqueness: actor-relative seat encoding already identifies its rotational
equivalents. No symmetry-generated copies exist in this pilot.

The checker independently replayed all 48 complete phone games (1,344 moves,
672 phone calls). Original own hand/public-only requests, acting seat, role,
legal actions, score, leader and complete 42-point final scores all matched.
Frozen weight and phone WASM identities matched. The two gameplay deals have
no original deal-hand partition overlap with the learning population. Each
variant has two independent source deals, four seat rotations and both team
roles; all 16 games per variant and all balanced blocks are complete, with zero
phone interruptions. The actual pinned phone profile uses 160 opening worlds
without partner check, then 40 worlds with partner check. It is local host WASM,
not physical-device execution.

| Candidate | Paired wins | Losses | Ties |
|---|---:|---:|---:|
| T0-128 teacher | 0 | 1 | 7 |
| N0 | 0 | 4 | 4 |
| N1 | 0 | 4 | 4 |

These unfavorable smoke outcomes are retained. Rotations and role swaps are
correlated; two source deals cannot support a useful strength conclusion.
The reported bounded interval [-1,1] appropriately rules out a claim of precision.
Candidate elapsed totals include forced and settled ascending decisions and
cannot be interpreted as pure network or teacher latency ratios.

The post-pilot noise diagnostic uses the first 96 training roots and 4,096 new
uniform-continuation worlds; it trains/selects no further model. Independent
arithmetic checks confirm centered-advantage label RMSE 0.019033,
low-teacher regret 0.003459 and selected-N0 regret 0.017637 on that control.
Together with validation worsening while training loss declines, this supports
investigating coverage/generalization and representation. It does not causally
isolate capacity, and the control is not an additional held-out success claim.

Final verdict remains: useful paired-value compression is demonstrated for the
declared surrogate; ladder improvement and competitive phone strength are not.

## Final report consistency acknowledgment

The reviewer read `REPORT.md`, `README.md`, `CONTRADICTED.md`, the final N0/N1
validation outputs, provenance cost records and byte-identical N0 reproduction
receipt. Reported decision values, qualifications, retained negative results and
summed-worker-wall versus CPU/campaign distinctions agree with the independently
audited evidence. The corrected authoritative validation outputs name N0 and N1
first respectively, matching the repaired gate selector. The N0 reproduction
receipt reports the same complete SHA256, not merely approximate predictions.

One reproducibility documentation omission was reported to the owner: a fresh
source-only experiment copy must build its own `kernel.dylib` before generating
labels. The earlier build command targeted the original directory. This is a
README routing repair and does not change accepted measurements. A minor wording
repair was also requested to describe 25.3% as reduction of the random baseline's
regret and 27.3% as capture of the random-to-teacher decision gap.

Both documentation repairs are now verified: the complete-new-run block starts
with a capped fresh-kernel build before initialization/label generation, and the
report distinguishes baseline regret reduction from capture of the teacher gap.
No remaining review blocker is identified.

No further empirical expansion, theorem or unbounded job was required for this
consistency review. The fixed-budget late-coverage control is a proposed next
experiment, not a result of this pilot.
