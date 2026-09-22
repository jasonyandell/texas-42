# og-v5: the anytime harvester — three promotions, a futility save, exam +101‰

2026-09-22. **EXPLORATORY**; records govern: `generations.jsonl`,
`exam.json`, `panel.json`, `state.txt`,
`../og-v4b/pilot-og-v5-proxy.json` (the fair proxy pilot that shaped this
campaign's design).

## The declaration

Target: the maintained gym field (`GymField(S0, 40)` — L1-40/8 partner,
L0-8 opponents; learner at S0 alone), campaign offset 500,000,000.
Training/construction/screening proxy: `GymField(S0, 4)` — same
architecture, 3.6 vs 24.7 ms/deal; the fair pilot (og-v4b incumbent vs
uniform, 1,024 paired seeds under both lineups) measured ρ = 0.38 and
Var(D_H−D_L) = 0.363 > Var(D_H) = 0.297 — direct promotion wins by 2.4×,
the second independent §6 measurement that paired make/set differences
are proxy-resistant on this game. Promotion mode **`anytime-direct`**:
per frozen candidate, two of the adjudicated CE-T4/T5 bounded-mean
betting mixtures (`walt::solver::evidence::BoundedMeanMixture` is the
authority; the campaign's gcd-free integer evaluation is gate-checked
equal), λ grid {1/64…1/2} equal-weighted, τ = 1/100, whole-stream risk
α_k = δ/(k(k+1)) split across the promote and futility sides, judged
every 512 paired deals, cap 65,536 — Ville makes stopping valid at every
n, so there is no per-checkpoint α spend.

## The run

| Gen | Proxy train Y | Dict | Verdict | Estimate | Resolved at n | Constructor admissions |
|---|---|---|---|---|---|---|
| 0 | 410‰ | 17 | **PROMOTED** | +3.8% | **6,656** | takes-trick\|opponent-winning, loses-trick\|opponent-winning, not-double∧would-not-lead-led |
| 1 | 441‰ | 20 | Unresolved | +1.5% | cap | not-double∧not-count5, not-double∧count0, double\|pos-lead |
| 2 | 451‰ | 23 | **PROMOTED** | +3.1% | **12,288** | not-double\|pos-lead, count0\|pos-lead, not-count0\|pos-lead |
| 3 | 456‰ | 26 | Unresolved | +0.7% | cap | not-master∧not-count5, not-master∧count0, count0∧not-trump-boss |
| 4 | 462‰ | 29 | **PROMOTED** | **+1.9%** | **60,416** | not-double∧not-trump, not-double∧trump, not-double∧not-master |
| 5 | 467‰ | 32 | **NotPromoted** | −0.5% | **22,528** | not-count5∧not-trump-boss, not-count5∧not-trump, not-master∧not-trump |
| 6 | 464‰ | 35 | Unresolved | +0.7% | cap | not-master∧count5, count5∧not-boss-led, not-count5 |
| 7 | 465‰ | 38 | Unresolved | +0.8% | cap | not-boss-led∧trump, would-not-lead-led∧not-trump, off-led∧trump |

Declared budget stop at 8 generations (the stall rule never reached 3).

## The anytime economics, against og-v4b's fixed checkpoints (same target)

- **Promotions resolved at 6,656 / 12,288 / 60,416 paired deals** — sized
  by the edge itself. og-v4b's fixed schedule spent 32,768 and 8,192 for
  its two promotions and could not, at ANY size, certify a sub-2.4% edge
  (its final-checkpoint EB radius floor). Generation 4's **+1.9%
  promotion is the first resolution inside the 1–2% band** across all six
  campaigns — the exact hole the fixed rules kept filing as Unresolved.
- **The futility side paid too**: generation 5's slightly-negative
  candidate was dismissed at 22,528 deals instead of burning the full cap.
- Five decisive verdicts in eight streams, versus og-v4b's two in five —
  with LESS risk spent per candidate (α_k = δ/(k(k+1)) whole-stream,
  no checkpoint splitting).
- Engineering note on the record: the first judge implementation
  normalized (gcd) ~850k-bit rationals and ground a stream to ~2 h; the
  gcd-free integer form (factors are integers over q·d with power-of-two
  λ denominators) runs 128 full-stream judges in 5.0 s, gate-checked
  equal to walt's `BoundedMeanMixture` observation for observation.

## The untouched exam (`exam.json`; DIRECT gym field, seeds 509,000,000+, n = 2,048, touched once)

| Actor | Makes | Rate |
|---|---|---|
| Final incumbent (3 promotions, 38 expressions) | 1,062 / 2,048 | **518‰** |
| Generation-0 uniform actor | 855 / 2,048 | 417‰ |

Paired difference **+207/2,048 = +101.0‰**, exact two-sided Hoeffding
interval at α = 1/20: **[+40.8‰, +161.3‰]** — the strongest gym-field
result of the program (og-v4b: +76.1‰ [15.9, 136.4] on its own disjoint
exam; independent measurements, not one tournament).

## Caveats

- Same target fence: J_T for this gym lineup and contract law only.
- The three cap-outs (+0.7–1.5%) remain UNRESOLVED, typed; edges below
  ~1.5% over τ want a longer declared cap, not a different rule.
- ~2.9M proxy deals + ~0.7M gym deals, ≈ 6.5 h wall (one gen re-run after
  the judge fix; the pre-fix partial run used no fresh seeds and recorded
  nothing).

## Reproduction

```
cd experiments/og-learning
cargo test --release          # 32 law-sized gates
./target/release/og_campaign init  --dir campaigns/og-v5 --field gym-v5 --promotion anytime-direct \
                                   --train 4096 --dev 2048 --stall 3 --offset 500000000 --constructor 1
./target/release/og_campaign panel --dir campaigns/og-v5
./target/release/og_campaign train --dir campaigns/og-v5 --generations 8
./target/release/og_campaign exam  --dir campaigns/og-v5 --deals 2048
```
