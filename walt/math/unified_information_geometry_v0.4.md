# Scheme, Information Geometry, and Dynamic Control Skeletons in Straight Texas 42

**Status:** unified standalone mathematical draft, v0.4  
**Date:** 2026-08-09  
**Primary scope:** post-declaration Straight points-and-marks Texas 42, finite contracted-hand continuations, exact rule support, finite beliefs and fields, and valuation-parametric decision problems  
**Supersedes for conceptual use:** `straight_42_scheme_parametric_model_v0.3.md` together with Amendment A; the earlier files remain provenance artifacts  
**Does not supersede:** the normative rules profile or the established *Mathematical Foundation of Straight Texas 42*  
**Formalization intent:** prose-and-LaTeX source for later proof-assistant decomposition; no theorem in this draft is machine checked merely because a proof is written here

This document is a clean reconstruction of the model after the first four experiment families. It is not an amendment stack. It states the mathematical object as it is currently understood, integrates the corrections forced by the experiments, and separates established foundations, mathematical consequences, finite reported evidence, conjectures, and open work.

The source artifacts behind this consolidation are:

- the current Straight 42 rules and mathematical foundation;
- the Scheme session capture and `SchemeV3.lean`;
- integration drafts v0.1, v0.2, and v0.3;
- Amendment A to v0.3;
- the exact-arithmetic reports for Experiments 1–3 and Experiment 4.

The experiment reports state exact integer and `Fraction` arithmetic, source-hand replay checks, independent scalar validators, full-breakpoint continuity checks, and whole-ray inequality certificates. Those reports are treated here as **reported exploratory receipts**, not as promoted corpus theorems, because this consolidation did not independently execute the attached programs.

---

## Claim discipline

- **[ADOPTED RULE]** A rule fixed by the Straight 42 rules profile.
- **[INHERITED]** Established in the current mathematical foundation, or represented by a proof term in the supplied exploratory Lean source where explicitly stated.
- **[DEFINITION]** A definition adopted for this integrated model.
- **[THEOREM — proved here]** Accompanied by a mathematical proof in this document; not thereby machine checked.
- **[PROPOSITION]** Proved under the assumptions stated beside it.
- **[COROLLARY]** A direct consequence of earlier statements.
- **[IMPLEMENTATION CONTRACT]** A semantic equality or preservation condition that an implementation must satisfy.
- **[EXPERIMENTAL RECEIPT — reported]** An exact finite result stated by the supplied probe reports, with the report’s own validation chain, but not independently rerun in this pass.
- **[INTERPRETATION]** A structural reading supported by the current theory and evidence but not proved for all Straight 42 domains.
- **[CONJECTURE]** A mathematically precise claim suggested by evidence and still open.
- **[BOUNDARY]** A scope restriction that must remain visible.
- **[OPEN]** A theorem, compiler, census, certificate, or experiment still owed.

---

# 0. Executive synthesis

The current model can be expressed in one chain, but the arrows must not be collapsed:

\[
\boxed{
\begin{array}{c}
\text{declaration-relative physical game}
\longrightarrow
\text{exact rule-support kernel }K
\longrightarrow
\text{latent domain }\Xi(K,e),\\[1mm]
\text{Scheme/Fix query with output roles}
\longrightarrow
\text{answer bundles over latent worlds},\\[1mm]
\text{information partition}
\longrightarrow
\text{glued, information-consistent policy class},\\[1mm]
\text{universal terminal-outcome law}
\longrightarrow
\text{valuation and utility}
\longrightarrow
(V,Q),\\[1mm]
\text{selected response contract}
\longrightarrow
\text{strategic constellations and lawful compression}.
\end{array}
}
\]

The load-bearing conclusions are these.

1. **A physical domino is a stable identity, not a stable strategic type.** Declaration, live context, holder relations, follow obligations, and continuation position determine its current role.

2. **Rule support, evidence, belief, field, and value are distinct.** Exact support states which hidden worlds are rule-compatible. Evidence records what was observed. Belief assigns weights. A field supplies continuation behavior and latent state. Value is derived only after all required inputs are named.

3. **A Scheme is a relational query, not a second game state.** It describes patterns inside exact worlds. A declared output interface distinguishes the roles returned by the query from internal existential witnesses used only to prove that the pattern exists.

4. **A true existential statement need not have a unique referent.** A Scheme can be certain in every world and still return several tiles in every world. World probability does not select among those answers. Selection and aggregation require explicit semantics.

5. **Identity through time requires rigid transport.** Re-running an existential query after a play may choose a new witness. Persistence, extinction, and birth are different from fresh satisfaction.

6. **The universal continuation object precedes valuation.** A count-blind solve must retain policy-indexed terminal laws, feature sets, symbolic game forms, or equivalent objects. Retaining only the action optimal at one baseline valuation destroys later valuation universality.

7. **Expected additive values are support functions.** Under fixed belief and a fixed valuation-independent field, every root action has a finite policy-feature polytope. Tile values select exposed faces of that polytope.

8. **Imperfect information is a policy-gluing constraint.** A lawful policy must take one action at every information state, not one action per hidden world. Refining information removes equality constraints, enlarges the attainable policy polytope, and can create, destroy, split, or move valuation breakpoints.

9. **Information value is a support-function gap.** For nested information structures

\[
\mathcal I_H\preceq\mathcal I_C\preceq\mathcal I_F,
\]

there are nested attainable polytopes

\[
P^H\subseteq P^C\subseteq P^F,
\]

and the value of revelation is the difference of their support functions.

10. **World compression and decision compression are different.** A static descriptor that preserves every world’s perfect-information response may need to reconstruct the world. The hidden-information decision can nevertheless have an extremely small exposed policy geometry.

11. **The emerging control ontology is dynamic.** Companion relations, current-context strength, forced followers, beater chains, mobility, successor context, and causal seat position matter. A flat root descriptor is not enough in general; the natural target is an exact relational transducer or lumpable latent quotient.

12. **A constellation is purpose-relative.** Group orbit, Scheme cell, world-response class, belief-state value class, policy-polytope class, and predictive dynamic class are all legitimate objects, but they are not interchangeable.

The current compression picture is therefore layered:

\[
\boxed{
\begin{array}{c}
\text{exact support compression}
\quad+
\text{relational/intensional compression}
\quad+
\text{decision-geometry compression},\\[2mm]
\text{with extensional world compression measured separately.}
\end{array}
}
\]

---

# 1. The Straight 42 base object

## 1.1 Pips, dominoes, seats, and teams

**[DEFINITION]** Let

\[
\mathbb P=\{0,1,2,3,4,5,6\}
\]

be the pip set. The double-six domino universe is

\[
\mathcal D=\operatorname{Sym}^2(\mathbb P),
\]

the 28 unordered two-end multisets over \(\mathbb P\). Write a stable physical identity as \(h\!:\!l\) with \(h\ge l\). The ordering is a naming convention, not a physical orientation.

Equivalently, \(\mathcal D\) is the edge set of the complete graph on seven pip vertices with one loop at every vertex. Mixed dominoes are ordinary edges; doubles are loops.

The seats are

\[
S=\mathbb Z/4\mathbb Z,
\qquad
s^+=s+1\pmod4.
\]

The partnerships are

\[
T_0=\{0,2\},
\qquad
T_1=\{1,3\},
\]

with team map \(\theta(s)=s\bmod2\).

For each pip \(p\), the natural incidence set is

\[
\sigma_p=\{d\in\mathcal D:p\in d\}.
\]

Each \(\sigma_p\) contains seven tiles. A double belongs to one natural incidence set; a mixed tile belongs to two.

## 1.2 Straight declarations and effective contexts

**[DEFINITION]** The Straight declaration set is

\[
\Delta_{\mathrm{Str}}
=
\mathbb P\cup\{\mathrm{DT},\mathrm{NT}\},
\]

where \(\mathrm{DT}\) is doubles-trump and \(\mathrm{NT}\) is no-trump/follow-me.

For \(\delta\in\Delta_{\mathrm{Str}}\), define the called set

\[
\kappa_\delta=
\begin{cases}
\sigma_p,&\delta=p\in\mathbb P,\\
\mathcal D^\circ,&\delta=\mathrm{DT},\\
\varnothing,&\delta=\mathrm{NT},
\end{cases}
\]

where \(\mathcal D^\circ\) is the set of doubles. In Straight 42 every nonempty called set is powered, so \(\pi_\delta=\kappa_\delta\).

Let

\[
Q=\{0,1,2,3,4,5,6,7\},
\]

where context \(7\) names the called suit. Define the effective incidence family

\[
\widehat\sigma_p^\delta=\sigma_p\setminus\kappa_\delta
\quad(p\in\mathbb P),
\qquad
\widehat\sigma_7^\delta=\kappa_\delta.
\]

Called tiles are absorbed into context \(7\) and removed from every natural effective incidence. Uncalled mixed tiles generally remain members of two natural effective incidences; the effective family is a covering, not a partition.

Define the follow predicate

\[
F_\delta(d,q)=\mathbf1[d\in\widehat\sigma_q^\delta].
\]

The led context is

\[
\ell_\delta(d)=
\begin{cases}
7,&d\in\kappa_\delta,\\
\operatorname{high}(d),&d\notin\kappa_\delta.
\end{cases}
\]

Under no-trump this is the high pip. Under pip trump and doubles-trump, called tiles lead context \(7\).

## 1.3 Trick order

**[DEFINITION]** For led context \(q\), give each tile a tier

\[
\operatorname{tier}_\delta(d,q)=
\begin{cases}
2,&d\in\pi_\delta,\\
1,&d\notin\pi_\delta\text{ and }F_\delta(d,q)=1,\\
0,&\text{otherwise.}
\end{cases}
\]

Let \(r_\delta(d)\) be the declaration-relative rank:

- under doubles-trump, doubles are ranked by pip value;
- under every other declaration, a natural double is top in its effective natural suit;
- a mixed tile is ranked by pip sum inside a nonzero tier.

The total trick key is the lexicographic pair

\[
\tau_\delta(d,q)
=
\bigl(\operatorname{tier}_\delta(d,q),r_\delta(d)\bigr),
\]

with every tier-two tile above every tier-one tile and every nonzero tier above tier zero.

**[INHERITED]** For any distinct four-tile trick with a specified lead, the maximum trick key is unique. Thus every legal trick has one winner.

Define

\[
\operatorname{BEATS}_\delta(q,d)
=
\{e\in\mathcal D:\tau_\delta(e,q)>\tau_\delta(d,q)\},
\]

and the when-led threat set

\[
\operatorname{THREAT}_\delta(d)
=
\operatorname{BEATS}_\delta(\ell_\delta(d),d).
\]

## 1.4 Count and terminal scoring

The ordinary Straight count decoration is

\[
c(d)=
\begin{cases}
5,&d\in\{5\!:\!0,4\!:\!1,3\!:\!2\},\\
10,&d\in\{6\!:\!4,5\!:\!5\},\\
0,&\text{otherwise.}
\end{cases}
\]

and

\[
\sum_{d\in\mathcal D}c(d)=35.
\]

Each won trick contributes one trick point plus the count labels on its four tiles. Hence a complete hand contains 42 points.

The valuation theory developed later treats \(c\) as one sparse specialization of a general tile valuation. That later generalization does not alter legal play unless the changed valuation is declared to be a common-knowledge rule change.

## 1.5 Objective game and finite grade

A complete deal is an ordered partition

\[
\omega_0=(H_0^0,H_1^0,H_2^0,H_3^0)
\]

of \(\mathcal D\) into four seven-tile hands. A declaration and bidder determine the first leader. A leader may play any remaining tile. A follower must play a member of the led effective context when able and may slough otherwise. The trick winner leads next.

A legal play removes one tile from one remaining hand, so the contracted post-declaration game is a finite graded directed acyclic graph. Perfect-information backward induction is therefore exact once the utility and optimization operator are fixed.

**[BOUNDARY]** The full pre-contract match process can contain unbounded repeated all-pass attempts. Finite-hand backward induction does not by itself prove a finite-horizon theorem for that process.

---

# 2. Exact rule support and the decision state

## 2.1 Complete deals, current remainders, and viewer support

Fix a viewer \(m\), a legal public history, and the viewer’s observed private record. Let \(H_m\) be the viewer’s known remaining hand. Let \(U\) be the hidden live pool: every live tile not in \(H_m\). Let \(k_s\) be the remaining capacity of hidden seat \(s\ne m\).

Public sloughs induce exact void constraints. For each hidden seat \(s\), let \(P_s\subseteq U\) be the tiles that remain locally possible at \(s\) after all rule-derived exclusions.

**[DEFINITION]** The current-remainder fiber is

\[
\Phi(\mathbf C)
=
\left\{
(H_s)_{s\ne m}:
\begin{array}{l}
H_s\subseteq P_s,\ |H_s|=k_s,\\
H_s\cap H_t=\varnothing\ (s\ne t),\\
\bigsqcup_{s\ne m}H_s=U
\end{array}
\right\},
\]

where \(\mathbf C=(U;(P_s,k_s)_{s\ne m})\) is the dependent capacity-cell system.

**[INHERITED]** In the Straight cell-theorem scope, \(\Phi(\mathbf C)\) is exactly the set of hidden current remainders compatible with the viewer’s private hand and the actor-attributed legal public prefix. It is not merely a local Hall relaxation.

A member

\[
\omega=(H_s)_{s\ne m}\in\Phi(\mathbf C)
\]

reconstructs one current objective world when joined to the viewer’s known hand and public physical residue.

## 2.2 Exact support normal form

Different cell presentations can decode the same fiber. The foundation therefore quotients support by extensional equality.

**[INHERITED]** Every feasible capacity-cell system has a canonical exact support normal form

\[
N=\mathcal N(\mathbf C)
\]

that decodes exactly \(\Phi(\mathbf C)\). Its native representation separates:

- tiles certainly held by each hidden seat;
- a residual ambiguous pool;
- residual capacities;
- the matching-supported binary or ternary ambiguity core.

Two feasible cell systems have the same normal form exactly when they decode the same support set. Thus \(N\) is the coarsest exact deterministic semantic representation of the selected current-remainder support.

**[BOUNDARY]** Feasibility and exact normal-form decoding do not by themselves imply legal Straight reachability. A support object used as a current game state must be inherited from legal construction or accompanied by an accepted reachability witness or certificate. Reachability is not a redundant Boolean field inside the support normal form.

**[BOUNDARY]** This is semantic/state-count minimality for exact support, not a universal claim about bits, runtime, cache locality, or strategic sufficiency.

## 2.3 The reduced physical/support kernel

Let \(\tau\) denote either the trick-boundary leader or the exact folded unresolved-trick residue. Let \(\alpha_{\mathcal A}\) be an accumulator sufficient for a declared purpose family \(\mathcal A\), possibly trivial.

**[DEFINITION]** The local viewer kernel is

\[
K=(\delta,H_m,N,\tau,\alpha_{\mathcal A}).
\]

Its exact rule worlds are

\[
\Phi(K):=\llbracket N\rrbracket.
\]

A concrete viewer situation is

\[
x=K\oplus\omega,
\qquad
\omega\in\Phi(K).
\]

The live carrier is determined by \(K\):

\[
L(K)=H_m\sqcup U(N).
\]

The current-holder relation is total exactly on \(L(K)\):

\[
\operatorname{Holds}_x(s,d)
\iff
d\in H_s(x).
\]

Played or captured tiles remain stable physical identities but have no current holder.

The chart—live set, hidden pool, current led context, current winner, and derived standings—is a deterministic view of the kernel. It is not an independent authority.

## 2.4 Evidence, latent field state, and belief

A path-free kernel need not retain every observation that a continuation field, learner, or utility can still use. Let

\[
e
\]

be the required retained viewer-known evidence beyond \(K\). It may be empty, a proved sufficient summary, or the relevant slice of the full perfect-recall record.

Let \(\mathcal Z\) be the latent continuation-state space required by the selected field—for example opponent policy type, persistent random tape, or correlated hidden state.

**[DEFINITION]** The admissible augmented latent domain is

\[
\Xi(K,e)
\subseteq
\Phi(K)\times\mathcal Z.
\]

It contains the pairs \(\xi=(\omega,z)\) satisfying every hard reconstruction and field-compatibility constraint.

Let

\[
\beta\in\Delta(\Xi(K,e))
\]

be the viewer’s normalized belief. Rule support gives the ambient allowed domain. \(\beta\) gives weights and may have smaller positive-mass support because of chance-law zeros, policy-likelihood zeros, or earlier conditioning.

**[DEFINITION]** The exact current decision state, relative to fixed rules, field class, utility family, and allowed focal strategy class, is

\[
\boxed{B=(K,e,\beta).}
\]

## 2.5 The noncollapse ledger

The following objects must remain distinct:

| Object | Exact role | Not equivalent to |
|---|---|---|
| \(\omega_0\) | one complete initial deal | a current remainder |
| \(K\) | exact physical/support kernel | a perfect-recall information state |
| \(\Phi(K)\) | rule-compatible current remainders | a probability measure |
| \(e\) | retained viewer-known evidence | hidden field state |
| \(\Xi(K,e)\) | admissible augmented latent domain | a probability law |
| \(\beta\) | belief on that domain | legality |
| \(\sigma\) or \(\Pi\) | continuation field | a rule |
| \(U\) | utility lens | mechanics |
| \(V,Q\) | derived values | physical state fields |

A public action can simultaneously have three different effects:

1. objective physical transition;
2. rule-support restriction and retyping;
3. likelihood reweighting and normalization.

No one of these may silently substitute for another.


## 2.6 Support graph, evidence graph, and planning graph

The exact support normal form is a closed transition state for rule support when the declaration and typed public observation context are supplied. It is not the whole continuation game.

There are exact projections

\[
\text{full augmented continuation process}
\longrightarrow
\text{physical/support-kernel process}
\longrightarrow
\text{support-normal-form process}.
\]

The projections may merge branches because:

- a viewer action can change the physical position while leaving hidden support unchanged;
- two public histories can have the same support and different likelihood weights;
- latent field states can differ over the same physical remainder;
- utility or banked outcome residue can differ over the same support;
- several actions can compile to the same successor support normal form.

Thus the support graph is a load-bearing factor of the plan graph, not a substitute for it. Exact support compression solves the legality-domain problem. It does not by itself solve the evidence, belief, policy, or value problem.

**[INHERITED / BOUNDARY]** A mechanical kernel can be an exact sufficient state for objective play while failing to be the original perfect-recall information state. Any coarser strategic key requires a separate value- or policy-preservation theorem.

---

# 3. Scheme and Fix as typed relational queries

## 3.1 Ambient frames and local slices

A pattern such as “four roles form this boss/precedence arrangement” may be meaningful across several declarations or kernels. A local decision, however, occurs at one exact \(K\).

**[DEFINITION]** An ambient frame \(\mathfrak B\) is a finite declared family of admissible kernels, together with any shared constants and typing data used by a query language.

A fixed-kernel semantics is the slice

\[
\mathfrak B=\{K\}.
\]

The declaration is part of each kernel. A query may bind or compare effective-context roles, but it does not replace the foundation’s nine-declaration algebra with an `Option Pip` surrogate.

## 3.2 Role schema and output interface

Let a finite role schema be

\[
\Sigma=(N_Q,N_C,N_D),
\]

where:

- \(N_Q\) names effective-context roles;
- \(N_C\) names chair roles;
- \(N_D\) names domino roles.

An interpretation into a kernel is a triple

\[
\iota=(\iota_Q,\iota_C,\iota_D)
\]

with codomains \(Q,S,\mathcal D\).

**[DEFINITION]** An output interface is a designated subschema

\[
O\subseteq\Sigma.
\]

Names in \(O\) are returned by the query. Names outside \(O\) are internal existential witnesses.

This distinction is load bearing. Internal proof choices must not accidentally become:

- extra referents;
- extra probability mass;
- tracked identities;
- valued objects;
- public observations.

The corners are:

\[
O=\varnothing
\quad\Rightarrow\quad
\text{Boolean event query},
\]

\[
O=\{e\}
\quad\Rightarrow\quad
\text{one-output role query},
\]

\[
O=\Sigma
\quad\Rightarrow\quad
\text{full witnessed realization query}.
\]

## 3.3 Structural signature

A core structural signature may include:

\[
\begin{array}{lll}
\operatorname{Live}(e),
&\operatorname{Holds}(c,e),
&\operatorname{In}(e,q),\\
\operatorname{Double}(e),
&\operatorname{Beats}(e,f,q),
&\operatorname{Boss}(e,q),\\
\operatorname{Void}(c,q),
&\operatorname{Quota}(c,n),
& e\doteq d,\\
\operatorname{ChairIs}(c,s),
&\operatorname{ContextIs}(q,r),
&\operatorname{Team}(c,t).
\end{array}
\]

The intended interpretations are derived from the exact kernel and concrete world. For example,

\[
\operatorname{In}(e,q)
\iff
F_\delta(\iota_D(e),\iota_Q(q))=1,
\]

\[
\operatorname{Beats}(e,f,q)
\iff
\tau_\delta(\iota_D(e),\iota_Q(q))
>
\tau_\delta(\iota_D(f),\iota_Q(q)),
\]

and

\[
\operatorname{Void}(c,q)
\iff
H_{\iota_C(c)}(x)\cap\widehat\sigma_{\iota_Q(q)}^\delta=\varnothing.
\]

Event predicates are typed separately and derived from \(\tau\) and the public residue:

\[
\operatorname{Leader}(c),
\quad
\operatorname{NextActor}(c),
\quad
\operatorname{LedContext}(q),
\quad
\operatorname{CurrentWinner}(c),
\quad
\operatorname{Played}(e).
\]

Derived predicates such as mastery are registered computations, not independent mutable facts.

The signature may be extended with bounded local-continuation relations—companion, forced-follower, beater-chain, mobility—provided their horizon, information access, and semantic definition are explicit. A predicate that calls the target solver or reads the response class is forbidden target leakage.

## 3.4 Equality patterns

Distinct role names may or may not denote distinct concrete objects. Injectivity is useful inside one branch, but possible identification must remain expressible.

For each role sort, let \(\pi\) be a partition of the names. Quotient the schema by \(\pi\), then interpret the quotient names injectively.

**[THEOREM — equality-pattern completeness]** Every arbitrary interpretation of a finite role schema factors uniquely through its kernel partition followed by an injective interpretation of the quotient schema.

**Proof.** For a map \(f:N\to X\), define \(a\sim b\) exactly when \(f(a)=f(b)\). The induced map \(\bar f:N/{\sim}\to X\) is injective, and \(f\) is the quotient map followed by \(\bar f\). Uniqueness follows because the kernel partition of \(f\) is unique. Apply this independently to each sort. ∎

Thus finite disjunction over equality patterns recovers every noninjective interpretation while keeping each branch cleanly injective.

## 3.5 Scheme cases and Fixes

**[DEFINITION]** A Scheme case is

\[
S=(\pi,\varphi),
\]

where \(\pi\) is an equality pattern and \(\varphi\) is a finite conjunction of structural, event, anchoring, or registered derived atoms over the quotient schema.

**[DEFINITION]** A Fix is a finite disjunction of Scheme cases over one common schema and output interface:

\[
F=S_1\lor\cdots\lor S_r.
\]

The empty Fix is false. Duplicate or semantically contained branches may be removed. A cut adds a conjunct branchwise and removes unsatisfiable branches.

One positive conjunctive case is not a Boolean lattice. The semantic property space is a Boolean algebra; a Fix is a finite disjunctive fragment chosen for explicitness and finite exactness.

Because all domains are finite, Fixes are extensionally complete at the expensive limit **provided** the formula registry can anchor the ambient kernel—or the semantics is restricted to one fixed-kernel slice—and can name every live holder and output referent required to isolate a concrete tuple. Under that explicit completeness hypothesis, any finite set of concrete answer tuples can be represented by a finite disjunction of complete ground cases. The research problem is compression, not bare expressibility.

---

# 4. Answer relations, witness bundles, and certainty

## 4.1 Answer relation

Fix an ambient frame \(\mathfrak B\), a Fix \(F\) over schema \(\Sigma\), and output interface \(O\).

**[DEFINITION]** The answer relation is

\[
\operatorname{Ans}^{O}_{\mathfrak B}(F)
=
\left\{
(K,\omega,\rho):
\begin{array}{l}
K\in\mathfrak B,\ \omega\in\Phi(K),\\
\rho\text{ interprets the output names }O,\\
\exists\iota\supseteq\rho\text{ satisfying one case of }F
\end{array}
\right\}.
\]

The output-answer fiber over one concrete world is

\[
W_F^O(K,\omega)
=
\{\rho:(K,\omega,\rho)\in\operatorname{Ans}^{O}_{\mathfrak B}(F)\}.
\]

The Boolean extension is the projection

\[
\operatorname{Ext}_{\mathfrak B}(F)
=
\{(K,\omega):W_F^O(K,\omega)\ne\varnothing\}.
\]

A role query therefore denotes a finite bundle of answer sets over worlds:

\[
(K,\omega)
\longmapsto
W_F^O(K,\omega).
\]

## 4.2 Boolean equivalence and role equivalence

Two queries can have the same Boolean extension while returning different answers.

Define

\[
F\equiv_{\mathrm{bool}}G
\iff
\operatorname{Ext}(F)=\operatorname{Ext}(G).
\]

For a fixed output interface, a stronger equivalence is

\[
F\equiv_{\mathrm{ans}}G
\iff
\operatorname{Ans}^{O}(F)=\operatorname{Ans}^{O}(G),
\]

or equality up to a declared output-role relabeling.

Boolean equivalence is strictly weaker. A query saying “some master exists” and a query saying “some tile exists” may both be true in every world while returning very different answer fibers.

## 4.3 Ground closures and the answer-level meta-fiber theorem

For a complete interpretation \(\iota\), let \(F[\iota]\) be the ground closure obtained by replacing every role name by its concrete denotation. Let

\[
\operatorname{Fib}_K(F[\iota])
=
\{\omega\in\Phi(K):K\oplus\omega\models F[\iota]\}.
\]

**[THEOREM — answer-level meta-fiber]** The full witnessed realization relation is the disjoint union of the ground fibers tagged by their interpretations:

\[
\operatorname{Ans}^{\Sigma}_{\{K\}}(F)
\cong
\bigsqcup_{\iota}
\left(
\{\iota\}\times\operatorname{Fib}_K(F[\iota])
\right).
\]

Projecting to output names gives the output-answer relation. Projecting away all names gives the Boolean extension.

**Proof.** A witnessed tuple satisfies \(F\) exactly when substitution by its interpretation yields a true ground closure. Tagging by the complete interpretation makes the union disjoint. The two projections are ordinary relational projections. ∎

## 4.4 A hierarchy of certainty

For a fixed local kernel \(K\), define:

**Event possibility**

\[
\exists\omega\in\Phi(K):W_F^O(K,\omega)\ne\varnothing.
\]

**Event certainty**

\[
\forall\omega\in\Phi(K):W_F^O(K,\omega)\ne\varnothing.
\]

**Constant multiplicity**

\[
\exists k\ \forall\omega:\ |W_F^O(K,\omega)|=k.
\]

**Constant answer set**

\[
\exists W_0\ \forall\omega:\ W_F^O(K,\omega)=W_0.
\]

**World-functional reference**

\[
\forall\omega:\ |W_F^O(K,\omega)|=1.
\]

**Identity certainty**

\[
\exists\rho_0\ \forall\omega:\ W_F^O(K,\omega)=\{\rho_0\}.
\]

Belief-almost-sure versions replace “every \(\omega\)” with “\(\beta\)-almost every latent state.” These levels must not share one word such as “known” without qualification.

In particular, event certainty does not license a definite description. The phrase “the master” requires uniqueness, a selector, or a set-valued aggregation convention.


---

# 5. Probability on worlds is not probability on answers

## 5.1 No canonical lift

Let \(B=(K,e,\beta)\), and let \(\mu=(\operatorname{pr}_\omega)_\#\beta\) be the physical-world marginal on \(\Phi(K)\). A structural Scheme normally depends only on the physical world component \(\omega\), so lift its Boolean event to the augmented domain by

\[
E_B(F)
=
\{(\omega,z)\in\Xi(K,e):W_F^O(K,\omega)\ne\varnothing\}.
\]

The belief \(\beta\) gives probabilities to latent worlds. It does not determine how to choose an output answer inside a multi-answer fiber.

**[THEOREM — no canonical answer lift]** Suppose some positive-mass world \(\omega\) has two distinct answers \(\rho_1,\rho_2\in W_F^O(K,\omega)\). Then there exist at least two different probability laws on world–answer pairs with the same physical-world marginal \(\mu\).

**Proof.** On every single-answer world the lift is forced. On the stated world, one lift assigns all conditional answer mass to \(\rho_1\); another assigns it to \(\rho_2\). Both project to the same world mass \(\mu(\omega)\), but they are distinct laws on answer pairs. ∎

Thus answer multiplicity is not hidden probability waiting to be normalized.

## 5.2 Selector kernels

**[DEFINITION]** A selector for query \(F\) is a conditional probability kernel

\[
\chi_{K,\omega}
\in
\Delta(W_F^O(K,\omega))
\]

on every world with a nonempty answer fiber.

It induces the joint law

\[
\widehat\mu_\chi(\omega,\rho)
=
\mu(\omega)\chi_{K,\omega}(\rho).
\]

The world marginal is preserved:

\[
\sum_{\rho\in W_F^O(K,\omega)}
\widehat\mu_\chi(\omega,\rho)
=
\mu(\omega).
\]

A selector may be:

- canonical by an explicitly chosen stable order;
- probabilistic;
- adversarial;
- generated by another latent variable;
- observed by the focal player;
- hidden from the focal player.

These choices define different information structures. None is supplied by existential satisfaction alone.

## 5.3 Naive answer counting and exact bias

Let

\[
m_F(\omega)=|W_F^O(K,\omega)|.
\]

Suppose one incorrectly gives equal primitive mass to each world–answer pair and then projects back to worlds. The resulting world law is

\[
\widetilde\mu_F(\omega)
=
\frac{m_F(\omega)\mu(\omega)}
{\mathbb E_\mu[m_F]}.
\]

**[THEOREM — multiplicity-bias covariance identity]** For every integrable world statistic \(X\),

\[
\boxed{
\mathbb E_{\widetilde\mu_F}[X]
-
\mathbb E_\mu[X]
=
\frac{\operatorname{Cov}_\mu(X,m_F)}
{\mathbb E_\mu[m_F]}.
}
\]

**Proof.** By definition,

\[
\mathbb E_{\widetilde\mu_F}[X]
=
\frac{\mathbb E_\mu[m_FX]}{\mathbb E_\mu[m_F]}.
\]

Subtract \(\mathbb E_\mu[X]\), place over the common denominator, and use

\[
\operatorname{Cov}_\mu(X,m_F)
=
\mathbb E_\mu[m_FX]
-
\mathbb E_\mu[m_F]\mathbb E_\mu[X].
\]

∎

Constant multiplicity makes this particular world-marginal distortion vanish, but it still does not choose a referent.

## 5.4 Fix union is not a probability mixture

For a Fix

\[
F=S_1\lor\cdots\lor S_r,
\]

its answer relation is the set union

\[
\operatorname{Ans}^O(F)
=
\bigcup_i\operatorname{Ans}^O(S_i).
\]

If one concrete answer satisfies two branches, it remains one answer. Branch overlap is not duplicated probability mass. A probabilistic mixture over branches is a separate model requiring explicit branch weights and a branch-selection variable.

## 5.5 Aggregation lenses

A role-valued utility need not select one answer. It may aggregate an answer set.

**[DEFINITION]** An aggregation lens is a typed map

\[
A_x:
\mathcal P_{\mathrm{fin}}(\text{answers})\times\mathcal C
\to R,
\]

where \(x\) is the current latent state and \(\mathcal C\) is the terminal outcome space.

Examples include:

- sum the rewards of every answer;
- value one canonical answer;
- average under a selector \(\chi_x\);
- take the minimum over answers;
- reward capture of at least one answer;
- reward capture of exactly one answer.

Linear selector-based lenses can often be reduced to ordinary tile valuations. Set-level predicates such as “at least one” depend on joint capture events and generally require the full terminal law.

The timing of adversarial or probabilistic selection matters. In general,

\[
\min_\rho\mathbb E[X_\rho]
\ne
\mathbb E[\min_\rho X_\rho].
\]

The first selects before continuation randomness; the second selects after the outcome. They are different games.

---

# 6. Exact dynamics, role persistence, and learning

## 6.1 Typed concrete transition

Let \(X_n\) be the finite set of concrete situations with \(n\) live tiles remaining in hands. A fully typed public observation token \(o\)—actor, played tile, lead/follow/slough classification, and the required public context—induces a partial deterministic map

\[
T_o:D_o\subseteq X_n\to X_{n-1}.
\]

The domain contains exactly the predecessor situations in which the observed action is legal and actor-correct.

For \(A\subseteq X_n\) and \(B\subseteq X_{n-1}\), define

\[
T_{o!}(A)
=
\{T_o(x):x\in A\cap D_o\},
\]

\[
T_o^*(B)
=
\{x\in D_o:T_o(x)\in B\}.
\]

**[THEOREM — image/preimage adjunction]**

\[
T_{o!}(A)\subseteq B
\iff
A\cap D_o\subseteq T_o^*(B).
\]

**Proof.** Both statements say that every executable \(x\in A\) has successor in \(B\). ∎

**[THEOREM — cut/step exchange]**

\[
\boxed{
T_{o!}\bigl(A\cap T_o^*(B)\bigr)
=
T_{o!}(A)\cap B.
}
\]

**Proof.** A successor belongs to the left exactly when it is the image of an executable \(x\in A\) whose image belongs to \(B\), which is exactly membership on the right. ∎

These laws define the semantic contract for any syntactic `step` compiler.

## 6.2 Exact support update

**[INHERITED]** Given the exact support normal form \(N\), declaration, actor, played tile, and led context or lead boundary, the exact successor support is uniquely determined.

For a hidden actor, the native computation is:

\[
\text{force holder edge}
\to
\text{delete slough-forbidden edges}
\to
\text{contract played tile}
\to
\text{matching-supported reduction}.
\]

For a viewer play, the hidden remainder assignment is unchanged, although the physical kernel and live carrier change.

Scheme must not maintain an independent rival holder state. It queries the exact kernel before and after the inherited support update.

## 6.3 Belief update

A public observation updates the augmented belief by legality, modeled likelihood, latent-state transition, and physical pushforward.

In a finite model, schematically,

\[
\beta'(\xi')
\propto
\sum_{\xi}
\beta(\xi)
\mathbf1[o\text{ legal from }\xi]
L_\Pi(o\mid\xi)
K_o(\xi,\xi').
\]

The physical marginal on current remainders alone is generally insufficient if worlds with the same remainder carry different field state or history likelihood.

A forced action contributes no discretionary likelihood ratio once the actor’s information state is fixed, though legality may still remove worlds for an outside observer. A viewer’s own action is an intervention relative to the viewer’s hidden-deal uncertainty when its randomization is independent of that uncertainty.

## 6.4 Rigid transport of output roles

A tile role normally denotes a stable physical domino identity. Let \(r_o\rho\) transport an output assignment through observation \(o\): tile identities persist, chair and context roles follow their declared transport rules, and a role may become dead or cease to satisfy its predicate without changing identity.

Define the lifted transition

\[
\widetilde T_o(x,\rho)
=
(T_o(x),r_o\rho).
\]

For an answer relation \(R\),

\[
\operatorname{Transport}_o(R)
=
\widetilde T_{o!}(R).
\]

A fresh successor query \(R'\) is evaluated independently on the successor state.

## 6.5 Persistence, extinction, and birth

**[DEFINITION]** Relative to transported prior answers \(R\) and fresh successor answers \(R'\), define

\[
\operatorname{Persistent}_o(R,R')
=
\operatorname{Transport}_o(R)\cap R',
\]

\[
\operatorname{Extinct}_o(R,R')
=
\operatorname{Transport}_o(R)\setminus R',
\]

\[
\operatorname{Born}_o(R,R')
=
R'\setminus\operatorname{Transport}_o(R).
\]

These are semantic set operations. They distinguish four questions:

1. Did the same physical object survive?
2. Does it still satisfy the old role predicate?
3. Which new objects satisfy the predicate now?
4. Which old and new witnesses are identical?

Fresh re-query alone cannot answer the first question.

## 6.6 Mastery monotonicity

**[DEFINITION]** A live tile \(d\) is an absolute master in kernel \(K\) when

\[
\operatorname{Master}_K(d)
\iff
 d\in L(K)
\land
\operatorname{THREAT}_\delta(d)\cap L(K)=\varnothing.
\]

**[THEOREM — surviving-master monotonicity]** If \(L'\subseteq L\), \(d\in L'\), and \(d\) is master in \(L\), then \(d\) is master in \(L'\).

**Proof.**

\[
\operatorname{THREAT}_\delta(d)\cap L'
\subseteq
\operatorname{THREAT}_\delta(d)\cap L
=
\varnothing.
\]

Since \(d\in L'\), it remains live and unthreatened. ∎

Thus a surviving absolute master cannot be demoted merely by deleting live tiles. New masters may be born when their last threats disappear.

## 6.7 Hindsight anchoring

Let \(A\) be a later answer predicate such as “output role \(e\) is tile \(1\!:\!0\).” The exact compatible prior answers are

\[
\boxed{
\operatorname{AnchorBack}_o(R,A)
=
R\cap\widetilde T_o^*(A).
}
\]

This is a backward preimage. It filters prior hypotheses using later information. It does not place the later observation into the player’s earlier information state and does not authorize a hindsight-informed earlier action.

## 6.8 Conditioning is not revelation

Analyst conditioning on event \(E\) keeps the player’s policy class fixed:

\[
J(\rho\mid E)
=
\mathbb E_{\beta(\cdot\mid E)}[U\mid\rho].
\]

Player revelation produces a new information state

\[
B^E=(K,e\oplus E,\beta(\cdot\mid E))
\]

and may enlarge the policy class because later actions can condition on \(E\).

Treating analyst conditioning as player revelation leaks hidden information and recreates strategy fusion.

## 6.9 Two orthogonal recursions

Learning moves forward:

\[
(K_t,e_t,\beta_t,R_t)
\longrightarrow
(K_{t+1},e_{t+1},\beta_{t+1},R_{t+1}).
\]

Planning moves backward over possible future observations and decisions:

\[
\text{terminal universal outcomes}
\longrightarrow
\text{policy-contingent continuation objects}
\longrightarrow
Q,V.
\]

These recursions meet at decision states. They are not one recursion run in opposite textual order.

---

# 7. Universal continuation before valuation

## 7.1 Legal terminal outcome space

Let \(\mathcal C\) be the finite set of terminal continuation outcomes reachable from the selected decision problem. A universal outcome records at least:

- the winner partnership of every remaining trick;
- the captor partnership of every physical tile whose capture is relevant;
- any banked universal feature retained at the root;
- every terminal residue required by the selected utility family.

\(\mathcal C\) is defined as the image of legal terminal histories. Conservation and capture/trick consistency therefore hold by construction.

## 7.2 Information-consistent policies

For player \(s\), let \(\mathcal I_s\) be the set of reachable future perfect-recall information states in the contracted continuation. A deterministic contingent policy is a map

\[
\rho_s:\mathcal I_s\to\mathcal A
\]

that chooses one legal action at every information state.

The policy may branch on observations actually received. It may not branch on the hidden world unless the world has been revealed in the declared information structure.

Let

\[
\mathcal R(K,e)
=
\prod_s\mathcal R_s(K,e)
\]

be the finite policy-profile set for the selected continuation model.

## 7.3 Universal outcome kernel

**[DEFINITION]** The valuation- and utility-free continuation kernel is

\[
\mathcal O_{K,e}:
\Xi(K,e)\times\mathcal R(K,e)
\to
\Delta(\mathcal C).
\]

For latent state \(\xi\) and policy profile \(\rho\), \(\mathcal O_{K,e}(\xi,\rho)\) is the induced terminal-outcome law, integrating only chance and field randomness not already conditioned into \(\xi\).

Given decision state \(B=(K,e,\beta)\), define the belief-integrated game form

\[
\Gamma_B(\rho)
=
\int_{\Xi(K,e)}
\mathcal O_{K,e}(\xi,\rho)
\,\beta(d\xi).
\]

This is the central universal continuation object.

## 7.4 Fixed-field specialization: Walt

Fix focal player \(m\) and a continuation field \(\sigma_{-m}\) for the other actors. For \(\rho_m\in\mathcal R_m\), set

\[
\Gamma_B^{\sigma}(\rho_m)
=
\Gamma_B(\rho_m,\sigma_{-m}).
\]

This map assigns one terminal law to every information-set-consistent focal policy. It is count-blind and utility-blind.

## 7.5 Universal evaluation theorem

Let \(w\) be a valuation parameter and let \(U_w:\mathcal C\to\mathbb R\) be any bounded measurable utility obtained from the retained terminal outcome.

Define

\[
J_B(\rho_m;w,U)
=
\sum_{c\in\mathcal C}
\Gamma_B^{\sigma}(\rho_m)(c)U_w(c).
\]

For legal root action \(a\),

\[
Q_B(a;w,U)
=
\max_{\rho_m:\rho_m(B)=a}
J_B(\rho_m;w,U),
\]

and

\[
V_B(w,U)=\max_a Q_B(a;w,U).
\]

**[THEOREM — universal fixed-field sufficiency]** The policy-indexed law \(\Gamma_B^{\sigma}\) determines the exact fixed-field best response and every root action value for every bounded terminal utility readable from \(\mathcal C\).

**Proof.** For each lawful policy, the law contains the complete distribution of every terminal quantity read by the utility. Integrating gives its exact value. Maximizing over the finite policy class, with or without a fixed root action, gives \(V\) and \(Q\). ∎

Independent private randomization cannot improve the optimum: conditioning on the random tape expresses the randomized value as a convex combination of deterministic contingent-policy values.

## 7.6 Strategy fusion boundary

Choosing the best action separately in each hidden world and then averaging is generally not an element of \(\mathcal R_m\). It is a different information structure.

The distinction can be written as

\[
\max_{\rho\text{ information-consistent}}
\mathbb E_\beta[U(\omega,\rho)]
\quad\le\quad
\mathbb E_\beta
\left[
\max_{\rho_\omega}U(\omega,\rho_\omega)
\right].
\]

The gap is the value of the omitted information relaxation under the selected field and utility.

Opposite seats share utility but have different private hands and different information states. A centralized partnership controller that sees both hands defines a different extensive-form game unless an equivalence theorem proves the extra information redundant.

## 7.7 Solution-operator boundary

The same universal outcome layer supports several named operators:

- perfect-information max/min backward induction;
- fixed stochastic field evaluation;
- fixed-field information-set best response;
- teacher or benchmark policy evaluation;
- a later equilibrium operator.

Theorems for one operator do not silently transfer to another. In particular, piecewise-affine support-function results for a finite fixed-field best response do not automatically imply piecewise-affine values for a valuation-dependent mixed-strategy equilibrium.

---

# 8. Universal additive outcomes and the valuation gauge

## 8.1 Terminal capture feature

Fix focal partnership \(T\). For terminal outcome \(c\), let

\[
t_T(c)=\#\{\text{remaining tricks won by }T\},
\]

and

\[
x_{T,d}(c)=
\mathbf1[d\text{ is captured by }T\text{ during the selected continuation}].
\]

In future-increment mode, only tiles still live at the root can contribute a one. In full-hand mode, the banked capture feature is added as in §8.5.

The universal additive feature is

\[
\phi_T(c)
=
\left(
 t_T(c),
 (x_{T,d}(c))_{d\in\mathcal D}
\right).
\]

For any selected continuation measured consistently from its root,

\[
\boxed{
\sum_{d\in\mathcal D}x_{T,d}(c)=4t_T(c).
}
\]

Each won trick in that continuation contributes exactly its four played tiles. In full-hand mode the same identity holds after banked and future features are added.

## 8.2 Free additive outcome object

Let

\[
E=\{\star\}\sqcup\mathcal D
\]

and let

\[
M=\mathbb N^{(E)}
\]

be the free commutative monoid on one trick generator \(e_\star\) and one generator \(e_d\) for every physical domino.

The terminal additive outcome is

\[
\Phi_T(c)
=
 t_T(c)e_\star
+
\sum_{d:x_{T,d}(c)=1}e_d.
\]

Let \(R\) be a commutative additive monoid. Choose a trick coefficient \(b\in R\) and tile valuation

\[
w:\mathcal D\to R.
\]

There is a unique monoid homomorphism

\[
\operatorname{ev}_{b,w}:M\to R
\]

with

\[
\operatorname{ev}_{b,w}(e_\star)=b,
\qquad
\operatorname{ev}_{b,w}(e_d)=w(d).
\]

**[THEOREM — universal additive factorization]** Every additive terminal score assigning \(b\) per trick and \(w(d)\) per captured tile factors uniquely as

\[
P_{T,b,w}(c)
=
\operatorname{ev}_{b,w}(\Phi_T(c))
=
 bt_T(c)+\sum_dx_{T,d}(c)w(d).
\]

**Proof.** This is the universal property of the free commutative monoid. ∎

Straight count is one sparse \(w\). No-count is \(w=0\). One arbitrarily valued tile is \(w=\lambda e_d\). All 28 independently valued tiles are the full coefficient family.

## 8.3 Exact additive gauge

Because legal outcomes satisfy \(\sum_dx_d=4t\), the coefficient pair \((b,w)\) is not identifiable.

For any scalar \(c\),

\[
(b,w)
\sim
(b-4c,w+c\mathbf1),
\]

since

\[
(b-4c)t+
\sum_d(w(d)+c)x_d
=
bt+
\sum_dw(d)x_d.
\]

**[THEOREM — additive gauge invariance]** Every pair on one affine gauge line defines the same score on every legal terminal capture feature:

\[
(b,w)\sim(b-4c,w+c\mathbf1).
\]

**Proof.** The displayed calculation uses only the universal legal identity \(\sum_dx_d=4t\). ∎

It is therefore natural to work in the known quotient

\[
(\mathbb R\times\mathbb R^{\mathcal D})/
\langle(-4,\mathbf1)\rangle,
\]

which has dimension \(28\). This removes one proved universal redundancy. A claim that no additional linear relation survives on a narrower selected terminal domain is a separate span theorem and must be proved for that domain.

## 8.4 Symmetric mode and anisotropy

For real coefficients, choose

\[
u=\frac1{28}\sum_{d\in\mathcal D}w(d),
\qquad
\eta=w-u\mathbf1.
\]

Then

\[
w=u\mathbf1+\eta,
\qquad
\sum_d\eta(d)=0.
\]

Then

\[
bt+
\sum_dw(d)x_d
=
(b+4u)t+
\sum_d\eta(d)x_d.
\]

The uniform component only changes the effective trick coefficient. The zero-sum field \(\eta\) contains the 27 independent tile anisotropies.

If every tile has value \(u\) and \(b=1\), then a team winning \(k\) tricks scores

\[
(1+4u)k.
\]

For \(u>-1/4\), raw expected-point action order is the same as trick-only action order. Fixed contract thresholds or mark utility are not automatically preserved unless the threshold is transported consistently.

## 8.5 Mid-hand banked features

At a mid-hand decision, already won tricks and captured tile identities may matter to later valuation.

Two exact modes are available.

### Future-increment mode

Only future capture features are revalued. The banked past is an action-independent constant under the original utility.

### Full-hand valuation-universal mode

Retain a banked feature

\[
\alpha_T
=
\left(
 t_T^{\mathrm{bank}},
 (x_{T,d}^{\mathrm{bank}})_d
\right).
\]

Then

\[
Q_{\mathrm{full}}(a;b,w)
=
\langle(b,w),\alpha_T\rangle
+
Q_{\mathrm{future}}(a;b,w).
\]

The banked term is independent of \(a\), so action order is unchanged. A scalar Straight-count bank is not sufficient for arbitrary revaluation of past tile identities.

## 8.6 Nonlinear utility hierarchy

Expected additive utility needs only expected feature vectors. Threshold, risk, role-set, and match utilities may require more.

The sufficiency hierarchy is:

1. terminal feature for deterministic additive evaluation;
2. expected feature for expected additive evaluation;
3. terminal feature law for arbitrary bounded utility of that feature;
4. full universal terminal law for utilities reading additional residue;
5. policy-indexed game form for optimization under changing utility.

No smaller quotient is universal without a theorem for the selected utility family.



## 8.7 Two meanings of changing the valuation

The phrase “change the tile values” names two different mathematical interventions.

### Payoff relabeling

Hold the already observed history, current belief, continuation field, and information structure fixed. Replace only the evaluation map

\[
w\mapsto U_w.
\]

This is the clean parametric experiment used by the support-function theory. Legal mechanics and rule support are unchanged after declaration.

### Common-knowledge scoring change

Suppose every player knew the new valuation before bidding and play. Then the field itself may depend on the valuation:

\[
w
\longmapsto
\Pi_w
\longmapsto
L_{\Pi_w}(h\mid\xi)
\longmapsto
\beta_w
\longmapsto
\Gamma_{B_w}
\longmapsto
Q_w.
\]

The likelihood of the already observed history can change, so the posterior can change even when the same physical worlds remain rule-compatible. At a history with zero probability under the new field, an explicit off-path assessment is required.

The payoff-relabeling experiment isolates continuation geometry. The common-knowledge experiment studies a different game and must re-run inference and policy behavior.

## 8.8 Valuation is not ordinary rule-support filtering

For a fixed Straight physical game, assigning a different public value to a tile does not normally make a hidden deal illegal. Hence the rule fiber

\[
\Phi(K)
\]

is unchanged by payoff relabeling.

A valuation can filter **role interpretations** when a query requires a role to carry a specified value. For a role-value constraint \(b\), define

\[
\operatorname{Ans}^{O}_{w,b}(F)
=
\{(K,\omega,\rho)\in\operatorname{Ans}^{O}(F):
 w(\rho(e))=b(e)\text{ for every constrained output role }e\}.
\]

This removes answers or world witnesses that fail the role label. It is a query restriction, not a new legality rule.

## 8.9 No universal context-free domino value

**[INHERITED]** The foundation contains a legal witness in which the same physical action under the same declaration and mechanical endpoint has opposite exact values under different hidden worlds and beliefs.

Therefore no scalar

\[
v(d)
\]

depending only on physical domino identity can equal exact strategic action value in every information state, belief, field, and utility.

Tile valuation \(w(d)\) in this document is a payoff coefficient. It is not the strategic value \(Q(B,d)\).

---

# 9. Expected additive policy geometry

## 9.1 Policy feature sets

Fix decision state \(B\), focal player \(m\), fixed valuation-independent field \(\sigma_{-m}\), and expected additive utility.

For each deterministic information-consistent focal policy \(\rho\), define its expected feature vector

\[
\mu_\rho
=
\mathbb E_{c\sim\Gamma_B^{\sigma}(\rho)}[\phi_T(c)]
\in\mathbb R^{29}.
\]

For a legal root action \(a\), let

\[
S_{B,a}
=
\{\mu_\rho:\rho(B)=a\}
\]

be the finite action feature set.

**[DEFINITION]** The action policy polytope is

\[
P_{B,a}
=
\operatorname{conv}S_{B,a}.
\]

The global root polytope is

\[
P_B
=
\operatorname{conv}\bigcup_aP_{B,a}.
\]

Independent focal randomization generates convex combinations and therefore adds no point outside these polytopes.

## 9.2 Support-function representation

For coefficient vector

\[
v=(b,w)
\]

modulo the additive gauge, let

\[
h_P(v)=\max_{x\in P}\langle v,x\rangle
\]

be the support function.

**[THEOREM — action-polytope value]**

\[
\boxed{
Q_B(a;v)=h_{P_{B,a}}(v),
\qquad
V_B(v)=h_{P_B}(v).
}
\]

**Proof.** Each deterministic policy has value \(\langle v,\mu_\rho\rangle\). A linear functional has the same maximum over a finite set and its convex hull. Restricting to a root action gives \(Q\); taking the union over actions gives \(V\). ∎

## 9.3 Polytope completeness

**[THEOREM — all-additive completeness]** For compact convex polytopes \(P,Q\),

\[
h_P(v)=h_Q(v)
\quad\text{for every }v
\iff
P=Q.
\]

**Proof.** Equality of polytopes implies equality of support functions. Conversely, if \(P\ne Q\), finite-dimensional strict separation gives a linear functional whose maximum differs on the two compact convex sets. ∎

Thus an action polytope is the complete finite signature of that action for every expected additive valuation direction in the selected belief/field/information problem.

For a restricted valuation family \(W\), only the restricted support function \(h_P|_W\) matters, so a smaller quotient may be exact.

## 9.4 One-tile rays and piecewise-affine values

For one specially valued tile \(d\), choose

\[
v_\lambda=v_0+\lambda e_d,
\qquad
\lambda\ge0.
\]

Each policy contributes one line

\[
L_\rho(\lambda)
=A_\rho+B_\rho\lambda,
\]

where \(A_\rho\) is its baseline value and \(B_\rho\) is its signed expected capture coordinate for \(d\).

Hence

\[
Q_B(a;\lambda)
=
\max_{\rho(B)=a}
(A_\rho+B_\rho\lambda).
\]

This is a continuous convex piecewise-affine function with rational breakpoints when the field, belief, and feature data are rational.

For several valued tiles, lines become affine hyperplanes. The normal fan of \(P_{B,a}\) partitions valuation space into regions exposing the same policy face.

## 9.5 Baseline faces and directional derivatives

For polytope \(P\) and baseline direction \(v\), define the exposed face

\[
F_P(v)
=
\{x\in P:\langle v,x\rangle=h_P(v)\}.
\]

**[THEOREM — support-function directional derivative]** For perturbation \(u\),

\[
\boxed{
D^+h_P(v;u)
=
\lim_{\lambda\downarrow0}
\frac{h_P(v+\lambda u)-h_P(v)}{\lambda}
=
h_{F_P(v)}(u).
}
\]

**Proof.** Policies outside \(F_P(v)\) have a strictly smaller baseline value and cannot become optimal under an infinitesimal perturbation. Among baseline-optimal points, the first-order term is maximized by \(u\). A standard finite-polytope argument makes this exact. ∎

Thus a one-tile defect first interrogates the baseline-optimal face. A new tile value can resolve a baseline tie at \(0^+\) even when there is no positive interior crossing.

## 9.6 Cone dominance

Let \(C\) be a permitted valuation cone. A policy feature \(y\) dominates \(x\) on \(C\) when

\[
\langle v,y-x\rangle\ge0
\quad\forall v\in C.
\]

If the inequality is strict for some allowed direction, \(x\) is never uniquely exposed on \(C\) and may be pruned for that purpose.

This is purpose-relative. A point dominated on nonnegative “more points” directions may be exposed when negative or threshold-sensitive valuations are allowed.

## 9.7 Active and inert valuation directions

**[DEFINITION]** A direction \(u\) is action-active for \((B,a,v_0,\Lambda)\) when

\[
\lambda\mapsto h_{P_{B,a}}(v_0+\lambda u)
\]

has more than one exposed affine segment on \(\Lambda\). It is root-active when the optimal root-action correspondence changes.

**[THEOREM — sufficient inertness criterion]** If

\[
\langle u,x\rangle=c
\qquad
\forall x\in P_{B,a},
\]

then

\[
Q_B(a;v_0+\lambda u)
=
Q_B(a;v_0)+\lambda c
\]

and the maximizing policy correspondence is independent of \(\lambda\).

**Proof.** Every policy receives the same added affine term. ∎

Activity is relative to the action, information structure, field, baseline, and permitted parameter domain. It is not an intrinsic property of a physical tile.

## 9.8 Valuation-family refinement

Let \(W\subseteq W'\) be valuation families, and define purpose equivalence by equality of the selected response for every valuation in the family.

**[THEOREM — monotone valuation refinement]**

\[
\equiv_{\mathcal P,W'}
\subseteq
\equiv_{\mathcal P,W}.
\]

**Proof.** Equality for every valuation in the larger family implies equality on its subset. ∎

Adding independent tile values can split response classes but cannot merge classes already distinguishable under the smaller family.

**[THEOREM — full independent terminal separation]** If two legal terminal outcomes have equal additive value for every tile valuation and the trick coefficient is observable, then their full trick-and-capture feature vectors are equal.

**Proof.** Evaluate at zero tile value to identify trick count. Then evaluate each coordinate valuation \(e_d\) to identify every capture indicator. ∎

Earlier in the game, the corresponding complete additive signature is the policy feature polytope, not one expected feature vector after optimization has discarded alternatives.


## 9.9 Symbolic parametric backward induction

The valuation-first construction is not limited to fixed-field best response.

Consider a finite complete-information continuation whose internal operators are finite maxima, finite minima, or fixed valuation-independent expectations. Let the terminal leaves be affine functions of the valuation coefficients. Build a symbolic expression recursively:

\[
E::=
\operatorname{affine}(\phi)
\mid
\max(E_1,\ldots,E_r)
\mid
\min(E_1,\ldots,E_r)
\mid
\sum_ip_iE_i.
\]

**[THEOREM — evaluation commutes with backward induction]** For every valuation \(v\), evaluating the symbolic expression after the structural recursion equals scalar backward induction performed directly under \(v\).

**Proof.** Terminal equality is additive factorization. At each predecessor, evaluation commutes definitionally with finite max, min, and the fixed linear expectation operator. Induction on remaining play completes the proof. ∎

**[COROLLARY — piecewise-affine structure]** Every resulting value and action-value function is continuous piecewise affine on finite-dimensional valuation space.

For a fixed-field best response, the functions are convex because they are support functions. For max/min perfect-information recursion they need not be globally convex. If mixed equilibrium strategies are recomputed at every valuation, rational or more complicated pieces can arise; that is outside this theorem.

---

# 10. Information partitions as policy-gluing geometry

This section is the principal mathematical addition forced by Experiment 4.

## 10.1 Decision nodes and information partitions

Fix a finite continuation, focal player \(m\), belief, field, root action convention, and universal feature map.

Let \(D_m\) be the finite set of future focal decision nodes across every latent world and reachable observation history. A node includes the concrete latent world and objective continuation history, but the player need not observe all of it.

**[DEFINITION]** An information structure \(\mathcal I\) is an equivalence relation on \(D_m\). We admit only valid information classes: nodes in one class have the same acting player and the same legal action labels, or else a declared bijective action transport. Nodes in one class are indistinguishable to the player and therefore require one transported action choice.

A deterministic policy is a legal map

\[
\rho:D_m\to\mathcal A
\]

constant on every \(\mathcal I\)-class.

If \(\mathcal I'\) is finer than \(\mathcal I\), write

\[
\mathcal I\preceq\mathcal I'.
\]

The finer structure distinguishes at least as many nodes. It imposes fewer action-equality constraints.

## 10.2 Policy-gluing inclusion

Let \(\mathcal R_{\mathcal I}(a)\) be the deterministic policies consistent with \(\mathcal I\) and root action \(a\).

**[THEOREM — information refinement enlarges the policy class]** If

\[
\mathcal I\preceq\mathcal I',
\]

then

\[
\mathcal R_{\mathcal I}(a)
\subseteq
\mathcal R_{\mathcal I'}(a).
\]

**Proof.** A policy constant on every coarse information class is automatically constant on each finer subclass. ∎

Let \(P_a^{\mathcal I}\) be the convex hull of expected features generated by \(\mathcal R_{\mathcal I}(a)\).

**[COROLLARY — information-refinement polytope inclusion]**

\[
\boxed{
P_a^{\mathcal I}
\subseteq
P_a^{\mathcal I'}.
}
\]

Therefore

\[
Q_a^{\mathcal I'}(v)
\ge
Q_a^{\mathcal I}(v)
\]

for every additive valuation direction \(v\).

This is the exact geometry of strategy-fusion prevention: hidden information glues policy coordinates that full revelation would allow to vary independently.

## 10.3 Hidden, continuation-revealed, and root-revealed treatments

Let

\[
\mu=(\operatorname{pr}_\omega)_\#\beta
\]

be the physical-world marginal and let \(\Omega=\operatorname{supp}_+(\mu)\). For each \(\omega\in\Omega\), condition the augmented model on physical world \(\omega\); any residual field or chance randomness remains inside that conditional model. For every root action \(a\) legal throughout the root information state, let

\[
P_{\omega,a}
\]

be the focal continuation feature polytope when \(\omega\) is revealed before every later focal decision.

Define three information treatments.

### H — actual hidden information

The focal player sees only the original information record. Let

\[
P_a^H
\]

be the action polytope generated by hidden-information-consistent policies.

### C — common root, continuation revealed

The root action \(a\) is common across worlds. Immediately afterward, the complete world is revealed before any later focal decision.

Because the continuation policy may now be chosen independently in every world, the aggregate action polytope is the weighted Minkowski sum

\[
\boxed{
P_a^C
=
\sum_{\omega\in\Omega}
\mu(\omega)P_{\omega,a}.
}
\]

Here

\[
\sum_\omega\mu(\omega)P_{\omega,a}
=
\left\{
\sum_\omega\mu(\omega)x_\omega:
 x_\omega\in P_{\omega,a}
\right\}.
\]

**[THEOREM — revealed-continuation Minkowski formula]** The displayed set is exactly the convex hull of expected features attainable under continuation revelation.

**Proof.** A deterministic revealed policy independently selects one deterministic local policy feature in every world, and world averaging gives the corresponding weighted sum. Taking the convex hull of all such selections equals the weighted Minkowski sum of the local convex hulls. Equivalently, arbitrary points of the displayed polytopes are realized by independent private mixtures or by convexification after deterministic selection. ∎

### F — world revealed before the root

The root action may also depend on \(\omega\). Define the local root polytope

\[
P_\omega^{\mathrm{root}}
=
\operatorname{conv}\bigcup_aP_{\omega,a},
\]

and

\[
\boxed{
P^F
=
\sum_\omega\mu(\omega)P_\omega^{\mathrm{root}}.
}
\]

For common-root treatment C and hidden treatment H, define

\[
P^C
=
\operatorname{conv}\bigcup_aP_a^C,
\qquad
P^H
=
\operatorname{conv}\bigcup_aP_a^H.
\]

## 10.4 The information-polytope chain

**[THEOREM — nested information polytopes]**

\[
\boxed{
P_a^H\subseteq P_a^C
\quad\text{for every root }a,
}
\]

and

\[
\boxed{
P^H\subseteq P^C\subseteq P^F.
}
\]

**Proof.** A hidden policy is a continuation-revealed policy that ignores the revealed world, giving the first inclusion. A hidden root policy is therefore a common-root revealed policy, giving \(P^H\subseteq P^C\). A common root is a special case of a world-dependent root choice, giving \(P^C\subseteq P^F\). Convexification preserves inclusion. ∎

Consequently,

\[
Q_a^C(v)\ge Q_a^H(v),
\]

and

\[
V^F(v)\ge V^C(v)\ge V^H(v).
\]

## 10.5 Information prices

Define the action-specific continuation-information price

\[
\boxed{
G_a^{\mathrm{cont}}(v)
=
Q_a^C(v)-Q_a^H(v)
=
h_{P_a^C}(v)-h_{P_a^H}(v).
}
\]

Define the optimized continuation-information price

\[
G^{\mathrm{cont}}(v)
=
V^C(v)-V^H(v),
\]

the root-choice information price

\[
G^{\mathrm{root}}(v)
=
V^F(v)-V^C(v),
\]

and total revelation value

\[
G^{\mathrm{total}}(v)
=
V^F(v)-V^H(v).
\]

**[THEOREM — exact decomposition]**

\[
\boxed{
G^{\mathrm{total}}(v)
=
G^{\mathrm{cont}}(v)
+
G^{\mathrm{root}}(v).
}
\]

**Proof.** Add and subtract \(V^C(v)\). ∎

Every price is nonnegative by the polytope inclusions.

## 10.6 Zero-information criterion

Let \(P\subseteq Q\) be compact convex sets. Let

\[
F_Q(v)=\{x\in Q:\langle v,x\rangle=h_Q(v)\}.
\]

**[THEOREM — zero information iff an exposed policy is implementable]**

\[
\boxed{
h_Q(v)=h_P(v)
\iff
F_Q(v)\cap P\ne\varnothing.}
\]

**Proof.** If the intersection contains \(x\), then

\[
h_P(v)\ge\langle v,x\rangle=h_Q(v),
\]

while \(P\subseteq Q\) gives the reverse inequality. Conversely, if the support values agree, a maximizer of \(v\) over compact \(P\) is also a maximizer over \(Q\), so it lies in the intersection. ∎

Applied to \(P_a^H\subseteq P_a^C\), revelation has zero continuation value exactly when some continuation-revealed optimal policy feature is already attainable by one hidden-information-consistent policy.

## 10.7 Value monotonicity does not imply breakpoint monotonicity

Information refinement monotonically enlarges attainable value. It does not preserve:

- vertices;
- exposed faces;
- normal fans;
- policy breakpoints;
- root-switch prices.

A larger polytope can have more, fewer, or entirely different breakpoints along a valuation ray. The reported Experiment 4 supplies an exact finite instance: a hidden three-segment action curve becomes a revealed nine-segment curve, while both hidden internal prices disappear.

Thus the valid general statement is:

\[
\boxed{
\text{more information raises support functions but need not preserve their normal fans.}
}
\]

## 10.8 Perfect information and fixed-field revelation are different operators

Per-world perfect-information minimax changes both information and the continuation operator when compared with a fixed stochastic field. Treatment C changes only focal information and keeps the field fixed. C is therefore the controlled causal comparison for information value.

Worldwise minimax affineness does not imply revealed fixed-field affineness. The experiment reports precisely this split.

## 10.9 Information-state count is not information value

Let

\[
E_B(a)=|\mathcal I^m_{B,a}|
\]

be the number of reachable future focal information states after root action \(a\). This is an exact computational observable. It is not a utility or information value.

A branch can have thousands of information states and zero revelation value when one world-independent continuation policy is optimal everywhere. Conversely, a much smaller information tree can contain strongly world-contingent optimal actions.

The meaningful object is incompatibility among optimal actions across glued nodes, not the raw node count.

---

# 11. Constellations on the correct carrier

## 11.1 Orbit, Scheme cell, and purpose class

Three useful objects must not share one unqualified definition.

### Orbit constellation

For a proved transformation group \(G\) acting on complete transported situations,

\[
\operatorname{Orb}_G(x)=\{g\cdot x:g\in G\}.
\]

The declaration, seats, partnerships, orientation, evidence, belief, field, output roles, and utility orientation must be transported as required. Arbitrary chair or pip permutations are not automatically Straight 42 symmetries.

### Scheme cell

A Scheme/Fix denotes a world set, answer relation, or decision-state observable fiber. It can contain part of one orbit, several orbits, or several strategic classes.

### Purpose constellation

Fix a carrier \(X\) and an exact response map

\[
R_{\mathcal P}:X\to Y.
\]

Define

\[
x\equiv_{\mathcal P}y
\iff
R_{\mathcal P}(x)=R_{\mathcal P}(y).
\]

The equivalence class \([x]_{\mathcal P}\) is the constellation for that carrier and purpose.

There is no purpose-free canonical constellation.

## 11.2 Possible carriers

The carrier may be:

- an objective physical state;
- an exact support kernel;
- a concrete hidden world;
- a decision state \(B=(K,e,\beta)\);
- a world–output-answer tuple;
- an information structure together with its policy polytope;
- a dynamic predictive state.

A world-level perfect-information response class is not automatically a hidden-information decision class. A support quotient is not automatically a belief quotient. A value class is not automatically a policy class.

## 11.3 Mechanical future equivalence

For a finite deterministic machine with action alphabet \(A\) and selected output contract, define

\[
x\equiv_{\mathrm{future}}y
\iff
\forall w\in A^*:\operatorname{resp}(x,w)=\operatorname{resp}(y,w).
\]

**[INHERITED]** This is a right congruence, and the reachable quotient is the unique smallest deterministic exact machine for the selected output contract.

The theorem is exact for the declared deterministic carrier and outputs. It does not automatically minimize a stochastic belief process or an imperfect-information policy problem.

## 11.4 Additive policy-polytope constellations

For a fixed belief, field, information structure, and root action, define

\[
B\equiv_{\mathrm{poly},a}B'
\iff
P_{B,a}=P_{B',a}
\]

under a declared feature/action transport.

By support-function completeness, this is exactly equality of every expected additive action value for every valuation direction.

A weaker restricted-family class uses

\[
h_{P_{B,a}}|_W=h_{P_{B',a}}|_W.
\]

A still weaker action-correspondence class preserves only the maximizing root set. These quotients can have very different cardinalities.

## 11.5 Law and policy classes

For nonlinear utility, equal expected-feature polytopes may be insufficient. One may require equality of:

- policy-indexed terminal feature laws;
- full terminal laws;
- optimal policy correspondences;
- predictive observation/transition kernels.

Optimized scalar value equality is weakest. Two states can have the same \(V\) and opposite optimal actions.

## 11.6 Symmetry and valuation stabilizers

If \(G\) is a structural automorphism group and \(w\) a fixed valuation, the surviving valuation symmetry group is

\[
G_w=\{g\in G:w(gd)=w(d)\ \forall d\}.
\]

A generic all-distinct valuation may have trivial stabilizer. A valuation family \(W\) has stabilizer

\[
G_W=\{g:gW=W\}.
\]

Orbit splitting under valuation and strategic-class splitting under valuation are related but need not coincide.


## 11.7 Exact seat gauges and reflection boundary

Under complete transport, seat rotations

\[
r_k(s)=s+k\pmod4
\]

form an exact \(C_4\) symmetry of Straight 42. Odd rotations exchange fixed team labels, so utility orientation must be transported.

Reflection does not commute with the fixed clockwise successor relation and is not an automorphism of the oriented extensive game. It becomes an exact coordinate gauge only after adjoining an orientation variable

\[
\eta\in\{+1,-1\},
\qquad
\operatorname{next}_\eta(s)=s+\eta.
\]

Rotations together with orientation-transporting reflected frames form a \(D_4\) coordinate gauge on the oriented family. This distinction matters for Scheme orbit claims: a syntax-level chair permutation is not a game symmetry until every orientation-dependent object is transported.

---

# 12. Compression science after Experiment 4

## 12.1 Static descriptor factorization

Let \(X\) be a finite domain, \(R^*:X\to Y\) an exact target response, and \(D:X\to Z\) a descriptor.

**[DEFINITION]** \(D\) is purpose-sound when

\[
D(x)=D(y)
\Longrightarrow
R^*(x)=R^*(y).
\]

It is purpose-exact when the descriptor fibers equal the target-response fibers.

**[THEOREM — factorization criterion]** \(D\) is purpose-sound exactly when there exists a unique

\[
\bar R:\operatorname{im}D\to\operatorname{im}R^*
\]

such that

\[
\boxed{R^*=\bar R\circ D.}
\]

**Proof.** If \(D\) is sound, define \(\bar R(D(x))=R^*(x)\); soundness makes this well defined. The converse follows by substitution. ∎

This is the exact static compression target.

## 12.2 Three distinct compression questions

### Extensional world compression

\[
C_{\mathrm{world}}(D)
=
\frac{|X|}{|\operatorname{im}D|}.
\]

This measures how many worlds are merged on average. It says nothing about formula size or update cost.

### Intensional relational compression

This measures the size and generality of the program, Scheme/Fix, or role grammar defining \(D\). A small generic relation can be useful even when its evaluated outputs separate many worlds.

No absolute scalar is canonical without a language and cost model. Candidate costs include atom count, syntax-tree size, role count, branch count, or proof-term complexity.

### Decision-geometry compression

A huge contingent-policy class may induce a polytope with very few exposed policies or valuation regions. Relevant counts include:

- number of distinct policy feature points;
- number of vertices;
- number of faces exposed by a selected valuation cone;
- number of ray segments;
- number of exact action regions.

This can be dramatic even when world compression is poor.

Exact support-normal-form compression is a fourth, already established semantic layer beneath these research measures.

## 12.3 What the static experiments established

The reported 90-world Experiment 3A found a four-atom descriptor with 33 purpose-sound cells for an eight-class world-level root-\(Q\) target:

\[
90\to33\to8.
\]

The winning atoms were control-shaped: companion, decisive-context partner strength, forced-follower team, and beater team.

Experiment 4B tested genericized versions on 12 fresh trick-six kernels and 78 valued-tile tasks. The four-shape descriptor was sound on 30 of 72 fresh tasks, compared with 21 of 72 for a holder-only baseline. This is genuine partial signal.

But 26 of 78 tasks contained differently labelled world pairs that no atom in the frozen 34-template registry could separate. Two missing resolutions dominated:

1. successor-trick context not visible from root-context roles;
2. causal seat position lost by partnership-only ownership.

Adding all-suit successor roles and seat-resolved roles removed the vocabulary ceiling but collapsed median held-out world compression to \(1\). The exact static world-response target then required effective world reconstruction.

## 12.4 What the failure does not prove

Experiment 4B targeted

\[
R_{\mathrm{PI}}(\omega),
\]

the perfect-information minimax root response of each individual world.

A static descriptor pure for \(R_{\mathrm{PI}}\) is sufficient for averaging oracle responses. It is not necessary for an exact hidden-information solve, because the hidden problem optimizes one policy across an information partition:

\[
\max_{\rho\text{ common}}
\mathbb E_\beta[U(\omega,\rho)]
\ne
\mathbb E_\beta
\left[
\max_{\rho_\omega}U(\omega,\rho_\omega)
\right].
\]

Two worlds with different oracle responses can still be merged in an exact hidden model if they have the same abstract observation/transition behavior and are treated by the same lawful policy.

Thus the 4B result is:

> A flat descriptor preserving every world’s perfect-information response did not transfer compactly in the tested vocabulary.

It is not:

> Straight 42 hidden information has no compact exact representation.

## 12.5 Dynamic control skeleton

The static control skeleton described root-local roles. The counterexamples show that the natural object must update when the actual successor context and actor position become known.

**[DEFINITION]** A dynamic control skeleton is a typed relational state \(D_t\) with update

\[
D_{t+1}
=
\delta_D(D_t,a_t,o_{t+1})
\]

or, more generally, a finite stochastic update kernel over descriptor states.

Its roles may include:

- current led-context strength;
- actual next actor and relative seat position;
- rigidly transported valued-tile and companion roles;
- forced-follow and slough mobility;
- beater and overtake chains in the context that is actually led;
- newly observed void and possession facts;
- support-normal-form summaries needed for exact transition.

The descriptor should instantiate successor-context roles when they become causally relevant rather than encode every possible future suit at the root.

## 12.6 Exact controlled lumpability target

The correct exact target is dynamic predictive sufficiency, not static world-response purity.

Let \(X\) be a finite latent state space at a recursion boundary, \(A(x)\) the legal focal actions, \(O\) the observation alphabet, \(R\) a finite universal-feature-increment alphabet, and

\[
K_a(x;r,o,x')
\]

the joint kernel for immediate universal feature increment \(r\), next observation \(o\), and successor latent state \(x'\).

Let

\[
d:X\to Y
\]

be a candidate latent descriptor.

**[DEFINITION]** \(d\) is strongly controlled-lumpable for the selected interface when, whenever \(d(x)=d(y)\):

1. \(A(x)=A(y)\);
2. for every legal \(a\), feature increment \(r\), observation \(o\), and descriptor state \(y'\),

\[
\sum_{x':d(x')=y'}K_a(x;r,o,x')
=
\sum_{x':d(x')=y'}K_a(y;r,o,x').
\]

This defines an induced abstract kernel

\[
\bar K_a(d(x);r,o,y').
\]

**[THEOREM — exact dynamic compression]** If \(d\) is strongly controlled-lumpable, then for every initial belief \(\beta\), every positive-probability observation history, and every policy measurable with respect to the preserved observation record and abstract belief:

1. the abstract belief \(\bar\beta=d_\#\beta\) updates using \(\bar K\) alone;
2. the joint law of observation histories and accumulated universal features is the same in the concrete and abstract models;
3. every utility of the preserved terminal outcome has the same policy value;
4. every optimization over the same abstract-policy class has the same \(V\) and \(Q\).

**Proof.** Condition 2 makes the distribution of \((r,o,d(x'))\) depend only on \(d(x)\), so \(\bar K\) is well defined. Induct on the finite continuation grade. At each step, concrete mass within one descriptor class produces the same abstract output/successor distribution, allowing exact pushforward and Bayesian normalization on positive-probability observations. The induction preserves the full joint law of accumulated features and observations. Utility equality and optimized value equality follow. ∎

**[BOUNDARY]** Strong lumpability is sufficient and deliberately stringent. Weaker belief-dependent, policy-relative, or bisimulation-style quotients may exist and require separate theorems.

## 12.7 Scheme/Fix as a control-skeleton language

Scheme/Fix can serve as the syntax for \(d\) when:

- output roles are explicit;
- rigid and fresh role semantics are distinguished;
- exact support remains authoritative;
- every derived continuation atom declares its horizon and information access;
- the step compiler is proved to preserve the intended answer relation;
- the induced descriptor transition satisfies the selected lumpability or factorization theorem.

The compression search should therefore produce not only a pure static partition but:

\[
\boxed{
\text{descriptor semantics}
+
\text{exact update law}
+
\text{response-preservation proof}.
}
\]

## 12.8 Decision-geometry compression is already large

The reported horizon-three hidden solve had:

- 168 binary future focal information states after root \(0\!:\!0\);
- 7,848 after root \(2\!:\!1\);
- 504 after root \(3\!:\!2\).

The formal deterministic policy spaces are correspondingly enormous. Yet across all twelve tested directions, only 15 distinct hidden policies were exposed, and the \(2\!:\!1\) branch had one affine continuation and zero revelation value.

This is exact evidence of a different compression:

\[
\boxed{
\text{vast information-consistent policy space}
\longrightarrow
\text{small exposed policy geometry}.
}
\]

That geometry is directly relevant to decision making even when a world-pure static quotient is unavailable.

## 12.9 Counterexample-guided synthesis

For a finite candidate language \(\mathcal L\), target \(R^*\), and descriptor budget:

1. compute exact response labels;
2. evaluate registered atoms or transducer states;
3. find a minimum-cost descriptor satisfying the target condition;
4. when it fails, emit a pair of states merged by the candidate but separated by the target;
5. classify the missing causal relation;
6. add a generic, horizon-declared atom or update rule;
7. retest on held-out kernels before changing the target language again.

Static purity search is an exact finite hitting-set problem. Dynamic synthesis additionally checks transition and observation-kernel compatibility.

The aim is not to force human-looking folklore into the model. It is to let exact continuation behavior reveal which relations form the game’s transferable ontology.


---

# 13. Mastery as a worked relational example

## 13.1 Four different predicates

The word “master” can conceal several distinct claims.

### Absolute live-set mastery

\[
\operatorname{Master}_K(d)
\iff
 d\in L(K)
\land
\operatorname{THREAT}_\delta(d)\cap L(K)=\varnothing.
\]

No live tile can beat \(d\) when \(d\) is led.

### World-relative sure-winning lead

For concrete world \(x\) and seat \(s\),

\[
\operatorname{SureLeadWin}_x(s,d)
\]

means \(s\) currently holds \(d\), leading \(d\) is legal, and every legal continuation of the trick leaves \(s\)’s partnership winning it. This can be weaker than absolute mastery because partner-held threats may be irrelevant, or stronger hypotheses about actual holders may settle the trick.

### Belief-support sure-winning lead

\[
\operatorname{KnownSureLeadWin}_B(s,d)
\iff
\forall\xi\in\operatorname{supp}_+(\beta):
\operatorname{SureLeadWin}_{\xi}(s,d).
\]

This is an epistemic validity statement over the selected belief support.

### Field-relative winning probability

\[
\Pr_B(d\text{ wins when led})
\]

integrates the selected field and belief. It can lie strictly between zero and one even when neither structural certainty nor impossibility holds.

These are respectively structural, control, epistemic, and probabilistic predicates. Strategic optimality is a fifth layer.

## 13.2 The \(1\!:\!0\) last-trick witness

Consider 4s trump with live set

\[
L=
\{1\!:\!0,5\!:\!5,3\!:\!2,6\!:\!3\}.
\]

The exploratory Lean source verifies:

\[
\operatorname{Master}_{4s}(1\!:\!0;L),
\]

\[
\neg\operatorname{Master}_{4s}(3\!:\!2;L),
\]

and adding live \(1\!:\!1\) removes mastery from \(1\!:\!0\).

The low-looking \(1\!:\!0\) is master because mastery is a relation to the current live algebra, not an intrinsic pip magnitude.

If the partnership leading \(1\!:\!0\) captures the trick, the universal additive increment is

\[
e_\star
+e_{1:0}+e_{5:5}+e_{3:2}+e_{6:3}.
\]

Under arbitrary tile valuation \(w\), the trick is worth

\[
1+w(1\!:\!0)+w(5\!:\!5)+w(3\!:\!2)+w(6\!:\!3).
\]

Under Straight count it is

\[
1+0+10+5+0=16.
\]

The structural fact that \(1\!:\!0\) is master is valuation-independent. Whether arranging to lead it was the best earlier policy need not be.

## 13.3 Truth versus reference

In the trick-six experiment, the query

\[
F_{\mathrm{master}}:
\exists e\,[\operatorname{Live}(e)\land\operatorname{Master}(e)]
\]

was true in every one of 90 worlds, but its tile-answer fiber was the four-element set

\[
\{0\!:\!0,2\!:\!2,4\!:\!4,5\!:\!2\}
\]

in every world.

Thus the event was certain and the answer set was constant, yet the reference was not unique. “The master” required a lens.

The reported role-valued experiment compared five lenses. For an extra value \(\lambda\):

| Lens | \(Q(0\!:\!0)\) | \(Q(2\!:\!1)\) | Crossing |
|---|---:|---:|---:|
| all four answers valued | \(\frac23+\frac85\lambda\) | \(-\frac23-\frac43\lambda\) | \(-\frac5{11}\) |
| least-ID canonical answer | \(\frac23+\lambda\) | \(-\frac23-\frac13\lambda\) | \(-1\) |
| uniform selector | \(\frac23+\frac25\lambda\) | \(-\frac23-\frac13\lambda\) | \(-\frac{20}{11}\) |
| worst selected answer | \(\frac23+\frac15\lambda\) | \(-\frac23-\frac13\lambda\) | \(-\frac52\) |
| at least one answer captured | \(\frac23+\frac13\lambda\) | \(-\frac23-\frac13\lambda\) | \(-2\) |

No crossing lies in \(\lambda\ge0\), but the slope changes substantially with the selection/aggregation semantics.

The linear lenses imply the per-answer signed capture vectors

\[
\bar g_{0:0}
=
\left(1,\frac15,\frac15,\frac15\right),
\]

\[
\bar g_{2:1}
=
-\frac13\mathbf1.
\]

The “at least one” lens is not a linear functional of these marginals; it depends on their joint terminal law.

---

# 14. Reported finite experiments

## 14.1 Evidentiary status and validation chain

Everything in this section is **[EXPERIMENTAL RECEIPT — reported]**.

The reports state:

- exact Python integers and `Fraction` arithmetic, with no floating-point solver arithmetic;
- replay of the complete source hands before experiments;
- exact validation of winners, follow legality, trick points, and hand outcomes;
- a disclosed piecewise-linear interval-endpoint bug found by the first multisegment domain;
- bit-identical v0.1/v0.2 outputs after the repair;
- independent scalar reimplementations for the new PWL results;
- continuity checks at every claimed breakpoint;
- whole-ray nonnegativity certificates for all information-price functions;
- independent brute-force reconstructions of fresh support fibers;
- exact descriptor soundness, entropy, and minimality checks.

Experiment 4 reports 6,390 independent scalar comparisons for Arm 4A and 1,872 for Arm 4B, all matching. It records 437 of 437 continuity checks and 827 whole-ray segment certificates.

These checks make the reports strong exploratory evidence. Promotion still requires adding the programs, records, and certificate checker to the project’s verifier-receipt process.

## 14.2 Experiment 1 — one valued tile at two tricks

### Domain

- receipt hand 0;
- threes trump;
- start of trick 6;
- focal seat S1 on lead with \(\{0\!:\!0,2\!:\!1\}\);
- six unseen tiles distributed \(2,2,2\) across the hidden seats;
- exact hidden fiber size \(90\);
- valued tile \(d=4\!:\!1\);
- focal partnership \(T_1=\{\mathrm{S1},\mathrm{S3}\}\);
- continuation utility

\[
\Psi(\lambda)
=
(\text{future tricks }T_1-\text{future tricks }T_0)
+
\lambda\,
(\mathbf1[T_1\text{ captures }d]-\mathbf1[T_0\text{ captures }d]).
\]

### Fixed-field hidden solve

Under uniform belief on the 90 worlds and a uniform-random legal field for the other seats,

\[
Q^H(0\!:\!0;\lambda)
=
\frac23+\frac15\lambda,
\]

\[
Q^H(2\!:\!1;\lambda)
=
-\frac23-\frac13\lambda.
\]

The lines cross at

\[
\lambda=-\frac52,
\]

so \(0\!:\!0\) is optimal for every \(\lambda\ge0\). Its feature point dominates on both expected-trick and capture coordinates.

The full terminal-law reconstruction produced 11 distinct outcomes after \(0\!:\!0\) and 26 after \(2\!:\!1\), yet the additive projection reduced each law to one line.

### Worldwise perfect-information response census

Every world/root response was one affine line. The 90 worlds formed eight parametric root-\(Q\) classes of sizes

\[
(26,22,16,12,8,2,2,2).
\]

At \(\lambda=0\), only four exact root-value vectors remained. Thus one valued tile refined the restricted root-\(Q\) partition

\[
4\to8.
\]

The optimal-action correspondence refined

\[
2\to3.
\]

In eight worlds the two roots tied at \(\lambda=0\), but \(2\!:\!1\) became strictly optimal for every \(\lambda>0\). This was a boundary tie resolution at \(0^+\), not a positive interior crossing.

### Location is not capture control

The partner held \(4\!:\!1\) in six worlds where the opponents captured it under optimal play. Opponents held it in four worlds where the focal team captured it. Holder location did not determine capture destiny.

A holder-only descriptor needed the holders of five of the six unseen tiles to become sound for the eight response classes—effectively complete world reconstruction.

## 14.3 Experiment 2 — witness bundles and role valuation

Two queries were audited on the same 90-world kernel.

### Absolute-master query

\[
F_{\mathrm{master}}:
\exists e\,[\operatorname{Live}(e)\land\operatorname{Master}(e)].
\]

- Boolean extension: 90 of 90 worlds;
- tile-answer realizations: 360;
- multiplicity: exactly four in every world.

### Opponent-beater query

\[
F_{\mathrm{beat21}}:
\exists c,e\,[
\operatorname{Opponent}(c)
\land\operatorname{Holds}(c,e)
\land\operatorname{Beats}(e,2\!:\!1)
].
\]

- Boolean extension: 90 of 90 worlds;
- answer realizations: 180;
- multiplicities: one answer in 18 worlds, two in 54, three in 18.

Both events were certain, but their answer bundles were different.

A deliberate multiplicity-weighting error changed the fixed-field action lines to

\[
\widetilde Q(0\!:\!0)
=
\frac8{15}+\frac1{10}\lambda,
\]

\[
\widetilde Q(2\!:\!1)
=
-\frac56-\frac{47}{120}\lambda.
\]

The optimum did not flip on \(\lambda\ge0\), but the bias was measurable and exactly predicted by the multiplicity covariance identity.

A Fix overlap audit found two 60-world branches with intersection 36 and union 84:

\[
60+60-36=84,
\]

not the naive branch sum 120.

### Rigid transport and fresh query

Before the actual trick 6, the masters were

\[
\{0\!:\!0,2\!:\!2,4\!:\!4,5\!:\!2\}.
\]

After plays \(0\!:\!0,5\!:\!2,4\!:\!4,2\!:\!0\):

- persistent master: \(2\!:\!2\);
- extinct masters: three played tiles;
- fresh masters: \(\{2\!:\!2,4\!:\!2\}\);
- newly born master: \(4\!:\!2\).

The public trick reduced exact support

\[
90\to6,
\]

and the later anchor “the \(4\!:\!1\)-holder is S2” reduced the compatible prior set

\[
6\to2.
\]

## 14.4 Experiment 3A — first static response compression

The 90-world eight-class target was used as a supervised exact synthesis domain. A 22-observable vocabulary contained holder facts and bounded local-control relations.

Exhaustive search over subsets of size at most four found eight minimal four-observable solutions. One was

\[
D=
\{\operatorname{comp41},
\operatorname{s3max2},
\operatorname{team}(2\!:\!0),
\operatorname{team}(4\!:\!2)\}.
\]

It produced 33 cells, each pure for the eight-class root-\(Q\) target:

\[
90\text{ worlds}
\to
33\text{ descriptor cells}
\to
8\text{ responses}.
\]

No registered descriptor of size at most three was sound, even for the coarser three-class action target.

The semantic world-compression ratio was

\[
\frac{90}{33}=\frac{30}{11},
\]

while the descriptor remained a strict refinement of the exact target by factor

\[
\frac{33}{8}=4.125.
\]

Every minimal solution contained a companion relation and decisive-suit strength. The result suggested a control skeleton rather than a holder coordinate.

## 14.5 Experiment 3B — first positive hidden-information breakpoint

### Domain

- receipt hand 0;
- start of trick 5;
- S1 holds \(\{0\!:\!0,2\!:\!1,3\!:\!2\}\);
- \(3\!:\!2\) is the last live trump;
- nine unseen tiles;
- exact fiber size \(1,680\);
- three root actions;
- twelve live-tile valuation directions.

### Perfect-information minimax census

Across

\[
1680\times3\times12=60{,}480
\]

world/root/direction solves, every root \(Q\) was affine. There were zero multisegment perfect-information curves in this domain.

### Hidden fixed-field solve

For each of the four unseen tiles without a 2-pip,

\[
\{1\!:\!1,4\!:\!1,4\!:\!4,5\!:\!1\},
\]

the hidden fixed-field action values were

\[
Q^H(0\!:\!0;\lambda)
=
\begin{cases}
\frac{37}{21}+\frac{22}{35}\lambda,
&0\le\lambda<\frac15,\\[1mm]
\frac{26}{15}+\frac{27}{35}\lambda,
&\frac15\le\lambda<4,\\[1mm]
\frac{176}{105}+\frac{11}{14}\lambda,
&4\le\lambda,
\end{cases}
\]

\[
Q^H(2\!:\!1;\lambda)
=
\frac53+\frac{20}{21}\lambda,
\]

\[
Q^H(3\!:\!2;\lambda)
=
\frac{37}{21}+\frac47\lambda.
\]

The globally optimal root switched at

\[
\boxed{\lambda^*=\frac7{19}}.
\]

The \(0\!:\!0\) action had internal policy changes at \(1/5\) and \(4\), each trading baseline trick value for a higher capture slope.

The eight other directions were affine under H. At this information treatment, follow obligations appeared to pin their capture coordinates.

The future focal information-state counts were

\[
168,
\qquad
7848,
\qquad
504
\]

after roots \(0\!:\!0,2\!:\!1,3\!:\!2\), respectively.

## 14.6 Experiment 4A — controlled information refinement

Experiment 4A held fixed:

- the 1,680 worlds;
- uniform belief;
- the same uniform-random legal field for the other seats;
- utility and valuation directions;
- the three root actions.

Only the focal player’s information changed through treatments H, C, and F.

### Hidden treatment H

The curves above were reproduced exactly.

### Continuation revelation C

For the four free-tile directions,

\[
Q^C(0\!:\!0;\lambda)
=
\begin{cases}
\frac{68}{35}+\frac{151}{210}\lambda,&0\le\lambda<\frac14,\\
\frac{233}{120}+\frac{76}{105}\lambda,&\frac14\le\lambda<\frac13,\\
\frac{163}{84}+\frac{611}{840}\lambda,&\frac13\le\lambda<\frac12,\\
\frac{543}{280}+\frac{613}{840}\lambda,&\frac12\le\lambda<\frac23,\\
\frac{1621}{840}+\frac{125}{168}\lambda,&\frac23\le\lambda<1,\\
\frac{1577}{840}+\frac{223}{280}\lambda,&1\le\lambda<\frac32,\\
\frac{3127}{1680}+\frac{113}{140}\lambda,&\frac32\le\lambda<2,\\
\frac{3097}{1680}+\frac{457}{560}\lambda,&2\le\lambda<3,\\
\frac{307}{168}+\frac{23}{28}\lambda,&3\le\lambda.
\end{cases}
\]

Meanwhile

\[
Q^C(2\!:\!1;\lambda)
=
\frac53+\frac{20}{21}\lambda
=
Q^H(2\!:\!1;\lambda).
\]

The common-root optimum switched from \(0\!:\!0\) to \(2\!:\!1\) at

\[
\boxed{\lambda=\frac{177}{131}}.
\]

The hidden breakpoints \(1/5\), \(7/19\), and \(4\) were not C breakpoints. The hidden three-segment action became a revealed nine-segment action with new prices

\[
\left\{
\frac14,\frac13,\frac12,\frac23,1,\frac32,2,3
\right\}.
\]

Thus revelation increased value and enriched the normal fan while replacing the specific hidden prices.

### Root revelation F

The fully root-revealed envelope had 42–53 segments in the four free directions. There was no single root correct in every world; the envelope averaged many world-specific root switches.

### Exact information prices

At \(\lambda=0\),

\[
G^{\mathrm{cont}}(0)
=
\frac{19}{105},
\]

\[
G^{\mathrm{root}}(0)
=
\frac{4051}{45360},
\]

\[
G^{\mathrm{total}}(0)
=
\frac{12259}{45360}.
\]

The exact decomposition held as PWL functions for all twelve directions.

For root \(2\!:\!1\),

\[
\boxed{G_{2:1}^{\mathrm{cont}}(\lambda)\equiv0}
\]

on the whole nonnegative ray in all twelve directions. After leading \(2\!:\!1\), the focal player retains the last live trump and the blank double; playing the trump next wins both remaining tricks in every world. Revelation cannot improve the continuation.

The world-contingency census at \(\lambda=1\) was:

| Root | Hidden information states | States whose C-optimal action depends on world |
|---|---:|---:|
| \(0\!:\!0\) | 168 | 168 |
| \(2\!:\!1\) | 7,848 | 0 |
| \(3\!:\!2\) | 504 | 504 |

This is a direct counterexample to using information-state count as information value.

### Control directions under revelation

Under H, all eight control directions were affine. Under C and F, seven of eight became multisegment. Only the focal player’s own last trump \(3\!:\!2\) remained affine in all three treatments.

Therefore “free versus pinned tile” is not an information-independent physical classification. Activity is relative to the selected information polytope.

## 14.7 Experiment 4B — transfer and vocabulary failure

### Corpus

Thirteen receipt hands supplied trick-six kernels. Hand 0 was the design kernel; twelve fresh kernels remained. All were pip-trump hands, spanning pip declarations \(0,1,3,4,5,6\). No doubles-trump or no-trump transfer claim was made.

Each kernel contributed all six unseen valued-tile directions, giving 78 tasks and 3,882 worldwise perfect-information solves. Fiber sizes ranged from 6 to 90.

### Frozen 34-template language

The registry included:

- literal and structural companion features;
- partner/opponent contextual strength;
- forced-follower control;
- beater-chain control;
- one-trick valued-tile mobility;
- holder team and seat baselines.

### Direct transfer

The four genericized control shapes were sound for the exact root-\(Q\) signature on

\[
30/72
\]

fresh tasks, compared with

\[
21/72
\]

for the holder-only baseline. On the held-out half the comparison was \(14/36\) versus \(8/36\).

This is partial transfer signal, not general sufficiency.

### Registry ceiling

For 26 of 78 tasks, differently labelled worlds existed that no frozen atom could separate. Five of hand 0’s other six valuation directions were already ceiling-blocked.

Among 256 unseparable pairs:

- 153 differed only by a swap between the two seats of one partnership;
- the remaining dominant failure involved tile relations in the successor trick’s led context, which need not be a root context.

The two missing resolutions were therefore:

1. successor-context relations;
2. seat-resolved causal position within a partnership.

Adding all-suit boss/runner-up roles and seat-resolved versions removed the ceiling on the training set. The full repaired registry became sound on all 36 held-out tasks, but median world compression fell to

\[
1.
\]

It reconstructed the world.

### Perfect-information affineness census

Across all 13 trick-six kernels and 78 tasks, the report counted 7,764 world-level root-\(Q\) curves and found zero multisegment curves. Combined with Experiment 3B, the reported short-suffix census contains

\[
60{,}480+7{,}764=68{,}244
\]

affine worldwise perfect-information curves and no multisegment example.

**[CONJECTURE — deliberately narrow]** One-tile parametric perfect-information minimax action values may be affine on a substantial class of short Straight 42 suffixes. The current evidence does not prove this and does not identify the exact class.

## 14.8 Consolidated empirical conclusions

The experiments support the following structural picture.

1. A one-tile defect can refine world responses immediately at \(0^+\).
2. Holder location is a weak predictor of capture control.
3. Role truth, answer multiplicity, selection, and probability are distinct.
4. A compact static control descriptor can exist locally.
5. The same static vocabulary need not transfer across contexts and causal seat positions.
6. Hidden information forms a glued policy polytope.
7. Revelation enlarges that polytope but does not preserve its normal fan or breakpoints.
8. Information value can be zero in a branch with the largest information tree.
9. Worldwise oracle-response purity can demand world reconstruction even when the hidden policy geometry is small.
10. The next exact compression target is a dynamic predictive quotient, not a larger flat root feature vector.


---

# 15. Compact formal interface

This section collects the integrated object without explanatory prose.

## 15.1 Physical and epistemic layer

\[
\begin{aligned}
\mathsf{Pip}&:=\operatorname{Fin}7,\\
\mathsf{Domino}&:=\operatorname{Sym}^2(\mathsf{Pip}),\\
\mathsf{Seat}&:=\mathbb Z/4\mathbb Z,\\
\mathsf{Decl}&:=\mathsf{Pip}\sqcup\{\mathrm{DT},\mathrm{NT}\},\\
\mathsf{Context}&:=\operatorname{Fin}8,\\
\mathsf{RuleAlg}(\delta)&:=(\widehat\sigma_q^\delta,F_\delta,\ell_\delta,\tau_\delta),\\
\mathsf{SupportNF}&:=N,\\
\mathsf{Kernel}_{\mathcal A}&:=(\delta,H_m,N,\tau,\alpha_{\mathcal A}),\\
\mathsf{World}(K)&:=\Phi(K),\\
\mathsf{Latent}(K,e)&:=\Xi(K,e),\\
\mathsf{Decision}&:=B=(K,e,\beta).
\end{aligned}
\]

## 15.2 Relational query layer

\[
\begin{aligned}
\mathsf{Schema}&:=\Sigma=(N_Q,N_C,N_D),\\
\mathsf{Output}&:=O\subseteq\Sigma,\\
\mathsf{EqPattern}&:=\pi,\\
\mathsf{SchemeCase}&:=(\pi,\varphi),\\
\mathsf{Fix}&:=\text{finite disjunction of cases},\\
\mathsf{Ans}^{O}_{\mathfrak B}(F)
&:=\{(K,\omega,\rho):\exists\iota\supseteq\rho,\ K\oplus\omega,\iota\models F\},\\
W_F^O(K,\omega)
&:=\{\rho:(K,\omega,\rho)\in\mathsf{Ans}^{O}(F)\},\\
\mathsf{Ext}(F)
&:=\{(K,\omega):W_F^O(K,\omega)\ne\varnothing\}.
\end{aligned}
\]

## 15.3 Answer probability and aggregation

\[
\begin{aligned}
\mu&:=(\operatorname{pr}_\omega)_\#\beta,\\
\chi_{K,\omega}
&\in\Delta(W_F^O(K,\omega)),\\
\widehat\mu_\chi(\omega,\rho)
&:=\mu(\omega)\chi_{K,\omega}(\rho),\\
m_F(K,\omega)
&:=|W_F^O(K,\omega)|,\\
\widetilde\mu_F(\omega)
&:=\frac{m_F(K,\omega)\mu(\omega)}{\mathbb E_\mu[m_F]},\\
A_x&:\mathcal P_{\mathrm{fin}}(W_F^O(x))\times\mathcal C\to R.
\end{aligned}
\]

## 15.4 Forward dynamics

\[
\begin{aligned}
T_o&:D_o\subseteq X_n\to X_{n-1},\\
T_{o!}(A)&:=\{T_o(x):x\in A\cap D_o\},\\
T_o^*(B)&:=\{x\in D_o:T_o(x)\in B\},\\
\widetilde T_o(x,\rho)&:=(T_o(x),r_o\rho),\\
\mathsf{Persistent}_o(R,R')&:=\widetilde T_{o!}(R)\cap R',\\
\mathsf{Extinct}_o(R,R')&:=\widetilde T_{o!}(R)\setminus R',\\
\mathsf{Born}_o(R,R')&:=R'\setminus\widetilde T_{o!}(R),\\
\mathsf{AnchorBack}_o(R,A)&:=R\cap\widetilde T_o^*(A),\\
\mathsf{BeliefStep}_o(K,e,\beta)&:=(K',e',\beta').
\end{aligned}
\]

## 15.5 Universal continuation layer

\[
\begin{aligned}
\mathsf{Terminal}&:=\mathcal C,\\
\mathsf{Policies}&:=\mathcal R(K,e),\\
\mathcal O_{K,e}&:\Xi(K,e)\times\mathcal R(K,e)\to\Delta(\mathcal C),\\
\Gamma_B(\rho)&:=\int\mathcal O_{K,e}(\xi,\rho)\,\beta(d\xi),\\
\Gamma_B^\sigma(\rho_m)&:=\Gamma_B(\rho_m,\sigma_{-m}).
\end{aligned}
\]

## 15.6 Additive layer

\[
\begin{aligned}
\phi_T(c)&:=\left(t_T(c),(x_{T,d}(c))_d\right),\\
\sum_dx_{T,d}(c)&=4t_T(c),\\
[(b,w)]&\in
(\mathbb R\times\mathbb R^{\mathcal D})/\langle(-4,\mathbf1)\rangle,\\
\mu_\rho&:=\mathbb E_{c\sim\Gamma_B^\sigma(\rho)}[\phi_T(c)],\\
P_{B,a}&:=\operatorname{conv}\{\mu_\rho:\rho(B)=a\},\\
Q_B(a;v)&:=h_{P_{B,a}}(v),\\
V_B(v)&:=h_{\operatorname{conv}\cup_aP_{B,a}}(v),\\
D^+Q_B(a;v,u)&:=h_{F_{P_{B,a}}(v)}(u).
\end{aligned}
\]

## 15.7 Information-refinement layer

\[
\begin{aligned}
\mathcal I_H&\preceq\mathcal I_C\preceq\mathcal I_F,\\
P_a^H&\subseteq P_a^C,\\
P^H&\subseteq P^C\subseteq P^F,\\
P_a^C&=\sum_\omega\mu(\omega)P_{\omega,a},\\
P^F&=\sum_\omega\mu(\omega)\operatorname{conv}\bigcup_aP_{\omega,a},\\
G_a^{\mathrm{cont}}(v)&:=h_{P_a^C}(v)-h_{P_a^H}(v),\\
G^{\mathrm{cont}}(v)&:=h_{P^C}(v)-h_{P^H}(v),\\
G^{\mathrm{root}}(v)&:=h_{P^F}(v)-h_{P^C}(v),\\
G^{\mathrm{total}}(v)&:=h_{P^F}(v)-h_{P^H}(v).
\end{aligned}
\]

## 15.8 Constellation and compression layer

\[
\begin{aligned}
x\equiv_{\mathcal P}y
&\iff R_{\mathcal P}(x)=R_{\mathcal P}(y),\\
\mathsf{Constellation}_{\mathcal P}(x)&:=[x]_{\mathcal P},\\
\mathsf{Sound}(D,R^*)&\iff D(x)=D(y)\Rightarrow R^*(x)=R^*(y),\\
\mathsf{Exact}(D,R^*)&\iff\ker D=\ker R^*,\\
R^*&=\bar R\circ D\quad\text{when }D\text{ is sound},\\
\mathsf{Lumpable}(d)&\iff
\text{legal sets and }(r,o,d(x'))\text{ kernels agree within }d\text{-classes}.
\end{aligned}
\]

---

# 16. Proof-assistant program

The cleanest Lean development should follow semantic dependency, not the chronology of discovery.

## 16.1 General strategy

The first formalization should remain finite and explicit.

- Use finite types for pips, dominoes, seats, declarations, contexts, hands, worlds, policies, observations, and terminal outcomes.
- Define semantics before optimized encodings.
- Prove extensional equalities before bit-packing refinements.
- Keep support, belief, query semantics, strategic policy, and convex geometry in separate modules.
- Introduce experiment data only through verified certificates or reflection, never as axioms.

A useful first geometric representation is the finite feature set

\[
S_{B,a}=\{\mu_\rho\}
\]

with

\[
Q(v)=\max_{x\in S_{B,a}}\langle v,x\rangle.
\]

The convex-hull theorem can be added afterward. This avoids requiring a large convex-analysis layer before the finite strategic semantics are stable.

## 16.2 Proposed module graph

```text
Texas42/Basic
Texas42/Declaration
Texas42/Trick
Texas42/Deal
Texas42/PlayState
Texas42/History
Texas42/Mechanical
Texas42/Fiber
Texas42/SupportNormalForm
Texas42/Reachability

Texas42/Belief/Finite
Texas42/Strategic/Information
Texas42/Strategic/Policy
Texas42/Strategic/UniversalOutcome

Texas42/Scheme/Schema
Texas42/Scheme/EqualityPattern
Texas42/Scheme/Formula
Texas42/Scheme/Answer
Texas42/Scheme/Selector
Texas42/Scheme/Dynamics

Texas42/Valuation/Feature
Texas42/Valuation/Gauge
Texas42/Valuation/FiniteSupport
Texas42/Valuation/PWL
Texas42/InformationGeometry/Refinement

Texas42/Constellation/Purpose
Texas42/Compression/Static
Texas42/Compression/Lumpability
Texas42/Certificates/Probe
```

The import direction should be strict:

- `Basic` through `Reachability` do not import belief, Scheme, or strategic solvers;
- Scheme imports the exact physical/support semantics, never the reverse;
- valuation imports universal outcomes and finite policies;
- compression imports the semantic layers it preserves;
- certificates import the checker, never the core definitions.

## 16.3 Phase A — import the inherited foundation

Formalize or import:

1. \(\mathbb P,\mathcal D,S,T_0,T_1\);
2. all nine declarations;
3. called/powered/effective incidences;
4. led context, follow relation, rank, tier, trick key;
5. legal objective transition and graded termination;
6. viewer history and current-remainder map;
7. capacity-cell fiber;
8. support-normal-form compile/decode equality;
9. typed support update;
10. finite belief update and fixed-field best-response existence.

The integrated theory should not redefine any of these with a smaller exploratory surrogate.

## 16.4 Phase B — Scheme semantics

### B1. Role schema and references

Define sorted finite name types and constant/variable references.

### B2. Equality patterns

Represent a branch by setoid/partition data or a quotient-name type. Prove the finite equality-pattern completeness theorem.

### B3. Formula semantics

Start with a finite predicate registry whose semantics is a function of the exact kernel and world. Derived predicates should be definitions, not constructors carrying independent truth.

### B4. Output interface and answer relation

Define output projection directly. Prove:

- answer relation equals projection of full realizations;
- Boolean extension equals nonempty answer fiber;
- ground-closure/meta-fiber theorem;
- Fix union semantics;
- finite extensional completeness.

### B5. Certainty and selectors

Formalize the certainty hierarchy and prove no canonical answer lift in the finite PMF setting. Prove selector marginal preservation and the multiplicity-bias identity.

## 16.5 Phase C — role dynamics

Define typed concrete transitions and lifted answer transitions. Prove:

- direct/inverse image adjunction;
- cut/step exchange;
- history-level honest-answer survival;
- rigid transport laws for stable tile identities;
- persistence/extinction/birth partition;
- hindsight anchoring by preimage;
- surviving-master monotonicity.

A syntactic Scheme/Fix step compiler is exact only after proving denotational equality with lifted direct image.

## 16.6 Phase D — universal continuation and policy consistency

Define future focal decision nodes and information partitions explicitly.

A policy can be represented as a dependent function from information-state IDs to legal actions. Prove:

- finiteness of deterministic contingent policies;
- randomized-policy domination by deterministic policies under independent private randomization;
- strategy-fusion exclusion by typing;
- universal terminal-law sufficiency;
- partnership information boundary.

## 16.7 Phase E — additive valuation

Define the finite capture feature over \(\mathbb Q\) first. Prove:

- four-tiles-per-trick conservation;
- free-monoid additive factorization;
- coefficient gauge;
- centered anisotropy decomposition;
- full independent valuation separation;
- banked-feature decomposition.

Then define finite policy feature sets and prove the max-of-dot-products formula.

## 16.8 Phase F — one-parameter PWL theory

For rational affine lines, define exact upper envelopes on \(\mathbb Q_{\ge0}\) or on rational interval records. Prove:

- evaluation correctness;
- continuity;
- exact breakpoints;
- exposed-policy labels;
- boundary tie versus interior crossing;
- cone-dominance pruning.

The earlier `_combine` defect shows why endpoint ownership and interval invariants should be proved, not only tested.

## 16.9 Phase G — information geometry

Represent an information structure as a partition of finite decision nodes. Prove:

1. refinement enlarges the policy set;
2. feature-set and polytope inclusion;
3. continuation-revealed weighted Minkowski formula;
4. \(P^H\subseteq P^C\subseteq P^F\);
5. support-function information-price inequalities;
6. exact total-price decomposition;
7. zero-information exposed-face criterion.

The theorem can be proved first for finite feature sets, then restated for convex hulls.

## 16.10 Phase H — dynamic compression

Define a finite controlled kernel with universal feature increments and observations. Formalize strong controlled lumpability and prove exact pushforward filtering and value preservation by induction.

Then instantiate candidate Scheme/Fix descriptors. The desired theorem shape is:

\[
\mathsf{SchemeDescriptorSound}(D)
\land
\mathsf{SchemeDescriptorLumpable}(D)
\Longrightarrow
Q_D=Q_{\mathrm{exact}}.
\]

Static response purity and dynamic lumpability should remain separate theorem families.

## 16.11 Phase I — experimental certificates

A certificate schema should contain:

```text
kernel identifier and reconstruction data
world enumeration/count certificate
field and belief identifier
policy or policy-map witness
terminal feature law
rational affine segments
breakpoint ordering and continuity witnesses
response-class labels
descriptor truth vectors
cell-purity or counterexample-pair witnesses
information-price nonnegativity segments
```

A verified checker may reflect these finite records into propositions. Raw external output should never become an unchecked axiom.

---

# 17. Claim ledger and honest boundaries

## 17.1 Inherited foundations used by this draft

This draft relies on the established Straight 42 foundation for:

- the 28-domino finite universe and seat/team structure;
- the nine-declaration relational algebra;
- exact follow and trick-winner rules;
- finite objective post-declaration play;
- perfect-recall information-state boundaries;
- exact current-remainder fibers;
- support-normal-form semantic minimality;
- typed support transitions;
- separation of support, evidence, belief, field, and value;
- exact finite fixed-field best-response existence;
- mechanical future-equivalence minimality for a selected deterministic output contract;
- seat-rotation and oriented-frame gauge boundaries.

Those results should remain source-authoritative when this document is translated into Lean.

## 17.2 Mathematical results proved in this draft

Subject to the displayed definitions and finite/compact assumptions, this document proves:

1. equality-pattern completeness;
2. answer-level meta-fiber decomposition;
3. absence of a canonical answer lift from world belief;
4. the exact multiplicity-bias covariance identity;
5. image/preimage adjunction and cut/step exchange;
6. surviving-master monotonicity;
7. universal fixed-field terminal-law sufficiency;
8. universal additive factorization;
9. additive gauge invariance and the 28-dimensional known quotient;
10. action-value support-function representation;
11. support-function completeness for all additive valuations;
12. the support-function directional derivative formula;
13. the sufficient valuation-inertness criterion;
14. monotone response refinement under growing valuation families;
15. information-refinement policy and polytope inclusion;
16. the revealed-continuation weighted Minkowski formula;
17. \(P^H\subseteq P^C\subseteq P^F\);
18. exact information-price decomposition;
19. the zero-information exposed-face criterion;
20. static descriptor factorization;
21. strong controlled-lumpability value preservation.

These are prose proofs, not machine-checked theorems.

## 17.3 Reported finite evidence

The probe reports provide exact finite evidence for:

- the two-trick hidden fixed-field lines;
- four-to-eight root-\(Q\) refinement and two-to-three action refinement;
- role-answer multiplicities and Fix overlap;
- measurable naive witness-weighting bias;
- mastery persistence, extinction, and birth;
- role-lens dependence;
- a local 90-to-33 static response factorization;
- a hidden horizon-three root switch at \(7/19\);
- hidden internal policy prices \(1/5\) and \(4\);
- zero multisegment curves in 68,244 reported short-suffix worldwise perfect-information root curves;
- continuation-revealed replacement of the hidden normal fan;
- the common-root revealed switch at \(177/131\);
- exact nonnegative information-price functions and decomposition;
- zero continuation-information value after root \(2\!:\!1\);
- partial transfer and exact failure modes of the static control vocabulary;
- compression collapse after adding enough static relations to eliminate the vocabulary ceiling.

These remain exploratory until receipt promotion.

## 17.4 Open mathematical questions

1. **Short-suffix PI affineness.** Characterize or refute the class of perfect-information one-tile parametric suffixes whose root action values are affine.
2. **Minimal information polytopes.** Determine exact vertex and face counts for broader horizons and fields.
3. **Dynamic control skeleton.** Find a compact Scheme/Fix transducer satisfying an exact predictive/lumpability theorem on nontrivial kernels.
4. **Weaker exact abstractions.** Develop belief-dependent or policy-relative quotients weaker than strong lumpability.
5. **Role-query closure.** Characterize which Scheme fragments are closed under exact observation transport and which require Fix disjunction or new predicates.
6. **Purpose-exact descriptions.** Find descriptors whose cells equal, rather than merely refine, selected response classes.
7. **Cross-declaration transfer.** Test doubles-trump and no-trump; the current transfer corpus contains only pip-trump hands.
8. **Nonlinear utility geometry.** Identify compact sufficient laws for make probability, mark utility, and match utility.
9. **Equilibrium valuation geometry.** Determine the correct parametric structure when other players’ strategies are recomputed under each valuation.
10. **Full-match scope.** Add an explicit infinite-horizon or almost-sure termination model for repeated pass-outs.
11. **Formal certificate promotion.** Internalize the finite experiments through a proof-checked reflection layer.

## 17.5 Explicit nonclaims

This document does not claim:

- that support determines belief;
- that a Scheme is an information state;
- that existential witnesses are probabilities;
- that “the” role is defined without uniqueness or a lens;
- that hindsight information was available to an earlier player;
- that the universal continuation object is one baseline-optimal policy;
- that expected capture vectors suffice for nonlinear utility;
- that more information preserves breakpoints or optimal policy identities;
- that worldwise perfect-information classes are the correct hidden-decision classes;
- that the four Experiment 3A atoms are globally sufficient;
- that the 33-cell descriptor is language-independent or transferable;
- that the repaired 64-template registry is a useful compression merely because it is sound;
- that information-state count measures information value;
- that perfect-information affineness is a theorem;
- that doubles-trump or no-trump transfer has been validated;
- that any experimental number is already a promoted foundation theorem;
- that the proposed Lean module plan has been implemented.

---

# 18. Final integrated statement

A Straight Texas 42 decision is not one hidden deal and not one mechanical coordinate. Relative to a viewer and a selected continuation problem, it is

\[
B=(K,e,\beta),
\]

where \(K\) is the exact physical/support kernel, \(e\) is the retained viewer-known evidence still relevant to continuation, and \(\beta\) is a probability law on the admissible augmented latent states.

A Scheme/Fix does not replace that decision state. It asks a typed relational question inside its worlds. An output interface states which roles are returned and which names are merely existential proof machinery. The result is an answer bundle over worlds, not automatically a Boolean, a unique referent, or a probability distribution.

Public play transforms the objective state, exact support, evidence, belief, and role answers in parallel. Rigid transport follows the same physical identities. Fresh query finds the roles true now. Their intersection and differences give persistence, extinction, and birth. Hindsight anchoring is a backward preimage, never retroactive player knowledge.

Planning begins from a universal terminal-outcome law. Information-consistent policies are defined on the player’s actual information partition. Valuation and utility are applied after the alternatives and terminal information needed by the selected purpose have been retained.

For expected additive utility, every action is represented exactly by a finite policy-feature polytope. Its support function gives every tile valuation. Its normal fan gives the policy regions. The additive gauge removes one proved redundant uniform direction, yielding one symmetric trick mode and 27 anisotropy coordinates in the known 28-dimensional quotient.

Imperfect information enters this geometry as policy gluing. Nodes the player cannot distinguish must share an action. Revealing information removes gluing constraints and enlarges the attainable polytope:

\[
P^H\subseteq P^C\subseteq P^F.
\]

The value of information is therefore a support-function gap. Values increase monotonically, but policy prices and breakpoints need not survive. They are features of the selected information polytope, not immutable prices written into the physical hand.

The experiments now show two different compression facts.

First, a local static control descriptor compressed 90 exact worlds to 33 response-sufficient cells for one eight-class target. This proved that present holder identity can contain far more detail than one selected continuation response needs.

Second, that flat descriptor did not transfer generally. Successor context and causal seat position forced richer distinctions, and a sound repaired static vocabulary reconstructed the world. This did not defeat the program. It identified the wrong carrier: worldwise oracle-response purity is stronger than the hidden decision requires.

The hidden problem’s natural carrier is the information-consistent policy geometry. Thousands of information states and an astronomical formal policy space produced only a handful of exposed policies. One root with 7,848 information states had exactly zero value of revelation because one world-independent continuation dominated everywhere.

The next exact object is therefore a dynamic control skeleton: a small relational state whose observation and feature transition law is closed, whose belief can be updated without returning to full worlds, and whose universal continuation values equal the exact hidden solve.

The target theorem is not merely

\[
R^*=\bar R\circ D
\]

for a static world label. It is the dynamic diagram

\[
\boxed{
\begin{array}{ccc}
\text{exact latent process}
&\xrightarrow{\ d\ }&
\text{control-skeleton process}\\[1mm]
\downarrow\text{ action, observation, feature}
&&
\downarrow\text{ induced abstract kernel}\\[1mm]
\text{exact successor law}
&\xrightarrow{\ d_\#\ }&
\text{abstract successor law}\\[2mm]
Q_{\mathrm{abstract}}(a;w,U)
&=&
Q_{\mathrm{exact}}(a;w,U)
\end{array}
}
\]

for the declared policy, field, valuation, and utility scope.

In one final factorization:

\[
\boxed{
\begin{array}{c}
\text{Straight declaration algebra}
+
\text{exact marked physical/support kernel}
+
\text{retained evidence}
+
\text{augmented belief}\\[1mm]
\xrightarrow{\text{Scheme/Fix answers and dynamic role transport}}
\text{lawful relational observables}\\[1mm]
\xrightarrow{\text{information-partition policy constraints}}
\text{universal policy-indexed terminal laws}\\[1mm]
\xrightarrow{\text{valuation and utility}}
(V,Q,\text{policy regions})\\[1mm]
\xrightarrow{\text{selected exact response contract}}
\text{purpose-relative constellations}\\[1mm]
\xrightarrow{\text{factorization or lumpability theorem}}
\text{proved compression}.
\end{array}
}
\]

This is the clearest current statement of the project’s direction:

> represent imperfect information without pretending it is certainty; express roles without turning witnesses into probability; preserve valuation freedom without discarding policies; treat information as a lawful constraint on policy variation; and discover compression from exact predictive behavior rather than imposing it on the hidden worlds in advance.

