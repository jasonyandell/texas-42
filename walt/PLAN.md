# walt — the imperfect-information seat

**Status: exploratory build. Nothing here is promoted.** The maintained wiki
entry point is [`wiki/walt.md`](../wiki/walt.md); the GPU-native branch is owned
by [`wiki/walt-gpu-native-trick1.md`](../wiki/walt-gpu-native-trick1.md). The
original mathematical basis is frozen at
`walt/math/unified_information_geometry_v0.4.md` (exploratory tier; its §17
claim ledger governs what is "proved prose" vs "reported receipt" vs "open").
For GPU-native trick 1, the received v0.2 guide is preserved byte-for-byte and
`GPU-NATIVE-TRICK1.md` v0.3 plus `CENSUS-RULINGS.md` GT1-A1..GT1-A9 are the
binding first-build authority; freeze 55 fixes the portable M0/M1 boundary.
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
- **NO-RESCUE (Jason, 2026-08-10).** When something doesn't work, that is
  not a thing to fix, spin, or assist with engineering — it is a reason to
  go back to the math, better informed. The mechanism is the
  counterexample: it's math, so a failure is a concrete refutation to
  carry back to the basis, and that is a *good* outcome ("if the whole
  thing falls on its face that's FINE"). Corollary on verification: verify
  against the reference we have — thousands of lines of proofs — but do
  not verify in triplicate; when independent mechanical verification is
  genuinely needed, the path is **Lean, not Python** (the `lean/` path is
  paved: 42 priority-0 rows kernel-proved). Standing epistemic frame: the
  mathematics has proven the object **exists**; it has **not** proven its
  utility. If the utility turns out bad, that is a conversation with
  Jason, not a rescue.

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

The GPU-native path is a narrow side branch of that workspace, not a rewrite of
the original import graph:

```text
walt-core      declaration/rules semantic source
    ↓
walt-gpu-spec  M0: U256 ABI, role/frame types, generated semantic tables,
               canonical hashing
    ↓
walt-gpu-ref   M1: narrow OpeningRootV1, scalar projector, independent direct
               parity carrier, bound envelope/stop receipts
```

There is no `walt-metal` crate while Gate 0 is closed, and no controller crate
before portable M0/M1 passes its integrated reproducibility gate.

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
- **S5d** (2026-08-10, evening): the re-tethering — fresh full read of
  v0.4; drift diagnosis (raw-interface lumpability; the PI carrier was
  already ruled out by §12.4/§17.5); Jason's theorem **§12.6A
  equivariant controlled lumpability** checked and filed as the v0.5
  track (`walt/math/equivariant_lumpability_v0.5.md`); direction reset
  to the lossless count-free equivariant quotient; m4/S6 deferred;
  `walt/math/implementers_guide.md` commissioned.
- **S5e** (2026-08-10, late evening): the situation census. r1 finest
  structural quotient: ECL PASS (the v0.5 existence OPEN resolves YES),
  670 cross-kernel classes, zero root merges. r2 coarsenings: lawful,
  still 647/647 roots. **r3 retrograde coarsest (Jason's pathfinding
  direction): 15,253 → 1,459 classes; roots 647 → 306; t7-lead alphabet
  63; class DAG 306/406/360/213/63/63/32/16/1; r1-refines-r3 HOLDS;
  independent ECL re-check PASS.** Full gate green. Results in
  `walt-factory/results/census_2026-08-10{,_r2,_r3}.txt`.
- **S5f** (2026-08-10, night): the t5 climb — 2.65M situations; roots
  16,112 → 12,924 (1.25:1); the bar clarified to its true object (the
  SEAT-level census); saturation + carrier-relativity measured.
- **S5g** (2026-08-11, small hours): the railyard — P1 discharged
  byte-identically (one grade-free routine); pruning platform (median
  live sub-DAG 179 @ t6 / 16,782 @ t5); parts catalog (31.6× vs
  368.6×, ω = 1.000); complete level-one alphabet = 64 exactly.
- **S5h** (2026-08-11, morning): the fiber-crush probe — three-arm
  ladder n=4/5/6 under the fiber-probe rulings (P-A1..P-A21);
  memoisation dividend 6×→100× and compounding; class DAG ~5× slower
  than the identity-cache control at first build (structural: cone
  identity cannot short-circuit descent) — class store = transport
  object, not build accelerator; cold H COMPLETED on full n=4 fibers
  in 7–17 s. Full gate green.
- **S5i** (2026-08-11): the fiber-refinement probe — declared
  exclusion remnants under rulings X-A1..X-A19; **Lemma X** proved
  (value-0 excision preserves objective + argmax, one-sided);
  predicate passes 0.1–3.7 ms = 100–1000× cheaper than the best
  storeless route (multi-pass economics measured real); the declared
  X's bite classes (up to 16–36%) but ~zero WORLDS at n=4/5 on
  declarer-focal coordinates — bite plausibly lives in count-bearing
  X's, non-declarer seats, and real voided support. Full gate green.
- **S5j** (2026-08-11): the endgame store — symmetry-reduced tablebase
  under rulings E-A1..E-A21; **Lemma E** proved (structural
  isomorphism ⇒ count-free value equality — lookup with no descent);
  persistence discipline first implemented (gitignored cache,
  freezes 14–17). Negatives, attributed: tablebase arms 1.6–2.7×
  slower than the plain cache (canonicalization ~4.6 µs dominates;
  A1's memo already collapsed what a hit saves); floor table 41×
  slower than closed-form trick resolution. Positives: convergence
  real (38–73% form-hit rates — relabeling repeats a state key can't
  see); closed-form bottom wins 2–5%; **level-1 form count = 32,532**
  (vs 64 classes) and level-2 store at 1.36M records un-saturated —
  first direct size data for the seat-level census. Full gate green.

- **S5k** (2026-08-11, evening): the seat-level census, answered BY
  PROOF at the finest seat-side equivalence — rulings S-A1..S-A21 from
  a fresh walt-math; **Lemma S** (seat-side transport), **Corollary
  S-rigid** (no nontrivial self-transport at the first play ⇒ the
  structural quotient is the IDENTITY: COUNT 1 = C(28,7) = 1,184,040,
  a THEOREM, ~11.84× over the 10^5 bar), **Lemma S-fold** (exact 7:1
  declaration fold via the order isomorphism π; comparison-reading
  caveat recorded), **Lemma S-det** (no compression at the first-trick
  interface). The insight: structural compression is bought with
  DEADNESS, and nothing is dead at the first play. Receipt build
  parked; coarser lawful equivalences named OPEN.

- **S6a** (2026-08-12): the v0.6 predictive-algebra track filed
  (Jason's upload; PSR-style exact rank over ℚ) and its first probe
  run under rulings R-A1..R-A24. **Lemma R** (constant-seeded closures
  are degenerate at dimension |X| — the distribution contracts were
  theorems before any code ran; only the expected-trick value closure
  V^val is measurable), **Corollary R-fold**. Measured, 52 s, all
  receipts green: dim V^val = 1 / 42–59 / 1461–**1680 = |X| (full
  rank)** at grades 1/2/3 vs |X| = 6/90/1680. **Gate B: payoff REFUTED
  at the pre-declared thresholds** — the value closure saturates by
  grade 3; linear predictive compression dies of the same
  public-attribution structure that killed the partition quotient.
  Standing: root-argmax partitions, dual policy geometry (Gate E),
  fixed shallow queries (the lead-recovery DP) — all untouched.

- **S6b** (2026-08-12): the policy-geometry probe (Gate E) under rulings
  PG-A1..PG-A18. **Proposition G-flat** (grades 1–2 have no policy
  geometry; grade 3 = the one measurement, N_pol = 2^k(a)) and **Lemma
  G** (backward Pareto pruning exact; Exp = unique maximiser; N_vec
  destroyed by pruning) delivered at adjudication. Measured: **7 of 9
  grade-3 (coordinate, lead) pairs have a SINGLETON Pareto frontier**
  (one policy dominates every alternative in all 1,680 worlds;
  N_par = N_exp = 1 vs plans up to 2^19930; dominance spot receipt and
  treatment-H authority receipt held); 2 of 9 — the non-boss trump
  leads of one coordinate — STOPPED at the frontier cap (16,384) in a
  trick-1 partial sum. Formal verdict STOPPED per the pre-declared
  discipline; the finding is the BIMODALITY: value span full (S6a) yet
  decision side collapsing to one dominant policy almost everywhere —
  the first exact evidence for Jason's outcome-similarity direction at
  the strategy level; the explosions sit exactly where 42 is genuinely
  tense.

- **S6c** (2026-08-13): the decision-deadness probe under rulings
  J-A1..J-A18 (Lemma J; Propositions J-0/J-1/J-win; Jason's binding
  count guard as a firing conjunct of every detector); runner made
  parallel + resumable under DS-A29..DS-A36 (freezes 41–43) after two
  killed runs. Measured at census scale (45 units, 174.25M detector
  calls): **soundness perfect — zero false positives**; 51% of 49.5M
  ground-truth-classified call sites are one-deviation ties ("junk
  everywhere" quantified); **detectors certify ~33% of all ties at
  ~25 ns/call** — D1-sym is the workhorse (8.26M hits, up to 97.5% of
  a unit's ties), D0 rare but ~100% on one wholly-dead grade-3 family,
  D1-win never fired. The trumpless-junk family's tie mechanism remains
  UNIDENTIFIED (J-A8: no fourth detector without proof). Mechanism
  receipts green: resume-validation PASS, deterministic block
  byte-identical across invocations, cache survived a mid-run kill at
  41/45. Also this session: decision-sparse v0.1 filed + audited
  (DS-A1..A18, errata E1–E6.5), second audit adjudicated (DS-A19..A28,
  errata §8 — DS-A9's cone clause corrected by Lemma E8/J(c′));
  Experiment A of the decision-sparse program is complete.

- **S6d** (2026-08-13): the separation probe — Experiment E, the first
  exact root-action certifications. Design SEPARATION-PROBE.md;
  rulings SEP-A1..SEP-A19; freezes 36 (candidate library v1) and 37
  (action-conditioned upper witness = the existing
  `revealed_summary().q_c`, DS-A7(iii)'s premise corrected at SEP-A7);
  Corollary E4.1 delivered at adjudication (primal ceiling: with
  H-argmax seeds L = Q^H necessarily, verdicts decided entirely by the
  U side; a failing pair would prove NO candidate set separates it
  under relaxation C). **All three grade-3 coordinates SEPARATED**, all
  five receipt families held: idx=0 root 00 certified against both
  exploded-frontier leads (margins 449/1120, 59/2240) — S6b's
  uncomputable frontier proved unnecessary for the root decision;
  idx=1299709 root 22 at the full 1/63 headroom; idx=2599418 both tied
  optima, Opt^H exhausted. **Price localization:** 7 of 9 per-action
  prices exactly 0; the two nonzero prices sit exactly at the two
  frontier-explosion leads. NOT the economy claim (SEP-A15(ii));
  the economy successor (non-exact seeds + Lemma E7 transport) is
  named at SEP-A17 and needs its own adjudication.

- **S6e** (2026-08-14): the economy-seed probe — SEP-A17's successor,
  adjudicated EC-A1..EC-A14 (freeze 46 closed arm list; freeze 36 v2
  transport for the declaration fold; Corollary S-fold-val delivered).
  **CERTIFIED-CHEAP at both positive-slack coordinates**: at idx=0,
  greatest-tile and trump-hoard certify the lead at economy gap ZERO
  (both independently reproduce the dominant playbook), while
  least-tile/beat-if-able miss by 3/7 — sixteen times the slack; the
  zero-slack control fired as the pre-declared theorem. The PRIMAL half
  of the economy claim is exercised; the full claim needs the U side
  cheapened (EC-A13 — Experiment D, freeze 38 reserved).

- **S6f** (2026-08-14): freeze-44 budgeted evaluators ((R0) blocking
  regression PASSED — grade-3 receipt reproduced with exactly the two
  permitted differences) and the n4 §5 measured rung: **NO-GO, gate
  failure filed as a result** (the measured cost model SEP-A10(i)
  demanded). U side affordable (~4.3B of 40B walk-steps); blocking:
  partition at (h0,00) exceeds P_max = 32M states, and est largest-unit
  wall ~58 min vs the 10-min gate; the declared fallback {h6,h4,h8}
  fails its own arithmetic at h8. Per N4-A12(c): RETURN to the rulings
  file. The n4 machinery (tiers, checkpointing, DS-A36 blocks) is built
  and committed, gated behind that return.

- **S6g–S6n and the seed survey** (2026-08-14/15, historical): the trick-1
  drawing theorem/probe, n = 4 overnight pass, lay-down catalogue, map-free rule
  walk, first and second nonanticipativity rungs, fee audition/correlation line,
  and the 100-seed survey all returned. Their detailed chronological records and
  corrections live in `LOG.md`; their mathematical ownership is the S6-era page,
  `CENSUS-RULINGS.md`, freezes 47–54 and the reference map. They are retained
  inputs and controls, not the active implementation ladder below.

- **GT1 portable M0/M1** (2026-08-16): received v0.2 source frozen and checksum
  gated; v0.3 adjudicated; GT1-A1..GT1-A9 and freeze 55 fixed; `walt-gpu-spec`
  and `walt-gpu-ref` implemented; Lean foundation added. The final source-bound
  canonical receipt regeneration and integrated Rust/Lean gate passed, closing
  **PORTABLE M0/M1 COMPLETE under freeze 55**. Metal Gate 0 is **NO-GO** because
  full Xcode is absent; M2+ has not run.

## Next: GPU-native trick-1 ladder

This is the active plan. The old compression-era agenda below is preserved as a
historical direction ledger; it is not permission to bypass this ladder.

1. **M0 — portable exact semantics: COMPLETE under freeze 55.** Keep
   `walt-gpu-spec` generated from `walt-core`; require checked U256/frame
   arithmetic, independent big-integer oracles, FIPS hash anchors, the complete
   prose-rules bridge, source manifest and no-float/warning gates. The integrated
   freeze-55 pass is green.
2. **M1 — scalar opening projector: COMPLETE under freeze 55.** Require exact
   `m=0..6` counts and mass conservation, every grades-2–4 reduced direct-parity
   coordinate, the grade-5 preflight zero-output stop, same-context reuse without
   physical-action collapse, fully bound canonical envelopes and fresh
   byte-for-byte regeneration. Grade 4 remains a correctness instrument, never
   opening evidence.
3. **M2 — Metal parity: BLOCKED AT GATE 0, NOT RUN.** A compatible full Xcode
   must be installed and selected by the machine owner first. Then inventory the
   actual compiler/library/capture/profiling surface, run one integer-only
   diagnostic, adjudicate and freeze GPU work units/order/arena/no-partial rules,
   choose the minimum Rust Metal binding, and require cell-for-cell and byte-for-
   byte CPU/GPU parity. No tolerance and no skipped-toolchain green.
4. **M3 — explicit grade-4 perfect-recall net: UNBUILT.** The key must be
   extensionally equal to the canonical focal observation state. Compare its
   lawful value with the existing hidden-information authority and require the
   deliberately world-split negative control to be rejected as treatment H.
5. **M4 — representation-growth gate: UNBUILT.** Only after M0–M3, measure basis
   rank, cells, query deduplication, materialization, arena high-water, sort
   volume and CPU/GPU crossover on declared carriers. Slabs, exact early
   reduction and fail-closed caps are mandatory; no low-grade result rescues or
   kills the opening target without the gate's transport argument.
6. **M5+ — stopped controller and opening attempt: UNBUILT.** Join one lawful
   shared-plan lower witness, action-conditioned uppers, the count-aware
   projector and perfect-recall net before gluing. Cover every root action. The
   only playable outcomes are the exact/canonical outcomes that determine freeze
   26's least-index action; unresolved at a declared cap is a result.

Lean proceeds beside, not behind, this ladder. The stable foundation is proved;
open obligations are the semantic `(response,e)` partition and `A/C/W`
conservation, posterior stratification/factorization, exact information-key
equivalence and deterministic-descriptor corollary, canonical least-index
verdict, sparse-DP/MITM refinement, and Rust/Lean plus Metal/Rust correspondence.
No executable parity result is renamed a theorem.

## Earlier direction ledger (historical, retained)

- **The compression program (v0.5 — the goal, reset 2026-08-10; first
  measurement DONE, S5e):** the trick-six census found the existence
  answer (nontrivial (d,Θ) exists, ECL PASS) and the first compression
  numbers — retrograde coarsest: 15,253 situations → 1,459 classes,
  roots 647 → 306, trick-7 target alphabet 63. **The bar (Jason,
  2026-08-10, clarified same night): order 10^5 situations facing the
  LEADER OF THE FIRST HAND — the trick-1 lead root stratum — versus the
  ~399M-world fiber behind that seat; working hypothesis: first plays
  and last plays have proportional class counts. Still open**; the
  trick-6 numbers are the first rung, and the root-class growth curve
  as the census climbs earlier tricks is the bar's test. Companion
  probe (Jason-endorsed): the SATURATION curve — grow the carrier hand
  by hand in declared order and watch the new-class rate fall; a
  converging inventory is the direct signature of "there are not that
  many situations." Next rungs, in order:
  1. ~~Trick 5~~ **DONE (S5f)**: roots 16,112 → 12,924 (1.25:1 —
     weakening with depth as predicted); t7 alphabet stable at 63–64;
     t6-lead inventory ~24k when properly sampled; t5 saturation far
     from converged. World-level trick-1 roots extrapolate
     astronomically — which redirects the bar to its true object:
  2. **The seat-level census** (the bar's actual object, next — design
     to walt-math before build): seat situations = (own hand, history)
     with the belief pushed forward onto world classes (v0.5
     conclusion 1); two seat situations equivalent when their pushed
     class-beliefs correspond under lawful transport. At trick-1 lead
     the raw seat space is C(28,7) = 1,184,040 hands × declaration —
     order 10^5 up to equivalence is the plausible regime, and THIS is
     where the 10^5 bar is tested. The world-level class DAG is the
     substrate/coordinates.
  3. **§12.7 compact descriptions** — make classes sayable (descriptor
     semantics + update law + preservation proof); needed for both the
     seat-level construction and analysis.
  4. **Role re-entry** — declare a nonempty O_D on the census classes
     and bring count/valuation back through the 12.6A gauge corollary;
     the m3 H instrument prices the seat-facing side.
  Infrastructure note: content-addressed class identities make the
  signature cache incremental across runs — a t4-or-earlier climb only
  computes cones for states not already classified (the emerging
  pathfinding infrastructure); carrier stops are declared, never
  silent.
  **S5g standing results (the railyard):** P1 is a rock — one
  grade-free routine reproduces r3 byte-identically at every level
  (Jason's "every trick is the same machine", discharged in code); the
  pruning platform exists (median live sub-DAG per actual t5 situation:
  16,782 classes — exhaustively searchable in milliseconds); the parts
  recur (v2-open suffix library: growth 31.6× vs class growth 368.6×,
  ω = 1.000 on the clean step — classes are menus over a compact
  shared catalog). Open: a second clean level step needs the t4 climb;
  the seat-level census remains the bar's object; §12.7 descriptions
  now naturally target the PARTS catalog (small) rather than raw
  classes.
  **S5h standing results (the fiber-crush probe, 2026-08-11 —
  `walt/FIBER-PROBE.md` + the fiber-probe rulings P-A1..P-A21;
  results `fiber_probe_2026-08-11{,_h}.txt`):** the manyfold for
  fiber evaluation is ORDINARY MEMOISATION and it compounds with
  depth (A1:A0 medians 0.166/0.024/0.010 at n=4/5/6 — up to 100×);
  the class DAG is ~5× SLOWER than the identity-cache control at
  first build at every rung (B:A1 ≈ 4.3–4.9) for a structural
  reason — class identity is a function of the future cone, so it
  cannot short-circuit descent; **the class store is a
  storage/transport object (reuse across coordinates, hands,
  weightings), never a first-build accelerator**. Root collapse nil,
  interior collapse real (1.50M sits → 129k classes). **Cold
  treatment H completed on full 34,650-world void-free fibers at
  every eligible n=4 coordinate in 7–17 s** (m3 dag-v1 solver,
  declared 200M budget) — the seat's actual object is already
  tractable at 4 tricks remaining; the weighted re-solve over the
  fixed class DAG (the platform number, P-A14) remains unmeasured —
  machinery absent. Open next measurements: full-fiber A1/B
  saturation (P-A16), warm-cache across a played trick (P-A17), the
  weighted-H re-solve.
  **Architecture note (Jason, 2026-08-11, small hours) — the seat as a
  fold with adapters:** the chassis shape is
  f(x ∘ f(y ∘ f(z ∘ …))) — an indexed fold (catamorphism) over the
  yard functor with a per-level adapter list, continuation-passing
  style (each level holds and may wrap the rest of the descent);
  "modify f as you go" = prepromorphism. The adapters are the
  seat-facing concerns, injected per level without touching the proven
  machine: prune (support restriction — the forward arrow, injected
  during the descent), believe (pushed-belief weighting, v0.5
  conclusion 1), value (role re-entry at the chosen level, 12.6A
  gauge), refine (declared subclassing where a counterexample demands).
  One frozen lawful f; everything else is declared, per-level,
  inspectable adapters. Native vocabulary in Lean (indexed functors,
  dependent folds); monomorphized shadow in Rust.
  Method unchanged: §12.9 counterexample-guided; every construction
  walt-math-adjudicated before build. Repair loop when the math itself
  needs work: counterexample in hand → exchange dispatch to 5.6 Pro —
  with Jason's explicit go per the exchange rules, as always.
- **Deferred, frozen until the compression question moves** (Jason,
  2026-08-10): S6 corpus-at-scale / all-9-declarations factory work;
  economy lifecycle extensions; per-action parallelism for big-fiber
  tree receipts. **m4 as a Python H checker is retired by the no-rescue
  policy** — if an independent H verification is ever genuinely needed,
  it is a Lean checker, not a Python one; until then the economy's
  triggered deletions stay mechanically blocked, which is safe by
  design. Nothing is discarded: the memoized H solver + tree
  cross-validation are retained as the seat-label ground-truth
  instrument.

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
  designated second implementation for the historical probe line). The
  GPU-native branch instead follows its own v0.3/GT1 authority chain, generated
  semantics bridge, canonical receipts and Lean obligations.
- Metal binding/runtime choice is deferred until Gate 0 opens and the exact
  diagnostic workload is measured. Installing or selecting full Xcode is a
  machine-owner action, never an inferred implementation step.
