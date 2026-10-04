# Bounded native frontier and actor-query batch service

2026-10-04. Exploratory native implementation evidence. This is an isolated
continuation of checkpoint `490997b13b161099dcfbcf19f2dec0819fd5a019`.
[Earlier numbered verdicts](../adversarial-20261004/REPORT.md),
[accepted history](../adversarial-20261004/ACCEPTED.md), and
[contradicted-source history](../adversarial-20261004/CONTRADICTED.md) are preserved
unchanged. [Independent check](checks/REPORT.md) and [reproduction](README.md).

## Result

The [native prototype](native/src/lib.rs) preserves tested finite-bundle
semantics and queues independent actor-specific lower-rung queries. **This port
does not demonstrate a cold speed advantage over the pinned optimized native
solver.** Four-worker frontier throughput improves, but straightforward
four-worker native recursion is faster. Full lower-query vectors do substantial
extra work when their consumer needs only a choice. An exact choice-only hybrid
reduces that cost, without beating the equal-worker recursive control.

This is a useful implementation and a demonstrated obstacle to this particular
array layout and eager expansion schedule. It is not a negative theorem about
batching, Scheme specialization, higher rungs, or another proposed design.

## Numbered evidence

1. **Pinned production identity retained.** The source fingerprint over 202
   files and phone artifact match Plunge
   `a0d9fa806166b0e63fe016bb49d93f91f47b1af8`, Rust source `cb1ef3b2`, and WASM
   SHA256 `40d7a1eea658627f7bab37d3c1888ad3bf00216fb23a5d2ceeb7d2d1f4219119`.
   [Identity receipt](results/identity/stdout.log). No production file changed.
   This service is not a complete phone player: it lacks the phone wrapper,
   review/refinement schedule and turn adapter. Consequently no match or
   playing-strength claim is made. Identical decisions on tested component
   inputs do not establish stronger play.

2. **Native semantics, not a silent transfer of the pilot's tape.** Historical
   depth tapes use exact uint64 multiply-high selection. Native Dice instead
   resets `SplitMix64(seed[sid] ^ record_hash(state))` at each public state and
   uses the pinned rejection/modulo `below(n)` rule. Both are explicit modes.
   [Pinned Dice source](../../walt/walt/src/solver/mod.rs:955),
   [independent integration requirements](checks/INTEGRATION.md).
   Worlds retain order and unit multiplicity, including duplicate samples.
   One decision follows aggregation over the entire reachable information
   group; first ascending tile wins ties. Internal seats normalize the bidding
   team to T1. Frozen finite samples need not be seat-rotation invariant; each
   rotation must be compared with its own appropriately seeded reference.

3. **Outer support and inner beliefs have separate authorities.** Every
   supplied outer world is reconstructed to four original seven-tile hands
   and legally replayed through the complete prefix, without early contract
   settlement. Ownership, turn order, follow legality, conservation, partial
   tricks, banked scores and native replay are checked. Lower queries receive
   only actor hand plus represented public frame. They use the actual native
   `InnerBelief::sample`, frozen level/seat/hand/public-state seed, unchanged
   shuffle consumption and sample budgets `[4,2,2,2]`. Default Voidless inner
   worlds intentionally omit historical void conditioning; filtering them
   like outer support would change the modeled policy. VoidsCounted is
   separately supported. The repaired historical witness again yields all
   700 lawful worlds among 1,680 partitions and 384 legal samples.
   [Support receipt](checks/support-repair/stdout.log).

4. **Native integer arrays and selective specialization implemented.**
   Immutable legal incidence and trick-strength tables specialize supported
   straight declarations ahead of time. Forward layers instantiate only
   supplied scenario branches and all legal focal choices. Prefix IDs are
   collision-free equality classes of `(prior prefix ID, tile)` at each
   depth, rooted separately per query. Backward integer sums precede shared
   MAX/MIN. No universal all-world/all-tape tree is materialized. Declaration
   grouping reduces concurrent residency; direct integer record hashing
   avoids temporary play-vector allocations. The existing
   [Scheme compiler](../../walt/walt/src/scheme/mod.rs) and
   [finite dynamics](../../walt/walt/src/scheme/dynamics.rs) do not supply a
   higher-rung tree compiler, so this experiment specializes the native rule
   algebra rather than claiming one exists.

5. **Full equality and bounded refusal checked independently.** Raw result
   cache identity includes complete context/frame, actor/hand, ordered worlds,
   nature mode and every seed/tape. Deterministically generated actor cache
   identity includes declaration, contract, all sample budgets, exact boundary,
   belief mode, banked totals, partial trick, played set, actor/hand and rung.
   Fixed selection, native source and seed schema are immutable process scope;
   there is no disk cache. Deadlines are checked even on warm hits. Refusals
   return no root vector. Completed lower answers can remain reusable after
   a parent refusal; partial answers are never inserted. Native choice results
   have a separate cache from full value vectors. Guards cover malformed frames,
   empty counted fibers, input support, tapes and budgets.
   The independent collaborator checked 38 fresh scalar roots including 31
   forced rejection seeds, 76 serial/parallel native vectors, 38 tripled bundles,
   30 bid31 variants, 24 L0/L1 actor vectors, 48 hybrid policy vectors, and 344
   parallel-protocol vectors. Relevant native source tests and **17 Lean
   declarations** pass. Those Lean results are conditional abstract lemmas,
   not refinement or complexity proofs of this Rust program.
   [Independent receipts and source hashes](checks/REPORT.md).
   The final [receipt/source audit](AUDIT.json) rechecks2,750 completed vector
   comparisons across final panels, including repeated measurements; this is
   not2,750 independent deals. The explicit historical-tape mode separately
   matches all47 roots at8/40/128 scenarios
   ([receipt](results/final-tape/summary.json)). All612 frozen predecessor files
   and its retained896 phone replay moves also verify unchanged
   ([inherited audit](results/frozen-checkpoint/stdout.log)). Those moves belong
   to the earlier review-on/off phone check, not this prototype.

6. **Fresh native scaling checked against fair controls.** Deals
   `960000–960127`, plies12–15, give 47 eligible pip-trump roots, 30 with partial
   tricks. Four alternating-order repetitions use nested scenario prefixes at
   8/40/128/512 worlds. Every completed vector and choice matches the pinned
   native reference. Timings below include frontier service setup, partitioning,
   rule preparation, grouping, expansion, folding and completed-cache insertion.
   Reference totals include its solver preparation. JSON/process and outer
   replay are measured separately.

   | Worlds | Frontier 1 worker | Native recursion 1 worker | Frontier 4 workers | Native recursion across 4 workers |
   | ---: | ---: | ---: | ---: | ---: |
   | 8 | 4.360 ms | 0.889 ms | 1.691 ms | 0.372 ms |
   | 40 | 14.702 ms | 10.583 ms | 6.166 ms | 4.104 ms |
   | 128 | 45.285 ms | 35.292 ms | 16.757 ms | 11.810 ms |
   | 512 | 170.962 ms | 132.256 ms | 64.271 ms | 42.727 ms |

   [One-worker standalone receipt](results/final-native-one/summary.json),
   [equal-worker control](results/final-native-four-batch/summary.json).
   Four workers improve frontier throughput about 2.6–2.7x at128/512;
   batching the native recursive control is also effective. Comparing only
   against its existing **within-root** parallel path would give a misleading
   favorable headline: that path was slower on this batch
   ([retained measurement](results/native-four/summary.json)). The fair
   independent-query control removes that scheduling advantage.
   These repetitions are local timing observations, not statistical population
   intervals or phone-turn latency.

7. **All major costs and memory are visible.** At512 worlds, one-worker cold
   evaluation charges 2,846,259 scenario-row edges; four workers charge
   2,723,221 (smaller groups retain fewer absorbed terminal rows). Four-worker
   counted array peaks sum to 32,350,496 bytes; observed child process high-water
   RSS is 59,965,440 bytes. This cumulative child high-water includes both
   frontier and baseline processes in that experiment, so it is not separately
   attributable to the frontier. The array metric includes retained edge buffers,
   selected row/reduction arrays and held parent frames during nested solves,
   but excludes maps, query/spec clones, cache overhead and allocator overhead.
   Summed worker peaks are an upper bound, not a simultaneous measurement.
   At512, four-worker input serialization takes a median 27.43 ms, outer replay
   2.727 ms and output parsing 1.418 ms. Separate cold processes including those
   costs give native-reference/frontier ratios 0.851 at128 and 0.836 at512.
   With fresh generation at the actual scenario count charged over all128
   proposed deals, ratios are 0.991 and 0.991. The common generation cost
   dominates this fixture workload; it does not create a speed advantage.
   Total child CPU for the512 batch is 0.2221 s frontier vs0.1808 s reference,
   including startup, JSON, support replay and frontier warm-check overhead.
   Four-worker repeated identical requests cost 43.5 µs at128 and 96 µs at512,
   with completed root hits. This is **exact repeated-input reuse**, not evidence
   of a high natural online cache hit rate. Cold actor hit rate below is mostly
   zero. [Full per-run accounting](results/final-native-four-batch/records.json).

8. **Actor demand reveals the obstacle and an exact partial improvement.**
   On47 roots responding to modeled L0, eight outer worlds produce 11,174 raw
   nonfocal consultations, 10,845 unique queued misses, 43,380 inner samples and
   zero cold cross-layer hits. At40 outer worlds the corresponding counts are
   55,014 / 51,826 / 207,304. Full frontier lower queries instantiate 1,659,930
   and 7,834,725 row edges across four workers. Its median times are55.565 and
   237.059 ms, versus2.669 and17.417 ms for four-worker native recursion.
   [Full actor service](results/final-policy-full-four/summary.json).
   A single service with50,000 total actor-query allowance refuses the40-world
   batch after49,812 completed lower queries and about0.55 s; no root vector is
   returned. [Refusal receipts](results/final-policy-full/records.json).
   Four workers have independently scoped caps and complete; this is a larger
   aggregate allowance, not removal of the cap.

   The hybrid invokes actual pinned `modeled_choice` for these demanded actor
   choices rather than pricing full lower vectors. The same four-worker inputs
   take8.945 /36.962 ms, reducing full-frontier time by6.2x /6.4x and row work to
   45,329 /224,489, while still being slower than native batch recursion.
   Native core nodes and sampler counters remain separate:339,936 /1,603,142
   core nodes and43,380 /207,304 reported inner worlds. This is an exact hybrid
   improvement over eager lower frontier expansion, not a pure-frontier win.
   [Hybrid receipt](results/final-policy-hybrid-four/summary.json).

9. **Bounded higher demand checked, without a scaling theorem.** On six new
   eligible roots from deals962000–962031 atplies16–19 with eight outer worlds,
   full frontier responses to fields1 and2 both match complete native vectors.
   Field1 generates3,607 L0 and468 L1 unique misses,251,201 row edges and37 cache
   hits, taking20.603 ms vs1.392 ms native. Field2 generates15,452 /3,708 /470
   unique misses atL0/L1/L2,949,728 row edges and326 hits, taking78.388 ms vs3.801
   ms native. The counters include every queued call in this eager schedule;
   they are not the minimum demand of the optimized baseline. The trials use
   sample budgets `[4,2,2,2]`, uniform modeled support and Fixed selection.
   [Field1](results/final-rung1/summary.json),
   [field2](results/final-rung2/summary.json). They do not solve larger-k,
   opening-scale play, or exact full-population response.

## Next concrete experiment

Freeze this implementation as a semantic oracle. Replace per-scenario copies
of public State with **one immutable public state per prefix coordinate**, and
store only `(query, scenario, coordinate)` in forward arrays. Compute native
record hash and public transition once per occupied coordinate; scatter integer
row indices and retain the same shared backward fold. Start with exactly the47
fresh native Dice roots above at128/512 worlds and the same one/four-worker
recursive controls. Require all full vectors, rejection seeds, scenario
multiplicity, tie choices and cache guards to pass before timing.

For actor consumers, keep the native choice-only hybrid while measuring whether
state sharing reduces enough overhead to justify porting the existing compact
choice/Boolean pruning into the frontier. Do not replace actor inner support
with outer scenarios, reduce budgets, or average per-world optimizers. The
near-zero cold actor reuse here is evidence to charge lookups conservatively,
not assume reusable universal trees. A queue driven by the native solver's
actually demanded branches is a later scheduling experiment; this eager
frontier's counts must not be presented as its minimum demand.

## Boundaries and retained history

Every Lean/build/test/benchmark command has a process-group watchdog allowance
of at most295 seconds. Service row/work/query/cache/deadline guards are
per-worker; native choice calls obey their fixed sample schedule and shared
deadline rather than the frontier row counter. They expose separate native
work counters. No training, paid resource, install, persistent access, external
fixture transmission, browser action, production merge or deployment occurred.

Earlier intermediate benchmarks and build/setup failures remain under results/;
their plan files pin executable hashes. The first build had an invalid compact
rank cast, and the first CLI build moved a budget value before use; both were
fixed before successful tests. Final-* directories are authoritative final-source
measurements. Missing phone integration prevents a player-strength test;
component parity alone does not fill that gap. The separately blocked external
fixture handoff remains outside this local task.
