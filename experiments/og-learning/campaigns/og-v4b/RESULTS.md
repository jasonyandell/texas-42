# og-v4b: the learner adds value beside an L1 partner — direct gym exam +76‰

2026-09-22. **EXPLORATORY**; records govern: `generations.jsonl`,
`exam.json`, `panel.json`, `state.txt`. Companion campaign to the og-v4
pilot (`../og-v4/RESULTS.md`), which measured the all-L0 proxy's
correlation too weak for multifidelity promotion; this campaign runs the
pilot-informed rule.

## The declaration

Target `og-v4/bid30-longest-pip/S0-learner/gym-field-l1p40-l0o8-v1`: the
learner holds S0 alone inside walt's maintained `GymField(S0, 40)` — the
L1 fixed-40/8 partner at S2, Level0Field(8) opponents. Campaign offset
400,000,000. The all-L0-8 proxy drives training, construction and
development screening only (selection, never a guarantee). Promotion mode
`direct-eb` (`EvidenceRule::gym_direct()`): DIRECT gym-field paired
streams judged with exact-rational empirical-Bernstein radii
(variance-sensitivity halves the Hoeffding radius at the measured
Var(D_H) ≈ 0.31), δ = 1/20, τ = 1/100, checkpoints 2,048 / 8,192 / 32,768.

## The run

| Gen | Proxy train Y | Dict | Candidate (direct gym stream) | Verdict | Constructor admissions |
|---|---|---|---|---|---|
| 0 | 409‰ | 17 | +961/32,768 ≈ **+2.9%** | **PROMOTED** (32k) | not-double∧would-not-lead-led, trump\|pos-lead, not-trump\|pos-lead |
| 1 | 439‰ | 20 | +334/8,192 ≈ **+4.1%** | **PROMOTED** (8k) | double∧off-led, not-double∧off-led, double∧not-boss-led |
| 2 | 458‰ | 23 | +209/32,768 ≈ +0.6% | Unresolved | not-double∧trump, not-count10∧trump, trump |
| 3 | 462‰ | 26 | +463/32,768 ≈ +1.4% | Unresolved | master\|pos-lead, not-master\|pos-lead, not-master∧not-count5 |
| 4 | 457‰ | 29 | +416/32,768 ≈ +1.3% | Unresolved | not-master∧not-trump, count0∧trump, count0∧not-trump |

Registered stall stop after three consecutive non-promotions. The two
promotions are the first ever certified against the gym field; the three
Unresolveds are small-but-positive residuals below the resolvable band
(the CE anytime-valid machinery remains the natural harvester). Where
og-v2/og-v3's admissions were count husbandry and trick contention, this
lineup pulled **lead discipline** into the dictionary (trump\|pos-lead,
master\|pos-lead — what S0 should lead in front of a strong partner) and
slough discipline (off-led conjunctions).

## The untouched exam (`exam.json`; DIRECT gym field, seeds 409,000,000+, n = 2,048, touched once)

| Actor | Makes | Rate |
|---|---|---|
| Final incumbent (2 promotions, 29 expressions) | 1,040 / 2,048 | **507‰** |
| Generation-0 uniform actor | 884 / 2,048 | 431‰ |

Paired difference **+39/512 = +76.1‰**, exact two-sided Hoeffding interval
at α = 1/20: **[+15.9‰, +136.4‰]** — clear of zero on the expensive target
itself, no proxy anywhere in the claim.

## Reading it against the other campaigns

The absolute gain is smaller than og-v2/og-v3's (+201‰, +186‰) for a
structural reason worth stating: those learners replaced BOTH partnership
seats of a uniform start; here the L1 partner already carries the
partnership to 431‰, and the learner's one seat adds +76‰ on top. The
question this campaign asked — can the outcome gradient add certified
value beside a competent teammate? — is answered yes.

## Costs and caveats

- ~30 min/generation wall (≈ 1.7M proxy deals plus up to 65,536 gym deals
  per promotion stream, rayon-parallel L1 partner at ~25 ms/deal);
  exam 104 s. Total ≈ 2.5 h.
- Same target fence as always: this is J_T for THIS gym lineup (L1-40/8
  partner, L0-8 opponents, bid-30 longest-pip law) — not L2, not the full
  auction, not humans.
- og-v4 (the MF pilot) and og-v4b together are one §6 story: pilot,
  measured verdict, resized rule, certified result — recorded end to end.

## Reproduction

```
cd experiments/og-learning
./target/release/og_campaign init  --dir campaigns/og-v4b --field gym --promotion direct-eb \
                                   --train 4096 --dev 2048 --stall 3 --offset 400000000 --constructor 1
./target/release/og_campaign panel --dir campaigns/og-v4b
./target/release/og_campaign train --dir campaigns/og-v4b --generations 8
./target/release/og_campaign exam  --dir campaigns/og-v4b --deals 2048
```
