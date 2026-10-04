# Walt constructive research with independent checks

2026-10-04. **Exploratory measurements; kernel lemmas identified separately.**
Canonical review dossier for this team. [Accepted findings](ACCEPTED.md),
[contradicted-source history](CONTRADICTED.md), [reproduction](README.md),
and [independent Sol report](sol/REPORT.md) are separate. Originals are preserved.

## Useful result

The strongest prototype is [frontier.py](frontier.py): full own-choice trees
evaluated one ply at a time, then folded backward with **one focal action per
complete public history**, after scenario aggregation. It retains the lawful
finite-tape objective; it does not impose the restricted ordering class.
History IDs are reindexed from `(previous history ID, played tile)` at each
depth, beginning with a root ID. No truncated base-29 history or hidden-world
decision key is used. It uses exact uint64 multiply-high tape selection.

On 105 development roots, including 69 partial tricks, all action vectors
matched grouped recursion. Four alternating-order measurements were
0.209–0.216 s versus 0.389–0.403 s. On **fresh deals 950000–950127**, 62 roots
were eligible and 52 had supported pip trumps. The independent holdout gives:

| Scenarios per root | Frontier median | Grouped recursion median | Reference / frontier |
|---:|---:|---:|---:|
| 8 | 0.07647 s | 0.04686 s | 0.61× |
| 40 | 0.13673 s | 0.21866 s | 1.60× |
| 128 | 0.31289 s | 0.66340 s | 2.12× |

All 156 holdout vectors and 12 tripled-scenario weight checks passed. At 128
scenarios, peak frontier width was 5,803 rows; counted retained arrays were
2,419,283 bytes. These are not full process memory measurements. Preparing all
128 proposed deals with 128 sampled worlds took 0.34946 s; including that common
cost gives **1.53×**, rather than 2.12×, for that full batch. Array preparation,
branch expansion, history sorting and reduction are included in frontier
timing. Imports and common fixture generation are excluded from both solve
columns. Four repeats on the same machine are timing observations, not a
population confidence interval. Small batches/bundles can lose to recursion.

[Development receipt](results/frontier/summary.json) ·
[holdout receipt](results/frontier-holdout/summary.json).
Scope: Python/NumPy CPU batch throughput, pip trumps 0–6, bid 30, public prefixes
at plies 12–15, uniform feasible outer support and frozen Dice continuations.
No native, GPU, phone latency, playing-strength or linear-k claim follows.

## Numbered verdicts

1. **Survives — original abstract mathematics.** All 30 declarations in the
   first team's six Lean files independently passed, including the original
   12. No admitted proof, custom axiom or `native_decide` was found. Lower-mind
   small regret can coexist with complete loss at a differently conditioned
   outer seat; uniform disagreement bounds plus a strict action margin give
   sufficient substitution conditions. These are conditional abstractions,
   not production-program refinement. The graph-fold theorem neither verifies
   the compiler nor proves array complexity. Nine new local lemmas also pass,
   for 39 declarations total. [Sol §5](sol/REPORT.md).

2. **Survives, narrowly — original performance qualifications.** Rerunning
   all 780 phase-3 vectors reproduces 592,717 universal coordinates, 58,835
   union-used, **90.07% unused**. Adaptive compilation plus one bid costs
   approximately 2.97× recursion; thirteen-payoff reuse costs approximately
   0.63×. These are different workloads. The statement is not that precisely
   thirteen reuses are necessary: thirteen was the measured reuse count.
   [New receipt](results/phase3-rerun/summary.json).

3. **Survives — substitution checking is too expensive in this implementation.**
   Fresh native build and all 84 eligible cases reproduce every world bundle,
   action count, bound and verdict. Only one is certified. Cold cheap solve
   plus checking plus fallback costs 1.699× fresh target evaluation; even the
   accepted case costs 112 µs versus 68 µs. This does not rule out a cheaper
   future necessary bound. [Receipt](results/cost-rerun/summary.json).

4. **Survives — production identity; qualified — strength.** Source hash over
   202 files and phone artifact identity match Plunge `a0d9fa80`, Rust
   `cb1ef3b2`, WASM SHA256 `40d7a1eea658627f7bab37d3c1888ad3bf00216fb23a5d2ceeb7d2d1f4219119`.
   The original 896 moves replay correctly. A fresh 32-game, four-deal,
   four-rotation, partnership-swapped phone holdout also yields 16 paired ties
   for review-off versus review-on, no fallback/interruptions, matched total
   budget 6,368,000 ms per side. Its conservative deal-cluster 95% interval is
   `[-1,1]`. There is no strength result. Our batching prototypes are not
   complete candidate phone players. [Identity](results/identity/stdout.log),
   [holdout](results/phone-holdout-summary/stdout.log).

5. **Survives — lawful uniform teacher; qualified — neural conclusions.**
   Uniform legal continuation labels are information-admissible. The creation
   source uses a fused comparison routine and a random row split. Exact
   regeneration finds 276,244 rows: **all 20,000 validation rows share a game
   with training**, which contains all 14,000 games. This is not held-out-game
   validation. It does not quantify accuracy inflation or invalidate the
   uniform teacher. Later checkpoints, paired-label variants and claimed
   neural regret numbers remain unverified. Issue #100 is a distinct memo
   experiment. [Sol §§3–4](sol/REPORT.md).

6. **Contradicted with executable evidence — archived sampler's bidder-irrelevance
   assumption, and new AUDIT-RESPONSE's assertion that it is unaffected.**
   `consistent_worlds` hard-codes bidder 0, prematurely settles a legal
   odd-bidder history, and accepts a hidden hand that violates follow-suit.
   This is distinct from `batched.py` v1's unchecked fallback. The subsequent
   [AUDIT-RESPONSE-2](incoming/drive-final/AUDIT-RESPONSE-2.md) explicitly retracts
   the assertion and acknowledges contamination of earlier odd-bidder Python
   results; its replacement engine is separately preserved. The separate
   function named `replay` does use the supplied bidder. Removed from accepted
   claims; [exact source, witness and correction](CONTRADICTED.md#c1).

7. **Survives after repair — valid historical support sampling.**
   [repaired_sampler.py](sol/repaired_sampler.py) changes only historical replay
   to prevent contract settlement from truncating it. On the witness it agrees
   with all 700 legal worlds among 1,680 capacity partitions and produces 384
   independently legal samples. This repairs the concrete bug while preserving
   the original files. Assumes a valid supplied history; not a hardened request
   validator, posterior theorem or universal correctness proof.
   The subsequently supplied `engine42.v2.py` independently passes the same
   exact witness, complete 1,680-partition/700-world check and 384 sampled-world
   check. [New version receipt](sol/receipts/final-packet-fixed/stdout.log).
   Its supplied regression script's old-path branch constructs invalid
   original deals by treating a current remainder as a full hand; its reported
   28% is therefore not an established contamination rate of the old sampler.
   The combined 37,872-world claim remains unrerun because its patched
   `nofusion_sc` and `batched` dependencies were not supplied in that packet.

8. **Qualified — new `batched.v2.py`.** Retrieved and hashed the actual new
   files, rather than relying on announced existence. With correct bidder,
   `tries=0` yields zero-filled false-mask slots; one try fills 117/128 and six
   fill 128/128 on the witness. All true-mask worlds independently replay
   legally. The claimed `l2.py` caller is absent: refusal/replacement handling
   there remains unverified. Cloning a verified world is lawful support but
   changes finite sample multiplicity/dependence; it is not fresh IID sampling.
   [Versioned packet](incoming/drive-0815/PROVENANCE.json), [Sol report](sol/REPORT.md).

9. **Survives — fixed-world realization.** For a deterministic world/tape,
   place the desired own sequence first in chronological priority. Earlier
   cards have been removed, so first-legal picks the desired next card.
   Remaining cards can follow arbitrarily. Sol independently compared 9,648
   orderings against 3,718 distinct tree traces on 402 scenarios: exact trace
   equality. [Lean local step and adaptive distinction](sol/PriorityRealization.lean)
   check relevant lemmas; the whole engine induction remains a mathematical
   argument. Per-world extrema equivalence is not lawful cross-world equivalence.

10. **Survives — sum-before-choice with truly shared plans; qualified — policy class.**
    `shared_orders.py` uses canonical actual ordering tuples shared across all
    scenarios, with identical per-world tapes. Every action satisfies
    `shared-order value ≤ lawful grouped value ≤ fused value` in focal-success
    orientation. On 134 roots, 65 have positive optimal-value gaps and 11 incur
    positive root-action regret; maxima are 10/40 and 4/40 respectively. This
    **quantifies Claude's explicitly acknowledged restriction**, not a
    contradiction of his proposal. Concrete complete fixtures and payoff tables
    are retained in [the panel](results/shared-orders/summary.json).

11. **Survives — flat vectorization and public-history refinement.**
    On 105 pip roots, all 100,800 vectorized payoff cells match scalar plans.
    Batch time 0.182–0.186 s includes array preparation, versus scalar 1.300 s
    and grouped recursion 0.385 s under the same interpreter. A nested
    partner-currently-winning refinement expands 24 to 96 plans. Independent
    scalar checking matches all 403,200 cells; aggregate value-gap counts
    improve 181→164 and root-regret counts 15→11. Time rises to 0.744–0.752 s,
    making full frontier batching the stronger next direction. These are
    development fixtures, not unbiased generalization estimates.
    [Base](results/vector-orders/summary.json), [refinement](results/refined-orders/summary.json).

12. **Survives on tested semantics — full shared-history frontier batching.**
    Exact finite-tape vectors and fresh-deal scaling above give a constructive
    measurable result without the ordering restriction. Both forward routing
    and backward shared decisions are charged. Branches can still grow rapidly;
    no exponential work is hidden or removed by the flat representation.

13. **Qualified — native rung growth and existing root parallelism.**
    Production already has a root-parallel path. Our native probe, using eight
    outer worlds and inner budgets `[4,2,2,2]`, completed 144 serial/parallel
    runs on 18 roots with matching action vectors. Modeled field indices 0–3
    require 129,984 / 795,414 / 3,367,805 / 10,874,648 serial node visits;
    solve time 4.857 / 21.455 / 97.919 / 365.610 ms. Four-thread final-field time
    is 142.790 ms. These are solve-only totals, not end-to-end phone latency.
    The default inner beliefs are actor-specific **Voidless**, whereas outer
    sampling honors voids. Policy counters count miss computations, not unique
    information states. Neither observed growth nor abstract demand trees prove
    a universal k lower bound. [Receipts](results/demand-mid/summary.json).

14. **Qualified — stochastic grouping, tape boundaries, support and rotations.**
    Continuous uniform selection uses floor(`u*n`) with zero-based indices;
    the literal ceil wording fails at boundaries. Finite RNG grids are not
    exactly uniform unless their size is divisible by the legal count; frozen
    tape evaluations can nevertheless be exact. Stochastic folds require
    accumulated reach weights before shared MAX/MIN. The archived unweighted
    stochastic fold is a source concern with an abstract reversal, but our
    653 tested legal roots yielded no concrete disagreement; actual Texas42
    impact remains unverified. `all_deals` generates duplicate-free ordered
    capacity partitions and then filters; binomial counts are before filtering.
    The audited 700-world case and native seat rotation pass. No general
    implementation-equivalence theorem is claimed. [Sol §§2,8–9](sol/REPORT.md).

15. **Qualified — Scheme specialization.** Pinned `Fix::compile` validates
    relational syntax and resolves immutable predicates. `step_belief` filters
    legality, multiplies supplied observation likelihood, steps and merges.
    Neither is an implemented higher-k game-tree compiler. Compact state merging
    still needs policy/tape memory sufficiency. [Source](../../walt/walt/src/scheme/dynamics.rs).

16. **Unverified — historical strength, later neural runs, full conversation
    transcript and complete higher-rung frontier.** First-team provenance says
    all 66 visible message bodies were read; no saved full conversation body
    transcript was located. The archive, recovered creation source and original
    nine Drive files were found and all nine were refetched byte-identically.
    Eleven later engineering files are separately frozen. The `flatplan.v2`
    performance report compares against per-world optimized `exact_tape`, so
    its gaps mix policy restriction with the fusion relaxation; our grouped
    reference is different. Its 25% opening agreement and 0.34 loss are neither
    pooled with our late-root results nor independently reproduced. The newer
    `flatplan.v3` source reports both grouped and fused comparisons on the same
    bundle, correcting the objective distinction. Its new numerical table is
    not independently reproduced here. No missing
    proof or unreproduced number is classified as contradicted.

## Next concrete experiment

Use the frontier as a batch service for **independent actor-specific lower-rung
queries** from the pinned native solver. First preserve its current sampler,
budgets, tie rules, modeled-belief mode and complete cache identity. Record
unique queued queries, edges, hit rate, row widths, refusal cost, peak memory
and total CPU work at each rung. Do not substitute outer worlds for an actor's
inner belief. Evaluate one fixed contract per query, charge preprocessing and
serialization, and compare to the existing native parallel path. A native
integer-array port should precede GPU work.

Only after full fixed-budget semantic parity and a complete candidate phone
adapter should playing strength be tested against the pinned WASM: new source
deals, all four seats, partnership swaps, matched budgets, complete-block
accounting and deal-cluster uncertainty. This checkpoint establishes a useful
batching mechanism, **not linear-cost exact higher k**.

## Provenance and boundaries

Independent worktree: `/Users/jason/Documents/Codex/2026-10-04/task-2/research-worktree`.
Branch `codex/walt-adversarial-20261004`. Stable copied checkpoint
`2aca32a8dce3cc7fcc866c420bc1633144400877` contains first-team commit `726fc654`
plus a before/after-hash-stable snapshot of its uncommitted phases 2–3 and
engineering packet. [SNAPSHOT.json](SNAPSHOT.json) pins 612 copied files.
Separate Git metadata lives in sibling `source.git`, with read-only shared
objects. First-team files/processes/browser tabs were not changed or interrupted.

The requested fresh `gpt-6.1-sol` collaborator ran successfully with independent
context and challenged source, proofs, experiments and the lead's prototypes.
Tool configuration is recorded; no independent runtime identity attestation is
available. All Lean/build/experiment invocations use process-group watchdogs
at ≤295 seconds. No paid resource, installed dependency, persistent access,
production merge or deployment was used. Existing NumPy Python is at
`/Users/jason/code/mk5-main/.venv/bin/python`.

There is no local execution blocker. Missing transcripts/checkpoints and
the absent `l2.py` are evidence gaps. The separate coordination read found the
first task active/in-progress with no error or visible pending approval; the
last visible browser call had completed. It did not confirm its Claude send.
[Exact status receipt](results/first-team-status.json).

The parent reported an automatic approval rejection for transmitting the
private local counterexample fixture to Claude, and requested explicit user
approval. No fixture transmission was attempted here; local verification
continued. That external handoff is pending authorization.

[Checkpoint audit](results/checkpoint-audit-final/stdout.log) verifies all 612 frozen
source files unchanged, replays every new phone move, checks matched budgets,
hashes downloaded versions, and validates retained run allowances and elapsed
times. Its source is [verify_checkpoint.py](verify_checkpoint.py).
