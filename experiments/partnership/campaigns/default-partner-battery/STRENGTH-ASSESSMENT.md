# What the existing evidence can tell us without another game

Exploratory post-batch analysis of the completed 100-deal paired panels.
No player runs were launched for this assessment. All rankings below concern
the executed players under this deal/contract/budget protocol, not universal
Texas 42 strength or a transitive rating across unplayed opponents.

## A probabilistic ranking from the saved scores

Let pW, pL, pT be the probabilities of a pair win, loss, and tie. The mean pair
score is pW-pL; comparative contract win fraction is (1+pW-pL)/2.

Assume independent, representative deal pairs and a symmetric
Dirichlet(1,1,1) prior on (pW,pL,pT), equivalent to adding one pseudocount to
each category. After W/L/T observations, the posterior is
Dirichlet(W+1,L+1,T+1). Conditional win share pW/(pW+pL) is Beta(W+1,L+1).
Thus probability of a positive average edge is the probability that this
Beta variable exceeds 1/2. This is exact arithmetic under this stated model,
not an assumption-free confidence certificate or a new observation.

For integer parameters, with n=W+L+1:

    Pr(pW > pL | data) = sum(comb(n,k), k=0..W) / 2**n

| A versus B | Observed W/L/T | Posterior probability A has a positive average edge |
|---|---|---:|
| L2 Partner default vs L1 default | 14/14/72 | 50.0% |
| L2 Partner with voids vs without | 12/17/71 | 18.1% |

Equivalently, the model gives **81.9% probability that voidless L2 Partner
has the positive average edge** in its matchup. This is moderate directional
evidence, not an 81.9% game win rate. It does not say how large the edge is.
The symmetric prior assigns zero probability to exact equality; a model with
an explicit point mass at equality would answer a different question.

This model was selected after observing the results and is presented as a
transparent descriptive assessment. It does not supersede the original report
or create a predeclared stopping rule. The raw estimates remain 50.0% and
47.5%, with the previously reported uncertainty. More informative assumptions
can narrow an answer, but do not add empirical evidence.

## What the saved trajectories show

At the first different tile in the two mirrored games, physical hands and
public history still match. Compare tiles only up to that first split; later
plies can represent different information states.

| Match | Different trajectories | Same contract outcome despite different play | First difference with both primary searches completed |
|---|---:|---:|---:|
| L2 Partner vs L1 | 98/100 | 70/100 | 88/100 |
| L2 Partner with voids vs without | 100/100 | 71/100 | 80/100 |

L2/L1 first splits occurred in tricks 1/2/3 on 69/25/4 deals; two games were
identical throughout. Voids/legacy first splits occurred on 78/20/2 deals.
Most first disagreements are early, so skipping a shared prefix alone will
not produce a large throughput gain. These counts are descriptive: excluding
fallback cases from a strength estimate would change the evaluated player
and can select a different subset of deals.

## A more informative next instrument

Use a declared panel of information states to compare **conditional policy
strength**: run both candidates from the same state, over the same lawful
hidden-hand completions, using a specified opponent field. Preserve each
candidate's actual partnership continuation. Keep hidden information inside
the referee; executed decisions still receive only their seat information.
Couple exogenous randomness where possible and measure variance reduction
per unit of computation before assuming a speedup. Matched simulation inputs
can reduce variance, but their benefit is model-dependent
([Glasserman and Yao, 1992](https://pubsonline.informs.org/doi/abs/10.1287/mnsc.38.6.884)).

This can reveal which kinds of positions benefit or regress, and permits
cheap later-state tests. It is not automatically a whole-game rating:
selected positions need declared selection/weighting, and policies may reach
different states during play. More completions do not replace more independent
focal hands. History-compatible support is not automatically the behavioral
posterior induced by observed choices.

A distinct one-move test can force the two proposed moves, then use one common
continuation policy. That isolates a move under that continuation; it can miss
or misgrade partnership plans whose later execution differs. Label the
estimand explicitly instead of calling either test a universal quality score.

Do not compare each candidate's own pmake estimates as if they came from an
independent judge: their modeled fields and approximations differ. A stronger
common evaluator is still a model whose bias needs checks. Perfect-information
Q can provide diagnostic annotations, not replace the lawful-policy comparison.

## Present practical choice

L1 default costs about one fifth as much as L2 Partner in the completed batch,
with no observed net contract disadvantage. It is a defensible operating
default now without claiming a proved strategic ordering. For research,
inspect where L2 helps and hurts rather than waiting for a tiny global edge
to become statistically decisive. A meaningful minimum improvement should be
chosen before a future promotion/stopping rule is defined.
