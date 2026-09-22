# og-v1: outcome-grounded policy learning ran to completion and its exam is clean

2026-09-21. **EXPLORATORY** finite-target evidence; every number below is a
statement about the declared campaign target only, never a strength claim
against L0/L1 fields, human tables, other contracts, or the auction. The
machine-readable records govern this prose: `generations.jsonl` (one JSON
object per generation), `exam.json`, `panel.json`, `state.txt` (the final
incumbent's exact coefficients), `../og-v0-pilot/` (the superseded
single-step pilot, kept as provenance).

## What ran

The first complete campaign of the outcome-grounded learning contract
(`walt/math/outcome_grounded_policy_learning_v0.1.md`; intake companion
governs where it narrows; parent adjudication still PENDING — the build was
authorized by Jason in-session on 2026-09-21). The whole loop of the
parent's §5 executed autonomously: freeze → on-policy complete deals →
outcome gradient → bounded coefficient updates → frozen finalists →
risk-budgeted promotion → repeat → registered stall stop → untouched exam.
No teacher, no exact solver, no human tactical feedback anywhere in the
loop: the only learning signal was make/set of complete deals.

**Target** (`og-v1/bid30-longest-pip/S0S2-learner/S1S3-hash-legal-v1`):
uniform seeded 28-tile deals; S0 declares its longest pip suit (ties high)
at bid 30 and leads; utility Y = declaring side banks ≥ 30 of the 42
(make/set); S0+S2 are two lawful invocations of ONE shared coefficient
vector (each sees only its own hand + public record); S1+S3 are walt's
`HashField` (hash-seeded uniform legal — a pure function of public state,
so paired arms couple automatically). Changing any of this is a new target.

**Actor**: rational multiplicative-weights softmax over the versioned
14-clause relational library (`scheme-relational-actor-v1/grammar-v1/
straight-v0.4`), evaluated as SET-valued features x_j(I,a) through
`CompiledFix::evaluate_viewer` — hidden-world predicates are rejected by
the scheme runtime's type system. All probabilities and draws exact
rational; θ = ln r never materialized; every learned coefficient below is
an exact power of 9/8.

**Configuration** (frozen at init, `state.txt`): 8 inner steps per
generation × 4,096 fresh on-policy deals per step; step rule: multiply
r_j by (9/8)^±1 on coordinates with |Ĝ_j| ≥ max|Ĝ|/4, clamp to [1/64, 64];
snapshot candidates after 2/4/8 steps; development screen on 2,048 fresh
paired deals; evidence rule δ = 1/20 total, τ = 1/100, checkpoints
n = 256/1,024/4,096/16,384/65,536, α[k,j] = δ/(k(k+1)j(j+1)), exact
rational Hoeffding upper bounds; stall stop after 3 consecutive
non-promotions. Seed ranges disjoint by construction (crate README).

## The run (from `generations.jsonl`)

| Gen | Train mean Y | Frozen candidate | Paired sum D / n | Verdict |
|---|---|---|---|---|
| 0 | 454‰ | k=1, 8 steps, 35 moves | +769 / 16,384 | **PROMOTED** |
| 1 | 497‰ | k=2, 8 steps, 30 moves | +1,043 / 65,536 | Unresolved |
| 2 | 495‰ | k=3, 8 steps, 22 moves | +1,459 / 65,536 | Unresolved |
| 3 | 497‰ | k=4, 8 steps, 37 moves | +1,870 / 65,536 | **PROMOTED** |
| 4 | 531‰ | k=5, 8 steps, 38 moves | +2,105 / 65,536 | **PROMOTED** |
| 5 | 555‰ | k=6, 4 steps, 19 moves | +950 / 65,536 | Unresolved |
| 6 | 554‰ | k=7, 4 steps, 21 moves | +710 / 65,536 | Unresolved |
| 7 | 558‰ | k=8, 8 steps, 39 moves | +1,415 / 65,536 | Unresolved → stall stop |

Three promotions in eight generations; the campaign stopped by its own
registered stall rule (3 consecutive non-promotions), not by judgment.
The non-promotions are the discipline working: candidates 2, 3, 6, 7, 8
all had positive paired means (+1.1% to +2.2%) whose lower confidence
bounds did not clear τ = 1% — they remain UNRESOLVED, typed outcomes,
never relabeled. Total ≈ 1.3M complete deals, ≈ 15 minutes wall,
single-threaded.

## The untouched exam (`exam.json`; seeds 9,000,000..9,004,095, touched once)

| Actor | Makes | Rate |
|---|---|---|
| Final incumbent (3 promotions) | 2,255 / 4,096 | **550‰** |
| Generation-0 uniform actor | 1,739 / 4,096 | 424‰ |

Paired difference **+129/1,024 = +125.9‰**, two-sided exact-rational
Hoeffding interval at α = 1/20: **[+83.3‰, +168.5‰]** (radius upper bound
8,519/200,000). The learned actor makes the 30-bid about 12.6 percentage
points more often than the uniform actor it started as, on this target.

## What it learned (final coefficients, `state.txt`, as powers of 9/8)

| Clause | Exponent | Reading (mechanical description, not a proved tactic) |
|---|---|---|
| `master` | **+24** (≈ ×16.9) | strongly prefer playing current masters |
| `partner-winning-master` | −14 (≈ ×0.19) | …but not onto tricks partner already holds |
| `count-5` | −10 (≈ ×0.31) | avoid exposing five-count in contested tricks |
| `partner-winning-count-5` | +10 (≈ ×3.25) | feed five-count to partner's winning trick |
| `partner-winning-count-10` | +5 (≈ ×1.80) | feed the ten-count likewise |
| `partner-winning-count-0` | −17 (≈ ×0.13) | don't waste the blank throw when partner is winning |
| `count-0` | +6 (≈ ×2.03) | otherwise prefer dumping zero-count |
| `boss-led` | +3 (≈ ×1.42) | prefer boss of the led suit |
| `count-10` | +1 | weak |
| `legal`, `partner-winning-legal`, `follow-led`, `partner-winning-follow-led`, `partner-winning-boss-led` | 0 | untouched (the first four are action-independent or near-constant on this target's decisions — see `panel.json`) |

Count husbandry and master timing emerged from make/set outcomes alone.
These readings are descriptions of the learned coefficients, not
adjudicated 42 strategy.

## Caveats, in the parent's own terms

- **The claim is J_T for this T.** Fixed random-legal co-players, fixed
  declaration rule, bid 30, make/set. Nothing here extends to stronger
  fields, humans, other contracts, or the auction (parent §1). The natural
  next targets — L0-8 opponents, the gym field — are new campaigns.
- The gradient is a proposal statistic (declared dyadic 2^-32 quantization;
  LOO baseline); every promotion decision consumed only exact make/set
  counts and exact-rational radius upper bounds.
- Hoeffding is conservative; variance-sensitive confidence sequences
  (walt's adjudicated CE machinery) would resolve the +1–2% candidates this
  rule left UNRESOLVED — that is the intake companion's agenda item 6.
- Dev-column incumbents across generations use different seed regions and
  are not a comparable series; only the paired promotion streams and the
  exam compare actors on common deals.
- Running out the stall rule is UNRESOLVED-shaped, not a grammar ceiling
  claim (parent §9): the v1 library plainly still carries unresolved
  positive directions (candidate 8's +2.2%).

## Reproduction

```
cd experiments/og-learning
cargo test --release          # 21 law-sized gates
cargo build --release
./target/release/og_campaign init  --dir campaigns/og-v1 --train 4096 --dev 2048 --stall 3
./target/release/og_campaign panel --dir campaigns/og-v1
./target/release/og_campaign train --dir campaigns/og-v1 --generations 8
./target/release/og_campaign exam  --dir campaigns/og-v1 --deals 4096
```

Deterministic end to end: deals, actor draws, and `HashField` are pure
functions of declared seeds and public state (replay audit: `bench
--deals 300` reproduces 139/300 exactly across binary rebuilds).
