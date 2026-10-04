# Same late policy through native recursion and portable WASM

2026-10-04. Exploratory executable evidence; conditional Lean facts are labeled
separately. Inherited checkpoint `47db3c50ba8af31544e7de05bb2745b24f0abff5`
and every predecessor file remain unchanged. Read the fresh
[independent review](checks/REVIEW.md) and [reproduction instructions](README.md).

1. **Accepted practical result: the existing recursive backend is the useful
   route for this named late policy.** The local [adapter](adapter/src/lib.rs)
   evaluates exactly the inherited uniform40 outer worlds / Voidless inner
   beliefs / budgets `[4,2,2,2]` / `Field::Level(2)` / ascending-tile ties. It
   supports native execution and a WASM ABI with the production host clock and
   legal checkpoint convention. Early, forced, settled and refused positions
   retain the pinned production player. No production source or deployment is
   changed. This repairs the next-step practical gap in inherited report item9;
   it is an adapter for an existing solver, not a new faster recursive kernel.

   The complete experiment has a **fixed late40 policy**. The complete-call
   `worlds`/`partner` fields select the phone fallback profile; they do not
   parameterize the experimental late policy. Measured calls use precisely
   opening160/20s/partner-off and ordinary40/14s/partner-on. No arbitrary caller
   sampling-profile preservation or new auction/think-deeper policy is claimed.
   `backend:"phone"` explicitly disables the experimental late route.

2. **Accepted pinned provenance and finite policy conformance.** The independent
   [identity receipt](checks/identity/run.json) checks all202 production source
   inputs against Rust `cb1ef3b23072e4c268f31f625f2b61d5facc1929`, source SHA256
   `5bedf14af0e5a5ef354acca4765f67843469d074d163e8e4786631aa23436e32`, and exact
   Plunge `a0d9fa806166b0e63fe016bb49d93f91f47b1af8` saved Git blobs. The phone
   WASM is `40d7a1eea658627f7bab37d3c1888ad3bf00216fb23a5d2ceeb7d2d1f4219119`.
   This is local source/blob equality, not a live deployment query.

   A separately written checker selects36 fresh eligible public roots over all
   nine declarations and all four seats. It independently reconstructs ordered
   outer draws, rejection consumption, duplicate mass, final RNG,108 modeled
   rung0–2 preparations and27 repeated actor choices. Each
   [native](checks/native-adapter-stable/run.json) and
   [WASM](checks/wasm-adapter-stable/run.json) audit matches every complete
   vector/choice and independently full-replays1,440 sampled deals;16 vectors
   contain ties. Complete actor context/cache identities remain authoritative;
   no hidden referee hands or source-deal seeds enter chooser requests.

   The separate production-policy
   [phone conformance](results/phone-conformance/summary.json) matches42 requests
   and84 comparisons across native phone routing, rebuilt linked WASM and the
   exact pinned blob. Every result field except `elapsed_us`, `solver_us` and
   `over_budget` must agree. This compares the **correct L1/partner reference**
   for that route; it never treats experimental L3 as equivalent to default L1.
   The [reviewer audit](checks/phone-records/run.json) independently rereads it.

3. **Accepted local WASM viability; qualified production readiness.** Both builds
   run without dependency installation or browser access. The
   [full-wire audit](checks/wire-stable/run.json) matches36 complete native/WASM
   hybrid calls,72 legal checkpoints, invalid profiles/private payload refusals,
   zero-budget late refusals and controlled monotonic-clock exhaustion. A caller
   that exhausts its budget keeps its legal checkpoint; no additional minimum
   phone solve is launched. Refusal supplies no partially evaluated vector.

   The checker found and the builder repaired a fixed-solver-budget bypass and
   duplicate node-counter flush before the accepted builds. Development receipts
   remain retained and distinguished from stable evidence. The installed host
   clock fixes the recoverable WASM scope issue: the inherited frontier wrapper
   relies on `std::time::Instant`, while this bridge uses production
   `walt::clock::Instant`. No production frontend is installed or changed.

   WASM size is6,506,127B versus the pinned blob6,331,412B, an additional174,715B
   (about2.76%). The linked unchanged phone ABI coexists with experimental
   `native_late_*` exports; [the host](wasm_worker.mjs) explicitly selects the
   latter. Native binary SHA256 is
   `51259e4088de71f813ab1c8ad0b81345129ea01c26dbefec60cba462c1878de7`, WASM
   `11e2f2ca71b14220d8cf76ac9b21e63450257ee1ec5b86cc765f66d250963f3d`.
   Browser interruption/allocation behavior, physical-phone timings and an
   intentional product policy choice remain unmeasured production obligations.

4. **Accepted larger independent-deal balanced panel; qualified strength.** The
   [prespecified plan](plan.json) chooses18 new source deals, stratifies all nine
   declarations twice, rotates every deal through four seats and swaps both
   partnerships at fixed bid30. Each backend completes144 games/4,032 moves,
   with276 completed late decisions and zero refusals/host interruptions.
   Native and WASM use the same deals: they are **18 source deals, not36**.
   Every recorded move/outcome and complete late vector agrees across backends.
   Each independent [native](checks/games-native/run.json) and
   [WASM](checks/games-wasm/run.json) audit reconstructs all own/public requests,
   independently implements the game mechanics, and freshly revalidates all276
   distinct late vectors with production sampler/RNG checking.

   [Native summary](results/native-games/summary.json) and
   [WASM summary](results/wasm-games/summary.json) each give **0 paired wins,
   2 losses,70 ties**, mean make advantage **−1/36 = −2.78 percentage points**.
   Source1505032 has the two losing rotations; the other17 source-deal mean
   differences are zero. Average four rotations within each source deal before
   computing the conservative bounded95% interval:
   **[−0.667992972, +0.612437416]**. It is conditional on independent PRNG source
   deals and this fixed declaration/profile panel. Rotations are correlated;
   deterministic pseudorandom sampling is not a proof of population independence.
   This is practical play/conformance evidence, with **no strength gain or policy
   equivalence conclusion**. Unlike the inherited one-deal smoke, the independent
   deal interval is informative enough to exclude the full extremes, but remains
   far too broad to establish a small play-strength difference.

5. **Qualified same-policy cold latency gain, with full relevant costs.** The
   [single final panel](results/cold-same-policy/summary.json) uses the first late
   public turn from each of12 eligible actual-game source deals. Six repetitions
   rotate six variants through all six process positions. Every432 complete
   vector matches; there are zero refused timing observations. One solver worker,
   the same outer40 stream, inner budgets/belief/ties and2s solver allowance are
   retained. Frontier per-worker caps are500k rows/20m charged work/50k queries/
   50k cache entries; native has its own cache/work/deadline instrument. These are
   not equal logical memory budgets. Node/V8 runtime compilation can use internal
   threads; no homogeneous cross-runtime hardware-worker claim is made.

   | Backend | Median complete cold12-turn batch | Median cold request | Median PID RSS |
   | --- | ---: | ---: | ---: |
   | Resumable | 310.439ms | 27.905ms | 6,529,024B |
   | Arena | 356.267ms | 30.564ms | 5,767,168B |
   | Prior lazy | 272.964ms | 23.109ms | 4,972,544B |
   | Inherited native control | 57.729ms | 4.940ms | 3,932,160B |
   | Direct native adapter | **52.622ms** | **4.339ms** | **3,284,992B** |
   | Cold Node/WASM adapter | 692.507ms | 58.054ms | 86,949,888B |

   Native is about**5.90×** faster than resumable on this complete cold panel.
   Resumable retains its arena improvement and remains slower than prior lazy.
   Direct native versus inherited native is a small adapter/packaging difference,
   not a new algorithm result. Process/JSON/tempfile/parse teardown, strict status
   and full-history setup, every outer sampling attempt and solve are charged.
   Cold Node additionally charges module compilation and instantiation. The
   already-running common Python driver's import/startup is not charged per
   request; reading/choosing saved inputs costs8.343ms separately. All batch
   startup is charged by its [11.007s capped receipt](results/cold-same-policy-log/run.json).
   This is a finite actual-turn panel, not a population latency confidence
   interval, physical phone benchmark, general higher-k gain or whole-Walt gain.
   Child CPU and PID RSS remain in raw records; incomplete bound-array counters
   are not substituted for process memory. Cold WASM linear memory peaks7,012,352B.

6. **Qualified complete-player costs: portable and fast late turns, no whole
   player win.** In the retained-worker game panel, native late276 turns total
   539.113ms with median0.409ms; WASM late276 total717.627ms with median1.469ms.
   WASM instances are fresh per decision and compiled modules reused. These
   include actual public reconstruction, sampling and host work; a fresh Node
   process per cold timing request is a different operating condition.

   Across2,016 candidate turns, native total47,116.916ms versus pinned-phone
   2,016 turns46,842.253ms. WASM candidate47,063.170ms versus phone46,721.140ms.
   Different policies and resulting histories prevent a causal kernel attribution;
   there is no aggregate player speedup. Both partnerships receive total28,656s
   scheduled compute allowance, with identical frozen profiles. The measured
   panel's complete capped batch walls charge Python startup, deal setup, worker
   process work, replay, serialization/receipt writing and cleanup: native
   **96.911s**, WASM **96.660s** across18 separately bounded eight-game batches.

   Native campaign388 worker processes consume104.754s child CPU; WASM288
   processes consume105.623s. Maximum individual worker RSS is169,639,936B and
   178,388,992B respectively, dominated by Node. These are per-PID maxima, not
   simultaneous whole-campaign memory totals. Candidate WASM linear memory peaks
   37,355,520B versus phone36,175,872B. Full process rusage and per-turn costs are
   retained; parent/referee CPU is included in elapsed wall but not added to
   waited-child CPU. Build costs are separate capped receipts: initial native
   28.016s, initial WASM20.240s, final warm rebuilds3.682s/1.210s, inherited
   control7.981s. Compiler caches differ; no equal cold-build comparison is claimed.

7. **Accepted conditional Lean facts; qualified implementation refinement.**
   Fresh [PolicyEquivalence.lean](checks/PolicyEquivalence.lean) proves finite
   Bellman equality when complete ordered successors/terminal values/folds agree,
   ordered duplicate-mass preservation and lossless full-context cache decoding.
   [Kernel receipt](checks/lean-policy/run.json) prints actual axioms; no `sorry`,
   `native_decide` or external PASS axiom appears. Three inherited Lean files are
   freshly rechecked too. These conditional laws do not prove their Rust/RNG/
   deadline/information-view premises, complexity or arbitrary shared-plan
   equivalence. Finite parity and proof obligations remain separate evidence.

8. **Qualified constructive next step: use this portable native baseline before
   adding parallel or higher-k machinery.** The adapter is locally viable and
   the same-policy native route is substantially cheaper than frontier machinery.
   It does not establish that the named L3 policy is a better product policy.
   A useful next bounded experiment is a fixed held-out policy/budget ladder
   through this backend, including the phone's actual L1/partner reference and
   deliberate higher-rung inner budgets, then independent-deal intervals. Keep
   latency/RSS/refusal and completed-vector parity alongside play results.

   Embarrassingly parallel **whole positions/complete actor queries** can reuse
   existing across-query native batching. Compare one/four solver workers at the
   same total budget and charge setup/sampling/cache/serialization/aggregate RSS;
   this checkpoint claims no measured scaling result. Shared lower actors must
   be keyed by complete immutable policy context plus actor information state;
   neither hidden world ID nor referee deal may redefine that view. Schemes or
   tapes may schedule/evaluate already frozen policy expressions, and bulk lookup
   may reuse exact identical actor queries, provided ordered sample mass, seeds,
   budgets, public branches and ties stay unchanged. The conditional Lean laws
   identify those premises; proving a generic higher-k speed or arbitrary shared
   plan theorem remains open. Historical Dice-component gains are not contradicted
   by this complete-policy measurement.

9. **Accepted preservation and execution boundaries; no new sourced
   contradiction.** The independent history audit checks predecessor manifests;
   new source, raw records, failed developmental receipts and final hashes are
   retained separately. Git objects were repacked into this checkout and the
   temporary shared-object link removed; [connectivity audit](results/git-independent/run.json)
   passes without any alternates. The original task7 checkout/branch/history are
   untouched. Every individual build/Lean/test/timing/match batch has allowance
   at most295s and elapsed below300s. The final benchmark is sequential while
   checker compute is paused. No jobs remain running at delivery.

   The [final retained audit](results/final-audit-fixed/run.json) additionally
   matches every non-clock full-game response, including native/WASM work counts,
   and rereads all current caps/identities. Its first attempt mistakenly included
   late `status_us`, `outer_sample_us` and `solve_us` timer diagnostics in semantic
   equality. That [failed receipt](results/final-audit/run.json) is preserved;
   the corrected exclusions remove clocks only, with complete values, requests,
   samples, routes and work counts still checked. It is a checker repair, not a
   policy discrepancy. The two other failed reviewer development checks are
   likewise excluded from accepted evidence.

   No original report claims higher-k linearity, phone strength or general
   resumable superiority; none is manufactured as a refutation here.
   [CONTRADICTED.md](CONTRADICTED.md) keeps this distinction. No merge/deploy,
   browser interaction, paid resource, external message, credentials/persistent
   access or fixture transmission occurred. The specific Claude disclosure
   remains blocked pending explicit approval; local use supplies no indirect route.
