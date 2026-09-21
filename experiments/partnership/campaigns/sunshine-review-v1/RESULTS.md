# General move statistics restored — 2026-09-13

This is a review instrument change, not a player-strength experiment. Native
live presets and their choices are unchanged. Plunge companion commit:
`ac7e65a20f213823bfb926ad62e5b3c5443bbb4c`.

The Mac review displays original completed option estimates, correctly oriented
to the acting team's make/set objective. Rejected primary results never replace
accepted fallback evidence, which retains its actual sample size. Fresh 40/160
world native L1 estimates use only the actor's hand and public prefix and are
stored separately. No hidden examiner hands enter that endpoint.

## Validation

- All 149 Plunge tests passed. After layout/parser changes, all 18 focused
  native stats, transport, cache and component tests passed again.
- All 100 partnership Python tests passed. Added coverage for 40/160 cache
  separation, immutable original playing evidence, presentation-independent
  reuse, strict own/public input, concurrent inspection rejection, separate
  live worker, retry after incomplete evaluation, and worker cleanup.
- Native-mode typecheck/build passed, including the final two-column layout.
- Live native HTTP audit verified 40/160 completed estimates, exact cache
  reuse, forced moves without scores, hidden-input rejection, and unchanged
  hashes for all original decisions, flags and gym inputs.
- Browser checks covered both saved examples, the original/fresh distinction,
  preservation of the user's note/alternative, and a human move's 40-world
  analysis. Screenshots identified and corrected low-contrast helper text.

Run receipts and the live audit script are under
`/Users/jason/data/texas-42/sunshine-review-v1/`. Test/build/audit runs used the
existing external 295-second watchdog. Individual native inspections retain
the existing 14-second decision deadline.

## Concrete examples

Flag `5ed686b48b5e493e84866704d7b9b862`: Ruby discarded 6–6 instead of 5–1.
The recorded 40-world sample gives 24/40 sets for 6–6 (60%) versus 22/40 (55%)
for 5–1. A later native 160-world recheck gives 87/160 (54.375%) versus 75/160
(46.875%), again selecting 6–6. The UI preserves both samples. These are
model-relative estimates, not proof that 6–6 is the optimal move.

Flag `dbd53c9c432e444cb2db9010bcdc1f27`: Earl's 2–2 overtook partner Ruby's
6–2 lead. Deuces were trump; 6–2 led trumps, and 2–2 was Earl's only remaining
trump. The view shows the legal constraint and no probability table.

Selecting the human double-three lead in the first flagged hand and asking
for 40 worlds produced a complete five-option native comparison in 0.31s.
The double-six 160-world inspection took 0.55s. These are example timings,
not a general performance estimate.
