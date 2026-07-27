# Research Mandate for GPT-5.6 Pro
## A front-to-back native formalization of Texas 42

**Prepared:** 2026-07-23  
**Repository:** `jasonyandell/mk5-main`  
**Primary live work:** PR #95, stacked on PR #92 and PR #91  
**Requested mode:** maximum mathematical effort, adversarial proof, no implementation-first shortcut

---

## 0. The assignment

Produce a rigorous, front-to-back mathematical specification of straight Texas 42 using the emerging native representation:

> **A hand is a marked substructure embedded in the game algebra, not a list of independently meaningful pieces.**

and:

> **An action spends one currently controlled node and is valued by the transition it induces in the entire structural and epistemic state.**

Do not merely polish the existing wiki language. Re-derive the game from first principles, decide which current claims are actually theorems, repair false or overstated claims, and prove the corrected specification to the greatest extent possible.

The final theory must cover, in one coherent system:

1. the double-six domino algebra;
2. the auction;
3. declaration as selection of a game algebra;
4. objective world state;
5. each seat’s information state;
6. exact delimited hidden-location cells;
7. the compatible-world fiber;
8. belief as a measure on that fiber;
9. policy and action evidence;
10. play as structural transition plus observation;
11. hand-as-set / marked-substructure semantics;
12. scoring, contracts, marks, and terminal utility;
13. value and best response;
14. congruences, quotients, gauges, and symmetries;
15. the exact boundary between physics, information, belief, policy, and value.

The intended output is not a philosophical essay. It is a specification with definitions, commutative diagrams where useful, lemmas, theorems, proofs, counterexamples, and a proof-status ledger.

A CFR implementation outline is a **stretch annex only**. Do it only after the game-theoretic object is correct. Do not let CFR terminology distort the underlying theory.

---

## 1. Read these sources before committing to notation

The repository is moving. Re-read the current heads, not only the commit hashes named here.

### Primary current sources

- PR #95 — `wiki/topics/fundamental-factorization.md`
- PR #92 — `atlas/SPEC.md`, `atlas/`, `wiki/entities/atlas.md`
- PR #91 — `roles/`, `wiki/topics/role-threat-tensor.md`
- PR #84 — `hoyt/equivcensus.py`, `wiki/topics/endgame-equivalence-census.md`
- `wiki/topics/forty-two-native-object.md`
- `wiki/topics/suit-algebra-spec.md`
- `wiki/topics/play-phase-algebra.md`
- `wiki/topics/rules-of-42.md`
- `wiki/topics/belief-policy-value-algebra.md`
- `wiki/topics/walt-spec.md`
- `forge/oracle/tables.py`
- `forge/oracle/declarations.py`
- the TypeScript engine rules and transitions where they remain the production authority

### Existing machinery to treat as evidence, not axioms

- Atlas parity gates C1–C6
- role/threat gates R1–R6
- Walt correctness gates
- Hoyt exact-value and CFR reference machinery
- the endgame equivalence census
- the bid-decoder and belief work
- the measured forced-action and branching statistics

Tests and measurements are strong receipts. They are not automatically mathematical proofs beyond their exact enumerated scope.

---

## 2. Required epistemic discipline

Every important statement in the final specification must be tagged as one of:

- **DEFINITION**
- **AXIOM / RULE AUTHORITY**
- **THEOREM — proved mathematically**
- **THEOREM — exhaustive finite verification**
- **PROPOSITION — proved under explicit assumptions**
- **IMPLEMENTATION INVARIANT — parity gated**
- **EMPIRICAL FINDING**
- **CONJECTURE**
- **COUNTEREXAMPLE / REFUTATION**
- **OPEN PROOF OBLIGATION**

Never slide from one category into another.

In particular:

- Do not promote the role-basis regression result into a theorem that structural coordinates are universally smoother.
- Do not promote a C4 spot gate into proof that strategic value is a function of a path-free coordinate.
- Do not call a compact representation a Cartesian factorization unless the compatibility constraints and dependence among factors are handled explicitly.
- Do not call “node deletion” a global physical law if the node is actually relocated from hand to trick to played pile.
- Do not identify a standard extensive-form information set with a belief state without proving the equivalence and preserving perfect recall.
- Do not claim CFR convergence that is unavailable in a four-agent shared-team imperfect-information game.

When a current project statement is wrong, state the counterexample clearly and replace it.

---

# Part I — The finite game algebra

## 3. Basic sets

Let

\[
\mathbb P=\{0,1,2,3,4,5,6\}
\]

be the pip set.

The double-six domino set is

\[
\mathcal D=\{\{i,j\}: i,j\in\mathbb P,\ i\le j\},
\qquad |\mathcal D|=28.
\]

A domino is a two-element multiset; doubles are allowed.

Let the seats be

\[
S=\mathbb Z/4\mathbb Z.
\]

The fixed partnerships are

\[
T_0=\{0,2\},\qquad T_1=\{1,3\}.
\]

The final theory should use seat-relative coordinates wherever possible and explicitly identify absolute-seat labeling as a gauge when it is one.

---

## 4. Declarations and declaration layers

For straight points/marks play, use the nine game declarations

\[
\Delta_{\rm game}
=
\mathbb P
\cup
\{\mathrm{DT},\mathrm{NT}\},
\]

where `DT` is doubles-trump and `NT` is no-trump/follow-me.

The rule tables may also contain doubles-as-an-unpowered-suit for nello. Keep that algebraic layer available as an extension, but do not silently include it in the straight-game auction domain.

For each declaration \(\delta\), define and prove the properties of:

1. the called set \(\kappa_\delta\subseteq\mathcal D\);
2. the powered set \(\pi_\delta\subseteq\kappa_\delta\);
3. the effective suit family
   \[
   \widehat\Sigma^\delta
   =
   \{\widehat\sigma^\delta_0,\ldots,\widehat\sigma^\delta_7\};
   \]
4. the led-suit map
   \[
   \ell_\delta:\mathcal D\to\{0,\ldots,7\};
   \]
5. the follow relation
   \[
   F_\delta(d,\ell)\in\{0,1\};
   \]
6. the declaration-relative rank;
7. the tier function;
8. the trick-order map
   \[
   \tau_\delta(d,\ell).
   \]

The formalization should preserve the important fact that the natural suits are a **covering**, not a partition. A mixed domino has two natural pip incidences, but declaration may absorb it into the called suit and remove it from its natural follow relations.

A physical domino identity is therefore not a stable strategic type.

---

## 5. The resident relational algebra

Define the declaration-indexed finite relational structure

\[
\mathcal A_\delta
=
\left(
\mathcal D;
\ \ell_\delta,
\ F_\delta,
\ \tau_\delta,
\ c,
\ \ldots
\right),
\]

where

\[
c:\mathcal D\to\{0,5,10\}
\]

is the count-point mark.

At minimum include these exact derived relations.

### 5.1 Contextual domination

For led-suit context \(q\in\{0,\ldots,7\}\),

\[
\operatorname{BEATS}_\delta(q,d)
=
\{e\in\mathcal D:
\tau_\delta(e,q)>\tau_\delta(d,q)\}.
\]

This is the full contextual comparison family.

### 5.2 Lead threat

\[
\operatorname{THREAT}_\delta(d)
=
\operatorname{BEATS}_\delta(\ell_\delta(d),d).
\]

This is the diagonal “when \(d\) leads” slice.

Given a live set \(O\subseteq\mathcal D\),

\[
R_\delta(d;O)
=
\operatorname{THREAT}_\delta(d)\cap O
\]

is the live lead-threat set.

Then:

\[
d\text{ is a walker relative to }O
\iff
R_\delta(d;O)=\varnothing.
\]

Its live threat count is

\[
r_\delta(d;O)=|R_\delta(d;O)|.
\]

### 5.3 Required theorem: monotone role promotion

If \(O'\subseteq O\), then

\[
R_\delta(d;O')\subseteq R_\delta(d;O)
\]

and

\[
r_\delta(d;O')\le r_\delta(d;O).
\]

This is an elementary theorem. State exactly what it means and what it does **not** mean. It proves monotonicity of this role coordinate as the live set shrinks; it does not prove monotonicity of strategic value.

### 5.4 Required theorem: unique trick winner

For any four distinct played dominoes with a designated leader, prove that the trick-order maximum is unique under the straight-game rules.

Connect the proof to the tier/rank injectivity inside the winning tier.

---

# Part II — The full game, front to back

## 6. Separate the one-hand game from the marks race

Define two games.

### 6.1 A single hand

A hand begins with:

- a dealer/shaker;
- a deal partition;
- an auction;
- a winning bidder;
- a contract;
- a declaration;
- seven tricks;
- contract settlement.

### 6.2 The match

A match has marks score

\[
M=(M_0,M_1)
\]

and terminates when a team reaches the target, normally seven marks.

The hand-to-hand transition must specify dealer progression and marks awarded.

Do not confuse:

- raw trick points;
- contract success;
- marks awarded for the hand;
- match utility.

Define each.

---

## 7. The deal

A complete deal is a partition

\[
\omega_0=(H_0,H_1,H_2,H_3)
\]

such that

\[
\mathcal D
=
H_0\sqcup H_1\sqcup H_2\sqcup H_3,
\qquad
|H_s|=7.
\]

State the chance distribution over deals. If the engine uses uniform random partitions, formalize that exactly.

If draw order or shaker rules produce the same uniform partition distribution, prove it or state the implementation convention.

---

## 8. The auction

Formalize the straight auction as an extensive-form phase.

At minimum define:

- dealer/shaker \(d\);
- first bidder;
- clockwise one-action-per-seat order;
- public auction history \(\beta\);
- legal action set at each auction node;
- pass;
- point bids \(30,\ldots,41\);
- mark bids;
- the all-pass rule used by the engine;
- the winning bidder;
- bid value and stake;
- declaration timing.

If plunge or other special contracts are excluded from the primary theory, say so. Do not partially model them.

The auction has two distinct roles:

1. it selects the contract and eventually the declaration;
2. each discretionary bid is public evidence about a private hand.

Before declaration, the game algebra is not one selected \(\mathcal A_\delta\). It is the declaration bundle

\[
\boldsymbol{\mathcal A}
=
(\mathcal A_\delta)_{\delta\in\Delta_{\rm game}}.
\]

The bidder evaluates one marked hand through all declaration layers. Winning the auction and declaring \(\delta\) selects one layer for play.

Formalize this phase change.

---

## 9. Contract semantics

Define a contract object \(K\) containing at least:

- bidder seat \(b\);
- bidder team \(T_b\);
- bid type;
- required point threshold;
- marks at stake;
- declaration \(\delta\).

For point bid \(n\in\{30,\ldots,41\}\), the declaring team makes the contract iff it takes at least \(n\) trick points.

For a straight mark bid, specify the engine’s exact required threshold and stake.

At settlement, define the marks transfer:

\[
\operatorname{award}(K,\text{declaring points}).
\]

Prove that this terminal award is a deterministic function of the contract and play result.

---

## 10. Objective play state

For a fixed complete deal and declaration, define the full physical play state.

A clean objective state may contain:

\[
X_t=
\left(
(H_s^t)_{s\in S},
L_t,
C_t,
P_t,
K,
M
\right),
\]

where:

- \(H_s^t\) is seat \(s\)’s remaining hand;
- \(L_t\) is the current trick leader;
- \(C_t\) is the ordered current trick prefix;
- \(P_t\) is banked hand points by team;
- \(K\) is the contract;
- \(M\) is match score if match utility matters.

Earlier completed-play order may be omitted from the **physical continuation state** if all rule-relevant residue has been retained.

Prove the Markov property of the chosen physical state:

\[
\Pr(X_{t+1},r_t\mid X_{\le t},a_t)
=
\Pr(X_{t+1},r_t\mid X_t,a_t).
\]

Because the rules are deterministic after the deal, this is a deterministic transition.

---

## 11. A play is relocation globally, deletion locally

Be precise about the “node deletion” slogan.

A domino is not annihilated from the global state. On play, it moves:

\[
\text{remaining hand}
\to
\text{current trick}
\to
\text{completed/played region}.
\]

Thus the global action is a **location-mark transition**.

It is deletion only from:

- the acting seat’s controllable remaining-hand set;
- the live future-action set after trick resolution.

Use terminology such as **node expenditure**, **live-node deletion**, or **location transition** if mathematically cleaner.

Required distinction:

> “Play is node deletion” is exact for the residual controllable/live substructure, but not for the full marked 28-node world.

---

## 12. Legality and transition

At a state \(X_t\), the acting seat is determined by leader and trick length.

If leading, every remaining domino is legal.

If following, define the subset able to follow the led suit. If nonempty, legal actions are exactly that subset; otherwise all remaining dominoes are legal.

Define the deterministic transition

\[
T(X_t,a_t)=X_{t+1}.
\]

When the fourth domino completes a trick:

- compute the unique winner;
- compute trick points;
- bank them to the winner’s team;
- set the winner as next leader;
- clear current trick.

Prove:

1. exactly one node leaves a remaining hand per play;
2. exactly 28 plays occur;
3. exactly seven tricks complete;
4. total hand points equal 42;
5. the play graph is a finite DAG graded by remaining live actions.

---

# Part III — Perspective, information, and exact hidden-state support

## 13. Objective world versus viewer perspective

Do not conflate these.

### 13.1 Objective world

An objective world contains the actual hidden deal and exact physical state.

### 13.2 Viewer information state

For viewer \(m\), the standard perfect-recall information state is

\[
I_t^m=(H_m^0,h_t),
\]

where:

- \(H_m^0\) is the viewer’s private initial hand;
- \(h_t\) is the full public action history through time \(t\).

Since all bids and plays are public, \(h_t\) is common. Remaining own hand is derivable.

This history-level object is the safe extensive-form definition. Any smaller coordinate is a quotient and must earn its sufficiency claim.

---

## 14. Mechanical coordinate versus evidence path

Define a projection

\[
q_m:h_t,H_m^0\mapsto c_t^m
\]

that retains the exact **mechanical and support residue** required for future rules and compatible hidden deals.

A coordinate may contain:

- phase;
- viewer;
- own remaining hand;
- public contract/declaration;
- current actor/leader/trick;
- banked points and marks residue;
- per-seat remaining counts;
- played/live masks as needed;
- publicly established voids;
- dealer only if strategically relevant;
- any memory state required by the modeled future policy.

The coordinate intentionally may discard some aspects of action order.

The public path \(h_t\), however, also carries **choice evidence**. Two histories may have the same mechanical coordinate and different likelihood under candidate hidden worlds.

Required core distinction:

\[
\text{mechanical support}
\neq
\text{posterior measure}.
\]

A coordinate can determine the first without determining the second.

---

## 15. Delimited location cells

For viewer \(m\), represent each hidden seat \(s\ne m\) by a capacity-constrained possible-location cell

\[
C_s=(P_s,k_s),
\]

where:

- \(P_s\subseteq U\) is the set of currently possible unseen dominoes for seat \(s\);
- \(k_s\) is the exact number of dominoes that seat still holds.

Known regions are degenerate cells:

\[
(P,k)\text{ is exact if }|P|=k.
\]

The viewer’s hand is one such exact cell.

The current trick and publicly played pile are also exact marked locations.

### 15.1 These cells are not independent factors

The hidden cells obey global constraints:

- common unseen pool \(U\);
- exact capacities;
- pairwise disjointness;
- union/conservation;
- void exclusions.

Therefore the information object is a dependent tuple, not an arbitrary Cartesian product.

Write it as a constrained family, pullback, fiber product, or another mathematically accurate construction.

---

## 16. The compatible-world fiber

Define

\[
\Phi(c_t^m)
\]

as the set of complete hidden assignments compatible with the coordinate.

For three hidden seats:

\[
\Phi(c)
=
\left\{
(H_s)_{s\ne m}:
H_s\subseteq P_s,
\ |H_s|=k_s,
\ H_s\cap H_{s'}=\varnothing,
\ \bigsqcup_{s\ne m}H_s=U
\right\}.
\]

Add every other exact physical constraint retained by the coordinate.

Enumeration is a query on this intensional object. It is not the state itself.

### 16.1 Required theorem or refutation: upper-bound-only physics

For straight 42 play, investigate and prove the claim:

> Public legal-play observations impose only exclusion constraints and exact capacities on still-hidden hands; no additional surviving positive “must contain at least one member of \(Q\)” clause is needed.

The intuitive argument is:

- failing to follow proves a void and removes a whole follow-set from \(P_s\);
- successfully following proves possession only of the domino just played, which immediately leaves the hidden hand;
- leading creates no suit-possession constraint;
- all remaining exact information is conservation and capacity.

Do not accept this argument casually. Prove it by induction over public play histories, or find the missing constraint.

State the exact scope:

- straight points/marks play;
- normal follow-if-possible;
- no special contract with altered hand visibility or sitting out;
- no policy-derived evidence, only rule-derived feasibility.

### 16.2 Feasibility theorem

For capacity cells, prove the Hall-type feasibility condition.

A hidden assignment exists iff for every subset \(R\) of hidden seats,

\[
\left|\bigcup_{s\in R}P_s\right|
\ge
\sum_{s\in R}k_s,
\]

together with total-pool equality.

Since there are three hidden seats from one viewer’s perspective, this can be checked exactly with a small finite family of inequalities or max flow.

### 16.3 Support refinement

After a public observation, define the updated coordinate \(c'\) and show how its fiber relates to the old fiber.

Be careful with the domain: worlds before and after a play have different remaining-hand representations. State the projection/transition map under which

\[
\Phi(c')\subseteq T_a(\Phi(c))
\]

or its correct equivalent holds.

Do not use an untyped subset claim across incompatible world spaces.

---

# Part IV — Belief, policy, and strategic state

## 17. Policy decomposition

For seat \(j\), define a behavioral policy

\[
\pi_j(a\mid I)
=
\mathbf 1[a\in A(I)]
\widetilde\sigma_j(a\mid I),
\]

where:

- legality is rule physics;
- \(\widetilde\sigma_j\) is discretionary choice over legal actions.

State the assumptions under which policies are common knowledge, stationary, Markov, history-sensitive, deterministic, or stochastic.

These assumptions materially change valid quotients and solver complexity.

---

## 18. Belief as a measure over the exact fiber

Let \(p_0(\omega\mid H_m^0)\) be the prior over deals compatible with the viewer’s hand.

Given public history

\[
h_t=(a_0,\ldots,a_{t-1}),
\]

define the posterior

\[
\mu_t^m(\omega)
\propto
p_0(\omega\mid H_m^0)
\mathbf 1[\omega\in\Phi(c_t^m)]
\prod_{\tau<t}
\widetilde\sigma_{j(\tau)}
\left(
a_\tau
\mid
I_\tau^{j(\tau)}(\omega)
\right).
\]

The legality indicators may be absorbed into the support term.

Define the physics posterior \(u_t^m\) as the normalized prior restricted only by exact feasibility.

Then

\[
\mu_t^m(\omega)
\propto
u_t^m(\omega)e^{g_t(\omega)}
\]

with

\[
g_t(\omega)
=
\sum_{\tau<t}
\log
\widetilde\sigma_{j(\tau)}
\left(
a_\tau
\mid
I_\tau^{j(\tau)}(\omega)
\right).
\]

Prove this tilt form.

---

## 19. Bayesian update under a new action

Given current belief \(\mu_t^m\) and observed action \(a_t\), define:

1. support transition;
2. likelihood multiplication;
3. normalization;
4. pushforward to the next world representation.

Write this as a filtering recursion.

Separate:

- the physical state transition;
- the observer’s support refinement;
- the posterior reweighting.

A single public play causes all three.

---

## 20. The correct strategic state

The current PR language risks claiming that value factors through a path-free coordinate under a “non-signaling” field. Repair this.

In general, two histories can reach the same mechanical coordinate and the same support fiber while inducing different posterior measures. Their optimal actions can differ.

Therefore the rational decision state against a known field is at least

\[
(c_t^m,\mu_t^m).
\]

If the future policy reads an internal memory state \(z_t\) not recoverable from \(c_t^m\), augment it:

\[
(c_t^m,\mu_t^m,z_t).
\]

Required theorem:

> Conditional on a fixed continuation-policy model and utility, expected continuation value is a function of the exact mechanical coordinate, the belief measure over its fiber, and any policy-memory state required by the continuation model.

Required counterexample:

> Mechanical coordinate alone does not determine strategic value when different paths induce different posteriors on the same fiber.

Construct the smallest explicit finite counterexample, preferably inside a realizable 42 subgame if practical.

---

## 21. Value is derived

For strategy profile \(\pi\), define

\[
V_i^\pi(I_t^i)
=
\mathbb E_{\omega\sim\mu_t^i}
\left[
G_i
\mid
I_t^i,\omega,\pi
\right].
\]

For legal action \(a\),

\[
Q_i^\pi(I_t^i,a)
=
\mathbb E_{\omega\sim\mu_t^i}
\left[
G_i
\mid
I_t^i,\omega,a,\pi
\right].
\]

Value is not an independent factor of the game state. It is a functional of:

- rules/algebra;
- structural coordinate;
- belief;
- continuation policy;
- utility.

Make this dependency explicit.

---

# Part V — The native hand object

## 22. Do not formalize the hand as a standalone induced graph

The phrase

> “a hand is a seven-node marked substructure”

is directionally correct but mathematically incomplete if interpreted as the induced relation on the seven held nodes.

External live nodes determine:

- threats;
- walkers;
- future promotions;
- hidden holders;
- follow possibilities;
- count exposure.

The correct object is a marked embedding **in situ**.

One possible formulation is:

\[
\mathfrak H_t^m
=
\left(
\mathcal A_\delta,
\ \lambda_t^m,
\ \iota:H_m^t\hookrightarrow\mathcal D,
\ c_t^m
\right),
\]

where \(\lambda_t^m\) marks every algebra node by known or possible location and live status.

Equivalently, use:

- the owned marked subset;
- its boundary relations to the ambient live algebra;
- the exact/delimited location cells on the complement.

Choose the cleanest category-theoretic, relational, or combinatorial formulation, but preserve the ambient boundary.

---

## 23. The hand before declaration

Before declaration, the hand is embedded in the declaration bundle:

\[
\mathfrak H_{\rm auction}^m
\subset
(\mathcal A_\delta)_{\delta\in\Delta_{\rm game}}.
\]

A declaration is not merely an integer feature. It selects/re-marks the algebra:

- called-set absorption changes effective membership;
- power changes comparison tiers;
- led-suit behavior changes;
- the same physical node acquires a different relational role.

Formalize declaration as an algebra-selection or re-marking operator.

Do not claim that the current five-feature declaration stack is the full \(28\times D\) object. Distinguish:

- the exact tilewise declaration stack;
- lossy pooled hand summaries used as engineered features.

---

## 24. Action as node expenditure

At time \(t\), a legal action is selection of one node

\[
a=d\in H_m^t.
\]

Its meaning is not a context-free scalar \(v(d)\).

It defines an operator

\[
\mathcal T_d:
(c_t^m,\mu_t^m)
\mapsto
(c_{t+1}^m,\mu_{t+1}^m)
\]

together with immediate reward when a trick completes.

Thus

\[
Q(c,\mu,d)
=
r(c,d)
+
\mathbb E[V(c',\mu')].
\]

The central thesis to prove or precisely scope is:

> The action is individual, but its strategic meaning is a functional of the whole marked hand-in-situ, the ambient hidden-state measure, and the transition induced by spending that node.

This should replace any primitive notion that dominoes possess independent strategic values which are later corrected by interactions.

---

## 25. Required invariance: local hand order is gauge

Any ordering of the remaining dominoes inside a hand is representational.

If a local permutation \(\rho\) reorders the hand tokens, a valid policy/value representation must be equivariant:

\[
Q(\rho\cdot \mathfrak H,\rho(d))
=
Q(\mathfrak H,d).
\]

A hand-value representation must be invariant:

\[
V(\rho\cdot \mathfrak H)=V(\mathfrak H).
\]

Prove the game-level local-index gauge and derive architecture requirements:

- set/graph encoders are natural;
- slot-index-sensitive encoders require explicit symmetrization or augmentation;
- action outputs should permute with the owned nodes.

This is a theorem about representation, not proof that a particular neural architecture will learn better.

---

## 26. Interaction is primary, not a correction term

Investigate whether any useful decomposition exists:

\[
Q(\mathfrak H,d)
=
q_1(d)
+
q_2(d,\mathfrak H\setminus\{d\})
+
q_{\ge 3}.
\]

Do not assume it does.

The deletion-derivative program should measure:

- first-order node contributions;
- shared blockers;
- redundancy;
- complementarity;
- guard/walker pairs;
- order dependence;
- higher-order interaction.

The important theoretical possibility is that no canonical context-free \(q_1(d)\) exists. If so, say so.

---

## 27. Lead versus follow roles

The lead-threat tensor is only one slice of the algebra.

The full native hand representation must be capable of expressing:

- lead roles;
- follow obligations;
- contextual trick rank under every possible led suit;
- slough status;
- trump power;
- count payload;
- current trick context.

Do not let “walker” become the ontology of the whole game.

Use the full `BEATS[declaration][led_suit][tile]` family or an equivalent exact structure.

---

# Part VI — Congruence, quotient, symmetry, and proof boundaries

## 28. History-to-coordinate congruence

Let \(\mathcal H\) be the set of legal histories and

\[
q:\mathcal H\to\mathcal C
\]

a proposed coordinate map.

Define a physical congruence precisely. For histories \(h,h'\) with \(q(h)=q(h')\), require a correspondence of:

- legal actions;
- immediate rewards;
- next coordinates;
- terminal conditions.

A strong deterministic form is:

\[
q(T(h,a))=\bar T(q(h),a)
\]

and

\[
r(h,a)=\bar r(q(h),a).
\]

Prove this for the selected coordinate or state the missing fields.

This establishes dynamics factoring through \(q\). It does not establish posterior equality.

---

## 29. Strategic quotient theorem

State the correct theorem.

Two histories may be merged for a given decision problem if they induce the same:

1. physical coordinate;
2. posterior measure, up to a value-preserving world isomorphism;
3. continuation-policy memory state;
4. utility residue.

More generally, define field-relative strategic equivalence:

\[
h\sim_{\pi,U}h'
\]

iff every allowed continuation action/strategy has the same expected utility under the modeled field and beliefs.

A coarser coordinate is valid only relative to the class of fields and utilities blind to what it drops.

This is the correct home for the “tower of quotients.”

Do not order the tower as if every listed rung has already been proved.

---

## 30. Coordinate versus standard information set

A standard extensive-form information set in a perfect-recall game is defined by observational indistinguishability, not by equal physical residue.

A player remembers:

- its private hand;
- all public actions;
- its own prior actions.

Since public actions are observed by all, the full public history normally remains part of the information-set key.

A path-free coordinate may define a **restricted Markov policy class**, but it may not be the same extensive-form information partition.

Prove any proposed merge preserves perfect recall. Otherwise label it as abstraction.

This point is mandatory before any CFR design.

---

## 31. Seat gauges

Investigate and prove exact seat symmetries.

Candidates:

- simultaneous rotation of all seats;
- bidder-anchored canonicalization;
- team-preserving reflection;
- team-swapping transformations with payoff sign reversal.

Do not assume a 4× address reduction until the full contract, dealer, auction order, leader, viewer, marks, attribution, and utility all commute with the transformation.

Separate:

- play-phase symmetry;
- whole-hand symmetry;
- match symmetry.

---

## 32. Pip relabeling and structural isomorphism

The existing evidence says almost all pip relabelings fail because of:

- higher-end lead choice;
- numeric rank;
- count marks;
- declaration coupling.

State the exact proven symmetry group for the chosen scope.

Do not generalize a measured finite census beyond its enumerated domain without proof.

---

## 33. Higher-order knowledge

The current synthesis says higher-order knowledge “collapses.”

Formalize the claim under explicit assumptions:

- common prior;
- common knowledge of rules;
- common knowledge of policy model;
- all actions public;
- each player privately knows only its own hand;
- perfect recall.

Then decide whether higher-order beliefs are:

1. literally absent;
2. uniquely induced from the common model and first-order private information;
3. still strategically relevant but not independent state variables.

Prefer the precise statement. Prove it or narrow it.

---

# Part VII — Scoring lenses and utilities

## 34. Distinguish utility lenses

Define at least:

### 34.1 Hand points

Declaring-team points or signed point differential.

### 34.2 Contract success

Make/set indicator.

### 34.3 Hand marks

Marks awarded under the bid.

### 34.4 Match-to-seven value

Probability of eventually winning the match or expected match utility given current marks.

The same physical state can induce different optimal actions under different utility lenses.

Thus payoff stripping is valid only under proved affine relationships and fixed scope.

---

## 35. Terminal and transition reward decompositions

For point-differential utility, trick-completion rewards yield a score-free terminal state.

For contract or marks utility, banked points and threshold residue may matter.

Derive minimal sufficient payoff state for each lens.

Do not assume one coordinate is minimal for all utilities.

---

# Part VIII — Required theorem and counterexample ledger

## 36. Minimum theorem set

The final work should attempt formal proofs of at least:

1. Effective-suit membership properties.
2. Follow-if-possible legality characterization.
3. Unique trick winner.
4. Score conservation at 42.
5. Finite graded-DAG property of fixed-deal play.
6. Physical Markov sufficiency of the chosen objective play state.
7. Exactness of the contextual `BEATS` relation.
8. Monotone live-threat/role promotion.
9. Local hand-order gauge and action-value equivariance.
10. Lossless `(P,k)` hidden-cell representation under stated straight-game assumptions.
11. Hall/max-flow feasibility of hidden cells.
12. Exact support-transition/refinement law.
13. Bayes likelihood factorization over public actions.
14. Exponential tilt form.
15. Strategic-state sufficiency of `(coordinate, belief, required policy memory)`.
16. Coordinate-only value insufficiency by counterexample.
17. Declaration as selection/re-marking of one algebra from the auction bundle.
18. Correct contract and marks settlement.
19. Any claimed seat symmetry.
20. Any claimed quotient/congruence.

---

## 37. Required counterexamples or negative results

Provide explicit counterexamples to:

1. context-free domino value;
2. coordinate-only strategic value when path-induced beliefs differ;
3. treating hidden cells as independent;
4. treating global play as literal node annihilation;
5. using only lead-threat roles as a complete play ontology;
6. identifying a physical coordinate with a perfect-recall information set;
7. assuming a four-agent team CFR result has two-player zero-sum guarantees;
8. treating the current pooled declaration features as a lossless tilewise representation.

If a counterexample cannot be constructed inside legal 42, explain whether the claim may actually hold and prove it.

---

## 38. Existing receipts to integrate honestly

The theory should locate, without overstating:

- Atlas engine-transition parity;
- Atlas exact-fiber parity;
- C4 same-coordinate fixed-field value checks;
- role/threat parity and monotonicity tests;
- the measured role-feature regression result;
- the endgame equivalence census;
- the 2↔3 symmetry finding;
- Walt’s exact information-set best response against a deterministic field;
- Hoyt’s low-gap CFR references and their multiplayer/team honesty line;
- the measured prevalence of forced actions;
- the auction decoder’s hand-dependent causal signature.

For every receipt, state exactly which formal claim it supports.

---

# Part IX — Deliverables from Pro

## 39. Primary deliverable

Create a standalone specification, preferably:

`docs/theory/TEXAS_42_NATIVE_FORMALIZATION.md`

or an equivalently authoritative wiki topic.

It should read as a mathematical monograph, not a project update.

Required structure:

1. scope and rule authority;
2. notation;
3. algebra;
4. auction;
5. objective state and transitions;
6. perspective coordinates;
7. location cells and fiber;
8. belief filtering;
9. marked hand-in-situ;
10. action semantics;
11. value and solution concepts;
12. congruences and symmetries;
13. proofs;
14. counterexamples;
15. implementation correspondence;
16. open obligations.

---

## 40. Proof ledger

Create a proof ledger, either as an appendix or separate file, with columns:

| ID | Claim | Exact statement | Assumptions | Status | Proof/receipt | Counterexample risk | Code gate |
|---|---|---|---|---|---|---|---|

No major claim should remain unclassified.

---

## 41. Implementation correspondence

Map each formal object to current code:

- rule authority;
- Atlas algebra;
- coordinate;
- fiber;
- roles;
- narration;
- Walt world enumeration and best response;
- Hoyt subgame/CFR machinery;
- engine auction and marks layers.

Identify code whose name encodes a mathematically misleading claim.

Do not change code merely to match the theory unless requested. First state the mismatch.

---

## 42. Minimality analysis

For each phase and utility, distinguish:

- sufficient coordinate;
- proven minimal coordinate;
- candidate removable fields;
- fields known to carry only evidence;
- fields required only by a policy class;
- fields required for perfect recall.

“Small” is not “minimal.” Require a proof or counterexample for minimality.

---

## 43. Architecture consequences

Only after the formal theory, derive modeling consequences.

At minimum assess:

- permutation-equivariant set encoder for the owned hand;
- contextual node/action head;
- ambient-boundary representation;
- declaration queries over a shared hand;
- belief representation over cells/fiber;
- leave-one-out or post-action residual encoding;
- exact relational planes versus engineered pooled summaries.

These are hypotheses about function class and learning efficiency, not mathematical consequences that guarantee performance.

Specify decisive experiments such as P-ENC, P-DEL, and P-DECL.

---

# Optional Annex — CFR or regret-based implementation outline

## 44. Include this annex only if the foundations are stable

Do not simply say “run CFR on the coordinate.”

First distinguish three different problems.

### 44.1 Fixed-field best response

One seat optimizes an information-set-consistent strategy against a fixed field.

This is Walt’s clean object:

\[
\rho^*
=
\arg\max_\rho
\mathbb E_{\omega\sim\mu}
[
U(\operatorname{terminal}(\omega;\rho,\sigma))
].
\]

The native coordinate/fiber representation can support exact or sampled solution.

### 44.2 Four-agent shared-team regret minimization

There are four decision makers with utilities shared by partners.

This is not automatically a two-player zero-sum game for CFR purposes because teammates do not share private observations and cannot choose a centralized joint action at runtime.

Standard two-player zero-sum CFR convergence and exploitability claims do not transfer automatically.

### 44.3 Team/coordinator reformulation

Investigate whether the game can be converted to a two-team coordinator game using common-information or prescription strategies.

If so:

- define coordinator information;
- define prescriptions from private hands to actions;
- prove behavioral equivalence;
- state the blowup.

If not practical, state why.

---

## 45. CFR information sets

For the full perfect-recall extensive-form game, an actor’s information set is based on:

\[
(\text{private initial hand},\text{public history})
\]

or a proved sufficient perfect-recall statistic.

Do not key regret tables only by a path-free mechanical coordinate unless intentionally solving a Markov-restricted abstraction.

Belief is induced by strategy and history. It is not normally part of the observable information-set key in standard CFR.

---

## 46. Native traversal outline

A valid outline may use:

- chance node for uniform deal;
- auction nodes;
- declaration choice;
- deterministic play transitions;
- exact legal-action masks;
- action handles as remaining owned nodes;
- actor-relative canonicalization;
- forced-action compression;
- structural/permutation-equivariant policy parameterization;
- exact or sampled hidden-world traversal;
- external sampling or outcome sampling;
- exact terminal marks utility;
- independent best-response certification where available.

Explain how the marked-substructure representation changes:

- state construction;
- action indexing;
- information-set hashing;
- feature sharing;
- forced-chain collapse;
- belief/world sampling;
- branch grouping.

Do not claim it removes the fundamental imperfect-information branching.

---

## 47. Regret and certification honesty

Any CFR annex must state:

- which regret notion is minimized;
- whether convergence is proved;
- whether the game is two-player zero-sum, multiplayer general-sum, team zero-sum with decentralized information, or a restricted policy game;
- what a “gap” certifies;
- whether deviations are single-seat or team-joint;
- whether a low single-seat BR gap pins value;
- how approximation and sampling error are bounded.

The current Hoyt honesty line is important: single-seat best-response gaps in the four-seat team game do not automatically establish a unique equilibrium value.

---

# Part X — Final standard

## 48. What success looks like

The finished work should make these sentences precise:

> A physical domino is a stable node identifier, not a stable strategic type.

> Declaration selects a relational algebra in which the node’s suit membership, power, and comparison role are defined.

> A player’s remaining hand is a marked, controllable subset embedded in that ambient algebra and coupled to a delimited hidden complement.

> A play spends one controllable node, relocates it in the global state, selects or responds to a trick context, transforms the residual hand, and generates a public observation.

> The exact mechanical coordinate determines legal physical futures and the compatible-world fiber.

> The public path supplies discretionary evidence that induces a belief measure on that fiber.

> Strategic value is a functional of coordinate, belief, continuation policy, and utility—not a property of a domino and not generally a property of the coordinate alone.

> Independent domino evaluation is therefore not the native decomposition of Texas 42. It is an optional approximation that must earn its validity.

The specification should prove these claims where they are mathematical, delimit them where they depend on assumptions, and refute them where they are too strong.

---

## 49. Do not stop at agreeable language

Push until the formal object either closes or breaks.

Specifically:

- Try to prove losslessness of the cell representation.
- Try to prove the exact strategic sufficiency theorem.
- Construct the coordinate-only-value counterexample.
- Formalize the hand as an ambient marked embedding, not a slogan.
- Determine the exact extensive-form information partition.
- Determine which quotient maps preserve dynamics, belief, value, or only implementation behavior.
- Determine whether “higher-order knowledge collapses” is a theorem and under what common-knowledge assumptions.
- Determine whether a team-coordinator CFR reformulation is exact, and what it costs.
- Separate generic extensive-form facts from discoveries specific to 42.

The goal is a theory that the implementation can be checked against for years.

> **Do not protect the current prose. Protect the game.**
