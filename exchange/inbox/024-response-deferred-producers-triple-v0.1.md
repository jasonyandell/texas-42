# RESPONSE — Three Deferred Producers

## Max-preserving E3 bounds, structural hazard exclusion, and a first-split motif algebra

**Status:** exploratory mathematical response to  
`exchange/drafts/deferred-producers-triple.md`.

**Date:** 2026-08-24

**Reviewed source:** `exchange/drafts/deferred-producers-triple.md`, blob
`e3ae5346306ee93c608276136e92812b6a5f5e40` on `main`.

**Scope:** finite perfect-recall deterministic-transition games with a fixed
focal policy class, fixed declared fields, Boolean terminal utility, and an
i.i.d. declared world stream from a finite rational belief. The motif section
specializes to partnership trick-taking games.

**Companion program:** `verify_deferred_producers_triple_v0_1.py`. It uses
integers and `fractions.Fraction` only and finishes with `ALL CHECKS PASS`.

---

## 0. Executive answer

The three deferred constructions all have clean answers, but not quite the
answers suggested by the initial candidate routes.

### Part 1

There is a **class-size-free anytime upper confidence sequence for the maximum
of finitely many means**.

The key asymmetry is this:

> To upper-bound a maximum, it is enough that one fixed true maximizer has a
> valid upper confidence sequence. Simultaneous coverage of every branch is
> unnecessary.

Let \(S_{\rho,n}\) be the number of split-reach successes of policy \(\rho\)
on the first \(n\) common worlds and let

\[
S_n^\star=\max_{\rho\in\Pi_a}S_{\rho,n}.
\]

If \(U_{\delta,N}(s,n)\) is the exact-rational one-mean upper confidence
endpoint obtained by inverting the existing lower-tail e-process on the known
finite-population grid \(G_N=\{0,1/N,\ldots,1\}\), then

\[
\boxed{
\widehat R^{E3}_{a,n}
=
\min_{1\le t\le n}
U_{\delta,N}(S_t^\star,t)
}
\]

satisfies

\[
\boxed{
\Pr\!\left(
\exists n:\ R_a>\widehat R^{E3}_{a,n}
\right)\le\delta.
}
\]

There is **no \(|\Pi_a|\)-way risk split**. The empirical optimizer may change
with \(n\); that causes no selection defect. The theorem needs only one fixed
true maximizing policy, selected by the underlying belief rather than by the
sample.

The same construction gives directional uppers for \(R_a^+\) and \(R_a^-\).

Pathwise,

\[
S_n^\star
\le
\sum_{i=1}^n \max_{\rho\in\Pi_a} D_\rho(\omega_i),
\]

so the new E3 bound is never looser than the sampled fused-E2 baseline when
both use the same one-mean upper engine and risk. It can be strictly tighter.

### Part 2

There is no uniquely “weakest local exchange rule.” The weakest exact
structural condition is the safety property itself:

> In the synchronized product of the \(a\)-execution and \(b\)-execution on
> the same world, the hazard terminal \(u_a=0,u_b=1\) is unreachable.

The proper machine-checkable proof object is a **Hazard-Exclusion Invariant**:
an inductive invariant over the paired transition system that covers the full
initial fiber, is closed under paired transitions, and contains no hazard
terminal.

This condition is sound and complete at the semantic level. Its practical
value comes from representing the invariant by a small structural partition
rather than by singleton worlds.

A cheap first producer is a deliberately incomplete
**One-Round Trump-Extraction Witness**. It certifies the familiar
highest-trump-versus-vulnerable-card pattern when one high-trump lead removes
every hostile trump and the vulnerable card is then safe. A three-trick,
two-round extraction example gives strict dominance while the one-round
producer correctly declines, proving honest non-coverage.

### Part 3

The current trace supports a small, objective **first-split morphology**
alphabet. It does **not** yet support a truthful causal label such as
`RevealResponse`.

For each chosen tile, derive the ordered local signature

\[
\Sigma(t)=
(
\text{next led context},
\text{provisional winning partnership},
\text{count payload},
\text{is trump},
\text{residual suit shape},
\text{played strength}
).
\]

The primary motif is the first coordinate on which
\(\Sigma(t_0)\ne\Sigma(t_1)\). This gives six exclusive motifs plus a
residual:

1. `LeadContextFork`
2. `ImmediateControlFork`
3. `CountCommitmentFork`
4. `TrumpCommitmentFork`
5. `SuitShapeFork`
6. `StrengthCommitmentFork`
7. `Other`

All coordinate differences should also be emitted as orthogonal flags. The
primary label is a partition; the flags preserve co-occurring structure.

Because the present `FieldSplitTrace` is persisted only for
terminal-outcome-changing worlds, these motifs partition **correction mass**,
not all field exposure. To classify exposure, traces or aggregate motif
counts must also be produced for exposed worlds with \(u_0=u_1\).

---

# Part 1 — The max-preserving admissible-upper E3 producer

## 1.1 First correct the testing orientation

The candidate “single mixture e-process over branches” is natural, but it
targets the wrong side if used naively.

A mixture of branch e-processes for

\[
H_0:\mu_\rho\le x\quad\text{for every }\rho
\]

tests the intersection null

\[
\max_\rho\mu_\rho\le x.
\]

Rejecting that null gives a **lower** confidence statement on the maximum.

An upper confidence statement for

\[
R=\max_\rho\mu_\rho
\]

must protect against the union null

\[
R\ge x
\iff
\exists\rho:\mu_\rho\ge x.
\]

A weighted mixture is generally invalid for that union null: only one branch
must satisfy its null, while the other branch processes may grow without
restriction and drive the mixture.

The correct upper-side construction is an intersection-union argument:
to reject \(R\ge x\), every possible maximizing branch must be rejected.
Equivalently, take the maximum of branchwise upper confidence endpoints.

The surprise is that this does **not** require a Bonferroni split.

## 1.2 Exact one-mean upper endpoint

Let \(X_1,X_2,\ldots\in\{0,1\}\) be i.i.d. with mean \(p\). After \(s\)
successes and \(f=n-s\) failures, the existing lower-tail evidence against

\[
H_0:p\ge c
\]

is

\[
E^-_{s,f}(c)
=
E^>_{f,s}(1-c)
=
\frac1c
\int_0^c
\left(\frac rc\right)^s
\left(\frac{1-r}{1-c}\right)^f\,dr.
\]

For a uniform finite fiber of known size \(N\), every exact policy mean lies
in

\[
G_N=\left\{0,\frac1N,\ldots,\frac{N-1}{N},1\right\}.
\]

Define the exact-rational endpoint

\[
U_{\delta,N}(s,n)
=
\max\left\{
c\in G_N:
E^-_{s,n-s}(c)<\frac1\delta
\right\},
\]

with the endpoint conventions:

- \(c=0\) is never rejected;
- \(c=1\) remains possible exactly when \(s=n\).

Then

\[
\Pr_p\!\left(
\exists n:\ p>U_{\delta,N}(S_n,n)
\right)\le\delta.
\]

This is direct inversion of the existing anytime test at the true grid point
\(c=p\).

### Monotonicity in the success count

For fixed \(n\), replacing one failure by one success multiplies each
lower-mixture integrand by

\[
\frac{r/c}{(1-r)/(1-c)}
=
\frac{r(1-c)}{c(1-r)}
\le1
\qquad (r\le c).
\]

Hence \(E^-_{s,n-s}(c)\) is nonincreasing in \(s\), and therefore
\(U_{\delta,N}(s,n)\) is nondecreasing in \(s\).

This monotonicity is what lets the empirical optimizer collapse the full
policy family to one integer count.

## 1.3 The max-preserving upper-CS theorem

### Theorem M1 — same-\(\delta\) upper confidence for a finite maximum

Let \(\Pi\) be finite and nonempty. For every \(\rho\in\Pi\), let
\(X_{\rho,1},X_{\rho,2},\ldots\in\{0,1\}\) be evaluated on the declared
world stream, with

\[
\mu_\rho=\mathbb E[X_{\rho,i}].
\]

The branch vectors on one world may have **arbitrary dependence**.

Suppose \(U_{\rho,n}\) is an anytime upper confidence sequence for the fixed
branch mean \(\mu_\rho\) at level \(\delta\):

\[
\Pr\!\left(
\exists n:\mu_\rho>U_{\rho,n}
\right)\le\delta
\quad\text{for each fixed }\rho.
\]

Set

\[
R=\max_{\rho\in\Pi}\mu_\rho,
\qquad
U_n^\star=\max_{\rho\in\Pi}U_{\rho,n}.
\]

Then

\[
\boxed{
\Pr\!\left(
\exists n:R>U_n^\star
\right)\le\delta.
}
\]

### Proof

Choose a deterministic true maximizer

\[
\rho^\star\in\arg\max_{\rho\in\Pi}\mu_\rho,
\]

using a fixed tie rule. It depends on the underlying belief, not on the
sample.

For every \(n\),

\[
U_n^\star\ge U_{\rho^\star,n}.
\]

Therefore

\[
\{R>U_n^\star\}
\subseteq
\{\mu_{\rho^\star}>U_{\rho^\star,n}\}.
\]

Taking the union over all \(n\),

\[
\left\{\exists n:R>U_n^\star\right\}
\subseteq
\left\{\exists n:
\mu_{\rho^\star}>U_{\rho^\star,n}
\right\},
\]

whose probability is at most \(\delta\). ∎

### What the theorem does not require

It does not require:

- independence between branches;
- positive association;
- a union bound over policies;
- a fixed empirical winner;
- a serialized policy DAG;
- simultaneous coverage of every branch.

It does require:

- a fixed policy class for the evidence epoch;
- an attained maximum, guaranteed here by finiteness;
- a valid branchwise upper process;
- a fixed target world law;
- no data-dependent mutation of policy identities inside the epoch.

## 1.4 Corollary M2 — one empirical optimum count is enough

For one root action \(a\), define

\[
X_{\rho,i}=D_\rho(\omega_i),
\qquad
S_{\rho,n}=\sum_{i=1}^nX_{\rho,i}.
\]

Let

\[
S_{a,n}^\star
=
\max_{\rho\in\Pi_a}S_{\rho,n}.
\]

The empirical split-reach solver already computes this maximum exactly over
the sampled empirical measure, provided it uses one information-consistent
policy across all sampled worlds.

By monotonicity,

\[
\max_{\rho\in\Pi_a}
U_{\delta,N}(S_{\rho,n},n)
=
U_{\delta,N}(S_{a,n}^\star,n).
\]

Thus define the nested E3 bound

\[
\boxed{
\widehat R^{E3}_{a,n}
=
\min_{1\le t\le n}
U_{\delta,N}(S_{a,t}^\star,t).
}
\]

The minimum makes the reported bound nonincreasing without changing its
coverage:

\[
\boxed{
\Pr\!\left(
\exists n:
R_a>\widehat R^{E3}_{a,n}
\right)\le\delta.
}
\]

The fact that the empirical maximizing policy may switch at every \(t\) is
irrelevant. The proof compares \(S_{a,t}^\star\) with the count of one fixed
true maximizing policy.

### Load-bearing implementation condition

The empirical solver must compute

\[
\max_{\rho\in\Pi_a}
\sum_{i=1}^n D_\rho(\omega_i)
\]

with one information-consistent \(\rho\) shared across all worlds.

If the action may be chosen separately per world, the computed count is the
clairvoyant fused E2 count, not the E3 count.

## 1.5 Directional E3 bounds

Apply the same construction to

\[
X^+_{\rho,i}
=
\mathbf 1\{u_1(\rho,\omega_i)=1,\ u_0(\rho,\omega_i)=0\},
\]

and

\[
X^-_{\rho,i}
=
\mathbf 1\{u_1(\rho,\omega_i)=0,\ u_0(\rho,\omega_i)=1\}.
\]

Let

\[
S^{+,\,\star}_{a,n}
=
\max_{\rho\in\Pi_a}
\sum_{i=1}^n X^+_{\rho,i},
\]

\[
S^{-,\,\star}_{a,n}
=
\max_{\rho\in\Pi_a}
\sum_{i=1}^n X^-_{\rho,i}.
\]

Then

\[
\widehat R^{+,E3}_{a,n}
=
\min_{t\le n}
U_{\delta_+,N}(S^{+,\,\star}_{a,t},t),
\]

\[
\widehat R^{-,E3}_{a,n}
=
\min_{t\le n}
U_{\delta_-,N}(S^{-,\,\star}_{a,t},t)
\]

are valid directional uppers at their declared risks.

The maximizing positive-correction policy and maximizing negative-correction
policy may differ. That is already allowed by the definitions of
\(R_a^+\) and \(R_a^-\).

## 1.6 Relationship to the fused E2 baseline

Define the per-world fused indicator

\[
D_a^\star(\omega)
=
\max_{\rho\in\Pi_a}D_\rho(\omega).
\]

Its prefix success count is

\[
F_{a,n}
=
\sum_{i=1}^nD_a^\star(\omega_i)
=
\sum_{i=1}^n
\max_{\rho\in\Pi_a}D_\rho(\omega_i).
\]

For every stream prefix,

\[
S_{a,n}^\star
=
\max_\rho\sum_iD_\rho(\omega_i)
\le
\sum_i\max_\rho D_\rho(\omega_i)
=
F_{a,n}.
\]

Therefore, pathwise,

\[
\boxed{
\widehat R^{E3}_{a,n}
\le
\widehat R^{E2\text{-sample}}_{a,n}
}
\]

when both use the same \(U_{\delta,N}\) and risk.

No stochastic ordering or positive-dependence argument is needed. This is
the exact max-versus-sum-of-max inequality.

The directional fused indicators are

\[
C_a^{+,\,\star}(\omega)
=
\max_{\rho\in\Pi_a}
\mathbf1\{u_1=1,u_0=0\},
\]

\[
C_a^{-,\,\star}(\omega)
=
\max_{\rho\in\Pi_a}
\mathbf1\{u_1=0,u_0=1\}.
\]

They require running the per-world fused branch to a decided terminal, as the
brief anticipated.

## 1.7 Exact worked example

Take four equiprobable worlds and two information-consistent policies:

\[
D_{\rho_0}=(1,1,0,0),
\qquad
D_{\rho_1}=(0,0,1,1).
\]

Then

\[
R=\max(d_{\rho_0},d_{\rho_1})=\frac12.
\]

On the stream

\[
\omega_0,\omega_1,\omega_2,\omega_3,
\]

each fixed policy has two successes, so

\[
S_4^\star=2.
\]

The fused indicator is one on every world, so

\[
F_4=4.
\]

Take \(\delta=1/4\), threshold \(1/\delta=4\), and the exact grid
\(G_4=\{0,1/4,1/2,3/4,1\}\).

For \(s=f=2\),

\[
E^-_{2,2}(3/4)
=
E^>_{2,2}(1/4)
=
\frac13+\frac12+\frac3{10}
=
\frac{17}{15}
<4.
\]

Thus \(3/4\) is not rejected. The point \(1\) is rejected because two failures
were observed. Hence

\[
\widehat R^{E3}_4=\frac34.
\]

For the fused sequence, all four observations are successes, so \(1\) remains
possible:

\[
\widehat R^{E2}_4=1.
\]

Therefore

\[
\boxed{
R=\frac12
\le
\widehat R^{E3}_4=\frac34
<
\widehat R^{E2}_4=1.
}
\]

The companion program exhaustively checks every two-policy Boolean table on
four worlds and every length-four stream. The worst finite-horizon
undercoverage probability is

\[
\frac{11}{128}
<
\frac14,
\]

and E3 is pathwise no looser than fused E2 on every checked prefix.

## 1.8 Where the risk payment remains

The no-\(|\Pi_a|\) result is **inside one scalar maximum**.

The screen may consume many separate claims:

- one positive bound per root action;
- one negative bound per root action;
- baseline value intervals;
- optional equivalence claims.

Those claims must be jointly valid on the screen event. Their risks still need
a declared allocation whose sum is at most the screen budget.

The theorem removes the policy-class penalty inside \(R_a^\pm\). It does not
erase the ledger across distinct screen inputs.

## 1.9 Exact E4 and typing

A sampled E3 upper may exceed the exact E4 value. At its declared failure
probability it may also under-cover. It must be typed as

```text
RootActionExposureUpper {
    action,
    direction,
    upper,
    delta,
    stream_epoch,
    prefix_length,
    policy_class_id,
    method = EmpiricalOptimumUpperCS
}
```

An exact E4 result replaces it with \(\delta=0\). The sampled result is never
described as tighter than E4 merely because a realized number happens to be
smaller.

## 1.10 Verdict on the proposed routes

- **Baseline (0): sound.** A mean upper on the fused per-world indicator is
  admissible and remains a useful cheap fallback.
- **Route (a): valid but unnecessarily expensive if it splits
  \(\delta\) across policies.** The max-preserving theorem removes that split.
- **Route (b): wrong orientation for an upper bound if implemented as a
  branch mixture against \(R\le x\).** It produces lower evidence on the
  maximum.
- **Route (c): realized.** The empirical information-consistent optimizer
  supplies \(S_n^\star\); the existing one-mean upper engine supplies the
  admissible upper.

---

# Part 2 — Structural hazard bounds

## 2.1 There is no canonical weakest local exchange predicate

“Higher tile,” “wins the current trick,” and “extracts trump” are not by
themselves dominance rules.

Winning earlier may:

- change who leads;
- force a partner to spend or withhold count;
- expose or conceal information;
- remove an entry;
- alter future follow obligations;
- cross or fail a contract threshold.

A local tile-order comparison cannot be the general answer.

The exact boundary is reachability in the paired product system.

## 2.2 The paired hazard system

Fix two frozen focal policies \(a,b\), one field \(\sigma\), and one root.

For each world \(\omega\), run both executions in lockstep by global play
index. A paired state is

\[
z=(\omega,s_a,s_b),
\]

where \(s_a,s_b\) are the complete game states of the two executions.

Because transitions, policies, and field are deterministic, there is one
paired successor map

\[
T_{a,b,\sigma}:Z\to Z
\]

on nonterminal states.

Define the hazard terminal set

\[
\mathcal H
=
\{z:\ z\text{ terminal},\ u_a(z)=0,\ u_b(z)=1\}.
\]

Then

\[
H(a\mid b)
=
\beta\{\omega:
\text{the paired trajectory from }\omega
\text{ reaches }\mathcal H\}.
\]

Thus

\[
\boxed{
H(a\mid b)=0
\iff
\operatorname{Reach}(\mathcal I,T_{a,b,\sigma})
\cap\mathcal H
=
\varnothing,
}
\]

where \(\mathcal I\) is the initial paired-state set over the belief support.

## 2.3 The Hazard-Exclusion Invariant

### Definition

A **Hazard-Exclusion Invariant** is a decidable paired-state predicate
\(\mathcal S\subseteq Z\) satisfying:

1. **Initial coverage**
   \[
   \mathcal I\subseteq\mathcal S.
   \]

2. **Forward closure**
   \[
   z\in\mathcal S,\ z\text{ nonterminal}
   \Longrightarrow
   T_{a,b,\sigma}(z)\in\mathcal S.
   \]

3. **Terminal safety**
   \[
   z\in\mathcal S,\ z\text{ terminal}
   \Longrightarrow
   u_b(z)\le u_a(z).
   \]

### Theorem H1 — soundness

If a Hazard-Exclusion Invariant exists, then

\[
\boxed{H(a\mid b)=0.}
\]

### Proof

Every initial paired state lies in \(\mathcal S\). Forward closure keeps the
entire deterministic paired trajectory in \(\mathcal S\). At terminal,
terminal safety excludes \(u_a=0,u_b=1\). This holds for every world in the
belief support, so the hazard event has zero mass. ∎

### Theorem H2 — semantic completeness

If \(H(a\mid b)=0\), the exact reachable set

\[
\mathcal S
=
\operatorname{Reach}(\mathcal I,T_{a,b,\sigma})
\]

is a Hazard-Exclusion Invariant.

Therefore the invariant condition is not merely sufficient. On a finite
game it is equivalent to zero hazard.

The incomplete part is not the mathematics. It is the chosen **witness
language** used to represent \(\mathcal S\) compactly.

## 2.4 The machine-checkable witness object

A practical witness should be a finite symbolic proof DAG:

```text
HazardExclusionWitness {
    root_id
    policy_a
    policy_b
    field_id
    cells: [SymbolicPairedCell]
    initial_cover
    successor_obligations
    terminal_implications
}
```

A cell is a structural predicate over:

- the common physical world constraints;
- both branch histories;
- remaining hands;
- leader and current trick;
- banked score or threshold slack;
- any declared transport between residual positions.

The verifier checks:

1. the exact fiber has zero worlds outside the initial cell cover;
2. every cell’s deterministic paired successor is in one listed successor
   cell;
3. every terminal cell satisfies \(u_b\le u_a\).

The first check may use the exact fiber counter to prove the complement empty.
It need not enumerate every world. The second check is rules-level symbolic
closure. Discovery may be expensive; verification is intended to be cheap.

This is the correct type-level route:

```text
StructuralHazardZero {
    witness_hash
    root_id
    policy_a
    policy_b
    field_id
    hazard_upper = 0
    delta = 0
}
```

A sampled object can never inhabit this type.

## 2.5 A cheap first producer: One-Round Trump-Extraction Witness

The general invariant verifier should be the authority. A first producer can
target a common strict-dominance shape.

### Sufficient hypotheses

At a focal lead with two remaining tricks:

1. Policy \(a\) leads \(h\), the highest remaining trump.
2. Policy \(b\) leads \(d\), a nontrump vulnerable tile.
3. Every hostile hand contains at most one remaining trump.
4. If no hostile trump is available, no hostile same-suit card can beat \(d\).
5. Under the declared field, a hostile seat that is void in \(d\)’s suit and
   holds trump uses a trump on \(d\).
6. Leading \(h\) forces every hostile trump holder to follow, so no hostile
   trump remains after the trick.
7. Losing the \(d\)-trick makes \(b\)’s Boolean contract impossible.
8. In the no-hostile-trump cell, success of \(b\) implies success of \(a\)
   under the declared residual continuation.

These conditions define a two-cell structural partition:

- `NoHostileTrump`
- `OneHostileTrump`

and a tiny Hazard-Exclusion Invariant.

### Why the hypotheses restore the exchange argument

The naive assertion “highest trump is stronger” is not the proof.

The proof needs:

- exhaustive removal of the only threat;
- residual safety of the vulnerable tile;
- a contract condition that turns the lost vulnerable trick into failure;
- a paired residual implication in the no-threat case.

Those are exactly the points at which general trick-taking exchange arguments
usually break.

## 2.6 Exact worked example

Consider a two-player, two-trick game.

- Trump suit: \(T\).
- Focal hand: \(H\) (highest trump), \(D\) (high off-suit card).
- World \(\omega_0\): opponent holds \(L\) (low trump) and \(X\) (irrelevant
  off-suit).
- World \(\omega_1\): opponent holds \(X,Y\), no trump.
- Must follow suit.
- If void in the led suit, the field uses a trump when available.
- Focal payoff is one exactly when the focal seat wins both tricks.
- Belief is uniform on \(\{\omega_0,\omega_1\}\).

Policy \(a\): lead \(H\), then \(D\).

Policy \(b\): lead \(D\), then \(H\).

### World \(\omega_0\)

Under \(a\), \(H\) forces \(L\), wins, and leaves \(D\) safe. Thus \(u_a=1\).

Under \(b\), the opponent trumps \(D\) with \(L\). The focal seat may win the
last trick with \(H\), but has already lost one trick, so \(u_b=0\).

### World \(\omega_1\)

No trump threatens \(D\). Both orders win both tricks:

\[
u_a=u_b=1.
\]

Therefore

\[
H(a\mid b)=0,
\qquad
B(a\mid b)=\frac12.
\]

Policy \(a\) strictly dominates \(b\).

The companion program verifies both the structural producer and the exhaustive
outcome table.

## 2.7 Explicit non-coverage

Now give the focal seat three cards:

\[
H,\ M,\ D,
\]

where \(H\) and \(M\) are the two highest trumps.

In the threat world the opponent holds two lower trumps \(L_1,L_0\) and an
off-suit card.

Policy \(a\):

\[
H,\ M,\ D.
\]

Policy \(b\):

\[
D,\ H,\ M.
\]

Policy \(a\) extracts the two hostile trumps in two rounds and then cashes
\(D\). Policy \(b\) exposes \(D\) before extraction and loses it.

Again,

\[
H(a\mid b)=0,
\qquad
B(a\mid b)=\frac12.
\]

But the One-Round Trump-Extraction Witness declines because a hostile hand may
contain more than one trump.

This is intended. A more expressive multi-phase Hazard-Exclusion Invariant can
prove the dominance; the cheap one-round producer cannot.

## 2.8 Composition across fields

Two independent witnesses under \(\sigma_0\) and \(\sigma_1\) prove dominance
under those two fields separately.

A stronger reusable witness may quantify over a **field action family**:
at every non-focal information state, closure must hold for every action that
either field may choose. One such robust witness proves zero hazard for both
fields and any selector inside that family.

That is useful, but strictly stronger and potentially much harder to produce.
There is no automatic cross-field dominance theorem from a witness under only
one field.

---

# Part 3 — A motif vocabulary over first-split traces

## 3.1 The present domain is correction traces

The stated trace producer persists `FieldSplitTrace` for worlds where

\[
u_0\ne u_1.
\]

Therefore a motif partition over the present records partitions

\[
C^+\ \dot\cup\ C^-,
\]

the terminal correction event.

It does **not** partition all exposed worlds \(D=1\), because exposed worlds
with \(u_0=u_1\) have no trace record under the stated schema.

This distinction must travel with every aggregate:

- “31/1200 positive correction worlds have motif \(M\)” may be exact.
- “Motif \(M\) accounts for this fraction of field exposure” is unsupported
  unless non-pivotal exposed worlds are also classified.

## 3.2 Root semantics must be resolvable

The local structural predicates require:

- declaration/trump;
- the current trick frame and led context;
- partnership map and focal seat;
- rule version.

These are not all literal fields of the trace. They are recoverable only if
`root_id` resolves to an immutable root-semantic record.

The classifier should therefore require:

```text
(root_id, root_semantics_hash) -> immutable RootFrame
```

or the trace schema should carry a pinned root snapshot.

If the lookup fails, classification returns `Other(reason=missing_root_frame)`.
It must not guess.

## 3.3 The local split signature

Let the common record before the split be \(r\), the acting seat be \(s\), its
private hand be \(H_s\), and the two field choices be \(t_0,t_1\).

For a candidate tile \(t\), define:

1. **Next led context**
   \[
   L(t)=
   \begin{cases}
   \ell(t),&\text{if the split actor leads the trick},\\
   \text{current led context},&\text{otherwise}.
   \end{cases}
   \]

2. **Immediate control**
   \[
   W(t)=
   \text{partnership of the provisional trick winner after }r\cdot t.
   \]

3. **Committed payload**
   \[
   C(t)=\text{count value carried by }t.
   \]

4. **Trump commitment**
   \[
   T(t)=\mathbf1\{t\text{ is effective trump}\}.
   \]

5. **Residual suit shape**
   \[
   S(t)=
   \bigl(
   |(H_s\setminus\{t\})\cap q|
   \bigr)_{q\in\text{effective contexts}}.
   \]

6. **Played strength**
   \[
   K(t)=
   \text{declaration-relative trick key of }t
   \text{ in the active context}.
   \]

The ordered signature is

\[
\Sigma(t)=(L(t),W(t),C(t),T(t),S(t),K(t)).
\]

## 3.4 The primary alphabet

The primary motif is the first differing coordinate of
\(\Sigma(t_0)\) and \(\Sigma(t_1)\).

### M1 — `LeadContextFork`

\[
L(t_0)\ne L(t_1).
\]

**Reading.** The fields choose different contexts to establish on lead. This
changes every follower’s legal set and the public meaning of the trick.
It is the most immediate structural fork, but not by itself a causal
explanation of the terminal correction.

### M2 — `ImmediateControlFork`

The led context agrees, but

\[
W(t_0)\ne W(t_1).
\]

**Reading.** One field spends enough strength or trump to change which
partnership provisionally controls the trick; the other does not.

### M3 — `CountCommitmentFork`

Context and immediate control agree, but

\[
C(t_0)\ne C(t_1).
\]

**Reading.** The fields agree on immediate control but differ on whether and
how much count payload is released into the trick. This is “cash versus
retain” morphology, not proof that count timing caused the final result.

### M4 — `TrumpCommitmentFork`

The earlier coordinates agree, but

\[
T(t_0)\ne T(t_1).
\]

**Reading.** One field spends trump while the other preserves it, without
changing the immediate partnership control or the already-prioritized count
coordinate.

### M5 — `SuitShapeFork`

The earlier coordinates agree, but

\[
S(t_0)\ne S(t_1).
\]

**Reading.** The two plays leave different effective-suit length profiles:
different future voids, follow obligations, discard flexibility, and
signaling opportunities.

### M6 — `StrengthCommitmentFork`

The earlier coordinates agree, but

\[
K(t_0)\ne K(t_1).
\]

**Reading.** The plays have the same coarse context, control, count, trump
status, and residual suit shape, but spend different rank strength.

### Residual — `Other`

No declared coordinate differs, or the required root/rule derivation is
unavailable.

**Reading.** The present local alphabet does not explain the split at this
resolution. No nearest readable label is substituted.

## 3.5 Orthogonal flags

The first-difference rule gives a primary partition, but several coordinates
may differ simultaneously.

Every trace should therefore also emit the derived Boolean flags

```text
diff_context
diff_control
diff_count
diff_trump
diff_suit_shape
diff_strength
```

and, when the root frame is available,

```text
split_actor_relation = partner | opponent
terminal_sign = favors_field1 | favors_field0
```

The flags may co-occur. They are not a second authority; they are direct
derived views of the same signature pair.

## 3.6 Coverage and exclusivity

The primary labels are mutually exclusive by construction: the classifier
returns the first differing coordinate in one fixed ordered tuple.

They are exhaustive over records for which all signature coordinates are
defined, with `Other` as the residual.

The ordering is a taxonomy convention, not a causal ranking. A
`LeadContextFork` may also differ in count and trump commitment; the flags
retain those facts.

### No invented residual-rate forecast

The abstract game model does not imply a numerical residual rate. Assigning
one now would be an arbitrary magic number.

The first corpus run should report:

- residual fraction overall;
- residual fraction by root grade;
- residual fraction by field pair;
- the most common raw signature pairs inside `Other`.

A high residual rate may mean:

- the alphabet omitted a common local axis;
- the mechanism is genuinely nonlocal;
- the root resolver lacks necessary semantics;
- locally equivalent actions produce different downstream policies.

Those are different scientific outcomes. The residual is an instrument, not
an embarrassment.

## 3.7 Aggregate typing

For exact full-fiber correction traces, define

\[
m_k^+
=
\Pr(M=k,\ u_1=1,u_0=0),
\]

\[
m_k^-
=
\Pr(M=k,\ u_1=0,u_0=1).
\]

Then

\[
r_k=m_k^++m_k^-,
\qquad
c_k=m_k^+-m_k^-.
\]

These are exact evidence-bearing decompositions when:

- the fiber is enumerated exactly;
- every pivotal world is present exactly once;
- the motif predicate is total and deterministic;
- root and field identities are fixed.

They satisfy

\[
\sum_k m_k^+=c^+,
\qquad
\sum_k m_k^-=c^-,
\qquad
\sum_k c_k=c.
\]

When \(r_k>0\), the exact conditional directional tilt is

\[
\tau_k=\frac{c_k}{r_k}.
\]

### Sampled-prefix motif outputs

On a sampled prefix, raw motif histograms are descriptive unless accompanied
by a declared valid inference process.

They do not feed the screen merely because the motif names are structural.

### Aggregates I would refuse to publish

I would not publish:

- “motif \(k\) caused the win”;
- “motif \(k\) is good/bad play” pooled across roots;
- field-exposure mass by motif from correction-only traces;
- dominance labels from sampled motif hazards;
- unweighted motif rates pooled across different fibers, bids, fields, or
  policy identities.

The safe phrasing is:

> “Among exact correction worlds for this root, field pair, and frozen policy,
> the first mechanical split had motif \(k\) on mass \(m_k\).”

## 3.8 Why `RevealResponse` is not currently decidable

At the first split, the fields choose different public actions. That alone
does not show:

- which downstream seat’s policy changed because of the observation;
- whether the relevant information was about trump, count, or suit shape;
- whether the first split was a but-for cause of the terminal correction;
- whether a later divergence would have occurred anyway.

The current trace contains the first split and final outcomes, but not the two
post-split public suffixes.

Therefore `RevealResponse` would be a seductive judgment label, not a derived
predicate.

## 3.9 Minimal schema enrichment for a second-layer response vocabulary

Persist raw derived-from-execution data, not motif tags:

```text
branch0_suffix: [(actor, tile), ...]
branch1_suffix: [(actor, tile), ...]
root_semantics_hash
```

The suffixes are short in a finite trick-taking hand and remain replayable.

From them an offline classifier may derive:

- first later focal action divergence;
- first later partner action divergence;
- whether the two branches next reach the same seat in comparable trick
  coordinates;
- the two information-state keys, legal sets, and selected actions there.

A later second-layer label might be

```text
PartnerResponseCandidate
```

only when a formal predicate identifies a later partner action difference
following the split.

Even then, call it a response **candidate**, not a causal attribution.

A true but-for label would require an intervention replay, for example:
force the field-1 execution to take the field-0 action at the first split,
then continue under field 1 and test whether the terminal correction
disappears. That is a separate producer.

---

# 5. Recommended implementation order

1. **Build the Part-1 E3 producer first.**  
   The empirical information-consistent split-reach optimizer already
   computes the load-bearing count \(S_n^\star\). The new work is the exact
   upper-CS inversion, prefix minimum, result type, and risk wiring.

2. **Add directional E3 with separate ledger entries.**  
   Positive and negative empirical objectives are separate solves and may
   have different maximizing policies.

3. **Introduce the general Hazard-Exclusion Invariant verifier before a
   library of named patterns.**  
   Pattern producers should emit witnesses for one authority, not each invent
   their own dominance semantics.

4. **Implement One-Round Trump Extraction only as the first incomplete
   producer.**  
   Its refusal path is part of its correctness.

5. **Ship the six-motif morphology classifier over current correction traces.**
   Keep `Other`. Emit all coordinate-difference flags.

6. **Do not ship `RevealResponse` yet.**  
   First add post-split suffixes or another equally raw replayable schema.

7. **Mechanize three small theorems early.**
   - max-preserving upper-CS theorem;
   - Hazard-Exclusion Invariant soundness;
   - first-difference motif partition.

These are compact, general, and independent of Texas 42’s detailed rule
algebra.

---

# 6. Proof ledger

## P1 — one-mean grid inversion

1. `[USES: lower-tail e-process anytime crossing]`  
   At the true attainable mean \(p\in G_N\),
   \[
   \Pr(\exists n:E^-_n(p)\ge1/\delta)\le\delta.
   \]

2. `[USES: definition of U_{\delta,N}]`  
   If \(p>U_{\delta,N}(S_n,n)\), then \(p\) was rejected at \(n\), so
   \(E^-_n(p)\ge1/\delta\).

3. `[USES: steps 1–2]`  
   \[
   \Pr(\exists n:p>U_{\delta,N}(S_n,n))\le\delta.
   \]

## P2 — max-preserving upper confidence

1. `[USES: finiteness]`  
   Choose fixed \(\rho^\star\in\arg\max_\rho\mu_\rho\).

2. `[USES: maximum definition]`  
   \[
   U_n^\star=\max_\rho U_{\rho,n}\ge U_{\rho^\star,n}.
   \]

3. `[USES: R=\mu_{\rho^\star}]`  
   \[
   \{R>U_n^\star\}
   \subseteq
   \{\mu_{\rho^\star}>U_{\rho^\star,n}\}.
   \]

4. `[USES: branch anytime coverage]`  
   Union over \(n\) has probability at most \(\delta\).

## P3 — empirical optimum collapse

1. `[USES: definition of S_n^\star]`  
   \(S_n^\star\ge S_{\rho,n}\) for every \(\rho\).

2. `[USES: upper endpoint monotonicity in s]`  
   \[
   U_{\delta,N}(S_n^\star,n)
   \ge
   U_{\delta,N}(S_{\rho,n},n).
   \]

3. `[USES: P2 with true maximizer]`  
   The empirical-optimum endpoint is a same-\(\delta\) upper CS for \(R\).

4. `[USES: intersection over prefixes]`  
   Taking the prefix minimum preserves coverage and makes the bound
   nonincreasing.

## P4 — E3 no looser than sampled fused E2

1. `[USES: max-sum inequality]`  
   \[
   \max_\rho\sum_iX_{\rho,i}
   \le
   \sum_i\max_\rho X_{\rho,i}.
   \]

2. `[USES: endpoint monotonicity]`  
   Applying \(U_{\delta,N}\) preserves the inequality.

3. `[USES: prefix minimum]`  
   The nested E3 bound is pathwise no larger than the nested fused-E2 bound.

## P5 — Hazard-Exclusion Invariant

1. `[USES: initial coverage]`  
   Every paired initial state is in \(\mathcal S\).

2. `[USES: forward closure; induction on finite play depth]`  
   Every state on every paired trajectory remains in \(\mathcal S\).

3. `[USES: terminal safety]`  
   No terminal trajectory has \(u_a=0,u_b=1\).

4. `[USES: definition of hazard mass]`  
   \(H(a\mid b)=0\).

## P6 — motif partition

1. `[USES: total ordered signature]`  
   Every classifiable trace has two finite six-coordinate signatures.

2. `[USES: least differing index]`  
   If a coordinate differs, there is a unique least differing index and
   therefore one unique primary motif.

3. `[USES: residual definition]`  
   If no coordinate differs or derivation is unavailable, the trace maps to
   `Other`.

4. `[USES: cases 2–3]`  
   Every trace gets exactly one primary label.

---

# MACHINE-CHECKABLE ARTIFACTS

`PART 1 ANSWER: CONSTRUCTION (max-preserving empirical-optimum upper CS)`

`PART 2 ANSWER: CONDITION (Hazard-Exclusion Invariant; One-Round Trump-Extraction producer)`

`PART 3 ANSWER: ALPHABET (6 motifs + residual)`

The deterministic exact-rational companion program is:

```text
verify_deferred_producers_triple_v0_1.py
```

Its checks include:

- every two-policy Boolean table on four worlds;
- every length-four stream for each table;
- exact finite-horizon undercoverage probability;
- pathwise E3-versus-fused-E2 comparison;
- the exact \(R=1/2,\ E3=3/4,\ E2=1\) worked example;
- the high-trump strict-dominance example;
- the deliberate non-coverage example;
- twenty trace dictionaries and the complete primary motif partition.

Expected final line:

```text
ALL CHECKS PASS
```
