# walt-strat

Owns the operators registry of the crate map, kept deliberately distinct per
v0.4 §10.8 (theorems for one operator never silently transfer):

- **PI** (`pi` + `census`): worldwise perfect-information symbolic parametric
  backward induction (§9.9) -- focal-team nodes take the pointwise envelope
  maximum, opposing nodes the minimum, leaves are affine trick increments --
  with the fiber census by parametric root-Q vector, baseline value, and
  optimal-action correspondence (§14.2's census vocabulary).
- **Scalar PI** (`scalar`): the same operator at one integer valuation, cached
  at trick boundaries (full-window-exact entries, in-trick alpha-beta), fast
  enough for 10,000-world censuses at horizon 5. Carries the exp5 probe-suite
  census targets: `q_trick`/`q_points` value classes and optimal-action-set
  classes.
- **H** (`hidden`): the actual hidden-information fixed-field treatment,
  solved exactly on pooled information states against the fiber.
- **C**/**F** (`revealed`): continuation- and root-revelation with the field
  held fixed (§10.8), aggregated at the support level.
- **Prices** (`price`): the §10.5 information prices with nonnegativity and
  the exact decomposition asserted on every result; §10.6 read along the ray.

`info` carries the substrate: decision nodes over fiber-worlds, the canonical
perfect-recall information partition (§10.1), and information-consistent
policies keyed by opaque information-state ids (§7.2 -- world-peeking is
unconstructible by type).

**Imports: `walt-core`, `walt-kernel`, `walt-geom`** -- the strict import
direction of v0.4 §16.2.

Validated against the probe corpus (exploratory tier, regression pins, never
axioms): the §14.2 trick-6 experiment (fixed-field lines, the 8/4/3 census
with sizes (26,22,16,12,8,2,2,2), the eight-world boundary tie resolving at
zero-plus); the §14.5--§14.6 trick-5 record (Q^H segments at 1/5 and 4, root
switch 7/19, the nine-segment Q^C with prices {1/4..3}, C switch 177/131,
G^cont(0) = 19/105, G^root(0) = 4051/45360, G^cont(2-1) ≡ 0 in all twelve
directions); and the exp5 census suite (`walt/probes/exp5`): both exhaustive
horizon-2/3 tables, the trick-6 q_param row, and the sampled h1t3/h3t3
headline pins (10 and 5,345 q_points classes on the recorded 10,000-draw
samples, regenerated from the recorded seeds and frozen under
`tests/data/`).
