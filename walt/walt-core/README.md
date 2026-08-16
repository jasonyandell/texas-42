# walt-core

Owns the Straight 42 rules layer of `walt/math/unified_information_geometry_v0.4.md`
§1: pips, the 28 dominoes, seats and teams, the nine declarations, effective
contexts under absorption, the rule algebra (called set, effective incidence,
follow, led context, tier, declaration-relative rank, lexicographic trick key,
BEATS/THREAT), the count decoration and trick scoring, follower legality, and
receipt parsing plus history replay.

**Imports: nothing** (std only, no third-party dependencies). Every crate above
walt in the graph gets its rules from here; nothing else may restate them.

Derived views are functions, never stored beside the state: effective
incidence, legal sets, remaining hands, and observable voids are all recomputed
from the semantic state.

Validated by `tests/receipt_replay.rs`, which re-derives all 13 hands of
`rob/receipts/verify_player.txt` (READ-ONLY) from these rules alone, and by
`tests/exhaustive.rs`, which asserts every exhaustive count the spec states --
including unique trick winners over all 737,100 four-tile tricks across the
nine declarations.
