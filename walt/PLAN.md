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

## Session log (summaries — full records in `walt/LOG.md`)

- **S1** (2026-08-09): workspace + CI gate; walt-core rules complete
  (13-hand receipt replay, exhaustive counts); walt-kernel fibers
  (enumeration, counting DP, exact uniform sampling). 30 tests.
- **S2** (2026-08-09): walt-geom (i128 rationals, PWL envelopes with
  half-open piece ownership) + PI symbolic parametric backward induction;
  all trick-6 §14.2 pins exact; exp5 pins blocked (→ S3.5).
- **S3** (2026-08-09): information layer — perfect-recall partitions,
  H/C/F operators, information prices; every §14.5–14.6 pin exact
  (7/19, 177/131, 19/105, 4051/45360, ≡0); §14.5 counts reconciled as
  choice states.
- **S3.5** (2026-08-09): exp5 census pins unblocked via the rescued probe
  suite (`walt/probes/exp5/`); scalar PI solver; h1t3 = 10 and
  h3t3 = 5,345 q_points classes exact, plus both horizon-2/3 tables.
- **S4** (2026-08-09): `ControlSkeleton` (closed update by signature) +
  exhaustive §12.1 soundness and §12.6 lumpability checkers; first
  synthesis run: chassis+holder-all lumpable on all 13 kernels
  (5,887 → 2,857), every coarser candidate fails; h0/h11 UNSOUND at ≤4
  under the holder vocabulary (→ S4.5).
- **S4.5** (2026-08-10): exp3A 22-atom control vocabulary ported from the
  rescued probes (`walt/probes/exp3a/`); 90 → 33 → 8 reproduced through
  walt's own checker; the full ≤4 search record reproduced; control atoms
  break every S4 holder ceiling.
- **S5a** (2026-08-10): the regret walker (conflict generator) —
  mid-trick `ReceiptDecision` kernels (364 points, thrice-validated),
  exact fiber-expected regret, world-count dominance triples, live-world
  lost verdicts; full 52-transcript corpus walked (77.5% zero-regret,
  25/52 lost verdicts, earliest t2; regret lives in tricks 1–2); killed
  run diagnosed (cache memory), bounded caches + byte-verified resume.
- **S5b** (2026-08-10): the `Lesson` type (two-sorted implicants, graded
  and labeled verdicts, full widening traces) + witness-terminated greedy
  generalizer; measured basins at t5–6: tiny (median 0–1) — the honest
  falsification pressure that re-scoped S5c-m1.
- **S5b.1** (2026-08-10): walt-math adjudication folded in — dominance
  triples stored, purpose-split basins (refutation vs safe-substitution),
  DomainSpec-gated application (`lesson_applies`), per-carrier
  denominators; falsification deferred to a sharpened S5c-m1.
- **S5c-m1** (2026-08-10): the falsification test proper — relaxable
  bound pairs, cut refinement (Introduce), the t3–6 fiber-capped domain
  (exclusion never sampling), milestone-1 purpose-specific rent.
  **Survived**: pure-atom implicants transfer across hands, rent equals
  origin regret exactly on single-decision basins; 10/16 lessons carry
  selecting atom cells.
- **S5c-m2** (2026-08-10): honesty amendments + the (H,
  fixed-uniform-legal) re-measurement — new scalar H solver (pooled
  maximization, unit-fraction weights, budgeted, capped-never-sampled);
  **10 survive / 0 fail** (5 unmeasured: 4 budget-capped + 1 empty) —
  the inventory is NOT label-fragile; capped fibers point m3 at
  pooled-state H memoization. Registered prediction: team-split beaters
  are the next numerics when totals first fail to separate (§14.7).
- Label unification (2026-08-10, post-m2): `Grade` carries the
  `OperatorPair` product everywhere; walker + lesson fixtures regenerated
  ("worldwise-dominance at (C, minimax-omniscient)").
- **S5c-m3** (2026-08-10, daytime): dag-v1 pooled-state H memoization
  (value-transparency CI-pinned; budget redeclared over the memoized DAG,
  exclusion-determinism); all four big fibers lifted at declared 10^9 —
  **1 survive / 3 FAIL at (H, fixed-uniform-legal)**, every fail
  cross-validated byte-identical by the uncapped tree walk (4/4-decision
  receipt): m2's "not label-fragile" was scale-limited, fragility lives
  on the big early-trick fibers. The lesson DB economy: projected-content
  identity with merge, candidate-complete watched index (exhaustive CI
  cross-check), dual-ledger H-primary rent (unmeasured ≠ zero,
  measured-consecutive deletion, per-row clearance), §16.11 certificates
  (eleven records, checker-coverage annotations), restart-with-retention.
  The re-priced economy: three deletions TRIGGERED (empty-basin + both
  h1 S2 t4 H-fails), each mechanically BLOCKED — no independent H
  checker registered. Eight walt-math forks + two post-hoc reviews
  adjudicated in-session.

## Next

- **m4: the independent Python H checker** — stdlib-only, written from
  `walt-factory/docs/certificate-schema.md` and v0.4 §7.4 semantics
  (never from the Rust), registering against `HCheckerRegistry`;
  per-row clearance appends; the hard precondition for executing any
  H-rent deletion. Then the first live deletion pass, and re-verification
  of the three H-fails by a third implementation.
- **S6+**: walt-factory corpus at scale (all 9 declarations, deals +
  play — finally exercising DT and NT), the dynamic skeleton search
  proper with richer update-law vocabularies (search coarsenings of the
  semantic state that keep kernel agreement), seat chassis wiring (four
  seats, full hands), lessons flowing between all of it. Noted: per-action
  parallelism for big-fiber tree receipts if they become routine.

## Open decisions deliberately deferred

- Support normal form: reimplement per foundation spec vs. thin port of
  rob's (lean: reimplement; greenfield, and the spec is the authority).
- Rational width: start i128-backed `Ratio`; escalate to BigRational where
  denominators demand (fiber-weighted expectations at H≥5 will).
- Seat chassis process model (four independent seat states) lands with
  walt-strat, not before.
- Wiki integration: decided 2026-08-10 (Jason) — `wiki/walt.md` is the
  hub, exploratory-tier labeled throughout; nothing walt is quotable
  above that tier until independently re-verified (Python probes are the
  designated second implementation).
