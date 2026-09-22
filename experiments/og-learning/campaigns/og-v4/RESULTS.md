# og-v4: the multifidelity pilot — the harness ran, measured itself, and said "direct"

2026-09-22. **EXPLORATORY**; records govern: `generations.jsonl`,
`panel.json`, `state.txt`. **No exam was run and none is meaningful: no
candidate promoted, so the final incumbent IS the generation-0 uniform
actor, and an exam of identical actors is vacuous (declared).**

## What this campaign was

The first run of the §6 multifidelity harness, against the maintained gym
field (`GymField(S0, 40)`: L1-40/8 partner, L0-8 opponents; learner at S0
alone; offset 300M). Declared cheap proxy: the all-L0-8 lineup (0.7
ms/deal vs the gym's 24.7). Training, construction and screening on the
proxy; promotion by the two-batch estimator
Δ̂ = mean_N(D_L) + mean_M(D_H − D_L) over independent seed subregions,
exact empirical-Bernstein radii per batch, τ = 1/25, checkpoints
(N, M) = (16384, 256), (65536, 1024), (65536, 4096).

## The run: three real candidates, zero resolutions, honest stall

| Gen | Δ̂ (final) | Verdict | Constructor admissions |
|---|---|---|---|
| 0 | +1537/32768 ≈ **+4.7%** | Unresolved | takes-trick\|opponent-winning, loses-trick\|opponent-winning, not-double∧would-not-lead-led |
| 1 | +1905/32768 ≈ **+5.8%** | Unresolved | not-double∧not-boss-led, trump\|opponent-winning, not-trump\|opponent-winning |
| 2 | +1021/16384 ≈ **+6.2%** | Unresolved | double∧off-led, not-double∧off-led, double∧not-boss-led |

Registered stall stop at three consecutive non-promotions. Every estimate
sat ABOVE the declared τ = 4% — the failures were resolution, not absence
of signal. The candidates' proxy training (make rate 42.0% → 43.5% across
the generations' working actors) and the admissions (trick contention
against defenders, as in og-v3) behaved exactly as in prior campaigns.

## The pilot measurement — the §6 economics, quantified on the record

The generation-0 stream doubled as the pilot the parent demands
("estimate the variances and actual complete costs in pilots"):

- Var(D_L) = 20,514,391/67,104,768 ≈ **0.306**
- Var(D_H − D_L) ≈ **0.404** (checkpoint 3; 0.390–0.411 across checkpoints)
- Sample Cov(D_H, D_L) = 99,861/931,840 ≈ **+0.107** ⇒ implied
  Var(D_H) ≈ 0.313, correlation ρ ≈ **0.35**
- Measured costs: cheap pair ≈ 1.4 ms, correction quadruple ≈ 52 ms,
  direct gym pair ≈ 49 ms
- Recorded §6 allocation ratio (optimal N/M given MF): ≈ 6.27 — the
  declared 16:1 schedule over-weighted the cheap batch, also measured.

**The verdict the numbers force:** Var(D_H − D_L) > Var(D_H). The
correction variable is noisier than the direct difference — the L1↔L0
partner swap decorrelates more than the shared opponents correlate — and
a correction quadruple costs as much as a direct pair. Optimal-allocation
MF at these statistics is ≈ 1.8× WORSE than direct sampling per unit
cost. The parent's own caveat, "a proxy with poor correlation can cost
more than it saves," is now a measured record, not a warning. Secondary
finding: with D ∈ {−1,0,1} differences, τ = 1/25 under these radii
required Δ̂ ≳ 9% to promote — mis-sized against the 4–6% gains actually
on offer.

## What follows (og-v4b, declared)

Same target, same proxy roles where they pay (training, construction,
screening — no guarantee ever claimed off-proxy), promotion switched to
the pilot-informed mode: DIRECT gym streams with variance-sensitive
empirical-Bernstein radii (`EvidenceRule::gym_direct()`: τ = 1/100,
checkpoints 2,048 / 8,192 / 32,768), campaign offset 400M. The MF
machinery stays in the crate for proxies that earn it (the record now
says what "earning it" measures as).

## Reproduction

```
cd experiments/og-learning
./target/release/og_campaign init  --dir campaigns/og-v4 --field gym --train 4096 --dev 2048 \
                                   --stall 3 --offset 300000000 --constructor 1
./target/release/og_campaign panel --dir campaigns/og-v4
./target/release/og_campaign train --dir campaigns/og-v4 --generations 8
```
