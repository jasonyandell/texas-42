# FACTOR-BELIEF — the Slice C–G design skeleton (specify, do not optimize)

**Status:** SLICES C (COMPLETE), D, E, F AND G LANDED (2026-08-30);
ANYTIME PROOF-STATE PHASES 0–8 ALL LANDED (2026-08-31/09-01, the
follow-on parent `walt/math/anytime_proof_state_score_v0.1.md`,
rulings APS-A1..A9 — THE PARENT PROGRAM IS COMPLETE; Phase 8's
paragraph below is the capstone); DOOM CENSUS LANDED (2026-09-01, the
first post-program structural producer — the §70 answer, paragraph
after the capstone); SLICE MB0 LANDED (2026-09-01, the model-belief
exact vertical slice — the §76 go/no-go evidence, paragraph after the
doom census). Phase 0 is the RefineV1 semantic freeze — freeze 58 in the
register: `solver/refine.rs` as merged at `25b40d9` takes no new
fields, variants, or work items, ever; its four gates never weaken; the
coming proof-state core must reproduce it wherever scopes overlap and
stays removable. Phase 2 is the §18 fixed-policy score profile
`viewer_score_profile` in `solver/factor_belief.rs`, beside its Slice
D/E/G siblings: the same factorized recursion carrying the full 43-bin
exact score object — bin `s` = the exact world mass banking exactly `s`
declaring-team points — instead of one tail sum. The profile is
viewer-independent (parity enters only at projection) and never reads
the bid, so one run yields the whole bid-threshold curve; the price is
the decided cutoff (a decided state knows the indicator, not the
score), and the probe measured that price at **~7–12% extra wall at
trick 4** for roughly double the nodes — the forgone subtrees are the
cheap late-hand ones. Gated by
`walt/walt/tests/solver_factor_profile.rs` (5 gates: mass conservation
`Σ H = Z` with tail projection to the Slice D success mass under both
viewer parities and no early cutoff; the §3 tail-sum identity; §44
contract reuse under a bid-blind semantics, checked against independent
re-runs whose cutoffs differ per contract; the reuse BOUNDARY as a
frozen specimen — σ0 reads the bid by construction, so under σ0 a
cross-contract answer is a re-run, not a projection, made mechanical at
h10-t6 threshold 42 where projection 12 ≠ evaluation 9; and entrywise
parity with an independent complete-world replay to true terminals).
Probe: `factorprofile report` (`probes/factor_belief/profile_run1.txt`)
— all ten gated roots under σ0, two focals, no drops; first §10/§11
band masses on real roots (h12-t6's certain miss is exactly 20 points
in every world — inside the d=10 rescue band at 1000‰; h3-t4 under
σ0-as-focal prices 280‰ with 22 distinct scores and 207‰ of mass
within ten points below the contract). No envelope across policies is
built anywhere — a profile is the record of ONE policy (APS-A4).

**The §49 architecture spike PASSED** (2026-08-31, the round after
Phases 0+2): `solver::proof_state` — the smallest honest kernel of a
persistent, serializable, identity-scoped proof state over one root,
with an OPEN producer registry. All seven §49 requirements proven by
six gates (`walt/walt/tests/solver_proof_state.rs`): the §25 top state
is sound and serializes/resumes bytewise; RefineV1's final interval
endpoints import as typed facts and closure reproduces its survivors,
exclusions, bar, and typed result on every enumerable root under both
ample configurations (freeze 58 consumed strictly as the frozen
oracle); closure is idempotent and insertion-order-independent (a pure
derived view — facts are the only stored authority); identity
mismatches in ANY §51 coordinate and malformed values are rejected
while stored facts round-trip serialization under re-validated content
hashes; a score-profile fact raises the EXECUTABLE bar through §41
closure derivation with `B_exec ≤ B_proof` asserted inside every
closure; and a producer defined in the TEST FILE (a §5 banked-floor
structural producer closing a repriced root to the exact
Equivalent-at-1 tie with no best-response solve) registers without
editing any enum in the module. Verdict: the in-crate shape is
CONFIRMED — zero duplication pressure was encountered, every authority
was reached through existing public APIs, and nothing imports the
module except the crate root (deletable, §67.10). Phase 1 fleshing
(the work frontier, declared solve goals) and Phase 3 (contract
projection + certified regret) remain queued on Jason's word.

**PHASE 3 LANDED** (2026-08-31, the round after the spike): contract
projection and certified regret. The closure now carries the §31
global upper `U* = max_a U_a` and the certified pmake regret
`Γ = U* − B_exec` (vacuous floor 0 at zero executable work), the
executable witness is a full `ExecWitness` (action, value, authority,
δ-status, fact id), and `ProofState::recommend()` derives the §33
block: recommended action and policy, pmake floor, global upper, Γ,
declaring score floor/ceiling, the §7 residual (exactly 0 for exact
profiles; positive values arrive with envelope cells), the §10/§11
d = 1 declaring bands, proof class, and the sampled-scope summary.
Five gates (`walt/walt/tests/solver_proof_regret.rs`): exact profiles
project exactly (the recommendation is the stronger candidate policy
at its independently recomputed exact value); Γ contains the exact
best-response regret against the bundled authority `exact_root_value`,
before and after the RefineV1 fact import, with `Q* ≤ U*`; Γ never
increases, `U*` never rises, and `B_exec` never falls under fact-by-
fact refinement; a non-executable grammar lower raises only the proof
bar (nothing executable → nothing recommended); and report quantities
reuse across contracts exactly when the semantics is bid-blind (the
σ0 boundary stays owned by the profile gates' frozen specimen). Probe:
`proofreport report` (`probes/factor_belief/proofreport_run1.txt`) —
the first §33 blocks on real roots: h5-t6 certifies REGRET ZERO at
444‰ (certified optimality far from certain make — the §31 point);
h3-t4 is §30's gap on trace — the settled best ACTION is 3-1
(Q = 350‰) while the best MATERIALIZED policy starts 4-4 at floor
267‰, Γ = 83‰, because 3-1's naive lowest-first continuation prices
below 4-4's (pmake belongs to the policy, not the first tile; the
next §33 work item is materializing a stronger 3-1 continuation —
Phase 6's argmax extraction); certain outcomes certify Γ = 0 in both
directions (h12-t6 floor = upper = 0; h10-t6 floor = upper = 1000‰).

**PHASE 6 LANDED** (2026-08-31, same day on Jason's go): §63 argmax
extraction and residual policy bounds — the answer to Phase 3's h3-t4
finding. `extract_success_policy` (factor_belief) is the §48/§36 max
recursion returning, beside the optimum, ONE policy attaining it: the
argmax DAG under the declared lowest-tile-index tie rule, keyed by
post-root history (the viewer's information state at a focal node),
living on the Slice B decided quotient, completed off-DAG by the same
declared rule, and re-priced UNCHANGED by the fixed-policy evaluators
(`ExtractedPolicy` is an ordinary `SlicePolicy`; its id is a content
address of the choice table — one realizable policy, never an
envelope, the §20 fence). `residual_split` is the §63 residual
Bellman: the exact `(M*, D)` pair with `D` the best deviating-class
mass (`None` = empty class, Slice B's Option discipline; hidden nodes
deviate in AT LEAST ONE branch — keep `M*` everywhere except the
cheapest-downgrade branch). `solver::extraction::ExtractionProducer`
is the first shipped ProofProducer: per root action, extract, re-price
through `viewer_score_profile`, install under the extracted content
id — how a response optimum stops being proof-bar-only and enters
`B_exec` (§30's bridge). Six gates
(`walt/walt/tests/solver_extraction.rs`): re-pricing equality both
sources at every enumerable root child; one realizable profile
(conservation + contract projection); the §12 cover identity
`M* = max(M^G, D)` with the residual walk's `M*` held to Slice G's;
closure AND escape both directions with three non-vacuity counters;
the §20 envelope specimen (tails cross, the threshold-wise max is no
single policy's record, and what extraction serializes is its own
evaluation bin for bin); and the §30 bridge end to end — `B_exec =
B_proof` after the producer at every root, Γ = 0 exactly once the §36
exact uppers price the upper side. Two discoveries the gates forced:
RefineV1 settles on cross-action dominance, so the WINNER'S own upper
can stay vacuous and Γ stays honestly positive at a settled root
(h4-t6, 2/15) until an upper fact prices it; and Slice E's "two-source
grammar ties free at t5/t6" is structural SATURATION — post-root focal
states there hold ≤ 2 tiles, so the deviating class is literally empty
(`None` everywhere multi-source at t5/t6). Probe: `extractreport
report` (`probes/factor_belief/extractreport_run1.txt`) — h3-t4 Γ
83‰ → 0‰ EXACTLY: the producer materializes 3-1's optimal continuation
(a 12,420-state DAG, ~1.1 s extraction), `B_exec` rises 267‰ → 350‰,
and the recommendation switches from 4-4 to 3-1 under the extracted
content id; h8-t5 Γ 282‰ → 10‰ (the residue is the winner's δ-tier
upper); and the two-source grammar ESCAPES on every h3-t4 root action
(m* > gram, e.g. 4047 > 3498 on 3-1) — at trick 4 the grammar leaks
and the residual proves exactly where, matching Slice E's trick-4
finding from the other side. Phase 4's envelope cells and Phase 5's
count-threat covers (which would bound `D` without walking it) remain
unbuilt; the exact residual is its own tightest cover in this slice.

**PHASE 1 LANDED** (2026-08-31, same day on Jason's "phase 1 up next
then!"): the Part IX work frontier — declared solve goals, work items
as proof transformers, §42 safe steering bounds, §41 closure-aware
selection, §43 containment (the §58 skeleton half was the §49 spike's,
already gated). `solver::frontier` holds four typed goals
(SelectAction, RecommendEpsilonPolicy(ε), StrengthenToExact,
ComputeFullScoreProfile — each with its own debt in its own units,
never one scalar), four deterministic work items (baseline profile,
§36 exact value with the §30 executable/proof-bar split kept, §63
targeted extraction, and the §41 macro `ExactValueSurvivors` — which
is LOAD-BEARING from the first step: from the top state every
standalone exact upper has provably zero effect on U* while any other
upper is vacuous), a declared forecast cost model (Z per fixed-policy
walk, 3Z per max walk — forecasts, never measurements), and
`Frontier::advance`, the anytime loop: refuse zero-potential items
(§34 as amended by §41), buy best bound-per-cost, install through the
ordinary fence, assert the §42 law on every purchase. Six gates
(`walt/walt/tests/solver_frontier.rs`): debt typing; the §42 law
recomputed from the report across every root × goal; SelectAction
settles with refusal honesty (a refused zero-potential item,
hand-executed, moves exactly nothing — the specimen is §39's own
sentence inverted: exact values are unconditionally irrelevant to the
profile goal); the macro moves the upper side and dominated
extractions are skipped (the frontier targets what Phase 6's ample
producer bought wholesale); §43 containment — StrengthenToExact lands
every surviving interval on the independently recomputed exact value,
twice, with identical schedules and byte-identical serializations; and
budget honesty with §44 resume-equals-uninterrupted. Probe:
`frontierreport report` (`probes/factor_belief/frontierreport_run1.txt`)
— goal separation is real money: h10-t6/h3-t5 certify Γ = 0 for ONE
baseline (1Z) while SelectAction costs 7–10Z; h3-t4 SelectAction
settles at 16Z WITHOUT buying any extraction (only the ε-goal pays for
the DAG, and its recommendation is then 3-1 at Γ = 0); h4-t6
SelectAction is 5Z — one exact upper excludes 0-0 against the baseline
bar and exact(1-1) is never bought. One honest waste, recorded: under
vacuous uppers the §42 bounds cannot distinguish a dominated
extraction from the winner (every U_a − B_exec ties), so h3-t4's
ε-goal bought all four extractions before the macro priced the uppers
— 28Z where uppers-first would pay ~15Z; §43 verbatim (a poor forecast
wastes time, it cannot weaken the proof state), and bound refinement
that prices upper-information value is future work, not a patch
rushed here.

**PHASES 4 + 5 LANDED** (2026-08-31, same day on Jason's "go for 4/5"):
the §61 score-aware residual Bellman and the §62 count-threat covers —
the machinery that connects Slice F's unresolved field classes to root
bounds and prices what the residual can still steal. Phase 4
(`staged_response_interval` / `staged_policy_envelope` in
`solver/factor_belief.rs`, producer in new `solver/residual.rs`): the F
staircase applies at each path's first field decision — the same
classification bottleneck Slice F instrumented — with the capped CEGAR
loop replayed prefix-stably, exact classes MERGED BY PUBLIC ACTION
(§23) and recursed through the exact §36 response, and the unresolved
mass attached as the §5 envelope, which at an undecided node
contributes exactly (0, R): the interval width IS the residual mass.
Nesting and the exact endpoint are theorems by mass additivity (both
proved in the fn doc, both gated). The fixed-policy side yields §54
tail envelopes — the new `Fact::Envelope` closes to an executable
lower and puts the first NONZERO §7 contract-sensitive residual on the
recommendation. The frontier gains `ResidualInterval` under the §41
CENSUS LAW: potential is nonzero exactly where an open interval lets
closure consume one (and rightly ZERO for the ε-goal on the top state
— the §41 stall applies to the census item too); at the declared flat
3Z forecast the exact item dominates it (never bought — honest, the
staged item's cost advantage arrives with Phase 8's staged cost
model). Phase 5 (`solver/covers.rs`): `Fact::Cover` carries the §13
resource decomposition (contested tricks + named count tiles — the §5
remainder decomposes EXACTLY, asserted) plus a verified uniform
movement bound from the new `declaring_score_range` walk (free-focal
range ⊇ every information-consistent deviation, incumbent range read
off its profile fact — no second walk); closure pairs cover with
incumbent and derives the §10/§11 rescue-band upper; no incumbent
profile → decline → no number. Ten gates
(`walt/walt/tests/solver_residual.rs`, 6, incl. the §23 cellwise-max
counterexample REJECTED — the fused per-class sum strictly exceeds the
lawful merged optimum of its own domain on h3-t4, the same-domain
comparison being the theorem's; `solver_covers.rs`, 4[3 fns]).
Probe: `bellmanreport report`
(`probes/factor_belief/bellmanreport_run1.txt`, 15.7s) — findings: the
staircase is a real anytime object (h3-t4 walks [145,606]‰ → exact in
5–6 stages with monotone narrowing; h8-t5 0-0 climbs 86‰ → 771‰ exact
across five stages); h12-t6's cover CERTIFIES V* = 0 for one range
walk (gain 0 collapses both actions — a cover proving failure at
range-walk cost instead of response-walk cost), h10-t6 collapses the
other way (certain make); h4-t6's range walk beats the arithmetic
envelope by exactly one point and leaves the 5-5 ten-count hazard
visible at 134‰; and the honest §70 caveat is live — at rich early
roots (h8-t5, h3-t4) every resource is still contested, gain equals
the envelope, and the first-generation covers are vacuous: richer
structural producers (protection conditions, per-cell partitions) are
the declared answer, not a patched heuristic.

**PHASE 7 LANDED** (2026-08-31, same day, the "7 can follow" half of
Jason's go): the §16 typed laydown hierarchy. `universal_viewer_success`
in `solver/factor_belief.rs` — one Boolean walk with three focal
quantifier regimes (Fixed π / Exists / All) under the universal field
quantifier ∀σ: hidden nodes branch over every could-play tile with
OVERLAPPING posteriors (sound for universal semantics; the walk ranges
over a per-seat relaxation of the world set — certification-sound,
possibly conservative, §17's structural route is the declared
tightening); the Exists witness is per-public-history, hence an
information-consistent policy. `solver/laydown.rs`: `classify_root` =
the four-tier census (PolicyCertainMake IS `viewer_success_mass = Z`,
the recursion already trusted) with the hierarchy held as internal
fences, and `LaydownProducer` ("laydown-v1") installing ONLY
`ProofTag::Deterministic` facts — §64's no-sampled-route law, gated.
§17's zero-cost closure is the walk's first line (a decided root
classifies with zero tree). Four gates (`solver_laydown.rs`):
boss-chain control = TRUE Laydown at the all-or-nothing contract,
proved by walk not phrase; already-made = 3 decided reads, zero tree;
loose-boss and possible-ruff counterexamples break every universal
tier; hierarchy + §16.1-coincidence on receipt roots; producer
determinism + immediate closure. Probe `laydownreport`
(`probes/factor_belief/laydownreport_run1.txt`): control certifies in
1.49M nodes / 0.37s, already-made in 10μs, loose-boss refutes
fail-fast in 280 nodes — and h10-t6 is a REAL receipt-root Laydown
(all four tiers, witness 2-2). Boundary: the walk is an ENDGAME
instrument (exponential in remaining plays); opening-depth laydown
certificates need a future structural producer, not a bigger budget.

**PHASE 8 LANDED — THE PARENT PROGRAM IS COMPLETE** (2026-08-31/09-01,
the "bring us home" go): the §65 opening-root iterative run.
`solver/opening.rs` — `OpeningLadder::run_stop` executes the §65 steps
in declared order against ONE append-only proof state: sampled root
bounds at each stop's declared prefix (the Slice A endpoints under
fresh per-stop δ scopes, imported through the ordinary §48 adapter —
the pinned-level1 witness IS the "cheap executable policy"); the
§39–§43 frontier pass at a declared Z budget; count-threat covers
through their producer (§62 decline while no incumbent exists,
installs guarded by fact equality); the §49 census as a REPORTED
COORDINATE at the Phase 4 stage; then the full §65 panel with the
typed verdict (exact / δ-qualified / ε-optimal / unresolved). Reads no
clock; RefineV1 untouched (the Slice A witness declaration restated as
configuration); deletable with its new-core siblings. Five gates
(`solver_opening.rs`): the §25 top state with the §67.4 bytewise round
trip; monotone narrowing with an EXACT derived risk ledger and an
idempotent presence guard; §67.5 resume ≡ uninterrupted, panel for
panel and byte for byte; the honest cliff at h0-t1 (buys nothing,
installs nothing, never manufactures a winner — §66.14); ample-budget
settlement on enumerable roots with mass-conserving census. Probe
`openingreport` (`probes/factor_belief/openingreport_run1.txt`), the
ladder p=16/64/256/512 at h0-t1 (Z = 399,072,960, seven legal leads,
contract 30 on threes): the bar climbs 0 → 407 → 594 → 732‰ and Γ
falls 1000 → 592 → 405 → 267‰ with the recommendation MIGRATING
0-0 → 2-1 → 6-5 as evidence deepens; nothing prunes (7/7 survive);
the cliff replays on the new core with a sharpening — all 29 frontier
refusals are pure AFFORDABILITY (with sampled facts installed every
exact item has positive declared potential); the stage-4 census reads
159–444‰ exact mass (the opening field far from action-exact — Slice
F's fragmentation from a new angle); and at p=512 the "batting 1000"
uppers finally break (U* 1000 → 999‰ — every sample-fitted optimum
lost at least one of 512 worlds) while the bar HOLDS at 732‰: the
SAMPLED TIER PLATEAUS. The remaining Γ ≈ 267‰ is policy gap plus lock
looseness, purchasable only by structural work — extraction across
the cliff, or counted failing sub-fibers installed as deterministic
uppers (Jason's "doom census" framing: characterize beating
arrangements by hand signature, size them with the §22 binomials,
subtract exact mass — the §70 "richer structural producers" answer,
the ∀-fail dual of Phase 7's laydown certificates). The final state:
56 facts, 10,439 bytes, risk 14/25 ≤ 3/5 itemized over 56 scopes,
verdict honest UNRESOLVED at ε = 1/4 — §65's first target met in its
certified-regret form: play 6-5, floor 732‰, at most 267‰ unclaimed.

**DOOM CENSUS LANDED** (2026-09-01, Jason's "construct and then size
the number of beating arrangements" with explicit latitude): the first
post-program structural producer — counterexample MASS as a
deterministic upper, the ∀-fail dual of Phase 7's laydown hierarchy
and the §70 falsifier's answer. `solver/doom.rs`:
`universal_viewer_failure` (focal nodes AND over every viewer escape —
one survivor kills the class; hidden nodes PARTITION the
record-consistent support by the declared σ0's deterministic choice —
`pmake`'s own field semantics; per-seat relaxation sound in the
certifying direction, phantoms only ever BLOCK); `doom_census` (the
§28/§49 signature vocabulary descending over one to three hidden seats
in acting order, exact oracle masses, the §46 partition law asserted
in full mode, a declared punish-priority mode — opponents
nastiest-first, partner weakest-first, feasible-class cut — for rich
roots); `doom_enumeration` (the per-world ground truth: a singleton
class is a belief, so the exact recursion is a world-aware make
check); `DoomCensusProducer` (uppers `(Z − M_doom)/Z`,
`ProofTag::Deterministic`, field identity in the authority, idempotent
against the append-only store). Eight gates (`solver_doom.rs`): the
already-set root dooms its WHOLE fiber in one decided read; the
loose-boss census meets the exact recursion EXACTLY (1120/1680, upper
= the exact value, with the per-seat phantom escape defeated by
physical tile conservation, not a deeper split); soundness against
`response_success_mass` on receipt roots; honest deterministic
starvation; producer/closure/round-trip; enumeration ≡ census on the
fixture; enumeration DOMINATES the census and stays sound; the
priority census is a sound partial harvest. Probe `doomreport`
(`probes/factor_belief/doomreport_run1.txt`): where doom lives the
census harvests it wholesale — 809–1000‰ of the per-world truth on
the enumerable receipt roots, h12-t6 whole-fiber at one node — and at
the h0-t1 opening root it certifies an honest ZERO whose diagnosis is
the finding: two adversarially hand-built crusher worlds and a
declared 228-point stride grid ALL let the world-aware viewer make 30
against σ0 after the 0-0 lead. A doom-family upper is floored at the
God make rate, and that rate is ≈ 1 here: the plateau's remaining
Γ ≈ 267‰ is overwhelmingly the INFO-CONSISTENCY PRICE — purchasable
by floor work (extraction across the cliff) and info-consistency-aware
uppers, never by counterexample counting. The census's working domain
is the endgame and in-play middlegame (t4–t6, where Phase 7's walk
also lives, and where every played hand eventually arrives); the
opening root priced the wall honestly — each non-forced σ0 read is a
modeled-mind mini-solve, the field-classification bottleneck measured
from the doom side.

**SLICE MB0 LANDED** (2026-09-01, the model-belief exact vertical
slice — the §74 assignment of
`walt/math/model_belief_base_player_v0.1.md` under rulings MB-A1..A8
and brief `walt/briefs/BRIEF-MB0.md`): the field model as a persistent
hidden coordinate over the existing counted-belief machinery, no live-
player change. `solver/model_belief.rs`: `BehaviorType`/`BehaviorTypeId`
(the §51 content address — construction, parent field identity, tie
rule, persistence scope, deterministic no-tape marker) registering
F₀ = σ0 = `Level0 { n0: 2 }` and F₁ = `Level1 { n_outer: 2, n0: 2 }`
per the intake's corrected rung table; `ModelBelief` = the exact
Ξ = Ω×Θ profile expansion (Theorem 7.1 made a struct) — one
`FactorBelief` per type profile driven by a seat-dispatching
`ProfileField`, integer prior weights cleared of the denominator
(ν = (1/2,1/2)³ is eight profiles over denominator 8), the §52
hand-type factor a DERIVED VIEW under Theorem 12.1's φ-map agreement
law, persistence structural (types fixed per lineage, no resampling
path exists — MB-I2); hidden branching merged by PUBLIC ACTION before
every max (§13/§32, MB-I4/MB-I6 asserted at the merged level), the
focal policy consulted exactly ONCE per information state for the
whole bundle (MB-I1: a type-keyed policy is unconstructible, witnessed
by a counting instrument); `mixture_response` = the exact Q(ν) with an
extracted history-keyed argmax `MixturePolicy`; `separated_upper` =
Theorem 18.1's U^sep with the §19 fusion price per root action. Eight
gates (`walt/walt/tests/solver_model_belief.rs`): (ω,θ) enumeration
parity on three roots (masses, branch partitions, posteriors, response
vectors, Q(ν), and argmax re-pricing through the raw pair walk);
point-mass parity BOTH ways (δ_F₀ value AND selected action against
the raw σ0 authority on all six enumerable roots; δ_F₁ on the raw
Level1 authority's entire terminating domain, its refusal complement
pinned exactly); Theorem 12.1 closure per profile plus the §9 ½-vs-¼
persistence specimen (SYNTHETIC declared carriers on a real t5 root
with a pinned-hand table, searched deterministically: first action
halves the mass, second moves NONE where the carriers provably
disagree); merge-before-max; exact ν-linearity on a swept rational
grid including non-product profile priors; the Thm 18.1/19.1
biconditional at every tested action; §51 identity under every
coordinate change; and the σ1 boundary gate. TWO DISCOVERIES the
gates pin: (1) THE σ1 POSITIVE-SUPPORT BOUNDARY — the Level1 mind's
§4.2 shuffle-and-reject sampler has an EMPTY acceptance region exactly
at zero-joint-mass information states, and the shared conditioning
route classifies the acting seat's raw support, which contains such
states at depth (harmless dead weight under σ0, NON-TERMINATION under
σ1: the untightened recursion cannot run on ANY undecided tested root
— live specimen h5-t6, history [4-1, 4-3, 1-1], hand {4-2 4-4},
exhaustively unsatisfiable AND zero-mass by fiber enumeration); the
module's positive-support tightening (narrow the acting factor to
`actor_completion_weights`' nonzero support before every conditioning
— pure counting, exactness-neutral by the zero-entry law) is the
ENABLING construction: the δ-F₁ walk goes from non-terminating to
12 ms at h5-t6 with exact enumeration parity. The raw Level1
authority's terminating domain among the tested roots is EXACTLY
{h12-t6, h10-t6} (the root-decided fixtures); its refusal on the four
undecided roots {h5-t6, h4-t6, h8-t5, h3-t5} is pinned in G2, so the
F₁ half of point-mass parity is a SCOPED YES — exact on the entire
terminating domain, enumeration-anchored on the blocked roots — and
the caveat travels with the result wherever it is restated. The
sampler itself is REPORTED, never repaired here (per ruling the
repair and its evidence pass are the immediate follow-up slice, and
the unpatched sampler is deliberately left in place for that slice's
before-side determinism capture): the hazard is pre-existing shipped
code with FIVE byte-identical copies — the library copy
(`solver/mod.rs:897`, reached by field.rs → level1_evaluate) and four
un-deduplicated bin copies (walt_bridge, playout, playtable,
divergence — including the live player's own), with `level1_evaluate`
itself separately triplicated (solver/mod.rs, walt_bridge, playtable)
as named debt; MB0 is simply the first caller to sweep enough
information states to reach an empty acceptance region, and the wall
plausibly gates MB1's larger roots. (2) THE FUSION PRICE IS
ZERO EVERYWHERE ON THIS CORPUS — across the registered F₀/F₁ mixture
AND the synthetic carrier mixture on every root action of all six
roots, plus a 302-fixture hunt (every root × hidden seat ×
positive-mass pinned hand), Q(ν) = U^sep exactly and one common policy
is always pointwise optimal (the Theorem 19.1 biconditional green at
every fixture): the type-revealed upper is TIGHT here — the opposite
direction from §73's vacuous-upper falsifier — and a strict specimen
is expected to need earlier roots with more pre-revelation focal
decisions (MB1's first structural target). Probe `modelbeliefreport`
(`probes/factor_belief/modelbelief_run1.txt`, 5.3 s total): the §75
report on six roots × both mixtures — REAL type evidence moves on the
registered mixture (h3-t5's third observation prices a seat's marginal
at F₀ 51/75 against F₁ 24/75; h4-t6 reaches 24/40 vs 16/40) while the
carrier mixture concentrates to full identification in one action;
aggregation census up to 186 typed rows against 27 merged public
branches on one line; the §76 criteria: 1, 2, 3, 5 YES (parity,
closure, a nontrivial mixture response on exactly 3 registered root
actions — gate-pinned in G6, never probe-only — and the type dimension
small), criterion 4 honestly NO (the upper is never strict on this
corpus). Deviations from the brief, recorded: δ_F₁ parity "on
every tested root" narrowed to the raw authority's terminating domain
{h12-t6, h10-t6} (the raw authority provably cannot price the rest —
G8's enumeration anchor covers the δ_F₁ endpoint there), and `separated_upper` computes
q(θ) by the single-profile respond walk rather than
`response_success_mass` (same wall; anchored to it on its terminating
domain by G2). EXPLORATORY tier; the go/no-go reading is Jason's.

Slice G (§50, Part VIII §32–37) is the integrated refinement controller
`refine_root` in `solver/refine.rs`, plus the §36 EscalateExact endpoint
`response_success_mass` in `solver/factor_belief.rs` — the full-action-set
recursion §48 sequenced for exactly this slice, gated to extensional
parity with the bundled exact authority (`exposure::exact_root_value`)
at every gated root and action, the C→G cross-representation capstone.
The controller keeps one TYPED interval `[L_a, U_a]` per legal root
action (§42's constructor discipline: sampled δ bounds carry their full
Slice A record; exact bounds carry integer masses over the shared root
`Z`), excludes an action exactly when its upper falls below the bar
`B = max_a L_a`, and runs the §36 loop over the buildable §33 work-item
subset — `SampledLower`/`SampledUpper` (Slice A's δ-valid witness and
optimization-lock upper, each endpoint a distinct `ScopedDelta` against
a declared root risk scope), `ExactFixed`/`ExactGrammar` (the Slice D/E
factorized recursions as exact lowers), `EscalateExact` (the interval
collapses to the exact point `Q_a`), and `ConsequenceCensus` (carried
precisely to demonstrate §34: zero declared root-width reduction, so
the steering rule refuses it at every bar). Scheduling is §35 verbatim:
best-case reduction of the declared decision-width scalar
`D = (|survivors| − 1) + Σ (U_a − B)` per declared integer forecast,
exact rationals cross-multiplied; the budget charges FORECASTS, never
wall time, so every run is a pure function of its inputs. Results are
typed `Settled` / `Equivalent` (deterministic point intervals at the
bar) / `Unresolved` (the honest surviving set with a NAMED fallback
rule, never promoted — §37.9), with the proof class `DeltaQualified`
whenever a sampled side took part in a decisive exclusion. Gated by
`walt/walt/tests/solver_factor_refine.rs` (4 gates: escalation parity
with the bundled authority plus the containment chain fixed ≤ grammar ≤
response; the §37 soundness invariant walked over full runs with every
exact bound independently recomputed; §34 refusals with bytewise
run-determinism; §36 step-12 starvation honesty with the δ ledger
re-asserted through `assert_screen_risk_allocation`). Probe:
`factorrefine report` — the findings: (1) the exact ladder settles all
ten gated roots (six SETTLED, four honest EQUIVALENT ties), and twice
it settles WITHOUT escalating the winner — the winner's cheap exact
lower cleared every rival's escalated point (h4-t6, h4-t4), §36's
one-witness promise realized; (2) on the small t5/t6 fibers the sampled
tier settles roots before ANY exact recursion runs (h4-t6 at 64 work
units against 420 exact-only; h8-t5 at 3,776 against 13,860), correctly
δ-qualified, while at trick 4 the sampled uppers are too loose to prune
and the exact ladder does the work at negligible sampled overhead —
both regimes on one trace; (3) at the opening root h0-t1 the controller
walks the affordability cliff honestly: every exact item is refused by
its own declared forecast (the §40 contraction/field-classification
walls), fourteen sampled endpoints produce real intervals over the
399,072,960-world fiber, nothing prunes, and the result is the honest
UNRESOLVED surviving set with the fallback named and never promoted.
EXPLORATORY tier.

**Slice F** (landed the same day, an earlier round) —
Slice F (§49, §27–31) is the consequence-CEGAR hand-class instrument
`refine_to_action_exact` in `solver/factor_belief.rs`: §28's feature map
`κ` (`ClassSignature` — the §49 starting vocabulary of critical tile
membership, trump count/highest trump, led-suit count, count-tile
possession, current-winner/ruff possibility) partitions the acting
seat's support at the field-classification bottleneck, and the §30 loop
aggregates action-uniform classes exactly while splitting the
largest-mass non-uniform class by a WITNESS PAIR — two same-class hands
with different field actions, their lowest differing tile entering the
§31 critical set — until residual class mass is zero (termination ≤ 28
refinements: a witnessed discriminator is provably outside the critical
set). Gated by `walt/walt/tests/solver_factor_consequence.rs` (4 gates:
Theorem 30.1's monotone narrowing with NESTED per-branch intervals
`[L_t, U_t]` and an action-exact endpoint; endpoint parity — the fully
refined abstraction reproduces `branch_masses` tile for tile; §49's
witness requirement re-derived independently, the field itself
re-consulted on hand-built records; and non-vacuity — the bare
vocabulary resolves positive mass, classes aggregate, the loop fires).
Probe: `factorcegar report` — the two-sided finding is that MASS
CONCENTRATES BUT THE TAIL FRAGMENTS: at the opening root under σ0,
805‰ of the 399,072,960-world posterior mass sits in action-exact
classes at 36,923 classes (3 hands/class; 513‰ already at 5,387
classes, 21 hands/class), but driving residual to ZERO costs full
fragmentation — 116,280 singleton classes, §51's falsifier for the last
slice of mass under a SAMPLED modeled mind — which vindicates §49's
interval discipline (carry small residual as branch intervals; don't
chase the endpoint). The instrument pays one classification per support
hand (the same bill as `branch_masses`) and claims representational
structure only, never a faster classifier; the trivial-field endpoints
DO aggregate (255/495, 56/126, 147/495 at trick 4), so the tail
fragmentation is a property of σ0's sampling, not of the vocabulary.
EXPLORATORY tier.

**Slice E** (landed the same day, an earlier round) —
Slice E (§48) is the factorized grammar best response
`grammar_success_mass` in `solver/factor_belief.rs`: the §23 recursion with the focal case's
frozen action replaced by a max over the grammar's actions —
`M^G(B) = max_{t ∈ G(I)} M^G(B·t)`, lawful on the cleared side because
every focal child shares `Z(B)`, so the max of masses is the max of
values; hidden nodes keep the conservation sum, and nodewise max equals
the §12 policy-class optimum `Q^G` by the cylinder-partition argument
(the belief is a function of the public history, so focal nodes are in
bijection with the viewer's information states). Gated by
`walt/walt/tests/solver_factor_response.rs` (4 gates: per-root-action
parity with Slice B's enumeration split `exact_grammar_split` under σ0,
with the root call the max over grammar root actions; a singleton
grammar collapsing exactly to the Slice D fixed-policy recursion;
source dominance plus non-vacuity — the constraint BINDS somewhere, via
singleton grammars, because the two-source grammar ties the free
optimum on every enumerable root; and the every-node checker with the
grammar max structure enumerated at every focal node). The §48 fence is
kept: NOTHING maximizes over the full action set — `free` figures come
only from the Slice B split, and no argmax/policy is extracted (that
needs a declared tie order; not a Slice E claim). Probe:
`factorresponse report` — the finding is that AT DEPTH THE MIX PAYS: at
trick-4 roots the grammar optimum strictly beats every source (h4-t4
trivial: Q^G = Z = 34,650, certain make, against 34,170 for the best
source; h3-t4: 3,815 against 3,062), while at trick-5/6 roots it never
exceeds the best source and the two-source grammar saturates every
reached undecided state (every §12 verdict "closes" with no deviating
continuation). EXPLORATORY tier.

**Slice D** (landed the same day, concurrent session) — Slice D
is the general
support contraction `SupportOracle` (§25.2's acting-hand loop
generalized to conditioned completions, §25.4's sparse-support walk) and
the §23 factorized fixed-policy recursion `viewer_success_mass` (§47),
both in `solver/factor_belief.rs`, gated by
`walt/walt/tests/solver_factor_recursion.rs` (5 gates: extensional
parity with backend zero across the C0 domain including the opening
root's contraction; surviving-world mass parity beyond one table with
backend zero's refusal preserved at the boundary; §47 value parity with
the bundled walk under the trivial field and under σ0 on every
enumerable root; and the every-node checker — mass equals the
surviving-world count and branch masses equal the world partition at
EVERY node of the recursion tree). The recursion computes the
viewer-objective success mass `M` with `V = M/Z` the exact integer pair
— §23 cleared of denominators by conservation, no rationals anywhere.
One law was discovered at depth: `condition` restricts its support walk
to hands CONSISTENT WITH THE PUBLIC RECORD (own plays contained,
others' plays excluded) — such hands are provably zero-mass and their
action likelihood is undefined; σ0's type-enforced information-state
constructor is what caught the unlawful classification, and at one ply
the filter is a no-op, so the C1 conditioning-support law is unchanged.
Probe: `factorrecursion report` — value parity on every row including
trick-4 roots (fiber 34,650, 16 post-root plies, 121,868 conditionings
under σ0); the honest negatives (bundled faster at worlds/hands ≈ 3,
recursion classifies more σ0 states than the bundled walk meets) are in
the probe README, and Slice E's probe extends them (the Slice B
enumeration split is 30–40× faster than the grammar recursion at these
same small ratios). EXPLORATORY tier.

**Slice C in full** — stages C0, C1 AND C2 LANDED (2026-08-30) —
`walt/walt/src/solver/factor_belief.rs`, gated by
`walt/walt/tests/solver_factor_belief.rs` (11 gates: the seven C0 gates —
three-way mass parity, branch-mass parity with complete-world enumeration
under two trivial fields and the σ0 level-0 mind, the Theorem 20.1
conditioning route, conditioned marginals against enumeration, the
declared domain refusals, the §22 opening-root contraction — plus the
four C1 cache laws: σ0 branch parity with the bundled one-ply oracle on
every receipt fiber WITH full extensional cache identity between the
routes, classification once per information state, zero sharing across
public histories under the full §43 identity key, and the opening root's
116,280 hands classified exactly once). C1 added no library code: the C0
contraction plus `FieldModel`'s insert-only cache already classify once
per state; C1 is the gates and the measurements. Probe:
`walt/probes/factor_belief/` — opening-root branch masses in 8.7 ms over
a 399,072,960-world fiber; the σ0 opening classification realized in
5.36 s (46 µs/hand); the REPEAT contraction 23.3 ms (200 ns/query pure
cache identity, ×230); cross-history reuse exactly 0 (the honest
negative that routes classifier compression to Slice F). EXPLORATORY
tier. Source: `walt/math/counted_belief_sandwich_v0.1.md` Part V–VI
(§18–26), rulings CBS-A6 and CBS-A9.

Stage C2 closes the slice and, like C1, added NO library code and no new
gate — it is the report §46 asks for: all seven required coordinates
from ONE opening-root run under the σ0 field (`factorbelief c2`, record
`walt/probes/factor_belief/c2_run1.txt`). Hands 116,280 (asserted);
contraction 5,933 µs for the completion weights alone, 21,818 µs warm
(weights plus full §43-key identity, zero classifications); field
classification 5,339,731 µs derived by subtraction from the 5,361,549 µs
cold pass — 45 µs/hand, 99% of the bill; 20 distinct branch tiles; reuse
×245 at 187 ns/query; memory as TWO figures kept apart — 23,563,392
bytes of declared accounting for the action cache (88-byte entries,
262,144 buckets, one control byte each, plus the key's one-tile history
Vec) against a 63,340,544-byte MEASURED maximum resident size
(`/usr/bin/time -l`, agreeing with the in-run `/bin/ps` reading at exit;
peak footprint 62,390,680 bytes); conservation exact at 399,072,960.
The memory coordinate deferred by C1 is therefore discharged, with the
accounting never presented as a measurement. The Slice D recursion
(§47) landed the same day, in a concurrent session (see Status above).

Build-time deviations from the sketch below, under L2-A3's naming
latitude (module docs carry the same list): `branch_masses`/`condition`
take no seat argument — the acting seat is the derived view
`seat_to_move()`, and passing it would store one authority twice;
masses are `u128` with checked arithmetic (the kernel's counting
width), not `BigUint`; `count_cell` stays deferred — Slice F's hand
class turned out to be a ONE-seat predicate, which `marginal` counts
exactly, while Part IV's multi-seat structural cells still have no
consumer (the slice that first needs a cross-seat cell mass gives them
their type). The C1 cache study is done — its §26
coordinates live in the probe README, and its finding is that
within-history reuse is total while cross-history reuse is zero by the
identity law. Slice C is complete (the C2 report discharges the §46
coordinates), and Slices D, E, F and G are landed (see Status). The
C→G ladder of `counted_belief_sandwich_v0.1.md`'s Part XI program is
COMPLETE: root intervals, grammar/residual, factorized contractions,
recursion, grammar response, consequence CEGAR, and the integrated
controller all exist with gates and probe records. What remains beyond
this file's scope: the §29 action-exact class verifier (named, never
built), the unbuilt §33 producers (`SplitPolicyCylinder`,
`CountThreatCover`, `EnumerateResidual`, …), cross-root reuse and any
cost-model refinement, and — before any default change — arena and
conformance gates: the existing controller player remains the fallback
surface (§50), and nothing on this ladder touches the default player.
The ladder's successor program is the anytime proof-state parent
(`walt/math/anytime_proof_state_score_v0.1.md`, APS-A1..A9): its Phase
0 (freeze 58, RefineV1) and Phase 2 (the score profile — see Status)
are landed; the §49 architecture spike, contract projection with
certified regret (Phase 3), and everything after are queued on Jason's
word.

## The objects (parent §18–21)

A **hand factor** is one hidden seat's exact nonnegative-rational weight
table over its possible root hands — `φ_{s,h}: C(U, k_s) → ℚ≥0` — and a
**factor belief** is the seat factors joined by the disjoint-cover
constraint, with the public history that produced them:

```text
HandFactor identity   = (seat, capacity, weight-table representation)
FactorBelief identity = (root physical fiber id,
                         public state sufficient for every field read,
                         field identity,
                         every HandFactor's representation and weights,
                         stochastic-tape factor id if any,
                         utility and bid/declaration parameters)
```

The identity list is §43 verbatim and BINDING (CBS-A6): a cache hit
under an omitted coordinate is the PiKey defect reborn. Weights are
integers/rationals only; the uniform lawful fiber is the special case
where every factor is a 0/1 legality predicate.

## The contraction interface (parent §24)

```rust
/// One abstract authority; a backend changes only under exact
/// extensional parity gates (CBS-O13 shape). Every returned mass is an
/// exact integer or rational derived from one canonical factor state.
trait ExactCoverOracle {
    /// Z_h — the exact-cover partition function of the factor state.
    fn mass(&self, belief: &FactorBelief) -> BigUint;
    /// Per possible hand A of `seat`: φ_{s,h}(A) · C_{-s,h}(U \ A),
    /// the exact compatible-completion weight (§21).
    fn actor_completion_weights(&self, belief: &FactorBelief, seat: Seat)
        -> Vec<(HandId, BigUint)>;
    /// {action t ↦ Z_{ht}} for the acting hidden seat under the declared
    /// field — the one-ply branch-mass target (§21's boxed equation).
    fn branch_masses(&self, belief: &FactorBelief, seat: Seat, field: &FieldModel)
        -> Vec<(Domino, BigUint)>;
    /// The posterior update: multiply ONLY the acting seat's factor by
    /// its action likelihood (Theorem 20.1 — closure is the theorem).
    fn condition(&self, belief: &FactorBelief, seat: Seat, action: Domino)
        -> FactorBelief;
    /// Exact mass of a structural predicate over deals (Part IV cells).
    fn count_cell(&self, belief: &FactorBelief, predicate: &CellPredicate) -> BigUint;
    /// Exact marginal mass of a hand predicate for one seat.
    fn marginal(&self, belief: &FactorBelief, seat: Seat, predicate: &HandPredicate)
        -> BigUint;
}
```

(Names indicative; Rust surface free at build time — L2-A3's naming
latitude applies.)

## Backend zero, and the gates

- **Backend zero is `kernel/fiber.rs` (`FiberDp`)** — the shipped
  tile-pattern capacity DP *is* the uniform-root special case (CBS-A6's
  attribution amendment and its happiest finding). Slice C's stage C0
  wraps it behind the trait for 0/1 factors; no new counting mathematics.
- **Mass conservation gate:** `Z_h = Σ_t Z_{ht}` exactly, at every
  contraction (§46 C0).
- **Parity oracle gate:** every backend result equals complete-world
  enumeration on its declared domain (`bundle` and explicit enumeration
  are the extensional oracles — CBS-O13). Small/medium fibers first;
  the trick-1 target (399,072,960 worlds, 116,280 acting-seat hands) is
  stage C2 and is a REPRESENTATION result even if the field classifier
  stays slow (§46) — as reported, the classifier IS the bill, 99% of the
  cold pass against 5.9 ms of counting.
- **Boundary obligation (CBS-A6, binding):** any field or belief with
  cross-seat coupling voids Theorem 20.1 until represented as explicit
  additional factors — never silently projected into seat-local form.
  The shipped level-0/level-1 fields are verified seat-local (intake
  companion, code-boundary audit).

## Build order when authorized (§46–48, unchanged)

C0 trivial field (`FixedPreference::lowest_first`) → C1 cached level-0
field with per-hand classification → C2 opening root branch masses. All
three are walked. Recursion (§47's factorized fixed-policy Bellman) only
after one-ply parity and cost are understood — they now are, and Slice D
is in flight. Measured coordinates per §26: contraction arithmetic,
distinct hand materializations, field cost per hand, reuse, support
shrinkage, cache identity cost, integer width, trick-1 memory (C2's
report — an accounting and a measurement, kept apart),
SIMD/GPU/WASM suitability (a measurement coordinate under the ripcord
discipline — CBS-A9, never an authorization).
