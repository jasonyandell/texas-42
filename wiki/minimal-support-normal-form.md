# The Globally Minimal Exact Support Normal Form

[Home](Home.md) · owns: the normal form, its minimality, the 81-bit census ·
Sources: both packages Math §7.10–7.12 (shared). Related:
[capacity-dp](capacity-dp.md), [reachability](reachability.md),
[support-dynamics](support-dynamics.md).

## Two minimality notions (never conflate)

**Semantic/state-count minimality** (can two representation states be merged while
decoding the same support set?) vs **encoding/execution minimality** (fewest
bits/ops — meaningless without a cost model) [Math §7.10 preamble; CELL-29].
This section proves the strongest *cost-model-independent* result.

## The normal form

For a nonempty reduced system, split the pool by exact holder sets `A(d)`:

- **certain marks** `K_s = {d : A(d) = {s}}` — exact hidden-location knowledge;
- **ambiguous pool** `W` with residual capacities `r_s = k_s − |K_s|`.

**Native active-seat trichotomy** [Lemma — proved, CELL-12]: the number of
positive-residual seats is 0, 2, or 3 — never 1. Hence the tagged sum:

```
Determinate                       (W = ∅, all residuals 0)
Binary(inactive seat ι, W, q)     (every d∈W possible at both active seats; r_a=q)
Ternary(W, r0, r1, ε)             (ε: partial map d ↦ the one excluded seat)
```

**[Theorem — proved, CELL-13]**: `N(C) = ((K_s), ambiguity)` is in bijection with
nonempty exact support fibers, eliminating the stored pool, all three possible-sets,
certain-holder edges, zero-residual seats, the explicit binary pair, one capacity (by
conservation), per-tile binary holder fields, and all positive ternary edges.

**Global quotient [Theorem — proved, CELL-14]**: with a single `Empty` state for all
infeasible systems, the total normal form is *exactly* the quotient of cell systems by
support equality; **every exact deterministic support representation factors onto it**.
No further semantic merge is possible; on any finite subdomain it attains the minimum
representation-state count.

## Sharpness of the ambiguity core

- **Strict Hall** [Theorem — proved, CELL-18]: every proper active-seat subset has ≥1
  unit of slack — no Hall-tight subcomponent survives reduction.
- **Every stored ternary exclusion is essential** [Theorem — proved, CELL-19]:
  removing one strictly enlarges the decoded fiber.
- **Linear ternary validator** [Corollary — proved, CELL-20]: a ternary payload is
  valid iff residuals are positive, conserved, and `n − n_s ≥ r_s + 1` per seat —
  three comparisons, no matching search.

## Compilation: one assignment + one SCC pass

**[Theorem — proved, CELL-15]**: orient the used holder edges `s→d` and unused allowed
edges `d→s`; an unused edge is marginally supported iff its endpoints share a strongly
connected component (alternating-cycle argument). So exact normal-form compilation =
one feasibility solve + one linear graph pass over ≤21+3 vertices / ≤63 edges. Inside a
certified simulation the actual hidden world is a free witness (kept compiler-private —
exposing it would leak hidden information) [Corollary — proved, CELL-16].

**Zero supplemental bits** [Corollary — proved, CELL-17]: relative to a mechanical
coordinate that retains the deriving fields, cells/fiber/normal form add *no* semantic
information — they are views. (This is the theorem behind v0.7's "derived views"
executable repair; see [discrepancies D2](discrepancies.md).)

## Exact compiled forms (Math §7.12)

- **Binary counts**: `|Φ| = C(|W|, q)`, `≤ C(14,7) = 3432`; sampling = uniform subset.
- **Ternary counts**: depend only on the six integers `(r_0,n_0,r_1,n_1,r_2,n_2)`
  [Theorem — proved, CELL-21]; 136,514 valid seat-labeled signatures, 1,667,666
  feasible allocation matrices (≤114 per signature); `S₃` relabeling gauge collapses
  to 23,842 orbits [CELL-22], with stabilizer-orbit sampling exactly preserving the
  labeled law [CELL-23]. Explicit packed-table byte budgets (86 KB count table…)
  are constructive upper bounds, not claimed minima.
  - **Burnside decomposition of the 23,842 orbit count** [exchange-adjudicated
    CONFIRMED (x:005) — external tier ([claim-ledger](claim-ledger.md))]: the `S₃`
    fixed-point profile is **136,514** signatures fixed by the identity, **2,156** by
    each transposition, **35** by each three-cycle:
    `(136,514 + 3·2,156 + 2·35)/6 = 23,842`. These per-element fixed counts are new
    to the corpus — established by `exchange/adjudication/programs/005.py` plus two
    independent referee derivations.
- **Ordered completion automaton** [Theorem — proved, CELL-25]: residual-capacity
  vectors form the unique minimal partial DFA for a fixed tile order; native ternary
  state fits 9 bits universally [CELL-25A].
- **Fiber-local world rank**: `⌈log₂|Φ|⌉ ≤ 29` bits per world *given* its normal form
  [CELL-26]; the 42-bit two-mask form suits transition-heavy code [CELL-24].

## The 81-bit standalone census

Over the full native cell-schema domain (28 labeled tiles, 3 hidden seats,
capacities ≤7, one extensional `Empty`):

```
N_empty = 1
N_det   = 8,102,258,940,222,814
N_bin   = 11,495,078,055,913,018,482
N_ter   = 1,830,955,704,129,296,418,354,864
total   = 1,830,967,207,309,611,271,596,161   (2^80 < total < 2^81)
```

⇒ **81 bits necessary and sufficient** for a standalone fixed-width support code
[Theorem — proved + exhaustive finite verification, Math §7.12.5, CELL-27].
This counts *feasible-schema* states; legal play reaches far fewer — the reachable
restriction is where the open problem lives ([reachability](reachability.md)).
