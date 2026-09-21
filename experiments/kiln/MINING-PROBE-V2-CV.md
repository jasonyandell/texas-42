# Pre-label extension: do the patterns reduce estimator noise?

2026-09-18. Added after the relational fit was frozen, **before any fresh outcome
analysis**. Parent fit:
`173bd310c97901a6f4629117bcc082492b49bff272c9b2abcaa649578f3fbe8a`.
An earlier fit counted role declarations in its atom-complexity metadata. That
count was corrected before fresh evaluation; all 13 query sources, their order,
and both fitted coefficients remained identical. Both artifacts are preserved.
The fresh full-game batch remains the same fixed 2,880-game sample. This extension
does not change the replication gates or select different hypotheses.

For each of the two primary queries (literal double-five ownership and learned
top-trump ownership), the event is possession of a single identifiable tile by
one hidden chair. With a uniform opening completion, its probability is exactly
7/21=1/3 when that tile is unseen, and zero when it is in the viewer's hand or
does not exist (no-trump has no called boss). This is a probability under the
campaign's intended opening target, not a posterior from bids or discretionary
actions. Queries outside this recognized singleton grammar are refused.

Let Y be failure to make 30 in the complete fixed-player game, X be the query
indicator, and p its known conditional probability for this own hand/declaration.
Fit one coefficient beta per primary using ONLY original discovery data:

`beta = sum_cells sum_i (X_i-Xbar)(Y_i-Ybar) / sum_cells sum_i (X_i-Xbar)^2`.

Freeze beta before opening fresh labels. Then measure the corrected observation
`Z = Y - beta * (X-p)`. For any fixed beta, `E[Z]=E[Y]` under the original target.
No clipping or probability reinterpretation is applied; a finite-sample corrected
mean can lie outside [0,1]. No additional games or changed sampling are required.

Report average within-cell sample variance of Y and Z across all 360 fresh cells,
their ratio, and the same comparison restricted to nonzero-prevalence cells.
The cell sample variances estimate the average sampling variance of independent
eight-game cell means up to the common factor 1/8. A descriptive 1,999-draw
source-deal bootstrap (ten groups) gives a 95% interval on the ratio. It preserves
within-deal/hand/declaration dependence but has only ten source groups; it is not
an exact confidence guarantee. Report both primaries and all controls even when
variance grows. There is no model search or outcome-dependent sample extension.

This tests **measurement efficiency for the fixed played-game bidder**. It does
not show cheaper Walt-internal planning, stronger play, a causally correct threat,
or a general solution to belief proposals. The value to a future mining rig is
having a practical measurement to test after finding predictive descriptions.
