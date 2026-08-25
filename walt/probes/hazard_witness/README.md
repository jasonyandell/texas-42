# Hazard-witness probe — slice 4b: the one-round producer over the cancel corpus

**EXPLORATORY INSTRUMENT OUTPUT** — sits below every evidentiary tier and
is cited by nothing above it. Every number here is a plain count or exact
rational over instrument records, never a receipt; no strength claim is
made or implied. The load-bearing gates are the tests
(`walt/walt/tests/solver_hazard_witness.rs`); this directory is the
instrument view of the same machinery at the declared pair. **[L2
thread; dominance objective-level]** (PANEL-A7's thread labels).

Mathematical source: Part 2 (§§2.1–2.8) and proof ledger P5 of the x:024
response (`exchange/inbox/024-response-deferred-producers-triple-v0.1.md`),
adopted by rulings **TRIPLE-A4/A5** (`walt/CENSUS-RULINGS.md`, "The
deferred-producers adjudication (2026-08-25)"); intake companion
`walt/math/response_deferred_producers_triple_v0.1_intake.md`; card
`kanban/backlog/slice3-deferred-producers.md` item 2.
Producer: `walt/walt/src/bin/hazard_witness.rs` at the commit that adds
this README, over `solver::hazard` (the Hazard-Exclusion Invariant
verifier — the single dominance-bound authority — the δ = 0
`StructuralHazardZero` type, the benefit exhibit, and the One-Round
Trump-Extraction producer).

**The binding frame (TRIPLE-A4/A5) on every record here:** the producer
emits witnesses for the one general verifier and owns no dominance
semantics; a decline is a refusal to certify, never a hazard claim; a
verified accept is `H(a|b) = 0` for ONE root, ordered pair, and field —
no cross-field composition (§2.8). Mostly declines is the expected,
correct outcome for a deliberately narrow first producer; no hypothesis
is widened to force an accept.

## The declared (σ0, σ1) epoch pair

The slice-3 cancel probe's, unchanged: **σ0** = `Level0 { n0 = 8 }`,
**σ1** = `Level1 { n_outer = 4, n0 = 2 }`, frozen focal candidates at
declared schedule `[8, 2]` (`ActionRule::PinnedThenLevel1`, one per legal
root action). Roots: receipt-h7-t5 (fiber 1680), receipt-h8-t4 (fiber
1200), receipt-h4-t6 (fiber 90). Every ordered candidate pair runs under
each field. (The tests declare the separate cheap pair `Level0{2}` /
`Level1{2,2}`, frozen `[2,2]`, plus void-engineered synthetic specimens.)

## What the records are

1. **`root`** — identities: decl, bid, fiber, root_id, action count, the
   epoch pair, the frozen schedule.
2. **`pair`** — one record per (root, field, ordered candidate pair):
   `accept` (witness hash, exact-fiber H and B cross-checked by exact
   enumeration, the exhibited benefit world's existence, the valid-bound
   dominance kind beside the exact kind) or `decline` (the failed
   hypothesis, named).
3. **`summary`** — pair census, accept/decline counts, the
   failed-hypothesis histogram.

## Reproduction

From `walt/` (records are byte-deterministic except the `micros` fields):

```
cargo build --release -p walt --bin hazard_witness
./target/release/hazard_witness run probes/hazard_witness/records.jsonl
```

Knobs (positional, after the output path): `n0_field0=8 n_outer_field1=4
n0_field1=2 n_outer_frozen=8 n0_frozen=2`.

## Result (2026-08-25, defaults above)

**40 pairs, 0 accepts, 40 declines.** The failed-hypothesis histogram:

| decline | count |
|---|---|
| LeadNotHighestTrump (hyp 1) | 28 |
| HostileTrumpsExceedOneRound (hyp 3) | 6 |
| HostileSuitBeater (hyp 4) | 4 |
| LeadNotVulnerableNontrump (hyp 2) | 2 |

Honestly framed:

- **Zero accepts on this corpus is the pattern working as adopted**, not
  a failure: the one-round shape demands a focal hand that leads the
  globally highest remaining trump against a fiber whose every world
  keeps at most one trump hostile — a narrow endgame silhouette these
  three roots never present. TRIPLE-A5 adopted the producer WITH its
  refusal path; the counts are reported, not engineered.
- **The standing exact `Dominated` specimen declines here.** On h4-t6,
  pin-1-1 over pin-0-0 (exact enumeration: σ0 B = 50/90, σ1 B = 44/90,
  H = 0 — the slice-3 cancel probe's first-in-the-wild dominance) is
  refused by the one-round producer at hypothesis 3: the 90-world fiber
  contains worlds with two-plus hostile trumps, so a one-round extraction
  cannot cover it. Same shape as the response's §2.7 non-coverage
  instance: dominance real, witness language too small — exactly the
  honest boundary H2 predicts.
- The producer's accept path IS exercised — by the gates'
  void-engineered specimens (the §2.6 worked example realized as an exact
  two-world 42 endgame: accept, verify, B = 1/2, H = 0, and the first
  `Dominated` produced through the PANEL-A7 valid-bound route), not by
  this corpus.

## Files

- `records.jsonl` — all records above (44 lines, three roots).
- `run.log` — the producing run's stderr.
