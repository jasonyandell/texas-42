# Step-8 probe — V5 flip repair and per-fixed-pair E0 calibration

**EXPLORATORY INSTRUMENT OUTPUT** — sits below every evidentiary tier and
is cited by nothing above it. Every number here is an exact computation
over instrument records or a plain count of them, never a receipt;
nothing in this directory is a P-A21 statement. Forecasts are forecasts;
settlement is governed solely by the exact evidence threshold.

Parent: `walt/math/calculated_evidence_v0.1.md` §22 step 8, gated by §19
V5 (the historical 40/160 flip) and §19 V6 (fixed-pair cost calibration —
the per-fixed-pair E0 calibration; "E0" is the tilt audit's standing ID,
SP-A4, and the parent's correction to it is V6's per-pair discipline).
Rulings CE-A1..A8, L2-A6 (`walt/CENSUS-RULINGS.md`). Producers:
`walt/walt/src/bin/v5flip.rs` and `walt/walt/src/bin/e0cal.rs` at the
commit that adds this README; library authority `solver::calibrate`,
gated by `walt/walt/tests/solver_calibrate.rs`.

## What one V5 record is (`v5.jsonl`)

A flip-shaped root re-run under the §16.4 adaptive controller on ONE
epoch and ONE common indexed stream at the ascending cap ladder
40/160/640 (40 and 160 are the historical coordinates, kept as replay
fixtures per CE-A5 — resource limits here, never settlement rules), with
the §11.3 exact escalation armed at 1:1 declared weights. The V5 law is
asserted mechanically on every ladder (`assert_cap_ladder`): unresolved
may settle later; settled stays settled identically; exact stays exact
identically; NEVER two caps "settled" with different answers.

Two specimen families:

1. **The step-7 shadow run's four exact-route disagreements**
   (`kind:"flip"`) — the decisions in `walt/probes/shadow/*.jsonl` where
   the live 200/8 choice differed from the exact frozen-set answer
   (receipt hands 4, 7, 11; driven hand 14). Each is reconstructed by
   rules replay from its recorded deal and line prefix, and the
   reconstruction is pinned byte-for-byte by asserting the recomputed
   §5.3 evaluation epoch equals the shadow record's epoch hash. The
   exact frozen-set reference is recomputed beside the ladder and must
   reproduce the recorded exact wins.
2. **The count-timing shape family** (`kind:"count-timing"`) — the
   2026-08-23 plunge review's trick-1 near-tie
   (`wiki/walt-seat-play.md`; `walt/LEVEL2-PROBE.md` specimen 2: bid 30
   on sixes, the bidder's 6-6 lead, the partner holding 6-2 and 6-4 and
   nothing else in the suit, so the decision is exactly slough-the-count
   vs hold-the-count-trump). The LITERAL position's game seeds live
   plunge-side (L2-A6 cards them as [[gran-anchor-reconstruction]]), so
   this is a deterministic six-member family with the specimen's shape,
   honestly labeled; the blocked marker is the ignored test
   `v5_literal_count_timing_position_reconstructs`.

Candidates are the step-7 frozen level-1 continuations
(`ActionRule::PinnedThenLevel1`, declared 8/2 schedule, level-0
evaluation field at n0 = 2 — every result is model-relative to those
declared inner approximations, §18 Phase-1 fence). Risk: δ_run = 1/100,
δ_d = δ_run/(d(d+1)) at the fixture's recorded decision ordinal
(count-timing uses d = 1).

## What one E0 record is (`e0.jsonl`)

One unordered pair of one flip fixture's frozen candidate set, compared
PER PAIR (never pooled, V6) across:

- **exact coordinates** from full-fiber enumeration via the kernel:
  pivotal counts `(a, b)`, `q`, `τ`, `g`, `H` — the true parameters of
  the §11.2 sampling law for this pair;
- **initial evidence state**: `E± = 1`, `R_debt = T`, and the exact
  minimum pivots to settle `h±_min(0,0;T) = 12` at the declared
  T = 400 (δ_pair = 1/200, m = 2 edge threshold);
- **forecasts**, all exact rationals: the §7 information-rate and
  leading-order raw-world forecast as interval bounds (series with
  rational tails), and the §8.4 exact forecast DP under the exact
  predictive law `p̃+ = a/n, p̃− = b/n` at γ = 1/2 and 9/10 within the
  declared horizon `dp_h_max`;
- **observed settlement**: three replicate runs of the anytime-valid
  pair evaluator (`evaluate_pair`) on declared instrument streams
  (epochs derived from `E0_SEED` and the pair/replicate identity),
  world cap 1024 — settled index and winner, or honest `Unresolved`
  with empirical `q̂/τ̂`, its information-rate bounds, and the same DP
  forecast refit from the observed counts continuing from `(a, b)` —
  the per-pair grounding a controller cost forecast consumes.

## Files

- `v5.jsonl` — 4 flip records + 6 count-timing records, each with its
  full cap ladder, pair counts, exact reference (flips), and verdict.
- `e0.jsonl` — 18 pair records (3 + 3 + 6 + 6 over the four fixtures).
- `summarize.py` — stdlib-only aggregator reproducing the tables below.

## Reproduction

From `walt/` (units run in parallel; records are byte-deterministic —
no wall-clock fields in these instruments):

```
cargo build --release -p walt --bin v5flip --bin e0cal
./target/release/v5flip probes/step8/v5.jsonl 6
./target/release/e0cal  probes/step8/e0.jsonl 3 1024 192
python3 probes/step8/summarize.py probes/step8/v5.jsonl probes/step8/e0.jsonl
```

Knob meanings: `v5flip <out> [count_timing_n]`;
`e0cal <out> [reps] [world_cap] [dp_h_max]`.

## V5 verdicts (2026-08-24 run; regenerate with summarize.py)

Every ladder satisfied the V5 law; **no cap-dependent flip occurred
anywhere**. Per specimen:

- **receipt h4 d3** (fiber 60, live 1-0 vs exact 2-1): Unresolved at 40,
  **§11.3 escalation fired at stream index 80** → `ExactFrozenSet`
  winner 2-1 at caps 160 and 640, identical — "remain unresolved at 40
  and settle later," with the settle being the exact target itself.
- **receipt h7 d5** (fiber 28, live 6-3 vs exact 6-2): escalated at
  index 16 at every cap → `ExactFrozenSet` winner 6-2, identical
  everywhere. ExactStable.
- **receipt h11 d4** (fiber 1750, live 4-2 vs exact 3-0): honest
  `Unresolved` at 40/160/640 — the e0 table shows why: the
  exact-winner-vs-live pair (3-0 vs 4-2) has τ = 11/175 ≈ 0.06, with a
  leading-order forecast of ~8k–14k worlds even at the per-pair
  T = 400 (the decision ladder's T = 24000 is farther still). The
  exact reference (3-0 by 1448/1750 to 1415/1750) stands beside it.
- **driven h14 d4** (fiber 50, live 3-1 vs exact 2-1): Unresolved at
  40, escalation at index 64 → `ExactFrozenSet` winner 2-1 at 160/640.
- **count-timing g0..g5** (fiber 46,558,512 — no exact route): honest
  `Unresolved` at every cap, all six family members. At cap 640 the
  pair counts sit at q̂ ≈ 0.3 with |τ̂| ≈ 0.01–0.25 — the near-tie the
  specimen described. THE FLIP MODE IS GONE: at both historical
  coordinates (40 and 160) the controller returns `Unresolved`, and
  δ-settling this pair would take the work the forecast machinery now
  quantifies, not a coin-flip between two sample sizes.

## E0 calibration summary (2026-08-24 run)

18 pairs, 54 replicate runs: 45 `DeltaSettled`, 9 honest `Unresolved`
(exactly the three small-|τ| pairs × 3 replicates). Reading, per regime:

- **Strong pairs** (|τ| ≥ ~0.7): DP and leading-order forecasts bracket
  the observed settlements tightly — e.g. driven h14 (0,3):
  q = 9/25, τ = 1, DP γ=1/2 at 33 and γ=9/10 at 44, leading-order
  ~[19,33], observed 30/28/24. driven h14 (1,3): DP 37/58, observed
  32/46/68.
- **Middling pairs** (|τ| ~ 0.4–0.7): observed settlements scatter
  around the forecast scale as an exponential-tailed stopping time
  should — e.g. receipt h11 (2,3): DP 171, observed 76/40/253; receipt
  h4 (0,1): leading-order ~[203,335], observed 224/150/621. The
  forecast is a scale, not a promise; that is its declared type.
- **Near-ties** (|τ| ≤ ~0.11): DP reports no crossing within 192,
  leading-order says 4k–14k worlds, and all replicates are honest
  `Unresolved` at 1024 — forecast and observation agree that these are
  expensive, which is exactly the information the controller's routing
  needs.
- Every settled replicate's winner agreed with the sign of the exact τ
  (45/45) — consistent with the δ-validity the evidence theorem
  guarantees (this count is regression evidence, not the theorem).

## Reading the records honestly

- Everything is model-relative to the declared frozen schedule (8/2
  level-1 continuations over a level-0 field at n0 = 2). Exact means
  exact FOR THAT FROZEN SET under that field, nothing more.
- `Unresolved` is a successful output (§1.5). The count-timing family's
  uniform `Unresolved` is the repair of the recorded episode: where the
  old fixed-n player flipped between 40 and 160 worlds, the controller
  reports the near-tie and prices its resolution.
- The 40/160/640 caps are instrument knobs. Nothing in the correctness
  path reads them; they only truncate one common stream at three
  places, which is what makes the ladder a mechanical demonstration.
- The e0 forecast/observation comparison is 3 replicates per pair —
  orientation-grade calibration of the forecast SCALE, not a
  distributional test. Step 9 consumes these per-pair baselines
  (L2-A6: the field-swap build's Stage-1 evidence layer).
