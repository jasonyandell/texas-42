# BRIEF-SIGMA1-REPAIR — terminate the sampler, then make him better than he was

**Authorized:** 2026-09-01, Jason's explicit grant ("breaking the live player
temporarily is entirely approved"), deferred out of MB0 by orchestrator ruling
so the evidence window stayed open. **Binding context:** the "the sigma1
boundary" section of `walt/probes/factor_belief/modelbelief_run1.txt`, the
MB0 paragraph in `walt/FACTOR-BELIEF.md`, and the two scratch-tier notes files
`walt/briefs/MB0-COLLISION-NOTES.md` (hazard diagnosis, specimen, dedup
inventory) and `walt/briefs/MB0-HANDOFF-BUILDER2.md`. Read all four before
writing code.

## The hazard (verified at source by two independent auditors)

`walt/walt/src/solver/mod.rs` `sample_belief` (~line 897–931) is an unbounded
shuffle-and-reject loop: `while out.len() < n { shuffle; reject if
w[s] & voids[s] != 0 }` — no attempt cap, no feasibility precheck. An
infeasible deduced-void frame spins forever. It is PRE-EXISTING in shipped
code; MB0 was merely the first caller to reach it (MB0's G2/G8 gates pin the
reachability and the live specimen in CI permanently — those gates must stay
green, untouched). Mechanism note from the MB0 audit: the acceptance region is
empty exactly at zero-joint-mass states — hands record-consistent for their
own seat but jointly uncompletable — which the raw route's UNTIGHTENED support
classification contains.

**Five byte-identical copies** (normalized-body-hash verified equal, library
copy differs by `pub` only):
`solver/mod.rs:897` (library), `bin/walt_bridge.rs:534`, `bin/playout.rs:571`,
`bin/playtable.rs:565`, `bin/divergence.rs:635`. Their dependencies are also
byte-identical across all five: `impl SplitMix64` (next_u64 + below),
`fn mask_bits`, `const FULL_MASK = 0x0FFF_FFFF`.

## Mission, strictly ordered

1. **BEFORE-side determinism capture, FIRST, while the sampler is unpatched.**
   Record F₁ field actions (and/or sampled-deal transcripts) over a feasible
   corpus — enough roots/seeds to be a real witness (the field action cache is
   deterministic per (key, spec), so equality is a clean check). Persist the
   capture as a committed fixture. This is the mechanical witness that the
   repair preserves feasible-frame behavior; the gpu-ref receipt byte-diffs do
   NOT witness this (different crate, never touches the sampler).
2. **Repair the library copy.** Feasibility precheck (exact counting — e.g.
   the guarded exact-partition search MB0's boundary work proved faithful) or
   a bounded-attempt cap, with a TYPED outcome on the infeasible path (a
   refusal/error type, never a silent fallback, never a panic in live-player
   paths — walt_bridge must handle the outcome honestly). The draw sequence on
   FEASIBLE frames must be unchanged — same RNG stream, same accept path.
   No floats anywhere.
3. **Deduplicate: point all four bin copies at the repaired library function
   and delete their local copies.** Expect the TYPE-IDENTITY CASCADE: each
   bin's local `SplitMix64` is a distinct Rust type, so redirecting forces the
   bin to delete its local `SplitMix64`/`mask_bits`/`FULL_MASK` and import the
   library's — which cascades to unrelated uses in the same binary (e.g. seed
   paths at walt_bridge.rs:353, playout.rs:361). The cascade is
   draw-preserving (impls hash identical) and is NOT a reason to bail: take
   it. Fallback (only on a real obstacle, justified in the report): identical
   fix applied to all five copies, duplication recorded as named debt.
4. **AFTER-side capture and equality gate:** re-run the item-1 corpus through
   the repaired path and gate exact equality against the committed fixture.
5. **Do NOT touch `level1_evaluate`** (triplicated at solver/mod.rs:1039,
   walt_bridge.rs:575, playtable.rs:665 — named debt, out of scope). Do not
   touch refine.rs (freeze 58), ingest/, or MB0's module/gates beyond keeping
   them green.

## Gates (new file, `walt/walt/tests/solver_sigma1_repair.rs`)

- R1 the infeasible specimen terminates with the typed outcome: seat S3, hand
  {4-2 4-4}, history [4-1 4-3 1-1], sizes [1,1,1,2], voids
  [16786368, 69173248, 33586176, 16786368].
- R2 before/after determinism: the committed feasible-corpus fixture
  reproduces exactly through the repaired library path.
- R3 dedup identity: the four bins compile against the library function; no
  local `fn sample_belief` remains anywhere (gate by grep or by compile —
  state which).
- R4 the previously-blocked MB0 roots: δ-F₁ on {h5-t6, h4-t6, h8-t5, h3-t5}
  now either terminates with a value or refuses TYPED — no hang. (If values
  now exist, record them; MB0's scoped parity domain just grew, which is a
  reportable finding.)
- R5 MB0's G2/G8 and the whole existing workspace suite stay green untouched.

## Report wording (verdict-enforced by the auditor)

Enumerate covered call sites exactly: covered-for-free (already library):
ordering_bench, webtable, controller_bridge, shadow, waking_bridge, tiltaudit;
covered-by-dedup: walt_bridge, playout, playtable, divergence. State plainly
that `level1_evaluate` remains triplicated (named debt). Never write "the σ1
hazard is repaired" without that enumeration.

## Discipline

`walt/ci/check.sh` green before done (cold rebuild + full workspace release
suite — budget it). Checkpoint-commit FREQUENTLY (`walt sigma1-repair: WIP —`
after each ordered item; drops cost minutes when you commit, runs when you
don't). Final commit message starts `walt sigma1-repair:`. Do not push, no
PR — the orchestrating session reviews and lands. Everything EXPLORATORY
tier. Ambiguity protocol on any conflict. FACTOR-BELIEF.md status paragraph
in house style.

## Report back (final message)

Slice status; gate count; the R4 finding (did the blocked roots open?); the
covered-call-sites enumeration; wall-time of the determinism corpus; anything
you'd flag for MB1 or the level1_evaluate debt.
