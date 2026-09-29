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
