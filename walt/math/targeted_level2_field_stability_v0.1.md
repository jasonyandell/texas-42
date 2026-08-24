# HANDOFF — Targeted Level-2 Field-Swap Geometry for Unified Walt

## First-disagreement localization, field-stability bounds, interpretable corrections, and cycle detection

**Status:** EXPLORATORY mathematical proposal. Intended for verbatim intake under the project’s `walt/math/` convention. Nothing is promoted by this document’s existence.

**Date:** 2026-08-24

**Repository snapshot reviewed:** `jasonyandell/texas-42` main at `4231cb248e21aaea809bd188f79ffafedba32123`.

**Prerequisite:** `HANDOFF-walt-calculated-evidence-v0.1.md` is assumed to have been intaken, adjudicated, and implemented on the correctness path. In particular, this document assumes:

- one canonical exact outer fiber;
- exactly uniform indexed world streams;
- frozen and content-addressed policies;
- disjoint discovery and evaluation streams;
- anytime-valid exact-rational evidence;
- explicit risk ledgers;
- exact-fiber escalation;
- mechanically distinct exact, probabilistically settled, unresolved, and heuristic results.

This document extends that foundation. It does not restate or replace it.

**Current project basis:** one unified `walt` crate with the authoritative `rules`, `kernel`, `geom`, `strat`, `spec`, `carrier`, and `solver` modules; a thin `walt-wasm`; and the separate GPU boundary. The exact kernel and the live player are now adjacent. No new Walt implementation family is authorized by this document.

**Sources read as current exploratory project documents:** `SCENARIO-PLAYER.md`, `TILT-AUDIT.md`, `LEVEL2-PROBE.md`, `math/signed_pivotal_geometry_v0.1.md` through its maintained intake companion and SP-A1..SP-A12, and the calculated-evidence handoff named above.

**Vocabulary:** the SP-A vocabulary governs: **frozen policy**, **pivotal mass** \(q\), **tilt** \(\tau\), **gap** \(g\), **pivotal cover**, and **pivotal win share**. A field model is named by an immutable **FieldId**. A probabilistic result is a **\(\delta\)-settlement**. An exhaustive result is **exact**. The word *certificate* remains reserved elsewhere and is not used for the objects introduced here.

---

## 0. Executive conclusion

The next step is **not** to run every Walt decision under a level-1 field and call the result level 2.

The correct next step is to measure and exploit the fact that two field models are identical until they actually choose different public actions.

Let:

- \(\sigma_0\) be the current level-0 field used by level-1 Walt;
- \(\sigma_1\) be the level-1 field against which level-2 Walt would best-respond.

For a fixed focal policy and a fixed physical world, couple the two executions from the same root. Until some non-focal seat reaches an information state where

\[
\sigma_0(J)\ne \sigma_1(J),
\]

the two public histories are identical, the focal policy receives identical observations, every focal action is identical, and the terminal outcomes cannot yet differ.

Therefore:

> **Only worlds and branches that reach a field-disagreement state can carry any level-2 correction.**

That gives a targeted level-2 program with four layers.

1. **Solve the level-1 root correctly.**  
   Use calculated evidence or exact-fiber evaluation. No magic \(n\).

2. **Bound exposure to the field-disagreement frontier.**  
   For each legal root action, determine how much belief mass could reach a state where \(\sigma_0\) and \(\sigma_1\) act differently.

3. **Prove field stability wherever the level-1 margin dominates the maximum possible correction.**  
   Those roots need no level-2 solve.

4. **Run level-2 optimization only on the actions and branches that remain capable of changing the decision.**

For an optimized root action \(a\), let \(Q_a^{(f)}\) be its value against field \(\sigma_f\), and let \(R_a\) be the greatest probability—over all information-consistent continuations beginning with \(a\)—of ever reaching a field disagreement. Then:

\[
\boxed{
\left|Q_a^{(1)}-Q_a^{(0)}\right|\le R_a.
}
\]

Consequently, if \(a\) is the level-1 winner and

\[
\boxed{
Q_a^{(0)}-Q_b^{(0)}>R_a+R_b
}
\]

for every rival \(b\), then \(a\) remains optimal under the level-1 field. No full level-2 search is needed.

This is the central targeting theorem.

The best-response cycling concern is real in principle but is **not yet a blocker**:

- the immediate program compares two named field models, \(\sigma_0\) and \(\sigma_1\);
- it makes no convergence claim;
- it does not automatically construct \(\sigma_2,\sigma_3,\ldots\);
- finite deterministic best-response towers are eventually periodic, so cycles are detectable if and when further levels are studied.

The next implementation should therefore include policy and field fingerprints from day one, but it should not add damping, mixtures, or equilibrium claims before a cycle is observed.

---

## 1. What the Plunge specimens establish—and what they do not

The Plunge review screenshots provide two valuable in-the-wild anchor families.

### 1.1 The retained 6-4 hand

In the bid-30 sixes hand that ended 25–17:

- the bidder led 6-6;
- Gran held both 6-2 and the ten-count 6-4;
- the trick-1 level-1 review at 40 worlds preferred 6-2 at 90% over 6-4 at 80%;
- the project record reports that a 160-world rerun changed the pick;
- when Gran later played 6-4 on trick 3, the 160-world review still showed a close comparison, 77% versus 72%.

This is a clean anchor for:

- fixed-\(n\) instability;
- retained-versus-revealed count timing;
- possible partner-response value;
- a level-0 versus level-1 field comparison.

### 1.2 The revealed 6-4 hand

In the bid-31 sixes hand that ended 36–0:

- Gran exposed the 6-4 under the bidder’s winning 6-6 early in the hand;
- the selected trick-4 review later showed all four legal options at 100% over 160 worlds.

The all-100% display is not proof that the four plays are exactly equivalent. It means only that the sampled panel observed no make/fail separation among them.

For every displayed pair on that panel,

\[
N_+=N_-=0.
\]

Directional pivotal evidence has not moved from its initial value.

### 1.3 The mechanism suggested by the pair

The physical tile is privately known to Gran throughout. The strategic variable is where knowledge of the tile lives across the partnership.

Before reveal:

\[
\Pr_G(L_{64}=G)=1,
\]

while the bidder’s belief about the holder of 6-4 is nondegenerate.

After reveal:

\[
\Pr_Y(L_{64}=G\mid \text{Gran publicly played 6-4})=1.
\]

That posterior collapse may change the bidder’s future action.

The proposed mechanism is therefore a **belief externality**:

> Gran’s play can matter because it changes her partner’s information state and therefore her partner’s continuation policy.

This is exactly the sort of mechanism a richer field model can represent.

### 1.4 The mathematical caution

The screenshots are discovery evidence, not a completed field-swap result.

They do **not** by themselves establish that:

- \(\sigma_0\) and \(\sigma_1\) actually choose different partner actions;
- revealing 6-4 caused the 36–0 result;
- level 2 prefers reveal on the failed hand;
- the two hands form a controlled comparison.

The field-swap probe must reconstruct the exact game seeds and records, couple the two field models, and locate the first actual policy split. Until that replay exists, “partner belief response” is the leading mechanism hypothesis, not a promoted conclusion.

This document makes that test precise.

---

## 2. Mathematical setting

Fix:

- one outer information state \(I\);
- declaration and bid;
- Boolean `pmake` utility;
- an exactly specified outer belief \(\beta\);
- two deterministic field models \(\sigma_0,\sigma_1\);
- one legal root action \(a\);
- an information-consistent focal continuation policy \(\rho\in\Pi_a\).

The current intended instantiation is:

- \(\sigma_0\): level-0 modeled minds;
- \(\sigma_1\): level-1 modeled minds;
- level-1 Walt: best response to \(\sigma_0\);
- level-2 Walt: best response to \(\sigma_1\).

The model index is part of the result. Raising the level changes the model; it does not change the rules of Texas 42 and does not imply equilibrium progress.

For physical world \(\omega\), let

\[
u_f(\rho,\omega)\in\{0,1\}
\]

be the terminal make indicator when the focal seat follows \(\rho\) and all non-focal seats follow \(\sigma_f\).

The fixed-policy value is

\[
V_f(\rho)=\mathbb E_{\omega\sim\beta}[u_f(\rho,\omega)].
\]

The optimized root-action value is

\[
Q_a^{(f)}
=
\sup_{\rho\in\Pi_a}V_f(\rho).
\]

Under the finite game this supremum is a maximum, but \(\sup\) keeps the proof notation neutral.

For two fixed focal policies \(\rho_a,\rho_b\), define the ordinary within-field paired difference

\[
Y_f(\omega)
=
u_f(\rho_a,\omega)-u_f(\rho_b,\omega)
\in\{-1,0,+1\}.
\]

Its field-\(f\) gap is

\[
g_f(a,b)
=
\mathbb E[Y_f].
\]

The field-swap lift is

\[
\boxed{
\Lambda_{a,b}
=
g_1(a,b)-g_0(a,b).
}
\]

The calculated-evidence handoff already separates:

- **response wake-up:** field change alters disagreement structure;
- **value wake-up:** \(\Lambda_{a,b}\ne0\);
- **decision wake-up:** the settled root choice changes.

That separation remains binding.

---

## 3. The field-disagreement frontier

A non-focal information state is denoted \(J\). It contains exactly the information available to the modeled seat: its hand and the public record or a proved sufficient reduction.

Define the field-disagreement frontier

\[
\boxed{
\mathcal F_{0,1}
=
\{J:\sigma_0(J)\ne\sigma_1(J)\}.
}
\]

Current modeled fields are deterministic in their information state. If a later field is stochastic, the construction must be lifted to scenarios with an explicit common tape or other declared coupling. This document’s first implementation scope is deterministic fields.

### 3.1 Coupled execution

For one fixed \((\rho,\omega)\), run two executions:

- execution 0 under \((\rho,\sigma_0)\);
- execution 1 under \((\rho,\sigma_1)\).

Start from the same root, physical world, and focal policy.

As long as the public histories agree:

- the focal seat has the same information state in both executions;
- information consistency forces \(\rho\) to choose the same action;
- every non-focal seat has the same information state in both executions;
- if that state is not in \(\mathcal F_{0,1}\), the two fields choose the same action.

Define \(D_\rho(\omega)=1\) when the coupled execution reaches a state in \(\mathcal F_{0,1}\) before termination, and \(D_\rho(\omega)=0\) otherwise.

This is the **field-exposure event** of policy \(\rho\).

### L2-T1 — first-disagreement localization

For every fixed information-consistent focal policy \(\rho\) and every world \(\omega\),

\[
D_\rho(\omega)=0
\quad\Longrightarrow\quad
u_0(\rho,\omega)=u_1(\rho,\omega).
\]

Equivalently,

\[
\boxed{
|u_1(\rho,\omega)-u_0(\rho,\omega)|
\le
D_\rho(\omega).
}
\]

#### Proof

Induct on primitive play steps.

At step zero the public histories are equal.

Assume the histories are equal before a step.

- If the focal seat acts, its private hand and public record are equal in both executions. Information consistency of \(\rho\) gives the same action.
- If a non-focal seat acts and the common information state is outside \(\mathcal F_{0,1}\), the two field policies give the same action.
- The rules transition is deterministic, so equal state and equal action give equal next public history.

If no frontier state is reached, the induction continues through terminal. The terminal histories and Boolean payoffs are equal. ∎

### 3.2 Fixed-policy correction bound

Define the policy correction variable

\[
C_\rho(\omega)
=
u_1(\rho,\omega)-u_0(\rho,\omega)
\in\{-1,0,+1\},
\]

and its mean

\[
c_\rho
=
V_1(\rho)-V_0(\rho).
\]

Let

\[
d_\rho
=
\Pr(D_\rho=1).
\]

By L2-T1,

\[
\boxed{
|c_\rho|
\le
\mathbb E[|C_\rho|]
\le
d_\rho.
}
\]

The correction can be much smaller than exposure: the fields may split but later produce the same make/fail outcome. Exposure is an upper bound, not a value estimate.

### 3.3 Fixed-pair correction bound

For frozen \(\rho_a,\rho_b\),

\[
\Lambda_{a,b}
=
c_{\rho_a}-c_{\rho_b}.
\]

Therefore

\[
\boxed{
|\Lambda_{a,b}|
\le
d_{\rho_a}+d_{\rho_b}.
}
\]

If

\[
g_0(a,b)>d_{\rho_a}+d_{\rho_b},
\]

then

\[
g_1(a,b)>0.
\]

Thus a sufficiently large level-1 margin survives the field upgrade without directly estimating the level-2 gap.

---

## 4. Optimized root-action field stability

Fixed-policy exposure is a detector. To make a statement about an optimized root action, the optimization lock must also be respected.

For legal root action \(a\), define the maximum field exposure

\[
\boxed{
R_a
=
\sup_{\rho\in\Pi_a}
\Pr(D_\rho=1).
}
\]

This is the greatest probability with which any lawful information-consistent continuation beginning with \(a\) can encounter the field-disagreement frontier.

### L2-T2 — root-action field Lipschitz bound

For every root action \(a\),

\[
\boxed{
\left|Q_a^{(1)}-Q_a^{(0)}\right|
\le
R_a.
}
\]

#### Proof

For every \(\rho\in\Pi_a\), L2-T1 gives

\[
V_1(\rho)
\le
V_0(\rho)+\Pr(D_\rho=1)
\le
V_0(\rho)+R_a.
\]

Taking the supremum over \(\rho\in\Pi_a\),

\[
Q_a^{(1)}
\le
Q_a^{(0)}+R_a.
\]

Interchanging fields 0 and 1 gives

\[
Q_a^{(0)}
\le
Q_a^{(1)}+R_a.
\]

Combine the inequalities. ∎

### L2-T3 — root winner stability

Suppose action \(a\) is optimal under \(\sigma_0\). If, for every rival \(b\),

\[
\boxed{
Q_a^{(0)}-Q_b^{(0)}
>
R_a+R_b,
}
\]

then \(a\) is strictly optimal under \(\sigma_1\).

#### Proof

For every \(b\),

\[
Q_a^{(1)}
\ge
Q_a^{(0)}-R_a
>
Q_b^{(0)}+R_b
\ge
Q_b^{(1)}.
\]

Therefore \(a\) remains strictly best. ∎

This is the core theorem that makes targeted level 2 mathematically safe.

---

## 5. Interval form and the admissible level-2 set

Exact root values and exact \(R_a\) values will not always be available. Let valid bounds be

\[
Q_a^{(0)}
\in
[L_a^{(0)},U_a^{(0)}]
\]

and

\[
R_a
\le
R_a^U.
\]

Then valid field-1 bounds are

\[
\boxed{
L_a^{(1)}
=
L_a^{(0)}-R_a^U,
}
\]

\[
\boxed{
U_a^{(1)}
=
U_a^{(0)}+R_a^U.
}
\]

Define

\[
B
=
\max_a L_a^{(1)}.
\]

The **admissible level-2 action set** is

\[
\boxed{
\mathcal A_1
=
\{a:U_a^{(1)}\ge B\}.
}
\]

### L2-T4 — safe action screening

Every action excluded from \(\mathcal A_1\) is incapable of being optimal under field \(\sigma_1\), on the event that all input bounds are valid.

#### Proof

If \(a\notin\mathcal A_1\), then

\[
Q_a^{(1)}
\le
U_a^{(1)}
<
B.
\]

By definition of \(B\), some action \(c\) satisfies

\[
L_c^{(1)}=B,
\]

so

\[
Q_c^{(1)}
\ge
B
>
Q_a^{(1)}.
\]

Thus \(a\) cannot be optimal. ∎

Consequences:

- If \(|\mathcal A_1|=1\), the root is field-stable and needs no level-2 optimization.
- If \(|\mathcal A_1|>1\), level-2 work is confined to \(\mathcal A_1\).
- Excluded actions do not need level-1-field continuation discovery.
- The screen is only as strong as the \(R_a^U\) bounds, but it is never made unsound by looseness. Loose bounds merely admit too many actions.

### 5.1 Field-stability slack

For a proposed winner \(a\) against rival \(b\), define

\[
\boxed{
S_{a,b}
=
L_a^{(0)}-U_b^{(0)}
-
R_a^U-R_b^U.
}
\]

Interpretation:

- \(S_{a,b}>0\): the pair is field-stable;
- \(S_{a,b}=0\): exact boundary under current bounds;
- \(S_{a,b}<0\): the field upgrade can still alter the ordering.

The most negative slack identifies where the next unit of field-level compute belongs.

This is the level-2 analogue of evidence debt.

---

## 6. Three distinct exposure result tiers

The project must not conflate policy-specific observations with root-action bounds.

### 6.1 `FrozenPolicyExposure`

For one named frozen policy \(\rho\), estimate or exactly compute

\[
d_\rho=\Pr(D_\rho=1).
\]

This supports fixed-policy statements such as

\[
|V_1(\rho)-V_0(\rho)|\le d_\rho.
\]

It does not account for omitted continuations.

### 6.2 `LibraryExposure`

For a fixed finite policy library \(\mathcal L_a\subseteq\Pi_a\), define

\[
R_a^{\mathcal L}
=
\max_{\rho\in\mathcal L_a}d_\rho.
\]

This supports field-stability statements only for the named library.

### 6.3 `RootActionExposureUpper`

A valid upper bound

\[
R_a\le R_a^U
\]

ranges over **all** information-consistent continuations in \(\Pi_a\).

Only this tier supports L2-T2 through L2-T4 for the optimized root action.

The data type and serialized result must identify which tier produced the number.

---

## 7. Obtaining useful exposure upper bounds

The project should tighten exposure in rungs. Each rung is useful. No rung may be silently promoted to a stronger one.

### 7.1 Rung E0 — exact field equality

If \(\sigma_0\) and \(\sigma_1\) choose the same action at every non-focal information state reachable after root action \(a\), then

\[
R_a=0.
\]

This is the strongest and cheapest result when it occurs.

For a complete dependency-closed reachable domain, exact policy equality is enough.

### 7.2 Rung E1 — structural split cover

Construct a structural world predicate \(P_a(\omega)\) satisfying

\[
D_\rho(\omega)=1
\quad\Longrightarrow\quad
P_a(\omega)=1
\]

for every \(\rho\in\Pi_a\).

Then

\[
\boxed{
R_a
\le
\beta(P_a).
}
\]

Under the current uniform fiber, an exact count gives

\[
\beta(P_a)
=
\frac{|\Phi(I)\cap P_a|}{|\Phi(I)|}.
\]

This is the direct counted-boundary route.

The predicate may be loose. Looseness costs pruning power, not correctness.

### 7.3 Rung E2 — clairvoyant split-reach cover

For each physical world, allow the focal seat to choose future actions with full knowledge of that world and ask whether **some** legal continuation can reach \(\mathcal F_{0,1}\).

Let \(P_a^{\mathrm{PI}}(\omega)\) be that event.

Every information-consistent continuation is among the clairvoyant possibilities, so

\[
D_\rho(\omega)
\le
P_a^{\mathrm{PI}}(\omega)
\]

and therefore

\[
\boxed{
R_a
\le
\Pr(P_a^{\mathrm{PI}}=1).
}
\]

This deliberately permits strategy fusion, but only in the safe direction: it is an upper bound on exposure, never a playable policy or lower witness.

Because the objective terminates at the first field split and ignores final game value, the perfect-information reachability problem may be substantially cheaper than level-2 play.

### 7.4 Rung E3 — information-consistent split-reach solve

Define a new Boolean control objective:

\[
U_{\mathrm{split}}=
\mathbf 1\{\text{the play reaches }\mathcal F_{0,1}\}.
\]

Before the first split, \(\sigma_0\) and \(\sigma_1\) make the same non-focal moves. Therefore this is a single fixed-field imperfect-information control problem.

For root action \(a\), its exact optimal value is precisely

\[
R_a.
\]

This **split-reach solver** can reuse Walt’s existing machinery:

- same lawful outer fiber;
- same information-consistent focal policies;
- same exact or calculated-evidence controller;
- terminal value 1 at the first field split;
- terminal value 0 at hand end without a split.

It is not a naive level-2 solve. It never simulates beyond the first field disagreement.

A sampled lower witness to \(R_a\) is not an upper bound. To feed L2-T4, this solve must return:

- an exact value;
- an admissible upper bound;
- or a valid structural over-approximation.

This optimization-lock boundary is load-bearing.

### 7.5 Rung E4 — exact dependency closure

At late grades or small fibers, enumerate the complete outer fiber and solve the split-reach objective exactly.

This yields exact \(R_a\), exact field-stability screening, and a ground-truth validation target for the looser rungs.

---

## 8. A targeted level-2 controller

The controller below assumes the calculated-evidence and exact-escalation machinery already exists.

### Stage 0 — identify the two fields

Materialize immutable:

- `FieldId(σ0)`;
- `FieldId(σ1)`.

A field identity includes:

- model level;
- policy construction version;
- inner decision configuration;
- risk and equivalence settings;
- fallback semantics;
- seed schedules;
- tie handling;
- policy-library identity;
- any exact versus heuristic mode affecting actions.

A changed field creates a new experiment epoch.

### Stage 1 — solve the level-1 root

Under \(\sigma_0\), obtain valid values or bounds for every legal root action.

Possible result tiers:

- exact root values;
- exact frozen-set values;
- \(\delta\)-valid fixed-policy intervals;
- unresolved.

No field-stability claim outruns the baseline result tier.

### Stage 2 — obtain all-action exposure bounds

For every legal action \(a\), obtain \(R_a^U\) using the cheapest available rung:

1. exact equality;
2. structural split cover;
3. clairvoyant split-reach cover;
4. information-consistent split-reach upper bound;
5. exact split-reach value.

The screen must consider every legal root action. Heuristic “signaling-looking” features may determine evaluation order, but they may not exclude an action from the mathematical screen.

### Stage 3 — compute the admissible set

Calculate

\[
L_a^{(1)}=L_a^{(0)}-R_a^U,
\qquad
U_a^{(1)}=U_a^{(0)}+R_a^U,
\]

\[
B=\max_aL_a^{(1)},
\]

\[
\mathcal A_1=\{a:U_a^{(1)}\ge B\}.
\]

If \(|\mathcal A_1|=1\), stop with a field-stability result at the tier supported by the inputs.

### Stage 4 — run field-1 work only on survivors

For actions in \(\mathcal A_1\):

- discover or materialize field-1-optimal continuation policies;
- evaluate them on common indexed worlds;
- use calculated evidence;
- escalate to exact fiber when cheaper;
- maintain optimization bounds.

Actions outside \(\mathcal A_1\) remain excluded by the field-stability bound and consume no level-2 rollout budget.

### Stage 5 — return a typed result

Proposed metadata:

- `FieldStableExactRoot`;
- `FieldStableExactFrozenSet`;
- `FieldStableDeltaFrozenSet`;
- `FieldSensitive`;
- `FieldDecisionChanged`;
- `FieldUnresolved`;
- `HeuristicFallback`.

The project may choose different Rust names. The semantic distinctions must remain.

### 8.1 Controller pseudocode

```text
baseline = solve_under_field(sigma0, root)

for each legal action a:
    exposure_upper[a] = cheapest_sound_exposure_upper(root, a, sigma0, sigma1)

for each action a:
    lower1[a] = baseline.lower[a] - exposure_upper[a]
    upper1[a] = baseline.upper[a] + exposure_upper[a]

bar = max_a lower1[a]
survivors = { a | upper1[a] >= bar }

if survivors has one action:
    return FieldStable(...)

level2 = solve_under_field(
    sigma1,
    root,
    root_actions = survivors,
    bounds_for_excluded_actions = upper1
)

return typed(level2)
```

The algorithm is safe because the screen uses upper bounds. It remains useful even if every bound is initially 1; the first result merely degenerates to the naive survivor set until the project learns tighter structure.

---

## 9. Calculated evidence inside the field-swap program

No new fixed sample counts are introduced here.

### 9.1 Fixed-policy correction geometry

For one focal policy \(\rho\),

\[
C_\rho
=
u_1(\rho,\omega)-u_0(\rho,\omega)
\in\{-1,0,+1\}.
\]

Define:

\[
q_\rho^\Delta
=
\Pr(|C_\rho|=1),
\]

\[
\tau_\rho^\Delta
=
\mathbb E[C_\rho\mid |C_\rho|=1],
\]

\[
c_\rho
=
q_\rho^\Delta\tau_\rho^\Delta.
\]

The ordinary signed-pivotal evidence process applies directly to \(C_\rho\).

This tells whether the field upgrade improves or harms that fixed policy.

### 9.2 Pair correction

For two policies,

\[
Z
=
Y_1-Y_0
\in\{-2,-1,0,1,2\},
\]

\[
\mathbb E[Z]=\Lambda_{a,b}.
\]

Use the bounded-mean evidence engine on

\[
X=Z/2\in[-1,1].
\]

Do not reduce \(Z\) to its sign. Difference magnitude matters.

### 9.3 Direct field-1 comparison

When the target is simply the preferred action under \(\sigma_1\), evaluate \(Y_1\) and use the ordinary exact pivotal evidence process.

The correction decomposition is still logged for diagnosis and explanation.

### 9.4 Field-stability threshold test

Suppose safe exposure bounds \(R_a^U,R_b^U\) are known.

To establish fixed-pair stability without estimating \(g_1\), test directly

\[
H_0:
g_0(a,b)
\le
R_a^U+R_b^U
\]

against

\[
g_0(a,b)
>
R_a^U+R_b^U.
\]

Because \(Y_0\in[-1,1]\), the bounded-mean process can test this threshold exactly and anytime-validly whenever the threshold lies inside the observation range.

If the exposure sum is at least 1, no Boolean value gap can dominate it; the pair cannot be screened by this bound.

### 9.5 Information rate

Within each field,

\[
\mathcal I_f
=
q_fD_{\mathrm{Ber}}
\left(
\frac{1+\tau_f}{2}
\middle\Vert
\frac12
\right).
\]

The field-1 comparison is easier to sample when

\[
\mathcal I_1>\mathcal I_0.
\]

The targeted controller additionally records:

- exposure probability;
- first-split depth;
- correction evidence growth;
- post-split compute cost.

A larger \(q_1\) alone is not a success criterion.

---

## 10. First-split traces as explanations

The first field split is not merely a performance optimization. It is a model-grounded explanation.

For every world contributing to a material field correction, persist a `FieldSplitTrace` containing:

1. root information-state identity;
2. physical world identity;
3. focal root action and frozen focal PolicyId;
4. `FieldId(σ0)` and `FieldId(σ1)`;
5. first common public record \(R^\star\) where the fields disagree;
6. acting non-focal seat;
7. that seat’s private hand at \(R^\star\);
8. the complete modeled information-state key;
9. \(\sigma_0\)’s chosen tile;
10. \(\sigma_1\)’s chosen tile;
11. the public observation that distinguishes the relevant branches;
12. terminal outcome under each field;
13. whether the correction favors or harms the root action;
14. structural motif tags.

For the Gran 6-4 hypothesis, the expected trace shape is:

- one Gran action reveals 6-4 and the other retains it;
- the bidder later reaches different public information states;
- the level-0 and level-1 bidder models choose different actions;
- those actions separate make from fail on some worlds.

The experiment must print the actual trace or reject that mechanism.

### 10.1 Aggregate explanation

Across correction-pivotal worlds, report:

- mass reaching any field split;
- mass of positive corrections;
- mass of negative corrections;
- first-split seat;
- first-split trick;
- recurring private/public knowledge motif;
- exact or estimated belief mass of the motif;
- conditional outcome difference.

The final human-facing explanation can then be:

> Playing 6-4 now reveals the ten-count trump to your partner. On the worlds where the two field models differ, the richer partner model changes its next play after that reveal, and those changed responses favor the contract by the measured amount.

That is an explanation of the modeled causal path, not a post-hoc slogan.

---

## 11. The Gran anchor experiment

The two hands should become named, reproducible anchor positions after their game seeds and records are recovered.

The screenshots themselves are preserved as discovery artifacts. The seed-derived game records are the computational source.

### G1 — failed-hand trick-1 root

Root:

- bid 30;
- sixes;
- bidder leads 6-6;
- Gran holds 6-2 and 6-4;
- compare immediate 6-4 reveal versus 6-2 retention.

Required outputs:

1. exact root/fiber identity;
2. level-1 values or valid bounds;
3. level-1 result kind;
4. all-action field exposure bounds;
5. admissible level-2 action set;
6. \(q_0,\tau_0,g_0,\mathcal I_0\);
7. \(q_1,\tau_1,g_1,\mathcal I_1\);
8. field lift \(\Lambda\);
9. first field-split trace;
10. exact or \(\delta\)-settled selected action under each field;
11. sample-versus-enumerate route;
12. policy and field identities.

### G2 — successful-hand early reveal root

Reconstruct Gran’s decision when she could first expose 6-4 under the bidder’s winning trump.

Do not use the later trick-4 all-100% review as the causal root. That card is a saturation footprint after the information has already become public.

Run the same outputs as G1.

### G3 — later trick-4 saturation root

At the displayed all-100% position:

- calculate exact fiber size;
- compute pairwise pivotal counts;
- show whether \(q=0\) exactly, \(q\) is merely small, or the 160-world panel was uninformative;
- route to exact enumeration when cheap;
- return `EpsilonEquivalent`, exact equivalence, or `Unresolved` rather than four unlabeled “walt’s pick” badges.

### G4 — mechanism adjudication

The mechanism “revelation changes the partner’s belief and therefore its play” is accepted for the anchor only if:

- a first field split is observed or exactly proved reachable;
- the split occurs on a partner information state affected by the reveal;
- the changed partner action contributes nonzero value correction.

If the fields never split on that mechanism, the anchor remains useful: it refutes the proposed explanation and directs attention elsewhere.

---

## 12. Why this is more targeted than naive level 2

A naive level-2 pass pays for:

\[
\text{every root}
\times
\text{every legal action}
\times
\text{every sampled world}
\times
\text{every field decision}
\times
\text{every level-1 modeled solve}.
\]

The targeted program pays in this order:

1. level-1 root value;
2. field-disagreement exposure bounds;
3. level-2 work only for actions whose field-1 upper bound overlaps the best field-1 lower bound;
4. coupled correction only on worlds that actually reach a split;
5. exact enumeration whenever it is cheaper.

Its operative complexity is not “the cost of level 2 everywhere.”

It is:

\[
\boxed{
\text{mass and search complexity of the field-disagreement frontier near the root decision boundary.}
}
\]

### 12.1 Cost decomposition

Let:

- \(C_0\): cost of the correct level-1 root;
- \(C_R(a)\): cost of obtaining exposure bound \(R_a^U\);
- \(\mathcal A_1\): admissible field-1 action set;
- \(C_1(a)\): cost of field-1 optimization for survivor \(a\).

Then

\[
\boxed{
C_{\mathrm{target}}
=
C_0
+
\sum_a C_R(a)
+
\sum_{a\in\mathcal A_1}C_1(a).
}
\]

Compare with

\[
C_{\mathrm{naive}}
=
\sum_a C_1(a)
\]

plus any duplicated baseline work.

The exposure screen earns its keep when it is cheaper than the survivor work it eliminates.

### 12.2 Routing by stability debt

Prioritize pairs with the smallest or most negative

\[
S_{a,b}
=
L_a^{(0)}-U_b^{(0)}-R_a^U-R_b^U.
\]

Prioritize exposure-bound tightening for action \(a\) by the amount that reducing \(R_a^U\) can shrink \(\mathcal A_1\), divided by measured cost.

This is a direct continuation of calculated evidence: spend only where the decision can still move.

---

## 13. Best-response towers and cycling

The user’s concern is mathematically well-founded.

A best-response tower need not converge. Rock-paper-scissors-style cycles are possible in finite strategic systems.

The correct response is not to assume stability and not to abandon level 2. It is to type the tower correctly and instrument it from the beginning.

### 13.1 The tower operator

Fix:

- the finite game;
- belief semantics;
- exact or deterministic decision procedure;
- tie rule;
- resource/fallback semantics;
- all policy-construction parameters.

Let \(\mathcal S\) be the finite set of deterministic field-policy profiles under those rules.

Define

\[
B:\mathcal S\to\mathcal S
\]

where \(B(\sigma)\) is the profile obtained when each modeled seat uses the declared best-response construction against field \(\sigma\).

The level tower is

\[
\sigma_{k+1}=B(\sigma_k).
\]

### L2-T5 — eventual periodicity

If \(B\) is deterministic and \(\mathcal S\) is finite, then the sequence

\[
\sigma_0,\sigma_1,\sigma_2,\ldots
\]

is eventually periodic.

#### Proof

Some pair \(i<j\) must satisfy \(\sigma_i=\sigma_j\) by the pigeonhole principle. Determinism gives

\[
\sigma_{i+t}=\sigma_{j+t}
\]

for every \(t\ge0\). ∎

The period may be 1, but no theorem here forces it.

### 13.2 Why cycling is not yet a danger to the next step

The immediate program performs one named comparison:

\[
\sigma_0
\longrightarrow
\sigma_1.
\]

Level-2 Walt is a best response to the fixed field \(\sigma_1\).

Nothing in this construction requires:

- \(\sigma_1\) to be better than \(\sigma_0\) in every sense;
- \(\sigma_2\) to resemble \(\sigma_1\);
- the tower to converge;
- level index to be a monotone strength coordinate.

The first field swap is therefore mathematically well-defined even if a later tower cycles.

Cycling becomes a direct concern only when the project proposes automatic promotion to higher \(k\) or interprets high \(k\) as convergence.

### 13.3 Three stability notions

#### Root-decision stability

The selected root action is unchanged between two named fields.

This can be established by L2-T3/L2-T4 even when the field policies differ elsewhere.

#### Behavioral field stability

The two fields choose the same action on every information state reached by a declared common panel or anchor closure.

This is empirical on a panel unless the domain is complete.

#### Exact local policy stability

The field policies agree on a dependency-closed set of information states sufficient to construct every policy action used in the target computation.

If the closure is complete for the local operator and

\[
\sigma_{k+1}=\sigma_k
\]

on that closure, then the local tower is at a fixed point there.

### 13.4 Cycle detection levels

Persist for every level:

- `FieldId`;
- every materialized `PolicyId`;
- action map over queried information states;
- dependency edges showing which lower-level policy states were consulted;
- common-panel outcome bitsets;
- root decisions;
- field-exposure values between successive levels.

Classify repetitions as follows.

1. **Root-action recurrence.**  
   The same finite sequence of root choices repeats. This is a symptom, not a policy-cycle proof.

2. **Behavioral recurrence.**  
   Field action fingerprints and outcome bitsets repeat on a fixed common panel.

3. **Exact local cycle.**  
   Complete policy fingerprints repeat on a dependency-closed domain.

4. **Exact global cycle.**  
   Complete field profiles repeat over the full finite policy domain.

Do not promote a panel recurrence to an exact cycle.

### 13.5 The one-more-level tripwire

Before any future broad level-3 project, run a small targeted tripwire on the level-2-sensitive anchor corpus:

- compare \(\sigma_1\) and \(\sigma_2\);
- measure first-split exposure;
- compare the sign of the \(0\to1\) and \(1\to2\) field corrections;
- flag roots where the chosen action returns to the level-1 answer.

This is enough to detect an early two-cycle tendency without building level 3 everywhere.

### 13.6 What to do if a cycle appears

The first response is classification, not damping.

Lawful options include:

1. **Named-level selection.**  
   Keep level 2 as a fixed field model because it performs best on declared tests. No convergence claim.

2. **Robust choice over a detected finite cycle.**  
   For cycle fields \(\Sigma\), choose

   \[
   \arg\max_a\min_{\sigma\in\Sigma}Q_a^\sigma.
   \]

   This is conservative and is not a best response to one field.

3. **Declared mixture over cycle fields.**  
   Choose an explicit prior over cycle phases and optimize expected value. This creates a stochastic field and requires explicit tape/coupling semantics.

4. **Regularized or damped update.**  
   This changes the operator and requires new mathematics. It must not be slipped into the current level definition as a patch.

No mitigation is authorized merely because cycles are possible in principle.

---

## 14. Interpretability as a first-class mathematical output

The project did not begin by aiming for explanation, but the field-disagreement frontier makes explanation structural.

A level-2 explanation should answer:

1. **Where did the two field models first act differently?**
2. **What information did that seat possess at that moment?**
3. **What public observation created or removed the uncertainty?**
4. **What actions did the two models choose?**
5. **Which terminal outcomes changed?**
6. **How much belief mass followed that mechanism?**
7. **Was the mechanism necessary for the root decision to change?**

The explanation should never be generated from narrative intuition alone. It should be backed by stored first-split and correction-pivotal worlds.

### 14.1 Suggested Plunge language

For an unresolved saturation:

> Walt has not seen a world where these plays change make versus set. That is not proof they are equal. It is continuing until the evidence settles or exact enumeration becomes cheaper.

For a field-stable decision:

> A richer partner model changes play on at most this much of the current belief, which is too little to overcome the measured advantage of this tile.

For a field-sensitive reveal:

> Revealing 6-4 changes your partner’s modeled next play on the pivotal worlds. Those response changes are why the richer model prefers the reveal.

The UI may simplify the arithmetic, but logs must retain the exact result type and evidence.

---

## 15. Exactness boundaries

The three locks remain.

### 15.1 Measure lock

The outer belief mass of exposure, correction, and pivotal regions must be exact or validly estimated under the same fiber.

### 15.2 Response lock

The actions and terminal outcomes under each named field must be exact relative to immutable field policies.

### 15.3 Optimization lock

A root-action claim must account for omitted focal continuations.

Consequently:

- fixed-policy correction does not prove optimized root correction;
- library exposure does not prove root exposure;
- sampled observation of no field split does not prove \(R_a=0\);
- a field-stable frozen set is not an exact field-stable root;
- a richer field that changes values is not thereby an equilibrium improvement.

The result type carries which locks are closed.

---

## 16. Experiment program

### L2-E0 — theorem fixtures

Construct tiny finite games where:

1. fields never disagree;
2. fields disagree but payoff never changes;
3. fields disagree and correction is positive;
4. fields disagree and correction is negative;
5. baseline margin exceeds exposure sum;
6. baseline margin does not exceed exposure sum;
7. a rival excluded by L2-T4 is provably nonoptimal;
8. a two-cycle exists under the best-response operator.

Verify every theorem and result type exactly.

### L2-E1 — Gran anchors

Run G1–G4 from §11.

Primary question:

> Does immediate 6-4 revelation produce a first partner field split and a nonzero correction on the reconstructed world fiber?

### L2-E2 — saturation and near-tie corpus

Build a predeclared corpus from:

- race-mode saturation remnants;
- historical 40/160 choice flips;
- the level-2 trick-1 tie episode;
- Plunge review positions;
- divergence-miner positions;
- ordinary control positions with clear level-1 margins.

For each root, report:

- baseline action bounds;
- exposure upper bounds;
- admissible field-1 set size;
- exact versus \(\delta\)-settled status;
- field correction;
- first-split motif;
- targeted versus naive measured cost.

### L2-E3 — exact small-fiber parity

At grades where full fibers are affordable:

1. compute exact \(Q_a^{(0)}\);
2. compute exact \(R_a\);
3. apply L2-T4;
4. compute exact \(Q_a^{(1)}\);
5. verify that every excluded action is nonoptimal;
6. compare the targeted action set with the cold full level-2 solve.

This is the load-bearing implementation gate.

### L2-E4 — exposure-rung study

For the same roots, compare:

- structural cover mass;
- clairvoyant reach cover mass;
- information-consistent \(R_a\);
- actual fixed-policy exposure;
- compute cost of each rung.

This tells the project which exposure object is worth building at early grades.

### L2-E5 — explanation recurrence

Cluster first-split traces under the existing equivariance quotient.

Measure whether a small motif library captures:

- count revelation;
- trump support;
- void revelation;
- protection/guarding;
- partner winner recognition;
- defensive signaling.

This is the interpretability analogue of the signed-boundary census.

### L2-E6 — cycle tripwire

On only the field-sensitive anchor corpus:

- materialize \(\sigma_0,\sigma_1,\sigma_2\) on the dependency closure;
- compare field hashes, action maps, and correction signs;
- look for fixed points, reversals, and period-2 behavior.

This is an instrument, not authorization for a broad level-3 player.

---

## 17. Success conditions and falsifiers

### 17.1 The targeted hypothesis is strengthened if

- most roots have small exposure bounds or large positive stability slack;
- \(|\mathcal A_1|\) is usually much smaller than the legal action count;
- field corrections concentrate on recognizable first-split motifs;
- exact small-fiber parity always validates the screen;
- level-2-sensitive roots are concentrated in signaling and coordination situations;
- field-1 information rate exceeds field-0 rate on the near-tie corpus;
- the Gran anchor prints the predicted partner-response trace;
- policy fingerprints show local stability rather than immediate cycling.

### 17.2 It is weakened if

- safe exposure bounds remain near 1 for almost every action;
- the screen rarely excludes anything;
- obtaining \(R_a^U\) costs as much as cold level 2;
- exact small-fiber runs find a level-2 winner outside the admissible set;
- correction motifs are structurally diffuse;
- the field upgrade adds response disagreement but no value or decision wake-up;
- level-2 continuation discovery dominates every other cost;
- early policy levels immediately enter broad cycles.

A failed targeting hypothesis is still useful. It tells the project honestly that the richer model’s influence is global rather than localized.

---

## 18. Unified engineering shape

No new crate.

Recommended local modules inside unified `walt`:

### `solver::field`

Owns:

- `FieldId`;
- field level and dependencies;
- immutable modeled-policy access;
- field action cache;
- field-to-field action comparison;
- dependency graph for cycle analysis.

### `solver::exposure`

Owns:

- field-disagreement frontier;
- coupled pre-split replay;
- `FrozenPolicyExposure`;
- structural split covers;
- clairvoyant reach cover;
- information-consistent split-reach objective;
- exposure upper-bound typing;
- first-split depth statistics.

### `solver::field_swap`

Owns:

- fixed-policy correction;
- pair lift \(\Lambda\);
- response/value/decision wake-up;
- admissible level-2 action set;
- field-stability slack;
- targeted survivor routing;
- `FieldSplitTrace`.

### Existing `solver::evidence`

Owns all anytime-valid arithmetic and risk ledgers. `field_swap` consumes it and never reimplements it.

### Existing `kernel`

Remains the one authority for:

- fiber identity;
- exact count;
- indexed uniform world stream;
- exact enumeration;
- exact structural masses.

### Thin consumers

Plunge review, WASM, arena, and research binaries call one API. They may choose resource policies; they may not invent alternate level-2 semantics.

---

## 19. Acceptance contract

The first targeted level-2 implementation is complete only when all of the following hold.

1. The calculated-evidence correctness path is already in place.
2. `FieldId(σ0)` and `FieldId(σ1)` are immutable and serialized.
3. The focal policy is frozen before cross-field evidence begins.
4. Coupled executions share the same physical world.
5. Before the first field split, the two public histories are asserted equal.
6. Every reported field correction stores its first split or states that no split occurred.
7. Fixed-policy exposure and root-action exposure are mechanically different types.
8. A sampled fixed-policy exposure is never used as an upper bound on omitted continuations.
9. Every root-action exposure upper bound names its derivation rung.
10. All legal root actions receive a safe exposure bound before exclusion.
11. The admissible set implements L2-T4 exactly.
12. Excluded actions are not sent to the level-2 optimizer.
13. Field-1 optimization is restricted to the admissible set but remains information-consistent.
14. Field correction evidence uses complete signed or bounded differences, never sign frequency alone.
15. Response, value, and decision wake-up are reported separately.
16. The Gran trick-1 anchor is reconstructed from a game seed and public record.
17. The all-100% trick-4 anchor is reported as exact equivalence, practical equivalence, or unresolved—not inferred equality.
18. Small-fiber exact parity validates every exclusion.
19. Field and policy fingerprints are persisted by level.
20. No automatic level escalation beyond level 2 is enabled.
21. Any observed recurrence is labeled root, behavioral, local exact, or global exact.
22. No cycle mitigation is introduced without a separate mathematical intake.
23. Live level-1 play remains available as fallback during shadow validation.
24. The Plunge UI preserves result type and does not flatten field-stable, field-sensitive, unresolved, and heuristic outputs into one unlabeled percentage.

---

## 20. Proposed new obligations

These numbers continue the calculated-evidence proposal’s O20–O28 line. They are proposals for intake and adjudication, not self-issued rulings.

### O29 — field identity and purity

Every modeled field action is a pure function of its declared information state and immutable `FieldId`. Cross-field comparison cannot read hidden data outside that state.

**Route:** data-flow audit, deterministic replay, hidden-world adversarial tests.

### O30 — first-disagreement localization

The implementation satisfies L2-T1: before the first field action difference, coupled histories and focal actions remain identical; no terminal correction is attributed to an unsplit world.

**Route:** paper proof here, stepwise assertions, tiny exact fixtures.

### O31 — exposure-bound typing

`FrozenPolicyExposure`, `LibraryExposure`, and `RootActionExposureUpper` are distinct. Only the last may feed optimized root-action screening.

**Route:** type/API audit and negative compile or serialization tests.

### O32 — root field-stability screening

The implementation of L2-T2 through L2-T4 uses valid value bounds and valid exposure upper bounds for every legal action.

**Route:** exact small-fiber parity and theorem replay.

### O33 — field-correction evidence

Fixed-policy correction, pair lift, and direct field-1 comparison use the calculated-evidence processes on complete signed values with a complete risk ledger.

**Route:** exact-rational tests and common-world regression.

### O34 — split-reach optimization correctness

The information-consistent split-reach solver computes or bounds

\[
\sup_{\rho\in\Pi_a}\Pr(D_\rho=1)
\]

without strategy fusion. Any clairvoyant version is labeled and consumed only as an upper bound.

**Route:** exact late-grade oracle and information-flow audit.

### O35 — mechanism-trace fidelity

Every explanation names an actual stored first field split and its downstream correction. Narrative motif labels cannot outrun the trace.

**Route:** trace replay and explanation snapshot tests.

### O36 — level-model typing

A level-2 result is explicitly best response to \(\sigma_1\). It is not labeled equilibrium, convergence, or monotone improvement.

**Route:** API/UI vocabulary audit.

### O37 — cycle detection discipline

Policy recurrence is classified by domain completeness. Panel recurrence is never promoted to exact cycle; exact local cycle requires a dependency-closed domain.

**Route:** synthetic cycle fixtures and hash/dependency tests.

### O38 — targeted completeness

No root action is excluded from field-1 optimization unless L2-T4 or a stronger exact argument removes it.

**Route:** all-action admission audit and exact small-fiber parity.

---

## 21. Recommended implementation order

### Step 1 — intake and adjudicate

File this parent verbatim with a checksum. Create a maintained companion that:

- verifies L2-T1 through L2-T5;
- checks all inequality directions;
- adjudicates result names and obligation numbers;
- reconciles this document with the current `LEVEL2-PROBE.md`;
- records the exact Gran anchor identifiers once reconstructed.

### Step 2 — land the calculated-evidence prerequisite

Do not start the targeted player by rebuilding fixed-\(n\) level 2.

The new field-swap code must consume the common evidence and exact-escalation authority.

### Step 3 — add `FieldId` and field comparison

Materialize \(\sigma_0\) and \(\sigma_1\) actions through one interface. Log exact information-state keys and action differences.

### Step 4 — build coupled first-split replay

For one fixed focal policy:

- run both fields on one world;
- assert common prefixes;
- stop or fork at the first split;
- return \(D_\rho,C_\rho\), and a trace.

### Step 5 — run the Gran fixed-policy smoke

Reconstruct the failed-hand trick-1 root. Compare reveal and retain policies under both fields. This is detector work only.

### Step 6 — implement exposure rungs E0–E2

Start with:

- exact field equality;
- structural split cover;
- clairvoyant split reach.

Measure whether those bounds already prune actions on exact small roots.

### Step 7 — implement information-consistent split reach

Reuse the solver with the binary hit-frontier objective. Add exact/upper-bound result typing.

### Step 8 — implement the admissible set

Apply L2-T4 to all legal actions. Add exact parity tests before any live use.

### Step 9 — target field-1 optimization

Run field-1 continuation discovery and evaluation only on the admissible set.

### Step 10 — shadow on Plunge review

For every reviewed decision, persist:

- level-1 result;
- exposure bounds;
- admissible field-1 set;
- targeted level-2 result;
- first-split trace;
- chosen action;
- result kind;
- exact reference where available.

Do not change live play defaults yet.

### Step 11 — run the predeclared corpus

Execute L2-E1 through L2-E5.

### Step 12 — add the cycle tripwire

Before any broad level-3 work, execute L2-E6 on the field-sensitive anchor closure.

---

## 22. What this document does not claim

1. It does not claim level 2 is globally stronger than level 1.
2. It does not claim the best-response tower converges.
3. It does not claim the Gran screenshots prove the partner-response mechanism.
4. It does not claim a field split necessarily changes value.
5. It does not claim increased pivotal mass means easier sampling.
6. It does not claim fixed-policy exposure bounds optimized root actions.
7. It does not claim a sampled exposure estimate is a safe upper bound on omitted policies.
8. It does not claim the structural split cover will be small.
9. It does not claim the exposure screen will always be cheaper than full level 2.
10. It does not claim a level-2 action change is an equilibrium improvement.
11. It does not claim an observed panel recurrence is an exact cycle.
12. It does not authorize damping, mixtures, or robust cycle policies now.
13. It does not delete level 1. Level 1 remains a named, useful, grounded model and fallback.
14. It does not require trick-1 exact level 2 immediately.
15. It does not replace the optimization lock with sampling confidence.

---

## 23. Final mathematical thesis

The cost of level 2 is not fundamentally the cost of re-solving every world under a richer field.

For a fixed focal policy, the field correction is supported only on worlds that reach

\[
\mathcal F_{0,1}
=
\{J:\sigma_0(J)\ne\sigma_1(J)\}.
\]

The correction magnitude is bounded by exposure to that frontier:

\[
\boxed{
|V_1(\rho)-V_0(\rho)|
\le
\Pr(D_\rho=1).
}
\]

For optimized root action \(a\),

\[
\boxed{
|Q_a^{(1)}-Q_a^{(0)}|
\le
R_a.
}
\]

The level-1 decision survives whenever its margin is larger than the maximum field correction available to either contender:

\[
\boxed{
Q_a^{(0)}-Q_b^{(0)}
>
R_a+R_b.
}
\]

That turns level 2 from a universal replacement into a calculated refinement.

Walt should:

- solve level 1 correctly;
- measure where the field models can actually disagree;
- prove stability where the disagreement cannot overcome the margin;
- spend level-2 compute only on the surviving action set;
- preserve the first field split as the explanation;
- escalate to exact fibers whenever cheaper;
- record the level tower without assuming convergence;
- and detect cycles before attempting to cure them.

The hoped-for stable object is not “higher \(k\) always wins.”

It is:

> **A root decision whose value margin is larger than every still-possible correction induced by the next field model.**

That is a mathematical notion of stability Walt can calculate.

And it is exactly the kind of stability this project has earned the right to look for.
