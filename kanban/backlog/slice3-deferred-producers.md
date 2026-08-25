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

## Done when

Each item either has a producer with gates (and the probe README's
Deferred section shrinks accordingly), or a recorded ruling that it is
not wanted. Partial closure is fine — these are three independent
design questions, not one deliverable.

## Links

[[panel-response-audits]], [[slice3-cancellation-ladder]],
[[level2-field-swap-probe]], `walt/probes/fieldswap_cancel/README.md`,
wiki/walt-math-open-questions.md §10
