# The Salvation Complex and Information-Cut Calculus of Walt

## Doom, policy columns, gluing cuts, counted belief, and best-response towers as one finite geometry

**Status:** exploratory mathematical synthesis for intake, adversarial review, and engineering design. This document does not promote any current implementation result merely by restating it.

**Date:** 2026-09-01

**Repository state inspected:** `jasonyandell/texas-42` `main` at `08fe3d2ddca2a17cf8faf633add69d3f83592ec5`, including the counted-belief/anytime proof-state program and the doom census at `eb5a459da8e0aac7f49e27038d00819445b0eb23`.

**Primary current sources:**

- `walt/math/counted_belief_sandwich_v0.1.md`
- `walt/math/anytime_proof_state_score_v0.1.md`
- `walt/FACTOR-BELIEF.md`
- `walt/walt/src/solver/{factor_belief,proof_state,residual,covers,laydown,opening,doom}.rs`
- `walt/probes/factor_belief/{cegar_run1,refine_run1,openingreport_run1,doomreport_run1}.txt`
- their maintained intake companions, rulings, and CI gates

No external literature theorem is used as authority here. Several familiar algorithmic interpretations are mentioned only after the finite statements are defined and proved internally.

---

# 0. Executive result

The doom census did more than test one proposed upper bound. It exposed the next abstraction.

For a fixed root action, field, contract, and hidden world, consider the set of all information-consistent focal policies that make the contract in that world. Call it the world's **salvation set**.

The entire fixed-field Walt problem is encoded by the weighted family of those salvation sets.

From that one object:

- a **doomed world** has an empty salvation set;
- a **perfectly safe world under every focal policy** has the whole policy space as its salvation set;
- a lawful policy's `pmake` is the total belief mass of salvation sets containing that policy;
- exact best response is the policy lying in the greatest total mass of salvation sets;
- signed pivotal geometry compares the incidence columns of two policies;
- a policy grammar restricts the available columns;
- world-revealed play projects every salvation set all the way down to the question “empty or nonempty”;
- a salvation mask is a one-information-state projection of the same set;
- gluing retains more shared policy coordinates and thereby shrinks the upper;
- count and score replace one Boolean threshold by a nested family of salvation sets;
- the best-response tower changes the field and therefore changes the salvation family, but not the mathematics used to price it.

There is an exact combinatorial reformulation.

Let a set of worlds be **jointly saveable** when one information-consistent policy makes in every world in that set. The jointly saveable subsets form a downward-closed finite family: the **salvation complex**.

Its inclusion-minimal nonsaveable subsets are **salvation conflicts**.

Then:

\[
\boxed{
1-Q
=
\text{minimum belief mass of a set of worlds that hits every salvation conflict}.
}
\]

Equivalently, exact best response is a minimum-weight transversal problem over an implicit conflict hypergraph.

This turns the current program into a genuine primal-dual refinement process.

- A lawful executable policy supplies a feasible failure set and raises the `pmake` floor.
- Doom supplies singleton conflicts and lowers the upper.
- Salvation masks and gluing discover higher-order conflicts.
- Counted belief gives exact weights without enumerating complete deals.
- A conflict packing gives a deterministic lower bound on unavoidable failure and therefore a deterministic upper on `pmake`.
- Exactness arrives when the best policy failure mass meets the conflict-derived unavoidable-failure mass.

In matrix language:

> **Policies are columns. Worlds are rows. Counted belief compresses rows. Policy search generates good columns. Gluing generates valid cuts.**

That appears to be the larger shape.

The doom census also gives a crucial diagnostic correction.

At the opening root, it found no certified doom and its structured singleton checks all remained world-saveable. That says the present upper plateau is not being reduced by the current physical-doom producer. It does **not**, by itself, prove that the remaining gap is information-consistency price. The unclaimed mass decomposes into:

\[
\text{physical doom}
+
\text{information-consistency price}
+
\text{incumbent policy gap}.
\]

With physical doom apparently small, the remaining gap is in the last two terms. Only a better lawful policy or an information-aware upper can determine which.

On the enumerable trick-5/trick-6 coordinates, however, the current records give a stronger and surprising signal: combining the per-world doom truth with the exact lawful lower/response values closes all fourteen action coordinates at the world-revealed upper. On those specimens, every individually saveable world is jointly saveable by one lawful policy. The information-consistency price is exactly zero.

That suggests a highly valuable new empirical object:

> **the fusion horizon — how far from the end of a hand the world-revealed upper remains exact.**

If a broad suffix is fusion-free, it can become an exact cut beneath both the opening solver and the best-response tower.

---

# Part I — The implicit Walt matrix

## 1. Fixed object

Fix:

- a finite physical world or scenario set \(\Omega\);
- a rational belief \(\beta\) on \(\Omega\), with \(\beta(\omega)>0\) on its declared support;
- one focal information state and root action \(a\);
- the finite class \(\Pi_a\) of lawful deterministic information-consistent focal continuation policies beginning with \(a\);
- a fixed deterministic field \(\sigma\) for all non-focal seats;
- a terminal declaring-team score
  \[
  S^\sigma_a(\omega,\rho)\in\{0,\ldots,42\};
  \]
- a contract \(c\).

Randomized focal policies need not be added to maximize expected `pmake`: a mixture has value equal to a convex combination of its pure components and therefore cannot exceed its best pure component.

For a stochastic field, replace \(\omega\) by an augmented scenario \(\xi=(\omega,z)\) carrying the persistent random tape. Everything below then applies to \(\Xi\). The current engineering path is deterministic, so the physical-world notation is retained.

## 2. The score matrix and threshold matrix

Define the implicit score matrix

\[
\mathsf S_{\omega,\rho}
=
S^\sigma_a(\omega,\rho).
\]

For contract \(c\), threshold it to the Boolean incidence matrix

\[
\boxed{
\mathsf A^c_{\omega,\rho}
=
\mathbf 1\{\mathsf S_{\omega,\rho}\ge c\}.
}
\]

Rows are worlds. Columns are lawful contingent policies.

The value of one policy is the weighted column sum

\[
V_c(\rho)
=
\sum_{\omega\in\Omega}
\beta(\omega)\mathsf A^c_{\omega,\rho}.
\]

The exact fixed-field root-action value is

\[
\boxed{
Q_a^\sigma(c)
=
\max_{\rho\in\Pi_a}
V_c(\rho).
}
\]

Walt never needs to materialize this matrix. The matrix is the mathematical object that all current evaluators price implicitly.

## 3. Current methods as matrix operations

The current mathematics already occupies recognizable parts of this matrix.

- A frozen policy evaluation prices one column.
- A grammar searches a declared subset of columns.
- Exact information-consistent best response searches all lawful columns.
- Signed pivotal geometry subtracts two columns.
- The pivotal set is the support of that column difference.
- Benefit and hazard are the positive and negative parts.
- Counted belief groups and weights rows.
- Sampling draws rows.
- Doom asks whether a row contains any one.
- A laydown-like universal statement asks whether selected rows contain only ones under the declared quantifiers.
- World-revealed optimization takes the row maximum before the weighted sum.
- Lawful best response takes the weighted sum before the column maximum.

The central noncommutation is

\[
\boxed{
\sum_\omega\beta(\omega)
\max_\rho\mathsf A^c_{\omega,\rho}
\;\ge\;
\max_\rho
\sum_\omega\beta(\omega)\mathsf A^c_{\omega,\rho}.
}
\]

The left side is the world-revealed, or “God,” upper. The right side is Walt's lawful value.

Their difference is the information-consistency price.

---

# Part II — Salvation sets and the salvation complex

## 4. The salvation set of a world

For each world \(\omega\), define

\[
\boxed{
\mathcal S_\omega(c)
=
\{\rho\in\Pi_a:
S^\sigma_a(\omega,\rho)\ge c\}.
}
\]

This is the set of every lawful total policy that makes in that one world.

The world is physically doomed, relative to the root action and declared field, exactly when

\[
\mathcal S_\omega(c)=\varnothing.
\]

The world is safe under every focal policy exactly when

\[
\mathcal S_\omega(c)=\Pi_a.
\]

For a policy \(\rho\),

\[
V_c(\rho)
=
\beta\{\omega:\rho\in\mathcal S_\omega(c)\}.
\]

Thus the best response is the policy of maximum weighted incidence depth:

\[
\boxed{
Q_a^\sigma(c)
=
\max_{\rho\in\Pi_a}
\beta\{\omega:\rho\in\mathcal S_\omega(c)\}.
}
\]

## 5. Joint salvation

A set \(T\subseteq\Omega\) is **jointly saveable** when one policy succeeds in all of it:

\[
T\text{ is jointly saveable}
\iff
\bigcap_{\omega\in T}\mathcal S_\omega(c)\ne\varnothing.
\]

Define

\[
\boxed{
\mathcal K_c^\sigma(a)
=
\left\{
T\subseteq\Omega:
\bigcap_{\omega\in T}\mathcal S_\omega(c)\ne\varnothing
\right\}.
}
\]

The empty set belongs to \(\mathcal K\). If \(T\in\mathcal K\) and \(R\subseteq T\), then \(R\in\mathcal K\). Therefore \(\mathcal K\) is a finite downward-closed family—an abstract simplicial complex on its actually saveable vertices.

Doomed worlds simply fail to appear as vertices: their singleton sets are not faces.

This is the **salvation complex**.

## 6. Maximum-weight face theorem

### Theorem 6.1

\[
\boxed{
Q_a^\sigma(c)
=
\max_{T\in\mathcal K_c^\sigma(a)}
\beta(T).
}
\]

### Proof

Every policy \(\rho\) saves the world set

\[
T_\rho
=
\{\omega:\rho\in\mathcal S_\omega(c)\}.
\]

The same policy witnesses \(T_\rho\in\mathcal K\), and

\[
\beta(T_\rho)=V_c(\rho).
\]

Hence the maximum face weight is at least \(Q\).

Conversely, if \(T\in\mathcal K\), choose a policy \(\rho\) in the defining intersection. It saves every world in \(T\), perhaps more, so

\[
V_c(\rho)\ge\beta(T).
\]

Taking maxima gives equality. ∎

The exact fixed-field problem is therefore:

> Find a maximum-belief-mass face of the salvation complex.

## 7. Common-salvation theorem

Let

\[
D
=
\{\omega:\mathcal S_\omega(c)=\varnothing\}
\]

and

\[
U^\mathrm{God}
=
1-\beta(D).
\]

### Theorem 7.1 — zero information price

Assuming positive belief mass on the declared support,

\[
\boxed{
Q_a^\sigma(c)=U^\mathrm{God}
\iff
\bigcap_{\omega\notin D}
\mathcal S_\omega(c)
\ne\varnothing.
}
\]

### Proof

A policy in the intersection saves every individually saveable world and therefore attains \(U^\mathrm{God}\).

Conversely, a policy attaining \(U^\mathrm{God}\) cannot save a doomed world. To have total success mass equal to all saveable mass, it must save every positive-mass saveable world. Thus it belongs to the intersection. ∎

Call a policy in this intersection a **God-tight policy** at the declared root, field, and contract.

A God-tight policy is not a universal laydown. It is a single lawful policy that realizes every success the world-revealed upper says is physically available against one fixed field.

## 8. Exact three-part failure decomposition

For an executable policy \(\rho\), define:

\[
d_\mathrm{phys}
=
1-U^\mathrm{God}
=
\beta(D),
\]

\[
d_\mathrm{info}
=
U^\mathrm{God}-Q,
\]

\[
d_\mathrm{policy}(\rho)
=
Q-V_c(\rho).
\]

Then

\[
\boxed{
1-V_c(\rho)
=
d_\mathrm{phys}
+
d_\mathrm{info}
+
d_\mathrm{policy}(\rho).
}
\]

The three terms answer different questions.

1. **Physical doom:** how much world mass cannot be saved even with full world knowledge?
2. **Information price:** how much individually saveable mass cannot be saved simultaneously by one blind policy?
3. **Policy gap:** how much value does the current lawful policy leave below the best blind policy?

A zero doom census affects only the first term.

It does not distinguish the second and third.

## 9. What the current doom run says

The current doom producer is soundly one-sided:

- it certifies classes on which every focal continuation fails against the declared deterministic field;
- exact class mass becomes a deterministic upper;
- phantom per-seat states can block certification but cannot manufacture doom;
- refused or survived classes remain unclaimed.

On the current enumerable receipt roots, the per-world doom truth can be compared with the exact lawful records.

The following action values close at the God upper:

| Root | Root actions | Per-world doom implication | Matching lawful value |
|---|---|---:|---:|
| h12-t6 | 4-4, 6-0 | \(6/6\) doomed | \(Q=0\) |
| h10-t6 | 2-2, 3-3 | \(0/19\) doomed | \(Q=1\) |
| h5-t6 | 4-1, 5-2 | \(15/27\) doomed | \(Q=12/27=4/9\) |
| h4-t6 | 0-0 | \(60/90\) doomed | \(Q=30/90=1/3\) |
| h4-t6 | 1-1 | \(12/90\) doomed | \(Q=78/90=13/15\) |
| h8-t5 | 0-0 | \(21/92\) doomed | \(Q=71/92\) |
| h8-t5 | 5-0 | \(28/92\) doomed | \(Q=64/92=16/23\) |
| h8-t5 | 5-3 | \(1/92\) doomed | \(Q=91/92\) |
| h3-t5 | 4-0, 5-0, 6-6 | \(0/200\) doomed | \(Q=1\) |

This is an inference obtained by combining two independent committed record families: the per-world doom truth and the lawful lower/exact response values.

On these fourteen coordinates,

\[
\boxed{
d_\mathrm{info}=0.
}
\]

Every individually saveable world is jointly saveable by one information-consistent policy.

That is a much stronger structural signal than “doom found some failures.”

At the opening root, the priority census certified no doomed mass, two deliberately hostile worlds remained saveable by a world-aware player, and a structured 228-world grid contained no doomed specimen. This is not a proof that the complete opening doom mass is zero. It is strong evidence that the God upper is near one under the declared field and that further doom counting is unlikely to lower the present upper materially.

The remaining opening gap is therefore best described as **nonphysical unresolved mass**:

\[
U^\mathrm{God}-L
=
d_\mathrm{info}
+
d_\mathrm{policy}.
\]

A better policy attacks \(d_\mathrm{policy}\). Gluing and salvation conflicts attack \(d_\mathrm{info}\). Both should be pursued until one side closes the question.

---

# Part III — Salvation conflicts and the exact cut theorem

## 10. Minimal nonsaveable sets

A nonempty set \(C\subseteq\Omega\) is a **salvation conflict** when

\[
\bigcap_{\omega\in C}
\mathcal S_\omega(c)
=
\varnothing.
\]

It is minimal when every proper subset is jointly saveable.

Let

\[
\mathcal H_c^\sigma(a)
\]

be the finite hypergraph whose vertices are worlds and whose hyperedges are the minimal salvation conflicts.

Doomed worlds are exactly the singleton hyperedges.

The higher-order hyperedges are pure information-consistency conflicts: each member world may be individually saveable, but the worlds cannot all be saved by one policy.

## 11. Failure sets hit every conflict

For policy \(\rho\), define its failure set

\[
F_\rho
=
\{\omega:\rho\notin\mathcal S_\omega(c)\}.
\]

Every \(F_\rho\) intersects every conflict hyperedge. Otherwise \(\rho\) would lie in every salvation set of that hyperedge, contradicting empty intersection.

So every policy failure set is a transversal, or hitting set, of \(\mathcal H\).

## 12. Exact minimum-transversal theorem

Let

\[
\tau_\beta(\mathcal H)
=
\min\left\{
\beta(F):
F\subseteq\Omega,\;
F\cap C\ne\varnothing
\text{ for every }C\in\mathcal H
\right\}.
\]

### Theorem 12.1

\[
\boxed{
1-Q_a^\sigma(c)
=
\tau_\beta(\mathcal H_c^\sigma(a)).
}
\]

### Proof

Every policy failure set is a hitting set, so

\[
\tau_\beta(\mathcal H)
\le
\min_\rho\beta(F_\rho)
=
1-Q.
\]

Conversely, let \(F\) hit every minimal conflict and put \(T=\Omega\setminus F\). If \(T\) were not jointly saveable, finiteness would give an inclusion-minimal nonsaveable subset \(C\subseteq T\). That \(C\) is a hyperedge disjoint from \(F\), contradicting the hitting property.

Thus \(T\) is jointly saveable. Some policy succeeds throughout \(T\), so its failure set is contained in \(F\) and has mass at most \(\beta(F)\). Taking the minimum over hitting sets gives

\[
1-Q\le\tau_\beta(\mathcal H).
\]

Hence equality. ∎

This is the exact cut formulation of Walt's Boolean fixed-field objective.

## 13. Doom and information price are the size-one and higher-order cuts

No minimal conflict of size at least two contains a doomed world, because the doomed singleton would already be a smaller conflict.

Let

\[
\mathcal H_{\ge2}
=
\{C\in\mathcal H:|C|\ge2\}.
\]

Then every hitting set must contain every doomed world and must additionally hit the higher-order conflicts among the saveable worlds.

Therefore

\[
\boxed{
1-Q
=
\beta(D)
+
\tau_\beta(\mathcal H_{\ge2}).
}
\]

Consequently,

\[
\boxed{
d_\mathrm{info}
=
U^\mathrm{God}-Q
=
\tau_\beta(\mathcal H_{\ge2}).
}
\]

The information-consistency price is exactly the minimum additional mass that must be sacrificed to hit every non-singleton salvation conflict.

Doom and gluing are not two unrelated upper ideas.

> **Doom discovers one-vertex conflicts. Gluing discovers larger conflicts.**

## 14. Any verified conflict family gives an admissible upper

Suppose Walt has proved only a subfamily

\[
\mathcal C\subseteq\mathcal H.
\]

Let \(\tau_\beta(\mathcal C)\) be its minimum hitting weight. Since every lawful policy failure set hits every member of \(\mathcal C\),

\[
1-Q\ge\tau_\beta(\mathcal C).
\]

Therefore

\[
\boxed{
Q
\le
1-\tau_\beta(\mathcal C).
}
\]

Each new verified conflict can only increase the lower bound on unavoidable failure and lower the `pmake` upper.

The doom upper is the special case in which \(\mathcal C\) contains only the certified singleton conflicts.

## 15. Conflict packing gives a cheap deterministic upper

Computing the exact minimum hitting set of all accumulated conflicts may itself be expensive.

A cheap dual lower bound is enough.

Assign a nonnegative rational \(\lambda_C\) to each verified conflict \(C\), subject to the world-capacity constraints

\[
\sum_{C\ni\omega}\lambda_C
\le
\beta(\omega)
\qquad
(\omega\in\Omega).
\]

Every hitting set \(F\) must hit every conflict. Charging each \(\lambda_C\) to one hit vertex gives

\[
\sum_C\lambda_C
\le
\beta(F).
\]

Thus

\[
\boxed{
Q
\le
1-\sum_C\lambda_C.
}
\]

Pairwise disjoint conflict cores are the simplest packing: each forces at least the minimum world weight in that core to be lost, and disjointness lets those charges add.

At opening scale, individual world weights are tiny. The useful form will usually be a counted structural block certificate or an action-mask mass table, not isolated world pairs.

## 16. The success polytope interpretation

Every lawful policy has a Boolean success vector

\[
z^\rho_\omega
=
\mathsf A^c_{\omega,\rho}.
\]

Define the policy success polytope

\[
\mathcal P
=
\operatorname{conv}
\{z^\rho:\rho\in\Pi_a\}.
\]

Because the objective is linear,

\[
Q
=
\max_{z\in\mathcal P}
\beta\cdot z.
\]

The world-revealed doom relaxation begins with the box

\[
0\le z_\omega\le1
\]

and fixes \(z_\omega=0\) on proved doomed worlds.

A conflict core \(C\) supplies the valid cut

\[
\sum_{\omega\in C}z_\omega
\le
|C|-1.
\]

Known lawful policies supply feasible vertices.

So a proof state may be understood as maintaining:

\[
\boxed{
\mathcal P_t^-
\subseteq
\mathcal P
\subseteq
\mathcal P_t^+.
}
\]

- Add a policy: enlarge the inner approximation and raise the lower bound.
- Add a conflict cut: shrink the outer approximation and lower the upper.
- Refine counted cells: make the objective weights and cut coefficients sharper.
- Stop when the two objective values meet or the declared regret is small enough.

This is the clean column-and-cut form of Walt.

---

# Part IV — Partial information consistency and the gluing lattice

## 17. Policy coordinates

Let \(\mathcal I\) be the finite set of focal information states in the continuation domain. For each \(I\in\mathcal I\), let \(A(I)\) be its legal action set.

A deterministic information-consistent policy is a total assignment

\[
\rho\in
\prod_{I\in\mathcal I}A(I).
\]

Actions assigned to unreachable information states are harmless coordinates; they make the policy space a finite product.

For \(J\subseteq\mathcal I\), let

\[
\pi_J(\rho)
=
\rho|_J
\]

be the projection to the selected information states.

## 18. Projected salvation sets

Define

\[
\boxed{
\mathcal S_{\omega,J}(c)
=
\pi_J(\mathcal S_\omega(c)).
}
\]

An assignment \(\eta\) on \(J\) belongs to this projected set exactly when it has some successful completion in world \(\omega\).

Define the partially glued upper

\[
\boxed{
U_J(c)
=
\max_{\eta\in\prod_{I\in J}A(I)}
\sum_{\omega}
\beta(\omega)
\mathbf1\{\eta\in\mathcal S_{\omega,J}(c)\}.
}
\]

Equivalently,

\[
U_J(c)
=
\max_{\eta}
\mathbb E_\omega
\left[
\max_{\substack{\rho\in\Pi_a\\\rho|_J=\eta}}
\mathbf1\{S(\omega,\rho)\ge c\}
\right].
\]

The states in \(J\) must use one shared assignment across worlds. Every policy coordinate outside \(J\) remains world-specific inside the relaxation.

This is a partial movement of the maximization from inside the expectation to outside it.

## 19. Endpoint and monotonicity theorem

### Theorem 19.1

\[
\boxed{
U_\varnothing
=
U^\mathrm{God},
\qquad
U_{\mathcal I}
=
Q.
}
\]

If

\[
J\subseteq K\subseteq\mathcal I,
\]

then

\[
\boxed{
Q
\le
U_K
\le
U_J
\le
U^\mathrm{God}.
}
\]

### Proof

For \(J=\varnothing\), there is one empty assignment. It has a successful completion in a world exactly when that world's salvation set is nonempty. Hence \(U_\varnothing\) is the doom upper.

For \(J=\mathcal I\), a projected assignment is a complete policy and has no omitted coordinates. The formula becomes exact best response.

If \(J\subseteq K\), every \(K\)-assignment projects to a \(J\)-assignment, and every world admitting a successful completion of the former admits one of the latter. Maximizing over \(K\)-assignments cannot exceed maximizing over \(J\)-assignments. ∎

The family

\[
J\longmapsto U_J
\]

is the **information-consistency lattice**.

## 20. Projected salvation complexes

For each \(J\), define

\[
\mathcal K_J
=
\left\{
T\subseteq\Omega:
\bigcap_{\omega\in T}
\mathcal S_{\omega,J}(c)
\ne\varnothing
\right\}.
\]

Then

\[
\mathcal K_{\mathcal I}
=
\mathcal K
\]

is the lawful salvation complex, while

\[
\mathcal K_\varnothing
\]

is the full simplex on all individually saveable worlds.

Moreover,

\[
J\subseteq K
\implies
\mathcal K_K\subseteq\mathcal K_J.
\]

Every gluing step shrinks the relaxed salvation complex and reveals new nonfaces.

The partially glued upper is the maximum-weight face:

\[
\boxed{
U_J
=
\max_{T\in\mathcal K_J}\beta(T).
}
\]

It also has its own exact minimum-transversal representation through the minimal conflicts of the projected salvation sets.

## 21. Salvation masks

For one information state \(I\), the projected set

\[
\boxed{
M_\omega(I,c)
=
\mathcal S_{\omega,\{I\}}(c)
\subseteq A(I)
}
\]

is the world's salvation mask at \(I\).

The one-glue upper is

\[
\boxed{
U_{\{I\}}(c)
=
\max_{x\in A(I)}
\sum_\omega
\beta(\omega)
\mathbf1\{x\in M_\omega(I,c)\}.
}
\]

In 42 there are at most seven legal focal actions at one information state, so at most

\[
2^7=128
\]

distinct masks.

Doom is the empty mask after all other policy coordinates have been existentially eliminated.

## 22. Score-ceiling tensor

Define the partial-assignment world ceiling

\[
\boxed{
C_{\omega,J}(\eta)
=
\max_{\substack{\rho\in\Pi_a\\\rho|_J=\eta}}
S^\sigma_a(\omega,\rho).
}
\]

Then

\[
\mathcal S_{\omega,J}(c)
=
\{\eta:C_{\omega,J}(\eta)\ge c\}.
\]

This one tensor specializes to all current upper objects.

- \(J=\varnothing\): one world-aware optimistic score ceiling; thresholding gives doom.
- \(J=\{I\}\): an action-indexed score-ceiling vector; thresholding gives salvation masks.
- larger \(J\): joint-action salvation relations.
- \(J=\mathcal I\): the exact score matrix.

This is the score-aware form of gluing.

A class-level implementation may keep an upper envelope

\[
\overline C_{B,J}(\eta)
\ge
\max_{\omega\in B}
C_{\omega,J}(\eta).
\]

Whenever

\[
\overline C_{B,J}(\eta)<c,
\]

assignment \(\eta\) is safely removed for the entire class.

Unproved entries stay possible. That preserves upper validity.

## 23. Interaction: one-state masks are not complete

There is no general theorem that one useful information state must lower the upper.

Consider two binary policy coordinates \(x,y\) and two equally weighted worlds:

\[
\mathcal S_1
=
\{00,11\},
\]

\[
\mathcal S_2
=
\{01,10\}.
\]

Both worlds are individually saveable, so

\[
U_\varnothing=1.
\]

Projecting onto \(x\) alone gives \(\{0,1\}\) in both worlds. The same is true for \(y\). Hence

\[
U_{\{x\}}=U_{\{y\}}=1.
\]

But no complete assignment saves both worlds:

\[
U_{\{x,y\}}
=
Q
=
\frac12.
\]

The conflict is purely joint.

Therefore:

- unary salvation masks are a valuable first rung;
- zero one-glue improvement does not prove zero information price;
- two or more glues may need to be purchased as one macro-refinement.

## 24. No universal greedy law

Let

\[
G(J)=U_\varnothing-U_J
\]

be the information price exposed by gluing \(J\).

The preceding example has

\[
G(\{x\})=G(\{y\})=0,
\qquad
G(\{x,y\})=\frac12.
\]

Thus \(G\) need not have diminishing returns.

Other finite systems have the opposite shape: each singleton glue helps greatly and their joint gain overlaps.

So \(G\) is neither generally submodular nor generally supermodular.

A scheduler that refuses every zero-immediate-gain glue can miss a decisive coalition.

The proof-state repair is the same one already learned for prerequisites:

> Evaluate short gluing coalitions or a closure-aware macro-work item, not only isolated one-step gain.

## 25. Gluing rank

Define the exact gluing rank

\[
\boxed{
r_0
=
\min\{|J|:U_J=Q\}.
}
\]

For tolerance \(\varepsilon\),

\[
\boxed{
r_\varepsilon
=
\min\{|J|:U_J-Q\le\varepsilon\}.
}
\]

Cardinality is only a first measure. A cost-weighted rank should charge each information state by:

- number of represented world or hand cells;
- action-domain size;
- score-ceiling cost;
- field materialization cost;
- and interaction with existing cuts.

The tractability hypothesis is not that the full policy has few information states.

It is:

> **The information-consistency price may be carried by a small or low-cost gluing set.**

## 26. An optimizer disagreement is not yet a cut

Suppose a world-aware solver returns one successful policy for each world and those selected policies disagree at an information state.

That is a candidate witness, not a proof of incompatibility.

Example:

\[
\mathcal S_1=\{00,01\},
\qquad
\mathcal S_2=\{00,10\}.
\]

A solver might return \(01\) for the first world and \(10\) for the second. They disagree. But the common policy \(00\) saves both worlds.

A valid gluing cut must use:

- complete salvation masks;
- safe supersets whose intersection is proved empty;
- exact score ceilings;
- or another proof that no alternate successful completion restores compatibility.

Arbitrary chosen argmax actions cannot safely lower an upper.

---

# Part V — The inner/outer policy-language sandwich

## 27. One language, approximated from both sides

Let \(\Pi\) be the exact lawful policy language.

A grammar or candidate library gives an inner language

\[
\Pi^-_t\subseteq\Pi.
\]

A partial-gluing relaxation gives an outer language

\[
\Pi\subseteq\Pi^+_t.
\]

Define

\[
L_t
=
\max_{\rho\in\Pi^-_t}V_c(\rho),
\]

\[
U_t
=
\max_{\rho\in\Pi^+_t}V_c(\rho).
\]

Then

\[
\boxed{
L_t\le Q\le U_t.
}
\]

- Add a lawful policy or grammar action: enlarge \(\Pi^-_t\), so \(L_t\) rises.
- Add a gluing constraint: shrink \(\Pi^+_t\), so \(U_t\) falls.
- Refine world cells: sharpen evaluation of both.

The lower and upper campaigns are two approximations to the same policy language.

## 28. The failure-coordinate form

Let

\[
\tau^\star=1-Q
\]

be the minimum achievable failure mass.

A lawful executable policy \(\rho\) supplies an achievable failure

\[
\overline\tau_\rho
=
1-V_c(\rho)
\]

with

\[
\tau^\star\le\overline\tau_\rho.
\]

Verified conflict cuts supply an unavoidable-failure lower bound

\[
\underline\tau_t
\le\tau^\star.
\]

Therefore

\[
\boxed{
\underline\tau_t
\le
\tau^\star
\le
\overline\tau_t,
}
\]

where \(\overline\tau_t\) is the smallest failure mass among current executable policies.

The certified regret is the same gap in the opposite coordinates:

\[
\boxed{
U_t-L_t
=
\overline\tau_t-\underline\tau_t.
}
\]

This is a particularly natural controller view.

- Better policy columns lower \(\overline\tau_t\).
- Doom and gluing cuts raise \(\underline\tau_t\).
- Exactness arrives when they meet.

## 29. Two-oracle refinement

A strong next solver can alternate two adversarially useful queries.

### Primal query — repair the policy

Given incumbent \(\rho\), find a world or counted class that:

1. is individually saveable under the current upper relaxation;
2. is failed by \(\rho\);
3. has enough mass or contract relevance to matter.

Use its successful continuations to:

- add a policy column;
- enlarge the grammar;
- split a policy cylinder;
- or modify the executable policy at a lawful information state.

This attacks the policy gap.

### Dual query — prove incompatibility

Given the current relaxed upper, find a set of worlds or counted classes whose successful completions cannot be made information-consistent.

Use salvation masks or joint score ceilings to prove a conflict and add a cut.

This attacks the information price.

The two queries race.

- If a policy reaches the God upper, the root is God-tight.
- If a conflict cut lowers the upper, positive information price has been proved.
- If they meet between those extremes, exactness has been reached without resolving every row or every policy coordinate.

## 30. Proof-state interpretation

The current append-only proof state already has the right semantic shape.

It should eventually be able to carry:

```text
PolicyColumn {
    policy_id,
    field_id,
    contract,
    exact_or_delta_value,
    executable,
    score_profile_or_envelope
}

DoomVertex {
    world_or_cell_id,
    exact_mass,
    universal_failure_proof
}

SalvationProjection {
    glue_set_id,
    world_or_cell_id,
    possible_assignment_mask,
    exact_mass_or_residual_interval,
    proof
}

ConflictCut {
    glue_set_id,
    participating_worlds_or_blocks,
    empty_intersection_proof,
    field_id,
    contract
}

ConflictPacking {
    cut_ids,
    rational_charges,
    unavoidable_failure_lower
}

UpperFromCuts {
    unavoidable_failure_lower,
    pmake_upper
}
```

Closure takes:

- the maximum of all valid executable/inner-language lowers;
- the minimum of sampled, doom, gluing, residual, count, and field-transfer uppers;
- or, equivalently, the tightest achievable and unavoidable failure bounds.

The controller does not need one privileged upper producer.

---

# Part VI — Count and the score filtration

## 31. Threshold-indexed salvation complexes

For a fixed field behavior and root action,

\[
c_1\le c_2
\implies
\mathcal S_\omega(c_2)
\subseteq
\mathcal S_\omega(c_1).
\]

Therefore

\[
\boxed{
\mathcal K_{c_2}
\subseteq
\mathcal K_{c_1}.
}
\]

The 43 contracts define a nested score filtration:

\[
\mathcal K_{42}
\subseteq
\mathcal K_{41}
\subseteq
\cdots
\subseteq
\mathcal K_0.
\]

Raising the bid can:

- turn saveable worlds into doomed vertices;
- create new higher-order conflicts;
- increase the minimum transversal mass;
- and reduce the best-response value.

### Field-dependence boundary

This nesting assumes the field mapping itself is held fixed while the score threshold changes.

If a modeled field reads the bid and changes its actions, then changing \(c\) also changes \(\sigma\). The resulting complexes belong to different field identities and need not be nested without a coupling proof.

The current score-profile machinery already observes this boundary.

## 32. The 42 score signature

In Straight 42,

\[
S
=
T+5N_5+10N_{10},
\]

with:

\[
0\le T\le7,
\quad
0\le N_5\le3,
\quad
0\le N_{10}\le2.
\]

The coarse signature has only

\[
8\cdot4\cdot3=96
\]

states.

The exact count-mask signature has at most

\[
8\cdot2^5=256
\]

states.

A score or count profile is therefore tiny relative to the world or policy space.

## 33. Action-indexed score envelopes

At one focal information cut \(I\), let \(B\) be an exact-mass world or hand cell. For each legal action \(x\), maintain

\[
\ell_B(x)
\le
\inf_{\omega\in B}
\inf_{\rho:\rho(I)=x}
S(\omega,\rho),
\]

and

\[
u_B(x)
\ge
\sup_{\omega\in B}
\sup_{\rho:\rho(I)=x}
S(\omega,\rho),
\]

under the declared remaining relaxation.

Then:

- \(u_B(x)<c\): action \(x\) is impossible salvation throughout \(B\);
- \(\ell_B(x)\ge c\): action \(x\) is a certain make throughout \(B\);
- \(\ell_B(x)<c\le u_B(x)\): contract-relevant uncertainty remains;
- all \(u_B(x)<c\): \(B\) is doomed;
- all \(\ell_B(x)\ge c\): every action is safe on \(B\) under the declared suffix quantifiers.

The possible salvation mask is

\[
\overline M_B(I,c)
=
\{x:u_B(x)\ge c\}.
\]

It is a safe superset of every true world mask inside \(B\).

## 34. Doom is the empty action-ceiling vector

The current doom walk checks whether every focal escape fails.

The same recursion can expose action-conditioned results:

1. begin with every legal action in the possible mask;
2. for action \(x\), run or reuse a universal-failure proof conditioned on \(x\);
3. if every represented world and every remaining focal completion fails, clear bit \(x\);
4. leave an unproved action present;
5. when all bits are cleared, the existing doom conclusion is recovered.

Thus:

> **Doom is not a separate producer from salvation masks. It is the empty-mask endpoint of an action-indexed ceiling producer.**

A score-ceiling version is even more reusable: each action stores the greatest contract not yet ruled out.

## 35. Why the F tail may simplify after projection

Slice F found that exact reproduction of the sampled \(\sigma_0\) action map eventually fragments to one class per acting hand.

That does not imply the salvation-mask map fragments.

Different field actions may:

- lead to the same optimistic score ceiling;
- preserve the same set of rescuing focal actions;
- differ only on worlds already safely above or below the contract;
- or create score differences too small to cross the current bid.

The next abstraction should therefore measure, at each F refinement stage:

- exact field-action mass;
- exact or possible salvation-mask mass;
- contract-sensitive residual mass;
- score-envelope width;
- conflict-cut mass;
- and resulting root upper.

The relevant falsifier is not “action classes reach singletons.”

It is:

> **The contract-sensitive projected salvation relation also approaches singleton hands before producing useful cuts.**

---

# Part VII — Fusion-free suffixes and information cuts

## 36. God-tight nodes

A belief node \(B\) is **God-tight** for contract \(c\) when

\[
Q(B,c)
=
U^\mathrm{God}(B,c).
\]

By Theorem 7.1, this holds exactly when one information-consistent policy saves every individually saveable world represented by \(B\).

A machine receipt can establish God-tightness by producing:

1. a deterministic doom upper \(U^\mathrm{God}\);
2. an executable policy lower \(L_\rho\);
3. exact identity of root, field, contract, and belief;
4. equality
   \[
   L_\rho=U^\mathrm{God}.
   \]

No full unrestricted best-response value is required.

## 37. The late-root signal

The current trick-5/trick-6 action coordinates are all God-tight in the inspected exact corpus.

This suggests—without proving—a **fusion-free suffix hypothesis**:

> Beyond some game-dependent horizon, physical doom may be the only source of unavoidable failure, and one lawful policy may realize every saveable world.

If true on a broad reachable domain, the expensive information-consistency problem is concentrated before that horizon.

## 38. Fusion horizon

Define the God gap

\[
\Phi(B,c)
=
U^\mathrm{God}(B,c)-Q(B,c).
\]

A grade or trick depth is fusion-free on a declared corpus when

\[
\Phi(B,c)=0
\]

for every tested root-action coordinate.

The **fusion horizon** is the earliest depth beyond which this remains true on the declared domain.

This is an empirical object first. A theorem should be proposed only after adversarial counterexample search.

## 39. Fusion-cut substitution

Let \(\mathcal F\) be a finite public-belief frontier intersecting every continuation path below a root.

Suppose each \(B\in\mathcal F\) carries:

- an exact continuation value;
- an executable continuation policy attaining it;
- compatible actions whenever two frontier records denote the same focal information state.

Then replacing every suffix below \(\mathcal F\) by those values and policy continuations preserves the exact root value.

This is ordinary finite-horizon Bellman substitution, but the source of the exact leaf values matters:

> God-tightness can make the cut leaves exact without a full information-consistent solve below the cut.

A broad God-tight suffix would be a powerful exact backend for:

- early root solving;
- policy extraction;
- repeated belief experiments;
- and nested best-response fields.

## 40. A cut-oriented experiment

Run an exact **God-gap census** by depth:

\[
(U^\mathrm{God},Q,\Phi)
\]

for every affordable root-action coordinate.

For every zero-gap coordinate, extract and persist a God-tight policy.

For every positive-gap coordinate, retain:

- the earliest focal information state at which worldwise successful completions become incompatible;
- exact salvation masks where affordable;
- minimal joint glue sets found;
- count and first-split motifs.

The primary output is not only a histogram of \(\Phi\).

It is a candidate public-history frontier at which most or all descendants are God-tight.

---

# Part VIII — The best-response tower as dynamics on salvation complexes

## 41. Field-indexed complexes

Let the base field be Dice, \(F_{-1}=D\), and define the named tower

\[
F_0=\operatorname{BR}(D),
\]

\[
F_1=\operatorname{BR}(F_0),
\]

\[
F_2=\operatorname{BR}(F_1),
\]

and so on, under a declared deterministic selector.

Every field \(F_k\) induces its own score matrix, salvation sets, salvation complex, and conflict hypergraph:

\[
\mathsf S^{(k)},
\quad
\mathcal S_\omega^{(k)}(c),
\quad
\mathcal K_c^{(k)},
\quad
\mathcal H_c^{(k)}.
\]

The tower does not change the rules or the physical fiber. It changes which non-focal actions occur, and therefore changes the salvation geometry.

There is no monotonicity in \(k\).

A higher response level may increase, decrease, or leave unchanged:

- doom mass;
- information price;
- exact best-response value;
- or the selected root action.

## 42. Best response as minimum failure transversal

For fixed predecessor field \(F_k\),

\[
1-Q^{(k)}
=
\tau_\beta(\mathcal H^{(k)}).
\]

A best-response policy \(F_{k+1}\) has a failure set attaining this minimum.

Thus the response tower can be written conceptually as

\[
\boxed{
F_{k+1}
\in
\operatorname{MinFailurePolicy}
\bigl(\mathcal H(F_k)\bigr).
}
\]

The same column-and-cut machinery used to lower current uppers is the machinery needed to construct every later tower level.

The upper problem and the tower problem are not separate programs.

## 43. Why project level 2 is the first reciprocal-thinking partner level

Under the current indexing:

- \(F_0\) models every other seat as Dice;
- \(F_1\) models its partner as an \(F_0\) thinker, but that thinker models the focal seat as Dice;
- \(F_2\) models its partner as an \(F_1\) thinker, and that \(F_1\) partner models the focal seat as an \(F_0\) thinker.

Therefore project level 2 is the first named level at which the modeled partner treats the focal player as a thinking responder rather than Dice.

That is a representational fact, not a claim that \(F_2\) is always stronger or equilibrium-like.

## 44. Finite tower and cycles

A deterministic best-response selector on a finite policy space defines a finite map. Iterating it is eventually periodic.

No convergence assumption is needed to construct the finite named tower

\[
D,F_0,F_1,F_2.
\]

Beyond that, record:

- construction identity;
- exact behavior digest on a declared dependency-closed domain;
- root-action recurrence;
- value recurrence;
- and full behavioral cycles.

Tie-inertial exact selection remains attractive:

> retain the predecessor action when it is among the exact maximizers; otherwise use the canonical tie rule.

This removes behavior churn unsupported by value differences while remaining an exact best response.

## 45. Reuse across the tower

Facts fall into three types.

### Field-independent

- physical fiber counts;
- legal-play structure;
- count totals;
- structural score conservation;
- information-state identity;
- some policy grammar and consequence coordinates.

### Field-relative but transportable

- fixed-policy score profiles;
- doom and salvation masks;
- conflict cuts;
- God-tight suffix policies;
- directional field-swap bounds.

These may transfer only through explicit coupling or equality proofs.

### Field-specific

- modeled actions;
- exact salvation sets;
- best-response values;
- extracted policies;
- conflict hypergraphs.

A higher-level proof state should start from transported valid intervals, not from \([0,1]\), but every imported fact must retain both field identities and its coupling proof.

## 46. Tower targeting through the fusion horizon

If late suffixes are God-tight under \(F_0,F_1,F_2\), then nested best-response construction need not recursively invoke a full mind stack to terminal.

Each level may:

1. search only above the fusion cut;
2. use exact compiled suffix values and policies below it;
3. invoke the richer predecessor field only on wake-relevant states;
4. inherit field-swap bounds elsewhere.

This is the most plausible route by which a three-deep tower becomes practical without becoming three nested full-game solves.

---

# Part IX — Engineering program

## 47. Immediate ruling

Do not broaden the doom census indiscriminately at the opening root.

Its endgame performance is valuable and its opening negative is informative. Preserve it as:

- a deterministic singleton-conflict producer;
- an exact God-upper ground truth on enumerable roots;
- a suffix-candidate detector;
- and the base case of salvation-mask generation.

The next upper work should be information-consistency-aware.

## 48. Slice U0 — God-tightness and fusion-horizon census

### Build

For every affordable root-action coordinate:

1. compute exact per-world doom truth or a certified doom upper;
2. compute exact lawful \(Q\), or combine a lower policy with the doom upper;
3. record
   \[
   d_\mathrm{phys},
   \quad
   d_\mathrm{info},
   \quad
   d_\mathrm{policy};
   \]
4. when lower equals God upper, extract and persist a God-tight policy;
5. stratify by trick, grade, contract, trump structure, count state, and field level.

### Required result types

```text
GodUpper
GodTightPolicy
PositiveGodGap
UnknownGodGap
```

A zero certified doom mass with no exact \(Q\) is `UnknownGodGap`, not `PositiveGodGap`.

### Primary question

Where does the first genuine information-consistency price appear?

## 49. Slice U1 — action-ceiling and salvation-mask producer

Extend the universal failure walk so one focal information state remains uneliminated.

For an exact or counted class \(B\), produce a possible action mask:

```text
PossibleSalvationMask {
    root_id,
    field_id,
    contract,
    info_key,
    cell_id,
    exact_mass,
    possible_actions,
    cleared_actions: [(action, universal_failure_proof)],
    residual
}
```

Rules:

- a bit is cleared only by a universal-failure proof;
- an unexamined or refused bit remains present;
- empty mask converts to the existing doom fact;
- masks are merged only under exact cell-mass and identity rules.

### One-state upper

At a focal belief node reached by every represented world,

\[
U_I
=
\max_x
\sum_B
\mu(B)
\mathbf1\{x\in\overline M_B\}
+
\text{declared residual allowance}.
\]

### Gates

- empty-mask mass equals doom mass;
- exact singleton masks equal explicit world-aware action checks;
- possible masks contain exact masks;
- \(Q\le U_I\le U^\mathrm{God}\);
- unresolved actions can only loosen, never tighten, the upper;
- arbitrary selected worldwise optimizers are rejected as mask proofs.

## 50. Slice U2 — conflict ledger and packing upper

Persist verified conflicts.

Start with:

- singleton doom conflicts;
- exact disjoint unary masks;
- small exact joint-mask conflicts;
- counted block conflicts only when the proof establishes a valid mass loss, not merely an empty intersection of coarse labels.

Implement two deterministic upper producers.

### Disjoint packing

Select pairwise vertex- or block-disjoint conflicts and add their mandatory failure charges.

### Rational fractional packing

Maintain \(\lambda_C\ge0\) under exact capacity constraints and install

\[
U=1-\sum_C\lambda_C.
\]

No floating-point optimizer is required for the first slice. Greedy exact-rational packing is a valid lower bound even when not optimal.

### Gates

- every policy failure set hits every persisted conflict on exact fixtures;
- packing never exceeds the exact minimum failure;
- adding a conflict never raises the upper;
- complete conflict enumeration reproduces \(1-Q\) on tiny finite systems.

## 51. Slice U3 — gluing CEGAR

Maintain a selected information-state set \(J\) and upper \(U_J\).

Loop:

1. solve or bound the current partial-glue relaxation;
2. inspect its successful world/cell completions;
3. search for a valid salvation-projection conflict;
4. reject mere disagreement of arbitrarily selected optimizers;
5. add one information state or a short gluing coalition;
6. recompute the upper;
7. persist the new conflict cuts and value drop.

Rank by:

\[
\frac{
\text{maximum possible reduction in certified regret or survivor debt}
}{
\text{declared cost}
}.
\]

Because one-state gains may be zero while a pair is decisive, permit short coalition macro-items.

## 52. Slice U4 — fusion cut

Using the God-tightness census:

1. choose a public-history frontier;
2. attach exact God-tight suffix values and executable policies;
3. verify information-key compatibility across frontier nodes;
4. replace suffix recursion by these exact leaves;
5. compare value, extracted policy, and proof state with the full authority on affordable roots.

The output is an exact reusable suffix oracle, not a heuristic.

## 53. Slice T0 — small exact tower laboratory

On dependency-closed late-game carriers:

1. construct \(D,F_0,F_1,F_2,F_3,\ldots\);
2. use exact best responses and both canonical and inertial tie selectors;
3. record behavior digests and cycles;
4. compute doom, God gap, salvation-mask, and field-swap coordinates at each level;
5. locate partner and opponent first splits separately.

The practical target is \(F_2\). Additional levels diagnose dynamics.

## 54. Slice T1 — lazy practical \(F_2\)

Use one generic `BestResponseField(parent_field_id, responder_spec)`.

At each newly encountered information state:

1. begin with the predecessor action as executable incumbent;
2. import valid field-transfer bounds;
3. consult God-tight suffixes;
4. run proof-state refinement under deterministic work budget;
5. freeze the selected action and quality record;
6. keep fallback and regret explicit.

No separate copied Level2 implementation should be created.

## 55. Suggested shared data model

```rust
struct SalvationContext {
    root_id: RootId,
    field_id: FieldId,
    contract: u8,
    root_action: Domino,
}

struct GodUpper {
    context: SalvationContext,
    doomed_mass: BigUint,
    fiber_mass: BigUint,
    proof: DoomProof,
}

struct PossibleMask {
    context: SalvationContext,
    info_key: InfoKey,
    cell_id: CellId,
    mass: BigUint,
    possible: DominoSet,
    cleared: Vec<(Domino, FailureProofId)>,
}

struct ConflictCore {
    context: SalvationContext,
    glue_set: Vec<InfoKey>,
    support: ConflictSupport,
    proof: EmptyProjectionIntersectionProof,
}

struct ConflictPacking {
    context: SalvationContext,
    charges: Vec<(ConflictCoreId, Rational)>,
    unavoidable_failure_lower: Rational,
}

struct GodTightPolicy {
    context: SalvationContext,
    policy_id: PolicyId,
    value: Rational,
    god_upper: Rational,
    equality_receipt: ReceiptId,
}

struct GlueState {
    context: SalvationContext,
    glued: Vec<InfoKey>,
    upper: Rational,
    conflict_ledger: Vec<ConflictCoreId>,
}
```

## 56. Current-authority reuse

Reuse current Walt for:

- exact fibers and factor beliefs;
- score profiles;
- deterministic fields;
- frozen policies and argmax extraction;
- doom certification;
- residual Bellman intervals;
- exact grammar and unrestricted response;
- risk and provenance;
- complete-world and bundled parity.

The new work is a producer and proof-state extension, not another rules engine.

---

# Part X — Experiments, falsifiers, and proof obligations

## 57. Core experiment matrix

| Experiment | Primary question | Success signal | Falsifier |
|---|---|---|---|
| God-gap census | Is the late suffix fusion-free? | broad region with \(U^\mathrm{God}=Q\) and extracted policies | positive gaps common at every depth |
| Unary masks | Does one information state price meaningful conflict mass? | \(U_I<U^\mathrm{God}\) on real roots | every unary mask upper remains God-vacuous |
| Joint masks | Are conflicts low-order? | small \(J\) captures most God gap | gluing rank approaches full policy dimension |
| Conflict packing | Can cuts lower uppers without exact Q? | packed unavoidable-failure mass grows cheaply | cuts overlap so heavily that packing stays near doom |
| Score masks | Does action fragmentation collapse after contract projection? | few score/mask classes carry most mass | projected masks fragment like exact actions |
| Fusion cut | Can a God-tight suffix replace deep recursion? | exact parity with large wall reduction | suffix compatibility or gap fails broadly |
| Tower laboratory | Is \(F_2\) stable and interpretable? | localized field wake-ups, manageable cycles, suffix reuse | widespread churn or nested cost explosion |
| Two-oracle solver | Do columns and cuts meet early? | certified regret closes before full solve | both policy and conflict fronts remain vacuous |

## 58. Measurements

Per root action and field level, record:

- physical fiber mass;
- exact or bounded God upper;
- executable lower;
- exact \(Q\) where affordable;
- physical doom;
- information price;
- policy gap;
- salvation-mask histogram;
- gluing set and gluing rank;
- conflict-core count and order;
- packed unavoidable-failure mass;
- score-ceiling and count signature;
- God-tight suffix status;
- root survivor set;
- certified regret;
- work by producer.

The central plots are:

\[
\text{unavoidable-failure lower}
\quad\text{and}\quad
\text{achievable-failure upper}
\]

against cumulative cost, and

\[
U_J-Q
\]

against gluing cost.

## 59. Mathematical obligations

**SC-O1 — Incidence formulation.** Formalize \(Q\) as the maximum weighted column depth.

**SC-O2 — Salvation complex.** Prove downward closure and the maximum-weight-face theorem.

**SC-O3 — Common salvation.** Prove God-tightness iff all nonempty salvation sets have common intersection.

**SC-O4 — Failure decomposition.** Prove physical doom plus information price plus policy gap.

**SC-O5 — Minimal conflict theorem.** Prove every policy failure set hits every minimal conflict.

**SC-O6 — Exact transversal theorem.** Prove \(1-Q=\tau_\beta(\mathcal H)\).

**SC-O7 — Doom/information split.** Prove singleton doom mass plus higher-order transversal mass.

**SC-O8 — Partial conflict upper.** A verified conflict subfamily gives \(Q\le1-\tau\).

**SC-O9 — Conflict packing.** Capacity-feasible rational charges lower-bound every hitting set.

**SC-O10 — Partial-glue upper.** Prove the projected-salvation formula, endpoints, and monotonicity.

**SC-O11 — Projected conflict theorem.** Every \(U_J\) equals one minus the minimum transversal of the projected conflict hypergraph.

**SC-O12 — No greedy law.** Preserve explicit finite counterexamples to submodularity and supermodularity.

**SC-O13 — Mask soundness.** Possible-mask supersets give admissible uppers; arbitrary selected optimizer actions do not.

**SC-O14 — Score filtration.** Prove threshold nesting under fixed field identity and state the bid-dependent-field nonimplication.

**SC-O15 — Fusion-cut substitution.** Prove exact frontier replacement under value and information-key compatibility.

**SC-O16 — Tower periodicity.** A deterministic selector on a finite field-policy space is eventually periodic.

## 60. Lean order

The generic finite set-system layer is an attractive next formalization tranche because it is independent of Texas 42 mechanics.

```text
Walt/Salvation/Incidence.lean
Walt/Salvation/Complex.lean
Walt/Salvation/Conflict.lean
Walt/Salvation/Transversal.lean
Walt/Salvation/Projection.lean
Walt/Salvation/ScoreFiltration.lean
Walt/Salvation/Tower.lean
Texas42/WaltSalvationInstance.lean
```

The first four files require only finite types, finite sets, rational weights, maxima, and intersections. They do not require martingale or conditional-expectation infrastructure.

The doom, pivotal, grammar, and gluing theorems can then be instantiated as corollaries of one generic incidence relation.

---

# 61. Final synthesis

The current project can be summarized by one implicit object:

\[
\boxed{
\mathsf A^c_{\omega,\rho}
=
\mathbf1\{
\text{policy }\rho\text{ makes contract }c
\text{ in world }\omega
\}.
}
\]

Everything else is a disciplined way of avoiding materializing it.

- Counted belief compresses its rows.
- Policy extraction finds strong columns.
- Pivotal geometry compares columns.
- Doom finds all-zero rows.
- Score profiles retain the integer matrix before thresholding.
- Salvation masks project columns onto selected information-state coordinates.
- Gluing moves maximization outside the world expectation.
- Conflict cuts record world sets no one column can cover.
- The proof state keeps inner columns and outer cuts.
- The best-response tower changes the matrix and reruns the same mathematics.

The most exact compact statement is:

\[
\boxed{
1-Q
=
\text{minimum weighted transversal of the salvation-conflict hypergraph}.
}
\]

That makes the emerging solver a column-and-cut system.

- The current policy is a feasible transversal.
- Doom contributes mandatory singleton cuts.
- Salvation conflicts contribute higher-order cuts.
- Count gives their exact weight.
- Better policies lower the achievable failure ceiling.
- Better cuts raise the unavoidable failure floor.
- Certified regret is the distance between them.

The doom census suggests two very different regimes.

- In the current late-game corpus, the God upper is already exact: all saveable worlds share a common lawful salvation policy.
- At the opening, physical doom appears unable to explain the upper plateau, but the current evidence does not yet distinguish a missing common policy from a genuine information conflict.

The correct next experiment is therefore not “glue everything.”

It is a two-sided race:

\[
\boxed{
\text{construct a God-tight policy}
\quad\text{versus}\quad
\text{exhibit a salvation conflict}.
}
\]

Whichever lands first tells Walt what kind of difficulty the root actually contains.

And the most promising large structural hypothesis is now precise:

> **The full game may have a wide fusion-free suffix and a low-order information-conflict prefix.**

If that is true, exact counted suffixes, a small number of gluing cuts, and a shallow best-response tower may be enough to produce the fast, strong, gloriously boring base player the project is aiming for.

If it is false, the proof state still remains correct. It will tell us whether the obstruction is:

- physical doom;
- policy quality;
- high-order information conflict;
- score-sensitive count structure;
- or a changing field model.

That is the luxury the mathematics has created.
