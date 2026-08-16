<!-- HARVEST METADATA
dispatch: exchange/outbox/016-cheap-upper-witness-handoff.md
channel: ChatGPT 5.6 Pro (app/web), informal conversational register (x:014 style)
transport: hand-ferried by Jason (courier automation watches the main checkout,
  not this worktree; Jason pasted the GitHub raw of the outbox body and uploaded
  Pro's reply file on 2026-08-14)
harvested: 2026-08-14
status: UNADJUDICATED — no claim below may touch the wiki until walt-math
  adjudicates (exchange protocol: witnesses re-run, proofs step-checked).
  The note self-classifies claims (exact result / certificate schema /
  research proposal); those labels are Pro's, not ours, until confirmed.
body: verbatim below this comment, byte-for-byte from the uploaded file.
-->

---
title: "Decision-Sparse Exact Solving: Nonanticipativity Taxes and a Compositional Plan Calculus for Straight Texas 42"
version: "0.1"
status: "Research note; exact lemmas, certificate schemas, and experiment specification"
date: "2026-08-14"
repository: "jasonyandell/texas-42"
branch: "worktree-walt-s2"
---

# Decision-Sparse Exact Solving

## Nonanticipativity Taxes and a Compositional Plan Calculus for Straight Texas 42

**Version 0.1 — 2026-08-14**

## Abstract

This note isolates a mathematical route from the clairvoyant treatment \(C\) upper witness to the lawful imperfect-information treatment \(H\) value without first solving or partitioning the entire physical-world space.

The central observation is that the difference

\[
U_a^C-Q^H(a)
\]

is exactly the minimum expected **clairvoyance regret** paid by any lawful policy after root action \(a\). Treatment \(C\) is too optimistic only because it may choose different future actions in hidden worlds that are indistinguishable to the focal player. The primitive correction is therefore not a global partition of all physical worlds. It is a local **nonanticipativity constraint**: latent copies of the same focal information state must choose the same action.

This yields four connected structures.

1. A canonical reveal-delay ladder from \(C\) to \(H\), with at most six future focal-decision layers after an opening lead.
2. An exact first-layer fusion tax, equal to an average-of-maxima minus a maximum-of-averages Jensen gap.
3. Cheap upper certificates built from action-conditioned continuation bounds, regret events, block-gluing costs, or zero-mean information taxes.
4. A compositional lower-witness calculus in which a lawful partial plan secures some outcome and hands a publicly typed residual position to another certificate.

The intended decision proof is small on both sides: one lawful plan for the candidate action, and one unavoidable information tax for each competitor. The underlying truth may remain high-dimensional even when the proof of the decision is not.

---

## 1. Provenance, scope, and claim discipline

This note is intended to sit beside the project documents:

- `walt/math/decision_sparse_exact_solving_v0.1.md`
- `walt/math/decision_sparse_exact_solving_v0.1_errata.md`
- `walt/math/decision_sparse_second_audit_v0.1.md`
- `exchange/outbox/016-cheap-upper-witness-handoff.md`

The project definitions and errata govern where they differ from this note. In particular, the valid root-action comparison is action-conditioned throughout. No cross-action theorem may substitute an unrestricted treatment-\(C\) value for the treatment-\(C\) value obtained after the same fixed root action.

This note separates three statuses.

- **Exact result:** proved here from the displayed assumptions.
- **Certificate schema:** a valid theorem once its stated local inequalities or lawfulness obligations are supplied.
- **Research proposal:** a suggested representation or experiment, not yet a completed theorem about the full opening-hand space.

The note is count-neutral unless explicitly stated. Extending a theorem from trick utility to Straight Texas 42 point or contract utility requires carrying the correct payoff state and reproving any step that depends on the payoff decoration.

---

## 2. Setup and notation

Fix the following data.

- \(s\): a focal information state.
- \(m\): the focal seat.
- \(a\): a fixed legal root action at \(s\).
- \(\Omega_a\): the physical worlds compatible with \(s\) and root action \(a\).
- \(\beta\): the belief distribution on \(\Omega_a\).
- \(\sigma_{-m}\): the fixed field policy for the other seats.
- \(U\): terminal utility under the declared payoff contract.

A physical world \(\omega\in\Omega_a\) supplies the hidden allocation and any other hidden data needed to make the continuation a perfect-information game.

### 2.1 Treatment \(C\)

Treatment \(C\) fixes root action \(a\), reveals \(\omega\) to the focal controller for the continuation, and then permits a world-specific optimal continuation policy.

Let

\[
V_a^C(\omega)
\]

be the resulting optimal expected terminal utility in physical world \(\omega\). Its action-conditioned upper witness is

\[
U_a^C
=
\mathbb E_{\omega\sim\beta}\!
\left[V_a^C(\omega)\right].
\]

### 2.2 Treatment \(H\)

Treatment \(H\) permits only lawful focal policies: the same action must be chosen at every latent history that induces the same focal information state.

Let \(\mathcal R_H(a)\) be the set of lawful continuation policies whose root action is \(a\). For \(\rho\in\mathcal R_H(a)\), define

\[
\alpha_\rho(\omega)
=
\mathbb E
\left[
U
\mid
\omega,\rho,\sigma_{-m}
\right].
\]

Then

\[
Q^H(a)
=
\max_{\rho\in\mathcal R_H(a)}
\mathbb E_{\omega\sim\beta}
\left[
\alpha_\rho(\omega)
\right].
\]

Because a clairvoyant controller can imitate any lawful controller in every world,

\[
Q^H(a)\le U_a^C.
\]

The difference is the strategy-fusion value created by revealing information that the focal player does not possess.

---

## 3. The fusion-gap identity

### Definition 3.1 — Clairvoyance regret

For fixed root action \(a\), physical world \(\omega\), and lawful policy \(\rho\in\mathcal R_H(a)\), define

\[
r_a(\omega,\rho)
=
V_a^C(\omega)-\alpha_\rho(\omega).
\]

Since treatment \(C\) optimizes after seeing \(\omega\),

\[
r_a(\omega,\rho)\ge0.
\]

### Theorem 3.2 — Exact fusion-gap identity

\[
\boxed{
U_a^C-Q^H(a)
=
\min_{\rho\in\mathcal R_H(a)}
\mathbb E_{\omega\sim\beta}
\left[
r_a(\omega,\rho)\right].
}
\]

#### Proof

Because \(U_a^C\) does not depend on \(\rho\),

\[
\begin{aligned}
U_a^C-Q^H(a)
&=
\mathbb E[V_a^C]
-
\max_{\rho\in\mathcal R_H(a)}
\mathbb E[\alpha_\rho]
\\[2mm]
&=
\min_{\rho\in\mathcal R_H(a)}
\left(
\mathbb E[V_a^C]
-
\mathbb E[\alpha_\rho]
\right)
\\[2mm]
&=
\min_{\rho\in\mathcal R_H(a)}
\mathbb E
\left[
V_a^C-\alpha_\rho
\right]
\\[2mm]
&=
\min_{\rho\in\mathcal R_H(a)}
\mathbb E[r_a(\omega,\rho)].
\end{aligned}
\]

\(\square\)

### Corollary 3.3 — Fusion-tax upper certificate

Suppose a proved quantity \(\underline\Gamma_a\) satisfies

\[
0\le \underline\Gamma_a
\le
U_a^C-Q^H(a).
\]

Then

\[
\boxed{
Q^H(a)
\le
U_a^C-\underline\Gamma_a.
}
\]

If \(L_{a^\star}\) is a lawful lower witness for candidate action \(a^\star\), then competitor \(a\) is separated whenever

\[
\boxed{
\underline\Gamma_a
\ge
U_a^C-L_{a^\star}.
}
\]

This is the central dual objective. We need not solve \(H\) completely. We need only prove enough unavoidable clairvoyance regret to close the observed comparison gap.

---

## 4. The geometry between \(C\) and \(H\)

### 4.1 Local copies of an information state

Let \(I\) be a focal information state that may occur after root action \(a\). Let

\[
X_I
\subseteq
\Omega_a
\]

be the set of physical worlds whose latent continuation can reach \(I\) with positive mass.

Treatment \(C\) may use one action variable

\[
a_{I,\omega}
\]

for every \(\omega\in X_I\). Treatment \(H\) requires

\[
a_{I,\omega}=a_{I,\omega'}
\qquad
\text{for all }\omega,\omega'\in X_I,
\]

with the usual action transport if equivalent information states use different concrete labels.

The forbidden power in \(C\) is therefore local: it can distinguish latent copies of the same information state.

### 4.2 Partition relaxations

Let \(\Pi_I\) be a partition of \(X_I\). A \(\Pi_I\)-relaxed controller must choose one common action within each block of \(\Pi_I\), but may choose different actions in different blocks.

- Treatment \(C\): \(\Pi_I\) is the singleton partition.
- Treatment \(H\): \(\Pi_I=\{X_I\}\), the one-block partition.
- Intermediate treatment: any partition between those extremes.

Across all focal information states, the relaxation geometry is a product of partition lattices:

\[
\boxed{
\mathfrak P
=
\prod_I \operatorname{Part}(X_I).
}
\]

A global feature such as trump split, boss ownership, ruff structure, or entry structure matters only insofar as it induces useful blocks inside one or more \(X_I\).

This reverses the natural but expensive starting point. We need not first partition all physical worlds. We may instead ask exactly which latent action copies must be glued to remove enough clairvoyant value.

### 4.3 Decision-relative distance

Let \(U_a(\Pi)\) be the relaxed upper value under a collection of local partitions \(\Pi=(\Pi_I)_I\). Let \(\operatorname{cost}(\Pi)\) measure representation, counting, or solve cost.

For a target value \(T\), define

\[
\boxed{
\kappa_a(T)
=
\min
\left\{
\operatorname{cost}(\Pi)
:
U_a(\Pi)\le T
\right\}.
}
\]

For root-action separation, the natural target is

\[
T=L_{a^\star}.
\]

Thus \(\kappa_a(L_{a^\star})\) asks for the cheapest proved nonanticipativity structure that pushes competitor \(a\) below the incumbent witness. It is a more relevant notion of distance from \(C\) than the number of world blocks or the entropy of a global world partition.

---

## 5. A canonical reveal-delay ladder

Arbitrary feature partitions are not the only interpolation between \(C\) and \(H\). There is a canonical semantic ladder defined by delaying revelation through successive focal decisions.

### Definition 5.1 — The treatments \(C^{(k)}\)

Set

\[
C^{(0)}=C.
\]

For \(k\ge1\), treatment \(C^{(k)}\):

1. fixes root action \(a\);
2. withholds the physical world through the next \(k\) future focal decisions;
3. requires lawful information-state actions at those decisions;
4. reveals the world immediately after the \(k\)-th such decision; and
5. permits treatment-\(C\) play thereafter.

Let \(U_a^{(k)}\) be its value.

### Proposition 5.2 — Monotonicity

\[
\boxed{
U_a^{(0)}
\ge
U_a^{(1)}
\ge
U_a^{(2)}
\ge
\cdots
\ge
Q^H(a).
}
\]

#### Proof

Every policy admissible under \(C^{(k+1)}\) is admissible under \(C^{(k)}\): the latter reveals the world no later and therefore imposes no additional restriction. Every lawful \(H\)-policy is admissible under every \(C^{(k)}\). Inclusion of policy classes gives the inequalities. \(\square\)

### Proposition 5.3 — Finite termination at \(H\)

Suppose that after root action \(a\), the focal player can act at most \(N\) more times before terminal play. Then

\[
\boxed{
U_a^{(N)}=Q^H(a).
}
\]

#### Proof

After revelation has been withheld through every remaining focal decision, no focal action remains on which revealed information could act. The admissible focal behavior is therefore exactly lawful behavior. \(\square\)

For an opening lead in a seven-trick hand, there are at most six later focal decisions. Hence

\[
U_a^{(6)}=Q^H(a).
\]

For a root with four tiles remaining before the root play, there are at most three later focal decisions. Hence

\[
U_a^{(3)}=Q^H(a).
\]

This is a bound on **semantic depth**, not on computational width. A single layer may still contain many worlds and information states. Nevertheless, it replaces one undifferentiated strategy-fusion gap with at most six decision layers.

### Definition 5.4 — Layer taxes

Define

\[
\Delta_a^{(k)}
=
U_a^{(k-1)}-U_a^{(k)}
\qquad
(k=1,\ldots,N).
\]

Then

\[
\boxed{
U_a^C-Q^H(a)
=
\sum_{k=1}^{N}\Delta_a^{(k)}.
}
\]

Each \(\Delta_a^{(k)}\) is the exact value of hiding the physical world for one additional focal decision.

---

## 6. The exact first fusion layer

The first reveal-delay layer has a closed form.

### 6.1 Next-decision frontier

Let \(T\) be the focal player's next decision time after root action \(a\), or terminal if the focal player never acts again.

For each nonterminal focal information state \(I\) at this frontier, let

\[
\mu_I(\omega)
=
\Pr_\beta(\omega, I_T=I)
\]

be the unnormalized joint mass of world \(\omega\) and arrival at \(I\). Worlds with zero mass may be omitted.

For legal action \(b\in\mathcal A(I)\), define

\[
q_I(\omega,b)
\]

as the expected terminal utility obtained by:

1. reaching information state \(I\) in world \(\omega\);
2. choosing action \(b\);
3. revealing \(\omega\) immediately after that action; and
4. using treatment \(C\) thereafter.

Because all histories represented by \(I\) have the same focal hand and public record, \(\mathcal A(I)\) is common across its latent worlds.

Terminal-frontier contributions are identical under \(C^{(0)}\) and \(C^{(1)}\); write their total as \(T_a\).

### Theorem 6.1 — Exact first-layer values

\[
\boxed{
U_a^{(0)}
=
T_a
+
\sum_I
\sum_{\omega\in X_I}
\mu_I(\omega)
\max_{b\in\mathcal A(I)} q_I(\omega,b).
}
\]

\[
\boxed{
U_a^{(1)}
=
T_a
+
\sum_I
\max_{b\in\mathcal A(I)}
\sum_{\omega\in X_I}
\mu_I(\omega)q_I(\omega,b).
}
\]

#### Proof

Under \(C^{(0)}\), the focal controller knows \(\omega\) at \(I\), so it may maximize \(q_I(\omega,b)\) separately in each world. Under \(C^{(1)}\), it must choose one action from the information state \(I\) before revelation, so the same \(b\) applies to every latent world in \(X_I\). After that action the world is revealed, exactly as represented in \(q_I\). Summing the mutually exclusive frontier states and adding common terminal terms gives the result. \(\square\)

### Definition 6.2 — Local first-layer tax

Let

\[
m_I(\omega)
=
\max_{c\in\mathcal A(I)}q_I(\omega,c).
\]

Define

\[
\boxed{
\delta_I
=
\sum_\omega
\mu_I(\omega)m_I(\omega)
-
\max_b
\sum_\omega
\mu_I(\omega)q_I(\omega,b).
}
\]

Then

\[
\boxed{
\Delta_a^{(1)}
=
\sum_I \delta_I.
}
\]

The information states on the next-decision frontier are mutually exclusive, so their taxes add exactly.

### Proposition 6.3 — Regret form

\[
\boxed{
\delta_I
=
\min_{b\in\mathcal A(I)}
\sum_\omega
\mu_I(\omega)
\left[
m_I(\omega)-q_I(\omega,b)
\right].
}
\]

#### Proof

The first term in the definition of \(\delta_I\) is independent of \(b\). Therefore subtracting the maximum expected action value is equivalent to minimizing expected regret. \(\square\)

### Corollary 6.4 — Zero-tax criterion

\[
\boxed{
\delta_I=0
\iff
\bigcap_{\mu_I(\omega)>0}
\arg\max_{b\in\mathcal A(I)} q_I(\omega,b)
\ne\varnothing.
}
\]

#### Proof

Every summand in the regret form is nonnegative. Their weighted sum is zero exactly when one common action has zero regret in every positive-mass world. \(\square\)

This criterion must use the complete optimal action sets. Inspecting one arbitrarily tie-broken treatment-\(C\) optimizer can manufacture a conflict that another optimizer on the same optimal face avoids.

---

## 7. The binary-action formula

Suppose \(\mathcal A(I)=\{b_0,b_1\}\). Let

\[
p_I=\sum_\omega\mu_I(\omega),
\qquad
\nu_I(\omega)=\frac{\mu_I(\omega)}{p_I},
\]

and define the action advantage

\[
d(\omega)
=
q_I(\omega,b_1)-q_I(\omega,b_0).
\]

Let the conditional local tax be

\[
\widehat\delta_I=\frac{\delta_I}{p_I}.
\]

### Proposition 7.1 — Exact binary tax

Under posterior \(\nu_I\),

\[
\boxed{
\widehat\delta_I
=
\frac12
\left(
\mathbb E_{\nu_I}|d|
-
\left|\mathbb E_{\nu_I}d\right|
\right).
}
\]

Equivalently,

\[
\boxed{
\widehat\delta_I
=
\min
\left\{
\mathbb E_{\nu_I}[d_+],
\mathbb E_{\nu_I}[(-d)_+]
\right\}.
}
\]

#### Proof

Relative to always choosing \(b_0\), treatment \(C\) gains \(\mathbb E[d_+]\), while the best common action gains \(\max(0,\mathbb E[d])\). Hence

\[
\widehat\delta_I
=
\mathbb E[d_+]-\max(0,\mathbb E[d]).
\]

Using

\[
d=d_+-(-d)_+,
\qquad
|d|=d_++(-d)_+,
\]

gives both displayed forms. \(\square\)

The interpretation is exact.

- Worlds with \(d>0\) prefer \(b_1\).
- Worlds with \(d<0\) prefer \(b_0\).
- Treatment \(C\) harvests both advantage masses.
- A lawful common action must surrender the smaller opposing mass.

No explicit pairing of worlds is required.

---

## 8. More than two actions: fusion cores are hyperedges

Pairwise disagreement is not a complete description when three or more actions are legal.

Consider three worlds with treatment-\(C\) optimal action sets

\[
A_1=\{a,b\},
\qquad
A_2=\{b,c\},
\qquad
A_3=\{a,c\}.
\]

Every pair has a common optimal action, but

\[
A_1\cap A_2\cap A_3=\varnothing.
\]

Thus every pair can appear harmless while the three-world block necessarily pays a fusion tax.

### Definition 8.1 — Minimal fusion core

A finite set \(S\subseteq X_I\) is a minimal fusion core if

\[
\bigcap_{\omega\in S}
\arg\max_b q_I(\omega,b)
=
\varnothing,
\]

but every proper subset of \(S\) has nonempty intersection.

### Proposition 8.2 — Small qualitative witness

Let \(k=|\mathcal A(I)|\). If

\[
\bigcap_{\mu_I(\omega)>0}
\arg\max_b q_I(\omega,b)
=
\varnothing,
\]

then some fusion core uses at most \(k\) worlds.

#### Proof

For each action \(b\in\mathcal A(I)\), the empty full intersection implies that there exists a positive-mass world \(\omega_b\) in which \(b\) is not optimal. The set

\[
S=\{\omega_b:b\in\mathcal A(I)\}
\]

has at most \(k\) worlds and excludes every action from the intersection. A minimal subset of \(S\) with empty intersection is a fusion core. \(\square\)

In Straight Texas 42, a player has at most seven tiles and therefore at most seven legal plays. A qualitative proof that one information state cannot retain the full treatment-\(C\) value can consequently have a witness involving no more than seven worlds, even if the information state represents millions of worlds.

### Definition 8.3 — Regret matrix

\[
\boxed{
R_I(\omega,b)
=
m_I(\omega)-q_I(\omega,b)
\ge0.
}
\]

Then

\[
\boxed{
\delta_I
=
\min_b
\sum_\omega\mu_I(\omega)R_I(\omega,b).
}
\]

The regret matrix is the correct quantitative object. A graph of pairwise world conflicts can discard genuine multi-action structure.

---

## 9. Exact block-gluing calculus

At the next-decision frontier, arbitrary blocks can be priced exactly.

For \(B\subseteq X_I\), define

\[
Q_B(b)
=
\sum_{\omega\in B}
\mu_I(\omega)q_I(\omega,b)
\]

and

\[
\boxed{
v_I(B)=\max_b Q_B(b).
}
\]

If \(\Pi_I\) is a partition of \(X_I\), then the relaxed value contributed by information state \(I\) is

\[
\boxed{
v_I(\Pi_I)
=
\sum_{B\in\Pi_I}v_I(B).
}
\]

- Singleton blocks reproduce treatment \(C\) at \(I\).
- The one-block partition reproduces treatment \(C^{(1)}\) at \(I\).

### Proposition 9.1 — Exact merge cost

For disjoint blocks \(B_1,B_2\subseteq X_I\), define

\[
\boxed{
c_I(B_1,B_2)
=
v_I(B_1)+v_I(B_2)-v_I(B_1\cup B_2).
}
\]

Then

\[
c_I(B_1,B_2)\ge0,
\]

and \(c_I(B_1,B_2)\) is exactly the upper-value reduction caused by requiring one common action across \(B_1\cup B_2\) instead of one action for each block.

#### Proof

Separate blocks may each choose their own maximizing action. The merged block must choose one action for their combined mass, so

\[
\max_b Q_{B_1}(b)+\max_b Q_{B_2}(b)
\ge
\max_b\left(Q_{B_1}(b)+Q_{B_2}(b)\right).
\]

The difference is precisely the displayed cost. \(\square\)

This gives an exact adaptive gluing rule at one frontier: choose block merges by proved upper shave relative to their representation or counting cost, and stop when the competitor crosses the incumbent lower witness.

Merge costs inside one information state can interact. They are not generally additive across an arbitrary sequence unless the value is recomputed after each merge. By contrast, local taxes from mutually exclusive frontier information states add exactly.

---

## 10. A cheap action-conditioned upper theorem

The exact \(q_I(\omega,b)\) table may itself be too expensive at trick 1. A valid upper witness does not require exact continuation values.

### Theorem 10.1 — Glued simple-function upper bound

For every next-decision information state \(I\), world \(\omega\in X_I\), and legal action \(b\), suppose

\[
B_I(\omega,b)
\ge
q_I(\omega,b).
\]

Then

\[
\boxed{
Q^H(a)
\le
T_a
+
\sum_I
\max_{b\in\mathcal A(I)}
\sum_{\omega\in X_I}
\mu_I(\omega)B_I(\omega,b).
}
\]

#### Proof

At information state \(I\), every lawful policy chooses one common action \(b(I)\). Its continuation under \(H\) is no greater than the continuation obtained by revealing the world after that action, namely \(q_I(\omega,b(I))\), and this is no greater than \(B_I(\omega,b(I))\). Therefore

\[
\sum_\omega\mu_I(\omega)
\operatorname{Value}_H(\omega,b(I))
\le
\sum_\omega\mu_I(\omega)B_I(\omega,b(I))
\le
\max_b\sum_\omega\mu_I(\omega)B_I(\omega,b).
\]

Summing the mutually exclusive information states and terminal terms gives the result. \(\square\)

The decisive order is

\[
\boxed{
\max_b\sum_\omega
}
\]

rather than

\[
\boxed{
\sum_\omega\max_b.
}
\]

That exchange glues the next focal decision and removes its strategy-fusion value.

### 10.2 Candidate upper features

The functions \(B_I(\omega,b)\) should be action-conditioned. Useful terms may include indicators or small integer bounds for:

- unavoidable loss of a boss;
- availability of a ruff after action \(b\);
- failure to retain or regain the lead;
- expenditure of a necessary trump;
- creation of an opponent entry;
- destruction of a partner entry;
- incompatible control obligations;
- inability to cash a protected count tile before control is lost;
- forced exposure of a losing suit;
- shortage of follower supply for an intended forcing sequence.

If \(B_I\) is a small simple function over Scheme/Fix-style predicates, its expectation may be computable by exact weighted model counting rather than physical-world enumeration.

A hand-only or action-independent upper feature is unlikely to be selective enough. The certificate must explain why this particular action cannot realize certain continuations.

---

## 11. Action-cover regret certificates

A second route is to lower-bound the tax directly.

### Theorem 11.1 — Regret minorant certificate

Suppose

\[
0
\le
g_I(\omega,b)
\le
R_I(\omega,b)

\]

for every positive-mass world and legal action. Then

\[
\boxed{
\delta_I
\ge
\min_b
\sum_\omega
\mu_I(\omega)g_I(\omega,b).
}
\]

#### Proof

For every common action \(b\), pointwise domination gives

\[
\sum_\omega\mu_I(\omega)R_I(\omega,b)
\ge
\sum_\omega\mu_I(\omega)g_I(\omega,b).
\]

Taking the minimum over \(b\) preserves the inequality. \(\square\)

### Corollary 11.2 — Event-form action cover

For each legal action \(b\), suppose there is an event \(E_b\subseteq X_I\) and a number \(\eta_b\ge0\) such that

\[
R_I(\omega,b)
\ge
\eta_b
\qquad
\text{for every }\omega\in E_b.
\]

Then

\[
\boxed{
\delta_I
\ge
\min_b
\eta_b\Pr(E_b,I).
}
\]

Here \(\Pr(E_b,I)=\sum_{\omega\in E_b}\mu_I(\omega)\).

This certificate may require only one exactly countable regret event per legal action. With at most seven legal actions, a useful upper proof may have at most seven action obligations at a given information state rather than hundreds of millions of world obligations.

### 11.3 Proving a regret event by a local sandwich

The exact \(R_I\) need not be known. On event \(E_b\), it is enough to exhibit:

- an alternative action \(c\);
- a lawful or otherwise valid local lower witness \(L_c(\omega)\) for action \(c\); and
- a valid local upper bound \(B_b(\omega)\) for action \(b\),

such that

\[
L_c(\omega)-B_b(\omega)
\ge
\eta_b
\qquad
(\omega\in E_b).
\]

Indeed,

\[
m_I(\omega)
\ge
q_I(\omega,c)
\ge
L_c(\omega),
\]

while

\[
q_I(\omega,b)
\le
B_b(\omega).
\]

Therefore

\[
R_I(\omega,b)
=m_I(\omega)-q_I(\omega,b)
\ge
\eta_b.
\]

This is the upper-side analogue of a plan theorem. Whichever common action is chosen, some positive-mass family of worlds makes that action pay a proved amount.

### 11.4 No free summation

Regret certificates may overlap. Two lower bounds on the same \(R_I(\omega,b)\) cannot simply be added unless their supports are disjoint or a separate proof shows that their sum remains pointwise below \(R_I\).

Safe addition occurs in three common cases.

1. The events are disjoint for each action.
2. A single combined function \(g_I\) is proved pointwise below \(R_I\).
3. The taxes arise at mutually exclusive information states or successive exact reveal-delay layers.

---

## 12. Hard gluing and zero-mean information taxes

Nonanticipativity also has a dual penalty form.

### Theorem 12.1 — One-stage zero-mean penalty bound

For each information state \(I\), choose numbers \(\lambda_I(\omega,b)\) satisfying

\[
\boxed{
\sum_\omega
\mu_I(\omega)\lambda_I(\omega,b)
=0
\qquad
\text{for every legal }b.
}
\]

Then

\[
\boxed{
U_a^{(1)}
\le
T_a
+
\sum_I
\sum_\omega
\mu_I(\omega)
\max_b
\left[
q_I(\omega,b)-\lambda_I(\omega,b)
\right].
}
\]

Consequently the right-hand side is also an upper bound on \(Q^H(a)\).

#### Proof

Fix one common action \(b\) at information state \(I\). Centering gives

\[
\sum_\omega\mu_I(\omega)q_I(\omega,b)
=
\sum_\omega\mu_I(\omega)
\left[q_I(\omega,b)-\lambda_I(\omega,b)\right].
\]

For every \(\omega\), the chosen action's penalized value is no greater than the maximum penalized value over actions. Hence

\[
\sum_\omega\mu_I(\omega)q_I(\omega,b)
\le
\sum_\omega\mu_I(\omega)
\max_c
\left[q_I(\omega,c)-\lambda_I(\omega,c)\right].
\]

Maximize the left side over common actions and sum over \(I\). \(\square\)

### Proposition 12.2 — Exact recovery of the first glued value

Let

\[
p_I=\sum_\omega\mu_I(\omega)
\]

and

\[
\bar q_I(b)
=
\frac1{p_I}
\sum_\omega\mu_I(\omega)q_I(\omega,b).
\]

Choose

\[
\boxed{
\lambda_I(\omega,b)
=
q_I(\omega,b)-\bar q_I(b).
}
\]

Then the centering condition holds and the penalty upper bound is exactly \(U_a^{(1)}\).

#### Proof

The penalty is centered by the definition of \(\bar q_I(b)\), and

\[
q_I(\omega,b)-\lambda_I(\omega,b)
=
\bar q_I(b),
\]

which is independent of \(\omega\). Therefore

\[
\sum_\omega\mu_I(\omega)
\max_b\left[q_I(\omega,b)-\lambda_I(\omega,b)\right]
=
p_I\max_b\bar q_I(b)
=
\max_b\sum_\omega\mu_I(\omega)q_I(\omega,b).
\]

\(\square\)

Thus hard gluing and zero-mean information taxes are primal and dual descriptions of the same first-layer object.

### 12.3 Feature-based penalties

Let \(\phi_j(\omega,I,b)\) be structural features. A centered feature penalty has the form

\[
\lambda_I(\omega,b)
=
\sum_j\theta_j
\left(
\phi_j(\omega,I,b)
-
\mathbb E_{\nu_I}
[\phi_j(\cdot,I,b)]
\right).
\]

Every coefficient vector \(\theta\) gives a valid one-stage penalty. The coefficients may then be chosen to minimize the resulting upper witness.

This gives a principled role to features such as trump split, boss location, ruff access, follower supply, and entry ownership: they become shadow-price coordinates used to neutralize illegal clairvoyant advantage, rather than hidden facts granted to the focal player.

### 12.4 Multi-stage caution

At multiple focal decisions, global zero-mean centering is not sufficient. The penalty increment at a decision must have conditional mean zero given the lawful information state and chosen action. Equivalently, the penalty sequence should form a martingale-difference process relative to the focal information filtration.

A fully formal multi-stage penalty theorem should be built by induction along the reveal-delay ladder. Until that theorem is written, stagewise conditional centering is the safe contract.

---

## 13. A compositional calculus for lawful lower plans

The upper witness is only half of a decision proof. The candidate action requires a lawful lower witness. The drawing-hand theorem suggests a broader compositional language.

### 13.1 Partial plan certificate

A partial plan certificate is a tuple

\[
P=(G,\pi,\tau,g,\{L_h\}).
\]

Its components are:

- \(G\): an observable root guard, expressed only in the focal hand, declaration, and public record;
- \(\pi\): a lawful partial focal policy;
- \(\tau\): a public stopping time at which the plan hands control to a residual certificate;
- \(g(h)\): the accumulated guaranteed or exactly evaluated payoff component at public leaf history \(h\); and
- \(L_h\): a lawful lower witness for the residual position determined by public leaf \(h\).

The residual witness may depend on the public leaf history \(h\). It may not depend on the hidden world inside that leaf.

If terminal utility has not been decomposed additively, omit \(g\) and let \(L_h\) bound the complete terminal utility from the leaf state, including the public score ledger already accumulated.

### Theorem schema 13.1 — Fixed-field plan value

Against a fixed stochastic or deterministic field policy, suppose that whenever guard \(G\) holds:

1. \(\pi\) is legal and information-consistent through \(\tau\);
2. every reached leaf is publicly identified by \(h\);
3. \(L_h\) is a valid lower bound from that public residual state; and
4. the payoff decomposition used by \(g(h)+L_h\) is valid.

Then

\[
\boxed{
L(P)
=
\mathbb E
\left[
g(H_\tau)+L_{H_\tau}\right]
}
\]

is a lawful lower witness for the root action prescribed by \(\pi\).

#### Proof

On every reached public leaf \(h\), the continuation policy certified by \(L_h\) is lawful and guarantees at least \(L_h\). The partial policy \(\pi\) and residual policies are pasted only along publicly distinguishable leaves, so the composite policy remains information-consistent. Taking expectation over the field and hidden-world distribution yields the stated lower bound. \(\square\)

For an adversarial-field theorem, replace the field expectation at adversarial choice nodes by the corresponding minimum and prove the plan against every legal field response.

### 13.2 Composition operations

The plan language supports the following lawful operations.

#### Sequential composition

Execute one forcing or control phase, stop at a public residual state, and invoke another plan certificate.

#### Public case split

Choose different residual plans after observations the focal player actually receives. A branch on hidden ownership or an unobserved world feature is forbidden.

#### Fixed-field or chance fold

Average over field responses or chance outcomes under the declared evaluation contract.

#### Adversarial fold

Take the minimum over legal field responses when the theorem promises a guarantee independent of field behavior.

#### Candidate maximum

Compare completed lawful plan values outside the fixed-policy evaluator. Do not select a plan separately in each hidden world; that would reintroduce strategy fusion.

### 13.3 Drawing hands as the terminal case

A drawing-hand certificate is the special case in which every public leaf is terminal and the plan guarantees all remaining tricks. In a seven-trick count-free contract, the terminal component is

\[
g=7.
\]

A partial laydown can be more general:

- draw \(r\) trumps;
- retain specified entries;
- secure the next \(k\) tricks;
- force one of a small family of public residual contracts; and
- price each residual contract with an existing lower witness.

This is the proposed hierarchy behind statements of the form “guarantee \(k\), then price the remainder.”

### 13.4 Straight-count extension

For Straight Texas 42, a residual state may need to carry an outcome vector such as

\[
(\text{tricks secured},\text{count secured},\text{contract status},\text{lead/control state}).
\]

The plan-composition theorem survives if this vector is sufficient to evaluate or lower-bound the final utility. However, a count-free plan theorem does not automatically prove its count-decorated analogue. Count tiles, contract thresholds, and make/set scoring can change both the ordering of plans and the value of residual leaves.

### 13.5 A likely target: traditional partial-laydown knowledge

Traditional judgments such as a strong “34 on two trumps” hand plausibly have a compositional form rather than a monolithic expected-value proof:

- a forcing sequence;
- protected count captures;
- one or more preserved entries;
- a lead-transfer condition; and
- a residual suffix with a small certified value range.

This note does not yet prove that characterization. It supplies a lawful language in which such a theorem can be stated and tested.

---

## 14. The combined decision certificate

A complete root-action proof can now be expressed as a two-sided sandwich.

### 14.1 Candidate side

For candidate action \(a^\star\), construct a lawful plan certificate with value

\[
L_{a^\star}
\le
Q^H(a^\star).
\]

### 14.2 Competitor side

For each competitor \(a\ne a^\star\):

1. compute or bound the action-conditioned clairvoyant value \(U_a^C\);
2. prove an unavoidable fusion tax \(\underline\Gamma_a\); and
3. conclude

\[
Q^H(a)
\le
U_a^C-\underline\Gamma_a.
\]

### 14.3 Separation theorem

If, for every competitor \(a\ne a^\star\),

\[
\boxed{
L_{a^\star}
\ge
U_a^C-\underline\Gamma_a,
}
\]

then \(a^\star\) is optimal under treatment \(H\). Strict inequalities give a unique optimal action, subject to the payoff and tie conventions of the project.

The proof object need not describe the complete lawful value function. It may consist only of:

- one compact candidate plan;
- one compact tax certificate per live competitor; and
- exact counting or symbolic verification of the events used by those certificates.

---

## 15. Exact experiment on the four-trick corpus

The linked handoff reports four exact negative comparisons in which even the exact lawful candidate value lies below a competitor's action-conditioned treatment-\(C\) upper witness by approximately

\[
0.01724,
\qquad
0.03652,
\qquad
0.01780,
\qquad
0.14059
\]

of a trick.

Because the candidate side is exact in these comparisons, no stronger lawful primal witness can repair them. The mathematically targeted repair is to remove enough strategy-fusion value from the competitor upper witness.

### Experiment 15.1 — Compute the first exact tax

For each failed competitor root action \(a\):

1. reuse the treatment-\(C\) continuation solve;
2. emit \(q_I(\omega,b)\) for every next focal information state, positive-mass world, and legal action;
3. preserve complete tie sets;
4. compute every \(\delta_I\); and
5. form

\[
U_a^{(1)}=U_a^C-\sum_I\delta_I.
\]

The first decision question is

\[
\boxed{
L_{a^\star}\ge U_a^{(1)}\;?
}
\]

If yes, one layer of exact gluing separates the pair.

### Experiment 15.2 — Explain the tax

For each \(I\), record:

\[
\Pr(I),
\qquad
\delta_I,
\qquad
\arg\max_b\sum_\omega\mu_I(\omega)q_I(\omega,b),
\]

as well as the regret columns

\[
R_I(\omega,b).
\]

Then identify:

- the few information states contributing most of the needed shave;
- minimal qualitative fusion cores;
- the actions whose regret columns carry the tax; and
- structural predicates that explain those regret columns.

### Experiment 15.3 — Compile exact regret into symbolic events

Search for small events such as:

- spending the only safe trump;
- allowing a ruff before a boss can be cashed;
- losing the only return entry;
- choosing the wrong order between two incompatible controls;
- exhausting follower supply needed by a forcing line; or
- exposing protected count before lead can be retained.

For each common action \(b\), prove an event lower bound

\[
R_I(\omega,b)
\ge
\eta_b\mathbf1_{E_b}(\omega).
\]

Use exact weighted counting to test whether the resulting action-cover certificate still closes the comparison.

### Experiment 15.4 — Move down the ladder only as needed

If \(U_a^{(1)}\) remains too high, compute \(U_a^{(2)}\). With four tiles remaining before the root play, the full ladder has at most three future focal-decision layers:

\[
C^{(0)}
\supseteq
C^{(1)}
\supseteq
C^{(2)}
\supseteq
C^{(3)}=H.
\]

This experiment determines whether the observed fusion value is predominantly first-order or distributed across planning depth.

---

## 16. Trick-1 program

At trick 1, an exact \(q_I(\omega,b)\) table over roughly 399 million physical worlds may remain too expensive. The reduced corpus should therefore be used to discover the structural tax language before scaling.

A proposed trick-1 pipeline is:

1. **Generate candidate lower plans.** Use drawing hands, partial laydowns, forcing sequences, entry-preservation plans, and residual-state composition.
2. **Compute treatment-\(C\) competitor uppers.** Keep them action-conditioned.
3. **Select only live competitors.** Ignore actions already below the candidate witness.
4. **Choose one frontier or feature family.** Prefer the next focal decision or one small set of local information-state partitions.
5. **Construct action-conditioned simple upper functions.** Avoid world-independent hand scores.
6. **Construct action-cover regret events.** Require a proved loss for every common action.
7. **Count events exactly.** Use Scheme/Fix, orbit counting, weighted model counting, or other exact compressed representations.
8. **Accumulate only certified taxes.** Respect overlap and conditional-lawfulness requirements.
9. **Stop at separation.** Do not solve more of \(H\) than the root decision requires.
10. **Refine only unresolved actions.** Move to a deeper reveal-delay layer or a richer feature basis when necessary.

The target scale is not “compress every physical world into one global quotient.” It is “find the smallest exact proof that this competitor's apparent advantage depends on hidden-world-contingent future actions.”

---

## 17. Current laydown-catalogue report

The current branch work reports the following implementation-relative facts.

- The expanded catalogue contains exactly \(301\) laydowns per pip declaration.
- Across seven pip declarations, this gives \(2{,}107\) hand/declaration pairs.
- Exhaustive search over that catalogue found no deal in which all four hands are laydowns.
- A deal with three laydowns was exhibited.

Thus the observed maximum is three **relative to the current catalogue, generator, and exhaustive search implementation**.

This is not promoted here to a definition-independent mathematical theorem. Before promotion, preserve:

- the exact catalogue definition;
- the producing script;
- the commit hash;
- the three-laydown witness deal; and
- the exhaustive no-four certificate or independently replayable search.

The result is nevertheless strikingly aligned with the traditional memory: three laydowns and a strong fourth hand, but never four.

---

## 18. Open mathematical questions

### 18.1 How concentrated is the first tax?

Does a small number of next-decision information states account for most of \(\Delta_a^{(1)}\), or is the tax broadly distributed?

### 18.2 How small are exact fusion cores in practice?

The qualitative bound is at most the number of legal actions, hence at most seven. Are most decisive cores binary or ternary?

### 18.3 Which structural predicates span useful penalties?

Can trump control, boss ownership, entries, ruffs, follower supply, and count exposure form a small feature basis whose optimized zero-mean penalties nearly recover exact first-layer gluing?

### 18.4 Is first-order gluing usually enough?

For root-action separation, how often does \(U_a^{(1)}\) already cross the incumbent lower witness? When it does not, how much is gained by \(C^{(2)}\) and \(C^{(3)}\)?

### 18.5 Can block selection be optimized submodularly or approximately?

Exact block merge costs are available, but interactions may be nontrivial. Is there useful diminishing-return structure under restricted action or feature families?

### 18.6 What is the minimal sufficient residual state for plan composition?

Can partial laydowns hand off to a small typed residual contract involving only control, entries, bosses, follower supply, and public score—rather than a complete game state?

### 18.7 Can traditional bidding knowledge be expressed as finite plan families?

Can statements such as “34 on two trumps” be recovered as unions of compositional plan certificates with exact guards and verified residual values?

### 18.8 Can the multi-stage penalty dual be formalized cleanly?

A conditional martingale-difference penalty should dualize nonanticipativity across successive focal decisions. A full theorem would connect the reveal-delay ladder, feature penalties, and adaptive gluing in one framework.

### 18.9 Can the theorem layer be mechanized?

The fusion-gap identity, first-layer formulas, block costs, regret minorants, and one-stage penalty theorem are small enough to be formalized independently of the full 42 engine. Their hypotheses should be isolated in a proof-assistant-friendly finite model.

---

## 19. Claim ledger

| Claim | Status | What remains |
|---|---|---|
| Fusion-gap identity | Exact result | Nothing beyond the stated finite-policy assumptions |
| Local partition geometry | Exact representation | Formal action transport where information-state labels differ |
| Reveal-delay monotonicity and finite termination | Exact result | Engine mapping from game histories to focal-decision count |
| At most six layers after an opening lead | Exact combinatorial bound | None under seven-trick play |
| Exact first-layer Jensen/regret formula | Exact result | Emit the required frontier data |
| Binary tax formula | Exact result | None |
| Fusion-core witness size at most legal-action count | Exact result | None |
| Exact block merge cost | Exact result | Recompute after interacting merges |
| Glued simple-function upper theorem | Certificate schema | Prove each pointwise continuation upper bound |
| Action-cover regret theorem | Certificate schema | Prove each regret minorant and count its event mass |
| One-stage zero-mean tax theorem | Exact result | Choose a useful tractable penalty family |
| Multi-stage martingale penalty method | Research proposal | Write and prove the conditional induction theorem |
| Compositional plan calculus | Certificate schema | Specify payoff state, guards, stopping times, and residual witnesses |
| First-layer gluing closes the four tight failures | Open experiment | Compute \(U_a^{(1)}\) exactly |
| Small structural events reproduce the exact tax | Open experiment | Mine and prove event certificates |
| 301 laydowns per declaration; max three in a deal | Implementation-relative report | Pin code, commit, witnesses, and independent replay |

---

## 20. Conclusion

The treatment-\(C\) upper witness is not one mysterious giant object that must be replaced by a complete treatment-\(H\) solve.

It is an optimistic value created by violating local nonanticipativity. Its excess over \(H\) is exactly the smallest expected clairvoyance regret paid by a lawful policy. That excess can be removed in a finite sequence:

\[
\boxed{
\text{world revealed now}
\longrightarrow
\text{hidden for one focal decision}
\longrightarrow
\cdots
\longrightarrow
\text{hidden throughout}.
}
\]

After an opening lead, there are at most six such focal-decision layers. The first layer already has:

- an exact average-of-maxima minus maximum-of-averages formula;
- an exact regret interpretation;
- a complete optimal-face criterion;
- qualitative conflict witnesses involving at most seven worlds;
- exact block-merge prices;
- action-conditioned simple-function upper certificates;
- action-cover regret certificates; and
- a zero-mean dual-penalty representation.

On the lower side, a lawful plan need not solve every continuation. It may secure a forcing phase, branch only on public observations, and hand a publicly typed residual position to another certificate.

The resulting proof architecture is:

\[
\boxed{
\text{one compact lawful plan}
\quad+
\quad
\text{one compact unavoidable information tax per competitor}.
}
\]

The full state space may remain enormous. The decision proof need not be.

> **The truth can remain high-dimensional while both sides of the decision proof—the plan and the unavoidable information tax—stay small.**
