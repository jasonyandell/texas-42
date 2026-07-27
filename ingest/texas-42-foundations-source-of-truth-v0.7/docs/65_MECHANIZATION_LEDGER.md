# Mechanization Ledger

This ledger is the first proof-assistant work queue. It is intentionally much
smaller than the full mathematical claim ledger. A row enters this list only
when later core theorems depend on it or when it guards a known historical
overclaim.

Status values:

- `DEFINE` — introduce the object with decidable equality/finite enumeration;
- `PROVE` — direct kernel proof preferred;
- `REFLECT` — proved internal decision procedure plus kernel evaluation;
- `WITNESS` — concrete internal counterexample/computation;
- `DEFER` — not needed for the first closed finite foundation.

## A. Finite rule algebra

| PA-ID | Target | Depends on | Route | Priority |
|---|---|---|---|---|
| PA-A01 | `Pip`, `Domino`, `Seat`, `Team`, `Declaration`, `LedSuit` finite types | none | DEFINE | 0 |
| PA-A02 | enumeration of exactly 28 canonical dominoes | PA-A01 | PROVE/REFLECT | 0 |
| PA-A03 | natural incidence covering and pair intersections | PA-A01 | PROVE | 0 |
| PA-A04 | count-point function and total 35 | PA-A02 | PROVE/REFLECT | 0 |
| PA-A05 | called/powered/effective-suit definitions for nine declarations | PA-A01–A03 | DEFINE | 0 |
| PA-A06 | effective membership is one or two; called absorption | PA-A05 | PROVE | 0 |
| PA-A07 | led context and follow exactness | PA-A05 | PROVE | 0 |
| PA-A08 | rank, tier, trick key | PA-A05–A07 | DEFINE | 0 |
| PA-A09 | lead always occupies nonzero tier | PA-A07–A08 | PROVE | 0 |
| PA-A10 | rank injective inside each possible winning tier | PA-A08 | PROVE | 0 |
| PA-A11 | unique trick winner | PA-A09–A10 | PROVE | 0 |
| PA-A12 | exhaustive prose-resolver agreement | PA-A11 plus independent internal resolver | REFLECT | 2 |
| PA-A13 | contextual `BEATS` exactness | PA-A08–A11 | PROVE | 0 |
| PA-A14 | live-threat monotonicity | PA-A13 | PROVE | 0 |
| PA-A15 | lead-threat incompleteness witness | PA-A07, PA-A13 | WITNESS | 1 |
| PA-A16 | count-preserving pip maps are identity and `2 <-> 3` | PA-A02, PA-A04 | PROVE/REFLECT | 1 |
| PA-A17 | scoped `2 <-> 3` declaration transport | PA-A05–A13, PA-A16 | PROVE/REFLECT | 1 |

## B. Auction, contract, and objective hand

| PA-ID | Target | Depends on | Route | Priority |
|---|---|---|---|---|
| PA-B01 | configured Straight auction state/action types | PA-A01 | DEFINE | 0 |
| PA-B02 | auction legality and deterministic transition | PA-B01 | DEFINE/PROVE | 0 |
| PA-B03 | reachable mark ceiling `min(maxMarkBid,5)` | PA-B02 | PROVE | 1 |
| PA-B04 | exact finite auction history counts for caps 1..7 | PA-B02 | REFLECT | 3 |
| PA-B05 | uniform ordered deal type and finite cardinality | PA-A02 | DEFINE/PROVE | 1 |
| PA-B06 | contract/declaration/award definitions | PA-A05, PA-B02 | DEFINE | 0 |
| PA-B07 | phase-indexed full location and contracted-play state | PA-A01, PA-B06 | DEFINE | 0 |
| PA-B08 | legal-play characterization | PA-A07, PA-B07 | PROVE | 0 |
| PA-B09 | atomic play transition preserves partition/location invariants | PA-A11, PA-B07–B08 | PROVE | 0 |
| PA-B10 | seven tricks, 28 plays, 42 total points | PA-A04, PA-B09 | PROVE | 0 |
| PA-B11 | fixed-hand play graph is finite and graded | PA-B09 | PROVE | 1 |
| PA-B12 | objective physical Markov congruence | PA-B07–B09 | PROVE | 1 |
| PA-B13 | deterministic contract/mark settlement | PA-B06, PA-B10 | PROVE | 1 |
| PA-B14 | early settlement is a scoped outcome quotient, not full-play equality | PA-B13 | PROVE | 2 |

## C. Information, cells, and fiber

| PA-ID | Target | Depends on | Route | Priority |
|---|---|---|---|---|
| PA-C01 | primitive public/private event history and perfect-recall information | PA-B01–B09 | DEFINE | 0 |
| PA-C02 | mechanical projection and derived `unseen/capacity/possible` cells | PA-C01, PA-A07 | DEFINE | 0 |
| PA-C03 | cell dependency and Hall feasibility statement | PA-C02 | DEFINE | 0 |
| PA-C04 | current-remainder world and fiber | PA-C02 | DEFINE | 0 |
| PA-C05 | initial deal/remainder correspondence | PA-B05, PA-C04 | PROVE | 0 |
| PA-C06 | upper-bound-only observation update | PA-A07, PA-B09, PA-C02 | PROVE | 0 |
| PA-C07 | cell/fiber losslessness over legal Straight prefixes | PA-C05–C06 | PROVE by induction | 0 |
| PA-C08 | Hall/max-flow feasibility equivalence | PA-C03–C04 | PROVE | 1 |
| PA-C09 | hidden-play typed predecessor/successor bijection | PA-C06–C08 | PROVE | 0 |
| PA-C10 | viewer-play identity on hidden remainders | PA-C06 | PROVE | 0 |
| PA-C11 | exact capacity-DP count recurrence and soundness | PA-C04 | PROVE | 1 |
| PA-C12 | exact uniform count-ratio sampler under named law | PA-C11 | PROVE | 2 |
| PA-C13 | local possible holder is not marginal possible holder | PA-C04 | WITNESS | 1 |
| PA-C14 | marginal holder edge iff forced successor feasible | PA-C08 | PROVE | 1 |
| PA-C15 | support reduction preserves fiber and is canonical | PA-C14 | PROVE | 1 |

## D. Normal form and reachability

| PA-ID | Target | Depends on | Route | Priority |
|---|---|---|---|---|
| PA-D01 | determinate/binary/ternary trichotomy | PA-C15 | PROVE | 0 |
| PA-D02 | `WellFormedFeasibleSupportNormalForm` | PA-D01 | DEFINE | 0 |
| PA-D03 | decode is feasible and reconstructs reduced holder relation | PA-D02 | PROVE | 0 |
| PA-D04 | compile/decode inverse laws | PA-C15, PA-D03 | PROVE | 0 |
| PA-D05 | total support normal form classifies exact fibers | PA-D04 | PROVE | 0 |
| PA-D06 | global deterministic support minimality/factorization | PA-D05 | PROVE | 1 |
| PA-D07 | one-assignment SCC marginal compiler soundness | PA-C14 | PROVE | 2 |
| PA-D08 | strict Hall irreducibility and essential exclusions | PA-D01–D03 | PROVE | 2 |
| PA-D09 | reachability predicate on mechanical states | PA-B09, PA-C02 | DEFINE | 0 |
| PA-D10 | reachability is proof-irrelevant semantic evidence | PA-D09 | PROVE/extensionality | 0 |
| PA-D11 | reachable support image and restricted minimality | PA-D04, PA-D09 | PROVE | 1 |
| PA-D12 | exact 50 hidden-capacity profiles | PA-B09, PA-D09 | PROVE/REFLECT | 2 |
| PA-D13 | seven observable lead contexts per declaration | PA-A07 | PROVE | 1 |
| PA-D14 | lead-witness necessity | PA-A07, PA-D09 | PROVE | 1 |
| PA-D15 | witness validator soundness/completeness | PA-D09, PA-D11 | PROVE | 1 |
| PA-D16 | feasible-but-unreachable support | PA-C08, PA-D09 | WITNESS | 1 |
| PA-D17 | exact full-schema 81-bit support census | PA-D04–D06 | REFLECT or keep external | 4 |
| PA-D18 | reachable-support 26–46-bit interval | PA-D11–D16 | PROVE/REFLECT | 4 |

## E. Finite belief and strategic boundary

| PA-ID | Target | Depends on | Route | Priority |
|---|---|---|---|---|
| PA-E01 | finite PMF prior and policy/field kernel | PA-C01 | DEFINE | 0 |
| PA-E02 | history likelihood product and posterior normalization | PA-E01 | PROVE | 0 |
| PA-E03 | pushforward belief to current remainder fiber | PA-C05, PA-E02 | PROVE | 0 |
| PA-E04 | physics-only posterior under uniform chance assumptions | PA-B05, PA-E02 | PROVE | 1 |
| PA-E05 | finite exponential-tilt form | PA-E02–E04 | PROVE | 1 |
| PA-E06 | forced-action world-nondiscrimination | PA-B08, PA-E02 | PROVE | 1 |
| PA-E07 | exact augmented strategic state sufficiency for fixed field/utility | PA-B12, PA-E02–E03 | PROVE | 0 |
| PA-E08 | deterministic information-set best-response existence | PA-B11, PA-E07 | PROVE | 1 |
| PA-E09 | coordinate-only scalar factorization criterion | PA-E07 | PROVE | 1 |
| PA-E10 | same-support 90-world posterior/action reversal | PA-E01–E09 | WITNESS/REFLECT | 0 |
| PA-E11 | context-free domino value counterexample | PA-B09, PA-E07 | WITNESS | 1 |
| PA-E12 | threshold-utility reversal | PA-E07 | WITNESS | 2 |

## F. Quotients, gauges, and boundaries

| PA-ID | Target | Depends on | Route | Priority |
|---|---|---|---|---|
| PA-F01 | local hand-order invariant/equivariant gauge | PA-B07–B09 | PROVE | 1 |
| PA-F02 | simultaneous seat rotation symmetry | PA-B01–B13 | PROVE | 2 |
| PA-F03 | bidder anchoring as transported post-auction gauge | PA-F02, PA-B06 | PROVE | 2 |
| PA-F04 | reflection failure witness | PA-B09 | WITNESS | 2 |
| PA-F05 | physical congruence versus information-state equality | PA-B12, PA-C01–C02 | PROVE | 0 |
| PA-F06 | field-relative strategic isomorphism theorem | PA-E07 | PROVE | 2 |
| PA-F07 | shared utility does not centralize partner information | PA-C01, PA-E07 | PROVE/BOUNDARY | 1 |

## Deferred modules

| Topic | Reason for deferral |
|---|---|
| standard-Borel latent fields | finite PMF core proves the native game first |
| full match almost-sure termination | repeated all-pass attempts require a separate stochastic theorem |
| special contracts | different information, visibility, or scoring rules |
| CFR/regret convergence | depends on exact extensive-form/team solution concept |
| implementation bitmasks and packed addresses | require refinement proofs after semantic closure |
| neural architecture and experiments | empirical hypotheses, not foundation theorems |

## Definition of done for the first proof-assistant release

The first formal release should include all priority-0 rows and no unmarked
axioms beyond the adopted Straight rule profile and ordinary foundational
library assumptions. Every external finite receipt not yet internalized must
remain visibly external in the generated theorem inventory.
