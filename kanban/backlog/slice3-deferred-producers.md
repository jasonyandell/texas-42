id: [[slice3-deferred-producers]]
opened: 2026-08-24

## What

The three build-list items slice 3 deferred LOUDLY rather than
approximating them [L2 thread]. Each needs a design pass before it can
have a producer; each is stated in the probe README's Deferred section
(`walt/probes/fieldswap_cancel/README.md`) and adopted-but-unbuilt on
the PANEL-A7/A8 list carried by [[panel-response-audits]]. Closing card:
[[slice3-cancellation-ladder]] (PR #38, main `151ea4f`).

1. **δ-valid admissible-upper E3 producer.** §7.4 admits a sampled route
   into the screen only through an exact value, an admissible upper
   bound, or a valid structural over-approximation. The E3 that exists is
   the ESTIMATE sibling, typed so it cannot enter a screen. Deferred
   because a valid upper bound on a **supremum** is not a mean-style
   evidence problem — the evidence engine has no instrument for it and
   inventing one silently would be the exact error the typing prevents.
2. **Dominance valid-bound route.** PANEL-A7 admits `Dominated` via
   "exact enumeration **or** a valid bound"; only the exact-enumeration
   producer (H = 0 ∧ B > 0) was built. Deferred because a structural
   hazard-bound type with no producer behind it invites misuse — the
   sampled masses deliberately have no dominance method at all
   (type-level lock), and that lock is only sound while no half-built
   bound route exists beside it.
3. **§10 motif tags (item 14).** `FieldSplitTrace` carries items 1–13
   (precision: item 11, the distinguishing public observation, has no
   dedicated field — implicit in tile0/tile1/history; flagged as an
   enrichment candidate in the drafted dispatch); the structural motif
   vocabulary (e.g. "reveal-response") is **absent, not approximated**. Deferred because naming a motif before the
   vocabulary is designed would make aggregates readable and wrong;
   today's aggregates ship seat/trick histograms and the conditional
   outcome difference only.

## Adjudicated designs (x:024 response, intaken 2026-08-25)

All three deferrals now have adjudicated designs (rulings TRIPLE-A1..A7,
`walt/CENSUS-RULINGS.md`; intake
`walt/math/response_deferred_producers_triple_v0.1_intake.md`). The card
stays open: closure still requires gated producers, per Done-when.

1. **E3 producer = max-preserving upper CS.** No new estimator needed:
   the shipped `sampled_split_reach` count IS S\*_n (verified at
   intake). Build: exact inversion of the one-mean lower-tail e-process
   on G_N at S\*_t, prefix minimum, typed result with (δ, epoch,
   prefix, policy-class id), risk wiring per §1.8 (risks still sum
   across screen inputs). Directional variants = separate solves,
   separate ledger entries, coupled branches to terminal.
2. **Dominance bound = Hazard-Exclusion Invariant.** Build the general
   invariant verifier FIRST (single authority; sound H1, semantically
   complete H2), then the one-round trump-extraction witness as the
   first deliberately incomplete producer (its refusal path is part of
   its correctness; the three-trick specimen is the standing
   non-coverage instance). No cross-field composition without a
   field-action-family witness. Type lock unchanged: sampled masses
   never reach `StructuralHazardZero`.
3. **Motifs = six-motif first-split morphology + Other**, least
   differing coordinate of the six-coordinate signature, orthogonal
   flags mandatory, `Other(missing_root_frame)` never guessed.
   BINDING: current traces partition correction mass, never exposure.
   `RevealResponse` stays refused; the prerequisite is schema
   enrichment (branch suffixes + root_semantics_hash — also closes the
   item-11 gap), and even then only `PartnerResponseCandidate`.
   *Status 2026-08-25: SHIPPED with gates (slice 4c) —
   `walt/walt/src/solver/motif.rs` (classifier, flags, root-frame
   resolution, suffix enrichment, exact m_k⁺/m_k⁻ decomposition,
   descriptive-tier lock), tests
   `walt/walt/tests/solver_fieldswap_motifs.rs`, probe
   `walt/probes/fieldswap_motifs/` (453 correction worlds classified,
   residual 0). `RevealResponse` still refused, per TRIPLE-A7.*

Advisory build order (response §5): E3 → directional E3 → invariant
verifier → one-round producer → motif classifier; no `RevealResponse`
until suffixes land. Three compact mechanization candidates recorded
for Jason's unified Lean treatment (his side project with Pro).

## Done when

Each item either has a producer with gates (and the probe README's
Deferred section shrinks accordingly), or a recorded ruling that it is
not wanted. Partial closure is fine — these are three independent
design questions, not one deliverable.

## Links

[[panel-response-audits]], [[slice3-cancellation-ladder]],
[[level2-field-swap-probe]], `walt/probes/fieldswap_cancel/README.md`,
wiki/walt-math-open-questions.md §10
