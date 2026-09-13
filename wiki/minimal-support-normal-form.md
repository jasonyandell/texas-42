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

The two halves of CELL-14 sit at different kernel tiers. The **classification half**
(total normal form = the quotient of cell systems by support equality) is
kernel-proved — PA-D05 `fiber_eq_iff_totalNF_eq` (`lean/Texas42/NormalForm.lean:1423`,
2026-07-31). The **global factorization half** (every exact deterministic support
representation factors onto it, with the minimum-state-count consequence) is **not
kernel-proved**: PA-D06 (priority 1) remains open. Both halves are corpus THEOREM —
proved; only the kernel tier differs.

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
  labeled law [CELL-23]. Explicit packed-table byte budgets (86 KB count table — rec
  Math §7.12.1's "tightly bit-packed count table uses 86,428 bytes", beside the larger
  split and orbit tables listed there) are constructive upper bounds, not claimed minima.
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

## Mechanization status (proof-assistant kernel tier)

Rows are **v0.7** `65_MECHANIZATION_LEDGER.md` `PA-` rows (priority in parentheses);
"proved" = a declaration under `lean/Texas42/` checked by the Lean kernel over at most
`propext`/`Classical.choice`/`Quot.sound`, with no `sorry`, `native_decide` or local
axiom (grep re-verified 2026-09-12), as of commit d190b26 (2026-08-02; all 42
priority-0 rows closed). Row-to-declaration map: [lean-row-index](lean-row-index.md).
A kernel theorem never promotes a corpus status; rob receipts named here
(`rob/receipts/verify_normal_form.txt`, `verify_outer.txt`) are conformance evidence,
never a status change. Layer D closed at priority 0 on 2026-07-31 (commit 54fb2acc;
`NormalForm.lean` is the longest module, 1,455 lines, 56 theorems). No solver is
imported: feasibility of well-formed normal forms is proved from the §7.11 strict
singleton Hall inequalities through the generic capacitated Hall lemma.

| Result on this page | Ledger row (priority) | Kernel status (d190b26) | Declaration (`lean/Texas42/`) |
|---|---|---|---|
| CELL-12 active-seat trichotomy `{0,2,3}` | PA-D01 (0) | **proved** | `NormalForm.lean:349` `active_trichotomy` |
| CELL-13 normal form ↔ nonempty exact fibers: well-formed payload, decode feasible and marginal-exact, compile/decode inverse laws | PA-D02, PA-D03, PA-D04 (0) | **proved**: `SupportNF`, `SupportNF.WellFormed` (the TYPE-03 total well-formedness contract); `decode ∘ compile = red`, `compile ∘ decode = id` | `NormalForm.lean:849` `feasible_decode`, `:912` `decode_marginal`, `:1126` `decode_compile`, `:1251` `compile_decode` |
| CELL-14 classification half (total form = quotient by support equality) | PA-D05 (0) | **proved** | `NormalForm.lean:1423` `fiber_eq_iff_totalNF_eq` |
| CELL-14 global factorization half (every exact deterministic representation factors onto it) | PA-D06 (1) | **open — not kernel-proved** | — |
| CELL-15/16 one-assignment SCC compiler; erasable witness | PA-D07 (2) | **open**; rob `r_nf_quotient` "22,620 SCC compilations" conformance | — |
| CELL-17 zero supplemental bits | no row (corollary; the TYPE/derived-view discipline) | not mechanized; rob `r_nf_zero_supplemental` conformance | — |
| CELL-18/19/20 strict Hall, essential exclusions, linear validator | PA-D08 (2) | **open** as a row; the §7.11 strict singleton inequality (an active seat's ambiguous marginal neighbourhood has at least `r_s + 1` tiles) *is* proved and is the well-formedness condition the decoder's feasibility rests on; rob `r_nf_quotient` "2,151 essential exclusions" conformance | `NormalForm.lean:241` `strict_singleton_hall` |
| CELL-21/22/23 ternary signature census, `S₃` orbits, stabilizer sampling; Burnside profile (x:005) | no row; x:005 is exchange-adjudicated tier | not mechanized; rob `r_nf_ternary_census`, `x-r_out_burnside` conformance | — |
| CELL-24/25/25A/26 complement-elided codes, completion automaton, world rank | no row | not mechanized; rob `r_nf_quotient` "22,620 rank/unrank" conformance | — |
| CELL-27 81-bit standalone census | PA-D17 (4, "REFLECT or keep external") | **deliberately external** — ingest verifier (re-run 2026-09-12, identical), x:005 independent reproduction, rob `r_nf_census_81` in exact `BigUint` | — |
| CELL-29 minimality boundary | boundary | n/a | — |
