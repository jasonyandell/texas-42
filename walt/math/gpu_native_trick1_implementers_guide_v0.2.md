# Walt GPU-Native Trick-1 Solver

## Implementer's Guide to Exact Count-Aware Projection, Stopped Scheme/Fix Search, and Metal Wavefront Execution

**Version:** 0.2  
**Date:** 2026-08-16  
**Status:** exploratory implementation design; not a rules or theorem authority  
**Primary target:** Apple Silicon M5 Max, Metal first, unified-memory aware  
**Host language:** Rust, reusing the existing `walt-*` semantic crates  
**Proof authority:** the existing CPU/reference semantics and independently replayable receipts  

**Revision focus:** make the build explicitly hybrid rather than indiscriminately GPU-only; make the next-focal-decision epoch the host-level recursion unit; add an exact meet-in-the-middle coefficient projector; state the lawful information key correctly; strengthen action-indexed pruning; and remove one unsafe implication shortcut from basis canonicalization.  

---

## 0. Executive decision

Build a new GPU-native solver. Do **not** port the recursive CPU solver node for node.

“GPU-native” here does **not** mean “put every task on the GPU.” It means that the state model, exact arithmetic, batch boundaries, and proof receipts are designed so that every regular high-volume stage can stay resident and run efficiently on the GPU. Irregular graph construction, Scheme/Fix synthesis, best-first proof control, and tiny frontiers remain on the CPU unless profiling proves otherwise.

The new solver should treat trick-1 play as a decision-relative exact proof search over five cooperating mechanisms:

1. **A tiny count automaton.** For a fixed contract, carry only the defenders' remaining loss allowance, at most 12 points.
2. **An exact symbolic posterior.** Project the opening trick from 399,072,960 hidden deals into a bounded collection of public responses and exactly weighted posterior objects.
3. **A focal-epoch recursion.** After a focal action, run the fixed field in one bulk stage until the focal seat acts again or the branch terminates. The CPU controls a shallow decision DAG; the GPU absorbs the wide field-only work between decisions.
4. **A stopped Scheme/Fix recursion.** End a branch as soon as a lawful plan, a contract bound, a tax certificate, or an exact endgame leaf settles its contribution.
5. **A heterogeneous bulk engine.** Use GPU wavefronts, coefficient projectors, sorting, and exact reductions for homogeneous work; use the CPU for irregular symbolic construction and proof scheduling; place the mandatory information-set “net” exactly where the focal player must make one common decision.

The production objective is not to solve every state and not to find the most globally faithful compression. It is:

> **Minimize total processor work required to prove the root action.**

The proof terminates when one root action has a lower bound strictly above every competitor's upper bound. Every optimization in this guide is subordinate to that inequality.

---

## 0.1 Current measured baseline

The implementation is responding to a compounding wall, not one isolated slow routine.

Current exact and exploratory measurements supplied by the project:

- the last-four-tricks survey held about **20.8 million frontier information states** and **723 million arrivals** at full scale;
- one fixed opening hand has **399,072,960** hidden allocations, while the complete ordered-deal universe is roughly **472 trillion**;
- historical July 18 CPU measurements for the earlier exact Walt profile had a severe tail: W1/H4 was about 5 ms at p50, 624 ms mean, 1.9 s at p95, and 153 s maximum, with about 7% of solves consuming 91.5% of time; W0/H4 was roughly 15–30 s at p50, and each added tile was observed to cost on the order of 100–200x;
- under the newer count-aware survey, a 30 bid was already decided in about half of the last-four-trick states, while 42 contracts were decided almost everywhere;
- the smallest-frontier root action was uniquely optimal somewhere between 43.5% and 58.8% of the measured four-action coordinates, against a 25% baseline.

These numbers are engineering baselines and hypothesis generators. They are not transported trick-1 conclusions. Re-measure every performance quantity on the new engine and target machine.

## 0.2 Reference inputs

This guide consolidates and extends:

- `ingest/texas-42-foundations-source-of-truth-v0.7-reconstructed/docs/10_RULES.md`;
- `walt/math/implementers_guide.md`;
- `walt/math/decision_sparse_exact_solving_v0.1.md` and its errata/audit;
- the durable nonanticipativity, second-rung, face-fee, and stopped-Scheme/Fix notes produced in this exchange;
- the current count-pruning, decidedness, move-ordering, and one-trick-predicate summary supplied by the project;
- the earlier Metal-first Walt GPU architecture discussion.

For Metal resource and profiling behavior, consult Apple's current documentation on shared storage, indirect compute dispatch, shader libraries, GPU captures, occupancy, and counter statistics. The implementation must query actual pipeline and device limits rather than freezing assumed hardware constants into the mathematics.

## 0.3 Scope and non-goals

The v0.2 solver target is exact best response for one focal seat and its team under a frozen field profile, using the count-bearing Straight contract objective selected by the request. It is not yet an equilibrium solver against adaptive opponents, and it must not be described as one.

The opening closed form and the integer scale `420` depend on the current **uniform-random-legal** field. A different field profile remains admissible only through an explicitly versioned probability kernel with its own exact arithmetic contract.

The first production objective is binary make/set probability for one actual contract. Expected-points and mark-valued objectives may share the engine later, but they require separately stated utility and width bounds.

The learned omega-to-`Q` model is optional. Exact late-game dynamic programming may be faster for the small carriers encountered by the proof search. No neural component is required for correctness or completeness.

## 0.4 Notation to say aloud

When these symbols appear: `omega` for `ω`, `delta` for `Δ` or `δ`, `lambda` for `λ`, `theta` for `θ`, `pi` for `π`, `gamma` for `Γ`, and `kappa` for `κ`.

---

# 1. Non-negotiable semantic contract

## 1.1 The GPU is an implementation, never a rules authority

The existing `walt-core`, rules corpus, and CPU reference evaluators remain authoritative for:

- effective-context membership;
- legal play;
- trick order and winner;
- count decoration;
- public-record encoding;
- lawful information-set identity;
- field behavior;
- utility and contract settlement.

The GPU consumes frozen lookup tables generated by the CPU authority. It must not independently reinterpret the rules through a second hand-written suit or rank implementation.

Every GPU run carries a semantic digest covering at least:

```text
rules profile
static table hash
declaration
contract and utility profile
field profile
information-key version
Scheme/Fix registry hash
arithmetic scale contract
kernel build hash
```

A changed digest invalidates cached values and receipts.

## 1.2 Exact strategy consistency is structural

At a focal decision, all hidden worlds or symbolic cells that share the same lawful information state must receive one common action.

The GPU must therefore contain an explicit **information-set net stage**:

```text
hidden branches / cells
        ↓ lawful information key
sort or bucket
        ↓
one segment per information state
        ↓
choose one action for the whole segment
```

The lawful information key includes everything the focal player actually knows and the objective actually exposes, including at least:

```text
focal seat
focal current hand
public play record / current trick prefix
leader and seat to act
declaration
publicly known contract state and remaining loss budget
versioned public policy descriptor, if a Plan Scheme uses one
```

It must never contain:

- hidden-world identity;
- symbolic-cell identity;
- hidden holder facts;
- perfect-information `Q` values;
- proof-only Scheme/Fix witnesses;
- a tie-broken clairvoyant action.

Keep two descriptor types distinct:

```rust
struct PolicyDescriptor(...); // computable from focal knowledge and public observations
struct ProofDescriptor(...);  // hidden relational witness; checker-only
```

Only `PolicyDescriptor` may enter the lawful information key or affect a focal action.

A GPU implementation that maximizes independently per world is treatment `C`, not the lawful treatment `H`.

## 1.3 Count-free results do not become count-bearing values

The earlier count-free machinery remains valid as information geometry:

- reveal-delay ladders;
- nonanticipativity gluing;
- first-rung taxes;
- slack–tax adjustment;
- martingale penalties;
- Scheme/Fix semantics;
- stopped quotient reasoning.

Its measured numerical values do not transport to Straight count valuation. The new engine is count-aware from its root utility inward.

## 1.4 No silent approximation

The proof path uses:

- exact integer arithmetic;
- exact combinatorial counts;
- exact legal action masks;
- exact public-state grouping;
- explicit upper and lower intervals.

Approximate models, including the cheap omega-to-`Q` oracle, may order work or propose certificates. They may not silently enter a proof value.

Use distinct types:

```rust
struct ExactMass(...);
struct ExactBound { lo: ExactMass, hi: ExactMass }
struct HeuristicScore(f32);
struct ApproxQ(f32);
```

No implicit conversion from a heuristic type to an exact type exists.

## 1.5 Fail closed

These conditions terminate a batch without a verdict:

- arena overflow;
- arithmetic overflow;
- unsupported basis rank;
- incomplete action set;
- incomplete response enumeration;
- failed mass conservation;
- hash collision not discharged by full-key comparison;
- unsupported field profile;
- receipt mismatch.

The engine may rerun with a smaller batch, wider arithmetic, a different backend, or a larger budget. It may never truncate and continue.

---

# 2. Exact count objective: the 13-state loss automaton

## 2.1 Contract normal form

For a point bid `B`, the declaring team makes the contract exactly when the defenders receive at most

\[
R = 42-B
\]

points. For a mark bid, `R = 0`.

For all ordinary point bids,

\[
0 \le R \le 12.
\]

The hot solver should therefore receive one decision-specific value:

```rust
remaining_loss_budget: u8 // 0..12
```

At each completed trick:

```text
if declaring team wins:
    budget unchanged
else:
    budget -= 1 + count_in_trick
```

A negative budget is exact contract failure. If the declaring team has already banked the threshold, the node is exact success.

## 2.2 Why one budget, not thirteen, is the default

A survey may batch every budget `0..12`, but a production proof asks about one actual contract. Carrying thirteen full exact values per state multiplies memory traffic and reduction cost for no root-decision benefit.

Default:

```text
one coordinate
one contract
one remaining-loss budget
```

Optional survey mode may vectorize budgets after the single-budget engine is correct and profiled.

## 2.3 Sparse count consequences

The only count tiles are `5-5`, `6-4`, `5-0`, `4-1`, and `3-2`, worth `10,10,5,5,5`.

On any successful line under an ordinary bid, the defenders can receive only:

- no count;
- one 5-count tile;
- two 5-count tiles;
- or one 10-count tile.

No 15-count subset fits inside a loss budget of 12.

This should inform pruning and Scheme/Fix searches, but it is not itself a complete continuation model. The exact state remains the remaining budget plus the physical/public state.

## 2.4 Free decidedness bounds

At every lawful information state, count conservation gives a policy-independent interval for the defenders' final score:

\[
D_{\mathrm{banked}}
\le
D_{\mathrm{final}}
\le
D_{\mathrm{banked}}+P_{\mathrm{live}},
\]

where `P_live` is the total point value still unbanked: remaining trick points plus the count values on unplayed count tiles.

Therefore:

- if `D_banked > R`, the contract is already failed;
- if `D_banked + P_live <= R`, the contract is already guaranteed;
- otherwise the branch remains live.

These tests are free bit/count arithmetic and should run before any projection. They are public and policy-independent, so they commute safely through focal maxima.

More refined bounds may be computed per action and per posterior component, but they must be aggregated **by action across the complete information set before taking a maximum**. Section 10.4 states the safe interval algebra.

---

# 3. Exact arithmetic without rationals on the GPU

## 3.1 The scale constant

Under the current fixed uniform-random-legal field, every field action chooses uniformly from a legal set of size `k`, where

\[
1 \le k \le 7.
\]

Let

\[
L = \operatorname{lcm}(1,2,3,4,5,6,7)=420.
\]

At a field node with `k` legal actions, replace the probability factor `1/k` by the exact integer multiplier

\[
\frac{420}{k} \in \{420,210,140,105,84,70,60\}.
\]

The denominator gains one factor of `420` per field action. No division is required in a shader.

## 3.2 Scaled Bellman recurrence

Let `F(s)` be the number of nonfocal field actions still scheduled in the full hand from state `s`. If a branch is certified early, the unused field actions are treated as virtual probability-one continuations solely to maintain the common root scale. Let `V(s)` be a contract-success probability. Define

\[
\widehat V(s)=420^{F(s)}V(s).
\]

Then:

- at a focal decision,

  \[
  \widehat V(s)=\max_a \widehat V(s_a);
  \]

- at a field decision with `k` legal actions,

  \[
  \widehat V(s)=\frac{420}{k}\sum_a \widehat V(s_a);
  \]

- at a successful terminal leaf,

  \[
  \widehat V(s)=420^{F(s)};
  \]

- at a failed terminal leaf,

  \[
  \widehat V(s)=0.
  \]

The same recurrence works for integer-valued utilities by multiplying terminal or trick increments by the appropriate power of `420`.

## 3.3 Full opening-hand width bound

After the bidder's opening lead, the three field seats make exactly 21 plays over the hand. With a fixed focal hand there are

\[
N_0=\frac{21!}{7!^3}=399{,}072{,}960
\]

hidden deals.

For contract success, the exact root denominator is

\[
D=N_0\,420^{21}.
\]

The numerator is less than `2^212`.

For a utility with absolute magnitude at most 42,

\[
42N_0 420^{21}<2^{217}.
\]

Therefore an unsigned or signed 256-bit accumulator is sufficient for the entire current Straight-hand profile, with substantial headroom.

This is a mathematical arithmetic contract, not a benchmark guess.

## 3.4 GPU integer types

Core exact types:

```rust
U64Mass
U128Mass  // four 32-bit limbs in Metal
U192Mass  // six 32-bit limbs
U256Mass  // eight 32-bit limbs
```

Treat these names as arithmetic-width contracts, not assumptions that the shader language provides equally fast native scalar types at every width.

Shader variants are specialized by the required width. The correctness-first implementation may use `U256Mass` everywhere; the optimized implementation widens by elapsed field depth and whether world multiplicity is included.

Recommended aggregate width by full-prior scaled mass:

| Field actions already weighted | Required bits, rough ceiling | Suggested type |
|---:|---:|---|
| 0–3 | <= 56 | `u64` |
| 4–9 | <= 108 | `u128` |
| 10–15 | <= 160 | `U192Mass` |
| 16–21 | <= 212 | `U256Mass` |

Do not use wide atomics. Perform exact wide reductions by deterministic segmented tree reduction.

Required primitives:

```text
zero
add
sub
compare
multiply_by_small_u32
multiply_by_precomputed_420_power
shift/copy between width tiers
checked top-limb carry
```

A debug/profile build records an overflow flag. A release proof build still checks the final carry; the static bound is a theorem, not permission to ignore implementation errors.

## 3.5 Global-mass representation

A path or symbolic cell after `e` field actions carries an integer coefficient with denominator `N0 * 420^e`.

For an unresolved leaf, its full-horizon upper contribution is

```text
current_scaled_mass * 420^(21-e)
```

A branch certified as success contributes the same amount to both lower and upper bounds. A failure contributes zero.

This permits interval search without evaluating a local rational or multiplying a posterior probability by a separately stored continuation rational.

## 3.6 Exact residue backend for sum-only kernels

Wide limbs are required wherever values are compared, especially at focal maxima. They are not the only exact representation available for chance-only work.

For a projector or chance reduction that performs only additions and multiplication by known small integers, the GPU may compute the result modulo several pairwise-coprime machine-word moduli. If the product of the moduli exceeds the proved magnitude bound, Chinese-remainder reconstruction yields the unique exact integer.

Use this backend only when all of the following hold:

```text
no ordering comparison occurs before reconstruction
all moduli and reconstruction code are frozen and receipted
the modulus product exceeds the static value bound
every channel has its own mass-conservation check
```

Reconstruct on the CPU or in a dedicated exact stage before any `max`, interval comparison, or final proof inequality. This is an exact performance backend, not an approximation. Benchmark it against limb arithmetic; do not assume it wins.

---

# 4. Static tables generated from the semantic authority

The GPU receives compact tables generated by `walt-core` and hash-checked at startup.

Minimum tables:

```text
context_mask[declaration][context]       -> u32 tile mask
lead_contexts[declaration][tile]         -> u8 context-bitset
rank[declaration][tile]                  -> u8
trick_key[declaration][led_context][tile]-> u16
beats_mask[declaration][context][tile]   -> u32
count_value[tile]                        -> u8
small_scale[k] = 420/k                   -> u16
choose[n][k], n<=21                      -> u32/u64
```

Root and mid-trick action records are generated by the CPU authority. Represent a lead action explicitly as `(tile, selected_led_context)` even when the current rules profile makes that context unique. This prevents the GPU architecture from baking in a stronger uniqueness assumption than the semantic kernel.

The present arithmetic contract still assumes the current profile has at most one legal lead action per tile and therefore at most seven field actions from a seven-tile hand. A future variant that makes context choice an additional field action must supply a new legal-set bound and denominator scale.

Legal follower play is a mask operation:

```rust
let followers = hand & context_mask[decl][led];
let legal = if followers != 0 { followers } else { hand };
```

This preserves the load-bearing fact that effective contexts form a covering, not a partition. Do not assign each uncalled mixed tile one permanent suit ID.

Trick resolution is a four-way table maximum over public tile identities. Count is a table sum. The shader should contain no declaration-specific branch tree in its hot path.

---

# 5. The exact opening-trick projector

This is the first production GPU milestone because it attacks trick 1 directly and has a small, closed output bound.

## 5.1 Reuse by led context

For a fixed focal hand, the hidden pool is independent of the selected root lead action.

Under the uniform-random-legal field, the opponents' response law depends on the **selected led effective context**, not on the lead tile's rank.

Therefore:

> Ask the CPU semantic authority for the complete legal `LeadAction` set, group those actions by selected led context, build one opening response kernel per represented context, and reuse it for every action in that group.

Action-specific work after projection is only:

- resolve the winner using the actual lead tile;
- add the lead tile's count decoration;
- update the focal hand;
- apply the contract budget;
- attach the action-specific successor public state.

## 5.2 Definitions

Fix a root action `a` and let `q` be its led context.

Let:

- `U` be the 21 hidden tiles;
- `M = U & context_mask[q]` be the hidden tiles that can follow;
- `x = (x1,x2,x3)` be an ordered distinct response triple for seats 1, 2, and 3;
- `F` be the response seats whose `xs` belongs to `M`;
- `Z` be the seats whose response is outside `M`.

A seat in `Z` must hold no tile in `M`. A seat in `F` holds its played matching tile plus `es` additional matching tiles.

The remaining matching-tile counts satisfy

\[
\sum_{s\in F}e_s=|M|-|F|.
\]

Because `|M| <= 6`, there are at most ten such vectors.

## 5.3 Compatible-world count

After removing the responses, each hidden seat has six tiles.

For a fixed `e` vector, the number of compatible hidden deals is

\[
A(e,x)=
\frac{|M'|!}{\prod_{s\in F}e_s!}
\cdot
\frac{|N'|!}
{\left(\prod_{s\in F}(6-e_s)!\right)(6!)^{|Z|}},
\]

where:

- `M'` is `M` with matching response tiles removed;
- `N'` is `U \ M` with void-seat response tiles removed.

Implement this as products of binomial coefficients, not raw factorial division. Intermediate values then remain comfortably inside `u64`.

## 5.4 Exact scaled likelihood

For a follower seat, the chosen response had probability `1/(es+1)`. For a void seat, the chosen response had probability `1/7`.

At the `420^3` opening-response scale, define

\[
C(e,x)=
\left(\prod_{s\in F}\frac{420}{e_s+1}\right)
60^{|Z|}.
\]

The cell's scaled mass is

\[
W(e,x)=A(e,x)C(e,x).
\]

The total response-triple mass is

\[
W(x)=\sum_e W(e,x).
\]

All `W(e,x)` values fit in `u64`.

The required mass-conservation receipt is

\[
\sum_x W(x)=N_0 420^3.
\]

## 5.5 Output cell

Do not emit only the response mass. Emit the exact posterior components.

Each valid `(x,e)` produces a uniformly weighted allocation cell:

```rust
struct OpeningCell {
    public_response: [u8; 3],
    remaining_pool: u32,
    capacities: [u8; 3],          // [6,6,6]
    matching_mask: u32,            // M after played responses removed
    matching_counts: [u8; 3],     // post-play e-vector; zero for void seats
    per_world_coeff: ExactMass,    // C(e,x)
    support_count: u32,            // A(e,x)
}
```

Every hidden allocation inside this cell has the same scaled likelihood. This is what permits exact continuation without retaining the original 399 million worlds.

## 5.6 Output bound

There are at most

\[
21\cdot20\cdot19=7{,}980
\]

ordered distinct response triples and at most ten cells per triple.

Therefore one root context produces at most 79,800 raw cells before:

- impossible triples are removed;
- contract-failing responses are discarded;
- equivalent cells are merged;
- Scheme/Fix leaves close;
- repeated root contexts are shared.

This is the first exact compression boundary.

## 5.7 Opening GPU kernels

Recommended v0 pipeline:

```text
K-OPEN-1 enumerate candidate response triples
K-OPEN-2 enumerate feasible count vectors and emit raw cells
K-OPEN-3 reduce cell masses by response triple
K-OPEN-4 resolve trick for every lead action sharing the context
K-OPEN-5 apply budget pruning and emit successor nodes
K-OPEN-6 canonicalize and merge identical cell shapes
```

Correctness mode keeps the kernels separate. Performance mode may fuse `K-OPEN-2` through `K-OPEN-4` after differential tests are permanent.

## 5.8 Do not offload a tiny opening batch by ideology

Fewer than 79,800 raw cells for one context may be faster on one CPU core than through a standalone Metal dispatch. The exact data model remains GPU-native, but the runtime must benchmark and retain both routes.

The GPU opening kernel should normally batch:

- every led context in the coordinate;
- several coordinates during surveys;
- or the opening projection together with immediate action resolution and pruning.

Record the chosen route in the performance receipt. A GPU-native solver is allowed to execute a small exact stage on the CPU when that lowers end-to-end proof time.

---

# 6. Exact symbolic posterior representations

The opening cells generalize into one early-game symbolic representation, but they must not become the only lens. The engine supports both explicit disjoint cell mixtures and compact weighted posterior circuits.

## 6.1 Shape and coefficient

A factor-cell term is:

```rust
struct CellTerm {
    shape_id: ShapeId,
    coeff: ExactMass,
}
```

A shape denotes a set of hidden allocations. Every allocation in that set has the same coefficient.

The shape contains:

```rust
struct CellShape {
    ambiguous_pool: u32,
    forced_holder: [u32; 3],
    allowed_tiles: [u32; 3],
    residual_capacity: [u8; 3],
    basis_rows: BasisId,
}
```

A basis row records an exact count of one tile mask in each hidden seat:

```rust
struct BasisRow {
    tile_mask: u32,
    count_s1: u8,
    count_s2: u8,
    count_s3: u8,
}
```

The sum of the three counts equals the number of basis-mask tiles not already forced.

A lawful information state owns a **mixture** of cell terms. The policy sees its own hand and public record, never the term index.

Every canonical shape must satisfy these invariants:

```text
forced-holder masks are pairwise disjoint
forced tiles are absent from the ambiguous pool
every ambiguous tile has at least one allowed seat
the three residual capacities sum to popcount(ambiguous_pool)
every basis-row count is within the corresponding seat capacity
row counts sum to the number of row-mask tiles still ambiguous
all masks are restricted to the current remaining tile universe
```

The CPU reference checker validates these invariants before a shape is admitted to a proof run.

## 6.2 Why this representation is exact

Whenever a field action is observed, its likelihood depends on the acting seat's legal-set size. Split the parent support by the exact legal-set count. Inside each resulting stratum:

- the observed action is legal for every allocation;
- the action probability is identical for every allocation;
- multiplying the parent coefficient by `420/k` preserves a constant coefficient.

Thus conditioning a uniform weighted cell produces a finite mixture of new uniform weighted cells.

Repeated application is exact. The cost is the number and projection width of the cells, which is a measured quantity rather than an assumed small one.

Only one persistent row is needed for each effective context. When an observed action first exposes a legal-set count, the cell is split and the corresponding `420/k` factor is absorbed into the coefficient. The row then carries the **current** per-seat count for that context and is decremented as matching tiles are publicly played. Repeating the context reuses the row. Therefore ordinary play contributes at most eight persistent context rows; exploratory Scheme queries remain temporary unless they become part of public conditioning.

## 6.3 Basis canonicalization

Canonicalize every new shape before hashing:

1. remove forced tiles from the ambiguous pool;
2. decrement capacities and basis counts accordingly;
3. intersect allowed masks with the current pool;
4. drop empty-mask rows after checking zero counts;
5. drop full-pool rows implied by capacities;
6. merge duplicate masks after checking equal count vectors;
7. remove a row only when accompanied by an exact implication certificate;
8. sort rows by mask and then counts;
9. reject negative counts, over-capacity counts, or unsupported tiles;
10. compute a full canonical key and a hash.

For v0.2, the safe implication rules are deliberately conservative:

- duplicate rows with identical counts;
- complements implied directly by residual capacities;
- exact disjoint-union identities whose mask equality and count sum are checked bitwise;
- or a general integer implication result produced by an independent checker.

Do **not** drop rows merely because floating-point or rational linear algebra calls them dependent. Overlapping incidence rows are integer constraints on labeled tiles; naive row reduction can change the support.

Hash equality is only a candidate match. Final merging compares full canonical keys.

## 6.4 Shape/mass separation

Projection depends on the shape, not the coefficient.

Before launching an expensive projection:

```text
sort requests by (shape, query)
deduplicate identical requests
run one projector per unique request
scatter the histogram to every dependent coefficient
```

This is expected to be one of the largest practical savings if the game is deep but narrow.

## 6.5 Seat-potential posterior circuits

Repeatedly disjointizing every legal-set count can widen the mixture even when the posterior itself has a short formula. Preserve a second exact representation.

For a fixed public history `h`, let `H_s` be hidden seat `s`'s compatible hand and let `L_{s,h}(H_s)` be the exact scaled likelihood that this seat, under the frozen field profile, would have produced its observed actions. Then the posterior weight has the form

\[
w_h(H_1,H_2,H_3)
=
\mathbf 1[H_1,H_2,H_3\text{ form the required disjoint allocation}]
\prod_{s=1}^{3}L_{s,h}(H_s).
\]

This factorization is exact. It can be stored as:

```text
three seat-local likelihood evaluators
a disjoint-allocation constraint
publicly forced ownership / void constraints
an exact normalization scale
```

A CPU-built algebraic decision diagram or deterministic arithmetic DAG may compile the same object more compactly. The CPU performs irregular DAG construction and reduction; the GPU evaluates large homogeneous DAG layers and coefficient queries.

A `PosteriorCircuit` backend is preferable when the cell mixture is wide but the seat-local likelihood programs remain small. A short source formula does not guarantee a cheap projection, so the cost model compares compiled projector work, not source length.

## 6.6 Representation boundary

`FactorCellMixture` and `PosteriorCircuit` are interchangeable exact descriptions of the same weighted posterior only after an independent normalization and query cross-check. The runtime may switch between them, but it must record the conversion receipt and must not infer strategy from the representation choice.

---

# 7. The exact factor-cell counting primitive

All symbolic continuation depends on one reusable operation.

## 7.1 Primitive signature

```rust
fn count_histogram(
    shape: CellShape,
    query_mask: u32,
    fixed_owners: FixedOwners,
) -> ExactHistogram;
```

The result counts hidden allocations satisfying the cell constraints, grouped by the query-mask count in each seat.

Because the three seat counts sum to the number of query tiles, only two counts need to be stored explicitly.

This primitive supports:

- field-response conditioning;
- legal-set denominators;
- void tests;
- ownership events;
- one-trick count predicates;
- Scheme/Fix Boolean atoms;
- universal-plan checks;
- Tax-Scheme event masses;
- exact materialization counts.

Conceptually it extracts coefficients from

\[
\prod_{d\in P}
\left(
\sum_{s\in A(d)}
 x_s
 \prod_j y_{j,s}^{\mathbf 1[d\in R_j]}
 z_s^{\mathbf 1[d\in Q]}
\right),
\]

where `P` is the ambiguous pool, `A(d)` is the allowed-seat set, `R_j` are persistent basis masks, and `Q` is the temporary query mask. The implementations below are exact coefficient extractors for this polynomial.

## 7.2 Tile-class factorization

For one request, classify each ambiguous tile by:

```text
allowed-seat bits
membership bits across active basis masks
membership in the temporary query mask
optional temporary decoration required by the query
```

Tiles with the same signature form one labeled class of size `n`.

Do **not** permanently refine a posterior cell for every exploratory Scheme query. Query masks are temporary. Only public conditioning and likelihood-required counts enter the persistent basis.

## 7.3 Sparse dynamic program

Process one tile class at a time.

For a class of size `n`, enumerate allowed distributions

\[
(x_1,x_2,x_3),\qquad x_1+x_2+x_3=n.
\]

Multiply by the labeled-tile multinomial coefficient

\[
\frac{n!}{x_1!x_2!x_3!}.
\]

The DP state tracks:

- residual capacities for seats 1 and 2;
- residual basis counts for seats 1 and 2;
- query counts for seats 1 and 2.

Seat 3 is implied by totals.

With at most eight persistent context rows, the packed key fits in 64 bits:

```text
2 capacities × 3 bits
2 counts per row × 3 bits × at most 8 rows
2 query counts × 3 bits
```

The GPU expansion is:

```text
current sparse states
    × valid class distributions
        ↓ emit (task_id, packed_state, count)
radix sort
segmented exact sum
        ↓
next sparse states
```

At completion, retain only states satisfying the target capacities and basis counts, and reduce by query-count bin.

## 7.4 GPU-native meet-in-the-middle projector

For high basis rank, the sparse DP may spend more time sorting intermediate states than the 21-tile universe warrants. A bounded meet-in-the-middle projector is the second exact workhorse.

Split the ambiguous tiles into two halves, balancing the product of their allowed-seat counts rather than merely the tile count. With at most 21 tiles, the unconstrained half enumerations are bounded by

\[
3^{11}=177{,}147,
\qquad
3^{10}=59{,}049.
\]

During half enumeration, prune an assignment as soon as any seat capacity or target basis-row count is exceeded. Consequently every packed count remains in `0..7` even though a half may contain eleven tiles.

For every surviving allowed owner assignment in one half, emit:

```text
seat-1 and seat-2 capacities used
seat-1 and seat-2 counts for every basis row
seat-1 and seat-2 query counts
assignment multiplicity, if tile classes were grouped
```

Group half assignments by the constraint signature excluding query counts, retaining a tiny histogram over query counts. For each left signature, compute the exact complementary right signature required by the target capacities and basis counts, then convolve the two query histograms.

This algorithm has several advantages on the target GPU:

- a fixed finite enumeration bound independent of DP state width;
- flat branch-light assignment generation;
- 64-bit packed signatures at basis rank at most eight;
- deterministic sort/reduce and exact complement joins;
- natural batching across shapes and queries.

Cache reusable half-assignment tables by `(shape, half partition)` when memory permits. Query-specific counts may be projected from stored owner masks without rebuilding the constraint signature.

## 7.5 Projector portfolio

### Tiny sparse DP

Use one threadgroup when the estimated state count fits threadgroup memory.

Good cases:

- one or two active context masks;
- small classes;
- low remaining grade;
- opening-derived cells.

### Meet-in-the-middle

Use the bounded half enumeration when basis rank or overlap makes sparse intermediate state growth uncertain.

Good cases:

- medium or high basis rank;
- many overlapping rows;
- one or several query histograms on the same shape;
- enough homogeneous tasks to amortize sorting.

### Global sparse DP

Flatten many tasks into one class-by-class global wavefront. Include `task_id` in the sort key.

Good cases:

- many cells with similar class sequences;
- sparse reachable signatures far below the half-enumeration bound;
- workloads large enough to occupy the GPU.

The scheduler benchmarks all three and maintains an online calibrated crossover model. A wrong prediction changes performance only, never the value.

## 7.6 Field-action conditioning

For an acting hidden seat `s` and candidate observed tile `x`:

1. condition `x` to be held by `s`;
2. remove `x` from the ambiguous pool or forced set;
3. update all persistent basis rows containing `x`;
4. query the remaining led-context mask;
5. enumerate count bins;
6. if `x` follows, require pre-play legal count `k = post_count_s + 1`;
7. if `x` does not follow, require post-count `0` and use `k = hand_size_before_play`;
8. multiply the coefficient by `420/k`;
9. add the exact post-play context-count row to the basis when it is not implied;
10. canonicalize and merge.

The mass-conservation receipt at one field action is:

\[
\sum_{x\text{ public response}} \operatorname{mass}(x)
=420\operatorname{mass}(\text{parent})
\]

at the next denominator scale.

## 7.7 Repeated contexts are cheap

If the led context is already represented in the persistent basis, the current count in each seat is known inside every cell. No histogram split is required merely to discover legal-set sizes.

Repeated suit pressure therefore deepens the game without necessarily widening the symbolic posterior. This is one concrete mechanism by which the solve may be deep but narrow.

---

# 8. Adaptive representation switching

No one representation is required to win everywhere.

## 8.1 Available backends

### `FactorCell`

Use while the posterior is compact under active-context constraints.

### `PosteriorCircuit`

Use a seat-factorized likelihood program or compiled arithmetic decision DAG when disjoint cell count is large but the weighted posterior remains compact.

### `ExplicitWorld`

Materialize every hidden allocation when the cell fiber is small enough or the factor projector is predicted to cost more than enumeration.

At trick boundaries with equal remaining hidden-hand size `g`, the unconstrained hidden-world counts are:

| `g` per hidden seat | Worlds |
|---:|---:|
| 7 | 399,072,960 |
| 6 | 17,153,136 |
| 5 | 756,756 |
| 4 | 34,650 |
| 3 | 1,680 |
| 2 | 90 |
| 1 | 6 |

Constraints often lower these numbers substantially. Away from trick boundaries, use the general count

\[
\frac{n!}{c_1!c_2!c_3!},
\]

for the current hidden pool size `n` and unequal residual capacities `c1,c2,c3`.

### `ExactEndgame`

Hand a small explicit carrier to the existing exact CPU solver or the new GPU endgame wavefront.

### `SchemeLeaf`

Stop with a proved plan, tax, count destiny, or invariant.

### `IntervalLeaf`

Use a certified affine or other bounded approximation when available. Exactness is purchased only if the interval remains decision-relevant. Leaf endpoints must either be supplied directly in the common full-horizon integer scale or converted outward with exact floor/ceiling arithmetic; a floating conversion may not tighten an interval.

## 8.2 Backend choice

Estimate:

```text
factor-DP emitted states
meet-in-the-middle half assignments and join width
posterior-circuit compiled width
explicit world count
expected field-only epoch expansion
expected net width
available Scheme/Fix coverage
root influence of the node
CPU/GPU transfer and dispatch cost
```

Choose the backend and processor minimizing estimated work **for the root proof**, not the backend with the smallest abstract state description.

Use hysteresis: after switching representation, require a substantial predicted advantage before switching back. This prevents a difficult node from thrashing between symbolic and explicit forms.

## 8.3 Exact materialization without rejection

The counting DP can support rank/unrank over a cell's satisfying assignments. Store suffix counts for DP states, then map an assignment index to seat distributions and tile choices.

This permits GPU materialization in bounded chunks:

```text
cell support count
    ↓ choose index range
exact rank/unrank
    ↓
packed explicit worlds
```

No rejection sampler and no duplicate filtering are needed.

---

# 9. Explicit-world GPU wavefront with the mandatory net

The explicit backend follows the previously selected wavefront-with-net architecture.

## 9.0 Focal epochs are the host-level recursion unit

The CPU proof controller should not regain control after every tile play. Its natural edge is:

```text
one lawful focal information state
        ↓ choose one focal action
field-only evolution under the frozen field
        ↓
next focal decision, terminal contract state, or certified leaf
```

Call this an **action-conditioned focal epoch**.

Within an epoch there is no focal choice, so cells or explicit worlds may evolve independently. The GPU may perform several field expansions, trick resolutions, contract checks, and compactions before emitting the next focal frontier. Only at the epoch boundary is the information-set net mandatory.

This matches the reveal-delay mathematics and keeps the host decision DAG shallow: after an opening lead there are at most five future nonforced focal decisions, because the final one-tile play is forced. It also reduces CPU synchronization, repeated sorting, and descriptor traffic.

Epoch lengths are variable because the focal seat may act at different positions in the next trick. Every branch therefore carries its elapsed field-action exponent and is normalized to the common full-horizon scale before merging.

## 9.1 Do not store the focal hand per particle

All particles in one information segment share the focal hand. Store it once in the segment descriptor.

A particle stores only hidden remainders and its exact weight:

```rust
struct Particle<W> {
    h1: u32,
    h2: u32,
    h3: u32,
    weight: W,
}
```

A segment stores:

```rust
struct SegmentDesc {
    particle_offset: u32,
    particle_count: u32,
    public_state: PackedPublicState,
    focal_hand: u32,
    elapsed_field_actions: u8,
    remaining_loss_budget: i8,
    policy_descriptor: u32,
    parent_edge: u32,
}
```

Proof-only descriptors remain attached to posterior components or certificate edges. They are never stored as one common segment field unless a checker has proved them uniform, and they never enter the lawful key.

## 9.2 Field expansion without a global world sort

The tile universe has only 28 actions. For one parent information segment:

1. each particle computes its legal mask;
2. build a 28-bin child count histogram;
3. prefix-sum the bins;
4. scatter each legal child into its `(parent segment, public tile)` bucket;
5. multiply weight by `420/popcount(legal)`;
6. discard empty buckets.

This preserves child segment contiguity by construction and avoids a full global radix sort at every field action.

Use threadgroup-local histograms when a segment is large enough to own a threadgroup. Use global `u32` count atomics only for reservation/counting, never for exact masses.

## 9.3 Focal action expansion

At a focal segment, the legal mask is common.

Create one action child descriptor per legal action. The hidden-particle slice is immutable and may be referenced by every action child until a later field expansion creates modified hands.

This copy-on-write segment design avoids replicating the whole posterior merely because the focal player is considering alternatives.

## 9.4 The net stage

Field observations may cause branches from different parent cell terms or particle slices to reach the same lawful information state: the same focal hand and the same public record under the same objective state.

At every focal decision:

1. emit the complete lawful information key, including the focal current hand;
2. radix-sort branch descriptors by the full key;
3. build one `InfoNode` per key;
4. concatenate or gather all particle/cell slices into that node;
5. merge identical symbolic shapes;
6. choose one action for the whole node.

This is the mandatory level-synchronous point. It is not an optional optimization.

## 9.5 Trick resolution

A public trick prefix has one winner and one count total. Resolve once per segment, not once per particle.

Then:

- decrement the loss budget if defenders won;
- apply public contract decidedness;
- update leader and trick index;
- clear the prefix;
- update the public `PolicyDescriptor` from the observed trick;
- update proof-only descriptors within their component/checker layer;
- widen the weight type if the next depth tier requires it.

## 9.6 Backward backup

Store parent/action edges. Process completed layers in reverse:

- field/chance node: exact sum of child masses;
- focal information node: exact maximum over action-child masses;
- interval node: propagate lower and upper separately;
- terminal/certificate leaf: use its exact or bounded full-horizon mass.

No wide atomic is needed. Children are sorted or bucketed by parent edge and reduced deterministically.

---

# 10. Count-aware stopped proof search

A full wavefront solve remains too expensive at trick 1. The controller therefore runs an interval proof search.

## 10.1 Root state

For every root action `a`, maintain exact full-denominator bounds:

```rust
RootBounds {
    lower: U256Mass,
    upper: U256Mass,
}
```

The strict verdict condition is:

```text
lower(best candidate) > max upper(all competitors)
```

An exact tie is reported only when the relevant action intervals have collapsed to exact equal values. Merely touching or overlapping bounds are unresolved, never rounded into a tie.

## 10.2 Leaf classes

An unresolved leaf begins with:

```text
lower = 0
upper = its entire remaining full-horizon mass
```

It may become:

- contract-failed: `[0,0]`;
- contract-guaranteed: `[mass,mass]`;
- Plan-Scheme certified: exact or lower-bounded;
- Tax-Scheme bounded;
- affine/certified interval;
- count-aware clairvoyant upper leaf;
- nonanticipativity-glued upper leaf;
- exact endgame leaf.

## 10.3 One-trick survival upper

From the exact current-trick kernel, sum the mass of public responses that do not already defeat the contract. Replace every surviving continuation by success.

This is an exact action-conditioned upper bound requiring no residual solve.

At the opening lead, its full-horizon numerator is:

```text
surviving_scaled_mass_at_420^3 * 420^18
```

The same kernel may contribute a lower bound from branches where the contract is already publicly guaranteed.

## 10.4 Action-indexed interval algebra and safe pruning

Let an information node contain posterior components `i` with exact weights `w_i`, and let `a` be one common legal action.

### Upper bounds decompose by component

If component `i` supplies any valid action-conditioned upper bound `u_i(a)`, then

\[
U(a)=\sum_i w_i u_i(a)
\]

is a valid upper bound for that common action. The node upper is

\[
U_{\mathrm{node}}=\max_a U(a).
\]

The forbidden operation is

\[
\sum_i w_i\max_a u_i(a),
\]

unless that expression is intentionally being used as a clairvoyant upper relaxation.

### Lower bounds require one shared lawful continuation

Lower witnesses do **not** decompose freely by hidden component. Let `pi` be one lawful continuation policy or one Plan Scheme whose future choices depend only on lawful information. If `ell_i(a;pi)` is a valid component contribution under that same `pi`, then

\[
L(a;\pi)=\sum_i w_i\ell_i(a;\pi)
\]

is valid, and

\[
L(a)=\max_{\pi\in\mathcal P_a}L(a;\pi),
\qquad
L_{\mathrm{node}}=\max_a L(a),
\]

where `P_a` is the set of certified common plans evaluated so far.

Never compute

\[
\sum_i w_i\max_\pi \ell_i(a;\pi).
\]

That is strategy fusion on the primal side. Component-local exact values may contribute to a lower bound only when they are terminal/action-independent, or when the proof bundle identifies the one common lawful policy attaining all of them.

This is the safe posterior-aggregated pruning rule:

- upper contributions may be closed component by component and summed by action;
- lower contributions are grouped by a common Plan-Scheme or policy identifier before summation;
- the focal maximum occurs only after those lawful aggregates are formed.

Exact action dominance follows immediately. If

\[
U(a)<\max_b L(b),
\]

then action `a` can never be optimal at this information node and may be removed permanently. Record the separating inequality and the common lower-witness policy in the receipt.

At a chance sum, process successor blocks in an order useful to the current proof. If a partially evaluated action has exact partial upper `P` and unprocessed full mass `M`, then `P+M` is an exact remaining upper for binary contract utility. Stop evaluating that action as soon as it falls below the active competing lower bound. This is the solver's direct escape-time analogue.

## 10.5 Search priority

Let the current root separation deficit be

\[
\Gamma=
\max_{a\ne a^\star}U(a)-L(a^\star),
\]

where `a*` is the current incumbent. For every unresolved leaf estimate its maximum possible reduction of `Gamma` per predicted unit of work:

```text
root influence × locally relevant interval width / predicted expansion cost
```

At chance nodes, root influence is multiplied by exact branch mass. At max nodes, only actions capable of attaining the active upper or improving the active lower are immediately relevant.

However, the second-rung slack–tax result forbids following only one returned relaxed optimizer. Retain every escape action whose slack is small enough that it could evade the proposed downstream tax.

Batch the highest-priority leaves by homogeneous GPU signature:

```text
representation backend
grade
actor position
basis rank
arithmetic width
declaration/led-context kernel
Scheme/Fix program
```

Use proof-aware batch sizing. Large batches maximize throughput while the root gap is wide. As `Gamma` approaches zero, shrink slabs so the GPU does not finish millions of branches after a separating inequality was already available.

Maintain two kinds of work explicitly:

- incumbent work that can raise a lower bound;
- competitor work that can lower an upper bound.

The scheduler should attack whichever side currently controls `Gamma`.

## 10.6 Move ordering

Ordering is heuristic and cannot change a proof value.

Recommended order:

1. strongest existing lawful lower plan;
2. smallest surviving symbolic frontier;
3. highest immediate contract-survival mass;
4. lowest predicted projection cost;
5. exact late-game `Q` when cheaper than the learned oracle;
6. learned omega-to-`Q` score as a final ordering signal.

The previously measured smallest-frontier association belongs here: useful for ordering, never a theorem about trick-1 optimality.

---

# 11. Scheme/Fix in the GPU engine

Scheme/Fix must describe strategy and proof obligations, not only static world partitions.

## 11.1 Three compiled program classes

### Plan Scheme

A lawful partial strategy with:

```text
observable guard
public action program
public descriptor update
hidden invariant checker
rank / termination measure
terminal or residual lower contract
```

Examples include preserving control, respecting partner's lead, manufacturing a void, retaining an entry, promoting a walker, or escorting count.

### Tax Scheme

An action-conditioned upper certificate or regret minorant. It may use hidden facts in the proof, but it may not let the focal policy branch on them.

### Descriptor Fix

A recursive automaton carrying the information needed by a Plan or Tax Scheme. Its state is split into:

- a `PolicyDescriptor`, updated from focal knowledge and public observations and therefore lawful to consult when choosing an action;
- a `ProofDescriptor`, which may summarize hidden relational facts but is used only to verify coverage, invariants, or bounds.

## 11.2 Research interpreter and production compilation

Use a hybrid path:

1. **CPU synthesis and DAG construction** for irregular search, overlap analysis, minimization, and counterexample-guided repair.
2. **CPU bytecode interpretation** for tiny or highly divergent batches.
3. **GPU bytecode interpretation** only for large homogeneous survey batches.
4. **Generated Metal kernels or compiled arithmetic DAG layers** for promoted high-volume programs.

The production compiler should disjointize overlapping Fix branches into a deterministic decision DAG. Exact event mass is then a sum over disjoint leaves rather than an inclusion-exclusion accident.

## 11.3 Cell-level three-valued evaluation

For a factor cell, a predicate may be:

```text
definitely true
definitely false
mixed
```

- definite results require no split;
- mixed results invoke the exact counting primitive;
- a temporary query does not enter the persistent posterior basis unless public conditioning requires it.

A universal Plan invariant is verified by proving zero mass for its negation over the entire information-set posterior.

## 11.4 Optimize saved work, not descriptive beauty

The Scheme synthesizer's objective should include:

```text
root bound improvement
mass covered
projection cost
compiled program cost
verification cost
fallback work avoided
```

A locally specialized program is acceptable when it saves more work than it costs. A beautifully general program is a failure if its exact projector is slower than the branch it replaces.

---

# 12. Nonanticipativity taxes after count pruning

The original gluing machinery remains an upper-bound engine on unresolved count-aware leaves.

## 12.1 First rung on GPU

For each frontier lawful information key and action, compute exact count-aware continuation values under the chosen relaxed treatment.

A learned `Q` estimate cannot enter this reduction. It may identify promising regions, but every tax used in a proof comes from an exact evaluator or a separately certified action interval.

Then reduce:

```text
per world: max action value
per information segment: sum of world maxima
per information segment/action: sum action values
per information segment: max common-action sum
local tax = first sum - second maximum
```

All reductions use common exact scales and segmented wide-integer reduction.

## 12.2 Second rung

The GPU emits per `(I,b,J)` downstream taxes and per-first-action values. The CPU or GPU then computes

```text
min_b(first-action slack + downstream tax)
```

for every first information state.

Every first action must be covered. Taxing only the first-rung optimal face is unsafe.

The focal-epoch interface is the natural implementation boundary: an action child is advanced to the next focal frontier, then downstream taxes are grouped by the resulting lawful information states.

## 12.3 Apply gluing only where it can change the root verdict

Contract-pruned, Plan-certified, and dominated leaves are absent from the gluing workload.

This is the implementation of:

> **No information tax on nodes the count-aware proof never needs to consider.**

## 12.4 Perfect-information `Q` source boundary

The `Q` source is pluggable:

```rust
trait QSource {
    fn evaluate_batch(&self, states: &[PhysicalWorldState]) -> QBatch;
    fn exactness(&self) -> Exactness;
}
```

An exact late-game DP may be cheaper than a neural forward pass and should be selected when predicted so.

The learned oracle may:

- order actions;
- locate difficult cells;
- propose Tax Schemes;
- inspect melted-candlewax geometry;
- generate likely counterexamples.

It does not define lawful strategy, and perfect-information ties do not define strategic equivalence.

---

# 13. GPU execution architecture on Apple Silicon

## 13.0 GPU-native, deliberately heterogeneous

Keep these tasks on the CPU by default:

```text
best-first proof scheduling
irregular Scheme/Fix synthesis
symbolic DAG construction and minimization
small-frontier exact work
final CRT reconstruction and proof-bundle assembly
```

Move these tasks to the GPU when batched:

```text
opening and focal-epoch projection
meet-in-the-middle half enumeration and joins
large sparse coefficient DPs
explicit-world field wavefronts
radix grouping and segmented exact sums
compiled Scheme/Fix DAG evaluation
exact late-game valuation batches
```

The runtime keeps an empirical CPU/GPU crossover table keyed by task signature. Unified memory makes switching cheap relative to a discrete-GPU design, but dispatch and synchronization are still real costs.

## 13.1 Metal first

Use Metal directly for the first target. Keep the backend boundary narrow enough for a later CUDA implementation, but do not weaken the Metal design to satisfy an unbuilt portability layer.

Recommended layers:

```text
Rust proof/controller crates
        ↓
small Metal runtime abstraction
        ↓
precompiled metallib compute kernels
```

The binding crate is an implementation detail behind the runtime abstraction.

## 13.2 Unified memory is a scheduling advantage, not a free bandwidth theorem

Use CPU/GPU shared buffers for:

- work descriptors;
- compact summaries;
- overflow flags;
- proof receipts;
- priority-queue handoff.

The CPU and GPU still require explicit synchronization before reading one another's writes.

Benchmark hot frontier buffers in both shared and GPU-private storage. GPU-only ping-pong arenas may benefit from private storage even on unified-memory hardware; compact control buffers should remain shared.

## 13.3 GPU-driven dispatch

After correctness, let compaction kernels write indirect dispatch arguments for subsequent passes. This avoids a CPU round trip merely to learn the next frontier length.

A typical command buffer can encode:

```text
count outputs
prefix scan
scatter/compact
write indirect grid size
indirect next-stage dispatch
```

The CPU should regain control at proof-relevant synchronization points, not after every wavefront pass.

When one proof frontier is too narrow to occupy the device, batch independent root actions or coordinates under separate task IDs. Never merge their semantics or statistics; batching is only a utilization technique.

## 13.4 Arena model

No hot-path allocation.

Preallocate:

```text
frontier A / frontier B
cell-shape arena
cell-term arena
segment descriptors
sort keys and permutation arrays
wide-value arena
histogram scratch
indirect-dispatch arguments
receipt counters
```

Use ping-pong or triple-buffered arenas. A capacity miss sets an overflow flag and records the required count when possible. Rerun a smaller slab; never drop output.

## 13.5 Structure of arrays

Use SoA for fields touched independently in large scans:

```text
public keys
shape ids
coefficients / weights
particle hand masks
parent ids
action ids
flags
```

Use aligned AoS only for small immutable records read together, such as `PackedPublicState` or `U256Mass`.

## 13.6 Kernel specialization

Compile variants using function constants or generated entry points for:

```text
arithmetic width
basis rank band
utility type
field profile
grade band
tiny vs global projector
```

Avoid a single branch-heavy universal shader.

## 13.7 No megakernel

Do not assign one thread an entire game tree or one complete information-set solve.

The hot architecture is bulk synchronous:

```text
expand
compact
net/group
reduce
resolve
repeat
```

Fuse adjacent passes only when profiling shows memory traffic dominates and the fused kernel preserves test visibility.

---

# 14. Recommended crate and directory layout

```text
walt/
  walt-gpu-spec/
    Rust/MSL-compatible packed types
    table generator and semantic digests
    exact arithmetic contracts

  walt-gpu-ref/
    scalar CPU mirror of every GPU kernel
    opening projector reference
    factor-cell histogram reference
    wide integer reference

  walt-metal/
    device, queues, buffers, pipeline cache
    command-buffer and indirect-dispatch orchestration
    profiling hooks

  walt-metal/kernels/
    arithmetic.metal
    scan_compact.metal
    radix_segment.metal
    opening_project.metal
    factor_histogram_tiny.metal
    factor_histogram_global.metal
    factor_mitm.metal
    posterior_dag_eval.metal
    focal_epoch.metal
    residue_sum.metal
    cell_canonicalize.metal
    field_expand.metal
    focal_net.metal
    trick_resolve.metal
    interval_backup.metal
    gluing_reduce.metal
    scheme_vm.metal

  walt-posterior/
    factor-cell and seat-potential representations
    exact conversion and normalization receipts
    CPU DAG construction and canonicalization

  walt-trick1/
    root proof controller
    cost model
    backend switching
    priority scheduler
    proof bundle writer

  walt-gpu-bench/
    microbenchmarks
    grade-4 parity runs
    opening-kernel benchmark
    long-run thermal/profile harness
```

Existing semantic crates remain dependencies. Do not move semantic authority into the GPU crates.

---

# 15. Core host interfaces

## 15.1 Solve request

```rust
pub struct Trick1SolveRequest {
    pub focal_seat: Seat,
    pub public_state: PublicState,
    pub declaration: Decl,
    pub focal_hand: DominoSet,
    pub contract: Contract,
    pub objective: ContractObjective,
    pub field: FieldProfile,
    pub target_team: Team,
    pub exact_work_budget: WorkBudget,
    pub memory_budget: MemoryBudget,
    pub scheme_registry: SchemeRegistryId,
}
```

## 15.2 Result

```rust
pub struct Trick1SolveResult {
    pub verdict: RootVerdict,
    pub action_bounds: Vec<(LeadAction, ExactBound)>,
    pub proof_bundle: ProofBundle,
    pub performance_receipt: PerformanceReceipt,
}
```

## 15.3 Projection backend

```rust
pub trait ProjectionBackend {
    fn expand_field(
        &mut self,
        batch: FieldExpansionBatch,
    ) -> Result<FieldExpansionResult, ExactFailure>;

    fn count_query(
        &mut self,
        batch: CountQueryBatch,
    ) -> Result<CountQueryResult, ExactFailure>;

    fn materialize(
        &mut self,
        batch: MaterializeBatch,
    ) -> Result<ExplicitWorldBatch, ExactFailure>;
}
```

## 15.4 Certificate backend

```rust
pub trait CertificateBackend {
    fn apply_plans(&mut self, batch: NodeBatch) -> CertificateResult;
    fn apply_taxes(&mut self, batch: ActionNodeBatch) -> CertificateResult;
    fn apply_count_deadness(&mut self, batch: NodeBatch) -> ReductionResult;
}
```

## 15.5 Exact evaluator

```rust
pub trait ExactLeafEvaluator {
    fn evaluate(&mut self, batch: ExactLeafBatch)
        -> Result<ExactLeafValues, ExactFailure>;
}
```

The implementation may choose CPU DP, GPU DP, explicit wavefront, or a cached exact result per homogeneous batch.

---

# 16. Performance rip cords

Apply these in approximately this order.

| Optimization | Why it matters | Correctness condition |
|---|---|---|
| Solve one actual loss budget | Avoid 13x value traffic | Contract fixed |
| Group root actions by led context | Share opening posterior projection | Field response law context-only |
| Opening closed-form cells | Remove first 399M-world enumeration | Mass receipt equals `N0*420^3` |
| Integer scale `420` | Remove GPU rational arithmetic | Field legal size divides 420 |
| Shape/mass separation | Reuse projector across weighted terms | Canonical shapes exact |
| Deduplicate projection requests | Compute repeated residual law once | Full-key equality |
| Active-context tile classes | Factor over only relevant masks | Query decoration complete |
| Meet-in-the-middle projector | Bound high-rank coefficient extraction by half assignments | Complement join exact |
| Seat-potential circuit | Avoid needless cell disjointization | Exact normalization/query receipt |
| Repeated-context reuse | Avoid unnecessary cell split | Basis count already exact |
| Basis-rank reduction | Shrink DP key and classes | Pointwise equation implication |
| Temporary Scheme queries | Avoid permanent posterior refinement | Query not needed for likelihood |
| Count-budget pruning | End contract-dead branches early | Information-set-safe certificate |
| Count-label deadness | Recover simpler residual valuation | Threshold-crossing proof |
| Plan/Tax Scheme leaves | Stop recursive regions | Independent checker passes |
| Symbolic/explicit switching | Use cheapest representation locally | Exact rank/unrank or projector |
| Segment histograms for field actions | Avoid global sort every ply | Full 28-action completeness |
| Copy-on-write focal branches | Avoid posterior duplication | Hidden slices immutable |
| Focal-epoch execution | Remove host intervention between focal choices | Net at every epoch boundary |
| GPU net only at focal decisions | Preserve strategy consistency | Lawful key includes focal hand and public record |
| Exact action dominance | Delete actions unable to attain the node maximum | Action-indexed bounds |
| Chunked chance short-circuit | Stop sums once remaining mass cannot change verdict | Exact residual-mass upper |
| CPU/GPU crossover model | Avoid offloading tiny irregular work | Same exact backend contract |
| Indirect dispatch | Remove CPU frontier-length stalls | Overflow/control flags retained |
| Width-specialized exact masses | Reduce bandwidth | Static width bound per variant |
| Residue exact-sum backend | Replace wide limbs in sum-only kernels | CRT bound and reconstruction receipt |
| Q/DP cost-based selection | Avoid needless neural or DP work | Exactness label respected |
| Root-influence scheduling | Refine only decision-relevant leaves | Bounds propagated soundly |
| Gluing after count pruning | Tax only surviving ambiguity | Same count-aware utility |

---

# 17. Correctness and receipt plan

## 17.1 Golden semantic fixtures

Before performance work, freeze fixtures covering:

- all nine declarations;
- every effective-context overlap case;
- called tiles removed from natural contexts;
- every legal-set size `1..7`;
- every count tile;
- partner and opponent trick winners;
- mid-trick focal decisions;
- known voids;
- contract budgets `0,5,10,12`;
- exact ties.

## 17.2 Required receipts

Every proof run records:

```text
input and semantic digests
root hidden-deal count
opening raw/valid cell counts
opening total scaled mass
per-stage parent/child mass conservation
complete legal-action masks
complete public-response masks
lawful information-key version, including focal-hand encoding
focal-epoch input/output mass by elapsed field exponent
factor-DP / meet-in-the-middle state and histogram totals
posterior-circuit normalization and conversion receipts
cell merge before/after counts
basis-row implication certificates
contract-pruned success/failure/unresolved masses
Scheme/Fix program and checker hashes
exact leaf evaluator hashes and values
information-net key version and segment counts
max-after-sum upper aggregation checks
common-policy identifiers for every aggregated lower witness
exact action-dominance inequalities
CPU/GPU backend choice and crossover estimate
wide-integer / residue overflow and reconstruction flags
root bounds and final separating inequality
```

## 17.3 Differential tests

For each kernel:

1. compare GPU output with `walt-gpu-ref` on randomized small inputs;
2. compare both with existing semantic functions;
3. run exhaustive grade-2 and grade-3 carriers;
4. run selected grade-4 full carriers;
5. cross-check the opening projector against direct 399,072,960-world enumeration on existing trick-1 probes where available.

## 17.4 Independent quantities, not self-identities

A receipt must compare quantities produced by genuinely distinct paths or against frozen references. Do not report algebraic identities computed from the same in-memory values as independent verification.

## 17.5 P-A21 discipline

Grade-4 performance and closure rates validate implementation and generate hypotheses. They are not trick-1 claims.

The opening projection theorem and its mass receipt are directly about trick 1. Later-depth compression claims require direct opening-carrier measurements.

---

# 18. Milestone plan

## Milestone 0 — Arithmetic and semantic tables

Deliver:

- generated table blob and hash;
- `U256Mass` CPU and Metal implementations;
- exact small-multiply and segmented reduction;
- proof-bound test `42*N0*420^21 < 2^217`;
- GPU/CPU arithmetic fuzz parity.

Exit gate:

```text
zero mismatches
zero overflows
reproducible buffer hashes
```

## Milestone 1 — Opening-context projector

Deliver:

- an independently written scalar CPU closed-form projector;
- a measured CPU/GPU crossover for one context, one coordinate, and survey batches;
- context-grouped response kernels;
- raw opening cells;
- exact response masses;
- action-specific trick resolution and loss-budget pruning.

Exit gates:

- total mass exactly `N0*420^3`;
- per-response direct-enumeration parity on frozen coordinates;
- no missing or duplicate response triples;
- deterministic output hash;
- measured Metal profile on the M5 Max.

This is the first implementable benchmark and should be built before the full solver controller.

## Milestone 2 — Explicit grade-4 wavefront

Deliver:

- particle segments;
- field histogram/scatter;
- focal net;
- exact contract utility;
- backward reduction.

Exit gate:

One GPU solve matches the CPU reference on:

```text
root exact values
chosen move
complete legal actions
surviving posterior weights
terminal mass
```

The target is semantic parity, not merely throughput.

## Milestone 3 — Exact coefficient projector portfolio

Deliver:

- conservative basis canonicalizer with implication receipts;
- tiny and global sparse DPs;
- meet-in-the-middle half enumerator and complement join;
- seat-potential posterior-circuit prototype;
- count-query API;
- field-action conditioning;
- within-batch projection deduplication;
- exact rank/unrank materialization.

Exit gates:

- opening cells reproduced through the generic engine;
- grade-4 and grade-5 cell counts match explicit enumeration;
- every histogram conserves support count;
- cache/dedup receipts stable.

## Milestone 4 — Next-focal-epoch symbolic propagation

Run directly from opening coordinates. Advance each action through the fixed field until the next focal decision, terminal contract state, or certificate leaf; do not stop merely at the next trick boundary.

Measure:

```text
unresolved posterior mass
number of public nodes
raw and canonical cell terms
unique cell shapes
basis-rank distribution
factor-DP emitted states
projection-cache hit rate
contract-pruned mass
bytes and GPU time per surviving root mass
```

This milestone determines whether the opening game is fast escape, deep but narrow, or deep and wide under each chosen representation.

Decision gate:

```text
If cells are wide but seat-potential/DAG width is narrow, continue with compiled projection.
If both symbolic widths are high but explicit fibers have become small, switch earlier.
If every exact representation is deep and wide, mine a language- or projector-specific lower-bound witness before redesigning.
```

## Milestone 5 — Stopped Scheme/Fix controller

Deliver:

- Plan/Tax/Descriptor program ABI;
- research bytecode VM;
- exact cell predicate counting;
- root interval DAG;
- influence/cost priority scheduler;
- proof bundle.

Exit gate:

A nontrivial count-aware root comparison closes with less exact work than a full carrier solve on a grade where the full answer is independently known.

## Milestone 6 — GPU gluing

Deliver:

- first-rung exact tax reductions;
- second-rung slack–tax reductions;
- action-completeness and escape-action receipts;
- count-aware utility profile.

Exit gate:

Reproduce frozen first- and second-rung exact rationals on the old corpus, then run only on count-aware unresolved leaves in the new solver.

## Milestone 7 — Trick-1 proof attempt

For one chosen opening coordinate:

1. project all root contexts;
2. build cheap count-aware intervals;
3. seed lawful Plan Schemes;
4. expand by root influence per cost;
5. switch representations adaptively;
6. apply gluing only to unresolved upper boundaries;
7. stop at a strict root inequality or an exact tie.

A capped run returns a complete progress receipt, never a partial verdict.

---

# 19. Performance measurement

Record both throughput and compression. A fast kernel attached to an exploding representation is not a successful solver.

## 19.1 Kernel metrics

```text
items/s
bytes read and written per item
GPU time per pass
thread occupancy
bandwidth limiter
sort/scan/reduction throughput
meet-in-the-middle assignments and joins/s
wide-limb versus residue throughput
CPU/GPU crossover by task signature
arena high-water mark
```

Use Metal GPU captures and Apple GPU counter statistics. Profile long runs for thermal behavior rather than trusting a short burst.

## 19.2 Mathematical-compression metrics

```text
opening cells per context
cells per public successor
unique shapes / total terms
basis rank
active-context count
histogram-DP states
meet-in-the-middle half width and join matches
posterior-circuit/DAG width
projection cache hit rate
materialized worlds avoided
contract-pruned mass
Scheme-certified mass
unresolved mass by trick depth
root bound width over time
exact work saved
```

## 19.3 Decision metric

The primary end-to-end metric is:

```text
processor-seconds and peak bytes required to certify the root verdict
```

Secondary metrics are useful only insofar as they explain that number.

---

# 20. Main risks and the tests that distinguish them

## 20.1 Cell explosion

Symptom:

```text
unresolved mass remains high
cell terms and unique shapes both grow rapidly
basis rank approaches the full context set
```

Response:

- materialize selected cells;
- search for Scheme/Fix recurrence;
- improve basis implication/canonicalization;
- prove a language-specific lower bound before replacing the architecture.

## 20.2 Projection dominates

Symptom:

```text
shapes stay few
factor-DP state count per shape is large
```

This is deep but narrow with an expensive projector.

Response:

- cache shape/query results;
- specialize low-rank DPs;
- compile deterministic decision diagrams;
- use meet-in-the-middle or subset-convolution variants;
- compare exact materialization cost;
- consider residue arithmetic only for pure sum queries that require no GPU-side comparison.

## 20.3 Strategy language misses the game

Symptom:

```text
many locally simple lawful policies exist
Scheme/Fix coverage remains poor
perfect-information diagnostics look informative but lawful regret does not improve
```

Response:

- synthesize temporal Plan Schemes against exact lawful leaves;
- add generic combinators such as preserve/force/promote/retain rather than hand-selected static features;
- keep hidden proof facts separate from public policy state;
- judge by saved exact work and root bounds.

## 20.4 GPU irregularity overwhelms arithmetic throughput

Symptom:

```text
low occupancy
many tiny dispatches
large divergence
CPU waits between wavefront passes
```

Response:

- batch by homogeneous signatures;
- indirect-dispatch compacted frontiers;
- tiny/global projector split;
- fuse only high-traffic adjacent passes;
- use CPU scheduling less frequently;
- process several coordinates or root actions concurrently when proof search alone is too narrow to occupy the GPU.

## 20.5 GPU over-offload

Symptom:

```text
small frontiers spend more time in dispatch, synchronization, or sorting than in arithmetic
```

Response:

- execute the exact stage on the CPU;
- batch more independent task IDs;
- move the host boundary to a larger focal epoch;
- compile several tiny queries into one projector pass;
- retain the same proof and receipt interfaces across processors.

## 20.6 Memory becomes the wall

Response order:

1. reduce before materializing;
2. merge shape-identical cell terms;
3. share immutable particle slices across focal actions;
4. stream slabs through fixed arenas;
5. widen arithmetic only when required;
6. spill independent root actions or public-prefix groups to separate runs;
7. preserve exact hashes so streamed reductions remain replayable.

---

# 21. The recommended first build

Do not start with the full trick-1 solver.

Start with this narrow vertical slice:

```text
one fixed trick-1 coordinate
    ↓
CPU-generated semantic tables
    ↓
scalar CPU opening-context projector + batched GPU variant
    ↓
exact response cells and masses
    ↓
action-specific trick resolution
    ↓
loss-budget survival upper for every root action
    ↓
CPU reference comparison and receipt
```

Why this slice:

- it is directly about trick 1;
- the work size is bounded before implementation;
- the exact answer has a closed combinatorial reference;
- it exercises Metal, exact arithmetic, table semantics, compaction, reduction, and proof receipts;
- it produces immediately useful count-aware upper bounds;
- its output is the input representation for the larger solver.

In parallel, build the explicit grade-4 wavefront as the semantic parity harness for the later information-set net.

Only after both pass should the factor-cell engine and stopped controller be joined.

---

# 22. Claim ledger

## Derived exactly in this guide

Under the stated Straight-hand, fixed focal hand, and uniform-random-legal field profile:

1. one contract solve needs a remaining-loss budget of at most 12;
2. `420` clears every field legal-set denominator;
3. a full root aggregate with utility magnitude at most 42 fits in 217 bits, so 256 bits suffice;
4. root-action comparisons can use exact integer numerators under one common denominator;
5. the opening field response law can be shared by legal root `LeadAction`s with the same selected led context;
6. the opening projection has at most 7,980 response triples and at most ten count strata per triple;
7. each opening stratum is a uniformly weighted exact allocation cell;
8. repeated count-stratification yields an exact mixture of uniform weighted cells;
9. under the frozen seat-local field, a fixed public history's posterior weight factorizes into a disjoint-allocation indicator times one likelihood potential per field seat;
10. factor-cell counting is exact coefficient extraction and admits sparse-DP and bounded meet-in-the-middle implementations;
11. one meet-in-the-middle half has at most `3^11 = 177,147` unconstrained assignments for the 21-tile hidden pool;
12. action-conditioned upper bounds may be summed by action across posterior components before the focal maximum;
13. lower contributions may be summed across posterior components only under one shared lawful continuation policy or Plan Scheme;
14. exact action dominance follows from `U(a) < max_b L(b)`;
15. the mandatory focal net preserves one action per lawful information state when its key includes the focal hand and complete public record.

## Engineering proposals

These require measurement:

- factor cells or seat-potential circuits remain narrow enough beyond the opening trick;
- shape/query and circuit-query deduplication have a high hit rate;
- the tiny/global projector split is effective;
- private hot buffers outperform shared hot buffers on the target workload;
- GPU-driven indirect dispatch materially reduces host stalls;
- Scheme/Fix certificates stop enough mass to close a trick-1 root action;
- GPU gluing is cheaper than deeper exact expansion on the surviving boundary.

## Open mathematical/computational questions

- How quickly does persistent basis rank grow on real opening coordinates?
- Does cell width reflect true strategy complexity or only the current posterior lens?
- Which Plan Scheme grammar captures control, escort, void creation, and walker promotion with low exact projection cost?
- Where is the switch point between symbolic projection and explicit-world materialization?
- Does focal-epoch batching reduce synchronization enough to dominate per-ply execution?
- Which cells favor sparse DP, meet-in-the-middle, posterior circuits, explicit worlds, or CPU execution?
- Can exact projector lower bounds identify regions that genuinely must decompress?
- How much count-aware nonanticipativity tax remains after contract pruning?

---

# 23. Revision ledger from v0.1

The second pass made the following substantive changes:

1. Restored the intended hybrid boundary: irregular symbolic/search/proof work is CPU-first; compiled regular valuation and projection work is GPU-first.
2. Replaced per-ply host control with action-conditioned focal epochs.
3. Added a bounded exact meet-in-the-middle projector with at most `3^11` assignments in the larger half.
4. Added seat-potential posterior circuits so disjoint cell mixtures are not the only symbolic lens.
5. Corrected the lawful information key to include the focal hand and separated policy descriptors from proof-only descriptors.
6. Replaced the conservative “uniform whole-node only” pruning language with the stronger safe rule: aggregate action-indexed component bounds before the focal maximum.
7. Added exact action dominance and chance-sum short-circuiting.
8. Removed unsound generic row deletion based on unnamed linear dependence.
9. Added an optional exact residue backend for sum-only GPU kernels.
10. Added explicit CPU/GPU crossover measurement and a risk section for over-offloading small work.

---

# 24. Closing architecture

The new solver should look like this:

```text
                         CPU proof controller
                    priority, bounds, receipts
                                 │
                                 ▼
                  shared compact work descriptors
                                 │
                ┌────────────────┴────────────────┐
                ▼                                 ▼
      CPU irregular exact work             Metal bulk exact work
  scheduling / DAG construction       opening and focal-epoch projectors
  Scheme synthesis / tiny tasks       sparse DP / meet-in-the-middle
  CRT reconstruction / receipts       explicit wavefront / net / reductions
                │                                 │
                └────────────────┬────────────────┘
                                 ▼
                       exact/glued intervals
                                 │
                                 ▼
                       exact root intervals
                                 │
                                 ▼
                 strict separation, exact tie, or cap
```

The M5 Max is not being asked to brute-force the 472-trillion-deal game universe. It is being used for what its GPU is unusually well suited to do:

- evaluate huge flat batches of tiny exact transitions;
- run sparse and meet-in-the-middle coefficient projectors;
- advance action-conditioned focal epochs;
- compact and group lawful information states;
- perform deterministic reductions;
- keep large shared arenas resident;
- and spend arithmetic only on the unresolved proof boundary.

The implementation target is not “GPU 42” in the abstract.

It is:

> **An exact count-aware root-proof machine whose first trick is projected symbolically, whose fixed-field work advances in GPU focal epochs, whose strategy choices are netted by the complete lawful information key, and whose heterogeneous runtime decompresses only what the proof has not already killed.**

That is the build.
