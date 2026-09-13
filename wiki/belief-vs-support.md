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
boundary, and both packages demanded it become a named kernel theorem in mechanization
(v0.7 Handoff §9; rec Kernel K15). **It is one.** PA-E10 `Witness.ninety_world_witness`
(`lean/Texas42/Witness.lean:728`) was kernel-proved on 2026-08-02 (commit d190b26 — the
commit that closed the 42 priority-0 rows), internalized whole: the fiber is enumerated
and proved equal to the cells, every world is replayed, the posteriors are exact
rationals, and the values come from kernel-evaluated rollouts of the committed play
machinery — no external receipt imported (TRUST-01). What the kernel settles is that
the witness *holds*; whether 90 is the *smallest* such fiber remains **open**
([open-problems](open-problems.md) item 3).

Related pointwise fact: the same physical tile `4-1` at the same endpoint has exact
world-conditional values −22 and +22 in two fiber members — **no context-free scalar
domino value exists** [Constructed counterexample, Math §9.7, HAND-07].

## Mechanization status (proof-assistant kernel tier)

Rows are **v0.7** `65_MECHANIZATION_LEDGER.md` `PA-` rows (priority in parentheses);
"proved" = a declaration under `lean/Texas42/` checked by the Lean kernel over at most
`propext`/`Classical.choice`/`Quot.sound`, with no `sorry`, `native_decide` or local
axiom (grep re-verified 2026-09-12), as of commit d190b26 (2026-08-02; all 42
priority-0 rows closed). Map: [lean-row-index](lean-row-index.md). A kernel theorem
never promotes a corpus status: STR-06..09 remain corpus THEOREM — finite verification
(`verify_foundation.py`, re-run 2026-09-12 on this machine: "history witness fiber
worlds: 90 … opposite best leads: PASS", identical); the kernel theorem is a second,
independent tier. rob's `r_cell_ninety_world_support: 90` is conformance evidence only.

| Result on this page | Ledger row (priority) | Kernel status (d190b26) | Declaration (`lean/Texas42/`) |
|---|---|---|---|
| Finite PMF prior; policy/field kernel | PA-E01 (0) | **proved / defined** — exact rational masses, finite-first (no measure theory in the native game) | `Belief.lean` `FinPMF`, `PolicyKernel` |
| BEL-04 Bayes posterior: normalization and the likelihood chain rule; within-attempt history likelihood product | PA-E02 (0) | **proved** | `Belief.lean` `FinPMF.condition`, `:97` `condition_mul`, `likelihoodFrom`, `:171` `likelihoodFrom_append`, `posterior` |
| BEL-05 physical belief = pushforward through the remainder map; **support ⊆ fiber** | PA-E03 (0) | **proved** — the kernel statement of "support bounds belief without determining it" | `Belief.lean:274` `physicalBelief_support_isWorld` |
| BEL-07/07A physics-only uniformity under the uniform deal law | PA-E04 (1) | **open** | — |
| Finite exponential-tilt form; forced-action world-nondiscrimination (BEL-12) | PA-E05, PA-E06 (1) | **open** | — |
| INFO-11, BEL-03A, BEL-06/06B, BEL-11/11A, BEL-14, BEL-15/OPEN-07, STR-11 | no rows | not mechanized | — |
| STR-06: the endpoint fiber is exactly 90 worlds, each rule-realized | PA-E10 (0) | **proved**: `worldPairs.card = 90`; fiber = cells (`isWorld_iff`); every world realized by a rule-compatible deal that legally replays the five-trick prefix (90 kernel replays) | `Witness.lean:99` `card_worldPairs`, `:590` `isWorld_iff`, `:692` `replay_check`, `:708` `rule_fiber` |
| STR-07: two distinct legal auction histories `α_A`, `α_B`, same result (`P(31)`, seat 3) | PA-E10 | **proved** | `Witness.lean:506` `auction_histories` |
| STR-08: both posteriors give all 90 worlds positive mass | PA-E10 | **proved** (weights sum 210 and 120) | `Witness.lean:233` `same_full_support` |
| STR-09: the lead flips under **all four** lenses | PA-E10 | **proved for two lenses** — expected signed differential (`−160/21, 10/7, −217/30, −52/5`) and contract-make probability (`1/3, 16/35, 1/3, 1/5`); the remaining two §10.4 lenses (expected declaring points; one-mark hand utility) are positive affine transforms of these and are **not separately stated** in Lean — the four-lens claim stays at corpus finite-verification tier | `Witness.lean:400` `expected_differentials`, `:438` `make_probabilities`, `:477` `posterior_action_reversal`, `:728` `ninety_world_witness` |
| STR-10 no strategy fusion (every own action forced after the root lead) | — | not a named kernel theorem; the values `Q` come from 180 deterministic lowest-ID rollouts of the committed `PlayState.step` (`Witness.lowestLegal`, `rollout`) | — |
| HAND-07 world-conditional values `−22` / `+22` of `4-1` | PA-E11 (1, WITNESS) | row **open**; the anchor table (10, −22, −22, +22) is kernel-evaluated but not stated as the HAND-07 counterexample | `Witness.lean:147` `anchor_values` |
| Minimality of 90 | no row | **open** | — |

Cost record (engineering, not a game result): `Witness.lean` verifies in 33 s under
`decide +kernel` (all 35 such uses in the library are in this file); the elaborator
path was OOM-killed after 16 h; full library 47 s (commit d190b26 body; `lean/PROOFS.md`).
