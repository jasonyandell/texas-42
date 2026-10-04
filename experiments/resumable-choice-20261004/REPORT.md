# Resumable exact choices and a complete local player adapter

2026-10-04, **exploratory finite engineering evidence**. Isolated checkpoint
`97c1c1759f2966e8b985d66b19d7cc4ddc08be42` →
`codex/walt-resumable-20261004`. [Reproduce](README.md),
[independent numbered review](checks/REVIEW.md), [team](TEAM.json),
[accepted history](ACCEPTED.md), [contradictions](CONTRADICTED.md).

Retained paths and incremental folds improve the previous arena on every
completed service cell, preserving its exact field2 demand. They offer small
and inconsistent gains over the prior lazy evaluator and remain far slower
than native recursion. A complete local hybrid player now runs the verified
kernel on late turns and the pinned phone player elsewhere. Eight balanced
smoke games complete; playing strength and phone-device latency remain unknown.

## Numbered findings

1. **Accepted — fresh team, isolation, preserved history and pinned identity.**
   The explicitly requested implementation and independent review agents each
   accepted `gpt-6.1-sol`; separate runtime attestation is unavailable.
   A separate bare `--no-hardlinks` local clone and worktree inside task-7 leave
   task-6 originals and Git administration untouched. Repository `CLAUDE.md`,
   `QUICKSTART.md` and instrument inventory were read; no applicable `.agents`
   was found. No wiki/claim-tier or production source is changed.

   The reviewer checks all **1,414 historical manifest entries**, production
   Plunge `a0d9fa806166b0e63fe016bb49d93f91f47b1af8`, Rust
   `cb1ef3b23072e4c268f31f625f2b61d5facc1929`, 202 source inputs and WASM
   SHA256 `40d7a1eea658627f7bab37d3c1888ad3bf00216fb23a5d2ceeb7d2d1f4219119`.
   Prior 6,180-vector/30-refusal arena arithmetic and exhaustive support
   compilation reproduce. Old arena/demand binaries are freshly built and
   actually hashed; relocation can change binary bytes. Historical ignored
   executables are not silently claimed current.

2. **Accepted finite mechanism — retain the path and update touched ancestors.**
   [resumable.rs](native/src/resumable.rs) adds `with_resumable_choices()` and
   JSON `resumable_choices:true`, preserving the arena comparator. Each tree
   owns ordered nodes/world indices/edges and a fixed 17-ID descent stack.
   Nature updates subtract the previous child interval and add the new one.
   MAX lower and MIN upper extrema update monotonically; the opposite bound
   rescans only when the touched child tied the old extremum. Unchanged parent
   bounds stop ancestor propagation; active-child selection still updates when
   needed. Root certification retains strict earlier/weak later inequalities
   and ascending integer tile ties. Every focal child receives its **whole
   ordered information-set group**; nature partitions only after complete lower
   actor replies. Context/State/actor/own hand/rung remains the complete key.

   Completed choices are cached immediately, then owned tree arrays are dropped
   and charged rows/bytes released. There is no capacity recycling pool. A later
   refusal can retain earlier completed top choices, unlike arena's whole-forest
   success insertion. Prepared/unresolved choices stay uncached; accounting
   restores saved live totals on success/error. This is a stated refusal/cache
   semantic difference, not transactional equivalence.

3. **Accepted finite parity; conditional mathematics.** Eleven owner release
   tests pass. Fresh independent checks retain 22,220 interval boxes, both-team
   roots, real ties, complete vectors, seed permutations/multiplicity, every
   demanded-call comparison, key distinctions, mixed forests and refusal checks.
   Eight-world trace has 11,904 replies/11,666 unique specs; forty-world Voidless
   41,677/39,847 and counted 38,149/35,653 replies/specs match independent full
   eager/native. New 228 mixed actor views, choice-cache/full-vector separation,
   interleaving, row/work/cache-cap and completed-cache survival checks pass.
   A further failed-forest witness independently retrieves10/31 completed top
   choices after work-cap1000/2000 refusal, all equal to full eager.
   The optional existing eager-L0 floor is separately checked.

   [Seven new Lean laws](math/ResumableChoice.lean) kernel-check, alongside six
   arena/seven bounds laws. These prove conditional arithmetic, unchanged-fold
   substitution and saved-choice independence from tree ownership, under their
   stated premises. They do not prove Rust scheduling/overflow/refusal/RNG,
   information privacy, sampler correctness, cache completeness or complexity.
   Printed standard axioms and fresh capped receipts are retained in `checks/`.

4. **Qualified profile — less repeated traversal, unchanged field2 demand.**
   The pre-change profile, failed setup and native-field pilot are preserved
   separately from the corrected modeled-field diagnostic in
   [native notes](native/NOTES.md). In the matched final four-worker six-root
   forty-world field2 panel, resumable/arena/lazy each expands 541,034 nodes,
   creates 743,656 children, charges 1,027,249 row edges and misses exactly
   39,847 actor specs `[28689,9376,1782]`. Native's internal cache/call counters
   are a different instrument and are not substituted for those misses.

   Resumable performs 554,645 actual new stack pushes versus 1,785,943 implied
   full-path refinement visits, 977,749 ancestor updates and 415,580 unchanged
   stops. It releases 39,847 completed trees. The retained `bounded_refinements`
   counter deliberately measures the old implied path length; actual work is
   in `res_stack_steps`. Nested phase intervals overlap; microsecond sample
   counters truncate short calls and are not causal CPU fractions.

5. **Qualified matched service improvement — native remains strongest.**
   Six sequential cold repetitions rotate six variants through each process
   position, with identical samples, `[4,2,2,2]`, belief, worker count and frontier
   allowances per worker: 500k retained rows, 20m row work, 50k queries/cache,
   30s. Four workers have four separate allowances. Native has its own
   node/cache accounting and deadline, not equivalent frontier logical caps.
   All 6,180 completed comparison vectors match; 30 refused observations emit
   no root vector and enter no speed ratio.

   | Panel / field | Workers | Worlds | Resumable | Arena | Prior lazy | Native |
   | --- | ---: | ---: | ---: | ---: | ---: | ---: |
   | Six roots / 2 | 1 | 8 | 57.727ms | 71.281ms | 48.744ms | 3.869ms |
   | Six roots / 2 | 1 | 40 | 181.754ms | 215.925ms | 160.572ms | 13.569ms |
   | Six roots / 2 | 4 | 8 | 30.164ms | 37.730ms | 46.348ms | 1.667ms |
   | Six roots / 2 | 4 | 40 | 97.314ms | 110.275ms | 98.251ms | 6.519ms |
   | Larger31 / 2 | 4 | 8 | 120.122ms | 128.642ms | 102.285ms | 7.019ms |
   | Larger31 / 1 | 4 | 40 | 121.146ms | 136.862ms | 136.523ms | 9.799ms |

   Resumable is faster than arena in all 21 completed cells and faster than
   lazy in 14. One-worker field2 cells and larger eight-world field2 remain
   slower. On larger forty-world field2, resumable/arena/lazy/full all refuse
   50k queries; hybrid/native complete. No higher-k affordability follows.
   [One worker](results/final-one/summary.json),
   [four](results/final-four/summary.json),
   [larger](results/final-holdout/summary.json).

   The controlled [eager-L0 ablation](results/ablation/summary.json), eight
   four-position rotations, gives forty-world res98.696ms, res+floor86.076ms,
   lazy93.443ms, native6.502ms, with unchanged completed field2 unique misses.
   Its eight-world lazy32.246ms differs materially from the six-way panel's
   46.348ms. These repeated local medians support a small implementation gain
   on particular panels, not a robust general lazy win or confidence interval.

6. **Qualified complete costs and memory.** On the four-worker six-root
   forty-world field2 panel, resumable/arena/lazy/native child CPU is
   261.934/294.650/268.419/20.449ms; PID RSS medians are
   24,412,160/21,012,480/17,752,064/7,118,848 bytes. Resumable's counted bound
   peak falls to 1,720,516B from arena2,511,324B, but remains above lazy77,572B.
   Those are summed worker maxima and incomplete counts: fixed paths, prepared
   queries/specs, spare capacities, temporary maps/groups, cache ownership and
   allocator overhead are excluded. Actual RSS worsens against both arena and
   lazy here; array release does not establish process memory reduction.

   Standalone JSON/process/replay/service/tempfile/parse medians are
   101.597/114.906/102.571/9.878ms. Charging all32 original forty-world fixture
   proposals adds41.397ms, yielding142.994/156.303/143.968/51.275ms. Separately,
   the [fully cold frontend](results/cold-primary/summary.json) starts fresh
   Python and native on every request with the same compiled support frontend,
   setup, all proposals, filtering, JSON, replay, scheduling, hashing/printing,
   teardown and wrapper parse. Resumable168.292ms, arena179.155ms,
   lazy172.983ms, native76.021ms: local1.065x over arena/1.028x over lazy,
   still2.214x slower than native. Eight four-position rotations match every
   world/vector. Larger forty-world frontier calls refuse; native130.355ms
   completes, so refusal elapsed is not a speed result.

   Compiler costs are separately charged in capped build receipts: fresh old
   arena/demand, candidate, checker and adapter builds are retained, including
   warm incremental final rebuilds. They are not amortized into an invented
   per-turn figure or presented as equal cold compiler caches. Common Python
   harness import/AST work is included in both cold modes; this does not claim
   minimum historical startup. Frontend wait4 CPU may include waited descendants
   and must not be added to nested native CPU. The prior exact-stream support
   gains remain valid local fixture evidence, not a new phone sampler change.

7. **Accepted practical local integration — a complete named hybrid now exists.**
   [player.py](player.py), [native adapter](integration/src/main.rs) and
   [arena.py](arena.py) consume exactly the acting seat's seven own/public
   request fields. The shared strict production status validator checks turns,
   ownership, own historical legality, void consistency and hidden capacity
   feasibility. Eligibility is16–24 exposed plies, unsettled Straight, at least
   two legal moves. Native outer sampling preserves production's mixed seed,
   complete uniform shuffle-and-reject stream, accepted order/multiplicity and
   final RNG on completed calls; bounded sampling/solver refusals trigger the
   phone route. It already uses public masks, so the earlier Python fixture
   optimization is not newly imposed on production sampling.

   Eligible calls solve the complete exact vector for **40 outer worlds,
   Voidless lower beliefs, budgets `[4,2,2,2]`, `Field::Level(2)`, ascending ties**
   using resumable choices. This is an experimental late L3 policy, distinct
   from the pinned phone's default L1/partner review. Early/forced/settled/
   unsupported late/failed calls use the exact pinned phone player with its
   opening160/20s and ordinary40/14s profile. Native failure time consumes the
   original per-turn compute allowance; remaining phone work keeps its completed
   legal checkpoint. Transport/JSON/schema/illegal-result/time failures close
   the kernel process before fallback. No hidden referee hands enter chooser
   payloads. This is a complete local play-only Straight bid30 adapter, not a
   deployed WASM replacement or a new auction policy.

   Independent integration checks cover36 fresh public roots over all nine
   declarations/all four seats:144 complete res/arena/lazy/native vectors,
   1,440 independently full-replayed worlds,29 malformed/private requests and
   early/forced/settled/kernel/transport fallback mocks. Actual full games are
   additionally replayed independently below. This repairs the prior concrete
   absence of an adapter while preserving that historical qualification.

8. **Qualified same-policy adapter latency and balanced phone smoke.** Four
   saved eligible pilot turns, eight four-position rotated cold native calls,
   compare96 complete vectors with identical public requests/production outer
   stream and the same L3 policy. Total complete cold costs for those four calls
   average65.499ms res,76.718ms arena,60.804ms lazy,15.866ms native. Status replay,
   sampling, JSON, process startup/teardown and parse are charged; one worker and
   two-second solver allowance each. Native's logical work/cache accounting
   differs. This actual adapter domain retains the arena gain but loses to lazy
   and native; [raw summary](results/adapter-cold/summary.json).

   The fixed bid30/declaration6 fresh deal975102 is rotated through all four
   seats and both candidate partnerships: **eight complete games/224 moves**,
   26 completed late-kernel decisions, zero late refusals, four paired make/set
   ties. Candidate112 turns total3408.268ms versus phone112 turns3085.244ms;
   candidate late26 turns408.492ms. These include real own/public reconstruction,
   native/Node/WASM startup, sampling, wrapper work and fallbacks. Different
   policies and resulting histories prevent attributing aggregate latency to
   the kernel alone. The phone is pinned Node-hosted WASM on this Mac, not a
   physical phone timing test. [Balanced block](results/h2h/summary.json).

   One independent source deal is a smoke test; its rotations are correlated.
   A conservative bounded95% independent-deal interval is the entire[-1,1].
   Zero measured mean paired make difference is not equivalence or stronger
   play. Actual binary/source/phone hashes accompany every game; the earlier
   pilot has a different build identity and remains explicitly excluded.

9. **Qualified next step and exact production blockers.** A local complete
   player adapter is feasible and now checked. Production integration would
   require strict WASM/public wire exposure of this kernel, browser clock and
   checkpoint mapping, measured allocation/query/deadline fallbacks, full
   native/WASM vector conformance and a deliberate late-policy/budget choice.
   The experimental late policy is different, and the new kernel does not beat
   native recursion. The minimal useful next step is to route the same named
   late policy through the existing native recursion backend (already supported
   as an adapter control), then test new independent-deal balanced blocks and
   phone/WASM viability before choosing a production policy. No merge/deploy is
   authorized or done.

   General higher-k linearity, broad parallelism and arbitrary shared-plan
   equivalence are unproved. Existing Schemes/tapes/precomputed coordinates,
   bulk positions and lookup aggregation remain constructive candidates under
   full information-set and belief/cache identity obligations. This result
   neither assumes that intuition a theorem nor refutes the historical
   large-Dice-component gains. No new actual sourced claim is contradicted.

10. **Accepted execution/provenance boundaries.** Every individual build,
    test, Lean and benchmark has the inherited process-group watchdog,
    allowance≤295s and elapsed<300s. Failed profile/checker assumptions remain
    retained and excluded from green evidence. All final timing runs are
    sequential with builder/reviewer compute paused. Source hashes, manifests,
    [final audit](results/final-audit/stdout.log) and Git pin the deliverable;
    no jobs remain pending. No external transmission, paid resources,
    credentials/persistent-access changes, production merge/deploy or browser
    interaction occurred. The specific blocked Claude fixture export remains
    blocked; local use never routes it externally.
