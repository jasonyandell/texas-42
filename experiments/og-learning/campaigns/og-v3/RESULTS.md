# og-v3: the constructor holds up against σ0 — exam +186‰ vs L0-8 defenders

2026-09-21. **EXPLORATORY** finite-target evidence; every number is about
the declared campaign target only. Machine-readable records govern:
`generations.jsonl`, `exam.json`, `panel.json`, `state.txt`.

## The target

`og-v3/bid30-longest-pip/S0S2-learner/S1S3-level0-n8-v1`, authorized by
Jason in-session ("go — declare og-v3 against L0-8 and let it run").
Identical law to og-v2 — uniform seeded deals, S0 declares its longest pip
suit at bid 30 and leads, S0+S2 are two lawful invocations of one shared
coefficient vector, make/set utility, constructor on, same evidence rule
(δ = 1/20, τ = 1/100, checkpoints to 65,536 paired deals) — except the
fixed seats: S1+S3 are walt's **`Level0Field::new(8)`**, the σ0 modeled
mind (n0 = 8 no-void belief worlds from the frozen INNER_SEED derivation,
best response against the Dice field; `walt/walt/src/solver/policy.rs`).
σ0 is a pure function of seat, hand and public record, so paired arms
couple and the campaign replays exactly (verified cross-process at init).
Campaign seed offset 200,000,000 — no deal shared with og-v1/og-v2.
Measured at init: 0.70 ms/deal, so the full og-v2 configuration ran
unchanged and the §6 multifidelity harness stays honestly queued for
genuinely expensive lineups (the gym field's L1 partner).

## The run (`generations.jsonl`)

| Gen | Train Y | Dict | Admitted (constructor) | Candidate | Paired D / n | Verdict |
|---|---|---|---|---|---|---|
| 0 | 336‰ | 17 | takes-trick\|opponent-winning, loses-trick\|opponent-winning, not-master∧loses-trick | k=1 | +1,102 / 16,384 | **PROMOTED** |
| 1 | 383‰ | 20 | not-double∧not-count5, not-double∧count0, not-count5∧not-trump-boss | k=2 | +2,285 / 65,536 | **PROMOTED** |
| 2 | 424‰ | 23 | trump, not-trump, not-count10∧not-trump | k=3 | +1,476 / 65,536 | Unresolved |
| 3 | 427‰ | 26 | not-double∧would-not-lead-led, double∧off-led, not-double∧off-led | k=4 | +824 / 16,384 | **PROMOTED** |
| 4 | 462‰ | 29 | not-double∧not-trump, not-double∧not-count10, count0∧not-trump-boss | k=5 | +618 / 65,536 | Unresolved |
| 5 | 469‰ | 32 | would-not-lead-led∧not-trump, count0∧takes-trick, would-not-lead-led∧loses-trick | k=6 | +2,026 / 65,536 | **PROMOTED** |
| 6 | 486‰ | 35 | not-master∧count0, not-master∧not-count5, not-master∧not-count10 | k=7 | +100 / 65,536 | Unresolved |
| 7 | 493‰ | 38 | not-double∧not-trump-boss, not-double∧not-master, not-count5∧not-trump | k=8 | +615 / 65,536 | Unresolved |

Four promotions in eight generations; **declared budget stop** at the
8-generation budget (the stall rule never fired — gains were flattening,
gen 6 nearly flat at +0.15%, but gen 7 still measured +0.9% unresolved).
Candidates 3, 5, 7, 8 stay UNRESOLVED, typed, never relabeled.

**The constructor's choices tracked the lineup.** Against random-legal
seats (og-v2), generation 0 admitted count-husbandry conjunctions. Against
σ0 defenders, generation 0 admitted **trick-contention** relations —
`takes-trick|opponent-winning`, `loses-trick|opponent-winning` — and
generation 3 added slough discipline (`off-led` conjunctions), generation 5
cheap steals (`count0∧takes-trick`). Same language, same admission rule,
different opponents, different mathematics selected. No seed clause could
express any of the `beats`-composite relations.

## The untouched exam (`exam.json`; seeds 209,000,000+, touched once)

| Actor | Makes | Rate |
|---|---|---|
| Final incumbent (4 promotions, 38 expressions) | 1,978 / 4,096 | **482‰** |
| Generation-0 uniform actor | 1,216 / 4,096 | 296‰ |

Paired difference **+381/2,048 = +186.0‰**, exact two-sided Hoeffding
interval at α = 1/20: **[+143.4‰, +228.6‰]**.

## The three campaigns side by side

Each against its own uniform start, on its own disjoint untouched exam
range; these are three independent measurements, not one tournament.

| Campaign | Fixed seats | Language | Exam: uniform → learned | Paired | Exact 95% CI |
|---|---|---|---|---|---|
| og-v1 | random-legal | fixed 14-clause library | 424‰ → 550‰ | +125.9‰ | [83.3, 168.5]‰ |
| og-v2 | random-legal | constructor open | 437‰ → 638‰ | +201.1‰ | [158.5, 243.7]‰ |
| og-v3 | **σ0 (L0-8)** | constructor open | 296‰ → **482‰** | **+186.0‰** | [143.4, 228.6]‰ |

The open-language gain transferred essentially undiminished to the harder
lineup (+201‰ → +186‰), while the absolute problem got much harder (the
uniform start makes only 296‰ against σ0). The learned partnership now
makes the 30-bid against the σ0 field at nearly the rate the uniform
partnership managed against random players.

## Caveats

- The claim is J_T for THIS target: σ0 opponents at n0 = 8 under this
  declaration rule and contract. σ0 is the field the live level-1 player
  models — a meaningful lineup in this project's ontology — but this is
  still not a strength claim against L1, the gym field, or humans.
- Cross-campaign rows above are independent experiments on different deal
  laws' seeds; only within-campaign paired numbers carry the evidence
  rule's guarantees.
- Budget stop, not a ceiling: gens 7's +0.9% and the still-productive
  admissions say the language has more to give; the CE anytime-valid
  machinery (companion agenda item 6) is the cheapest way to harvest the
  1–2% band the Hoeffding rule leaves unresolved.
- ~2.7M complete deals, ≈ 50 min single-threaded, ~0.7 ms/deal.

## Reproduction

```
cd experiments/og-learning
cargo test --release          # 26 law-sized gates
cargo build --release
./target/release/og_campaign init  --dir campaigns/og-v3 --field l0-8 --train 4096 --dev 2048 \
                                   --stall 3 --offset 200000000 --constructor 1
./target/release/og_campaign panel --dir campaigns/og-v3
./target/release/og_campaign train --dir campaigns/og-v3 --generations 8
./target/release/og_campaign exam  --dir campaigns/og-v3 --deals 4096
```
