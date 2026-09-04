# BRIEF-FH4 — independent audit of the focal-horizon program before it lands

**Authorized:** 2026-09-04 (Jason: one PR after an independent audit;
the builders cannot audit themselves — the standing audit-independence
policy: never build what you audit; the independence IS the
instrument). **Scope:** every commit on branch `walt-fh` from the merge
base with `main` (`a80b982`, PR #87) to its tip: FH0 (intake and
rulings), FH1 (engine), CI1 (concurrent runner + fixtures), FH2
(ladder), FH3 (report of record + anchors), and the orchestrator's
process commits (CLAUDE.md "Agents" section and gate-sizing rule,
`walt/MAP.md`, briefs). You did not write any of it.

Read first: `CLAUDE.md`; `walt/MAP.md`; the FH-A1..A11 rulings section
at the tail of `walt/CENSUS-RULINGS.md`; the companion
`walt/math/focal_horizon_sandwich_v0.1_intake.md`; the four reports
`walt/briefs/FH1-REPORT.md`, `CI1-REPORT.md`, `FH2-REPORT.md`,
`FH3-REPORT.md`; then the code and gates. `git log --oneline
a80b982..HEAD` and `git diff --stat a80b982..HEAD` are your inventory.

**EXPLORATORY tier throughout;** the audit changes no tier.

## The audit moves (each yields PASS, BLOCK, or NOTE with a citation)

1. **Gate, not prose.** For every numbered claim in the four reports
   and in `walt/FACTOR-BELIEF.md`'s new paragraphs, find the assertion
   that makes it true. A number in prose with no gate behind it is a
   NOTE; a number presented as gated that is not is a BLOCK (the
   standing "ungated rational presented as gated is a tier violation"
   move). Sample at least: every FH1 scout finding, every FH3 anchor
   result, CI1's "no assertion changed" claim.
2. **Frozen oracle.** For every gate that compares the engine to an
   independent value (`response_success_mass`, `viewer_success_mass`,
   `doom_enumeration`, `horizon_census`, the committed record values
   quoted from the companion's Q6 table): confirm the independent
   value is computed by a DIFFERENT code path than the thing under
   test, and that CI1's fixtures did not turn an independent
   recomputation into a read of the engine's own output. Confirm no
   test regenerates its own oracle. Confirm the committed records
   (`focal_run0.txt`, the ladder and report records) reproduce: rerun
   at least one root of each with the probe and diff.
3. **The correctness-failure list (parent §41, eight items).** For
   each, name the gate that would catch it, or BLOCK if none does:
   (1) a lower that cannot replay as one lawful policy; (2) exact `Q_a`
   outside an interval; (3) a lower falling with `k`; (4) an upper
   rising with `k`; (5) a hidden branch consuming horizon; (6)
   cellwise max before merge; (7) a budget refusal returning a
   truncated number; (8) a suffix receipt reused under mismatched
   identity.
4. **The rulings are obeyed.** FH-A2 vocabulary (grep "sandwich" in
   code, gates, reports, ledger — allowed only when citing the
   parent's title); FH-A3 (no trivial upper installed as a fact —
   read the ladder's install path); FH-A4 (σ0 tail identity includes
   the contract; no cross-contract projection); FH-A6 (`focal_depth`
   and the engine share `decided_success`); FH-A7 (off-DAG = tail);
   FH-A9 (intersection never replacement; every lower fact carries a
   policy; suffix key = full belief); FH-A10 (no live default change,
   no arena claim, no Ω×Θ code); FH-A11 (FH1 had no partial install).
5. **Derived views, never stored state.** In the ladder: is any root
   view stored beside the facts it is derived from? In the engine: is
   the posterior ever stored twice?
6. **Tier discipline.** Every substantive statement in the new wiki/
   ledger text carries EXPLORATORY; no walt number is quoted above
   its tier anywhere on the branch; `walt/MAP.md` presents nothing as
   more than measured.
7. **CI honesty.** Rerun `walt/ci/check.sh` yourself in the foreground
   (about 4 minutes) and report the wall; confirm the runner's binary
   count equals `cargo test --no-run`'s test-target count; confirm a
   failing test fails the gate (scratch, never committed).
8. **The process commits.** CLAUDE.md's new text is accurate (the
   wedge timeline matches the transcript in `FH1-REPORT.md`'s
   deviations); `MAP.md`'s numbers match the records they cite.

## Deliverable

`walt/briefs/FH4-AUDIT.md`: a table of moves × slices with PASS /
BLOCK / NOTE and a one-line citation each, then the BLOCKs in full
(what, where, why it matters, the minimal fix), then the NOTEs. A
BLOCK is a defect the PR must not land with; a NOTE is owed work that
may land as a card. Do not fix anything yourself; the orchestrator
routes fixes to a builder or does them and you re-audit the fix only.
Commit the audit file with a message starting `walt FH4:`; no push, no
PR. Never end a turn with background work pending. Report back with
the table and every BLOCK, under about 700 words.
