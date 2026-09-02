# BRIEF-U0 — the God-gap census and fusion-horizon instrument

**Authorized:** 2026-09-01, Jason's "full go" (rulings SC-A1..A8,
`walt/CENSUS-RULINGS.md`). **Binding theory:**
`walt/math/salvation_complex_v0.1.md` §§4–9 (salvation sets,
God-tightness, the three-part failure decomposition, what the doom run
says), §§36–40 (God-tight nodes, fusion horizon, cut-oriented
experiment), §48 (the U0 slice spec) — under obligations SC-O3, SC-O4
and the §47 immediate ruling (doom is preserved, never broadened
indiscriminately at the opening). **The intake companion governs where
it narrows the parent** (`walt/math/salvation_complex_v0.1_intake.md`).
Read `walt/FACTOR-BELIEF.md` (the status ledger) before writing code.

## Mission

Build the God-gap census as a proof-state instrument: for affordable
root-action coordinates, establish and persist which of the four §48
result types holds, extract God-tight policies where equality lands,
and produce the first fusion-horizon evidence. In-crate additive: new
module `walt/walt/src/solver/godgap.rs` (or extend `solver/doom.rs` if
that is genuinely cleaner — your call, justify it in the report), a
gate file, a probe binary. No live-player change; nothing about
`solver/refine.rs` (freeze 58) moves.

1. **Result types** (§48, exactly these four, typed):
   `GodUpper` (deterministic: 1 − doomed mass/Z, from per-world doom
   truth on enumerable roots or a certified doom census upper),
   `GodTightPolicy` (an executable policy whose exact lawful value
   EQUALS the God upper — the §36 receipt shape: lower meets doom
   upper at exact identity of root/field/contract/belief),
   `PositiveGodGap` (exact Q strictly below the God upper — requires
   the exact response value), and `UnknownGodGap` (everything else —
   zero certified doom with no exact Q is NEVER `PositiveGodGap`,
   SC-A4).
2. **The census walk:** run every enumerable receipt-root action
   coordinate (the t4/t5/t6 corpus the doom census already covers)
   under σ0 at the declared contracts. Reuse the doom producer's
   per-world truth machinery (`solver/doom.rs`) and the exact
   response/extraction machinery (`solver/extraction.rs`,
   `solver/factor_response.rs`) — compose, never copy.
3. **Record the decomposition** per coordinate: d_phys (doomed
   mass/Z), d_info (God upper − Q, when Q is exact), d_policy for the
   incumbent extracted policy where one exists. Exact rationals only.
4. **Extract and persist God-tight policies** where L = U^God: the
   extracted policy (existing argmax-extraction machinery) re-priced
   unchanged, stored with the §36 equality receipt.
5. **Fusion-horizon table:** stratify by trick/grade; report the
   earliest depth at which every tested coordinate is God-tight, and
   every exception. This is an empirical object — no theorem language
   anywhere in output (SC-A4).
6. **The intake-table gate (SC-A2, binding):** one gate must
   re-derive the salvation parent's §9 fourteen-coordinate table
   mechanically — per-world doom truth and matching lawful values —
   and assert d_info = 0 on all fourteen. The table stops being a
   quoted number and becomes a checked one.

## Gates (`walt/walt/tests/solver_godgap.rs`)

- G1 the §9 table re-derived: fourteen coordinates, doom truth
  matches `doomreport_run1.txt`'s committed values, d_info = 0 on
  each (exact rational equality Q = 1 − doomed/Z).
- G2 result-type soundness: `UnknownGodGap` on a fixture with zero
  certified doom and no exact Q; `PositiveGodGap` only ever carries
  an exact Q witness.
- G3 God-tight receipts: every extracted God-tight policy re-prices
  to exactly the God upper through the independent fixed-policy
  evaluator; the receipt binds root/field/contract/belief identity.
- G4 census refusals are typed and honest: an unaffordable
  coordinate refuses with a reason, never silently drops (match the
  §34/§35 refusal discipline already shipped in `refine`/`opening`).
- G5 doom preservation (§47): the doom producer's existing gates
  still pass untouched; the census consumes doom, never modifies it.

## Probe (`godgapreport`, output to `walt/probes/factor_belief/godgap_run1.txt`)

Per coordinate: fiber mass Z; doomed mass; God upper; exact Q or
UNKNOWN; d_phys/d_info/d_policy; result type; God-tight policy id
where extracted; wall time. Then the fusion-horizon table by trick,
and a plain-language two-regime summary (late corpus vs opening) —
findings only, no promotion.

## Discipline

Same as every slice: `walt/ci/check.sh` green; `ingest/` untouched;
freeze 58 untouched; exploratory tier; no floats; append the status
paragraph to `walt/FACTOR-BELIEF.md`; ambiguity protocol on any spec
conflict. Commit on the current branch with a message starting
`walt U0:`; do not push or open a PR — the orchestrating session
reviews and lands it.

## Report back (your final message)

Slice status; gate count green; the fusion-horizon finding (earliest
God-tight depth, exceptions); how many God-tight policies were
extracted and persisted; the opening-root verdict (expected:
UnknownGodGap — say what would change it); anything you'd flag for U1
(salvation masks) or MB2 (the sep upper).
