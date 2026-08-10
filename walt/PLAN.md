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
- **S4.5 (2026-08-10): exp3A pin unblocked.** The atom semantics thought
  lost were rescued to `walt/probes/exp3a/` (commit 9357536):
  `lambda_probe_v3.py` Part 1 holds the full 22-atom registry incl.
  `comp41`/`s3max2`. **COMPLETE**: `walt/ci/check.sh` PASS; the registry
  semantics are ported as `Exp3aAtom`/`Exp3aContext`/`Exp3aDescriptor`
  (atoms.rs; reimplemented from the probe's definitions at the partition
  level -- walt's `Decl::rank` is order-isomorphic to the probe's ranking
  and atoms feed only equality cells and strict comparisons; marked
  `StaticPassenger`, §14.4 is a static result), the search driver is
  generic over both vocabularies (synth.rs), and the blocked test is
  replaced by three green ones (harness.rs): the context derivation lands
  on the probe's constants (valued 4-1, decisive 2-1, suit 2, boss 2-2,
  floor 2-0, 22 atoms); **D = {comp, focal-max, team(2-0), team(4-2)}
  reproduces 90 -> 33 -> 8 through walt's own §12.1 checker** (and stays
  sound at 33 cells for the 3-class action target); the full <= 4 search
  reproduces the probe's whole Part 1 record -- minimal size 4, exactly
  eight solutions ({comp | comp-rank} x {holder | team}) at 69/53/53/33
  cells, both targets. Corpus-wide (walt's generalization of the
  vocabulary parameters -- decisive tile = viewer tile whose led context
  touches the most pool tiles -- walt-tier, not probe-backed off-design):
  the control registry breaks EVERY ceiling the S4 holder registry hit --
  h0 sound at size 4 on all three targets, h11 at size 4 on all three
  (were UNSOUND at <= 4), and minimal sizes drop elsewhere (h1 4 -> 3,
  h5 q_points 4 -> 3, both via `comp`) -- pinned in
  `tests/synthesis_run.rs`. DISCREPANCIES "exp3A descriptor pin" moved to
  reconciled. The four §14.4 atoms demonstrably carry control-shaped
  content the holder vocabulary lacks; they seed the lesson vocabulary.
- **S5a (2026-08-10): the regret walker (conflict generator).**
  **COMPLETE**: `walt/ci/check.sh` PASS; 3 new tests (~6.5s CI cost: 1
  walt-kernel exhaustive constructor validation, 2 walt-factory corpus-pins
  + byte-frozen fixture). New **walt-factory** crate (the crate map's
  factory layer — the walker is the factory's first module), plus three
  supporting pieces below it: `walt-kernel/src/decision.rs`
  (`ReceiptDecision` — the §2.1 kernel at *arbitrary* decision points:
  mid-trick pool removal, in-partial-trick led-context void revelation,
  true-world view for validators), `ScalarPi::action_values` (mid-trick
  solver entry; values are §8.1 future increments — the unresolved current
  trick counts in full at resolution, completed tricks are the caller's
  action-independent bank), and `walt-strat/src/label.rs` (`OperatorLabel`
  {PI, H, C, F}, `WeightingLabel` {UniformOverFiber} — declared knobs as
  data). Constructor validated three ways over all 13 x 4 x 7 = 364
  decision points: (a) exact `Kernel` equality with `from_receipt_trick` at
  all 91 viewer-lead trick starts; (b) the receipt's actual deal inhabits
  the fiber at every decision point; (c) fiber counts monotonically
  nonincreasing along every seat-transcript (the restriction-injection
  argument is in the test header). Walker, per decision: exact fiber count
  (counting DP) always; exhaustive enumeration at/below a declared
  threshold, recorded-per-decision-seed exactly-uniform samples above —
  marked `Sampled`, never silently; per-world exact scalar-PI action
  values, fiber expectations as exact rationals (integer sums over an exact
  count — under a *uniform* weighting no LCM accumulation arises, so i128
  suffices; a future non-uniform weighting is the BigRational boundary);
  regret of the transcript action; strict worldwise-dominance pairs; and an
  all-actions-lose flag against the real made/set condition (focal points =
  (42 + diff)/2 vs the bid). Per (hand, seat): total regret decomposed by
  decision — exact because each decision's banked term is
  action-independent (§8.5 future-increment mode) — and the localization
  verdict "lost from tX pY under PI semantics, worldwise", constructed only
  from exhaustive bases. Conflict vocabulary
  (`walt-factory/src/conflict.rs`): one sum type — `Regret` beside the S4
  `Purity`/`Lumpability` witnesses — and `Grade` =
  (dominance-vs-expectation) x (operator, weighting) carried as *fields*,
  so no verdict is quotable without its labels. Parallelization: scoped
  std threads over fixed work chunks, per-thread solver caches; every
  reduction is an exact integer sum or boolean lattice op (associative +
  commutative), so results are schedule- and partition-independent (~15x
  on this box). **S5a design notes (walt-math review, adopted)**: grades
  are pairs — worldwise dominance is weighting-free, NOT semantics-free
  (the exp4 record's `G^cont(2-1) ≡ 0` vs `G^cont(0-0) > 0` is the
  action-specific gluing-gap mechanism; PI dominance never implies Q^H
  dominance); conflicts carry grade + operator + weighting labels from day
  one, sampled always marked; "lost at the deal" is always labeled — the
  label-free verdict needs §7.7's max-min imperfect-information operator,
  which is not built and not implied. **Corpus results** (CI config:
  tricks 3-7, exhaustive <= 40,000 worlds, 64-draw recorded samples above;
  pins in `walt-factory/tests/data/ci_corpus_pins.txt`, pip-trump-only
  caveat restated there): 214/260 walked decisions are zero-regret
  (82.3%); 18/52 seat-transcripts have zero total regret; 162 strict
  dominance pairs; 11 decisions with a worldwise-dominated chosen action
  across 9 transcripts (correction, S5b: this line first said 12; the
  committed pins sum to 11); 25/52 transcripts earn a labeled lost verdict
  (9 declaring, 16 defending; earliest-walked-trick t3: 7 of them). The
  designated byte-frozen artifact (walt's first receipt-shaped output,
  exploratory tier, marked in its own header) is hand 0 / seat S1 (the
  bidder: ply-0 first decision = the dealt kernel), whole transcript at
  the fixture config: total regret 10079/672, one worldwise-dominated
  choice (t5: 3-2 chosen, 2-1 dominates), verdict "lost from t3 p3 under
  PI semantics, worldwise". Full-corpus walk (threshold 10^6, 2,000
  draws, whole transcripts) lives in the `walk_corpus` release binary;
  its run is summarized in `walt-factory/results/`.
  **S5a part 2 (2026-08-10): the full walk, the kill, the resume, the
  adjudication alignment.** The first full-corpus run died at ~22/52
  transcripts, mid h5 S2 — memory pressure, not a fault: no panic (the
  workspace aborts loudly on overflow), no crash report, the last written
  block internally complete, and the per-thread solver caches at
  horizon-6/7 decisions measured at 14-19 GB for a single decision.
  Fixes, both provably output-neutral: `ScalarPi::trim_cache` with a
  per-thread bound in the walker (4M entries, ~4 GB total — cache entries
  are exact values of projected states, so trimming costs only
  recomputation), and `walk_corpus [start_hand [start_seat [max_pairs]]]`
  resume (per-decision sample seeds are a fixed function of (base seed,
  hand, seat, trick), never of walk order). Resume verified before
  restarting: h5 S0 re-walked under the new binary is byte-identical to
  the killed run's block, the wall-ms field aside; the corpus artifact is
  assembled from the two part files in `walt-factory/results/`
  (`full_walk_2026-08-10_assembled.txt`, provenance header inside).
  **Full-walk results** (52 whole transcripts, threshold 10^6, 2,000-draw
  recorded samples above it — 276/364 decisions exhaustive, 88 sampled
  and marked): 282/364 decisions zero-regret (77.5%); 9/52 transcripts
  fully zero-regret (was 18/52 on the tricks-3-7 CI subset — the early
  sampled tricks carry real regret); 82 conflicts (41 exact-expectation,
  31 sampled-grade, 10 worldwise-dominance); 12 worldwise-dominated
  chosen actions; 25/52 lost verdicts (9 declaring / 16 defending),
  earliest by trick {t2: 2, t3: 7, t4: 2, t5: 3, t6: 6, t7: 5} — h0 S1
  and h12 S3 are worldwise-lost under PI by their *second* decision.
  Largest transcript regret: h11 S3 defending, 87569113/2494800 (~35.1
  valuation units) — and still verdict-lost from t6, so the throwaway
  and the doom coexist. Runtime: part 1 ~70 min for 22 pairs (unbounded
  caches, died); part 2 20 min for the remaining 30 pairs at 17% peak
  memory (vs 39%) — the cache bound made the walk faster, not slower.
  **Adjudication alignment (the S5b walt-math amendments applied to the
  walker):** (1) the stored dominance primitive is now the world-count
  triple per ordered action pair (`DecisionRecord::triples`; T/W/S/I
  derived, conflicts fire on W; `dominance`/`chosen_dominated` kept as
  derived-at-construction views for cross-module stability). Cross-check:
  the walker's t5 triple for (2-1 over 3-2) on h0 S1 reads (488/1192/0) —
  exactly the S5b basin triple for the same conflict. (2) The
  localization primitive is the live-world count (`live_worlds`: worlds
  whose best action still clears the win threshold; all-actions-lose =
  live 0), with the win condition restated in the adjudicated
  role-shifted strict-< form — semantics unchanged, and the regenerated
  CI pins are byte-identical, which pins the invariance. (3) Operator
  naming: every rendered artifact now carries "operator PI = the product
  pair (C, minimax-omniscient) of §10.3 x §10.8". The `Grade` enum still
  spells the label `OperatorLabel::Pi` because S5b's byte-frozen lesson
  fixture renders origin-conflict grades through it — swapping that field
  to `OperatorPair` is a coordinated regeneration with S5b, flagged for
  the orchestrator, not silently done.
- **S5b (2026-08-10): the Lesson type + generalizer.** **COMPLETE**:
  `walt/ci/check.sh` PASS; 4 new tests (~0.1s CI cost — the domain builds
  in ~40 ms and each generalization runs in milliseconds), the
  `lesson_run` example (the measured run, written to
  `walt-factory/results/lesson_basins_2026-08-10.txt`), one byte-frozen
  lesson receipt (`tests/data/lesson_h0_S1_t5.txt`). New walt-factory
  modules `lesson`/`basin`/`generalize`/`lesson_report`; placement: the
  lesson machinery consumes conflicts, the corpus, and both S4 checkers,
  so it lives in the factory — the one walt-skeleton addition is
  `Exp3aContext::try_eval`, the vocabulary's honest partial-evaluation
  API, so atom semantics stay owned where they are defined. **Lesson** =
  two-sorted implicant (decision cells: hand/seat/decl/role/horizon/ply;
  atom cells `atom = value` over the union vocabulary — holder, team, and
  beater-count facts per pool tile plus the ten exp3A control shapes,
  instantiated kernel-generically through the S4.5 context derivation and
  *partial* where an atom's precondition fails) -> graded, labeled
  verdict, with its origin conflict (whose own grade travels separately —
  a sampled origin is never upgraded by a worldwise-verified lesson), the
  full widening trace with every terminating witness (complete deal +
  value row), and the measured basin. Quantifier placement is part of the
  verdict type: refutation and win verdicts hold per matching (decision,
  world) — atom cells select worlds inside each fiber; the checker
  verdict (not-lumpable, the conflict-species-spanning form) holds per
  matching decision with atom cells read fiber-valid. Actions are named
  by kernel-generic selectors (decisive / max-count / min-count /
  concrete-tile fallback; seeding picks the most generic selector that
  reproduces the origin's tiles). **Generalizer** = greedy
  drop-in-declared-order (identity cells, then atom cells, then the
  public frame) with witness-terminated widening, re-verified
  EXHAUSTIVELY over the whole declared domain at every step, plus one
  reverse-order restart — a good cut cheaply, never a minimal core;
  vacuous verification is allowed and reported as such. **Domain** (a
  parameter): tricks 5-6, all 13 hands x 4 seats x all plies = 104
  decisions / 23,790 worlds, every fiber enumerated, every world's exact
  PI action values and every atom column precomputed. **Measured run**
  (17 lessons: 11 refutation seeds = every dominated-chosen decision of
  the CI-config walker corpus, 5 win-form lessons where an origin action
  is worldwise-optimal, 1 checker seed; S5a's log line said "12
  dominated" — the committed pins and the regenerated walk both sum to
  11, corrected in place above): **refutation basins {0 x6, 1 x2, 2 x2,
  3 x1} decisions; win basins {0 x1, 1 x2, 2 x1, 5 x1}; the chassis
  §12.6 lesson widens to the empty implicant with basin 13/13 eligible
  lead kernels.** Falsification verdict, honestly: dominance-lesson
  basins on this domain are TINY — median 0-1 decisions. Six of eleven
  refutations never reach the domain: four are t3/t4 origins whose
  load-bearing `horizon` cell pins them to their own horizon, two are
  tile-anchored pairs never jointly legal at tricks 5-6. But the
  direction is not dead: the transfer that exists is exactly
  selector-shaped — the h1 S0 lesson ("at ply 2, 5-2 beats the decisive
  tile" / "attains the optimum") crosses hands h1/h6/h7/h9 (up to 5
  decisions, 651 worlds), and a *sampled* t3 conflict's lesson
  re-verifies worldwise at h1 S2 t6 (cross-horizon). The atom vocabulary
  never ends up load-bearing: at these horizons its fiber-constant cells
  are mostly degenerate (zero-beater vectors of masters, occasional
  `opp-beaters=0`/`bestkeep=true` at horizon 4-5) and all drop —
  identity and frame cells carry every lesson. Next levers, in order:
  world-selecting (non-constant) atom cells introduced on widening
  failure (cut refinement proper, S5c+), and richer domains (full-hand
  decision points once the corpus walk lands). **walt-math design
  amendments (2026-08-10, adopted mid-build):** (1) the dominance
  primitive is the world-count triple (#gt, #eq, #lt) — stored per
  matched decision and summed per basin; T/W/S/I are *derived* views
  (§9.6: conflicts fire on W, T is never a conflict; ties never
  collapsed, S3.5 precedent). The triple caught a real misclassification:
  h4 S1's "refutation" basin is (0, 1686, 0) — class T, an
  interchangeability-at-label statement, and its receipt now carries
  §10.9's caveat (label-level payoff ties do not make actions
  interchangeable for the seat). h4 S3's basin reads (1, 1889, 0) —
  class W by a single strict world, visibly near-degenerate. (2)
  Operator labels are the (focal-info, field) product pair from §10.3 x
  §10.8, never a single rung: lesson grades carry `OperatorPair`, the
  walker's scalar statistic is named (C, minimax-omniscient) (S5a's enum
  spells it "PI"; an S5a part-2 amendment aligns the walker), and §12.4
  makes basins label-relative — every basin line restates its full grade,
  and origin conflicts keep quoting their own S5a-recorded labels. (3)
  Win lessons are per-world sufficiency ONLY, never "guaranteed"/"safe"
  at any seat-facing label (§7.6 fusion gap) — enforced in the verdict's
  rendering and type docs; the one exported corollary runs the other way
  (worldwise *loss* negates guarantees, reserved (H, minimax-omniscient)
  label — noted, not built tonight). Remaining design calls flagged for
  review: equality-only cell language; basin-0 lessons keep their
  vacuously-emptied final implicants (the trace says why); checker
  eligibility (ply 0, horizon <= 2) is declared applicability, not
  implicant content; restart policy = forward + reverse only, kept by
  (decisions, worlds) lexicographic.
- **S5b.1 (2026-08-10): design-call adjudication folded in.** walt-math
  adjudicated the seven S5b design calls: 1, 3, 4, 7 CONFIRMED as built;
  2, 5, 6 AMENDED and implemented (`walt/ci/check.sh` PASS; 1 new test,
  CI cost unchanged at ~0.1s; results regenerated to
  `results/lesson_basins_2026-08-10_r2.txt`, the committed first table
  left in place). (2) **Purpose-split basins** (§9.6 is purpose-relative;
  derived views only, verification unchanged): every refutation basin
  splits into a refutation subbasin (strict-somewhere decisions — the
  pruning-grade content) and a safe-substitution subbasin (every matched
  decision — weak dominance verified, zero loss per matched world,
  T-coverage counts), both in every receipt and pin line; e.g. h4 S3 t5
  p2 = refutation 1 / safe-substitution 2, h4 S1 (the T lesson) =
  refutation 0 / safe-substitution 2. S5c rent will be purpose-specific.
  (5) **DomainSpec-gated application** (TRUST-01 shape: a verdict's scope
  is its verified domain; the must-fix): the new application entry point
  `lesson_applies` — S5c's pruning hook — checks the lesson's stored
  `DomainSpec` before reading the implicant, so applying a lesson outside
  its verified domain is unconstructible without re-verification; the
  generalizer itself is unchanged (vacuous traces stay). Gate test: the
  h2 S2 t4 empty-implicant lesson does NOT apply at its own trick-4
  origin even though both its tiles are legal there; the h0 S1 t5 lesson
  applies at its in-domain origin (whole 1,680-world fiber). (6)
  **Per-carrier denominators** (§11.1: measures are carrier-relative):
  every basin line prints covered/eligible on the verdict's own named
  carrier (selector-resolvable decisions x fiber worlds; lead-kernel
  trees at ply 0, horizon <= 2), with the full domain as labeled context
  only — the checker lesson's line is now 13/13 lead-kernel trees, never
  13/104. The carrier view separates two basin-0 kinds the flat
  denominator hid: empty carrier (four tile-anchored refutations whose
  tile pair is never jointly legal at tricks 5-6: eligible 0/0) vs
  inhabited-but-unreached (h3 S3: 0/53 eligible; h4 S0: 0/9 — the
  load-bearing horizon cell keeps them at their own horizon).
  **Adjudicated headline: falsification deferred, falsifier sharpened.**
  S5c's first milestone is re-scoped to BE the falsification test: a
  t3-4 domain where the atoms are discriminative, order cells so pins
  can relax instead of vanish, purpose-split rent, DomainSpec-gated
  application. Standing label note: everything so far is at
  (C, MinimaxOmniscient); basins are label-relative (§12.4), so any
  positive here is a hypothesis about (H, FixedUniformLegal), not a
  result. Also adjudicated: grade non-inheritance (sampled origin /
  worldwise domain claim) confirmed correct as built; the tile-anchored
  transfer inversion (concrete 5-2 crossing h1/h6/h7/h9 where abstract
  selectors did not) flagged as §14.7-consistent and kept visible — the
  receipt's selector evaluation basis stays mandatory.
- **S5c-m1 (2026-08-10): the falsification test proper.** **COMPLETE**:
  `walt/ci/check.sh` PASS; 1 new CI test (the tricks-3-6 fiber-cap-3,000
  subset domain: two t4-anchored seed pins + rent-coherence invariants;
  whole lesson suite ~1 s), the `falsification_run` example
  (`results/falsification_2026-08-10.txt`), the S5b-machinery record
  regenerated as `results/lesson_basins_2026-08-10_r3.txt` (r1/r2 left
  as committed). Built, per the adjudicated re-scope: (1) **order
  cells** — `horizon` and the registered numerics (`beaters-total(d)` =
  the beater vector summed; `opp-beaters`) enter implicants as bound
  PAIRS (`>=` and `<=`, together the equality; §3.3-style registered
  predicates with declared partial semantics — a bound over an undefined
  numeric is unsatisfied). The generalizer RELAXES a bound stepwise,
  witness-terminated; a refuted relaxation holds the bound at its last
  verified value, named (`horizon>=5` held on h11 S1's win lesson) — the
  S5b failure mode where four zero-basin lessons died on a horizon pin
  equality could only keep or delete is gone. (2) **Cut refinement** —
  on a failed widening the generalizer may INTRODUCE a world-selecting
  cell from the origin's registered vocabularies (constant equality or
  interval bound across the pairs verified under the pre-widening
  implicant, excluding the witness; at most 4 per pass, first candidate
  in vocabulary order, fully rolled back if the widening still fails),
  traced as `Introduce` steps distinct from drops. (3) **The t3-6
  domain** — `DomainSpec` gains a fiber cap with EXCLUSION semantics
  (never sampling; the excluded count travels in every receipt, and the
  application gate treats an excluded fiber as out of scope): 179
  decisions / 924,813 worlds at cap 40,000 (29 in-range decisions
  excluded, all trick-3), precomputed exhaustively in ~5 s at 12
  threads. (4) **Milestone-1 rent** — `measure_rent` through the gated
  application path, purpose-specific by type: refutation rent = applied
  + strict-applied + exact mean matched-world improvement (never paid in
  T-coverage), win rent = worlds covered + actions pruned, checker rent
  = applied; DB/deletion/restart machinery deliberately not built (m2).
  **The measured run** (16 lessons: 10 refutation + 5 win + 1 checker;
  seeds = every dominated-chosen walker decision at exhaustive threshold
  100,000 — where h4 S1 t3 p2's sampled-basis dominance did NOT survive
  exhaustive re-examination at 90,090 worlds, so S5b's 11 seeds are 10
  at grade: never-quote-above-grade, demonstrated on our own seed):
  **On the t3-6 discriminative domain at (C, minimax-omniscient), the
  atom vocabulary is expressively sufficient** (walt-math's sharpened
  m2 phrasing — sufficiency, not necessity): most final implicants are
  pure atom cells with the frame fully dropped, several transferring
  across decisions and hands — including transfer that excludes the
  lesson's own origin (h1 S2 t3: `beaters-total(1-0)<=1`, basin 3/89
  frame-compatible, its excluded-fiber origin not among them). Necessity
  holds for the cross-decision cases by the S5b contrast (the frame-only
  language could not reach them). A minority of atom implicants are
  equality-in-disguise re-descriptions of their origin (re-pinned bound
  pairs, e.g. `beaters-total(3-0)=2` on h3 S3) and carry no selection
  content beyond the frame — flagged `re-pinned` and excluded from
  selection counts, giving the honest tally **10 of 16 lessons with
  selecting atom cells** (12 gained cells by cut refinement). The
  basin/frame-compatible rates (1/64, 3/89, 3/130, 1/14, ...) show the
  latent cells, not the frame, doing the selection where it happens.
  Honest caveats: absolute basins stay small (refutation {0 x1, 1 x7,
  3 x2}; win {1 x2, 3 x2, 5 x1} — few conflicts, deeply analyzed, per
  the declared regime); with introductions doing the work, the surviving
  (drop-survivor) lists are mostly empty while the selection content
  sits in `introduced` — both printed everywhere.
  Rent: each single-decision refutation basin's improvement equals its
  origin regret exactly (65/112, 1243/1225, 1163/840, ... — an
  independent cross-check against the S5a fixture); cross-decision
  basins exceed it (h1 S2 t3: 16/15 over 3 applied); the largest win
  rent covers 74,382 worlds / 109,032 pruned actions (h11 S1 t3).
  **Falsification verdict, in the spine's vocabulary: the direction
  survives its designed falsification point** — with relaxable bounds
  and cut refinement, conflicts generalize into atom-vocabulary lessons
  that pay measurable purpose-specific rent; the remaining pressure is
  basin SCALE, which points m2 at the database economy over more
  conflicts, and at (H, FixedUniformLegal) re-measurement (standing
  note: everything here is at (C, MinimaxOmniscient), label-relative
  per §12.4). Design calls flagged: numeric bounds replace per-slot
  beater vectors in the implicant language (the vector remains column
  substrate); introduction candidates are equality-on-constant or
  interval-on-defined; INTRO_BUDGET = 4 per pass; bound pairs are
  relaxed independently (no joint ladder).
- **S5c-m2 (2026-08-10): honesty amendments + the (H,
  fixed-uniform-legal) re-measurement.** **COMPLETE**: `walt/ci/check.sh`
  PASS; 1 new CI test (an H re-measurement pinned end-to-end with exact
  rationals; lesson suite now 7 tests, ~0.8 s); new
  `walt-strat/src/hidden_scalar.rs` (the scalar H solver:
  pooled-information viewer maximization against the §7.4 fixed
  uniform-legal field, mid-trick roots, unit-fraction particle weights so
  rational work concentrates at trick resolutions, budgeted — an
  over-budget solve returns nothing, exclusion never sampling) and
  `walt-factory/src/label_transfer.rs` (the re-measurement driver);
  results in `results/label_transfer_2026-08-10.txt`, the m1 records
  regenerated under the amended vocabulary as
  `falsification_2026-08-10_r2.txt` / `lesson_basins_2026-08-10_r4.txt`
  (committed revisions untouched). **Part A, the m1 adjudication
  amendments:** witness exclusion for introduced cells confirmed
  enforced by construction (an in-interval witness falls through to no
  candidate, never a budget-spending no-op), stated in the generalizer
  doc and CI-asserted per trace (`cell_holds_at`); vocabulary renamed
  load-bearing -> surviving (beside `introduced`; selecting = the
  union); re-pinned pairs flagged, rendered as the derived equality
  `atom=k` (never an interval), and excluded from atom-selection counts;
  intro-budget spent/4 on every pin and receipt; the control-bias
  annotation travels with every capped domain description (all 29
  exclusions are trick-3, and fiber size anti-correlates with focal
  control — exp5 covariate — so the excluded set skews low-control).
  Registered prediction (failure ledger): when cut refinement first
  meets a world pair no beater TOTAL can separate, the next registered
  numerics are team-split beaters (focal-side / opp-side per tile) —
  §14.7's seat-swap mode is invisible to totals by construction;
  predicted now so the ceiling confirms rather than surprises. **Part
  B, the label-transfer measurement** (before any DB economy: label
  transfer is the inventory's weakest load-bearing assumption, §12.4,
  and its outcome defines the economy's currency). At H the verdict
  quantifier changes shape by necessity — H values live on pooled
  information states, so a re-measured refutation is ONE inequality
  `Q^H(better) >= Q^H(worse)` per matching decision and a win is
  `Q^H(action) = max`, with atom cells read fiber-valid; both label
  coordinates move to the seat-facing pair while the root weighting
  stays the declared uniform-over-fiber. **Result: 10 survive, 0 fail,
  5 unmeasured of 15 value lessons — every measurable lesson
  transfers.** Refutations 7/10 survive (h0 S1: Q^H(2-1) = 80/7 >
  202/21 = Q^H(3-2); h3 S3: -75961447/3628800 > -557701759/25401600;
  H-ties hold weakly where present, e.g. h1 S1 t5 p3 at -623/360 both);
  measurable wins 3/3 (tile 5-2 exactly H-optimal at all five basin
  decisions across four hands). The five unmeasured: four budget-capped
  (fibers 16,632-34,650 at 10^8 particle-steps — capped, never sampled)
  and one empty basin; not-fiber-valid zero everywhere (every (C)-basin
  membership was already fiber-valid); the checker lesson is not
  re-measured (§12.6 already lives at the fixed field). **The inventory
  is NOT label-fragile on the measured subdomain** — the economy's
  currency can be seat-facing H rent for small-fiber lessons, and the
  capped big fibers point m3 at pooled-state H memoization, not at new
  mathematics. Honest boundary: survival is measured on each lesson's
  OWN basin (small sets), and nothing here promotes any (C)-graded
  claim — every H row carries its own label and quantifier.
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
