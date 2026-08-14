# walt mathematics — the freeze register

[Home](Home.md) · owns: the register of walt's determinism freezes 1–43 —
number, content, and the ruling that fixed each · Sources:
`walt/CENSUS-RULINGS.md` (every freeze is declared in a ruling there);
cross-checked against the `Freeze NN:` doc comments in
`walt/walt-factory/examples/*.rs` and the results-file headers. Related:
[the reference map](walt-math-reference.md),
[information geometry](walt-math-information-geometry.md),
[decision-sparse witnesses](walt-math-decision-sparse.md).

> **Tier: EXPLORATORY throughout**, below every tier on
> [Home](Home.md#evidentiary-tiers--never-promoted-never-blurred).

## What a freeze is, and what it is not

A **freeze** is a declared constant, encoding or ordering that a quoted number
depends on. Freezes exist so that a reported figure is reproducible from the
repository alone. They are *not* mathematics: nothing is proved by freezing it,
and a freeze may be replaced by a later adjudication.

Three standing rules, and they are the reason the register exists at all:

- **Numbers are never reused.** A spent number stays spent even when the build
  it served is parked. Freezes 18–21 belong to the parked seat-census build;
  R-A22 declined to recycle them, precisely because S-A2 cites "freeze 18"
  inside the licensing argument of Lemma S-fold.
- **Freeze-relativity is declared, not assumed.** A predictive dimension is
  freeze-independent; the basis, the closure matrices and every sparsity figure
  are freeze-dependent (R-A21). Which side of that line a number falls on is
  part of what the results file says.
- **A stored artifact carries the freeze-set digest.** A record whose digest
  differs from the running freeze set is **corrupt, not stale**: the cache is
  discarded **entire**, never partially reused (DS-A30, X-A6(i), P-A17, E-A18).

## The register

All 43 numbers are accounted for. "Fixed at" names the declaring ruling.

| # | Content | Fixed at |
|---|---|---|
| 1 | The content-addressed class encoding — a class identity is the 128-bit FNV-1a hash of its signature bytes, and a signature names successors by *their* hashes, so identity is a function of the future cone alone | r3 Q4/Q5.3 |
| 2 | The per-state canonical move order — moves sorted by (increment, classification, successor class hash), ties broken by the state's concrete tile order | r3 Q5.3 |
| 3 | The yard tree encoding — step and handoff-leaf byte forms, children sorted by (increment, classification, child encoding) | yard v1 pass, registered at P-Q6 |
| 4 | The shape canonical form — leaf colours refined to a fixpoint, then the minimum encoding over orderings still tied, with a declared ceiling past which the run STOPS rather than approximating | yard v1 pass, registered at P-Q6 |
| 5 | The suffix cut — a depth-*d* suffix replaces everything below by a hole carrying that subtree's exact **interned** identity (interning, not hashing); the equality pattern over holes is recomputed locally inside each suffix | shape v2, registered at P-Q6 |
| 6 | The open variant — at unconstrained nodes only, options deduplicated by (increment, classification, child suffix) after children are already in open form, bottom up | shape v2, registered at P-Q6 |
| 7 | The fiber enumeration order, stated precisely enough to reproduce an index → world map (hidden slots in offset order, each slot's k-combinations in lexicographic order, slot 0 outermost) | P-A18 |
| 8 | The decimation rule (g, W) per rung, **for the fiber probe (S5h)** — indices {i·g mod N}, gcd(g,N) = 1 asserted in-run, no prefix, no RNG. Distinct from freeze 25, which is the predictive-rank track's own constants | P-A18 |
| 9 | The fold weighting definition — an exact integer, stated offset-from-focal, never orientation-flavoured | P-A18 (definition at P-A13) |
| 10 | The operator and the valuation | P-A18 |
| 11 | The per-arm key functions — the packed semantic-state key at trick boundaries, and the r3 128-bit signature | P-A18 |
| 12 | Each exclusion predicate's definition, declared **intensionally** — never as a list of class hashes, which are themselves freeze-dependent — printing the set size beside the definition | X-A11 |
| 13 | The flag keying: a flag is keyed by (predicate id, freeze-set id) | X-A11 (keying at X-A6(i)) |
| 14 | The store record format and its freeze-set digest | X-A11, first implemented at E-A19 |
| 15 | The canonical-form key definition and its byte encoding — internal to r1 before, a persistent key now, therefore a freeze | E-A19 |
| 16 | The floor domain and its closed-form count | E-A19 |
| 17 | The declared coordinate order for the warm arm and the receipt stride | E-A19 |
| 18 | The seat-side form: the holder sort, the S-A1 invariant list, **the S-A2 comparison reading** (winner-determining order per live context, tier-0 tiles collapsed to one bottom class), and the byte encoding | S-A19 |
| 19 | The declaration-fold maps and whether the run reports folded, unfolded, or both (both mandatory) | S-A19 |
| 20 | The interface-element encoding — the ordered play record — and the record and hand enumeration orders | S-A19 |
| 21 | The landing form's observation content: the support-normal-form void encoding, the leader-offset encoding, and the empty declaration set | S-A19 |
| 22 | The information-interface encoding — live set, capacity-cell system, leader offset, grade, empty output interface and no monitor, no accumulated outcome — plus the coordinate enumeration order | R-A22 |
| 23 | The closure discipline — primitive-step granularity, per-contract terminal seed sets, the deterministic pivot rule (first nonzero in kernel world enumeration order), basis storage order, index convention | R-A22 |
| 24 | The observation-label encoding, with the derived transition tuple γ = (leader offset from focal; three followers' follow/slough classifications; count-free increment) | R-A22 (γ defined at R-A13) |
| 25 | The decimation constants (g, W) per grade, **for the predictive-rank track (S6a) and everything built on its coordinates (S6b, S6c, S6d)** — implemented `[(7919,12), (104729,6), (1299709,3)]`. Same P-A15 form as freeze 8, different constants and a different track: do not cross-wire them | R-A22 |
| 26 | **The concrete authority**: the H solver and its version, budget, valuation, fiber weighting and observation model — `ScalarHidden::action_values_dag` (dag-v1), `trick_only`, uniform fiber weighting, observation contract = the full public record, `AUTHORITY_BUDGET = 200_000_000`; **the bridge Q_diff = 2·Q_count − grade**; **the tie rule: least domino index among the argmax** | R-A22; the bridge and tie rule ratified as freeze-26 content at SEP-A3(iii)/SEP-A8 |
| 27 | The vector encoding — world order = the kernel world order of freeze 23, exact rationals — and the dedup order | PG-A14 |
| 28 | The dominance-check order and the incremental-fold order over observation branches — a determinism freeze because the stop point depends on it | PG-A14 |
| 29 | The exposure programme and its pivot rule — exact-rational primal simplex with Bland's rule | PG-A14 (programme at PG-A9) |
| 30 | The caps: per-interface frontier cap, per-partial-sum cap, per-coordinate budget, and the grade-3 conditionality rule | PG-A14 |
| 31 | The policy-counting convention — plans versus reduced | PG-A14 (convention at PG-A3) |
| 32 | The detector predicates — D0 in Proposition J-0's form (including the potential-leader quantifier and the still-leadable-context definition), D1-sym in J-1's form, D1-win in J-win's form — and their bitset encodings. Explicitly: **no exhaustion-margin constant exists to freeze** | J-A16 |
| 33 | The detector call sites and the charging rule | J-A16 (rule at J-A13) |
| 34 | The ground-truth classifier: which denominator, computed how | J-A16 (per J-A10) |
| 35 | The harvest arms, rungs, coordinates, budget unit, and the control's solver identification | J-A16 |
| 36 | **The candidate-policy library v1** — see below. **Now v2**: clause (e)'s transport is opened from identity-only to identity plus the declaration fold | **SEP-A4** (reserved at DS-A13); **(e) amended at EC-A8 — freeze 36 v2** |
| 37 | **The action-conditioned upper witness and its solver identification** — see below | **SEP-A6** (reserved at DS-A13) |
| 38 | The gluing-cut language, the validity-proof obligation, and the cut ordering (a determinism freeze because the stop point depends on it) — **RESERVED, untouched** | DS-A13, confirmed reserved at SEP-A18 |
| 39 | The circuit representation and its evaluation order — exact rational arithmetic is order-insensitive in value, but reported node and operation counts are not — **RESERVED** | DS-A13 |
| 40 | The reachable-belief family defining W_reach, with its deal-level typing — **RESERVED** | DS-A13, as revised by DS-A23 |
| 41 | The checkpoint record format and its freeze-set digest; a record whose digest differs is **corrupt, not stale**, and the cache is discarded entire | DS-A36 (detail at DS-A30(i)) |
| 42 | The unit identity and the canonical assembly order | DS-A36 |
| 43 | The sequential timing rung's selection rule and its W = 1 requirement — declared before the parallel pass, by rule over the canonical unit order, **never by result** | DS-A36 (discipline at DS-A33/A34) |
| 44 | **The walk-step unit and the budgeted-walk contract**: one walk-step per (particle, node) visit, charged `bag.len()` at each `walk` entry before any child call; `walk` takes `budget: &mut u64` and returns `Option`, charge-then-descend, and on exhaustion **no partial fold of any kind is retained**; the stop point is a function of (kernel, budget) alone. Extends the `Option` contract to all six §3.2 evaluators. Constants: **B = 10,000,000,000** walk-steps per (coordinate, action) for the per-action traversals, **4B** whole-call for `revealed_summary`, **P_max = 32,000,000** partition states per (coordinate, action), and the §5 rung's decimation constant **g = 15,485,863** — a fresh prime, deliberately not a freeze-25 constant (the no-cross-wiring clause applies) | N4-A1 |
| 45 | **The n = 4 coordinate identity**: grade 4, declaration pip, the viewer's hand and pool as canonical ascending-domino-index tile lists, leader offset from focal asserted **0**, \|X\| = 34,650 asserted against `kernel.count()`, and the freeze-7/23 fiber enumeration order. Corpus hand id and trick number are **provenance only**, never identity components; the kernel is rebuilt in-run from the printed identity and asserted equal. **No library entry is written at any n = 4 coordinate** | N4-A3 |
| 46 | **The economy-successor arm list, CLOSED**: X (exact control, the H-argmax seed recomputed in-pass — receipts, not measurements, g = 0 by Corollary E4.1(2)); T (transport, the four library entries by φ to p′ = 6 plus idx = 0 to p′ ∈ {1..5} — receipts under Corollary S-fold-val); P1 least-tile, P2 greatest-tile, P3 beat-if-able, P4 trump-hoard; and R, the heuristic re-key, labelled **HEURISTIC RE-KEY (NOT A TRANSPORT)** on every row. Plus the transport and image-key construction, the canonical run order, and the results-file column set. An open arm list is not a freeze: a later arm is a **freeze-46 v2** fixed by a later adjudication | EC-A1 |

## Freezes 36 and 37 in full

These two were reserved by DS-A13 in the intake audit and fixed by the
Experiment E adjudication. They are the newest freezes and the ones a successor
is most likely to need.

### Freeze 36 — the candidate-policy library, v1 (SEP-A4)

**(a) Key:** (grade, base index, declaration ∈ {0..6}, root action) under the
S6a unranking; **pip is derived** from the index by that unranking and is a
printed field, never a key component. **(b) Body:** a total map from observation
record to chosen tile over the built information partition, serialised as
(observation record, chosen tile) pairs sorted lexicographically by record under
the canonical ascending domino-index order, the record being the plays since the
kernel decision point with the root action first; **the in-process information
state handle never appears in a stored entry**. **(c) Frame, mandatory on every
entry:** observation contract, field, belief, |X|, and the freeze-set digest.
**(d)** Stored content is a policy and its provenance only — no value, rank,
verdict or dominance status; the file is a cache, never an authority; a loaded
entry is re-priced before use; a digest mismatch is corruption and the file is
discarded entire. **(e) Transport:** identity only in v1 — **amended to v2 at
EC-A8**, see below. **(f) Seed rule:** the
argmax-recording pooled H solve over the same partition, **unmemoized**, with
freeze 26's tie rule **cited, not restated**; the seed contributes no number to
any reported L. **(g)** The DS-A16 header note: entries remain valid
primal-witness sources under count re-entry; their count-free quality verdicts
do not survive.

**Freeze 36 v2 (EC-A8) — clause (e) only.** Transport becomes: *identity, and
the declaration fold φ_{p→p′} of Lemma S-fold — image key computed by the
freeze-46(b) construction, R9 receipts asserted in-run, values licensed by
Corollary S-fold-val and verdict transport by Lemma E7 with β′ = T_*β. Any
further transport re-enters with its own adjudication.* Note what was
**rejected**: the class formulation, which would have admitted "transports with
an exhibited isomorphism" as a class — a freeze is a constant, not a rule, and a
class clause would delegate future adjudications to the freeze. Conditions:
transported candidates are in-process objects; **no image entry is written to
the library file** in the successor run, which stays at its four entries; a
transported policy is re-priced before anything is reported. Numbers are never
reused — versioning content by ruling, as here, is the pattern (freeze 46(e)
cites this precedent for its own future v2).

### Freeze 37 — the action-conditioned upper witness (SEP-A6)

**(a) Evaluator:** U_a := the per-root-action column of the revealed summary,
read at the declared direction, identified as E_β[V*_a]. **(b)** The relaxation
is named treatment **C**, not C⁺ — on this carrier the latent is ξ = ω and the
two coincide. **(c)** The declared direction is the count-free focal trick
differential; the reporting convention is the count convention and the freeze-26
bridge is asserted exactly **at the reporting boundary only**; the bridge is
affine with positive slope, so verdicts are convention-invariant. **(d)** Belief
uniform over the full enumerated fiber, identical on both sides; **no decimated
world set appears inside any L or U**. **(e)** Conditions (C1)–(C4) asserted
in-run. **(f)** The per-action price is the existing per-root continuation-price
column, asserted nonnegative; its aggregate siblings are named once and never
confused with it. **(g)** The envelope and scalar H solvers are in the **same
units** and are asserted **equal exactly, with no bridge**; the root is asserted
trick-leading so their action lists coincide. **(h)** Budget honesty: the scalar
authority is budgeted and its exhaustion is a declared stop; the envelope path
carries no budget and no stop, and the results file says so in place.

## Two discrepancies on the record

Both were found by cross-checking the rulings against the code and the published
results headers. Neither has been resolved by editing anything, per the
project's ambiguity protocol.

1. **Freeze 1 vs 2 ordering.** The r3 Q5.3 passage names them in the order
   "canonical move order and content-addressed encoding", but the numbering in
   the implementation and in the X-Q7 citation is the reverse: content-addressed
   encoding is freeze 1, canonical move order is freeze 2. The code and X-Q7
   agree with each other; the prose ordering in r3 Q5.3 is the odd one out.
2. **Freeze 2's sort key.** The census-era results headers describe the order's
   first component as `k` — the r3, t5, pruned, yard and yard-v2 files all do —
   while the implementation and the fiber-probe results header say `increment`.
   The two names denote the same quantity in the census-era files, which define
   `k` as the count-free increment, but the register records the divergence
   because the freeze is quoted by name across both eras. **The code is
   authoritative.**
