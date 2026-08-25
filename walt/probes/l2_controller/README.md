# Targeted field-1 controller probe — §8 Stages 1–5 assembled, schedule-controlled

**EXPLORATORY INSTRUMENT OUTPUT** — sits below every evidentiary tier and
is cited by nothing above it. Every number here is an exact computation
over instrument records or a plain count of them, never a receipt.
Estimates are never receipts; open results are successful outputs;
refusals are typed records, not degraded numbers. No play-strength claim
is made or implied, and nothing here changes any default (CE-A7/§20.16:
the controller is an instrument + library layer beside the live player,
never inside it).

Parent: `walt/math/targeted_level2_field_stability_v0.1.md` §8 (the
targeted level-2 controller), §12.1–12.2 (cost decomposition, routing by
stability debt), under rulings L2-A1..A7, PANEL-A7/A8, TRIPLE-A2/A3
(`walt/CENSUS-RULINGS.md`) and obligations O31/O32/O34/O38
(`walt/SCENARIO-PLAYER.md` §10). Producer:
`walt/walt/src/bin/l2_controller.rs` at the commit that adds this README;
library authority `solver::targeted` (which assembles `solver::exposure`,
`solver::field_swap`, `solver::upper_cs`, and the CE evidence engine —
one authority per concept, nothing reimplemented), gated by
`walt/walt/tests/solver_targeted.rs` and the module's compile_fail locks.

## What the controller adds over the slice-2/3 screen probes

The rung spend is itself schedule-controlled:

- **Cheapest rung first** (E1 covers → the shared E0/E2 reach walk →
  exact E4 → directional), every bound typed with its rung, every
  phase's spend recorded in integer microseconds.
- **The steering lemma.** The admissible set is monotone nondecreasing
  in each exposure bound, so for exact values `R*` with
  `ℓ ≤ R* ≤ R^cur` pointwise, `𝓐₁(ℓ) ⊆ 𝓐₁(R*) ⊆ 𝓐₁(R^cur)`. The frozen
  candidate's exact fixed-policy exposure `d_ρ_a` is a lawful LOWER
  witness to `R_a`; when the admissible set at the lower witnesses
  equals the set at the current bounds, escalating to exact E4 provably
  cannot prune, and the spend is refused as `provably-useless`. Lower
  witnesses steer spend only — the `SteeringLower` type has no screen
  accessor, and the screen still consumes nothing but
  `RootActionExposureUpper` (L2-A4/O31). The refusal is verified against
  real E4 in the gate tests.
- **Honest degradation over the exact cap**: δ-valid Stage-1 intervals
  (`DeltaFrozenSet` tier, the CE one-mean inversion at both endpoints),
  the degenerate E1 bound stated (mass exactly 1 by definition of the
  trivial cover — never counted at 46M worlds), the sampled δ-valid E3
  route only where the zero-hypothetical shows a prune is possible, and
  typed refusals everywhere else.

## The declared (σ0, σ1) epoch pair

The fieldswap-screen/cancel probe epoch's pair, unchanged:

- **σ0** = `Level0 { n0 = 8 }`;
- **σ1** = `Level1 { n_outer = 4, n0 = 2 }` (per-state `FIELD_DOMAIN`
  seeds, insert-only action cache);
- frozen focal candidates at declared schedule `[8, 2]`,
  `ActionRule::PinnedThenLevel1`, ONE freeze construction for the whole
  probe epoch (the level1-continuation library shape), one candidate per
  legal root action, frozen before any cross-field evidence.

Both `FieldId`s ride every record. A different schedule is a different
`FieldId` and a different experiment epoch; in particular the step-9
probe's σ0 was `Level0 { n0 = 2 }`, so its per-pair results compose with
nothing here.

## Predeclared corpus (single-look discipline, O14)

Declared in the producer's header before any number was read — existing
roots only, nothing new:

1. **The three slice-2/3 screen roots**: receipt-h7-t5 (fiber 1680),
   receipt-h8-t4 (fiber 1200), receipt-h4-t6 (fiber 90). Route: EXACT.
2. **The four step-9 flip-fixture roots**
   (`calibrate::FLIP_FIXTURES`: receipt h4 d3, h7 d5, h11 d4; driven
   h14 d4), reconstructed decisions. Route: EXACT.
3. **The six count-timing shape-family members**
   (`calibrate::CountTimingSpec`, g = 0..5 at drive n0 = 8, fiber
   46,558,512 each). Route: SAMPLED — the honest degradation path. The
   LITERAL plunge position stays blocked on
   [[gran-anchor-reconstruction]] (L2-A6); this family is the honest
   stand-in and is labeled as such.

## Declared budget and risks

Exact-fiber cap 4096 (a resource limit, never a settlement rule);
directional phase enabled. Sampled roots: baseline prefix 128 worlds,
E3 prefix 24 worlds; screen budget δ = 1/50 per root; per-action
one-sided baseline endpoints 1/800 each; per-action symmetric E3 1/400.
Every entry is a `ScopedDelta`; the exact-rational ledger sum is
asserted against the budget (`assert_screen_risk_allocation`,
TRIPLE-A2 §1.8) and recorded per root. The `EpsilonEquivalent` ladder
label runs at declared ε = 1/20.

## Files

- `records.jsonl` — per root: root/row/screen/directional/spend/stage4/
  refusal/risk records (byte-deterministic except the `micros` fields).
- `summarize.py` — stdlib-only aggregator: recomputes every screen from
  the row records with exact fractions, re-verifies bar, admissible set,
  and the full slack table, checks stage-4 survivor confinement and the
  decision flag, checks the risk ledger, prints the tables below.

## Reproduction

From `walt/`:

```
cargo build --release -p walt --bin l2_controller
./target/release/l2_controller run probes/l2_controller/records.jsonl
python3 probes/l2_controller/summarize.py probes/l2_controller/records.jsonl
```

Knobs: `l2_controller run <out> [n0_field0 n_outer_field1 n0_field1
n_outer_frozen n0_frozen exact_cap baseline_prefix e3_prefix
ct_members]`.

## Headline readings (2026-08-25 run; regenerate with summarize.py)

| root | tier | result | stop | 𝓐₁/legal | rungs paid | stage 4 |
|---|---|---|---|---|---|---|
| receipt-h7-t5 | exact-frozen-set | FieldSensitive | provably-useless | 3/3 | E0 | 1-0 → 1-0 (exact ties at 0) |
| receipt-h8-t4 | exact-frozen-set | **FieldDecisionChanged** | provably-useless | 4/4 | E1/E2 | **2-1 → 5-5** |
| receipt-h4-t6 | exact-frozen-set | FieldStableExactFrozenSet | **pruned** | 1/2 | E2 | singleton 1-1, no σ1 work |
| flip-h4-d3 | exact-frozen-set | FieldSensitive | provably-useless | 3/3 | E2 | 1-0 → 1-0 |
| flip-h7-d5 | exact-frozen-set | FieldSensitive | provably-useless | 3/3 | E2 | 6-3 → 6-3 |
| flip-h11-d4 | exact-frozen-set | FieldSensitive | provably-useless | 4/4 | E2 | 4-2 → 4-2 |
| flip-h14-d4 | exact-frozen-set | FieldSensitive | provably-useless | 4/4 | E1/E2 | 2-1 → 2-1 |
| count-timing g0–g5 | delta-frozen-set | FieldSensitive (6/6) | provably-useless | 2/2 | E1 | honestly **open** (6/6) |

- **Rung E4 was never paid on this corpus.** One root pruned before
  reaching it (h4-t6: the E2 screen alone excludes 0-0, singleton 1-1,
  zero σ1 spend — the slice-2 result reproduced by the assembled
  pipeline at a fraction of the rung cost); on the other twelve the
  steering lemma refused E4 as provably useless. This is the §17.2
  falsifier direction answered by schedule: where the screen was going
  to prune nothing, the controller now proves that cheaply and skips
  the exact solves instead of paying for them (the slice-2 probe paid
  10.9s of rungs at h8-t4 to prune nothing; the lemma's guarantee is
  that the E4 bounds could not have changed the admissible set).
- **What σ1 purchased at h8-t4: an actual decision change.** The
  σ0-settled 2-1 (1103/1200) drops to 1065/1200 under σ1 while 5-5
  rises to 1098/1200 — the frozen-set selection flips to 5-5
  (`FieldDecisionChanged`; exact route, both baselines complete-fiber).
  The survivor ladders put all four candidates at `EpsilonEquivalent`
  (ε = 1/20) with corrections c between −19/600 and +1/300 riding
  exposure d between 1087/1200 and 569/600 — massive cancellation
  (PANEL-A7's |c| ≤ r ≤ d in the wild), and the interpretation rule
  applies: a value statement under this objective/belief/model pair,
  nothing more.
- **The count-timing family degrades exactly as designed**: δ-tier
  baselines at 128 worlds overlap (near-tie family), the
  zero-hypothetical shows no exposure bound could prune under those
  intervals, so E3 is refused as provably useless (24-world walks
  never paid), survivor σ1 intervals stay honestly open 6/6, and the
  risk ledger reads 1/100 spent of the 1/50 budget per member. Nothing
  here is a value statement about 6-2 versus 6-4.
- **The directional phase is honest about itself**: skipped as provably
  useless on 5 exact roots (the frozen c± lower witnesses already admit
  everything), ran once (flip-h7-d5, 8ms) and pruned nothing extra —
  recorded, not hidden. h7-t5's E0 firing reproduces the slice-2
  result: R = 0 for all three actions at the root-action level, and the
  survivor ladders read `NoFieldExposure` (d = 0 exactly).
- **Spend accounting note**: micros are wall-clock under a contended
  13-root parallel run — cross-root comparisons are indicative only.
  Phase attribution note: the steering walks warm the σ1 field's
  insert-only action cache, so `stage4-sigma1` on exact roots reads
  nearly free (h8-t4: 14ms) — the σ1 cost was largely prepaid inside
  `steering`/`rung-e2`. The honest total is the per-root sum, not any
  single phase.

## Reading the records honestly

- Every claim is model-relative to the declared freezes and tier-capped
  by its Stage-1 baseline: `FieldDecisionChanged` at h8-t4 is a
  frozen-set statement under this epoch pair, never an optimized-root
  or play-strength statement (O18).
- `provably-useless` is a statement about THIS corpus's geometry (the
  screen could not have pruned at any lawful tightening), not a law
  about E4's value elsewhere; the gate tests verify the refusal against
  real E4 bounds wherever it fires.
- The sampled route's open results are successful outputs at the
  declared budgets; a bigger prefix is a bigger experiment, not a
  correction.
- The step-9 detection layer's per-pair wake records (different σ0
  epoch) compose with nothing here; both probes cite the same parent
  and rulings but are separate experiments.
