# Decision-Sparse Exact Solving for Straight Texas 42

## Policy Envelopes, Scheme-Weighted Integration, and Adaptive Information Gluing

**Status:** exploratory mathematical handoff, v0.1  
**Date:** 2026-08-13  
**Primary dependencies:**  
- `walt/math/unified_information_geometry_v0.4.md`
- `walt/math/equivariant_lumpability_v0.5.md`
- `walt/math/predictive_algebra_v0.6.md`

**Branch evidence used:** PR 4, especially `walt/LOG.md` sessions S5e–S6c, the binding rulings in `walt/CENSUS-RULINGS.md`, and the exact exploratory result records under `walt/walt-factory/results/`.

**Purpose:** identify the next exact mathematical object after the structural-quotient and universal-predictive-rank programs reached their natural limits, and give the implementation side a finite experimental program.

**Non-purpose:** restate the Straight 42 rules, support theory, Scheme/Fix semantics, information geometry, valuation gauge, or the proofs already carried by v0.4–v0.6.

**Operator boundary:** unless a section explicitly says otherwise, “optimal” means the exact finite-horizon, perfect-recall, fixed-field best response under treatment \(H\): one information-consistent focal policy against the declared fixed field for the other seats. Results for \(C\), \(F\), world-informed evaluation, and perfect-information minimax are not silently interchangeable.

**Count boundary:** the first layer is count-free. Straight count, arbitrary tile values, and nonlinear contract utility re-enter only through the declared outcome contract in §11.

---

# 0. Executive synthesis

The recent work has established a sharp and useful combination of facts.

1. **Local count-free trick mechanics compress completely.** The complete pip-trump last-trick alphabet has exactly

   \[
   4\cdot 2^3\cdot 2=64
   \]

   classes: actor offset, three follow/slough bits, and the focal-team trick outcome.

2. **The first-play structural quotient is rigid.** On the 28-tile opening carrier, every seven-tile hand is its own count-free pip-trump structural class:

   \[
   \binom{28}{7}=1{,}184{,}040.
   \]

3. **The universal linear value closure becomes almost or fully saturated by grade three.** On the measured \(1{,}680\)-world fibers,

   \[
   \dim V^{\mathrm{val}}\in\{1461,\ldots,1680\},
   \]

   with one coordinate at full dimension.

4. **The policy side can nevertheless collapse almost completely.** In every grade-three root/lead case that completed within the declared frontier budget, one policy weakly dominated every lawful alternative in every world. The two unfinished cases were precisely the tense non-boss trump leads.

The correct synthesis is therefore

\[
\boxed{
\text{the opening truth may be high-dimensional while the opening decision is sparse.}
}
\]

The first two compression programs asked for a compact representation of **all future distinctions**. Exact action choice needs less. It needs a compact proof of a maximum.

The new target is

\[
\boxed{
\begin{array}{c}
\text{a small lawful candidate-policy set}\\
+\ \text{exact Scheme circuits for candidate and upper-bound outcomes}\\
+\ \text{exact weighted integration over the opening fiber}\\
+\ \text{a small set of information-gluing or dominance certificates}.
\end{array}
}
\]

For each root action \(a\), construct a lawful lower bound \(L_a\) and a valid relaxed upper bound \(U_a\):

\[
L_a
\le
Q^H(a)
\le
U_a.
\]

The opening play is certified without solving every action exactly as soon as some \(a^\star\) satisfies

\[
\boxed{
L_{a^\star}\ge U_a
\qquad
\text{for every }a\ne a^\star.
}
\]

This is the most direct mathematical route now visible from the branch.

---

# 1. What the branch has ruled out—and what it has not

## 1.1 State-partition compression

A state quotient asks for an equivalence relation such that equivalent states have the same future transition and outcome behavior.

The retrograde quotient and equivariant controlled-lumpability work found real compression in suffixes, history forgetting, and transported local structure. They did **not** find a nontrivial structural quotient of opening hands.

This rules out the simplest picture:

\[
\text{opening hand}
\longmapsto
\text{one of a small number of exact state classes}.
\]

It does not rule out compact circuits, sparse policies, or short optimality proofs.

## 1.2 Universal linear predictive compression

The predictive-algebra program asks whether every selected future test lies in a small rational vector space.

The grade-three value closures nearly saturated or saturated the complete world space. Complete public attribution explains why richer distribution closures are forced to full dimension: eventually every physical tile is publicly played by a named seat, so complete records distinguish worlds.

This rules out a small **flat linear basis for all selected future tests**.

It does not rule out:

- a small upper envelope after maximization;
- a small arithmetic or Boolean circuit with heavy subexpression reuse;
- a small set of active information constraints;
- a small proof that one root action dominates the others;
- a purpose-specific rather than universal evaluator.

Matrix rank is a lower bound on linear factorizations. It is not a lower bound on nonlinear circuit size or proof-DAG size.

## 1.3 Decision geometry

Let \(\alpha_\rho\) be the world-indexed value vector of one lawful policy. Optimization takes the support function

\[
\beta
\longmapsto
\max_\rho\langle\beta,\alpha_\rho\rangle.
\]

The operation \(\max\) destroys many distinctions retained by linear span. A set of vectors may span the entire world space while only one vector ever matters to the maximum.

That is exactly the texture reported by S6a and S6b:

\[
\boxed{
\text{value richness}
\quad+\quad
\text{decision simplicity}.
}
\]

---

# 2. Exact fixed-field decision object

Fix a decision state

\[
B=(K,e,\beta)
\]

in the sense of v0.4, focal player \(m\), fixed field \(\sigma_{-m}\), and a finite terminal utility \(U\).

Let

\[
X_B=\operatorname{supp}_+(\beta)
\]

be the finite augmented latent support relevant to the continuation.

For a root action \(a\), let

\[
\mathcal R_H(B,a)
\]

be the finite set of deterministic focal policies that

- choose \(a\) at the root;
- choose one transported action at every future focal information state;
- branch only on observations available under treatment \(H\).

For \(\rho\in\mathcal R_H(B,a)\), define the world-value vector

\[
\boxed{
\alpha_\rho(\xi)
=
\mathbb E[
U
\mid
\xi,\rho,\sigma_{-m}
].
}
\]

The expectation integrates only the declared field and chance randomness after conditioning on latent state \(\xi\).

Then

\[
J_B(\rho)
=
\langle\beta,\alpha_\rho\rangle
=
\sum_{\xi\in X_B}
\beta(\xi)\alpha_\rho(\xi),
\]

and

\[
\boxed{
Q^H_B(a)
=
\max_{\rho\in\mathcal R_H(B,a)}
\langle\beta,\alpha_\rho\rangle.
}
\]

The global value and optimal root-action set are

\[
V^H_B=\max_a Q^H_B(a),
\]

\[
\operatorname{Opt}^H(B)
=
\arg\max_a Q^H_B(a).
\]

For an opening hand with 21 unseen tiles distributed \(7,7,7\), the raw hidden-deal fiber has

\[
\binom{21}{7}\binom{14}{7}
=
399{,}072{,}960
\]

worlds before additional support restrictions.

The purpose of this document is not to alter that exact domain. It is to avoid materializing one independent solve per world.

---

# 3. Three different notions of complexity

For one root action \(a\), define the finite policy-vector set

\[
\mathcal A_{B,a}
=
\{
\alpha_\rho:
\rho\in\mathcal R_H(B,a)
\}
\subseteq
\mathbb Q^{X_B}.
\]

Three unrelated cardinalities or dimensions may now be measured.

## 3.1 Behavioral state complexity

How many distinct complete future behaviors do latent states have?

This is the partition/right-congruence question.

## 3.2 Predictive linear complexity

What is the dimension of the span of selected future tests?

This is the continuation-matrix or residual-closure question.

## 3.3 Decision-envelope complexity

How many policy vectors are required to represent

\[
\beta\mapsto
\max_{\alpha\in\mathcal A_{B,a}}
\langle\beta,\alpha\rangle
\]

on the belief region that actually matters?

This is the new central object.

The branch now supplies evidence that the first two quantities can be large while the third is one.

---

# 4. Policy envelopes and decision width

## 4.1 Belief-relative envelope

Let

\[
\mathcal B
\subseteq
\Delta(X_B)
\]

be a declared set of beliefs.

A subset

\[
E_{B,a}\subseteq\mathcal A_{B,a}
\]

is **envelope-sufficient on \(\mathcal B\)** when

\[
\boxed{
\max_{\alpha\in E_{B,a}}
\langle\beta,\alpha\rangle
=
\max_{\alpha\in\mathcal A_{B,a}}
\langle\beta,\alpha\rangle
\qquad
\forall\beta\in\mathcal B.
}
\]

Define the **decision width**

\[
\boxed{
W_{\mathcal B}(B,a)
=
\min
\left\{
|E|:
E\subseteq\mathcal A_{B,a},
\ E\text{ envelope-sufficient on }\mathcal B
\right\}.
}
\]

The minimum exists because \(\mathcal A_{B,a}\) is finite.

## 4.2 Theorem — envelope sufficiency

If \(E_{B,a}\) is envelope-sufficient on \(\mathcal B\), then every action value at every belief in \(\mathcal B\) is exactly recoverable from \(E_{B,a}\):

\[
Q^H_\beta(a)
=
\max_{\alpha\in E_{B,a}}
\langle\beta,\alpha\rangle.
\]

### Proof

This is the defining equality of envelope sufficiency. The point is semantic: every policy vector outside \(E_{B,a}\) may be discarded for this belief family without changing the support function. ∎

## 4.3 Full-simplex and reachable-belief widths

Two belief domains are especially important.

### Full-simplex width

\[
\mathcal B_{\mathrm{all}}
=
\Delta(X_B).
\]

This asks for a representation valid for every possible weighting of the declared fiber. It is stronger than a seat normally needs.

### Reachable-belief width

Let

\[
\mathcal B_{\mathrm{reach}}(B)
\]

be the set of posteriors that can arise from the declared initial belief under legal focal policies, the fixed field, and positive-probability public observation histories.

Then

\[
W_{\mathrm{reach}}(B,a)
=
W_{\mathcal B_{\mathrm{reach}}(B)}(B,a).
\]

Because

\[
\mathcal B_{\mathrm{reach}}(B)
\subseteq
\Delta(X_B),
\]

we have

\[
\boxed{
W_{\mathrm{reach}}(B,a)
\le
W_{\mathrm{all}}(B,a).
}
\]

### Proof

Any set preserving the upper envelope on the larger domain also preserves it on the subset. Taking minima gives the inequality. ∎

The branch’s full-simplex exposure counts are therefore demanding upper-level diagnostics, not direct counts of policies an actual seat must retain.

## 4.4 Current-belief width

At the single current belief \(\{\beta\}\),

\[
W_{\{\beta\}}(B,a)=1
\]

whenever the maximum is attained, which it always is in the finite model.

That statement alone has no computational content: the hard problem is finding and certifying the maximizing policy. The useful complexity is the size of the **proof or search structure** needed to identify it.

---

# 5. The advantage quotient

Absolute value functions can be high-dimensional for reasons irrelevant to policy choice.

Let \(\alpha_0\) be one fixed reference policy vector, chosen globally across the candidate set under comparison. Define

\[
\delta_\rho
=
\alpha_\rho-\alpha_0.
\]

Then

\[
\langle\beta,\alpha_\rho\rangle
=
\langle\beta,\alpha_0\rangle
+
\langle\beta,\delta_\rho\rangle.
\]

The first term is independent of \(\rho\). Therefore

\[
\boxed{
\arg\max_\rho
\langle\beta,\alpha_\rho\rangle
=
\arg\max_\rho
\langle\beta,\delta_\rho\rangle.
}
\]

More generally, adding the same world function \(g\) to every policy vector changes no policy or root-action correspondence:

\[
\arg\max_\rho
\langle\beta,\alpha_\rho+g\rangle
=
\arg\max_\rho
\langle\beta,\alpha_\rho\rangle.
\]

For absolute \(V\) and \(Q\), retain the common baseline value

\[
\langle\beta,\alpha_0\rangle.
\]

For action choice, the geometry lives in policy differences.

Define the active advantage dimension for an envelope \(E\) by

\[
\boxed{
d_{\mathrm{adv}}(E)
=
\dim
\operatorname{span}
\{
\alpha-\alpha_0:
\alpha\in E
\}.
}
\]

A singleton envelope has

\[
d_{\mathrm{adv}}=0
\]

even if the absolute value closure has dimension \(|X_B|\).

This is the first quantity to measure on the two tense S6b roots.

**Root-comparison caution.** If separate references are chosen for separate root actions, their baseline offsets must be retained. A common translation across all root-action policy vectors preserves the global root argmax automatically; independent per-action translations do not.

---

# 6. Exact policy pruning

## 6.1 Pointwise dominance

For \(\alpha,\alpha'\in\mathbb Q^{X_B}\), write

\[
\alpha\preceq\alpha'
\]

when

\[
\alpha(\xi)\le\alpha'(\xi)
\qquad
\forall\xi\in X_B.
\]

## 6.2 Theorem — pointwise dominance pruning

If

\[
\alpha\preceq\alpha',
\]

then for every belief \(\beta\in\Delta(X_B)\),

\[
\langle\beta,\alpha\rangle
\le
\langle\beta,\alpha'\rangle.
\]

Therefore \(\alpha\) may be removed from every value computation over nonnegative beliefs.

If the inequality is strict on at least one positive-mass world, then \(\alpha\) is not optimal at that belief. It may still tie at beliefs assigning zero mass to every strict coordinate.

### Proof

Every belief coordinate is nonnegative. Multiply each pointwise inequality by \(\beta(\xi)\) and sum. ∎

## 6.3 Positive composition preserves dominance

Let \(P\) be any nonnegative stochastic or substochastic transition operator. If

\[
f\le g
\]

pointwise, then

\[
Pf\le Pg
\]

pointwise.

### Proof

Each coordinate of \(Pf\) and \(Pg\) is a nonnegative linear combination of the corresponding values of \(f\) and \(g\). ∎

Consequently, replacing a dominated continuation vector by a dominating vector at one successor information interface cannot make any predecessor policy worse. This is the mathematical basis for exact backward Pareto pruning under the fixed field.

Policy vectors must still be composed according to the real observation partition: one successor choice per focal observation, never one choice per hidden emission.

## 6.4 Decision deadness and universal dominance

These are different exact phenomena.

### Decision deadness

An information state is decision-dead when

\[
\alpha_\rho=\alpha_{\rho'}
\qquad
\forall\rho,\rho'\in\mathcal R_H(I).
\]

The entire policy set has one value vector. All focal max nodes below that state may be eliminated.

### Universal dominance

An information state has a universal dominant policy \(\rho^\star\) when

\[
\alpha_\rho\preceq\alpha_{\rho^\star}
\qquad
\forall\rho.
\]

The policy set may contain many distinct vectors, but its Pareto frontier is one point. The continuation may be fixed to \(\rho^\star\).

The completed S6b cases certify the second phenomenon. The S6c inspection suggests that many of them may satisfy the stronger first phenomenon.

---

# 7. Commuting actions and order exchangeability

The grade-three deadness specimens suggest that policy multiplicity often comes from choosing the order of two remaining tiles.

The right general object is an outcome-decorated stochastic kernel.

Let

\[
\mathsf K_a
\]

denote the kernel induced by choosing action block \(a\), including

- field probability;
- public observation;
- count-free or richer outcome increment;
- successor information interface;
- declared role transport.

Composition is the usual kernel composition, with outcome increments combined in the selected monoid.

## 7.1 Theorem — commuting-kernel exchangeability

Suppose two lawful focal action blocks \(a\) and \(b\) satisfy, after the declared action, observation, role, and successor transports,

\[
\boxed{
\mathsf K_a\star\mathsf K_b
=
\mathsf K_b\star\mathsf K_a
}
\]

as joint kernels of

\[
(\text{public observation trace},
 \text{accumulated outcome},
 \text{successor information interface}).
\]

Then every continuation that differs only by exchanging the order of \(a\) and \(b\) has the same policy-indexed terminal law and therefore the same value for every utility readable from the preserved outcome contract.

### Proof

The displayed equality gives the same law after the two action blocks, including the same transported successor interface. Compose both sides with the same lawful successor policy kernel. Associativity of kernel composition preserves equality. Every terminal readout therefore has the same law. ∎

This theorem is stronger and more reusable than a detector tied to one grade. A useful D1 result should eventually be promoted into a trace-equivalence or partial-order-reduction layer.

**Count boundary.** Exchangeability under count-free tricks does not imply exchangeability under Straight score. The kernel equality must include the richer count or role-capture increment before the stronger claim is made.

---

# 8. Exact lower and upper certificates

The opening action can be certified without computing every \(Q^H(a)\) exactly.

## 8.1 Lawful lower bound

Choose any finite candidate set

\[
\mathcal C_a
\subseteq
\mathcal R_H(B,a).
\]

Define

\[
\boxed{
L_a(\beta)
=
\max_{\rho\in\mathcal C_a}
\langle\beta,\alpha_\rho\rangle.
}
\]

Because every candidate is lawful,

\[
L_a(\beta)\le Q^H_B(a).
\]

## 8.2 Relaxed upper bound

Choose any finite relaxation

\[
\mathcal R_H(B,a)
\subseteq
\mathcal R^+_a.
\]

Define

\[
\boxed{
U_a(\beta)
=
\max_{\rho\in\mathcal R^+_a}
\langle\beta,\alpha_\rho\rangle.
}
\]

Then

\[
Q^H_B(a)\le U_a(\beta).
\]

The canonical first relaxation is treatment \(C\): the root action remains common across worlds, but the complete world is revealed before later focal decisions. This keeps the field fixed and relaxes only focal information.

Perfect-information minimax is a different operator and is not automatically a valid upper bound for the fixed stochastic field.

## 8.3 Theorem — exact value sandwich

For every root action \(a\),

\[
\boxed{
L_a(\beta)
\le
Q^H_B(a)
\le
U_a(\beta).
}
\]

If

\[
L_a(\beta)=U_a(\beta),
\]

then both equal the exact hidden-information action value \(Q^H_B(a)\).

### Proof

The candidate policy set is a subset of the lawful policy set, which is a subset of the relaxed policy set. Maximizing the same objective over nested finite sets gives the inequalities. Equality of the endpoints forces equality of the middle value. ∎

## 8.4 Theorem — exact optimal-action certificate

Suppose there exists a root action \(a^\star\) such that

\[
\boxed{
L_{a^\star}(\beta)
\ge
U_a(\beta)
\qquad
\forall a\ne a^\star.
}
\]

Then \(a^\star\) is an exact optimal root action under treatment \(H\).

If every inequality is strict, then \(a^\star\) is uniquely optimal.

### Proof

For every competing action \(a\),

\[
Q^H_B(a^\star)
\ge
L_{a^\star}
\ge
U_a
\ge
Q^H_B(a).
\]

Thus no competitor has larger exact value. Strict inequalities give uniqueness. ∎

This theorem is central. To select the optimal opening tile, the solver does **not** need an exact solution for every action. It needs

- one sufficiently strong lawful lower witness for the winning action;
- sufficiently tight valid upper bounds for the alternatives.

## 8.5 Approximate corollary

If

\[
L_{a^\star}
\ge
\max_{a\ne a^\star}U_a-\varepsilon,
\]

then the regret of \(a^\star\) is at most \(\varepsilon\).

This is useful later, but the immediate program remains exact: drive the certified gap to zero or separate the actions.

---

# 9. Adaptive information gluing

Treatment \(C\) can be much easier than treatment \(H\), but it permits continuation policies to vary by hidden world. Treatment \(H\) glues those policy coordinates at indistinguishable nodes.

This suggests exact constraint generation.

## 9.1 Relaxed policy sequence

Fix a root action \(a\). Begin with

\[
\mathcal R_0(a)=\mathcal R_C(B,a),
\]

or another declared relaxation satisfying

\[
\mathcal R_H(B,a)\subseteq\mathcal R_0(a).
\]

At iteration \(k\), solve exactly over

\[
\mathcal R_k(a)
\supseteq
\mathcal R_H(B,a).
\]

If an optimal relaxed policy is information-consistent under \(H\), it is an exact \(H\)-optimal policy.

Otherwise find a focal information class on which that relaxed policy selects different transported actions at nodes the focal player cannot distinguish. Add the required equality constraint, producing

\[
\mathcal R_{k+1}(a)
\subsetneq
\mathcal R_k(a),
\]

while preserving

\[
\mathcal R_H(B,a)
\subseteq
\mathcal R_{k+1}(a).
\]

## 9.2 Theorem — finite adaptive gluing

The procedure above is exact and terminates after finitely many gluing additions.

More precisely:

1. every relaxed optimum gives an upper bound on \(Q^H_B(a)\);
2. if the relaxed optimal face contains an \(H\)-consistent policy, the upper bound is exact;
3. every valid information-gluing equality preserves every \(H\)-policy;
4. adding a violated equality removes at least the selected unlawful relaxed policy;
5. because the deterministic relaxed policy set and the collection of information equalities are finite, repeated refinement terminates, at worst at the exact \(H\) policy class.

### Proof

Items 1 and 3 follow from set inclusion. For item 2, an \(H\)-consistent relaxed optimizer belongs to both the relaxation and the exact policy class, so the relaxed and exact maxima coincide. A violated equality is obeyed by every \(H\)-policy but not by the selected relaxed policy, proving item 4. Finiteness gives termination. ∎

## 9.3 Exposed-face stopping criterion

It is not necessary that the first relaxed optimizer returned by an implementation be lawful. The exact criterion is

\[
\boxed{
\operatorname{Opt}(\mathcal R_k,\beta)
\cap
\mathcal R_H
\ne
\varnothing.
}
\]

This is the policy-set form of v0.4’s zero-information exposed-face criterion.

## 9.4 Bounds during gluing

Let

\[
U_a^{(k)}
=
\max_{\rho\in\mathcal R_k(a)}
J_B(\rho).
\]

Then

\[
U_a^{(0)}
\ge
U_a^{(1)}
\ge
\cdots
\ge
Q^H_B(a).
\]

Any discovered lawful candidate policy supplies a lower bound. The algorithm can stop for root-action selection as soon as the cross-action certificate of §8.4 holds, even if some individual upper/lower gaps remain open.

## 9.5 Scheme-generalized gluing cuts

A concrete violation names two or more latent decision nodes that belong to the same focal information class but received different actions.

A Scheme/Fix may generalize that violation into a transported family of equalities.

Such a cut is valid only when it proves:

1. every matched pair belongs to the same declared focal information class, up to action transport;
2. the cut enforces equality rather than branching on the hidden Scheme event;
3. every lawful \(H\)-policy satisfies it.

The Scheme is an analyst/compiler language here. It does not become a player observation.

This is the mathematically lawful CDCL-style use of Scheme/Fix.

---

# 10. Scheme/Fix as an exact weighted-integration language

The previous Scheme work used formulas mainly to define cells, roles, or descriptor states. The opening evaluator needs a second use:

\[
\boxed{
\text{represent and integrate outcome functions over a broad exact fiber.}
}
\]

## 10.1 Observation likelihood as a Scheme simple function

Suppose hidden seat \(s\) uses the uniform-legal field. In latent state \(\xi\), let

\[
L_s(\xi)
\]

be its legal tile set.

For a public observation that seat \(s\) plays tile \(d\),

\[
\Pr(d\mid\xi)
=
\frac{
\mathbf1[d\in L_s(\xi)]
}{
|L_s(\xi)|
}.
\]

Because a seat has at most seven tiles,

\[
\boxed{
\Pr(d\mid\xi)
=
\sum_{k=1}^{7}
\frac1k\,
\mathbf1[
d\in L_s(\xi)
\land
|L_s(\xi)|=k
].
}
\]

Every event on the right can be expressed using the existing relational vocabulary plus exact legal-set cardinality:

- holder;
- led context;
- follow incidence;
- void;
- actor;
- table state;
- legal action;
- quota/cardinality.

If the public token distinguishes lead, follow, and slough, include that typed classification in the event.

Thus every fixed-field public likelihood is a rational simple function generated by finite Scheme events.

## 10.2 Finite Scheme algebra

For a finite latent domain \(X_t\), let

\[
\mathfrak A_t
\]

be a finite Boolean algebra of Scheme/Fix-definable events.

Let

\[
\mathrm{SF}(\mathfrak A_t)
\]

be the rational vector space of functions constant on the atoms of \(\mathfrak A_t\).

For a typed action/observation kernel \(K_{a,o}\), define the unnormalized preexpectation

\[
\operatorname{Pre}_{a,o}(f)(x)
=
\sum_{x'}
K_{a}(x;o,x')f(x').
\]

## 10.3 Theorem — finite Scheme-mass closure

Assume, at every reachable interface:

1. \(1\in\mathrm{SF}(\mathfrak A_t)\);
2. every terminal readout needed by the selected outcome contract lies in the terminal Scheme algebra;
3. for every successor atom \(A'\in\mathfrak A_{t+1}\),

   \[
   \operatorname{Pre}_{a,o}(\mathbf1_{A'})
   \in
   \mathrm{SF}(\mathfrak A_t);
   \]

4. the required immediate outcome functions are in \(\mathrm{SF}(\mathfrak A_t)\).

Then the vector of current atom masses determines exactly:

- every positive-probability observation likelihood;
- every successor atom mass after Bayesian normalization;
- every expected terminal readout;
- every fixed-policy value whose branch decisions are lawful under the declared information structure.

### Proof

For successor atom \(A'\), its unnormalized posterior mass is

\[
\widetilde\beta'(A')
=
\mathbb E_{\beta}
[
\operatorname{Pre}_{a,o}(\mathbf1_{A'})
].
\]

By assumption, the preexpectation is constant on current atoms, so its expectation is determined by current atom masses. Summing over \(A'\) gives the observation probability; division gives normalized successor masses. Induction over the finite grade proves exact filtering. Expected terminal readouts are linear combinations of terminal atom masses. A fixed lawful policy chooses one action per observed information state, so the same induction evaluates it exactly without hidden-world branching. ∎

This is a belief- and purpose-relative exact abstraction. It is weaker than universal strong lumpability.

## 10.4 Explicit atoms are not required

The theorem is stated with a Boolean algebra for clarity. An implementation need not enumerate all atoms.

A Scheme outcome function may be represented as a shared arithmetic/Boolean DAG built from:

- indicator nodes \(\mathbf1_F\);
- rational constants;
- addition and subtraction;
- multiplication;
- typed preexpectation;
- exact finite max only at lawful information states;
- outcome-monoid decoration.

A high-dimensional function can have a small circuit. Full predictive rank does not preclude this.

The relevant measurement is therefore

\[
\boxed{
\text{exact circuit or decision-diagram size},
}
\]

not merely span dimension or partition count.

## 10.5 Exact weighted model counting

Under a uniform belief on an exact support fiber,

\[
\mathbb E_\beta[\mathbf1_F]
=
\frac{
|\{\omega\in\Phi(K):\omega\models F\}|
}{
|\Phi(K)|
}.
\]

The support-normal-form counter can evaluate many grounded holder/void constraints without enumerating worlds:

1. enumerate only the bounded output-role assignments required by \(F\);
2. translate a grounded case into forced and forbidden holder edges;
3. reduce the residual matching/capacity system;
4. count exact completions;
5. combine equality-pattern branches and Fix unions with exact set semantics;
6. memoize common subformulas.

Overlapping branches must not be summed as independent mass. Use a disjoint form, exact inclusion–exclusion, a BDD/ZDD-style representation, or another checked union counter.

For nonuniform beliefs, counts become exact weights. Cheap compilation then requires a proved factorization of the belief or likelihood model; it is not automatic.

## 10.6 Policy observability firewall

Scheme/Fix now has two roles.

### Integration formula

An integration formula may inspect hidden latent structure. It represents an analyst-side function to be averaged.

### Executable policy guard

A policy guard may control an action only if it is measurable with respect to the focal information partition.

Write

\[
\boxed{
\mathcal I_m\vdash F:\mathrm{observable}
}
\]

when

\[
x\sim_{\mathcal I_m}y
\Longrightarrow
(x\models F\iff y\models F).
\]

Only a Fix carrying this judgment may branch an executable policy.

A hidden-world Scheme may evaluate one lawful policy compactly. It may not choose a different policy in each hidden cell. That would recreate strategy fusion.

---

# 11. Outcome and valuation re-entry

The count-free scalar \(\alpha_\rho\) is only the first contract.

## 11.1 Expected additive feature vector

For each latent world and policy, retain

\[
\mu_\rho(\xi)
=
\mathbb E[
\phi_T(c)
\mid
\xi,\rho,\sigma_{-m}
],
\]

where

\[
\phi_T(c)
=
\left(
t_T(c),
(x_{T,d}(c))_{d\in\mathcal D}
\right).
\]

A coefficient pair \(v=(b,w)\) gives

\[
\alpha_\rho^v(\xi)
=
\langle v,\mu_\rho(\xi)\rangle.
\]

The legal conservation law

\[
\sum_d x_{T,d}=4t_T
\]

induces the gauge

\[
(b,w)\sim(b-4c,w+c\mathbf1).
\]

The policy-envelope machinery acts on the quotient valuation space exactly as in v0.4 §8.

## 11.2 Role-indexed re-entry

Under a nontrivial tile transport, physical valuations do not automatically descend.

The safe order is:

1. preserve the count-free process;
2. transport declared domino roles;
3. recover role-capture features from the transported role trace;
4. apply role coefficients;
5. apply the gauge when capture completeness has been proved;
6. require a valuation stabilizer if one fixed physical \(w:\mathcal D\to\mathbb R\) is expected to survive the transport.

## 11.3 Cone dominance

Let \(C\) be a declared cone of allowed valuation directions.

A policy \(\rho^\star\) dominates \(\rho\) over \(C\) when, for every latent world,

\[
\langle v,
\mu_{\rho^\star}(\xi)-\mu_\rho(\xi)
\rangle
\ge0
\qquad
\forall v\in C.
\]

Equivalently, every feature difference lies in the dual cone \(C^\ast\).

Such a policy may be pruned or fixed for every valuation in \(C\).

A singleton frontier under expected tricks need not remain singleton under ordinary count or arbitrary role values. The cone must be named.

## 11.4 Nonlinear utility

For make probability, mark utility, risk, or score thresholds, expected feature vectors may be insufficient.

Replace \(\mu_\rho(\xi)\) by the policy-indexed terminal feature law or full terminal law. The envelope, sandwich, integration, and gluing theorems remain valid because they depend only on linear expectation of the selected terminal utility against a retained law.

---

# 12. The combined exact architecture

The components now fit into one proof-oriented solver.

For each root action \(a\):

1. maintain a small set of lawful candidate policies \(\mathcal C_a\);
2. represent each candidate’s outcome function by a Scheme arithmetic circuit;
3. integrate those circuits exactly under the current support and belief to obtain \(L_a\);
4. solve a relaxed policy problem to obtain \(U_a\);
5. if necessary, add information-gluing cuts and recompute the upper bound;
6. discover new lawful policies from counterexamples, tense regions, or the relaxed solution;
7. stop when the action certificate separates one root action.

## 12.1 Theorem — decision-sparse exact factorization

Assume:

1. every policy in \(\mathcal C_a\) is information-consistent under \(H\);
2. every relaxation \(\mathcal R^+_a\) contains the exact policy class;
3. the Scheme circuit evaluator returns exact expectations under \(\beta\);
4. every gluing cut is valid for all \(H\)-policies;
5. all action, observation, role, field, and utility transports obey the declared interfaces.

Then the lower and upper values computed by the architecture satisfy

\[
L_a\le Q^H_B(a)\le U_a
\]

for every root action. Any action certified by §8.4 is an exact optimal play in the original fixed-field information game.

### Proof

Conditions 1 and 2 give the nested policy sets underlying the sandwich theorem. Condition 3 makes every reported objective exact. Condition 4 preserves the exact policy set inside every refined relaxation. Condition 5 ensures that the compared policies and outcomes belong to the same typed game. Apply §§8–9. ∎

This theorem does not claim that the candidate set, circuit, or gluing set is small. That is the experimental question.

---

# 13. Why the two stopped low-trump roots matter

The two S6b frontier explosions are the smallest known exact specimens of genuine strategy-side complexity.

They should be treated as mathematical microscopes, not implementation failures.

For the first information state where the frontier has width greater than one, extract two incomparable policies \(\rho,\sigma\) and define

\[
\Delta_{\rho,\sigma}(\xi)
=
\alpha_\rho(\xi)-\alpha_\sigma(\xi).
\]

Partition the fiber into

\[
X^+
=
\{\xi:\Delta>0\},
\]

\[
X^0
=
\{\xi:\Delta=0\},
\]

\[
X^-
=
\{\xi:\Delta<0\}.
\]

The next ontology should be learned from these sets.

Questions to answer exactly:

1. At which public information state does the first incomparability appear?
2. What two action choices create it?
3. What is the affine/advantage dimension of the frontier at that point?
4. Which policies are exposed by reachable posteriors rather than arbitrary simplex beliefs?
5. Which Scheme/Fix relation separates \(X^+\) from \(X^-\)?
6. Is the separator about:
   - trump exhaustion;
   - successor context;
   - partner timing;
   - control return;
   - forced follow;
   - beater chains;
   - causal seat position;
   - a conjunction of these?
7. Can one generalized information-gluing cut remove a whole family of unlawful revealed policies?
8. Can one commuting-action theorem collapse a whole family of lawful policies?

Do not enlarge the generic atom registry before this anatomy has been extracted. These two roots can tell the language what it is missing.

---

# 14. Experimental program

All experiments below remain exploratory and require exact-rational equality against an enumerated authority on the declared small carrier.

## Experiment A — complete the S6c deadness run

Run the already adjudicated detector experiment to completion.

Measure separately:

- forced nodes;
- exact decision-dead nodes;
- universally dominant but non-dead nodes;
- D0 hits;
- D1 exchangeability hits;
- false positives, which must be zero;
- detector cost;
- exact solve-cost dividend.

A successful D1 should be restated as a commuting-kernel theorem with a declared outcome contract.

## Experiment B — tense-root anatomy

On both stopped non-boss trump roots:

1. identify the first frontier split;
2. retain the complete frontier at that smallest split;
3. compute pairwise advantage sign sets;
4. compute

   \[
   d_{\mathrm{adv}};
   \]

5. enumerate reachable posteriors on that small horizon;
6. measure \(W_{\mathrm{reach}}\);
7. synthesize minimal Scheme separators for the advantage sign sets;
8. record every failed separator as a counterexample pair.

**Primary result:** a causal relational description of the first genuine policy incompatibility.

## Experiment C — Scheme-weighted filtering

Choose a grade-three or grade-four support fiber that can still be exhaustively enumerated.

Represent:

- the initial uniform belief;
- one hidden uniform-legal observation likelihood;
- the posterior after that observation;
- one fixed lawful policy value function.

Use Scheme/Fix arithmetic circuits and exact support counting.

Assert bit-exact equality with concrete enumeration for:

- observation probability;
- every successor support mass;
- every selected posterior moment;
- the fixed-policy expected value.

Report:

\[
\text{world count},
\quad
\text{circuit nodes},
\quad
\text{unique grounded subproblems},
\quad
\text{counter calls},
\quad
\text{runtime}.
\]

The decisive question is whether circuit growth is substantially below world growth.

## Experiment D — adaptive gluing

On one tense grade-three root:

1. begin from treatment \(C\);
2. solve the relaxed action value;
3. test whether the optimal exposed face contains an \(H\)-lawful policy;
4. if not, emit one violated information equality;
5. generalize it with Scheme only after proving validity;
6. re-solve;
7. stop at exactness.

Report:

- initial \(C-H\) information price;
- number of concrete cuts;
- number of generalized cuts;
- worlds/nodes covered per cut;
- relaxed upper value after each cut;
- best lawful lower value after each iteration;
- iteration where the exact gap closes.

**Primary result:** active gluing complexity, not world count.

## Experiment E — lower/upper root-action certification

On a tractable multi-action root:

- lower bounds from a tiny lawful policy library;
- upper bounds from \(C\);
- tighter upper bounds from the first few gluing iterations.

Test whether the exact optimal root action is certified before any action value is fully solved.

Report the first iteration satisfying

\[
L_{a^\star}\ge\max_{a\ne a^\star}U_a.
\]

This is the direct small-scale model of the intended opening solver.

## Experiment F — reachable envelope

For each measured root action, compare:

\[
N_{\mathrm{pol}},
\quad
N_{\mathrm{par}},
\quad
W_{\mathrm{all}},
\quad
W_{\mathrm{reach}},
\quad
d_{\mathrm{adv}}.
\]

Do not conflate them.

A policy exposed by an impossible belief is not required by the actual seat.

## Experiment G — count and score lift

After the count-free experiments are stable:

1. lift the retained policy vectors to trick-and-capture features;
2. apply the gauge;
3. test ordinary Straight count;
4. re-run dominance under the relevant valuation cone;
5. identify which count-free dead/dominant regions survive;
6. record every frontier split caused by count re-entry.

## Experiment H — grade climb

Only after Experiments C–E show positive structure, climb one grade at a time.

At each grade, measure four independent growth rates:

\[
\begin{array}{ll}
\text{truth size:}&|X|,\\
\text{circuit size:}&N_{\mathrm{circ}},\\
\text{active policy size:}&W_{\mathrm{reach}}\text{ or retained candidates},\\
\text{constraint size:}&N_{\mathrm{glue}}.
\end{array}
\]

The opening program lives if the last three grow materially slower than \(|X|\), even when structural classes and predictive rank do not.

---

# 15. Implementation contracts

The implementation may count on the following.

## 15.1 Exact mechanics and support remain authoritative

No strategic compression changes:

- legal plays;
- trick winners;
- support normal form;
- public void deductions;
- evidence;
- field likelihood;
- information states.

A Scheme circuit is not a rival game state.

## 15.2 One policy per information state

Every executable policy variable is indexed by the player’s actual information state, not by hidden world.

Any hidden-world formula used to select actions is strategy fusion unless it carries an observability proof.

## 15.3 Candidate policies are primal witnesses

A lawful candidate policy gives a valid lower bound immediately.

Its value must be integrated under the declared belief and field, not graded worldwise and averaged after world-specific optimization.

## 15.4 Relaxations are upper witnesses

Treatment \(C\) is a valid upper bound on treatment \(H\) when the root action, field, belief, mechanics, and utility are held fixed and only later focal information is relaxed.

PI minimax is a different operator.

## 15.5 Exact action can precede exact value

The action certificate of §8.4 is sufficient. Do not continue solving every action after one action has a lawful lower bound above every competitor’s valid upper bound.

## 15.6 Rank is no longer the runtime target

Full predictive rank is compatible with a small policy envelope and a small exact circuit.

Do not infer runtime impossibility from S6a.

## 15.7 A class store is not automatically an accelerator

The class and tablebase results remain useful for:

- reuse;
- transport;
- canonical storage;
- suffix libraries;
- upper-bound evaluation;
- cross-coordinate amortization.

They do not short-circuit first construction merely because a quotient exists.

## 15.8 Count-free claims stay count-free

No count-free class, deadness result, dominance result, or transport survives score re-entry without a feature-law, cone, role, or stabilizer proof.

## 15.9 Caps produce no hidden verdict

A stopped frontier has no reported size bound unless a separate theorem provides one. Record the partial object and the exact stop point.

---

# 16. Claim ledger

## 16.1 Branch-derived exploratory findings used here

This document treats the following as reported exact exploratory results of PR 4, not independently rerun facts:

1. the complete 64-class count-free last-trick pip-trump alphabet;
2. first-play structural rigidity and the \(1{,}184{,}040\) opening-hand count;
3. predictive value dimensions \(1\), \(42\)–\(59\), and \(1461\)–\(1680\) at grades one through three;
4. full value dimension on one grade-three coordinate;
5. seven completed singleton Pareto frontiers at grade three;
6. two stopped non-boss trump leads at the declared frontier cap;
7. the preliminary deadness/indifference anatomy motivating S6c;
8. the recurring suffix-parts catalog and the negative first-build economics of the class/tablebase routes.

The implementation should consult the exact result and ruling files for all carrier, field, belief, and scope qualifiers.

## 16.2 Theorems proved in this document

Under the displayed finite-model assumptions:

1. policy-envelope sufficiency;
2. monotonicity of decision width under belief-domain restriction;
3. common-translation/advantage invariance;
4. pointwise dominance pruning;
5. preservation of dominance under positive composition;
6. commuting-kernel exchangeability;
7. the lower/upper value sandwich;
8. the exact root-action certificate;
9. finite adaptive information gluing;
10. finite Scheme-mass closure;
11. the combined decision-sparse exact-factorization theorem.

These are prose proofs, not machine-checked theorems.

## 16.3 Open mathematical questions

1. How does \(W_{\mathrm{reach}}\) grow with grade?
2. Is active advantage dimension small in tense regions?
3. Does the first policy incompatibility admit a compact Scheme/Fix separator?
4. Can Scheme-weighted posterior and policy integration be represented by circuits whose size grows far below the fiber?
5. Is the number of active information-gluing cuts small?
6. Which count-free dominant policies survive ordinary Straight count?
7. Do order-exchangeability diamonds recur at earlier grades?
8. Can a compact exact upper-bound hierarchy certify the first play without computing exact \(Q\) for every action?
9. Which parts of the suffix catalog become reusable arithmetic-circuit components at the opening?
10. What changes when the continuation field is learned, strategic, or equilibrium-derived rather than fixed?

---

# 17. Final mathematical direction

The recent negative results do not say that the opening hand requires one solve per hidden world.

They say that the opening cannot be compressed by insisting that all future truths share one small state label or one small universal linear basis.

The next exact object is a proof of optimality:

\[
\boxed{
\begin{array}{c}
\text{lawful candidate policy}\\
\downarrow\\
\text{exact Scheme outcome circuit}\\
\downarrow\\
\text{exact weighted value }L
\end{array}
\qquad
\begin{array}{c}
\text{information relaxation}\\
\downarrow\\
\text{Scheme-generalized gluing cuts}\\
\downarrow\\
\text{exact upper value }U.
\end{array}
}
\]

The two sides meet when

\[
L=U
\]

for one action value, or—more economically—when

\[
L_{a^\star}
\ge
U_a
\qquad
\forall a\ne a^\star.
\]

Then the optimal play is proved even if:

- every opening hand has a distinct structural coordinate;
- the universal predictive space has full dimension;
- the full hidden fiber contains \(399{,}072{,}960\) worlds;
- most of those worlds were never individually solved.

The strongest current hypothesis is therefore:

\[
\boxed{
\text{Straight 42 is information-rich and outcome-rich, but decision-sparse.}
}
\]

The implementation program should now measure three quantities—not one:

\[
\boxed{
\text{exact circuit size}
\quad+\quad
\text{active policy-envelope size}
\quad+\quad
\text{active information-gluing size}.
}
\]

If those quantities remain in the millions while the full saturated world/tree representation is orders of magnitude larger, the opening has been factored at the right mathematical level.
