# Field-cache probe — the two surgical levers in the acting hot path

**EXPLORATORY INSTRUMENT OUTPUT** — sits below every evidentiary tier and
is cited by nothing above it. Every number here is a machine-local,
single-shot wall reading, never a receipt; nothing in this directory is
quotable above the Ideas tier without a brief amendment that adds it to a
verifier receipt.

The two levers (both shipped; both value-identical by construction and by
gate):

1. **Cached field in act** — `solver::act` evaluates its SetSpec under a
   `FieldModel` (`FieldKind::Level0`, spec = `act_field_spec`) instead of
   the bare `Level0Field`. The Level0 materialization DELEGATES every
   modeled choice to the one bare-field authority and adds only the
   insert-only action cache (O29 shape), so the chosen actions are the
   bare field's exactly.
2. **Decided cutoff in the per-world replay** —
   `replay_viewer_success` returns at the first trick boundary where the
   pmake indicator is decided for every continuation (`decided_success`,
   the monotone bid arithmetic shared with the exposure walks): points
   only accumulate, so a decided prefix decides the terminal Boolean.

Producer: `walt/walt/src/bin/field_cache_bench.rs` (run:
`cargo run --release --bin field_cache_bench`, default features).
Gates: `walt/walt/tests/solver_field_cache.rs` (complete `SetEvaluation`
identity bare-versus-cached on both declared roots, exact and sampled
endpoints; cutoff-versus-full-replay per world over two complete fibers,
re-deriving the pinned wins vectors [78, 34, 34] and
[1118, 654, 563, 556]; late-root decided cases), plus every pre-existing
pinned suite unchanged (`solver_controller`, `solver_act`,
`solver_policy`, `solver_shadow`).

## Workload (declared)

Three roots, the SAME (world × candidate) replay grid per arm, FRESH
policy instances per arm (no cross-arm cache warmth), and the arms' wins
vectors asserted identical inside the producer:

- **exact-h4-t6** — receipt hand 4 trick 6, complete fiber 90.
- **exact-h11-t5** — receipt hand 11 trick 5, complete fiber 1120.
- **sampled-h0-t1** — receipt hand 0 trick 1, fiber 399,072,960; the
  first 128 worlds of a declared evidence stream (the interactive
  `world_cap`), exactly as the controller draws them. This is the
  convicted regime (trick-1/2 decisions at the interactive cap).

Candidates: one act-shaped frozen level-1 continuation per legal root
action at the interactive declared schedule [n_outer 8, n0 2]; field
n0 = 2 (`ActConfig::interactive`).

Arms: (a) `bare-full` = bare `Level0Field`, full-terminal replay (the
pre-lever baseline, reconstructed bench-side); (b) `cached-full` =
cached `FieldModel`, full-terminal replay (lever 1 alone); (c)
`cached-cut` = cached `FieldModel` through the library replay with the
decided cutoff (levers 1+2 — the shipped configuration).

## Results (single shot, one machine, integer micros)

| item          | fiber       | worlds×cands | (a) bare-full | (b) cached-full | (c) cached-cut | (a)/(c)      |
|---------------|-------------|--------------|---------------|-----------------|----------------|--------------|
| exact-h4-t6   | 90          | 90×2         | 3,820         | 1,828           | 1,729          | 3820/1729    |
| exact-h11-t5  | 1,120       | 1120×3       | 364,791       | 306,367         | 303,289        | 364791/303289 |
| sampled-h0-t1 | 399,072,960 | 128×7        | 42,911,487    | 42,505,711      | 41,191,341     | 42911487/41191341 |

Wins vectors agreed across all three arms on every item (the
value-identity contract, live).

## Reading — plainly

**The trick-1 sampled-route case, the number this probe exists for, is
NOT a big ratio: (a)/(c) = 42911487/41191341, about a 4% saving.** The
levers do not rescue the convicted regime on CPU.

Why, mechanically: a non-focal information state is (own hand, FULL
public record). Two worlds — or two candidates — share a cache entry only
while their play histories are still an identical prefix. At trick 1 the
prefix tree fans out immediately (the root pin already differs per
candidate; hidden hands differ per world), so the expensive shallow
modeled choices almost never recur, and the states that do collide are
the cheap late ones — which the decided cutoff removes anyway. On short
horizons the funnel is tight and the cache pays (2.2x on the fiber-90
root); at trick 1 it cannot.

The cutoff's own contribution is real but bounded by where decisions
happen: about 3% here (deciding tricks land late in a 30-bid hand's
replay).

## Caveats

- Single shot, one machine, wall micros; no variance estimate. Arms run
  in the fixed order a→b→c, so any OS warmth bias favors (b)/(c) — which
  strengthens, not weakens, the "no big win" reading.
- The bench times the per-world replay grid (the part the levers touch),
  not the full `evaluate_set` bookkeeping around it; the grid dominates
  that endpoint's cost.
- The wins vectors on the sampled item are declared-stream observations
  over 128 worlds, not values of record.
