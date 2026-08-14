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

## The S6 era — S6a to S6d

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
  not quotable). One detector never fired; the trumpless-junk tie mechanism stays
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
