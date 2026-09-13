# walt — what the frozen-basis programs established (2026-08-09 → 2026-08-16), by result

[Home](Home.md) · owns: the results of walt's pre-pivot research programs — the compression programs, the decision-sparse reframe, the four-trick interval work, the two rules-level theorems and the lesson factory — stated as results with the era pages as provenance · Sources: `walt/probes/factory-results/*.txt` (the artifacts that govern every number here; producers archive-only at commit `648f93a`, protocol in `walt/ARCHIVE.md`); `walt/CENSUS-RULINGS.md` (ruling families F1–F7, Q1–Q5, Y1–Y3, P-A, X-A, E-A, S-A, R-A, PG-A, J-A, DS-A, SEP-A, N4-A, EC-A, T1-A, LD-A, RW-A, FT-A, SR-A, FF-A, FC-A, SS-A); `walt/math/unified_information_geometry_v0.4.md`, `equivariant_lumpability_v0.5.md`, `predictive_algebra_v0.6.md`, `decision_sparse_exact_solving_v0.1.md` and its `_errata.md`; `walt/DISCREPANCIES.md`; `walt/LOG.md`. Provenance pages: [walt-foundation-era](walt-foundation-era.md), [walt-factory-era](walt-factory-era.md), [walt-census-era](walt-census-era.md), [walt-s6-era](walt-s6-era.md); architecture: [walt-decision-sparse](walt-decision-sparse.md); mathematics: [walt-math-reference](walt-math-reference.md), [walt-math-structure-transport](walt-math-structure-transport.md), [walt-math-information-geometry](walt-math-information-geometry.md), [walt-math-deadness](walt-math-deadness.md), [walt-math-decision-sparse](walt-math-decision-sparse.md), [walt-math-freezes](walt-math-freezes.md); refutations: [walt-negative-results](walt-negative-results.md); the program's resets: [walt-program](walt-program.md); what came after: [walt-calculated-evidence](walt-calculated-evidence.md), [walt-counted-belief-era](walt-counted-belief-era.md), [walt-focal-horizon-era](walt-focal-horizon-era.md).

> **Epistemic tier: EXPLORATORY, every statement on this page.** Everything here was
> computed by one Rust implementation over walt's own frozen bases and adjudicated by
> walt-math into `walt/CENSUS-RULINGS.md`. Nothing here is a corpus status, a
> proof-assistant kernel result, an exchange-adjudicated CONFIRMED result, or a rob
> conformance receipt, and nothing here is cited by any page above this tier. A number
> on this page is quotable as a result only through the results file that carries it
> (named in the same sentence) or by brief amendment that adds it to a verifier receipt.
> The **results files govern over prose**: where `walt/LOG.md`, an era page, or this page
> disagrees with a file under `walt/probes/factory-results/`, the file wins, and the
> known disagreements are tabled in [Appendix C](#appendix-c--prose-versus-artifact-disagreements).

## How to read this page

Eight days of research (2026-08-09 through 2026-08-16) produced about 3,770 lines of
session record across four era pages and the decision-sparse page. Those pages are kept
verbatim as provenance; this page is the inversion — the same material organized by what
was established, at roughly a quarter of the length, for a reader who was not in the
room. It ends where the program pivoted (2026-08-17, [walt-program](walt-program.md)
reset 7) to building the seat that plays; the lineage from here into the counted-belief,
anytime and focal-horizon programs is summarised on
[walt-decision-sparse](walt-decision-sparse.md#lineage-since-2026-08-24).

Five conventions every number below depends on:

- **Valuation.** Every value here is the count-free **trick differential** of the focal
  team (`trick_diff`: +1 per trick taken, −1 per trick conceded), reported in the
  **count convention** through freeze 26's exact bridge `Q_diff = 2·Q_count − grade`.
  This predates the 2026-08-17 ruling that the seat's objective is **pmake**
  (P(make the bid)); nothing here is a pmake number, and count re-entry voids every
  count-free record wholesale (E-A2) except where a ruling says otherwise.
- **Coordinate, fiber, grade.** A *coordinate* is (declaration, focal hand, hidden
  pool, leader offset). Its *fiber* is the set of hidden deals consistent with the focal
  hand — the **void-free capacity fiber Φ(C₀)**, a declared cost domain whose members are
  **feasible and never reachable** (R-A2, P-A1); |X| = 1,680 at grade 3, 34,650 at grade
  4 (four tricks out), 399,072,960 at trick 1. *Grade* is the number of tiles the focal
  seat holds. A *carrier* is the declared list of coordinates a probe ran over; a carrier
  chosen by outcome is **not a sample** (the selection fence).
- **The two operators.** **Treatment H** is the seat's actual pooled hidden-information
  solve — one policy per information state, the world never revealed — under the
  declared uniform-legal field and uniform fiber weighting; its concrete authority is
  `ScalarHidden::action_values_dag` (dag-v1, freeze 26, live at
  `walt/walt/src/strat/hidden_scalar.rs`). **Treatment C** reveals the world after the
  root action; its per-action value `U_a = E_β[V*_a]` is the action-conditioned upper
  witness (freeze 37, live at `walt/walt/src/strat/revealed.rs`). `Q^H(a) ≤ U_a` always
  (Lemma E3); the difference is the **fusion gap**.
- **Support is not belief.** The uniform weighting over a fiber is a declared
  aggregation argument on a fabricated kernel (P-A12), not any seat's belief, and the
  field is a separate declaration from the belief (R-A9).
- **A verdict travels with its fences.** Member-not-set (a non-strict separation proves
  membership in the optimal set, never uniqueness — Theorem E6.4); no cost, timing or
  tractability claim follows from any count (SEP-A15(iii), R-A23); nothing measured at
  grade ≤ 4 is quoted for trick 1 or the opening (P-A21); and a stop is a stop.

**Session key.** S1–S4.5 = [walt-foundation-era](walt-foundation-era.md) (2026-08-09/10);
S5a–S5d = [walt-factory-era](walt-factory-era.md) (2026-08-10); S5e–S5k =
[walt-census-era](walt-census-era.md) (2026-08-10/11); S6a–S6n =
[walt-s6-era](walt-s6-era.md) (2026-08-12 → 08-14); SS = the seed survey (2026-08-15/16,
owned here — no era page carries it).

---

## 1. What a seat cannot compress

### 1.1 The bar and its three migrating objects

Jason's bar (2026-08-10, [walt-program](walt-program.md) reset 4): *show or disprove
that the number of count-free canonical situations is of order 10^5*, either outcome a
result (ruling F7, the NO-RESCUE policy). The object the bar was tested against moved
three times, and the answers at the three stations are **not comparable** — every count
is carrier-relative.

| Station | Object | Carrier and state set | Answer | Record |
|---|---|---|---|---|
| S5e (2026-08-10) | world-level trick-six lead roots | 15,253 situations reachable from the 13 pip-trump trick-6 receipt kernels; 647 roots | structural quotients merge **none** of the 647 roots (647/647 at r1, r2); the retrograde quotient r3 reaches **306** | `census_2026-08-10.txt`, `_r2.txt`, `_r3.txt` |
| S5f (2026-08-11) | world-level trick-five lead roots | 2,651,280 situations on the 1,680-world trick-five kernels; 16,112 roots | 16,112 → **12,924** root classes (1.25:1 against trick six's 2.1:1); unconverged at 13 hands (+1,300–1,600 classes per fresh fiber); world-level trick-1 roots extrapolate roughly 370× per trick | `census_t5_2026-08-10.txt` |
| S5k (2026-08-11) | the **seat**-level census at the first play (the bar's true object, per Jason's clarification) | the C(28,7) = 1,184,040 first-play hands × 7 pip declarations | answered **by proof, not enumeration**: the structural quotient is the identity, COUNT 1 = **1,184,040**, exceeding 10^5 by 29601/2500 ≈ 11.84× | `walt/CENSUS-RULINGS.md` § "Seat-census rulings" (no results file; build PARKED) |

### 1.2 Three compression negatives

**(a) The only lumpable skeletons were world-reconstructing (S4, 2026-08-09).** On all 13
trick-6 receipt kernels, exhaustively (h0: 738 nodes = 90 roots + 648 future nodes), the
only candidate descriptors whose update was closed over observations and lumpable were
those that reconstructed the world; every strictly coarser candidate lost predictive
sufficiency. The genuine compression found was history-forgetting, not state-coarsening:
`chassis+holder-all` (who *holds* what, forgetting who played what and when) is lumpable
and nontrivial on all 13 kernels — corpus totals **5,887 nodes → 2,857 classes, 3,030
merged** — and adding beater counts changes no class. The soundness ceilings that came
with it (h0 and h11 UNSOUND at every descriptor size ≤ 4) were later shown to be
**vocabulary, not physics**: the rescued exp3A control atoms (S4.5, commit 9357536)
reproduce the probe's 90 → 33 → 8 record through walt's own §12.1 checker and break every
ceiling the holder registry hit. Deterministic runs, no seeds; the checker crate
`walt-skeleton` is archive-only at `648f93a`.

**(b) Structural matching compresses nothing where the decisions are (S5e, S5f).** On the
15,253-situation / 647-root trick-six carrier, the finest structural relabeling quotient
r1 passes equivariant controlled lumpability (ECL, §1.4 below) exhaustively — the v0.5
existence question resolves YES — with 11,949 classes and 670 cross-kernel merges, and
**zero root merges**. The two declared coarsenings (c2: 11,380; c3: 9,125; c2+c3: 8,659,
largest class 200) also pass ECL and also stay at 647/647. Only the retrograde quotient
r3, which names a situation by its future cone, compresses at all, and it compresses
**late**:

| grade | ply | stratum | situations | r3 classes |
|---|---|---|---|---|
| 8 | 0 | trick-6 lead (kernel roots) | 647 | 306 |
| 7 | 1 | trick-6 mid-trick | 1,294 | 406 |
| 6 | 2 | trick-6 mid-trick | 2,056 | 360 |
| 5 | 3 | trick-6 mid-trick | 3,216 | 213 |
| 4 | 4 | trick-7 boundary (play forced) | 2,010 | 63 |
| 3 | 5 | trick-7 mid-trick | 2,010 | 63 |
| 2 | 6 | trick-7 mid-trick | 2,010 | 32 |
| 1 | 7 | trick-7 mid-trick | 2,010 | 16 |
| 0 | — | hand end | — | 1 (terminal, by ruling) |

(`census_2026-08-10_r3.txt`; full carrier 15,253 → 1,459 classes, 446 singletons, largest
class 477. Receipts: r1-refines-r3 HOLDS; an independent ECL re-check over r3 PASSES with
1,013 classes, 13,794 pairs per condition, 0 counterexamples. The 306 is a 647-root
keyhole: measured inside the trick-five run, the trick-six lead stratum is 179,936
situations → 23,592 classes, `census_t5_2026-08-10.txt`.) Classes are dynamics-equivalence
classes named by future cones on a declared carrier, not hidden-decision PI classes, and
carry no compact description (§12.7 remains open).

**(c) The seat's first-play quotient is the identity (S5k, by proof).** Two theorems and a
lemma, adjudicated in `walt/CENSUS-RULINGS.md` § "Seat-census rulings" (S-A1..S-A21) and
indexed on [walt-math-structure-transport](walt-math-structure-transport.md):

- **Corollary S-rigid.** For every pip-trump declaration δ, the group of self-transports
  of the δ-structure on the full 28-tile live set is trivial; fixing the focal seat kills
  rotation; so the seat-side hand form at the first play *is the hand*, and the seat-side
  structural quotient is the identity. *Proof sketch:* a self-transport φ induces a
  context bijection π with π(7) = 7 (the trump context is preserved). (i) For a non-trump
  q:r the led context is max(q,r), so π commutes with max on the six non-trump pips; a
  bijection of a finite chain commuting with max is order-preserving, hence the identity.
  (ii) φ is then the identity on non-trumps, since q:r is the unique member of σ̂_q ∩ σ̂_r
  and a double is the unique double in its suit. (iii) φ permutes the trumps preserving
  the strict total order of context 7 (δ:δ top, then δ:r by r descending), so it is the
  identity there too. Hence φ = id. Routes (i) and (iii) are independent: dropping either
  invariant does not rescue compression, it only makes the form unsound.
- **Lemma S-fold.** The seven pip declarations fold exactly **7:1** under the map sending
  p ↦ p′ with the unique order isomorphism on the remaining six pips; it is the unique
  transport, and transports compose. So COUNT 1 = C(28,7) = **1,184,040** folded
  (8,288,280 unfolded). The fold is comparison-reading-dependent: under the literal §1.3
  tier-0 reading only 0 ↔ 6 folds, giving 7,104,240 — and the bar's answer is
  insensitive to the choice, both being far above 10^5. Both statements were checked at
  adjudication over all 5,040 pip permutations for all 49 ordered declaration pairs under
  both readings (a check of the proofs, not their authority).
- **Lemma S-det.** The landing state is a function of (declaration, hand, ordered
  trick-1 record) — but the bounded first-trick alphabet *is* the raw record space: no
  compression at the top.

Five gaps in the design's proposed invariant list, each of which would have produced a
spuriously small count, were caught at adjudication (S-Q1). The receipt build was
**parked**; freezes 18–21 were spent on it and are never reused (R-A22). Whether a
**coarser lawful equivalence** — dynamics-style (needing descent) or
value-partition-style — reaches 10^5 is **OPEN** and explicitly not addressed (S-A20).

**The sentence the program is built on (S-A21, verbatim):**

> "The last-trick census quotients 55,036,800 situations onto 32,532 forms and 64 classes;
> the first-play census quotients 1,184,040 hands onto 1,184,040 forms. Same machinery,
> same structure; the entire difference is deadness. At level 1 twenty-four tiles are
> dead, most contexts are inert and erased, and the surviving relations admit enormous
> symmetry. At the first play nothing is dead and nothing is inert. Structural
> compression in this project has always been bought with dead tiles and dead contexts,
> and the seat has neither at the first play."

The two ladders side by side, each count naming its carrier:

| carrier | raw | canonical forms (r1) | count-free classes (r3) | record |
|---|---|---|---|---|
| the complete level-one alphabet under pip-trump: every ordered assignment of four distinct dominoes to four seats × 4 leaders × 4 focal seats × 7 declarations = 55,036,800 situations (a declared exhaustive carrier — the one census number with no corpus caveat) | 55,036,800 | **32,532** | **64** | `census_a1_complete_2026-08-11.txt`, `endgame_floor_2026-08-11.txt` (re-derived in 72 s) |
| the first play: C(28,7) hands per declaration | 1,184,040 | **1,184,040** | — (the quotient is the identity) | S-A20 (proof) |

The trick-five corpus realizes all 64 level-one classes; the trick-six corpus misses one.
Doubles-trump and no-trump are out of declared scope (F1): this is the complete
*pip-trump* level-one alphabet, not the complete Straight 42 one.

### 1.3 Transport objects, not accelerators (S5h, S5j, S5i — 2026-08-11)

Three probes asked whether the compression objects make the seat's solve cheaper. All
three came back negative on cost while confirming the object was real, and the attribution
in each case is structural rather than an implementation accident.

| probe | object | the honest control | result (state set named) | why |
|---|---|---|---|---|
| S5h fiber-crush, `fiber_probe_2026-08-11.txt` | the r3 class DAG as a memo (arm B) against the plain identity-key boundary cache (arm A1) — the raw-vs-class comparison was ruled a **strawman** (P-Q2) | A1 | B:A1 rung medians **4.737 / 4.297 / 4.903** at n = 4/5/6 (per-coordinate 3.983–5.041): the class DAG computes identical values at about **5× the cost**. The memoisation dividend proper is the manyfold (A1:A0 medians 0.166 / 0.024 / 0.010). Interior collapse is real anyway: n = 4 h0 carries 1,502,362 situations in 128,860 classes. Receipts bit-exact 240/240 · 24/24 · 6/6 | class identity is a function of the **future cone**, computable only after full expansion; retrograde identity cannot short-circuit descent the way a state key can |
| S5j endgame store, `endgame_store_2026-08-11.txt`, `endgame_floor_2026-08-11.txt` | a symmetry-reduced tablebase keyed by r1 canonical form (Lemma E: equal forms ⇒ equal count-free values), 17 coordinates, 1,685 hit receipts re-expanded to terminals bit-exact | the plain A1 cache (T0) and closed-form last-trick resolution | tablebase arms **lose at evaluation, 1.57×–2.69× slower** at every coordinate (canonicalization ≈ 4.6 µs per form vs ≈ 0.1 µs per state-key probe); the convergence is **real** — **830,399 form-hits**, ≈ 38% of level-2 boundary probes, 73% at hand 3; 1,358,231 level-2 records, saturation not reached; T1′ (closed-form bottom) wins at 0.88–0.99 of control; the level-1 floor **table** is a 41× negative (1,430 ns vs 35 ns closed form) | structural identity *can* short-circuit descent, but harvesting it costs a canonicalization per distinct state, which under a plain memo exceeds the harvest at level 2 |
| S5i fiber-refinement, `fiber_refine_2026-08-11.txt` | Lemma X (deleting worlds of value zero leaves the unnormalised objective and its argmax unchanged, one-sided) plus a predicate engine over the class store | the cheapest **storeless** route (anti-strawman, X-A13) | the engine is essentially free (0.1–3.7 ms per pass vs 197–958 ms storeless; reachability/confinement have no storeless alternative) and **bites nothing**: X_val0 flags 0.1–3.5% of classes and **0 of the 3,120 + 96 evaluated worlds** across the n = 4 and n = 5 rungs; exclusion saves nothing at evaluation once the store is paid | V* = 0 requires losing every remaining trick under world-informed play, and the corpus focal is the declarer. Remnants are **analyst conditioning** (§6.8) — a third thing beside support and belief |

Two side findings of S5h that later programs rest on: cold treatment H **completed** on
the full 34,650-world void-free fiber at every eligible n = 4 coordinate in 6.86–16.63 s
(`fiber_probe_h_2026-08-11.txt`; dag-v1 13×–125× less work than tree-v0), which is what
made the n = 4 exact columns of §3 available; and the weighted re-solve over a fixed class
DAG — the number the belief/policy-iteration platform claim actually rests on — remains
**UNMEASURED** (P-A14). The persistence discipline minted here (append-only,
content-addressed, cache-never-authority, digest mismatch = corrupt not stale) is
inherited by every later store.

### 1.4 §12.6A in one page

The diagnosis that produced it (S5d, 2026-08-10 evening, [walt-program](walt-program.md)
reset 3): the S4 lumpability instantiation compared observation and feature alphabets
**raw** — it quotiented the state side but never the interface alphabets — which is
exactly why only world-reconstructing descriptors could pass. Jason authored the missing
theorem in session as `walt/math/equivariant_lumpability_v0.5.md` (662 lines; v0.4 stays
frozen). **Equivariant controlled lumpability (ECL):** two situations are the same when,
given what the seat knows and does not, one policy applied through declared typed
transports Θ to any matching world produces the same outcome under the count-free
quotient. The theorem gives exact compression (belief updates push forward with the
quotient kernel; lifted abstract policies are lawful with the same joint law); the
corollary descends the §8 valuation gauge; v0.4 §12.6 is recovered at identity
transports. Its own ledger listed the existence of a nontrivial (d, Θ) satisfying ECL as
OPEN; S5e resolved it **YES** on the trick-six carrier (§1.2(b)) — and then §1.2(c) showed
that at the first play the only transports are the seven-fold declaration transports.
Later, on 2026-08-17, the pmake advisory ruling (`walt/math/WALT-MATH-RULING-2026-08-17-…`)
stated the concrete ECL instance for the playing seat and recorded an honest negative on
hand 8's trick-4 pool: no nontrivial (π, θ) survives, predicted compression < 2×.

---

## 2. Decision sparsity

The reframe (2026-08-12/13, [walt-program](walt-program.md) resets 5–6): the opening
truth may be high-dimensional while the opening **decision** is sparse. This section is
the evidence for both halves.

### 2.1 Lemma R(c) and the Gate B refutation (S6a, 2026-08-12)

Jason's v0.6 predictive algebra offered a linear escape from §1: linear rank can sit far
below partition-lump size. Adjudication converted most of it into a theorem before any
code ran. **Lemma R(c)** (`walt/CENSUS-RULINGS.md` § "Predictive-rank probe rulings",
R-A1..R-A24): in Straight 42 every tile is eventually played and every play is publicly
attributed, so a complete continuation record determines the latent world; hence every
closure whose terminal seed contains a nonzero constant has predictive dimension exactly
|X| at every interface. The trick-count distribution contract and its next-leader
enrichment are therefore **theorem rows** (R-A24), and exactly one object remained
measurable: dim V^val, the value closure of the count-free expected-trick contract (zero
terminal seed). **Corollary R-fold:** predictive dimension is declaration-fold invariant;
bases and matrices are freeze-relative and never fold-compared.

Gate B's criterion was fixed before any number existed (R-A20): with D(n) the per-grade
maximum of dim V^val, the payoff is CONFIRMED iff dimension growth is at most one third of
fiber growth at both steps, REFUTED iff it reaches two thirds at either.

| grade | \|X\| | coordinates (freeze 25 decimation) | dim V^val | behavioural rows |
|---|---|---|---|---|
| 1 | 6 | 12 | 1 at all twelve | 1 or 2 |
| 2 | 90 | 6 | 42, 42, 52, 54, 56, 59 | 43–72 |
| 3 | 1,680 | 3 | 1461, 1492, **1680** | 1470, 1505, 1680 |

(`predictive_rank_2026-08-12.txt`, 52,021 ms, correctness gate MET at every coordinate
against the freeze-26 authority, fold 7/7.) D(2)/D(1) = 59 against a fiber ratio of 15;
D(3)/D(2) = 1680/59 ≈ 28.5 against 1680/90 ≈ 18.7: the dimension grows **at least as fast
as the fiber** at both steps. **Gate B REFUTED at its pre-declared criterion.** The
mechanism is the same public-attribution structure that killed the structural quotient —
the field-share weights are world-discriminating even with the constant excluded. Not
killed by it, and each became a later section: root-action argmax partitions (§2.2–§2.5),
dual policy geometry (§2.2), and moment compilation for fixed shallow queries (Gate D,
never designed; freeze 39 reserved). Fences: a predictive dimension licenses no runtime
claim and, by P-A21, three rungs are not a law — no dimension is quoted for the opening.

### 2.2 Policy geometry, with the STOPPED verdict (S6b, 2026-08-12)

Gate E asked whether the enormous information-consistent policy set induces few distinct
value vectors. **Proposition G-flat:** grades 1 and 2 carry no policy geometry at all
(the continuations are forced); at grade 3 the only free layer is trick 2 with
N_pol(a) = 2^k(a). **Lemma G:** backward Pareto pruning is exact through positive
composition, the incremental fold is mandatory, N_vec is destroyed by pruning; the design's
own definition of exposure was not pruning-safe and was replaced by the unique maximiser
(PG-A4). PG-A13 fixed the cap discipline: a capped coordinate reports **no** N_par.

| coordinate (grade 3, \|X\| = 1,680) | lead | k(a) | N_pol | N_par | N_exp |
|---|---|---|---|---|---|
| idx = 0, hand [00 10 11] | 00 | 384 | 2^384 | 1 | 1 |
| idx = 0 | 10 | — | — | **STOPPED** (cap 16,384) | STOPPED |
| idx = 0 | 11 | — | — | **STOPPED** (cap 16,384) | STOPPED |
| idx = 1299709, hand [21 22 63] | 21 | 7416 | 2^7416 | 1 | 1 |
| idx = 1299709 | 22 | 6018 | 2^6018 | 1 | 1 |
| idx = 1299709 | 63 | 19930 | 2^19930 | 1 | 1 |
| idx = 2599418, hand [11 22 42] | 11 | 8748 | 2^8748 | 1 | 1 |
| idx = 2599418 | 22 | 7146 | 2^7146 | 1 | 1 |
| idx = 2599418 | 42 | 6018 | 2^6018 | 1 | 1 |

(`policy_geometry_2026-08-12.txt`, 74,365 ms; the grade-1 and grade-2 rows are 24
receipt rows reading 1 by proposition, never evidence of collapse.) **Seven of nine
(coordinate, lead) pairs have a singleton Pareto frontier** — one policy weakly dominates
every lawful alternative in every one of the 1,680 worlds. **Two of nine STOPPED**: the
non-boss trump leads at idx = 0 blew past the declared cap. **The formal verdict is
STOPPED / NO VERDICT** (PG-A13): a capped coordinate forbids the global claim, and this
dissent travels with every citation of the 7/9 — it is never presented as seven-of-nine
success. The bimodality is the finding: total strategy-side collapse almost everywhere,
genuine frontier explosion exactly where the 42 is tense. No similarity or tolerance
claim is made (PG-A17); a vector here is an expected-trick profile over a declared
fiber, not an outcome law.

### 2.3 The deadness census (S6c, 2026-08-13)

Jason's framing ("junk everywhere — I can't beat anything and nothing I play changes any
outcome") and his binding constraint, verbatim in intent: **count vetoes elimination** —
`hand ∩ COUNT = ∅` is a firing conjunct of every detector, not an option. Three one-sided
detectors, each proved before it ran (J-A1..J-A18; [walt-math-deadness](walt-math-deadness.md)):
**D0** (Proposition J-0, no-possible-winner plus the guard, no exhaustion margin needed),
**D1-sym** (Proposition J-1, the two-tile transposition isomorphism), **D1-win**
(Proposition J-win, count-free only — its verdicts are void the instant count re-enters).
Ground truth is the one-deviation tie classifier, a **superset** of exact decision-deadness,
so recall against it understates the detectors (J-A10). Typing (J-A1): *forced*
(|legal| = 1, no decision) / *decision-dead* (every information-consistent policy
value-identical — the object) / *dominant* (one Pareto-optimal vector); S6b's singleton
frontiers are dominance, not deadness. (Decision-dead is also not *decided* — the pmake
ruling's banked cutoff — and not *laydown* — the typed universal quantifier of the later
anytime program; the three are distinct objects, see [walt-math-deadness](walt-math-deadness.md).)

Over 45 units (3 grade-3 coordinates and 9 eligible n = 4 receipt-rung coordinates,
each crossed with its leads; n = 5 a declared stop), `deadness_2026-08-12.txt`:

| quantity | value | state set |
|---|---|---|
| detector calls | **174,250,255** | every call site of the 45 units |
| fires | 27,980,333 (D0 1,408,468; D1-sym 26,667,301; D1-win 0) | same |
| false positives | **zero** — every fired site ground truth could classify was indifferent; every J-A14 V/Q assertion held | same |
| one-deviation ties | **25,255,316 of 49,522,677 classified call sites (51%)** | call sites with support ≤ 400 |
| recall against the tie denominator | D0 138,208 / D1-sym 8,263,821 / D1-win 0 / any **8,335,057 (≈ 33%, understated per J-A10)** | the 25,255,316 ties |
| D1-sym at its best unit | 2,358,996 of 2,418,996 ties (97.5%) at n4 hand 12 lead 65 | that unit |
| D0 total | 7,416/7,416 at idx = 1299709 lead 21; 19,924/19,930 at lead 63 | that root family |
| open mechanism (J-A8) | the trumpless-junk family idx = 0 has 276 / 1,239 / 1,773 ties per lead and **zero hits**; no fourth detector without a proof | idx = 0, three leads |

**Cost, from the only quotable instrument.** The freeze-43 sequential timing rung
(W = 1, one process, 18 cores, release build; `deadness_rung_2026-08-13.txt`, four lines):
grade-3 unit idx = 0 lead 00, plain 26 ms vs detector-arm 24 ms, **6,745 ns over 384
calls (17 ns/call)**; n = 4 hand 0 lead 00, plain 28,125 ms vs detector-arm 28,357 ms,
**151,339,539 ns over 3,540,143 calls (42 ns/call)** — under 1% of the solve. The
"≈ 25 ns/call" figure in `walt/LOG.md` and older wiki text is contended (a W > 1 run) and
not quotable (DS-A32). The rung run is what several 2026-08-24 pages still describe as
"unrun" — see Appendix C.

### 2.4 The one inequality, and the E-series

The decision-sparse parent (`walt/math/decision_sparse_exact_solving_v0.1.md`, received
2026-08-13 at commit 8ee1c9e, kept verbatim) with its maintained errata moved the target
from *compressing truth* to **proving the root action**. For each root action a, a
**primal witness** L_a (the exact value of a *fixed* lawful information-consistent policy,
no maximisation below the root — Lemma E4) and an **upper witness** U_a (treatment C —
Lemma E3) give

> **L_a ≤ Q^H(a) ≤ U_a** (Theorem E6.3, adjudicated name *value sandwich*),

and the root action is proved as soon as **L_{a⋆} ≥ U_a for every a ≠ a⋆** (Theorem E6.4,
root-action separation; member-not-set in the statement). Later programs renamed the
object: this pair is the ancestor of the **root interval** and **survivor set** of the
counted-belief program (CBS-A3 rules that "sandwich" is not an object name), of
**certified regret** Γ = U* − B_exec (the anytime program's term of art), and of the
focal-horizon hierarchy [L_{a,k}, U_{a,k}] — see
[walt-decision-sparse](walt-decision-sparse.md#lineage-since-2026-08-24). Four
attached conditions hold jointly or the separation is void: same field (C1), same
belief and same world set with **no decimation inside L or U** (C2), same contract
(C3), perfect-information minimax is not a substitute (C4). DS-A1 bars the word
"certificate" for this object forward from 2026-08-13; the D3 sense on
[reachability](reachability.md) is the **necessary outer profile**, and the two never blur.

| name | one line | what it repairs / status |
|---|---|---|
| Theorem E1 / E1′ | order exchange under a declared (fully transported) involution Θ intertwining the kernels, over a Θ-closed policy class | replaces parent §7.1, **UNSOUND as written** (DS-A6); E1′ new (DS-A19) |
| Definition E2 / Prop. E2.1 | advantage dimension, affine and reference-free; the reference form is off by exactly one | replaces parent §5 (DS-A3/A4) |
| Lemma E3 / Remark E3.1 | the action-conditioned upper witness is valid pointwise by strategy fusion; the unconditioned aggregate is valid but action-constant, hence vacuous | formalises parent §8.2/§15.4 (DS-A7) |
| Corollary E3.2 | zero global fusion gap ⇒ the whole remaining difficulty is primal | new (DS-A22) |
| Lemma E4 / Non-theorem E4′ | a fixed lawful policy priced by a fixed-policy evaluator is a lower bound; priced by *any* world-informed evaluator it is an **upper** bound and the interval inverts (two-world witness) | formalises parent §8.1/§15.3; the soundness failure mode (DS-A14) |
| **Corollary E4.1** | (1) L ≤ Q^H for every candidate set; (2) H-argmax seeds give L = Q^H for every tie rule; (3) **if Q^H(a⋆) < U_a then no finite candidate set separates the pair** under that relaxation — the only lever is a tighter relaxation | ruled at SEP-A2; filing as errata §4.3 **still owed** as of 2026-09-07 — `walt/CENSUS-RULINGS.md` is its only authority |
| Proposition E5 (+ E5.0–E5.2) | atom-mass **linear** filtering is noncompressive on this carrier (Lemma R(c) again) | parent §10.3 sound but degenerate here (DS-A5, A20, A21) |
| Theorem E6.1 | width monotonicity on a common world space; W_all = N_exp at the full simplex | parent §4.3 typed (DS-A2); superseded in part by Definition E9 |
| Theorem E6.2 | backward Pareto pruning exact; incremental fold required; pruning destroys N_vec | parent §6.3 completed (DS-A4, A26) |
| Theorem E6.3 | the value sandwich, hypotheses inside the statement | parent §8.3 (DS-A7) |
| Theorem E6.4 | root-action separation; strict inequalities give uniqueness; member-not-set in the statement | parent §8.4 |
| Theorem E6.5 | finite adaptive gluing terminates under (G1) proved relaxed bounds and (G2) a search over the whole optimal face; the termination bound is doubly exponential and **never a complexity claim** | parent §9.2 with two obligations (DS-A3) |
| Lemma E7 | dominance travels only under an exhibited value-order isomorphism plus the transported belief; a policy transport establishes lawfulness only | new (DS-A25) |
| Lemma E8 / Lemma J(c′) | J-0/J-1 verdicts survive every valuation whose tile-value schedule is constant on the focal hand | new (DS-A24); corrects DS-A9 |
| Definition E9 | interface-local reachable decision width | new (DS-A23); freeze 40 reserved for W_reach |

Citation rule (DS-A17): the errata theorem number for the mathematics, the DS-A ruling
for provenance; the errata governs over the parent. Formal statements:
[walt-math-decision-sparse](walt-math-decision-sparse.md).

### 2.5 The thesis in one measured instance (S6d, 2026-08-13)

All three grade-3 coordinates **SEPARATED** — the branch's first exact root-action
proofs — `separation_2026-08-13.txt`, gate MET, receipts R1–R5 HELD, 3,942 ms:

| coordinate (\|X\| = 1,680) | root a⋆ | margins L − U against competitors | prices U_a − Q^H(a) per action |
|---|---|---|---|
| idx = 0, [00 10 11], 0-trump | **00** | **449/1120** vs 10; **59/2240** vs 11 — precisely the two leads S6b could not complete | 00: **0**; 10: **11/1120**; 11: **29/420** |
| idx = 1299709, [21 22 63] | **22** | 1/42 vs 21; **1/63** vs 63 (U exactly tight at the runner-up) | 0, 0, 0 |
| idx = 2599418, [11 22 42] | **11 and 22** (tied H-optima, each against the other at margin 0) | 1/21 vs 42 | 0, 0, 0 |

**Seven of nine per-action prices are exactly 0**, and the only two nonzero prices sit
precisely at the two leads where S6b's frontier exploded. Corollary E4.1 was delivered at
this adjudication and reorganised the reading: with H-argmax seeds L = Q^H necessarily, so
every verdict is decided by the U side, and a NOT-SEPARATED pair would have been an exact
negative. Two sentences travel together and neither stands for the other (SEP-A15(ii)):
*the frontier is unnecessary for the root decision*; and *this run does NOT test the
parent's economy claim* — it computes the exact H solve at every action because DS-A10's
receipts require it. Candidate library v1 (freeze 36) was written with four entries; the
384-versus-108 naming rule for the idx = 0 playbook is at EC-A12 (use 384, the
receipt-backed free-decision count).

---

## 3. The interval at four tricks

### 3.1 The carrier and its fences

Nine real-deal n = 4 coordinates: the bidder's hand and pool at trick 4 of nine of the
13 `verify_player.txt` receipt hands (h0, h1, h2, h4, h5, h6, h8, h9, h12; h3, h7, h10,
h11 out of scope because the leader is not the declaring seat), each over the void-free
fiber of 34,650 deals — the voids the play record had already revealed are deliberately
discarded, and at h2, h5 and h8 the void-filtered fiber is a proper subset (23,100;
14,700; 1,200 of 34,650), a column that licenses nothing. Every value below is the
count-free trick differential in the count convention under the uniform belief and
uniform-legal field. **No row is a statement about correct play in that deal, about
reachability, or about any belief other than the declared one** — and nothing measured
at grade 4 is quoted for trick 1 or the opening.

Three probes fed the master table: S6h (the separation pass, `separation_n4_2026-08-14.txt`),
S6k (the fusion tax, `fusion_tax_2026-08-14.txt`, over the five negative-margin
coordinates), S6l (the second rung, `second_rung_2026-08-14.txt`, at h2 and h9), with S6j
(`rule_economy_n4_2026-08-14.txt`), S6m (`feature_fee_v11_2026-08-14.txt`) and S6n
(`fc_correlation_2026-08-14.txt`) beside them.

### 3.2 The master table

| coord | pip, hand | S6h verdict | binding margin L − U (count) | Δ¹ (rung one) | Δ² (rung two) | escape states | oracle-θ fee capture (F2) | cheapest rule that separates (S6j) |
|---|---|---|---|---|---|---|---|---|
| h0 | 3, [00 21 32 53] | **EXACT NEGATIVE** (a⋆ = 53 vs 00) | −300647/2138400 | 19863799/179625600 | 387281/5132160 | not run | **76.4628%** over the 574 leading states of unit a = 00; 29.2679% over its 758 following states; 75.1420% over all 1,332 swept states | none (E4.1(3) receipt) |
| h1 | 6, [11 43 60 66] | **SEPARATED** (root 11) | +23/2970 (vs 60 and 66) | not run | not run | not run | not run | P2 greatest-tile at economy gap **exactly 0** |
| h2 | 5, [21 33 53 54] | **EXACT NEGATIVE** (53 and 54 tied; each vs the other) | −9557/554400 (the tightest of the nine) | 145/22176 | 1483/138600 | **36 of 330** first-frontier states (one signature) | **exactly 0** over the 216 swept states at each of the two units, 3,126 breakpoints each | none (E4.1(3) receipt) |
| h4 | 1, [21 40 51 65] | **SEPARATED** (root 65) | +4318547/19958400 (vs 21 and 51) | not run | not run | not run | not run | all four rules P1–P4 |
| h5 | 5, [31 51 55 63] | **SEPARATED** (root 55) [void-filtered 14,700] | +8023709/43545600 (vs 63) | not run | not run | not run | not run | P2 greatest-tile |
| h6 | 4, [11 40 43 53] | **EXACT NEGATIVE** (a⋆ = 40 vs 11) | −8524657/479001600 | 611579/21772800 | 5399143/479001600 | not run | not run | none (E4.1(3) receipt); **CLOSED by the rung-one gluing cut**, surplus L − U^(1) = 4930081/479001600 → Opt^H(h6) = {40} |
| h8 | 5, [21 31 33 55] | **SEPARATED** (root 55) [void-filtered 1,200] | +5003513/43545600 (vs 33) | not run | not run | not run | not run | P2 greatest-tile |
| h9 | 4, [30 41 54 61] | **NOT PRICED** — count-only pass COMPLETED at **517,562,322** partition states, 2.7× P_max v2 = 192,000,000; no primal witness | −2116837/8870400 at both H-optimal actions 41 and 54, from the filed Q^H and U alone (RW-A3(i)): the worst of the nine | 227251/3326400 | 4532503/26611200 | **498 of 1,320** first-frontier states (tile 61 at every one) | not run | none can (E4.1(3)); the best rule (P2) lands within **1202339/8870400** (≈ 0.136 trick) of the optimum at the binding actions |
| h12 | 0, [20 30 40 65] | **EXACT NEGATIVE** (20, 30, 40 tied; each vs each) | −364429/9979200 | 34519/1995840 | 95917/4989600 | not run | not run | none (E4.1(3) receipt) |

Reading the columns. **Verdicts** (S6h; every coordinate reached Tier 1 — authority gate
MET, R6 step-determinism, R1, and R7 HELD, so Lemma N, the pooled-cost decomposition,
held to the step at all nine): 4 SEPARATED (member-not-set verbatim on each), 4 EXACT
NEGATIVES — Corollary E4.1(3) fired for real: *no candidate set whatsoever* separates
those pairs under relaxation C — and 1 NOT PRICED, a stop that is a result with its exact
count printed. **Δ¹ and Δ²** (S6k): between treatment C and treatment H sits the
reveal-delay ladder C^(k); at grade 4 it has exactly two rungs (Corollary FT-grade4, from
Lemma FT-trunc: the last decision is over a one-tile hand and is forced), so
U_a^C − Q^H(a) = Δ¹ + Δ² is the **complete** layer decomposition and one computation of
U^(1) determines it; the identity held on real data at all twelve binding pairs
(FT-A26(i)). Of the twelve pairs, **one closed** (h6) and eleven did not — the ten tied
pairs with shortfall exactly Δ² (an arithmetic identity, not evidence — FT-A26(ii)) and
h0's untied pair. The tax is sparse and non-uniform: **12,639 of 281,542** frontier states
across the nine units (4.49%) pay a positive tax — h9 1,296/1,320, h2 216/330, h0
1,332/16,136, h6 4,041/53,570, h12 1,414/69,512 — and every minimal fusion core is binary.
**Escapes** (S6l): the slack–tax interchange law Δ² = Σ_I min_b [s_{I,b} + d_{I,b}]
instantiates exactly (ten receipts HELD at all four units; the adjudicator re-derived
every quantity with zero deviations at 3,300 states); the minimising first action leaves
the rung-one optimal face at 36 of 330 states at h2 and 498 of 1,320 at h9 — the first
measured policy adjustment in the branch. A witness taxing only the optimal face would
have overstated the tax by 4.0459% at h2 (1543/138600 vs the true 1483/138600) and
11.7881% at h9 (12667/66528 vs 4532503/26611200; Proposition SR-loc), so **every future
rung-two lower witness must cover every first action**. These counts are one structural
phenomenon reached by many field continuations, **never a rate**.

### 3.3 The mandatory sentences

Three sentences travel with this table verbatim wherever any part of it is quoted.

- **FT-A25(vi), on the h6 closure:** *this coordinate's optimal set was already
  determined by the filed Q^H column; what this verdict demonstrates is that the
  two-sided proof architecture now closes here, and that the lever was a gluing cut and
  never a better candidate — which is exactly what Corollary E4.1(3) proved was the only
  lever available.* The closure could have failed — it did at the other eleven pairs — so
  the run is a genuine test; the conclusion could not have come out otherwise, so it is
  not evidence about the game. (The composed verdict Opt^H(h6) = {40} mixes two
  relaxations under Lemma FT-mix and receipt (FT-R8), whose sharpest clause asserts that
  the competitor row set equals the legal action set at the rebuilt kernel minus a⋆,
  never a row count in a filed file.)
- **"A cross-check is not a witness."** Both of h9's filed columns have each been
  reconstructed once by a different route — its U by (FT-R1) from the frontier
  decomposition, its Q^H by the depth-two ladder identity (SR-R4) — and h9's NOT PRICED
  label stands verbatim: NOT PRICED is a statement about the primal pipeline, and
  agreement between two computations of the same quantity does not manufacture the
  object the pipeline could not build. No composed verdict may be filed at h9
  (FT-A25(vii)).
- **The selection fence.** The five fusion-tax coordinates were chosen by negative
  binding margin: they are a **carrier, not a sample, and the selection criterion is
  correlated with the quantity being described**. No distribution measured on them —
  tax sparsity, escape rate, fee capture — may be read as a distribution over
  coordinates or hands; nothing causal is claimed; nothing is quoted for trick 1.

### 3.4 The economy claim, primal half only (S6e, S6j; the S6f NO-GO)

The parent's economy claim — "the solver does not need an exact solution for every
action" — splits (EC-A13) and is never written as one thing. The **primal half** asks
whether the witness at a⋆ must be an exact solve. **S6e** (`economy_seed_2026-08-14.txt`,
grade 3) seeded L from four fixed tile rules (P1 least-tile, P2 greatest-tile, P3
beat-if-able, P4 trump-hoard), a transport arm T (structure-proving, gap zero by
Corollary S-fold-val) and a heuristic re-key R: a seed separates iff its economy gap
g = Q^H(a⋆) − L is at most the slack s(a⋆) = Q^H(a⋆) − max U (identity R8). At idx = 0
(slack 59/2240) **P2 and P4 separate at gap zero** while P1 and P3 fail by 3/7 — sixteen
times the slack; at idx = 1299709 (slack 1/63) every arm separates; at the zero-slack
control idx = 2599418 every arm prints "ZERO-SLACK: SEED EXACTLY OPTIMAL (NOT ECONOMY)"
as pre-declared by theorem. Verdict label: **CERTIFIED-CHEAP** at both positive-slack
coordinates. **S6j** took the same question map-free to n = 4 — a rule walk reading only
(record, legal) in O(1) memory, (RW-R2) blocking HELD against the materialised
50,712-state map at shared ground: every coordinate where separation is possible at all
is separated by a four-word rule (the rule column of §3.2); the exact-solve seed was never
needed anywhere separable at trick 4 on real deals. At h9 the rule walks reach
37,136,634–105,131,400 states in O(1) memory against the 517M-state map — a reached count,
never a cost claim.

The **full** claim additionally needs the U side cheapened, and the h6 closure is the only
pair where that has happened; S6k itself prices U exactly everywhere else. **Any sentence
saying "the economy claim was tested" without the word *primal* has over-claimed.**

**S6f** (`separation_n4_rung_2026-08-14.txt`) is the gate failure filed as a result: the
freeze-44 budget refactor landed (every walk-based evaluator carries a deterministic
walk-step budget; on exhaustion no partial fold is retained), (R0) the blocking
regression PASSED, and the §5 measured rung returned **NO-GO** — the U side is affordable
(estimated whole-fiber revealed cost 4,327,256,587 walk-steps against a 4 × 10^10
budget) but the partition build at (h0, action 0-0) stopped at the P_max v1 cap of
32,000,000 states, where the design's labelled estimate had said 24,825,150. The
artifact's partition line prints "states 0, walk-steps 10,000,000,000, cap_hit true";
N4-A13(i) rules that printed step count a **poison artifact** (the build zeroes the
budget cell on cap exceedance) and confirms the stop was the cap, not the budget — so
the prose "> P_max" and the artifact's "cap_hit" are the same event, and the rung's
post-discard resident size (2,797,840 KiB against 32,000,000 states, ≈ 89.5 bytes per
state) is what set P_max v2 = 192,000,000 at N4-A16. The declared fallback {h6, h4, h8}
failed its own arithmetic at h8, and per N4-A12(c) the pass returned to the rulings
file rather than being nudged.

### 3.5 What retired grade 4 (SR-degen)

**Proposition SR-degen:** at grade 4, U^(2) = Q^H (Corollary FT-grade4) and
L = Q^H at a ceiling-attaining witness (Corollary E4.1(2)), so L_{a⋆} ≥ U_a^(2) holds
**unconditionally at every binding pair**, strictly exactly when the pair is untied,
with surplus Q^H(a⋆) − Q^H(a). **No grade-4 experiment can test whether the second rung
closes a pair**; the answer is fixed by two already-filed columns. S6l therefore reported
**no closure verdict at all** — only the identity, the decomposition and the escape
census — and every rung-two fusion core has size exactly 2 by arithmetic (the received
note's "cores remain binary" question is unmeasurable below grade 5). There is no rung
three at grade 4. **Grade 4 is exhausted as a test-bed for the ladder**; the next real
question needed a coordinate where a rung's value is not already known before it is
computed. That is where the program stood on 2026-08-16, and it is what the counted-belief
program answered by a different route (the lineage section of
[walt-decision-sparse](walt-decision-sparse.md#lineage-since-2026-08-24)).

### 3.6 The fee story (S6m, S6n — 2026-08-14)

Which cheap structural quantities price the rung-one tax? For a feature φ of (world,
action) at a frontier state, the fee is θ·(φ − centre) and the question is how much of the
state's local tax an optimal θ removes. Three results fix the reading: **Proposition
FF-blind** — an action-blind fee removes exactly zero (the penalty-side twin of T1-blind
and FT-flat: *a witness, a bound or a fee must be conditioned on the decision it prices*);
**Lemma FF-min** — the objective is convex piecewise-linear and exactly minimisable over
its breakpoints, no grid, no float; **Proposition FF-oracle** — optimising θ per state is a
lookup table, not a feature basis, so a **low** capture refutes conclusively and a **high**
capture licenses exactly one follow-on, the shared fit. **Proposition FF-degen** — zero
breakpoints is exactly vacuity — is what made a defect in the first run's frozen feature
list catchable from the committed file: six of twelve (feature, unit) cells were vacuous
by construction and are typed **unmeasured, not zero** (freeze 52 v1.1–v1.4).

Results, each with its state set (`feature_fee_2026-08-14.txt`, `feature_fee_v11_2026-08-14.txt`):

- **Jason's boss-keyed feature F1 is REFUTED where it has a domain**: over h0's 574
  leading states (the only part of this carrier where a boss trump survives to the
  frontier), swept across 23,016 breakpoints, oracle-θ capture 88457474377/24082518161460
  ≈ **3,673 ppm** (the leading-part fraction as stated in the FF-A rulings; the results
  file prints the all-states capture 88457474377/24775917854710 ≈ 3,570 ppm at its F1
  summary line and the leading/following split beneath it); a family that cannot break
  0.37% with 574 free parameters cannot break
  it with one. Elsewhere it is inapplicable — a boss-keyed feature's domain shrinks as the
  hand simplifies. Table reasoning is a source of hypotheses and no kind of evidence for
  them; losing this job says nothing about Jason's reading of that hand.
- **The beatability feature F2 bites at h0 and not at h2**: 2841944614/3716765745 ≈
  **76.4628%** over h0's 574 leading states, 29.2679% over its 758 following states,
  75.1420% over all 1,332 swept states; **exactly 0** over h2's 216 swept states at each
  of the two units, with 3,126 breakpoints each — a refutation, not a tautology.
- **The shared fit**: one pooled θ* = −56/45 over the same 574 leading states gives
  61431886/80449475 ≈ 76.3608%, i.e. **≈ 99.87% of the per-state oracle survives
  collapsing 574 free rationals to one** (27 distinct per-state optima, none zero). The
  one confound-free comparison: on the same 574 states the two action-conditioned
  candidates return 0.3673% and 76.46%, about **208×** apart.

**Why h2's zero is zero (S6n, `fc_correlation_2026-08-14.txt`).** **Proposition FC-drop**:
capture ≥ correlation × reach, an exact lower bound needing no minimisation, with a null
control fixed by theorem (Corollary FC-null). **Proposition FC-width**: the subgradient's
width is the mass-weighted spread of the feature across the clairvoyant tie — without ties
the interval is a point and a zero capture would demand an exact rational identity; with
ties it has positive width and zero is robust. Measured: at both h2 units the two one-sided
slopes **strictly straddle zero at 216 of 216 swept states, neither slope zero anywhere**
— the zero is **tie-driven, unanimously** — with a non-singleton clairvoyant argmax at
236,784 of 362,880 (state, world) arrivals per h2 unit (65.25%) against 59,776 of 266,132
at h0's one unit over its 1,332 swept states (22.46%). Consequence: a **pre-fee screening
statistic** (the argmax cardinality profile) exists, computable before any fee is built.
Screen quality, in the words the ruling binds: over h0's 1,252 straddle-false states the
bound is attained at 258 (20.61%) and the summed bound recovers 14.873% of the summed
capture; **a positive bound proves a fee bites at that state; a zero or small bound proves
nothing**. The house form minted here: *do not grade an instrument — state what follows
from a positive reading and what follows from a negative one.* Everything in this
subsection is rung one only, on a carrier chosen by negative margin, at grade 4; the
graded boss feature is proved positive at 322 of 574 states and is likely negligible
(the refuted binary is straddle-false at 374 of the same 574 and cashed out at 0.367%).

### 3.7 The seed survey (SS, 2026-08-15/16) — the ending

The first carrier in the branch **not selected by outcome** (SS-A1..SS-A18, freeze 54,
`seed_survey_2026-08-15.txt`, 703 lines; companion 4.59 GB out of tree, SHA-256
`bbf151e9…`): 100 seeds by index(n) = (n · 292,032,399,099,041) mod 472,518,347,558,400
(the least prime at or above D/φ, frozen before the build), mixed-radix combinadic
unranking, declaration PipTrump(n mod 7), seat 0 leads, the freeze-26 least-index policy
plays three tricks, the trick-3 winner is focal, **every legal root action is a unit**:
400 units, **0 declared stops**, 100 distinct coordinate keys, receipts (SS-R3)–(SS-R9)
HELD, the complete-face receipt asserted at every one of the 20,833,948 depth-one
frontier states. The design defect it repaired is worth recording: "seed n → deal n by
the canonical enumeration" would have produced a hundred deals sharing one first hand
(the first hand does not change until index 399,072,960).

- **The pre-declared association (SS-A7(b): tie multiplicity tracks separation
  structure) is NOT REPORTABLE** (SS-A15). The per-unit and arrival-pooled conventions
  agree on the total (450,298 vs 463,222 ppm over the 400 units) and **reverse the
  verdict-cell ordering** (484,114 > 448,365 > 447,315 vs 469,362 > 465,742 > 422,608);
  the contrast is carried by frontier size, and the survey's own confound table
  **does not separate cleanly** (three of four strata agree in sign with the overall
  contrast; smallest cell 20). The multiplicity → separation-difficulty arrow of the
  §3.6 screening chain is broken at unselected coordinates; the h2/h0 fee contrast is
  not refuted, merely not corroborated.
- **The finding nobody pre-declared: optimal root actions produce smaller depth-one
  frontiers.** Opt^H units are 52 of 101 in the smallest-|I₁| quartile (51.5%) against
  about a fifth in each other quartile; within-coordinate, among the four actions of one
  hand, the optimal action is the smallest-frontier action in **43.5% to 58.8%** of 85
  clean seeds (ties → worst [37, 28, 11, 9]; ties → best [50, 19, 11, 5]) against a 25%
  baseline — a convention-stated range, never a single percentage (SS-A17(ii)). It is
  an observation, not a test, with no mechanism claimed; its use is a **solve-free
  move-ordering statistic** for a count-pruning search (SS-A16(iii)).
- Off-carrier tax sparsity 1302799/20833948 = **6.25%** (against 4.49% on the
  margin-selected carrier: the selected carrier was sparser than typical); forced
  frontier states 7559571/20833948 = **36.3%**, typed per J-A1 as its own column and
  never as a deadness count; the survey mean 450,298 ppm sits above the median 425,217 —
  a right-skewed tail of high-tie units. SS-A16(vi)'s U-shape prediction (ties
  non-monotone in the count-decidedness distance) is **REFUTED** by the flip check
  (SS-A17(iv)): a count quantity does not transport to the count-free tie statistic.
- Ten units NOT PRICED on the primal-witness route (partition counts 194,292,666 to
  403,432,392 > P_max v2), which bars that route only; every such unit still carries its
  solve, its tie census and its taxes.

Nothing measured at grade 4 is quoted for trick 1 or the opening (P-A21). The seed survey
was the last research run before the pivot; no era page owned it until this one.

---

## 4. Two theorems about the rules

Both are short, self-contained and about the rules rather than the model — and both carry
the same risk (T1-A12): **every statement is proved relative to walt's implementation of
the rule algebra as read from `walt/walt/src/rules.rs` at adjudication time**, every
receipt is computed by that same implementation, and no receipt inside these sections can
detect a disagreement with the rules corpus. **The corpus check is owed before either is
cited outside walt.** (S1's differential against `rules42.py` over all 737,100 four-tile
tricks × 9 declarations with zero mismatches is an exploratory cross-check between two
implementations, not the corpus check; GT1-A3's prose-rules resolver of 2026-08-16 covers
the portable M0/M1 slice only.)

### 4.1 Theorem T1-draw (S6g, 2026-08-14)

The proposed instrument — a per-world adversarial-field lower bound and a cooperative-field
upper bound summed over the 399,072,960-world trick-1 fiber (the refuted "bounded
sandwich" of T1-A) — was **refuted by proof before any compute ran**: **Proposition
T1-blind** (a lower witness valid at every root action, as every hand-only counting
guarantee is, can never strictly exclude a competitor, since U_a ≥ Q^H(a) ≥ L for that
same a) and **Proposition T1-corner** (the corner bounds close at trick 1 only when the
focal seat holds the entire trump suit). What replaced it needs no relaxation.
**Theorem T1-draw:** on a closed, fully enumerated family of **294 declared trick-1
coordinates** (freeze 47), the focal seat takes all seven tricks against **every** field
behaviour in **every** world, so Q^H(a) = +7 for every trump lead; **Corollary T1-ruff**
prices the double lead strictly by the ruff it invites.

Result (`trick1_draw_2026-08-14.txt`, 2,891 lines, 159,678 ms): of the 294 coordinates, 7
are all-trump hands labelled TRIVIAL (no competitor, no decision proved); on the remaining
**287, Opt^H is determined EXACTLY** — every trump lead at +7, every double lead strictly
excluded with its exact q from an exhaustive integer count over all 399,072,960 worlds,
no decimation. Flagship: declaration PipTrump(6), hand {6:6 6:5 6:4 6:3 6:2 6:1 5:5},
**Q^H(5:5) = 7 − 143/5814 = 40555/5814 exactly**, so Opt^H is the six trump leads.
Theorem E6.4's member-not-set caveat is **discharged, not waived**: both sides are exact
values. The reduced-grade authority cross-check (T1-R2) held to the rational at grades 2,
3 and 4 (11/3 at grade 4); grade 5 was a declared stop, the authority budget of
200,000,000 exhausted at 199,999,988 steps. **Corpus arm:** none of the 13 receipt trick-1
hands draws (expected, filed as a result); their exact corner gaps 7 − k − E_β[f] are
4, 5, 89/15, 6 (×4), 92/15 (×5) and **19/3** — the file governs over the LOG's "4 .. 92/15".
The membership half is belief-free and field-free — a statement about the rules, the one
place in walt where the feasible-versus-reachable fence does not bind a verdict, because
the statement ranges over everything; the exclusion half is model-relative. *A drawing
hand is a hand that plays itself; the theorem says nothing about hands that require
judgement.*

### 4.2 Theorem LD and the four-laydown question (S6i, 2026-08-14)

Family lore: a single deal holds at most three lay downs — hands that take every trick
from the lead. **Theorem LD** (LD-A1..LD-A13, freeze 48): a hand is a lay down **iff**
(L1) its top trump run is at least as long as the outstanding trump set and (L2) every
non-trump's threat lies inside trumps ∪ hand. Corollaries: every lay down holds ≥ 4
trumps; T1-draw is a strict inner class (42 of 301 at every declaration).

Result (`laydown_2026-08-14.txt`, 234 ms; `laydown_catalogue_2026-08-14.txt`, 2,109
lines): **exactly 301 lay downs per declaration** of the 1,184,040 hands tested under
each, 2,107 (hand, declaration) pairs. Receipts: (LD-R1) HELD against an independently
derived closed form; (LD-R2) all 294 T1-draw members present, containment strict;
(LD-R3) the LD plan swept every trick in every world against every field behaviour at
reduced analogues (120,960–362,880 adversarial leaves); (LD-R4) HELD — all seven counts
equal, Corollary LD-fold receipted rather than observed (a later FT-A16(ii) sentence
saying "(LD-R4) remains owed" is stale; FT-A29 corrected it). **Phase 2: NO FOUR-LAYDOWN
DEAL EXISTS** — exhaustive over the complete catalogue from every full-suit anchor, every
declaration triple, every disjoint pair, the forced fourth hand tested under every
remaining declaration. The family's ≤ 3 conjecture is **PROVED** relative to Theorem LD
and `rules.rs`, and the maximum 3 is exhibited: [00 10 11 20 30 40 50] under PipTrump(0),
[21 22 32 33 42 44 62] under PipTrump(2), [51 52 53 54 55 65 66] under PipTrump(5),
leftover [31 41 43 60 61 63 64]. The question is **combinatorial, not a situation**: only
one seat declares and leads, so four lay downs can never be realised together; the
question is whether the 28 tiles partition so that each hand *would* sweep if it were the
one to declare and lead.

---

## 5. The lesson factory (S5a–S5d, 2026-08-10)

The conflict-driven lesson factory ([walt-factory-era](walt-factory-era.md)) was walt's
second direction: harvest failures from a regret walker over the 13-hand receipt corpus,
generalise each into a typed, labelled lesson, prune with it, and let an economy retire
lessons that stop paying rent — a **stance** borrowed from CDCL, never an algorithm, with
"never trust the solver" expressed in types. It is a **dead branch**: the machinery was
frozen on 2026-08-10 evening (reset 3), the producers are archive-only at `648f93a`, and
none of its numbers is quoted by any later program. Two things survive it.

**The one result worth keeping: the inventory was label-fragile.** Everything measured
through S5c-m2 was measured at the label (C, minimax-omniscient) — each world solved
alone, omniscient, then averaged — while the seat's actual label is (H,
fixed-uniform-legal): one choice per information state. S5c-m2 re-measured 15 value
lessons at the seat label and got 10 survive / 0 fail / 5 unmeasured (capped), concluding
"not label-fragile"; S5c-m3's memoized dag-v1 solver at budget 10^9 lifted the four capped
big early-trick fibers (`label_transfer_2026-08-10_r3.txt`):

| lesson | seat-label measurement (count-free, H) | verdict |
|---|---|---|
| h1 S2 t4 refutation | Q^H(4-3) = 79/11 < Q^H(6-0) = 111269/13860 | **FAILS** — at the seat's label the "refuted" action is better |
| h1 S2 t4 win | Q^H(4-3) = 79/11; best is 1-1 at 2183/270 | **FAILS** |
| h11 S1 t3 win | at its origin Q^H(2-0) = −1927714337/319334400 loses to 5-1 at −547477589/91238400 | **FAILS** at the origin; its two transfer decisions hold |
| h11 S1 t3 refutation | Q^H(2-0) = −1927714337/319334400 > Q^H(0-0) = −2897509283/479001600 | HOLDS |

Combined tally at the seat label, r2 and r3 cited together: **11 survive / 3 fail / 1
empty-basin of 15**, every failure on a big early-trick fiber where the omniscient and
seat-facing fields diverge most. The uncapped, unmemoized tree walk reproduced every
per-action Q^H vector byte-identically on all four decisions
(`h_tree_crossval_2026-08-10.txt`: 15,486,288,612 / 10,766,263,412 / 4,214,899,874 /
65,449,828,676 tree particle-steps in 718.5 / 549.3 / 208.4 / 3,693.8 s against
123,882,398 / 226,094,450 / 226,613,736 / 537,862,903 dag steps — ratios 125×, 47.6×,
18.6×, 121.7×): **real label fragility, not a memoization artifact**. The economy then
priced the three H-fails as measured zeros and the deletion rule fired on three lessons
(keys 04994a29c448b18e, 7f168f67d352a3e1, dd17eaf9856df52b), each **TRIGGERED and each
mechanically BLOCKED** for want of a registered independent H checker
(`economy_2026-08-10_r2.txt`); nothing was ever deleted, and the planned Python checker
was retired by the NO-RESCUE policy (Lean, not Python). The sixteen §16.11 lesson records
under `walt/probes/factory-results/certificates_2026-08-10/` are walt's own pre-DS-A1
record type and keep their name.

**The disciplines that survive** (all inherited by every later era): grades and labels
travel with every verdict; a lesson never quotes above its grade (the m1 seed count moved
11 → 10 when a sampled-basis dominance failed exhaustive re-examination at 90,090 worlds);
caps exclude and never sample, and unmeasured is never zero; H-primary pricing, never
summed; deletion requires two measured-consecutive zero-rent epochs and an independent
checker; probes are validators, never source. The dag-v1 memoized H solver built here
(`hidden_scalar.rs::action_values_dag`) became freeze 26, the concrete authority every S6
receipt compares against.

---

## Appendix A — the freeze register, 1–58

A **freeze** is a declared constant, encoding or ordering that a quoted number depends on;
it proves nothing and exists so a figure is reproducible from the repository alone.
Standing rules ([walt-math-freezes](walt-math-freezes.md)): numbers are never reused;
freeze-relativity is declared, not assumed; a stored artifact carries the freeze-set
digest and a mismatch is **corrupt, not stale**; a clause states a constant *or* a
generating rule, never both (FT-A23(v)). **58 numbers issued, 56 spent, 39 and 40
RESERVED.** "Still live" means the frozen object exists in the unified crate at
c00717d1 or a live freeze cites it unchanged; everything else is archive-only at
producer commit `648f93a`. (Measured 2026-09-12 on this machine: a grep over
`walt/walt/src` and `walt/walt/tests` finds freeze citations by number only for 1, 44,
55, 56, 57 and 58; freezes 26 and 37 are live as objects — `strat/hidden_scalar.rs`,
`strat/revealed.rs` — without naming their number.) Cross-checked against
[walt-math-freezes](walt-math-freezes.md) and the declaring rulings in
`walt/CENSUS-RULINGS.md`; that page's header was brought to "58 issued; 56 spent; 39 and
40 reserved" in the 2026-09-13 book rewrite (its "freezes 1–57" clause refers to the
freezes declared in `CENSUS-RULINGS.md`, which is correct — 58 is issued by the register),
see Appendix C row 12.

| # | date | content (one line) | fixed by | still live? |
|---|---|---|---|---|
| 1 | 2026-08-10 | content-addressed class encoding: 128-bit FNV-1a of the signature bytes; identity is a function of the future cone | r3 Q4/Q5.3 | archive; one doc comment in `walt/walt/src/solver/proof_state.rs` cites "the freeze-1 hash family" |
| 2 | 2026-08-10 | per-state canonical move order (increment, classification, successor hash); named `k` in census headers, `increment` in code — code authoritative | r3 Q5.3 | archive |
| 3–6 | 2026-08-11 | yard tree encoding; shape canonical form with a STOP ceiling; suffix cut by interned identity; the open variant | yard v1 / shape v2, registered at P-Q6 | archive |
| 7 | 2026-08-11 | the fiber enumeration order (hidden slots in offset order, k-combinations lexicographic) | P-A18 | **live** — cited unchanged by 44–55 |
| 8 | 2026-08-11 | decimation rule (g, W) for the fiber probe | P-A18 | archive |
| 9–11 | 2026-08-11 | fold weighting; operator and valuation; per-arm key functions | P-A18 | archive |
| 12–14 | 2026-08-11 | intensional exclusion predicates; flag keying; store record format with freeze-set digest | X-A11 (14 first implemented at E-A19) | archive |
| 15–17 | 2026-08-11 | canonical-form key; floor domain and closed-form count; warm-arm coordinate order | E-A19 | archive |
| 18–21 | 2026-08-11 | seat-side form and S-A2 comparison reading; fold maps; interface encoding; landing form | S-A19 | **spent on the parked build**, never reused (R-A22) |
| 22–24 | 2026-08-12 | information-interface encoding; closure discipline (pivot rule); observation-label encoding γ | R-A22 | archive; 23 cited by 55 |
| 25 | 2026-08-12 | decimation constants (7919,12), (104729,6), (1299709,3) for the S6a–S6d track | R-A22 | archive |
| 26 | 2026-08-12 | **the concrete authority**: `ScalarHidden::action_values_dag` dag-v1, `trick_only`, uniform fiber weighting, full-record observation contract, budget 200,000,000, bridge Q_diff = 2·Q_count − grade, least-index tie rule | R-A22; bridge and tie rule ratified at SEP-A3(iii)/SEP-A8 | **live object** (`walt/walt/src/strat/hidden_scalar.rs`); cited by 36–55 |
| 27–31 | 2026-08-12 | vector encoding; dominance and fold order; exact-simplex exposure programme (Bland); caps; policy-counting convention | PG-A14 | archive |
| 32–35 | 2026-08-12 | detector predicates and bitsets (no exhaustion margin exists to freeze); call sites and charging rule; ground-truth classifier; harvest arms | J-A16 | archive |
| 36 | 2026-08-13 | candidate-policy library v1 (no values, no verdicts, cache never authority); **v2** opens transport to the declaration fold | SEP-A4; v2 at EC-A8 | archive (library file archived) |
| 37 | 2026-08-13 | the action-conditioned upper witness U_a = `revealed_summary().q_c`, treatment C, conditions (C1)–(C4) asserted in-run | SEP-A6 | **live object** (`walt/walt/src/strat/revealed.rs`) |
| 38 | reserved 2026-08-13; **filled** 2026-08-14 | the gluing cut v1, scoped: cut language (identifies action variables at one information state; removes no world), validity discharged for the canonical family, cut ordering; clause (d) exhibited as v1.1(d); v2 not opened | DS-A13; FT-A17; SR-A21(ii) | archive |
| 39 | 2026-08-13 | circuit representation and evaluation order — **RESERVED** | DS-A13 | reserved |
| 40 | 2026-08-13 | the reachable-belief family defining W_reach — **RESERVED** | DS-A13, DS-A23 | reserved |
| 41–43 | 2026-08-13 | checkpoint record format and digest; unit identity and assembly order; sequential timing rung selection rule (W = 1, by rule never by result) | DS-A36 | archive |
| 44 | v1 2026-08-13; v2 2026-08-14 | the walk-step unit and budgeted-walk contract (charge-then-descend, no partial fold on exhaustion; binds every walk-based evaluator); v2: P_max = 192,000,000, B = 10^10, g = 15,485,863 | N4-A1; RW-A8; N4-A16(vi) | **live** (11 source citations) |
| 45 | 2026-08-13 | the n = 4 coordinate identity form; corpus hand id is provenance only | N4-A3 | archive; inherited by 47, 54 |
| 46 | 2026-08-13 | the economy arm list, CLOSED: X, T, P1–P4, R (HEURISTIC RE-KEY, NOT A TRANSPORT) | EC-A1 | archive |
| 47 | 2026-08-14 | the trick-1 carrier: 294 drawing coordinates in canonical order plus the 13 corpus hands; cross-check ladder grades 2–5 | T1-A11 | archive; cited by 55 |
| 48 | 2026-08-14 | the lay-down catalogue order and record format; phase-2 search order | LD-A9(iii) | archive |
| 49 | 2026-08-14 | the n4 economy carrier: nine coordinates, all four actions, arms P1–P4, rule argument list (record, legal) | RW-A8 | archive |
| 50 | 2026-08-14, v1.1 same day | the fusion-tax carrier: five negative-margin coordinates enumerated with no generating rule (sort clause struck at FT-A23), binding pairs only, emission cut by content (FT-A24) | FT-A18(vi); FT-A23(iv); FT-A24(viii) | archive |
| 51 | 2026-08-14 | the depth-two carrier: h2 then h9, second frontier counted forced or not | SR-A22(iii) | archive |
| 52 | 2026-08-14, v1 → v1.4 same day | the feature-fee audition carrier (h0 unit 00; h2 units 53, 54), per-action centring, exact minimisation, tie rule; domain clause, screen, census sets, per-cell screen with null control exempt | FF-A6; FF-A15(i); FF-A20(iii); FF-A23(iv); FF-A33(iii) | archive |
| 53 | 2026-08-14 | the fee-correlation diagnostic on the same three units; frozen comparison table transcribed, never re-parsed | FC-A5 | archive |
| 54 | 2026-08-15 | the seed-survey carrier: index(n) = (n · 292,032,399,099,041) mod 472,518,347,558,400, every legal root action a unit, committed-summary/companion split | SS-A4; range closed at SS-A18 | archive |
| 55 | 2026-08-16 | GPU-native trick-1 portable M0/M1 authority `GT1_FREEZE_SET_DESCRIPTOR_V1` and its encodings | GT1-A9 | **live** (`walt-gpu-ref` receipts) |
| 56 | 2026-08-16; v2 2026-08-24 | the binding M2 Metal parity authority (899 bytes, SHA-256 `7bdc5e05…`); v2 re-issued at the unified layout with a cumulative source manifest | GT1-A17; FZ-A1..A6 | **live** (`ci/verify_m2_sources.sh`; standing receipt = old-layout evidence) |
| 57 | 2026-08-17 | the binding M3 perfect-recall-net gate (962 bytes, SHA-256 `e5efe6ce…`); authorizes only the gate, records no M3 result | GT1-A24 | **live contract** |
| 58 | 2026-08-31 | RefineV1: `walt/walt/src/solver/refine.rs` at main 25b40d9 semantically frozen | APS-A9 — **number issued by the wiki register, not by CENSUS-RULINGS.md**, which never names it | **live** |

Two recorded discrepancies stand unresolved by design (freeze 1 vs 2 ordering in the r3
Q5.3 prose; freeze 2's sort-key name) — the code is authoritative. The focal-horizon
program (2026-09-04) issued no freeze; its identity coordinates are declared, not frozen.

## Appendix B — the rulings method

The slice's most durable product is a working method, each rule adopted after something
went wrong. Every later era inherits all of it verbatim.

| discipline | ruling / origin | what it means in practice |
|---|---|---|
| Adjudicate before building | every probe (walt/CENSUS.md onward); DS-A28 append-only protocol | a design document (question, construction, receipts, declared stops, failure criterion) goes to walt-math before code; rulings are binding and append-only; a superseded ruling is marked, never rewritten |
| Both outcomes are results | F7, the NO-RESCUE policy (2026-08-10) | a failure is a counterexample carried back to the mathematics, never a thing to fix, spin or engineer around; a gate NO-GO is filed as a result (S6f) |
| Declare the criterion in advance; report the verdict, not the texture | R-A20 (Gate B), PG-A13/A15 (STOPPED) | a capped coordinate reports no verdict and forbids the global claim |
| Caps exclude, never sample; unmeasured is never zero | factory ledger; FF-A15 (vacuous cells typed unmeasured) | a budget cap removes work from scope and the excluded set travels with the result |
| Grades and labels travel with every verdict | §12.4 label-relativity; S5c-m3 | a verdict without its operator pair and weighting is worthless; never quote above grade |
| Determinism is declared | the freeze register; DS-A29 (every stop a deterministic count, never wall-clock; no clock, RNG or environment value in any decision) | numbered freezes, never reused; freeze-set digest on every stored record; mismatch = corrupt |
| Contended timings are recordable, never quotable, bias direction named | DS-A32, DS-A33 | the sequential W = 1 rung is the only quotable cost instrument |
| The selection fence | FT-A26(iii), SR-A25(iii), P-A21 | a carrier chosen by outcome is not a sample; nothing measured at grade ≤ 4 is quoted for trick 1 or the opening |
| "A cross-check is not a witness" | SR chapter (h9) | agreement between two computations does not manufacture an object a pipeline could not build |
| "A freeze clause states a constant or a generating rule, never both" | FT-A23(v), bought with the freeze-50(a) defect | where both are wanted, the rule is authoritative and the list is an asserted derived check |
| "By construction is not a receipt" | PG-A8; Proposition SR-taut | an identity in a probe's own recomputed quantities cannot fail and is never counted among receipts HELD |
| A receipt names the carrier of its reference value | FT-A28(i), FT-A29 | a frozen table with its provenance line or an in-run recomputation, never "the previous emission"; an obligation-creating clause is not evidence the obligation is still open |
| Independent verification means an independent predicate, not an independent party | S6n | two agents running one grep are one check; a wrong predicate returns exactly the answer being hoped for |
| The house form for instruments | FC chapter | do not grade an instrument — state what follows from a positive reading and what follows from a negative one |
| Every count names its carrier and state set in the same sentence | FF-A18, FC-A10(iv), SS-R9 | a scope derived from an adjective is not a scope |
| Probes are validators, never source | S4.5, S3.5 | the rescued Python suites are frozen regression records; walt reimplements from definitions and pins against them |

## Appendix C — prose-versus-artifact disagreements

Drift between prose and an artifact is a bug and is recorded rather than reconciled by
editing. Rows 1–7 are the numeric disagreements between a prose source and a results
file (the file governs in each); rows 8–13 are staleness or provenance disagreements.
The 2026-09-07 survey counted these differently (twelve open plus one resolved on the S6
page; four on the factory page; two on the freeze register; three new); this is the
complete set known to this page.

| # | where | prose says | artifact says | status |
|---|---|---|---|---|
| 1 | S5a, worldwise-dominated chosen actions | LOG: 12 | `full_walk_2026-08-10_assembled.txt` (CI config): 11; counts are config-relative (11 / 12 / 10) | corrected in place on the factory page |
| 2 | S5c-m3, crossval speed ratios | LOG: "28×–122×" | `h_tree_crossval_2026-08-10.txt`: 125×, 47.6×, 18.6×, 121.7× (re-derived arithmetic) | file governs |
| 3 | S5c-m1, lessons with atom cells | `falsification_2026-08-10.txt`: "12 with surviving atom cells" | `_r2.txt`: "10 with selecting atom cells" (re-pinned pairs excluded) | r2 governs; both files kept |
| 4 | S6b, G-flat receipt rows | LOG: 30 rows | `policy_geometry_2026-08-12.txt`: 24 (12 at grade 1, 12 at grade 2) | file governs |
| 5 | S6g, the corpus corner-gap range | LOG: "4 .. 92/15" | `trick1_draw_2026-08-14.txt`: widest is 19/3 (corpus hand 11) | file governs |
| 6 | S6h, the tightest exact negative | LOG: 8524657/479001600 (h6) | `separation_n4_2026-08-14.txt`: 9557/554400 at h2 = 8257248/479001600, strictly smaller | file governs |
| 7 | S6j, the h9 rule bar | LOG: "within 1202339/8870400" unqualified | `rule_economy_n4_2026-08-14.txt`: that gap is at the two H-optimal actions 41 and 54; the smallest gap anywhere at h9 is 177253/3326400 at the non-binding action 61 | qualifier mandatory |
| 8 | S6k, how many pairs failed | FT-A25(vi) commentary: "ten of twelve" | `fusion_tax_2026-08-14.txt`: one CLOSED, ten tied NOT CLOSED, one untied (h0) NOT CLOSED — eleven of twelve | corrected at FT-A29; the mandatory sentence itself unaffected |
| 9 | S6c, detector cost | the 2026-08-24 wiki (`walt-s6-era.md` lines 283–285, 928–929; `walt.md` line 113; `walt-instruments.md` line 571; `game-of-42.md` line 401 at `c00717d1`): the timing rung is "unrun" / cost "not quotable" | `deadness_rung_2026-08-13.txt` exists: 17 ns/call (384 calls) and 42 ns/call (3,540,143 calls) at W = 1 | prose was stale; LOG and `walt-instruments.md` already carried the rung; the 2026-09-12/13 book pages (`game-of-42.md` §5.2, this page §2.3) cite the rung — `walt-s6-era.md`'s two sentences are the remaining stale text |
| 10 | S6f, the cause of the NO-GO | wiki and LOG: the partition exceeds P_max = 32,000,000 (estimate said 24.8M) | `separation_n4_rung_2026-08-14.txt`: "partition STOPPED — states 0, walk-steps 10,000,000,000, cap_hit true" | consistent: N4-A13(i) rules the printed step count a poison artifact and confirms the stop was the 32,000,000-state cap; the 24,825,150 estimate is struck at N4-A4's superseded note |
| 11 | S6i, (LD-R4) | FT-A16(ii): "(LD-R4) remains owed" | `laydown_2026-08-14.txt`: (LD-R4) HELD, 2,107 receipted | stale ruling text; corrected at FT-A29 |
| 12 | the freeze register's header | `walt-math-freezes.md` at `c00717d1`: "freezes 1–57", "56 issued, 54 spent"; `walt-math-reference.md`: "all 57 issued freezes" | the same page's 2026-08-31 addendum issues 58 | RESOLVED in the 2026-09-13 book rewrite: both pages now read 58 issued, 56 spent, 39/40 reserved (Appendix A agrees) |
| 13 | freeze 1/2 | r3 Q5.3 prose orders them "move order, then encoding"; census-era headers call the sort key `k` | code and X-A7: encoding is 1, move order is 2; code names the key `increment` | recorded, unresolved by design; code authoritative |

Also carried: the S6d "108-decision playbook" was RESOLVED at EC-A12 (use 384); the
S6c LOG provenance ("survived a mid-run kill at 41/45", "byte-diff IDENTICAL") is
LOG-only, the results file being a RESUMED run; S6b's authority receipt and 4,096 cap
are LOG-only provenance; the S6d results-file header cites SEP-A1..A18 while its body
cites SEP-A19(b), ruled at build time.

## Appendix D — reproducing an artifact

Every file under `walt/probes/factory-results/` carries in its header the tier, the
rulings, the freeze-set digest and the regenerate command. The producers (`walt-factory`,
24 examples, and `walt-skeleton`) were deleted from the tree on 2026-08-24 (commits
`ad355e9`, `fa3fe74`; the fold `d1499d4`); the 65 tracked summaries were relocated to
`walt/probes/factory-results/` and the untracked bulk (8.3 GB of results, of which the
second-rung frontier body is 8.2 GiB; 514 MB of stores including `endgame_l2.store` at
499 MB) lives at `~/data/texas-42/walt-factory-archive-2026-08-24/` with a
`manifest.sha256`, and a private HF dataset `jasonyandell/texas-42-walt-archive` whose
upload is tracked by [[hf-archive-upload]] (`kanban/doing`). The protocol
(`walt/ARCHIVE.md`):

```
git switch --detach 648f93a          # the last commit with every producer present
cargo run --release -p walt-factory --example <producer> [args]
# census_* ← census_run (r2/r3/t5/prune/yard/yard2/a1); deadness_* ← deadness_probe;
# economy_* ← economy_run{,_r2}/economy_seed; falsification_* ← falsification_run;
# fc_correlation_* ← fc_correlation; feature_fee* ← feature_fee{,_v11};
# fiber_probe_*/fiber_refine_*/endgame_* ← fiber_probe (h/refine/endgame/floor);
# fusion_tax* ← fusion_tax; label_transfer_* ← label_transfer_run; laydown_* ← laydown_probe;
# lesson_basins_* ← lesson_run; policy_geometry_* ← policy_geometry;
# predictive_rank_* ← predictive_rank; rule_economy_n4_* ← rule_economy_n4;
# second_rung* ← second_rung; seed_survey_* ← seed_survey;
# separation_* ← separation_probe (-- n4 for the pass); trick1_draw_* ← trick1_draw;
# full_walk_* ← the walk_corpus bin.
```

Verify against the manifest digest before treating a regeneration as the same
computation: frozen seeds make byte-identity the expected outcome, and **a mismatch is a
finding, not a shrug**. Run costs (provenance, never a dividend): seconds to minutes for
most files; the n = 4 solves 10^4–10^5 ms per unit; the trick-five census 2.65M
situations; the crossval tree walk 62 minutes; the seed survey a night at 10 workers.
Regeneration promotes nothing — the artifacts keep exactly the tier they had. Not verified
in this pass: whether the 8.2 GiB frontier body and the 4.59 GB seed-survey companion are
still present at `~/data` (outside the worktree).

## Where it went next

The program that produced this page ended on 2026-08-16 with grade 4 exhausted and a
"longer ladder" as the named next step. On 2026-08-17 Jason pivoted walt to building the
seat that plays ([walt-program](walt-program.md) reset 7; [walt-seat-play](walt-seat-play.md)),
with pmake as the objective; the root interval, certified regret and the focal-horizon
hierarchy are this program's mathematical descendants, and the counted-belief program's
exact branch table at the opening root (399,072,960 worlds through 116,280 acting-seat
hands) is the answer to the trick-1 obligation that this page leaves standing. That
lineage, with the obligations re-stated as superseded where they are, is on
[walt-decision-sparse](walt-decision-sparse.md#lineage-since-2026-08-24).
