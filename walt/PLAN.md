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

## Conflict-driven lesson learning (the S5 spine, adopted 2026-08-10)

Decision (Jason + Claude, 2026-08-10): the factory's outer loop is organized
CDCL-style — **harvest failure, generalize it, prune with it**. Not an
algorithm import; a stance import, from the one community that made
exhaustive search industrial and whose safety culture (proof logging,
independent verified checkers, "never trust the solver") independently
evolved this project's own receipt/TRUST-01 discipline.

- **Conflict** = a refuted line: a regretted decision (fiber-expected regret
  > 0 for the action taken, under *declared* continuation semantics and
  world weighting) or a checker failure. The S4 checkers already return
  typed conflict objects (`PurityCounterexample`, `LumpabilityFailure` with
  the witnessing pair and the exact disagreeing event) — that is the reason
  material cut analysis walks. **Reasons everywhere** stays a standing
  requirement: every verdict carries its derivation, or the means to re-run
  it.
- **Lesson** = a generalized verdict: an implicant over the descriptor/atom
  vocabulary → an action verdict, produced by greedy constraint-dropping
  (1UIP culture: a good cut cheaply, never a minimal core expensively),
  widened until an exhaustive checker returns a witness. Every lesson
  carries a **grade** (worldwise dominance > exact expectation under
  declared semantics > sampled) and its **labels** (continuation operator,
  weighting) — lessons are typed, tier discipline applies, and a lesson
  never quotes above its grade.
- **Learn from wins too** (the QBF cube analog): "this plan suffices across
  this class" is a lesson with a witness strategy, dual to "this line is
  refuted across this class."
- **The database is a working set, not an archive**: lessons pay rent
  (measured pruning/regret reduction on the receipt corpus) or are deleted;
  application must be near-free via watched-feature indexing; synthesis
  loops **restart freely, keeping only the lesson DB** (search state is
  disposable, lessons are the asset).
- **Proof-logged**: every lesson emits a certificate per v0.4 §16.11,
  checkable by an independent implementation (the preserved Python probes
  first; a Lean checker is the long-term crown). External PASS is still
  never an axiom.
- **Rate regime honesty**: SAT learns millions of shallow clauses; walt's
  conflicts cost real solves, so the regime is *few conflicts, deeply
  analyzed, maximally generalized* — lemma learning, not industrial SAT.
  Lesson quality per conflict is the metric, not throughput.

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
- **S3.5 (2026-08-09)**: exp5 census pins unblocked. The probe suite landed
  at `walt/probes/exp5/` (commit b3cb523), supplying the missing definition:
  `q_points` = exact PI root value vector with each trick worth ±(1 + count
  points of its four tiles), focal minus opponents; the h1t3/h3t3 headline
  counts are censuses of recorded 10,000-draw uniform samples (seeds
  42042013/42042033), i.e. sampled lower bounds of the fiber census.
  **COMPLETE**: `walt/ci/check.sh` PASS. New `walt-strat/src/scalar.rs`
  (integer PI minimax, trick-boundary cache with full-window-exact entries,
  in-trick alpha-beta) + `tests/exp5_census.rs` (4 tests, ~25s release):
  the samples regenerated via the probe's own sampler and frozen as fixtures
  (distinct counts 9,920/9,933 match the records exactly); **h1t3 = 10 and
  h3t3 = 5,345 q_points classes reproduce exactly**, plus act_points 8/31,
  q_trick 2/1007, act_trick 1/31, the recorded h1t3 class representative,
  true-world class membership for both, both exhaustive horizon-2/3 report
  tables (13 kernels × q/act × trick/points), the trick-6 q_param row, and
  scalar-vs-symbolic agreement on all 647 trick-6 worlds × 2 valuations.
  `act_param` deliberately not pinned (probe canonicalizes correspondences
  by segment identity, walt by argmax value -- different statistics;
  `walt/DISCREPANCIES.md`, "exp5 census pins").
- **S4 (2026-08-09)**: walt-skeleton -- trait, both checkers, static
  passenger, first synthesis run. **COMPLETE**: `walt/ci/check.sh` PASS; 9
  new tests (7 harness incl. 1 `#[ignore]`d blocked, 2 synthesis-run), all
  adding ~0.1s to the gate. `ControlSkeleton` (skeleton.rs): typed
  relational `State` + `init(kernel, world)` (the one latent read) +
  `step(d, obs)` closed over one observed play -- a recompute-from-world
  update is *unconstructible by signature*, so the only legal degenerate
  form is the constant update, marked via `UpdateKind::StaticPassenger`
  (`StaticWrap`). Observations are seat-honest (seat, tile) pairs derived
  from walt-strat's record (obs.rs). §12.1 checker (soundness.rs):
  exhaustive fiber-domain factorization census `|X| -> |im D| -> |im R*|`
  with §12.9 witness pairs. §12.6 checker (lumpability.rs): the carrier is
  ALL reachable viewer-decision nodes (world x record) from every fiber
  world plus an absorbing terminal, `o` = field-play segments between
  viewer decisions (exactly one trick resolves per segment, asserted), `r`
  = that trick's signed `ScalarValuation` increment, field = the fixed
  uniform-legal chance law; every kernel row is asserted to sum to exactly
  1 and the carrier is cross-checked worldwise against S3's
  `InfoPartition` (records and pooled-world counts agree; h0t6 carrier =
  90 roots + 648 future nodes = 738). Checked exhaustively -- no sampling,
  no bounds -- on all 13 trick-6 kernels. **Run results** (walt-tier pins,
  `tests/synthesis_run.rs`): registry = team/holder fact per pool tile +
  valued-tile beater counts (13 atoms at t6, valued = highest-count
  unseen). Static axis (subsets <= 4, all three targets: q_points, action,
  parametric): minimal sound subsets range size 0 (h3/h6/h12, single-class
  targets) to 4; h0 -- the §14.4 design kernel -- and h11 (q_points,
  parametric) are UNSOUND at every size <= 4: walt's holder-shaped
  vocabulary does not reproduce 3A's four-atom result (the 3A atom
  semantics are lost; blocked pin, `walt/DISCREPANCIES.md` "exp3A
  descriptor pin"). Trick-5 h0 (fiber 1680, registry 19, subsets <= 3):
  UNSOUND for both scalar targets -- the §12.3 ceiling reappears one
  horizon deeper. Dynamic axis (7 candidates x 13 kernels): the static
  passenger ALWAYS fails condition 1 (its frozen state merges each world's
  root with that world's future nodes, where legal sets differ --
  structurally forced, pinned on h0); chassis alone, +team facts,
  +beaters, and every minimal-sound static winner fail condition 2
  (kernel-mass witnesses) on every kernel where they compress; but
  **chassis+holder-all is LUMPABLE and nontrivial on all 13 kernels**
  (h0: 738 nodes -> 366 classes; corpus totals 5,887 nodes -> 2,857
  classes, 3,030 merged) -- the semantic-state projection that remembers
  who *holds* what but forgets who *played* what and in which order is a
  genuine closed-update quotient, and adding beater counts changes no
  class (they are a function of the holder map). Honest summary: at this
  candidate-space size, the only lumpable skeletons found are exactly the
  world-reconstructing ones; every strictly coarser candidate loses
  predictive sufficiency -- §14.7's "it reconstructed the world" recurs on
  the dynamic axis, while the genuine compression found lives in
  *history-forgetting*, not state-coarsening. Deterministic run, no seeds.
- **S4.5: unblock the exp3A pin.** The atom semantics thought lost were
  rescued to `walt/probes/exp3a/` (commit 9357536): `lambda_probe_v3.py`
  Part 1 holds the full 22-atom registry incl. `comp41`/`s3max2`. Port the
  registry semantics into `walt-skeleton/src/atoms.rs`, un-`#[ignore]` the
  pin, reproduce 90 -> 33 -> 8 through walt's own checkers, and clear the
  DISCREPANCIES entry. High information either way: S4's holder-shaped
  registry was UNSOUND where these four atoms succeeded, so they
  demonstrably carry control-shaped content — they seed the lesson
  vocabulary.
- **S5a: the regret walker (conflict generator).** Given a transcript
  (receipt hand or factory game) and a seat: at each decision point, the
  fiber-expected value of every legal action on `scalar.rs`'s solver, under
  declared knobs (continuation operator from the S3 registry; weighting =
  uniform-over-fiber for now). Output: per-decision regret, localization
  ("lost at the deal" as a provable verdict), and worldwise-dominance
  detection (the semantics-free grade). Exact rationals; sampled fibers
  (kernel sampler exists) only where enumeration is infeasible, marked.
- **S5b: the Lesson type + generalizer.** Lesson = implicant over the atom
  vocabulary -> graded, labeled verdict, with its conflict of origin and
  certificate. Generalizer = greedy constraint-dropping, re-verifying via
  the existing exhaustive checkers at each widening; the returned witness
  ends the widening and names the load-bearing constraint. Deliverable:
  measured basin sizes on the 13-kernel corpus — the falsification point
  for the whole direction (tiny basins = report and rethink).
- **S5c: the loop.** Lesson DB with watched-feature indexing, rent
  collection (pruning/regret reduction measured on the corpus), deletion,
  restart-with-retention in the synthesis loops, §16.11 certificate
  emission with the Python probes as independent checkers.
- S6+: walt-factory corpus at scale (all 9 declarations, deals + play —
  finally exercising DT and NT), the dynamic skeleton search proper with
  richer update-law vocabularies (the S4 result says: search coarsenings
  of the semantic state that keep kernel agreement, e.g. suit-profile
  quotients, not root-fact tuples), seat chassis wiring (four seats, full
  hands), lessons flowing between all of it.

## Open decisions deliberately deferred

- Support normal form: reimplement per foundation spec vs. thin port of
  rob's (lean: reimplement; greenfield, and the spec is the authority).
- Rational width: start i128-backed `Ratio`; escalate to BigRational where
  denominators demand (fiber-weighted expectations at H≥5 will).
- Seat chassis process model (four independent seat states) lands with
  walt-strat, not before.
- Wiki pages: only when something earns a tier above exploratory.
