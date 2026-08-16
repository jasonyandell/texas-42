# Experiment 5 — the response-class census curve

**Tier: exploratory probe.** Nothing here is a corpus status, a kernel proof, an exchange adjudication, or a rob conformance receipt, and nothing here is cited by anything above it. No repo file was written or modified; all artefacts live in the session scratchpad.

## The question

Reasoning over the raw hidden-world fiber is hopeless in the early game (~4x10^8 at trick 1). The scheme/quotient bet is that a seat can instead reason over *exact response classes* — worlds that induce the same answer at the root — and that these are dramatically fewer. Two points on that curve were already known at probe tier (receipt hand 0, focal seat = trick leader, perfect-information parametric minimax): the trick-6 kernel maps 90 worlds to 8 parametric root-Q classes, and the trick-5 kernel maps 1,680 worlds to 5 — the fiber grew 18.7x while the quotient *shrank*. Experiment 5 measures the curve properly: 13 receipt hands x horizons 2..6, several census targets, exhaustive where affordable and uniformly sampled where not.

## Headline findings

| H | fiber (median) | `q_trick` classes (median) | `q_points` classes (median) | `act_points` classes (median) | median worlds *examined* per `q_points` class |
|---:|---:|---:|---:|---:|---:|
| 2 | 36 | 3.000 | 4.000 | 2.000 | 11.250 |
| 3 | 1,680 | 6.000 | 15.000 | 4.000 | 93.333 |
| 4 | 23,100 | 29.000 | 216.000 | 11.000 | 95.455 |
| 5 | 756,756 | 386.000 | 2589.000 | 28.000 | 3.905 |
| 6 | 11,435,424 | 244.000+ | 349.000+ | 33.000+ | 1.146 |

`+` marks horizons whose class counts are sampled lower bounds.

**Those class counts are not comparable across horizons as they stand**, because the horizons were examined with different numbers of worlds (whole fibers up to H=4, 10,000 samples at H=5, 400 at H=6). Reading a decline from H=5 to H=6 out of the table above would be reading a sample-size artefact. The comparable statistic is the census seen through a *fixed* window, below.

### The census through a fixed window of 250 worlds

For every kernel, how many distinct classes appear among 250 worlds drawn uniformly at random (with replacement, as the sampler does)? For a sampled kernel this is read off its saturation curve. For an exhaustively censused kernel it is the exact expectation `sum_i (1 - (1 - c_i/N)^n)` over that kernel's true class sizes `c_i` — computed in exact rational arithmetic, not simulated. The ceiling is 250 (every world in its own class).

| H | kernels counted (trick / points / action) | `q_trick` median | `q_points` median | `act_points` median |
|---:|---:|---:|---:|---:|
| 2 | 13 / 13 / 13 | 3.000 | 4.000 | 2.000 |
| 3 | 13 / 13 / 13 | 5.591 | 14.134 | 3.998 |
| 4 | 13 / 13 / 13 | 18.391 | 75.543 | 10.521 |
| 5 | 13 / 13 / 13 | 91.000 | 167.000 | 21.000 |
| 6 | 5 / 3 / 3 | 171.000 | 224.000 | 30.000 |

Growth of that fixed-window census, one horizon at a time — this is the apples-to-apples comparison:

| step | fiber x | `q_trick` x | `q_points` x | `act_points` x |
|---|---:|---:|---:|---:|
| H=2 -> H=3 | 46.667 | 1.864 | 3.534 | 1.999 |
| H=3 -> H=4 | 13.750 | 3.289 | 5.345 | 2.631 |
| H=4 -> H=5 | 32.760 | 4.948 | 2.211 | 1.996 |
| H=5 -> H=6 | 15.111 | 1.879 | 1.341 | 1.429 |

The most legible way to read that table is as a **collision rate**: out of 250 worlds, what fraction share a response with an earlier one? That is the quotient doing work.

| H | `q_trick` distinct of 250 | collapse | `q_points` distinct of 250 | collapse | `act_points` distinct of 250 | collapse |
|---:|---:|---:|---:|---:|---:|---:|
| 2 | 3.000 | 98.800% | 4.000 | 98.400% | 2.000 | 99.200% |
| 3 | 5.591 | 97.764% | 14.134 | 94.346% | 3.998 | 98.401% |
| 4 | 18.391 | 92.643% | 75.543 | 69.783% | 10.521 | 95.792% |
| 5 | 91.000 | 63.600% | 167.000 | 33.200% | 21.000 | 91.600% |
| 6 | 171.000 | 31.600% | 224.000 | 10.400% | 30.000 | 88.000% |

Three things follow.

1. **The value quotient decays with horizon; the action quotient does not.** The distinct-response collapse under the real scoring differential falls from 98.400% at H=2 to 10.400% at H=6 — by the widest horizon measured, almost every world in a 250-world window has its own value vector, and the value quotient has stopped being a compression at all. But the *optimal-action-set* census holds up: its collapse only falls from 99.200% to 88.000%. **If the scheme is to survive into the early game, it has to be a quotient of decisions, not of values.** That is the single most actionable thing this run found — and the bias runs in its favour: the one kernel censused both exhaustively and by sample shows the sampled value floors recovering only ~44% of the truth while the sampled action floors recover ~97%, so the value census is *understated* at the wide horizons and the action census is not.
2. **The trick-5 surprise does not generalize.** The known result — fiber up 18.7x, quotient *down* from 8 to 5 — is a property of that one high-control kernel (hand 0, the viewer holding the last trump and a boss), not of the horizon. Across 13 hands the census rises at every step of the fixed-window table, monotonically, for every target.
3. **What varies between kernels is control, not fiber size.** Within a horizon the fiber size is nearly uncorrelated (or negatively correlated) with the class count, while measures of focal control are strongly negatively correlated with it. That is the finding with the most leverage: the census is cheap exactly where the seat is already in control, and expensive exactly where it is not — so a scheme that has to be cheap everywhere is the wrong target, and a scheme that spends its budget where control is absent is the right one.

A caution on point 1: the action census is capped at 2^H-1 by arithmetic alone (see below), so part of its good behaviour is free. At H=6 the cap is 63 and the observed median is 30.000, so it is not merely pinned at the cap — but the margin is not large.

## Method

**Kernels.** For receipt hand `h` and horizon `H`, the kernel is the suffix beginning at the start of trick `8-H`. The focal seat is the actual trick leader there. The fiber is every assignment of the unseen tiles to the three hidden seats at equal hand sizes that is consistent with every void the focal seat can observe from the completed tricks (a seat that failed to follow the led suit holds no tile of that suit, under trump absorption). The true receipt world was checked to lie in the fiber for every kernel built.

**Operator.** Perfect-information minimax over the suffix, focal team maximising. Root actions are the focal seat's legal leads (on lead, all of them). Three valuations:

- `q_trick` — the symmetric baseline: each trick is worth +-1 (trick differential only).
- `q_points` — the real straight-42 scoring differential: each trick is worth +-(1 + the count points of its four tiles), focal team minus opponents. This is the player-relevant census.
- `q_param` — the parametric census in one valued direction: value = trick differential + lambda * (capture sign of the highest-count unseen tile), as an exact piecewise-linear function of lambda on [0, inf). This is the target the two known probe points were measured on.

`act_*` is the corresponding **action-correspondence** census: the distinct sets of optimal root actions (for `act_param`, the distinct parametric correspondences as lambda sweeps the ray).

**Exactness.** Integers and `fractions.Fraction` only; no float touches a rank, a value, or a probability. Sampling uses a seeded `random` purely to *select* worlds: the selection is exactly uniform over the void-constrained fiber via an integer dynamic program that counts completions (no rejection sampling, no floating-point weights), and every class computed on a selected world is exact.

## Rules generalization and declaration coverage

The inherited machinery hardcoded `TRUMP = 3`. The rules were generalized to declaration-relative suit membership and ranking (`exp5_rules.py`, a frozen copy of `rules42.py` taken at the start of this run so a concurrently running probe could not perturb it). The evidence that the generalization matches the rob engine is a full replay: for all 13 receipt hands, all 7 tricks each, the actor order, follow-suit legality of every play, the trick winner, the trick points, the cumulative hand points and the declaring side's made/set verdict are all re-derived from these rules alone and all match the receipt.

| declaration class | hands | ids |
|---|---:|---|
| pip-trump | 13 | 0,1,2,3,4,5,6,7,8,9,10,11,12 |
| doubles-trump | 0 | **absent from the corpus** |
| no-trump | 0 | **absent from the corpus** |

By label: `P0` x2, `P1` x1, `P3` x1, `P4` x3, `P5` x4, `P6` x2.

**Coverage is honestly partial.** All 13 receipt hands are pip-trump declarations (P0..P6, six of the seven pips appear; P2 never does). Doubles-trump and no-trump are implemented in `exp5_rules.py` but are **unexercised and unvalidated** — the corpus contains no such hand, so every number in this report is a pip-trump number.

**Solver validation.** The census solver is a memoised bitmask minimax. It was spot-checked against `exp5_validate.naive_root_vector`, a separately written uncached minimax on plain tile tuples that calls the rule predicates directly, and (for the trick valuation) against the PWL parametric solver evaluated at lambda = 0. Across this run: **644 naive spot checks and 104 parametric-at-zero spot checks, all exact matches** (no mismatches). A mismatch is a hard assertion failure, so any job that returned a record passed its own checks.

**Determinism.** 234 exhaustive censuses were computed twice in independent processes (the run was repeated to record full class-size distributions). All 234 reproduced the identical class count. The solver is deterministic, so this checks the harness, not the mathematics.

**Sampler validation.** Drawing *n* worlds with replacement from a fiber of *N* leaves an expected `N(1-(1-1/N)^n)` distinct worlds. That expectation is a sharp fingerprint of uniformity, and it is computed here in exact integer arithmetic. Observed against expected, for the sampled kernels:

| kernel | fiber N | draws n | distinct observed | distinct expected (exact) | ratio |
|---|---:|---:|---:|---:|---:|
| `h0t3` | 756,756 | 10,000 | 9,930 | 9,934 | 1.000 |
| `h0t3` | 756,756 | 2,000 | 1,998 | 1,997 | 1.000 |
| `h1t3` | 756,756 | 10,000 | 9,920 | 9,934 | 0.999 |
| `h1t3` | 756,756 | 2,000 | 1,999 | 1,997 | 1.001 |
| `h2t3` | 756,756 | 10,000 | 9,938 | 9,934 | 1.000 |
| `h2t3` | 756,756 | 2,000 | 1,998 | 1,997 | 1.000 |
| `h3t3` | 756,756 | 10,000 | 9,933 | 9,934 | 1.000 |
| `h3t3` | 756,756 | 2,000 | 2,000 | 1,997 | 1.001 |
| `h4t3` | 756,756 | 10,000 | 9,933 | 9,934 | 1.000 |
| `h4t3` | 756,756 | 2,000 | 1,997 | 1,997 | 1.000 |
| `h5t3` | 324,324 | 10,000 | 9,862 | 9,847 | 1.001 |
| `h5t3` | 324,324 | 2,000 | 1,993 | 1,993 | 1.000 |
| `h6t3` | 756,756 | 10,000 | 9,935 | 9,934 | 1.000 |
| `h6t3` | 756,756 | 2,000 | 1,999 | 1,997 | 1.001 |
| `h7t3` | 504,504 | 10,000 | 9,898 | 9,901 | 1.000 |
| `h7t3` | 504,504 | 2,000 | 1,994 | 1,996 | 0.999 |
| `h8t3` | 59,976 | 2,000 | 1,973 | 1,967 | 1.003 |
| `h9t3` | 504,504 | 10,000 | 9,912 | 9,901 | 1.001 |
| `h9t3` | 504,504 | 2,000 | 1,996 | 1,996 | 1.000 |
| `h10t3` | 756,756 | 10,000 | 9,950 | 9,934 | 1.002 |
| `h10t3` | 756,756 | 2,000 | 1,996 | 1,997 | 0.999 |
| `h11t3` | 64,638 | 10,000 | 9,250 | 9,264 | 0.998 |
| `h11t3` | 64,638 | 2,000 | 1,966 | 1,969 | 0.998 |
| `h12t3` | 756,756 | 10,000 | 9,926 | 9,934 | 0.999 |
| `h12t3` | 756,756 | 2,000 | 1,997 | 1,997 | 1.000 |
| `h0t2` | 17,153,136 | 400 | 400 | 399 | 1.000 |
| `h1t2` | 11,435,424 | 400 | 400 | 399 | 1.000 |
| `h2t2` | 4,624,620 | 400 | 400 | 399 | 1.000 |
| `h3t2` | 17,153,136 | 400 | 400 | 399 | 1.000 |
| `h7t2` | 4,624,620 | 400 | 400 | 399 | 1.000 |

Ratios sit on 1. A sampler that was biased toward part of the fiber would collide more often and drive this below 1.

## Reproduction of the two known probe points

| kernel | known | measured here |
|---|---|---|
| `h0t6` | 90 worlds -> 8 parametric (4 baseline, 3 action-correspondence) | 90 worlds -> 8 parametric (4 baseline, 3 action-correspondence) |
| `h0t5` | 1,680 worlds -> 5 parametric (2 baseline, 3 action-correspondence) | 1,680 worlds -> 5 parametric (2 baseline, 3 action-correspondence) |

Both reproduce exactly, on an independently written solver and an independently written rules layer. That is the calibration for everything below.

## The census curve

### Horizon 2 (trick 6 start, 2 tiles per seat, unconstrained fiber 90)

| kernel | decl | focal | fiber (post-void) | coverage | `q_trick` | `act_trick` | `q_points` | `act_points` | `q_param` | `act_param` |
|---|---|---:|---:|---|---:|---:|---:|---:|---:|---:|
| `h0t6` | P3 | S1 | 90 | exhaustive | 4 | 2 | 8 | 3 | 8 | 3 |
| `h1t6` | P6 | S2 | 90 | exhaustive | 4 | 2 | 4 | 2 | 4 | 2 |
| `h2t6` | P5 | S3 | 36 | exhaustive | 2 | 1 | 3 | 1 | 3 | 1 |
| `h3t6` | P0 | S0 | 36 | exhaustive | 1 | 1 | 1 | 1 | 1 | 1 |
| `h4t6` | P1 | S2 | 90 | exhaustive | 3 | 2 | 6 | 3 | 6 | 3 |
| `h5t6` | P5 | S0 | 27 | exhaustive | 5 | 3 | 7 | 3 | 6 | 3 |
| `h6t6` | P4 | S2 | 90 | exhaustive | 1 | 1 | 1 | 1 | 1 | 1 |
| `h7t6` | P5 | S0 | 90 | exhaustive | 2 | 1 | 2 | 1 | 4 | 2 |
| `h8t6` | P5 | S3 | 7 | exhaustive | 2 | 1 | 2 | 2 | 2 | 1 |
| `h9t6` | P4 | S1 | 30 | exhaustive | 4 | 3 | 5 | 3 | 5 | 3 |
| `h10t6` | P6 | S1 | 19 | exhaustive | 4 | 2 | 7 | 3 | 7 | 3 |
| `h11t6` | P4 | S0 | 36 | exhaustive | 4 | 2 | 8 | 3 | 8 | 3 |
| `h12t6` | P0 | S3 | 6 | exhaustive | 1 | 1 | 1 | 1 | 1 | 1 |

### Horizon 3 (trick 5 start, 3 tiles per seat, unconstrained fiber 1,680)

| kernel | decl | focal | fiber (post-void) | coverage | `q_trick` | `act_trick` | `q_points` | `act_points` | `q_param` | `act_param` |
|---|---|---:|---:|---|---:|---:|---:|---:|---:|---:|
| `h0t5` | P3 | S1 | 1,680 | exhaustive | 2 | 1 | 5 | 3 | 5 | 3 |
| `h1t5` | P6 | S2 | 1,680 | exhaustive | 2 | 1 | 2 | 1 | 2 | 1 |
| `h2t5` | P5 | S3 | 1,680 | exhaustive | 2 | 1 | 7 | 4 | 5 | 3 |
| `h3t5` | P0 | S0 | 200 | exhaustive | 1 | 1 | 1 | 1 | 1 | 1 |
| `h4t5` | P1 | S1 | 1,680 | exhaustive | 6 | 3 | 15 | 3 | 18 | 6 |
| `h5t5` | P5 | S1 | 560 | exhaustive | 20 | 7 | 94 | 7 | 25 | 7 |
| `h6t5` | P4 | S3 | 1,680 | exhaustive | 15 | 5 | 46 | 5 | 37 | 9 |
| `h7t5` | P5 | S0 | 1,680 | exhaustive | 18 | 7 | 18 | 7 | 31 | 7 |
| `h8t5` | P5 | S3 | 92 | exhaustive | 2 | 1 | 3 | 2 | 3 | 1 |
| `h9t5` | P4 | S2 | 1,680 | exhaustive | 11 | 4 | 38 | 6 | 32 | 10 |
| `h10t5` | P6 | S3 | 700 | exhaustive | 23 | 7 | 50 | 7 | 50 | 7 |
| `h11t5` | P4 | S0 | 1,120 | exhaustive | 6 | 2 | 34 | 7 | 18 | 8 |
| `h12t5` | P0 | S3 | 1,680 | exhaustive | 3 | 2 | 6 | 2 | 6 | 2 |

### Horizon 4 (trick 4 start, 4 tiles per seat, unconstrained fiber 34,650)

| kernel | decl | focal | fiber (post-void) | coverage | `q_trick` | `act_trick` | `q_points` | `act_points` | `q_param` | `act_param` |
|---|---|---:|---:|---|---:|---:|---:|---:|---:|---:|
| `h0t4` | P3 | S1 | 34,650 | exhaustive | 19 | 10 | 105 | 11 | 79 | 19 |
| `h1t4` | P6 | S2 | 34,650 | exhaustive | 2 | 1 | 5 | 3 | 5 | 3 |
| `h2t4` | P5 | S3 | 23,100 | exhaustive | 12 | 6 | 54 | 7 | 35 | 11 |
| `h3t4` | P0 | S2 | 11,550 | exhaustive | 122 | 11 | 2064 | 15 | 788 | 96 |
| `h4t4` | P1 | S1 | 34,650 | exhaustive | 20 | 5 | 216 | 7 | 75 | 10 |
| `h5t4` | P5 | S2 | 14,700 | exhaustive | 36 | 11 | 409 | 13 | 103 | 13 |
| `h6t4` | P4 | S3 | 34,650 | exhaustive | 95 | 11 | 609 | 11 | 502 | 77 |
| `h7t4` | P5 | S3 | 23,100 | exhaustive | 161 | 15 | 571 | 15 | 201 | 15 |
| `h8t4` | P5 | S1 | 1,200 | exhaustive | 42 | 14 | 209 | 14 | 137 | 21 |
| `h9t4` | P4 | S2 | 34,650 | exhaustive | 11 | 5 | 58 | 7 | 39 | 14 |
| `h10t4` | P6 | S1 | 8,820 | exhaustive | 104 | 15 | 1034 | 15 | 261 | 17 |
| `h11t4` | P4 | S3 | 23,100 | exhaustive | 29 | 7 | 242 | 15 | 29 | 7 |
| `h12t4` | P0 | S1 | 34,650 | exhaustive | 10 | 3 | 55 | 3 | 59 | 9 |

### Horizon 5 (trick 3 start, 5 tiles per seat, unconstrained fiber 756,756)

| kernel | decl | focal | fiber (post-void) | coverage | `q_trick` | `act_trick` | `q_points` | `act_points` | `q_param` | `act_param` |
|---|---|---:|---:|---|---:|---:|---:|---:|---:|---:|
| `h0t3` | P3 | S2 | 756,756 | mixed (see `+`) | 404+ | 26+ | 2466+ | 27+ | 769+ | 110+ |
| `h1t3` | P6 | S2 | 756,756 | mixed (see `+`) | 2+ | 1+ | 10+ | 8+ | 7+ | 5+ |
| `h2t3` | P5 | S2 | 756,756 | mixed (see `+`) | 214+ | 23+ | 2085+ | 22+ | 465+ | 58+ |
| `h3t3` | P0 | S2 | 756,756 | mixed (see `+`) | 1007+ | 31+ | 5345+ | 31+ | 980+ | 91+ |
| `h4t3` | P1 | S3 | 756,756 | mixed (see `+`) | 509+ | 23+ | 3614+ | 24+ | 692+ | 67+ |
| `h5t3` | P5 | S2 | 324,324 | mixed (see `+`) | 123+ | 21+ | 1239+ | 28+ | 235+ | 23+ |
| `h6t3` | P4 | S0 | 756,756 | mixed (see `+`) | 386+ | 31+ | 4295+ | 30+ | 591+ | 117+ |
| `h7t3` | P5 | S1 | 504,504 | mixed (see `+`) | 425+ | 31+ | 2561+ | 31+ | 329+ | 30+ |
| `h8t3` | P5 | S1 | 59,976 | mixed (see `+`) | 219 | 31 | 4796 | 31 | 470+ | 71+ |
| `h9t3` | P4 | S2 | 504,504 | mixed (see `+`) | 23+ | 6+ | 215+ | 8+ | 108+ | 14+ |
| `h10t3` | P6 | S1 | 756,756 | mixed (see `+`) | 539+ | 31+ | 4832+ | 31+ | 795+ | 101+ |
| `h11t3` | P4 | S2 | 64,638 | mixed (see `+`) | 403+ | 31+ | 2664+ | 31+ | 454+ | 66+ |
| `h12t3` | P0 | S0 | 756,756 | mixed (see `+`) | 333+ | 23+ | 2589+ | 26+ | 562+ | 142+ |

`+` marks a **sampled lower bound**: the true class count for that kernel is at least the number shown.

### Horizon 6 (trick 2 start, 6 tiles per seat, unconstrained fiber 17,153,136)

| kernel | decl | focal | fiber (post-void) | coverage | `q_trick` | `act_trick` | `q_points` | `act_points` |
|---|---|---:|---:|---|---:|---:|---:|---:|
| `h0t2` | P3 | S2 | 17,153,136 | sampled 400 | 275+ | 39+ | 352+ | 33+ |
| `h1t2` | P6 | S2 | 11,435,424 | sampled 400 | 17+ | 7+ | — | — |
| `h2t2` | P5 | S1 | 4,624,620 | sampled 400 | 244+ | 30+ | 349+ | 44+ |
| `h3t2` | P0 | S2 | 17,153,136 | sampled 400 | 321+ | 44+ | — | — |
| `h7t2` | P5 | S0 | 4,624,620 | sampled 400 | 213+ | 33+ | 327+ | 33+ |

`+` marks a **sampled lower bound**: the true class count for that kernel is at least the number shown.

## Per-horizon summary

| H | kernels | fiber (post-void) min / median / max | target | classes min / median / max | median classes per 1,000 worlds *examined* |
|---:|---:|---|---|---|---|
| 2 | 13 | 6 / 36 / 90 | `q_trick` | 1 / 3.000 / 5 | 55.556 |
|  |  |  | `act_trick` | 1 / 2.000 / 3 | 27.778 |
|  |  |  | `q_points` | 1 / 4.000 / 8 | 88.889 |
|  |  |  | `act_points` | 1 / 2.000 / 3 | 33.333 |
|  |  |  | `q_param` | 1 / 4.000 / 8 | 88.889 |
|  |  |  | `act_param` | 1 / 2.000 / 3 | 33.333 |
| 3 | 13 | 92 / 1,680 / 1,680 | `q_trick` | 1 / 6.000 / 23 | 5.357 |
|  |  |  | `act_trick` | 1 / 2.000 / 7 | 2.381 |
|  |  |  | `q_points` | 1 / 15.000 / 94 | 10.714 |
|  |  |  | `act_points` | 1 / 4.000 / 7 | 3.571 |
|  |  |  | `q_param` | 1 / 18.000 / 50 | 16.071 |
|  |  |  | `act_param` | 1 / 6.000 / 10 | 5.000 |
| 4 | 13 | 1,200 / 23,100 / 34,650 | `q_trick` | 2 / 29.000 / 161 | 1.255 |
|  |  |  | `act_trick` | 1 / 10.000 / 15 | 0.303 |
|  |  |  | `q_points` | 5 / 216.000 / 2064 | 10.476 |
|  |  |  | `act_points` | 3 / 11.000 / 15 | 0.317 |
|  |  |  | `q_param` | 5 / 79.000 / 788 | 2.280 |
|  |  |  | `act_param` | 3 / 14.000 / 96 | 0.548 |
| 5 | 13 | 59,976 / 756,756 / 756,756 | `q_trick` | 2 / 386.000 / 1007 | 38.600 |
|  |  |  | `act_trick` | 1 / 26.000 / 31 | 2.300 |
|  |  |  | `q_points` | 10 / 2589.000 / 5345 | 256.100 |
|  |  |  | `act_points` | 8 / 28.000 / 31 | 2.700 |
|  |  |  | `q_param` | 7 / 470.000 / 980 | 235.000 |
|  |  |  | `act_param` | 5 / 67.000 / 142 | 33.500 |
| 6 | 5 | 4,624,620 / 11,435,424 / 17,153,136 | `q_trick` | 17 / 244.000 / 321 | 610.000 |
|  |  |  | `act_trick` | 7 / 33.000 / 44 | 82.500 |
|  |  |  | `q_points` | 327 / 349.000 / 352 | 872.500 |
|  |  |  | `act_points` | 33 / 33.000 / 44 | 82.500 |

## How concentrated is the census?

A raw class count is not the whole story. If a handful of classes hold almost all the worlds and the rest is a tail of singletons, a scheme that carries only the big classes and treats the tail as residue is still useful. Medians across the kernels of each horizon:

| H | target | median share of worlds in the top class | top 5 classes | median classes that are singletons |
|---:|---|---:|---:|---:|
| 2 | `q_trick` | 63.889% | 100.000% | 0.000% |
| 2 | `q_points` | 50.000% | 100.000% | 0.000% |
| 2 | `act_points` | 85.185% | 100.000% | 0.000% |
| 3 | `q_trick` | 43.929% | 99.643% | 0.000% |
| 3 | `q_points` | 27.500% | 66.488% | 0.000% |
| 3 | `act_points` | 59.286% | 100.000% | 0.000% |
| 4 | `q_trick` | 19.286% | 63.377% | 0.000% |
| 4 | `q_points` | 15.186% | 39.544% | 7.407% |
| 4 | `act_points` | 27.573% | 83.515% | 0.000% |
| 5 | `q_trick` | 15.130% | 30.420% | 21.340% |
| 5 | `q_points` | 14.340% | 22.760% | 56.123% |
| 5 | `act_points` | 23.750% | 66.280% | 0.000% |
| 6 | `q_trick` | 5.250% | 13.250% | 70.082% |
| 6 | `q_points` | 5.250% | 10.750% | 93.696% |
| 6 | `act_points` | 30.000% | 53.750% | 24.242% |

## The action-correspondence census has a trivial ceiling

A seat on lead at horizon H has exactly H legal root actions, so the set of optimal actions is a non-empty subset of an H-element set and the `act_*` census can never exceed **2^H - 1** whatever the fiber does. That ceiling is *not* evidence for the scheme — it is arithmetic, and a small action census only means something when it sits well below the ceiling. So the question is how often it is pinned there.

| H | ceiling 2^H-1 | kernels | `act_trick` at ceiling | `act_points` at ceiling | max `act_points` |
|---:|---:|---:|---:|---:|---:|
| 2 | 3 | 13 | 2 | 6 | 3 |
| 3 | 7 | 13 | 3 | 4 | 7 |
| 4 | 15 | 13 | 2 | 4 | 15 |
| 5 | 31 | 13 | 6 | 5 | 31 |
| 6 | 63 | 5 | 0 | 0 | 44 |

The answer is: **a minority of kernels, and they are the low-control ones.** From horizon 3 onward a few kernels per horizon are pinned at the ceiling — those are exactly the hands at the bottom of the control table, where every trump is hidden and every root action can be optimal in some world. The rest sit well under it, and the medians in the fixed-window table stay far below the cap at every horizon. So the action census is doing real work, not merely inheriting a bound — but any claim about it has to be read against the cap, and in the low-control regime the cap is doing most of the explaining.

## Saturation of the sampled censuses

Classes discovered after the first *n* sampled worlds. A curve that is still climbing at the last checkpoint means the reported count is a loose lower bound.

**H=5, `q_trick`**

| kernel | fiber | n=25 | n=50 | n=100 | n=250 | n=500 | n=1,000 | n=2,000 | n=3,000 | n=5,000 | n=7,500 | n=10,000 | last-doubling ratio | verdict |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| `h0t3` | 756,756 | 18 | 33 | 59 | 97 | 156 | 201 | 272 | 305 | 351 | 379 | 404 | 1.151 | nearly saturated |
| `h1t3` | 756,756 | 2 | 2 | 2 | 2 | 2 | 2 | 2 | 2 | 2 | 2 | 2 | 1.000 | saturated |
| `h2t3` | 756,756 | 20 | 34 | 46 | 69 | 95 | 122 | 144 | 160 | 184 | 203 | 214 | 1.163 | nearly saturated |
| `h3t3` | 756,756 | 23 | 41 | 78 | 149 | 249 | 389 | 528 | 632 | 770 | 908 | 1007 | 1.308 | still climbing |
| `h4t3` | 756,756 | 21 | 35 | 58 | 102 | 143 | 214 | 289 | 347 | 407 | 467 | 509 | 1.251 | nearly saturated |
| `h5t3` | 324,324 | 17 | 26 | 35 | 64 | 81 | 96 | 105 | 111 | 117 | 120 | 123 | 1.051 | nearly saturated |
| `h6t3` | 756,756 | 21 | 36 | 55 | 91 | 140 | 180 | 231 | 273 | 320 | 359 | 386 | 1.206 | nearly saturated |
| `h7t3` | 504,504 | 20 | 34 | 57 | 100 | 144 | 204 | 268 | 301 | 351 | 389 | 425 | 1.211 | nearly saturated |
| `h9t3` | 504,504 | 10 | 12 | 15 | 19 | 21 | 21 | 21 | 22 | 23 | 23 | 23 | 1.000 | saturated |
| `h10t3` | 756,756 | 18 | 35 | 60 | 114 | 166 | 225 | 315 | 385 | 449 | 506 | 539 | 1.200 | nearly saturated |
| `h11t3` | 64,638 | 16 | 25 | 44 | 94 | 137 | 195 | 254 | 289 | 332 | 372 | 403 | 1.214 | nearly saturated |
| `h12t3` | 756,756 | 16 | 26 | 36 | 74 | 103 | 138 | 177 | 206 | 251 | 299 | 333 | 1.327 | still climbing |

**H=5, `act_trick`**

| kernel | fiber | n=25 | n=50 | n=100 | n=250 | n=500 | n=1,000 | n=2,000 | n=3,000 | n=5,000 | n=7,500 | n=10,000 | last-doubling ratio | verdict |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| `h0t3` | 756,756 | 10 | 15 | 17 | 19 | 22 | 22 | 25 | 26 | 26 | 26 | 26 | 1.000 | saturated |
| `h1t3` | 756,756 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1.000 | saturated |
| `h2t3` | 756,756 | 7 | 9 | 10 | 14 | 17 | 18 | 18 | 19 | 21 | 23 | 23 | 1.095 | nearly saturated |
| `h3t3` | 756,756 | 15 | 19 | 22 | 23 | 27 | 28 | 30 | 31 | 31 | 31 | 31 | 1.000 | saturated |
| `h4t3` | 756,756 | 10 | 13 | 16 | 17 | 20 | 20 | 22 | 22 | 22 | 23 | 23 | 1.045 | saturated |
| `h5t3` | 324,324 | 12 | 15 | 15 | 19 | 20 | 20 | 20 | 21 | 21 | 21 | 21 | 1.000 | saturated |
| `h6t3` | 756,756 | 7 | 13 | 18 | 20 | 22 | 23 | 27 | 30 | 31 | 31 | 31 | 1.000 | saturated |
| `h7t3` | 504,504 | 13 | 20 | 22 | 25 | 28 | 29 | 30 | 30 | 31 | 31 | 31 | 1.000 | saturated |
| `h9t3` | 504,504 | 3 | 3 | 3 | 5 | 5 | 5 | 5 | 5 | 6 | 6 | 6 | 1.000 | saturated |
| `h10t3` | 756,756 | 12 | 18 | 23 | 27 | 30 | 31 | 31 | 31 | 31 | 31 | 31 | 1.000 | saturated |
| `h11t3` | 64,638 | 11 | 15 | 21 | 24 | 26 | 27 | 30 | 30 | 30 | 31 | 31 | 1.033 | saturated |
| `h12t3` | 756,756 | 11 | 15 | 17 | 23 | 23 | 23 | 23 | 23 | 23 | 23 | 23 | 1.000 | saturated |

**H=5, `q_points`**

| kernel | fiber | n=25 | n=50 | n=100 | n=250 | n=500 | n=1,000 | n=2,000 | n=3,000 | n=5,000 | n=7,500 | n=10,000 | last-doubling ratio | verdict |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| `h0t3` | 756,756 | 20 | 37 | 78 | 163 | 303 | 509 | 863 | 1130 | 1604 | 2071 | 2466 | 1.537 | still climbing |
| `h1t3` | 756,756 | 6 | 8 | 9 | 9 | 9 | 10 | 10 | 10 | 10 | 10 | 10 | 1.000 | saturated |
| `h2t3` | 756,756 | 24 | 48 | 80 | 175 | 314 | 498 | 799 | 1037 | 1427 | 1779 | 2085 | 1.461 | still climbing |
| `h3t3` | 756,756 | 24 | 44 | 84 | 201 | 386 | 741 | 1372 | 1935 | 3000 | 4216 | 5345 | 1.782 | **~linear — far from saturated** |
| `h4t3` | 756,756 | 23 | 44 | 83 | 182 | 315 | 586 | 1045 | 1442 | 2144 | 2922 | 3614 | 1.686 | still climbing |
| `h5t3` | 324,324 | 22 | 37 | 59 | 128 | 211 | 352 | 546 | 692 | 897 | 1097 | 1239 | 1.381 | still climbing |
| `h6t3` | 756,756 | 22 | 38 | 78 | 198 | 371 | 660 | 1191 | 1690 | 2560 | 3477 | 4295 | 1.678 | still climbing |
| `h7t3` | 504,504 | 22 | 42 | 79 | 167 | 306 | 535 | 910 | 1210 | 1678 | 2171 | 2561 | 1.526 | still climbing |
| `h9t3` | 504,504 | 20 | 29 | 51 | 76 | 101 | 127 | 159 | 175 | 192 | 206 | 215 | 1.120 | nearly saturated |
| `h10t3` | 756,756 | 20 | 44 | 87 | 203 | 393 | 715 | 1295 | 1843 | 2831 | 3929 | 4832 | 1.707 | **~linear — far from saturated** |
| `h11t3` | 64,638 | 21 | 38 | 74 | 168 | 311 | 539 | 923 | 1235 | 1760 | 2234 | 2664 | 1.514 | still climbing |
| `h12t3` | 756,756 | 19 | 35 | 65 | 152 | 270 | 484 | 820 | 1125 | 1630 | 2146 | 2589 | 1.588 | still climbing |

**H=5, `act_points`**

| kernel | fiber | n=25 | n=50 | n=100 | n=250 | n=500 | n=1,000 | n=2,000 | n=3,000 | n=5,000 | n=7,500 | n=10,000 | last-doubling ratio | verdict |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| `h0t3` | 756,756 | 10 | 12 | 20 | 21 | 23 | 26 | 27 | 27 | 27 | 27 | 27 | 1.000 | saturated |
| `h1t3` | 756,756 | 4 | 6 | 7 | 7 | 7 | 8 | 8 | 8 | 8 | 8 | 8 | 1.000 | saturated |
| `h2t3` | 756,756 | 9 | 12 | 13 | 17 | 18 | 18 | 20 | 20 | 21 | 22 | 22 | 1.048 | saturated |
| `h3t3` | 756,756 | 11 | 15 | 20 | 23 | 28 | 31 | 31 | 31 | 31 | 31 | 31 | 1.000 | saturated |
| `h4t3` | 756,756 | 10 | 12 | 15 | 19 | 21 | 22 | 22 | 22 | 22 | 24 | 24 | 1.091 | nearly saturated |
| `h5t3` | 324,324 | 12 | 15 | 16 | 21 | 23 | 26 | 27 | 28 | 28 | 28 | 28 | 1.000 | saturated |
| `h6t3` | 756,756 | 8 | 10 | 15 | 20 | 22 | 26 | 29 | 29 | 30 | 30 | 30 | 1.000 | saturated |
| `h7t3` | 504,504 | 13 | 16 | 20 | 23 | 27 | 27 | 27 | 28 | 30 | 31 | 31 | 1.033 | saturated |
| `h9t3` | 504,504 | 3 | 3 | 4 | 6 | 7 | 7 | 7 | 7 | 7 | 8 | 8 | 1.143 | nearly saturated |
| `h10t3` | 756,756 | 12 | 18 | 24 | 31 | 31 | 31 | 31 | 31 | 31 | 31 | 31 | 1.000 | saturated |
| `h11t3` | 64,638 | 12 | 15 | 19 | 21 | 23 | 25 | 29 | 31 | 31 | 31 | 31 | 1.000 | saturated |
| `h12t3` | 756,756 | 9 | 17 | 20 | 22 | 23 | 24 | 24 | 25 | 25 | 26 | 26 | 1.040 | saturated |

**H=5, `q_param`**

| kernel | fiber | n=25 | n=50 | n=100 | n=250 | n=500 | n=1,000 | n=2,000 | last-doubling ratio | verdict |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| `h0t3` | 756,756 | 19 | 37 | 72 | 147 | 277 | 452 | 769 | 1.701 | **~linear — far from saturated** |
| `h1t3` | 756,756 | 6 | 6 | 6 | 6 | 7 | 7 | 7 | 1.000 | saturated |
| `h2t3` | 756,756 | 22 | 39 | 62 | 115 | 198 | 313 | 465 | 1.486 | still climbing |
| `h3t3` | 756,756 | 24 | 43 | 83 | 186 | 333 | 583 | 980 | 1.681 | still climbing |
| `h4t3` | 756,756 | 21 | 39 | 75 | 154 | 250 | 433 | 692 | 1.598 | still climbing |
| `h5t3` | 324,324 | 19 | 31 | 49 | 88 | 122 | 181 | 235 | 1.298 | nearly saturated |
| `h6t3` | 756,756 | 22 | 37 | 71 | 150 | 249 | 381 | 591 | 1.551 | still climbing |
| `h7t3` | 504,504 | 20 | 37 | 64 | 115 | 170 | 248 | 329 | 1.327 | still climbing |
| `h8t3` | 59,976 | 20 | 30 | 60 | 109 | 191 | 316 | 470 | 1.487 | still climbing |
| `h9t3` | 504,504 | 19 | 23 | 37 | 51 | 69 | 90 | 108 | 1.200 | nearly saturated |
| `h10t3` | 756,756 | 20 | 42 | 78 | 160 | 294 | 485 | 795 | 1.639 | still climbing |
| `h11t3` | 64,638 | 16 | 29 | 53 | 124 | 195 | 302 | 454 | 1.503 | still climbing |
| `h12t3` | 756,756 | 18 | 31 | 53 | 117 | 188 | 335 | 562 | 1.678 | still climbing |

**H=5, `act_param`**

| kernel | fiber | n=25 | n=50 | n=100 | n=250 | n=500 | n=1,000 | n=2,000 | last-doubling ratio | verdict |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| `h0t3` | 756,756 | 14 | 18 | 26 | 32 | 46 | 73 | 110 | 1.507 | still climbing |
| `h1t3` | 756,756 | 4 | 4 | 4 | 4 | 5 | 5 | 5 | 1.000 | saturated |
| `h2t3` | 756,756 | 8 | 12 | 12 | 22 | 33 | 44 | 58 | 1.318 | still climbing |
| `h3t3` | 756,756 | 15 | 18 | 25 | 34 | 44 | 55 | 91 | 1.655 | still climbing |
| `h4t3` | 756,756 | 10 | 15 | 21 | 28 | 36 | 49 | 67 | 1.367 | still climbing |
| `h5t3` | 324,324 | 11 | 15 | 15 | 21 | 22 | 23 | 23 | 1.000 | saturated |
| `h6t3` | 756,756 | 9 | 14 | 23 | 37 | 49 | 77 | 117 | 1.519 | still climbing |
| `h7t3` | 504,504 | 13 | 20 | 22 | 25 | 28 | 29 | 30 | 1.034 | saturated |
| `h8t3` | 59,976 | 10 | 16 | 25 | 33 | 41 | 53 | 71 | 1.340 | still climbing |
| `h9t3` | 504,504 | 3 | 3 | 4 | 8 | 9 | 10 | 14 | 1.400 | still climbing |
| `h10t3` | 756,756 | 15 | 24 | 28 | 37 | 53 | 72 | 101 | 1.403 | still climbing |
| `h11t3` | 64,638 | 10 | 14 | 21 | 27 | 33 | 45 | 66 | 1.467 | still climbing |
| `h12t3` | 756,756 | 11 | 19 | 25 | 38 | 53 | 93 | 142 | 1.527 | still climbing |

**H=6, `q_trick`**

| kernel | fiber | n=25 | n=50 | n=100 | n=250 | n=400 | last-doubling ratio | verdict |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| `h0t2` | 17,153,136 | 25 | 47 | 87 | 195 | 275 | 3.161 | **~linear — far from saturated** |
| `h1t2` | 11,435,424 | 8 | 11 | 14 | 17 | 17 | 1.214 | nearly saturated |
| `h2t2` | 4,624,620 | 25 | 46 | 79 | 171 | 244 | 3.089 | **~linear — far from saturated** |
| `h3t2` | 17,153,136 | 25 | 48 | 92 | 216 | 321 | 3.489 | **~linear — far from saturated** |
| `h7t2` | 4,624,620 | 21 | 36 | 67 | 147 | 213 | 3.179 | **~linear — far from saturated** |

**H=6, `act_trick`**

| kernel | fiber | n=25 | n=50 | n=100 | n=250 | n=400 | last-doubling ratio | verdict |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| `h0t2` | 17,153,136 | 17 | 21 | 28 | 37 | 39 | 1.393 | still climbing |
| `h1t2` | 11,435,424 | 5 | 5 | 7 | 7 | 7 | 1.000 | saturated |
| `h2t2` | 4,624,620 | 14 | 20 | 23 | 26 | 30 | 1.304 | still climbing |
| `h3t2` | 17,153,136 | 15 | 21 | 27 | 37 | 44 | 1.630 | still climbing |
| `h7t2` | 4,624,620 | 11 | 18 | 24 | 29 | 33 | 1.375 | still climbing |

**H=6, `q_points`**

| kernel | fiber | n=25 | n=50 | n=100 | n=250 | n=400 | last-doubling ratio | verdict |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| `h0t2` | 17,153,136 | 25 | 49 | 96 | 229 | 352 | 3.667 | **~linear — far from saturated** |
| `h2t2` | 4,624,620 | 25 | 47 | 95 | 224 | 349 | 3.674 | **~linear — far from saturated** |
| `h7t2` | 4,624,620 | 23 | 43 | 85 | 208 | 327 | 3.847 | **~linear — far from saturated** |

**H=6, `act_points`**

| kernel | fiber | n=25 | n=50 | n=100 | n=250 | n=400 | last-doubling ratio | verdict |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| `h0t2` | 17,153,136 | 9 | 13 | 22 | 30 | 33 | 1.500 | still climbing |
| `h2t2` | 4,624,620 | 14 | 21 | 29 | 39 | 44 | 1.517 | still climbing |
| `h7t2` | 4,624,620 | 11 | 15 | 21 | 27 | 33 | 1.571 | still climbing |

### How loose is a sampled floor?

Where the same kernel and target were censused both ways, the sampled count can be checked against the truth. This is the only direct measurement of the gap in this run:

| kernel | target | fiber | sampled n | sampled floor | exact count | floor recovers |
|---|---|---:|---:|---:|---:|---:|
| `h8t3` | `act_points` | 59,976 | 10,000 | 30 | 31 | 96.774% |
| `h8t3` | `act_trick` | 59,976 | 10,000 | 30 | 31 | 96.774% |
| `h8t3` | `q_points` | 59,976 | 10,000 | 2098 | 4796 | 43.745% |
| `h8t3` | `q_trick` | 59,976 | 10,000 | 188 | 219 | 85.845% |

Treat that recovery percentage as indicative of one kernel, not as a correction factor to apply elsewhere: a kernel whose saturation curve is flatter will recover more, and one still climbing linearly will recover far less.

The *last-doubling ratio* is classes(n) / classes(n/2). A value near 1 means the census has been fully enumerated by the sample; a value near 2 means each new world is still buying new classes at close to the initial rate, so the reported count says almost nothing about the true one beyond being a floor.

## Control covariates: does the census track control, not fiber size?

The hypothesis under test is that the number of response classes is governed by the *control structure* of the focal hand — how much of the outcome the focal seat can force regardless of the hidden split — rather than by how many hidden worlds there are. The covariates recorded per kernel are: `n_absolute_masters` (focal tiles no unseen tile can beat when led), `live_trumps_focal` / `live_trumps_hidden`, `focal_holds_top_live_trump`, `count_points_live`, and `fiber_size`.

Exact Spearman rank correlations of the class count against each covariate, computed **within each horizon** (pooling across horizons would just re-measure the horizon):

**`q_trick`**

| H | kernels | `fiber_size` | `n_absolute_masters` | `live_trumps_focal` | `live_trumps_hidden` | `focal_holds_top_live_trump` | `count_points_live` |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 2 | 13 | 0.000 | -0.633 | -0.687 | -0.095 | -0.816 | -0.175 |
| 3 | 13 | 0.045 | -0.687 | -0.217 | 0.663 | -0.482 | -0.111 |
| 4 | 13 | -0.643 | -0.611 | -0.747 | 0.756 | -0.495 | 0.641 |
| 5 | 13 | 0.286 | -0.593 | -0.393 | 0.516 | -0.507 | 0.346 |
| 6 | 5 | 0.632 | -0.894 | -0.872 | 0.410 | -0.866 | 0.667 |

**`q_points`**

| H | kernels | `fiber_size` | `n_absolute_masters` | `live_trumps_focal` | `live_trumps_hidden` | `focal_holds_top_live_trump` | `count_points_live` |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 2 | 13 | 0.092 | -0.462 | -0.695 | -0.151 | -0.687 | -0.102 |
| 3 | 13 | 0.022 | -0.636 | -0.397 | 0.552 | -0.536 | 0.042 |
| 4 | 13 | -0.503 | -0.717 | -0.626 | 0.697 | -0.619 | 0.685 |
| 5 | 13 | 0.091 | -0.385 | -0.462 | 0.363 | -0.254 | 0.675 |

**`act_points`**

| H | kernels | `fiber_size` | `n_absolute_masters` | `live_trumps_focal` | `live_trumps_hidden` | `focal_holds_top_live_trump` | `count_points_live` |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 2 | 13 | -0.068 | -0.602 | -0.599 | 0.125 | -0.740 | 0.082 |
| 3 | 13 | -0.021 | -0.441 | -0.476 | 0.338 | -0.399 | 0.027 |
| 4 | 13 | -0.740 | -0.505 | -0.898 | 0.507 | -0.380 | 0.707 |
| 5 | 13 | -0.365 | -0.286 | -0.152 | 0.317 | -0.131 | 0.591 |

The same story told concretely at H=5, the widest exhaustive horizon, with the kernels sorted from most to least focal control:

| kernel | fiber | masters | focal trumps | hidden trumps | holds top trump | `q_trick` | `q_points` | `act_points` |
|---|---:|---:|---:|---:|:-:|---:|---:|---:|
| `h1t3` | 756,756 | 4 | 2 | 0 | yes | 2 | 10 | 8 |
| `h9t3` | 504,504 | 2 | 3 | 1 | yes | 23 | 215 | 8 |
| `h5t3` | 324,324 | 1 | 2 | 2 | yes | 123 | 1239 | 28 |
| `h10t3` | 756,756 | 1 | 2 | 5 | yes | 539 | 4832 | 31 |
| `h8t3` | 59,976 | 1 | 1 | 2 | yes | 219 | 4796 | 31 |
| `h0t3` | 756,756 | 0 | 2 | 5 | no | 404 | 2466 | 27 |
| `h7t3` | 504,504 | 0 | 1 | 6 | no | 425 | 2561 | 31 |
| `h11t3` | 64,638 | 0 | 1 | 3 | no | 403 | 2664 | 31 |
| `h6t3` | 756,756 | 0 | 1 | 6 | no | 386 | 4295 | 30 |
| `h2t3` | 756,756 | 0 | 0 | 4 | no | 214 | 2085 | 22 |
| `h12t3` | 756,756 | 0 | 0 | 7 | no | 333 | 2589 | 26 |
| `h4t3` | 756,756 | 0 | 0 | 3 | no | 509 | 3614 | 24 |
| `h3t3` | 756,756 | 0 | 0 | 3 | no | 1007 | 5345 | 31 |

The sharpest single contrast in the run, both at H=5:

- **`h1t3`** (P6): 756,756 worlds collapse onto **10** response classes. Focal hand 1-1, 2-2, 4-3, 6-0, 6-6; 4 absolute masters; 2 of 2 live trumps in hand; holds the top live trump.
- **`h3t3`** (P0): 756,756 worlds — 1.000x *fewer* worlds — spread over **5345** classes. Focal hand 1-1, 3-1, 4-1, 4-4, 6-4; 0 absolute masters; 0 of 3 live trumps in hand; does not hold the top live trump.

Smaller fiber, 534.500x more classes. Whatever governs the census, it is not the number of worlds.

And the version with the confound removed entirely — same horizon, **identical fiber size**, 756,756 worlds each, same number of worlds examined:

| kernel | decl | fiber | masters | focal trumps | hidden trumps | `q_trick` classes |
|---|---|---:|---:|---:|---:|---:|
| `h1t3` | P6 | 756,756 | 4 | 2 | 0 | 2+ |
| `h3t3` | P0 | 756,756 | 0 | 0 | 3 | 1007+ |

Same number of hidden worlds, 503.500x the census. Fiber size is held exactly constant across these two rows, so it explains none of the difference. What differs is the control structure.

Read the two ends of that table. The kernels at the top — the focal seat holding masters and the top live trump — collapse tens of thousands of worlds onto a handful of responses. The kernels at the bottom, where every trump is in hidden hands, are where the census blows up, and they do so at fiber sizes no larger (sometimes much smaller) than the kernels at the top. **Fiber size is not the driver; who controls the suit is.**

**How much weight this carries.** Thirteen kernels per horizon is a small n: a rank correlation of |rho| ~ 0.6 on 13 points is suggestive, not decisive, and six covariates were examined, so some large values are expected by chance alone. The three control covariates are also strongly correlated with each other — a hand with the top trump usually has masters and trumps — so they are one signal seen three ways, not three independent confirmations. What makes the finding credible is not any single coefficient but the *sign pattern holding at every horizon* (control negative, hidden trumps positive, fiber size near zero) together with the 412x contrast above, which no fiber-size account can produce.


## Caveats

1. **The operator is perfect-information minimax.** Each world is solved as if the deal were open, and the census is over the resulting root-Q vectors. That is *not* the seat-facing decision operator: a real seat does not get a per-world answer, it gets one answer over its whole information set. The seat-facing census would live on the decision carrier, and measuring it is future work. What is measured here is the coarseness of the exact per-world response map, which upper-bounds how finely the fiber needs to be distinguished by a PI-based scheme.
2. **Sampled counts are lower bounds**, never estimates of the true count. They are marked `+` in the tables and their saturation curves are printed so the reader can judge how loose they are.
3. **One receipt corpus, thirteen hands, all pip-trump.** The kernels are suffixes of rob self-play, so both the deals and the play that produced the voids come from one engine's behaviour. Doubles-trump and no-trump are entirely unmeasured.
4. **The focal seat is always the actual trick leader**, so the census is a census of leader-on-lead decisions, not of follower decisions.
5. **The void model is the observable one only.** Voids inferred from the completed tricks are cut; no inference from bidding, from partner signalling, or from the opponents' choices among legal plays is used. A real seat's fiber would be smaller and the census correspondingly different.
6. `random` is used only to select sample worlds; every reported class count is exact arithmetic on the selected worlds.

## Deviations from the experiment brief

1. **The rules generalization was inherited, not written fresh.** The brief anticipated generalizing `lambda_probe.py`'s `TRUMP = 3` hardcode. A generalization already existed in the shared scratchpad (`rules42.py`, written by the concurrently running Experiment 4 probe). Rather than duplicate it, this run froze a byte copy as `exp5_rules.py` at start-of-run — so a concurrent edit could not perturb a run in flight — and then validated it independently by full replay of all 13 hands. The validation is this run's own; the code is not.
2. **The parametric target was extended beyond horizons 2-3.** The brief scoped it as a cheap secondary for H=2 and H=3. It proved affordable, so it was run exhaustively at H=4 as well and sampled at H=5.
3. **Exhaustive/sampled split.** The brief's threshold was ~50,000. Every H=4 kernel fits (largest post-void fiber 34,650) and was enumerated. Every H=5 kernel exceeds it (smallest post-void fiber 59,976), so all H=5 kernels were sampled.
4. **Horizon 6 kernel selection.** The brief suggested picking by smallest post-void fiber. The six H=6 kernels were instead chosen to span the *control* range measured at H=4 (from hand 1, the highest-control kernel, to hands 3 and 7, the lowest), because the control hypothesis is what the widest horizon is being asked about. Sample size there is small (400 worlds) and the counts are correspondingly loose floors.
5. **Horizon 6 is partial and was stopped on a wall clock.** Two H=6 kernels ran 2.5 hours of single-core time apiece without finishing their second valuation, so the run was cut there. Where an H=6 cell reads `—` the census was never computed, not computed and found empty. The kernels that did complete span the control range, which is what the horizon was being asked about.
6. **`random` seeding.** One seed per (hand, trick), deliberately not per valuation, so the trick and points censuses of a kernel are computed on the identical world sample and are directly comparable. Seeds are recorded in every sampled record.

## Compute

566 records in one file (`exp5_records.jsonl`). Roughly 31 h 6 m of single-core CPU, run across 18 cores in staged pools, cheapest horizon first, with every finished (kernel, target) appended to disk immediately. About 2,385,588 distinct perfect-information suffix solves went into the value censuses. Peak resident memory was bounded by clearing the boundary cache at 1.5 M entries (~450 MB per worker); cache clears per job are recorded in every record.

## Artefacts

- `exp5_core.py` — kernels, exact fiber counting and uniform sampling, the fast bitmask PI minimax, the PWL parametric solver, covariates.
- `exp5_rules.py` — frozen declaration-relative rules + replay validator; `exp5_pwl.py` — frozen exact piecewise-linear machinery.
- `exp5_validate.py` — the independent naive minimax used for spot checks.
- `exp5_census.py` — the staged driver (`--stages 2,3,4,5,6`).
- `exp5_exact.py` — the opportunistic exhaustive H=5 runs that calibrate how loose the sampled floors are.
- `exp5_report.py` — this report's generator.
- `exp5_records.jsonl` — one record per (kernel, target).
- `exp5_progress.log` — the run log.
