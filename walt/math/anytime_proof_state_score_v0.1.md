# Anytime Proof-State Walt

## Count-aware score bounds, certified regret, laydown semantics, and iterative refinement

**Status:** exploratory mathematical design for intake and adversarial review. This document does not promote any implementation, probe, or mathematical claim merely by restating it.

**Date:** 2026-08-31

**Engineering basis inspected:** `jasonyandell/texas-42` `main` at commit `25b40d9f4243e4c7941d4324ed9859c68b943843`.

**Primary mathematical parents:**

- *The Mathematics of Walt v0.1*
- *Counted Belief Sandwiches and the Refinement Calculus for Walt v0.1*
- *Counted Residual Bellman Calculus for Walt v0.1*
- the maintained project rulings and exact parity gates governing the corresponding implementations

**Scope:** finite partnership trick-taking games with hidden hands, public play, perfect recall, a fixed modeled field, and a bounded team score. Texas 42 is the first concrete instance. The proof-state calculus, score-threshold projection, regret bound, and most refinement machinery are game-independent. The 42-specific specialization is the 42-point total, five count dominoes, seven trick points, declaration/trump semantics, and the `pmake` contract objective.

**Vocabulary fence:** undecided is not incorrect; a heuristic is not a bound; an exact score envelope is not necessarily an executable policy; a threshold-wise optimum need not be one policy; action uncertainty is not score uncertainty; score uncertainty is not contract uncertainty; a model-relative certain make is not automatically a laydown; a new orchestration core is not permission to duplicate the rules, belief kernel, field model, or exact authorities.

---

# 0. Executive ruling

The current counted-belief program has reached a natural architectural boundary.

Slices C through G established, with independent parity gates, that Walt can:

1. represent a belief by seat-local hand factors instead of complete hidden deals;
2. propagate that representation exactly through public observations;
3. evaluate a fixed focal policy exactly;
4. optimize exactly inside a lawful information-state grammar;
5. instrument field-action classes by witness-guided consequence refinement;
6. maintain typed root-action intervals and safely exclude actions;
7. settle affordable roots exactly or with declared risk;
8. return a fully honest unresolved set when the available work does not settle the root.

Slice F also established the important negative: the sampled modeled field's final action-exact tail can fragment all the way to singleton hands even after most posterior mass has become action-exact. Slice G then correctly refused the consequence census because its branch-level intervals had no producer connecting them to a root-value interval.

The next improvement is not to force the field-action abstraction to completion.

It is to change the primary mathematical object from a one-shot answer to a persistent proof state, and to project unresolved behavior through the score before projecting it through the contract.

The central chain is

\[
\boxed{
\text{behavior uncertainty}
\longrightarrow
\text{score uncertainty}
\longrightarrow
\text{contract uncertainty}.
}
\]

Only the last quantity controls `pmake`.

The solver should therefore begin with the largest sound undecided state, apply free consequences, and monotonically refine only the policy regions, belief cells, score envelopes, or probabilistic bounds capable of changing one of its declared goals.

The primary finite-budget output is not merely a tile. It is:

- a sound surviving action set;
- a recommended executable policy;
- a certified lower bound on that policy's `pmake`;
- a valid upper bound on the unknown best response;
- a certified `pmake` regret;
- a score profile or score envelope;
- the exact mass still capable of crossing the contract;
- a risk ledger;
- and a resumable frontier of useful next refinements.

The central recommendation is:

> **Build a fresh proof-state orchestration core, while retaining the current Walt as the exact reference, producer library, parity oracle, and playable fallback. Do not rewrite or copy the rules, kernel, factor belief, field identities, evidence engines, or exact evaluators.**

This is a bounded greenfield core, not another independent Walt.

The present `refine_root` is a successful first controller. It is intentionally one-shot, root-local, scalar-`pmake`, closed over a small work-item enum, and not designed as a persistent theorem state. Retrofitting score profiles, policy-region graphs, residual Bellman objects, multiple solve goals, dependency-aware closure, resumability, and proof provenance directly into that successful reference risks turning it into the very patch stack this project has repeatedly avoided.

A new orchestration core can instead consume the current authorities through adapters and earn parity one theorem and one root at a time. The current implementation remains untouched except for narrow reusable APIs and independently justified fixes. The new core is promoted only after it reproduces the old one wherever their scopes overlap and demonstrates the new capabilities where the old scope ends.

---

# Part I — The score is the object beneath `pmake`

## 1. Finite score game

Fix:

- a finite physical world set \(\Omega\);
- a belief \(\beta\) on \(\Omega\);
- a focal seat \(m\);
- a fixed field \(\sigma\) for every non-focal seat;
- a lawful information-consistent focal policy \(\rho\);
- a terminal team score
  \[
  S_\rho^\sigma(\omega)\in\{0,1,\ldots,P\};
  \]
- a contract threshold \(c\in\{0,\ldots,P\}\).

For Texas 42,

\[
P=42.
\]

The contract utility is

\[
u_c(\rho,\omega)
=
\mathbf 1\{S_\rho^\sigma(\omega)\ge c\},
\]

and the policy's make probability is

\[
V_c(\rho)
=
\mathbb E_\beta[u_c(\rho,\omega)].
\]

For legal root action \(a\), let \(\Pi_a\) be the complete finite class of lawful focal policies that play \(a\) at the root. The fixed-field root value is

\[
Q_a(c)
=
\max_{\rho\in\Pi_a}V_c(\rho).
\]

`pmake` is therefore one threshold projection of a richer score-valued process.

## 2. Exact score profile

For a fixed policy \(\rho\), define its unnormalized score mass profile

\[
H_\rho(s)
=
\sum_{\omega\in\Omega}
w(\omega)
\mathbf 1\{S_\rho^\sigma(\omega)=s\},
\qquad
s=0,\ldots,P,
\]

where \(w(\omega)\) is the exact integer or rational world weight and

\[
Z=\sum_{\omega}w(\omega)
\]

is the total belief mass.

Then

\[
\sum_{s=0}^{P}H_\rho(s)=Z.
\]

The tail mass at threshold \(k\) is

\[
T_\rho(k)
=
\sum_{s=k}^{P}H_\rho(s),
\]

and

\[
V_k(\rho)=\frac{T_\rho(k)}{Z}.
\]

The entire bid-threshold curve is therefore present in one \(P+1\)-entry exact table.

For 42 this is only 43 score bins.

## 3. Tail-sum identity

For any integer-valued score \(S\in\{0,\ldots,P\}\),

\[
S
=
\sum_{k=1}^{P}\mathbf 1\{S\ge k\}.
\]

Taking expectations gives

\[
\boxed{
\mathbb E[S]
=
\sum_{k=1}^{P}\Pr(S\ge k).
}
\]

In unnormalized mass form,

\[
\boxed{
\sum_{s=0}^{P}sH(s)
=
\sum_{k=1}^{P}T(k).
}
\]

This identity gives a precise relation between expected points and the family of contract probabilities. Expected points are the area under the score-tail curve. They remain a secondary objective; they do not replace the actual contract threshold.

## 4. The Texas 42 score signature

The declaring team's final score decomposes as

\[
\boxed{
S
=
\tau
+
5(n_{50}+n_{41}+n_{32})
+
10(n_{64}+n_{55}),
}
\]

where:

- \(\tau\in\{0,\ldots,7\}\) is the number of tricks won;
- each \(n_d\in\{0,1\}\) records whether the declaring team captured count domino \(d\);
- the five count dominoes are \(5\!:\!0,4\!:\!1,3\!:\!2,6\!:\!4,5\!:\!5\).

Thus a terminal count-aware signature is

\[
(\tau,M)
\in
\{0,\ldots,7\}\times\{0,1\}^{5},
\]

only \(8\cdot32=256\) raw signatures.

A score-only evaluator may project immediately to 43 bins. A structural or explanatory evaluator may retain the 256 count-aware bins and project to score later. The latter remembers which particular ten-count or five-count moved the score and can therefore explain the bound.

A still smaller terminal-only signature is

\[
(\tau,n_5,n_{10})
\in
\{0,\ldots,7\}\times\{0,\ldots,3\}\times\{0,\ldots,2\},
\]

96 raw signatures, but it erases count-tile identity. That erasure is acceptable only after future legality and signaling no longer depend on which count tile was involved.

## 5. Current-state score envelope

Let \(b\) be the declaring team's banked points and let \(r\) be the total unbanked point mass remaining in the hand. Before any further analysis,

\[
\boxed{
b
\le
S
\le
b+r.
}
\]

This is already enough to recover the monotone decided cutoff:

- if \(b\ge c\), every continuation makes;
- if \(b+r<c\), every continuation fails.

The coarsest correct proof state is therefore not devoid of information. It begins with the complete policy class, the complete belief support, and an exact arithmetic score envelope.

---

# Part II — Contract projection of score uncertainty

## 6. Exact-mass score cells

Let \(\mathcal C\) be a finite exact partition of the belief support. Each cell \(C\in\mathcal C\) has exact mass

\[
\mu(C)
=
\sum_{\omega\in C}w(\omega).
\]

For a fixed executable policy \(\rho\), suppose Walt has proved a score envelope

\[
\ell_C
\le
S_\rho^\sigma(\omega)
\le
u_C
\qquad
(\omega\in C).
\]

Then the cell's make indicator is bounded by

\[
\mathbf1\{\ell_C\ge c\}
\le
\mathbf1\{S_\rho^\sigma(\omega)\ge c\}
\le
\mathbf1\{u_C\ge c\}.
\]

Summing exact masses gives

\[
\boxed{
L_c(\rho)
=
\frac1Z
\sum_C\mu(C)\mathbf1\{\ell_C\ge c\}
\le
V_c(\rho)
\le
\frac1Z
\sum_C\mu(C)\mathbf1\{u_C\ge c\}
=
U_c(\rho).
}
\]

## 7. Contract-sensitive residual mass

The make-interval width is exactly

\[
\boxed{
U_c(\rho)-L_c(\rho)
=
\frac1Z
\sum_C
\mu(C)\mathbf1\{\ell_C<c\le u_C\}.
}
\]

Define

\[
\boxed{
W_\rho(c)
=
\frac1Z
\sum_C
\mu(C)\mathbf1\{\ell_C<c\le u_C\}.
}
\]

This is the **contract-sensitive residual mass**.

It is the exact mass on which the current score uncertainty can still change `pmake`.

Three states that all look "unresolved" at the action layer are mathematically different:

1. **action-ambiguous, score-exact:** several field actions remain possible, but they all induce the same score;
2. **score-ambiguous, contract-stable:** the score varies, but every possibility lies on the same side of \(c\);
3. **contract-sensitive:** the score envelope straddles \(c\).

Only the third requires more work for the present `pmake` decision.

## 8. Integrated score width

Define the normalized aggregate score-envelope width

\[
\boxed{
P_\rho
=
\frac1Z
\sum_C\mu(C)(u_C-\ell_C).
}
\]

For each threshold \(k\), define

\[
W_\rho(k)
=
\frac1Z
\sum_C
\mu(C)\mathbf1\{\ell_C<k\le u_C\}.
\]

Because scores are integral,

\[
u_C-\ell_C
=
\sum_{k=1}^{P}\mathbf1\{\ell_C<k\le u_C\}.
\]

Therefore

\[
\boxed{
P_\rho
=
\sum_{k=1}^{P}W_\rho(k).
}
\]

The aggregate point width is the area under the unresolved-threshold curve.

This makes the distinction precise:

- \(P_\rho\) measures total score uncertainty across every possible contract;
- \(W_\rho(c)\) measures uncertainty relevant to the actual contract;
- a refinement can reduce \(P_\rho\) greatly while leaving \(W_\rho(c)\) unchanged;
- a small refinement involving one count tile can collapse \(W_\rho(c)\) even when most action ambiguity remains.

## 9. Monotone score refinement

Suppose one cell \(C\) is replaced by exact subcells \(C_1,\ldots,C_m\) with

\[
C=\dot\bigcup_jC_j,
\qquad
\mu(C)=\sum_j\mu(C_j),
\]

and sound narrower envelopes

\[
[\ell_j,u_j]\subseteq[\ell_C,u_C].
\]

Then:

\[
L_c'(\rho)\ge L_c(\rho),
\qquad
U_c'(\rho)\le U_c(\rho),
\]

\[
W_\rho'(c)\le W_\rho(c),
\qquad
P_\rho'\le P_\rho.
\]

Thus count-aware score refinement is an anytime-valid narrowing process. It need not make the field action exact.

---

# Part III — Point gain, rescue bands, and fragile makes

## 10. Uniform positive score-gain bound

Fix an executable incumbent policy \(\pi\). Let \(\mathcal R\) be a residual policy region. Suppose Walt proves

\[
S_\rho^\sigma(\omega)
\le
S_\pi^\sigma(\omega)+d^+
\qquad
(\rho\in\mathcal R,\ \omega\in\Omega).
\]

Then

\[
\{S_\rho^\sigma\ge c\}
\subseteq
\{S_\pi^\sigma\ge c-d^+\}.
\]

Hence

\[
\boxed{
\sup_{\rho\in\mathcal R}V_c(\rho)
\le
V_{c-d^+}(\pi).
}
\]

The maximum `pmake` improvement over \(\pi\) is bounded by

\[
\boxed{
\sup_{\rho\in\mathcal R}
\bigl(V_c(\rho)-V_c(\pi)\bigr)
\le
V_{c-d^+}(\pi)-V_c(\pi)
=
\Pr(c-d^+\le S_\pi<c).
}
\]

Define the **rescue band**

\[
\boxed{
R_\pi(c;d^+)
=
\Pr(c-d^+\le S_\pi<c).
}
\]

Only worlds where the incumbent misses by at most \(d^+\) can be rescued by that residual region.

## 11. Uniform negative score-loss bound

Suppose instead that a perturbation, alternate field, or policy approximation can lower the incumbent's score by at most \(d^-\):

\[
S_{\widetilde\pi}^{\widetilde\sigma}(\omega)
\ge
S_\pi^\sigma(\omega)-d^-.
\]

Then every world with

\[
S_\pi^\sigma(\omega)\ge c+d^-
\]

remains a make. Therefore

\[
\boxed{
V_c(\widetilde\pi,\widetilde\sigma)
\ge
V_{c+d^-}(\pi,\sigma).
}
\]

The possible make loss is bounded by the **fragile-make band**

\[
\boxed{
F_\pi(c;d^-)
=
\Pr(c\le S_\pi<c+d^-).
}
\]

So a two-sided score perturbation

\[
-d^-\le
S_{\rho}^{\sigma'}-S_\pi^\sigma
\le d^+
\]

implies

\[
\boxed{
V_{c+d^-}(\pi)
\le
V_c(\rho,\sigma')
\le
V_{c-d^+}(\pi).
}
\]

## 12. Cellwise gain bounds

A global \(d^+\) can be unnecessarily loose. Let \(\mathcal C\) be an exact partition and suppose

\[
S_\rho^\sigma(\omega)
\le
S_\pi^\sigma(\omega)+d_C^+
\qquad
(\omega\in C,\ \rho\in\mathcal R).
\]

Then

\[
\boxed{
\sup_{\rho\in\mathcal R}V_c(\rho)
\le
\frac1Z
\sum_C
\mu(C)
\mathbf1\{S_\pi^\sigma(\omega)\ge c-d_C^+\text{ on the relevant subcell}\}.
}
\]

When the incumbent score is exact per world or exact subcell, the possible improvement is bounded by

\[
\boxed{
\frac1Z
\sum_C
\mu\{\omega\in C:
c-d_C^+
\le
S_\pi^\sigma(\omega)
<
c
\}.
}
\]

When only an incumbent envelope \([\ell_C,u_C]\) is known, a safe residual upper is

\[
\boxed{
U_{\mathcal R}(c)
=
\frac1Z
\sum_C
\mu(C)
\mathbf1\{u_C+d_C^+\ge c\}.
}
\]

Together with the incumbent make lower,

\[
L_\pi(c)
=
\frac1Z
\sum_C\mu(C)\mathbf1\{\ell_C\ge c\},
\]

this gives the safe regret upper

\[
\boxed{
U_{\mathcal R}(c)-L_\pi(c)
\le
\frac1Z
\sum_C
\mu(C)
\left(
\mathbf1\{u_C+d_C^+\ge c\}
-
\mathbf1\{\ell_C\ge c\}
\right).
}
\]

## 13. Count-threat score caps

For 42, a coarse but mechanically useful score-gain witness can be represented by

\[
\Delta_C^+
=
r_C
+
5|K_{5,C}|
+
10|K_{10,C}|,
\]

where:

- \(r_C\) is an upper bound on additional trick-point swing;
- \(K_{5,C}\) is a set of five-count dominoes whose ownership may still be changed by residual behavior;
- \(K_{10,C}\) is the analogous ten-count set.

If every residual policy's improvement over the incumbent is confined to those trick and count resources, then

\[
d_C^+\le\Delta_C^+.
\]

This need not describe the exact residual policy. It need only upper-bound how many declaring-team points it can gain in the cell.

The resulting object is a valid `CountThreatCover`:

```text
CountThreatCover {
    policy_region,
    belief_cell,
    trick_gain_upper,
    five_count_tiles,
    ten_count_tiles,
    score_gain_upper,
    exact_cell_mass,
    proof,
}
```

Its value to the root is determined by whether the incumbent's score in the cell lies inside the corresponding rescue band.

## 14. Positive-part bound

For two fixed policies, define

\[
D^+(\omega)
=
\bigl(S_\rho(\omega)-S_\pi(\omega)\bigr)_+.
\]

Whenever \(\rho\) turns a miss into a make, it must gain at least one integral point. Therefore

\[
\mathbf1\{S_\rho\ge c,\ S_\pi<c\}
\le
D^+.
\]

Taking expectations,

\[
\boxed{
\Pr(S_\rho\ge c,\ S_\pi<c)
\le
\mathbb E[D^+].
}
\]

This is usually weaker than a rescue-band bound, but it can be produced from expected count-threat mass when a uniform point cap is unavailable.

---

# Part IV — Bids near 42 and laydown semantics

## 15. Loss from perfection

Let the contract be

\[
c=P-N.
\]

Suppose a policy or policy region has a proven score floor

\[
S(\omega)\ge P-d
\]

throughout its declared domain. Then

\[
d\le N
\quad\Longrightarrow\quad
S(\omega)\ge P-N=c.
\]

Thus

\[
\boxed{
\text{maximum loss from perfection}\le P-c
\quad\Longrightarrow\quad
\mathrm{pmake}=1.
}
\]

For 42,

\[
\boxed{
S(\omega)\ge42-d,\quad d\le42-c
\quad\Longrightarrow\quad
\mathrm{pmake}=1.
}
\]

The inclusive inequality is correct because making exactly the bid succeeds.

A narrow unanchored score interval is not enough. The useful object is a lower score endpoint or an upper bound on loss from 42.

## 16. Typed laydown hierarchy

The word `laydown` carries strong game meaning and should not be attached to a merely model-relative result. Distinguish the quantifiers.

### 16.1 Fixed-field policy certainty

\[
\operatorname{PolicyCertainMake}(\pi,\sigma)
\iff
\forall\omega\in\Omega:
S_\pi^\sigma(\omega)\ge c.
\]

This is exact `pmake = 1` for one policy against one declared field.

### 16.2 Adversarially robust policy

\[
\operatorname{AdversarialPolicyMake}(\pi)
\iff
\forall\omega\in\Omega
\ \forall\sigma\in\Sigma_{\mathrm{legal}}:
S_\pi^\sigma(\omega)\ge c.
\]

One fixed focal policy survives every compatible world and every legal defense.

### 16.3 Strategic forced make

\[
\operatorname{ForcedMake}
\iff
\exists\pi\
\forall\omega\
\forall\sigma:
S_\pi^\sigma(\omega)\ge c.
\]

The focal partnership has a strategy that forces the contract against adversarial defense.

### 16.4 Universal laydown

Following the strongest user-supplied sense, reserve bare `Laydown` for

\[
\boxed{
\operatorname{Laydown}
\iff
\forall\omega
\ \forall\pi\in\Pi_{\mathrm{legal}}
\ \forall\sigma\in\Sigma_{\mathrm{legal}}:
S_\pi^\sigma(\omega)\ge c.
}
\]

Every legal continuation makes. The hand can be exposed without depending on any strategic choice.

A seven-trump 42 hand is the canonical shape to test, but the implementation must prove the universal property from the actual rules and state rather than from the phrase.

These four notions should have distinct result types. None should be inferred from sampled `pmake = 1`.

## 17. Laydown as a zero-cost or structural closure

A laydown producer is a special score-floor producer. It proves

\[
\inf_{\omega,\pi,\sigma}S_\pi^\sigma(\omega)\ge c
\]

over the relevant universal domain.

Once such a fact enters the proof state, `pmake` closes immediately without solving the ordinary best-response problem. This is a good example of why the proof state should run a free logical closure after every new fact.

---

# Part V — Score-profile Bellman calculus

## 18. Fixed-policy score recursion

Let \(B\) be a factor belief with total mass \(Z(B)\). Define an unnormalized terminal score profile

\[
H_\pi(B;s)
\]

for a fixed focal policy \(\pi\).

### Decided or terminal node

If every continuation represented by \(B\) has final declaring score \(s_0\), then

\[
H_\pi(B;s)
=
\begin{cases}
Z(B),&s=s_0,\\
0,&\text{otherwise}.
\end{cases}
\]

A monotone decided node may use a score interval rather than an exact score when the exact final score is not needed. For full profile parity, continue or attach a profile envelope.

### Focal node

If \(\pi\) chooses public action \(a=\pi(I_B)\), then

\[
H_\pi(B;\cdot)=H_\pi(Ba;\cdot).
\]

### Hidden field node

Let \(B_t\) be the exact posterior branch for public action \(t\). Since the branches partition the represented world mass,

\[
\boxed{
H_\pi(B;s)
=
\sum_tH_\pi(B_t;s).
}
\]

Mass conservation follows:

\[
\sum_sH_\pi(B;s)=Z(B).
\]

## 19. Contract-specific grammar or full response

For a grammar \(G\) and fixed contract \(c\), the hidden-node rule remains profile addition.

At a focal node, each legal grammar action \(a\in G(I_B)\) produces a complete child profile. Choose one action maximizing the tail mass

\[
T_a(c)=\sum_{s\ge c}H^G(Ba;s),
\]

using a declared deterministic tie rule, and carry the entire profile of that one chosen child:

\[
\boxed{
H^G_c(B;\cdot)
=
H^G_c(Ba^\star;\cdot).
}
\]

This constructs one lawful grammar-optimal policy for the declared contract and retains its complete score profile.

The unrestricted response uses the same rule with all legal focal actions.

## 20. Do not maximize score bins independently

The pointwise maximum of two score tails is a valid upper envelope, but it need not be the score profile of any executable policy.

Example:

- policy \(A\) scores 42 on half the mass and 0 on half;
- policy \(B\) scores 21 on all the mass.

Both have expected score 21.

The threshold-wise maximum uses \(B\) for thresholds \(1,\ldots,21\) and \(A\) for thresholds \(22,\ldots,42\). Its tail-sum "expected score" is

\[
21+\frac{21}{2}=31.5,
\]

which neither policy attains.

Therefore:

> **A threshold-wise upper family is a bound object, not an executable score profile.**

For an executable recommendation, Walt must retain the profile of one actual policy selected by one lawful objective and tie rule.

## 21. Lexicographic secondary preference

When several focal actions tie exactly on `pmake` at contract \(c\), Walt may use a declared secondary order without changing the primary theorem.

One conservative order is:

1. maximize \(T(c)\);
2. maximize \(T(c+1)\);
3. maximize \(T(c+2)\);
4. continue upward;
5. then maximize expected score;
6. then use canonical tile order.

This selects, among exact `pmake` ties, the policy with the strongest score margin above the contract.

The choice is still one action at the information state. It is not a componentwise mixture.

## 22. Interval score recursion over unresolved field classes

Suppose a hidden node has:

- exact certified public-action branches \(t\), with profile bounds
  \[
  \underline H_t(s)\le H_t(s)\le\overline H_t(s);
  \]
- unresolved mass \(R\) whose public action and continuation remain unknown.

Then a sound score-profile envelope is

\[
\boxed{
\underline H(s)
=
\sum_t\underline H_t(s),
}
\]

\[
\boxed{
\overline T(k)
=
\sum_t\overline T_t(k)+R,
}
\]

where the unresolved mass may contribute to any score tail in the worst case.

At the actual contract,

\[
\underline T(c)
\le
T(c)
\le
\sum_t\overline T_t(c)+R.
\]

If the unresolved class carries a score interval \([\ell_R,u_R]\), its contribution can be sharper:

- add all of \(R\) to lower tails \(k\le\ell_R\);
- add all of \(R\) only to upper tails \(k\le u_R\);
- add nothing above \(u_R\).

This is the count-aware form of the residual Bellman bridge.

## 23. Merge before focal maximization

If several hidden hand cells produce the same public action, they must be merged before a later focal maximization.

The focal player observes the public action, not the hidden class identity. Optimizing each same-action class independently would let the continuation react to information it never receives and would reintroduce strategy fusion.

The lawful order is:

1. union every cell producing public action \(t\);
2. sum their factor weights into one posterior branch \(B_t\);
3. run one focal continuation decision on \(B_t\).

This requirement applies equally to exact profiles, interval profiles, residual Bellman values, and count-threat bounds.

---

# Part VI — The proof state is the solver

## 24. Concrete semantics and abstraction

Let \(\Theta\) denote the exact mathematical semantics of the declared root:

- exact physical belief;
- exact field;
- complete information-consistent policy classes;
- exact score outcomes;
- exact root values;
- exact optimal-policy set.

A proof state \(X\) denotes a set

\[
\gamma(X)
\]

of exact semantics still compatible with every fact Walt has established.

The state is **sound** when

\[
\Theta\in\gamma(X).
\]

Define refinement by

\[
X'\preceq X
\iff
\gamma(X')\subseteq\gamma(X).
\]

A valid work item is a sound transformer \(F\) satisfying

\[
\Theta\in\gamma(X)
\Longrightarrow
\Theta\in\gamma(F(X)),
\]

and

\[
F(X)\preceq X.
\]

## 25. The top proof state

At zero paid work, each legal root action \(a\) begins with

\[
0\le Q_a(c)\le1.
\]

The policy region is the complete \(\Pi_a\). The belief region is the complete fiber. The score envelope is

\[
b\le S\le b+r.
\]

The root surviving set is every legal action.

This is the largest sound answer.

It is not a solver failure. It is the initial proof state.

## 26. Zero-cost closure

Let \(\mathcal C_0\) be the operator that repeatedly applies every currently free deduction:

- already-made and already-set arithmetic;
- forced legal actions;
- empty or zero-mass support removal;
- exact factor mass conservation;
- direct interval projection to `pmake`;
- root bar and survivor recomputation;
- exact tie detection;
- laydown or certain-fail consequences of existing score floors;
- derived rescue/hazard bands from already present score profiles;
- invalidated-work removal;
- dependency-triggered bound installation.

The operator should satisfy

\[
\mathcal C_0(X)\preceq X,
\]

\[
\mathcal C_0(\mathcal C_0(X))
=
\mathcal C_0(X).
\]

Every paid work item should be followed by zero-cost closure before the scheduler evaluates the next step.

## 27. Deterministic and probabilistic proof states

For deterministic exact or structural facts, soundness is unconditional.

For sampled facts, define a validity event \(E_X\) such that

\[
\Pr(E_X)\ge1-\delta_X
\]

and

\[
E_X
\Longrightarrow
\Theta\in\gamma(X).
\]

The risk ledger bounds \(\delta_X\) across all active probabilistic facts.

A deterministic refinement of a \(\delta\)-sound state does not consume additional risk. A new sampled producer adds only its declared scoped risk. Exact recomputation may replace a sampled fact with a deterministic one, but it does not retroactively make earlier probabilistic exclusions exact unless every decisive comparison has been re-established deterministically.

## 28. Decision exactness is projection exactness

The full semantics \(\Theta\) need not be identified before the action is.

Let

\[
\operatorname{Opt}(\theta)
\]

be the optimal root-action set under exact semantics \(\theta\).

The proof state settles a unique action \(a^\star\) when

\[
\forall\theta\in\gamma(X):
\operatorname{Opt}(\theta)=\{a^\star\}.
\]

It settles an exact tie set \(A^\star\) when

\[
\forall\theta\in\gamma(X):
\operatorname{Opt}(\theta)=A^\star.
\]

The rest of the score distributions, policies, or hidden worlds may remain unresolved.

This is why correct incompleteness can be fast.

---

# Part VII — Root bounds and certified policy regret

## 29. Root intervals

For each legal root action maintain

\[
L_a\le Q_a(c)\le U_a.
\]

Define the proof bar

\[
\boxed{
B^{\mathrm{proof}}
=
\max_aL_a
}
\]

and the surviving set

\[
\boxed{
\mathcal S
=
\{a:U_a\ge B^{\mathrm{proof}}\}.
}
\]

On the joint-validity event, every exact optimum lies in \(\mathcal S\).

## 30. Proof bar versus executable bar

A lower bound may arise from:

- a fully materialized executable policy;
- an exact grammar optimum whose argmax policy has not yet been extracted;
- a structural existence proof;
- a sampled policy evaluation.

Only the first category can be played immediately.

Define

\[
\boxed{
B^{\mathrm{exec}}
=
\max_{\pi\in\mathcal E}L(\pi),
}
\]

where \(\mathcal E\) is the set of currently materialized lawful executable policies.

Always,

\[
B^{\mathrm{exec}}
\le
B^{\mathrm{proof}}
\le
Q^\star,
\]

where

\[
Q^\star=\max_aQ_a.
\]

The proof bar is used to exclude actions. The executable bar is used to recommend a policy.

A grammar lower may raise \(B^{\mathrm{proof}}\) immediately. It enters \(B^{\mathrm{exec}}\) only after an argmax policy DAG is extracted and re-priced by the fixed-policy evaluator.

## 31. Certified `pmake` regret

Let

\[
U^\star=\max_aU_a.
\]

Choose an executable policy \(\widehat\pi\) attaining \(B^{\mathrm{exec}}\). Then, on the joint-validity event,

\[
Q^\star
\le
U^\star
\]

and

\[
V_c(\widehat\pi)
\ge
B^{\mathrm{exec}}.
\]

Therefore

\[
\boxed{
0
\le
Q^\star-V_c(\widehat\pi)
\le
U^\star-B^{\mathrm{exec}}.
}
\]

Define

\[
\boxed{
\Gamma^{\mathrm{exec}}
=
U^\star-B^{\mathrm{exec}}.
}
\]

This is the recommended policy's certified `pmake` regret.

It is meaningful even when every root action remains formally alive.

At zero work,

\[
\Gamma^{\mathrm{exec}}\le1.
\]

Under monotone refinement:

- \(U^\star\) never increases;
- \(B^{\mathrm{exec}}\) never decreases;

so

\[
\Gamma^{\mathrm{exec}}
\]

never increases.

If

\[
\Gamma^{\mathrm{exec}}\le\varepsilon,
\]

the policy is certified \(\varepsilon\)-optimal under the declared field and belief.

## 32. Root-action local regret

For an executable policy \(\pi_a\) beginning with action \(a\), define

\[
\Gamma_a
=
U^\star-L(\pi_a).
\]

This is the global regret bound for actually playing that policy.

A local within-action gap,

\[
U_a-L(\pi_a),
\]

measures continuation-policy debt after committing to \(a\). It is not the same as global regret because another root action may have a larger upper.

## 33. Sound fallback recommendation

At any finite budget, recommend the executable policy maximizing the valid `pmake` lower bound.

A conservative tie order may then prefer:

1. smaller fragile-make mass near the contract;
2. higher worst-case score floor;
3. smaller contract-sensitive residual;
4. higher lower bound on expected score;
5. canonical tile order.

The primary recommendation remains the policy with the strongest established `pmake` floor. A policy with a narrow low score interval must not outrank a policy with a higher make floor merely because it is less uncertain.

The output should say:

```text
recommended policy
root action
pmake lower
global best-response upper
certified pmake regret
score floor / score ceiling
contract-sensitive residual
fragile-make band
proof class
risk scope
```

---

# Part VIII — Several different kinds of debt

## 34. Selection debt

Let \(b\) be a canonical holder of the proof bar. Define the lexicographic selection debt

\[
\boxed{
D_{\mathrm{select}}
=
\left(
|\mathcal S|-1,\;
\sum_{a\in\mathcal S\setminus\{b\}}
\max(0,U_a-B^{\mathrm{proof}})
\right).
}
\]

The first coordinate counts extra surviving actions. The second measures their excess upper mass above the bar.

Selection is complete exactly when the first coordinate is zero.

## 35. Executable regret debt

\[
\boxed{
D_{\mathrm{regret}}
=
U^\star-B^{\mathrm{exec}}.
}
\]

This can be small even while several actions survive. It directly answers how much `pmake` the current playable policy might be leaving on the table.

## 36. Contract-resolution debt

For the recommended executable policy,

\[
\boxed{
D_{\mathrm{contract}}
=
W_{\widehat\pi}(c).
}
\]

This is the exact mass whose score envelope still straddles the current bid.

## 37. Score-pricing debt

\[
\boxed{
D_{\mathrm{score}}
=
\frac1Z
\sum_C\mu(C)(u_C-\ell_C).
}
\]

This is useful for explanation, bidding experiments, and secondary scoring, even when current `pmake` is already settled.

## 38. Proof-strength debt

A decision may be \(\delta\)-settled while the project wants a deterministic exact result. That is a distinct objective:

\[
D_{\mathrm{proof}}
=
\mathbf1\{\text{a decisive fact is probabilistic}\}.
\]

Exact strengthening may be worth doing even when no numerical interval changes.

## 39. Declared solve goals

The controller should declare one primary goal:

```text
SelectAction
RecommendEpsilonPolicy(epsilon)
PriceRecommendedPolicy
ProveLaydown
StrengthenToExact
ClassifyExactTie
ExplainCountRisk
ComputeFullScoreProfile
```

A work item is useful relative to the declared goal. The same item can be irrelevant to action selection and valuable to score pricing.

This avoids mixing unlike objectives into one scalar.

---

# Part IX — Work selection and iterative refinement

## 40. Work items as proof transformers

A work item \(W\) declares:

- the proof facts it requires;
- the exact scope it may change;
- the fact or bound type it can produce;
- deterministic or probabilistic proof class;
- exact risk charge;
- forecast cost;
- an upper bound on possible goal-debt reduction;
- cache and identity dependencies.

Examples:

```text
EvaluateExecutablePolicyScoreProfile
ExtractGrammarArgmaxPolicy
TightenSampledRootUpper
RefineResidualFieldClass
PropagateResidualBellman
SplitPolicyCylinder
CountThreatCover
ProveScoreFloor
ProveLaydown
CompileFieldClass
BoundCompiledFieldExposure
EnumerateResidual
EscalateExact
```

## 41. Closure-aware usefulness

A work item may produce a fact that does not immediately change a root interval but enables a free derivation or a second producer that does.

Therefore "presently useless" must be evaluated after closure.

Let

\[
\widehat F_W
=
\mathcal C_0\circ F_W.
\]

The immediate potential of \(W\) is based on

\[
D(X)-D(\widehat F_W(X)),
\]

not merely on the raw field it writes.

For prerequisite-producing work, the scheduler may evaluate a short declared macro-plan

\[
W_1;W_2;\ldots;W_k
\]

as one candidate item. This prevents a necessary first step from being refused because it has zero standalone root-width effect.

The current consequence census is the canonical example: before residual Bellman or score projection existed, it correctly had zero root effect. Once those consumers exist, the same census can become useful through closure.

## 42. Upper bounds on possible work value

For an exact-mass score cell \(C\), no refinement can reduce current-contract uncertainty by more than

\[
\boxed{
\mu(C)\mathbf1\{\ell_C<c\le u_C\}/Z.
}
\]

No refinement can reduce aggregate score width by more than

\[
\mu(C)(u_C-\ell_C)/Z.
\]

For a challenger action \(a\), no upper-tightening work can prune it unless its total possible reduction can eventually exceed

\[
e_a=U_a-B^{\mathrm{proof}}.
\]

For a lower-witness producer, no result can raise the bar above its declared achievable ceiling.

These are safe steering bounds. They may be conservative.

## 43. Iteration is not an alternative to exact solving

A complete exact solve is one valid work item.

Thus one-shot exact solving is a degenerate refinement schedule:

1. start at the top proof state;
2. choose `EscalateExact`;
3. finish.

The iterative controller contains the exact solver. It does not replace it with a weaker method.

Whenever the predicted cost of all useful partial refinements exceeds exact escalation, the exact work item should win the schedule.

A poor forecast can waste time. It cannot weaken the proof state.

## 44. Resumability

The proof state should be serializable and content-addressed.

After interruption, Walt resumes from:

- exact factor and cell masses;
- materialized executable policies;
- policy-region frontier;
- score profiles and envelopes;
- sampled evidence counts and canonical next stream index;
- scoped risk ledger;
- exact exclusions;
- work dependency graph;
- completed fact hashes.

A resumed deterministic run under the same scheduler and budget extension should reproduce an uninterrupted run, modulo explicitly declared parallel scheduling nondeterminism that is reconstructed canonically before facts are accepted.

Changing the field, contract, utility, or policy-class identity invalidates only facts that depend on that coordinate.

A full score profile can be reused across many contract thresholds. This is one of the strongest reasons to retain the score object rather than only one `pmake` scalar.

---

# Part X — The architecture decision

## 45. What current Walt should remain

Current Walt is an extraordinary asset. It should remain the authority for:

- game rules and legality;
- public-record reconstruction;
- canonical fibers and exact counts;
- exactly uniform sampling;
- frozen policy identities and materialization;
- field identities and pure field actions;
- evidence processes and risk arithmetic;
- complete-world replay;
- bundled replay;
- factor beliefs and exact-cover contraction;
- fixed-policy, grammar, and unrestricted exact response values;
- existing F consequence classes;
- existing G root-interval results;
- parity fixtures and probes;
- the current playable fallback surfaces.

These are not rewritten.

## 46. Why a fresh orchestration core is justified

The present `solver::refine` is successful in its declared scope:

- one root per call;
- scalar `pmake` intervals;
- one closed finite work-item universe;
- local ephemeral state;
- immediate width-oriented steering;
- exact or sampled endpoint installation;
- no persistent policy-region graph;
- no score-profile algebra;
- no dependency-aware fact closure;
- no resumable proof-state serialization;
- no distinction among selection, regret, score, laydown, and proof-strength goals.

Those constraints made Slice G small, auditable, and correct. They are not defects.

The next object is different enough that forcing it into the same local loop would likely produce a growing enum, optional fields, repeated recomputation, and hidden coupling between producers and consumers.

The recommended change is a fresh **proof-state kernel**.

It may live as:

- `solver::proof_state` inside the existing crate; or
- a sibling `walt-proof` crate that depends on narrow public Walt interfaces.

The implementer should choose the dependency shape that avoids cycles and duplicated code.

It should not be called a separate player generation. The player remains Walt. This is a new orchestration backend.

## 47. The greenfield boundary

Build fresh:

- proof-state data model;
- fact/provenance graph;
- score profile and tail-envelope types;
- goal-specific debt computation;
- dependency-aware closure;
- open work-item registry;
- resumable scheduler;
- recommendation and regret report;
- migration adapters.

Reuse without copying:

- rules;
- kernels;
- factor beliefs;
- fields;
- policies;
- evidence;
- exact evaluators;
- current F/G producers.

If the new core needs a capability that current Walt owns privately, expose one narrow pure API. Do not reimplement the capability in the new core.

## 48. A single replacement path, not permanent duplication

The bounded migration is:

1. freeze the current G controller semantically as `RefineV1`;
2. build the proof-state core behind an experimental entry point;
3. adapt existing producers one at a time;
4. reproduce every V1 result where the same facts and goal are supplied;
5. add score and residual capabilities V1 cannot express;
6. shadow-run both;
7. promote the new controller only after conformance and arena gates;
8. retain V1 as a reference for a declared period;
9. retire duplicate orchestration once the new core subsumes it.

At no point should both controllers acquire independent rule or belief semantics.

## 49. Decision checkpoint before committing to the new core

A short architecture spike should prove:

- zero-budget proof state serializes and resumes;
- one existing exact lower and one sampled upper can be imported;
- closure recomputes the same survivor set as G;
- one fixed-policy score profile is accepted;
- no game rule is duplicated;
- state identity is complete;
- a producer can be added without editing a central closed enum.

If this spike requires broad duplication or awkward cyclic dependencies, keep the new types in an in-crate module instead of a sibling crate. The fresh data model remains the recommendation; the package boundary is optional.

---

# Part XI — Proposed proof-state data model

## 50. Core state

```rust
struct WaltProofState {
    semantics: SemanticsIdentity,
    goal: SolveGoal,

    root_actions: BTreeMap<Domino, RootActionProof>,
    executable_policies: PolicyWitnessRegistry,
    policy_regions: PolicyRegionGraph,
    belief_cells: BeliefCellGraph,

    score_facts: ScoreFactStore,
    general_facts: FactStore,
    dependencies: DependencyGraph,

    risk: RiskLedger,
    work_frontier: WorkFrontier,
    trace: Vec<ProofEvent>,
}
```

## 51. Semantics identity

```rust
struct SemanticsIdentity {
    root_id: RootId,
    rules_id: RulesId,
    field_id: FieldId,
    utility_id: UtilityId,
    contract: u8,
    belief_id: BeliefId,
    policy_class_id: PolicyClassId,
    score_semantics_id: ScoreSemanticsId,
}
```

Every fact names the coordinates on which it depends.

## 52. Root action proof

```rust
struct RootActionProof {
    action: Domino,

    lower_bounds: Vec<LowerFactId>,
    upper_bounds: Vec<UpperFactId>,

    installed_lower: Rational,
    installed_upper: Rational,

    executable_incumbents: Vec<PolicyId>,
    policy_region_root: PolicyRegionId,

    score_tail_envelopes: Vec<ScoreTailFactId>,
    contract_sensitive_residual: Option<Rational>,

    status: ActionStatus,
}
```

The installed lower is the maximum of all valid lower facts. The installed upper is the minimum of all valid upper facts.

## 53. Exact score profile

```rust
struct ExactScoreProfile {
    policy: PolicyId,
    field: FieldId,
    root: RootId,
    mass: u128,
    bins: [u128; 43],
}
```

For generalized games, the bin count is \(P+1\).

Optional 42-specific explanation profile:

```rust
struct CountSignatureProfile {
    policy: PolicyId,
    bins: [[u128; 32]; 8], // tricks × captured-count mask
}
```

## 54. Tail envelope

```rust
struct ScoreTailEnvelope {
    scope: SemanticScope,
    lower_tail: [Rational; 43],
    upper_tail: [Rational; 43],
    authority: BoundAuthority,
    executable_policy: Option<PolicyId>,
}
```

A tail envelope without an executable policy is never presented as a policy score profile.

## 55. Score cell fact

```rust
struct ScoreCellFact {
    policy_or_region: PolicyScope,
    belief_cell: BeliefCellId,
    exact_mass: u128,

    score_lower: u8,
    score_upper: u8,

    gain_upper: Option<u8>,
    loss_upper: Option<u8>,

    count_threat: Option<CountThreat>,
    proof: FactProof,
}
```

## 56. Proof provenance

Every installed fact should carry:

- statement type;
- exact semantic scope;
- producing authority;
- input fact IDs;
- deterministic or scoped-\(\delta\) proof class;
- stream identity if sampled;
- reproducibility parameters;
- content hash.

The proof state is a derivation DAG, not merely a bag of current numbers.

---

# Part XII — Implementation program

## 57. Phase 0 — preserve and expose the reference

Before new work:

1. pin current `main` and all C–G parity fixtures;
2. give the current G controller a stable reference entry point;
3. expose pure adapters for:
   - exact fixed-policy evaluation;
   - exact grammar evaluation;
   - exact unrestricted response;
   - root interval producers;
   - F-stage hand-cell partitions and witness records;
   - factor belief identities and masses;
4. forbid new orchestration semantics from entering those authorities.

No behavior change is required.

## 58. Phase 1 — proof-state skeleton

Build:

- semantics identity;
- top root intervals \([0,1]\);
- proof and executable bars;
- survivor set;
- fact registry;
- zero-cost closure;
- serialization;
- resume;
- exact and \(\delta\)-qualified result typing.

Gates:

- zero-budget state is sound and contains every legal action;
- importing V1's final interval facts reproduces V1's survivor/result;
- closure is idempotent;
- resume equals uninterrupted execution;
- facts with mismatched identities are rejected;
- no producer can install a lower as an upper or vice versa.

## 59. Phase 2 — fixed-policy score profile

Extend the exact fixed-policy factor recursion to return the 43-bin score profile.

Gates:

\[
\sum_sH(s)=Z,
\]

\[
\sum_{s\ge c}H(s)=M_{\mathrm{pmake}},
\]

\[
\sum_ssH(s)=\sum_{k=1}^{42}T(k),
\]

and exact parity with complete-world replay on affordable fibers.

Add the 256-bin count-signature profile only if it is useful for count-threat explanations; the 43-bin profile is sufficient for the first theorem.

## 60. Phase 3 — contract projection and certified regret

Import executable score profiles into the proof state.

Produce:

- make lower/upper at any bid;
- rescue bands;
- fragile-make bands;
- expected score;
- score floor and ceiling;
- \(B^{\mathrm{exec}}\);
- \(U^\star\);
- certified regret \(\Gamma^{\mathrm{exec}}\).

Gates:

- exact profiles project to exact current `pmake`;
- changing only the contract reuses the same profile;
- regret contains exact best-response regret on every affordable fixture;
- regret never increases under monotone refinement;
- a non-executable grammar lower cannot enter \(B^{\mathrm{exec}}\).

## 61. Phase 4 — score-aware residual Bellman

Connect F-stage unresolved field classes to root bounds.

At each hidden node:

1. merge same-public-action exact classes;
2. recurse on exact branches;
3. attach the unresolved mass with a score envelope;
4. project to contract tails;
5. propagate exact or interval profiles upward.

Gates:

- every intermediate F stage gives a root interval;
- intervals nest across the F staircase;
- endpoint equals exact factorized response;
- same-action cells are merged before focal max;
- a deliberate cellwise-max counterexample is rejected;
- `ConsequenceCensus` now has nonzero root potential exactly where closure can consume it.

## 62. Phase 5 — count-threat covers

Build the first safe, deliberately incomplete `CountThreatCover` producer.

Begin with simple resources:

- remaining trick-point swing;
- ownership of one named five-count;
- ownership of one named ten-count;
- one ruff-created count transfer;
- one highest-trump protection condition.

The producer may decline.

Its verifier checks the uniform point-gain inequality over the policy region and belief cell. Once verified, closure turns it into a rescue-band upper.

Gates:

- accepted covers never understate exact residual score gain on complete enumerable fixtures;
- declined covers produce no number;
- exact zero gain collapses the residual at that cell;
- a rare ten-count hazard remains visible even when average score movement is small.

## 63. Phase 6 — grammar argmax extraction and residual policy bounds

Extract an executable policy DAG from the exact grammar recursion under a declared tie rule. Re-price it through fixed-policy score evaluation.

Then implement the residual Bellman recurrence from the prior design:

- grammar-conforming value;
- first-deviation residual value;
- unrestricted value.

Allow score-gain and count-threat covers to upper-bound off-grammar regions.

Gates:

- extracted policy value equals the grammar optimum;
- extracted policy profile is one realizable profile;
- grammar plus residual exactly covers the full policy class;
- residual upper below grammar lower proves unrestricted closure;
- no threshold-wise profile envelope is serialized as the extracted policy.

## 64. Phase 7 — laydown producers

Implement result types:

```text
PolicyCertainMake
AdversarialPolicyMake
ForcedMake
Laydown
```

Start with structural fixtures:

- all seven trumps;
- already-made arithmetic;
- complete remaining control with all count secured;
- deliberate near-laydown counterexamples where one count or one ruff breaks universality.

No sampled route may construct any laydown type.

## 65. Phase 8 — opening-root iterative run

At one opening root:

1. create the zero-budget proof state;
2. apply zero-cost closure;
3. import sampled root bounds;
4. evaluate cheap executable policies;
5. build score profiles where affordable;
6. run partial F refinement;
7. propagate score-aware residual Bellman intervals;
8. apply count-threat covers;
9. refine the highest-debt policy region or belief cell;
10. stop at several declared budgets.

Report at each budget:

- surviving root actions;
- proof bar;
- executable bar;
- global upper;
- certified regret;
- contract-sensitive residual;
- score-width debt;
- exact F mass and unresolved F mass;
- policy cylinders;
- count-threat cells;
- risk;
- work;
- wall time;
- recommended policy;
- whether the result is exact, \(\delta\)-qualified, \(\varepsilon\)-optimal, or unresolved.

The first target is not a seven-trick exact opening solution.

The first target is a materially smaller correct survivor set or a useful certified-regret recommendation under a playable budget.

---

# Part XIII — Acceptance conditions

## 66. Mathematical acceptance

1. Every proof-state transition preserves soundness.
2. Every deterministic fact is exact or structural under its declared scope.
3. Every sampled fact carries a complete scoped risk identity.
4. Installed lowers are achievable or valid lower witnesses.
5. Installed uppers cover the entire named policy region.
6. Root survivor soundness follows from the installed intervals.
7. Certified regret follows from the global upper and executable lower.
8. Score profiles conserve mass.
9. Contract projection reproduces `pmake`.
10. Score refinements narrow make and score intervals.
11. Same-public-action cells merge before focal optimization.
12. Threshold-wise envelopes are not confused with executable profiles.
13. Laydown results use the declared universal quantifiers.
14. Budget exhaustion returns a sound proof state and never manufactures a winner.

## 67. Engineering acceptance

1. No rule, legality, winner, scoring, kernel, field, or evidence logic is copied into the new core.
2. Every existing authority is reached through one adapter.
3. Current G results reproduce when given the same producers and goal.
4. State serialization is deterministic and versioned.
5. Resume is semantically identical to uninterrupted refinement.
6. A field or contract identity mismatch invalidates dependent facts.
7. Exact and sampled facts are mechanically distinct.
8. The proof state can retain all legal actions at zero budget.
9. The recommended policy is always materialized and lawful.
10. The new core can be removed without changing current Walt.
11. Current Walt remains the parity oracle until explicit promotion.
12. The new controller does not become another permanent copy.

## 68. Probe acceptance

The first score-aware probe should contain:

- one certain make;
- one certain fail;
- one exact tie;
- one rare count rescue;
- one rare count hazard;
- one action-fragmented but contract-stable F stage;
- one action-fragmented and contract-sensitive F stage;
- one grammar lower that is not yet executable;
- one extracted grammar policy;
- one \(\varepsilon\)-optimal unresolved recommendation;
- one universal laydown fixture;
- one model-relative `pmake = 1` fixture that is not a laydown.

---

# Part XIV — Falsifiers and honest negative outcomes

## 69. Score projection fails to compress

If F's action tail fragments and the contract-sensitive score residual remains nearly as large at every useful stage, count has not produced the hoped-for compression at those roots.

That is a result, not a reason to weaken the proof types.

## 70. Count-threat covers remain vacuous

If safe point-gain caps almost always include every remaining count and trick point, rescue-band uppers will remain near one. The next response is a richer structural producer or a different bound, not an unverified count heuristic.

## 71. Executable bar remains weak

If grammar and structural lower facts rise but policy extraction is too expensive, the proof bar may improve while certified executable regret remains poor. That identifies policy materialization as the bottleneck.

## 72. Proof-state overhead dominates

If serialization, dependency closure, and fact bookkeeping cost more than the underlying small-grade solves, use the new core only above a grade/fiber threshold. Correct architecture does not require one backend to win every regime.

## 73. Residual Bellman state explosion

If partial score profiles create too many factor states, retain contract-specific scalar intervals at the current bid and compute full profiles only for executable policies or selected explanation roots.

## 74. Fresh core creates duplication

If the new package boundary forces copies of private game logic, abandon the sibling crate and place the fresh data model inside the existing crate. The clean semantic boundary matters more than repository aesthetics.

---

# Part XV — Lean program

## 75. Generic theorem layer

The following results are game-independent and appropriate for the generic Walt namespace.

### PS-T1 — top-state soundness

The initial root intervals, full policy regions, full belief support, and arithmetic score envelopes contain the true semantics.

### PS-T2 — refinement composition

Sound reductive transformers compose into a sound reductive transformer.

### PS-T3 — zero-cost closure

An idempotent sound closure preserves soundness and refinement order.

### PS-T4 — survivor theorem

Simultaneously valid root intervals contain every exact optimum in the surviving set.

### PS-T5 — certified regret

\[
Q^\star-V(\widehat\pi)
\le
U^\star-L(\widehat\pi).
\]

### PS-T6 — cell contract projection

Exact cell masses and score envelopes produce valid make bounds.

### PS-T7 — contract-sensitive width identity

\[
U_c-L_c
=
\sum_C\beta(C)\mathbf1\{\ell_C<c\le u_C\}.
\]

### PS-T8 — integrated score-width identity

\[
\sum_C\beta(C)(u_C-\ell_C)
=
\sum_{k=1}^{P}W(k).
\]

### PS-T9 — rescue-band theorem

A uniform or cellwise positive score-gain cap gives an upper bound on residual `pmake`.

### PS-T10 — fragile-make theorem

A score-loss cap gives a lower bound on perturbed `pmake`.

### PS-T11 — loss-from-perfection safety

\[
S\ge P-d,\ d\le P-c
\Longrightarrow
S\ge c.
\]

### PS-T12 — score-profile mass conservation

The fixed-policy Bellman profile sums to the factor-belief mass.

### PS-T13 — score-tail projection

The exact profile's threshold tail equals the Boolean contract value.

### PS-T14 — merge-before-max

Publicly indistinguishable hidden cells must be joined before a focal maximization; the merged recursion is information-consistent.

### PS-T15 — threshold-envelope non-realizability witness

Formalize a finite counterexample showing that pointwise maxima of policy tail profiles need not be achieved by one policy.

## 76. 42 instance layer

The Texas 42 instantiation should prove:

- total score is 42;
- score decomposes into seven trick points and the five count weights;
- the 256 count signatures project to scores in \(0,\ldots,42\);
- banked plus unbanked points gives the initial score envelope;
- the current `pmake` utility is the threshold projection;
- declared all-trump laydown fixtures satisfy the exact universal theorem;
- count-threat witnesses correctly bound possible point swing.

---

# Part XVI — Final perspective

The exact answer is no longer the only mathematically respectable object.

At zero work, Walt knows the entire lawful problem and almost none of its resolution.

At intermediate work, Walt may know:

- which actions cannot be optimal;
- which policy it can actually execute;
- how good that policy is guaranteed to be;
- how much better any unknown policy could still be;
- where the remaining score uncertainty sits;
- which count tiles can still move the contract;
- whether further refinement can change `pmake`;
- and which exact computation would buy the most proof.

At full work, the same object collapses to the exact best response.

The progression is

\[
\boxed{
\text{total undecided}
\supset
\text{bounded survivor set}
\supset
\text{certified low-regret policy}
\supset
\text{settled action}
\supset
\text{exact value and policy}.
}
\]

No stage needs to pretend it has reached the next.

The score layer gives the key economic principle:

> **Do not refine behavior merely because behavior differs. Refine the mass on which the difference can move the score across the contract.**

The proof-state layer gives the key computational principle:

> **Undecided is the default theorem object. Search is the monotone removal of possibilities, and it may stop safely after any removal.**

The architecture gives the key engineering principle:

> **Start the new orchestration model cleanly, but keep the current Walt as the mathematical backstop. Rewrite the controller state, not the truth machinery.**

That is the next route toward the desired base player: strong, cautious, interpretable, resumable, and gloriously boring in exactly the places where future refinement can act.
