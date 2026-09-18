# First Kiln calibration: sixes, bid 36

Frozen model forecast: **121/160 = 75.625%**. Observed result: **21 makes in
100 fresh hidden completions**, Wilson 95% interval **14.17–29.98%**.
This score substantially overestimated success against the executed table players.
It is not a calibrated probability of winning that match-up.

The fixed own hand was `[0,3,13,21,22,23,26]`, seat 3. The case was selected
before playing any outcomes: closest to .78 among six initial .75-at-eight
candidates re-evaluated at 160 worlds. The selection and all six receipts are
preserved. This is one hand, not an aggregate calibration curve or a general
estimate of player strength.

The auction score solves an L1 response against a modeled sigma0 field. The
executed games used all four deployed L1+partner players: 40/8, a 160-world
bidder opening without partner review, 14-second ordinary budgets and 20-second
opening budgets. Each seat replans from its own hand and public record. This
policy mismatch is a candidate explanation, not a cause established by the
experiment. Neither hidden-information legality nor correct rule execution makes
the model's field predictive of this opponent population.

There are 100 independent seeded uniform unseen-hand completions with the
bidder's own hand held fixed. Each actor receives only its original seven tiles,
public record, contract, seat, and policy seed. No actual other hands reach Walt.
A game stops when making or setting the contract is irrevocable. All saved moves
were checked while running against the independent Python rules. A second audit
replayed **1,808 decisions through Plunge's actual game engine**, verifying each
own-hand request, public prefix, legal choice, leader, score and final outcome.
All 100 matched. The audit does not establish equivalence between modeled and
executed continuation policies.

Evidence outside Git:
`/Users/jason/data/texas-42/kiln-v1/calibration-six36/` contains case.json,
100 per-game receipts, status.json and plunge-audit.json. Selection evidence is
in the sibling calibration-prices directory. Compact evidence is committed in
calibration-six36-summary.json.

Case ID: `ecd47c3c40f1f7b8e5a942291911314cb22f213d85fc7526538c3b6210e72b1e`.
Ordered game-file digest:
`b69ab2b0f11bb1d35cc78ed30ca18835e9b6dc425e122f89f73d2217c71c3c9d`.

Reproduce the cross-engine audit in plunge-sunshine:

```sh
KILN_CALIBRATION_DIR=/Users/jason/data/texas-42/kiln-v1/calibration-six36 \
  npx vitest run tests/kiln-calibration-audit.test.ts
```

Kiln will preserve these values as model scores. Its first game integration
retains the current minimum-raise and partner-pass auction policy; it does not
turn this uncalibrated panel into a new maximum-bid policy. Future work can
separately measure the modeled-field match-up, actual continuation match-ups,
and additional hands before revising the model or its bid threshold.

## Diagnostic follow-up: what is and is not explained

Inspection of the saved 100 games found that their independently seeded opening
evaluations averaged **76.7125%** for the move actually chosen (range 70–85.625%).
Tile 0 was chosen 53 times (10 makes, mean model score 76.10%); tile 13 was
chosen 47 times (11 makes, mean model score 77.41%). Thus the discrepancy is
present across repeated forecasts for this hand, not just the originally
selected 121/160 receipt. These remain one-hand observations.

All 100 opening evaluations completed 160 worlds; all 1,195 non-opening
evaluated decisions completed 40 worlds; the remaining 513 moves were forced.
None of the 1,808 decisions reported a budget overrun. The partnership review
changed exactly one move, in game 39 at ply 22 (that game made). It made no
choice changes on the other 99 observed trajectories. Deadline truncation and
frequent partner overrides therefore do not describe this execution.

Two structural mismatches are confirmed by the source:

- Auction pricing uses `Field::SeatLevels([0; 4])`. These modeled level-0
  policies optimize against Dice on eight inner worlds; they are not literal
  random players. Actual seats used the deployed L1 player and partner review.
- The forecast optimizes the bidder's future choices over its original sampled
  worlds. As modeled seats act, `combine_buckets` conditions those worlds on
  their policy-dependent moves. Actual play samples anew on each decision:
  `partnership::evaluate` conditions the outer sample on legality and voids,
  without carrying the forecast's policy-conditioned posterior or contingent
  plan. SCENARIO-PLAYER.md section 4.4 explicitly records this distinction.

The causal contributions remain unmeasured. Finite-sample plan overfitting,
changed future belief updates/replanning, and changed partner/opponent policies
are separate candidate contributors. First replay held-out completions with
the deployed bidder and the same modeled field, then separately change partner
and opponent policies. To isolate plan fitting from deployed replanning, a
frozen bidder policy must declare its behavior on histories absent from the
training sample before evaluation on held-out completions. This inspection
does not establish that any one change repairs the gap.
