# Outcome-grounded autonomous policy learning for Texas 42

Version 0.1 — September 21, 2026

**Status:** Proposed mathematical contract and independent finite checks. Not an
implemented Texas 42 learner, a strength result, or a kernel-verified theorem.
The paused research campaign was not restarted.

## 1. Goal and experimental authority

The reusable object is a complete lawful actor, not an answer for one hidden deal.
A human specifies the rules, terminal utility, intended table population, legal
observation interface, total learning budget, deployed execution limits, practical
improvement tolerance, and statistical risk budget. The learning program then
chooses numerical updates, candidate expressions, additional experience, and
promotion decisions without human tactical feedback.

A campaign target T records:

- initial physical-deal distribution, seat/role assignment, declaration and bid;
- which seat or partnership is controlled by the learner;
- the fixed other-player profiles, including their full policy semantics;
- all chance and policy-randomness laws;
- actor observation and memory semantics, legal actions, fallback and deadlines;
- utility Y in [0,1], with make/set rather than expected points for contract play;
- target identity, immutable program identities, and compute limits.

For actor pi, J_T(pi) is its expected terminal utility under that complete target.
Training and experimental evaluation need not model humans to claim improvement
against a specified program lineup. Such a claim does not extend automatically
to human partners, different opponents, other contracts, or a complete auction.

Changing an opponent model changes the target. Changing a continuation changes
the policy being evaluated. Changing a deadline can change executed behavior.

## 2. Exact meanings, trainable preferences

A typed Viewer expression F returns a SET of legal action candidates from the
actor's own hand and public record. Define

    x_F(I,a) = 1{a belongs to F(I) intersect Legal(I)}.

Existential witnesses are deduplicated. Hidden-world predicates are prohibited
in the deployed actor. Functions of public history may be admitted with explicit,
versioned semantics; the fact that a record stores history does not establish
that the present expression evaluator exposes it.

For a bounded dictionary F_1,...,F_d, use

    s_theta(I,a) = s_base(I,a) + sum_j theta_j x_Fj(I,a)
    pi_theta(a|I) = exp(s_theta(I,a)) / sum_{b legal} exp(s_theta(I,b)).

This is a proposed weighted consumer, not the ordered first-match semantics of
PR #93. Fix the temperature/scale convention. Bound expression size, coefficient
precision, and execution work. An argmax conversion or symbolic distillation is a
new actor and requires new complete-game evaluation.

The same parameter vector may serve both partners. Each invocation still sees
only its own information. Shared parameters are not shared private observations.

## 3. The outcome-gradient bridge

Fix the other players during an update. Assume a finite game, parameter-independent
rules and chance law, and differentiable positive probabilities on legal actions.
For a full trajectory h, its probability factors as

    p_theta(h) = c_T(h) product_{t in controlled turns} pi_theta(a_t | I_t),

where c_T does not depend explicitly on theta. Opponents may react arbitrarily
to their observations; their fixed response functions are contained in c_T.
Differentiating the finite sum for expected terminal utility gives

    grad J_T(theta)
      = E[ Y sum_{t in controlled turns} grad log pi_theta(a_t|I_t) ].

This identity works for one learned seat or two decentralized teammates sharing
parameters. It requires no differentiable game engine, learned judge, globally
optimal teacher, or enumeration of every compatible world. A baseline measurable
before the action can be subtracted without changing the expectation, provided
its use satisfies the usual conditional-zero identity.

For the linear relational logits,

    partial_j log pi(a|I) = x_Fj(I,a) - E_{b~pi} x_Fj(I,b).

Consequently a new expression F, introduced at coefficient zero, has derivative

    g_F = E_trajectories sum_t Cov_{a~pi(.|I_t)}(x_F(I_t,a), Q^pi(I_t,a)).

Here Q^pi means forcing a legal action and then executing the CURRENT frozen
actor at every later controlled turn, with the fixed target other-player lineup.
It is not Q-star, an old C0/C1 teacher, or a perfect-information action value.
The covariance is a principled proposal score, not proof of finite-step gain or
global optimality. Large changes require whole-policy evaluation.

On-policy complete deals give samples from the correct induced history law
without calculating every history's exact posterior. Resetting a late position
uniformly over its mechanical support defines a different experiment unless that
reset is the declared target. A branch evaluator may retain the sampled hidden
deal, but its actors never receive it. Forcing each legal root action and then
averaging lawful continuation outcomes is permitted; selecting a different best
policy in each hidden world is not.

## 4. A finite-step bound, including decentralized teammates

This is an elementary finite-horizon derivation, not a claim of a new general
policy-improvement theorem. Use a fully detailed state X only for analysis; the
actor still consumes its permitted observation I(X).

Let there be at most H controlled decisions and terminal utility in [0,1]. Pad
shorter games with absorbing, single-action decisions. Compare old policy pi and
candidate pi'. Let Q_t^pi(x,a) execute pi after the action. Define

    f_t(x) = sum_a [pi'(a|I(x)) - pi(a|I(x))] Q_t^pi(x,a)
    S_pi(pi') = sum_t E_{X_t~d_t^pi} f_t(X_t).

Suppose the total-variation distance between pi and pi' is at most alpha at
EVERY reachable controlled observation, not just at sampled training rows.
Then

    | J(pi') - J(pi) - S_pi(pi') | <= H(H-1) alpha^2.

Proof: finite Bellman telescoping gives

    J(pi') - J(pi) = sum_t E_{d_t^pi'} f_t.

Since Q lies in [0,1], |f_t| <= alpha. Couple the trajectories until their first
changed controlled action. At the t-th controlled decision,
TV(d_t^pi',d_t^pi) <= (t-1)alpha. Therefore the expectation difference for f_t is
at most 2(t-1)alpha^2. Summing yields H(H-1)alpha^2.

For the bounded relational features above, an additive logit update obeys

    alpha <= min(1, ||theta' - theta||_1 / 4),

when the dictionary is treated as fixed and new expressions start at weight zero.
To see this, interpolate the logits. At a state the derivative of each action
probability is pi(a)(delta_s(a)-E_pi delta_s). The total-variation speed is at most
one quarter of the range of delta_s. The range is at most the L1 coefficient
change because all features lie in [0,1]. Integrate over the interpolation.

These bounds may be very conservative. They can audit a small-step surrogate;
they do not promise that the sample cost of certifying a useful gain is small.
For larger updates, use independent complete-game comparisons rather than
pretending local teacher loss is a global certificate.

The existing single-focal optimal-regret telescope remains a separate tool:

    V-star(I0) - V^P(I0) = E_P sum_t [V-star(It) - Q-star(It,P(It))].

Its original fixed-field/coherent-belief assumptions must remain intact. Do not
silently apply its single-seat information-state recursion to a jointly optimized
partnership that is allowed to combine private information.

## 5. Autonomous construction, not human-written tactics

The constructor operates over a fixed typed language of mechanical, own-hand,
and public-history primitives. It generates bounded compositions, selectors,
conjunctions, and admissible numeric thresholds. The campaign fixes the language
and resource caps; a human does not select the next strategic relation.

A generation consists of:

1. Freeze the current actor and target.
2. Generate actor-induced complete games and a bounded set of lawful branches.
3. Estimate outcome gradients and candidate expression derivatives.
4. Deduplicate expressions on a versioned probe panel; empirical agreement is
   not global semantic equivalence.
5. Fit coefficient updates and propose a bounded number of dictionary edits.
6. Keep some block edits, independent candidates, and exploration even when
   single-expression derivatives vanish: useful plan changes can be complementary.
7. Freeze finalists; grade their complete behavior on fresh deal groups.
8. Promote only by the registered evidence rule; otherwise retain the incumbent.
9. Save evidence and remaining budget, then repeat or stop.

Centering features across legal actions removes action-independent offsets.
The preference covariance geometry identifies redundant directions and can guide
small steps. It does not prove that all useful 42 strategy is smooth or expressible
by the chosen language. Current three-clause programs are controls, not an a priori
ceiling on the autonomous constructor.

## 6. Reuse cheap modeled players without changing the goal

For fixed candidate P and incumbent P0, let D_H be their paired terminal-return
difference against the target lineup. Let D_L be the analogous difference against
a cheaper lineup, under a coupling with the correct marginal law for EACH run.

    E D_H = E D_L + E(D_H - D_L).

Thus independent batches provide the unbiased estimator

    Delta_hat = average_N(D_L) + average_M(D_H - D_L).

The M correction observations each evaluate target and cheap versions on paired
exogenous inputs. No actor gets hidden information or another branch's history.
With the two batches independent,

    Var(Delta_hat) = Var(D_L)/N + Var(D_H-D_L)/M.

If the cost per cheap observation is c_L and per paired correction is c_C, the
continuous optimal allocation for known positive variances satisfies

    N/M = sqrt[ Var(D_L) c_C / (Var(D_H-D_L) c_L) ].

This ratio is an allocation calculation, not a performance promise. Estimate the
variances and actual complete costs in pilots; keep a direct-target-only control.
A proxy with poor correlation can cost more than it saves. A coefficient other
than one is possible, but coefficient fitting and sampling must preserve the
estimator's authority. The same expectation decomposition applies componentwise
to unbiased gradient estimates at one frozen theta.

The earlier first-disagreement coupling bound also gives, for a fixed policy P,

    |J_sigma(P) - J_proxy(P)| <= Pr_P(first differing field action).

Transferring a comparison between P and P0 needs bounds for BOTH policies. A bound
for the incumbent does not certify every candidate or an unrestricted optimum.

This role for PR #93 is different from treating its compiled players as accurate
teachers. Their usefulness here depends on cost and correlation, not on being
stronger opponents. Their previously reported 2.13x result is not a measurement
of this proposed learning estimator.

## 7. Where the other mathematics belongs

- **Support and counting:** legal domain, compatible sampling, conditional moments.
  Support does not determine a behavior-conditioned posterior.
- **Response vectors:** complete lawful policies are candidates; paired differences
  preserve the distinction between saving a world and selecting a feasible policy.
- **Pivotal comparison:** for Boolean outcomes D is -1,0,1;
  E D = benefit - hazard and Var(D) = pivotal_mass - (E D)^2.
- **Regret and intervals:** identify cheap-to-certify decisions, supported errors,
  and uncertainty requiring more work. Attach root/field/prior authority.
- **Prices, gluing and conflicts:** optional upper producers that bound opportunities
  left by the actor. Center prices at the full information state and solve/bound
  the relaxed optimization correctly. A discovered favorable relaxed path alone
  is not an upper. A lost world need not be fixable by one lawful common policy.
- **Important invariance:** with fixed rows/weights/lowers, tightening the common
  `max U` in `max U - L(a)` shifts every actor's loss equally. It cannot directly
  improve the action ranking. Use such bounds for certification or allocation.
- **Complexity:** grammar size, coefficients, history features, runtime and sample
  groups are part of the learning problem. More games for one hand do not create
  more independent hands.

No requirement exists to activate all these producers in every update. A sound
producer may still be uneconomical. The scheduler needs an empirical comparison
of useful progress per fully charged work, with exploration retained for producer
combinations that have little isolated value.

## 8. Promotion, repeated testing and the risk budget

Each candidate is chosen using past data, then frozen before its fresh promotion
stream. The target also remains fixed. Use independent physical-deal bundles;
mirrored arms and derived branches are components of a bundle, not extra
independent observations. Full policies, including later actions and fallbacks,
are compared.

One simple, conservative construction allocates

    alpha[k,j] = delta / (k(k+1) j(j+1)),  k,j >= 1,

to candidate k at deterministic checkpoint j. Each paired bundle difference D is
in [-1,1]. At n bundles the two-sided Hoeffding radius is

    r = sqrt(2 log(2/alpha[k,j]) / n).

The double sum of alpha is delta. Conditional on the past, each new frozen
candidate's future stream has the stated law. A union bound therefore controls
all interval failures across the run by delta. This is an illustrative safe
construction; variance-sensitive confidence sequences can be more economical.

Promote when the lower bound on J(candidate)-J(incumbent) exceeds the declared
practical improvement threshold. Failed or unresolved candidates need not be
deleted from training search, but cannot be relabeled as stronger incumbents.
Running out of compute is UNRESOLVED, not a loss or a proof of a grammar ceiling.
A final untouched exam remains useful for an overall reported learning curve.

## 9. Old-data boundary and stopping claim

Retained Walt games support initialization, supervised calibration for their own
behavioral target, diagnostics, and reproducible counterexamples. They do not
become current-policy on-policy samples after the student changes. Deterministic
logs have zero support for actions never taken; ordinary importance weighting
cannot manufacture those missing outcomes. Generate new continuations or use an
explicitly justified off-policy method.

Stop on budget, a registered stall condition, or exhausted admissible work. The
honest result is a best validated actor plus unresolved error/coverage/representation
questions. It is not a proof that no stronger 42 policy exists.

## 10. Verification included

`verify_learning_bridge.py` is a standalone Python-standard-library check over
96 tiny, fully enumerated, partially observed games. It checks 576 decentralized
score-gradient components against finite differences, the branch-covariance
identity, 288 finite-step remainder bounds, and 96 exact rational multifidelity,
pivotal and first-disagreement calculations. `verification_results.json` records
the actual results. These checks catch algebra/implementation mistakes in the
bridge. They are not Texas 42 experiments, population certificates, or proofs of
learning efficiency.

## Source lineage

Recovered mathematical sources:

- TEXAS42-UNIFIED-REVIEW-v0.1.md (September 5, 2026), especially sections 2, 4,
  7-10, 12-16.
- PATH-TO-GENERALIZING-SCHEME-POLICIES-v0.1.md (September 7), especially sections
  4-6 and 9.
- SCHEME-GEOMETRY-AND-SMOOTH-POLICIES-v0.1.md (September 7), especially sections
  3-6, 8-10. The relational policy and outcome-gradient bridge are already there.
- POLICY-ANTS-WALT-v0.1.md (September 12), especially sections 6-10.

Repository sources inspected:

- jasonyandell/texas-42 main afd46420435713de4b3b2b98ae09778f3e85c971:
  walt/scheme/INFORMATION-PRICES.md.
- PR #93 head cd70c578f6e71fca4e7823e1ca76304fb26c17b9:
  experiments/response-ladder/REVIEW-START-HERE.md. The latest publication now
  links the public raw corpus; publication checks are not the staged final audit.

External orientation, not 42-specific guarantees:

- Howard et al., Time-uniform, nonparametric, nonasymptotic confidence sequences,
  Annals of Statistics (2021), arXiv:1810.08240.
- Liu et al., A Multi-Fidelity Control Variate Approach for Policy Gradient
  Estimation, arXiv:2503.05696v4 (February 12, 2026).
- Geist, Scherrer and Pietquin, A Theory of Regularized Markov Decision Processes,
  ICML (2019), PMLR 97.
