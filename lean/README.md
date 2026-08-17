# Texas42 — Lean 4 formalization

Kernel-checked formalization of the Texas 42 foundations, as a
[Lake](https://github.com/leanprover/lean4/tree/master/src/lake) project
depending on [mathlib4](https://github.com/leanprover-community/mathlib4).

This file is the **layout index**: what each module contains, declaration by
declaration. Three companions carry the rest, and this file does not duplicate
them:

- [`PROOFS.md`](PROOFS.md) — the working discipline: the hard rules, what
  `decide` may do, and the idioms that work here. Read it before a new slice.
- [`../wiki/lean.md`](../wiki/lean.md) — the artifact in context: why the
  `decide`/`native_decide` line falls where it does, the build and extension
  workflow, and where the kernel sits in the evidence hierarchy.
- [`../wiki/lean-row-index.md`](../wiki/lean-row-index.md) — the mechanization
  ledger's rows mapped to the declarations that discharge them.

The plan above all of it — the trust boundary, the K0–K15 spine, mechanization
priorities and the scoreboard — is
[`../wiki/proof-assistant-plan.md`](../wiki/proof-assistant-plan.md).

## Authority order

Lean formalizes **ingest as reconciled by the wiki**: the source of truth is
`ingest/texas-42-foundations-source-of-truth-v0.7/` (plus the reconstructed
package), reconciled through `wiki/` (see `wiki/proof-assistant-plan.md`).
`rob/` is outside the kernel trust boundary and cross-validates via receipts.
Per the trust boundary (TRUST-01): external `PASS` receipts are never imported
as axioms — finite claims enter the kernel only via direct proof, a
proved-sound decision procedure, or proved reflection.

## Layout

- `Texas42/Basic.lean` — Layer A finite algebra, first slice:
  - `Pip` (`Fin 7`), `Seat`/`Team`, `Domino` (canonical `(high, low)` pair
    with `low ≤ high`), with `DecidableEq` and `Fintype` instances (PA-A01);
  - `Domino.card_domino : Fintype.card Domino = 28` (PA-A02);
  - the natural incidence covering `σ_p`: `card_incidence` (`|σ_p| = 7`),
    `card_pip_memberships` (doubles in one incidence, mixed in two), and
    `incidence_inter` (`σ_p ∩ σ_q = {p:q}`) (PA-A03);
  - `Domino.countPoints` and
    `Domino.total_countPoints : ∑ d : Domino, countPoints d = 35` (PA-A04);
  - the shared-pip injectivity helpers behind rank injectivity
    (`eq_of_hasPip_of_pipSum_eq`, `eq_double_of_hasPip`).
- `Texas42/Trick.lean` — Layer A declaration algebra, second slice:
  - `Declaration` (nine, `card_declaration`) and `Suit` (eight led contexts,
    `card_suit`) (PA-A01/PA-A05);
  - `called`/`powered`, `effMem` (the follow relation), `ledSuit` (PA-A05);
  - effective-membership bounds and called absorption (`card_effMem`,
    `not_effMem_natural_of_called`, `effMem_natural_iff`) and follow
    exactness (`effMem_ledSuit`) (PA-A06/PA-A07);
  - `rank`, `tier`, and the lexicographic trick `key` (PA-A08);
  - `tier_ledSuit_pos` — a lead always occupies a nonzero tier (PA-A09);
  - `eq_of_key_eq` — key injectivity in nonzero tiers, via the structural
    shared-pip argument, not enumeration (PA-A10);
  - `existsUnique_winner` — four distinct dominoes with a designated lead
    have a unique key-maximal winner (PA-A11);
  - `Beats`/`beatsSet`/`threat`, `beats_exact` (PA-A13),
    `threat_removal_mono` (PA-A14), `lead_threat_incomplete` (PA-A15).
- `Texas42/Transport.lean` — Layer A close-out, pip transports:
  - `Domino.mapPips`/`Suit.mapPips`/`Declaration.mapPips` (Math §3.9);
  - `countPreserving_iff` — a pip permutation preserves every count label
    iff it is the identity or the swap `2 ↔ 3`, by the spec's analytic
    forcing argument (PA-A16);
  - `swap23_bijective`, `swap23_called`/`swap23_effMem`/`swap23_ledSuit`,
    and `swap23_transport_iff` — the swap transports contextual game order
    exactly between the layers of declarations 2 and 3 (PA-A17).

Layer A is complete except PA-A12 (the 737,100-case exhaustive
prose-resolver agreement), which is a deliberate later reflection target.

- `Texas42/Auction.lean` — Layer B, the Straight auction (Math §4.3):
  - `Bid` (with the `P(30) < ⋯ < P(41) < M(1) < ⋯` value embedding),
    `AuctionConfig`, `AuctionState` (PA-B01);
  - decidable `legalBid` (range, exceedance, mark entry/overcall) and the
    deterministic `step`; `LegalAuction` derivations (PA-B02);
  - `mark_le_ceiling` and `mark_five_reachable` — the structural
    reachable mark ceiling `min(maxMarkBid, 5)` (PA-B03).
- `Texas42/Deal.lean` — Layer B, ordered deal worlds (Math §4.1, PA-B05
  define): `Deal` (four labeled seven-tile hands, pairwise disjoint),
  `biUnion_eq_univ`, `existsUnique_mem`, computable `owner`. The deal
  cardinalities (`28!/(7!)⁴`; `21!/(7!)³`) remain open targets.
- `Texas42/Play.lean` — Layer B, contract and contracted play
  (Math §§4.5, 5.1–5.6):
  - `Contract` with derived `threshold`/`stake`/`Makes`/`award` (PA-B06);
  - `PlayState` — the reduced `X_t`: hands, leader, trick prefix, banked
    scores, contract; `actor`, `gamma`, derived `tricksDone` and
    `scoredTiles` (PA-B07);
  - `legalSet` with the lead/follow/slough characterization and
    `legalSet_nonempty` (PA-B08);
  - `step` (trick resolution via the key-maximal winner), the `Inv`
    partition/location invariant, `inv_step`, `inv_init` (PA-B09);
  - conservation: `gamma_init`/`gamma_step` (28 plays), and
    `terminal_scores` — seven tricks and 42 total points at any terminal
    invariant state (PA-B10).
- `Texas42/Trick1Foundation.lean` — the bounded foundation for the adjudicated
  GPU-native opening design (GT1-A):
  - the derived loss allowance is at most 12 for every legal nonpass bid,
    including the zero allowance for marks;
  - the seven-tile hand cap holds initially and is transition-preserved, so an
    actual live `PlayState.legalSet` has cardinality `1..7` and divides 420;
  - the concrete opening-deal count `399072960`, the 212-bit root denominator,
    and the 217-magnitude-bit utility bound;
  - a state-tied unbanked-point invariant derived from `PlayState.Inv`, with the
    unresolved current-trick tiles proved to remain in the unbanked set and the
    invariant preserved across every legal step;
  - the seven exact nonempty opening-cell counts for `m = 0..6`, with sharp
    maximum 11,730;
  - positive componentwise upper summation, one-shared-policy lower summation,
    action dominance, and the distinction between non-strict optimal membership
    and strict uniqueness.

  This is deliberately a foundation, not an implementation-refinement theorem:
  the `(response,e)` partition, projector code, and Rust/Metal correspondence
  remain named proof debt.
- `Texas42/Cells.lean` — Layer C, the public record, capacity cells, and
  **the losslessness theorem** (Math §§6.3–6.4, 7.1–7.5):
  - replay machinery (`replayFrom`/`LegalFrom`) and `legalSet_congr` —
    legality is public given the actor's hand;
  - `PubState` — the public record machine (leader, trick, played-by-seat
    `B_s`, voids `V_s`, scores) computed from the action list alone, with
    the upper-bound-only void update `voids_mono` (PA-C01/C02/C06);
  - `Coheres` — the invariant tying a deal's objective replay to the
    public replay, carrying void soundness; preserved by every legal play
    (PA-C09/C10 groundwork);
  - the derived cells `pool`/`allowed`/`capacity` and the fiber `IsWorld`
    (PA-C02–C04), `Compatible` deals and the `remainder` map (Math §6.3);
  - **`losslessness`** (PA-C05/C07): the cell fiber equals the remainder
    image of the rule-compatible deals — soundness direct from coherence,
    completeness by the spec's four-case induction (viewer act / hidden
    lead / hidden follow / hidden slough);
  - `remainder_injective` (PA-C09): the fixed-history bijection.

  Mechanization finding: the reverse construction needs a fact the prose
  leaves implicit — a hidden seat's publicly played tile must respect that
  seat's previously recorded voids; it is derived from true-trajectory
  void soundness (`hd_allowed` inside the completeness proof).
- `Texas42/Reachability.lean` — `Reachable`, `CertifiedState` with
  equality through the projection (PA-D09/D10, TYPE-01).
- `Texas42/Information.lean` — `DealLocalInfo` and `mech_not_injective`:
  the Math §6.6 counterexample (PA-F05).
- `Texas42/Reduction.lean` — the generic capacitated cell kernel
  (rec K7): `CellSys`, marginal holder support, and the canonical
  reduction — fiber-preserving, contractive, idempotent, coordinatewise
  least, with the coarsest-exact-quotient equivalence
  `Φ(C) = Φ(Q) ↔ red(C) = red(Q)` (Math §7.9, PA-C15 backbone); the game
  cells instantiate it (`ViewerCtx.cellSys`).
- `Texas42/NormalForm.lean` — the exact support normal form
  (Math §§7.10–7.11, PA-D01–D05):
  - the marginal anatomy of a feasible system — exact holder sets
    `A(d)`, certain marks `K_s`, ambiguous pool `W`, residuals,
    active seats — with the pinning lemma `K_s ⊆ A s` in every world;
  - `active_trichotomy` — with three holders `|J⁺| ∈ {0,2,3}`, plus the
    binary (`A(d) = J⁺`) and ternary (`≤ 1` exclusion) patterns
    (PA-D01);
  - `strict_singleton_hall` — the §7.11 inequality `|N({s})| ≥ r_s + 1`,
    proved at the marginal level with no matching theory;
  - `exists_partition_of_hall` — a generic capacitated Hall lemma by
    slot expansion into mathlib's Hall theorem (PA-C08 groundwork);
  - `SupportNF` with `WellFormed` — the validated normal-form type of
    handoff §6, its ternary branch carrying the §7.11 linear validator
    (PA-D02);
  - `decode` with `feasible_decode` and `decode_marginal` — well-formed
    forms decode to feasible systems whose marginal relation is exactly
    the declared payload; every declared edge is realized through the
    forced-edge partition (PA-D03);
  - `compile` with `wellFormed_compile`, `decode_compile`
    (`decode (𝒩(C)) = red(C)`), and `compile_decode`
    (`𝒩(decode N) = N`) — the inverse laws (PA-D03/D04);
  - `totalNF` and **`fiber_eq_iff_totalNF_eq`** — the `Empty`-tagged
    total form classifies exact fibers over *all* systems, feasible or
    not: `Φ(C) = Φ(Q) ↔ 𝒩̄(C) = 𝒩̄(Q)` (PA-D05, the §7.10 global
    representation-minimal support quotient).
- `Texas42/Belief.lean` — the finite belief layer (PA-E01–E03,
  Math §§8.1–8.3): exact rational `FinPMF` with Bayes `condition`
  (normalization + the `condition_mul` chain rule), policy kernels and
  the history-likelihood product, `posterior` on the fixed deal domain,
  `physicalBelief` pushforward, and
  `physicalBelief_support_isWorld` — the posterior's support lies inside
  the cell fiber (support is not belief; the fiber bounds it exactly).

- `Texas42/Strategic.lean` — strategic sufficiency (PA-E07,
  Math §§10.1–10.2): `BeliefProc`, a finite-horizon viewer decision
  process with latent state — Markov observation/latent kernel, exact
  record transition, Bayes filter (`filter`), segment reward and
  terminal utility; `latentVal` (ground truth along true latent
  trajectories) and `beliefVal` (the §10.2 Bellman recursion in
  `(s, β)`, one boundary convention, zero-probability segments
  contributing zero); **`beliefVal_eq_exp_latentVal`** — the expected
  continuation value of every fixed admissible strategy is a function
  of the strategic state `B = (c, e, β)`, by backward induction on the
  remaining-play grade exactly as §10.1; `bestResponse_eq` — the
  fixed-field best-response value over any finite strategy class is a
  function of `B`.

- `Texas42/Witness.lean` — **the exact legal 90-world witness**
  (PA-E10, Math §10.4; K15's named theorem):
  - the §10.4 endpoint internalized — contract `P(31)` no-trump by
    seat 3, the five-trick public prefix, the six-tile pool, and the
    endpoint cells computed from the public record (voids, capacities,
    pool) by kernel evaluation;
  - the 90-world fiber enumerated (`Fin 90`-indexed, tied to the
    `powersetCard` cells by decidable image equality) and characterized:
    `isWorld_iff` proves the cell fiber is exactly the 90 worlds, and
    `rule_fiber` realizes every world by a rule-compatible complete
    deal replaying the prefix legally (90 kernel replays);
  - both §10.4 auction histories legal, distinct, same result;
  - the two Bayes posteriors (PA-E02 conditioning of the uniform fiber
    prior), with identical full support;
  - the value columns `Q(·, 3:1)`, `Q(·, 4:1)` kernel-verified by 180
    deterministic lowest-ID rollouts of `PlayState.step`, matching
    §10.4's anchors and class means exactly (`-160/21, 10/7, -217/30,
    -52/5`; make probabilities `1/3, 16/35, 1/3, 1/5`);
  - **`ninety_world_witness`** — same mechanical endpoint, same
    90-world fiber, same posterior support, opposite optimal leads
    under both the expected-differential and contract-make lenses.
    Mechanical state alone is not an exact strategic state.

- `Texas42/ConstellationCore.lean` — the constellation thread's core, in its
  own `Constellation` namespace (x:013):
  - `Pip`, `Domino` (canonical `(high, low)` subtype), `countPoints`, the nine
    `Declaration`s and eight `SuitContext`s, the called set, effective-suit
    `follows` and `ledContext`;
  - the lexicographic tier/rank trick key `trickKey : Fin 42`, with `winner`
    and `award` on a completed `Trick`;
  - `winner_maximal`, `positive_key_injective` (by `decide` over every
    declaration/context/tile pair), and `unique_winner` — four distinct
    dominoes led by the first play have a unique key-maximal winner; two
    worked tricks are checked by kernel evaluation.
- `Texas42/ConstellationSuffix.lean` — depth-`k` suffix play and its exact
  value (x:015):
  - `SuffixPos k` (four `k`-tile hands, a leader, a declaration) and the
    `MidState` machine — `actor`, lead/follow-if-possible `legalMoves`,
    trick-resolving `step`, with `legalMoves_subset_hand` and
    `step_remaining`;
  - fuel-indexed exact `minimax` over the `4 * k` remaining plays, signed
    from the `(0,2)` partnership, and `value` with `value_k1_forced` — at
    depth one the forced trick alone fixes the margin;
  - two `k = 1` values (`-11`, `16`) evaluated by the kernel.

  These two files are self-contained: they re-derive their own core rather
  than importing Layers A–B, and are not yet reconciled with `Basic.lean` and
  `Trick.lean`.

All theorems depend only on the standard axioms
(`propext`, `Classical.choice`, `Quot.sound`) — no `sorry`, no `native_decide`.

## Building

```sh
# elan (toolchain manager) must be on PATH; it lives in ~/.elan/bin
cd lean
lake exe cache get   # fetch prebuilt mathlib oleans (multi-GB, one-time)
lake build
```

The toolchain is pinned by `lean-toolchain` to match mathlib's pin.
Build artifacts live in `.lake/` (gitignored). The `.github/` workflows from
the mathlib template are inert while `lean/` is a subdirectory (GitHub only
runs workflows from the repository root); they are kept for a possible future
split into a standalone repo.
