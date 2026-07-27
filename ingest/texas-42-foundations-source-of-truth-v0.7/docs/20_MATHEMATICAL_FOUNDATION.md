# Mathematical Foundation of Straight Texas 42

## 0. Claim discipline

This document uses the following statuses:

- **[DEFINITION]**
- **[ADOPTED RULE]**
- **[THEOREM — proved mathematically]**
- **[LEMMA — proved mathematically]**
- **[THEOREM — exhaustive finite verification]**
- **[FINITE VERIFICATION RECEIPT — stated corpus]**
- **[PROPOSITION — proved under explicit assumptions]**
- **[COROLLARY — proved mathematically]**
- **[COROLLARY / STRUCTURAL SYNTHESIS]**
- **[CONSTRUCTED COUNTEREXAMPLE]**
- **[BOUNDARY]**
- **[CONJECTURE]**
- **[UNRESOLVED]**

A finite program is a proof only of the finite statement it exhausts. A
finite verification receipt can also check a deliberately selected finite
corpus without claiming exhaustion of the surrounding mathematical domain. A
proved corollary is a demonstrated consequence of preceding definitions or
theorems. A structural synthesis organizes preceding results but is not
presented as an independent theorem with hidden assumptions. The claim ledger
in `40_CLAIM_STATUS.md` indexes the major statements.

---

## 1. Native factorization

> **[COROLLARY / STRUCTURAL SYNTHESIS]** A physical domino is a stable node
> identifier, not a stable strategic type. A declaration selects the
> relational algebra in which the node's effective suit membership, power,
> rank, and contextual comparison role are interpreted. A player's remaining
> hand is a controllable marked subset embedded in that ambient algebra and
> coupled by conservation and capacities to a hidden complement. A legal play
> relocates one controlled node, changes the remaining marked embedding, and
> creates a public observation. Rule information determines a compatible-world
> support object. A prior and a policy model place a measure on that support.
> Strategic value is derived from the resulting physical and epistemic
> transition under a named utility; it is not generally a function of the
> physical domino or of a path-free mechanical coordinate alone.

The principal layers are:

| Layer | Mathematical object |
|---|---|
| Physics | declaration algebra and objective transition |
| Information | private-observation record plus public history |
| Rule support | compatible complete deals, current hidden remainders, and the legal-prefix reachable image |
| Belief | probability measure on compatible latent worlds |
| Field | behavioral or correlated action law and required latent continuation state |
| Value | expected utility derived from all preceding objects |

The domains must remain distinct. In particular:

- a complete initial deal is not a current remainder assignment;
- a current remainder fiber is not a posterior distribution;
- a Hall-feasible support is not automatically reachable by a legal Straight
  prefix;
- a policy likelihood is not a legality rule;
- a mechanical projection is not automatically a perfect-recall information
  state;
- value is a derived functional, not a stored physical field.

---

## 2. Basic finite objects

### 2.1 Pip and domino sets

**[DEFINITION]**

\[
\mathbb P=\{0,1,2,3,4,5,6\}.
\]

The double-six domino universe is the set of two-element multisets

\[
\mathcal D
=
\{\{i,j\}_{\mathrm{ms}}:i,j\in\mathbb P\}.
\]

For computation, write a physical identity as \((h,l)\) with \(h\ge l\).
This is a canonical naming convention, not a physical orientation.

**[THEOREM — proved mathematically]**

\[
|\mathcal D|=\binom{7+2-1}{2}=\binom82=28.
\]

**Proof.** This is the number of multisets of size two drawn from seven pip
values. ∎

Define

\[
\operatorname{sum}(\{i,j\})=i+j,
\qquad
\operatorname{high}(\{i,j\})=\max(i,j).
\]

Let

\[
\mathcal D^\circ=\{p\!:\!p:p\in\mathbb P\},
\qquad
\mathcal D^\times=\mathcal D\setminus\mathcal D^\circ.
\]

### 2.2 Natural incidence covering

For each \(p\in\mathbb P\), define the natural pip-incidence set

\[
\sigma_p=\{d\in\mathcal D:p\in d\}.
\]

**[THEOREM — proved mathematically]**

1. \(|\sigma_p|=7\);
2. every double lies in exactly one natural incidence set;
3. every mixed domino lies in exactly two natural incidence sets;
4. for \(p\ne q\),
   \[
   \sigma_p\cap\sigma_q=\{p\!:\!q\}.
   \]

**Proof.** The seven members of \(\sigma_p\) are \(p\!:\!k\) for
\(k\in\mathbb P\). A double contains one distinct pip and a mixed domino
contains two. The only domino containing both distinct pips \(p,q\) is
\(p\!:\!q\). ∎

Thus \((\sigma_p)_{p\in\mathbb P}\) is a covering, not a partition.

### 2.3 Seats and partnerships

**[DEFINITION]**

\[
S=\mathbb Z/4\mathbb Z,
\qquad
s^+=s+1\pmod4.
\]

The fixed partnerships are

\[
T_0=\{0,2\},
\qquad
T_1=\{1,3\},
\]

and \(\theta(s)=s\bmod2\).

Given bidder \(b\), declaring/defending orientation is

\[
\theta_b(s)=
\begin{cases}
0,&s\equiv b\pmod2,\\
1,&s\not\equiv b\pmod2.
\end{cases}
\]

### 2.4 Count labels

**[DEFINITION]**

\[
c(d)=
\begin{cases}
10,&d\in\{5\!:\!5,6\!:\!4\},\\
5,&d\in\{5\!:\!0,4\!:\!1,3\!:\!2\},\\
0,&\text{otherwise.}
\end{cases}
\]

**[THEOREM — proved mathematically]**

\[
\sum_{d\in\mathcal D}c(d)=10+10+5+5+5=35.
\]

**Proof.** The five nonzero labels are exactly the five displayed count
dominoes, and every other domino contributes zero. ∎

---

## 3. Declaration-indexed relational algebra

### 3.1 Declaration domain

**[DEFINITION]**

\[
\Delta_{\mathrm{straight}}
=
\mathbb P\cup\{\mathrm{DT},\mathrm{NT}\},
\]

where `DT` is doubles-trump and `NT` is no-trump/follow-me.

There are exactly nine Straight 42 declarations.

### 3.2 Called and powered sets

For \(\delta\in\Delta_{\mathrm{straight}}\), define

\[
\kappa_\delta=
\begin{cases}
\sigma_p,&\delta=p\in\mathbb P,\\
\mathcal D^\circ,&\delta=\mathrm{DT},\\
\varnothing,&\delta=\mathrm{NT},
\end{cases}
\]

and

\[
\pi_\delta=
\begin{cases}
\kappa_\delta,&\delta\in\mathbb P\cup\{\mathrm{DT}\},\\
\varnothing,&\delta=\mathrm{NT}.
\end{cases}
\]

In Straight 42 every nonempty called set is powered. A called domino is
removed from all natural effective incidences and placed in the called suit.

### 3.3 Effective suits

Let

\[
Q=\{0,1,2,3,4,5,6,7\},
\]

where \(7\) names the called suit. Define

\[
\widehat\sigma_p^\delta=\sigma_p\setminus\kappa_\delta
\quad(p\in\mathbb P),
\qquad
\widehat\sigma_7^\delta=\kappa_\delta.
\]

**[THEOREM — proved mathematically: effective membership]** For every
\(\delta\in\Delta_{\mathrm{straight}}\):

1. \(\bigcup_{q\in Q}\widehat\sigma_q^\delta=\mathcal D\);
2. every called domino belongs only to effective suit \(7\);
3. every uncalled mixed domino belongs to exactly two natural effective suits;
4. every uncalled double belongs to exactly one natural effective suit.

**Proof.** If \(d\in\kappa_\delta\), the definition inserts it into
\(\widehat\sigma_7^\delta\) and subtracts it from every natural incidence.
If \(d\notin\kappa_\delta\), its natural memberships are unchanged. Apply the
natural-covering theorem. ∎

The effective family remains a covering rather than a partition in general.

### 3.4 Led suit and follow relation

**[DEFINITION]**

\[
\ell_\delta(d)=
\begin{cases}
7,&d\in\kappa_\delta,\\
\operatorname{high}(d),&d\notin\kappa_\delta.
\end{cases}
\]

**[DEFINITION]**

\[
F_\delta(d,q)=\mathbf1[d\in\widehat\sigma_q^\delta].
\]

A domino can satisfy led context \(q\) exactly when \(F_\delta(d,q)=1\).

### 3.5 Declaration-relative rank, tier, and trick key

Let \(R=\{0,1,\ldots,12\}\cup\{\top\}\), with \(\top\) above every
integer. Define the total declaration-relative rank

\[
r_\delta(d)=
\begin{cases}
p,&\delta=\mathrm{DT},\ d=p\!:\!p,\\
\top,&d\in\mathcal D^\circ\text{ and }\delta\ne\mathrm{DT},\\
\operatorname{sum}(d),&d\in\mathcal D^\times.
\end{cases}
\]

The rank is total, but it matters only inside a nonzero trick tier.

Given led context \(q\), define

\[
\operatorname{tier}_\delta(d,q)=
\begin{cases}
2,&d\in\pi_\delta,\\
1,&d\notin\pi_\delta\text{ and }F_\delta(d,q)=1,\\
0,&\text{otherwise.}
\end{cases}
\]

and the total trick key

\[
\tau_\delta(d,q)=
\begin{cases}
(0,0),&\operatorname{tier}_\delta(d,q)=0,\\
(\operatorname{tier}_\delta(d,q),r_\delta(d)),&\text{otherwise.}
\end{cases}
\]

Keys are compared lexicographically. Tier-zero dominoes are intentionally tied
at the bottom: the lead guarantees that tier zero is never the winning tier.

### 3.6 Unique winner

**[LEMMA — proved mathematically]** A lead domino has tier one or two.

**Proof.** A called lead lies in suit \(7\) and, in Straight 42, is powered.
An uncalled lead lies in the natural effective suit named by its higher pip.
∎

**[THEOREM — proved mathematically: unique trick winner]** For any four
distinct physical dominoes with one designated lead, the four contextual trick
keys have a unique maximum.

**Proof.** The highest occupied tier is not zero by the lemma.

In tier two, either:

- pip trump is in force, in which case the trump double has rank \(\top\) and
  mixed trumps have distinct sums \(p+k\); or
- doubles-trump is in force, in which case the seven doubles have distinct
  ranks \(0,\ldots,6\).

Thus tier-two ranks are injective.

Suppose instead that the highest occupied tier is one. Its led context cannot
be \(7\): a called lead in Straight 42 is powered and would occupy tier two.
Hence the led context is a natural pip \(q\). Every tier-one occupant is an
uncalled member of that natural effective suit. Mixed members have form
\(q\!:\!k\) and distinct sums \(q+k\). If the natural double remains in that
suit it has rank \(\top\); if it was called away, it is absent. Thus tier-one
ranks are injective.

The maximum in the highest occupied tier is therefore unique. ∎

Legality of the three follower plays is not needed; distinctness and a
specified lead suffice.

**[THEOREM — exhaustive finite verification]** The external finite verifier checks
all

\[
9\cdot28\cdot\binom{27}{3}=737{,}100
\]

Straight-declaration/lead/three-subset cases twice: once through the algebraic
trick key and once through a separately coded prose-rule resolver. Both give
one winner and agree in every case.

### 3.7 Contextual comparison and threat

The declaration algebra is naturally two-sorted. Dominoes and suit contexts
are different kinds of objects:

\[
\mathcal A_\delta=
\left(
\mathcal D,Q;
\kappa_\delta,
\pi_\delta,
(\widehat\sigma_q^\delta)_{q\in Q},
\ell_\delta:\mathcal D\to Q,
F_\delta\subseteq\mathcal D\times Q,
r_\delta,
\tau_\delta,
c
\right).
\]

The numerical rank and trick-key codomains are convenient coordinates for the
game order. Define the contextual comparison relation

\[
d\prec_{\delta,q}e
\iff
\tau_\delta(d,q)<\tau_\delta(e,q),
\]

and let the **game-semantic order reduct** be

\[
\mathcal G_\delta=
\left(
\mathcal D,Q;
\kappa_\delta,
\pi_\delta,
(\widehat\sigma_q^\delta)_{q\in Q},
\ell_\delta,
F_\delta,
(\prec_{\delta,q})_{q\in Q},
c
\right).
\]

An isomorphism of this reduct transports both sorts and preserves every
rule-relevant comparison while forgetting the particular integers used to
coordinatize ranks and keys.

For led context \(q\), define

\[
\operatorname{BEATS}_\delta(q,d)
=
\{e\in\mathcal D:\tau_\delta(e,q)>\tau_\delta(d,q)\}.
\]

**[THEOREM — proved mathematically]** If \(d\) is the current winner in a
trick led in context \(q\), a later play \(e\) becomes current winner exactly
when \(e\in\operatorname{BEATS}_\delta(q,d)\).

**Proof.** The current winner has maximal key among plays seen so far. The new
play replaces it exactly when its key is strictly larger, which is the defining
membership condition for `BEATS`. ∎

Define the when-led threat set

\[
\operatorname{THREAT}_\delta(d)
=
\operatorname{BEATS}_\delta(\ell_\delta(d),d).
\]

For a live external set \(O\subseteq\mathcal D\), let

\[
R_\delta(d;O)=\operatorname{THREAT}_\delta(d)\cap O.
\]

**[THEOREM — proved mathematically: monotone threat removal]** If
\(O'\subseteq O\), then

\[
R_\delta(d;O')\subseteq R_\delta(d;O).
\]

**Proof.** Intersecting a fixed set with a subset cannot add elements:
\(\operatorname{THREAT}_\delta(d)\cap O'\subseteq
\operatorname{THREAT}_\delta(d)\cap O\). ∎

This is monotonicity of one relational query, not monotonicity of action value.

**[CONSTRUCTED COUNTEREXAMPLE: threat is not a complete ontology]** In
no-trump, `0-0` and `1-1` each have empty when-led threat sets, yet `0-0`
follows blanks and not ones while `1-1` follows ones and not blanks. Threat
does not determine follow behavior or the whole declaration algebra.

### 3.8 Declaration selection

Before declaration, the rule object is the indexed bundle

\[
\boldsymbol{\mathcal A}
=(\mathcal A_\delta)_{\delta\in\Delta_{\mathrm{straight}}}.
\]

Declaration applies the projection

\[
\operatorname{Sel}_\delta:
\boldsymbol{\mathcal A}\to\mathcal A_\delta.
\]

**[COROLLARY / STRUCTURAL SYNTHESIS]** Declaration is selection of a relational
interpretation, not assignment of a scalar feature. For one stable physical
node, selection can change effective incidences, led suit, power, rank, and
every contextual `BEATS` set.

### 3.9 Pip transports

A pip permutation \(\sigma\in S_7\) transports a physical domino by applying
\(\sigma\) to both ends, transports pip declarations by
\(p\mapsto\sigma(p)\), and fixes `DT`, `NT`, and called-suit label \(7\).
The induced suit-context map is

\[
\widehat\sigma(q)=
\begin{cases}
\sigma(q),&q\in\mathbb P,\\
7,&q=7.
\end{cases}
\]

**[THEOREM — proved mathematically: count-preserving pip classification]** A
pip permutation preserves every count label if and only if it is either the
identity or the swap \(2\leftrightarrow3\).

**Proof.** The unique ten-point double `5-5` forces \(\sigma(5)=5\). The other
ten-point domino `6-4` then forces
\(\{\sigma(6),\sigma(4)\}=\{6,4\}\). Among the five-point dominoes, `5-0` is
the unique one incident to pip 5, so \(\sigma(0)=0\). Of pips 4 and 6, only
pip 4 is incident to a five-point domino (`4-1`), so \(\sigma(4)=4\) and
\(\sigma(6)=6\). Then `4-1` forces \(\sigma(1)=1\). The only remaining pips
are 2 and 3, and `3-2` permits either fixing or swapping them. Both resulting
permutations visibly preserve the five count labels. ∎

Let \(s\) denote the nontrivial swap \(2\leftrightarrow3\), acting on both the
domino sort and the natural suit-context sort while fixing context 7.

**[THEOREM — proved mathematically: scoped game-order transport]** Among
pip-endpoint relabelings that preserve the count labels, the sole nonidentity
permutation induces exactly the two directed nontrivial Straight-layer
transports

\[
\mathcal G_2\cong\mathcal G_3,
\qquad
\mathcal G_3\cong\mathcal G_2.
\]

Under the same endpoint and suit-context transport, it is not an automorphism
or cross-layer isomorphism of any other Straight declaration layer. This is a
classification of count-preserving **pip transports**, not a claim that every
abstract automorphism of every displayed relational reduct has been
classified.

**Proof.** Consider declaration 2 transported to declaration 3; the reverse is
symmetric. The swap sends the called and powered set \(\sigma_2\) to
\(\sigma_3\), sends every effective suit to its transported effective suit,
and preserves led-suit and follow relations. For an uncalled domino, the only
potential failure of numeric `high` transport would involve crossing pips 2
and 3; a domino containing the called pip 2 is not uncalled, and on the
remaining domain the restricted swap preserves the relevant higher-end order.

Tier is therefore preserved. Within the powered suit, the trump double maps to
the trump double. The other-end map

\[
\{0,1,3,4,5,6\}\longrightarrow\{0,1,2,4,5,6\}
\]

is order-preserving, so mixed-trump order is preserved. Within any natural
effective suit, the called other end is removed. The swap restricted from the
remaining other ends to the target remaining other ends is order-preserving;
natural doubles map to natural doubles and remain top. Hence every contextual
comparison relation is preserved.

For failure elsewhere, explicit reversals suffice:

- in no-trump, `0-2` and `0-3` reverse order in led context 0;
- in doubles-trump, `2-2` and `3-3` reverse order in called context 7;
- under pip declaration \(p\notin\{2,3\}\), the powered dominoes `p-2` and
  `p-3` reverse order.

Thus no other Straight layer survives. ∎

The transport preserves game order, not literal numeric rank labels. For
example, `2-0` has numeric rank 2 in twos while its image `3-0` has numeric
rank 3 in threes. Calling this an isomorphism of the full function-valued
\(\mathcal A_\delta\) with fixed numeric codomain would therefore be too
strong.

**[THEOREM — exhaustive finite verification]** An external finite verifier checks
all \(7!=5040\) pip permutations. Exactly the identity and
\(2\leftrightarrow3\) preserve all count labels, and the nontrivial swap
preserves the full transported led-suit, follow, and pairwise contextual-order
signature only for declarations 2 and 3. This is an exhaustive receipt for the
analytic classification.

## 4. Deal, chance, auction, and contract

### 4.1 Complete deal worlds

**[DEFINITION]** A complete deal world for one deal attempt is an ordered
partition

\[
\omega_0=(H_0^0,H_1^0,H_2^0,H_3^0)
\]

such that

\[
\mathcal D
=H_0^0\sqcup H_1^0\sqcup H_2^0\sqcup H_3^0,
\qquad
|H_s^0|=7.
\]

**[THEOREM — proved mathematically]**

\[
N_{\mathrm{deal}}
=
\binom{28}{7}\binom{21}{7}\binom{14}{7}
=
\frac{28!}{(7!)^4}
=
472{,}518{,}347{,}558{,}400.
\]

Conditional on one specified seven-domino hand, the other three labeled hands
have

\[
\frac{21!}{(7!)^3}=399{,}072{,}960
\]

unconstrained assignments.

**Proof.** Choose seven dominoes for seat 0, then seven of the remaining 21
for seat 1, then seven of the remaining 14 for seat 2; seat 3 receives the
last seven. This gives
\(\binom{28}{7}\binom{21}{7}\binom{14}{7}=28!/(7!)^4\).
After fixing one labeled hand, the same argument on the remaining 21 dominoes
gives \(21!/(7!)^3\). ∎

### 4.2 Chance law

**[ADOPTED RULE]** The baseline chance law is uniform over ordered deal worlds:

\[
p_0(\omega_0)=\frac{(7!)^4}{28!}.
\]

Across deal attempts, the baseline model makes the next ordered deal
conditionally independent of all pre-attempt history and non-deal latent
variables, with the same uniform law, unless another cross-deal law is named.

**[PROPOSITION — proved under explicit assumptions]** An ideal Fisher–Yates
shuffle with independent uniform choices, followed by four labeled
seven-domino slices, induces the uniform ordered-deal law.

**Proof.** Fisher–Yates produces each of the \(28!\) linear orders with
probability \(1/28!\). Each ordered four-hand partition has exactly
\((7!)^4\) preimage orders, from independently permuting the seven positions
inside each labeled slice. Hence each partition has probability
\((7!)^4/28!\). ∎

A deterministic seeded procedure does not by itself prove that an arbitrary
finite seed distribution induces exact uniformity.

### 4.3 Auction attempt

Fix shaker \(d\in S\). Acting seats are

\[
s_k=d+1+k\pmod4,
\qquad k=0,1,2,3.
\]

Let \(m_{\max}\in\mathbb N^+\). Bids are

\[
\mathrm{pass},
\qquad
P(n)\ (30\le n\le41),
\qquad
M(m)\ (1\le m\le m_{\max}).
\]

Their order is

\[
P(30)<\cdots<P(41)<M(1)<\cdots<M(m_{\max}).
\]

A nonpass bid must exceed the current high bid. Before a mark bid, entry may be
at \(M(1)\) or \(M(2)\), subject to the cap. After \(M(r)\), the only legal
mark overcall is \(M(r+1)\), subject to the cap. Each seat acts once.

**[THEOREM — proved mathematically: structural reachable ceiling]** The
largest reachable mark bid is

\[
m_{\mathrm{reachable}}=\min(m_{\max},5).
\]

Consequently all configurations with \(m_{\max}\ge5\) induce the same legal
auction tree.

**Proof.** A first mark bid is at most two. After the first mark bid, at
most three actors remain, and each later mark overcall adds exactly one. Thus
no path exceeds \(2+3=5\). The path
\(M(2),M(3),M(4),M(5)\) reaches five whenever the cap permits it. Lower caps
truncate this chain at the cap. ∎

**[THEOREM — proved mathematically: attempt finiteness]** One auction attempt
is finite even if the abstract mark-bid domain is not globally capped.

**Proof.** The attempt has depth four. At any node before a mark bid, the
legal nonpass set consists of finitely many remaining point bids plus at most
\(M(1),M(2)\). After \(M(r)\), there is at most one legal mark overcall,
\(M(r+1)\). Hence every node is finitely branching and the depth is finite. ∎

The profile still adopts a finite configured cap; the theorem states that cap
finiteness is not load-bearing for one-attempt finiteness.

**[THEOREM — exhaustive finite verification]** The verifier enumerates every
four-action auction history. The terminal-history counts for caps 1 through 7
are respectively

\[
2380,
3060,
3196,
3213,
3214,
3214,
3214,
\]

and the largest reached mark is respectively
\(1,2,3,4,5,5,5\).

**[BOUNDARY]** Four passes begin a new deal attempt. Arbitrarily many pass-outs
are possible as a matter of game-tree structure, so the pre-contract match
process is not a finite-horizon tree without an added bound or termination
assumption.

**[PROPOSITION — almost-sure contraction under a uniform lower bound]** If,
at every deal attempt and conditional on every reachable pre-attempt history,
the selected chance/policy model assigns probability at least
\(\varepsilon>0\) to obtaining a contract in that attempt, then the number
\(N\) of attempts until the next contract satisfies

\[
\Pr(N>n)\le(1-\varepsilon)^n,
\qquad
\mathbb E[N]\le\frac1\varepsilon.
\]

A match to target \(T\) then terminates almost surely, and its expected total
number of deal attempts is at most \((2T-1)/\varepsilon\).

**Proof.** Conditional survival through each additional attempt is at most
\(1-\varepsilon\), so induction gives the tail bound. The tail-sum formula
gives \(\mathbb E[N]=\sum_{n\ge0}\Pr(N>n)\le1/\varepsilon\). At most
\(2T-1\) contracted hands are required by §5.8; apply conditional expectation
and sum the per-contract bounds. ∎

Without the lower-bound assumption, an always-pass field is a valid model and
nontermination can have probability one.

### 4.4 Rule support versus auction and declaration evidence

Straight bids and declarations have no hand-content eligibility condition.

**[THEOREM — proved mathematically]** Conditional on a viewer's hand, legal
straight bids and the legal declaration choice remove no deal world solely by
rule feasibility. Before play, rule support remains all ordered deals
consistent with the viewer's hand.

**Proof.** Every straight pass, point bid, configured mark bid, and declaration
has legality determined by public auction state and configuration, not by
private hand content. ∎

Those actions can nevertheless reweight deal worlds under a hand-sensitive
policy model. Rule support and action evidence are different objects.

### 4.5 Contract

**[DEFINITION]** A contracted hand has

\[
K=(b,\operatorname{kind},v,\Theta,w,\delta),
\]

where \(b\) is bidder, \(\operatorname{kind}\in\{\mathrm{point},\mathrm{mark}\}\),
\(v\) is the bid parameter (the point amount \(n\) or mark amount \(m\)),
\(\Theta\) is declaring-team point threshold, \(w\) is mark stake, and
\(\delta\) is declaration.

For \(P(n)\), \(\operatorname{kind}=\mathrm{point}\), \(v=n\), and:

\[
\Theta=n,
\qquad
w=1.
\]

For \(M(m)\), \(\operatorname{kind}=\mathrm{mark}\), \(v=m\), and:

\[
\Theta=42,
\qquad
w=m.
\]

Given final declaring points \(P_D\),

\[
\operatorname{make}(K,P_D)=\mathbf1[P_D\ge\Theta].
\]

**[THEOREM — proved mathematically]** The receiving partnership and award
amount are deterministic functions of \((K,P_D)\).

**Proof.** The contract fixes threshold and stake. Comparing \(P_D\) with the
threshold selects declaring or defending partnership, and the stake is the
award amount. ∎

**[THEOREM — proved mathematically: 42-point threshold equals a seven-trick
sweep]** In a full Straight 42 hand, the declaring partnership finishes with
42 points if and only if it wins all seven tricks.

**Proof.** If the declaring partnership wins all seven tricks, it receives
all seven base trick points and every domino carrying the 35 count points, for
a total of 42. Conversely, every completed trick is worth at least its one
base point. If the defending partnership wins even one trick, it receives at
least one point; since the two final scores sum to 42, the declaring
partnership then has at most 41. Therefore 42 declaring points are possible
exactly on a seven-trick sweep. ∎

Thus the threshold formulation of a mark contract and the traditional
"take every trick" formulation define the same terminal success event in this
full-play profile. The equivalence depends on the one-point base award for
every trick and on playing all seven tricks.

---

## 5. Objective physical game

### 5.1 Full location state

Let the global location set contain

\[
\operatorname{Hand}(s),
\quad
\operatorname{CurrentTrick}(k),
\quad
\operatorname{Completed}(j,k).
\]

A full physical state contains a location map

\[
\lambda_t:\mathcal D\to\operatorname{Loc}
\]

plus order data for the current leader, current trick, and completed trick
boundaries.

**[THEOREM — proved mathematically: location conservation]** Every domino
occupies exactly one global location. A play relocates one node from a hand to
the public trick/completed region; it does not annihilate the node.

**Proof.** The location map is total and single-valued by construction. An
atomic transition changes the image of exactly the played domino and leaves
all other images unchanged; trick completion only retags the four current
trick locations as completed locations. ∎

### 5.2 Reduced contracted-play state and match residue

After declaration, define the reduced contracted-play state

\[
X_t=
\left(
(H_s^t)_{s\in S},
L_t,
C_t,
P_t,
K,
\varphi_t
\right),
\]

where:

- \(H_s^t\) is seat \(s\)'s remaining hand;
- \(L_t\) is the current trick leader;
- \(C_t=((s_0,d_0),\ldots,(s_{k-1},d_{k-1}))\), \(0\le k\le3\), is
  the current trick prefix;
- \(P_t=(P_{0,t},P_{1,t})\) is banked hand score by fixed partnership;
- \(K\) contains bidder, bid semantics, and declaration;
- \(\varphi_t\) is the within-hand phase.

The acting seat is

\[
a(X_t)=L_t+|C_t|\pmod4.
\]

For match continuation, retain separately

\[
Y_r=(M_r,d_r,T),
\]

where \(M_r\) is the partnership mark score before deal attempt \(r\), \(d_r\)
is the current shaker, and \(T\) is the target.

Completed play order is unnecessary for future *physical* transition once all
rule-relevant residue is retained. It can remain essential as public evidence.
The full location state and the reduced state are therefore different exact
objects with different purposes.

### 5.3 Legal actions

Let \(u=a(X_t)\). If \(C_t=\varnothing\), every \(d\in H_u^t\) is legal.
Otherwise let \(q=\ell_\delta(d_0)\), where \(d_0\) is the trick lead, and
set

\[
H_{u,q}^t=\{d\in H_u^t:F_\delta(d,q)=1\}.
\]

**[THEOREM — proved mathematically: exact legal set]**

\[
A(X_t)=
\begin{cases}
H_u^t,&C_t=\varnothing,\\
H_{u,q}^t,&C_t\ne\varnothing\text{ and }H_{u,q}^t\ne\varnothing,\\
H_u^t,&C_t\ne\varnothing\text{ and }H_{u,q}^t=\varnothing.
\end{cases}
\]

This is a direct formalization of lead-anything and follow-if-possible.

**Proof.** On lead there is no suit obligation. After a lead, the rule requires
a follower exactly when the actor holds at least one member of the led
effective suit; otherwise every held domino is legal. These are the three
displayed cases. ∎

### 5.4 Atomic play transition

For legal \(d\in A(X_t)\), remove \(d\) from \(H_u^t\) and append \((u,d)\)
to \(C_t\).

If the new trick length is less than four, leader and banked score remain.
If it is four, let \(w\) be the unique winner and define

\[
g(C)=1+\sum_{(_,e)\in C}c(e).
\]

Add \(g(C)\) to partnership \(\theta(w)\), set the next leader to \(w\), and
clear the current trick.

### 5.5 Physical Markov congruence

**[THEOREM — proved mathematically]** For a contracted hand, \(X_t\) is a
deterministic Markov state for physical continuation.

**Proof.** The actor, legal set, successor hands, trick result, score increment,
next leader, terminal status, and successor phase are functions of \(X_t\) and
the chosen legal action. ∎

**[COROLLARY — baseline chance profile]** Under the baseline independent-deal
chance law, \((X_t,Y_r)\) is sufficient for objective hand settlement,
match-score progression, shaker advancement, and the next deal-attempt chance
transition. The shaker component of \(Y_r\) cannot be omitted when continuation
past the current hand is modeled; the same requirement applies to a separate
all-pass auction state. A different cross-deal chance law may require
additional retained state.

### 5.6 Full-play invariants and graded DAG

Let

\[
\gamma(X)=\sum_{s\in S}|H_s|.
\]

**[THEOREM — proved mathematically]** In a contracted full-play hand:

1. \(\gamma(X_0)=28\);
2. every play lowers \(\gamma\) by one;
3. exactly 28 plays occur;
4. exactly seven tricks complete;
5. the final partnership point totals sum to \(35+7=42\);
6. the contracted-play graph is finite and acyclic under grade \(\gamma\).

**Proof.** Initially four hands contain seven dominoes each. Every legal play
removes exactly one domino from exactly one hand and no transition returns a
domino to a hand, so \(\gamma\) descends from 28 to 0 in 28 steps. A trick
completes after each block of four plays, giving seven tricks. Every physical
domino appears in exactly one completed trick, so all 35 count points are
awarded once, together with seven base trick points. Finiteness of the state
space and strict grade descent give a finite acyclic graph. ∎

### 5.7 Perfect-information backward induction

Fix a complete deal, contract, declaration, terminal or additive utility, and a
well-defined decision operator at every complete-information history node.
Examples include:

- expectation under a fixed continuation policy;
- maximum at every node for a single controller;
- maximum at one partnership's nodes and minimum at the other partnership's
  nodes for a zero-sum perfect-information team game;
- another explicitly defined selector on the finite successor-value vector.

Let the perfect-information continuation object have legal complete objective
histories as nodes and legal plays as edges. Before quotienting equal states it
is a finite rooted tree; viewed as an acyclic graph it is graded by the same
remaining-tile count \(\gamma\).

For additive immediate reward \(r\), terminal value \(u\), and named finite
node operator \(\mathcal O_h\), the recursion is

\[
V(h)=u(h)\quad\text{at terminal }h,
\]

\[
Q(h,a)=r(h,a)+V(h\cdot a),
\qquad
V(h)=\mathcal O_h\bigl((Q(h,a))_{a\in A(h)}\bigr)
\]

at nonterminal histories. A terminal-only utility is the case \(r=0\).

**[THEOREM — proved mathematically]** The graded complete-information history
tree admits a unique recursively defined value for every history node and legal
action under the named utility and node operators.

**Proof.** Terminal values are given. At grade \(k>0\), every successor has
grade \(k-1\). Applying the specified finite-node operator to already defined
successor values defines the predecessor value. Induction over
\(k=0,1,\ldots,28\) completes the recursion. ∎

**[COROLLARY — state-DAG memoization under explicit congruence]** The same
recursion factors through the reduced state \(X_t\), or another compressed
state, only when terminal/additive utility residue and every node operator are
functions of that state and commute with its transition quotient. Otherwise
backward induction still exists on histories, but equal physical states may
not have equal strategic values.

The recursion theorem establishes a unique mathematical value; finiteness
alone does not yet establish an effective exact algorithm.

**[COROLLARY — proved mathematically: effective exact computation]** Suppose
terminal utilities, additive rewards, and every named node operator are
effectively computable in an exact representation class that is closed under
the finitely many operator applications reached by the recursion. Then
backward induction computes every history and action value exactly in finitely
many recursive steps.

**Proof.** The history tree has finite depth and finite branching. Terminal
values are exactly computable by assumption. At each higher grade, finitely
many already computed exact successor values are combined by an effectively
computable operator whose result remains in the exact representation class.
Induction over grades therefore gives a terminating exact computation. ∎

**[BOUNDARY]** A uniquely defined terminal utility may contain a
noncomputable real, or an operator may be mathematically specified without an
effective procedure. In that case the finite recursion still defines a unique
value but does not imply that an algorithm can produce an exact representation
of it. The phrase *perfect-information oracle* is therefore underspecified
unless the utility, optimizing actors, fixed policies, node operators, exact
value representation, and any history-to-state quotient are named.

### 5.8 Contracted-hand match bound

**[THEOREM — proved mathematically]** A match beginning at marks \((0,0)\) and
ending when either partnership reaches target \(T\) lasts at most \(2T-1\)
contracted hands.

**Proof.** Immediately before the terminal contracted hand, both partnership
scores are at most \(T-1\), so at most \(2T-2\) marks can have been awarded.
Every nonterminal contracted hand awards at least one mark, so there can be at
most \(2T-2\) of them. Adding the terminal contracted hand gives at most
\(2T-1\). ∎

This counts contracted hands, not all-pass deal attempts.

---

## 6. Histories, information, and world domains

### 6.1 Objective, public, and private records

A legal objective history records deal worlds, public actions, and physical
transitions. The base public record contains match/deal-attempt boundaries and
shaker identity, bids and passes with actors, declaration with actor, and
actor-attributed plays in order. All-pass, auction winner, acting seat, led
suit, trick winner, trick points, cumulative hand score, settlement, marks,
and match result are deterministic public facts derived from that base record,
configuration, and rules. They may be materialized in a canonical public
history, but they add no information and must not become a second source of
truth.

For player \(m\), let \(\psi_\tau^m\) be the ordered private-observation record:
the sequence of own hands observed on every deal attempt up to time \(\tau\).

### 6.2 Perfect-recall information states

For a deal considered in isolation, or conditional on a fixed pre-deal private
record, a safe deal-local information record is

\[
I_{r,t}^{m,\mathrm{deal}}=(H_{m,r}^0,h_{r,t}),
\]

where \(h_{r,t}\) is the public prefix of deal attempt \(r\), together with the
public match residue supplied at its start.

**[THEOREM — proved mathematically: deal-local scope]** Relative to an isolated
deal, or after conditioning on a fixed pre-deal private record, this deal-local
record has perfect recall for the current attempt.

**Proof.** It retains the player's current private hand and every public event
and own action in the attempt. The earlier private record is outside the
conditional subproblem and is held fixed by hypothesis. ∎

Across the whole match, define

\[
I_\tau^m=(\psi_\tau^m,h_\tau),
\]

where \(\psi_\tau^m\) is the ordered sequence of all privately observed hands,
including hands from abandoned all-pass attempts.

**[THEOREM — proved mathematically: match-global scope]** The match-global
record has perfect recall under the observation model of the rules.

**Proof.** It retains every private hand observed by the player and every
public event, including all of the player's own actions. No modeled private
observation is discarded. ∎

A selected policy model may introduce additional private signals or internal
state observed by an actor. Those are not observations supplied by the Straight
42 rules. When present, they augment that actor's type or information record;
they must not be hidden inside the public history.

The shorter expression \((H_m^0,h_t)\) is therefore exact only for a single
deal attempt under the stated conditioning. It is not a match-global
perfect-recall record after redeals.

### 6.3 Complete-deal support and current remainder map

Fix current deal attempt \(r\), within-attempt time \(t\), and viewer \(m\).
Let \(I_{r,t}^{m,\mathrm{deal}}\) be the current-deal component of the
match-global record.

**[DEFINITION]** The compatible current-deal support is

\[
\Omega_{r,t}^m(I_{r,t}^{m,\mathrm{deal}})
=
\{\omega_r:\omega_r\text{ is an ordered deal compatible by rule with }
I_{r,t}^{m,\mathrm{deal}}\}.
\]

This is a set of complete deals for attempt \(r\), not a set of entire match
histories. Earlier private observations can affect beliefs or persistent policy
state, but under the baseline independent-deal law they do not add a
rule-feasibility constraint to the current deal beyond its current-deal
component.

Let \(B_s(h_{r,t})\) be the set of dominoes publicly played by seat \(s\)
through time \(t\) in the current attempt, including any tile currently in the
trick. For a compatible deal, define the current hidden-remainder map

\[
\rho_{r,t}^m(\omega_r)
=
(H_{s,r}^0\setminus B_s(h_{r,t}))_{s\ne m}.
\]

A complete deal world and its current hidden remainder live in different
spaces. The first is fixed for the attempt; the second changes type as public
plays remove dominoes from hidden hands.

### 6.4 Mechanical/support projection

A mechanical/support projection for the current deal is

\[
q_{r,m}:I_\tau^m\to c_{r,t}^m.
\]

For an isolated hand, the domain can be replaced by the deal-local record. In
a match, the projection may discard earlier private hands for physical/support
purposes while the complete information state remains available to policy and
belief models. The projection retains enough information to determine:

1. the viewer's current hand and legal actions;
2. the public physical rule residue;
3. hidden-seat capacities;
4. every rule-derived exclusion on current hidden hands;
5. the exact current-remainder fiber;
6. typed support updates after a public action;
7. the payoff residue required by a named utility.

A concrete sufficient Straight 42 reference object may retain

\[
c_{r,t}^m=
\left(
\varphi_t,m,H_{m,r}^t,\delta,K,L_t,C_t,P_t,Y_r,
(B_s(h_{r,t}))_{s\in S},(V_s(h_{r,t}))_{s\in S}
\right),
\]

where \(V_s\) is the set of publicly established effective-suit voids for
seat \(s\). The actor-attributed played sets include tiles currently in the
trick; the ordered current trick \(C_t\) supplies their within-trick order.
The unseen pool, capacities, and local possible-holder sets are **derived**:

\[
U_{r,t}^m
=
\mathcal D\setminus
\left(H_{m,r}^t\cup\bigcup_{s\in S}B_s(h_{r,t})\right),
\]

\[
k_s=7-|B_s(h_{r,t})|,
\qquad
P_s
=
U_{r,t}^m\setminus
\bigcup_{q\in V_s(h_{r,t})}\widehat\sigma_q^\delta
\quad(s\ne m).
\]

These derived cells present exact Straight rule support by §7.5. Their
canonical feasible support payload—certain hidden-location marks plus the
tagged ambiguity component of §7.10—is also derived, not stored as a second
source of truth. This removes duplication without claiming that every other
field of the displayed mechanical coordinate is minimal. The coordinate
intentionally omits the order of completed plays and the losing action
sequence of the auction; other exact encodings may replace retained fields by
proved-equivalent residue.

The displayed form is a post-declaration contracted-play coordinate. During
the auction, the corresponding mechanical object retains public auction state,
match residue, the viewer's hand, and the declaration bundle
\(\boldsymbol{\mathcal A}\) rather than a selected \(\delta\) and contract
\(K\). Rule support is still the full conditional deal set because straight
auction actions impose no hand-content eligibility predicate.

### 6.5 Objective congruence and viewer bundle

**[THEOREM — proved mathematically]** Complete objective histories that map to
the same reduced objective state \(X_t\) have the same physical legal actions,
rewards, terminal status, and successor reduced states.

**Proof.** This is the congruence statement of §5.5: each listed quantity is a
deterministic function of \(X_t\) and the chosen legal action. ∎

**[THEOREM — proved mathematically]** A viewer mechanical state \(c_{r,t}^m\) does
not identify one reduced objective state. It identifies a family of objective
continuations indexed by compatible current hidden remainders. Under the
reconstruction assumptions stated in §7, pairing \((c_{r,t}^m,\omega_t)\) with one
remainder world reconstructs one current reduced contracted-play state
\(X_t\). Reconstructing a full location history requires whatever additional
public attribution the chosen full-state object declares.

**Proof.** The mechanical component supplies the viewer hand, contract,
leader, current trick, scores, and phase. One remainder world supplies the
three hidden remaining hands. Together these are exactly the fields of the
reduced state \(X_t\). A full location history contains additional completed
location/order data not necessarily retained by the mechanical projection. ∎

### 6.6 A mechanical state is not the original information state

**[CONSTRUCTED COUNTEREXAMPLE]** Two auctions can have the same shaker, bidder,
winning bid, declaration, and later play state while differing in which losing
seat bid 30. Every player observed the losing bid and remembers its actor.
Therefore the two histories are distinct perfect-recall information states even
if a path-free mechanical projection merges them.

A coarser key can be an intentional abstraction or a sufficient statistic for
a named decision problem. It is not thereby the original extensive-form
information partition.

### 6.7 Rule support is not belief

**[THEOREM — proved mathematically]** Exact rule support does not determine
relative probability within that support.

**Proof.** Rule compatibility is a Boolean predicate on complete deals.
Relative probability additionally depends on the chance law and on the
likelihood of discretionary public actions under a policy model. Two histories
can have the same compatibility predicate and different likelihood functions.
The full-fiber witness in §10.4 makes this difference action-relevant. ∎

**[BOUNDARY: rule support versus chance support]** Throughout the cell theorem,
*support* means compatibility with the Straight 42 legality rules. Under the
baseline uniform independent-deal law, every such current deal has positive
chance mass before discretionary-action likelihood is applied. A separately
selected chance law with structural zeros or cross-deal restrictions adds an
explicit prior-support restriction. Let
\(p_{r,0}^{m,\omega}=(\operatorname{pr}_\omega)_\#p_{r,0}^m\) be the
current-deal marginal. The ordered-deal domain is finite, so define its
positive-mass support by

\[
\operatorname{supp}_+(p_{r,0}^{m,\omega})
=
\{\omega:p_{r,0}^{m,\omega}(\{\omega\})>0\}.
\]

Before discretionary-action likelihood is applied, the physical
positive-mass support is

\[
\rho_{r,t}^m\left(
\Omega_{r,t}^m(I_{r,t}^{m,\mathrm{deal}})
\cap\operatorname{supp}_+(p_{r,0}^{m,\omega})
\right),
\]

which may be a strict subset of \(\Phi(c_{r,t}^m)\); zero modeled action
likelihood can shrink the posterior positive-mass support further. These are
explicit chance- and policy-model restrictions, not failures of the
capacity-cell representation of rule support.

---

## 7. Capacity cells and the current-remainder fiber

### 7.1 Hidden pool and capacity cells

Fix viewer \(m\) during one declared deal attempt. Let
\(B_s(h_{r,t})\) be the publicly played set of seat \(s\), including a tile
currently in the trick. Define the current hidden pool

\[
U_{r,t}^m
=
\mathcal D
\setminus
\left(
H_{m,r}^t\cup\bigcup_{s\in S}B_s(h_{r,t})
\right).
\]

For hidden seat \(s\ne m\), define its public void set

\[
V_{s,r,t}
=
\left\{
q\in Q:
\begin{array}{l}
\text{at some earlier follower turn led in context }q,\\
\text{seat }s\text{ publicly played }d\text{ with }F_\delta(d,q)=0
\end{array}
\right\}.
\]

The upper-bound possible-holder set and exact capacity are constructed by

\[
P_{s,r,t}
=
U_{r,t}^m
\setminus
\bigcup_{q\in V_{s,r,t}}\widehat\sigma_q^\delta,
\qquad
k_{s,r,t}=7-|B_s(h_{r,t})|.
\]

Define the capacity cell

\[
C_{s,r,t}=(P_{s,r,t},k_{s,r,t})
\]

and the cell system

\[
\mathbf C_{r,t}^m
=
\left(U_{r,t}^m;(P_{s,r,t},k_{s,r,t})_{s\ne m}\right).
\]

Thus the cells are constructed only from the common unseen pool, public
played-by-seat attribution, exact remaining capacities, and observed failures
to follow. They are not defined by first enumerating the desired fiber. There
are exactly three hidden seats from one viewer's perspective.

### 7.2 Cells are dependent

Cells are not independent per-seat marginals. They share one conserved pool.

**[CONSTRUCTED COUNTEREXAMPLE]** Let \(U=\{a,b\}\), let two seats each have
possible set \(U\), and let both capacities be one. Independent choices give
four ordered pairs, but only \((a,b)\) and \((b,a)\) are disjoint conserved
assignments.

### 7.3 Intensional current-remainder fiber

Suppress the fixed indices \(r,t,m\) in this subsection.

**[DEFINITION]** The Straight 42 current-remainder fiber of a cell system
\(\mathbf C=(U;(P_s,k_s)_{s\ne m})\) is

\[
\Phi(\mathbf C)
=
\left\{
(H_s^t)_{s\ne m}:
\begin{array}{l}
H_s^t\subseteq P_s,\\
|H_s^t|=k_s,\\
H_s^t\cap H_{s'}^t=\varnothing\quad(s\ne s'),\\
\bigsqcup_{s\ne m}H_s^t=U_t^m
\end{array}
\right\}.
\]

This is an intensional set defined by exactly the displayed Straight 42
constraints. For a mechanical state \(c\), the notation \(\Phi(c)\) is
shorthand for \(\Phi(\mathbf C(c))\), where \(\mathbf C(c)\) is its cell
projection. Therefore the support object factors through the cell system: two
mechanical states with equal cells have equal current-remainder fibers even
when their leaders, scores, contracts, histories, or strategic values differ.

Enumeration is one exact query returning the fiber's extensional members;
enumeration is not the definition. A different rules profile may add separately
named constraints, but none are hidden inside this definition.

For an explicit predicate \(R\),

\[
\Phi_R(\mathbf C)=\{\omega\in\Phi(\mathbf C):R(\omega)\}
\]

is an exact restriction. A hidden cap, sample, or truncation is not the same
object.

### 7.4 Initial cells after a straight auction and declaration

Before the first play of a contracted Straight 42 hand,

\[
U_0^m=\mathcal D\setminus H_m^0,
\qquad
P_s=U_0^m,
\qquad
k_s=7
\quad(s\ne m).
\]

The straight auction and declaration can reweight deals under a policy model,
but their legality does not remove any deal consistent with the viewer's hand.

### 7.5 Losslessness and deal–remainder correspondence

**[THEOREM — proved mathematically: exact Straight 42 cell support]** For every
legal public play prefix in the stated scope,

\[
\Phi(c_{r,t}^m)
=
\rho_{r,t}^m\bigl(\Omega_{r,t}^m(I_{r,t}^{m,\mathrm{deal}})\bigr).
\]

Equivalently, common pool, exact capacities, per-seat upper-bound possible sets,
disjointness, and conservation express exactly the rule-derived support of the
three current hidden hands. No surviving positive clause such as “seat \(s\)
must still hold at least one member of \(Q\)” is required.

**Proof by induction on public plays.**

**Base.** After a straight auction and declaration and before play, every
partition of the 21 unseen dominoes into three labeled seven-domino hands is
rule-compatible. This is exactly the initial cell fiber.

Assume equality before the next legal play. For any hidden actor, legality of
the extended history implies that the publicly played domino \(d\) belongs to
the actor's predecessor possible set; every reverse construction below is
relative to that fixed pre-action condition.

**Viewer action.** If the actor is \(m\), the played domino came from the known
viewer hand, not the hidden pool. The three hidden remainder hands are
unchanged. Leading imposes no hidden-hand condition; following legality is
already determined by the viewer's known hand. Thus the hidden-remainder map
is the identity and the cell fiber remains exact.

**Hidden-seat lead.** If hidden seat \(s\) leads \(d\), a compatible pre-world
must contain \(d\) in \(H_s\). The action then removes \(d\) from the hidden
pool and lowers \(k_s\) by one. Leading imposes no condition on the other
remaining dominoes. Conversely, add \(d\) back to seat \(s\) in any successor
assignment satisfying the updated cells; the reconstructed lead is legal.

**Hidden-seat successful follow.** If \(d\) follows led suit \(q\), the played
domino itself is a witness that the pre-play hand contained a follower. It is
immediately removed from the hidden remainder. No positive follower condition
survives. Conversely, adding the following domino \(d\) back makes the observed
play legal regardless of what other followers remain.

**Hidden-seat failure to follow.** If \(d\) does not follow \(q\), legality
implies that the actor's entire pre-play hand was disjoint from
\(\widehat\sigma_q^\delta\). Because \(d\notin\widehat\sigma_q^\delta\), the
successor remainder is still disjoint from that set. Delete the whole follow
set from \(P_s\), remove \(d\) globally, and lower \(k_s\). Conversely, every
successor assignment satisfying this exclusion can be extended by adding the
nonfollower \(d\); the reconstructed pre-play hand has no follower, so the
observed action is legal.

Every update preserves exact capacities, disjointness, and conservation. The
forward and reverse constructions establish equality at the successor. ∎

**[COROLLARY — proved mathematically: fixed-history bijection]** For a fixed
actor-attributed legal public history in this scope, the remainder map is a
bijection between compatible complete deals and the current fiber.

**Proof.** Surjectivity is the theorem. For injectivity, public history fixes
each seat's played set \(B_s(h_{r,t})\). For hidden seats,

\[
H_{s,r}^0=H_{s,r}^t\sqcup B_s(h_{r,t}),
\]

while the viewer's initial hand is already part of
\(I_{r,t}^{m,\mathrm{deal}}\). Thus one current hidden-remainder assignment
reconstructs one initial hand for every seat and hence one complete deal. ∎

A path-free coordinate that omits played-by-seat attribution can still encode
exact *current* support, but the inverse deal reconstruction and policy
likelihood are then not functions of that coordinate alone.

**[FINITE VERIFICATION RECEIPT — stated corpus]** The external finite verifier
constructs 12 deterministic legal contracted hands under each of the nine
Straight declarations. For every prefix from play 20 through play 28, it
independently:

1. derives the capacity cells from unseen pool, capacities, and public voids;
2. enumerates the resulting current-remainder fiber;
3. reconstructs every capacity-compatible complete deal candidate and replays
   the actor-attributed public prefix under the rules; and
4. requires exact set equality.

All 972 stated prefixes pass; 970 contain at least one derived public void.
This is a regression receipt for the theorem and its typed updates, not an
exhaustion of all reachable Straight 42 histories.

### 7.6 Scope of the losslessness theorem

The theorem assumes:

- one viewer and three hidden active seats;
- legal Straight 42 play;
- ordinary follow-if-possible rules;
- no hand-content bid or declaration eligibility;
- no sitting out, draw, exchange, exposure, claim, or private side observation;
- public attribution of every played domino to its actor in the underlying
  history;
- rule-derived support only, not policy likelihood.

A contract such as plunge, whose legal bid reveals a positive private-hand
predicate, lies outside this theorem.

### 7.7 Hall/max-flow feasibility

Let \(J=S\setminus\{m\}\). Assume \(P_s\subseteq U\) and \(k_s\ge0\).

**[THEOREM — proved mathematically]** The capacity-cell fiber is nonempty if
and only if

\[
|U|=\sum_{s\in J}k_s
\]

and, for every \(R\subseteq J\),

\[
\left|\bigcup_{s\in R}P_s\right|
\ge
\sum_{s\in R}k_s.
\]

Under these conditions, the full-set inequality and \(P_s\subseteq U\) imply
\(\bigcup_sP_s=U\); retaining that equality as an explicit validator is
harmless and useful.

**Proof.** Replace seat \(s\) by \(k_s\) labeled slots, each adjacent to every
domino in \(P_s\). A compatible assignment is a matching that covers all
slots and all dominoes. Hall's theorem applies to subsets of slots. For any
slot subset \(A\), let \(R\) be the seats represented in \(A\). Then

\[
|A|\le\sum_{s\in R}k_s
\le\left|\bigcup_{s\in R}P_s\right|
=|N(A)|.
\]

Thus the seat-subset inequalities imply all slot-level Hall inequalities.
Necessity follows by counting the dominoes assigned to any seat subset.
Because total slot count equals \(|U|\), a slot-covering matching is a complete
assignment. ∎

For three hidden seats, there are seven nonempty seat subsets to check.

**[THEOREM — exhaustive finite verification]** The external finite verifier checks
66,968 abstract three-seat cell systems on universes of sizes one through four
and confirms exact agreement between direct assignment enumeration and the
Hall conditions.

### 7.8 Exact fiber cardinality and the measure boundary

Let \(J\) be the hidden seats and let
\(\mathbf C=(U;(P_s,k_s)_{s\in J})\). The coefficient identity and deletion
recurrence below remain valid for arbitrary finite possible sets and
nonnegative capacities, with count zero when the requested occupancies cannot
cover the pool. The later resource bounds additionally assume the structural
cell-system conservation invariant

\[
|U|=\sum_{s\in J}k_s,
\]

which every native `CellSystem` satisfies. Introduce one formal variable
\(x_s\) per hidden seat and define

\[
Z_{\mathbf C}(x)
=
\prod_{d\in U}
\left(
\sum_{s\in J:\ d\in P_s}x_s
\right).
\]

For a multivariate polynomial, let
\([\prod_s x_s^{k_s}]Z\) denote the coefficient of the displayed monomial.

**[THEOREM — proved mathematically: exact fiber-count coefficient]**

\[
|\Phi(\mathbf C)|
=
\left[\prod_{s\in J}x_s^{k_s}\right]
Z_{\mathbf C}(x).
\]

**Proof.** Expanding the product chooses, for each domino \(d\in U\), one
term \(x_s\) with \(d\in P_s\). Such a choice is exactly an allowed holder
function \(f:U\to J\). The exponent of \(x_s\) is
\(|f^{-1}(s)|\). Therefore the coefficient of
\(\prod_sx_s^{k_s}\) counts exactly the allowed holder functions having the
required seat capacities. Their inverse images are pairwise disjoint, cover
\(U\), and are subsets of the corresponding \(P_s\), so they are precisely
the worlds in \(\Phi(\mathbf C)\). ∎

Choose any \(d\in U\). For an allowed seat \(s\) with \(k_s>0\), let
\(\mathbf C^{d\to s}\) remove \(d\) from \(U\) and from every possible set,
and reduce \(k_s\) by one.

**[COROLLARY — proved mathematically: exact deletion recurrence]**

\[
N(\mathbf C)
=
\sum_{\substack{s\in J\\d\in P_s,\ k_s>0}}
N(\mathbf C^{d\to s}),
\qquad
N(\varnothing;(\varnothing,0)_{s\in J})=1,
\]

with every structurally invalid base case assigned count zero. Here
\(N(\mathbf C)=|\Phi(\mathbf C)|\).

**Proof.** Partition the fiber by the unique holder of \(d\). Removing that
fixed tile from its holder is a bijection from each part to the corresponding
successor fiber. The parts are disjoint and exhaustive, so their cardinalities
sum. ∎

The coefficient identity also gives a direct dynamic program. Fix an order
\(d_1,\ldots,d_n\) of \(U\). For an occupancy vector
\(a=(a_s)_{s\in J}\), let \(A_i(a)\) be the number of allowed assignments of
the first \(i\) dominoes with exactly \(a_s\) assigned to seat \(s\). Set

\[
A_0(0,\ldots,0)=1
\]

and all other \(A_0\) values to zero. For every allowed holder of
\(d_{i+1}\), update

\[
A_{i+1}(a+e_s)\mathrel{+}=A_i(a)
\quad\text{when }d_{i+1}\in P_s\text{ and }a_s<k_s.
\]

**[THEOREM — proved mathematically: exact capacity dynamic program]** The
terminal value is

\[
N(\mathbf C)=A_n((k_s)_{s\in J}).
\]

Every occupancy vector appearing at layer \(i\) satisfies

\[
\sum_{s\in J}a_s=i.
\]

Under the structural conservation invariant \(|U|=\sum_sk_s\), across
*all* layers the algorithm visits at most

\[
\prod_{s\in J}(k_s+1)
\]

distinct occupancy vectors. The unique capacity vector \((k_s)_s\) lies at
the terminal layer and is not extended. An implementation that tests every
seat at every nonterminal live vector therefore performs at most

\[
|J|\left(\prod_{s\in J}(k_s+1)-1\right)
\]

candidate-holder checks. Of those checks, at most

\[
\sum_{s\in J} k_s
\prod_{r\in J\setminus\{s\}}(k_r+1)
\]

are capacity-eligible extension updates before locally disallowed holder edges
are removed. At one fixed layer, any chosen \(|J|-1\) coordinates determine
the last coordinate from the layer sum. Thus one layer contains at most

\[
\min_{r\in J}\prod_{s\in J\setminus\{r\}}(k_s+1)
\]

live coefficients.

**Proof.** The invariant is that \(A_i(a)\) counts exactly the holder functions
on \(\{d_1,\ldots,d_i\}\) that obey the local allowed edges and have occupancy
vector \(a\). The update partitions such functions by the holder of the next
domino. Induction on \(i\) proves the invariant, and the required capacity
vector at \(i=n\) is exactly the fiber definition.

Every assignment of \(i\) processed dominoes has total occupancy \(i\), proving
the layer-sum identity. A bounded occupancy vector has at most
\(\prod_s(k_s+1)\) possible values and, because its coordinate sum fixes its
layer, cannot occur in two different layers. The capacity vector is the sole
bounded vector whose coordinate sum is \(|U|=\sum_sk_s\), so it is the only
possible terminal-layer vector and is never extended. This proves the
candidate-check bound. For a fixed seat \(s\), exactly
\(k_s\prod_{r\ne s}(k_r+1)\) bounded vectors satisfy \(a_s<k_s\); every
such vector is nonterminal. Summing over seats gives the capacity-eligible
update bound. At a fixed layer, deleting any one coordinate leaves that
coordinate uniquely determined by the sum, giving the displayed memory bound.
∎

For native Straight 42, there are three hidden seats, \(|U|\le21\), and every
\(k_s\le7\). Hence one unrestricted fiber count visits at most 512 occupancy
vectors over its entire run and performs at most

\[
3(8^3-1)=1{,}533
\]

candidate-holder checks. At most

\[
3\cdot7\cdot8^2=1{,}344
\]

of those checks can produce capacity-eligible extension updates before local
holder restrictions are applied. The general layer bound is 64 live
coefficients. It can be sharpened exactly to 48: enlarging capacities to
\((7,7,7)\) can only add occupancy vectors, and the number at layer \(i\) is
the coefficient of \(x^i\) in

\[
(1+x+\cdots+x^7)^3.
\]

By bounded-composition inclusion--exclusion this coefficient is

\[
L_i=
\sum_{j=0}^{3}(-1)^j\binom3j
\binom{i-8j+2}{2},
\]

where a binomial coefficient is zero when its upper argument is below two.
For \(i=0,\ldots,10\), the values are

\[
1,3,6,10,15,21,28,36,42,46,48,
\]

and symmetry \(L_i=L_{21-i}\) gives the remaining layers, with
\(L_{11}=48\). Thus no native layer has more than 48 live occupancy states.

Moreover,

\[
N(\mathbf C)
\le
\frac{|U|!}{\prod_sk_s!}
\le
\frac{21!}{(7!)^3}
=
399{,}072{,}960.
\]

The first inequality forgets all holder restrictions. For the second, the
multinomial coefficient is nondecreasing when any one capacity is increased:
increasing \(k_s\) by one multiplies it by
\((|U|+1)/(k_s+1)\ge1\). Its maximum over
\(0\le k_s\le7\) is therefore attained at \((7,7,7)\).

Thus unrestricted native-fiber counting has a sharply bounded exact algorithm,
even though extensional enumeration can still contain nearly four hundred
million worlds. Arbitrary predicate-restricted counting and generalized cell
systems with a variable number of seats are different computational problems.

**[THEOREM — exhaustive finite verification]** The verifier exhausts all
\(8^3=512\) unrestricted native capacity triples
\((k_0,k_1,k_2)\in\{0,\ldots,7\}^3\). For the explicitly instrumented
occupancy-vector algorithm, it confirms the exact multinomial count and exact
attainment of the total-state, candidate-check, and capacity-eligible-update
formulas for every triple. Across the 512 profiles it confirms the displayed
maxima of 399,072,960 worlds, 512 total occupancy states, 1,533 candidate
checks, 1,344 capacity-eligible updates, and 48 live states in one layer; for
\((7,7,7)\) it also checks the complete 22-layer sequence displayed above.

**[THEOREM — proved mathematically: support does not select a sampling law]** A
finite fiber with at least two worlds does not determine a unique normalized
probability measure, even if every world is required to have positive mass.

**Proof.** Let \(|\Phi|=n\ge2\), choose distinct worlds
\(\omega_0,\omega_1\), and choose \(0<\varepsilon<1/n\). The uniform law
assigns \(1/n\) to every world. A second full-support law assigns
\(1/n+\varepsilon\) to \(\omega_0\), \(1/n-\varepsilon\) to
\(\omega_1\), and \(1/n\) to every other world. Both have the same support
and total mass one, but they are different measures. ∎

An empty fiber supports no probability law; a singleton fiber supports one.
Uniformity on a nontrivial fiber is therefore a separately selected belief or
a consequence of additional chance assumptions such as §8.4. A sampler that
accepts only a fiber is mathematically incomplete unless its sampling law is
explicitly named.

Now explicitly select the uniform law on a nonempty fiber. For any
\(d\in U\), use the same successor cell systems \(\mathbf C^{d\to s}\) as in
the deletion recurrence, assigning count zero to a disallowed or structurally
invalid successor.

**[THEOREM — proved mathematically: uniform holder marginals and exact
count-ratio sampling]** Under the uniform law on \(\Phi(\mathbf C)\),

\[
\Pr(d\in H_s)
=
\frac{N(\mathbf C^{d\to s})}{N(\mathbf C)}.
\]

Fix any deterministic rule for selecting the next remaining domino. Beginning
from \(\mathbf C\), repeatedly choose the selected domino's holder \(s\)
with probability

\[
\frac{N(\mathbf C^{d\to s})}{N(\mathbf C)}
\]

and recurse on \(\mathbf C^{d\to s}\). The resulting complete assignment is
exactly uniform on \(\Phi(\mathbf C)\).

**Proof.** Worlds in which \(d\) belongs to seat \(s\) are in bijection
with \(\Phi(\mathbf C^{d\to s})\), so the first display is the cardinality
of that part divided by total cardinality. For a fixed final world, the holder
of every successively selected domino is unique. Along its unique recursion
path, the product of conditional probabilities telescopes:

\[
\frac{N(\mathbf C_1)}{N(\mathbf C_0)}
\frac{N(\mathbf C_2)}{N(\mathbf C_1)}
\cdots
\frac{N(\mathbf C_{|U|})}{N(\mathbf C_{|U|-1})}
=
\frac{1}{N(\mathbf C_0)},
\]

because the terminal valid empty system has count one. Every world therefore
has the same probability, and the recurrence shows that the holder
probabilities at each step sum to one. ∎

This theorem does not make the uniform law canonical. For native Straight 42,
the exact capacity dynamic program makes every required unrestricted successor
count explicitly bounded; the sampler still requires an exact random mechanism
for the displayed integer weights. It never needs to materialize the complete
fiber. Predicate-restricted sampling or a generalized cell type can have a
different computational boundary.

**[THEOREM — exhaustive finite verification]** On the same 66,968 abstract
three-seat systems used for the Hall receipt, the external finite verifier confirms
that direct assignment count, generating-function coefficient extraction, and
the deletion recurrence agree exactly. On every nonempty system in that domain,
it also follows every world through the count-ratio recursion and confirms
22,620 exact world probabilities, each equal to the reciprocal fiber count and
summing to one within its system.

### 7.9 Marginal holder support and canonical edge reduction

The locally allowed set \(P_s\) need not equal the set of dominoes that seat
\(s\) can actually hold in some globally conserved assignment. Define the
**marginal holder support**

\[
P_s^\star
=
\bigcup_{\omega\in\Phi(\mathbf C)}H_s(\omega),
\]

with the union interpreted as empty when the fiber is empty, and define the
**canonical support reduction**

\[
\operatorname{red}(\mathbf C)
=\mathbf C^\star
=
\left(U;(P_s^\star,k_s)_{s\in J}\right).
\]

**[THEOREM — proved mathematically: exact edge-support criterion and canonical
reduction]** For every \(d\in U\) and seat \(s\),

\[
d\in P_s^\star
\iff
\left(d\in P_s,\ k_s>0,\ N(\mathbf C^{d\to s})>0\right).
\]

Equivalently, the forced successor \(\mathbf C^{d\to s}\) satisfies the Hall
conditions. Furthermore, for fixed pool and capacities:

1. \(\Phi(\mathbf C^\star)=\Phi(\mathbf C)\);
2. if \(\mathbf Q=(U;(Q_s,k_s)_s)\) has the same fiber, then
   \(P_s^\star\subseteq Q_s\) for every seat;
3. \(\mathbf C^\star\) is therefore the unique coordinatewise least
   possible-set system representing that fiber;
4. the reduction is contractive:
   \(\mathbf C^\star\le\mathbf C\) coordinatewise;
5. it is idempotent:
   \(\operatorname{red}(\operatorname{red}(\mathbf C))
   =\operatorname{red}(\mathbf C)\);
6. it is monotone: if \(\mathbf C\le\mathbf Q\) coordinatewise, then
   \(\operatorname{red}(\mathbf C)\le\operatorname{red}(\mathbf Q)\);
7. it is a canonical normal form:
   \[
   \Phi(\mathbf C)=\Phi(\mathbf Q)
   \iff
   \operatorname{red}(\mathbf C)=\operatorname{red}(\mathbf Q).
   \]

**Proof.** A world with \(d\in H_s\) is taken by removing \(d\) from seat
\(s\) to a world in \(\Phi(\mathbf C^{d\to s})\), and adding \(d\) back is
the inverse. This proves the first equivalence. Positive count is equivalent
to Hall feasibility by §7.7.

Every original world uses only edges that occur in an original world, so it
satisfies \(\mathbf C^\star\). Conversely
\(P_s^\star\subseteq P_s\), so every world satisfying
\(\mathbf C^\star\) already satisfies \(\mathbf C\). This proves fiber
equality and contractivity. If \(\mathbf Q\) has the same fiber and
\(d\in P_s^\star\), some common world assigns \(d\) to \(s\), forcing
\(d\in Q_s\). Hence every equivalent possible-set system contains
\(P_s^\star\), proving unique coordinatewise leastness.

Because \(\mathbf C\) and \(\mathbf C^\star\) have the same fiber, taking the
marginal holder union again returns the same sets, proving idempotence. If
\(\mathbf C\le\mathbf Q\), then \(\Phi(\mathbf C)\subseteq\Phi(\mathbf Q)\);
taking per-seat unions over those world sets proves monotonicity. Equal fibers
have equal marginal holder unions, so their reductions are equal. Conversely,
if the reductions are equal, each original fiber equals the fiber of that
common reduction, proving the normal-form equivalence. ∎

Because it removes unsupported edges, \(\operatorname{red}\) is contractive;
in order-theoretic language it is an interior/kernel operator on the fixed
possible-set lattice rather than an extensive closure operator. This package
therefore calls it a **reduction**.

Under the uniform law on a nonempty fiber,

\[
d\in P_s^\star
\iff
\Pr(d\in H_s)>0.
\]

This follows immediately from the exact count-ratio marginal in §7.8.
Computing all three marginal holder supports needs at most one Hall test for
each of the at most \(3|U|\le63\) locally allowed holder edges; it does not
require fiber enumeration or exact counting.

**[CONSTRUCTED COUNTEREXAMPLE: local allowance is not marginal possibility]**
Let \(U=\{a,b\}\), capacities be one for seats 0 and 1, and

\[
P_0=\{a,b\},\qquad P_1=\{a\}.
\]

The sole world assigns \(b\) to seat 0 and \(a\) to seat 1. Thus
\(a\in P_0\) but \(a\notin P_0^\star\). A locally allowed edge can be
eliminated globally by conservation and another seat's capacity.

The theorem as stated is a fixed-schema result. Section 7.10 extends it
across all nonempty feasible pools and capacities, identifies the resulting
coarsest exact semantic quotient, and removes the tuple fields that remain
derivable in the native three-hidden-seat case.

**[CONSTRUCTED COUNTEREXAMPLE: canonical reduction is not
transition-stable]** Let three seats have

\[
U=\{a,b,c\},\qquad
(k_0,k_1,k_2)=(0,1,2),
\]

\[
P_0=\varnothing,\qquad
P_1=\{a,b\},\qquad
P_2=\{a,b,c\}.
\]

This system is already reduced: its two worlds are

\[
(H_1,H_2)=(\{a\},\{b,c\})
\quad\text{and}\quad
(\{b\},\{a,c\}),
\]

so every displayed holder edge occurs. Now observe seat 2 play \(a\). The raw
lead/removal update gives

\[
U'=\{b,c\},\qquad
(k_0',k_1',k_2')=(0,1,1),
\]

\[
P_0'=\varnothing,\qquad
P_1'=\{b\},\qquad
P_2'=\{b,c\}.
\]

Its sole world assigns \(b\) to seat 1 and \(c\) to seat 2, so the edge
\(b\to2\) has become unsupported. Thus a reduced predecessor can have a raw
successor that is not reduced. A representation that elects to remain in
canonical reduced form must apply the exact physical/support update and then
apply \(\operatorname{red}\) again. The rule-derived local cells remain exact
without performing that optional normalization.

**[THEOREM — exhaustive finite verification]** On the 66,968 abstract
three-seat systems of §7.7, the verifier derives \(P_s^\star\) both from direct
world projection and from forced-successor Hall feasibility, requires exact
agreement, and confirms fiber preservation, contractivity, idempotence, and the
canonical-normal-form equivalence throughout the stated finite domain.

### 7.10 Global representation minimality and the exact support normal form

There are two different meanings of “minimal representation.” They must not be
conflated.

- **Semantic/state-count minimality** asks whether two representation states may
  be merged while still decoding exactly the same support set.
- **Encoding or execution minimality** asks for the fewest bits, operations,
  memory transactions, or some other declared cost. No encoding is absolutely
  minimal without fixing that cost model and the operations it must support.

This subsection proves the strongest cost-model-independent result. The literal
capacity-cell tuple is not minimal. Exact support itself is the quotient, and a
canonical tagged normal form realizes exactly one representation state per
distinct support set.

Fix the three labeled hidden seats \(J\), and let \(\mathscr C\) be the class of
all finite three-seat cell systems over the domino universe, including
infeasible systems. Let

\[
\mathscr C_+=\{\mathbf C\in\mathscr C:\Phi(\mathbf C)\ne\varnothing\}.
\]

Two systems are **extensionally support-equivalent** when their sets of labeled
hidden-hand triples are equal.

**[THEOREM — proved mathematically: nonempty support recovery]** For
\(\mathbf C,\mathbf Q\in\mathscr C_+\),

\[
\Phi(\mathbf C)=\Phi(\mathbf Q)
\iff
\operatorname{red}(\mathbf C)=\operatorname{red}(\mathbf Q),
\]

where equality of reduced systems includes the pool, labeled capacities, and
marginal holder sets. Thus the fixed-schema normal form of §7.9 extends across
varying pools and capacities on the nonempty domain.

**Proof.** The reverse implication follows from fiber preservation. For the
forward implication, choose any world in the common nonempty fiber. Its union
of labeled hands recovers the pool, and each hand cardinality recovers the
corresponding capacity. For every domino \(d\) and seat \(s\), membership of
\(d\) in the marginal holder set is exactly the statement that some world in
the common fiber assigns \(d\) to \(s\). Hence the pool, capacities, and every
marginal holder edge are determined by the fiber. ∎

For a nonempty reduced system, define the exact holder set of each domino

\[
A(d)=\{s\in J:d\in P_s^\star\}.
\]

Separate the **certain hidden-location marks**

\[
K_s=\{d\in U:A(d)=\{s\}\}
\]

from the **ambiguous pool**

\[
W=U\setminus\bigsqcup_{s\in J}K_s,
\]

and define residual capacities

\[
r_s=k_s-|K_s|.
\]

Every \(d\in W\) has at least two possible holders. Let
\(J^+=\{s:r_s>0\}\).

**[LEMMA — proved mathematically: native active-seat trichotomy]** With three
hidden seats,

\[
|J^+|\in\{0,2,3\};
\]

one active seat is impossible. With two active seats every ambiguous domino is
possible at both. With three active seats every ambiguous domino is possible
at all three or excludes exactly one.

**Proof.** An ambiguous domino must have at least two holders, and a
zero-residual seat cannot hold one. Thus a nonempty \(W\) requires at least two
active seats. If \(W=\varnothing\), conservation gives every residual capacity
zero. The remaining holder-pattern statements follow because a subset of a
two-element active set with cardinality at least two is the whole set, while a
subset of a three-element set with cardinality at least two is either the whole
set or omits exactly one seat. ∎

The ambiguity object is therefore a canonical tagged sum rather than a generic
three-cell tuple. Fix the clockwise hidden-seat order relative to the viewer.

\[
\mathcal K_{\mathrm{amb}}
\in
\begin{cases}
\mathsf{Determinate}, & W=\varnothing,\\[2mm]
\mathsf{Binary}(\iota,W,q), & |J^+|=2,\\[2mm]
\mathsf{Ternary}(W,r_0,r_1,\varepsilon), & |J^+|=3.
\end{cases}
\]

In the binary case \(\iota\) is the one inactive hidden seat. Let
\((a,b)\) be the canonically ordered pair \(J\setminus\{\iota\}\). Then

\[
1\le q<|W|,
\qquad
r_a=q,
\qquad
r_b=|W|-q,
\qquad
r_\iota=0.
\]

Only the inactive seat is stored; the active pair is its complement. No
per-domino holder relation is stored because every domino in \(W\) can occupy
either active seat.

In the ternary case all three residual capacities are positive,

\[
r_2=|W|-r_0-r_1,
\]

and the sparse partial exclusion function is

\[
\varepsilon:W\rightharpoonup J,
\qquad
\varepsilon(d)=s
\iff
A(d)=J\setminus\{s\}.
\]

An undefined value means \(A(d)=J\). Only two independent residual capacities
are stored.

**[DEFINITION]** The **feasible exact support normal form** is

\[
\mathcal N(\mathbf C)
=
\left((K_s)_{s\in J},\mathcal K_{\mathrm{amb}}\right)
\qquad(\mathbf C\in\mathscr C_+).
\]

Its decoded worlds are \(H_s=K_s\sqcup G_s\), where the ambiguous hands
partition \(W\), meet the residual capacities, and obey the ternary exclusions.
The certain sets are exact location marks, not “unknowns with singleton
support.” The tagged ambiguity component is precisely the delimited ignorance
that remains after those exact marks are extracted.

**[THEOREM — proved mathematically: exact minimized feasible payload]** On
\(\mathscr C_+\), \(\mathcal N\) is in bijection with the reduced cell normal
form and therefore with exact nonempty support fibers. It removes from the
literal capacity-cell presentation:

1. the separately stored pool, recovered from the certain and ambiguous sets;
2. all three full possible-holder sets;
3. all certain-holder edges from the ambiguity engine;
4. all zero-residual seats from the ambiguity engine;
5. the explicit binary active pair, recovered as the complement of one inactive
   seat;
6. one residual capacity by conservation;
7. every per-domino holder field in the binary case;
8. every positive holder edge in the ternary case, retaining only actual
   single-seat exclusions.

**Proof.** The displayed components are canonical functions of the reduced
holder relation. Conversely, the tagged decoder reconstructs every exact holder
set: singleton \(K_s\) tiles have holder \(s\); binary tiles have holders
\(J\setminus\{\iota\}\); ternary tiles have holders \(J\) when \(\varepsilon\)
is undefined and \(J\setminus\{\varepsilon(d)\}\) otherwise. It reconstructs
\(U=W\sqcup\bigsqcup_sK_s\) and \(k_s=|K_s|+r_s\), with omitted seats and
residual capacities supplied by complement and conservation. Thus it
reconstructs \(\operatorname{red}(\mathbf C)\) exactly. ∎

The nonempty hypothesis above is needed only to recover type from a world. If
support semantics is the ordinary extensional set \(\Phi(\mathbf C)\), every
infeasible system denotes the same empty set. Define the **total exact support
normal form**

\[
\overline{\mathcal N}(\mathbf C)
=
\begin{cases}
\mathsf{Empty}, & \Phi(\mathbf C)=\varnothing,\\[2mm]
\mathsf{Feasible}(\mathcal N(\mathbf C)), & \Phi(\mathbf C)\ne\varnothing.
\end{cases}
\]

**[THEOREM — proved mathematically: global representation-minimal support
quotient]** For every \(\mathbf C,\mathbf Q\in\mathscr C\),

\[
\Phi(\mathbf C)=\Phi(\mathbf Q)
\iff
\overline{\mathcal N}(\mathbf C)
=
\overline{\mathcal N}(\mathbf Q).
\]

Let an exact deterministic support representation be a map
\(E:\mathscr C\to R\) with a decoder \(D\) satisfying

\[
D(E(\mathbf C))=\Phi(\mathbf C).
\]

Then there is a unique map on the image of \(E\),

\[
\gamma:E(\mathscr C)\to\overline{\mathcal N}(\mathscr C),
\]

such that

\[
\overline{\mathcal N}=\gamma\circ E.
\]

Consequently every exact deterministic representation refines
\(\overline{\mathcal N}\): it cannot merge distinct total normal forms. Equivalently,
\(\overline{\mathcal N}\) induces a bijection

\[
\mathscr C/{\sim_\Phi}
\;\cong\;
\overline{\mathcal N}(\mathscr C),
\qquad
\mathbf C\sim_\Phi\mathbf Q
\iff
\Phi(\mathbf C)=\Phi(\mathbf Q),
\]

whose inverse is exact decoding. There is therefore no nontrivial further
semantic quotient that still decodes exact support. On any finite subdomain,
every exact representation must have at least as many reachable representation
states as there are distinct support sets, and \(\overline{\mathcal N}\) attains
that lower bound exactly.

**Proof.** If both fibers are empty, both total normal forms are
\(\mathsf{Empty}\). If they are nonempty, the nonempty recovery theorem and the
bijection above give equality of feasible normal forms. The converse follows by
exact decoding. For factorization, define
\(\gamma(E(\mathbf C))=\overline{\mathcal N}(\mathbf C)\). If
\(E(\mathbf C)=E(\mathbf Q)\), exact decoding gives equal fibers and therefore
equal total normal forms, so \(\gamma\) is well defined. Its value is forced at
every point of \(E(\mathscr C)\), proving uniqueness. The finite state-count
bound is the induced injection from distinct quotient classes into
\(E(\mathscr C)\). ∎

This is the requested global representation-minimality theorem. It disproves
global minimality of the literal capacity cells and replaces them with the
coarsest exact support quotient. It does **not** select a universal byte layout
or execution strategy; those require a cost model.

An API that deliberately treats the world type \((U,(k_s)_s)\) as part of empty
support semantics should use \(\mathsf{Empty}(U,(k_s)_s)\) instead of one
untyped \(\mathsf{Empty}\) tag. The same proof applies within each typed world
space. Reachable Straight 42 information states are feasible, so their runtime
support uses the feasible branch.

**[THEOREM — proved mathematically: one-assignment marginal-support
compiler]** Let \(\mathbf C\in\mathscr C_+\), choose any world
\(M\in\Phi(\mathbf C)\), and form a directed graph on domino vertices \(U\)
and seat vertices \(J\):

- orient the holder edge used by \(M\) as \(s\to d\);
- orient every other locally allowed edge as \(d\to s\).

For an unused locally allowed edge \(d\to s\),

\[
d\in P_s^\star
\iff
s\text{ reaches }d
\iff
d\text{ and }s\text{ lie in the same strongly connected component}.
\]

Every used holder edge is already marginally supported. Hence one feasible
assignment plus one strongly-connected-component pass recovers the complete
marginal holder relation and therefore \(\mathcal N(\mathbf C)\).

**Proof.** If \(s\) reaches \(d\), the unused edge \(d\to s\) closes a directed
alternating cycle. Reversing holder choices around that cycle preserves one
holder per domino and every seat capacity, producing a world that uses
\(d\to s\). Conversely, if another world uses \(d\to s\), compare it with
\(M\). At each domino where they differ, the old holder edge points into the
domino and the new holder edge points out; at each seat, equal capacities make
incoming and outgoing differences balance. The directed symmetric difference
decomposes into cycles, and the cycle containing \(d\to s\) supplies a path
from \(s\) back to \(d\). ∎

Thus exact normal-form compilation does not require one independent Hall solve
per candidate edge. A feasibility assignment may be obtained by max flow or any
other exact method; the marginal reduction then uses one linear graph pass over
at most 21 domino vertices, three seat vertices, and 63 local holder edges.

**[COROLLARY — proved mathematically: reachable witness and tag erasure]** A
certified reachable objective state already contains one actual current
remainder world \(M\in\Phi(c)\). Therefore support is nonempty, the total
normal form's `Empty/Feasible` tag is unnecessary inside the certified runtime
type, and normal-form compilation may begin directly with the SCC pass using
that actual world rather than running a separate feasibility search.

**Proof.** Reachability supplies the complete deal that generated the state;
removing the publicly played tiles gives a current remainder world. Exact cell
losslessness places it in \(\Phi(c)\). The SCC compiler's output is the
world-independent marginal relation proved above, so using the actual world as
its witness changes neither support nor the information returned to the
player. ∎

The witness must remain an internal compiler input. Exposing it, branching
semantically on its identity, or caching witness-dependent output would leak
hidden information and violate the theorem's interface.

**[COROLLARY — proved mathematically: zero supplemental support state]** If a
containing mechanical coordinate retains the fields in §6.4 from which the
rule-derived cells are reconstructed, then

\[
\mathbf C(c),\qquad \Phi(c),\qquad
\overline{\mathcal N}(c)
\]

are deterministic functions of that coordinate. An independently serialized
cell tuple, reduced relation, normal form, or fiber carries no additional
semantic information. The minimal **supplemental** support state relative to
that coordinate is therefore the one-element type; any materialized support
object is a cache or compiled view.

**Proof.** Section 6.4 gives deterministic formulas for \(U,k_s,P_s\) from the
mechanical fields. Sections 7.3 and 7.10 deterministically map those cells to
the fiber and total support normal form. Appending a deterministic function of
an already retained object does not refine its information partition. ∎

This corollary is context-relative. If a proposed mechanical coordinate drops
one of the fields needed to derive exact support, a separate support payload may
again be necessary. It also says nothing about whether caching a compiled view
is faster for a named workload.

### 7.11 Strict Hall irreducibility and essential exclusions

For the ambiguity component of the support normal form, let

\[
N(R)=\bigcup_{s\in R}\{d\in W:s\in A(d)\}
\]

for \(R\subseteq J^+\), and let \(r(R)=\sum_{s\in R}r_s\).

**[THEOREM — proved mathematically: strict Hall ambiguity component]** If
\(W\ne\varnothing\), then for every nonempty proper subset
\(R\subsetneq J^+\),

\[
|N(R)|\ge r(R)+1.
\]

Therefore the ambiguity component has no nontrivial Hall-tight seat subset
and no further independent matching component.

**Proof.** Ordinary Hall gives \(|N(R)|\ge r(R)\). Suppose equality. Every
world assigns exactly \(r(R)\) dominoes to seats in \(R\), all drawn from the
\(r(R)\)-element set \(N(R)\). Hence every domino of \(N(R)\) is assigned to
\(R\) in every world and has no marginal holder outside \(R\). By definition,
no domino outside \(N(R)\) has a holder in \(R\). Thus the marginal holder
graph separates \(R\) from \(J^+\setminus R\).

Both sides have positive residual capacity. With two active seats, each side is
a singleton, forcing every domino on either side to have one holder, contrary
to the definition of \(W\). With three active seats, either \(R\) or its
complement is a singleton; dominoes on that side again have one holder,
contradiction. Therefore equality is impossible. ∎

**[THEOREM — proved mathematically: every stored exclusion is essential]** In
a ternary ambiguity component, let \(d\in W\) exclude seat \(s\). If
that one exclusion is removed while every other ambiguity-component field is
held fixed,
the decoded fiber strictly enlarges. Equivalently, there is a newly admitted
world assigning \(d\) to \(s\).

**Proof.** Force \(d\) to \(s\), remove \(d\) from \(W\), and decrement
\(r_s\). For any seat subset \(R\) containing \(s\), its successor neighbor
set loses at most \(d\), while its required capacity also falls by one, so
ordinary Hall remains true. For any nonempty \(R\) not containing \(s\), the
capacity is unchanged and the neighbor set loses at most \(d\); strict Hall
from the preceding theorem supplies the required one unit of slack. The full
active-seat set satisfies equality by conservation. Therefore the forced
successor is Hall-feasible. Adding \(d\) back at \(s\) gives a world that was
forbidden before the exclusion was removed. ∎

Thus the ternary ambiguity component is not merely sparse. Inside the direct
holder/quota language, every stored negative holder fact changes the world set.
There are no unsupported positive edges, no certain tiles left in the ambiguous
domain, no zero-capacity active seats, no redundant final capacity, and no
removable exclusion.

Let \(n_s=|\{d\in W:\varepsilon(d)=s\}|\) and \(n=|W|\).

**[COROLLARY — proved mathematically: linear ternary normal-form validator]** A
candidate ternary ambiguity payload with positive residual capacities and
\(\sum_sr_s=n\) is a nonempty reduced exact support component if and only if

\[
n-n_s\ge r_s+1
\qquad(s=0,1,2).
\]

Consequently validation needs one pass counting the three excluded-seat
categories and three integer comparisons; no Hall search, matching, or world
enumeration is required.

**Proof.** Necessity is strict Hall for each singleton seat. Conversely, the
three inequalities give singleton Hall with one unit of slack. Every two-seat
subset neighbors all of \(W\), because a domino excludes at most one seat, and
its quota is \(n-r_s\le n-1\) because the omitted seat has positive residual
capacity. The full set satisfies equality by conservation. Hall gives a world,
and the forced-edge argument in the essential-exclusion proof shows every
allowed edge occurs in a world, so the payload is reduced. ∎

The binary validator is the corresponding closed form:
\(W\ne\varnothing\) and \(1\le q<|W|\). The determinate branch requires an
empty ambiguity pool and zero residual capacities.

The corresponding slot-expanded bipartite graph is connected and every edge
lies in a perfect matching. The strict Hall proof above is the native
three-seat form of that matching-covered irreducibility; no matching-theory
terminology is required by the result.

### 7.12 Exact compiled forms of the support normal form

The support normal form is the semantic support object. Several smaller or
faster exact compiled forms follow from its ambiguity component. They are
compiled representations of one fixed support normal form, not new meanings of
support.

#### 7.12.1 Closed forms, complete count signatures, and exact memo tables

If \(|J^+|=0\), the fiber is the singleton certain assignment.

If \(|J^+|=2\), let \((a,b)\) be the ordered active pair, let
\(n=|W|\), and let \(q=r_a\). Every ambiguous domino can be held by either
seat, so choosing the \(q\)-element hand of seat \(a\) determines the other
hand:

\[
|\Phi|=\binom{n}{q}.
\]

Uniform sampling is exactly uniform fixed-size subset sampling; no fiber DP is
needed. For count and sampler compilation only, the involution
\(q\leftrightarrow n-q\) is a gauge: use

\[
\bar q=\min(q,n-q)
\]

and complement the sampled subset when the canonical side is the other active
seat. Native binary ambiguity has \(n\le14\), so

\[
|\Phi|\le\binom{14}{7}=3432<2^{12}.
\]

This binary gauge preserves counts and transports worlds; it does not erase the
actual labeled-seat capacity from the semantic normal form.

If \(|J^+|=3\), define

\[
W_\star=W\setminus\operatorname{dom}\varepsilon,
\qquad
W_s=\{d\in W:\varepsilon(d)=s\},
\]

and \(n_\star=|W_\star|\), \(n_s=|W_s|\). Exact fiber cardinality is

\[
[x_0^{r_0}x_1^{r_1}x_2^{r_2}]
(x_0+x_1+x_2)^{n_\star}
(x_1+x_2)^{n_0}
(x_0+x_2)^{n_1}
(x_0+x_1)^{n_2}.
\]

Thus counting depends on domino identities only through four eligibility-class
sizes. For a category-allocation matrix \(a_{c,s}\) whose row sums are the
category sizes, whose column sums are the residual capacities, and whose
forbidden diagonal entries are zero, the number of labeled worlds with that
matrix is

\[
w(a)=\prod_c\frac{n_c!}{\prod_s a_{c,s}!}.
\]

Therefore

\[
|\Phi|=\sum_a w(a).
\]

The matrix sum has a smaller native parameterization. Let \(x_0\) be the
number of \(W_0\) tiles assigned to seat 1, \(x_1\) the number of \(W_1\)
tiles assigned to seat 0, and \(x_2\) the number of \(W_2\) tiles assigned to
seat 0. The unrestricted-row allocations are then forced:

\[
\begin{aligned}
y_0&=r_0-x_1-x_2,\\
y_1&=r_1-x_0-(n_2-x_2),\\
y_2&=r_2-(n_0-x_0)-(n_1-x_1).
\end{aligned}
\]

Summing over triples for which every \(y_s\ge0\) gives

\[
|\Phi|
=
\sum_{x_0=0}^{n_0}
\sum_{x_1=0}^{n_1}
\sum_{x_2=0}^{n_2}
\binom{n_0}{x_0}
\binom{n_1}{x_1}
\binom{n_2}{x_2}
\frac{n_\star!}{y_0!y_1!y_2!}.
\]

This examines at most

\[
(n_0+1)(n_1+1)(n_2+1)\le512
\]

candidate split triples under native bounds. The inequality follows because
\(n_0+n_1+n_2\le21\), so the three positive integers \(n_s+1\) have sum at
most 24 and their product is maximized by \(8,8,8\). At most 114 candidates are
feasible for any reduced native signature.

Sampling a feasible matrix with probability \(w(a)/|\Phi|\), then uniformly
partitioning the labeled dominoes inside each category according to that row,
produces the exact uniform fiber law. Each labeled world has one allocation
matrix and is selected with probability \(1/|\Phi|\).

The four-count histogram is sufficient for cardinality and category-level
sampling but not for world-level physics: trick relations still depend on which
domino identities occupy each category.

**[THEOREM — proved mathematically: complete six-integer ternary count
signature]** Let

\[
n=r_0+r_1+r_2,
\qquad
n_\star=n-(n_0+n_1+n_2),
\]

with every \(r_s\in\{1,\ldots,7\}\) and every \(n_s\ge0\). The six integers

\[
(r_0,n_0,r_1,n_1,r_2,n_2)
\]

define a nonempty reduced ternary ambiguity signature exactly when

\[
n_\star\ge0
\quad\text{and}\quad
n-n_s\ge r_s+1
\qquad(s=0,1,2).
\]

The unrestricted category size \(n_\star\) is derived and must not be stored as
a seventh independent key field.

**Proof.** Necessity is the singleton case of strict Hall together with category
conservation. For sufficiency, the singleton inequalities give Hall with slack.
Every two-seat set neighbors all of \(W\), because each domino excludes at most
one seat, and its required capacity is \(n-r_s\le n-1\); the full set satisfies
equality by conservation. Hall therefore gives a world. To show reduction,
force any allowed edge \(d\to s\), remove \(d\), and decrement \(r_s\). Seat
subsets containing \(s\) lose at most one neighbor while their quota also falls
by one; subsets not containing \(s\) use the displayed one-unit slack. The
forced successor is feasible, so every allowed edge occurs in some world. ∎

Under native bounds there are exactly **136,514 seat-labeled** six-integer
ternary signatures satisfying these conditions. Across them there are
**1,667,666** feasible category-allocation matrices, with at most **114** for
any one signature. These are exhaustive finite-verification counts, not
assumptions of the analytic theorem.

**[THEOREM — proved mathematically: complete signature relabeling gauge]** The
eligibility incidence structure has one unrestricted category and three
excluded-seat categories. Every row/column-role-preserving automorphism fixes
the unrestricted category and simultaneously permutes seats and their
corresponding excluded-seat categories. Its full structural automorphism group
is therefore \(S_3\).

Consequently a complete count/sampling memo key modulo universal relabeling is

\[
\lambda
=
\operatorname{MSet}\{(r_s,n_s):s=0,1,2\}.
\]

The value \(n_\star=\sum_s r_s-\sum_s n_s\) is recovered from \(\lambda\). A
selected canonical permutation is inverted when an allocation is returned; a
deterministic original-seat tie-break resolves equal pairs.

**Proof.** The unrestricted category is the unique category adjacent to all
three seats, so it is fixed. Each remaining category has exactly one
nonneighbor, and that nonneighbor uniquely identifies the corresponding seat.
Thus any structure-preserving relabeling is one simultaneous seat/exclusion
permutation, and every such permutation is valid. Count coefficients and
multinomial weights are unchanged by renaming variables and rows together. ∎

Under native bounds the 136,514 labeled signatures collapse to exactly
**23,842** \(S_3\)-orbits. Their canonical representatives contain **296,721**
feasible labeled allocation matrices in total, still at most 114 for one
representative. Domino identity never enters these table keys.

A canonical signature can have a nontrivial stabilizer

\[
G_\lambda
=
\{p\in S_3:(r_{p(s)},n_{p(s)})=(r_s,n_s)\text{ for all }s\}.
\]

It acts on the representative's allocation matrices. Matrices in one orbit have
the same multinomial weight.

**[COROLLARY — proved mathematically: stabilizer-orbit sampling table]** An exact
category-level sampler may store one matrix representative per
\(G_\lambda\)-orbit, weight that orbit by \(|\mathcal O|w(a)\), choose an orbit
by exact weight, then choose uniformly among its distinct matrix images. This
is exactly the same law as storing every labeled matrix.

**Proof.** Every matrix in an orbit has weight \(w(a)\), so the orbit contains
exactly \(|\mathcal O|w(a)\) labeled worlds. The two-stage choice gives each
matrix probability \(w(a)/|\Phi|\), after which uniform within-category
partitioning gives each world probability \(1/|\Phi|\). ∎

The 23,842 canonical signatures split into 21,686 trivial-stabilizer, 2,121
order-two-stabilizer, and 35 full-\(S_3\)-stabilizer cases. Their 296,721 labeled
matrices collapse to **279,048 stabilizer orbits**, at most **103** orbits for
one signature. Orbit sizes are among \(\{1,2,3,6\}\).

Every count, matrix weight, and orbit mass is below \(2^{29}\). Strict Hall and
native capacities imply \(n_s\le13\), so each split value fits in four bits.
The canonical six-integer key fits in 21 bits: three bits for each
\(r_s-1\in\{0,\ldots,6\}\) and four bits for each \(n_s\in\{0,\ldots,13\}\),
after sorting the three pairs. A dense canonical-signature identifier needs 15
bits because \(23{,}842<2^{15}\). A matrix representative needs 12 split bits;
its orbit size has four possibilities and needs two more bits. A packed header
needs 19 offset bits, seven length bits, and 29 count bits, for 55 total.

These facts give constructive exact storage bounds:

- a tightly bit-packed count table uses **86,428 bytes**; a simple `uint32`
  table uses 95,368 bytes;
- a tightly packed stabilizer-orbit split table uses **652,248 bytes** under
  dense indexing, or 714,834 bytes with one explicit packed 21-bit key per
  header;
- a tightly packed split-plus-orbit-mass table uses **1,663,797 bytes** under
  dense indexing, or 1,726,383 bytes with explicit packed keys;
- simple aligned forms using `uint16` split/orbit records and `uint64` headers
  use 748,832 bytes, while `uint64` split/weight/orbit records and headers use
  2,423,120 bytes; adding one `uint32` key per header raises them to 844,200 and
  2,518,488 bytes respectively.

The packed counts are exact concatenated bitstream sizes rounded up once at the
byte boundary; aligned sizes are ordinary array payloads before container
metadata or additional alignment. These are constructive upper bounds for
named layouts, not universal storage minima. Computing all at most 512 candidate
splits on demand requires no matrix table at all.

In a ternary ambiguity component the four holder categories are all semantically
possible: unrestricted or excluded from exactly one of three seats. Therefore a
fixed-width dense per-domino category code needs at least two bits and a two-bit
code attains the bound. The sparse exclusion map is usually smaller when
unrestricted tiles dominate; the dense two-bit code is a proved exact
GPU-friendly alternative, not a different support object.

#### 7.12.2 Complement-elided explicit world storage

Certain sets \(K_s\) belong to the support normal form and are not repeated in each world.
For one decoded world on \(W\), store hands for only the first
\(|J^+|-1\) active seats; the last hand is the complement in \(W\). Therefore:

- zero active seats require no world payload;
- two active seats require one \(|W|\)-bit subset mask, with \(|W|\le14\);
- three active seats require two disjoint \(|W|\)-bit subset masks, with
  \(|W|\le21\).

Thus a native binary world needs at most 14 raw mask bits and a native ternary
world at most 42 raw mask bits under a normal-form-local domino order. A three-hand payload is
redundant within the raw subset-mask schema. This is not a global bit-minimum;
§7.12.4 gives the fiber-local information bound.

#### 7.12.3 Minimal ordered completion automaton

Fix an order \(d_0,\ldots,d_{n-1}\) of \(W\). Encode a world as the holder
string in that order. After a prefix, let

\[
v=(v_s)_{s\in J^+}
\]

be the remaining residual-capacity vector. The next position is not an
independent state field:

\[
i=n-\sum_{s\in J^+}v_s.
\]

Thus \(v\) alone determines both the next ambiguous domino and every remaining
quota. Retain exactly the residual vectors that are reachable from the root and have
at least one accepted completion. The resulting partial acyclic deterministic
graph is the **ordered completion automaton**.

**[THEOREM — proved mathematically: minimal ordered automaton]** For the fixed
domino order, the reachable-and-coaccessible residual-capacity vectors form
the unique minimal partial deterministic acyclic automaton, up to isomorphism, accepting
exactly the normal form's ambiguous-holder strings. No separate position field
is required.

**Proof.** Equal residual vectors have equal coordinate sum, hence the same
derived position, the same remaining domino suffix, the same eligibility
constraints, and the same remaining quotas. They therefore have the same
accepted suffix language and may be merged.

For distinct residual vectors, if their coordinate sums differ then their
accepted suffixes have different lengths. If their sums agree, the vectors
require different counts of at least one holder symbol in every accepted
suffix, so their nonempty suffix languages are disjoint. Thus no two retained
states are equivalent. The right-language characterization of deterministic
automaton minimization gives minimality and uniqueness up to isomorphism. ∎

A backward exact completion-count table constructs this automaton, removes all
zero-completion residual vectors, counts the fiber, and supplies exact branch
weights for uniform sampling. A standalone automaton state stores only its
residual vector. When an external layer/position loop is already present, one
residual coordinate is derived from that layer and only \(|J^+|-1\) counters
need be materialized. Forced extraction replaces the original capacities by
the usually smaller residual capacities before this table is built.

**[COROLLARY — proved mathematically: optimal native nine-bit residual
state]** Every native ternary residual coordinate lies in \(\{0,\ldots,7\}\),
so the direct code

\[
\operatorname{code}(v)=v_0+2^3v_1+2^6v_2
\]

uses nine bits and is injective. Nine bits are necessary for one fixed-width
code that supports every native ternary ordered automaton: for the unrestricted
initial component with \(r=(7,7,7)\), every one of the \(8^3=512\) residual
vectors is coaccessible. This is a universal native-state-code bound; a
particular smaller automaton may admit a smaller dense local identifier. ∎

#### 7.12.4 Fiber-local optimal world ranks

Let \(N=|\Phi|\) and fix the domino and holder orders. Lexicographic rank gives
a bijection

\[
\Phi\longleftrightarrow\{0,\ldots,N-1\}.
\]

The completion-count table supports exact rank and unrank. A fixed-width binary
encoding requires exactly

\[
\lceil\log_2N\rceil
\]

bits, and no injective fixed-width binary code can use fewer by the pigeonhole
principle. Because every native Straight 42 fiber has
\(N\le399{,}072{,}960<2^{29}\), every world admits a fiber-local 29-bit rank.
The rank is meaningful only together with its support normal form and ordering; it is not a
context-free world identifier, and transition-heavy code may prefer the
42-bit two-mask form.

#### 7.12.5 Native standalone support census and optimal fixed-width code

The semantic quotient theorem also permits an exact global bit bound once the
encoding problem is stated. Fix the 28 labeled domino identities, one labeled
three-hidden-seat frame, capacities at most seven, and ordinary extensional
empty-support semantics. Let 
\(\mathscr S_{28,7}\) be the image of the total support normal form over this
full native cell-schema domain. This is the standalone support component: no
mechanical coordinate is supplied as external context.

For nonnegative bounds \(b=(b_0,b_1,b_2)\), define

\[
F(R;b)
=
\sum_{\substack{0\le c_s\le b_s\\ c_0+c_1+c_2\le R}}
\frac{R!}{c_0!c_1!c_2!(R-c_0-c_1-c_2)!}.
\]

This counts assignments of \(R\) labeled dominoes to three certain-holder
categories and one outside-pool category, with at most \(b_s\) certain tiles at
seat \(s\).

The determinate branch count is

\[
N_{\mathrm{det}}=F(28;(7,7,7)).
\]

For a binary branch, choose the inactive seat \(\iota\), let \((a,b)\) be the
canonically ordered active pair, choose residuals
\(r_a,r_b\in\{1,\ldots,7\}\), put \(r_\iota=0\) and
\(n=r_a+r_b\). Its contribution is

\[
\binom{28}{n}
F\left(28-n;(7-r_0,7-r_1,7-r_2)\right),
\]

summed over all such choices. The binomial factor chooses the one binary
ambiguity category.

For a valid ternary six-integer signature
\(\lambda=(r_0,n_0,r_1,n_1,r_2,n_2)\), let

\[
n=r_0+r_1+r_2,
\qquad
n_\star=n-n_0-n_1-n_2.
\]

Its contribution is

\[
\frac{28!}
{(28-n)!\,n_0!n_1!n_2!n_\star!}
F\left(28-n;(7-r_0,7-r_1,7-r_2)\right).
\]

The first factor assigns labeled dominoes to the four ternary ambiguity
categories; \(F\) assigns every remaining domino either to a certain holder or
outside the hidden pool.

**[THEOREM — proved mathematically: exact native support-state census]** The
four disjoint branches of \(\mathscr S_{28,7}\) contain exactly

\[
\begin{aligned}
N_{\mathrm{empty}}&=1,\\
N_{\mathrm{det}}&=8{,}102{,}258{,}940{,}222{,}814,\\
N_{\mathrm{bin}}&=11{,}495{,}078{,}055{,}913{,}018{,}482,\\
N_{\mathrm{ter}}&=1{,}830{,}955{,}704{,}129{,}296{,}418{,}354{,}864.
\end{aligned}
\]

Therefore

\[
|\mathscr S_{28,7}|
=
1{,}830{,}967{,}207{,}309{,}611{,}271{,}596{,}161,
\]

and

\[
2^{80}<|\mathscr S_{28,7}|<2^{81}.
\]

Consequently **81 bits are necessary and sufficient** for one universal
fixed-width binary code for the standalone exact support quotient on this full
native cell-schema domain. Removing the unreachable `Empty` branch does not
change the bound because the feasible count alone still exceeds \(2^{80}\).

**Proof.** The normal-form trichotomy is disjoint and exhaustive. In the
determinate branch, every domino is outside the hidden pool or certain at one
of three seats, and the capacity limit is exactly the bound in \(F\). In a
binary branch, the ambiguity pool is one holder category; after choosing it,
all remaining labels are certain or outside, with bounds reduced by the two
positive residual capacities. In a ternary branch, the six-integer validity
theorem enumerates every and only reduced residual/category signature. The
multinomial factor chooses its four ambiguity categories, and \(F\) chooses
the certain and outside categories. Different branch data decode to different
support fibers by the global quotient theorem, so the displayed sums neither
omit nor double-count a state.

Evaluating the finite sums gives the displayed integers. Any fixed-width
binary code for \(N\) states needs at least \(\lceil\log_2N\rceil\) bits.
Conversely, order the four branches, their valid parameter tuples, and their
labeled category assignments lexicographically. Combinatorial rank/unrank
using the displayed block counts maps the quotient bijectively to
\(\{0,\ldots,N-1\}\), attaining 81 bits without a table of all states. ∎

This is a global fixed-width minimum for a **standalone support state** on the
stated full cell-schema domain. It is not a transition-cost optimum. A
mechanical state that already derives support needs zero supplemental support
bits by §7.10. Section 7.13 restricts the quotient to exact Straight-reachable
support, proves a 26-bit lower bound and a 46-bit necessary-outer-profile ceiling for
a standalone reachable-support identifier, and leaves the exact reachable
cardinality unresolved. The 81-bit support-state rank is also different from
the at-most 29-bit rank of one world *inside one supplied fiber*.

**[THEOREM — exhaustive finite verification]** On every feasible system among
the 66,968 abstract three-seat systems of §7.7, the verifier constructs the
support normal form and confirms exact decode equality. It independently
computes marginal holder support by direct world projection, forced-successor
Hall feasibility, and the one-assignment SCC compiler, requiring exact
agreement for **every feasible witness world** in each tiny system. It confirms
that active-seat count is never one, every binary component
is unrestricted, and strict Hall holds for every proper active-seat subset. In
the ternary cores it removes each stored exclusion in turn and confirms a
strict fiber increase. It also checks the determinate and binary closed forms,
the grouped ternary coefficient, ordered-automaton acceptance/counts, and exact
rank/unrank on the stated finite domain. For the unrestricted native
\((7,7,7)\) component it separately confirms that all 512 residual vectors are
coaccessible and map bijectively to the nine-bit codes 0 through 511.

A separate native-signature census confirms all 136,514 seat-labeled
six-integer ternary signatures, their 23,842 complete structural-relabeling
orbits, all 1,667,666 labeled matrices, and 296,721 matrices across canonical
signature representatives. Stabilizer reduction leaves 279,048 matrix orbits,
at most 103 for one signature; the unreduced maximum remains 114 matrices per
signature. It also confirms the 21-bit key, 15-bit dense identifier, and packed
table bounds stated above.


### 7.13 Straight reachability of exact support

The global quotient of §7.10 ranges over every finite three-seat capacity-cell
system in the stated native schema. A legal Straight 42 continuation visits a
strictly smaller domain. Reachability must be treated as an inductive property
of the game, not inferred from cell feasibility alone.

Fix a viewer \(m\), write the three hidden seats in clockwise viewer-relative
order as

\[
h_1=m+1,\qquad h_2=m+2,\qquad h_3=m+3\pmod 4,
\]

and let \(\mathcal H_{\mathrm{Str}}^m\) be the set of legal contracted-hand
public prefixes together with the viewer's current private hand under the
Straight profile. Define the **reachable support image**

\[
\mathscr R_{\mathrm{Str}}^m
=
\left\{
\mathcal N\!\left(\mathbf C(c(h,m))\right):
 h\in\mathcal H_{\mathrm{Str}}^m
\right\}.
\]

Every member is feasible; the `Empty` branch of the total normal form never
occurs inside this image.

**[COROLLARY — proved mathematically: viewer-relative gauge]** Simultaneous
rotation of seats sends \(\mathscr R_{\mathrm{Str}}^m\) bijectively to
\(\mathscr R_{\mathrm{Str}}^{m+r}\). After naming hidden seats by clockwise
offset from the viewer, all four reachable-support domains are the same labeled
object. A viewer-relative standalone support code therefore stores no absolute
viewer identifier.

**Proof.** Seat rotation preserves clockwise order, partnerships, legal deals,
auction/play transitions, and the viewer-relative hidden-seat labels. Rotating
a legal witness gives a legal witness for the rotated viewer, with inverse
rotation supplying the inverse map. ∎

**[THEOREM — proved mathematically: reachable-domain support minimality]** The
restriction of \(\mathcal N\) to legal Straight prefixes is the coarsest exact
deterministic representation of current rule support on the reachable domain.
More precisely, if

\[
E:\mathcal H_{\mathrm{Str}}^m\to R
\]

has an exact decoder \(D\) satisfying

\[
D(E(h))=\Phi(c(h,m)),
\]

then there is a unique map on \(E(\mathcal H_{\mathrm{Str}}^m)\) such that

\[
\mathcal N(\mathbf C(c(h,m)))=\gamma(E(h)).
\]

Thus reachability deletes unrealized support classes; it does not permit two
different reachable fibers to be merged.

**Proof.** Apply the factorization argument of the global quotient theorem to
the restricted domain. Equal representation values have equal decoded fibers,
and equal nonempty fibers have equal feasible normal forms. The factor map is
therefore well defined and forced. ∎

**[COROLLARY — proved mathematically: reachability carries no runtime flag]** A
type constructed only by a legal initial-state constructor and legal transition
constructors needs no stored `reachable` Boolean. Reachability is an inductive
invariant of the constructor path. A state accepted from an external source
requires an exact witness or validator, but the witness and validation tag may
be erased after certification.

**[CLARIFICATION — proof-irrelevant certification]** Reachability is the
proposition that at least one legal origin and prefix projects to the semantic
state. A particular origin, prefix, validator trace, or proof term is evidence
for that proposition, not an additional coordinate of the game. Two certified
values with equal semantic-state projections are therefore one game state even
when their witnesses differ. Provenance may be retained in a separate audit
object, but it must not refine game equality, hashing, serialization, policy
input, or transition.

This is distinct from support feasibility. Hall feasibility proves that a cell
system has at least one current hidden assignment. It does not prove that a
legal deal and legal public play prefix could have produced that system.

**[BOUNDARY — reachable support is not a transition state]** Membership in
\(\mathscr R_{\mathrm{Str}}^m\) says that at least one legal prefix realizes
the support. It does not retain declaration, actor, current trick, score, or a
particular realizing prefix. The same support can occur in mechanically
different states with different legal actions and successor supports. There is
therefore no exact game-transition function on reachable support identifiers
alone. Transition preservation belongs to the containing certified mechanical
state; the support identifier is a decoded component or cache.

#### 7.13.1 Reachable capacity profiles

Let \(j\in\{0,\ldots,7\}\) be the number of completed tricks. During an active
trick let \(a\in\{0,1,2,3\}\) be the number of already played tiles. Every seat
has then played either \(j\) or \(j+1\) tiles. Hence every hidden capacity is
\(7-j\) or \(6-j\).

**[THEOREM — proved mathematically: exact hidden-capacity reachability]** A
triple \(k=(k_1,k_2,k_3)\in\{0,\ldots,7\}^3\) occurs at some legal Straight
play prefix if and only if

\[
\max_s k_s-\min_s k_s\le1.
\]

There are exactly

\[
8+7(2^3-2)=50
\]

such labeled hidden-seat profiles.

**Proof.** Necessity is the completed-trick/current-prefix observation above.
For sufficiency, equal profiles \((h,h,h)\) occur at a completed-trick boundary
after \(7-h\) tricks. For a nonconstant profile let \(h=\max k_s\), let
\(j=7-h\), and let \(B=\{h_i:k_i=h-1\}\). The following legal current-trick
prefixes have exactly the indicated hidden low-capacity set:

| \(B\) | current-trick prefix |
|---|---|
| \(\varnothing\) | no hidden seat has acted |
| \(\{h_1\}\) | \((m,h_1)\) |
| \(\{h_2\}\) | \((h_2)\) |
| \(\{h_3\}\) | \((h_3)\) |
| \(\{h_1,h_2\}\) | \((m,h_1,h_2)\) |
| \(\{h_1,h_3\}\) | \((h_3,m,h_1)\) |
| \(\{h_2,h_3\}\) | \((h_2,h_3)\) |
| \(\{h_1,h_2,h_3\}\) | \((h_1,h_2,h_3)\) |

To realize each row physically, use no-trump and give the row's current leader
all seven doubles. That seat may lead and win the preceding \(j\) tricks with
distinct doubles and then begin the displayed current prefix. Every response
has at least one legal action. This constructs a legal witness for every
profile. The count consists of eight equal triples and, for each of seven
adjacent value pairs \(h-1,h\), six nonconstant labelings. ∎

Capacities are therefore derived from trick progress in any certified
mechanical state. They are not three independent three-bit fields.

#### 7.13.2 Observable lead contexts

For declaration \(\delta\), define the **lead-context image** and lead fiber

\[
\Lambda_\delta
=
\{\ell(d,\delta):d\in\mathcal D\},
\qquad
L_{\delta,q}
=
\{d\in\mathcal D:\ell(d,\delta)=q\}.
\]

**[THEOREM — proved mathematically: seven observable contexts]** Every Straight
declaration has exactly seven reachable lead contexts:

\[
\Lambda_t=(\mathbb P\setminus\{t\})\cup\{7\}
\quad\text{for pip trump }t,
\]

\[
\Lambda_{\mathsf{doubles}}=\{1,2,3,4,5,6,7\},
\]

\[
\Lambda_{\mathsf{notrump}}=\mathbb P.
\]

The seven lead fibers partition \(\mathcal D\), and their cardinalities are
always the multiset

\[
\{1,2,3,4,5,6,7\}.
\]

**Proof.** Under no-trump, a tile leads its maximum pip; context \(q\) has the
\(q+1\) tiles \(q:0,\ldots,q:q\). Under doubles-trump, the seven doubles lead
called context 7, while the mixed tiles with maximum \(q\in\{1,\ldots,6\}\)
form a lead fiber of size \(q\). Under pip trump \(t\), the seven called tiles
lead context 7. Order the remaining pips increasingly as
\(a_1<\cdots<a_6\). Context \(a_i\) is led by \(a_i:a_i\) and the \(i-1\)
uncalled mixed tiles joining \(a_i\) to earlier active pips, so its lead fiber
has size \(i\). These fibers are disjoint because every tile has one led
context, and their sizes sum to 28. ∎

A public void can be established only in a context in \(\Lambda_\delta\).
Thus a declaration-known Straight implementation needs at most seven void bits
per hidden seat, not eight. The omitted context is different by declaration:

- pip trump \(t\): natural context \(t\) is empty after calling;
- no-trump: called context 7 is empty;
- doubles-trump: natural context 0 is nonempty as a follow set but is
  **unleadable**, so the rule never queries a follower against a 0 lead.

The last case is a genuine reachability reduction rather than an algebraic
emptiness statement.

#### 7.13.3 The exact turn-schedule void language

This subsection temporarily projects away tile identities, hand contents, and
trick comparison while retaining cyclic actor order, one lead context per
trick, and which hidden followers have already acted. It gives an exact
schedule language and a necessary outer domain for physical reachability.

For a triple of public void masks \(V_i\subseteq\Lambda_\delta\), define

\[
M(q)=\{h_i:q\in V_i\},
\qquad
Q=\{q:M(q)\ne\varnothing\}.
\]

For a nonconstant capacity profile, let \(B\) be the hidden low-capacity set
from §7.13.1. Among the current-prefix realizations of that profile, the largest
set of hidden seats that can already have acted as followers is

| \(B\) | \(F(B)\) |
|---|---|
| \(\varnothing\) | \(\varnothing\) |
| \(\{h_1\}\) | \(\{h_1\}\) |
| \(\{h_2\}\) | \(\varnothing\) |
| \(\{h_3\}\) | \(\varnothing\) |
| \(\{h_1,h_2\}\) | \(\{h_1,h_2\}\) |
| \(\{h_1,h_3\}\) | \(\{h_1\}\) |
| \(\{h_2,h_3\}\) | \(\{h_3\}\) |
| \(\{h_1,h_2,h_3\}\) | \(\{h_2,h_3\}\) |

For an equal profile use \(F=\varnothing\) and the completed-trick boundary.

**[THEOREM — proved mathematically: schedule-language characterization]** Let
\(j\) be the number of completed tricks associated with the profile. A triple
of void masks is realizable in the projected turn schedule if and only if

\[
|Q|\le j,
\]

or

\[
|Q|=j+1
\quad\text{and}\quad
\exists q\in Q:
\varnothing\ne M(q)\subseteq F(B).
\]

**Proof.** Each completed trick has one lead context, so at most \(j\) distinct
contexts can have produced hidden follower observations before the current
trick. The current partial trick can introduce at most one additional context,
and only already-acted hidden followers can acquire that new void.

Conversely, for \(|Q|\le j\), assign each used context to one completed trick
and assign its nonempty membership pattern to the hidden followers that slough
in that projected trick; unused completed tricks repeat any context or produce
no new void. For \(|Q|=j+1\), place a context whose membership is a nonempty
subset of \(F(B)\) in the current trick and place the other \(j\) contexts in
the completed tricks. At this projection level a desired next leader may be
designated as the winner of the last completed trick. ∎

This theorem is exact for the turn-schedule projection only. It deliberately
does not assert that the required tiles exist, that one deal can supply all
responses, or that the designated winners follow from trick order.

Let

\[
A_j
=
\sum_{u=0}^{j}\binom7u7^u.
\]

For a current hidden-follower set of size \(f\in\{1,2\}\), let

\[
T_{j,f}
=
A_j
+
\binom7{j+1}
\left(7^{j+1}-(8-2^f)^{j+1}\right)
\qquad(j\le6),
\]

and let \(T_{j,0}=A_j\). The factor \(7^u\) assigns one of the seven nonempty
hidden-seat membership patterns to each used context. In the current-context
term, \(2^f-1\) membership patterns are nonempty subsets of \(F(B)\).

The exact counts are:

| \(j\) | \(A_j=T_{j,0}\) | \(T_{j,1}\) | \(T_{j,2}\) |
|---:|---:|---:|---:|
| 0 | 1 | 8 | 22 |
| 1 | 50 | 323 | 743 |
| 2 | 1,079 | 5,524 | 10,844 |
| 3 | 13,084 | 51,759 | 88,159 |
| 4 | 97,119 | 286,770 | 428,562 |
| 5 | 450,066 | 947,017 | 1,244,937 |
| 6 | 1,273,609 | 1,817,216 | 2,080,768 |
| 7 | 2,097,152 | — | — |

These numbers are phase-specific language sizes, not counts of reachable
support fibers.

#### 7.13.4 Tile witnesses and exact reachability certification

A schedule-admissible void is not enough. If context \(q\) has ever produced a
public hidden-seat void, some tile in its lead fiber must already have been
played.

**[THEOREM — proved mathematically: lead-witness necessity]** For every legal
Straight prefix and every \(q\in Q\),

\[
L_{\delta,q}\setminus U\ne\varnothing.
\]

**Proof.** Take the first public failure to follow in context \(q\). The trick's
lead tile \(d\) satisfies \(\ell(d,\delta)=q\), so \(d\in L_{\delta,q}\).
That tile is already public and played, while \(U\) contains only current
hidden remainder tiles. Hence \(d\notin U\). ∎

The exact executable definition uses a full legal witness rather than treating
necessary conditions as sufficient.

**[DEFINITION]** A **Straight support-reachability witness** contains:

1. a viewer and Straight declaration;
2. a valid complete deal and contract residue;
3. an actor-attributed legal public play prefix generated by the objective
   transition system; and
4. the claimed exact support normal form at that prefix.

The validator replays the prefix, projects the rule-derived cells, compiles the
exact support normal form, and accepts exactly when it equals the claimed
normal form.

**[THEOREM — proved mathematically: witness completeness and soundness]** A
feasible support normal form belongs to \(\mathscr R_{\mathrm{Str}}^m\) if and
only if it has an accepted Straight support-reachability witness.

**Proof.** Soundness follows because replay begins from a valid complete deal,
uses only legal transitions, and compares the exact projected support. For
completeness, a legal prefix in the definition of
\(\mathscr R_{\mathrm{Str}}^m\), together with its generating deal and
contract, is such a witness. ∎

For internal simulation, the witness is the already existing objective trace;
no separate search or serialized proof is required. For arbitrary external
support objects, this validator is exact but may be expensive. A cheaper test
must identify itself as necessary, sufficient, or approximate.

**[COROLLARY — proved mathematically: finite decidability]** Membership in
\(\mathscr R_{\mathrm{Str}}^m\) is decidable.

**Proof.** The double-six deal space is finite. Under the configured Straight
profile the legal one-round auction, declaration set, and every contracted-hand
play tree are finite. Exhaustively generate their legal prefixes, project each
prefix to its exact support normal form, and compare with the candidate. Because
Straight bid legality has no private-hand eligibility condition, any contracted
prefix witness can be placed in a one-attempt auction; earlier all-pass attempts
are irrelevant to current rule support. The procedure is finite, although far
too large to be proposed as the ordinary validator. ∎

#### 7.13.5 Feasible does not imply reachable

**[CONSTRUCTED COUNTEREXAMPLE / THEOREM — finite verification: a feasible
unreachable support]** Fix hidden capacities

\[
(k_1,k_2,k_3)=(6,6,6)
\]

and the 18-tile hidden pool

\[
U
=
\sigma_0
\cup
\mathcal D^\circ
\cup
\{2\!:\!1,3\!:\!1,3\!:\!2,4\!:\!1,4\!:\!2\}.
\]

Let

\[
P_1=U\setminus\sigma_0,
\qquad
P_2=P_3=U.
\]

This cell system is feasible and already support-reduced. To see this directly,
write \(Z=\sigma_0\) and \(N=U\setminus Z\), so \(|Z|=7\) and \(|N|=11\).
Seat 1 can receive any chosen nonzero tile together with five other members of
\(N\); the remaining twelve tiles can be split six-six between seats 2 and 3.
A chosen nonzero tile can instead be placed at seat 2 or 3 after assigning six
other nonzero tiles to seat 1. A chosen zero-bearing tile can be placed at seat
2 or 3 after assigning any six nonzero tiles to seat 1. Thus every displayed
holder edge occurs in a conserved world and no omitted seat-1/zero edge is
locally allowed.

It is not Straight-reachable.

**Proof.** With equal hidden capacities six, at most one lead context can have
created a hidden void: the state is after one completed trick, after a viewer
lead of the next trick, or after three hidden plays of the first trick. No-void
cells have every local holder edge and reduce to a different support.

Exhaust the nine Straight declarations, their seven leadable contexts, and the
seven nonempty hidden-seat void-membership patterns. For each of these 441
one-context generators, and for the nine no-void generators, form the raw cells
and compile their exact marginal support. Exactly two of the 450 reduced
supports equal the target:

- zeroes-trump, called context 7, with only seat 1 void;
- no-trump, natural context 0, with only seat 1 void.

Both have effective follow set \(\sigma_0\), but their lead fibers are not the
same:

\[
L_{0,7}=\sigma_0,
\qquad
L_{\mathsf{notrump},0}=\{0\!:\!0\}.
\]

Each lead fiber is contained in \(U\). Hence neither generator has a tile
outside the hidden pool that could already have led the required context. The
lead-witness theorem is violated in both cases, and no other static generator
decodes to the target. Therefore no legal prefix can produce it. ∎

Thus

\[
\mathscr R_{\mathrm{Str}}^m
\subsetneq
\mathcal N(\mathscr C_+).
\]

Reachability is not Hall feasibility, not schedule admissibility, and not a
property of capacities alone.

#### 7.13.6 A proved 46-bit reachable-support ceiling

Although the exact cardinality of \(\mathscr R_{\mathrm{Str}}^m\) is not yet
closed, the preceding necessary structure gives a much smaller rigorous
standalone ceiling than the 81-bit full-schema census.

For \(n\in\{0,\ldots,21\}\) and \(u\in\{0,\ldots,7\}\), define

\[
B_{n,u}
=
\sum_{\substack{Q\subseteq\Lambda_\delta\\|Q|=u}}
[x^n]
\left(
\prod_{q\in Q}
\left((1+x)^{|L_{\delta,q}|}-x^{|L_{\delta,q}|}\right)
\right)
\left(
\prod_{q\notin Q}(1+x)^{|L_{\delta,q}|}
\right).
\]

The subtraction forbids choosing all lead-capable tiles of a used context into
\(U\), thereby enforcing the necessary lead-witness condition. Because the
lead-fiber cardinalities are always the multiset \(1,\ldots,7\), \(B_{n,u}\)
is declaration-independent.

For a reachable capacity profile \(k\), put \(n=\sum_sk_s\). Let \(j\) and
\(f=|F(B)|\) be supplied by §§7.13.1–7.13.3. Define the number of
**declaration-tagged necessary outer profiles** for that profile by

\[
C(k)
=
\sum_{u=0}^{j}7^u B_{n,u}
+
\mathbf1[f>0]
\left(7^{j+1}-(8-2^f)^{j+1}\right)B_{n,j+1},
\]

with the terminal profile assigned one canonical empty-void profile per
declaration. These profiles enforce:

- a reachable hidden-capacity shape;
- the exact turn-schedule void language;
- the exact hidden-pool cardinality; and
- at least one non-hidden lead witness for every used void context.

They intentionally do not claim complete tile/deal reachability and may decode
to infeasible or unreachable support. Every actually reachable support has at
least one such profile.

**[THEOREM — exhaustive finite verification: reachable-support upper bound]**
The exact finite sums are

\[
\sum_{k:\,\max k-\min k\le1} C(k)
=
7{,}124{,}838{,}074{,}989
<2^{43}
\]

for one supplied declaration, and

\[
9\sum_{k:\,\max k-\min k\le1} C(k)
=
64{,}123{,}542{,}674{,}901
<2^{46}
\]

over all nine declaration tags. The largest fixed-profile block is

\[
\max_k C(k)=839{,}220{,}930{,}919<2^{40}.
\]

Therefore

\[
|\mathscr R_{\mathrm{Str}}^m|<2^{46},
\]

and 46 bits are sufficient for some fixed-width standalone code of exact
reachable support. One constructive, though not necessarily fast, code chooses
the lexicographically least outer profile that decodes to the target
reachable support and ranks it within the displayed finite outer-profile set.

This is an upper bound, not an assertion that 46 bits are necessary or that the
outer-profile layout is transition-optimal. The same verified outer-profile count
gives useful context-relative ceilings:

- with the declaration supplied externally, fewer than \(2^{43}\) outer profiles;
- with the capacity profile supplied externally, fewer than \(2^{43}\);
- with both declaration and capacity profile supplied, fewer than \(2^{40}\).

These remain necessary-outer-profile bounds, not exact minima. With the complete
certified mechanical state supplied, the supplemental cost is zero bits rather
than 40.

A nontrivial lower bound follows from universally reachable no-void families.
Every pool of the stated size is reachable for each of these capacity shapes:

\[
(7,7,7),
\]

all three permutations of \((6,7,7)\), all three permutations of
\((6,6,7)\), and

\[
(6,6,6).
\]

For \((6,7,7)\), choose any one complement tile as a hidden lead. For
\((6,6,7)\), the nine-tile complement contains either two doubles or two tiles
sharing a pip; declare doubles or that pip and use the pair as the two hidden
plays of a legal prefix. If the two low seats are separated by the viewer in
clockwise order, choose the viewer's intervening play from the other seven
complement tiles: play a follower when one exists and otherwise any slough, and
retain the other six as the viewer's current hand. For \((6,6,6)\), the
ten-tile complement contains either three doubles or three distinct tiles
sharing a pip. If it contains at most two doubles, counting each tile-pip
incidence once gives at least 18 incidences among seven pips, so some pip occurs
on at least three distinct tiles. Use those three tiles as a legal hidden
three-play prefix and give the other seven complement tiles to the viewer.

These families are disjoint and contain

\[
\binom{28}{21}
+3\binom{28}{20}
+3\binom{28}{19}
+\binom{28}{18}
=
44{,}352{,}165
>2^{25}
\]

reachable supports. Hence any universal fixed-width standalone code needs at
least 26 bits.

The current proved interval is therefore

\[
26
\le
\left\lceil\log_2|\mathscr R_{\mathrm{Str}}^m|\right\rceil
\le
46.
\]

This interval is deliberately not collapsed by guesswork. The exact cardinality
of \(\mathscr R_{\mathrm{Str}}^m\), and therefore the optimal standalone
fixed-width bit count inside this interval, remains unresolved. Relative to a
certified mechanical coordinate from which support is derived, the supplemental
support and reachability cost remains zero bits by §7.10; the 26–46-bit
statement concerns a standalone support identifier with no containing game
state.

### 7.14 Typed support transitions and exact refinement

Let \(c\) be a pre-action mechanical state.

**Hidden actor.** For hidden seat \(s\ne m\) publicly playing \(d\), let

\[
D_{s,d}
=
\{\omega\in\Phi(c):d\in H_s(\omega)\text{ and the observed play is legal}\}.
\]

Define the typed removal map

\[
\vartheta_{s,d}:D_{s,d}\to\Phi(c')
\]

by removing \(d\) from \(H_s\) and leaving the other hidden hands unchanged.
Then

\[
\Phi(c')=\vartheta_{s,d}(D_{s,d}).
\]

**Viewer actor.** If \(s=m\), define

\[
\vartheta_{m,d}=\operatorname{id}_{\Phi(c)}.
\]

The viewer's known hand and public physical state change, but the three hidden
remainder hands do not. Therefore the successor hidden fiber is the identity
image, even though the containing mechanical state is different.

**[COROLLARY — proved mathematically: typed play bijection]** In the cell-
theorem scope, the hidden-actor removal map is a bijection from
\(D_{s,d}\) onto \(\Phi(c')\), with inverse given by adding the fixed domino
\(d\) back to seat \(s\). The viewer-action identity is likewise a bijection
between the predecessor and successor hidden-remainder fibers.

**Proof.** Surjectivity is the exact support-update equality above. Removing a
fixed domino from a fixed seat is injective, and adding it back is the inverse
constructed in the losslessness proof. The viewer map is literally the
identity on hidden remainders. ∎

The shorthand \(\Phi(c')\subseteq\Phi(c)\) is generally type-incorrect for a
hidden actor because predecessor and successor worlds use different pools and
capacities. Exact refinement has two correctly typed forms.

**[THEOREM — proved mathematically: fixed-deal support refinement]** Within one
fixed deal attempt, appending a legal public action gives

\[
\Omega_{r,t+1}^m\subseteq\Omega_{r,t}^m
\]

as literal sets of complete initial deals.

**Proof.** Compatibility with the longer public prefix includes every
constraint of the shorter prefix plus the newly observed action. ∎

**[COROLLARY — proved mathematically: typed fiber cardinality refinement]** For
a hidden action,

\[
|\Phi(c')|=|D_{s,d}|\le|\Phi(c)|.
\]

For a viewer action,

\[
|\Phi(c')|=|\Phi(c)|.
\]

Thus play never increases rule-support cardinality within one attempt, even
though hidden-action fibers are not literally subsets of their predecessors.
At all-pass, a new deal attempt creates a new domain and this statement does
not compare the two attempts.

**[FINITE VERIFICATION RECEIPT — stated corpus]** Across the 972 reachable
support-parity prefixes of §7.5, the verifier checks all 864 consecutive
prefix transitions from plays 20 through 28. The 648 hidden-player actions
never increase fiber cardinality, and the 216 viewer actions preserve it
exactly.

**[THEOREM — exhaustive finite verification]** Independently of the reachable
42 corpus, the verifier exhausts every abstract three-seat cell system on
universes of sizes one through three. For every actor with positive capacity,
every tile in that actor's possible set, and every abstract follow set, it
compares the exact image of legal predecessor worlds with the fiber produced
by the typed update. All 14,412 lead cases, 56,460 successful-follow cases,
and 56,460 failure-to-follow cases agree exactly.

This finite receipt checks the algebra of the update on a complete tiny domain;
the general theorem remains the induction proof of §7.5.

### 7.15 What the minimality theorem does and does not claim

The total exact support normal form is globally minimal in the semantic
state-count order of §7.10: every exact deterministic representation of
Straight capacity-cell support factors onto it. All infeasible systems collapse
to the one extensional empty-support state; every feasible branch contains the
unique minimized payload for its nonempty fiber. The literal capacity-cell tuple
is therefore **not** globally minimal. Its proper roles are:

1. a simple rule-derived presentation of support;
2. a local update form driven directly by public voids;
3. an input from which the canonical support normal form is compiled.

The feasible normal form extracts exact hidden-location marks and leaves a
tagged ambiguity component. Inside the native direct holder/quota language,
certain tiles, zero-residual seats, the explicit binary active pair, one
conserved quota, binary per-tile holder relations, unsupported positive edges,
and every non-exclusion ternary edge have been removed. Every remaining ternary exclusion is essential. One feasible
assignment plus an SCC pass computes the whole marginal relation exactly.

No representation can be called absolutely smallest in bytes or fastest on all
hardware without fixing a cost model and required operations. A two-bit dense
ternary category code is optimal within fixed-width per-tile category codes; a
sparse exclusion map may use less memory; a fiber-local rank is fixed-width
bit-optimal for one world; complement-elided masks are often cheaper to mutate;
and rule-derived void cells are often cheaper to update. These are
proved-equivalent compiled views with different operational costs, not rival
definitions of the game.

The minimality theorem is only for the **support component**. It does not prove
that the complete mechanical coordinate, retained record, augmented latent
state, policy model, or utility residue is minimal.

---

## 8. Belief and filtering

### 8.1 Policy models, inherited latent state, and history likelihood

A behavioral action kernel for actor \(j\) has the form

\[
\pi_j(a\mid I,z)
=
\mathbf1[a\in A(I)]\,\widetilde\sigma_j(a\mid I,z),
\]

where \(\widetilde\sigma_j\) is normalized over the legal set and \(z\) is any
field state required by the model.

Fix current deal attempt \(r\). Let \(\zeta_r\) contain every unobserved
persistent variable inherited at the start of the attempt that later behavior
may use: for example opponent policy type, remembered private hands from
earlier attempts, a correlation device, or unresolved hidden prior-attempt
state. Define the augmented current-attempt root world

\[
\xi_r=(\omega_r,\zeta_r),
\]

where \(\omega_r\) is the complete current deal. The viewer's prior
\(p_{r,0}^m\) on \(\xi_r\) is conditioned on the viewer's complete
match-global information at the start of the attempt, including the newly
observed private hand. Earlier evidence is carried into this prior rather than
silently discarded.

Let

\[
L_{\Pi,r}(h_{r,t}\mid\xi_r)
\]

be the exact conditional probability of the current attempt's public action
prefix under the field, given the inherited public record already absorbed
into \(p_{r,0}^m\) and \(\zeta_r\). There are two exact and distinct ways to
represent this law.

**Kernel representation.** Retain stochastic action and field-state transition
kernels. The chain rule always factors history probability into exact
conditional probabilities of each observed action given the preceding public
prefix and root world. A local product

\[
L_{\Pi,r}(h_{r,t}\mid\xi_r)
=
\prod_{u<t}
\widetilde\sigma_{j(u)}
\left(a_u\mid I_{r,u}^{j(u)},z_{r,u}\right)
\]

is valid only when the retained current state and its transition law are
sufficient so that these are the field's exact conditional probabilities. If
field-state evolution is stochastic, the product is integrated or summed over
the latent state path. Correlated action laws may equivalently expose the
joint history likelihood directly.

**Seed-augmented representation.** Suppose the selected stochastic field admits
a measurable randomization realization: there is a random tape \(R\), with a
specified law, and measurable deterministic update functions whose marginal
law equals the kernel field. This holds automatically for the finite action and
state models used by the exact verifier, and more generally for the usual
standard-Borel kernels once a realization theorem is invoked. Enlarge the root
latent variable by that complete tape. Conditional on
\((\omega_r,\zeta_r,R)\), field-state transitions and actions are
deterministic. The conditional history likelihood is then the indicator that
the generated public prefix equals the observed one. Marginalizing \(R\)
recovers the stochastic-kernel likelihood by the assumed realization property.

The two representations are equivalent only under that realization assumption;
the kernel representation is primary when no such representation has been
specified. Their conditionals must never be mixed. Once a full random tape is
conditioned on, the action probability is zero or one; multiplying again by
the unconditioned stochastic action probability would double-count
randomization.

The current-attempt representation is match-global only when inherited latent
state is broad enough to retain every hidden persistent variable relevant to
later behavior. If the field resets between attempts, \(\zeta_r\) can be
correspondingly smaller.

### 8.2 Posterior on the fixed current-attempt domain

Assume the observed within-attempt history has positive model probability.
The posterior measure on augmented current-attempt worlds is

\[
\nu_{r,t}^m(d\xi)
=
\frac{
 \mathbf1[\omega_r\in
 \Omega_{r,t}^m(I_{r,t}^{m,\mathrm{deal}})]
 L_{\Pi,r}(h_{r,t}\mid\xi)
 p_{r,0}^m(d\xi)
}{Z_{r,t}^m}.
\]

In a finite or countable latent model, the corresponding probability mass is

\[
\nu_{r,t}^m(\xi)
=
\frac{
 p_{r,0}^m(\xi)
 \mathbf1[\omega_r\in
 \Omega_{r,t}^m(I_{r,t}^{m,\mathrm{deal}})]
 L_{\Pi,r}(h_{r,t}\mid\xi)
}{Z_{r,t}^m}.
\]

**[THEOREM — proved mathematically: Bayes factorization]** These formulas are
Bayes' rule on a fixed current-attempt latent domain. Public play does not
remove tiles from the initial deal; it changes which deals remain compatible
and how their latent
worlds are weighted.

**Proof.** Multiply the conditioned prior by the rule-compatibility indicator
and the conditional likelihood of the observed public prefix, then divide by
the positive total mass. This is precisely Bayes' rule. The domain variable
\(\omega_r\) remains the fixed initial deal throughout the attempt. ∎

A new deal attempt is a new chance event and therefore creates a new current-
deal domain. It is not merely another reweighting of the abandoned deal.

### 8.3 Pushforward to the current fiber

Let \(\operatorname{pr}_{\omega}(\xi_r)=\omega_r\). The current physical
belief is the pushforward marginal

\[
\mu_{r,t}^m
=
(\rho_{r,t}^m\circ\operatorname{pr}_{\omega})_\#\nu_{r,t}^m.
\]

For a discrete latent model and \(\omega_t\in\Phi(c_{r,t}^m)\),

\[
\mu_{r,t}^m(\omega_t)
=
\sum_{\xi:\rho_{r,t}^m(\omega_r)=\omega_t}
\nu_{r,t}^m(\xi).
\]

If field state matters to continuation, let
\(\widetilde\nu_{r,t}^m(d\xi,dz)\) be the exact joint posterior of the root
world and current field state after the observed prefix. In a valid
seed-augmented realization, \(z=z_{r,t}(\xi,h)\) is deterministic and this
joint measure is an ordinary graph pushforward of \(\nu\). In the kernel
representation it is obtained by the exact conditional state kernel and need
not be a graph measure.

The augmented current belief is

\[
\beta_{r,t}^m
=
\left(
(\xi,z)\longmapsto
(\rho_{r,t}^m(\omega_r),z)
\right)_\#
\widetilde\nu_{r,t}^m.
\]

The coupled augmented belief \(\beta\) is the general exact object.

**[PROPOSITION — proved for finite/countable domains: conditional-kernel
factorization]** Let \(\beta(\omega,z)\) be a normalized joint mass function.
Define

\[
\mu(\omega)=\sum_z\beta(\omega,z),
\qquad
K(z\mid\omega)=
\frac{\beta(\omega,z)}{\mu(\omega)}
\quad\text{when }\mu(\omega)>0,
\]

and choose any normalized \(K(\cdot\mid\omega)\) on zero-mass worlds. Then

\[
\beta(\omega,z)=\mu(\omega)K(z\mid\omega).
\]

**Proof.** On positive-mass worlds this is substitution; on zero-mass worlds
both sides are zero. Summing \(K(\cdot\mid\omega)\) gives one wherever the
conditional matters. ∎

Thus \((\mu,K)\) is an exact alternative representation of the coupled law,
not an independence assumption. If
\(K(\cdot\mid\omega)=\delta_{f(\omega)}\), the joint law is supported on a
graph and \((\mu,f)\) is exact. One constant field-state value is the special
case \(f(\omega)=z_0\). Separate marginals \((\mu,\lambda_z)\) are exact only
when \(K(\cdot\mid\omega)=\lambda_z\) almost surely and the continuation and
filter operations preserve every needed consequence of that factorization.
For general measurable spaces, writing
\(\beta(d\omega,dz)=\mu(d\omega)K(dz\mid\omega)\) requires an appropriate
regular conditional or disintegration assumption. Otherwise retain
\(\beta\) directly.

### 8.4 Physics-only belief and uniformity

Define the physics-only current-deal posterior by omitting the current
attempt's discretionary action likelihood:

\[
\nu_{r,t,\mathrm{phys}}^m(d\xi)
\propto
\mathbf1[\omega_r\in
\Omega_{r,t}^m(I_{r,t}^{m,\mathrm{deal}})]
 p_{r,0}^m(d\xi).
\]

**[THEOREM — proved mathematically]** Suppose the current-deal marginal of
\(p_{r,0}^m\), before conditioning on the within-attempt public prefix, is
uniform over ordered deals consistent with the viewer's observed initial hand.
Fix an actor-attributed legal Straight 42 prefix in the cell-theorem scope.
Then the physical pushforward of the normalized rule-only restriction
\(\nu_{r,t,\mathrm{phys}}^m\) to \(\Phi(c_{r,t}^m)\) is uniform.

**Proof.** The assumed marginal gives equal mass to every initial deal
consistent with the viewer's hand. Restriction by the fixed legal public
prefix keeps exactly the compatible deals and does not alter their relative
rule-only masses. Section 7.5 gives a bijection between those deals and the
current remainder assignments. Marginalizing inherited latent variables has
already been accounted for in the equal current-deal marginal. Therefore each
fiber member receives the same pushforward mass. ∎

The baseline independent uniform deal law, including its conditional
independence from inherited non-deal latent state, supplies the stated
current-deal marginal. The theorem is history-relative: a coarse mechanical
coordinate that merges histories does not by itself specify the inverse deal
map or the likelihood model.

**[COROLLARY — proved mathematically]** Under the same assumptions, the
physics-only probability that unseen domino \(d\) is currently held by hidden
seat \(s\) is

\[
\mu_{r,t,\mathrm{phys}}^m(d\in H_s)
=
\frac{N(\mathbf C_{r,t}^{m,d\to s})}
     {N(\mathbf C_{r,t}^m)}.
\]

The exact count-ratio sampler of §7.8 therefore samples this physics-only
belief exactly. The statement does not survive arbitrary action-likelihood
tilting without replacing raw fiber counts by the corresponding posterior
mass sums.

### 8.5 Exponential tilt

On a finite compatible augmented current-attempt domain, let

\[
g_{r,t}^m(\xi)=\log L_{\Pi,r}(h_{r,t}\mid\xi),
\qquad \log0=-\infty.
\]

If \(u_{r,t}^m\) is the normalized prior restricted only by rule compatibility
on the same augmented domain, then

\[
\nu_{r,t}^m(\xi)
=
\frac{u_{r,t}^m(\xi)e^{g_{r,t}^m(\xi)}}
{\sum_{\xi'}u_{r,t}^m(\xi')e^{g_{r,t}^m(\xi')}}.
\]

**[THEOREM — proved mathematically]** This is algebraically equivalent to the
Bayes factorization whenever the denominator is positive. For a general
measurable latent space, the same statement is a Radon--Nikodym tilt with the
sum replaced by an integral.

**Proof.** Substitute \(e^{g(\xi)}=L(h\mid\xi)\) into the normalized restricted
prior and cancel the common rule-support normalizer. The remaining denominator
is the Bayes evidence. ∎

### 8.6 One-step filtering and deal-attempt transitions

Within one deal attempt, the root-world posterior updates after public action
\(a_t\) by

\[
\nu_{r,t+1}^m(d\xi)
\propto
\mathbf1[a_t\text{ is legal in }\xi]
\Pr_{\Pi}(a_t\mid h_{r,t},\xi)
\nu_{r,t}^m(d\xi),
\]

where the conditional action probability integrates any current field-state
uncertainty not already included in \(\xi\). The initial deal component of
\(\xi\) does not transition. Append the public action to form
\(h_{r,t+1}\), then push through \(\rho_{r,t+1}^m\) for the physical marginal.

For direct current-state filtering, begin with the augmented current belief
\(\beta_{r,t}\). The exact field supplies a Markov kernel over observed action,
successor field state, and any public continuation record. Conditioning that
kernel on the observed action and normalizing gives a reweighted predecessor
measure \(\bar\beta_{r,t}\), then

\[
\beta_{r,t+1}
=
\bar\beta_{r,t}\widehat K_{a_t},
\]

where \(\widehat K_{a_t}\) combines the field-state transition with the typed
physical world map. For a hidden actor, the physical component is typed
removal; for a viewer action it is the identity on hidden remainders. All
predecessor masses sharing a successor are summed.

In a valid seed-augmented realization, \(\widehat K_{a_t}\) is a deterministic
map and the last display reduces to an ordinary pushforward. In the kernel
representation it is a genuine kernel image. The two notations are not
interchanged silently.

**[BOUNDARY]** The physical marginal \(\mu_{r,t}\) alone does not generally
support this exact update: worlds with the same current remainder can carry
different field state or different history likelihood. A direct remainder
filter is exact only when the retained augmented current state is sufficient
for action likelihood and latent transition.

If an action ends the attempt by all-pass, first apply its likelihood update to
the abandoned attempt. Then apply the persistent-state transition and the
selected new-deal chance kernel to construct \(p_{r+1,0}^m\). The new deal is
not obtained by applying a remainder-removal map to the old one.

**[THEOREM — proved mathematically]** A public action can have three distinct
effects:

1. a physical state transition;
2. a rule-support restriction or retyping;
3. likelihood reweighting and normalization.

No one effect should be silently substituted for another. A deal-ending
action can additionally be followed by a new-deal chance-kernel extension;
that chance transition is not an action-likelihood update.

**Proof.** The objective transition acts on physical state, the legality
indicator changes the compatible domain or its remainder type, and the policy
likelihood changes relative mass before normalization. The definitions permit
each component to be nontrivial while another is trivial, so they are distinct
operations. ∎

### 8.7 Forced and own actions

**[THEOREM — proved mathematically]** At an actor information state whose legal
set is a singleton, every normalized behavioral policy assigns probability one
to that sole action. Conditional on that same actor information state, the
action contributes no discretionary likelihood ratio.

**Proof.** A probability distribution normalized on a one-element set assigns
mass one to its only element. ∎

An outside observer may be uncertain whether the actor's legal set was a
singleton across candidate worlds; legality can therefore still remove worlds.

**[PROPOSITION — own-action cancellation]** For a viewer's own action, if the
viewer's policy randomization depends only on information already known to the
viewer and on private randomness independent of hidden deal uncertainty, the
likelihood factor is constant across the viewer's compatible deal worlds and
cancels from the viewer's posterior. This need not hold if the model includes
uncertain correlated policy state.

### 8.8 Evidence and off-path boundaries

**[THEOREM — proved mathematically]** Rule legality fixes impossible actions and
normalized policies fix forced actions at probability one. Beyond those
constraints, a discretionary bid or play has no policy-independent quantitative
likelihood meaning: different valid policy models can assign opposite likelihood
ratios to the same public action.

**Proof.** Choose two compatible latent worlds in which the displayed action is
legal and each induced actor information state has at least one other legal
action—for example, an opening auction action between `pass` and \(P(30)\). One
valid policy model may assign likelihoods \((3/4,1/4)\) to the displayed action
in those worlds, while another assigns \((1/4,3/4)\), placing the remaining
mass on another legal action. Bayes factors reverse while rule support is
unchanged. Forced-action likelihood one is the boundary already proved in
§8.7. ∎

**[BOUNDARY]** Bayes' rule uniquely determines beliefs only at histories with
positive probability under the chosen prior and policy model. Zero-probability
histories require an explicitly supplied off-path belief rule or assessment.

---

## 9. The native marked hand

### 9.1 Why an owned induced graph is insufficient

Let \(H_m^t\subseteq\mathcal D\) be the viewer's remaining hand. Its induced
relations contain only tuples whose nodes all lie in \(H_m^t\).

**[CONSTRUCTED COUNTEREXAMPLE]** Fix no-trump and an owned hand containing
`6-4`. The owned induced structure is unchanged between two ambient states in
which:

- `6-5` and `6-6` are already completed; or
- one or both remain in the hidden live complement.

When `6-4` leads, those external nodes are precisely higher sixes that can beat
it. The induced owned structure cannot say whether they are live, who may hold
them, or which public voids exclude holders. Thus the owned subgraph alone does
not determine the action's boundary relations.

### 9.2 Ambient marking

For viewer \(m\), let

\[
\Lambda_t^m
=
\left(
H_m^t,
C_t,
(B_s(h_t))_{s\in S},
(V_s(h_t))_{s\in S},
(K_s)_{s\ne m},
\mathcal K_{\mathrm{amb},t}^m
\right),
\]

where owned and public regions are exact markings, \((K_s)\) are exact
support-implied hidden-location markings, and
\(\mathcal K_{\mathrm{amb},t}^m\) is the tagged ambiguity component of §7.10. The
rule-derived pool and cells remain available as deterministic views of the
public physical residue; they are not repeated in \(\Lambda_t^m\).

Define the native marked-hand object

\[
\mathfrak H_t^m
=
\left(
\mathcal A_\delta,
\Lambda_t^m,
\iota_t:H_m^t\hookrightarrow\mathcal D,
K,
\eta_t
\right),
\]

where \(\eta_t\) retains current trick, control, score, phase, and the residue
required by a named utility.

This object is the ambient 28-node relational algebra expanded by predicates
such as `Owned`, exact public location, `PubliclyVoid`, `CertainHiddenHolder`,
`LocallyAllowedHolder`, `MarginallyPossibleHolder`, and current control. These
predicates must not be conflated. The certain-hidden marks plus the tagged ambiguity
component are the globally minimal semantic support quotient; local cells are a
derived rule-facing presentation. No minimality claim is made for the complete
marked-hand object beyond that support component.

### 9.3 Boundary profiles

For relation \(R\) in \(\mathcal A_\delta\), the boundary of \(H\) consists of
relation tuples containing at least one node in \(H\) and one outside \(H\).
Let \(O_t\subseteq\mathcal D\setminus H_m^t\) denote a named live external
set, such as all uncompleted nonowned nodes in one objective world. A
comparison boundary includes

\[
\left(
\operatorname{BEATS}_\delta(q,d)\cap O_t
\right)_{d\in H_m^t,\ q\in Q}
\]

and explicitly typed local-allowance or marginal-holder constraints for each
external node.

**[THEOREM — proved mathematically]** The ambient marked expansion determines
the owned induced structure and every boundary query defined from the included
relations and markings. The owned induced structure does not determine the
ambient marked expansion.

**Proof.** Restricting the ambient relations and marks to owned nodes gives the
induced structure, and any named boundary query is evaluated directly from the
retained ambient relations and marks. The counterexample in §9.1 gives two
ambient expansions with the same owned restriction and different boundary
answers, proving failure of the converse. ∎

### 9.4 Auction hand as a declaration bundle

Before declaration, the same physical hand is embedded in

\[
\boldsymbol{\mathcal A}
=(\mathcal A_\delta)_{\delta\in\Delta_{\mathrm{straight}}}.
\]

A precise auction-hand object is therefore a marked physical hand together
with the whole declaration-indexed bundle and current auction/match residue.
Declaration selects one layer for play.

### 9.5 Node-expenditure operator

At a viewer decision, a legal action is one physical node
\(d\in H_m^t\cap A(c_t^m)\). Let \(e_t^m\) denote the retained viewer-known
continuation record beyond the mechanical state: public or private observation
residue still consulted by the selected strategy, continuation model, or utility.
Its induced update may be written

\[
\mathcal T_d:
(c_t^m,e_t^m,\beta_t^m)
\longmapsto
(c_{t+1}^m,e_{t+1}^m,\beta_{t+1}^m),
\]

where \(\beta\) includes any latent field state needed for exact continuation.
The operator contains distinct physical, retained-record, support, and belief
components. The action's meaning is the whole transition, not deletion of a
number from a list.

### 9.6 Local slot order is a gauge

Let an encoding \(\chi:\{1,\ldots,n\}\to H_m^t\) assign local slots to
stable physical domino identities. For a slot permutation \(\rho\), define
the re-encoding \(\chi'=\chi\circ\rho^{-1}\). The physical marked hand is
unchanged.

**[THEOREM — proved mathematically]** Any exact target expressed in physical
node identities is invariant under re-encoding. In slot coordinates,

\[
V(\chi')=V(\chi),
\qquad
Q_{\chi'}(i)=Q_\chi(\rho^{-1}(i)).
\]

The theorem transports the encoding and output indices together. It does not
assert that an arbitrary slot-sensitive implementation automatically obeys the
gauge.

**Proof.** The two encodings name the same physical marked object. New slot
\(i\) names the node previously named by slot \(\rho^{-1}(i)\), so scalar
physical value is unchanged and node-indexed outputs are relabeled by the
displayed permutation. ∎

### 9.7 No universal context-free domino value

The exact witness in §10.4 contains the same physical action `4-1`, under the
same no-trump declaration and same mechanical endpoint, with pointwise root
values

\[
Q_{\varpi_0}(4\!:\!1)=-22,
\qquad
Q_{\varpi_1}(4\!:\!1)=22.
\]

**[CONSTRUCTED COUNTEREXAMPLE]** No scalar \(v(d)\) depending only on physical
domino identity can equal exact action value in every world, information state,
belief, policy field, and utility.

### 9.8 Additive decompositions are not canonical

Suppose an attribution is written

\[
Q(\mathfrak H,d)
=q_1(d)+q_2(d,\mathfrak H\setminus\{d\})+q_{\ge3}.
\]

For any function \(f(d)\), replacing

\[
q_1'(d)=q_1(d)+f(d),
\qquad
q_2'(d,\cdot)=q_2(d,\cdot)-f(d)
\]

leaves \(Q\) unchanged.

**[THEOREM — proved mathematically]** The displayed additive split is
nonidentifiable absent additional identifying constraints. An intervention
domain, baseline, weighting measure, and normalization convention are examples
of conventions that can make a particular attribution problem well posed; the
game rules do not select them automatically.

**Proof.** The displayed transformation changes \(q_1\) and \(q_2\) whenever
\(f\ne0\) while leaving their sum, and hence \(Q\), unchanged. Therefore the
observed total does not identify the components without an additional
constraint that rules out this gauge freedom. ∎

---

## 10. Strategic state and the exact history witness

### 10.1 Exact strategic state

Let \(c\) be a mechanical/support state. Let \(e\) be the **required retained
continuation record**: the viewer-known observation residue, beyond \(c\), that
the selected decision strategy, continuation model, or utility can still
consult. Its public component may retain common public-history residue; its
private component may retain earlier hands or other viewer-private observations
omitted by \(c\). Hidden actors' private records and uncertain latent field
state are not placed in \(e\) merely because they affect continuation; they
remain in the augmented latent state and belief. Depending on the problem,
\(e\) may be empty, a proved sufficient summary, or the relevant full slice of
the viewer's perfect-recall information record. This definition makes no
minimality claim.

Let \(\mathcal Z\) be the domain of latent continuation state required by the
field, and define the **ambient admissible augmented domain**

\[
\Xi(c,e)\subseteq\Phi(c)\times\mathcal Z
\]

as the pairs satisfying every hard reconstruction and model-compatibility
constraint under \((c,e)\). The subset records impossibility constraints; it
does not by itself encode probabilistic correlation. Even on a finite domain,
a correlated joint law can assign positive mass to every point of the full
Cartesian product, while a product law can assign zero mass outside a proper
product subset because one of its marginals has zeros.

Let \(\beta\) be the viewer's normalized probability measure on the measurable
domain \(\Xi(c,e)\). It may be concentrated on a proper measurable subset
because of chance-law zeros, action-likelihood zeros, or earlier conditioning.
On a finite or countable domain, its canonical positive-mass support is

\[
\operatorname{supp}_+(\beta)
=
\{x\in\Xi(c,e):\beta(\{x\})>0\}.
\]

For a general measurable latent space, a topological support is not determined
until a topology has also been declared; the strategic theorem needs the
measure itself, not a separately postulated support set. Two decision states
can therefore share the same \((c,e)\) and ambient domain while carrying
different measures, different null sets, or—in the discrete case—different
positive-mass supports.

Fix the rules profile, continuation field/model, utility, and allowed
decision-strategy class. Assume:

1. \((c,e,\omega,z)\) reconstructs every objective, actor-information,
   field-state, and retained-record component needed by the continuation model;
2. physical transitions are Markov in the reconstructed state and action;
3. the continuation field and its latent-state transition are Markov in that
   reconstructed state;
4. each new public or viewer-private observation updates \((e,\beta)\) by the
   exact record transition and Bayesian filter;
5. every utility-relevant past residue is retained in \((c,e,z)\);
6. the contracted continuation has finite horizon;
7. every required map and kernel is measurable, and cumulative reward plus
   terminal utility is integrable under every reachable induced belief
   (bounded utility is sufficient).

**[THEOREM — proved mathematically: strategic sufficiency]** For every fixed
admissible continuation strategy, expected continuation and action values are
functions of

\[
B=(c,e,\beta).
\]

Consequently, any well-defined fixed-field best-response value over the named
allowed strategy class is a function of \(B\). When the maximum is attained,
the best-response correspondence is determined by \(B\) as well.

**Proof.** Use backward induction on the remaining-play grade. At a terminal
node, the tuple \((c,e,\omega,z)\) determines the named terminal utility by the
assumptions, and measurability and integrability make its expectation under
\(\beta\) well defined. At a predecessor, the assumptions determine the
legal-action correspondence, immediate reward, distribution of field actions
and viewer observations, successor retained record, posterior successor
belief, and successor physical state. Integrating the already-defined
successor values over \(\beta\) therefore defines every fixed-strategy
predecessor value from \(B\) alone. Hence \(B\) determines the expected-
utility functional on the fixed allowed strategy class; its supremum and, when
nonempty, its argmax set are therefore determined by \(B\). ∎

When the selected strategy, field, and utility are blind to every viewer-known
observation omitted by \(c\), or when that observation record is fixed once and
for all in the conditioned subproblem, \(e\) is trivial and \((c,\beta)\) is
exact shorthand. If field state is almost surely a known deterministic function
\(z=f(\omega)\), the graph representation \((c,e,\mu,f)\) is exact; the
constant-state notation \((c,e,\mu,z_0)\) is a special case. A conditional
kernel representation \((c,e,\mu,K)\) is also exact under the conditions of
§8.3. Bare separate marginals require a proved factorization and preservation
under filtering and transition. The theorem is a sufficiency statement, not a
minimality theorem.

### 10.2 Bellman form without double counting

Assume utility has an additive decomposition

\[
U=\sum_t r_t+u_{\mathrm{terminal}}.
\]

Choose one recursion-boundary convention. Let \(o\) index the possible
viewer-observation segments from immediately after legal viewer action \(a\) to
the next recursion boundary. A segment contains the intervening public
events and any newly observed viewer-private event. Let
\(P(o\mid c,e,\omega,z,a)\) be the corresponding conditional kernel. Define

\[
\overline R(c,e,\omega,z,a,o)
=
\mathbb E[\text{cumulative additive reward on the segment}
\mid c,e,\omega,z,a,o].
\]

When segment reward is determined by the current augmented state and observed
segment—as it is for ordinary Straight 42 trick scoring—this conditional
expectation is that deterministic reward. Then

\[
Q(B,a)
=
\int_{\Xi(c,e)}
\sum_o
P(o\mid c,e,\omega,z,a)
\left[
\overline R(c,e,\omega,z,a,o)+V(B_{a,o})
\right]
\,\beta(d\omega,dz).
\]

For a continuous observation space, replace the sum by the corresponding
integral. The measurability and integrability conditions of §10.1 apply. The
successor state \(B_{a,o}=(c_{a,o},e_{a,o},\beta_{a,o})\) is
required only for observations with positive predictive probability; it may be
assigned arbitrarily on a zero-probability term because that term contributes
zero. If the next boundary follows the root action immediately,
\(\overline R\) is just that action's immediate reward. If \(o\) bundles later
field actions, \(\overline R\) must account for every additive reward in the
bundled segment. The observation kernel, conditional reward, retained-record
transition, and successor filter must use the same boundary convention.

If utility is represented only as terminal utility, set every intermediate
segment reward to zero. One must not add banked reward here and then include it
again in terminal utility.

### 10.3 Exact criterion for a coordinate-only value

Fix a decision problem, let \(\mathsf V(I)\) denote its exact continuation
value, and let \(q:I\mapsto c\) be a projection.

**[THEOREM — proved mathematically: necessary and sufficient criterion]** There
exists a scalar function

\[
v:q(\mathcal I)\to\mathbb R
\]

on the image of the stated information-state domain \(\mathcal I\) such that

\[
\mathsf V=v\circ q
\]

if and only if

\[
q(I_1)=q(I_2)\Longrightarrow \mathsf V(I_1)=\mathsf V(I_2).
\]

**Proof.** Necessity follows by substitution. For sufficiency, define
\(v(c)\) to be \(\mathsf V(I)\) for any \(I\) with \(q(I)=c\). Constancy on
each nonempty projection fiber makes the definition well defined. Values on
coordinate points outside \(q(\mathcal I)\) are irrelevant; if a total function
on a larger declared codomain is desired, extend \(v\) arbitrarily there. ∎

For action values, the corresponding theorem additionally requires a
well-defined legal-action transport between records mapped to the same
coordinate and constancy of each transported \(\mathsf Q(I,a)\).

**[CONSTRUCTED COUNTEREXAMPLE: scalar value factorization is weaker than action
factorization]** Let two information states \(I_1,I_2\) share one coordinate
\(c\), with common legal actions \(a,b\), and let

\[
(\mathsf Q(I_1,a),\mathsf Q(I_1,b))=(1,0),
\qquad
(\mathsf Q(I_2,a),\mathsf Q(I_2,b))=(0,1).
\]

Then \(\mathsf V(I_1)=\mathsf V(I_2)=1\), so the optimized scalar value factors
through \(c\), but neither the action-value vector nor an optimal action does.
Consequently a value cache can be exact on a coordinate that is still
insufficient for action selection or policy representation.

The criterion is exact but does not explain *why* constancy holds. One strong
joint sufficient package is:

1. equal augmented posterior measures up to a value-preserving world/latent
   isomorphism;
2. equal required retained continuation record, or a proved record transport;
3. equal field-state transition and continuation behavior under that
   isomorphism;
4. equal utility residue;
5. commuting action, observation, physical, record, and belief-transition
   correspondence.

These conditions are not individually necessary, and even the joint package
is stronger than necessary: different posteriors can accidentally produce the
same values. A coordinate-Markov future policy alone is not sufficient because
evidence accumulated before reaching \(c\) can leave different posterior
weights.

### 10.4 Exact legal 90-world history counterexample

This construction lies entirely inside no-trump Straight 42. It is exhaustively
checked by `verification/verify_foundation.py`.

#### Common contract and endpoint

- shaker: seat 3;
- bidder: seat 3;
- winning bid: \(P(31)\);
- declaration: no-trump;
- viewer and decision maker: seat 3;
- five complete tricks have been played;
- fixed-team scores are \((2,18)\), so the declaring partnership has 18;
- leader is seat 3;
- current trick is empty;
- seat 3 holds `3-1` and `4-1`.

The contract remains live. Twenty-two points remain; the declaring partnership
needs 13 of them to reach 31.

The compared path-free mechanical projection retains shaker, bidder, winning
bid, declaration, viewer hand, current leader and trick, fixed-team scores,
played-domino attribution, current capacities, public void exclusions, and the
resulting cells. It deliberately does not retain the losing auction-action
sequence. The two histories below map to the same value of that projection.

The common public play history is:

1. `3:6-3, 0:6-1, 1:6-4, 2:6-0` — seat 1 wins 11;
2. `1:0-0, 2:2-2, 3:5-0, 0:2-0` — seat 1 wins 6;
3. `1:4-3, 2:4-2, 3:4-0, 0:5-4` — seat 0 wins 1;
4. `0:1-1, 1:3-0, 2:3-3, 3:2-1` — seat 0 wins 1;
5. `0:1-0, 1:6-6, 2:5-2, 3:5-1` — seat 3 wins 1.

The derived public voids are: seat 1 is void in ones; seat 2 is void in blanks
and ones; seats 0 and 3 have no derived void. The unseen pool is

```text
5-5, 4-4, 3-2, 6-5, 5-3, 6-2
```

and each hidden seat has capacity two. None of these six dominoes lies in the
excluded blank or one suits. Therefore every ordered partition into three
labeled pairs is legal:

\[
|\Phi(c)|=\frac{6!}{(2!)^3}=90.
\]

The verifier replays the public prefix in all 90 complete deals and confirms
that every play is legal and every mechanical endpoint is identical.

#### Two pointwise anchor worlds

Two members of the 90-world fiber are:

| seat | \(\varpi_0\) | \(\varpi_1\) |
|---:|---|---|
| 0 | `5-5, 4-4` | `5-5, 6-5` |
| 1 | `3-2, 6-5` | `3-2, 4-4` |
| 2 | `5-3, 6-2` | `5-3, 6-2` |
| 3 | `4-1, 3-1` | `4-1, 3-1` |

Use the canonical global DominoId order

```text
0-0, 1-0, 1-1, 2-0, 2-1, 2-2, ..., 6-6
```

and, after the endpoint, let every nonviewer seat play its lowest legal ID.
Seat 3 chooses the root lead. After that lead seat 3 has only one
domino, so every later seat-3 action is forced; averaging root values across
worlds introduces no strategy fusion.

For signed differential on the remaining 22 points, exact backward induction
gives:

\[
\begin{array}{c|rr}
&Q(3\!:\!1)&Q(4\!:\!1)\\\hline
\varpi_0&10&-22\\
\varpi_1&-22&22
\end{array}
\]

These are pointwise anchors, not the whole fiber and not point-mass posterior
claims.

#### Two public auction histories

Let

\[
\begin{aligned}
\alpha_A&=(0:\mathrm{pass},\ 1:P(30),\ 2:\mathrm{pass},\ 3:P(31)),\\
\alpha_B&=(0:P(30),\ 1:\mathrm{pass},\ 2:\mathrm{pass},\ 3:P(31)).
\end{aligned}
\]

Both are legal and produce the same bidder, contract, declaration, and later
mechanical endpoint.

For seats \(s\in\{0,1\}\), define a valid stochastic bidding field as follows.
When \(P(30)\) is legal at that seat's turn,

\[
\Pr(P(30)\mid I_s)=
\begin{cases}
2/3,&4\!:\!4\in H_s^0,\\
1/3,&4\!:\!4\notin H_s^0,
\end{cases}
\]

and the remaining probability is assigned to pass. If \(P(30)\) is no longer
legal in the relevant history, the seat passes with probability one. Seat 2
passes, and seat 3 bids \(P(31)\) and declares no-trump with probability one
on these paths. At information states not covered by these clauses, choose any
normalized legal continuation; those off-path choices do not affect the
witness.

During the displayed five-trick prefix, choose a common field that assigns
probability one to the displayed next tile whenever it is legal; it is legal in
all 90 worlds. At any other information state, complete the field with an
arbitrary normalized legal policy. Thus the displayed play prefix contributes
equal likelihood to every world and does not alter the auction-induced ratios.
The post-endpoint lowest-legal continuation is identical after both auction
histories, uses no additional latent field state, and is blind to the earlier
auction path. The required retained continuation record \(e\) therefore has
the same trivial value after both histories. Hence the strategic difference at
the
endpoint is exactly the posterior weighting, not hidden continuation state or
an omitted history input.

Partition the fiber by the hidden holder of `4-4`. Each holder class has 30
worlds. Under the uniform deal prior, Bayes' rule gives:

\[
\begin{array}{c|ccc}
&4\!:\!4\text{ at seat 0}&4\!:\!4\text{ at seat 1}&4\!:\!4\text{ at seat 2}\\\hline
\mu_A&1/7&4/7&2/7\\
\mu_B&1/2&1/4&1/4
\end{array}
\]

Every one of the 90 worlds has strictly positive probability under both
posteriors. The rule support and even the posterior support are therefore
identical; only the weights differ.

#### Exact values over the full fiber

The exact class-conditional means are:

\[
\begin{array}{c|rr|rr}
\text{holder of }4\!:\!4
&\mathbb E Q(3\!:\!1)&\mathbb E Q(4\!:\!1)
&\Pr(\text{make}\mid3\!:\!1)&\Pr(\text{make}\mid4\!:\!1)\\\hline
0&-104/15&-98/5&1/3&0\\
1&-122/15&86/5&1/3&4/5\\
2&-104/15&-98/5&1/3&0
\end{array}
\]

Here make/set is the actual \(P(31)\) contract. If \(q\) is signed differential
on the remaining 22 points, declaring remaining points are \((q+22)/2\), so
make requires \(q\ge4\).

Combining those class means with the two exact posteriors gives:

\[
\begin{array}{c|rr|c|rr|c}
&\mathbb E Q(3\!:\!1)&\mathbb E Q(4\!:\!1)
&\text{point-diff best}
&\Pr(\text{make}\mid3\!:\!1)&\Pr(\text{make}\mid4\!:\!1)
&\text{contract best}\\\hline
\mu_A&-160/21&10/7&4\!:\!1&1/3&16/35&4\!:\!1\\
\mu_B&-217/30&-52/5&3\!:\!1&1/3&1/5&3\!:\!1
\end{array}
\]

The already banked point differential is the same constant in both histories,
so the point-differential action ordering is also the final-differential
ordering.

**[THEOREM — exhaustive finite verification / CONSTRUCTED COUNTEREXAMPLE]**
Two legal public histories have the same mechanical endpoint, the same exact
90-world rule fiber, and the same 90-world posterior support, yet their
likelihood weights make opposite leads optimal. Because declaring points are a
positive affine transform of signed differential, and one-mark hand utility is
a positive affine transform of contract-make probability, the flip holds under
all four named current-hand lenses:

- expected declaring points;
- expected signed point differential;
- contract-success probability;
- declaring-oriented one-mark hand utility.

Mechanical state alone is not an exact strategic state for this policy model.

### 10.5 Higher-order beliefs

Assume a common prior, common knowledge of rules and the complete policy model,
public actions, complete private types (including hand records and any modeled
private field signals), and perfect recall. Work in a finite or countable type
model, or more generally in a standard-Borel model for which the required
regular conditional probabilities have been fixed. At any positive-probability
information state, each player's posterior is then determined by Bayes' rule.
A player's belief about another player's belief is the pushforward of the
first posterior through the measurable map from latent world and complete type
to the other player's posterior. This construction iterates.

**[THEOREM — proved mathematically, on-path scope]** Under those assumptions,
every finite order of belief is induced by the common prior, public history,
complete private types, and common policy model; it is not an additional
independent primitive.

**Proof.** First-order posteriors are fixed by Bayes' rule at positive-
probability information states. Given beliefs through order \(k\), the latent
world and another player's complete type determine that player's order-
\(k\) belief object; pushing the current player's posterior through this map
determines order \(k+1\). Induction gives every finite order. ∎

At zero-probability histories, or under uncertainty about policy models,
private signals, or correlation devices, an enlarged type space and an
off-path assessment are required.

---

## 11. Utility and value

### 11.1 Utility lenses

Let \(P_D\) be final declaring-partnership points.

\[
U_{\mathrm{pts}}=P_D.
\]

\[
U_{\mathrm{diff}}
=P_D-(42-P_D)
=2P_D-42.
\]

\[
U_{\mathrm{make}}=\mathbf1[P_D\ge\Theta].
\]

Declaring-oriented hand-mark utility is

\[
U_{\mathrm{marks}}=
\begin{cases}
+w,&P_D\ge\Theta,\\
-w,&P_D<\Theta.
\end{cases}
\]

A match utility is an explicitly named function of the terminal match result
and, if desired, any additional retained history. No one lens is selected as
the universal utility.

### 11.2 Value is derived

For strategy profile or continuation field \(\pi\), let \(\Xi_I\) be an
ambient admissible augmented latent domain compatible with information state
\(I\) under the selected chance and field model, and let \(\beta_I\) be the
normalized belief measure on that domain. It may be concentrated on a proper
measurable subset. Let \(Z_{\mathrm{term}}\) denote the terminal outcome.
Assume the displayed conditional utilities are measurable and integrable
(bounded utility is sufficient). Then

\[
V_i^\pi(I)
=
\int_{\Xi_I}
\mathbb E^\pi[U_i(Z_{\mathrm{term}})\mid I,\xi]
\,\beta_I(d\xi).
\]

For legal action \(a\),

\[
Q_i^\pi(I,a)
=
\int_{\Xi_I}
\mathbb E^\pi[U_i(Z_{\mathrm{term}})\mid I,\xi,a]
\,\beta_I(d\xi).
\]

**[COROLLARY / STRUCTURAL SYNTHESIS]** Value is determined by rules, support,
retained observation record, belief, continuation behavior, required latent
field state, and utility. It is not an independent component of physical
state.

### 11.3 Utility relationships and an explicit threshold reversal

**[THEOREM — proved mathematically]** In full play,
\(U_{\mathrm{diff}}=2U_{\mathrm{pts}}-42\). Therefore expected points and
expected signed point differential induce the same action ordering.

**Proof.** The defending partnership receives exactly \(42-P_D\) points, so
the signed differential is \(P_D-(42-P_D)=2P_D-42\). Positive affine
transformation preserves expected-value order. ∎

**[CONSTRUCTED COUNTEREXAMPLE]** For threshold 31, compare two terminal
lotteries:

- \(A\): 30 declaring points with probability one;
- \(B\): 42 points with probability \(0.7\) and 0 points with probability
  \(0.3\).

Then

\[
\mathbb E[P_D\mid A]=30
>
29.4=\mathbb E[P_D\mid B],
\]

but

\[
\Pr(\mathrm{make}\mid A)=0
<
0.7=\Pr(\mathrm{make}\mid B).
\]

Expected points and contract-success utility can rank terminal-score lotteries
oppositely. This distributional witness does not claim that the two displayed
lotteries arise as actions at one legal information state; §10.4 supplies an
actual legal action reversal.

### 11.4 Fixed-field information-set best response

Fix one player's contracted-hand continuation problem, a normalized belief
\(\beta\), a fixed continuation field for every other actor, and a bounded
utility. Assume the native finite Straight 42 observation model, or another
model with only finitely many reachable information records for seat \(m\).
Let \(\mathcal R_m\) be the finite set of deterministic contingent policies
assigning one legal action to every such future information record.

**[THEOREM — proved mathematically: best-response existence]** There exists a
deterministic information-set-consistent best response

\[
\rho_m^*
\in
\arg\max_{\rho_m\in\mathcal R_m}
\mathbb E_\beta
\left[
U_m(\operatorname{terminal};\rho_m,\sigma_{-m})
\right].
\]

Allowing arbitrary **private randomization independent of the hidden world
and fixed field**, including correlation across the player's future information
records, cannot improve on the best deterministic contingent policy. A random
variable correlated with the world or field is an additional signal or
correlation device and changes the information structure; it is not mere
private randomization.

**Proof.** The contracted game has finitely many deals, public histories,
private hand records, and legal actions, so \(\mathcal R_m\) is finite and a
maximum exists. Full perfect-recall information records contain a strictly
longer public prefix as play advances, so the same record cannot be revisited
along one continuation.

Condition on the complete realization of the independent private random
tape used by a randomized policy. That realization selects one legal action at
every future information record and therefore defines one member of
\(\mathcal R_m\). Because the tape is independent of the hidden world and
fixed field, averaging over it expresses randomized-policy utility as a convex
combination, or integral mixture, of the same deterministic-policy utilities.
It cannot exceed their maximum. ∎

The policy selects one action at each information state. It may condition on
later public observations, but not on the hidden world itself. Choosing the
best action separately in each hidden world and then averaging is strategy
fusion unless those world-contingent choices correspond to information the
player will actually observe.

**[BOUNDARY]** The displayed finite-set proof does not automatically cover a
field model that gives the player an arbitrary infinite private-signal or
internal-state space. Extensions can still admit deterministic best responses,
for example under standard-Borel state spaces, measurable finite legal-action
correspondences, measurable continuation values, and an appropriate measurable
selection argument. Those conditions must be stated; they are not supplied by
finiteness of the physical 42 hand alone.

### 11.5 Partnership information boundary

Opposite seats share utility but observe different private hands and act at
different information states.

**[PROPOSITION — proved by information comparison]** Replacing a partnership
by one controller that observes both hands defines a different extensive-form
information structure unless the added observation is redundant under a
separate equivalence theorem. Shared utility alone does not establish such an
equivalence or justify centralization.

**Proof.** In the original game, each partner's information record contains
one private hand and the public history. The centralized controller's record
contains both private hands, so it strictly refines the original information
partition whenever two deals agree on the acting partner's observations but
differ in the other partner's hand. Equality of utility functions does not
identify those information partitions. ∎

No equilibrium concept is selected by this foundation.

---

## 12. Congruences, quotients, gauges, and restrictions

### 12.1 Physical congruence

A projection from objective histories to reduced states is a physical
congruence when equal projected states have corresponding legal actions,
rewards, terminal status, and successor projected states.

**[THEOREM — proved mathematically]** The contracted-play state \(X_t\) is a
physical congruence for the remainder of the current hand. Under the baseline
independent-deal chance law, the pair \((X_t,Y_r)\) is a congruence for hand
settlement, objective match progression, and the next deal-attempt chance
transition. A different cross-deal law may require additional retained state.

**Proof.** Within the hand this is §5.5. Settlement is determined by the
contract and final hand score in \(X_t\); updated marks, terminal target test,
and next shaker are determined by \(Y_r\). Under the baseline law the next
deal distribution depends on no omitted prior state. ∎

A viewer mechanical/support state is instead a bundle of possible objective
states.

### 12.2 Field-relative strategic quotient

Fix a finite contracted continuation, policy field, utility, and allowed
strategy class.

**[THEOREM — proved mathematically]** An isomorphism preserving:

1. legal-action correspondence;
2. physical transition and viewer-observation correspondence;
3. the required retained continuation record and its update;
4. augmented posterior measure;
5. latent field-state transition and continuation behavior;
6. immediate and terminal utility

preserves every continuation and action value in the stated decision problem.

**Proof.** Terminal utilities agree by hypothesis. At each predecessor, the
isomorphism transports legal actions, immediate utility, observation kernels,
retained-record updates, posterior updates, field continuation, and successor
values. Backward induction on the finite remaining-play grade therefore
preserves every value and action value. ∎

Every proposed strategic quotient must name and prove the hypotheses it uses.

### 12.3 Current support can forget some attribution while belief cannot

**[PROPOSITION — proved under Straight 42 cell assumptions]** Actor attribution
of already completed dominoes need not be stored in the *current hidden-hand
fiber* once unseen pool, exact capacities, current void exclusions, and all
other current support residue are retained.

**Proof.** The fiber definition in §7.3 depends only on the named pool,
capacities, possible-holder sets, disjointness, and conservation. Completed-
tile attribution influences that fiber only through derived residue such as
capacities and voids; once those values are retained, the raw attribution does
not appear in the defining predicate. ∎

The attribution remains part of public history. It can be required to
reconstruct initial hands, evaluate policy likelihood, or preserve perfect
recall. A support quotient is not automatically an evidence quotient.

### 12.4 Original information partition versus abstraction

**[COROLLARY / STRUCTURAL SYNTHESIS]** A key exactly
represents the original perfect-recall information partition only if it does
not merge distinct private-observation/public-history records belonging to the
same player.

A coarser key may still be:

- an intentional abstraction;
- a sufficient statistic for a fixed policy/utility class;
- a value-preserving quotient under a separate theorem.

Those claims are weaker than equality with the original information state and
must be named as such.

### 12.5 Seat rotations

For \(k\in\mathbb Z/4\mathbb Z\), let

\[
r_k(s)=s+k\pmod4.
\]

Transport every seat-indexed object simultaneously: hands, shaker, bidder,
leader, actors, public/private records, cells, policy types, team labels, and
utility orientation.

**[THEOREM — proved mathematically]** These rotations form a \(C_4\) symmetry
of Straight 42 under complete transport. Odd rotations exchange the fixed team
labels, so utility orientation must be transported as well.

**Proof.** Rotation commutes with clockwise successor:
\(r_k(s+1)=r_k(s)+1\). It preserves opposite-seat partnership incidence,
domino physics, auction order, trick order, and shaker advancement after all
seat-indexed objects are transported. Composition satisfies
\(r_k\circ r_j=r_{k+j}\), giving the cyclic group of order four. ∎

### 12.6 Bidder anchoring

After a bidder \(b\) exists, rotate by \(-b\) so the bidder is seat zero while
retaining every relative seat offset.

**[THEOREM — proved mathematically]** Bidder anchoring is an exact post-auction
seat-label gauge when shaker offset, partnership orientation, public history,
private types, cells, and utility are all transported. It is undefined before
a bidder exists.

**Proof.** Apply the exact rotation \(r_{-b}\) from §12.5. It sends bidder
\(b\) to seat zero and preserves every transported rule and strategic object.
Before auction completion there is no bidder label from which to choose the
rotation. ∎

### 12.7 Reflection failure

Let \(f(s)=-s\pmod4\). Then for every seat

\[
f(s+1)=f(s)-1\ne f(s)+1\pmod4.
\]

**[CONSTRUCTED COUNTEREXAMPLE]** After seat 0 leads, the next actor in the
clockwise game is seat 1. Reflection fixes seat 0 and maps that original next
actor to seat 3, while the reflected state governed by the same clockwise rule
still requires seat 1 to act. Thus reflection does not commute with the
one-action transition relation and is not an automorphism of the oriented
extensive game.

A completed trick's final winner can sometimes be transported after reversing
the displayed follower order, but that reversal changes the causal order in
which followers observed and acted. It is not a symmetry of mid-trick decision
nodes. Reflection is an isomorphism only to a separately defined
counterclockwise variant when every orientation-dependent rule is transported.

### 12.8 Explicit predicates and stopping frontiers

For any world or state predicate \(R\), set intersection with \(R\) is an
exact restricted object.

A traversal stopped at the first state satisfying \(R\) returns a frontier.
The frontier states do not become terminal game states and have no terminal
utility unless an additional quotient supplies one.

**[COROLLARY / STRUCTURAL SYNTHESIS]** A named predicate does not change the
unrestricted object. Silent hand horizons, world caps, truncation, or sampling
answer different questions.

### 12.9 Outcome-determined terminal quotient

At a completed-trick boundary, let declaring and defending banked points be
\(P_D,P_F\).

For point threshold \(n\):

- if \(P_D\ge n\), make/set is fixed as made;
- if \(P_F>42-n\), make/set is fixed as set.

For a mark contract:

- if \(P_F>0\), make/set is fixed as set.

**[THEOREM — proved mathematically]** Replacing such a state by a terminal node
preserves:

1. contract-success utility for the current hand;
2. the current hand's mark award;
3. the immediate match-score update;
4. whether that award ends the match at the current target.

It does not preserve final raw hand points or omitted public observations.

**Proof.** In each listed condition, no allocation of the remaining 42-point
residue can change make versus set. Contract and stake therefore fix the
receiving partnership and mark award. Applying that fixed award to the retained
match score fixes the immediate score update and whether the target is reached.
Unplayed point allocation and public actions are not determined. ∎

**[PROPOSITION — full future match value under extra assumptions]** The
quotient also preserves continuation value through later hands if the future
deal law and continuation model reset so that, after the current award, future
behavior depends only on the updated match score, next shaker, and other
retained state—not on omitted plays, final raw points, or learning from those
observations.

**Proof.** The theorem gives identical updated retained match state. Under the
stated reset/conditional-independence assumption, both branches then induce
the same future chance and policy law and therefore the same continuation-
utility distribution. ∎

Without those assumptions, “preserves match value” is too strong. Full
seven-trick play remains the primitive game.

---

## 13. Typed commutative diagrams

### 13.1 Complete deal to current remainder

```text
compatible current deals Ω_{r,t}(I_{r,t}) --ρ_{r,t}--> remainder fiber Φ(c_{r,t})
                    |                                      |
                    | append legal public action           | typed remainder update
                    v                                      v
compatible current deals Ω_{r,t+1}(I_{r,t+1}) --ρ_{r,t+1}-> Φ(c_{r,t+1})
```

The left domain contains fixed initial deals. The right domain contains current
remaining hidden hands.

### 13.2 Viewer bundle

```text
mechanical state c + remainder world ω_t  --->  one current objective state X
       |                                              |
       | public action                                | objective transition
       v                                              v
mechanical state c' + remainder world ϑ(ω_t) -> successor objective state X'
```

For a viewer action, \(\vartheta\) is the identity on hidden remainders.
An all-pass transition instead ends the current deal domain and applies the
new-deal chance kernel described in §8.6.

### 13.3 Belief filter

```text
current-attempt augmented belief ν_{r,t}
                 |
                 | legality × modeled action likelihood
                 v
current-attempt augmented belief ν_{r,t+1}
                 |
                 | pushforward through ρ_{r,t+1}
                 v
augmented current belief β_{r,t+1}
```

### 13.4 Declaration selection

```text
marked auction hand in (A_δ)_δ
              |
              | Sel_δ
              v
marked play hand in A_δ
```

---

## 14. Honest boundaries

Established:

- the reduced contracted-play state is sufficient for physical continuation;
- that state plus match residue including shaker is sufficient for objective
  match continuation under the selected cross-deal law;
- rule-derived capacity cells are lossless for Straight 42 current rule
  support within the exact scope of §7.5;
- the total support normal form of §7.10 is the coarsest exact deterministic
  semantic representation of all capacity-cell support, with one empty tag for
  infeasible systems and one minimized payload per nonempty fiber;
- its native ambiguity component is irreducible in the direct holder/quota
  language, and every stored ternary exclusion is essential;
- exact strategic state \((c,e,\beta)\) is sufficient for a fixed continuation
  model and utility under §10.1 assumptions.

Not established:

1. global minimality of the complete mechanical state beyond its support
   component;
2. one byte-minimal or runtime-minimal encoding independent of a named cost
   model and required operations;
3. a minimal field-state or augmented latent representation for arbitrary
   fields;
4. a minimal utility residue for every history-dependent utility;
5. a general low-dimensional exact strategic quotient beyond proved gauges;
6. unique beliefs at zero-probability histories without an assessment;
7. extension of the cell and support-normal-form theorems to special contracts
   or altered information rules;
8. computational tractability of arbitrary predicate-restricted counting or
   enumeration;
9. a canonical probability measure or sampling law determined by support
   alone;
10. deterministic best-response existence for arbitrary added infinite
    private-signal models without stated measurability/selection assumptions;
11. algorithmic exact computability from finiteness alone when utilities or
    operators are not effectively representable.

Additionally, no finite-horizon backward-induction claim is made for the full
pre-contract match process with unbounded repeated pass-outs. A full-match
value requires a named infinite-horizon formulation or assumptions ensuring
termination and the needed integrability.

These are mathematical boundaries, not instructions about what must be studied
next.

---

## 15. Final factorization

Let

\[
I_\tau^m=(\psi_\tau^m,h_\tau)
\]

be the match-global perfect-recall information state, and let
\(I_{r,t}^{m,\mathrm{deal}}\) be its current-deal component. Let

\[
c_{r,t}^m=q_{r,m}(I_\tau^m)
\]

be an exact mechanical/support projection for a named current-deal scope. Let

\[
e_{r,t}^m=s_{r,m}(I_\tau^m)
\]

be the required retained continuation record: precisely the viewer-known public
or private observation residue not carried by \(c_{r,t}^m\) that the selected
strategy, continuation model, or utility can still consult. It may be trivial, a
proved summary, or the relevant full slice of the viewer's perfect-recall
record. Hidden actors' private residue remains in the augmented latent state.

Let

\[
\Omega_{r,t}^m(I_{r,t}^{m,\mathrm{deal}})
\]

be the compatible complete deals for attempt \(r\), and let

\[
\Phi(c_{r,t}^m)
=
\rho_{r,t}^m
\left(
\Omega_{r,t}^m(I_{r,t}^{m,\mathrm{deal}})
\right)
\]

be the exact current-remainder fiber under the Straight 42 losslessness
scope of §7.5. It is presented locally by rule-derived capacity cells and
canonically represented by the feasible branch
\(\mathcal N(c_{r,t}^m)\) of the total support normal form in §7.10. When
\(c_{r,t}^m\) is constructed by the legal objective transition system,
\(\mathcal N(c_{r,t}^m)\in\mathscr R_{\mathrm{Str}}^m\) by construction;
reachability is an inductive type invariant, not another serialized field.

Let \(p_{r,0}^m\) be the viewer's current-attempt prior on augmented root
worlds \(\xi_r=(\omega_r,\zeta_r)\), conditioned on the complete match-global
record at the start of the attempt. Let \(\nu_{r,t}^m\) be the posterior on
those root worlds after the current public prefix. Let
\(\widetilde\nu_{r,t}^m\) additionally retain the exact current field state,
either by a valid deterministic random-tape realization or by the field's
conditional state kernel, and let \(\beta_{r,t}^m\) be its pushforward to
current remainder and every field-state component required for continuation.
It is a normalized measure on the ambient admissible domain
\(\Xi(c_{r,t}^m,e_{r,t}^m)\). It may be concentrated on a strict measurable
subset; in a finite or countable model its positive-mass support may be a
strict subset of the ambient domain.

**[COROLLARY / STRUCTURAL SYNTHESIS]** The conceptual expansion of the
mechanical state is deterministic:

\[
c_{r,t}^m
\longmapsto
\left(
\mathcal A_\delta,
\mathfrak H_{r,t}^m,
\mathbf C(c_{r,t}^m),
\overline{\mathcal N}(c_{r,t}^m),
\Phi(c_{r,t}^m),
\Omega_{r,t}^m(I_{r,t}^{m,\mathrm{deal}})
\right).
\]

These are exact semantic views of one physical/support source, not six
independent state fields. In particular, declaration algebra, marked-hand
view, rule-derived cells, support normal form, current fiber, compatible
current-attempt deals, and the fact of legal-prefix reachability must not be
redundantly serialized merely because each has a useful mathematical name.

The belief construction remains

\[
(p_{r,0}^m,\Pi,h_{r,t})
\longmapsto
\nu_{r,t}^m
\longmapsto
\widetilde\nu_{r,t}^m
\longmapsto
\beta_{r,t}^m.
\]

Relative to fixed rules, continuation field \(\Pi\), utility \(U\), and allowed
strategy class, the independent exact current decision state is therefore

\[
B_{r,t}^m=(c_{r,t}^m,e_{r,t}^m,\beta_{r,t}^m),
\]

and, for legal action \(d\),

\[
Q_U^\Pi(I_\tau^m,d)
=
\mathcal V_U^\Pi
\left(
 c_{r,t}^m,
 e_{r,t}^m,
 \beta_{r,t}^m,
 d
\right).
\]

Here:

- \(c_{r,t}^m\) retains exact physical/support residue for the named scope and
  derives \(\mathcal A_\delta\), \(\mathfrak H\), cells, normal form, fiber,
  and compatible current-attempt deals;
- \(e_{r,t}^m\) retains exactly the viewer-known public or private observation
  input still required by the selected strategy, continuation field, or
  utility beyond \(c_{r,t}^m\);
- \(p_{r,0}^m\) carries earlier match evidence relevant to the current attempt
  through its conditioning and inherited latent state;
- \(\beta_{r,t}^m\) carries current-world weights together with every latent
  continuation-state component that cannot be factored away;
- the action transition relocates one controlled node and updates physical
  state, retained record, support typing, and belief;
- \(\mathcal V_U^\Pi\) derives value under the named field and utility.

The conceptual thesis remains algebra × marked structure × exact hidden
complement × evidence-weighted continuation. The executable source of truth is
stricter: deterministic views are derived once from \(c\), not duplicated as
independent state. No minimality claim is made for \(c\), \(e\), or \(\beta\)
beyond the support-component theorem proved in §7.10 and its exact
Straight-reachable restriction in §7.13.
