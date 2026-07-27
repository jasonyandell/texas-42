# Changelog

## v0.7 — proof-assistant boundary revision — 2026-07-26

This revision preserves v0.6's mathematical results and finite numerical
receipts. It repairs the semantic/executable boundary before mechanization.

### Corrected

- Reachability is now a proof-irrelevant proposition on physical/mechanical
  state. Replay witnesses and lifecycle provenance no longer participate in
  game-state equality, hashing, serialization, or transition.
- `MechanicalState` no longer stores rule cells as a second semantic source of
  truth. Cells, support reduction, normal form, fiber, and native-hand view are
  derived functions; coherent caches are explicitly nonsemantic.
- `NativeHand` is renamed `NativeHandView` in the executable specification to
  make its derived-view status explicit.
- Hidden-support transition pseudocode now updates semantic physical fields and
  re-derives cells rather than mutating duplicate cell state.
- The feasible support normal form now has one complete well-formedness
  contract and explicit compile/decode round-trip obligations.
- Claim-ledger absolute-value notation no longer breaks Markdown tables.
- The package no longer calls its two verifier entry points independent
  implementations; the second reuses helpers from the first.
- The 46-bit necessary outer-language object is renamed from a reachability
  certificate to `ReachabilityOuterNecessaryProfile`; passing its check remains
  necessary only and cannot construct a reachable state.

### Added

- `docs/55_V06_REVIEW.md` — adversarial comparison and revision rationale.
- `docs/60_PROOF_ASSISTANT_HANDOFF.md` — formalization order, trust boundary,
  type design, and milestone plan.
- `docs/65_MECHANIZATION_LEDGER.md` — prioritized theorem work queue.
- `docs/70_THREAD_CONTINUITY.md` — crosswalk from the founding thread and
  explicit deferred annexes.
- `provenance/RESEARCH_MANDATE_2026-07-23.md` — verbatim founding mandate,
  non-normative.
- Claim-ledger rows for proof-irrelevant reachability, derived support views,
  normal-form validity, and the external-verification boundary.

### Unchanged

- Straight rule profile;
- declaration algebra;
- auction counts and reachable mark ceiling;
- exact cell/fiber theorems;
- support-normal-form mathematics and census;
- reachability bounds and feasible-unreachable witness;
- 90-world posterior/value counterexample;
- all recorded verifier numerical outputs.
