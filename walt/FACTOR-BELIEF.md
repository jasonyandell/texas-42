# FACTOR-BELIEF — the Slice C–G design skeleton (specify, do not optimize)

**Status:** SLICES C (COMPLETE), D, E, F AND G LANDED (2026-08-30);
ANYTIME PROOF-STATE PHASES 0–8 ALL LANDED (2026-08-31/09-01, the
follow-on parent `walt/math/anytime_proof_state_score_v0.1.md`,
rulings APS-A1..A9 — THE PARENT PROGRAM IS COMPLETE; Phase 8's
paragraph below is the capstone); DOOM CENSUS LANDED (2026-09-01, the
first post-program structural producer — the §70 answer, paragraph
after the capstone); SLICE MB0 LANDED (2026-09-01, the model-belief
exact vertical slice — the §76 go/no-go evidence, paragraph after the
doom census); SLICE σ1-REPAIR LANDED (2026-09-01, MB0's follow-up — the
belief sampler terminates and the five copies are one, paragraph after
MB0); SLICE U0 LANDED (2026-09-02, the God-gap census — the §8
decomposition made mechanical and the §38 fusion horizon measured at
trick 5, paragraph after σ1-repair); SLICE MB1 LANDED (2026-09-02, the
model-belief recursion joined to the solver — and the STRICT model-fusion
price found at trick 4, paragraph after U0); SLICE UP0 LANDED (2026-09-02,
the FIRST SLICE OF THE NEW UNIFIED WALT PLAYER — one decision function
over the whole instrument stack, provenance always, both recursions
played and named, paragraph after MB1); SLICE UP1a LANDED (2026-09-03, the
lazy carry — the posterior is recorded at every ply and materialized
only when read; the lean rung's 2.1 s of unread carry became 0 µs,
paragraph after UP0); SLICE U0b LANDED (2026-09-03, the in-solve horizon
census — the God-gap census at every frontier node a trick-4 solve
reaches, and the exact price of a §39 fusion cut at that depth,
paragraph after UP1a); SLICE FH1 LANDED (2026-09-04, the focal-horizon
hierarchy engine — three instruments become the endpoints of one
refinement object indexed by focal decisions, affordable-or-refuse,
paragraph after U0b); SLICE FH2 LANDED (2026-09-04, the focal-horizon
ladder — the engine made anytime under Proposition FH-int: budgeted
passes that stop and resume on a store of node facts installed by
intersection, proof-state facts with their witnesses, exact suffix
reuse under the full belief identity, paragraph after FH1); SLICE FH3
LANDED (2026-09-04, the report of record and the FH8 anchors — every §38
measurement at 33 (root, contract) coordinates × k ∈ {0, 1, 2, 3}, the
anchors' settling horizons DISCOVERED: trick-4 anchors at k ≤ 2, the
trick-3 root at k = 3 with Γ_2 = 34‰, the ply cut's wrong action never
certified, paragraph after FH2). Phase 0 is the RefineV1 semantic freeze — freeze 58 in the
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
uppers, never by counterexample counting. [Corrected 2026-09-03: this
sentence outruns SC-A1/SC-A4 — zero doom leaves the split between
`d_info` and `d_policy` UNKNOWN, and U0 typed the opening `UnknownGodGap`;
see `walt/DISCREPANCIES.md`, "doom-census ledger paragraph".] The census's working domain
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

**SLICE σ1-REPAIR LANDED** (2026-09-01, brief
`walt/briefs/BRIEF-SIGMA1-REPAIR.md`, authorized as MB0's immediate
follow-up so the evidence window stayed open): the §4.2
shuffle-and-reject sampler MB0 reported now TERMINATES, and there is one
of it. `solver::belief_frame_feasibility` decides a declared belief
frame by counting alone — Hall's condition in deficiency form over the
tiles-to-seats assignment, so for every subset S of the three other
seats the unseen tiles no seat outside S may hold must not outnumber S's
declared room plus the leftover the sampler's prefix slicing never
deals. Eight subsets, exact integers, no search. It decides the
sampler's own acceptance region rather than approximating it: gate R5
agrees with exhaustive exact-partition search on 2,000 swept frames with
both verdicts present, and a companion gate pins the
prefix-versus-partition distinction the leftover creates.
`sample_belief` now returns `Result<Vec<[u32;4]>, InfeasibleFrame>` —
the precheck runs first, consumes no randomness and rejects no feasible
frame, so a feasible frame's draw sequence is bit-identical to the
unguarded loop's. That is not asserted but WITNESSED: a before-side
determinism capture was taken against the UNPATCHED sampler and
committed first (`walt/walt/tests/data/sigma1_before_v1.txt` — 48 belief
frames, 43 carrying real deduced voids, each with its exact drawn worlds
and its post-draw RNG word, plus 2,469 σ1 field actions and every pmake
indicator over the four undecided receipt roots), and gate R2 reproduces
it byte for byte through the repaired path. The refusal is typed all the
way out as `Level1Refusal::{Deadline, InfeasibleFrame}` on
`level1_evaluate` and its race siblings: the live bridge reports
`eval refused (<frame>); playing lowest legal` — never a panic, never
the deadline's message, never a silent fallback — while the three
`SlicePolicy` boundaries raise the frame's own description because the
trait returns a tile and has no typed channel. The auction and pre-play
draws go through `sample_open_belief` instead, which is TOTAL: with
every void mask zero the rejection test cannot fire, so the acceptance
region is the whole deal space and there is no refusal to handle. That
proof lives once, in the library, rather than as an `expect` at each of
the twenty-two call sites that used to restate it — which is what lets
the live player hold no error branch on a sampler result anywhere, as
opposed to one it could only argue was unreachable (gate R6). THE FIVE
COPIES ARE NOW ONE: walt_bridge, playout, playtable and divergence lost
their local samplers and dependencies (297 lines), the latter by the
forced type-identity cascade — a local `SplitMix64` is a distinct Rust
type, so importing the sampler drags the RNG, `mask_bits` and
`FULL_MASK` with it. Two witnesses gate the dedup: a source grep over
`src/`, which catches a local copy that compiles because nothing calls
it; and the compile itself, since each deduplicated binary imports the
library name at module scope and a local `fn sample_belief` beside it is
an E0255 collision — which is also the witness that survives a rename of
the authority, where a fixed-name grep would not. The six binaries that
already called the library copy — ordering_bench, webtable,
controller_bridge, shadow, waking_bridge, tiltaudit — inherit the repair
with no diff at all (audit follow-up: the covered-for-free half of the
call-site enumeration, stated here so the coverage claim is complete in
one place). Label note for cross-referencers: the gate names "R5"/"R6"
in this paragraph are the test file's own (counting-oracle-vs-exhaustive
and the walt_bridge no-error-branch sweep); BRIEF-SIGMA1-REPAIR.md's
"R5" names a different obligation — MB0's gates staying green — which
also holds (8 passed, untouched). `level1_evaluate`
remains TRIPLICATED (`solver/mod.rs`, walt_bridge, playtable) — named
debt, deliberately unpaid here. THE FINDING: the four roots G2 pinned as
the raw σ1 authority's refusal set do NOT open. Each now terminates
promptly with a named refusal instead of hanging — h5-t6 in 10.4 ms
(the pinned specimen exactly: three unseen tiles confined to seats
{S0, S2}, which hold room for two), h4-t6 in 52.5 ms, h8-t5 in 45.7 ms,
h3-t5 in 109.3 ms. MB0's scoped parity domain therefore did not grow,
and the caveat above stands unchanged; what changed is its status, from
a wall the machinery ran into to a boundary it can name. The blocked set
is a real property of the untightened route — zero-joint-mass hands sit
in its raw support — so MB0's positive-support tightening is the
enabling fix and not a workaround. Seven gates
(`walt/walt/tests/solver_sigma1_repair.rs`); MB0's eight, G2 and G8
included, stay green untouched. EXPLORATORY tier.

**SLICE U0 LANDED** (2026-09-02, the God-gap census — §48 of
`walt/math/salvation_complex_v0.1.md` under rulings SC-A1..A8 and brief
`walt/briefs/BRIEF-U0.md`): the §8 three-part failure decomposition
made mechanical, and with it the first measurement of WHERE
information-consistency starts costing anything.
`1 − V(ρ) = d_phys + d_info + d_policy(ρ)` — physical doom, the
information price `U^God − Q`, the policy gap `Q − V(ρ)` — where a zero
doom census moves only the first term and says nothing at all about
the other two. New module `solver/godgap.rs`, a SIBLING of `doom.rs`
rather than an extension of it (§47/SC-A3 preserves the doom census as
the God-upper ground truth, so the composing module is the one that
moves): `GodGapWalk::god_gap` establishes the God upper from
`doom_enumeration`'s per-world truth where the fiber is enumerable and
from `doom_census`'s certified harvest otherwise, computes exact `Q`
through `response_success_mass`, extracts the incumbent with
`extract_success_policy` and re-prices it through the INDEPENDENT
fixed-policy evaluator `viewer_success_mass` (a different recursion —
a frozen policy, not a max — so the §36 equality receipt is a
cross-check and not a restatement), and types the coordinate as
exactly one of §48's four: `GodTightPolicy`, `PositiveGodGap`,
`GodUpper` (a nonvacuous deterministic upper with the gap beneath it
unmeasured), `UnknownGodGap`. SC-A4 is structural, not a convention:
`PositiveGodGap` holds its exact witness mass in the type, and the two
honest variants hold no value field into which an unmeasured gap could
be written. Six gates (`solver_godgap.rs`), plus `GodGapProducer` in
the §49 registry and the probe `godgapreport`
(`probes/factor_belief/godgap_run1.txt`).

THE §9 TABLE IS NOW A CHECKED NUMBER (SC-A2, the binding gate): G1
parses the per-world truth column out of the committed
`doomreport_run1.txt` and re-derives all fourteen coordinates —
matching truth mass, and `d_info = 0` asserted by exact rational
equality `Q = 1 − doomed/Z` on every one. Re-deriving the class
census's weaker harvest in the same pass turned up a small correction
to the intake companion: the record carries THREE truth-vs-census
divergences, not the two the companion names — h8-t5 0-0 (17 certified
against truth 21) joins h4-t6 0-0 (56 against 60) and h8-t5 5-3 (0
against 1). Nothing above it moves — the §9 table cites truth at every
coordinate — and the gate now pins all three
(`walt/DISCREPANCIES.md`).

THE FUSION HORIZON, MEASURED (§38, and an empirical object only —
SC-A4 forbids theorem language here): over 37 coordinates on the ten
gated roots plus the opening root, **the earliest fusion-free depth is
trick 5**. Every one of the fourteen t5/t6 coordinates is God-tight
(twelve of them substantively — h12-t6's two are whole-fiber doom);
all twelve t4 coordinates that have anything to save carry a POSITIVE
God gap, `Φ` between 6‰ and 22‰ (the largest 43/1925 at h3-t4 4-4).
The census distinguishes degenerate God-tightness from the real kind
and the horizon reading depends on it: h12-t4's four coordinates are
God-tight only because the whole fiber is doomed — where nothing is
saveable every policy is God-tight and the equality carries no
information — so t4 has 4 vacuous receipts and 12 measured prices, and
is not fusion-free. Two strata (t5, t6) are SUBSTANTIVELY fusion-free.
The number this makes concrete: at t4 the exact best information-
consistent play leaves 6–22‰ of individually saveable mass unsaved,
and `d_policy = 0` at every single coordinate — the incumbent IS the
argmax — so that remainder is not a bad policy and not physical doom.
It is the price of not knowing, and no amount of further counterexample
counting can reach it. Eighteen God-tight policies were extracted and
persisted (14 with a score profile at the declared cap); installing a
coordinate's facts makes the closure show the executable lower meeting
the deterministic upper with `Γ = 0`, which is the doom census's
upper-only store acquiring an executable bar it never had.

THE OPENING VERDICT IS UnknownGodGap, on all seven actions, and that
is the honest floor rather than a disappointment: the exact side is
unaffordable (fiber 399,072,960 against the declared cap) and the doom
side certifies zero, so the God upper is the vacuous 1 and NOTHING is
claimed about `d_info` or `d_policy` there. What would change it: an
exact `Q` at the opening (out of reach), or any nonvacuous opening doom
mass (which §47 declines to chase, and which the committed doom record
argues is near zero anyway) — or, the route the mathematics actually
points at, an information-consistency-aware upper. Jason's framing of
the round, now with a number under it: 42 is two recursions running in
opposite directions, and on this corpus they trade dominance between
trick 4 and trick 5. EXPLORATORY tier throughout; the horizon is a
measurement on the declared corpus, never a theorem.

**SLICE MB1 LANDED** (2026-09-02, the model-belief recursion joins the
solver — §§16–23 and §§29–33 of
`walt/math/model_belief_base_player_v0.1.md` under rulings MB-A1..A8
and brief `walt/briefs/BRIEF-MB1.md`; report of record
`walt/briefs/MB1-REPORT.md`): MB0's exact mixture machinery taken from
"evaluated at roots" to "runs inside the recursion," and with it the
answer to the question MB0 left open. THE NUMBER: **the model-fusion
price is STRICTLY POSITIVE at trick 4.** MB0 censused `Φ = 0` at all
fourteen of its t5/t6 root-action coordinates and reported §76's
criterion 4 as an honest NO; one stratum earlier, under the same
registered F₀/F₁ mixture at the same ν = (1/2,1/2) per hidden seat,
every substantive coordinate tested is strict — h8-t4 (fiber 1,200)
gives 47/9600, 38/9600, 90/9600, 58/9600 on its four actions (4, 3, 9,
6‰; 4.84M field reads, 98 s) and h3-t4 (fiber 11,550) gives four more
at 3, 1, 5 and 3‰ (24.2M reads, 412 s). Eight substantive trick-4
coordinates, eight strict prices. Against them h12-t4's four zeros are
typed VACUOUS — the whole fiber is decided at the root, so `U^sep` sits
at an endpoint and every lawful policy attains every point-mass optimum
for an arithmetic reason — which is U0's degenerate-God-tightness
discipline carried across unchanged and earning its keep on first use
(without it trick 4 would read "8 strict, 4 zero" instead of "8 strict,
0 substantive zeros"). The same lens turned backwards on MB0's own
corpus splits its fourteen zeros SEVEN substantive and SEVEN vacuous —
h5-t6 and h4-t6 two each and h8-t5 three, against h12-t6 (`Q = 0`),
h10-t6 and h3-t5 (`Q = 1`), which are endpoint cases where nothing was
at stake — so MB0's "14 zero / 0 strict" reads more precisely as "7
substantive zeros". THE COROLLARY that makes the finding
load-bearing, and the one piece of this slice that is mathematics
rather than measurement: Theorem 19.1 at FULL support forces the
attaining policy ρ* to satisfy `V_θ(ρ*) = q_a(θ)` at every θ
whatsoever, so `Q_a(ν′) ≥ ⟨ν′, v_{ρ*}⟩ = U^sep_a(ν′)` for any other
belief ν′ and Theorem 18.1 gives the reverse — hence a zero measured at
one full-support belief is a zero at EVERY belief over the same types.
MB0's fourteen zeros were therefore never movable by re-weighting, and
a strict specimen had to come from a new ROOT; MB1 carries the witness
explicitly (`CommonOptimizer`, present on exactly the zero coordinates)
and gate M6 sweeps a rational ν grid to confirm the single witness
policy attains `U^sep` at every point. What was built: new sibling
module `solver/model_recursion.rs` — `PosteriorTrace` /
`trace_heaviest_line` (the posterior carried down a public line, every
reported field a derived view of the one carried `ModelBelief`, no
second authority to drift from), `ModelBeliefProducer` installing `Q_a`
as a matched executable-lower/deterministic-upper pair and `U^sep_a` as
the Theorem 18.1 upper into the §49 store, `MixtureOutcome::reprice`
and `ResponseEnvelope` (§21's column-and-cut library, compared over
MASSES because one state shares its `Z_θ`, so the argmax is
division-free) with `sweep_envelope`'s audited facet count, and
`RootModelCensus` with the vacuity typing. Inside MB0's own module
(extended, never forked — the walk and the dispatch live there):
`ReadLedger`, an append-only per-type field-consultation census
recorded at the `ProfileField` dispatch itself and shared by a whole
lineage; `MixtureRefusal::ReadBudget` carrying the MEASURED spend, the
declared ceiling and the history it stopped at, with no value field in
any variant; budgeted entry points whose unbudgeted siblings are the
same walk under an absent ceiling; positive-support tightening now the
DEFAULT in `branch_masses` and `typed_branch_census` (MB0 flag 3 —
exactness-neutral by the zero-entry law, with every dropped entry's zero
completion weight independently confirmed in a gate); and
`observe_with_survivors`, the index alignment a scattering recursion
needs. THE FIELD-IDENTITY FENCE (U0's SC-A7 flag, item 7): `CoupledFact`
has private members and no public constructor, so
`couple_fixed_field_fact` is the only way a fixed-field fact reaches
the model-space recursion and a bare `Fact` does not type-check there;
`FieldCoupling` has `Identical` and `PointMassParity` and no `Assumed`;
the degenerate coupling is discharged only by a re-run WITNESS carrying
both authorities' exact pairs, and refuses where the raw authority
cannot run. `CouplingRefusal` names five reasons, every one
constructible and every one gated — a variant nothing can produce would
advertise a check that does not exist. Where structure cannot reach — the §49 store takes any
well-typed fact under a matching identity — the fence is the identity:
`mixture_identity` gives a model-belief state `field_id =
model-mixture:<content address>`, so a σ0-authored fact is rejected
`IdentityMismatch` by machinery that already existed, and a REWEIGHTED
mixture is a different identity again. MB1 TRANSPORTS NOTHING; the gate
exists so the first future transport is honest, which is the only time
it can be built. Seven gates
(`walt/walt/tests/solver_model_belief_recursion.rs`, 20.8 s in parallel, of
which 20.5 s is M6 re-deriving a trick-4 coordinate rather than citing
it): M1 recursion-versus-(ω,θ)-enumeration parity on the FULL six-root MB0
corpus (augmented masses, all fourteen `Q_a`, every `q_a(θ)` behind
every `U^sep_a`, the selected action, and the carried posterior
marginals against the surviving pairs' weighted counts); M2 the
§16/§23 repricing identity and the §21 envelope, plus item 6's
non-product prior (weights 5,1,1,1,1,1,1,5, shown non-product by the
product identity it fails, repricing to the independent prior's own
walked value because a response vector belongs to the policy and the
state); M3 point-mass collapse at DEPTH against both the enumeration and
an independently conditioned raw σ0 `response_success_mass`; M4 typed
budget refusals (a zero ceiling spends zero, a starved ceiling reports
the ledger's own measurement, an ample ceiling changes no value, a
refused coordinate proposes no fact); M5 the consumed instruments
unperturbed, doom bit-identical either side of a model census; M6 the
pinned specimen and the corollary; M7 the fence on both sides. Probe
`modelbeliefrecursionreport`
(`probes/factor_belief/modelbelief_recursion_run1.txt`).

PROBE FINDINGS beyond the headline (`modelbeliefrecursionreport`,
2,379 s): the strictly-pre-t4 coordinate **h8-t3 (fiber
59,976) REFUSES all five root actions**, each on the response side,
each having spent the declared 7,000,000-read ceiling — 35,000,039
consultations and 1,864 s for five typed refusals and no value, every
one naming the public history it stopped at, so the answer to the
question the ceiling was chosen to ask ("does a trick-3 coordinate close
within the most a trick-4 coordinate cost?") is a clean and located NO;
**78% of the probe's wall bought those five refusals**, the same shape
U0 reported when 86% of its wall returned `UnknownGodGap`, and it is not
waste because the refusal IS the measurement. The ν sweeps found **one
facet across seven grid points** at both h5-t6 and h8-t5 — a single
argmax policy optimal along the whole ν line, so six of seven beliefs
are answered by dot product alone, and at h8-t5 the VALUE moves across
the line (770, 769, 767, 766, 764, 763, 762‰) while the policy does not,
which is repricing doing real work at the cost of one walk. Where the
ceiling is checked, stated precisely because the probe makes it visible:
at the boundary of every walked bundle node, before that node is
expanded — a node that passes then classifies every live profile's
acting support, so the ledger can pass the ceiling by up to ONE NODE's
classification cost (two of h8-t3's refusals report 7,000,011 and
7,000,028 against 7,000,000). The ceiling is a budget, not a hard
bound; what is guaranteed is that the reported number is the ledger's
MEASURED total, never the ceiling and never a value rounded to it.

Deviations
recorded in the report: "pre-t4" read as "earlier than MB0's corpus"
with a strictly-pre-t4 coordinate attempted under a declared ceiling
and reported as a typed refusal rather than an absence (the smallest
trick-3 receipt fiber is 59,976 against trick 4's 1,200, and trick 4
already costs 98–412 s per root); items 3 and 4 landing inside MB0's
module because the ledger must be recorded at the dispatch and the
budget threaded through the walk, both of which live there; M5 a
property gate rather than a suite conjunction, since a test cannot run
other test files. MB0's eight gates, σ1-repair's seven and U0's six
stay green untouched (verified inside one cold `check.sh` PASS, exit 0,
all eight phases, 120 suites, zero failures); `doom.rs`, `godgap.rs`,
`refine.rs` and their
probe records are byte-identical to the pre-slice baseline. EXPLORATORY
tier; the trick-4 prices are measurements on a declared corpus, never
theorems — the §19 corollary alone is mathematics and carries Theorem
19.1's status, not this slice's.

**SLICE UP0 LANDED** (2026-09-02, the first slice of the NEW UNIFIED
WALT PLAYER — Jason's §76 GO, "the interesting one is the new one,
that's what I'm itching to play"; brief `walt/briefs/BRIEF-UP0.md`,
report of record `walt/briefs/UP0-REPORT.md`): one decision function
that consults every exact instrument the counted-belief program built,
in the order the mathematics says they become affordable, with the
model-belief posterior carried down the line, and with every decision
carrying a typed provenance naming the instrument that produced it, the
budget it spent and every refusal it fell through. New module
`solver/unified.rs` — ADDITIVE, importing its siblings and imported by
nothing but the crate root; the diff against the pre-slice baseline over
`refine.rs`, `doom.rs`, `godgap.rs`, `model_belief.rs`,
`model_recursion.rs`, `proof_state.rs`, `factor_belief.rs`,
`adaptive.rs`, `field.rs` and the old player's four binaries is EMPTY,
and the one edit outside new files is the module registration.
`UnifiedPlayer::decide(state, budget)` runs a TOTAL five-tier cascade,
each tier entered only on declared affordability and exited only by a
typed refusal: (a) the free decided arithmetic, (b) the endgame exact —
CONSUME a zero-regret necessary outer profile the §49 store already
holds, else the exact world-space recursion where the fiber affords
enumeration, (c) MB1's exact model-space response under the carried
posterior and a declared read ceiling, (d) the §33 recommendation off
the store's facts within a declared Γ, (e) the σ0 field, which always
answers and is named as the fallback it is. Every decision knows which
of the two recursions it stands in: `Recursion::direction` answers
`backward` or `forward` in one word and `space` separates the world
recursion over `Φ(C)` from the model recursion over `Ξ = Ω×Θ`, so
Jason's frame is a field of the record rather than a remark about it.
PROVENANCE IS A DERIVED VIEW: there is no `tier` field to lie in —
`Provenance::tier()` is `Evidence::tier()`, and each evidence variant
carries exactly what its tier can prove (the enumeration's exact mass
pair and its §63 re-priced mass, or the consumed fact's id and value, or
`Q(ν)` with the ledger's MEASURED spend, or the §33 block, or nothing
but the field's own name); `Provenance` and `Decision` hold private
members and the single assembly site is one private function, so a
fabricated evidence value has nowhere to go. Eighteen gates
(`walt/walt/tests/solver_unified.rs`, 17.0 s): UP1 totality over three
rungs × seven roots plus the source gate that the module holds no
`unwrap`/`panic!`/`unreachable!`/`todo!` and every `expect` in it is
annotated a rules invariant; UP2 U0's God-tight receipts seeded into the
store and CONSUMED at zero spend, re-pricing to the God upper recomputed
in the gate, with the godgap/doom instruments byte-identical either
side; UP3 the carried posterior checked as a derived view of (root,
public line) against a per-seat independent replay AND against MB1's own
`trace_heaviest_line`; UP4 starved-budget fall-through, decision
determinism, and budget monotonicity; UP5 the consumed instruments
unperturbed and the counting decorator value-neutral; UP6 every claimed
tier re-derived independently. `check.sh` PASS cold; MB0's eight,
σ1-repair's seven, U0's six and MB1's seven green untouched.

THE MEASUREMENTS, and the first is not about deciding. Splitting the
transcript's wall into time inside `decide` and time spent ADVANCING the
carried posterior: on the lean rung **99.4% of the wall (2,105,672 µs of
2,117,924 µs over 72 decisions) is carrying a posterior that no tier
consulted** — the decisions themselves cost 12,117 µs. Advancing a model
belief past a hidden play means classifying the acting seat's support
under every live profile, at every ply, for every open line, and a lean
budget affords no tier that reads the result. The lever is declared and
gated: an empty type library opens no line and changes no action, tier
or exact value the world-space tiers produced. THE JOIN, on played
lines: 27 readings, **9 moved the value (7 of them purely) and 2
flipped the argmax** (the flips move the value too — subset, not sum) —
MB1's "values move before argmaxes" separated cleanly on real play,
with both flips pinned exactly in gate UP3 — h8-t4 t4-p2, model 0-0
against fixed-field 6-2 at 617/864 versus 173/216, under an UNTOUCHED
eight-profile prior; and h3-t5 t6-p2, model 6-4 against fixed-field 3-1
at 3/5 versus 5/6, under a posterior that has already zeroed two
profiles, which makes the second the stronger of the pair (each
specimen pins its live profile count for that reason). The second flip
was transcript-only when this paragraph was first written and its
rationals were quoted here as though gated; the independent audit
caught the overclaim and the gate was extended to carry it, which is
recorded as deviation 7 in the slice report. The declared cascade plays
tier (b)'s answer and RECORDS the disagreement, and the report is
explicit that this ordering is a declared choice rather than a theorem:
MB0's point-mass parity makes tier (b) the `δ_{F₀}` special case of tier
(c), so (b) is not deeper certainty than (c) but the same recursion
under a strictly narrower belief. Inverting them is UP1's first
question and two exact specimens now exist to answer it on. THE
LIBRARY IS FALSIFIED BY THE PLAYER'S OWN PLAY: nine times across the
transcript an observed action left the registered F₀/F₁ library's
support entirely (support sets of one to three tiles), so UP0 checks the
merged branch table BEFORE observing and retires the line with a typed
`Falsification` rather than reaching MB0's positive-mass assertion — the
posterior is never repaired, re-seeded or widened. A two-rung ladder of
the same architecture does not contain the play of an exact solver, which
is a second reason to run MB1's proposed experiment with a library
chosen to disagree earlier. TIER OCCUPANCY, over 216 decisions: trick 7
is entirely free on every rung (28/28 tier (a)); tier (c) never fires
under a natural ladder, because the declared order only reaches it where
tier (b) already refused and both natural rungs put the enumeration cap
above the mixture cap — a third rung with the two structural caps
SWAPPED was added so the model tier answers and can be gate-verified,
since a transcript in which a built tier never fires has not exercised
it; and tier (d) never fires anywhere, because the only producer filling
the store is tier (b), which deposits only where it also answered — its
domain begins the moment a producer that is not the exact walk fills the
store, which is UP2's target. Probe `unifiedreport`
(`probes/factor_belief/unified_run1.txt`, 216 decisions, 31.5 s), a
TRANSCRIPT and never an evaluation: no play-strength claim, no
comparison to the existing player, arena on Jason's word. The dominant
refusal, 63 of 104, is `ProofStateUnavailable`: `ProofState::open`
asserts a trick-START root, so tiers (b1) and (d) refuse at every
mid-trick decision while (b2)'s enumeration, which needs no store, still
runs — a scope limit of the §49 spike, typed rather than worked around,
and UP1's clearest single piece of work. EXPLORATORY tier throughout.

**SLICE UP1a LANDED** (2026-09-03, the lazy carry — Jason's "do the carry
fix ... your way"; report of record `walt/briefs/UP1A-REPORT.md`): UP0's
transcript had measured that on the lean rung 99.4% of the wall was
classifying acting-seat supports for a posterior no tier read, and gate
UP3 had already proved the posterior a derived view of (root, public
line) — which makes eager advancing a stored second copy of a fact the
line holds. `solver/unified.rs` now RECORDS the line at every ply (one
`(seat, tile)` push per open line) and MATERIALIZES the posterior only
when a tier reads it (`UnifiedPlayer::materialize_line`, incremental and
idempotent), charging every consultation the materialization spends to
the decision that read it (`Spend::carry_reads`). THE NUMBERS
(`probes/factor_belief/unified_run2.txt`, the same corpus and rungs as
`unified_run1.txt`): lean rung 2,105,672 µs of carrying → 0 µs of
recording, 72 decisions in 11.6 ms all in, carry reads 0; ample and
model rungs charge 17,724 and 12,164 carry reads to their consulting
decisions and their deciding wall grows by roughly what the carrying wall
lost (wall shared with the gate suites, approximate); 27 join readings,
9 value moves, 2 argmax flips — identical. Falsifications are now
DISCOVERED at materialization, at the ply they happened: 2 of the
transcript's 9 by a consulting decision during play, 7 at a final
materialization after the hand, and a lean player that never reads its
posterior never learns its library was falsified — its provenance says
so through the new `materialized=k/n` field. Five gates
(`walt/walt/tests/solver_unified_carry.rs`, 67 s): UC1 nothing read,
nothing paid (lean rung: zero carry at every decision, every lineage
ledger at zero, and the deferred bill real when collected afterwards);
UC2 conservation — each seat's ledger total equals the sum of its
decisions' `carry_reads + mixture_reads`; UC3 the lazy discovery of a
falsification agrees with an independent eager replay on history, seat,
tile and supported set; UC4 idempotence and currency; UC5 lazy ≡ eager on
every action, evidence, refusal, frame and join reading at all 216
decisions. UP0's 18 gates green with ONE driver edit (materialize before
reading a line's views). Two defects the gates caught in the first
version, recorded: the ledger handle was lost when a retired line dropped
its belief (the falsifying read vanished from the charge — fixed by
holding the ledger on the line), and tier (c) materialized before its
free structural checks (a 2,178-read charge on the lean rung — fixed by
checking fiber and read caps first). The (b)/(c) ordering and the
trick-start proof-state boundary are untouched. EXPLORATORY tier.

**SLICE U0b LANDED** (2026-09-03, the in-solve horizon census — the §38/§40
God-gap census of `salvation_complex_v0.1.md` run at EVERY belief node the
exact recursion reaches at a declared depth, and the exact root price of
a §39 fusion cut at that depth; report of record
`walt/briefs/U0B-REPORT.md`): new `solver/horizon.rs` descends from a
root exactly as `response_success_mass` does, prices each frontier node
by exact `Q`, per-world `U^God` (the doom census's OWN line walk,
reached through `pub(crate)` visibility rather than a copy — gate H2
holds the two equal on all eighteen receipt coordinates) and `Φ`, and
re-prices the root twice, with exact leaves (asserted equal to
`response_success_mass` at the root) and with God uppers at the frontier
— what a fusion cut would compute — under the lowest-tile tie rule for
both argmaxes. Five gates (`solver_horizon.rs`, 142 s); probe
`horizonreport` (`probes/factor_belief/horizon_run1.txt`, 53 censuses,
19 min). THE FINDINGS: (1) the trick-5 frontier inside a trick-4 solve is
NOT fusion-free — 9/22/31% of frontier nodes at h8/h3/h4-t4 carry a
positive price at the receipt contract, mass-weighted 13–14‰ (U0's
"trick 5 is fusion-free" was true of fourteen uniform receipt roots
with the viewer on lead; the cut-4 root over-pricing reproduces U0's
twelve Φ exactly by partition additivity, since no focal decision lies
between the lead and that frontier — a consistency check, not a
finding); (2) the trick-6 frontier is nearly exact in value — root
over-pricing 0–7‰, exactly 0 at three h3-t4 contracts — and still FLIPS
the root play at h8-t4 under contracts 36/39 (7‰), 2 rows of 30; (3) the
price is contract-sensitive: a trick-5 cut over-prices h4-t4 by 13‰ at
bid 30 and 105‰ at 39, h8-t4 by 16‰ at 30 and 102‰ at 36 where it flips
the play, and the receipt contract is the friendliest on this corpus;
(4) h8-t3 (fiber 59,976) solved EXACTLY under σ0 in 14 min / 289M reads
— `Q* = 28859/29988` (962‰), argmax 1-1, the program's first trick-3
exact value (MB1's 31-minute refusal was the eight-profile mixture; the
single-field recursion completes) — and a cut at its trick-4 frontier
(624 of 2,098 nodes with a gap, mass-weighted 37‰) over-prices by 31‰
and FLIPS the play 1-1 → 3-3. Three layers on this corpus: trick-6 cut
0–7‰ and occasionally wrong on the play; trick-5 cut 3–105‰,
contract-sensitive, flips 3 of 15 substantive rows; trick-4 cut 31‰ and
flips. Every positive-gap frontier node is listed in the record with
history, mass, doomed count and gap — U1's input. Deterministic fields
only; no producer and no substitution built (U4's territory).
EXPLORATORY tier; the horizon is a measurement, never a theorem (SC-A4).

**SLICE FH1 LANDED** (2026-09-04, the focal-horizon hierarchy engine —
the parent's §28 generic engine of `walt/math/focal_horizon_sandwich_v0.1.md`
as narrowed by its companion and rulings FH-A1..A11; report of record
`walt/briefs/FH1-REPORT.md`): new `solver/focal_horizon.rs` computes,
per legal root action, the focal-horizon interval `[L_{a,k}, U_{a,k}]`
in exact-mass form for `k` additional focal layers — at `k = 0` the
lower is `viewer_success_mass` under the tail (σ0 driving the viewer
seat, FH-A4's primary; lowest-first as the gate tail) and the upper is
the undoomed-world count through the doom census's OWN line walk
(`horizon.rs`'s `doom_over_belief`, now `pub(crate)`); each further `k`
maxes over EVERY legal action at one public information state, hidden
branches summing at the same `k`, the argmax materialized under the
lowest-tile rule as `π_k` — a total `SlicePolicy` whose off-DAG
continuation is the tail, never the lowest tile (FH-A7). The root
reports bar, survivor set, verdict (`Settled` / `Equivalent` by FH-tie /
`Unresolved`), `L_exec = V(π_k)`, `U*_k`, `Γ_k`, the spend with forced
tail evaluations and the k-th frontier's ply histogram; a frontier node
above the fiber cap refuses the WHOLE root, typed (FH-A11, FH-A3).
`focal_depth` is the §6 walk, independent, forced nodes counting (FH-A6).
Ten gates (`solver_focal_horizon.rs`, ~6 min): endpoint parity with
`viewer_success_mass` and `doom_enumeration` on ten roots × two
contracts (harmonicity of `G` through hidden nodes checked on real
roots); `horizon_census` cut-4 = `U_{a,0}` and cut-8 = `U_{a,1}` live
(FH-cut) plus the companion's Q6 record values; nesting; collapse at
k = 6 − T with every tail consultation FORCED and zero consultations at
k = 7 − T = h_f (FH-last, FH-A6); action containment and survivor
monotonicity; `V(π_k) = L_k` through the independent evaluator at root
and every child under both tails; merge-before-max against a test-local
fused walk that EQUALS `U_{a,0}` and the salvation-mask identity for
`U_{a,1}` (Theorem 5); the FH-A8 anchor laws at h8-t4 bids 36/39;
refusal shape; determinism. Probe `focalreport`
(`probes/factor_belief/focal_run0.txt`, T4 at the receipt contract,
k = 0, 1, 2, σ0 tail). THE FINDINGS: (1) one focal layer settles h3-t4
(k = 1, bar 338‰ over the next upper 328‰) and h4-t4 settles at k = 0
already (the σ0 tail after 6-5 is worth 964‰ against every other God
upper ≤ 869‰); h8-t4 needs k = 2, survivors 4 → 3 → 1; (2) at k = 1 the
remaining width is almost all policy gap — fusion price `U − Q` 0–3‰
per action, `Q − L` 9–41‰ — so on this corpus the better-tail question
outweighs the glue question; (3) at h8-t4 k = 1 `π_1` plays 2-1 while
the exact best is 3-3, and the certified regret 39‰ contains the true
13‰ — §20 live; (4) collapse at k = 2 is 100% forced consultations, k = 3
zero; (5) h4-t4 costs 5.0M / 8.3M / 10.0M field reads at k = 0 / 1 / 2.
Budget honesty with retained intervals and suffix reuse are FH2 (gated
on FH-int); the report of record and the FH8 anchors are FH3.
EXPLORATORY tier; no live default change (FH-A10).

**SLICE FH2 LANDED** (2026-09-04, the focal-horizon ladder — the
parent's §23 sound interruption, §19 preserved facts and §25
continuation substitution as narrowed by the companion's P8/P11/P12 and
rulings FH-A3/FH-A9/FH-A11 with Proposition FH-int; report of record
`walt/briefs/FH2-REPORT.md`): new `solver/focal_ladder.rs` makes FH1's
engine ANYTIME. A `FocalLadder` is a per-root store of NODE FACTS under
FH1's identity minus the horizon — `[L(C), U(C)]` in mass form with the
policy attaining `L(C)` stored beside it and the residual horizons each
side was established at — installed by INTERSECTION only (lower = max
keeping the winning policy, the prior winning ties; upper = min), and
only when a node's whole subtree completed at its residual horizon:
nothing partial is ever written, so the fact set is a function of the
set of completed nodes. `advance(ctx, k, budget, memo)` runs the
recursion at horizon `k` under a read ceiling (field + tail reads, the
exact unit) and the fiber cap; at the ceiling the pass STOPS
deterministically and the `Interrupted` outcome carries the residual
frontier (every node whose parent was entered and which did not
complete, typed `Stopped` / `Enclosing` / `Unvisited` / `Unaffordable`,
with its mass and retained fact), the stopping node, reads spent and the
ceiling; a cap refusal leaves the enclosing root child unfinished and the
pass continues at the next (FH-A3). A resume is the same horizon again:
nodes completed at that residual return their stored fact read-free. The
root is a DERIVED VIEW of the facts — per action the stored fact or the
placeholder (lower 0 with the tail, NO upper), bar, survivors (an absent
upper survives and blocks `Settled`), verdict, `π` as the union of the
children's stored tables, `U*` and `Γ` only when every upper exists.
Decided nodes store the §5 arithmetic as a fact at zero reads. Within a
ladder a collapsed fact (`L = U`) becomes a receipt in a `SuffixMemo`
keyed by the belief's FULL identity (bucketed by history, matched by
`FactorBelief`'s componentwise equality — root, position incl. contract,
history, field id, the posterior with weights); a later pass returns the
receipt instead of descending (P12). `FocalHorizonProducer:
ProofProducer` emits `Fact::Bound` per action — lowers under
`focal-horizon:<tail id>:k=<k>:lower`, executable iff the stored policy
re-prices to the value through `viewer_success_mass` AT PRODUCTION;
uppers under `focal-horizon:god:k=<k>:upper`, never executable — with
retained values from an interrupted pass. FH1's engine gained one
`pub(crate)` seam (`Engine::price_frontier`, the k = 0 leaf both walks
call) and is otherwise unchanged; its ten gates stay green. Ten gates
(`solver_focal_ladder.rs`, ~24 s standalone): parity with `focal_horizon`
(a fresh direct-k ladder reproduces FH1's `π_k` id; the sequential
ladder every value view); FH7's five bullets at h8-t4 (no child dropped,
retained k = 0 facts equal the uncapped run's, `Q_a` inside every
interval, the boundary typed, resume + completion ≡ uninterrupted
bytewise on the fact render and on every derived view, the spend as a
sum); monotone under a pinned eight-step ceiling schedule; the
placeholder is not a fact (h4-t6 and h8-t4 at ceiling 0 — no interval,
no regret — against h10-t6 where decided children are facts at zero
reads); proof-state install/closure parity and tightening through k = 2;
executability honest including the retained k = 0 lower; suffix reuse
invisible in value at h8-t4 and h3-t4 with the first hit PINNED at
[2-1 0-0 2-0 3-0]; the identity is the full belief (a frozen contract-30
memo answers a contract-36 ladder with zero hits; a belief narrowed in
one factor alone misses); determinism of an interrupted run; the
in-pass fiber-cap refusal (FH-C, added post-audit for FH4-AUDIT N3: an
h8-t4 k = 0 pass at cap 40 refuses under two root children and completes
the other two — `unaffordable` non-empty, no stopping node, no fact at
or under a refused node, the enclosing children unfinished with
placeholders, the others at the uncapped run's k = 0 facts, the ample
cap then completing). Probe
`focalreport ladder` / `ladder-record`
(`probes/factor_belief/focal_ladder_run1.txt`, h8-t4 and h3-t4, pinned
schedules). THE FINDINGS (exploratory): (1) at h3-t4 the INTERRUPTED
k = 1 pass settles 3-1 at 1.20M reads with 6-4 still unfinished — its
retained k = 0 upper already sits below the new bar — where FH1's k = 1
needed 2.63M; (2) the suffix memo cuts the k = 2 pass to 0.42M reads at
h3-t4 (FH1: 2.83M) and 0.13M at h8-t4 (FH1: 0.66M) with identical views;
(3) the price: peak RSS at h3-t4 662 MB memo-on / 509 MB memo-off against
FH1's 411 MB (the fact store's per-node policy tables and the memo's
belief clones), and a resume re-reads the stopped chain (h8-t4: under 0.4%
over the uninterrupted pass). EXPLORATORY tier; no live default change
(FH-A10).

**SLICE FH3 LANDED** (2026-09-04, the report of record and the FH8
anchors — the parent's §38 measurements, §37 anchors and §39–§41 verdict
as narrowed by FH-A8 (the answers discovered, never pinned); report of
record `walt/briefs/FH3-REPORT.md`): `focalreport report` writes
`probes/factor_belief/focal_run1.txt` — at every (root, contract) of the
corpus (T4 × {receipt, 33, 36, 39, 42}, T56 × {receipt, 36}) and the
h8-t3 anchor, horizons k ∈ {0, 1, 2, 3}, the direct engine per k (reads
WITHOUT reuse, on a fresh σ0 instance) beside ONE memo-on ladder per
coordinate (reads WITH reuse); per action `L`, `U`, `U − L`, `Δ^L`,
`Δ^U`, the survivor mark, `Q_a` by a fresh `response_success_mass` (the
record's 14-minute values, cited, at h8-t3) and the split; per horizon
the bar, survivors, verdict, both `π_k` ids where the ladder's differs
on a tie, `L_exec`, `U*`, `Γ_k`, the lower policy's root action and its
changes by horizon, the ply cut's argmax under Proposition FH-cut and
whether any horizon certifies it, `h_f`, the first suffix hit; the §41
laws asserted at every coordinate (a failure stops the record naming
it — none fired). Gate file `walt/walt/tests/solver_focal_anchors.rs`
(4 gates; post-audit for FH4-AUDIT N13 the fixture admits at most five
h4-t4 jobs at once — every h4-t4 evaluation costs ~1.6 GB whatever its
kind, so the 18.2 GB / 78 s standalone peak was eighteen threads each
holding one, not the ladders alone — now 8.8 GB / 165 s standalone,
248 s in-gate; `check.sh` PASS, 123 binaries) at anchors (ii) h8-t4 × {36, 39} and (iii) h4-t4 × five
contracts with `Q_a` recomputed independently and the ply cut recomputed
live: FHA1 containment/nesting and FH-last collapse at k = 2;
FHA2 `Settled ⇒` unique exact argmax, `Equivalent ⇒` the exact tie set,
`Unresolved ⊇` the exact maximizers, and FH-A8's law in conditional
form; FHA3 the census's cut readings ARE `U_{a,0}`/`U_{a,1}` (FH-cut)
and wherever the cut's argmax is not exact no horizon certifies it;
FHA4 the memo-on ladder equals the direct engine in every value. Anchor
(i) h8-t3 is probe-only (its k ≤ 2 ladder alone is ~9 min and 17 GB).
THE FINDINGS (exploratory): (1) every live trick-4 coordinate settles by
k ≤ 2 — five at k = 0 (h4-t4 at 30/33/36/42, h3-t4 at 42), six more at
k = 1 (h3-t4 at 30/33/36/39, h4-t4 at 39, h8-t4 at 33), the last three
at k = 2 (h8-t4 at 30/36/39, the collapse); Γ_1 ≤ 45‰ everywhere at
trick 4; (2) anchor (ii) settles 2-1 only at k = 2, exactly as FH-A8's
law said (`U_{5-5,1} = 757‰ > Q_{2-1} = 750‰`), and neither the cut-4
argmax (3-3) nor the cut-8 argmax (5-5) is ever certified; (3) anchor
(iii) settles 6-5 at k = 0 at four contracts and at k = 1 at bid 39
(`B_0 = 651‰` against `U_{4-0,0} = 655‰` — the tail-quality question of
FH-A8 answered by 4‰); (4) anchor (i) h8-t3: k = 0 Γ 141‰ / k = 1 100‰
/ k = 2 34‰ with survivors {1-1 2-1 3-3} and π_2 already playing the
exact 1-1 uncertified; k = 3 (the FH-last collapse, run because k = 2
came in under the 10-minute gate) SETTLES 1-1 at `28859/29988` = the
record's 14-minute exact value, in 27.2M + 73.5M + 69.7M + 20.0M reads
with reuse (the direct engine: 27.2M at k = 0, 79.2M at k = 1) — the
cut-4 argmax 3-3 that flipped the record's play is never certified;
(5) the cost trend, up front: the fact store at trick 3 is 3.82M facts
and the record run's peak RSS 19.4 GB (FH2's 662 MB at h3-t4 → 17–19 GB at
h8-t3, 7.8 GB for five concurrent h4-t4 coordinates); reads per horizon
are unchanged from FH1/FH2; the σ0 cache makes a warm ladder pass 15×
faster in wall than a cold one at identical reads (h8-t3 k = 0: 154 s
cold, 10 s warm), which is why the record uses a fresh field instance
per direct run. EXPLORATORY tier; no live default change (FH-A10).

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
