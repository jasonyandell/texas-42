# Texas42 — Lean 4 formalization

Kernel-checked formalization of the Texas 42 foundations, as a
[Lake](https://github.com/leanprover/lean4/tree/master/src/lake) project
depending on [mathlib4](https://github.com/leanprover-community/mathlib4).

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
