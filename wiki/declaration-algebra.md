# Declaration Algebra

[Home](Home.md) · owns: the domino universe, the nine algebras, transports and
mechanics classes · Sources: both packages Math §§2–3, **rec Math §3.10** (rec-only).
Related: [rules-profile](rules-profile.md), [support-fiber](support-fiber.md).

## The universe

- Pips `P = {0..6}`; dominoes are 2-multisets: `|D| = C(8,2) = 28`
  [Theorem — proved, Math §2.1].
- Natural pip incidences `σ_p = {d : p ∈ d}` have size 7; doubles lie in exactly one,
  mixed tiles in exactly two; `σ_p ∩ σ_q = {p:q}` — a **covering, not a partition**
  [Theorem — proved, Math §2.2].
- **rec-only view [Definition/Corollary — proved, rec Math §2.1–2.2, ALG-20]**:
  `D = Sym²(P)` is exactly the edge set of complete looped `K₇`; natural suits are
  closed stars. **[ALG-21, Theorem — proved]**: count is the *antidiagonal decoration*
  — `c(d) = sum(d)` exactly when the sum is 5 or 10, else 0 (hence 10+10+5+5+5 = 35).

## Declaration = selection of a relational algebra

A declaration δ (7 pip trumps, DT, NT) determines called set `κ_δ`, powered set `π_δ`,
effective suits `σ̂_q^δ` (called tiles absorbed into suit 7), led context `ℓ_δ`,
follow relation `F_δ`, total rank `r_δ`, contextual tier (2 powered / 1 follower /
0 slough), and lexicographic trick key `τ_δ` [Definitions, Math §3.2–3.5].
Declaration is *selection of a relational interpretation, not a scalar feature*
[Corollary/Synthesis, Math §3.8]. A physical domino is a stable node whose strategic
type is declaration-relative — the package thesis.

**Unique trick winner** [Theorem — proved, Math §3.6]: any four distinct dominoes with
a designated lead have a unique maximal trick key (lead is never tier 0; ranks are
injective within the winning tier). Checked against an independently coded prose-rule
resolver on all `9·28·C(27,3) = 737,100` cases
[Theorem — exhaustive finite verification, ALG-12].

`BEATS_δ(q,d)` is exactly the set of keys above `d`'s in context `q`
[Theorem — proved, Math §3.7]; live-threat removal is monotone [ALG-14] but threat is
**not** a complete ontology — in NT, `0-0` and `1-1` both have empty when-led threat
sets yet follow different suits [Constructed counterexample, ALG-15].

## Transports and symmetry classes

- **Count-preserving pip permutations**: exactly identity and the swap `2↔3`
  [Theorem — proved + finite verification over all 5,040 permutations, Math §3.9,
  ALG-17]. The swap induces a *game-order* isomorphism exactly between declaration
  layers 2 and 3 (and no others), transporting order but not literal numeric ranks
  [ALG-18/19].
- **rec-only [ALG-22/23/24, Theorems — proved + finite verification, rec Math §3.10]**:
  drop the count decoration and literal rank labels ("unscored mechanics") and *all
  seven pip trumps become isomorphic* via the order-preserving complement map; the nine
  declarations then fall into exactly **three unscored mechanics classes** — pip trump,
  doubles-trump, no-trump — separated by the invariant
  `(|powered|, #{unpowered tiles with one effective suit})` = (7,6) / (7,0) / (0,7).
  All 49 ordered pip-trump transports verified (307,328 contextual order comparisons,
  `verify_reduced_kernel.py`).

The three-class result is why a solver's *legality/mechanics* code paths need only
three cases, while anything touching count (bidding, valuation) collapses only under
`2↔3`. The unscored transport moreover bijects *reachable support images* across pip
trumps — `f_{t,u}(R_t) = R_u`, exchange-adjudicated CONFIRMED (x:004), collapsing the
reachable census from nine tags to three classes — see [reachability](reachability.md).

## Lead contexts (used heavily by reachability)

Every declaration has exactly **seven leadable contexts**, and the seven lead fibers
partition the 28 tiles with cardinality multiset `{1,…,7}` [Theorem — proved, Math
§7.13.2, REACH-05]. Doubles-trump's natural context 0 is nonempty as a *follow* set
but unleadable — a genuine reachability reduction, so void masks need 7 bits/seat,
not 8.
