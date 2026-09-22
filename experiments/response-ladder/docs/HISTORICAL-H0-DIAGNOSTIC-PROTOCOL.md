# Historical H0 versus compatible teaching

Prepared after implementing and validating the separately identified teacher
modes, before their calibration labels are generated. No H2H promotion or
historical-H1 extension is part of this diagnostic.

Reuse the original calibration request IDs, physical groups and train/dev
split. Copy only manifest and requests with hashes into two new campaigns:
`compiled-h0-calibration-v1` and `compiled-h0-seed-calibration-v1`. Keep eight
worlds, 50ms query caps and the sixteen-clause feature vocabulary. Run the
first with `--mode historical-h0`, the second with `--mode compatible-h0-seed`.
The existing n8 compatible T0 corpus `compiled-calibration-v3` is the third
control. Preserve all original ordered worlds/tapes, effective sampling seeds,
complete vectors, settled roots, refusals and errors in every comparison.

H0 uses the native fixed seat/current-hand/record seed and Voidless sampler.
The second arm changes only to public-void-compatible sampling with the same
seed. The existing T0 arm changes to its declared request seed. The targets
are explicitly different; this distinction is not an implementation bug or
evidence that historical sampling is strategically superior.

On identical completed roots, report canonical-choice agreement, intersection
of optimal sets, and cross-cost of each canonical choice under each target.
Stratify by role, remaining hand size, and presence of public void deductions.
Retain all missing/censored roots in coverage denominators. With no public
void constraints, require exact equality of ordered H0/compatible-H0seed worlds,
tapes and action vectors, including partial-trick capacities. Any mismatch is
an adapter failure, not a strategic result.

Fit a separately named H0 role-only actor with the existing sixteen-clause,
maximum-three-clause grammar and training-only exact selection. Compare it
with frozen v3/v4 actors on the same H0 development labels, and cross-score
on the existing T0 labels. Report canonical prediction accuracy separately
from optimal-set agreement and normalized teacher loss. Tied teacher actions
can still induce different public observations in a modeled field.

Do not infer stronger play from imitation accuracy or local teacher loss.
No C1 label may be relabeled after C0 changes. Any online field substitution
requires an explicit rung graph and a separately frozen complete-game protocol.
All runs use 55s cooperative cycles under the existing 60s process-group
watchdog, with no competing builds or timed jobs.
