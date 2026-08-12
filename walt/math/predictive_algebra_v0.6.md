# Straight Texas 42 Predictive Algebra
## Scheme-Generated Outcome Coordinates and the Opening-Hand Experiment

**Status:** experimental implementation handoff, v0.1  
**Date:** 2026-08-12  
**Primary dependency:** `straight_42_unified_information_geometry_v0.4.md`  
**Additional assumed theorem:** the post-v0.4 extension “Equivariant controlled lumpability over declared role interfaces,” intended as §12.6A  
**Purpose:** state the next mathematical object and the exact experiments needed to test whether a broad opening fiber can be evaluated without enumerating its concrete worlds  
**Non-purpose:** restate the physical rules, support normal form, Scheme/Fix semantics, belief model, policy-gluing theory, valuation gauge, or the full controlled-lumpability development  
**Exact-coefficient scope:** the first experiments and certificates below use rational transition probabilities and linear algebra over $\mathbb Q$. If a selected field introduces other exact coefficients, replace $\mathbb Q$ by the exact coefficient field it generates.

---

# 0. Executive statement

The native implementation has reported three important facts. They are treated here as experimental inputs to the next program, not as independently promoted theorems of v0.4.

1. Endgame continuation states compress strongly.
2. A completed trick admits a small, reusable class structure; the current implementation reports 64 count-free trick classes.
3. Directly nesting forward hidden-world expansion with backward continuation classes causes the two recursions to meet in fully specified coordinates. At that meeting point, ordinary partition compression disappears.

These facts do **not** imply that the opening hand requires one computation per hidden world. They imply that the wrong basis was used at the join.

The direct implementation represents a belief and a continuation value in the delta basis of concrete worlds:

\[
J_\rho(B)
=
\sum_{\xi\in X}
\underbrace{\beta(\xi)}_{\text{forward coordinate}}
\underbrace{V_\rho(\xi)}_{\text{backward coordinate}}.
\]

That basis has one coordinate per latent world. A different exact factorization may exist:

\[
\boxed{
J_\rho(B)
=
\psi(B)\,c_\rho,
}
\]

where

- \(\psi(B)\in\mathbb Q^r\) is a small vector of predictive moments of the opening belief;
- \(c_\rho\in\mathbb Q^r\) is a dual continuation-policy vector;
- \(r\) is the rank of the selected controlled outcome theory, not the number of concrete worlds and not the number of partition classes.

The new target is therefore:

\[
\boxed{
\text{exact support fiber}
\xrightarrow{\text{Scheme moments}}
\text{predictive coordinate}
\xrightarrow{\text{dual policy geometry}}
\text{policy-indexed outcome laws}.
}
\]

The 64 trick classes should be used primarily as a **typed transition alphabet and operator library**, not as the state partition of the opening solve.

The core experimental question is no longer

\[
\text{“How many opening hands are isomorphic?”}
\]

but

\[
\boxed{
\text{“What is the exact controlled outcome rank of the opening decision problem?”}
}
\]

A domain can have one behavioral class per concrete hand and still have small linear predictive rank.

---

# 1. Relation to v0.4

This handoff begins where v0.4 leaves the dynamic-compression program. It depends on, but does not reproduce, the following components.

| v0.4 component | Used here as |
|---|---|
| §2 exact support and decision state | authoritative latent domain and initial belief |
| §3–§5 Scheme/Fix, output interfaces, and answer lenses | language for defining quantitative observables |
| §6 rigid role transport and exact forward dynamics | typed transport across one-trick operators |
| §7 universal policy-indexed terminal laws | semantic target being preserved |
| §8 additive outcome and valuation gauge | valuation readout after outcome preservation |
| §9 expected additive policy geometry | fixed-belief outcome-side geometry |
| §10 policy gluing by information partitions | prohibition on worldwise continuation choices |
| §12.5 dynamic control skeleton | intended relational state language |
| §12.6 strong controlled lumpability | positive one-hot special case of the new linear theory |
| proposed §12.6A equivariant lumpability | declared role, action, observation, and coordinate transports |

The new material is narrower:

1. quantitative Scheme observables;
2. controlled continuation-test spaces;
3. exact finite predictive rank;
4. forward moment coordinates and backward policy coordinates;
5. outcome-generating operators;
6. stopping-time monitors such as control return;
7. an exact experiment and certificate program.

Nothing here changes the Straight 42 rule algebra or licenses a coarser support object. The exact support normal form remains authoritative. The proposed predictive coordinate is a purpose-relative strategic factor of the latent process, not a replacement for legal state semantics.

---

# 2. The clarified compression picture

Three different compression mechanisms are now visible.

## 2.1 Local transition compression

A concrete four-play trick can be canonicalized into one of the reported 64 count-free trick classes, with the required seat, pip, context, action, observation, and output-role transports.

This reduces the number of distinct **local transition templates**.

It does not follow that there are only 64 continuation states.

## 2.2 Predictive-state compression

A belief over a broad hidden fiber may be replaceable, for a selected outcome contract, by finitely many exact expectations:

\[
\psi(B)
=
\left(
\mathbb E_\beta[f_1],
\ldots,
\mathbb E_\beta[f_r]
\right).
\]

This reduces the dimension of the **forward information message**.

The observables \(f_i\) may overlap heavily. They need not define a partition.

## 2.3 Decision-geometry compression

An enormous information-consistent policy set may induce only a small number of distinct or exposed continuation vectors.

This reduces the size of the **backward decision message**.

The final root action value is the pairing of the forward predictive coordinate and a backward policy vector:

\[
Q_B(a)
=
\max_{c\in C_{B,a}}
\psi(B)c.
\]

These mechanisms stack:

\[
\boxed{
\begin{array}{c}
\text{64-class local operator reuse}\\[1mm]
+
\\[-1mm]
\text{low-dimensional predictive coordinates}\\[1mm]
+
\\[-1mm]
\text{small exposed policy geometry}.
\end{array}
}
\]

The native traversal asked the first mechanism to solve all three problems. It cannot. The next implementation should give each mechanism its own job.

---

# 3. Typed trick-boundary process

The cleanest first experiment is graded by completed tricks and fixed to one declared continuation operator, such as a fixed stochastic field for the nonfocal seats.

## 3.1 Interface nodes

Let \(\mathfrak i\) denote a typed focal information interface at a trick boundary. It carries everything that must remain literally or transportably available to the player and compiler, including:

- declaration and oriented seat frame;
- focal remaining hand;
- exact public trick history retained by the selected information model;
- exact support/evidence interface needed to reconstruct the latent fiber;
- legal local control labels;
- declared output-role interface;
- grade, meaning tricks remaining;
- selected outcome-monitor state.

Let

\[
X_{\mathfrak i}
\]

be the finite augmented latent fiber over that information interface. A point \(\xi\in X_{\mathfrak i}\) contains the hidden current remainder and any latent field state required by v0.4 §2 and §7.

All points in one interface fiber have the same focal legal-action labels, up to a declared bijective transport. This is a typing requirement, not a learned property.

## 3.2 One-trick focal controls

At a trick boundary the focal seat may not be the leader. Its play can depend on the public plays observed before its turn.

Therefore the macro control alphabet is not always “choose one tile now.” Let

\[
U(\mathfrak i)
\]

be the finite set of lawful **one-trick local controllers**. A controller chooses the focal play at every focal information state reachable before the current trick completes, while making one common choice at indistinguishable nodes.

When the focal seat leads, a root tile action can be represented as a restriction on \(U(\mathfrak i)\).

## 3.3 Transition and output labels

Let

\[
\Gamma_{64}=\{1,\ldots,64\}
\]

be the reported count-free trick-class alphabet.

Keep three labels distinct:

1. \(\gamma\in\Gamma_{64}\): normalized local trick structure;
2. \(o\in\mathsf{Obs}(\mathfrak i,u)\): what the focal player is declared to observe;
3. \(r\in\mathsf{Inc}\): the selected universal outcome increment.

The player may observe enough public information to reconstruct \(\gamma\), but the mathematics must not assume that every analyst output is a player observation. Policy branching is allowed only on \(o\).

The one-trick macro-kernel is

\[
K_{\mathfrak i,u}
(\xi;\gamma,r,o,\xi'),
\]

where \(\xi'\in X_{\mathfrak j(o)}\) and \(\mathfrak j(o)\) is the successor information interface determined by the public observation.

This kernel is compiled from the primitive legal four-play process. It must sum every concrete within-trick branch consistent with the local controller, field, chance state, and observation contract.

## 3.4 What “trick 1 is the same as trick 7” licenses

The reported 64-class isomorphism licenses reuse of normalized local structure and transport laws.

It does **not** by itself license a single stationary matrix for all seven grades. The induced operator can still depend on:

- grade;
- remaining focal hand;
- support/evidence interface;
- current leader and orientation;
- selected local controller;
- successor interface type;
- field state;
- outcome monitor.

The safe conclusion is:

\[
\boxed{
\Gamma_{64}
\text{ is a reusable transition alphabet, not a complete state space.}
}
\]

The implementation should test how much of the operator is determined by \(\gamma\) and how much additional typed boundary data is necessary.

---

# 4. Quantitative Scheme observables

Scheme/Fix has so far been used mainly to define sets, answer bundles, and descriptor cells. The next use is linear.

## 4.1 Definition

Fix a Scheme/Fix query \(F\), output interface \(O\), and a declared finite-dimensional aggregation lens

\[
\Lambda_x:
W_F^O(x)
\longrightarrow
V
\]

or, more generally,

\[
\Lambda_x:
\mathcal P_{\mathrm{fin}}(W_F^O(x))
\longrightarrow
V,
\]

where \(V\) is a finite-dimensional rational vector space carrying any required role transports.

The associated quantitative Scheme observable is

\[
\boxed{
f_{F,O,\Lambda}(x)
=
\Lambda_x(W_F^O(x)).
}
\]

Examples include:

\[
\mathbf1[W_F^O(x)\ne\varnothing],
\]

\[
|W_F^O(x)|,
\]

\[
\sum_{\rho\in W_F^O(x)}e_{\operatorname{team}(\rho(c))},
\]

\[
\sum_{\rho\in W_F^O(x)}
\mathbf1[\rho(e)\text{ is a boss in transported context }q],
\]

and role-indexed one-hot tensors.

## 4.2 The old probability warning still applies

Using answer multiplicity as an observable does not turn answers into probability mass.

The quantity

\[
|W_F^O(x)|
\]

is a declared statistic of one latent world. The world still receives weight \(\beta(x)\), not \(|W_F^O(x)|\beta(x)\), unless the model explicitly defines such a reweighting.

Boolean existence and answer count are different observables and may require different counting algorithms.

## 4.3 Internal witnesses remain internal

Only output roles may appear in \(\Lambda\). Internal existential witnesses may contribute to truth of the query but do not become coordinates, counted objects, tracked identities, or valued features unless they are promoted to the output interface.

## 4.4 Equivariant observables

For a declared interface transport \(\Theta:x\to y\), an observable may be invariant,

\[
f(y)=f(x),
\]

or covariant through a linear representation,

\[
\boxed{
f(y)=f(x)R(\Theta).
}
\]

Chair-, context-, and domino-role tensors should usually be covariant rather than forced into scalar invariants. This preserves causal orientation without rebuilding physical coordinates.

---

# 5. Controlled continuation tests and outcome rank

Partition size is not the right lower bound for the proposed representation. Matrix rank is.

## 5.1 Continuation tests

A finite continuation test from interface \(\mathfrak i\) consists of:

- a finite prescription of focal local controls;
- branches only on declared observations;
- optional conditions on normalized trick classes and analyst-visible outcome emissions;
- a terminal or stopped readout from the selected outcome contract.

For test \(t\), define

\[
h_t(\xi)
=
\Pr_\xi[t]
\]

for event tests, or the corresponding exact expected readout for linear outcome tests.

Let \(\mathcal T_{\mathfrak i}\) be the finite test family induced by the selected horizon and outcome contract.

## 5.2 Continuation matrix

Define

\[
H_{\mathfrak i}(\xi,t)=h_t(\xi),
\qquad
\xi\in X_{\mathfrak i},
\quad
t\in\mathcal T_{\mathfrak i}.
\]

The number of distinct rows of \(H_{\mathfrak i}\) is the number of exact behavioral equivalence classes for that test family.

The quantity relevant to linear predictive compression is

\[
\boxed{
r_{\mathfrak i}
=
\operatorname{rank}_{\mathbb Q}H_{\mathfrak i}.
}
\]

Every row may be distinct while \(r_{\mathfrak i}\) remains small.

## 5.3 Theorem — finite predictive-rank minimality

An exact linear predictive realization of dimension \(r\) is a map

\[
F_{\mathfrak i}:X_{\mathfrak i}\to\mathbb Q^r
\]

such that every continuation test has a coefficient vector \(c_t\in\mathbb Q^r\) with

\[
h_t(\xi)=F_{\mathfrak i}(\xi)c_t.
\]

Then:

\[
\boxed{
\text{the least possible exact linear dimension is }
\operatorname{rank}_{\mathbb Q}H_{\mathfrak i}.
}
\]

### Proof

Collect the row vectors \(F_{\mathfrak i}(\xi)\) into a matrix \(F\), and the test coefficient vectors into a matrix \(C\). Then

\[
H_{\mathfrak i}=FC,
\]

so

\[
\operatorname{rank}H_{\mathfrak i}\le r.
\]

Conversely, choose any rank factorization

\[
H_{\mathfrak i}=FC
\]

with inner dimension \(\operatorname{rank}H_{\mathfrak i}\). The rows of \(F\) are exact predictive coordinates and the columns of \(C\) recover every test. ∎

## 5.4 What the theorem does and does not say

It proves an exact representational lower and upper bound for the selected finite controlled outcome theory.

It does not prove that:

- the rank is small;
- a minimum-rank basis has a compact Scheme expression;
- the coordinates are nonnegative;
- the coordinates are probabilities over abstract states;
- their moments can be computed cheaply from a 399M-world support fiber;
- the policy polytope has few exposed vectors.

Those are separate experiments.

A minimum linear realization may use signed rational coordinates. Such coordinates are exact predictive statistics, but they must not be called abstract-world probabilities.

The hierarchy is roughly

\[
\boxed{
\text{linear rank}
\le
\text{positive realization size}
\le
\text{partition-lump size}
\le
|X|.
}
\]

Strong controlled lumpability is the one-hot positive special case.

---

# 6. Residual closure: the constructive form of the rank theorem

The continuation matrix need not be built by enumerating every policy/test column. The selected test span can be generated backward.

## 6.1 Controlled preexpectation

For a typed transition event

\[
e=(\gamma,r,o)
\]

from \(\mathfrak i\) to \(\mathfrak j(o)\), define

\[
\boxed{
\operatorname{Pre}_{\mathfrak i,u,e}(f)(\xi)
=
\sum_{\xi'\in X_{\mathfrak j(o)}}
K_{\mathfrak i,u}(\xi;e,\xi')f(\xi').
}
\]

This is an **unnormalized** preexpectation. For \(f=1\), it is the probability of event \(e\).

## 6.2 Graded continuation spaces

At terminal interfaces let \(\mathcal V_{\mathfrak i}\) be the rational span of:

- the constant function \(1\);
- every selected terminal readout.

At a nonterminal interface define \(\mathcal V_{\mathfrak i}\) backward as the span of:

- \(1\);
- immediate readouts required by the outcome contract;
- every residual

\[
\operatorname{Pre}_{\mathfrak i,u,e}(f)
\]

for every legal local controller \(u\), typed event \(e\), and basis function \(f\in\mathcal V_{\mathfrak j(o)}\).

Because the continuation is graded, this recursion terminates.

## 6.3 Theorem — residual span equals continuation-test span

For every interface \(\mathfrak i\), \(\mathcal V_{\mathfrak i}\) is exactly the linear span of all selected continuation-test functions from \(\mathfrak i\).

### Proof

Proceed by induction on remaining tricks.

At grade zero, a continuation test is a terminal readout, so the claim holds by definition.

Assume it holds at every successor interface. Any nonterminal continuation test first chooses a legal local controller, observes one typed event, and then runs a successor test. Its test function is a finite sum of residuals of successor test functions, plus any immediate readout. By the induction hypothesis those successor functions lie in the successor continuation spaces, so the current test lies in \(\mathcal V_{\mathfrak i}\).

Conversely, each generator of \(\mathcal V_{\mathfrak i}\) is either a terminal/immediate test or the residual of a realizable successor test after a legal current controller and event. Hence every generator, and therefore every linear combination, lies in the continuation-test span. ∎

## 6.4 Consequence

The exact rank can be measured by exact row reduction during this backward closure construction:

\[
\boxed{
r_{\mathfrak i}=\dim\mathcal V_{\mathfrak i}.
}
\]

This is the preferred first rank experiment. It separates the algebraic question “is the future low rank?” from the language question “can Scheme/Fix express a compact basis?”

---

# 7. Exact controlled predictive modules

Choose a basis

\[
F_{\mathfrak i}
=
(f_{\mathfrak i,0},\ldots,f_{\mathfrak i,r_{\mathfrak i}-1})
\]

of \(\mathcal V_{\mathfrak i}\), with

\[
f_{\mathfrak i,0}=1.
\]

Treat \(F_{\mathfrak i}(\xi)\) as a row vector.

## 7.1 Closure matrices

Residual closure gives a unique matrix

\[
M_{\mathfrak i,u,e}
\in
\mathbb Q^{r_{\mathfrak i}\times r_{\mathfrak j(o)}}
\]

such that

\[
\boxed{
\operatorname{Pre}_{\mathfrak i,u,e}
F_{\mathfrak j(o)}
=
F_{\mathfrak i}M_{\mathfrak i,u,e}.
}
\tag{PCM}
\]

Pointwise, for every latent state \(\xi\),

\[
\sum_{\xi'}
K_{\mathfrak i,u}(\xi;e,\xi')
F_{\mathfrak j(o)}(\xi')
=
F_{\mathfrak i}(\xi)M_{\mathfrak i,u,e}.
\]

Equation (PCM) is the central finite certificate.

## 7.2 Predictive moment of a belief

For belief \(\beta\in\Delta(X_{\mathfrak i})\), define

\[
\boxed{
\psi_{\mathfrak i}(\beta)
=
\mathbb E_{\xi\sim\beta}
[F_{\mathfrak i}(\xi)].
}
\]

The first coordinate is one.

## 7.3 Exact observation update

Let \(o\) be the player-observed successor token. Sum every unobserved analyst emission that maps to \(o\):

\[
M_{\mathfrak i,u,o}
=
\sum_{e:\operatorname{obs}(e)=o}
M_{\mathfrak i,u,e}.
\]

The unnormalized successor moment is

\[
\boxed{
\widetilde\psi'
=
\psi_{\mathfrak i}(\beta)
M_{\mathfrak i,u,o}.
}
\]

Because the first successor basis coordinate is \(1\),

\[
\boxed{
\Pr(o\mid\beta,u)
=
\widetilde\psi'_0.
}
\]

For a positive-probability observation,

\[
\boxed{
\psi'
=
\frac{\widetilde\psi'}{\widetilde\psi'_0}.
}
\]

This is exact filtering in predictive coordinates.

## 7.4 Theorem — exact predictive filtering and test evaluation

Assume (PCM) holds at every reachable typed interface and the basis contains every terminal readout required by the selected outcome contract.

Then for every initial belief, every legal local-controller sequence, and every positive-probability observation history:

1. the recursively updated \(\psi\) equals the expectation of the concrete basis under the exact concrete posterior;
2. every selected continuation-test probability or expected readout is a linear function of \(\psi\);
3. the concrete latent posterior need not be materialized to predict any selected future test.

### Proof

The initial statement is the definition of \(\psi\). Suppose it holds at one interface. Multiplying by the appropriate closure matrix gives the concrete unnormalized expectation of the successor basis by (PCM) and linearity of expectation. Its first coordinate is the concrete observation probability. Dividing by that probability gives the expectation under the exact conditioned posterior. Induction over the finite observation history proves the filtering claim. Every test lies in the continuation span and therefore has a basis coefficient vector, giving exact linear evaluation. ∎

---

# 8. Backward policy coordinates and exact decision values

The forward message is \(\psi\). The backward message is a set of coefficient vectors.

## 8.1 Policy value vectors

For a deterministic lawful continuation policy \(\rho\) from interface \(\mathfrak i\), let

\[
V_\rho(\xi)
\]

be its expected scalar utility from latent state \(\xi\). Since this is a continuation test/readout,

\[
V_\rho(\xi)
=
F_{\mathfrak i}(\xi)c_\rho
\]

for a unique coefficient vector relative to the chosen basis.

Therefore

\[
\boxed{
J_\rho(\beta)
=
\psi_{\mathfrak i}(\beta)c_\rho.
}
\]

This is the exact replacement for the concrete world sum.

## 8.2 Lawful recursive construction

Suppose event \(e\) carries immediate scalar utility \(g(e)\), and let \(\mathbf e_0\) be the successor coefficient vector representing the constant function \(1\).

A policy chooses one local controller \(u\), then one continuation policy for each **player observation** \(o\). It may not choose separately for hidden emissions sharing the same \(o\).

If \(c_o\) is the successor continuation coefficient vector selected after observation \(o\), then the current policy vector is

\[
\boxed{
c_\rho
=
\sum_o
\sum_{e:\operatorname{obs}(e)=o}
M_{\mathfrak i,u,e}
\left(
 g(e)\mathbf e_0+c_o
\right).
}
\tag{POL}
\]

Equation (POL) is policy gluing in coefficient form.

## 8.3 Action policy-vector sets

For root action \(a\), let

\[
C_{B,a}
=
\{c_\rho:\rho(B)=a\}
\]

and define its convex hull

\[
\mathcal C_{B,a}=\operatorname{conv}C_{B,a}.
\]

Then

\[
\boxed{
Q_B(a)
=
\max_{c\in\mathcal C_{B,a}}
\psi(B)c.
}
\]

The global root value is

\[
\boxed{
V_B
=
\max_a Q_B(a).
}
\]

This is the dual predictive form of the policy-polytope geometry in v0.4 §9.

- v0.4 fixes the belief and represents policies by expected outcome-feature points.
- The predictive form represents policies by coefficient vectors and can evaluate them against any belief sharing the same basis/interface.

They give the same scalar support function after pairing.

## 8.4 Theorem — exact optimality without strategy fusion

Assume the predictive bases span all selected action-indexed continuation tests and all policy branching in (POL) is indexed only by declared player observations.

Then the predictive recursion returns the same exact \(V\), root \(Q\), and optimal-action correspondence as the concrete perfect-recall fixed-field problem for the selected utility.

### Proof

By structural induction, every lawful concrete policy produces a coefficient vector through (POL), because its successor choice is one policy per observable successor information state. Conversely, every vector generated by (POL) specifies one lawful local controller and one lawful continuation per observable branch, so it denotes a concrete information-consistent policy.

The predictive pairing equals each policy’s concrete expectation. Maximizing the same finite policy set, with or without a root-action restriction, gives equal \(V\) and \(Q\). Hidden emissions that share one observation share one continuation choice, so no worldwise strategy fusion is introduced. ∎

## 8.5 What may compress dramatically

Even if \(r_{\mathfrak i}\) is moderate, \(C_{B,a}\) may be enormous. Exact convex-hull and dominance pruning can still leave few exposed vectors for the permitted utility or belief region.

The opening solve therefore has two independent favorable possibilities:

\[
\boxed{
\text{small predictive rank}
\quad\text{and/or}\quad
\text{small exposed dual policy geometry}.
}
\]

Both should be measured.

---

# 9. Outcome laws rather than one scalar

The scalar recursion above is the easiest correctness test. The intended object is richer: a policy-indexed law of selected outcomes.

## 9.1 Outcome monoid algebra

Let \(\mathcal M\) be a finite or finitely generated monoid of outcome increments. Examples:

- \(\mathbb N e_\star\) for focal-team trick count;
- \(\mathbb N^2\) for trick and Straight-score increments;
- a role-indexed additive monoid for transported captures;
- a finite terminal category set.

Use the exact monoid algebra

\[
\mathbb Q[\mathcal M].
\]

A transition with increment \(r\in\mathcal M\) contributes the monomial \([r]\). Matrix multiplication then convolves outcome increments automatically.

Define the outcome-decorated operator

\[
\boxed{
\widehat M_{\mathfrak i,u,o}
=
\sum_{e:\operatorname{obs}(e)=o}
[e.r]\,M_{\mathfrak i,u,e}.
}
\]

Products of these operators yield exact generating functions or finite outcome laws.

## 9.2 Theorem — outcome-algebra lift

If the rational closure equation (PCM) holds separately for every finite outcome increment, then replacing each event matrix by its monomial-decorated matrix preserves the exact joint law of accumulated outcome and observation history.

### Proof

Expand a product of decorated matrices. Each term selects one concrete event label per step, multiplies the corresponding rational transition weights, and multiplies the monomials. Monoid multiplication adds or combines the emitted outcomes. Summing terms therefore gives exactly the concrete path sum grouped by accumulated outcome and observation history. ∎

This theorem changes the coefficient algebra, not the physical transition semantics or predictive basis.

---

# 10. Concrete outcome contracts to test

The implementation should add outcome richness in stages.

## 10.1 Team trick distribution

Use one formal variable \(u\). Emit

\[
u
\]

when the focal partnership wins a completed trick and \(1\) otherwise.

For policy \(\rho\), the final polynomial is

\[
G_{B,\rho}(u)
=
\sum_{k=0}^{7}
\Pr_\rho(T=k\mid B)u^k.
\]

The coefficient \([u^k]G\) is the exact probability of winning \(k\) tricks.

Expected tricks, make probabilities based on trick thresholds, and other trick-only utilities become linear readouts of this distribution.

## 10.2 Straight score distribution

For the fixed ordinary Straight count schedule, emit

\[
u^{\Delta t}z^{\Delta s},
\]

where \(\Delta t\in\{0,1\}\) and \(\Delta s\) is the trick point plus captured count when the focal team wins the trick.

The final polynomial

\[
G_{B,\rho}(u,z)
=
\sum_{k,s}
\Pr_\rho(T=k,S=s\mid B)u^kz^s
\]

is the exact joint trick/score law.

The count-free 64-class label need not determine \(\Delta s\). That is not a failure. The concrete macro compiler can attach the score monomial before summing transitions into the predictive operator.

A fixed Straight-score decoration breaks pip symmetries that do not preserve the count labels. Operator reuse must therefore transport the score decoration or retain the required valuation label.

## 10.3 Arbitrary tile valuation

For valuation-parametric work, do not bake one physical score into the operator and then claim universality.

Use the v0.4 §8 outcome/gauge architecture and proposed §12.6A:

1. preserve count-free trick structure;
2. reintroduce tile features through declared rigid domino roles or a capture-complete physical feature contract;
3. apply valuation after the law is constructed;
4. quotient coefficients by the gauge only when the represented capture coordinates satisfy the four-tiles-per-trick conservation relation;
5. require the physical valuation stabilizer when a fixed physical valuation is expected to survive a nontrivial tile transport.

## 10.4 Win and contract outcomes

A win, make, set, or mark outcome is a deterministic terminal readout of a sufficiently rich joint law. It should normally be computed after the score/trick distribution, not installed as the only primitive output, unless that Boolean is the entire selected purpose.

## 10.5 Control return

“Control” must be a declared Scheme predicate, not an intuitive scalar. Examples include:

\[
C_{\mathrm{seat}}(x)
=
\mathbf1[\text{the focal seat leads the next trick}],
\]

\[
C_{\mathrm{team}}(x)
=
\mathbf1[\text{the focal partnership leads the next trick}],
\]

or a richer role predicate such as “the focal team leads with a live master in the led context.”

Define the first return time

\[
\tau_C
=
\inf\{j\ge1:C(X_j)=1\}.
\]

The useful object is the joint stopped law

\[
\boxed{
\Pr(
\tau_C=j,
\text{accumulated outcome}=m,
\psi_{\tau_C}\in A
\mid B,\rho
).
}
\]

The marginal \(\Pr(\tau_C=j)\) answers “how likely do I get control back in exactly \(j\) tricks?” The joint law is needed for decision quality, because returning control with different successor predictive states is not equivalent.

## 10.6 Finite monitor construction

A first-return law does not require a new game semantics. Adjoin a finite deterministic monitor to the trick-boundary process. Its state records:

- whether return has occurred;
- elapsed trick count up to the finite horizon;
- any stopped outcome summary retained at return.

Take the product of the information interface and monitor state, then run the same predictive closure construction.

### Theorem — finite monitor preservation

If the base predictive module preserves the joint law of observations, Scheme predicates read at boundaries, and selected outcome increments, then its finite deterministic monitor product preserves every terminal or first-hit statistic computed by that monitor.

### Proof

The monitor successor is a deterministic function of the preserved current monitor state, observation, Scheme predicate value, and outcome increment. Equal laws of those inputs induce equal laws of the product successor. Induction over the finite grade gives equal monitor-path and terminal laws. ∎

---

# 11. Ordering and pruning

Outcome laws do not have one purpose-free total order.

## 11.1 Utility-cone dominance

For a declared utility family \(\mathcal U\), define

\[
\nu_1\succeq_{\mathcal U}\nu_2
\iff
\langle U,\nu_1-\nu_2\rangle\ge0
\quad
\forall U\in\mathcal U.
\]

A policy law dominated in this order can be pruned for that purpose family.

For coefficient vectors evaluated over a permitted predictive-state region \(\Psi\), define

\[
c_1\succeq_\Psi c_2
\iff
\psi(c_1-c_2)\ge0
\quad
\forall\psi\in\Psi.
\]

This supports exact dual-space pruning.

## 11.2 Score stochastic dominance

If utility is known only to be nondecreasing in final score, first-order stochastic dominance of score distributions is sound.

If utility also depends on control, risk, contract thresholds, or successor state, score dominance alone may be insufficient.

## 11.3 No universal control order

Earlier control return is not automatically better. It may trade away score, consume a master, put the wrong partner on lead, or alter future information.

Control return should first be retained as part of a joint outcome law. Any ordering comes from a declared utility or dominance cone afterward.

---

# 12. Equivariance in predictive coordinates

Equivariance remains useful even when it does not merge opening hands.

## 12.1 Coordinate transport

For a declared interface transport

\[
\Theta:\mathfrak i\to\mathfrak i',
\]

let

\[
R_{\mathfrak i}(\Theta)
\]

be the invertible linear map transporting predictive coordinates, defined by

\[
\boxed{
F_{\mathfrak i'}(\Theta\xi)
=
F_{\mathfrak i}(\xi)R_{\mathfrak i}(\Theta).
}
\]

The transport must respect the §3 role schema, the output interface, seat orientation, focal-team orientation, and every rigid-role rule assumed by the selected basis.

## 12.2 Operator intertwining

For a transported local controller and event, representative independence requires

\[
\boxed{
M_{\mathfrak i',\Theta u,\Theta e}
=
R_{\mathfrak i}(\Theta)^{-1}
M_{\mathfrak i,u,e}
R_{\mathfrak j}(\Theta').
}
\]

Here \(\Theta'\) is the induced successor-interface transport.

This is the linear analogue of equivariant controlled lumpability. It says that changing concrete coordinates and then applying the canonical operator equals applying the original operator and then transporting the successor coordinates.

## 12.3 Theorem — equivariant representative independence

If the basis transport law and operator intertwining law hold coherently, then all predictive updates, test probabilities, policy values, and transported outcome laws are independent of the chosen representative of a trick class or interface orbit.

### Proof

Substitute the basis transport law into (PCM). The intertwining identity carries each concrete residual calculation to the transported one. Products of intertwined operators telescope through the intermediate coordinate changes, so every complete path calculation differs only by the declared terminal coordinate transport. Transported readouts therefore agree. ∎

## 12.4 What equivariance now contributes

Equivariance can provide:

- canonical operator templates for the 64 trick classes;
- reuse across transported chair/context/domino roles;
- symmetry blocks in the predictive vector space;
- smaller certificates;
- common Scheme expressions for basis coordinates;
- representative-independent outcome evaluation.

It need not provide a second opening hand in the same orbit.

The clarified division of labor is:

\[
\boxed{
\text{equivariance organizes coordinates; rank compresses coordinates.}
}
\]

---

# 13. Compiling the opening fiber into moments

A low-dimensional predictive basis is useful at the opening only if its expectation can be computed without enumerating the full fiber.

## 13.1 Initial predictive coordinate

For opening decision state \(B=(K,e,\beta)\), compute

\[
\boxed{
\psi(B)
=
\left(
\mathbb E_\beta[f_0],
\ldots,
\mathbb E_\beta[f_{r-1}]
\right).
}
\]

Once this vector is certified, runtime continuation evaluation uses \(\psi\) and the operator/policy library. It no longer needs one coordinate per hidden deal.

## 13.2 Uniform opening belief

Under a uniform belief on the exact support fiber,

\[
\mathbb E_\beta[f]
=
\frac{
\sum_{\omega\in\Phi(K)}f(\omega)
}{|\Phi(K)|}.
\]

For a Boolean Scheme event this is exact model counting. For an answer-count observable it is a weighted sum of answer multiplicities. Those are different computations.

## 13.3 Scheme moment compiler

A first exact compiler for bounded-role observables can use the support-normal-form structure:

1. enumerate only the finite interpretations of the declared output roles;
2. translate each grounded Scheme case into holder, void, equality, context, and role constraints on the exact support cell;
3. force or delete the corresponding holder edges;
4. count residual feasible assignments with the accepted matching-supported counter;
5. combine equality-pattern branches and Fix unions with exact set semantics;
6. apply the declared aggregation lens;
7. divide by the exact support weight when a normalized expectation is required.

For Boolean Fixes, overlapping witness branches must not be added naïvely. Use a disjoint normal form, exact inclusion–exclusion where tractable, a decision diagram, or another certified union counter.

For nonuniform beliefs, replace counts by exact weights. This requires a factorization or weighted counter compatible with the actual belief/field model. v0.4 does not prove that every such belief admits a cheap compiler.

## 13.4 Separate research questions

Two independent questions must be measured.

### Representational compression

Is

\[
r=\dim\mathcal V
\]

small?

### Moment-compilation complexity

Can

\[
\mathbb E_\beta[f_i]
\]

be computed from the exact support/belief representation without enumerating the fiber?

A low-rank basis whose moments require full enumeration does not yet solve the opening computation. A cheap Scheme moment family that is not closed under continuation is not sufficient either.

The target is their intersection.

---

# 14. Two-stage basis discovery

Do not require the first algebraic basis to be human-readable.

## 14.1 Stage one — discover whether compression exists

On an enumerated finite experimental domain:

1. represent each candidate observable as an exact rational vector over latent states;
2. generate the continuation spaces by residual closure;
3. row-reduce exactly;
4. record the dimensions and closure matrices.

This yields a minimum-rank algebraic basis for the selected domain and outcome contract.

The basis may be a table rather than a compact Scheme.

## 14.2 Stage two — realize the basis in Scheme language

Given the algebraic continuation space, search for a small family of Scheme-generated observables whose vectors span it.

This is a separate synthesis target:

\[
\boxed{
\operatorname{span}
\{f_{F_1,\Lambda_1},\ldots,f_{F_m,\Lambda_m}\}
=
\mathcal V.
}
\]

Possible outcomes are informative:

- **low rank, compact Scheme basis:** strongest success;
- **low rank, large Scheme basis:** exact compression exists but the language or synthesis method is weak;
- **high rank, small exposed policy set:** predictive compression is poor but decision geometry still helps;
- **high rank and large policy geometry:** selected outcome contract or horizon may be intrinsically broad;
- **small basis but expensive moments:** representational success, counting bottleneck.

This separation prevents a failed hand-designed Scheme vocabulary from being mistaken for a proof of high predictive complexity.

---

# 15. Experimental program

The experiments should proceed from exact small domains to the opening hand.

## Experiment 0 — certify the 64-class macro compiler

**Goal:** establish that each primitive legal trick path is represented exactly once by a typed macro transition.

For every enumerated test interface:

1. enumerate primitive four-play paths under each local controller;
2. compute the canonical trick class \(\gamma\);
3. record observation, outcome increment, successor interface, and role transport;
4. aggregate into the macro-kernel;
5. compare total probability and every successor/output bucket with the primitive engine.

**Required checks:**

\[
\sum_{\gamma,r,o,\xi'}
K_{\mathfrak i,u}(\xi;\gamma,r,o,\xi')=1
\]

for every legal \((\mathfrak i,u,\xi)\), plus exact equality of primitive and folded path laws.

**Important result to record:** whether \(\gamma\) alone determines the normalized operator or whether a small additional boundary-interface tag is required.

## Experiment 1 — exact outcome-rank census

**Scope:** count-free fixed-field suffixes of one, two, and three tricks.

Run the residual-closure algorithm for increasingly rich contracts:

1. expected focal tricks;
2. full focal trick-count distribution;
3. trick count plus next-leader/team-control predicate;
4. first return of declared control;
5. fixed Straight score distribution.

Record, for every interface and grade:

- number of latent states;
- number of exact behavioral rows where computable;
- partition-lump size;
- predictive rank \(r\);
- basis sparsity;
- operator sparsity;
- rank after symmetry block decomposition.

The load-bearing comparison is

\[
|X|
\quad\text{versus}\quad
\#\text{behavioral classes}
\quad\text{versus}\quad
r.
\]

## Experiment 2 — dual policy-geometry census

Using the same exact bases, build policy coefficient sets backward with one continuation choice per player observation.

Record:

- raw number of deterministic policy vectors;
- number of distinct vectors;
- number of convex-hull vertices;
- number exposed by selected utility/valuation cones;
- root action regions;
- equality with concrete enumeration on every small domain.

This measures whether the decision side compresses even when the predictive rank does not.

## Experiment 3 — Scheme-basis synthesis

Begin with candidate quantitative observables built from:

- current and successor context roles;
- actual relative seat position;
- boss, runner-up, and beater-chain roles;
- forced-follow and slough mobility;
- companion relations;
- current and next-leader roles;
- 64 trick-class indicator/tensor features;
- bounded role-answer counts;
- support-normal-form local ambiguity summaries.

For each candidate, evaluate its exact vector on the enumerated domain. Search for a minimum-cost subset spanning the algebraic continuation space.

When the span is incomplete, emit an exact residual vector or a pair of states agreeing on the candidate span but separated by a continuation test. Use that witness to synthesize the next generic Scheme observable.

The success condition is span equality, not static response-cell purity.

## Experiment 4 — equivariant operator reuse

For each declared transport between trick/interface representatives:

1. compute the predictive coordinate transport matrix \(R(\Theta)\);
2. verify basis covariance;
3. verify operator intertwining;
4. verify terminal readout transport;
5. verify score/valuation stabilizer conditions for any decorated outcome.

Record the number of canonical matrices after transport quotienting.

## Experiment 5 — exact moment compiler

For a chosen small Scheme basis:

1. compute each moment by explicit world enumeration on reduced or suffix domains;
2. compute the same moment through the support-normal-form counter;
3. compare exact rationals;
4. scale the counter to the opening support fiber without materializing worlds;
5. emit per-coordinate counting certificates.

Measure:

- role-interpretation count;
- residual matching-counter calls;
- memoization reuse;
- arithmetic size;
- total compile time;
- maximum Scheme arity;
- Fix-overlap handling cost.

## Experiment 6 — outcome-law operators

Lift the verified predictive operators to:

1. \(\mathbb Q[u]\) for trick distribution;
2. \(\mathbb Q[u,z]\) for joint trick/score distribution;
3. a finite monitor product for control-return distribution;
4. role-indexed additive outcomes for selected valuation tests.

On small domains compare every coefficient of every policy law with the concrete terminal-law enumerator.

## Experiment 7 — opening-action prototype

For one opening hand:

1. compile \(\psi_7(B)\) from the broad exact fiber;
2. load or build the dual policy-vector sets for each legal opening action;
3. compute

\[
Q_B(a;U)
=
\max_{c\in\mathcal C_{B,a}}
\psi_7(B)c;
\]

4. return the policy-indexed trick, control, and score laws of the exposed root policies;
5. inspect action sensitivity under declared utility families;
6. compare against independent sampling only as a sanity check, never as the correctness certificate.

The first opening experiment need not solve all seven tricks with the richest outcome law. A successful two- or three-trick exact predictive suffix attached to an opening moment compiler would already validate the architecture.

---

# 16. Exact certificate package

Each experiment should emit machine-checkable finite records.

## 16.1 Macro-kernel certificate

```text
interface identifier
latent-state enumeration or reconstruction certificate
local controller
primitive path buckets
canonical 64-class label
observation label
outcome increment
successor interface/state
role transport
exact rational probability
```

## 16.2 Basis certificate

```text
interface identifier
ordered latent-state domain
ordered basis functions
exact basis evaluation matrix
pivot rows or columns
exact rank
constant-coordinate index
terminal-readout coefficient vectors
```

## 16.3 Closure certificate

For every \((\mathfrak i,u,e)\):

```text
successor interface
exact preexpectation matrix/vector
claimed closure matrix M
certificate that Pre(F_successor) = F_current * M
```

The checker recomputes both sides over the finite domain using exact arithmetic.

## 16.4 Equivariance certificate

```text
source and target interfaces
role/action/observation transports
predictive coordinate transport R
basis covariance equality
operator intertwining equality
terminal readout transport equality
valuation stabilizer result, when applicable
```

## 16.5 Moment certificate

```text
root support-normal-form identifier
belief/weight model identifier
Scheme observable identifier
output interface and aggregation lens
grounded role cases
residual weighted counts
union/overlap certificate
exact numerator and denominator
final rational moment
```

## 16.6 Policy-vector certificate

```text
interface and root action
local controller
one successor-policy reference per player observation
hidden event aggregation map
resulting exact coefficient vector
convex-hull or dominance witness, if pruned
```

## 16.7 End-to-end certificate

For every small validation domain:

```text
concrete policy terminal law
predictive policy terminal law
coefficient-by-coefficient equality
concrete Q and V
predictive Q and V
optimal-action correspondence equality
```

Floating-point agreement is not a proof. Rational arithmetic, finite-field screening followed by rational reconstruction, or another exact certificate path is required.

---

# 17. Implementation boundaries

The existing implementation guide remains the source for the full runtime architecture. The new modules needed for this experiment are narrower.

```text
Texas42/Predictive/Test
Texas42/Predictive/Preexpectation
Texas42/Predictive/Span
Texas42/Predictive/Basis
Texas42/Predictive/Operator
Texas42/Predictive/Equivariance
Texas42/Predictive/MomentCompiler
Texas42/Predictive/PolicyDual
Texas42/Predictive/OutcomeAlgebra
Texas42/Predictive/Monitor
Texas42/Certificates/Predictive
```

Recommended ownership:

- `Test`: finite continuation-test syntax and denotation;
- `Preexpectation`: exact macro-kernel action on observables;
- `Span`: exact rational closure and rank;
- `Basis`: basis records and coordinate conversion;
- `Operator`: closure matrices and filtering;
- `Equivariance`: coordinate transports and intertwining checks;
- `MomentCompiler`: exact support/belief integration of Scheme observables;
- `PolicyDual`: lawful coefficient-vector recursion and convex geometry;
- `OutcomeAlgebra`: polynomial/finite-law matrix coefficients;
- `Monitor`: control-return and other finite stopping automata;
- `Certificates`: independent finite checkers.

Do not put exact rule support, belief semantics, Scheme truth, or physical transitions under these modules. They consume those authoritative layers.

---

# 18. Failure modes this program is designed to expose

## 18.1 Confusing trick classes with global states

A 64-class local alphabet may still induce many grade- and interface-specific operators.

## 18.2 Measuring only partition classes

Distinct continuation rows do not imply high predictive rank.

## 18.3 Using low rank as a runtime claim

Low rank does not imply cheap root moment compilation.

## 18.4 Using a compact Scheme family without closure

Useful-looking moments are not sufficient unless every selected controlled residual remains in their span.

## 18.5 Treating signed coordinates as probabilities

A minimum-rank linear basis may have negative coordinates or transition entries.

## 18.6 Conditioning on analyst outputs

Policy branches may depend only on declared player observations, not hidden trick-class, role, or reward emissions.

## 18.7 Reintroducing strategy fusion in the dual recursion

All hidden emissions sharing one player observation must share one continuation policy choice.

## 18.8 Assuming count-free equivariance preserves Straight score

The count schedule is not pip-symmetric. Score decorations require transport, a stabilizer, or additional retained labels.

## 18.9 Applying the valuation gauge to incomplete captures

The gauge descends only when the represented role/feature contract satisfies the required four-tiles-per-trick conservation relation.

## 18.10 Summing existential witnesses as worlds

Scheme answer counts are observables only under a declared lens. They do not alter the world marginal by default.

## 18.11 Compressing away distinguishable useful history

If two player histories are merged, the predictive test space must prove that every legal action has the same selected future law from both. Similar scalar value is not enough.

## 18.12 Optimizing one scalar too early

Retaining only the policy best for one utility destroys later outcome and valuation experiments. Store policy-indexed laws or coefficient sets until the purpose is fixed.

---

# 19. Mathematical additions recommended for the next theory revision

The following definitions and theorems are the minimal clean extension of v0.4.

## 19.1 Quantitative Scheme observable

A Scheme/Fix answer bundle plus a declared finite-dimensional aggregation lens defines an exact scalar or tensor observable.

## 19.2 Controlled preexpectation

Define \(\operatorname{Pre}_{u,e}\) on quantitative observables and prove linearity, typing, and transport compatibility.

## 19.3 Continuation-test span theorem

Prove that backward residual closure equals the span of all finite continuation tests for the selected outcome contract.

## 19.4 Finite predictive-rank theorem

Prove rank minimality of exact linear realizations and identify \(\dim\mathcal V\) with the controlled continuation-matrix rank.

## 19.5 Predictive filtering theorem

Prove that closure matrices update exact conditional predictive moments without materializing the latent posterior.

## 19.6 Predictive optimality theorem

Prove equality of concrete and predictive \(V,Q\) when policy recursion branches only on preserved observations and the basis spans all selected action-indexed continuation tests.

## 19.7 Equivariant predictive-module theorem

Extend §12.6A from one-hot descriptor states to covariant linear coordinates and prove operator intertwining, representative independence, and transported outcome equality.

## 19.8 Outcome-algebra lift

Prove that eventwise closure lifts to exact generating functions and finite outcome laws over a monoid algebra.

## 19.9 Finite monitor theorem

Prove preservation of first-hit, return-time, and other finite monitor outcomes built from preserved Scheme predicates and emissions.

## 19.10 Domain-specific moment-compiler theorem

For each accepted support/belief compiler, prove that the emitted Scheme moments equal direct integration over the exact latent domain. This theorem is implementation-specific; no general cheap-counting claim should be made.

---

# 20. Decision gates

The experiment should produce a clear answer even if the hoped-for opening compression does not appear.

## Gate A — local folding

Does the 64-class macro compiler reproduce the primitive one-trick law exactly?

## Gate B — predictive rank

Is \(r_n\) materially smaller than the latent-state or behavioral-class count at useful horizons?

## Gate C — Scheme realizability

Can the continuation space be spanned by a compact family of transported Scheme observables?

## Gate D — moment compilation

Can the opening \(\psi(B)\) be computed exactly without enumerating the broad fiber?

## Gate E — decision geometry

Do policy coefficient sets collapse to a manageable number of distinct, vertex, or exposed vectors?

## Gate F — outcome usefulness

Do trick, control-return, and score laws distinguish root actions in a stable and interpretable way?

A failure at one gate does not invalidate the others. It identifies the actual bottleneck:

- transition template count;
- predictive dimension;
- relational language;
- exact counting;
- policy geometry;
- or outcome contract.

---

# 21. Final unified picture

The direct traversal stores the opening belief and every continuation-value function in the concrete-world basis. For a policy \(\rho\),

\[
J_\rho(B)=\beta^T v_\rho.
\]

Let \(H\) be the selected continuation matrix whose columns include the policy, event, and outcome tests required by the experiment. The proposed architecture searches for an exact rank factorization

\[
\boxed{
H=FC.
}
\]

Each policy-value column then has the form

\[
v_\rho=Fc_\rho,
\]

and therefore

\[
\boxed{
J_\rho(B)
=
\beta^T F c_\rho
=
\psi(B)c_\rho.
}
\]

The complete experiment stack is:

\[
\boxed{
\begin{array}{c}
\text{exact Straight support and belief}\\
\downarrow\text{ exact Scheme moment compiler}\\
\psi_7(B)\\[1mm]
\downarrow\text{ 64-class typed predictive operators}\\
\text{exact observation and outcome laws}\\[1mm]
\uparrow\text{ lawful dual policy recursion}\\
\mathcal C_{B,a}\\[1mm]
\downarrow\text{ support-function pairing}\\
Q_B(a),\ V_B,\ \text{trick law},\ \text{control-return law},\ \text{score law}.
\end{array}
}
\]

The opening hand does not need a second hand in its equivalence class. It needs a finite coordinate in a continuation space that is closed under the game’s controlled residuals.

The 64 trick classes do not need to be the opening states. They need to generate reusable, transported operators on that continuation space.

Scheme/Fix does not need to reconstruct the world. It needs to generate enough quantitative observables to span the selected future theory and enough exact counting structure to integrate those observables over the opening belief.

The decisive mathematical target is therefore:

\[
\boxed{
\text{a compact Scheme-generated, equivariant, controlled predictive module}
}
\]

with

\[
\boxed{
\text{exact moment compilation}
+
\text{exact operator closure}
+
\text{exact policy gluing}
+
\text{exact outcome-law readout}.
}
\]

That is the next level of the model. It is strictly more expressive than a partition of positions, remains finite and certifiable, and attacks the opening fiber at the only place where its full cardinality actually enters: the choice of coordinates used to pair belief with continuation.
