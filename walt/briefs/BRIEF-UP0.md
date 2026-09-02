# BRIEF-UP0 — the unified walt player, slice 0: one decision core, every instrument, provenance always

**Authorized:** 2026-09-02, Jason's §76 GO ("it is also entirely good with me
to make a new unified Walt and or Walt player... the interesting one is the
new one. thats what I'm itching to play") and the overnight build grant
("if the draft feels worth building you should build it"). **Boundary kept:**
building the new player ≠ making it the default ≠ arena-scale evaluation —
the latter two stay on Jason's word. The old player is not to be protected
but also not to be touched here: UP0 is ADDITIVE. Unusably slow is
acceptable; dishonest is not.

**Read first, in order:** `walt/FACTOR-BELIEF.md` (newest paragraphs: MB1,
U0, sigma1-repair, MB0), `walt/briefs/MB1-REPORT.md` ("What the unified
player would need from this module" + the affordability wall), 
`walt/briefs/U0-REPORT.md` (the fusion horizon + "For the unified player"),
then the instruments as you wire them: `solver/proof_state.rs`,
`solver/extraction.rs`, `solver/frontier.rs`, `solver/opening.rs`,
`solver/godgap.rs`, `solver/model_belief.rs`, `solver/model_recursion.rs`.

## Mission

One decision function for live play — seat, history, hand in; action out —
that consults every exact instrument the counted-belief program built, in
the order the mathematics says they become affordable, with the model-belief
posterior carried down the line (the §76 join), and with EVERY decision
carrying a typed provenance record naming the instrument that produced it,
the budget it spent, and every refusal it fell through on the way. The
player never panics and never silently truncates: the fallback cascade is
total and ends at a raw σ0 evaluation that always answers.

The sequencing input is measured, not guessed (U0 + MB1): from trick 5 in,
exact instruments are effectively free (fibers ≤ 200, ≤ 3.7 s) and God-tight
receipts certify optimality; trick 4 is minutes per root — affordable for
analysis, declared-budget territory for live play; trick 3 and earlier are
past the wall — exact-mixture reads there get typed refusals, and the player
falls back. Jason's frame, now twice-measured: "42 is 2 recursions running
in opposite directions." UP0 is the first artifact that PLAYS both
recursions — enumerable exactness walking backward, sampled/structural play
walking forward — and knows at every depth which one it is standing in.

Build items (restructure freely if the math says so — report deviations):

1. **The decision core** (`solver/unified.rs`, new): `UnifiedPlayer` with
   `decide(seat, history, hand, budget) -> Decision`. `Decision` carries
   the action AND a `Provenance` — a typed, append-only record of the
   cascade: which tier answered, exact spend in field consultations, every
   typed refusal passed through. Derived views only — the player stores no
   authority the semantic state doesn't already own; the posterior and
   proof-state facts are carried, never duplicated.
2. **The tier cascade**, deepest-certainty first, each tier entered only on
   affordability (declared budget, §34/§35 style) and exited only by
   typed refusal:
   a. terminal/decided arithmetic (free);
   b. endgame exact: at roots where the fiber affords enumeration
      (measured t5/t6 band), consume the exact recursion — and where a
      God-tight receipt exists in the store, CONSUME it (§39 frontier-record
      shape; godgap.rs is consume-only) rather than recompute;
   c. middlegame budgeted: MB1's `mixture_response_budgeted` under the
      declared per-move budget, repricing the priced library under the
      carried posterior where the envelope allows (the one built lever:
      walk once, re-decide free);
   d. certified-regret recommendation off the proof state (§33 recommend()
      / extraction argmax) where facts already installed make Γ small or
      zero;
   e. σ0 fallback: total, always answers, named as such in provenance.
3. **The join**: construct `ModelBelief` at the root, `focal_play`/
   `observe` down the line as play proceeds. The posterior is carried,
   never recomputed. Every tier that can consult it does; provenance
   records when the posterior CHANGED the decision vs the fixed-field
   answer (MB1 measured: values move before argmaxes — record both).
4. **Budget discipline**: per-move budget in field consultations (the unit
   the ledger already measures), declared at construction, never wall-clock.
   All refusals typed all the way out. No unbounded walks anywhere — the
   repaired sampler rules apply to every sampled mind.
5. **Self-play probe** (`unifiedreport` →
   `walt/probes/factor_belief/unified_run1.txt`): walk each receipt root to
   terminal with UP0 choosing every seat's action under a declared budget
   ladder (at least two budgets). Per move: tier answered, spend, refusals,
   posterior-consulted flag, wall time. Findings language only. This is a
   TRANSCRIPT, not an evaluation — no strength claims, no comparison to the
   old player (that is arena territory, Jason's word).
6. **Field-identity fence inherited**: any fact consumed across a field
   identity goes through MB1's coupling machinery (`couple_fixed_field_fact`
   is the only route). UP0 transports nothing new; the fence stays gated.

## Gates (`walt/walt/tests/solver_unified.rs`)

- UP1 totality: on every receipt root and a sweep of mid-walk states, every
  legal state gets a Decision with well-formed Provenance; the cascade
  terminates; zero panics/expect/unwrap on instrument results (grep-gated
  like sigma1-repair's R6).
- UP2 endgame consistency: where a God-tight receipt exists (U0's 14
  persisted profiles), the unified decision re-prices to the God upper —
  consumed, not recomputed; byte-identical godgap/doom instruments either
  side of a decision (M5/G5 style).
- UP3 the join: the carried posterior equals MB1's trace on the same line
  (derived-view law); the value-moves specimen pinned (h8-t5 770→762‰
  or whatever the walk yields); if any argmax flips under the posterior on
  the probe corpus, pin it — if none flips, the gate pins the honest
  absence (censused, corpus-scoped, never "posteriors don't matter").
- UP4 budget honesty: decisions under a starved budget fall through with
  every refusal typed and the σ0 fallback named; same state + same budget
  → byte-identical Decision (determinism, transcript-stable).
- UP5 the inherited suites: MB0 8 + sigma1 7 + U0 6 + MB1 7 all green
  untouched (property gate + check.sh conjunction, MB1's M5 pattern).
- UP6 provenance soundness: a Decision's claimed tier is verifiable —
  a decision claiming tier (b) carries the enumeration's exact value; one
  claiming (c) carries the ledger spend; fabricating a tier is
  unconstructible or gate-caught.

## Discipline

House rules unchanged: check.sh green; no floats; vocabulary greps
("necessary outer profile", never bare "certificate"); EXPLORATORY tier;
checkpoint-commit per item, never squash; final commit `walt UP0:`; no
push/PR (orchestrator lands); ambiguity protocol; FACTOR-BELIEF.md
paragraph; independent audit follows (mb0-builder-3), anchors at base.
Consume-only: ingest/, refine.rs (freeze 58), doom.rs (§47), godgap.rs
(U0), AND model_belief.rs + model_recursion.rs + both their gate files —
UP0 is a CONSUMER of the recursion, and the freeze is what makes gate UP5's
inheritance claim checkable rather than asserted (the doom→U0 and
godgap→MB1 precedent). If you genuinely need an additive API surface on
either module (a public getter, a re-export), STOP and message the
orchestrator with the exact need before writing it — that path is
ruled-per-case, never silent. The old player (walt_bridge, playout, playtable,
webtable, level1_evaluate) is NOT TOUCHED in UP0 — the unified player is a
new artifact beside it; bridging/defaults/arena are future slices on
Jason's word.

Report via SendMessage ONLY (plain final text is never delivered); keep
long waits on foreground-visible work, never sleep on a Monitor (two
session drops last night came from Monitor blocks); commit
`walt/briefs/UP0-REPORT.md` before declaring done; WIP checkpoint after
every item.

## Report back

Slice status; gates; the probe transcript's shape (tier occupancy by trick
— where each recursion actually answered); the join specimens (value-moves
and any argmax flip); per-move wall times under both budgets; what UP1
(bridging/interactivity) and UP2 (structural producers past the wall) need;
deviations.
