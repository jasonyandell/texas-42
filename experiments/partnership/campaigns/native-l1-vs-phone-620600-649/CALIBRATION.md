# Native L1 versus the preserved phone reference

Exploratory fixed-budget calibration: **50 paired deals / 150 games**, bid 30 throughout.

The externally capped batch completed in **202.12 seconds**, below the 295-second watchdog allowance.

Native L1 uses `baseline`, 40/8, voidless inner beliefs, fixed samples and no tie refinement. Phone uses the archived WASM at 40/8 with its original racing/refinement.

| Native role | Favorable flips | Unfavorable flips | Ties |
|---|---:|---:|---:|
| declaring | 5 | 8 | 37 |
| defending | 3 | 7 | 40 |

Combined: **8 favorable / 15 unfavorable / 77 ties**; native net **-7.0 percentage points** per matched contract opportunity.

Phone declaring: 31/50 makes. Native declaring: 28/50. Phone facing native defense: 35/50.

A descriptive mean ±2 standard-error band, with one paired observation per deal, is **-14.0 to +0.0 percentage points**. This is a rough normal approximation, not an anytime-valid interval or a formal equivalence test. The predeclared practical equivalence band was ±5 points; absence of a detected difference would not establish equivalence.

| Measured decision time | Phone | Native L1 |
|---|---:|---:|
| All moves | 0.636s | 0.080s |
| Nonforced decisions | 0.964s | 0.121s |

phone: 15 fallbacks / 1843 nonforced decisions.

baseline: 0 fallbacks / 908 nonforced decisions.

At identical public histories encountered by both implementations, 145/241 nonforced choices agreed; 7 comparisons included a fallback. This prefix-conditioned action diagnostic establishes neither population action agreement nor playing-strength equivalence.

All primary rows use the complete contiguous seed prefix. Per-move records, source/binary identities, independent replay verification, fallbacks, and the external cap are retained here.
