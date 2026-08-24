# walt — session log

The session index. Each entry is a few lines: what the session asked, what it
found, and the wiki page that now owns the full account.

**Pruned 2026-08-13.** This file used to carry the complete per-session build
records. Those records were absorbed into the wiki — see `wiki/walt.md` for the
map — and the full prose remains in git history (`git log -p walt/LOG.md`, or any
commit before the prune). What is kept here is the chronological spine, so that
"what happened when" is answerable without leaving `walt/`.

Same tier discipline as before: everything here is exploratory; pins are
regression pins, never axioms. Entries are appended, never rewritten —
corrections are recorded in place with their provenance, as the S5a count
correction was.

Owning pages: [foundation era](../wiki/walt-foundation-era.md) ·
[factory era](../wiki/walt-factory-era.md) ·
[census era](../wiki/walt-census-era.md) ·
[S6 era](../wiki/walt-s6-era.md) ·
[negative results](../wiki/walt-negative-results.md) ·
[instruments](../wiki/walt-instruments.md) ·
[program and resets](../wiki/walt-program.md) ·
[decision-sparse](../wiki/walt-decision-sparse.md) ·
[GPU-native trick-1](../wiki/walt-gpu-native-trick1.md) ·
[Scheme/Fix](../wiki/walt-scheme-fix.md) ·
[math reference](../wiki/walt-math-reference.md).

## The foundation era — S1 to S4.5

Full account: [`wiki/walt-foundation-era.md`](../wiki/walt-foundation-era.md).

- **S1 (2026-08-09)**: decisions + skeleton. Workspace, CI gate, walt-core
  complete with replay and exhaustive-count tests, walt-kernel fiber
  enumeration/counting/sampling. 30 tests; unique-winner assertion over all
  737,100 four-tile tricks × 9 declarations; no discrepancies opened.
- **S2 (2026-08-09)**: walt-geom + PI minimax + the trick-6 census. Every §14.2
  vector reproduced exactly (fiber 90; the two Q^H lines; 8/4/3 classes at sizes
  26,22,16,12,8,2,2,2). The exp5 pins were **blocked**, not guessed — v0.4 §14
  defines neither exp5 nor `q_points`.
- **S3 (2026-08-09)**: the information layer — perfect-recall partitions, the H/C/F
  operators, information prices. The whole §14.5–14.6 record reproduced exactly
  (fiber 1680; 7/19; 177/131; 19/105; 4051/45360; G^cont(2-1) ≡ 0). §14.5's
  information-state counts reconciled as genuine-choice states.
- **S3.5 (2026-08-09)**: exp5 pins unblocked by the rescued probe suite, which
  supplied the missing definition. h1t3 = 10 and h3t3 = 5,345 q_points classes
  reproduce exactly, plus both horizon-2/3 tables. `act_param` deliberately left
  unpinned — the two implementations canonicalize different statistics.
- **S4 (2026-08-09)**: `ControlSkeleton` (closed update by signature) plus the
  exhaustive §12.1 soundness and §12.6 lumpability checkers; first synthesis run.
  **The era's negative result**: at this candidate scale the only lumpable
  skeletons are world-reconstructing; the genuine compression found is
  history-forgetting (5,887 nodes → 2,857 classes), not state-coarsening. h0 and
  h11 UNSOUND at every size ≤ 4 under the holder vocabulary.
- **S4.5 (2026-08-10)**: the exp3A 22-atom control vocabulary rescued and
  reimplemented from definitions. 90 → 33 → 8 reproduced through walt's own
  checker; the probe's whole ≤ 4 search record reproduced (eight minimal size-4
  solutions). The control atoms break every ceiling S4 hit — the ceilings were
  vocabulary, not physics.

## The lesson factory — S5a to S5d

Full account: [`wiki/walt-factory-era.md`](../wiki/walt-factory-era.md).

- **S5a (2026-08-10)**: the regret walker, the factory's conflict generator —
  mid-trick decision kernels (364 points, thrice-validated), exact fiber-expected
  regret, world-count dominance triples, live-world lost verdicts. **S5a part 2**:
  the first full-corpus run died on memory, was diagnosed (unbounded solver
  caches), fixed with bounded caches plus a byte-verified resume, and completed —
  52 transcripts, 282/364 decisions zero-regret (77.5%), 82 conflicts, 25/52
  transcripts worldwise-lost from some decision on, regret concentrated in tricks
  1–2. Correction recorded in place: an earlier line said 12 worldwise-dominated
  chosen actions at the CI config; the committed pins sum to 11.
- **S5b (2026-08-10)**: the `Lesson` type — two-sorted implicants, graded and
  labeled verdicts, full widening traces — plus the witness-terminated greedy
  generalizer. Measured basins at tricks 5–6 are **tiny**, median 0–1 decisions:
  reported as falsification pressure rather than smoothed over.
- **S5b.1 (2026-08-10)**: walt-math adjudication folded in — dominance triples
  stored as the primitive, purpose-split basins, DomainSpec-gated application,
  per-carrier denominators. Falsification deferred and the falsifier sharpened.
- **S5c-m1 (2026-08-10)**: the falsification test proper — relaxable bound pairs,
  cut refinement, the trick-3–6 fiber-capped domain (exclusion, never sampling),
  purpose-specific rent. **The direction survived**: implicants generalize into
  atom cells that transfer across hands and pay rent equal to origin regret
  exactly. Basin scale remains the standing pressure.
- **S5c-m2 (2026-08-10)**: honesty amendments plus the first re-measurement at the
  seat-facing label, via a new budgeted scalar H solver. Result 10 survive / 0
  fail / 5 unmeasured — read at the time as "not label-fragile". Four of the five
  unmeasured were budget-capped big fibers.
- **S5c-m3 (2026-08-10)**: the dag-v1 pooled-state memoized H solver
  (value-transparency CI-pinned), which lifted those caps. **The correction**:
  3 of the 4 big-fiber lessons FAIL at the seat's label — m2's conclusion was
  scale-limited, and an uncapped unmemoized tree walk reproduced every value
  byte-identically, so the fragility is real and not a memoisation artifact. Also
  the lesson-DB economy: projected-content identity, candidate-complete watched
  index, dual-ledger H-primary rent, §16.11 record emission. Re-priced against the
  lifted budget as the milestone's exit artifact (`economy_2026-08-10_r2.txt`), at
  which point three deletions triggered — each mechanically **blocked** for want of
  an independent checker, and still blocked.
  *(Logged out of order historically: m3 was written above m2. Dependency order is
  m1, m2, m3.)*
- **S5d (2026-08-10, evening)**: the re-tethering. Direction session with Jason,
  no build. Fresh full read of v0.4; drift diagnosed (the lumpability
  instantiation compared interface alphabets raw); Jason authored **§12.6A
  equivariant controlled lumpability**, opening the v0.5 track; direction reset to
  the lossless count-free equivariant quotient; the **no-rescue policy** adopted;
  factory and economy infrastructure frozen.

## The compression era — S5e to S5k

Full account: [`wiki/walt-census-era.md`](../wiki/walt-census-era.md).

- **S5e (2026-08-10)**: the situation census. r1, the finest structural quotient,
  passes ECL exhaustively — the v0.5 existence question resolves YES — and merges
  **zero of 647 roots**; declared coarsenings stay at 647/647. Only the
  **retrograde** quotient compresses: 15,253 situations → 1,459 classes, roots
  647 → 306, trick-7 alphabet 63. Both receipts held.
- **S5f (2026-08-10/11)**: the trick-five climb — 2,651,280 situations, roots
  16,112 → 12,924 (1.25:1 against trick six's 2.1:1). Compression *weakens* going
  earlier and the inventory is unconverged, so world-level trick-1 roots
  extrapolate past any bar. This redirected the bar to its true object, the
  **seat-level** census.
- **S5g (2026-08-11)**: the railyard. P1 discharged in code — one grade-free
  routine reproduces the retrograde partition byte-for-byte at every level. The
  pruning platform exists (median live sub-DAG 179 classes at trick 6, 16,782 at
  trick 5). The first shape notion proved vacuous and was carried back, never
  reinterpreted; the recurrence lives in the **parts** (library growth 31.6×
  against class growth 368.6×, overlap 1.000 on the clean step).
- **S5h (2026-08-11)**: the fiber-crush probe, three arms. **Ordinary
  transposition memoisation is the manyfold** and compounds with depth (6× at four
  tricks remaining to 100× at six); the class DAG runs ~4.3–4.9× *slower* than
  that control at first build, for a structural reason — class identity is a
  function of the future cone and cannot short-circuit descent. The class store is
  a storage and transport object, never an accelerator. Cold treatment H completed
  on full 34,650-world fibers in seconds.
- **S5i (2026-08-11)**: the fiber-refinement probe. **Lemma X** proved
  (value-zero excision preserves objective and argmax exactly, one-sided);
  multi-pass economics measured real (predicate passes orders of magnitude cheaper
  than the best storeless route). But the declared exclusions bite classes and
  essentially zero *worlds* at these coordinates: the engine is proven, the bite
  lives elsewhere. Remnants typed as **analyst conditioning** — a third thing
  beside support and belief.
- **S5j (2026-08-11)**: the endgame store. **Lemma E** proved (equal canonical
  forms are game isomorphisms, so count-free value lookup needs no descent) and
  its implementation validated by 1,685 re-expanded hit receipts. The tablebase
  arms nonetheless run 1.57–2.69× slower than the plain cache — canonicalization
  dominates — and the floor table loses 41× to closed-form trick resolution. The
  convergence being chased is real (830,399 form-hits); the finding is about where
  to spend it. Level-1 form count 32,532 against a 64-class alphabet.
- **S5k (2026-08-11)**: the seat-level census, **answered by proof**. Lemma S,
  Corollary S-rigid (no nontrivial self-transport at the first play, so the
  seat-side structural quotient is the IDENTITY — COUNT 1 = C(28,7) = 1,184,040,
  ~11.84× over the bar), Lemma S-fold (exact 7:1 declaration fold), Lemma S-det
  (no compression at the first-trick interface). Five gaps in the proposed
  invariant list were caught at adjudication. **The insight**: structural
  compression is bought with deadness, and nothing is dead at the first play.

## The S6 era — S6a to S6n

Full account: [`wiki/walt-s6-era.md`](../wiki/walt-s6-era.md); the architecture
and its state: [`wiki/walt-decision-sparse.md`](../wiki/walt-decision-sparse.md).

- **S6a (2026-08-12)**: predictive algebra (v0.6) and the dimension census.
  **Lemma R(c)** with teeth — complete continuation records determine worlds, so
  any constant-seeded closure is degenerate at dimension |X|, making the
  distribution contracts theorems before any code ran and leaving one measurable
  object. Measured: dim V^val = 1 / 42–59 / 1461–1680 at grades 1/2/3 against
  |X| = 6/90/1680, one coordinate at full rank. **Gate B refuted** at the
  pre-declared thresholds: linear predictive compression dies of the same
  public-attribution structure that killed the partition quotient. Root-argmax
  partitions, Gate E, and fixed shallow queries stand untouched.
- **S6b (2026-08-12)**: the policy-geometry probe (Gate E). Proposition G-flat and
  Lemma G at adjudication, then measured: **7 of 9 grade-3 pairs collapse to a
  singleton Pareto frontier** (one policy weakly dominating every lawful
  alternative in all 1,680 worlds, against plan counts to 2^19930), while 2 of 9 —
  the non-boss trump leads — blew past the declared cap. **Formal verdict:
  STOPPED, no verdict.** The bimodality is the finding, and read with S6a it is
  value richness and decision simplicity coexisting.
- **S6c (2026-08-13)**: the decision-deadness probe. Three proved one-sided
  detectors under Jason's binding count guard, run at census scale: 174,250,255
  detector calls, **zero false positives**; 51% of 49.5M classified call sites are
  one-deviation ties; the detectors certify ~33% of them at ~25 ns/call (contended,
  not quotable; the freeze-43 sequential rung, run 2026-08-14, is the quotable
  instrument: 17 ns/call at the g3 unit, 42 ns/call over 3,540,143 calls at the
  n4 unit, detector-arm overhead under 1% of the solve — deadness_rung results). One detector never fired; the trumpless-junk tie mechanism stays
  a named open question. Experiment A of the decision-sparse program complete.
  Also this session: decision-sparse v0.1 filed, audited twice, its repaired
  mathematics maintained in an errata.
- **S6d (2026-08-13)**: the separation probe — Experiment E. **All three grade-3
  coordinates SEPARATED**, the branch's first exact root-action certifications,
  with all five receipt families held. At one coordinate the root is certified
  against exactly the two leads whose Pareto frontiers S6b could not complete, so
  the frontier is proved unnecessary for the root decision (fenced: this is NOT
  the parent's economy claim). Seven of nine per-action information prices are
  exactly zero, and the two nonzero prices sit precisely at the two
  frontier-explosion leads. Next per SEP-A17: the economy-claim successor.
- **S6e (2026-08-14)**: the economy-seed probe — SEP-A17's successor, adjudicated
  EC-A1..EC-A14 (freeze 46; freeze 36 v2 opens transport for the declaration fold
  under the new Corollary S-fold-val). **CERTIFIED-CHEAP at both positive-slack
  coordinates**: at idx=0, greatest-tile and trump-hoard — two four-word rules —
  independently reproduce the dominant playbook's cash-the-boss, hoard-the-trump
  line and certify the lead at economy gap zero, while least-tile and beat-if-able
  fail by 3/7, sixteen times the slack; at the tight rung every cheap arm
  certifies (the indifference-collapsed coordinate); the zero-slack control fired
  as the pre-declared theorem (SEED EXACTLY OPTIMAL, NOT ECONOMY). Nine fold-image
  coordinates all receipt-clean. **The PRIMAL half of the economy claim is
  exercised; the full claim still needs the U side cheapened** (EC-A13, Experiment
  D territory). Details: [walt-s6-era](../wiki/walt-s6-era.md).
- **S6f (2026-08-14)**: the freeze-44 budget refactor and the n4 §5 measured rung
  — N4-A1..N4-A12. Every walk-based evaluator now carries a deterministic
  walk-step budget with declared stops; **(R0), blocking, PASSED**: the filed
  grade-3 receipt reproduced with exactly the two enumerated permitted
  differences. The rung (W=1, M_max = 40 GiB run-owner declared): **NO-GO — the
  gate failure is filed as a result**, the measured cost model SEP-A10(i)
  demanded. The U side is affordable (est 4.33B walk-steps of the 40B budget);
  the blocking objects are the partition at (h0, 00), which EXCEEDS P_max =
  32,000,000 states (the 24.8M estimate was under), and per-unit wall (~58 min
  est at h9 vs the 10-minute gate). The declared fallback {h6, h4, h8} fails its
  own gate arithmetic at h8 (~648s vs 600s), so per N4-A12(c) this returns to the
  rulings file. Details: [walt-s6-era](../wiki/walt-s6-era.md).
- **S6g (2026-08-14, overnight)**: the trick-1 draw probe — the direct run at
  the first-trick dream, math first. The bounded-sandwich proposal was REFUTED
  BY PROOF before any compute (Propositions T1-blind and T1-corner: a hand-only
  lower bound excludes nothing, and the corner sandwich closes only on the
  all-trump hand); Theorem T1-draw and Corollary T1-ruff replaced it, needing
  no relaxation. **287 first-trick plays PROVED**: at every non-trivial
  coordinate of the closed 294-member drawing family, Opt^H is determined
  EXACTLY — every trump lead at +7, belief-free and field-free (a statement
  about the rules, the one place R-A2 does not bind a verdict), every double
  lead strictly excluded with its exact q from an exhaustive integer count over
  all 399,072,960 worlds. Member-not-set DISCHARGED, not waived. The flagship
  {66 65 64 63 62 61 55}: Q^H(5:5) = 7 − 143/5814; Opt^H = the six trump
  leads. The reduced-grade authority ladder held to the rational at grades
  2/3/4. No corpus hand draws; the 13 exact corner gaps (4 .. 92/15) are the
  filed specification for freeze 38's gluing — the next dispatch. Fence: a
  drawing hand is a hand that plays itself; nothing is said about hands that
  require judgement. Risk carried (T1-A12): proved relative to walt's
  implementation of the rules; the corpus check is mandatory before external
  citation. Details: [walt-s6-era](../wiki/walt-s6-era.md).
- **S6h (2026-08-14, overnight)**: the n4 overnight pass — Experiment E at all
  nine real-deal coordinates, four tricks out, under N4-A13..A20 (W=8,
  checkpointed, admission by measured count). **Every coordinate Tier 1** (gate
  MET; R6 held; R1 held; R7 held at all nine — Lemma N confirmed to the step:
  the whole-fiber revealed charge equals the quoted tree-v0 exactly). The
  verdict set is genuinely mixed and therefore maximally informative:
  **4 SEPARATED** (h1 root 11, h4 root 65, h5 root 55, h8 root 55 — the last
  two carrying their real-deal fence markers inline); **4 EXACT NEGATIVES** —
  the branch's first (h0, h2, h6, h12): Corollary E4.1(3) fired for real, so at
  those coordinates NO candidate set whatsoever separates under relaxation C,
  with failing gaps as tight as 8524657/479001600 — Experiment D's inputs,
  finally non-empty; **h9 NOT PRICED with its exact count printed:
  517,562,322 partition states** (2.7× P_max v2), measured by a COMPLETED
  count-only pass per N4-A16(iv), a stop that is a result. Mechanisms all
  green: resume-validation PASS (h0 recomputed whole-call), DS-A36
  deterministic block BYTE-IDENTICAL fresh vs resumed, 36/36 units
  checkpointed and reloaded. Details: [walt-s6-era](../wiki/walt-s6-era.md).
- **S6i (2026-08-14)**: the lay-down catalogue and the four-laydown theorem —
  Jason's family lore, adjudicated and settled. **Theorem LD** (walt-math):
  a hand is a lay down iff (L1) its top trump run is at least as long as the
  outstanding trump set and (L2) every non-trump's threat lies inside
  trumps ∪ hand — two bitset tests, an exact characterization; every lay down
  holds ≥ 4 trumps; T1-draw is a strict inner class (Jason's missing-6:5 hand
  IS a lay down — banking needs a second trump to hide behind, which is
  precisely what (L1) measures). The catalogue: **exactly 301 lay downs per
  declaration** (LD-R1 held against the independent closed form; 42 of 301 are
  T1-draw). The LD plan verified by exhaustive adversarial play at reduced
  analogues (up to 362,880 field-behaviour leaves, zero lost tricks).
  **Phase 2: NO FOUR-LAYDOWN DEAL EXISTS** — exhaustive over the complete
  catalogue from every full-suit anchor; the family's ≤ 3 conjecture is
  PROVED (relative to Theorem LD and the rules-as-implemented caveat;
  corpus check pending before external citation). **Max = 3, witness
  constructed**: blanks {00 10 11 20 30 40 50}, twos {21 22 32 33 42 44 62},
  fives {51 52 53 54 55 65 66}, leftover {31 41 43 60 61 63 64}. 235 ms.
- **S6j (2026-08-14)**: the rule-economy probe at the n4 carrier — the map-free
  rule walk under RW-A1..A8/freeze 49, after walt-math proved h9's verdict from
  the filed numbers alone (NOT SEPARATED by Corollary E4.1(3), the worst margin
  of the nine — a result the pass had left on the table). RW-R2 (blocking):
  the rule walk reproduces the materialised map exactly on shared ground.
  **THE RESULT: every coordinate where separation is possible at all is
  certified by a four-word rule.** P2 greatest-tile separates at all four
  positive-margin coordinates — at h1 with gap ZERO — and at h4 all four rules
  separate. The exact-solve seed was never needed anywhere certifiable, at
  trick 4, on real deals. At h9, Jason's dumb-heuristic bar is set: best rule
  within 1202339/8870400 (~0.136 tricks) of the exact optimum, measured where
  the exact route cannot price at all (rule walks reach 37M–105M states in
  O(1) memory vs the 517M-state map that broke the cap). Negative-margin rows
  typed as E4.1(3) receipts; gaps as measurements; nothing conflated.
- **S6k (2026-08-14)**: the fusion-tax probe — inbox 016 (Pro's
  nonanticipativity-taxes note) adjudicated by walt-math-10 (FT-A1..A29, six
  new lemmas/propositions incl. Lemma FT-arrive, Lemma FT-trunc, Prop FT-flat,
  Lemma FT-post, Lemma FT-mix), freeze 38 v1 FILLED (the gluing cut, the slot
  E4.1(3) reserved), freeze 50 the five-coordinate carrier (h9 in scope via
  the depth-1 frontier — never the 517M-state map). Experiment 15.1 built and
  run: exact first-layer tax Δ¹ at twelve binding pairs; Corollary FT-grade4
  makes the ladder exactly two rungs, so (Δ¹, Δ²) is the COMPLETE fusion-gap
  decomposition. **THE RESULT: the first gluing-cut closure** — h6 pip 4
  [11 40 43 53], competitor 11 EXCLUDED, strict, surplus 4930081/479001600;
  composed with S6h's frozen rows under Lemma FT-mix and receipt FT-R8:
  **Opt^H(h6) = {40}, uniqueness** — the two-sided architecture (one lawful
  plan + one information tax per competitor) closing end-to-end for the first
  time, via the only lever E4.1(3) allowed (FT-A25(vi): a machinery result,
  not a game discovery — the filed Q^H column already knew). All ten TIED
  pairs NOT CLOSED with shortfall = Δ² exactly: those gaps are genuinely
  second-order. The tax is SPARSE (12,639 of 281,542 frontier states, 4.49%)
  and every fusion core is BINARY. FT-R1 reconstruction held at h9 — the
  first independent confirmation its filed U has ever received. Receipts
  FT-R1..R8 all held; emission split on the content seam (freeze 50 v1.1(c):
  positive support committed, zero rows regenerable under a pinned SHA-256).
  Next target has a name: Δ².
- **S6l (2026-08-14)**: the second-rung probe — inbox 017 (Pro's second-rung
  gluing note) adjudicated by walt-math-11 (SR-A1..SR-A36; eight new results:
  Lemma SR-coord, Lemma SR-forced, Prop SR-sep, Prop SR-post, Cor SR-conv,
  Prop SR-degen, Prop SR-taut, Prop SR-loc), **freeze 51** the depth-two
  carrier (h2 both units, then h9 both units; h0/h6/h12 out of scope), and
  **freeze 38 clarified to v1.1(d)** — the induced rung-two cut order
  exhibited, no new content, v1 not amended and v2 NOT opened. The centrepiece
  is the **slack–tax interchange law** Δ² = Σ_I min_b [s + d], the first object
  in the branch that prices *policy adjustment* rather than only conflict — and
  the whole slack column was already inside S6k's frontier pass, unprinted.
  **Prop SR-degen, filed before the run, reshaped the build**: at grade 4 rung
  two closes every binding pair unconditionally, so NO grade-4 experiment can
  test closure, and **no closure verdict is reported**. Ten receipts HELD at
  all four units, (SR-R9) blocking and HELD at grade 3 against the engine's own
  H operator; **arm 2 completed, no stop**; and the adjudicator re-derived every
  quantity from the committed rows with **zero deviations at 3,300 states**.
  **THE FINDING: ESCAPE ACTIONS ARE PRESENT** — 36/330 at h2, 498/1320 at h9 —
  the first measured instance of policy adjustment, so **every future rung-two
  lower witness must cover EVERY first action**, not the complete optimal face.
  Prop SR-loc prices the naive alternative exactly: overstatement 4.0459% at h2
  and 11.7881% at h9. The census is **one structural signature reached by many
  field continuations, NOT a rate** — the FT-A26(iii) selection fence and P-A21
  travel with every escape number. h9's Q^H independently reconstructed a second
  time; **NOT PRICED stands verbatim, a cross-check is not a witness**. FT-A28
  **fully discharged** (four frontier digests) — a discharge resting on SR-A33:
  the build's own streaming SHA-256 had a buffered-length defect, caught by a
  published known-answer self-check before any carrier number, and a
  mis-buffering hash is still deterministic, so the digest receipt would
  otherwise have been green and worthless. Two builder self-found defects
  disclosed in full, neither touching a receipt or a number; first pass in the
  chapter with no specification conflict. Four small items owed on the next SR
  emission, nothing owed now. No rung three at grade 4; the next question needs
  a longer ladder, and FT-A21 becomes the binding constraint.
- **S6m (2026-08-14)**: the feature-fee audition — Jason's table-derived control
  feature, priced. Adjudicated FF-A1..FF-A33 (walt-math-11) with **freeze 52**
  and its v1.1/v1.2/v1.3/v1.4 amendments; Prop FF-blind, Lemma FF-min, Prop
  FF-oracle, Prop FF-degen, Prop FF-corr. Two of the four requested elements
  died on contact: one candidate is action-blind and captures **exactly zero by
  theorem** (repurposed as the null control), and the requested per-state
  centring is unsound — the fee must be centred **per action** or it bounds
  nothing. **Prop FF-oracle fixed the reading before any number**: per-state θ
  is a lookup table, so a LOW capture refutes conclusively and a HIGH capture
  establishes nothing and licenses only the shared fit. **My defect (FF-A11)**:
  the no-outstanding-trump fallback was attached to all three features though
  only two use it, voiding six of twelve cells — typed UNMEASURED, NOT ZERO;
  catchable only because Prop FF-degen makes zero breakpoints diagnostic of
  vacuity. **F1 (Jason's) REFUTED** at h0's 574 leading states, oracle-θ
  capture 3,673 ppm over that set, 23,016 breakpoints — and inapplicable
  elsewhere, the scope discovery being worth more: a boss-keyed feature's
  domain shrinks precisely as the hand simplifies. **F2 (the sibling) BIT**:
  76.4628% over h0's 574 leading, 29.2679% over its 758 following, and
  **exactly 0 over h2's 216 swept states per unit, with 3,126 breakpoints at
  each — a genuine refutation, not the first run's zero-breakpoint tautology**. **THE RESULT: over those same 574 leading
  states one shared θ* = −56/45 gives 76.3608% against the per-state oracle's
  76.4628% — ~99.87% survives collapsing 574 free rationals to one**, the first small fee family shown to
  carry a first-layer tax. On the same 574 states the two candidates differ by
  ~208×. Fences bind hard: one coordinate part, a carrier not a sample, no
  grade-4 verdict moved, rung-one only, **nothing quoted for trick 1**. Nothing
  further commissioned. Filed alongside: **SR-A37** withdrew the
  claim-ledger/FINDINGS/open-problems obligation carried by the SR chapter as
  never owed — the correct count is zero.
- **S6n (2026-08-14)**: the fee-correlation chapter — why a fee bites, measured.
  Adjudicated FC-A1..FC-A22 (walt-math-11) with **freeze 53**; Prop FC-drop, Cor
  FC-null, Prop FC-width, Prop FC-tight. Artifact
  `fc_correlation_2026-08-14.txt` at `08f1b61`. The chapter **declined** the
  third coordinate S6m had named — a third observation taken before the
  mechanism is measured enlarges the confound rather than resolving it — and
  commissioned the diagnostic on the carrier already held. **Prop FC-drop**:
  capture is at least **correlation times reach**, a lower bound needing no
  minimisation, the first quantity in the branch that predicts where a fee can
  bite without computing whether it does. **THE ANSWER, unanimous: h2's exact
  zero is TIE-DRIVEN.** At both h2 units, over each unit's 216 swept states, at the
  beatability feature, the slopes strictly straddle zero at every state with
  neither slope zero anywhere;
  genuine orthogonality is **refuted at every state of the carrier**, not merely
  unsupported. **Prop FC-width is the mechanism**: subgradient width = the
  mass-weighted spread of the feature across the clairvoyant tie, so without
  ties zero capture needs an exact identity and with ties it needs only that
  zero fall inside a positive-width interval. Non-singleton argmax at 236,784 of the
  362,880 (state, world) arrivals at each h2 unit's 216 swept states (65.25%),
  against 59,776 of 266,132 at h0's one unit over its 1,332 swept states
  (22.46%). The profile is a property of the coordinate, not of a feature. **THE CONSEQUENCE: the h2 refutation was never about that
  feature.** No fee keyed on the clairvoyant choice can be *expected* to bite where
  the face is widely non-singleton, so the branch now holds a **pre-fee screening statistic**
  — the argmax cardinality profile, measurable before any fee is built — and the
  fee route **is not to be expected to bite, robustly so**, at such coordinates,
  not defectively served by a candidate — the width result makes zero capture
  robust, never positive capture impossible. The programme's first question at a new coordinate is
  no longer "which feature" but **"is the clairvoyant choice pinned down enough
  for any fee to bite"**. Screen quality, in the bound wording: over the 1,252
  straddle-false states of h0's one unit at F2 the bound is **ATTAINED** (never
  "exact") at 258 (20.61%) and the summed bound recovers 14.873%; **it is a
  lower bound everywhere, and which states attain it is not knowable without
  κ_I** — so in use it is exactly as weak as 14.873%. The reach is measured to
  the nearest *candidate* breakpoint, which may fall short of a true kink, so
  the bound carries a second independent conservatism: both make it smaller,
  never larger, and this belongs with any citation of the 14.873%. The bound is
  **one-sided** and that is what may be said of it: **a positive bound PROVES a
  fee bites at that state; a zero or small bound proves NOTHING** — no false
  positives, unbounded false negatives. Screening and estimating stay different
  jobs; no adjective grades the instrument. Also: null control both slopes zero at all 1,764 swept
  states of the three units; (FC-R2) non-null pairing held at 518 of h0's 574
  leading states at F2; F1g **proved positive** at 322 of those 574 **but** the
  refuted binary form is straddle-false at 374 of the same 574 and cashed out at
  0.367% — proved-positive and negligible are compatible; sweep **declined**.
  Third coordinate still uncommissioned, now selected on **measured
  multiplicity** rather than guessed trump survival. Fences: a carrier not a
  sample, grade 4 so no verdict moved, Δ^(1) only, **nothing quoted for trick
  1**.
- **Corrections recorded in place (2026-08-14, librarian verification pass
  against the results files, which govern):** three numbers above are wrong.
  S6g's corner-gap range: the widest of the thirteen corpus corner gaps is
  **19/3** (corpus hand 11), not 92/15. S6h's tightest exact negative is
  **9557/554400 at h2** (= 8257248/479001600), not h6's 8524657/479001600.
  S6j's h9 bar **1202339/8870400 holds at the two H-optimal actions only**;
  stated unqualified it is false — the smallest rule gap anywhere at h9 is
  177253/3326400, at the non-binding action 61. Full divergence records in
  `wiki/walt-s6-era.md`'s discrepancy section.

- **Ledger closure and seed-survey backfill (2026-08-16).** The S6n line above
  names `FC-A1..FC-A22`; the live append-only family closes at **FC-A23**, whose
  exact one-sided phrasing and chapter-close rule govern. The 2026-08-15
  outcome-independent hundred-seed survey is adjudicated at **SS-A1..SS-A18**
  with **freeze 54**: 100 generated coordinates, every legal root action a unit,
  complete-face/tie-multiplicity and count-only measurements. SS-A18 repairs the
  SS-A6(vi) cross-reference — **FF-A26(iv)**, not `FC-A26(iv)` — and closes the
  range without changing a receipt, result, measured object or reading.

- **GPU-native trick-1 portable M0/M1 (2026-08-16).** The received v0.2 guide is
  preserved and checksum-gated; `GPU-NATIVE-TRICK1.md` v0.3 is the maintained
  repaired contract; **GT1-A1..GT1-A9** close the adjudication and **freeze 55**
  fixes the exact portable boundary. `walt-gpu-spec` implements checked U256
  mass/frame arithmetic and generated `SemanticTablesCanonicalV2` with an
  independent prose-rules bridge. `walt-gpu-ref` implements the narrow
  `OpeningRootV1` scalar projector, exact opening counts/mass, reduced grades-2–4
  direct parity, grade-5 zero-output stop, same-context reuse without action
  collapse, and root/action/profile/table/freeze/build-bound persisted envelopes.
  The Lean foundation proves the stable budget/420/width/current-trick-point/
  cell-count/interval layer and leaves the implementation-refinement obligations
  explicit. The final checked source manifest, committed canonical envelope and
  stop, fresh byte comparison, release workspace tests, strict clippy/float gates
  and `lake build Texas42.Trick1Foundation` passed together. Status:
  **PORTABLE M0/M1 COMPLETE under freeze 55**.

  Metal Gate 0 is **NO-GO** on the present Apple M5 Max host: Metal is supported,
  but only Command Line Tools are selected and `xcodebuild`, `metal`, `metallib`,
  `metal-ar` and `xctrace` are unavailable. M2–M5, the explicit perfect-recall
  net, controller, root value and opening action are unrun/unbuilt. No portable
  result is reported as a GPU result.

## The scenario-player era — the seat plays (2026-08-17 →)

Owning page: [walt-seat-play](../wiki/walt-seat-play.md); spec:
`SCENARIO-PLAYER.md`; results: `walt-m3-probe/arena_results_2026-08-17.txt`,
`walt-m3-probe/level2_results_2026-08-17.txt`,
`walt-m3-probe/divergence_results_2026-08-18.txt`.

- 2026-08-17: first lawful hands (scenario → level-1 → playout/viewer); web
  table with trump picking; `walt_bridge` into the mk5 arena over the
  rob_bridge protocol (zero arena changes, ~15k decisions rules-clean).
- 2026-08-17/18: dropped-30 3×384 × 3 seeds vs the E[Q] n=10 champion —
  pooled walt 630/1152, McNemar z=+6.28, every seed CI-positive; loses
  points, wins marks (pmake objective in data). Exploratory arena outcome.
- 2026-08-17: level-2 (field model as parameter) laddered on the frozen
  carrier — agrees with level-1 at every rung; 5-5 opening unique, perfect
  at n=3200.
- 2026-08-18: rayon-parallel level-2 (~5.6×, byte-identical across thread
  counts) exposed and fixed the PiKey banked-aliasing defect; fix applied to
  all play binaries post-pool (new baseline). Spec-after-build written
  (`SCENARIO-PLAYER.md`) with the obligations ledger as the graduation path.
- 2026-08-18 (overnight): divergence miner — 900 self-played hands, 4,156
  level-2-shadowed decisions; large-gap divergence ~2× in partner-bid and
  defense vs self-bid; top case: count fed to partner's winning trump pull.
- 2026-08-18: `walt-wasm` — the browser decision oracle for the plunge
  client. Player logic consolidated into the library (`level1_evaluate`,
  `best_of`, the bridge's audited `replay`; `Deadline` abstraction inert on
  wasm), string API (`play`/`bid`/`declare`) behind a no-unsafe ABI, ~250 KB
  `pkg/walt.wasm` + typed `walt.ts` wrapper. Full-hand native test (all four
  seats walt, walt-core refereeing, determinism byte-checked) and a Node
  smoke proving the wasm binary reproduces the native trace 28/28 plays.
- 2026-08-18: bidcurve calibration corpus launched — three nested-CRN
  passes (n = 12/40/200) over the same 200 frozen hands
  (`probes/bidcurve/run_calibration.sh`); first 40 worlds of the n=200
  pass are exactly the n=40 worlds, so cross-pass deltas are pure
  sample-size effect. Target: calibrate the auction threshold θ against
  the known small-n saturation overbid. Estimates only.
- 2026-08-18: intake — `math/signed_pivotal_geometry_v0.1.md` (verbatim,
  sha256 filed; house-mathematician pass via the side channel). Central
  objects: pivotal mass q, signed tilt τ, exact gap g = qτ, fixed-pair
  difficulty H = 1/(qτ²)−1; the E0 frozen-plan signed-pivotal audit; the
  three locks (measure/response/optimization). Intake companion
  (`..._intake.md`): all boxed identities verified by hand + 2,000
  exact-rational spot instances; clean on D3; θ symbol collision flagged
  (pivotal win share vs auction threshold — resolution proposed, not
  ruled); O12–O19 filed into the SCENARIO-PLAYER ledger; E0 gap list
  (plan extraction, bitset replay, world/tape seed separation).
- 2026-08-18 (night): signed-pivotal intake ADJUDICATED (`CENSUS-RULINGS.md`
  SP-A1..SP-A12, walt-math). Sound throughout with one repair: §2.1's
  "paired is strictly sharper" holds iff Cov(u_a,u_b) > 0 (SP-A5 —
  anticorrelated Case C inverts it). Renames ruled: pivotal win share
  (never bare θ — the auction keeps θ), **pivotal cover** (never
  "envelope" — that word stays with value upper envelopes), **frozen
  policy** (not "plan"). E0 adopted as **the tilt audit** with corrections:
  freeze tuple = the policy (no DAG serialization needed, SP-A8), replay ≈
  re-solve until extraction exists so panels stay at the hundreds scale
  (SP-A9), corpus anchors named (SP-A10 — "n=800 panel" binds nothing).
  O10–O11 permanently retired (SP-A11). Gate E concordance filed (SP-A12).
- 2026-08-18 (night): `TILT-AUDIT.md` — smoke design for E0 under the
  SP-A rulings: phases A–D on mid/late-grade divergence anchors first,
  implicit frozen policies (freeze tuple = the policy), hundreds-scale
  panels, `tiltaudit.rs` driver shape. Phase E blocked on the SP-A6
  world/tape seed split. θ-sweep analysis predeclared (O14) at 23:49,
  before the n=200 reference pass completed.
- 2026-08-19 (small hours): the tilt audit RAN (`tiltaudit.rs`;
  `Solver::modeled_choice` exposes the lib's pi — one authority). Field
  found DETERMINISTIC (no tape; scenario = world; Phase E vacuous until a
  stochastic field exists). Trick-6: pure Case B (q 20–30%, τ = +1000pm
  exact, H ≤ 4). Trick-4: strong gaps recover at 25 worlds; near-ties are
  honest; instability small (≤ 8/100) but hand 0 caught a live
  discovery-selection error (panel prefers 62 over the majority's 65).
- 2026-08-19: race-then-refine applied to the seat: `level1_raced` (CRN
  block racing, exact binomial elimination — opening leads 745ms vs full
  1230ms at 100 worlds vs 40; disagreements are saturation ties only) and
  `level1_race_refined` (survivor ties → the 16× refinement), shipped as
  walt-wasm opt-in `race 1` with a race-mode full-hand conformance test.
  Default path byte-identical (Node smoke still 28/28 vs native trace).
  Next gate: arena bracket race-vs-full. Exploratory play policy.
- 2026-08-19 01:34: bidcurve calibration corpus COMPLETE (3 passes × 200
  hands, zero died cells; logs + predeclared single-look analysis filed in
  `probes/bidcurve/`). θ CALIBRATED: at n=40 vs the n=200 reference,
  θ=1/2 overbid 37/200 (mean walked bid ~41 — the known saturation
  overbid, quantified); **11/16 = first rung with 0 overbids and 0 missed
  bids** → new default in walt-wasm `bid` and webtable (θ stays a request
  parameter). Saturated n=40 cells average 9810bp in the reference; n=12
  is unfixable by θ (11 overbids at 11/16); trump first-max n40-vs-n200
  agreement 159/200 (declaration noisier than bidding). Solo-auction
  protocol caveat travels with all numbers. Estimates, never receipts.
- 2026-08-19 01:50: arena gate (24 mirrored deals, bid 30): race-refined
  vs full is a strength DEAD HEAT (paired makes 1 vs 2, both 11) at
  slower mean decision cost in the tie-saturated bid-30 regime (177ms vs
  116ms) — the racing edge is regime-dependent (openings, high bids);
  opt-in posture confirmed correct. Filed in TILT-AUDIT.md.
- 2026-08-23: walt live in plunge ("How'd I do? Ask walt" review). Two
  review specimens (a 100%-saturation revelation tie; a 40-vs-160-world
  near-tie flip on a count-timing choice) motivated the level-2
  program: `LEVEL2-PROBE.md` filed as SPEC ONLY — field-swap pivotal
  mass (q wakes up when field upgrades level-0 → level-1) as the
  detector, gated on walt unification + adaptive-sampling math.
  Deliberately not started. Next: merge PR #6, unify the walt crates,
  wiki re-synthesis.
