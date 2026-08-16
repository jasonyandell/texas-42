# walt — the foundation era (S1–S4.5)

[Home](Home.md) · owns: the walt foundation era — building the rules-to-operators
stack and the control-skeleton checkers, S1 through S4.5 · Sources:
[`walt/LOG.md`](../walt/LOG.md) (authoritative per-session records),
[`walt/PLAN.md`](../walt/PLAN.md), [`walt/DISCREPANCIES.md`](../walt/DISCREPANCIES.md),
the frozen basis [`walt/math/unified_information_geometry_v0.4.md`](../walt/math/unified_information_geometry_v0.4.md),
the Rust workspace under `walt/walt-*`. Related: [walt](walt.md) (hub),
[walt-math-reference](walt-math-reference.md) (theorem statements — cited, never
restated here), [walt-factory-era](walt-factory-era.md),
[walt-census-era](walt-census-era.md), [walt-instruments](walt-instruments.md),
[walt-scheme-fix](walt-scheme-fix.md), [walt-decision-sparse](walt-decision-sparse.md).

> **Epistemic tier: EXPLORATORY — below every tier on
> [Home](Home.md#evidentiary-tiers--never-promoted-never-blurred).** The v0.4 basis,
> the Rust workspace, every count and every reproduced rational below. walt's
> cross-implementation pins are **regression pins against probe records, never
> axioms** — no `PASS` is imported as an axiom (TRUST-01), and a green
> `walt/ci/check.sh` is evidence about finite kernels under a declared field, never
> a status change. Nothing here may be quoted in a brief, a dispatch,
> [FINDINGS](FINDINGS.md), or any claim-tier page.

## The arc

Jason's decision of 2026-08-09 set the shape: **freeze** the mathematical basis at
v0.4, build **greenfield Rust** against it, and carry the **dynamic control
skeleton from the jump** — a descriptor is a transducer (state plus a closed update
law), not a labeling. Six sessions in two days bought a rules layer that replays
rob's own play receipt, a fiber layer with exact counting and exact uniform
sampling, an exact one-parameter policy geometry, the four named operators (PI, H,
C, F) with the information prices between them, and the two exhaustive checkers —
static soundness (§12.1) and controlled lumpability (§12.6) — that make "is this
descriptor any good?" a decidable question on finite kernels rather than a matter
of taste.

Two disciplines explain most of the design. *Exact arithmetic only* — no floats
anywhere, rationals over i128 with overflow checks on, which held for the whole era
without escalation to BigRational. *Information honesty by type* — seat-facing code
consumes observation types only and hidden-world types are not constructible from
seat context, so "no dominoes showing that weren't legal to see" is a compile-time
property as far as Rust can carry it. Derived views, never stored state;
reachability stays proof-irrelevant, with no identity-bearing reachability artifact.

## S1 — decisions and skeleton (2026-08-09)

- **Question.** Can a greenfield rules layer reproduce the project's existing play
  ground truth exactly, and the viewer fiber be enumerated, counted, sampled exactly?
- **Method.** Workspace plus the CI gate (`walt/ci/check.sh`: fmt, clippy
  `-D warnings -D float_arithmetic`, a no-float grep, tests); `walt-core` complete
  through v0.4 §1; `walt-kernel` implementing the §2.1 viewer kernel and its
  capacity-cell fiber with enumeration, exact counting DP, exact uniform sampling.
- **Result.** Gate PASS; 30 tests — 13 exhaustive-count, 3 receipt-replay covering
  all 13 hands of `rob/receipts/verify_player.txt`, 9 fiber known-value over 52
  exp5 kernels, 5 sampler. The unique-winner assertion covers all **737,100**
  four-tile tricks × 9 declarations. A one-off differential of the whole rule
  algebra against the reference `rules42.py` — the same 737,100 tricks × 9
  declarations for winner and trick points, all 2,016 follow predicates and 252 led
  contexts — found **zero mismatches**, DT and NT included. No spec-vs-reference
  discrepancies opened.
- **What it taught.** The bridge is real but narrow, and the narrowness is recorded
  rather than smoothed over: `verify_player.txt` is pip-trump only (P0, P1, P3, P4,
  P5, P6 — no P2, no doubles-trump, no no-trump), so it validates the pip-trump path
  and nothing else; DT and NT rest on the exhaustive structural tests.

## S2 — geometry, PI minimax, the trick-6 census (2026-08-09)

- **Question.** Does exact one-parameter policy geometry reproduce the basis's own
  worked trick-6 experiment (§14.2) to the last rational?
- **Method.** `walt-geom`: i128 rationals, affine lines in one valuation parameter,
  continuous piecewise-linear envelopes on [0, ∞) with **half-open piece ownership
  as the type invariant**, argmax correspondences, 29-dimensional capture features,
  finite feature sets carried as generating point sets with support functions (§9.2,
  finite-first per §16.1 — support evaluation never needs the hull). `walt-strat`
  minimal: PI symbolic parametric backward induction (§9.9), the fiber census, and a
  degenerate fixed-field evaluation for kernels with no post-root focal choice.
- **Result.** Gate PASS; 27 new tests (22 walt-geom, 4 walt-strat census, 1
  `#[ignore]`d blocked). Every §14.2 vector reproduced exactly: fiber **90**; Q^H
  lines 2/3 + (1/5)λ and −2/3 − (1/3)λ, crossing at −5/2; 180/180 affine world and
  root curves; 8 parametric classes sized (26, 22, 16, 12, 8, 2, 2, 2); 4 baseline;
  3 action classes (2 at λ = 0); the 8-world boundary tie resolving to 2:1 at 0+.
- **What it taught.** Endpoint ownership had to become a type invariant, not a
  convention (the `_combine` lesson). And the ambiguity protocol earned its keep:
  the exp5 pins inherited from PLAN (h1t3 = 10 "q_points classes", h3t3 = 5345) were
  **blocked**, not guessed — v0.4 §14 defines neither exp5 nor `q_points` and the
  probe source was thought lost, so the test was written `#[ignore]`d rather than
  fitted to a plausible reading.

## S3 — the information layer (2026-08-09)

- **Question.** Do the three treatments of §10.3 — hidden (H), continuation-revealed
  (C), root-revealed (F) — and the information prices between them reproduce the
  §14.5–14.6 record exactly, on a fiber nearly twenty times larger?
- **Method.** `info.rs`: decision nodes as (world, history) particles over the
  fiber, one shared observation-tree walk, the canonical perfect-recall
  `InfoPartition` (§10.1 validity holds by construction; coarser gluings deferred —
  they invalidate backward induction and need their own solver), `Policy` as a map
  from opaque `InfoStateId`s so world-peeking is unconstructible by type (§7.2).
  Operators: **H** (exact symbolic solve on pooled information states), **C**/**F**
  (field held fixed per §10.8, aggregated at the support level — Minkowski sums and
  hulls of unions never materialized), prices (§10.5 nonnegativity and the exact
  decomposition asserted on every result). S2's `field.rs` is superseded by H.
- **Result.** Gate PASS; 12 new tests. Fiber **1680**; masters = {3-2}; Q^H(0-0)
  segments at 1/5 and 4 with the reported coefficients; H root switch **7/19**; the
  nine-segment Q^C(0-0) with prices {1/4, 1/3, 1/2, 2/3, 1, 3/2, 2, 3}; C root
  switch **177/131**; G^cont(0) = **19/105**, G^root(0) = **4051/45360**,
  G^total(0) = **12259/45360**, the same in all twelve live-tile directions;
  G^cont(2-1) ≡ **0** in all twelve; eight control directions affine under H, seven
  of eight multisegment under C and F, only 3-2 affine in all three treatments; V^F
  segment counts 51/51/42/53, inside the reported 42–53. The twelve-direction
  H+C+F sweep runs about 13 s in release; i128 sufficed throughout.
- **What it taught.** Operators are not interchangeable and the code must say so:
  C == H on the trick-6 kernel only because there is no post-root focal choice
  there, while G^root stays a strict resource (§7.6, the strategy-fusion boundary).
  Also the era's first reconciled reading: §14.5's information-state counts
  168 / 7,848 / 504 are the states with a **genuine choice**; walt's full reachable
  totals 60,360 / 69,600 / 164,088 are frozen alongside, consistent with — not
  sourced from — the record.

## S3.5 — the exp5 pins unblocked (2026-08-09)

- **Question.** With the missing definitions recovered, do S2's blocked census pins
  reproduce?
- **Method.** The exp5 probe suite was preserved at `walt/probes/exp5/` (commit
  b3cb523), supplying what the spec lacked: a `q_points` class is an exact PI root
  value vector under the real scoring differential — each trick worth ±(1 + count
  points of its four tiles), focal minus opponents — and the headline counts are
  censuses of a **sampled** world set, 10,000 exactly-uniform draws at seeds
  42042013/42042033, i.e. sampled lower bounds of the fiber census. New
  `walt-strat/src/scalar.rs`: integer PI minimax, trick-boundary cache, alpha-beta.
- **Result.** Gate PASS; 4 tests, about 25 s in release. The samples were
  regenerated with the probe's own sampler and frozen as fixtures; distinct counts
  **9,920 / 9,933** match the records exactly, fingerprinting the streams.
  **h1t3 = 10** and **h3t3 = 5,345** q_points classes reproduce exactly, plus
  act_points 8/31, q_trick 2/1007, act_trick 1/31, the recorded h1t3 class
  representative, true-world class membership for both, both exhaustive
  horizon-2/3 report tables (13 kernels × q/act × trick/points), the trick-6 q_param
  row, and scalar-vs-symbolic agreement on all **647** trick-6 worlds × 2 valuations.
- **What it taught.** A pin is only as good as the definition under it. `act_param`
  was deliberately **not** pinned: the probe canonicalizes a parametric
  optimal-action correspondence by segment identity on the upper envelope, walt's
  `ArgmaxCorrespondence` by argmax value with at-point events, distinguishing an
  isolated boundary tie from none. Pinning one against the other would blur the
  definitions rather than cross-validate either.

## S4 — the control skeleton and its two checkers (2026-08-09)

- **Question.** Does a nontrivial control skeleton exist — a descriptor whose update
  is closed over observations, sound for a response target, and lumpable — on honest
  finite domains?
- **Method.** `walt-skeleton`. The `ControlSkeleton` trait is a typed relational
  `State` plus `init(kernel, world)` (the one latent read) plus `step(d, obs)`
  closed over one observed play: a recompute-from-world update is **unconstructible
  by signature**, so the only legal degenerate form is the constant update, marked
  `UpdateKind::StaticPassenger`. Observations are seat-honest (seat, tile) pairs.
  The §12.1 checker runs an exhaustive fiber-domain factorization census
  |X| → |im D| → |im R*| with §12.9 witness pairs. The §12.6 checker's carrier is
  ALL reachable viewer-decision nodes (world × record) plus an absorbing terminal,
  with `o` the field-play segments between viewer decisions (exactly one trick
  resolves per segment, asserted), `r` that trick's signed `ScalarValuation`
  increment, and the fixed uniform-legal chance law; every kernel row must sum to
  exactly 1, and the carrier is cross-checked worldwise against S3's
  `InfoPartition`. All 13 trick-6 kernels checked exhaustively — no sampling.
- **Result.** Gate PASS; 9 new tests, about 0.1 s added to the gate. Registry =
  team/holder fact per pool tile plus valued-tile beater counts (13 atoms at t6;
  valued = highest-count unseen). h0t6 carrier = 90 roots + 648 future nodes = 738.
  *Static axis* (subsets ≤ 4; targets q_points, action, parametric): minimal sound
  subsets range from size 0 (h3/h6/h12, single-class targets) to 4, but **h0 — the
  §14.4 design kernel — and h11 (q_points, parametric) are UNSOUND at every size
  ≤ 4** under walt's holder-shaped vocabulary. Trick-5 h0 (fiber 1680, registry 19,
  subsets ≤ 3) is UNSOUND for both scalar targets — the §12.3 ceiling reappears one
  horizon deeper. *Dynamic axis* (7 candidates × 13 kernels): the static passenger
  ALWAYS fails condition 1, structurally — its frozen state merges each world's root
  with that world's future nodes, where legal sets differ; chassis alone, +team
  facts, +beaters, and every minimal-sound static winner fail condition 2
  (kernel-mass witnesses) on every kernel where they compress; but
  **chassis+holder-all is lumpable and nontrivial on all 13 kernels** — h0: 738
  nodes → 366 classes; corpus totals **5,887 nodes → 2,857 classes, 3,030 merged**.
  Adding beater counts changes no class (they are a function of the holder map).
  Deterministic run, no seeds.
- **What it taught — the era's first-class negative result.** As the log states it:
  *at this candidate-space size, the only lumpable skeletons found are exactly the
  world-reconstructing ones; every strictly coarser candidate loses predictive
  sufficiency.* §14.7's "it reconstructed the world" recurs on the **dynamic** axis,
  not only the static one. The genuine compression found lives in
  **history-forgetting**, not state-coarsening: remembering who *holds* what while
  forgetting who *played* what and in which order is a real closed-update quotient.
  A result, not a failure — the build was specified so that "no nontrivial lumpable
  skeleton exists on honest domains" would be reportable, and the UNSOUND ceilings
  were recorded as a blocked pin rather than argued around.

## S4.5 — the exp3A control vocabulary (2026-08-10)

- **Question.** Were S4's ceilings a fact about the game or about walt's vocabulary?
- **Method.** The atom semantics thought lost were rescued to `walt/probes/exp3a/`
  (commit 9357536); `lambda_probe_v3.py` Part 1 carries the full 22-atom registry
  including `comp41` and `s3max2`. The semantics were **reimplemented from the
  probe's definitions at the partition level** — probes are validators, never source,
  and nothing was copied; walt's `Decl::rank` is order-isomorphic to the probe's
  ranking and every atom feeds only equality cells and strict comparisons, so the
  induced world-partitions are identical. Ported as `Exp3aAtom`/`Exp3aContext`/
  `Exp3aDescriptor`, marked `StaticPassenger` (§14.4 is a static result); the search
  driver was made generic over both vocabularies.
- **Result.** Gate PASS; the blocked test replaced by three green ones. Context
  derivation lands on the probe's constants (valued 4-1, decisive 2-1, suit 2, boss
  2-2, floor 2-0, 22 atoms). **D = {comp, focal-max, team(2-0), team(4-2)}
  reproduces 90 → 33 → 8** through walt's own §12.1 checker, and stays sound at 33
  cells for the 3-class action target. The full ≤ 4 search reproduces the probe's
  entire Part 1 record: minimal size 4, exactly **eight** solutions — the
  {comp | comp-rank} × {holder | team} family — at 69/53/53/33 cells, for both
  targets. Corpus-wide (walt's own generalization of the vocabulary parameters:
  decisive tile = the viewer tile whose led context touches the most hidden-pool
  tiles, ties to the higher tile — walt-tier, **not** probe-backed off the design
  kernel): the control registry breaks **every** ceiling the S4 holder registry hit
  — h0 sound at size 4 on all three targets, h11 at size 4 on all three (both were
  UNSOUND at ≤ 4) — and minimal sizes drop elsewhere (h1 4 → 3, h5 q_points 4 → 3,
  both via `comp`).
- **What it taught.** The S4 ceilings were vocabulary, not physics. The four §14.4
  atoms demonstrably carry **control-shaped** content that a holder coordinate
  lacks; they became the seed of the lesson vocabulary the factory era builds on
  (see [walt-factory-era](walt-factory-era.md)). Note the naming: walt's `focal-max`
  is its own name for the probe's `s3max2` — the focal/partner slot's best rank in
  the decisive context.

## Ground-truth bridges — regression pins, never axioms

The era's conformance suite is the set of records walt pinned itself against. Each
is a **regression pin**: it detects drift in walt, it confers status on nothing. The
probe records are exploratory-tier themselves; TRUST-01 applies unchanged.

| Bridge | What it pins | Where |
|---|---|---|
| rob receipt replay | all 13 hands of `rob/receipts/verify_player.txt` — actor order, follow legality, winners, points, made/set | S1, `walt-core/tests/receipt_replay.rs` |
| §14.2 trick-6 vectors | fiber 90; the two Q^H lines; 8/4/3 class census with sizes (26,22,16,12,8,2,2,2) | S2, `walt-strat/tests/trick6_census.rs` |
| §14.5–14.6 record | fiber 1680; 7/19; 177/131; 19/105; 4051/45360; G^cont(2-1) ≡ 0; V^F 51/51/42/53 | S3, `walt-strat/tests/exp4_information.rs` |
| exp5 censuses | h1t3 = 10, h3t3 = 5,345 q_points classes (sampled lower bounds), plus the horizon-2/3 tables | S3.5, `walt-strat/tests/exp5_census.rs` |
| exp3A record | 90 → 33 → 8; eight minimal size-4 solutions at 69/53/53/33 cells | S4.5, `walt-skeleton/tests/harness.rs` |

The rob bridge is the only one reaching another *tier's* artifact, and the direction
is one-way: walt conforms to rob's receipt; rob is unaffected by anything walt does.

## What the era left behind

**Instruments** ([walt-instruments](walt-instruments.md); strict import direction
per the §16.2 module graph):

| Crate | What it provides |
|---|---|
| `walt-core` | the rules layer: pips, 28 dominoes, seats/teams, nine declarations, contexts, the rule algebra (incidence, follow, tier, rank, trick key, BEATS/THREAT), count, trick winner, legality, replay. Imports nothing. |
| `walt-kernel` | the §2.1 viewer kernel, observable voids, the capacity-cell fiber: enumeration, exact counting DP, exact uniform sampling (`FiberDp`, `SplitMix64`). |
| `walt-geom` | exact rationals, affine lines, PWL envelopes with endpoint ownership, argmax correspondences, 29-dim capture features, finite feature sets with support functions. |
| `walt-strat` | decision nodes, the perfect-recall `InfoPartition`, policies over opaque info-state ids, and the operator registry: PI (symbolic and scalar), H, C/F, §10.5 prices. |
| `walt-skeleton` | the deliverable: the `ControlSkeleton` trait, the §12.1 soundness checker, the exhaustive §12.6 lumpability checker, both atom vocabularies (chassis/holder and exp3A), and the search driver. |

Both checkers return **typed conflict objects** — `PurityCounterexample` and
`LumpabilityFailure`, carrying the witnessing pair and the exact disagreeing event.
That is the material the CDCL-style lesson factory of S5 walks: the era's real
bequest is that failures arrive as data, not as a verdict.

**Discrepancy ledger** ([`walt/DISCREPANCIES.md`](../walt/DISCREPANCIES.md)) — two
pins blocked and both later unblocked, plus three readings reconciled in place;
`walt/DISCREPANCIES.md` records **no open discrepancy as of S5a**:

- *exp5 census pins* — blocked S2 (v0.4 §14 defines neither exp5 nor `q_points`),
  reconciled S3.5 from the rescued suite; `act_param` left unpinned by design.
- *exp3A descriptor pin* — blocked S4 (§14.4 names `comp41`/`s3max2`, defines
  neither, source thought lost), reconciled S4.5 from the rescued registry.
- *§14.5 information-state counts* — reconciled S3 as genuine-choice states.
- *rank of a mixed tile* — §1.3's "pip sum" and the reference's off-pip ranking are
  the same order inside a tier; walt implements the spec's formulation and asserts
  rank injectivity within every nonzero tier exhaustively.
- *declaration coverage* — the receipt bridge is pip-trump only, in a test that
  fails if the corpus shape changes.

## Where it goes next

The [factory era](walt-factory-era.md) turns these typed conflicts into graded
lessons; the [census era](walt-census-era.md) is where the direction resets, after
the v0.4 lumpability target is diagnosed and Jason's §12.6A equivariant controlled
lumpability opens the v0.5 track. Theorem statements and proof provenance:
[walt-math-reference](walt-math-reference.md). The fence: the [walt hub](walt.md).
