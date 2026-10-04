# Independent Sol adversarial audit

2026-10-04. **Exploratory research**, with kernel evidence separately identified.
Requested collaborator role: gpt-6.1-sol; this is not runtime attestation.
Latest constructive checkpoint: the full own-choice frontier independently
matches all 105 development-root action vectors while taking about 0.21 s
versus 0.39 s for the same-runtime recursive reference. Restricted shared plans
also evaluate correctly and cheaply; their acknowledged class limitation is
quantified rather than treated as a contradiction of Claude's proposal.
Read CLAUDE.md and QUICKSTART.md. All work used the isolated worktree checkpoint
`2aca32a8dce3cc7fcc866c420bc1633144400877`. No edits or executions in the first
team's worktree, browser/Claude communication, dependency installation, neural
training, paid resources, deployment, or production merge. All new files are
under this `sol/` directory; every experiment used the original capped runner.

## 1. Archived sampler accepts an illegal hidden world — concrete contradiction

**Exploratory executable witness.** The first team's static warning survives and
is now concrete, but its wording needs precision: the hard-coded bidder is in
`consistent_worlds`, **not** the separate `replay` function. Drive
`incoming/drive/engine42.py:178` creates `Game(...,30,0)` and claims the bidder is
irrelevant; `:94` ignores play after `done`; `:112` stops at defender points 13;
`:182` continues checking the history anyway. In contrast `replay` at `:193`
uses the supplied bidder. The first-team claim is `phase2/REPORT.md:111`.
Paths in this report beginning `incoming/` are relative to
`experiments/astra-sol-20261004/phase2/`; other first-team paths are spelled out.

The complete saved fixture is
[bidder-counterexample-numpy/stdout.log](receipts/bidder-counterexample-numpy/stdout.log).
Trump 2, bid 30, bidder 1, viewer 0. Its actual full deal and 16 public plays
replay legally under the independent partnership rules, ending even-team 7,
odd-team 17, four completed tricks, viewer 0 on lead with tiles `[4,16,17]`.
Neither true settlement threshold has been reached. The archived sampler
returns a hidden deal in which, at zero-based ply 7, seat 2 plays tile 0 (0-0)
after lead tile 6 (3-0), although it holds tile 24 (6-3) and must follow with 24.
All 16 archived legality checks accept it: the fake bidder-0 replay stopped
after the first trick, when the odd side took 16 points, then left an empty
table for later checks. Its state reports one trick rather than four.

Search receipt: [bidder-counterexample-numpy/run.json](receipts/bidder-counterexample-numpy/run.json),
0.210 s, source SHA256
`0f6a36b483ab7659ad7b23eed2c562cb06e90d8c2ecd69e17702d7f5daf52852`.
The direct saved-fixture reproduction
[bidder-fixture-verify/stdout.log](receipts/bidder-fixture-verify/stdout.log)
also tests the original ZIP engine, SHA256
`94bdbf50b9596bf9788dd5e3a830ba21756565390844718461a2e570fd134199`.
Both accept the revoke. Suppressing only premature settlement in the local
replay causes both to reject it at ply 7. No incoming file was changed.
This disproves these archived samplers' advertised history-consistent support;
it supplies no production Walt or match-strength contradiction.

## 2. Stochastic no-fusion likelihood omission — actual Texas42 impact unresolved

**Exploratory source finding, not a legal-game contradiction.** Drive
`incoming/drive/nofusion_sc.py:44` stores only the local chance weight;
`:63` selects later shared focal actions using unweighted continuation sums;
`:67` applies chance weights afterward. The first team's equal-prior abstract
reversal at `phase2/REPORT.md:104` correctly identifies what can go wrong in a
generic stochastic information tree. Unit-weight frozen tapes avoid this
particular prefix-weight issue. This does not establish a Texas42 action error.

I wrote an independent scalar rational observation-tree solver: each nonfocal
edge divides its scenario's cumulative reach mass by its legal-move count;
at focal nodes it selects one shared action using the weighted bundle. The
archived vectorized expectimax and this solver showed **zero value differences**
on 515 random legal roots after 16 public plays and 138 after 12 plays, with
up to five uniform-prior legal remaining worlds per root. The latter run
independently replays every supplied full deal. Those bundles are deliberately
small and are not posterior estimates or exhaustive coverage. Neither search
establishes correctness or refutes the static concern. No action reversal or
legal base-29 history collision was established.

[nofusion_counterexample.py](nofusion_counterexample.py) and receipts
[nofusion-search](receipts/nofusion-search/run.json) /
[nofusion-search-12](receipts/nofusion-search-12/run.json) retain the null results.
They exit 1 for “no witness found”; the runner's `failed` status here is the
declared search outcome, not an exception. The 12-play run self-stopped after
50.387 s within its 60 s allowance. No broader search is needed for this audit.

## 3. Uniform neural teacher is lawful; its comparator is fused

**Exploratory source audit.** The claim at `phase2/REPORT.md:127` survives:
`incoming/rollout_net.py:29` generates random hidden deals and `:38`–`:43`
plays uniformly from each acting seat's own legal hand. A simulator may know
the world while executing an information-admissible fixed continuation policy.
Its labels estimate that uniform policy's outcome under the training occupancy,
not optimal lawful continuation values. Seeing hidden data in simulation is
insufficient to infer fusion. `TeacherFusion.lean:16`–`:34` is explicitly an
abstract oracle-label example, not a claim about this uniform teacher.

The comparison at `incoming/rollout_net.py:148` invokes `walt_decide`.
`incoming/drive/engine42.py:225`–`:227` optimizes own orderings separately per
world before adding them. A legal root input does not repair that continuation
relaxation. Consequently the reported disagreement/regret is against an
archived fused reference, not a demonstrated production phone-player regret.
This survives the independent challenge and is separate from verdict 1.

## 4. Same-game validation leakage — independently quantified

**Exploratory execution evidence.** `incoming/rollout_net.py:45`–`:46`
repeats each game's final label on its position rows; `:81`–`:82` permutes rows,
not games. [teacher_audit.py](teacher_audit.py) extracts only the three data
functions through AST, appending the already-created game-id array to the
return value for instrumentation. It executes no top-level training or save
code. It matches seed 42, 14,000 games, own hand and trump, and the parameter
initialization random draws before the row split.

The generation produced 276,244 rows. Training contains all 14,000 games;
validation contains 20,000 rows across 10,375 games, and **all 20,000 validation
rows have another row of their same game in training**. Every label equals its
game's final outcome. This is exact source/split evidence, not a measured
validation-accuracy inflation. The 56,577-parameter count independently agrees.
[teacher-data-split/stdout.log](receipts/teacher-data-split/stdout.log), 0.206 s.

## 5. Thirty original Lean declarations survive; application scope is conditional

**Kernel evidence.** Fresh checks with Lean `v4.33.0-rc1` passed all six files:

| Source relative to experiments/astra-sol-20261004 | Declarations | Fresh receipt |
|---|---:|---|
| math/lead/Disagreement.lean | 6 | [phase1-lead](receipts/phase1-lead/run.json) |
| math/sol/ArgmaxAmplification.lean | 6 | [phase1-sol](receipts/phase1-sol/run.json) |
| phase2/math/sol/FirstDivergence.lean | 4 | [phase2-coupling](receipts/phase2-coupling/run.json) |
| phase2/TeacherFusion.lean | 6 | [phase2-teacher](receipts/phase2-teacher/run.json) |
| phase3/math/sol/CompiledDAG.lean | 6 | [phase3-dag](receipts/phase3-dag/run.json) |
| phase3/math/sol/InnerFiber.lean | 2 | [phase3-inner](receipts/phase3-inner/run.json) |

No `sorry`, `admit`, custom `axiom`, or `native_decide`; printed dependencies
contain only standard axioms. Each receipt includes command, allowance, exit,
elapsed, stdout and stderr. The six concurrent small checks took 1.095–1.215 s
each, so these durations are not Lean performance measurements.
[receipt-audit/stdout.log](receipts/receipt-audit/stdout.log) gives source hashes,
all declaration counts and complete printed axiom dependencies.

`Disagreement.lean:12` requires payoff coupling; `:39` requires uniform policy
error and an attained source value. `ArgmaxAmplification.lean:53` changes the
host's conditioning relative to the modeled belief; it is not a continuity
counterexample under one unchanged belief. `FirstDivergence.lean:25` returns
current payoff at horizon zero and `:34` makes the zero-horizon certificate
true: its `:48` equality is for the chosen finite replay horizon, so game
completion needs a separate sufficient-horizon argument. `Lawful` at `:44`
requires focal action legality; field legality and game semantics are caller
obligations. These do not invalidate the proofs.

`CompiledDAG.lean:34` is a functional persistent store, not verified array
indexing or a complexity theorem; `:41` proves evaluation equivalence for the
supplied acyclic graph. Correct compilation and shared-information graph
construction remain outside it, as the phase3 report explicitly acknowledges.
`InnerFiber.lean:14` proves its four-world Boolean example, not a Texas42 fiber
compiler theorem. None of these files proves a hardware speedup, general
higher-k scaling, or complete production-program refinement.

## 6. Constructive action parallelism passes semantic review; speed scope is narrow

**Exploratory code/receipt audit, not an independent timing rerun.** Reviewed
`../parallel_roots.py:76`–`:191` and its 20 retained timing rows. Each root
action keeps its complete world/tape bundle; `:93`–`:103` uses a shared focal
MAX and groups public nonfocal actions. Postorder packed child references
precede parents. Fixed-bid pruning at `:90` uses the correct team thresholds.
All 134 action vectors and 292,449 visited nodes agree across the 20 runs;
87 roots have partial tricks. Duplicate `(world,tape)` scenarios retain weight.

The saved median cold solve times reproduce: recursive serial 0.939 s versus
four-worker 0.312 s; compiled serial 1.052 s versus four-worker 0.343 s. The
common serial fixture generation costs 0.434 s, so end-to-end gains including
it are materially smaller. The worker timing includes pool startup, IPC and
shutdown; `jobs_of` preparation is excluded on both sides. Worker CPU totals
exclude bootstrap work. The summed worker peaks omit coordinator RSS and are
not simultaneous memory measurements. The 6,139,589 packed bytes are aggregate
job payload, not peak resident graph memory. A small pilot is slower parallel.
These results concern completed Python frozen-Dice roots, not native Walt or
higher-k fibers. Existing native root parallelism is a separate baseline.

I supplied [SettledPruning.lean](SettledPruning.lean): `:8` proves made remains
made under nondecreasing bidder points; `:12` proves a defender threshold keeps
the bid set under monotone defender points and final total at most 42; `:20`
proves action-list values unchanged when workers return the same semantic Q.
All three pass [settled-pruning](receipts/settled-pruning/run.json), with standard
axioms only. Game-transition and worker correctness premises remain explicit.
Early-stop graphs specialize the bid; changing it requires recompilation.

## 7. Fixed-world ordering realization is valid; shared ordering is incomplete

**Kernel-local lemma plus exploratory finite-game evidence.** For one physical
world and fixed deterministic other-seat tape, take any legal own-choice tree
leaf. List its own cards in the order actually played, then append unplayed own
cards arbitrarily if settlement ended the hand early. At each own turn every
earlier priority card is already removed; the desired next card is legal; all
unplayed future cards are later in the order. First-legal therefore chooses the
desired card. Identical prior actions yield identical deterministic other-seat
continuations, so induction reproduces the complete leaf trace. Conversely,
every first-legal ordering is a legal own-choice tree path. Thus trace sets and
their minima/maxima are equal **for one fixed world/tape**. No hidden choice
needs to be shared with another world in this argument.

[PriorityRealization.lean:15](/Users/jason/Documents/Codex/2026-10-04/task-2/research-worktree/experiments/adversarial-20261004/sol/PriorityRealization.lean:15)
proves the first-legal step for an unavailable chronological prefix and legal
next card. The complete deterministic engine trace induction above is a
mathematical argument, not a full Lean engine refinement. The other three
declarations formalize the distinction: two publicly different observations
with identical remaining own legal actions can require opposite first actions;
an observation-adaptive policy wins twice, while every common priority order
wins once. This is an abstract lawfulness/completeness gap, not a Texas42 gap
fixture. [priority-realization](receipts/priority-realization/run.json) checks
all four declarations, standard axioms only, 0.266 s.

[ordering_trace_audit.py](ordering_trace_audit.py) independently enumerated
all 9,648 own orderings on 402 fixed scenarios (three worlds each on the 134
fresh parent roots). Their full public leaf-trace sets exactly equal the
adaptive own-choice trees' 3,718 distinct leaves. The run uses full histories,
same deterministic tape interpretation and fixed-bid early settlement.
[ordering-trace-audit/stdout.log](receipts/ordering-trace-audit/stdout.log),
0.389 s, retains all case counts and the source plan hash.

The archive's `incoming/drive/exact_tape.py:55`–`:57` and `:86` both take the
per-world extremum before averaging. Their mutual equivalence is a sound
fixed-world optimization check; it does not establish that a single shared
ordering is complete among lawful history-adaptive cross-world policies.
Claude explicitly qualified shared orderings as a restricted class. The gaps
here quantify that acknowledged limitation and validate the design distinction;
they are not a refutation of its stated restricted-plan proposal.

## 8. Native rung demand is substantial but measured under voidless modeled beliefs

**Exploratory source and receipt audit.** The native wrapper rotates the viewer
by replay's `r` at `../native/src/main.rs:24`, maps bidder partnership to internal
T1, and complements T1 counts for a T0 focal player at `:44`; no rotation flaw
was found. It uses exact rational root values on eight sampled outer scenarios,
inner budgets `[4,2,2,2]`, modeled levels 0–3, and pinned production-core code.
The 144 saved midgame runs on 18 roots complete; every serial/parallel action
vector agrees. Node and time totals independently match the parent receipts.

The outer `sample_belief` at `:34` honors replayed public voids, but `Key.voids`
is `None` at `:26`, and `Shared::new` retains default `InnerBelief::Voidless`.
Its modeled actors sample their own hands/capacities and played-card information
while **ignoring historical void deductions**. These are actor-specific
voidless modeled beliefs, not exact actor mechanical-support fibers.
`walt/walt/src/solver/mod.rs:1237` checks the policy cache before `:1251`
increments `pi_calls`; that instrumentation counts miss computations, not all
consultations or distinct information states. Parallel miss work may duplicate
the same key. Retained cache entries count only completed stored policies.

Serial nodes by modeled field are 129,984 / 795,414 / 3,367,805 / 10,874,648;
serial solve times are 4,857 / 21,455 / 97,919 / 365,610 microseconds. These
support steep demand growth on this fixed-budget panel. They do not prove a
general k lower bound, full information-key coverage, or phone-player behavior.
[receipt-audit-final/stdout.log](receipts/receipt-audit-final/stdout.log) retains
independent totals and all-pair equality checks, without a timing rerun.

## 9. Small-fiber all_deals and tape boundaries survive narrower checks

**Exploratory finite-fixture evidence and source reasoning.** On the saved
odd-bidder legal history, `incoming/drive/nofusion_sc.py:10`–`:24` enumerates
700 remaining deals, all distinct; restoring played cards and replaying every
deal with independent rules succeeds. The falsely accepted sampler world from
verdict 1 is absent. `all_deals` supplies the actual initial bidder at `:24`,
so it does not share the hard-coded-bidder bug. Its nested combinations assign
each ordered-seat partition once; no duplicate-generating permutation loop was
found. This checks one finite fiber, not every history or the cap/refusal path.
[support-enumeration/stdout.log](receipts/support-enumeration/stdout.log), 0.084 s.

The archive's NumPy tape code uses nonnegative truncation of `u*n`, clamped at
`n-1`, and the constructive integer code uses `(u*n)>>64`. Thus `u=0` selects
legal index zero. The header's `ceil` wording is not the exact boundary spec.
On a finite random grid, bucket sizes differ by at most one and are exactly
equal only if the legal count divides the grid size. This tiny finite-grid
distinction does not change exactness of a **declared frozen tape** evaluation;
it does prevent identifying finite-tape replay with exact integration over
continuous uniform choices. The saved arithmetic diagnostic uses a 256-point
grid to expose the distinction without claiming a floating-point theorem.
Grouping histories alone is insufficient for stochastic exactness: cumulative
reach weights must be retained at the shared decision, as verdict 2 explains.

## 10. Constructive sampler repair preserves the exact fixture support

**Exploratory repair and exhaustive fixture evidence.**
[repaired_sampler.py](repaired_sampler.py) preserves the pinned original sampler
signature and proposal stream, and adds only `g.done[:] = False` after each
historical `g.play`. The known-valid historical legality replay then updates
all hands, tables and leaders regardless of the sampler's irrelevant local
bidder-0 score thresholds. An AST extraction keeps the original immutable.
[consistent_worlds.patch](consistent_worlds.patch) is the equivalent reviewable
source patch; it was not applied to incoming or production files.

[repair_audit.py](repair_audit.py) assigns the nine unseen cards exhaustively
to three capacity-three hidden hands: 1,680 capacity assignments. The repaired
acceptance set equals all 700 archived exact `all_deals` worlds and excludes
the known revoke world. Three fixed seeds produce 128 repaired samples each;
all 384 restore to independently legal histories, preserve the viewer's own
remaining hand and end at the expected scores/leader.
[sampler-repair/stdout.log](receipts/sampler-repair/stdout.log), 0.132 s.
This is a concrete narrow repair for known-valid histories, not a hardened
adapter or behavioral action-likelihood posterior. The shuffle/rejection
argument requires uniform proposals and the correct acceptance predicate;
these property checks are not a statistical uniformity theorem.

## 11. New batched v2 fill mask works on the fixture; source attribution needs correction

**Exploratory source and fixture checks.** The later immutable snapshot
`../incoming/drive-0815/batched.v2.py` supplies the actual bidder and returns
unfilled slots as zeros with a Boolean mask. Under the same legal bidder-1
history and four viewer-hand problems, tries 0 gives 0/128 filled, tries 1 gives
117/128, and tries 6 gives 128/128. Every reported-filled world independently
replays legally; all unfilled slots remain zero. Source SHA256
`a16a852f808a5b8aa4dfccb0d483224ebcae77daf9873f319830cda649adcb00`.
[batched-v2-audit/stdout.log](receipts/batched-v2-audit/stdout.log), 0.090 s.
The absent `l2.py` caller prevents verifying that real consumers honor the mask
and retain problem-local duplicate weights; no integration claim is made.

The new
[AUDIT-RESPONSE.md:12](/Users/jason/Documents/Codex/2026-10-04/task-2/research-worktree/experiments/adversarial-20261004/incoming/drive-0815/AUDIT-RESPONSE.md:12)
incorrectly says archived `engine42.consistent_worlds` was unaffected.
[MATH-TEAM-STATUS.md:12](/Users/jason/Documents/Codex/2026-10-04/task-2/research-worktree/experiments/adversarial-20261004/incoming/drive-0815/MATH-TEAM-STATUS.md:12)
incorrectly attributes the math team's bypassed original sampler bug to newly
supplied `batched.py` v1. There are two separate failures: verdict 1's original
hard-coded-bidder premature settlement, and `batched.py:47`'s unchecked fallback.
V2's correct bidder/fill-mask implementation does not erase the original witness.
The fixed-world realization qualification in response item 3 is accurate.
Its historical hand-strength numbers remain attributed, not reproduced here.

## 12. Vector shared plans and nested public-signal plans are valid constructive steps

**Exploratory independent complete-matrix checks.** `../vector_orders.py`
uses the actual bidder, correctly oriented scores and partial trick state,
avoids the broken sampler, and sums world rows by the same plan tuple before
maximizing. Its split-32-bit multiplication equals the high 64 bits exactly:
write `u = H*2^32 + L`, so
`floor(u*n/2^64) = floor((H*n + floor(L*n/2^32))/2^32)`.
For at most seven legal tiles all intermediate products fit well within uint64.
The independent rerun checks all 100,800 payoff cells and values on 105 pip roots.
[vector-orders-recheck](receipts/vector-orders-recheck/run.json), 2.334 s.
Its three array-preparation-inclusive evaluations take about 0.184 s versus
1.304 s scalar plans and 0.388 s grouped trees in that runtime. Imports/common
fixtures are excluded, arrays are a partial census, and these are batch CPU
measurements rather than GPU or native-player claims.

`../refined_orders.py:26` uses only whether the **public current partial trick**
is won by the focal seat's partner. Empty tricks set the bit false. Each plan is
a shared `(base, alternative)` tuple; static `(base,base)` members remain.
Three adjacent-swap alternatives per base produce 96 plans. The same public
signal is used at the root to assign plans to root-action values, so root
conditioning is correct even when the partner is already winning.

[refinement_audit.py](refinement_audit.py) reimplements scalar play independently
with partnership rules and checks the complete 403,200-cell refined matrix,
not the parent's 57,120-cell spot check. It also verifies equal public history
and same plan choose the same legal own action: 537,144 distinct observations,
468,484 repeated-observation consistency checks. All per-action values satisfy
`static <= refined <= full lawful tree`, and all saved vectors match.
Summed best-value gaps improve 181→164 mass units over 105 roots/40 scenarios;
summed root regrets improve 15→11, with 12 value improvements and three regret
improvements. None worsens on this panel.
[refinement-full-audit/stdout.log](receipts/refinement-full-audit/stdout.log),
6.780 s. The 96-plan evaluations cost 0.744–0.752 s, reflecting four times the
rows and exceeding the grouped-tree baseline on these roots. This is a valid
development tradeoff, not held-out strength evidence.

[NestedPlans.lean](NestedPlans.lean) adds two kernel declarations:
class inclusion plus attainment preserves/increases the restricted objective;
per-action improvements do **not** imply monotonically improving full-tree
root-action regret. The explicit numerical example is true values A=100,B=99,
base values 98,90 and refined values 98,99: greedy choice switches from zero
regret to regret one while the restricted objective improves. Its conditions
remain explicit. [nested-plans](receipts/nested-plans/run.json), 0.262 s.

## 13. Full-tree frontier is the strongest tested throughput checkpoint

**Exploratory code review, independent rerun and stress.**
`../frontier.py:46` groups `(previous history ID, played tile)` exactly at each
depth. Root identity initializes those IDs; injectivity by depth preserves
full public prefixes without base-29 truncation/hash collisions. Different
roots cannot combine their values. Each focal backward fold sums all surviving
unit-weight scenario rows for a shared action before choosing one shared MAX.
Nonfocal nature has one deterministic successor per frozen scenario; absorbed
terminal rows use a `-1` continuation and carry their payoffs backward. Empty
focal layers are handled without manufacturing choices. This matches the
finite frozen-Dice objective, including duplicated scenarios as weight.

The independent four-repeat alternating-order rerun matches every full action
vector on 105 roots, including 69 partial tricks. Frontier times are about
0.209–0.211 s versus 0.386–0.391 s recursive reference, including preparation,
sorting and forward branch materialization.
[frontier-recheck](receipts/frontier-recheck/run.json), 2.475 s; source SHA256
`9984d25e6fde4e6ac722b20a7f046cf1d23ffc01f9112ec2e13a771539c49e4e`.
This evaluates the full own-choice tree rather than a restricted plan class.

[frontier_stress.py](frontier_stress.py) separately tests 12 roots with mixed
bundle sizes 1/1/120/40, identical roots assigned distinct output identifiers,
root permutation, triplicated world/tape weights and 216 mixed terminal steps.
All reference vectors agree; triplication triples every value. Its 30,002
boundary/random uint64 high-multiply cases exactly match Python's unbounded
integer oracle. [frontier-stress-final](receipts/frontier-stress-final/run.json),
0.383 s. The first stress attempt reused a seed for two different output roots;
the seed-keyed answer map then overwrote a result. That was my invalid harness
identifier choice, corrected to unique root identifiers; the failed receipt
is retained. This prototype is not a general request adapter.

Parent's fresh 52-root panel reports a useful size crossover: at eight worlds
frontier 0.076 s is slower than recursive 0.047 s; at 40 worlds about 0.137 s
beats 0.219 s; at 128 worlds about 0.313 s beats 0.663 s. These are arithmetic
readings of the retained fresh-panel receipts, not another timing rerun. Import
and common source-fixture generation are excluded; sampling 128 worlds costs
0.349 s separately. Array census omits temporary/old-engine allocations; RSS
is process high-water. Keep batch throughput distinct from one-turn latency.
The batching supports a concrete engineering hypothesis; actor-specific inner
sampling, broader declarations, GPU/native refinement and higher-k dependencies
remain separate work. No runtime/memory/k-linearity theorem is established.

## 14. Final revised engine passes the exact local witness; regression rate needs qualification

**Independently reproduced repair; corrected attribution.** The final incoming
[engine42.v2.py](../incoming/drive-final/engine42.v2.py), SHA256
`db020fc8a47741c6de4596fad96950bd636af476ec7a6b4e2bcee50e5cd94a75`,
adds `no_stop` at construction, suppresses settlement at line 116, and uses
`no_stop=True` in the sampler at line 178. This has the same historical replay
semantics as the narrow repair in section 10 for known valid public histories.
[final_packet_audit.py](final_packet_audit.py) loads that exact saved source.
It reproduces the old acceptance, rejects our exact illegal witness at
zero-based ply 7, and checks all 1,680 valid candidate partitions: precisely
700 are accepted, equal to the independently enumerated legal support. Across
three seeds all 384 sampled full deals independently replay legally, have four
disjoint seven-tile hands covering the deck, and preserve the viewer's current
hand. [final-packet-fixed](receipts/final-packet-fixed/run.json), 0.088 s.
The first invocation omitted the reference module's import path; that harness
failure is retained in `receipts/final-packet`, then corrected. Neither receipt
transmits the fixture externally. This is a finite support/property result,
not a policy-posterior or statistical uniformity theorem.

[AUDIT-RESPONSE-2.md](../incoming/drive-final/AUDIT-RESPONSE-2.md) explicitly
supersedes and corrects the old claim that the original sampler was unaffected.
That resolves the attribution issue in section 11. It also separates class gap
against the lawful shared-history tree from the additional fusion slack of
per-world optimization. The new [flatplan.v3.py](../incoming/drive-final/flatplan.v3.py)
does draw one canonical deduplicated ordering list (lines 18–25), sums worlds
before optimizing a shared plan (39–44), and calls the lawful tree on the same
worlds and tapes (64). Those source changes are coherent; the packet's new
benchmark numbers have not been independently rerun here. Its modified
`nofusion_sc` and `batched` dependency files are not supplied with this packet,
so the combined `test_support.py` claim of 37,872 checked worlds is also an
external result, not a local rerun receipt.

The old-path arm of
[test_support.py:67](/Users/jason/Documents/Codex/2026-10-04/task-2/research-worktree/experiments/adversarial-20261004/incoming/drive-final/test_support.py:67)
passes a **current remaining** own hand to `random_deals`, which treats that
mask as an original hand and assigns seven tiles to each other seat. It then
ORs historical tiles back into those candidates. This can omit deck tiles,
overlap hands and give the wrong original hand sizes. Our exact fixture
reproduces 256/256 invalid original deals under that proposal, all accepted by
the old replay. Therefore its reported 3,731/13,288 (28%) is not established as
the illegal-world rate of the actual old `consistent_worlds` proposal. This
does not weaken the original valid-deal counterexample or the v2 repair.

The final manifest independently confirms **39 kernel declarations: 30
original plus nine added**; their individual fresh Lean executions and axiom
outputs are retained. [receipt-audit-latest](receipts/receipt-audit-latest/run.json),
0.041 s. Their abstract scope and semantic premises remain those in sections
5–6 and 12; the count is not an engine/compiler refinement guarantee.

## Clickable source locations

- [Archived bidder assumption](/Users/jason/Documents/Codex/2026-10-04/task-2/research-worktree/experiments/astra-sol-20261004/phase2/incoming/drive/engine42.py:178)
- [Stochastic focal unweighted sum](/Users/jason/Documents/Codex/2026-10-04/task-2/research-worktree/experiments/astra-sol-20261004/phase2/incoming/drive/nofusion_sc.py:63)
- [Uniform teacher](/Users/jason/Documents/Codex/2026-10-04/task-2/research-worktree/experiments/astra-sol-20261004/phase2/incoming/rollout_net.py:29)
- [Row split](/Users/jason/Documents/Codex/2026-10-04/task-2/research-worktree/experiments/astra-sol-20261004/phase2/incoming/rollout_net.py:81)
- [Per-world ordering optimization](/Users/jason/Documents/Codex/2026-10-04/task-2/research-worktree/experiments/astra-sol-20261004/phase2/incoming/drive/exact_tape.py:86)
- [Finite replay horizon](/Users/jason/Documents/Codex/2026-10-04/task-2/research-worktree/experiments/astra-sol-20261004/phase2/math/sol/FirstDivergence.lean:25)
- [Abstract compiled store](/Users/jason/Documents/Codex/2026-10-04/task-2/research-worktree/experiments/astra-sol-20261004/phase3/math/sol/CompiledDAG.lean:34)
- [Worker shared MAX](/Users/jason/Documents/Codex/2026-10-04/task-2/research-worktree/experiments/adversarial-20261004/parallel_roots.py:93)
- [Default inner belief](/Users/jason/Documents/Codex/2026-10-04/task-2/research-worktree/walt/walt/src/solver/inner_belief.rs:14)
- [Native policy miss counter](/Users/jason/Documents/Codex/2026-10-04/task-2/research-worktree/walt/walt/src/solver/mod.rs:1251)

## Reproduction and remaining limits

From the isolated research-worktree root, use fresh receipt output directories:

```sh
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 30 --output-dir /tmp/sol-bidder-direct-NEW -- /Users/jason/code/mk5-main/.venv/bin/python experiments/adversarial-20261004/sol/verify_bidder_fixture.py
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 30 --output-dir /tmp/sol-teacher-split-NEW -- /Users/jason/code/mk5-main/.venv/bin/python experiments/adversarial-20261004/sol/teacher_audit.py
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 30 --output-dir /tmp/sol-pruning-NEW -- lean +leanprover/lean4:v4.33.0-rc1 experiments/adversarial-20261004/sol/SettledPruning.lean
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 30 --output-dir /tmp/sol-priority-NEW -- lean +leanprover/lean4:v4.33.0-rc1 experiments/adversarial-20261004/sol/PriorityRealization.lean
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 30 --output-dir /tmp/sol-orderings-NEW -- python3 experiments/adversarial-20261004/sol/ordering_trace_audit.py
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 30 --output-dir /tmp/sol-support-NEW -- /Users/jason/code/mk5-main/.venv/bin/python experiments/adversarial-20261004/sol/support_enumeration_audit.py
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 30 --output-dir /tmp/sol-repair-NEW -- /Users/jason/code/mk5-main/.venv/bin/python experiments/adversarial-20261004/sol/repair_audit.py
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 30 --output-dir /tmp/sol-refinement-NEW -- /Users/jason/code/mk5-main/.venv/bin/python experiments/adversarial-20261004/sol/refinement_audit.py
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 30 --output-dir /tmp/sol-frontier-stress-NEW -- /Users/jason/code/mk5-main/.venv/bin/python experiments/adversarial-20261004/sol/frontier_stress.py
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 30 --output-dir /tmp/sol-final-packet-NEW -- /Users/jason/code/mk5-main/.venv/bin/python experiments/adversarial-20261004/sol/final_packet_audit.py
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 30 --output-dir /tmp/sol-receipts-NEW -- python3 experiments/adversarial-20261004/sol/receipt_audit.py
```

The exact original six Lean commands are in their `run.json` files; replace
only the output directory to rerun. Direct archived executions used an existing
Python 3.12.13 / NumPy 2.4.4 environment. System Python lacked NumPy; the first
failed attempt is retained rather than hidden. No installation was needed.
No remaining execution blocker or background work. The stochastic nofusion
Texas42 reversal, meaningful held-out neural strength, and general higher-k
acceleration remain unestablished; this audit does not label them contradicted.
The fresh original proofs total 30 declarations, with nine added in this
directory (three settled/partition, four priority, and two nested-plan lemmas).
