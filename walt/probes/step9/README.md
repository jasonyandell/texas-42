# Step-9 probe — the level-2 detection layer (wake-up split under a field swap)

**EXPLORATORY INSTRUMENT OUTPUT** — sits below every evidentiary tier and
is cited by nothing above it. Every number here is an exact computation
over instrument records or a plain count of them, never a receipt;
nothing in this directory is a P-A21 statement. Estimates are never
receipts; open results are successful outputs; refusals are typed
records, not degraded numbers.

Parent: `walt/LEVEL2-PROBE.md` as amended — the CE-A6 wake-up split
(`walt/math/calculated_evidence_v0.1.md` §14) in the L2-A5 role
(`walt/CENSUS-RULINGS.md`): this probe is the **detection layer** of the
one field-swap program. Its paired per-pair output is the Stage-1/2
detection evidence the targeted controller of
`walt/math/targeted_level2_field_stability_v0.1.md` consumes; the
targeting layer (exposure bounds, the stability screen, survivor-only
field-1 optimization) is owned there and is NOT built here. Producers:
`walt/walt/src/bin/wakeup.rs` at the commit that adds this README;
library authority `solver::wakeup`, gated by
`walt/walt/tests/solver_wakeup.rs` and the module's compile_fail locks.

## The three wake-ups (kept distinct, never collapsed)

For one frozen action pair under σ0 and σ1 (CE §14; CE-A6):

- **Response wake-up** — `q₁ − q₀ > ε_q`: newly active response
  structure. NOT by itself a value statement (§14.4: the upgraded field
  may create many disagreements whose signs balance exactly). The record
  type carries no gap and converts to nothing.
- **Value wake-up** — the signed gap change `g₁ − g₀`, settled by the
  §14.6 paired field-correction evidence: on the same world,
  `Z = Y⁽¹⁾ − Y⁽⁰⁾ ∈ {−2..2}` with `E[Z] = g₁ − g₀`, driven through the
  CE bounded-mean engine on `Z/2`.
- **Decision wake-up** — the pair's selected action changes, or an open
  comparison becomes settled (each cross-field case its own typed kind,
  including the honest reverse `newly-open`).

Sampling cost under each field is compared ONLY by the information rate
`𝓘_f = q_f·D_{1/2}(τ_f)` as exact rational interval bounds with a typed
interval verdict — never by `q̂` alone, never by a plug-in `Ĥ` ordering.

**Exact-zero discipline (§14.7):** `q = 0` is pronounced only on the
exact route (complete-fiber enumeration); the sampled route supports
only `q < ε_q` at declared risk, as a `PracticalZero` crossing witness
(private constructor — a sampled zero count cannot inhabit the claim by
convention). The locks are compile_fail doctests in `solver::wakeup`.

## Predeclared corpus (single-look discipline, O14)

Declared from logs before any number was read:

1. **The four step-7 shadow flip fixtures** (`calibrate::FLIP_FIXTURES`:
   receipt h4 d3, h7 d5, h11 d4; driven h14 d4) with their full frozen
   candidate families — the same 18 fixed pairs step 8's E0 calibration
   priced (`walt/probes/step8/e0.jsonl`). Route: EXACT (complete-fiber
   coupled enumeration under both fields).
2. **The count-timing six-member shape family**
   (`calibrate::CountTimingSpec`, g = 0..5 at drive n0 = 8 — step 8's
   exact construction), one pair each ({6-2, 6-4}: slough the count vs
   hold the count-trump). Route: SAMPLED (fiber 46,558,512 — no exact
   route at this budget). The LITERAL plunge position stays blocked on
   [[gran-anchor-reconstruction]] (L2-A6); the ignored test
   `v5_literal_count_timing_position_reconstructs` is the marker; this
   family is the honest stand-in and is labeled as such.

## The declared (σ0, σ1) epoch pair

- **σ0 = Level0 { n0 = 2 }** — the SAME evaluation field as step 8's E0
  calibration and the step-7 shadow exact references, so step 8's
  per-pair baselines are literally this probe's Stage-1 σ0 evidence
  layer (L2-A6). The bin asserts this mechanically at the committed
  defaults: the σ0 leg's per-candidate exact win totals must reproduce
  the recorded shadow exact wins on every flip fixture (all four
  `σ0 exact-wins CHECK PASSED`).
- **σ1 = Level1 { n_outer = 4, n0 = 2 }** — the standing fieldswap
  probe-epoch σ1 (`solver::field`, per-state FIELD_DOMAIN seeds,
  insert-only action cache). The freeze is part of the probe's policy
  identity (SP-A8); both `FieldId`s travel on every record; a changed σ0
  is a NEW experiment and composes with nothing here.
- Frozen candidates: `ActionRule::PinnedThenLevel1` at declared schedule
  [8, 2] (the step-7/step-8 shadow tuple, verbatim). Everything is
  model-relative to those declared inner approximations (§18 Phase-1
  fence): exact means exact FOR THE FROZEN PAIR under the declared
  field, nothing more.

## Declared risks and constants

ε_q = 1/20. δ_decision = 1/200 per field scope (m = 2 edge threshold
400 — the step-8 initial-state minimum of 12 unanimous pivots, gated in
`solver_wakeup.rs`); δ_value = 1/100 split over the two one-sided
paired-Z engines (threshold 200 each); δ_response = 1/100 (threshold
100); δ_practical-zero = 1/100 (threshold 100). Betting mixture
(weight, λ): (1/4, 1/8), (1/4, 1/4), (1/4, 1/2), (1/4, 3/4). Sampled
world cap 256 with dig-until-settled early stop (the cap is a resource
limit, never a settlement rule); minimum honest sampled budget 64;
exact enumeration budget 4096 worlds. Information-rate series depth 24
terms with exact rational tails.

## Files

- `records.jsonl` — 10 root records + 18 exact pair records + 6 sampled
  pair records (byte-deterministic; no wall-clock fields).
- `summarize.py` — stdlib-only aggregator reproducing the tables below.

## Reproduction

From `walt/`:

```
cargo build --release -p walt --bin wakeup
./target/release/wakeup run probes/step9/records.jsonl
python3 probes/step9/summarize.py probes/step9/records.jsonl
```

Knobs: `wakeup run <out> [n0_field0 n_outer_field1 n0_field1
n_outer_frozen n0_frozen world_cap min_worlds exact_budget]`.

## Headline readings (2026-08-25 run; regenerate with summarize.py)

### Corpus and cross-checks

10 roots, 18 exact pairs, 6 sampled pairs, **0 refusals** at the default
budgets. All four flip fixtures passed the σ0 exact-wins consumption
check (the paired walk's σ0 leg reproduces the shadow/step-8 exact win
counts, candidate for candidate).

### Exact route (18 pairs, complete-fiber coupled enumeration)

- **Value wake-up 18/18**: every pair's exact gap changes under the
  field upgrade (`g₁ ≠ g₀`).
- **Decision wake-up 8/18**: the exact frozen-pair selection changes on
  8 pairs — h4's three pairs all collapse a→/b→exact-tie, and the exact
  winner FLIPS outright on h7 (5-1 v 6-2: b→a), h7 (6-2 v 6-3: a→b),
  h11 (1-0 v 6-4: b→a), h11 (3-0 v 4-2: a→b), h14 (3-1 v 5-2: a→b).
- **Response wake-up is the rarest and points the other way**: strict
  `q₁ > q₀` on only 5/18, `> ε_q` on 2/18; on 13/18 the pivotal mass
  DROPS under σ1. The extreme is receipt h4: all three of its pairs
  reach **`q₁ = 0` exactly** — under the level-1 field the frozen
  candidates become outcome-identical over the whole fiber (an
  exact-zero pronounced lawfully by the enumeration route, §14.7), so
  the σ0 disagreement structure there was entirely an artifact of the
  level-0 field.
- **The §14.4 separation shows up in the wild**: h7 (6-2 v 6-3) has
  `dq = 0` (no response wake at all) while τ flips from +3/5 to −1
  (unanimous) and the exact winner flips — value and decision wake with
  zero q movement. The three wake-ups are genuinely different detectors.
- **𝓘 verdicts**: field0-higher 12/18, field1-higher 6/18 — on these
  mid/late-trick receipt roots the level-0 field is more often the
  higher-information-rate (cheaper-evidence) regime.

### Sampled route (count-timing family, cap 256, one common paired stream)

- Response and value: **honestly open on all 6** members — the near-tie
  regime the specimen predicted; nothing here is a value statement.
- **Decision wake-up: 2/6 newly-settled** (g2, g5) — the σ0 leg stays
  open at the full budget while the σ1 leg δ-settles (g2 at world 33,
  g5 at world 84), both on winner **6-4 (hold the count-trump)**. This
  is §14.3's unresolved-becomes-settled wake: on this family the
  upgraded field separates options the modeled field leaves fogged.
- **𝓘 verdicts: field1-higher on 5/6** — the information-rate estimate
  under σ1 exceeds σ0's on five of six members (estimate tier), the
  direction the LEVEL2-PROBE hypothesis expects ("level 2 should make
  decisions easier to sample") on exactly the family built from its
  motivating specimen. The exact route above shows the opposite sign on
  most receipt roots, so this is family-specific, not a law.
- **Mechanism notes**: virtually every world reaches the
  field-disagreement frontier (255–256/256 exposed), with first splits
  concentrated at seat S1 — the modeled BIDDER — in tricks 1–2
  (roughly 60 trick-1 and 160–175 trick-2 first splits per member): the
  disagreement channel is the bidder's early count handling, the
  under-layer the specimen described.
- No practical-zero witness was minted anywhere (`q̂₀ ≈ 0.3` on every
  member — far from ε_q), and none was claimed.

## Reading the records honestly

- Response wake-up alone is NOT a value statement (§14.4); the record
  types keep the three wake-ups mechanically separate, and the sampled
  open states establish nothing in either direction.
- Everything is model-relative to the declared freezes. A different σ1
  schedule, a different frozen schedule, or a different σ0 is a
  different experiment with different FieldIds/PolicyIds.
- The sampled route's per-field δ-settlements are probabilistic
  statements at their declared scopes; the exact route's selections are
  exact for the frozen pair only — never optimized-root statements
  (O18).
- The count-timing rows sample six SHAPE-family stand-ins, not the
  literal plunge hand; nothing here quotes the literal specimen.
- Cost comparisons quote the 𝓘 interval verdicts only. The verdict
  labels on the sampled route are estimates and carry the
  `undefined-*` states honestly.
