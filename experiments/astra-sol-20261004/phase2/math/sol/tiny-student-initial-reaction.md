# Sol initial reaction to the tiny per-hand student account

2026-10-04. **EXPLORATORY; reported numbers not yet verified against artifacts.**
This records a mathematical diagnosis, not permission to train or a strength
result. The recovered Fable archive was subsequently inspected; see report.md.

The reported 276k binary rollout labels and 79% outcome accuracy do not establish
a useful action chooser. The accompanying regret .028, versus random .031 and
Walt .001, suggests that predictable outcomes consumed much of the learning
signal while small within-position action differences remained unresolved. A
classifier can predict whether many positions are won while giving almost the
same score to all legal actions. Accuracy, calibration, optimal-set agreement,
regret, and full-game marks are different quantities.

The reported iteration sequence .470 → .460 → .452 → .458 is not a monotonicity
theorem or persuasive progress by itself. Establish what each number measures,
the fixed opponent field, fresh samples, paired seeds, uncertainty, and whether
those positions were used for training or variant selection. Raising model level
changes the field and can worsen actual play even with exact best responses.

The reported 8.7k paired examples and 56% policy accuracy still leave the target
ambiguous. A canonical-action label penalizes equally optimal actions and hides
the cost of large-gap mistakes. Prefer a complete same-query legal-action value
vector with its empirical best value and advantages, plus exact sample counts,
tie set, estimated gap, and label uncertainty. Retain original scenarios,
multiplicities, worlds, and tapes so independent repricing can recover every
target. Common-random-number pairing estimates action differences without
subtracting independently noisy means; its variance benefit depends on the
actual covariance and should be measured rather than assumed.

The reported paired V0 control using 2048 continuations per action, regret .006
and 89% agreement on clear cases, is especially informative: it suggests a
teacher/target or label-noise bottleneck that should be separated from network
capacity. A retained 2048-scenario mean is exact **on that finite panel** if its
continuations were completed; it is not automatically exact on the full fiber
or under the true Bayesian posterior. If V0 itself loses substantial quality
relative to Walt, faithfully fitting V0 does not close that gap.

Three information distinctions are essential:

- **Lawful student inputs:** own hand and complete public history/contract.
  A learned function of those inputs can implicitly integrate a belief; it need
  not explicitly draw posterior worlds on every inference. Its accuracy and
  prior/history conditioning still need to be established.
- **Posterior simulation:** a teacher may sample hidden hands to average lawful
  policy outcomes. Sampling is a numerical procedure, not an illicit extra
  observation, provided the acting policy never receives the sample identity or
  hidden hands. Public void support alone does not determine a posterior.
- **Teacher strategy fusion:** optimizing future focal actions separately in
  each revealed world produces an upper relaxation. Lawful student inputs do
  not repair the teacher's target semantics. Conversely, knowing the world in a
  simulator while replaying one fixed lawful policy is not strategy fusion.

Per-hand training and universal distillation have different feasibility claims.
Splitting histories within one original seven-tile hand can assess interpolation
for that hand but cannot establish transfer to unseen hands. For a shared policy,
split by original own hand/source deal before creating related histories and
labels, and reserve new on-policy game deals for final play assessment. For a
runtime per-hand learner, include the complete label-generation and fitting
latency in that hand's allowance; tiny inference does not make teaching free.

No measured Mac training throughput has been established in this review. The
local paused C0/C1 predecessor is fitted Scheme policy work, not the remembered
tiny neural student, and its pause was recorded as a user usage-budget stop.
A small model's arithmetic may be tractable on the Mac while expensive exact
or paired teacher generation remains the dominant cost. Archive provenance,
representation size, existing labels, teacher completion rates, and inference
cost must be read before even a small feasibility run. No substantial training,
no repetition of the single-outcome experiment, and no imported-script execution
have occurred.

Our first-disagreement certificate offers a separate possible bridge: any frozen
student is a lawful cheap field, and a completed all-policy disagreement bound
can protect particular sampled root choices relative to a named exact target.
That certificate may itself be too expensive or loose. A local teacher-regret
number alone cannot substitute for it, as the phase 1 argmax amplification
counterexample proves.

After the lead read the full authorized Claude conversation, message 20
identifies this tiny student's original teacher as fixed uniform-rollout
outcomes. Those labels do not themselves involve per-world future optimization.
The fusion defect found in the later archive's exact-tape player is a separate
experiment. The tiny experiment's missing neural source, weights and logs still
prevent reproduction; the archive contains none of them. Its reported regret
and label behavior warrant a target-noise/posterior/occupancy investigation,
not an assertion that its original teacher cheated.
