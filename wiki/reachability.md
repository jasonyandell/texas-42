# Strict Straight Reachability

[Home](Home.md) · Sources: both packages Math §7.13 (shared), **rec Math §7.13.7**
(rec-only). Related: [minimal-support-normal-form](minimal-support-normal-form.md),
[support-dynamics](support-dynamics.md), [open-problems](open-problems.md).

**Feasibility is not reachability.** Hall answers "does a hidden assignment exist
now?"; reachability asks "could a valid deal plus a legal actor-attributed prefix have
produced exactly this support?" The reachable support image `R_Str^m` is the image of
legal contracted-hand prefixes under the feasible normal form [Definition, REACH-01].

## Structure theorems (all shared)

- **Viewer gauge** [Corollary — proved, REACH-01A]: seat rotation bijects all four
  viewer-indexed domains; a viewer-relative code stores no absolute viewer id.
- **Reachable-domain minimality** [Theorem — proved, REACH-02]: restricting the normal
  form to legal prefixes deletes unrealized classes but never lets two distinct
  reachable fibers merge.
- **No runtime flag** [Corollary — proved, REACH-03]: states built by legal
  constructors are reachable by induction; external states need exact validation,
  after which tag and witness are erasable. v0.7 sharpens this into the
  *proof-irrelevance* clarification (witnesses never refine game equality) — see
  [discrepancies D1](discrepancies.md).
- **50 capacity profiles** [Theorem — proved + finite verification, REACH-04]: a
  hidden-capacity triple occurs iff `max − min ≤ 1`; exactly `8 + 7·6 = 50` labeled
  profiles, so capacities are *derived from trick progress*, never three free fields.
- **Seven leadable contexts** per declaration with lead-fiber sizes `{1..7}`
  [Theorem — proved, REACH-05] ([declaration-algebra](declaration-algebra.md)).
- **Schedule language** [Theorem — proved (projection-exact), REACH-06]: with `j`
  completed tricks, at most `j` distinct void contexts, or `j+1` when the newest is
  held only by already-acted current-trick followers `F(B)`; census counts `A_j`,
  `T_{j,1}`, `T_{j,2}` verified [REACH-06A].
- **Lead-witness necessity** [Theorem — proved, REACH-07]: every used void context
  must have a lead-fiber tile already outside the hidden pool.
- **Exact witness criterion + decidability** [Theorems — proved, REACH-08/09]:
  a support is reachable iff a replay witness (deal + contract + legal prefix +
  claimed normal form) validates; membership is decidable by (impractically large)
  exhaustive search.

## Feasible-but-unreachable witness

**[Constructed counterexample + finite verification, Math §7.13.5, REACH-10]**
Capacities (6,6,6); 18-tile pool `σ₀ ∪ doubles ∪ {2:1,3:1,3:2,4:1,4:2}`;
`P₁ = U \ σ₀`, `P₂ = P₃ = U`. Hall-feasible and already reduced — yet among all 450
static zero/one-void-context generators only two decode to it (zeros-trump called-7
and NT context-0, both with only seat 1 void), and in both the *entire lead fiber is
inside the hidden pool*, violating lead-witness necessity. Hence
`R_Str ⊊ N(feasible)`: reachability is not capacities, not schedule, not Hall.

## Second feasible-but-unreachable witness — and the follower-supply obstruction

**[Exchange-adjudicated CONFIRMED — program executed ALL_PASS; 3/3 adversarial referees
SOUND; a new evidentiary tier, not a corpus "Theorem — proved" and not a kernel proof]**
Witness `(NT, capacities (6,6,6), V₁={6}, 18-tile pool)`. Unlike REACH-10, this support
*passes* all four outer necessary checks — capacity shape, schedule admissibility,
**lead-witness**, and Hall — yet is still unreachable: exhausting all 450 static
generators yields exactly three decoder matches and, across all 425,520 trace
candidates, **0 realizers**. So the outer necessary language is *not tight* even at the
`j=1` equal-capacity one-void phase, settling [open-problems](open-problems.md) Q2
negatively. Source: `exchange/inbox/002-outer-language-tightness.md`, verified program
`exchange/adjudication/programs/002.py` (16/16 PASS, 0.9s); three referees each
re-verified by an independent method (1,276,560-trace single-layer enumeration; a
301,860-state recursive game DFS with max-flow feasibility; corpus ID/integer
cross-check).

**Follower-supply obstruction** — a new fifth, mechanically-checkable necessary
condition beyond the four outer checks: a singleton hidden void in context `q` at the
`(6,6,6)` phase requires at least two distinct `σ_q` tiles outside the used pool `U`;
here `σ₆ \ U = {6:6}` supplies only one, which is exactly what kills this witness.
Adding this condition can tighten the REACH-11 46-bit ceiling below (quantifying how
far is open new work, not yet done).

**Independent cross-language reproduction (rob, Rust):** rob's `verify_unreachable`
binary re-runs this witness through its own outer validators — 4/4 classic checks pass,
3 of 450 generators decode, 425,520 shallow candidates yield 0 realizers, and the fifth
follower-supply check rejects it (`x-r_unr_002_*` lines in
`rob/receipts/verify_unreachable.txt`; see [verification](verification.md) §"rob (Rust)
— independent reproduction, slice 02"). This is conformance evidence for the
exchange-adjudicated result, **not** a status upgrade; the exchange-adjudicated framing
above remains primary.

## The 35–46-bit interval (the flagship open problem)

*(The corpus-proved interval is [26,46], REACH-11/12; exchange adjudication narrows the
floor to ≥35 bits — REACH-17 below — at the exchange-adjudicated evidentiary tier, not
the kernel-proved tier.)*

- **Ceiling** [Theorem — exhaustive finite verification, REACH-11]: the necessary
  outer language (reachable capacity shape × schedule-admissible void masks ×
  pool with lead witnesses), declaration-tagged, has exactly
  **64,123,542,674,901 < 2⁴⁶** members (7,124,838,074,989 per declaration < 2⁴³;
  largest fixed-profile block < 2⁴⁰). Every reachable support has ≥1 outer profile ⇒
  ≤46 bits standalone. Context-relative: ≤43 with declaration supplied, ≤43 with
  capacities, ≤40 with both, **0 supplemental bits** given a certified mechanical
  state [REACH-11A; CELL-17].
  *Naming*: v0.7 calls these **necessary outer profiles** (they may decode to
  infeasible/unreachable supports — not certificates); rec's older "outer
  certificates" naming is deprecated ([discrepancies D3](discrepancies.md)).
- **Floor** [Theorem — proved + finite verification, REACH-12]: four disjoint
  universally reachable no-void families — pools at capacities (7,7,7), (6,7,7)×3,
  (6,6,7)×3, (6,6,6) — give `C(28,21) + 3·C(28,20) + 3·C(28,19) + C(28,18)` =
  **44,352,165 > 2²⁵** reachable supports ⇒ ≥26 bits. (The (6,6,6) construction uses a
  pigeonhole: a 10-tile complement with ≤2 doubles has ≥18 pip incidences over 7 pips,
  so some pip sits on ≥3 tiles, providing a legal 3-play prefix.)
- **Sharpened floor** [exchange-adjudicated CONFIRMED — program executed ALL_PASS
  (15.9s); 3/3 adversarial referees SOUND; a new evidentiary tier, **not** a corpus
  "Theorem — proved" and **not** a kernel proof, REACH-17]: a certified *disjoint*
  family of **17,668,066,045 > 2³⁴** reachable supports — `559,316,142` no-void +
  `8,387,350,664` called-void + `8,721,399,239` natural-void — proving **≥35 bits**
  standalone and narrowing the standalone interval to **[35,46]**. Verification tier
  (kept visible): reachability and disjointness of the counted family rest on prose
  trace-templates closed by each referee's adversarial replay, **not** on end-to-end
  machine replay. Machine-hardened fallback tiers: discarding all four winning-void-trick
  rows still leaves `14,144,456,893 > 2³³` (i.e. **[34,46]**); the no-void family alone
  (`559,316,142`) gives **≥30 bits**. Ceiling unchanged corpus ground truth (REACH-11).
  Source: `exchange/inbox/001-reachable-support-cardinality.md`, verified program
  `exchange/adjudication/programs/001.py`. (REACH-12's 44,352,165 family remains the
  earlier *corpus-proved* floor of ≥26 bits.)
- **Transport-commutation** [exchange-adjudicated CONFIRMED — program executed ALL_PASS
  (4.6s); 3/3 adversarial referees SOUND; a new evidentiary tier, not a corpus theorem
  and not a kernel proof]: the order-preserving complement transport commutes with
  legal-prefix generation, `f_{t,u}(R_t)=R_u`, so `|R_t|` is independent of the pip
  trump `t` and the declaration-tagged reachable census collapses from nine tags to
  **three classes** (one pip-trump class, doubles-trump `R_DT`, no-trump `R_NT`). Tagged
  census `|R~| = 7·r_pip + |R_DT| + |R_NT|`; depth-0 overlap `≥ C(28,7) = 1,184,040`
  normal forms common to all nine `R_δ`; union bound
  `|R| ≤ 7·r_pip + |R_DT| + |R_NT| − 8·C(28,7)`. Verified anchors: 307,328 ALG-22
  comparisons; 45,472 commutation checks (38,976 nontrivial) over 6,496 prefixes / 224
  deals / depths 0–28 / all 7 trumps; 4/4 injected mutations caught. The Step-15
  quotient-cardinality corollary depended on the cocycle identity
  `f_{u,v} ∘ f_{t,u} = f_{t,v}`; that identity is now discharged by finite check over
  all 343 ordered pip-trump triples on the 28-tile transports (finite verification
  receipt, exchange-side: `exchange/adjudication/programs/004-cocycle.py`, ALL_PASS),
  so the corollary no longer stands CONDITIONAL. Source:
  `exchange/inbox/004-transport-reachability-commutation.md`, verified program
  `exchange/adjudication/programs/004.py`.
  *Independent cross-language reproduction (rob, Rust):* rob's `verify_transport`
  binary reproduces the commutation on a corpus of 588 transported hands — 16,464
  symbolically-accepted transitions and 17,052 depth-wise NF equalities
  `f(N_t(prefix)) = N_u(f(prefix))` — plus the 3-class quotient
  (`rob/receipts/verify_transport.txt`; see [verification](verification.md) §"rob
  (Rust) — independent reproduction, slice 02"). Conformance evidence for
  `f_{t,u}(R_t)=R_u`, **not** a status upgrade; the exchange-adjudicated framing here
  remains primary.
- **[UNRESOLVED, REACH-13 / OPEN-11]**: the exact `|R_Str^m|`, and hence the optimal
  standalone width in `35..46` (exchange-adjudicated; corpus-proved `26..46`), is open.
  Both packages refuse to collapse it by guesswork.

Boundary [REACH-03A + rec TRANS-08]: standalone reachable support is not a complete
game state (no declaration/actor/trick/score); but rec proves support *is* a closed
transition state once declaration and the typed public observation context are
supplied — see [support-dynamics](support-dynamics.md).

## rec-only: symbolic support reachability (deal-free certificates)

**[Theorem — proved, rec Math §7.13.7, REACH-14]** Replay a public trace against the
*support itself*: start from unrestricted 21-tile support; accept a hidden action iff
its typed conditioned successor support is nonempty; accept viewer actions iff legal in
the known hand. A trace is accepted **iff at least one complete deal legally realizes
it**, and the final symbolic support is exactly the trace's rule fiber.
(Completeness: carry a realizing deal forward. Soundness: reverse the typed
transitions from any final world back to a full deal via the fixed-history bijection.)

Corollaries [REACH-15/16]: an exact reachability certificate needs **no hidden deal**
— viewer hand + contract/declaration + leader + attributed trace + claimed normal form
suffices; and the symbolic play/support graph is a finite, remaining-tile-graded DAG
whose support-output image is exactly `R_Str^m`. This DAG is the natural substrate for
ever *counting* `R_Str^m` — see [open-problems](open-problems.md). Receipt: 108
deterministic complete hands, 3,024 transitions replayed symbolically
(`verify_reduced_kernel.py`).
