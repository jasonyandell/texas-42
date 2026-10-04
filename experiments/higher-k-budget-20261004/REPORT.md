# Higher best-response k under fixed budgets

2026-10-04, **exploratory finite local evidence**. Source checkpoint
`56794dc321f3546223ff1359ed30b87911130fd9`, isolated branch
`codex/walt-higher-k-budget-20261004`. [Reproduce](README.md),
[independent Sol review](checks/REVIEW.md), [raw summary](results/summary.json),
[cost/scaling/action analysis](results/analysis.json).

The useful result is a horizon-dependent budget frontier and a measured route
for parallel offline queries. With three tricks left, k3 completed every eligible
20 ms call; k5 sometimes refused. With four tricks left, k3 needed about30 ms and
k5 usually failed even200 ms. Four whole-query workers cut complete batch wall
by3.45×, at higher CPU and memory cost. Extra k changed actions and sometimes
helped one suffix deal, but this panel establishes no playing-strength gain.

1. **Accepted isolation, source and inherited history; qualified policy scope.**
   The original task8 checkout and all predecessor experiments remain unchanged.
   This local clone has independent Git objects and no alternates. Repository
   `CLAUDE.md`, `QUICKSTART.md`, instrument inventory and relevant prior Lean
   sources were read; no applicable `AGENTS.md` or `.agents` was found. No wiki
   tier, production crate or deployment is changed. The reviewer freshly checks
   [2,443 historical manifest entries](checks/inherited-audit/run.json), the202
   Rust source inputs pinned to`cb1ef3b23072e4c268f31f625f2b61d5facc1929`, and the
   [production phone pin](checks/identity/run.json): Plunge
   `a0d9fa806166b0e63fe016bb49d93f91f47b1af8`, WASM SHA256
   `40d7a1eea658627f7bab37d3c1888ad3bf00216fb23a5d2ceeb7d2d1f4219119`.
   This is local provenance, not a live deployment query.

   [The new adapter](adapter/src/lib.rs) uses the unchanged recursive kernel.
   Chooser rung k=1..5 is the best response against`Field::Level(k-1)`. Every
   rung has the same40 ordered outer draws, original mixed seed/rejection stream,
   duplicate mass, Voidless modeled belief, fixed ascending-tile ties, and inner
   budget vector`[4,2,2,2,2,2]`. L0 is a response to Dice. The first four entries
   reproduce the inherited k3 policy; [nine fresh comparisons](checks/inherited-k3-conformance/run.json)
   match the old four-entry adapter's complete values/worlds/final RNG/attempts.
   k1 here uses n0=4; production L1 uses n0=8 plus partner review. Phone comparisons
   therefore compare policy families, not an isolated k effect. The phone control
   has40worlds/partner-on at an experimental200 ms, rather than production14 s.

   Choosers receive exactly seven own/public fields, the actor's full original hand and public
   records; hidden referee hands/source-deal seeds never enter them. Outer draws
   obey public void constraints. Inherited Voidless lower beliefs deliberately
   forget void deductions; they are not exact perfect-recall behavioral beliefs
   or a claim that every modeled inner world reproduces full historical legality.
   No actor group is split into per-hidden-world best responses. Refusals retain
   ascending legal checkpoints and contain no partial evaluation vector.

2. **Accepted nominal-budget completion frontier; qualified total wall.**
   [The original plan](plan.json) was hashed before results. It includes54fresh
   source deals, all9declarations six times, four seat rotations and both roles.
   Uniform legal random prefixes are generated independently of tested k. The
  16-ply cost panel has216planned roots,96eligible from24source deals. The
   [explicit adaptive extension](earlier-plan.json) tests ALL the same54deals
   one trick earlier after seeing that eligibility count; it adds no independent
   deals. Its216roots have140eligible from35source deals. No outcome-dependent
   eligibility filtering or pooling of horizons occurs.

   | Remaining tricks | Nominal allowance | Eligible calls completed: k1 / k2 / k3 / k4 / k5 |
   | --- | --- | --- |
   | 3 | 20 ms | 96 / 96 / 96 / 96 / **80** of96 |
   | 3 | 200 ms | 96 / 96 / 96 / 96 /96 of96 |
   | 4 | 20 ms | 140 / 140 / **23 /0 /0** of140 |
   | 4 | 200 ms | 140 / 140 /140 / **133 /16** of140 |

   At200 ms/three tricks, eligible median complete host costs for k1..5 are
  0.413/1.061/2.813/6.329/10.253 ms. At200 ms/four tricks, k3 completes with
   median29.852 ms; k4 and k5 completed-only medians are selected subsets and must
   not be read as unconditional cost. Sampling, attempts, nodes, per-level modeled
   computations/worlds, cache entries, refusals and all host/wrapper clocks are
   retained for every call in [both root panels](results/analysis.json).
   [Deal-level refusal intervals](results/budget-intervals.json) additionally use
   ALL54planned clusters (ineligible requests have zero kernel-refusal indicator).
   For k5 at200 ms/four tricks, unconditional planned-root refusal124/216=0.57407
   has pointwise95% interval[0.38926,0.75889]; this differs from124/140conditional
   eligible refusals. Shared horizons add no independence.

   These are nominal solver allowances including status/replay/outer sampling;
   transport/process setup/serialization and deadline polling can exceed them.
   The first cold k4/20 ms request took**267.066 ms host wall**, while its Rust
   wrapper reported13.944 ms. The253.122 ms difference is retained without a causal
   diagnosis; no strict20 ms end-to-end deadline is claimed. Smaller deadline
   overruns are also retained. Every experiment still has a separate≤295 s
   process-group watchdog and elapsed<300 s.

3. **Accepted empirical work growth; no complexity lower-bound theorem.**
   [Unconstrained anchors](results/unconstrained/) use the first9frozen source
   deals, four rotations at16plies and rotation0at12plies, k1..5 with30 s nominal
   allowance. There are225planned calls; only4eligible independent source roots
   at12plies and8eligible rotations from2sources at16plies. Every eligible
   call completes; excluded roots remain in raw data.

   For the SAME four eligible12-ply roots, median complete k1→k5 host wall grows
  1.548→438.908 ms, while median summed modeled computations grow1,199→1,421,657.
   Those are approximately283× and1,186× finite ratios of medians, not asymptotic
   rates or a universal exponential lower bound. Observed native counters include
   L0 recomputations because`bypass-l0-cache-all` is enabled; they are not unique
   actor-query counts. Peak native PID RSS is61,095,936 B in these anchors, and
   the largest completed policy cache has491,109entries. Cache-entry counts are
   not a complete memory estimate. Fixed-budget root panels peak at13,631,488 B
   (three tricks) and31,752,192 B (four tricks); refusal costs remain included.

4. **Accepted paired balanced suffix evidence; qualified strength.**
   All3,024games/36,288moves complete. Each of7cells has54independent source-deal
   clusters,4rotations and both partnership roles; all reuse the same54deals.
   Games begin after16random legal plies and end at28, bid30. The independent
   [data audit](checks/data-final-artifacts-audit/run.json) reconstructs fixtures, full suffix
   mechanics, every request/policy/move/outcome and interval arithmetic.

   | Candidate/reference | Allowance | Paired wins / losses / ties | Mean make advantage | Deal-level95% interval |
   | --- | --- | --- | --- | --- |
   | k3 / k1 | 20 ms | 2 /4 /210 | −0.00926 | [−0.37889,+0.36037] |
   | k5 / k1 | 20 ms | 0 /6 /210 | −0.02778 | [−0.39741,+0.34185] |
   | k3 / k1 | 200 ms | 2 /4 /210 | −0.00926 | [−0.37889,+0.36037] |
   | k5 / k1 | 200 ms | 0 /4 /212 | −0.01852 | [−0.38815,+0.35111] |
   | k1 / phone | 200 ms | 2 /2 /212 | 0 | [−0.36963,+0.36963] |
   | k3 / phone | 200 ms | 2 /4 /210 | −0.00926 | [−0.37889,+0.36037] |
   | k5 / phone | 200 ms | 0 /4 /212 | −0.01852 | [−0.38815,+0.35111] |

   Average the four rotation differences within each source before the conservative
   bounded Hoeffding radius`sqrt(2*log(40)/54)=0.3696284`. Intervals are pointwise,
   not a simultaneous7cell guarantee. Source PRNG draws are assumed independent;
   neither PRNG independence nor population representativeness is proved.
   This is a random-prefix suffix distribution, not whole-player/production-play
   strength. No gain, equivalence or reliable disadvantage is established.

   There are concrete useful contrasts: [source3289004](results/games/deal-3289004.json)
   gives k3 two paired wins over k1 (rotations2/3), while
   [source3976200](results/games/deal-3976200.json) gives four losses at both
   allowances. On [source3608860](results/games/deal-3608860.json), k5's extra
   two20 ms losses disappear at200 ms. k5/20 ms has16refusals among458candidate
   attempts; k5/200 ms completes454/454 on its resulting histories. k3 completes
  466/466 at both allowances. Changed histories make their turn costs and attempt
   denominators different; this does not isolate a causal kernel effect.
   All54suffix batches together take15.637 s capped wall. Root+suffix workers
   use12.144 s waited-child CPU across108PIDs, maximum PID RSS15,024,128 B.

5. **Accepted practical parallel batch gain; qualified affordability.**
   [Whole-query batching](experiment.py) freezes36requests at12plies from the
   first9deals:16eligible k5 roots from4source deals,20ineligible requests.
   One/four independent serial native solver processes receive the SAME30 s
   allowance per query, ordered inputs, samples, immutable contexts and fresh
   request caches. Six repetitions alternate the two execution positions. All
  432non-clock responses/counters match, including192completed vectors and
  240legal ineligible responses; no request refuses.

   | Workers | Median complete capped batch wall | Median waited-child CPU | Median sum of PID peak RSS |
   | --- | --- | --- | --- |
   | 1 | **7.11896 s** | 7.01510 s | 67,698,688 B |
   | 4 | **2.06637 s** | 7.90257 s | 254,713,856 B |

   Primary complete-wall gain is**3.445×**. Internal batch intervals exclude
   Python import/referee/input preparation/hash/receipt work and give3.521×;
   they are secondary. Full cap walls charge that work and process setup/sampling/
   cache/JSON/output/teardown. Child CPU rises**12.65%**. Sum of per-PID lifetime
   maxima is a conservative memory envelope, not simultaneous aggregate RSS.
   This improves local offline batch waiting time, not CPU economy, a single
   decision, recursive frontier parallelism, physical phones or browsers.
   [Independent scaling receipts](checks/scaling-final-artifacts-audit/run.json)
   verify the arithmetic and exact complete-response parity.

6. **Accepted finite native/WASM and phone conformance; conditional math only.**
   [Final native](checks/native-adapter-final/run.json) and
   [final WASM](checks/wasm-adapter-final/run.json) each match45independently prepared vectors
   on9fresh two-trick roots/all9declarations/k1..5,31tied vectors,1,800ordered
   outer worlds/full independent world replays, final RNG and native counters.
   Zero-budget refusals preserve legal checkpoints and no vector; malformed/private
   calls refuse. [Final deeper audit](checks/deeper-conformance-final/run.json) adds20reference
   vectors on the4eligible frozen12-ply roots and40native/WASM comparisons,
   plus controlled clock exhaustion. [Final phone control](checks/phone-conformance-final/run.json)
   matches9fresh200mscalls across new native bridge/new WASM/exact pinned phone
   blob:18non-clock comparisons and WASM checkpoint sequences agree. This is
   local Node/WASM evidence; no browser/device validation occurred.

   [ChangingOpponent.lean](checks/ChangingOpponent.lean) proves an axiom-free
   generic two-action counterexample: two exact responses to two different
   opponents can worsen payoff against one fixed reference. It is not a Texas42
   witness or an implementation theorem. Existing
   [Bellman/ordered-mass/cache laws](../native-policy-check-20261004/checks/PolicyEquivalence.lean)
   require unchanged full successors/folds/values; the prior
   [nested-policy monotonic theorem](../adversarial-20261004/sol/NestedPlans.lean)
   requires one fixed value map and policy-class inclusion. Changing modeled
   fields with k does not automatically meet those premises. Empirical growth
   is not promoted to a complexity theorem.

7. **Accepted constructive route and retained failures.**
   Use four complete-query workers for offline native workloads when waiting
   time matters and the larger memory/CPU envelope is acceptable. In this
   measured policy family, k3 is the practical exploratory frontier: it fits
  20 ms with three tricks remaining and200 ms with four. k5 is affordable on
   many three-trick roots and becomes costly one trick earlier. This is a
   measured domain/budget statement, not a recommended stronger player.

   Highest-value next player experiment: a frozen budget controller that retains
   a completed k1/k3 checkpoint BEFORE spending leftover time on k5, then evaluate
   fresh production-generated prefixes with balanced source-deal intervals. It
   replaces ascending-first-legal fallback with a completed shallower choice; its
   strength remains unmeasured. It remains
   an unmeasured new policy, not an established strength improvement. Preserve
   every rung's exact sampled policy, full actor groups, ties and seeds. Shared
   lower-actor work must use complete immutable context/view keys; this batch
   evidence gives no license to split information sets or resample/deduplicate.

   Failed current hypotheses are retained in [CONTRADICTED.md](CONTRADICTED.md):
   k5 does not fit200 ms at four tricks, deeper responses do not show a measured
   strength gain, and four workers do not reduce CPU/memory. No contradictory
   assertion is attributed to prior reports. A builder summary schema initially
   overwrote candidate/reference labels with cost dictionaries; [development
   summary](results/summary-development.json) and measured source snapshot remain,
   corrected output uses`candidate_cost/reference_cost`. A descriptive analysis
   had a missing game-wall aggregate; its development version is retained too.
   Reviewer also found a malformed-input discrepancy: raw k=4,294,967,297
   was rejected by native64 but wrapped to1 on WASM32. Final source checks raw
   u64 range BEFORE converting it; all measured1..5policy expressions are
   unchanged. Measured adapter/binaries and failed scalar checks remain retained,
   with [final scalar conformance](checks/scalar-boundaries-final/run.json) and
   all valid native/WASM/deeper vectors separately rerun. Reviewer first build failed its own led-context type and was repaired; failed
   receipts remain excluded from accepted evidence.

   Measured binaries remain preserved and pinned in [ARTIFACTS.json](ARTIFACTS.json);
   their SHA256 values are native`4a4899d77e6faa731e10ddff63f67bd0c485240d5c440623b3f11e18b700e779`
   and WASM`a50f865ae39a439f510f7d9f032c62f4e53ccac70244b4660b8f48103cd25245`.
   Final scalar-hardened binaries are native`2154b88b380822759a138937444c653be1b7644d138069eebaae4cf04cc12bd5`
   and WASM`fe04e6628166abb0917401393a3629d86125037edebd6d9e5f12d3bf7d460bab`
   (6,503,456 B). Build costs are separate receipts, not amortized timing claims.
   Fresh output routing was checked without overwriting gold evidence. A
   [clean local clone](results/cold-clone-log/run.json) restored all four pinned
   binaries from archives and passed the [full saved-data audit](results/cold-saved-data-audit/run.json). Source
   snapshots pin all measured harness versions; the final routing/summary-only
   changes do not change any solver, RNG or policy expression.

   No production merge/deploy, paid resources, credentials/persistent access,
   external messages, browser manipulation or fixture transmission occurred.
   The specific Claude disclosure remains blocked pending explicit approval;
   local use provides no indirect route. Device hosting and an intentional
   product policy choice remain unmeasured. All jobs are reaped at checkpoint.
