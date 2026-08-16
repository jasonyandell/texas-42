# Second Audit and Unifying Addendum to *Decision-Sparse Exact Solving for Straight Texas 42*, v0.1

## Validated repairs, remaining scope corrections, and the action-separation calculus

**Status:** exploratory mathematical review and implementation handoff  
**Date:** 2026-08-13  
**Repository context:** `jasonyandell/texas-42`, PR 4, branch `worktree-walt-s2`  
**Reviewed branch head:** `f2971aca2aa779f0f1d7dc505d8b8591c29e49d3`

**Governing sources:**

- `walt/math/decision_sparse_exact_solving_v0.1.md` — the received parent handoff;
- `walt/CENSUS-RULINGS.md`, decision-sparse intake audit DS-A1–DS-A18;
- `walt/math/decision_sparse_exact_solving_v0.1_errata.md` — repaired statements E1–E6.5;
- the standing v0.4, v0.5, and v0.6 mathematical tracks and their adjudicated exploratory results.

**Reading rule.** The parent remains a provenance artifact. Where the parent and its errata differ, the errata governs. This second audit does not replace either file. It validates the principal repairs, narrows several overbroad readings, and gives the implementation side one unified exact architecture.

**Tier.** Everything here remains exploratory prose mathematics. No claim is promoted by this review.

---

# 0. Executive verdict

The last two commits contain legitimate mathematical findings. They do not weaken the decision-sparse program. They remove two false shortcuts, repair one genuinely unsound theorem, and make the lower/upper-bound architecture substantially safer.

The strongest durable synthesis is still

\[
\boxed{
\text{the latent truth can be high-dimensional while the exact decision is sparse.}
}
\]

The project should no longer seek one representation that compresses every latent distinction, every normalized posterior, every continuation test, and every policy simultaneously. The exact opening problem can instead be attacked as a proof of an action maximum:

\[
\boxed{
\begin{array}{c}
\text{lawful primal policies}\
+\ \text{purpose-specific exact evaluators}\
+\ \text{action-conditioned upper relaxations}\
+\ \text{valid information-gluing cuts}\
\Longrightarrow\
\text{root-action separation}.
\end{array}
}
\]

For every root action \(a\), maintain

\[
L_a\le Q^H_B(a)\le U_a.
\]

The opening action is proved as soon as one action \(a^\star\) satisfies

\[
\boxed{
L_{a^\star}\ge U_a
\qquad\forall a\ne a^\star.
}
\]

This does not require every \(Q^H_B(a)\) to be solved exactly. It requires a strong lawful lower witness for the likely winner and valid upper witnesses for its competitors.

---

# 1. Findings from the intake audit that are correct

## 1.1 The parent’s order-exchange theorem was unsound

The parent §7.1 asserted that if the two-block kernels for \(ab\) and \(ba\) agree after declared transports, then every continuation using “the same” successor policy has the same terminal law.

That conclusion does not follow under perfect recall. A lawful continuation policy is a function of the public history. The two orders generate transported histories, not identical histories:

\[
\tau_{ab}\ne\tau_{ba}.
\]

A policy may lawfully act differently after those histories. Therefore one cannot compose both sides with a literally identical history-indexed continuation and infer equality.

The repaired Theorem E1 moves the conclusion to the correct invariant object. Under:

1. a declared bijection or involution \(\Theta\) between the two trace families;
2. an intertwining identity for the two-block kernels;
3. closure of the lawful continuation-policy class under \(\Theta\);

the two orders have the same **set of achievable terminal laws**. Consequently they have the same optimum for every utility readable from the preserved outcome contract.

The proof is correct: a continuation \(\pi\) after \(ab\) is paired with the transported continuation \(\pi\circ\Theta^{-1}\) after \(ba\). The theorem does not pretend that one fixed perfect-recall policy behaves identically on different histories.

### Nuance

The parent already mentioned declared transports, so the audit’s “literal trace supports are disjoint” objection is best read as an underspecification warning, not as the decisive defect. The decisive defect is the invalid fixed-policy composition step. That defect alone requires the E1 repair.

## 1.2 The affine advantage dimension is the correct definition

The parent used a reference-based quantity

\[
\dim\operatorname{span}\{\alpha-\alpha_0:\alpha\in E\},
\]

while allowing \(\alpha_0\) to be outside the envelope \(E\). It then claimed a singleton envelope always has dimension zero. Those statements are incompatible.

The errata correctly defines

\[
\boxed{
d_{\mathrm{adv}}(E)
=
\dim\operatorname{span}
\{\alpha-\alpha':\alpha,\alpha'\in E\},
}
\]

the affine dimension of \(E\).

This definition is:

- independent of a reference choice;
- invariant under a common translation of every policy vector;
- zero exactly for a singleton envelope;
- bounded by \(\min(|E|-1,|X|)\).

The errata’s off-by-one proposition is also exact: the external-reference span has the affine dimension when the reference lies in \(\operatorname{aff}(E)\), and one greater otherwise.

## 1.3 The useful treatment-C upper witness must be action-conditioned

The already measured world-informed aggregate has the form

\[
U^{\mathrm{agg}}
=
\mathbb E_\beta\left[\max_a V_a^\ast(\xi)\right].
\]

It is a valid upper bound, but it maximizes over root actions separately inside each world. Therefore it is the same bound for every root action and is generally useless for action separation.

For one root action \(a\), define instead

\[
V_a^\ast(\xi)
=
\max_{c\text{ world-dependent after }a}
\mathbb E[U\mid \xi,a,c,\sigma_{-m}],
\]

and

\[
\boxed{
U_a
=
\mathbb E_\beta[V_a^\ast].
}
\]

Every lawful hidden-information policy with root action \(a\) induces one admissible continuation behavior in each fixed world, so

\[
\alpha_\rho(\xi)\le V_a^\ast(\xi)
\]

pointwise. Averaging and maximizing gives

\[
\boxed{
Q^H_B(a)\le U_a.
}
\]

This is the correct one-sided use of the strategy-fusion relaxation.

## 1.4 The primal-witness failure mode is real and dangerous

A lawful lower witness must be the exact value of a **fixed information-consistent policy**:

\[
\alpha_\rho(\xi)
=
\mathbb E[U\mid\xi,\rho,\sigma_{-m}],
\]

with every later focal action supplied by \(\rho\), not selected by a hidden-world optimizer.

For a finite lawful candidate set \(\mathcal C_a\subseteq\mathcal R_H(B,a)\),

\[
\boxed{
L_a
=
\max_{\rho\in\mathcal C_a}
\langle\beta,\alpha_\rho\rangle
\le
Q^H_B(a).
}
\]

If a candidate is priced using a revealed-world, per-world best-response, or other evaluator that maximizes after conditioning on \(\xi\), the reported quantity is an upper bound, not the candidate’s value. Installing it as \(L_a\) reverses the first inequality in the action-separation chain and can certify a strictly inferior action.

This is a soundness failure, not merely an efficiency problem.

## 1.5 The backward Pareto-pruning completion is sound

Positive stochastic and substochastic operators preserve pointwise dominance:

\[
f\le g\Longrightarrow Pf\le Pg.
\]

The parent stopped there. Exact recursive pruning also needs:

- frontier reproduction after replacing every successor set by its Pareto maxima;
- exact incremental Minkowski folding with pruning after each observation branch;
- explicit acknowledgment that a pruned run cannot recover the unpruned distinct-vector count.

The errata’s E6.2 supplies those missing pieces. The proofs are sound and align with the previously adjudicated Lemma G.

## 1.6 Adaptive information gluing remains exact

Let

\[
\mathcal R_H(B,a)\subseteq\mathcal R_k(a)
\]

be a finite relaxation of the hidden-information policy class. Add only equality constraints that every lawful \(H\)-policy satisfies. Then the relaxed maxima form a descending sequence of valid upper bounds:

\[
U_a^{(0)}\ge U_a^{(1)}\ge\cdots\ge Q^H_B(a).
\]

If the relaxed optimal face contains an \(H\)-consistent policy, the upper bound is exact.

The audit correctly adds two obligations:

1. the relaxed solve must return the exact optimum or a proved upper bound;
2. one unlawful returned optimizer does not prove that the whole optimal face is unlawful.

The stopping condition is

\[
\boxed{
\operatorname{Opt}(\mathcal R_k,\beta)
\cap
\mathcal R_H
\ne\varnothing.
}
\]

An unlawful optimizer licenses another cut. It does not license the claim that the relaxation remains strictly loose.

## 1.7 Universal Scheme atom filtering degenerates on the current carrier

The audit’s central §10.3 observation is legitimate on the current physical-world carrier.

Assume at every reachable interface that:

- the constant \(1\) is retained;
- the Scheme algebra is closed under every typed observation preexpectation of successor atom indicators;
- the field assigns positive probability to every legal play;
- a complete public record names every remaining hidden tile and the seat that played it.

Residualizing \(1\) along a complete record yields

\[
\xi\longmapsto
\Pr_\xi(\text{that complete record}).
\]

On the current physical-world domain this is a positive multiple of one singleton indicator. Every world has at least one positive-probability complete record, so the algebra contains every singleton indicator. Hence its atoms are discrete and

\[
\boxed{
\dim\mathrm{SF}(\mathfrak A)=|X|.
}
\]

Therefore the parent’s universal normalized atom-mass filtering theorem is sound but noncompressive here.

The live escape is not a smaller universal atom algebra. It is a smaller exact circuit for a declared finite family of fixed-policy values, moments, and bounds.

---

# 2. Additional corrections recommended by this second audit

These are proposed mathematical amendments, not yet binding project rulings.

## 2.1 Generalize E1 to transport successor latent states and interfaces

E1’s explicit equation transports traces but leaves the successor latent state \(\xi'\) literally fixed. That is sound for applications in which both action orders land in the same physical successor state, but it is narrower than the surrounding equivariant language.

The fully transported form should use

\[
\boxed{
\mathsf K_{ba}
=
(\Theta_T\times\Theta_M\times\Theta_X)_*
\mathsf K_{ab},
}
\]

where:

- \(\Theta_T\) transports public traces;
- \(\Theta_M\) transports outcome coordinates, often identically;
- \(\Theta_X\) transports successor latent states and their information interfaces.

The policy-class closure hypothesis is then stated on the induced successor-information transport.

The current E1 remains sound as the identity-\(\Theta_X\) specialization.

## 2.2 Add a latent-separation hypothesis to E5

The general v0.4 latent state may be

\[
\xi=(\omega,z),
\]

where \(z\) is persistent field state, policy type, a random tape, or another correlated latent variable. A complete tile-play record can determine the physical world \(\omega\) while leaving \(z\) unresolved.

E5 therefore needs one of the following explicit hypotheses:

### Current-carrier form

The latent domain contains only physical current-world assignments, and a complete public record determines each assignment uniquely.

### General separating-record form

\[
\boxed{
\xi\ne\xi'
\Longrightarrow
\exists r\text{ complete record such that }
\Pr_\xi(r)\ne\Pr_{\xi'}(r).
}
\]

Without this, the proof yields indicators of complete-record equivalence classes, not necessarily singleton indicators of the augmented latent domain.

The same distinction applies to the upper relaxation. Revealing the physical world \(\omega\) is treatment \(C\); revealing all of \(\xi=(\omega,z)\) is a stronger, still lawful upper relaxation, but it should not be named treatment \(C\) without qualification.

## 2.3 Narrow the filtering conclusion

The proved negative is:

\[
\boxed{
\text{universal exact atom-mass filtering under complete-record closure}
\Longrightarrow
\text{discrete atoms}.
}
\]

It does not rule out compact:

- arithmetic or Boolean circuits;
- factorized graphical representations;
- BDD/ZDD-style formulas;
- fixed-prior symbolic propagation;
- purpose-specific moment compilation;
- fixed-policy weighted model counting.

The safe language is **atom-mass linear filtering is noncompressive here**, not “filtering and compression are incompatible” without qualification.

Likewise, the measured predictive rank does not by itself lower-bound the size of an unrestricted nonlinear arithmetic/Boolean DAG. It lower-bounds the corresponding linear factorization target.

## 2.4 State the precise zero-global-gap corollary

If

\[
U^{\mathrm{agg}}=V^H,
\]

then for every exact \(H\)-optimal root action \(a^\star\),

\[
\boxed{
U_{a^\star}
=
Q^H(a^\star)
=
V^H.
}
\]

Proof:

\[
V^H=Q^H(a^\star)
\le U_{a^\star}
\le U^{\mathrm{agg}}
=V^H.
\]

This is real prior evidence for a tight action-conditioned relaxation at the optimal action.

It does **not** imply:

- every action-conditioned bound is tight;
- competitor bounds already separate;
- a lawful primal policy attaining \(V^H\) has already been found.

## 2.5 Retype reachable decision width locally

The parent’s \(W_{\mathrm{reach}}(B,a)\) mixes root policy vectors on \(X_B\) with posteriors that naturally live at changing successor interfaces. Moving every belief to a common full-deal carrier fixes the set-theoretic typing but not the operational semantics: a posterior at history \(h\) was generated by a particular prefix and should evaluate continuation policies beginning at \(h\).

The clean object is local to one information interface \(I\):

\[
\boxed{
W_{\mathrm{reach}}^{\mathrm{loc}}(I,a)
=
\min\left\{
|E|:
\max_{\alpha\in E}\langle\beta,\alpha\rangle
=Q_I(a;\beta)
\quad
\forall\beta\in\mathcal B_{\mathrm{reach}}(I)
\right\},
}
\]

where:

- \(X_I\) is the latent domain at interface \(I\);
- \(\mathcal A_{I,a}\) contains continuation vectors beginning at \(I\);
- \(\mathcal B_{\mathrm{reach}}(I)\subseteq\Delta(X_I)\) consists of posteriors reaching that same interface.

Possible global summaries include:

\[
\max_I W_{\mathrm{reach}}^{\mathrm{loc}}(I,a),
\]

or the size of one transported policy library covering all reachable interfaces. These are different quantities and must not share one name.

Until this typing is fixed, Experiment F should remain pending rather than “designable now.”

## 2.6 Restrict DS-A9’s cone claim to transport-invariant valuations

DS-A9 says that J-0 or J-1 plus the no-count guard makes the feature difference vanish identically and therefore works for every valuation cone. That is too strong.

The existing results establish equality for:

- count-free trick value;
- ordinary trick-plus-count value under the guard;
- in J-1, valuations invariant under the declared transposition.

They do not establish equality of the full physical tile-capture feature vector.

Two zero-count focal tiles may be exchanged between tricks. Ordinary count cannot distinguish them, but an arbitrary physical valuation can:

\[
w(t_1)\ne w(t_2).
\]

The correct rule is:

\[
\boxed{
\text{J-0/J-1 equality extends only to valuation directions
invariant under the declared transport,}
}
\]

or to role-indexed valuations transported with the roles.

For a fixed physical valuation, require

\[
\boxed{
w\circ\Theta=w.}
\]

Thus:

- count-free survives;
- ordinary Straight count survives under the established guard for J-0/J-1;
- arbitrary anisotropy requires a stabilizer or role transport;
- “every cone at once” is false in fixed physical coordinates.

## 2.7 Narrow “dominance never travels”

A policy’s old dominance verdict does not travel merely because the policy itself can be transported. That safety rule is correct.

But dominance can travel under a proved value-order isomorphism. If transports give bijections of worlds and policies and preserve values,

\[
\alpha_{T\rho}(T\xi)=\alpha_\rho(\xi),
\]

then

\[
\alpha_\rho\le\alpha_\sigma
\iff
\alpha_{T\rho}\le\alpha_{T\sigma}.
\]

The durable formulation is:

> **Dominance does not travel with a policy alone.** It travels only under an explicit theorem preserving the comparison policy class, latent domain, field, outcome contract, belief orientation, and valuation orientation.

## 2.8 Narrow the pruning wording

The errata says no pruning rule preserves the distinct-vector count. The exact implementation rule should be:

> Pareto and convex pruning do not preserve \(N_{\mathrm{vec}}\) in general. A pruned run may not report \(N_{\mathrm{vec}}\) unless it maintains a separate complete unpruned accounting.

## 2.9 State the fixed-policy evaluator invariant semantically

“No max node below the root” is a good implementation guard but slightly too syntactic. The semantic invariant is:

\[
\boxed{
\text{Every later focal action is supplied by the candidate policy;
no optimizer may select a focal action using hidden-state information.}
}
\]

Nonfocal expectation, chance summation, deterministic singleton choices, and harmless implementation maxima over singleton sets do not violate the fixed-policy requirement.

---

# 3. The unified exact object: an action-separation calculus

The project no longer needs one universal compressed state. It needs a typed proof object for the root action.

For every root action \(a\), maintain four components.

## 3.1 Primal policy library

\[
\mathcal C_a\subseteq\mathcal R_H(B,a).
\]

Every entry is a complete lawful information-consistent policy. It may be obtained from:

- a prior exact solve;
- a transported policy under a declared policy transport;
- a deadness or order-exchange reduction;
- a tense-root analysis;
- a relaxed optimizer repaired into lawful form;
- counterexample-guided policy discovery.

A transported policy remains a lawful candidate only after its policy transport is proved. Its prior dominance, value, and optimality verdicts do not come with it unless separately transported by theorem.

## 3.2 Purpose-specific exact evaluator

For a fixed lawful policy \(\rho\), compute

\[
\boxed{
\mathsf{Eval}_B(\rho)
=
\langle\beta,\alpha_\rho\rangle.
}
\]

The evaluator may use hidden-world Scheme/Fix formulas internally. It may inspect hidden structure to **evaluate** one lawful policy. It may not use hidden cells to **select** among policies.

The evaluator’s representation may be:

- a shared arithmetic/Boolean DAG;
- a weighted model-counting circuit;
- a BDD/ZDD-like exact union representation;
- a dynamic program over support-normal-form constraints;
- another exact purpose-specific proof DAG.

Its correctness obligation is equality with exact enumeration on declared small carriers.

## 3.3 Action-conditioned upper relaxation

Let

\[
\mathcal R_H(B,a)
\subseteq
\mathcal R_a^+(\mathcal G_a),
\]

where \(\mathcal G_a\) is the current set of proved information-gluing equalities.

Define

\[
\boxed{
U_a(\mathcal G_a)
=
\max_{\rho\in\mathcal R_a^+(\mathcal G_a)}
\mathsf{Eval}_B(\rho).
}
\]

The initial relaxation may be action-conditioned treatment \(C\). Every gluing cut must:

1. constrain only the relaxation;
2. be satisfied by every lawful \(H\)-policy;
3. preserve the field, mechanics, utility, belief, and world domain;
4. enforce an equality of transported actions rather than branch on a hidden event.

## 3.4 Lower witness

\[
\boxed{
L_a
=
\max_{\rho\in\mathcal C_a}
\mathsf{Eval}_B(\rho).
}
\]

Because every library entry is lawful and evaluated as a fixed policy,

\[
L_a\le Q^H_B(a).
\]

## 3.5 Root-action separation theorem

Assume for every action \(a\):

- every candidate in \(\mathcal C_a\) is lawful;
- every candidate is evaluated exactly as a fixed policy;
- \(\mathcal R_a^+(\mathcal G_a)\) contains every lawful \(H\)-policy with root action \(a\);
- the relaxed solve returns an exact maximum or a proved upper bound;
- \(L_a\) and \(U_a\) use the same field, belief, world set, utility, and valuation contract;
- no sampling or decimation occurs inside either bound.

Then

\[
\boxed{
L_a\le Q^H_B(a)\le U_a.
}
\]

If some \(a^\star\) satisfies

\[
L_{a^\star}\ge U_a
\qquad\forall a\ne a^\star,
\]

then

\[
\boxed{
a^\star\in\operatorname{Opt}^H(B).}
\]

If every inequality is strict, \(a^\star\) is uniquely optimal.

This is the exact computational target.

---

# 4. Decision proof complexity

The relevant opening complexity is no longer one number such as state count or predictive rank.

For one action \(a\), define a proof-complexity profile

\[
\boxed{
\mathfrak C_a(B)
=
\bigl(
N_a^{\mathrm{primal}},
S_a^{\mathrm{eval}},
N_a^{\mathrm{cut}},
S_a^{\mathrm{upper}}
\bigr),
}
\]

where:

- \(N_a^{\mathrm{primal}}\) is the number of lawful candidate policies actually needed;
- \(S_a^{\mathrm{eval}}\) is the exact circuit or proof-DAG size used to evaluate them;
- \(N_a^{\mathrm{cut}}\) is the number of active gluing equalities needed to tighten the relaxation;
- \(S_a^{\mathrm{upper}}\) is the size or cost of the final relaxed upper solve.

The opening action can be tractable even when:

- the structural quotient is the identity;
- the predictive value closure is full-dimensional;
- complete records separate every physical world.

What matters is whether the proof-complexity profile remains moderate.

The S6b singleton frontiers are direct evidence that \(N_a^{\mathrm{primal}}\) and the active decision geometry can be tiny even when the underlying value functions span the whole world space.

---

# 5. Implementation obligations

Any implementation consuming this direction should enforce the following as types, structural checks, or mandatory receipts.

## 5.1 Every lower witness is a fixed lawful policy

- One action per focal information state.
- No hidden-world focal optimization below the root.
- Structural assertion of the evaluator mode.
- Wherever treatment \(H\) completes, assert

  \[
  L_a\le Q^H_B(a)
  \]

  exactly.

## 5.2 Every upper witness is action-conditioned

- Root action held fixed.
- Same field, belief, world set, utility, and valuation as the lower side.
- No decimation or sampled mean inside the bound.
- Perfect-information minimax is not substituted for a fixed-field information relaxation.
- Wherever treatment \(H\) completes, assert

  \[
  Q^H_B(a)\le U_a
  \]

  exactly.

## 5.3 Every gluing cut has a validity proof

A cut constrains the relaxed policy class, never the world fiber. It must prove that all lawful \(H\)-policies satisfy the equality.

One unlawful optimizer proves only that the optimizer violates a lawful equality. It does not prove that the whole relaxed optimal face lacks a lawful policy.

## 5.4 Every policy transport is declared

A policy is indexed by information states. Moving it to another coordinate requires:

- a canonical key;
- a transport of observations and information states;
- a transport of legal actions;
- preservation of policy lawfulness.

Value and dominance verdicts require additional value-preservation hypotheses.

## 5.5 Every valuation transport is declared

Count-free game transport does not preserve arbitrary physical tile valuation.

For a fixed physical valuation, require the stabilizer condition

\[
w\circ\Theta=w.
\]

Otherwise transport roles and reapply coefficients in role coordinates.

## 5.6 Every circuit claim is purpose-relative

The circuit header names:

- the exact policy or finite policy family evaluated;
- the observation likelihoods represented;
- the moments or terminal readouts computed;
- whether normalized filtering is performed;
- the field and belief;
- the declared carrier;
- the comparison authority.

A small circuit for one fixed-policy value is not a small universal predictive representation, and should not be described as one.

---

# 6. Recommended next experimental sequence

## 6.1 Complete S6c

Finish the declared deadness run before expanding the detector family.

Measure separately:

- forced nodes;
- decision-dead nodes;
- universally dominant but non-dead nodes;
- detector recall under each valuation tag;
- the detector’s net harvest dividend after its own cost.

Do not infer arbitrary-valuation survival from the no-count guard.

## 6.2 Build the action-conditioned treatment-C evaluator

This is the most immediate missing component.

For each root action \(a\), compute

\[
U_a=\mathbb E_\beta[V_a^\ast].
\]

Cross-check on every small coordinate where treatment \(H\) completes:

\[
Q^H_B(a)\le U_a.
\]

Also retain

\[
U_a\le U^{\mathrm{agg}}
\]

as a consistency receipt.

## 6.3 Test root-action separation on completed grade-three coordinates

Use the exact dominant or H-optimal policies already found as primal candidates. Evaluate them with the fixed-policy path to obtain \(L_a\). Pair them with the new action-conditioned \(U_a\).

Record:

- how many actions are separated immediately;
- which gaps remain open;
- whether zero global fusion-gap coordinates close at the optimal action as predicted;
- how many lawful candidate policies were needed.

## 6.4 Perform tense-root anatomy

On the two capped non-boss trump leads, identify the first interface at which a **completed** frontier has width greater than one. Do not infer the first split from the location where a capped run stopped.

For the first incomparable policies \(\rho,\sigma\), analyze

\[
\Delta_{\rho,\sigma}(\xi)
=
\alpha_\rho(\xi)-\alpha_\sigma(\xi),
\]

and the exact regions

\[
X^+=\{\Delta>0\},
\qquad
X^0=\{\Delta=0\},
\qquad
X^-=\{\Delta<0\}.
\]

Use these regions as Scheme/Fix synthesis targets. The purpose is to discover the causal relation responsible for the policy tension, not to build another broad untargeted atom registry.

## 6.5 Build one fixed-policy Scheme circuit

Choose one declared small-grade lawful policy and one declared value contract. Compile its exact world-value function into a shared Scheme arithmetic/Boolean DAG and integrate it under the exact fiber belief.

Compare bit-for-bit with enumeration.

Report:

- world count;
- circuit nodes;
- arithmetic operations;
- exact weighted-counting calls;
- repeated subexpression reuse;
- construction time and evaluation time.

Do not impose universal normalized atom closure. This is the first real Gate-D experiment: exact finite inner-product evaluation.

## 6.6 Then test adaptive gluing

Start from action-conditioned treatment \(C\) on one tense grade-three action. Add only proved information equalities violated by a returned relaxed optimizer.

Measure:

- number of cuts;
- number of generalized Scheme cuts;
- upper-bound decrease after each cut;
- whether the relaxed optimal face contains a lawful policy;
- whether root-action separation occurs before exact \(H\) equality.

## 6.7 Delay the reachable-width experiment until retyped

Define interface-local reachable width first. Then measure it only on carriers where all reachable posteriors and continuation vectors can be enumerated exactly.

## 6.8 Keep count re-entry as its own adjudicated design

Policies survive as candidate witnesses. Their count-free values, dominance, deadness, and form-keyed records do not automatically survive.

A count/score lift must declare:

- the valuation family or cone;
- the role-capture feature construction;
- the stabilizer or role transport;
- which verdicts are re-proved;
- which count-free stores are invalidated.

---

# 7. What should now be considered closed

Further work is unlikely to benefit from:

- larger universal static opening-hand quotients;
- higher-grade universal predictive-rank censuses under the complete-record contract;
- explicit universal Scheme atom algebras for normalized filtering;
- class-table construction treated as a presumed first-build accelerator;
- broad descriptor expansion without a concrete advantage, conflict, likelihood, or policy-evaluation target.

Those routes answered their mathematical questions.

The negative findings delimit the problem:

\[
\boxed{
\begin{array}{l}
\text{No small universal opening state quotient has appeared;}\\
\text{no small universal linear continuation realization has appeared;}\\
\text{universal normalized atom filtering becomes discrete;}\\[1mm]
\text{but a small policy envelope, compact fixed-policy circuit,}\\
\text{small active gluing set, and short action-separation proof remain open.}
\end{array}
}
\]

---

# 8. Final unified statement

The initial hidden-deal fiber may genuinely contain

\[
399{,}072{,}960
\]

worlds. The opening hand may genuinely occupy its own structural class. The selected future value functions may genuinely span almost or all of the world-coordinate space.

None of those facts proves that the optimal opening action requires one independent game-tree solve per world.

The exact solver may instead construct a proof object of the form

\[
\boxed{
\begin{array}{c}
\text{a lawful policy library}\
+\ \text{compact exact evaluators for those policies}\
+\ \text{action-conditioned relaxed upper bounds}\
+\ \text{a small family of valid information equalities}\
\Longrightarrow\
L_{a^\star}\ge U_a\quad\forall a\ne a^\star.
\end{array}
}
\]

The decisive shift is:

\[
\boxed{
\text{do not compress all latent truth; construct an exact proof of the maximum.}
}
\]

This is not a retreat from the earlier mathematics. It is what the earlier mathematics has isolated as the remaining live target.

The branch now supports a serious and disciplined conjecture:

> **Straight Texas 42 may be information-rich and outcome-rich while remaining decision-sparse, with most exact complexity concentrated in a comparatively small family of tense policy conflicts.**

The next experiments can test that conjecture without weakening the information model, confusing world-informed values with lawful policy values, or requiring the opening hand to belong to a small structural class.
