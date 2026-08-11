# Equivariant Controlled Lumpability over Declared Role Interfaces

**Status:** basis amendment draft, **v0.5 track** — exploratory tier, below every
project evidentiary tier; the claim discipline of `unified_information_geometry_v0.4.md`
applies unchanged
**Date:** 2026-08-10
**Author:** Jason Yandell (statement, definitions, theorem, corollaries, and proofs
delivered in-session and recorded verbatim up to the mechanical LaTeX transport
repairs listed in Appendix B)
**Extends:** v0.4 §12.6. The v0.4 document remains frozen and authoritative for
everything it states; this amendment reads as **§12.6A**, between v0.4 §12.6 and
§12.7. A future consolidated v0.5 reconstruction should integrate it in place.
**Formalization intent:** prose-and-LaTeX source for later proof-assistant
decomposition; no theorem here is machine checked merely because a proof is written.

## Why this amendment exists

v0.4 §12.6 defines strong controlled lumpability with **literal** equality of
legal-action and observation interfaces inside one descriptor class, and its
feature alphabet names concrete tiles. Under that raw-interface reading, two
situations differing only by tile identity can never merge, and the S4 synthesis
measurement found exactly what that predicts: only world-reconstructing skeletons
pass. The correct quotient condition for the lossless compression program is
**equivariant** — the interfaces (actions, observations, roles, features) are
compared under declared typed transports, and outcomes are compared under the
count-free quotient. Two situations are the same when, given what the seat knows
and does not know, one policy applied through the transports to any matching
world produces the same outcome under the quotient. This amendment states that
condition and proves the corresponding exact-compression theorem, recovering
v0.4 §12.6 as the identity-interface case.

---

# 12.6A Equivariant controlled lumpability over declared role interfaces

The strong lumpability theorem above uses literal equality of legal-action and
observation interfaces inside one descriptor class. Scheme semantics is more
general: two concrete states may realize the same relational state through
different chair, context, or domino denotations, provided those interfaces are
related by declared typed transports.

The correct quotient condition is therefore equivariant rather than literally
label preserving.

## Interface-decorated carrier

Fix a finite role schema

\[
\Sigma=(N_Q,N_C,N_D)
\]

and a declared output interface

\[
O_\Sigma\subseteq\Sigma.
\]

Work after the active equality-pattern quotient. Thus names identified by the
equality pattern have already been merged, and the remaining output names are
interpreted injectively within each sort. If different equality patterns may
occur, the descriptor must retain enough information to determine the resulting
quotient-interface type.

For each latent state \(x\in X\), let

\[
\rho_x
\]

be the declared concrete interpretation of \(O_\Sigma\).

This interface must be functionally instantiated on the carrier. It may arise
from a unique Scheme answer, from a declared selector incorporated into the
latent state, or from some other explicit interface construction. A merely
existential multi-answer Scheme fiber does not by itself define \(\rho_x\).

Names in \(\Sigma\setminus O_\Sigma\) remain internal witnesses and acquire no
persistence or transport merely because they participated in the proof of a
Scheme case.

Let

\[
d:X\to Y
\]

be a candidate dynamic descriptor.

## Declared interface transports

Whenever

\[
d(x)=d(y),
\]

assume a declared typed interface transport

\[
\Theta_{xy}
=
\left(
\Theta^Q_{xy},
\Theta^C_{xy},
\Theta^D_{xy}
\right)
\]

between the concrete realizations of the output roles, determined on represented
objects by role-name correspondence:

\[
\Theta^Q_{xy}(\rho_x(q))=\rho_y(q),
\]

\[
\Theta^C_{xy}(\rho_x(c))=\rho_y(c),
\]

\[
\Theta^D_{xy}(\rho_x(e))=\rho_y(e).
\]

These maps need only be defined on the declared represented interface unless a
larger transport is required by the action or observation language. They are not
asserted to be global symmetries of Straight 42.

Require the usual coherence laws

\[
\Theta_{xx}=\mathrm{id},
\]

\[
\Theta_{yx}=\Theta_{xy}^{-1},
\]

and

\[
\Theta_{xz}
=
\Theta_{yz}\circ\Theta_{xy}
\]

whenever \(d(x)=d(y)=d(z)\).

The chair transport must also carry the declared partnership and orientation
convention. In particular, \(e_\star\) always denotes the trick coordinate of
the transported focal partnership.

Assume further declared bijections

\[
\Theta^A_{xy}:A(x)\simeq A(y)
\]

on legal action labels and

\[
\Theta^{\mathrm{obs}}_{xy}:
\operatorname{Obs}(x)\simeq\operatorname{Obs}(y)
\]

on observation labels.

For output roles declared rigid through a step, the interface transports must
commute with the rigid role update. If observation \(o\) leads to successor
representatives \(x',y'\) of one abstract successor, then on the rigid
subinterface

\[
\boxed{
\Theta^\Sigma_{x'y'}\circ r_o
=
r_{\Theta^{\mathrm{obs}}_{xy}(o)}
\circ
\Theta^\Sigma_{xy}.
}
\]

Fresh successor roles are re-evaluated at the successor and make no claim of
predecessor identity. Their equality is governed by the successor descriptor
semantics rather than by the rigid square.

## Count-free controlled kernel

Let the preserved immediate outcome alphabet be only the count-free trick
component

\[
R_\star
\subseteq
\mathbb N e_\star.
\]

At a primitive play step this is normally \(\{0,e_\star\}\): zero unless the
step completes a trick won by the focal partnership, and \(e_\star\) when it
does.

Write

\[
K_a(x;k,o,x')
\]

for the joint kernel of count-free increment

\[
k\in R_\star,
\]

next observation \(o\), and successor latent state \(x'\).

No physical-domino capture coordinate is part of this primitive lumpability
contract.

## Definition — equivariant strong controlled lumpability

The descriptor \(d\), together with the declared interface transports
\(\Theta\), is equivariantly strongly controlled-lumpable for the count-free
role interface when, for every \(x,y\in X\) with

\[
d(x)=d(y),
\]

the following hold.

First, legality is preserved up to the declared action transport:

\[
\boxed{
A(y)=\Theta^A_{xy}(A(x)).
}
\]

Second, for every \(a\in A(x)\), count-free increment \(k\in R_\star\),
observation \(o\), and abstract successor \(z\in Y\),

\[
\boxed{
\sum_{x':\,d(x')=z}
K_a(x;k,o,x')
=
\sum_{y':\,d(y')=z}
K_{\Theta^A_{xy}(a)}
\left(
y;
k,
\Theta^{\mathrm{obs}}_{xy}(o),
y'
\right).
}
\tag{ECL}
\]

Thus the distribution of

\[
(\text{count-free increment},
\text{transported observation},
\text{successor descriptor})
\]

depends only on the current descriptor and on the transported abstract action,
not on the chosen concrete representative.

Let

\[
\bar a=[x,a],
\qquad
\bar o=[x,o]
\]

denote the action and observation classes induced by the declared transports.

Then

\[
\bar K_{\bar a}(d(x);k,\bar o,z)
:=
\sum_{x':\,d(x')=z}
K_a(x;k,o,x')
\]

is well defined.

---

## THEOREM — equivariant controlled lumpability

Suppose the continuation is finite and graded, and \(d\) is equivariantly
strongly controlled-lumpable for the count-free role interface.

Then, for every initial belief \(\beta\) on \(X\), every positive-probability
transported observation history, and every policy measurable with respect to
the abstract observation record and abstract belief:

1. the pushed belief

\[
\bar\beta=d_\#\beta
\]

updates exactly using \(\bar K\) alone;

2. lifting an abstract policy through the declared action transports produces a
lawful concrete policy, and every such lifted policy induces exactly the same
joint law in the concrete and abstract systems for

\[
\left(
\bar o_1,\ldots,\bar o_n,
\sum_i k_i,
d(x_n)
\right);
\]

3. the rigid output-role trace is well defined on the quotient: changing
concrete representative changes only its declared concrete realization, not its
abstract role history;

4. consequently the concrete and abstract systems induce the same law of the
count-free terminal outcome

\[
\boxed{
\Phi_T^\star(c)=t_T(c)e_\star;
}
\]

5. more generally, every terminal statistic readable from the preserved
transported role trace and count-free terminal outcome has the same law in the
concrete and abstract systems;

6. every bounded utility of those preserved quantities has the same value under
corresponding policies;

7. optimization over the corresponding abstract-policy class preserves every
root value and action value:

\[
\boxed{
V_{\mathrm{quot}}=V_{\mathrm{exact}},
\qquad
Q_{\mathrm{quot}}(\bar a)
=
Q_{\mathrm{exact}}(a).
}
\]

**Proof**

Condition (ECL) makes the definition of \(\bar K\) independent of the chosen
representative \(x\) of a descriptor state, because replacing \(x\) by \(y\)
and transporting \(a\) and \(o\) leaves every aggregate successor probability
unchanged.

Push the initial concrete belief through \(d\). Assume inductively that the
abstract belief equals the pushforward of the concrete posterior at some
recursion grade. An abstract policy chooses an abstract action class
\(\bar a\); the action bijections select the corresponding legal concrete
action at every representative in the current descriptor fiber.

For every current representative, (ECL) gives the same joint law of count-free
increment, transported observation, and next descriptor. Summing over the
concrete posterior therefore gives exactly the transition law obtained from
\(\bar K\). Conditioning on any positive-probability transported observation
and normalizing preserves the pushforward equality. Induction on the finite
continuation grade proves exact filtering and equality of the full abstract
observation/count-free-outcome law.

The commuting rigid-transport square shows simultaneously that the denotation
of every rigid output role follows the same abstract role history regardless of
the chosen representative. Fresh roles are determined by successor semantics
and hence by the successor abstract state. Therefore every terminal quantity
defined from the preserved role trace and the accumulated \(e_\star\)
coordinate has the same law.

Policy-value equality follows by integrating any bounded utility against this
common law. Maximizing over the same transported abstract-policy class, with or
without a fixed root action, gives equality of \(V\) and \(Q\). ∎

---

## Role re-entry of tile features

The preceding theorem is deliberately count-free. It does not require the
primitive quotient kernel to carry the 28 physical capture coordinates.

Tile anisotropy may instead re-enter through declared domino roles.

Let

\[
O_D\subseteq O_\Sigma
\]

be the domino part of the transported output interface, after equality-pattern
quotient. For a rigid domino role \(e\in O_D\), suppose the preserved
role/observation trace determines whether the physical tile denoted by that
role was captured by the focal partnership.

Define the quotient role-capture coordinate

\[
\bar x_{T,e}(\bar c)
=
\mathbf1[
\text{the transported tile occupying role }e
\text{ is captured by }T
].
\]

Then the role-indexed additive outcome is

\[
\boxed{
\bar\Phi_{T,O}(\bar c)
=
t_T(\bar c)e_\star
+
\sum_{e\in O_D}
\bar x_{T,e}(\bar c)e_e.
}
\]

This is not a return to physical-world coordinates. The coordinate \(e_e\)
names a transported role, and its concrete physical denotation is supplied by
\(\rho_x(e)\).

For coefficients

\[
b\in\mathbb R,
\qquad
\lambda:O_D\to\mathbb R,
\]

define

\[
\bar P_{T,b,\lambda}(\bar c)
=
b\,t_T(\bar c)
+
\sum_{e\in O_D}
\lambda(e)\bar x_{T,e}(\bar c).
\]

The corresponding physical valuation in representative \(x\) is the role
pullback

\[
w_x(\rho_x(e))=\lambda(e).
\]

Hence role coefficients are invariant under change of representative even when
the physical domino occupying that role changes.

---

## COROLLARY — valuation gauge descends to the quotient

Assume the domino-role interface is capture-complete for the selected valuation
mode: after equality-pattern quotient, every tile whose capture belongs to the
selected additive outcome occurs exactly once as a rigid domino role, with any
required banked or unresolved-trick residue included consistently.

Then every legal quotient terminal outcome satisfies

\[
\boxed{
\sum_{e\in O_D}\bar x_{T,e}=4t_T.
}
\]

Therefore, for every scalar \(c\),

\[
(b,\lambda)
\sim
(b-4c,\lambda+c\mathbf1)
\]

defines the same quotient score, and valuation factors through

\[
\boxed{
(\mathbb R\times\mathbb R^{O_D})
/
\langle(-4,\mathbf1)\rangle.
}
\]

In particular,

\[
\bar P_{T,b,\lambda}
=
\bar P_{T,b-4c,\lambda+c\mathbf1}
\]

on every legal quotient terminal outcome.

For every gauge class

\[
[(b,\lambda)],
\]

the exact and quotient policy values coincide:

\[
\boxed{
Q_{\mathrm{quot}}(\bar a;[(b,\lambda)])
=
Q_{\mathrm{exact}}(a;[(b,w_x)]),
}
\]

and

\[
\boxed{
V_{\mathrm{quot}}([(b,\lambda)])
=
V_{\mathrm{exact}}([(b,w_x)]).
}
\]

**Proof**

Capture completeness gives the same four-tiles-per-won-trick conservation law
as the physical additive feature:

\[
\sum_e\bar x_{T,e}=4t_T.
\]

Hence

\[
\begin{aligned}
(b-4c)t_T
+
\sum_e(\lambda(e)+c)\bar x_{T,e}
&=
bt_T+\sum_e\lambda(e)\bar x_{T,e}\\
&\quad
+c\left(\sum_e\bar x_{T,e}-4t_T\right)\\
&=
bt_T+\sum_e\lambda(e)\bar x_{T,e}.
\end{aligned}
\]

Thus evaluation depends only on the gauge class. Role-capture coordinates are
readable from the transported interface trace, so the equivariant lumpability
theorem preserves their joint terminal law. Integrating and optimizing gives
the displayed equality of \(Q\) and \(V\). ∎

---

## Fixed physical valuations and the stabilizer boundary

The role-coordinate result does not imply that an arbitrary fixed physical
valuation

\[
w:\mathcal D\to\mathbb R
\]

is invariant under every declared domino transport.

A fixed physical valuation descends through one descriptor class only when

\[
\boxed{
w(\Theta^D_{xy}(d))=w(d)
}
\]

for every valuation-relevant represented tile \(d\), equivalently when the
declared transport lies in the stabilizer of \(w\) on the represented carrier.

If this fails, then the physical valuation distinguishes representatives that
the count-free quotient intentionally identifies. Exact valuation-parametric
compression then requires one of three things:

- transport the valuation with the role interface;
- retain the distinguishing valuation label in the descriptor;
- or refine the descriptor class.

Thus the dynamic quotient is fundamentally valuation-free. Tile anisotropy
re-enters through transported roles, and the §8 additive gauge acts only after
that role-indexed valuation interface has been declared.

---

**[COROLLARY — §12.6 as the identity-interface case]**
The strong controlled-lumpability theorem of §12.6 is recovered by taking every
interface, action, and observation transport to be the identity and omitting
the role-indexed readout.

---

# Claim ledger — v0.5 amendment

- **[THEOREM — proved here]** Equivariant controlled lumpability (statement and
  prose proof above; not machine checked).
- **[COROLLARY]** Valuation gauge descends to the quotient under
  capture-completeness; §12.6 recovered as the identity-interface case.
- **[BOUNDARY]** Value equality in conclusion 7 is over the transported
  abstract-policy class, exactly as in v0.4 §12.6's conclusion 4. Whether the
  unrestricted concrete optimum is attained inside that class is a separate
  sufficiency question, deliberately not claimed here.
- **[BOUNDARY]** The transports are declared per descriptor class; nothing here
  asserts global symmetries of Straight 42 (cf. v0.4 §11.1, §11.7).
- **[OPEN]** Existence of a nontrivial pair \((d,\Theta)\) satisfying (ECL) on
  real Straight 42 kernels; the class-count census on the existing probe corpus
  is the designated first measurement. This sharpens v0.4 §17.4 open problems
  3–5.

# Appendix A — review notes (recorded by walt, 2026-08-10; commentary, not part of the authored text)

1. **Coherence scope.** The coherence laws are stated for \(\Theta\); for the
   induced classes \(\bar a=[x,a]\) and \(\bar o=[x,o]\) to be well-defined
   equivalence classes, the same laws must hold for \(\Theta^A\) and
   \(\Theta^{\mathrm{obs}}\). Adopted reading: "the usual coherence laws" scope
   over all declared transports. Flagged for author confirmation.
2. **Automatic coherence on represented objects.** Because \(\Theta_{xy}\) is
   determined by the role-name correspondence
   \(\Theta(\rho_x(n))=\rho_y(n)\), the coherence laws hold automatically on
   the represented interface; the requirement has independent force only on any
   declared extension beyond represented objects (e.g. as required by the
   observation language).
3. **Observation-language extension.** Concrete observation tokens can name
   tiles outside the represented interface; the authored text already provides
   for this ("unless a larger transport is required by the action or
   observation language"). Any implementation must declare that extension
   explicitly.

# Appendix B — transport repairs

The authored text was delivered through a channel that dropped some `=` signs
at display-math line breaks and some subscript underscores. The following
mechanical, content-preserving repairs were made; no wording, symbol choice, or
mathematical content was altered:

- restored `=` in: the definition of \(\Theta_{xy}\); the composition coherence
  law; the rigid-transport square; condition (ECL); conclusion 7's
  \(Q\)-equality; the definitions of \(\bar x_{T,e}\), \(\bar\Phi_{T,O}\),
  \(\bar P_{T,b,\lambda}\); the corollary's \(Q\)/\(V\) equalities and gauge
  equality of \(\bar P\);
- restored subscript underscores in \(\Theta^{\mathrm{obs}}_{xy}\) and
  \(\Theta^\Sigma_{xy}\) inside the rigid square, and in \(d_\#\beta\);
- normalized `[...]` display delimiters to `\[...\]` and repaired `\\` line
  breaks inside the corollary proof's aligned block;
- typographic spacing (`b\,t_T`, `x':\,d(x')=z`) as in v0.4 house style.
