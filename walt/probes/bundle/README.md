# Bundle probe — the bundled world evaluator vs the per-world exact route

**EXPLORATORY INSTRUMENT OUTPUT** — sits below every evidentiary tier and
is cited by nothing above it. Every number here is a machine-local wall
reading or an exact integer counter over instrument records, never a
receipt; nothing in this directory is quotable above the Ideas tier
without a brief amendment that adds it to a verifier receipt.

Parent ruling: `walt/CENSUS-RULINGS.md` **E-A15 (order, not set)** —
changing the ORDER of evaluation is lawful; changing the SET is a
declared exclusion. Both routes here evaluate the SAME complete fiber
and the bench asserts their wins totals agree before printing anything;
only the order and grouping of the work differs.

Primitive: `walt/walt/src/solver/bundle.rs` (`bundled_set_outcomes` —
one shared-tree walk per candidate carrying the whole fiber, outcomes
attributed per member world, decided cutoff settling whole bundles).
Gates: `walt/walt/tests/solver_bundle.rs` (element-wise equality against
`calibrate::exact_set_outcomes` on three receipt roots; pinned wins;
attribution completeness; focal-purity rejection).
Producer: `walt/walt/src/bin/bundle_bench.rs`
(run: `cargo run --release --bin bundle_bench`, default features).

## Workload (declared)

Three `verify_player` receipt roots — hand 4 trick 6 (fiber 90, m=3),
hand 11 trick 5 (fiber 1120, m=4), and hand 11 trick 4 (fiber 23100,
m=3, the largest affordable pick from the exp5 corpus; 34650/59976 add
nothing structural). Candidates: the `solver_controller`
preference-order `FrozenPolicy` fixtures, fresh per route. Two field
configurations per root, each a FRESH cold instance per route:

- **cached-level0-n2** — `FieldModel` (`FieldKind::Level0 { n0: 2 }`),
  the realistic exact-route configuration (insert-only action cache,
  O29);
- **lowest-first** — `FixedPreference::lowest_first`, a trivial field
  that isolates walk sharing from per-query field cost.

Routes: **per-world** = the `replay_viewer_success` loop exactly as the
controller's §11.5 escalation endpoint runs it; **bundled** = the
`solver::bundle` primitive. A bench-owned counting shim wraps the field
in both routes, so `field_queries` is an exact integer.

## Counter semantics

`plays` = m·fiber·total (the per-world route runs every post-root play
of every world to terminal — its member-ply count, exact).
`nodes` = bundle-tree nodes expanded across all candidates (the bundled
route's shared public histories). `early_settled`/`terminal_settled` =
(candidate, world) cells the decided cutoff attributed before terminal
vs at terminal (their sum is always m·fiber). `field_queries` = calls
reaching the field model, after the bundled walk's per-node
distinct-hand memo.

## Readings (2026-08-25, M-series, release, single-shot walls)

```
root h4-t6 fiber=90 m=3 total_plays=8
  [cached-level0-n2] per-world: wall_us=1164 plays=2160 field_queries=1620
  [cached-level0-n2] bundled:   wall_us=1004 nodes=657 early=120 terminal=150 field_queries=762
  [lowest-first]     per-world: wall_us=131  plays=2160 field_queries=1620
  [lowest-first]     bundled:   wall_us=160  nodes=687 early=112 terminal=158 field_queries=784
root h11-t5 fiber=1120 m=4 total_plays=12
  [cached-level0-n2] per-world: wall_us=42970 plays=53760 field_queries=40320
  [cached-level0-n2] bundled:   wall_us=36335 nodes=11629 early=2941 terminal=1539 field_queries=14932
  [lowest-first]     per-world: wall_us=3375  plays=53760 field_queries=40320
  [lowest-first]     bundled:   wall_us=3506  nodes=9681 early=2843 terminal=1637 field_queries=12997
root h11-t4 fiber=23100 m=3 total_plays=16
  [cached-level0-n2] per-world: wall_us=1040642 plays=1108800 field_queries=831600
  [cached-level0-n2] bundled:   wall_us=859696  nodes=224983 early=45571 terminal=23729 field_queries=295433
  [lowest-first]     per-world: wall_us=62287   plays=1108800 field_queries=831600
  [lowest-first]     bundled:   wall_us=43623   nodes=173221 early=46200 terminal=23100 field_queries=252712
```

Wins agreed between the routes on every row (asserted live). The two
field configurations produce different wins vectors, as they must — a
different field is a different evaluation.

## Honest finding: the claim is NOT confirmed at this slice

The claim under test — "bundling collapses exact-route cost
superlinearly with fiber size" — does not hold in these readings. The
structural sharing is real and large: at fiber 23100 the bundled tree
expands ~4.9–6.4x fewer nodes than the per-world route's member-plies,
and field queries drop 2.8–3.3x. But wall time improves only ~1.2x
(cached level-0) / ~1.4x (trivial field), roughly flat in fiber size,
and at fiber 90 with a trivial field the bundled route is slightly
SLOWER (overhead dominates a tiny root). Two costs keep both routes
linear in member-plies:

1. **Per-member partition work.** At every field node the bundled walk
   still touches every member (derive its remaining hand, group it), so
   total member-touches ≈ member-plies minus cutoff savings — the same
   asymptotics as the per-world route. Sharing collapses the NODE work
   (trick arithmetic, records, focal queries), not the member work.
2. **Field-state materializations.** With the cached level-0 field, the
   distinct information states each cost one modeled solve in EITHER
   route (the state set is identical — same fiber, same trees), and
   those misses dominate the realistic configuration. Bundling removes
   redundant cache HITS only.

The decided cutoff is effective as accounting: roughly 2/3 of cells
settle before terminal depth (e.g. 45571 of 69300 at h11-t4), but
settlement is deep, so the ply savings are modest.

Follow-up levers this probe locates (not built here): sharing member
work across bundle levels (incremental per-member hands; partitioning
by acting-seat hand classes), and cheaper lawful field-cache hits. Both
are order/representation changes, E-A15-lawful, and belong to later
slices if the exact route's cost matters at scale.

## Honest confounders

Wall micros are single-shot readings on one machine, release profile —
instrument readings, not statistics. The per-world route benefits from
the same warm policy/field caches within its own run (fresh instances
per route, so neither route reads the other's warmth). The bundled
route's node counts depend on the field (different fields, different
trees), so cross-config node comparisons are shape comparisons only.
