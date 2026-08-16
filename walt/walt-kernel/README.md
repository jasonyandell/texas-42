# walt-kernel

Owns the viewer kernel and its current-remainder fiber (v0.4 §2.1): the
viewer's known hand, the hidden live pool `U`, per-hidden-seat capacities
`k_s`, the observable void constraints that cut each `P_s`, and the fiber
`Phi(C)` itself -- lazy exact enumeration, exact `|Phi(C)|` by an integer DP
over allowed-slot groups, and exact uniform sampling driven by that same DP.

**Imports: `walt-core`** (plus `num-bigint`/`num-rational` for the exact
sampler fingerprint). It may not restate a rule; a void cuts by
`Decl::effective_incidence`, so absorption is inherited rather than
re-derived.

Randomness is selection-only. `SplitMix64` is local so the workspace carries no
RNG dependency, every branch weight in the sampler is an exact integer count of
completions, and no float is ever near a probability.

Validated against the exp5 probe corpus (exploratory tier, used as regression
pins) on 52 kernels drawn from the receipt hands, and internally by the
agreement of the DP, the enumeration, and `Kernel::contains` on the same set --
with the receipt's own deal required to be inside every fiber.
