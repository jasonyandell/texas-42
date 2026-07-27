# Support Is Not Belief

[Home](Home.md) · owns: the support/belief separation, Bayes machinery, the 90-world
witness · Sources: both packages Math §6.7, §8, §10.4 (shared). Related:
[support-fiber](support-fiber.md), [strategic-state](strategic-state.md).

## The separation

Rule support is a Boolean compatibility predicate; belief is a measure over compatible
latent worlds shaped by the chance law **and the likelihood of discretionary actions
under a policy model**. Two histories can have identical support and different
likelihood functions [Theorem — proved, Math §6.7, INFO-11]. Legality fixes zero
likelihood for impossible actions and one for forced actions (singleton legal set,
BEL-12); beyond that, discretionary likelihood ratios are policy-model-relative —
different valid models reverse Bayes factors on the same action [Theorem — proved,
Math §8.8, BEL-14].

## Belief machinery (Math §8, shared)

- Augmented root world `ξ = (deal, inherited latent state)`; posterior = inherited
  prior × rule-compatibility indicator × within-attempt history likelihood, normalized
  [Theorem — proved (Bayes), BEL-04]. Two exact randomness representations — kernel
  fields vs seed-augmented (random-tape) fields — must never be mixed (conditioning on
  a tape makes action likelihood 0/1; multiplying again double-counts) [BEL-03A].
- Physical belief = pushforward through the remainder map [BEL-05]; when field state
  matters, keep the *coupled* augmented belief (graph / conditional-kernel
  factorizations are exact special cases; bare marginals need proved independence)
  [BEL-06/06B].
- **Physics-only uniformity** [Theorem — proved, BEL-07/07A]: under the uniform deal
  law and no action-likelihood tilt, the posterior on any fixed-history fiber is
  uniform, and the count-ratio sampler ([capacity-dp](capacity-dp.md)) samples it
  exactly.
- One public action has three separable effects — physical transition, support
  restriction/retyping, likelihood reweighting — plus, for deal-ending actions, a
  new-deal chance extension [Theorem — proved, BEL-11/11A]. All-pass creates a **new
  domain**, not a reweighting of the old deal.
- Off-path (zero-probability) histories require an explicit assessment [Boundary,
  BEL-15 / OPEN-07]. On-path higher-order beliefs are induced, not primitive
  [Theorem — proved, STR-11].

## The 90-world counterexample (the package's crown witness)

**[Theorem — exhaustive finite verification + constructed counterexample,
Math §10.4, STR-06..09]** — fully replayed by `verify_foundation.py`.

Setup: seat 3 bids `P(31)`, declares no-trump; after five fixed tricks the mechanical
endpoint has seat 3 to lead holding `3-1, 4-1`; the 6-tile unseen pool with capacities
(2,2,2) and no relevant voids gives exactly `6!/(2!)³ = 90` legal remainder worlds.

Two *legal* auction histories reach this same endpoint, differing only in which losing
seat bid 30:

```
α_A: 0:pass, 1:P(30), 2:pass, 3:P(31)
α_B: 0:P(30), 1:pass, 2:pass, 3:P(31)
```

Under a fixed stochastic bidding field (bid-30 probability 2/3 with `4-4` in hand,
1/3 without), both posteriors give **all 90 worlds strictly positive mass** — same
rule support, same posterior support — but the `4-4`-holder marginals differ
(A: 1/7, 4/7, 2/7 vs B: 1/2, 1/4, 1/4), and exact backward induction gives opposite
optimal leads: A prefers `4-1`, B prefers `3-1`. The flip holds under **all four**
utility lenses (expected points, signed differential, contract success, one-mark hand
utility) [STR-09]; after the root lead every own action is forced, so no strategy
fusion [STR-10].

**Moral**: a path-free mechanical coordinate is an exact *support* state but not an
exact *strategic* state for history-sensitive fields. This witness guards the central
boundary and both packages demand it become a named kernel theorem in mechanization
(v0.7 Handoff §9).

Related pointwise fact: the same physical tile `4-1` at the same endpoint has exact
world-conditional values −22 and +22 in two fiber members — **no context-free scalar
domino value exists** [Constructed counterexample, Math §9.7, HAND-07].
