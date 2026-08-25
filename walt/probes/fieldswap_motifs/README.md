# First-split motif probe — slice 4c: the six-motif morphology over correction traces

**EXPLORATORY INSTRUMENT OUTPUT** — sits below every evidentiary tier and
is cited by nothing above it. Every number here is a plain count or exact
rational over instrument records, never a receipt; no strength claim is
made or implied. The load-bearing gates are the tests
(`walt/walt/tests/solver_fieldswap_motifs.rs`); this directory is the
instrument view of the same machinery at the declared pair. **[L2
thread]**.

Mathematical source: Part 3 (§§3.1–3.9) and proof ledger P6 of the x:024
response (`exchange/inbox/024-response-deferred-producers-triple-v0.1.md`),
adopted by rulings **TRIPLE-A6/A7** (`walt/CENSUS-RULINGS.md`, "The
deferred-producers adjudication (2026-08-25)"); intake companion
`walt/math/response_deferred_producers_triple_v0.1_intake.md`; card
`kanban/backlog/slice3-deferred-producers.md` item 3.
Producer: `walt/walt/src/bin/fieldswap_motifs.rs` at the commit that adds
this README, over `solver::motif` (root frames, suffix enrichment,
signatures, classifier, exact decomposition) and `solver::exposure` /
`solver::field_swap` (coupled replay, ladders).

**The binding rule (TRIPLE-A6, §3.1) on every record here:** the traces
exist only for worlds with u0 ≠ u1, so every mass below partitions
**CORRECTION MASS**, never field exposure. The safe phrasing template:

> "Among exact correction worlds for this root, field pair, and frozen
> policy, the first mechanical split had motif k on mass m_k."

Refused aggregates (§3.7, adopted verbatim): no causal claims, no pooled
good/bad-play labels, no exposure-by-motif from correction-only traces,
no dominance from sampled motif hazards, no unweighted pooling across
fibers/bids/fields/policy identities, no invented residual-rate forecast.

## The declared (σ0, σ1) epoch pair

The slice-3 cancel probe's, unchanged: **σ0** = `Level0 { n0 = 8 }`,
**σ1** = `Level1 { n_outer = 4, n0 = 2 }`, frozen focal candidates at
declared schedule `[8, 2]` (`ActionRule::PinnedThenLevel1`, one per legal
root action). Roots: receipt-h7-t5 (fiber 1680), receipt-h8-t4 (fiber
1200), receipt-h4-t6 (fiber 90). A different schedule is a different
FieldId and a different experiment. (The tests declare the separate cheap
pair `Level0{2}` / `Level1{2,2}`, frozen `[2,2]`, on the parity roots.)

## What one root's records are

1. **`root`** — identities: decl, bid, viewer, fiber, root_id, the
   **root_semantics_hash** (FNV-1a 64 over the immutable RootFrame:
   rule version, decl, bid, partnership/focal frame, root trick frame,
   prior mask), FieldIds, frozen schedule, specimen cap.
2. **`trace`** (specimens, capped at `specimen_cap` per action; full set
   recomputed by rerunning the binary) — RAW ONLY per TRIPLE-A7: world,
   the first split (seat, trick, ply, tile0, tile1, hand, history), both
   terminals, and the enrichment — `branch0_suffix`, `branch1_suffix`
   (the post-split plays as (actor, tile) pairs, strictly after the split
   play) and `root_semantics_hash`. **No motif tag is persisted on any
   trace record**; the suffixes close the item-11 gap (the distinguishing
   public observation is now explicit and replayable). Records in
   `../fieldswap_cancel/` predate this enrichment and lack these fields.
3. **`motif_histogram`** — per (root, action): the exact decomposition
   m_k⁺/m_k⁻ per motif with r_k, c_k, and conditional tilt τ_k = c_k/r_k
   (identities Σm_k⁺ = c⁺, Σm_k⁻ = c⁻, Σc_k = c asserted in the binary
   AND re-checked by summarize.py), the six coordinate-difference flag
   counts, split-actor relation counts (partner|opponent), terminal-sign
   counts, and the §3.6 residual fraction.
4. **`residual`** — the raw signature pairs inside `Other`, with counts
   (empty on this corpus; the instrument stays in place).

## Files

- `motifs.jsonl` — all records above (59 lines, three roots).
- `summarize.py` — stdlib-only checker: recomputes every identity from
  the raw counts with exact fractions; `ALL 569 CHECKS PASS` on the
  committed records, and prints the aggregate table.
- `run.log` — the producing run's stderr.

## Reproduction

From `walt/` (roots run in parallel with the `parallel` feature; records
are byte-deterministic except the `micros` fields):

```
cargo build --release -p walt --bin fieldswap_motifs --features parallel
./target/release/fieldswap_motifs run probes/fieldswap_motifs/motifs.jsonl
python3 -B probes/fieldswap_motifs/summarize.py probes/fieldswap_motifs/motifs.jsonl
```

Knobs (positional, after the output path): `n0_field0=8 n_outer_field1=4
n0_field1=2 n_outer_frozen=8 n0_frozen=2 specimen_cap=8`.

## Aggregate (2026-08-25, defaults above; regenerate with summarize.py)

Correction-mass partition per (root, action) — counts are worlds out of
the fiber; the signed split is (m_k⁺·N / m_k⁻·N):

| root (fiber) | action | corr | LeadContext | ImmControl | CountCommit | TrumpCommit | SuitShape | Strength | Other |
|---|---|---|---|---|---|---|---|---|---|
| h7-t5 (1680) | 1-0, 5-3, 5-5 | 0 | — | — | — | — | — | — | — |
| h8-t4 (1200) | 2-1 | 120 | 11/12 | 8/12 | 10/40 | 0 | 12/15 | 0 | 0 |
| | 3-1 | 154 | 20/8 | 9/17 | 25/47 | 4/6 | 4/14 | 0 | 0 |
| | 3-3 | 117 | 3/2 | 5/12 | 10/16 | 5/10 | 22/32 | 0 | 0 |
| | 5-5 | 56 | 0 | 1/1 | 5/4 | 0 | 24/21 | 0 | 0 |
| h4-t6 (90) | 0-0 | 6 | 0 | 4/0 | 0 | 2/0 | 0 | 0 | 0 |
| | 1-1 | 0 | — | — | — | — | — | — | — |

Residual fraction: **0 overall and per root** (453/453 correction worlds
classified into the six motifs; `Other` reachable in the gates via both
routes, including `missing_root_frame`). The correction totals re-derive
the committed cancel-probe ladders exactly (e.g. h8-t4 2-1: 120 = r·N =
(1/10)·1200; h4-t6 0-0: 6 = (1/15)·90).

Highlights, honestly framed:

- **Priority capture is visible exactly as the intake predicted.** On
  h8-t4 2-1, `diff_suit_shape` is set on **120/120** correction traces
  and `diff_strength` on 119/120, yet `SuitShapeFork` is primary on only
  27 — the earlier coordinates captured the rest. The ordering is a
  taxonomy convention, not a causal ranking; the flags carry the
  co-occurrence.
- **The morphology differs by action within one root.** 2-1's
  corrections are count-led (50/120 CountCommitmentFork, tilt −3/5:
  count-release splits run against the focal action under this pair),
  while 5-5's are shape-led (45/56 SuitShapeFork, tilt +1/15 ≈ a dead
  heat). Same root, same field pair, same fiber — different first-split
  morphology per pinned candidate.
- **h4-t6 0-0 is one-sided:** all 6 corrections favor field 1 (c⁻ = 0),
  4 ImmediateControlFork + 2 TrumpCommitmentFork, all with
  `diff_trump` set — the σ1 mind's trump handling at the split is the
  entire correction event here, read as morphology, not cause.
- **StrengthCommitmentFork did not occur in the wild** on this corpus
  (it is gated reachable synthetically). Two distinct tiles almost
  always differ in an earlier coordinate first; a pure strength fork
  needs identical context/control/count/trump/shape — rare in real play.
- **The residual instrument read zero** — on THIS corpus the six-axis
  local alphabet explains every first split. That is a statement about
  these three roots at this pair, not a forecast (§3.6: no residual-rate
  number is invented).
- **Split-actor relation skews opponent** on h8-t4 (e.g. 103/120 on
  2-1) but not uniformly (3-3: 45 partner / 72 opponent). Descriptive
  reading only.

Cost (integer microseconds in the records; instrument-grade): h8-t4
7.0–9.2 s per action (exposure + enrichment replay dominate); h7-t5
22–48 ms per action (no corrections to enrich); h4-t6 under 46 ms.

## What this probe deliberately does NOT ship

- **No `RevealResponse` and no response/causal label** (TRIPLE-A7
  REFUSED). The suffix enrichment is the prerequisite for a possible
  later `PartnerResponseCandidate` predicate — a separate construction,
  not requested.
- **No exposure-by-motif.** Non-pivotal exposed worlds (u0 = u1 with a
  split) are not classified; until they are, motif masses partition
  corrections only.
- **No cross-root or cross-action pooling.** Every histogram names its
  (root, action, policy, field pair); the table above juxtaposes rows,
  it does not sum them.

## Reading the records honestly

- Everything is model-relative to the declared sampled inner minds and
  the declared `[8, 2]` frozen candidates; nothing here says σ1 is a
  better mind, and nothing here says a motif caused a correction.
- A motif is first-split MORPHOLOGY: what kind of mechanical fork the
  two field choices made at the first disagreement, read off the local
  signature. The terminal correction is recorded beside it, never
  attributed to it.
- Three roots, one declared pair: orientation for the §3.6 residual
  watch, not a settlement.
