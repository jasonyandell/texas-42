# Model-Belief Walt and the Closing of the First Book

## Persistent behavioral hypotheses, point-mass response geometry, sparse counterfactual fields, and a completion criterion for the tractable base player

**Status:** exploratory mathematical design for intake, adversarial review, implementation, and later formalization. This document does not promote any probe, implementation, or theorem merely by restating it.

**Date:** 2026-09-01

**Repository state inspected:** `jasonyandell/texas-42` `main` at commit `08fe3d2ddca2a17cf8faf633add69d3f83592ec5`.

**Mathematical parents:**

- *The Mathematics of Walt v0.1*;
- *Counted Belief Sandwiches and the Refinement Calculus for Walt v0.1*;
- *Counted Residual Bellman Calculus for Walt v0.1*;
- *Anytime Proof-State Walt v0.1*;
- *The Salvation Complex and Information-Cut Calculus of Walt v0.1*.

**Engineering sources inspected:**

- `wiki/walt-counted-belief-era.md`;
- `walt/FACTOR-BELIEF.md`;
- `walt/walt/src/solver/{proof_state,frontier,factor_belief,residual,covers,laydown,opening,doom,field,policy}.rs`;
- `walt/probes/factor_belief/{openingreport_run1,doomreport_run1}.txt`;
- the corresponding gate files and commit records through PR #80.

No external literature theorem is imported as authority here. Where a broad algorithmic family is named, it is a direction, not a receipt.

---

# 0. Executive ruling

Walt is approaching the first goal:

> **a strong, tractable, mathematically bounded Texas 42 player whose ordinary behavior is conservative, explainable, and deliberately unromantic.**

It is not finished. But the remaining work no longer looks like an indefinite attempt to make one monolithic exact solver faster.

The current system already has:

- a canonical physical fiber;
- exact counted belief without complete-world materialization;
- information-consistent focal optimization;
- persistent typed proof states;
- exact score profiles;
- lawful extracted policies;
- certified `pmake` regret;
- residual Bellman intervals;
- count-threat covers;
- typed laydown results;
- doom uppers;
- a resumable work frontier;
- and honest unresolved output.

The opening specimen now has a strong executable lower floor but an upper near one. The doom census says that, at least on the tested opening structures, physical impossibility is not the main source of that upper. The next useful upper work must price **incompatibility across hidden possibilities**, not merely count individually unsalvageable worlds.

At the same time, the best-response ladder has clarified the partnership-cognition depth that project level 2 represents. But the ladder makes one restrictive assumption that Texas 42 itself does not require:


after choosing a rung, Walt treats the entire nonfocal world as one known complete behavioral policy.

The next unifying step is therefore:

> **Treat the field model itself as hidden state.**

A fixed field is a point-mass belief over behavioral types. The Dice-to-best-response ladder becomes a library of possible field types rather than the only permissible worldview. Public actions update the posterior over those persistent types. Walt branches only where their behavior differs and only where the difference can still affect the contract.

This does not replace the existing mathematics. It lifts it to an augmented latent space:

\[
\text{physical deal}
\times
\text{behavioral type profile}
\times
\text{persistent random tape, if needed}.
\]

Every prior Walt object then survives:

- fibers;
- factor beliefs;
- score profiles;
- pivotal geometry;
- doom;
- salvation masks;
- gluing cuts;
- policy columns;
- certified regret;
- and proof-state refinement.

The best-response ladder becomes the degenerate point-mass case of a broader model-belief solver.

The first book can reasonably close after Walt has:

1. a small persistent model library containing Dice and the first useful response rungs;
2. exact point-mass parity for every library member;
3. a finite model-belief field with posterior updating;
4. a type-revealed upper assembled from point-mass responses;
5. at least one nontrivial type-gluing or model-conflict upper;
6. sparse counterfactual refinement at consequential model disagreements;
7. a model residual that remains honest when the library is incomplete;
8. a playable policy with certified `pmake` regret under a declared model belief;
9. late-game exactness or fusion-cut reuse where affordable;
10. and arena evidence that the resulting base player is both strong and tractable.

A joint partnership solve is a major possible next abstraction. It need not be completed before the first book closes. The architecture should avoid blocking it.

---

# Part I — Where Walt stands

## 1. The current mathematical output

For each legal root action \(a\), current Walt maintains a valid interval

\[
L_a\le Q_a\le U_a,
\]

where \(Q_a\) is the exact best-response value against one named fixed field.

It also maintains:

- a proof bar
  \[
  B_{\mathrm{proof}}=\max_aL_a;
  \]
- an executable bar
  \[
  B_{\mathrm{exec}}=
  \max_{\rho\text{ materialized}}L(\rho);
  \]
- the global upper
  \[
  U^\star=\max_aU_a;
  \]
- and certified executable regret
  \[
  \Gamma=U^\star-B_{\mathrm{exec}}.
  \]

At the inspected opening root, the executable floor reached approximately \(0.732\), while the strongest installed upper remained approximately \(0.999\). The result was correctly `UNRESOLVED`, with a named recommendation and approximately \(0.267\) unclaimed `pmake`.

That is already a useful player object. It is not yet the dream player because its opening upper does not sufficiently reflect hidden-information incompatibility.

## 2. What doom did and did not diagnose

Let \(U^{\mathrm{God}}\) be the world-revealed upper: the focal player is told the physical world and may choose a different continuation in each world.

For an executable incumbent \(\widehat\rho\),

\[
1-V(\widehat\rho)
=
\underbrace{1-U^{\mathrm{God}}}_{\text{physical doom}}
+
\underbrace{U^{\mathrm{God}}-Q}_{\text{information-consistency price}}
+
\underbrace{Q-V(\widehat\rho)}_{\text{policy gap}}.
\]

The doom census attacks the first term.

The opening census found little or no certifiable physical doom in its tested regions. Therefore additional doom counting alone cannot be expected to close the upper.

But zero observed or certified doom does **not** by itself prove that the entire remaining gap is information-consistency price. Some of the gap may remain on the lower side because the best materialized policy is not yet optimal.

The next solver should therefore race two directions:

- **column generation:** find better lawful policies and raise the floor;
- **information cuts:** prove that no single policy can realize all hidden-state-specific successes and lower the upper.

## 3. What the ladder establishes

Let \(D\) denote Dice and define

\[
F_0=\operatorname{BR}(D),
\qquad
F_1=\operatorname{BR}(F_0),
\qquad
F_2=\operatorname{BR}(F_1).
\]

Under the project’s indexing, \(F_2\) is the first rung at which the modeled partner is itself reasoning about a thinking version of the focal player.

This is a real and useful cognitive landmark.

The ladder remains an excellent policy generator and a clean source of point-mass behavioral models.

What it does not establish is that the true or most useful rollout environment is exactly one rung lower.

## 4. The first-book boundary

The first book is not “solve all imperfect-information 42.”

It is:

> **Build a base player that plays a lawful, strong, persistent policy under a declared model of uncertainty; knows what it has proved; knows what remains open; and spends only where the open part can still change the decision.**

The next book begins when the primary question changes from:

> How do we make a correct bounded 42 decision tractable?

into questions such as:

- How should beliefs over behavior be learned?
- How do conventions emerge?
- How should a policy explore to identify a partner or opponent?
- How should populations of strategies evolve?
- How can joint partnership prescriptions be optimized?
- How can Walt generate verified targets for learned policies?

---

# Part II — The field model as hidden state

## 5. Finite behavioral types

For each nonfocal seat \(s\), let \(\Theta_s\) be a finite set of behavioral types.

A type \(\theta_s\in\Theta_s\) specifies a seat-local policy kernel

\[
K_{\theta_s}(t\mid H_s,h),
\]

where:

- \(H_s\) is the seat’s own private hand or private state;
- \(h\) is the public record;
- \(t\) is a legal public action.

For deterministic fields,

\[
K_{\theta_s}(t\mid H_s,h)
\in\{0,1\}.
\]

A complete field type profile is

\[
\theta=(\theta_s)_{s\ne m}
\in
\Theta=
\prod_{s\ne m}\Theta_s.
\]

The first practical library might include:

- Dice;
- \(F_0\);
- \(F_1\);
- \(F_2\);
- one safety or count-preservation policy;
- and an explicit residual or `Other` model region.

The exact membership is an engineering declaration, not a theorem.

## 6. Joint physical-and-behavioral latent state

Let \(\Omega\) be the physical world set and \(\Theta\) the field-type profile set.

Define the augmented latent space

\[
\boxed{
\Xi=\Omega\times\Theta.
}
\]

If a field uses persistent randomness, adjoin a finite random tape \(z\):

\[
\Xi=\Omega\times\Theta\times Z.
\]

Let \(\mu\) be a rational belief over \(\Xi\).

A focal policy \(\rho\) remains lawful only when it is a function of the focal seat’s information state. It does not observe \(\omega\), \(\theta\), or \(z\) unless those coordinates have become inferable from public play.

Define

\[
V_\mu(\rho)
=
\mathbb E_{\xi\sim\mu}
[u_\rho(\xi)],
\]

and for root action \(a\),

\[
\boxed{
Q_a(\mu)
=
\max_{\rho\in\Pi_a}
V_\mu(\rho).
}
\]

## 7. Augmented-world reduction theorem

### Theorem 7.1

A finite model-belief Walt problem is exactly a finite fixed-semantics Walt problem on the augmented latent space \(\Xi\).

### Proof

Fix \(\xi=(\omega,\theta,z)\). The physical rules are determined by \(\omega\). Every nonfocal action is determined by the seat-local type and its persistent tape. Therefore, conditional on \(\xi\), one focal policy produces one well-defined terminal utility.

The only remaining optimization is over information-consistent focal policies, exactly as in the original finite Walt problem. Taking expectation under \(\mu\) gives the displayed value. ∎

### Consequence

Every theorem whose proof used only:

- finiteness;
- lawful focal information consistency;
- a bounded utility;
- and a fixed latent distribution;

lifts immediately to \(\Xi\).

This includes:

- Boolean response vectors;
- score profiles;
- pivotal geometry;
- calculated evidence;
- root intervals;
- salvation sets and conflicts;
- doom;
- gluing;
- certified regret;
- and exact refinement in the finite limit.

The model-belief extension is not a parallel theory. It is the same theory over a richer hidden state.

## 8. The point-mass ladder as a special case

A fixed field profile \(\theta_0\) corresponds to the point belief

\[
\nu(\theta)
=
\mathbf1\{\theta=\theta_0\}.
\]

Thus:

\[
Q_a(\delta_{F_k})

automatically reproduces ordinary best response to rung \(F_k\).

Every point-mass field result becomes:

- a parity gate for the model-belief implementation;
- a reusable response coordinate;
- and a possible upper-bound component for broader type beliefs.

The ladder is therefore demoted from ontology to basis.

It remains extremely valuable.

## 9. Type persistence is load-bearing

A model type is sampled or declared once for its intended persistence scope—normally once per seat per hand—and then remains the same latent object.

It is not resampled at every action.

For example, suppose type \(0\) always emits action sequence \(00\) and type \(1\) always emits \(11\), each with prior probability \(1/2\).

Under persistent type semantics,

\[
\Pr(00)=\frac12.
\]

If the type were incorrectly resampled independently at each action,

\[
\Pr(00)=\frac14.
\]

The latter is a different field model.

Persistence matters because earlier public actions carry information about later behavior.

## 10. Correlated behavioral types

The general belief \(\mu(\omega,\theta)\) need not factor independently across seats.

This matters for partnership conventions.

A useful structured representation introduces a shared convention latent \(\kappa\):

\[
\mu(H_1,H_2,H_3,\theta_1,\theta_2,\theta_3,\kappa)
\propto
\mathbf1\{H_1\dot\cup H_2\dot\cup H_3=U\}
\psi(\kappa)
\prod_s
\phi_s(H_s,\theta_s,\kappa).
\]

Then observing one partner’s action updates beliefs about \(\kappa\), which in turn updates beliefs about the other partner’s type.

The first implementation may assume independent finite seat types. The identity and factor interfaces should not assert that independence as a permanent theorem.

---

# Part III — Counted belief with persistent model types

## 11. Hand-type factors

The natural extension of a hand factor is

\[
\boxed{
\phi_{s,h}(H_s,\theta_s).
}
\]

At public history \(h\), define the unnormalized joint weight

\[
W_h(\mathbf H,\mathbf\theta)
=
\mathbf1\{H_1\dot\cup H_2\dot\cup H_3=U\}
\prod_s
\phi_{s,h}(H_s,\theta_s).
\]

The partition function is

\[
Z_h
=
\sum_{\mathbf H,\mathbf\theta}
W_h(\mathbf H,\mathbf\theta).
\]

The normalized posterior is \(W_h/Z_h\).

## 12. Posterior closure theorem

Suppose hidden seat \(s\) publicly plays tile \(t\).

Define

\[
\phi_{s,ht}(H,\theta)
=
\phi_{s,h}(H,\theta)
K_\theta(t\mid H,h),
\]

while every other seat factor remains unchanged.

### Theorem 12.1

The exact posterior after observing \(t\) remains in the same hand-type factor family.

### Proof

Bayes’ rule multiplies the prior joint weight by the likelihood of the observed action. Seat locality gives

\[
\Pr(t\mid\mathbf H,\mathbf\theta,h)
=
K_{\theta_s}(t\mid H_s,h),
\]

which depends only on the acting seat’s hand, type, and public record. The likelihood can therefore be absorbed into that seat’s factor. The disjoint-cover constraint and every other factor remain unchanged. Normalization produces the posterior. ∎

This is the same closure theorem that made counted physical belief possible.

## 13. Exact branch masses

For acting seat \(s\), the exact mass of public response \(t\) is

\[
Z_{ht}
=
\sum_{H,\theta}
\phi_{s,h}(H,\theta)
K_\theta(t\mid H,h)
C_{-s,h}(U\setminus H),
\]

where \(C_{-s,h}\) is the exact compatible-completion weight of the other seat factors, including their type masses.

Then

\[
\Pr(t\mid h)=\frac{Z_{ht}}{Z_h}.
\]

The hidden field branch remains a public-action branch, not a type branch.

Different types choosing the same public action stay aggregated.

## 14. Behavioral quotients

At a belief node \(B\), two types may be equivalent on the relevant reachable domain.

A strong exact equivalence is

\[
\theta\sim_B\theta'
\]

when they choose the same action at every nonfocal information state reachable from \(B\) under every surviving focal policy.

Types in one exact equivalence class may be merged by adding their weights.

A weaker, local quotient groups types only by their current public action. That grouping is exact for the immediate branch but may need to split later.

The solver should preserve the distinction:

- **global or dependency-closed behavioral equivalence** permits permanent merging;
- **current-action equivalence** permits only branch-local aggregation.

## 15. The residual model type

A finite library is a model, not reality.

The base player should not silently assume that Dice and a few response rungs exhaust all behavior.

Let known type mass be \(1-r\), with residual type region \(\mathfrak R\) of mass \(r\).

If the residual utility is only known to lie in \([0,1]\), then a known-model value interval \([L_K,U_K]\) lifts to

\[
\boxed{
(1-r)L_K
\le
V
\le
(1-r)U_K+r.
}
\]

If residual behavior is contract-stable on some cells, those cells contribute zero width despite unresolved action details.

The residual is therefore another exact-mass proof-state frontier—not an excuse to invent a complete field.

---

# Part IV — Response geometry over a model belief

## 16. Response vectors

For a fixed focal policy \(\rho\), define its model-response vector

\[
\boxed{
\mathbf v_\rho
=
\bigl(V_\theta(\rho)\bigr)_{\theta\in\Theta}.
}
\]

For model belief \(\nu\),

\[
V_\nu(\rho)
=
\langle\nu,\mathbf v_\rho\rangle.
\]

Thus fixed-policy value is linear in model belief.

The exact root-action response is

\[
\boxed{
Q_a(\nu)
=
\max_{\rho\in\Pi_a}
\langle\nu,\mathbf v_\rho\rangle.
}
\]

It is the upper envelope of finitely many linear forms.

## 17. Convexity theorem

### Theorem 17.1

For beliefs \(\nu_0,\nu_1\) and \(0\le\lambda\le1\),

\[
Q_a(\lambda\nu_0+(1-\lambda)\nu_1)
\le
\lambda Q_a(\nu_0)+(1-\lambda)Q_a(\nu_1).
\]

### Proof

For every \(\rho\), linearity gives

\[
V_{\lambda\nu_0+(1-\lambda)\nu_1}(\rho)
=
\lambda V_{\nu_0}(\rho)+(1-\lambda)V_{\nu_1}(\rho)
\le
\lambda Q_a(\nu_0)+(1-\lambda)Q_a(\nu_1).
\]

Take the maximum over \(\rho\). ∎

This means point-mass responses define a reusable convex upper surface over the model simplex.

## 18. The type-revealed point-mass upper

For each type profile \(\theta\), define its point-mass optimum

\[
q_a(\theta)
=
Q_a(\delta_\theta)
=
\max_{\rho\in\Pi_a}V_\theta(\rho).
\]

Define

\[
\boxed{
U_a^{\mathrm{sep}}(\nu)
=
\sum_\theta
\nu(\theta)q_a(\theta).
}
\]

### Theorem 18.1

\[
\boxed{
Q_a(\nu)
\le
U_a^{\mathrm{sep}}(\nu).
}
\]

### Proof

For every policy \(\rho\),

\[
\sum_\theta\nu(\theta)V_\theta(\rho)
\le
\sum_\theta\nu(\theta)q_a(\theta).
\]

Take the maximum over \(\rho\) on the left. ∎

### Interpretation

The upper \(U^{\mathrm{sep}}\) allows Walt to learn the hidden field type before choosing its complete focal continuation.

It may therefore choose a different optimal policy for each type.

That is a relaxation and hence an upper.

It is the model-type analogue of the world-revealed God upper.

## 19. Zero model-fusion price

Define the model-fusion price

\[
\boxed{
\Phi_a^{\mathrm{model}}(\nu)
=
U_a^{\mathrm{sep}}(\nu)-Q_a(\nu).
}
\]

### Theorem 19.1

Assume every type in the support of \(\nu\) has positive mass. Then

\[
\Phi_a^{\mathrm{model}}(\nu)=0
\]

if and only if there exists one lawful policy \(\rho^\star\in\Pi_a\) such that

\[
V_\theta(\rho^\star)=q_a(\theta)
\]

for every type in the support of \(\nu\).

### Proof

The reverse direction is immediate.

For the forward direction, let \(\rho^\star\) attain \(Q_a(\nu)\). Then

\[
0
=
U_a^{\mathrm{sep}}-Q_a
=
\sum_\theta
\nu(\theta)
\bigl(q_a(\theta)-V_\theta(\rho^\star)\bigr).
\]

Every summand is nonnegative and every weight is positive. Therefore every summand is zero. ∎

This is the model-space version of joint salvation.

## 20. Combining point-mass intervals

Suppose Walt has valid point-mass upper intervals

\[
q_a(\theta)
\le
U_{a,\theta}.
\]

On their simultaneous-validity event,

\[
\boxed{
Q_a(\nu)
\le
\sum_\theta
\nu(\theta)U_{a,\theta}.
}
\]

Likewise, for one common executable policy \(\rho\), any valid per-type lower evaluations combine linearly:

\[
\boxed{
L_a^\rho(\nu)
=
\sum_\theta
\nu(\theta)L_{a,\theta}^\rho
\le
V_\nu(\rho)
\le Q_a(\nu).
}
\]

If point-mass bounds are sampled, the risk ledger must cover all component claims used in the weighted result.

No policy-count penalty is introduced by the model mixture itself.

## 21. Column-and-cut form

Let \(\mathcal R\subseteq\Pi_a\) be a finite library of executable policies.

The policy-column lower is

\[
\boxed{
L_a^{\mathcal R}(\nu)
=
\max_{\rho\in\mathcal R}
\langle\nu,\mathbf v_\rho\rangle.
}
\]

The point-mass upper is \(U_a^{\mathrm{sep}}\).

Thus:

\[
L_a^{\mathcal R}(\nu)
\le
Q_a(\nu)
\le
U_a^{\mathrm{sep}}(\nu).
\]

Adding policies raises the lower.

Gluing behavioral types lowers the upper.

This is the same column-and-cut geometry already visible in the salvation complex.

## 22. Diagnosing the model-belief gap

For executable incumbent \(\widehat\rho\),

\[
U_a^{\mathrm{sep}}-V_\nu(\widehat\rho)
=
\underbrace{U_a^{\mathrm{sep}}-Q_a(\nu)}_{\text{model-fusion price}}
+
\underbrace{Q_a(\nu)-V_\nu(\widehat\rho)}_{\text{policy gap}}.
\]

This is the exact analogue of the physical doom / information price / policy gap decomposition.

The proof state can now distinguish:

- need a better common policy;
- need a tighter type-consistency upper;
- need a better posterior over models;
- or need none of those because the current contract projection is already stable.

## 23. Reuse across model beliefs

Once response vectors are available, changing the prior or posterior over behavioral types becomes a dot product.

This is a major possible win for future belief experiments.

Rather than rerunning every policy under every new behavioral prior, Walt may cache

\[
\mathbf v_\rho
\]

and reprice it under many \(\nu\).

The active optimal policy changes only when the model belief crosses a facet of the response envelope.

The model-space decision geometry is therefore directly analogous to Walt’s physical belief geometry.

---

# Part V — Type gluing and model conflicts

## 24. Partitions of the type space

Let \(\mathcal P\) be a partition of \(\Theta\).

Imagine a relaxed focal player that is told which block \(B\in\mathcal P\) contains the true type profile, but not which exact type inside that block.

Define

\[
\boxed{
U_a^{\mathcal P}(\nu)
=
\sum_{B\in\mathcal P}
\max_{\rho\in\Pi_a}
\sum_{\theta\in B}
\nu(\theta)V_\theta(\rho).
}
\]

No normalization is required in this unnormalized form.

## 25. Type-partition gluing theorem

### Theorem 25.1

Let \(\mathcal P_{\mathrm{fine}}\) refine \(\mathcal P_{\mathrm{coarse}}\). Then

\[
Q_a(\nu)
\le
U_a^{\mathcal P_{\mathrm{coarse}}}(\nu)
\le
U_a^{\mathcal P_{\mathrm{fine}}}(\nu)
\le
U_a^{\mathrm{sep}}(\nu).
\]

At the endpoints:

\[
U_a^{\{\Theta\}}(\nu)=Q_a(\nu),
\]

\[
U_a^{\{\{\theta\}:\theta\in\Theta\}}(\nu)=U_a^{\mathrm{sep}}(\nu).
\]

### Proof

A finer partition reveals more type information and allows a separate policy on more blocks. Every policy feasible under a coarser partition is feasible under the finer one, so the optimum cannot decrease under refinement. The endpoint identities follow from one shared block and singleton blocks. ∎

### Interpretation

A type-gluing step merges blocks and requires one policy to work across them.

The upper falls exactly because hidden behavioral type is no longer available as an illegal policy-selection coordinate.

## 26. Counterexample-guided type gluing

A useful loop is:

1. compute or bound the point-mass response on each active type block;
2. identify blocks whose relaxed optima use incompatible focal policies;
3. merge one or more such blocks;
4. solve the merged block response;
5. install the smaller upper;
6. repeat only while the root proof goal can still move.

An arbitrary disagreement between two selected optimizers is not enough to prove a strict gluing gain. Exact ties may admit a common optimizer not selected by either tie rule.

A valid strict-conflict witness must prove that no common policy achieves the two relevant type-specific thresholds.

The salvation-set language provides the correct witness object.

## 27. The augmented salvation complex

On augmented atom \(\xi=(\omega,\theta)\), define

\[
\mathcal S_\xi(c)
=
\{\rho:S_\rho(\xi)\ge c\}.
\]

All salvation-complex results apply with augmented mass \(\mu(\xi)\).

Thus:

- physical doom and model-specific doom are empty salvation sets;
- hidden-world conflicts and hidden-type conflicts are both higher-order nonsaveable atom sets;
- a field-model gluing cut is one information-consistency cut in the same hypergraph;
- policy columns are still lawful focal policies;
- the exact failure mass remains the minimum-weight transversal of all verified salvation conflicts.

The physical and behavioral hidden coordinates are different semantics but the same finite geometry.

## 28. Coalitions of glues

As with physical information-state gluing, one type merge may produce no immediate upper reduction while two merges together are decisive.

Therefore the scheduler must permit short type-gluing macro items.

A zero immediate gain does not imply zero prerequisite value.

The proof-state usefulness test should apply after zero-cost closure and, when declared, after a short bounded macro sequence.

---

# Part VI — Unresolved fields and sparse counterfactual thinking

## 29. Three meanings of an unresolved field

The phrase should be typed.

### 29.1 Bayesian model belief

A declared probability distribution \(\nu\) over persistent complete behavioral types.

The objective is expected value:

\[
Q_a(\nu)
=
\max_\rho
\mathbb E_{\theta\sim\nu,
\omega\sim\beta}
[u].
\]

### 29.2 Credal or robust model region

A set \(\mathcal N\) of plausible model beliefs or a set \(\Sigma\) of complete field policies.

A fixed policy can receive an interval

\[
\inf_{\sigma\in\Sigma}V(\rho,\sigma)
\le
V(\rho)
\le
\sup_{\sigma\in\Sigma}V(\rho,\sigma).
\]

A robust recommendation may maximize the lower endpoint. This is a different decision criterion and must be named.

### 29.3 Local behavioral envelope

At one nonfocal information state \(J\), the field may be known only to choose from

\[
A_\Sigma(J).
\]

Treating these local choices as independently selectable generally enlarges the set of complete fields. It is an outer relaxation suitable for bounds, not automatically the original model class.

The base player can begin with the Bayesian form and a typed residual. The other two forms remain useful upper and safety tools.

## 30. Field cylinders

A partial field cylinder \(q\) assigns actions at a finite set of nonfocal information states.

Let

\[
[q]
=
\{\sigma:\sigma\text{ is a complete field extending }q\}.
\]

At unresolved state \(J\), split by legal action:

\[
[q]
=
\dot\bigcup_t
[q\cup\{J\mapsto t\}].
\]

This is the field-side dual of a focal policy cylinder.

A finite type library induces field cylinders automatically by grouping types according to their action at \(J\).

## 31. Sparse contingency fields

Choose a cheap baseline field \(\sigma_0\).

At most nonfocal states, use \(\sigma_0\) without question.

At a small consequential set \(\mathcal J\), admit alternatives:

\[
A^\dagger(J)
=
\{\sigma_0(J)\}
\cup
\{\text{credible alternatives proposed by the model library}
\}.
\]

A complete sparse-contingency field is one persistent assignment of alternatives to the selected states.

The focal solver can then ask:

- What if partner plays the count tile here?
- What if the opponent is void and ruffs?
- What if the field retains the boss trump?
- Does that branch change score, `pmake`, or the root survivor set?

The alternative set may be generated by Dice, \(F_0\), \(F_1\), \(F_2\), motifs, human rules, or counterexamples.

Correctness comes from the declared model family or outer bound, not from the proposer.

## 32. Exact branch-by-public-action rule

At a hidden field node, different types and hands must first be grouped by the public action they produce.

For each action \(t\), define the exact posterior branch

\[
B_t
=
B\mid\{K_\theta(t\mid H,h)>0\}.
\]

The focal continuation may condition on \(t\) because \(t\) is public.

It may not condition on the hidden type identity among types that produced the same \(t\).

Thus the correct recursion is

\[
M(B)
=
\sum_tM(B_t),
\]

not a sum over types with an independent focal maximization inside each type.

This is merge-before-max in model space.

## 33. The model-disagreement frontier

For active type support \(\Theta_B\), define

\[
\boxed{
\mathcal F_{\Theta}(B)
=
\left\{
J:
\left|
\{\sigma^\theta(J):\theta\in\Theta_B\}
\right|>1
\right\}.
}
\]

Before play reaches this frontier, every active type produces the same public actions.

Therefore type uncertainty is behaviorally dormant.

If the frontier is unreachable under a focal policy, that policy has identical value under every active type.

This gives the first targeting rule:

> **Do not spend model-belief compute before a reachable field disagreement.**

## 34. Mixture-to-reference transfer bound

Choose reference type \(\theta_0\).

For fixed focal policy \(\rho\), let

\[
r_\rho(\theta,\theta_0)
=
\Pr_\omega
\bigl(
 u_\rho(\omega,\theta)
e
 u_\rho(\omega,\theta_0)
\bigr).
\]

Then

\[
\boxed{
|V_\nu(\rho)-V_{\theta_0}(\rho)|
\le
\sum_\theta
\nu(\theta)r_\rho(\theta,\theta_0).
}
\]

For root action \(a\), define

\[
R_a^{\nu,\theta_0}
=
\sup_{\rho\in\Pi_a}
\sum_\theta
\nu(\theta)r_\rho(\theta,\theta_0).
\]

Then

\[
\boxed{
|Q_a(\nu)-Q_a(\delta_{\theta_0})|
\le
R_a^{\nu,\theta_0}.
}
\]

### Proof

The fixed-policy bound follows because Boolean utility difference is bounded by the outcome-disagreement indicator. For optimized values, use

\[
|\max f-\max g|
\le
\max|f-g|.
\]

∎

The existing field-swap exposure and outcome-change machinery can upper-bound \(R_a^{\nu,\theta_0}\).

This allows a point-mass solve to seed a model-belief interval without restarting from \([0,1]\).

## 35. Active information value

A persistent model belief creates genuine planning value.

A focal action may be useful because it induces a public response whose distribution differs across behavioral types. The response updates the posterior, which changes later focal choices.

Define a restricted `commit` policy class that may not condition on future model-revealing observations. Let

\[
Q^{\mathrm{commit}}(B)
\]

be its optimum and \(Q(B)\) the ordinary model-belief optimum.

Then

\[
\boxed{
\operatorname{VOI}_{\mathrm{model}}(B)
=
Q(B)-Q^{\mathrm{commit}}(B)
\ge0.
}
\]

The nonnegativity is simply policy-class inclusion.

This is the first object in current Walt that behaves like the targeted human question:

> What can I learn from what they do next, and will that knowledge change my play?

A point-mass field has zero uncertainty about model type and therefore cannot express this particular value of information.

## 36. Decision-relevant model uncertainty

Raw posterior entropy over behavioral types is not the target.

Two types may have different internal computations and still:

- choose the same action;
- produce different actions with identical score consequences;
- or differ only on states that cannot be reached.

The useful quantity is the part of model uncertainty that can still change:

- the score envelope;
- the contract projection;
- the root upper;
- or the executable recommendation.

Thus model refinement should be scheduled by proof-state effect, not by classification accuracy.

---

# Part VII — The upper portfolio after model belief

## 37. Upper sources

For model-belief root action \(a\), Walt may have several valid uppers:

\[
U_a^{\mathrm{sample}},
\quad
U_a^{\mathrm{doom}},
\quad
U_a^{\mathrm{sep}},
\quad
U_a^{\mathrm{type\ glue}},
\quad
U_a^{\mathrm{world\ glue}},
\quad
U_a^{\mathrm{residual}},
\quad
U_a^{\mathrm{count}},
\quad
U_a^{\mathrm{field\ transfer}}.
\]

The installed upper is

\[
\boxed{
U_a
=
\min_jU_{a,j}.
}
\]

Each producer attacks a different relaxation.

## 38. Doom in augmented space

An augmented atom \(\xi=(\omega,\theta)\) is doomed if even a world-and-type-revealed focal continuation cannot make the contract.

This produces the strongest physical-and-model-specific singleton conflict.

But as before, individually saveable augmented atoms may be jointly incompatible under one focal policy.

Doom is the first cut, not the complete upper.

## 39. Salvation masks with model types

At focal information state \(I\), partition the current augmented belief into exact-mass cells \(C\).

For each cell define the rescuing-action mask

\[
\mathcal A_C(c)
=
\{a:\overline S(C,a)\ge c\},
\]

where \(\overline S(C,a)\) is a valid optimistic score ceiling after forcing action \(a\).

Then the one-state gluing upper is

\[
\boxed{
U_I
=
\max_a
\sum_C
\mu(C)
\mathbf1\{a\in\mathcal A_C(c)\}.
}
\]

The cells may differ by physical hand, behavioral type, or both.

The empty mask is doom.

Conflicting singleton masks produce information cuts.

## 40. Physical and model gluing commute only semantically

World gluing and type gluing are both restrictions on which hidden coordinates may select different focal continuations.

They can be represented in one augmented salvation complex.

But an implementation should retain their provenance:

- a physical-world cut says one action must cover indistinguishable deals;
- a model-type cut says one action must cover indistinguishable behavioral hypotheses;
- a joint cut may involve both.

The mathematics permits one common cut ledger. Explanation and reuse require typed origins.

## 41. Count-aware model cuts

A model disagreement should not be refined merely because the actions differ.

If every action in the disagreement class leaves the score on the same side of the contract, then its `pmake` width is zero.

For exact-mass augmented cell \(C\) with score envelope

\[
\ell_C\le S\le u_C,
\]

only

\[
\ell_C<c\le u_C
\]

contributes contract-sensitive width.

Thus the same behavior-to-score-to-contract filtration applies after adding model types.

A type can remain unresolved forever if its uncertainty cannot cross the contract.

## 42. Fusion horizons under a model belief

A belief node is model-God-tight when an executable common policy attains the augmented world-and-type-revealed upper.

If a broad suffix is model-God-tight, the early solver may substitute exact continuation values and extracted policies at the frontier.

This generalizes the physical fusion horizon.

The useful census is not merely:

- by trick, how much doom exists?

It is:

- by trick and model belief, how large is the revealed-information gap?
- how often is one policy optimal across all active types?
- how much of the suffix can be replaced by exact common continuation policies?

This may be one of the largest remaining performance wins.

---

# Part VIII — The base player

## 43. Definition of the gloriously boring player

A base Walt decision is **boring in the desirable sense** when it:

1. plays one lawful materialized policy;
2. maximizes a certified `pmake` floor among the policies it can actually execute;
3. carries a global upper on the unknown best response;
4. reports certified regret;
5. does not collapse behavioral uncertainty without evidence;
6. does not explore a counterfactual unless it can still affect the declared goal;
7. prefers score-safe and contract-stable lines under a declared secondary rule;
8. becomes exact in affordable suffixes;
9. and labels every residual heuristic choice honestly.

The player may still make an informed bet.

It simply does not make an unpriced one.

## 44. Primary decision criterion

Let \(\mathcal E\) be the current materialized executable policy set.

Choose

\[
\widehat\rho
\in
\arg\max_{\rho\in\mathcal E}
L_\nu(\rho).
\]

Let

\[
B_{\mathrm{exec}}
=
L_\nu(\widehat\rho),
\]

and

\[
U^\star
=
\max_aU_a.
\]

Then

\[
0
\le
Q^\star-V_\nu(\widehat\rho)
\le
U^\star-B_{\mathrm{exec}}.
\]

The right side is the certified regret under the declared physical-and-model belief.

## 45. Secondary ordering

Among policies with equal primary lower floor, a declared conservative order may prefer:

1. smaller contract-sensitive residual;
2. higher worst-case declaring score floor;
3. smaller fragile-make mass;
4. smaller robust regret over the residual model class;
5. higher expected declaring score;
6. a stable deterministic tile order.

Expected points remain secondary to `pmake` unless the game mode declares another objective.

## 46. Model-belief base algorithm

At one root:

1. restore or construct the physical proof state;
2. attach the declared persistent model library and prior;
3. apply zero-cost physical, score, model-agreement, and late-suffix closure;
4. import point-mass response intervals for active model types;
5. form the type-revealed upper;
6. evaluate strong common executable policies across the model belief;
7. update root intervals and certified regret;
8. branch only on public actions produced by active types;
9. refine only reachable model disagreements that remain contract-sensitive;
10. add doom, salvation-mask, count, type-gluing, and world-gluing uppers where useful;
11. extract a better common policy when lower work has greater value;
12. stop at exactness, declared \(\epsilon\)-regret, practical equivalence, or budget exhaustion;
13. play the best executable floor and persist the entire proof state.

## 47. A natural small model library

The first base field may use

\[
\Theta_{\mathrm{base}}
=
\{D,F_0,F_1,F_2\},
\]

possibly augmented by one safety policy and one residual type.

This choice is not sacred.

Its virtues are:

- every element already has operational meaning;
- point-mass parity is testable;
- the types are ordered by a known response construction;
- their disagreements identify plausible consequential counterfactuals;
- and project level 2 contains the desired reciprocal-thinking partnership depth.

The library can later expand through population methods or learned models.

## 48. Book-one completion criteria

The first Walt book may close when all of the following hold.

### Mathematical

- model-belief value is defined on the augmented latent space;
- point-mass fields reproduce existing Walt exactly;
- persistent type posterior updates are exact;
- a valid type-revealed upper is installed;
- at least one type-gluing or model-conflict upper is operational;
- the residual model class is typed and never silently discarded;
- executable regret is valid under the joint physical/model belief;
- fallback behavior is never presented as a settled theorem.

### Engineering

- no new duplicated rules or game semantics;
- existing physical fibers, factor beliefs, fields, policies, and evidence engines are reused;
- model type is an identity-bearing latent coordinate;
- public-action branches aggregate types correctly;
- pause/resume preserves posterior and proof facts exactly;
- exact small fixtures agree with augmented-state enumeration;
- point-mass parity agrees with every existing field fixture;
- opening decisions run within the declared deployment budget.

### Empirical

- the base player is stronger than the current default in a predeclared mirrored arena;
- its latency distribution is tractable on the target hardware;
- its unresolved rate and regret distribution are reported by trick and contract;
- model posterior movement and disagreement spend are measurable;
- no performance claim outruns the declared field belief or arena corpus.

## 49. What book one does not require

The following may remain open:

- a universally correct prior over human behavior;
- convergence of an infinite response ladder;
- equilibrium play;
- a complete learned field model;
- a globally exact opening solution;
- a full joint partnership solve;
- or proof that the selected model library contains reality.

The base player is complete as an engineering and mathematical object when it handles those absences honestly.

---

# Part IX — Engineering program

## 50. Architectural ruling

Do not create a separate third Walt.

Extend the proof-state core with a model-belief axis and adapt existing authorities through narrow interfaces.

Reuse:

- canonical roots and fibers;
- `FactorBelief` and exact-cover oracles;
- `FieldModel` and `FieldId`;
- frozen policies and extracted policies;
- score profiles;
- root interval and evidence machinery;
- residual Bellman;
- count-threat covers;
- doom;
- salvation and gluing mathematics;
- and the persistent fact registry.

The new code should own:

- model type identities;
- type priors and posteriors;
- hand-type factors;
- point-mass response vectors;
- type-partition uppers;
- field-region frontiers;
- and sparse counterfactual scheduling.

## 51. Proposed identities

```text
BehaviorTypeId = content address of:
    complete field construction,
    parent field identity if response-generated,
    seed and tape semantics,
    tie rule,
    fallback,
    solve budget,
    quality claim,
    persistence scope.

ModelBeliefId = content address of:
    type registry,
    prior or posterior weights,
    correlation/convention factors,
    root physical identity,
    public history,
    update semantics.
```

Changing any behavior-affecting coordinate creates a new identity.

## 52. Proposed data types

```rust
struct BehaviorType {
    id: BehaviorTypeId,
    field: FieldId,
    quality: ResponseQuality,
}

enum ResponseQuality {
    ExactBestResponse,
    DeltaBestResponse { delta: ScopedDelta },
    EpsilonBestResponse { epsilon: Rational },
    Heuristic,
}

struct TypeWeight {
    type_id: BehaviorTypeId,
    weight: Rational,
}

struct ModelBelief {
    root_id: RootId,
    public_state: PublicStateId,
    type_profile_factors: Vec<TypeFactor>,
    residual: Option<ModelResidual>,
}

struct HandTypeFactor {
    seat: Seat,
    capacity: u8,
    weights: HandTypeRepresentation,
}

struct ModelResponseVector {
    policy: PolicyId,
    values: Vec<(BehaviorTypeProfileId, RationalInterval)>,
}

enum ModelUpper {
    PointMassSeparated,
    TypePartition { partition: TypePartitionId },
    ModelConflict { witness: ConflictId },
    FieldTransfer { reference: FieldId },
    ResidualEnvelope,
}

struct FieldRegion {
    constraints: Vec<FieldActionConstraint>,
    mass_or_credal_weight: RegionWeight,
}
```

## 53. Slice M0 — model registry and point-mass table

### Build

- Register Dice, \(F_0\), \(F_1\), and \(F_2\) as identity-bearing field types.
- Give each type an explicit persistence scope.
- Produce exact or typed interval point-mass root responses on the affordable corpus.
- Produce response vectors for existing executable policies.

### Gates

- point prior on each type reproduces the existing fixed-field value and selected policy;
- identities change under every behavior-changing parameter;
- two behaviorally identical types may be quotient-merged only under an explicit domain receipt;
- no point-mass quality claim is inferred from construction depth alone.

### Probe

Report:

- pairwise field disagreement by trick;
- response/value/decision wake-up;
- common-optimal-policy rate across type subsets;
- type-revealed upper versus each point-mass value;
- response-vector reuse across priors.

## 54. Slice M1 — persistent hand-type factor belief

### Build

Extend one hidden-seat factor from \(H\) to \((H,\theta)\).

Then extend all hidden seats under an independent type prior.

Condition by observed public action using Theorem 12.1.

### Gates

- exact parity with explicit augmented-state enumeration on small fibers;
- mass conservation;
- point prior collapse to ordinary `FactorBelief`;
- persistent-type sequence probabilities differ from action-wise resampling on the declared fixture;
- posterior type weights update correctly after observed actions;
- same-action types remain aggregated in one public branch;
- hidden type is inaccessible to focal policy keys.

## 55. Slice M2 — model-belief fixed-policy and grammar recursion

### Build

- exact fixed-policy score profile under the finite model belief;
- exact grammar response under the finite model belief;
- extracted executable grammar policy;
- certified regret using a vacuous or point-mass upper.

### Gates

- parity with augmented-world enumeration;
- fixed-policy linearity in type weights;
- public-action merge before focal maximization;
- extracted policy re-prices unchanged;
- score profile projects to `pmake` exactly.

## 56. Slice M3 — type-revealed upper

### Build

For each root action:

\[
U_a^{\mathrm{sep}}
=
\sum_\theta\nu(\theta)U_{a,\theta}.
\]

Install it as a typed upper producer.

### Gates

- covers the exact finite-mixture response on every small fixture;
- equality when one common policy is pointwise optimal;
- strict gap on an opposed-types fixture;
- risk ledger includes every sampled point-mass component consumed;
- minimum with existing doom/sample/count uppers remains valid.

### Probe

At the opening root, compare:

- current sampled upper;
- doom upper;
- point-mass separated upper;
- best executable mixture policy lower;
- resulting certified regret.

This is the first direct test of whether the ladder’s point solutions are reusable enough to lower the unresolved-field upper.

## 57. Slice M4 — type-partition gluing

### Build

- represent a partition of active type profiles;
- solve or bound one common response inside each block;
- merge selected blocks;
- install the resulting upper;
- retain a conflict or non-conflict trace.

### Gates

- coarsest partition equals the exact finite-mixture response;
- finest partition equals the point-mass separated upper;
- coarsening never raises the upper;
- arbitrary optimizer disagreement cannot produce a conflict receipt;
- a common optimizer produces zero gluing loss;
- a two-type opposed-response fixture lowers strictly.

### Scheduler

Rank type merges by potential root-upper reduction per cost, but permit short coalition macros.

## 58. Slice M5 — sparse counterfactual field refinement

### Build

At every reached nonfocal information state:

1. query active types;
2. group them by public action;
3. apply score/contract projection;
4. refuse the disagreement if it cannot alter the goal;
5. otherwise branch, update the posterior, and continue.

Allow a baseline field plus a small persistent contingency family proposed by the type library.

### Gates

- if all types agree, the model-aware recursion equals the representative point field;
- branching occurs by public action, never hidden type label;
- one consequential disagreement can create positive value of information;
- unresolved residual mass widens the result only by its valid envelope;
- counterfactual refinement narrows root intervals monotonically.

## 59. Slice M6 — residual model and robust report

### Build

- explicit `Other` model region;
- declared mass or credal range;
- lower/upper score and `pmake` contribution;
- robust secondary recommendation among policies with similar Bayesian floor.

### Gates

- no residual mass is silently dropped;
- zero residual reproduces the finite library;
- contract-stable residual contributes zero `pmake` width;
- worst-case and Bayesian criteria remain different result types.

## 60. Slice M7 — the base-player run

Run the integrated model-belief proof state over:

- exact late fixtures;
- the existing trick-4/5 transition corpus;
- count-timing fixtures;
- the Gran reveal/retain specimen when reconstructed;
- and opening roots.

Per decision report:

- physical fiber size;
- active type support;
- posterior type weights;
- distinct public actions among types;
- model-disagreement exposure;
- point-mass upper;
- glued upper;
- executable lower;
- certified regret;
- score and contract residual;
- counterfactuals expanded and refused;
- wall time by producer;
- result type.

Then run the predeclared mirrored strength and latency arena.

## 61. Freeze condition for `BaseWaltV1`

Freeze a first base player when:

- its semantics identity is complete;
- point-mass and mixture parity gates are green;
- its model residual is explicit;
- its fallback is executable and regret-bounded;
- its target-device budget is met on a declared percentile;
- it beats or materially improves on the current default under a predeclared arena;
- and its proof-state record is sufficient to reproduce every decision.

The name `BaseWaltV1` should mean a frozen mathematical contract, not the end of research.

---

# Part X — The joint partnership bridge

## 62. Why the partner may not belong in the field forever

The focal player and partner share the same team utility.

Modeling the partner as an external field is computationally convenient, but conceptually indirect.

A more natural team objective is

\[
\boxed{
\max_{\rho_m,\rho_p}
\mathbb E
[u(\rho_m,\rho_p,\sigma_{\mathrm{opp}})]
}
\]

subject to each teammate using only that teammate’s information.

This makes signaling endogenous rather than an emergent consequence of nested best responses.

## 63. Common-information prescriptions

At public history \(h\), a public coordinator knows the common belief but not either teammate’s private hand.

For acting teammate \(s\), the coordinator selects a prescription

\[
\gamma_s:
\mathcal H_s(h)	o A_s,
\]

mapping each possible private hand to a legal action.

The teammate applies \(\gamma_s\) to the hand they actually hold.

This is not telepathy:

- the prescription is selected from public information;
- the seat supplies its own private hand;
- the resulting action is public.

The exact prescription space is enormous.

But the current machinery already studies compressed mappings from hand classes to actions.

## 64. Prescription grammars

A tractable first joint-team approximation can restrict prescriptions to actions proposed by:

- Dice;
- \(F_0\);
- \(F_1\);
- \(F_2\);
- safety policies;
- count-aware policies.

At hand class \(C\), define

\[
G_{\mathrm{team}}(C)
=
\{\text{actions proposed by source policies on }C\}.
\]

Optimize jointly inside this prescription grammar.

Then upper-bound off-grammar prescriptions through the same residual and salvation machinery.

This is the natural bridge from the ladder to a team solve.

## 65. Why joint partnership can wait

The finite model-belief base player already provides:

- persistent hypotheses about partner behavior;
- posterior updating;
- targeted signaling analysis;
- value of information;
- and a path to level-2 reciprocal cognition.

Those capabilities are sufficient to finish the first goal.

A joint partnership solve should begin as a small exact late-game laboratory after the model-belief base is stable.

The architecture should keep:

- public common belief;
- hand-class prescriptions;
- and partner/opponent identity distinctions

available for that future work.

---

# Part XI — The new book

## 66. Beliefs

Once the finite model-belief player exists, the next research questions include:

- learning priors from play;
- posterior calibration;
- distinguishing stable type from transient action noise;
- correlated partnership conventions;
- and transferring beliefs across opponents, tables, and formats.

The response-vector geometry makes repeated belief experiments cheap.

## 67. Dynamics

A policy population may evolve through:

\[
\mathcal P_{k+1}
=
\mathcal P_k
\cup
\{\operatorname{BR}(\mu_k)\},
\]

where \(\mu_k\) is a distribution over the current population.

The ordinary ladder is the special case in which \(\mu_k\) is a point mass on the most recent policy.

Population methods can retain cycles rather than declaring them failures.

## 68. Learning and RLVR

Walt can become an oracle that supplies:

- exact or bounded targets;
- extracted policies;
- certified regret;
- hard salvation conflicts;
- consequential hidden-state features;
- counterfactual response traces;
- exact late-game continuations;
- and examples where intuitive policies fail.

A learned policy may compress that information into a much faster player.

The learned player should be judged against Walt’s proof-state output, not only against raw outcomes.

## 69. The real transition

The first book asks:

> Can a bounded, correct imperfect-information solver become playable?

The second asks:

> What should a thinking partnership believe, learn, signal, explore, and become?

The model-belief extension is the final bridge between those books.

---

# Part XII — Proof obligations

## 70. Mathematical obligations

**MB-O1 — Augmented-world reduction.** Formalize Theorem 7.1 for finite physical worlds, finite type profiles, and finite persistent tapes.

**MB-O2 — Point-mass parity.** A delta type belief recovers the original fixed-field Walt value.

**MB-O3 — Hand-type posterior closure.** Prove Theorem 12.1 under seat-local action kernels.

**MB-O4 — Exact branch mass.** Prove \(Z_{ht}/Z_h\) is the conditional public-action probability.

**MB-O5 — Fixed-policy model linearity.** \(V_\nu(\rho)\) is linear in \(\nu\).

**MB-O6 — Response convexity.** Prove Theorem 17.1.

**MB-O7 — Type-revealed upper.** Prove Theorem 18.1.

**MB-O8 — Zero model-fusion characterization.** Prove Theorem 19.1.

**MB-O9 — Point-mass interval combination.** Weighted simultaneous point-mass uppers cover the model-belief response.

**MB-O10 — Type-partition lattice.** Prove Theorem 25.1.

**MB-O11 — Augmented salvation complex.** Lift maximum-weight face and minimum-transversal results to \(\Omega\times\Theta\).

**MB-O12 — Persistent-type semantics.** Distinguish persistent type from action-wise model resampling.

**MB-O13 — Behavioral quotient.** Prove exact value preservation under a declared dependency-closed equivalence.

**MB-O14 — Residual model envelope.** Exact residual mass with bounded utility contributes at most its mass to interval width.

**MB-O15 — Model-to-reference transfer.** Prove the fixed-policy and optimized bounds of §34.

**MB-O16 — Public-action merge.** Hidden types producing the same public action must remain merged before focal maximization.

**MB-O17 — Sparse field cylinder cover.** Field-cylinder children form a complete disjoint cover of their parent model region.

**MB-O18 — Model information value.** Prove nonnegativity by policy-class inclusion.

**MB-O19 — Type-gluing risk composition.** Sampled point-mass components and block solves consume one explicit model-upper ledger.

**MB-O20 — Common-information prescription equivalence.** For a finite partnership team, formalize the correspondence between decentralized policies and public-history prescription policies before any joint solve is promoted.

## 71. Implementation obligations

**MB-I1 — No hidden-type policy key.** Focal policy identity and action lookup cannot read the true model type.

**MB-I2 — One persistent type per declared scope.** No action-wise resampling under a hand-persistent type identity.

**MB-I3 — Full model identity.** Priors, type registry, correlation structure, persistence, seeds, fallback, and quality claims are identity.

**MB-I4 — Public-action aggregation.** Branch by tile/action, not by hidden type ID.

**MB-I5 — Point-mass exact parity.** Every type point prior reproduces its current field authority.

**MB-I6 — Model posterior conservation.** Branch masses sum exactly to the parent mass.

**MB-I7 — Residual honesty.** Unknown model mass is stored and propagated, never discarded.

**MB-I8 — Upper provenance.** Type-revealed, type-glued, doom, count, and transfer uppers remain separately typed.

**MB-I9 — Executable witness.** A model-belief lower enters the executable bar only with a materialized policy.

**MB-I10 — Resume fidelity.** Serialized model posterior and proof facts reproduce uninterrupted execution bytewise or semantically under the declared canonical format.

## 72. Empirical questions

1. How often do Dice, \(F_0\), \(F_1\), and \(F_2\) disagree at reachable states?
2. How often do those disagreements alter score, contract, or root decision?
3. How quickly do observed plays concentrate posterior type mass?
4. How loose is the point-mass separated upper?
5. How much does the first type merge lower it?
6. Does one policy often attain the point-mass optimum across several types?
7. Does a small sparse contingency set recover most of the benefit of full \(F_2\)?
8. Does model-belief active information change focal play in meaningful positions?
9. How large must the residual `Other` mass be before robust fallback behavior changes?
10. At what trick does the model-God gap usually become zero?

## 73. Falsifiers

The approach may fail economically if:

- point-mass separated uppers remain near one everywhere;
- no small type partition gives useful gluing reductions;
- active types disagree at nearly every reachable state;
- most disagreements remain contract-sensitive;
- hand-type contraction multiplies cost by the full type-profile count;
- posterior type beliefs do not concentrate;
- the residual `Other` class dominates every useful bound;
- sparse contingencies approach a complete field policy;
- or the common policy needed across types is much weaker than every point response.

None of these falsifiers threatens correctness.

They determine whether the finite model-belief layer belongs in the base player or only in the research laboratory.

---

# Part XIII — First implementation session

## 74. Bounded assignment

The first session should not build the full base player.

Build one exact finite-type vertical slice:

1. Register two existing deterministic fields as persistent `BehaviorType`s.
2. Define a rational prior \((1/2,1/2)\).
3. Extend one hidden-seat factor to \((H,\theta)\).
4. Compute exact public-action branch masses.
5. Condition the posterior after one observed action.
6. Evaluate one frozen focal policy through one full small root.
7. Verify parity against explicit enumeration of \((\omega,\theta)\).
8. Verify both point-mass endpoints reproduce existing Walt.
9. Compute the point-mass separated upper.
10. Compute the exact mixture response on the small root.
11. Record the model-fusion gap.
12. Return no live-player change.

## 75. Required first report

For each test root report:

- physical world mass;
- augmented world/type mass;
- type prior;
- active types by seat;
- branch masses by public action;
- posterior type weights after each action;
- fixed-policy mixture value;
- exact mixture best response;
- point-mass separated upper;
- model-fusion price;
- distinct type actions and merged public branches;
- wall time and memory;
- exact parity result.

## 76. First go/no-go criterion

Proceed to the full model-belief base only when:

- point-mass parity is exact;
- persistent posterior closure is exact;
- the mixture response differs nontrivially from at least one point-mass response on a specimen;
- the point-mass upper is sometimes strict but finite and interpretable;
- and the type dimension remains small after public-action aggregation.

If the exact vertical slice does not show those properties, retain the ladder as the base field and use model belief only as an offline research instrument.

---

# 77. Final perspective

Walt began as a best response to a simple field.

It then became:

- an information-consistent policy optimizer;
- a counted belief engine;
- a score-aware proof state;
- a column-and-cut solver;
- and a salvation-conflict geometry.

The next abstraction does not discard any of that.

It changes one point-mass assumption:

\[
\text{the field is known}
\]

into

\[
\text{the field is another persistent hidden coordinate}.
\]

Once that change is made:

- the ladder becomes a model basis;
- point responses become reusable upper data;
- public actions become evidence about behavior;
- targeted “what if” reasoning becomes exact posterior branching;
- model uncertainty can remain unresolved when it does not affect the contract;
- and sparse counterfactual search becomes a first-class proof-state operation.

The base player need not understand all of 42.

It needs to know:

- what it believes;
- what it can prove;
- what could still overturn the play;
- and where another unit of thought is worth spending.

That is close to the dream.

The first book can end with a player that is gloriously boring because its caution is informed, its bets are priced, and its uncertainty is carried rather than hidden.

The next book begins when Walt stops merely responding to a world model and starts learning, questioning, coordinating, and changing that model.
