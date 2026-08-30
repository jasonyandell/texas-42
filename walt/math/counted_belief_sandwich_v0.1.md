# Counted Belief Sandwiches and the Refinement Calculus for Walt

## A correctness-preserving path from sampled orientation to factorized exact best response

**Status:** exploratory mathematical design for intake and adversarial review. This document does not promote any existing probe, implementation, or mathematical claim by restating it. It extends the working framework of *The Mathematics of Walt v0.1* and is written against the engineering state inspected on `jasonyandell/texas-42` `main` at commit `a1d2219ba22270f1c789ca1892c85f93e3cd7d42`.

**Date:** 2026-08-29

**Scope:** fixed-field best response in a finite partnership trick-taking game, with Texas 42 as the first implementation. The theory is substantially game-independent. The 42-specific parts are the current-remainder fiber, follow-suit and trump semantics, count points, and the Boolean `pmake` terminal objective.

**Vocabulary:** support is not belief; feasible is not reachable; sampled is not exact; a resource cap is not a stopping theorem; a heuristic schedule is not a bound; a lower witness is not an upper witness; exact best-of-a-frozen-set is not exact best response over the complete information-consistent policy class.

**Engineering sources inspected:**

- `walt/probes/ordering/README.md`
- `walt/probes/field_cache/README.md`
- `walt/probes/bundle/README.md`
- `walt/probes/waking/README.md`
- `walt/probes/l2_controller/README.md`
- `walt/walt/src/solver/{adaptive,controller,bundle,act,field,policy,upper_cs,exposure,hazard,motif}.rs`
- `walt/walt/src/kernel/{kernel,fiber,sample}.rs`

No external literature theorem is imported as authority in this document. Where an algorithmic family is mentioned, it is a possible backend, not a receipt.

---

# 0. Executive ruling

The recent speed work has done something valuable: it has localized the expensive object.

Move ordering reduced some explored children but sorting and cache-order effects consumed most of the wall gain. Field-action caching and the monotone `pmake` cutoff helped late and small fibers but barely moved the large trick-1 route. Bundling collapsed public-tree nodes and redundant field queries, yet remained nearly linear in world-member touches and distinct field information states. The waking profile then showed that the ordinary early-grade baseline, not the richer-model escalation, dominates whole-hand cost.

The conclusion is not that exactness is hopeless. It is narrower and more useful:

> **A fundamentally faster solver must stop representing the belief as an explicit list of complete worlds and stop treating the policy class as an undifferentiated object that must be solved everywhere.**

The proposed solver has two coupled refinement axes.

1. **Policy-space refinement.** Maintain lower and upper bounds over information-consistent policy regions. Seed the lower side with fast lawful policies and small policy grammars. Cover the omitted continuations by admissible upper bounds. Split only the policy region that can still change the root decision.
2. **Belief-space refinement.** Represent the posterior as exact seat-local hand factors joined by the disjoint-deal constraint. Count action buckets and structural consequence cells without materializing complete deals. Split only the hand class or structural cell whose unresolved mass can still change the root decision.

At every finite budget the solver returns one of:

- an exact root winner;
- a declared-\(\delta\) root winner;
- an exact or declared-\(\delta\) surviving action set;
- an exact tie or declared practical-equivalence set;
- an honestly unresolved set, followed by a separately typed heuristic fallback if play must continue.

Completeness is desirable but not required for correctness at a finite budget. The same refinement system becomes complete when every relevant policy region and belief cell is eventually resolved.

There are two immediate implementation tracks.

- **Immediate bounded track:** generalize the already-implemented E3 max-preserving confidence construction from split reach to the actual root-action `pmake` objective. This closes the optimization lock probabilistically without a policy-count penalty.
- **Exact backend track:** build a one-ply exact factorized-belief contraction, then recurse only after parity and cost are understood. This attacks the explicit-world wall directly.

These tracks share the same root interval and survivor-set interface. Neither is a competing Walt.

---

# Part I — The exact target and the correct incomplete output

## 1. Fixed-field root values

Fix:

- a finite physical world set \(\Omega\);
- a rational belief \(\beta\) on \(\Omega\);
- a focal seat \(m\);
- a fixed information-pure field \(\sigma\) for every non-focal seat;
- a bounded terminal utility \(u\), Boolean in the present `pmake` application;
- the focal root information state \(I_0\).

For legal root action \(a\), let \(\Pi_a\) be the finite nonempty class of all information-consistent focal policies that play \(a\) at \(I_0\). Define

\[
V(\rho)=\mathbb E_{\omega\sim\beta}[u_\rho^\sigma(\omega)],
\]

\[
\boxed{Q_a=\max_{\rho\in\Pi_a}V(\rho).}
\]

The exact fixed-field best response is any action in

\[
\operatorname{Opt}=\operatorname*{argmax}_a Q_a.
\]

The desired solver is not required to compute every \(Q_a\) exactly before identifying an element of \(\operatorname{Opt}\).

## 2. Root sandwiches

At refinement stage \(t\), maintain for every legal root action

\[
\boxed{L_a^{(t)}\le Q_a\le U_a^{(t)}.}
\]

The bounds may be:

- exact and deterministic;
- structural and deterministic;
- declared-\(\delta\) probabilistic under a scoped risk ledger.

Define the current bar

\[
B_t=\max_a L_a^{(t)}
\]

and the admissible root set

\[
\boxed{
\mathcal S_t=\{a:U_a^{(t)}\ge B_t\}.
}
\]

### Theorem 2.1 — survivor soundness

On the event that every input interval is valid, every optimal root action lies in \(\mathcal S_t\). Therefore any action outside \(\mathcal S_t\) is safely excluded. If \(\mathcal S_t=\{a^\star\}\), then \(a^\star\) is the unique exact optimum on that event.

**Proof.** If \(a\) is optimal, then

\[
U_a^{(t)}\ge Q_a=\max_bQ_b\ge\max_bL_b^{(t)}=B_t.
\]

Hence \(a\in\mathcal S_t\). If the set is a singleton, every optimum is that singleton. ∎

### Result typing

A root solver should distinguish at least:

- `ExactRootWinner`
- `DeltaRootWinner`
- `ExactRootTie`
- `BoundedRootSet`
- `EpsilonRootSet`
- `UnresolvedRootSet`
- `HeuristicFallback`

`HeuristicFallback` may select among a valid surviving set. It must never be serialized as an exact or \(\delta\)-settled root winner.

## 3. Monotone refinement

A refinement is monotone when

\[
L_a^{(t+1)}\ge L_a^{(t)},
\qquad
U_a^{(t+1)}\le U_a^{(t)}.
\]

Then

\[
B_{t+1}\ge B_t
\]

and

\[
\mathcal S_{t+1}\subseteq\mathcal S_t.
\]

The search may pause after any refinement. Correctness does not depend on completing the refinement schedule.

---

# Part II — The optimization-lock sampling cure

## 4. Why the current controller is not yet a root best-response bound

The current calculated-evidence controller compares a finite frozen candidate set. Exact full-fiber evaluation closes the measure and response locks for those candidates. It does not close the optimization lock over every policy in \(\Pi_a\).

The missing upper bound is obtainable from the same max-preserving theorem already used by the E3 split-reach producer.

The crucial object is not the sampled value of one explored policy. It is the exact empirical optimum over the complete information-consistent policy class on one common world prefix.

## 5. The max-preserving root-action upper theorem

Let \(\omega_1,\omega_2,\ldots\) be iid draws from \(\beta\). For \(\rho\in\Pi_a\), define

\[
X_i^\rho=u_\rho^\sigma(\omega_i)\in\{0,1\},
\]

\[
S_{\rho,n}=\sum_{i=1}^nX_i^\rho.
\]

Define the empirical best-response count

\[
\boxed{
S_{a,n}^\star=\max_{\rho\in\Pi_a}S_{\rho,n}.
}
\]

Let \(U_\delta(s,n)\) be an anytime-valid upper confidence endpoint for a Bernoulli mean, nondecreasing in \(s\): for every fixed Bernoulli stream of mean \(p\),

\[
\Pr\!\left(\exists n:\ p>U_\delta(S_n,n)\right)\le\delta.
\]

### Theorem 5.1 — optimization-lock upper confidence sequence

Define

\[
\boxed{
\widehat U_{a,n}
=
\min_{1\le t\le n}U_\delta(S_{a,t}^\star,t).
}
\]

Then

\[
\boxed{
\Pr\!\left(\exists n:\ Q_a>\widehat U_{a,n}\right)\le\delta.
}
\]

There is no factor of \(|\Pi_a|\) in the risk.

**Proof.** Choose one fixed true maximizer

\[
\rho^\star\in\arg\max_{\rho\in\Pi_a}V(\rho).
\]

This policy depends on the game and belief, not on the sampled stream. For every prefix,

\[
S_{a,n}^\star\ge S_{\rho^\star,n}.
\]

Monotonicity of \(U_\delta\) gives

\[
U_\delta(S_{a,n}^\star,n)
\ge
U_\delta(S_{\rho^\star,n},n).
\]

Therefore

\[
\{Q_a>U_\delta(S_{a,n}^\star,n)\}
\subseteq
\{V(\rho^\star)>U_\delta(S_{\rho^\star,n},n)\}.
\]

Taking the union over all prefixes, or equivalently taking the running minimum of the upper endpoints, stays inside the one fixed policy's anytime undercoverage event, whose probability is at most \(\delta\). ∎

### Corollary 5.2 — optimistic empirical solvers remain valid

If an implementation returns a count \(\widetilde S_{a,n}\) satisfying

\[
\widetilde S_{a,n}\ge S_{a,n}^\star
\]

pathwise, then

\[
U_\delta(\widetilde S_{a,n},n)
\]

is still an admissible upper endpoint. A relaxation may cost tightness but not validity.

A lower approximation to \(S_{a,n}^\star\) is not safe as an upper-bound producer.

### Load-bearing hypotheses

1. The policy class \(\Pi_a\) is fixed for the evidence epoch.
2. The field and utility are fixed.
3. Every policy is evaluated on the same world at each stream index.
4. The world stream is iid from the declared belief, or satisfies the exact conditional-null hypothesis of the evidence engine.
5. The empirical optimizer computes \(S_{a,n}^\star\) exactly or an upper bound on it.
6. The one-mean upper endpoint is anytime-valid and monotone in the success count.
7. Policy selection remains outside the hidden-world expectation; \(S_{a,n}^\star\) is the value of one information-consistent policy over the whole prefix, not a per-world fused maximum.

Joint dependence among policy outcomes on a common world is harmless. It is the pathwise inequality against one fixed true maximizer that carries the proof.

## 6. Root-action lower confidence bounds

Let \(\rho_a\in\Pi_a\) be one frozen lawful policy. On an evaluation stream independent of any data used to select \(\rho_a\), let

\[
T_{a,n}=\sum_{i=1}^nu_{\rho_a}^\sigma(\omega_i).
\]

If \(L_\delta(s,n)\) is an anytime-valid Bernoulli lower endpoint, then

\[
\boxed{
\widehat L_{a,n}
=
\max_{t\le n}L_\delta(T_{a,t},t)
\le
V(\rho_a)
\le Q_a
}
\]

except on the declared \(\delta\) event.

A policy may be discovered adaptively on another stream. Conditioning on the discovery record makes the selected policy fixed; the independent evaluation stream then retains validity.

A same-stream selected argmax is not a lower witness without an additional uniform-selection argument. The clean route is discovery/evaluation separation.

## 7. The immediate root solver

For every legal action \(a\):

1. compute \(S_{a,n}^\star\) with the exact-on-prefix information-consistent solver;
2. invert it into \(\widehat U_{a,n}\) by Theorem 5.1;
3. discover one or more strong lawful policies on a separate stream;
4. freeze and evaluate them on a lower-bound stream;
5. set \(L_a\) to the best valid lower witness and \(U_a\) to the optimization-lock upper;
6. form \(\mathcal S\) by the root bar.

Risk is allocated across root actions and lower/upper claims, not across policies inside \(\Pi_a\).

This produces a declared-\(\delta\) bound on the actual fixed-field best response, not merely best-of-a-frozen-set.

## 8. What policy partitioning does and does not buy statistically

Suppose

\[
\Pi_a=\dot\bigcup_j\mathcal P_j.
\]

If each region is evaluated on the same prefix and the same monotone endpoint is applied, then

\[
\max_jU_\delta(S_{j,n}^\star,n)
=
U_\delta\!\left(\max_jS_{j,n}^\star,n\right).
\]

Therefore partitioning by itself does not tighten the direct upper bound on the union.

Partitioning becomes valuable when it allows one of the following:

- a strong lower witness inside one region;
- an exact or structural upper on the residual region only;
- different consequence covers for different regions;
- safe elimination of regions before further sampling;
- a smaller empirical optimizer after eliminated regions are removed under a fresh or preallocated evidence epoch.

The useful comparison is usually not “upper of the whole policy class” but

\[
\text{lower of the incumbent region}
\quad\text{versus}\quad
\text{upper of every residual region}.
\]

---

# Part III — Policy cylinders and small policy grammars

## 9. Partial-policy cylinders

A partial policy \(p\) is a finite map from focal information states to legal actions. Its cylinder is

\[
[p]=\{\rho:\rho\text{ is lawful and extends }p\}.
\]

Define

\[
Q(p)=\max_{\rho\in[p]}V(\rho).
\]

If focal information state \(I\notin\operatorname{dom}(p)\), then for each legal action \(x\in A(I)\), let

\[
p_x=p\cup\{I\mapsto x\}.
\]

### Theorem 9.1 — cylinder partition

\[
[p]=\dot\bigcup_{x\in A(I)}[p_x]
\]

and

\[
\boxed{Q(p)=\max_{x\in A(I)}Q(p_x).}
\]

**Proof.** Every total policy extending \(p\) assigns exactly one legal action at \(I\), so it lies in exactly one child cylinder. Maximizing over the disjoint union is the maximum of the child maxima. ∎

### Bound propagation

If every child has valid bounds

\[
L(p_x)\le Q(p_x)\le U(p_x),
\]

then

\[
\boxed{
\max_xL(p_x)
\le Q(p)
\le
\max_xU(p_x).
}
\]

Combining these with an existing parent interval gives the monotone update

\[
L'(p)=\max\!\left(L(p),\max_xL(p_x)\right),
\]

\[
U'(p)=\min\!\left(U(p),\max_xU(p_x)\right).
\]

## 10. Branch-and-bound over policy regions

Maintain a frontier of cylinders. Each cylinder carries:

- the partial policy constraints;
- a lawful completion used as a lower witness;
- an admissible upper witness for all completions;
- a proof/risk identity for each bound;
- an estimated refinement cost;
- an optional disagreement witness showing where its upper relaxation violates information consistency.

Let \(L^\star\) be the best current lower witness across all cylinders. Any cylinder with

\[
U(p)<L^\star
\]

cannot contain an optimal policy and may be discarded.

The next split should occur at an information state where:

- the upper relaxation selects different actions in worlds that share the same focal information state;
- or fast policies disagree;
- or a structural consequence class remains unresolved;
- and tightening that region can still change the root survivor set.

The split order is heuristic. The cylinder cover and bounds carry correctness.

## 11. Policy grammars

A policy grammar is a map

\[
G:I\mapsto G(I)\subseteq A(I),
\qquad G(I)\ne\varnothing.
\]

Its lawful policy class is

\[
\Pi^G=\{\rho:\rho(I)\in G(I)\text{ for every }I\}.
\]

A small set of fast policies \(\rho_1,\ldots,\rho_k\) induces the grammar

\[
G(I)=\{\rho_1(I),\ldots,\rho_k(I)\}.
\]

The grammar may legally combine the source policies by information state. It does not choose by hidden world.

### Important non-theorem

The pointwise union

\[
\omega\mapsto\max_i u_{\rho_i}(\omega)
\]

is not generally a lawful policy. It is a world-informed relaxation.

“Two mostly disjoint policies” therefore provide:

- two lower witnesses;
- a small lawful grammar;
- a source of policy-cylinder split order.

They do not provide theoretical coverage of omitted policies without a residual upper bound.

## 12. Grammar/residual separation

Let

\[
\Pi_a^G=\Pi_a\cap\Pi^G,
\qquad
\Pi_a^{\neg G}=\Pi_a\setminus\Pi_a^G.
\]

Write

\[
Q_a^G=\max_{\rho\in\Pi_a^G}V(\rho),
\qquad
Q_a^{\neg G}=\max_{\rho\in\Pi_a^{\neg G}}V(\rho).
\]

Then

\[
Q_a=\max(Q_a^G,Q_a^{\neg G}).
\]

If the restricted grammar solve is exact and

\[
\boxed{Q_a^G>U_a^{\neg G}\ge Q_a^{\neg G},}
\]

then an exact grammar optimizer is globally optimal within root action \(a\).

If only a lower witness \(L_a^G\) is available and

\[
L_a^G>U_a^{\neg G},
\]

then every global optimizer lies in the grammar, but optimization may remain unfinished inside the grammar.

The residual can be split into first-deviation cylinders: policies whose first off-grammar information state, under a declared canonical order, is \(I\) and whose action there is \(x\notin G(I)\). This is a disjoint policy-class cover. An implementation may discover those cylinders lazily rather than enumerate them in advance.

---

# Part IV — Counted consequence cells

## 13. Cell-wise value bounds

Let the physical fiber be partitioned exactly:

\[
\Omega=\dot\bigcup_{j=1}^JC_j,
\qquad
w_j=\beta(C_j).
\]

For one fixed policy \(\rho\), suppose a verifier proves

\[
\ell_j\le u_\rho(\omega)\le r_j
\qquad
\text{for every }\omega\in C_j.
\]

Then

\[
\boxed{
\sum_jw_j\ell_j
\le V(\rho)
\le
\sum_jw_jr_j.
}
\]

For Boolean utility, a cell may be:

- exact success: \([1,1]\);
- exact failure: \([0,0]\);
- unresolved: \([0,1]\).

If the unresolved residual has mass \(r\), its contribution to the value interval has width at most \(r\).

## 14. Pairwise cell calculus

For two fixed policies define

\[
Y(\omega)=u_a(\omega)-u_b(\omega)\in\{-1,0,1\}.
\]

If cell \(C_j\) has a proved interval

\[
\ell_j\le Y(\omega)\le r_j
\qquad(\omega\in C_j),
\]

then

\[
\boxed{
\sum_jw_j\ell_j
\le V(a)-V(b)
\le
\sum_jw_jr_j.
}
\]

Useful cell types are:

| cell type | interval for \(Y=a-b\) |
|---|---:|
| `Benefit` — \(a\) wins, \(b\) fails | \([1,1]\) |
| `Hazard` — \(a\) fails, \(b\) wins | \([-1,-1]\) |
| `Equal` | \([0,0]\) |
| `NoHazard` | \([0,1]\) |
| `NoBenefit` | \([-1,0]\) |
| `Unknown` | \([-1,1]\) |

If all unresolved pairwise mass is \(r\), the resulting gap interval has width at most \(2r\). If uncertainty is one-sided, the width is at most \(r\).

## 15. Benefit regions and hazard covers

Suppose structural reasoning proves

\[
P\subseteq\{u_a=1,u_b=0\}
\]

and

\[
\{u_a=0,u_b=1\}\subseteq C.
\]

Then

\[
B(a\mid b)\ge\beta(P),
\qquad
H(a\mid b)\le\beta(C),
\]

so

\[
\boxed{
V(a)-V(b)
\ge
\beta(P)-\beta(C).
}
\]

Therefore

\[
\beta(P)>\beta(C)
\]

settles the comparison without replaying every world.

The exact-zero hazard witness is the special case \(\beta(C)=0\). A useful solver should also exploit small nonzero hazard covers.

## 16. Policy-family threat bounds

Let \(\pi\) be one incumbent policy and \(\mathcal P\) a policy region. For every world define the world-informed threat and guaranteed safety indicators

\[
T_{\mathcal P\mid\pi}(\omega)
=
\mathbf1\{\exists\rho\in\mathcal P:
 u_\rho(\omega)=1,
 u_\pi(\omega)=0\},
\]

\[
S_{\pi\mid\mathcal P}(\omega)
=
\mathbf1\{u_\pi(\omega)=1
\text{ and }u_\rho(\omega)=0
\text{ for every }\rho\in\mathcal P\}.
\]

For every \(\rho\in\mathcal P\), pointwise,

\[
u_\rho-u_\pi
\le
T_{\mathcal P\mid\pi}-S_{\pi\mid\mathcal P}.
\]

Hence

\[
\boxed{
Q(\mathcal P)-V(\pi)
\le
\mathbb E[T_{\mathcal P\mid\pi}]
-
\mathbb E[S_{\pi\mid\mathcal P}].
}
\]

A proved threat cover \(C\supseteq\{T=1\}\) and a proved safety region \(P\subseteq\{S=1\}\) give

\[
Q(\mathcal P)-V(\pi)
\le
\beta(C)-\beta(P).
\]

If the right-hand side is negative, \(\pi\) beats every policy in \(\mathcal P\).

This is the formal version of two expert questions:

- **What can beat this line?** — build a threat cover.
- **What can set this contract?** — build a hazard cover or guaranteed-failure region.

The cover may be coarse. Soundness requires containment, not exact characterization.

## 17. Cell refinement theorem

Suppose one cell \(C\) of weight \(w\) and interval \([\ell,r]\) is replaced by an exact subpartition

\[
C=\dot\bigcup_kC_k,
\qquad
\ell\le\ell_k\le r_k\le r.
\]

Then the aggregate lower bound cannot decrease and the aggregate upper bound cannot increase.

**Proof.** Since \(\sum_kw_k=w\),

\[
\sum_kw_k\ell_k\ge w\ell,
\qquad
\sum_kw_kr_k\le wr.
\]

All other cells are unchanged. ∎

This is the belief-space analogue of policy-cylinder refinement.

---

# Part V — Exact posterior mass without complete worlds

## 18. Root deals as a factorized exact-cover belief

Let \(U\) be the hidden current-remainder tile set at the root. Let \(S\) be the hidden seats and let \(k_s\) be seat \(s\)'s remaining hand size.

A physical hidden deal is a tuple

\[
H=(H_s)_{s\in S}
\]

such that

\[
|H_s|=k_s,
\qquad
H_s\cap H_t=\varnothing\quad(s\ne t),
\qquad
\dot\bigcup_{s\in S}H_s=U.
\]

For each hidden seat let

\[
\phi_{s,0}:\binom{U}{k_s}\to\mathbb Q_{\ge0}
\]

be a local root-hand factor. The unnormalized root weight is

\[
\boxed{
W_0(H)
=
\mathbf1\{H\text{ is a disjoint cover of }U\}
\prod_{s\in S}\phi_{s,0}(H_s).
}
\]

The partition function is

\[
Z_0=\sum_HW_0(H),
\]

and the root belief is

\[
\beta_0(H)=W_0(H)/Z_0.
\]

The current uniform lawful fiber is the special case in which each \(\phi_{s,0}\) is a \(0/1\) legality predicate induced by capacity and public void constraints.

## 19. Seat-local field kernels

At public history \(h\), hidden seat \(s\)'s remaining hand is a deterministic function

\[
R_s(H_s,h).
\]

A seat-local field kernel is

\[
K_s(t\mid H_s,h)\in\mathbb Q_{\ge0},
\qquad
\sum_tK_s(t\mid H_s,h)=1,
\]

with zero probability on illegal actions.

A deterministic field is the special case

\[
K_s(t\mid H_s,h)
=
\mathbf1\{\sigma_s(R_s(H_s,h),h)=t\}.
\]

The current frozen level-0 and level-1 field models are intended to have exactly this locality: once their identities and seeds are fixed, the action is a pure function of the acting seat's own hand and the public record.

## 20. Seat-factor posterior-closure theorem

For public continuation history \(h\), define

\[
\phi_{s,h}(H_s)
=
\phi_{s,0}(H_s)
\prod_{j:\text{actor}(j)=s}
K_s(t_j\mid H_s,h_{j-1}).
\]

### Theorem 20.1 — posterior closure

Conditioned on the public history \(h\), the exact posterior has unnormalized weight

\[
\boxed{
W_h(H)
=
\mathbf1\{H\text{ covers }U\}
\prod_{s\in S}\phi_{s,h}(H_s).
}
\]

Thus the posterior remains a product of seat-local root-hand factors coupled only by the disjoint-cover constraint.

**Proof.** By Bayes' rule,

\[
\Pr(H\mid h)
\propto
\Pr(H)\Pr(h\mid H).
\]

The root term is the stated exact-cover product. Every non-focal public action contributes the local likelihood \(K_s(t_j\mid H_s,h_{j-1})\). Focal actions are fixed by the focal policy and common public information, so along a consistent branch they contribute a factor independent of hidden hands. Multiplying the non-focal likelihoods and regrouping by acting seat yields the displayed product. ∎

### Boundaries

The theorem requires:

1. the root belief to have this factor form, or to be represented by a broader factor graph;
2. field action probabilities to depend only on the acting seat's private hand and public history;
3. no unrepresented shared hidden tape coupling different seats' actions;
4. exact likelihoods for stochastic fields;
5. the public history to include everything the field reads.

A globally correlated learned belief or a shared persistent latent can still be represented, but it introduces additional factors and can increase contraction width. It may not be silently projected into seat-local factors.

## 21. Exact branch masses

Let

\[
Z_h=\sum_HW_h(H).
\]

If hidden seat \(s\) acts and public action \(t\) is observed, update only its factor:

\[
\phi_{s,ht}(H_s)
=
\phi_{s,h}(H_s)K_s(t\mid H_s,h),
\]

with every other seat factor unchanged.

Then

\[
\boxed{
\Pr(t\mid h)=\frac{Z_{ht}}{Z_h}.}
\]

Define the exact completion weight of a proposed hand \(A\) for seat \(s\):

\[
C_{-s,h}(U\setminus A)
=
\sum_{(H_r)_{r\ne s}:
\dot\cup_{r\ne s}H_r=U\setminus A}
\prod_{r\ne s}\phi_{r,h}(H_r).
\]

Then

\[
\boxed{
Z_{ht}
=
\sum_{A\in\binom{U}{k_s}}
\phi_{s,h}(A)
K_s(t\mid A,h)
C_{-s,h}(U\setminus A).
}
\]

This equation is the one-ply contraction target.

It replaces a loop over complete deals by a loop or symbolic sum over possible hands of the acting seat, weighted by exact compatible completions.

## 22. The opening-scale arithmetic in 42

At trick 1 the focal hand removes seven dominoes, leaving \(21\) hidden dominoes split \(7/7/7\) across three seats.

The number of complete worlds is

\[
\binom{21}{7}\binom{14}{7}
=
116{,}280\cdot3{,}432
=
399{,}072{,}960.
\]

One particular hidden seat has only

\[
\binom{21}{7}=116{,}280
\]

possible root hands.

At the uniform root, each such hand has exactly

\[
\binom{14}{7}=3{,}432
\]

compatible completions for the other two seats.

Therefore the exact first hidden-seat action distribution does not intrinsically require replaying \(399\) million complete deals. It can be expressed as a weighted classification of \(116{,}280\) acting-seat hands.

The field classification of those hands may still be expensive. That is a separate compression problem, not a reason to retain complete worlds as the belief representation.

## 23. Exact factorized Bellman recursion

A factorized belief state is

\[
\mathcal B=(h,(\phi_{s,h})_{s\in S},Z_h).
\]

For the fixed-field best-response value \(V(\mathcal B)\):

### Terminal or decided state

If utility is already determined by the public state,

\[
V(\mathcal B)=u(h).
\]

For `pmake`, the existing monotone decided cutoff applies.

### Focal node

All worlds represented by \(\mathcal B\) share the same public history and focal hand. Therefore they share one focal information state. The exact lawful recursion is

\[
\boxed{
V(\mathcal B)
=
\max_{a\in A(I(h))}
V(\mathcal B\cdot a).
}
\]

The factor arrays do not need to change for a focal play; the public history changes, which changes future legality and future field kernels.

### Hidden field node

If hidden seat \(s\) acts,

\[
\boxed{
V(\mathcal B)
=
\sum_t
\frac{Z_{ht}}{Z_h}
V(\mathcal B\cdot t).
}
\]

### Theorem 23.1 — exactness

Under the hypotheses of Theorem 20.1, this recursion equals

\[
\max_\rho\mathbb E_{\beta_0}[u_\rho^\sigma].
\]

**Proof.** The branch ratios are exact conditional probabilities by Theorem 20.1. At a focal node one action is selected for the shared focal information state, enforcing information consistency. At a field node the field action is integrated under its exact posterior distribution. Backward induction on the finite remaining horizon gives the fixed-field best-response value. ∎

No explicit list of complete worlds appears in the mathematical recursion.

---

# Part VI — The exact-cover contraction interface

## 24. One abstract authority

The solver should depend on an exact contraction interface, not on one representation:

```text
ExactCoverOracle
  mass(factors) -> Z
  actor_completion_weights(factors, seat) -> weight by root hand
  branch_masses(factors, seat, public, field) -> {action -> Z_action}
  condition(factors, seat, public, action) -> new factors
  count_cell(factors, structural_predicate) -> exact mass
  marginal(factors, seat, hand_predicate) -> exact mass
```

Every returned mass is an integer or rational derived from one canonical factor state. A backend may be changed only under exact parity gates.

## 25. Candidate backends

### 25.1 Existing tile-pattern dynamic program

The current kernel groups tiles by allowed-seat pattern and counts capacity-respecting assignments. This is the root special case in which local hand factors are induced by tile-wise allowance predicates.

It remains the correct authority for the current physical fiber.

### 25.2 Acting-hand enumeration with exact completion weights

Enumerate only possible hands of the seat to act. For each hand:

1. compute its local factor weight;
2. compute the exact compatible-completion weight of the other seats;
3. query the field action once;
4. add the product to that action bucket.

At an opening 42 root this is \(116{,}280\) field-hand classifications rather than \(399{,}072{,}960\) deal replays.

### 25.3 Ranked subset convolution

Represent each seat factor as a function on subsets of \(U\), supported on the correct hand size. The exact-cover partition function is an iterated subset convolution. A generic exact implementation can be obtained by ranked zeta transforms in time polynomial in \(|U|\) times \(2^{|U|}\), rather than proportional to the number of complete deals.

This is a candidate backend, not a performance claim. Exact integer growth, constant factors, sparse supports, and repeated-history reuse must be measured.

### 25.4 Sparse hand tables and complement indices

When action observations eliminate most hands, store only nonzero seat factors. Use complement-indexed hash tables or bitsets to contract sparse supports.

### 25.5 Decision diagrams or factor graphs

A zero-suppressed decision diagram, binary decision diagram, or custom factor graph may compactly represent hand families defined by tile membership, suit counts, voids, and action-consistency filters.

The mathematical interface does not depend on adopting any named representation.

## 26. What must be measured separately

1. contraction arithmetic cost;
2. number of distinct acting-seat hands requiring field materialization;
3. field-action cost per hand;
4. reuse of hand classifications across public histories and candidates;
5. factor support shrinkage after observations;
6. cache identity cost;
7. exact integer width;
8. memory at trick 1;
9. suitability for SIMD/GPU/WebAssembly.

A fast partition function with an expensive field classifier is not a fast Walt. A fast field sketch with an incorrect mass contraction is not an exact Walt.

---

# Part VII — Consequential dominoes as counterexample-guided abstraction

## 27. Consequence is contextual

There is no context-free set of “important dominoes.” A domino is consequential relative to:

- the root public state;
- the focal action or policy region under comparison;
- the fixed field;
- the utility;
- the current belief factor state;
- the unresolved bound.

A tile can matter because it changes:

- legal following;
- trump control;
- current trick strength;
- count exposure;
- the possibility of a ruff;
- an entry or lead transfer;
- a modeled seat's action;
- the contract outcome.

## 28. Hand abstractions

For hidden seat \(s\), let

\[
\kappa_s:\binom{U}{k_s}\to K_s
\]

be a finite feature map. It partitions possible hands into classes.

Candidate coordinates include:

- possession of named critical tiles;
- highest remaining trump rank;
- number of remaining trumps;
- suit lengths and void-enabling counts;
- count-tile ownership;
- protected winners and stoppers;
- entries needed to regain lead;
- motif-derived reveal/retain coordinates;
- current field action.

The names do not make a class exact.

## 29. Exact-response and bounded-response classes

At public history \(h\), a class \(C\) is **action-exact** for field seat \(s\) if

\[
\sigma_s(R_s(H,h),h)=t_C
\]

for every hand \(H\in C\) with nonzero posterior weight.

It is **action-bounded** by \(A_C\) if every such hand chooses an action in \(A_C\).

For value propagation, a stronger property may be needed: all hands in the class must induce the same abstract successor or a proved common value interval.

A class verifier may return:

```text
ExactAction(tile)
ActionSet(nonempty set)
ExactValue(interval point)
ValueInterval(lower, upper)
SplitWitness(hand_left, hand_right, semantic_difference)
Unknown
```

Only verifier-produced exactness may be aggregated without residual uncertainty.

## 30. Counterexample-guided refinement

Start from a deliberately coarse \(\kappa\). For every class whose mass can still affect the root decision:

1. test whether its field action or value bound is uniform;
2. if uniform, aggregate the class exactly;
3. otherwise produce two hands \(H,H'\) in the same class with different semantic behavior;
4. choose a decidable discriminator \(\psi\) with \(\psi(H)\ne\psi(H')\);
5. split the class by \(\psi\);
6. repeat only while the class can still move a root interval across the bar.

The discriminator may be tile membership, a suit-count threshold, a highest-trump category, or another exact feature.

### Theorem 30.1 — refinement safety

If every class carries a sound semantic interval and a refinement replaces it by subcells with sound narrower intervals and exact masses, the root interval remains valid and narrows monotonically.

This follows directly from Theorem 17.1.

### Completeness

On a finite hand domain, repeatedly splitting until every class is a singleton hand yields exact field actions and exact hand-level factors. Therefore the abstraction is complete in the limit even if it is very coarse at every playable budget.

## 31. The critical-tile interpretation

A tile enters the current critical set when it separates a witnessed semantic disagreement that the current abstraction failed to distinguish.

This gives a rigorous meaning to expert attention:

> “Look for the high trump, the count tile, the only ruff, or the stopper” means “try these as low-cost discriminator coordinates because human play suggests they often separate semantic classes.”

The suggestion is heuristic. The witness and resulting class verifier determine whether the tile actually matters at the current root.

---

# Part VIII — A unified dual-refinement solver

## 32. Solver state

For every root action maintain:

- one or more lawful lower-witness policies;
- an optimization-lock upper confidence sequence;
- a frontier of policy cylinders;
- optional policy grammar and residual family;
- factorized belief states for exact contractions;
- structural consequence cells;
- exact or \(\delta\)-valid bounds;
- a scoped risk ledger;
- cost and potential-impact forecasts;
- an explanation trace for every decisive refinement.

## 33. Work item types

```text
ImproveLower(action, policy_source)
TightenRootUpper(action, more_worlds)
SolveGrammar(action, grammar)
SplitPolicyCylinder(region, information_state)
CountThreatCover(region, predicate)
RefineConsequenceCell(cell, discriminator)
ContractFieldBranch(belief, seat)
MaterializeFieldClass(class)
EnumerateResidual(domain)
EscalateExact(action_or_region)
```

Every work item declares:

- which lower or upper bound it may change;
- the strongest possible resulting bound;
- the root bars it can affect;
- deterministic or \(\delta\)-qualified proof type;
- estimated cost;
- prerequisites and cache identities.

## 34. Generalized steering rule

Let current root bar be \(B\). A challenger action \(a\) can be pruned only if its upper falls below \(B\). If a proposed refinement has a proved best-case upper floor \(U_a^{\min}\) with

\[
U_a^{\min}\ge B,
\]

then that refinement cannot prune \(a\) at the current bar and may be refused as presently useless.

Likewise, a proposed lower-witness computation for action \(a\) is presently useless for root pruning if even its proved best-case lower ceiling cannot raise the bar or settle an outstanding comparison.

This is the policy/belief generalization of the existing “provably useless E4” steering lemma.

## 35. Decision-oriented scheduling

A heuristic scheduler should rank work by an estimate of

\[
\frac{
\text{maximum possible reduction in root decision width}
}{
\text{cost}
}.
\]

For a structural cell of mass \(w\) and current pairwise interval width \(d\), the absolute maximum contribution to gap-width reduction is \(wd\). For an unknown Boolean pair cell, \(d=2\). For a one-sided cell, \(d=1\).

For a policy cylinder, the relevant quantity is its excess upper over the current bar:

\[
\max(0,U(p)-B).
\]

Sorting legal moves inside one exact solve is a special, low-level scheduling rule. The new scheduler chooses which entire policy family or belief mass to avoid solving.

## 36. The full loop

```text
1. Build the canonical root and factorized root belief.
2. Seed lawful lower policies from level 1, level 2 sketch, safety rules,
   count-preservation rules, and transported/library policies.
3. Build direct optimization-lock uppers from empirical best-response counts.
4. Form root intervals and the admissible root set.
5. If singleton, return exact/δ-settled winner according to bound types.
6. For surviving actions, build a small grammar from the strongest policies.
7. Bound the residual policy family with sampled, structural, or exact uppers.
8. Count consequence cells and threat/hazard covers where they can close a gap.
9. Use factorized contractions for exact branch masses and exact continuation
   where the cost model predicts a gain.
10. Split the policy cylinder or hand class with the largest decision impact.
11. Repeat until settled, equivalent, exact, or out of budget.
12. On budget exhaustion, return the honest surviving set; a named fallback
    may choose within it.
```

## 37. Soundness invariant

At every point:

1. every lower bound is the exact or \(\delta\)-valid value of a lawful information-consistent policy or restricted lawful class;
2. every upper bound covers the complete policy region it names;
3. every exact mass is computed under the same belief/fiber as the bound consuming it;
4. every sampled claim names its stream, epoch, policy class, field, utility, and risk scope;
5. policy-region children cover their parent;
6. consequence-cell children partition their parent;
7. omitted regions are never silently dropped;
8. heuristic scheduling reads bounds but cannot manufacture them;
9. fallback choices are never promoted to settled results.

---

# Part IX — Completeness and tractability

## 38. Finite completeness theorem

Assume:

- the game, world set, and information-state set are finite;
- every policy cylinder can eventually be split at every unassigned focal information state;
- every hand abstraction can eventually be split to singleton hands;
- exact contraction or explicit enumeration is available on singleton domains;
- every exact leaf utility terminates.

Then exhaustive refinement eventually computes every \(Q_a\) exactly.

**Proof.** The finite policy class can be partitioned into singleton policies by finitely many cylinder splits. The finite world set can be partitioned into singleton worlds by finitely many belief-cell splits. Exact evaluation at singleton leaves gives exact policy values; maxima over finite exact children give exact cylinder and root values. ∎

This theorem proves completeness in principle, not tractability.

## 39. The tractability hypothesis

The practical hypothesis is that most roots settle long before singleton refinement because:

- a small grammar contains a near-optimal or optimal policy;
- residual policy regions have small threat mass;
- a small number of critical tiles separates most field actions;
- exact seat-hand contractions are much smaller than complete world replay;
- many structural cells are outcome-inert;
- root separation needs only one lower witness to clear all rival uppers;
- honest near-ties can remain unresolved or be declared practically equivalent.

The hypothesis is falsifiable. The solver remains correct if it fails.

## 40. Failure modes worth distinguishing

1. **Upper-bound looseness:** the direct sampled optimization upper remains near one because the policy class overfits prefixes.
2. **Grammar failure:** residual policy uppers remain competitive.
3. **Consequence explosion:** the critical feature set approaches the full hidden hand.
4. **Contraction wall:** exact-cover arithmetic scales comparably to complete deals.
5. **Field-classification wall:** too many distinct hands require expensive modeled-mind solves.
6. **Belief-state explosion:** public actions generate too many distinct factor states.
7. **True decision hardness:** the root actions genuinely remain nearly tied.

These require different responses. None should be hidden under a generic “slow” label.

---

# Part X — Engineering architecture

## 41. One Walt, new authorities inside it

Do not create another player implementation. Add the new mathematics behind the unified solver interfaces.

Suggested conceptual modules:

```text
solver/root_sandwich.rs
  root-action lower/upper intervals, survivor set, result typing

solver/policy_region.rs
  partial-policy cylinders, grammars, residual families, split ledger

solver/factor_belief.rs
  seat-local root-hand factors and posterior updates

solver/exact_cover.rs
  contraction interface and backends

solver/consequence.rs
  counted cells, threat/hazard covers, refinement witnesses

solver/refine.rs
  work items, steering, monotone update, budget termination
```

Existing authorities should be reused:

- `kernel` for the canonical physical root fiber and exact root count;
- `evidence` and `upper_cs` for exact-rational confidence sequences;
- `policy` for frozen policy identity;
- `controller` for risk-ledger and result discipline;
- `exposure` for information-consistent empirical optimization patterns;
- `hazard` for structural zero-hazard verification;
- `motif` for feature discovery and explanations;
- `bundle` and explicit enumeration as extensional parity oracles.

## 42. Proposed data types

```rust
struct RootActionInterval {
    action: Domino,
    lower: LowerBound,
    upper: UpperBound,
    policy_class: PolicyRegionId,
}

enum LowerBound {
    ExactPolicy { value: Rational, policy: PolicyId },
    DeltaPolicy { lower: Rational, delta: ScopedDelta, policy: PolicyId },
    ExactRestricted { value: Rational, grammar: GrammarId },
}

enum UpperBound {
    ExactRegion { value: Rational, region: PolicyRegionId },
    StructuralRegion { upper: Rational, witness: WitnessId },
    DeltaEmpiricalMax {
        upper: Rational,
        delta: ScopedDelta,
        region: PolicyRegionId,
        stream: StreamIdentity,
    },
}

struct HandFactor {
    seat: Seat,
    capacity: u8,
    weights: HandFamilyRepresentation,
}

struct FactorBelief {
    root_id: RootId,
    public: PublicState,
    hidden_universe: DominoSet,
    factors: Vec<HandFactor>,
    mass: BigUint,
}

struct ConsequenceCell {
    predicate: PredicateId,
    exact_mass: BigUint,
    semantic_interval: RationalInterval,
    proof: CellProof,
}
```

Constructors should keep sampled estimates from inhabiting structural or exact bound types.

## 43. Identity and cache discipline

A factor-state identity must include:

- root physical fiber identity;
- public history/state sufficient for all field choices and utility;
- field identity;
- each hand-factor representation and weight table;
- stochastic-tape factor identity if present;
- policy-region constraints when solving a cylinder;
- utility and bid/declaration parameters.

A cache hit under an omitted field or factor coordinate would be the factorized analogue of the earlier PiKey defect.

---

# Part XI — Recommended implementation program

## 44. Slice A — optimization-lock root sandwich

This is the first build.

### Build

For each legal root action:

1. run the existing exact-on-prefix information-consistent solver to obtain \(S_{a,n}^\star\) on nested common prefixes;
2. produce `DeltaEmpiricalMax` by Theorem 5.1;
3. discover a strong policy on a domain-separated stream;
4. freeze it;
5. evaluate it on a lower-bound stream and produce `DeltaPolicy`;
6. form root intervals and a typed survivor set.

### Gates

- on every affordable exact-root fixture, \(L_a\le Q_a\le U_a\);
- every exact optimizer remains in the survivor set;
- singleton survivors agree with the exact root action;
- same-stream selected lower witness is unconstructible or rejected;
- empirical-max upper accepts exact or optimistic sample solvers, rejects lower approximations;
- policy count does not enter the upper risk allocation;
- root-action risk allocations sum under the existing ledger;
- caps yield `UnresolvedRootSet`, never a forced winner;
- batch/pause invariance follows the existing canonical stream semantics.

### Probe

Run exact-root fixtures by trick and fiber size. Report:

- exact \(Q_a\);
- lower/upper intervals by prefix;
- survivor-set size;
- worlds to singleton where it occurs;
- upper looseness attributable to optimization overfit;
- lower shortfall attributable to policy quality.

This immediately separates “need a better policy” from “need a tighter upper.”

## 45. Slice B — two-policy grammar and residual upper

Construct a small grammar from:

- current level-1 continuation;
- current level-2 sketch or waking continuation;
- one safety/trump-control or count-preservation policy.

Solve the restricted grammar exactly on the sample and, where affordable, over the exact factor or fiber. Build one residual policy region containing every off-grammar continuation. Apply the empirical-max upper theorem to the residual.

Measure:

- \(Q^G\) versus exact \(Q\);
- residual upper versus grammar lower;
- first off-grammar information states in exact counterexamples;
- grammar action-set size per information state;
- whether a grammar optimizer closes the root before full policy search.

Do not infer coverage from the two policies' disjointness. Coverage is the residual bound.

## 46. Slice C — one-ply factorized branch mass

Build `FactorBelief` and `ExactCoverOracle` only far enough to compute one hidden seat's next-action distribution.

### Stage C0 — trivial field

Use `FixedPreference::lowest_first` or another exact cheap field. Verify:

\[
Z_h=\sum_tZ_{ht}
\]

and parity with complete-world enumeration on small and medium fibers.

### Stage C1 — cached level-0 field

Classify every possible acting-seat hand once, weight by exact completions, and compare action buckets with the bundle oracle.

### Stage C2 — opening root

On a trick-1 root with \(399{,}072{,}960\) worlds, compute the exact first hidden-seat action bucket masses without materializing the complete fiber.

Report separately:

- number of acting-seat hands;
- contraction time;
- field-classification time;
- number of distinct field actions;
- cache reuse;
- memory;
- exact mass conservation.

A successful slice proves a representation change, even if the field classifier remains slow.

## 47. Slice D — recursive factorized fixed-policy evaluation

Before exact best response, evaluate one frozen focal policy under a simple fixed field by the factorized Bellman recursion. Compare its exact value and all branch masses with bundled or explicit enumeration.

Then move to the current level-0 field.

This isolates posterior contraction from focal optimization.

## 48. Slice E — factorized exact best response inside a grammar

At focal nodes maximize only over grammar actions. At field nodes contract exact action buckets. This gives an exact lower witness \(Q^G\) without full world enumeration.

Only after this lands should the full action set be enabled.

## 49. Slice F — consequence CEGAR

Instrument hand classes at the field-classification bottleneck.

Start from a small feature vocabulary:

- critical tile membership;
- trump count/highest trump;
- led-suit count;
- count-tile possession;
- current winner/ruff possibility.

Require witness pairs for every refinement. Measure residual class mass and root interval impact, not classifier accuracy.

## 50. Slice G — integrated refinement controller

Unify:

- root sampled upper/lower intervals;
- grammar/residual policy regions;
- factorized exact contractions;
- consequence-cell bounds;
- explicit enumeration fallback;
- existing calculated-evidence and risk ledgers.

The existing controller player remains the fallback surface until arena and conformance gates authorize a default change.

---

# Part XII — Experiments and falsifiers

## 51. Experiment matrix

| Experiment | Primary question | Success signal | Falsifier |
|---|---|---|---|
| Optimization-lock sandwich | Can sampling bound the full policy class? | singleton or small root survivor sets at affordable prefixes | upper remains vacuous until exact cost |
| Grammar/residual | Do two or three policies cover the useful action grammar? | exact grammar optimum clears residual upper | residual remains competitive everywhere |
| One-ply contraction | Can exact response mass avoid complete worlds? | actor-hand or symbolic scaling, exact parity | contraction cost tracks full worlds |
| Field-hand abstraction | Do few features determine field action? | most posterior mass in action-exact classes | feature set approaches all hidden tiles |
| Counted consequence cells | Can exact mass close pairwise gaps? | benefit lower exceeds hazard upper | residual mass remains too large |
| Recursive factor belief | Is posterior factorization computationally reusable? | repeated factors and sublinear world scaling | factor-state explosion erases gain |
| Integrated solver | Does correct incompleteness become playable? | frequent singleton/small survivor sets under budget | nearly all early roots stay fully unresolved |

## 52. Required corpus

Use at least:

- affordable exact-root fixtures across tricks 4–6;
- the four historical fixed-set disagreements;
- the h8-t4 field-decision change;
- the count-timing family;
- the reconstructed Gran reveal/retain roots when available;
- deliberately generated high-trump versus vulnerable-double roots;
- opening roots with full fiber \(399{,}072{,}960\).

## 53. Reported coordinates

Per root report:

- physical fiber size;
- hidden-seat hand-domain sizes;
- exact or interval \(Q_a\);
- root survivor count by refinement stage;
- lower-witness shortfall;
- optimization-upper excess;
- policy cylinders opened/pruned;
- grammar size;
- consequence-cell masses and interval widths;
- factor states and contraction operations;
- distinct field-hand materializations;
- worlds never materialized;
- wall time by phase;
- result type.

The central graph is root decision width versus cumulative cost.

---

# Part XIII — Proof obligations

## 54. Mathematical obligations

**CBS-O1 — Optimization-lock upper theorem.** Formalize Theorem 5.1 over finite policy classes and an abstract monotone Bernoulli upper confidence sequence.

**CBS-O2 — Independent lower witness.** Formalize conditional validity when a policy is chosen on a discovery sigma-algebra independent of evaluation.

**CBS-O3 — Root survivor theorem.** Simultaneous interval validity implies safe exclusion and singleton optimality.

**CBS-O4 — Cylinder partition.** Partial-policy children form a disjoint cover and parent optimum is the child maximum.

**CBS-O5 — Grammar/residual decomposition.** The lawful policy class is exactly the union of grammar and residual regions.

**CBS-O6 — Cell aggregation.** Exact masses and pointwise cell intervals produce valid global intervals.

**CBS-O7 — Threat/safety family bound.** Prove Theorem 16's pointwise and expected inequalities.

**CBS-O8 — Seat-factor closure.** Prove Theorem 20.1 for deterministic and rational stochastic seat-local fields.

**CBS-O9 — Branch-mass ratio.** Prove \(Z_{ht}/Z_h\) is the exact conditional action probability.

**CBS-O10 — Factorized Bellman correctness.** Prove Theorem 23.1 by finite-horizon induction.

**CBS-O11 — Refinement monotonicity.** Policy-cylinder and consequence-cell refinements preserve validity and narrow bounds.

**CBS-O12 — Finite completeness.** Exhaustive splitting and exact leaf evaluation converge to exact root values.

**CBS-O13 — Backend parity.** Every exact-cover backend agrees extensionally with complete-world enumeration on its declared domain.

**CBS-O14 — Factor identity sufficiency.** Every field and utility input read by a contraction is represented in its cache key.

**CBS-O15 — Risk composition.** Root action, lower witness, policy-region, and structural-estimate risks fit under one serialized run ledger.

## 55. Candidate Lean module map

```text
Walt/RootSandwich.lean
Walt/PolicyCylinder.lean
Walt/PolicyGrammar.lean
Walt/CellBounds.lean
Walt/ThreatHazard.lean
Walt/FactorBelief.lean
Walt/PosteriorClosure.lean
Walt/FactorBellman.lean
Walt/Refinement.lean
Walt/FiniteCompleteness.lean
Walt/EmpiricalMaxUpper.lean

Texas42/CurrentRemainderFactor.lean
Texas42/PmakeCellBounds.lean
```

The deterministic finite theorems should precede the probability formalization. `EmpiricalMaxUpper` may initially assume the one-mean confidence-sequence contract already proved elsewhere, then later connect it to the e-process construction.

---

# Part XIV — Acceptance contract for the first engineering intake

## 56. First-session assignment

The first implementation session should build **Slice A only**, plus a design skeleton for Slice C. It should not attempt the recursive factorized solver in the same session.

### Required Slice A outputs

1. `RootActionInterval` and root survivor result types.
2. A producer for \(S_{a,n}^\star\) on nested common prefixes.
3. A `DeltaEmpiricalMax` upper using the existing exact-rational upper confidence sequence.
4. A domain-separated discovery/freeze/evaluation route for one lower policy per root action.
5. Exact risk allocation across actions and endpoints.
6. Small-fiber exact-root parity.
7. An instrument reporting interval and survivor evolution.
8. No change to the live default player.

### Slice C design skeleton

Specify, but do not yet optimize:

- `HandFactor` identity;
- `FactorBelief` identity;
- `ExactCoverOracle` trait;
- one-ply branch-mass output;
- parity oracle and mass-conservation gates.

## 57. Stop conditions

Stop and report rather than patch if:

- the existing empirical solver does not actually compute the whole-prefix information-consistent optimum \(S_{a,n}^\star\);
- nested prefixes change the field or policy class;
- the lower witness cannot be separated from discovery data;
- root-action risks do not fit the ledger;
- a factorized branch requires data not contained in the acting seat's hand and public record;
- current root beliefs contain global correlations not representable by the proposed factors;
- a cache key omits factor or field identity.

Each is a mathematical boundary, not an implementation inconvenience.

---

# Part XV — Final synthesis

The current exact engine is extraordinary, but its hot path still pays for explicit worlds and distinct modeled-hand states. The recent optimizations have shown that rearranging those costs does not remove them.

The next solver should reason in three nested spaces:

1. **Root actions**, represented by lower/upper intervals and a surviving set.
2. **Policy regions**, represented by lawful lower witnesses and admissible residual uppers.
3. **Belief mass**, represented by exact seat-local hand factors and counted consequence cells rather than complete worlds.

Sampling is retained, but its role changes:

\[
\boxed{
\text{sample to discover and bound,}
\quad
\text{count to eliminate,}
\quad
\text{contract to solve,}
\quad
\text{enumerate only the unresolved residue.}
}
\]

The immediate sampled optimization-lock theorem supplies a correct bounded root solver before the exact backend is complete. The factorized posterior theorem supplies the representation needed to attack the opening-scale exact route. Policy grammars and consequence classes connect human strategic attention to mathematically safe refinement without granting heuristics any authority they have not earned.

The governing principle is:

> **Do not ask which computation looks promising. Ask which unresolved policy family or belief mass can still change the root decision, and what is the cheapest valid operation that can remove that possibility.**

If the tractability hypothesis is right, most of the full game never needs to be solved at most roots. If it is wrong, Walt still returns a correct bounded survivor set and a plainly labeled fallback. Either way, no magic sample count and no hidden approximation is required.
