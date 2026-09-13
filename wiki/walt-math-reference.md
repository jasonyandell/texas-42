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
> labelled "measured 2026-09-12 on this machine" (Chapters 1–4) or "measured
> 2026-09-13 on this machine" (Chapters 5–12 and the appendices, completed
> 2026-09-13 after the first pass was interrupted at the end of Chapter 4).

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

## Chapter 5 — The one object: a lawful policy below, a relaxation above

**Plain statement.** Every bound walt states about a play is an interval. The
lower end is what one honest, information-consistent policy actually earns,
computed exactly; the upper end is what a relaxed player earns — one who is
told the hidden deal, or who is allowed to choose per world, or whose worlds
have been sampled. An action whose lower end beats every rival's upper end is
*settled*; the actions whose upper end still clears the best lower end are the
*survivors*. Nothing in walt ever certifies more than this, and the object has
had three names as it grew.

**The precise object.** For a root action `a` at public belief node `B`,
`[L_a, U_a]` with `L_a = V(π)` for one lawful policy `π` (never inverted into
an upper: Non-theorem E4′) and `U_a ≥ Q_a` for a declared relaxation;
`B = max_a L_a` the bar; `S = {a : U_a ≥ B}` the survivor set; Γ = U* − B_exec
the certified regret of the *materialised* policy.

| Object | One line | Ruling / parent | Full statement |
|---|---|---|---|
| **Lemma E4, Lemma E3, Non-theorem E4′** | A primal witness is valid (one lawful policy, exactly priced); an action-conditioned upper witness is valid under the four attached conditions; inverting a lower into an upper is the failure mode, named. | DS-A; SEP-A | errata §4.1, §3.2, §4.2; [decision-sparse witnesses](walt-math-decision-sparse.md) |
| **Theorem E6.3 — "value sandwich"** (the adjudicated name, quotable as such) | `L_a(β) ≤ Q^H_B(a) ≤ U_a(β)` when both ends use the same α-map and the same belief over the same world set with no decimation inside either bound (C2); equal endpoints trap the middle. | DS-A; errata §6 | `decision_sparse_exact_solving_v0.1_errata.md` § "Theorem E6.3" |
| **Theorem E6.4 — root-action separation** | `L_{a⋆} ≥ U_a` for every rival certifies a *member* of the optimal set; strict everywhere certifies the set. The caveat is in the statement. | DS-A; SEP-A | errata § "Theorem E6.4" |
| **Corollary E4.1 — the primal ceiling** | The separation probe's primal witness is an H-optimal policy re-priced; the economy claim's primal half is what Experiment E tests. Owed as errata §4.3, unfiled (Appendix G). | SEP-A2, EC-A13 | `CENSUS-RULINGS.md` § "Experiment E adjudication"; [open questions](walt-math-open-questions.md) item 5 |
| **Freezes 36 and 37** | The separation probe's identity (36, v2 at EC-A8 admits the declaration fold) and the revealed upper `revealed_summary().q_c[a]` (37). | SEP-A4/A6, EC-A8 | [the freeze register](walt-math-freezes.md) § "Freezes 36, 37 and 38 in full" |
| **CBS Theorem 2.1 — root interval and survivor set** | The same object over pmake with exact masses: a δ-valid empirical-max upper (Theorem 5.1 = TRIPLE-A2's M1/M2 restated over Π_a) over a frozen-policy lower; exclusion is permanent; seven result types extend the CE ladder; `HeuristicFallback` is never serialised as a settled winner. **"Sandwich" retired as an object name** (CBS-A3). | CBS-A1..A3 | `counted_belief_sandwich_v0.1.md` §2, §5; gates `solver_root_interval.rs`; record `walt/probes/root_interval/run1.txt` |
| **APS — proof bar, executable bar, certified regret** | `B_exec ≤ B_proof ≤ Q*`; **Γ = U* − B_exec** with `0 ≤ Q* − V(π̂) ≤ Γ` on the joint-validity event, monotone under refinement; Γ ≤ ε certifies ε-optimality *under the declared field and belief*. Grammar and full-response optima are proof-bar-only until an argmax policy is extracted and re-priced (§30). | APS-A6, APS-A7 | `anytime_proof_state_score_v0.1.md` §31–§33; gates `solver_proof_regret.rs` (5), `solver_extraction.rs` (6) |
| **The §49 proof state** | An append-only, identity-scoped fact store (Bound / Profile / Envelope / Cover) whose closure — bars, survivors, exclusions, U*, Γ, result — is a pure derived view; idempotent and insertion-order-independent; producers are an open registry. RefineV1 (`refine.rs`) reproduced as the frozen oracle. | APS-A9; freeze 58 | `solver/proof_state.rs`; gates `solver_proof_state.rs` (6) |
| **FH action interval `[L_{a,k}, U_{a,k}]`, bar `B_k`, survivor set `S_k`, focal depth `h_f`** | The object indexed by `k` focal decisions made exact: lower = one lawful tail evaluated exactly, upper = the world-revealed continuation `G`, both collapsing to `Q_a` at `k ≥ h_f`; `Γ_k` non-increasing under FH-int's intersection discipline; **FH-tie**: when every survivor has collapsed the survivors at the bar are exactly the optimal set. Names fixed by FH-A2. | FH-A1..A11; Theorems 1–6 | `focal_horizon_sandwich_v0.1.md`; intake companion P0–P14; gates `solver_focal_horizon.rs`, `solver_focal_ladder.rs` (10), `solver_focal_anchors.rs` (4) |

**The measurement that last moved it.** The object's first live output at
the top of the game is the Phase 8 opening-root verdict (2026-09-01): at
h0-t1 (Z = 399,072,960, contract 30 on threes, seven legal leads) the ladder
p = 16/64/256/512 lifts the executable bar 0 → 407 → 594 → **732‰** while Γ
falls 1000 → 592 → 405 → **267‰**, the recommendation migrating 0-0 → 2-1 →
**6-5**; the sampled tier plateaus at p = 512; verdict **honest UNRESOLVED at
ε = 1/4** — "play 6-5, floor 732‰, at most 267‰ unclaimed"
(`walt/probes/factor_belief/openingreport_run1.txt`; gates `solver_opening.rs`
(5) incl. the §67.5 resume ≡ uninterrupted law; the per-stop numbers are probe
record). Where the object closes: argmax extraction took h3-t4's Γ from 83‰ to
**0‰ exactly** and switched the recommendation 4-4 → 3-1 (`extractreport_run1.txt`;
gate `the_extraction_producer_collapses_certified_regret_at_settled_roots`);
h5-t6 certifies Γ = 0 at 444‰ — optimality certainty is not make certainty
(`proofreport_run1.txt`). And the last renaming: FH3's 33 coordinates settle
every live trick-4 root by k ≤ 2 (Chapter 6).

**Owning pages.** [decision-sparse witnesses](walt-math-decision-sparse.md)
(E-series); [walt-counted-belief-era](walt-counted-belief-era.md) (root
intervals, proof state, certified regret, the opening verdict);
[walt-focal-horizon-era](walt-focal-horizon-era.md) (the hierarchy).

---

## Chapter 6 — Relaxing information: cuts, taxes, complexes, horizons

**Plain statement.** The upper end of the interval is bought by telling the
relaxed player something the real seat cannot know. How much that help is
worth — the *price* of not knowing — is the quantity this chapter's
mathematics names, bounds and, in the end, measures. The finding that
changed the program's direction: once one focal decision is made exact, that
price is a few per mille and the remaining width is the *tail's* fault.

| Object | One line | Ruling / parent | Full statement |
|---|---|---|---|
| **Theorem E6.5 and freeze 38 — the cut language** | Finite adaptive gluing; a *cut* is a declared partition of the latent world set of one focal information state — an identification of action variables that removes no world and asserts nothing about reachability; the reveal-delay ladder `C^(k)` is the canonical family. Freeze 38 **FILLED** at FT-A17 (2026-08-14). | DS-A; FT-A17; SR-A21 (v1.1(d)) | errata § "Theorem E6.5"; `CENSUS-RULINGS.md` FT-A17 |
| **The FT/SR ladder** | Lemma FT-arrive, Lemma FT-trunc + Corollary FT-grade4, Propositions FT-flat / FT-tie, Lemma FT-post, Corollary FT-conv, Lemma FT-mix; one rung deeper Lemma SR-coord, SR-forced, Propositions SR-sep / SR-post / SR-degen (grade 4 has exactly two rungs) / SR-taut / SR-loc, Corollary SR-conv. Owed to errata §9, unfiled. | FT-A1..A29, SR-A1..A37 (x:016, x:017) | `CENSUS-RULINGS.md` §§ "The fusion tax", "The second rung"; [walt-decision-sparse](walt-decision-sparse.md) |
| **The salvation complex** | `1 − Q` is the minimum belief-mass transversal of the salvation-conflict hypergraph (Theorem 12.1); doom is its singleton cuts, information price its higher-order cuts (§13: `1 − Q = β(D) + τ(H_{≥2})`); any verified conflict family gives an admissible upper (§14); rational packing is a cheap dual (§15). Two fences: an optimizer disagreement is not a cut (§26) and G is neither sub- nor supermodular (§23) — zero one-glue gain proves nothing. No verifier shipped: the gates are the checks. | SC-A1, SC-A2, SC-A5 | `salvation_complex_v0.1.md`; gates `solver_godgap.rs`, `solver_doom.rs` |
| **The §8 decomposition** | `1 − V(ρ) = d_phys + d_info + d_policy(ρ)`; a zero doom census moves only `d_phys` and says nothing about the other two — the 2026-09-03 correction of the doom paragraph. `UnknownGodGap` is a distinct result type. | SC-A4; `walt/DISCREPANCIES.md` | `solver/godgap.rs`; U0-REPORT |
| **The fusion horizon — an empirical object** | The fusion-free-suffix hypothesis (§37–38) is a census target, never a theorem until adversarial counterexample search. Measured: **trick 5** at uniform receipt roots (14 t5/t6 coordinates God-tight; 12 trick-4 coordinates carry Φ = d_info 6–22‰ with d_policy = 0), and **not fusion-free inside a trick-4 solve** (positive-price trick-5 frontier nodes 9–31%, 13–14‰ mass-weighted at bid 30, up to 105‰ at higher contracts). | SC-A4; U0, U0b | `godgap_run1.txt`, `horizon_run1.txt`; gates `solver_godgap.rs`, `solver_horizon.rs` (5); [open questions](walt-math-open-questions.md) item 21 |
| **FH-God, FH-int, FH-cut, FH-last, FH-tie** | The world-revealed continuation is a Bellman supersolution under a deterministic field (terminal-exact, public-branch harmonic, focally optimistic), so `Q ≤ G` and `U_0 ≡ G`; the §23 interruption rule is sound under an intersection discipline with the policy travelling with every lower fact; a ply cut at a trick boundary on a viewer-lead root is a focal cut (cut-4 = `U_{a,0}`, cut-8 = `U_{a,1}`) — so U0/U0b were the hierarchy's endpoints before it existed; trick 7 forced ⇒ trick-4 roots exact at k = 2, trick-3 at k = 3; survivors exact when all have collapsed. | delivered with FH-A1..A11 | `CENSUS-RULINGS.md` § "The focal-horizon adjudication", the five propositions |
| **The trivial upper is a refusal** | `GodUpper::fact() = None`: an unaffordable God branch is a typed refusal that leaves its child unfinished, never an installed upper of 1. | FH-A3, FH-A11 | `solver/focal_horizon.rs`; gate FH-C (the in-pass cap refusal, added after the FH4 audit) |

**The measurement that last moved it — the direction-changing one.** FH3's
report of record (33 (root, contract) coordinates at k ≤ 3, 2026-09-04,
`walt/probes/factor_belief/focal_run1.txt`, anchors gated in
`solver_focal_anchors.rs`, independent FH4 audit PASS with one vocabulary
BLOCK fixed): every live trick-4 coordinate settles by k ≤ 2 — five at k = 0
with no search, six at k = 1 with Γ₁ ≤ 45‰, three at the k = 2 collapse — and
per action, once one focal layer is explicit, **`U − Q` is 0–3‰ at trick 4 and
1–2‰ at trick 3 while `Q − L` is 9–41‰ and 12–33‰**: the residual is the tail's
policy gap, not the fusion price. The trick-3 anchor h8-t3 settles only at the
k = 3 collapse (survivors 5 → 5 → 3 → 1; Γ 141 → 100 → 34 → 0‰), reproducing
`28859/29988` at 1-1. The two ply-cut flips U0b had reported (h8-t4 at bids
36/39) are upper-side artifacts and no coordinate anywhere certifies a wrong
action. Costs as findings: 190M reads for the exact trick-3 answer with suffix
reuse against 289M plain; 3.82M facts and **19.4 GB** peak RSS at h8-t3.

**Owning pages.** [walt-decision-sparse](walt-decision-sparse.md) (the FT/SR
ladder); [walt-focal-horizon-era](walt-focal-horizon-era.md) (U0, U0b, the
hierarchy, the consolidation ruling); [walt-negative-results](walt-negative-results.md).

---

## Chapter 7 — How deep to look: sampling as calculation

**Plain statement.** When the hidden worlds cannot be counted exactly, walt
samples them — but never "n worlds" as a magic number. The work is
*calculated* from declared risk, the contenders' separation, and the
evidence observed so far, and the sampling stops when the evidence settles the
decision or the budget is honestly exhausted. "How many is enough" is not an
open question; it was answered by design on 2026-08-24.

| Object | One line | Ruling / parent | Full statement |
|---|---|---|---|
| **Signed pivotal geometry** | `g = q·τ`, `E[Y²] = q`, `Var(Y) = q − g²`, `H = 1/(qτ²) − 1`; world/tape projection; the cover identity. One general claim false as written, repaired: pairing is sharper iff `Cov(u_a, u_b) > 0` (SP-A5). Discovery ≠ evaluation; vocabulary pivotal cover / pivotal win share / frozen policy. | SP-A1..A12 (2026-08-18) | `signed_pivotal_geometry_v0.1.md` + intake companion; `walt/TILT-AUDIT.md` |
| **Calculated evidence CE-T1..T5** | Anytime-valid exact-rational evidence processes (Bernoulli-threshold, signed-pivotal, bounded-mean betting supermartingales); a run → decision → edge risk ledger (`δ_d = δ_run/(d(d+1))`); the information rate **`𝓘 = q·D_{1/2}(τ)`** as the only sampling-cost coordinate (never `q̂` alone); monotone escalation to exactness. Fixed counts leave the correctness path (CE-A5); a sample cap is a resource limit, never a proof rule. | CE-A1..A8 (2026-08-24) | `calculated_evidence_v0.1.md`; gates `solver_evidence.rs`, `solver_adaptive.rs`, `solver_controller.rs` |
| **The six-way result ladder** | `ExactFiberRoot / ExactFrozenSet / DeltaSettled / EpsilonEquivalent / Unresolved / HeuristicFallback`, binding on every new path; exact-for-the-frozen-set ≠ exact root. | CE-A3 | gate `result_kinds_serialize_with_the_type_tag_preserved` |
| **Claim D's counterexample and the future-only rule** | Retrospective edge-risk assignment fails: three one-shot e-values of mass 1/8 at 8 give false-cross probability `1 − (7/8)³ = 169/512 > 1/4`; repairs are future-only opening or preallocation. "Refund" language retired. | PANEL-A3, PANEL-A4 (x:019–023) | `response_walt_panel_and_cancellation_v0.1_intake.md` |
| **The max-preserving upper CS (Theorem M1 / Corollary M2)** | For finitely many branch means under arbitrary dependence the branchwise-max upper endpoint covers `R = max μ` at the *same* δ — no Bonferroni split — and endpoint monotonicity collapses the family to the empirical-optimum count the shipped solver already computes: the sup is a one-mean problem for a maximizer you never name. Branch-mixture e-processes retired for uppers on maxima. | TRIPLE-A2, TRIPLE-A3 (x:024) | `response_deferred_producers_triple_v0.1_intake.md`; gates `solver_e3_upper.rs` (worst undercoverage 11/128 < 1/4 and the specimen `R = 1/2 ≤ E3 = 3/4 < E2 = 1` are gate-pinned) |
| **world_cap 512** | Engineering, not mathematics: Jason's 2026-08-24 ruling that 128/40/160 were phone-tier budgets; `ActConfig::interactive() = 128`, `::full() = 512`, exact cap 2000. | ruling 2026-08-24, PR #32 | [walt-calculated-evidence](walt-calculated-evidence.md) § "The cap analysis" |

**The measurement that last moved it.** Step 8 (2026-08-24,
`walt/probes/step8/`): on one epoch and one common stream at caps 40/160/640
no cap-dependent flip occurs anywhere — the historical 40-vs-160 flip comes
back as an honest `Unresolved` near-tie at every cap (the V5 law gated) — and
the per-pair E0 calibration reads 45/54 DeltaSettled with 45/45 winner-sign
agreement against exact τ. The shadow instrument beside the live player
(`walt/probes/shadow/`, results file outranks the era prose): at world_cap 128,
183 decisions, 67 ExactFrozenSet / 116 Unresolved, live choice among survivors
116/116; at the separately committed **512 epoch**, 3 DeltaSettled — two of
them *against* the live choice — and the live opening lead δ-eliminated once;
the 128-epoch forecast "~108/116 settle by 512" was **wrong** (observed 2 of
42) and the README retracts it. The sampled route at tricks 1–3 is honestly
Unresolved almost everywhere, which is why the counted representation of
Chapter 2 replaced it as the primary path and this engine became the fallback
tier.

**Owning pages.** [walt-calculated-evidence](walt-calculated-evidence.md);
[walt-seat-play](walt-seat-play.md) (the tilt audit, the controller as a
player).

---

## Chapter 8 — Choosing the model: fields, levels, and the field as hidden state

**Plain statement.** Everything above is relative to a declared model of how
the other three seats play — the *field*. Changing the field changes every
number, so walt keeps two axes apart by name: **CE** is how deep to sample
under one field; **L2** is which field. A "level-2" walt is a best response to
a *named* level-1 field, never an equilibrium, and the newest mathematics makes
the field itself one more hidden coordinate the seat holds a belief over.

| Object | One line | Ruling / parent | Full statement |
|---|---|---|---|
| **The rung registration** | `D = FieldModel::Dice`; `F₀ = BR(D) = σ0`; `F₁ = BR(F₀)` = level-1 walt; `F₂ = BR(F₁)` **unbuilt**. Corrected at intake — "Dice = σ0" was wrong and retracted. | MB-A3 | `CENSUS-RULINGS.md` § "The model-belief base-player adjudication" |
| **L2-T1..T5** | First-disagreement localization; the root-action Lipschitz bound `|Q_a^(1) − Q_a^(0)| ≤ R_a`; winner stability when margin > `R_a + R_b`; safe admissible-set screening (only a `RootActionExposureUpper` may feed it); eventual periodicity of deterministic best-response towers (a genuine period-4 cycle exhibited). Model-checked 19/19 over 1,584 enumerated games. | L2-A1, L2-A4 | `targeted_level2_field_stability_v0.1.md`; gates `solver_fieldswap.rs`, `solver_fieldswap_screen.rs` |
| **Exposure rungs E0–E4** | Exact equality → structural cover → clairvoyant reach → the exact split-reach route whose optimum *is* `R_a`; the ladder `E1 ≥ E2 ≥ E4 = R_a ≥ d_ρ` verified exactly. E0 fired at receipt-h7-t5 (`R_a = 0` exactly); the first pruning singleton at h4-t6. | L2-A4; slice 2 | `walt/probes/fieldswap_screen/` |
| **The cancellation ladder and the Hazard-Exclusion Invariant** | `|c| ≤ r ≤ d` with three distinct zeros (behavioral, outcome, value); pairwise `(B, H, q, g)`; **dominance** `H(a|b) = 0 ∧ B(a|b) > 0` is one-sided unforced risk and never "cancellation"; the invariant (initial coverage, forward closure, terminal safety) is the single dominance-bound authority, sound and semantically complete; sampled zero hazards never prove `H = 0`. | PANEL-A7/A8; TRIPLE-A4/A5 | `solver_fieldswap_cancel.rs`, `solver_hazard_witness.rs`; `walt/probes/fieldswap_cancel/` |
| **The six-motif alphabet** | First-split morphology + Other with mandatory orthogonal flags, partitioning *correction mass* only; `RevealResponse` refused pending raw suffix enrichment. | TRIPLE-A6/A7 | `solver_fieldswap_motifs.rs`; `walt/probes/fieldswap_motifs/` (453/453 classified, residual 0) |
| **The cycle tripwire** | Recurrence claims typed root / behavioral / local exact / global exact; the §13.5 σ₁-vs-σ₂ tripwire is a standing precondition on level-3 work; no damping, mixtures or robust-cycle policies without a separate intake. Never run — no σ₂ exists. | L2-A7; MB-A3 | [open questions](walt-math-open-questions.md) item 15 |
| **Ξ = Ω × Θ — the field as hidden state** | Theorem 7.1: a finite model-belief problem on `Ξ = Ω × Θ (× Z)` *is* a fixed-semantics walt problem, so every theorem using only finiteness, lawful focal information, bounded utility and a fixed latent distribution lifts verbatim; persistence (the type is a coordinate of the record); the residual **Other** type is mandatory (known mass `1 − r` lifts `[L, U]` to `[(1−r)L, (1−r)U + r]`); the model-fusion price `Φ_a = U^sep_a − Q_a(ν)` with merge-before-max over public actions; Theorem 19.1's corollary: zero at one full-support belief ⇒ zero at every belief. | MB-A1, MB-A5, MB-I4 | `model_belief_base_player_v0.1.md` §7, §15, §19; gates `solver_model_belief.rs`, `solver_model_belief_recursion.rs` |

**The measurement that last moved it.** MB1 (2026-09-02): the model-fusion
price under the registered F₀/F₁ mixture at ν = (1/2, 1/2) per hidden seat is
**strictly positive at trick 4** on all eight substantive coordinates — h8-t4
47/9600, **38/9600**, 90/9600, 58/9600 on 2-1/3-1/3-3/5-5; h3-t4 173/46200,
157/92400, 37/6600, 7/2200 — and exactly zero at every trick-5/6 receipt
coordinate. **The 38/9600 at h8-t4 3-1 is gate-pinned** (`M6_SPECIMEN =
(8_323, 8_361, 9_600)` in `solver_model_belief_recursion.rs`); the rest is
probe record (`modelbelief_recursion_run1.txt`, `walt/briefs/MB1-REPORT.md`).
The affordability wall of the mixture: trick 4 costs 98–412 s per root, and a
trick-3 mixture coordinate refused all five root actions at 7,000,000 reads.
Step 9 (2026-08-25, `walt/probes/step9/`, bin `wakeup`) supplied the first
corpus-level wake-up data: exact value wake 18/18, decision wake 8/18 (five
outright winner flips), pivotal mass *dropping* under σ1 on most pairs — the
wake lives in the value and decision channels, not the response channel. And
at the table, no partner model has beaten level 1 (Chapter 12; the results
files of [walt-partnership-program](walt-partnership-program.md)).

**Owning pages.** [walt-calculated-evidence](walt-calculated-evidence.md)
(the L2 thread through step 9 and the targeted controller);
[walt-focal-horizon-era](walt-focal-horizon-era.md) (MB0/MB1, σ1 repair);
[walt-seat-play](walt-seat-play.md) § "The level-2 question".

---

## Chapter 9 — The score beneath the contract

**Plain statement.** pmake asks one yes/no question — will the bid be made?
Underneath it is a distribution over the 43 possible declaring scores
(0..42), and one bid-blind run of the same recursion yields the whole
bid-threshold curve at once. The score layer is what lets walt say *why* a
hand is safe or fragile, and what it must not do is treat an envelope across
policies as if it were a policy.

| Object | One line | Ruling / parent | Full statement |
|---|---|---|---|
| **The 43-bin profile and the tail-sum identity** | `viewer_score_profile` yields the exact declaring-score profile (bin `s` = mass banking exactly `s` points); `E[S] = Σ_k Pr(S ≥ k)`; one profile is the record of ONE policy. | APS-A2 | `anytime_proof_state_score_v0.1.md`; gate `solver_factor_profile.rs`; record `profile_run1.txt` |
| **`W_ρ(c)` — the contract-sensitive residual** | The steering object: mass the policy leaves unresolved *for this contract*; rescue and fragile bands; count-threat covers as verified resource decompositions with `declaring_score_range`-verified movement bounds. | APS-A3 | §§ score layer; gates `solver_covers.rs` (3), `solver_residual.rs` |
| **The envelope fence** | A threshold-wise envelope across policies is never an executable profile — the **63/2** non-realisability specimen; no envelope is serialised as an extracted policy. | APS-A4 | gate `no_threshold_envelope_is_serialized_as_the_extracted_policy` (`solver_extraction.rs`) |
| **Bid-blind reuse and its σ0 boundary** | The profile is bid-blind, but σ0 *reads the bid*, so cross-contract reuse of a σ0 evaluation is **VOID**: frozen specimen h10-t6 at threshold 42 — projection 12 ≠ evaluation 9. σ0's make mass spikes exactly at the bid (h8-t5 σ0-as-focal: 445‰ at s = 30). | Phase 2 finding | `profile_run1.txt` (the h10-t6 specimen and the spike are probe record); the contract-reuse law itself is gate 2 of `solver_factor_profile.rs` |
| **Covers at rich roots — the §70 caveat** | h12-t6's verified gain 0 (against an arithmetic envelope of 7) certifies `V* = 0` for one range walk; h4-t6's range walk beats arithmetic by exactly one point; at rich roots (h8-t5, h5-t6, h3-t4) every resource is contested and first-generation covers are vacuous. | Phases 4+5 | `bellmanreport_run1.txt`; `solver_covers.rs` |

**The measurement that last moved it.** Phase 2 (2026-08-31): the exact 43-bin
profile costs 7–12% extra wall at trick 4 for roughly double the nodes
(h3-t4 σ0 1,148,591 µs against 1,023,334 µs for success mass); certain
outcomes carry their explanation (h12-t6 a single bin 20:6; h10-t6 35:1 41:6
42:12). Nothing after Phase 5 touched the score layer; the partnership gym's
"points are diagnostics only" (Chapter 12) is the same fence read from the
other side.

**Owning pages.** [walt-counted-belief-era](walt-counted-belief-era.md)
(Phases 2, 4, 5).

---

## Chapter 10 — Symmetry and transport, and why it pays little here

**Plain statement.** Two positions that are the same up to a relabelling
should have the same value, and a tablebase should store one of them. In 42
the relabellings that survive the rules are few, count is not preserved by
any of them, and the one that pays — the seven pip declarations folding 7:1 —
transports values and policies but never verdicts by themselves.

| Object | One line | Ruling | Full statement |
|---|---|---|---|
| **Lemmas V, X, E, S** | A node-rule value reading only actor offset and canonically ordered pairs is constant on r3 classes (V, not for treatment H); zero-contribution excision is one-sided (X); equal r1 canonical forms ⇒ isomorphic remaining games ⇒ equal *count-free* value (E, fenced by E-A2); the seat-side analogue (S). | P-A, X-A, E-A, S-A (2026-08-11) | [structure and transport](walt-math-structure-transport.md) |
| **Corollary R-fold** | The predictive dimension is declaration-fold invariant — the branch's only exhibited value-order isomorphism (DS-A25). | R-A | same page |
| **Lemma E7 — when dominance travels** | Dominance does not travel with a policy alone; seeds are heuristics for *finding* witnesses, and witnesses are validated by exact evaluation, always (DS-A15). | DS-A24 | errata §8.3 |
| **E-A2 — the count boundary** | Structural transports preserve BEATS relations, not pip counts; if count re-enters, every form-keyed record is void **wholesale, never extended**. The one asset that crosses: a lawful policy as a primal-witness source (DS-A16) — policies extend, verdicts do not. | E-A2, DS-A16 | `CENSUS-RULINGS.md` § "Endgame-store rulings"; [open questions](walt-math-open-questions.md) item 8 |
| **Jason's §12.6A ECL theorem and its pmake instance** | The lossless count-free equivariant quotient (v0.5); the concrete instance on hand 8's t = 4 pool is a provably almost-trivial group with predicted compression < 2× (Chapter 3). | v0.5; 2026-08-17 ruling §Q2 | `equivariant_lumpability_v0.5.md`; `WALT-MATH-RULING-2026-08-17` §Q2 |
| **Freeze 36 v2** | The separation probe's transport opened to the declaration fold under an explicit image-key construction, receipts, and Corollary S-fold-val; every further transport still re-enters. | EC-A8 | [the freeze register](walt-math-freezes.md) |

**The measurement that last moved it.** None since 2026-08-17. The counted
representation of Chapter 2 does not use symmetry at all — it counts hands,
and hands do not fold — which is the practical verdict on this chapter: at the
scales walt now solves exactly (trick 4 in seconds to minutes, trick 3 once)
symmetry was never the lever.

**Owning pages.** [structure and transport](walt-math-structure-transport.md);
[walt-pre-pivot-results](walt-pre-pivot-results.md).

---

## Chapter 11 — The discipline that makes the mathematics citable

**Plain statement.** None of this would be quotable a week after it was
written without a set of rules about how a received document is handled,
how a number is frozen, and what counts as evidence. The rules were each
bought with a mistake; the appendices below carry the full lists. This
chapter gives the shape.

| Rule | What it says | Where it was bought |
|---|---|---|
| **The intake protocol** | A received parent is filed **verbatim** with a `.sha256` pin; an intake **companion** governs wherever it narrows or repairs the parent; a stdlib **verifier** is scratch-tier session evidence, never a receipt (TRUST-01); rulings are filed the **same day**; obligations go into a named ledger. Nine parents are pinned and all nine re-hash exactly (re-verified 2026-09-07; the FH parent re-hashed 2026-09-13 on this machine: `892bc343…`). | DS-A18; SP-A11 (an unfiled import is a defect); MB-A4 (transit damage recorded, not repaired) |
| **Freezes** | A freeze clause states a constant *or* a generating rule, never both; a receipt comparing against a prior run names the **carrier** of the reference value; the register is the authority for what is issued — 58 numbers, 56 spent, 39/40 reserved, and freeze 58 named by the register alone (Appendix G). | FT-A23(v), FT-A28(i), FT-A29(i); [the freeze register](walt-math-freezes.md) |
| **NO-RESCUE (F7)** | Both outcomes of every experiment are results; a mismatch against the concrete authority is stop-and-report, never patched. | the census fork, 2026-08-10 |
| **"By construction is not a receipt" (PG-A8)** | An assertion that cannot fail is not evidence; a check must not produce what it checks — SR-A33's lesson from the FIPS pattern. | SEP-A13; SR-A33 |
| **Scope naming** | Every count names its state set; a capped coordinate reports no count at all (PG-A13); "attained, never exact" (FC-A22); a probe number becomes quotable only through a gate or a verifier receipt, and the probe README is the authority for probe findings (CBS-A8). | PG-A13, FC-A22, CBS-A8 |
| **The supersession chain** | The rulings file is append-only; a corrected clause gets a bracketed pointer marker at its site (DS-A28(i)); the corrected mathematics is filed in the maintained errata (DS-A28(ii)); received documents stay verbatim, walt's own documents stay correct (DS-A28(iii)). Post-2026-09 corrections that are not rulings live in `walt/DISCREPANCIES.md` (Appendix D). | DS-A28 |
| **The live-default fence** | No lineage changes the default player until arena and conformance gates justify it on Jason's word. Restated at every intake since. | CE-A7/§20.16; CBS-A9; APS-A9; MB-A7; FH-A10 |
| **Vocabulary** | Witness vs receipt; no "certificate" (D3, DS-A1) except the phrase *necessary outer profile*, bracketed quotation, and *certified regret*; no "sandwich" as an object name (CBS-A3, FH-A2); bare "laydown" reserved for the universal type (APS-A5); level 2 = best response to a named σ1 (O36, L2-A7). The FH4 audit's one BLOCK was a single word. | [vocabulary](vocabulary.md) |

**Owning pages.** [walt-math-intakes](walt-math-intakes.md) (the protocol
applied, artifact by artifact); [the freeze register](walt-math-freezes.md);
Appendices D and E below.

---

## Chapter 12 — What the seat's mathematics still owes

**Plain statement.** By idea, not by date. Each line names the question, the
ruling that fixed its shape, and the measurement that last moved it; the full
statements are on [open questions](walt-math-open-questions.md), whose item
numbers are given.

| Owed | Ruling / report that shapes it | Last measurement | Item |
|---|---|---|---|
| **The tail** — the cheapest lawful tail, and whether a tail-improvement ladder converges faster than `k` | FH-A4 (σ0-as-focal is the primary tail), FH-A7; the FH-RESPONSE-TO-PRO letter, question 1 | FH3: `Q − L` 9–41‰ vs `U − Q` 0–3‰ at trick 4 (2026-09-04) | 19 |
| **σ0's sufficient statistic** — the read-key study | `walt/MAP.md` "Next"; letter question 2 | cross-history cache reuse exactly 0 (`cache_run1.txt`); warm instance 15× faster at identical reads | 18 |
| **The honest guarantee at tricks 1–3** | APS-A7 (Γ is model-relative); letter question 3 | opening Γ 267‰ at p = 512, split UNKNOWN (2026-09-01/03) | 20, 13 |
| **The fusion-free-suffix hypothesis** | SC-A4 (empirical first) | trick 5 at receipt roots; not inside trick-4 solves (U0/U0b, 2026-09-02/03); contract-varying falsifier not run | 21 |
| **A coarser lawful seat equivalence** | S-A20 (open since 2026-08-11); S-A21's reading (compression is bought with deadness) | none | 2 |
| **Moment compilation** vs the unintaken IMPROVISATION note | DS-A11; freeze 39 reserved | none adjudicated (2026-09-05 note, no intake) | 4, 27 |
| **The specimens' tie mechanism** | J-A8 (deliberately unfilled) | 276 / 1,239 / 1,773 ties, zero detector hits (2026-08-12) | 1 |
| **The count-and-score lift** | DS-A12; E-A2 | none | 8 |
| **The x:018 covering dual** | correspondence 2026-08-14; SC §15's rational packing is the nearest built object | no reply after 24 days at `c00717d1` | 11 |
| **The objective and the tie-break at exact indifference** | none — Jason's call (C); `MORNING-2026-09-05.md` item 4 | G2 locked from trick 3, all 280 trick-4 deals at 1/1 at both levels (2026-09-05, branch record) | 16 |
| **Void-aware inner belief (O5)** | SCENARIO-PLAYER O5; Jason's rank/unrank ruling 2026-09-05 | live 33/19/20 (+262 of 6,048) vs reduced 76/83/33 (−226 of 16,128); 9/5/36, 5/8/37, 12/17/71 — non-composing | 17 |
| **The (b)/(c) ordering** in the unified player | UP0-REPORT "What UP1 needs" 4 | two argmax-flip specimens gate-pinned (2026-09-02) | 22 |
| **The divergence referee** | `divergence_results_2026-08-18.txt` header | never run | 23 |
| **G2/G3's deal; G4** | gran-anchor card; call (F) | six-way ambiguous; G4 does not exist | 24 |
| **The errata §9 / §4.3 debt; freeze 58's number** | DS-A28(ii); APS-A9 | unfiled / unnamed, verified 2026-09-13 | 25 |
| **Lean catch-up** for everything after the GT1 tree; the four obligation ledgers in no file | GT1-A23; CBS-A9, APS-A9, MB-A8, SC-A2; `kanban/backlog/lean-catchup.md` | none | 26, 9 |
| **The three unintaken packet notes** | the walt/math convention; SP-A11 precedent | manifests verify; no intake | 27 |
| **The ladder's memory** | FH4 N8; `[[ladder-policy-store]]` | 19.4 GB at h8-t3, sinks unmeasured | 28 |

Two things that look owed and are not: the magic-sample-count question
(answered by design, CE-A5) and the rank question for the distribution
contracts (a theorem row, Lemma R(c)) — both recorded under "Things that are
settled" on the open-questions page.

---

# Appendices

The appendices are the reference material the 2026-08-24 edition of this page
carried, kept in force and extended through `c00717d1`: the governing
documents, the object index of the pre-2026-08-24 lineages (the later lineages
are indexed in Chapters 1–9 above), the ruling-family table, the supersession
chain, the standing disciplines, the run-name decoder, what is owed on the
record, and the dated historical addenda kept verbatim.

## Appendix A — The governing documents, and which governs

walt's decision-sparse mathematics lives in three places with three different
disciplines. The GPU-native trick-1 branch adds a received source, a maintained
portable contract, exact accepted M2/M3 rebriefs and frozen binding M2/M3
contracts; the signed-pivotal and scenario-player era adds a checksum-pinned
received parent with an intake companion, a maintained seat spec, and two probe
documents. The distinction is load-bearing and is the first thing to
internalise. The full artifact-by-artifact provenance map is
[received artifacts and intakes](walt-math-intakes.md).

| Document | Discipline | Role |
|---|---|---|
| `walt/math/decision_sparse_exact_solving_v0.1.md` and `walt/math/decision_sparse_second_audit_v0.1.md` | **Received, verbatim, never edited** (DS-A18) | Handed-in documents. Preserved exactly as filed, for the same reason `ingest/` is: a corrected source destroys the record of what was corrected. |
| `walt/math/gpu_native_trick1_implementers_guide_v0.2.md` | **Received, byte-frozen and checksum-gated** (GT1-A1) | GPU-native design input. Original source commit `ca18bc6807b974b31d4640786d7a2d63ae0b79fe`, intake commit `c230949c77ff7e8e22f912ed70f8206488ac9022`, SHA-256 `ee2e78da20eb7d087fb121f467a56bafc0179a45fb692ca0b938f4c4210b6a44`. It remains the record of what was received, not a silently repaired source. |
| `walt/math/decision_sparse_exact_solving_v0.1_errata.md` | **Maintained** (DS-A28(iii)) | The repaired mathematics, with full statements and proofs. Hypotheses may be added and language narrowed *in place*, each change carrying a dated provenance marker naming its ruling. |
| `walt/GPU-NATIVE-TRICK1.md` | **Maintained adjudicated contract v0.3** (GT1-A1) | Binding first-build authority for the GPU-native branch wherever it narrows, repairs or rejects the received v0.2 guide. It does not promote the branch, prove the Rust implementation, or report Metal. |
| `walt/math/gpu_native_trick1_m2_rebrief_v0.1.md` | **Exact accepted M2 rebrief** (GT1-A10) | The mandatory bridge from freeze 55 to M2: 44,079 bytes, SHA-256 `9183132529a42289a104a73d8f7e196eb95058ac2edda60bb42c715f1f8a139a`. It supersedes the old host observation only as current environment status; it does not rewrite that receipt. |
| `walt/GPU-NATIVE-TRICK1-M2.md` | **Frozen binding M2 contract v1** (GT1-A17) | The exact M2 arithmetic/projector parity authority, SHA-256 `aacb6df5e9106b3b6bf00ccfb496c71f762c0fb4644c13a17f76d2ac2f0326e3`. It neither widens the portable parent nor authorizes an action-value, controller, performance or player claim. |
| `walt/CENSUS-RULINGS.md` | **Append-only** (DS-A28(i)) | The adjudication record. No ruling's text is ever rewritten; a corrected clause receives a bracketed dated pointer marker at its site. Also the home of the named lemmas from before the errata existed. |
| `walt/math/gpu_native_trick1_m3_rebrief_v0.1.md` | **Exact M3 rebrief** (GT1-A18) | The mandatory bridge from freeze 56 to M3, SHA-256 `07b3c993…`. The binding contract supersedes it only where more specific. |
| `walt/GPU-NATIVE-TRICK1-M3.md` | **Frozen binding M3 contract v1** (GT1-A24) | The M3 perfect-recall-net gate authority, SHA-256 `79de73e9…`, under **freeze 57**. It authorizes only the gate and records **no M3 result**; the GT1 range is re-frozen closed at A1..A24. |
| `walt/math/signed_pivotal_geometry_v0.1.md` | **Received, verbatim, checksum-pinned** | Pro's signed-pivotal-geometry note (2026-08-18), SHA-256 `b9d93715…`. Audited at SP-A1..A12; SP amendments bind every consumer. |
| `walt/math/signed_pivotal_geometry_v0.1_intake.md` | **Intake companion** | The exact-rational verification of every boxed identity in the parent, which the SP-A audit takes as read. |
| `walt/SCENARIO-PLAYER.md` | **Maintained spec v0.1** | The sampling-stack seat's mathematical specification — definitions and **proof obligations, not established results**; its §10 obligations ledger (O1–O9, O12–O19) is the graduation queue. |
| `walt/TILT-AUDIT.md` | **Maintained probe document** | The E0 experiment as adopted and amended by SP-A1..A12; smoke design, objects, and the 2026-08-19 smoke results. Estimates, never receipts. |
| `walt/LEVEL2-PROBE.md` | **Maintained probe spec, amended by dated appends** | The level-2 field-swap detector — amended 2026-08-24 with the three-way wake-up split and the `𝓘` cost coordinate (CE-A6) and reconciled as the **detection layer** inside the targeted level-2 controller (L2-A5); the targeting layer is owned by the field-stability parent. **Step 9 ran on 2026-08-25** (PR #49, `solver::wakeup`, bin `wakeup`; records `walt/probes/step9/`); step 8 landed 2026-08-24, and the two field-swap slices built the detection layer's inputs (exposure rungs E0–E2, the exact route E4, the L2-T4 screen). |
| `walt/math/calculated_evidence_v0.1.md` | **Received, verbatim, checksum-pinned** | *Calculated Evidence for Unified Walt* (hand-ferried 2026-08-24), SHA-256 `9b32b14f…`. Anytime-valid adaptive settlement (CE-T1..T5), risk ledgers, the information rate `q·D_{1/2}(τ)`, exact-fiber escalation, result typing, O20–O28. **Adjudicated CE-A1..A8 same day**; the §22 sequence is the build program (CE-A7). |
| `walt/math/calculated_evidence_v0.1_intake.md` | **Intake companion** | The exact-rational verification of every boxed identity in the parent (18/18 PASS, three-way on the central closed form; `verify_calculated_evidence_v0.1.py`), vocabulary and O-numbering adjudication notes, verified code boundaries, and the Pro refinement agenda. |
| `walt/math/targeted_level2_field_stability_v0.1.md` | **Received, verbatim, checksum-pinned** | *Targeted Level-2 Field-Swap Geometry for Unified Walt* (hand-ferried 2026-08-24, second drop, same lineage), SHA-256 `597d33c3…`. First-disagreement localization (L2-T1), root-action field Lipschitz and winner-stability bounds (L2-T2/T3), safe admissible-set screening (L2-T4), best-response tower periodicity and cycle typing (L2-T5), three exposure tiers, rungs E0–E4, first-split traces, Gran anchors G1–G4, experiments L2-E0..E6, proposed O29–O38. **Adjudicated L2-A1..A7 same day** under the standing same-lineage go; the field-swap build slots after the CE shadow step (L2-A6). The Gran anchors it names were **reconstructed 2026-09-04** — G1 complete and mechanically validated, G2/G3 a validated partial ([walt-gran-anchors](walt-gran-anchors.md)). |
| `walt/math/targeted_level2_field_stability_v0.1_intake.md` | **Intake companion** | Exact finite-game model checking of L2-T1..T5 (19/19 PASS over 1,584 enumerated games, every policy and world exact; `verify_targeted_level2_field_stability_v0.1.py`), all eight L2-E0 phenomena exhibited, vocabulary/numbering freshness audit, code-boundary and LEVEL2-PROBE reconciliation notes, the Gran-anchor gap (closed for G1 on 2026-09-04; the companion's own pointer to `walt/probes/gran/` is still owed per the anchor card), and the seven-point adjudication agenda. |
| `exchange/inbox/019-023-response-panel-and-cancellation-v0.1.md` + `walt/math/response_walt_panel_and_cancellation_v0.1_intake.md` | **Received (x:019–023), checksum-pinned; intake companion** | Pro's adversary-panel response (2026-08-24, SHA-256 `a3f468aa…`): four briefs certified by Pro's own step-check (promotes nothing), Claim D counterexampled (169/512), Part VI's cancellation ladder and directional bounds. **Adjudicated PANEL-A1..A8** same day; verifier 36/36 = session evidence. Chapters 7–8. |
| `exchange/inbox/024-response-deferred-producers-triple-v0.1.md` + `walt/math/response_deferred_producers_triple_v0.1_intake.md` | **Received (x:024), checksum-pinned; intake companion** | The deferred-producers response (2026-08-25, SHA-256 `337296a7…`): the max-preserving upper CS (Theorem M1 / Corollary M2), the Hazard-Exclusion Invariant, the six-motif alphabet. **Adjudicated TRIPLE-A1..A7**; verifier 13/13 = session evidence; all three producers built with gates the same night. Chapters 7–8. |
| `walt/math/counted_belief_sandwich_v0.1.md` + `_intake.md` | **Received, verbatim, checksum-pinned; intake companion** | *The counted-belief sandwich* (hand-ferried 2026-08-30, SHA-256 `4d2dfbe0…`; the title is quotable, the word is not an object name — CBS-A3). Theorems 2.1, 5.1, 9.1, 17.1, 20.1, 23.1, 30.1; the C→G build program (§56/§57). **Adjudicated CBS-A1..A9** same day; verifier 20/20 = session evidence; obligations CBS-O1..O15. Chapters 2, 5. |
| `walt/math/anytime_proof_state_score_v0.1.md` + `_intake.md` | **Received, verbatim, checksum-pinned; intake companion** | *Anytime proof state and the score calculus* (2026-08-31, SHA-256 `7a8c60fb…`): the score layer, the typed laydown hierarchy, proof bar / executable bar, certified regret, the §49 architecture. **Adjudicated APS-A1..A9**; verifier 36/36 = session evidence; obligations PS-T1..T15. Chapters 4, 5, 9. |
| `walt/walt/src/solver/refine.rs` | **Frozen — freeze 58 (RefineV1), issued by the register at APS-A9** | The Slice G integrated controller at main `25b40d9`, semantically frozen as the reference the §49 proof state must reproduce; four gates in `solver_factor_refine.rs`. The rulings file never names the number (Appendix G). |
| `walt/math/model_belief_base_player_v0.1.md` + `_intake.md` | **Received, verbatim, checksum-pinned; intake companion** | *The model-belief base player* (2026-09-01, SHA-256 `1ffabf86…`): `Ξ = Ω × Θ`, Theorem 7.1, persistence, the residual Other type, the model-fusion price; two transcription errata recorded, companion governs (MB-A4). **Adjudicated MB-A1..A8**; verifier 40/40 = session evidence; obligations MB-O1..O20 / MB-I1..I10. Chapter 8. |
| `walt/math/salvation_complex_v0.1.md` + `_intake.md` | **Received, verbatim, checksum-pinned; intake companion** | *The salvation complex* (2026-09-01, SHA-256 `eca69bd5…`): Theorems 6.1, 7.1, 12.1, the §13 split, the §23/§26 counterexamples, the fusion-free-suffix hypothesis. **Adjudicated SC-A1..A8**; **no verifier shipped — U0's gates are the checks** (SC-A2); obligations SC-O1..O16 (§60). Chapters 4, 6. |
| `walt/math/focal_horizon_sandwich_v0.1.md` + `_intake.md` | **Received, verbatim, checksum-pinned; intake companion** | *The focal-horizon sandwich* (2026-09-04, SHA-256 `892bc343…`; title quotable, word retired — FH-A2): Theorems 1–6 with full proofs P0–P14 in the companion; five propositions delivered in the rulings (FH-God, FH-int, FH-tie, FH-cut, FH-last). **Adjudicated FH-A1..A11**; verifier's "24 CHECK FAMILIES" is a printed literal (31 asserts in 18 blocks; FH-A1). Chapters 5, 6. |
| `walt/math/WALT-MATH-QUESTION-2026-08-17-…` and `WALT-MATH-RULING-2026-08-17-pmake-and-the-walk-to-trick-1.md` | **In-house question/ruling pair; advisory, no ruling family, no `.sha256`** | P1–P4 and the path-dependence counterexample (Chapter 1); the §12.6A instance and the allowance automaton (Chapters 3, 4). Its basis, `ladder.rs` in `walt/walt-m3-probe` at `171cd22`, no longer exists on main (crate deleted 2026-08-24); read historically. |
| `walt/briefs/*-REPORT.md`, `walt/briefs/FH4-AUDIT.md`, `walt/briefs/MORNING-2026-09-05.md`, `walt/MAP.md` | **Reports of record and readouts (2026-09-02 → 09-05)** | Not parents and not rulings: the measured findings of MB1, U0, U0b, UP0, UP1a, FH1–FH3 and the independent FH4 audit, plus the 6-4 readout. A number in them is quotable only through the gate or record they name. |
| `experiments/partnership/packet/…/TEXAS42-UNIFIED-REVIEW-v0.1.md`, `…/TEXAS42-IMPROVISATION-v0.1.md`, `experiments/partnership/packet/texas42_relational_learning/PATH-TO-GENERALIZING-SCHEME-POLICIES-v0.1.md` | **Received, hashed by their packets, NOT intaken** | Three mathematical notes of 2026-09-05/07 with no companion, no ruling family and no verifier filed under the walt convention; below even the intaken corpus (Appendix G; [walt-math-intakes](walt-math-intakes.md) §9). |

**Citation rule (DS-A17), binding on every design and results file:** cite the
**errata theorem number** for the mathematics and the **DS-A ruling** for its
provenance. Where parent and errata differ, **the errata governs**. The rulings
file remains the adjudication record; it is no longer the home of the repaired
mathematics.

For portable M0/M1, cite v0.3 and GT1-A1..GT1-A9. For M2, cite the frozen M2
contract and GT1-A10..GT1-A17 with freeze 56; cite the exact rebrief when the
bridge from freeze 55 is the fact at issue. The received v0.2 guide is cited by
source identity when the fact at issue is what arrived, never as authority over a
repair. The historical Gate-0 NO-GO receipt remains true about the environment it
recorded and is never rewritten as though M2 had already existed.

A trap worth naming up front: the lemmas proved inside `CENSUS-RULINGS.md`
before the errata existed — Lemmas V, X, **E**, S, S-fold, S-det, R, G, J and
the S-rigid/R-fold corollaries — are a *different naming series* from the
errata's Lemma **E3**, **E4**, **E5.0**, **E7**, **E8** and Theorems **E1**,
**E6.1**–**E6.5**. `Lemma E` (structural isomorphism ⇒ count-free value
equality) has nothing to do with `Lemma E3` (the action-conditioned upper
witness). A naive grep for "Lemma E" conflates them.

---

## Appendix B — The complete object index of the pre-2026-08-24 lineages

Every named object in the corpus, with the page that owns it. **All exploratory
tier.** "Full statement" names the file and section where the statement and
proof live; the owning page carries the compact form and the caveats.

### Structure, transport, and the quotient — [page](walt-math-structure-transport.md)

| Object | One line | Full statement |
|---|---|---|
| **Lemma V** | A value defined by a node rule reading only actor offset and canonically ordered (increment, successor value) pairs is constant on r3 classes. **Does not cover treatment H.** | `CENSUS-RULINGS.md` § "Fiber-probe rulings", Lemma V |
| **Lemma X** | Zero-contribution excision: worlds with world-informed value 0 contribute 0 to the unnormalised objective under every lawful policy, so deleting them preserves the argmax exactly. **One-sided.** | same file, § "Fiber-refinement rulings", Lemma X |
| **Lemma E** | Equal r1 canonical forms (as amended by F2 A1–A4) ⇒ isomorphic remaining games ⇒ equal count-free value. **Count-free only (E-A2).** | same file, § "Endgame-store rulings", Lemma E |
| **Lemma S** | The seat-side analogue of Lemma E: a seat transport carries every count-free censal question across. | same file, § "Seat-census rulings", Lemma S |
| **Corollary S-rigid** | The first-play seat transport group is **trivial**: the hand form *is* the hand. A proved negative. | same file, § "Seat-census rulings" |
| **Lemma S-fold** | The seven pip declarations fold exactly 7:1 under the unique order isomorphism. The S-A2 conditionality (freeze 18) is **form-level only**: transports of the recorded relational form are reading-dependent, but **values are not** — see Corollary S-fold-val. | same file, § "Seat-census rulings" |
| **Corollary S-fold-val** | Value transport along the declaration fold is **reading-independent** — Q^H, every fixed-policy L and every treatment-C U_a correspond exactly along φ whichever S-A2 reading is adopted. Extends Lemma S-fold and Corollary R-fold. | same file, § "Corollary S-fold-val" (delivered at EC-A4) |
| **Lemma S-det** | The landing state is a function of (δ, H, trick-1 record); a slough proves a void in the led context and nothing else. Determination holds; the alphabet is the raw record space. | same file, § "Seat-census rulings", inside S-Q2 |
| **Corollary R-fold** | Predictive dimension, behavioural-row counts and policy values are declaration-fold invariant. Bases and closure matrices are **not**. | same file, § "Predictive-rank probe rulings" |

### Information geometry and the width ladder — [page](walt-math-information-geometry.md)

| Object | One line | Full statement |
|---|---|---|
| **Lemma R** | Three continuation closures (event ⊇ observation ⊇ value); lawful policy values lie in V^val; and the separating-observation **degeneracy**: *where a complete continuation record determines the latent point* — as it does here — any closure seeded with a nonzero constant has dimension exactly \|X\|. | `CENSUS-RULINGS.md` § "Predictive-rank probe rulings", Lemma R |
| **Lemma G** | Backward Pareto pruning is exact (frontier preserved as a set, ties included), incremental folding is exact and mandatory, exposure is preserved — and N_vec is destroyed by both pruning rules actually in use (narrowed from "every rule" by DS-A26; duplicate-discarding rules preserve it trivially). | same file, § "Policy-geometry probe rulings", Lemma G; clause (6) as narrowed at errata §6, Theorem E6.2(c) |
| **Proposition G-flat** | Grades 1 and 2 carry no policy geometry at all; at grade 3 the only free layer is trick 2, giving N_pol = 2^k(a). | same file, § "Policy-geometry probe rulings" |
| **Definition E9** | The interface-local reachable decision width W^loc_reach(I,a) — the quantity that answers "how many policies must the seat retain *here*". | errata §8.4 |

### Decision-deadness and valuation scope — [page](walt-math-deadness.md)

| Object | One line | Full statement |
|---|---|---|
| **The three-property typing** | forced ⊂ decision-dead ⊂ dominant, both inclusions strict on the S6b evidence. Never fused. | `CENSUS-RULINGS.md` § "Decision-deadness probe rulings" |
| **Lemma J** | Non-interference (NI) ⇒ the node is decision-dead count-free; with the guard H ∩ COUNT = ∅, also under trick-plus-count. | same file, § "Decision-deadness probe rulings", Lemma J |
| **Lemma J(c′)** | The sharp valuation clause: the value is identical for every tile-value schedule **constant on H**, of which the guarded count schedule is the constant-0 case. | errata §8.5(e) |
| **Proposition J-0** | D0 — three bitset tests, exactly sound, **no exhaustion margin exists to get wrong**, because the beater is the led tile of the very trick focal plays to. | `CENSUS-RULINGS.md` § "Decision-deadness probe rulings" |
| **Proposition J-1** | D1-sym — a transposition preserving the still-leadable structure makes the two tiles' values equal; the guard lifts it to trick-plus-count because the transport moves only two zero-point tiles. | same file |
| **Proposition J-win** | D1-win — count-free only, and **this is where the guard fails**: focal winning tricks changes which of the *other* seats' count tiles fall where. | same file |
| **Lemma E8** | The exact valuation scope of J-0/J-1: w constant on the exchanged tiles makes the feature difference vanish, always — and is also necessary in the generic case. Gauge-stable, hence a condition on the valuation class. | errata §8.5 |

### Decision-sparse witnesses — [page](walt-math-decision-sparse.md)

| Object | One line | Full statement |
|---|---|---|
| **Theorem E1** + **Corollaries E1.1–E1.2** | Order exchange under a declared involution Θ. Repairs parent §7.1, which is **unsound as written**. E1.1 is the J-1 instance; E1.2 is count survival under the guard. | errata §1.3, §1.6 |
| **Theorem E1′** | E1 generalised to a fully transported involution (Θ_T, Θ_M, Θ_X), adding continuation equivariance (H4′) and utility invariance (H5′). | errata §8.1 |
| **Definition E2** + **Proposition E2.1** | d_adv as the **affine** dimension; the reference-based form is off by exactly one when the reference lies outside the affine hull. | errata §2 |
| **Lemma E3** + **Remark E3.1** | The action-conditioned upper witness U_a = E_β[V*_a] ≥ Q^H(a). The unconditioned aggregate is valid but **action-constant**, hence vacuous for separation. | errata §3.2–3.3 |
| **Corollary E3.2** | Zero global fusion gap ⇒ U_a = Q^H(a) = V^H for every H-optimal a, and U_a ≤ V^H for all a. At such a coordinate the whole remaining difficulty is primal. | errata §8.2 |
| **Lemma E4** + **Non-theorem E4′** | The primal witness L_a ≤ Q^H(a), and the inversion failure mode with a two-world witness showing the separated action can be **strictly worse** than the rejected one. | errata §4 |
| **Corollary E4.1** | The primal ceiling: an H-argmax-seeded candidate gives L_a = Q^H(a) exactly, for every tie-break; and if Q^H(a⋆) < U_a then **no candidate set whatsoever** separates that pair. | `CENSUS-RULINGS.md` § "Experiment E adjudication"; to be filed as errata §4.3 |
| **Proposition E5** + **Lemma E5.0** + **Corollaries E5.1–E5.2** | The Scheme-mass closure is degenerate in this game (atoms are singletons); (S) holds on the measured carrier; the binding negative is "**atom-mass linear filtering is noncompressive on this carrier**". | errata §5 |
| **Theorems E6.1–E6.5** | The parent's §§4.3, 6.3, 8.3, 8.4, 9.2 restated with their load-bearing hypotheses **inside the statements**: width monotonicity, backward pruning, the value sandwich, root-action separation (member-not-set), finite adaptive gluing. | errata §6 |
| **Lemma E7** | When dominance travels: an exhibited value-order isomorphism α_{Tρ}(Tξ) = α_ρ(ξ), plus a transported belief for belief-relative verdicts. | errata §8.3 |

### The nonanticipativity ladder — FT and SR — [page](walt-decision-sparse.md)

Sixteen objects delivered in one day, adjudicating two received external notes.
**All sixteen live only in `CENSUS-RULINGS.md`**: none has an errata number, so
the DS-A17 citation rule cannot yet be followed for any of them. **Fifteen of the
sixteen are queued for the errata §9 amendment** — Corollary FT-grade4 is named
in none of the queueing rulings. Their compact statements and the caveats that
travel with them are on the track page's two chapters.

| Object | One line | Full statement |
|---|---|---|
| **Lemma FT-arrive** | The arrival law at the *first* frontier is policy-independent, with an explicit inverse-legal-set product form. **Fails at depth two and below** — which is why the ladder is not a decomposition of the value. | `CENSUS-RULINGS.md` § "The fusion tax: inbox 016 adjudicated" |
| **Lemma FT-trunc** + **Corollary FT-grade4** | A suffix of forced decisions truncates the reveal-delay ladder; since the last focal decision is always forced, the last tax is identically zero. At grade 4 the ladder has **exactly two rungs**, so (Δ¹, Δ²) is the complete decomposition. | same section |
| **Proposition FT-flat** | An upper feature independent of the **frontier** action returns a bound at least U^C, so it can never improve the filed witness. The upper twin of Proposition T1-blind. | same section |
| **Proposition FT-tie** | A tied competitor closes **only** if the relaxation is exact — an all-or-nothing threshold. A fence on the reading, filed before the run. | same section |
| **Lemma FT-post** | The frontier posterior is legal-set weighted, **not uniform**. A residual witness priced as a fresh uniform coordinate prices the wrong measure and the composition is **void, not loose**. Two forms compose; nothing else does. | same section |
| **Corollary FT-conv** | Taxes scale with the valuation convention and verdicts do not; a differential tax is exactly twice its count value, and cross-convention comparison is void. | same section |
| **Lemma FT-mix** | Heterogeneous upper witnesses compose: the competitors' witnesses need not share a relaxation, an evaluator, a traversal or a run. Licenses a **mixed** verdict without stretching Theorem E6.4. | same section |
| **Lemma SR-coord** | Theorem 4.1's unnamed hypothesis, discharged for this engine: first-frontier states are distinct **information states** (not histories), so a lawful first-stage policy ranges over a free product; and a second-frontier state has a **unique parent**. | `CENSUS-RULINGS.md` § "The second rung: inbox 017 adjudicated" |
| **Lemma SR-forced** | Gluing a forced decision is free, and the two ladder indexings — by decisions, and by *nontrivial* decisions — agree. Fixes the counted-not-skipped convention. | same section |
| **Proposition SR-sep** | The policy-level minimum separates into the local formula because the first-stage policy ranges over a free product — **not** because of mutual exclusivity and fixed arrival, which do a different job. | same section |
| **Proposition SR-post** | The backward rung recursion is **occupancy-free**: everything below the first frontier enters through the policy-independent lawful posterior, and occupancy enters exactly once, at the first frontier. | same section |
| **Corollary SR-conv** | FT-conv at rung two, with the two bridges kept separate: a *difference* is exactly twice its count value, a *p-weighted value* maps affinely. Collapsing them is the failure mode most likely to look like a discovery. | same section |
| **Proposition SR-degen** | At grade 4 the second rung closes every binding pair **unconditionally**, strictly exactly at the untied ones. **No grade-4 experiment can test closure.** | same section |
| **Proposition SR-taut** | Which depth-two identities cannot fail. The two structural assertions of the received verifier are identities in its own recomputed quantities and hold for any input whatsoever — arithmetic remarks, never receipts. | same section |
| **Proposition SR-loc** | Escape is exactly where the safety inequality is strict, so the escape set is precisely the support of a naive witness's error and the error is the sum over that support **and nothing else**. | same section, closing note |
| **Proposition FF-blind** | An **action-blind fee** removes exactly zero fusion value, at every state and for every coefficient. The penalty-side twin of FT-flat and T1-blind. | `CENSUS-RULINGS.md` § "The feature-fee audition: Jason's control feature, specified" |
| **Lemma FF-min** | The fee objective is convex piecewise-linear, bounded below, and **exactly minimisable** by enumerating breakpoints — no grid, no search, no float. Supplies two genuine receipts, since the swept minimum and the filed tax are different computations. | same section |
| **Proposition FF-oracle** | Per-state fees bound shared fees, so **a low capture refutes conclusively and a high capture establishes nothing** about a usable family. Forces the name **oracle-θ capture** on every such column. | same section |
| **Proposition FF-degen** | **Zero breakpoints is exactly vacuity.** A zero capture with many breakpoints is a measurement; the same zero with none is a tautology. The diagnostic that made a defect catchable from a committed artifact. | same section, first closing note |
| **Proposition FF-corr** | Exactly what a fee bites on: capture is zero iff zero minimises the objective, which for unique clairvoyant argmax means the centred feature has zero mean along the clairvoyant policy. | same section, second closing note |
| **Proposition FC-drop** | The quantitative form of FF-corr: **capture is at least correlation times reach** — one directional slope times one breakpoint distance, computable with **no minimisation**. A **lower** bound at every state, so a large value proves a fee bites and a small value proves nothing. | `CENSUS-RULINGS.md` § "The fee-correlation chapter: what a fee bites on, measured" |
| **Corollary FC-null** | An action-blind feature has exactly zero correlation, recovering FF-blind and giving the diagnostic a null control whose value is fixed **by theorem** rather than by a filed number. | same section |
| **Proposition FC-width** | The subgradient's width is exactly the mass-weighted spread of the feature **across the clairvoyant tie**. Without ties it is a point, so zero capture needs an exact identity; with ties it has positive width and zero capture is **robust rather than coincidental**. | same section, closing note |
| **Proposition FC-tight** | The drop bound is **attained** exactly when the descent is a single linear piece. It is never *exact* as a property of the functional anywhere, and **which states attain it is not knowable without the captured amount** — so attainment is a fact about the gap's distribution, never a usable property of the instrument. | same section, closing note |

### GPU-native trick-1 portable foundation, M2 Metal parity, and the M3 gate — [page](walt-gpu-native-trick1.md)

| Object | One line | Full statement |
|---|---|---|
| **`OpeningRootV1`** | The only accepted first slice: focal = bidder = leader = actor, seven known tiles, empty public prefix, complete ordered hidden 7/7/7 support, derived point/mark loss budget, and closed evidence/prior/field/utility/horizon profiles. It is not a generic public-state API. | `GPU-NATIVE-TRICK1.md` §§2–4; GT1-A2/A3 |
| **Opening-response projector** | Exact `(response,e)` cells with separately typed `A`, `C` and `W=A*C`; exact `m=0..6` cell counts, hard maximum 11,730, and total scaled mass `399072960*420^3`. Same-context payload reuse never merges physical action identity. | same file §5; GT1-A4 |
| **Reduced parity carrier** | Complete direct-world parity at grades 2–4 below the 100,000-world cap; grade 5 is a 756,756-world preflight declared stop with zero partial output. This is a correctness carrier, never opening evidence. | same file §9 M1; GT1-A5 |
| **Portable receipt boundary** | Persist only a root/action/profile/table/freeze/build-bound envelope or a fully bound grade-5 stop. Raw projector payloads are non-persistable; validators canonicalize and fail closed. | GT1-A6 and freeze 55 |
| **Lean GT1 foundation** | Kernel proofs for the legal loss bound, live legal-set/420 facts, numeric widths, current-trick-aware unbanked points, exact cell counts and the stable interval algebra. Projector refinement, posterior factorization, information-key equivalence, Rust/Lean correspondence and Metal/Rust correspondence remain explicit debt. | `lean/Texas42/Trick1Foundation.lean`; GT1-A8 |
| **`U256MetalCorpusV1` and `OpeningChooseTableV1`** | A fixed 16,384-case edge/SplitMix arithmetic corpus with an independent BigUint oracle, plus a separately identified 22-by-22 extraction checked entrywise against unchanged `SemanticTablesCanonicalV2`. These are exact arithmetic/table evidence, not game values. | `GPU-NATIVE-TRICK1-M2.md` §§3–4; GT1-A12 |
| **`M2OpeningParityCarrierV1`** | The complete ordered Reduced, GradeMatching and SameContextPair carrier. One official work unit is one validated opening context and command buffer; physical-action binding remains distinct from reusable context payload. The complete official run has 614 tasks. | same file §§5–7; GT1-A11/A13 |
| **`M2SequentialRunnerV1`** | Completion-only reads, fixed arenas, stable host compaction, typed progress frames, separate CPU/command watchdogs and zero accepted evidence after any failure. It deliberately admits no atomics, concurrency, adaptive scheduling or performance conclusion. | same file §§6–10; GT1-A13..A16 |
| **M2 receipt boundary** | Two fresh complete Metal runs must produce one another's bytes and the immutable committed comparand exactly. The receipt is executable evidence under freeze 56, never a Lean theorem or a reusable projector value. | `walt/receipts/gpu_native_trick1_m2_v1/`; GT1-A16/A17 |
| **Lean M2 finite foundation** | Kernel proofs for the fixed arena bounds, GradeMatching coverage and count, at-most-ten matching-vector bound, stable-filter order, and all-or-nothing acceptance. The projector formulas and Rust/Lean and Metal/Rust correspondence remain proof debt. | `lean/Texas42/Trick1MetalFoundation.lean`; GT1-A17 |
| **M3 carrier and claim fence** | One grade-4 h8 carrier immediately before trick 4, uniform over the exact 1,200 compatible worlds, roots 21/31/33/55, objectives M3A (future-trick differential, strict `C > H` required) and M3B (P30 make), treatments H lawful-perfect-recall vs C world-revealed. The sole green sentence is the gate sentence; no trick-1 value, lead, performance, controller or player claim. | `GPU-NATIVE-TRICK1-M3.md`; GT1-A19 |
| **`M3PerfectRecallKeyV1` / `M3WorldRevealedKeyV1`** | The scoped S1 observation with complete own action-observation memory and no hidden-world identity, vs the disjoint world-revealed C type. Unique parent/action, complete face retention, sum-before-max license the exact unnormalized recurrence; no strategy fusion, key renormalization or cross-world C pooling admitted. | same file; GT1-A20 |
| **The two-family reduction algebra** | Exactly two noncoexisting REDUCE families (`MASS_BUCKET`, `BACKWARD_VALUE`); conservation and terminal-bucket checks are disjoint host folds, never a third family; count-one retirement, real epoch order, the 21-level/range proof and closed command/frame/byte caps. | same file; GT1-A21 |
| **M3 evidence, controls and reproducibility** | Sole-owner semantic streams rendered independently on CPU and post-Metal; the 36-control registry; closed receipt grammars with no partial salvage; two fresh builds and two fresh runs must produce byte-identical library and receipt bytes. | same file; GT1-A22 |
| **Lean M3 proof boundary** | `Texas42.Trick1PerfectRecallNet` must build and pass the axiom audit (codec/scoping, replay, unique parent, complete face, sum-before-max, mass, objective bridges, two-family census/range/compaction/cap proofs, all-or-nothing composition). Rust-to-Lean, Metal-to-Rust, general oracle correctness and grade-4-to-trick-1 transport remain named correspondence debt. | GT1-A23 |

### The scenario-player era — signed pivotal geometry, the tilt audit, and the level-2 detector

The era's mathematics lives in four documents rather than one probe chapter:
the received signed-pivotal parent and its intake companion, the
`walt/SCENARIO-PLAYER.md` spec, the pmake advisory ruling, and the two probe
documents (`walt/TILT-AUDIT.md`, `walt/LEVEL2-PROBE.md`). Artifact provenance
is on [received artifacts and intakes](walt-math-intakes.md); the track
narrative is on [walt-seat-play](walt-seat-play.md). **A status column is
mandatory here and nowhere else on this page**, because this era's named
objects are *mostly unproved*: the spec is explicit that its statements are
definitions and proof obligations, not established results, and each obligation
carries a ledger row.

| Object | One line | Status | Full statement |
|---|---|---|---|
| **The signed-pivotal boxed identities** | g = qτ; E[Y²] = q; Var(Y) = q − g²; H = 1/(qτ²) − 1; the world/tape projection; strata linearity g = Σ_j w_j μ_j; the cover identity w²·Var(Y\|P) = wq − g², hence H_P = w/(qτ²) − 1. | Verified by hand and on 2,000 random exact-rational instances at intake; SOUND at SP-A audit | `walt/math/signed_pivotal_geometry_v0.1.md` §§2, 4, 5, 6; intake companion; SP-A headline |
| **The SP-A5 paired-variance repair** | Var(Y) = Var(u_a) + Var(u_b) − 2·Cov(u_a,u_b): pairing is sharper **iff Cov > 0**. The parent's §2.1 "strictly sharper" is the corpus's one general claim FALSE as written; its own Case C is the counterexample and stays in the parent. | Ruled; binds every consumer | `CENSUS-RULINGS.md` § "Signed-pivotal intake adjudication", SP-A5 |
| **The adopted vocabulary** | **Pivotal mass** q, **tilt** τ, **gap** g, **fixed-pair hardness** H, **scenario** ξ = (ω, r), **panel**; **pivotal cover** (never "envelope"), **pivotal win share** (never bare θ), **frozen policy** (never "plan"). | Ruled | SP-A1..A4 |
| **SP-A6 tape typing** | The tape r is a seed assigned world-independently; the scenario law is the product law. Walt's single-stream derivation does **not** satisfy the split, so d(ω)/s(ω) estimates are undefined artifacts until it exists. | Ruled; the split is unbuilt | SP-A6 |
| **The no-tape structural finding** | The level-0 mind is a pure function of (seat, hand, record) — under the current field model a scenario IS a world, every world is tape-stable by construction, and Phase E is vacuous until a stochastic field model exists. | Smoke finding (2026-08-19), a measurement about the code, not a lemma | `walt/TILT-AUDIT.md` § "Smoke results" |
| **The spec's object definitions** | Public record R (2.1), information state I_s (2.2), solver key κ(R) (2.3), PiKey (3.1), level-0/level-k minds (3.2/3.3), seed discipline (3.6), fiber (4.1), outer sampler with its inline sampler-correctness obligation (4.2), the declared no-void inner simplification (4.3), level-k walt (6.1), objective and decided cutoffs (6.2). Plus two load-bearing remarks: **2.5** — banked is NOT a function of (played, leader, plays), the mathematical content of the PiKey defect; **4.4** — beliefs are lawfulness-only today, behavior-Bayes happens only on modeled continuations. **Numbering caveat (2026-08-24):** the sampler-correctness lemma is *unnumbered* in the §4 body (it sits inside Def 4.2) but the §10 ledger's O3 row cites it as "Lemma 4.2" — both readings are indexed here; neither text is edited. | Definitions; the sampler-correctness lemma is Obligation O3, the no-void cost Obligation O5 | `walt/SCENARIO-PLAYER.md` §§2–6 |
| **Lemma 2.4 (key sufficiency)** | Under the Boolean pmake objective with fixed dcl and b, the continuation value depends on R only through the reduced key κ(R) plus the alive set. | **OBLIGATION O2 — unproved on paper**, heavily exercised | `walt/SCENARIO-PLAYER.md` §2 |
| **Def 3.4 (cache purity invariant)** | Every π value cached under (k, PiKey) must be a pure function of that key — which requires the key to carry everything the computation reads, banked totals included. Carries the PiKey defect record (the documented invariant the code violated for a day). | Invariant + receipt on `f5fff91`; never an axiom | same file §3 |
| **Def 3.5 (Dice field / the tickertape)** | Field randomness keyed on the *record*, not the path: the same world at the same record plays the same tile in every branch, so worlds partition by drawn move instead of multiplying branches. | Definition | same file §3 |
| **Lemma 5.1 (conservation)** | The move buckets partition the alive set exactly. | Asserted at every node in every run | same file §5 |
| **Lemma 5.2 (posterior semantics)** | The bucket weight is the posterior probability of the bucket given the modeled move; the root value is the best-response expectation under "field seats play their modeled policies". | **OBLIGATION O4 — the load-bearing one**, unproved | same file §5 |
| **Lemma 5.3 (support safety at the bottom)** | Under Dice every legal move of every world has positive probability, so the level-0 bottom excludes no lawful world; deterministic higher levels refine support intentionally per 5.2. | Argued in place | same file §5 |
| **Def 6.3 (tie protocol)** | Saturation ties are never broken by tile index: tied candidates re-evaluate on fresh 4× samples until separated or bounded — support ≠ belief, and 1-on-sample is not certainty. | Definition; bias question is Obligation O8 | same file §6 |
| **Claim 8.1 (execution-order invariance)** | Cache purity + exact rationals + fixed argmax order + partition semantics ⇒ results invariant under any thread count and interleaving; only work statistics vary. | **Theorem-shaped claim, OBLIGATION O7**; byte-identical 1-vs-18-thread receipt | same file §8 |
| **The obligations ledger** | O1–O9 (spec-native: no-strategy-fusion, key sufficiency, sampler correctness, posterior semantics, no-void cost, sampling error, order invariance, tie bias, bid generalization), O12–O19 (filed from the signed-pivotal §14; O10–O11 permanently retired, SP-A11), **O20–O28** (calculated evidence, accepted at CE-A4) and **O29–O38** (level-2 field stability, accepted at L2-A2). | The graduation queue; nothing graduates by existing | same file §10 |
| **The pmake verdicts P1–P4** | Decided cutoffs SOUND (with the totality bonus: at any terminal T1 + T0 = 42 forces a cutoff, so the recursion is total with no explicit terminal case); viewer early exit SOUND; pmake key reduction SOUND (and unsound for trick-differential, as claimed); gcd-normalized projective posteriors SOUND-WITH-CAVEAT (fail-closed overflow), exact by the projective lemma V(R, c·w) = V(R, w) for integer c ≥ 1. | Advisory, recorded outside the rulings file; against `ladder.rs` at 171cd22; no ruling family | `walt/math/WALT-MATH-RULING-2026-08-17-pmake-and-the-walk-to-trick-1.md` |
| **The path-dependence counterexample** | The exact posterior is a function of the full ordered record but NOT of the reduced boundary key: same reduced key, odds 1:1 on one trick order vs 3:4 on the other. The exact-posterior key is not redundant. | Advisory, concrete counterexample verified in the ruling | same file, "Bonus" |
| **The §12.6A invariance-lemma instance** | The concrete ECL instance the ladder should quotient by: (π, θ) fixing played tiles and the focal hand, transporting contexts, preserving trick-key order and count, gives V and Q preserved tile-for-tile — the v0.5 theorem's tile-feature role re-entry form. Honest negative: on the hand-8 carrier the group is provably almost trivial. | Advisory; proof shape stated, group computation hand-checkable | same file, §Q2; parent `walt/math/equivariant_lumpability_v0.5.md` |
| **The allowance-automaton coarsening** | Under pmake the banked pair collapses to the T0-allowance state a = 12 − banked_T0 ∈ {0..12} ∪ {busted}; one ≤14-valued coordinate where trick-differential needs the exact tally pair. | Advisory | same file, §Q2 close |
| **The minimal-sufficient-statistic negative** | Under a uniform-random field, worlds act only through hidden legal sets and distinct posteriors at equal public reductions have genuinely distinct continuation laws — the posterior is the minimal sufficient statistic, and **no coarser belief relation should be hunted**. The belief-class count is the irreducible size. | Advisory; the standing negative that shapes the cell-representation recommendation | same file, §Q2 close |
| **The racing instruments** | Three named modes from the tilt smoke: **replay-race** (frozen policies on a common panel — the audit-faithful object, replay ≈ re-solve without extraction), **block-race** (`level1_raced`, CRN blocks with paired sign-test elimination), **race-then-refine** (`level1_race_refined`, wired opt-in). Regime-dependent: pays off where evaluation is expensive and candidates separate; fixed-bid-30 self-play is its worst case. | Smoke measurements and engineering verdicts, never receipts | `walt/TILT-AUDIT.md` §§ "Racing", "Arena gate" |
| **The level-2 detector** | A decision is level-2-relevant exactly where pivotal mass wakes up under a field upgrade: q(level-0 field) ≈ 0 but q(level-1 field) > 0. Companion hypothesis: modeling the partner's response should grow g and drop H — level 2 may be *cheaper to sample* at equal confidence. | Amended 2026-08-24 (CE-A6 three-way wake-up split + 𝓘 coordinate; L2-A5 detection layer); **step 9 ran 2026-08-25** (records `walt/probes/step9/`; the wake lives in the value and decision channels — Chapter 8) — see [walt-calculated-evidence](walt-calculated-evidence.md) | `walt/LEVEL2-PROBE.md` |

### Inline mathematics outside the probe chapters

Named or boxed mathematics that lives in a basis document, a design document,
or an era page rather than a ruling chapter. Rows marked **orphan** have no
ruling-ID home anywhere — they are indexed here so they stop being findable
only by accident, and indexing confers nothing.

| Object | One line | Home | Full statement |
|---|---|---|---|
| **The §12.6A theorem and condition (ECL)** | Jason's equivariant-controlled-lumpability theorem over declared role interfaces: the lossless count-free equivariant quotient, with role re-entry of tile features (count, trick-key order) as the lawful route back for non-count-free payoffs. | The v0.5 basis; concretely instantiated at the pmake ruling's Q2 lemma | `walt/math/equivariant_lumpability_v0.5.md`; canonical prose form at [the factory era](walt-factory-era.md) § "S5d — the re-tethering". **Dated caveat (2026-08-24):** two pages paraphrase it imprecisely — [walt-program](walt-program.md) §3 drops count-freeness and the legal-set/kernel conditions, and [the census era](walt-census-era.md) § "What the era left" reverses the gauge ordering (the §8 additive gauge acts only *after* the role-indexed valuation interface is declared). The v0.5 source and the factory-era prose govern; neither page is edited by this index. |
| **Walt's Appendix A reader notes** | Two walt-authored caveats on the v0.5 basis: coherence scope over the transports, and the abstract-policy-class optimization boundary. | **Orphan** — no ruling home | [the factory era](walt-factory-era.md) § "S5d — the re-tethering" |
| **The v0.4 objects as a user guide** | Purpose-soundness D(x) = D(y) ⇒ R*(x) = R*(y) (R* = R̄ ∘ D); strong controlled lumpability's two conditions; the Scheme/Fix grammar (Σ = (N_Q, N_C, N_D), O ⊆ Σ, S = (π, φ), F = S₁ ∨ … ∨ S_r, empty Fix false); §12.7's six conditions and its boxed three-part deliverable. | The v0.4 basis (its own §17 claim ledger governs status) | `walt/math/unified_information_geometry_v0.4.md`; worked restatement at [walt-scheme-fix](walt-scheme-fix.md) §§2–3, 7 |
| **The slack identity** | a⋆ separates iff g(a⋆, seed) ≤ s(a⋆), with the economy gap g := Q^H(a⋆) − L^seed(a⋆) and the separation slack s := Q^H(a⋆) − max_{a≠a⋆} U_a, proved in the design. | EC-A3 ("EC-Q3: R8 CONFIRMED"). Collision hazard: an unrelated receipt also named (R8) sits in the N4 section — the EC-A3 citation governs here | `walt/ECONOMY-SUCCESSOR.md` §1.2 |
| **Definition (walk-step)** | One unit charged as `bag.len()` at each `walk` entry — one unit per (particle, node) visit, deliberately the same unit as the scalar authority's particle-step. | Freeze 44 ([register](walt-math-freezes.md)) | `walt/SEPARATION-RUNG-N4.md` §3.1 |
| **The convention bridge Q_diff = 2·Q_count − grade** | The exact affine bridge between the differential and count conventions, asserted at reporting boundaries only; generalised in the rulings as the case α = 2, c = −grade. | Freeze 26 content, ratified at SEP-A3(iii)/SEP-A8 | [the register](walt-math-freezes.md), row 26; restated in `walt/SEPARATION-PROBE.md` and `walt/SEPARATION-RUNG-N4.md` |
| **The cardinality ladder** | N_pol ≥ N_vec ≥ N_par ≥ W_all ≥ W_reach ≥ 1, with forced ⊂ decision-dead (N_vec = 1) ⊂ dominant (N_par = 1) and the W_all = N_exp identification. | Owned by the information-geometry page | [information geometry](walt-math-information-geometry.md) § "The cardinality ladder" |
| **The rank reconciliation** | v0.4 §1.3's pip-sum order and the rules corpus's off-pip ranking are the same order: ranks compare only inside one tier, a tier fixes one context q, and pip_sum = q + off_pip is monotone in off_pip; with the doubles-sentinel argument. | **Orphan** — argued in place, no ruling ID | `walt/DISCREPANCIES.md` § "Rank of a mixed tile" |
| **The `q_points` class definition** | An exact PI root value vector under the real scoring differential — each trick worth ±(1 + count points of its four tiles), focal minus opponents. | **Orphan** — definition in place, no ruling ID | `walt/DISCREPANCIES.md` § "exp5 census pins"; restated at [the foundation era](walt-foundation-era.md) § "S3.5" |
| **The decisive-tile rule** | Decisive tile = the viewer tile whose led context touches the most hidden-pool tiles, ties to the higher tile. Explicitly typed **a choice, not a theorem**. | **Orphan** — declared in place, no ruling ID | `walt/DISCREPANCIES.md` § "exp3A descriptor pin"; [walt-scheme-fix](walt-scheme-fix.md) §5 rule 1 |

### The lineages after 2026-08-24 — where their objects are indexed

The object index above stops at the 2026-08-24 sync. The later lineages are
indexed by idea in the chapters, one row per named object with its ruling and
full-statement pointer, so they are not repeated here:

| Lineage (parent; rulings) | Objects | Indexed in |
|---|---|---|
| Panel response (x:019–023; PANEL-A1..A8) | Claim D's 169/512 counterexample and the future-only rule; W7–W11; τ coupling; the cancellation ladder `|c| ≤ r ≤ d`, pairwise `(B, H, q, g)`, the dominance theorem, directional bounds R± | Chapters 7, 8 |
| Deferred producers (x:024; TRIPLE-A1..A7) | Theorem M1 / Corollary M2 (the max-preserving upper CS); the Hazard-Exclusion Invariant; the six-motif alphabet | Chapters 7, 8 |
| Counted belief (CBS-A1..A9) | Theorems 2.1 (root interval / survivor set), 5.1, 9.1 (cylinder partition, the §12 grammar), 17.1, 20.1 (seat-factor posterior closure), 23.1 (the factorised recursion), 30.1 (refinement safety); the cross-history cache negative | Chapters 2, 5 |
| Anytime proof state (APS-A1..A9) | The 43-bin profile and tail-sum identity; `W_ρ(c)`; rescue/fragile bands; count-threat covers; the envelope fence (63/2); the typed laydown hierarchy; proof bar / executable bar; certified regret Γ; the §49 proof state; freeze 58 | Chapters 4, 5, 9 |
| Model belief (MB-A1..A8) | The rung registration `D, F₀ = σ0, F₁, F₂`; Theorem 7.1 (`Ξ = Ω × Θ`); persistence; the residual Other type; the model-fusion price Φ; Theorem 19.1's corollary | Chapter 8 |
| Salvation complex (SC-A1..A8) | Theorem 6.1, Theorem 7.1 (God-tightness), Theorem 12.1 (the transversal), the §13 split, §14/§15 (conflict families, rational packing), the §23/§26 counterexamples, the §8 decomposition, the fusion-free-suffix hypothesis, `UnknownGodGap` | Chapters 4, 6 |
| Focal horizon (FH-A1..A11) | Theorems 1–6; the action interval, bar, survivor set, focal depth; FH-God, FH-int, FH-tie, FH-cut, FH-last; the trivial-upper refusal | Chapters 5, 6 |
| The unified player (UP0/UP1a; no parent) | The five-tier cascade, provenance, the lazy carry, the (b)/(c) question | Chapter 1; [open questions](walt-math-open-questions.md) item 22 |
| The 6-4 readout (no parent) | Exact indifference at pmake = 1; `TieRule::LowestTileIndex`; the release margin | Chapter 4; [walt-gran-anchors](walt-gran-anchors.md) |

---

## Appendix C — The ruling families

Thirty-five indexed families or ruling series, one adjudicator (walt-math),
all exploratory: the 27 rows below through `L2-A` (the 2026-08-24 edition
said "twenty-six" against 27 rows; the Definition/Q2/Q3 shape row counts as
one) plus the seven families that followed it, none of which carries a
range-close marker. Ranges below are
**ruling-ID ranges**, which are append-only and do not move; the sections are
located by heading, never by line number, because line numbers drift with every
append. Three families span more than one section heading — `DS-A` runs across
three, `N4-A` across two, `GT1-A` across three — and the family, never the
heading, is what inherits.

| Family | Range | Section | Date | Scope |
|---|---|---|---|---|
| `F1`–`F7` + "Extra item" | F1..F7 | Census fork rulings | 2026-08-10 | The census fork: carrier, invariant list, transports by canonicalization, probability model, primitive-step granularity, quotable statistics, the **NO-RESCUE failure protocol (F7)**, and the empty output interface. |
| r3 `Q1`–`Q5` | Q1..Q5 | r3 — retrograde coarsest quotient | 2026-08-10 | The coarsest equivariantly-lumpable quotient: soundness and coarsestness, successor-class equality, the signature tuple, intrinsic vs carrier-relative reading. |
| `Y1`–`Y3` | Y1..Y3 | The railyard factoring | 2026-08-10 | Level = tricks remaining: one-trick contract and stacking, periodicity split into obligation vs measurement, the pruning operator. |
| `Definition`/`Q2`/`Q3` | — | Shape notion v2 | 2026-08-10 | The repaired instrument: the depth-*d* suffix library in two variants, the refutation criterion, the hereditary-shape rung. |
| `P-A` | P-A1..**P-A21** | Fiber-probe rulings | 2026-08-11 | The fiber probe. Carries **Lemma V**. |
| `X-A` | X-A1..**X-A19** | Fiber-refinement rulings | 2026-08-11 | Declared exclusion remnants. Carries **Lemma X** and the support/belief/exclusion typing. |
| `E-A` | E-A1..**E-A21** | Endgame-store rulings | 2026-08-11 | The symmetry-reduced tablebase. Carries **Lemma E** and **E-A2**, the count-free scope limit. |
| `S-A` | S-A1..**S-A21** | Seat-census rulings | 2026-08-11 | The seat-level census, counts only. Carries **Lemma S**, **S-rigid**, **S-fold**, **S-det**. |
| `R-A` | R-A1..**R-A24** | Predictive-rank probe rulings | 2026-08-12 | Predictive dimension. Carries the v0.6 proof audit, **Lemma R**, **R-fold**, and **R-A18**, the correctness gate. |
| `PG-A` | PG-A1..**PG-A18** | Policy-geometry probe rulings | 2026-08-12 | Policy counts. Carries **Proposition G-flat**, **Lemma G**, and **PG-A13**, the stop discipline. |
| `J-A` | J-A1..**J-A18** | Decision-deadness probe rulings | 2026-08-12 | Deadness detectors. Carries **Lemma J**, **J-0**, **J-1**, **J-win**. |
| `DS-A` | DS-A1..**DS-A36** | Three sections: Decision-sparse intake audit; Second-audit adjudication; S6c runner | 2026-08-13 | One continuously numbered family. Intake audit of the received v0.1 document (A1–A18); adjudication of the second audit (A19–A28, including **DS-A28**, the append-only protocol); execution scheduling and persistence (A29–A36). |
| `SEP-A` | SEP-A1..**SEP-A19** | Experiment E adjudication: the separation probe | 2026-08-13 | Root-action separation by primal and upper witnesses. Carries **Corollary E4.1**, freezes 36 and 37. |
| `N4-A` | N4-A1..**N4-A20** | Two sections: The n = 4 separation rung adjudication; The n = 4 rung return: the overnight pass | 2026-08-13 / 2026-08-14 | One continuously numbered family. The SEP-A10 successor rung (A1–A12), carrying **freeze 44** (the walk-step unit and budgeted-walk contract, now at **v2**) and **freeze 45** (the n = 4 coordinate identity); then the authorised overnight pass (A13–A20), carrying **Lemma N** and Corollaries N-1..N-3 and the raised `P_max v2`. |
| `EC-A` | EC-A1..**EC-A14** | The economy-successor adjudication | 2026-08-13 | The primal half of the economy claim. Carries **Corollary S-fold-val**, **freeze 46** (the closed arm list), **freeze 36 v2** (EC-A8, transport opened to the declaration fold), the 384-versus-108 typing (EC-A12), and the primal/full split (EC-A13). |
| `T1-A` | T1-A1..**T1-A12** | The trick-1 witness: the bounded sandwich, refuted and replaced | 2026-08-14 | The first-trick target. Carries **Lemma T1-run**, **Lemma T1-force**, **Proposition T1-blind** and **Proposition T1-corner** (the refutation, itself a filed result), **Theorem T1-draw**, **Corollary T1-ruff**, and **freeze 47** (the trick-1 carrier). **T1-A12 is the implementation-versus-corpus risk** and is inherited by everything below it. |
| `LD-A` | LD-A1..**LD-A13** | Lay downs: the characterization, and the four-laydown question | 2026-08-14 | The family term made exact. Carries **Theorem LD** ((L1) ∧ (L2)), **Corollary LD-fold**, **freeze 48** (the lay-down catalogue), the settled four-laydown question (LD-A11), and LD-A10(ii), which carries T1-A12's risk forward sharpened. |
| `RW-A` | RW-A1..**RW-A8** | The map-free rule walk, and what h9 already decided | 2026-08-14 | The rule-economy probe at the n = 4 carrier. Carries **freeze 49** (the n4 economy carrier), the closed rule argument list `(record, legal)`, RW-A3's label pair (NOT PRICED / RULE-EVALUATED, never merged), and h9's coordinate verdict filed from the S6h numbers alone. |
| `FT-A` | FT-A1..**FT-A29** | The fusion tax: inbox 016 adjudicated | 2026-08-14 | The upper side. Carries **Lemma FT-arrive**, **Lemma FT-trunc** + **Corollary FT-grade4**, **Proposition FT-flat**, **Proposition FT-tie**, **Lemma FT-post**, **Corollary FT-conv** and **Lemma FT-mix**; **freeze 38 v1** (FT-A17, the reservation discharged) and **freeze 50 v1.1** (FT-A18, amended at FT-A23 and FT-A24). The closing notes (A23–A28) adjudicate the returned run; **FT-A29** files two self-corrections to the section. |
| `SR-A` | SR-A1..**SR-A37** | The second rung: inbox 017 adjudicated | 2026-08-14 | The upper side, one rung deeper. Carries **Lemma SR-coord**, **Lemma SR-forced**, **Proposition SR-sep**, **Proposition SR-post**, **Corollary SR-conv**, **Proposition SR-degen**, **Proposition SR-taut** and **Proposition SR-loc**; **freeze 51** (SR-A22, the depth-two carrier) and **freeze 38 v1.1(d)** (SR-A21, a clarification — v1 not amended, v2 not opened). The closing note (A27–A32) adjudicates the returned run and discharges FT-A28 entire; **A33** and **A34** adjudicate two defects the build found in itself, **A35** types the companion's cross-process digest as an audit note, **A36** records the chapter's first pass with no specification conflict, and **A37** withdraws a carried obligation that was never owed. |
| `FF-A` | FF-A1..**FF-A33** | The feature-fee audition: Jason's control feature, specified | 2026-08-14 | Which cheap structural features price the first-layer tax, on a carrier where the perfect answer is filed. Carries **Proposition FF-blind**, **Lemma FF-min**, **Proposition FF-oracle**, **Proposition FF-degen** and **Proposition FF-corr**; **freeze 52** with amendments **v1.1** (FF-A15), **v1.2** (FF-A20), **v1.3** (FF-A23) and **v1.4** (FF-A33). Two closing notes: A10–A24 adjudicate the first run and the defect it exposed, A25–A33 the corrected re-run, the shared-θ fit, and the chapter's close. |
| `FC-A` | FC-A1..**FC-A23** | The fee-correlation chapter: what a fee bites on, measured | 2026-08-14 | Why a fee bites, measured where the exact answer is filed. Carries **Proposition FC-drop**, **Corollary FC-null**, **Proposition FC-width**, **Proposition FC-tight** and **freeze 53**. Delivers the branch's first **pre-fee screening statistic** and its first structural limit on the fee route. A22 binds the "attained, never exact" phrasing; **A23 closes the range and replaces adjective-led claims with the exact one-sided reading**. |
| `SS-A` | SS-A1..**SS-A18** | The seed survey: a hundred fresh coordinates, designed | 2026-08-15 / 2026-08-16 | The outcome-independent 100-seed, 400-unit grade-4 survey. Carries **freeze 54**, the repaired spreading generator, the **SS-R1..SS-R9 receipt series** (R1/R2 blocking pre-run; R5 amended at SS-A11, its stated content having been tautological), complete-face/tie-multiplicity receipts, the returned survey reading and its corrections. **SS-A18** repairs one cross-reference (`FF-A26(iv)`, not `FC-A26(iv)`) and closes the range without changing freeze 54. |
| `GT1-A` | GT1-A1..**GT1-A24** | Three sections: GPU-native trick-1: the bounded portable foundation; the binding M2 Metal parity gate; the binding M3 perfect-recall-net parity gate | 2026-08-16 / 2026-08-17 | One continuously numbered family. The received-v0.2/adjudicated-v0.3 parent through **freeze 55**, then the exact M2 rebrief, binding M2 contract, integer corpus, extracted choose table, typed ABI, complete parity carrier, sequential runner, persistence gate and finite Lean foundation through **freeze 56**. The historical Gate-0 NO-GO remains a true old-environment receipt. **M2 METAL PROJECTOR PARITY COMPLETE under freeze 56**; it computes no action value, selected lead, optimal set, information net, continuation, performance claim or player. A18–A24 add the exact M3 rebrief, the binding M3 contract, the perfect-recall/world-revealed key types, the two-family reduction algebra, the `Texas42.Trick1PerfectRecallNet` proof boundary and **freeze 57** — the gate only, **no M3 result recorded**. **RANGE RE-FROZEN at A1..A24 and the chapter closed**; any later GT1 ruling requires an explicit rebrief and another range re-freeze. |
| `SP-A` | SP-A1..**SP-A12** | Signed-pivotal intake adjudication | 2026-08-18 | The intake audit of the received `signed_pivotal_geometry_v0.1.md`, in the DS-A1..A16 shape. Central mathematics SOUND (boxed identities verified exactly at intake); exactly one general claim FALSE as written, repaired at **SP-A5**; renames **pivotal cover / pivotal win share / frozen policy** (SP-A1..A3); tape typing (SP-A6); the sandwich discipline inherited into the three locks (SP-A7); E0 **ADOPTED as the tilt audit** with corrections SP-A8..A10; O10–O11 permanently retired (SP-A11); Gate E concordance (SP-A12). |
| `FZ-A` | FZ-A1..**FZ-A6** | The freeze-56 v2 amendment | 2026-08-24 | The one-crate unification meets the source closure: freeze-56's cumulative source closure re-issued append-only at the post-fold layout (`…m0_m2_sources_v2.sha256`, a **new** build identity attested by no hardware receipt yet); the 32-entry fold-translation table as verifier amendment; drift disposition; full-closure checking demoted to **freeze-event** verification; the standing M2 receipt explicitly **old-layout evidence**, its re-earning deferred to [[m2-receipt-reearn]]. Unlike SS-A and GT1-A, neither SP-A nor FZ-A carries a range-close marker, so both ranges end open. |
| `CE-A` | CE-A1..**CE-A8** | Calculated-evidence intake adjudication | 2026-08-24 | The adjudication of the received `calculated_evidence_v0.1.md` (anytime-valid adaptive settlement; the parent embodied Jason's Pro refinement pass). Identities **SOUND** at intake, 18/18 exact (CE-A1); the **θ/ϑ split** adopted walt-wide (CE-A2, superseding the SP companion's bare-θ proposal); the six-way **result-type ladder binding** on the new correctness path (CE-A3); **O20–O28 accepted** into the SCENARIO-PLAYER ledger (CE-A4); fixed counts leave the correctness path, block racer narrowed to heuristic, §10.1 becomes gate fixture V7 (CE-A5); LEVEL2-PROBE amended with the three-way wake-up split and the `𝓘 = q·D_{1/2}(τ)` cost coordinate (CE-A6); the parent's **§22 adopted as the build program**, A.6 vertical slice first, old player stays default until gates justify a change on Jason's word (CE-A7); refinement-agenda disposition, no panel convened (CE-A8). Superseded as the file's final ruling by the L2-A family later the same day; the range ends open. |
| `L2-A` | L2-A1..**L2-A7** | Targeted level-2 field-stability intake adjudication | 2026-08-24 | The adjudication of the received `targeted_level2_field_stability_v0.1.md` (second same-day drop, same Pro-session lineage; **filed under the standing same-lineage go — the authorization note travels with the rulings**). L2-T1..T5 **SOUND** at intake, 19/19 exact model-check, targeting frame adopted (L2-A1); **O29–O38 accepted** into the SCENARIO-PLAYER ledger (L2-A2); the seven field-swap **result kinds binding**, Rust naming free (L2-A3); **exposure-tier typing binding** — only `RootActionExposureUpper` feeds the L2-T2..T4 screen, every bound names its rung (L2-A4); **LEVEL2-PROBE amended** to the detection layer inside the targeted controller (L2-A5); the field-swap **build slots after CE §22 step 7** (shadow), Gran anchors carded as [[gran-anchor-reconstruction]] (L2-A6; reconstructed 2026-09-04); **cycle discipline adopted** — recurrence typing, the §13.5 level-3 tripwire, no mitigation without a separate intake (L2-A7). L2-A7 was the file's final ruling for part of 2026-08-24 (the 2026-08-24 edition of this page said so); PANEL-A followed the same day; the range ends open. |
| `PANEL-A` | PANEL-A1..**PANEL-A8** | The panel-response adjudication | 2026-08-24 | Pro's adversary-panel response to the four CE/L2 briefs (x:019–023): verdicts accepted at instrument tier — "certified" is the panel's word and promotes nothing (A1); sign-safe wording binding (A2); **Claim D repaired** — retrospective edge-risk opening counterexampled at 169/512 > 1/4, future-only or preallocated (A3); "refund" language retired (A4); W7–W11 adopted (A5); the τ stopping-time coupling definition (A6); the Part VI **cancellation ladder** with pairwise masses and diagnostics adopted for slice 3 (A7); **directional bounds** adopted as the slice-3 rung (A8). The committed lift corrected 41/1200 → 31/1200. No range-close marker. |
| `TRIPLE-A` | TRIPLE-A1..**TRIPLE-A7** | The deferred-producers adjudication | 2026-08-25 | x:024: intake accepted at instrument tier, verifier 13/13 (A1); the **max-preserving upper CS** adopted for the E3 producer — no Bonferroni split (A2); directional E3 = separate solves with separate ledger entries (A3); the **Hazard-Exclusion Invariant** adopted as the single dominance-bound authority (A4); the one-round trump-extraction witness as the first deliberately incomplete producer (A5); the first-split **motif alphabet** for correction mass (A6); `RevealResponse` remains refused, path via raw suffix enrichment (A7). All three producers built with gates the same night. No range-close marker. |
| `CBS-A` | CBS-A1..**CBS-A9** | The counted-belief-sandwich adjudication | 2026-08-30 | Intake accepted at instrument tier, verifier 20/20 (A1); Part II recognised as the pmake generalisation of TRIPLE-A2's M1/M2 — no new statistics (A2); **root-interval result types adopted; "sandwich" ruled not a citable object name** — root interval, survivor set (A3); policy cylinders and grammars adopted, the §11 non-theorem = the O34 fence (A4); counted cells and threat/safety covers (A5); **Theorem 20.1's seat-factor posterior closure adopted with its boundary** — any cross-seat coupling voids the closure until represented as factors; `FiberDp` is the uniform-root oracle (A6); consequence CEGAR (A7); §0 probe citations amended — probe READMEs remain the authority (A8); the build program adopted, §56 first, §57 stop conditions, live default untouched; **CBS-O1..O15** accepted (A9). No range-close marker. |
| `APS-A` | APS-A1..**APS-A9** | The anytime-proof-state adjudication | 2026-08-31 | Intake accepted, verifier 36/36 (A1); the score layer sound and adopted (A2); rescue/fragile bands and count-threat covers (A3); **the envelope fence binding** — 63/2 (A4); **the laydown hierarchy adopted as typed vocabulary**, bare "laydown" reserved (A5); proof bar / executable bar split; audit finding that `bar_of` in `refine.rs` is the proof bar (A6); **certified regret adopted** as the finite-budget guarantee (A7); closure-aware usefulness amends the steering doctrine (A8); architecture adopted as candidate, build program queued, **RefineV1 frozen semantically** — the register issues the number 58 — **PS-T1..T15** accepted (A9). No range-close marker. |
| `MB-A` | MB-A1..**MB-A8** | The model-belief base-player adjudication | 2026-09-01 | The substrate `Ξ = Ω × Θ` adopted, Theorem 7.1 (A1); the missing parent resolved same day (A2); **the rung registration corrected at intake** — `D = Dice`, `F₀ = BR(D) = σ0`, `F₁`, `F₂` unbuilt; "Dice = σ0" retracted (A3); two transcription errata, companion governs (A4); **the residual Other type mandatory** (A5); identity discipline compatible (A6); boundaries restated — live default untouched (A7); the build program restructured and authorised, **MB-O1..O20 / MB-I1..I10** (A8). No range-close marker. |
| `SC-A` | SC-A1..**SC-A8** | The salvation-complex adjudication | 2026-09-01 | The geometry adopted, the record corroborates it — the §9 table's fourteen `d_info = 0` coordinates checked against `doomreport` per-world truth (A1); **no verifier shipped, the gates are the checks** (A2); the §47 immediate ruling (A3); **the fusion horizon is an empirical object first**; `UnknownGodGap` is a distinct type (A4); **the two fences binding** — an optimizer disagreement is not a cut; G is neither sub- nor supermodular (A5); provenance notes (A6); reuse discipline across the tower (A7); the U/T program adopted, U0 pulled forward, the §60 tranche (**SC-O1..O16**) (A8). No range-close marker. |
| `FH-A` | FH-A1..**FH-A11** + five delivered propositions | The focal-horizon adjudication | 2026-09-04 | Intake accepted at instrument tier; the verifier's "24 CHECK FAMILIES" is a literal (A1); **vocabulary and thread ruled** — hierarchy, interval, action interval, bar, survivor set, focal depth; "certified regret" stays; L2 fixed-field thread (A2); **the upper tail admissible, proved (FH-God); the trivial upper is a refusal** (A3); the lower tail ruled — σ0-as-focal primary (A4); existing instruments identified in the new vocabulary (A5); `h_f` ruled (A6); the tie rule ruled, FH5 the lower-side fusion gate (A7); **the anchors confirmed, answers not pinned** (A8); interruption, preserved facts and suffix identity ruled (FH-int) (A9); **non-goals binding** — no live default change, no arena claim (A10); the split — FH1 first as an affordable-or-refuse engine, then FH2, FH3 (A11). Propositions FH-God, FH-int, FH-tie, FH-cut, FH-last delivered in the section. No range-close marker; the last section of the file at `c00717d1`. |

Three numbering traps. **`E-A` and `E-Q` are spent** by the endgame-store section
of 2026-08-11 — SEP-A had to renumber the separation design's own `E-Q1..E-Q8`
to `SEP-Q1..SEP-Q8` to avoid a collision inside one file. `F2`'s internal
amendments are bare `A1`–`A4`, local to F2, not a prefixed family. And **standing
rulings inherit as whole families, by name, never as ranges**: two design headers
recite `X-A1..X-A17`, `E-A1..E-A20`, `S-A1..S-A18`, but the families actually run
to X-A19, E-A21 and S-A21. The full corpus binds regardless of a header's recited
range; those recited ranges are typos, not scope declarations.


---

## Appendix D — The supersession chain

Nineteen pointer markers, governed by **DS-A28**. Every one is navigation, not a
rewrite: the original text stands and the marker names its replacement. Their
absence would be drift. Two later corrections (2026-09) are not rulings and
live outside the rulings file, in `walt/DISCREPANCIES.md`; they are listed in
the third table below because a reader of the doom paragraph or the salvation
companion will otherwise meet the superseded form without its correction.

### In `CENSUS-RULINGS.md` — twelve markers

| Marked site | Marker | Corrected by | Durable replacement |
|---|---|---|---|
| **Lemma J**, clause (c) | GENERALISED | DS-A24 | **Lemma J(c′)**, errata §8.5(e). (c) is *sound as filed* — generalised, not repaired. |
| **DS-A7(iii)**, the sentence naming `revealed.rs` as needing an evaluator built | CORRECTED | SEP-A7 | **SEP-A7** with freeze 37 at SEP-A6. `revealed_summary().q_c[a]` has been the action-conditioned U_a since S3; what remained to build was the harness. |
| **DS-A9**, the cone clause "for every cone at once" | CORRECTED | DS-A24 | **Lemma E8**, errata §8.5. Equality is guaranteed only for valuation directions constant on the exchanged tiles. |
| **DS-A10**, the Experiment E receipt clause | SUPPLEMENTED | SEP-A12 | **SEP-A12 (R1)–(R5)**. The clause stands, but its two assertions hold by construction and are not receipts in PG-A8's sense. |
| **SEP-A13**, the third counted quantity | DISAMBIGUATED | SEP-A19 | **SEP-A19**. The third quantity is the distinct partition states the walk *reaches*; equality with `InfoPartition::len()` is unsatisfiable for any pruning policy. |
| **SEP-A4(e)**, freeze-36 transport = identity only | AMENDED | EC-A8 | **Freeze 36 v2** additionally admits the declaration fold under its explicit image-key construction, receipts and Corollary S-fold-val; every further transport still re-enters. |
| **SEP-A17**, “the 108-decision playbook” | DISAMBIGUATED | EC-A12 | **108 is the strictly-mattering subset**, a derived difference of two measurements; the receipt-backed free-decision count is 384. |
| **N4-A1(i)**, the cross-traversal fence | NARROWED | Lemma N | Traversal counts remain incomparable unless the traversals are exhibited as the same; the partition build and envelope-H walk are one such exhibited identity. |
| **N4-A1(c)**, the six evaluator inventory | CLARIFIED | RW-A8 | Freeze 44(b)'s `Option`/no-partial contract binds every `walk`-based evaluator; the six were an inventory, not a closed type list. |
| **N4-A1(e)**, `P_max = 32,000,000` at insertion | AMENDED | N4-A16(vi) | **Freeze 44 v2:** `P_max = 192,000,000`, applied to the completed count-only result before allocation; insertion is a defensive stop only. |
| **N4-A4**, provenance of `P_max v1` | SUPERSEDED IN PART | N4-A16 | The v1 estimate was measured wrong; N4-A4's rule survives — v2 is declared in adjudication and is never derived from machine memory at run time. |
| **N4-A12(b)**, the three-coordinate fallback | SUPERSEDED | N4-A19 | Its wall gate was retired; the result-independent replacement is the full nine-coordinate pass with measured per-unit admission. |

### In the errata — seven in-place markers

| Marked site | Marker | Under | Effect |
|---|---|---|---|
| §3.1, the object | Naming clause added | DS-A20 | Treatment **C** reveals ω only; revealing (ω,z) is **C⁺** and never called C unqualified. |
| §4.2, the Obligation | Restated semantically | DS-A27 | The invariant is semantic; "no max node below the root" is demoted to a sufficient *implementation form* — a receipt for the invariant, not the invariant. |
| §5.1, Proposition E5 | Statement amended | DS-A20 | Hypothesis **(S)** added explicitly; **Lemma E5.0** (§5.1a) shows it holds on the measured carrier. |
| §5.2, Corollary E5.2 | Language narrowed | DS-A21 | The negative is "atom-mass **linear** filtering is noncompressive on this carrier", not "filtering and compression are incompatible". |
| §5.3, the reframe | Sharpened | DS-A21 | Predictive rank lower-bounds the **linear factorisation** target only, not unrestricted circuit size. |
| §6, Theorem E6.1's typing clause | Superseded in part | DS-A23 | The primary object is **Definition E9**'s interface-local width; the root-level W_reach is a different, fourth quantity. |
| §6, Theorem E6.2(c) | Narrowed | DS-A26 | Restricted to the two pruning rules actually in use; duplicate-discarding rules preserve the count trivially. |

**The shape of the chain.** DS-A24 is the mathematical correction inside the
original decision-sparse theorem chain, and it fires twice, both landing on
errata §8.5. DS-A20/A21/A23/A26/A27 are errata maintenance. The later markers
are implementation-boundary, freeze-version, scope or provenance repairs; none
is permission to rewrite the marked text or to transport a result across the
new boundary silently.

### In `walt/DISCREPANCIES.md` — two 2026-09 reconciliations, and one companion erratum

| Marked site | Marker | Corrected by | Durable replacement |
|---|---|---|---|
| `walt/FACTOR-BELIEF.md`, the doom-census paragraph (2026-09-01): "the plateau's remaining Γ ≈ 267‰ is overwhelmingly the info-consistency price" | RECONCILED (one-line pointer at the site; the ledger is a dated running record and is not rewritten) | SC §8–§9, SC-A4; U0 (2026-09-02); the DISCREPANCIES entry of 2026-09-03 | A zero doom census moves only `d_phys`; the unclaimed mass is `d_info + d_policy` and zero doom does not distinguish them; the opening root is `UnknownGodGap`; **the 267‰ is UNKNOWN in its split**. |
| `walt/math/salvation_complex_v0.1_intake.md`, "two divergence points" | RECONCILED (companion is a dated intake record; not repaired) | U0's gate `solver_godgap.rs::the_section_nine_table_is_re_derived_from_the_committed_record` (2026-09-02) | Three coordinates where the class census's certified mass falls short of per-world truth (the two named plus h8-t5 0-0, 17 of 21); nothing above it moves — the §9 table cites truth at all fourteen coordinates. |
| `walt/math/model_belief_base_player_v0.1.md` §8 display and §34 operator | Transcription errata, recorded not repaired (the parent is pinned) | MB-A4 | Cite the companion's forms: `Q_a(δ_{F_k})` = the ordinary best response to rung `F_k`; the §34 operator is ≠. |


---

## Appendix E — Standing disciplines a successor must not relearn the hard way

Each of these is a ruling, not a convention, and each was bought with a mistake.

- **F7, NO-RESCUE.** Both outcomes of every experiment are results. A mismatch
  against the concrete authority is stop-and-report; never patched, never
  reconciled by adjustment.
- **DS-A1, vocabulary.** *witness* = a mathematical object exhibited to prove a
  claim; *receipt* = a machine-checked verification artifact regenerated by a
  run. **The word "certificate" is not used in walt artifacts written under this
  ruling** (D3); quotations of documents that use it are bracketed. DS-A1 was
  ruled on 2026-08-13 and **binds forward, not retroactively**, so a grep of
  `walt/` returns many hits and none of them means the rule is dead: received
  documents preserved verbatim (DS-A18), pre-reset code and artifacts, prose
  written before the ruling landed, and sentences that state the fence itself
  all contain the word legitimately. **No inventory of those hits is maintained
  here** — the tree moves, and any list would rot into a false completeness
  claim. The operative test is prospective: in anything you write, use *witness*,
  *receipt*, or *root-action separation*, and bracket the word when quoting a
  source that uses it.
- **PG-A8, "by construction is not a receipt."** An assertion that cannot fail
  is not evidence. SEP-A13 is the sharpest instance: an `is_affine()` check that
  is vacuous at the declared direction.
- **E-A2, the count boundary.** Structural transports preserve BEATS relations,
  not pip counts. If count re-enters, every form-keyed record is void
  **wholesale, never extended**. The one thing that survives is a *policy* as a
  primal-witness source (DS-A16) — the policies extend, the verdicts do not.
- **R-A2 and P-A1, the reachability fence.** The measured domain is the
  void-free capacity fiber, a declared cost domain. Its members are FEASIBLE and
  never reachable; no object here is identity-bearing. Restate it verbatim
  wherever a witness is reported.
- **PG-A13, a stop is a stop.** A capped coordinate reports no count at all —
  not a partial one and not a bound. Do not infer where the first split happens
  from where a capped run stopped.
- **R-A18, the correctness gate.** Treatment H is the concrete authority; a
  disagreement is a bug. If H does not complete within budget, that is a
  declared stop printed with what was reached, and every dependent row prints
  "correctness gate unmet" beside it — never silently.
- **DS-A15/Lemma E7, seeds versus witnesses.** Seeds are heuristics for
  *finding* witnesses; witnesses are validated by exact evaluation, always.
  Dominance does not travel with a policy alone.

Added by the rulings of 2026-08-24 → 2026-09-04, each likewise bought:

- **CE-A5, no magic n on the correctness path.** Fixed sample counts are
  resource limits, never proof rules; a run that returns to a magic count on
  the correctness path is a regression. **CE-A6:** sampling cost is compared
  only by `𝓘 = q·D_{1/2}(τ)`, never by `q̂` alone.
- **CE-A7 / §20.16, restated at CBS-A9, APS-A9, MB-A7, FH-A10 — the live
  default fence.** No lineage changes the default player until arena and
  conformance gates justify it on Jason's word; no arena claim rides on a
  slice.
- **L2-A4, every bound names its rung; L2-A7, level 2 is a best response to a
  named σ1.** A sampled lower witness is never an upper bound; "equilibrium",
  "convergence" and monotone improvement are not level-2 words.
- **PANEL-A2/A7, sign-safe wording; cancellation is never dominance.**
  Cancellation justifies a value statement under one declared objective,
  belief and model — never pathwise safety, structural irrelevance, dominance
  or reweighting stability. Dominance is `H = 0 ∧ B > 0`, inhabited only by
  verifier-checked witnesses (TRIPLE-A4).
- **CBS-A3 / FH-A2, "sandwich" is not an object name.** Root interval,
  survivor set, focal-horizon hierarchy / interval / action interval, bar,
  survivor set, focal depth. The FH4 audit's single BLOCK was this word in a
  gate name.
- **CBS-A4 / SC-A5 / MB-I4 / FH §6, the strategy-fusion fence, four times.**
  Two lower witnesses are never coverage of omitted policies; an optimizer
  disagreement is not a cut; merge before max over public actions; a
  per-world optimum is at most an upper bound.
- **CBS-A8, probe READMEs are the authority for probe findings.** A number
  in a ledger paragraph or era page that disagrees with the record is the
  bug; the 2026-09-03 doom-paragraph reconciliation is the instance.
- **APS-A4, the envelope fence.** A threshold-wise envelope across policies is
  never an executable profile and is never serialised as one.
- **APS-A5, bare "laydown" is the universal type.** A model-relative
  pmake = 1 is *indifference under a field model*, never a laydown; the G2
  lock is the instance.
- **MB-A5, unknown model mass is stored, never discarded.** The residual
  Other type is mandatory from the first post-MB0 slice.
- **SC-A2, the gates are the checks.** A parent may ship no verifier when the
  build's gates re-derive its tables from the committed record; then the
  gate, not the parent's prose, is what is cited.
- **SC-A4, a hypothesis is a census target until adversarial search fails
  to break it.** `UnknownGodGap` is a result type; zero certified doom with
  no exact Q is never `PositiveGodGap`.
- **FH-A3, the trivial upper is a refusal.** An unaffordable relaxation
  leaves its node unfinished; it never installs 1 as a fact.
- **FH-A11 / FH-int, no partial intervals before the intersection-and-witness
  discipline exists.** A node's fact is the intersection of prior and new;
  every lower fact carries the policy attaining it; resume ≡ uninterrupted
  because facts are a function of the completed set.
- **The 2026-09-04 consolidation ruling (Jason).** No new mathematical parent
  until the consolidation slice lands; endpoints of one recursion are retired
  as instruments, not added to.


---

## Appendix F — Run names, and where a quoted measurement comes from

Rulings and results files refer to runs by session label. The labels are not
self-explanatory and no page above decodes them, so here is the minimum a reader
of these pages needs. **The session ledger itself lives in `walt/LOG.md`** (the
retired `walt/PLAN.md` carried it historically; `git show 56e2173:walt/PLAN.md`).
Result artifacts cited by `results/...` basenames live at
`walt/probes/factory-results/` since the 2026-08-24 relocation.

| Run | What it was | The artifact that carries its numbers |
|---|---|---|
| **S5g** | the railyard | `results/census_yard*_2026-08-10.txt` |
| **S5h** | the fiber-crush probe (three-arm baseline ladder) | `results/fiber_probe_2026-08-11.txt`, `fiber_probe_h_2026-08-11.txt` |
| **S5i** | the fiber-refinement probe (declared exclusion remnants) | `results/fiber_refine_2026-08-11.txt` |
| **S5j** | the endgame store (symmetry-reduced tablebase) | `results/endgame_store_2026-08-11.txt`, `endgame_floor_2026-08-11.txt` |
| **S5k** | the seat-level census | answered by proof; see Corollary S-rigid |
| **S6a** | the predictive-rank probe | `results/predictive_rank_2026-08-12.txt` |
| **S6b** | the policy-geometry probe | `results/policy_geometry_2026-08-12.txt` |
| **S6c** | the decision-deadness probe | `results/deadness_2026-08-12.txt` |
| **S6d** | the separation probe (Experiment E) | `results/separation_2026-08-13.txt` |
| **S6e** | the economy-seed probe | `results/economy_seed_2026-08-14.txt` |
| **S6f** | the measured n = 4 admission rung | `results/separation_n4_rung_2026-08-14.txt` |
| **S6g** | the trick-1 drawing-family probe | `results/trick1_draw_2026-08-14.txt` |
| **S6h** | the full n = 4 overnight pass | `results/separation_n4_2026-08-14.txt`, `separation_n4_2026-08-14_deterministic_block.txt` |
| **S6i** | the lay-down characterization/catalogue | `results/laydown_2026-08-14.txt`, `laydown_catalogue_2026-08-14.txt` |
| **S6j** | the map-free rule-economy pass | `results/rule_economy_n4_2026-08-14.txt` |
| **S6k** | the first-rung fusion-tax probe | `results/fusion_tax_2026-08-14.txt` |
| **S6l** | the depth-two nonanticipativity rung | `results/second_rung_2026-08-14.txt` |
| **S6m** | the feature-fee audition and repaired rerun | `results/feature_fee_2026-08-14.txt`, `feature_fee_v11_2026-08-14.txt` |
| **S6n** | the fee-correlation diagnostic | `results/fc_correlation_2026-08-14.txt` |
| **Seed survey** | the unnumbered 2026-08-15 hundred-seed survey and its declared scratch cuts | `results/seed_survey_2026-08-15.txt`, `seed_survey_2026-08-15_cutA.txt` |
| **GT1 M0/M1** | the portable GPU-native trick-1 foundation | `walt/receipts/gpu_native_trick1_m0_m1_v1/` |
| **GT1 historical Gate 0** | the immutable NO-GO observation from the old Command-Line-Tools-only environment | `walt/receipts/gpu_native_trick1_gate0_2026-08-16.txt` |
| **GT1 M2** | the committed receipt for the exact freeze-56 status sentence; executable fixed-carrier evidence, not a theorem | `walt/receipts/gpu_native_trick1_m2_v1/` |
| **Scenario-player first day** | the level-1/level-2 ladders and the 3×384 arena vs the E[Q] champion (2026-08-17) — arena outcomes about play, never exact values | `walt/probes/m3/arena_results_2026-08-17.txt`, `level1_results_2026-08-17.txt`, `level2_results_2026-08-17.txt` |
| **Divergence miner** | 900 self-played hands / 4,156 level-2-shadowed decisions (2026-08-18) — the tilt audit's first anchor corpus | `walt/probes/m3/divergence_results_2026-08-18.txt`, corpus under `walt/probes/m3/mined/` |
| **Tilt-audit smoke** | the E0 smoke (2026-08-19): trick-6/trick-4 anchors, the no-tape finding, the racing bench and arena gate — estimates, never receipts | `walt/TILT-AUDIT.md` § "Smoke results", `walt/probes/tilt_arena_2026-08-19.log` |
| **CE step-7 shadow** | the controller beside the live player (2026-08-24): 33 hands / 183 decisions at δ_run = 1/100, world_cap 128 — instrument records, model-relative winners, no live change. The committed records are the **world_cap = 128 epoch**; the bin default is 512 since the cap ruling | `walt/probes/shadow/` (README + JSONL + `summarize.py`) |
| **Field-swap smoke** | the L2 §21 step-5 fixed-policy smoke (2026-08-24): three roots, two frozen pins each, `FrozenPolicyExposure` only — never root-action screening | `walt/probes/fieldswap/` (README + JSONL + `summarize.py`) |
| **CE step-8 calibration** | the §19 V5 cap-ladder replay and §19 V6 per-fixed-pair E0 calibration (2026-08-24): 10 V5 records over the ladder 40/160/640, 18 pairs × 3 replicates at T = 400 — forecasts are forecasts, settlement is the exact evidence threshold | `walt/probes/step8/` (README + JSONL + `summarize.py`) |
| **Field-swap screen** | the L2 §21 steps 6–8 rung/screen instrument (2026-08-24): three exact parity roots at one declared (σ0, σ1) epoch pair — rungs E0–E2, the exact split-reach route E4 (`R_a` exactly), the L2-T4 admissible set and slack table | `walt/probes/fieldswap_screen/` (README + JSONL + `summarize.py`) |
| **Field-swap cancel / motifs / hazard** | slice 3's cancellation ladder and directional bounds (ε = 1/20; the first `FieldDecisionChanged`, `FieldStableExactRoot`, `Dominated`), slice 4c's motif classification (453/453, residual 0), slice 4b's hazard producer (0/40 wild accepts) — 2026-08-24/25, epoch σ0 = `Level0{n0 = 8}` | `walt/probes/fieldswap_cancel/`, `fieldswap_motifs/`, `hazard_witness/` (README + JSONL + `summarize.py`) |
| **Step 9** | the detection layer run (2026-08-25, PR #49; bin `wakeup`, not `step9`): 10 roots / 24 pairs at one declared epoch pair (σ0 = `Level0{n0 = 2}`) — typed wake labels; **does not compose** with the n0 = 8 field-swap corpora | `walt/probes/step9/` (README + `records.jsonl` + `summarize.py`) |
| **L2 controller** | the targeted field-1 controller's first corpus (2026-08-25): rung spend schedule-controlled, exact E4 never paid | `walt/probes/l2_controller/` |
| **Shadow 512 epoch** | the two committed 512-world reruns of the step-7 shadow (PRs #34/#36): 3 DeltaSettled, two against the live choice; the live opening lead δ-eliminated once; the "~108/116 settle by 512" forecast retracted in the README | `walt/probes/shadow/receipt_512.jsonl`, `driven_512.jsonl` |
| **Waking seat** | the 2-hand / 56-decision phase conviction (2026-08-25): 729‰ of decision compute in the σ0 baseline, 926‰ in tricks 1–2; the G1 replay (6/7 agreement, one wake at trick 5) | `walt/probes/waking/` (README + JSONL + `summarize.py`) |
| **Root interval, grammar residual** | Slices A and B (2026-08-30): the first root intervals over pmake; the §12 triple walk and the §8 residual identity | `walt/probes/root_interval/run1.txt`, `walt/probes/grammar_residual/run1.txt` |
| **Factor belief C0/C1/C2** | the opening root contracted (8.7 ms trivial field / 5.36 s σ0), the cache study (reuse ×230 within a history, exactly 0 across), the seven §46 coordinates | `walt/probes/factor_belief/run1.txt`, `opening_level0_run1.txt`, `cache_run1.txt`, `c2_run1.txt` |
| **Slices D–G** | the factorised recursion, the grammar response, consequence CEGAR, the integrated controller (RefineV1) | `recursion_run1.txt`, `response_run1.txt`, `cegar_run1.txt`, `refine_run1.txt` (all under `walt/probes/factor_belief/`) |
| **Phases 0–8 and the doom census** | the score profile, proof state, certified regret, argmax extraction, work frontier, residual Bellman + covers, laydowns, the opening-root ladder (§65), the ∀-fail census | `profile_run1.txt`, `proofreport_run1.txt`, `extractreport_run1.txt`, `frontierreport_run1.txt`, `bellmanreport_run1.txt`, `laydownreport_run1.txt`, `openingreport_run1.txt`, `doomreport_run1.txt` |
| **MB0 / MB1** | the model-belief substrate and the model-fusion price (2026-09-02); 38/9600 gate-pinned (M6) | `modelbelief_run1.txt`, `modelbelief_recursion_run1.txt`; `walt/briefs/MB1-REPORT.md` |
| **U0 / U0b** | the God-gap census (fusion horizon at trick 5) and the in-solve horizon census (not fusion-free inside a trick-4 solve; h8-t3 solved exactly) | `godgap_run1.txt`, `horizon_run1.txt`; `walt/briefs/U0-REPORT.md`, `U0B-REPORT.md` |
| **UP0 / UP1a** | the unified player's two rungs and the lazy carry (2,105,672 µs → 0 µs) | `unified_run1.txt`, `unified_run2.txt`; `walt/briefs/UP0-REPORT.md`, `UP1A-REPORT.md` |
| **FH1 / FH2 / FH3** | the focal-horizon scout, the ladder, and the report of record over 33 coordinates (2026-09-04); the FH4 audit reproduced all three | `focal_run0.txt`, `focal_ladder_run1.txt`, `focal_run1.txt`; `walt/briefs/FH1-REPORT.md`, `FH2-REPORT.md`, `FH3-REPORT.md`, `FH4-AUDIT.md` |
| **Gran anchors** | G1 transcribed and validated, G2/G3 partial; the waking seat's replay and driven runs on G1 (2026-09-04) | `walt/probes/gran/` (README, `g1.receipt.txt`, `g2g3.receipt.txt`, JSONL, summaries) |
| **Gran level 2 and the synthetic lock** — *branch only* | the 2026-08-17 level-2 player on G1, G2 and the hand-built lock (2026-09-05); G2 exactly locked from trick 3 | `walt-g1-l2:walt/probes/gran/level2_g1.txt`, `level2_g2.txt`, `level2_lock.txt`, `synthetic_lock.receipt.txt` (tip `6abdd78f`, not in main) |
| **O5** — *branch only* | the no-void inner simplification measured; the mirrored match at two epochs (33/19/20 live; 76/83/33 reduced), "dead heat" withdrawn | `walt-o5:walt/probes/o5/README.md` (tip `2981e090`, not in main) |
| **Partnership batteries** | the foundation and default batteries (2026-09-06): 14/14/72, 12/17/71, 9/5/36, 5/8/37 — results files outrank prose | `experiments/partnership/campaigns/*/RESULTS.md` and the per-match `MATCH.md` |
| **The gym** | 6 / 170 / 433 / 30 exercises with exact, thrice-audited keys (2026-09-06/07) | `walt/gym/RESULTS.md`, `DISCOVERY.md`, `BID-MAKING.md`, `PARTNERSHIP-COMPOSITION.md`; [walt-gym](walt-gym.md) |

Every measured number quoted anywhere on these pages comes from one of those
files. **They sit one tier below even these pages** (probe output is exploratory
material cited by nothing above it), and a number becomes quotable as a *result*
only by brief amendment adding it to a verifier receipt. Where a page names a
measurement — the grade-3 predictive dimensions, the detector recall figures,
the singleton frontiers — treat it as a pointer into the artifact above, never
as a standing claim.

The design documents that each ruling family adjudicates live at the top of
`walt/` (`walt/POLICY-GEOMETRY.md`, `walt/SEPARATION-PROBE.md` and others) —
except the seven retired 2026-08-24 after their probes closed (`CENSUS`,
`FIBER-PROBE`, `FIBER-REFINE`, `ENDGAME-STORE`, `SEAT-CENSUS`,
`PREDICTIVE-RANK`, `DEADNESS-PROBE`), whose bytes are preserved at
`git show 2de8a05:walt/<NAME>.md`. A ruling that says "the design" means the
one named in its section's opening paragraph.


---


## Appendix G — What is owed on the record (as of `c00717d1`, checked 2026-09-13)

Not mathematics owed — that is Chapter 12 — but the record's own debts: things
a reader will look for in the place the conventions say they should be, and
not find.

| Debt | Where it should be | State | Where to read the fact |
|---|---|---|---|
| **Errata §9** — the FT/SR/FF/FC nonanticipativity objects | `walt/math/decision_sparse_exact_solving_v0.1_errata.md` §9, per DS-A28(ii) (owed since FT-A27(i), 2026-08-14) | **Unfiled.** The errata's headings end at "## 8.6 Index addendum"; `CENSUS-RULINGS.md` remains the only authority for Lemmas FT-*, SR-*, FF-*, FC-*. | [open questions](walt-math-open-questions.md) item 25 |
| **Errata §4.3** — Corollary E4.1, the primal ceiling | errata §4.3 (owed since SEP-A2, 2026-08-13) | **Unfiled**; the only statement is `CENSUS-RULINGS.md` § "Experiment E adjudication". | same |
| **Freeze 58's number** | a ruling in `CENSUS-RULINGS.md` | **CENSUS-RULINGS never names freeze 58.** APS-A9 froze `solver/refine.rs` "semantically as the RefineV1 reference" without a number; the number was issued by [the freeze register](walt-math-freezes.md) (2026-08-31 addendum) and is consumed by `walt/FACTOR-BELIEF.md`, `walt/MAP.md`, `walt/LOG.md`, eleven briefs and six solver source/test files. A search for "freeze 58" in the rulings file returns 0 hits (measured 2026-09-13 on this machine). No freeze 59 exists anywhere. | register row 58; item 25 |
| **The four obligation ledgers** — CBS-O1..O15 (CBS-A9), PS-T1..T15 + the 42-instance layer (APS-A9), MB-O1..O20 / MB-I1..I10 (MB-A8), SC-O1..O16 / §60 (SC-A2) | a "Lean side-project ledger" | **Exists in no file.** The identifiers occur only in `CENSUS-RULINGS.md`, `walt/FACTOR-BELIEF.md` and [walt-math-intakes](walt-math-intakes.md); nothing under `lean/`, `wiki/lean.md` or `kanban/` names them; `kanban/backlog/lean-catchup.md` is still backlog. | items 9, 26 |
| **The intake table** — every received artifact, its pin, companion, rulings and what came of it | [walt-math-intakes](walt-math-intakes.md) | Complete through the FH intake of 2026-09-04 (nine pinned parents, all re-hashing exactly; §7 the pinned manifests); its §9 indexes the unintaken notes below. This page does not duplicate it. | that page |
| **The three unintaken packet notes** — *TEXAS42-UNIFIED-REVIEW-v0.1* (2026-09-05, `daae4332…`), *TEXAS42-IMPROVISATION-v0.1* (2026-09-05, `867db550…`), *PATH-TO-GENERALIZING-SCHEME-POLICIES-v0.1* (2026-09-07, `deff51a3…`) | `walt/math/` with companion, verifier, same-day rulings, obligations ledger | **Received, hashed by their packets, never intaken**; UNIFIED-REVIEW names two verification files that are not in the packet; IMPROVISATION and PATH are already consumed by `walt/scheme/INFORMATION-PRICES.md` and `experiments/partnership/RELATIONAL-LEARNING.md` without an adjudication; each uses the word "certificate". The launch packet's `MANIFEST.sha256` verifies 6/6 OK (2026-09-13). | [walt-math-intakes](walt-math-intakes.md) §9; item 27 |
| **x:018** — the fee-correlation correspondence | `exchange/inbox/` | No reply; `submission_count.txt` = 24; the last inbox response is 024 (2026-08-25). Every later parent was hand-delivered outside the courier ledger. | [exchange](exchange.md); item 11 |
| **The pmake QUESTION/RULING pair's basis** | `walt/walt-m3-probe/src/bin/ladder.rs` at `171cd22` | The crate was deleted in the 2026-08-24 unification; the ruling is read historically and has no ruling family and no `.sha256`. Its Q2 lemma shaped three later programs. | Appendix A row |
| **The reproduction of the era's σ0-only records after `c59f1115`** | the records under `walt/probes/factor_belief/` | The 2026-09-06 gym change to `factor_belief.rs::condition_via` postdates every committed record; masses should be unchanged, the "σ0 states materialized" and per-route wall coordinates plausibly not. No record has been regenerated. | item 28 |
| **The reference page's own history** | this page | The 2026-08-24 edition's addenda and object index are carried verbatim below; every line-number pointer in them was true at the date it was written and drifts with each append — locate by heading and ruling ID. | the addenda |

---

# Historical addenda (kept verbatim from the 2026-08-24 edition; line-number pointers in them are dated)

> These two addenda are the 2026-08-17 and 2026-08-14 editions' text, kept
> word for word as the record of what a successor inherited then. Three
> things in them have since moved and are corrected above rather than here:
> the `GT1-A` family is closed at A24 (not A17) with freeze 57; freeze 38 is
> filled; and the vocabulary predates CBS-A3/FH-A2, so "sandwich" appears
> below as the pre-ruling name of the T1-A object (itself REFUTED) — it is
> quoted, not endorsed. Line-number pointers ("≈ lines …") were true on their
> date and drift with every append; locate by heading and ruling ID.

## Addendum, 2026-08-17 — M2 closure over the portable trick-1 boundary

The portable boundary was recorded on 2026-08-16; the executable M2 conjunction
closed on 2026-08-17. Pointers only; the rulings and contracts govern.

- **The `GT1-A` family is closed at GT1-A17 and carries freezes 55 and 56.** The
  portable parent remains exactly the received-v0.2/adjudicated-v0.3 chain,
  narrow `OpeningRootV1`, generated semantics, U256 mass/frame ABI, opening-cell
  generator, reduced carrier and persistence boundary. M2 adds only the exact
  rebrief/contract, U256 corpus, extracted choose table, typed Metal ABI,
  `M2OpeningParityCarrierV1`, sequential runner, closed receipt and finite Lean
  obligations. Freeze 26 is cited unchanged; 39/40 remain reserved; freeze 44
  and M3+ are excluded.
- **`PORTABLE M0/M1 COMPLETE under freeze 55`.** The final checked source
  manifest, committed canonical envelope and stop, fresh byte comparison, Rust
  gate and Lean target passed together. This bounded status establishes no Metal
  result, perfect-recall net, controller, root value or opening action.
- **M2 METAL PROJECTOR PARITY COMPLETE under freeze 56**. This establishes only
  exact Metal/Rust arithmetic and opening-projector parity on the frozen carrier;
  it computes no action value, selected lead, optimal set, information net,
  continuation, performance claim or player. M3+ and every opening-root
  verdict remain untouched.
- **The historical Metal Gate-0 NO-GO remains true.** It is an immutable receipt
  of the old Command-Line-Tools-only environment, not the current host status and
  not a receipt to rewrite after the toolchain became available.
- **The Lean foundation is substantial and deliberately incomplete.** The parent
  module proves the legal budget/420/width/current-trick-point/cell-count/interval
  layer; `Trick1MetalFoundation` proves the finite arena/count/order/no-partial
  obligations. Neither proves the semantic opening partition and formulas,
  posterior refinement, information-key equivalence, canonical least-index
  verdict, Rust/Lean correspondence or Metal/Rust correspondence. The M2 receipt
  is executable fixed-carrier evidence bearing on the last relation, not a
  theorem or general correspondence proof.
- **The `SS-A` family is now administratively closed at SS-A18 and carries
  freeze 54.** SS-A18 corrects `FC-A26(iv)` to `FF-A26(iv)` in the non-null
  pairing provenance and closes the range; it changes no survey receipt, number,
  object or reading.
- **Freeze 44 is v2.** `P_max = 192,000,000` is applied to the completed exact
  count-only result before allocation; a larger unit is `NOT PRICED`, and the
  insertion check is a defensive stop rather than a receipt. The older 32M row
  is not the live freeze.

## Addendum, 2026-08-14 — what a walt-math successor inherits

Pointers only. Nothing below is restated from its source; open the ruling.
Written for walt-math-11 at the close of the FT chapter and extended the same day
as walt-math-11 filed the SR, FF and FC chapters. **Sections run newest first;
the FC items are the most likely to be needed.**

### The FC chapter (newest in the 2026-08-14 addendum)

- **The `FC-A` family** — `FC-A1..FC-A23`, in `CENSUS-RULINGS.md` § "The
  fee-correlation chapter: what a fee bites on, measured" (≈ lines 11170–12099 as
  of this date). Four named objects, indexed above, and **freeze 53**.
- **A PHRASING RULE BINDS ANYONE WRITING FROM THESE RULINGS, and it corrected the
  adjudicator's own text before it could reach a wiki page.** The drop bound is a
  **lower bound at every state**. Where it equals the frozen captured amount it is
  **ATTAINED**, never *exact* — "exact at 258 states" invites the reading that the
  screen predicts capture a fifth of the time, which is what the chapter exists to
  prevent. **And attainment is not identifiable in advance**: nothing in the
  emitted data says *which* states attain it without computing the very quantity
  the bound exists to avoid computing. So say *attained at 258 of 1,252*, say *a
  lower bound everywhere*, and say *which states attain it is not knowable without
  the captured amount*.
- **The generalised scope rule**: a figure's scope names **every** dimension it
  ranges over — state set, feature set and unit set — in the same sentence, and **a
  scope derived from an adjective rather than stated as a set is not a scope.** The
  chapter's own demonstration is the sharpest argument for it: one census reads
  1,010/322 over one state set and 252/322 over another, and the first invites
  "zero at most states" while the second says what is true.
- **Independent verification means an independent *predicate*, not an independent
  *party*.** Two agents running one grep are one check, however many agents there
  are. This was minted the hard way: a check written with a regex anchor where a
  literal was meant matched nothing and returned a clean, confident zero
  violations — **a wrong predicate returning exactly the answer being hoped for**,
  caught by implausibility across several simultaneous queries rather than by
  suspicion of the predicate. The operational tell: *a check that returns exactly
  the hoped-for answer with no exceptions deserves one more query by a different
  route before it is believed.* Its companion: **a predicate that fails loudly on a
  near-miss is strictly better than one that silently matches it.**
- **Two more disciplines**: receipt what the probe recomputes each run, and leave a
  documented one-time audit where the object is fixed source; and emit over the
  full set, read over the meaningful subset, **naming both**.
- **What the chapter leaves open**: the third coordinate is still uncommissioned
  and still needs its own freeze, but its selection criterion has changed — it is
  now chosen on **measured argmax multiplicity**, with the earlier trump-survival
  input demoted to a hypothesis about a correlate.

### The FF chapter

- **The `FF-A` family** — `FF-A1..FF-A33` with two closing notes, in
  `CENSUS-RULINGS.md` § "The feature-fee audition: Jason's control feature,
  specified" (≈ lines 9889–11167 as of this date). It delivers **five** named
  objects, indexed above, and **freeze 52** with four amendments. **FF-A32
  declares the chapter CLOSED**; nothing further is commissioned.
- **SR-A37 withdraws an obligation that was never owed**, and the withdrawal is
  the durable part. SR-A25(v) and SR-A32(iv) had twice filed "claim-ledger,
  FINDINGS and open-problems cross-references are owed"; the wiki owner declined
  with cause, walt-math verified the cause rather than accepting it, and the
  correct count is **zero** — `walt` appears zero times in all three pages, and
  no walt chapter has ever acquired a row. **The rule it yields: an obligation to
  write somewhere is asserted only after reading that destination's own `owns:`
  line and its rulings — a cross-reference list in governing text names
  candidates, never obligations.**
- **Three more disciplines this chapter filed, all general.** *(a)* **A results
  file may restate a reading-rule that a ruling has fixed; it may not originate
  one.** Numbers and provenance are the artifact's to assert; how a number may be
  read belongs to the rulings. *(b)* **A byte-diff between two emissions must be
  produced while both objects exist, or not at all** — it cannot be reconstructed
  after a sanctioned regeneration has overwritten its comparand. Name the check
  *and* the moment its comparand exists. *(c)* Where a superseded figure retains
  a legitimate use, that use is named exactly and every other use is closed: here
  the pre-amendment capture survives **only** as the historical measurement of
  the feature-as-frozen and hence as one term of the comparison that confirmed
  its own supersession.
- **Two disciplines from this chapter are general and should outlive it.**
  *(1)* **No capture figure, and no count, appears anywhere without the state set
  it ranges over named in the same sentence.** This file reports the same
  quantity over nested state sets, and scope mislabelling is its standing hazard —
  it caught the adjudicator twice in one chapter. *(2)* **A null control is
  complete only when paired with a case whose correct answer is known to be
  non-null**, because a control expecting zero cannot distinguish a working
  instrument from one that always returns zero.
- **Freeze 52's amendment sequence is worth reading before writing another
  freeze**: a scoping clause was written for a *family* after defining a term
  only *part* of the family uses, and was never checked against each member's own
  definition. It voided six of twelve measurement cells by construction. The
  repair is that **every feature carries its own domain clause**, and that a unit
  whose domain is empty is declared an **EMPTY TEST** — reported as a unit that
  did not run, **never as a zero**.
- **A pre-declared gate quantified over "both arms" silently assumes both arms
  are non-empty.** One did not fire as written here, and a narrower verdict was
  substituted in the open rather than the gate being read as satisfied. **A
  pre-declared outcome must either quantify over non-empty arms or carry an
  explicit empty-arm branch.**

### The SR chapter

- **The `SR-A` family** — `SR-A1..SR-A37` plus its closing note, in
  `CENSUS-RULINGS.md` § "The second rung: inbox 017 adjudicated" (≈ lines
  7986–9827 as of this date; the heading governs). It adjudicates a second
  received external note claim by claim and delivers **eight** named objects,
  indexed in the family table above.
- **SR-A30's discharge of FT-A28 depends on SR-A33, and the dependency is
  recorded rather than assumed.** The probe's own streaming SHA-256 had a
  buffered-length defect, caught by a FIPS known-answer self-check **before any
  carrier number existed**. That matters because a mis-buffering hash is still
  *deterministic*, so two runs would still have agreed and the digest receipt
  would have been **green and worthless** — a broken compression function may be
  wildly non-injective, and (FT-R7c)'s scope claim ("one scalar reaches every
  individual value across executions") is a statement about **the digest
  function**, not about the probe. The standing discipline: *a receipt whose
  assertion is an equality of digests carries a second, silent obligation — that
  the digest function is anchored to published known-answer vectors covering the
  code path actually used, including the streaming path if the receipt streams.*
  A one-shot-only vector set would have passed here. This is the same family as
  Proposition SR-taut and FT-A28(i): **a check is only a check against something
  it does not itself produce.**
- **The 8.8 GB companion's cross-process digest identity is an AUDIT NOTE of real
  weight, never a receipt.** Its evidentiary surface is far broader than the four
  frontier digests — every depth-two row across two processes — but it is not
  asserted in-run against a transcribed constant, not reproduced by any verify
  path, and does not survive into a future run. It is convertible at zero cost by
  carrying it in the frozen table, which is one of the four items owed below.
- **Freeze 51** (SR-A22(iii)) is the depth-two probe carrier. **Freeze 38 stands
  at v1 with clause (d) exhibited as v1.1(d)** (SR-A21(ii)) — *a clarification
  with no new content*: v1 is not amended and **v2 is not opened**. Both are now
  in [the freeze register](walt-math-freezes.md); 39 and 40 remain reserved.
- **FT-A28 is now FULLY DISCHARGED** by SR-A30 — the deferred frontier digest is
  carried by all four SR units, closing FT-A28(iii)'s named residual **by receipt
  rather than evidentially**. Nothing remains owed on that line.
- **The errata §9 queue has grown again.** DS-A28(ii) is still carried, and the
  queue now holds the seven FT objects *plus* **Lemma SR-coord, Lemma SR-forced,
  Proposition SR-sep, Proposition SR-post, Corollary SR-conv, Proposition
  SR-degen, Proposition SR-taut** and **Proposition SR-loc**, together with the
  confirmed second-rung mathematics of the received note.
- **Four obligations are owed on the next second-rung emission, and nothing is
  owed now** (consolidated at the end of the section; an earlier clause says
  "two" and was written before A33–A36): the escape column's `yes`/`no` case fix;
  the filed binding pairs transcribed into the frozen table and **asserted**,
  which converts a construction into a comparison against a named carrier; the
  companion digest carried in the same table; and — before any rung-three or
  longer-ladder build — a **re-design**, not a re-application, of the
  committed/companion emission split. The companion is 8.8 GB here against the
  previous chapter's 36 MB, and the growth is **not incidental**: the depth-two
  state count grows with the field plies between frontiers, so a longer ladder
  multiplies it again.
- **Grade 4 is exhausted as a test-bed.** Proposition SR-degen bars it from
  testing closure, and there is no rung three there. The next question needs a
  longer ladder, which makes FT-A21's three trick-1 obligations the binding
  constraint rather than a distant destination.

### The FT chapter

- **The `FT-A` family exists and is large.** `FT-A1..FT-A29` plus its closing
  notes, in `CENSUS-RULINGS.md` § "The fusion tax: inbox 016 adjudicated"
  (≈ lines 6567–7984 as of this date; the heading governs, not the line numbers).
  It adjudicates a received external note claim by claim, delivers eight named
  objects (indexed in the family table above), and is the home of the upper-side
  mathematics. **The section now runs to FT-A29** — the file is append-only and a
  section grows after its own closing note, as this one did three times in a day,
  so **read to the end before assuming any ruling is the last one**.
- **FT-A29 files two self-corrections to the section.** The first was found by an
  outside check, the second by a census of the results file: FT-A16(ii)'s "(LD-R4) remains owed" was wrong (the
  receipt had already run and held), and FT-A25(vi)'s commentary "ten of twelve
  pairs" undercounted — **the closure failed at eleven of twelve**. Neither
  touches a verdict, a receipt, a freeze or any results-file number. The
  discipline it yields is worth more than the corrections: **a ruling that creates
  an obligation is not evidence the obligation is still open — only the artifact
  is.** That is "by construction is not a receipt" transposed from claims to
  obligations, and it joins FT-A23(v) and FT-A28(i) as the third instance of one
  failure shape: *asserting a status from a text that governs it rather than from
  the object that carries it.*
- **Two freezes moved, and the versions are not interchangeable.** **Freeze 38
  stands at v1**, scoped — the gluing cut, its reservation discharged at FT-A17;
  feature penalties, multi-stage penalties, adaptive search beyond the first
  frontier and any cost model are explicitly *not* in it and re-enter as v2.
  **Freeze 50 stands at v1.1**, amended twice on the day it was fixed: clause (a)
  at FT-A23 (the enumeration governs, the sort clause struck) and clause (c) at
  FT-A24 (emission cut by content). **39 and 40 remain reserved.** Full text on
  [the freeze register](walt-math-freezes.md).
- **DS-A28(ii) is STILL OWED.** Corollary E4.1's filing as errata §4.3 has been
  carried since SEP-A2 and is still due at the next errata amendment. FT-A27(i)
  added the seven FT objects and the confirmed first-layer mathematics to the same
  queue as a new **errata §9**; the SR chapter has since added eight more, so read
  the SR section above for the current queue. Until that amendment lands,
  `CENSUS-RULINGS.md` is **their only authority**.
- **FT-A28** (≈ lines 7825–7936) ratifies the (FT-R7) two-half discharge,
  versioning the halves rather than renumbering them — **(FT-R7a)** the cross-run
  invariant receipt against a frozen table, whose scope is corrected *upward* to
  reach both `Σ_I δ_I` and `|supp δ_I|` per unit, and **(FT-R7b)** the in-run
  reproduction receipt, which reaches every individual value within one process.
  It names the residual the conjunction does not cover, defers the closure to
  **(FT-R7c)**, and rules that the orchestrator's byte-diff is an **audit note and
  never a receipt**. **All of this is now discharged** — see the SR section above;
  the bullet is kept because the *reasoning* is the precedent, not the status.
- **Three specification defects of one shape, all found by someone else.**
  FT-A23(v), FT-A28(i) and FT-A29(i) are one error three times: **asserting a
  status from a text that governs it rather than from the object that carries
  it**, or equivalently **naming a relation without naming its relata**. A freeze
  clause states a constant *or* a generating rule, never both; a receipt comparing
  against a prior run must name the **carrier** of the reference value — a frozen
  table with its provenance line, or an in-run recomputation — never "the previous
  emission" unqualified; and an obligation-creating clause is not evidence the
  obligation is still open. A prior run is not an object and its results text is
  not an interface.
- **T1-A12's implementation-versus-corpus risk is now inherited by seven
  families.** Every statement in T1-A, LD-A, RW-A, FT-A, SR-A, FF-A and FC-A is proved
  relative to walt's *implementation* of the rules, read from the code at
  adjudication time, and **no receipt inside those sections can detect a
  disagreement with the rules corpus** because every receipt is computed by that
  same implementation. It is sharper at SR: Lemma SR-coord, the hypothesis that
  makes the whole rung-two law true, was itself discharged by reading the
  implementation — and two agreeing traversals inside one implementation cannot
  detect a divergence from the corpus. The corpus check is owed before any of it
  is cited outside walt, and (LD-R4) is a probe of the risk, never a discharge.

  **2026-08-16 boundary:** GT1-A3 addresses T1-A12 for the portable M0/M1 slice
  with a separate prose-rules resolver and complete declared-domain comparisons
  for context/follow/winner/points. That is executable bridge evidence for this
  slice, not a retroactive proof of the seven earlier families and not a Lean
  rules-refinement theorem.


---


## Where the rest lives

- [Received artifacts and intakes](walt-math-intakes.md) — the first-class
  index of the frozen bases, Pro-channel intakes and rebriefs, the seven
  hand-ferried side-channel parents of 2026-08-24 → 09-04, their companions,
  the pinned manifests, and (§9) the three unintaken packet notes: what each
  is, where the verbatim parent lives, what came of it.
- [The freeze register](walt-math-freezes.md) — all 58 issued freezes with
  content, version and declaring ruling (39 and 40 reserved; 58 issued by the
  register alone).
- [Open questions](walt-math-open-questions.md) — what is genuinely
  unresolved, items 1–28, and why none of it belongs in
  [open-problems](open-problems.md).
- [The focal-horizon era](walt-focal-horizon-era.md), [the counted-belief
  era](walt-counted-belief-era.md), [the calculated-evidence
  era](walt-calculated-evidence.md), [the Gran anchors](walt-gran-anchors.md),
  [the partnership program](walt-partnership-program.md), [the
  gym](walt-gym.md) — the programs as records; the measurements this page
  cites are theirs.
- [The walt hub](walt.md) — the build map, sessions, and the exploratory fence.
  Owned by another page; this reference does not restate it.
