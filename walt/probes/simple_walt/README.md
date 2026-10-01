# simple_walt — walt on one page (EXPLORATORY)

Below every evidentiary tier; cited by nothing above it. Pure-Python stdlib.

`simple_walt.py` is the whole algorithm: a ladder of best responses that bottoms
out at random, with pip-trump rules from the suit algebra in
`walt/walt/src/rules/rules.rs`. Live walt corresponds to `decide(..., level=1)`.
Differences from the Rust walt: no tie refinement, no race/refine sampling
schedule, pip trumps only, bid 30 only; each decision gets fresh memos.

`experiment.py` asks whether replay matters. Positions come from self-play
by memoized walt (level 1, 30 own-seat deals, 8 deals per modeled mind). At each
non-forced decision walt is rerun on the same own-seat deals as
memo/fresh × inner seed A/B ("fresh" = no value memo, no decision memo, so every
modeled decision and random draw is redone at every visit).

Run 2026-09-29 (`results.jsonl`, 179 decisions, 11 deals, seeds 1000–1011
minus 1006, which hit the chunk timeout):

| comparison | move differs |
|---|---:|
| memo A vs memo B (noise floor: only inner randomness reseeded) | 59 / 179 |
| fresh A vs fresh B | 60 / 179 |
| memo A vs fresh A (same seeds; only replay removed) | 61 / 179 |
| memo B vs fresh B | 62 / 179 |

By trick: 1–2 → noise 29/61 vs replay-removed 26/61; 3–4 → 25/60 vs 30/60;
5–7 → 5/58 vs 5/58. Time: memo 685 s, fresh 401 s (memo bookkeeping costs
more than it saves); nodes 77.4M vs 82.7M.

Why: memo hit rates measured at sample positions (deals 1000–1002) — modeled
decisions replayed 0% at tricks 1 and 3 and 4–13% at trick 5; position memo
hits under 1% throughout. A modeled seat almost never holds the identical hand
in two sampled deals, so the decision cache rarely has anything to replay.
One move per situation is carried by the structure instead: `decide` reads only
(seat, hand, public), and the real seat's max in `value` is taken once over the
whole group of deals.

## Keying the model's randomness (`crn_experiment.py`, 2026-10-01)

Question: does keying the model's randomness (level-0 dice and modeled seats'
belief shuffles) on (world, seat, trick) instead of the public record make the
root's argmax steadier, by making the noise common across candidate tiles?
Noise floor = move changes when only the inner base seed changes, same
own-seat deals. Positions from self-play; 30 own-seat deals, 8 per modeled
mind. `crn.jsonl` (seeds 2000–2019) and `crnv.jsonl` (3000–3015, with option
values recorded).

| keying | noise floor, pooled 591 decisions |
|---|---:|
| running rng (walt42.py as committed) | 193 / 591 = 33% |
| record-keyed (the Rust ticker tape) | 201 / 591 = 34% |
| trick-keyed (world, seat, trick) | 192 / 591 = 32% |

No effect. The root's candidate tiles diverge the play after one trick, so
"the same random number at (world, seat, trick)" lands on different situations
and the correlation the idea relied on decays almost immediately.

Own-seat deals also barely move the floor (running keying, `crn15.jsonl`,
`crn.jsonl`, `crn60.jsonl`): 15 deals 35%, 30 deals 36%, 60 deals 31%.

The flips are not harmless ties. On the 270 decisions with values recorded,
scoring one seed's pick with the other seed's values gives a mean cross-regret
of ~2.5 deals of 30 (about 8 points of pmake), with ~40% of flips at 3+ deals;
exact ties under both scorings are rare (0–4 of ~90). The noise is in the
modeled seats (8-world best responses, reseeded in both arms), which neither
keying nor outer deals address. The Rust player's saturation-tie refinement is
aimed at the same symptom.

## Sequential sampling in the modeled mind (`seq_experiment.py`, 2026-10-01, round 1 of 3)

EXPLORATORY. Question: does a modeled mind that samples worlds sequentially
(blocks from its own chair, stop when its top two options separate) give the
root a lower noise floor than a fixed per-mind world count at the same cost?
Setup: 273 non-forced decisions from self-play (16 games, seeds 4000+; the same
positions in every variant, so comparisons are paired); 30 own-seat deals at the
root, fixed across variants and A/B; inner randomness one running rng, seed
A vs B. Variant names are the spec: `fixedN` = N worlds per modeled decision,
`seqBxMAXmM` = blocks of B worlds, at most MAX, stop at a lead of M deals. A
sequential mind re-solves one joint problem over every world drawn so far.
Noise floor = A and B pick different tiles. Cross-regret = for a flip, the larger
of what either pick loses under the other seed's option values (deals of 30),
averaged over ALL decisions (non-flips count 0); per-flip mean in brackets.
Interval = 2 SE binomial (+-, percentage points); numbers re-derived from the
`seq_*.jsonl` files, not from the run summaries.

| variant | decisions | noise floor (flips) | 2-SE | cross-regret / decision [per flip] | s/decision | inner worlds / modeled decision |
|---|---:|---:|---:|---:|---:|---:|
| fixed8 | 273 | 30.0% (82) | +-5.5 | 0.83 [2.77] | 1.24 | 8.00 |
| fixed16 | 273 | 28.6% (78) | +-5.5 | 0.73 [2.56] | 2.44 | 16.00 |
| seq4x16m2 | 273 | 35.5% (97) | +-5.8 | 0.90 [2.54] | 3.71 | 9.23 |
| seq4x32m2 | 273 | 31.9% (87) | +-5.6 | 0.86 [2.70] | 7.29 | 11.94 |

Paired differences (same positions; 2 SE of the mean difference; regret in
deals/decision): fixed16 - fixed8 -1.5 pp +-6.2, -0.10 +-0.21; seq4x16m2 -
fixed8 +5.5 pp +-6.2, +0.07 +-0.22; seq4x32m2 - fixed8 +1.8 pp +-6.8, +0.03
+-0.24; seq4x16m2 - fixed16 +7.0 pp +-6.6, +0.17 +-0.22; seq4x32m2 - fixed16
+3.3 pp +-6.4, +0.13 +-0.21.

What it shows:
- Matched on inner worlds (seq4x16m2 at 9.2 vs fixed8; seq4x32m2 at 11.9 vs
  fixed8/fixed16) no sequential variant has a lower floor or lower regret than
  its fixed neighbour. The point estimates go the wrong way (+5.5, +1.8 pp
  vs fixed8), all inside noise. Nothing beats a fixed variant by 2 SE.
- Matched on seconds the comparison cannot be made yet: sequential costs 2.6-3.9x
  more time per world drawn (0.16 and 0.25 ms/world vs 0.06 for fixed), because
  the joint problem is re-solved after every block. seq4x16m2 (3.7 s) sits near
  an untested fixed24, seq4x32m2 (7.3 s) near an untested fixed48; the fixed
  grid ends at 16. On time, fixed16 (2.4 s) already has a lower floor point
  estimate than either sequential variant at 1.5-3x the cost.
- Doubling the fixed count 8 -> 16 moved the floor by 1.5 pp +-6.2 (30.0% ->
  28.6%). The floor is nearly flat in inner world count over this range, so
  the experiment has little dynamic range for any allocation scheme to win.

Caveats: 273 decisions from 16 games, so decisions within a game are correlated
and the binomial SE is optimistic; a 2 SE interval is about +-5.5 pp, so only
differences above ~7 pp (paired ~6-7 pp) are detectable. Cross-regret is
scored from 30-deal option values that carry their own noise, and "per flip"
vs "per decision" differ by the floor (the run summaries print per flip).
Time includes bookkeeping and was measured on a 4-core machine with
Pool(4). Realized mean worlds are the sequential cost, not the cap. Round 1:
not settled; round 2 proposes fixed24, fixed48 and fixed32 to extend the fixed
grid to the sequential variants' time cost and to 32 worlds.

## Sequential sampling, round 2 of 3: cost-matched fixed grid (`seq_experiment.py`, 2026-10-01)

EXPLORATORY. Question: with the fixed grid extended to the sequential variants'
realized cost (fixed24, fixed32, fixed48 added to round 1), does any sequential
mind beat the fixed mind of similar cost on noise floor AND cross-regret?
Setup unchanged from round 1 (same 273 positions, same 30 root deals, seed A vs
B, one running rng). All seven results files hold 273 unique rows with both
seeds; fixed8's earlier duplicate smoke rows were already removed. Numbers
re-derived from the `seq_*.jsonl` files. Cross-regret is per decision (flips
only count, non-flips are 0), per-flip mean in brackets (the run summaries
print only the per-flip figure). Interval = 2 SE binomial, percentage points
for the floor; +-2 SE of the mean for regret.

| variant | decisions | noise floor (flips) | 2-SE | cross-regret / decision [per flip] | s/decision | inner worlds / modeled decision |
|---|---:|---:|---:|---:|---:|---:|
| fixed8 | 273 | 30.0% (82) | +-5.5 | 0.83 +-0.18 [2.77] | 1.24 | 8.00 |
| fixed16 | 273 | 28.6% (78) | +-5.5 | 0.73 +-0.17 [2.56] | 2.44 | 16.00 |
| seq4x16m2 | 273 | 35.5% (97) | +-5.8 | 0.90 +-0.18 [2.54] | 3.71 | 9.23 |
| fixed24 | 273 | 35.2% (96) | +-5.8 | 0.90 +-0.18 [2.55] | 3.82 | 24.00 |
| seq4x32m2 | 273 | 31.9% (87) | +-5.6 | 0.86 +-0.18 [2.70] | 7.29 | 11.94 |
| fixed32 | 273 | 31.1% (85) | +-5.6 | 0.75 +-0.16 [2.40] | 5.14 | 32.00 |
| fixed48 | 273 | 28.2% (77) | +-5.4 | 0.71 +-0.16 [2.52] | 7.63 | 48.00 |

Cost matches. Time per world is flat for fixed (0.056-0.063 ms/world) and
0.16 / 0.25 ms/world for sequential, so seq4x16m2 costs the time of about
fixed24 (3.71 vs 3.82 s) and seq4x32m2 about fixed48 (7.29 vs 7.63 s); on
worlds drawn they sit at 9.2 and 11.9, near fixed8/fixed16. Paired differences
(same positions; floor in pp +-2 SE of the paired difference; regret in
deals/decision +-2 SE):

| sequential - fixed | match | floor | regret |
|---|---|---:|---:|
| seq4x16m2 - fixed24 | time | +0.4 pp +-6.5 | +0.00 +-0.21 |
| seq4x32m2 - fixed48 | time | +3.7 pp +-6.9 | +0.15 +-0.22 |
| seq4x16m2 - fixed8 | worlds | +5.5 pp +-6.2 | +0.07 +-0.22 |
| seq4x32m2 - fixed16 | worlds | +3.3 pp +-6.4 | +0.13 +-0.21 |
| seq4x32m2 - fixed24 | (24 worlds, half the time) | -3.3 pp +-6.8 | -0.04 +-0.21 |

Fixed grid against fixed8: fixed16 -1.5 pp +-6.2, fixed24 +5.1 +-6.5, fixed32
+1.1 +-6.4, fixed48 -1.8 +-6.4 (regret -0.10, +0.07, -0.08, -0.12, each +-0.2).

What it shows:
- No sequential variant beats its cost-matched fixed variant. On time the
  differences are +0.4 and +3.7 pp (wrong or zero sign, inside noise); on
  worlds they are +5.5 and +3.3 pp (wrong sign, inside noise). The one negative
  difference, seq4x32m2 vs fixed24 (-3.3 pp), is neither a worlds match nor a
  time match, is inside its +-6.8, and rests on fixed24 being the high point of
  the fixed grid.
- The fixed floor is flat from 8 to 48 worlds per modeled decision: 30.0, 28.6,
  35.2, 31.1, 28.2%, with no trend (fixed48 - fixed8 = -1.8 pp +-6.4; regret
  -0.12 +-0.21). Six times the worlds buys nothing detectable, so there is no
  gradient for an allocation rule to improve. fixed24's 35% is a bump, not a
  trend (fixed24 - fixed16 is +6.6 pp +-5.9; fixed48 - fixed24 is -7.0 +-6.8 and
  seq4x16m2 - fixed16 is +7.0 +-6.5 are also past 2 SE, and the bump reverses by
  fixed32/48), which also says the binomial interval is optimistic.
- Flips remain costly: at every cost 39-53% of flips lose 3+ deals and none of
  the flips is a zero-regret tie. The floor is therefore not a function of the
  modeled mind's world count in this range.
- The sequential rule is a time loss without a quality gain: it re-solves the
  joint problem after every block (about 2.6-4.4x the time per world).

Caveats: 273 decisions from 16 games; clustering by game gives 2-SE of 4.0-6.5
pp for the floor (fixed8 smaller, fixed16/24 as large as 6.5), so differences
under ~7 pp, paired, are not detectable and a true sequential gain of a few pp
is not excluded; the result is a null at that resolution. One margin (2) and
one block size (4) only. fixed48's (30,80) and (110,100) chunks were issued
concurrently and shared the 4 cores; its 0.056 ms/world is the lowest on the
grid, so its time looks unaffected. Cross-regret is scored from 30-deal option
values that carry their own noise. Time includes bookkeeping, measured on 4
cores with Pool(4). Round 2: settled as no evidence of benefit; rounds are not
needed unless a lower-overhead sequential implementation is wanted.

Skeptic's review (adversarial pass, same session): data and numbers reproduce
bit-for-bit from an instrumented re-run; modeled seats sample only from their
own chair; each look is one joint solve. Objections to the framing, recorded
verbatim in substance: (1) cluster-robust 2-SE on the key paired comparisons
is 7-9 pp, not 6-7; (2) the rule tested is miscalibrated — at block 4 with an
absolute margin of 2, ~39% of modeled decisions stop at 4 worlds and a similar
share run to the cap, so the null is for this loose rule, not sequential
sampling as such; (3) each look re-rolls the dice and re-solves from scratch,
so the 2.6-4.4x time overhead is implementation-specific; the world-matched
comparison is the implementation-independent one (also no gain); (4) 15 of the
273 positions involve no modeled decision at all; (5) the floor is a
self-referential proxy, not play quality: "no quality gain" should read "no
floor/cross-regret gain". What survives all of it: the fixed floor is flat in
the modeled minds' world count from 8 to 48, as it was flat in root deals from
15 to 60. The noise is not in either sample size.
