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

## Which random source carries the floor? (`noise_source.py`, 2026-10-01)

EXPLORATORY. Question: the level-1 player's root move changes in about 30% of
decisions when only the inner seed changes. Inside its model of the other seats
there are two random sources, (S) the shuffles that decide which worlds a
modeled level-0 mind imagines from its own chair, and (D) the dice that decide
which legal tile each non-mind seat plays in each imagined world. Which one
carries the floor, and do they add?

Setup: the same 273 frozen positions (16 games, seeds 4000-4015), fixed n0=8,
30 own-seat root deals drawn from the same `Random(base_of(seed, state))` in
every arm and in both halves of every pair. Here S and D are separate keyed
streams (pure functions of a base and a context; no running rng), so reseeding
one base leaves the other source bit-identical. Four arms, each an A/B pair of
independent evaluations: `both` reseeds S and D, `shuffle` reseeds S only,
`dice` reseeds D only, `none` reseeds nothing (a determinism check; the pair
really re-runs). Noise floor = A and B pick different tiles. Cross-regret as in
rounds 1-2: for a flip, the larger of what either pick loses under the other's
option values (deals of 30), averaged over ALL decisions, per-flip mean in
brackets. Numbers re-derived from the four `noise_*.jsonl` files (all 273
unique rows, same position set, seed/seat identical to `seq_positions.jsonl`),
not from the run summaries. Intervals are 2 SE: binomial (percentage points),
and cluster-by-game on the `seed` field (16 clusters of 6-22 decisions, ratio
estimator with the G/(G-1) correction). Regret interval is +-2 SE of the mean
over decisions.

| arm | decisions | floor (flips) | 2-SE binomial | 2-SE by game | cross-regret / decision +-2 SE [per flip] |
|---|---:|---:|---:|---:|---:|
| both (S+D) | 273 | 32.60% (89) | +-5.67 | +-3.84 | 0.81 +-0.18 [2.47] |
| shuffle (S) | 273 | 33.33% (91) | +-5.71 | +-7.05 | 0.82 +-0.18 [2.46] |
| dice (D) | 273 | 35.53% (97) | +-5.79 | +-6.79 | 0.93 +-0.19 [2.62] |
| none | 273 | 0.00% (0) | degenerate (0) | degenerate (0) | 0.00 [0.00] |

Paired differences (same positions, per-position differences; floor in pp,
regret in deals/decision; +-2 SE iid / by game):

| contrast | floor | regret |
|---|---|---|
| shuffle - none | +33.3 +-5.7 / +-7.1 | +0.82 +-0.18 / +-0.18 |
| dice - none | +35.5 +-5.8 / +-6.8 | +0.93 +-0.19 / +-0.24 |
| dice - shuffle | +2.2 +-6.1 / +-6.0 | +0.11 +-0.21 / +-0.22 |
| both - shuffle | -0.7 +-6.1 / +-6.6 | -0.01 +-0.22 / +-0.20 |
| both - dice | -2.9 +-5.8 / +-6.8 | -0.12 +-0.22 / +-0.26 |
| both - shuffle - dice (additivity) | -36.3 +-8.2 / +-11.6 | n/a |

Overlap of the flipped positions (273 positions): shuffle flips 91, dice flips
97, 60 flip under both single sources (32.3 expected if independent; chi2 = 55
on 1 df), 128 under at least one; of the 89 `both` flips, 75 are in that union
and 14 are not. Flip losses: 3+ deals in 34/89 (both), 37/91 (shuffle), 45/97
(dice) flips; no flip in any arm is a zero-regret tie. Evaluation cost 2.23
s/evaluation in the three active arms, 2.18 in `none` (the same code path
without a reseed; A and B times match to a few ms, all rows).

What it shows:
- Either source alone produces the whole floor. Shuffle-only 33.3% and dice-only
  35.5% against both 32.6%; every pairwise difference is inside 1 SE of its
  paired interval (the largest, dice - both, is +2.9 pp against +-5.8 iid /
  +-6.8 by game). Neither source carries it "more": dice - shuffle is +2.2 pp
  +-6.0, and the 0.93 vs 0.82 regret is +0.11 +-0.21. The point estimates order
  dice > shuffle > both, but that order is not a result.
- The two sources are not additive; they saturate. Adding the single-source
  floors predicts 68.9% against the observed 32.6% (-36.3 pp, 8.2 iid / 11.6 by
  game past additive), and even the independent-union prediction 1-(1-p_S)(1-p_D)
  = 57.0% is 24 pp above observed. The same arithmetic on regret gives 478 deals
  (shuffle + dice) against 220 (both). Reseeding both together does not add
  flips beyond reseeding one.
- The flips concentrate on a shared set of positions: 60 of the 91 shuffle flips
  also flip under dice-only (about twice the 32 expected under independence).
  But the sets are not identical (31 flip only under shuffle, 37 only under
  dice, 14 `both` flips under neither), so "a fixed fragile set that anything
  tips" is too strong; what the data support is a position-dependent flip
  probability that is high for many positions and roughly the same whichever
  source is reseeded.
- `none` is exactly 0, and not only in the picks: for all 273 positions A and B
  agree on the pick AND on every recorded option value (`A_made == B_made`,
  273/273). The player is a deterministic function of (position, root deals, S
  base, D base): there is no hidden fifth source (clock, hash seed, process
  scheduling, pool order). That is a determinism proof for this code path on
  these positions, a different kind of statement from the nonzero rows, whose
  binomial interval is not meaningful at 0/273 (it is a structural zero, not an
  estimated rate). Also checked: in every arm, A's picks and values are
  identical to `both`'s A (shared s1, d1), so the arms differ only in B.
- The cost of a flip is the same everywhere: 2.46-2.62 deals per flip, no
  zero-regret flips, 38-46% of flips lose 3+ deals. The floor is not made
  harmless by the source that produces it.

Caveats: 273 decisions from 16 games. The cluster-by-game 2 SE is not stable at
16 clusters of unequal size (6 to 22 decisions): `both` gets +-3.8 pp but
`shuffle` and `dice` +-7.0 and +-6.8, and the binomial figure sits between;
read the resolution as roughly +-6-7 pp, so differences under ~7 pp paired (all
contrasts among the three active arms) are not detectable, and a modest
true difference between S and D (or a modest additivity from S+D) cannot be
excluded. The additivity gap (-36.3 pp) is the exception: it is far outside
every interval, but it measures sub-additivity of flip probabilities, which is
automatic once single-source floors exceed 33% (they cannot add past 100%, and
a union of independent effects would still be 57%), so "saturating" is the
finding, not "S and D interact". Own-seat root deals are held fixed and
identical in every arm, so this decomposes only the inner (modeled-seat)
floor; the root-deal contribution is in `crn15/crn/crn60.jsonl` (flat from 15
to 60 deals). Dice are keyed by (dice base, world id, seat, public record) with
the world id a context hash, so "reseeding D" here means a different keyed
table, not a different draw order; it is not claimed to match the running-rng
floor of rounds 1-2 beyond the observed agreement (30.0% fixed8 there, 32.6-
35.5% here, same positions, different generator). Cross-regret is scored from
30-deal option values that carry their own noise. Timings come from four
separate foreground runs of the arm, each in two chunks, on 4 cores with
Pool(4); the per-row seconds reproduce the reported chunk walls (per-arm
CPU-seconds / 4 = 105 s and 199 s for chunks 0-69 and 70-272, against 111 s
and 201 s reported, 108 s and 197 s for `none`).

Skeptic's review (adversarial pass, same session). Numbers reproduce exactly
(16 rows re-run bit-for-bit; A sides identical across arms; `none` 273/273
identical picks and values). Two corrections and one refutation: (1) "A and B
times match to a few ms" is false (185/273 rows differ by >0.01 s, max 1.04 s);
determinism holds for picks and values, not timings. (2) "sub-additivity is
automatic above 33%" is wrong: additive predicts 68.9%, independent union 57%,
observed 32.6% is far below both. (3) DESIGN CONFOUND: the dice draw is a pure
function of (dice base, world slot, seat, record), but the realized tile is
`legal[u % len(legal)]` and `legal` is that slot's hand in the world, so
reseeding the SHUFFLES re-maps ~75% of the opponents' realized plays (dice
reseed alone changes ~43%). The `shuffle` arm is therefore "S plus most of D's
realized effect", the near-equality of `shuffle` and `both` is mechanical, and
"either source alone carries the floor" is not measured. What survives: the
`dice` arm is clean (worlds fixed) and reproduces the whole floor, 35.5%. A
true S-only arm needs dice keyed by the hand that plays, not the slot.

## Clean shuffle-only noise, and does K average the floor away? (`dice_noise.py`, 2026-10-01)

EXPLORATORY. Follow-up to `noise_source.py` and its skeptic's review. Two
questions. Q1: with the dice keyed so that a shuffle reseed cannot re-map the
realized plays (the design confound the skeptic found), does shuffle-only noise
still produce a floor, and how big is it against dice-only? Q2: if each modeled
mind imagines every world K times (K independent dice tapes) and best-responds
to the 8K copies, does the floor fall with K = 1, 4, 16 under `both`?

Setup: same 273 frozen positions (16 games, seeds 4000-4015), fixed n0=8 worlds
per modeled mind, same 30 own-seat root deals in every arm and in both halves of
every pair. Dice are now keyed by (dice base, tape index, acting seat, that
seat's REMAINING HAND in the world, public record), never by world slot;
shuffles by (shuffle base, seat, hand, record) as before. Arms, each an A/B pair
of independent evaluations (s, d = seed index): `shuffle_k1` A=(s1,d1) B=(s2,d1);
`dice_k1` A=(s1,d1) B=(s1,d2); `both_k1/k4/k16` A=(s1,d1) B=(s2,d2). Floor and
cross-regret as in earlier sections (flip = A and B pick different tiles;
cross-regret = for a flip the larger loss of either pick under the other's
option values, in deals of 30, averaged over ALL decisions, per-flip mean in
brackets). Numbers re-derived from the five `dice_*.jsonl` files, not from the
run summaries (those agree for the four complete arms). Intervals are 2 SE:
binomial (pp) and cluster-by-game on `seed` (ratio estimator, G/(G-1)
correction). Timings are s/evaluation from the per-row seconds.

STATE OF `both_k16`: INCOMPLETE. A single K=16 evaluation of an opening position
takes over 570 s, so the arm was finished by a detached job outside the 600 s
rule, and that job is still appending to `dice_both_k16.jsonl` as this is
written. Everything about K=16 below is computed from the FIRST 176 LINES of the
file (175 unique positions: 0-175 minus position 174; position 77 appears twice
with identical picks and values, last row kept), which are games 4000-4009 whole
plus one position of 4010, an index prefix and not a random sample. K=16 is
compared to the other arms only on those 175 positions ("matched" rows).
Positions 176-272 (and 174) are unmeasured at K=16.

| arm | decisions | floor (flips) | 2-SE binomial | 2-SE by game | cross-regret / decision +-2 SE [per flip] | s/evaluation |
|---|---:|---:|---:|---:|---:|---:|
| shuffle_k1 (S only) | 273 | 28.2% (77) | +-5.4 | +-5.5 | 0.73 +-0.17 [2.58] | 2.40 |
| dice_k1 (D only) | 273 | 29.3% (80) | +-5.5 | +-3.2 | 0.88 +-0.20 [2.99] | 2.48 |
| both_k1 | 273 | 32.2% (88) | +-5.7 | +-4.9 | 0.79 +-0.17 [2.45] | 2.46 |
| both_k4 | 273 | 29.7% (81) | +-5.5 | +-5.8 | 0.71 +-0.17 [2.40] | 10.16 |
| matched: shuffle_k1 | 175 | 30.3% (53) | +-6.9 | +-6.1 | 0.72 +-0.21 [2.38] | 2.48 |
| matched: dice_k1 | 175 | 28.0% (49) | +-6.8 | +-4.4 | 0.78 +-0.22 [2.80] | 2.64 |
| matched: both_k1 | 175 | 33.1% (58) | +-7.1 | +-5.7 | 0.78 +-0.21 [2.34] | 2.50 |
| matched: both_k4 | 175 | 29.7% (52) | +-6.9 | +-6.7 | 0.66 +-0.18 [2.21] | 10.79 |
| matched: both_k16 | 175 | 33.1% (58) | +-7.1 | +-5.9 | 0.78 +-0.21 [2.34] | 42.89 |

World-copies per modeled decision: 8 (K=1), 32 (K=4), 128 (K=16); modeled
decisions per evaluation 2440-2451 (K=1, 273 positions), 2630 (K=4), 2815
(K=16, matched positions; the matched K=1 arms show 2473-2495 there).

Paired differences (same positions, per-position differences; floor in pp,
regret in deals/decision; +-2 SE iid / by game; time ratio in brackets):

| contrast | positions | floor | regret |
|---|---:|---|---|
| shuffle_k1 - dice_k1 | 273 | -1.1 +-5.9 / +-5.9 | -0.15 +-0.20 / +-0.17 |
| both_k1 - shuffle_k1 | 273 | +4.0 +-6.2 / +-6.2 | +0.06 +-0.19 / +-0.17 |
| both_k1 - dice_k1 | 273 | +2.9 +-6.3 / +-3.9 | -0.08 +-0.19 / +-0.12 |
| both_k4 - both_k1 [4.1x] | 273 | -2.6 +-6.8 / +-6.8 | -0.08 +-0.21 / +-0.15 |
| both_k4 - both_k1 [4.3x] | 175 | -3.4 +-8.7 / +-10.2 | -0.12 +-0.25 / +-0.23 |
| both_k16 - both_k1 [17.1x] | 175 | +0.0 +-8.3 / +-6.8 | +0.00 +-0.27 / +-0.22 |
| both_k16 - both_k4 [4.0x] | 175 | +3.4 +-7.9 / +-8.9 | +0.12 +-0.24 / +-0.25 |

Other re-derived quantities (matched 175 unless stated). Mean |A - B| of the
root's option values, per option, in deals of 30: 1.13 (K=1), 1.15 (K=4), 1.08
(K=16); 273 positions: 1.17 (shuffle), 1.13 (dice), 1.18 (both K=1), 1.20 (K=4).
Flipped positions: shuffle and dice flips overlap on 46 of 77 / 80 (22.6
expected if independent; union 111; 273 positions). Across K: 17 positions flip
at all three K (5.7 expected if independent), 96 flip at some K; both_k1 and
both_k16 flips overlap on 32 of 58 / 58 (19.2 expected). No flip in any arm is
a zero-regret tie; flips losing 3+ deals: 34/77, 46/80, 36/88, 31/81 (K=4),
22/58 (K=16 matched). Flip rate by the root's top-two option-value gap in A
(matched `both_k16`): gap 0: 29/81, 1: 6/17, 2: 2/7, 3-4: 21/59, 5+: 0/11;
`both_k1` and `both_k4` have the same shape (flips at gap 3-4 are 23/62 and
25/63).

Integrity checks run: A sides of `shuffle_k1`, `dice_k1`, `both_k1` are
identical (pick and all option values) on 273/273 positions, so the arms differ
only in B; seed and seat of every row equal `seq_positions.jsonl`; the
duplicated K=16 row for position 77 reproduces picks and values bit-for-bit
(timings differ, 51.9/53.7 vs 52.8/50.8 s). `dice_noise.py check --positions 200
240` (run here at K=1 and K=4): under a shuffle reseed every shared
(tape, seat, hand, record) dice site realized the identical tile (28,504 of
28,504 shared sites at position 200 K=1; 84,190 of 84,190 at position 240 K=4),
no within-run conflicts, and the check printed `check ok`; under a dice reseed
only 68-70% of shared sites agree. The shared sites are only about 1-3% of all
sites under a shuffle reseed (hands differ in different worlds), so see the
caveat on what "shuffle-only" means.

What it shows:
- Q1: yes, the clean shuffle-only arm shows a floor, and it is the same size as
  dice-only. Shuffle-only 28.2% (77/273, binomial 2-SE +-5.4, so the interval
  excludes 0 by a wide margin) against dice-only 29.3% (80/273): shuffle - dice
  is -1.1 pp +-5.9, regret 0.73 vs 0.88 (-0.15 +-0.20). Ratio of flips 77:80.
  Neither is larger at this resolution, and both are within the +-6 pp pair
  resolution of `both` (32.2%). This replaces the confounded 33.3% shuffle
  figure of `noise_source.py`: with the dice table held fixed (the shared-site
  check) the floor is 28.2%, not lower than the confounded figure by any amount
  we can detect (-5.1 pp from 33.3 is inside the interval).
- The sources saturate rather than add: S + D predicts 57.5%, the independent
  union 49.2%, observed `both` is 32.2%. The flipped sets overlap about twice
  chance (46 vs 22.6 expected) but are not identical (31 shuffle-only flips, 34
  dice-only, 111 in the union).
- Q2: the floor does not fall with K. 32.2% (K=1), 29.7% (K=4) on all 273;
  matched 175: 33.1%, 29.7%, 33.1% for K = 1, 4, 16. Paired: K=4 - K=1 is -2.6
  pp +-6.8 (273) and -3.4 +-8.7 / +-10.2 (175); K=16 - K=1 is +0.0 +-8.3 / +-6.8;
  K=16 - K=4 is +3.4 +-7.9 / +-8.9. Regret is flat too (0.78, 0.66, 0.78; K=16 -
  K=1 +0.00 +-0.27). The data exclude a K=1-to-K=16 drop larger than about 8 pp
  (the lower end of the iid interval); they cannot exclude a few pp. A floor
  that shrank toward zero would have needed -33 pp.
- Cost of K: 4.3x the time per evaluation at K=4 (10.79 vs 2.50 s) and 17.1x at
  K=16 (42.89 vs 2.50 s) on the matched positions, for a K=16 - K=1 floor
  difference of +0.0 pp.
- The values do not average out either, not just the picks: the root's option
  values move by 1.13, 1.15, 1.08 deals per option between independent
  evaluations at K = 1, 4, 16, with 16 times the dice copies. The floor sits on
  a shared fragile set of positions (17 flip at all three K against 5.7 by
  chance) and flips are not concentrated where the root's own top-two gap is
  small (gap 3-4 flips 21/59 at K=16, gap 0 29/81), so it is not just root
  near-ties.
- What the flat K implies (interpretation, not a measurement): K multiplies the
  dice tapes over the SAME 8 imagined worlds. It averages D but never touches
  S. The floor is flat in K and the shuffle-only arm already carries it at K=1,
  so K could not have removed it by construction unless D were the only source;
  the clean S-only floor of 28.2% says D is not the only source. Together with
  `seq_experiment.py` round 2 (floor flat when the modeled mind's worlds go 8 to
  48, a different generator, same positions), this points to the modeled minds'
  noisy picks not averaging out under either extra dice copies or extra worlds:
  the root's sensitivity is to WHICH near-equivalent tile a modeled seat plays,
  and where a modeled mind is genuinely indifferent between tiles no number of
  playouts K settles the choice. That reading is consistent with the data and is
  NOT confirmed by it. A competing reading that the data do not exclude: the
  modeled mind's value estimate is an 8-world sample and its pick is sampling
  noise of those 8 worlds that K cannot reduce (only n0 can); the two readings
  differ in whether the population gap between the top two tiles is zero or
  merely small.
- Proposed diagnostic (one, not run; it needs K=16 and is the expensive part):
  instrument `Walt.model` to log, for every modeled decision with 2+ legal tiles,
  the top-two gap of its `made` counts expressed in worlds (gap in copies divided
  by K), at K=16 on a fixed subset (for example the shortest 40 positions,
  which finish in minutes). Report the fraction with gap exactly 0 (a tie, which
  `max`/`min` break deterministically by tile order and so is not a noise
  source), the fraction with 0 < gap <= 1 world, and the fraction with gap > 1
  world, at K=4 and K=16. Confirmation of the indifference reading: a large
  fraction at gap <= 1 world that does not shrink from K=4 to K=16 (the gap in
  worlds converges to the 8-world gap, not to 0). Against it: mass moving to
  gap > 1 world at K=16 while the flips stay put, which would point at the
  8-world sampling reading instead. If the fraction at gap <= 1 world is large,
  the follow-up that separates population indifference from 8-world noise is the
  same instrument at n0 = 32, K=4 (same 128 copies, 4x the worlds).

Caveats: 273 decisions from 16 games for K = 1 and 4; K=16 rests on 175
decisions from 11 games (10 whole), an index prefix that is the early part of
each of the earliest games, not a sample of all 16; the matched K=1 and K=4
floors (33.1%, 29.7%) agree with their 273-position values (32.2%, 29.7%), so
the prefix does not look unusual, but that is not a test. The cluster-by-game
2 SE is unstable at 11-16 clusters of unequal size (dice_k1 +-3.2 pp against a
binomial +-5.5; K=4 matched +-6.7 against +-6.9), so read the resolution as
roughly +-6-9 pp for paired contrasts and do not lean on the small cluster
figures. "Shuffle-only" means the dice TABLE is held fixed, not that realized
plays are: only about 1-3% of dice sites are shared between a shuffle reseed's two
halves, because different worlds give different hands, so most realized
opponent plays are fresh draws in a shuffle reseed, as they must be in any new
world; the clean statement is "a floor exists without reseeding the dice table",
not "without re-rolling any realized play". Dice are keyed differently from
`noise_source.py` (hand, not slot), so the dice-only floor here (29.3%) is not
the 35.5% there; the 6 pp gap is inside noise but the keys are not identical.
The gap-by-flip table uses A's root values only and 30-deal values carry their
own noise. Cross-regret is scored from 30-deal option values. Timings come from
foreground chunks on 4 cores with Pool(4); K=16 rows were produced by a detached
job and the `check` run overlapped it for part of its time, so K=16 seconds may
include contention (duplicate rows differ by about 1-2 s). The K=16 job was
left running by the earlier worker and was not started or stopped here; K=16
numbers are pinned to the first 176 lines (the file is append-only), and a
complete K=16 arm needs the run to finish and `summarize both_k16` to be
re-derived.

Completion note: the K=16 arm finished after the analysis above was written
(a worker left it running detached, against the no-background rule; it was
waited for in the foreground and one duplicate row for position 77, identical
picks and values, was removed). Full-set K=16: 92/273 = 33.7% flips,
cross-regret 0.82 deals/decision (2.42 per flip), 46.7 s/evaluation (CPU-
contended). Floor in K over all 273 positions: K=1 32.2%, K=4 29.7%, K=16
33.7%. Flat.

Skeptic's review (adversarial pass, same session). Mechanics hold: hand-keyed
dice realize the same tile at the same (tape, seat, hand, record) under a
shuffle reseed; the K copies have independent tapes and one joint solve; the
root is unchanged; every number re-derives from the files. Refuted or
corrected: (1) the gap-by-flip table in the section above does not reproduce,
and under its stated definition flips DO concentrate at small root top-two gaps
at K=16 (41% at gap 0-1 vs 16% at gap >=2); (2) the "shared fragile set"
overlap baselines assume independent arms, but all K arms share the shuffle
streams and the A side, so excess overlap is built in; (3) Q2 as run was
circular: the K arms reseed both sources, and with a 28% shuffle-only floor
already present a flat `both` floor in K could not show dice averaging out.
The skeptic ran the missing dice-only K arms on the cheapest 150 (late-game)
positions: dice_k1 19.3%, dice_k4 16.7%, dice_k16 16.7% (K16-K1 -2.7 +-7.3 pp;
full averaging would predict about -19), and the proposed gap diagnostic on
the cheapest 100: the modeled minds' top-two gap is <= 1 world in 74.6 /
74.6 / 75.4% of modeled decisions at K = 1 / 4 / 16, with no mass moving to
> 1 world. Both support the indifference reading on that subset; opening
positions at K=16 dice-only remain untested. Also noted: "no flip is a
zero-regret tie" is a tautology of the first-max tie-break; the hand-keying
artifact (1-3% shared sites, ~7.5 effective worlds) does not move the floor
in a slot-keyed control; K=16 timings are CPU-contended.
