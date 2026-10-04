# Fresh independent native-policy review

Exploratory executable evidence and separately labeled conditional Lean kernel
facts. Reviewer scope is only this `checks/` directory. The inherited checkpoint
and its qualification history remain unchanged. Final accepted adapter identities
are native `51259e4088de71f813ab1c8ad0b81345129ea01c26dbefec60cba462c1878de7`
and WASM `11e2f2ca71b14220d8cf76ac9b21e63450257ee1ec5b86cc765f66d250963f3d`.

1. **Accepted: the actual sourced policy distinction is preserved.** The inherited
   [report items 7–9](../../resumable-choice-20261004/REPORT.md) explicitly name
   uniform40/Voidless lower beliefs/budgets `[4,2,2,2]`/`Field::Level(2)`/ascending
   ties and explicitly distinguish it from the pinned phone L1/partner policy.
   Its native control calls the existing recursive solver for that same late
   policy; it does not claim that L3 equals the deployed default. Its one-deal
   smoke and latency qualifications also stand. No such sourced assertion is
   newly contradicted. The new [adapter](../adapter/src/lib.rs) implements that
   named late policy directly; it does not derive its values from the experimental
   frontier and does not modify the source-pinned production crates.

2. **Accepted: production source and phone identity independently rerun.**
   [Identity receipt](identity/run.json) reruns the exact source-v2 audit over 202
   files. Their SHA256 is `5bedf14af0e5a5ef354acca4765f67843469d074d163e8e4786631aa23436e32`;
   Git source comparison passes against `cb1ef3b23072e4c268f31f625f2b61d5facc1929`.
   Phone WASM SHA256 is `40d7a1eea658627f7bab37d3c1888ad3bf00216fb23a5d2ceeb7d2d1f4219119`;
   saved production Git blobs match Plunge `a0d9fa806166b0e63fe016bb49d93f91f47b1af8`.
   This establishes local source/blob provenance, not current external deployment
   or physical-device execution. The reviewer used the inherited identity script
   read-only; its receipt lives here.

3. **Accepted finite actor-view/RNG/multiplicity conformance.** Reviewer generation
   [receipt](generate-fresh/run.json) selects36 eligible roots from165 independently
   proposed PRNG deals beginning 1071300, spanning all 9 Straight declarations and
   all 4 actor seats. [Independent checker](runner/src/main.rs) uses a separately
   written SplitMix64, rejection-zone/modulo draw, public-record seed mix and
   shuffle/reject routine. [Successful oracle receipt](independent-fresh-fixed/run.json)
   checks 1,440 ordered outer worlds against production sampling and its final
   RNG:59 duplicate accepted rows and3,691 rejected draws are retained. No sample
   deduplication or replacement by a different uniform sampler is permitted.
   It checks 108 modeled L0/L1/L2 preparations, including L0 Dice seeds after
   sampling;27 repeated actor choices match independently prepared full-vector
   native evaluation. Repeated L1/L2 calls also preserve policy-miss counters.

   Source audit confirms modeled choice depends on level, actor/remaining hand,
   public played mask/leader/current ordered trick/banked scores, and immutable
   declaration/contract/budget/frame/belief/selection context. The source cache
   packs current trick injectively including length/zero. Voidless deliberately
   forgets void deductions in modeled lower beliefs; outer draws still obey
   public voids. That is the frozen modeled policy, not a claim of exact
   perfect-recall behavioral belief. Policy cache reuse guards all immutable
   context and lawful boundary normalization. Its outer sampler seed/RNG and
   the lower policy seed streams are distinct. Cache identities do not include
   hidden referee worlds or source-deal seeds. Deadline refusal never constitutes
   a complete policy value.

   The first checker incorrectly expected repeated L0 to hit a cache; its
   [failed receipt](independent-fresh/run.json) is retained. Production
   `cpu-speedups` includes `bypass-l0-cache-all`, which intentionally recomputes
   L0. The repaired checker tests repeated choice equality there, with hit-counter
   assertions only at L1/L2. The initial manifest-path typo and its failed build
   receipt also remain preserved. Neither failed receipt is green evidence.

4. **Accepted finite complete native/WASM late-vector and public-wire checks.**
   [Stable native](native-adapter-stable/run.json) and
   [stable WASM](wasm-adapter-stable/run.json), through a separately written
   [reviewer host](wasm_worker.mjs), each compare36 complete vectors with the
   independent native oracle, all ordered worlds, final RNG and attempt counts.
   Each independently replays1,440 reconstructed full deals with the Python
   referee and checks17 malformed/missing/private request refusals and3
   early/forced/settled ineligible roots. Sixteen vectors contain value ties;
   exact complete values and ascending strict tie choices agree. These are72
   adapter vector comparisons on36 roots; they are not72 independent roots.

   [Stable full-wire/checkpoint receipt](wire-stable/run.json) additionally
   checks36 complete hybrid native/WASM vectors,72 lawful WASM checkpoints,
   8 zero-budget late refusals with no partial evaluation/world vector,
   11 invalid complete profiles on each backend before any checkpoint, and4
   controlled monotonic host-clock exhaustion cases retaining the initial legal
   checkpoint without launching a minimum100ms fallback after budget exhaustion.
   Every checkpoint has the correct own/public legality/score/leader boundary.
   ABI installs the host clock before evaluation and uses fresh WASM instances;
   output/checkpoint bytes are copied before borrowed Rust buffers disappear.
   The named late policy supports Straight bids30–42 and nine declarations;
   this experiment's complete games remain fixed bid30. Native/WASM conformance
   is finite executable evidence, not an exhaustive proof over every possible
   JSON byte sequence, allocation limit, browser scheduling or hardware clock.

5. **Qualified: implementation issues found early were repaired; development
   receipts are not final authority.** Initial eligible calls used a fixed
   sampler/solver allowance despite a smaller accepted complete-call budget.
   The final source deducts wrapper work, caps sampling and solver against the
   remaining caller allowance, and retains the legal checkpoint when less than
   the phone's100ms minimum remains. Initial source also flushed native node
   accounting twice; `action_values` already flushes it, so the duplicate was
   removed. Python full-game host routing received the corresponding remaining
   budget and retained-checkpoint repairs. These observations challenge the
   concrete adapter behavior during development and do not refute the inherited
   report's explicitly scoped policy claim. Earlier `*-fresh`/`*-final` receipt
   names predate the last wrapper repair; only `*-stable` receipts above establish
   the final adapter identities. None is a stable performance conclusion.

6. **Accepted conditional Lean kernel mathematics; qualified refinement scope.**
   [PolicyEquivalence.lean](PolicyEquivalence.lean) and
   [fresh receipt](lean-policy/run.json) prove finite Bellman refinement by
   horizon induction when terminal values, complete ordered successor lists and
   node folds agree. Separate laws preserve ordered duplicate mass and show
   complete lossless cache encoding preserves a pure policy. Actual printed
   axioms are `propext`/`Quot.sound` for finite Bellman and duplicate mass,
   `propext` for ordered mass, and none for cache refinement. No `sorry`,
   `native_decide`, external PASS axiom or opaque native computation is used.
   [Inherited resumable](lean-resumable/run.json), [arena](lean-arena/run.json)
   and [bounds](lean-bounds/run.json) freshly recheck7+6+7 declarations with
   their printed axioms. These conditional theorems do not prove the Rust
   implementation satisfies their premises, machine overflow, RNG, lawful
   actor views, deadlines, cache insertion/drop ordering, complexity or privacy.
   In particular the Bellman theorem does not promote benchmark parity to a
   generic higher-k or shared-plan equivalence theorem.

7. **Accepted finite retained production-policy control.** Parent-owned
   [phone conformance summary](../results/phone-conformance/summary.json),
   independently audited from its compressed raw records in
   [reviewer receipt](phone-records/run.json), verifies42 public requests and84
   comparisons between native production routing, rebuilt linked WASM and the
   exact pinned phone blob. The comparison recursively removes only
   `elapsed_us`, `solver_us` and `over_budget`; every other result field matches,
   and interrupted calls are excluded. The36 late inputs plus6 opening/early/
   forced/settled inputs cover all 9 declarations and the strict public boundary.
   This is the separate L1/partner control, not L3-versus-L1 policy equality or
   physical-device execution. [Final identity rerun](identity-final/run.json)
   again verifies the 202-input source pin after all adapter work.

8. **Accepted complete prespecified local game panel; qualified strength.**
   [Independent native game audit](games-native/run.json) and
   [WASM audit](games-wasm/run.json) each verify144 complete games/4,032 moves
   on 18 prespecified source deals, four rotations and both partnership roles.
   Reviewer-local Straight mechanics reconstruct every original deal, legal
   follow/ownership/turn, first-best trick winner, score, hand depletion, request,
   outcome, binary/source identity and aggregate arithmetic. Each panel's276
   distinct completed late vectors freshly rerun against the native policy with
   production sampler/final-RNG validation; there are zero late refusals or
   interrupted turns. [Cross-backend trace audit](game-traces/run.json) verifies
   all 144 corresponding full game trajectories and4,032 actor requests/decisions
   match, including each complete late vector. These two backends reuse the same
   source deals; their outcomes cannot be pooled as36 independent source deals.

   Both summaries yield0 paired wins,2 losses and70 ties, mean paired make
   difference−1/36, source-deal Hoeffding 95% interval
   `[-0.6679929719910582, 0.6124374164355026]`. The independent-deal assumption
   is explicitly a PRNG sampling assumption with fixed nine-declaration
   stratification; rotations are clustered. The audit checks the formula rather
   than proving that PRNG seeds are IID random variables. No stronger-play or
   equivalence result follows. Native candidate 2016 turns total47,116.916ms
   versus phone46,842.253ms; WASM candidate47,063.170ms versus phone46,721.140ms.
   Natural-game latency includes different policies/histories and early phone
   work, so it cannot isolate kernel causality or establish a whole-Walt speedup.
   Node on this Mac is not a physical-phone/browser device benchmark.

9. **Accepted finite same-policy cold arithmetic; qualified cost inference.**
   [Independent cold audit](cold-arithmetic/run.json) checks432 completed vectors
   on12 actual eligible public turns, one per distinct prespecified source deal,
   six repetitions and six balanced process positions. It freshly reconstructs
   the recorded first-eligible-turn selection from the game files, checks every
   complete vector/input identity, actual binary hash, cost sum, median, sample
   cost and RSS statistic. All432 vectors agree; no observation refuses. Process
   position balance is12 occurrences per variant per position. No timing is
   rerun during this audit.

   Median12-request panel costs are native52.622ms, inherited native control
   57.729ms, lazy272.964ms, resumable310.439ms, arena356.267ms and cold Node/WASM
   692.507ms. Resumable retains its arena improvement and loses to lazy/native
   here. The native adapter is the existing recursive policy made portable;
   this is not an algorithmic victory over native recursion. Per-request costs
   charge request JSON, child process/module setup, strict status/history replay,
   sampling attempts, solve, serialization/parse, temporary files and teardown.
   Common persistent Python driver startup/imports and saved-referee selection
   are outside those intervals; selection is separately reported. Cold Node
   compilation is charged, while the game host reuses a compiled module, so the
   cold WASM median is not the natural per-turn device cost. RSS is whole child
   process memory; WASM linear memory is a separate statistic. Recursive and
   frontier logical work/query/cache limits remain different. These are finite
   local medians, not distribution-free latency, memory or higher-k guarantees.

10. **Qualified complete-call profile and exact remaining obligations.** The
    named late policy is frozen at40 outer worlds. A valid complete-call `worlds`
    option controls the phone fallback; it does not reconfigure that late policy.
    The measured opening160/ordinary40 production profile is preserved. Evidence
    above does not establish arbitrary-world-budget semantics, Nel-O support by
    this new strict seven-field hybrid, or behavior across every browser/JSON/
    allocation/deadline state. `action:"late"` is a separate local audit/control
    interface and can return explicit refusal without a checkpoint; complete
    calls establish a lawful initial checkpoint before expensive work. Unsupported
    request types refuse. A fresh instantiation and synchronous host copying are
    ABI requirements. Actual browser/device hosting, allocation stress/failure
    policy, product wire exposure and deliberate production policy/budget choice
    remain production obligations. No merge/deploy, broader policy equivalence,
    arbitrary-k linearity or playing-strength theorem is supplied.

11. **Accepted preserved history and execution boundaries.** [Historical byte
    audit](history/run.json) verifies all 1,882 manifest entries: astra 176,
    adversarial 420, native-frontier 256, prefix-state 251, demand-bounds 232,
    choice-arena 255 and resumable 292. No original source, report or receipt was
    edited. [Final reviewer receipts](receipts/run.json) audits the 27 preceding completed
    or retained failed reviewer executions; its own run receipt adds the 28th.
    All have allowance≤295s, elapsed<300s, cleanup-error-free and reaped children. [Source hashes](SOURCE_HASHES.json)
    pin reviewer implementation/theorem and current adapter/game host sources
    plus actual final native/WASM binaries. The two failed checker assumptions
    described above remain retained and excluded from accepted results. All
    reviewer jobs ended; no background jobs remain. This reviewer writes only
    `checks/`, makes no commit or production change, and performs no external
    transmission/browser/credential/paid-resource action. The specific fixture
    disclosure to Claude remains blocked; all fixture use here is local.
