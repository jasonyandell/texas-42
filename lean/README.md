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
