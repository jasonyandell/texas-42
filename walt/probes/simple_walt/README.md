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
