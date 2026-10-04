# Independent native-frontier check

2026-10-04. Fresh independent collaborator; exploratory local research at pinned canonical checkpoint 490997b13b161099dcfbcf19f2dec0819fd5a019. Only `checks/` authored here. No external fixture transmission, installation, production change, higher-k cost theorem or playing-strength claim.

## Pinned-source findings

[INTEGRATION.md](INTEGRATION.md) records the concrete requirements and source locations. Most consequential: native Dice uses public-state-reset SplitMix64 rejection/modulo, unlike historical NumPy depth-indexed multiply-high tapes. Modeled policies sample actor-specific beliefs; default Voidless deliberately omits historical void constraints. `record_hash` is not complete cache identity. The shared scope additionally fixes rules, contract, budgets, boundary, belief mode and selection rule. Every sample has unit mass including duplicates; aggregate shared-history action values before choosing. First ascending tile wins action-value ties. Canonical rotation normalizes declaring team to internal T1 but does not erase all seat-dependent RNG differences.

The native `solver::replay_contract` does not stop on contract settlement, unlike the archived Python sampler's corrected defect. It checks order/uniqueness and derives voids but does not validate hidden ownership or full original hand follow legality. Validate supplied outer worlds by reconstructing each original hand from its remainder plus its own historical plays and independently replaying the entire prefix. Do not apply that historical legality filter to default Voidless inner worlds: that changes modeled policy semantics.

The cpu-speedups feature includes uncached L0 evaluation. Service deduplication of deterministic L0 queries may reduce work while preserving choices; native miss counters are not unique-state counts.

## Independently rerun evidence

All commands used `experiments/astra-sol-20261004/tools/run_capped.py` process-group watchdogs with allowances at most 295 seconds. Every retained command completed with runner status completed and exit zero.

- **17 Lean declarations** across CompiledDAG (6), InnerFiber (2), PriorityRealization (4), SettledPruning (3), NestedPlans (2) passed. Relevant receipts: `lean-*/run.json`, stdout/stderr. Printed axioms are absent or standard `propext`/`Quot.sound`; no admitted proof, custom axiom or `native_decide` appears. These are conditional abstract results, not a Rust frontier compiler refinement or complexity proof.
- **Concrete historical-support repair**: `support-repair/` reproduces all 1,680 candidate capacity partitions, exactly 700 lawful worlds, rejection of the bad witness and 384 independently legal samples. One finite witness, not a general sampler distribution theorem.
- **Pinned native RNG**: `pinned-rng-test/` passes the rejection-boundary test that checks outputs and final RNG state against the original modulo path, including seeds whose first output is exactly the zone or u64 maximum.
- **Pinned native cache**: `pinned-cache-tests/` passes full represented-key equivalence and unfinished-trick injectivity including tile zero and all lengths up to three.
- **Pinned native choice**: `pinned-policy-test/` passes fixed choice vs complete value vectors at nonterminal higher modeled levels, both objective orientations and supported declaration variants.
- **Independent scalar oracle**: [scalar_oracle.py](scalar_oracle.py) implements grouped full-history scalar recursion using separately audited Python rule transitions and native Dice draws. `scalar-self/` checks 105 roots and 12 scenario-tripling cases. This is preparation and mass conservation, not native implementation parity yet.
- **Independent targeted RNG seeds**: [rng_boundaries.py](rng_boundaries.py), `rng-boundaries/` constructs 168 inverse-SplitMix cases for n=1..28 and outputs their forced-rejection traces, suitable for exercising a raw native service bundle.

## Independent native implementation parity

The guarded implementation passed [native-independent-hybrid/run.json](native-independent-hybrid/run.json), using a separate Rust harness and the independent Python scalar oracle. The fresh panel has 38 eligible roots among new deals 963000–963127, eight lawful outer worlds per root, plies 16–23, 28 partial tricks, all four bidders, and 31 targeted seeds whose first selected nonfocal draw is exactly the rejection zone.

- All **38 full action vectors** match scalar grouped recursion on separate Python rule transitions, including forced rejection behavior.
- All **76 full native reference vectors** (serial and parallel for every root) and choices match.
- All **38 tripled scenario bundles** exactly triple counts and retain choices, preserving sample multiplicity.
- **30 unsettled bid31 variants** match fresh pinned reference vectors and exercise contract cache separation.
- **24 actor-specific L0/L1 vectors and pinned modeled choices** match, across Voidless and VoidsCounted beliefs. This includes full L1 best responses to deterministic L0 policies; it does not establish a complete higher-rung phone integration.
- All 38 warm root requests hit the completed cache; initial-row refusal inserts no answer.
- Nine malformed direct actor-frame guards, including a valid context-union void state with empty completion fiber, refuse without panicking. Direct tape dimensions refuse; expired sampling refuses; expired service refuses a previously cached complete query without changing retained completed entries. Nineteen distinct ActorSpec key variants and six raw Query variants remain distinct under full equality.
- [protocol-guards-parallel/](protocol-guards-parallel/) runs ten malformed/capped JSON requests and then a valid request in one process. All refusals are correct and the valid request completes.

The final choice-only hybrid also passes: **48 policy-root full vectors** (12 roots × two field levels × two belief modes) agree with the full frontier backend and **96 pinned serial/parallel reference vectors**. The hybrid preserves complete `actors()` answer vectors by keeping a separate choice cache; cache capacity1 recomputation remains exact. It invokes the actual pinned `modeled_choice` with an evaluation-scoped Shared indexed by full Context. This is a hybrid demand-batching route, not proof of a pure-frontier performance improvement. The native core's nodes, policy-cache entries, misses and reported inner-world counts are separate from frontier row work. Summed declared budgets are explicitly labeled as budgets rather than measured sampling.

[parallel-protocol-final/](parallel-protocol-final/) passes **20 JSON requests**, ten paired workers1/workers4 comparisons, **344 full reference vectors**, including **152 scalar-checked custom-seed native vectors**. It covers native Dice and policy fields0/1, both belief modes, full-frontier and hybrid backends. It verifies returned root order, warm hits, separate worker statistics and their aggregate sums. All three native crate tests independently pass in [native-source-tests-final/](native-source-tests-final/), including exhaustive ordered four-tile outcomes across all nine supported declarations.

Each worker retains its own caches and caps. Four workers can use up to four times the declared row/work/query/cache allowance. Aggregated worker peaks are a sum of separate maxima, an upper bound rather than a simultaneous peak. The parallel route changes scheduling and cache reuse, not sample budgets or scenario support.

Source review found and the lead corrected invalid compact rank encoding, missing refusal-counter increments, missing direct-frame/tape checks, and counted-array underestimation from pretending folded edge buffers were deallocated. The corrected memory statistic includes all retained layer edges plus selected row/reduction arrays and held parent arrays during nested calls. It remains a **subset**: HashMaps, query/spec clones, cache allocation overhead and other objects are excluded; process peak RSS is distinct.

Two retained unsuccessful independent commands are harness/setup errors: `native-independent-first` changed bid to31 without excluding roots already set at T0=12, and `protocol-guards` was run before a CLI executable had been built. Both were corrected and rerun; neither is a solver discrepancy. Relevant final receipts and [AUDIT.json](AUDIT.json) pin the checked sources and watchdog allowances.

## Baseline-only CLI extension and final timing scope

The final library hash remained unchanged. The added `references` helper uses indexed rayon collection, retaining original query order; every reference query has its own pinned native Shared/Solver. `reference_only` validates and prepares the supplied outer corpus, evaluates the actual native reference and returns before allocating frontier services. `reference_total_us` measures the wall interval around native preparation plus solving after outer validation; `reference_solve_us` remains the sum of query solve intervals. Batch reference distributes independent queries across the existing four-thread pool. Deadlines remain per reference query, and every invocation also runs under the process-group watchdog.

[reference-protocol-final-metadata/](reference-protocol-final-metadata/) independently checks **26 requests and 348 full vectors** across serial baseline, batch baseline and batch plus inner parallel baseline, both belief modes, fields0/1/2 and custom native seeds. Twelve bounded field2 full-frontier roots also match those references. It checks answer order and the nesting of reference wall time within total request time. The previous parallel corpus was rerun unchanged with **344 vectors** in [parallel-protocol-reference-extension/](parallel-protocol-reference-extension/). [protocol-guards-reference-extension/](protocol-guards-reference-extension/) passes 12 refusals followed by a valid request, adding reference-only-without-reference and unsupported tape-reference checks.

[FINAL_RESULTS_AUDIT.json](FINAL_RESULTS_AUDIT.json) checks the seven retained final panels, reproduces median/ratio formulas and verifies every completed standalone frontier vector against its separate baseline process. It confirms:

- Pure native Dice, four workers on each side, 47 roots: cold frontier / reference wall is **1.419 at 128 worlds and 1.504 at 512**, so the frontier is slower on these measured bundles.
- Policy0 full frontier, one worker, 40 worlds: **all four repeats refuse the 50,000 actor query cap**; no answer is reported. Four workers complete with separate per-worker allowances and caches.
- Policy0 hybrid, four workers on each side: cold / reference wall is **3.488 at 8 worlds and 2.095 at 40**. The hybrid reduces work relative to the full frontier but remains slower than the matching native reference.
- Fields1/2 complete tiny six-root, eight-world panels with exact vectors; this is bounded demand evidence, not a general rung-growth theorem.

The source retains legacy `reference_over_frontier` and `end_to_end_common_ratio` diagnostics based on summed query solve intervals and maximum-count generation. They must not be interpreted as batch wall-throughput gains. Valid comparisons use `reference_total_over_frontier`, separate-process `standalone_ratio`, or actual-count-generation totals. Standalone frontend timing includes serialization, process startup, support validation, output parsing and its warm-cache verification; native-only baseline skips warm verification. Generation is separately rerun and charged for each actual world count over all proposed deals. CPU deltas are scoped to each child invocation. CHILDREN RSS is a cumulative high-water observation across prior frontend and baseline processes, not a separately attributed frontend RSS. Summed unique misses are per-worker counts, not a globally deduplicated state count.

## Checked final source identity

- `experiments/native-frontier-20261004/native/src/lib.rs`: SHA256 `995064fa56678660b93188d91dc00a0e297530b129120b150f9c4726cc61e6eb`.
- `experiments/native-frontier-20261004/native/src/main.rs`: SHA256 `60b3a85378e292f9cfef47bd4c277279a2bf1b2fd2bd343dff8dbfb3ec5544d6`.

[AUDIT.json](AUDIT.json) records 35 completed-or-retained receipts existing before its latest audit invocation, all watchdog allowances at most 295 seconds, all elapsed within allowance plus cleanup, and no cleanup errors. The two failed harness/setup receipts are explained above.

## Remaining limits

These checks establish finite tested parity and reviewed structural requirements, not a Rust program refinement proof, universal sampler correctness theorem, GPU result, favorable native performance, linear cost in rung, or playing strength. Full finite frontier trees can expand substantially. Service row-work counters count projected charged rows on a refusal, not necessarily all materialized edges or total native CPU work. The full public API is limited to validated late straight-contract frames; direct counted-frame empty support now refuses, and the choice-only hybrid is covered by the finite parity checks above. No runtime caller or phone adapter is validated here.

