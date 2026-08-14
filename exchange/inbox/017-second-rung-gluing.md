<!-- HARVEST METADATA
dispatch: exchange/outbox/017-second-rung-gluing-handoff.md
channel: ChatGPT 5.6 Pro (app/web), informal conversational register (x:014 style)
transport: hand-ferried by Jason (courier automation watches the main checkout,
  not this worktree; Jason pasted the GitHub body of outbox 017 and uploaded
  Pro's reply file on 2026-08-14)
harvested: 2026-08-14
status: UNADJUDICATED -- no claim below may touch the wiki until walt-math
  adjudicates (exchange protocol: witnesses re-run, proofs step-checked).
  The note self-classifies claims (exact result / certificate schema /
  experimental receipt -- reported / open); those labels are Pro's, not ours,
  until confirmed.
body: verbatim below this comment, byte-for-byte from the uploaded file.
-->

---
title: "Second-Rung Gluing: Policy-Dependent Occupancies, the Slack–Tax Interchange Law, and Exact Martingale Penalties for Straight Texas 42"
version: "0.1"
status: "Standalone research note; exact finite-tree theorems, certificate schemas, reported experiment audit, and open implementation obligations"
date: "2026-08-14"
repository: "jasonyandell/texas-42"
branch: "worktree-walt-s2"
source_handoff: "exchange/outbox/017-second-rung-gluing-handoff.md"
extends: "decision_sparse_nonanticipativity_taxes_and_plan_calculus_v0.1.md"
---

# Second-Rung Gluing

## Policy-Dependent Occupancies, the Slack–Tax Interchange Law, and Exact Martingale Penalties for Straight Texas 42

**Version 0.1 — 2026-08-14**

## Abstract

The first nonanticipativity rung has now survived adjudication and exact experiment. It closed one previously unclosable root comparison, and the remaining failures identify the next obstruction exactly: clairvoyant value at the focal player's second future decision.

At depth two, the clean first-rung sum cannot simply be repeated. The second-frontier occupancy law depends on the common action selected at the first frontier. The correct primitive is therefore an **action-conditioned occupancy measure** indexed by

\[
(I_1,b_1,I_2).
\]

After fixing the first information state \(I_1\) and first common action \(b_1\), the second-frontier states \(I_2\) are mutually exclusive and their taxes add. Across alternative first actions, however, taxes do not add. The controller may change its first action to avoid a large downstream fusion tax, paying some first-rung value to do so.

The exact second-rung law is

\[
\boxed{
U^{(1)}-U^{(2)}
=
\sum_{I_1}
\min_{b_1}
\left
\{
 s_{I_1,b_1}
 +
 \sum_{I_2}
 \delta_{I_1,b_1,I_2}
\right\}.
}
\]

Here \(s_{I_1,b_1}\) is the loss from choosing \(b_1\) instead of a first-rung-optimal action, and \(\delta_{I_1,b_1,I_2}\) is the exact second-decision Jensen/regret tax under the occupancy induced by \(b_1\). This **slack–tax interchange law** is the missing policy-adjustment term. It says that every first action must be covered in one of two ways: it is already inferior at rung one, or its descendants necessarily pay a second-layer nonanticipativity tax.

The same finite-tree model admits an exact multistage dual. Any penalty increment whose conditional mean is zero for every public information state and each fixed lawful action gives a valid upper witness. Backward-centered continuation-value penalties recover \(U^{(k)}\) exactly for every rung \(k\). Thus hard gluing and martingale-difference information penalties remain primal and dual descriptions of the same object beyond the first frontier.

The note also gives the safe depth-two regret-event calculus, a receipt schema for exact replay, a correction ledger for the preceding v0.1 note, and an audit of the five exact Experiment 15.1 second-rung values reported in handoff 017.

---

## 1. Provenance, scope, and corrections to v0.1

This note extends:

- `walt/math/decision_sparse_exact_solving_v0.1.md`;
- `walt/math/decision_sparse_exact_solving_v0.1_errata.md`;
- `walt/math/decision_sparse_second_audit_v0.1.md`;
- `exchange/outbox/016-cheap-upper-witness-handoff.md`; and
- `decision_sparse_nonanticipativity_taxes_and_plan_calculus_v0.1.md`.

The project rules and mathematical-foundation documents govern on conflict. Root-action comparison remains action-conditioned throughout.

This note adopts four repairs reported in handoff 017.

### 1.1 The first-frontier arrival lemma is an explicit hypothesis

The first-rung formula uses a policy-independent arrival law only because its frontier is the focal player's **next** decision after the fixed root action. No focal choice occurs before that frontier. At depth two, the arrival law is action-conditioned and must be indexed by the first common action.

### 1.2 The opening-lead ladder has five effective gluing rungs

After an opening lead, the focal player has six future plays. The final play is from a one-tile hand and is forced. Revealing the world before that forced play has no decision value. Therefore the effective ladder has at most five nontrivial future focal-decision rungs:

\[
U^{(0)}\ge U^{(1)}\ge\cdots\ge U^{(5)}=Q^H.
\]

At a grade-4 root, after the root play there are three future focal plays, the last forced, so

\[
\boxed{U^{(2)}=Q^H.}
\]

This supersedes the six-rung and three-rung counts in the preceding v0.1 note.

### 1.3 An action-independent pointwise upper feature cannot improve treatment \(C\)

Suppose \(B_I(\omega)\) is independent of the action and satisfies

\[
B_I(\omega)\ge q_I(\omega,b)
\qquad
\text{for every }b.
\]

Then pointwise

\[
B_I(\omega)\ge \max_b q_I(\omega,b),
\]

and therefore

\[
\max_b\sum_\omega \mu_I(\omega)B_I(\omega)
=
\sum_\omega\mu_I(\omega)B_I(\omega)
\ge
\sum_\omega\mu_I(\omega)\max_bq_I(\omega,b).
\]

Thus the resulting glued upper is no smaller than the treatment-\(C\) branch it was meant to improve.

\[
\boxed{
\text{A pointwise continuation upper must be action-conditioned to shave }C.
}
\]

### 1.4 Residual-plan composition must carry the actual posterior

Under a random-legal field, surviving worlds are generally not uniformly weighted. Their masses include the products of inverse legal-set sizes along the observed path. A residual witness is admissible in either of two forms:

1. it is evaluated under the actual posterior induced at the public stopping history; or
2. it is a pointwise guarantee valid in every compatible world, so its value is posterior-independent.

A residual coordinate evaluated under a fresh uniform belief cannot be inserted without proving that its belief agrees with the arrival posterior.

### 1.5 Claim discipline

The note distinguishes:

- **Exact result:** proved here for the stated finite model;
- **Certificate schema:** exact once its local inequalities and counts are supplied;
- **Experimental receipt — reported:** exact values reported by the branch, not independently regenerated here; and
- **Open:** a representation or theorem family not yet instantiated at trick-1 scale.

The note is payoff-neutral except where a reported experiment fixes a convention. Any Straight-count, contract, or mark extension must carry the correct payoff state.

---

## 2. Finite depth-two model

Fix:

- a focal information state \(s\);
- a focal seat \(m\);
- a fixed root action \(a\);
- a finite physical-world set \(\Omega_a\);
- an initial belief \(\beta\) on \(\Omega_a\);
- a fixed stochastic field policy \(\sigma_{-m}\); and
- terminal utility \(U\).

All probabilities below include the field's stochastic choices and any other exogenous randomness. Because the field is fixed and expected utility is linear, a deterministic focal policy is optimal; randomized focal policies add no value and may be omitted without loss.

Treatment \(C^{(0)}\) reveals the physical world immediately after the root action. Treatment \(C^{(1)}\) withholds it through the next nontrivial focal decision and reveals it immediately after that focal action. Treatment \(C^{(2)}\) withholds it through the next two nontrivial focal decisions and reveals it immediately after the second.

Write their action-conditioned values as

\[
U^{(0)},\qquad U^{(1)},\qquad U^{(2)}.
\]

### 2.1 First frontier

Let \(\mathcal I_1\) be the set of nonterminal focal information states at the first future nontrivial focal decision. Let

\[
\mu_I(\omega)
=
\Pr(\omega\text{ and first frontier }I)
\]

be the unnormalized joint mass for \(I\in\mathcal I_1\). Let

\[
p_I=\sum_\omega\mu_I(\omega).
\]

When \(p_I>0\), the posterior is

\[
\nu_I(\omega)=\frac{\mu_I(\omega)}{p_I}.
\]

This posterior need not be uniform.

Let \(\mathcal A(I)\) be the common legal-action set at information state \(I\).

### 2.2 Second frontier conditioned on the first action

Fix \(I\in\mathcal I_1\) and \(b\in\mathcal A(I)\). After choosing \(b\), let the field and chance evolve until either:

1. the continuation terminates before another nontrivial focal decision; or
2. the next nontrivial focal information state \(J\) is reached.

Let \(\mathcal I_2(I,b)\) be the set of such second-frontier states. The public state \(J\) includes the focal player's remembered information and action history. Under perfect recall, each such \(J\) has a unique relevant parent branch \((I,b)\). If an implementation uses a coarser coordinate, the parent history must be restored or the correct cross-parent nonanticipativity equalities must be imposed.

Define the action-conditioned unnormalized occupancy

\[
\boxed{
\mu_{I,b,J}(\omega)
=
\Pr(\omega, I, \text{ choose }b, \text{ then reach }J).
}
\]

Equivalently,

\[
\mu_{I,b,J}(\omega)
=
\mu_I(\omega)
K_{I,b}(J\mid\omega),
\]

where \(K_{I,b}\) is the field-and-chance transition kernel from \((I,b)\) to the next focal frontier.

Let

\[
\Theta_{I,b}
\]

be the unnormalized expected terminal contribution from paths that terminate after \(b\) but before a second nontrivial focal decision.

For \(c\in\mathcal A(J)\), define

\[
q_{I,b,J}(\omega,c)
\]

as the expected terminal utility obtained by reaching \(J\) in world \(\omega\), choosing \(c\), revealing \(\omega\) immediately after \(c\), and using treatment \(C\) thereafter.

Define the revealed-world best continuation at \(J\):

\[
m_{I,b,J}(\omega)
=
\max_{c\in\mathcal A(J)}q_{I,b,J}(\omega,c).
\]

Let \(T_0\) be the common expected contribution of paths terminating before the first frontier.

---

## 3. Why the first occupancy is fixed and the second is not

### Lemma 3.1 — Policy-independent first-frontier arrival

After the root action is fixed, \(\mu_I(\omega)\) is independent of the focal continuation policy.

#### Proof

By definition, \(I\) is the focal player's next decision frontier. Between the fixed root action and arrival at \(I\), only the fixed field and exogenous chance act. No focal continuation action exists on that interval. Therefore the probability of every world/frontier event \((\omega,I)\) is determined entirely by \(\beta\), the rules, the fixed root action, the field policy, and chance. \(\square\)

Under a uniform-random-legal field, a realized path \(h\) in world \(\omega\) receives a factor of the form

\[
\prod_{t\in h}\frac{1}{|\mathcal A_t(\omega,h_t)|}.
\]

Consequently, conditioning on the public path does not generally produce a uniform distribution over worlds.

### Lemma 3.2 — Action-conditioned second-frontier arrival

For fixed \((I,b)\), the occupancies \(\mu_{I,b,J}(\omega)\) are well-defined and the events

\[
\{\text{terminal before frontier 2}\}
\quad\text{and}\quad
\{J:J\in\mathcal I_2(I,b)\}
\]

are mutually exclusive and exhaustive. In general, the occupancy family changes with \(b\).

#### Proof

Once \((I,b)\) is fixed, no focal action occurs before the next focal frontier. The transition law is therefore determined by the field and chance, giving the kernel \(K_{I,b}\). Distinct next public information states are mutually exclusive along one realized trajectory, and either one is reached or the continuation terminates first. Different values of \(b\) induce different successor hands, led contexts, and field legal sets, so no action-invariance is available in general. \(\square\)

The conditioning order is load-bearing:

\[
\boxed{
\text{fix }I,
\quad
\text{fix }b,
\quad
\text{then add over mutually exclusive }J.
}
\]

Alternative first actions are counterfactual branches, not disjoint events under one policy.

---

## 4. Exact depth-two values

For every first branch \((I,b)\), define

\[
\boxed{
F^{(1)}_{I,b}
=
\Theta_{I,b}
+
\sum_{J\in\mathcal I_2(I,b)}
\sum_\omega
\mu_{I,b,J}(\omega)
 m_{I,b,J}(\omega).
}
\]

This is the branch value when the first action \(b\) must be common at \(I\), but the world is revealed before the second focal choice is made.

Define

\[
\boxed{
F^{(2)}_{I,b}
=
\Theta_{I,b}
+
\sum_{J\in\mathcal I_2(I,b)}
\max_{c\in\mathcal A(J)}
\sum_\omega
\mu_{I,b,J}(\omega)
q_{I,b,J}(\omega,c).
}
\]

This is the branch value when both the first action at \(I\) and the second action at each reached \(J\) must be lawful before revelation.

### Theorem 4.1 — Exact nested formulas for \(U^{(1)}\) and \(U^{(2)}\)

\[
\boxed{
U^{(1)}
=
T_0
+
\sum_{I\in\mathcal I_1}
\max_{b\in\mathcal A(I)}F^{(1)}_{I,b}.
}
\]

\[
\boxed{
U^{(2)}
=
T_0
+
\sum_{I\in\mathcal I_1}
\max_{b\in\mathcal A(I)}F^{(2)}_{I,b}.
}
\]

#### Proof

The first-frontier states \(I\) are mutually exclusive and have policy-independent occupancies by Lemma 3.1. A lawful first-stage policy therefore chooses one common action independently at each possible \(I\), producing the outer maxima and sum.

Fix one branch \((I,b)\). Under \(C^{(1)}\), the world is revealed immediately after \(b\). If the path reaches second state \(J\), the focal controller may select a world-specific action, giving \(m_{I,b,J}(\omega)\). Conditional mutual exclusivity of the second states under fixed \((I,b)\) gives \(F^{(1)}_{I,b}\).

Under \(C^{(2)}\), revelation is delayed through the second focal decision. At each public state \(J\), one common action \(c\) must be selected across all positive-mass worlds reaching that same \(J\). Different public states \(J\) are mutually exclusive after \((I,b)\), and perfect recall makes their policy choices independent coordinates. This gives \(F^{(2)}_{I,b}\).

Terminal contributions before either frontier are common to the relevant treatments and are carried by \(T_0\) and \(\Theta_{I,b}\). \(\square\)

### 4.1 Equivalent partial-policy form

Let \(\pi_1\) be a lawful first-frontier policy, so \(\pi_1(I)\in\mathcal A(I)\). Define

\[
V^{(1)}(\pi_1)
=
T_0+
\sum_I F^{(1)}_{I,\pi_1(I)}
\]

and

\[
D^{(2)}(\pi_1)
=
\sum_I
\left(
F^{(1)}_{I,\pi_1(I)}-F^{(2)}_{I,\pi_1(I)}
\right).
\]

Then

\[
U^{(1)}=\max_{\pi_1}V^{(1)}(\pi_1),
\]

and

\[
\boxed{
U^{(2)}
=
\max_{\pi_1}
\left[
V^{(1)}(\pi_1)-D^{(2)}(\pi_1)
\right].
}
\]

This is the correct joint object. The occupancy appearing in \(D^{(2)}(\pi_1)\) is induced by the selected first-stage policy.

---

## 5. Exact conditional second-stage tax

For fixed \((I,b,J)\), define

\[
\boxed{
\delta_{I,b,J}
=
\sum_\omega
\mu_{I,b,J}(\omega)
 m_{I,b,J}(\omega)
-
\max_c
\sum_\omega
\mu_{I,b,J}(\omega)
 q_{I,b,J}(\omega,c).
}
\]

### Proposition 5.1 — Regret form

\[
\boxed{
\delta_{I,b,J}
=
\min_c
\sum_\omega
\mu_{I,b,J}(\omega)
\left[
 m_{I,b,J}(\omega)-q_{I,b,J}(\omega,c)
\right].
}
\]

#### Proof

The first sum is independent of \(c\). Subtracting the maximum common-action value is therefore equivalent to minimizing the expected regret. \(\square\)

Every \(\delta_{I,b,J}\) is nonnegative.

### Corollary 5.2 — Complete-optimal-face criterion at depth two

\[
\delta_{I,b,J}=0
\]

if and only if

\[
\bigcap_{\mu_{I,b,J}(\omega)>0}
\arg\max_c q_{I,b,J}(\omega,c)
\ne\varnothing.
\]

The complete optimal action sets are required. A tie-broken optimizer is not a valid substitute.

### Proposition 5.3 — Conditional additivity

Define the downstream tax under first action \(b\) by

\[
\boxed{
d_{I,b}
=
F^{(1)}_{I,b}-F^{(2)}_{I,b}.
}
\]

Then

\[
\boxed{
d_{I,b}
=
\sum_{J\in\mathcal I_2(I,b)}\delta_{I,b,J}.
}
\]

#### Proof

Subtract the displayed definitions of \(F^{(1)}_{I,b}\) and \(F^{(2)}_{I,b}\). The terminal contribution \(\Theta_{I,b}\) cancels. The remaining difference separates over the mutually exclusive second-frontier states \(J\) under the already-fixed branch \((I,b)\). Each summand is exactly \(\delta_{I,b,J}\). \(\square\)

This is the full extent of free additivity at rung two. The quantities \(d_{I,b}\) for alternative first actions are not additive.

---

## 6. The slack–tax interchange law

For a first information state \(I\), define its rung-one branch maximum

\[
M_I
=
\max_{b\in\mathcal A(I)}F^{(1)}_{I,b}.
\]

Define the first-action slack

\[
\boxed{
s_{I,b}
=
M_I-F^{(1)}_{I,b}
\ge0.
}
\]

The slack is the rung-one value sacrificed by committing to first action \(b\).

### Lemma 6.1 — Finite interchange identity

Let \(x_b\) and \(y_b\) be finite families with \(y_b\le x_b\). Put

\[
M=\max_bx_b,
\qquad
s_b=M-x_b,
\qquad
d_b=x_b-y_b.
\]

Then

\[
\boxed{
M-\max_by_b
=
\min_b(s_b+d_b).
}
\]

#### Proof

For every \(b\),

\[
y_b=x_b-d_b=M-s_b-d_b.
\]

Therefore

\[
\max_by_b
=M-\min_b(s_b+d_b),
\]

which rearranges to the result. \(\square\)

### Theorem 6.2 — Exact second-rung gluing law

\[
\boxed{
\Delta^{(2)}
:=
U^{(1)}-U^{(2)}
=
\sum_{I\in\mathcal I_1}
\min_{b\in\mathcal A(I)}
\left[
 s_{I,b}
 +
 d_{I,b}
\right].
}
\]

Equivalently,

\[
\boxed{
\Delta^{(2)}
=
\sum_I
\min_b
\left[
 s_{I,b}
 +
 \sum_{J\in\mathcal I_2(I,b)}
 \delta_{I,b,J}
\right].
}
\]

#### Proof

By Theorem 4.1,

\[
\Delta^{(2)}
=
\sum_I
\left[
\max_bF^{(1)}_{I,b}
-
\max_bF^{(2)}_{I,b}
\right].
\]

For each fixed \(I\), apply Lemma 6.1 with

\[
x_b=F^{(1)}_{I,b},
\qquad
y_b=F^{(2)}_{I,b},
\qquad
d_b=d_{I,b}.
\]

Then use Proposition 5.3. \(\square\)

### 6.1 Policy-level form

The same identity may be written

\[
\boxed{
U^{(1)}-U^{(2)}
=
\min_{\pi_1}
\left[
U^{(1)}-V^{(1)}(\pi_1)
+D^{(2)}(\pi_1)
\right].
}
\]

This says:

> To survive the second gluing rung, the relaxed controller may change its first-stage policy. It pays the rung-one slack of that change and then pays the remaining second-stage tax under the occupancy induced by the changed policy. It chooses the cheapest combined escape route.

Because first-frontier states are mutually exclusive and policy-independent, the policy-level minimum separates into the local formula of Theorem 6.2.

### 6.2 Exact zero criterion

Since \(s_{I,b}\ge0\) and \(d_{I,b}\ge0\), the local second-rung tax at \(I\) is zero exactly when there exists a first action \(b\) such that

\[
s_{I,b}=0
\qquad\text{and}\qquad
d_{I,b}=0.
\]

Equivalently:

\[
\boxed{
\Delta_I^{(2)}=0
\iff
\exists b\in\arg\max_cF^{(1)}_{I,c}
\text{ whose every reached second frontier has a common }C\text{-optimal action.}
}
\]

### 6.3 Why taxing only the rung-one optimizer is unsafe

Let

\[
\mathcal B_I^*=\arg\max_bF^{(1)}_{I,b}.
\]

Then

\[
\Delta_I^{(2)}
=
\min_b(s_{I,b}+d_{I,b})
\le
\min_{b\in\mathcal B_I^*}d_{I,b}.
\]

The right side is generally an **upper** bound on the true tax, not a lower bound. A first action outside the rung-one optimal face may sacrifice a small amount of rung-one value but avoid much more downstream fusion. Such an action is an **escape action**.

Therefore a valid lower certificate for \(\Delta_I^{(2)}\) must cover every first action, not only one optimizer and not only the complete optimal face.

### 6.4 When the first-rung action remains optimal

A first action \(b^*\in\mathcal B_I^*\) remains optimal after the second gluing rung if and only if

\[
d_{I,b^*}
\le
s_{I,b}+d_{I,b}
\qquad
\text{for every }b.
\]

If this fails, policy adjustment is not a nuisance in the proof; it is part of the exact optimal response to the added nonanticipativity constraint.

---

## 7. A recursive law for deeper rungs

The depth-two formula is the first nontrivial instance of a general Bellman identity.

At any public focal information state \(G\), suppose the previous relaxation gives action-conditioned branch values

\[
F^{(r-1)}_{G,a}
\]

and gluing one additional future focal decision changes them to

\[
F^{(r)}_{G,a}
\le
F^{(r-1)}_{G,a}.
\]

Define

\[
d^{(r)}_{G,a}
=
F^{(r-1)}_{G,a}-F^{(r)}_{G,a},
\]

\[
M^{(r-1)}_G
=
\max_aF^{(r-1)}_{G,a},
\]

and

\[
s^{(r)}_{G,a}
=
M^{(r-1)}_G-F^{(r-1)}_{G,a}.
\]

Then

\[
\boxed{
M^{(r-1)}_G-
\max_aF^{(r)}_{G,a}
=
\min_a
\left[
 s^{(r)}_{G,a}+d^{(r)}_{G,a}
\right].
}
\]

The identity is always local. At depth one, the root-to-frontier occupancy is fixed, so local taxes flatten into one simple global sum. At deeper depths, occupancies depend on earlier actions, so the correct global object is a nested Bellman recursion. It must not be flattened with policy-independent weights unless an additional invariance theorem proves that flattening valid.

This gives a finite recursive calculus for all five effective opening-lead rungs.

---

## 8. Incremental penalty dual for the second rung

The primal formula glues the second decision directly. The dual formulation prices the illegal use of hidden information at that decision.

For every \((I,b,J)\) and legal second action \(c\), choose a penalty

\[
\lambda_{I,b,J}(\omega,c)
\]

satisfying the action-conditioned centering equations

\[
\boxed{
\sum_\omega
\mu_{I,b,J}(\omega)
\lambda_{I,b,J}(\omega,c)
=0
\qquad
\text{for every }c.
}
\]

### Theorem 8.1 — Valid incremental second-rung penalty

Define

\[
\begin{aligned}
\overline U^{(2)}(\lambda)
:=
T_0
+
\sum_I
\max_b
\Bigg[
&\Theta_{I,b}
\\
&+
\sum_{J\in\mathcal I_2(I,b)}
\sum_\omega
\mu_{I,b,J}(\omega)
\max_c
\left(
q_{I,b,J}(\omega,c)
-
\lambda_{I,b,J}(\omega,c)
\right)
\Bigg].
\end{aligned}
\]

Then

\[
\boxed{
U^{(2)}
\le
\overline U^{(2)}(\lambda).
}
\]

#### Proof

Fix \((I,b,J)\) and any common lawful second action \(c\). Centering gives

\[
\sum_\omega\mu_{I,b,J}(\omega)q_{I,b,J}(\omega,c)
=
\sum_\omega\mu_{I,b,J}(\omega)
\left[q_{I,b,J}(\omega,c)-\lambda_{I,b,J}(\omega,c)\right].
\]

For each \(\omega\), the selected action's penalized value is no greater than the maximum penalized value over actions. Therefore the penalized revealed-world expression upper-bounds the best common action at \(J\). Sum over conditionally mutually exclusive \(J\), add \(\Theta_{I,b}\), maximize over the common first action \(b\), and finally sum over the mutually exclusive first-frontier states \(I\). \(\square\)

The centering law is indexed by \(b\). A penalty centered under one first action's occupancy is not automatically valid under another first action.

### Proposition 8.2 — Exact recovery of \(U^{(2)}\)

For a positive-mass state \((I,b,J)\), define

\[
\bar q_{I,b,J}(c)
=
\frac{
\sum_\omega\mu_{I,b,J}(\omega)q_{I,b,J}(\omega,c)
}{
\sum_\omega\mu_{I,b,J}(\omega)
}.
\]

Choose

\[
\boxed{
\lambda^*_{I,b,J}(\omega,c)
=
q_{I,b,J}(\omega,c)-\bar q_{I,b,J}(c).
}
\]

For a zero-mass state, set the penalty arbitrarily, for example to zero. Then

\[
\boxed{
\overline U^{(2)}(\lambda^*)=U^{(2)}.
}
\]

#### Proof

The penalty is centered by construction, and

\[
q_{I,b,J}(\omega,c)-\lambda^*_{I,b,J}(\omega,c)
=
\bar q_{I,b,J}(c),
\]

which is independent of \(\omega\). Hence

\[
\sum_\omega\mu_{I,b,J}(\omega)
\max_c
\left[q_{I,b,J}(\omega,c)-\lambda^*_{I,b,J}(\omega,c)\right]
=
\max_c
\sum_\omega\mu_{I,b,J}(\omega)q_{I,b,J}(\omega,c).
\]

Substitution into Theorem 8.1 gives exactly the formula for \(U^{(2)}\) in Theorem 4.1. \(\square\)

Thus the second-rung hard glue and the action-conditioned centered penalty are exact primal and dual forms of the same operation.

---

## 9. Full multistage martingale-difference dual

The preceding result starts from \(U^{(1)}\) and prices the second illegal revelation. This section starts from the fully revealed treatment and handles any finite number of glued focal decisions.

### 9.1 Filtrations and policy classes

Let the nontrivial future focal decision epochs be \(t=1,\ldots,k\).

- \(\mathcal G_t\) is the lawful public information available to the focal player at decision \(t\): own hand, public record, remembered prior focal actions, and any other project-defined observables.
- \(\mathcal F_t\supseteq\mathcal G_t\) is the relaxed information that additionally contains the physical world revealed by treatment \(C\).
- \(G_t\) denotes the realized public information state.
- \(X_t\) denotes a full latent state sufficient to evaluate continuation probabilities.

A policy in \(C^{(k)}\) must choose its first \(k\) focal actions using only \(\mathcal G_t\); after the \(k\)-th such action the world may be revealed. A fully relaxed policy may use \(\mathcal F_t\) at every one of those decisions.

Because the public history includes prior focal actions, the lawful posterior

\[
\nu_t(\cdot\mid G_t=g)
\]

already incorporates their policy-dependent effect on occupancy. For deterministic lawful policies, the actions in a reached history are fixed. For behavioral randomized lawful policies, each action probability depends only on the public history and therefore cancels from Bayes' rule inside that information state. Thus the posterior is well-defined for every reachable public history, independent of which lawful policy reaches it.

### Definition 9.1 — Stagewise conditionally centered penalty

For each stage \(t\), public state \(g\), and legal action \(a\), let

\[
\lambda_t(X_t,g,a)
\]

be integrable and satisfy

\[
\boxed{
\mathbb E
\left[
\lambda_t(X_t,g,a)
\mid G_t=g
\right]
=0
\qquad
\text{for every fixed }a\in\mathcal A(g).
}
\]

Equivalently, under every policy lawful through stage \(t\),

\[
\mathbb E
\left[
\lambda_t(X_t,G_t,A_t)
\mid G_t,A_t
\right]
=0.
\]

The second formulation makes the chronology explicit: first condition on the actual public history, then on the common action chosen there.

### Theorem 9.2 — Multistage weak duality

Let \(\Lambda=(\lambda_1,\ldots,\lambda_k)\) be stagewise conditionally centered. Define the penalized fully relaxed value

\[
\overline U_k(\Lambda)
=
\sup_{\rho\in C^{(0)}}
\mathbb E_\rho
\left[
U-
\sum_{t=1}^k
\lambda_t(X_t,G_t,A_t)
\right].
\]

Then

\[
\boxed{
U^{(k)}\le\overline U_k(\Lambda).
}
\]

#### Proof

Take any policy \(\rho\) lawful through the first \(k\) focal decisions. Since \(A_t\) is selected from the lawful information at stage \(t\), Definition 9.1 and the tower property give

\[
\mathbb E_\rho
\lambda_t(X_t,G_t,A_t)=0
\qquad
\text{for each }t.
\]

Therefore

\[
\mathbb E_\rho U
=
\mathbb E_\rho
\left[
U-
\sum_{t=1}^k\lambda_t
\right].
\]

The fully relaxed policy class contains \(\rho\), so its penalized supremum is at least this value. Maximizing over all policies lawful through stage \(k\) gives the result. \(\square\)

Under a lawful policy, the partial sums of the penalties have zero conditional increments. They are martingale differences relative to the lawful public decision filtration.

### Theorem 9.3 — Exact recovery by backward-centered penalties

In the finite model, there exists a stagewise conditionally centered penalty family \(\Lambda^*\) such that

\[
\boxed{
\overline U_k(\Lambda^*)=U^{(k)}.
}
\]

Consequently,

\[
\boxed{
U^{(k)}
=
\inf_{\Lambda\text{ conditionally centered}}
\overline U_k(\Lambda),
}
\]

and the infimum is attained.

#### Construction and proof

Work backward from stage \(k\).

After the \(k\)-th lawful action, revelation is permitted. Let the already-defined continuation after that action include terminal paths, field/chance evolution, and optimal revealed-world play.

Assume inductively that the perfect penalties for stages \(t+1,\ldots,k\) have been defined and that the optimally penalized relaxed continuation at every next public state \(g'\) equals the lawful value \(V_{t+1}(g')\), independent of its latent copy.

For current latent state \(x\), public state \(g\), and action \(a\), define

\[
Q_t^*(x,g,a)
=
\mathbb E
\left[
\text{payoff accumulated before the next focal decision}
+
V_{t+1}(G_{t+1})
\mid X_t=x,G_t=g,A_t=a
\right],
\]

with terminal payoff used when no next decision occurs. At \(t=k\), the continuation term is the revealed-world continuation value after the action.

Define the lawful action value

\[
\bar Q_t(g,a)
=
\mathbb E
\left[
Q_t^*(X_t,g,a)
\mid G_t=g
\right]
\]

and the lawful Bellman value

\[
V_t(g)=\max_{a\in\mathcal A(g)}\bar Q_t(g,a).
\]

Now set

\[
\boxed{
\lambda_t^*(x,g,a)
=
Q_t^*(x,g,a)-\bar Q_t(g,a).
}
\]

This increment is conditionally centered for every fixed action. Moreover,

\[
Q_t^*(x,g,a)-\lambda_t^*(x,g,a)
=
\bar Q_t(g,a),
\]

which is independent of \(x\). Even a relaxed controller that sees \(x\) therefore obtains

\[
\max_a
\left[
Q_t^*(x,g,a)-\lambda_t^*(x,g,a)
\right]
=
\max_a\bar Q_t(g,a)
=
V_t(g).
\]

This proves the induction step. At the initial frontier, the penalized relaxed value is exactly the lawful-through-\(k\) value \(U^{(k)}\). \(\square\)

### 9.2 Explicit perfect penalties for \(U^{(2)}\)

At the second state \(J\), Proposition 8.2 gives

\[
\lambda_2^*(\omega;I,b,J,c)
=
q_{I,b,J}(\omega,c)-\bar q_{I,b,J}(c).
\]

Let

\[
V_2(I,b,J)
=
\max_c\bar q_{I,b,J}(c).
\]

For a latent world \(\omega\) at first state \(I\), define

\[
Q_1^*(\omega,I,b)
=
\text{conditional expected terminal contribution before frontier 2}
+
\sum_J
K_{I,b}(J\mid\omega)V_2(I,b,J).
\]

Let

\[
\bar Q_1(I,b)
=
\mathbb E_{\nu_I}
\left[Q_1^*(\omega,I,b)\right].
\]

Then

\[
\lambda_1^*(\omega;I,b)
=
Q_1^*(\omega,I,b)-\bar Q_1(I,b).
\]

The second penalty removes world-contingent second actions. The first penalty then removes world-contingent first actions after evaluating the already-glued second-stage continuation. Together they recover \(U^{(2)}\) exactly.

### 9.3 Feature-based multistage penalties

For structural features \(\phi_{t,j}(x,g,a)\), every penalty of the form

\[
\boxed{
\lambda_t(x,g,a)
=
\sum_j\theta_{t,j}
\left(
\phi_{t,j}(x,g,a)
-
\mathbb E[\phi_{t,j}(X_t,g,a)\mid G_t=g]
\right)
}
\]

is valid.

At depth two, the center at the second frontier must use the posterior indexed by the complete public parent history, equivalently by \((I,b,J)\). Trump split, boss ownership, ruff access, entry preservation, follower supply, and control obligations can therefore be used as shadow-price features without granting those hidden facts to the player.

Exact recovery proves existence, not cheapness. The perfect penalty contains the exact continuation values and may be as difficult to compute as \(U^{(k)}\). The trick-1 program is to find a small structural feature basis whose conditionally centered penalties remove enough fusion value to decide the root action without reconstructing the perfect penalty.

---

## 10. Depth-two regret-event certificates

Define the exact second-stage regret

\[
R_{I,b,J}(\omega,c)
=
m_{I,b,J}(\omega)-q_{I,b,J}(\omega,c)
\ge0.
\]

### Theorem 10.1 — Regret-minorant certificate under policy-dependent occupancy

Suppose that for every \((I,b,J,c)\) a function

\[
g_{I,b,J}(\omega,c)
\]

satisfies

\[
0\le g_{I,b,J}(\omega,c)
\le R_{I,b,J}(\omega,c)
\qquad
\text{pointwise}.
\]

Define

\[
\underline\delta_{I,b,J}
=
\min_c
\sum_\omega
\mu_{I,b,J}(\omega)
 g_{I,b,J}(\omega,c)
\]

and

\[
\underline d_{I,b}
=
\sum_{J\in\mathcal I_2(I,b)}
\underline\delta_{I,b,J}.
\]

Then

\[
\underline\delta_{I,b,J}\le\delta_{I,b,J},
\qquad
\underline d_{I,b}\le d_{I,b}.
\]

#### Proof

For each fixed second action \(c\), pointwise domination implies the corresponding weighted inequality. Taking the minimum over \(c\) preserves it. Conditional additivity over \(J\) then gives the branch inequality. \(\square\)

### 10.1 Event form

A simple certificate has

\[
g_{I,b,J}(\omega,c)
=
\eta_{I,b,J,c}
\mathbf1_{E_{I,b,J,c}}(\omega),
\]

where the game algebra proves:

> On event \(E_{I,b,J,c}\), choosing common second action \(c\) loses at least \(\eta_{I,b,J,c}\) relative to the best revealed-world continuation.

Then

\[
\underline\delta_{I,b,J}
=
\min_c
\eta_{I,b,J,c}
\mu_{I,b,J}(E_{I,b,J,c}).
\]

More elaborate pointwise minorants may sum several event terms, but their combined value must remain pointwise below the exact regret.

### 10.2 The right primitive is action-conditioned

An event stated without \(b\) is reusable only if its regret inequality holds for every first action under consideration. Even then, its mass must be recomputed under each action-conditioned occupancy:

\[
\mu_{I,b,J}(E).
\]

There is generally no policy-independent depth-two event probability.

Thus the primitive certificate is indexed by

\[
\boxed{(I,b,J,c).}
\]

A \(b\)-uniform event is a theorem about a family of such primitives, not a replacement for the action conditioning.

---

## 11. Covering policy adjustment: slack plus event tax

Theorem 6.2 shows that a downstream regret certificate alone is insufficient. The first action may change.

### Theorem 11.1 — Two-stage action-cover certificate

Suppose for every \((I,b)\) there is a proved slack lower bound

\[
0\le\underline s_{I,b}\le s_{I,b}
\]

and a proved downstream-tax lower bound

\[
0\le\underline d_{I,b}\le d_{I,b}.
\]

Then

\[
\boxed{
\underline\Delta^{(2)}
:=
\sum_I
\min_b
\left[
\underline s_{I,b}+
\underline d_{I,b}
\right]
\le
U^{(1)}-U^{(2)}.
}
\]

#### Proof

For every \((I,b)\),

\[
\underline s_{I,b}+
\underline d_{I,b}
\le
s_{I,b}+d_{I,b}.
\]

Taking the minimum over \(b\), then summing over \(I\), and applying Theorem 6.2 proves the result. \(\square\)

The proof obligation has a direct interpretation:

\[
\boxed{
\text{Every first action must be covered by either rung-one slack or a downstream fusion tax.}
}
\]

### 11.1 Cheap slack lower bounds

Suppose

\[
L_I\le M_I
\]

is any lower witness for the best rung-one branch at \(I\), and

\[
F^{(1)}_{I,b}\le B_{I,b}
\]

is an action-conditioned upper bound for branch \(b\). Then

\[
\boxed{
\underline s_{I,b}
=
\max\{0,L_I-B_{I,b}\}
\le s_{I,b}.
}
\]

Indeed,

\[
s_{I,b}=M_I-F^{(1)}_{I,b}\ge L_I-B_{I,b},
\]

and \(s_{I,b}\ge0\).

This permits a structural second-rung proof without exact branch values. Some first actions may be covered because they are already demonstrably inferior; the remaining near-optimal actions must be covered by action-conditioned second-frontier regret events.

### 11.2 Safe-addition rules

At rung two:

1. **Within one \((I,b,J,c)\):** event terms may be added only if their supports are disjoint or their sum is proved pointwise below \(R_{I,b,J}(\omega,c)\).
2. **Across second actions \(c\):** take a minimum; do not add.
3. **Across second states \(J\) after fixed \((I,b)\):** add, because those states are mutually exclusive.
4. **Across alternative first actions \(b\):** take the slack-plus-tax minimum; do not add.
5. **Across first states \(I\):** add, because their arrival is policy-independent and they are mutually exclusive.

### Theorem 11.2 — Safe telescoping across reveal-delay rungs

Let

\[
\Delta^{(r)}=U^{(r-1)}-U^{(r)}.
\]

If

\[
0\le\underline\Delta^{(r)}\le\Delta^{(r)}
\]

is separately certified for every \(r=1,\ldots,k\), then

\[
\boxed{
\sum_{r=1}^k\underline\Delta^{(r)}
\le
U^{(0)}-U^{(k)}.
}
\]

#### Proof

The exact increments telescope:

\[
U^{(0)}-U^{(k)}
=
\sum_{r=1}^k
\left(U^{(r-1)}-U^{(r)}\right)
=
\sum_{r=1}^k\Delta^{(r)}.
\]

Sum the rungwise lower bounds. \(\square\)

Event overlap across different rungs is not itself a problem when each event certificate lower-bounds the correct adjacent-rung increment. It is unsafe to count the same structural loss twice merely because it can be described at two stages; each claimed amount must be attached to and proved against its own marginal relaxation difference.

---

## 12. Exact receipt schema for a second-rung computation

A compact aggregate receipt can replay the depth-two recurrence without storing every world row.

For every first state \(I\), first action \(b\), and reached second state \(J\), record:

\[
C_{I,b,J}
=
\sum_\omega
\mu_{I,b,J}(\omega)
\max_cq_{I,b,J}(\omega,c),
\]

and, for every legal second action \(c\),

\[
A_{I,b,J,c}
=
\sum_\omega
\mu_{I,b,J}(\omega)
q_{I,b,J}(\omega,c).
\]

Also record \(\Theta_{I,b}\). Then compute

\[
\delta_{I,b,J}
=
C_{I,b,J}-\max_cA_{I,b,J,c},
\]

\[
F^{(1)}_{I,b}
=
\Theta_{I,b}+\sum_JC_{I,b,J},
\]

\[
F^{(2)}_{I,b}
=
\Theta_{I,b}+\sum_J\max_cA_{I,b,J,c},
\]

\[
s_{I,b}
=
\max_{b'}F^{(1)}_{I,b'}-F^{(1)}_{I,b},
\]

and finally

\[
\boxed{
\Delta^{(2)}
=
\sum_I
\min_b
\left[
s_{I,b}+\sum_J\delta_{I,b,J}
\right].
}
\]

The aggregate receipt verifies the gluing recurrence. To verify the aggregates independently, a full receipt must additionally provide the world-level masses and \(q\)-values, or another independently checkable exact counting certificate.

### 12.1 Deterministic exact-rational verifier

The following standard-library-only Python verifies the aggregate recurrence. Fractions are JSON strings such as `"1483/138600"` or `"0"`.

```python
from __future__ import annotations

from fractions import Fraction
from typing import Any


def F(value: str | int) -> Fraction:
    return Fraction(value)


def verify_second_rung(receipt: dict[str, Any]) -> Fraction:
    """Verify an aggregate depth-two receipt and return exact Delta^(2).

    Schema:
      {
        "first_states": [
          {
            "id": "I0",
            "actions": [
              {
                "id": "b0",
                "terminal": "0",
                "second_states": [
                  {
                    "id": "J0",
                    "clairvoyant_sum": "7/10",
                    "common_action_sums": {
                      "c0": "3/5",
                      "c1": "1/2"
                    }
                  }
                ]
              }
            ]
          }
        ],
        "expected_delta2": "1/10"
      }
    """
    total = Fraction(0)

    for first_state in receipt["first_states"]:
        rows: list[tuple[str, Fraction, Fraction, Fraction]] = []

        for branch in first_state["actions"]:
            terminal = F(branch.get("terminal", "0"))
            f1 = terminal
            f2 = terminal
            downstream_tax = Fraction(0)

            for second_state in branch.get("second_states", []):
                clairvoyant = F(second_state["clairvoyant_sum"])
                action_sums = [
                    F(v)
                    for v in second_state["common_action_sums"].values()
                ]
                assert action_sums, "Every positive-mass second state needs an action."
                glued = max(action_sums)
                delta = clairvoyant - glued
                assert delta >= 0, "Negative local tax: invalid receipt."

                f1 += clairvoyant
                f2 += glued
                downstream_tax += delta

            assert f1 - f2 == downstream_tax
            rows.append((branch["id"], f1, f2, downstream_tax))

        assert rows, "Every first state needs a legal first action."
        rung1_max = max(row[1] for row in rows)
        local_direct = rung1_max - max(row[2] for row in rows)
        local_interchange = min(
            (rung1_max - f1) + downstream_tax
            for _, f1, _, downstream_tax in rows
        )
        assert local_direct == local_interchange
        total += local_direct

    if "expected_delta2" in receipt:
        assert total == F(receipt["expected_delta2"])

    return total
```

This verifier does not establish that a reported `clairvoyant_sum` or `common_action_sum` came from the Texas 42 rules. It verifies the exact depth-two algebra once those aggregates have an independent receipt.

---

## 13. Grading against Experiment 15.1

Handoff 017 reports the following exact decomposition on five grade-4 coordinates. Because grade 4 has only two nontrivial future focal decisions after the root play,

\[
U^{(2)}=Q^H.
\]

| coordinate | reported \(\Delta^{(1)}\) | reported \(\Delta^{(2)}\) | decimal \(\Delta^{(2)}\) | second-rung share of total fusion |
|---|---:|---:|---:|---:|
| h0 | \(19863799/179625600\) | \(387281/5132160\) | 0.075461599 | 40.561% |
| h6 | \(611579/21772800\) | \(5399143/479001600\) | 0.011271660 | 28.637% |
| h2 | \(145/22176\) | \(1483/138600\) | 0.010699856 | 62.070% |
| h9 | \(227251/3326400\) | \(4532503/26611200\) | 0.170323135 | 71.372% |
| h12 | \(34519/1995840\) | \(95917/4989600\) | 0.019223385 | 52.640% |

These values are **experimental receipts reported by the branch**. The theorem above does not manufacture their numerators from the five-row summary. It states the unique exact recurrence that a depth-two frontier receipt must satisfy.

### 13.1 h6: first rung already closes

The reported first-rung surplus is

\[
\frac{4930081}{479001600}.
\]

Applying the exact second rung can only lower the competitor further. Adding the reported \(\Delta^{(2)}\) gives a full-\(H\) strict surplus of

\[
\boxed{
\frac{1291153}{59875200}
\approx0.021564070.
}
\]

The proof architecture is therefore complete on the upper side. The compact lower-plan witness remains the open compression problem; the current lower witness is the exact \(H\)-seeded value.

### 13.2 h0: rung two closes what rung one missed

The post-rung-one shortfall reported for h0 is

\[
\frac{5390549}{179625600}.
\]

The reported second-rung tax exceeds that shortfall by

\[
\boxed{
\frac{387281}{5132160}
-
\frac{5390549}{179625600}
=
\frac{4082143}{89812800}
\approx0.045451684.
}
\]

Therefore, under the same exact candidate witness used in the experiment, \(U^{(2)}=H\) closes h0 strictly.

### 13.3 h2, h9, and h12: exact ties terminate at equality

At the three tied coordinates, the post-rung-one shortfall is reported to equal \(\Delta^{(2)}\) exactly. Since \(U^{(2)}=H\), the second rung removes the entire remaining fusion gap and reaches equality. No valid method should produce strict separation between actions whose exact \(H\) values are tied.

For these coordinates, the depth-two theorem is not merely an upper-bound improvement. It is the exact final nonanticipativity correction.

### 13.4 What the aggregate table does and does not reveal

The reported values show:

- h2 has a small total fusion gap, but its second rung supplies about 62% of that gap;
- h9 is strongly depth-two dominated, with about 71% of its fusion gap on the second rung;
- h12 is comparatively balanced across the two rungs.

The five-row summary alone cannot determine whether those differences come from:

- more second-frontier occupancy mass;
- larger per-world second-action regret;
- more second-frontier information states with conflict;
- different first-action escape slacks; or
- a combination of all four.

The decomposition required to distinguish them is exactly

\[
(I,b)\mapsto
\left(
 s_{I,b},
 \{\delta_{I,b,J}\}_J
\right).
\]

### 13.5 h2 hand audit boundary

Handoff 017 reports h2's

\[
\Delta^{(2)}=\frac{1483}{138600}
\]

from a 330-state frontier. A by-hand or independent machine replay requires the branch receipt described in §12: at minimum, \(\Theta_{I,b}\), each \(C_{I,b,J}\), and every \(A_{I,b,J,c}\). Those rows are not contained in the handoff summary, so this note does not claim an independent reconstruction of the numerator \(1483\).

Once that receipt is emitted, the exact audit is finite and direct:

1. verify every \(\delta_{I,b,J}=C_{I,b,J}-\max_cA_{I,b,J,c}\);
2. add over \(J\) only after fixing \((I,b)\);
3. compute every first-action slack;
4. take \(\min_b(s+d)\) at each \(I\); and
5. sum over \(I\) and reduce the fraction.

This is the smallest useful next receipt, because it tests the action-conditioned occupancy and the policy-adjustment minimum separately.

---

## 14. What rung two changes in the trick-1 program

The opening-lead target now has a precise recursive shape.

### 14.1 The upper proof is an action cover at every stage

At the first rung, every candidate common action at a frontier state must incur a proved regret or share a common optimum.

At the second rung, every **first** action must be covered by one of two mechanisms:

- it has enough rung-one slack to be irrelevant; or
- under its own induced occupancy, every viable second action pays enough regret.

This is a decision-sparse certificate. It need not reproduce the full policy. It needs only to show that no first-action escape route preserves the clairvoyant upper.

### 14.2 The penalty route does not require a flat depth-two frontier

Validity requires conditional centering, not explicit enumeration. A structural penalty family may be counted by exact symbolic methods provided the following quantities can be obtained under every relevant public/action prefix:

\[
\mathbb E
\left[
\phi_{t,j}(X_t,G_t,a)
\mid G_t
\right].
\]

This is the route around a 399-million-world extraction map. The state space may remain large while the feature moments and their proof obligations remain small.

### 14.3 Exact recovery is a benchmark, not the cheap construction

The perfect martingale penalty proves that the dual has no mathematical gap: stagewise centered penalties can recover every rung exactly. But the perfect penalty encodes exact continuation values.

The practical research question is now sharply stated:

> Is there a small action-conditioned feature family whose conditionally centered penalties remove enough of each competitor's fusion value to cross the incumbent lower witness?

The reduced grade-4 receipts can answer which structural features approximate the perfect penalties and which first-action escape routes must be blocked.

### 14.4 The lower side remains compositional

The h6 closure uses an exact \(H\)-seeded candidate witness. The durable end-to-end architecture still asks for a compact lawful plan:

\[
\boxed{
\text{one compositional lower plan}
+
\text{one multistage nonanticipativity tax per live competitor}.
}
\]

Residual plans must carry the true arrival posterior or be pointwise guaranteed. That repair strengthens rather than weakens the plan calculus.

---

## 15. Claim ledger

| Claim | Status | Remaining obligation |
|---|---|---|
| First-frontier occupancy is policy-independent | Exact result | Frontier must truly be the next focal decision after the fixed root action |
| Second-frontier occupancy must be indexed by \((I,b,J)\) | Exact result | Engine must preserve the complete public/action history |
| Exact nested formulas for \(U^{(1)}\) and \(U^{(2)}\) | Exact result | Emit exact occupancies and continuation aggregates |
| Conditional additivity \(d_{I,b}=\sum_J\delta_{I,b,J}\) | Exact result | Add only after fixing \((I,b)\) |
| Slack–tax interchange law \(\Delta^{(2)}=\sum_I\min_b(s+d)\) | Exact result | None beyond the finite-model hypotheses |
| Taxing only a rung-one optimizer is unsafe | Exact corollary | Search all first-action escape routes |
| Recursive local law for deeper rungs | Exact algebraic result | Preserve nested policy-dependent occupancies in implementation |
| Incremental second-rung centered-penalty bound | Exact result | Center separately under every \((I,b,J)\) occupancy |
| Perfect second-rung penalty recovers \(U^{(2)}\) | Exact result | Exact \(q\)-values are required for the perfect penalty |
| Multistage martingale-difference weak duality | Exact result | Public filtration and action chronology must match the engine |
| Backward-centered penalties recover every finite rung | Exact result | Computational cheapness is not implied |
| Structural feature penalties are valid after conditional centering | Certificate schema | Prove/count each conditional feature moment exactly |
| Depth-two regret-event minorants | Certificate schema | Prove pointwise regret and action-conditioned event mass |
| Two-stage action-cover lower bound | Certificate schema | Lower-bound every first action's slack plus downstream tax |
| Safe telescoping of adjacent-rung certificates | Exact result | Each amount must target its own adjacent relaxation difference |
| Five effective opening-lead rungs | Exact combinatorial correction | Forced-action convention must be explicit |
| Action-independent pointwise upper cannot shave \(C\) | Exact result | None |
| Residual-plan witness must use arrival posterior or a pointwise guarantee | Exact validity boundary | Carry posterior in the plan-certificate representation |
| Experiment 15.1 exact fractions | Experimental receipt — reported | Independent replay from branch artifacts |
| h0 closes strictly at rung two | Exact arithmetic consequence of reported receipts | Same candidate lower witness as the experiment |
| h2's \(1483/138600\) reconstructed from frontier rows | Open | Emit the 330-state aggregate or world-level receipt |
| Small feature penalties close trick-1 competitors | Open | Discover, prove, and count a sufficient feature basis |
| Second-rung fusion cores remain binary | Open | Measure the second-frontier conflict hypergraph separately |

---

## 16. Conclusion

The second wall is not a failure of the first calculus. It is the first place where policy adjustment becomes visible.

At rung one, frontier states have a fixed arrival law and their local Jensen gaps add. At rung two, the first common action changes the worlds and public states that reach the next decision. The controller can respond to a new nonanticipativity constraint by changing that first action. The exact price is therefore not the downstream tax under one chosen optimizer.

It is

\[
\boxed{
\text{first-action slack}
+
\text{second-action fusion tax under that action's occupancy},
}
\]

minimized over every first action:

\[
\boxed{
U^{(1)}-U^{(2)}
=
\sum_I
\min_b
\left[
 s_{I,b}
+
\sum_J\delta_{I,b,J}
\right].
}
\]

The corresponding proof certificate has a simple logic:

> Every possible first action is already too costly, or it leads to a second decision at which hidden-world-contingent play has a provable value.

The dual says the same thing dynamically. Center an information penalty under the lawful posterior at every public/action prefix. The increments have zero expectation for every lawful policy, so they are valid. Center exact continuation values backward, and the penalized clairvoyant solve becomes the lawful solve exactly.

Therefore the door found at rung one does not close at rung two. It becomes recursive.

\[
\boxed{
\text{Nonanticipativity has a Bellman calculus.}
}
\]

One rung down, four effective opening-lead rungs remain. The mathematics is now explicit about what each rung must prove, what may be added, what must be minimized, and how a symbolic penalty family can replace a flat enumeration.
