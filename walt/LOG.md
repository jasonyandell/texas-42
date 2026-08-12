# walt — session log

Full per-session build records, moved out of `walt/PLAN.md` (which keeps
one-line summaries and the forward plan). Same tier discipline: everything
here is exploratory; pins are regression pins, never axioms. Entries are
appended, never rewritten — corrections are recorded in place with their
provenance, as the S5a count corrections were.

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
  naming — the coordinated swap, step 1 (this session, post-1da8799):
  `Grade` and the walker's record/verdict types now carry `OperatorPair`
  (imported from the lesson module — legal intra-crate, since Rust
  modules may reference each other; hoisting the label grid to
  `walt-strat::label` as the long-term shared home is noted, not done,
  to keep S5b's file untouched mid-flight). Rendered grades read the
  product form ("worldwise-dominance at (C, minimax-omniscient)");
  walker fixtures and CI pins regenerated (the pins' only delta is the
  verdict wording "under (C, minimax-omniscient) semantics"; every
  number is unchanged). `walt-strat::OperatorLabel` remains as the
  single-rung registry enum for callers that mean a rung, not a pair.
  Step 2 (walt-s5b's half): regenerate the lesson-side frozen receipt —
  the one red test in the workspace is
  `lesson_pins::h0_s1_t5_refutation_and_win_lessons_match_the_pins`,
  whose fixture's origin line still reads "grade worldwise-dominance
  (PI)"; the regenerated text differs in exactly that line. The
  2026-08-10 walk artifacts keep their recorded "PI" strings — they are
  receipts of the run that produced them, with the mapping named in the
  assembled artifact's provenance header.
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
- **S5c-m3 (2026-08-10, daytime): the memoized seat currency, the economy,
  and the label-fragility discovery.** Three units, all committed
  (f7eaa05 m3-A, 97228ad m3-B, m3-C following), all adjudicated by a
  resident walt-math consultant across eight design forks plus per-unit
  post-hoc review; two builder agents (walt-m3a walt-strat-side, walt-m3b
  walt-factory-side) under strict file fences.
  **m3-A (the dag-v1 H solver):** `hidden_scalar.rs` gains
  `action_values_dag` — per-measurement-call cache, entries only at
  trick-boundary pooled states, key = (canonical weighted world-multiset
  × leader) with weights MANDATORY (pooled maximization depends on the
  weight profile; a weight-free key is unsound — walt-math Fork 1),
  gcd-normalized unit-fraction denominators as the projective normal
  form, exact rescale on hit, frame carried by table scope. Soundness
  argument as adjudicated: every entry carries the full ruled key with
  the prefix component empty; (boundary state, observed prefix) →
  mid-trick state is surjective determination, NOT bijection — convergent
  mid-trick states are foregone hits by design. Budget REDECLARED as
  particle-steps over the memoized DAG (semantics=dag-v1): hits cost zero
  by unit definition, deterministically — measurability is a function of
  declared inputs, never cache warmth (exclusion-determinism, Fork 2);
  every row carries the semantics identifier and tree-equiv (the exact
  tree-v0 cost of the identical computation). Value transparency
  CI-pinned: byte-identical Q^H vs the unmemoized walk on all 16
  m2-measured decisions (`tests/h_value_transparency.rs`).
  **The measurement (r2 + r3):** at dag-v1 10^8 every previously-measured
  value is byte-identical and the four big fibers are honestly still
  capped (r2); at dag-v1 10^9 — a budget change only, declared — all four
  lift (r3), and the outcome is **1 survive / 3 FAIL at
  (H, fixed-uniform-legal)**: the h1 S2 t4 refutation (Q^H(4-3) = 79/11 <
  Q^H(6-0) = 111269/13860 — at the seat's label the "refuted" action is
  better), the h1 S2 t4 win (best is 1-1 at 2183/270), and the h11 S1 t3
  win at its own origin (2-0 loses to 5-1 = -547477589/91238400; its two
  transfer decisions hold exactly). The h11 refutation survives.
  **Cross-validation receipt** (`h_tree_crossval_2026-08-10.txt`): the
  uncapped, unmemoized tree walk independently reproduced every
  per-action Q^H vector byte-identically on all four decisions —
  1.55e10/1.08e10/4.2e9/6.5e10 tree steps (the last ~62 min of exact
  rationals) vs 28×–122× fewer dag steps. The FAILS are label fragility,
  not a memoization artifact. **The honest headline: m2's "not
  label-fragile" was scale-limited** — the combined tally at the seat
  label is 11 survive / 3 fail / 1 empty-basin of 15 value lessons
  (quoted across two declared budgets, r2+r3 cited together), and every
  failure lives on a big early-trick fiber, exactly where the omniscient
  field (C) and the seat-facing field (H) diverge. §12.4's label-
  relativity caveat, which rides every basin line, was the truth.
  Registered-prediction check: the h1 S2 t4 fail separates on beater
  TOTALS alone — the §14.7 team-split-beaters prediction is NOT yet
  triggered.
  **m3-B (the economy):** four new walt-factory modules. `db.rs` —
  working set + append-only archive; lesson identity = projected content
  (canonical implicant cells, verdict, DomainSpec, operator-pair labels;
  GRADE IS NOT IDENTITY — quotable grade = max-grade archived derivation;
  re-derivations MERGE, demonstrated live: one entry, two archived
  derivations). `index.rs` — watched-feature index under a
  candidate-completeness contract (index excludes only what the gate
  provably refuses; every candidate still passes full `lesson_applies`;
  exhaustively cross-checked in CI: 179 × 16 = 2,864 pairs, 893
  candidates ⊇ 39 appliers at every decision). `ledger.rs` — dual ledger,
  H-primary, never summed (Fork 3): H rent is the pricing currency,
  (C) rent a recorded diagnostic; UNMEASURED IS NEVER ZERO — capped
  lessons are provisionally held and never advance deletion streaks;
  deletion = N=2 MEASURED-consecutive zero-rent epochs (capped epochs
  neither advance nor reset; evidence patterns cite epoch ids with gaps);
  per-row SINGLE-IMPLEMENTATION stamps with append-only clearance
  records — a deletion must cite only independently-cleared rows, and
  the first H-rent deletion is token-blocked until an independent H
  checker registers (the "never trust the solver" law in types);
  restart-with-retention keeps DB+archive+ledger, discards search state
  (memo tables are search state). Standalone rent priced, overlap
  recorded, never summed (Fork 7; 7 overlap decisions in the run).
  `certificate.rs` — §16.11's ELEVEN records per lesson: value content
  as per-decision multisets with the declared comparison protocol and
  the world-alignment-unchecked caveat; record 9 restored to per-world
  truth vectors in a canonical world order (the walt-math dependency
  check caught the compression); NOT-APPLICABLE records present-and-
  empty with reasons; per-record checker-coverage annotations, H rows
  honestly UNCHECKED-EXTERNALLY. 16 certificates emitted against the
  self-contained `docs/certificate-schema.md` (schema-v1, written for
  the future independent Python H checker).
  **m3-C (the re-priced economy, the m3 exit artifact):** the ledger
  wired to dag-v1 at 10^9 (`economy_2026-08-10_r2.txt`). All 15 value
  lessons now measurable; the three H-fails price as MEASURED ZEROS with
  their reason rendered ("verdict FAILED at the H label"), and the
  economy's deletion rule fires on all three zero-streak lessons — the
  empty-basin refutation and both h1 S2 t4 lessons — **each TRIGGERED
  and each mechanically BLOCKED** (no registered checker; the crossval
  receipt is deliberately NOT a registered checker — context only, the
  Python checker remains the only clearance path). The h11 win prices
  positive on its two holding decisions with its origin failure in the
  failed-count: per-decision honesty end-to-end. The seat's currency now
  actually prices the inventory, and its first candidates for deletion
  are exactly the lessons the seat's own label measured as worthless.
  **Deferred, named:** the independent Python H checker (m3's exit
  criterion for executing any H-rent deletion; hard precondition, in
  types); tree-walk per-action parallelism if big-fiber tree receipts
  become routine; the walt-genesis worktree reconciliation.
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
- **S5d (2026-08-10, evening): the re-tethering — back to the basis, the
  equivariance amendment.** Direction session with Jason; no build. (1)
  Fresh full read of v0.4 (all 3,820 lines) after Jason's diagnosis that
  the build had come untethered from the mathematics. Findings: §12.4 +
  §17.5 had already ruled worldwise-PI classes the wrong hidden-decision
  carrier — m3's label-fragility result re-confirmed a boundary the
  basis had drawn in ink; §14.8 conclusion 10 and §18 already name the
  dynamic predictive quotient as the target; §12.8's exposed-policy
  geometry (7,848 information states, zero revelation value, 15 exposed
  policies) is the standing down payment that the game is small on the
  right carrier. The S4 lumpability instantiation compared observation
  and feature labels RAW, so only world-reconstructors could pass —
  Jason's critique ("changing the domino changes the output; what must
  match is the output under the quotient") locates the gap precisely:
  v0.4 quotients the state side but never the interface alphabets. (2)
  Jason authored **§12.6A — equivariant controlled lumpability over
  declared role interfaces** (definition, theorem with proof,
  gauge-descent corollary, valuation stabilizer boundary, recovery of
  §12.6 at identity transports), delivered in-session; proofs checked
  (two reader notes recorded in its Appendix A: coherence scope over
  the action/observation transports, and the abstract-policy-class
  optimization boundary) and filed as
  `walt/math/equivariant_lumpability_v0.5.md` — the v0.5 track opens;
  v0.4 stays frozen. The theorem is count-free by design: the primitive
  outcome alphabet is the trick coordinate alone; tile anisotropy
  re-enters through transported roles; the §8 additive gauge acts on
  the quotient. (3) Direction reset (Jason's call): the goal is the
  LOSSLESS count-free equivariant quotient — situations identical up to
  declared transports, outcomes compared under the quotient, "hundreds,
  not 399M" — player- or analyst-facing. Infrastructure frozen: m4 (the
  Python H checker), S6 corpus-at-scale, and further economy lifecycle
  work all DEFERRED (the economy stays mechanically blocked, which is
  safe by design); no rescuing implementations. Next work: find a
  nontrivial (d, Θ) satisfying (ECL) on the existing probe kernels and
  count classes — counterexample-guided per §12.9, carrier chosen per
  §11.2, Scheme/Fix as descriptor language per §12.7. (4)
  `walt/math/implementers_guide.md` commissioned (subagent): the model
  + proved outcomes + implementation contracts + nonclaims, no proof
  chains — per Jason's spec. (5) Addendum, the **NO-RESCUE policy**
  (Jason): failures are counterexamples to carry back to the math, never
  things to fix/spin/assist with engineering — "if the whole thing falls
  on its face that's FINE"; don't verify in triplicate (the reference is
  thousands of lines of proofs); when independent mechanical verification
  is genuinely needed, use Lean, not Python (m4-as-Python retired; the
  lean/ path is paved at 42/42 P0 kernel-proved). Standing frame: the
  math has proven the object exists, not its utility — if utility is bad,
  that's a conversation, and the tools stay valuable for other
  explorations either way.
- **S5e (2026-08-10, late evening): the situation census — r1, r2, r3,
  and the broken wall.** The first measurement against Jason's bar
  (order-10^5 count-free canonical situations). Design `walt/CENSUS.md`
  + walt-math rulings `walt/CENSUS-RULINGS.md` (F1–F7, then the r3
  section) — every fork pre-adjudicated; builder fenced; full
  `ci/check.sh` gate green at the end (fast-iteration commits carried
  explicit deferred-gate caveats, closed by f9355bc). Carrier: all
  situations reachable from the 13 pip-trump trick-six kernels under
  primitive steps — 15,253 situations, 647 roots, uniform-legal field,
  count-free contract, bank-as-emission. **r1 (finest structural
  relabeling quotient): ECL PASS exhaustively** — the v0.5 OPEN item
  (existence of a nontrivial equivariantly lumpable (d,Θ)) resolves YES
  on this domain — 11,949 classes, 670 cross-kernel (same certified
  situation under different hands and different trumps), but **zero
  root merges** (647/647). **r2 (declared coarsenings c2 drop-double,
  c3 drop-beaten-table-tiles, c2+c3): all ECL PASS**, full-carrier
  classes down to 8,659, largest class 200 — and still 647/647 at
  roots; dividend exactly 1.000 at plies 0–2. Structural matching
  cannot compress decision-rich states on this corpus. **r3 (Jason's
  direction: the retrograde coarsest quotient — backward induction from
  hand-end over the graded carrier, content-addressed hereditary
  signatures (grade, actor offset; per-move (k, classification,
  successor class)), position-matching transports per walt-math's
  coherence amendment): full carrier 15,253 → 1,459 classes; roots
  647 → 306 with 132 merging classes; the trick-7 lead target alphabet
  1,275 → 63; class DAG by grade 306/406/360/213/63/63/32/16/1.**
  Receipts: r1-refines-r3 asserted in-run (HOLDS); independent ECL
  re-check over the r3 partition PASS (1,013 classes, 13,794 pairs per
  condition, 0 counterexamples). Scope caveats as mandated:
  dynamics-equivalence classes named by future cones (carrier-relative;
  no compact-description claim — v0.4 §12.7 open; not PI classes,
  §12.4). Results: `walt-factory/results/census_2026-08-10{,_r2,_r3}.txt`.
  Next rungs named: climb to trick 5 (the 1,680-world kernel) to
  measure root-class growth with depth — that curve tests the 10^5
  bar; and §12.7 compact descriptions for the 306 root classes (what
  IS a class, in words a player could read).
- **S5f (2026-08-10/11, night): the trick-five climb + the bar's true
  object.** Bar clarified by Jason: 10^5 = situations facing the
  trick-1 LEADER (seat-level), vs the ~399M-world fiber; saturation
  curve adopted as companion probe. The t5 run (same adjudicated r3
  construction, grades 12→0, full gate PASS): carrier 2,651,280
  situations / 16,112 roots (20M declared stop, not reached). **Roots
  16,112 → 12,924 classes (1.25:1 vs t6's 2.1:1 — root compression
  weakens going earlier); DAG 12,924/27,178/40,938/37,848/23,592/
  11,943/5,393/1,704/64/64/32/16/1 (peak inward at grade 10, collapse
  to the stable 63–64-class trick-7 alphabet); the t6-lead stratum
  inside this run: 179,936 situations → 23,592 classes** — the t6
  census's 306 was a 647-root keyhole on a ~24k inventory,
  carrier-relativity demonstrated exactly as the Q4 caveat warned.
  Saturation: t6 root curve near-flat (+3 final); t5 curve far from
  flat (fresh 1,680-world fibers still ~90% new classes) — the t5
  inventory is much larger than 12,924 and unconverged at 13 hands.
  Receipts: r1-refines-r3 HOLDS (2,001,355 finest classes nest);
  independent ECL re-check PASS (90,003 classes, 2,489,584 pairs per
  condition, 0 counterexamples). Focal-alignment caveat recorded
  (t5 focal = trick-5 leader; cross-run identity comparisons not
  quoted). **Interpretation on the record: world-level trick-1 roots
  extrapolate astronomically (~370× per trick at the lead stratum);
  the bar's object is the SEAT-level census — pushed beliefs over
  world classes (v0.5 conclusion 1 is the bridge); at trick-1 lead
  the seat's raw situation space is C(28,7) = 1,184,040 hands ×
  declaration, and order 10^5 up to lawful equivalence is exactly the
  plausible regime. The seat-level census is the next construction,
  to walt-math before build.** Results:
  `walt-factory/results/census_t5_2026-08-10.txt`.
- **S5g (2026-08-11, small hours): the railyard — pruning platform,
  P1 rock, and the parts catalog.** Jason's direction: every trick is
  the same machine ("the railway yard stacked on itself"); derive the
  yard once, prune it with the actual situation, search the remnant
  exhaustively — the platform for policies and beliefs. Three rounds,
  each walt-math-adjudicated (CENSUS-RULINGS.md sections: "railyard
  factoring — shaping"; "shape notion v2"):
  **(1) The pruning probe** (support-only restriction of the verified
  r3 class DAG by each real kernel's fiber; class-successor agreement
  re-asserted in-run): trick 6 median live sub-DAG **179 classes** per
  actual situation (range 28–453); trick 5 median **16,782 classes /
  30,812 edges** vs 241,762 raw situations (extremes 312:1 and 8:1).
  The exhaustive-search platform exists at t5 — milliseconds where raw
  walks were quarter-million situations. Support object only; belief
  concentrates, never widens. **(2) The yard, P1:** walt-math split
  Jason's periodicity claim into P1 (grade-freeness — PROVABLE) and P2
  (realized self-similarity — measurable); P1 discharged in code: ONE
  shared routine with no level argument reproduces r3's partition
  **byte-for-byte at every level of both rungs**. The yard is a
  refactoring of r3, inheriting its receipts; the first shape notion
  proved arity-blind cross-level (flagged in-run as vacuous, carried
  back, never reinterpreted — the leak is systemic: every
  unconstrained node has full-hand arity by rule). **(3) The suffix
  library (shape v2)** + hereditary rung: hereditary DEGENERATE for a
  structural reason (level-1 trees are forced single-leaf paths;
  hereditary shapes ARE the classes — no recurrence between shapes
  and classes). The recurrence lives in the PARTS: on the one
  non-carrier-limited step (t5 level 1→2), v2-open depth-3 library
  growth **31.6× vs class growth 368.6×, ω = 1.000 at all depths** —
  every one of level 1's 129 parts recurs inside level 2; 4,071
  open-depth-3 parts under 23,592 classes. Criterion answer on the
  clean step: **shared-machinery payoff SUPPORTED** — classes are
  menus over a compact shared parts catalog. Scope: one clean step;
  instrument tier; a t4 climb supplies the second step. All committed
  through aff5c8e; targeted suites green throughout (23 tests); full
  gate green on the t5 tree.

## S5h — 2026-08-11: the fiber-crush probe (the three-arm ladder)

Jason's morning frame: crush and guide fiber enumeration and the
level-1 seat platform exists — raw fibers are quick at 4 tricks
remaining, untenable at 6 (17.2M worlds), intractable at 7 (399M, the
deal itself); belief/policy iteration means countless re-evaluations.
Design (`walt/FIBER-PROBE.md`) sent to walt-math BEFORE build; the
rulings (CENSUS-RULINGS.md "Fiber-probe rulings", P-A1..P-A21)
reshaped it hard: the proposed raw-vs-class comparison was ruled a
STRAWMAN — the honest control is the ordinary transposition cache the
project already banked (`scalar.rs`), so the probe runs THREE arms
(A0 plain tree / A1 identity-key boundary cache / B r3-class DAG) and
the equivariance dividend proper is B:A1, never B:A0. Other
amendments: the object is the VOID-FREE CAPACITY FIBER Φ(C₀), a
declared cost domain, never "the seat's fiber" (feasible ≠ reachable;
P-A1); M2 "pruning" REJECTED as a mechanism (root collapse is its own
line); the re-weighting is the "declared fold weighting (timing
instrument)" — neither support nor belief, an aggregation argument
(§5.5); prefix sampling REJECTED for deterministic decimation
(i·g mod N, gcd = 1, freeze 8); Lemma V (per-world operator values
descend to r3 classes) stated and used as the P-A9 in-run receipt;
the amortisation claim NARROWED — re-fold amortisation is about the
per-world operator; the platform claim rests on treatment H (P-A14).

Results (`results/fiber_probe_2026-08-11.txt`, exploratory tier,
receipts P-A9 held bit-exact everywhere, 240/240 · 24/24 · 6/6):

- **The memoisation dividend is the manyfold, and it grows with
  depth**: A1:A0 wall medians 0.166 (n=4) → 0.024 (n=5) → 0.010
  (n=6) — 6× to 100×. The shared interior is real and compounds.
- **The equivariance dividend proper is NEGATIVE at build time**:
  B:A1 ≈ 4.0–5.0 at every rung (median 4.7/4.3/4.9) — the class DAG
  computes identical values at ~5× the cost of the plain cache.
  Structural reason, not implementation accident: class identity is a
  function of the future cone, so it is only computable AFTER full
  expansion — retrograde identity cannot short-circuit descent the
  way a state key can. **The class store is a storage/transport
  object (reuse across coordinates, hands, weightings), never a
  first-build accelerator.** Root collapse on evaluated sets: nil
  (240→239-240), as the rulings predicted.
- Interior collapse is real anyway: e.g. n=4 h0 carrier 1.50M
  situations → 129k classes (11.7×) — the inventory compresses; the
  build cost does not.
- **The H row is the day's surprise (fiber_probe_h file)**: cold
  treatment H — the seat's ACTUAL pooled hidden-information solve,
  m3's dag-v1 memoized scalar solver, uniform weighting — COMPLETED
  on the full 34,650-world void-free fiber at every eligible n=4
  coordinate (8 of 13; the rest have a non-declarer leading, out of
  the attempt's scope) in **7–17 s each**, inside the declared 200M
  particle-step budget; dag-v1 did 13–125× less work than tree-v0.
  The weighted-re-solve-over-the-fixed-DAG number (the one the
  platform claim rests on) remains UNMEASURED — machinery absent
  (uniform-only solver, Y3 K̄ integration unbuilt) — stated in the
  results file per P-A14.
- P-A2 gap lines: the seat's real support is sometimes a sliver of
  the capacity fiber (h8 at n=4: 1,200 of 34,650 = 3.4%).
- Extrapolation (P-A21, declared one-more-step law, hand 0): implied
  n=7 per-world A0 ~ 2,970 s — raw is dead, as expected; A1 growth
  ~10.5×/rung on near-cold small-W samples (full-fiber saturation
  unmeasured this run — declared stop; P-A16's full-fiber B side not
  run).

Machinery note: `build_r3` sized its grade ladder by
MAX_MATCHED_TILES (=12, r1's canonicalization-domain guard) — a
borrowed constant that capped the retrograde pass at 3 tricks; fixed
to the true bound (grade ≤ 28), capacity not semantics, no receipt
changes. Full gate green. Fast-iteration declared stops throughout
(per-rung hand subsets and W; every stop printed).

## S5i — 2026-08-11: the fiber-refinement probe (declared exclusion remnants)

Jason's hope, verbatim spirit: "everything that could happen, except X"
— refine fibers by declared exclusions; smaller fibers, not just fewer;
fine if it goes nowhere. Design `walt/FIBER-REFINE.md`; walt-math's
rulings (X-Q1..X-Q7, X-A1..X-A19) delivered a genuine theorem —
**Lemma X (zero-contribution excision)**: under the non-negative
q_trick valuation, deleting worlds whose Lemma-V value is zero leaves
the unnormalised objective AND its argmax exactly unchanged for every
information-consistent policy. ONE-SIDED: the value-max dual forces
nothing (X_val_max re-typed bite-only). Remnants typed as ANALYST
CONDITIONING (§6.8): a third thing besides support and belief —
excluded means neither impossible nor improbable; never re-optimise
over a remnant and call it a seat value; predicates carry their
quantifier (X_reach∃ vs X_conf∀); branch-level exclusion REJECTED
(changes the operator); pass-2 economics measured against the cheapest
STORELESS alternative (X-A13, anti-strawman); persistence discipline
accepted (X-A16..A19: append-only content-addressed, collision
verification across runs, cache-never-authority, cone-intrinsic
records only, warm-across-coordinates lawful but store-relative).

Results (`results/fiber_refine_2026-08-11.txt`, rungs n=4 all 13
hands / n=5 four hands, receipts ALL green — flag receipts at stride
97 through the independent A1 path agree everywhere, 32–3,134 classes
per coordinate; Lemma-X objective agreement asserted):

- **The machinery is essentially free**: every predicate pass over a
  built store costs 0.1–3.7 ms against multi-second builds — and for
  the value predicates the cheapest storeless route costs 200–960 ms,
  so **pass-2 over the store is ~100–1000× cheaper than the best
  alternative**, while reachability/confinement have NO storeless
  alternative at all (cone identity is not on a state key). Jason's
  multi-pass economics is measured and real.
- **The declared X's bite classes, not worlds, at these rungs**:
  X_val0 flags 0.1–3.5% of classes (zero-value sub-cones exist) but
  **0 of 3,120+96 evaluated WORLDS** across both rungs — no root hits
  value 0, so the Lemma-X excision fired on nothing. Structural
  reading: V* = 0 requires losing ALL n remaining tricks under
  world-informed play, which gets harder as n grows, and the corpus
  focal is the DECLARER (a strong hand by construction). X_reach∃(F0)
  is nearly vacuous at roots (~100% — you can always play badly);
  X_conf∀(F0) bites 0.6–16% of classes, ~0 worlds; X_val_max
  ("lay-down from here") bites 2–36% of worlds — real inventory
  signal, not excisable.
- **X-A15 honest row**: exclusion saves nothing at evaluation time
  once the store is paid (remnant summation ~µs either way) — a
  result per F7.

Where bite plausibly lives instead (open, not claimed): count-bearing
X's after role re-entry ("loses the bid"), non-declarer focal seats
(weak hands lose everything far more often), and the seat's real
voided support. The predicate ENGINE is proven either way; Lemma X
stands as a theorem regardless of bite. Full gate green.

## S5j — 2026-08-11: the endgame store (the symmetry-reduced tablebase probe)

Jason's direction: build from the end back and memorize it; pathfind
forward to known solutions — same fiber, different enumeration order
leveraging precomputed outcomes; precompute only the cheap end, fill
the rest lazily; "lots and lots of convergence late game"; walls
welcome. Design `walt/ENDGAME-STORE.md`; walt-math's rulings
(E-Q1..E-Q7, E-A1..E-A21) delivered the third theorem of the day —
**Lemma E (structural isomorphism ⇒ count-free value equality)**:
equal r1 canonical forms give a tile-bijection + seat rotation
carrying one remaining game to the other, so every count-free fold is
equal — REPLACING the proposed r1→ECL→r3 chain (rejected: ECL is
checked, not proved, and carrier-scoped). Attribution mandatory
(E-A1): this measures the STRUCTURAL transport dividend (a
symmetry-reduced tablebase), not the r3 machinery; S5h's negative
stands unrescued. Hard scope: COUNT-FREE ONLY (E-A2 — the store is
invalidated wholesale if count re-enters). The floor's honest
competitor is CLOSED-FORM last-trick resolution (E-A9). First
implementation of the persistence discipline (freezes 14–17;
gitignored cache; cold regenerate path for every headline; E-A4 hit
receipts re-expanded to TERMINALS).

Results (`results/endgame_store_2026-08-11.txt`,
`endgame_floor_2026-08-11.txt`; every receipt green — 4-arm bit-exact
equality at all 17 coordinates, 1,685 hit receipts re-expanded to
terminals all bit-exact):

- **The tablebase arms LOSE at evaluation: T2/T3 = 1.57–2.69× SLOWER
  than the plain A1 cache at every coordinate.** E-A10 attribution is
  unambiguous: canonicalization dominates (~4.6 µs per form vs ~0.1 µs
  per state-key probe); under an A1 memo the subtree a hit saves is
  already collapsed, so the per-hit saving is smaller than the per-
  state canonicalization cost.
- **The convergence Jason smelled is REAL**: 830,399 form-hits across
  the traversal (~38% of grade-8 boundary probes; up to 73% at h3) —
  relabeling-symmetric repeats a state key cannot see. The negative is
  about WHERE to spend it (evaluation speed), not whether it exists.
- Warm cross-coordinate increment: small but nonzero and growing
  (n=5 h1: 8,580 warm hits vs 2,966 cold). Store: 1,358,231 level-2
  records after 17 coordinates, growth near-linear — saturation NOT
  reached (store- and order-relative, E-A20).
- **T1' (closed-form last-trick bottom): a real if modest win** —
  0.88–0.99 of T0, the one arm that beat the control.
- **The floor (complete, 55,036,800 states enumerated in 72 s):
  E-A8's new number is 32,532 distinct r1 canonical forms at level 1**
  — the form space is 508× finer than the 64-class alphabet (~2–3 MB
  as a table). E-A9 verdict: floor-table lookup 1,430 ns vs closed-
  form control 35 ns — the floor TABLE is a 41× negative, reported as
  one; the closed-form control is what the arms actually use.
- Lemma E's implementation is VALIDATED (the receipts): the one
  canonical-form code path never disagreed with re-expansion to
  terminals across 1,685 samples spanning both layers.

Structural lesson (completing S5h's): cone identity cannot
short-circuit descent (S5h); structural identity CAN — but harvesting
it costs a canonicalization per distinct state, and under a plain memo
that price exceeds the harvest at level 2. Symmetry pays where the
per-hit saving is large (deeper boundaries), where the form is cheap,
or where the object of interest IS the form inventory — and the
32,532/64 split plus the 1.36M-record level-2 curve are the first
direct data for the seat-level census's size question. Both negatives
are attributed, not spun; next moves are Jason's. Full gate green.

## S5k — 2026-08-11 (evening): the seat-level census, answered by proof

Design `walt/SEAT-CENSUS.md` (Jason's flat-stack framing: three alphabets —
hand forms, first-trick interface, landing — composed never). Adjudicated by
a fresh walt-math (predecessor retired; the rulings file is the inherited
memory): S-A1..S-A21 plus three theorems, no build needed for the headline.

- **Lemma S** (seat-side transport): a transport of the declared seat-side
  structure carries every count-free censal question across; equal forms ⇒
  equal answers.
- **Corollary S-rigid**: the pip-trump structure on all 28 live tiles has NO
  nontrivial self-transport, and focal-fixed kills rotation — so the
  seat-side structural quotient at the first play is the IDENTITY.
  **COUNT 1 = C(28,7) = 1,184,040 is a THEOREM**, missing the 10^5 bar
  ~11.84×. My proposed invariant list had five gaps (g1–g5) that would have
  produced a spuriously small count; all five caught at adjudication.
- **Lemma S-fold**: the seven pip declarations fold exactly 7:1 under
  π_{p→p'} = (p ↦ p') + the unique order isomorphism on the remaining six
  pips (comparison-reading-dependent: under the literal §1.3 tier-0 reading
  only 0↔6 folds; the bar's answer is insensitive to the choice).
- **Lemma S-det**: interface determination holds; the bounded first-trick
  alphabet IS the raw record space — no compression at the top.

THE INSIGHT (the one that reframed everything): structural compression is
bought with DEADNESS — dead tiles, inert contexts — which is why level 1
gives 55M → 32,532 forms → 64 classes, and nothing is dead at the first
play. The identity quotient is not a failure of technique; it is the
statement that the abstraction level was too fine for the question. Named
OPEN: whether a coarser lawful equivalence (dynamics-style, needing descent
per S5h; or value-partition-style) reaches 10^5. The S-A18 receipt build is
PARKED (theorems stand without it). Jason: "I'm actually thrilled by this
negative... proof that what we have doesn't represent what I'm thinking.
That's an opportunity."

## S6a — 2026-08-12: predictive algebra (v0.6) and the dimension census

Jason brought a new math track: `walt/math/predictive_algebra_v0.6.md` —
predictive state coordinates over ℚ: continuation tests, exact predictive
rank (rank_Q of the continuation matrix; "predictive dimension" hereafter,
R-A3), residual closure, forward moments ψ(B) × backward policy vectors
c_ρ, J_ρ(B) = ψ(B)c_ρ. The escape S-rigid left open: linear rank ≤
partition-lump size; every behavioral row can be distinct while the rank
stays small. Filed as the v0.6 track (exploratory); design
`walt/PREDICTIVE-RANK.md` = v0.6 Experiments 0+1 only, adjudicated by a
fresh walt-math BEFORE build (R-A1..R-A24).

**Adjudication headline — Lemma R(c), a theorem with teeth:** in Straight
42 every tile is eventually played and publicly attributed, so a complete
continuation record determines the latent world; hence ANY closure whose
terminal seed contains a nonzero constant has predictive dimension exactly
|X| — v0.6 §6.2 verbatim and the distribution contracts are THEOREM rows,
not measurements. The sole measurable object: dim V^val for the count-free
expected-trick contract (zero terminal seed). Also delivered: the v0.6
proof audit (all SOUND; implicit hypotheses H1–H3 now builder obligations;
gaps G1–G3); Corollary R-fold (dimension is declaration-fold invariant;
bases/matrices are freeze-relative and never fold-compared); the concrete
authority re-aimed at treatment H (my design had equated the m3 solver
with the P-A6 world-informed aggregate — strategy fusion, P-Q2's trap in
new dress, caught at adjudication); root-only focal-lead fence with
primitive-step closure (interior interfaces of every leadership visited,
U(i) never materialized); freezes 22–26 (the 18–21 collision REJECTED —
spent numbers are never reassigned).

**The build** (`walt-factory/examples/predictive_rank.rs`, ~1,300 lines):
exact sparse row reduction over arbitrary-precision rationals (BigRational
— first use in walt; i128 ratios only at declared boundaries); the value
closure via per-record generator families {g_u0} ∪ {B_{s,a} − B_{s,u0(s)}}
∪ {w_o·ι_o(f)}; the R-A18 gate (per-lead Q vs ScalarHidden dag-v1 through
the affine bridge Q_diff = 2·Q_count − grade, exact); Lemma R(b)
membership+pairing receipts (extracted H policy value ∈ V^val, ψ·c equals
the concrete expectation); E0 mass and two-path bucket receipts; Lemma
S-det bijection receipts; the fold receipt at all 7 declarations per
coordinate. Two of my errors were caught BY the receipts mid-build: the
fold transport is the ORDER ISOMORPHISM of Lemma S-fold, not a pip
transposition (run 1 crash); and the distinct-matrix census is
freeze-relative, NOT fold-comparable — basis-dependent collisions moved
counts by 1–2 across declinations exactly as R-A7 warned (run 3 crash;
the lawful fold receipt is per-γ record counts, γ being pip-free).

**THE RESULT (results/predictive_rank_2026-08-12.txt, 52 s total, all
receipts green, gates MET, fold 7/7 everywhere):**

- grade 1: dim V^val = 1 at all 12 coordinates (|X| = 6).
- grade 2: dim V^val ∈ {42, 42, 52, 54, 56, 59} (|X| = 90); behavioral
  rows 43–72 — a real but modest linear-over-partition win.
- grade 3: dim V^val ∈ {1461, 1492, 1680} (|X| = 1680) — one coordinate
  at FULL rank |X| exactly; the others at 87–89% of |X|; behavioral rows
  ≈ dims (the partition/rank gap closes).
- **Gate B verdict, per the pre-declared criterion: payoff REFUTED** —
  D(2)/D(1) = 59 vs fiber ratio 15; D(3)/D(2) ≈ 28.5 vs 56/3 ≈ 18.7. The
  dimension grows AT LEAST as fast as the fiber.

**Reading (exploratory, coordinate-relative, the fence applies):** the
value closure saturates by grade 3 — the span of lawful policy values is
essentially all of ℚ^X. The mechanism rhymes with Lemma R(c): even with
the constant excluded, the field-share weights w_o(ξ) (products of
1/|legal|) are world-discriminating, and hundreds of record-wise pullbacks
inject nearly independent directions. Linear predictive compression under
this contract/field/observation model is dead at the depths that matter,
for the same structural reason the partition quotient was: the game's
public-attribution observation structure. NOT killed: root-action ARGMAX
partitions (the dropped-30 evidence — value spans can be full while the
decision function is simple), v0.6's dual policy geometry (Gate E,
unmeasured), and moment compilation for FIXED shallow queries (the
lead-recovery DP never needed a spanning basis). Carried back to the math,
better informed — exactly the deal.

## S6b — 2026-08-12: the policy-geometry probe (v0.6 Gate E)

Jason's direction after the Gate B refutation: his hope is SIMILARITY OF
OUTCOMES ("likely to get 32 one way or the other"; the melted-candlewax
oracle PDFs — spiky, clustered, long dead flats), not exact low dimension;
Gate E is the exact, adjudicable fragment. Design
`walt/POLICY-GEOMETRY.md` (four cardinalities N_pol/N_vec/N_par/N_exp,
never conflated); walt-math rulings PG-A1..PG-A18 delivered
**Proposition G-flat** (grades 1–2 carry NO policy geometry — forced
continuations; grade 3's only free layer is trick 2, N_pol(a) = 2^k(a);
the probe has exactly ONE measurement and my growth-ratio criterion was
structurally unavailable — replaced by absolute bands) and **Lemma G**
(backward Pareto pruning is exact through the positive composition, the
incremental fold is mandatory, Exp must be defined as the UNIQUE
maximiser — my "some belief" definition was not pruning-safe, the silent
shrink caught at adjudication; N_vec is destroyed by any pruning; convex
pruning is lawful for N_exp only). Exposure method frozen: Lark's LP,
exact-rational primal simplex with Bland's rule, witnesses both ways.

**THE RESULT (results/policy_geometry_2026-08-12.txt, 74 s, gate green):**
of the 9 measured grade-3 (coordinate, lead) pairs:

- **7 of 9: the Pareto frontier is a SINGLETON** — one policy weakly
  dominates every lawful alternative in every one of the 1,680 worlds;
  N_par = N_exp = 1 against plan counts up to 2^19930. Receipts: the
  dominance spot receipt (1,024 explicit policy variants all pointwise
  under the singleton), the authority receipt (frontier max = treatment H
  exactly, every row), G-flat receipts at grades 1–2 (30 rows, all 1).
- **2 of 9 (one coordinate, leads 1-0 and 1-1 under 0-trump — the
  non-boss trump leads): STOPPED** — the running frontier exceeded the
  declared cap (4,096, then 16,384 on a declared raise) at the SAME
  trick-1 partial sum both times. No N_par is reported for them (a
  partial frontier bounds nothing, PG-A13).
- **Formal verdict, per the pre-declared discipline: STOPPED, NO
  VERDICT** (a capped coordinate forbids the global claim). The texture
  is the finding: BIMODAL — total strategy-side collapse almost
  everywhere; genuine frontier explosion exactly where the 42 is
  genuinely tense (leading a low trump rather than the boss).

Read with S6a: the value SPAN is full (Gate B refuted) while the decision
side collapses to one dominant policy at most measured roots — value
richness and decision simplicity coexisting, which is the dropped-30
lesson made exact, and the first exact evidence FOR the
outcome-similarity direction at the strategy level. Where the frontier
explodes, the incomparability structure ("better except when X") is
exactly what the spike-anatomy frame wants to name. No similarity or
tolerance claim is made (PG-A17); δ-similarity remains future
mathematics.
