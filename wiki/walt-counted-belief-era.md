[Home](Home.md) · owns: the counted-belief and anytime proof-state era, 2026-08-30 → 2026-09-01 — the CBS mathematics and the C→G exact-mass ladder (Slices A–G, PRs #61–#69, freeze 58), the anytime proof-state program (Phases 0–8, PRs #71–#78) and the doom census (#79); the era's diagnosis of the opening root together with its 2026-09-03 correction · Sources: `walt/math/counted_belief_sandwich_v0.1.md` (received parent, SHA-256 `4d2dfbe0…`, adjudicated **CBS-A1..A9** 2026-08-30) and `walt/math/anytime_proof_state_score_v0.1.md` (received parent, `7a8c60fb…`, adjudicated **APS-A1..A9** 2026-08-31) with their intake companions and scratch-tier verifiers ([walt-math-intakes](walt-math-intakes.md)); `walt/CENSUS-RULINGS.md`; [`walt/FACTOR-BELIEF.md`](../walt/FACTOR-BELIEF.md) (the dated running record); `walt/probes/factor_belief/README.md` (the quotable authority for probe findings, CBS-A8) and the records under `walt/probes/{factor_belief,root_interval,grammar_residual}/`; `walt/DISCREPANCIES.md` (the 2026-09-03 correction); the gate files `walt/walt/tests/solver_*.rs`; main's history #60–#80; [the freeze register](walt-math-freezes.md) (freeze 58)

# walt — the counted-belief and anytime proof-state era (2026-08-30 → 2026-09-01)

> **Epistemic tier: EXPLORATORY — below every tier on
> [Home](Home.md#evidentiary-tiers--never-promoted-never-blurred).** Every
> mass, bound, regret figure, wall time and probe number on this page is
> walt-exploratory. A number is quotable as a result only through the gate
> file or verifier receipt named beside it; a number carried only by a probe
> record is labelled so. Nothing here is cited by any claim-tier page. The
> live default player was not touched by anything on this page (CE-A7,
> CBS-A9, APS-A9).

> **Scope.** This page owns three days: the counted-belief intake and the
> C→G ladder (2026-08-30), the anytime proof-state intake and Phases 0–8
> (2026-08-31 → 09-01), and the doom census (2026-09-01). What followed —
> the book-one closing intakes of 2026-09-01, model belief, the God-gap
> censuses, the unified player and the focal-horizon hierarchy — is owned by
> the successor page [walt-focal-horizon-era](walt-focal-horizon-era.md).
> The predecessor is [walt-calculated-evidence](walt-calculated-evidence.md).
> This page replaces the first-pass synthesis committed 2026-09-01 (PR #80,
> `08fe3d2d`); the one sentence of that synthesis that was later corrected is
> quoted, with its correction, in §7.

## What the era is

For a newcomer. A seat choosing a tile in 42 does not know where 21 of the
28 tiles are. At the first trick there are 399,072,960 ways the hidden tiles
can be split among the other three seats. Before this era walt's solver
represented "what might be" as a list of those complete worlds, or a sample
of them, and its answers were estimates with a declared risk
([walt-calculated-evidence](walt-calculated-evidence.md)). In these three
days the solver learned to **count instead of list** (one hidden seat has
only 116,280 possible hands, and the rest is a binomial), to compute exact
integer success masses through those counts wherever a root is affordable,
to **refuse with a typed reason** wherever it is not, and to keep everything
it has learned about one root in an **append-only proof state** whose
deliverable is a certified-regret recommendation: *play this action; the
probability of making the bid under the declared model is at least
`B_exec`; at most `Γ = U* − B_exec` is unclaimed.*

For the mathematician. Two received parents supplied the mathematics, both
adjudicated the day they arrived and both filed verbatim, checksum-pinned.
The counted-belief parent (Pro, hand-delivered by Jason 2026-08-29/30;
rulings CBS-A1..A9) gave the seat-factor posterior closure (Theorem 20.1),
the exact-cover partition functions, the §23 factorized Bellman recursion
cleared of denominators, the §12 grammar/residual decomposition, and the
consequence CEGAR; its Part II (Theorem 5.1) was recognized at intake as
Theorem M1 and Corollary M2 of the x:024 adjudication restated over pmake —
no new statistics (CBS-A2). The anytime proof-state parent (Pro,
hand-delivered 2026-08-31; APS-A1..A9) gave the epistemic container: a
proof state whose facts are the only stored authority and whose closure
(bars, survivors, exclusions, the global upper, the certified regret, the
verdict) is a derived view; the 43-bin score layer; the proof-bar /
executable-bar split; and the typed laydown hierarchy. Both parents keep the
objective **pmake** (ruled 2026-08-17) and the fixed declared field σ0 (the
level-0 modeled mind) as an L2 model choice named in every identity.

For the engineer. Everything landed in-crate under `walt/walt/src/solver/`,
additively, gated at every landing, with a probe record per slice under
`walt/probes/factor_belief/`. The previous controller `RefineV1`
(`solver/refine.rs`) was frozen semantically as **freeze 58** and consumed
only as a frozen oracle. The instruments and how to run them are in
Appendix A; the gate index is Appendix B.

### The three days, dated

| date | landing | PR · commit |
|---|---|---|
| 2026-08-30 | Counted-belief intake: parent and verifier filed, CBS-A1..A9 | #60 · `e4c10ead` |
| 2026-08-30 | Slice A — root intervals, survivor sets, the pmake empirical-max upper | #61 · `a99fcb9f` |
| 2026-08-30 | Slice C stage C0 — the factor belief, `ExactCoverOracle`, backend zero | #62 · `eba8c940` |
| 2026-08-30 | Slice B — the two-policy grammar, the residual split, the §8 identity mechanical | #63 · `2d6eb442` |
| 2026-08-30 | Slice C1 — the cache study: once-per-state laws, zero cross-history sharing | #64 · `bd86c18a` |
| 2026-08-30 | Slice D — `SupportOracle` and the §23 fixed-policy mass recursion | #65 · `8d260844` |
| 2026-08-30 | Slice C2 — the opening-root seven-coordinate report | #66 · `8bea94ca` |
| 2026-08-30 | Slice E — the factorized grammar best response | #67 · `1cae2419` |
| 2026-08-30 | Slice F — consequence CEGAR: mass concentrates, the tail fragments | #68 · `246cb260` |
| 2026-08-30 | Slice G — the integrated controller `refine_root` (RefineV1); the C→G ladder complete in one day | #69 · `25b40d9f` |
| 2026-08-31 | Anytime proof-state intake, APS-A1..A9 | #70 · `aed2a38b` |
| 2026-08-31 | Phases 0 + 2 — freeze 58 and the exact 43-bin score profile | #71 · `8545bab1` |
| 2026-08-31 | The §49 architecture spike — `solver::proof_state` | #72 · `d6320b36` |
| 2026-08-31 | Phase 3 — contract projection and certified regret | #73 · `a1b24d09` |
| 2026-08-31 | Phase 6 — argmax extraction; the h3-t4 gap closed exactly | #74 · `1586ff78` |
| 2026-08-31 | Phase 1 — the work frontier | #75 · `1ceadb7c` |
| 2026-08-31 | Phases 4 + 5 — the residual-Bellman staircase and count-threat covers | #76 · `1ee6669a` |
| 2026-08-31 | Phase 7 — the typed laydown hierarchy | #77 · `a8987fd1` |
| 2026-09-01 | Phase 8 — the §65 opening-root iterative run; the program complete | #78 · `e14c1b35` |
| 2026-09-01 | The doom census | #79 · `eb5a459d` |
| 2026-09-01 | First-pass wiki synthesis (superseded by this page) | #80 · `08fe3d2d` |

Hashes are main's first-parent commits (`git log --first-parent main`,
checked 2026-09-12).

## 1. Counting instead of enumerating (Slice C: stages C0, C1, C2)

**The arithmetic (CBS §22; EXPLORATORY, gate-pinned).** At trick 1 the
focal hand removes seven tiles; the remaining 21 split 7/7/7 across three
seats. The number of complete worlds is C(21,7)·C(14,7) = 116,280 · 3,432 =
**399,072,960**. One particular hidden seat has only C(21,7) = **116,280**
possible root hands, and at the uniform root each such hand has exactly
C(14,7) = **3,432** compatible completions for the other two seats. The
first hidden seat's action distribution is therefore a weighted
classification of 116,280 hands, not a replay of 399 million deals — a
3,432× collapse of the representation. Gate:
`walt/walt/tests/solver_factor_belief.rs::opening_root_contraction_without_worlds`
(the §22 contraction with conservation at 399,072,960) and
`::opening_root_level0_classification_is_once_per_hand` (116,280 hands
classified exactly once).

**Theorem 20.1, posterior closure (CBS §20; EXPLORATORY, adjudicated
CBS-A6, step-checked at intake, gated).** Conditioned on a public history,
the exact posterior over hidden hands has unnormalized weight
`W_h(H) = 1{H covers U} · Π_s φ_{s,h}(H_s)`: a product of seat-local
root-hand factors coupled only by the disjoint-cover constraint, where
observing a hidden seat's action multiplies **only that seat's factor** by
its action likelihood. Boundary, binding: the field must be seat-local
(each seat's action a pure function of its own hand, the public record and
a frozen identity). The shipped level-0 and level-1 modeled minds were
verified seat-local at intake; any cross-seat coupling voids the closure
until represented as explicit factors. Gate:
`solver_factor_belief.rs::condition_recovers_each_branch_mass`. The
intake companion calls this the parent's genuinely new mathematics; its
verifier reproduces it on a 90-deal fixture (Z = 282 both ways, scratch
tier).

**The object.** `FactorBelief` (`solver/factor_belief.rs`) holds one
`HandFactor` per hidden seat over root hands; `ExactCoverOracle` supplies
the exact-cover counts. The existing `FiberDp` (`kernel/fiber.rs`) was
recognized at intake as the uniform-root special case of that oracle
("backend zero", the C0 domain, at most one conditioned factor); the
general contraction `SupportOracle` arrived with Slice D.

**The cost law (probe record `c2_run1.txt`; the timing figures are
probe-only).** Counting is milliseconds; classifying each hand through the
field is the bill. At the opening root h0-t1 under σ0 `Level0{n0=2}`:

| coordinate (`c2_run1.txt`, 2026-08-30) | value as printed |
|---|---|
| 1 — number of acting-seat hands | 116,280 (asserted == 116,280); worlds per hand 3,432 |
| 2 — contraction time | completion weights 5,933 µs; warm contraction (weights + full §43-key cache identity, zero classifications) 21,818 µs; fiber mass alone 63 µs |
| 3 — field-classification time | cold pass 5,361,549 µs; classification alone (cold − warm) 5,339,731 µs; **per hand 45 µs — 99 of the cold pass in percent** |
| 4 — distinct field actions | 20 distinct branch tiles (1-0 mass 125,370,960 … 6-6 mass 3,432, of 399,072,960) |
| 5 — cache reuse | first contraction 5,361,549 µs, 116,280 states; repeat 21,818 µs, 0 states, 0 classifications; identity cost 187 ns/query; saving ×245 |
| 6 — memory | DECLARED accounting for the σ0 action cache 23,563,392 bytes (88-byte entries, 262,144 buckets); MEASURED resident set 63,340,544 bytes at end (`/bin/ps`), classification delta 51,314,688 bytes |
| 7 — exact mass conservation | Σ branch masses = 399,072,960 = fiber mass; completion weights sum to the fiber mass exactly |
| beyond the seven — support shrinkage | conditioning on the heaviest branch 1-0: 15,864 µs, 0 new states, conditioned support 36,530 of 116,280 hands |

The external measurement (`/usr/bin/time -l`, darwin): 5.56 s real,
63,340,544 bytes maximum resident set, 62,390,680 bytes peak footprint. The
record keeps the two memory figures apart by design — "the accounting is
arithmetic; the footprint is a measurement; neither is offered as the
other" — and no CI gate pins C2's coordinates (they are properties of a
run, not laws; PR #66). Read as a law: **every exact recursion's cost is
its σ0 read count**, at roughly 45 µs per read; this unit carries through
the whole program and into the successor era.

**Stage C0 (`run1.txt`, `opening_level0_run1.txt`).** Both routes —
contraction (hands weighted by exact completions) and enumeration (every
world classified) — agree on the one-ply branch masses at all six
enumerable receipt roots under the trivial lowest-first field and under σ0
(gate `branch_masses_match_complete_world_enumeration`,
`level0_field_branch_parity`). At the opening root only the contraction
runs: branch masses in **8,671 µs** under the trivial field (15 branch
tiles), 5,622,659 µs under σ0 (20 tiles), conservation exact. Measured
2026-09-12 on this machine (`factorbelief run`, release binary built
2026-09-07): 0.02 s, the fifteen trivial-field masses identical to
`run1.txt` Section C.

**Stage C1 (`cache_run1.txt`; gated).** Within one history reuse is total
(repeat contraction 23,279 µs, 200 ns/query, zero classifications); across
histories it is exactly **0 of 36 queries** — the full §43 identity key
carries the public history, so sharing needs a proven state reduction,
never a looser key (the PiKey lesson, CBS-A6 binding). Gates:
`classification_is_once_per_information_state`,
`the_full_identity_key_shares_nothing_across_candidates_or_roots`,
`level0_branch_parity_with_the_bundled_one_ply_oracle`. C1 added no library
code; it is the gates and the measurement. This honest negative routed
classifier compression to Slice F.

## 2. Exact masses through a hand (Slices D, E, F)

**Slice D — `V = M/Z` (PR #65; EXPLORATORY, gated).** The §23 factorized
fixed-policy recursion `viewer_success_mass` computes the exact success
mass `M` of one frozen focal policy under the declared field; the value is
the integer pair `M/Z`, no rational anywhere (§23 cleared of denominators by
conservation). Parity with the bundled complete-world walk holds on every
gated row, trick-4 roots included: h4-t4 under σ0 lands 25,039/34,650
(722‰) through 121,868 conditionings; h3-t4 under σ0 2,759/11,550 (238‰)
(`recursion_run1.txt` Section C; gates
`frozen_policy_values_match_the_bundled_walk_under_the_trivial_field` /
`_level0_field`, `every_node_mass_and_branch_parity_with_world_enumeration`).
One law was discovered at depth and is recorded in the ledger's Slice D
paragraph: `condition` restricts its support walk to hands **consistent with
the public record** (own plays contained, others' plays excluded) — such
hands are provably zero-mass and their action likelihood is undefined; σ0's
type-enforced information-state constructor is what caught the unlawful
classification, and at one ply the filter is a no-op, so C1's
conditioning-support law is unchanged. Honest negatives (probe-only): at
worlds/hands ≈ 3 the bundled walk is faster (h4-t4 σ0: 7,172,768 µs
recursion against 2,367,679 µs bundled) and the recursion materializes more
σ0 states (h3-t4: 146,342 against 52,322). No recursion was attempted at
the opening root anywhere in the era.

**Slice E — the grammar max (PR #67; parity gated, the strict-beat numbers
probe-only).** `grammar_success_mass` maximizes over the actions a declared
policy grammar admits at each focal state — lawful because the max is taken
after hidden worlds with the same public history are merged and every
child of a focal node shares its `Z` — and never over the full action set
(the §48 fence). Parity with the Slice B enumeration split holds at every
gated root (gate
`grammar_root_values_match_the_slice_b_split_under_the_level0_field`).
The finding (`response_run1.txt` Section C, trivial field): at trick-4
roots the two-source mix strictly beats every single source —

| root | Z | Q^G | best source | worst source |
|---|---:|---:|---:|---:|
| h3-t4 | 11,550 | 3,815 (330‰) | highest-first 3,062 (265‰) | lowest-first 2,808 (243‰) |
| h4-t4 | 34,650 | **34,650 = Z (certain make)** | 34,170 (986‰) | 30,480 (879‰) |
| h8-t4 | 1,200 | 1,163 (969‰) | 1,073 (894‰) | 975 (812‰) |

and h4-t4 under σ0 gives Q^G = 33,953 (979‰) through 1,652,377 σ0 states in
9.78 s. At trick-5/6 roots the mix never exceeds the best source and the
two-source grammar saturates every reached undecided state (every §12
verdict "closes", no deviating continuation). Phase 6 later explained the
saturation structurally (§5.4 below). Honest negative: the Slice B
enumeration split is 30–40× faster at these small fibers (h8-t5 0-0:
12,330 µs recursion against 365 µs split).

**Slice F — consequence CEGAR (PR #68; Theorem 30.1 nesting, endpoint and
witness gated; the opening-root class numbers probe-only).**
`refine_to_action_exact` partitions the acting seat's hands into §28
feature classes at the field-classification bottleneck and refines only on
a §30 witness pair (two same-class hands with different field actions,
discriminated by a decidable feature — CBS-A7: heuristics propose,
verifiers aggregate). At the opening root under σ0 (`cegar_run1.txt`
Section C; the classification bill paid once at 5,420,302 µs, the 16-stage
refine loop 183,283 µs on cache hits):

| stage | classes | action-exact classes | exact mass (‰ of Z) | max branch width (‰) |
|---:|---:|---:|---:|---:|
| 0 | 314 | 30 | 3 | 828 |
| 6 | 5,387 | 3,434 | 513 | 178 |
| 10 | 36,923 | 31,962 | 805 | 81 |
| 12 | 70,829 | 67,613 | 925 | 32 |
| 15 | 116,280 | 116,280 | 1000 | 0 |

Read both ways. **Mass concentrates**: 513‰ of posterior mass is
action-exact at 5,387 classes (21 hands per class) and 805‰ at 36,923
classes (3 per class). **The tail fragments**: driving residual to zero
costs full fragmentation to 116,280 singleton classes after 15
refinements, with the critical set reaching 15 of the 21 pool tiles. On the
same roots the trivial-field endpoints do aggregate (h3-t4 246 of 330
hands, h4-t4 255 of 495, h8-t4 56 of 126, h12-t4 147 of 495 —
`cegar_run1.txt` Section B), so the fragmentation is a property of σ0's
sampled mind, not of the vocabulary. The lesson the record states: carry a
small residual as an interval, never chase the endpoint (the
residual-interval discipline the parent's §51 falsifier predicted). The
§29 action-exact class verifier is named and was never built. Gates
(`solver_factor_consequence.rs`, 4):
`refinement_narrows_monotonically_to_the_exact_endpoint`,
`the_refined_endpoint_reproduces_the_exact_contraction`,
`every_refinement_carries_a_valid_witness_pair`,
`the_base_vocabulary_carries_mass_and_classes_aggregate`.

## 3. Root intervals and survivor sets (Slices A, B, G)

**Vocabulary (CBS-A3).** The parent's title uses a word this project had
already retired for two adjudicated objects (the decision-sparse E6.3
"value sandwich" and the refuted T1-A bounded object); the ruling adopted
**root interval** `[L_a, U_a]` per legal root action and **survivor set**
(the actions whose upper clears the bar `B = max_a L_a`), and the parent's
title survives only as a citation. The same ruling was repeated for the
successor parent (FH-A2). See [vocabulary](vocabulary.md).

**Theorem 5.1 is adopted, not new (CBS-A2).** The optimization-lock upper
confidence sequence over pmake is Theorem M1 plus Corollary M2 of the x:024
adjudication (TRIPLE-A2) restated over `Π_a`; no `|Π_a|` factor enters the
risk; the shipped inversion (`solver/upper_cs.rs`) and `ScopedDelta` ledger
apply as adopted. Slice A's one green-field producer is the pmake-objective
sampled optimizer (`sampled_root_optimum`), the declared-prefix sibling of
`exact_root_value` with branches run to decided terminals; Corollary 5.2's
one-way rule binds it (a pathwise upper approximation to S* is admissible,
a lower one is not). Gate
`solver_root_interval.rs::the_mirror_endpoint_is_the_complement_and_the_lower_sweep_matches_the_adjudication`
(worst undercoverage 11/128 at δ = 1/4, the verifier's exhaustive sweep
re-derived).

**Slice A (PR #61; `walt/probes/root_interval/run1.txt`, coverage gated by
`intervals_cover_the_exact_values_and_the_exact_optimizer_survives`).** Six
trick-5/6 receipt roots, prefix 16, δ = 1/20 per endpoint, pinned level-1
`[2,2]` witness:

| root | fiber | decision | reading |
|---|---:|---|---|
| h4-t6 | 90 | `DeltaRootWinner{1-1; bar 7/10}` | the exact optimum (13/15 vs 1/3) settled after **8 sampled worlds** |
| h8-t5 | 92 | `DeltaRootSet{2 of 3; bar 35/46}` | 5-0 (16/23) excluded at t = 15; 5-3 (91/92) and 0-0 (71/92) still entangled |
| h10-t6, h5-t6, h12-t6, h3-t5 | 19, 27, 6, 200 | `UnresolvedRootSet` | exact ties (Q = 1, 4/9, 0, 1) — the parent's §40.7 "true decision hardness"; the typing refuses to invent a winner |

All 14 action rows have L ≤ Q ≤ U. Measured 2026-09-12 on this machine
(`rootinterval run`): 0.19 s, the same six decisions.

**Slice B (PR #63; `walt/probes/grammar_residual/run1.txt`; the §8
identity and the exact split gated in `solver_grammar.rs`, 8 gates).** The
§12 triple (free / gram / dev, with free = max(gram, dev) asserted at every
node) over grammars G1 = {lowest-first}, G2 = {lowest-first, highest-first},
G3 = {pinned level-1 [2,2], the σ0 mind, count-preservation}. On all six
exact fixtures G2 and G3 attain the exact root optimum; the singleton G1
fails root closure at h4-t6 (30 vs 78) and h8-t5 (64 vs 91), and its one
counterexample carries a lazy first-deviation witness (h8-t5 0-0: after
`0-0,3-0,6-0,6-6` the optimal line plays 5-3, depth 4). The §12 exclusion
is realized with margin at h8-t5 under G3 (0-0: gram 71, dev 70; 5-3:
gram 91, dev 90). The §8 identity holds in the numbers: the residual
empirical-max upper and the full-class upper have byte-identical count
paths (gate `residual_upper_is_the_full_class_upper`). The §11 non-theorem
— pointwise fusion is not lawful coverage — is the O34 strategy-fusion
fence restated (CBS-A4): two seed policies give two lower witnesses and a
grammar, never coverage of omitted policies.

**Slice G — RefineV1 (PR #69, `25b40d9f`; escalation parity gated at every
root and action).** `refine_root` (`solver/refine.rs`) keeps one typed
interval per legal root action (sampled δ bounds carrying their Slice A
record; exact bounds as integer masses over the shared `Z`), excludes an
action exactly when its upper falls below the bar, runs the §36 loop over
the buildable §33 work items — `SampledLower`, `SampledUpper`,
`ExactFixed`, `ExactGrammar`, `EscalateExact` (the §36 endpoint
`response_success_mass`, the full-action-set recursion), `ConsequenceCensus`
(carried to demonstrate §34's refusal) — under §34 refusals
(`ExcludedAction`, `PresentlyUseless`, `ExceedsBudget`, `ExceedsRiskScope`)
and §35 width-per-cost scheduling on `D = (|survivors| − 1) + Σ (U_a − B)`.
Budgets charge **declared forecasts**, never wall time, so every run is a
pure function of its inputs. Results are typed `Settled` / `Equivalent`
(deterministic point intervals at the bar) / `Unresolved` with a **named
fallback never promoted** (§37.9), and `DeltaQualified` whenever a sampled
side took part in a decisive exclusion. Gates (`solver_factor_refine.rs`,
4): `escalation_matches_the_bundled_exact_authority`,
`the_controller_narrows_monotonically_and_soundly`,
`steering_refuses_presently_useless_work_deterministically`,
`budget_exhaustion_returns_the_honest_surviving_set`.

The record (`refine_run1.txt`) shows three regimes:

1. **The exact-only ladder settles all ten gated roots** (Section A): five
   `SETTLED` (h4-t6 1-1, h8-t5 5-3, h3-t4 3-1, h4-t4 6-5, h8-t4 3-3) and
   five honest `EQUIVALENT` ties (h12-t6, h10-t6, h5-t6, h3-t5, h12-t4),
   risk 0. In every settled case the winner's own upper stays vacuous: at
   h4-t6 (work 420) and h4-t4 (118,800) the winner's exact fixed-policy
   lower alone cleared every rival's escalated point, and at h8-t5 (13,860)
   and h8-t4 (163,350) the winner's grammar lower did. *Note: the ledger
   paragraph in `walt/FACTOR-BELIEF.md` says "six SETTLED, four honest
   EQUIVALENT ties"; the record prints five and five, and the record wins.*
2. **The sampled tier settles small fibers before any exact recursion
   runs** (Section B, prefix 16, δ = 1/20): h4-t6 at 64 work units against
   420 exact-only, h8-t5 at 3,776 against 13,860, both correctly
   δ-qualified; at trick 4 the sampled uppers are too loose to prune and the
   exact ladder does the work.
3. **The opening root walks the affordability cliff honestly** (Section D,
   budget 100,000): every exact item is refused by its own declared
   forecast, fourteen sampled endpoints produce real intervals over the
   399,072,960-world fiber (bar 490‰; one sampled upper 974‰ on 3-2), 7 of 7
   actions survive, result `UNRESOLVED` with fallback 0-0 named and never
   promoted, risk 7/10, wall 12.0 s.

Exact values pinned by this record (gate 1 recomputes them against
`exposure::exact_root_value`): h3-t4 3-1 = 1349/3850 (350‰), 4-1 =
1047/3850 (271‰), 4-4 = 3797/11550 (328‰), 6-4 = 331/1155 (286‰); h4-t4
2-1 = 9131/11550, 4-0 = 2936/3465, 5-1 = 1511/1925, 6-5 lower 442/495
(892‰); h8-t4 2-1 = 287/300, 3-1 = 67/75, 5-5 = 283/300, 3-3 bar
1163/1200 (969‰). Walls: h4-t4 19.9 s exact-only / 30.8 s two-tier; h3-t4
8.9 s / 10.0 s; h8-t4 2.7 s / 3.5 s.

**Freeze 58 (APS-A9, 2026-08-31).** `solver/refine.rs` as merged at main
`25b40d9` is the semantically frozen RefineV1 reference: no new fields,
variants or work items, ever; its four gates never weaken; the proof-state
core must reproduce it wherever scopes overlap and stays removable. Entry
58 in [walt-math-freezes](walt-math-freezes.md). (The successor era's map
already lists `refine.rs` as removable once the focal-horizon hierarchy
landed — see [walt-focal-horizon-era](walt-focal-horizon-era.md).)

## 4. The proof state

Plainly: a proof state is the file walt keeps about one root. It only ever
grows. It stores facts; everything a reader wants — which actions survive,
the floor, the ceiling, the recommendation — is computed from the facts and
never stored beside them.

Precisely (`solver/proof_state.rs`, PR #72; EXPLORATORY, six gates):

- **Facts are the only stored authority; closure is a derived view.** A
  `ProofState` is opened under a `SemanticsIdentity` of eight §51
  coordinates (root, rules, field, utility, contract, belief, policy
  class, score semantics); every fact carries a `ProofTag`
  (`Deterministic`, or `Sampled{scope, δ}` with its full scoped risk
  identity). The fact vocabulary is closed — `Bound`, `Profile`,
  `Envelope`, `Cover` — and the producer registry is open: a
  `ProofProducer` defined in a test file registers without editing any
  enum (gate `a_producer_registers_without_editing_the_module`).
- **Serialization** is the line format `walt-proof-state-v1` with 128-bit
  FNV-1a content hashes; a zero-budget state serializes and resumes
  bytewise, and stored facts round-trip under re-validated hashes while
  identity mismatches in any coordinate are rejected (gates
  `the_zero_budget_state_is_sound_and_serializes_and_resumes`,
  `fences_reject_and_stored_facts_round_trip`).
- **Closure** yields bars, survivors, exclusions, the global upper `U* =
  max_a U_a`, the certified regret and the typed result; it is idempotent
  and insertion-order-independent (gate
  `closure_is_idempotent_and_insertion_order_independent`), and RefineV1's
  endpoints imported as facts reproduce its verdicts on every enumerable
  root (gate `imported_refine_v1_facts_reproduce_the_controller_verdicts`).
- **Proof bar and executable bar (APS §30, APS-A6).** `B_exec` is the max
  over lowers witnessed by a *materialized* lawful policy; `B_proof ≥
  B_exec` also counts grammar and full-response optima. The intake audit
  found RefineV1's `bar_of` maxes over all lowers including `ExactGrammar`
  — the shipped bar is exactly the proof bar, sound for action selection —
  and the distinction becomes load-bearing the moment a *policy* is
  recommended: a grammar lower enters `B_exec` only after argmax extraction
  and fixed-policy re-pricing (gate
  `a_score_profile_fact_raises_the_executable_bar_through_closure`, with
  `B_exec ≤ B_proof` asserted inside every closure).
- **Certified regret (APS §31, APS-A7).** `Γ = U* − B_exec`. On the
  joint-validity event, `0 ≤ Q* − V(π̂) ≤ Γ`; Γ never increases under
  refinement (`U*` never rises, `B_exec` never falls); `Γ ≤ ε` certifies
  ε-optimality of the recommended executable policy under the declared
  field and belief. The §33 recommendation block — action, policy, pmake
  floor, global upper, Γ, declaring score floor/ceiling, the §7 residual,
  the §10/§11 d = 1 bands, proof class, sampled-scope summary — is the
  report shape, with the pmake floor primary (a narrow low score interval
  never outranks a higher make floor). Gates in `solver_proof_regret.rs`:
  `certified_regret_contains_exact_best_response_regret`,
  `regret_never_increases_under_monotone_refinement`.
- **A note on the word.** "Certified regret" is walt's term of art for the
  quantity `Γ` (APS-A7; reaffirmed by FH-A2). It is a bound with a declared
  scope, not a document; the project's D3 rule on the word "certificate"
  ("necessary outer profile", never "certificate") is unaffected and was
  checked at intake not to collide.

The spike's verdict: the in-crate shape is confirmed; nothing imports the
module except the crate root and its new-core siblings, so the whole core
is deletable as one boundary (APS §67.10).

## 5. Eight phases in two days

Each phase names its gate file; probe walls are probe-only.

### 5.1 Phases 0 + 2 — freeze 58 and the score profile (PR #71)

Phase 0 is freeze 58 (§3). Phase 2 is `viewer_score_profile`
(`solver/factor_belief.rs`): the same factorized recursion carrying the
full **43-bin** exact score object — bin `s` = the exact world mass banking
exactly `s` declaring-team points — instead of one tail sum. The profile
never reads the bid, so one bid-blind run yields the whole bid-threshold
curve; its price is the forgone decided cutoff, measured at **~7–12% extra
wall at trick 4** for roughly double the nodes (`profile_run1.txt`: h3-t4
σ0-as-focal 1,148,591 µs against 1,023,334 µs success-mass; h4-t4 σ0
3,315,858 against 2,954,635). Certain outcomes carry their explanation
(h12-t6: the single bin 20:6, a miss by exactly 20 points in every world;
h10-t6: bins 35:1 41:6 42:12). σ0's make mass spikes at the bid (h8-t5
σ0-as-focal: 445‰ fragile within one point, 30:41 in the bins).
**Cross-contract reuse is void under the bid-reading σ0** — the frozen
specimen at h10-t6 threshold 42: projection 12 ≠ evaluation 9. Five gates
(`solver_factor_profile.rs`): mass conservation with tail projection, the
§3 tail-sum identity, bid-blind reuse, the reuse boundary specimen, parity
with complete-world replay. A profile is the record of one policy; no
envelope across policies exists anywhere (APS-A4).

### 5.2 The §49 spike — `solver::proof_state` (PR #72)

Section 4 above; six gates in `solver_proof_state.rs`. No probe: the gates
were the deliverable.

### 5.3 Phase 3 — contract projection and certified regret (PR #73)

The closure gains `U*`, `Γ` and a full executable witness;
`ProofState::recommend()` derives the §33 block. Five gates
(`solver_proof_regret.rs`). The first blocks on real roots
(`proofreport_run1.txt`, RefineV1 two-tier prefix 16, δ = 1/20, scope 1/2,
plus lowest-first continuation profiles):

| root | recommendation | floor | upper | Γ |
|---|---|---:|---:|---:|
| h12-t6 | 4-4 | 0‰ | 0‰ | 0‰ |
| h10-t6 | 2-2 | 1000‰ | 1000‰ | 0‰ |
| h5-t6 | 4-1 | 444‰ | 444‰ | **0‰ — optimality certainty far from make certainty** |
| h4-t6 | 1-1 | 866‰ | 1000‰ | 133‰ |
| h8-t5 | 5-3 | 717‰ | 1000‰ | 282‰ |
| h3-t5 | 4-0 | 1000‰ | 1000‰ | 0‰ |
| h3-t4 | **4-4** | 267‰ | 350‰ | **83‰** |

The h3-t4 row is §30 on trace: the settled best *action* is 3-1
(Q = 350‰) but the best *materialized* policy starts 4-4 at floor 267‰,
because 3-1's naive lowest-first continuation prices below 4-4's. **pmake
belongs to the policy, not the first tile.** Probe wall 14.1 s, 13.35 s of
it h3-t4.

### 5.4 Phase 6 — argmax extraction (PR #74)

`extract_success_policy` returns, beside the optimum, one policy attaining
it — the argmax DAG under the declared lowest-tile-index tie rule, keyed by
post-root history, completed off-DAG by the same rule, re-priced unchanged
by the fixed-policy evaluators (`ExtractedPolicy` is an ordinary
`SlicePolicy`; its id is a content address of the choice table — one
realizable policy, never an envelope). `residual_split` is the §63 exact
`(M*, D)` pair. `ExtractionProducer` (`solver/extraction.rs`) is the first
shipped producer: extract, re-price through `viewer_score_profile`, install
under the extracted content id — the §30 bridge into `B_exec`. Six gates
(`solver_extraction.rs`), the last asserting `B_exec = B_proof` after the
producer at every root. The record (`extractreport_run1.txt`): **h3-t4's Γ
83‰ → 0‰ exactly** — a 12,420-state DAG extracted in 1,139,726 µs,
`B_exec` 267‰ → 350‰, the recommendation switches 4-4 → 3-1 under
`profile:argmax-full-legal-5357e0c9…`; h8-t5 Γ 282‰ → 10‰ (the residue is
the winner's δ-tier upper 1000‰); h4-t6 keeps Γ = 133‰ honestly. Two
discoveries the gates forced: RefineV1 settles on cross-action dominance,
so a winner's own upper can stay vacuous and Γ stays positive at a settled
root until an upper fact prices it; and Slice E's t5/t6 "ties free" is
**structural saturation** — post-root focal states there hold ≤ 2 tiles, so
the deviating class is literally empty (`empty-class` on every
multi-source t5/t6 verdict), while at trick 4 the two-source grammar
escapes on every h3-t4 action (m* = 4047 > gram 3498 on 3-1). Probe wall
29.2 s.

### 5.5 Phase 1 — the work frontier (PR #75)

`solver/frontier.rs`: four typed goals with debts in their own units —
`SelectAction`, `RecommendEpsilonPolicy(ε)`, `StrengthenToExact`,
`ComputeFullScoreProfile` — four deterministic work items (baseline
profile, §36 exact value, §63 targeted extraction, and the §41 macro
`ExactValueSurvivors`, the only lawful mover of the upper side from the top
state), a declared cost model (Z per fixed-policy walk, 3Z per max walk;
forecasts, never measurements), and `Frontier::advance`: refuse
zero-potential items, buy the best bound-per-cost, install through the
ordinary fence, assert the §42 law on every purchase. Nothing reads a
clock. Six gates (`solver_frontier.rs`) incl. §43 containment and
resume ≡ uninterrupted. The record (`frontierreport_run1.txt`): goal
separation is real money — h10-t6 and h3-t5 certify Γ = 0 for one baseline
at 1Z while `SelectAction` costs 7–10Z; h3-t4 `SelectAction` settles at 16Z
without buying any extraction (recommendation 4-4 at Γ = 83‰) while the
ε-goal pays 28Z and reads 3-1 at Γ = 0; h4-t6 `SelectAction` is 5Z. One
honest waste recorded: under vacuous uppers the §42 bounds cannot rank
extractions, so h3-t4's ε-goal bought all four before the macro priced the
uppers — 28Z where uppers-first would pay about 15Z; §43 verbatim, a poor
forecast wastes budget and cannot weaken the proof state. Probe wall
13.3 s.

### 5.6 Phases 4 + 5 — the residual-Bellman staircase and count-threat covers (PR #76)

Phase 4 (`staged_response_interval` / `staged_policy_envelope`, producer
`solver/residual.rs`, `RESIDUAL_STAGE = 4` declared): the Slice F
staircase at each path's first field decision, exact classes merged by
public action and recursed exactly, the unresolved mass attached as the §5
envelope — the interval width *is* the residual mass. The §23 cellwise-max
counterexample is rejected live on h3-t4 (gate
`merged_branches_are_law_and_the_cellwise_max_counterexample_is_rejected`).
Phase 5 (`solver/covers.rs`): `Fact::Cover` carries the §13 resource
decomposition (contested tricks and named count tiles, decomposing the §5
remainder exactly) with a verified uniform movement bound from
`declaring_score_range`; closure derives the §10/§11 rescue-band upper; no
incumbent profile → decline → no number. Ten gates (`solver_residual.rs` 6,
`solver_covers.rs` 3). The record (`bellmanreport_run1.txt`, total wall
15,696,614 µs): h3-t4 action 3-1 walks [66,830]‰ → [168,640] → [275,457] →
[279,425] → [326,374] → [350,350]‰ in five stages, exact mass rising 236‰ →
1000‰; 4-1 walks [145,606]‰ → [271,271]‰ (*the ledger's "[145,606]‰"
quotes action 4-1's stage 0; 3-1 starts at [66,830]‰*); h8-t5 0-0 climbs
86‰ → 771‰ exact across five stages. Covers: h12-t6's verified gain 0
(against an arithmetic envelope of 7) certifies V* = 0 for one range walk;
h4-t6's range walk beats arithmetic by exactly one point (gain 11 vs 12)
and leaves the 5-5 ten-count hazard visible; at rich roots (h8-t5, h5-t6,
h3-t4) gain equals the envelope and first-generation covers are vacuous —
the §70 caveat live and recorded, not patched.

### 5.7 Phase 7 — the typed laydown hierarchy (PR #77)

`universal_viewer_success` is one Boolean walk with three focal quantifier
regimes (Fixed π / Exists / All) under the universal field quantifier ∀σ,
hidden nodes branching over every could-play tile with overlapping
posteriors — a per-seat relaxation of the world set, sound in the
certifying direction, possibly conservative. `solver/laydown.rs` classifies
a root into the four §16 tiers — `PolicyCertainMake` (∀ω, one π, one σ),
`AdversarialPolicyMake` (∀ω ∀σ), `ForcedMake` (∃π ∀ω ∀σ, with witness),
`Laydown` (∀ω ∀π ∀σ) — and `LaydownProducer` installs only
`ProofTag::Deterministic` facts: no sampled route constructs any of the
four, and bare "laydown" is reserved for the universal type (APS-A5). Four
gates (`solver_laydown.rs`). The record (`laydownreport_run1.txt`): the
boss-chain control is a true `Laydown` in 1,492,276 walk nodes / 373,469
µs, proved by walk not phrase; already-made classifies in 3 nodes / 10 µs
(§17's zero-cost closure); loose-boss refutes fail-fast in 280 nodes;
**h10-t6 is a real receipt-root `Laydown`** (all four tiers, forcing
witness 2-2); h12-t6, h5-t6, h4-t6 are not. The walk is an endgame
instrument, exponential in remaining plays. Measured 2026-09-12 on this
machine (`laydownreport report`): 0.45 s, the same seven verdicts,
boss-chain 1,492,276 nodes in 420,220 µs.

### 5.8 Phase 8 — the opening-root iterative run (PR #78)

Section 6.

## 6. The opening root, honestly

The parent's own target, verbatim (APS §65):

> The first target is not a seven-trick exact opening solution.
>
> The first target is a materially smaller correct survivor set or a useful certified-regret recommendation under a playable budget.

`solver/opening.rs`: `OpeningLadder::run_stop` executes the §65 steps in
declared order against one append-only proof state — sampled root bounds
at each stop's prefix (the Slice A endpoints under fresh per-stop δ
scopes, imported through the ordinary adapter; the pinned-level-1 witness
is the "cheap executable policy"), the frontier pass at a declared Z
budget, count-threat covers through their producer, the §49 census as a
reported coordinate at the Phase 4 stage, then the full §65 panel with the
typed verdict (exact / δ-qualified / ε-optimal / unresolved). It reads no
clock. Five gates (`solver_opening.rs`): the top state with the §67.4
bytewise round trip; monotone narrowing with an exact derived risk ledger;
§67.5 resume ≡ uninterrupted, panel for panel and byte for byte; the honest
cliff at h0-t1 (buys nothing, installs nothing, never manufactures a
winner); ample-budget settlement on enumerable roots with a
mass-conserving census.

**The record (`openingreport_run1.txt`; probe-only numbers, the ladder's
laws gated).** Root h0-t1: Z = 399,072,960, seven legal leads, contract 30
on threes, σ0 `Level0{n0=2}` under `SupportOracle`, **δ = 1/100 per
endpoint** against a root risk scope of 3/5, **ε = 1/4**, census at stage 4,
frontier budgets in Z units.

| stop | wall | facts | proof bar = exec bar | `U*` | Γ | recommendation | risk spent |
|---|---:|---:|---:|---:|---:|---|---|
| 0, zero budget | 69 µs | 0 | 0‰ | 1000‰ | 1000‰ | none (no executable witness yet) | 0 |
| 1, p = 16 | 5,682,965 µs (5.7 s) | 14 | 407‰ | 1000‰ | 592‰ | 0-0 | 7/50 (14 scopes) |
| 2, p = 64 | 29,826,738 µs (30 s) | 28 | 594‰ | 1000‰ | 405‰ | 2-1 | 7/25 (28 scopes) |
| 3, p = 256 + census + frontier Z/2 | 631,739,549 µs (10.5 min) | 42 | 732‰ | 1000‰ | 267‰ | 6-5 | 21/50 (42 scopes) |
| 4, p = 512 + frontier Z/2 | 4,559,438,863 µs (76 min) | 56 | 732‰ | **999‰** | 267‰ | 6-5 | 14/25 (56 scopes) |

Read the table. The bar climbs 0 → 407 → 594 → 732‰ while Γ falls 1000 →
592 → 405 → 267‰ and the recommendation migrates 0-0 → 2-1 → 6-5 as
evidence deepens; nothing prunes (7 of 7 survive at every stop). Between
p = 256 and p = 512 the bar and Γ do not move while the last vacuous uppers
break (`U*` 1000 → 999‰: every sample-fitted optimum finally lost at least
one of 512 worlds) — **the sampled tier plateaus**, and the record says the
remaining Γ is purchasable only by structural work, not by more prefix.
At stops 3 and 4 the frontier executes nothing: all **29 refusals**
(baseline-profile, exact-value, extract-argmax and residual-interval for
each of the seven leads, plus exact-value-survivors) are pure
affordability — "exceeds remaining budget" — the cliff, not the §41 stall.
The stage-4 census reads 159–444‰ exact mass across the seven leads
(richest after 5-3 at 444‰, thinnest after 6-5 at 159‰): the opening field
is far from action-exact at the declared vocabulary depth, Slice F's
fragmentation from a new angle. The whole opening-root proof state is
**56 facts, 10,439 bytes** (`walt-proof-state-v1`). The per-stop walls are
the optimization-lock upper's cost growing quadratically in the prefix
while its declared forecast unit is linear; a staged cost model for the
sampled tier is priced but unbuilt. The values differ from RefineV1's
Section D at the same prefix because Section D declared δ = 1/20; this is
stated in the instrument, not drift.

**What "UNRESOLVED at ε = 1/4" means as a playable recommendation.** The
verdict at every stop is honest UNRESOLVED because Γ = 267‰ exceeds ε =
250‰. The recommendation stands anyway and is the deliverable: *lead the
6-5; under the declared σ0 field and the uniform root belief, the
materialized policy makes 30 with probability at least 732‰ (δ-qualified at
1/100 per endpoint, risk 14/25 spent over 56 scopes), no lawful policy can
exceed 999‰, and the 267‰ between is what the evidence cannot yet claim —
not a measured loss.* This is §65's first target met in its
certified-regret form; what the 267‰ is made of is the question §7 records
and the successor era measures.

## 7. Doom, God, and the price of playing blind

**The instrument (`solver/doom.rs`, PR #79; eight gates in
`solver_doom.rs`).** Counterexample **mass** as a deterministic upper — the
∀-fail dual of Phase 7's laydown walk. `universal_viewer_failure` walks
focal nodes AND over every viewer escape (one survivor kills the class) and
hidden nodes as a partition of the record-consistent support by the
declared σ0's deterministic choice; `doom_census` descends the §28/§49
signature vocabulary over one to three hidden seats with exact oracle
masses (a full mode under the §46 partition law and a declared
punish-priority mode for rich roots); `doom_enumeration` is the per-world
ground truth (a singleton class is a belief, so the exact recursion is a
world-aware make check); `DoomCensusProducer` installs uppers
`(Z − M_doom)/Z` under `ProofTag::Deterministic` with the field identity in
the authority string. Nothing sampled exists on any doom path. Gates:
`an_already_set_root_dooms_the_whole_fiber_at_zero_cost`,
`the_census_meets_the_exact_recursion_on_the_loose_boss` (1120/1680),
`doom_uppers_are_sound_on_the_enumerable_receipt_roots`,
`a_starved_budget_refuses_honestly_and_deterministically`,
`the_producer_installs_uppers_the_closure_consumes`,
`the_enumeration_meets_the_census_on_the_loose_boss`,
`the_enumeration_dominates_the_census_and_stays_sound`,
`the_priority_census_is_a_sound_partial_harvest`.

**Where doom lives, the census harvests it (`doomreport_run1.txt` Part 1;
soundness gated, the recovery figures probe-record).**

| root | action | census certified | per-world truth | recovery |
|---|---|---:|---:|---:|
| h12-t6 | 4-4, 6-0 | 6 of 6 (one decided read) | 6 | 1000‰ |
| h10-t6 | 2-2, 3-3 | 0 | 0 | nothing to find |
| h5-t6 | 4-1, 5-2 | 15 of 27 | 15 | 1000‰ |
| h4-t6 | 0-0 | 56 of 90 | 60 | **933‰** |
| h4-t6 | 1-1 | 12 of 90 | 12 | 1000‰ |
| h8-t5 | 0-0 | 17 of 92 | 21 | **809‰** |
| h8-t5 | 5-0 | 28 of 92 | 28 | 1000‰ |
| h8-t5 | 5-3 | 0 of 92 | 1 | **0‰ — the single doomed world missed** |
| h3-t5 | 4-0, 5-0, 6-6 | 0 | 0 | nothing to find |

Stated exactly: all doom is recovered on h12-t6, h5-t6, h4-t6 1-1 and
h8-t5 5-0; 933‰ on h4-t6 0-0; 809‰ on h8-t5 0-0; and 0 of 1 on h8-t5 5-3
(the per-seat relaxation's price, printed by the instrument itself). The
class census is one-sided: a certified harvest never exceeds the truth,
asserted per coordinate. These three truth-vs-census divergences are the
subject of the second entry in `walt/DISCREPANCIES.md` (2026-09-02): the
salvation-complex intake companion counted two, the record carries three,
and the successor era's gate
`solver_godgap.rs::the_section_nine_table_is_re_derived_from_the_committed_record`
now pins all three by exact value.

**At the opening root the census certifies zero, and the God grid shows the
zero is real (Parts 2–3).** All seven h0-t1 leads: 0 doomed worlds of
399,072,960 under the declared top-8 priority descent (node budget 500,000,
walk cap 100,000, max level 3), 41–55 s per lead; the trump leads 3-2 / 3-3
/ 5-3 exhaust their priority region with no refusals, 6-0 and 6-5 refuse at
the node budget inside their first huge classes. Diagnosis by singleton
checks: two adversarially hand-built crusher worlds after the 0-0 lead
(opponents holding the top pool trump, both tens and both loose fives with a
junk partner; a trump wall over the viewer's 3-5) both let the world-aware
viewer make 30 against σ0, and a declared stride-512 grid over the S2
support finds 0 of 228 grid worlds doomed — a structured grid, not a
probability estimate. The composed panel (p16 + p64 sampled stops, then 0
doom uppers installed) leaves Γ at 405‰ unchanged.

**The diagnosis as first written, and its correction.** The ledger
paragraph of 2026-09-01 (`walt/FACTOR-BELIEF.md`) closed with:

> A doom-family upper is floored at the God make rate, and that rate is ≈ 1 here: the plateau's remaining Γ ≈ 267‰ is overwhelmingly the INFO-CONSISTENCY PRICE — purchasable by floor work (extraction across the cliff) and info-consistency-aware uppers, never by counterexample counting.

The first-pass version of this page (2026-09-01) restated it as "so the
plateau's Γ ≈ 267‰ is overwhelmingly the info-consistency price of playing
blind, purchasable by floor work and info-consistency-aware uppers, never by
counterexample counting". Both outran the mathematics adjudicated the same
day. The salvation-complex parent (`walt/math/salvation_complex_v0.1.md`
§8, adjudicated SC-A1..A8 on 2026-09-01, see
[walt-focal-horizon-era](walt-focal-horizon-era.md)) decomposes failure
exactly:

> `1 − V_c(ρ) = d_phys + d_info + d_policy(ρ)`
>
> with `d_phys = 1 − U^God = β(D)` (physical doom: how much world mass cannot be saved even with full world knowledge), `d_info = U^God − Q` (information price: how much individually saveable mass cannot be saved simultaneously by one blind policy), `d_policy(ρ) = Q − V_c(ρ)` (policy gap: how much value the current lawful policy leaves below the best blind policy). **A zero doom census affects only the first term.**

The correction is filed in `walt/DISCREPANCIES.md` (2026-09-03, "doom-census
ledger paragraph: 'overwhelmingly the info-consistency price' outruns what
was established"), and the ledger carries a bracketed pointer to it:

> The salvation parent adjudicated the same session (SC-A1) states the correction in its §8–§9: a zero doom census moves only `d_phys` and "does **not**, by itself, prove that the remaining gap is information-consistency price" — the unclaimed mass is `d_info + d_policy`, and zero doom does not distinguish them. U0 then typed the opening root `UnknownGodGap` on all seven actions, claiming nothing about either term (SC-A4). The ledger sentence therefore overclaims relative to the mathematics adjudicated the same day.
>
> What the later evidence suggests, without settling it: U0's twelve trick-4 information prices are 6–22‰ with `d_policy = 0` at every one, and U0b's in-solve census finds the trick-5 frontier's mass-weighted price at 13–14‰ under the trick-4 roots; the opening upper of 999‰ is a 512-world sampled optimization lock, not a doom bound. The honest statement is that the 267‰ is UNKNOWN in its split, with the sampled upper's looseness and the policy gap both live candidates.

So the standing statement of this page is: at h0-t1 the census certifies
zero doom and the God grid shows the zero is real; per SC §8 the remaining
267‰ is nonphysical unresolved mass `d_info + d_policy` **whose split is
unknown** (U0, 2026-09-02: `UnknownGodGap` on all seven leads); the
successor era's trick-4 and trick-5 measurements (Φ 6–22‰ with
`d_policy = 0` at trick 4; 13–14‰ mass-weighted at the trick-5 frontier;
at k ≥ 1 of the focal-horizon hierarchy the residual is the tail's policy
gap, not fusion price) are suggestive, not settling, and are owned by
[walt-focal-horizon-era](walt-focal-horizon-era.md).

**The census's working domain** is the endgame and the in-play middlegame
(t4–t6), where fibers are enumerable-adjacent and σ0 reads are cheap — the
same domain as Phase 7's walk, and one every played hand reaches. The
opening root priced the wall honestly: each non-forced σ0 read is a
modeled-mind mini-solve (about 2.5–9k walk nodes per second at trick-1
depth), the field-classification bottleneck seen from the doom side. The
salvation-complex ruling SC-A3 preserved the census as it stands
(singleton-conflict producer, God-upper ground truth on enumerable roots,
suffix-candidate detector, the empty-mask base case of salvation masks) and
declined to broaden it at the opening root; the next upper work was ruled
information-consistency-aware.

## 8. Boundaries the era never crossed

Stated in every module doc and in the probe README's "Boundaries":
deterministic fields only (a stochastic field needs an explicit tape factor
— CBS-A6 — and has no entry point); no exact recursion at the opening root
(only one-ply contractions and the sampled tier ran at h0-t1); the Slice D
recursion evaluates one frozen policy and the Slice E recursion maximizes
over grammar actions only, the full action set reachable only through the
§36 escalation endpoint; the Phase 2 profile is the record of one policy
and is void across contracts under σ0; the Slice F loop refines at one
hidden node with no cross-node class transfer, and the §29 class verifier is
named, not built; the controller works at one root per run with no
cross-root reuse; the remaining §33 work-item kinds RefineV1 named
(`SplitPolicyCylinder`, `CountThreatCover` as a scheduled item,
`EnumerateResidual`) were never built; no arena, no conformance gate, no
default-player change. The live player is deliberately untouched, and
RefineV1 never changes again.

## 9. After the era

The build continued the same day. Two further Pro parents were intaken on
2026-09-01 (MB-A1..A8, SC-A1..A8) and opened the model-belief program, the
God-gap censuses, the unified player and the focal-horizon hierarchy;
that story, including the naming of Phase 6's "structural saturation" as
God-tightness and the correction of §7's diagnosis, is
[walt-focal-horizon-era](walt-focal-horizon-era.md). By 2026-09-04 the
successor era's map (`walt/MAP.md`) declared `extraction.rs`, `godgap.rs`
and `horizon.rs` measurement scaffolding and `refine.rs` removable behind
the focal-horizon hierarchy; the consolidation slice was not built as of
main `c00717d1` (2026-09-07).

**Reproduction caveat (repository fact, git).** On 2026-09-06 the
partnership-gym commit `c59f1115` changed `solver/factor_belief.rs`
(+32 lines): the private `condition_via` now restricts the acting factor to
hands with positive completion support before consulting a policy, as the
model-belief path already did, retaining original factor weights and
removing only zero-joint-mass entries; the head note of
`walt/FACTOR-BELIEF.md` claims exact masses preserved. Every record on this
page predates that change. Masses should reproduce; the "sigma0 states
materialized" and per-route wall coordinates in `recursion_run1.txt`,
`response_run1.txt` and `cegar_run1.txt` plausibly do not. **Reproduction
of the era's records against current code has not been verified** — the
spot measurements below (C0 masses, Slice A decisions, the laydown census)
matched, and no campaign was run.

## Appendix A — instruments and records of the era

All binaries are `walt/walt/src/bin/<name>.rs`, built to
`walt/target/release/<name>`; every one takes an output path and writes the
record there (several also echo to stdout; `rootinterval` and
`laydownreport` write only the file). Records live under
`walt/probes/factor_belief/` unless stated. Walls are the record's, one
machine, one run. See [walt-instruments](walt-instruments.md) for the
catalog across programs and [walt-architecture](walt-architecture.md) for
the module layout.

| binary | modes | record | wall (record) | gate file (functions) |
|---|---|---|---|---|
| `rootinterval` | `run <out> [prefix]` | `../root_interval/run1.txt` | 0.52 s (2026-09-07); 0.19 s measured 2026-09-12 | `solver_root_interval.rs` (6) |
| `grammarsplit` | `run <out> [prefix]` | `../grammar_residual/run1.txt` | exact splits ≈ 0.35 s total | `solver_grammar.rs` (8) |
| `factorbelief` | `run` (C0), `opening-level0`, `cache` (C1), `c2` | `run1.txt`, `opening_level0_run1.txt`, `cache_run1.txt`, `c2_run1.txt` | C0 0.02 s (measured 2026-09-12); σ0 opening ≈ 5.4–5.6 s | `solver_factor_belief.rs` (11) |
| `factorrecursion` | `report <out>` | `recursion_run1.txt` | ≈ 15 s (h4-t4 σ0 7.2 s) | `solver_factor_recursion.rs` (5) |
| `factorresponse` | `report <out>` | `response_run1.txt` | ≈ 12 s (h4-t4 σ0 9.8 s) | `solver_factor_response.rs` (4) |
| `factorcegar` | `report <out>` | `cegar_run1.txt` | ≈ 6 s (opening classification 5.4 s) | `solver_factor_consequence.rs` (4) |
| `factorrefine` | `report <out>` | `refine_run1.txt` | ≈ 90 s | `solver_factor_refine.rs` (4; ≈ 146 s at landing, 70 s after CI1 — a gate-corpus-trim candidate) |
| `factorprofile` | `report <out>` | `profile_run1.txt` | 18.8 s | `solver_factor_profile.rs` (5) |
| — (spike) | — | — | — | `solver_proof_state.rs` (6) |
| `proofreport` | `report <out>` | `proofreport_run1.txt` | 14.1 s | `solver_proof_regret.rs` (5) |
| `extractreport` | `report <out>` | `extractreport_run1.txt` | 29.2 s | `solver_extraction.rs` (6) |
| `frontierreport` | `report <out>` | `frontierreport_run1.txt` | 13.3 s | `solver_frontier.rs` (6) |
| `bellmanreport` | `report <out>` | `bellmanreport_run1.txt` | 15.7 s | `solver_residual.rs` (6), `solver_covers.rs` (3) |
| `laydownreport` | `report <out>` | `laydownreport_run1.txt` | 0.4 s | `solver_laydown.rs` (4) |
| `openingreport` | `report <out>` | `openingreport_run1.txt` | ≈ 87 min across stops (last stop 76 min) | `solver_opening.rs` (5) |
| `doomreport` | `report <out>`; `scout <idx> <nodes> <cap> <level> [top-k]`; `enumscout <idx> <outer-limit>` | `doomreport_run1.txt` | ≈ 6 min (41–55 s per opening lead) | `solver_doom.rs` (8) |

`walt/probes/factor_belief/README.md` is the only quotable authority for
probe findings (CBS-A8); regenerate a record only through its binary, never
by hand. The 2026-08-30 records were produced by the binaries as landed at
their PRs; the release binaries present on this machine were built
2026-09-07 11:54 from an unrecorded branch.

## Appendix B — the gate index

Seventeen gate files, 96 `#[test]` functions (counted 2026-09-12 with
`grep -c '#\[test\]'`), all under `walt/walt/tests/`; every one ran green
in `walt/ci/check.sh` at its landing per the ledger, and cold `check.sh`
PASS runs are recorded by the successor slices (MB1, UP0, FH3, FH4). Not
re-run for this page.

| file | n | functions |
|---|---:|---|
| `solver_root_interval.rs` | 6 | the_mirror_endpoint_is_the_complement_and_the_lower_sweep_matches_the_adjudication · intervals_cover_the_exact_values_and_the_exact_optimizer_survives · a_same_stream_selected_lower_witness_is_unconstructible · malformed_count_paths_are_rejected · interval_pairing_and_risk_allocation_are_enforced · starved_budgets_stay_unresolved_and_the_fallback_stays_labeled |
| `solver_grammar.rs` | 8 | exact_split_matches_the_exact_optimizer_and_realizes_a_counterexample · singleton_grammar_value_equals_the_frozen_replay_count · sampled_split_matches_the_sampled_optimizer_by_prefix · grammar_value_is_monotone_in_sources · off_grammar_root_action_is_all_residual · residual_upper_is_the_full_class_upper · level1_continuation_source_is_dominated_by_its_grammar · grammar_refusals |
| `solver_factor_belief.rs` | 11 | uniform_mass_three_way_parity_and_focal_invariance · branch_masses_match_complete_world_enumeration · level0_field_branch_parity · condition_recovers_each_branch_mass · conditioned_marginal_matches_enumeration · c0_domain_and_identity_refusals · opening_root_contraction_without_worlds · level0_branch_parity_with_the_bundled_one_ply_oracle · classification_is_once_per_information_state · the_full_identity_key_shares_nothing_across_candidates_or_roots · opening_root_level0_classification_is_once_per_hand |
| `solver_factor_recursion.rs` | 5 | support_backend_matches_backend_zero_across_the_c0_domain · beyond_one_table_mass_matches_surviving_worlds_and_backend_zero_refuses · frozen_policy_values_match_the_bundled_walk_under_the_trivial_field · …_under_the_level0_field · every_node_mass_and_branch_parity_with_world_enumeration |
| `solver_factor_response.rs` | 4 | grammar_root_values_match_the_slice_b_split_under_the_level0_field · a_singleton_grammar_reduces_to_the_fixed_policy_recursion · grammar_sources_are_dominated_and_the_constraint_binds_somewhere · every_node_mass_and_grammar_max_parity_with_world_enumeration |
| `solver_factor_consequence.rs` | 4 | refinement_narrows_monotonically_to_the_exact_endpoint · the_refined_endpoint_reproduces_the_exact_contraction · every_refinement_carries_a_valid_witness_pair · the_base_vocabulary_carries_mass_and_classes_aggregate |
| `solver_factor_refine.rs` | 4 | escalation_matches_the_bundled_exact_authority · the_controller_narrows_monotonically_and_soundly · steering_refuses_presently_useless_work_deterministically · budget_exhaustion_returns_the_honest_surviving_set |
| `solver_factor_profile.rs` | 5 | profiles_conserve_mass_and_project_to_the_success_mass · the_tail_sum_identity_holds_on_every_computed_profile · one_profile_answers_every_contract_under_a_bid_blind_semantics · a_bid_reading_field_voids_cross_contract_reuse · profiles_match_complete_world_replay |
| `solver_proof_state.rs` | 6 | the_zero_budget_state_is_sound_and_serializes_and_resumes · imported_refine_v1_facts_reproduce_the_controller_verdicts · closure_is_idempotent_and_insertion_order_independent · fences_reject_and_stored_facts_round_trip · a_score_profile_fact_raises_the_executable_bar_through_closure · a_producer_registers_without_editing_the_module |
| `solver_proof_regret.rs` | 5 | the_recommendation_projects_exact_profiles_exactly · certified_regret_contains_exact_best_response_regret · regret_never_increases_under_monotone_refinement · a_grammar_lower_raises_only_the_proof_bar · report_quantities_reuse_across_contracts_when_bid_blind |
| `solver_extraction.rs` | 6 | an_extracted_policy_reprices_to_the_extraction_optimum · an_extracted_profile_is_one_realizable_profile · grammar_plus_residual_covers_the_full_policy_class · a_residual_below_the_grammar_lower_proves_unrestricted_closure · no_threshold_envelope_is_serialized_as_the_extracted_policy · the_extraction_producer_collapses_certified_regret_at_settled_roots |
| `solver_frontier.rs` | 6 | goal_debts_are_typed_and_zero_exactly_at_satisfaction · steering_bounds_are_safe_across_roots_and_goals · select_action_settles_and_zero_potential_refusals_are_real · the_macro_moves_the_upper_side_and_dominated_extractions_are_refused · strengthen_to_exact_contains_the_exact_solve_deterministically · a_starved_run_is_honest_and_resume_equals_uninterrupted |
| `solver_residual.rs` | 6 | staircase_intervals_bracket_nest_and_reach_the_exact_endpoint · merged_branches_are_law_and_the_cellwise_max_counterexample_is_rejected · policy_envelopes_bracket_nest_and_collapse_to_the_exact_profile · envelope_facts_close_to_executable_lowers_with_real_residual · census_potential_is_nonzero_exactly_where_closure_consumes · score_ranges_nest_and_contain_the_profile_support |
| `solver_covers.rs` | 3 | accepted_covers_never_understate_the_residual_gain · declined_covers_produce_no_number · zero_gain_collapses_and_rare_hazards_stay_visible (*the ledger's "4[3 fns]" counts three functions*) |
| `solver_laydown.rs` | 4 | the_boss_chain_control_is_a_laydown · zero_cost_arithmetic_and_the_near_laydown_counterexamples · the_hierarchy_holds_on_receipt_roots · the_producer_is_deterministic_and_closes_immediately |
| `solver_opening.rs` | 5 | zero_budget_stop_is_the_top_state_with_deterministic_serialization · sampled_ladder_narrows_monotonically_with_exact_risk_ledger · resume_is_semantically_identical_to_uninterrupted_refinement · opening_root_frontier_pass_is_an_honest_cliff · ample_ladder_settles_enumerable_roots_with_consistent_panels |
| `solver_doom.rs` | 8 | listed in §7 |

## Appendix C — the verifier companions (scratch tier)

Both Pro-shipped verifiers are filed verbatim beside their parents in
`walt/math/` and run with stdlib Python only; they are session evidence,
never receipts, never imported into code (TRUST-01).
`verify_counted_belief_sandwich_v0.1.py` (SHA-256 `ff87fb67…`): 20 checks —
the optimization-lock sweep over all 256 two-policy tables × 65,536 streams
(worst undercoverage 11/128 < δ = 1/4), the 90-deal factor-belief closure
(Z = 282 both ways, factorized Bellman = 30/47), the CEGAR fixture.
`verify_anytime_proof_state_score_v0.1.py` (`9efa51a0…`): 36 checks — the
256-signature score census, the 65,625-case certified-regret sweep, the 63/2
envelope non-realizability specimen, merge-before-max, the laydown
hierarchy, the closure-aware scheduling counterexample; the intake companion
flags checks 14, 16, 17 and 32 as near-tautological or definitional.
Measured 2026-09-12 on this machine from `walt/math/` with
`python3 -I -B`: both print `ALL CHECKS PASS`, exit 0, in 3.77 s and 0.16 s,
leaving no `__pycache__` (the D15 trap).

## Appendix D — rulings and freezes touched by this era

CBS-A1 (intake accepted at instrument tier), CBS-A2 (Theorem 5.1 = x:024
M1/M2 over pmake), CBS-A3 (root interval / survivor set; the retired word),
CBS-A4 (grammars; the §11 non-theorem = the O34 fence), CBS-A5 (counted
cells and covers), CBS-A6 (posterior closure with seat-locality boundary;
`FiberDp` = backend zero; the PiKey rule), CBS-A7 (witness-required
refinement), CBS-A8 (probe READMEs are the authority), CBS-A9 (build
program; live player untouched). APS-A1 (accepted), APS-A2 (score layer),
APS-A3 (bands and covers), APS-A4 (the envelope fence), APS-A5 (the
laydown hierarchy as typed vocabulary), APS-A6 (proof bar / executable bar),
APS-A7 (certified regret as the finite-budget deliverable), APS-A8
(closure-aware usefulness), APS-A9 (architecture as candidate; freeze 58;
Phases 0–8). Full text in `walt/CENSUS-RULINGS.md`; the lineage rows in
[walt-math-intakes](walt-math-intakes.md); freeze 58 in
[walt-math-freezes](walt-math-freezes.md); the negative results of the era
(zero cross-history reuse, the fragmenting tail, the bundled walk's speed at
small ratios, vacuous first-generation covers, the sampled plateau) are
indexed on [walt-negative-results](walt-negative-results.md).

Successor: [walt-focal-horizon-era](walt-focal-horizon-era.md) — two
recursions running in opposite directions (2026-09-01 → 2026-09-05).
