[Home](Home.md) · owns: the map of walt's mathematical corpus organized by idea — every named object, the parent document and ruling that fixed it, the measurement that last moved it, and the page that owns the detail · Sources: `walt/CENSUS-RULINGS.md` (the adjudication record, 35 ruling families, last section "The focal-horizon adjudication (2026-09-04)", 14,358 lines at `c00717d1`), `walt/math/decision_sparse_exact_solving_v0.1_errata.md` (the maintained decision-sparse mathematics), the received parents and intake companions under `walt/math/` (indexed on [walt-math-intakes](walt-math-intakes.md)), `walt/MAP.md`, `walt/FACTOR-BELIEF.md`, the briefs under `walt/briefs/`, the probe records under `walt/probes/`, the gates under `walt/walt/tests/`, and `experiments/partnership/packet/` (three received notes with no intake).

# walt mathematics — the reference map, by idea

Related: [walt hub](walt.md) · [walt-program](walt-program.md) ·
[received artifacts and intakes](walt-math-intakes.md) ·
[structure and transport](walt-math-structure-transport.md) ·
[information geometry](walt-math-information-geometry.md) ·
[decision-deadness](walt-math-deadness.md) ·
[decision-sparse witnesses](walt-math-decision-sparse.md) ·
[the freeze register](walt-math-freezes.md) ·
[open questions](walt-math-open-questions.md) ·
[negative results](walt-negative-results.md) ·
[vocabulary](vocabulary.md) · [timeline](timeline.md).

> **Epistemic tier: EXPLORATORY — below every tier on
> [Home](Home.md#evidentiary-tiers--never-promoted-never-blurred).** Every
> statement indexed on this page is exploratory, without exception. A proved
> lemma at the exploratory tier is still exploratory: it is proved *relative
> to* walt's own declared basis (v0.4/v0.5/v0.6, themselves exploratory) and
> to walt's *implementation* of the rules (T1-A12), and it may not be quoted
> in a brief, a dispatch, [FINDINGS](FINDINGS.md), or any claim-tier page. The
> strongest evidence any walt number carries is a Rust gate under
> `walt/walt/tests/` or a byte-checked probe record under `walt/probes/`;
> every Python verifier under `walt/math/` is session evidence, never a
> receipt (TRUST-01). This page is a map, not a promotion. Repository state
> is as of 2026-09-07 (`c00717d1`); fresh checks made for this page are
> labelled "measured 2026-09-12 on this machine".

## The naming rules this page obeys

Vocabulary is load-bearing in this corpus, and four rules govern every line
below.

- **"Sandwich" is never an object name.** Two rulings retired it (CBS-A3,
  2026-08-30; FH-A2, 2026-09-04) because it collides with three adjudicated
  names: the decision-sparse **Theorem E6.3 "value sandwich"** (errata §6),
  the **REFUTED** T1-A "trick-1 bounded sandwich" (T1-A1..A12), and SP-A7's
  sandwich discipline. The objects are the **root interval** and **survivor
  set** (CBS-A3) and the **focal-horizon hierarchy**, **focal-horizon interval
  `[L_k(B), U_k(B)]`**, **action interval `[L_{a,k}, U_{a,k}]`**, **bar
  `B_k`**, **survivor set `S_k`** and **focal depth `h_f`** (FH-A2). The
  adjudicated name of Theorem E6.3 and the parents' file titles
  (`counted_belief_sandwich_v0.1.md`, `focal_horizon_sandwich_v0.1.md`) may be
  quoted; nothing else may be called a sandwich.
- **"Certificate" is not used** (DS-A1, D3). The permitted uses are the
  explanatory phrase *necessary outer profile*, bracketed quotation of a
  source that uses the word, and walt's own term of art **certified regret**
  `Γ = U* − B_exec` (APS §31; FH-A2 records explicitly that it does not
  collide with the ban). Witnesses are exhibited; receipts are regenerated.
- **Typed distinctions are never blurred**: support ≠ belief; feasible ≠
  reachable (R-A2, P-A1); possible ≠ probable; estimate ≠ receipt;
  exact-for-the-frozen-set ≠ exact root (L2 §15.3, O18); decision-dead ≠
  decided ≠ laydown (Chapter 4); a walt "level 2" is a best response to a
  named σ1, never an equilibrium (O36, L2-A7).
- **The objective is pmake** — the make-the-bid indicator, ruled 2026-08-17;
  a trick-difference or point proxy is never the target.

## How to read this page

Twelve chapters, one idea each. Every chapter gives the plain statement, then
the precise objects with their ruling IDs and parent documents, then the
measurement that last moved the idea (record path and the gate that pins it,
or "probe record, not gate-pinned"), then the page that owns the detail. Long
derivations live on the owning pages and in the parents; this page does not
restate them. After the twelve chapters come the appendices the previous
edition of this page carried and which remain in force: the governing
documents, the complete object index for the pre-2026-08-24 lineages, the
ruling-family table, the supersession chain, the standing disciplines, and the
run-name decoder — followed by the dated historical addenda, kept verbatim.

The two documents a reader of this page will open most: `walt/CENSUS-RULINGS.md`
(locate a ruling by ID and section heading, never by line number — the file
is append-only and line numbers drift) and
[walt-math-intakes](walt-math-intakes.md) (what was received, when, with what
hash, and what came of it).

---

## Chapter 1 — The seat's problem

**Plain statement.** A seat at the table knows its own seven tiles, the bid,
and everything played so far. It does not know the other 21 tiles. The
question walt asks is: *given only what this seat can see, which play
maximises the chance of making the bid?* — against a declared model of how
the other three seats play.

**The precise object.** A public belief state `B` (the information state:
own hand, full public record, bid and declaration); the *fiber* of hidden
worlds consistent with it; a declared, deterministic *field* σ for the other
three seats; the class of *lawful* (information-consistent) focal policies
Π; and

`Q(B) = max_{π ∈ Π} E_{ω ~ β(B)} [ pmake(π, σ, ω) ]`,

where the maximum is taken **after** worlds sharing one public history are
merged. Reversing that order — choosing per world — is *strategy fusion*, the
thing the game punishes and the fence every walt parent restates (O1/O34 in
`walt/SCENARIO-PLAYER.md`; CBS §11; APS §23; MB-I4; SC §26; FH §6).

| Object | One line | Ruling / parent | Full statement |
|---|---|---|---|
| The objective **pmake** | P(make the bid) is the target; trick differential is a proxy that admits no Boolean pruning. | Jason's ruling 2026-08-17; `WALT-MATH-RULING-2026-08-17-…` | `walt/math/WALT-MATH-RULING-2026-08-17-pmake-and-the-walk-to-trick-1.md` (advisory, no ruling family) |
| **P1–P4** | Decided cutoffs SOUND (banked_T1 ≥ 30 ⇒ 1; banked_T0 > 12 ⇒ 0; totality T1 + T0 = 42 at every terminal); viewer early exit SOUND; pmake key reduction SOUND (unsound for trick differential); gcd-normalised projective posteriors SOUND-WITH-CAVEAT (fail-closed u128 overflow). | same, §Verdicts | same file. Its basis, `ladder.rs` in the `walt-m3-probe` crate at `171cd22`, no longer exists on main (the crate was deleted in the 2026-08-24 unification); the ruling is read historically. |
| **The path-dependence counterexample** | The exact posterior is a function of the full ordered record but **not** of the reduced boundary key (played mask, leader, banked): same reduced key, odds 1:1 on one trick order vs 3:4 on the other. | same, "Bonus" | same file |
| **The field σ0** | The level-0 banked-correct modeled mind for the other three seats: deterministic, reads the bid and the **full** public record; one read ≈ µs; every recursion's cost is its read count. Registered as `F₀ = BR(D) = σ0` where `D = FieldModel::Dice` (MB-A3, corrected at intake — the first reading "Dice = σ0" was wrong and retracted); `F₁ = BR(F₀)` = level-1 walt; `F₂ = BR(F₁)` unbuilt. | MB-A3 | `walt/walt/src/solver/field.rs`; `walt/CENSUS-RULINGS.md` § "The model-belief base-player adjudication" |
| **Lawful policy / information consistency** | A focal policy reads only the focal information state; the §2 spec objects (public record R, information state I_s, solver key κ(R), PiKey, level-k minds, Dice tickertape) and the load-bearing Remark 2.5 (banked is NOT a function of (played, leader, plays) — the PiKey defect's content). | `walt/SCENARIO-PLAYER.md` §§2–6; Def 3.4 cache purity | spec; obligations O1 (no-strategy-fusion audit), O2 (key sufficiency, Lemma 2.4, unproved), O4 (posterior semantics, Lemma 5.2, "the load-bearing one") in §10 |
| **The strategy-fusion fence** | Pointwise (per-world) optimisation is never lawful coverage; a per-world optimum is at most an *upper* bound. | O34; CBS-A4 (§11 non-theorem "stays load-bearing"); SC-A5 (§26: an optimizer disagreement is not a cut) | `walt/CENSUS-RULINGS.md` CBS-A4, SC-A5 |
| **Jason's frame** | "42 is 2 recursions running in opposite directions": late tricks enumerable with no model uncertainty (backward exactness); the model-belief physics lives earlier (forward play). UP0 made it a field of the record (`Recursion::direction`). | quoted framing, 2026-09-02 | `walt/briefs/BRIEF-MB1.md`; `walt/walt/src/solver/unified.rs` module doc |

**The measurement that last moved it.** The unified player (UP0/UP1a,
2026-09-02/03) plays both recursions as one five-tier cascade — decided
arithmetic → endgame exact → middlegame mixture → certified regret → σ0
fallback — with provenance on every decision; on the lean rung 99.4% of wall
was carrying a posterior nothing read until the lazy carry (2,105,672 µs →
0 µs over 72 decisions; `walt/probes/factor_belief/unified_run1.txt`,
`unified_run2.txt`; gates `walt/walt/tests/solver_unified.rs`,
`solver_unified_carry.rs` UC1–UC5). The live default player is untouched by
all of it (CE-A7/§20.16, restated at CBS-A9, APS-A9, FH-A10).

**Owning pages.** [walt-seat-play](walt-seat-play.md) (how the seat decides),
[walt-focal-horizon-era](walt-focal-horizon-era.md) (the unified player),
[walt-architecture](walt-architecture.md) (the code).

---

## Chapter 2 — Counting the hidden world

**Plain statement.** At the first trick there are 399,072,960 ways the other
21 tiles can be dealt. walt never lists them. It counts them — exactly, in
integers — and it counts *hands*, because only the hand of the seat about to
act matters to that seat's model.

| Object | One line | Ruling / parent | Full statement |
|---|---|---|---|
| **Support normal form and the void-free capacity fiber** | The declared cost domain of every census: members are FEASIBLE and never reachable (R-A2, P-A1); `FiberDp` in `kernel/fiber.rs` is the uniform-root exact-cover oracle. | R-A2, P-A1; CBS-A6 (attribution) | [support-fiber](support-fiber.md) for the corpus-tier object; `walt/walt/src/kernel/fiber.rs` |
| **The opening arithmetic** | C(21,7) · C(14,7) = 116,280 · 3,432 = 399,072,960; one hidden seat has 116,280 possible root hands, each with exactly 3,432 compatible completions. | CBS §22; re-derived at CBS-A1 | gates `walt/walt/tests/solver_factor_belief.rs::opening_root_contraction_without_worlds`, `::opening_root_level0_classification_is_once_per_hand` |
| **Theorem 20.1 — seat-factor posterior closure** | Under a seat-local field, conditioning on an observed hidden action multiplies **only** the acting seat's factor by its action likelihood; the posterior stays a product of seat factors coupled only by disjoint cover. The parent's genuinely new mathematics. Binding boundary: any cross-seat coupling voids the closure until represented as explicit factors. | CBS-A6 | `walt/math/counted_belief_sandwich_v0.1.md` §20; gate `solver_factor_belief.rs::condition_recovers_each_branch_mass` |
| **Theorem 23.1 — the factorised fixed-policy recursion** | `V = M/Z` in integers: `viewer_success_mass` computes the exact success mass of one lawful policy without materialising a world. A record-consistency law was discovered at depth (hands inconsistent with the public record are provably zero-mass and are dropped). | CBS-A1 (step-checked); Slice D | `solver/factor_belief.rs`; gates `solver_factor_recursion.rs` (5) |
| **Theorem 9.1 — cylinder partition; the §12 grammar** | Policy cylinders partition the class; a grammar's success mass is `max` over a declared source set, exact and lawful; coverage is only ever a *residual* bound. | CBS-A4 | `counted_belief_sandwich_v0.1.md` §9, §12; gates `solver_grammar.rs` (8), `solver_factor_response.rs` (4) |
| **Theorem 30.1 — refinement safety (consequence CEGAR)** | Hand classes refine by witness pairs; nested intervals never widen. | CBS-A1; Slice F | gates `solver_factor_consequence.rs` (4) |
| **The cross-history cache negative** | Within one history reuse is total (200 ns/query, ×230); across histories it is **exactly zero** because the full §43 identity key carries the public history. Sharing needs a proven state reduction, never a looser key (the PiKey lesson). | CBS-A6 binding | `walt/probes/factor_belief/cache_run1.txt`; gate `solver_factor_belief.rs::the_full_identity_key_shares_nothing_across_candidates_or_roots` |

**The measurement that last moved it.** The opening root's exact one-ply
branch table over 399,072,960 worlds in **8.7 ms** under the trivial field
and **5.36 s** under σ0 — 116,280 hands classified once at ~45 µs each, so
classification is 99% of the cold pass and counting is milliseconds
(`walt/probes/factor_belief/run1.txt` §C, `opening_level0_run1.txt`,
`c2_run1.txt`; hand count, once-per-hand classification and conservation
gate-pinned in `solver_factor_belief.rs`, timings probe-only). The counted
representation then carried the trick-3 root h8-t3 (Z = 59,976) to an exact
value — Chapter 6 — where the explicit-world ladder of 2026-08-17 had died
at ~600 s and 300M nodes (`WALT-MATH-QUESTION-2026-08-17 §Q2`). Slice F's
honest negative: mass concentrates (805‰ action-exact at 3 hands per class)
but driving the residual to zero fragments the opening root into 116,280
singleton classes under σ0 (`cegar_run1.txt` §C; Theorem 30.1 laws gated,
class counts probe-only).

**Owning pages.** [walt-counted-belief-era](walt-counted-belief-era.md)
(Slices A–G); [information geometry](walt-math-information-geometry.md) for
why this is *representation*, not compression of `dim V^val` (Corollary E5.2's
"factorised representation" escape hatch, which Theorem 20.1 took).

---

## Chapter 3 — What does not compress

**Plain statement.** Every attempt to make the top of the game smaller by
symmetry or by algebra failed at its declared scope, and each failure was
filed as a result (F7: both outcomes of every experiment are results). At the
first play nothing is dead and nothing is inert; compression in this project
is bought with deadness (S-A21).

| Object | One line | Ruling | Full statement |
|---|---|---|---|
| **Corollary S-rigid** | The first-play seat transport group is **trivial**: the hand form *is* the hand, so COUNT 1 = C(28,7) = 1,184,040 is a theorem missing the 10⁵ bar by 29601/2500. Fenced by S-A20: answers the bar only for the finest structural equivalence. | S-A (2026-08-11) | `CENSUS-RULINGS.md` § "Seat-census rulings"; [structure and transport](walt-math-structure-transport.md) |
| **Lemma S-fold / Corollary S-fold-val** | The seven pip declarations fold exactly 7:1; value transport along the fold is reading-independent. | S-A; EC-A4 | same section; § "Corollary S-fold-val" |
| **Lemma S-det** | The landing state is a function of (δ, H, trick-1 record); the raw record space is the alphabet — the trick-2 boundary is provably ≤ 7 × 21 × 20 × 19 = 55,860 records (pmake ruling §Q2). | S-A; 2026-08-17 ruling | same; `WALT-MATH-RULING-2026-08-17` §Q2 |
| **Lemma R(c)** | Any closure seeded with a nonzero constant has predictive dimension exactly \|X\| where a complete continuation record determines the latent point — as it does here. S6a measured grade-3 `dim V^val ∈ {1461, 1492, 1680}` against \|X\| = 1680: Gate B **REFUTED**. | R-A1..A24 (2026-08-12) | § "Predictive-rank probe rulings"; `walt/probes/factory-results/predictive_rank_2026-08-12.txt` |
| **Proposition E5 / Lemma E5.0 / Corollaries E5.1–E5.2** | The Scheme-mass closure is degenerate (atoms are singletons); the binding negative is "atom-mass **linear** filtering is noncompressive on this carrier"; E5.2 leaves open circuits, factorised representations, BDDs, moment compilation. | DS-A5, DS-A20, DS-A21 | errata §5; [decision-sparse witnesses](walt-math-decision-sparse.md) |
| **Lemma G(6)** | A run that prunes cannot report `N_vec`; narrowed to the two pruning rules in use (DS-A26). | PG-A; DS-A26 | § "Policy-geometry probe rulings"; errata §6 Theorem E6.2(c) |
| **The almost-trivial ECL group on hand 8** | The concrete §12.6A instance (π fixing played tiles and the focal hand, θ transporting contexts, preserving trick-key order and count) survives on the hand-8 t = 4 pool as a provably almost-trivial group; predicted compression < 2×. Under a uniform-random field the posterior is the **minimal sufficient statistic** and no coarser belief relation should be hunted (139k distinct posteriors at t = 4, ≥ 4.69M at t = 3, incomplete). | 2026-08-17 ruling §Q2 (advisory) | `WALT-MATH-RULING-2026-08-17` §Q2; parent `walt/math/equivariant_lumpability_v0.5.md` |
| **The specimens' tie mechanism — UNIDENTIFIED** | 276 / 1,239 / 1,773 classified ties at the trumpless-junk grade-3 family, zero detector hits; deliberately not filled. | J-A8 | [open questions](walt-math-open-questions.md) item 1 |

**The measurement that last moved it.** Nothing since 2026-08-17 has changed
any of these; the counted-belief factorisation (Chapter 2) is the escape
hatch E5.2 named, taken as representation. The scenario-era conjecture that
belief states grow to 10⁷–10⁹ at mid boundaries (one measured 34× per-trick
ratio) now has one hard point against it in the other direction: h8-t3 was
solved exactly by the factorised recursion (Chapter 6) without any world
list.

**Owning pages.** [walt-negative-results](walt-negative-results.md) (the
refutations as first-class findings); [structure and
transport](walt-math-structure-transport.md); [information
geometry](walt-math-information-geometry.md); [walt-pre-pivot-results](walt-pre-pivot-results.md).

---

## Chapter 4 — Where the seat's choice does not matter

**Plain statement.** Much of a hand is forced or indifferent. Three kinds of
"does not matter" are kept apart because they are proved differently:
**decision-dead** (every choice has the same value — `N_vec = 1`),
**decided** (the outcome indicator is already fixed by banked points — P1),
and **laydown** (every legal continuation of every seat makes — a universal
quantifier, APS-A5). A node can be decided without being dead, and dead
without being a laydown.

| Object | One line | Ruling | Full statement |
|---|---|---|---|
| **The three-property typing** | forced ⊂ decision-dead ⊂ dominant, both inclusions strict on S6b; never fused. Cardinality ladder `N_pol ≥ N_vec ≥ N_par ≥ W_all ≥ W_reach ≥ 1`. | J-A; PG-A | [decision-deadness](walt-math-deadness.md); [information geometry](walt-math-information-geometry.md) § "The cardinality ladder" |
| **Lemma J, Lemma J(c′)** | Non-interference ⇒ decision-dead count-free; with the guard `H ∩ COUNT = ∅`, also under trick-plus-count; (c′) the sharp valuation clause (any schedule constant on H). | J-A; DS-A24 | § "Decision-deadness probe rulings"; errata §8.5(e) |
| **Propositions J-0, J-1, J-win** | D0: three bitset tests, exactly sound, no exhaustion margin; D1-sym: the transposition detector, the workhorse; D1-win: count-free only, **never fired**. The refused fourth detector (J-A8). | J-A | same section |
| **Lemma E8** | The exact valuation scope of J-0/J-1: gauge-stable constancy on the exchanged tiles. | DS-A24 | errata §8.5 |
| **384 versus 108** | The idx = 0 lead-00 extraction has 50,712 states, 384 with a genuine two-tile choice (receipt-backed, `separation_2026-08-13.txt`), 276 tie under one deviation; 108 is a derived difference present in no receipt — name the entry by 384. | EC-A12 | [decision-deadness](walt-math-deadness.md) § "384 versus 108" |
| **P1 decided cutoffs** | `banked_T1 ≥ 30 ⇒ 1`, `banked_T0 > 12 ⇒ 0`, total by T1 + T0 = 42; the allowance automaton `a = 12 − banked_T0 ∈ {0..12} ∪ {busted}` replaces the tally pair under pmake. | 2026-08-17 ruling P1, §Q2 close | Chapter 1; tier (a) of the unified player |
| **The typed laydown hierarchy** | PolicyCertainMake (∀ω, one π one σ) / AdversarialPolicyMake (∀ω ∀σ) / ForcedMake (∃π ∀ω ∀σ) / Laydown (∀ω ∀π ∀σ). Bare "laydown" is reserved for the universal type; a model-relative pmake = 1 is never called a laydown; no sampled route constructs any of the four. | APS-A5 | `anytime_proof_state_score_v0.1.md` §16; gates `solver_laydown.rs` (4); `laydownreport_run1.txt` — the boss-chain control is a TRUE Laydown in 1,492,276 walk nodes, h10-t6 a real receipt-root Laydown (forcing witness 2-2) |
| **God-tightness and the doom census** | A node is God-tight when the lawful value meets the world-revealed upper (SC Theorem 7.1: iff the saveable worlds have a nonempty common intersection); the doom census counts worlds no policy can save (∀-fail) as deterministic uppers `(Z − M_doom)/Z`. | SC-A1; doom census 2026-09-01 | `salvation_complex_v0.1.md` §7; gates `solver_doom.rs` (8); `doomreport_run1.txt`: 809–1000‰ of per-world doom recovered on enumerable receipt roots (and 0 of 1 at h8-t5 5-3, the per-seat relaxation's price); an honest **zero** at the opening root |
| **Exact indifference at pmake = 1** | When every play makes, P(make) has no gradient and the play falls to `TieRule::LowestTileIndex` plus `best_of`'s keep-the-incumbent reduce — a deterministic bias against high-index count tiles (the 6-4 is index 25 of 0..27). A design question, not a feature. | 2026-09-05 readout (source reading, not executed) | `walt/briefs/MORNING-2026-09-05.md` items 1–4; [walt-gran-anchors](walt-gran-anchors.md); [open questions](walt-math-open-questions.md) item 16 |

**The measurement that last moved it.** G2 — the made Gran hand — is exactly
locked from trick 3 at the walt seat: all 280 deals at trick 4 make whatever
Gran plays, the four tied tiles are the screenshot panel's four, and level 1
gives the identical tie set (`walt/briefs/MORNING-2026-09-05.md` item 1;
record `walt/probes/gran/level2_g2.txt` on branch `walt-g1-l2`, 8 commits not
in main at `c00717d1`; probe record, not gate-pinned). The doom-census
correction of 2026-09-03 travels with the census: a zero doom census moves
only `d_phys` and says nothing about the `d_info`/`d_policy` split, so the
opening root's remaining 267‰ is **UNKNOWN in its split**
(`walt/DISCREPANCIES.md`, first entry under "Reconciled"; SC §8–§9, SC-A4).

**Owning pages.** [decision-deadness](walt-math-deadness.md);
[walt-counted-belief-era](walt-counted-belief-era.md) (laydowns, doom);
[walt-gran-anchors](walt-gran-anchors.md) (the lock).

---
