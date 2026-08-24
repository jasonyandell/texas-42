# Field-swap probe — §21 step 5: the fixed-policy smoke

**EXPLORATORY INSTRUMENT OUTPUT** — sits below every evidentiary tier and
is cited by nothing above it. Every number here is a plain count or exact
rational over instrument records, never a receipt; nothing in this
directory is a P-A21 statement, and no strength claim is made or implied.
This is detector work only (parent §21 step 5): fixed-policy exposure and
correction, NEVER a root-action bound — a `FrozenPolicyExposure` does not
account for omitted continuations and can never feed screening (L2-A4,
O31).

Parent: `walt/math/targeted_level2_field_stability_v0.1.md` §3, §6.1, §10,
§21 steps 3–5 (adjudicated L2-A1..A7, `walt/CENSUS-RULINGS.md`).
Producer: `walt/walt/src/bin/fieldswap.rs` at the commit that adds this
README, over `solver::field` (FieldId, the two materialized field models)
and `solver::exposure` (coupled pre-split replay, FrozenPolicyExposure).

The Gran anchor seeds are not yet reconstructed
(`kanban/backlog/gran-anchor-reconstruction.md`), so per the slice brief
the smoke runs on roots reconstructable from the step-7 shadow run
instead (`walt/probes/shadow/README.md`): the driven scenario's trick-1
root and two receipt-hand roots found by `fieldswap scan` (bidder leads,
small exact fiber, the ten-count trump 5-5 in hand).

## What one record is

Two field models are materialized through one interface with immutable
content-addressed `FieldId`s (parent §8 Stage 0):

- **σ0** = the banked-correct level-0 modeled mind at declared n0 = 8
  (the field the step-7 shadow bin drives non-focal seats with;
  `Level0Field`, one authority).
- **σ1** = the level-1 machinery (`solver::level1_evaluate`) run per
  non-focal seat at declared inner schedule 4×2, seeded per state from
  (FieldId, seat information state) under the domain-separated
  `FIELD_DOMAIN` — information-consistent by construction: no field
  action can read any hidden hand beyond the acting seat's own (O29).

For each root, two focal policies are frozen
(`ActionRule::PinnedThenLevel1`, declared schedule 8×2, discovery worlds
from the policy's own domain-separated stream): a **reveal**-shaped pin
(lead the countable trump now) and a **retain**-shaped pin (hold it
back). For every world of the declared domain the coupled §3.1 replay
runs both fields from the same root, asserts public-history equality
before the first field split (acceptance item 5), records the first
split (seat, trick/ply, both tiles, the acting seat's hand, the common
record), and forks both branches to terminal. Every world's row asserts
the L2-T1 pointwise bound |u1 − u0| ≤ D (O30); every policy row asserts
|ĉ_ρ| ≤ E[|C_ρ|] ≤ d̂_ρ exactly (§3.2). All arithmetic is exact —
integers and rationals, no floats.

Declared world sets (the domain is part of the result):

- receipt roots: **exact-fiber** — lazy enumeration of the complete
  fiber; d_ρ and c_ρ are exact under the uniform fiber measure for these
  frozen policies under these declared field models.
- the driven trick-1 root (fiber 399,072,960): **stream-prefix** —
  worlds 0..64 of the kernel's exactly-uniform with-replacement indexed
  stream at epoch 0; d̂_ρ and ĉ_ρ are exact over those 64 stream worlds,
  estimates of d_ρ and c_ρ.

## Files

- `fieldswap.jsonl` — root records, per-world coupled rows (with full
  first-split traces), and per-policy `FrozenPolicyExposure` summaries.
- `summarize.py` — stdlib-only aggregator: recomputes every tally from
  the world rows, re-verifies L2-T1 pointwise and the correction bound,
  and prints the counts below.

## Reproduction

From `walt/` (roots run in parallel; records are byte-deterministic
except the `micros` fields, which are wall-clock measurements):

```
cargo build --release -p walt --bin fieldswap
./target/release/fieldswap scan                     # the root chooser's evidence
./target/release/fieldswap run probes/fieldswap/fieldswap.jsonl
python3 probes/fieldswap/summarize.py probes/fieldswap/fieldswap.jsonl
```

Defaults baked into those commands (positional knobs after the fixed
args, in order): `n0_field0=8 n_outer_field1=4 n0_field1=2
n_outer_frozen=8 n0_frozen=2 stream_worlds=64`.

## Aggregate (plain counts; regenerate with summarize.py)

2026-08-24, defaults above; three roots, two frozen policies each:

| root | domain | policy | d | c = +/− | d̂_ρ | ĉ_ρ |
|---|---|---|---|---|---|---|
| receipt-h7-t5 (P5 by T0, S0 leads, fiber 1680) | exact-fiber | reveal-5-5 | 0/1680 | +0/−0 | **0** | **0** |
| receipt-h7-t5 | exact-fiber | retain-1-0 | 0/1680 | +0/−0 | **0** | **0** |
| receipt-h8-t4 (P5 by T1, S1 leads, fiber 1200) | exact-fiber | reveal-5-5 | 1138/1200 | +30/−26 | 569/600 | **+1/300** |
| receipt-h8-t4 | exact-fiber | retain-3-3 | 1117/1200 | +45/−72 | 1117/1200 | **−9/400** |
| driven-h0-t1 (P30 by T1, S1 leads, fiber 399,072,960) | stream 0..64 | reveal-5-5 | 63/64 | +6/−8 | 63/64 | −1/32 |
| driven-h0-t1 | stream 0..64 | retain-3-3 | 64/64 | +2/−9 | 1 | −7/64 |

First-split location (exposed worlds only):

- receipt-h8-t4 reveal-5-5: by trick t4:979 t5:136 t6:23; by seat
  s0:135 **s2:898** s3:105 — after the 5-5 lead, the first field
  disagreement sits overwhelmingly at trick 4 ply 1, seat 2: the seat
  that must respond to the reveal first (an opponent of the S1 bidder;
  see the reading notes on seat labels).
- receipt-h8-t4 retain-3-3: t4:768 t5:306 t6:43; s0:293 s2:485 s3:339 —
  the disagreement spreads later and across all three modeled seats.
- driven-h0-t1: t1-heavy for both pins (42–44 of 64 split in trick 1);
  at trick 1 the richer field wakes up almost everywhere, so the
  fixed-policy exposure is ≈ 1 — an honest, expected degeneracy (parent
  §8.1: bounds near 1 degenerate to the naive survivor set; tightening
  is the E0–E2 rungs' job, not this smoke's).
- receipt-h7-t5: the two fields NEVER split on any of the 1680 fiber
  worlds under either policy — d_ρ = 0 exactly, so c_ρ = 0 exactly:
  the level-1 upgrade cannot move this root's value for these frozen
  policies. This is the parent's central targeting phenomenon observed
  in the wild on the first probe.

Wall time (integer microseconds in the records; instrument-grade
orientation only, roots run in parallel): receipt-h7-t5 ≈ 0.04 s per
policy (never splits, so every coupled replay stays one merged walk);
receipt-h8-t4 ≈ 14 s per policy; driven-h0-t1 ≈ 19 s per policy.

## Reading the records honestly

- Everything is model-relative to declared sampled inner minds (σ0 at
  n0 = 8, σ1 at 4×2, focal continuations at 8×2). A different schedule is
  a different FieldId/PolicyId and a different experiment. Nothing here
  says σ1 is a better mind — level-model typing per O36.
- `d̂_ρ` on the stream-prefix domain is an exact fraction over the 64
  enumerated stream worlds and an ESTIMATE of d_ρ. It is fixed-policy
  exposure (parent §6.1): it says nothing about omitted continuations
  and is not, and can never become, a `RootActionExposureUpper` (L2-A4).
- The exact-fiber rows ARE exact d_ρ/c_ρ for their named frozen
  policies: on receipt-h8-t4 the field upgrade helps the reveal
  continuation (+1/300) and hurts the retain continuation (−9/400) —
  but at these magnitudes, |Λ| = |c_reveal − c_retain| = 31/1200
  (corrected 2026-08-24 from a 41/1200 arithmetic slip, caught by the
  x:019–023 response §32 and re-verified from this file's raw world
  records: (+30−26) − (+45−72) = 4+27 = 31 over 1200) is a
  fixed-PAIR statement (§3.3), and no decision claim is made: that is
  the admissible-set slice's job, on valid root-action bounds only.
- Seat indices in split rows are absolute solver seats (S0..S3), not
  relative positions; the bidder at both split-heavy roots is S1, whose
  partner is S3 and opponents S0/S2. On receipt-h8-t4 reveal-5-5 the
  first split concentrates at **s2 (an opponent)**, trick 4 ply 1 — the
  seat that must respond to the 5-5 lead first. The motif is
  reveal-response-shaped, but which seat's model wakes up is a
  measurement, not the Gran partner hypothesis confirmed; the Gran
  anchors themselves remain gated on seed reconstruction (L2-A6).
