# Texas 42: a fast partnership-aware player

Version 0.1 · 2026-09-06 · Exploratory implementation and gameplay experiment

Prepared for a fresh Codex session on Jason's M5 Max. This packet is a launch brief, not a report of a new player or a mathematical certification. Its preparation read the supplied notes and selected GitHub files; it did not build the project, run Texas 42 experiments, or benchmark the Mac.

## 1. Mission and authority

Jason has a working Texas 42 player, Walt 1. He reports that it beats the previous double-dummy-based players head to head and is fun to play. The key gap is partnership reasoning: its decisions do not adequately account for what its partner wants to accomplish or how a play changes the partner's information and response.

The task is to close that gap with a playable trick-1 player on the M5 Max. A satisfactory outcome is an information-consistent player showing useful partnership behavior and at least comparable strength to Walt 1 at reasonably comparable sampling, within the time budget. The outcome is exploratory: whether a particular approach wins is a question to investigate.

Once Jason launches this brief, proceed with implementation and small experiments. Make routine choices without repeatedly asking for permission. Work locally in an isolated git worktree, preserve unrelated work, and create useful local checkpoints. No publication, deployment, or merge is needed to complete this task.

The following are Jason's explicit instructions for this experiment and take precedence over conflicting repository workflow requirements:

| Topic | Agreed instruction |
| --- | --- |
| Machine | M5 Max Mac; inspect the actual local machine/toolchain before making hardware-specific decisions. |
| Player latency | At most **60 seconds per trick**, starting at trick 1. This supersedes the earlier whole-game minute target. |
| Experiment duration | **300 seconds HARD wall time per experiment or benchmark**, across its child work as well. Enforce termination externally. |
| Work style | Fast, informative iterations today. No multi-hour experiments. |
| Workspace | New git worktree. Backward compatibility is not a consideration. |
| CI | Skip the full Rust CI, which exceeds the budget. Run focused correctness checks relevant to the experiment. |
| Strength reference | Walt 1's actual phone version/configuration. Roughly comparable sampling is a guideline for an informative comparison, not an equality law. |
| Method | L2 optional. CPU and GPU both allowed. Other lawful partnership models welcome. |
| Required correctness | The player and the plans it evaluates as executable must respect each acting seat's information. No strategy fusion. |
| Optimality | No requirement to solve the full imperfect-information game exactly or close a global optimality proof. |
| Learning from results | An L2 that loses is useful knowledge. Record it honestly. |

Preserve the repository's immutable source packages and its separation between exploratory observations and foundational claims. The CI waiver and compatibility freedom do not waive rule fidelity or information consistency. Do not modify historical instructions merely to remove friction; record this user-authorized experiment scope in the new worktree.

## 2. What partnership awareness means

The partner is another seat with its own private hand and public observations. It can have a plan, uncertainty, and a response to what we reveal or withhold. The candidate should value those consequences in some explicit, inspectable way.

Questions the player should be able to distinguish include:

- If I show this tile now, what uncertainty does that resolve for my partner, and what might they do differently?
- Does this play support a continuation my partner can actually recognize and execute?
- Am I preserving their route to the lead, spending a resource they need, or making them commit before useful information arrives?
- Will giving count to a partner's winner or keeping a control tile change partnership contract success?

Those are examples, not mandatory hand-coded rules. L2, a cheaper approximation to a reasoning partner, a small family of lawful partnership policies, or another explicit mechanism can address them. Merely adding a generic teammate bonus or sampling L1 more does not by itself establish that the gap has been closed.

Two concrete motivating situations already appear in `walt/LEVEL2-PROBE.md`:

1. A partner can reveal 5-5 during play, relieving the bidder's uncertainty about an opponent holding it. The old evaluator saturation-ties choices whose informational consequences differ for a thinking bidder.
2. Under the bidder's winning 6-6 lead, the partner holds 6-2 and the ten-count 6-4. The timing of giving count and revealing control affects what the bidder can infer and plan.

Use the repo's reconstruction records for exact fixtures. The prose above is insufficient to reconstruct a full legal deal. Some historical live-position anchors were recorded as missing or blocked; inspect their current status and use an available validated case if necessary. Do not invent the missing tiles or turn a synthetic analogue into the original hand.

## 3. Timing and comparison conventions

Use the ordinary game meaning of a trick: four plays. For an automated table controlling all four seats, target at most 60 seconds of combined computation for those four decisions. Record each seat separately as well. Exclude human thinking and UI animation, but include decision-time belief work, model calls, synchronization, and GPU transfers. This is a conservative operational interpretation of the agreed per-trick target; do not silently grant every seat a separate minute.

Separate initialization/build costs from steady-state play, and report them. Caches or offline preparation can be useful, but disclose their source and cost. Avoid constructing a huge expensive cache first and describing only its subsequent lookup as today's experiment.

A complete seven-trick hand can theoretically exceed the five-minute experiment cap even when every trick meets its player allowance. A full-hand benchmark must still stop within five minutes. Start with individual positions and tricks; use faster configurations or small complete-hand batches for play comparisons. A timed-out hand is incomplete, not a win, loss, or completed performance measurement.

Put an entire declared benchmark batch inside the watchdog. Do not launch an unbounded parameter sweep whose individual games happen to be short. Run a small batch, examine it, and choose the next experiment from what was learned. Avoid disguising a preplanned multi-hour run as repeated five-minute fragments.

For sampling comparisons, record outer worlds, every inner sampling budget, tie-refinement behavior, seeds, cache state, and actual work counts where available. Holding outer samples equal is useful, but L2's nested work must still be visible. Keep two comparisons distinct:

1. **Reasoning comparison:** similar outer evidence and known inner budgets; did the partner model change behavior and outcomes?
2. **Playable comparison:** each candidate operates within the Mac latency target; how strong and responsive is the resulting player?

Freeze the actual L1 baseline configuration before tuning. The inspected browser README declares defaults `n=40`, `n0=8`, a serial WASM implementation, and an optional racing mode. Confirm the local API and deployed client settings; a README default is not proof of what Jason's phone currently invokes. If client settings are unavailable, use those documented defaults as a clearly named provisional baseline and continue, reporting the limitation.

Keep auction and declaration behavior matched in the first partnership-play comparison. The existing L2 browser package already uses L1 bidding/declaration. The target utility remains partnership contract success: making the declared bid for the declaring team, or setting it for defenders. Extra points alone are not the success criterion. If a harness is fixed to bid 30, identify that scope explicitly.

## 4. Information consistency is the hard correctness boundary

A simulator may use a sampled complete world to execute mechanics and evaluate a policy. An acting policy may use only its own hand, lawful public history, and explicitly allowed state/randomness. The partner's actual hidden hand is not an input to the focal player.

At the same lawful information history, a proposed continuation must make the same decision across indistinguishable worlds, for fixed permitted randomness. In particular:

- Do not optimize one future policy in each sampled deal and average those optimized values as if they described one executable plan.
- Do not give a simulated partner the focal player's private hand, sampled-world identity, outer sample population, or hidden-dependent random seed.
- Do not combine both teammates' hands into a centralized team controller at execution time.
- Do not split identical public observations into separately optimized branches based on hidden labels.
- Do not infer hidden evidence from an action selected by a clairvoyant evaluator. Conditional updates must match what the lawful player could observe.

Seed schedules, caches, and GPU keys are part of this boundary. Two identical information states should not behave differently because of encounter order, thread scheduling, or which hidden world reached a cache first. Preserve relevant banked points, contract, public record, model identity, and fallback semantics in keys. Inspect the current implementation rather than assuming every historical surface has the same repairs.

Use targeted tests: legal replay; information-state invariance across hidden completions; correct acting-seat inputs; cache/seed purity; and consistency of proposed continuations. For a GPU port, add small CPU/GPU parity cases under identical semantics. These checks fit the experimental scope; none licenses claiming the entire repository CI passed.

A hidden-information relaxation may still be a useful explicitly labeled upper-bound instrument. It must not impersonate an executable policy or be used to claim the player's own reasoning is fusion-free without checking the continuation construction.

## 5. Repository orientation and source map

Repository: <https://github.com/jasonyandell/texas-42>

The handoff inspection observed default-branch commit `a80b98291a60c5fda22d44f528efc16016c53425` (merge of PR #87). The local checkout may be newer or contain relevant unmerged work. Record the actual base commit and local state; do not reset to this historical reference merely because the packet names it.

Start from the user's selected local repository. Verify its origin, working-tree state, and existing worktrees. Create a new branch/worktree from the appropriate current source without discarding changes in the original. Reuse available build artifacts where safe, and build the smallest relevant target. If setup itself threatens to become a long job, narrow it and report the obstacle rather than beginning an hours-long bootstrap.

Read `QUICKSTART.md` and applicable local instructions, then follow this map by task. Do not spend the whole session reconstructing every historical math or proof thread.

| Path relative to repo | Why it matters |
| --- | --- |
| `QUICKSTART.md`, `CLAUDE.md`, applicable `AGENTS.md` if present locally | Project orientation, immutable packages, evidence discipline. No root `AGENTS.md` was found in the inspected GitHub snapshot. |
| `wiki/walt-seat-play.md` | L1/L2 history, partnership divergences, baseline and historical purity repairs. Reported outcomes, not rerun in this packet. |
| `wiki/walt-instruments.md` | Existing tools and which older producers are archive-only. Read the relevant entries before recreating a tool. |
| `walt/SCENARIO-PLAYER.md` | Sampling-stack semantics, cache rules, outstanding obligations. |
| `walt/LEVEL2-PROBE.md` | Partner-information examples, targeted field upgrade, and later amendments. Its initial “not started” text is historical; the closing amendment says detection was built. |
| `walt/walt/src/solver/` | Native solver implementation; inspect module declarations and definitions before choosing an extension point. |
| `walt/walt/src/solver/field.rs` | Declared L0/L1 field identities, modeled-policy interface, information-state keys, and cache purity. |
| `walt/walt/src/bin/level1.rs`, `level2.rs` | Existing native L1/L2 entry points to inspect for a small probe. Verify accepted arguments locally. |
| `walt/walt/src/bin/walt_bridge.rs` | Existing arena subprocess protocol and independent leader/score cross-check. The inspected header says its objective is fixed to P(make 30); do not silently benchmark other bids through that contract. |
| `walt/walt-wasm/src/api.rs`, `pkg/README.md`, `tests/full_hand.rs`, `smoke.mjs` | Phone baseline request semantics, documented knobs, native/WASM full-hand comparisons. |
| `walt/walt2-wasm/src/api.rs`, `pkg/README.md`, `tests/full_hand.rs`, `smoke.mjs` | Existing L2 play, matched L1 auction, and small-sample latency observations. Useful reference, not an established strength improvement. |
| `walt/walt/src/solver/field_swap.rs` and related modules | Existing work on where changing modeled players can affect decisions. Locate exposure/wakeup machinery through module declarations. |
| `walt/walt/src/solver/unified.rs` | Newer budgeted tier cascade and model-belief integration; exploratory and distinct from the phone player. Inspect if useful, without making its entire integration a prerequisite. |
| `walt/briefs/UP0-REPORT.md`, `UP1A-REPORT.md`, `U0B-REPORT.md` | Newer unified-player, lazy posterior carry, and conditioned-frontier observations. Read only if using that machinery. |
| `walt/walt-metal/`, GPU-related crates and `wiki/walt-instruments.md` | Existing Metal work. Establish what its kernels actually compute; prior GPU receipt parity is not evidence of a GPU L2 solver. |
| `walt/probes/m3/`, mined divergence fixtures, `walt/probes/step9/` | Candidate historical test cases and recorded experiment outputs. Verify exact paths and scope locally. |
| `wiki/walt-negative-results.md` | Boundaries of failed compression claims; consult if pursuing a related route. |

Useful searches after orientation:

```sh
rg --files walt | rg 'level[12]|field|wakeup|divergence|bridge|wasm|metal|benchmark'
rg -n 'level1_evaluate|FieldKind|FieldModel|Field::Level|budgetMs|n_outer' walt/walt walt/walt-wasm walt/walt2-wasm
```

The repository has multiple player and research surfaces. Do not equate the latest unified research player, the old arena bridge, and the shipped phone oracle without checking their behavior. Likewise, the historical arena result has a stated pre-fix binary scope; compare fresh candidates against a freshly identified baseline.

The existing docs describe an external mk5/plunge arena. Prefer available local harnesses, but do not require another repository or external dataset before doing a valid small comparison. A minimal native harness can be built in the worktree if necessary.

## 6. How the supplied mathematics can help

Both source files are included unchanged under `math/`. Read the unified review first for semantics, particularly §§2, 4, 7, 12–16; then the improvisation note for candidate cost reductions. Their proposals are options to exploit, not a requirement to implement every layer.

### 6.1 One lawful plan across hidden possibilities

For a fixed focal belief and modeled behavior of the other seats, including the partner, a candidate is one information-consistent contingent policy. Its value averages that policy's results across hidden scenarios. Optimization selects among such policies. This preserves the order “choose one lawful policy, then average its outcomes.”

A fixed partner model can respond to public history. “Fixed” means the same rule is used, not that the partner ignores new information. This gives a direct route to pricing revelation and coordination without solving a full team equilibrium.

The supplied dual construction fixes the nonfocal field and relaxes the focal player's information. Jointly optimizing a pair of decentralized partner policies would be a different target with additional information constraints; the fixed-field theorem does not automatically cover it.

### 6.2 Capacity-saturated exact integration

When the hidden posterior factors into seat-local hand weights coupled by exact cover, fixed capacities allow selected completion-weight contractions through ordinary OR/zeta/Möbius transforms in O(n 2^n) ring operations. At a 21-tile opening this uses arrays of 2^21 entries.

The supplied note reports exact agreement with an independent full enumeration of 399,072,960 ordered deals on synthetic nonuniform weights. This is a reported counting experiment, not a reproduced benchmark here and not an opening game solve. It omits field classification and recursive policy search. Real posterior arithmetic width and factor preparation may change the cost substantially.

Use this route if profiling identifies a compatible counting bottleneck. It does not by itself make a partner model more intelligent. Correlated types or nonfactorizing beliefs need additional representation; do not force them into independent seat factors.

### 6.3 Information prices and inexpensive comparisons

The improvisation note describes penalties whose expectation is zero under every lawful policy. A clairvoyant evaluator minus such a penalty gives a valid upper when centering, inner maximization, and outer evaluation are sound. Good penalties can tighten bounds and reduce variation across sampled worlds. A small separable feature family may make the conditional centers cheap to compute.

This could help spend effort on action differences that matter. However, exact centering of a poor forecast preserves validity while merely approximate centering needs an error allowance. A sampled estimate of a relaxed maximum is not automatically a certified upper. Do not require the whole price-fitting system before establishing a playable partnership-aware reference.

### 6.4 Information deadlines

The note's “observations arriving before options expire” interpretation is particularly relevant to partnership. Revealing a tile may matter because the partner must choose a continuation before some resource is gone. This supplies a useful mechanism to inspect and a candidate feature vocabulary. It does not justify a greedy rule that only avoids immediate loss of makeability.

### 6.5 Approximation is allowed; misleading labels are not

A player can be lawful and approximate. Exact optimization is not required for this milestone. Sampling, cheap policy families, and empirical opponent models are eligible. Keep empirical improvements distinct from exact-value or optimality claims, and distinguish a useful partner model from a demonstrated model of human behavior.

## 7. Suggested first experiments — adapt after each result

The sequence below is a starting direction, not a mandatory architecture or a queue of long runs. Time out every execution, inspect evidence, and choose the next smallest informative step.

### A. Establish a small runnable baseline

Locate the phone L1 implementation/configuration and the current L2 implementation. Build only what a tiny native probe needs. Freeze a handful of valid trick-1 inputs plus one later-trick sanity position, including a partner-bid or defender case when available. Record decisions, timing, sample budgets, model identities, and seeds. Confirm the acting seat's input contains only its legal information.

Success for this step is a repeatable local comparison surface. Do not begin with the full CI, a full corpus census, or an exhaustive opening solve.

### B. Determine the existing L2 cost and partnership mechanism

Run a conservative sample schedule on one or a few roots under a short timeout. Compare to L1 on shared outer evidence where the implementations support it. Inspect the first useful disagreement: what changes about the partner's legal information or continuation?

If L2 is already affordable, use it as a candidate immediately. If it is expensive, profile enough to distinguish repeated modeled-seat solves, recursion, sampling/counting, arithmetic, and overhead. Stop a configuration that predictably exceeds the useful budget rather than waiting five minutes by habit.

### C. Pick one route suggested by the bottleneck

Possible routes include reusing existing caches and batch/CPU parallelism correctly; upgrading only the modeled partner first; selectively paying for a more capable field; compiling a cheaper lawful partner model; or batching suitable work on Metal. The selective/partner-only variants define different modeled fields and must be named as such.

For a GPU route, move an identified useful workload, preserve information-state semantics, and measure transfers and synchronization. No requirement exists to move the whole tree to the GPU. Avoid changing arithmetic near probabilities/ranks contrary to repository discipline merely to simplify a port.

For selective partner reasoning, establish that the trigger and fallback use lawful information. A heuristic trigger can produce a lawful approximate player, but it cannot certify that all missed positions are irrelevant without additional proof. Label that distinction.

### D. Check whether it helps

Use the known partnership cases for mechanism debugging, then a separate small held-out batch for strength. Rotate or mirror seats on the same deals and match contract/declaration conditions. Include bidder, bidder's partner, and defense where the small budget permits. Pair with L1 as well as the new candidate when practical, since coordination with an unchanged partner is part of the motivating goal.

Do not grade a candidate only by its own estimated Q values or its own preferred partner model. Use actually executed legal play against declared opponents and teammates. A partner ablation—same budget and search, weaker partner model—can help identify why behavior changed. Fewer samples can be a practical compromise, but disclose the tradeoff rather than crediting every outcome to reasoning depth.

Small batches can reveal regressions and promising effects without proving superiority. Report denominators, paired outcomes, ties, uncertainty where justified, and timeouts. Avoid a significance claim from repeatedly peeking until one batch looks favorable. If meaningful strength evidence does not fit today's runs, leave the result explicitly preliminary.

### E. Leave a playable artifact

Expose the candidate through a native runner, existing bridge/table adapter, or another small runnable interface that makes real decisions from legal observations. Compatibility is optional; usability is not. Document how to launch it and its supported straight-42 scope, knobs, and budget behavior.

A computational deadline needs an in-budget legal fallback and interruptible or bounded work units. The benchmark watchdog alone cannot turn an unfinished solve into a move. Include relevant preprocessing in the trick allowance, retain a usable candidate action as work proceeds, and report when fallback chooses the move. A fallback to L1 does not automatically prove non-regression for every mixed policy; test the executed candidate.

## 8. Deliverables and honest outcomes

Leave these in the experiment worktree:

1. The candidate code and a short runnable command, or the smallest runnable prototype if a remaining obstacle blocks play.
2. A fixed L1 reference description: commit/binary identity, sample settings, auction behavior, seed schedule, and any unresolved phone-setting uncertainty.
3. Small named legal fixtures and focused checks for the changed mechanics/information boundary.
4. Per-run logs and structured records with exact commands, elapsed time, work/sample settings, exits/timeouts, and results.
5. A concise report answering: what the partner model does; whether the player meets the per-trick limit on this Mac; how its actually executed outcomes compare; what remains unknown; and the next most informative small experiment.
6. A clear statement that full CI was deliberately not run under Jason's instruction, plus the focused checks that were run.

Report separately **lawfulness**, **latency**, **partnership behavior**, and **gameplay strength**. A failure or uncertainty in one dimension must not be hidden by another. A fast L2 that loses can satisfy the first three while yielding a negative strength result. That is valuable experimental progress and a legitimate stopping point for the tested approach.

Do enough to make the result usable and reviewable. Do not end at a high-level plan, spend the day on infrastructure, or keep expanding tests once the concrete experiment is adequately checked. Ask a focused question only when a genuinely unresolved choice prevents useful progress; otherwise choose, record the assumption, and continue.

## 9. Source provenance

User requirements in §§1–3 come from the planning conversation culminating in approval of this launch packet. The two mathematical files were supplied directly by Jason and are preserved byte-for-byte. They reference companion verifiers and older documents that are not bundled; their reported results are not independently rerun here.

Selected repository pages/files were read through GitHub during the planning pass, especially:

- [Quickstart](https://github.com/jasonyandell/texas-42/blob/main/QUICKSTART.md)
- [Seat-play history](https://github.com/jasonyandell/texas-42/blob/main/wiki/walt-seat-play.md)
- [L2 partnership probe](https://github.com/jasonyandell/texas-42/blob/main/walt/LEVEL2-PROBE.md)
- [Instrument inventory](https://github.com/jasonyandell/texas-42/blob/main/wiki/walt-instruments.md)
- [L1 browser package](https://github.com/jasonyandell/texas-42/blob/main/walt/walt-wasm/pkg/README.md)
- [L2 browser package](https://github.com/jasonyandell/texas-42/blob/main/walt/walt2-wasm/pkg/README.md)
- [Field-model implementation](https://github.com/jasonyandell/texas-42/blob/main/walt/walt/src/solver/field.rs)
- [Arena bridge](https://github.com/jasonyandell/texas-42/blob/main/walt/walt/src/bin/walt_bridge.rs)

These are navigation and provenance references. Check the actual local files and their latest amendments before relying on a claim or invoking a command. The packet supplies no new measured Texas 42 strength, latency, or GPU result.
