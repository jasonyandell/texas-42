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
