# Waking-seat natural-play profile

**EXPLORATORY TIER.** This directory sits below every evidentiary tier
and is cited by nothing above it. Probe numbers here become quotable
results only by brief amendment adding them to a verifier receipt.
Estimates, never receipts; not a P-A21 statement.

## What this is

The **unordered-baseline profile** of the waking seat
(`walt/src/solver/waking.rs`, driven by `walt/src/bin/waking_bridge.rs`
in `driven` mode): whole fresh-deal hands played start to finish with
the waking seat making every play decision at all four seats, one typed
census record per decision, every phase's spend in integer
microseconds.

**The affordability question this probe was originally chartered for is
already answered by the numbers below: NO, not as-is** — a natural hand
costs minutes of decision compute at the live epoch. What the profile
is FOR is attribution: which phase the microseconds actually go to
(baseline vs wake check vs escalation, and inside the escalation the
controller's own `PhaseSpend` pipeline vector, carried verbatim on
every wake record). That attribution picks the target for the next
build (reorder-not-cull ordering work).

Threads (CE = sampling depth, L2 = model choice): the baseline is CE,
the escalation is L2; per-record phases carry the split.

## Declared configuration

- **Epoch pair (live):** σ0 = `Level0 { n0 = 2 }` — the same field
  act's evaluation runs against; σ1 = `Level1 { n_outer = 4, n0 = 2 }`;
  frozen focal candidates at declared schedule [8, 2]
  (`ActionRule::PinnedThenLevel1`). **The l2_controller probe's epoch
  is DIFFERENT (σ0 n0 = 8): numbers do not compose across the two.**
- **Baseline:** `solver::act` at `ActConfig::interactive` (world cap
  128, exact cap 2000, fallback 200×8), δ_run = 1/100 per hand.
- **Wake check:** budget 24 paired worlds (sampled route), exact route
  at fiber ≤ 1024; wake rule = the σ1 leg positively selects the rival
  (module docs). Waking risk budget δ = 1/20 per hand, split
  per-decision by the telescoping `decision_delta` convention under
  `wake:`-prefixed scopes.
- **Retune, declared:** the exact wake cap was retuned 64 → 1024 after
  the first smoke hand priced the routes. Under the telescoping risk
  the sampled probe's δ-settlement threshold needs a net pivotal margin
  of roughly 10–17 worlds out of 24 — effectively uncrossable, so its
  honest outcome is almost always `no-wake-budget-exhausted` (recorded,
  never forced). The exact route settles always and cost fractions of a
  second up to fiber ~1000 in the probe, so the wake gate's real
  coverage is the exact route; the 24-world budget stays as the
  declared cost bound above the cap.
- **Escalation:** `solver::targeted::targeted_root`, exact fiber cap
  4096, baseline prefix 128, E3 prefix 24, directional off; ε = 1/20.
- **Seeds:** deals from `WAKING_DRIVEN_SEED = 0x51EE_D42A_11FE_600D`
  mixed per hand index; auction beliefs from
  `WAKING_DECLARE_SEED = 0x7A3E_9B21_5C48_D6F1`. No wall clock, no
  ambient entropy — the played line is a pure function of the hand
  count (timings vary run to run; counts and choices do not).
- **Hands:** 2 driven hands (the scaled run was deliberately skipped —
  the affordability answer was already legible at 2 hands, and a bigger
  sample would measure a moot outcome). Bidder rotates by hand index,
  trump named by the existing level-1 auction policy (7 pip-trump
  candidates), bid fixed at 30.

## How to reproduce

```
cargo build --release -p walt --bin waking_bridge
./target/release/waking_bridge driven driven.jsonl 2
python3 summarize.py driven.jsonl
```

## The profile (driven.jsonl, 2 hands, 56 decisions)

### The phase conviction

**The σ0 baseline — act itself — is where the microseconds go: 729
permille of all decision compute** (207.0s of 283.9s across 56
decisions). The wake check is 269 permille (76.5s, dominated by the
sampled checks on trick-1/2 fibers), and the escalation is 1 permille
(0.45s — one wake, one settled selection). By trick: tricks 1–2 alone
carry 262.9s of the 283.9s total (926 permille of the whole profile),
baseline-dominant in both; from trick 4 on the wake check dominates
what little remains. Inside the one escalation that ran, the
controller's own PhaseSpend vector says the rungs are the cost: rung-e2
(the shared E0/E2 reach walk) 529 permille of escalation spend,
steering (exact lower witnesses) 328, the σ0 exact baseline 139, and
survivor σ1 work 2 — the screen machinery, not the σ1 values, is the
escalation's expense at this scale.

So the target for ordering work is act's own evaluation on big early
fibers — not the wake check, and not the escalation.

Other headline reads: wake rate 1/34 checked decisions (the one wake at
trick 5, exact route, escalated to `exact-survivors /
provably-useless / exact-argmax` and MOVED the play off σ0 — agreement
55/56); the exact wake route settled every check it reached (13 of 13:
10 ties, 2 baseline-confirms, 1 rival), while all 21 sampled checks
above fiber 1024 stayed honestly open, as the retune note predicted.

### Full summarize output (verbatim, also in `summary.txt`)

```
census records (decisions): 56
hand summaries: 2
  bids made: 1/2 (1 of 2)
PHASE ATTRIBUTION (integer micros; share in exact permille, floor):
  baseline (sigma0 act):  206971014 (729 permille)
  wake check:             76474588 (269 permille)
  escalation:             454039 (1 permille)
  grand total:            283899641
  by trick (baseline / wake / escalation micros, dominant phase):
    trick 1: 153339519 / 49195088 / 0  (trick total 202534607; dominant: baseline, 757 permille)
    trick 2: 42870340 / 17502535 / 0  (trick total 60372875; dominant: baseline, 710 permille)
    trick 3: 8233034 / 5558121 / 0  (trick total 13791155; dominant: baseline, 596 permille)
    trick 4: 2398000 / 2594696 / 0  (trick total 4992696; dominant: wake-check, 519 permille)
    trick 5: 128192 / 1576947 / 454039  (trick total 2159178; dominant: wake-check, 730 permille)
    trick 6: 1929 / 47201 / 0  (trick total 49130; dominant: wake-check, 960 permille)
    trick 7: 0 / 0 / 0  (trick total 0; dominant: baseline, 0 permille)
  escalation PhaseSpend breakdown (controller pipeline phases):
    rung-e2: 240300 micros (529 permille of escalation), items=3, records=1
    steering: 149329 micros (328 permille of escalation), items=3, records=1
    baseline-sigma0: 63223 micros (139 permille of escalation), items=3, records=1
    stage4-sigma1: 1130 micros (2 permille of escalation), items=3, records=1
    rung-e1: 12 micros (0 permille of escalation), items=1, records=1
    stage4-ladders: 8 micros (0 permille of escalation), items=3, records=1
paths:
  forced: 11/28 (22 of 56)
  no-wake-budget-exhausted: 3/8 (21 of 56)
  no-wake-settled: 3/14 (12 of 56)
  wake: 1/56 (1 of 56)
forced fraction: 11/28 (22 of 56)
wake-check rate (non-forced): 17/28 (34 of 56)
wake rate over checked decisions: 1/34 (1 of 34)
wake rate over all decisions: 1/56 (1 of 56)
by trick (decisions / checked / wakes):
  trick 1: 8 / 6 / 0  (wake rate 0/1 (0 of 6))
  trick 2: 8 / 6 / 0  (wake rate 0/1 (0 of 6))
  trick 3: 8 / 6 / 0  (wake rate 0/1 (0 of 6))
  trick 4: 8 / 6 / 0  (wake rate 0/1 (0 of 6))
  trick 5: 8 / 6 / 1  (wake rate 1/6 (1 of 6))
  trick 6: 8 / 4 / 0  (wake rate 0/1 (0 of 4))
  trick 7: 8 / 0 / 0  (wake rate 0/0)
wake-check evidence kinds:
  exact-sigma1-selects-baseline: 2
  exact-sigma1-selects-rival: 1
  exact-sigma1-tie: 10
  sampled-open: 21
escalation outcomes (StageFourOutcome / EscalationStop / route):
  exact-survivors / provably-useless / exact-argmax: 1
agreement with sigma0: 55/56 (55 of 56)
  moved off sigma0: 1 decision(s), at tricks [5]
spend (integer microseconds):
  baseline_us (all): n=56 min=0 p50=1236 p90=14648451 p99=58463801 max=58463801 total=206971014
  wake_us (checked): n=34 min=36 p50=758838 p90=7543875 p99=10366110 max=10366110 total=76474588
  escalation_us (wakes): n=1 min=454039 p50=454039 p90=454039 p99=454039 max=454039 total=454039
  decision total (all): n=56 min=0 p50=14337 p90=21665288 p99=68829911 max=68829911 total=283899641
  wake_worlds (checked): n=34 min=4 p50=24 p90=200 p99=780 max=780 total=2238
fibers: min=1 p50=780 p90=8588580 max=399072960
```

## Caveats (honest, all of them)

- Two hands. Counts are exact for these hands; rates (wake density,
  agreement) are small-sample reads, not estimates with stated risk.
- Single machine (Apple Silicon, this repo's dev box), release build,
  wall-clock integer microseconds; other local work ran during parts of
  the session — treat spend distributions as indicative, counts and
  choices as exact.
- The wake rule is one declared rule (σ1 leg settles for the rival).
  Other decision-wake readings (e.g. newly-settled agreement) are
  recorded in the census but do not gate escalation in this epoch.
- Wake density is relative to THIS epoch pair and THIS baseline
  (act at cap 128): a different σ1 or a different baseline cap is a
  different experiment.
- Bids are always 30 points by the rotating bidder; no competitive
  auction. The profile covers play decisions only (the auction is the
  existing level-1 policy, not a waking decision, and its wall time is
  NOT in the census records).
- The census's `agreed` flag measures deviation from act's σ0 choice,
  not correctness; no strength claim is made here (CE-A7/§20.16 —
  nothing in this probe touches the live player or any default).
