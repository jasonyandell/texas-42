# Independent fixed-budget coverage review

Exploratory. Preregistered design critique; implementation/results pending.
The original pilot and its reviewed receipts remain unchanged.

The primary comparison should measure late-position regret of mixed32 versus
late32 on one unchanged late-only held-out reference panel. Matching training
roots and worlds isolates an occupancy intervention, provided source-deal
coverage/multiplicity, teacher identity, architecture, optimizer, epoch budget,
validation panel and selection rule are also matched and reported. Source
histories are off-policy uniform-play trajectories; the support-conditioned
belief omits historical action likelihood and bidding as before.

The 912-root precision comparison should hold roots/encoding/model fixed at
128 versus 512 worlds. Reuse the main late arm's 128 labels on those exact roots;
prefer extending the same sampled-world/tape stream for 512 labels. Comparing
3,648 roots at 128 worlds against 912 at 512 holds nominal root-world budget
fixed but jointly changes coverage and precision. Legal action counts, rollout
horizons, training updates and actual cost can still differ.

Mine and freeze at most 16 source histories per deal before label generation.
Select live, nonforced roots with explicit absolute ply boundaries. Deduplicate
actor-normalized inputs before selecting training arms. All related roots and
histories from one original deal remain in one split and one uncertainty cluster.
Verify disjoint source partitions and normalized inputs against the old pilot
and phone smoke. A common late-only validation panel controls epoch selection
for all models, including mixed32. Fresh high-budget reference seeds are separate
from label seeds.

Only the predeclared late32 versus mixed32 validation gate may trigger one rung:
at least 10% regret reduction, negative upper paired 95% difference bound, and
improvement over random/ascending. The capacity64 and precision arms are
diagnostics; they do not authorize an adaptive replacement gate. Freeze all
model weights before opening test results. No general learning or gameplay
improvement theorem follows from a gate pass.

## Pre-label availability repair

The initial initializer failed before labels/training because source 80 lacked
three distinct live late roots after 16 uniform trajectories. A pre-label repair
is reasonable: retain all 2,048 unconditional fresh deals, mine exactly 16
trajectories for each, preserve candidate counts, then deterministically allocate
3,648 roots by round-robin quotas capped at each source's available late roots.
Both main arms must have exactly the same eligible source set and multiplicity.
Missing-late sources stay in mining/source manifests and panel denominators;
they must not be silently replaced by additional sampled deals or trajectories.

Availability is label-independent, not generally outcome-independent: an
unsettled contract depends on public scores and live-game survival. The population
is therefore explicitly conditional on live, nonforced late-root availability.
The mixed arm is also restricted to those late-eligible training sources, giving
the intended matched-source control but a narrower scope than unconditional
mixed-occupancy training. No all-deal strength claim follows from this selection.

Freeze mixed-band priority, quota order, 912-root subset selection and the revised
plan before generating any labels. Preserve the exact initial plan and failed
receipt. Stop for another explicit design amendment if the total unique late
candidate supply cannot meet the fixed root budget. Held-out late results use
their actual eligible source-cluster counts alongside the original 256-deal
denominators.

## Repaired implementation and first paired-data audit

`checks/results/structure-first-audit/run.json` passed in 2.942 seconds. The
reviewer independently regenerated every original source deal from the frozen
source RNG, verified all 8,430 retained roots against original full-deal legal
replay, and checked current actor, live contract, nonforced legal choice and ply
band. All normalized inputs are unique and disjoint from the original pilot.
All 2,048 source partitions are unique and disjoint from original pilot and
phone-smoke source deals. The new C kernel is byte-identical to the old kernel.

Round-robin quota allocation was independently recomputed from frozen mining
counts. Both main arms have exactly 3,648 roots from the same 1,522 training
sources with identical per-source multiplicities; 14 unavailable training sources
remain recorded. Mixed bands contain 1,522/1,216/910 early/mid/late roots.
The precision subset contains one late root from each of the first 912 eligible
sources. Validation has 512 late roots from all 256 sources; test has 508 from
254 sources, retaining the two unavailable sources in the 256-source denominator.

Independent replay of all 16 trajectories on 24 predetermined sources, including
12 unavailable sources, reproduces mining candidate counts and selected-root
membership. This is a finite generator replay in addition to complete retained
row legality, not a claim that every mined trajectory was independently rerun.

`checks/results/label-membership-prefix-audit/run.json` passed in 2.849 seconds.
Each main dataset contains exactly its 3,648 assigned train roots plus the common
512 late validation roots; each subset contains 912 assigned train roots plus
the same validation panel. Packed outcomes reconstruct every saved Q. Teacher
identity is frozen and uniform. Overlapping 128-world targets are byte-identical;
all 512-world rows have the same first 128 paired outcomes. Replaying sampled
world/tape streams on 12 subset roots confirms their prefix hashes match the
main 128-world sample identities. Subset512 training uses 512 worlds while its
validation uses 128, with per-record world counts correctly retained.

Source audit confirms the unchanged trainer uses identical late-only validation
MSE selection for all arms, with test excluded. The model freeze records 100
epochs, 1,500 updates per full arm and 400 updates per subset arm; every selected
checkpoint is epoch two. The capacity64 control uses the same late128 labels.
Training update count differs in the coverage/precision tradeoff and is reported,
not silently asserted equal.

The primary validation gate fails: late32 improves mixed regret numerically by
only 1.824%, below 10%, and the paired interval includes zero. The capacity and
precision diagnostics cannot replace that gate. No new rung is authorized by
the predeclared protocol. Independent final metric arithmetic remains pending.

## Independent validation arithmetic and adaptive exposure control

`checks/results/validation-arithmetic-audit/run.json` passed in 0.189 seconds.
Packed independent-reference outcomes reproduce every selected-action regret and
reported equal-deal mean. The gate is independently false. A separate 10,000-draw
bootstrap gives late32-minus-mixed32 regret -0.000824 with interval
[-0.006979, 0.005051], so neither the required 10% improvement nor a resolved
primary benefit is established. Capacity64 and the same-root precision comparison
also have intervals crossing zero. More roots at 128 worlds beat fewer roots at
512 on this validation panel, but that remains a coverage/precision/update-count
tradeoff diagnostic, not an isolated causal result.

The owner proposed a separately recorded adaptive diagnostic before opening test:
train the same 912-root precision arms for 375 epochs, making their maximum
optimizer updates 1,500, matching the 3,648-root arms' 100 epochs. The unchanged
validation-MSE selector may still select the early identical checkpoint. Maximum
updates and selected-checkpoint updates must both be reported; equality of maximum
budgets is not equality of learned exposure. Freeze these extra weights before
test, retain the failed primary gate, and do not replace it with a favorable
diagnostic. No new labels, architecture scaling or rung follows from this change.

## Frozen adaptive checkpoints and fresh test arithmetic

`checks/results/final-test-metrics-audit/run.json` passed in 0.317 seconds.
All five original controls and two adaptive controls match their frozen hashes.
Both 375-epoch adaptive checkpoints are byte-identical to their respective
100-epoch checkpoints and selected epoch two (eight updates). Their extra allowed
updates therefore do not change selected decisions. The original local audit
verified both model-freeze file times precede saved test-generation receipt UTC
times. Filesystem chronology is local evidence: checkout/relocation resets
mtimes, so the replay script marks this check optional rather than rejecting
otherwise identical committed evidence on a new checkout.

Complete packed-label audit passed in 3.258 seconds in
`checks/results/final-all-data-audit/run.json`. Fresh test metric arithmetic
passes both late-only 508 roots/254 eligible sources and all-band 1,020 roots/
256 original sources. Independent 10,000-draw late-test source bootstraps give:

| Comparison (lower regret is better) | Mean difference | 95% interval |
|---|---:|---:|
| late32 minus mixed32 | +0.006954 | [-0.002958, 0.017310] |
| late64 minus late32 | -0.005505 | [-0.017492, 0.006511] |
| subset512 minus subset128 | +0.000550 | [-0.001257, 0.002590] |
| late32 minus subset512 | +0.000813 | [-0.008926, 0.010916] |

All intervals include zero. The validation coverage/precision tradeoff favoring
more roots did not repeat on test. None of the tested controls establish late-only
coverage, capacity or label-precision superiority. The failed primary gate stays
failed, with no new rung. Phone-hybrid and final-report reviews remain pending.

## Complete late-only phone-hybrid audit

`checks/results/phone-hybrid-audit/run.json` passed in 0.150 seconds. The checker
independently replayed all 48 complete games/1,344 moves against the existing
referee, checking original-own-hand/public request boundaries, actor turn,
legality, public score, leader, final 42 points and frozen model/phone identities.
Both new gameplay source partitions are disjoint from both learning experiments
and the prior phone-smoke sources. Each variant contains exactly two deals, four
rotations and both candidate partnership assignments.

The route condition is exactly live, nonforced absolute plies 16 through 23 for
the candidate partnership. All 125 overrides reproduce their exact teacher/net
choices (41 teacher, 42 mixed32, 42 late32). Every other turn uses the pinned
phone, including early/mid, forced, settled and after-ply-23 turns. All 1,219
phone calls are uninterrupted. The actual available route counts differ by
variant (80/78/80 total eligible moves), because eligibility depends on trajectory.
There are no omitted incomplete or invalid games.

Every variant has 0 paired wins, 0 losses and 8 ties. This two-source hybrid smoke
is not evidence of equivalent competitive strength; its bounded [-1,1] interval
and correlated role/rotation qualification remain appropriate. Route timings
cannot provide an equal-cost or pure-inference speed ratio. Isolated latency
reports, cost arithmetic and final documentation review remain pending.

## Cost, noise and portable chronology evidence

`checks/results/cost-noise-arithmetic-audit/run.json` passed in 0.490 seconds.
Mixed128, late128 and subset512 each allocate 466,944 training root-worlds.
Training candidate continuations differ: 1,739,136 mixed, 1,188,096 late and
1,190,400 subset512. Shared validation labels were regenerated in each arm and
their actual compute is retained. These are allocated candidate samples,
not rollout plies; early settlement shortens work. Reported generation seconds
sum worker wall intervals, not CPU seconds or parallel campaign elapsed.

Parameter counts, raw bytes, actual file sizes and maximum/selected optimizer
updates agree with frozen artifacts. Independent legal-centered paired-outcome
noise arithmetic on all 912 subset roots confirms estimated target-mean variance
0.000248896 at 128 worlds and 0.0000622463 at 512. Lower measured label variance
does not establish a decision-regret or gameplay benefit; the matched test
precision comparison remains inconclusive.

`checks/results/portable-snapshot-metrics-audit/run.json` passed in 0.310 seconds.
The retained chronology snapshot hashes match both frozen manifests, and its
recorded original local times precede saved test-generation receipt times. This
is portable preservation of the original local audit evidence, not tamper-proof
creation-time attestation. Portable model/data identities remain authoritative.

The superseding isolated warmed CPU latency records separate pure forward time
from validated encoding/legal decision time and exclude loading/startup. The
unchanged benchmark source uses 1,024 single-position samples. Earlier concurrent
measurements are retained but should not be used as accepted isolated timings.
No new hardware benchmark was run by this reviewer.

## Final report consistency review

The reviewer read `REPORT.md`, `README.md`, `HISTORY.md` and `TEAM.md` against
the retained audits and authoritative outputs. Reported regret values, failed
primary gate, lack of a new rung, conditional live-late scope, negative capacity/
precision tests, adaptive duplicate checkpoints, isolated inference timings and
worker-wall accounting agree with the evidence. The proposed next experiment
is a recommendation and was not automatically run.

Two documentation repairs were requested. First, the pre-label repair preserved
the intended 2,048 unconditional source count, not the exact original failed
deal sequence: the revised code separates source RNG from trajectory RNG, while
the failed initial code interleaved them. The initial failure stopped at source
80 before complete manifests, labels or training. Second, the disposable full
reproduction copy must remove its generated `plan.json` before `initialize`,
which explicitly refuses an existing plan. The canonical original plan/history
must stay preserved. These are provenance/reproduction wording issues, not
changes to accepted measured results; verification of their repair is pending.

Verdict: accept reproducible bounded negative/inconclusive control evidence with
useful teacher-relative signal retained by the mixed model. Do not infer coverage,
capacity, precision, optimizer-budget, ladder or phone-strength improvement from
this follow-through. No production or original-pilot modification was needed.

## Final repaired-documentation acknowledgment

Both requested documentation repairs are verified. The report now states that
the repaired generator retained the intended 2,048-source count, separated RNG
streams before labels and changed the original failed sequence after source zero.
README cleanup removes generated `plan.json` only from a disposable reproduction
copy, preserving canonical history. The sequence generates common held-out teacher
labels and independent references separately and freezes models before test.

The new `train_adaptive.py` reproduction wrapper was source-reviewed without
retraining. It reproduces the saved H32/375-epoch commands on the same subset
datasets with 295-second child caps, requires the failed primary gate, refuses an
existing adaptive freeze, and writes the historical freeze format. It does not
replace or rerun historical measurements. Moving the unopened-test guard before
its training loop was suggested as a fail-early robustness improvement.

Final acceptance: all substantive conclusions and deliverables agree with the
independent retained evidence. No remaining empirical or documentation review
blocker is identified. The failed primary gate remains failed and no new rung ran.
