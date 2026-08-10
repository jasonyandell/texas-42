# walt — the imperfect-information seat

**Status: exploratory build. Nothing here is promoted; no wiki page cites this
directory yet.** The mathematical basis is frozen at
`walt/math/unified_information_geometry_v0.4.md` (exploratory tier; its §17
claim ledger governs what is "proved prose" vs "reported receipt" vs "open").
rob is the exact solver; **walt is the seat** — the full-hand
imperfect-information player, named by the doc's own fixed-field
specialization (v0.4 §7.4). Decision 2026-08-09 (Jason): freeze v0.4, build
greenfield, **dynamic control skeleton from the jump**, Rust.

## Non-negotiable disciplines (inherited from the project)

- **Exact arithmetic only.** No floats anywhere: clippy denies
  `float_arithmetic`, CI greps deny `f32`/`f64`. Rationals via `num-rational`
  (i128 or BigInt as needed).
- **Information honesty by type.** Seat-facing code consumes observation
  types only; hidden-world types are not constructible from seat context
  (module privacy). "Zero dominoes showing that weren't legal to see" is a
  compile-time property as far as Rust can carry it.
- **Derived views, never stored state**; no identity-bearing reachability
  certificates; equality/hashing through projected state only (rob rules).
- **Every exhaustive count in the spec is a CI assertion.** The Python probe
  results are a free conformance suite (below).
- **Tier discipline**: everything exploratory until walt grows its own
  byte-diffed receipts; external PASS never imported as axiom (TRUST-01).

## Crate map (v0.4 §16.2 module graph, strict import direction)

```
walt-core      pips, dominoes(28), seats/teams, 9 declarations, contexts,
               rule algebra (effective incidence, follow, led context,
               tier/rank/trick key, BEATS/THREAT), count, trick winner,
               legality, deal/history replay          [imports: nothing]
walt-kernel    viewer kernel, observable voids, capacity-cell fiber:
               enumeration, exact counting DP, exact uniform sampling;
               later: support normal form             [imports: core]
walt-geom      exact rationals, PWL envelopes (endpoint-ownership invariants
               — the _combine lesson), capture features, finite feature sets
               as polytopes (finite-first, v0.4 §16.1), support functions
                                                      [imports: core]
walt-strat     decision nodes, information partitions, policies as maps
               from info-state ids, operators registry: PI minimax,
               fixed-field H, revealed C/F            [imports: core, kernel, geom]
walt-skeleton  THE deliverable: trait ControlSkeleton — typed relational
               state + closed update d' = step(d, obs); exhaustive
               lumpability checker (v0.4 §12.6 conditions on finite
               domains); static soundness checker (R* = R̄∘D); day-one
               passenger: the static 3A-style descriptor wrapped as a
               (degenerate-update) transducer, flagged as such
                                                      [imports: all above]
walt-factory   corpus generation (deals + play across all 9 declarations —
               finally exercising DT and NT), census pipelines, synthesis
               loops (counterexample-guided, exact hitting-set), certificate
               emission per v0.4 §16.11 schema        [imports: all above]
```

Scheme/Fix as a query language enters later inside walt-skeleton's
descriptor vocabulary; it imports physics, never the reverse.

## Dynamic-from-the-jump (what it means operationally)

A descriptor is a **transducer**, not a labeling: state + closed update law.
The factory grades every candidate on BOTH axes:
1. **soundness** for the selected response target (static factorization,
   v0.4 §12.1), and
2. **lumpability** of its update (v0.4 §12.6: legal-set agreement and
   (feature-increment, observation, successor-class) kernel agreement within
   classes — exhaustively checkable on finite kernels).

Static-only descriptors are legal passengers (constant/recompute updates)
but are *marked*; the search objective prefers closed updates. If no
nontrivial lumpable skeleton exists on honest domains, that is a reportable
result, not a failure (Jason: "if we don't succeed, that tells us something
useful").

## Ground-truth bridges (conformance suite from day one)

1. **Receipt replay**: walt-core must replay all 13 hands of
   `rob/receipts/verify_player.txt` — actor order, follow legality, winners,
   points, made/set — as integration tests. Same bridge the Python probes
   used; proves the rules layer matches rob.
2. **Cross-implementation test vectors** (exact rationals from the probe
   reports, exploratory tier, used here as regression pins, not as axioms):
   - trick-6 kernel (hand 0): fiber 90; fixed-field lines 2/3+(1/5)λ and
     −2/3−(1/3)λ; PI census 8 parametric / 4 baseline / 3 action classes;
     class sizes (26,22,16,12,8,2,2,2).
   - trick-5 kernel: fiber 1680; masters = {3-2}; H-treatment
     Q(0-0) segments at 1/5 and 4, root switch λ* = 7/19;
     C-treatment root switch 177/131; G^cont(0) = 19/105,
     G^root(0) = 4051/45360; G^cont(2-1) ≡ 0.
   - exp5 census values on shared kernels (e.g. h1t3: 10 q_points classes;
     h3t3: 5345).
3. **Python probes as independent validators**: the scratchpad-era scripts
   (lambda_probe*.py, exp4/exp5 code) remain the second implementation for
   spot checks; the factory can shell out to them in validation mode later.

## Session log

- **S1 (2026-08-09, this worktree)**: decisions + skeleton. Workspace, CI
  script (fmt, clippy -D warnings -D float_arithmetic, no-float grep,
  tests), walt-core complete with replay + exhaustive-count tests,
  walt-kernel fiber enumeration/counting/sampling with known-value tests.
  **COMPLETE**: `walt/ci/check.sh` PASS; 30 tests (13 exhaustive-count, 3
  receipt-replay covering all 13 hands, 9 fiber known-value over 52 exp5
  kernels, 5 sampler). Unique-winner assertion covers all 737,100 four-tile
  tricks x 9 declarations. No spec-vs-reference discrepancies
  (`walt/DISCREPANCIES.md`).
- **S2 (2026-08-09)**: walt-geom + PI minimax + trick-6 census.
  **COMPLETE except the exp5 h1t3/h3t3 pins (blocked, see below)**:
  `walt/ci/check.sh` PASS; 27 new tests (22 walt-geom: lines, envelope
  `_combine` with endpoint-ownership invariants unit + exhaustive small-case,
  argmax correspondences, features/support/gauge; 4 walt-strat census; 1
  `#[ignore]`d blocked). walt-geom: i128 rationals, continuous PWL envelopes
  on [0, inf) with half-open piece ownership as the type invariant, argmax
  correspondences with at-point/after-point sets, 29-dim features, finite
  feature sets with support envelopes (§9.2, finite-first). walt-strat
  (minimal, per the crate map): PI symbolic parametric backward induction
  (§9.9) + fiber census + degenerate fixed-field evaluation for kernels with
  no post-root focal choice. Trick-6 §14.2 vectors all reproduced exactly:
  fiber 90; Q^H lines 2/3+(1/5)λ and −2/3−(1/3)λ (crossing −5/2); 180/180
  affine world/root curves; 8 parametric classes sized (26,22,16,12,8,2,2,2);
  4 baseline; 3 action classes (2 at λ=0); the 8-world boundary tie resolving
  to 2:1 at 0+. The exp5 census pins (h1t3: 10 "q_points classes"; h3t3:
  5345) are **blocked**: v0.4 §14 never defines exp5 or "q_points"
  (`walt/DISCREPANCIES.md`, "exp5 census pins").
- **S3 (2026-08-09)**: walt-strat information layer -- partitions, H/C/F,
  prices. **COMPLETE**: `walt/ci/check.sh` PASS; 12 new tests (4 walt-geom:
  envelope `sub`/`sum_of`/nonnegativity and the §10.6 finite-first
  exposed-witness criterion; 7 exp3B/exp4A cross-validation; 1 trick-6
  revelation-degeneracy) plus the trick-6 fixed-field test rewritten through
  the general operator. `info.rs`: decision nodes as (world, history)
  particles over the fiber, one shared observation-tree walk, the canonical
  perfect-recall `InfoPartition` (§10.1 validity holds by construction;
  coarser gluings deliberately deferred -- they invalidate backward induction
  and need their own solver), `Policy` as a map from opaque `InfoStateId`s
  (§7.2: world-peeking unconstructible by type), and a no-maximization policy
  evaluator. Operators registry: **H** (`hidden.rs`, exact symbolic solve on
  pooled information states), **C**/**F** (`revealed.rs`, field held fixed
  per §10.8, aggregated at the support level -- Minkowski sums and hulls of
  unions never materialized), prices (`price.rs`, §10.5 nonnegativity and the
  exact decomposition asserted on every result, §10.6 read along the ray).
  S2's `field.rs` (`fixed_field_root_lines`, which refused any post-root
  focal choice) is superseded by H and deleted; its trick-6 pins reproduce
  through H, and C == H there (no post-root choice), while `G^root` stays a
  strict resource (§7.6). The §14.5--§14.6 record reproduced exactly: fiber
  1680; masters = {3-2}; Q^H(0-0) segments at 1/5 and 4 with the reported
  coefficients; root switch 7/19; the nine-segment Q^C(0-0) with prices
  {1/4, 1/3, 1/2, 2/3, 1, 3/2, 2, 3}; C root switch 177/131;
  G^cont(0) = 19/105, G^root(0) = 4051/45360, G^total(0) = 12259/45360, the
  same in all twelve live-tile directions; G^cont(2-1) ≡ 0 in all twelve;
  eight control directions affine under H, seven of eight multisegment under
  C and F, only 3-2 affine in all three treatments; V^F segment counts
  51/51/42/53 within the reported 42--53. One reading reconciled
  (`walt/DISCREPANCIES.md`): the reported information-state counts
  168/7848/504 are the states with a genuine choice; walt's full reachable
  totals (60360/69600/164088) are frozen as walt-tier pins. i128 rationals
  sufficed throughout (overflow-checks on, no escalation to BigRational);
  the twelve-direction H+C+F sweep runs ~13s in release.
- S4: walt-skeleton (trait, lumpability checker, static passenger) + first
  synthesis run against factory targets.
- S5+: walt-factory corpus at scale (all 9 declarations), the dynamic
  skeleton search proper, seat chassis wiring (four seats, full hands).

## Open decisions deliberately deferred

- Support normal form: reimplement per foundation spec vs. thin port of
  rob's (lean: reimplement; greenfield, and the spec is the authority).
- Rational width: start i128-backed `Ratio`; escalate to BigRational where
  denominators demand (fiber-weighted expectations at H≥5 will).
- Seat chassis process model (four independent seat states) lands with
  walt-strat, not before.
- Wiki pages: only when something earns a tier above exploratory.
